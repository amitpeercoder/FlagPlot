//! The wording pass after the second round of scenario 9 play (decision 0051,
//! "The wording pass after the second round"): what the tools tell a side,
//! each new field, warning and hint tested where it shows. Nothing here is a
//! rule: every figure is checked against what another tool or the engine
//! says.

mod support;

use std::collections::BTreeMap;

use serde_json::{json, Value};
use sim_agent::{
    briefing, get_entity, list_forces, measure, MeasureAnswer, Question, Seat, Session,
};
use sim_core::state::Side;
use support::check::{assert_numbers_in_data, valid};
use support::{load, play, schemas};

fn answer_value<T: serde::Serialize>(answer: &T) -> Value {
    serde_json::to_value(answer).expect("an answer serialises")
}

fn session_for(name: &str, seed: u64) -> Session {
    let scenario = load(name, seed);
    let title = scenario.name.clone();
    Session::new(
        scenario.world,
        &title,
        scenario.days,
        Seat::Player,
        Seat::Script(BTreeMap::new()),
    )
}

fn call(session: &mut Session, tool: &str, arguments: Value) -> Value {
    session
        .call(Side::Allied, tool, &arguments)
        .unwrap_or_else(|e| panic!("{tool} was refused as a whole: {}", e.to_json()))
}

fn text_of(session: &mut Session) -> String {
    call(session, "get_briefing", json!({"text": true}))["text"]
        .as_str()
        .expect("text")
        .to_string()
}

// ---- 1. The levels ----

#[test]
fn the_levels_are_in_the_briefing_from_day_one_as_data_and_as_text() {
    let schemas = schemas();
    for side in [Side::Allied, Side::Japanese] {
        let scenario = load("05_eastern_solomons.toml", 1942);
        let view = scenario.world.side_view(side);
        let made = briefing(&view, "Eastern Solomons");
        let json = answer_value(&made);
        valid(&schemas, "briefing.schema.json", &json, "briefing");
        assert_eq!(
            json["objectives"]["levels"],
            json!({"minor": "3", "major": "8", "decisive": "16"})
        );
        let text = made.to_text();
        assert!(
            text.contains(
                "A margin of 3 or more is a minor victory, 8 or more a major one, 16 or more a decisive one; the same margins against you are defeats; less than 3 either way is a stalemate"
            ),
            "{text}"
        );
        assert_numbers_in_data(&text, &json, "briefing text");
        // The accessor reads the scenario's own levels, not a side's.
        let levels = view.levels().expect("the scenario has levels");
        assert_eq!(levels.minor, sim_core::fixed::Fixed::from_int(3));
    }
}

#[test]
fn a_scenario_without_objectives_has_no_levels() {
    let scenario = load("01_transit.toml", 1);
    let view = scenario.world.side_view(Side::Allied);
    assert!(view.levels().is_none());
    let made = answer_value(&briefing(&view, "Transit"));
    assert_eq!(made["objectives"]["levels"], Value::Null);
}

#[test]
fn the_result_says_the_outcome_in_words_for_both_sides_and_labels_the_history() {
    let schemas = schemas();
    let mut session = session_for("05_eastern_solomons.toml", 1942);
    call(&mut session, "issue_orders", json!({"orders": []}));
    let ended = call(&mut session, "end_day", json!({}));
    valid(&schemas, "end_day.schema.json", &ended, "end_day");
    assert_eq!(ended["game_over"], true);
    let text = text_of(&mut session);
    let result = &ended["result"];
    for (name, side) in [("Allied", "allied"), ("Japanese", "japanese")] {
        let outcome = result[side]["outcome"].as_str().expect("an outcome");
        // A stalemate shows level null and says "stalemate" in `outcome`;
        // a victory or defeat names its level.
        if result[side]["result"] == "stalemate" {
            assert_eq!(result[side]["level"], Value::Null);
            assert_eq!(outcome, "stalemate");
        } else {
            assert!(result[side]["level"].is_string());
        }
        let line = text
            .lines()
            .find(|l| l.starts_with(&format!("- {name}: ")))
            .unwrap_or_else(|| panic!("no result line for {name}\n{text}"));
        assert!(
            line.ends_with(outcome),
            "{line} does not end with {outcome}"
        );
    }
    let history = result["history"]
        .as_str()
        .expect("scenario 5 has a history");
    assert!(
        text.contains(&format!(
            "History (what happened in the real battle, not in this game): {history}"
        )),
        "{text}"
    );
}

