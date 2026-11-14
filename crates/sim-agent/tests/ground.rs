//! The fight ashore and a base that changes side in the agent's answers
//! (decision 0090), on scenario 23 read on the frozen figures.
//!
//! The scenario's own hand calculation is in the core's tests; the numbers
//! asserted here are the ones that calculation gives for day 1, read as a
//! side is told them. A side numbers its own land units 1, 2, 3 in the
//! engine's order, so the scenario's land unit 4 (the Allied regiment at Y)
//! is the Allied side's `land_unit:2`, and unit 5 its `land_unit:3`; the
//! Japanese battalion at X (unit 2) is the Japanese side's `land_unit:1` and
//! the garrison at Y (unit 3) its `land_unit:2`.
//!
//! Day 1 at Y: unit 4 is on attack with 2,000 men, all fed, and fires 20
//! shots (10 shots for each 1,000 men). Y falls: base 3 changes side with
//! 0 t supply and 40 t fuel kept (half its 80 t fuel is lost), and its air
//! group 1 and the ship in its port are lost with it, the destroyer patrol
//! losing its home port. At Z the platoon takes base 5 with no fight, with
//! 30 t supply and 15 t fuel kept (half of 60 t and 30 t); air groups 2 and
//! 3 were at it, and group 3 is flown to X by the Japanese order of day 1
//! given here.

mod support;

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::state::Side;
use support::check::valid;
use support::{load, schemas};

fn session() -> Session {
    let scenario = load("23_fight_ashore.toml", 1942);
    Session::new(
        scenario.world,
        "Scenario 23",
        scenario.days,
        Seat::Player,
        Seat::Player,
    )
}

fn orders(session: &mut Session, side: Side, list: &[Value]) -> Vec<Value> {
    let answer = session
        .call(side, "issue_orders", &json!({ "orders": list }))
        .expect("issue_orders is answered");
    valid(&schemas(), "issue_orders.schema.json", &answer, "ground");
    answer["results"].as_array().expect("results").clone()
}

fn end_day(session: &mut Session) {
    for side in [Side::Allied, Side::Japanese] {
        session
            .call(side, "end_day", &json!({}))
            .expect("end_day is answered");
    }
}

/// The text and the data of a side's briefing, both checked against the
/// schema.
fn briefing(session: &mut Session, side: Side) -> (String, Value) {
    let text = session
        .call(side, "get_briefing", &json!({ "text": true }))
        .expect("the briefing");
    valid(
        &schemas(),
        "briefing_text.schema.json",
        &text,
        "ground text",
    );
    let data = session
        .call(side, "get_briefing", &json!({ "text": false }))
        .expect("the briefing");
    valid(&schemas(), "briefing.schema.json", &data, "ground briefing");
    (text["text"].as_str().expect("text").to_string(), data)
}

fn answer(session: &mut Session, side: Side, tool: &str, args: Value, file: &str) -> Value {
    let a = session.call(side, tool, &args).expect("answered");
    valid(&schemas(), file, &a, tool);
    a
}

/// Day 1 played as the scenario's script has it: group 3 is flown from Z to
/// X by the Japanese.
fn after_day_one() -> Session {
    let mut session = session();
    let results = orders(
        &mut session,
        Side::Japanese,
        &[json!({ "unit": "air_group:3", "order": { "kind": "rebase", "base": 2 } })],
    );
    assert_eq!(results[0]["status"], "queued", "{results:?}");
    end_day(&mut session);
    session
}

fn by_handle<'a>(list: &'a Value, key: &str, handle: &str) -> &'a Value {
    list.as_array()
        .expect("a list")
        .iter()
        .find(|x| x[key]["handle"] == handle)
        .unwrap_or_else(|| panic!("{handle} in {list}"))
}

fn unit_line<'a>(data: &'a Value, handle: &str) -> &'a Value {
    by_handle(&data["land_units"]["units"], "land_unit", handle)
}

