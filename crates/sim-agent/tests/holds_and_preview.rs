//! Holds, release and previews, by the tools of a seat with a staff
//! (decision 0107, step 3, "The tools" and rulings 17 to 28), over scenario
//! 36: the Allied side keeps base 2 at 30 days of supply from base 1 with
//! ships 1 and 2 (`tests/scenario_36_one_convoy.rs`). The figures asserted
//! are the hand calculation's: on day 1 the first run carries 360 t in ship 1
//! (load 300 - 90 + 150) and with the floor at 40 days it carries 460 t
//! (400 - 90 + 150).
//!
//! What is tested here is the tools: that a hold and a release are queued and
//! refused as the other commands are, that `command_state` shows holds, that a
//! preview changes nothing and says what the staff would do, and that
//! approving a preview queues exactly what it showed, or is refused as void.

mod support;

use std::collections::BTreeMap;

use serde_json::{json, Value};
use sim_agent::{CallError, Seat, Session, COMMAND_TOOLS};
use sim_core::state::Side;
use support::{load, schemas};

fn session() -> Session {
    let mut scenario = load("36_one_convoy.toml", 1942);
    scenario.replace_commands(BTreeMap::new());
    let name = scenario.name.clone();
    Session::new(
        scenario.world,
        &name,
        scenario.days,
        Seat::Player,
        Seat::Script(BTreeMap::new()),
    )
    .with_staffs(sim_staff::staffs())
}

/// A player who checks every answer against its schema.
struct Player {
    session: Session,
    schemas: support::validator::Schemas,
}

