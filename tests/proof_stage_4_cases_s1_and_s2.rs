//! Stage 4's proof, cases S1 and S2: its spreads hold (decision 0161,
//! condition 7; `docs/validation/stage-4-proof-setup.md`, condition 7). Two
//! built cases with dice where the picture is right: scenario 68's map,
//! bases, ships and course, played for 21 days with scenario 76's ground
//! figures, in which a shot hits by chance. In both the side's opening
//! estimate is the garrison's true count, nothing is believed beyond it,
//! the enemy's base holds the 200 t the trial's figures assume, and the
//! enemy has no order.
//!
//! - **S1:** unit 1 alone, 600 men, against a garrison of 200, estimated at
//!   200 (`case_s1_one_unit_against_200.toml`).
//! - **S2:** units 1 and 2, 600 men each, against a garrison of 300,
//!   estimated at 300 (`case_s2_two_units_against_300.toml`).
//!
//! These tests assert only that each case loads and that the opening
//! estimate equals the garrison. The cases are not played here: the
//! spreads (a trial of 64 worlds beside 200 games) are read by the proof's
//! harness, `tests/proof_stage_4_harness.rs`, once, for the judged run;
//! the harness is tried on seed 2,001 to show that it runs.
#![allow(clippy::doc_lazy_continuation, clippy::doc_markdown)]

mod common;

use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::state::Side;

use common::stage_4::{case_text, load_case};

const B: Hex = Hex { q: 4, r: 0 };

/// The side's units and the truth's garrison and estimate of one case.
fn read(name: &str) -> (Vec<(u32, u32)>, u32, u32, Fixed) {
    let s = load_case(name, &case_text(name));
    let truth = s.world.umpire();
    let allied: Vec<(u32, u32)> = truth
        .land_units
        .values()
        .filter(|u| u.side == Side::Allied)
        .map(|u| (u.id.raw(), u.men_ashore))
        .collect();
    let garrison: u32 = truth
        .land_units
        .values()
        .filter(|u| u.side == Side::Japanese && u.hex == Some(B))
        .map(|u| u.men_ashore)
        .sum();
    let estimates: Vec<u32> = s
        .world
        .side_view(Side::Allied)
        .ground_estimates()
        .map(|e| e.men)
        .collect();
    assert_eq!(estimates.len(), 1, "one opening estimate, of B");
    assert_eq!(s.days, 21);
    assert_eq!(s.believed_men(Side::Allied).count(), 0);
    assert_eq!(s.believed_forces(Side::Allied).count(), 0);
    assert_eq!(s.believed_intents(Side::Allied).count(), 0);
    assert!(
        s.scripted_commands().is_empty(),
        "no command, no enemy order"
    );
    let supply = truth.bases[&sim_core::ids::BaseId::new(2)].supply_t;
    (allied, garrison, estimates[0], supply)
}

#[test]
fn s1_loads_and_the_estimate_is_the_garrison() {
    let (allied, garrison, estimate, supply) = read("case_s1_one_unit_against_200");
    assert_eq!(allied, vec![(1, 600)]);
    assert_eq!(garrison, 200);
    assert_eq!(estimate, garrison);
    assert_eq!(supply, Fixed::from_int(200));
}

#[test]
fn s2_loads_and_the_estimate_is_the_garrison() {
    let (allied, garrison, estimate, supply) = read("case_s2_two_units_against_300");
    assert_eq!(allied, vec![(1, 600), (2, 600)]);
    assert_eq!(garrison, 300);
    assert_eq!(estimate, garrison);
    assert_eq!(supply, Fixed::from_int(200));
}
