//! Integration tests of `sim-cli events --report` and `--strike`, and of the
//! narrowed `--why` of a launch (decision 0138, items 1, 2 and 7): who held a
//! report when and by which road, and why a group did or did not launch.
//!
//! The scenarios are 62, 63 and 64 (their hand calculations are in
//! `tests/scenario_62_reports_between_commands.rs`, `scenario_63_...` and
//! `scenario_64_...`: who held what when), on the frozen figures as those
//! tests run, and the Eastern Solomons from intents with command levels on
//! and the live report figures, made in the test and not kept as a file.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use sim_core::choices::{Moment, PulsePhase};
use sim_core::events::EventKind;
use sim_core::ids::AirGroupId;
use sim_core::orders::AirMission;
use sim_core::state::Side;
use sim_core::view::strike_review::ContactFate;
use sim_data::{LoadOptions, Scenario};
use sim_staff::{as_staffs, staffs};

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
const FROZEN: &str = sim_data::FROZEN_REFERENCE_DIR;
const S62: &str = "data/scenarios/62_reports_between_commands.toml";
const S63: &str = "data/scenarios/63_a_superiors_knowledge.toml";
const S64: &str = "data/scenarios/64_commanders_and_initiative.toml";

fn binary_at(args: &[&str], frozen: bool) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_sim-cli"));
    command.args(args);
    if frozen {
        command.args(["--reference", FROZEN]);
    }
    command
        .current_dir(ROOT)
        .output()
        .expect("the binary starts")
}

