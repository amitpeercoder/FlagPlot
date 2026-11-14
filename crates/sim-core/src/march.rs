//! Marching overland (`docs/decisions/0094-marching-overland.md`).
//!
//! The marches are the first step of the ground combat phase, before the
//! day's fights. Units with a march order are taken in unit order. Each
//! marches one more day, which counts as its movement factor for the hexes
//! it leaves and enters (decision 4 of 0098); when the days counted reach
//! `march_days_per_hex` its men ashore are in the new hex, with the supply
//! it carried from its side's bases in the hex it left, and its order is
//! spent. The fatigue of a day's march is gained in the phase's fatigue
//! step, after the fights (`ground_combat.rs`). There is no randomness.
//!
//! Reads: a land unit's men ashore, hex, side, march and skills; the map's
//! land links and terrain; the bases of the side in both hexes and their
//! stock; the ground figures. Writes: a land unit's hex and march; a base's
//! supply; a new base; and the events.

use std::collections::BTreeMap;

use crate::events::{Breakdown, Cause, EventKind, Factor, MarchStop, SupplyUse};
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{BaseId, EventId, LandUnitId};
use crate::map::{Map, Terrain};
use crate::refdata::GroundFigures;
use crate::state::{Base, March, Skills};
use crate::world::World;

/// The name of a movement skill in a breakdown, by its terrain.
const fn terrain_skill_name(terrain: Terrain) -> &'static str {
    match terrain {
        Terrain::Clear => "clear_skill",
        Terrain::Jungle => "jungle_skill",
        Terrain::Mountain => "mountain_skill",
        Terrain::Atoll => "atoll_skill",
    }
}

/// The factors a breakdown names where a unit's weight acts (decision 5 of
/// 0098): its weight and the factor.
pub(crate) fn weight_factors(weight: u8, factor: Fixed) -> [Factor; 2] {
    [
        Factor {
            name: "weight",
            value: Fixed::from_int(i64::from(weight)),
        },
        Factor {
            name: "weight_factor",
            value: factor,
        },
    ]
}

/// The terrain of a hex for a march (decision 4 of 0098). A hex the map
/// gives no terrain counts as jungle, as the generated map marks every land
/// hex (decision 0052, 14).
pub(crate) fn terrain_at(map: &Map, hex: Hex) -> Terrain {
    map.get(hex)
        .and_then(|data| data.terrain)
        .unwrap_or(Terrain::Jungle)
}

/// The movement skill that counts for a march from `from` to `to`: the
/// lower of the unit's skills for the terrain of the two hexes (decision 4
/// of 0098).
pub(crate) fn march_skill(map: &Map, skills: &Skills, from: Hex, to: Hex) -> u8 {
    skills
        .movement(terrain_at(map, from))
        .min(skills.movement(terrain_at(map, to)))
}

