//! The fight ashore, a land unit's fatigue, a side beaten in a hex and a
//! base that changes side
//! (`docs/decisions/0090-the-fight-ashore-and-a-base-that-changes-side.md`).
//!
//! The ground combat phase runs after the third pulse of the day and before
//! the day's base count, and only when the ground figures are loaded. Hexes
//! with men ashore are taken in hex order. In each: the day's fight if the
//! hex is contested, then each unit's fatigue, the tired units back to
//! defend, the test of a side beaten, and the bases of a beaten side changing
//! side. A hex not contested where a side attacks an enemy base with no men
//! has no fight, and the base falls (decision 16).
//!
//! Draws: one stream for each firing unit, keyed on the day, no pulse, the
//! ground combat subsystem, `Draw::GroundFire` and the unit. Both sides fire
//! on the strengths they had at the start of the fight, so the order of
//! firing does not matter.
//!
//! Reads: each land unit's men ashore, hex, side, posture, fatigue and fed
//! share; the bases in a hex, their side and stocks; the air groups at a
//! base; a ship in port at it; a force's home port; an air group's mission;
//! the ground figures. Writes: a land unit's men, posture, fatigue; a base's
//! side, supply, fuel and whether it is on the map; an air group's aircraft,
//! pilots and place; a ship sunk; a force's home port; an air group's
//! mission; and the events.

use std::collections::{BTreeMap, BTreeSet};

use crate::damage::roll_shots;
use crate::events::{Breakdown, Cause, EventKind, Factor, LandFatigueChange};
use crate::fixed::Fixed;
use crate::ground::take_in_proportion;
use crate::hex::Hex;
use crate::ids::{AirGroupId, BaseId, EntityRef, EventId, LandUnitId, ShipId, TaskForceId};
use crate::orders::{AirMission, Posture};
use crate::refdata::GroundFigures;
use crate::rng::{Draw, Subsystem};
use crate::state::{AirGroupLocation, ShipLocation, Side};
use crate::world::World;

/// The other side.
const fn other(side: Side) -> Side {
    match side {
        Side::Allied => Side::Japanese,
        Side::Japanese => Side::Allied,
    }
}

/// Index of a side in a pair, Allied first.
const fn at(side: Side) -> usize {
    match side {
        Side::Allied => 0,
        Side::Japanese => 1,
    }
}

/// A unit's firing men (decision 6): its men ashore, times its fed share,
/// times 100 less its fatigue over 100, cut to a whole man. Returns the
/// firing men and the fatigue factor.
pub(crate) fn firing_men(men_ashore: u32, fed_share: Fixed, fatigue: Fixed) -> (u32, Fixed) {
    let hundred = Fixed::from_int(100);
    let fatigue_factor = (hundred - fatigue) / hundred;
    let firing = (Fixed::from_int(i64::from(men_ashore)) * fed_share)
        .mul_div(hundred - fatigue, hundred)
        .trunc()
        .max(0);
    (
        u32::try_from(firing).expect("firing men are at most the men ashore"),
        fatigue_factor,
    )
}

/// Whether a side with `men` ashore is beaten by one with `enemy_men`
/// (decision 15): it has no man, or fewer than `beaten_share` times the
/// other side's men.
pub(crate) fn is_beaten(men: u32, enemy_men: u32, beaten_share: Fixed) -> bool {
    men == 0 || Fixed::from_int(i64::from(men)) < beaten_share * i64::from(enemy_men)
}

/// One side's loss in a fight: the side, each unit with its men at the start
/// and its part, the loss in the hex and the side's men there at the start.
type SideLoss = (Side, Vec<(LandUnitId, u32, u32)>, u32, u32);

/// One hex's units with men ashore at the start of the phase, by side.
#[derive(Default)]
struct HexUnits {
    units: [Vec<LandUnitId>; 2],
}

impl World {
    /// The ground combat phase (decision 0090). Does nothing without the
    /// ground figures.
    pub(crate) fn fight_ashore(&mut self, day: u32) {
        let Some(figures) = self.ref_data.ground else {
            return;
        };
        let mut hexes: BTreeMap<Hex, HexUnits> = BTreeMap::new();
        for unit in self.truth.land_units.values() {
            if let (Some(hex), true) = (unit.hex, unit.men_ashore > 0) {
                hexes.entry(hex).or_default().units[at(unit.side)].push(unit.id);
            }
        }
        for (hex, units) in hexes {
            self.resolve_hex(day, hex, &units, &figures);
        }
    }

    fn men_ashore(&self, units: &[LandUnitId]) -> u32 {
        units
            .iter()
            .map(|unit| self.truth.land_units[unit].men_ashore)
            .sum()
    }