fn stdout_of(args: &[&str]) -> String {
    let output = binary_at(args, true);
    assert!(
        output.status.success(),
        "{args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("utf-8")
}

fn error_of(args: &[&str]) -> String {
    let output = binary_at(args, true);
    assert_eq!(output.status.code(), Some(1), "{args:?}");
    assert!(output.stdout.is_empty(), "{args:?} printed to stdout");
    String::from_utf8_lossy(&output.stderr).to_string()
}

/// The lines of `run --events` as (event number, text).
fn run_events(scenario: &str) -> Vec<(u32, String)> {
    stdout_of(&["run", scenario, "--events"])
        .lines()
        .filter_map(|line| {
            let number = line.strip_prefix('#')?.split(' ').next()?.parse().ok()?;
            Some((number, line.to_string()))
        })
        .collect()
}

/// The number of the first event whose text contains `needle`, and the
/// number its text names after `of #`, if any.
fn event_with(events: &[(u32, String)], needle: &str) -> u32 {
    events
        .iter()
        .find(|(_, text)| text.contains(needle))
        .unwrap_or_else(|| panic!("no event says {needle:?}"))
        .0
}

fn lines_of(output: &str) -> Vec<&str> {
    output.lines().skip(1).collect()
}

// ------------------------------------------------------------------ Item 1

#[test]
fn scenario_62_the_report_reached_the_area_command_and_never_the_others() {
    let events = run_events(S62);
    // Task force 1's "nothing there" and its passing (hand calculation: up
    // on time, sideways never, down never).
    let report = event_with(&events, "found nothing");
    let sent = event_with(&events, "task_force:1 sends the report of");
    let down = event_with(&events, "the area command sends the report of");
    let out = stdout_of(&["events", S62, "--report", &report.to_string()]);
    assert!(out.lines().next().unwrap().contains("seed 1942"), "{out}");
    assert_eq!(
        lines_of(&out),
        vec![
            format!("the area command first held report #{report} in the morning of day 1: sent up by task_force:1 (event #{sent}), on time."),
            format!("task_force:1 first held report #{report} in the morning of day 1: its own (it made the report)."),
            format!("task_force:2 never held report #{report} (sent sideways by task_force:1, event #{sent}: lost; sent down by the area command, event #{down}: lost)."),
            format!("base:1 never held report #{report} (sent sideways by task_force:1, event #{sent}: lost; sent down by the area command, event #{down}: lost)."),
        ]
    );
    // One command, and the same with --json.
    let one = stdout_of(&[
        "events",
        S62,
        "--report",
        &format!("#{report}"),
        "--command",
        "base:1",
    ]);
    assert_eq!(lines_of(&one), vec![lines_of(&out)[3]]);
    let json = stdout_of(&["events", S62, "--report", &report.to_string(), "--json"]);
    let value: serde_json::Value = serde_json::from_str(json.trim()).expect("one JSON value");
    assert_eq!(value["report"], report);
    assert_eq!(value["commands"].as_array().unwrap().len(), 4);
    assert_eq!(value["commands"][2]["held"], false);
    assert_eq!(value["commands"][2]["road"], "never");
    assert_eq!(value["commands"][0]["road"], "up");
    assert_eq!(value["commands"][1]["road"], "own");
}

#[test]
fn a_report_that_no_command_passed_is_refused_and_so_are_bad_commands() {
    // The contact the Allied side starts with is held by every command from
    // the start: nothing passed it.
    let err = error_of(&["events", S62, "--report", "1"]);
    assert!(
        err.contains("event #1 is not a report passed between commands"),
        "{err}"
    );
    // So is an event that is not a report at all, and one that is not in the
    // log.
    let nothing = event_with(&run_events(S62), "found nothing") + 1;
    let err = error_of(&["events", S62, "--report", &nothing.to_string()]);
    assert!(
        err.contains("is not a report passed between commands"),
        "{err}"
    );
    let err = error_of(&["events", S62, "--report", "99999"]);
    assert!(err.contains("no event #99999"), "{err}");
    let report = event_with(&run_events(S62), "found nothing");
    let err = error_of(&[
        "events",
        S62,
        "--report",
        &report.to_string(),
        "--command",
        "base:9",
    ]);
    assert!(
        err.contains("no base:9 of the Allied side in this run"),
        "{err}"
    );
    let err = error_of(&[
        "events",
        S62,
        "--report",
        &report.to_string(),
        "--command",
        "ship:1",
    ]);
    assert!(err.contains("not a command"), "{err}");
}

#[test]
fn scenario_63_the_base_heard_it_by_the_first_of_two_roads_that_arrived_together() {
    let events = run_events(S63);
    let report = event_with(&events, "found nothing");
    let sent = event_with(&events, "task_force:1 sends the report of");
    let down = event_with(&events, "the area command sends the report of");
    assert!(sent < down);
    let out = stdout_of(&["events", S63, "--report", &report.to_string()]);
    let lines = lines_of(&out);
    // Base 1 was sent it sideways and, by the area command, down, both on
    // time in the morning: the first by the event's number is the road.
    assert!(
        lines.contains(&format!(
            "base:1 first held report #{report} in the morning of day 1: sent sideways by task_force:1 (event #{sent}), on time."
        ).as_str()),
        "{out}"
    );
    assert!(
        lines.iter().any(|l| l.starts_with(&format!(
            "the area command first held report #{report} in the morning of day 1: sent up by task_force:1 (event #{sent}), on time"
        ))),
        "{out}"
    );
    assert!(lines.iter().all(|l| !l.starts_with("note:")), "{out}");
    // The down passing is a road to a command the sideways one did not reach:
    // with the report figures of the scenario, find one that is told down.
    let _ = down;
}

#[test]
fn a_scenario_with_command_levels_off_says_the_side_held_it_when_it_was_made() {
    // The frozen Eastern Solomons has no command levels: any report event.
    const EASTERN: &str = "data/scenarios/step2-frozen/05_eastern_solomons.toml";
    let events = run_events(EASTERN);
    let report = event_with(&events, "Allied report:");
    let out = stdout_of(&["events", EASTERN, "--report", &report.to_string()]);
    assert_eq!(
        lines_of(&out),
        vec!["the Allied side: command levels are off: the side held it when it was made."]
    );
}

#[test]
fn a_game_with_command_levels_and_no_report_figures_says_reports_do_not_travel() {
    // Scenario 61 is scenario 58 with command levels on and no report
    // figures: every command held the report when it was made.
    const S61: &str = "data/scenarios/61_commands_and_their_pictures.toml";
    let events = run_events(S61);
    let report = event_with(&events, "found nothing");
    let out = stdout_of(&["events", S61, "--report", &report.to_string()]);
    assert_eq!(
        lines_of(&out),
        vec!["the Allied side: reports do not travel in this game: every command of the side held it when it was made."]
    );
}

// ------------------------------------------------------------------ Item 2

#[test]
fn scenario_62_a_group_that_launched_and_a_group_that_launched_nothing() {
    let events = run_events(S62);
    let launch = event_with(&events, "air_group:3 launched");
    let report = event_with(&events, "found nothing");
    // Group 3 of task force 1 launched in the morning of day 1 at the
    // contact the side held at the start.
    let out = stdout_of(&[
        "events",
        S62,
        "--strike",
        "air_group:3",
        "--day",
        "1",
        "--pulse",
        "morning",
    ]);
    assert!(out.contains("belongs to task_force:1"), "{out}");
    assert!(
        out.contains("the rules gave: launch 10 aircraft at contact 1 (carrier) at (12,0), 240 nm"),
        "{out}"
    );
    assert!(
        out.contains(&format!(
            "the log has: #{launch} launched 10 aircraft at contact 1 at (12,0), 240 nm"
        )),
        "{out}"
    );
    assert!(out.contains("contact age limit 2 pulses"), "{out}");
    assert!(out.contains("contact 1 at (12,0): a carrier, 1 ship, 1 pulse old, rests on report #1, told: chosen: 10 aircraft sent at it"), "{out}");
    assert!(!out.contains("warning"), "{out}");

    // Group 6 of task force 2 launched nothing: it holds the contact, a
    // pulse old, against an age limit of 0. It was never told the report
    // that dropped the contact from task force 1's picture.
    let out = stdout_of(&[
        "events",
        S62,
        "--strike",
        "air_group:6",
        "--day",
        "1",
        "--pulse",
        "morning",
        "--report",
        &report.to_string(),
    ]);
    assert!(out.contains("belongs to task_force:2"), "{out}");
    assert!(out.contains("the rules gave: launch nothing"), "{out}");
    assert!(
        out.contains("the log has no launch of air_group:6 in this pulse"),
        "{out}"
    );
    assert!(out.contains("contact age limit 0 pulses"), "{out}");
    assert!(
        out.contains("too old: 1 pulse against the limit of 0"),
        "{out}"
    );
    assert!(
        out.contains(&format!(
            "report #{report}: not held by task_force:2 as a contact at that moment"
        )),
        "{out}"
    );
    assert!(
        out.contains(&format!(
            "task_force:2 never held report #{report} (sent sideways by task_force:1"
        )),
        "{out}"
    );
    assert!(!out.contains("warning"), "{out}");

    // A group that is not on the naval strike mission: its gate says so.
    let out = stdout_of(&[
        "events",
        S62,
        "--strike",
        "air_group:1",
        "--day",
        "1",
        "--pulse",
        "morning",
    ]);
    assert!(
        out.contains("the group cannot launch: the group is not on the naval strike mission"),
        "{out}"
    );
    assert!(out.contains("the rules gave: launch nothing"), "{out}");
    assert!(
        out.contains("too old: 1 pulse against the limit of 0"),
        "{out}"
    );

    // The same as JSON.
    let json = stdout_of(&[
        "events",
        S62,
        "--strike",
        "air_group:6",
        "--day",
        "1",
        "--pulse",
        "morning",
        "--json",
    ]);
    let value: serde_json::Value = serde_json::from_str(json.trim()).expect("one JSON value");
    assert_eq!(value["command"], "task_force:2");
    assert_eq!(value["agree"], true);
    assert_eq!(
        value["contacts"][0]["fate"],
        "too old: 1 pulse against the limit of 0"
    );
}

#[test]
fn the_strike_question_refuses_what_it_cannot_answer() {
    let err = error_of(&[
        "events",
        S62,
        "--strike",
        "air_group:3",
        "--day",
        "1",
        "--pulse",
        "night",
    ]);
    assert!(
        err.contains("no strike is decided in a night pulse"),
        "{err}"
    );
    let err = error_of(&[
        "events",
        S62,
        "--strike",
        "air_group:99",
        "--day",
        "1",
        "--pulse",
        "morning",
    ]);
    assert!(err.contains("no air_group:99 in this run"), "{err}");
    let err = error_of(&[
        "events",
        S62,
        "--strike",
        "air_group:3",
        "--pulse",
        "morning",
    ]);
    assert!(err.contains("--strike needs --day"), "{err}");
    let err = error_of(&[
        "events", S62, "--strike", "ship:3", "--day", "1", "--pulse", "morning",
    ]);
    assert!(err.contains("--strike takes an air group"), "{err}");
}

#[test]
fn scenarios_63_and_64_have_reviews_that_agree_with_their_logs() {
    for scenario in [S63, S64] {
        let out = stdout_of(&[
            "events",
            scenario,
            "--strike",
            "air_group:3",
            "--day",
            "1",
            "--pulse",
            "morning",
        ]);
        assert!(!out.contains("warning"), "{scenario}: {out}");
        assert!(out.contains("the rules gave:"), "{out}");
    }
}

// ------------------------------------------------------- The narrowed --why

/// The Eastern Solomons from intents with command levels on and the live
/// report figures, as a temporary file with absolute data paths, made here.
fn eastern_from_intents_with_levels() -> PathBuf {
    let root = Path::new(ROOT).canonicalize().expect("the root");
    let text =
        std::fs::read_to_string(root.join("data/scenarios/second-proof/05a_eastern_solomons.toml"))
            .expect("the scenario reads");
    let text = text.replace("\"../../", &format!("\"{}/data/", root.display()));
    let text = format!(
        "command_levels = true\nreports = \"{}/data/reference/reports.toml\"\n{text}",
        root.display()
    );
    let file = std::env::temp_dir().join(format!(
        "flag_plot_who_knew_{}_05a.toml",
        std::process::id()
    ));
    std::fs::write(&file, text).expect("the temporary scenario writes");
    file
}

#[test]
fn why_a_launch_follows_a_report_only_along_the_road_to_the_command_that_decided() {
    let file = eastern_from_intents_with_levels();
    let file_name = file.display().to_string();
    // Live figures: no --reference.
    let run = |args: &[&str]| {
        let output = binary_at(args, false);
        assert!(
            output.status.success(),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).expect("utf-8")
    };
    let events: Vec<(u32, String)> = run(&["run", &file_name, "--events", "--seed", "3"])
        .lines()
        .filter_map(|line| {
            let number = line.strip_prefix('#')?.split(' ').next()?.parse().ok()?;
            Some((number, line.to_string()))
        })
        .collect();
    // A launch whose report was passed between commands.
    let passed: Vec<u32> = events
        .iter()
        .filter_map(|(_, text)| text.split("sends the report of #").nth(1))
        .filter_map(|rest| rest.split(':').next()?.parse().ok())
        .collect();
    let (launch, report) = events
        .iter()
        .filter(|(_, text)| text.contains(" launched ") && text.contains(" aircraft at contact:"))
        .find_map(|(number, text)| {
            let cited: Vec<u32> = text
                .split("<-")
                .nth(1)?
                .split(", ")
                .filter_map(|c| c.trim().strip_prefix('#')?.trim().parse().ok())
                .collect();
            cited
                .into_iter()
                .find(|c| passed.contains(c))
                .map(|c| (*number, c))
        })
        .expect("a launch rests on a report that was passed");
    let narrowed = run(&[
        "events",
        &file_name,
        "--seed",
        "3",
        "--why",
        &launch.to_string(),
    ]);
    let whole = run(&[
        "events",
        &file_name,
        "--seed",
        "3",
        "--why",
        &report.to_string(),
    ]);
    let passings = |text: &str| {
        text.lines()
            .filter(|l| l.trim_start().starts_with("-> #") && l.contains("sends the report of"))
            .count()
    };
    // Straight on the report: every passing is listed, as before.
    assert!(passings(&whole) >= 2, "{whole}");
    // Under the launch: the road to the command that decided, in words and
    // at most the passings on that road.
    assert_eq!(
        narrowed.lines().filter(|l| l.contains(" decided]")).count(),
        1,
        "{narrowed}"
    );
    assert!(passings(&narrowed) <= 2, "{narrowed}");
    assert!(passings(&narrowed) < passings(&whole), "{narrowed}");
    // The words are item 1's: the same line as `--report` gives for that
    // command.
    let line = narrowed
        .lines()
        .find(|l| l.contains(" decided]"))
        .expect("the road line")
        .trim_start()
        .trim_start_matches("-> ")
        .to_string();
    let command = line.split(" first held").next().expect("a command");
    let asked = run(&[
        "events",
        &file_name,
        "--seed",
        "3",
        "--report",
        &report.to_string(),
        "--command",
        if command == "the area command" {
            "area"
        } else {
            command
        },
    ]);
    assert!(
        line.starts_with(asked.lines().nth(1).expect("an answer")),
        "{line}\n{asked}"
    );
    let _ = std::fs::remove_file(&file);
}

// ---------------------------------------------------------- Consistency

/// What a run of a game from intents gives: for each naval strike group at
/// each strike decision, the review's launch and its chosen contact, and the
/// log's launches.
#[derive(Default)]
struct Checked {
    decisions: usize,
    launches: usize,
    unlaunched: usize,
    /// The fates of the contacts met, by name.
    fates: BTreeMap<&'static str, usize>,
}

/// What a review said: the launch (contact and aircraft) and the contact
/// its walk chose.
type Reviewed = (Option<(u32, u32)>, Option<u32>);

/// A strike decision: the day, the pulse's code and the group.
type Decision = (u32, u64, AirGroupId);

fn fate_name(fate: ContactFate) -> &'static str {
    match fate {
        ContactFate::Chosen { .. } => "chosen",
        ContactFate::TooOld { .. } => "too old",
        ContactFate::OutOfReach { .. } => "out of reach",
        ContactFate::KindNotOnList => "kind not on the list",
        ContactFate::CameFirst { .. } => "came first",
        ContactFate::Served { .. } => "served",
        ContactFate::GroupStopped(_) => "group stopped",
    }
}

