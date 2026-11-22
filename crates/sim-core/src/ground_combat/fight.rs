use std::collections::BTreeMap;

use crate::condition::{unit_fighting_share, FightingShare};
use crate::damage::roll_shots;
use crate::events::{Breakdown, Cause, EventKind, Factor, SupplyUse};
use crate::fixed::Fixed;
use crate::ground::take_in_proportion;
use crate::hex::Hex;
use crate::ids::{BaseId, EntityRef, EventId, LandUnitId};
use crate::orders::Posture;
use crate::refdata::{FightSupplyFigures, GroundFigures};
use crate::rng::{Draw, Subsystem};
use crate::state::{Side, Skills};
use crate::world::World;

use super::{at, other, FightOutcome, HexUnits};

/// A unit's firing men (decision 6, and decision 0208, 6): its men ashore,
/// times its fighting share (its fed share where the figures of 0208 are not
/// given), times 100 less its fatigue over 100, cut once to a whole man.
/// Returns the firing men and the fatigue factor.
pub(crate) fn firing_men(men_ashore: u32, fighting_share: Fixed, fatigue: Fixed) -> (u32, Fixed) {
    let hundred = Fixed::from_int(100);
    let fatigue_factor = (hundred - fatigue) / hundred;
    let firing = (Fixed::from_int(i64::from(men_ashore)) * fighting_share)
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

/// A side's share of what its fight wanted that was charged (decision
/// 0209, 6): `charged` over `wanted`, a division cut at the sixth place, and
/// one where nothing was wanted.
pub(crate) fn supplied_share(charged: Fixed, wanted: Fixed) -> Fixed {
    if wanted > Fixed::ZERO {
        (charged / wanted).min(Fixed::ONE)
    } else {
        Fixed::ONE
    }
}

/// A side's fire share (decision 0209, 7): `unsupplied` plus one less it
/// times the supplied share.
pub(crate) fn fire_share(unsupplied: Fixed, supplied: Fixed) -> Fixed {
    unsupplied + (Fixed::ONE - unsupplied) * supplied
}

/// What a side's fight was supplied with (decision 0209): its supplied
/// share, its fire share, and the event of its charge where one was
/// recorded.
#[derive(Clone, Copy)]
struct SideSupplied {
    supplied_share: Fixed,
    fire_share: Fixed,
    charge: Option<EventId>,
}

/// What a base gave a fight's charge (decision 0209, 5): the base, what was
/// charged before it, what was asked of it, what it paid and what it has
/// left.
type StockTaken = (BaseId, Fixed, Fixed, Fixed, Fixed);

/// One unit's fire in a day's fight, worked out before any die is rolled
/// (decision 4 of 0105): what it fires with, its shots before and after the
/// limit on engaged men, and its chance.
#[derive(Clone, Copy)]
struct PlannedFire {
    unit: LandUnitId,
    men_ashore: u32,
    fighting: FightingShare,
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
    /// Charges a side's men at grips in the day's fight at `hex` (decision
    /// 0209, 4 to 7 and 14): its want, the men at grips times
    /// `fight_supply_t_per_man`, taken from the ammunition of each of its
    /// bases in the hex in base order, then from the supply of each, as far
    /// as they go. A side the world takes as supplied in full (rule 12) is
    /// charged nothing and records nothing, with a supplied share of one.
    /// Otherwise, where anything was wanted, the charge is recorded citing
    /// the fight, and each base charged records its stock spent citing the
    /// charge: its ammunition first, then its supply. Draws nothing.
    fn charge_fight(
        &mut self,
        day: u32,
        hex: Hex,
        side: Side,
        at_grips: Fixed,
        figures: &FightSupplyFigures,
        fight: EventId,
    ) -> SideSupplied {
        let unsupplied = figures.unsupplied_fire_share;
        let wanted = at_grips * figures.fight_supply_t_per_man;
        if self.supplied_in_full == Some(side) || wanted <= Fixed::ZERO {
            return SideSupplied {
                supplied_share: Fixed::ONE,
                fire_share: fire_share(unsupplied, Fixed::ONE),
                charge: None,
            };
        }
        // The side's bases in the hex, in the order the night's charge takes
        // them (decision 0066, 16).
        let bases: Vec<BaseId> = self
            .truth
            .bases
            .values()
            .filter(|base| base.side == side && base.hex == hex)
            .map(|base| base.id)
            .collect();
        // What each base gives, ammunition first then supply: (base, charged
        // before it, asked, paid, left).
        let mut from_ammunition: Vec<StockTaken> = Vec::new();
        let mut from_supply: Vec<StockTaken> = Vec::new();
        let mut charged = Fixed::ZERO;
        for base_id in &bases {
            let base = self.truth.bases.get_mut(base_id).expect("the base exists");
            if base.ammunition_t <= Fixed::ZERO || charged >= wanted {
                continue;
            }
            let asked = wanted - charged;
            let paid = asked.min(base.ammunition_t);
            base.ammunition_t -= paid;
            from_ammunition.push((*base_id, charged, asked, paid, base.ammunition_t));
            charged += paid;
        }
        let ammunition_t = charged;
        for base_id in &bases {
            if self.truth.bases[base_id].supply_t <= Fixed::ZERO || charged >= wanted {
                continue;
            }
            let asked = wanted - charged;
            let (paid, left) = self.take_supply(*base_id, asked);
            from_supply.push((*base_id, charged, asked, paid, left));
            charged += paid;
        }
        let supply_t = charged - ammunition_t;
        let supplied = supplied_share(charged, wanted);
        let fire = fire_share(unsupplied, supplied);
        let breakdown = self.config.record_breakdowns.then(|| Breakdown {
            factors: vec![
                Factor {
                    name: "men_at_grips",
                    value: at_grips,
                },
                Factor {
                    name: "fight_supply_t_per_man",
                    value: figures.fight_supply_t_per_man,
                },
                Factor {
                    name: "wanted_t",
                    value: wanted,
                },
                Factor {
                    name: "from_ammunition_t",
                    value: ammunition_t,
                },
                Factor {
                    name: "from_supply_t",
                    value: supply_t,
                },
            ],
            result: supplied,
        });
        let charge = self.events.record(
            day,
            None,
            EventKind::FightSupplyCharged {
                side,
                hex,
                men_at_grips: at_grips,
                wanted_t: wanted,
                from_ammunition_t: ammunition_t,
                from_supply_t: supply_t,
                supplied_share: supplied,
            },
            vec![Cause::Event(fight)],
            breakdown,
        );
        // What was asked of a base: the want less what was charged before it.
        let asked_breakdown = |record: bool, before: Fixed, asked: Fixed| {
            record.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "wanted_t",
                        value: wanted,
                    },
                    Factor {
                        name: "charged_before_t",
                        value: before,
                    },
                ],
                result: asked,
            })
        };
        let record = self.config.record_breakdowns;
        for (base, before, asked_t, paid_t, left_t) in from_ammunition {
            self.events.record(
                day,
                None,
                EventKind::BaseAmmunitionSpent {
                    base,
                    asked_t,
                    paid_t,
                    left_t,
                },
                vec![Cause::Event(charge)],
                asked_breakdown(record, before, asked_t),
            );
        }
        for (base, before, asked_t, paid_t, left_t) in from_supply {
            self.events.record(
                day,
                None,
                EventKind::BaseSupplySpent {
                    base,
                    reason: SupplyUse::Fight,
                    aircraft: 0,
                    men: 0,
                    asked_t,
                    paid_t,
                    left_t,
                },
                vec![Cause::Event(charge)],
                asked_breakdown(record, before, asked_t),
            );
        }
        SideSupplied {
            supplied_share: supplied,
            fire_share: fire,
            charge: Some(charge),
        }
    }

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
                let (men_ashore, fighting, fatigue, posture, experience, skills) = (
                    unit.men_ashore,
                    unit_fighting_share(unit, figures),
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
                let (firing, fatigue_factor) = firing_men(men_ashore, fighting.share, fatigue);
                // Its shots, times its weight factor (decision 2 of 0098).
                let weight_factor = Skills::factor(skills.weight, figures.weight_strength);
                let shots =
                    Fixed::from_int(i64::from(firing)) / figures.men_per_shot * weight_factor;
                planned[at(side)].push(PlannedFire {
                    unit: unit_id,
                    men_ashore,
                    fighting,
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
        let mut at_grips = [Fixed::ZERO; 2];
        for side in Side::ALL {
            let side_firing: u32 = planned[at(side)].iter().map(|fire| fire.firing).sum();
            let engaged = figures.engagement_ratio * i64::from(fighting[at(other(side))]);
            let firing = Fixed::from_int(i64::from(side_firing));
            if firing > engaged {
                limits[at(side)] = Some((side_firing, engaged));
            }
            // The side's men at grips (decision 0209, 3): its firing men, no
            // more than the limit, a quantity not cut to a whole man.
            at_grips[at(side)] = firing.min(engaged);
            for fire in &mut planned[at(side)] {
                fire.shots = engaged_shots(fire.shots_before_limit, side_firing, engaged);
            }
        }

        // What the fight costs (decision 0209, 3 to 8): each side's men at
        // grips charged to its bases in the hex, Allied first, on the stocks
        // as they stand before any die is rolled; then each unit's shots,
        // after the limit, times its side's fire share.
        let mut supplied: [Option<SideSupplied>; 2] = [None, None];
        if let Some(fight_figures) = figures.hunger.and_then(|hunger| hunger.fight_supply) {
            for side in Side::ALL {
                let side_supplied =
                    self.charge_fight(day, hex, side, at_grips[at(side)], &fight_figures, fight);
                for fire in &mut planned[at(side)] {
                    fire.shots = fire.shots * side_supplied.fire_share;
                }
                supplied[at(side)] = Some(side_supplied);
            }
        }
        let mut weights = [Fixed::ZERO; 2];
        for side in Side::ALL {
            for fire in &planned[at(side)] {
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
            let side_supplied = supplied[at(side)];
            for fire in planned[at(side)].clone() {
                let PlannedFire {
                    unit: unit_id,
                    men_ashore,
                    fighting,
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
                    // The fed share, and where the figures of 0208 are
                    // given the nights of hunger, the condition and the
                    // fighting share (decision 0208, 15).
                    let mut factors = vec![Factor {
                        name: "men_ashore",
                        value: Fixed::from_int(i64::from(men_ashore)),
                    }];
                    factors.extend(fighting.factors());
                    factors.extend([
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
                    ]);
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
                    // The side's supplied share and fire share, only where
                    // the figures of 0209 are given (decision 0209, 15).
                    if let Some(side_supplied) = side_supplied {
                        factors.push(Factor {
                            name: "supplied_share",
                            value: side_supplied.supplied_share,
                        });
                        factors.push(Factor {
                            name: "fire_share",
                            value: side_supplied.fire_share,
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
                // Its side's charge for the fight (decision 0209, 14).
                causes.extend(
                    side_supplied
                        .and_then(|side_supplied| side_supplied.charge)
                        .map(Cause::Event),
                );
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
