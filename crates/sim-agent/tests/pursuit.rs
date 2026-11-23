//! A force asks leave to go after what it has found, as a seat meets it
//! (decision 0214, stage C): the stop in the evening before the day is
//! played, the two tools, and what `command_state` and `report` say of it.
//!
//! The staff that asks is not the doctrine's (it lands in another stage): it
//! is a small staff of this test's own, which hands in a notice that task
//! force 1 asks leave to pursue contact 1 until the side's command state
//! holds a pursuit for task force 1 (an answer taken in). The game is
//! scenario 58's, which has an intent in force for task force 1 from its
//! first day; no pulse of it is asked to be understood, and what the asking
//! says is the test staff's own invention: a carrier at (16,0) and (10,0) to
//! go to, in 400 / 30 hours.

mod support;

use std::collections::BTreeMap;

use serde_json::{json, Value};
use sim_agent::{CallError, Seat, Session, COMMAND_TOOLS};
use sim_core::command::{
    Evidence, NoticeBasis, NoticeKind, PlanId, PursuitFind, PursuitNotice, Reason, Serves, Staff,
    StaffDecisions, StaffNotice,
};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{IntentId, TaskForceId};
use sim_core::refdata::ShipKind;
use sim_core::state::Side;
use sim_core::view::SideView;
use support::{load, schemas};

/// The test's staff: asks for task force 1 and contact 1 while the side has
/// no pursuit for the force.
struct AskingStaff;

impl Staff for AskingStaff {
    fn decide(&self, view: SideView<'_>) -> StaffDecisions {
        let state = view.command_state();
        let force = TaskForceId::new(1);
        if state.intents_in_force().next().is_none() || state.pursuits.contains_key(&force) {
            return StaffDecisions::default();
        }
        StaffDecisions {
            no_orders: vec![Reason {
                serves: Serves {
                    intent: Some(IntentId::new(1)),
                    plan: Some(PlanId::new(1)),
                    amendment: None,
                    statement: String::new(),
                    chain: Vec::new(),
                    level: None,
                },
                evidence: vec![Evidence {
                    what: "a contact".to_string(),
                    day: 1,
                }],
                binding: vec!["the night's steaming".to_string()],
                worked: "400 / 30 hours".to_string(),
                would_change: vec!["leave to pursue".to_string()],
                answer_of: None,
                cautions: Vec::new(),
                limits: Vec::new(),
            }],
            notices: vec![StaffNotice {
                kind: NoticeKind::AsksLeaveToPursue,
                plan: PlanId::new(1),
                run: None,
                rests_on: NoticeBasis::NoOrder(0),
                pursuit: Some(PursuitNotice {
                    force,
                    intent: IntentId::new(1),
                    find: Some(PursuitFind {
                        contact: 1,
                        kind: ShipKind::Carrier,
                        age: 0,
                        reported_at: Hex::new(16, 0),
                        to: Hex::new(10, 0),
                        hours: Some(Fixed::from_int(400) / Fixed::from_int(30)),
                    }),
                }),
            }],
            ..StaffDecisions::default()
        }
    }
}

/// A staff that does nothing: the Japanese side's.
struct Quiet;

impl Staff for Quiet {
    fn decide(&self, _view: SideView<'_>) -> StaffDecisions {
        StaffDecisions::default()
    }
}

fn session(inside_day: bool) -> Session {
    let mut scenario = load("58_a_carrier_force_closes.toml", 1942);
    let commands = scenario.scripted_commands().clone();
    scenario.replace_commands(BTreeMap::new());
    let title = scenario.name.clone();
    let mut staffs: BTreeMap<Side, Box<dyn Staff + Send>> = BTreeMap::new();
    staffs.insert(Side::Allied, Box::new(AskingStaff));
    staffs.insert(Side::Japanese, Box::new(Quiet));
    let session = Session::new(
        scenario.world,
        &title,
        scenario.days,
        Seat::Player,
        Seat::Script(BTreeMap::new()),
    )
    .with_staffs(staffs)
    .with_commands(commands);
    if inside_day {
        session.with_orders_inside_a_day(true, Default::default())
    } else {
        session
    }
}

struct Player {
    session: Session,
    schemas: support::validator::Schemas,
}

