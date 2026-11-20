//! What a side is told of its objectives (decisions 6 to 8 of 0054).
//!
//! 1. A side is given its own mission and objectives and never the enemy's.
//! 2. A scenario without objectives says `none`.
//! 3. Known progress is counted from the side's own units and the events it
//!    may see: two worlds that differ only in something the side cannot see
//!    (the state of an enemy base) give the side the same known progress and
//!    the same briefing, while the umpire's result differs; and a difference
//!    in the side's own base is seen.
//! 4. On the day the game ends the final briefing and the `end_day` answer
//!    carry the umpire's result, and before that nothing of it.

mod support;

use serde_json::{json, Value};
use sim_agent::briefing;
use sim_core::objectives::KnownBasis;
use sim_core::state::Side;
use sim_core::World;
use support::check::valid;
use support::play::{play_day, session_of};
use support::{load_text, play, scenario_text, schemas};

const RAID: &str = "06a_base_raid.toml";

/// Objectives for the base raid: base 1 is Japanese, base 2 Allied.
const RAID_OBJECTIVES: &str = r#"
[objectives]
minor = 1
major = 2
decisive = 3
history = "A raid on a placeholder airfield."

[objectives.allied]
mission = "ALLIED MISSION: keep your airfield open."

[[objectives.allied.objective]]
kind = "base_open"
base = 2
points = 1

[[objectives.allied.objective]]
kind = "base_open"
base = 1
closed = true
points = 1

[[objectives.allied.objective]]
kind = "aircraft_destroyed"
points = 1

[objectives.japanese]
mission = "JAPANESE MISSION: shut the enemy airfield."

[[objectives.japanese.objective]]
kind = "base_open"
base = 2
closed = true
points = 1

[[objectives.japanese.objective]]
kind = "base_open"
base = 1
points = 1

[[objectives.japanese.objective]]
kind = "aircraft_destroyed"
points = 1
"#;

fn raid_text(extra: &str) -> String {
    format!("{}{extra}", scenario_text(RAID))
}

fn raid(text: &str, seed: u64) -> sim_data::Scenario {
    load_text(text, RAID, seed)
}

#[test]
fn a_side_is_given_its_own_mission_and_objectives_and_never_the_enemys() {
    let scenario = raid(&raid_text(RAID_OBJECTIVES), 1);
    for (side, own, other) in [
        (Side::Allied, "ALLIED MISSION", "JAPANESE MISSION"),
        (Side::Japanese, "JAPANESE MISSION", "ALLIED MISSION"),
    ] {
        let view = scenario.world.side_view(side);
        assert!(view.mission().expect("a mission").starts_with(own));
        assert_eq!(view.objectives().len(), 3);
        let made = briefing(&view, "Raid");
        let text = made.to_text();
        let json = serde_json::to_string(&made).expect("serialises");
        assert!(text.contains(own), "{text}");
        assert!(!text.contains(other), "{text}");
        assert!(!json.contains(other), "{json}");
        // The history note comes with the result, not before.
        assert!(!text.contains("A raid on a placeholder airfield"), "{text}");
        assert!(made.objectives.result.is_none());
    }
}

#[test]
fn a_scenario_without_objectives_says_none() {
    let scenario = support::load("01_transit.toml", 1);
    let view = scenario.world.side_view(Side::Allied);
    assert_eq!(view.mission(), None);
    assert!(view.known_progress().is_empty());
    let made = briefing(&view, "Transit");
    assert_eq!(made.objectives.mission, None);
    let json: Value = serde_json::to_value(&made).expect("serialises");
    assert_eq!(
        json["objectives"],
        json!({"mission": null, "objectives": [], "levels": null, "result": null})
    );
    let text = made.to_text();
    assert!(
        text.contains("OBJECTIVES\n- none: this scenario gives no mission and no objectives\n"),
        "{text}"
    );
    valid(&schemas(), "briefing.schema.json", &json, "briefing");
}