#[test]
fn a_stalemate_is_named_in_words_where_it_is_the_result() {
    // The result of a margin inside the minor level, in the words the
    // briefing gives it, from the engine's own scoring on plain counts.
    let levels = sim_core::objectives::Levels::new(
        sim_core::fixed::Fixed::from_int(3),
        sim_core::fixed::Fixed::from_int(8),
        sim_core::fixed::Fixed::from_int(16),
    )
    .expect("levels");
    let margin = sim_core::fixed::Fixed::from_int(2);
    assert_eq!(
        sim_core::objectives::outcome_of(margin, &levels).name(),
        "stalemate"
    );
}

// ---- 2. A strike group with nothing in reach ----

/// Every warning of a briefing, recomputed with `measure`: the group's radius,
/// the nearest place named and its distance are the figures `air_reach` gives,
/// and the place is beyond the radius.
fn check_out_of_reach(view: &sim_core::view::SideView<'_>, label: &str) -> usize {
    let made = answer_value(&briefing(view, label));
    let mut found = 0;
    for w in made["warnings"]["strike_group_out_of_reach"]
        .as_array()
        .expect("the warning list")
    {
        found += 1;
        let group = w["air_group"]["handle"].as_str().expect("a group");
        let nearest = w["nearest"]["handle"].as_str().expect("a place");
        let reach = measure(
            view,
            &Question::from_json(
                &json!({"question": "air_reach", "air_group": group, "to": [nearest]}).to_string(),
            )
            .expect("a question"),
        );
        let MeasureAnswer::AirReach(reach) = reach else {
            panic!("{label}: {group} to {nearest} is not known to measure");
        };
        assert_eq!(reach.places.len(), 1);
        assert!(!reach.places[0].within, "{label}: {w}");
        assert_eq!(w["radius_nm"], answer_value(&reach.radius_nm), "{label}");
        assert_eq!(
            w["distance_nm"].as_u64(),
            Some(u64::from(reach.places[0].distance_nm)),
            "{label}"
        );
        let mission = w["mission"].as_str().expect("a mission");
        let on = made["air_groups"]
            .as_array()
            .expect("groups")
            .iter()
            .find(|g| g["air_group"]["handle"] == group)
            .expect("the group is listed");
        assert_eq!(on["mission"], mission, "{label}");
    }
    // And no group on a strike mission is left out for a place in reach: a
    // naval_strike group with contacts and none in reach is warned.
    for g in made["air_groups"].as_array().expect("groups") {
        if g["mission"] != "naval_strike" {
            continue;
        }
        let group = g["air_group"]["handle"].as_str().expect("a group");
        let contacts: Vec<String> = made["contacts"]["held"]
            .as_array()
            .expect("held")
            .iter()
            .map(|c| {
                c["contact"]["handle"]
                    .as_str()
                    .expect("a handle")
                    .to_string()
            })
            .collect();
        if contacts.is_empty() {
            continue;
        }
        let MeasureAnswer::AirReach(reach) = measure(
            view,
            &Question::from_json(
                &json!({"question": "air_reach", "air_group": group, "to": contacts}).to_string(),
            )
            .expect("a question"),
        ) else {
            // A group with no place (lost with its ship) is nothing to
            // measure from, and is never warned.
            assert_eq!(g["place"], Value::Null, "{label}: {group} is not known");
            continue;
        };
        let none_within = reach.places.iter().all(|p| !p.within);
        let warned = made["warnings"]["strike_group_out_of_reach"]
            .as_array()
            .expect("list")
            .iter()
            .any(|w| w["air_group"]["handle"] == group);
        assert_eq!(none_within, warned, "{label}: {group}");
    }
    found
}

