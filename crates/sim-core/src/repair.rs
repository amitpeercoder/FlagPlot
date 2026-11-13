//! Ship repair: forward at a base and in a yard off the map.
//!
//! The rule is `docs/decisions/0069-ship-repair.md`. There is no random draw.
//!
//! - **Forward repair** ([`World::repair_in_port`]): at the start of each day,
//!   after the day's orders, a base that mends points of damage a day spends
//!   them, through the staff and with no order, on the forces of its side
//!   stopped in its hex.
//! - **The yard**: a force stopped at a base that is a port for the yard can
//!   be ordered to the yard ([`crate::orders::TaskForceOrder::SendToYard`]).
//!   Its ships leave the map ([`ShipLocation::Yard`]), and
//!   [`World::return_from_yard`] brings them back, repaired, refuelled and
//!   rearmed, at the start of the day of return. The days away are worked out
//!   from the ships' damage by [`days_of_repair`] and nothing else.

use crate::events::{Breakdown, Cause, EventKind, Factor};
use crate::fixed::Fixed;
use crate::hex::HEX_NM;
use crate::ids::{BaseId, EventId, ShipId, TaskForceId};
use crate::refdata::{RefData, SurfaceCombatFigures};
use crate::state::{
    Ship, ShipLocation, ShipSystem, SystemState, TaskForce, TaskForceStatus, TruthState, YardTrip,
};
use crate::world::World;
use std::collections::BTreeMap;

/// How many of a ship's named systems are degraded and how many are out. A
/// system her class lacks is always working and counts for nothing.
pub fn systems_degraded_and_out(ship: &Ship) -> (u32, u32) {
    let mut degraded = 0;
    let mut out = 0;
    for system in ShipSystem::ALL {
        match ship.systems.get(system) {
            SystemState::Working => {}
            SystemState::Degraded => degraded += 1,
            SystemState::Out => out += 1,
        }
    }
    (degraded, out)
}

/// The days of yard repair of one ship, from her damage alone, with no
/// passage (decision 0069):
/// `ceil( yard_days_per_flotation_point x flotation_damage +
/// yard_days_per_system_degraded x D + yard_days_per_system_out x O )`.
/// Each product is one fixed-point multiplication; the sum is rounded up to a
/// whole day. Fire damage adds nothing. The one place yard days are worked
/// out: the order, its acceptance, `measure` and decision 0088's count all
/// read it.
pub fn days_of_repair(ship: &Ship, figures: &SurfaceCombatFigures) -> u32 {
    let (degraded, out) = systems_degraded_and_out(ship);
    let days = figures.yard_days_per_flotation_point * ship.flotation_damage
        + figures.yard_days_per_system_degraded * Fixed::from_int(i64::from(degraded))
        + figures.yard_days_per_system_out * Fixed::from_int(i64::from(out));
    let whole = days.trunc();
    let rounded_up = if days.raw() > whole * Fixed::SCALE {
        whole + 1
    } else {
        whole
    };
    u32::try_from(rounded_up).expect("days of repair are not negative and fit in a u32")
}

/// The days away of a force: the port's passage plus the largest days of
/// repair among its ships afloat (decision 0069).
pub fn days_away(passage_days: u32, ships: &[(ShipId, u32)]) -> u32 {
    passage_days + ships.iter().map(|(_, days)| *days).max().unwrap_or(0)
}

/// What a yard order would do, worked from the state as it stands.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct YardPlan {
    /// The port: the lowest-numbered base of the side in the force's hex that
    /// is a port for the yard.
    pub base: BaseId,
    pub passage_days: u32,
    /// Each ship afloat and her days of repair, in number order.
    pub ships: Vec<(ShipId, u32)>,
    pub days_away: u32,
}

impl YardPlan {
    /// The day of return of a force sent on `day`: the day of the order plus
    /// the days away.
    pub fn return_day(&self, day: u32) -> u32 {
        day + self.days_away
    }
}

/// [`World::yard_port`], on the truth state alone.
pub(crate) fn yard_port_in(truth: &TruthState, force: &TaskForce) -> Option<BaseId> {
    truth
        .bases
        .values()
        .find(|base| {
            base.side == force.side
                && base.hex == force.position()
                && base.yard_passage_days.is_some()
        })
        .map(|base| base.id)
}

