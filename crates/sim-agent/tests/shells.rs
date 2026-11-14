//! Shells and bombs on a base, and what a side knows of the enemy ashore, in
//! the agent's answers (decision 0092), on scenario 24 read on the frozen
//! figures.
//!
//! The hand calculation is in `tests/scenario_24_shells_and_estimates.rs`. The
//! parts of it asserted here are the ones that do not depend on a roll: on the
//! night of day 1 the Japanese cruiser (8 guns at 3 rounds a minute for 30
//! minutes) fires 720 shells of 800 aboard, and the destroyer (4 guns at 6 a
//! minute) 720 of its 600 rounds, which is 600; the cruiser's shells are weighed
//! at 20 over 20, the destroyer's at 5 over 20, so the force fires 1320 shells
//! of 870 weighed shots. A side's numbers are its own: the Japanese side is
//! told `task_force:1`, `ship:1` and `ship:2`, and the Allied side numbers its
//! land units `land_unit:1` (the regiment at X) and `land_unit:2` (the division
//! at Y). A hit that lands on men removes `men_lost_per_base_hit` men, 10 on
//! the placeholder figures, as many as are there.
//!
//! A ground report is the true count times an error of 0.5 to 1.5. After day
//! 1's fight 2940 Allied men and 420 Japanese stand at Y; after day 2's, 2940
//! and 40.

mod support;

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::state::Side;
use support::check::valid;
use support::{load, schemas};

fn session_of(name: &str) -> Session {
    let scenario = load(name, 1942);
    Session::new(
        scenario.world,
        name,
        scenario.days,
        Seat::Player,
        Seat::Player,
    )
}

fn session() -> Session {
    session_of("24_shells_and_estimates.toml")
}

fn end_day(session: &mut Session) {
    for side in [Side::Allied, Side::Japanese] {
        session
            .call(side, "end_day", &json!({}))
            .expect("end_day is answered");
    }
}

/// The Japanese orders of the scenario's script, which a session does not
/// read: day 1, steam to X at cruise speed; day 3, steam back out to (2,0).
fn script(session: &mut Session, day: u32) {
    let list = match day {
        1 => vec![
            json!({ "unit": "task_force:1", "order": { "kind": "set_route", "waypoints": [[0, 0]] } }),
            json!({ "unit": "task_force:1", "order": { "kind": "set_speed", "speed": "cruise" } }),
        ],
        3 => vec![
            json!({ "unit": "task_force:1", "order": { "kind": "set_route", "waypoints": [[2, 0]] } }),
        ],
        _ => Vec::new(),
    };
    if list.is_empty() {
        return;
    }
    let answer = session
        .call(Side::Japanese, "issue_orders", &json!({ "orders": list }))
        .expect("issue_orders is answered");
    valid(&schemas(), "issue_orders.schema.json", &answer, "script");
    for result in answer["results"].as_array().expect("results") {
        assert_eq!(result["status"], "queued", "{answer}");
    }
}

fn after_days(days: u32) -> Session {
    let mut session = session();
    for day in 1..=days {
        script(&mut session, day);
        end_day(&mut session);
    }
    session
}

fn briefing(session: &mut Session, side: Side) -> (String, Value) {
    let text = session
        .call(side, "get_briefing", &json!({ "text": true }))
        .expect("the briefing");
    valid(&schemas(), "briefing_text.schema.json", &text, "text");
    let data = session
        .call(side, "get_briefing", &json!({ "text": false }))
        .expect("the briefing");
    valid(&schemas(), "briefing.schema.json", &data, "briefing");
    (text["text"].as_str().expect("text").to_string(), data)
}

fn list(session: &mut Session, side: Side, tool: &str, args: Value) -> Value {
    let a = session.call(side, tool, &args).expect("answered");
    valid(&schemas(), &format!("{tool}.schema.json"), &a, tool);
    a
}

fn n(value: &Value) -> u64 {
    value.as_u64().expect("a whole number")
}

fn dec(value: &Value) -> u64 {
    value
        .as_str()
        .expect("a decimal string")
        .parse()
        .expect("whole")
}

fn estimate_at(estimates: &Value, q: i64, r: i64) -> &Value {
    estimates
        .as_array()
        .expect("a list")
        .iter()
        .find(|e| e["place"]["q"] == q && e["place"]["r"] == r)
        .unwrap_or_else(|| panic!("an estimate at ({q},{r}) in {estimates}"))
}

