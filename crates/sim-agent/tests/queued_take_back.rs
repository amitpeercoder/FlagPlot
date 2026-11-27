//! The two session tools of decision 0221, "Taking back" (rules 15 to 18):
//! `queued`, which lists what a seat has queued with each entry's place, and
//! `take_back`, which withdraws one entry and what goes with it.
//!
//! No figure of the game is asserted: what is asserted is what the record
//! says, read back from the tools' own answers.
//!
//! The expected results, written from the record before the code:
//!
//! 1. `queued` lists the pulse queue, then the dusk queue, then the commands,
//!    each in the order given, each entry with its place (queue and number
//!    from 1), the unit and the order body (or the tool and the command) in
//!    the side's numbers, and when it acts: an order of dusk and a command
//!    "at dusk" of the day the orders are for; inside a day an order of the
//!    seven kinds "at pulse" of the open day, and an order kept for dusk "at
//!    dusk" of the day after the open day.
//! 2. `take_back` of one of two orders leaves the other, renumbered; a stale
//!    `as_of` is refused and changes nothing; a place that is none is refused.
//! 3. A hold queued for a unit goes with the last queued order to that unit
//!    when it is taken back; the hold says what it rests on; a hold taken
//!    back alone leaves the order.
//! 4. A `set_intent` that makes a new intent takes with it the limits
//!    queued for that intent.
//! 5. An order to the force a queued forming order makes rests on the
//!    forming order: taking the forming order back is refused, the words
//!    name the dependent entry and its place, and both stay queued.
//! 6. A preview made before a take back is void after it.
//! 7. Both tools are in the seat's list after `end_day`, and a watched game's
//!    seat that is ready is refused them as any call.

use crate::support;

use std::collections::BTreeMap;

use serde_json::{json, Value};
use sim_agent::{CallError, Called, OrdersInsideDay, Seat, Session};
use sim_core::state::Side;
use support::{load, schemas};

const ALLIED: Side = Side::Allied;

fn session_of(name: &str, with_commands_off: bool) -> Session {
    let mut scenario = load(name, 1942);
    if with_commands_off {
        scenario.replace_commands(BTreeMap::new());
    }
    let title = scenario.name.clone();
    Session::new(
        scenario.world,
        &title,
        scenario.days,
        Seat::Player,
        Seat::Script(BTreeMap::new()),
    )
    .with_staffs(sim_staff::staffs())
}

/// Scenario 78 (orders inside a day on), no orders of its own: the clock is
/// stopped at 0600 of day 1 with the route and speed of dusk given.
fn at_0600() -> Session {
    let scenario = load("78_orders_inside_a_day.toml", 1942);
    let title = scenario.name.clone();
    let mut s = Session::new(
        scenario.world,
        &title,
        scenario.days,
        Seat::Player,
        Seat::Script(BTreeMap::new()),
    )
    .with_staffs(sim_staff::staffs())
    .with_orders_inside_a_day(true, OrdersInsideDay::new());
    let mut p = Player::of(&mut s);
    p.ok(
        "issue_orders",
        json!({"orders": [
            {"unit": "task_force:1", "order": {"kind": "set_route", "waypoints": [[8, 3]]}},
            {"unit": "task_force:1", "order": {"kind": "set_speed", "speed": "cruise"}}
        ]}),
    );
    p.ok("advance", json!({"pulses": 1}));
    s
}

/// A call of the Allied seat with its answer held to its schema.
struct Player<'s> {
    session: &'s mut Session,
}