#[test]
fn a_strike_group_with_contacts_but_none_in_reach_is_warned_with_the_figures() {
    // Eastern Solomons after its day: the contact is 200 miles from the
    // carriers, the torpedo bombers' radius is 190 and the dive bombers' 200.
    let mut scenario = load("05_eastern_solomons.toml", 1942);
    scenario.run_to_end().expect("the scenario runs");
    let view = scenario.world.side_view(Side::Allied);
    let made = answer_value(&briefing(&view, "Eastern Solomons"));
    let warned: Vec<&str> = made["warnings"]["strike_group_out_of_reach"]
        .as_array()
        .expect("list")
        .iter()
        .map(|w| w["air_group"]["handle"].as_str().expect("handle"))
        .collect();
    assert_eq!(warned, ["air_group:4", "air_group:8", "air_group:10"]);
    let first = &made["warnings"]["strike_group_out_of_reach"][0];
    assert_eq!(first["mission"], "naval_strike");
    assert_eq!(first["radius_nm"], "190");
    assert_eq!(first["nearest"]["handle"], "contact:1");
    assert_eq!(first["distance_nm"], 200);
    let text = briefing(&view, "Eastern Solomons").to_text();
    assert!(
        text.contains("Out of reach: air_group:4 \"VT-3\" is on naval strike with a radius of 190 miles and holds contacts, but none is within it; the nearest is contact:1, 200 miles away"),
        "{text}"
    );
    assert_numbers_in_data(&text, &made, "briefing text");
    valid(&schemas(), "briefing.schema.json", &made, "briefing");
    assert_eq!(check_out_of_reach(&view, "Eastern Solomons"), 3);
}

#[test]
fn a_base_strike_beyond_the_radius_is_warned_in_the_scenarios_that_have_one() {
    let mut base_strikes = 0;
    let mut naval = 0;
    for name in ["06_rabaul_guadalcanal.toml", "07_full_slice.toml"] {
        let mut scenario = load(name, 1942);
        let title = scenario.name.clone();
        play(&mut scenario, |s| {
            for side in [Side::Allied, Side::Japanese] {
                let view = s.world.side_view(side);
                let made = answer_value(&briefing(&view, &title));
                for w in made["warnings"]["strike_group_out_of_reach"]
                    .as_array()
                    .expect("list")
                {
                    match w["mission"].as_str().expect("mission") {
                        "base_strike" => base_strikes += 1,
                        _ => naval += 1,
                    }
                }
                check_out_of_reach(&view, &title);
                valid(&schemas(), "briefing.schema.json", &made, "briefing");
            }
        });
    }
    // The scripted games send their base strikes within the radius.
    assert_eq!(base_strikes, 0);
    assert!(naval > 0, "no naval strike out of reach was seen");
}

#[test]
fn a_base_strike_at_a_base_beyond_the_radius_is_warned_with_the_distance() {
    // The Marine dive bombers of Henderson Field ordered at Rabaul, 556 miles
    // away.
    let mut session = session_for("07_full_slice.toml", 1942);
    let answer = call(
        &mut session,
        "issue_orders",
        json!({"orders": [{"unit": "air_group:11", "order":
            {"kind": "set_air_mission", "mission": "base_strike", "target_base": 2}}]}),
    );
    assert_eq!(answer["results"][0]["status"], "queued", "{answer}");
    call(&mut session, "end_day", json!({}));
    let made = call(&mut session, "get_briefing", json!({"text": false}));
    valid(&schemas(), "briefing.schema.json", &made, "briefing");
    let warnings = made["warnings"]["strike_group_out_of_reach"]
        .as_array()
        .expect("list");
    let warning = warnings
        .iter()
        .find(|w| w["air_group"]["handle"] == "air_group:11")
        .unwrap_or_else(|| panic!("no warning for air_group:11 in {warnings:?}"));
    assert_eq!(warning["mission"], "base_strike");
    assert_eq!(warning["nearest"]["handle"], "base:2");
    let distance = call(
        &mut session,
        "measure",
        json!({"question": "air_distance", "from": "air_group:11", "to": "base:2"}),
    );
    assert_eq!(warning["distance_nm"], distance["distance_nm"]);
    let reach = call(
        &mut session,
        "measure",
        json!({"question": "air_reach", "air_group": "air_group:11", "to": ["base:2"]}),
    );
    assert_eq!(warning["radius_nm"], reach["radius_nm"]);
    assert_eq!(reach["places"][0]["within"], false);
    let text = text_of(&mut session);
    assert!(
        text.contains(&format!(
            "Out of reach: air_group:11 \"{}\" is on base strike with a radius of {} miles, but its target base:2 \"{}\" is {} miles away",
            made["air_groups"]
                .as_array()
                .unwrap()
                .iter()
                .find(|g| g["air_group"]["handle"] == "air_group:11")
                .unwrap()["air_group"]["name"]
                .as_str()
                .unwrap(),
            warning["radius_nm"].as_str().unwrap(),
            warning["nearest"]["name"].as_str().unwrap(),
            warning["distance_nm"]
        )),
        "{text}"
    );
}