#[test]
fn the_side_that_bombards_is_told_its_shells_and_its_claim_and_never_a_hit() {
    let mut session = after_days(1);
    let (text, data) = briefing(&mut session, Side::Japanese);

    let made = data["action"]["bombardments"].as_array().expect("a list");
    assert_eq!(made.len(), 1, "{made:?}");
    let b = &made[0];
    assert_eq!(b["force"]["handle"], "task_force:1");
    assert_eq!(b["base"]["handle"], "base:1");
    assert_eq!(b["place"]["q"], 0);
    assert_eq!(b["place"]["r"], 0);
    assert_eq!(b["shells"], "1320");
    assert_eq!(b["weighed_shots"], "870");
    let keys: Vec<&String> = b.as_object().expect("object").keys().collect();
    assert_eq!(
        keys,
        ["base", "event", "force", "place", "shells", "weighed_shots"],
        "a bombardment line has no hit and no chance"
    );

    // Each ship: its own shells and the rounds it has left.
    let fire = data["action"]["shell_fire"].as_array().expect("a list");
    assert_eq!(fire.len(), 2, "{fire:?}");
    assert_eq!(fire[0]["ship"]["handle"], "ship:1");
    assert_eq!(fire[0]["shells"], "720");
    assert_eq!(fire[0]["weighed_shots"], "720");
    assert_eq!(fire[0]["rounds_left"], "80");
    assert_eq!(fire[1]["ship"]["handle"], "ship:2");
    assert_eq!(fire[1]["shells"], "600");
    assert_eq!(fire[1]["weighed_shots"], "150");
    assert_eq!(fire[1]["rounds_left"], "0");
    for line in fire {
        let keys: Vec<&String> = line.as_object().expect("object").keys().collect();
        assert!(
            !keys.iter().any(|k| k.starts_with("hit") || *k == "chance"),
            "a ship's fire line has no hit: {line}"
        );
    }

    // The claim reads as one from the sea, of aircraft on the ground.
    let claims = data["losses"]["claims"].as_array().expect("a list");
    assert_eq!(claims.len(), 1, "{claims:?}");
    let claim = &claims[0];
    assert_eq!(claim["from_bombardment"], true);
    assert_eq!(claim["from_air"], false);
    assert_eq!(claim["claimed_sunk"], 0);
    assert_eq!(claim["claimed_shot_down"], 0);
    assert_eq!(claim["place"]["q"], 0);
    let claimed = n(&claim["claimed_on_ground"]);
    assert!(
        text.contains(&format!(
            "Claim from the sea at (0,0): {claimed} aircraft destroyed on the ground"
        )),
        "{text}"
    );
    assert!(
        text.contains("Shore bombardment by task_force:1 ")
            && text.contains("1320 shells, 870 weighed shots"),
        "{text}"
    );
    assert!(
        text.contains("fired 720 shells (720 weighed shots) at base:1"),
        "{text}"
    );

    // The claim is never below what the Allied side lost on the ground.
    let (_, allied) = briefing(&mut session, Side::Allied);
    let lost: u64 = allied["losses"]["aircraft"]
        .as_array()
        .expect("a list")
        .iter()
        .map(|a| n(&a["destroyed_on_ground"]))
        .sum();
    assert!(claimed >= lost, "claimed {claimed}, truly destroyed {lost}");
}