/// [`World::yard_plan`], on the truth state and the reference data alone.
pub(crate) fn yard_plan_in(
    truth: &TruthState,
    ref_data: &RefData,
    force: TaskForceId,
) -> Option<YardPlan> {
    let force = truth.task_forces.get(&force)?;
    let figures = ref_data.surface_combat.as_ref()?;
    let base = yard_port_in(truth, force)?;
    let passage_days = truth.bases[&base].yard_passage_days?;
    let ships: Vec<(ShipId, u32)> = force
        .ships
        .iter()
        .filter(|id| truth.ships[id].location != ShipLocation::Sunk)
        .map(|id| (*id, days_of_repair(&truth.ships[id], figures)))
        .collect();
    if ships.is_empty() {
        return None;
    }
    let days_away = days_away(passage_days, &ships);
    Some(YardPlan {
        base,
        passage_days,
        ships,
        days_away,
    })
}

impl World {
    /// The port for the yard that serves a force at its hex: the
    /// lowest-numbered base of its side there with a `yard_passage_days`.
    pub fn yard_port(&self, force: &TaskForce) -> Option<BaseId> {
        yard_port_in(&self.truth, force)
    }

    /// What a yard order given to `force` now would do: its port, passage,
    /// each ship's days of repair and the days away. `None` if the force does
    /// not exist, has no ship afloat in it, has no port for the yard or the
    /// scenario has no surface combat figures. Reads the state as it stands
    /// and checks nothing else (the order's checks are `check_order`'s).
    pub fn yard_plan(&self, force: TaskForceId) -> Option<YardPlan> {
        yard_plan_in(&self.truth, &self.ref_data, force)
    }

    /// Applies an accepted yard order on `day` (decision 0069): the ships go
    /// out of the force to [`ShipLocation::Yard`], the force is left idle at
    /// its base with no route, cargo order, leg or alert, and `yard_trip` is
    /// set. Returns the event to record, `ForceLeftForYard`; the caller
    /// records it and puts its number into `yard_trip.departure`.
    pub(crate) fn send_force_to_yard(&mut self, force_id: TaskForceId, day: u32) -> EventKind {
        let plan = self
            .yard_plan(force_id)
            .expect("the order was validated: the force has ships and a port for the yard");
        let return_day = plan.return_day(day);
        let force = self
            .truth
            .task_forces
            .get_mut(&force_id)
            .expect("the force exists");
        let ships = std::mem::take(&mut force.ships);
        force.status = TaskForceStatus::Idle;
        force.route = crate::state::Route::default();
        force.route_order = None;
        force.cargo_order = None;
        force.toward_hex = None;
        force.progress_nm = Fixed::ZERO;
        force.leg_nm = Fixed::from_int(HEX_NM);
        force.alerted = false;
        force.alerted_by = None;
        force.yard_trip = Some(YardTrip {
            base: plan.base,
            return_day,
            // The caller sets it from the event's number.
            departure: EventId::new(0),
        });
        for ship in ships {
            self.truth
                .ships
                .get_mut(&ship)
                .expect("ship exists")
                .location = ShipLocation::Yard(force_id);
        }
        EventKind::ForceLeftForYard {
            task_force: force_id,
            base: plan.base,
            passage_days: plan.passage_days,
            ships: plan.ships,
            days_away: plan.days_away,
            return_day,
        }
    }

    /// The breakdown of a `ForceLeftForYard` event: the passage; then for
    /// each ship in number order her flotation damage, systems degraded,
    /// systems out and days of repair; then the days away. Recorded only when
    /// breakdowns are on, and changes nothing.
    pub(crate) fn yard_breakdown(&self, event: &EventKind) -> Option<Breakdown> {
        let EventKind::ForceLeftForYard {
            passage_days,
            ships,
            days_away,
            ..
        } = event
        else {
            return None;
        };
        if !self.config.record_breakdowns {
            return None;
        }
        let mut factors = vec![Factor {
            name: "passage_days",
            value: Fixed::from_int(i64::from(*passage_days)),
        }];
        for (ship_id, days) in ships {
            let ship = &self.truth.ships[ship_id];
            let (degraded, out) = systems_degraded_and_out(ship);
            factors.push(Factor {
                name: "flotation_damage",
                value: ship.flotation_damage,
            });
            factors.push(Factor {
                name: "systems_degraded",
                value: Fixed::from_int(i64::from(degraded)),
            });
            factors.push(Factor {
                name: "systems_out",
                value: Fixed::from_int(i64::from(out)),
            });
            factors.push(Factor {
                name: "days_of_repair",
                value: Fixed::from_int(i64::from(*days)),
            });
        }
        factors.push(Factor {
            name: "days_away",
            value: Fixed::from_int(i64::from(*days_away)),
        });
        Some(Breakdown {
            factors,
            result: Fixed::from_int(i64::from(*days_away)),
        })
    }

