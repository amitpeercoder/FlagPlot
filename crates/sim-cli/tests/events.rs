//! Integration tests of `sim-cli events` (decision 0052, item 9).
//!
//! The listings are compared with `run --events`, line by line, and with a
//! world the test runs itself through `sim-data`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::process::{Command, Output};

use sim_core::ids::EntityRef;
use sim_data::{LoadOptions, Scenario};

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
const EASTERN: &str = "data/scenarios/05_eastern_solomons.toml";
/// These tests name events of a run (that a ship sank, that a group lost
/// aircraft), which the figures decide, so they run on the frozen figures of
/// step 1 and not on the ones step 2 tunes (docs/step2-spec.md, "Tests while
/// figures move").
const FROZEN: &str = sim_data::FROZEN_REFERENCE_DIR;

fn binary(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_sim-cli"))
        .args(args)
        .args(["--reference", FROZEN])
        .current_dir(ROOT)
        .output()
        .expect("the binary starts")
}

fn stdout_of(args: &[&str]) -> String {
    let output = binary(args);
    assert!(
        output.status.success(),
        "{args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("utf-8")
}

/// The event lines of `run --events`: those that begin with `#N `.
fn run_event_lines(scenario: &str) -> BTreeMap<u32, String> {
    stdout_of(&["run", scenario, "--events"])
        .lines()
        .filter_map(|line| {
            let number = line.strip_prefix('#')?.split(' ').next()?.parse().ok()?;
            Some((number, line.to_string()))
        })
        .collect()
}

/// A listing's lines without the header and the count: (via force, event
/// number, the event's own text).
fn listing(output: &str) -> Vec<(Option<String>, u32, String)> {
    let lines: Vec<&str> = output.lines().collect();
    assert!(lines.len() >= 2, "header and count expected: {output}");
    lines[1..lines.len() - 1]
        .iter()
        .map(|line| {
            let (via, rest) = match line.strip_prefix("via ") {
                Some(rest) => {
                    let (force, rest) = rest.split_once("  ").expect("two spaces after via");
                    (Some(force.to_string()), rest)
                }
                None => (None, *line),
            };
            let number = rest
                .strip_prefix('#')
                .and_then(|r| r.split(' ').next())
                .and_then(|n| n.parse().ok())
                .expect("an event line");
            (via, number, rest.to_string())
        })
        .collect()
}

fn world(scenario: &str) -> Scenario {
    let mut scenario = Scenario::load_with(
        &Path::new(ROOT).join(scenario),
        &LoadOptions::frozen_step1(Path::new(ROOT)),
    )
    .expect("loads");
    scenario.run_to_end().expect("runs");
    scenario
}

#[test]
fn ryujo_lists_hits_sinking_and_its_forces_movement_as_run_prints_them() {
    let all = run_event_lines(EASTERN);
    let out = stdout_of(&["events", EASTERN, "--unit", "ship:33"]);
    let first = out.lines().next().expect("a header");
    assert!(
        first.contains("seed 1942") && first.contains("ship:33"),
        "{first}"
    );
    let lines = listing(&out);
    let count_line = out.lines().last().expect("a count");
    assert_eq!(count_line, format!("{} events", lines.len()));
    for (_, number, text) in &lines {
        assert_eq!(
            &all[number], text,
            "event #{number} differs from run --events"
        );
    }
    let has = |needle: &str| lines.iter().any(|(_, _, t)| t.contains(needle));
    assert!(has("ship:33 took"), "hits on flotation");
    assert!(has("ship:33 of task_force:4 sank"), "its sinking");
    assert!(has("bombs at ship:33"), "the attacks");
    assert!(
        lines
            .iter()
            .any(|(via, _, t)| via.as_deref() == Some("task_force:4") && t.contains("steamed")),
        "its force's movement, marked via"
    );
    // The events are in order.
    assert!(lines.windows(2).all(|w| w[0].1 < w[1].1));
    // Nothing after the sinking is marked via its force.
    let sunk = lines
        .iter()
        .find(|(_, _, t)| t.contains("ship:33 of task_force:4 sank"))
        .expect("sinking")
        .1;
    assert!(lines.iter().all(|(via, n, _)| via.is_none() || *n < sunk));
}

#[test]
fn an_air_group_lists_its_launch_attacks_and_losses() {
    let out = stdout_of(&["events", EASTERN, "--unit", "air_group:12"]);
    let lines = listing(&out);
    assert!(lines.iter().all(|(via, _, _)| via.is_none()));
    let has = |needle: &str| lines.iter().any(|(_, _, t)| t.contains(needle));
    assert!(has("air_group:12 launched"), "launch");
    assert!(has("air_group:12 lost"), "losses");
    assert!(has("patrol at the bombers of air_group:12"), "fights");
    // An air group aboard a ship does not take the ship's events.
    assert!(!has("burned"));
}

#[test]
fn day_keeps_only_that_day() {
    let all = listing(&stdout_of(&["events", EASTERN, "--unit", "task_force:4"]));
    let day1 = stdout_of(&["events", EASTERN, "--unit", "task_force:4", "--day", "1"]);
    assert!(day1.lines().next().unwrap().contains("on day 1"));
    let day1 = listing(&day1);
    assert!(!day1.is_empty());
    assert!(day1.iter().all(|(_, _, t)| t.contains(" D1 ")));
    let expected: Vec<_> = all.iter().filter(|(_, _, t)| t.contains(" D1 ")).collect();
    assert_eq!(day1.len(), expected.len());
    let none = stdout_of(&["events", EASTERN, "--unit", "task_force:4", "--day", "40"]);
    assert!(none.contains("no event on that day names it"), "{none}");
    assert!(none.ends_with("0 events\n"));
}

#[test]
fn why_the_sinking_ends_in_orders_and_includes_launch_and_sighting() {
    let all = run_event_lines(EASTERN);
    let sunk = all
        .iter()
        .find(|(_, t)| t.contains("ship:33 of task_force:4 sank"))
        .expect("the sinking")
        .0;
    let out = stdout_of(&["events", EASTERN, "--why", &sunk.to_string()]);
    let lines: Vec<&str> = out.lines().collect();
    assert!(lines[0].contains(&format!("why event #{sunk}")));
    // The root is the event, in the text `run --events` prints.
    assert_eq!(lines[1], all[sunk]);
    // Every event line of the tree is that event's text, with indentation.
    for line in &lines[2..] {
        let body = line
            .trim_start()
            .strip_prefix("<- ")
            .unwrap_or(line.trim_start());
        if let Some(rest) = body.strip_prefix('#') {
            if let Ok(number) = rest.split(' ').next().unwrap().parse::<u32>() {
                if all.contains_key(&number) {
                    assert_eq!(body, all[&number]);
                }
            }
        }
    }
    assert!(out.contains("launched"), "the strike's launch");
    assert!(out.contains("report:"), "the sighting report");
    assert!(out.contains("searched for"), "the search");
    assert!(out.contains("breakdown:"), "a breakdown");
    // The deepest lines are orders.
    let deepest = lines
        .iter()
        .map(|l| l.len() - l.trim_start().len())
        .max()
        .unwrap();
    let last = lines
        .iter()
        .rev()
        .find(|l| {
            l.len() - l.trim_start().len() == deepest && !l.trim_start().starts_with("breakdown")
        })
        .unwrap();
    assert!(last.trim_start().starts_with("<- order:"), "{last}");
    // No event is printed twice in full.
    let mut seen = BTreeSet::new();
    for line in &lines[1..] {
        let body = line.trim_start().trim_start_matches("<- ");
        if let Some(number) = body
            .strip_prefix('#')
            .and_then(|r| r.split(' ').next())
            .and_then(|n| n.parse::<u32>().ok())
        {
            if !body.contains("(shown above)") {
                assert!(seen.insert(number), "#{number} printed twice");
            }
        }
    }
}

#[test]
fn an_event_reached_twice_is_referred_to_by_number() {
    // The attack on a ship by two groups share causes somewhere in the run:
    // find an event whose tree has a repeat.
    let all = run_event_lines(EASTERN);
    let repeated = all.keys().any(|n| {
        stdout_of(&["events", EASTERN, "--why", &n.to_string()]).contains("(shown above)")
    });
    assert!(repeated, "some tree reaches an event twice");
}

#[test]
fn the_unit_and_event_forms_parse_and_errors_are_clear() {
    let a = stdout_of(&["events", EASTERN, "--why", "#133"]);
    let b = stdout_of(&["events", EASTERN, "--why", "event:133"]);
    assert_eq!(a, b);
    let c = stdout_of(&["events", EASTERN, "--unit", "japanese:contact:1"]);
    assert!(listing(&c)
        .iter()
        .all(|(_, _, t)| t.contains("Japanese report") || t.contains("contact:1")));

    for (args, message) in [
        (vec!["--unit", "ship:9999"], "no ship:9999"),
        (vec!["--unit", "frigate:3"], "not a unit handle"),
        (vec!["--unit", "ship"], "not a unit handle"),
        (vec!["--unit", "ship:x"], "not a unit handle"),
        (vec!["--unit", "allied:ship:3"], "only before a contact"),
        (vec!["--why", "99999"], "no event #99999"),
        (vec!["--why", "abc"], "not an event number"),
        (vec!["--unit", "ship:33", "--why", "3"], "exactly one"),
        (vec![], "exactly one"),
        (vec!["--why", "3", "--day", "1"], "--day goes with --unit"),
    ] {
        let mut full = vec!["events", EASTERN];
        full.extend(args.iter());
        let output = binary(&full);
        assert_eq!(output.status.code(), Some(1), "{full:?}");
        assert!(output.stdout.is_empty(), "{full:?} printed to stdout");
        let err = String::from_utf8_lossy(&output.stderr);
        assert!(
            err.starts_with("error: ") && err.contains(message),
            "{full:?}: {err}"
        );
    }
    let missing = binary(&["events", "data/scenarios/nope.toml", "--unit", "ship:1"]);
    assert_eq!(missing.status.code(), Some(1));
}

#[test]
fn a_unit_with_no_events_says_so_and_succeeds() {
    // A base nothing happened at in scenario 1. It was a base of scenario 5
    // until aircraft landed at its only base when Enterprise's deck went out
    // (decisions 7 to 10 of 0059): scenario 5 has no unit left that no event
    // names, so the test reads scenario 1.
    const TRANSIT: &str = "data/scenarios/01_transit.toml";
    let scenario = world(TRANSIT);
    let named: BTreeSet<EntityRef> = scenario
        .world
        .events()
        .iter()
        .flat_map(|e| e.kind.entities())
        .collect();
    let quiet = scenario
        .world
        .umpire()
        .bases
        .keys()
        .find(|b| !named.contains(&EntityRef::Base(**b)))
        .expect("a base no event names");
    let out = stdout_of(&["events", TRANSIT, "--unit", &quiet.to_string()]);
    assert!(
        out.contains("is in this run, but no event names it"),
        "{out}"
    );
    assert!(out.ends_with("0 events\n"));
}

#[test]
fn every_event_with_entities_is_in_the_listing_of_each_unit_it_names() {
    for path in [
        "data/scenarios/01_transit.toml",
        "data/scenarios/04_savo_island.toml",
        EASTERN,
    ] {
        let scenario = world(path);
        let log: BTreeMap<u32, Vec<EntityRef>> = scenario
            .world
            .events()
            .iter()
            .map(|e| (e.id.raw(), e.kind.entities()))
            .collect();
        let all = run_event_lines(path);
        let units: BTreeSet<EntityRef> = log.values().flatten().copied().collect();
        let mut reached: BTreeSet<u32> = BTreeSet::new();
        for unit in units {
            let handle = unit.to_string();
            let out = stdout_of(&["events", path, "--unit", &handle]);
            for (via, number, text) in listing(&out) {
                assert_eq!(all[&number], text, "{path} {handle} #{number}");
                let names = &log[&number];
                match via {
                    None => assert!(
                        names.contains(&unit),
                        "{path} {handle}: #{number} does not name it"
                    ),
                    Some(force) => {
                        assert!(matches!(unit, EntityRef::Ship(_)), "via only for a ship");
                        assert!(
                            names.iter().any(|n| n.to_string() == force),
                            "{path} {handle}: #{number} does not name {force}"
                        );
                        assert!(!names.contains(&unit));
                    }
                }
                reached.insert(number);
            }
            // And the listing is complete: every event that names it is there.
            let listed: BTreeSet<u32> = listing(&out).iter().map(|(_, n, _)| *n).collect();
            for (number, names) in &log {
                if names.contains(&unit) {
                    assert!(listed.contains(number), "{path} {handle} misses #{number}");
                }
            }
        }
        for (number, names) in &log {
            if !names.is_empty() {
                assert!(
                    reached.contains(number),
                    "{path}: #{number} is in no listing"
                );
            }
        }
    }
}

#[test]
fn a_saved_game_gives_the_same_listing_as_its_scenario() {
    let dir = std::env::temp_dir().join(format!("flagplot-events-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("temp dir");
    let save = dir.join("eastern.save.json");
    let save = save.to_str().unwrap();
    stdout_of(&["run", EASTERN, "--save", save]);
    let from_scenario = stdout_of(&["events", EASTERN, "--unit", "ship:33"]);
    let from_save = stdout_of(&["events", save, "--unit", "ship:33"]);
    assert_eq!(from_scenario, from_save);
    let seeded = binary(&["events", save, "--unit", "ship:33", "--seed", "3"]);
    assert_eq!(seeded.status.code(), Some(1));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_output_is_the_same_on_every_run() {
    for args in [
        vec!["events", EASTERN, "--unit", "ship:33"],
        vec!["events", EASTERN, "--why", "133"],
    ] {
        assert_eq!(stdout_of(&args), stdout_of(&args));
    }
    // A different seed is a different run, and says so in its header.
    let other = stdout_of(&["events", EASTERN, "--unit", "ship:33", "--seed", "7"]);
    assert!(other.lines().next().unwrap().contains("seed 7"));
}
