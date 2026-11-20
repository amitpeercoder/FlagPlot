//! Scenario 102: the enemy's objectives shown. Each side reads, in its
//! briefing, what the other side is scored for: the kinds, the places and
//! what each is worth. It reads nothing of the enemy's mission, progress
//! or state.
//!
//! Proves decision 0183. There is no arithmetic in it. What follows is
//! stated by the architect before the code, and each statement is a test.
//! If the code disagrees, the code or the rule is wrong: a mismatch goes
//! back to the architect.
//!
//! It runs on existing scenarios, through a session as the other tests of
//! the agent's interface open one. No new data.
//!
//! - **ES:** `05_eastern_solomons.toml`: both sides have objectives, none
//!   naming a base.
//! - **FS:** `07_full_slice.toml`: both sides have objectives, some naming
//!   a base (supply landed at Lunga, Lunga open, Lunga and Tulagi held).
//! - **T:** `01_transit.toml`: no objectives.
//! - **CL:** `stage-2b-proof/05a_eastern_solomons.toml`: command levels
//!   on, and objectives.
//!
//! # Statements
//!
//! 1. **The enemy's objectives are listed, as the scenario gives them.**
//!    On FS, at the Allied seat on the evening before day 1: the
//!    briefing's objectives part has, after the side's own objectives,
//!    the Japanese side's, one for one and in the scenario's order, each
//!    with its kind, the base it names where it names one (by the map's
//!    number and name), whether a base is to be open or shut, and its
//!    worth (points; points a ton; points for each kind of ship), equal
//!    to the scenario file's own figures. On FS the Japanese objectives
//!    name Lunga and Tulagi, and one is that Lunga be shut. (Corrected by
//!    the architect on Nov 20, 2026, on the build's report: this said
//!    Lunga alone, which the file does not bear out.) And at the Japanese seat, the Allied
//!    side's, likewise.
//! 2. **What a seat is shown of the enemy's is what the enemy is shown
//!    of his own,** but for progress: for each objective, the kind, the
//!    base, open or shut, and the worth in the Allied briefing's list of the Japanese
//!    objectives equal those in the Japanese briefing's list of its own.
//! 3. **No progress and no state.** An enemy's objective in a briefing
//!    is of a form that has no field for a count, days counted, points
//!    earned, a basis or "open now"; the briefing's schema says so. Played to day 5 on FS with the script's orders handed in for both
//!    sides, so that Lunga changes side on day 1, with fights and landings, the enemy's
//!    list in each side's briefing is, byte for byte, what it was on the
//!    evening before day 1.
//! 4. **The enemy's mission is not shown.** No briefing, as text or as
//!    data, holds any sentence of the other side's mission; and two
//!    worlds that differ only in the enemy's mission give the same
//!    briefing, byte for byte (a case in the agent's source test).
//! 5. **A base is named by the map's number and name.** On FS, in the
//!    Japanese briefing, the Allied objective of supply landed names
//!    Lunga as the Japanese seat's own tools name that base, before it
//!    changes side and after.
//! 6. **The text says whose they are, from the reader's side.** The
//!    briefing as text has the side's own objectives and the line of the
//!    levels as before, then a line "The enemy is scored for:" and the
//!    enemy's, each in words written from the reader's side: "your ships
//!    sunk", "tons the enemy unloads at Lunga", "Lunga is the enemy's at
//!    the end", "days Lunga is shut". No line of the enemy's says "enemy
//!    ships sunk" or "you unload".
//! 7. **A scenario with no objectives shows none:** on T the briefing,
//!    as text and as data, is what it was before this record, byte for
//!    byte but for the version of the agent's interface.
//! 8. **A seat at a command is shown none:** on CL the briefing at a
//!    task force's seat has no enemy objectives, as it has none of its
//!    side's.
//! 9. **A side with objectives whose enemy has none** is told so in one
//!    line. Tested on a copy of ES made by the test in which the
//!    Japanese side keeps its table and its mission and has no objective
//!    entries: the data has the part, empty, and the text the line.
//! 10. **Nothing else changed.** On ES and FS the daily hashes are what
//!     they were; the staffs give the orders they gave; the result at
//!     the end is what it was.
//! 11. **The answers are checked against their schemas,** and the
//!     briefing's schema names the new part.

