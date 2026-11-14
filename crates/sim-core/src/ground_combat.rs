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

use crate::choices::ground::{GroundAnswer, GroundQuestion, UnitDay};
use crate::choices::{Answer, Question};
use crate::damage::roll_shots;
use crate::events::{Breakdown, Cause, EventKind, ExperienceSource, Factor, LandFatigueChange};
use crate::fixed::Fixed;
use crate::ground::take_in_proportion;
use crate::hex::Hex;
use crate::ids::{AirGroupId, BaseId, EntityRef, EventId, LandUnitId, ShipId, TaskForceId};
use crate::orders::{Activity, AirMission, Posture};
use crate::refdata::GroundFigures;
use crate::rng::{Draw, Subsystem};
use crate::state::{AirGroupLocation, LandUnit, ShipLocation, Side, Skills, TruthState};
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

/// A unit's chance to hit and the factor experience gives it (decision 2 of
/// 0097): the side's chance times one plus the strength times the
/// experience less 2, cut to 0 to 1. At a strength of 0 the factor is 1.
#[cfg(test)]
pub(crate) fn experience_chance(
    side_chance: Fixed,
    strength: Fixed,
    experience: u8,
) -> (Fixed, Fixed) {
    skilled_chance(side_chance, strength, experience, Fixed::ONE)
}

