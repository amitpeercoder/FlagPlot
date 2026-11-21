use std::collections::BTreeMap;

use crate::damage::roll_shots;
use crate::events::{Breakdown, Cause, EventKind, Factor};
use crate::fixed::Fixed;
use crate::ground::take_in_proportion;
use crate::hex::Hex;
use crate::ids::{EntityRef, EventId, LandUnitId};
use crate::orders::Posture;
use crate::refdata::GroundFigures;
use crate::rng::{Draw, Subsystem};
use crate::state::{Side, Skills};
use crate::world::World;

use super::{at, other, FightOutcome, HexUnits};

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

/// One side's loss in a fight: the side, each unit with its men at the start
/// and its part, the loss in the hex and the side's men there at the start.
type SideLoss = (Side, Vec<(LandUnitId, u32, u32)>, u32, u32);

/// Whether a side that attacked failed that day (decision 4 of 0105): its
/// weight of fire is below the other side's. A side that did not attack
/// never fails; if both attacked the weaker fails, and if they are equal
/// neither does.
pub(crate) fn attack_failed(attacked: bool, weight: Fixed, enemy_weight: Fixed) -> bool {
    attacked && weight < enemy_weight
}

/// A side's shots after the limit on engaged men (decision 1 of 0105): a
/// unit's `shots` times `engaged` over the side's `side_firing` men, where
/// `engaged` is the limit, `engagement_ratio` times the other side's fighting
/// men; unchanged where the side's firing men are not above the limit.
pub(crate) fn engaged_shots(shots: Fixed, side_firing: u32, engaged: Fixed) -> Fixed {
    let firing = Fixed::from_int(i64::from(side_firing));
    if firing > engaged {
        shots.mul_div(engaged, firing)
    } else {
        shots
    }
}

/// One unit's fire in a day's fight, worked out before any die is rolled
/// (decision 4 of 0105): what it fires with, its shots before and after the
/// limit on engaged men, and its chance.
#[derive(Clone, Copy)]
struct PlannedFire {
    unit: LandUnitId,
    men_ashore: u32,
    fed_share: Fixed,
    fatigue_factor: Fixed,
    firing: u32,
    shots_before_limit: Fixed,
    shots: Fixed,
    weight_factor: Fixed,
    skills: Skills,
    experience: u8,
    experience_factor: Fixed,
    side_chance: Fixed,
    holding_factor: Fixed,
    defences: Fixed,
    defences_factor: Fixed,
    chance: Fixed,
    posture: Posture,
}

