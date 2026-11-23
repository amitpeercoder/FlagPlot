//! A seat at a task force or a base, and the easier setting at the area seat
//! (decision 0138, item 5).
//!
//! A seat at a command is given what that command holds and nothing else of
//! its side's. The proof is by two worlds that differ only in what the seat's
//! command does not hold (another force's start hex, ships and aircraft, a
//! base's stocks, a starting contact only another command saw): every answer
//! of every tool at the seat is byte for byte the same in the two worlds, at
//! the start and after each day. Over scenario 62's text (command levels on,
//! reports up on time, sideways and down never) changed in the test.

mod support;

use std::collections::BTreeMap;

use serde_json::{json, Value};
use sim_agent::{Post, Seat, Session, COMMAND_TOOLS, TOOLS};
use sim_core::ids::{BaseId, TaskForceId};
use sim_core::pictures::CommandRef;
use sim_core::state::Side;
use support::{load_text, scenario_text};

const SCENARIO: &str = "62_reports_between_commands.toml";
const STALE: &str = "63_a_superiors_knowledge.toml";

fn force(number: u32) -> Post {
    Post::At(CommandRef::TaskForce(TaskForceId::new(number)))
}

fn base(number: u32) -> Post {
    Post::At(CommandRef::Base(BaseId::new(number)))
}

fn session_named(text: &str, name: &str, post: Post) -> Result<Session, String> {
    let scenario = load_text(text, name, 1942);
    let name = scenario.name.clone();
    // The staff is in every seat, and the scenario's own commands are given as
    // the commander's, as `sim-cli` opens a game.
    let commands = scenario.scripted_commands().clone();
    Session::new(
        scenario.world,
        &name,
        scenario.days,
        Seat::Player,
        Seat::Script(BTreeMap::new()),
    )
    .with_staffs(sim_staff::staffs())
    .with_commands(commands)
    .with_post(Side::Allied, post)
}

fn session(text: &str, post: Post) -> Session {
    session_named(text, SCENARIO, post).expect("the seat is placed")
}

fn call(session: &mut Session, tool: &str, args: Value) -> Value {
    session
        .call(Side::Allied, tool, &args)
        .unwrap_or_else(|e| panic!("{tool} {args}: {e}"))
}

// ---------------------------------------------------------------------
// The two worlds.
// ---------------------------------------------------------------------

/// Another carrier force and its stocks, as the text says them.
fn change(text: &str, from: &str, to: &str) -> String {
    assert!(text.contains(from), "the text has no {from:?}");
    text.replacen(from, to, 1)
}

/// A contact that only task force 2 saw.
const TF2_CONTACT: &str = "[[contact]]\nside = \"allied\"\nq = 12\nr = 6\nship_count = 2\nkind = \"cruiser\"\nage_pulses = 0\nseen_by = { task_force = 2 }\n\n# The Allied commander's commands";

/// A contact that only task force 1 saw.
const TF1_CONTACT: &str = "[[contact]]\nside = \"allied\"\nq = 3\nr = 9\nship_count = 2\nkind = \"cruiser\"\nage_pulses = 0\nseen_by = { task_force = 1 }\n\n# The Allied commander's commands";

/// Worlds that differ from scenario 62 in what task force 1 and base 1 do not
/// hold (they are not task force 2's, and a base's stocks are not a force's).
fn unseen_by_force_one(text: &str) -> Vec<(&'static str, String)> {
    vec![
        (
            "task force 2 starts elsewhere",
            change(text, "q = 7\nr = 0\nmission", "q = 9\nr = 0\nmission"),
        ),
        (
            "task force 2's ship has less fuel",
            change(
                text,
                "name = \"Placeholder carrier 2\"\nclass = 4\nside = \"allied\"\nfuel_t = 3000",
                "name = \"Placeholder carrier 2\"\nclass = 4\nside = \"allied\"\nfuel_t = 2000",
            ),
        ),
        (
            "task force 2's fighters are fewer",
            change(
                text,
                "name = \"Placeholder fighters 4\"\nside = \"allied\"\nship = 2\naircraft_type = 4\nready = 12",
                "name = \"Placeholder fighters 4\"\nside = \"allied\"\nship = 2\naircraft_type = 4\nready = 8",
            ),
        ),
        (
            "base 1 has other stocks",
            change(
                text,
                "supply_t = 1000\nfuel_t = 5000",
                "supply_t = 400\nfuel_t = 2000",
            ),
        ),
        (
            "a contact only task force 2 saw",
            change(text, "# The Allied commander's commands", TF2_CONTACT),
        ),
    ]
}

