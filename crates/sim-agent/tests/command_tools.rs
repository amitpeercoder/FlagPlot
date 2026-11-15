//! The tools of a seat with a staff, over scenario 36 (decision 0107, "As
//! built", "The tools"): `command_state`, `plans`, `propose`, `staff_orders`,
//! `why` and the six commands a seat queues.
//!
//! Scenario 36 is one convoy: the Allied side keeps base 2 at 30 days of
//! supply from base 1 with ships 1 and 2 (the hand calculation is in
//! `tests/scenario_36_one_convoy.rs`). Here the Allied seat is a player who
//! says by the tools what the scenario's `[[command]]` tables say, and the
//! staff does the rest. The figures asserted are the hand calculation's: on
//! day 1 the first run carries 360 t in ship 1 and is loaded on day 1, sails
//! on day 2, is due on day 2 and its ship is free on day 5; the second run
//! carries 150 t from day 16.

mod support;

use std::collections::BTreeMap;

use serde_json::{json, Value};
use sim_agent::{CallError, Seat, Session, COMMAND_TOOLS, STAFF_DEFAULT_INTERVAL_DAYS, TOOLS};
use sim_core::state::Side;
use support::{load, schemas};

/// Scenario 36 with the Allied seat a player, no scripted command and a
/// staff for each side if `staffs`.
fn session(staffs: bool) -> Session {
    let mut scenario = load("36_one_convoy.toml", 1942);
    scenario.replace_commands(BTreeMap::new());
    let name = scenario.name.clone();
    let session = Session::new(
        scenario.world,
        &name,
        scenario.days,
        Seat::Player,
        Seat::Script(BTreeMap::new()),
    );
    if staffs {
        session.with_staffs(sim_staff::staffs())
    } else {
        session
    }
}

/// A player who writes down each call and its answer.
struct Player {
    session: Session,
    log: Vec<Value>,
    schemas: support::validator::Schemas,
}

impl Player {
    fn new() -> Player {
        Player {
            session: session(true),
            log: Vec::new(),
            schemas: schemas(),
        }
    }

    fn try_call(&mut self, tool: &str, args: Value) -> Result<Value, CallError> {
        let result = self.session.call(Side::Allied, tool, &args);
        let answer = match &result {
            Ok(answer) => answer.clone(),
            Err(error) => error.to_json(),
        };
        // Every answer fits the schema of its kind.
        let kind = answer["kind"].as_str().expect("an answer has a kind");
        let errors = self.schemas.check(&format!("{kind}.schema.json"), &answer);
        assert!(errors.is_empty(), "{tool} {args}: {errors:?}\n{answer}");
        self.log.push(json!({
            "seq": self.log.len() + 1,
            "tool": tool,
            "arguments": args,
            "answer": answer,
        }));
        result
    }

    fn call(&mut self, tool: &str, args: Value) -> Value {
        self.try_call(tool, args)
            .unwrap_or_else(|e| panic!("{tool}: {e}"))
    }
}

fn ints(value: &Value) -> Vec<u64> {
    value
        .as_array()
        .expect("a list")
        .iter()
        .map(|v| v.as_u64().expect("a whole number"))
        .collect()
}

