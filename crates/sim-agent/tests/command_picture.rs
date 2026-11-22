//! A command's picture and held state in `command_state`, and refusals by
//! cause in `report` (decision 0138, items 3 and 4).

mod support;

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::state::Side;
use support::{load_text, scenario_text, schemas};

/// A session of the Allied seat with the doctrine's staff, the scenario's
/// scripted commands submitted as the commander's, played to the morning of
/// `day`.
fn session_at(name: &str, text: &str, day: u32) -> Session {
    let mut scenario = load_text(text, name, 1942);
    let commands = scenario.scripted_commands().clone();
    scenario.replace_commands(BTreeMap::new());
    let scenario_name = scenario.name.clone();
    let mut session = Session::new(
        scenario.world,
        &scenario_name,
        scenario.days,
        Seat::Player,
        Seat::Script(BTreeMap::new()),
    )
    .with_staffs(sim_staff::staffs())
    .with_commands(commands);
    while session.day() + 1 < day {
        let a = session
            .call(Side::Allied, "end_day", &json!({}))
            .expect("a day ends");
        assert_eq!(a["resolved"], true);
    }
    session
}

fn call(session: &mut Session, tool: &str) -> Value {
    let answer = session
        .call(Side::Allied, tool, &json!({}))
        .unwrap_or_else(|e| panic!("{tool}: {e}"));
    let kind = answer["kind"].as_str().expect("a kind");
    let errors = schemas().check(&format!("{kind}.schema.json"), &answer);
    assert!(errors.is_empty(), "{tool}: {errors:?}\n{answer}");
    answer
}

fn held<'a>(answer: &'a Value, key: &str, handle: &str) -> &'a Value {
    answer["forces_held"]
        .as_array()
        .expect("forces_held is a list")
        .iter()
        .find(|row| row[key]["handle"] == handle)
        .unwrap_or_else(|| panic!("{handle} is held: {answer}"))
}

fn at(row: &Value, q: i64, r: i64) {
    assert_eq!(row["place"]["q"], q, "{row}");
    assert_eq!(row["place"]["r"], r, "{row}");
}

const S62: &str = "62_reports_between_commands.toml";
const S63: &str = "63_a_superiors_knowledge.toml";
const S64: &str = "64_commanders_and_initiative.toml";

/// Scenario 62 (hand calculation in `tests/scenario_62_reports_between_commands.rs`):
/// at the start every command holds contact 1, 1 carrier at (12,0), age 0,
/// and the area command holds every force and base as of the start, which is
/// the pulse just ended. Task force 1 strikes and reports "nothing there" in
/// the morning of day 1; the area command is told on time and drops contact
/// 1 in that morning. Reports up arrive on time, so after day 1 it holds task
/// force 1 at (6,0), task force 2 at (7,0) and base 1 at (0,5), each as of
/// the afternoon of day 1, which is the pulse just ended.
#[test]
fn scenario_62_gives_the_area_picture_and_what_the_area_holds_at_the_start_and_after_day_1() {
    let mut s = session_at(S62, &scenario_text(S62), 1);
    let a = call(&mut s, "command_state");
    let picture = &a["picture"];
    assert_eq!(picture["command"], "area");
    assert_eq!(picture["reports_travel"], true);
    let contacts = picture["contacts"].as_array().expect("contacts");
    assert_eq!(contacts.len(), 1, "{picture}");
    let c = &contacts[0];
    assert_eq!(c["contact"]["handle"], "contact:1");
    at(c, 12, 0);
    assert_eq!(c["taken_for"], "carrier");
    assert_eq!(c["ships"], 1);
    assert_eq!(c["age_pulses"], 0);
    // The area command has no units of its own to have sighted it.
    assert_eq!(c["own"], false);
    // The contact is the one list_contacts gives, by the same number.
    let list = call(&mut s, "list_contacts");
    assert_eq!(list["contacts"][0]["contact"], c["contact"], "{list}");

    let held_forces = a["forces_held"].as_array().expect("forces_held");
    assert_eq!(held_forces.len(), 3, "{a}");
    for (key, handle, q, r) in [
        ("task_force", "task_force:1", 0, 0),
        ("task_force", "task_force:2", 7, 0),
        ("base", "base:1", 0, 5),
    ] {
        let row = held(&a, key, handle);
        at(row, q, r);
        assert_eq!(row["as_of"], "the start", "{row}");
        assert_eq!(row["current"], true, "{row}");
    }
    assert_eq!(
        held(&a, "task_force", "task_force:1")["radio_silence"],
        false
    );
    assert!(held(&a, "base", "base:1").get("radio_silence").is_none());

    let mut s = session_at(S62, &scenario_text(S62), 2);
    let a = call(&mut s, "command_state");
    assert_eq!(a["picture"]["contacts"], json!([]), "{a}");
    for (key, handle, q, r) in [
        ("task_force", "task_force:1", 6, 0),
        ("task_force", "task_force:2", 7, 0),
        ("base", "base:1", 0, 5),
    ] {
        let row = held(&a, key, handle);
        at(row, q, r);
        assert_eq!(row["as_of"], "the afternoon of day 1", "{row}");
        assert_eq!(row["current"], true, "{row}");
    }
}

