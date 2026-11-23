//! A queued order names a unit by the side's own number (decision 0225's
//! ruling 4, on pull request 706).
//!
//! The fault, found at the table: `queued` named a submarine by the engine's
//! number where every other answer gives the side's own, so the Japanese
//! seat's one boat, I-168, was `submarine:1` everywhere and `submarine:5` in
//! `queued`, and the side could count the four enemy boats before her.
//!
//! The game is a small one of this file's own, on the placeholder map of two
//! airfields: each side has a base, a task force of a carrier and a
//! destroyer, an air group on the carrier, land units ashore and submarines,
//! the Allied side four boats and the Japanese one. Its ids interleave the
//! sides, so that the engine's number and the side's own differ for every
//! kind of unit:
//!
//! | kind       | Allied ids → side numbers | Japanese ids → side numbers |
//! | ---------- | ------------------------- | --------------------------- |
//! | ship       | 2, 4 → 1, 2               | 3, 5 → 1, 2                 |
//! | task force | 2 → 1                     | 3 → 1                       |
//! | air group  | 2 → 1                     | 3 → 1                       |
//! | land unit  | 2, 4 → 1, 2               | 3 → 1                       |
//! | submarine  | 2, 3, 4, 6 → 1, 2, 3, 4   | 5 → 1                       |
//!
//! Bases keep their numbers (decision 0086): they are geography, known to
//! both sides.
//!
//! The architect's statements, one test each (written before the code, not
//! to be changed by anyone else):
//!
//! 1. A station order the Japanese seat queues for its one boat is named in
//!    `queued` as `submarine:1`, the number every other tool gives her.
//! 2. No answer of `queued`, `take_back`, `command_state`, `list_forces`,
//!    `staff_orders`, `why`, `preview` or `events` to the Japanese seat holds
//!    the handle `submarine:5` or any submarine handle above `submarine:1`.
//! 3. The same holds for the Allied seat with its four boats: its handles are
//!    `submarine:1` to `submarine:4` in every answer, and a queued order for
//!    its third boat is `submarine:3`.
//! 4. A queued order for a ship, a task force and an air group of each side
//!    is named in `queued` by the same handle `list_forces` gives it.
//! 5. `take_back` of a queued station order takes back the order of the boat
//!    the seat named, and no other.
//!
//! Statement 4 has no test here yet: `list_forces` gives no handle to a ship
//! or an air group (its lines are task forces, land units, submarines and
//! units to enter), so the statement cannot be held as written. It is with
//! the architect.
//!
//! The team lead's own test, beside the architect's: the same for a land
//! unit, in an order to it, a hold on it and a limit that waits for it.

mod support;

use std::collections::BTreeSet;

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::state::Side;
use support::{load_text, schemas};

const NAME: &str = "queued_numbers.toml";

const GAME: &str = r#"
name = "A queued order names a unit by the side's own number"
placeholder = true
seed = 1942
days = 3
map = "../map/placeholder_two_airfields.toml"
ship_classes = "../reference/ship_classes.csv"
port_sizes = "../reference/port_sizes.csv"
aircraft_types = "../reference/aircraft_types.csv"
search = "../reference/search.toml"
surface_combat = "../reference/surface_combat.toml"
air_combat = "../reference/air_combat.toml"
ground = "../reference/ground.toml"
submarine_classes = "../reference/submarine_classes_placeholder_scenario_128.csv"
submarine_torpedoes = "../reference/submarine_torpedoes_placeholder_scenario_128.csv"
submarines = "../reference/submarines_placeholder_scenario_128.toml"

[[base]]
id = 1
name = "Placeholder Allied base"
side = "allied"
q = 0
r = 0
supply_t = 1000

[[base]]
id = 2
name = "Placeholder Japanese base"
side = "japanese"
q = 7
r = 0
supply_t = 1000

[[task_force]]
id = 2
name = "Placeholder Allied force"
side = "allied"
at_base = 1
mission = "carrier"

[[task_force]]
id = 3
name = "Placeholder Japanese force"
side = "japanese"
at_base = 2
mission = "carrier"

[[ship]]
id = 2
name = "Placeholder Allied carrier"
class = 4
side = "allied"
fuel_t = 3000
task_force = 2