#[test]
fn the_side_that_is_shelled_is_told_the_fire_and_where_it_landed_and_never_the_ships() {
    let mut session = after_days(1);
    let (text, data) = briefing(&mut session, Side::Allied);

    let against = data["action"]["bombardments_against"]
        .as_array()
        .expect("a list");
    assert_eq!(against.len(), 1, "{against:?}");
    let b = &against[0];
    assert_eq!(b["base"]["handle"], "base:1");
    assert_eq!(b["place"]["q"], 0);
    assert_eq!(b["shells"], "1320");
    assert_eq!(b["weighed_shots"], "870");
    let (hits, air, supply, men) = (
        n(&b["hits"]),
        n(&b["hits_on_aircraft"]),
        n(&b["hits_on_supply"]),
        n(&b["hits_on_men"]),
    );
    assert!(air + supply + men <= hits, "{b}");
    let keys: Vec<&String> = b.as_object().expect("object").keys().collect();
    assert!(
        !keys
            .iter()
            .any(|k| *k == "force" || *k == "ship" || *k == "chance"),
        "the force that fired is not named: {b}"
    );
    assert!(
        text.contains("Bombarded from the sea: base:1") && text.contains("the ships are not known"),
        "{text}"
    );
    // The enemy's force and ships are named nowhere: this side has none.
    assert!(
        !text.contains("task_force:") && !text.contains("ship:"),
        "{text}"
    );
    assert!(
        !data.to_string().contains("task_force:") && !data.to_string().contains("ship:"),
        "{data}"
    );

    // What it lost: the supply and aircraft by the events it had, the men as
    // a loss to fire, 10 a hit that landed on men.
    let men_lost = data["land_units"]["lost_yesterday"]
        .as_array()
        .expect("a list")
        .iter()
        .find(|l| l["cause"] == "fire")
        .unwrap_or_else(|| panic!("men lost to fire in {data}"));
    assert_eq!(men_lost["land_unit"]["handle"], "land_unit:1");
    assert_eq!(n(&men_lost["men"]), 10 * men);
    assert_eq!(n(&men_lost["ashore"]), 1000 - 10 * men);
    assert!(
        text.contains("lost 90 men to bombs or shells")
            || text.contains("to bombs or shells at (0,0)"),
        "{text}"
    );
    let base = data["bases"]
        .as_array()
        .expect("a list")
        .iter()
        .find(|b| b["base"]["handle"] == "base:1")
        .expect("base 1");
    if supply > 0 {
        assert!(dec(&base["destroyed_yesterday_t"]) > 0, "{base}");
    }

    // The side that shelled is told no hit and no loss of the Allied side's.
    let (_, japanese) = briefing(&mut session, Side::Japanese);
    assert!(japanese["land_units"]["lost_yesterday"]
        .as_array()
        .expect("a list")
        .iter()
        .all(|l| l["cause"] != "fire"));
    assert!(japanese["action"]["bombardments_against"].is_null());
}

#[test]
fn an_opening_estimate_is_dated_day_zero_and_a_report_replaces_it() {
    // At the start of day 1 the Japanese side holds its opening estimate and
    // the Allied side none.
    let mut session = session();
    let (text, data) = briefing(&mut session, Side::Japanese);
    let estimates = &data["enemy_ashore"]["estimates"];
    assert_eq!(estimates.as_array().expect("a list").len(), 1);
    let e = estimate_at(estimates, 8, 0);
    assert_eq!(e["men"], 300);
    assert_eq!(e["day"], 0);
    assert_eq!(e["age_days"], 0);
    assert!(
        text.contains("ENEMY MEN ASHORE (ESTIMATES)")
            && text.contains("about 300 enemy men ashore at (8,0)")
            && text.contains(", opening estimate, day 0, 0 days old")
            && text.contains("can be wrong by up to half either way"),
        "{text}"
    );
    let (allied_text, allied) = briefing(&mut session, Side::Allied);
    assert!(allied["enemy_ashore"].is_null());
    assert!(!allied_text.contains("ENEMY MEN ASHORE"), "{allied_text}");

    // After day 1 both sides have a report of the other's men at Y, within
    // half either way of the truth after the day's fight.
    let mut session = after_days(1);
    let (text, data) = briefing(&mut session, Side::Japanese);
    let e = estimate_at(&data["enemy_ashore"]["estimates"], 8, 0).clone();
    assert_eq!(e["day"], 1);
    assert_eq!(e["age_days"], 0);
    let men = n(&e["men"]);
    assert!((1470..=4410).contains(&men), "a report of 2940 men: {men}");
    assert!(text.contains("reported day 1, 0 days old"), "{text}");
    let (_, allied) = briefing(&mut session, Side::Allied);
    let men = n(&estimate_at(&allied["enemy_ashore"]["estimates"], 8, 0)["men"]);
    assert!((210..=630).contains(&men), "a report of 420 men: {men}");

    // An estimate is never aged or dropped: after day 3 the Japanese unit is
    // gone and no report is made, so the day-2 estimate stands, a day old.
    let mut session = after_days(3);
    let (text, data) = briefing(&mut session, Side::Allied);
    let e = estimate_at(&data["enemy_ashore"]["estimates"], 8, 0);
    assert_eq!(e["day"], 2);
    assert_eq!(e["age_days"], 1);
    let men = n(&e["men"]);
    assert!((20..=60).contains(&men), "a report of 40 men: {men}");
    assert!(text.contains("reported day 2, 1 day old"), "{text}");
}

