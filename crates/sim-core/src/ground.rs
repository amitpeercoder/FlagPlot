//! Men ashore: fed from their side's base each night, and lost unfed
//! (decisions 16 to 20 of `docs/decisions/0066-land-units-carried-and-fed.md`).
//!
//! The men's supply is its own phase of the night pulse, after the daily
//! supply for aircraft and before air group rest, and runs only when the
//! ground figures are loaded. Hexes are taken in hex order and, in a hex,
//! sides in the order of their codes. There is no randomness.
//!
//! Reads: each land unit's men ashore and hex, the bases of the side in the
//! hex and their supply stock, the two ground figures. Writes: a base's
//! supply stock, a land unit's men ashore and hex, and the events.

use std::collections::BTreeMap;

use crate::events::{Breakdown, Cause, EventKind, Factor, SupplyUse};
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{BaseId, EventId, LandUnitId, ShipId};
use crate::state::Side;
use crate::time::Pulse;
use crate::world::World;

/// The men to take from each unit when `loss` men are lost from units of
/// `men` men each, in unit order (decision 0066, 20): unit i gives the loss
/// times its men over all the men, cut; the men still to take are taken one
/// from each unit in unit order, round again if need be, from units that
/// still have men. Never takes more than a unit has, nor more than all
/// have.
pub(crate) fn take_in_proportion(loss: u32, men: &[u32]) -> Vec<u32> {
    let total: u64 = men.iter().map(|m| u64::from(*m)).sum();
    if total == 0 {
        return vec![0; men.len()];
    }
    let loss = u64::from(loss).min(total);
    let mut parts: Vec<u64> = men.iter().map(|m| loss * u64::from(*m) / total).collect();
    let mut left = loss - parts.iter().sum::<u64>();
    while left > 0 {
        for (part, m) in parts.iter_mut().zip(men) {
            if left == 0 {
                break;
            }
            if *part < u64::from(*m) {
                *part += 1;
                left -= 1;
            }
        }
    }
    parts
        .into_iter()
        .map(|part| u32::try_from(part).expect("a part is at most a unit's men"))
        .collect()
}

impl World {
    /// The men's supply phase of the night pulse (decisions 16 to 20 of
    /// 0066). For each hex, and each side with men ashore there: the
    /// lowest-numbered base of the side in the hex is asked for the daily
    /// charge of every man and pays what its stock covers; the tons paid over
    /// the charge for a man, cut, are fed, and the rest unfed. A share of the
    /// unfed is lost, cut, and one man at least, taken from the side's units
    /// there in proportion.
    pub(crate) fn feed_men_ashore(&mut self, day: u32, pulse: Pulse) {
        let Some(figures) = self.ref_data.ground else {
            return;
        };
        // Units with men ashore, by hex and side code, in unit order.
        let mut ashore: BTreeMap<(Hex, u8), (Side, Vec<LandUnitId>)> = BTreeMap::new();
        for unit in self.truth.land_units.values() {
            if let (Some(hex), true) = (unit.hex, unit.men_ashore > 0) {
                ashore
                    .entry((hex, unit.side.code()))
                    .or_insert_with(|| (unit.side, Vec::new()))
                    .1
                    .push(unit.id);
            }
        }

        for ((hex, _), (side, units)) in ashore {
            let men: u32 = units
                .iter()
                .map(|unit| self.truth.land_units[unit].men_ashore)
                .sum();
            let base: Option<BaseId> = self
                .truth
                .bases
                .values()
                .find(|base| base.side == side && base.hex == hex)
                .map(|base| base.id);

            // The charge (decision 16). A base with no stock is asked
            // nothing and pays nothing, and nothing is recorded as paid
            // (decision 17).
            let mut fed_causes: Vec<Cause> = Vec::new();
            let mut paid = Fixed::ZERO;
            if let Some(base) = base.filter(|id| !self.truth.bases[id].supply_t.is_zero()) {
                let asked = figures.daily_supply_t_per_man * i64::from(men);
                let (paid_t, left) = self.take_supply(base, asked);
                paid = paid_t;
                let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                    factors: vec![
                        Factor {
                            name: "men_ashore",
                            value: Fixed::from_int(i64::from(men)),
                        },
                        Factor {
                            name: "daily_supply_t_per_man",
                            value: figures.daily_supply_t_per_man,
                        },
                    ],
                    result: asked,
                });
                let spent = self.events.record(
                    day,
                    Some(pulse),
                    EventKind::BaseSupplySpent {
                        base,
                        reason: SupplyUse::Men,
                        aircraft: 0,
                        men,
                        asked_t: asked,
                        paid_t: paid,
                        left_t: left,
                    },
                    Vec::new(),
                    breakdown,
                );
                fed_causes.push(Cause::Event(spent));
            }