impl<'s> Player<'s> {
    fn of(session: &'s mut Session) -> Player<'s> {
        Player { session }
    }

    fn call(&mut self, tool: &str, args: Value) -> Result<Value, CallError> {
        let result = self.session.call(ALLIED, tool, &args);
        let answer = match &result {
            Ok(answer) => answer.clone(),
            Err(error) => error.to_json(),
        };
        let kind = answer["kind"].as_str().expect("an answer has a kind");
        let found = schemas().check(&format!("{kind}.schema.json"), &answer);
        assert!(found.is_empty(), "{tool} {args}: {found:?}\n{answer}");
        result
    }

    fn ok(&mut self, tool: &str, args: Value) -> Value {
        self.call(tool, args.clone())
            .unwrap_or_else(|e| panic!("{tool} {args}: {e}"))
    }

    fn err(&mut self, tool: &str, args: Value) -> String {
        match self.call(tool, args.clone()) {
            Err(e) => e.message,
            Ok(answer) => panic!("{tool} {args} was not refused: {answer}"),
        }
    }

    fn queued(&mut self) -> Value {
        self.ok("queued", json!({}))
    }

    /// `take_back` of the place, given the latest `as_of`.
    fn take_back(&mut self, queue: &str, number: u64) -> Result<Value, CallError> {
        let as_of = self.queued()["as_of"].as_str().unwrap().to_string();
        self.call(
            "take_back",
            json!({"queue": queue, "number": number, "as_of": as_of}),
        )
    }
}

fn place(queue: &str, number: u64) -> Value {
    json!({"queue": queue, "number": number})
}

fn orders_dusk() -> Value {
    json!([
        {"unit": "task_force:1", "order": {"kind": "detach", "ships": [2, 3]}},
        {"unit": "task_force:1", "order": {"kind": "set_speed", "speed": "cruise"}}
    ])
}

/// The places of a list of entries, as "queue number".
fn places(entries: &Value) -> Vec<String> {
    entries
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            format!(
                "{} {}",
                e["place"]["queue"].as_str().unwrap(),
                e["place"]["number"]
            )
        })
        .collect()
}

// ---------------------------------------------------------------------
// 1. queued
// ---------------------------------------------------------------------

#[test]
fn queued_lists_two_orders_and_a_command_at_dusk_in_the_sides_numbers() {
    // Scenario 4a: the Allied force is the umpire's task force 2 and its
    // ships 4, 5 and 6; the side's numbers are 1 and 1, 2 and 3.
    let mut s = session_of("04a_night_action.toml", true);
    let mut p = Player::of(&mut s);
    let empty = p.queued();
    assert_eq!(empty["entries"], json!([]));
    assert_eq!(
        empty["summary"].as_str().unwrap().split('.').next(),
        Some("Nothing is queued")
    );
    p.ok("issue_orders", json!({"orders": orders_dusk()}));
    p.ok("set_leave", json!({"leave": "launch", "given": true}));
    let answer = p.queued();
    assert_eq!(answer["kind"], "queued");
    assert_eq!(answer["version"], sim_agent::VERSION);
    assert_eq!(answer["clock"]["at"], "dusk");
    assert!(answer["as_of"].as_str().unwrap().starts_with("0-dusk-"));
    let entries = &answer["entries"];
    assert_eq!(places(entries), ["dusk 1", "dusk 2", "commands 1"]);

    let detach = &entries[0];
    assert_eq!(detach["what"], "order");
    assert_eq!(detach["unit"], "task_force:1");
    assert_eq!(detach["order"], json!({"kind": "detach", "ships": [2, 3]}));
    assert_eq!(detach["acts"], json!({"at": "dusk", "day": 1}));
    assert_eq!(detach["acts_words"], "at 1800, for day 1");
    assert_eq!(detach["goes_with"], json!([]));
    assert_eq!(detach["rests_on"], json!([]));
    let words = detach["text"].as_str().unwrap();
    assert!(
        words.contains("task_force:1") && words.contains("detach"),
        "{words}"
    );
    assert!(
        words.contains("ship:2") && words.contains("ship:3"),
        "{words}"
    );

    let speed = &entries[1];
    assert_eq!(speed["unit"], "task_force:1");
    assert_eq!(
        speed["order"],
        json!({"kind": "set_speed", "speed": "cruise"})
    );

    let leave = &entries[2];
    assert_eq!(leave["what"], "command");
    assert_eq!(leave["tool"], "set_leave");
    assert_eq!(leave["command"]["kind"], "set_leave");
    assert_eq!(leave["acts"], json!({"at": "dusk", "day": 1}));
    // None of the umpire's numbers is in the answer: not the force 2, not
    // the ships 4 to 6.
    let text = answer.to_string();
    assert!(
        !text.contains("task_force:2") && !text.contains("ship:5"),
        "{text}"
    );
    assert!(
        answer["summary"]
            .as_str()
            .unwrap()
            .starts_with("3 entries queued: 2 orders and 1 command."),
        "{answer}"
    );
    // Any argument is refused.
    let refusal = p.err("queued", json!({"day": 1}));
    assert!(refusal.contains("takes none"), "{refusal}");
}

