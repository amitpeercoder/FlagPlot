//! Men who replace the lost (decision 0197): a base's stream of
//! replacement men, given each night to the land units of its side ashore
//! in its hex that are below their establishment and not in a fight.
//!
//! [`World::receive_replacement_men`] runs in the night pulse, a step of
//! its own after the supply lines and before the night's aircraft and men
//! are paid for, so that the men who join are fed that night (rule 4), from
//! day 2 on (rule 3). For each stream, in the order the state keeps them,
//! whose days hold the day and whose base is held by the stream's side: the
//! side's land units with men ashore in the base's hex that are below their
//! establishment (men ashore and aboard together, rule 17), not on attack,
//! not on a march or falling back, and that did not take part in a fight
//! ashore the day before (rule 15), are served in unit order, each up to
//! its establishment, until the night's men are gone. Men not given are
//! lost. Each unit that takes men records `MenJoined`, with no cause.
//!
//! Experience (rules 12 to 14): a unit counts the men who have joined it
//! since its level last changed. A unit above the replacements' level (the
//! ground figure `replacement_experience`, 2) loses one level on the night
//! that count reaches its share (`replacement_share`, a half) of its
//! establishment, and the count begins again from nothing. A unit at or
//! below that level is not changed. The count also begins again when the
//! unit gains a level by fighting or training.
//!
//! Reads: the streams; the day; the holder and hex of a base; each land
//! unit's side, men ashore and aboard, hex, posture, march and experience;
//! each unit's strength; the ground figures. Writes: a unit's men ashore,
//! experience and count of men joined. No random number.
//!
//! The rule keeps, besides, each unit's last day fought (written by the
//! fight ashore, [`World::note_fought`]) and lowers a unit's establishment
//! when it is divided ([`World::note_division`]). All of it is kept only
//! where the scenario has a stream or an establishment: with neither,
//! `TruthState::men_replacement` is empty and nothing here does anything.

use crate::events::{Breakdown, EventKind, Factor};
use crate::fixed::Fixed;
use crate::ids::LandUnitId;
use crate::orders::Posture;
use crate::state::Strength;
use crate::time::Pulse;
use crate::world::World;

impl World {
    /// The night's step of replacement men (decision 0197, rules 3 to 8 and
    /// 12). See the module comment.
    pub(crate) fn receive_replacement_men(&mut self, day: u32, pulse: Pulse) {
        // Day 1 has nothing to replace (rule 3).
        if day < 2 {
            return;
        }
        let Some(figures) = self.ref_data.ground else {
            return;
        };
        let streams = self.truth.men_replacement.streams.clone();
        for stream in streams {
            if !stream.gives_on(day) {
                continue;
            }
            let Some(base) = self.truth.bases.get(&stream.base) else {
                continue;
            };
            if base.side != stream.side {
                continue;
            }
            let hex = base.hex;
            let candidates: Vec<LandUnitId> = self
                .truth
                .land_units
                .values()
                .filter(|unit| {
                    unit.side == stream.side
                        && unit.men_ashore > 0
                        && unit.hex == Some(hex)
                        && unit.posture != Posture::Attack
                        && unit.march.is_none()
                })
                .map(|unit| unit.id)
                .collect();
            let mut left = stream.men_per_day;
            for id in candidates {
                if left == 0 {
                    break;
                }
                let Some(strength) = self.truth.men_replacement.units.get(&id).copied() else {
                    continue;
                };
                // In a fight: on attack (above), or fought the day before
                // (rule 15).
                if strength.last_fought == Some(day - 1) {
                    continue;
                }
                let aboard = self.truth.men_aboard(id);
                let unit = &self.truth.land_units[&id];
                let men_before = unit.men_ashore + aboard;
                let short = strength.establishment.saturating_sub(men_before);
                if short == 0 {
                    continue;
                }
                let men = left.min(short);
                left -= men;
                let experience_before = unit.experience;
                let counted = strength.joined + men;
                // The count reaches the share of the establishment: compared
                // exactly, as men joined against establishment times share.
                let threshold =
                    Fixed::from_int(i64::from(strength.establishment)) * figures.replacement_share;
                let falls = experience_before > figures.replacement_experience
                    && Fixed::from_int(i64::from(counted)) >= threshold;
                let (experience_after, joined) = if falls {
                    (experience_before - 1, 0)
                } else {
                    (experience_before, counted)
                };
                let unit = self
                    .truth
                    .land_units
                    .get_mut(&id)
                    .expect("the land unit exists");
                unit.men_ashore += men;
                unit.experience = experience_after;
                let men_after = unit.men_ashore + aboard;
                let side = unit.side;
                self.truth
                    .men_replacement
                    .units
                    .insert(id, Strength { joined, ..strength });
                let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                    factors: vec![
                        Factor {
                            name: "men_left_in_stream",
                            value: Fixed::from_int(i64::from(left + men)),
                        },
                        Factor {
                            name: "establishment",
                            value: Fixed::from_int(i64::from(strength.establishment)),
                        },
                        Factor {
                            name: "men_before",
                            value: Fixed::from_int(i64::from(men_before)),
                        },
                        Factor {
                            name: "joined_before",
                            value: Fixed::from_int(i64::from(strength.joined)),
                        },
                        Factor {
                            name: "replacement_share",
                            value: figures.replacement_share,
                        },
                        Factor {
                            name: "replacement_experience",
                            value: Fixed::from_int(i64::from(figures.replacement_experience)),
                        },
                    ],
                    result: Fixed::from_int(i64::from(men)),
                });
                self.events.record(
                    day,
                    Some(pulse),
                    EventKind::MenJoined {
                        unit: id,
                        side,
                        base: stream.base,
                        men,
                        men_after,
                        experience_before,
                        experience_after,
                        joined,
                    },
                    Vec::new(),
                    breakdown,
                );
            }
        }
    }

    /// Notes that the units took part in a fight ashore on `day`, attacking
    /// or defending (decision 0197, 15). Does nothing to a unit with no
    /// strength kept, so nothing in a scenario with no stream.
    pub(crate) fn note_fought(&mut self, day: u32, units: &[LandUnitId]) {
        for unit in units {
            if let Some(strength) = self.truth.men_replacement.units.get_mut(unit) {
                strength.last_fought = Some(day);
            }
        }
    }

    /// A unit made by division (decision 0197, 18): the new unit's
    /// establishment is the men it is made with and its count begins at
    /// nothing; the unit it left has its establishment lowered by as many
    /// and keeps its count and its last day fought. Does nothing where the
    /// parent has no strength kept.
    pub(crate) fn note_division(&mut self, from: LandUnitId, unit: LandUnitId, men: u32) {
        let units = &mut self.truth.men_replacement.units;
        let Some(parent) = units.get_mut(&from) else {
            return;
        };
        parent.establishment = parent.establishment.saturating_sub(men);
        units.insert(
            unit,
            Strength {
                establishment: men,
                joined: 0,
                last_fought: None,
            },
        );
    }

    /// The count of men joined begins again when the unit gains a level by
    /// fighting or training (decision 0197, 12).
    pub(crate) fn note_experience_raised(&mut self, unit: LandUnitId) {
        if let Some(strength) = self.truth.men_replacement.units.get_mut(&unit) {
            strength.joined = 0;
        }
    }
}