/// The text with the Allied airfield (base 2) emptied of supply: a difference
/// in the Allied side's own base and, for the Japanese side, in an enemy base.
fn without_supply() -> String {
    let text = raid_text(RAID_OBJECTIVES);
    let edited = text.replacen("supply_t = 1000", "supply_t = 0", 1);
    assert_ne!(edited, text);
    edited
}

#[test]
fn two_worlds_that_differ_in_an_enemy_base_give_a_side_the_same_known_progress() {
    // A base is counted at the end of each day (decision 0067), so a day is
    // resolved in each world. Base supply is not in play in this scenario:
    // the stock changes nothing but the count.
    let after_a_day = |text: &str| {
        let mut scenario = raid(text, 1);
        scenario.next_day().expect("day 1 resolves").expect("a day");
        scenario.world
    };
    let stocked = after_a_day(&raid_text(RAID_OBJECTIVES));
    let empty = after_a_day(&without_supply());
    // The umpire's view differs: base 2 ended the day open in one world and
    // shut in the other, so the Japanese objective to shut it scores in one.
    let (a, b) = (
        stocked.result().expect("a result"),
        empty.result().expect("a result"),
    );
    assert_ne!(a.japanese.total, b.japanese.total);

    // The Japanese side cannot see base 2's supply: what it knows of its
    // objective on that base, and its whole briefing, are the same.
    let (one, two) = (
        stocked.side_view(Side::Japanese),
        empty.side_view(Side::Japanese),
    );
    assert_eq!(one.known_progress(), two.known_progress());
    let known = one.known_progress();
    assert_eq!(known[0].basis, KnownBasis::NotObservable);
    assert_eq!(known[0].count.raw(), 0);
    assert_eq!(known[0].points, None);
    assert_eq!(
        briefing(&one, "Raid").to_text(),
        briefing(&two, "Raid").to_text()
    );
    // Its own base 1 it does see (the control below is for base 2).
    assert_eq!(known[1].basis, KnownBasis::Exact);

    // The Allied side owns base 2 and does see the difference.
    let (one, two) = (
        stocked.side_view(Side::Allied),
        empty.side_view(Side::Allied),
    );
    assert_ne!(one.known_progress(), two.known_progress());
    assert_eq!(one.known_progress()[0].basis, KnownBasis::Exact);
    // One day of one, against none of one.
    assert_eq!(one.known_progress()[0].count.raw(), 1_000_000);
    assert_eq!(one.known_progress()[0].days, Some(1));
    assert_eq!(two.known_progress()[0].count.raw(), 0);
    assert_eq!(two.known_progress()[0].days, Some(1));
    // The Allied objective to shut the Japanese base 1 (supply 0, in both
    // worlds) is not observable and says nothing.
    assert_eq!(one.known_progress()[1].basis, KnownBasis::NotObservable);
}

#[test]
fn known_progress_over_a_whole_run_is_exact_for_own_bases_and_blind_for_the_enemys() {
    for seed in 1..=5 {
        let mut scenario = raid(&raid_text(RAID_OBJECTIVES), seed);
        play(&mut scenario, |s| {
            let world: &World = &s.world;
            for side in Side::ALL {
                let known = world.side_view(side).known_progress();
                let result = world.result().expect("a result");
                let counted = &result.side(side).objectives;
                assert_eq!(known.len(), counted.len());
                for (k, true_line) in known.iter().zip(counted) {
                    match k.basis {
                        // Exact: as good as the truth, for what is its own.
                        KnownBasis::Exact => assert_eq!(k.count, true_line.count),
                        // Not observable: nothing, whatever the truth.
                        KnownBasis::NotObservable => assert_eq!(k.count.raw(), 0),
                        // Claims (decision 0056) can be above the truth, and
                        // are the sum of the side's own claims, in aircraft
                        // for this scenario's objective.
                        KnownBasis::Claims => {
                            let claimed: u32 = world
                                .side_view(side)
                                .claims()
                                .iter()
                                .map(|c| c.claimed_shot_down + c.claimed_on_ground)
                                .sum();
                            assert_eq!(k.count.raw(), i64::from(claimed) * 1_000_000);
                        }
                    }
                }
            }
        });
    }
}

