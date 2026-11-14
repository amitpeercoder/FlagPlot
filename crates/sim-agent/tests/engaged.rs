//! Who is engaged in a fight ashore, and an attack that fails, in what a
//! side is told (decision 0105, 9).
//!
//! Scenario 33, on the live figures. On day 1 the Japanese regiment at K,
//! the battalion at L (ordered attack with `when_spent` fall_back) and the
//! battalion at N attack and their attacks fail by weight of fire; the
//! Japanese side is told of each failed attack, the place only, and of its
//! spent unit's falling back, whole. The Allied side is told nothing of
//! either: no failed attack, no weight, no falling back, and nothing of the
//! Japanese instruction.

mod support;

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::state::Side;
use sim_data::Scenario;
use support::check::valid;
use support::{scenario_path, scenario_text, schemas};

const NAME: &str = "33_engaged_and_failed.toml";

fn session_of(text: &str) -> Session {
    // On the live figures: the frozen ones switch the rule of decision 0105
    // off (engagement_ratio 1000000, failed_attack_fatigue 0), and on them
    // the attacked units are destroyed before any is spent.
    let scenario = Scenario::from_text(text, &scenario_path(NAME))
        .expect("the scenario loads")
        .with_seed(1942)
        .expect("a seed can be set");
    Session::new(
        scenario.world,
        "Engaged",
        scenario.days,
        Seat::Player,
        Seat::Player,
    )
}

fn call(session: &mut Session, side: Side, tool: &str, args: &Value) -> Value {
    session.call(side, tool, args).expect("the tool answers")
}

/// The orders of scenario 33's day 1, given through the tool: the session
/// does not play the scenario's scripted orders. The Japanese units 1, 2 and 3
/// are those at K, L and N; the Allied unit 4 is the one at P.
fn give_orders(session: &mut Session) {
    let answer = call(
        session,
        Side::Japanese,
        "issue_orders",
        &json!({"orders": [
            {"unit": "land_unit:1", "order": {"kind": "set_posture", "posture": "attack", "when_spent": "hold"}},
            {"unit": "land_unit:2", "order": {"kind": "set_posture", "posture": "attack", "when_spent": "fall_back"}},
            {"unit": "land_unit:3", "order": {"kind": "set_posture", "posture": "attack"}}
        ]}),
    );
    assert_eq!(answer["refused_in_call"], 0, "{answer}");
    let answer = call(
        session,
        Side::Allied,
        "issue_orders",
        &json!({"orders": [
            {"unit": "land_unit:4", "order": {"kind": "set_posture", "posture": "attack"}}
        ]}),
    );
    assert_eq!(answer["refused_in_call"], 0, "{answer}");
}

fn end_day(session: &mut Session) {
    for side in [Side::Allied, Side::Japanese] {
        call(session, side, "end_day", &json!({}));
    }
}

fn briefing(session: &mut Session, side: Side) -> Value {
    call(session, side, "get_briefing", &json!({"text": false}))
}

fn text(session: &mut Session, side: Side) -> String {
    call(session, side, "get_briefing", &json!({"text": true}))["text"]
        .as_str()
        .expect("text")
        .to_string()
}

#[test]
fn the_japanese_side_is_told_of_its_failed_attacks_and_its_spent_fall_back() {
    let mut s = session_of(&scenario_text(NAME));
    let schemas = schemas();
    give_orders(&mut s);
    end_day(&mut s);
    let b = briefing(&mut s, Side::Japanese);
    valid(&schemas, "briefing.schema.json", &b, "after day 1");
    let land = &b["land_units"];
    let t = text(&mut s, Side::Japanese);
    let failed = land["attacks_failed_yesterday"].as_array().expect("failed");
    assert!(!failed.is_empty(), "{land}");
    for f in failed {
        assert!(f["event"].is_u64());
        assert!(f["place"]["q"].is_i64());
        assert!(f.get("weight").is_none());
    }
    assert!(t.contains("Attack failed yesterday at"), "{t}");
    // The spent units fell back in the same day's resolution: told whole.
    for f in land["fell_back_yesterday"].as_array().expect("fell back") {
        assert_eq!(f["reason"], "attack_spent", "{f}");
    }
    assert!(t.contains("its attack spent"), "{t}");
    assert!(!t.contains("after the defeat"), "{t}");
}