#[test]
fn queued_inside_a_day_gives_a_pulse_order_for_its_pulse_and_a_kept_one_for_dusk() {
    let mut s = at_0600();
    let mut p = Player::of(&mut s);
    let given = p.ok(
        "issue_orders",
        json!({"orders": [
            {"unit": "task_force:1", "order": {"kind": "set_speed", "knots": "8"}},
            {"unit": "task_force:1", "order": {"kind": "set_mission", "mission": "surface_combat"}}
        ]}),
    );
    assert_eq!(given["kept_in_call"], 1, "{given}");
    p.ok("set_leave", json!({"leave": "launch", "given": true}));
    let answer = p.queued();
    assert_eq!(answer["clock"]["at"], "0600");
    assert!(answer["as_of"].as_str().unwrap().starts_with("0-morning-"));
    let entries = &answer["entries"];
    assert_eq!(places(entries), ["pulse 1", "dusk 1", "commands 1"]);
    assert_eq!(
        entries[0]["acts"],
        json!({"at": "pulse", "day": 1, "pulse": "morning"})
    );
    assert_eq!(
        entries[0]["acts_words"],
        "at the start of the morning pulse of day 1"
    );
    assert_eq!(
        entries[0]["order"],
        json!({"kind": "set_speed", "knots": "8"})
    );
    assert_eq!(entries[1]["acts"], json!({"at": "dusk", "day": 2}));
    assert_eq!(entries[1]["acts_words"], "at 1800, for day 2");
    assert_eq!(
        entries[1]["order"],
        json!({"kind": "set_mission", "mission": "surface_combat"})
    );
    assert_eq!(entries[2]["acts"], json!({"at": "dusk", "day": 2}));
    // A pulse order is taken back as any other, and a kept one too.
    let back = p.take_back("pulse", 1).expect("taken back");
    assert_eq!(places(&back["taken_back"]), ["pulse 1"]);
    assert_eq!(places(&back["queued"]), ["dusk 1", "commands 1"]);
    let back = p.take_back("dusk", 1).expect("taken back");
    assert_eq!(places(&back["queued"]), ["commands 1"]);
}

// ---------------------------------------------------------------------
// 2. take_back
// ---------------------------------------------------------------------

#[test]
fn taking_back_one_of_two_orders_leaves_the_other_and_the_list_after_it() {
    let mut s = session_of("04a_night_action.toml", true);
    let mut p = Player::of(&mut s);
    p.ok("issue_orders", json!({"orders": orders_dusk()}));
    let before = p.queued();
    let back = p.take_back("dusk", 1).expect("the detach is taken back");
    assert_eq!(back["kind"], "take_back");
    assert_eq!(back["taken_back"], json!([before["entries"][0].clone()]));
    assert_eq!(places(&back["queued"]), ["dusk 1"]);
    assert_eq!(back["queued"][0]["order"]["kind"], "set_speed");
    assert_ne!(back["as_of"], before["as_of"]);
    assert!(
        back["summary"]
            .as_str()
            .unwrap()
            .starts_with("Taken back: task_force:1: detach"),
        "{back}"
    );
    assert!(back["summary"]
        .as_str()
        .unwrap()
        .contains("1 entry remains queued."));
    // The list after it is the list it answered, with its token.
    let after = p.queued();
    assert_eq!(after["entries"], back["queued"]);
    assert_eq!(after["as_of"], back["as_of"]);
    let (dusk, pulse, commands) = s.queue_of(ALLIED);
    assert_eq!((dusk.len(), pulse.len(), commands.len()), (1, 0, 0));
}