impl World {
    /// The day's fight in a contested hex (decisions 4 to 10 of 0090, 1 and
    /// 4 of 0105, and correction 1 of 0109).
    pub(super) fn fight(
        &mut self,
        day: u32,
        hex: Hex,
        units: &HexUnits,
        attacks: [bool; 2],
        figures: &GroundFigures,
    ) -> FightOutcome {
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

        // Each unit's fire on the strengths at the start (decisions 6, 7, 9),
        // worked out for both sides before any die is rolled: its firing men,
        // its shots and its chance; then each side's shots limited to the men
        // who can come to grips (decision 1 of 0105), and each side's weight
        // of fire (decision 4 of 0105).
        // The units in the fight (correction 1 of decision 0109): where the
        // other side is not on attack in the hex, a unit falling back is out
        // of it both ways. It does not fire, it is not counted for the
        // limit, and it takes no share of the losses.
        let engaged_units = [
            self.in_fight(&units.units[0], attacks[1]),
            self.in_fight(&units.units[1], attacks[0]),
        ];
        // Each unit in the fight took part in it, attacking or defending
        // (decision 0197, 15).
        self.note_fought(day, &engaged_units[0]);
        self.note_fought(day, &engaged_units[1]);
        let mut planned: [Vec<PlannedFire>; 2] = [Vec::new(), Vec::new()];
        let mut fed_today: [Option<EventId>; 2] = [None, None];
        for side in Side::ALL {
            let fed = self.events.latest_where(|kind| {
                matches!(kind, EventKind::MenFed { side: owner, hex: at_hex, .. }
                    if *owner == side && *at_hex == hex)
            });
            fed_today[at(side)] = fed.filter(|event| {
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
            for unit_id in engaged_units[at(side)].clone() {
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
                planned[at(side)].push(PlannedFire {
                    unit: unit_id,
                    men_ashore,
                    fed_share,
                    fatigue_factor,
                    firing,
                    shots_before_limit: shots,
                    shots,
                    weight_factor,
                    skills,
                    experience,
                    experience_factor,
                    side_chance,
                    holding_factor,
                    defences,
                    defences_factor,
                    chance,
                    posture,
                });
            }
        }
        // The limit (decision 1 of 0105): a side's firing men count up to
        // `engagement_ratio` times the other side's fighting men in the fight
        // at its start; above it each unit's shots are cut
        // in the same proportion.
        let fighting = [
            self.fighting_men_ashore(&engaged_units[0]),
            self.fighting_men_ashore(&engaged_units[1]),
        ];
        let mut limits: [Option<(u32, Fixed)>; 2] = [None, None];
        let mut weights = [Fixed::ZERO; 2];
        for side in Side::ALL {
            let side_firing: u32 = planned[at(side)].iter().map(|fire| fire.firing).sum();
            let engaged = figures.engagement_ratio * i64::from(fighting[at(other(side))]);
            if Fixed::from_int(i64::from(side_firing)) > engaged {
                limits[at(side)] = Some((side_firing, engaged));
            }
            for fire in &mut planned[at(side)] {
                fire.shots = engaged_shots(fire.shots_before_limit, side_firing, engaged);
                weights[at(side)] += fire.shots * fire.chance;
            }
        }

        // Each unit fires: the dice and the record, in side and unit order.
        let mut hits = [0u32; 2];
        let mut fire_events: [Vec<EventId>; 2] = [Vec::new(), Vec::new()];
        let mut fired: BTreeMap<LandUnitId, EventId> = BTreeMap::new();
        let mut fired_any: BTreeMap<LandUnitId, EventId> = BTreeMap::new();
        for side in Side::ALL {
            let limit = limits[at(side)];
            for fire in planned[at(side)].clone() {
                let PlannedFire {
                    unit: unit_id,
                    men_ashore,
                    fed_share,
                    fatigue_factor,
                    firing,
                    shots_before_limit,
                    shots,
                    weight_factor,
                    skills,
                    experience,
                    experience_factor,
                    side_chance,
                    holding_factor,
                    defences,
                    defences_factor,
                    chance,
                    posture,
                } = fire;
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
                    // The limit on engaged men only where it cuts, so a
                    // breakdown with the limit idle is as it was (decision 1
                    // of 0105).
                    if let Some((side_firing, engaged)) = limit {
                        factors.push(Factor {
                            name: "shots_before_limit",
                            value: shots_before_limit,
                        });
                        factors.push(Factor {
                            name: "side_firing_men",
                            value: Fixed::from_int(i64::from(side_firing)),
                        });
                        factors.push(Factor {
                            name: "engaged_men",
                            value: engaged,
                        });
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
                causes.extend(fed_today[at(side)].map(Cause::Event));
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

        // An attack that failed (decision 4 of 0105): a side that attacked
        // and whose weight of fire is below the other side's. It cites the
        // fight and both sides' fire, whose weights it compares.
        let mut failed: [Option<EventId>; 2] = [None, None];
        for side in Side::ALL {
            let (weight, enemy_weight) = (weights[at(side)], weights[at(other(side))]);
            if !attack_failed(attacks[at(side)], weight, enemy_weight) {
                continue;
            }
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "weight",
                        value: weight,
                    },
                    Factor {
                        name: "enemy_weight",
                        value: enemy_weight,
                    },
                ],
                result: weight,
            });
            let mut causes = vec![Cause::Event(fight)];
            causes.extend(
                fire_events[at(side)]
                    .iter()
                    .chain(&fire_events[at(other(side))])
                    .map(|event| Cause::Event(*event)),
            );
            failed[at(side)] = Some(self.events.record(
                day,
                None,
                EventKind::GroundAttackFailed {
                    side,
                    hex,
                    weight,
                    enemy_weight,
                },
                causes,
                breakdown,
            ));
        }

        // The losses, worked for both sides from the strengths at the start
        // and then taken (decision 9), shared among the units in the fight.
        let men_per_hit = u32::try_from(figures.men_per_hit.trunc()).unwrap_or(u32::MAX);
        let mut losses: Vec<SideLoss> = Vec::new();
        for side in Side::ALL {
            let list = &engaged_units[at(side)];
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
                // A defending unit's cover saves a share of its part
                // (decision 3 of 0106); a unit on attack has none.
                let cover = crate::cover::unit_cover(&self.truth, &self.truth.land_units[&unit_id]);
                let lost = crate::cover::after_cover(part, cover, figures.cover_in_fight);
                let covered = figures.cover_in_fight > Fixed::ZERO && cover > Fixed::ZERO;
                if lost == 0 {
                    continue;
                }
                let unit = self
                    .truth
                    .land_units
                    .get_mut(&unit_id)
                    .expect("the land unit exists");
                unit.men_ashore -= lost;
                if unit.men_ashore == 0 {
                    unit.hex = None;
                }
                let left = unit.men_ashore;
                let breakdown = self.config.record_breakdowns.then(|| {
                    let mut factors = vec![
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
                    ];
                    // Only where cover saves men (decision 3 of 0106).
                    if covered {
                        factors.extend([
                            Factor {
                                name: "men_before_cover",
                                value: Fixed::from_int(i64::from(part)),
                            },
                            Factor {
                                name: "cover",
                                value: cover,
                            },
                            Factor {
                                name: "cover_in_fight",
                                value: figures.cover_in_fight,
                            },
                        ]);
                    }
                    Breakdown {
                        factors,
                        result: Fixed::from_int(i64::from(lost)),
                    }
                });
                let lost_event = self.events.record(
                    day,
                    None,
                    EventKind::MenLostInFight {
                        unit: unit_id,
                        hex,
                        lost,
                        ashore: left,
                    },
                    causes.clone(),
                    breakdown,
                );
                self.destroy_if_no_men(day, None, unit_id, lost_event);
            }
        }
        FightOutcome {
            fight,
            fired,
            fired_any,
            failed,
        }
    }
}