    fn attacks(&self, units: &[LandUnitId]) -> bool {
        units
            .iter()
            .any(|unit| self.truth.land_units[unit].posture == Posture::Attack)
    }

    /// The posture orders of the units on attack, in unit order: what made
    /// the side attack.
    fn attack_orders(&self, units: &[LandUnitId]) -> Vec<Cause> {
        units
            .iter()
            .map(|unit| &self.truth.land_units[unit])
            .filter(|unit| unit.posture == Posture::Attack)
            .filter_map(|unit| unit.posture_order.map(Cause::Order))
            .collect::<BTreeSet<Cause>>()
            .into_iter()
            .collect()
    }

    fn resolve_hex(&mut self, day: u32, hex: Hex, units: &HexUnits, figures: &GroundFigures) {
        let attacks = [self.attacks(&units.units[0]), self.attacks(&units.units[1])];
        let present = [!units.units[0].is_empty(), !units.units[1].is_empty()];
        let contested = present[0] && present[1] && (attacks[0] || attacks[1]);

        let mut fight_event = None;
        let mut fired: BTreeMap<LandUnitId, EventId> = BTreeMap::new();
        if contested {
            let (event, fire_events) = self.fight(day, hex, units, attacks, figures);
            fight_event = Some(event);
            fired = fire_events;
        }

        // Fatigue (decision 12) and the tired back to defend (decision 14),
        // for every unit that still has men ashore, in unit order.
        let mut all: Vec<LandUnitId> = units.units[0]
            .iter()
            .chain(&units.units[1])
            .copied()
            .collect();
        all.sort();
        for unit in all {
            self.tire_or_rest(day, unit, fired.get(&unit).copied(), figures);
        }

        // A side beaten (decisions 15 and 16).
        let men = [
            self.men_ashore(&units.units[0]),
            self.men_ashore(&units.units[1]),
        ];
        // Each side beaten, with the causes of its base changing side: the
        // test that beat it, and what that test cites.
        let mut beaten: Vec<(Side, Vec<Cause>)> = Vec::new();
        if let Some(fight) = fight_event {
            let tested: Vec<Side> = Side::ALL
                .into_iter()
                .filter(|side| attacks[at(other(*side))])
                .filter(|side| {
                    is_beaten(men[at(*side)], men[at(other(*side))], figures.beaten_share)
                })
                .collect();
            // Both can be beaten at once only when neither has a man left;
            // then neither is (As built).
            if tested.len() == 1 {
                let side = tested[0];
                let event = self.record_beaten(
                    day,
                    hex,
                    side,
                    men,
                    true,
                    vec![Cause::Event(fight)],
                    figures,
                );
                beaten.push((side, vec![Cause::Event(event), Cause::Event(fight)]));
            }
        } else {
            for side in Side::ALL {
                let attacker = other(side);
                let defenceless = men[at(side)] == 0
                    && men[at(attacker)] > 0
                    && attacks[at(attacker)]
                    && self
                        .truth
                        .bases
                        .values()
                        .any(|base| base.side == side && base.hex == hex);
                if defenceless {
                    let orders = self.attack_orders(&units.units[at(attacker)]);
                    let event =
                        self.record_beaten(day, hex, side, men, false, orders.clone(), figures);
                    let mut causes = vec![Cause::Event(event)];
                    causes.extend(orders);
                    beaten.push((side, causes));
                }
            }
        }

        // The bases of a beaten side change side (decisions 17 to 23).
        for (side, causes) in beaten {
            let bases: Vec<BaseId> = self
                .truth
                .bases
                .values()
                .filter(|base| base.side == side && base.hex == hex)
                .map(|base| base.id)
                .collect();
            for base in bases {
                self.change_side(day, base, other(side), causes.clone(), figures);
            }
        }
    }