impl Player {
    fn new() -> Player {
        Player {
            session: session(),
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

    /// Scenario 36's intent and its forces limit, queued.
    fn command_scenario_36(&mut self) {
        let a = self.call(
            "set_intent",
            json!({"base": 2, "floor_days": 30, "interval_days": 15}),
        );
        assert_eq!(a["status"], "queued", "{a}");
        let a = self.call(
            "set_limit",
            json!({"limit": "forces", "intent": 1, "ships": [1, 2], "source": 1}),
        );
        assert_eq!(a["status"], "queued", "{a}");
    }

    /// Plays to the morning of day 2: the intent and its limit, the leave to
    /// launch given and the plan approved, the day ended. The staff forms
    /// task force 1 of ship 1 on day 1.
    fn on_day_2(&mut self) {
        self.command_scenario_36();
        self.call("set_leave", json!({"leave": "launch", "given": true}));
        self.call("approve", json!({"plan": 1}));
        let end = self.call("end_day", json!({}));
        assert_eq!(end["resolved"], true);
    }
}

fn runs(side: &Value) -> Vec<(Vec<u64>, String)> {
    side["runs"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            (
                r["ships"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|s| s.as_u64().unwrap())
                    .collect(),
                r["load_t"].as_str().unwrap().to_string(),
            )
        })
        .collect()
}

#[test]
fn the_three_tools_are_in_the_list_after_set_leave_and_before_approve() {
    let at = |tool: &str| COMMAND_TOOLS.iter().position(|t| *t == tool).unwrap();
    assert_eq!(at("hold"), at("set_leave") + 1);
    assert_eq!(at("release"), at("hold") + 1);
    assert_eq!(at("preview"), at("release") + 1);
    assert_eq!(at("approve"), at("preview") + 1);
    assert_eq!(COMMAND_TOOLS.len(), 18);
    assert_eq!(sim_agent::VERSION, 13);
}

#[test]
fn a_hold_and_a_release_are_queued_and_refused_as_the_other_commands_are() {
    let mut p = Player::new();
    p.command_scenario_36();

    let a = p.call("hold", json!({"ship": 1}));
    assert_eq!(a["status"], "queued", "{a}");
    assert_eq!(a["tool"], "hold");
    assert_eq!(a["for_day"], 1);
    assert_eq!(a["command"], json!({"kind": "hold", "ship": 1}));
    assert_eq!(a["queued"], 3);
    assert!(a["notes"][0].as_str().unwrap().contains("gives no order"));

    // Held already: the engine's reason, in words, and nothing queued.
    let a = p.call("hold", json!({"ship": 1}));
    assert_eq!(a["status"], "refused", "{a}");
    assert!(
        a["reason"].as_str().unwrap().contains("already held"),
        "{a}"
    );
    assert_eq!(a["queued"], 3);

    // A ship that is none of the side's: refused in the number written, and
    // the engine's own number never shows.
    let a = p.call("hold", json!({"ship": 9}));
    assert_eq!(a["status"], "refused", "{a}");
    assert!(a["reason"].as_str().unwrap().contains("ship:9"), "{a}");
    assert_eq!(a["command"], json!({"kind": "hold", "ship": 9}));
    assert!(!a.to_string().contains("4000000000"), "{a}");
    for (key, number) in [("task_force", 4), ("air_group", 1)] {
        let mut args = json!({ key: number });
        if key == "air_group" {
            args["setting"] = json!("search_arc");
        }
        let a = p.call("hold", args);
        assert_eq!(a["status"], "refused", "{a}");
        assert!(!a.to_string().contains("4000000000"), "{a}");
        assert!(a["reason"]
            .as_str()
            .unwrap()
            .contains(&format!("{key}:{number}")));
    }

    // A release of what is not held is refused; of what is, queued.
    let a = p.call("release", json!({"ship": 2}));
    assert_eq!(a["status"], "refused", "{a}");
    assert!(a["reason"].as_str().unwrap().contains("not held"), "{a}");
    let a = p.call("release", json!({"ship": 1}));
    assert_eq!(a["status"], "queued", "{a}");
    assert_eq!(a["queued"], 4);

    // The call as a whole is refused for arguments that are no target.
    for bad in [
        json!({}),
        json!({"ship": 1, "task_force": 1}),
        json!({"air_group": 1, "setting": "mission"}),
        json!({"ship": 1, "setting": "search_arc"}),
        json!({"ship": 1, "plan": 1}),
        json!({"ship": 0.5}),
    ] {
        let error = p.try_call("hold", bad.clone()).expect_err("refused");
        assert!(
            error.message.contains("malformed command")
                || error.message.contains("is not an argument")
                || error.message.contains("\"setting\""),
            "{bad}: {error}"
        );
    }

    // The commands queued, in order, are the ones that were taken.
    let state = p.call("command_state", json!({}));
    let kinds: Vec<&str> = state["queued"]
        .as_array()
        .unwrap()
        .iter()
        .map(|q| q["command"]["kind"].as_str().unwrap())
        .collect();
    assert_eq!(kinds, ["set_intent", "set_limit", "hold", "release"]);
    assert_eq!(
        state["queued"][2]["command"],
        json!({"kind": "hold", "ship": 1})
    );
    // Held and then released in one morning: nothing is held when it begins.
    assert_eq!(state["holds"], json!([]));
}

#[test]
fn command_state_shows_a_hold_by_command_and_a_hold_by_an_order() {
    let mut p = Player::new();
    p.command_scenario_36();

    // Queued, a hold is part of what stands as the day begins.
    p.call("hold", json!({"ship": 2}));
    let state = p.call("command_state", json!({}));
    assert_eq!(
        state["holds"],
        json!([{
            "target": {"ship": 2},
            "day": 1,
            "made_by": "command",
            "order": null,
            "text": "ship 2, held by your hold command of day 1",
        }])
    );

    // An order of the commander's to a ship in the forces limit holds it
    // from that morning, and is shown as a hold made by that order.
    let a = p.call(
        "issue_orders",
        json!({"orders": [{"unit": "base:1", "order": {"kind": "form_task_force", "ships": [1]}}]}),
    );
    assert_eq!(a["results"][0]["status"], "queued", "{a}");
    let state = p.call("command_state", json!({}));
    let holds = state["holds"].as_array().unwrap();
    assert_eq!(holds.len(), 2, "{holds:?}");
    assert_eq!(
        holds[0],
        json!({
            "target": {"ship": 1},
            "day": 1,
            "made_by": "order",
            "order": 1,
            "text": "ship 1, held by your order 1 of day 1",
        })
    );
    assert_eq!(holds[1]["made_by"], "command");

    // The holds stand after the day, are shown on day 2, and a release lifts
    // either: the one an order made as well.
    p.call("set_leave", json!({"leave": "launch", "given": true}));
    p.call("end_day", json!({}));
    let state = p.call("command_state", json!({}));
    assert_eq!(state["for_day"], 2);
    assert_eq!(state["holds"].as_array().unwrap().len(), 2);
    assert_eq!(state["holds"][0]["day"], 1);
    for ship in [1, 2] {
        let a = p.call("release", json!({"ship": ship}));
        assert_eq!(a["status"], "queued", "{a}");
        assert!(a["notes"][0]
            .as_str()
            .unwrap()
            .contains("free to the staff again"));
    }
    let state = p.call("command_state", json!({}));
    assert_eq!(state["holds"], json!([]));
}

#[test]
fn a_task_force_and_a_setting_are_held_in_the_sides_numbers() {
    let mut p = Player::new();
    p.on_day_2();
    // The staff formed task force 1 of ship 1 on day 1.
    let a = p.call("hold", json!({"task_force": 1}));
    assert_eq!(a["status"], "queued", "{a}");
    assert_eq!(a["command"], json!({"kind": "hold", "task_force": 1}));
    let state = p.call("command_state", json!({}));
    assert_eq!(
        state["holds"][0],
        json!({
            "target": {"task_force": 1},
            "day": 2,
            "made_by": "command",
            "order": null,
            "text": "task force 1, held by your hold command of day 2",
        })
    );
    // The scenario has no air group: a setting of one is refused, as an
    // enemy's or a missing one is, in the number written.
    let a = p.call("hold", json!({"air_group": 3, "setting": "search_arc"}));
    assert_eq!(a["status"], "refused");
    assert!(a["reason"].as_str().unwrap().contains("air_group:3"), "{a}");
}

#[test]
fn a_preview_changes_nothing_and_says_what_the_staff_would_do() {
    let mut p = Player::new();
    p.command_scenario_36();
    p.call("hold", json!({"ship": 1}));

    let hash = p.session.state_hash();
    let state_before = p.call("command_state", json!({}));
    let orders_before = p.call("staff_orders", json!({}));
    let transcript = p.session.transcript().len();

    let v = p.call(
        "preview",
        json!({"command": "set_intent", "intent": 1, "base": 2, "floor_days": 40, "interval_days": 15}),
    );
    assert_eq!(v["kind"], "preview");
    assert_eq!(v["preview"], 1);
    assert_eq!(v["day"], 1);
    assert_eq!(v["intent"], 1);
    assert_eq!(v["valid"], true);
    assert_eq!(
        v["command"],
        json!({"kind": "set_intent", "intent": 1, "base": 2, "floor_days": 40, "interval_days": 15})
    );
    assert!(v["shown"].as_str().unwrap().contains("40 days"), "{v}");

    // As things stand the first run's load is 360 t (floor of 30 days); with
    // the floor at 40 days it is 460 t, and both are ordered today. (Which
    // ship the staff gives it, with ship 1 held, is the doctrine's to say,
    // and is tested with the doctrine.)
    assert_eq!(runs(&v["as_things_stand"])[0].1, "360", "{v}");
    assert_eq!(runs(&v["with_change"])[0].1, "460", "{v}");
    assert_eq!(
        v["as_things_stand"]["next_run"],
        v["as_things_stand"]["runs"][0]
    );
    assert_eq!(v["with_change"]["next_run"], v["with_change"]["runs"][0]);
    assert_eq!(v["with_change"]["first_day_below"], 1);
    let given = v["with_change"]["orders"].as_array().unwrap();
    assert_eq!(given[0]["order"]["kind"], "form_task_force");
    assert_eq!(given[1]["order"]["weight_t"], "460");
    assert!(given.iter().all(|o| o["intent"] == 1));
    assert_eq!(
        v["held"],
        json!(["ship 1 is held by your hold command of day 1"])
    );
    assert_eq!(v["runs_in_hand"], json!([]));
    assert!(v["made_ready_lost"]
        .as_str()
        .unwrap()
        .starts_with("nothing"));
    assert_eq!(v["other_plans_give_up"], json!([]));
    assert!(v["note"].as_str().unwrap().contains("Nothing is changed"));

    // Nothing is changed: the state's hash, the queue, the command state and
    // what the staff would do are as they were.
    assert_eq!(p.session.state_hash(), hash);
    assert_eq!(p.call("command_state", json!({})), state_before);
    assert_eq!(p.call("staff_orders", json!({})), orders_before);
    assert!(p.session.commands_handed_in().is_empty());
    // Only the calls themselves are in the transcript.
    assert_eq!(p.session.transcript().len(), transcript + 3);
}

#[test]
fn a_preview_of_a_limit_and_one_that_is_refused() {
    let mut p = Player::new();
    p.command_scenario_36();

    // A limit: the pool cut to ship 2, the run is ship 2's.
    let v = p.call(
        "preview",
        json!({"command": "set_limit", "limit": "forces", "intent": 1, "ships": [2], "source": 1}),
    );
    assert_eq!(v["preview"], 1);
    assert_eq!(v["intent"], 1);
    assert_eq!(runs(&v["as_things_stand"])[0], (vec![1], "360".to_string()));
    assert_eq!(runs(&v["with_change"])[0], (vec![2], "360".to_string()));
    assert_eq!(v["command"]["task_forces"], json!([]));

    // A reserve is for a base, and read as about the intent that loads there.
    let v = p.call(
        "preview",
        json!({"command": "set_limit", "limit": "reserve", "base": 1, "supply_t": "4900", "fuel_t": 0}),
    );
    assert_eq!(v["preview"], 2);
    assert_eq!(v["intent"], 1);

    // A bad command is refused with the reason, as the tool would, and uses
    // no number.
    let a = p.call(
        "preview",
        json!({"command": "set_intent", "base": 9, "floor_days": 30}),
    );
    assert_eq!(a["kind"], "command");
    assert_eq!(a["tool"], "preview");
    assert_eq!(a["status"], "refused");
    assert!(a["reason"].as_str().unwrap().contains("base:9"), "{a}");
    let v = p.call(
        "preview",
        json!({"command": "set_intent", "intent": 1, "base": 2, "floor_days": 31}),
    );
    assert_eq!(v["preview"], 3);

    // What cannot be previewed is an error of the call.
    for bad in [
        json!({}),
        json!({"command": "hold", "ship": 1}),
        json!({"command": "set_intent", "base": 2, "floor_days": 30, "clear": true}),
        json!({"command": "set_intent", "base": 2, "floor_days": 30, "plan": 1}),
    ] {
        p.try_call("preview", bad.clone()).expect_err("an error");
    }
}

#[test]
fn approving_a_good_preview_queues_exactly_what_it_showed() {
    let mut p = Player::new();
    p.command_scenario_36();
    let change = json!({"command": "set_intent", "intent": 1, "base": 2, "floor_days": 40, "interval_days": 15});
    let first = p.call("preview", change.clone());
    let second = p.call("preview", change);
    // A second preview gets the next number.
    assert_eq!(first["preview"], 1);
    assert_eq!(second["preview"], 2);

    let a = p.call("approve", json!({"preview": 2}));
    assert_eq!(a["status"], "queued", "{a}");
    assert_eq!(a["tool"], "approve");
    assert_eq!(a["preview"], 2);
    assert_eq!(a["command"], second["command"]);
    assert_eq!(a["text"], second["shown"]);
    assert_eq!(a["queued"], 3);
    let state = p.call("command_state", json!({}));
    assert_eq!(state["queued"][2]["command"], second["command"]);
    assert_eq!(state["intents"][0]["floor_days"], 40);

    // What the staff does is what the preview said.
    let orders = p.call("staff_orders", json!({}));
    assert_eq!(orders["orders"][1]["order"]["weight_t"], "460");

    // The commands were queued as the tools queue them.
    p.call("end_day", json!({}));
    let handed: Vec<&str> = p.session.commands_handed_in()[&1]
        .iter()
        .map(|(_, c)| c.name())
        .collect();
    assert_eq!(handed, ["set_intent", "set_limit", "set_intent"]);
}

#[test]
fn a_preview_with_a_new_intent_number_chosen_is_approved_as_it_was_shown() {
    let mut p = Player::new();
    let v = p.call(
        "preview",
        json!({"command": "set_intent", "base": 2, "floor_days": 30}),
    );
    assert_eq!(v["intent"], 1);
    assert_eq!(v["as_things_stand"]["runs"], json!([]));
    let a = p.call("approve", json!({"preview": 1}));
    assert_eq!(a["status"], "queued", "{a}");
    assert_eq!(a["intent"], 1);
    assert_eq!(a["command"]["intent"], 1);
}

#[test]
fn queuing_anything_after_a_preview_voids_it_and_approve_says_why() {
    let mut p = Player::new();
    p.command_scenario_36();
    let change = json!({"command": "set_intent", "intent": 1, "base": 2, "floor_days": 40});

    // A command queued after it.
    p.call("preview", change.clone());
    p.call("hold", json!({"ship": 1}));
    let a = p.call("approve", json!({"preview": 1}));
    assert_eq!(a["kind"], "command");
    assert_eq!(a["status"], "refused", "{a}");
    assert_eq!(a["queued"], 3);
    let reason = a["reason"].as_str().unwrap();
    assert!(
        reason.starts_with(
            "preview 1 is void: something was queued after it (you queued: hold ship:1)"
        ),
        "{reason}"
    );
    assert_eq!(a["command"]["floor_days"], 40);
    // Nothing was queued by the refusal.
    let state = p.call("command_state", json!({}));
    assert_eq!(state["queued"].as_array().unwrap().len(), 3);

    // An order queued after it.
    let v = p.call("preview", change.clone());
    assert_eq!(v["preview"], 2);
    p.call(
        "issue_orders",
        json!({"orders": [{"unit": "base:1", "order": {"kind": "form_task_force", "ships": [2]}}]}),
    );
    let a = p.call("approve", json!({"preview": 2}));
    assert_eq!(a["status"], "refused", "{a}");
    assert!(
        a["reason"]
            .as_str()
            .unwrap()
            .starts_with("preview 2 is void: something was queued after it (1 order was queued)"),
        "{a}"
    );

    // A queue that was emptied.
    let v = p.call("preview", change.clone());
    assert_eq!(v["preview"], 3);
    p.call("hold", json!({"ship": 2, "clear": true}));
    let a = p.call("approve", json!({"preview": 3}));
    assert_eq!(a["status"], "refused", "{a}");
    assert!(a["reason"].as_str().unwrap().contains("void"), "{a}");

    // A preview is void once it is approved: what was queued is what it was.
    let v = p.call("preview", change);
    let n = v["preview"].as_u64().unwrap();
    let a = p.call("approve", json!({"preview": n}));
    assert_eq!(a["status"], "queued", "{a}");
    let a = p.call("approve", json!({"preview": n}));
    assert_eq!(a["status"], "refused", "{a}");
    assert!(a["reason"].as_str().unwrap().contains("void"), "{a}");
}

#[test]
fn resolving_a_day_voids_a_preview() {
    let mut p = Player::new();
    p.command_scenario_36();
    p.call(
        "preview",
        json!({"command": "set_intent", "intent": 1, "base": 2, "floor_days": 40}),
    );
    p.call("end_day", json!({}));
    let a = p.call("approve", json!({"preview": 1}));
    assert_eq!(a["status"], "refused", "{a}");
    assert_eq!(
        a["reason"],
        "preview 1 is void: a day was resolved after it"
    );
    assert_eq!(a["for_day"], 2);
}

#[test]
fn approve_refuses_what_is_not_a_preview_of_the_side() {
    let mut p = Player::new();
    p.command_scenario_36();
    let error = p
        .try_call("approve", json!({"preview": 7}))
        .expect_err("no such preview");
    assert!(error.message.contains("preview 7"), "{error}");
    p.call(
        "preview",
        json!({"command": "set_intent", "intent": 1, "base": 2, "floor_days": 40}),
    );
    for bad in [
        json!({"preview": 1, "plan": 1}),
        json!({"preview": 1, "clear": true}),
        json!({"preview": "one"}),
    ] {
        p.try_call("approve", bad).expect_err("an error");
    }
    // The plan form is as it was.
    let a = p.call("approve", json!({"plan": 1}));
    assert_eq!(a["status"], "queued", "{a}");
    assert!(a.get("preview").is_none());
}

#[test]
fn a_preview_names_the_holds_that_keep_ships_from_the_staff_and_the_runs_in_hand() {
    let mut p = Player::new();
    p.on_day_2();
    // Ship 1's run (360 t) has sailed on day 2 or is loaded; hold ship 2 and
    // then the force of ship 1, so the pool is held whole.
    p.call("hold", json!({"ship": 2}));
    let v = p.call(
        "preview",
        json!({"command": "set_intent", "intent": 1, "base": 2, "floor_days": 40}),
    );
    assert_eq!(v["day"], 2);
    assert!(v["held"][0]
        .as_str()
        .unwrap()
        .starts_with("ship 2 is held by your hold command of day 2"));
    // Run 1 is in hand: the change is not of the base, so it goes on as
    // ordered.
    let in_hand = v["runs_in_hand"].as_array().unwrap();
    assert_eq!(in_hand.len(), 1, "{v}");
    assert!(in_hand[0].as_str().unwrap().starts_with("run 1 is "));
    assert!(in_hand[0].as_str().unwrap().ends_with("goes on as ordered"));

    // A change of base: a run that has sailed goes on to the old base.
    let v = p.call(
        "preview",
        json!({"command": "set_intent", "intent": 1, "base": 1, "floor_days": 30}),
    );
    let in_hand = v["runs_in_hand"].as_array().unwrap();
    assert_eq!(in_hand.len(), 1, "{v}");
    assert_eq!(in_hand[0], "run 1 has sailed and goes on to base 2");
}

#[test]
fn a_held_ship_takes_its_force_out_of_the_pool_and_the_preview_says_so() {
    let mut p = Player::new();
    p.on_day_2();
    // Task force 1 holds ship 1: holding the force names the force.
    p.call("hold", json!({"task_force": 1}));
    let v = p.call(
        "preview",
        json!({"command": "set_limit", "limit": "risk", "intent": 1, "risk": "press"}),
    );
    assert_eq!(
        v["held"],
        json!(["task force 1 is held by your hold command of day 2, so ship 1 of the pool is not free either"])
    );
}
