//! Fuelling at sea (decision 0201, stage A).
//!
//! A ship whose class has the mark (`ShipClass::fuels_at_sea`) gives the fuel
//! she carries as cargo to a force under the order to fuel from hers, while
//! both steam on together at no more than the fuelling speed. This module
//! holds the order's checks (rule 5), its ends (rule 7), the steering of the
//! force for the giver (rule 6), the pairing at the start of the movement
//! step (rules 8, 9 and 15), and the fuel passed at the end of that step
//! (rules 10 to 16). The speed itself is capped where the movers are made
//! (`movement.rs`).
//!
//! What is worked within a pulse, which pairs fuel and the hours each line
//! is used, lives in a [`FuellingPulse`] for the length of the movement step
//! and is not state (rule 18). Nothing here draws a random number.

use std::collections::{BTreeMap, BTreeSet};

use crate::events::{Breakdown, Cause, EventKind, Factor, FuelOrderEnd};
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{EventId, OrderId, ShipId, TaskForceId};
use crate::orders::{Order, RejectReason, SpeedSetting};
use crate::refdata::{FuellingAtSeaFigures, ShipKind};
use crate::state::{Route, ShipLocation, TaskForce, TaskForceStatus};
use crate::time::Pulse;
use crate::world::World;

/// One force and the force it fuels from, paired at the start of a
/// movement step (decision 0201, 8).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct FuelPair {
    pub taker: TaskForceId,
    pub giver: TaskForceId,
    pub order: OrderId,
}

/// What lasts for one movement step of fuelling at sea, and is not state
/// (decision 0201, 18).
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub(crate) struct FuellingPulse {
    /// The pairs, in the taker's force order.
    pub pairs: Vec<FuelPair>,
    /// The forces that make no more than the fuelling speed this pulse, both
    /// of every pair (9), and that speed.
    pub slowed: BTreeSet<TaskForceId>,
    pub speed_kn: Fixed,
}

impl FuellingPulse {
    /// The most `force` makes this pulse, if it is slowed for fuelling.
    pub(crate) fn cap_for(&self, force: TaskForceId) -> Option<Fixed> {
        self.slowed.contains(&force).then_some(self.speed_kn)
    }
}

impl World {
    /// The figures of fuelling at sea, if the scenario's figures give them.
    fn fuelling_figures(&self) -> Option<FuellingAtSeaFigures> {
        self.ref_data.surface_combat.fuelling_at_sea
    }

    /// Whether `ship` is afloat and her class has the mark (decision 0201,
    /// 1).
    fn is_oiler(&self, ship: ShipId) -> bool {
        let record = &self.truth.ships[&ship];
        record.location != ShipLocation::Sunk
            && self.ref_data.ship_classes[&record.class].fuels_at_sea
    }

    /// The ships of `force` with the mark, afloat, in ship order.
    fn oilers_of(&self, force: &TaskForce) -> Vec<ShipId> {
        force
            .ships
            .iter()
            .copied()
            .filter(|ship| self.is_oiler(*ship))
            .collect()
    }

    /// The cargo fuel the ships of `force` with the mark hold together
    /// (decision 0201, 2).
    fn cargo_fuel_of_oilers(&self, force: &TaskForce) -> Fixed {
        self.oilers_of(force)
            .iter()
            .map(|ship| self.truth.ships[ship].cargo_fuel_t)
            .fold(Fixed::ZERO, |sum, tons| sum + tons)
    }

    /// What `ship` lacks of her class's fuel capacity, not below 0.
    fn fuel_lacked(&self, ship: ShipId) -> Fixed {
        let record = &self.truth.ships[&ship];
        let capacity = self.ref_data.ship_classes[&record.class].fuel_capacity_t;
        (capacity - record.fuel_t).max(Fixed::ZERO)
    }

    /// The ships of `force` that may take fuel at sea and lack some: afloat
    /// and without the mark (decision 0201, 3), in ship order.
    fn ships_lacking(&self, force: &TaskForce) -> Vec<ShipId> {
        force
            .ships
            .iter()
            .copied()
            .filter(|ship| {
                self.truth.ships[ship].location != ShipLocation::Sunk
                    && !self.is_oiler(*ship)
                    && self.fuel_lacked(*ship) > Fixed::ZERO
            })
            .collect()
    }