#[test]
fn a_player_commands_scenario_36_by_the_tools() {
    let mut p = Player::new();

    // The two leaves start as the spec's default: prepare given, launch
    // withheld; nothing is set, so the staff does nothing.
    let state = p.call("command_state", json!({}));
    assert_eq!(state["for_day"], 1);
    assert_eq!(state["leaves"]["prepare"], true);
    assert_eq!(state["leaves"]["launch"], false);
    assert_eq!(state["intents"], json!([]));
    assert!(state["notes"][0]
        .as_str()
        .unwrap()
        .contains("does nothing for your side until you set an intent"));
    let idle = p.call("staff_orders", json!({}));
    assert_eq!(idle["orders"], json!([]));
    assert_eq!(idle["commands"], json!([]));
    assert_eq!(idle["no_orders"], json!([]));

    // An intent with no number takes the side's next free one.
    let a = p.call(
        "set_intent",
        json!({"base": 2, "floor_days": 30, "interval_days": 15}),
    );
    assert_eq!(a["status"], "queued", "{a}");
    assert_eq!(a["intent"], 1);
    assert_eq!(a["for_day"], 1);
    assert_eq!(a["queued"], 1);
    assert!(a["notes"][0].as_str().unwrap().contains("next free number"));
    assert!(a["notes"][1].as_str().unwrap().contains("no forces limit"));
    let a = p.call(
        "set_limit",
        json!({"limit": "forces", "intent": 1, "ships": [1, 2], "source": 1}),
    );
    assert_eq!(a["status"], "queued", "{a}");
    assert_eq!(a["command"]["task_forces"], json!([]));
    let a = p.call(
        "set_limit",
        json!({"limit": "reserve", "base": 1, "supply_t": "1000", "fuel_t": "0"}),
    );
    assert_eq!(a["status"], "queued", "{a}");
    let a = p.call("set_leave", json!({"leave": "launch", "given": true}));
    assert_eq!(a["status"], "queued", "{a}");
    assert_eq!(a["queued"], 4);

    // The command state as the coming day will find it.
    let state = p.call("command_state", json!({}));
    let intent = &state["intents"][0];
    assert_eq!(intent["number"], 1);
    assert_eq!(intent["in_force"], true);
    assert_eq!(intent["base"]["handle"], "base:2");
    assert_eq!(intent["floor_days"], 30);
    assert_eq!(intent["interval_days"], 15);
    assert_eq!(ints(&intent["limits"]["forces"]["ships"]), [1, 2]);
    assert_eq!(intent["limits"]["forces"]["source"]["handle"], "base:1");
    // Base 2 holds 100 t and uses 10 t a day: 10 days, below the floor from
    // the first day.
    assert_eq!(intent["projection"]["today"]["stock_t"], "100");
    assert_eq!(intent["projection"]["today"]["use_t"], "10");
    assert_eq!(intent["projection"]["today"]["days"], "10");
    assert_eq!(intent["projection"]["first_day_below_floor"], 1);
    assert_eq!(state["leaves"]["launch"], true);
    let reserve = &state["reserves"][0];
    assert_eq!(reserve["base"]["handle"], "base:1");
    assert_eq!(reserve["supply_t"], "5000");
    assert_eq!(reserve["supply_reserve_t"], "1000");
    assert_eq!(state["plans"][0]["number"], 1);
    assert_eq!(state["plans"][0]["state"], "draft");
    assert_eq!(state["queued"].as_array().unwrap().len(), 4);
    assert_eq!(state["queued"][1]["command"]["kind"], "set_limit");

    // The proposal: the first run is 360 t in ship 1, loaded on day 1,
    // sailing on day 2, due on day 2 and free on day 5; the second is 150 t
    // loaded on day 16.
    let proposal = p.call("propose", json!({"intent": 1}));
    assert_eq!(proposal["plan"]["state"], "draft");
    assert_eq!(proposal["displaces"], "nothing: one intent");
    assert_eq!(proposal["projection"]["today"]["days"], "10");
    assert_eq!(proposal["projection"]["first_day_below_floor"], 1);
    let first = &proposal["runs"][0];
    assert_eq!(first["load_t"], "360");
    assert_eq!(ints(&first["ships"]), [1]);
    assert_eq!(
        first["days"],
        json!({"load": 1, "sail": 2, "arrive": 2, "free": 5})
    );
    assert!(first["reason"]["worked"]
        .as_str()
        .unwrap()
        .contains("360 t"));
    let second = &proposal["runs"][1];
    assert_eq!(second["load_t"], "150");
    assert_eq!(second["days"]["load"], 16);
    assert_eq!(proposal["would_hand_in"]["for_day"], 1);

    // Approve the plan: a draft becomes preparing, and the staff, with leave
    // to launch, releases it the same day.
    let a = p.call("approve", json!({"plan": 1}));
    assert_eq!(a["status"], "queued", "{a}");
    assert_eq!(a["text"], "approve plan:1");

    // What the staff would hand in on day 1.
    let orders = p.call("staff_orders", json!({}));
    assert_eq!(orders["for_day"], 1);
    let given = orders["orders"].as_array().unwrap();
    assert_eq!(given.len(), 2);
    assert_eq!(given[0]["unit"], "base:1");
    assert_eq!(
        given[0]["order"],
        json!({"kind": "form_task_force", "ships": [1]})
    );
    // The force the order will make is the next number the side would give.
    assert_eq!(given[1]["unit"], "task_force:1");
    assert_eq!(
        given[1]["order"],
        json!({"kind": "load", "cargo": "supply", "weight_t": "360"})
    );
    let kinds: Vec<&str> = orders["commands"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["kind"].as_str().unwrap())
        .collect();
    assert_eq!(kinds, ["approve", "add_run", "commit_ships"]);
    assert_eq!(orders["commands"][1]["load_t"], "360");
    assert_eq!(ints(&orders["commands"][1]["ships"]), [1]);
    assert_eq!(orders["commands"][2]["to_day"], 4);
    // Every decision has the five parts, and no engine id is left in them.
    for decision in given.iter().chain(orders["commands"].as_array().unwrap()) {
        let reason = &decision["reason"];
        for part in ["serves", "evidence", "binding", "worked", "would_change"] {
            assert!(reason.get(part).is_some(), "{part}: {decision}");
        }
        assert_eq!(reason["serves"], json!({"intent": 1, "plan": 1}));
        assert!(!reason.to_string().contains("1000001"), "{reason}");
    }
    assert!(given[0]["reason"]["worked"]
        .as_str()
        .unwrap()
        .contains("Load = max(0, 300 - 90) + 150 (15 days of use) = 360 t"));

    // The plan as it would stand: preparing is not shown yet (the staff's own
    // approval comes at the day's step 4), but the draft has become
    // preparing by the player's approval.
    let plans = p.call("plans", json!({}));
    assert_eq!(plans["plans"][0]["state"], "preparing");
    assert_eq!(plans["plans"][0]["baseline_day"], 1);

    // The day is resolved; the game is what the engine does with them.
    let end = p.call("end_day", json!({}));
    assert_eq!(end["resolved"], true);
    assert_eq!(end["day"], 1);
    // The commands were handed in, in the order queued.
    let handed: Vec<&str> = p.session.commands_handed_in()[&1]
        .iter()
        .map(|(_, c)| c.name())
        .collect();
    assert_eq!(
        handed,
        [
            "set_intent",
            "set_limit",
            "set_limit",
            "set_leave",
            "approve"
        ]
    );

    // On day 2 the plan is released, its run made and ship 1 committed.
    let state = p.call("command_state", json!({}));
    assert_eq!(state["for_day"], 2);
    assert_eq!(state["queued"], json!([]));
    assert_eq!(state["plans"][0]["state"], "released");
    let run = &state["plans"][0]["runs"][0];
    assert_eq!(run["load_t"], "360");
    assert_eq!(
        run["days"],
        json!({"load": 1, "sail": 2, "arrive": 2, "free": 5})
    );
    assert_eq!(state["commitments"][0]["ship"], 1);
    assert_eq!(state["commitments"][0]["from_day"], 1);
    assert_eq!(state["commitments"][0]["to_day"], 4);
    assert_eq!(state["reserves"][0]["supply_t"], "4640");
    assert_eq!(state["reserves"][1]["supply_t"], "90");
    let plans = p.call("plans", json!({}));
    let run = &plans["plans"][0]["runs"][0];
    assert_eq!(run["phase"], "under_way");
    assert_eq!(run["ship_states"][0]["ship"], 1);
    assert_eq!(run["ship_states"][0]["supply_aboard_t"], "360");
    assert!(run["ship_states"][0]["at"]
        .as_str()
        .unwrap()
        .contains("task_force:1"));

    // The staff's decisions of day 1 are listed with the numbers `why` takes.
    let decisions = state["staff_decisions_yesterday"].as_array().unwrap();
    let find = |what: &str, text: &str| {
        decisions
            .iter()
            .find(|d| d["for"] == what && d["summary"].as_str().unwrap().contains(text))
            .unwrap_or_else(|| panic!("no {what} {text}: {decisions:?}"))["number"]
            .as_u64()
            .unwrap()
    };
    let forming = find("order", "base:1: form_task_force");
    let loading = find("order", "task_force:1: load");
    let approving = find("command", "approve plan:1");
    let adding = find("command", "add a run of 360 t");

    // An order the staff gave: its unit, its body and that it was accepted.
    let why = p.call("why", json!({"number": forming}));
    assert_eq!(why["day"], 1);
    assert_eq!(why["for"]["type"], "order");
    assert_eq!(why["for"]["unit"], "base:1");
    assert_eq!(
        why["for"]["body"],
        json!({"kind": "form_task_force", "ships": [1]})
    );
    assert_eq!(why["for"]["accepted"], true);
    assert_eq!(why["for"]["intent"], 1);
    assert!(why["reason"]["worked"].as_str().unwrap().contains("360 t"));
    let why = p.call("why", json!({"number": loading}));
    assert_eq!(why["for"]["unit"], "task_force:1");
    assert_eq!(why["for"]["body"]["weight_t"], "360");
    assert_eq!(why["for"]["accepted"], true);
    // A command the staff gave: the command and that it was taken.
    let why = p.call("why", json!({"number": approving}));
    assert_eq!(why["for"]["type"], "command");
    assert_eq!(why["for"]["text"], "approve plan:1");
    assert_eq!(why["for"]["issuer"], "staff");
    assert_eq!(why["for"]["taken"], true);
    let why = p.call("why", json!({"number": adding}));
    assert_eq!(why["for"]["command"]["kind"], "add_run");
    assert_eq!(why["for"]["command"]["load_t"], "360");
    assert_eq!(ints(&why["for"]["command"]["ships"]), [1]);
    // A number that is no staff reason is not known, as `explain` says of an
    // event the side may not see.
    let unknown = p.call("why", json!({"number": 1}));
    assert_eq!(unknown["kind"], "not_known");
    assert_eq!(unknown["query"], "why");
    assert_eq!(p.call("why", json!({"number": 99999})), unknown);
    assert_eq!(p.call("why", json!({"number": 0})), unknown);

    // Day 2: the staff routes the force it formed.
    let orders = p.call("staff_orders", json!({}));
    assert_eq!(orders["for_day"], 2);
    assert_eq!(orders["orders"][0]["unit"], "task_force:1");
    assert_eq!(orders["orders"][0]["order"]["kind"], "set_route");

    // A day with no order has a number too: on day 3 the staff gives none
    // and says which reading the next run waits for.
    p.call("end_day", json!({}));
    p.call("end_day", json!({}));
    let state = p.call("command_state", json!({}));
    assert_eq!(state["for_day"], 4);
    let none = state["staff_decisions_yesterday"]
        .as_array()
        .unwrap()
        .iter()
        .find(|d| d["for"] == "no_order")
        .expect("a day with no order has a reason");
    let why = p.call("why", json!({"number": none["number"]}));
    assert_eq!(why["for"], json!({"type": "no_order"}));
    assert!(why["reason"]["worked"].as_str().unwrap().contains("run"));

    // The transcript, for whoever reads it.
    if let Ok(path) = std::env::var("FLAGPILOT_TOOLS_TRANSCRIPT") {
        std::fs::write(path, serde_json::to_string_pretty(&p.log).unwrap())
            .expect("the transcript is written");
    }
}