// ---- 3. The history ----

#[test]
fn the_history_field_is_described_as_the_real_battle_in_the_schema() {
    let text = std::fs::read_to_string(format!(
        "{}/schemas/briefing.schema.json",
        env!("CARGO_MANIFEST_DIR")
    ))
    .expect("the schema reads");
    assert!(text.contains("What happened in the real battle, not in this game"));
}

// ---- 4. The last day ----

#[test]
fn the_briefing_says_plainly_when_the_day_is_the_last() {
    let mut session = session_for("05_eastern_solomons.toml", 1942);
    let text = text_of(&mut session);
    assert!(
        text.contains("- Day: 1 of 1\n- This is the last day: the game ends when you end it.\n")
    );
    // A longer game says it on its last day only.
    let mut session = session_for("06a_base_raid.toml", 1942);
    let first = text_of(&mut session);
    assert!(!first.contains("This is the last day"), "{first}");
    for _ in 0..session.days() - 1 {
        call(&mut session, "issue_orders", json!({"orders": []}));
        call(&mut session, "end_day", json!({}));
    }
    let last = text_of(&mut session);
    assert!(
        last.contains("This is the last day: the game ends when you end it."),
        "{last}"
    );
    call(&mut session, "issue_orders", json!({"orders": []}));
    call(&mut session, "end_day", json!({}));
    let after = text_of(&mut session);
    assert!(!after.contains("This is the last day"), "{after}");
}

// ---- 5. Hints on refusals ----

fn refusal_of(session: &mut Session, unit: &str, body: Value) -> String {
    let answer = call(
        session,
        "issue_orders",
        json!({"orders": [{"unit": unit, "order": body}], "clear": true}),
    );
    assert_eq!(answer["results"][0]["status"], "refused", "{answer}");
    answer["results"][0]["reason"]
        .as_str()
        .expect("a reason")
        .to_string()
}

#[test]
fn a_load_refused_away_from_a_base_says_what_load_needs() {
    // Task force 1 lies at sea, idle.
    let mut session = session_for("05_eastern_solomons.toml", 1942);
    let reason = refusal_of(&mut session, "task_force:1", json!({"kind": "load"}));
    assert!(
        reason.starts_with("there is no base of your side here ("),
        "{reason}"
    );
    assert!(
        reason.contains("load needs the task force stopped in the hex of a base of your own side"),
        "{reason}"
    );
}

#[test]
fn a_base_strike_on_an_own_base_says_a_base_the_enemy_holds_is_needed() {
    let mut session = session_for("06a_base_raid.toml", 1942);
    let reason = refusal_of(
        &mut session,
        "air_group:4",
        json!({"kind": "set_air_mission", "mission": "base_strike", "target_base": 2}),
    );
    assert!(
        reason.starts_with("the base does not exist or is held by your side ("),
        "{reason}"
    );
    assert!(reason.contains("list_bases shows them"), "{reason}");
}

#[test]
fn a_reason_with_no_hint_is_the_engines_words_unchanged() {
    let mut session = session_for("05_eastern_solomons.toml", 1942);
    let reason = refusal_of(
        &mut session,
        "air_group:3",
        json!({"kind": "set_search_arc", "from_deg": 0, "to_deg": 90, "range_nm": "9999"}),
    );
    assert!(reason.contains("at most the aircraft's 200 nm"), "{reason}");
    assert!(!reason.contains('('), "{reason}");
}

