//! The area seat reads its own forces and bases as the area command holds
//! them (decision 0136, 9 and 11): a force or a base as last reported says
//! the moment it is of, a force on radio silence says so, and the order
//! `set_radio_silence` is taken. Over scenario 62 (command levels on, reports
//! up on time, sideways and down never), with task force 1 kept silent from
//! the start where a test needs one stale force beside a fresh one.

use crate::support;

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::state::Side;
use support::{load_text, scenario_text, schemas};

const SCENARIO: &str = "62_reports_between_commands.toml";
const REPORTS_KEY: &str = "reports = \"../reference/reports_placeholder_scenario_62.toml\"";

/// A reports file of this test's own: reports up never arrive.
fn never_up() -> String {
    let text = std::fs::read_to_string(
        PathBuf::from(support::ROOT).join("data/reference/reports_placeholder_scenario_62.toml"),
    )
    .expect("the reports figures read")
    .replace("on_time = \"1\"", "on_time = \"0\"");
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("reports_never_up.toml");
    std::fs::write(&path, text).expect("the file is written");
    path.display().to_string()
}

fn session(text: &str) -> Session {
    let scenario = load_text(text, SCENARIO, 1942);
    let name = scenario.name.clone();
    Session::new(
        scenario.world,
        &name,
        scenario.days,
        Seat::Player,
        Seat::Script(BTreeMap::new()),
    )
}

/// Task force 1 on radio silence from the start, reports up on time.
fn silent_text() -> String {
    scenario_text(SCENARIO).replacen(
        "name = \"Placeholder carrier force 1\"\nside = \"allied\"\n",
        "name = \"Placeholder carrier force 1\"\nside = \"allied\"\nradio_silence = true\n",
        1,
    )
}

struct Player {
    session: Session,
    schemas: support::validator::Schemas,
}

impl Player {
    fn new(text: &str) -> Player {
        Player {
            session: session(text),
            schemas: schemas(),
        }
    }

    fn call(&mut self, tool: &str, args: Value) -> Value {
        let answer = self
            .session
            .call(Side::Allied, tool, &args)
            .unwrap_or_else(|e| panic!("{tool}: {e}"));
        let kind = answer["kind"].as_str().expect("an answer has a kind");
        let errors = self.schemas.check(&format!("{kind}.schema.json"), &answer);
        assert!(errors.is_empty(), "{tool} {args}: {errors:?}\n{answer}");
        answer
    }

    /// Both forces steam east, then the day is ended.
    fn move_both_and_end_day(&mut self) {
        let a = self.call(
            "issue_orders",
            json!({"orders": [
                {"unit": "task_force:1", "order": {"kind": "set_route", "waypoints": [[3, 0]]}},
                {"unit": "task_force:2", "order": {"kind": "set_route", "waypoints": [[10, 0]]}},
            ]}),
        );
        assert_eq!(a["results"][0]["status"], "queued", "{a}");
        assert_eq!(a["results"][1]["status"], "queued", "{a}");
        let end = self.call("end_day", json!({}));
        assert_eq!(end["resolved"], true);
    }
}

fn force(list: &Value, number: u64) -> &Value {
    list["task_forces"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["task_force"]["handle"] == format!("task_force:{number}"))
        .unwrap_or_else(|| panic!("task force {number} is listed: {list}"))
}