mod common;

#[allow(dead_code)]
#[path = "../crates/sim-agent/tests/support/validator.rs"]
mod validator;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use sim_agent::{Post, SeatKind, Session};
use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::ids::{BaseId, TaskForceId};
use sim_core::objectives::Objective;
use sim_core::pictures::CommandRef;
use sim_core::refdata::ShipKind;
use sim_core::state::Side;
use sim_core::World;
use sim_data::Scenario;

const ES: &str = "05_eastern_solomons.toml";
const FS: &str = "07_full_slice.toml";
const T: &str = "01_transit.toml";
const CL: &str = "stage-2b-proof/05a_eastern_solomons.toml";
const A: Side = Side::Allied;
const J: Side = Side::Japanese;

/// The line that opens the enemy's objectives in the text.
const SCORED_FOR: &str = "- The enemy is scored for:";

// ---------------------------------------------------------------------
// Helpers. Every game here is on the live figures: no statement asserts a
// number that depends on a figure, only what the scenario files state and
// what the tools say of it.
// ---------------------------------------------------------------------

fn path(name: &str) -> PathBuf {
    common::data_dir().join("scenarios").join(name)
}

fn path_text(name: &str) -> String {
    path(name).to_str().expect("a path in words").to_string()
}

fn other(side: Side) -> Side {
    match side {
        A => J,
        J => A,
    }
}

/// A game opened as `serve --watched` opens one, both seats the script's, so
/// that the session hands in the script's orders for both sides; read by
/// the watcher, as each seat would be answered.
fn scripted(name: &str) -> Session {
    sim_cli::game::open(&path_text(name), None, BTreeMap::new(), &BTreeMap::new())
        .expect("the game opens")
        .0
        .watched()
}

/// As [`scripted`], on a copy of the scenario's text made by the test.
fn scripted_text(name: &str, text: &str) -> Session {
    let scenario = Scenario::from_text(text, &path(name)).expect("the copy loads");
    sim_cli::game::open_loaded(
        &path_text(name),
        scenario,
        BTreeMap::new(),
        &BTreeMap::new(),
    )
    .expect("the game opens")
    .0
    .watched()
}

/// A game with one seat a player's at a post, the other the script's.
fn at_post(name: &str, side: Side, post: Post) -> Session {
    let seats = BTreeMap::from([(side, SeatKind::Player)]);
    let posts = BTreeMap::from([(side, post)]);
    sim_cli::game::open(&path_text(name), None, seats, &posts)
        .expect("the game opens")
        .0
}

/// The briefing at a seat, as data and as the text answer.
struct Read {
    data: Value,
    text_answer: Value,
}

impl Read {
    fn text(&self) -> &str {
        self.text_answer["text"].as_str().expect("the text")
    }

    fn objectives(&self) -> &Value {
        &self.data["objectives"]
    }

    fn enemy(&self) -> &Vec<Value> {
        self.objectives()["enemy_objectives"]
            .as_array()
            .expect("the enemy's objectives are listed")
    }
}

fn watch(s: &mut Session, side: Side, tool: &str, args: Value) -> Value {
    s.watch(side, tool, &args)
        .unwrap_or_else(|e| panic!("{tool} at {side:?}: {}", e.message))
}

fn read(s: &mut Session, side: Side) -> Read {
    Read {
        data: watch(s, side, "get_briefing", json!({"text": false})),
        text_answer: watch(s, side, "get_briefing", json!({"text": true})),
    }
}

fn call(s: &mut Session, side: Side, tool: &str, args: Value) -> Value {
    s.call(side, tool, &args)
        .unwrap_or_else(|e| panic!("{tool} at {side:?}: {}", e.message))
}

/// The watcher steps until `day` is resolved and the clock stands at dusk.
fn to_day(s: &mut Session, day: u32) {
    while s.day() < day || s.open_day().is_some() {
        let stepped = s.step();
        stepped
            .answer
            .unwrap_or_else(|e| panic!("the step is refused: {}", e.message));
    }
}

/// The lines of the text's OBJECTIVES part.
fn objectives_lines(text: &str) -> Vec<&str> {
    text.lines()
        .skip_while(|l| *l != "OBJECTIVES")
        .skip(1)
        .take_while(|l| !l.is_empty())
        .collect()
}

