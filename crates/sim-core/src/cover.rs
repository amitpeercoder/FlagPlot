//! Cover, men who dig in, and a place under bombardment
//! (`docs/decisions/0106-cover-and-digging-in.md`).
//!
//! A land unit's cover is the higher of its side's lowest-numbered base's
//! defences in its hex and its own fieldworks; a unit that is marching or on
//! attack has none (decision 1). Cover cuts the men a hit on men takes from
//! a unit (decision 2, in `bombardment.rs`) and the men a defending unit
//! loses in the fight (decision 3, in `ground_combat.rs`). A fighting unit on
//! the defence that did not march digs fieldworks each day, in the ground
//! combat phase after the day's fatigue (decision 5), and loses them when it
//! marches, falls back, is taken aboard or is set to attack (decision 6). A
//! point of a base's defences costs more in each grade (decision 8, in
//! `works.rs`). Hits on men are counted for the day in each hex and side;
//! the ground combat phase settles strain on them in place of rest (decision
//! 10, in `ground_combat.rs`), and each hit wears cover (decision 12).
//!
//! Reads: a land unit's posture, march, hex, side, men ashore, working mark
//! and fieldworks; the bases of its side in its hex and their defences; the
//! ground figures. Writes: a unit's fieldworks, a base's defences, and the
//! events `FieldworksChanged` and `DefencesWorn`. Draws no random number.

use std::collections::BTreeMap;

use crate::events::{Breakdown, Cause, EventKind, Factor, FieldworksChange};
use crate::fixed::Fixed;
use crate::ids::{EventId, LandUnitId};
use crate::orders::Posture;
use crate::refdata::GroundFigures;
use crate::state::{LandUnit, TruthState};
use crate::time::Pulse;
use crate::world::World;

/// A unit's cover (decision 1 of 0106): 0 if it is marching or on attack,
/// and else the higher of the defences of its side's lowest-numbered base in
/// its hex and its own fieldworks. Reads only the unit and its side's bases.
pub(crate) fn unit_cover(truth: &TruthState, unit: &LandUnit) -> Fixed {
    if unit.march.is_some() || unit.posture == Posture::Attack {
        return Fixed::ZERO;
    }
    let Some(hex) = unit.hex else {
        return Fixed::ZERO;
    };
    let defences = truth
        .bases
        .values()
        .find(|base| base.side == unit.side && base.hex == hex)
        .map_or(Fixed::ZERO, |base| base.defences);
    defences.max(unit.fieldworks)
}

/// The men a unit loses of `men` it would lose, behind `cover` that saves
/// `share` of them at full cover (decisions 2 and 3 of 0106): the men times
/// 1 less the cover over 100 times the share, cut to a whole man. Never more
/// than `men`.
pub(crate) fn after_cover(men: u32, cover: Fixed, share: Fixed) -> u32 {
    let kept = Fixed::ONE - share.mul_div(cover, Fixed::from_int(100));
    let lost = (Fixed::from_int(i64::from(men)) * kept).trunc().max(0);
    u32::try_from(lost).unwrap_or(u32::MAX).min(men)
}

/// The piece of a unit's part `part` of a call's loss that the `k`-th of
/// `hits` hits takes (1-based), spreading the part evenly over the hits
/// (decision 2 of 0106, as built): `part` x k / hits, cut, less `part` x
/// (k - 1) / hits, cut. The pieces add to the part.
pub(crate) fn piece_of_part(part: u32, k: u32, hits: u32) -> u32 {
    let upto = |n: u32| u64::from(part) * u64::from(n) / u64::from(hits.max(1));
    u32::try_from(upto(k) - upto(k - 1)).expect("a piece is at most the part")
}

/// The fatigue strain adds to a unit in a day (decision 10 of 0106): its
/// side's hits on men in its hex times `strain_per_hit`, no more than
/// `strain_per_day`, times 1 less its cover over 100 times
/// `cover_from_strain`, and no more than takes its fatigue before the day
/// to `strain_limit`. Returns the strain and the strain before the limit.
pub(crate) fn strain(
    hits: u32,
    cover: Fixed,
    fatigue_before: Fixed,
    figures: &GroundFigures,
) -> (Fixed, Fixed) {
    let raw = (figures.strain_per_hit * i64::from(hits)).min(figures.strain_per_day);
    let cut = raw
        * (Fixed::ONE
            - figures
                .cover_from_strain
                .mul_div(cover, Fixed::from_int(100)));
    let room = (figures.strain_limit - fatigue_before).max(Fixed::ZERO);
    (cut.min(room), cut)
}