#[test]
fn the_side_is_told_its_own_fire_in_the_fight_and_never_the_hits() {
    let mut session = after_day_one();
    let (text, data) = briefing(&mut session, Side::Allied);

    let fire = by_handle(
        &data["land_units"]["fire_yesterday"],
        "land_unit",
        "land_unit:2",
    );
    assert_eq!(fire["place"]["q"], 6);
    assert_eq!(fire["place"]["r"], 0);
    assert_eq!(fire["firing_men"], 2000);
    assert_eq!(fire["shots"], "20");
    let keys: Vec<&String> = fire.as_object().expect("object").keys().collect();
    assert_eq!(
        keys,
        ["event", "firing_men", "land_unit", "place", "shots"],
        "a fire line has no hits and no chance"
    );
    assert!(
        text.contains("Fire yesterday: land_unit:2") && text.contains("with 2000 men, 20 shots"),
        "{text}"
    );
    let land = data["land_units"].to_string();
    assert!(!land.contains("hits") && !land.contains("chance"), "{land}");
    assert!(
        !text.contains(" hit") && !text.contains("chance"),
        "no hit count in the text"
    );

    // The fight at X and at Y is told as a place, never as an attacker.
    let fights = data["land_units"]["fights_yesterday"]
        .as_array()
        .expect("fights");
    let hexes: Vec<(i64, i64)> = fights
        .iter()
        .map(|f| {
            (
                f["place"]["q"].as_i64().unwrap(),
                f["place"]["r"].as_i64().unwrap(),
            )
        })
        .collect();
    assert_eq!(hexes, [(0, 0), (6, 0)]);
    assert!(text.contains("Fight ashore yesterday at (6,0)"), "{text}");
    assert!(!text.contains("attacker"), "{text}");

    // Its own men lost in the fight at X: 20 of the 1,000.
    let lost = data["land_units"]["lost_yesterday"]
        .as_array()
        .expect("lost")
        .iter()
        .find(|l| l["cause"] == "fight")
        .expect("a loss in the fight");
    assert_eq!(lost["land_unit"]["handle"], "land_unit:1");
    assert_eq!(lost["men"], 20);
    assert_eq!(lost["ashore"], 980);
    assert!(
        text.contains("Men lost yesterday: land_unit:1")
            && text.contains("lost 20 men in the fight at (0,0)"),
        "{text}"
    );

    // Fatigue: the unit on attack that fired gained 20.
    let tired = by_handle(
        &data["land_units"]["fatigue_yesterday"],
        "land_unit",
        "land_unit:2",
    );
    assert_eq!(tired["change"], "gained");
    assert_eq!(tired["fatigue"], "20");
    assert!(
        text.contains("Fatigue yesterday: land_unit:2")
            && text.contains("attacked and tired, fatigue now 20"),
        "{text}"
    );
}

#[test]
fn neither_side_is_told_an_enemy_land_unit_or_the_enemys_men() {
    let mut session = after_day_one();
    for side in [Side::Allied, Side::Japanese] {
        let (text, data) = briefing(&mut session, side);
        let both = format!("{text}{data}");
        let enemy_words: [&str; 3] = match side {
            Side::Allied => ["battalion", "garrison", "Japanese regiment"],
            Side::Japanese => ["Allied regiment", "platoon", "Allied battalion"],
        };
        for word in enemy_words {
            assert!(!both.contains(word), "{side:?} was told of {word}");
        }
    }
    let (text, _) = briefing(&mut session, Side::Allied);
    for men in ["600 men", "300 men", "500 men"] {
        assert!(!text.contains(men), "the Allies were told {men}");
    }
    // The 165 men of the Japanese unit that surrendered are told, as the
    // surrender: the hex and the men, never which unit (decision 0104, 8).
    assert!(
        text.contains("An enemy unit surrendered at (6,0)")
            && text.contains("165 men taken prisoner"),
        "{text}"
    );
    assert_eq!(text.matches("165 men").count(), 1, "{text}");
    let (text, _) = briefing(&mut session, Side::Japanese);
    for men in ["2000 men", "980 men", "48 men"] {
        assert!(!text.contains(men), "the Japanese were told {men}");
    }
}

