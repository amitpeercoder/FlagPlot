//! A force that acts at a place and steams on, as the agent interface tells it
//! (decision 0096).
//!
//! Scenario 26 (the live file). Handles are each side's own: the Japanese
//! cruiser force is the Japanese side's `task_force:1`; the Allied convoy is
//! the Allied side's `task_force:1` and its destroyer patrol `task_force:2`.
//!
//! The cruiser force is ordered to Allied base 1 at X (0,0), marked to act
//! there, and back to (4,0). It halts at X in the night pulse, bombards, and
//! steams on in the morning. The convoy unloads at base 2 at Y (0,3), marked,
//! then goes home to Z (4,3): it halts in the morning of day 1, works its
//! cargo, and steams on in the night of day 2. The destroyer passes a marked
//! waypoint where it has no work. The orders are the scenario's, given here
//! through `issue_orders`.

mod support;

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::state::Side;
use support::check::valid;
use support::{load, schemas};

fn session() -> Session {
    let scenario = load("26_act_and_steam_on.toml", 1942);
    Session::new(
        scenario.world,
        "Scenario 26",
        scenario.days,
        Seat::Player,
        Seat::Player,
    )
}

fn give(session: &mut Session, side: Side, list: &[Value]) -> Vec<Value> {
    let answer = session
        .call(side, "issue_orders", &json!({ "orders": list }))
        .expect("issue_orders is answered");
    valid(
        &schemas(),
        "issue_orders.schema.json",
        &answer,
        "act orders",
    );
    answer["results"].as_array().expect("results").clone()
}

fn route(unit: &str, waypoints: Value, act_at: Value) -> Value {
    json!({ "unit": unit, "order": { "kind": "set_route", "waypoints": waypoints, "act_at": act_at } })
}

fn cruise(unit: &str) -> Value {
    json!({ "unit": unit, "order": { "kind": "set_speed", "speed": "cruise" } })
}

/// The scenario's day 1 orders, given by both sides.
fn session_with_orders() -> Session {
    let mut session = session();
    let allied = give(
        &mut session,
        Side::Allied,
        &[
            json!({ "unit": "task_force:1", "order": { "kind": "unload" } }),
            route("task_force:1", json!([[0, 3], [4, 3]]), json!([[0, 3]])),
            cruise("task_force:1"),
            route("task_force:2", json!([[2, 5], [4, 5]]), json!([[2, 5]])),
            cruise("task_force:2"),
        ],
    );
    assert!(allied.iter().all(|r| r["status"] == "queued"), "{allied:?}");
    let japanese = give(
        &mut session,
        Side::Japanese,
        &[
            route("task_force:1", json!([[0, 0], [4, 0]]), json!([[0, 0]])),
            cruise("task_force:1"),
        ],
    );
    assert!(
        japanese.iter().all(|r| r["status"] == "queued"),
        "{japanese:?}"
    );
    session
}

fn end_day(session: &mut Session) {
    for side in [Side::Allied, Side::Japanese] {
        session
            .call(side, "end_day", &json!({}))
            .expect("end_day is answered");
    }
}

fn briefing(session: &mut Session, side: Side) -> (String, Value) {
    let text = session
        .call(side, "get_briefing", &json!({ "text": true }))
        .expect("the briefing");
    valid(&schemas(), "briefing_text.schema.json", &text, "act text");
    let data = session
        .call(side, "get_briefing", &json!({ "text": false }))
        .expect("the briefing");
    valid(&schemas(), "briefing.schema.json", &data, "act briefing");
    (text["text"].as_str().expect("text").to_string(), data)
}

fn force<'a>(data: &'a Value, handle: &str) -> &'a Value {
    data["task_forces"]
        .as_array()
        .expect("forces")
        .iter()
        .find(|f| f["task_force"]["handle"] == handle)
        .unwrap_or_else(|| panic!("{handle}"))
}

fn coords(places: &Value) -> Vec<(i64, i64)> {
    places
        .as_array()
        .expect("places")
        .iter()
        .map(|p| (p["q"].as_i64().unwrap(), p["r"].as_i64().unwrap()))
        .collect()
}

