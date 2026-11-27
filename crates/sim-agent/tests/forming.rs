//! Decision 0111 in the agent's answers: a load up to a weight, the reserves
//! of a base, and forces formed, joined and stood down.
//!
//! Runs on scenario 39 (`data/scenarios/39_load_reserve_and_forces.toml`),
//! whose hand calculation the architect wrote in
//! `tests/scenario_39_load_reserve_and_forces.rs`. The figures asserted here
//! are the ones of that calculation:
//!
//! - Base 1 (S) holds 1000 t of supply and 500 t of fuel, with reserves of 400
//!   and 300. Base 2 (T) holds 100 t of supply and keeps no reserve. Ships 5
//!   and 6 lie in port at base 2 in no force.
//! - A load of supply by force 1 up to 700 t: it asks 700 t of a stock of 1000
//!   t with 600 t above the reserve, so it may take 700 - 600 = 100 t below
//!   the reserve. Up to 250 t it asks less than the 600 t above the reserve
//!   and no answer is given.
//! - Day 1's refuelling at S: force 1's ship 1 takes 150 t of the 200 t above
//!   the reserve, and ship 2, which wants 100 t, takes the 50 t left and is
//!   50 t short of full.
//! - Ships 5 and 6, formed on day 1, make the side's next force by position,
//!   force 4. On day 2 force 3 joins it; on day 3 it is stood down.

use crate::support;

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::state::Side;
use support::check::valid;
use support::{load, schemas};

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
    let answer = session
        .call(Side::Allied, "issue_orders", &json!({ "orders": orders }))
        .expect("issue_orders is answered");
    valid(&schemas(), "issue_orders.schema.json", &answer, "orders");
    answer
}

fn one(session: &mut Session, unit: &str, order: Value) -> Value {
    issue(session, json!([{ "unit": unit, "order": order }]))["results"][0].clone()
}

fn end_day(session: &mut Session) {
    session
        .call(Side::Allied, "end_day", &json!({}))
        .expect("end_day is answered");
}

fn briefing(session: &mut Session) -> (String, Value) {
    let text = session
        .call(Side::Allied, "get_briefing", &json!({ "text": true }))
        .expect("the briefing");
    let data = session
        .call(Side::Allied, "get_briefing", &json!({ "text": false }))
        .expect("the briefing");
    valid(&schemas(), "briefing.schema.json", &data, "briefing");
    (text["text"].as_str().expect("text").to_string(), data)
}

fn refused_with(result: &Value, words: &str) {
    assert_eq!(result["status"], "refused", "{result}");
    let reason = result["reason"].as_str().expect("a reason");
    assert!(reason.contains(words), "{reason}");
}

#[test]
fn the_briefing_and_the_lists_show_reserves_and_ships_in_port() {
    let mut session = session();
    let (text, data) = briefing(&mut session);
    let s = &data["bases"][0];
    assert_eq!(s["reserves"]["supply_reserve_t"], "400");
    assert_eq!(s["reserves"]["fuel_reserve_t"], "300");
    // Base 2 keeps no reserve: the part is left out.
    let t = &data["bases"][1];
    assert!(t.get("reserves").is_none(), "{t}");
    assert!(s.get("ships_in_port").is_none());
    assert_eq!(t["ships_in_port"][0]["handle"], "ship:5");
    assert_eq!(t["ships_in_port"][1]["handle"], "ship:6");
    assert!(
        text.contains("Reserves: 400 t supply, 300 t fuel (refuelling and the staff's loads"),
        "{text}"
    );
    assert!(
        text.contains("In port in no force: ship:5") && text.contains("form_task_force"),
        "{text}"
    );

    let bases = session
        .call(Side::Allied, "list_bases", &json!({}))
        .expect("list_bases");
    valid(&schemas(), "list_bases.schema.json", &bases, "list_bases");
    assert_eq!(bases["bases"][0]["reserves"]["fuel_reserve_t"], "300");
    let entity = session
        .call(Side::Allied, "get_entity", &json!({ "handle": "base:2" }))
        .expect("get_entity");
    valid(&schemas(), "entity.schema.json", &entity, "base entity");
    let ships: Vec<&str> = entity["entity"]["ships_in_port"]
        .as_array()
        .expect("ships in port")
        .iter()
        .map(|h| h["handle"].as_str().expect("handle"))
        .collect();
    assert_eq!(ships, ["ship:5", "ship:6"]);
}

