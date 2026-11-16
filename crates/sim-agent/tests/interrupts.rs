//! Interrupts, `run` and `report`, by the tools of a seat with a staff
//! (decision 0107, step 4, items 29, 30 and 46 and rulings 31 to 37), over
//! scenario 36 and scenario 40a. The figures asserted are the architect's hand
//! calculation of scenario 42 (`tests/scenario_42_interrupts_and_runs.rs`):
//! with interrupt 1 on base 2 below 35 days, a `run` of 30 days resolves day 1
//! (10 days of supply), a second resolves days 2 to 13 (34 days) and, the
//! interrupt acknowledged, a third resolves days 14 to 18 and the game is
//! over. In scenario 40a the staff cannot meet intent 2 on days 1 to 4 and
//! intent 1 on days 10 to 13.
//!
//! What is tested here is the tools: the answers fit their schemas, a `run`
//! is a day-for-day copy of `end_day`, and `report` and `command_state` say
//! what the hand calculation says.

mod support;

use std::collections::BTreeMap;

use serde_json::{json, Value};
use sim_agent::{CallError, Seat, Session, COMMAND_TOOLS};
use sim_core::state::Side;
use support::{load, schemas};

/// A session over a scenario with the Allied seat a player and the script's
/// own commands taken out (`own_commands` false) or kept (true).
fn session(name: &str, own_commands: bool, japanese: Seat) -> Session {
    let mut scenario = load(name, 1942);
    let commands = scenario.scripted_commands().clone();
    scenario.replace_commands(BTreeMap::new());
    let title = scenario.name.clone();
    let session = Session::new(
        scenario.world,
        &title,
        scenario.days,
        Seat::Player,
        japanese,
    )
    .with_staffs(sim_staff::staffs());
    if own_commands {
        session.with_commands(commands)
    } else {
        session
    }
}

struct Player {
    session: Session,
    schemas: support::validator::Schemas,
}

impl Player {
    fn new(session: Session) -> Player {
        Player {
            session,
            schemas: schemas(),
        }
    }

    fn try_call(&mut self, tool: &str, args: Value) -> Result<Value, CallError> {
        let result = self.session.call(Side::Allied, tool, &args);
        let answer = match &result {
            Ok(answer) => answer.clone(),
            Err(error) => error.to_json(),
        };
        let kind = answer["kind"].as_str().expect("an answer has a kind");
        let errors = self.schemas.check(&format!("{kind}.schema.json"), &answer);
        assert!(errors.is_empty(), "{tool} {args}: {errors:?}\n{answer}");
        result
    }

    fn call(&mut self, tool: &str, args: Value) -> Value {
        self.try_call(tool, args)
            .unwrap_or_else(|e| panic!("{tool}: {e}"))
    }

    fn queued(&mut self, tool: &str, args: Value) -> Value {
        let a = self.call(tool, args);
        assert_eq!(a["status"], "queued", "{a}");
        a
    }

    /// Scenario 36's commands: the intent, the forces limit, the reserve and
    /// the two leaves, and interrupt 1 on base 2 below 35 days.
    fn command_scenario_36(&mut self, with_interrupt: bool) {
        self.queued(
            "set_intent",
            json!({"base": 2, "floor_days": 30, "interval_days": 15}),
        );
        self.queued(
            "set_limit",
            json!({"limit": "forces", "intent": 1, "ships": [1, 2], "source": 1}),
        );
        self.queued(
            "set_limit",
            json!({"limit": "reserve", "base": 1, "supply_t": 1000, "fuel_t": 0}),
        );
        self.queued("set_leave", json!({"leave": "prepare", "given": true}));
        self.queued("set_leave", json!({"leave": "launch", "given": true}));
        if with_interrupt {
            let a = self.queued("set_interrupt", json!({"base": 2, "below_days": 35}));
            assert_eq!(a["interrupt"], 1, "no number given: the next free");
        }
    }
}

fn days_resolved(a: &Value) -> (u64, u64, u64) {
    let d = &a["days_resolved"];
    (
        d["first"].as_u64().unwrap(),
        d["last"].as_u64().unwrap(),
        d["count"].as_u64().unwrap(),
    )
}