/// Worlds that differ in what base 1 does not hold: both forces, their ships
/// and a contact only task force 1 saw.
fn unseen_by_base_one(text: &str) -> Vec<(&'static str, String)> {
    vec![
        (
            "task force 1 starts elsewhere",
            change(text, "q = 0\nr = 0\nmission", "q = 1\nr = 0\nmission"),
        ),
        (
            "task force 2's ship has less fuel",
            change(
                text,
                "name = \"Placeholder carrier 2\"\nclass = 4\nside = \"allied\"\nfuel_t = 3000",
                "name = \"Placeholder carrier 2\"\nclass = 4\nside = \"allied\"\nfuel_t = 2000",
            ),
        ),
        (
            "a contact only task force 1 saw",
            change(text, "# The Allied commander's commands", TF1_CONTACT),
        ),
    ]
}

/// Every answer of every tool the seat has, the call's whole outcome as text,
/// under a label. Asked of units the seat holds, of units it does not, of
/// numbers that are nothing, and of every event it may be shown.
fn everything(session: &mut Session) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    let mut ask = |session: &mut Session, label: String, tool: &str, args: Value| {
        let outcome = match session.call(Side::Allied, tool, &args) {
            Ok(answer) => answer.to_string(),
            Err(error) => error.to_json().to_string(),
        };
        out.insert(label, outcome);
    };
    ask(session, "briefing".into(), "get_briefing", json!({}));
    ask(
        session,
        "briefing text".into(),
        "get_briefing",
        json!({"text": true}),
    );
    // What the seat holds of the enemy, on one answer (decision 0226).
    ask(session, "intelligence".into(), "intelligence", json!({}));
    for tool in ["list_forces", "list_bases", "list_contacts"] {
        ask(session, tool.to_string(), tool, json!({}));
        ask(
            session,
            format!("{tool} in an area"),
            tool,
            json!({"area": {"q": 0, "r": 0, "radius_nm": 200}}),
        );
    }
    let mut handles: Vec<String> = Vec::new();
    for kind in [
        "task_force",
        "ship",
        "air_group",
        "land_unit",
        "contact",
        "base",
    ] {
        for number in 0..=8 {
            handles.push(format!("{kind}:{number}"));
        }
    }
    for handle in &handles {
        ask(
            session,
            format!("entity {handle}"),
            "get_entity",
            json!({"handle": handle}),
        );
    }
    let questions = [
        json!({"question": "air_distance", "from": "task_force:1", "to": "base:1"}),
        json!({"question": "air_distance", "from": "task_force:2", "to": "base:1"}),
        json!({"question": "air_distance", "from": {"q": 0, "r": 0}, "to": "contact:1"}),
        json!({"question": "air_distance", "from": "air_group:1", "to": "contact:2"}),
        json!({"question": "sea_route", "from": "task_force:1", "to": {"q": 3, "r": 0}}),
        json!({"question": "sea_route", "from": "task_force:2", "to": {"q": 3, "r": 0}}),
        json!({"question": "passage", "task_force": "task_force:1", "to": {"q": 4, "r": 0}}),
        json!({"question": "passage", "task_force": "task_force:2", "to": {"q": 4, "r": 0}}),
        json!({"question": "yard", "task_force": "task_force:1"}),
        json!({"question": "yard", "task_force": "task_force:2"}),
        json!({"question": "air_reach", "air_group": "air_group:1", "to": [{"q": 5, "r": 0}, "base:1"]}),
        json!({"question": "air_reach", "air_group": "air_group:4", "to": [{"q": 5, "r": 0}]}),
        json!({"question": "choices", "unit": "task_force:1"}),
        json!({"question": "choices", "unit": "task_force:2"}),
        json!({"question": "choices", "unit": "air_group:1"}),
        json!({"question": "choices", "unit": "air_group:2"}),
        json!({"question": "choices", "unit": "air_group:3"}),
        json!({"question": "choices", "unit": "air_group:4"}),
        json!({"question": "choices", "unit": "air_group:6"}),
        json!({"question": "choices", "unit": "land_unit:1"}),
    ];
    for (i, question) in questions.into_iter().enumerate() {
        ask(session, format!("measure {i}"), "measure", question);
    }
    // Every event the seat may be shown, and some it may not.
    for event in 0..=40 {
        ask(
            session,
            format!("explain {event}"),
            "explain",
            json!({"event": event}),
        );
    }
    // Orders to every unit, the seat's and not, judged and not queued: the
    // call is cleared at the end of this ask.
    let orders: Vec<Value> = ["task_force", "air_group", "ship", "land_unit", "base"]
        .into_iter()
        .flat_map(|kind| (1..=6).map(move |n| (kind, n)))
        .map(|(kind, n)| {
            let order = match kind {
                "task_force" => json!({"kind": "set_speed", "speed": "full"}),
                "air_group" => json!({"kind": "set_air_mission", "mission": "rest"}),
                "land_unit" => json!({"kind": "set_posture", "posture": "defend"}),
                "base" => json!({"kind": "form_task_force", "ships": [1]}),
                _ => json!({"kind": "set_speed", "speed": "full"}),
            };
            json!({"unit": format!("{kind}:{n}"), "order": order})
        })
        .collect();
    ask(
        session,
        "orders".into(),
        "issue_orders",
        json!({"orders": orders, "clear": true}),
    );
    ask(
        session,
        "orders cleared".into(),
        "issue_orders",
        json!({"orders": [], "clear": true}),
    );
    out
}