// ---------------------------------------------------------------------
// The result at the end of the game.
// ---------------------------------------------------------------------

#[test]
fn the_final_briefing_and_the_end_day_answer_carry_the_result_and_nothing_before() {
    let scenario = load_text(
        &scenario_text("05_eastern_solomons.toml"),
        "05_eastern_solomons.toml",
        1942,
    );
    let (mut session, scripts) = session_of(scenario, &[Side::Allied]);
    let schemas = schemas();

    // Before the game ends: the mission and the objectives, no result.
    let before = session
        .call(Side::Allied, "get_briefing", &json!({"text": false}))
        .expect("a briefing");
    valid(&schemas, "briefing.schema.json", &before, "briefing");
    assert!(before["objectives"]["mission"]
        .as_str()
        .expect("a mission")
        .starts_with("Find and sink"));
    assert_eq!(before["objectives"]["result"], Value::Null);
    assert_eq!(
        before["objectives"]["objectives"].as_array().unwrap().len(),
        3
    );

    let orders = scripts[&Side::Allied].get(&1).cloned().unwrap_or_default();
    let (_refused, ended) = play_day(&mut session, Side::Allied, &orders);
    valid(&schemas, "end_day.schema.json", &ended, "end_day");
    assert_eq!(ended["game_over"], true);
    let result = &ended["result"];
    assert_eq!(result["allied"]["total"], "13");
    // 5 + 11 x 0.05 = 5.55 (11 Allied-destroyed aircraft counted for the
    // Japanese: see scenario_10_objectives), margin 13 - 5.55 = 7.45.
    assert_eq!(result["japanese"]["total"], "5.55");
    assert_eq!(result["allied"]["margin"], "7.45");
    assert_eq!(result["japanese"]["margin"], "-7.45");
    assert_eq!(result["allied"]["outcome"], "minor victory");
    assert_eq!(result["japanese"]["outcome"], "minor defeat");
    assert_eq!(result["allied"]["result"], "victory");
    assert_eq!(result["allied"]["level"], "minor");
    assert!(result["history"].as_str().unwrap().contains("Ryujo"));

    // The final briefing says the same, as data and as text.
    let after = session
        .call(Side::Allied, "get_briefing", &json!({"text": false}))
        .expect("a briefing");
    valid(&schemas, "briefing.schema.json", &after, "final briefing");
    assert_eq!(&after["objectives"]["result"], result);
    let text = session
        .call(Side::Allied, "get_briefing", &json!({"text": true}))
        .expect("a briefing");
    let text = text["text"].as_str().expect("text");
    assert!(
        text.contains("The game is over. The result, counted from the truth:"),
        "{text}"
    );
    assert!(
        text.contains("Allied: 13 points, margin 7.45, minor victory"),
        "{text}"
    );
    assert!(
        text.contains("Japanese: 5.55 points, margin -7.45, minor defeat"),
        "{text}"
    );
    assert!(
        text.contains(
            "History (what happened in the real battle, not in this game): Ryujo was sunk"
        ),
        "{text}"
    );

    // A second end_day after the end gives the result again.
    let again = session
        .call(Side::Allied, "end_day", &json!({}))
        .expect("answered");
    assert_eq!(&again["result"], result);
}

#[test]
fn a_game_without_objectives_ends_with_a_null_result() {
    let scenario = support::load("01_transit.toml", 1);
    let days = scenario.days;
    let (mut session, scripts) = session_of(scenario, &[Side::Allied]);
    let mut ended = Value::Null;
    for day in 1..=days {
        let orders = scripts[&Side::Allied]
            .get(&day)
            .cloned()
            .unwrap_or_default();
        ended = play_day(&mut session, Side::Allied, &orders).1;
    }
    valid(&schemas(), "end_day.schema.json", &ended, "end_day");
    assert_eq!(ended["game_over"], true);
    assert_eq!(ended["result"], Value::Null);
    let after = session
        .call(Side::Allied, "get_briefing", &json!({"text": false}))
        .expect("a briefing");
    assert_eq!(after["objectives"]["result"], Value::Null);
}

