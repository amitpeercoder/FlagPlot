//! Decision 0219 (an order may be given at any stop, and acts when it can):
//! the eight statements of the record, played through a session.
//!
//! No number here is an expected figure. Each test compares two ways of
//! playing the same game (the daily hashes and the events), or reads a fact
//! back from the game's own answers. The games are on the live figures,
//! scenario 78 (scenario 1's transit with orders inside a day on) with its
//! own order inside a day taken out, as `tests/advance.rs` has it.

use crate::support;

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde_json::{json, Value};
use sim_agent::Session;
use sim_core::orders::OrderRequest;
use sim_core::state::Side;
use sim_data::Scenario;
use support::play::{script_of, session_of};
use support::ROOT;

const ALLIED: Side = Side::Allied;

fn live_path(name: &str) -> PathBuf {
    PathBuf::from(ROOT).join("data/scenarios").join(name)
}

fn live_text(name: &str) -> String {
    std::fs::read_to_string(live_path(name)).expect("the scenario reads")
}

fn scenario(name: &str, text: &str) -> Scenario {
    Scenario::from_text(text, &live_path(name)).expect("the scenario loads")
}

/// Scenario 78 with its own order inside a day taken out.
fn case_c_text() -> String {
    let text = live_text("78_orders_inside_a_day.toml");
    let cut = text.find("# Day 1, at 0600").expect("the order is marked");
    let rest = &text[cut..];
    let end = rest.find("# Days 2 and 3").expect("the file goes on");
    format!("{}{}", &text[..cut], &rest[end..])
}

fn session(scenario: Scenario, player: Side) -> (Session, BTreeMap<u32, Vec<OrderRequest>>) {
    let commands = scenario.scripted_commands().clone();
    let inside_on = scenario.orders_inside_a_day();
    let inside = scenario.scripted_orders_inside_day().clone();
    let script = script_of(&scenario, player);
    let (session, _) = session_of(scenario, &[player]);
    let session = session
        .with_staffs(sim_staff::staffs())
        .with_commands(commands)
        .with_orders_inside_a_day(inside_on, inside);
    (session, script)
}

fn call(s: &mut Session, tool: &str, args: Value) -> Value {
    s.call(ALLIED, tool, &args)
        .unwrap_or_else(|e| panic!("{tool} {args} refused: {e}"))
}

fn check(file: &str, answer: &Value) {
    let found = support::schemas().check(file, answer);
    assert!(found.is_empty(), "{file}: {found:?}\n{answer}");
}

fn issue(s: &mut Session, orders: Value) -> Value {
    let answer = call(s, "issue_orders", json!({ "orders": orders }));
    check("issue_orders.schema.json", &answer);
    answer
}

fn route_and_cruise() -> Value {
    json!([
        {"unit": "task_force:1", "order": {"kind": "set_route", "waypoints": [[8, 3]]}},
        {"unit": "task_force:1", "order": {"kind": "set_speed", "speed": "cruise"}}
    ])
}

fn mission(name: &str) -> Value {
    json!({"unit": "task_force:1", "order": {"kind": "set_mission", "mission": name}})
}

/// A session at the first dusk with the route and speed given, advanced to
/// the stop at 0600 (`pulses` = 1) or 1200 (2).
fn at_stop(text: &str, pulses: u32) -> Session {
    let (mut s, _) = session(scenario("78_orders_inside_a_day.toml", text), ALLIED);
    issue(&mut s, route_and_cruise());
    call(&mut s, "advance", json!({"pulses": pulses}));
    s
}

fn finish(s: &mut Session) -> Vec<u64> {
    let rest = call(s, "advance", json!({"pulses": 30}));
    assert_eq!(rest["stopped"], "game_over", "{rest}");
    s.day_hashes().to_vec()
}

fn day_events(s: &mut Session, day: u64) -> Value {
    call(s, "events", json!({"day": day}))["events"].clone()
}

/// The state hash at the dusk ending day 1, the dusk orders given and no
/// other, played as `advance` stops.
fn baseline_dusk_hash(text: &str) -> u64 {
    let mut s = at_stop(text, 2);
    call(&mut s, "advance", json!({"pulses": 1}));
    assert!(s.open_day().is_none());
    s.state_hash()
}

/// Statement 1: at 1200 a mission order is kept, not refused; the afternoon
/// is as if it had not been given.
#[test]
fn statement_1_a_mission_order_at_1200_is_kept_and_changes_nothing_that_afternoon() {
    let text = case_c_text();
    let base = baseline_dusk_hash(&text);
    let mut s = at_stop(&text, 2);
    let given = issue(&mut s, json!([mission("surface_combat")]));
    let r = &given["results"][0];
    assert_eq!(r["status"], "queued", "{given}");
    assert_eq!(r["kept"], true, "{given}");
    let acts = r["acts"].as_str().unwrap();
    assert!(acts.contains("1800") && acts.contains("day 2"), "{given}");
    assert!(r.get("for").is_none(), "{given}");
    assert_eq!(given["kept_in_call"], 1);
    assert_eq!(given["kept"], 1);
    assert_eq!(given["queued"], 0, "no order is for the pulse: {given}");
    assert!(
        given["summary"]
            .as_str()
            .unwrap()
            .contains("1 kept for 1800"),
        "{given}"
    );
    call(&mut s, "advance", json!({"pulses": 1}));
    assert!(s.open_day().is_none());
    // The state at dusk is the state of the game that was given nothing.
    assert_eq!(s.state_hash(), base);
    // The kept order is still waiting for that dusk.
    let (dusk, pulse, _) = s.queue_of(ALLIED);
    assert_eq!((dusk.len(), pulse.len()), (1, 0));
}