#[test]
fn a_stale_as_of_or_a_bad_place_is_refused_and_changes_nothing() {
    let mut s = session_of("04a_night_action.toml", true);
    let mut p = Player::of(&mut s);
    p.ok("issue_orders", json!({"orders": orders_dusk()}));
    let listed = p.queued();
    // Something is queued after it was listed: its token is stale.
    p.ok("set_leave", json!({"leave": "launch", "given": true}));
    let refusal = p.err(
        "take_back",
        json!({"queue": "dusk", "number": 1, "as_of": listed["as_of"]}),
    );
    assert_eq!(
        refusal,
        "the queue has changed since it was listed: call queued again"
    );
    // No token, and a made-up one, are refused alike.
    let missing = p.err("take_back", json!({"queue": "dusk", "number": 1}));
    assert!(missing.contains("as_of"), "{missing}");
    p.err(
        "take_back",
        json!({"queue": "dusk", "number": 1, "as_of": "0-dusk-0"}),
    );
    // Places that are nothing, with the right token.
    let fresh = p.queued();
    let as_of = fresh["as_of"].clone();
    for (queue, number) in [("dusk", 9), ("dusk", 0), ("pulse", 1), ("commands", 2)] {
        let refusal = p.err(
            "take_back",
            json!({"queue": queue, "number": number, "as_of": as_of}),
        );
        assert!(
            refusal.contains(&format!("nothing is queued at place {queue} {number}")),
            "{refusal}"
        );
    }
    let refusal = p.err(
        "take_back",
        json!({"queue": "later", "number": 1, "as_of": as_of}),
    );
    assert!(refusal.contains("\"queue\""), "{refusal}");
    p.err(
        "take_back",
        json!({"queue": "dusk", "number": 1, "as_of": as_of, "extra": 1}),
    );
    // Nothing changed: the same list, the same token.
    assert_eq!(p.queued(), fresh);
}

#[test]
fn a_take_back_when_the_game_is_over_is_refused() {
    let mut s = session_of("04a_night_action.toml", true);
    let mut p = Player::of(&mut s);
    p.ok("issue_orders", json!({"orders": orders_dusk()}));
    let as_of = p.queued()["as_of"].clone();
    p.ok("end_day", json!({}));
    // The only day is played: the game is over.
    let refusal = p.err(
        "take_back",
        json!({"queue": "dusk", "number": 1, "as_of": as_of}),
    );
    assert_eq!(refusal, "the game is over");
}

// ---------------------------------------------------------------------
// 3. A hold goes with the order it was sent before
// ---------------------------------------------------------------------

fn air_order(kind: Value) -> Value {
    json!({"unit": "air_group:1", "order": kind})
}

fn search() -> Value {
    air_order(json!({"kind": "set_air_mission", "mission": "search"}))
}

fn arc() -> Value {
    air_order(json!({"kind": "set_search_arc", "from_deg": 60, "to_deg": 120, "range_nm": "400"}))
}