#[test]
fn a_silent_force_is_given_as_last_reported_and_a_fresh_one_is_not() {
    let mut p = Player::new(&silent_text());
    // At the start every command is held exactly as of the start, which is
    // the pulse just ended: no moment is given.
    let before = p.call("list_forces", json!({}));
    assert!(force(&before, 1).get("as_of").is_none(), "{before}");
    assert_eq!(force(&before, 1)["radio_silence"], true, "{before}");
    assert!(force(&before, 2).get("radio_silence").is_none(), "{before}");
    let start_place = force(&before, 1)["place"].clone();
    let fresh_place = force(&before, 2)["place"].clone();

    p.move_both_and_end_day();
    let after = p.call("list_forces", json!({}));
    // Task force 1 sent nothing: held where it was, of the start.
    let silent = force(&after, 1);
    assert_eq!(silent["as_of"], "the start", "{after}");
    assert_eq!(silent["place"], start_place, "{after}");
    assert_eq!(silent["radio_silence"], true);
    // Task force 2 reported at the end of the day's last pulse: current, no
    // moment, and it has moved.
    let fresh = force(&after, 2);
    assert!(fresh.get("as_of").is_none(), "{after}");
    assert_ne!(fresh["place"], fresh_place, "{after}");
    assert!(fresh.get("radio_silence").is_none());

    // A base reports on time too: no moment.
    let bases = p.call("list_bases", json!({}));
    assert!(bases["bases"][0].get("as_of").is_none(), "{bases}");

    // The detail and the briefing carry the same moment, in words as well.
    let entity = p.call("get_entity", json!({"handle": "task_force:1"}));
    assert_eq!(
        entity["entity"]["summary"]["as_of"], "the start",
        "{entity}"
    );
    let text = p.call("get_briefing", json!({"text": true}));
    let text = text["text"].as_str().unwrap();
    assert!(text.contains("(as last reported, of the start)"), "{text}");
    let briefing = p.call("get_briefing", json!({}));
    let held = briefing["task_forces"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["task_force"]["handle"] == "task_force:1")
        .unwrap();
    assert_eq!(held["as_of"], "the start", "{briefing}");
}

#[test]
fn where_no_report_arrives_everything_is_as_of_the_start_and_the_base_too() {
    let text =
        scenario_text(SCENARIO).replacen(REPORTS_KEY, &format!("reports = {:?}", never_up()), 1);
    let mut p = Player::new(&text);
    p.move_both_and_end_day();
    let forces = p.call("list_forces", json!({}));
    for number in [1, 2] {
        assert_eq!(force(&forces, number)["as_of"], "the start", "{forces}");
    }
    let bases = p.call("list_bases", json!({}));
    assert_eq!(bases["bases"][0]["as_of"], "the start", "{bases}");
    // A day later nothing has arrived either: still of the start.
    let end = p.call("end_day", json!({}));
    assert_eq!(end["resolved"], true);
    let forces = p.call("list_forces", json!({}));
    assert_eq!(force(&forces, 2)["as_of"], "the start", "{forces}");
}

#[test]
fn with_command_levels_off_no_force_or_base_has_a_moment() {
    let text = scenario_text(SCENARIO).replacen("command_levels = true\n", "", 1);
    let mut p = Player::new(&text);
    p.move_both_and_end_day();
    let forces = p.call("list_forces", json!({}));
    for number in [1, 2] {
        assert!(force(&forces, number).get("as_of").is_none(), "{forces}");
        assert!(force(&forces, number).get("radio_silence").is_none());
    }
    let bases = p.call("list_bases", json!({}));
    assert!(bases["bases"][0].get("as_of").is_none(), "{bases}");
}

#[test]
fn set_radio_silence_is_taken_and_shown() {
    let mut p = Player::new(&scenario_text(SCENARIO));
    let a = p.call(
        "issue_orders",
        json!({"orders": [{"unit": "task_force:1", "order": {"kind": "set_radio_silence", "on": true}}]}),
    );
    assert_eq!(a["results"][0]["status"], "queued", "{a}");
    let end = p.call("end_day", json!({}));
    assert_eq!(end["resolved"], true);
    let forces = p.call("list_forces", json!({}));
    assert_eq!(force(&forces, 1)["radio_silence"], true, "{forces}");
    // Off again: the line says nothing of it.
    let a = p.call(
        "issue_orders",
        json!({"orders": [{"unit": "task_force:1", "order": {"kind": "set_radio_silence", "on": false}}]}),
    );
    assert_eq!(a["results"][0]["status"], "queued", "{a}");
    p.call("end_day", json!({}));
    let forces = p.call("list_forces", json!({}));
    assert!(force(&forces, 1).get("radio_silence").is_none(), "{forces}");

    // A missing "on" is refused at the decoder.
    let refused = p.session.call(
        Side::Allied,
        "issue_orders",
        &json!({"orders": [{"unit": "task_force:1", "order": {"kind": "set_radio_silence"}}]}),
    );
    assert!(refused.is_err() || refused.unwrap()["results"][0]["status"] != "queued");
}