    /// Brings home every force whose day of return is `day` (decision 0069),
    /// at the start of the day, before refuelling and the day's orders.
    /// Forces are taken in number order and the ships of each in number
    /// order. Each ship is repaired, refuelled and rearmed; the force is idle
    /// at its port with no route, cargo order, alert or fatigue, and keeps
    /// its standing orders. Records `ForceReturnedFromYard`, citing the
    /// departure.
    pub(crate) fn return_from_yard(&mut self, day: u32) {
        let due: Vec<TaskForceId> = self
            .truth
            .task_forces
            .values()
            .filter(|force| force.yard_trip.is_some_and(|trip| trip.return_day <= day))
            .map(|force| force.id)
            .collect();
        for force_id in due {
            let trip = self.truth.task_forces[&force_id]
                .yard_trip
                .expect("the force is away");
            let ships: Vec<ShipId> = self
                .truth
                .ships
                .values()
                .filter(|ship| ship.location == ShipLocation::Yard(force_id))
                .map(|ship| ship.id)
                .collect();
            for ship_id in &ships {
                let class = &self.ref_data.ship_classes[&self.truth.ships[ship_id].class];
                let fuel = class.fuel_capacity_t;
                let guns = Fixed::from_int(
                    i64::from(class.main_guns) * i64::from(class.gun_rounds_per_gun),
                );
                let has_tubes = class.torpedo_tubes > 0;
                let ship = self.truth.ships.get_mut(ship_id).expect("ship exists");
                ship.flotation_damage = Fixed::ZERO;
                ship.fire_damage = Fixed::ZERO;
                ship.systems = crate::state::ShipSystems::ALL_WORKING;
                ship.system_repair_points = Fixed::ZERO;
                ship.fuel_t = fuel;
                ship.gun_rounds = guns;
                ship.torpedoes_ready = has_tubes;
                ship.torpedoes_one_side_fired = false;
                ship.location = ShipLocation::TaskForce(force_id);
            }
            let force = self
                .truth
                .task_forces
                .get_mut(&force_id)
                .expect("the force exists");
            force.ships.extend(ships.iter().copied());
            force.yard_trip = None;
            force.status = TaskForceStatus::Idle;
            force.toward_hex = None;
            force.progress_nm = Fixed::ZERO;
            force.leg_nm = Fixed::from_int(HEX_NM);
            force.route = crate::state::Route::default();
            force.route_order = None;
            force.cargo_order = None;
            force.arrived_at_hours = None;
            force.fatigue = Fixed::ZERO;
            force.alerted = false;
            force.alerted_by = None;
            let event = self.events.record(
                day,
                None,
                EventKind::ForceReturnedFromYard {
                    task_force: force_id,
                    base: trip.base,
                    ships,
                },
                vec![Cause::Event(trip.departure)],
                None,
            );
            self.truth
                .task_forces
                .get_mut(&force_id)
                .expect("the force exists")
                .arrival_event = Some(event);
        }
    }

