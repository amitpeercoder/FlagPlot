//! Ask, then run, then compare (decision 0100, 7).
//!
//! The main test of step 3's stage 0. For every scenario in the repository
//! and every day of it, each choice the engine makes for a side's own unit is
//! set beside the answer the side's view gives to the same question:
//!
//! - asked before the day runs (the moment `DayBegins`, before the day's
//!   orders are taken in), and
//! - asked at the moment just before the step of the day in which the engine
//!   made it (the moment the trace records).
//!
//! The day is run twice from the same start: once with the trace on, to learn
//! which questions the engine answered and at which moment, and once more
//! with the same orders, stopping at each moment to ask exactly those
//! questions from each side's view. A day is deterministic, so the second run
//! is the first (the test asserts the daily hashes are equal).
//!
//! What must hold:
//!
//! 1. Asked at the moment before its step, the view gives the engine's
//!    answer, except where something the choice reads changes inside that
//!    same step before the choice is made. Those cases are named below, one
//!    for each kind of question, and no other kind may differ. Each traced
//!    question carries the inputs that exist only inside its step (the
//!    fatigue after the day's update, the ships a fight began with, whether a
//!    unit marched or its hex was contested), so the cases are few.
//! 2. Asked before the day, an answer that differs from the engine's is
//!    explained by something that changed inside the day before the choice
//!    was made: the test checks that each such answer agrees at the moment
//!    of its step, or is one of the named in-step cases. The counts are
//!    printed with `--nocapture`.
//! 3. Asking changes nothing: a day run while its questions are asked gives
//!    the hash of the same day run without.
//!
//! The named in-step cases (`IN_STEP`), each a choice made in the middle of a
//! step after the step itself has changed what it reads. Every one was
//! found in the scenarios, and no other kind differs:
//!
//! - `BreakOff`: the force's ships are hit in the rounds of the fight before
//!   it breaks off (Savo Island, the night actions, the full slice, the
//!   soak).
//! - `Stragglers`: the straggler pass runs again after each fight in the
//!   movement step, on the hits of that fight.
//! - `HeadHome`, `Reroute` and `Halt`: a force the straggler pass detached
//!   in the movement step (stragglers sent home) is sent home, given its
//!   route and moved in the same step; it did not exist at the moment before
//!   it, so the view has no answer.
//! - `UnitDay`: a unit's men lost in the day's fight ashore, earlier in the
//!   same step, before its day is worked out (scenarios 23 and 24: the
//!   engine finds no men where the view, before the fight, found a unit at
//!   ready).
//!
//! No number in this test depends on a figure, so it runs on the live
//! figures; the frozen copies of `data/scenarios/step2-frozen` run on the
//! frozen figures, as their README says.

mod common;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use sim_core::choices::{Answer, Moment, Question, TracedChoice};
use sim_core::World;
use sim_data::Scenario;

/// The kinds of question whose answer may differ, asked at the moment before
/// the step, because the step changes what they read before the choice.
const IN_STEP: &[&str] = &[
    "BreakOff",
    "Stragglers",
    "HeadHome",
    "Reroute",
    "Halt",
    "UnitDay",
];

/// The name of a question's kind: its variant's name, from its `Debug` form.
fn kind(question: &Question) -> String {
    let text = format!("{question:?}");
    // `Air(Strike { group: .. })` -> `Strike`.
    let inner = text
        .split_once('(')
        .map(|(_, rest)| rest)
        .unwrap_or(&text)
        .to_string();
    inner
        .split(|c: char| !c.is_alphanumeric())
        .next()
        .unwrap_or("")
        .to_string()
}

#[derive(Default, Clone, Copy)]
struct Tally {
    made: u32,
    agree_before_day: u32,
    agree_at_moment_only: u32,
    differ_in_step: u32,
}

/// Every scenario in the repository: `data/scenarios/*.toml` on the live
/// figures and `data/scenarios/step2-frozen/*.toml` on the frozen ones.
/// `data/scenarios/not-run/` is not walked (decision 0072).
fn scenarios() -> Vec<(PathBuf, bool)> {
    let mut found = Vec::new();
    for (dir, frozen) in [("scenarios", false), ("scenarios/step2-frozen", true)] {
        let mut paths: Vec<PathBuf> = std::fs::read_dir(common::data_dir().join(dir))
            .expect("the scenario folder reads")
            .map(|entry| entry.expect("an entry").path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "toml"))
            .collect();
        paths.sort();
        found.extend(paths.into_iter().map(|path| (path, frozen)));
    }
    found
}

