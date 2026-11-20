//! Stock from off the map in the briefing (decision 0196, 4 and 5): a side's
//! own supply lines are listed under what is due, in `arrivals.supply_lines`,
//! with their tons and days; the enemy's are never given, and a game with no
//! line has no such key.
//!
//! Scenario 72, base 1 Allied and base 2 Japanese. The Allied line at base 1
//! gives 12.5 tons of supply and 4 of fuel on days 1 to 3.

mod support;

use serde_json::{json, Value};
use sim_core::state::Side;
use support::play::session_of;
use support::{load_text, scenario_text};

fn session(line: bool) -> sim_agent::Session {
    let mut text = scenario_text("72_reconnaissance.toml");
    if line {
        text.push_str(
            "\n[[supply_line]]\nbase = 1\nsupply_t = \"12.5\"\nfuel_t = 4\nfirst_day = 1\nlast_day = 3\n",
        );
    }
    let scenario = load_text(&text, "72_reconnaissance.toml", 1942);
    session_of(scenario, &[Side::Allied, Side::Japanese]).0
}

fn call(session: &mut sim_agent::Session, side: Side, tool: &str, args: Value) -> Value {
    session
        .call(side, tool, &args)
        .unwrap_or_else(|e| panic!("{tool} was refused: {e}"))
}

#[test]
fn the_owning_side_is_told_of_its_line_and_the_enemy_is_not() {
    let mut s = session(true);
    let allied = call(&mut s, Side::Allied, "get_briefing", json!({}));
    let lines = allied["arrivals"]["supply_lines"]
        .as_array()
        .expect("lines");
    assert_eq!(lines.len(), 1, "{allied}");
    assert_eq!(lines[0]["base"]["handle"], "base:1");
    assert_eq!(lines[0]["supply_t"], "12.5");
    assert_eq!(lines[0]["fuel_t"], "4");
    assert_eq!(lines[0]["first_day"], 1);
    assert_eq!(lines[0]["last_day"], 3);
    let text = call(&mut s, Side::Allied, "get_briefing", json!({"text": true}));
    assert!(text["text"].as_str().expect("text").contains("Supply line"));

    let japanese = call(&mut s, Side::Japanese, "get_briefing", json!({}));
    assert!(
        !japanese["arrivals"]
            .as_object()
            .expect("arrivals")
            .contains_key("supply_lines"),
        "{japanese}"
    );
    assert!(!japanese.to_string().contains("12.5"));
    let text = call(
        &mut s,
        Side::Japanese,
        "get_briefing",
        json!({"text": true}),
    );
    assert!(!text["text"].as_str().expect("text").contains("Supply line"));
}

#[test]
fn a_game_with_no_line_has_no_key() {
    let mut s = session(false);
    for side in [Side::Allied, Side::Japanese] {
        let briefing = call(&mut s, side, "get_briefing", json!({}));
        assert!(!briefing["arrivals"]
            .as_object()
            .expect("arrivals")
            .contains_key("supply_lines"));
    }
}