/// The enemy's block of the text: the line that says whose they are and the
/// lines under it.
fn enemy_block(text: &str) -> Vec<String> {
    let lines = objectives_lines(text);
    let Some(start) = lines.iter().position(|l| l.starts_with(SCORED_FOR)) else {
        return Vec::new();
    };
    let mut block = vec![lines[start].to_string()];
    block.extend(
        lines[start + 1..]
            .iter()
            .take_while(|l| l.starts_with("  - "))
            .map(|l| l.to_string()),
    );
    block
}

/// The scenario's configuration, as the umpire holds it.
fn objectives_of(name: &str, side: Side) -> (Vec<Objective>, u32) {
    let scenario = Scenario::load(&path(name)).expect("loads");
    let objectives = scenario.world.objectives().expect("objectives");
    (
        objectives.side(side).objectives.clone(),
        objectives.horizon_days,
    )
}

fn dec(value: Fixed) -> Value {
    json!(value.to_string())
}

fn ship_points(points: &sim_core::objectives::ShipPoints) -> Value {
    Value::Array(
        ShipKind::ALL
            .into_iter()
            .filter_map(|kind| {
                points
                    .0
                    .get(&kind)
                    .map(|p| json!({"kind": kind.name(), "points": dec(*p)}))
            })
            .collect(),
    )
}

/// What a line of the enemy's must say of one objective, worked here from
/// the configuration and apart from the library: the kind, the base by the
/// map's number and name, open or shut, and the worth.
fn expected_line(world: &World, objective: &Objective, horizon: u32) -> Value {
    let base = |id: BaseId| {
        let name = world.umpire().bases[&id].name.clone();
        json!({"handle": format!("base:{}", id.raw()), "name": name})
    };
    let mut line = json!({
        "kind": objective.name(),
        "base": objective.base().map_or(Value::Null, base),
        "closed": Value::Null,
        "ship_points": [],
        "points": Value::Null,
    });
    match objective {
        Objective::ShipsSunk(p) | Objective::ShipsCrippled(p) => {
            line["ship_points"] = ship_points(p);
        }
        Objective::ShipsOutOfAction(p) => {
            line["ship_points"] = ship_points(p);
            line["horizon_days"] = json!(horizon);
        }
        Objective::AircraftDestroyed { points } | Objective::BaseHeld { points, .. } => {
            line["points"] = dec(*points);
        }
        Objective::CargoLanded { points_per_t, .. } => {
            line["points"] = dec(*points_per_t);
            // The worth for a thousand tons, which the text gives.
            line["points_per_1000_t"] = dec(*points_per_t * 1000);
        }
        Objective::BaseOpen { closed, points, .. } => {
            line["closed"] = json!(closed);
            line["points"] = dec(*points);
        }
        Objective::MenLost { points_per_100_men } => line["points"] = dec(*points_per_100_men),
    }
    line
}

/// An own objective's line cut to what an enemy's line has: no progress and
/// nothing of now.
fn without_progress(line: &Value) -> Value {
    let mut cut = line.clone();
    let object = cut.as_object_mut().expect("an object");
    object.remove("known");
    object.remove("open_now");
    cut
}

fn schemas() -> validator::Schemas {
    validator::Schemas::load(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("../crates/sim-agent/schemas"),
    )
}

fn valid(schemas: &validator::Schemas, read: &Read, label: &str) {
    for (file, value) in [
        ("briefing.schema.json", &read.data),
        ("briefing_text.schema.json", &read.text_answer),
    ] {
        let errors = schemas.check(file, value);
        assert!(errors.is_empty(), "{label}, {file}: {errors:#?}");
    }
}

/// The handle a seat's own `list_bases` gives the base of that name, among
/// its own bases or the enemy's.
fn listed_handle(s: &mut Session, side: Side, name: &str) -> (Value, &'static str) {
    let listed = watch(s, side, "list_bases", json!({}));
    for list in ["bases", "enemy_bases"] {
        for line in listed[list].as_array().expect("a list") {
            if line["base"]["name"] == name {
                return (line["base"].clone(), list);
            }
        }
    }
    panic!("{name} is not listed at {side:?}: {listed}")
}

