//! World for world, by counting (scenario 96's property; decision 0177,
//! amended 10), on records made by the test. Compiled only for the crate's
//! own tests; the source test does not read it.

use super::{compare, WorldForWorld};
use crate::estimate::{Losses, WorldFigures};

/// A world's own figures: met on `met` or not, and lost again or not.
fn world(number: u32, met: Option<u32>, lost_again: bool) -> WorldFigures {
    WorldFigures {
        number,
        met_day: met,
        lost_again,
        side: Losses::default(),
        enemy: Losses::default(),
        assumptions: Vec::new(),
    }
}

#[test]
fn five_worlds_give_one_in_each_count_and_one_apart() {
    // World 1 met and kept at both; 2 at the first alone; 3 at the later
    // alone; 4 at neither; 5 could not be played at the later date.
    let first = [
        world(1, Some(4), false),
        world(2, Some(4), false),
        world(3, None, false),
        world(4, None, false),
        world(5, Some(4), false),
    ];
    let later = [
        world(1, Some(8), false),
        world(2, None, false),
        world(3, Some(8), false),
        world(4, None, false),
    ];
    assert_eq!(
        compare(&first, &[], &later, &[5]),
        WorldForWorld {
            both: vec![1],
            first_alone: vec![2],
            later_alone: vec![3],
            neither: vec![4],
            apart: vec![5],
        }
    );
}

#[test]
fn met_and_lost_again_counts_as_not_met_and_kept() {
    let first = [world(1, Some(3), true), world(2, Some(3), false)];
    let later = [world(1, Some(8), false), world(2, Some(5), true)];
    assert_eq!(
        compare(&first, &[], &later, &[]),
        WorldForWorld {
            both: vec![],
            first_alone: vec![2],
            later_alone: vec![1],
            neither: vec![],
            apart: vec![],
        }
    );
}

#[test]
fn a_world_that_could_not_be_played_at_the_first_is_counted_apart() {
    let later = [world(1, Some(8), false), world(2, Some(8), false)];
    let w = compare(&[world(2, Some(4), false)], &[1], &later, &[]);
    assert_eq!(w.apart, vec![1]);
    assert_eq!(w.both, vec![2]);
}