fn check_reviews_against_log(mut scenario: Scenario, label: &str) -> Checked {
    let held = staffs();
    let mut reviews: BTreeMap<Decision, Reviewed> = BTreeMap::new();
    let mut fates: BTreeMap<&'static str, usize> = BTreeMap::new();
    loop {
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |world, moment| {
                let Moment::Phase {
                    day,
                    pulse,
                    phase: PulsePhase::StrikeDecisions,
                } = moment
                else {
                    return;
                };
                let key = world.umpire_key();
                for side in Side::ALL {
                    let view = world.side_view(side);
                    let groups: Vec<_> = view
                        .air_groups()
                        .filter(|g| g.mission == AirMission::NavalStrike)
                        .map(|g| g.id)
                        .collect();
                    for group in groups {
                        let review = view
                            .strike_review(group, pulse)
                            .expect("a group of the side has a review");
                        let launches = review.launches().map(|strike| {
                            (strike.choice.contact.umpire_raw(&key), strike.aircraft)
                        });
                        let chosen = review.chosen_contact().map(|c| c.number);
                        for contact in &review.contacts {
                            *fates.entry(fate_name(contact.fate)).or_insert(0) += 1;
                        }
                        reviews.insert((day, pulse.code(), group), (launches, chosen));
                    }
                }
            })
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
    }
    let key = scenario.world.umpire_key();
    let mut logged: BTreeMap<Decision, (u32, u32)> = BTreeMap::new();
    for event in scenario.world.events().iter() {
        if let EventKind::StrikeLaunched {
            air_group,
            contact,
            aircraft,
            ..
        } = &event.kind
        {
            let pulse = event.pulse.expect("a launch has a pulse");
            let previous = logged.insert(
                (event.day, pulse.code(), *air_group),
                (contact.umpire_raw(&key), *aircraft),
            );
            assert!(
                previous.is_none(),
                "{label}: two launches of a group in a pulse"
            );
        }
    }
    let mut checked = Checked {
        fates,
        ..Checked::default()
    };
    for (at, (launches, chosen)) in &reviews {
        checked.decisions += 1;
        assert_eq!(
            *launches,
            logged.get(at).copied(),
            "{label}: the review and the log differ for group {} at day {} pulse {}",
            at.2,
            at.0,
            at.1
        );
        match launches {
            Some((contact, _)) => {
                checked.launches += 1;
                assert_eq!(
                    chosen,
                    &Some(*contact),
                    "{label}: the walk took another contact"
                );
            }
            None => {
                checked.unlaunched += 1;
                assert_eq!(
                    chosen, &None,
                    "{label}: chosen for a group that did not launch"
                );
            }
        }
    }
    // Every launch of the log was a decision the review covered.
    for at in logged.keys() {
        assert!(
            reviews.contains_key(at),
            "{label}: a launch of no review {at:?}"
        );
    }
    checked
}