#[test]
fn the_unit_line_shows_the_instruction_and_the_reason_it_is_falling_back() {
    let mut s = session_of(&scenario_text(NAME));
    let schemas = schemas();
    give_orders(&mut s);
    let line = |b: &Value, handle: &str| -> Value {
        b["land_units"]["units"]
            .as_array()
            .expect("units")
            .iter()
            .find(|u| u["land_unit"]["handle"] == handle)
            .cloned()
            .expect("the unit")
    };
    end_day(&mut s);
    let b = briefing(&mut s, Side::Japanese);
    valid(&schemas, "briefing.schema.json", &b, "day 2");
    // The order of the regiment at K, still on attack, carries its
    // instruction, which the line shows and the text says after the posture;
    // the order of a unit that was spent is gone with its instruction.
    assert_eq!(line(&b, "land_unit:1")["when_spent"], "hold");
    assert!(line(&b, "land_unit:2").get("when_spent").is_none());
    assert!(text(&mut s, Side::Japanese).contains("when its attack is spent: hold"));
    for handle in ["land_unit:2", "land_unit:3"] {
        let march = &line(&b, handle)["march"];
        assert_eq!(march["falling_back"]["reason"], "attack_spent", "{march}");
        assert!(march.get("order").is_none());
    }
    assert!(line(&b, "land_unit:1").get("march").is_none());
    let t = text(&mut s, Side::Japanese);
    assert!(t.contains("after its attack at"), "{t}");
    assert!(t.contains("was spent"), "{t}");
    assert!(!t.contains("after the defeat at"), "{t}");
}

#[test]
fn the_allied_side_is_told_nothing_of_a_failed_attack_or_a_spent_fall_back() {
    let mut s = session_of(&scenario_text(NAME));
    let schemas = schemas();
    give_orders(&mut s);
    for day in 1..=4 {
        end_day(&mut s);
        let b = briefing(&mut s, Side::Allied);
        valid(&schemas, "briefing.schema.json", &b, "allied");
        let land = &b["land_units"];
        // The Allied side's own attack at P may fail (from day 3 it does),
        // and it is told of that; of the Japanese attacks at K, L and N,
        // which fail every day, nothing.
        for f in land["attacks_failed_yesterday"]
            .as_array()
            .into_iter()
            .flatten()
        {
            assert_eq!(f["place"]["q"], 30, "day {day}: {f}");
        }
        // The Allied side may have a unit of its own beaten and falling back;
        // never one whose attack was spent, and never the enemy's.
        for f in land["fell_back_yesterday"].as_array().into_iter().flatten() {
            assert_eq!(f["reason"], "beaten", "day {day}: {f}");
        }
        let json = b.to_string();
        for word in [
            "ground_attack_failed",
            "attack_spent",
            "when_spent",
            "enemy_weight",
            "shots_before_limit",
            "falling_back",
        ] {
            assert!(!json.contains(word), "{word} in {json}");
        }
        let t = text(&mut s, Side::Allied);
        for line in t.lines().filter(|l| l.contains("Attack failed")) {
            assert!(line.contains("(30,0)"), "day {day}: {line}");
        }
        for word in ["attack spent", "when its attack is spent"] {
            assert!(!t.contains(word), "{word} in {t}");
        }
    }
}

#[test]
fn an_order_with_when_spent_is_taken_and_a_defend_order_with_it_is_refused() {
    let mut s = session_of(&scenario_text(NAME));
    let schemas = schemas();
    let answer = call(
        &mut s,
        Side::Japanese,
        "issue_orders",
        &json!({"orders": [
            {"unit": "land_unit:1", "order": {"kind": "set_posture", "posture": "defend", "when_spent": "hold"}},
            {"unit": "land_unit:1", "order": {"kind": "set_posture", "posture": "attack", "when_spent": "hold"}}
        ]}),
    );
    valid(&schemas, "issue_orders.schema.json", &answer, "when_spent");
    println!("{answer:#}");
    let results = answer["results"].as_array().expect("results");
    assert_eq!(results[0]["status"], "refused", "{answer}");
    assert!(
        results[0]["reason"]
            .to_string()
            .contains("when_spent says what a unit does"),
        "{answer}"
    );
    assert_eq!(results[1]["status"], "queued", "{answer}");
}

#[test]
fn the_japanese_instruction_changes_nothing_the_allied_side_is_told() {
    let mut a = session_of(&scenario_text(NAME));
    let mut b = session_of(&scenario_text(NAME));
    give_orders(&mut a);
    give_orders(&mut b);
    // Unit 2, the battalion at L, is told to fall back in one game and to
    // hold in the other; the later order replaces the earlier.
    for (session, instruction) in [(&mut a, "fall_back"), (&mut b, "hold")] {
        call(
            session,
            Side::Japanese,
            "issue_orders",
            &json!({"orders": [
                {"unit": "land_unit:2", "order": {"kind": "set_posture", "posture": "attack", "when_spent": instruction}}
            ]}),
        );
    }
    for day in 1..=3 {
        end_day(&mut a);
        end_day(&mut b);
        let ja = briefing(&mut a, Side::Allied);
        let jb = briefing(&mut b, Side::Allied);
        assert_eq!(ja["land_units"], jb["land_units"], "day {day}");
        assert_eq!(
            text(&mut a, Side::Allied),
            text(&mut b, Side::Allied),
            "day {day}"
        );
    }
}
