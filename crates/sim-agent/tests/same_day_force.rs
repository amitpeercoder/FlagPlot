//! An order to a force a queued forming order will make, handed in the same
//! day (decision 0111's open point, settled in decision 0107, "As built"):
//! the session checks an order beside the orders it has queued
//! (`World::check_order_queued`).
//!
//! Runs on scenario 39, as `forming.rs` does: ships 5 and 6 lie in port at
//! base 2 in no force, and the side has three forces, so the force a forming
//! order makes is force 4 by position.

mod support;

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::state::Side;
use support::load;

fn session() -> Session {
    let scenario = load("39_load_reserve_and_forces.toml", 1);
    Session::new(
        scenario.world,
        "Scenario 39",
        scenario.days,
        Seat::Player,
        Seat::Script(Default::default()),
    )
}

fn issue(session: &mut Session, orders: Value) -> Value {
    session
        .call(Side::Allied, "issue_orders", &json!({ "orders": orders }))
        .expect("issue_orders is answered")
}

fn status(answer: &Value, index: usize) -> &str {
    answer["results"][index]["status"]
        .as_str()
        .expect("a status")
}

fn form() -> Value {
    json!({ "unit": "base:2", "order": { "kind": "form_task_force", "ships": [5, 6] } })
}

fn load_new() -> Value {
    json!({ "unit": "task_force:4", "order": { "kind": "load", "weight_t": "100" } })
}

#[test]
fn an_order_to_the_force_a_queued_forming_order_makes_is_queued_in_the_same_call() {
    let mut session = session();
    let answer = issue(&mut session, json!([form(), load_new()]));
    assert_eq!(status(&answer, 0), "queued", "{answer}");
    assert_eq!(status(&answer, 1), "queued", "{answer}");
}

#[test]
fn an_order_to_that_force_is_queued_when_the_forming_order_was_queued_by_an_earlier_call() {
    let mut session = session();
    let first = issue(&mut session, json!([form()]));
    assert_eq!(status(&first, 0), "queued", "{first}");
    let second = issue(&mut session, json!([load_new()]));
    assert_eq!(status(&second, 0), "queued", "{second}");
}

#[test]
fn with_no_forming_order_queued_the_force_is_not_known() {
    let mut session = session();
    let answer = issue(&mut session, json!([load_new()]));
    assert_eq!(status(&answer, 0), "refused", "{answer}");
    assert_eq!(answer["results"][0]["reason"], "not known");
    // A number further past the forces than the forming orders make is not
    // known either.
    let answer = issue(
        &mut session,
        json!([form(), {
            "unit": "task_force:5",
            "order": { "kind": "load", "weight_t": "100" }
        }]),
    );
    assert_eq!(status(&answer, 0), "queued", "{answer}");
    assert_eq!(status(&answer, 1), "refused", "{answer}");
    assert_eq!(answer["results"][1]["reason"], "not known");
}

#[test]
fn the_day_resolves_with_the_new_force_loading() {
    let mut session = session();
    let answer = issue(&mut session, json!([form(), load_new()]));
    assert_eq!(status(&answer, 1), "queued", "{answer}");
    session
        .call(Side::Allied, "end_day", &json!({}))
        .expect("end_day is answered");
    let briefing = session
        .call(Side::Allied, "get_briefing", &json!({ "text": true }))
        .expect("the briefing");
    let text = briefing["text"].as_str().expect("text");
    assert!(text.contains("- Accepted yesterday: 2"), "{text}");
    assert!(text.contains("- Refused yesterday: none"), "{text}");
    assert!(
        text.contains("task_force:4") && text.contains("cargo aboard: 100 t supply"),
        "{text}"
    );
}
