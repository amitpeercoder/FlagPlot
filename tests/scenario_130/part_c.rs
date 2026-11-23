//! Scenario 130, part C: the tests of its statements (see the doc comment
//! of the file above).

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde_json::{json, Value};
use sim_agent::{CallError, Seat, Session};
use sim_core::state::Side;
use sim_data::Scenario;
use sim_staff::staffs;

use super::common;

#[allow(dead_code)]
#[path = "../../crates/sim-agent/tests/support/validator.rs"]
mod validator;

const ALLIED: Side = Side::Allied;

fn scenario_path(name: &str) -> PathBuf {
    common::data_dir().join("scenarios").join(name)
}

/// A session over the scenario file `name`, built as `sim-cli` builds one,
/// with the Allied seat played, the Japanese seat the script's, and the
/// scenario's own commands taken out: the commander gives his own.
fn session_of(name: &str) -> Session {
    let path = scenario_path(name);
    let mut scenario: Scenario = common::load(&path).expect("the scenario loads");
    scenario.replace_commands(BTreeMap::new());
    let inside = scenario.orders_inside_a_day();
    let script_inside = scenario.scripted_orders_inside_day().clone();
    let scenario_name = scenario.name.clone();
    Session::new(
        scenario.world,
        &scenario_name,
        scenario.days,
        Seat::Player,
        Seat::Script(BTreeMap::new()),
    )
    .with_staffs(staffs())
    .with_orders_inside_a_day(inside, script_inside)
}

fn call(session: &mut Session, tool: &str, args: Value) -> Result<Value, CallError> {
    session.call(ALLIED, tool, &args)
}

fn ask(session: &mut Session, tool: &str, args: Value) -> Value {
    call(session, tool, args.clone()).unwrap_or_else(|e| panic!("{tool} {args}: {}", e.message))
}

/// The answer checked against its schema.
fn checked(answer: &Value) {
    let schemas = validator::Schemas::load(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../crates/sim-agent/schemas"),
    );
    let kind = answer["kind"].as_str().expect("a kind");
    let errors = schemas.check(&format!("{kind}.schema.json"), answer);
    assert!(errors.is_empty(), "{errors:?}\n{answer}");
}

/// Both leaves given, as the scenarios' own commanders give them.
fn give_leaves(s: &mut Session) {
    for leave in ["prepare", "launch"] {
        let a = ask(s, "set_leave", json!({"leave": leave, "given": true}));
        assert_eq!(a["status"], "queued", "{a}");
    }
}

/// The orders of a preview's "with change" reading, as the answer gives them.
fn would_hand_in(preview: &Value, part: &str) -> Value {
    preview["with_change"]["would_hand_in"][part].clone()
}

/// The deny at (0,5) of scenario 53, as `preview` takes it.
fn deny(limits: Value) -> Value {
    json!({"command": "set_intent", "intent_kind": "deny", "hex": [0, 5], "distance": 0,
           "limits": limits})
}

/// The three limits of statement 13: a forces limit naming one of the
/// three bomber groups, a risk, and an area.
fn three_limits(risk: &str) -> Value {
    json!([
        {"limit": "forces", "air_groups": [1]},
        {"limit": "risk", "risk": risk},
        {"limit": "area", "hexes": [[0, 4], [0, 5]]},
    ])
}