// ---------------------------------------------------------------------
// 1. The enemy's objectives are listed, as the scenario gives them.
// ---------------------------------------------------------------------

#[test]
fn s1_the_enemys_objectives_are_listed_as_the_scenario_gives_them() {
    let world = Scenario::load(&path(FS)).expect("loads").world;
    let mut s = scripted(FS);
    for side in [A, J] {
        let r = read(&mut s, side);
        let (enemy, horizon) = objectives_of(FS, other(side));
        let expected: Vec<Value> = enemy
            .iter()
            .map(|o| expected_line(&world, o, horizon))
            .collect();
        assert_eq!(r.enemy(), &expected, "{side:?}");
        // After the side's own: the part still has them, with the levels.
        assert_eq!(
            r.objectives()["objectives"].as_array().expect("own").len(),
            objectives_of(FS, side).0.len()
        );
        assert!(r.objectives()["levels"].is_object());
    }
    // As the scenario file writes the Japanese side's, read at the Allied
    // seat, figure by figure.
    let r = read(&mut s, A);
    let lunga = json!({"handle": "base:1", "name": "Lunga"});
    let tulagi = json!({"handle": "base:4", "name": "Tulagi"});
    let ships = |d: &str, half: &str| {
        json!([
            {"kind": "carrier", "points": if d == "sunk" { "10" } else { "5" }},
            {"kind": "battleship", "points": if d == "sunk" { "8" } else { "4" }},
            {"kind": "cruiser", "points": if d == "sunk" { "4" } else { "2" }},
            {"kind": "destroyer", "points": half},
            {"kind": "transport", "points": if d == "sunk" { "2" } else { "1" }},
            {"kind": "auxiliary", "points": half},
        ])
    };
    assert_eq!(
        r.enemy(),
        &vec![
            json!({"kind": "ships_sunk", "base": null, "closed": null, "ship_points": ships("sunk", "1"), "points": null}),
            json!({"kind": "ships_crippled", "base": null, "closed": null, "ship_points": ships("crippled", "0.5"), "points": null}),
            json!({"kind": "aircraft_destroyed", "base": null, "closed": null, "ship_points": [], "points": "0.05"}),
            json!({"kind": "base_open", "base": lunga, "closed": true, "ship_points": [], "points": "10"}),
            json!({"kind": "base_held", "base": lunga, "closed": null, "ship_points": [], "points": "10"}),
            json!({"kind": "base_held", "base": tulagi, "closed": null, "ship_points": [], "points": "5"}),
            json!({"kind": "men_lost", "base": null, "closed": null, "ship_points": [], "points": "1"}),
        ]
    );
    // One of them is that Lunga be shut.
    assert!(r
        .enemy()
        .iter()
        .any(|l| l["kind"] == "base_open" && l["base"] == lunga && l["closed"] == true));
}

// ---------------------------------------------------------------------
// 2. What a seat is shown of the enemy's is what the enemy is shown of his
//    own, but for progress.
// ---------------------------------------------------------------------

#[test]
fn s2_what_a_seat_is_shown_of_the_enemys_is_what_the_enemy_is_shown_of_his_own() {
    for name in [FS, ES] {
        let mut s = scripted(name);
        let allied = read(&mut s, A);
        let japanese = read(&mut s, J);
        for (reader, owner) in [(&allied, &japanese), (&japanese, &allied)] {
            let own: Vec<Value> = owner.objectives()["objectives"]
                .as_array()
                .expect("own")
                .iter()
                .map(without_progress)
                .collect();
            assert!(!own.is_empty(), "{name}");
            assert_eq!(reader.enemy(), &own, "{name}");
        }
    }
}

// ---------------------------------------------------------------------
// 3. No progress and no state.
// ---------------------------------------------------------------------