#[test]
fn a_load_with_a_weight_is_queued_and_says_what_it_may_take_below_the_reserve() {
    let mut session = session();
    // 250 t is within the 600 t above the reserve: queued, nothing said.
    let result = one(
        &mut session,
        "task_force:1",
        json!({ "kind": "load", "weight_t": "250" }),
    );
    assert_eq!(result["status"], "queued", "{result}");
    assert!(result.get("below_reserve").is_none(), "{result}");
    // 700 t is 100 t more than the stock above the reserve.
    let result = one(
        &mut session,
        "task_force:1",
        json!({ "kind": "load", "cargo": "supply", "weight_t": "700" }),
    );
    assert_eq!(result["status"], "queued", "{result}");
    let below = &result["below_reserve"];
    assert_eq!(below["tons"], "100", "{result}");
    assert_eq!(below["reserve_t"], "400");
    assert_eq!(below["cargo"], "supply");
    assert_eq!(below["base"], "base:1");
    assert!(
        below["text"]
            .as_str()
            .expect("text")
            .contains("load of supply may take 100 t below base:1"),
        "{result}"
    );
    // A load with no weight fills the room (3000 t): 1000 - 400 = 600 above
    // the reserve, so 400 t may go below it (the stock is 1000 t).
    let result = one(&mut session, "task_force:1", json!({ "kind": "load" }));
    assert_eq!(result["below_reserve"]["tons"], "400", "{result}");
    // Fuel at S: 500 t, 200 t above the reserve, 1000 t of room in the cargo.
    let result = one(
        &mut session,
        "task_force:1",
        json!({ "kind": "load", "cargo": "fuel" }),
    );
    assert_eq!(result["below_reserve"]["tons"], "300", "{result}");
}

#[test]
fn a_weight_that_is_not_above_zero_or_goes_with_a_unit_is_refused() {
    let mut session = session();
    let result = one(
        &mut session,
        "task_force:1",
        json!({ "kind": "load", "weight_t": "0" }),
    );
    refused_with(&result, "weight 0 t is not above 0");
    assert!(
        result["reason"]
            .as_str()
            .unwrap()
            .contains("weight_t names"),
        "{result}"
    );
    let result = one(
        &mut session,
        "task_force:1",
        json!({ "kind": "load", "unit": "land_unit:1", "weight_t": "10" }),
    );
    refused_with(&result, "malformed order");
    let result = one(
        &mut session,
        "task_force:1",
        json!({ "kind": "load", "cargo": "none", "weight_t": "10" }),
    );
    refused_with(&result, "malformed order");
    let result = one(
        &mut session,
        "task_force:1",
        json!({ "kind": "load", "weight_t": 10 }),
    );
    refused_with(&result, "malformed order");
}