impl World {
    /// The day's marches (decision 0094, 4 to 10). Returns, for each unit
    /// that marched a day, the event of that day's march: the fatigue step
    /// reads it (decision 8).
    pub(crate) fn march_land_units(
        &mut self,
        day: u32,
        figures: &GroundFigures,
    ) -> BTreeMap<LandUnitId, EventId> {
        let marching: Vec<(LandUnitId, March)> = self
            .truth
            .land_units
            .values()
            .filter_map(|unit| unit.march.map(|march| (unit.id, march)))
            .collect();
        let mut marched: BTreeMap<LandUnitId, EventId> = BTreeMap::new();
        for (unit_id, march) in marching {
            let unit = &self.truth.land_units[&unit_id];
            // Only men ashore march (decision 6). A unit with none left, or
            // one whose men are now in a hex with no link to where it was
            // going, stops.
            let stop = match unit.hex.filter(|_| unit.men_ashore > 0) {
                None => Some(MarchStop::NoMenAshore),
                Some(here) if !self.map.has_land_link(here, march.to) => {
                    Some(MarchStop::NoLandLink)
                }
                Some(_) => None,
            };
            if let Some(reason) = stop {
                self.truth
                    .land_units
                    .get_mut(&unit_id)
                    .expect("the land unit exists")
                    .march = None;
                self.events.record(
                    day,
                    None,
                    EventKind::MarchStopped {
                        unit: unit_id,
                        to: march.to,
                        days: march.days,
                        reason,
                    },
                    vec![Cause::Order(march.order)],
                    None,
                );
                continue;
            }
            let from = unit.hex.expect("a unit with men ashore has a hex");
            // The day counts as the movement factor of the lower of the
            // unit's skills for the terrain of the hex it leaves and of the
            // hex it enters (decision 4 of 0098).
            let (terrain_from, terrain_to) =
                (terrain_at(&self.map, from), terrain_at(&self.map, march.to));
            let (skill_from, skill_to) = (
                unit.skills.movement(terrain_from),
                unit.skills.movement(terrain_to),
            );
            let skill = march_skill(&self.map, &unit.skills, from, march.to);
            let factor = Skills::factor(skill, figures.movement_skill_strength);
            let days = march.days + 1;
            let counted = march.counted + factor;
            let of = figures.march_days_per_hex;
            // Only where the skill acts, so a breakdown of a standard march
            // is as it was (none).
            let breakdown = (self.config.record_breakdowns && factor != Fixed::ONE).then(|| {
                let mut factors = vec![Factor {
                    name: terrain_skill_name(terrain_from),
                    value: Fixed::from_int(i64::from(skill_from)),
                }];
                if terrain_to != terrain_from {
                    factors.push(Factor {
                        name: terrain_skill_name(terrain_to),
                        value: Fixed::from_int(i64::from(skill_to)),
                    });
                }
                factors.extend([
                    Factor {
                        name: "movement_skill",
                        value: Fixed::from_int(i64::from(skill)),
                    },
                    Factor {
                        name: "movement_skill_strength",
                        value: figures.movement_skill_strength,
                    },
                    Factor {
                        name: "movement_factor",
                        value: factor,
                    },
                    Factor {
                        name: "counted_before",
                        value: march.counted,
                    },
                ]);
                Breakdown {
                    factors,
                    result: counted,
                }
            });
            let day_event = self.events.record(
                day,
                None,
                EventKind::MarchDay {
                    unit: unit_id,
                    from,
                    to: march.to,
                    days,
                    counted,
                    of,
                },
                vec![Cause::Order(march.order)],
                breakdown,
            );
            marched.insert(unit_id, day_event);
            if counted < Fixed::from_int(i64::from(of)) {
                self.truth
                    .land_units
                    .get_mut(&unit_id)
                    .expect("the land unit exists")
                    .march = Some(March {
                    days,
                    counted,
                    ..march
                });
                continue;
            }
            self.arrive(day, unit_id, from, march.to, day_event, figures);
        }
        marched
    }