/// Plays `days` days with the seat ending each, and gives everything the
/// seat is shown at the start and after each day.
fn play_and_ask(text: &str, post: Post, days: u32) -> Vec<BTreeMap<String, String>> {
    play_and_ask_in(text, SCENARIO, post, days)
}

fn play_and_ask_in(text: &str, name: &str, post: Post, days: u32) -> Vec<BTreeMap<String, String>> {
    let mut session = session_named(text, name, post).expect("the seat is placed");
    let mut shown = vec![everything(&mut session)];
    for _ in 0..days {
        let end = call(&mut session, "end_day", json!({}));
        assert_eq!(end["resolved"], true, "{end}");
        shown.push(everything(&mut session));
    }
    // The answers are answers, not one refusal repeated.
    for (day, answers) in shown.iter().enumerate() {
        let briefing = &answers["briefing"];
        assert!(
            briefing.contains("\"kind\":\"briefing\"") && briefing.contains("\"task_forces\""),
            "day {day}: {briefing}"
        );
        assert!(answers["list_forces"].contains("\"kind\":\"list_forces\""));
    }
    shown
}

fn assert_the_same(post: Post, worlds: Vec<(&'static str, String)>, base_text: &str) {
    assert_the_same_in(SCENARIO, 2, post, worlds, base_text);
}

fn assert_the_same_in(
    name: &str,
    days: u32,
    post: Post,
    worlds: Vec<(&'static str, String)>,
    base_text: &str,
) {
    let plain = play_and_ask_in(base_text, name, post, days);
    for (what, text) in worlds {
        let other = play_and_ask_in(&text, name, post, days);
        for (day, (a, b)) in plain.iter().zip(&other).enumerate() {
            assert_eq!(a.len(), b.len(), "{what}");
            for (label, answer) in a {
                assert_eq!(
                    answer, &b[label],
                    "seat {post:?}: {label} differs when {what}, after day {day}"
                );
            }
        }
    }
}

#[test]
fn a_task_force_seat_is_given_nothing_that_another_force_or_a_base_holds() {
    let text = scenario_text(SCENARIO);
    assert_the_same(force(1), unseen_by_force_one(&text), &text);
}

#[test]
fn the_other_task_force_seat_is_the_same_by_symmetry() {
    // Task force 2 is the seat; task force 1 and base 1 are not its own.
    let text = scenario_text(SCENARIO);
    let worlds = vec![
        (
            "task force 1 starts elsewhere",
            change(&text, "q = 0\nr = 0\nmission", "q = 1\nr = 0\nmission"),
        ),
        (
            "task force 1's fighters are fewer",
            change(
                &text,
                "name = \"Placeholder fighters 1\"\nside = \"allied\"\nship = 1\naircraft_type = 4\nready = 12",
                "name = \"Placeholder fighters 1\"\nside = \"allied\"\nship = 1\naircraft_type = 4\nready = 8",
            ),
        ),
        (
            "base 1 has other stocks",
            change(
                &text,
                "supply_t = 1000\nfuel_t = 5000",
                "supply_t = 400\nfuel_t = 2000",
            ),
        ),
        (
            "a contact only task force 1 saw",
            change(&text, "# The Allied commander's commands", TF1_CONTACT),
        ),
    ];
    assert_the_same(force(2), worlds, &text);
}

#[test]
fn a_base_seat_is_given_nothing_that_a_force_holds() {
    let text = scenario_text(SCENARIO);
    assert_the_same(base(1), unseen_by_base_one(&text), &text);
}

// ---------------------------------------------------------------------
// What a seat is given.
// ---------------------------------------------------------------------

#[test]
fn a_task_force_seat_holds_its_own_units_its_own_picture_and_the_public_map() {
    let text = scenario_text(SCENARIO);
    let mut s = session(&text, force(2));
    // Its own force is its first and only: the seat numbers its own units.
    let forces = call(&mut s, "list_forces", json!({}));
    let list = forces["task_forces"].as_array().unwrap();
    assert_eq!(list.len(), 1, "{forces}");
    assert_eq!(list[0]["task_force"]["handle"], "task_force:1");
    assert_eq!(
        list[0]["task_force"]["name"], "Placeholder carrier force 2",
        "{forces}"
    );
    assert_eq!(list[0]["place"]["q"], 7);
    // No base of its own; every base is a public fact, with its holder.
    let bases = call(&mut s, "list_bases", json!({}));
    assert!(bases["bases"].as_array().unwrap().is_empty(), "{bases}");
    let public = bases["enemy_bases"].as_array().unwrap();
    assert_eq!(public.len(), 1, "{bases}");
    assert_eq!(public[0]["base"]["handle"], "base:1");
    assert_eq!(public[0]["held_by"], "allied");
    assert!(public[0].get("supply_t").is_none(), "{bases}");
    // Its picture holds the starting contact.
    let contacts = call(&mut s, "list_contacts", json!({}));
    assert_eq!(contacts["contacts"].as_array().unwrap().len(), 1);
    // Its own groups are its own; the other force's are nobody's here.
    let entity = call(&mut s, "get_entity", json!({"handle": "air_group:3"}));
    assert_eq!(entity["entity"]["entity_kind"], "air_group", "{entity}");
    for handle in ["air_group:4", "ship:2", "task_force:2"] {
        let answer = call(&mut s, "get_entity", json!({"handle": handle}));
        assert_eq!(answer["kind"], "not_known", "{handle}: {answer}");
    }
    // The briefing says what it is not given, and lists none of the side's.
    let briefing = call(&mut s, "get_briefing", json!({}));
    assert_eq!(briefing["objectives"]["not_given_to_this_seat"], true);
    assert_eq!(briefing["task_forces"].as_array().unwrap().len(), 1);
    assert_eq!(briefing["air_groups"].as_array().unwrap().len(), 3);
    let words = call(&mut s, "get_briefing", json!({"text": true}));
    let words = words["text"].as_str().unwrap();
    assert!(
        words
            .contains("none given to this seat: the mission and objectives are the area command's"),
        "{words}"
    );
    assert!(!words.contains("carrier force 1"), "{words}");
}

#[test]
fn the_command_tools_are_refused_and_not_offered_at_a_seat_at_a_command() {
    let text = scenario_text(SCENARIO);
    let mut s = session(&text, force(1));
    assert_eq!(s.tools(Side::Allied), TOOLS.to_vec());
    for tool in COMMAND_TOOLS {
        let answer = s
            .call(Side::Allied, tool, &json!({}))
            .expect_err("a command tool is refused");
        assert_eq!(
            answer.message,
            "stating intents from a seat that is not the area command's is not built (decision 0132)"
        );
    }
    // The area seat keeps its tools.
    let area = session(&text, Post::Area);
    assert!(area.tools(Side::Allied).contains(&"command_state"));
    assert_eq!(area.post(Side::Allied), Post::Area);
}

#[test]
fn an_order_to_a_unit_that_is_not_the_commands_is_refused_with_one_reason() {
    let text = scenario_text(SCENARIO);
    let mut s = session(&text, force(1));
    // The seat's own force is task_force:1; task_force:2 is another force of
    // the side's in the scenario's numbering, and means nothing here, as does
    // an enemy's or a number that is nothing. A ship, a base and a land unit
    // the seat does not hold are refused alike.
    let orders = json!({"orders": [
        {"unit": "task_force:2", "order": {"kind": "set_speed", "speed": "full"}},
        {"unit": "task_force:7", "order": {"kind": "set_speed", "speed": "full"}},
        {"unit": "air_group:4", "order": {"kind": "set_air_mission", "mission": "rest"}},
        {"unit": "air_group:99", "order": {"kind": "set_air_mission", "mission": "rest"}},
        {"unit": "land_unit:1", "order": {"kind": "set_posture", "posture": "defend"}},
        {"unit": "base:1", "order": {"kind": "form_task_force", "ships": [1]}},
        {"unit": "ship:1", "order": {"kind": "set_speed", "speed": "full"}},
        {"unit": "task_force:1", "order": {"kind": "escort", "charge": "task_force:2"}},
        {"unit": "task_force:1", "order": {"kind": "join", "into": "task_force:2"}},
        {"unit": "task_force:1", "order": {"kind": "detach", "ships": [2]}},
    ]});
    let answer = call(&mut s, "issue_orders", orders);
    for (index, result) in answer["results"].as_array().unwrap().iter().enumerate() {
        assert_eq!(result["status"], "refused", "{index}: {answer}");
        assert_eq!(
            result["reason"], "not one of your command's units",
            "{index}: {answer}"
        );
    }
    // Its own units are taken as ever.
    let answer = call(
        &mut s,
        "issue_orders",
        json!({"orders": [
            {"unit": "task_force:1", "order": {"kind": "set_speed", "speed": "full"}},
            {"unit": "air_group:3", "order": {"kind": "set_air_mission", "mission": "naval_strike"}},
        ]}),
    );
    assert_eq!(answer["queued_in_call"], 2, "{answer}");
}

#[test]
fn a_seat_at_a_base_is_given_its_groups_its_stocks_and_its_men() {
    // Base 1 with a fighter group of its own.
    let text = change(
        &scenario_text(SCENARIO),
        "# The contact\n",
        "[[air_group]]\nid = 7\nname = \"Placeholder base fighters 7\"\nside = \"allied\"\nbase = 1\naircraft_type = 4\nready = 8\n\n# The contact\n",
    );
    let mut s = session(&text, base(1));
    let bases = call(&mut s, "list_bases", json!({}));
    let own = bases["bases"].as_array().unwrap();
    assert_eq!(own.len(), 1, "{bases}");
    assert_eq!(own[0]["base"]["handle"], "base:1");
    assert_eq!(own[0]["supply_t"], "1000", "{bases}");
    let forces = call(&mut s, "list_forces", json!({}));
    assert!(forces["task_forces"].as_array().unwrap().is_empty());
    let briefing = call(&mut s, "get_briefing", json!({}));
    assert_eq!(briefing["air_groups"].as_array().unwrap().len(), 1);
    assert_eq!(
        briefing["air_groups"][0]["air_group"]["name"],
        "Placeholder base fighters 7"
    );
    // Its own group is ordered; a force's is not its own.
    let answer = call(
        &mut s,
        "issue_orders",
        json!({"orders": [
            {"unit": "air_group:1", "order": {"kind": "set_air_mission", "mission": "combat_air_patrol"}},
            {"unit": "air_group:2", "order": {"kind": "set_air_mission", "mission": "combat_air_patrol"}},
            {"unit": "task_force:1", "order": {"kind": "set_speed", "speed": "full"}},
        ]}),
    );
    assert_eq!(answer["results"][0]["status"], "queued", "{answer}");
    assert_eq!(
        answer["results"][1]["reason"], "not one of your command's units",
        "{answer}"
    );
    assert_eq!(
        answer["results"][2]["reason"], "not one of your command's units",
        "{answer}"
    );
    // A day passes with the staff in every other seat.
    let end = call(&mut s, "end_day", json!({}));
    assert_eq!(end["resolved"], true);
    let again = call(&mut s, "get_briefing", json!({}));
    assert_eq!(again["air_groups"][0]["mission"], "combat_air_patrol");
}

#[test]
fn a_seat_at_a_command_that_is_not_there_is_refused_and_levels_off_refuse_any() {
    let text = scenario_text(SCENARIO);
    let refused = session_named(&text, SCENARIO, force(9))
        .err()
        .expect("refused");
    assert!(
        refused.contains("is not a task force or a base of the allied side at the start"),
        "{refused}"
    );
    // Japanese task force 1 is not on the Allied side's list.
    let off = text.replacen("command_levels = true\n", "", 1);
    for post in [force(1), base(1)] {
        let refused = session_named(&off, SCENARIO, post).err().expect("refused");
        assert_eq!(
            refused,
            "command levels are off in this scenario: the only seat is the area command's"
        );
    }
    // The area seat, and its easier setting, are open with levels off.
    for post in [Post::Area, Post::AreaEasier] {
        session_named(&off, SCENARIO, post).expect("the area seat is placed");
    }
    // Nor after the game has begun.
    assert_eq!(Post::parse("task_force:2"), Ok(force(2)));
    assert_eq!(Post::parse("base:1"), Ok(base(1)));
    assert_eq!(Post::parse("area:easier"), Ok(Post::AreaEasier));
    assert!(Post::parse("ship:1").is_err());
    assert!(Post::parse("task_force:x").is_err());
    assert_eq!(force(3).text(), "task_force:3");
}

// ---------------------------------------------------------------------
// The easier setting.
// ---------------------------------------------------------------------

fn place_of(list: &Value, number: u64) -> (i64, i64) {
    let line = list["task_forces"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["task_force"]["handle"] == format!("task_force:{number}"))
        .unwrap_or_else(|| panic!("task force {number} is listed: {list}"));
    (
        line["place"]["q"].as_i64().unwrap(),
        line["place"]["r"].as_i64().unwrap(),
    )
}

#[test]
fn the_easier_setting_gives_the_sides_own_forces_exactly_where_the_area_seat_gives_them_as_reported(
) {
    // Scenario 63: task force 1 on radio silence, so the area command holds
    // it as of the start while it steams.
    let text = scenario_text(STALE);
    let steam = json!({"orders": [
        {"unit": "task_force:1", "order": {"kind": "set_route", "waypoints": [[3, 0]]}},
    ]});
    let mut area = session_named(&text, STALE, Post::Area).expect("placed");
    let mut easier = session_named(&text, STALE, Post::AreaEasier).expect("placed");
    for s in [&mut area, &mut easier] {
        let a = call(s, "issue_orders", steam.clone());
        assert_eq!(a["results"][0]["status"], "queued", "{a}");
        assert_eq!(call(s, "end_day", json!({}))["resolved"], true);
    }
    let held = call(&mut area, "list_forces", json!({}));
    let exact = call(&mut easier, "list_forces", json!({}));
    let held_line = &held["task_forces"][0];
    let exact_line = &exact["task_forces"][0];
    assert_eq!(held_line["as_of"], "the start", "{held}");
    assert!(exact_line.get("as_of").is_none(), "{exact}");
    assert_ne!(place_of(&held, 1), place_of(&exact, 1), "{held} {exact}");
    assert_eq!(place_of(&held, 1), (0, 0));
    assert_eq!(place_of(&exact, 1).0, 3);
    // The briefing says the same in words, with no moment.
    let words = call(&mut area, "get_briefing", json!({"text": true}));
    assert!(words["text"]
        .as_str()
        .unwrap()
        .contains("(as last reported, of the start)"));
    let words = call(&mut easier, "get_briefing", json!({"text": true}));
    assert!(!words["text"].as_str().unwrap().contains("as last reported"));
    let entity = call(&mut easier, "get_entity", json!({"handle": "task_force:1"}));
    assert!(
        entity["entity"]["summary"].get("as_of").is_none(),
        "{entity}"
    );
    // The easier setting keeps every tool of the area seat's.
    assert_eq!(easier.tools(Side::Allied), area.tools(Side::Allied));
}

#[test]
fn the_easier_setting_gives_the_sides_own_picture_every_report_at_once() {
    // A contact only task force 2 saw is in that force's picture and the
    // side's own, and in no one else's: the area command does not hold it.
    let text = change(
        &scenario_text(SCENARIO),
        "# The Allied commander's commands",
        TF2_CONTACT,
    );
    let mut area = session(&text, Post::Area);
    let mut easier = session(&text, Post::AreaEasier);
    let held = call(&mut area, "list_contacts", json!({}));
    let own = call(&mut easier, "list_contacts", json!({}));
    assert_eq!(held["contacts"].as_array().unwrap().len(), 1, "{held}");
    assert_eq!(own["contacts"].as_array().unwrap().len(), 2, "{own}");
    // The seat at task force 2 holds it too, and not the contact only
    // another command's picture would give.
    let mut at_two = session(&text, force(2));
    let theirs = call(&mut at_two, "list_contacts", json!({}));
    assert_eq!(theirs["contacts"].as_array().unwrap().len(), 2, "{theirs}");
    let mut at_one = session(&text, force(1));
    let theirs = call(&mut at_one, "list_contacts", json!({}));
    assert_eq!(theirs["contacts"].as_array().unwrap().len(), 1, "{theirs}");
}

#[test]
fn the_area_seat_is_as_it_was_with_no_post_given() {
    let text = scenario_text(SCENARIO);
    let scenario = load_text(&text, SCENARIO, 1942);
    let name = scenario.name.clone();
    let commands = scenario.scripted_commands().clone();
    let mut plain = Session::new(
        scenario.world,
        &name,
        scenario.days,
        Seat::Player,
        Seat::Script(BTreeMap::new()),
    )
    .with_staffs(sim_staff::staffs())
    .with_commands(commands);
    let mut placed = session(&text, Post::Area);
    for day in 0..2 {
        for tool in ["get_briefing", "list_forces", "list_bases", "list_contacts"] {
            let a = call(&mut plain, tool, json!({}));
            let b = call(&mut placed, tool, json!({}));
            assert_eq!(a, b, "{tool} on day {day}");
        }
        for s in [&mut plain, &mut placed] {
            assert_eq!(call(s, "end_day", json!({}))["resolved"], true);
        }
    }
    // The command tools are the area seat's.
    assert!(plain
        .call(Side::Allied, "command_state", &json!({}))
        .is_ok());
    assert!(placed
        .call(Side::Allied, "command_state", &json!({}))
        .is_ok());
}

#[test]
fn the_test_can_tell_worlds_apart_when_the_seat_does_hold_the_difference() {
    // A control: a difference in what the seat holds is seen at once, so the
    // sameness of the other worlds is not an empty one.
    let text = scenario_text(SCENARIO);
    let own = change(&text, "q = 0\nr = 0\nmission", "q = 1\nr = 0\nmission");
    let a = play_and_ask(&text, force(1), 1);
    let b = play_and_ask(&own, force(1), 1);
    for day in 0..2 {
        assert_ne!(a[day]["list_forces"], b[day]["list_forces"], "day {day}");
        assert_ne!(
            a[day]["briefing text"], b[day]["briefing text"],
            "day {day}"
        );
    }
    // And the staff acted on the seat's units in the day played: its events
    // are in what the seat is shown.
    let mut s = session(&text, force(1));
    assert_eq!(call(&mut s, "end_day", json!({}))["resolved"], true);
    let briefing = call(&mut s, "get_briefing", json!({"text": true}));
    let words = briefing["text"].as_str().unwrap();
    assert!(
        words.contains("Naval strike launched: air_group:3"),
        "{words}"
    );
    assert!(words.contains("Accepted yesterday: 12"), "{words}");
}

// ---------------------------------------------------------------------
// A long game, a seat at each command.
// ---------------------------------------------------------------------

const WEEK: &str = "second-proof/07a_week.toml";

/// The week's file with command levels on and the live report figures.
fn week_with_levels() -> String {
    let text = scenario_text(WEEK).replacen(
        "placeholder = true\n",
        "placeholder = true\ncommand_levels = true\nreports = \"../../reference/reports.toml\"\n",
        1,
    );
    assert!(text.contains("command_levels = true"));
    text
}

/// Calls the read tools of a seat on a spread of handles and checks each
/// answer against its schema.
fn ask_everything_valid(s: &mut Session, schemas: &support::validator::Schemas) -> usize {
    let mut calls: Vec<(&str, Value)> = vec![
        ("get_briefing", json!({})),
        ("get_briefing", json!({"text": true})),
        ("list_forces", json!({})),
        ("list_bases", json!({})),
        ("list_contacts", json!({})),
    ];
    for kind in [
        "task_force",
        "ship",
        "air_group",
        "land_unit",
        "contact",
        "base",
    ] {
        for number in 1..=7 {
            calls.push(("get_entity", json!({"handle": format!("{kind}:{number}")})));
        }
    }
    for event in [0, 1, 2, 3, 5, 8, 13, 21, 34, 99] {
        calls.push(("explain", json!({"event": event})));
    }
    calls.push((
        "measure",
        json!({"question": "air_distance", "from": "task_force:1", "to": "base:1"}),
    ));
    for unit in ["task_force:1", "air_group:1", "air_group:2", "land_unit:1"] {
        calls.push(("measure", json!({"question": "choices", "unit": unit})));
    }
    let count = calls.len();
    for (tool, args) in calls {
        let answer = call(s, tool, args.clone());
        let kind = answer["kind"].as_str().expect("an answer has a kind");
        let errors = schemas.check(&format!("{kind}.schema.json"), &answer);
        assert!(errors.is_empty(), "{tool} {args}: {errors:?}\n{answer}");
    }
    count
}

#[test]
fn a_seat_at_each_command_of_a_long_game_is_given_valid_answers_and_may_order_its_own() {
    let text = week_with_levels();
    let schemas = support::schemas();
    let mut seats = 0;
    let posts = (1..=40)
        .map(force)
        .chain((1..=10).map(base))
        .collect::<Vec<_>>();
    for post in posts {
        let Ok(mut s) = session_named(&text, WEEK, post) else {
            continue;
        };
        seats += 1;
        let mut day = 0;
        loop {
            // A spread of days is read in full, the rest only played.
            if day % 7 == 0 {
                ask_everything_valid(&mut s, &schemas);
            }
            // The seat orders what is its own: its force, and its groups.
            let own = call(
                &mut s,
                "issue_orders",
                json!({"orders": [
                    {"unit": "task_force:1", "order": {"kind": "set_readiness", "readiness": "normal"}},
                    {"unit": "air_group:1", "order": {"kind": "set_air_mission", "mission": "rest"}},
                    {"unit": "land_unit:1", "order": {"kind": "set_posture", "posture": "defend"}},
                ]}),
            );
            for result in own["results"].as_array().unwrap() {
                let reason = result.get("reason").and_then(Value::as_str);
                assert_ne!(
                    reason,
                    Some("not known"),
                    "a seat is refused in the side's words: {own}"
                );
            }
            let end = call(&mut s, "end_day", json!({}));
            day += 1;
            if end["game_over"] == true {
                assert!(end["result"].is_null(), "{end}");
                break;
            }
        }
        ask_everything_valid(&mut s, &schemas);
    }
    assert!(seats >= 10, "{seats} seats were tried");
}

#[test]
fn in_the_week_a_carrier_force_and_a_base_are_given_nothing_of_the_rest() {
    // Over a long game with ships, land units and bases of every kind: task
    // force 1 (task force 16) and base 3 (Espiritu Santo) are given the same
    // answers whatever another force's ship holds or a base far away has.
    let text = week_with_levels();
    let worlds = vec![
        (
            "Saratoga has less fuel",
            change(
                &text,
                "id = 11\nname = \"USS Saratoga\"\nclass = 2015\nside = \"allied\"\nfuel_t = 8400",
                "id = 11\nname = \"USS Saratoga\"\nclass = 2015\nside = \"allied\"\nfuel_t = 6000",
            ),
        ),
        (
            "a Tulagi transport has less fuel",
            change(
                &text,
                "id = 73\nname = \"Placeholder troop transport, Tulagi 1\"\nclass = 10\nside = \"allied\"\nfuel_t = 1000",
                "id = 73\nname = \"Placeholder troop transport, Tulagi 1\"\nclass = 10\nside = \"allied\"\nfuel_t = 700",
            ),
        ),
    ];
    assert_the_same_in(WEEK, 8, force(1), worlds.clone(), &text);
    assert_the_same_in(WEEK, 8, base(3), worlds, &text);
    let stocks = vec![(
        "Espiritu Santo has other stocks",
        change(
            &text,
            "supply_t = 960\nfuel_t = 600",
            "supply_t = 700\nfuel_t = 300",
        ),
    )];
    assert_the_same_in(WEEK, 8, force(1), stocks, &text);
}