#[test]
fn the_four_tools_follow_staff_orders_in_the_list_and_the_version_is_11() {
    let at = |tool: &str| COMMAND_TOOLS.iter().position(|t| *t == tool).unwrap();
    assert_eq!(at("set_interrupt"), at("staff_orders") + 1);
    assert_eq!(at("acknowledge"), at("set_interrupt") + 1);
    assert_eq!(at("run"), at("acknowledge") + 1);
    assert_eq!(at("report"), at("run") + 1);
    assert_eq!(at("why"), at("report") + 1);
    assert_eq!(sim_agent::VERSION, 11);
}

#[test]
fn a_run_stops_for_an_interrupt_and_at_the_end_of_the_game() {
    let mut p = Player::new(session(
        "36_one_convoy.toml",
        false,
        Seat::Script(BTreeMap::new()),
    ));
    p.command_scenario_36(true);

    // The report before any day is resolved: nothing open.
    let r = p.call("report", json!({}));
    assert_eq!(r["day"], 0);
    assert_eq!(r["open_matters"], 0);
    assert_eq!(r["matters"], json!([]));
    assert_eq!(r["fired_last_day"], json!([]));

    // The first call: day 1 only, the interrupt already true when set.
    let a = p.call("run", json!({"days": 30}));
    assert_eq!(days_resolved(&a), (1, 1, 1), "{a}");
    assert_eq!(a["stopped"], "interrupt");
    assert_eq!(a["game_over"], false);
    assert_eq!(a["day"], 1);
    assert_eq!(a["fired"].as_array().unwrap().len(), 1, "{a}");
    let fired = &a["fired"][0];
    assert_eq!(fired["interrupt"], 1);
    assert_eq!(fired["base"]["handle"], "base:2");
    assert_eq!(fired["below_days"], 35);
    assert_eq!(fired["figure"], "10");
    assert_eq!(fired["day"], 1);
    assert_eq!(fired["acknowledged"], false);
    assert_eq!(fired["why"], json!([]));
    assert_eq!(
        a["summary"],
        "Resolved day 1 (1 day); stopped after day 1: interrupt:1 fired, base:2 at 10 days of supply, below 35."
    );
    // The staff's orders for the first day were worked before it resolved:
    // the force formed and the load of 360 t.
    let orders = a["staff_orders"]["orders"].as_array().unwrap();
    assert!(
        orders
            .iter()
            .any(|o| o["order"]["kind"] == "form_task_force"),
        "{}",
        a["staff_orders"]
    );
    let load = orders
        .iter()
        .find(|o| o["order"]["kind"] == "load")
        .unwrap_or_else(|| panic!("a load order: {}", a["staff_orders"]));
    assert_eq!(load["order"]["weight_t"], "360", "{load}");
    assert_eq!(a["staff_orders"]["for_day"], 1);

    // command_state shows the interrupt and its status after it fired.
    let state = p.call("command_state", json!({}));
    let set = &state["interrupts"]["set"][0];
    assert_eq!(set["interrupt"], 1);
    assert_eq!(set["below_days"], 35);
    assert_eq!(set["set_day"], 1);
    assert_eq!(set["armed"], false);
    assert_eq!(set["last_fired"], 1);
    assert_eq!(set["acknowledged"], false);
    assert_eq!(set["needs_acknowledgement"], true);
    assert_eq!(
        set["text"],
        "interrupt:1 (base:2 below 35 days), last fired day 1, not acknowledged"
    );

    // The second call: days 2 to 13, twelve days, a quiet week and more.
    let a = p.call("run", json!({"days": 30}));
    assert_eq!(days_resolved(&a), (2, 13, 12), "{a}");
    assert_eq!(a["stopped"], "interrupt");
    assert_eq!(a["fired"][0]["figure"], "34");
    assert_eq!(a["fired"][0]["day"], 13);
    assert_eq!(
        a["summary"],
        "Resolved days 2 to 13 (12 days); stopped after day 13: interrupt:1 fired, base:2 at 34 days of supply, below 35."
    );

    // The report: nothing open, the interrupt fired and not acknowledged.
    let r = p.call("report", json!({}));
    assert_eq!(r["day"], 13);
    assert_eq!(r["interrupts"]["set"][0]["last_fired"], 13);
    assert_eq!(r["fired_last_day"].as_array().unwrap().len(), 1);
    assert_eq!(
        r["summary"],
        "Day 13. No open matter. No matter closed. Interrupts: interrupt:1 (base:2 below 35 days), last fired day 13, not acknowledged."
    );

    // Acknowledging marks it seen; the third call runs to the end.
    let a = p.queued("acknowledge", json!({"interrupt": 1}));
    assert_eq!(a["command"], json!({"kind": "acknowledge", "interrupt": 1}));
    let a = p.call("run", json!({"days": 30}));
    assert_eq!(days_resolved(&a), (14, 18, 5), "{a}");
    assert_eq!(a["stopped"], "game_over");
    assert_eq!(a["game_over"], true);
    assert_eq!(a["fired"], json!([]));
    assert_eq!(
        a["summary"],
        "Resolved days 14 to 18 (5 days); stopped after day 18: the game is over."
    );
    // Over: refused.
    let error = p.try_call("run", json!({"days": 1})).expect_err("over");
    assert_eq!(error.message, "the game is over");
    let r = p.call("report", json!({}));
    assert_eq!(
        r["summary"],
        "Day 18. No open matter. No matter closed. Interrupts: interrupt:1 (base:2 below 35 days), last fired day 13, acknowledged."
    );
}