    /// The first force, in force order, with a ship afloat that is under the
    /// order to fuel from `giver`.
    pub(crate) fn taker_of(&self, giver: TaskForceId) -> Option<TaskForceId> {
        self.truth
            .task_forces
            .values()
            .find(|force| {
                !force.ships.is_empty() && force.fuel_from.is_some_and(|f| f.giver == giver)
            })
            .map(|force| force.id)
    }

    /// An escort order to `target` is refused where it takes part in
    /// fuelling at sea (decision 0201, 5: no chain of forces each following
    /// the next): under the order to fuel, or the giver for another.
    pub(crate) fn not_in_fuelling(&self, target: TaskForceId) -> Result<(), RejectReason> {
        if let Some(fuel_from) = self.truth.task_forces[&target].fuel_from {
            return Err(RejectReason::FuelsFrom(fuel_from.giver));
        }
        if let Some(taker) = self.taker_of(target) {
            return Err(RejectReason::IsAGiver { taker });
        }
        Ok(())
    }

    /// The checks of the order to fuel from `giver` given to `target`
    /// (decision 0201, 5), after the checks every task force order has.
    pub(crate) fn check_fuel_from(
        &self,
        order: &Order,
        target: TaskForceId,
        giver: TaskForceId,
    ) -> Result<(), RejectReason> {
        if giver == target {
            return Err(RejectReason::CannotFuelFromItself);
        }
        let other = match self.truth.task_forces.get(&giver) {
            Some(other) if other.side == order.side => other,
            _ => return Err(RejectReason::NotYourUnit),
        };
        if other.ships.is_empty() {
            return Err(self
                .force_ceased(giver)
                .unwrap_or(RejectReason::NoShipAfloat));
        }
        if self.oilers_of(other).is_empty() {
            return Err(RejectReason::GiverHasNoOiler(giver));
        }
        if other.fuel_from.is_some() {
            return Err(RejectReason::GiverIsFuelling(giver));
        }
        if other.charge.is_some() {
            return Err(RejectReason::GiverIsAnEscort(giver));
        }
        if let Some(escort) = self.escorts_of(giver).into_iter().next() {
            return Err(RejectReason::GiverHasAnEscort { giver, escort });
        }
        if let Some(taker) = self.taker_of(target) {
            return Err(RejectReason::IsAGiver { taker });
        }
        let force = &self.truth.task_forces[&target];
        if let Some(charge) = force.charge {
            return Err(RejectReason::EscortCannotFuel {
                charge: charge.force,
            });
        }
        if let Some(escort) = self.escorts_of(target).into_iter().next() {
            return Err(RejectReason::HasAnEscort { escort });
        }
        Ok(())
    }

    /// Rule 7 for every force under the order, in force order: the order
    /// ends where every ship is full, the giver's ships with the mark hold no
    /// cargo fuel, or the giver has no ship with the mark afloat in her
    /// force. A force with no ship afloat is passed by: it is never ordered
    /// and never moves.
    fn end_fuel_orders(&mut self, day: u32, pulse: Pulse) {
        let ending: Vec<(TaskForceId, FuelOrderEnd)> = self
            .truth
            .task_forces
            .values()
            .filter(|force| !force.ships.is_empty())
            .filter_map(|force| {
                let fuel_from = force.fuel_from?;
                let giver = self.truth.task_forces.get(&fuel_from.giver)?;
                let reason = if self.oilers_of(giver).is_empty() {
                    FuelOrderEnd::NoOiler
                } else if self.cargo_fuel_of_oilers(giver).is_zero() {
                    FuelOrderEnd::GiverEmpty
                } else if self.ships_lacking(force).is_empty() {
                    FuelOrderEnd::Full
                } else {
                    return None;
                };
                Some((force.id, reason))
            })
            .collect();
        for (taker, reason) in ending {
            self.end_fuel_order(day, pulse, taker, reason);
        }
    }