[[ship]]
id = 3
name = "Placeholder Japanese carrier"
class = 4
side = "japanese"
fuel_t = 3000
task_force = 3

[[ship]]
id = 4
name = "Placeholder Allied destroyer"
class = 2
side = "allied"
fuel_t = 500
task_force = 2

[[ship]]
id = 5
name = "Placeholder Japanese destroyer"
class = 2
side = "japanese"
fuel_t = 500
task_force = 3

[[air_group]]
id = 2
name = "Placeholder Allied dive bombers"
side = "allied"
ship = 2
aircraft_type = 3
ready = 18

[[air_group]]
id = 3
name = "Placeholder Japanese dive bombers"
side = "japanese"
ship = 3
aircraft_type = 3
ready = 18

[[land_unit]]
id = 2
name = "Placeholder Allied battalion 1"
side = "allied"
men = 600
q = 0
r = 0

[[land_unit]]
id = 3
name = "Placeholder Japanese battalion"
side = "japanese"
men = 600
q = 7
r = 0

[[land_unit]]
id = 4
name = "Placeholder Allied battalion 2"
side = "allied"
men = 400
q = 0
r = 0

[[submarine]]
id = 2
name = "Placeholder Allied boat 1"
side = "allied"
class = 6900
q = 0
r = 0
home = 1

[[submarine]]
id = 3
name = "Placeholder Allied boat 2"
side = "allied"
class = 6900
q = 0
r = 0
home = 1

[[submarine]]
id = 4
name = "Placeholder Allied boat 3"
side = "allied"
class = 6900
q = 0
r = 0
home = 1

[[submarine]]
id = 5
name = "Placeholder Japanese boat"
side = "japanese"
class = 6900
q = 7
r = 0
home = 2

[[submarine]]
id = 6
name = "Placeholder Allied boat 4"
side = "allied"
class = 6900
q = 0
r = 0
home = 1
"#;

fn game() -> Session {
    let scenario = load_text(GAME, NAME, 1942);
    Session::new(
        scenario.world,
        "Queued numbers",
        scenario.days,
        Seat::Player,
        Seat::Player,
    )
    .with_staffs(sim_staff::staffs())
}

/// A call answered, its answer held to its schema.
fn call(s: &mut Session, side: Side, tool: &str, args: Value) -> Value {
    let answer = s
        .call(side, tool, &args)
        .unwrap_or_else(|e| panic!("{tool} {args}: {e}"));
    let kind = answer["kind"].as_str().expect("an answer has a kind");
    let found = schemas().check(&format!("{kind}.schema.json"), &answer);
    assert!(found.is_empty(), "{tool} {args}: {found:?}\n{answer}");
    answer
}

/// One order queued, and answered `queued`.
fn order(s: &mut Session, side: Side, unit: &str, body: Value) {
    let a = call(
        s,
        side,
        "issue_orders",
        json!({"orders": [{"unit": unit, "order": body}]}),
    );
    assert_eq!(a["results"][0]["status"], "queued", "{unit}: {a}");
}

fn queued(s: &mut Session, side: Side) -> Value {
    call(s, side, "queued", json!({}))
}

/// The queued entry that is an order to `unit`.
fn entry_to<'a>(queued: &'a Value, unit: &str) -> &'a Value {
    queued["entries"]
        .as_array()
        .expect("entries")
        .iter()
        .find(|e| e["what"] == "order" && e["unit"] == unit)
        .unwrap_or_else(|| panic!("no queued order to {unit}: {queued}"))
}

/// Every number by which a value names a submarine: each `submarine:N` and
/// `submarine N` in its text, each number under a `submarine` key and each
/// under a `submarines` key.
fn boat_numbers(value: &Value, out: &mut BTreeSet<u64>) {
    match value {
        Value::String(text) => {
            for token in ["submarine:", "submarine "] {
                let mut rest = text.as_str();
                while let Some(at) = rest.find(token) {
                    rest = &rest[at + token.len()..];
                    let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
                    if let Ok(n) = digits.parse() {
                        out.insert(n);
                    }
                }
            }
        }
        Value::Array(items) => items.iter().for_each(|v| boat_numbers(v, out)),
        Value::Object(map) => {
            for (key, v) in map {
                match (key.as_str(), v) {
                    ("submarine", Value::Number(n)) => {
                        out.insert(n.as_u64().expect("a whole number"));
                    }
                    ("submarines", Value::Array(ns)) => {
                        out.extend(ns.iter().filter_map(Value::as_u64));
                    }
                    _ => boat_numbers(v, out),
                }
            }
        }
        _ => {}
    }
}