#[test]
fn s3_an_enemys_objective_has_no_field_for_progress_or_state() {
    let schema: Value = serde_json::from_str(
        &std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../crates/sim-agent/schemas/briefing.schema.json"),
        )
        .expect("the schema reads"),
    )
    .expect("the schema parses");
    let line = &schema["$defs"]["enemy_objective"];
    assert_eq!(line["additionalProperties"], false);
    let fields: Vec<&str> = line["properties"]
        .as_object()
        .expect("properties")
        .keys()
        .map(String::as_str)
        .collect();
    let mut sorted = fields.clone();
    sorted.sort_unstable();
    assert_eq!(
        sorted,
        [
            "base",
            "closed",
            "horizon_days",
            "kind",
            "points",
            "points_per_1000_t",
            "ship_points"
        ]
    );
    for banned in [
        "count",
        "days",
        "known",
        "basis",
        "open_now",
        "earned",
        "ships",
        "ships_out",
    ] {
        assert!(!fields.contains(&banned), "{banned}");
    }
}

#[test]
fn s3_played_to_day_5_the_enemys_list_is_what_it_was() {
    let mut s = scripted(FS);
    let before: BTreeMap<Side, (Vec<Value>, Vec<String>)> = [A, J]
        .into_iter()
        .map(|side| {
            let r = read(&mut s, side);
            (side, (r.enemy().clone(), enemy_block(r.text())))
        })
        .collect();
    let (lunga_before, list_before) = listed_handle(&mut s, J, "Lunga");
    assert_eq!(
        list_before, "bases",
        "Lunga is the Japanese side's at the start"
    );
    for day in 1..=5 {
        to_day(&mut s, day);
        for side in [A, J] {
            let r = read(&mut s, side);
            let (data, text) = &before[&side];
            assert_eq!(r.enemy(), data, "day {day}, {side:?}");
            assert_eq!(&enemy_block(r.text()), text, "day {day}, {side:?}");
            assert!(text.len() > 1);
            // Byte for byte as data too.
            assert_eq!(
                serde_json::to_string(r.enemy()).unwrap(),
                serde_json::to_string(data).unwrap()
            );
        }
        if day == 1 {
            let (lunga_after, list_after) = listed_handle(&mut s, J, "Lunga");
            assert_eq!(list_after, "enemy_bases", "Lunga changed side on day 1");
            assert_eq!(lunga_after, lunga_before);
        }
    }
    // There were fights and landings in those days.
    let world = s.into_world();
    let events = world.events();
    let changed_on_1 = events.iter().any(|e| {
        e.day == 1 && matches!(&e.kind, EventKind::BaseChangedSide { base, .. } if base.raw() == 1)
    });
    assert!(changed_on_1, "Lunga changed side on day 1");
    assert!(events
        .iter()
        .any(|e| matches!(e.kind, EventKind::MenLanded { .. })));
    assert!(events.iter().any(|e| matches!(
        e.kind,
        EventKind::GroundFight { .. } | EventKind::AirFight { .. } | EventKind::Salvo { .. }
    )));
}

// ---------------------------------------------------------------------
// 4. The enemy's mission is not shown.
// ---------------------------------------------------------------------