/// A unit's chance to hit and the factor its experience gives it, with the
/// factor of its holding skill (decision 2 of 0098; 1 unless its side
/// defends): the side's chance times both factors, cut to 0 to 1. With a
/// holding factor of 1 it is [`experience_chance`].
pub(crate) fn skilled_chance(
    side_chance: Fixed,
    strength: Fixed,
    experience: u8,
    holding_factor: Fixed,
) -> (Fixed, Fixed) {
    let factor = Fixed::ONE + strength * (i64::from(experience) - 2);
    (
        (side_chance * factor * holding_factor)
            .max(Fixed::ZERO)
            .min(Fixed::ONE),
        factor,
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

/// Whether a land unit goes back to defend (decision 14 of 0090): it is on
/// attack and its fatigue is above the limit. Decision 1 of 0100: the
/// engine and a side's view both ask it.
pub(crate) fn goes_back_to_defend(posture: Posture, fatigue: Fixed, limit: Fixed) -> bool {
    posture == Posture::Attack && fatigue > limit
}

/// What a land unit's day is (decision 3b of 0099): none if no man is
/// ashore or aboard; none if it marched or its hex was contested; else work
/// if it is a working unit (always) or was set to work, training if it was
/// set to train, and else, standing ready, rehearsal if it is on the roster
/// of a force that qualified today (`rostered_with`) and `with_force` says
/// it is in that force's hex or aboard its ships. `with_force` is asked only
/// then. Decision 1 of 0100: the engine and a side's view both ask it.
// The inputs are the unit's state and the three facts of the day; a struct
// would only rename them.
#[allow(clippy::too_many_arguments)]
pub(crate) fn unit_day(
    men_ashore: u32,
    men_aboard: u32,
    working: bool,
    activity: Activity,
    marched: bool,
    contested: bool,
    rostered_with: Option<TaskForceId>,
    with_force: impl FnOnce(TaskForceId) -> bool,
) -> UnitDay {
    if men_ashore == 0 && men_aboard == 0 {
        return UnitDay::NoMen;
    }
    if marched {
        return UnitDay::Marched;
    }
    if contested {
        return UnitDay::Contested;
    }
    // A working unit always works (decision 1 of 0099).
    let activity = if working {
        Activity::Work(activity.job())
    } else {
        activity
    };
    match activity {
        Activity::Work(job) => UnitDay::Work(job),
        Activity::Train => UnitDay::Train,
        Activity::Ready => match rostered_with {
            Some(force) if with_force(force) => UnitDay::Rehearse(force),
            _ => UnitDay::Ready,
        },
    }
}

/// Whether every man of `unit_id` is ashore in the hex of `force` or aboard
/// its ships, with at least one man (decision 3b of 0099, and 4 of 0095).
/// Reads only the unit and the force, which a side holds of its own.
pub(crate) fn is_with_force(truth: &TruthState, unit_id: LandUnitId, force: TaskForceId) -> bool {
    let unit = &truth.land_units[&unit_id];
    let force = &truth.task_forces[&force];
    let aboard_here: u32 = force
        .ships
        .iter()
        .filter_map(|ship| truth.ships[ship].troops)
        .filter(|troops| troops.unit == unit_id)
        .map(|troops| troops.men)
        .sum();
    let ashore_ok = unit.men_ashore == 0 || unit.hex == Some(force.from_hex);
    ashore_ok && aboard_here == truth.men_aboard(unit_id) && unit.men_ashore + aboard_here > 0
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
    }

    /// The day's activity, once every hex is resolved (decision 3b of 0099).
    /// First the loading of each force with a preparation; then each land
    /// unit's one activity, in unit order: none if it marched or its hex was
    /// contested, else work if it is a working unit or was set to work
    /// (decisions 3 and 3a of 0099), else training if it was ordered to
    /// train and the conditions of decision 3 of 0097 hold, else rehearsal if
    /// it is on the roster of a force that qualified today and is in its hex
    /// or aboard its ships; then the airfields the day's work built; then the
    /// day's count of fighting for each unit that fired.
    /// Each gain is given once because the phase runs once a day.
    pub(crate) fn day_activity(
        &mut self,
        day: u32,
        marched: &BTreeMap<LandUnitId, EventId>,
        contested_hexes: &BTreeSet<Hex>,
        fired: &BTreeMap<LandUnitId, EventId>,
        figures: &GroundFigures,
    ) {
        let qualifying = self.gain_loading(day, figures);
        let rostered = self.rostered_with(&qualifying);

        let units: Vec<LandUnitId> = self.truth.land_units.keys().copied().collect();
        let mut on_airfields: BTreeMap<BaseId, Vec<EventId>> = BTreeMap::new();
        for unit_id in units {
            let unit = &self.truth.land_units[&unit_id];
            let aboard = self.truth.men_aboard(unit_id);
            // A unit that marched, or whose hex was contested, marches or
            // fights: none of the three.
            let contested =
                unit.men_ashore > 0 && unit.hex.is_some_and(|hex| contested_hexes.contains(&hex));
            let marched_today = marched.contains_key(&unit_id);
            let rostered_with = rostered.get(&unit_id).copied();
            let side = unit.side;
            let answer = unit_day(
                unit.men_ashore,
                aboard,
                unit.working,
                unit.activity,
                marched_today,
                contested,
                rostered_with,
                |force| is_with_force(&self.truth, unit_id, force),
            );
            self.trace_choice(side, || {
                (
                    Question::Ground(GroundQuestion::UnitDay {
                        unit: unit_id,
                        marched: marched_today,
                        contested,
                        rostered_with,
                    }),
                    Answer::Ground(GroundAnswer::UnitDay(answer)),
                )
            });
            match answer {
                UnitDay::NoMen | UnitDay::Marched | UnitDay::Contested | UnitDay::Ready => {}
                UnitDay::Work(job) => self.work_day(day, unit_id, job, figures, &mut on_airfields),
                UnitDay::Train => self.train_day(day, unit_id, figures),
                UnitDay::Rehearse(force) => self.rehearse_day(day, unit_id, force, figures),
            }
        }
        // The airfields the day's work built (decision 5 of 0099).
        self.finish_airfields(day, &on_airfields, figures);

        // The days of fighting (decision 5 of 0097).
        for (unit_id, fire_event) in fired {
            let unit = &self.truth.land_units[unit_id];
            if unit.experience >= LandUnit::MAX_EXPERIENCE
                || (unit.men_ashore == 0 && self.truth.men_aboard(*unit_id) == 0)
            {
                continue;
            }
            let before = unit.fight_days;
            let experience = unit.experience;
            let days = before + 1;
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "days_before",
                        value: Fixed::from_int(i64::from(before)),
                    },
                    Factor {
                        name: "fight_days_per_level",
                        value: Fixed::from_int(i64::from(figures.fight_days_per_level)),
                    },
                ],
                result: Fixed::from_int(i64::from(days)),
            });
            let counted = self.events.record(
                day,
                None,
                EventKind::FightDayCounted {
                    unit: *unit_id,
                    days,
                    experience,
                },
                vec![Cause::Event(*fire_event)],
                breakdown,
            );
            self.count_toward_experience(
                day,
                *unit_id,
                days,
                figures.fight_days_per_level,
                ExperienceSource::Fighting,
                counted,
            );
        }
    }

    /// A day of training for a unit set to train, if the conditions of
    /// decision 3 of 0097 hold: all its men ashore at a base of its side,
    /// fed in full. At or above the ceiling nothing changes and nothing is
    /// recorded: the unit still spent the day.
    fn train_day(&mut self, day: u32, unit_id: LandUnitId, figures: &GroundFigures) {
        let unit = &self.truth.land_units[&unit_id];
        let Some(hex) = unit.hex.filter(|_| unit.men_ashore > 0) else {
            return;
        };
        let side = unit.side;
        let in_base = self
            .truth
            .bases
            .values()
            .any(|base| base.side == side && base.hex == hex);
        if self.truth.men_aboard(unit_id) > 0 || !in_base || unit.fed_share != Fixed::ONE {
            return;
        }
        if u32::from(unit.experience) >= figures.training_ceiling {
            return;
        }
        let before = unit.training_days;
        let experience = unit.experience;
        let days = before + 1;
        let mut causes: Vec<Cause> = unit.activity_order.map(Cause::Order).into_iter().collect();
        let fed = self.events.latest_where(|kind| {
            matches!(kind, EventKind::MenFed { side: owner, hex: at_hex, .. }
                if *owner == side && *at_hex == hex)
        });
        causes.extend(
            fed.filter(|event| {
                self.events
                    .get(*event)
                    .is_some_and(|event| event.day == day)
            })
            .map(Cause::Event),
        );
        let breakdown = self.config.record_breakdowns.then(|| Breakdown {
            factors: vec![
                Factor {
                    name: "days_before",
                    value: Fixed::from_int(i64::from(before)),
                },
                Factor {
                    name: "training_days_per_level",
                    value: Fixed::from_int(i64::from(figures.training_days_per_level)),
                },
            ],
            result: Fixed::from_int(i64::from(days)),
        });
        let counted = self.events.record(
            day,
            None,
            EventKind::TrainingDay {
                unit: unit_id,
                days,
                experience,
            },
            causes,
            breakdown,
        );
        self.count_toward_experience(
            day,
            unit_id,
            days,
            figures.training_days_per_level,
            ExperienceSource::Training,
            counted,
        );
    }

    /// Stores a day counted toward a step of experience (`days` is the count
    /// after the day); at `per_level` the experience rises by one and the
    /// count starts again, with `ExperienceRaised` citing the event of the
    /// day.
    fn count_toward_experience(
        &mut self,
        day: u32,
        unit_id: LandUnitId,
        days: u32,
        per_level: u32,
        source: ExperienceSource,
        counted: EventId,
    ) {
        let unit = self
            .truth
            .land_units
            .get_mut(&unit_id)
            .expect("the land unit exists");
        let raised = days >= per_level;
        let (stored, experience) = if raised {
            (0, unit.experience + 1)
        } else {
            (days, unit.experience)
        };
        match source {
            ExperienceSource::Training => unit.training_days = stored,
            ExperienceSource::Fighting => unit.fight_days = stored,
        }
        unit.experience = experience;
        if raised {
            self.events.record(
                day,
                None,
                EventKind::ExperienceRaised {
                    unit: unit_id,
                    experience,
                    by: source,
                },
                vec![Cause::Event(counted)],
                None,
            );
        }
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
        if contested {
            let (event, fire_events, every_fire_event) =
                self.fight(day, hex, units, attacks, figures);
            fight_event = Some(event);
            fired = fire_events;
            fired_any = every_fire_event;
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
            self.tire_or_rest(
                day,
                unit,
                fired.get(&unit).copied(),
                marched.get(&unit).copied(),
                figures,
            );
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
        }

        // The ground reports, after the fight and its losses (decision 13 of
        // 0092).
        self.report_ashore(day, hex, fight_event);
        (contested, fired_any)
    }

    /// The day's fight in a contested hex (decisions 4 to 10). Returns the
    /// fight's event, for each unit on attack that fired its fire event, and
    /// for every unit that fired, on attack or not, its fire event.
    #[allow(clippy::type_complexity)]
    fn fight(
        &mut self,
        day: u32,
        hex: Hex,
        units: &HexUnits,
        attacks: [bool; 2],
        figures: &GroundFigures,
    ) -> (
        EventId,
        BTreeMap<LandUnitId, EventId>,
        BTreeMap<LandUnitId, EventId>,
    ) {
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
        let mut fired_any: BTreeMap<LandUnitId, EventId> = BTreeMap::new();
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
            let defends = !attacks[at(side)];
            let side_chance = if defends {
                figures.defender_hit_chance
            } else {
                figures.hit_chance
            };
            // While a side defends, its units' chance is multiplied by its
            // defences: those of its lowest-numbered base in the hex
            // (decision 4 of 0099).
            let defences = if defends {
                self.truth
                    .bases
                    .values()
                    .find(|base| base.side == side && base.hex == hex)
                    .map_or(Fixed::ZERO, |base| base.defences)
            } else {
                Fixed::ZERO
            };
            let defences_factor = Fixed::ONE
                + figures
                    .defences_strength
                    .mul_div(defences, Fixed::from_int(100));
            for unit_id in units.units[at(side)].clone() {
                let unit = &self.truth.land_units[&unit_id];
                // Working men do not fire (decision 1 of 0099).
                if unit.working {
                    continue;
                }
                let (men_ashore, fed_share, fatigue, posture, experience, skills) = (
                    unit.men_ashore,
                    unit.fed_share,
                    unit.fatigue,
                    unit.posture,
                    unit.experience,
                    unit.skills,
                );
                // The unit's chance: the side's, times one plus the strength
                // times the unit's experience less 2 (decision 2 of 0097),
                // times its holding factor while its side defends (decision 2
                // of 0098), from 0 to 1.
                let holding_factor = if defends {
                    Skills::factor(skills.holding, figures.holding_skill_strength)
                } else {
                    Fixed::ONE
                };
                let (chance, experience_factor) = skilled_chance(
                    side_chance,
                    figures.experience_fire_strength,
                    experience,
                    holding_factor * defences_factor,
                );
                let (firing, fatigue_factor) = firing_men(men_ashore, fed_share, fatigue);
                // Its shots, times its weight factor (decision 2 of 0098).
                let weight_factor = Skills::factor(skills.weight, figures.weight_strength);
                let shots =
                    Fixed::from_int(i64::from(firing)) / figures.men_per_shot * weight_factor;
                let mut stream = self.rng(
                    day,
                    None,
                    Subsystem::GroundCombat,
                    Draw::GroundFire,
                    EntityRef::LandUnit(unit_id),
                );
                let unit_hits = roll_shots(&mut stream, shots, chance);
                hits[at(side)] += unit_hits;
                let breakdown = self.config.record_breakdowns.then(|| {
                    let mut factors = vec![
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
                    ];
                    // Each skill only where it acts, so a breakdown of a
                    // standard unit is as it was (decision 0098).
                    if weight_factor != Fixed::ONE {
                        factors.extend(crate::march::weight_factors(skills.weight, weight_factor));
                    }
                    factors.push(Factor {
                        name: "shots",
                        value: shots,
                    });
                    // Only where experience or holding does something, so a
                    // breakdown with the strengths at 0 is as it was.
                    let holds = holding_factor != Fixed::ONE;
                    let dug_in = defences_factor != Fixed::ONE;
                    if figures.experience_fire_strength > Fixed::ZERO || holds || dug_in {
                        factors.push(Factor {
                            name: "side_chance",
                            value: side_chance,
                        });
                    }
                    if figures.experience_fire_strength > Fixed::ZERO {
                        factors.push(Factor {
                            name: "experience",
                            value: Fixed::from_int(i64::from(experience)),
                        });
                        factors.push(Factor {
                            name: "experience_factor",
                            value: experience_factor,
                        });
                    }
                    if holds {
                        factors.push(Factor {
                            name: "holding_skill",
                            value: Fixed::from_int(i64::from(skills.holding)),
                        });
                        factors.push(Factor {
                            name: "holding_factor",
                            value: holding_factor,
                        });
                    }
                    // Only where defences act (decision 4 of 0099).
                    if dug_in {
                        factors.push(Factor {
                            name: "defences",
                            value: defences,
                        });
                        factors.push(Factor {
                            name: "defences_factor",
                            value: defences_factor,
                        });
                    }
                    factors.push(Factor {
                        name: "chance",
                        value: chance,
                    });
                    Breakdown {
                        factors,
                        result: shots * chance,
                    }
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
                if shots > Fixed::ZERO {
                    fired_any.insert(unit_id, event);
                    if posture == Posture::Attack {
                        fired.insert(unit_id, event);
                    }
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
        (fight, fired, fired_any)
    }

    /// A unit's fatigue for the day (decision 12), then back to defend if it
    /// is on attack and too tired (decision 14). A unit that marched gains
    /// the march's fatigue in place of resting, and one that marched and
    /// fired on attack gains both (decision 8 of 0094). A unit with no man
    /// ashore keeps its fatigue.
    fn tire_or_rest(
        &mut self,
        day: u32,
        unit_id: LandUnitId,
        fire_event: Option<EventId>,
        march_event: Option<EventId>,
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
        let hundred = Fixed::from_int(100);
        let mut factors = vec![Factor {
            name: "fatigue_before",
            value: before,
        }];
        let change = match (fire_event.is_some(), march_event.is_some()) {
            (true, false) => LandFatigueChange::Gained,
            (false, true) => LandFatigueChange::Marched,
            (true, true) => LandFatigueChange::MarchedAndAttacked,
            (false, false) => LandFatigueChange::Rested,
        };
        let after = if change == LandFatigueChange::Rested {
            factors.push(Factor {
                name: "rest_per_day",
                value: figures.rest_per_day,
            });
            (before - figures.rest_per_day).max(Fixed::ZERO)
        } else {
            let mut gained = Fixed::ZERO;
            if march_event.is_some() {
                factors.push(Factor {
                    name: "fatigue_per_march_day",
                    value: figures.fatigue_per_march_day,
                });
                gained += figures.fatigue_per_march_day;
            }
            if fire_event.is_some() {
                factors.push(Factor {
                    name: "fatigue_per_attack_day",
                    value: figures.fatigue_per_attack_day,
                });
                gained += figures.fatigue_per_attack_day;
            }
            (before + gained).min(hundred)
        };
        unit.fatigue = after;
        let mut fatigue_event = None;
        if after != before {
            let breakdown = self.config.record_breakdowns.then_some(Breakdown {
                factors,
                result: after,
            });
            let causes: Vec<Cause> = fire_event
                .into_iter()
                .chain(march_event)
                .map(Cause::Event)
                .collect();
            fatigue_event = Some(self.events.record(
                day,
                None,
                EventKind::LandUnitFatigue {
                    unit: unit_id,
                    change,
                    fatigue: after,
                },
                causes,
                breakdown,
            ));
        }
        let unit = &self.truth.land_units[&unit_id];
        let (side, posture, fatigue) = (unit.side, unit.posture, unit.fatigue);
        let back = goes_back_to_defend(posture, fatigue, figures.attack_fatigue_limit);
        self.trace_choice(side, || {
            (
                Question::Ground(GroundQuestion::BackToDefend {
                    unit: unit_id,
                    fatigue,
                }),
                Answer::Ground(GroundAnswer::BackToDefend(back)),
            )
        });
        if back {
            let unit = self
                .truth
                .land_units
                .get_mut(&unit_id)
                .expect("the land unit exists");
            unit.posture = Posture::Defend;
            unit.posture_order = None;
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

    // ---- A base that changes side, by hand-built worlds (decisions 20 to
    // 23 of 0090). Each world is Japanese base 1 at AT with nobody to
    // defend it, and 1,000 Allied men ashore there on attack: the base falls
    // without a fight in the ground combat phase.

    use crate::events::Event;
    use crate::ids::{AircraftTypeId, OrderId, ShipClassId};
    use crate::invariants;
    use crate::map::{HexData, Map};
    use crate::orders::{
        AirGroupOrder, Mission, OrderKind, OrderRequest, Origin, Reach, RejectReason,
    };
    use crate::refdata::{
        AircraftRole, AircraftType, AttackWeapon, PortSize, RefData, SearchFigures, ShipClass,
        ShipKind,
    };
    use crate::state::{
        AirGroup, Base, LandUnit, Replacement, Ship, TaskForce, Troops, TruthState, YardTrip,
    };

    /// The contested hex, with Japanese base 1; Japanese base 2 and Allied
    /// base 3 are elsewhere.
    const AT: Hex = Hex::new(0, 0);
    const ELSEWHERE: Hex = Hex::new(4, 0);
    const FAR: Hex = Hex::new(8, 0);

    fn n(whole: i64) -> Fixed {
        Fixed::from_int(whole)
    }

    fn bid(id: u32) -> BaseId {
        BaseId::new(id)
    }

    fn gid(id: u32) -> AirGroupId {
        AirGroupId::new(id)
    }

    fn lid(id: u32) -> LandUnitId {
        LandUnitId::new(id)
    }

    fn sid(id: u32) -> ShipId {
        ShipId::new(id)
    }

    fn tid(id: u32) -> TaskForceId {
        TaskForceId::new(id)
    }

    fn escort_class() -> ShipClass {
        ShipClass {
            id: ShipClassId::new(1),
            name: "Test ship".to_string(),
            kind: ShipKind::Cruiser,
            cruise_speed_kn: n(16),
            max_speed_kn: n(32),
            fuel_capacity_t: n(1000),
            endurance_nm: n(8000),
            cargo_capacity_t: Fixed::ZERO,
            main_guns: 0,
            gun_rounds_per_min: Fixed::ZERO,
            gun_rounds_per_gun: 0,
            gun_damage: Fixed::ZERO,
            torpedo_tubes: 0,
            torpedo_tubes_one_side: 0,
            torpedo_damage: Fixed::ZERO,
            toughness: n(100),
            radar: false,
            aircraft_capacity: 20,
            anti_aircraft: 0,
            troop_capacity_men: 600,
        }
    }

    fn fighter_type() -> AircraftType {
        AircraftType {
            id: AircraftTypeId::new(5),
            name: "Test fighter".to_string(),
            role: AircraftRole::Fighter,
            search_range_nm: Fixed::ZERO,
            sweep_width_nm: Fixed::ZERO,
            strike_range_nm: n(250),
            attack_weapon: AttackWeapon::None,
            attack_damage: Fixed::ZERO,
            pilot_armour: false,
            self_sealing_tanks: false,
        }
    }

    fn air_group(
        id: u32,
        side: Side,
        location: AirGroupLocation,
        ready: u32,
        damaged: u32,
        pilots: Option<u32>,
    ) -> AirGroup {
        AirGroup {
            ready,
            damaged,
            pilots,
            strength: ready + damaged + 2,
            ..AirGroup::new(
                gid(id),
                format!("Air group {id}"),
                side,
                location,
                AircraftTypeId::new(5),
            )
        }
    }

    /// The world: Japanese base 1 (60 t of supply, 40 t of fuel) at AT, with
    /// no Japanese man there; Japanese base 2 at ELSEWHERE; Allied base 3 at
    /// FAR. Allied unit 1, 1,000 men on attack at AT.
    ///
    /// Japanese: air group 1 at base 1 (9 ready, 2 damaged, 10 pilots); ship
    /// 1 in port at base 1, with air group 7 aboard (6 ready, 1 damaged, 5
    /// pilots) and 150 men of unit 2 (all it has); task force 1 at sea with
    /// ship 2 and base 1 as its home port; task force 2 at sea with ship 3
    /// and base 2 as its home port.
    ///
    /// Allied: air group 5 at base 3 on a strike on base 1; air group 6 at
    /// base 3 on a strike on base 2.
    fn world() -> World {
        let mut map = Map::new();
        for q in -1..=9 {
            for r in -1..=1 {
                map.insert(Hex::new(q, r), HexData::open_sea());
            }
        }
        let mut ref_data = RefData::default();
        ref_data
            .ship_classes
            .insert(escort_class().id, escort_class());
        ref_data
            .aircraft_types
            .insert(fighter_type().id, fighter_type());
        ref_data.search = Some(SearchFigures::from_search_toml());
        ref_data.port_sizes.insert(
            0,
            PortSize {
                berths: 0,
                berth_rate_t_per_h: Fixed::ZERO,
                lighter_rate_t_per_h: n(20),
            },
        );
        ref_data.ground = Some(GroundFigures::FIRST);

        let mut truth = TruthState::default();
        for (id, side, hex, supply, fuel) in [
            (1, Side::Japanese, AT, 60, 40),
            (2, Side::Japanese, ELSEWHERE, 10, 0),
            (3, Side::Allied, FAR, 100, 0),
        ] {
            truth.bases.insert(
                bid(id),
                Base {
                    supply_t: n(supply),
                    fuel_t: n(fuel),
                    ..Base::new(bid(id), format!("Base {id}"), side, hex)
                },
            );
        }
        let mut unit = LandUnit::new(
            lid(1),
            "Allied unit".to_string(),
            Side::Allied,
            1000,
            Some(AT),
        );
        unit.posture = Posture::Attack;
        truth.land_units.insert(unit.id, unit);
        truth.land_units.insert(
            lid(2),
            LandUnit::new(lid(2), "Japanese unit".to_string(), Side::Japanese, 0, None),
        );
        for (id, home, hex, ship) in [(1, 1, Hex::new(2, 0), 2), (2, 2, Hex::new(5, 0), 3)] {
            let mut force = TaskForce::at(
                tid(id),
                format!("Force {id}"),
                Side::Japanese,
                hex,
                Mission::Escort,
            );
            force.home_port = Some(bid(home));
            force.ships.insert(sid(ship));
            truth.task_forces.insert(force.id, force);
            truth.ships.insert(
                sid(ship),
                Ship::new(
                    sid(ship),
                    format!("Ship {ship}"),
                    ShipClassId::new(1),
                    Side::Japanese,
                    ShipLocation::TaskForce(tid(id)),
                ),
            );
        }
        truth.ships.insert(
            sid(1),
            Ship {
                troops: Some(Troops {
                    unit: lid(2),
                    men: 150,
                }),
                ..Ship::new(
                    sid(1),
                    "Ship in port".to_string(),
                    ShipClassId::new(1),
                    Side::Japanese,
                    ShipLocation::Port(bid(1)),
                )
            },
        );
        for group in [
            air_group(
                1,
                Side::Japanese,
                AirGroupLocation::Base(bid(1)),
                9,
                2,
                Some(10),
            ),
            air_group(
                7,
                Side::Japanese,
                AirGroupLocation::Ship(sid(1)),
                6,
                1,
                Some(5),
            ),
            AirGroup {
                mission: AirMission::BaseStrike(bid(1)),
                mission_order: Some(OrderId::new(1)),
                ..air_group(5, Side::Allied, AirGroupLocation::Base(bid(3)), 6, 0, None)
            },
            AirGroup {
                mission: AirMission::BaseStrike(bid(2)),
                mission_order: Some(OrderId::new(2)),
                ..air_group(6, Side::Allied, AirGroupLocation::Base(bid(3)), 6, 0, None)
            },
        ] {
            truth.air_groups.insert(group.id, group);
        }
        let world = World::new(7, ref_data, map, truth).expect("valid setup");
        assert!(invariants::check(&world.truth, &world.map, &world.ref_data).is_empty());
        world
    }

    fn all_events(world: &World) -> Vec<Event> {
        world.events().iter().cloned().collect()
    }

    fn sound(world: &World) {
        let violations = invariants::check(&world.truth, &world.map, &world.ref_data);
        assert!(violations.is_empty(), "{violations:?}");
    }

    /// The world after the ground combat phase of day 1.
    fn fallen() -> World {
        let mut world = world();
        world.fight_ashore(1);
        sound(&world);
        world
    }

    fn one(events: &[Event], pick: impl Fn(&EventKind) -> bool) -> &Event {
        let found: Vec<&Event> = events.iter().filter(|e| pick(&e.kind)).collect();
        assert_eq!(found.len(), 1, "{found:?}");
        found[0]
    }

    #[test]
    fn a_base_with_no_man_falls_to_a_unit_on_attack_with_half_its_stocks() {
        let world = fallen();
        let events = all_events(&world);
        let beaten = one(&events, |k| matches!(k, EventKind::SideBeatenAshore { .. }));
        assert_eq!(
            beaten.kind,
            EventKind::SideBeatenAshore {
                side: Side::Japanese,
                hex: AT,
                men: 0,
                enemy_men: 1000,
                fought: false,
            }
        );
        let changed = one(&events, |k| matches!(k, EventKind::BaseChangedSide { .. }));
        assert_eq!(
            changed.kind,
            EventKind::BaseChangedSide {
                base: bid(1),
                hex: AT,
                from: Side::Japanese,
                to: Side::Allied,
                supply_t: n(30),
                fuel_t: n(20),
                supply_lost_t: n(30),
                fuel_lost_t: n(20),
            }
        );
        assert!(changed.causes.contains(&Cause::Event(beaten.id)));
        let base = &world.truth.bases[&bid(1)];
        assert_eq!(
            (base.side, base.supply_t, base.fuel_t, base.on_map),
            (Side::Allied, n(30), n(20), true)
        );
        // The base at another hex is not touched.
        assert_eq!(world.truth.bases[&bid(2)].side, Side::Japanese);
        assert_eq!(world.truth.bases[&bid(2)].supply_t, n(10));
    }

    #[test]
    fn a_force_of_the_beaten_side_whose_home_port_it_was_has_none_and_another_keeps_its_own() {
        let world = fallen();
        let events = all_events(&world);
        let changed = one(&events, |k| matches!(k, EventKind::BaseChangedSide { .. }));
        let lost = one(&events, |k| matches!(k, EventKind::HomePortLost { .. }));
        assert_eq!(
            lost.kind,
            EventKind::HomePortLost {
                task_force: tid(1),
                base: bid(1),
            }
        );
        assert_eq!(lost.causes, vec![Cause::Event(changed.id)]);
        assert_eq!(world.truth.task_forces[&tid(1)].home_port, None);
        assert_eq!(world.truth.task_forces[&tid(2)].home_port, Some(bid(2)));
    }

    #[test]
    fn a_winning_group_on_a_strike_at_the_base_rests_and_one_on_another_base_is_untouched() {
        let world = fallen();
        let events = all_events(&world);
        let changed = one(&events, |k| matches!(k, EventKind::BaseChangedSide { .. }));
        let cleared = one(&events, |k| {
            matches!(k, EventKind::StrikeMissionCleared { .. })
        });
        assert_eq!(
            cleared.kind,
            EventKind::StrikeMissionCleared {
                air_group: gid(5),
                base: bid(1),
            }
        );
        assert_eq!(cleared.causes, vec![Cause::Event(changed.id)]);
        let five = &world.truth.air_groups[&gid(5)];
        assert_eq!((five.mission, five.mission_order), (AirMission::Rest, None));
        let six = &world.truth.air_groups[&gid(6)];
        assert_eq!(
            (six.mission, six.mission_order),
            (AirMission::BaseStrike(bid(2)), Some(OrderId::new(2)))
        );
        assert!(!events.iter().any(|e| matches!(&e.kind,
            EventKind::StrikeMissionCleared { air_group, .. } if *air_group == gid(6))));
    }

    #[test]
    fn an_air_group_at_the_base_is_lost_whole_with_its_pilots() {
        let world = fallen();
        let events = all_events(&world);
        let changed = one(&events, |k| matches!(k, EventKind::BaseChangedSide { .. }));
        let lost = one(&events, |k| {
            matches!(k, EventKind::AirGroupLostWithBase { .. })
        });
        assert_eq!(
            lost.kind,
            EventKind::AirGroupLostWithBase {
                air_group: gid(1),
                base: bid(1),
                aircraft: 11,
            }
        );
        assert_eq!(lost.causes, vec![Cause::Event(changed.id)]);
        let group = &world.truth.air_groups[&gid(1)];
        assert_eq!(group.location, AirGroupLocation::Lost);
        assert_eq!((group.ready, group.damaged, group.pilots), (0, 0, Some(0)));
        let pilots = events
            .iter()
            .find(|e| {
                matches!(&e.kind, EventKind::PilotsLost { air_group, .. } if *air_group == gid(1))
            })
            .expect("the pilots are lost");
        assert_eq!(
            pilots.kind,
            EventKind::PilotsLost {
                air_group: gid(1),
                lost: 10,
                left: 0,
            }
        );
        assert_eq!(pilots.causes, vec![Cause::Event(lost.id)]);
        // Allied groups elsewhere are not touched.
        assert_eq!(
            world.truth.air_groups[&gid(5)].location,
            AirGroupLocation::Base(bid(3))
        );
    }

    #[test]
    fn a_group_with_no_pilots_counted_is_lost_with_no_pilot_event() {
        let mut world = world();
        world.truth.air_groups.get_mut(&gid(1)).unwrap().pilots = None;
        world.fight_ashore(1);
        let events = all_events(&world);
        assert_eq!(world.truth.air_groups[&gid(1)].pilots, None);
        assert!(!events.iter().any(|e| matches!(&e.kind,
            EventKind::PilotsLost { air_group, .. } if *air_group == gid(1))));
        assert_eq!(
            world.truth.air_groups[&gid(1)].location,
            AirGroupLocation::Lost
        );
        sound(&world);
    }

    #[test]
    fn a_ship_in_port_is_sunk_with_the_group_and_the_men_aboard_her() {
        let world = fallen();
        let events = all_events(&world);
        let changed = one(&events, |k| matches!(k, EventKind::BaseChangedSide { .. }));
        let sunk = one(&events, |k| matches!(k, EventKind::ShipLostWithBase { .. }));
        assert_eq!(
            sunk.kind,
            EventKind::ShipLostWithBase {
                ship: sid(1),
                base: bid(1),
            }
        );
        assert_eq!(sunk.causes, vec![Cause::Event(changed.id)]);
        assert_eq!(world.truth.ships[&sid(1)].location, ShipLocation::Sunk);
        // The group aboard her is lost with her, as with a ship that sinks.
        let group = one(&events, |k| matches!(k, EventKind::AirGroupLost { .. }));
        assert_eq!(
            group.kind,
            EventKind::AirGroupLost {
                air_group: gid(7),
                ship: sid(1),
                aircraft: 7,
            }
        );
        assert_eq!(group.causes, vec![Cause::Event(sunk.id)]);
        let aboard = &world.truth.air_groups[&gid(7)];
        assert_eq!(
            (aboard.ready, aboard.damaged, aboard.pilots),
            (0, 0, Some(0))
        );
        // The men aboard her are lost with her, and a unit with no man left
        // is destroyed.
        let men = one(&events, |k| matches!(k, EventKind::MenLostWithShip { .. }));
        assert_eq!(
            men.kind,
            EventKind::MenLostWithShip {
                unit: lid(2),
                ship: sid(1),
                men: 150,
            }
        );
        assert_eq!(men.causes, vec![Cause::Event(sunk.id)]);
        assert_eq!(world.truth.ships[&sid(1)].troops, None);
        assert!(events
            .iter()
            .any(|e| e.kind == EventKind::LandUnitDestroyed { unit: lid(2) }));
        // A force at sea in the hex or near it is not touched.
        assert!(matches!(
            world.truth.ships[&sid(2)].location,
            ShipLocation::TaskForce(_)
        ));
    }

    #[test]
    fn every_order_to_a_group_lost_with_its_base_is_refused() {
        let mut world = fallen();
        let requests: Vec<OrderRequest> = [
            AirGroupOrder::SetAirMission(AirMission::Search),
            AirGroupOrder::SetSearchArc {
                from_deg: 0,
                to_deg: 90,
                range_nm: n(100),
            },
            AirGroupOrder::SetPatrolShare { share: Fixed::ONE },
            AirGroupOrder::SetRestThreshold { fatigue: n(50) },
            AirGroupOrder::SetReach {
                reach: Reach::Extended,
            },
            AirGroupOrder::SetTargetPriority {
                kinds: vec![ShipKind::Carrier],
            },
            AirGroupOrder::Rebase { base: bid(2) },
        ]
        .into_iter()
        .map(|order| OrderRequest {
            kind: OrderKind::AirGroup {
                target: gid(1),
                order,
            },
            origin: Origin::PLAYER,
        })
        .collect();
        let count = requests.len();
        world.submit_orders(Side::Japanese, requests);
        world.resolve_day().expect("the day resolves");
        let events = all_events(&world);
        let refused: Vec<&RejectReason> = events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::OrderRejected { reason, .. } => Some(reason),
                _ => None,
            })
            .collect();
        assert_eq!(refused, vec![&RejectReason::AirGroupLostWithBase; count]);
        assert!(!events
            .iter()
            .any(|e| matches!(e.kind, EventKind::OrderAccepted { .. })));
        // The group is still lost and flies nothing.
        assert_eq!(
            world.truth.air_groups[&gid(1)].location,
            AirGroupLocation::Lost
        );
        sound(&world);
    }

    #[test]
    fn a_force_away_at_the_yard_from_the_port_returns_to_the_hex_and_finds_no_base() {
        let mut world = world();
        // Force 1 left from base 1 and is due back on day 2. Its ship is at
        // the yard.
        {
            let force = world.truth.task_forces.get_mut(&tid(1)).unwrap();
            force.ships.clear();
            force.from_hex = AT;
            force.yard_trip = Some(YardTrip {
                base: bid(1),
                return_day: 2,
                departure: EventId::new(0),
            });
            world.truth.ships.get_mut(&sid(2)).unwrap().location = ShipLocation::Yard(tid(1));
        }
        sound(&world);
        world.resolve_day().expect("day 1 resolves: the base falls");
        assert_eq!(world.truth.bases[&bid(1)].side, Side::Allied);
        // The force is away and has lost its home port, with an event.
        assert_eq!(world.truth.task_forces[&tid(1)].home_port, None);
        assert!(world.truth.task_forces[&tid(1)].yard_trip.is_some());
        let events = all_events(&world);
        let changed = one(&events, |k| matches!(k, EventKind::BaseChangedSide { .. }));
        let lost = one(&events, |k| matches!(k, EventKind::HomePortLost { .. }));
        assert_eq!(lost.causes, vec![Cause::Event(changed.id)]);
        sound(&world);
        world
            .resolve_day()
            .expect("day 2 resolves: the force returns");
        let events = all_events(&world);
        let back = one(&events, |k| {
            matches!(k, EventKind::ForceReturnedFromYard { .. })
        });
        assert_eq!(back.day, 2);
        assert!(matches!(&back.kind,
            EventKind::ForceReturnedFromYard { task_force, base, .. }
                if *task_force == tid(1) && *base == bid(1)));
        let force = &world.truth.task_forces[&tid(1)];
        assert_eq!(force.yard_trip, None);
        assert_eq!(force.position(), AT);
        assert_eq!(force.home_port, None);
        assert_eq!(
            world.truth.ships[&sid(2)].location,
            ShipLocation::TaskForce(tid(1))
        );
        // No Japanese base is in the hex: it finds none.
        assert!(!world
            .truth
            .bases
            .values()
            .any(|base| base.side == Side::Japanese && base.hex == AT));
        sound(&world);
    }

    #[test]
    fn a_replacement_stream_of_the_beaten_side_delivers_nothing_after_the_base_falls() {
        let mut world = world();
        world.truth.replacements.push(Replacement {
            side: Side::Japanese,
            base: bid(1),
            aircraft_type: AircraftTypeId::new(5),
            aircraft_per_day: 5,
            pilots_per_day: 3,
        });
        world.resolve_day().expect("day 1");
        // An Allied group of the same type, short of its strength, is now at
        // base 1, which is the Allied side's.
        world.truth.air_groups.insert(
            gid(8),
            air_group(
                8,
                Side::Allied,
                AirGroupLocation::Base(bid(1)),
                4,
                0,
                Some(1),
            ),
        );
        let before = world.truth.air_groups[&gid(8)].clone();
        world.resolve_day().expect("day 2");
        world.resolve_day().expect("day 3");
        let events = all_events(&world);
        assert!(!events
            .iter()
            .any(|e| matches!(e.kind, EventKind::ReplacementsReceived { .. })));
        assert_eq!(world.truth.air_groups[&gid(8)], before);
        // The lost group has nothing either.
        let lost = &world.truth.air_groups[&gid(1)];
        assert_eq!(
            (lost.location, lost.ready, lost.pilots),
            (AirGroupLocation::Lost, 0, Some(0))
        );
        sound(&world);
    }

    #[test]
    fn from_the_next_day_the_base_feeds_men_of_its_new_side() {
        let mut world = world();
        world.resolve_day().expect("day 1: the base falls");
        let events = all_events(&world);
        // Night of day 1: no base of the side yet; the 1,000 men are unfed
        // and lose 50.
        let night_1 = events
            .iter()
            .find_map(|e| match &e.kind {
                EventKind::MenFed {
                    side: Side::Allied,
                    base,
                    men,
                    fed,
                    unfed,
                    ..
                } if e.day == 1 => Some((*base, *men, *fed, *unfed)),
                _ => None,
            })
            .expect("the night of day 1");
        assert_eq!(night_1, (None, 1000, 0, 1000));
        assert_eq!(world.truth.land_units[&lid(1)].men_ashore, 950);
        assert_eq!(world.truth.land_units[&lid(1)].fed_share, Fixed::ZERO);
        world.resolve_day().expect("day 2");
        let events = all_events(&world);
        // Night of day 2: the 950 men are fed from base 1, which pays
        // 950 x 0.005 = 4.75 t of its 30.
        let fed = events
            .iter()
            .find_map(|e| match &e.kind {
                EventKind::MenFed {
                    side: Side::Allied,
                    base,
                    men,
                    fed,
                    unfed,
                    ..
                } if e.day == 2 => Some((*base, *men, *fed, *unfed)),
                _ => None,
            })
            .expect("the night of day 2");
        assert_eq!(fed, (Some(bid(1)), 950, 950, 0));
        assert_eq!(
            world.truth.bases[&bid(1)].supply_t,
            "25.25".parse().unwrap()
        );
        assert_eq!(world.truth.land_units[&lid(1)].fed_share, Fixed::ONE);
        // The Japanese unit never fed from it, and nothing of the Japanese
        // side is charged there.
        assert!(!events.iter().any(|e| matches!(&e.kind,
            EventKind::MenFed { side: Side::Japanese, hex, .. } if *hex == AT)));
        sound(&world);
    }

    // ---- The choices of the land units (decision 0100) ----

    use crate::belief::{GroundReport, Report};
    use crate::choices::{EndOfDayPhase, Moment};
    use crate::orders::Job;

    /// Decision 14 of 0090 by hand: only a unit on attack goes back, and
    /// only above the limit of 60 (not at it).
    #[test]
    fn a_unit_goes_back_to_defend_only_on_attack_and_above_the_limit() {
        assert!(goes_back_to_defend(Posture::Attack, n(61), n(60)));
        assert!(!goes_back_to_defend(Posture::Attack, n(60), n(60)));
        assert!(!goes_back_to_defend(Posture::Defend, n(100), n(60)));
    }

    /// Decision 3b of 0099 by hand: no men is no day; a march or a contested
    /// hex is the day; a working unit works whatever its activity; training
    /// is training; a ready unit rehearses only if rostered and with the
    /// force (`with_force` is not asked otherwise).
    #[test]
    fn a_units_day_is_the_first_of_no_men_march_fight_work_train_rehearse_ready() {
        let day = |ashore, aboard, working, activity, marched, contested, roster, with| {
            unit_day(
                ashore,
                aboard,
                working,
                activity,
                marched,
                contested,
                roster,
                |_| with,
            )
        };
        let ready = Activity::Ready;
        assert_eq!(
            day(0, 0, false, ready, true, true, None, true),
            UnitDay::NoMen
        );
        assert_eq!(
            day(10, 0, false, ready, true, true, None, true),
            UnitDay::Marched
        );
        assert_eq!(
            day(0, 10, false, ready, false, true, None, true),
            UnitDay::Contested
        );
        assert_eq!(
            day(10, 0, true, Activity::Train, false, false, None, true),
            UnitDay::Work(None)
        );
        assert_eq!(
            day(
                10,
                0,
                false,
                Activity::Work(Some(Job::Port)),
                false,
                false,
                None,
                true
            ),
            UnitDay::Work(Some(Job::Port))
        );
        assert_eq!(
            day(
                10,
                0,
                false,
                Activity::Train,
                false,
                false,
                Some(tid(1)),
                true
            ),
            UnitDay::Train
        );
        assert_eq!(
            day(10, 0, false, ready, false, false, Some(tid(1)), true),
            UnitDay::Rehearse(tid(1))
        );
        assert_eq!(
            day(10, 0, false, ready, false, false, Some(tid(1)), false),
            UnitDay::Ready
        );
        assert_eq!(
            day(10, 0, false, ready, false, false, None, true),
            UnitDay::Ready
        );
    }

    /// The test world with a tired Allied unit on attack (unit 1, fatigue
    /// 100) and a second Allied unit (3) ashore at the Allied base with its
    /// force, idle there, preparing a landing with it on the roster.
    fn ground_world() -> World {
        let mut world = world();
        world
            .truth
            .land_units
            .get_mut(&lid(1))
            .expect("unit 1")
            .fatigue = n(100);
        world.truth.land_units.insert(
            lid(3),
            LandUnit::new(
                lid(3),
                "Allied unit 3".to_string(),
                Side::Allied,
                500,
                Some(FAR),
            ),
        );
        let mut force = TaskForce::at(
            tid(3),
            "Force 3".to_string(),
            Side::Allied,
            FAR,
            Mission::Escort,
        );
        force.preparation = Some(crate::state::Preparation {
            hex: FAR,
            roster: [lid(3)].into_iter().collect(),
            loading: Fixed::ZERO,
            order: OrderId::new(9),
        });
        world.truth.task_forces.insert(force.id, force);
        sound(&world);
        world
    }

    /// Asked from the Allied view before the phase, every question of the
    /// view gives the answer the engine traced for the same unit: unit 1,
    /// on attack at fatigue 100, goes back to defend (the engine's fatigue
    /// is 95 after resting, still above 60); unit 3 rehearses with force 3;
    /// unit 2 (Japanese, in port) is not the side's. The questions the
    /// engine traced are answered by the view as the engine did.
    #[test]
    fn the_view_gives_the_answers_the_engine_traced_for_land_units() {
        let mut world = ground_world();
        let before = world.clone();
        world.trace_choices(true);
        world.pass_moment(Moment::EndOfDay {
            day: 1,
            phase: EndOfDayPhase::GroundCombat,
        });
        world.fight_ashore(1);
        let trace = world.take_choice_trace();
        let allied = before.side_view(Side::Allied);
        let mut back = 0;
        let mut rehearse = 0;
        for entry in &trace {
            // The one exception, an in-step change: the Japanese base falls
            // in this same step and its ship in port is sunk with the men
            // aboard, so unit 2 has no men by the time its day is chosen.
            if entry.side == Side::Japanese {
                continue;
            }
            let asked = before.side_view(entry.side).ask(&entry.question);
            assert_eq!(asked, Some(entry.answer.clone()), "{entry:?}");
            match &entry.answer {
                Answer::Ground(GroundAnswer::BackToDefend(true)) => back += 1,
                Answer::Ground(GroundAnswer::UnitDay(UnitDay::Rehearse(force))) => {
                    assert_eq!(*force, tid(3));
                    rehearse += 1;
                }
                _ => {}
            }
        }
        assert_eq!((back, rehearse), (1, 1), "{trace:?}");
        // The questions of the view, with the staff's default inputs, are
        // answered as the engine answered them here, where nothing marched
        // or fought.
        for question in allied.questions() {
            let Some(Answer::Ground(answer)) = allied.ask(&question) else {
                panic!("a question the view lists has an answer: {question:?}");
            };
            assert!(
                trace.iter().any(|entry| {
                    entry.answer == Answer::Ground(answer.clone())
                        && matches!(
                            (&entry.question, &question),
                            (
                                Question::Ground(GroundQuestion::BackToDefend { unit: a, .. }),
                                Question::Ground(GroundQuestion::BackToDefend { unit: b, .. }),
                            ) | (
                                Question::Ground(GroundQuestion::UnitDay { unit: a, .. }),
                                Question::Ground(GroundQuestion::UnitDay { unit: b, .. }),
                            ) if a == b
                        )
                }),
                "{question:?} -> {answer:?}"
            );
        }
    }

    /// A march that stops, or a fight, changes `marched` and `contested`
    /// inside the step: the question carries them, and the view answers with
    /// them as given.
    #[test]
    fn a_marched_or_contested_unit_has_no_day_of_its_own_whatever_its_activity() {
        let world = ground_world();
        let view = world.side_view(Side::Allied);
        for (marched, contested, want) in [
            (true, false, UnitDay::Marched),
            (false, true, UnitDay::Contested),
            (false, false, UnitDay::Rehearse(tid(3))),
        ] {
            let answer = view.ask_ground(&GroundQuestion::UnitDay {
                unit: lid(3),
                marched,
                contested,
                rostered_with: Some(tid(3)),
            });
            assert_eq!(answer, Some(GroundAnswer::UnitDay(want)));
        }
        // Not the side's unit, or not the side's force: no answer.
        assert_eq!(
            view.ask_ground(&GroundQuestion::UnitDay {
                unit: lid(2),
                marched: false,
                contested: false,
                rostered_with: None
            }),
            None
        );
        assert_eq!(
            view.ask_ground(&GroundQuestion::UnitDay {
                unit: lid(3),
                marched: false,
                contested: false,
                rostered_with: Some(tid(1))
            }),
            None
        );
    }

    /// The default `contested` is true only when the side's own fighting
    /// unit there is on attack and it holds an estimate of enemy men in the
    /// hex.
    #[test]
    fn the_default_contested_needs_an_own_attack_and_an_estimate_of_enemy_men() {
        let mut world = ground_world();
        let contested = |world: &World| -> bool {
            world
                .side_view(Side::Allied)
                .ground_questions()
                .into_iter()
                .find_map(|question| match question {
                    GroundQuestion::UnitDay {
                        unit, contested, ..
                    } if unit == lid(1) => Some(contested),
                    _ => None,
                })
                .expect("a question for unit 1")
        };
        assert!(!contested(&world));
        let figures = SearchFigures::from_search_toml();
        world
            .beliefs
            .get_mut(&Side::Allied)
            .expect("belief")
            .apply_report(
                &Report::Ground(GroundReport {
                    hex: AT,
                    men: 300,
                    day: 0,
                    received_event: EventId::new(1),
                }),
                &figures,
            );
        assert!(contested(&world));
        world
            .truth
            .land_units
            .get_mut(&lid(1))
            .expect("unit 1")
            .posture = Posture::Defend;
        assert!(!contested(&world));
    }

    /// Two worlds that differ only in what the Allied side cannot see (an
    /// enemy unit's men ashore in its own hex, its force, its base's
    /// stores) give the same questions and answers from the Allied view.
    #[test]
    fn the_enemys_men_ashore_change_no_answer_of_the_view_of_the_land_units() {
        let a = ground_world();
        let mut b = ground_world();
        let enemy = b.truth.land_units.get_mut(&lid(2)).expect("unit 2");
        enemy.men_ashore = 400;
        enemy.hex = Some(AT);
        enemy.posture = Posture::Attack;
        enemy.fatigue = n(70);
        b.truth.bases.get_mut(&bid(1)).expect("base 1").supply_t = n(1);
        b.truth
            .task_forces
            .get_mut(&tid(1))
            .expect("force 1")
            .from_hex = Hex::new(3, 1);
        let (va, vb) = (a.side_view(Side::Allied), b.side_view(Side::Allied));
        assert_eq!(va.questions(), vb.questions());
        for question in va.questions() {
            assert_eq!(va.ask(&question), vb.ask(&question), "{question:?}");
        }
    }
}