#[test]
fn forming_is_refused_as_the_engine_refuses_it() {
    let mut session = session();
    refused_with(
        &one(
            &mut session,
            "base:2",
            json!({ "kind": "form_task_force", "ships": [] }),
        ),
        "no ships are listed",
    );
    refused_with(
        &one(
            &mut session,
            "base:2",
            json!({ "kind": "form_task_force", "ships": [5, 5] }),
        ),
        "ship:5 is listed twice",
    );
    // Ship 1 is in a force; ship 5 lies at the other base.
    refused_with(
        &one(
            &mut session,
            "base:2",
            json!({ "kind": "form_task_force", "ships": [1] }),
        ),
        "ship:1 is not one of your ships in port here in no force",
    );
    refused_with(
        &one(
            &mut session,
            "base:1",
            json!({ "kind": "form_task_force", "ships": [5] }),
        ),
        "ship:5 is not one of your ships in port here in no force",
    );
    // A ship that is none of the side's is refused with the number written.
    refused_with(
        &one(
            &mut session,
            "base:2",
            json!({ "kind": "form_task_force", "ships": [99] }),
        ),
        "ship:99 is not one of your ships",
    );
    // A base that is not the side's or does not exist.
    let result = one(
        &mut session,
        "base:99",
        json!({ "kind": "form_task_force", "ships": [5] }),
    );
    assert_eq!(result["status"], "refused", "{result}");
    refused_with(
        &one(
            &mut session,
            "base:2",
            json!({ "kind": "form_task_force", "ships": "5" }),
        ),
        "malformed order",
    );
    let result = one(
        &mut session,
        "base:2",
        json!({ "kind": "form_task_force", "ships": [5, 6] }),
    );
    assert_eq!(result["status"], "queued", "{result}");
    // Only a forming order is a base's.
    refused_with(
        &one(&mut session, "base:2", json!({ "kind": "stand_down" })),
        "malformed order",
    );
}

#[test]
fn a_join_and_a_stand_down_are_refused_as_the_engine_refuses_them() {
    let mut session = session();
    // Forces 1 and 2 are both stopped at base 1; force 3 is at base 2.
    let result = one(
        &mut session,
        "task_force:1",
        json!({ "kind": "join", "into": "task_force:2" }),
    );
    assert_eq!(result["status"], "queued", "{result}");
    refused_with(
        &one(
            &mut session,
            "task_force:1",
            json!({ "kind": "join", "into": "task_force:1" }),
        ),
        "a task force cannot join itself",
    );
    refused_with(
        &one(
            &mut session,
            "task_force:1",
            json!({ "kind": "join", "into": "task_force:3" }),
        ),
        "task_force:3 is not stopped at the same base",
    );
    // A force that is none of the side's, or does not exist, is not known.
    for into in ["task_force:99", "task_force:0"] {
        let result = one(
            &mut session,
            "task_force:1",
            json!({ "kind": "join", "into": into }),
        );
        assert_eq!(result["status"], "refused", "{result}");
        assert_eq!(result["reason"], "not known", "{result}");
    }
    refused_with(
        &one(
            &mut session,
            "task_force:1",
            json!({ "kind": "join", "into": "ship:2" }),
        ),
        "malformed order",
    );
    let result = one(
        &mut session,
        "task_force:3",
        json!({ "kind": "stand_down" }),
    );
    assert_eq!(result["status"], "queued", "{result}");
    // A force under way is refused: a route order does not make it so until
    // the day, but a force at sea would be; here, one not of the side.
    let result = one(
        &mut session,
        "task_force:99",
        json!({ "kind": "stand_down" }),
    );
    assert_eq!(result["reason"], "not known", "{result}");
}