/// Statement 13.
///
/// The game: scenario 53's, without its commanders' commands. Allied base 1
/// has three dive bomber groups (1 to 3), a flying boat group and a fighter
/// group, and holds three contacts in reach; the Allied commander gives both
/// leaves, then asks the staff's reading of "deny the waters at (0,5)" with
/// a forces limit naming bomber group 1, the risk "preserve" and an area,
/// all given with the order. He approves that preview, the day is resolved,
/// and what his staff handed in on the first morning is read from the
/// game's own record of it.
///
/// The preview's orders and commands are the staff's reading of the order as
/// signed, so they are those handed in; and they are not those of the order
/// alone (a preview of the deny with no limit differs: groups 2 and 3 are
/// used there and not here, and the strike share is the preserve risk's).
#[test]
fn statement_13_a_preview_with_its_limits_is_what_the_staff_hands_in_once_they_are_signed() {
    let mut s = session_of("53_deny_an_area.toml");
    give_leaves(&mut s);
    let alone = ask(&mut s, "preview", deny(json!([])));
    assert_eq!(alone["preview"], 1);
    let with = ask(&mut s, "preview", deny(three_limits("preserve")));
    assert_eq!(with["preview"], 2, "{with}");
    checked(&alone);
    checked(&with);
    assert_eq!(with["limits"].as_array().expect("limits").len(), 3);
    assert_eq!(with["shown_limits"].as_array().expect("words").len(), 3);
    assert_eq!(alone["limits"], json!([]));
    assert_eq!(alone["shown_limits"], json!([]));

    // The limits change the reading.
    assert_ne!(
        would_hand_in(&with, "orders"),
        would_hand_in(&alone, "orders"),
        "the staff's reading is of the order with its limits"
    );
    let orders = would_hand_in(&with, "orders");
    assert!(
        orders.as_array().expect("orders").len() > 0,
        "the staff hands in orders for the order: {with}"
    );
    // Only group 1 is used, and it is set at the preserve risk's share.
    let units: std::collections::BTreeSet<&str> = orders
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|o| o["unit"].as_str())
        .collect();
    assert!(
        !units.contains("air_group:2") && !units.contains("air_group:3"),
        "{units:?}"
    );
    assert!(
        orders.as_array().unwrap().iter().any(|o| {
            o["order"]["kind"] == "set_strike_held_share" && o["order"]["share"] == "0.333333"
        }),
        "{orders}"
    );

    // Sign it, resolve the day, and read what the staff handed in.
    let approved = ask(&mut s, "approve", json!({"preview": 2}));
    assert_eq!(approved["status"], "queued", "{approved}");
    let end = ask(&mut s, "end_day", json!({}));
    assert_eq!(end["resolved"], true, "{end}");
    let handed = &s.staff_handed_in()[&1][&ALLIED];
    let written = sim_agent::staff_decisions_to_json(handed);
    assert_eq!(written["orders"], would_hand_in(&with, "orders"));
    // The reading is of the order once signed: the commands it shows are the
    // last of those handed in (the day's first rounds approve the plan, and
    // the reading is of the plan released).
    let read = would_hand_in(&with, "commands");
    let read = read.as_array().expect("commands");
    let handed = written["commands"].as_array().expect("commands");
    assert!(
        !read.is_empty() && handed.ends_with(read),
        "{written}\n{with}"
    );
}

/// Statement 14.
///
/// The game: scenario 53's, without its commanders' commands, with both
/// leaves queued first (two commands). The commander previews the deny with
/// three limits and approves the preview: the order and its three limits are
/// queued together, in the order given; `queued` counts all six commands
/// and `limits_queued` says three came with the order; the queue lists them.
/// Taking back the order (the first of the commands the preview queued)
/// takes its three limits back with it, and leaves the two leaves.
#[test]
fn statement_14_signing_the_preview_queues_the_order_and_its_limits_and_taking_the_order_back_takes_them(
) {
    let mut s = session_of("53_deny_an_area.toml");
    give_leaves(&mut s);
    let p = ask(&mut s, "preview", deny(three_limits("press")));
    let a = ask(&mut s, "approve", json!({"preview": p["preview"]}));
    assert_eq!(a["status"], "queued", "{a}");
    checked(&p);
    checked(&a);
    assert_eq!(a["limits_queued"], 3);
    // The two leaves, the order and its three limits.
    assert_eq!(a["queued"], 6);
    assert_eq!(a["preview"], p["preview"]);

    let q = ask(&mut s, "queued", json!({}));
    let tools: Vec<&str> = q["entries"]
        .as_array()
        .expect("entries")
        .iter()
        .map(|e| e["tool"].as_str().expect("a tool"))
        .collect();
    assert_eq!(
        tools,
        [
            "set_leave",
            "set_leave",
            "set_intent",
            "set_limit",
            "set_limit",
            "set_limit"
        ]
    );
    let order = &q["entries"][2];
    assert_eq!(order["place"], json!({"queue": "commands", "number": 3}));
    assert_eq!(
        order["goes_with"],
        json!([
            {"queue": "commands", "number": 4},
            {"queue": "commands", "number": 5},
            {"queue": "commands", "number": 6}
        ])
    );
    let limits: Vec<&str> = q["entries"]
        .as_array()
        .unwrap()
        .iter()
        .skip(3)
        .map(|e| e["command"]["limit"].as_str().expect("a limit"))
        .collect();
    assert_eq!(limits, ["forces", "risk", "area"]);

    let back = ask(
        &mut s,
        "take_back",
        json!({"queue": "commands", "number": 3, "as_of": q["as_of"]}),
    );
    assert_eq!(back["kind"], "take_back", "{back}");
    let after = ask(&mut s, "queued", json!({}));
    let left: Vec<&str> = after["entries"]
        .as_array()
        .expect("entries")
        .iter()
        .map(|e| e["tool"].as_str().expect("a tool"))
        .collect();
    assert_eq!(left, ["set_leave", "set_leave"], "{after}");
}