/// The boats a side holds, by the side's numbers, in `list_forces`.
fn listed_boats(s: &mut Session, side: Side) -> Vec<String> {
    call(s, side, "list_forces", json!({}))["submarines"]
        .as_array()
        .expect("boats")
        .iter()
        .map(|b| {
            b["submarine"]["handle"]
                .as_str()
                .expect("a handle")
                .to_string()
        })
        .collect()
}

/// Every answer the eight tools give a seat over a day and a night of play:
/// each tool before the day is ended and after it is played, `why` of every
/// event it may see, and `take_back` of a queued entry.
fn every_answer(side: Side, boats: u32) -> Vec<(String, Value)> {
    let mut s = game();
    let mut answers: Vec<(String, Value)> = Vec::new();
    let mut ask = |s: &mut Session, label: &str, tool: &str, args: Value| {
        let a = call(s, side, tool, args);
        answers.push((format!("{label} {tool}"), a.clone()));
        a
    };
    ask(&mut s, "start", "list_forces", json!({}));
    ask(&mut s, "start", "command_state", json!({}));
    // A patrol with every boat, its limits, a hold on the last boat and a
    // station for each.
    let all: Vec<u32> = (1..=boats).collect();
    ask(
        &mut s,
        "start",
        "preview",
        json!({"command": "set_intent", "intent_kind": "patrol", "hex": [4, 0], "distance": 2,
               "limits": [{"limit": "forces", "submarines": all}]}),
    );
    ask(
        &mut s,
        "start",
        "set_intent",
        json!({"intent_kind": "patrol", "hex": [4, 0], "distance": 2}),
    );
    ask(
        &mut s,
        "start",
        "set_limit",
        json!({"limit": "forces", "intent": 1, "submarines": all}),
    );
    // A take of the enemy's base, told to wait for the side's first land
    // unit.
    let enemy_base = if side == Side::Allied { 2 } else { 1 };
    ask(
        &mut s,
        "start",
        "set_intent",
        json!({"intent_kind": "take_place", "base": enemy_base}),
    );
    ask(
        &mut s,
        "start",
        "set_limit",
        json!({"limit": "wait_for", "intent": 2, "land_unit": 1}),
    );
    ask(&mut s, "start", "hold", json!({"submarine": boats}));
    for boat in 1..=boats {
        order(
            &mut s,
            side,
            &format!("submarine:{boat}"),
            json!({"kind": "station", "hex": [3 + boat as i32 % 3, 0]}),
        );
    }
    ask(
        &mut s,
        "start",
        "preview",
        json!({"command": "set_limit", "limit": "forces", "intent": 1, "submarines": [boats]}),
    );
    ask(&mut s, "queued", "queued", json!({}));
    ask(&mut s, "queued", "command_state", json!({}));
    ask(&mut s, "queued", "list_forces", json!({}));
    ask(&mut s, "queued", "staff_orders", json!({}));
    // Taking back the wait for, then the first boat's station.
    let q = queued(&mut s, side);
    let wait = q["entries"]
        .as_array()
        .unwrap()
        .iter()
        .find(|e| e["command"]["limit"] == "wait_for")
        .expect("the wait for is queued")["place"]
        .clone();
    ask(
        &mut s,
        "back",
        "take_back",
        json!({"queue": wait["queue"], "number": wait["number"], "as_of": q["as_of"]}),
    );
    let q = queued(&mut s, side);
    let first = entry_to(&q, "submarine:1")["place"].clone();
    ask(
        &mut s,
        "back",
        "take_back",
        json!({"queue": first["queue"], "number": first["number"], "as_of": q["as_of"]}),
    );
    // The day played, both seats ending it.
    for seat in [Side::Allied, Side::Japanese] {
        call(&mut s, seat, "end_day", json!({}));
    }
    let events = ask(&mut s, "played", "events", json!({"day": 1}));
    ask(&mut s, "played", "queued", json!({}));
    ask(&mut s, "played", "command_state", json!({}));
    ask(&mut s, "played", "list_forces", json!({}));
    ask(&mut s, "played", "staff_orders", json!({}));
    ask(
        &mut s,
        "played",
        "preview",
        json!({"command": "set_limit", "limit": "forces", "intent": 1, "submarines": all}),
    );
    let numbers: Vec<u64> = events["events"]
        .as_array()
        .expect("events")
        .iter()
        .filter_map(|e| e["number"].as_u64())
        .collect();
    assert!(!numbers.is_empty(), "the day has events: {events}");
    for number in numbers {
        ask(&mut s, "played", "why", json!({"number": number}));
    }
    answers
}

