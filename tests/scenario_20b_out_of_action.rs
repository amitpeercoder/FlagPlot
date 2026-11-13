//! Scenario 20b: ships out of action on the objectives scale (decision 0088).
//!
//! THE HAND CALCULATION, written by the architect on Nov 13, 2026, before any
//! code. Nobody else changes an expected number here; a mismatch goes back to
//! the architect. Derived quantities are written cut to six places, as the
//! engine's fixed point cuts them.
//!
//! # The rule
//!
//! A ship's days out = her days of repair at the end (decision 0069's yard
//! function, without the passage; 0 if she is at the yard at the end)
//! + the days of repair she left with each time she left for a yard.
//! Her share = min(days out, horizon) / horizon. Her points = her kind's
//! points x her share. A ship sunk is counted as sunk and not here.
//! The horizon is the scenario's days unless `horizon_days` states a longer.
//!
//! # Figures
//!
//! The frozen figures, with decision 0069's five at their placeholders:
//! yard days 2 for a point of flotation damage, 5 for a system degraded,
//! 30 for a system out; `forward_repair_max_flotation` 10;
//! `forward_repair_system_points` 10. Days of repair =
//! ceil(2 x flotation + 5 x degraded + 30 x out). Fire adds nothing.
//!
//! # The scenario, as written
//!
//! 6 days. No fight: the two sides never meet and nothing searches. A
//! placeholder map; every force is stopped for the whole game unless it is
//! sent to the yard.
//!
//! Japanese bases: base 1, a port for the yard with a passage of 1 day and
//! no repair figure; base 2, which mends 1 point a day and is not a port;
//! base 3, with neither. Allied base 4, with neither.
//!
//! | Ship | Side | Kind | Force, at | Flotation | Systems not working |
//! | --- | --- | --- | --- | --- | --- |
//! | 1 | Japanese | carrier | 1, base 3 | 12 | flight deck degraded |
//! | 2 | Japanese | cruiser | 1, base 3 | 0 | propulsion out |
//! | 3 | Japanese | cruiser | 1, base 3 | 40 | none |
//! | 4 | Japanese | cruiser | 1, base 3 | 6 | fire control degraded |
//! | 5 | Japanese | destroyer | 2, base 1 | 1 | none |
//! | 6 | Japanese | destroyer | 3, base 1 | 3.25 | torpedo mounts degraded |
//! | 7 | Japanese | destroyer | 1, base 3 | 0 | none |
//! | 8 | Japanese | cruiser | 4, base 2 | 8 | none |
//! | 9 | Allied | battleship | 5, base 4 | 20 | main battery out |
//!
//! No ship has fire damage.
//!
//! Orders: day 2, force 2, `send_to_yard`. Day 4, force 3, `send_to_yard`.
//!
//! Objectives. Levels 3, 8, 16. `horizon_days = 60`.
//! Allied: `ships_sunk` (carrier 10, battleship 8, cruiser 4, destroyer 1)
//! and `ships_out_of_action` (carrier 5, battleship 4, cruiser 2,
//! destroyer 0.5). Japanese: the same two.
//!
//! # What happens
//!
//! - Force 2, day 2: ship 5's days of repair are ceil(2 x 1) = 2. Days away
//!   1 + 2 = 3. She is back on day 5, whole. The event gives her 2.
//! - Force 3, day 4: ship 6's days of repair are ceil(6.5 + 5) =
//!   ceil(11.5) = 12. Days away 13; the day of return would be 17, after
//!   day 6, so she never returns and ends at the yard. The event gives her 12.
//! - Force 4 at base 2: forward repair of 1 point at the start of each of
//!   days 1 to 6 (after the day's orders). Ship 8's flotation goes
//!   8, 7, 6, 5, 4, 3, 2. She ends with 2.
//! - Nothing else changes.
//!
//! # Days out at the end
//!
//! | Ship | At the end | Left with | Days out |
//! | --- | --- | --- | --- |
//! | 1 | ceil(24 + 5) = 29 | 0 | 29 |
//! | 2 | 30 | 0 | 30 |
//! | 3 | 80 | 0 | 80 |
//! | 4 | ceil(12 + 5) = 17 | 0 | 17 |
//! | 5 | 0 (whole) | 2 | 2 |
//! | 6 | 0 (at the yard: not counted twice) | 12 | 12 |
//! | 7 | 0 | 0 | 0: not listed |
//! | 8 | ceil(2 x 2) = 4 | 0 | 4 |
//! | 9 | ceil(40 + 30) = 70 | 0 | 70 |
//!
//! # Case 1: the horizon stated, 60 days
//!
//! | Ship | Share | Points |
//! | --- | --- | --- |
//! | 1 | 29 / 60 = 0.483333 | 5 x 0.483333 = 2.416665 |
//! | 2 | 30 / 60 = 0.5 | 2 x 0.5 = 1 |
//! | 3 | min(80, 60) / 60 = 1 | 2 |
//! | 4 | 17 / 60 = 0.283333 | 2 x 0.283333 = 0.566666 |
//! | 5 | 2 / 60 = 0.033333 | 0.5 x 0.033333 = 0.016666 |
//! | 6 | 12 / 60 = 0.2 | 0.5 x 0.2 = 0.1 |
//! | 8 | 4 / 60 = 0.066666 | 2 x 0.066666 = 0.133332 |
//! | 9 (for the Japanese) | min(70, 60) / 60 = 1 | 4 |
//!
//! Allied: out of action 7 ships (carrier 1, cruiser 4, destroyer 2),
//! 2.416665 + 1 + 2 + 0.566666 + 0.016666 + 0.1 + 0.133332 = 6.233329.
//! Sunk 0. Total 6.233329.
//! Japanese: out of action 1 ship (battleship), 4. Sunk 0. Total 4.
//! Allied margin 2.233329: below the minor level of 3, a stalemate.
//!
//! # Case 2: no horizon stated, so 6 days
//!
//! Every ship with 6 days out or more has a share of 1.
//!
//! | Ship | Share | Points |
//! | --- | --- | --- |
//! | 1 | 1 | 5 |
//! | 2 | 1 | 2 |
//! | 3 | 1 | 2 |
//! | 4 | 1 | 2 |
//! | 5 | 2 / 6 = 0.333333 | 0.5 x 0.333333 = 0.166666 |
//! | 6 | 1 | 0.5 |
//! | 8 | 4 / 6 = 0.666666 | 2 x 0.666666 = 1.333332 |
//! | 9 | 1 | 4 |
//!
//! Allied 5 + 2 + 2 + 2 + 0.166666 + 0.5 + 1.333332 = 12.999998.
//! Japanese 4. Allied margin 8.999998: at least the major level of 8 and
//! below the decisive 16, a major Allied victory.
//!
//! # Case 3: the horizon stated, 1,300 days
//!
//! | Ship | Share | Points |
//! | --- | --- | --- |
//! | 1 | 29 / 1300 = 0.022307 | 5 x 0.022307 = 0.111535 |
//! | 2 | 30 / 1300 = 0.023076 | 2 x 0.023076 = 0.046152 |
//! | 3 | 80 / 1300 = 0.061538 | 2 x 0.061538 = 0.123076 |
//! | 4 | 17 / 1300 = 0.013076 | 2 x 0.013076 = 0.026152 |
//! | 5 | 2 / 1300 = 0.001538 | 0.5 x 0.001538 = 0.000769 |
//! | 6 | 12 / 1300 = 0.009230 | 0.5 x 0.009230 = 0.004615 |
//! | 8 | 4 / 1300 = 0.003076 | 2 x 0.003076 = 0.006152 |
//! | 9 | 70 / 1300 = 0.053846 | 4 x 0.053846 = 0.215384 |
//!
//! Allied 0.111535 + 0.046152 + 0.123076 + 0.026152 + 0.000769 + 0.004615
//! + 0.006152 = 0.318451. Japanese 0.215384. Allied margin 0.103067: a
//! stalemate.
//!
//! # Refusals of the loader (each a variant of the scenario file)
//!
//! - `horizon_days = 5` with `days = 6`: refused.
//! - `horizon_days = 0`: refused.
//! - The Japanese side given `ships_crippled` beside the Allied
//!   `ships_out_of_action`: refused.
//!
//! # Unit tests, for what the scenario cannot reach (in `objectives.rs`)
//!
//! - A ship sunk after a yard trip is counted as sunk and not listed out of
//!   action.
//! - A ship that left with 45 days, came back whole and ends with flotation
//!   3 has 45 + 6 = 51 days out; at a horizon of 60 a cruiser of 2 points
//!   scores 2 x 0.85 = 1.7.
//! - A ship that left twice, with 20 and 50 days, has 70 days out and a
//!   share of 1 at a horizon of 60.
//! - `known_progress` gives nothing for the objective, on the basis it gives
//!   for `ships_crippled`.
//!
//! # Properties
//!
//! - The result is the same for every seed (seeds 1 to 50): the rule and the
//!   scenario make no draw that bears on it.
//! - A side's points for the objective never exceed the sum of its points
//!   for each enemy ship afloat.
//! - Every scenario that does not use the objective has the result, the
//!   event log and the hashes it had before.

