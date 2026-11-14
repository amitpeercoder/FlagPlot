//! A beaten side falls back, in what a side is told (decision 0104, 8).
//!
//! Scenario 32. On day 1 the Japanese working unit 1 at P is beaten with no
//! fight and falls back to R (a base of its side); the Japanese battalion 3
//! at T is beaten in a fight and falls back to V; the Japanese working unit 6
//! at S, a hex with no land link, surrenders its 200 men. The Japanese side
//! is told all three, each with its unit. The Allied side is told of the
//! surrender, the hex and the men, and nothing of the falling back: not
//! where, not that there was one, and nothing of the marches.

mod support;

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::state::Side;
use support::check::valid;
use support::{load_text, scenario_text, schemas};

const NAME: &str = "32_falling_back.toml";

fn session_of(text: &str) -> Session {
    let scenario = load_text(text, NAME, 1942);
    Session::new(
        scenario.world,
        "Back",
        scenario.days,
        Seat::Player,
        Seat::Player,
    )
}

fn call(session: &mut Session, side: Side, tool: &str, args: &Value) -> Value {
    session.call(side, tool, args).expect("the tool answers")
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

fn item<'a>(list: &'a Value, handle: &str) -> &'a Value {
    list.as_array()
        .expect("a list")
        .iter()
        .find(|x| x["land_unit"]["handle"] == handle)
        .unwrap_or_else(|| panic!("{handle} in {list}"))
}

fn unit_line<'a>(land: &'a Value, handle: &str) -> &'a Value {
    land["units"]
        .as_array()
        .expect("units")
        .iter()
        .find(|u| u["land_unit"]["handle"] == handle)
        .unwrap_or_else(|| panic!("{handle} in units"))
}

#[test]
fn the_japanese_side_is_told_of_its_falling_back_and_its_surrender() {
    let mut s = session_of(&scenario_text(NAME));
    let schemas = schemas();
    end_day(&mut s);
    let b = briefing(&mut s, Side::Japanese);
    valid(&schemas, "briefing.schema.json", &b, "falling back");
    let land = &b["land_units"];

    let fell = &land["fell_back_yesterday"];
    let one = item(fell, "land_unit:1");
    assert_eq!(
        (one["from"]["q"].as_i64(), one["from"]["r"].as_i64()),
        (Some(0), Some(0))
    );
    assert_eq!(
        (one["to"]["q"].as_i64(), one["to"]["r"].as_i64()),
        (Some(1), Some(0))
    );
    assert_eq!(one["men"], 300);
    let three = item(fell, "land_unit:2");
    assert_eq!(
        (three["to"]["q"].as_i64(), three["to"]["r"].as_i64()),
        (Some(11), Some(0))
    );
    assert_eq!(fell.as_array().unwrap().len(), 2);

    let surrendered = &land["surrendered_yesterday"];
    let six = item(surrendered, "land_unit:3");
    assert_eq!(six["men"], 200);
    assert_eq!(
        (six["place"]["q"].as_i64(), six["place"]["r"].as_i64()),
        (Some(20), Some(0))
    );
    assert!(land.get("prisoners_taken_yesterday").is_none());

    // The unit's own line says it is falling back, with no order.
    let march = &unit_line(land, "land_unit:1")["march"];
    assert_eq!(march["to"]["q"], 1);
    assert_eq!(march["falling_back"]["from"]["q"], 0);
    assert!(march.get("order").is_none());

    let t = text(&mut s, Side::Japanese);
    assert!(t.contains("falling back to"), "{t}");
    assert!(t.contains("after the defeat at"), "{t}");
    assert!(t.contains("carries no supply"), "{t}");
    assert!(t.contains("Fell back yesterday"), "{t}");
    assert!(t.contains("Surrendered yesterday"), "{t}");
}

#[test]
fn the_allied_side_is_told_of_the_surrender_and_nothing_of_the_falling_back() {
    let mut s = session_of(&scenario_text(NAME));
    let schemas = schemas();
    end_day(&mut s);
    let b = briefing(&mut s, Side::Allied);
    valid(&schemas, "briefing.schema.json", &b, "surrender seen");
    let land = &b["land_units"];
    let p = land["prisoners_taken_yesterday"]
        .as_array()
        .expect("prisoners");
    assert_eq!(p.len(), 1);
    assert_eq!(p[0]["men"], 200);
    assert_eq!(
        (p[0]["place"]["q"].as_i64(), p[0]["place"]["r"].as_i64()),
        (Some(20), Some(0))
    );
    assert!(land.get("fell_back_yesterday").is_none());
    assert!(land.get("surrendered_yesterday").is_none());

    let t = text(&mut s, Side::Allied);
    assert!(t.contains("An enemy unit surrendered at"), "{t}");
    assert!(t.contains("200 men"), "{t}");
    for word in [
        "fell back",
        "falling back",
        "carries no supply",
        "Surrendered",
    ] {
        assert!(!t.contains(word), "{word} in {t}");
    }
    let json = b.to_string();
    for word in ["falling_back", "fell_back", "cause_event"] {
        assert!(!json.contains(word), "{word} in {json}");
    }
}

/// The scenario with the Japanese base 2, at R, taken out: unit 1 then falls
/// back to Q.
fn without_base_2() -> String {
    let text = scenario_text(NAME);
    let at = text.find("[[base]]\nid = 2\n").expect("base 2");
    let end = text[at + 1..].find("[[base]]").expect("the next base") + at + 1;
    format!("{}{}", &text[..at], &text[end..])
}

#[test]
fn where_the_enemy_falls_back_to_changes_nothing_the_allied_side_is_told() {
    let mut a = session_of(&scenario_text(NAME));
    let mut b = session_of(&without_base_2());
    // The Japanese side does go to a different hex.
    end_day(&mut a);
    end_day(&mut b);
    let to = |s: &mut Session| {
        let j = briefing(s, Side::Japanese);
        let fell = j["land_units"]["fell_back_yesterday"].clone();
        item(&fell, "land_unit:1")["to"]["q"].as_i64()
    };
    // R is (1,0) and Q is (0,1).
    assert_eq!(to(&mut a), Some(1));
    assert_eq!(to(&mut b), Some(0));
    for day in 1..=6 {
        for side in [Side::Allied] {
            let (ja, jb) = (briefing(&mut a, side), briefing(&mut b, side));
            if ja != jb {
                for key in ja.as_object().unwrap().keys() {
                    // The result of the last day is the umpire's: it counts
                    // the men the Japanese side lost, and unit 1 at Q has no
                    // base to feed it.
                    if day == 6 && key == "objectives" {
                        continue;
                    }
                    assert_eq!(ja[key], jb[key], "json {key}, day {day}");
                }
            }
            // The text of the last day carries the result too.
            if day < 6 {
                assert_eq!(text(&mut a, side), text(&mut b, side), "text, day {day}");
            }
        }
        if day < 6 {
            end_day(&mut a);
            end_day(&mut b);
        }
    }
}