impl Player {
    fn new(inside_day: bool) -> Player {
        Player {
            session: session(inside_day),
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
}

fn counts(a: &Value) -> (u64, u64, u64) {
    let d = &a["days_resolved"];
    (
        d["first"].as_u64().unwrap(),
        d["last"].as_u64().unwrap(),
        d["count"].as_u64().unwrap(),
    )
}

#[test]
fn the_two_tools_are_command_tools_after_cancel_plan() {
    let at = |tool: &str| COMMAND_TOOLS.iter().position(|t| *t == tool).unwrap();
    assert_eq!(at("set_pursuit"), at("cancel_plan") + 1);
    assert_eq!(at("answer_pursuit"), at("set_pursuit") + 1);
    assert_eq!(COMMAND_TOOLS.len(), 21);
}

#[test]
fn a_run_stops_before_the_day_for_an_asking_and_changes_nothing() {
    let mut p = Player::new(false);
    let hash = p.session.state_hash();
    let a = p.call("run", json!({"days": 3}));
    assert_eq!(a["stopped"], "asks_leave", "{a}");
    assert_eq!(counts(&a), (1, 0, 0), "{a}");
    assert_eq!(a["day"], 0);
    assert_eq!(a["fired"], json!([]));
    assert_eq!(a["game_over"], false);
    let asking = a["asking"].as_array().expect("an asking");
    assert_eq!(asking.len(), 1, "{a}");
    let one = &asking[0];
    assert_eq!(one["task_force"], 1);
    assert_eq!(one["intent"], 1);
    assert_eq!(one["contact"], 1);
    assert_eq!(one["kind"], "carrier");
    assert_eq!(one["age_pulses"], 0);
    assert_eq!(one["reported_at"], json!({"q": 16, "r": 0}));
    assert_eq!(one["to"], json!({"q": 10, "r": 0}));
    assert_eq!(
        one["hours"],
        json!((Fixed::from_int(400) / Fixed::from_int(30)).to_string())
    );
    assert_eq!(
        one["interrupt"],
        json!({"kind": "asks_leave_to_pursue", "task_force": 1})
    );
    let summary = a["summary"].as_str().unwrap();
    assert!(
        summary.starts_with("Resolved no day; stopped at dusk before day 1: task force 1 asks leave to pursue contact 1, a carrier reported at (16,0), to (10,0), 13.333333 hours; intent 1 would wait"),
        "{summary}"
    );
    // The staff's orders of the first day are those it would hand in.
    assert_eq!(a["staff_orders"]["for_day"], 1);
    let notice = &a["staff_orders"]["notices"][0];
    assert_eq!(notice["kind"], "asks_leave_to_pursue");
    assert_eq!(notice["pursuit"]["force"], 1);
    assert_eq!(notice["pursuit"]["find"]["contact"], 1);
    // Nothing in the world changed.
    assert_eq!(p.session.state_hash(), hash);
    assert_eq!(p.session.day(), 0);
    let state = p.call("command_state", json!({}));
    assert_eq!(state["for_day"], 1);
}

#[test]
fn the_next_call_plays_the_day_without_stopping_before_it_again() {
    let mut p = Player::new(false);
    let a = p.call("run", json!({"days": 1}));
    assert_eq!(a["stopped"], "asks_leave");
    // Unanswered: the day is played. The asking fires the standing interrupt
    // that day, and that is no stop after it: the stop was before.
    let b = p.call("run", json!({"days": 1}));
    assert_eq!(b["stopped"], "days", "{b}");
    assert_eq!(counts(&b), (1, 1, 1), "{b}");
    assert!(b.get("asking").is_none());
    assert_eq!(b["fired"].as_array().unwrap().len(), 1, "{b}");
    assert_eq!(b["fired"][0]["standing"], "asks_leave_to_pursue");
    assert_eq!(b["fired"][0]["task_force"], 1);
    // The side was asked again at dusk of day 2 and the interrupt is no
    // longer armed: no stop, the day is played.
    let c = p.call("run", json!({"days": 1}));
    assert_eq!(c["stopped"], "days", "{c}");
    assert_eq!(counts(&c), (2, 2, 1));
}

#[test]
fn an_answer_given_at_the_stop_is_taken_in_before_the_staff_is_asked() {
    let mut p = Player::new(false);
    let a = p.call("run", json!({"days": 1}));
    assert_eq!(a["stopped"], "asks_leave");
    let q = p.queued(
        "answer_pursuit",
        json!({"task_force": 1, "contact": 1, "allow": true}),
    );
    assert_eq!(q["command"]["kind"], "answer_pursuit");
    assert_eq!(q["command"]["task_force"], 1);
    assert_eq!(q["command"]["contact"], 1);
    assert_eq!(q["command"]["allow"], true);
    // The preview has the command taken in: the pursuit is in its command
    // state, and the staff, asked on it, hands in no asking.
    let state = p.call("command_state", json!({}));
    assert_eq!(
        state["pursuits"],
        json!([{"task_force": 1, "by": "answer", "contact": 1, "since_day": 1}]),
        "{state}"
    );
    let staff = p.call("staff_orders", json!({}));
    assert!(staff.get("notices").is_none(), "{staff}");
    // So the next call plays the day with no stop and nothing fired.
    let b = p.call("run", json!({"days": 1}));
    assert_eq!(b["stopped"], "days", "{b}");
    assert_eq!(b["fired"], json!([]));
    assert_eq!(b["staff_orders"].get("notices"), None);
    let state = p.call("command_state", json!({}));
    assert_eq!(state["pursuits"][0]["by"], "answer");
    // A no is kept for the contact.
    let mut p = Player::new(false);
    p.call("run", json!({"days": 1}));
    p.queued(
        "answer_pursuit",
        json!({"task_force": 1, "contact": 1, "allow": false}),
    );
    let state = p.call("command_state", json!({}));
    assert_eq!(state["pursuits"], json!([]));
    assert_eq!(
        state["pursuit_refusals"],
        json!([{"task_force": 1, "contact": 1}])
    );
}

#[test]
fn switching_the_standing_interrupt_off_gives_no_stop() {
    let mut p = Player::new(false);
    p.queued(
        "set_interrupt",
        json!({"standing": "asks_leave_to_pursue", "on": false}),
    );
    let a = p.call("run", json!({"days": 1}));
    assert_eq!(a["stopped"], "days", "{a}");
    assert_eq!(counts(&a), (1, 1, 1));
    assert_eq!(a["fired"], json!([]));
    // The asking is still in the notices, unanswered: no leave.
    assert_eq!(
        a["staff_orders"]["notices"][0]["kind"],
        "asks_leave_to_pursue"
    );
    let state = p.call("command_state", json!({}));
    assert_eq!(state["pursuits"], json!([]));
    assert_eq!(
        state["interrupts"]["switched_off"],
        json!(["asks_leave_to_pursue"])
    );
}

#[test]
fn end_day_does_not_stop_for_an_asking() {
    let mut p = Player::new(false);
    let a = p.call("end_day", json!({}));
    assert_eq!(a["resolved"], true);
    assert_eq!(a["day"], 1);
}

#[test]
fn advance_from_dusk_stops_before_the_day_likewise() {
    let mut p = Player::new(true);
    let hash = p.session.state_hash();
    let a = p.call("advance", json!({"pulses": 2}));
    assert_eq!(a["stopped"], "asks_leave", "{a}");
    assert_eq!(a["pulses_resolved"], json!([]));
    assert_eq!(a["clock"]["at"], "dusk");
    assert_eq!(a["clock"]["day"], 1);
    assert_eq!(a["clock"]["days_resolved"], 0);
    assert_eq!(a["asking"][0]["task_force"], 1);
    assert_eq!(a["fired"], json!([]));
    let summary = a["summary"].as_str().unwrap();
    assert!(
        summary.contains("resolved 0 pulses; stopped at dusk before day 1: task force 1 asks leave to pursue contact 1"),
        "{summary}"
    );
    assert_eq!(p.session.state_hash(), hash);
    // The next call resolves the first pulse: no second stop for that day.
    let b = p.call("advance", json!({"pulses": 1}));
    assert_eq!(b["stopped"], "pulses", "{b}");
    assert_eq!(b["pulses_resolved"], json!([{"day": 1, "pulse": "night"}]));
}

#[test]
fn set_pursuit_is_queued_and_refused_as_set_leave_is() {
    let mut p = Player::new(false);
    // The default is read up the tree: none is set, and the leave to launch
    // is given by the script, so it is go.
    let state = p.call("command_state", json!({}));
    let intent = &state["intents"][0];
    assert_eq!(
        intent["pursue"],
        json!({"leave": "go", "from": "default", "set": false}),
        "{state}"
    );
    assert!(state["leaves"]["pursue_meaning"]
        .as_str()
        .unwrap()
        .contains("ask:"));
    let q = p.queued("set_pursuit", json!({"intent": 1, "leave": "ask"}));
    assert_eq!(q["command"]["kind"], "set_pursuit");
    assert_eq!(q["command"]["intent"], 1);
    assert_eq!(q["command"]["leave"], "ask");
    let state = p.call("command_state", json!({}));
    assert_eq!(
        state["intents"][0]["pursue"],
        json!({"leave": "ask", "from": 1, "set": true}),
        "{state}"
    );
    // Cleared by null, and by leaving it out.
    p.queued("set_pursuit", json!({"intent": 1, "leave": null}));
    let state = p.call("command_state", json!({}));
    assert_eq!(state["intents"][0]["pursue"]["set"], false, "{state}");
    p.queued("set_pursuit", json!({"intent": 1, "leave": "stay"}));
    p.queued("set_pursuit", json!({"intent": 1}));
    let state = p.call("command_state", json!({}));
    assert_eq!(state["intents"][0]["pursue"]["set"], false, "{state}");
    // Refused: an intent that is not in force, and a setting that is none.
    let r = p.call("set_pursuit", json!({"intent": 9, "leave": "go"}));
    assert_eq!(r["status"], "refused", "{r}");
    let e = p
        .try_call("set_pursuit", json!({"intent": 1, "leave": "maybe"}))
        .unwrap_err();
    assert!(e.message.contains("unknown leave to pursue"), "{e}");
    let e = p
        .try_call("set_pursuit", json!({"intent": 1, "extra": 1}))
        .unwrap_err();
    assert!(e.message.contains("is not an argument"), "{e}");
}

#[test]
fn answer_pursuit_is_refused_for_a_force_that_is_not_the_sides() {
    let mut p = Player::new(false);
    let r = p.call(
        "answer_pursuit",
        json!({"task_force": 9, "contact": 1, "allow": true}),
    );
    assert_eq!(r["status"], "refused", "{r}");
    assert_eq!(r["command"]["task_force"], 9, "the number as written");
    assert!(r["reason"].as_str().unwrap().contains('9'), "{r}");
    let state = p.call("command_state", json!({}));
    assert_eq!(state["queued"].as_array().unwrap().len(), 0);
    let e = p
        .try_call("answer_pursuit", json!({"task_force": 1, "contact": 1}))
        .unwrap_err();
    assert!(e.message.contains("allow"), "{e}");
}

#[test]
fn the_report_and_acknowledge_know_the_asking_by_force() {
    let mut p = Player::new(false);
    p.call("run", json!({"days": 1}));
    p.call("run", json!({"days": 1}));
    let report = p.call("report", json!({}));
    let matters = report["matters"].as_array().unwrap();
    assert_eq!(matters.len(), 1, "{report}");
    let m = &matters[0];
    assert_eq!(m["kind"], "asks_leave_to_pursue");
    assert_eq!(m["task_force"], 1);
    assert_eq!(m["contact"], 1);
    assert!(
        m["words"].as_str().unwrap().starts_with(
            "task force 1 asks leave to pursue contact 1, a carrier reported at (16,0), to (10,0)"
        ),
        "{m}"
    );
    let state = p.call("command_state", json!({}));
    let standing = &state["interrupts"]["standing"][0];
    assert_eq!(standing["standing"], "asks_leave_to_pursue");
    assert_eq!(standing["task_force"], 1);
    assert_eq!(standing["needs_acknowledgement"], true);
    let a = p.queued(
        "acknowledge",
        json!({"standing": "asks_leave_to_pursue", "task_force": 1}),
    );
    assert_eq!(a["command"]["task_force"], 1);
    // A force that is not the side's is refused.
    let r = p.call(
        "acknowledge",
        json!({"standing": "asks_leave_to_pursue", "task_force": 9}),
    );
    assert_eq!(r["status"], "refused", "{r}");
}

#[test]
fn in_a_watched_game_a_held_run_stops_before_the_day_at_the_watchers_step() {
    use sim_agent::Called;
    let session = session(false).watched();
    let mut p = Player {
        session,
        schemas: schemas(),
    };
    let called = p
        .session
        .seat_call(Side::Allied, "run", &json!({"days": 2}));
    assert_eq!(called, Called::Held);
    let hash = p.session.state_hash();
    let stepped = p.session.step();
    let step = stepped.answer.expect("a step is answered");
    assert_eq!(step["resolved"]["pulses"], json!([]), "{step}");
    assert_eq!(step["stops"], json!(["allied"]), "{step}");
    assert_eq!(p.session.state_hash(), hash);
    assert_eq!(stepped.answered.len(), 1);
    let (side, answer) = &stepped.answered[0];
    assert_eq!(*side, Side::Allied);
    let answer = answer.as_ref().expect("the held call is answered");
    assert_eq!(answer["stopped"], "asks_leave", "{answer}");
    assert_eq!(counts(answer), (1, 0, 0));
    // The seat calls again: the day is played at the next step.
    assert_eq!(
        p.session
            .seat_call(Side::Allied, "run", &json!({"days": 1})),
        Called::Held
    );
    let stepped = p.session.step();
    assert_eq!(stepped.answered.len(), 1, "{:?}", stepped.answer);
    let answer = stepped.answered[0].1.as_ref().unwrap();
    assert_eq!(answer["stopped"], "days", "{answer}");
    assert_eq!(counts(answer), (1, 1, 1));
}