            // The men fed: the tons paid over the charge for a man, cut, and
            // never more than the men (decision 17).
            let by_tons = u32::try_from((paid / figures.daily_supply_t_per_man).trunc().max(0))
                .unwrap_or(u32::MAX);
            let fed = by_tons.min(men);
            let unfed = men - fed;
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "men_ashore",
                        value: Fixed::from_int(i64::from(men)),
                    },
                    Factor {
                        name: "paid_t",
                        value: paid,
                    },
                    Factor {
                        name: "daily_supply_t_per_man",
                        value: figures.daily_supply_t_per_man,
                    },
                ],
                result: Fixed::from_int(i64::from(fed)),
            });
            let fed_event = self.events.record(
                day,
                Some(pulse),
                EventKind::MenFed {
                    side,
                    hex,
                    base,
                    men,
                    fed,
                    unfed,
                },
                fed_causes,
                breakdown,
            );
            if unfed == 0 {
                continue;
            }

            // The loss (decision 19): the unfed times the share, cut, and one
            // man at least.
            let share = figures.unfed_loss_share_per_day;
            let cut = u32::try_from((share * i64::from(unfed)).trunc().max(0)).unwrap_or(u32::MAX);
            let loss = cut.max(1).min(unfed);
            let unit_men: Vec<u32> = units
                .iter()
                .map(|unit| self.truth.land_units[unit].men_ashore)
                .collect();
            let parts = take_in_proportion(loss, &unit_men);
            for ((unit, unit_men), part) in units.iter().zip(&unit_men).zip(parts) {
                if part == 0 {
                    continue;
                }
                let land_unit = self
                    .truth
                    .land_units
                    .get_mut(unit)
                    .expect("the land unit exists");
                land_unit.men_ashore -= part;
                if land_unit.men_ashore == 0 {
                    land_unit.hex = None;
                }
                let left = land_unit.men_ashore;
                let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                    factors: vec![
                        Factor {
                            name: "unfed_in_hex",
                            value: Fixed::from_int(i64::from(unfed)),
                        },
                        Factor {
                            name: "unfed_loss_share_per_day",
                            value: share,
                        },
                        Factor {
                            name: "loss_in_hex",
                            value: Fixed::from_int(i64::from(loss)),
                        },
                        Factor {
                            name: "unit_men_ashore",
                            value: Fixed::from_int(i64::from(*unit_men)),
                        },
                        Factor {
                            name: "men_ashore_in_hex",
                            value: Fixed::from_int(i64::from(men)),
                        },
                        Factor {
                            name: "in_proportion",
                            value: Fixed::from_int(
                                i64::from(loss) * i64::from(*unit_men) / i64::from(men),
                            ),
                        },
                    ],
                    result: Fixed::from_int(i64::from(part)),
                });
                let lost = self.events.record(
                    day,
                    Some(pulse),
                    EventKind::MenLostUnfed {
                        unit: *unit,
                        hex,
                        lost: part,
                        ashore: left,
                    },
                    vec![Cause::Event(fed_event)],
                    breakdown,
                );
                self.destroy_if_no_men(day, Some(pulse), *unit, lost);
            }
        }
    }

    /// Records a land unit destroyed if it has no men ashore or aboard any
    /// ship (decision 0066, 1), citing the event that took its last men.
    pub(crate) fn destroy_if_no_men(
        &mut self,
        day: u32,
        pulse: Option<Pulse>,
        unit: LandUnitId,
        cause: EventId,
    ) {
        let ashore = self
            .truth
            .land_units
            .get(&unit)
            .map_or(0, |land_unit| land_unit.men_ashore);
        if ashore > 0 || self.truth.men_aboard(unit) > 0 {
            return;
        }
        self.events.record(
            day,
            pulse,
            EventKind::LandUnitDestroyed { unit },
            vec![Cause::Event(cause)],
            None,
        );
    }

    /// The men aboard a ship that sank are lost with her (decision 0066,
    /// 15), by an event caused by the sinking.
    pub(crate) fn lose_men_with_ship(
        &mut self,
        day: u32,
        pulse: Pulse,
        ship: ShipId,
        sunk: EventId,
    ) {
        let Some(troops) = self
            .truth
            .ships
            .get_mut(&ship)
            .and_then(|record| record.troops.take())
        else {
            return;
        };
        let lost = self.events.record(
            day,
            Some(pulse),
            EventKind::MenLostWithShip {
                unit: troops.unit,
                ship,
                men: troops.men,
            },
            vec![Cause::Event(sunk)],
            None,
        );
        self.destroy_if_no_men(day, Some(pulse), troops.unit, lost);
    }
}