#[test]
fn a_command_is_checked_at_once_and_a_refused_one_is_not_queued() {
    let mut p = Player::new();
    let a = p.call(
        "set_intent",
        json!({"intent": 1, "base": 2, "floor_days": 30}),
    );
    assert_eq!(a["status"], "queued");
    assert_eq!(a["intent"], Value::Null);
    // A base that is not the side's: refused, with the base as written.
    let refused = |a: &Value, reason: &str| {
        assert_eq!(a["status"], "refused", "{a}");
        assert_eq!(a["reason"], reason, "{a}");
    };
    let a = p.call("set_intent", json!({"base": 77, "floor_days": 30}));
    refused(&a, "base:77 is not one of the side's bases");
    assert_eq!(a["queued"], 1);
    // The number given is taken by the intent queued before.
    let a = p.call("set_intent", json!({"base": 2, "floor_days": 30}));
    assert_eq!(a["status"], "queued");
    assert_eq!(a["intent"], 2, "{a}");
    let a = p.call(
        "set_intent",
        json!({"intent": 0, "base": 2, "floor_days": 30}),
    );
    refused(&a, "intent:0 is not a number an intent may take");
    let a = p.call(
        "set_intent",
        json!({"intent": 4, "base": 2, "floor_days": 0}),
    );
    refused(&a, "a number of days is 0");
    // A ship that is not the side's, as the side wrote it.
    let a = p.call(
        "set_limit",
        json!({"limit": "forces", "intent": 1, "ships": [1, 9], "source": 1}),
    );
    refused(&a, "ship:9 is not one of the side's ships");
    let a = p.call(
        "set_limit",
        json!({"limit": "forces", "intent": 1, "task_forces": [5], "source": 1}),
    );
    refused(&a, "task_force:5 is not one of the side's forces");
    let a = p.call(
        "set_limit",
        json!({"limit": "forces", "intent": 1, "ships": [1], "source": 3}),
    );
    refused(&a, "base:3 is not one of the side's bases");
    let a = p.call(
        "set_limit",
        json!({"limit": "risk", "intent": 8, "risk": "press"}),
    );
    refused(&a, "intent:8 is not in force");
    let a = p.call(
        "set_limit",
        json!({"limit": "reserve", "base": 1, "supply_t": "-5", "fuel_t": 0}),
    );
    refused(&a, "-5 t is not an allowed weight");
    let a = p.call("approve", json!({"plan": 5}));
    refused(&a, "plan:5 does not exist");
    let a = p.call("withdraw_intent", json!({"intent": 3}));
    refused(&a, "intent:3 is not in force");
    // The staff's own commands are not a seat's: a command tool is one of
    // six, and an argument that is none of its is refused as a whole.
    let error = p
        .try_call("set_intent", json!({"base": 2, "floor_days": 30, "bse": 1}))
        .expect_err("refused as a whole");
    assert!(error
        .message
        .contains("\"bse\" is not an argument of set_intent"));
    assert!(error.allowed.contains(&"floor_days".to_string()));
    let error = p
        .try_call("set_leave", json!({"leave": "launch"}))
        .expect_err("refused as a whole");
    assert!(error.message.contains("malformed command"), "{error}");
    // Only the two commands queued stand.
    let state = p.call("command_state", json!({}));
    assert_eq!(state["queued"].as_array().unwrap().len(), 2);
    // Withdrawing an intent that is queued is taken after it.
    let a = p.call("withdraw_intent", json!({"intent": 2}));
    assert_eq!(a["status"], "queued");
    let state = p.call("command_state", json!({}));
    assert_eq!(state["intents"][1]["in_force"], false);
    assert_eq!(state["plans"][1]["state"], "cancelled");
    // `clear` empties the queue first, as it does for orders.
    let a = p.call(
        "set_leave",
        json!({"leave": "prepare", "given": false, "clear": true}),
    );
    assert_eq!(a["status"], "queued");
    assert_eq!(a["queued"], 1);
    let state = p.call("command_state", json!({}));
    assert_eq!(state["intents"], json!([]));
    assert_eq!(state["leaves"]["prepare"], false);
    // A refused command with `clear` still clears.
    let a = p.call("approve", json!({"plan": 1, "clear": true}));
    refused(&a, "plan:1 does not exist");
    assert_eq!(a["queued"], 0);
    // The queue closes with the day.
    p.call("end_day", json!({}));
    let state = p.call("command_state", json!({}));
    assert_eq!(state["for_day"], 2);
    assert_eq!(state["queued"], json!([]));
}

