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
    assert_eq!(result["japanese"]["total"], "5.4");
    assert_eq!(result["allied"]["margin"], "7.6");
    assert_eq!(result["japanese"]["margin"], "-7.6");
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
        text.contains("Allied: 13 points, margin 7.6, minor victory"),
        "{text}"
    );
    assert!(
        text.contains("Japanese: 5.4 points, margin -7.6, minor defeat"),
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
