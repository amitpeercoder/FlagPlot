//! A unit's day, and a land unit's formation, as the agent interface gives
//! them (decision 0140, 5 and 8).
//!
//! `available_from` is given in a unit's line and detail (a whole number)
//! while the unit may not yet be used for the orders now being given
//! (`SideView::may_use` says no), and is left out once its day has come. It
//! is the day the refusal `... may be used from day N` names. A land unit's
//! `get_entity` gives its formation and the formations above it.

use crate::support;

use serde_json::{json, Value};
use sim_core::state::Side;
use support::play::{play_day, session_of};
use support::{load_text, scenario_text};

/// `text` with `line` added after the `side` line of the table `kind` whose
/// `id` is `id`.
fn add_to_table(text: &str, kind: &str, id: u32, line: &str) -> String {
    let head = format!("[[{kind}]]\nid = {id}\n");
    let at = text.find(&head).unwrap_or_else(|| panic!("no {kind} {id}"));
    let side = at + text[at..].find("side = ").expect("a side line");
    let end = side + text[side..].find('\n').expect("a line end") + 1;
    format!("{}{line}\n{}", &text[..end], &text[end..])
}

fn call(session: &mut sim_agent::Session, tool: &str, args: Value) -> Value {
    session
        .call(Side::Allied, tool, &args)
        .unwrap_or_else(|e| panic!("{tool} was refused: {e}"))
}

fn landing() -> sim_agent::Session {
    let mut text = scenario_text("21_landing.toml");
    // The convoy (force 1, with ships 1 and 2 and battalions 1 and 2 aboard)
    // may be used from day 3; battalion 3, ashore, is a battalion of a
    // regiment of a division.
    text = add_to_table(&text, "task_force", 1, "available_from = 3");
    text = add_to_table(&text, "land_unit", 3, "formation = 12");
    text.push_str(
        "\n[[formation]]\nid = 10\nname = \"Placeholder division\"\nside = \"allied\"\n\
         \n[[formation]]\nid = 11\nname = \"Placeholder regiment\"\nside = \"allied\"\nparent = 10\n\
         \n[[formation]]\nid = 12\nname = \"Placeholder battalion group\"\nside = \"allied\"\nparent = 11\n",
    );
    let scenario = load_text(&text, "21_landing.toml", 1942);
    let (session, _) = session_of(scenario, &[Side::Allied]);
    session
}

fn force_line(list: &Value, handle: &str) -> Value {
    list["task_forces"]
        .as_array()
        .expect("task forces")
        .iter()
        .find(|f| f["task_force"]["id"] == handle || f["task_force"]["handle"] == handle)
        .unwrap_or_else(|| panic!("{handle} not listed in {list}"))
        .clone()
}

#[test]
fn a_force_and_what_it_carries_show_their_day_until_it_comes() {
    let mut session = landing();
    // Day 1 (the view's day is 0): the orders now given are for day 1.
    let list = call(&mut session, "list_forces", json!({}));
    let text = list.to_string();
    assert_eq!(list["version"], sim_agent::VERSION);
    assert_eq!(force_line(&list, "task_force:1")["available_from"], 3);
    assert!(
        force_line(&list, "task_force:2")
            .get("available_from")
            .is_none(),
        "{text}"
    );
    // Battalions 1 and 2 are aboard the convoy and take its day; battalion 3
    // has none.
    let units = list["land_units"].as_array().expect("land units");
    let days: Vec<Option<u64>> = units
        .iter()
        .map(|u| u.get("available_from").and_then(Value::as_u64))
        .collect();
    assert_eq!(days, vec![Some(3), Some(3), None], "{text}");

    let force = call(
        &mut session,
        "get_entity",
        json!({"handle": "task_force:1"}),
    );
    assert_eq!(force["entity"]["summary"]["available_from"], 3);
    for ship in force["entity"]["ships"].as_array().expect("ships") {
        assert_eq!(ship["available_from"], 3, "{force}");
    }
    let ship = call(&mut session, "get_entity", json!({"handle": "ship:1"}));
    assert_eq!(ship["entity"]["available_from"], 3);
    let unit = call(&mut session, "get_entity", json!({"handle": "land_unit:1"}));
    assert_eq!(unit["entity"]["summary"]["available_from"], 3);
    let other = call(
        &mut session,
        "get_entity",
        json!({"handle": "task_force:2"}),
    );
    assert!(other["entity"]["summary"].get("available_from").is_none());

    // The briefing's text says it in words.
    let briefing = call(&mut session, "get_briefing", json!({"text": true}));
    assert!(
        briefing["text"]
            .as_str()
            .expect("text")
            .contains("available from day 3"),
        "{briefing}"
    );

    // An order to it is refused with the day.
    let refused = call(
        &mut session,
        "issue_orders",
        json!({"orders": [{"unit": "task_force:1", "order": {"kind": "set_speed", "speed": "full"}}]}),
    );
    let reason = refused["results"][0].to_string();
    assert!(reason.contains("may be used from day 3"), "{reason}");

    // Day 1 played: the orders now given are for day 2, still before day 3.
    play_day(&mut session, Side::Allied, &[]);
    let list = call(&mut session, "list_forces", json!({}));
    assert_eq!(force_line(&list, "task_force:1")["available_from"], 3);
    // Day 2 played: the orders now given are for day 3, and the day has come.
    play_day(&mut session, Side::Allied, &[]);
    let list = call(&mut session, "list_forces", json!({}));
    assert!(
        force_line(&list, "task_force:1")
            .get("available_from")
            .is_none(),
        "{list}"
    );
    assert!(!list.to_string().contains("available_from"), "{list}");
    let force = call(
        &mut session,
        "get_entity",
        json!({"handle": "task_force:1"}),
    );
    assert!(!force.to_string().contains("available_from"), "{force}");
}

#[test]
fn a_land_unit_gives_its_formation_and_the_formations_above_it() {
    let mut session = landing();
    let unit = call(&mut session, "get_entity", json!({"handle": "land_unit:3"}));
    let formation = &unit["entity"]["formation"];
    assert_eq!(formation["number"], 12, "{unit}");
    assert_eq!(formation["name"], "Placeholder battalion group");
    assert_eq!(
        formation["parents"],
        json!([
            {"number": 11, "name": "Placeholder regiment"},
            {"number": 10, "name": "Placeholder division"},
        ])
    );
    // A unit with no formation has none given.
    let unit = call(&mut session, "get_entity", json!({"handle": "land_unit:1"}));
    assert!(unit["entity"].get("formation").is_none(), "{unit}");
}

#[test]
fn an_air_group_shows_its_day() {
    let text = add_to_table(
        &scenario_text("05b_strike.toml"),
        "air_group",
        2,
        "available_from = 3",
    );
    let scenario = load_text(&text, "05b_strike.toml", 1942);
    let (mut session, _) = session_of(scenario, &[Side::Allied]);
    let list = call(&mut session, "get_briefing", json!({}));
    assert!(list.to_string().contains("\"available_from\":3"), "{list}");
    let group = call(&mut session, "get_entity", json!({"handle": "air_group:2"}));
    assert_eq!(group["entity"]["summary"]["available_from"], 3);
    let group = call(&mut session, "get_entity", json!({"handle": "air_group:1"}));
    assert!(group["entity"]["summary"].get("available_from").is_none());
}