/// Each sentence of a mission.
fn sentences(mission: &str) -> Vec<String> {
    mission
        .split(". ")
        .map(|s| s.trim().trim_end_matches('.').to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn mission_of(name: &str, side: Side) -> String {
    let scenario = Scenario::load(&path(name)).expect("loads");
    let objectives = scenario.world.objectives().expect("objectives");
    objectives.side(side).mission.clone()
}

#[test]
fn s4_no_briefing_holds_a_sentence_of_the_enemys_mission() {
    for name in [FS, ES] {
        let mut s = scripted(name);
        for day in 0..=1 {
            to_day(&mut s, day);
            for side in [A, J] {
                let r = read(&mut s, side);
                let data = serde_json::to_string(&r.data).unwrap();
                let own = mission_of(name, side);
                assert!(r.text().contains(&own));
                for sentence in sentences(&mission_of(name, other(side))) {
                    assert!(!r.text().contains(&sentence), "{name} {side:?}: {sentence}");
                    assert!(!data.contains(&sentence), "{name} {side:?}: {sentence}");
                }
            }
        }
    }
}

#[test]
fn s4_two_worlds_differing_only_in_the_enemys_mission_give_the_same_briefing() {
    // The case in the agent's source test is
    // `leaks::the_enemys_mission_changes_nothing_the_briefing_says`, on ES.
    // Here the same on FS, through the session, on the evening before day 1
    // and after day 1.
    let text = std::fs::read_to_string(path(FS)).expect("reads");
    for side in [A, J] {
        let enemy_mission = mission_of(FS, other(side));
        let altered = common::replace_once(
            &text,
            &format!("mission = \"{enemy_mission}\""),
            "mission = \"PLACEHOLDER: another mission altogether.\"",
        );
        let mut one = scripted_text(FS, &text);
        let mut two = scripted_text(FS, &altered);
        for day in 0..=1 {
            to_day(&mut one, day);
            to_day(&mut two, day);
            let (a, b) = (read(&mut one, side), read(&mut two, side));
            assert_eq!(
                serde_json::to_string(&a.data).unwrap(),
                serde_json::to_string(&b.data).unwrap(),
                "{side:?} day {day}"
            );
            assert_eq!(a.text(), b.text(), "{side:?} day {day}");
        }
    }
}

// ---------------------------------------------------------------------
// 5. A base is named by the map's number and name.
// ---------------------------------------------------------------------

#[test]
fn s5_a_base_is_named_as_the_seats_own_tools_name_it() {
    let mut s = scripted(FS);
    let cargo = |r: &Read| -> Value {
        r.enemy()
            .iter()
            .find(|l| l["kind"] == "cargo_landed")
            .expect("the Allied objective of supply landed")["base"]
            .clone()
    };
    let r = read(&mut s, J);
    let (lunga, list) = listed_handle(&mut s, J, "Lunga");
    assert_eq!(list, "bases");
    assert_eq!(cargo(&r), lunga);
    assert_eq!(lunga, json!({"handle": "base:1", "name": "Lunga"}));
    to_day(&mut s, 1);
    let r = read(&mut s, J);
    let (lunga_after, list) = listed_handle(&mut s, J, "Lunga");
    assert_eq!(list, "enemy_bases", "after it changed side");
    assert_eq!(cargo(&r), lunga_after);
    assert_eq!(lunga_after, lunga);
}

// ---------------------------------------------------------------------
// 6. The text says whose they are, from the reader's side.
// ---------------------------------------------------------------------

#[test]
fn s6_the_text_says_whose_they_are_from_the_readers_side() {
    let mut s = scripted(FS);
    for side in [A, J] {
        let r = read(&mut s, side);
        let lines = objectives_lines(r.text());
        // The side's own objectives and the levels as before, then the
        // enemy's.
        assert!(lines[0].starts_with("- Mission: "));
        let levels = lines
            .iter()
            .position(|l| l.starts_with("- Levels: "))
            .expect("the levels");
        assert!(lines[1..levels]
            .iter()
            .all(|l| l.starts_with("- Objective: ") || l.starts_with("  - ")));
        assert_eq!(lines[levels + 1], SCORED_FOR);
        let block = enemy_block(r.text());
        assert_eq!(block.len(), 1 + r.enemy().len());
        assert_eq!(block.len(), lines.len() - levels - 1, "nothing after them");
        let enemy = block[1..].join("\n");
        assert!(enemy.contains("your ships sunk"), "{enemy}");
        assert!(enemy.contains("your aircraft destroyed"), "{enemy}");
        assert!(enemy.contains("your men lost"), "{enemy}");
        assert!(
            enemy.contains("base:1 \"Lunga\" is the enemy's at the end"),
            "{enemy}"
        );
        for banned in [
            "enemy ships sunk",
            "you unload",
            "enemy aircraft",
            "enemy men",
        ] {
            assert!(!enemy.contains(banned), "{banned}: {enemy}");
        }
        match side {
            A => assert!(enemy.contains("days base:1 \"Lunga\" is shut"), "{enemy}"),
            J => assert!(
                enemy.contains("tons of supply and fuel the enemy unloads at base:1 \"Lunga\""),
                "{enemy}"
            ),
        }
    }
}

// ---------------------------------------------------------------------
// 7. A scenario with no objectives shows none.
// ---------------------------------------------------------------------

#[test]
fn s7_a_scenario_with_no_objectives_shows_none() {
    let mut s = scripted(T);
    for day in 0..=1 {
        to_day(&mut s, day);
        for side in [A, J] {
            let r = read(&mut s, side);
            assert_eq!(
                r.objectives(),
                &json!({"mission": null, "objectives": [], "levels": null, "result": null}),
                "the part has the four fields it had and no fifth"
            );
            assert_eq!(
                objectives_lines(r.text()),
                ["- none: this scenario gives no mission and no objectives"]
            );
            assert!(!r.text().contains("scored for"));
            assert_eq!(r.data["version"], sim_agent::VERSION);
        }
    }
}

// ---------------------------------------------------------------------
// 8. A seat at a command is shown none.
// ---------------------------------------------------------------------

#[test]
fn s8_a_seat_at_a_command_is_shown_none() {
    let post = Post::At(CommandRef::TaskForce(TaskForceId::new(1)));
    let mut s = at_post(CL, A, post);
    let data = call(&mut s, A, "get_briefing", json!({"text": false}));
    let text = call(&mut s, A, "get_briefing", json!({"text": true}));
    let objectives = &data["objectives"];
    assert_eq!(objectives["not_given_to_this_seat"], true);
    assert_eq!(objectives["objectives"], json!([]));
    assert!(objectives.get("enemy_objectives").is_none(), "{objectives}");
    let words = text["text"].as_str().expect("text");
    assert!(!words.contains("scored for"), "{words}");
    // The area seat of the same game is shown them.
    let mut area = at_post(CL, A, Post::Area);
    let data = call(&mut area, A, "get_briefing", json!({"text": false}));
    assert!(!data["objectives"]["enemy_objectives"]
        .as_array()
        .expect("listed at the area seat")
        .is_empty());
}

// ---------------------------------------------------------------------
// 9. A side with objectives whose enemy has none.
// ---------------------------------------------------------------------

/// ES with the Japanese side's objective entries taken out: it keeps its
/// table and its mission.
fn es_without_japanese_objectives() -> String {
    let text = std::fs::read_to_string(path(ES)).expect("reads");
    let mut out = String::new();
    let mut skipping = false;
    for line in text.lines() {
        if line.starts_with('[') {
            skipping = line.trim() == "[[objectives.japanese.objective]]";
        }
        if !skipping {
            out.push_str(line);
            out.push('\n');
        }
    }
    assert_ne!(out, text);
    assert!(out.contains("[objectives.japanese]"));
    assert!(!out.contains("[[objectives.japanese.objective]]"));
    out
}

#[test]
fn s9_a_side_whose_enemy_has_no_objectives_is_told_so() {
    let mut s = scripted_text(ES, &es_without_japanese_objectives());
    let r = read(&mut s, A);
    assert_eq!(r.enemy(), &Vec::<Value>::new(), "the part is there, empty");
    let lines = objectives_lines(r.text());
    let at = lines
        .iter()
        .position(|l| l.starts_with(SCORED_FOR))
        .expect("the line");
    assert_eq!(
        lines[at],
        "- The enemy is scored for: nothing, this scenario gives the enemy no objectives"
    );
    assert_eq!(at, lines.len() - 1, "one line");
    // The Japanese side, with no objective of its own, is shown the Allied
    // side's.
    let r = read(&mut s, J);
    assert_eq!(r.objectives()["objectives"], json!([]));
    assert_eq!(r.enemy().len(), objectives_of(ES, A).0.len());
}

// ---------------------------------------------------------------------
// 10. Nothing else changed.
// ---------------------------------------------------------------------

/// The daily hashes and the result lines of the run baseline `sim-cli run`
/// wrote for the scenario before this record (a generated file, which
/// `sh scripts/regenerate.sh --check` holds to the code).
fn baseline(name: &str) -> (Vec<String>, Vec<String>) {
    let stem = name.trim_end_matches(".toml");
    let text = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../crates/sim-cli/tests/run_baseline")
            .join(format!("{stem}.txt")),
    )
    .expect("the baseline reads");
    let hashes = text
        .lines()
        .filter(|l| l.starts_with("day "))
        .map(|l| {
            let words: Vec<&str> = l.split_whitespace().collect();
            assert_eq!(words[2], "hash");
            words[3].to_string()
        })
        .collect();
    let results = text
        .lines()
        .filter(|l| {
            (l.starts_with("Allied: ") || l.starts_with("Japanese: ")) && l.contains(" points (")
        })
        .map(str::to_string)
        .collect();
    (hashes, results)
}

/// The game played to its end with the script's orders for both sides; with
/// `reading`, every briefing of both seats read on every evening.
fn played(name: &str, reading: bool) -> (Vec<u64>, String, bool, World) {
    let mut s = scripted(name);
    for day in 0..=s.days() {
        to_day(&mut s, day);
        if reading {
            for side in [A, J] {
                let r = read(&mut s, side);
                assert!(r.objectives()["enemy_objectives"].is_array());
            }
        }
    }
    assert!(s.is_over());
    let hashes = s.day_hashes().to_vec();
    let staff = format!("{:?}", s.staff_handed_in());
    let acted = s
        .staff_handed_in()
        .values()
        .flat_map(BTreeMap::values)
        .any(|d| !d.orders.is_empty());
    (hashes, staff, acted, s.into_world())
}

#[test]
fn s10_nothing_else_changed() {
    for name in [ES, FS] {
        let (hashes, staff, acted, world) = played(name, true);
        let (unread_hashes, unread_staff, _, unread_world) = played(name, false);
        // The daily hashes are what they were.
        let (before, results) = baseline(name);
        assert_eq!(results.len(), 2, "{name}: both sides' results");
        let now: Vec<String> = hashes.iter().map(|h| format!("{h:016x}")).collect();
        assert_eq!(now, before, "{name}");
        assert_eq!(hashes, unread_hashes);
        // The staffs give the orders they gave.
        assert_eq!(staff, unread_staff, "{name}");
        // Neither scenario gives a commander's intent, so no staff acts in
        // either: the record compared is empty, as it was.
        assert!(!acted, "{name}: {staff}");
        // The result at the end is what it was.
        let result = world.result().expect("a result");
        assert_eq!(Some(&result), unread_world.result().as_ref());
        for (side, line) in [(A, &results[0]), (J, &results[1])] {
            let r = result.side(side);
            assert!(
                line.contains(&format!(" {} points (", r.total)),
                "{name}: {line}"
            );
            assert!(
                line.contains(&format!("margin {}, {}", r.margin, r.outcome.name())),
                "{name}: {line}"
            );
        }
    }
}

/// Statement 10's staffs, where they act: CL, with command levels and the
/// scenario's commands, has staffs that give orders; read or not, they give
/// the same orders, the days hash the same and the result is the same.
#[test]
fn s10_where_the_staffs_act_they_give_the_orders_they_gave() {
    let (hashes, staff, acted, world) = played(CL, true);
    let (unread_hashes, unread_staff, _, unread_world) = played(CL, false);
    assert!(acted, "the staffs act on CL: {staff}");
    assert_eq!(staff, unread_staff);
    assert_eq!(hashes, unread_hashes);
    assert_eq!(world.result(), unread_world.result());
    assert!(world.result().is_some());
}

// ---------------------------------------------------------------------
// 11. The answers are checked against their schemas.
// ---------------------------------------------------------------------

#[test]
fn s11_the_answers_are_checked_against_their_schemas() {
    let schemas = schemas();
    let schema: Value = serde_json::from_str(
        &std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../crates/sim-agent/schemas/briefing.schema.json"),
        )
        .expect("reads"),
    )
    .expect("parses");
    assert_eq!(
        schema["$defs"]["objectives"]["properties"]["enemy_objectives"]["items"]["$ref"],
        "#/$defs/enemy_objective"
    );
    for name in [FS, ES, T] {
        let mut s = scripted(name);
        for day in 0..=1 {
            to_day(&mut s, day);
            for side in [A, J] {
                let r = read(&mut s, side);
                valid(&schemas, &r, &format!("{name} day {day} {side:?}"));
            }
        }
    }
    let mut s = scripted_text(ES, &es_without_japanese_objectives());
    for side in [A, J] {
        let r = read(&mut s, side);
        valid(
            &schemas,
            &r,
            &format!("ES without Japanese objectives {side:?}"),
        );
    }
    let mut s = at_post(CL, A, Post::At(CommandRef::TaskForce(TaskForceId::new(1))));
    let r = Read {
        data: call(&mut s, A, "get_briefing", json!({"text": false})),
        text_answer: call(&mut s, A, "get_briefing", json!({"text": true})),
    };
    valid(&schemas, &r, "CL at a task force");
}
