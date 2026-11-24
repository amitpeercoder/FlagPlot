//! Stage 4's proof, case R: preparation matters (decision 0161, condition
//! 9). The same order to take a place, at press and not before day 9, is
//! handed in on day 1 in one course and on day 4 in the other. Both land
//! on day 11. The landing prepared from day 1 puts its men ashore fresh;
//! the one prepared from day 4 puts them ashore tired.
//!
//! The data is placeholder data: see
//! `data/scenarios/stage-4-proof/case_r_preparation.toml`. It runs on the
//! frozen step 1 figures with its own ground figures file,
//! `ground_placeholder_stage_4_proof_r.toml`, and scenario 95's files of
//! the trial's figures and of sides. No enemy man is ashore in any world,
//! so no die is rolled and every world is the same.
//!
//! **Written before the case is built.** Every test below is empty and
//! marked `#[ignore = "statement written, not built"]`. It is a case of a
//! proof: the harness plays it once, for the judged run, and these tests
//! hold its statement so that the working can be checked before.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. If the code disagrees, the
//! code or the rule is wrong: do not change a number here to make a test
//! pass. A mismatch goes back to the architect.
//!
//! The day counts were first written in decision 0161 as 80 and 50 of
//! rehearsal, 920 and 800 fit men. That was wrong: it stopped the count at
//! the day the plan is released, and a landing goes on being made ready
//! for the two days it takes to load. The figures below replace them.
//! Each course was played once as a game by the architect on Nov 24,
//! 2026, with its commands written into a copy of the scenario, to check
//! this working; they agreed. No trial of the case has been played.
//!
//! ## The rules it reads, and their figures
//!
//! - **Preparing a landing** (decision 0095): a force with the order
//!   gains `preparation_per_day`, here 10, of loading for each day in
//!   port, and each unit of its roster the same of rehearsal, up to 100.
//! - **When a landing is ready** (decision 0141): at press, when the
//!   loading and each unit's rehearsal are at 50.
//! - **Told to wait** (decision 0159, 9): with "not before day 9" the
//!   plan is not released before the morning of day 9, and the orders to
//!   prepare stand through the wait.
//! - **The days of a lift** (scenario 76's): released on a morning, the
//!   staff orders the supply loaded that day, the unit set to attack and
//!   loaded the next, and the route and the unloading the day after. The
//!   passage of 160 nm takes 16 hours and the men land that day.
//! - **Fatigue on landing** (decision 0095): `unprepared_landing_fatigue`,
//!   here 40, times 100 less the unit's rehearsal, over 100.
//! - **Rest** (decision 0090): a unit that did not fire in the day loses
//!   `rest_per_day`, 5, of fatigue at the day's end.
//! - **Fit men** (decision 0143): men ashore times 100 less the fatigue,
//!   over 100, cut to a whole man.
//! - `unprepared_landing_share` is 1: the loading does not change how fast
//!   the men land, so both courses have all their men ashore on one day.
//!
//! ## Inputs
//!
//! Scenario 27's map: A at (0,0), port size 5; B at (4,0), port size 0;
//! 160 nm apart. Base 1 at A, Allied, 1,000 t. Base 2 at B, Japanese, on
//! the Allied map, with no man. Task force 1, two placeholder troop
//! transports of 600 men each, at base 1. Unit 1, 1,000 fighting men, at
//! A. No estimate and no contact. 14 days.
//!
//! **The course,** six commands of the Allied side, all on one day:
//! intent 1, take the place at (4,0); its forces limit, task force 1,
//! unit 1 and base 1 as the source; its risk, press; "not before day 9";
//! leave to prepare; leave to launch. **The early course** hands them in
//! on day 1 and **the late course** on day 4.
//!
//! **The trials:** of the Allied side, from the evening before day 1,
//! two trials of one number, so the same worlds: 16 worlds for 14 days,
//! the standard size, one with each course.
//!
//! ## The early course
//!
//! | Day | Loading and rehearsal at the day's end | The staff |
//! | --- | --- | --- |
//! | 1 | 10 | Approves the plan to preparing. `prepare_landing` at B, roster 1 |
//! | 2 to 5 | 20, 30, 40, 50 | |
//! | 6 | 60 | Notice: the landing is ready (50 on the morning of day 6) |
//! | 7, 8 | 70, 80 | Waits: not before day 9 |
//! | 9 | 90 | Releases the plan. A load of supply |
//! | 10 | 100 | Unit 1 set to attack. Load unit 1 |
//! | 11 | | A route to B and back, acting at B, and an unload order. All 1,000 men land |
//!
//! Unit 1 lands with a rehearsal of 100: its fatigue on landing is
//! 40 x 0 / 100 = 0. At the end of day 11 its fatigue is 0 and its fit
//! men are 1,000.
//!
//! ## The late course
//!
//! | Day | Loading and rehearsal at the day's end | The staff |
//! | --- | --- | --- |
//! | 4 | 10 | Approves the plan to preparing. `prepare_landing` at B, roster 1 |
//! | 5 to 8 | 20, 30, 40, 50 | |
//! | 9 | 60 | Notice: the landing is ready (50 on the morning of day 9). Releases the plan. A load of supply |
//! | 10 | 70 | Unit 1 set to attack. Load unit 1 |
//! | 11 | | A route to B and back, acting at B, and an unload order. All 1,000 men land |
//!
//! Unit 1 lands with a rehearsal of 70: its fatigue on landing is
//! 40 x 30 / 100 = 12. It does not fire, and rests 5 at the day's end:
//! its fatigue is 7 and its fit men are 1,000 x 93 / 100 = 930.
//!
//! ## What the condition reads
//!
//! In each of the 16 worlds, on the evening of day 11, the fit men of
//! unit 1 by the side's own view: 1,000 in the early course against 930
//! in the late. The early course has more in all 16 worlds; the mark is
//! at least 15.
//!
//! Each course gives six staff orders. The men are ashore on the same
//! day in both, which is what the condition asks: only the day
//! preparation began differs.
#![allow(clippy::doc_lazy_continuation, clippy::doc_markdown)]

/// The early course, as a game and in every world: the loading and the
/// rehearsal at each day's end, the notice of day 6, the release of day 9,
/// the six orders on their days, and 1,000 men ashore on day 11 with
/// fatigue 0.
#[test]
#[ignore = "statement written, not built"]
fn the_early_course() {}

/// The late course: the same, with the notice and the release on day 9,
/// rehearsal 70 at the landing, fatigue 12 on landing and 7 at the end of
/// day 11.
#[test]
#[ignore = "statement written, not built"]
fn the_late_course() {}

/// Both courses land every man on day 11.
#[test]
#[ignore = "statement written, not built"]
fn both_land_on_the_same_day() {}

/// The two trials of one number have the same worlds, and in each world the
/// fit men of unit 1 on the evening of day 11 are 1,000 against 930.
#[test]
#[ignore = "statement written, not built"]
fn fit_men_world_for_world() {}
