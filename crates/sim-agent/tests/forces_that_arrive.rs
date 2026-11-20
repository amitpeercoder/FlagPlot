//! Forces that arrive, as the agent interface gives them (decision 0195, 7
//! and 13): what is held apart for a side until its day is listed in the
//! briefing's `arrivals.to_enter` and in `list_forces`, `get_entity` answers
//! for it, an order naming it is refused with its day, and the enemy's entries
//! are never given. A game with nothing held reads as it always did.
//!
//! Scenario 72 with task force 1 (two transports in port at base 1) entering
//! on day 2, air group 1 (at base 1) on day 2, land unit 1 (ashore at base 1)
//! on day 3 and the Japanese garrison, land unit 10 (at base 2), on day 2.

mod support;

use serde_json::{json, Value};
use sim_core::state::Side;
use support::check::valid;
use support::play::{play_day, session_of};
use support::validator::Schemas;
use support::{load_text, scenario_text};

/// `text` with `line` added after the `side` line of the table `kind` whose
/// `id` is `id`.
fn add_to_table(text: &str, kind: &str, id: u32, line: &str) -> String {
    let head = format!("[[{kind}]]\nid = {id}\n");
    let at = text.find(&head).unwrap_or_else(|| panic!("no {kind} {id}"));
    let side = at + text[at..].find("side = ").expect("a side line");
    let end = side + text[side..].find('\n').expect("a line end") + 1;
    format!("{}{line}\n{}", &text[..end], &text[end..])
}

fn call(session: &mut sim_agent::Session, side: Side, tool: &str, args: Value) -> Value {
    session
        .call(side, tool, &args)
        .unwrap_or_else(|e| panic!("{tool} was refused: {e}"))
}

fn session(held: bool) -> sim_agent::Session {
    let mut text = scenario_text("72_reconnaissance.toml");
    if held {
        text = add_to_table(&text, "task_force", 1, "enters = 2");
        text = add_to_table(&text, "air_group", 1, "enters = 2");
        text = add_to_table(&text, "land_unit", 1, "enters = 3");
        text = add_to_table(&text, "land_unit", 10, "enters = 2");
    }
    let scenario = load_text(&text, "72_reconnaissance.toml", 1942);
    session_of(scenario, &[Side::Allied, Side::Japanese]).0
}

#[test]
fn the_briefing_lists_what_is_to_enter_under_what_is_due_and_the_enemys_has_none_of_it() {
    let mut s = session(true);
    let allied = call(&mut s, Side::Allied, "get_briefing", json!({}));
    assert_eq!(allied["version"], sim_agent::VERSION);
    let to_enter = allied["arrivals"]["to_enter"].as_array().expect("to_enter");
    assert_eq!(to_enter.len(), 3, "{allied}");
    let force = to_enter
        .iter()
        .find(|e| e["kind"] == "task_force")
        .expect("the force");
    assert_eq!(force["day"], 2);
    assert_eq!(force["unit"]["handle"], "task_force:1");
    assert_eq!(force["hex"]["q"], 0);
    assert_eq!(force["ships"], json!([{"kind": "transport", "ships": 2}]));
    assert_eq!(force["waiting"], false);
    assert!(force.get("base").is_none() && force.get("waits_for").is_none());
    let group = to_enter
        .iter()
        .find(|e| e["kind"] == "air_group")
        .expect("the group");
    assert_eq!(group["base"]["handle"], "base:1");
    assert_eq!(group["air_groups"][0]["aircraft"], 6);
    let unit = to_enter
        .iter()
        .find(|e| e["kind"] == "land_unit")
        .expect("the unit");
    assert_eq!(unit["day"], 3);
    assert_eq!(unit["land_units"][0]["men"], 600);

    // The Japanese side has one entry of its own, the garrison, and none of
    // the Allied side's.
    let japanese = call(&mut s, Side::Japanese, "get_briefing", json!({}));
    let theirs = japanese["arrivals"]["to_enter"].as_array().expect("theirs");
    assert_eq!(theirs.len(), 1, "{japanese}");
    assert_eq!(theirs[0]["kind"], "land_unit");
    assert_eq!(theirs[0]["land_units"][0]["men"], 150);

    // The text says it in words.
    let text = call(&mut s, Side::Allied, "get_briefing", json!({"text": true}));
    let text = text["text"].as_str().expect("text");
    assert!(text.contains("enters on day 2 at"), "{text}");
}

#[test]
fn a_game_with_nothing_held_has_none_of_the_new_fields() {
    let mut s = session(false);
    for side in [Side::Allied, Side::Japanese] {
        let briefing = call(&mut s, side, "get_briefing", json!({}));
        let arrivals = briefing["arrivals"].as_object().expect("arrivals");
        for key in ["to_enter", "entered_yesterday", "entry_waiting_yesterday"] {
            assert!(!arrivals.contains_key(key), "{key}: {briefing}");
        }
        let list = call(&mut s, side, "list_forces", json!({}));
        assert!(list.get("to_enter").is_none(), "{list}");
        let text = call(&mut s, side, "get_briefing", json!({"text": true}));
        let text = text["text"].as_str().expect("text");
        assert!(!text.contains("To enter") && !text.contains("enters on day"));
    }
}