#[test]
fn an_enemy_base_and_a_base_that_does_not_exist_are_refused_alike() {
    let scenario = load("06_rabaul_guadalcanal.toml", 1942);
    let enemy = scenario
        .world
        .side_view(Side::Allied)
        .enemy_bases()
        .next()
        .expect("the Allied side knows an enemy base")
        .id
        .raw();
    let (mut session, _) = support::play::session_of(scenario, &[Side::Allied]);
    session = session.with_staffs(sim_staff::staffs());
    let mut reasons = Vec::new();
    for base in [enemy, 99_999] {
        let a = session
            .call(
                Side::Allied,
                "set_intent",
                &json!({"base": base, "floor_days": 10}),
            )
            .expect("answered");
        assert_eq!(a["status"], "refused", "{a}");
        assert_eq!(a["queued"], 0);
        reasons.push(
            a["reason"]
                .as_str()
                .unwrap()
                .replace(&base.to_string(), "N"),
        );
        // Nothing more than "not one of the side's bases" is said of it.
        assert!(!a.to_string().contains("enemy"), "{a}");
    }
    assert_eq!(reasons[0], reasons[1]);
    assert_eq!(reasons[0], "base:N is not one of the side's bases");
}

#[test]
fn a_session_without_staffs_has_none_of_the_tools() {
    let mut s = session(false);
    assert!(!s.has_staff(Side::Allied));
    assert_eq!(s.tools(Side::Allied), TOOLS.to_vec());
    for tool in COMMAND_TOOLS {
        let error = s
            .call(Side::Allied, tool, &json!({}))
            .expect_err("not a tool");
        assert_eq!(error.message, format!("\"{tool}\" is not a tool"));
        assert_eq!(error.allowed, TOOLS.to_vec());
    }
    // And a session with staffs lists them after the others.
    let with = session(true);
    let tools = with.tools(Side::Allied);
    assert_eq!(tools.len(), TOOLS.len() + COMMAND_TOOLS.len());
    assert_eq!(&tools[..TOOLS.len()], &TOOLS);
    assert_eq!(&tools[TOOLS.len()..], &COMMAND_TOOLS);
    // Without staffs the game is played as it was: nothing is queued.
    assert!(s.commands_handed_in().is_empty());
}