#[test]
fn the_three_force_orders_play_through_and_the_briefing_words_them() {
    let mut session = session();
    // Day 1: ships 5 and 6 formed; force 1 and 2 are refuelled and loaded.
    let result = one(
        &mut session,
        "base:2",
        json!({ "kind": "form_task_force", "ships": [5, 6] }),
    );
    assert_eq!(result["status"], "queued", "{result}");
    end_day(&mut session);

    let (text, data) = briefing(&mut session);
    let changes = &data["force_changes"];
    let formed = &changes["formed"][0];
    assert_eq!(formed["task_force"]["handle"], "task_force:4", "{data}");
    assert_eq!(formed["base"]["handle"], "base:2");
    assert_eq!(formed["ships"][0]["handle"], "ship:5");
    assert_eq!(formed["ships"][1]["handle"], "ship:6");
    assert!(text.contains("FORCE CHANGES"), "{text}");
    assert!(
        text.contains("Formed yesterday: task_force:4") && text.contains("at base:2")
            || text.contains("Formed yesterday: task_force:4 at (10,0)"),
        "{text}"
    );
    // Day 1's refuelling at S: ship 2 is 50 t short of full.
    let held = &changes["refuelling_held_at_reserve"][0];
    assert_eq!(held["ship"]["handle"], "ship:2", "{changes}");
    assert_eq!(held["short_t"], "50");
    assert!(
        text.contains("Refuelling held at the reserve: ship:2")
            && text.contains("is 50 t short of full: the fuel at"),
        "{text}"
    );
    // Ships 5 and 6 are in a force now: no longer in port in no force.
    assert!(data["bases"][1].get("ships_in_port").is_none());

    // The new force is listed; an order to it is taken.
    let forces = session
        .call(Side::Allied, "list_forces", &json!({}))
        .expect("list_forces");
    valid(
        &schemas(),
        "list_forces.schema.json",
        &forces,
        "list_forces",
    );
    let handles: Vec<&str> = forces["task_forces"]
        .as_array()
        .expect("forces")
        .iter()
        .map(|f| f["task_force"]["handle"].as_str().expect("handle"))
        .collect();
    assert_eq!(
        handles,
        [
            "task_force:1",
            "task_force:2",
            "task_force:3",
            "task_force:4"
        ]
    );

    // Day 2: force 3 joins force 4.
    let result = one(
        &mut session,
        "task_force:3",
        json!({ "kind": "join", "into": "task_force:4" }),
    );
    assert_eq!(result["status"], "queued", "{result}");
    end_day(&mut session);
    let (text, data) = briefing(&mut session);
    let joined = &data["force_changes"]["joined"][0];
    assert_eq!(joined["joined"]["handle"], "task_force:3", "{data}");
    assert_eq!(joined["into"]["handle"], "task_force:4");
    assert_eq!(joined["ships"][0]["handle"], "ship:3");
    assert!(
        text.contains("Joined yesterday: task_force:3") && text.contains("has ceased to be"),
        "{text}"
    );
    // Force 3 is no longer listed, keeps its number and takes no order.
    let forces = session
        .call(Side::Allied, "list_forces", &json!({}))
        .expect("list_forces");
    let handles: Vec<&str> = forces["task_forces"]
        .as_array()
        .expect("forces")
        .iter()
        .map(|f| f["task_force"]["handle"].as_str().expect("handle"))
        .collect();
    assert_eq!(handles, ["task_force:1", "task_force:2", "task_force:4"]);
    let result = one(&mut session, "task_force:3", json!({ "kind": "unload" }));
    refused_with(&result, "it was joined to task_force:4");
    assert!(
        result["reason"]
            .as_str()
            .unwrap()
            .contains("has ceased to be"),
        "{result}"
    );

    // Day 3: force 4 is stood down.
    let result = one(
        &mut session,
        "task_force:4",
        json!({ "kind": "stand_down" }),
    );
    assert_eq!(result["status"], "queued", "{result}");
    end_day(&mut session);
    let (text, data) = briefing(&mut session);
    let stood = &data["force_changes"]["stood_down"][0];
    assert_eq!(stood["task_force"]["handle"], "task_force:4", "{data}");
    assert_eq!(stood["ships"].as_array().expect("ships").len(), 4);
    assert!(
        text.contains("Stood down yesterday: task_force:4")
            && text.contains("lie in port in no force"),
        "{text}"
    );
    let ships: Vec<&str> = data["bases"][1]["ships_in_port"]
        .as_array()
        .expect("ships in port")
        .iter()
        .map(|h| h["handle"].as_str().expect("handle"))
        .collect();
    assert_eq!(ships, ["ship:3", "ship:4", "ship:5", "ship:6"]);
    let result = one(&mut session, "task_force:4", json!({ "kind": "unload" }));
    refused_with(&result, "it was stood down in port");

    // explain works on the new events, in the side's own numbers.
    let number = data["force_changes"]["stood_down"][0]["event"]
        .as_u64()
        .expect("an event number");
    let explained = session
        .call(Side::Allied, "explain", &json!({ "event": number }))
        .expect("explain");
    valid(&schemas(), "explain.schema.json", &explained, "explain");
    assert!(
        explained["text"]
            .as_str()
            .unwrap_or_default()
            .contains("stood down")
            || explained.to_string().contains("stood down"),
        "{explained}"
    );
}