#[test]
fn a_hold_sent_before_an_order_goes_with_it_when_the_order_is_taken_back() {
    // Scenario 31: air group 1 at base 1.
    let mut s = session_of("31_airfield_damage.toml", true);
    let mut p = Player::of(&mut s);
    let held = p.ok("hold", json!({"air_group": 1}));
    assert_eq!(held["status"], "queued", "{held}");
    let given = p.ok("issue_orders", json!({"orders": [search()]}));
    assert_eq!(given["results"][0]["status"], "queued", "{given}");
    let listed = p.queued();
    assert_eq!(places(&listed["entries"]), ["dusk 1", "commands 1"]);
    // The hold rests on the order, and the order goes with the hold.
    assert_eq!(listed["entries"][1]["tool"], "hold");
    assert_eq!(
        listed["entries"][1]["command"],
        json!({"kind": "hold", "air_group": 1})
    );
    assert_eq!(listed["entries"][1]["rests_on"], json!([place("dusk", 1)]));
    assert_eq!(listed["entries"][1]["goes_with"], json!([]));
    assert_eq!(
        listed["entries"][0]["goes_with"],
        json!([place("commands", 1)])
    );
    assert_eq!(listed["entries"][0]["rests_on"], json!([]));
    // Both go.
    let back = p.take_back("dusk", 1).expect("the order is taken back");
    assert_eq!(places(&back["taken_back"]), ["dusk 1", "commands 1"]);
    assert_eq!(back["queued"], json!([]));
    assert!(
        back["summary"].as_str().unwrap().contains("; with it: "),
        "{back}"
    );
    assert!(back["summary"]
        .as_str()
        .unwrap()
        .contains("0 entries remain queued."));
    let (dusk, pulse, commands) = s.queue_of(ALLIED);
    assert_eq!((dusk.len(), pulse.len(), commands.len()), (0, 0, 0));
}

#[test]
fn taking_back_the_hold_alone_leaves_the_order() {
    let mut s = session_of("31_airfield_damage.toml", true);
    let mut p = Player::of(&mut s);
    p.ok("hold", json!({"air_group": 1}));
    p.ok("issue_orders", json!({"orders": [search()]}));
    let back = p.take_back("commands", 1).expect("the hold is taken back");
    assert_eq!(places(&back["taken_back"]), ["commands 1"]);
    assert_eq!(places(&back["queued"]), ["dusk 1"]);
    assert_eq!(back["queued"][0]["order"]["kind"], "set_air_mission");
    assert_eq!(back["queued"][0]["goes_with"], json!([]));
}

#[test]
fn a_hold_stays_while_another_order_to_its_unit_is_queued() {
    let mut s = session_of("31_airfield_damage.toml", true);
    let mut p = Player::of(&mut s);
    p.ok("hold", json!({"air_group": 1}));
    p.ok("issue_orders", json!({"orders": [search(), arc()]}));
    let listed = p.queued();
    assert_eq!(
        listed["entries"][2]["rests_on"],
        json!([place("dusk", 1), place("dusk", 2)])
    );
    // With another order to the unit queued, an order goes alone.
    assert_eq!(listed["entries"][0]["goes_with"], json!([]));
    let back = p
        .take_back("dusk", 1)
        .expect("the first order is taken back");
    assert_eq!(places(&back["taken_back"]), ["dusk 1"]);
    assert_eq!(places(&back["queued"]), ["dusk 1", "commands 1"]);
    // The last order to the unit takes the hold.
    let back = p
        .take_back("dusk", 1)
        .expect("the last order is taken back");
    assert_eq!(places(&back["taken_back"]), ["dusk 1", "commands 1"]);
    assert_eq!(back["queued"], json!([]));
}

// ---------------------------------------------------------------------
// 4. A new intent takes its limits
// ---------------------------------------------------------------------

