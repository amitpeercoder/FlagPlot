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
}
