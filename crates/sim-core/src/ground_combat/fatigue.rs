use crate::choices::ground::{GroundAnswer, GroundQuestion};
use crate::choices::{Answer, Question};
use crate::events::{Breakdown, Cause, EventKind, Factor, LandFatigueChange};
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{EventId, LandUnitId};
use crate::orders::{Posture, WhenSpent};
use crate::refdata::GroundFigures;
use crate::state::{March, Side, TruthState};
use crate::world::World;

use super::{at, other, HexUnits};

/// Whether a land unit goes back to defend (decision 14 of 0090): it is on
/// attack and its fatigue is above the limit. Decision 1 of 0100: the
/// engine and a side's view both ask it.
pub(crate) fn goes_back_to_defend(posture: Posture, fatigue: Fixed, limit: Fixed) -> bool {
    posture == Posture::Attack && fatigue > limit
}

/// Whether a side can feed a unit where it stands (decision 7 of 0105): it
/// has a base in `hex` with supply above 0, a dump its column put down
/// included. Reads only the side's own bases.
pub(crate) fn fed_where_it_stands(truth: &TruthState, side: Side, hex: Hex) -> bool {
    truth
        .bases
        .values()
        .any(|base| base.side == side && base.hex == hex && base.supply_t > Fixed::ZERO)
}

/// What a land unit whose attack is spent does (decisions 6 and 7 of 0105):
/// what its order said, and with no instruction, hold if it can be fed where
/// it stands and else fall back. Decision 1 of 0100: the engine and a side's
/// view both ask it.
pub(crate) fn when_attack_spent(instruction: Option<WhenSpent>, fed_here: bool) -> WhenSpent {
    instruction.unwrap_or(if fed_here {
        WhenSpent::Hold
    } else {
        WhenSpent::FallBack
    })
}

