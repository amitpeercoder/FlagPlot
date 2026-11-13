//! The air group line's `would_strike` (decision 6 of 0063): the contact a
//! naval strike group would launch at if the strike decision were made now.
//!
//! The briefing takes it from `SideView::strike_choice`, the function the
//! strike decision calls. These tests work it out again from what the
//! briefing itself tells the player: the group's place, radius or reach and
//! target list, and the contacts held with their place, kind and age. So a
//! player can check the line from the briefing alone.

mod support;

use serde_json::Value;
use sim_agent::briefing;
use sim_core::hex::Hex;
use sim_core::state::Side;
use support::{load, load_text, play, scenario_text};

/// The order of importance of the kinds: the default target list.
const KINDS: [&str; 6] = [
    "carrier",
    "battleship",
    "cruiser",
    "destroyer",
    "transport",
    "auxiliary",
];

/// `air_combat.toml: strike_contact_max_age_pulses` in the frozen step 1
/// figures, which these tests load.
fn max_age_pulses() -> u64 {
    0
}

fn hex_of(place: &Value) -> Hex {
    let coord =
        |key: &str| i32::try_from(place[key].as_i64().expect("a coordinate")).expect("fits");
    Hex::new(coord("q"), coord("r"))
}

fn number(handle: &Value) -> u64 {
    handle["handle"]
        .as_str()
        .and_then(|h| h.rsplit(':').next())
        .and_then(|n| n.parse().ok())
        .expect("a handle ends in a number")
}

/// The contact the briefing's own figures say the group would strike:
/// (handle, kind, distance).
fn worked_out(group: &Value, held: &[Value]) -> Option<(String, String, u64)> {
    if group["mission"] != "naval_strike" || group["ready"].as_u64() == Some(0) {
        return None;
    }
    if group["place"].is_null() {
        return None;
    }
    let origin = hex_of(&group["place"]);
    let miles = |key: &str| -> Option<u64> { group[key].as_str()?.parse::<u64>().ok() };
    let reach = miles("reach_nm").or_else(|| miles("strike_radius_nm"))?;
    let list: Vec<String> = match group["target_priority"].as_array() {
        Some(kinds) => kinds
            .iter()
            .map(|k| k.as_str().expect("a kind").to_string())
            .collect(),
        None => KINDS.iter().map(|k| (*k).to_string()).collect(),
    };
    held.iter()
        .filter(|c| c["age_pulses"].as_u64().expect("an age") <= max_age_pulses())
        .filter_map(|c| {
            let kind = c["taken_for"].as_str().expect("a kind");
            let rank = list.iter().position(|k| k == kind)?;
            let distance = u64::from(origin.air_distance_nm(hex_of(&c["place"])));
            (distance <= reach).then(|| {
                (
                    (rank, distance, number(&c["contact"])),
                    c["contact"]["handle"]
                        .as_str()
                        .expect("a handle")
                        .to_string(),
                    kind.to_string(),
                )
            })
        })
        .min()
        .map(|((_, distance, _), handle, kind)| (handle, kind, distance))
}

/// Checks every air group line of one side's briefing. Returns how many
/// lines said a group would strike, and how many of those had a list of
/// their own.
fn check(made: &Value, label: &str) -> (usize, usize) {
    let held = made["contacts"]["held"].as_array().expect("contacts held");
    let mut found = (0, 0);
    for group in made["air_groups"].as_array().expect("air groups") {
        let stated = group.get("would_strike").filter(|v| !v.is_null()).map(|w| {
            (
                w["contact"]["handle"]
                    .as_str()
                    .expect("a handle")
                    .to_string(),
                w["taken_for"].as_str().expect("a kind").to_string(),
                w["distance_nm"].as_u64().expect("whole miles"),
            )
        });
        assert_eq!(
            stated,
            worked_out(group, held),
            "{label}: {}",
            group["air_group"]["handle"]
        );
        if stated.is_some() {
            found.0 += 1;
            if group.get("target_priority").is_some_and(|v| !v.is_null()) {
                found.1 += 1;
            }
        }
    }
    found
}