#[test]
fn a_run_is_end_day_day_for_day() {
    let mut run = Player::new(session(
        "36_one_convoy.toml",
        false,
        Seat::Script(BTreeMap::new()),
    ));
    run.command_scenario_36(true);
    let mut by_hand = Player::new(session(
        "36_one_convoy.toml",
        false,
        Seat::Script(BTreeMap::new()),
    ));
    by_hand.command_scenario_36(true);

    for call in [30, 30, 30] {
        run.call("run", json!({"days": call}));
    }
    for _ in 0..18 {
        by_hand.call("end_day", json!({}));
    }
    assert_eq!(run.session.day_hashes().len(), 18);
    assert_eq!(run.session.day_hashes(), by_hand.session.day_hashes());
    assert_eq!(run.session.state_hash(), by_hand.session.state_hash());
}

#[test]
fn a_run_stops_for_its_number_of_days() {
    let mut p = Player::new(session(
        "36_one_convoy.toml",
        false,
        Seat::Script(BTreeMap::new()),
    ));
    p.command_scenario_36(false);
    let a = p.call("run", json!({"days": 5}));
    assert_eq!(days_resolved(&a), (1, 5, 5), "{a}");
    assert_eq!(a["stopped"], "days");
    assert_eq!(a["fired"], json!([]));
    assert_eq!(
        a["summary"],
        "Resolved days 1 to 5 (5 days); stopped after day 5: the 5 days asked for are resolved."
    );
}

#[test]
fn run_is_refused_for_no_days_and_not_offered_with_two_players() {
    let mut p = Player::new(session(
        "36_one_convoy.toml",
        false,
        Seat::Script(BTreeMap::new()),
    ));
    for bad in [
        json!({"days": 0}),
        json!({}),
        json!({"days": "2"}),
        json!({"days": -1}),
    ] {
        let error = p.try_call("run", bad.clone()).expect_err("refused");
        assert!(
            error.message.contains("\"days\" is needed"),
            "{bad}: {error}"
        );
    }
    let error = p
        .try_call("run", json!({"days": 1, "x": 1}))
        .expect_err("refused");
    assert!(error.message.contains("not an argument of run"));
    // Nothing was resolved by the refusals.
    assert_eq!(p.session.day(), 0);
    assert!(p.session.tools(Side::Allied).contains(&"run"));

    let two = session("36_one_convoy.toml", false, Seat::Player);
    assert!(!two.tools(Side::Allied).contains(&"run"));
    assert!(!two.tools(Side::Japanese).contains(&"run"));
    assert!(two.tools(Side::Allied).contains(&"report"));
    let mut two = Player::new(two);
    let error = two
        .try_call("run", json!({"days": 1}))
        .expect_err("refused");
    assert!(error.message.contains("exactly one seat"), "{error}");
    assert_eq!(two.session.day(), 0);
}