// ---------------------------------------------------------------------
// Ships out of action (decision 0088): scenario 20b.
// ---------------------------------------------------------------------

const OUT: &str = "20b_out_of_action.toml";

#[test]
fn a_game_of_ships_out_of_action_is_briefed_and_told_with_the_horizon_and_each_ship() {
    // Hand calculation: scenario 20b's own (tests/scenario_20b_out_of_action.rs):
    // a horizon of 60 days, and for the Allied side seven Japanese ships out,
    // 6.233329 points; the Japanese side one Allied ship, 4 points.
    let scenario = support::load(OUT, 1);
    let days = scenario.days;
    let (mut session, _scripts) = session_of(scenario, &[Side::Allied]);
    let schemas = schemas();

    // Day 1: the objective with its points and the horizon; the progress is
    // not observable.
    let first = session
        .call(Side::Allied, "get_briefing", &json!({"text": false}))
        .expect("a briefing");
    valid(&schemas, "briefing.schema.json", &first, "day 1 briefing");
    let lines = first["objectives"]["objectives"].as_array().expect("lines");
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0]["kind"], "ships_sunk");
    assert!(lines[0].get("horizon_days").is_none(), "{}", lines[0]);
    let out = &lines[1];
    assert_eq!(out["kind"], "ships_out_of_action");
    assert_eq!(out["horizon_days"], 60);
    assert_eq!(
        out["ship_points"],
        json!([
            { "kind": "carrier", "points": "5" },
            { "kind": "battleship", "points": "4" },
            { "kind": "cruiser", "points": "2" },
            { "kind": "destroyer", "points": "0.5" },
        ])
    );
    assert_eq!(out["known"]["basis"], "not_observable");
    assert_eq!(out["known"]["count"], "0");
    assert_eq!(out["known"]["points"], Value::Null);
    let text = session
        .call(Side::Allied, "get_briefing", &json!({"text": true}))
        .expect("a briefing");
    let text = text["text"].as_str().expect("text");
    assert!(
        text.contains("Objective: enemy ships out of action, counted by the share of the 60-day horizon each is away for repair (her days of repair at the end plus those she left for a yard with), points for a ship away for the whole horizon: carrier 5, battleship 4, cruiser 2, destroyer 0.5"),
        "{text}"
    );

    // Play the game; every answer is checked against its schema.
    let mut ended = Value::Null;
    for day in 1..=days {
        let (refused, answer) = play_day(&mut session, Side::Allied, &[]);
        assert!(refused.is_empty(), "{refused:?}");
        valid(&schemas, "end_day.schema.json", &answer, "end_day");
        if day < days {
            assert_eq!(answer["result"], Value::Null);
            let b = session
                .call(Side::Allied, "get_briefing", &json!({"text": false}))
                .expect("a briefing");
            valid(&schemas, "briefing.schema.json", &b, "briefing");
            assert_eq!(
                b["objectives"]["objectives"][1]["known"]["basis"],
                "not_observable"
            );
            assert_eq!(b["objectives"]["result"], Value::Null);
        }
        ended = answer;
    }
    assert_eq!(ended["game_over"], true);

    // The Allied side's score: the seven Japanese ships, which it has no
    // handle for, listed by their place; case 1 of the scenario's test.
    let result = &ended["result"];
    let score = &result["allied"]["objectives"][1];
    assert_eq!(score["kind"], "ships_out_of_action");
    assert_eq!(score["horizon_days"], 60);
    assert_eq!(score["count"], "7");
    assert_eq!(score["points"], "6.233329");
    assert_eq!(result["allied"]["total"], "6.233329");
    assert_eq!(result["japanese"]["total"], "4");
    let expected = json!([
        { "ship": "enemy_ship:1", "kind": "carrier", "at_end": 29, "left_with": 0, "days_out": 29, "share": "0.483333", "points": "2.416665" },
        { "ship": "enemy_ship:2", "kind": "cruiser", "at_end": 30, "left_with": 0, "days_out": 30, "share": "0.5", "points": "1" },
        { "ship": "enemy_ship:3", "kind": "cruiser", "at_end": 80, "left_with": 0, "days_out": 80, "share": "1", "points": "2" },
        { "ship": "enemy_ship:4", "kind": "cruiser", "at_end": 17, "left_with": 0, "days_out": 17, "share": "0.283333", "points": "0.566666" },
        { "ship": "enemy_ship:5", "kind": "destroyer", "at_end": 0, "left_with": 2, "days_out": 2, "share": "0.033333", "points": "0.016666" },
        { "ship": "enemy_ship:6", "kind": "destroyer", "at_end": 0, "left_with": 12, "days_out": 12, "share": "0.2", "points": "0.1" },
        { "ship": "enemy_ship:7", "kind": "cruiser", "at_end": 4, "left_with": 0, "days_out": 4, "share": "0.066666", "points": "0.133332" },
    ]);
    assert_eq!(score["ships_out"], expected);

    // The Japanese side's score: one ship, which is the Allied side's own
    // (ship:1 to it), away for the whole horizon and more.
    let japanese = &result["japanese"]["objectives"][1];
    assert_eq!(japanese["points"], "4");
    assert_eq!(
        japanese["ships_out"],
        json!([{
            "ship": "ship:1", "kind": "battleship", "at_end": 70, "left_with": 0,
            "days_out": 70, "share": "1", "points": "4"
        }])
    );
    // The other kinds' score lines have neither field.
    let sunk = &result["allied"]["objectives"][0];
    assert!(sunk.get("horizon_days").is_none() && sunk.get("ships_out").is_none());

    // The final briefing says the same, and the text gives each ship.
    let after = session
        .call(Side::Allied, "get_briefing", &json!({"text": false}))
        .expect("a briefing");
    valid(&schemas, "briefing.schema.json", &after, "final briefing");
    assert_eq!(&after["objectives"]["result"], result);
    let text = session
        .call(Side::Allied, "get_briefing", &json!({"text": true}))
        .expect("a briefing");
    let text = text["text"].as_str().expect("text");
    assert!(
        text.contains("ships out of action: count 7 over 60 days"),
        "{text}"
    );
    assert!(
        text.contains("enemy_ship:1 carrier: 29 days at the end + 0 left with = 29 days out, share 0.483333, points 2.416665"),
        "{text}"
    );
    assert!(
        text.contains(
            "ship:1 battleship: 70 days at the end + 0 left with = 70 days out, share 1, points 4"
        ),
        "{text}"
    );
}