// ---- 6. Cargo aboard ----

#[test]
fn cargo_aboard_is_on_the_force_line_when_there_is_any_or_the_mission_is_transport() {
    let scenario = load("02_supply_run.toml", 1);
    let mut checked = 0;
    let mut loaded = 0;
    let title = scenario.name.clone();
    let mut scenario = scenario;
    play(&mut scenario, |s| {
        let view = s.world.side_view(Side::Allied);
        let made = answer_value(&briefing(&view, &title));
        let listed = answer_value(&list_forces(&view, None));
        for line in made["task_forces"].as_array().expect("forces") {
            checked += 1;
            // The same line in the list.
            assert!(listed["task_forces"]
                .as_array()
                .expect("forces")
                .contains(line));
            let handle = line["task_force"]["handle"].as_str().expect("handle");
            let entity = answer_value(&get_entity(&view, handle));
            assert_eq!(&entity["entity"]["summary"], line);
            // The figures are the sums over the force's ships.
            let mut supply = sim_core::fixed::Fixed::ZERO;
            let mut fuel = sim_core::fixed::Fixed::ZERO;
            for ship in entity["entity"]["ships"].as_array().expect("ships") {
                let id = ship["ship"]["handle"].as_str().expect("ship");
                let detail = answer_value(&get_entity(&view, id));
                supply += detail["entity"]["supply_t"]
                    .as_str()
                    .expect("dec")
                    .parse()
                    .expect("a decimal");
                fuel += detail["entity"]["cargo_fuel_t"]
                    .as_str()
                    .expect("dec")
                    .parse()
                    .expect("a decimal");
            }
            let transport = line["mission"] == "transport";
            if supply > sim_core::fixed::Fixed::ZERO || fuel > sim_core::fixed::Fixed::ZERO {
                loaded += 1;
            }
            if transport
                || supply > sim_core::fixed::Fixed::ZERO
                || fuel > sim_core::fixed::Fixed::ZERO
            {
                assert_eq!(line["cargo_supply_t"], supply.to_string());
                assert_eq!(line["cargo_fuel_t"], fuel.to_string());
            } else {
                assert_eq!(line["cargo_supply_t"], Value::Null);
                assert_eq!(line["cargo_fuel_t"], Value::Null);
            }
            valid(&schemas(), "briefing.schema.json", &made, "briefing");
            valid(
                &schemas(),
                "list_forces.schema.json",
                &listed,
                "list_forces",
            );
            valid(&schemas(), "entity.schema.json", &entity, "entity");
        }
    });
    assert!(checked > 0);
    assert!(loaded > 0, "no force ever carried cargo in the scenario");
}

#[test]
fn the_text_of_a_force_line_shows_its_cargo() {
    let mut scenario = load("02_supply_run.toml", 1);
    let title = scenario.name.clone();
    let mut seen = false;
    play(&mut scenario, |s| {
        let view = s.world.side_view(Side::Allied);
        let made = briefing(&view, &title);
        let text = made.to_text();
        for f in &made.task_forces {
            if let (Some(supply), Some(fuel)) = (f.cargo_supply_t, f.cargo_fuel_t) {
                let want = format!(", cargo aboard: {} t supply, {} t fuel", supply.0, fuel.0);
                assert!(text.contains(&want), "{text}");
                seen = true;
            }
        }
        assert_numbers_in_data(&text, &answer_value(&made), "briefing text");
    });
    assert!(seen);
}

// ---- 7. Stocks yesterday ----