#[test]
fn the_side_that_lost_a_base_is_told_it_without_the_stocks_and_what_went_with_it() {
    let mut session = after_day_one();
    let (text, data) = briefing(&mut session, Side::Japanese);
    let changes = &data["base_changes"];

    let lost = changes["lost"].as_array().expect("bases lost");
    let handles: Vec<&str> = lost
        .iter()
        .map(|l| l["base"]["handle"].as_str().unwrap())
        .collect();
    assert_eq!(handles, ["base:3", "base:5"]);
    for l in lost {
        let keys: Vec<&String> = l.as_object().unwrap().keys().collect();
        assert_eq!(keys, ["base", "event", "place"], "no stocks: {l}");
    }
    assert!(changes["taken"].as_array().unwrap().is_empty());
    assert!(
        text.contains("Base lost yesterday: base:3")
            && text.contains("Base lost yesterday: base:5")
            && text.contains("is the enemy's now"),
        "{text}"
    );
    // No ton of the stock is in the lines of the bases lost.
    for line in text.lines().filter(|l| l.starts_with("- Base lost")) {
        assert!(!line.contains(" t "), "{line}");
    }

    // Air groups 1 and 2 were lost with their bases; group 3 was flown out
    // the day before and is not.
    let groups = changes["air_groups_lost"].as_array().expect("groups");
    let lost: Vec<(&str, &str, u64)> = groups
        .iter()
        .map(|g| {
            (
                g["air_group"]["handle"].as_str().unwrap(),
                g["base"]["handle"].as_str().unwrap(),
                g["aircraft"].as_u64().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        lost,
        [("air_group:1", "base:3", 11), ("air_group:2", "base:5", 6)]
    );
    assert!(
        text.contains("Air group lost with its base: air_group:1")
            && text.contains("with 11 aircraft, lost with base:3"),
        "{text}"
    );

    // The ship in port, and the force whose home port was base 3.
    assert_eq!(changes["ships_lost"][0]["ship"]["handle"], "ship:1");
    assert_eq!(changes["ships_lost"][0]["base"]["handle"], "base:3");
    assert_eq!(
        changes["home_ports_lost"][0]["task_force"]["handle"],
        "task_force:1"
    );
    assert!(text.contains("Ship lost in port: ship:1"), "{text}");
    assert!(text.contains("Home port lost: task_force:1"), "{text}");

    // A group lost with its base has no place and no location.
    let g1 = by_handle(&data["air_groups"], "air_group", "air_group:1");
    assert_eq!(g1["lost_with_base"], true);
    assert!(g1["location"].is_null() && g1["place"].is_null(), "{g1}");
    let g3 = by_handle(&data["air_groups"], "air_group", "air_group:3");
    assert!(g3.get("lost_with_base").is_none());
    assert_eq!(g3["location"]["handle"], "base:2");
    assert!(
        text.contains("air_group:1 \"Placeholder fighter squadron at Y\" (Placeholder fighter): lost with its base"),
        "{text}"
    );
    let entity = answer(
        &mut session,
        Side::Japanese,
        "get_entity",
        json!({ "handle": "air_group:1" }),
        "entity.schema.json",
    );
    assert_eq!(entity["entity"]["summary"]["lost_with_base"], true);
    let listed = answer(
        &mut session,
        Side::Japanese,
        "list_forces",
        json!({}),
        "list_forces.schema.json",
    );
    assert!(listed.to_string().contains("land_unit:1"));
    // It cannot be named as a place: it has none.
    let measured = answer(
        &mut session,
        Side::Japanese,
        "measure",
        json!({ "question": "air_distance", "from": "air_group:1", "to": "base:2" }),
        "measure.schema.json",
    );
    assert_eq!(measured["kind"], "not_known", "{measured}");
}

#[test]
fn the_side_that_took_a_base_is_told_it_whole_and_nothing_of_what_the_other_lost_with_it() {
    let mut session = after_day_one();
    let (text, data) = briefing(&mut session, Side::Allied);
    let changes = &data["base_changes"];

    let taken = changes["taken"].as_array().expect("bases taken");
    let kept: Vec<(&str, &str, &str)> = taken
        .iter()
        .map(|t| {
            (
                t["base"]["handle"].as_str().unwrap(),
                t["supply_t"].as_str().unwrap(),
                t["fuel_t"].as_str().unwrap(),
            )
        })
        .collect();
    assert_eq!(kept, [("base:3", "0", "40"), ("base:5", "30", "15")]);
    assert_eq!(taken[0]["supply_lost_t"], "0");
    assert_eq!(taken[0]["fuel_lost_t"], "40");
    assert_eq!(taken[1]["supply_lost_t"], "30");
    assert_eq!(taken[1]["fuel_lost_t"], "15");
    assert!(
        text.contains("Base taken yesterday: base:3")
            && text.contains("with 0 t supply and 40 t fuel kept")
            && text.contains("Base taken yesterday: base:5")
            && text.contains("with 30 t supply and 15 t fuel kept"),
        "{text}"
    );

    // Nothing of the groups, the ship or the home port of the enemy.
    assert!(changes["lost"].as_array().unwrap().is_empty());
    for part in [
        "air_groups_lost",
        "ships_lost",
        "home_ports_lost",
        "strike_missions_cleared",
    ] {
        assert!(changes[part].as_array().unwrap().is_empty(), "{part}");
    }
    assert!(data["air_groups"].as_array().unwrap().is_empty());
    for word in ["lost with its base", "Ship lost in port", "Home port lost"] {
        assert!(!text.contains(word), "{word}: {text}");
    }

    // The bases are the side's own now: in its own list and its objectives.
    let bases = answer(
        &mut session,
        Side::Allied,
        "list_bases",
        json!({}),
        "list_bases.schema.json",
    );
    let own: Vec<&str> = bases["bases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|b| b["base"]["handle"].as_str().unwrap())
        .collect();
    assert!(
        own.contains(&"base:3") && own.contains(&"base:5"),
        "{own:?}"
    );
    let enemy: Vec<&str> = bases["enemy_bases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|b| b["base"]["handle"].as_str().unwrap())
        .collect();
    assert!(
        !enemy.contains(&"base:3") && !enemy.contains(&"base:5"),
        "{enemy:?}"
    );

    // And the Japanese side's list shows the same bases as the enemy's.
    let bases = answer(
        &mut session,
        Side::Japanese,
        "list_bases",
        json!({}),
        "list_bases.schema.json",
    );
    let enemy: Vec<(&str, &str)> = bases["enemy_bases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|b| {
            (
                b["base"]["handle"].as_str().unwrap(),
                b["held_by"].as_str().unwrap(),
            )
        })
        .collect();
    assert!(enemy.contains(&("base:3", "allied")), "{enemy:?}");
    assert!(enemy.contains(&("base:5", "allied")), "{enemy:?}");
}

#[test]
fn a_base_held_objective_reads_the_base_as_it_is_now_and_men_lost_is_not_observable() {
    let mut session = session();
    let (text, data) = briefing(&mut session, Side::Allied);
    let objectives = data["objectives"]["objectives"].as_array().unwrap();
    let held = &objectives[0];
    assert_eq!(held["kind"], "base_held");
    assert_eq!(held["base"]["handle"], "base:3");
    assert_eq!(held["points"], "10");
    assert_eq!(held["known"]["basis"], "exact");
    assert_eq!(held["known"]["count"], "0");
    let men = objectives
        .iter()
        .find(|o| o["kind"] == "men_lost")
        .expect("men lost");
    assert_eq!(men["points"], "1");
    assert_eq!(men["known"]["basis"], "not_observable");
    assert!(text.contains("the base is not yours now"), "{text}");
    assert!(
        text.contains("Objective: enemy men lost, by any cause, points for each hundred men: 1"),
        "{text}"
    );

    // The base fell on day 1: it is the side's own, from the present view.
    end_day(&mut session);
    let (text, data) = briefing(&mut session, Side::Allied);
    let held = &data["objectives"]["objectives"][0];
    assert_eq!(held["known"]["count"], "1");
    assert_eq!(held["known"]["points"], "10");
    assert!(
        text.contains("the base is yours now, worth 10 points if it stays so"),
        "{text}"
    );
    let held_by_enemy = {
        let (_, data) = briefing(&mut session, Side::Japanese);
        data["objectives"]["objectives"][0].clone()
    };
    assert_eq!(held_by_enemy["base"]["handle"], "base:1");
    assert_eq!(held_by_enemy["known"]["count"], "0");
}

#[test]
fn a_unit_shows_its_posture_its_fatigue_and_its_fed_share() {
    let mut session = session();
    let (_, before) = briefing(&mut session, Side::Allied);
    let u2 = unit_line(&before, "land_unit:2");
    assert_eq!(u2["posture"], "attack");
    assert_eq!(u2["fatigue"], "0");
    assert_eq!(u2["fed_share"], "1");
    assert_eq!(unit_line(&before, "land_unit:1")["posture"], "defend");

    end_day(&mut session);
    let (text, data) = briefing(&mut session, Side::Allied);
    // The platoon at Z had no base to feed it the first night: fed share 0.
    let z = unit_line(&data, "land_unit:3");
    assert_eq!(z["fed_share"], "0");
    assert_eq!(z["fed_last_night"], "unfed");
    assert_eq!(z["posture"], "attack");
    // Unit 4 of the scenario, fed in full, fired 20 shots and is tired.
    let y = unit_line(&data, "land_unit:2");
    assert_eq!(y["fed_share"], "1");
    assert_eq!(y["fatigue"], "20");
    assert!(
        text.contains("posture attack, fatigue 20, fed share 1")
            && text.contains("posture attack, fatigue 0, fed share 0"),
        "{text}"
    );
    // The same line from get_entity and list_forces.
    let entity = answer(
        &mut session,
        Side::Allied,
        "get_entity",
        json!({ "handle": "land_unit:3" }),
        "entity.schema.json",
    );
    assert_eq!(entity["entity"]["summary"]["fed_share"], "0");
    let listed = answer(
        &mut session,
        Side::Allied,
        "list_forces",
        json!({}),
        "list_forces.schema.json",
    );
    let l3 = by_handle(&listed["land_units"], "land_unit", "land_unit:3");
    assert_eq!(l3["fed_share"], "0");
    assert_eq!(l3["posture"], "attack");
}

#[test]
fn a_unit_too_tired_goes_back_to_defend_and_the_side_is_told() {
    let mut session = session();
    for _ in 0..4 {
        end_day(&mut session);
    }
    // After day 4 the Japanese battalion at X, on attack from the start, is
    // at fatigue 80 and back on defend.
    let (text, data) = briefing(&mut session, Side::Japanese);
    let unit = unit_line(&data, "land_unit:1");
    assert_eq!(unit["posture"], "defend");
    assert_eq!(unit["fatigue"], "80");
    let back = by_handle(
        &data["land_units"]["back_to_defend_yesterday"],
        "land_unit",
        "land_unit:1",
    );
    assert_eq!(back["fatigue"], "80");
    assert!(
        text.contains("Back to defend yesterday: land_unit:1")
            && text.contains("too tired to attack at fatigue 80"),
        "{text}"
    );
    // The garrison at Y was destroyed on day 3.
    let garrison = unit_line(&data, "land_unit:2");
    assert_eq!(garrison["destroyed"], true);
}

#[test]
fn a_posture_order_is_taken_for_an_own_unit_and_refused_for_the_rest() {
    let mut session = session();
    let posture = |unit: &str, posture: &str| json!({ "unit": unit, "order": { "kind": "set_posture", "posture": posture } });
    let results = orders(
        &mut session,
        Side::Allied,
        &[
            posture("land_unit:1", "attack"),
            posture("land_unit:2", "defend"),
            // The Allied side has three units: a fourth is no unit of its
            // own, and the enemy's units cannot be named.
            posture("land_unit:4", "attack"),
            posture("land_unit:90004", "attack"),
            json!({ "unit": "land_unit:1", "order": { "kind": "set_posture", "posture": "charge" } }),
            json!({ "unit": "land_unit:1", "order": { "kind": "set_route", "waypoints": [[1, 0]] } }),
        ],
    );
    assert_eq!(results[0]["status"], "queued", "{results:?}");
    assert_eq!(results[1]["status"], "queued", "{results:?}");
    for i in [2, 3] {
        assert_eq!(results[i]["status"], "refused");
        assert_eq!(results[i]["reason"], "not known", "{results:?}");
    }
    for i in [4, 5] {
        assert_eq!(results[i]["status"], "refused");
        assert!(
            results[i]["reason"]
                .as_str()
                .unwrap()
                .starts_with("malformed order"),
            "{results:?}"
        );
    }

    // The order stands: unit 1 is on attack from the day, and its standing
    // order is shown by its number; unit 2 is back on defend.
    end_day(&mut session);
    let (text, data) = briefing(&mut session, Side::Allied);
    let u1 = unit_line(&data, "land_unit:1");
    assert_eq!(u1["posture"], "attack");
    assert_eq!(u1["posture_order"], 1);
    assert_eq!(u1["fatigue"], "20", "it fired on attack");
    assert!(text.contains("(posture set by your order 1)"), "{text}");
    let u2 = unit_line(&data, "land_unit:2");
    assert_eq!(u2["posture"], "defend");
    assert_eq!(u2["fatigue"], "0", "it did not fire");
    assert_eq!(data["orders"]["accepted"], 2);

    // The order of a unit that is not the side's own is refused alike on
    // every day, whatever the enemy holds.
    let results = orders(
        &mut session,
        Side::Japanese,
        &[posture("land_unit:3", "attack")],
    );
    assert_eq!(results[0]["reason"], "not known");
}

#[test]
fn a_destroyed_unit_takes_no_posture_order() {
    let mut session = session();
    for _ in 0..3 {
        end_day(&mut session);
    }
    let results = orders(
        &mut session,
        Side::Japanese,
        &[
            // The garrison at Y, destroyed on day 3.
            json!({ "unit": "land_unit:2", "order": { "kind": "set_posture", "posture": "attack" } }),
            json!({ "unit": "land_unit:1", "order": { "kind": "set_posture", "posture": "defend" } }),
        ],
    );
    assert_eq!(results[0]["status"], "refused", "{results:?}");
    let reason = results[0]["reason"].as_str().unwrap();
    assert!(
        reason.starts_with("the land unit is destroyed (a destroyed unit has no man left"),
        "{reason}"
    );
    assert_eq!(results[1]["status"], "queued", "{results:?}");
}

#[test]
fn every_answer_of_the_whole_game_passes_its_schema_and_the_result_scores_the_new_objectives() {
    let mut session = session();
    let mut last = Value::Null;
    for _ in 0..6 {
        for side in [Side::Allied, Side::Japanese] {
            briefing(&mut session, side);
            for tool in ["list_forces", "list_bases", "list_contacts"] {
                let file = format!("{tool}.schema.json");
                answer(&mut session, side, tool, json!({}), &file);
            }
            for handle in ["land_unit:1", "land_unit:2", "base:3", "base:5"] {
                let a = session
                    .call(side, "get_entity", &json!({ "handle": handle }))
                    .expect("answered");
                valid(&schemas(), "entity.schema.json", &a, handle);
            }
        }
        for side in [Side::Allied, Side::Japanese] {
            last = session
                .call(side, "end_day", &json!({}))
                .expect("end_day is answered");
            valid(&schemas(), "end_day.schema.json", &last, "end_day");
        }
    }
    assert_eq!(last["game_over"], true);
    let allied = &last["result"]["allied"]["objectives"];
    let kinds: Vec<&str> = allied
        .as_array()
        .unwrap()
        .iter()
        .map(|o| o["kind"].as_str().unwrap())
        .collect();
    assert_eq!(kinds, ["base_held", "base_held", "ships_sunk", "men_lost"]);
    assert_eq!(allied[0]["count"], "1");
    assert_eq!(allied[0]["points"], "10");
    let (text, _) = briefing(&mut session, Side::Allied);
    assert!(text.contains("base held at base:3"), "{text}");
    assert!(text.contains("men lost: count"), "{text}");
}

#[test]
fn the_fight_ashore_every_day_for_both_sides() {
    // Every answer on every day, and every event the side may see explained:
    // valid against its schema, every number of the text in the data, and no
    // enemy unit named except as a contact (decision 0048).
    let schemas = schemas();
    let mut scenario = load("23_fight_ashore.toml", 1942);
    let name = scenario.name.clone();
    support::play(&mut scenario, |s| {
        for side in [Side::Allied, Side::Japanese] {
            let view = s.world.side_view(side);
            support::check::check_view(&view, &name, &schemas, true);
        }
    });
}
