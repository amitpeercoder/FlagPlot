//! A land unit's skills in what a side is told (decision 0098).
//!
//! Scenario 29 gives some units skills: the Allied side's units 1 to 9 (ids 1
//! to 8 and 10) are `land_unit:1` to `land_unit:9` of that side, and the
//! Japanese side's units (ids 9 and 11) are its `land_unit:1` and
//! `land_unit:2`. A side is shown the skills of its own units, in their lines
//! of the briefing, `list_forces` and `get_entity` and in the text, and
//! nothing of the other side's: it has no line for an enemy land unit, and
//! asked for one by handle it is told "not known".

mod support;

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::state::Side;
use support::check::valid;
use support::{load, schemas};

fn session(scenario: &str) -> Session {
    let scenario = load(scenario, 1942);
    Session::new(
        scenario.world,
        "Skills",
        scenario.days,
        Seat::Player,
        Seat::Player,
    )
}

fn call(session: &mut Session, side: Side, tool: &str, args: &Value) -> Value {
    session.call(side, tool, args).expect("the tool answers")
}

fn skills_of(line: &Value) -> [i64; 7] {
    let s = &line["skills"];
    let m = &s["movement"];
    [
        s["assault"].as_i64().expect("assault"),
        s["holding"].as_i64().expect("holding"),
        s["weight"].as_i64().expect("weight"),
        m["clear"].as_i64().expect("clear"),
        m["jungle"].as_i64().expect("jungle"),
        m["mountain"].as_i64().expect("mountain"),
        m["atoll"].as_i64().expect("atoll"),
    ]
}

fn unit<'a>(list: &'a Value, handle: &str) -> &'a Value {
    list.as_array()
        .expect("a list")
        .iter()
        .find(|x| x["land_unit"]["handle"] == handle)
        .unwrap_or_else(|| panic!("{handle} in {list}"))
}

// Order: assault, holding, weight, clear, jungle, mountain, atoll.
const STANDARD: [i64; 7] = [2; 7];

#[test]
fn a_side_sees_the_skills_of_its_own_units_in_every_answer_that_has_a_unit_line() {
    let mut s = session("29_skills.toml");
    let schemas = schemas();

    let briefing = call(
        &mut s,
        Side::Allied,
        "get_briefing",
        &json!({"text": false}),
    );
    valid(&schemas, "briefing.schema.json", &briefing, "skills");
    let units = &briefing["land_units"]["units"];
    assert_eq!(units.as_array().unwrap().len(), 9);
    assert_eq!(skills_of(unit(units, "land_unit:1")), [2, 2, 2, 2, 2, 4, 2]);
    assert_eq!(skills_of(unit(units, "land_unit:2")), STANDARD);
    assert_eq!(skills_of(unit(units, "land_unit:3")), [2, 2, 2, 2, 2, 0, 2]);
    assert_eq!(skills_of(unit(units, "land_unit:5")), [4, 2, 2, 2, 2, 2, 2]);
    assert_eq!(skills_of(unit(units, "land_unit:7")), [0, 2, 2, 2, 2, 2, 2]);
    assert_eq!(skills_of(unit(units, "land_unit:8")), [2, 2, 4, 2, 2, 2, 2]);
    assert_eq!(skills_of(unit(units, "land_unit:9")), [2, 4, 2, 2, 2, 2, 2]);

    // The same line through list_forces and get_entity.
    let forces = call(&mut s, Side::Allied, "list_forces", &json!({}));
    valid(&schemas, "list_forces.schema.json", &forces, "skills");
    assert_eq!(
        skills_of(unit(&forces["land_units"], "land_unit:1")),
        [2, 2, 2, 2, 2, 4, 2]
    );
    let entity = call(
        &mut s,
        Side::Allied,
        "get_entity",
        &json!({"handle": "land_unit:8"}),
    );
    valid(&schemas, "entity.schema.json", &entity, "skills");
    let line = &entity["entity"]["summary"];
    assert_eq!(skills_of(line), [2, 2, 4, 2, 2, 2, 2], "{entity}");

    // The text says them too.
    let text = call(&mut s, Side::Allied, "get_briefing", &json!({"text": true}));
    let text = text["text"].as_str().expect("text");
    assert!(
        text.contains("skills assault 2, holding 2, weight 2, movement clear 2, jungle 2, mountain 4, atoll 2"),
        "{text}"
    );
}

#[test]
fn nothing_of_the_skills_reaches_the_other_side() {
    let mut s = session("29_skills.toml");

    // The Japanese side's weight 0 unit is its own `land_unit:1`; the Allied
    // side sees nine skill lines, one for each of its own units, and no
    // more, in any answer, whatever the Japanese units are good at.
    for (side, own) in [(Side::Allied, 9), (Side::Japanese, 2)] {
        let briefing = call(&mut s, side, "get_briefing", &json!({"text": false}));
        assert_eq!(
            briefing.to_string().matches("\"skills\"").count(),
            own,
            "{side:?} briefing"
        );
        let forces = call(&mut s, side, "list_forces", &json!({}));
        assert_eq!(
            forces.to_string().matches("\"skills\"").count(),
            own,
            "{side:?} list_forces"
        );
        let text = call(&mut s, side, "get_briefing", &json!({"text": true}));
        assert_eq!(
            text["text"]
                .as_str()
                .unwrap()
                .matches("skills assault")
                .count(),
            own,
            "{side:?} text"
        );
    }
    let japanese = call(&mut s, Side::Japanese, "list_forces", &json!({}));
    assert_eq!(
        skills_of(unit(&japanese["land_units"], "land_unit:1")),
        [2, 2, 0, 2, 2, 2, 2]
    );

    // A handle past a side's own units is not known to it.
    let allied_tenth = call(
        &mut s,
        Side::Allied,
        "get_entity",
        &json!({"handle": "land_unit:10"}),
    );
    assert!(
        !allied_tenth.to_string().contains("skills"),
        "{allied_tenth}"
    );
}

#[test]
fn a_march_and_its_day_show_the_days_counted() {
    let mut s = session("25_march.toml");
    let order = call(
        &mut s,
        Side::Allied,
        "issue_orders",
        &json!({"orders": [{"unit": "land_unit:1", "order": {"kind": "march", "hex": [1, 0]}}]}),
    );
    assert_eq!(order["results"][0]["status"], "queued", "{order}");
    for side in [Side::Allied, Side::Japanese] {
        call(&mut s, side, "end_day", &json!({}));
    }
    let schemas = schemas();
    let briefing = call(
        &mut s,
        Side::Allied,
        "get_briefing",
        &json!({"text": false}),
    );
    valid(&schemas, "briefing.schema.json", &briefing, "counted");
    let line = unit(&briefing["land_units"]["units"], "land_unit:1");
    let march = &line["march"];
    // A unit of standard movement counts each day marched as a day.
    assert_eq!(march["days_marched"], 1, "{march}");
    assert_eq!(march["days_counted"], "1", "{march}");
    let day = &briefing["land_units"]["marches_yesterday"][0];
    assert_eq!(day["days"], 1, "{day}");
    assert_eq!(day["counted"], "1", "{day}");
    let text = call(&mut s, Side::Allied, "get_briefing", &json!({"text": true}));
    let text = text["text"].as_str().unwrap();
    assert!(text.contains("(1 counted)"), "{text}");
}