#[test]
fn a_bases_stocks_yesterday_add_up_to_what_the_briefings_show() {
    // Over scenarios that land, load and refuel at a base and spend its supply:
    // each day's start is the end the day before, each end is the base's stock
    // now, and a base with no line did not change.
    let mut lines = 0;
    let mut causes_seen = std::collections::BTreeSet::new();
    for name in [
        "01b_refuel.toml",
        "02_supply_run.toml",
        "02b_fuel_run.toml",
        "06d2_base_supply.toml",
        "07_full_slice.toml",
    ] {
        for side in [Side::Allied, Side::Japanese] {
            let mut scenario = load(name, 1942);
            let title = scenario.name.clone();
            let mut previous: BTreeMap<String, (String, String)> = BTreeMap::new();
            play(&mut scenario, |s| {
                let view = s.world.side_view(side);
                let made = briefing(&view, &title);
                let json = answer_value(&made);
                valid(&schemas(), "briefing.schema.json", &json, name);
                let text = made.to_text();
                assert_numbers_in_data(&text, &json, name);
                for base in json["bases"].as_array().expect("bases") {
                    let handle = base["base"]["handle"].as_str().expect("handle").to_string();
                    let supply = base["supply_t"].as_str().expect("dec").to_string();
                    let fuel = base["fuel_t"].as_str().expect("dec").to_string();
                    let stocks = &base["stocks_yesterday"];
                    if let Some((was_supply, was_fuel)) = previous.get(&handle) {
                        // `Fixed` has no float: compare as decimals.
                        let parse = |t: &str| -> sim_core::fixed::Fixed { t.parse().expect("dec") };
                        match &stocks["supply"] {
                            Value::Null => assert_eq!(
                                &supply, was_supply,
                                "{name} {handle}: supply changed with no line"
                            ),
                            line => {
                                lines += 1;
                                assert_eq!(
                                    parse(line["start_t"].as_str().unwrap()),
                                    parse(was_supply),
                                    "{name} {handle}"
                                );
                                assert_eq!(parse(line["end_t"].as_str().unwrap()), parse(&supply));
                                let sum = |keys: &[&str]| {
                                    keys.iter().fold(sim_core::fixed::Fixed::ZERO, |a, k| {
                                        a + parse(line[*k].as_str().unwrap())
                                    })
                                };
                                let net = sum(&["landed_t"])
                                    - sum(&[
                                        "loaded_onto_ships_t",
                                        "sorties_and_upkeep_t",
                                        "repairs_t",
                                        "destroyed_by_raids_t",
                                    ]);
                                assert_eq!(
                                    parse(&supply) - parse(was_supply),
                                    net,
                                    "{name} {handle}"
                                );
                                for key in [
                                    "landed_t",
                                    "loaded_onto_ships_t",
                                    "sorties_and_upkeep_t",
                                    "repairs_t",
                                    "destroyed_by_raids_t",
                                ] {
                                    if parse(line[key].as_str().unwrap())
                                        > sim_core::fixed::Fixed::ZERO
                                    {
                                        causes_seen.insert(format!("supply {key}"));
                                    }
                                }
                                // The totals agree with the older fields.
                                let spent = sum(&["sorties_and_upkeep_t", "repairs_t"]);
                                assert_eq!(
                                    base["spent_yesterday_t"].as_str().map(parse),
                                    Some(spent)
                                );
                                assert_eq!(
                                    base["destroyed_yesterday_t"].as_str().map(parse),
                                    Some(sum(&["destroyed_by_raids_t"]))
                                );
                            }
                        }
                        match &stocks["fuel"] {
                            Value::Null => assert_eq!(
                                &fuel, was_fuel,
                                "{name} {handle}: fuel changed with no line"
                            ),
                            line => {
                                lines += 1;
                                assert_eq!(
                                    parse(line["start_t"].as_str().unwrap()),
                                    parse(was_fuel),
                                    "{name} {handle}"
                                );
                                assert_eq!(parse(line["end_t"].as_str().unwrap()), parse(&fuel));
                                let get = |k: &str| parse(line[k].as_str().unwrap());
                                assert_eq!(
                                    parse(&fuel) - parse(was_fuel),
                                    get("landed_t")
                                        - get("loaded_onto_ships_t")
                                        - get("refuelling_ships_t"),
                                    "{name} {handle}"
                                );
                                for key in ["landed_t", "loaded_onto_ships_t", "refuelling_ships_t"]
                                {
                                    if get(key) > sim_core::fixed::Fixed::ZERO {
                                        causes_seen.insert(format!("fuel {key}"));
                                    }
                                }
                            }
                        }
                    } else {
                        assert_eq!(
                            stocks,
                            &Value::Null,
                            "{name} {handle}: day one has no yesterday"
                        );
                    }
                    previous.insert(handle, (supply, fuel));
                }
            });
        }
    }
    assert!(lines > 0);
    for cause in [
        "supply landed_t",
        "supply loaded_onto_ships_t",
        "supply sorties_and_upkeep_t",
        "fuel refuelling_ships_t",
    ] {
        assert!(
            causes_seen.contains(cause),
            "never seen: {cause}; seen {causes_seen:?}"
        );
    }
}