#[test]
fn a_held_unit_is_not_on_the_map_but_list_forces_and_get_entity_say_when_and_where() {
    let mut s = session(true);
    let list = call(&mut s, Side::Allied, "list_forces", json!({}));
    assert!(
        list["task_forces"].as_array().expect("forces").is_empty(),
        "{list}"
    );
    assert!(list
        .get("land_units")
        .is_none_or(|u| u.as_array().is_some_and(|u| u.len() == 1)));
    assert_eq!(
        list["to_enter"].as_array().expect("to_enter").len(),
        3,
        "{list}"
    );
    // In an area that holds neither hex, nothing is to enter.
    let far = call(
        &mut s,
        Side::Allied,
        "list_forces",
        json!({"area": {"q": 30, "r": 30, "radius_nm": 1}}),
    );
    assert!(far.get("to_enter").is_none(), "{far}");

    for (handle, held) in [
        ("task_force:1", "task_force"),
        ("ship:1", "ship"),
        ("air_group:1", "air_group"),
        ("land_unit:1", "land_unit"),
    ] {
        let answer = call(
            &mut s,
            Side::Allied,
            "get_entity",
            json!({"handle": handle}),
        );
        assert_eq!(answer["entity"]["entity_kind"], "to_enter", "{answer}");
        assert_eq!(answer["entity"]["held"], held);
        assert_eq!(answer["entity"]["unit"]["handle"], handle);
        assert!(answer["entity"]["enters"]["day"].as_u64().is_some());
    }
    let ship = call(
        &mut s,
        Side::Allied,
        "get_entity",
        json!({"handle": "ship:2"}),
    );
    assert_eq!(ship["entity"]["enters"]["unit"]["handle"], "task_force:1");
    assert_eq!(ship["entity"]["enters"]["day"], 2);

    // The enemy's held unit is not named to the Allied side: its number is
    // one the Allied side does not have.
    let enemy = call(
        &mut s,
        Side::Allied,
        "get_entity",
        json!({"handle": "land_unit:3"}),
    );
    assert_eq!(enemy["kind"], "not_known", "{enemy}");
}

#[test]
fn an_order_naming_a_held_unit_is_refused_with_its_day() {
    let mut s = session(true);
    let refused = call(
        &mut s,
        Side::Allied,
        "issue_orders",
        json!({"orders": [
            {"unit": "task_force:1", "order": {"kind": "set_speed", "speed": "full"}},
            {"unit": "land_unit:1", "order": {"kind": "set_posture", "posture": "defend"}},
        ]}),
    );
    for index in 0..2 {
        let reason = refused["results"][index]["reason"]
            .as_str()
            .expect("reason");
        assert!(
            reason.contains("is not on the map yet: it enters on day"),
            "{reason}"
        );
        assert!(reason.contains("not_yet_entered"), "{reason}");
    }
    assert!(refused["results"][0]["reason"]
        .as_str()
        .unwrap()
        .contains("day 2"));
    assert!(refused["results"][1]["reason"]
        .as_str()
        .unwrap()
        .contains("day 3"));
}

#[test]
fn what_enters_is_told_the_next_day_and_leaves_the_list() {
    let mut s = session(true);
    // Day 1 played, day 2 played: the force and the group came in on day 2.
    play_day(&mut s, Side::Japanese, &[]);
    play_day(&mut s, Side::Allied, &[]);
    play_day(&mut s, Side::Japanese, &[]);
    play_day(&mut s, Side::Allied, &[]);
    let briefing = call(&mut s, Side::Allied, "get_briefing", json!({}));
    let entered = briefing["arrivals"]["entered_yesterday"]
        .as_array()
        .unwrap_or_else(|| panic!("entered_yesterday: {briefing}"));
    assert_eq!(entered.len(), 2, "{briefing}");
    assert!(entered
        .iter()
        .any(|e| e["unit"]["handle"] == "task_force:1"));
    // The land unit is still to enter, on day 3.
    let to_enter = briefing["arrivals"]["to_enter"]
        .as_array()
        .expect("to_enter");
    assert_eq!(to_enter.len(), 1, "{briefing}");
    // The force is on the map now and an order to it is taken.
    let list = call(&mut s, Side::Allied, "list_forces", json!({}));
    assert_eq!(
        list["task_forces"].as_array().expect("forces").len(),
        1,
        "{list}"
    );
}

#[test]
fn every_answer_about_a_held_unit_is_as_the_schemas_say() {
    let schemas =
        Schemas::load(&std::path::Path::new(support::ROOT).join("crates/sim-agent/schemas"));
    let mut s = session(true);
    let briefing = call(&mut s, Side::Allied, "get_briefing", json!({}));
    valid(&schemas, "briefing.schema.json", &briefing, "held briefing");
    let list = call(&mut s, Side::Allied, "list_forces", json!({}));
    valid(&schemas, "list_forces.schema.json", &list, "held list");
    for handle in ["task_force:1", "ship:1", "air_group:1", "land_unit:1"] {
        let entity = call(
            &mut s,
            Side::Allied,
            "get_entity",
            json!({"handle": handle}),
        );
        valid(&schemas, "entity.schema.json", &entity, handle);
    }
    // Once it has entered, the day after says so.
    play_day(&mut s, Side::Japanese, &[]);
    play_day(&mut s, Side::Allied, &[]);
    play_day(&mut s, Side::Japanese, &[]);
    play_day(&mut s, Side::Allied, &[]);
    let briefing = call(&mut s, Side::Allied, "get_briefing", json!({}));
    valid(
        &schemas,
        "briefing.schema.json",
        &briefing,
        "entered briefing",
    );
    let text = call(&mut s, Side::Allied, "get_briefing", json!({"text": true}));
    assert!(
        text["text"].as_str().unwrap().contains("Entered yesterday"),
        "{text}"
    );
}
