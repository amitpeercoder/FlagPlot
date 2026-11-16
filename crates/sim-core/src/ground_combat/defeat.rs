use crate::events::{Breakdown, Cause, EventKind, Factor};
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{AirGroupId, BaseId, EventId, LandUnitId, ShipId, TaskForceId};
use crate::orders::{AirMission, Posture};
use crate::refdata::GroundFigures;
use crate::state::{AirGroupLocation, March, ShipLocation, Side};
use crate::world::World;

use super::{at, other};

/// Whether a side with `men` ashore is beaten by one with `enemy_men`
/// (decision 15): it has no man, or fewer than `beaten_share` times the
/// other side's men.
pub(crate) fn is_beaten(men: u32, enemy_men: u32, beaten_share: Fixed) -> bool {
    men == 0 || Fixed::from_int(i64::from(men)) < beaten_share * i64::from(enemy_men)
}

impl World {
    /// Where a unit of `side` in `hex` falls back to (decision 2 of 0104):
    /// the first hex joined by a land link that holds a base of its side,
    /// else the first that holds no man ashore of an enemy fighting unit.
    /// It reads where the enemy's men truly are, so it stays with the rules.
    pub(super) fn fall_back_hex(&self, hex: Hex, side: Side) -> Option<Hex> {
        let links = self.map.land_neighbours(hex);
        let with_base = links.iter().copied().find(|to| {
            self.truth
                .bases
                .values()
                .any(|base| base.side == side && base.hex == *to)
        });
        with_base.or_else(|| {
            links.iter().copied().find(|to| {
                !self.truth.land_units.values().any(|other_unit| {
                    other_unit.side == other(side)
                        && other_unit.hex == Some(*to)
                        && !other_unit.working
                        && other_unit.men_ashore > 0
                })
            })
        })
    }