// ---- 9. measure ----

#[test]
fn a_single_place_is_a_list_of_one_for_air_reach() {
    let mut scenario = load("05_eastern_solomons.toml", 1942);
    scenario.run_to_end().expect("runs");
    let view = scenario.world.side_view(Side::Allied);
    let ask = |to: Value| {
        let q = Question::from_json(
            &json!({"question": "air_reach", "air_group": "air_group:2", "to": to}).to_string(),
        )
        .expect("a question");
        answer_value(&measure(&view, &q))
    };
    let one = ask(json!({"q": 7, "r": -10}));
    let list = ask(json!([{"q": 7, "r": -10}]));
    assert_eq!(one["kind"], "measure");
    assert_eq!(one, list);
    assert_eq!(ask(json!("contact:1")), ask(json!(["contact:1"])));
    // A list with a hex off the map is "not known" as a whole, as before
    // (decision 0049, 2): this is what the first Sonnet player was shown.
    let off = ask(json!([{"q": 7, "r": -15}, {"q": 7, "r": -2500}]));
    assert_eq!(off["kind"], "not_known");
    assert_eq!(off["query"], "measure");
}

#[test]
fn a_malformed_air_reach_is_an_error_that_names_the_shape_and_not_not_known() {
    let mut session = session_for("05_eastern_solomons.toml", 1942);
    let refused = session
        .call(
            Side::Allied,
            "measure",
            &json!({"question": "air_reach", "air_group": "air_group:2", "to": 7}),
        )
        .expect_err("a number is not a place");
    let text = refused.to_json().to_string();
    assert!(text.contains("not a question to measure"), "{text}");
    assert!(text.contains("a list of places"), "{text}");
    assert!(!text.contains("not_known"), "{text}");
    // A place that is shaped like one but means nothing stays "not known".
    let unknown = session
        .call(
            Side::Allied,
            "measure",
            &json!({"question": "air_reach", "air_group": "air_group:2", "to": ["ship:9999"]}),
        )
        .expect("an answer");
    assert_eq!(unknown["kind"], "not_known");
}

// ---- 13. Radius ----

#[test]
fn an_air_group_line_gives_its_types_radius_and_search_range() {
    let scenario = load("05_eastern_solomons.toml", 1942);
    let view = scenario.world.side_view(Side::Allied);
    let made = answer_value(&briefing(&view, "Eastern Solomons"));
    let mut checked = 0;
    for group in made["air_groups"].as_array().expect("groups") {
        let handle = group["air_group"]["handle"].as_str().expect("handle");
        let MeasureAnswer::AirReach(reach) = measure(
            &view,
            &Question::from_json(
                &json!({"question": "air_reach", "air_group": handle, "to": []}).to_string(),
            )
            .expect("a question"),
        ) else {
            panic!("{handle}");
        };
        assert_eq!(
            group["strike_radius_nm"],
            answer_value(&reach.radius_nm),
            "{handle}"
        );
        let kind = view
            .air_groups()
            .find(|g| g.id.to_string() == handle)
            .and_then(|g| view.aircraft_type(g.aircraft_type))
            .expect("the type");
        assert_eq!(group["search_range_nm"], kind.search_range_nm.to_string());
        let entity = answer_value(&get_entity(&view, handle));
        assert_eq!(&entity["entity"]["summary"], group);
        checked += 1;
    }
    assert_eq!(checked, 10);
    let text = briefing(&view, "Eastern Solomons").to_text();
    assert!(
        text.contains("(7,-5): 16 ready, radius 200 miles, search range 200 miles, mission rest"),
        "{text}"
    );
    valid(&schemas(), "briefing.schema.json", &made, "briefing");
}