/// The briefing at the start of day 1: the scripted orders are given by the
/// scenario, so the marks are on the routes of the forces that have them.
#[test]
fn a_route_with_a_mark_shows_it_and_one_without_shows_none() {
    let mut session = session_with_orders();
    let (_, data) = briefing(&mut session, Side::Allied);
    for f in data["task_forces"].as_array().unwrap() {
        assert!(f.get("halt").is_none(), "{f}");
    }
    end_day(&mut session);
    // After day 1: the destroyer's route is complete (arrived), so it shows
    // no mark; the convoy is halted at Y with its mark.
    let (text, data) = briefing(&mut session, Side::Allied);
    let convoy = force(&data, "task_force:1");
    assert_eq!(convoy["status"], "halted", "{convoy}");
    assert_eq!(coords(&convoy["act_at"]), vec![(0, 3)], "{convoy}");
    assert_eq!(coords(&json!([convoy["halt"]["place"]])), vec![(0, 3)]);
    assert_eq!(convoy["halt"]["waits_for"], json!(["cargo"]), "{convoy}");
    assert_eq!(convoy["halt"]["since"], "D1 10:00", "{convoy}");
    assert!(convoy["halt"].get("bombarded").is_none(), "{convoy}");
    let destroyer = force(&data, "task_force:2");
    assert_eq!(destroyer["status"], "arrived", "{destroyer}");
    assert!(destroyer.get("act_at").is_none(), "{destroyer}");
    assert!(destroyer.get("halt").is_none(), "{destroyer}");
    assert!(
        text.contains("HALTED at (0,3)") && text.contains("since D1 10:00 to work cargo"),
        "{text}"
    );
    assert!(text.contains("route marked to act at (0,3)"), "{text}");
    // Yesterday's halt, with the reason, and the event number to explain.
    let halted = &data["halts"]["halted"];
    assert_eq!(halted.as_array().unwrap().len(), 1, "{halted}");
    assert_eq!(halted[0]["task_force"]["handle"], "task_force:1");
    assert_eq!(halted[0]["waits_for"], json!(["cargo"]));
    assert!(data["halts"]["steamed_on"].as_array().unwrap().is_empty());
    assert!(text.contains("Halted yesterday: task_force:1"), "{text}");
    // The queries say the same of the force.
    let entity = session
        .call(
            Side::Allied,
            "get_entity",
            &json!({ "handle": "task_force:1" }),
        )
        .expect("entity");
    valid(&schemas(), "entity.schema.json", &entity, "act entity");
    assert_eq!(
        entity["entity"]["summary"]["halt"]["waits_for"],
        json!(["cargo"])
    );
    assert_eq!(coords(&entity["entity"]["summary"]["act_at"]), vec![(0, 3)]);
    let list = session
        .call(Side::Allied, "list_forces", &json!({}))
        .expect("list_forces");
    valid(&schemas(), "list_forces.schema.json", &list, "act list");
    let line = list["task_forces"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["task_force"]["handle"] == "task_force:1")
        .expect("the convoy");
    assert_eq!(line["status"], "halted", "{line}");
    assert_eq!(coords(&line["act_at"]), vec![(0, 3)]);
}