    /// The units of a side beaten in `hex` fall back (decisions 1 to 4 and 7
    /// of 0104), in unit order, after its bases have changed side. Each unit
    /// with men ashore is set to march to the first hex joined by a land
    /// link that holds a base of its side, else the first that holds no man
    /// ashore of an enemy fighting unit, tired by the defeat and on the
    /// defence; a unit that has no such hex surrenders. A unit already
    /// marching is left as it is.
    pub(super) fn fall_back(
        &mut self,
        day: u32,
        hex: Hex,
        side: Side,
        beaten_event: EventId,
        units: &[LandUnitId],
        figures: &GroundFigures,
    ) {
        let mut surrendering: Vec<LandUnitId> = Vec::new();
        for unit_id in units {
            let unit = &self.truth.land_units[unit_id];
            if unit.hex != Some(hex) || unit.men_ashore == 0 || unit.march.is_some() {
                continue;
            }
            let men = unit.men_ashore;
            let Some(to) = self.fall_back_hex(hex, side) else {
                surrendering.push(*unit_id);
                continue;
            };
            let unit = self
                .truth
                .land_units
                .get_mut(unit_id)
                .expect("the land unit exists");
            let before = unit.fatigue;
            let after = (before + figures.beaten_fatigue).min(Fixed::from_int(100));
            unit.fatigue = after;
            if unit.posture == Posture::Attack {
                unit.posture = Posture::Defend;
                unit.posture_order = None;
                unit.when_spent = None;
            }
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "fatigue_before",
                        value: before,
                    },
                    Factor {
                        name: "beaten_fatigue",
                        value: figures.beaten_fatigue,
                    },
                ],
                result: after,
            });
            let fell_back = self.events.record(
                day,
                None,
                EventKind::UnitFellBack {
                    unit: *unit_id,
                    side,
                    from: hex,
                    to,
                    men,
                    fatigue: after,
                },
                vec![Cause::Event(beaten_event)],
                breakdown,
            );
            self.lose_fieldworks(day, None, *unit_id, Cause::Event(fell_back));
            self.truth
                .land_units
                .get_mut(unit_id)
                .expect("the land unit exists")
                .march = Some(March {
                to,
                days: 0,
                counted: Fixed::ZERO,
                cause: Cause::Event(fell_back),
            });
        }
        if surrendering.is_empty() {
            return;
        }
        // One event tells the hex's surrender as a whole, so that what the
        // enemy is told shows no count of units (truth and belief).
        let total: u32 = surrendering
            .iter()
            .map(|unit_id| self.truth.land_units[unit_id].men_ashore)
            .sum();
        let together = self.events.record(
            day,
            None,
            EventKind::MenSurrenderedAshore {
                side,
                hex,
                men: total,
            },
            vec![Cause::Event(beaten_event)],
            None,
        );
        for unit_id in surrendering {
            self.surrender(day, hex, side, unit_id, together);
        }
    }

    /// A unit of a beaten side with nowhere to fall back to surrenders
    /// (decision 7 of 0104): every man ashore is lost, and the unit is
    /// destroyed if it has none aboard. `together` is the event that tells
    /// the hex's surrender as a whole.
    fn surrender(
        &mut self,
        day: u32,
        hex: Hex,
        side: Side,
        unit_id: LandUnitId,
        together: EventId,
    ) {
        let unit = self
            .truth
            .land_units
            .get_mut(&unit_id)
            .expect("the land unit exists");
        let men = unit.men_ashore;
        unit.men_ashore = 0;
        unit.hex = None;
        let surrendered = self.events.record(
            day,
            None,
            EventKind::UnitSurrendered {
                unit: unit_id,
                side,
                hex,
                men,
            },
            vec![Cause::Event(together)],
            None,
        );
        self.destroy_if_no_men(day, None, unit_id, surrendered);
    }

    // One argument more than clippy likes: the counts of both sides and
    // whether there was a fight are what the event records.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn record_beaten(
        &mut self,
        day: u32,
        hex: Hex,
        side: Side,
        men: [u32; 2],
        fought: bool,
        causes: Vec<Cause>,
        figures: &GroundFigures,
    ) -> EventId {
        let (own, enemy) = (men[at(side)], men[at(other(side))]);
        let breakdown = self.config.record_breakdowns.then(|| Breakdown {
            factors: vec![
                Factor {
                    name: "men_ashore",
                    value: Fixed::from_int(i64::from(own)),
                },
                Factor {
                    name: "enemy_men_ashore",
                    value: Fixed::from_int(i64::from(enemy)),
                },
                Factor {
                    name: "beaten_share",
                    value: figures.beaten_share,
                },
            ],
            result: figures.beaten_share * i64::from(enemy),
        });
        self.events.record(
            day,
            None,
            EventKind::SideBeatenAshore {
                side,
                hex,
                men: own,
                enemy_men: enemy,
                fought,
            },
            causes,
            breakdown,
        )
    }

    /// A base of a beaten side changes side, with everything that pointed at
    /// it (decisions 18 to 22). `causes` are the event that beat its side
    /// and what that cites: the fight, or under decision 16 the posture
    /// orders of the units on attack.
    pub(crate) fn change_side(
        &mut self,
        day: u32,
        base_id: BaseId,
        to: Side,
        causes: Vec<Cause>,
        figures: &GroundFigures,
    ) {
        let share = figures.captured_stock_share;
        let base = self.truth.bases.get_mut(&base_id).expect("the base exists");
        let from = base.side;
        let (supply_before, fuel_before) = (base.supply_t, base.fuel_t);
        let supply_kept = supply_before * share;
        let fuel_kept = fuel_before * share;
        base.side = to;
        base.on_map = true;
        base.supply_t = supply_kept;
        base.fuel_t = fuel_kept;
        let hex = base.hex;
        let breakdown = self.config.record_breakdowns.then(|| Breakdown {
            factors: vec![
                Factor {
                    name: "supply_t",
                    value: supply_before,
                },
                Factor {
                    name: "fuel_t",
                    value: fuel_before,
                },
                Factor {
                    name: "captured_stock_share",
                    value: share,
                },
                Factor {
                    name: "fuel_kept_t",
                    value: fuel_kept,
                },
            ],
            result: supply_kept,
        });
        let changed = self.events.record(
            day,
            None,
            EventKind::BaseChangedSide {
                base: base_id,
                hex,
                from,
                to,
                supply_t: supply_kept,
                fuel_t: fuel_kept,
                supply_lost_t: supply_before - supply_kept,
                fuel_lost_t: fuel_before - fuel_kept,
            },
            causes,
            breakdown,
        );
        // It stops being a command of the side that lost it and becomes one
        // of the side that took it (decision 0134, 7).
        self.base_changed_side(base_id, from);

        // Air groups of the beaten side at it are lost whole (decision 20); a
        // group of the side that takes it stays (decision 0101).
        let groups: Vec<AirGroupId> = self
            .truth
            .air_groups
            .values()
            .filter(|group| group.location == AirGroupLocation::Base(base_id) && group.side == from)
            .map(|group| group.id)
            .collect();
        for group_id in groups {
            let group = self
                .truth
                .air_groups
                .get_mut(&group_id)
                .expect("the air group exists");
            let aircraft = group.ready + group.damaged;
            group.ready = 0;
            group.damaged = 0;
            group.location = AirGroupLocation::Lost;
            let pilots = group.pilots.unwrap_or(0);
            if group.pilots.is_some() {
                group.pilots = Some(0);
            }
            let lost = self.events.record(
                day,
                None,
                EventKind::AirGroupLostWithBase {
                    air_group: group_id,
                    base: base_id,
                    aircraft,
                },
                vec![Cause::Event(changed)],
                None,
            );
            if pilots > 0 {
                self.events.record(
                    day,
                    None,
                    EventKind::PilotsLost {
                        air_group: group_id,
                        lost: pilots,
                        left: 0,
                    },
                    vec![Cause::Event(lost)],
                    None,
                );
            }
        }

        // A ship in port at it is lost, and sunk (decision 21), with the
        // air groups and men aboard her, as a ship that sinks.
        let ships: Vec<ShipId> = self
            .truth
            .ships
            .values()
            .filter(|ship| ship.location == ShipLocation::Port(base_id))
            .map(|ship| ship.id)
            .collect();
        for ship in ships {
            self.truth
                .ships
                .get_mut(&ship)
                .expect("the ship exists")
                .location = ShipLocation::Sunk;
            let sunk = self.events.record(
                day,
                None,
                EventKind::ShipLostWithBase {
                    ship,
                    base: base_id,
                },
                vec![Cause::Event(changed)],
                None,
            );
            self.lose_groups_aboard(day, ship, sunk);
            self.lose_men_with_ship(day, None, ship, sunk);
        }

        // A force of the beaten side whose home port it was has none
        // (decision 22).
        let forces: Vec<TaskForceId> = self
            .truth
            .task_forces
            .values()
            .filter(|force| force.side == from && force.home_port == Some(base_id))
            .map(|force| force.id)
            .collect();
        for force in forces {
            self.truth
                .task_forces
                .get_mut(&force)
                .expect("the task force exists")
                .home_port = None;
            self.events.record(
                day,
                None,
                EventKind::HomePortLost {
                    task_force: force,
                    base: base_id,
                },
                vec![Cause::Event(changed)],
                None,
            );
        }

        // An air group of the winning side whose mission was a strike on it
        // has no mission (decision 22).
        let strikers: Vec<AirGroupId> = self
            .truth
            .air_groups
            .values()
            .filter(|group| group.side == to && group.mission == AirMission::BaseStrike(base_id))
            .map(|group| group.id)
            .collect();
        for group_id in strikers {
            let group = self
                .truth
                .air_groups
                .get_mut(&group_id)
                .expect("the air group exists");
            group.mission = AirMission::Rest;
            group.mission_order = None;
            self.events.record(
                day,
                None,
                EventKind::StrikeMissionCleared {
                    air_group: group_id,
                    base: base_id,
                },
                vec![Cause::Event(changed)],
                None,
            );
        }
    }

    /// The air groups aboard a ship lost in port go down with her, ready and
    /// damaged, with their pilots, as a group lost with a ship that sinks
    /// (decision 5 of 0025). None is in the air after the last pulse.
    fn lose_groups_aboard(&mut self, day: u32, ship: ShipId, sunk: EventId) {
        let aboard: Vec<AirGroupId> = self
            .truth
            .air_groups
            .values()
            .filter(|group| group.location == AirGroupLocation::Ship(ship))
            .map(|group| group.id)
            .collect();
        for group_id in aboard {
            let group = self
                .truth
                .air_groups
                .get_mut(&group_id)
                .expect("the air group exists");
            let aircraft = group.ready + group.damaged;
            group.ready = 0;
            group.damaged = 0;
            let pilots = group.pilots.unwrap_or(0);
            if group.pilots.is_some() {
                group.pilots = Some(0);
            }
            let lost = self.events.record(
                day,
                None,
                EventKind::AirGroupLost {
                    air_group: group_id,
                    ship,
                    aircraft,
                },
                vec![Cause::Event(sunk)],
                None,
            );
            if pilots > 0 {
                self.events.record(
                    day,
                    None,
                    EventKind::PilotsLost {
                        air_group: group_id,
                        lost: pilots,
                        left: 0,
                    },
                    vec![Cause::Event(lost)],
                    None,
                );
            }
        }
    }
}