impl World {
    /// A unit's fatigue for the day (decision 12), then back to defend if it
    /// is on attack and too tired (decision 14). A unit that marched gains
    /// the march's fatigue in place of resting, and one that marched and
    /// fired on attack gains both (decision 8 of 0094). A unit that fired on
    /// attack for a side whose attack failed (`failed_event`) gains
    /// `failed_attack_fatigue` beside the day's attack (decision 5 of 0105).
    /// A unit with no man ashore keeps its fatigue. Returns, for a unit that
    /// went back to defend, that event and the instruction its attack order
    /// carried, now spent.
    pub(super) fn tire_or_rest(
        &mut self,
        day: u32,
        unit_id: LandUnitId,
        fire_event: Option<EventId>,
        march_event: Option<EventId>,
        failed_event: Option<EventId>,
        figures: &GroundFigures,
    ) -> Option<(EventId, Option<WhenSpent>)> {
        // Only a unit that fired on attack pays for its side's failure.
        let failed_event = failed_event.filter(|_| fire_event.is_some());
        // The day's hits on men of its side in its hex, and its cover now,
        // before the day's digging (decision 10 of 0106).
        let (under_fire, cover) = {
            let unit = &self.truth.land_units[&unit_id];
            let under_fire = unit
                .hex
                .and_then(|hex| self.truth.hits_on_men.get(&(hex, unit.side)))
                // With `strain_per_hit` at 0 the rule is off and a unit under
                // fire rests as it always did (the architect's ruling on the
                // frozen figures, Nov 14, 2026).
                .filter(|hits| hits.hits > 0 && figures.strain_per_hit > Fixed::ZERO)
                .cloned();
            (under_fire, crate::cover::unit_cover(&self.truth, unit))
        };
        let unit = self
            .truth
            .land_units
            .get_mut(&unit_id)
            .expect("the land unit exists");
        if unit.men_ashore == 0 {
            return None;
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
            // Men under fire do not rest (decision 10 of 0106).
            (false, false) if under_fire.is_some() => LandFatigueChange::Strained,
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
            // A failed day wears the attackers out (decision 5 of 0105).
            if failed_event.is_some() {
                factors.push(Factor {
                    name: "failed_attack_fatigue",
                    value: figures.failed_attack_fatigue,
                });
                gained += figures.failed_attack_fatigue;
            }
            // The strain of a day under fire (decision 10 of 0106), counted
            // on the fatigue before the day; named only where it adds.
            if let Some(hits) = &under_fire {
                let (strain, before_limit) =
                    crate::cover::strain(hits.hits, cover, before, figures);
                if before_limit > Fixed::ZERO {
                    factors.extend([
                        Factor {
                            name: "hits_on_men",
                            value: Fixed::from_int(i64::from(hits.hits)),
                        },
                        Factor {
                            name: "strain_per_hit",
                            value: figures.strain_per_hit,
                        },
                        Factor {
                            name: "strain_per_day",
                            value: figures.strain_per_day,
                        },
                        Factor {
                            name: "cover",
                            value: cover,
                        },
                        Factor {
                            name: "cover_from_strain",
                            value: figures.cover_from_strain,
                        },
                        Factor {
                            name: "strain_limit",
                            value: figures.strain_limit,
                        },
                        Factor {
                            name: "strain",
                            value: strain,
                        },
                    ]);
                }
                gained += strain;
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
                .chain(failed_event)
                .chain(
                    under_fire
                        .iter()
                        .flat_map(|hits| hits.causes.iter().copied()),
                )
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
        if !back {
            return None;
        }
        {
            let unit = self
                .truth
                .land_units
                .get_mut(&unit_id)
                .expect("the land unit exists");
            let instruction = unit.when_spent.take();
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
            let back_event = self.events.record(
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
            Some((back_event, instruction))
        }
    }

    /// What a unit whose attack is spent does (decisions 6 and 7 of 0105):
    /// read when it has gone back to defend because it is tired, in a hex
    /// where the enemy has fighting men ashore after the day's fight. It
    /// holds, staying on the defence, or falls back by the rule of decision
    /// 0104 to the first linked hex with a base of its side, else the first
    /// with no enemy fighting men, tired by `beaten_fatigue` and carrying
    /// nothing; with no such hex it stays, and does not surrender. A unit
    /// already marching is left as it is, as a beaten side's is ("As
    /// built").
    #[allow(clippy::too_many_arguments)]
    pub(super) fn attack_spent(
        &mut self,
        day: u32,
        hex: Hex,
        units: &HexUnits,
        unit_id: LandUnitId,
        back_event: EventId,
        instruction: Option<WhenSpent>,
        figures: &GroundFigures,
    ) {
        let unit = &self.truth.land_units[&unit_id];
        let side = unit.side;
        if unit.hex != Some(hex) || unit.men_ashore == 0 || unit.march.is_some() {
            return;
        }
        if self.fighting_men_ashore(&units.units[at(other(side))]) == 0 {
            return;
        }
        let choice = when_attack_spent(instruction, fed_where_it_stands(&self.truth, side, hex));
        self.trace_choice(side, || {
            (
                Question::Ground(GroundQuestion::AttackSpent { unit: unit_id, hex }),
                Answer::Ground(GroundAnswer::AttackSpent(choice)),
            )
        });
        if choice == WhenSpent::Hold {
            return;
        }
        let Some(to) = self.fall_back_hex(hex, side) else {
            return;
        };
        let unit = self
            .truth
            .land_units
            .get_mut(&unit_id)
            .expect("the land unit exists");
        let men = unit.men_ashore;
        let before = unit.fatigue;
        let after = (before + figures.beaten_fatigue).min(Fixed::from_int(100));
        unit.fatigue = after;
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
            EventKind::SpentUnitFellBack {
                unit: unit_id,
                side,
                from: hex,
                to,
                men,
                fatigue: after,
            },
            vec![Cause::Event(back_event)],
            breakdown,
        );
        self.lose_fieldworks(day, None, unit_id, Cause::Event(fell_back));
        self.truth
            .land_units
            .get_mut(&unit_id)
            .expect("the land unit exists")
            .march = Some(March {
            to,
            days: 0,
            counted: Fixed::ZERO,
            cause: Cause::Event(fell_back),
        });
    }
}