/// Scenario 63 (hand calculation in `tests/scenario_63_a_superiors_knowledge.rs`):
/// task force 1 is silent, sends one state report, in the morning of day 1,
/// and is back at (0,0) from the night of day 2. On day 3 the area command
/// holds her at (6,0) as of the morning of day 1, which is not the pulse just
/// ended, and base 1 as of the pulse just ended. It holds no contact: it
/// dropped contact 1 in the morning of day 1.
#[test]
fn scenario_63_gives_a_silent_force_as_last_reported_with_its_moment_and_not_current() {
    let mut s = session_at(S63, &scenario_text(S63), 3);
    let a = call(&mut s, "command_state");
    assert_eq!(a["picture"]["contacts"], json!([]), "{a}");
    let force = held(&a, "task_force", "task_force:1");
    at(force, 6, 0);
    assert_eq!(force["as_of"], "the morning of day 1", "{force}");
    assert_eq!(force["current"], false, "{force}");
    assert_eq!(force["radio_silence"], true, "{force}");
    let base = held(&a, "base", "base:1");
    at(base, 0, 5);
    assert_eq!(base["as_of"], "the afternoon of day 2", "{base}");
    assert_eq!(base["current"], true, "{base}");
    // Its moment is in the words list_forces uses.
    let list = call(&mut s, "list_forces");
    assert_eq!(list["task_forces"][0]["as_of"], force["as_of"], "{list}");
}

/// Scenario 63 with the reports up never arriving: the area command holds
/// everything as of the start in every pulse, so after day 1 the start is no
/// longer the pulse just ended, and the contact it held at the start it still
/// holds, aged by the pulses, until it is dropped at 5.
#[test]
fn where_reports_up_never_arrive_every_force_is_held_as_of_the_start_and_not_current() {
    let reports = std::fs::read_to_string(
        PathBuf::from(support::ROOT).join("data/reference/reports_placeholder_scenario_63.toml"),
    )
    .expect("the reports figures read")
    .replace("on_time = \"1\"", "on_time = \"0\"");
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("reports_63_never_up.toml");
    std::fs::write(&path, reports).expect("the file is written");
    let text = scenario_text(S63).replacen(
        "reports = \"../reference/reports_placeholder_scenario_63.toml\"",
        &format!("reports = {:?}", path.display().to_string()),
        1,
    );
    let mut s = session_at(S63, &text, 2);
    let a = call(&mut s, "command_state");
    for (key, handle, r) in [("task_force", "task_force:1", 0), ("base", "base:1", 5)] {
        let row = held(&a, key, handle);
        at(row, 0, r);
        assert_eq!(row["as_of"], "the start", "{row}");
        assert_eq!(row["current"], false, "{row}");
    }
    // Contact 1 was held at the start and nobody told the area command that
    // it was not there: it is held, aged by the four pulses of day 1.
    let contacts = a["picture"]["contacts"].as_array().expect("contacts");
    assert_eq!(contacts.len(), 1, "{a}");
    assert_eq!(contacts[0]["age_pulses"], 3, "{a}");
}

/// Scenario 64: the contact is a cruiser task force 1 saw, in her picture
/// alone: the area command holds none.
#[test]
fn scenario_64_the_area_command_holds_no_contact_a_force_saw_and_did_not_report() {
    let mut s = session_at(S64, &scenario_text(S64), 1);
    let a = call(&mut s, "command_state");
    assert_eq!(a["picture"]["contacts"], json!([]), "{a}");
    let force = held(&a, "task_force", "task_force:1");
    assert_eq!(force["as_of"], "the start");
    assert_eq!(force["current"], true);
}

/// With command levels off, nothing is added: the answer has the keys it had.
#[test]
fn with_command_levels_off_command_state_gives_no_picture_and_no_forces_held() {
    let text = scenario_text(S62).replacen("command_levels = true\n", "", 1);
    let mut s = session_at(S62, &text, 1);
    let a = call(&mut s, "command_state");
    assert!(a.get("picture").is_none(), "{a}");
    assert!(a.get("forces_held").is_none(), "{a}");
    // The keys are exactly those the answer had before this change, and the
    // two of decision 0214.
    let mut keys: Vec<&str> = a.as_object().unwrap().keys().map(String::as_str).collect();
    keys.sort_unstable();
    assert_eq!(
        keys,
        [
            "as_at",
            "commitments",
            "for_day",
            "format",
            "holds",
            "intents",
            "interrupts",
            "kind",
            "leaves",
            "notes",
            "plans",
            "pursuit_refusals",
            "pursuits",
            "queued",
            "reserves",
            "staff_decisions_yesterday",
            "version"
        ]
    );
}

/// The report counts refusals over the game: none in scenario 63, so the
/// summary says nothing of them; then a scripted command the engine refuses
/// (a hold on a ship that is not the side's) is counted, with a sentence.
#[test]
fn the_report_counts_refusals_by_cause_and_says_so_only_when_there_are_some() {
    let mut s = session_at(S63, &scenario_text(S63), 3);
    let r = call(&mut s, "report");
    assert_eq!(
        r["refusals"],
        json!({"staff_orders_not_as_held": 0, "staff_orders_other": 0, "commands": 0}),
        "{r}"
    );
    assert!(!r["summary"].as_str().unwrap().contains("Refusals"), "{r}");

    let text = format!(
        "{}\n[[command]]\nday = 2\nside = \"allied\"\nkind = \"hold\"\nship = 999\n",
        scenario_text(S63)
    );
    let mut s = session_at(S63, &text, 3);
    let r = call(&mut s, "report");
    assert_eq!(
        r["refusals"],
        json!({"staff_orders_not_as_held": 0, "staff_orders_other": 0, "commands": 1}),
        "{r}"
    );
    assert!(
        r["summary"]
            .as_str()
            .unwrap()
            .contains("Refusals: 1 command refused."),
        "{r}"
    );
}