#[test]
fn taking_back_a_new_intent_takes_the_limit_set_on_it() {
    // Scenario 36, no commands of its own.
    let mut s = session_of("36_one_convoy.toml", true);
    let mut p = Player::of(&mut s);
    let a = p.ok(
        "set_intent",
        json!({"base": 2, "floor_days": 30, "interval_days": 15}),
    );
    assert_eq!(a["intent"], 1);
    p.ok(
        "set_limit",
        json!({"limit": "forces", "intent": 1, "ships": [1, 2], "source": 1}),
    );
    p.ok("set_leave", json!({"leave": "launch", "given": true}));
    let listed = p.queued();
    assert_eq!(
        places(&listed["entries"]),
        ["commands 1", "commands 2", "commands 3"]
    );
    assert_eq!(
        listed["entries"][0]["goes_with"],
        json!([place("commands", 2)])
    );
    assert_eq!(listed["entries"][1]["goes_with"], json!([]));
    let back = p
        .take_back("commands", 1)
        .expect("the intent is taken back");
    assert_eq!(places(&back["taken_back"]), ["commands 1", "commands 2"]);
    assert_eq!(back["taken_back"][0]["tool"], "set_intent");
    assert_eq!(back["taken_back"][1]["tool"], "set_limit");
    // What is left is the leave, now the first command.
    assert_eq!(places(&back["queued"]), ["commands 1"]);
    assert_eq!(back["queued"][0]["tool"], "set_leave");
    let (_, _, commands) = s.queue_of(ALLIED);
    assert_eq!(commands.len(), 1);
}

#[test]
fn taking_back_the_limit_alone_leaves_the_intent() {
    let mut s = session_of("36_one_convoy.toml", true);
    let mut p = Player::of(&mut s);
    p.ok(
        "set_intent",
        json!({"base": 2, "floor_days": 30, "interval_days": 15}),
    );
    p.ok(
        "set_limit",
        json!({"limit": "forces", "intent": 1, "ships": [1, 2], "source": 1}),
    );
    let back = p.take_back("commands", 2).expect("the limit is taken back");
    assert_eq!(places(&back["taken_back"]), ["commands 2"]);
    assert_eq!(places(&back["queued"]), ["commands 1"]);
}

// ---------------------------------------------------------------------
// 5. A forming order with an order to the force it makes
// ---------------------------------------------------------------------

#[test]
fn a_forming_order_with_an_order_to_its_force_cannot_be_taken_back_first() {
    // Scenario 39: ships 5 and 6 lie in port at base 2 in no force; the
    // force they make is the side's fourth.
    let mut s = session_of("39_load_reserve_and_forces.toml", true);
    let mut p = Player::of(&mut s);
    let given = p.ok(
        "issue_orders",
        json!({"orders": [
            {"unit": "base:2", "order": {"kind": "form_task_force", "ships": [5, 6]}},
            {"unit": "task_force:4", "order": {"kind": "load", "weight_t": "100"}}
        ]}),
    );
    assert_eq!(given["results"][1]["status"], "queued", "{given}");
    let listed = p.queued();
    assert_eq!(listed["entries"][0]["unit"], "base:2");
    assert_eq!(listed["entries"][1]["unit"], "task_force:4");
    assert!(!listed.to_string().contains("1000001"), "{listed}");
    let refusal = p
        .take_back("dusk", 1)
        .expect_err("the forming order is relied on");
    let words = refusal.message;
    assert!(words.contains("(place dusk 2)"), "{words}");
    assert!(
        words.contains("rests on it: take that back first"),
        "{words}"
    );
    assert!(
        words.contains("task_force:4") && words.contains("load"),
        "{words}"
    );
    // Both stay queued, and the list and its token are what they were.
    let after = p.queued();
    assert_eq!(after["entries"], listed["entries"]);
    assert_eq!(after["as_of"], listed["as_of"]);
    let (dusk, _, _) = s.queue_of(ALLIED);
    assert_eq!(dusk.len(), 2);
    // The load goes first, then the forming order.
    let mut p = Player::of(&mut s);
    p.take_back("dusk", 2).expect("the load is taken back");
    p.take_back("dusk", 1)
        .expect("the forming order is taken back");
    assert_eq!(p.queued()["entries"], json!([]));
}