    /// A unit's men ashore arrive in `to` with the supply they carry
    /// (decisions 4, 9 and 10): `march_supply_days` days of the daily charge
    /// for its men, taken from its side's bases in `from` in base order as
    /// far as their stock goes, and put into its side's lowest-numbered base
    /// in `to`, or into a base formed there. Nothing carried forms no base.
    fn arrive(
        &mut self,
        day: u32,
        unit_id: LandUnitId,
        from: Hex,
        to: Hex,
        day_event: EventId,
        figures: &GroundFigures,
    ) {
        let unit = &self.truth.land_units[&unit_id];
        let (side, men) = (unit.side, unit.men_ashore);
        // Its men's daily charge, times its weight factor (decision 5 of
        // 0098).
        let weight = unit.skills.weight;
        let weight_factor = Skills::factor(weight, figures.weight_strength);
        let per_man = figures.daily_supply_t_per_man * weight_factor;
        let wanted = figures.march_supply_days * per_man * i64::from(men);
        let weighted = weight_factor != Fixed::ONE;

        // Taken from the bases of the hex left, in base order (decision 9).
        let bases_left: Vec<BaseId> = self
            .truth
            .bases
            .values()
            .filter(|base| base.side == side && base.hex == from)
            .map(|base| base.id)
            .collect();
        let mut causes = vec![Cause::Event(day_event)];
        let mut carried = Fixed::ZERO;
        for base in bases_left {
            if self.truth.bases[&base].supply_t.is_zero() || carried >= wanted {
                continue;
            }
            let asked = wanted - carried;
            let (paid_t, left_t) = self.take_supply(base, asked);
            let carried_before = carried;
            carried += paid_t;
            let breakdown = self.config.record_breakdowns.then(|| {
                let mut factors = vec![
                    Factor {
                        name: "men_marching",
                        value: Fixed::from_int(i64::from(men)),
                    },
                    Factor {
                        name: "march_supply_days",
                        value: figures.march_supply_days,
                    },
                    Factor {
                        name: "daily_supply_t_per_man",
                        value: figures.daily_supply_t_per_man,
                    },
                ];
                if weighted {
                    factors.extend(weight_factors(weight, weight_factor));
                }
                // Only where an earlier base of the side gave part.
                if !carried_before.is_zero() {
                    factors.push(Factor {
                        name: "taken_from_earlier_bases_t",
                        value: carried_before,
                    });
                }
                Breakdown {
                    factors,
                    result: asked,
                }
            });
            let spent = self.events.record(
                day,
                None,
                EventKind::BaseSupplySpent {
                    base,
                    reason: SupplyUse::March,
                    aircraft: 0,
                    men,
                    asked_t: asked,
                    paid_t,
                    left_t,
                },
                vec![Cause::Event(day_event)],
                breakdown,
            );
            causes.push(Cause::Event(spent));
        }

        // Put into the side's lowest-numbered base in the new hex, or a base
        // formed there as a landing forms one (decisions 9 and 10).
        let mut base = None;
        if carried > Fixed::ZERO {
            let existing = self
                .truth
                .bases
                .values()
                .find(|base| base.side == side && base.hex == to)
                .map(|base| base.id);
            let into = match existing {
                Some(id) => id,
                None => {
                    let id = BaseId::new(
                        self.truth
                            .bases
                            .keys()
                            .next_back()
                            .map_or(1, |id| id.raw() + 1),
                    );
                    self.truth.bases.insert(
                        id,
                        // A base formed by a march has no airfield
                        // (decision 5 of 0099).
                        Base {
                            on_map: false,
                            airfield: false,
                            ..Base::new(id, format!("Dump at {to}"), side, to)
                        },
                    );
                    let formed = self.events.record(
                        day,
                        None,
                        EventKind::MarchBaseFormed {
                            base: id,
                            side,
                            hex: to,
                            unit: unit_id,
                        },
                        causes.clone(),
                        None,
                    );
                    causes.push(Cause::Event(formed));
                    id
                }
            };
            self.truth
                .bases
                .get_mut(&into)
                .expect("the base exists")
                .supply_t += carried;
            base = Some(into);
        }

        let unit = self
            .truth
            .land_units
            .get_mut(&unit_id)
            .expect("the land unit exists");
        unit.hex = Some(to);
        unit.march = None;
        let breakdown = self.config.record_breakdowns.then(|| {
            let mut factors = vec![
                Factor {
                    name: "men_marching",
                    value: Fixed::from_int(i64::from(men)),
                },
                Factor {
                    name: "march_supply_days",
                    value: figures.march_supply_days,
                },
                Factor {
                    name: "daily_supply_t_per_man",
                    value: figures.daily_supply_t_per_man,
                },
            ];
            if weighted {
                factors.extend(weight_factors(weight, weight_factor));
            }
            factors.push(Factor {
                name: "wanted_t",
                value: wanted,
            });
            Breakdown {
                factors,
                result: carried,
            }
        });
        self.events.record(
            day,
            None,
            EventKind::MarchArrived {
                unit: unit_id,
                from,
                to,
                men,
                supply_t: carried,
                base,
            },
            causes,
            breakdown,
        );
    }
}