    /// Forward repair (decision 0069), at the start of the day after the
    /// day's orders. Each base's allowance for the day is its
    /// `ship_repair_points_per_day`, shared by every force of its side
    /// stopped in its hex. Forces are taken in number order, skipping a force
    /// away, a force under way or with a `toward_hex` and a force with no ship
    /// afloat; the ships of a force in number order. Each ship takes from the
    /// lowest-numbered base of the side in the hex with allowance left, as
    /// refuelling takes fuel, and spends the points in this order, each step
    /// taking the smaller of the allowance left and what she needs: fire
    /// damage; flotation damage, only if it is below
    /// `forward_repair_max_flotation` at that step; then the degraded
    /// systems, one at a time in system order, each needing
    /// `forward_repair_system_points` less the points banked on the ship. A
    /// system that is out is never mended forward. A ship that takes points
    /// records one `ShipRepaired`; one that takes none records nothing, except
    /// that a bank cleared with no degraded system left records one of 0
    /// points.
    pub(crate) fn repair_in_port(&mut self, day: u32) {
        let Some(figures) = self.ref_data.surface_combat.clone() else {
            return;
        };
        let mut allowance: BTreeMap<BaseId, Fixed> = self
            .truth
            .bases
            .values()
            .filter(|base| base.ship_repair_points_per_day > Fixed::ZERO)
            .map(|base| (base.id, base.ship_repair_points_per_day))
            .collect();
        if allowance.is_empty() {
            return;
        }
        let forces: Vec<TaskForceId> = self.truth.task_forces.keys().copied().collect();
        for force_id in forces {
            let force = &self.truth.task_forces[&force_id];
            if force.yard_trip.is_some()
                || force.status == TaskForceStatus::Underway
                || force.toward_hex.is_some()
            {
                continue;
            }
            let ships: Vec<ShipId> = force
                .ships
                .iter()
                .copied()
                .filter(|ship| self.truth.ships[ship].location != ShipLocation::Sunk)
                .collect();
            if ships.is_empty() {
                continue;
            }
            let (side, hex, arrival_event) = (force.side, force.from_hex, force.arrival_event);
            let causes: Vec<Cause> = arrival_event.into_iter().map(Cause::Event).collect();
            for ship_id in ships {
                let Some(base_id) = self
                    .truth
                    .bases
                    .values()
                    .find(|base| {
                        base.side == side
                            && base.hex == hex
                            && allowance
                                .get(&base.id)
                                .is_some_and(|left| *left > Fixed::ZERO)
                    })
                    .map(|base| base.id)
                else {
                    break;
                };
                let allowance_before = allowance[&base_id];
                let mut left = allowance_before;
                let ship = self.truth.ships.get_mut(&ship_id).expect("ship exists");
                let bank_before = ship.system_repair_points;
                // Step 1: fire damage.
                let fire_mended = left.min(ship.fire_damage);
                ship.fire_damage -= fire_mended;
                left -= fire_mended;
                // Step 2: flotation damage, below the figure only.
                let flotation_at_start = ship.flotation_damage;
                let flotation_mended = if flotation_at_start < figures.forward_repair_max_flotation
                {
                    left.min(flotation_at_start)
                } else {
                    Fixed::ZERO
                };
                ship.flotation_damage -= flotation_mended;
                left -= flotation_mended;
                // Step 3: the degraded systems, the first one at a time.
                let mut restored: Vec<ShipSystem> = Vec::new();
                let mut system_points_spent = Fixed::ZERO;
                loop {
                    let Some(system) = ShipSystem::ALL
                        .into_iter()
                        .find(|system| ship.systems.get(*system) == SystemState::Degraded)
                    else {
                        // No degraded system: nothing is being mended, so the
                        // bank is cleared, spending nothing.
                        ship.system_repair_points = Fixed::ZERO;
                        break;
                    };
                    if left <= Fixed::ZERO {
                        break;
                    }
                    let needs = figures.forward_repair_system_points - ship.system_repair_points;
                    if left >= needs {
                        ship.systems.set(system, SystemState::Working);
                        ship.system_repair_points = Fixed::ZERO;
                        left -= needs;
                        system_points_spent += needs;
                        restored.push(system);
                    } else {
                        ship.system_repair_points += left;
                        system_points_spent += left;
                        left = Fixed::ZERO;
                        break;
                    }
                }
                let points = allowance_before - left;
                allowance.insert(base_id, left);
                // A ship that takes no point records nothing, unless her bank
                // was cleared (its system was hit to out, with no other
                // degraded): every change to the truth records an event, so
                // that one is a `ShipRepaired` of 0 points.
                if points <= Fixed::ZERO && ship.system_repair_points == bank_before {
                    continue;
                }
                let (fire_damage, flotation_damage, bank) = (
                    ship.fire_damage,
                    ship.flotation_damage,
                    ship.system_repair_points,
                );
                let breakdown = self.config.record_breakdowns.then(|| {
                    let mut factors = vec![
                        Factor {
                            name: "allowance_before",
                            value: allowance_before,
                        },
                        Factor {
                            name: "fire_mended",
                            value: fire_mended,
                        },
                        Factor {
                            name: "flotation_mended",
                            value: flotation_mended,
                        },
                        Factor {
                            name: "system_points_spent",
                            value: system_points_spent,
                        },
                        Factor {
                            name: "allowance_after",
                            value: left,
                        },
                    ];
                    if flotation_at_start >= figures.forward_repair_max_flotation {
                        factors.push(Factor {
                            name: "forward_repair_max_flotation",
                            value: figures.forward_repair_max_flotation,
                        });
                    }
                    Breakdown {
                        factors,
                        result: points,
                    }
                });
                self.events.record(
                    day,
                    None,
                    EventKind::ShipRepaired {
                        ship: ship_id,
                        task_force: force_id,
                        base: base_id,
                        points,
                        fire_damage,
                        flotation_damage,
                        restored,
                        system_repair_points: bank,
                    },
                    causes.clone(),
                    breakdown,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests;