#[test]
fn asking_the_staff_changes_nothing_in_the_game() {
    let mut p = Player::new();
    let before = p.session.state_hash();
    p.call("set_intent", json!({"base": 2, "floor_days": 30}));
    p.call(
        "set_limit",
        json!({"limit": "forces", "intent": 1, "ships": [1, 2], "source": 1}),
    );
    for tool in ["command_state", "plans", "staff_orders"] {
        p.call(tool, json!({}));
    }
    p.call("propose", json!({"intent": 1}));
    assert_eq!(p.session.state_hash(), before);
    // An intent not in force has no proposal.
    let error = p
        .try_call("propose", json!({"intent": 4}))
        .expect_err("no such intent");
    assert!(error.message.contains("intent:4 is not an intent in force"));
}

#[test]
fn an_intent_with_no_interval_is_worked_at_the_staffs_default() {
    let mut p = Player::new();
    p.call("set_intent", json!({"base": 2, "floor_days": 30}));
    p.call(
        "set_limit",
        json!({"limit": "forces", "intent": 1, "ships": [1, 2], "source": 1}),
    );
    let state = p.call("command_state", json!({}));
    assert_eq!(state["intents"][0]["interval_days"], Value::Null);
    assert_eq!(
        state["intents"][0]["interval"],
        format!("not set: the staff uses {STAFF_DEFAULT_INTERVAL_DAYS}")
    );
    // The staff's run is the one the interval of 15 days gives: 360 t, and
    // what it would hand in says the days of use it adds.
    let proposal = p.call("propose", json!({"intent": 1}));
    assert_eq!(proposal["runs"][0]["load_t"], "360");
    assert!(proposal["would_hand_in"]["orders"][0]["reason"]["worked"]
        .as_str()
        .unwrap()
        .contains(&format!("({STAFF_DEFAULT_INTERVAL_DAYS} days of use)")));
}