    /// Ends the order of `taker` for `reason` (decision 0201, 7): the force
    /// is at cruise with no route, idle where it is; a force out of fuel stays
    /// out of fuel. Records `FuelOrderEnded`, citing the order and, for a
    /// giver with no ship with the mark, the latest event that took it from
    /// her.
    fn end_fuel_order(&mut self, day: u32, pulse: Pulse, taker: TaskForceId, reason: FuelOrderEnd) {
        let force = self
            .truth
            .task_forces
            .get_mut(&taker)
            .expect("the force exists");
        let Some(fuel_from) = force.fuel_from.take() else {
            return;
        };
        force.speed = SpeedSetting::Cruise;
        force.speed_order = None;
        if force.status != TaskForceStatus::OutOfFuel {
            force.route = Route::default();
            force.route_order = None;
            force.halt = None;
            force.status = TaskForceStatus::Idle;
        }
        let mut causes = vec![Cause::Order(fuel_from.order)];
        if reason == FuelOrderEnd::NoOiler {
            causes.extend(self.oiler_lost_by(fuel_from.giver).map(Cause::Event));
        }
        self.events.record(
            day,
            Some(pulse),
            EventKind::FuelOrderEnded {
                task_force: taker,
                giver: fuel_from.giver,
                reason,
            },
            causes,
            None,
        );
    }

    /// The latest event by which `giver` lost a ship with the mark, or all
    /// her ships: a ship of hers with the mark sunk or detached, or the
    /// force joined, stood down or sent to the yard.
    fn oiler_lost_by(&self, giver: TaskForceId) -> Option<EventId> {
        let marked = |ship: &ShipId| {
            self.truth
                .ships
                .get(ship)
                .is_some_and(|record| self.ref_data.ship_classes[&record.class].fuels_at_sea)
        };
        self.events.latest_where(|kind| match kind {
            EventKind::ShipSunk {
                ship, task_force, ..
            } => *task_force == giver && marked(ship),
            EventKind::ShipsDetached { parent, ships, .. } => {
                *parent == giver && ships.iter().any(marked)
            }
            EventKind::TaskForceJoined { joined, .. } => *joined == giver,
            EventKind::TaskForceStoodDownInPort { task_force, .. }
            | EventKind::ForceLeftForYard { task_force, .. } => *task_force == giver,
            _ => false,
        })
    }

    /// The start of the movement step for fuelling at sea, after the escorts
    /// have kept station (decision 0201, 6, 8, 9 and 15): orders that have
    /// ended are ended; each force under the order is paired with its giver
    /// where both are in one hex, a ship of it lacks fuel and the giver holds
    /// cargo fuel, and a force out of fuel only where the giver lies still;
    /// and each force not out of fuel is steered for its giver.
    pub(crate) fn settle_fuelling(&mut self, day: u32, pulse: Pulse) -> FuellingPulse {
        self.end_fuel_orders(day, pulse);
        let figures = self.fuelling_figures();
        let mut fuelling = FuellingPulse {
            speed_kn: figures.map_or(Fixed::ZERO, |figures| figures.speed_kn),
            ..FuellingPulse::default()
        };
        let takers: Vec<(TaskForceId, crate::state::FuelFrom)> = self
            .truth
            .task_forces
            .values()
            .filter(|force| !force.ships.is_empty())
            .filter_map(|force| force.fuel_from.map(|fuel_from| (force.id, fuel_from)))
            .collect();
        for (id, fuel_from) in takers {
            let force = &self.truth.task_forces[&id];
            let giver = &self.truth.task_forces[&fuel_from.giver];
            let together = force.position() == giver.position() && !giver.ships.is_empty();
            let out_of_fuel = force.status == TaskForceStatus::OutOfFuel;
            // A force out of fuel is fuelled only where the giver lies still
            // in its hex: with no route, arrived or halted there (15).
            let giver_still = giver.status != TaskForceStatus::Underway;
            let paired = figures.is_some()
                && together
                && !self.ships_lacking(force).is_empty()
                && self.cargo_fuel_of_oilers(giver) > Fixed::ZERO
                && (!out_of_fuel || giver_still);
            if paired {
                fuelling.pairs.push(FuelPair {
                    taker: id,
                    giver: fuel_from.giver,
                    order: fuel_from.order,
                });
                fuelling.slowed.insert(id);
                fuelling.slowed.insert(fuel_from.giver);
            }
            // A force out of fuel takes no route (chapter 2).
            if !out_of_fuel {
                self.steer_for_giver(day, pulse, id, fuel_from.giver, fuel_from.order);
            }
        }
        fuelling
    }