#[cfg(test)]
mod tests {
    use super::take_in_proportion;
    use super::*;

    #[test]
    fn the_short_stock_of_the_record_takes_21_and_9() {
        // Decision 0066, "A small worked case": 30 from 700 and 300.
        assert_eq!(take_in_proportion(30, &[700, 300]), vec![21, 9]);
    }

    #[test]
    fn the_men_left_over_go_one_from_each_unit_in_unit_order() {
        // Scenario 21, day 6: 58 from 585, 390 and 195 is 29, 19 and 9 with
        // one left over, from the first unit.
        assert_eq!(take_in_proportion(58, &[585, 390, 195]), vec![30, 19, 9]);
        assert_eq!(take_in_proportion(55, &[555, 371, 186]), vec![28, 18, 9]);
    }

    #[test]
    fn a_part_never_exceeds_a_units_men_and_the_parts_add_to_the_loss() {
        for (loss, men) in [
            (5, vec![1, 1, 1, 1, 1, 1]),
            (3, vec![1, 0, 1]),
            (7, vec![2, 2, 3]),
            (10, vec![3, 3]),
        ] {
            let parts = take_in_proportion(loss, &men);
            let total: u32 = men.iter().sum();
            assert_eq!(parts.iter().sum::<u32>(), loss.min(total));
            for (part, m) in parts.iter().zip(&men) {
                assert!(part <= m);
            }
        }
    }

    // ---- Men lost with a ship that sinks (decision 0066, 15)

    use crate::damage::{DamageSite, HitSet};
    use crate::events::DamageTrack;
    use crate::ids::{ShipClassId, TaskForceId};
    use crate::map::{HexData, Map};
    use crate::refdata::{GroundFigures, RefData, ShipClass, ShipKind};
    use crate::side_events::SideEventKind;
    use crate::state::{LandUnit, Ship, ShipLocation, TaskForce, Troops, TruthState};

    const AT: Hex = Hex::new(0, 0);

    fn fx(n: i64) -> Fixed {
        Fixed::from_int(n)
    }

    /// A transport with room for 600 men.
    fn transport() -> ShipClass {
        ShipClass {
            id: ShipClassId::new(1),
            name: "Transport".to_string(),
            kind: ShipKind::Transport,
            cruise_speed_kn: fx(10),
            max_speed_kn: fx(12),
            fuel_capacity_t: fx(1000),
            endurance_nm: fx(10000),
            cargo_capacity_t: fx(3000),
            main_guns: 0,
            gun_rounds_per_min: Fixed::ZERO,
            gun_rounds_per_gun: 0,
            gun_damage: Fixed::ZERO,
            torpedo_tubes: 0,
            torpedo_tubes_one_side: 0,
            torpedo_damage: Fixed::ZERO,
            toughness: fx(100),
            radar: false,
            aircraft_capacity: 0,
            anti_aircraft: 0,
            troop_capacity_men: 600,
        }
    }