#[test]
fn leave_to_launch_withheld_holds_the_plan_for_the_commander() {
    let mut p = Player::new();
    p.call(
        "set_intent",
        json!({"base": 2, "floor_days": 30, "interval_days": 15}),
    );
    p.call(
        "set_limit",
        json!({"limit": "forces", "intent": 1, "ships": [1, 2], "source": 1}),
    );
    // With launch withheld (the default) the staff prepares the draft, makes
    // its run and loads it, and waits for the commander to release it.
    let orders = p.call("staff_orders", json!({}));
    let kinds = |orders: &Value| -> Vec<String> {
        orders["commands"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| c["kind"].as_str().unwrap().to_string())
            .collect()
    };
    assert_eq!(kinds(&orders), ["approve", "add_run", "commit_ships"]);
    assert_eq!(orders["orders"].as_array().unwrap().len(), 2);
    let state = p.call("command_state", json!({}));
    assert_eq!(state["leaves"]["launch"], false);
    // The commander approves: the plan is preparing and the staff does not
    // approve it again, since the leave to launch is withheld.
    p.call("approve", json!({"plan": 1}));
    let state = p.call("command_state", json!({}));
    assert_eq!(state["plans"][0]["state"], "preparing");
    assert!(state["notes"]
        .as_array()
        .unwrap()
        .iter()
        .any(|n| n.as_str().unwrap().contains("approve it to release it")));
    let orders = p.call("staff_orders", json!({}));
    assert_eq!(kinds(&orders), ["add_run", "commit_ships"]);
    // A second approve releases it.
    let a = p.call("approve", json!({"plan": 1}));
    assert_eq!(a["status"], "queued", "{a}");
    let plans = p.call("plans", json!({}));
    assert_eq!(plans["plans"][0]["state"], "released");
    let orders = p.call("staff_orders", json!({}));
    assert_eq!(kinds(&orders), ["add_run", "commit_ships"]);
    assert_eq!(orders["orders"][0]["order"]["kind"], "form_task_force");
}
