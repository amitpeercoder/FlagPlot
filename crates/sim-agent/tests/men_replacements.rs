//! Men who replace the lost in the briefing (decision 0197): a side's own
//! streams are listed under what is due, in `arrivals.men_replacements`; the
//! men who joined a unit are listed in `men_joined_yesterday`; a land unit's
//! line gives its `establishment`. The enemy is told none of it, and a game
//! with no stream and no establishment has no such key.
//!
//! Scenario 113: Japanese base 2 gives 100 men a night from day 2 to the
//! units ashore there below their establishment: unit 1 (400 of 600,
//! experience 4), unit 2 (500 of 500), unit 3 (250 of 300).

mod support;

use serde_json::{json, Value};
use sim_core::state::Side;
use support::play::session_of;
use support::{frozen, schemas, ROOT};

fn session() -> sim_agent::Session {
    let path = std::path::Path::new(ROOT)
        .join("data/scenarios/men-who-replace-the-lost/113_men_who_replace_the_lost.toml");
    let text = std::fs::read_to_string(&path).expect("the scenario reads");
    let scenario = sim_data::Scenario::from_text_with(&text, &path, &frozen())
        .expect("the scenario loads")
        .with_seed(1942)
        .expect("a seed can be set");
    session_of(scenario, &[Side::Allied, Side::Japanese]).0
}

fn call(session: &mut sim_agent::Session, side: Side, tool: &str, args: Value) -> Value {
    session
        .call(side, tool, &args)
        .unwrap_or_else(|e| panic!("{tool} was refused: {e}"))
}

fn end_day(s: &mut sim_agent::Session) {
    for side in [Side::Japanese, Side::Allied] {
        call(s, side, "end_day", json!({}));
    }
}

fn unit_line(forces: &Value, handle: &str) -> Value {
    forces["land_units"]
        .as_array()
        .expect("land units")
        .iter()
        .find(|u| u["land_unit"]["handle"] == handle)
        .unwrap_or_else(|| panic!("no {handle}: {forces}"))
        .clone()
}

#[test]
fn the_stream_is_listed_for_its_side_only() {
    let mut s = session();
    let japanese = call(&mut s, Side::Japanese, "get_briefing", json!({}));
    let streams = japanese["arrivals"]["men_replacements"]
        .as_array()
        .expect("streams");
    assert_eq!(streams.len(), 1, "{japanese}");
    assert_eq!(streams[0]["base"]["handle"], "base:2");
    assert_eq!(streams[0]["men_per_day"], 100);
    assert_eq!(streams[0]["first_day"], 2);
    let text = call(
        &mut s,
        Side::Japanese,
        "get_briefing",
        json!({"text": true}),
    );
    assert!(text["text"]
        .as_str()
        .expect("text")
        .contains("Replacement men"));

    let allied = call(&mut s, Side::Allied, "get_briefing", json!({}));
    assert!(!allied["arrivals"]
        .as_object()
        .expect("arrivals")
        .contains_key("men_replacements"));
    let text = call(&mut s, Side::Allied, "get_briefing", json!({"text": true}));
    assert!(!text["text"]
        .as_str()
        .expect("text")
        .contains("Replacement men"));
}

#[test]
fn the_men_who_joined_are_told_to_their_side_only() {
    let mut s = session();
    let before = call(&mut s, Side::Japanese, "get_briefing", json!({}));
    assert!(!before["land_units"]
        .to_string()
        .contains("men_joined_yesterday"));
    // The stream gives in day 2's night, so day 3's briefing tells of it.
    end_day(&mut s);
    end_day(&mut s);
    let schemas = schemas();
    let japanese = call(&mut s, Side::Japanese, "get_briefing", json!({}));
    assert!(
        schemas.check("briefing.schema.json", &japanese).is_empty(),
        "{japanese}"
    );
    let lines = japanese["land_units"]["men_joined_yesterday"]
        .as_array()
        .unwrap_or_else(|| panic!("{japanese}"));
    let one = lines
        .iter()
        .find(|l| l["land_unit"]["handle"] == "land_unit:1")
        .expect("unit 1");
    assert_eq!(one["men"], 100);
    assert_eq!(one["men_after"], 500);
    assert_eq!(one["base"]["handle"], "base:2");
    assert!(one["experience_after"].as_u64() >= one["experience_before"].as_u64());
    let text = call(
        &mut s,
        Side::Japanese,
        "get_briefing",
        json!({"text": true}),
    );
    assert!(text["text"]
        .as_str()
        .expect("text")
        .contains("Men joined yesterday"));

    let allied = call(&mut s, Side::Allied, "get_briefing", json!({}));
    assert!(!allied.to_string().contains("men_joined_yesterday"));
}

#[test]
fn a_unit_line_gives_its_establishment() {
    let mut s = session();
    let forces = call(&mut s, Side::Japanese, "list_forces", json!({}));
    assert_eq!(unit_line(&forces, "land_unit:1")["establishment"], 600);
    assert_eq!(unit_line(&forces, "land_unit:2")["establishment"], 500);
    assert_eq!(unit_line(&forces, "land_unit:3")["establishment"], 300);
}

#[test]
fn a_game_with_no_stream_has_no_establishment_key() {
    let path = std::path::Path::new(ROOT).join("data/scenarios/23_fight_ashore.toml");
    let text = std::fs::read_to_string(&path).expect("the scenario reads");
    let scenario = sim_data::Scenario::from_text_with(&text, &path, &frozen())
        .expect("the scenario loads")
        .with_seed(1942)
        .expect("a seed can be set");
    let mut s = session_of(scenario, &[Side::Allied, Side::Japanese]).0;
    let mut seen = 0;
    for side in [Side::Allied, Side::Japanese] {
        let forces = call(&mut s, side, "list_forces", json!({}));
        for unit in forces["land_units"].as_array().expect("land units") {
            seen += 1;
            assert!(!unit
                .as_object()
                .expect("unit")
                .contains_key("establishment"));
        }
        let briefing = call(&mut s, side, "get_briefing", json!({}));
        assert!(!briefing["arrivals"]
            .as_object()
            .expect("arrivals")
            .contains_key("men_replacements"));
    }
    assert!(seen > 0);
}