    /// An Allied force 1 at (0,0) with ship 1 carrying `aboard` men of unit
    /// 1, which has `ashore` men ashore at (0,0) besides. Ship 2 is a sister
    /// with nobody aboard.
    fn world_with_troops(aboard: u32, ashore: u32) -> World {
        let mut map = Map::new();
        for q in -1..=1 {
            for r in -1..=1 {
                map.insert(Hex::new(q, r), HexData::open_sea());
            }
        }
        let mut ref_data = RefData::default();
        ref_data.ship_classes.insert(transport().id, transport());
        ref_data.ground = Some(GroundFigures {
            handling_t_per_man: Fixed::ONE,
            daily_supply_t_per_man: Fixed::ONE,
            unfed_loss_share_per_day: Fixed::ONE,
        });
        let mut truth = TruthState::default();
        let mut force = TaskForce::at(
            TaskForceId::new(1),
            "Force 1".to_string(),
            Side::Allied,
            AT,
            crate::orders::Mission::Transport,
        );
        for id in 1..=2 {
            let ship = Ship {
                troops: (id == 1).then_some(Troops {
                    unit: LandUnitId::new(1),
                    men: aboard,
                }),
                ..Ship::new(
                    ShipId::new(id),
                    format!("Ship {id}"),
                    ShipClassId::new(1),
                    Side::Allied,
                    ShipLocation::TaskForce(TaskForceId::new(1)),
                )
            };
            force.ships.insert(ship.id);
            truth.ships.insert(ship.id, ship);
        }
        truth.task_forces.insert(force.id, force);
        truth.land_units.insert(
            LandUnitId::new(1),
            LandUnit {
                id: LandUnitId::new(1),
                name: "Battalion".to_string(),
                side: Side::Allied,
                men_ashore: ashore,
                hex: (ashore > 0).then_some(AT),
            },
        );
        World::new(7, ref_data, map, truth).expect("valid setup")
    }

    /// Sinks ship 1 by the real path: a hit set of enough flotation damage.
    /// Returns the `ShipSunk` event.
    fn sink_ship_1(world: &mut World) -> EventId {
        let cause = world.events.record(
            1,
            Some(Pulse::Night),
            EventKind::BaseAtDayEnd {
                base: BaseId::new(1),
                supply_t: Fixed::ZERO,
                aircraft_ready: 0,
            },
            Vec::new(),
            None,
        );
        let hits = HitSet {
            cause,
            target: ShipId::new(1),
            target_force: TaskForceId::new(1),
            damage_per_hit: fx(100),
            track_hits: {
                let mut hits = [0, 0];
                hits[DamageTrack::Flotation as usize] = 1;
                hits
            },
            system_hits: Vec::new(),
        };
        let applied = world.apply_hit_set(
            DamageSite {
                day: 1,
                pulse: Pulse::Night,
                round: 0,
            },
            &hits,
        );
        assert_eq!(applied.sunk.len(), 1);
        applied.sunk[0].1
    }

    fn log_of(world: &World) -> Vec<crate::events::Event> {
        world.events().iter().cloned().collect()
    }