#[test]
fn a_ground_report_that_finds_a_base_puts_it_on_the_map_and_the_briefing_says_so() {
    let mut session = session();
    let before = list(&mut session, Side::Allied, "list_bases", json!({}));
    assert!(
        before["enemy_bases"]
            .as_array()
            .expect("a list")
            .iter()
            .all(|b| b["base"]["handle"] != "base:3"),
        "base 3 is not on the Allied map yet: {before}"
    );
    script(&mut session, 1);
    end_day(&mut session);
    let (text, data) = briefing(&mut session, Side::Allied);
    let found = data["enemy_ashore"]["bases_found_yesterday"]
        .as_array()
        .expect("a list");
    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!(found[0]["base"]["handle"], "base:3");
    assert_eq!(found[0]["place"]["q"], 8);
    assert!(
        text.contains("Enemy base found yesterday: base:3") && text.contains("now on your map"),
        "{text}"
    );
    let after = list(&mut session, Side::Allied, "list_bases", json!({}));
    assert!(
        after["enemy_bases"]
            .as_array()
            .expect("a list")
            .iter()
            .any(|b| b["base"]["handle"] == "base:3"),
        "{after}"
    );
    // The Japanese side found nothing: its bases were all known.
    let (_, japanese) = briefing(&mut session, Side::Japanese);
    assert!(japanese["enemy_ashore"]["bases_found_yesterday"].is_null());
}

#[test]
fn list_contacts_gives_the_estimates_as_a_separate_list() {
    let mut session = session();
    let a = list(&mut session, Side::Japanese, "list_contacts", json!({}));
    let e = estimate_at(&a["ground_estimates"], 8, 0);
    assert_eq!(e["men"], 300);
    assert_eq!(e["day"], 0);
    assert_eq!(e["age_days"], 0);
    assert!(e["event"].is_u64());
    assert!(a["contacts"].as_array().expect("a list").is_empty());
    // The area keeps only the estimates within it.
    let none = list(
        &mut session,
        Side::Japanese,
        "list_contacts",
        json!({ "area": { "q": 0, "r": 0, "radius_nm": 40 } }),
    );
    assert!(none["ground_estimates"].is_null(), "{none}");
    let some = list(
        &mut session,
        Side::Japanese,
        "list_contacts",
        json!({ "area": { "q": 8, "r": 0, "radius_nm": 40 } }),
    );
    assert_eq!(
        some["ground_estimates"].as_array().expect("a list").len(),
        1
    );
    // The Allied side holds none at the start.
    let allied = list(&mut session, Side::Allied, "list_contacts", json!({}));
    assert!(allied["ground_estimates"].is_null(), "{allied}");
    // The event the estimate names is one the side may explain.
    let explained = session
        .call(Side::Japanese, "explain", &json!({ "event": e["event"] }))
        .expect("answered");
    assert_eq!(explained["kind"], "explain", "{explained}");
}

#[test]
fn a_scenario_with_no_bombardment_estimate_or_man_hit_is_briefed_as_it_was() {
    let mut session = session_of("06a_base_raid.toml");
    for _ in 0..2 {
        end_day(&mut session);
        for side in [Side::Allied, Side::Japanese] {
            let (text, data) = briefing(&mut session, side);
            let object = data.as_object().expect("an object");
            assert!(!object.contains_key("enemy_ashore"));
            for key in ["bombardments", "shell_fire", "bombardments_against"] {
                assert!(!data["action"].as_object().unwrap().contains_key(key));
            }
            for raid in data["action"]["raids_on_bases"].as_array().expect("a list") {
                assert!(!raid.as_object().unwrap().contains_key("hits_on_men"));
            }
            for claim in data["losses"]["claims"].as_array().expect("a list") {
                assert!(!claim.as_object().unwrap().contains_key("from_bombardment"));
            }
            assert!(!text.contains("ENEMY MEN ASHORE") && !text.contains("Shore bombardment"));
            let contacts = list(&mut session, side, "list_contacts", json!({}));
            assert!(!contacts
                .as_object()
                .unwrap()
                .contains_key("ground_estimates"));
        }
    }
}

#[test]
fn every_answer_of_the_whole_game_passes_its_schema_for_both_sides() {
    // Every answer on every day, every event the side may see explained, every
    // number of the text in the data, and no enemy unit named except as a
    // contact (decision 0048).
    let schemas = schemas();
    let mut scenario = load("24_shells_and_estimates.toml", 1942);
    let name = scenario.name.clone();
    support::play(&mut scenario, |s| {
        for side in [Side::Allied, Side::Japanese] {
            let view = s.world.side_view(side);
            support::check::check_view(&view, &name, &schemas, true);
        }
    });
}