fn load(path: &Path, frozen: bool) -> Scenario {
    let scenario = if frozen {
        common::load(path)
    } else {
        common::load_live(path)
    };
    scenario.unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

/// The answer the side's view gives to `entry`'s question in `world`.
fn ask(world: &World, entry: &TracedChoice) -> Option<Answer> {
    world.side_view(entry.side).ask(&entry.question)
}

/// Runs `scenario` to its end, comparing every choice of every day, and adds
/// to `tallies`. Returns the failures, each as a line.
fn compare(
    name: &str,
    mut scenario: Scenario,
    tallies: &mut BTreeMap<String, Tally>,
) -> Vec<String> {
    let mut failures = Vec::new();
    let script = scenario.scripted_orders().clone();
    for day in 1..=scenario.days {
        for (side, request) in script.get(&day).cloned().unwrap_or_default() {
            scenario.world.submit_orders(side, vec![request]);
        }
        let start = scenario.world.clone();

        // First run: what the engine chose, and when.
        let mut traced = start.clone();
        traced.trace_choices(true);
        let first = traced.resolve_day().expect("the day resolves");
        let trace = traced.take_choice_trace();

        // Second run: the same day, asking the traced questions of each moment
        // as the day passes it, and every one of them before the day.
        let mut asked_at: Vec<Option<Answer>> = vec![None; trace.len()];
        let mut asked_before: Vec<Option<Answer>> = vec![None; trace.len()];
        let second = scenario
            .world
            .resolve_day_watched(&mut |world, moment| {
                if moment == (Moment::DayBegins { day }) {
                    for (i, entry) in trace.iter().enumerate() {
                        asked_before[i] = ask(world, entry);
                    }
                }
                for (i, entry) in trace.iter().enumerate() {
                    if entry.moment == moment {
                        asked_at[i] = ask(world, entry);
                    }
                }
            })
            .expect("the day resolves");
        assert_eq!(
            first.state_hash, second.state_hash,
            "{name} day {day}: asking changed the day"
        );

        for (i, entry) in trace.iter().enumerate() {
            let kind = kind(&entry.question);
            let tally = tallies.entry(kind.clone()).or_default();
            tally.made += 1;
            let engine = Some(entry.answer.clone());
            if asked_before[i] == engine {
                tally.agree_before_day += 1;
            } else if asked_at[i] == engine {
                tally.agree_at_moment_only += 1;
            } else if IN_STEP.contains(&kind.as_str()) {
                tally.differ_in_step += 1;
            } else {
                failures.push(format!(
                    "{name} day {day} {:?} {:?}: the engine chose {:?}; the view answered {:?} at {:?} and {:?} before the day",
                    entry.side, entry.question, entry.answer, asked_at[i], entry.moment, asked_before[i]
                ));
            }
        }
    }
    failures
}

#[test]
fn every_choice_of_every_scenario_is_the_answer_its_side_is_given() {
    let mut tallies: BTreeMap<String, Tally> = BTreeMap::new();
    let mut failures = Vec::new();
    let mut count = 0;
    for (path, frozen) in scenarios() {
        let name = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        let name = if frozen {
            format!("step2-frozen/{name}")
        } else {
            name
        };
        failures.extend(compare(&name, load(&path, frozen), &mut tallies));
        count += 1;
    }
    println!("{count} scenarios");
    println!(
        "{:<20} {:>8} {:>18} {:>22} {:>16}",
        "question", "made", "agree before day", "agree at step only", "differ in step"
    );
    for (kind, tally) in &tallies {
        println!(
            "{kind:<20} {:>8} {:>18} {:>22} {:>16}",
            tally.made, tally.agree_before_day, tally.agree_at_moment_only, tally.differ_in_step
        );
    }
    assert!(count >= 50, "the scenarios were found");
    // Every kind of question is made somewhere in the scenarios.
    for kind in [
        "Strike",
        "AircraftAbleToFly",
        "BaseStrike",
        "FighterSplit",
        "Escort",
        "Landing",
        "Rest",
        "Alert",
        "Ready",
        "BreakOff",
        "Stragglers",
        "HeadHome",
        "Reroute",
        "Halt",
        "SteamOn",
        "BackToDefend",
        "UnitDay",
    ] {
        assert!(
            tallies.get(kind).is_some_and(|tally| tally.made > 0),
            "no scenario makes the choice {kind}"
        );
    }
    assert!(
        failures.is_empty(),
        "{} differences:\n{}",
        failures.len(),
        failures.join("\n")
    );
}