fn frozen_scenario(file: &str, seed: u64) -> Scenario {
    let root = Path::new(ROOT);
    Scenario::load_with(&root.join(file), &LoadOptions::frozen_step1(root))
        .expect("the scenario loads")
        .with_seed(seed)
        .expect("the seed is set")
}

#[test]
fn the_review_agrees_with_the_log_on_scenarios_62_63_and_64_and_the_eastern_solomons() {
    let mut total = Checked::default();
    for file in [S62, S63, S64] {
        for seed in 1..=5 {
            let c = check_reviews_against_log(
                frozen_scenario(file, seed),
                &format!("{file} seed {seed}"),
            );
            total.decisions += c.decisions;
            total.launches += c.launches;
            total.unlaunched += c.unlaunched;
            for (name, n) in c.fates {
                *total.fates.entry(name).or_insert(0) += n;
            }
        }
    }
    let root = Path::new(ROOT).canonicalize().expect("the root");
    let path = root.join("data/scenarios/second-proof/05a_eastern_solomons.toml");
    let text = std::fs::read_to_string(&path).expect("the scenario reads");
    let text = format!(
        "command_levels = true\nreports = \"{}/data/reference/reports.toml\"\n{text}",
        root.display()
    );
    for seed in 1..=5 {
        let scenario = Scenario::from_text(&text, &path)
            .expect("the scenario loads")
            .with_seed(seed)
            .expect("the seed is set");
        let c = check_reviews_against_log(scenario, &format!("Eastern Solomons seed {seed}"));
        total.decisions += c.decisions;
        total.launches += c.launches;
        total.unlaunched += c.unlaunched;
        for (name, n) in c.fates {
            *total.fates.entry(name).or_insert(0) += n;
        }
    }
    // Every fate but a kind not on the list (the unit tests of the verdict
    // cover it: no group here has a target list) was met.
    for name in [
        "chosen",
        "too old",
        "out of reach",
        "came first",
        "served",
        "group stopped",
    ] {
        assert!(
            total.fates.contains_key(name),
            "{name} was never met: {:?}",
            total.fates
        );
    }
    assert!(
        total.decisions > 0 && total.launches > 0 && total.unlaunched > 0,
        "both kinds were met"
    );
}
