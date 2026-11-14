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
//!
//! The module is in files. `fight.rs` is the day's fight and the pure
//! functions it uses; `fatigue.rs` is a unit's fatigue and posture;
//! `defeat.rs` is a side beaten and falling back and a base changing side;
//! `activity.rs` is a unit's day and the choice of it a side can ask. This
//! file holds the phase, the hex driver and what more than one file uses.

mod activity;
mod defeat;
mod fatigue;
mod fight;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};

use crate::events::Cause;
use crate::hex::Hex;
use crate::ids::{BaseId, EventId, LandUnitId};
use crate::orders::{Posture, WhenSpent};
use crate::refdata::GroundFigures;
use crate::state::Side;
use crate::world::World;

pub(crate) use self::activity::{is_with_force, unit_day};
pub(crate) use self::defeat::is_beaten;
pub(crate) use self::fatigue::{fed_where_it_stands, goes_back_to_defend, when_attack_spent};
#[cfg(test)]
pub(crate) use self::fight::experience_chance;

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

/// What a day's fight leaves for the rest of the hex's step.
struct FightOutcome {
    /// The fight's event.
    fight: EventId,
    /// For each unit on attack that fired, its fire event.
    fired: BTreeMap<LandUnitId, EventId>,
    /// For every unit that fired, on attack or not, its fire event.
    fired_any: BTreeMap<LandUnitId, EventId>,
    /// For each side, Allied first, the event of its attack failing, if it
    /// failed (decision 4 of 0105).
    failed: [Option<EventId>; 2],
}

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
        // The marches come first, before the day's fights (decision 0094,
        // 4): a unit that arrives fights in its new hex.
        let marched = self.march_land_units(day, &figures);
        let mut contested_hexes: BTreeSet<Hex> = BTreeSet::new();
        let mut fired: BTreeMap<LandUnitId, EventId> = BTreeMap::new();
        let mut hexes: BTreeMap<Hex, HexUnits> = BTreeMap::new();
        for unit in self.truth.land_units.values() {
            if let (Some(hex), true) = (unit.hex, unit.men_ashore > 0) {
                hexes.entry(hex).or_default().units[at(unit.side)].push(unit.id);
            }
        }
        for (hex, units) in hexes {
            let (contested, fired_here) = self.resolve_hex(day, hex, &units, &marched, &figures);
            if contested {
                contested_hexes.insert(hex);
            }
            fired.extend(fired_here);
        }
        // The day's activity, once every hex is resolved (decision 3b of
        // 0099): loading, rehearsal and training, then the days of fighting.
        self.day_activity(day, &marched, &contested_hexes, &fired, &figures);
        // The day's digging, after the day's fatigue and work (decision 5
        // of 0106); then the day's hits on men are settled.
        self.dig_in(day, &marched, &figures);
        self.truth.hits_on_men.clear();
    }

    /// The men ashore of the fighting units among `units`: the men that
    /// make a hex contested and hold a base (decision 0099, ruling 1).
    /// Working men are not counted.
    fn fighting_men_ashore(&self, units: &[LandUnitId]) -> u32 {
        units
            .iter()
            .map(|unit| &self.truth.land_units[unit])
            .filter(|unit| !unit.working)
            .map(|unit| unit.men_ashore)
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

    fn resolve_hex(
        &mut self,
        day: u32,
        hex: Hex,
        units: &HexUnits,
        marched: &BTreeMap<LandUnitId, EventId>,
        figures: &GroundFigures,
    ) -> (bool, BTreeMap<LandUnitId, EventId>) {
        let attacks = [self.attacks(&units.units[0]), self.attacks(&units.units[1])];
        // Only fighting men make a hex contested (decision 0099, ruling 1).
        let present = [
            self.fighting_men_ashore(&units.units[0]) > 0,
            self.fighting_men_ashore(&units.units[1]) > 0,
        ];
        let contested = present[0] && present[1] && (attacks[0] || attacks[1]);

        let mut fight_event = None;
        // The units on attack that fired (for fatigue), and every unit that
        // fired, whatever its posture (for the day's count of fighting).
        let mut fired: BTreeMap<LandUnitId, EventId> = BTreeMap::new();
        let mut fired_any: BTreeMap<LandUnitId, EventId> = BTreeMap::new();
        let mut failed: [Option<EventId>; 2] = [None, None];
        if contested {
            let outcome = self.fight(day, hex, units, attacks, figures);
            fight_event = Some(outcome.fight);
            fired = outcome.fired;
            fired_any = outcome.fired_any;
            failed = outcome.failed;
        }

        // Fatigue (decision 12), with the fatigue of a failed day (decision
        // 5 of 0105), and the tired back to defend (decision 14), for every
        // unit that still has men ashore, in unit order.
        let mut all: Vec<LandUnitId> = units.units[0]
            .iter()
            .chain(&units.units[1])
            .copied()
            .collect();
        all.sort();
        let mut spent: Vec<(LandUnitId, EventId, Option<WhenSpent>)> = Vec::new();
        for unit in all {
            let side = self.truth.land_units[&unit].side;
            if let Some((back, instruction)) = self.tire_or_rest(
                day,
                unit,
                fired.get(&unit).copied(),
                marched.get(&unit).copied(),
                failed[at(side)],
                figures,
            ) {
                spent.push((unit, back, instruction));
            }
        }

        // What a spent attacker does (decisions 6 and 7 of 0105), in unit
        // order, before the test of a side beaten.
        for (unit, back, instruction) in spent {
            self.attack_spent(day, hex, units, unit, back, instruction, figures);
        }

        // A side beaten (decisions 15 and 16), counting fighting men only:
        // labourers do not hold a base (decision 1 and ruling 1 of 0099).
        let men = [
            self.fighting_men_ashore(&units.units[0]),
            self.fighting_men_ashore(&units.units[1]),
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
            // Its men fall back, or surrender (decision 0104). The first
            // cause is the test that beat the side.
            if let Some(Cause::Event(beaten_event)) = causes.first().copied() {
                self.fall_back(
                    day,
                    hex,
                    side,
                    beaten_event,
                    &units.units[at(side)],
                    figures,
                );
            }
        }

        // The ground reports, after the fight and its losses (decision 13 of
        // 0092).
        self.report_ashore(day, hex, fight_event);
        (contested, fired_any)
    }
}
