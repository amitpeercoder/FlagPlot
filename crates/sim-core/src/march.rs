//! Marching overland (`docs/decisions/0094-marching-overland.md`).
//!
//! The marches are the first step of the ground combat phase, before the
//! day's fights. Units with a march order are taken in unit order. Each
//! marches one more day; when it has marched `march_days_per_hex` days its
//! men ashore are in the new hex, with the supply it carried from its side's
//! bases in the hex it left, and its order is spent. The fatigue of a day's
//! march is gained in the phase's fatigue step, after the fights
//! (`ground_combat.rs`). There is no randomness.
//!
//! Reads: a land unit's men ashore, hex, side and march; the map's land
//! links; the bases of the side in both hexes and their stock; the ground
//! figures. Writes: a land unit's hex and march; a base's supply; a new
//! base; and the events.

use std::collections::BTreeMap;

use crate::events::{Breakdown, Cause, EventKind, Factor, MarchStop, SupplyUse};
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{BaseId, EventId, LandUnitId};
use crate::refdata::GroundFigures;
use crate::state::{Base, March};
use crate::world::World;

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
            let days = march.days + 1;
            let of = figures.march_days_per_hex;
            let day_event = self.events.record(
                day,
                None,
                EventKind::MarchDay {
                    unit: unit_id,
                    from,
                    to: march.to,
                    days,
                    of,
                },
                vec![Cause::Order(march.order)],
                None,
            );
            marched.insert(unit_id, day_event);
            if days < of {
                self.truth
                    .land_units
                    .get_mut(&unit_id)
                    .expect("the land unit exists")
                    .march = Some(March { days, ..march });
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
        let wanted = figures.march_supply_days * figures.daily_supply_t_per_man * i64::from(men);

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
                        Base {
                            on_map: false,
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
        let breakdown = self.config.record_breakdowns.then(|| Breakdown {
            factors: vec![
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
                Factor {
                    name: "wanted_t",
                    value: wanted,
                },
            ],
            result: carried,
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