/// Statement 1.
#[test]
fn a_station_order_for_the_japanese_boat_is_queued_as_submarine_1() {
    let mut s = game();
    assert_eq!(listed_boats(&mut s, Side::Japanese), ["submarine:1"]);
    order(
        &mut s,
        Side::Japanese,
        "submarine:1",
        json!({"kind": "station", "hex": [5, 0]}),
    );
    let q = queued(&mut s, Side::Japanese);
    let entry = entry_to(&q, "submarine:1");
    assert!(
        entry["text"].as_str().unwrap().starts_with("submarine:1: "),
        "{entry}"
    );
    let mut named = BTreeSet::new();
    boat_numbers(&q, &mut named);
    assert_eq!(named, BTreeSet::from([1]), "{q}");
}

/// Statement 2.
#[test]
fn no_answer_to_the_japanese_seat_names_a_boat_above_submarine_1() {
    for (label, answer) in every_answer(Side::Japanese, 1) {
        let mut named = BTreeSet::new();
        boat_numbers(&answer, &mut named);
        assert!(
            named.iter().all(|n| *n == 1),
            "{label} names boats {named:?}: {answer}"
        );
    }
}

/// Statement 3.
#[test]
fn every_answer_to_the_allied_seat_names_its_boats_1_to_4() {
    let mut seen = BTreeSet::new();
    for (label, answer) in every_answer(Side::Allied, 4) {
        let mut named = BTreeSet::new();
        boat_numbers(&answer, &mut named);
        assert!(
            named.iter().all(|n| (1..=4).contains(n)),
            "{label} names boats {named:?}: {answer}"
        );
        seen.extend(named);
    }
    assert_eq!(seen, BTreeSet::from([1, 2, 3, 4]));
    let mut s = game();
    assert_eq!(
        listed_boats(&mut s, Side::Allied),
        ["submarine:1", "submarine:2", "submarine:3", "submarine:4"]
    );
    order(
        &mut s,
        Side::Allied,
        "submarine:3",
        json!({"kind": "station", "hex": [4, 0]}),
    );
    let q = queued(&mut s, Side::Allied);
    let entry = entry_to(&q, "submarine:3");
    assert!(
        entry["text"].as_str().unwrap().starts_with("submarine:3: "),
        "{entry}"
    );
}

