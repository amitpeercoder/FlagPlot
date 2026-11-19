//! The estimate on records made by the test (scenario 95's properties "met
//! and lost again", "a place with no enemy base is met by a landing" and "a
//! plan displaced, by counting"; decision 0176, amended 2, 4 and 11). The
//! records hold no world: these read only the days' bases. Compiled only for
//! the crate's own tests; the source test does not read it.

use std::collections::{BTreeMap, BTreeSet};

use sim_core::command::IntentKind;
use sim_core::hex::Hex;
use sim_core::ids::IntentId;

use super::{met, met_in, other_plan, Displaced, End, Kept, Spread};
use crate::play::{BasesAt, PlayedDay, PlayedWorld};

const A: Hex = Hex { q: 0, r: 0 };
const B: Hex = Hex { q: 4, r: 0 };

fn end() -> End {
    End { places: vec![B] }
}

/// A day whose end has the side's bases at `own` and the enemy's at `enemy`.
fn day(day: u32, own: &[Hex], enemy: &[Hex]) -> PlayedDay {
    PlayedDay {
        day,
        hash: 0,
        staff: BTreeMap::new(),
        bases: BasesAt {
            own: own.iter().copied().collect::<BTreeSet<Hex>>(),
            enemy: enemy.iter().copied().collect::<BTreeSet<Hex>>(),
        },
    }
}

/// Eight days: B the side's (a base of its own, none of the enemy's) on the
/// days `ours` says, the enemy's on the others.
fn days(ours: impl Fn(u32) -> bool) -> Vec<PlayedDay> {
    (1..=8)
        .map(|d| {
            if ours(d) {
                day(d, &[A, B], &[])
            } else {
                day(d, &[A], &[B])
            }
        })
        .collect()
}

fn world(number: u32, days: Vec<PlayedDay>) -> PlayedWorld {
    PlayedWorld {
        number,
        assumptions: Vec::new(),
        days,
        refused: Vec::new(),
        failed: None,
        plan_orders: None,
        world: None,
    }
}

#[test]
fn a_place_is_the_sides_where_it_holds_a_base_and_the_enemy_none() {
    assert!(met(&end(), &day(1, &[A, B], &[]).bases));
    assert!(
        !met(&end(), &day(1, &[A, B], &[B]).bases),
        "both hold a base"
    );
    assert!(
        !met(&end(), &day(1, &[A], &[]).bases),
        "no base of the side's"
    );
    let both = End { places: vec![A, B] };
    assert!(!met(&both, &day(1, &[A], &[]).bases), "every place");
    assert!(met(&both, &day(1, &[A, B], &[]).bases));
}

#[test]
fn met_and_lost_again() {
    // The side's at the end of days 3 to 5 and not from day 6: met on day 3
    // and lost again, and not met and kept.
    let lost = days(|d| (3..=5).contains(&d));
    assert_eq!(met_in(&lost, &end()), (Some(3), true));
    // The side's on days 3 to 5, not on day 6, and again from day 7: met on
    // day 3 and kept.
    let back = days(|d| (3..=5).contains(&d) || d >= 7);
    assert_eq!(met_in(&back, &end()), (Some(3), false));
    // Met on the last day: kept.
    assert_eq!(met_in(&days(|d| d == 8), &end()), (Some(8), false));
    assert_eq!(met_in(&days(|_| false), &end()), (None, false));
}

#[test]
fn a_place_with_no_enemy_base_is_met_by_a_landing() {
    // The enemy has no base at B; the side first has one at the end of day 2.
    let landing: Vec<PlayedDay> = (1..=8)
        .map(|d| {
            if d >= 2 {
                day(d, &[A, B], &[])
            } else {
                day(d, &[A], &[])
            }
        })
        .collect();
    assert_eq!(met_in(&landing, &end()), (Some(2), false));
}

/// Four worlds, met and kept on the days given, none where `None`.
fn worlds(met_on: [Option<u32>; 4]) -> Vec<PlayedWorld> {
    (1..)
        .zip(met_on)
        .map(|(number, on)| world(number, days(|d| on.is_some_and(|first| d >= first))))
        .collect()
}

fn refs(worlds: &[PlayedWorld]) -> Vec<&PlayedWorld> {
    worlds.iter().collect()
}

#[test]
fn a_plan_displaced_by_counting() {
    let kind = IntentKind::TakePlace {
        hex: B,
        by_day: None,
    };
    let intent = IntentId::new(2);
    // Met and kept in 4 of 4 without the course and in 2 with it.
    let without = worlds([Some(5); 4]);
    let with = worlds([Some(5), None, Some(5), None]);
    let plan = other_plan(intent, kind, &end(), &refs(&with), &refs(&without));
    assert_eq!(
        plan.with_course,
        Kept {
            worlds: vec![1, 3],
            middle_day: Some(5)
        }
    );
    assert_eq!(
        plan.without,
        Kept {
            worlds: vec![1, 2, 3, 4],
            middle_day: Some(5)
        }
    );
    assert_eq!(plan.displaced, Some(Displaced::InFewerWorlds));
    // Met in 4 and 4, the middle days 5 without and 7 with: displaced by its
    // days.
    let later = worlds([Some(7); 4]);
    let plan = other_plan(intent, kind, &end(), &refs(&later), &refs(&without));
    assert_eq!(
        (plan.with_course.middle_day, plan.without.middle_day),
        (Some(7), Some(5))
    );
    assert_eq!(plan.displaced, Some(Displaced::Later));
    // Met in 4 and 4 on the same days: not named.
    let plan = other_plan(intent, kind, &end(), &refs(&without), &refs(&without));
    assert_eq!(plan.displaced, None);
}

#[test]
fn the_middle_is_the_value_at_place_ceil_n_over_2() {
    assert_eq!(Spread::of(&[]), None);
    let s = |lowest, middle, highest| Spread {
        lowest,
        middle,
        highest,
    };
    assert_eq!(Spread::of(&[100, 300, 100, 300]), Some(s(100, 100, 300)));
    assert_eq!(Spread::of(&[7]), Some(s(7, 7, 7)));
    assert_eq!(Spread::of(&[5, 1, 9]), Some(s(1, 5, 9)));
}