/// Statement 15.
///
/// The game: scenario 53's, without its commanders' commands. The commander
/// previews the deny with limits the engine would refuse: first a forces
/// limit naming task force 9, which the side has not, as the second of two
/// limits; then a forces limit naming bomber group 1 and listing
/// `air_groups` in `none_of`. Each answer is the refusal of that limit, says
/// which of the batch it is, queues nothing, and spends no preview number:
/// the preview that follows, of the order alone, is preview 1.
#[test]
fn statement_15_a_limit_the_engine_would_refuse_gives_the_refusal_and_queues_nothing() {
    let mut s = session_of("53_deny_an_area.toml");
    let before = ask(&mut s, "queued", json!({}));

    let first = ask(
        &mut s,
        "preview",
        deny(json!([
            {"limit": "risk", "risk": "press"},
            {"limit": "forces", "task_forces": [9]},
        ])),
    );
    assert_eq!(first["kind"], "command", "{first}");
    assert_eq!(first["status"], "refused", "{first}");
    checked(&first);
    assert_eq!(first["tool"], "preview");
    assert_eq!(first["of_batch"], json!({"position": 3, "size": 3}));
    assert_eq!(first["command"]["limit"], "forces");

    let second = ask(
        &mut s,
        "preview",
        deny(json!([{"limit": "forces", "air_groups": [1], "none_of": ["air_groups"]}])),
    );
    assert_eq!(second["status"], "refused", "{second}");
    assert_eq!(second["of_batch"], json!({"position": 2, "size": 2}));

    // Nothing was queued and no number was spent.
    let after = ask(&mut s, "queued", json!({}));
    assert_eq!(after["entries"], before["entries"]);
    assert_eq!(after["as_of"], before["as_of"]);
    let next = ask(&mut s, "preview", deny(json!([])));
    assert_eq!(next["preview"], 1, "{next}");

    // "limits" is not an argument of a preview of a limit.
    let err = call(
        &mut s,
        "preview",
        json!({"command": "set_limit", "limit": "risk", "intent": 1, "risk": "press",
               "limits": [{"limit": "risk", "risk": "press"}]}),
    );
    assert!(err.is_err());
}

/// The patrol shares the staff's reading of a preview sets, one for each
/// fighter group, as the decimal strings the orders carry.
fn patrol_shares(preview: &Value) -> Vec<String> {
    would_hand_in(preview, "orders")
        .as_array()
        .expect("orders")
        .iter()
        .filter(|o| o["order"]["kind"] == "set_patrol_share")
        .map(|o| o["order"]["share"].as_str().expect("a share").to_string())
        .collect()
}

/// Statement 16.
///
/// The game: scenario 47's, without its commanders' commands. Allied base 1
/// has two fighter groups, a flying boat group and a dive bomber group, all
/// resting. The commander gives both leaves and asks the staff's reading of
/// "cover base 1 from the air" with a forces limit naming the four groups,
/// once with the risk "press" and once with "preserve". The fighters' patrol
/// share is a quarter in the first reading and three quarters in the
/// second (scenario 47's figures), each for both fighter groups.
#[test]
fn statement_16_a_preview_at_press_and_at_preserve_show_different_patrol_shares() {
    let mut s = session_of("47_cover_from_the_air.toml");
    give_leaves(&mut s);
    let ask_at = |s: &mut Session, risk: &str| {
        ask(
            s,
            "preview",
            json!({"command": "set_intent", "intent_kind": "cover", "base": 1,
            "limits": [
                {"limit": "forces", "air_groups": [1, 2, 3, 4]},
                {"limit": "risk", "risk": risk},
            ]}),
        )
    };
    let press = ask_at(&mut s, "press");
    let preserve = ask_at(&mut s, "preserve");
    assert_eq!(patrol_shares(&press), ["0.25", "0.25"], "{press}");
    assert_eq!(patrol_shares(&preserve), ["0.75", "0.75"], "{preserve}");
    assert_eq!(press["shown_limits"][1], "risk of intent:1 is press");
    assert_eq!(preserve["shown_limits"][1], "risk of intent:1 is preserve");
}