    /// The day's fight in a contested hex (decisions 4 to 10). Returns the
    /// fight's event and, for each unit on attack that fired, its fire
    /// event.
    fn fight(
        &mut self,
        day: u32,
        hex: Hex,
        units: &HexUnits,
        attacks: [bool; 2],
        figures: &GroundFigures,
    ) -> (EventId, BTreeMap<LandUnitId, EventId>) {
        let mut causes = self.attack_orders(&units.units[0]);
        causes.extend(self.attack_orders(&units.units[1]));
        let fight = self.events.record(
            day,
            None,
            EventKind::GroundFight {
                hex,
                allied_attacks: attacks[0],
                japanese_attacks: attacks[1],
            },
            causes,
            None,
        );

        // Each unit fires on the strengths at the start (decisions 6, 7, 9).
        let mut hits = [0u32; 2];
        let mut fire_events: [Vec<EventId>; 2] = [Vec::new(), Vec::new()];
        let mut fired: BTreeMap<LandUnitId, EventId> = BTreeMap::new();
        for side in Side::ALL {
            let fed = self.events.latest_where(|kind| {
                matches!(kind, EventKind::MenFed { side: owner, hex: at_hex, .. }
                    if *owner == side && *at_hex == hex)
            });
            let fed_today = fed.filter(|event| {
                self.events
                    .get(*event)
                    .is_some_and(|event| event.day == day)
            });
            let chance = if attacks[at(side)] {
                figures.hit_chance
            } else {
                figures.defender_hit_chance
            };
            for unit_id in units.units[at(side)].clone() {
                let unit = &self.truth.land_units[&unit_id];
                let (men_ashore, fed_share, fatigue, posture) =
                    (unit.men_ashore, unit.fed_share, unit.fatigue, unit.posture);
                let (firing, fatigue_factor) = firing_men(men_ashore, fed_share, fatigue);
                let shots = Fixed::from_int(i64::from(firing)) / figures.men_per_shot;
                let mut stream = self.rng(
                    day,
                    None,
                    Subsystem::GroundCombat,
                    Draw::GroundFire,
                    EntityRef::LandUnit(unit_id),
                );
                let unit_hits = roll_shots(&mut stream, shots, chance);
                hits[at(side)] += unit_hits;
                let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                    factors: vec![
                        Factor {
                            name: "men_ashore",
                            value: Fixed::from_int(i64::from(men_ashore)),
                        },
                        Factor {
                            name: "fed_share",
                            value: fed_share,
                        },
                        Factor {
                            name: "fatigue_factor",
                            value: fatigue_factor,
                        },
                        Factor {
                            name: "firing_men",
                            value: Fixed::from_int(i64::from(firing)),
                        },
                        Factor {
                            name: "men_per_shot",
                            value: figures.men_per_shot,
                        },
                        Factor {
                            name: "shots",
                            value: shots,
                        },
                        Factor {
                            name: "chance",
                            value: chance,
                        },
                    ],
                    result: shots * chance,
                });
                let mut causes = vec![Cause::Event(fight)];
                causes.extend(fed_today.map(Cause::Event));
                let event = self.events.record(
                    day,
                    None,
                    EventKind::GroundFire {
                        unit: unit_id,
                        hex,
                        firing_men: firing,
                        shots,
                        chance,
                        hits: unit_hits,
                    },
                    causes,
                    breakdown,
                );
                fire_events[at(side)].push(event);
                if posture == Posture::Attack && shots > Fixed::ZERO {
                    fired.insert(unit_id, event);
                }
            }
        }

        // The losses, worked for both sides from the strengths at the start
        // and then taken (decision 9).
        let men_per_hit = u32::try_from(figures.men_per_hit.trunc()).unwrap_or(u32::MAX);
        let mut losses: Vec<SideLoss> = Vec::new();
        for side in Side::ALL {
            let list = &units.units[at(side)];
            let unit_men: Vec<u32> = list
                .iter()
                .map(|unit| self.truth.land_units[unit].men_ashore)
                .collect();
            let total: u32 = unit_men.iter().sum();
            let loss = hits[at(other(side))].saturating_mul(men_per_hit).min(total);
            let parts = take_in_proportion(loss, &unit_men);
            losses.push((
                side,
                list.iter()
                    .zip(unit_men)
                    .zip(parts)
                    .map(|((unit, men), part)| (*unit, men, part))
                    .collect(),
                loss,
                total,
            ));
        }
        for (side, parts, loss, total) in losses {
            let causes: Vec<Cause> = fire_events[at(other(side))]
                .iter()
                .map(|event| Cause::Event(*event))
                .collect();
            for (unit_id, unit_men, part) in parts {
                if part == 0 {
                    continue;
                }
                let unit = self
                    .truth
                    .land_units
                    .get_mut(&unit_id)
                    .expect("the land unit exists");
                unit.men_ashore -= part;
                if unit.men_ashore == 0 {
                    unit.hex = None;
                }
                let left = unit.men_ashore;
                let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                    factors: vec![
                        Factor {
                            name: "loss_in_hex",
                            value: Fixed::from_int(i64::from(loss)),
                        },
                        Factor {
                            name: "unit_men_ashore",
                            value: Fixed::from_int(i64::from(unit_men)),
                        },
                        Factor {
                            name: "men_ashore_in_hex",
                            value: Fixed::from_int(i64::from(total)),
                        },
                        Factor {
                            name: "in_proportion",
                            value: Fixed::from_int(
                                i64::from(loss) * i64::from(unit_men) / i64::from(total),
                            ),
                        },
                    ],
                    result: Fixed::from_int(i64::from(part)),
                });
                let lost = self.events.record(
                    day,
                    None,
                    EventKind::MenLostInFight {
                        unit: unit_id,
                        hex,
                        lost: part,
                        ashore: left,
                    },
                    causes.clone(),
                    breakdown,
                );
                self.destroy_if_no_men(day, None, unit_id, lost);
            }
        }
        (fight, fired)
    }

    /// A unit's fatigue for the day (decision 12), then back to defend if it
    /// is on attack and too tired (decision 14). A unit with no man ashore
    /// keeps its fatigue.
    fn tire_or_rest(
        &mut self,
        day: u32,
        unit_id: LandUnitId,
        fire_event: Option<EventId>,
        figures: &GroundFigures,
    ) {
        let unit = self
            .truth
            .land_units
            .get_mut(&unit_id)
            .expect("the land unit exists");
        if unit.men_ashore == 0 {
            return;
        }
        let before = unit.fatigue;
        let (change, after) = if fire_event.is_some() {
            (
                LandFatigueChange::Gained,
                (before + figures.fatigue_per_attack_day).min(Fixed::from_int(100)),
            )
        } else {
            (
                LandFatigueChange::Rested,
                (before - figures.rest_per_day).max(Fixed::ZERO),
            )
        };
        unit.fatigue = after;
        let mut fatigue_event = None;
        if after != before {
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "fatigue_before",
                        value: before,
                    },
                    Factor {
                        name: match change {
                            LandFatigueChange::Gained => "fatigue_per_attack_day",
                            LandFatigueChange::Rested => "rest_per_day",
                        },
                        value: match change {
                            LandFatigueChange::Gained => figures.fatigue_per_attack_day,
                            LandFatigueChange::Rested => figures.rest_per_day,
                        },
                    },
                ],
                result: after,
            });
            fatigue_event = Some(self.events.record(
                day,
                None,
                EventKind::LandUnitFatigue {
                    unit: unit_id,
                    change,
                    fatigue: after,
                },
                fire_event.map(Cause::Event).into_iter().collect(),
                breakdown,
            ));
        }
        let unit = self
            .truth
            .land_units
            .get_mut(&unit_id)
            .expect("the land unit exists");
        if unit.posture == Posture::Attack && unit.fatigue > figures.attack_fatigue_limit {
            unit.posture = Posture::Defend;
            unit.posture_order = None;
            let fatigue = unit.fatigue;
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "fatigue",
                        value: fatigue,
                    },
                    Factor {
                        name: "attack_fatigue_limit",
                        value: figures.attack_fatigue_limit,
                    },
                ],
                result: fatigue,
            });
            self.events.record(
                day,
                None,
                EventKind::LandUnitBackToDefend {
                    unit: unit_id,
                    fatigue,
                },
                fatigue_event
                    .or(fire_event)
                    .map(Cause::Event)
                    .into_iter()
                    .collect(),
                breakdown,
            );
        }
    }

    // One argument more than clippy likes: the counts of both sides and
    // whether there was a fight are what the event records.
    #[allow(clippy::too_many_arguments)]
    fn record_beaten(
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

        // Air groups at it are lost whole (decision 20).
        let groups: Vec<AirGroupId> = self
            .truth
            .air_groups
            .values()
            .filter(|group| group.location == AirGroupLocation::Base(base_id))
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

#[cfg(test)]
mod tests {
    use super::*;

    fn fx(text: &str) -> Fixed {
        text.parse().expect("a decimal")
    }

    #[test]
    fn firing_men_are_the_men_times_the_fed_share_times_the_rested_share_cut() {
        assert_eq!(firing_men(2000, Fixed::ONE, Fixed::ZERO).0, 2000);
        assert_eq!(firing_men(2000, Fixed::ONE, Fixed::from_int(20)).0, 1600);
        assert_eq!(firing_men(2000, Fixed::ONE, Fixed::from_int(60)).0, 800);
        assert_eq!(firing_men(285, Fixed::ZERO, Fixed::ZERO).0, 0);
        assert_eq!(firing_men(600, fx("0.5"), Fixed::from_int(20)).0, 240);
        // Cut to a whole man.
        assert_eq!(firing_men(7, fx("0.5"), Fixed::ZERO).0, 3);
        assert_eq!(
            firing_men(100, Fixed::ONE, Fixed::from_int(40)).1,
            fx("0.6")
        );
    }

    #[test]
    fn a_side_is_beaten_with_no_man_or_below_the_share_of_the_others_men() {
        let share = fx("0.2");
        assert!(is_beaten(0, 0, share));
        assert!(is_beaten(285, 2000, share));
        assert!(is_beaten(399, 2000, share));
        assert!(!is_beaten(400, 2000, share));
        assert!(!is_beaten(520, 600, share));
    }
}