    /// Steers `id` for `giver` by the escort's way (decision 0201, 6): in
    /// her hex it takes her route still to steam, cut where she halts this
    /// pulse, at her speed in knots, and stops where it is when she does not
    /// steam; out of her hex it steams for her hex at cruise. Written only
    /// where it changes the force's route, speed or status; the route order
    /// and speed order it writes are the order to fuel. A turning back is
    /// recorded, citing the order.
    fn steer_for_giver(
        &mut self,
        day: u32,
        pulse: Pulse,
        id: TaskForceId,
        giver: TaskForceId,
        order: OrderId,
    ) {
        let force = &self.truth.task_forces[&id];
        let given = &self.truth.task_forces[&giver];
        let together = force.position() == given.position();
        let wanted: Option<(Vec<Hex>, SpeedSetting)> = if together {
            self.charge_passage(giver, pulse)
                .map(|(waypoints, knots)| (waypoints, SpeedSetting::Knots(knots)))
        } else {
            Some((vec![given.position()], SpeedSetting::Cruise))
        };
        let remaining: Vec<Hex> = force
            .route
            .waypoints
            .get(force.route.next_index..)
            .map(<[Hex]>::to_vec)
            .unwrap_or_default();
        let moving = matches!(
            force.status,
            TaskForceStatus::Underway | TaskForceStatus::Halted
        );
        let unchanged = match &wanted {
            Some((waypoints, speed)) => {
                force.status == TaskForceStatus::Underway
                    && force.halt.is_none()
                    && remaining == *waypoints
                    && force.route.act_at.is_empty()
                    && force.route.stand_at.is_empty()
                    && force.speed == *speed
            }
            None => !moving && force.toward_hex.is_none(),
        };
        if unchanged {
            return;
        }
        let force = self
            .truth
            .task_forces
            .get_mut(&id)
            .expect("the force exists");
        let turned = match wanted {
            Some((waypoints, speed)) => {
                let turned = crate::escort::set_route(force, &self.map, waypoints);
                force.speed = speed;
                force.speed_order = Some(order);
                force.route_order = Some(order);
                turned
            }
            None => crate::escort::stop_where_it_is(force, &self.map),
        };
        if let Some(kind) = turned {
            self.events
                .record(day, Some(pulse), kind, vec![Cause::Order(order)], None);
        }
    }

    /// Whether the ships of `force` take fuel in port in the loading phase of
    /// this pulse (decision 0201, 16, read on the test of chapter 2's
    /// refuelling): it is stopped at the centre of a hex where a base of its
    /// side holds fuel above its reserve, and did not run dry this pulse.
    fn fuels_in_port(&self, day: u32, pulse: Pulse, force: &TaskForce) -> bool {
        let stopped = matches!(
            force.status,
            TaskForceStatus::Idle
                | TaskForceStatus::Arrived
                | TaskForceStatus::Halted
                | TaskForceStatus::OutOfFuel
        );
        if !stopped || force.toward_hex.is_some() {
            return false;
        }
        if self.serving_base(force.side, force.from_hex).is_none() {
            return false;
        }
        let ran_dry_now = force.status == TaskForceStatus::OutOfFuel
            && self.events.of_day_latest_first(day).any(|event| {
                event.pulse == Some(pulse)
                    && matches!(event.kind, EventKind::TaskForceOutOfFuel { task_force, .. } if task_force == force.id)
            });
        !ran_dry_now
    }