// The architect's doc comment above has lines that begin with "+" and "=",
// which rustdoc reads as list items; it is left as written.
#![allow(clippy::doc_lazy_continuation)]

// "Every other scenario's result, event log and hash as before" is shown by
// the run baselines (`crates/sim-cli/tests/run_baseline`) and the golden
// hashes, which this file does not repeat. The "Unit tests" above are in
// `crates/sim-core/src/objectives.rs`; the loader's own are in
// `crates/sim-data/src/scenario.rs`.

mod common;

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::ids::{BaseId, ShipId, TaskForceId};
use sim_core::objectives::{tallies, GameResult, Level, ObjectiveScore, Outcome, Tally};
use sim_core::refdata::ShipKind;
use sim_core::state::{Ship, ShipLocation, ShipSystem, Side, SystemState};
use sim_data::Scenario;

fn path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/20b_out_of_action.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn load() -> Scenario {
    common::load(&path()).expect("scenario 20b loads")
}

fn run() -> Scenario {
    let mut scenario = load();
    scenario.run_to_end().expect("the scenario runs");
    scenario
}

/// The scenario's text with one piece replaced by another. Panics unless the
/// piece appears exactly once.
fn with(original: &str, changed: &str) -> String {
    common::replace_once(&text(), original, changed)
}

