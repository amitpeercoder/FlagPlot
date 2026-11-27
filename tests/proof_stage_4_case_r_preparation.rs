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

use crate::common;

use sim_core::command::NoticeKind;
use sim_core::fixed::Fixed;
use sim_core::ids::LandUnitId;
use sim_core::state::{Side, TruthState};
use sim_data::Scenario;

use common::stage_4::{
    case_text, commands_on, course_of, hex, load_case, notice_days, orders_on, play_game,
    step_trial, world_commands_on, world_orders_on, Game, Given, Stepped4,
};

const NAME: &str = "case_r_preparation";
const ALLIED: Side = Side::Allied;
const A: sim_core::hex::Hex = sim_core::hex::Hex { q: 0, r: 0 };

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

/// The six commands of the course, all on `day`: intent 1, take the place
/// at (4,0); its forces limit; its risk, press; "not before day 9"; leave
/// to prepare; leave to launch.
fn course_text(day: u32) -> String {
    format!(
        r#"
[[command]]
day = {day}
side = "allied"
kind = "set_intent"
intent = 1
intent_kind = "take_place"
hex = [4, 0]

[[command]]
day = {day}
side = "allied"
kind = "set_limit"
limit = "forces"
intent = 1
task_forces = [1]
land_units = [1]
source = 1

[[command]]
day = {day}
side = "allied"
kind = "set_limit"
limit = "risk"
intent = 1
risk = "press"

[[command]]
day = {day}
side = "allied"
kind = "set_limit"
limit = "not_before"
intent = 1
from_day = 9

[[command]]
day = {day}
side = "allied"
kind = "set_leave"
leave = "prepare"
given = true

[[command]]
day = {day}
side = "allied"
kind = "set_leave"
leave = "launch"
given = true
"#
    )
}

/// The case with its course handed in on `day`.
fn scenario_on(day: u32) -> Scenario {
    load_case(NAME, &format!("{}\n{}", case_text(NAME), course_text(day)))
}

fn game_on(day: u32) -> Game {
    play_game(scenario_on(day), ALLIED)
}

fn unit_1(truth: &TruthState) -> &sim_core::state::LandUnit {
    &truth.land_units[&LandUnitId::new(1)]
}

/// Loading of task force 1 and rehearsal of unit 1 at the end of a day.
fn prepared(truth: &TruthState) -> (Option<Fixed>, Fixed) {
    let loading = truth
        .task_forces
        .values()
        .find(|t| t.id.raw() == 1)
        .and_then(|t| t.preparation.as_ref())
        .map(|p| p.loading);
    (loading, unit_1(truth).rehearsal)
}

fn approves(commands: &[String]) -> bool {
    commands.iter().any(|c| c == "Approve { plan: PlanId(1) }")
}

/// The orders of a lift as the tables have them: the landing prepared on
/// the day the plan is approved to preparing, the load of supply on the day
/// it is released, the unit set to attack and loaded the next, the route
/// and the unload the day after; six staff orders in all.
fn lift_orders(game: &Game, prepare: u32, release: u32) {
    let on = |d| orders_on(game, d, ALLIED);
    assert_eq!(on(prepare), vec![Given::Landing(hex(4, 0), vec![1])]);
    let load = on(release);
    assert!(matches!(load.as_slice(), [Given::LoadTo(_)]), "{load:?}");
    let mut set = on(release + 1);
    set.sort_by_key(|g| format!("{g:?}"));
    assert_eq!(set, vec![Given::Attack(1), Given::LoadUnit(1)]);
    let mut last = on(release + 2);
    last.sort_by_key(|g| format!("{g:?}"));
    assert_eq!(
        last,
        vec![
            Given::Route(vec![hex(4, 0), A], vec![hex(4, 0)]),
            Given::Unload
        ]
    );
    let all: usize = (1..=14).map(|d| on(d).len()).sum();
    assert_eq!(all, 6, "six staff orders in all");
}

/// The loading and the rehearsal at the end of days `first` to 10 are 10,
/// 20, ... 100 counted from the day the landing was ordered.
fn rehearsal_days(game: &Game, ordered: u32) {
    for day in ordered..=10 {
        let (loading, rehearsal) = prepared(&game.day(day).truth);
        let want = n(10 * i64::from(day - ordered + 1));
        assert_eq!(loading, Some(want), "loading, day {day}");
        assert_eq!(rehearsal, want, "rehearsal, day {day}");
    }
}

fn trial_of(course_day: u32) -> Stepped4 {
    let scenario = scenario_on(course_day);
    let course = course_of(&scenario, ALLIED);
    assert_eq!(course.len(), 6);
    assert!(course.iter().all(|c| c.day == course_day));
    step_trial(&scenario, ALLIED, 1, 16, 14, course)
}