    /// Whether `force` works cargo in the loading phase of this pulse
    /// (decision 0201, 16): stopped at a hex centre, idle, arrived or
    /// halted, with cargo to work there.
    fn works_cargo(&self, force: &TaskForce) -> bool {
        matches!(
            force.status,
            TaskForceStatus::Idle | TaskForceStatus::Arrived | TaskForceStatus::Halted
        ) && force.toward_hex.is_none()
            && self.halt_ground().has_cargo_work(force, force.from_hex)
    }

    /// The attacks of this pulse on `forces`: each raid's attack on a ship of
    /// either and each surface action either fought (decision 0201, 10).
    fn attacks_on(&self, day: u32, pulse: Pulse, forces: [TaskForceId; 2]) -> Vec<EventId> {
        let mut attacks: Vec<EventId> = self
            .events
            .of_day_latest_first(day)
            .filter(|event| event.pulse == Some(pulse))
            .filter(|event| match event.kind {
                EventKind::AirAttack { task_force, .. } => forces.contains(&task_force),
                EventKind::EngagementBegun { a, b, .. } => {
                    forces.contains(&a) || forces.contains(&b)
                }
                _ => false,
            })
            .map(|event| event.id)
            .collect();
        attacks.sort();
        attacks
    }

    /// The end of the movement step for fuelling at sea (decision 0201, 10
    /// to 16): for each pair, in the taker's force order, the fuel passes
    /// unless either force was attacked this pulse, and then orders that
    /// have ended are ended.
    pub(crate) fn pass_fuel_at_sea(&mut self, day: u32, pulse: Pulse, fuelling: &FuellingPulse) {
        let Some(figures) = self.fuelling_figures() else {
            return;
        };
        // When each line of each oiler is free, in hours from the pulse's
        // start: kept for the pulse, across every pair (18).
        let mut lines: BTreeMap<ShipId, Vec<Fixed>> = BTreeMap::new();
        for pair in &fuelling.pairs {
            let (Some(taker), Some(giver)) = (
                self.truth.task_forces.get(&pair.taker),
                self.truth.task_forces.get(&pair.giver),
            ) else {
                continue;
            };
            // Both still afloat and in one hex, and the order still the one
            // paired.
            if taker.ships.is_empty()
                || giver.ships.is_empty()
                || taker.position() != giver.position()
                || taker.fuel_from.map(|f| f.order) != Some(pair.order)
            {
                continue;
            }
            let attacks = self.attacks_on(day, pulse, [pair.taker, pair.giver]);
            if !attacks.is_empty() {
                let hex = taker.position();
                let mut causes = vec![Cause::Order(pair.order)];
                causes.extend(attacks.into_iter().map(Cause::Event));
                self.events.record(
                    day,
                    Some(pulse),
                    EventKind::FuellingBrokenOff {
                        task_force: pair.taker,
                        giver: pair.giver,
                        hex,
                    },
                    causes,
                    None,
                );
                continue;
            }
            // A port before an oiler (16).
            if self.fuels_in_port(day, pulse, taker) || self.works_cargo(giver) {
                continue;
            }
            self.fuel_pair(day, pulse, pair, &figures, &mut lines);
        }
        self.end_fuel_orders(day, pulse);
    }

