//! What a force in port lacks of fuel and the hours it would take (decision
//! 0112, 6), on scenario 44 at its start, before any pulse is resolved.
//!
//! Hand figures: S (port size 1: one berth, fuel at a berth 100 t an hour,
//! by lighter 40) holds forces 1 and 2. Ship 1 (cargo ship, 1000 t of fuel)
//! has 600 t: lacks 400 t, 4 hours at the berth rate. Ship 2 (cruiser, 2000
//! t) has 800 t: lacks 1200 t, 12 hours. U (port size 0: no berth, by
//! lighter 40) holds force 3: lacks 1500 t, 37.5 hours. V holds force 4: the
//! destroyer lacks 500 t, 12.5 hours. With the frozen port sizes, which give
//! no fuelling rate, the tons are the same and no hours are given.

use crate::support;

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::state::Side;
use std::collections::BTreeMap;

fn session(text: &str) -> Session {
    let scenario = support::load_text(text, "44_fuelling_takes_time.toml", 1942);
    let name = scenario.name.clone();
    Session::new(
        scenario.world,
        &name,
        scenario.days,
        Seat::Player,
        Seat::Script(BTreeMap::new()),
    )
}

fn forces(session: &mut Session) -> Vec<Value> {
    let list = session
        .call(Side::Allied, "list_forces", &json!({}))
        .expect("the forces are listed");
    list["task_forces"].as_array().expect("forces").clone()
}

fn force(forces: &[Value], number: u32) -> &Value {
    forces
        .iter()
        .find(|f| f["task_force"]["handle"] == format!("task_force:{number}"))
        .expect("the force is listed")
}

#[test]
fn a_force_in_port_says_what_it_lacks_and_the_hours_at_the_berth_or_by_lighter() {
    let mut session = session(&support::scenario_text("44_fuelling_takes_time.toml"));
    let list = forces(&mut session);
    for (number, lacked, hours, base) in [
        (1, "400", "4", "base:1"),
        (2, "1200", "12", "base:1"),
        (3, "1500", "37.5", "base:2"),
        (4, "500", "12.5", "base:3"),
    ] {
        let line = force(&list, number);
        assert_eq!(line["fuel_lacked_t"], lacked, "force {number}");
        assert_eq!(line["fuelling_hours"], hours, "force {number}");
        assert_eq!(line["fuelling_at"]["handle"], base, "force {number}");
    }
    let briefing = session
        .call(Side::Allied, "get_briefing", &json!({ "text": true }))
        .expect("the briefing is given");
    let text = briefing["text"].as_str().expect("text");
    assert!(
        text.contains("lacks 1200 t of fuel: 12 hours at base:1"),
        "{text}"
    );
}

#[test]
fn with_no_rate_given_the_tons_are_said_and_no_hours() {
    let text = support::scenario_text("44_fuelling_takes_time.toml")
        .replace("port_sizes_placeholder_fuelling.csv", "port_sizes.csv");
    let mut session = session(&text);
    let list = forces(&mut session);
    let line = force(&list, 2);
    assert_eq!(line["fuel_lacked_t"], "1200");
    assert!(line.get("fuelling_hours").is_none());
    let briefing = session
        .call(Side::Allied, "get_briefing", &json!({ "text": true }))
        .expect("the briefing is given");
    let text = briefing["text"].as_str().expect("text");
    assert!(
        text.contains("lacks 1200 t of fuel, taken in its first pulse in port"),
        "{text}"
    );
}

#[test]
fn a_force_under_way_or_with_a_full_tank_says_nothing() {
    // Scenario 1 starts with a force under no order and full ships: nothing.
    let scenario = support::load("01_transit.toml", 1942);
    let name = scenario.name.clone();
    let mut session = Session::new(
        scenario.world,
        &name,
        scenario.days,
        Seat::Player,
        Seat::Script(BTreeMap::new()),
    );
    for line in forces(&mut session) {
        assert!(line.get("fuel_lacked_t").is_none(), "{line}");
    }
}