/// Statement 2: handed in at the next dusk, accepted, and the force has the
/// mission for the coming day; the game from there is the game in which it
/// was given at that dusk.
#[test]
fn statement_2_the_kept_order_is_handed_in_at_dusk_as_if_given_then() {
    let text = case_c_text();
    // Kept at 1200.
    let mut kept = at_stop(&text, 2);
    issue(&mut kept, json!([mission("surface_combat")]));
    call(&mut kept, "advance", json!({"pulses": 1}));
    // Given at that dusk.
    let mut late = at_stop(&text, 2);
    call(&mut late, "advance", json!({"pulses": 1}));
    let answer = issue(&mut late, json!([mission("surface_combat")]));
    assert_eq!(answer["results"][0]["status"], "queued");
    assert!(answer["results"][0].get("kept").is_none(), "{answer}");
    let (a, b) = (finish(&mut kept), finish(&mut late));
    assert_eq!(a, b, "the daily hashes are the same");
    assert_eq!(
        kept.handed_in(),
        late.handed_in(),
        "the saved game is the same"
    );
    assert!(
        kept.handed_in()[&2]
            .iter()
            .any(|(side, r)| *side == ALLIED && format!("{:?}", r.kind).contains("SetMission")),
        "the order is one of day 2's"
    );
    for day in [2, 3] {
        assert_eq!(day_events(&mut kept, day), day_events(&mut late, day));
    }
    // And it is a game in which the order did something: the game with
    // neither differs.
    let mut none = at_stop(&text, 2);
    assert_ne!(finish(&mut none), a);
}

/// Statement 3: a route at 1200 acts at the start of the afternoon.
#[test]
fn statement_3_a_route_at_1200_acts_in_the_afternoon_as_before() {
    let text = case_c_text();
    let base = baseline_dusk_hash(&text);
    let mut s = at_stop(&text, 2);
    let given = issue(
        &mut s,
        json!([{"unit": "task_force:1", "order": {"kind": "set_route", "waypoints": [[4, 1]]}}]),
    );
    let r = &given["results"][0];
    assert_eq!(r["status"], "queued", "{given}");
    assert_eq!(r["for"], "the afternoon pulse of day 1", "{given}");
    assert!(
        r.get("kept").is_none() && r.get("acts").is_none(),
        "{given}"
    );
    assert_eq!(given["kept_in_call"], 0);
    assert_eq!(given["queued"], 1);
    let (dusk, pulse, _) = s.queue_of(ALLIED);
    assert_eq!((dusk.len(), pulse.len()), (0, 1));
    call(&mut s, "advance", json!({"pulses": 1}));
    assert_ne!(
        s.state_hash(),
        base,
        "the route moved the force that afternoon"
    );
}

/// Statement 4: an order kept at 0600 and one kept at 1200 are handed in in
/// the order given.
#[test]
fn statement_4_kept_orders_are_handed_in_in_the_order_given() {
    let text = case_c_text();
    let mut s = at_stop(&text, 1);
    issue(&mut s, json!([mission("surface_combat")]));
    call(&mut s, "advance", json!({"pulses": 1}));
    let second = issue(&mut s, json!([mission("bombardment")]));
    assert_eq!(second["kept"], 2, "{second}");
    call(&mut s, "advance", json!({"pulses": 1}));
    // Reverse order: the other mission is the later one.
    let mut r = at_stop(&text, 1);
    issue(&mut r, json!([mission("bombardment")]));
    call(&mut r, "advance", json!({"pulses": 1}));
    issue(&mut r, json!([mission("surface_combat")]));
    call(&mut r, "advance", json!({"pulses": 1}));
    // Handing in is at the next dusk's start of day 2.
    let (a, b) = (finish(&mut s), finish(&mut r));
    let kinds = |s: &Session| -> Vec<String> {
        s.handed_in()[&2]
            .iter()
            .filter(|(_, r)| format!("{:?}", r.kind).contains("SetMission"))
            .map(|(_, r)| format!("{:?}", r.kind))
            .collect()
    };
    let (ks, kr) = (kinds(&s), kinds(&r));
    assert_eq!(ks.len(), 2);
    assert!(
        ks[0].contains("SurfaceCombat") && ks[1].contains("Bombardment"),
        "{ks:?}"
    );
    assert!(
        kr[0].contains("Bombardment") && kr[1].contains("SurfaceCombat"),
        "{kr:?}"
    );
    assert_ne!(a, b, "the later order is the one that stands");
}