#[test]
fn a_second_forming_order_cannot_be_left_after_the_first_is_taken_back_under_an_order_to_it() {
    // Two forming orders make forces 4 and 5; an order to force 5 rests on
    // the first forming order too: without it force 5 is not made.
    let mut s = session_of("39_load_reserve_and_forces.toml", true);
    let mut p = Player::of(&mut s);
    let given = p.ok(
        "issue_orders",
        json!({"orders": [
            {"unit": "base:2", "order": {"kind": "form_task_force", "ships": [5]}},
            {"unit": "base:2", "order": {"kind": "form_task_force", "ships": [6]}},
            {"unit": "task_force:5", "order": {"kind": "load", "weight_t": "100"}}
        ]}),
    );
    assert_eq!(given["results"][2]["status"], "queued", "{given}");
    let refusal = p
        .take_back("dusk", 1)
        .expect_err("the first forming order is relied on");
    assert!(
        refusal.message.contains("(place dusk 3)"),
        "{}",
        refusal.message
    );
    assert_eq!(p.queued()["entries"].as_array().unwrap().len(), 3);
}

// ---------------------------------------------------------------------
// 6. A preview is void after a take back
// ---------------------------------------------------------------------

#[test]
fn a_preview_made_before_a_take_back_is_void_after_it() {
    let mut s = session_of("36_one_convoy.toml", true);
    let mut p = Player::of(&mut s);
    p.ok(
        "set_intent",
        json!({"base": 2, "floor_days": 30, "interval_days": 15}),
    );
    p.ok("hold", json!({"ship": 1}));
    let shown = p.ok(
        "preview",
        json!({"command": "set_intent", "intent": 1, "base": 2, "floor_days": 40, "interval_days": 15}),
    );
    assert_eq!(shown["preview"], 1);
    p.take_back("commands", 2).expect("the hold is taken back");
    let approved = p.ok("approve", json!({"preview": 1}));
    assert_eq!(approved["status"], "refused", "{approved}");
    let reason = approved["reason"].as_str().unwrap();
    assert!(reason.contains("preview 1 is void"), "{reason}");
    // Previewed again after it, the same change is queued.
    let again = p.ok(
        "preview",
        json!({"command": "set_intent", "intent": 1, "base": 2, "floor_days": 40, "interval_days": 15}),
    );
    let approved = p.ok("approve", json!({"preview": again["preview"]}));
    assert_eq!(approved["status"], "queued", "{approved}");
}

// ---------------------------------------------------------------------
// 7. The tools of a seat
// ---------------------------------------------------------------------

#[test]
fn the_tools_are_in_the_seats_list_after_end_day() {
    let s = session_of("04a_night_action.toml", true);
    let tools = s.tools(ALLIED);
    let at = |name: &str| tools.iter().position(|t| *t == name).unwrap();
    assert_eq!(at("queued"), at("end_day") + 1);
    assert_eq!(at("take_back"), at("queued") + 1);
    // A session with no staff has them too: they are of every seat.
    let scenario = load("04a_night_action.toml", 1942);
    let bare = Session::new(
        scenario.world,
        "bare",
        scenario.days,
        Seat::Player,
        Seat::Script(BTreeMap::new()),
    );
    assert!(bare.tools(ALLIED).contains(&"take_back"));
}

#[test]
fn a_watched_seat_that_is_ready_is_refused_them_and_the_watcher_has_neither() {
    let mut s = session_of("04a_night_action.toml", true).watched();
    // Either seat is a player's in a watched game; the Japanese is the script.
    assert!(matches!(
        s.seat_call(ALLIED, "queued", &json!({})),
        Called::Answered(Ok(_))
    ));
    assert!(matches!(
        s.seat_call(ALLIED, "end_day", &json!({})),
        Called::Held
    ));
    for tool in ["queued", "take_back"] {
        match s.seat_call(ALLIED, tool, &json!({})) {
            Called::Answered(Err(e)) => {
                assert!(e.message.contains("is ready and waits"), "{tool}: {e}")
            }
            other => panic!("{tool} was not refused: {}", matches!(other, Called::Held)),
        }
    }
    for tool in ["queued", "take_back"] {
        let refusal = s
            .watch(ALLIED, tool, &json!({}))
            .expect_err("the watcher reads and changes nothing here");
        assert!(
            refusal.message.contains("not a tool of the watcher's"),
            "{refusal}"
        );
    }
}