#[test]
fn two_worlds_that_differ_in_an_enemy_ships_damage_give_a_side_the_same_ships_out_of_action_briefing(
) {
    // Japanese ship 1 (a carrier) has 12 points of flotation damage in one
    // world and 30 in the other. The Allied side cannot see an enemy ship's
    // damage or her days in repair.
    let base = scenario_text(OUT);
    let worse = base.replacen("flotation_damage = \"12\"", "flotation_damage = \"30\"", 1);
    assert_ne!(base, worse);

    let record = |text: &str| {
        let mut scenario = load_text(text, OUT, 1);
        let mut days = Vec::new();
        play(&mut scenario, |s| {
            let view = s.world.side_view(Side::Allied);
            let b = briefing(&view, "20b");
            days.push((
                serde_json::to_value(&b).expect("serialises"),
                b.to_text(),
                view.known_progress(),
            ));
        });
        let result = scenario.world.result().expect("a result");
        (
            days,
            result.allied.total,
            result.allied.objectives[1].ships_out[0].days_out,
        )
    };
    let (one, total_one, out_one) = record(&base);
    let (two, total_two, out_two) = record(&worse);

    assert!(one.len() > 1);
    assert_eq!(one.len(), two.len());
    for (day, (a, b)) in one.iter().zip(&two).enumerate() {
        let at = day + 1;
        assert_eq!(a.0, b.0, "the briefing as data, at the start of day {at}");
        assert_eq!(a.1, b.1, "the briefing as text, at the start of day {at}");
        assert_eq!(a.2, b.2, "the known progress, at the start of day {at}");
        assert_eq!(a.2[1].basis, KnownBasis::NotObservable);
    }
    // The umpire's result differs, which is allowed: the carrier is away
    // 29 days in one world and more in the other.
    assert_eq!(out_one, 29);
    assert!(out_two > out_one, "{out_two}");
    assert_ne!(total_one, total_two);
}