#[test]
fn acknowledging_an_interrupt_that_never_fired_is_refused_with_the_engines_reason() {
    let mut p = Player::new(session(
        "36_one_convoy.toml",
        false,
        Seat::Script(BTreeMap::new()),
    ));
    p.command_scenario_36(false);
    p.queued("set_interrupt", json!({"base": 2, "below_days": 5}));
    // Set but never fired: the engine's words.
    let a = p.call("acknowledge", json!({"interrupt": 1}));
    assert_eq!(a["status"], "refused", "{a}");
    assert_eq!(a["reason"], "interrupt:1 has not fired");
    // Not set at all.
    let a = p.call("acknowledge", json!({"interrupt": 9}));
    assert_eq!(a["status"], "refused", "{a}");
    assert_eq!(a["reason"], "interrupt:9 was not set");
    // A standing one that never fired, a plan that is none, and a ship that
    // is not the side's (and one that does not exist), in the side's numbers.
    let a = p.call("acknowledge", json!({"standing": "cannot_meet", "plan": 1}));
    assert_eq!(a["status"], "refused", "{a}");
    let a = p.call("acknowledge", json!({"standing": "ship_lost", "ship": 7}));
    assert_eq!(a["status"], "refused", "{a}");
    assert_eq!(a["command"]["ship"], 7);
    assert!(a["reason"].as_str().unwrap().contains("ship:7"), "{a}");
    // Naming both ways, or neither, is a malformed command.
    let error = p
        .try_call(
            "acknowledge",
            json!({"interrupt": 1, "standing": "ship_lost"}),
        )
        .expect_err("malformed");
    assert!(error.message.contains("malformed command"), "{error}");
    // Only the one set_interrupt was queued.
    let state = p.call("command_state", json!({}));
    assert_eq!(state["queued"].as_array().unwrap().len(), 6);
}

#[test]
fn set_interrupt_numbers_itself_and_reads_its_arguments() {
    let mut p = Player::new(session(
        "36_one_convoy.toml",
        false,
        Seat::Script(BTreeMap::new()),
    ));
    p.command_scenario_36(false);
    let a = p.queued("set_interrupt", json!({"base": 2, "below_days": 20}));
    assert_eq!(a["interrupt"], 1);
    assert_eq!(
        a["command"],
        json!({"kind": "set_interrupt", "interrupt": 1, "base": 2, "below_days": 20})
    );
    let a = p.queued("set_interrupt", json!({"base": 1, "below_days": 3}));
    assert_eq!(a["interrupt"], 2);
    let a = p.queued(
        "set_interrupt",
        json!({"interrupt": 2, "base": 2, "below_days": 9}),
    );
    assert!(a.get("interrupt").is_none() || a["interrupt"] == 2);
    let a = p.queued(
        "set_interrupt",
        json!({"standing": "ship_lost", "on": false}),
    );
    assert_eq!(
        a["command"],
        json!({"kind": "set_interrupt", "standing": "ship_lost", "on": false})
    );
    // An enemy base, a base that does not exist and zero days: refused.
    let a = p.call("set_interrupt", json!({"base": 99, "below_days": 5}));
    assert_eq!(a["status"], "refused", "{a}");
    let a = p.call("set_interrupt", json!({"base": 2, "below_days": 0}));
    assert_eq!(a["status"], "refused", "{a}");
    // Mixed forms are malformed.
    for bad in [
        json!({"standing": "ship_lost", "on": true, "base": 2}),
        json!({"standing": "ship_lost"}),
        json!({"base": 2, "below_days": 5, "on": true}),
        json!({"standing": "nonsense", "on": true}),
    ] {
        let error = p
            .try_call("set_interrupt", bad.clone())
            .expect_err("refused");
        assert!(!error.message.is_empty(), "{bad}");
    }
    let state = p.call("command_state", json!({}));
    assert_eq!(state["interrupts"]["switched_off"], json!(["ship_lost"]));
    assert_eq!(state["interrupts"]["set"].as_array().unwrap().len(), 2);
}