/// The variant loaded on the frozen figures and run to the end.
fn run_text(source: &str) -> Scenario {
    let mut scenario = common::from_text(source, &path()).expect("the variant loads");
    scenario.run_to_end().expect("the variant runs");
    scenario
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

fn sh(n: u32) -> ShipId {
    ShipId::new(n)
}

fn tf(n: u32) -> TaskForceId {
    TaskForceId::new(n)
}

fn events_of(scenario: &Scenario) -> Vec<Event> {
    scenario.world.events().iter().cloned().collect()
}

fn ship(scenario: &Scenario, n: u32) -> &Ship {
    &scenario.world.umpire().ships[&sh(n)]
}

fn all_working(ship: &Ship) -> bool {
    ShipSystem::ALL
        .iter()
        .all(|system| ship.systems.get(*system) == SystemState::Working)
}

fn left_for_yard(scenario: &Scenario, force: u32) -> Event {
    events_of(scenario)
        .into_iter()
        .find(|event| {
            matches!(&event.kind, EventKind::ForceLeftForYard { task_force, .. } if *task_force == tf(force))
        })
        .expect("the force leaves for the yard")
}

fn result_of(scenario: &Scenario) -> GameResult {
    scenario
        .world
        .result()
        .expect("the scenario has objectives")
}

/// The `ships_out_of_action` line of a side's result.
fn out_line(result: &GameResult, side: Side) -> &ObjectiveScore {
    let line = &result.side(side).objectives[1];
    assert_eq!(line.kind, "ships_out_of_action");
    line
}

fn tallies_of(scenario: &Scenario) -> BTreeMap<Side, Tally> {
    let world = &scenario.world;
    tallies(world.umpire(), world.events(), world.ref_data())
}

// ------------------------------------------------------- What happens

#[test]
fn what_happens_two_forces_go_to_the_yard_and_one_comes_back() {
    let scenario = run();
    let two = left_for_yard(&scenario, 2);
    assert_eq!(two.day, 2);
    assert_eq!(
        two.kind,
        EventKind::ForceLeftForYard {
            task_force: tf(2),
            base: BaseId::new(1),
            passage_days: 1,
            ships: vec![(sh(5), 2)],
            days_away: 3,
            return_day: 5,
        }
    );
    let three = left_for_yard(&scenario, 3);
    assert_eq!(three.day, 4);
    assert_eq!(
        three.kind,
        EventKind::ForceLeftForYard {
            task_force: tf(3),
            base: BaseId::new(1),
            passage_days: 1,
            ships: vec![(sh(6), 12)],
            days_away: 13,
            return_day: 17,
        }
    );
    // Force 2 is back on day 5; force 3 never returns.
    let returned: Vec<(u32, TaskForceId)> = events_of(&scenario)
        .into_iter()
        .filter_map(|event| match event.kind {
            EventKind::ForceReturnedFromYard { task_force, .. } => Some((event.day, task_force)),
            _ => None,
        })
        .collect();
    assert_eq!(returned, vec![(5, tf(2))]);
    // Only those two forces left.
    let departures = events_of(&scenario)
        .iter()
        .filter(|event| matches!(event.kind, EventKind::ForceLeftForYard { .. }))
        .count();
    assert_eq!(departures, 2);
}

#[test]
fn what_happens_ship_five_is_back_whole_and_ship_six_is_at_the_yard() {
    let scenario = run();
    let five = ship(&scenario, 5);
    assert_eq!(five.location, ShipLocation::TaskForce(tf(2)));
    assert_eq!(five.flotation_damage, Fixed::ZERO);
    assert!(all_working(five));
    let six = ship(&scenario, 6);
    assert_eq!(six.location, ShipLocation::Yard(tf(3)));
    // She keeps what she left with.
    assert_eq!(six.flotation_damage, fx("3.25"));
    assert_eq!(
        six.systems.get(ShipSystem::TorpedoMounts),
        SystemState::Degraded
    );
}

#[test]
fn what_happens_forward_repair_takes_ship_eight_to_flotation_two_and_nothing_else_changes() {
    let scenario = run();
    assert_eq!(ship(&scenario, 8).flotation_damage, fx("2"));
    assert!(all_working(ship(&scenario, 8)));
    // The others as they began: flotation and the one system not working.
    type NotWorking = Option<(ShipSystem, SystemState)>;
    let as_begun: [(u32, &str, NotWorking); 5] = [
        (
            1,
            "12",
            Some((ShipSystem::FlightDeck, SystemState::Degraded)),
        ),
        (2, "0", Some((ShipSystem::Propulsion, SystemState::Out))),
        (3, "40", None),
        (
            4,
            "6",
            Some((ShipSystem::FireControl, SystemState::Degraded)),
        ),
        (7, "0", None),
    ];
    for (n, flotation, system) in as_begun {
        let ship = ship(&scenario, n);
        assert_eq!(ship.flotation_damage, fx(flotation), "ship {n}");
        assert_eq!(ship.fire_damage, Fixed::ZERO, "ship {n}");
        assert_eq!(ship.location, ShipLocation::TaskForce(tf(1)), "ship {n}");
        match system {
            Some((which, state)) => {
                assert_eq!(ship.systems.get(which), state, "ship {n}");
                let others = ShipSystem::ALL
                    .iter()
                    .filter(|other| **other != which)
                    .all(|other| ship.systems.get(*other) == SystemState::Working);
                assert!(others, "ship {n}");
            }
            None => assert!(all_working(ship), "ship {n}"),
        }
    }
    let nine = ship(&scenario, 9);
    assert_eq!(nine.flotation_damage, fx("20"));
    assert_eq!(nine.systems.get(ShipSystem::MainBattery), SystemState::Out);
    // No ship was sunk and none has fire.
    assert!(scenario
        .world
        .umpire()
        .ships
        .values()
        .all(|ship| ship.location != ShipLocation::Sunk && ship.fire_damage == Fixed::ZERO));
}

// ------------------------------------------------- Days out at the end

#[test]
fn the_tally_has_each_ships_days_at_the_end_and_left_with_as_the_table_says() {
    let scenario = run();
    let tallies = tallies_of(&scenario);
    // A side's tally holds the enemy's ships: the Japanese ships are in the
    // Allied tally and the Allied battleship in the Japanese.
    let allied: Vec<(u32, ShipKind, u32, u32, u32)> = tallies[&Side::Allied]
        .out_of_action
        .iter()
        .map(|(id, line)| {
            (
                id.raw(),
                line.kind,
                line.at_end,
                line.left_with,
                line.days_out(),
            )
        })
        .collect();
    assert_eq!(
        allied,
        vec![
            (1, ShipKind::Carrier, 29, 0, 29),
            (2, ShipKind::Cruiser, 30, 0, 30),
            (3, ShipKind::Cruiser, 80, 0, 80),
            (4, ShipKind::Cruiser, 17, 0, 17),
            (5, ShipKind::Destroyer, 0, 2, 2),
            (6, ShipKind::Destroyer, 0, 12, 12),
            // Ship 7 is whole, never left, and is not listed.
            (8, ShipKind::Cruiser, 4, 0, 4),
        ]
    );
    let japanese: Vec<(u32, ShipKind, u32, u32, u32)> = tallies[&Side::Japanese]
        .out_of_action
        .iter()
        .map(|(id, line)| {
            (
                id.raw(),
                line.kind,
                line.at_end,
                line.left_with,
                line.days_out(),
            )
        })
        .collect();
    assert_eq!(japanese, vec![(9, ShipKind::Battleship, 70, 0, 70)]);
    // Nothing is sunk, and the objective does not use `crippled`.
    assert!(tallies[&Side::Allied].sunk.is_empty());
    assert!(tallies[&Side::Japanese].sunk.is_empty());
}

// ------------------------------------------------------------- Cases

/// A ship's line: number, share, points.
type Line = (u32, &'static str, &'static str);

/// Asserts a case: the ships listed for each side with their shares and
/// points, the objective's count, kinds and points, each side's sunk points,
/// totals, margins and outcomes.
fn assert_case(
    result: &GameResult,
    horizon: u32,
    allied_lines: &[Line],
    allied_total: &str,
    japanese_total: &str,
    margin: &str,
    outcome: Outcome,
) {
    let allied = out_line(result, Side::Allied);
    assert_eq!(allied.horizon_days, Some(horizon));
    let listed: Vec<(u32, Fixed, Fixed)> = allied
        .ships_out
        .iter()
        .map(|out| (out.ship.raw(), out.share, out.points))
        .collect();
    let expected: Vec<(u32, Fixed, Fixed)> = allied_lines
        .iter()
        .map(|(n, share, points)| (*n, fx(share), fx(points)))
        .collect();
    assert_eq!(listed, expected);
    assert_eq!(allied.count, Fixed::from_int(7));
    assert_eq!(
        allied.by_kind,
        BTreeMap::from([
            (ShipKind::Carrier, 1),
            (ShipKind::Cruiser, 4),
            (ShipKind::Destroyer, 2),
        ])
    );
    assert_eq!(allied.points, fx(allied_total));
    assert_eq!(result.allied.objectives[0].kind, "ships_sunk");
    assert_eq!(result.allied.objectives[0].points, Fixed::ZERO);
    assert_eq!(result.allied.total, fx(allied_total));

    let japanese = out_line(result, Side::Japanese);
    assert_eq!(japanese.horizon_days, Some(horizon));
    assert_eq!(japanese.count, Fixed::ONE);
    assert_eq!(
        japanese.by_kind,
        BTreeMap::from([(ShipKind::Battleship, 1)])
    );
    assert_eq!(result.japanese.objectives[0].points, Fixed::ZERO);
    assert_eq!(result.japanese.total, fx(japanese_total));
    assert_eq!(japanese.points, fx(japanese_total));

    assert_eq!(result.allied.margin, fx(margin));
    assert_eq!(result.japanese.margin, Fixed::ZERO - fx(margin));
    assert_eq!(result.allied.outcome, outcome);
    assert_eq!(result.japanese.outcome, outcome.mirror());
}

/// The Japanese side's one line: ship 9 at 70 days out.
fn assert_ship_nine(result: &GameResult, horizon: u32, share: &str, points: &str) {
    let line = out_line(result, Side::Japanese);
    assert_eq!(line.ships_out.len(), 1);
    let out = line.ships_out[0];
    assert_eq!(
        (
            out.ship,
            out.kind,
            out.at_end,
            out.left_with,
            out.days_out,
            out.share,
            out.points
        ),
        (
            sh(9),
            ShipKind::Battleship,
            70,
            0,
            70,
            fx(share),
            fx(points)
        )
    );
    assert_eq!(line.horizon_days, Some(horizon));
}

#[test]
fn case_1_the_horizon_stated_at_60_days() {
    let scenario = run();
    let result = result_of(&scenario);
    assert_case(
        &result,
        60,
        &[
            (1, "0.483333", "2.416665"),
            (2, "0.5", "1"),
            (3, "1", "2"),
            (4, "0.283333", "0.566666"),
            (5, "0.033333", "0.016666"),
            (6, "0.2", "0.1"),
            (8, "0.066666", "0.133332"),
        ],
        "6.233329",
        "4",
        "2.233329",
        Outcome::Stalemate,
    );
    assert_ship_nine(&result, 60, "1", "4");
    // The days behind each line, as the table gives them.
    let days: Vec<(u32, u32, u32, u32)> = out_line(&result, Side::Allied)
        .ships_out
        .iter()
        .map(|out| (out.ship.raw(), out.at_end, out.left_with, out.days_out))
        .collect();
    assert_eq!(
        days,
        vec![
            (1, 29, 0, 29),
            (2, 30, 0, 30),
            (3, 80, 0, 80),
            (4, 17, 0, 17),
            (5, 0, 2, 2),
            (6, 0, 12, 12),
            (8, 4, 0, 4),
        ]
    );
}

#[test]
fn case_2_no_horizon_stated_so_the_game_is_6_days() {
    let source = with("horizon_days = 60\n", "");
    let scenario = run_text(&source);
    assert_eq!(
        scenario
            .world
            .objectives()
            .expect("objectives")
            .horizon_days,
        6
    );
    let result = result_of(&scenario);
    assert_case(
        &result,
        6,
        &[
            (1, "1", "5"),
            (2, "1", "2"),
            (3, "1", "2"),
            (4, "1", "2"),
            (5, "0.333333", "0.166666"),
            (6, "1", "0.5"),
            (8, "0.666666", "1.333332"),
        ],
        "12.999998",
        "4",
        "8.999998",
        Outcome::Victory(Level::Major),
    );
    assert_ship_nine(&result, 6, "1", "4");
}

#[test]
fn case_3_the_horizon_stated_at_1300_days() {
    let source = with("horizon_days = 60", "horizon_days = 1300");
    let scenario = run_text(&source);
    let result = result_of(&scenario);
    assert_case(
        &result,
        1300,
        &[
            (1, "0.022307", "0.111535"),
            (2, "0.023076", "0.046152"),
            (3, "0.061538", "0.123076"),
            (4, "0.013076", "0.026152"),
            (5, "0.001538", "0.000769"),
            (6, "0.009230", "0.004615"),
            (8, "0.003076", "0.006152"),
        ],
        "0.318451",
        "0.215384",
        "0.103067",
        Outcome::Stalemate,
    );
    assert_ship_nine(&result, 1300, "0.053846", "0.215384");
}

// ---------------------------------------------- Refusals of the loader

fn refusal_of(source: &str) -> String {
    common::from_text(source, &path())
        .expect_err("the variant is refused")
        .to_string()
}

#[test]
fn a_horizon_shorter_than_the_game_is_refused() {
    let message = refusal_of(&with("horizon_days = 60", "horizon_days = 5"));
    assert!(message.contains("horizon_days 5"), "{message}");
    assert!(message.contains("shorter than"), "{message}");
}

#[test]
fn a_horizon_of_zero_is_refused() {
    let message = refusal_of(&with("horizon_days = 60", "horizon_days = 0"));
    assert!(message.contains("horizon_days 0"), "{message}");
}

#[test]
fn the_japanese_side_given_ships_crippled_beside_the_allied_out_of_action_is_refused() {
    let source = with(
        "[[objectives.japanese.objective]]\nkind = \"ships_out_of_action\"",
        "[[objectives.japanese.objective]]\nkind = \"ships_crippled\"",
    );
    let message = refusal_of(&source);
    assert!(message.contains("ships_crippled"), "{message}");
    assert!(message.contains("0088"), "{message}");
}

// -------------------------------------------------------- Properties

#[test]
fn property_the_result_is_the_same_for_seeds_1_to_50() {
    let base = load();
    let mut reference = base.with_seed(1).expect("a seed");
    reference.run_to_end().expect("runs");
    let expected = result_of(&reference);
    for seed in 1..=50u64 {
        let mut scenario = base.with_seed(seed).expect("a seed");
        scenario.run_to_end().expect("runs");
        assert_eq!(result_of(&scenario), expected, "seed {seed}");
    }
}

/// For each side, its out-of-action points never exceed the sum of its kind
/// points over each enemy ship afloat at the end.
fn assert_points_within_the_ships_afloat(scenario: &Scenario) {
    let world = &scenario.world;
    let result = result_of(scenario);
    let objectives = world.objectives().expect("objectives");
    for side in Side::ALL {
        let points = match &objectives.side(side).objectives[1] {
            sim_core::objectives::Objective::ShipsOutOfAction(points) => points,
            other => panic!("{other:?}"),
        };
        let afloat: Fixed = world
            .umpire()
            .ships
            .values()
            .filter(|ship| ship.side != side && ship.location != ShipLocation::Sunk)
            .map(|ship| points.of(world.ref_data().ship_classes[&ship.class].kind))
            .fold(Fixed::ZERO, |sum, value| sum + value);
        let scored = out_line(&result, side);
        assert!(
            scored.points <= afloat,
            "{side:?}: {} above {afloat}",
            scored.points
        );
        // And no ship's share is above 1.
        assert!(scored.ships_out.iter().all(|out| out.share <= Fixed::ONE));
    }
}

#[test]
fn property_out_of_action_points_never_exceed_the_points_of_the_ships_afloat() {
    assert_points_within_the_ships_afloat(&run());
    assert_points_within_the_ships_afloat(&run_text(&with("horizon_days = 60\n", "")));
    assert_points_within_the_ships_afloat(&run_text(&with(
        "horizon_days = 60",
        "horizon_days = 1300",
    )));
}
