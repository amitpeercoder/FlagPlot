//! What the tools tell a player, after two models played scenarios 5 and 7
//! alone (decision 0051, as built; the findings of the review of that play).
//! Nothing here is a rule: each test is about what is said.

mod support;

use serde_json::{json, Value};
use sim_agent::Session;
use sim_core::state::Side;
use support::play::session_of;
use support::{load, schemas};

fn call(session: &mut Session, tool: &str, args: Value) -> Value {
    session
        .call(Side::Allied, tool, &args)
        .unwrap_or_else(|e| panic!("{tool} was refused: {e}"))
}

fn text_of(session: &mut Session) -> String {
    call(session, "get_briefing", json!({"text": true}))["text"]
        .as_str()
        .expect("text")
        .to_string()
}

fn allied(name: &str, seed: u64) -> Session {
    session_of(load(name, seed), &[Side::Allied]).0
}

fn end_days(session: &mut Session, n: u32) {
    for _ in 0..n {
        call(session, "end_day", json!({}));
    }
}

/// Item 2: the heading says how long the game is, and says so when it is over.
#[test]
fn the_heading_says_the_last_day_of_the_game() {
    let mut s = allied("05_eastern_solomons.toml", 1942);
    let text = text_of(&mut s);
    assert!(text.contains("\n- Day: 1 of 1\n"), "{text}");
    let data = call(&mut s, "get_briefing", json!({"text": false}));
    assert_eq!(data["heading"]["last_day"], json!(1));
    assert_eq!(data["heading"]["day"], json!(1));
    call(&mut s, "end_day", json!({}));
    let text = text_of(&mut s);
    assert!(
        text.contains("\n- Day: 2 (the game is over: day 1 was the last)\n"),
        "{text}"
    );

    let days = load("07_full_slice.toml", 1942).days;
    let mut s = allied("07_full_slice.toml", 1942);
    assert!(text_of(&mut s).contains(&format!("\n- Day: 1 of {days}\n")));
    end_days(&mut s, 2);
    assert!(text_of(&mut s).contains(&format!("\n- Day: 3 of {days}\n")));
}

/// Item 4: a force with a ship whose propulsion is out is said to be unable
/// to move, on its line, in the warnings and in the list, and one with a ship
/// degraded is said to be slowed. The same seeds, scenario 5, as the games
/// the models played found it.
#[test]
fn a_force_that_cannot_move_or_is_slowed_is_said_to_be() {
    let schemas = schemas();
    for (seed, effect, word, line_end) in [
        (10, "cannot_move", "out", "CANNOT MOVE (propulsion out)"),
        (32, "slowed", "degraded", "slowed (propulsion damaged)"),
    ] {
        let mut s = allied("05_eastern_solomons.toml", seed);
        end_days(&mut s, 1);
        let data = call(&mut s, "get_briefing", json!({"text": false}));
        assert!(
            schemas.check("briefing.schema.json", &data).is_empty(),
            "{:?}",
            schemas.check("briefing.schema.json", &data)
        );
        let warnings = data["warnings"]["task_force_propulsion"]
            .as_array()
            .expect("the warning is listed");
        assert_eq!(warnings.len(), 1, "seed {seed}");
        assert_eq!(warnings[0]["task_force"]["handle"], "task_force:1");
        assert_eq!(warnings[0]["effect"], effect, "seed {seed}");
        assert_eq!(warnings[0]["ships"][0]["ship"]["handle"], "ship:1");
        assert_eq!(warnings[0]["ships"][0]["propulsion"], word);

        // The same facts in the list, on the force's line.
        let list = call(&mut s, "list_forces", json!({}));
        assert!(schemas.check("list_forces.schema.json", &list).is_empty());
        let force = list["task_forces"]
            .as_array()
            .unwrap()
            .iter()
            .find(|f| f["task_force"]["handle"] == "task_force:1")
            .expect("the force");
        assert_eq!(force["movement"], effect);
        assert_eq!(force["status"], "idle", "the status alone says nothing");
        assert_eq!(force["propulsion_damaged"][0]["ship"]["handle"], "ship:1");

        // And in the text.
        let text = text_of(&mut s);
        assert!(text.contains(line_end), "{text}");
        assert!(
            text.contains(&format!("ship:1 \"USS Enterprise\" propulsion {word}")),
            "{text}"
        );
        // And the entity agrees, from the ship's own system states.
        let ship = call(&mut s, "get_entity", json!({"handle": "ship:1"}));
        assert_eq!(ship["entity"]["systems"]["propulsion"], word);
    }
}

/// A force with every propulsion working has no warning and a plain line.
#[test]
fn a_force_that_can_move_has_no_propulsion_warning() {
    let mut s = allied("05_eastern_solomons.toml", 1942);
    let data = call(&mut s, "get_briefing", json!({"text": false}));
    assert_eq!(data["warnings"]["task_force_propulsion"], json!([]));
    let list = call(&mut s, "list_forces", json!({}));
    for force in list["task_forces"].as_array().unwrap() {
        assert_eq!(force["movement"], "normal");
        assert_eq!(force["propulsion_damaged"], json!([]));
    }
    let text = text_of(&mut s);
    assert!(
        text.contains("- Task forces that cannot move or are slowed: none"),
        "{text}"
    );
    assert!(!text.contains("CANNOT MOVE"));
}