#[test]
fn the_line_names_the_contact_the_briefings_own_figures_give() {
    let mut found = (0, 0);
    let mut scenarios = vec![
        load("05_eastern_solomons.toml", 1942),
        load("05d_carrier_duel.toml", 1942),
        load("07_full_slice.toml", 1942),
    ];
    // Scenario 13a played for a second day, so that a briefing is made with
    // its target lists in force.
    let two_days = scenario_text("13a_target_priority.toml").replacen("days = 1", "days = 2", 1);
    for seed in 1..=40 {
        scenarios.push(load_text(&two_days, "13a_target_priority.toml", seed));
    }
    // The Eastern Solomons with a target list for Saratoga's dive bombers,
    // VB-3: one that has the carrier contact on it, and one that does not,
    // so the group holds.
    let listed = |kinds: &str| {
        format!(
            "{}\n[[order]]\nday = 1\nside = \"allied\"\nair_group = 6\nkind = \"set_target_priority\"\nkinds = [{kinds}]\n",
            scenario_text("05_eastern_solomons.toml")
        )
    };
    for kinds in ["\"transport\", \"carrier\"", "\"battleship\""] {
        scenarios.push(load_text(&listed(kinds), "05_eastern_solomons.toml", 1942));
    }
    for mut scenario in scenarios {
        let name = scenario.name.clone();
        play(&mut scenario, |s| {
            for side in [Side::Allied, Side::Japanese] {
                let view = s.world.side_view(side);
                let made = serde_json::to_value(briefing(&view, &name)).expect("it serialises");
                let (all, listed) = check(&made, &format!("{name}, day {}, {side:?}", view.day()));
                found.0 += all;
                found.1 += listed;
            }
        });
    }
    println!(
        "lines that would strike: {}, with a list of their own: {}",
        found.0, found.1
    );
    assert!(found.0 > 0 && found.1 > 0, "{found:?}");
}

#[test]
fn after_the_eastern_solomons_the_dive_bombers_would_strike_the_contact() {
    // On this seed Ryujo sank in the afternoon, and the strike reports after it
    // read the ships left afloat (differences row 7), so the contact is held
    // as a "cruiser". It is 160 miles from the carriers since Ryujo's
    // detachment reaches (7,-9) at noon (decision 0063, "As built (the
    // scenario orders)"); it was 200. The dive bombers' radius is 200 and the
    // torpedo bombers' 190, so every strike group aboard with aircraft would
    // strike it, each within its own radius, and a group warned out of reach
    // (none is, at this distance) would not.
    let mut scenario = load("05_eastern_solomons.toml", 1942);
    scenario.run_to_end().expect("the scenario runs");
    let view = scenario.world.side_view(Side::Allied);
    let made = briefing(&view, "Eastern Solomons");
    let value = serde_json::to_value(&made).expect("it serialises");
    // Aboard the carriers the contact is 160 miles off; from Henderson Field
    // it is 200.
    let (mut would, mut carrier_groups) = (0, 0);
    for group in value["air_groups"].as_array().expect("air groups") {
        let Some(w) = group.get("would_strike").filter(|v| !v.is_null()) else {
            continue;
        };
        would += 1;
        assert_eq!(group["mission"], "naval_strike");
        let radius = group["strike_radius_nm"].as_str().expect("a radius");
        assert!(radius == "200" || radius == "190", "{radius}");
        assert_ne!(group["ready"], 0);
        assert_eq!(w["contact"]["handle"], "contact:1");
        assert_eq!(w["taken_for"], "cruiser");
        let distance = w["distance_nm"].as_u64().expect("a distance");
        assert!(distance == 160 || distance == 200, "{distance}");
        assert!(distance <= radius.parse::<u64>().expect("a whole radius"));
        if distance == 160 {
            carrier_groups += 1;
        }
    }
    assert!(would > 0 && carrier_groups > 0);
    for warned in value["warnings"]["strike_group_out_of_reach"]
        .as_array()
        .expect("list")
    {
        let line = value["air_groups"]
            .as_array()
            .expect("air groups")
            .iter()
            .find(|g| g["air_group"]["handle"] == warned["air_group"]["handle"])
            .expect("a warned group has a line");
        assert!(line.get("would_strike").is_none_or(Value::is_null));
    }
    assert!(made
        .to_text()
        .contains("would strike contact:1 now (cruiser, 160 miles)"));
}