    /// The fuel passed in one pair (decision 0201, 11 to 15).
    fn fuel_pair(
        &mut self,
        day: u32,
        pulse: Pulse,
        pair: &FuelPair,
        figures: &FuellingAtSeaFigures,
        lines: &mut BTreeMap<ShipId, Vec<Fixed>>,
    ) {
        let pulse_hours = Fixed::from_int(pulse.hours());
        let taker = &self.truth.task_forces[&pair.taker];
        let giver = &self.truth.task_forces[&pair.giver];
        let oilers = self.oilers_of(giver);
        let line_count = usize::try_from(figures.lines.max(1)).unwrap_or(1);
        for oiler in &oilers {
            lines
                .entry(*oiler)
                .or_insert_with(|| vec![Fixed::ZERO; line_count]);
        }
        // The smallest share of capacity aboard first, then by id (13).
        let mut order: Vec<(Fixed, ShipId)> = self
            .ships_lacking(taker)
            .into_iter()
            .map(|ship| {
                let record = &self.truth.ships[&ship];
                let capacity = self.ref_data.ship_classes[&record.class].fuel_capacity_t;
                (record.fuel_t / capacity, ship)
            })
            .collect();
        order.sort();
        let mut restore = taker.status == TaskForceStatus::OutOfFuel;
        let mut previous_start = Fixed::ZERO;
        for (_, ship) in order {
            let record = &self.truth.ships[&ship];
            let destroyer = self.ref_data.ship_classes[&record.class].kind == ShipKind::Destroyer;
            // The oiler of lower id that still has hours and fuel (14): the
            // hour her lines are free, no earlier than the ship before
            // started (13).
            let chosen = oilers.iter().find_map(|oiler| {
                if self.truth.ships[oiler].cargo_fuel_t <= Fixed::ZERO {
                    return None;
                }
                let free = &lines[oiler];
                let (line_free, taken): (Fixed, Vec<usize>) = if destroyer {
                    let (index, hour) = free
                        .iter()
                        .enumerate()
                        .min_by_key(|(index, hour)| (**hour, *index))
                        .map(|(index, hour)| (index, *hour))
                        .expect("an oiler has a line");
                    (hour, vec![index])
                } else {
                    let hour = free.iter().copied().max().expect("an oiler has a line");
                    (hour, (0..free.len()).collect())
                };
                let start = line_free.max(previous_start);
                (start < pulse_hours).then_some((*oiler, start, taken))
            });
            // No ship starts before one earlier in the order (13).
            let Some((oiler, start, taken)) = chosen else {
                break;
            };
            previous_start = start;
            let lines_held = u32::try_from(taken.len()).unwrap_or(1);
            let rate = figures.line_rate_t_per_h * Fixed::from_int(i64::from(lines_held));
            let alongside_end = start + figures.alongside_hours;
            let hours = (pulse_hours - alongside_end).max(Fixed::ZERO);
            let lacked = self.fuel_lacked(ship);
            let held = self.truth.ships[&oiler].cargo_fuel_t;
            let tons = lacked.min(held).min(rate * hours);
            let done = if tons.is_zero() {
                pulse_hours
            } else {
                (alongside_end + tons / rate).min(pulse_hours)
            };
            let free = lines.get_mut(&oiler).expect("the oiler's lines");
            for index in taken {
                free[index] = done;
            }
            if tons.is_zero() {
                continue;
            }
            let fuel_t = {
                let record = self.truth.ships.get_mut(&ship).expect("the ship exists");
                record.fuel_t += tons;
                record.fuel_t
            };
            let oiler_cargo_fuel_t = {
                let record = self.truth.ships.get_mut(&oiler).expect("the oiler exists");
                record.cargo_fuel_t -= tons;
                record.cargo_fuel_t
            };
            // The first ship fuelled sets a force out of fuel idle where it
            // is, its route dropped, as in port (15).
            let restored = restore;
            if restore {
                restore = false;
                let force = self
                    .truth
                    .task_forces
                    .get_mut(&pair.taker)
                    .expect("the force exists");
                force.status = TaskForceStatus::Idle;
                force.route = Route::default();
                force.route_order = None;
            }
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "lacked_t",
                        value: lacked,
                    },
                    Factor {
                        name: "oiler_cargo_fuel_t",
                        value: held,
                    },
                    Factor {
                        name: "start_hours",
                        value: start,
                    },
                    Factor {
                        name: "alongside_hours",
                        value: figures.alongside_hours,
                    },
                    Factor {
                        name: "hours",
                        value: hours,
                    },
                    Factor {
                        name: "lines",
                        value: Fixed::from_int(i64::from(lines_held)),
                    },
                    Factor {
                        name: "rate_t_per_h",
                        value: rate,
                    },
                ],
                result: tons,
            });
            self.events.record(
                day,
                Some(pulse),
                EventKind::ShipFuelledAtSea {
                    ship,
                    task_force: pair.taker,
                    oiler,
                    giver: pair.giver,
                    tons,
                    fuel_t,
                    oiler_cargo_fuel_t,
                    restored,
                },
                vec![Cause::Order(pair.order)],
                breakdown,
            );
        }
    }
}