/// The factor a point of defences costs in the grade of `defences`
/// (decisions 7 and 8 of 0106): 1 below `prepared_from`,
/// `prepared_work_factor` from there to below `fortified_from`, and
/// `fortified_work_factor` from there.
pub(crate) fn grade_factor(defences: Fixed, figures: &GroundFigures) -> Fixed {
    if defences < figures.prepared_from {
        Fixed::ONE
    } else if defences < figures.fortified_from {
        figures.prepared_work_factor
    } else {
        figures.fortified_work_factor
    }
}

/// A base's defences after `work` on them (decision 4 of 0099, decision 8
/// of 0106): the work is spent point by point at the cost of each grade,
/// `work_per_defence_point` times the grade's factor, to 100 at most, and is
/// not cut to a whole point. Where the factors are equal the work is spent
/// at one cost. Returns the defences and the name and value of each grade
/// factor above 1 the work was spent at, in order.
pub(crate) fn defences_after_work(
    defences: Fixed,
    work: Fixed,
    figures: &GroundFigures,
) -> (Fixed, Vec<(&'static str, Fixed)>) {
    let hundred = Fixed::from_int(100);
    let mut at = defences;
    let mut left = work;
    let mut dearer: Vec<(&'static str, Fixed)> = Vec::new();
    while left > Fixed::ZERO && at < hundred {
        let factor = grade_factor(at, figures);
        // The top of the stretch at this cost: the next grade's start above
        // here where the cost changes, else 100.
        let top = [figures.prepared_from, figures.fortified_from]
            .into_iter()
            .find(|start| *start > at && grade_factor(*start, figures) != factor)
            .unwrap_or(hundred)
            .min(hundred);
        let name = if at < figures.fortified_from {
            "prepared_work_factor"
        } else {
            "fortified_work_factor"
        };
        if factor > Fixed::ONE && !dearer.contains(&(name, factor)) {
            dearer.push((name, factor));
        }
        let cost = figures.work_per_defence_point * factor;
        let needed = (top - at) * cost;
        if left >= needed {
            at = top;
            left -= needed;
        } else {
            at += left / cost;
            left = Fixed::ZERO;
        }
    }
    (at.min(hundred), dearer)
}

/// Whether a unit digs today (decision 5 of 0106): a fighting unit with men
/// ashore, on defend, not marching and not marched today.
pub(crate) fn digs(unit: &LandUnit, marched_today: bool) -> bool {
    !unit.working
        && unit.men_ashore > 0
        && unit.hex.is_some()
        && unit.posture == Posture::Defend
        && unit.march.is_none()
        && !marched_today
}

impl World {
    /// The day's digging, in unit order, after the day's fatigue (decision 5
    /// of 0106): each unit that digs gains `fieldworks_per_day`, up to
    /// `fieldworks_limit`. A unit whose fieldworks do not change records
    /// nothing.
    pub(crate) fn dig_in(
        &mut self,
        day: u32,
        marched: &BTreeMap<LandUnitId, EventId>,
        figures: &GroundFigures,
    ) {
        let units: Vec<LandUnitId> = self.truth.land_units.keys().copied().collect();
        for unit_id in units {
            let unit = &self.truth.land_units[&unit_id];
            if !digs(unit, marched.contains_key(&unit_id)) {
                continue;
            }
            let before = unit.fieldworks;
            let after = (before + figures.fieldworks_per_day).min(figures.fieldworks_limit);
            if after <= before {
                continue;
            }
            self.truth
                .land_units
                .get_mut(&unit_id)
                .expect("the land unit exists")
                .fieldworks = after;
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "fieldworks_before",
                        value: before,
                    },
                    Factor {
                        name: "fieldworks_per_day",
                        value: figures.fieldworks_per_day,
                    },
                    Factor {
                        name: "fieldworks_limit",
                        value: figures.fieldworks_limit,
                    },
                ],
                result: after,
            });
            self.events.record(
                day,
                None,
                EventKind::FieldworksChanged {
                    unit: unit_id,
                    change: FieldworksChange::Dug,
                    fieldworks: after,
                },
                Vec::new(),
                breakdown,
            );
        }
    }

    /// A unit leaves its fieldworks (decision 6 of 0106): they go to 0, by
    /// an event citing `cause`, if it had any.
    pub(crate) fn lose_fieldworks(
        &mut self,
        day: u32,
        pulse: Option<Pulse>,
        unit_id: LandUnitId,
        cause: Cause,
    ) {
        let Some(unit) = self.truth.land_units.get_mut(&unit_id) else {
            return;
        };
        if unit.fieldworks <= Fixed::ZERO {
            return;
        }
        unit.fieldworks = Fixed::ZERO;
        self.events.record(
            day,
            pulse,
            EventKind::FieldworksChanged {
                unit: unit_id,
                change: FieldworksChange::Lost,
                fieldworks: Fixed::ZERO,
            },
            vec![cause],
            None,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fx(text: &str) -> Fixed {
        text.parse().expect("a decimal")
    }

    fn live() -> GroundFigures {
        GroundFigures {
            cover_from_fire: fx("0.9"),
            cover_in_fight: fx("0.5"),
            fieldworks_per_day: fx("10"),
            fieldworks_limit: fx("30"),
            prepared_from: fx("30"),
            fortified_from: fx("60"),
            prepared_work_factor: fx("3"),
            fortified_work_factor: fx("10"),
            strain_per_hit: fx("1"),
            strain_per_day: fx("10"),
            strain_limit: fx("60"),
            cover_from_strain: fx("0.5"),
            cover_wear_per_hit: fx("0.2"),
            ..GroundFigures::FIRST
        }
    }

    #[test]
    fn cover_cuts_a_loss_to_a_whole_man_and_never_raises_it() {
        assert_eq!(after_cover(10, fx("20"), fx("0.9")), 8);
        assert_eq!(after_cover(10, fx("100"), fx("0.9")), 1);
        assert_eq!(after_cover(20, fx("80"), fx("0.5")), 12);
        assert_eq!(after_cover(7, fx("50"), Fixed::ZERO), 7);
        for men in 0..40 {
            for cover in [0, 13, 50, 100] {
                assert!(after_cover(men, Fixed::from_int(cover), fx("0.9")) <= men);
            }
        }
    }

    #[test]
    fn the_pieces_of_a_part_spread_it_evenly_and_add_to_it() {
        assert_eq!(
            (1..=3).map(|k| piece_of_part(10, k, 3)).collect::<Vec<_>>(),
            vec![3, 3, 4]
        );
        for (part, hits) in [(0, 4), (30, 3), (7, 9), (1000, 7)] {
            let total: u32 = (1..=hits).map(|k| piece_of_part(part, k, hits)).sum();
            assert_eq!(total, part);
        }
    }

    #[test]
    fn each_grade_costs_more_and_frozen_factors_give_the_old_rule() {
        let figures = live();
        // Scenario 34, base 2: 1800 at 30 is 3 points.
        assert_eq!(
            defences_after_work(fx("30"), fx("1800"), &figures).0,
            fx("33")
        );
        // Base 3: 1500 at 59 is one point at 600, then 900 / 2000.
        assert_eq!(
            defences_after_work(fx("59"), fx("1500"), &figures).0,
            fx("60.45")
        );
        assert_eq!(
            defences_after_work(fx("60.45"), fx("1500"), &figures).0,
            fx("61.2")
        );
        // The property: 800 at 29 gives 31.
        assert_eq!(
            defences_after_work(fx("29"), fx("800"), &figures).0,
            fx("31")
        );
        // Full defences cost 104,000.
        assert_eq!(
            defences_after_work(Fixed::ZERO, fx("104000"), &figures).0,
            fx("100")
        );
        assert!(defences_after_work(Fixed::ZERO, fx("103999"), &figures).0 < fx("100"));
        // The frozen factors: the work over 200, to 100.
        let frozen = GroundFigures::FIRST;
        for (at, work) in [
            ("0", "1000"),
            ("29.5", "1000"),
            ("59", "1500"),
            ("99", "999"),
        ] {
            let old = (fx(at) + fx(work) / fx("200")).min(fx("100"));
            let (new, dearer) = defences_after_work(fx(at), fx(work), &frozen);
            assert_eq!(new, old);
            assert!(dearer.is_empty());
        }
    }

    #[test]
    fn strain_is_capped_by_the_day_cut_by_cover_and_stops_at_the_limit() {
        let figures = live();
        assert_eq!(strain(3, fx("20"), Fixed::ZERO, &figures).0, fx("2.7"));
        assert_eq!(strain(12, Fixed::ZERO, Fixed::ZERO, &figures).0, fx("10"));
        assert_eq!(strain(12, fx("100"), Fixed::ZERO, &figures).0, fx("5"));
        assert_eq!(strain(9, fx("50"), fx("53"), &figures).0, fx("6.75"));
        assert_eq!(strain(12, fx("50"), fx("58"), &figures).0, fx("2"));
        assert_eq!(strain(12, fx("50"), fx("65"), &figures).0, Fixed::ZERO);
        assert_eq!(
            strain(5, Fixed::ZERO, Fixed::ZERO, &GroundFigures::FIRST).0,
            Fixed::ZERO
        );
    }
}