#[test]
fn a_force_that_steamed_on_is_told_why_and_explain_works_on_both_events() {
    let mut session = session_with_orders();
    end_day(&mut session);
    end_day(&mut session);
    // Day 2's report: the convoy steamed on in the night, its cargo done.
    let (text, data) = briefing(&mut session, Side::Allied);
    let steamed = &data["halts"]["steamed_on"];
    assert_eq!(steamed.as_array().unwrap().len(), 1, "{steamed}");
    assert_eq!(steamed[0]["task_force"]["handle"], "task_force:1");
    assert_eq!(steamed[0]["waited_for"], json!(["cargo"]));
    assert_eq!(steamed[0]["reasons"], json!(["cargo_work_done"]));
    assert!(
        text.contains("Steamed on yesterday: task_force:1")
            && text.contains(": its cargo work was done [event"),
        "{text}"
    );
    // explain of the steaming on cites the halt and the order.
    let number = steamed[0]["event"].as_u64().unwrap();
    let why = session
        .call(Side::Allied, "explain", &json!({ "event": number }))
        .expect("explain");
    valid(&schemas(), "explain.schema.json", &why, "act explain");
    let node = &why["nodes"][0];
    assert_eq!(node["event_kind"], "task_force_steamed_on", "{why}");
    assert!(
        node["text"].as_str().unwrap().contains("steamed on from"),
        "{why}"
    );
    let kinds: Vec<&str> = why["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|n| n["event_kind"].as_str())
        .collect();
    assert!(kinds.contains(&"task_force_halted"), "{kinds:?}");
    // The raiding force steamed on in day 1 and is home: day 2's report to
    // the Japanese side has no halt and no steaming on.
    let (text, data) = briefing(&mut session, Side::Japanese);
    assert!(data.get("halts").is_none(), "{}", data["halts"]);
    assert!(!text.contains("Steamed on yesterday"), "{text}");
}

#[test]
fn the_raiding_force_halts_bombards_and_is_told_its_reasons_on_day_two() {
    let mut session = session_with_orders();
    end_day(&mut session);
    let (text, data) = briefing(&mut session, Side::Japanese);
    let steamed = &data["halts"]["steamed_on"];
    assert_eq!(steamed.as_array().unwrap().len(), 1, "{steamed}");
    assert_eq!(steamed[0]["task_force"]["handle"], "task_force:1");
    assert_eq!(steamed[0]["waited_for"], json!(["bombard"]));
    assert_eq!(steamed[0]["reasons"], json!(["bombarded"]));
    let halted = &data["halts"]["halted"];
    assert_eq!(halted[0]["at"], "D1 04:00", "{halted}");
    assert_eq!(halted[0]["waits_for"], json!(["bombard"]));
    assert!(
        text.contains("Steamed on yesterday: task_force:1"),
        "{text}"
    );
    assert!(text.contains(": it had bombarded [event"), "{text}");
    // It is home again: no mark ahead of it and no halt.
    let cruiser = force(&data, "task_force:1");
    assert!(cruiser.get("halt").is_none() && cruiser.get("act_at").is_none());
}

/// A mark that is not one of the waypoints is refused, with the engine's
/// words and what the order needs.
#[test]
fn a_mark_that_is_not_a_waypoint_is_refused_with_a_hint() {
    let mut session = session();
    let result = give(
        &mut session,
        Side::Allied,
        &[route(
            "task_force:2",
            json!([[2, 5], [4, 5]]),
            json!([[3, 5]]),
        )],
    );
    assert_eq!(result[0]["status"], "refused", "{result:?}");
    let reason = result[0]["reason"].as_str().expect("a reason");
    assert!(
        reason.contains("(3,5) is marked to act at but is not one of the route's waypoints")
            && reason.contains("the last waypoint needs no mark"),
        "{reason}"
    );
    // The last waypoint may be named, and is then no mark: the order is queued.
    let result = give(
        &mut session,
        Side::Allied,
        &[route(
            "task_force:2",
            json!([[2, 5], [4, 5]]),
            json!([[4, 5]]),
        )],
    );
    assert_eq!(result[0]["status"], "queued", "{result:?}");
}

/// Everything each side is told in a day: the briefing as text and data, the
/// lists, every own force and the explanation of every event it may see.
fn told(session: &mut Session, side: Side) -> String {
    let (text, data) = briefing(session, side);
    let mut all = format!("{text}{data}");
    for question in [
        ("list_forces", json!({})),
        ("list_contacts", json!({})),
        ("list_bases", json!({})),
    ] {
        all.push_str(
            &session
                .call(side, question.0, &question.1)
                .expect("a list")
                .to_string(),
        );
    }
    for n in 1..=3 {
        let answer = session
            .call(
                side,
                "get_entity",
                &json!({ "handle": format!("task_force:{n}") }),
            )
            .expect("entity");
        all.push_str(&answer.to_string());
    }
    for event in 1..=80 {
        let answer = session
            .call(side, "explain", &json!({ "event": event }))
            .expect("explain");
        all.push_str(&answer.to_string());
    }
    all
}

/// Nothing of a side's halts, marks or steamings on is in the other side's
/// answers, on every day, in every answer (decision 0096, 7).
#[test]
fn the_halts_and_marks_of_each_side_are_told_to_that_side_alone() {
    let mut session = session_with_orders();
    let mut told_allied = String::new();
    let mut told_japanese = String::new();
    for _ in 0..3 {
        let allied = told(&mut session, Side::Allied);
        // The Japanese raid on X: the force's halt at X, its bombarding and
        // steaming on.
        for word in [
            "Placeholder raiding force",
            "Placeholder cruiser",
            "HALTED at (0,0)",
            "Halted yesterday: task_force:1 \"Placeholder raiding",
            "steamed on from (0,0)",
            "to bombard",
            "it had bombarded",
            "cannot_bombard_any_longer",
            "\"bombard\"",
        ] {
            assert!(!allied.contains(word), "the Allies were told {word}");
        }
        told_allied.push_str(&allied);
        let japanese = told(&mut session, Side::Japanese);
        // The Allied convoy's and destroyer's halts and marks.
        for word in [
            "Placeholder supply convoy",
            "Placeholder destroyer",
            "Placeholder cargo ship",
            "work cargo",
            "cargo_work_done",
            "its cargo work was done",
            "\"cargo\"",
            "D1 10:00",
            "HALTED at (0,3)",
            "act at (0,3)",
            "from (0,3)",
        ] {
            assert!(!japanese.contains(word), "the Japanese were told {word}");
        }
        told_japanese.push_str(&japanese);
        end_day(&mut session);
    }
    // Each side is told of its own.
    assert!(told_allied.contains("Halted yesterday: task_force:1"));
    assert!(told_allied.contains("its cargo work was done"));
    assert!(told_japanese.contains("Halted yesterday: task_force:1"));
    assert!(told_japanese.contains("it had bombarded"));
}