    #[test]
    fn all_the_men_aboard_a_ship_that_sinks_are_lost_with_her() {
        let mut world = world_with_troops(450, 0);
        let sunk = sink_ship_1(&mut world);

        let events = log_of(&world);
        let lost: Vec<_> = events
            .iter()
            .filter(|event| matches!(event.kind, EventKind::MenLostWithShip { .. }))
            .collect();
        assert_eq!(lost.len(), 1);
        assert_eq!(
            lost[0].kind,
            EventKind::MenLostWithShip {
                unit: LandUnitId::new(1),
                ship: ShipId::new(1),
                men: 450,
            }
        );
        // The loss is caused by the sinking, and nothing else.
        assert_eq!(lost[0].causes, vec![Cause::Event(sunk)]);
        assert!(matches!(
            world.events().get(sunk).expect("the sinking").kind,
            EventKind::ShipSunk { .. }
        ));

        // The ship holds no men; the unit has none aboard anywhere.
        assert_eq!(world.truth.ships[&ShipId::new(1)].troops, None);
        assert_eq!(world.truth.men_aboard(LandUnitId::new(1)), 0);
        assert_eq!(
            world.truth.ships[&ShipId::new(1)].location,
            ShipLocation::Sunk
        );

        // It had no men ashore: it is destroyed, by the loss.
        let destroyed: Vec<_> = events
            .iter()
            .filter(|event| matches!(event.kind, EventKind::LandUnitDestroyed { .. }))
            .collect();
        assert_eq!(destroyed.len(), 1);
        assert_eq!(
            destroyed[0].kind,
            EventKind::LandUnitDestroyed {
                unit: LandUnitId::new(1)
            }
        );
        assert_eq!(destroyed[0].causes, vec![Cause::Event(lost[0].id)]);
        // A unit destroyed keeps its number and has no men.
        assert_eq!(world.truth.land_units[&LandUnitId::new(1)].men_ashore, 0);

        assert!(crate::invariants::check(&world.truth, &world.map, &world.ref_data).is_empty());
    }

    #[test]
    fn a_unit_with_men_ashore_is_not_destroyed_when_a_ship_of_it_sinks() {
        let mut world = world_with_troops(450, 120);
        let sunk = sink_ship_1(&mut world);
        let events = log_of(&world);
        let lost = events
            .iter()
            .find(|event| matches!(event.kind, EventKind::MenLostWithShip { .. }))
            .expect("the loss");
        assert_eq!(lost.causes, vec![Cause::Event(sunk)]);
        assert!(!events
            .iter()
            .any(|event| matches!(event.kind, EventKind::LandUnitDestroyed { .. })));
        let land_unit = &world.truth.land_units[&LandUnitId::new(1)];
        assert_eq!(land_unit.men_ashore, 120);
        assert_eq!(land_unit.hex, Some(AT));
        assert_eq!(world.truth.men_aboard(LandUnitId::new(1)), 0);
        assert!(crate::invariants::check(&world.truth, &world.map, &world.ref_data).is_empty());
    }

    #[test]
    fn a_ship_with_no_men_aboard_that_sinks_loses_none() {
        let mut world = world_with_troops(450, 0);
        // The ship carries nobody: take the men and the unit away first.
        world
            .truth
            .ships
            .get_mut(&ShipId::new(1))
            .expect("ship")
            .troops = None;
        world.truth.land_units.clear();
        sink_ship_1(&mut world);
        assert!(!log_of(&world).iter().any(|event| matches!(
            event.kind,
            EventKind::MenLostWithShip { .. } | EventKind::LandUnitDestroyed { .. }
        )));
    }

    #[test]
    fn only_the_units_own_side_sees_the_men_lost_with_the_ship() {
        let mut world = world_with_troops(450, 0);
        sink_ship_1(&mut world);

        let names_men = |side: Side| {
            world
                .side_view(side)
                .events()
                .into_iter()
                .filter_map(|event| match event.kind {
                    SideEventKind::Whole(kind) => Some(kind),
                    _ => None,
                })
                .any(|kind| {
                    matches!(
                        kind,
                        EventKind::MenLostWithShip { .. } | EventKind::LandUnitDestroyed { .. }
                    )
                })
        };
        assert!(names_men(Side::Allied));
        assert!(!names_men(Side::Japanese));
        // Nor does the other side hold the unit.
        assert_eq!(world.side_view(Side::Japanese).land_units().count(), 0);
        assert_eq!(
            world
                .side_view(Side::Japanese)
                .men_aboard(LandUnitId::new(1)),
            None
        );
    }
}