/// The early course, as a game and in every world: the loading and the
/// rehearsal at each day's end, the notice of day 6, the release of day 9,
/// the six orders on their days, and 1,000 men ashore on day 11 with
/// fatigue 0.
#[test]
fn the_early_course() {
    let game = game_on(1);
    rehearsal_days(&game, 1);
    for day in 1..=14 {
        assert_eq!(
            approves(&commands_on(&game, day, ALLIED)),
            day == 1 || day == 9,
            "approval, day {day}"
        );
    }
    lift_orders(&game, 1, 9);
    assert_eq!(notice_days(&game, ALLIED, NoticeKind::Ready), vec![6]);
    let day11 = game.day(11);
    let unit = unit_1(&day11.truth);
    assert_eq!(unit.men_ashore, 1000);
    assert_eq!(unit.hex, Some(hex(4, 0)));
    assert_eq!(unit.fatigue, n(0));
    assert_eq!(day11.fit[&1], 1000);
    // And in every world the same, day by day.
    let trial = trial_of(1);
    for w in &trial.played.worlds {
        for day in 1..=10 {
            let (loading, rehearsal) = prepared(&trial.snapshot(w.number, day).truth);
            assert_eq!(loading, Some(n(10 * i64::from(day))), "world {}", w.number);
            assert_eq!(rehearsal, n(10 * i64::from(day)), "world {}", w.number);
        }
        let ashore = unit_1(&trial.snapshot(w.number, 11).truth);
        assert_eq!((ashore.men_ashore, ashore.fatigue), (1000, n(0)));
    }
}

/// The late course: the same, with the notice and the release on day 9,
/// rehearsal 70 at the landing, fatigue 12 on landing and 7 at the end of
/// day 11.
#[test]
fn the_late_course() {
    let game = game_on(4);
    rehearsal_days(&game, 4);
    for day in 1..=14 {
        assert_eq!(
            approves(&commands_on(&game, day, ALLIED)),
            day == 4 || day == 9,
            "approval, day {day}"
        );
    }
    lift_orders(&game, 4, 9);
    assert_eq!(notice_days(&game, ALLIED, NoticeKind::Ready), vec![9]);
    let day11 = game.day(11);
    let unit = unit_1(&day11.truth);
    assert_eq!(unit.men_ashore, 1000);
    assert_eq!(unit.hex, Some(hex(4, 0)));
    // 40 x 30 / 100 = 12 on landing, less the day's rest of 5.
    assert_eq!(unit.fatigue, n(7));
    assert_eq!(day11.fit[&1], 930);
    let trial = trial_of(4);
    for w in &trial.played.worlds {
        for day in 4..=10 {
            let (loading, rehearsal) = prepared(&trial.snapshot(w.number, day).truth);
            let want = n(10 * i64::from(day - 3));
            assert_eq!(loading, Some(want), "world {}", w.number);
            assert_eq!(rehearsal, want, "world {}", w.number);
        }
        let ashore = unit_1(&trial.snapshot(w.number, 11).truth);
        assert_eq!((ashore.men_ashore, ashore.fatigue), (1000, n(7)));
    }
}

/// Both courses land every man on day 11.
#[test]
fn both_land_on_the_same_day() {
    for course_day in [1, 4] {
        let game = game_on(course_day);
        for day in 1..=10 {
            assert_ne!(
                unit_1(&game.day(day).truth).hex,
                Some(hex(4, 0)),
                "course on day {course_day}: no man ashore at B on day {day}"
            );
        }
        let day11 = unit_1(&game.day(11).truth);
        assert_eq!(
            (day11.men_ashore, day11.hex),
            (1000, Some(hex(4, 0))),
            "course on day {course_day}"
        );
    }
}

/// The two trials of one number have the same worlds, and in each world the
/// fit men of unit 1 on the evening of day 11 are 1,000 against 930.
#[test]
fn fit_men_world_for_world() {
    let early = trial_of(1);
    let late = trial_of(4);
    assert_eq!(early.played.worlds.len(), 16);
    assert_eq!(late.played.worlds.len(), 16);
    for world in 1..=16u32 {
        let (e, l) = (
            &early.played.worlds[world as usize - 1],
            &late.played.worlds[world as usize - 1],
        );
        // The same worlds: the same assumptions.
        assert_eq!(e.assumptions, l.assumptions, "world {world}");
        assert!(e.failed.is_none() && l.failed.is_none(), "world {world}");
        assert!(
            e.refused.is_empty() && l.refused.is_empty(),
            "world {world}"
        );
        assert!(approves(&world_commands_on(e, 1, ALLIED)), "world {world}");
        assert!(approves(&world_commands_on(e, 9, ALLIED)), "world {world}");
        assert!(approves(&world_commands_on(l, 4, ALLIED)), "world {world}");
        assert!(approves(&world_commands_on(l, 9, ALLIED)), "world {world}");
        assert_eq!(
            world_orders_on(e, 1, ALLIED),
            vec![Given::Landing(hex(4, 0), vec![1])],
            "world {world}"
        );
        assert_eq!(
            world_orders_on(l, 4, ALLIED),
            vec![Given::Landing(hex(4, 0), vec![1])],
            "world {world}"
        );
        // Ashore on day 11 in both, and the fit men by the side's own view.
        for (course, stepped, fit) in [("early", &early, 1000), ("late", &late, 930)] {
            assert_ne!(
                unit_1(&stepped.snapshot(world, 10).truth).hex,
                Some(hex(4, 0)),
                "{course}, world {world}, day 10"
            );
            let day = stepped.snapshot(world, 11);
            assert_eq!(
                unit_1(&day.truth).men_ashore,
                1000,
                "{course}, world {world}"
            );
            assert_eq!(day.fit[&1], fit, "{course}, world {world}");
        }
    }
}