// ---------------------------------------------------------------------
// Men landed (decision 0186) and a base not on the reader's map.
// ---------------------------------------------------------------------

/// One objective of each kind for each side (a scenario counts damaged ships
/// one way, so `damage` is one of ships_crippled and ships_out_of_action: the
/// nine kinds are between the two texts); bases 1 and 2 of the raid.
const EVERY_KIND: &str = r#"
[objectives]
minor = 1
major = 2
decisive = 3
horizon_days = 3

[objectives.allied]
mission = "ALLIED MISSION"

[objectives.japanese]
mission = "JAPANESE MISSION"
"#;

fn every_kind_for(side: &str, damage: &str) -> String {
    format!(
        r#"
[[objectives.{side}.objective]]
kind = "ships_sunk"
points = {{ carrier = 1 }}
[[objectives.{side}.objective]]
kind = "{damage}"
points = {{ carrier = 1 }}
[[objectives.{side}.objective]]
kind = "aircraft_destroyed"
points = 1
[[objectives.{side}.objective]]
kind = "cargo_landed"
base = 1
points_per_t = "0.005"
[[objectives.{side}.objective]]
kind = "base_open"
base = 2
points = 1
[[objectives.{side}.objective]]
kind = "base_open"
base = 1
closed = true
points = 1
[[objectives.{side}.objective]]
kind = "base_held"
base = 2
points = 1
[[objectives.{side}.objective]]
kind = "men_lost"
points_per_100_men = 1
[[objectives.{side}.objective]]
kind = "men_landed"
base = 1
points_per_100_men = 1
"#
    )
}

fn nine_kinds_text(damage: &str) -> String {
    let (head, _) = EVERY_KIND.split_once("[objectives.allied]").expect("split");
    format!(
        "{}{head}[objectives.allied]\nmission = \"ALLIED MISSION\"\n{}\n[objectives.japanese]\nmission = \"JAPANESE MISSION\"\n{}",
        scenario_text(RAID),
        every_kind_for("allied", damage),
        every_kind_for("japanese", damage)
    )
}

/// A base as the text writes it: the handle, and the name in quotes.
fn handle_text(h: &sim_agent::Handle) -> String {
    match &h.name {
        Some(name) => format!("{} \"{}\"", h.handle, name),
        None => h.handle.clone(),
    }
}

#[test]
fn men_landed_has_its_words_for_the_side_and_for_the_enemy() {
    let scenario = raid(&nine_kinds_text("ships_crippled"), 1);
    let allied = scenario.world.side_view(Side::Allied);
    let made = briefing(&allied, "Raid");
    let text = made.to_text();
    let own = made
        .objectives
        .objectives
        .iter()
        .find(|l| l.kind == "men_landed")
        .expect("own line");
    assert_eq!(own.points.map(|p| p.0.to_string()), Some("1".to_string()));
    let base = handle_text(own.base.as_ref().expect("a base"));
    assert!(
        text.contains(&format!(
            "Objective: men you land from ships at {base} and do not take off again, points for each hundred men: 1"
        )),
        "{text}"
    );
    // Exact progress, 0 men at the start.
    assert_eq!(own.known.basis, "exact");
    assert!(
        text.contains("What you know of your progress: 0, worth 0 points"),
        "{text}"
    );
    let enemy = made
        .objectives
        .enemy_objectives
        .as_ref()
        .expect("enemy objectives")
        .iter()
        .find(|l| l.kind == "men_landed")
        .expect("enemy line");
    let base = handle_text(enemy.base.as_ref().expect("a base"));
    assert!(
        text.contains(&format!(
            "men the enemy lands from ships at {base} and does not take off again, points for each hundred men: 1"
        )),
        "{text}"
    );
}