/// Scenario 40a: two intents, one ship, the staff cannot meet intent 2 on
/// days 1 to 4 and intent 1 on days 10 to 13. The plans are numbered by the
/// intents they serve.
#[test]
fn the_report_of_scenario_40a_has_two_closed_matters_each_day_leading_to_a_reason() {
    let mut p = Player::new(session(
        "40a_one_pool_two_hexes.toml",
        true,
        Seat::Script(BTreeMap::new()),
    ));

    let a = p.call("run", json!({"days": 30}));
    assert_eq!(days_resolved(&a), (1, 1, 1), "{a}");
    assert_eq!(a["stopped"], "interrupt");
    let fired = a["fired"].as_array().unwrap();
    assert_eq!(fired.len(), 1, "{a}");
    assert_eq!(fired[0]["standing"], "cannot_meet");
    assert_eq!(fired[0]["plan"], 2);
    assert_eq!(fired[0]["acknowledged"], false);
    let why = fired[0]["why"].as_array().unwrap();
    assert!(
        !why.is_empty(),
        "a firing on a notice leads to its reason: {a}"
    );
    // Its words, and each why number gives a reason.
    assert_eq!(
        fired[0]["words"],
        "standing cannot_meet fired: plan:2 cannot meet its intent"
    );
    for number in why {
        let w = p.call("why", json!({"number": number}));
        assert_eq!(w["kind"], "why", "{w}");
    }
    // The report after day 1: plan 2 is open.
    let r = p.call("report", json!({}));
    assert_eq!(r["open_matters"], 1, "{r}");
    assert_eq!(r["matters"][0]["plan"], 2);
    assert_eq!(r["matters"][0]["open"], true);
    assert_eq!(r["matters"][0]["closed"], Value::Null);
    assert_eq!(r["matters"][0]["kind"], "cannot_meet");
    assert!(r["summary"].as_str().unwrap().starts_with(
        "Day 1. 1 open matter: plan:2 (intent:2) cannot meet its intent (since day 1)."
    ));
    assert_eq!(
        r["interrupts"]["standing"][0]["needs_acknowledgement"],
        true
    );
    p.queued("acknowledge", json!({"standing": "cannot_meet", "plan": 2}));

    let a = p.call("run", json!({"days": 30}));
    assert_eq!(days_resolved(&a), (2, 10, 9), "{a}");
    assert_eq!(a["fired"][0]["plan"], 1);
    let a = p.call("run", json!({"days": 30}));
    assert_eq!(days_resolved(&a), (11, 16, 6), "{a}");
    assert_eq!(a["stopped"], "game_over");

    let r = p.call("report", json!({}));
    assert_eq!(r["day"], 16);
    assert_eq!(r["open_matters"], 0, "{r}");
    assert_eq!(r["closed_matters"], 2, "{r}");
    let matters = r["matters"].as_array().unwrap();
    assert_eq!(matters.len(), 2, "{r}");
    // Closed ones by first day: plan 2 days 1 to 4, plan 1 days 10 to 13.
    for (matter, plan, days) in [
        (&matters[0], 2u64, 1u64..=4),
        (&matters[1], 1u64, 10u64..=13),
    ] {
        assert_eq!(matter["plan"], plan, "{matter}");
        assert_eq!(matter["run"], Value::Null);
        assert_eq!(matter["kind"], "cannot_meet");
        assert_eq!(matter["open"], false);
        assert_eq!(matter["opened"], *days.start());
        assert_eq!(matter["closed"], *days.end() + 1);
        let history = matter["history"].as_array().unwrap();
        let shown: Vec<u64> = history.iter().map(|h| h["day"].as_u64().unwrap()).collect();
        assert_eq!(shown, days.collect::<Vec<_>>());
        for entry in history {
            let number = entry["why"].as_u64().expect("each day leads to a reason");
            let w = p.call("why", json!({"number": number}));
            assert_eq!(w["kind"], "why", "{w}");
            assert_eq!(w["day"], entry["day"], "{w}");
            // The reason is of that plan's intent: it names the base the
            // plan keeps supplied.
            let text = w["reason"].to_string();
            let base = if plan == 2 { "base:3" } else { "base:2" };
            assert!(text.contains(base), "{base} in {text}");
        }
    }
    // Every firing is in the interrupts part, acknowledged first
    // acknowledged: plan 2's was acknowledged, plan 1's was not and leads.
    let standing = r["interrupts"]["standing"].as_array().unwrap();
    assert_eq!(standing.len(), 2, "{r}");
    assert_eq!(standing[0]["plan"], 1);
    assert_eq!(standing[0]["needs_acknowledgement"], true);
    assert_eq!(standing[1]["plan"], 2);
    assert_eq!(standing[1]["acknowledged"], true);
}

#[test]
fn a_standing_kind_switched_off_does_not_stop_the_clock() {
    let mut p = Player::new(session(
        "40a_one_pool_two_hexes.toml",
        true,
        Seat::Script(BTreeMap::new()),
    ));
    p.queued(
        "set_interrupt",
        json!({"standing": "cannot_meet", "on": false}),
    );
    let a = p.call("run", json!({"days": 30}));
    assert_eq!(days_resolved(&a), (1, 16, 16), "{a}");
    assert_eq!(a["stopped"], "game_over");
    let r = p.call("report", json!({}));
    assert_eq!(r["interrupts"]["switched_off"], json!(["cannot_meet"]));
    // The matters are still reported: a notice is not an interrupt.
    assert_eq!(r["closed_matters"], 2, "{r}");
}
