//! The tool `order` (decision 0159, rule E, items 15 to 17): the plan of one
//! of the commander's intents as the staff hands it in, on scenario 80, a
//! take that waits for a second unit. Its answer fits its schema, tells the
//! phases and what the plan waits for, and refuses an intent that is not the
//! commander's in the words `propose` uses.

use crate::support;

use serde_json::{json, Value};
use sim_core::state::Side;
use support::{load, schemas};

#[test]
fn the_plan_of_a_take_that_waits_is_given_as_an_order() {
    let scenario = load("80_a_take_that_waits.toml", 1942);
    let (mut session, _) = support::play::session_of(scenario, &[Side::Allied]);
    session = session.with_staffs(sim_staff::staffs());
    let schemas = schemas();
    let mut call = |tool: &str, args: Value| -> Result<Value, sim_agent::CallError> {
        let result = session.call(Side::Allied, tool, &args);
        if let Ok(answer) = &result {
            let kind = answer["kind"].as_str().expect("a kind");
            let errors = schemas.check(&format!("{kind}.schema.json"), answer);
            assert!(errors.is_empty(), "{tool} {args}: {errors:?}\n{answer}");
        }
        result
    };
    for (tool, args) in [
        (
            "set_intent",
            json!({"intent_kind": "take_place", "hex": [4, 0]}),
        ),
        (
            "set_limit",
            json!({"limit": "forces", "intent": 1, "task_forces": [1], "land_units": [1, 2], "source": 1}),
        ),
        ("set_leave", json!({"leave": "prepare", "given": true})),
        ("set_leave", json!({"leave": "launch", "given": true})),
    ] {
        let a = call(tool, args).expect("answered");
        assert_eq!(a["status"], "queued", "{a}");
    }
    call("end_day", json!({})).expect("day 1");
    let a = call("order", json!({"intent": 1})).expect("the order of intent 1");
    assert_eq!(a["kind"], "plan_order");
    assert_eq!(a["intent"], 1);
    assert_eq!(a["for_day"], 2);
    let names: Vec<&str> = a["phases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| p["name"].as_str().unwrap())
        .collect();
    assert_eq!(names, ["prepare", "move", "land"], "{a}");
    assert!(a["text"].as_array().unwrap().len() >= 3, "{a}");

    // The staff's own intent and an intent that is not there are refused in
    // the words `propose` uses.
    for intent in [2, 99] {
        let refused = call("order", json!({"intent": intent})).expect_err("refused");
        assert_eq!(
            refused.message,
            format!("intent:{intent} is not an intent in force (command_state lists the intents)")
        );
        let proposed = call("propose", json!({"intent": 99})).expect_err("refused");
        assert!(proposed
            .message
            .starts_with("intent:99 is not an intent in force"));
    }
    let missing = call("order", json!({})).expect_err("refused");
    assert!(
        missing.message.contains("\"intent\" is needed"),
        "{}",
        missing.message
    );
}