#[test]
fn an_objective_line_of_each_of_the_nine_kinds_is_written_without_panicking() {
    let mut kinds = std::collections::BTreeSet::new();
    for (damage, side) in [
        ("ships_crippled", Side::Allied),
        ("ships_out_of_action", Side::Japanese),
    ] {
        let scenario = raid(&nine_kinds_text(damage), 1);
        let view = scenario.world.side_view(side);
        let made = briefing(&view, "Raid");
        assert_eq!(made.objectives.objectives.len(), 9);
        assert_eq!(
            made.objectives
                .enemy_objectives
                .as_ref()
                .expect("enemy")
                .len(),
            9
        );
        kinds.extend(made.objectives.objectives.iter().map(|l| l.kind));
        let text = made.to_text();
        assert!(text.contains("men you land from ships"), "{text}");
        let json = serde_json::to_value(&made).expect("serialises");
        let schemas = schemas();
        valid(&schemas, "briefing.schema.json", &json, "nine kinds");
    }
    assert_eq!(kinds.len(), 9, "{kinds:?}");
}

#[test]
fn an_objective_at_a_base_not_on_the_readers_map_is_named_by_its_hex() {
    // Base 1, the Japanese airfield, is not on the Allied side's map.
    let text = nine_kinds_text("ships_crippled").replacen(
        "name = \"Placeholder airfield A\"\nside = \"japanese\"",
        "name = \"Placeholder airfield A\"\nside = \"japanese\"\non_map = false",
        1,
    );
    assert!(text.contains("on_map = false"));
    let scenario = raid(&text, 1);
    let japanese = scenario.world.side_view(Side::Japanese);
    let hex = japanese
        .bases()
        .find(|b| b.id == sim_core::ids::BaseId::new(1))
        .expect("base 1")
        .hex;
    let hex_text = hex.to_string();
    let allied = scenario.world.side_view(Side::Allied);
    assert!(!allied
        .enemy_bases()
        .any(|b| b.id == sim_core::ids::BaseId::new(1)));
    let made = briefing(&allied, "Raid");
    let text = made.to_text();
    // The Japanese objective at base 1 is written by its hex, no name.
    let enemy_lines: Vec<_> = made
        .objectives
        .enemy_objectives
        .as_ref()
        .expect("enemy")
        .iter()
        .filter(|l| matches!(l.kind, "men_landed" | "cargo_landed" | "base_open"))
        .filter(|l| l.base.as_ref().is_some_and(|b| b.handle == hex_text))
        .collect();
    assert!(enemy_lines.len() >= 3, "{enemy_lines:?}");
    assert!(enemy_lines
        .iter()
        .all(|l| l.base.as_ref().unwrap().name.is_none()));
    assert!(
        text.contains(&format!(
            "men the enemy lands from ships at {hex_text} and does not take off again"
        )),
        "{text}"
    );
    // The Allied side's own objective at base 1 (an enemy base it has not
    // found) is also written by its hex; base 2, on its map, by number.
    let own = made
        .objectives
        .objectives
        .iter()
        .find(|l| l.kind == "men_landed")
        .expect("own");
    assert_eq!(own.base.as_ref().expect("base").handle, hex_text);
    assert!(text.contains("base:2"), "{text}");
    assert!(!text.contains("base:1 "), "{text}");
}