/// Item 7: an arrival due on day N is still listed in the briefing for day N,
/// and says when it arrives; the group has its aircraft in the briefing for
/// day N + 1.
///
/// The arrival is VMF-224, air group 13, due day 10 (19 aircraft, 19 pilots).
/// It was Flight 300, air group 12, due day 4, until decision 11 of 0059 took
/// that arrival out of the full slice: the rule produces those aircraft now,
/// and on this seed no Allied deck is lost, so none arrive.
#[test]
fn an_arrival_due_on_a_day_is_listed_that_day_and_has_arrived_the_day_after() {
    let mut s = allied("07_full_slice.toml", 1942);
    // The briefing lists arrivals due on the coming day and the next two:
    // day 8 is the first that lists day 10.
    end_days(&mut s, 7);
    let text = text_of(&mut s);
    assert!(
        text.contains("- Due day 10: air_group:13 \"VMF-224\", 19 aircraft, 19 pilots, arriving in the night pulse that begins that day, before its morning"),
        "{text}"
    );
    let ready = |text: &str| {
        text.lines()
            .find(|l| l.starts_with("- air_group:13 "))
            .expect("the group")
            .contains(": 19 ready")
    };
    end_days(&mut s, 2);
    let day10 = text_of(&mut s);
    assert!(day10.contains("\n- Day: 10 of"), "{day10}");
    assert!(day10.contains("- Due day 10: air_group:13"), "{day10}");
    assert!(!ready(&day10), "not yet arrived in the briefing for day 10");
    end_days(&mut s, 1);
    let day11 = text_of(&mut s);
    assert!(day11.contains("\n- Day: 11 of"), "{day11}");
    assert!(ready(&day11), "arrived during day 10: {day11}");
    assert!(!day11.contains("Due day 10"), "{day11}");
}

/// Item 8: a group whose pilots are not counted says so in words.
#[test]
fn a_group_whose_pilots_are_not_counted_says_so() {
    let mut s = allied("07_full_slice.toml", 1942);
    let text = text_of(&mut s);
    let carrier = text
        .lines()
        .find(|l| l.starts_with("- air_group:101 "))
        .expect("a carrier group");
    assert!(
        carrier.contains("pilots not counted (no pilot limit)"),
        "{carrier}"
    );
    let base = text
        .lines()
        .find(|l| l.starts_with("- air_group:10 "))
        .expect("a base group");
    assert!(base.contains(", 19 pilots,"), "{base}");
    // The data keeps its null.
    let data = call(&mut s, "get_briefing", json!({"text": false}));
    let group = data["air_groups"]
        .as_array()
        .unwrap()
        .iter()
        .find(|g| g["air_group"]["handle"] == "air_group:101")
        .unwrap();
    assert_eq!(group["pilots"], Value::Null);
}

/// Item 9: an answer with a refused order among queued ones counts both.
#[test]
fn issue_orders_counts_what_was_queued_and_what_was_refused() {
    let mut s = allied("05_eastern_solomons.toml", 1942);
    let schemas = schemas();
    let answer = call(
        &mut s,
        "issue_orders",
        json!({"orders": [
            {"unit": "air_group:1", "order": {"kind": "set_air_mission", "mission": "search"}},
            {"unit": "air_group:999", "order": {"kind": "set_air_mission", "mission": "search"}},
            {"unit": "air_group:1", "order": {"kind": "set_search_arc", "from_deg": 5, "to_deg": 90, "range_nm": "100"}},
        ]}),
    );
    assert!(
        schemas
            .check("issue_orders.schema.json", &answer)
            .is_empty(),
        "{:?}",
        schemas.check("issue_orders.schema.json", &answer)
    );
    assert_eq!(answer["queued_in_call"], 1);
    assert_eq!(answer["refused_in_call"], 2);
    assert_eq!(answer["queued"], 1);
    let summary = answer["summary"].as_str().unwrap();
    assert!(
        summary.starts_with("In this call 1 of 3 orders queued and 2 refused."),
        "{summary}"
    );
    assert!(
        summary.contains("the others in the call stand"),
        "{summary}"
    );
    // A second call: its counts are its own, the queue is the total.
    let answer = call(
        &mut s,
        "issue_orders",
        json!({"orders": [
            {"unit": "air_group:1", "order": {"kind": "set_search_arc", "from_deg": 0, "to_deg": 90, "range_nm": "100"}},
        ]}),
    );
    assert_eq!(answer["queued_in_call"], 1);
    assert_eq!(answer["refused_in_call"], 0);
    assert_eq!(answer["queued"], 2);
}

/// Item 5: an enemy air group in a raid or an attack is called an enemy in
/// the text, and asking for it is answered as for every unit not known.
#[test]
fn an_enemy_in_a_raid_is_called_one_and_is_not_known() {
    let mut s = allied("06a_base_raid.toml", 1942);
    end_days(&mut s, 1);
    let text = text_of(&mut s);
    assert!(
        text.lines()
            .any(|l| l.starts_with("- Raid on base:2 ") && l.contains(" by enemy air_group:1: ")),
        "{text}"
    );
    let known = call(&mut s, "get_entity", json!({"handle": "air_group:1"}));
    let nothing = call(&mut s, "get_entity", json!({"handle": "air_group:9999"}));
    assert_eq!(known, nothing, "not distinguishable from one that is not");
    assert_eq!(known["kind"], "not_known");
}