/// Statement 6: an order for a unit of the other side given at 1200 is
/// refused at once.
#[test]
fn statement_6_an_order_for_the_other_sides_unit_is_refused_at_once() {
    let text = case_c_text();
    let mut s = at_stop(&text, 2);
    let given = issue(
        &mut s,
        json!([
            {"unit": "task_force:90001", "order": {"kind": "set_mission", "mission": "transport"}},
            {"unit": "task_force:90001", "order": {"kind": "set_speed", "speed": "full"}}
        ]),
    );
    for i in 0..2 {
        assert_eq!(given["results"][i]["status"], "refused", "{given}");
    }
    assert_eq!(given["kept"], 0);
    let (dusk, pulse, _) = s.queue_of(ALLIED);
    assert_eq!((dusk.len(), pulse.len()), (0, 0));
}

/// Statement 7: clearing at 1200 removes the kept order, and nothing is
/// handed in for it.
#[test]
fn statement_7_clearing_at_1200_removes_the_kept_order() {
    let text = case_c_text();
    let mut s = at_stop(&text, 2);
    issue(&mut s, json!([mission("surface_combat")]));
    let cleared = call(&mut s, "issue_orders", json!({"orders": [], "clear": true}));
    check("issue_orders.schema.json", &cleared);
    assert_eq!(cleared["kept"], 0, "{cleared}");
    let (dusk, pulse, _) = s.queue_of(ALLIED);
    assert_eq!((dusk.len(), pulse.len()), (0, 0));
    let a = finish(&mut s);
    assert!(s
        .handed_in()
        .get(&2)
        .into_iter()
        .flatten()
        .all(|(_, r)| !format!("{:?}", r.kind).contains("SetMission")));
    let mut none = at_stop(&text, 2);
    assert_eq!(finish(&mut none), a);
}

/// Statement 8: with the key off an order between days is as before and
/// nothing is kept.
#[test]
fn statement_8_without_the_key_nothing_changes() {
    let (mut s, _) = session(
        scenario("01_transit.toml", &live_text("01_transit.toml")),
        ALLIED,
    );
    assert!(!s.is_over());
    let given = issue(&mut s, json!([mission("surface_combat")]));
    let r = &given["results"][0];
    assert_eq!(r["status"], "queued", "{given}");
    assert!(r.get("kept").is_none() && r.get("acts").is_none() && r.get("for").is_none());
    assert!(given.get("kept").is_none() && given.get("kept_in_call").is_none());
    assert!(
        !given["summary"].as_str().unwrap().contains("kept"),
        "{given}"
    );
    assert_eq!(given["queued"], 1);
    // And the clock cannot be stopped inside a day.
    assert!(s.call(ALLIED, "advance", &json!({"pulses": 1})).is_err());
}

/// Statement 5, on a stand-in for a lost unit. No scenario on file loses a
/// whole task force in an afternoon (a search of every scenario with the key
/// on, both sides, found none), so this uses a unit that is gone by the dusk
/// in the other way a force can be: in port at Espiritu Santo on day 2, a
/// stand down is kept at 0600 and an order for the same force at 1200. At the
/// dusk the stand down is handed in first and the force ceases to be; the
/// second order is refused when handed in, on the state as it then stands,
/// with the engine's reason for a force that no longer exists. It is not the
/// reason for a force whose ships are sunk ("no ship afloat"): that case has
/// no scenario to play and is not proved here.
#[test]
fn statement_5_a_kept_order_for_a_unit_gone_by_dusk_is_refused_when_handed_in() {
    let text = case_c_text();
    let mut s = at_stop(&text, 1);
    // To 0600 of day 2: the force is in port at Espiritu Santo.
    call(&mut s, "advance", json!({"pulses": 3}));
    assert_eq!(s.open_day().map(|(d, _)| d), Some(2));
    let first = issue(
        &mut s,
        json!([{"unit": "task_force:1", "order": {"kind": "stand_down"}}]),
    );
    assert_eq!(first["results"][0]["kept"], true, "{first}");
    call(&mut s, "advance", json!({"pulses": 1}));
    // At 1200 the force still has its ships, so the order is kept.
    let second = issue(&mut s, json!([mission("surface_combat")]));
    assert_eq!(second["results"][0]["kept"], true, "{second}");
    assert_eq!(second["kept"], 2);
    // To the dusk and the day 3 that hands both in.
    call(&mut s, "advance", json!({"pulses": 2}));
    let events = day_events(&mut s, 3);
    let text: Vec<String> = events
        .as_array()
        .into_iter()
        .flatten()
        .map(|e| format!("{} {}", e["event_kind"], e["text"]))
        .collect();
    assert!(
        text.iter()
            .any(|t| t.contains("order_accepted") && t.contains("stand_down")),
        "{text:#?}"
    );
    let refusal = text
        .iter()
        .find(|t| t.contains("order_rejected") || t.contains("order_refused"))
        .unwrap_or_else(|| panic!("the second order is refused: {text:#?}"));
    assert!(refusal.contains("no longer exists"), "{refusal}");
}