/// Statement 5.
#[test]
fn take_back_of_a_station_takes_back_the_named_boats_order_and_no_other() {
    let mut s = game();
    for boat in 1..=4 {
        order(
            &mut s,
            Side::Allied,
            &format!("submarine:{boat}"),
            json!({"kind": "station", "hex": [boat + 1, 0]}),
        );
    }
    let q = queued(&mut s, Side::Allied);
    let place = entry_to(&q, "submarine:3")["place"].clone();
    let a = call(
        &mut s,
        Side::Allied,
        "take_back",
        json!({"queue": place["queue"], "number": place["number"], "as_of": q["as_of"]}),
    );
    let taken: Vec<&Value> = a["taken_back"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| &e["unit"])
        .collect();
    assert_eq!(taken, [&json!("submarine:3")], "{a}");
    assert_eq!(a["taken_back"][0]["order"]["hex"], json!([4, 0]), "{a}");
    let left: Vec<&Value> = a["queued"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| &e["unit"])
        .collect();
    assert_eq!(
        left,
        [
            &json!("submarine:1"),
            &json!("submarine:2"),
            &json!("submarine:4")
        ],
        "{a}"
    );
    // The Japanese seat's one boat, beside an order to its force.
    let mut s = game();
    order(
        &mut s,
        Side::Japanese,
        "task_force:1",
        json!({"kind": "set_speed", "speed": "cruise"}),
    );
    order(
        &mut s,
        Side::Japanese,
        "submarine:1",
        json!({"kind": "station", "hex": [5, 0]}),
    );
    let q = queued(&mut s, Side::Japanese);
    let place = entry_to(&q, "submarine:1")["place"].clone();
    let a = call(
        &mut s,
        Side::Japanese,
        "take_back",
        json!({"queue": place["queue"], "number": place["number"], "as_of": q["as_of"]}),
    );
    assert_eq!(a["taken_back"].as_array().unwrap().len(), 1, "{a}");
    assert_eq!(a["taken_back"][0]["unit"], "submarine:1", "{a}");
    assert!(a["summary"]
        .as_str()
        .unwrap()
        .starts_with("Taken back: submarine:1: "));
    assert_eq!(a["queued"].as_array().unwrap().len(), 1, "{a}");
    assert_eq!(a["queued"][0]["unit"], "task_force:1", "{a}");
}

/// The team lead's: a land unit is named in `queued` by the side's number in
/// an order to it, a hold on it and a limit that waits for it.
#[test]
fn a_queued_order_hold_and_wait_name_a_land_unit_as_list_forces_does() {
    for (side, units) in [(Side::Allied, 2u64), (Side::Japanese, 1)] {
        let mut s = game();
        let forces = call(&mut s, side, "list_forces", json!({}));
        let listed: Vec<&str> = forces["land_units"]
            .as_array()
            .unwrap_or_else(|| panic!("land units: {forces}"))
            .iter()
            .map(|u| u["land_unit"]["handle"].as_str().unwrap())
            .collect();
        let expected: Vec<String> = (1..=units).map(|n| format!("land_unit:{n}")).collect();
        assert_eq!(listed, expected, "{side:?}");
        let last = format!("land_unit:{units}");
        order(
            &mut s,
            side,
            &last,
            json!({"kind": "set_posture", "posture": "defend"}),
        );
        call(&mut s, side, "hold", json!({"land_unit": units}));
        let enemy_base = if side == Side::Allied { 2 } else { 1 };
        call(
            &mut s,
            side,
            "set_intent",
            json!({"intent_kind": "take_place", "base": enemy_base}),
        );
        // A unit the side does not have is refused in the number it wrote.
        let a = call(
            &mut s,
            side,
            "set_limit",
            json!({"limit": "wait_for", "intent": 1, "land_unit": 9}),
        );
        assert_eq!(a["status"], "refused", "{side:?}: {a}");
        assert_eq!(a["command"]["land_unit"], 9, "{side:?}: {a}");
        let a = call(
            &mut s,
            side,
            "set_limit",
            json!({"limit": "wait_for", "intent": 1, "land_unit": units}),
        );
        assert_eq!(a["status"], "queued", "{side:?}: {a}");
        assert_eq!(a["command"]["land_unit"], json!(units), "{side:?}: {a}");
        let q = queued(&mut s, side);
        let to_unit = entry_to(&q, &last);
        assert!(
            to_unit["text"]
                .as_str()
                .unwrap()
                .starts_with(&format!("{last}: ")),
            "{side:?}: {to_unit}"
        );
        let entries = q["entries"].as_array().unwrap();
        let hold = entries
            .iter()
            .find(|e| e["tool"] == "hold")
            .expect("the hold");
        assert_eq!(
            hold["command"]["land_unit"],
            json!(units),
            "{side:?}: {hold}"
        );
        let wait = entries
            .iter()
            .find(|e| e["command"]["limit"] == "wait_for")
            .expect("the wait for");
        assert_eq!(
            wait["command"]["land_unit"],
            json!(units),
            "{side:?}: {wait}"
        );
    }
}
