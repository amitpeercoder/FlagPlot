//! Objectives and results (decision 0054), on scenario 5, Eastern Solomons.
//!
//! # Hand calculation
//!
//! Written by the architect before the test code. The tests below assert
//! these numbers. If the code disagrees, the code or the rule is wrong: do
//! not change a number here to make a test pass.
//!
//! The scenario is `data/scenarios/05_eastern_solomons.toml`, seed 1942, as
//! it runs today. Nothing of the battle changes: the objectives only read
//! what happened. What happened, from the event log of that run. (Brought to
//! the scenario's new orders on Nov 12, 2026, after decisions 0059 and 0063:
//! Ryujo is in (7,-9), her raid on Henderson Field flies, and more American
//! groups reach her. The counts below are read from the events of the run, as
//! they were; the arithmetic is shown.)
//!
//! | Fact | Count |
//! | ---- | ----- |
//! | Japanese ships sunk | 1: ship 33, Ryujo, a carrier |
//! | Japanese ships afloat with propulsion or flight deck out | 0 |
//! | Allied ships sunk | 0 |
//! | Allied ships afloat with propulsion or flight deck out | 1: ship 1, Enterprise, a carrier, flight deck out |
//! | Japanese aircraft lost | shot down: group 11: 3; group 12: 10 and 5; group 14: 1; group 15: 8 and 4; group 18: 2 (over Henderson Field, by VMF-223). Lost with Ryujo: group 18: 0 and group 19: 0, since every flyable aircraft of both was in the air when she sank (decision 10 of 0059). Ran out of fuel on the way home from the raid: group 19: 3 (decision 3 of 0059). Came down in the sea for want of a deck: group 18: 21 (23 less 2 shot down) and group 19: 3 (6 less 3 ditched). 3 + 15 + 1 + 12 + 2 = 33 shot down; 33 + 0 + 3 + 21 + 3 = 60 |
//! | Allied aircraft lost | shot down: group 2: 1; group 6: 1; group 10: 1; group 9: 2 (over Henderson Field, by Ryujo's escorts); group 1: 2 and 1; and 3 of group 1 that came down in the sea when Enterprise's flight deck went out (21 of group 1 landed on ship 11, 16 and 16 of groups 2 and 3 and 14 of group 4 at base 1: a landing is no loss). 1 + 1 + 1 + 2 + 3 = 8 shot down, and 8 + 3 = 11 |
//!
//! Both sides hold the same three objectives on the placeholder scale:
//! ships sunk (carrier 10, battleship 8, cruiser 4, destroyer 1, transport
//! 2, auxiliary 1), ships crippled (half of each), and aircraft destroyed
//! at 0.05 each. Levels: minor 3, major 8, decisive 16.
//!
//! | Objective | Allied count | Allied points | Japanese count | Japanese points |
//! | --------- | ------------ | ------------- | -------------- | --------------- |
//! | Ships sunk | 1 carrier | 10 | 0 | 0 |
//! | Ships crippled | 0 | 0 | 1 carrier | 5 |
//! | Aircraft destroyed | 60 | 60 x 0.05 = 3 | 11 | 11 x 0.05 = 0.55 |
//! | Total | | 10 + 0 + 3 = 13 | | 0 + 5 + 0.55 = 5.55 |
//!
//! Margin for the Allied side: 13 - 5.55 = 7.45. It is at least 3 and less
//! than 8: a minor Allied victory, and so a minor Japanese defeat, margin
//! -7.45. (Before the change the Japanese had destroyed 8 and the margin was
//! 7.6.)
//!
//! What each side knows of its progress at the end is not fixed here: it
//! depends on which events decision 0047 lets a side see, and the developer
//! reports it. Two things are fixed. A side's count of its own losses is not
//! part of its progress (they are the enemy's points). And a side's known
//! count for an objective is never above the true count.
//!
//! # Properties
//!
//! - The result is the same run twice, and the same from a saved game
//!   replayed.
//! - The two sides' margins sum to zero, and their levels mirror.
//! - The state hash of every day is what it was before objectives existed
//!   (the golden files under `crates/sim-cli/tests/run_baseline` do not
//!   change).
//! - Sinking one more enemy ship, or losing one fewer of one's own, never
//!   lowers a side's margin (checked on the scoring function with made-up
//!   counts).
//! - Over seeds 1 to 200 the known count of each objective is never above
//!   the true count, for both sides.

mod common;

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::ids::{AirGroupId, ShipId};
use sim_core::objectives::{
    score, GameResult, KnownBasis, Level, Objective, Outcome, ShipPoints, SideResult, Tally,
};
use sim_core::refdata::ShipKind;
use sim_core::state::{ShipLocation, ShipSystem, Side, SystemState};
use sim_core::World;
use sim_data::Scenario;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
const NAME: &str = "05_eastern_solomons.toml";
const SEED: u64 = 1942;

/// The scenario the tests run on the frozen figures is the frozen copy,
/// `data/scenarios/step2-frozen/05_eastern_solomons.toml`, as decision 0073 and
/// the folder's README say: frozen figures go with frozen scenarios. Only the
/// comparison with the run baseline (`live_scenario_path`) reads the live file,
/// on the live figures.
fn scenario_path() -> PathBuf {
    PathBuf::from(ROOT)
        .join("data/scenarios/step2-frozen")
        .join(NAME)
}

fn live_scenario_path() -> PathBuf {
    PathBuf::from(ROOT).join("data/scenarios").join(NAME)
}

fn scenario_text() -> String {
    std::fs::read_to_string(scenario_path()).expect("the scenario reads")
}

fn load_text(text: &str, seed: u64) -> Scenario {
    common::from_text(text, &scenario_path())
        .expect("the scenario loads")
        .with_seed(seed)
        .expect("a seed can be set")
}

fn run(seed: u64) -> World {
    let mut scenario = load_text(&scenario_text(), seed);
    scenario.run_to_end().expect("the run resolves");
    scenario.world
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

fn n(value: i64) -> Fixed {
    Fixed::from_int(value)
}

fn result_of(world: &World) -> GameResult {
    world.result().expect("scenario 5 has objectives")
}

// ---------------------------------------------------------------------
// The facts the hand calculation starts from, read independently of the
// scoring function: from the event log and the final truth.
// ---------------------------------------------------------------------

#[test]
fn what_happened_is_what_the_hand_calculation_says() {
    let world = run(SEED);
    let truth = world.umpire();

    // Ships sunk: ship 33, Ryujo, and no other.
    let sunk: Vec<u32> = truth
        .ships
        .values()
        .filter(|s| s.location == ShipLocation::Sunk)
        .map(|s| s.id.raw())
        .collect();
    assert_eq!(sunk, vec![33]);
    let kind_of =
        |ship: u32| world.ref_data().ship_classes[&truth.ships[&ShipId::new(ship)].class].kind;
    assert_eq!(kind_of(33), ShipKind::Carrier);

    // Ships afloat with propulsion or the flight deck out: Enterprise, flight
    // deck out, and nobody else. Ryujo has both out but is sunk.
    let crippled: Vec<(u32, bool, bool)> = truth
        .ships
        .values()
        .filter(|s| s.location != ShipLocation::Sunk)
        .filter(|s| {
            s.systems.get(ShipSystem::Propulsion) == SystemState::Out
                || s.systems.get(ShipSystem::FlightDeck) == SystemState::Out
        })
        .map(|s| {
            (
                s.id.raw(),
                s.systems.get(ShipSystem::Propulsion) == SystemState::Out,
                s.systems.get(ShipSystem::FlightDeck) == SystemState::Out,
            )
        })
        .collect();
    assert_eq!(crippled, vec![(1, false, true)]);
    assert_eq!(kind_of(1), ShipKind::Carrier);
    let ryujo = &truth.ships[&ShipId::new(33)];
    assert_eq!(
        (
            ryujo.systems.get(ShipSystem::Propulsion),
            ryujo.systems.get(ShipSystem::FlightDeck)
        ),
        (SystemState::Out, SystemState::Out)
    );

    // Aircraft lost for good, by group and event.
    let mut shot_down: Vec<(u32, u32)> = Vec::new();
    let mut with_ship: Vec<(u32, u32)> = Vec::new();
    let mut no_deck: Vec<(u32, u32)> = Vec::new();
    let mut fuel: Vec<(u32, u32)> = Vec::new();
    let mut other_losses = 0;
    for event in world.events().iter() {
        match &event.kind {
            EventKind::AircraftLost {
                air_group,
                aircraft,
                ..
            } => shot_down.push((air_group.raw(), *aircraft)),
            EventKind::AirGroupLost {
                air_group,
                aircraft,
                ..
            } => with_ship.push((air_group.raw(), *aircraft)),
            EventKind::AircraftDitchedNoDeck {
                air_group,
                aircraft,
                ..
            } => no_deck.push((air_group.raw(), *aircraft)),
            EventKind::AircraftDitched {
                air_group, ditched, ..
            } => fuel.push((air_group.raw(), *ditched)),
            EventKind::AircraftDestroyedOnGround { .. }
            | EventKind::DamagedAircraftDestroyedOnGround { .. } => other_losses += 1,
            EventKind::AircraftWorn { written_off, .. } => other_losses += written_off,
            _ => {}
        }
    }
    assert_eq!(
        other_losses, 0,
        "no loss on the ground or by wear in this scenario"
    );
    let side_of = |group: u32| truth.air_groups[&AirGroupId::new(group)].side;
    let by_side = |list: &[(u32, u32)], side: Side| -> Vec<(u32, u32)> {
        let mut found: Vec<(u32, u32)> = list
            .iter()
            .copied()
            .filter(|(group, _)| side_of(*group) == side)
            .collect();
        found.sort();
        found
    };
    // Japanese, shot down: group 11: 3; 12: 10 and 5; 14: 1; 15: 8 and 4; 18:
    // 2 over Henderson Field. Lost with Ryujo: group 18: 0 and group 19: 0
    // (every flyable aircraft of both was in the air). Out of fuel: group 19:
    // 3. No deck: group 18: 21 and group 19: 3. 3 + 15 + 1 + 12 + 2 = 33;
    // 33 + 0 + 3 + 24 = 60.
    let japanese = by_side(&shot_down, Side::Japanese);
    assert_eq!(
        japanese,
        vec![
            (11, 3),
            (12, 5),
            (12, 10),
            (14, 1),
            (15, 4),
            (15, 8),
            (18, 2)
        ]
    );
    let lost_with_ryujo = by_side(&with_ship, Side::Japanese);
    assert_eq!(lost_with_ryujo, vec![(18, 0), (19, 0)]);
    let japanese_fuel = by_side(&fuel, Side::Japanese);
    assert_eq!(japanese_fuel, vec![(19, 3)]);
    let japanese_no_deck = by_side(&no_deck, Side::Japanese);
    assert_eq!(japanese_no_deck, vec![(18, 21), (19, 3)]);
    let total = |lists: &[&Vec<(u32, u32)>]| -> u32 {
        lists.iter().flat_map(|l| l.iter()).map(|(_, a)| a).sum()
    };
    assert_eq!(total(&[&japanese]), 33);
    assert_eq!(
        total(&[
            &japanese,
            &lost_with_ryujo,
            &japanese_fuel,
            &japanese_no_deck
        ]),
        60
    );
    // Allied, shot down: group 2: 1; group 6: 1; group 10: 1; group 9: 2;
    // group 1: 2 and 1; and 3 of group 1 that came down in the sea when
    // carrier 1's deck went out. 8 + 3 = 11.
    let allied = by_side(&shot_down, Side::Allied);
    assert_eq!(
        allied,
        vec![(1, 1), (1, 2), (2, 1), (6, 1), (9, 2), (10, 1)]
    );
    assert_eq!(by_side(&with_ship, Side::Allied), vec![]);
    assert_eq!(by_side(&fuel, Side::Allied), vec![]);
    let allied_no_deck = by_side(&no_deck, Side::Allied);
    assert_eq!(allied_no_deck, vec![(1, 3)]);
    assert_eq!(total(&[&allied]), 8);
    assert_eq!(total(&[&allied, &allied_no_deck]), 11);
}

// ---------------------------------------------------------------------
// The hand calculation.
// ---------------------------------------------------------------------

#[test]
fn the_result_is_what_the_hand_calculation_says() {
    let result = result_of(&run(SEED));

    let line = |side: &SideResult, i: usize| {
        let o = &side.objectives[i];
        (o.kind, o.count, o.points)
    };

    // Allied points: ships sunk 1 carrier = 10; ships crippled 0; aircraft
    // 60 x 0.05 = 3; total 13. (The 60 Japanese aircraft are the 33 shot
    // down, 3 out of fuel and 24 in the sea for want of a deck: the first
    // test sums them from the events.)
    let allied = &result.allied;
    assert_eq!(allied.objectives.len(), 3);
    assert_eq!(line(allied, 0), ("ships_sunk", n(1), n(10)));
    assert_eq!(
        allied.objectives[0].by_kind,
        BTreeMap::from([(ShipKind::Carrier, 1)])
    );
    assert_eq!(line(allied, 1), ("ships_crippled", n(0), n(0)));
    assert_eq!(line(allied, 2), ("aircraft_destroyed", n(60), n(3)));
    assert_eq!(allied.total, n(13));

    // Japanese points: ships sunk 0; ships crippled 1 carrier = 5; aircraft
    // 11 x 0.05 = 0.55; total 5.55.
    let japanese = &result.japanese;
    assert_eq!(line(japanese, 0), ("ships_sunk", n(0), n(0)));
    assert_eq!(line(japanese, 1), ("ships_crippled", n(1), n(5)));
    assert_eq!(
        japanese.objectives[1].by_kind,
        BTreeMap::from([(ShipKind::Carrier, 1)])
    );
    assert_eq!(line(japanese, 2), ("aircraft_destroyed", n(11), fx("0.55")));
    assert_eq!(japanese.total, fx("5.55"));

    // Margin 13 - 5.55 = 7.45: at least 3 and under 8, a minor Allied
    // victory and a minor Japanese defeat with margin -7.45.
    assert_eq!(allied.margin, fx("7.45"));
    assert_eq!(japanese.margin, fx("-7.45"));
    assert_eq!(allied.outcome, Outcome::Victory(Level::Minor));
    assert_eq!(japanese.outcome, Outcome::Defeat(Level::Minor));
    assert_eq!(allied.outcome.name(), "minor victory");
    assert_eq!(japanese.outcome.name(), "minor defeat");
    assert!(result
        .history
        .as_deref()
        .is_some_and(|h| h.starts_with("Ryujo was sunk and Enterprise was damaged")));
}

#[test]
fn the_objectives_are_those_the_decision_gives_both_sides() {
    let world = run(SEED);
    let objectives = world.objectives().expect("objectives");
    assert_eq!(
        (
            objectives.levels.minor,
            objectives.levels.major,
            objectives.levels.decisive
        ),
        (n(3), n(8), n(16))
    );
    assert_eq!(
        objectives.allied.mission,
        "Find and sink the Japanese carriers supporting the reinforcement of Guadalcanal, and keep your own carriers in action. The Japanese carriers are expected to come down from the north."
    );
    assert_eq!(
        objectives.japanese.mission,
        "Destroy the American carriers east of Guadalcanal, and keep your own in action. The American carriers are believed to be south or east of Guadalcanal."
    );
    let table = |sunk: bool| {
        let (carrier, battleship, cruiser, destroyer, transport, auxiliary) = if sunk {
            (n(10), n(8), n(4), n(1), n(2), n(1))
        } else {
            (n(5), n(4), n(2), fx("0.5"), n(1), fx("0.5"))
        };
        ShipPoints(BTreeMap::from([
            (ShipKind::Carrier, carrier),
            (ShipKind::Battleship, battleship),
            (ShipKind::Cruiser, cruiser),
            (ShipKind::Destroyer, destroyer),
            (ShipKind::Transport, transport),
            (ShipKind::Auxiliary, auxiliary),
        ]))
    };
    for side in Side::ALL {
        let own = &objectives.side(side).objectives;
        assert_eq!(own.len(), 3);
        assert_eq!(own[0], Objective::ShipsSunk(table(true)));
        assert_eq!(own[1], Objective::ShipsCrippled(table(false)));
        assert_eq!(own[2], Objective::AircraftDestroyed { points: fx("0.05") });
    }
}

// ---------------------------------------------------------------------
// What each side knows of its progress at the end of the day.
// ---------------------------------------------------------------------

#[test]
fn what_each_side_knows_at_the_end_is_what_its_claims_say() {
    // Decision 0056 (decision 14): a side's known progress reads its claims.
    // Ships sunk and aircraft destroyed are the sums of the claims, labelled
    // `claims`, and can be above the truth; ships crippled are told by no
    // claim and stay not observable. Which ships a claim counts, and the kind
    // each is worth points for, is the claim's (the kind its strike was sent
    // at). If a later change lets a side see more, this changes with it.
    let world = run(SEED);
    let objectives = world.objectives().expect("objectives");
    for side in Side::ALL {
        let view = world.side_view(side);
        let known = view.known_progress();
        assert_eq!(known.len(), 3);
        let claims = view.claims();
        assert!(!claims.is_empty());
        assert!(claims.iter().all(|c| c.from_air));

        // Ships sunk.
        let sunk: u32 = claims.iter().map(|c| c.claimed_sunk).sum();
        let attacked = claims.iter().any(|c| c.claimed_kind.is_some());
        assert_eq!(known[0].count, Fixed::from_int(i64::from(sunk)), "{side:?}");
        if sunk > 0 || attacked {
            assert_eq!(known[0].basis, KnownBasis::Claims);
            let Objective::ShipsSunk(points) = &objectives.side(side).objectives[0] else {
                panic!("the first objective is ships sunk");
            };
            let worth = claims
                .iter()
                .filter_map(|c| {
                    c.claimed_kind
                        .map(|k| points.of(k) * i64::from(c.claimed_sunk))
                })
                .fold(Fixed::ZERO, |sum, value| sum + value);
            assert_eq!(known[0].points, Some(worth), "{side:?}");
        } else {
            assert_eq!(known[0].basis, KnownBasis::NotObservable);
        }

        // Ships crippled: nothing says it.
        assert_eq!(known[1].basis, KnownBasis::NotObservable, "{side:?}");
        assert_eq!((known[1].count, known[1].points), (Fixed::ZERO, None));

        // Aircraft destroyed: shot down and destroyed on the ground.
        let aircraft: u32 = claims
            .iter()
            .map(|c| c.claimed_shot_down + c.claimed_on_ground)
            .sum();
        assert_eq!(known[2].basis, KnownBasis::Claims, "{side:?}");
        assert_eq!(known[2].count, Fixed::from_int(i64::from(aircraft)));
        assert_eq!(known[2].points, Some(known[2].count * fx("0.05")));
    }
}

// ---------------------------------------------------------------------
// Properties.
// ---------------------------------------------------------------------

#[test]
fn the_result_is_the_same_run_twice_and_from_a_saved_game_replayed() {
    let first = run(SEED);
    let second = run(SEED);
    assert_eq!(result_of(&first), result_of(&second));
    assert_eq!(first.day_hashes(), second.day_hashes());

    // A saved game replays the orders as handed in, in place of the
    // scenario's own script.
    let saved = load_text(&scenario_text(), SEED).scripted_orders().clone();
    let mut replay = load_text(&scenario_text(), SEED);
    replay.replace_script(saved);
    replay.run_to_end().expect("the replay resolves");
    assert_eq!(replay.world.day_hashes(), first.day_hashes());
    assert_eq!(result_of(&replay.world), result_of(&first));
}

#[test]
fn the_margins_sum_to_zero_and_the_levels_mirror_over_seeds() {
    for seed in 1..=50 {
        let result = result_of(&run(seed));
        assert_eq!(result.allied.margin + result.japanese.margin, Fixed::ZERO);
        assert_eq!(result.allied.outcome.mirror(), result.japanese.outcome);
        assert_eq!(
            result.allied.margin,
            result.allied.total - result.japanese.total
        );
    }
}

#[test]
fn objectives_change_no_hash_and_no_event() {
    // The same scenario with its [objectives] section cut off.
    let text = scenario_text();
    let cut = text
        .find("# Missions and objectives")
        .expect("the objectives section");
    let mut without = load_text(&text[..cut], SEED);
    assert!(without.world.objectives().is_none());
    without.run_to_end().expect("runs");
    let with = run(SEED);
    assert_eq!(with.day_hashes(), without.world.day_hashes());
    assert_eq!(with.state_hash(), without.world.state_hash());
    assert_eq!(with.events().len(), without.world.events().len());
    // And the hash is the one in the baseline file. The baseline follows the
    // live figures, so this run does too (docs/step2-spec.md, "Tests while
    // figures move").
    let live_text = std::fs::read_to_string(live_scenario_path()).expect("the live scenario reads");
    let mut live = Scenario::from_text(&live_text, &live_scenario_path())
        .expect("the scenario loads")
        .with_seed(SEED)
        .expect("a seed can be set");
    live.run_to_end().expect("runs");
    let baseline = std::fs::read_to_string(
        PathBuf::from(ROOT).join("crates/sim-cli/tests/run_baseline/05_eastern_solomons.txt"),
    )
    .expect("the baseline reads");
    assert!(baseline.contains(&format!("hash {:016x}", live.world.day_hashes()[0])));
    // The day's hash in the baseline: it was a5f3f2bbd3931d4b until decision
    // 0056 put the claims from the air into belief, and so into the hash,
    // 2f37141bddfdb856 until the hash went to version 2 (decision 9 of
    // 0055), c09495f6039a2a2a until the hash went to version 3 (decision
    // 0058, `leg_nm`) and the Ryujo detachment moved its start, and
    // e88694a2344625c0 until the hash went to version 4 (decision 0059, an
    // air group's reach), and bbd586296ddd4b8f until decisions 7 to 10 of
    // 0059 sent the aircraft in the air when a deck is lost elsewhere, and
    // a0dde1048adeb5ec until the hash went to version 5 (decision 0063, an
    // air group's target list and the day of its last base strike), and
    // b7fbc563ff170b89 until the scenario's orders of decision 0063 ("As
    // built (the scenario orders)": Ryujo in (7,-9), her raid, the carriers'
    // target lists) were given, and 747baccba462dd21 until the carriers'
    // target list gained the transport (the third trial of settlement 4).
    // That was 9887acc2b651abb9 until decision 0067 recorded each base's
    // count at the end of the day: the hash counts the events of the log. And
    // it was 798ce5b9ab626198 until the hash went to version 6 (decision
    // 0064, a ship's experience). And it was 1710b81aa6539e01 until
    // decisions 0073 and 0075 set the Japanese search from the record. And it
    // was 2ca5a5dd9f150a5d until the live scenario took the air groups and
    // orders of decision 0081 (remade for 0081 as it was for 0073). The strike
    // report reading the force after the damage (rulebook differences, row 7)
    // made it 0703fe7c146fc0f8 on the scenario before 0081, and leaves the
    // scenario of 0081 at 08c55b5f26162af0: Ryujo is not sunk on its day 1.
    // That was its hash until the state went to version 7 (decision 0083,
    // the side of a ship's torpedo tubes fired).
    assert!(baseline.contains("hash 02e1306e58d84811"));
}

/// The scenario run with the three false claim chances of decision 0056 set
/// to 0, from a copy of the frozen air combat figures under the build
/// directory.
fn run_with_no_false_claims(seed: u64) -> World {
    let options = common::altered_reference("no_false_claims", |dir| {
        common::edit_file(dir, "air_combat.toml", |figures| {
            let figures: Vec<String> = figures
                .lines()
                .map(|line| {
                    match [
                        "fighter_false_claim_chance",
                        "anti_aircraft_false_claim_chance",
                        "bomb_false_claim_chance",
                    ]
                    .iter()
                    .find(|key| line.starts_with(&format!("{key} = ")))
                    {
                        Some(key) => format!("{key} = \"0\""),
                        None => line.to_string(),
                    }
                })
                .collect();
            figures.join("\n")
        });
    });
    let mut scenario = common::from_text_with(&scenario_text(), &scenario_path(), &options)
        .expect("the scenario loads")
        .with_seed(seed)
        .expect("a seed can be set");
    scenario.run_to_end().expect("the run resolves");
    scenario.world
}

/// The ships the side's claims count only because they were left burning
/// (decision 5 of 0056, the rule of decision 0014, 16): the `burning` factor
/// of each of its claims from the air that has one.
fn burning_credits(world: &World, side: Side) -> Fixed {
    world
        .events()
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::ClaimReceived {
                side: claimant,
                from_air: true,
                ..
            } if *claimant == side => event
                .breakdown
                .as_ref()
                .and_then(|b| b.factors.iter().find(|f| f.name == "burning"))
                .map(|f| f.value),
            _ => None,
        })
        .fold(Fixed::ZERO, |sum, value| sum + value)
}

/// With the three false claim chances at 0 the known count of each
/// objective is never above the true count, for both sides, seeds 1 to 200,
/// except in the one way the rule of a claim of ships sunk allows: a ship
/// left burning is claimed sunk. So for ships sunk the known count is the
/// true count plus those burning credits, exactly. This replaces the
/// property without the condition, which held until claims could be false
/// (decision 0056): a side that claims a hit that was a miss knows more than
/// the truth has.
#[test]
fn over_seeds_1_to_200_with_no_false_claims_the_known_count_is_never_above_the_true_count() {
    for seed in 1..=200 {
        let world = run_with_no_false_claims(seed);
        let result = result_of(&world);
        for side in Side::ALL {
            let known = world.side_view(side).known_progress();
            let truth = &result.side(side).objectives;
            assert_eq!(known.len(), truth.len());
            for (k, t) in known.iter().zip(truth) {
                if k.kind == "ships_sunk" {
                    assert_eq!(
                        k.count,
                        t.count + burning_credits(&world, side),
                        "seed {seed} {side:?}: ships sunk"
                    );
                    continue;
                }
                assert!(
                    k.count <= t.count,
                    "seed {seed} {side:?} {}: knows {} of {}",
                    k.kind,
                    k.count,
                    t.count
                );
                if let Some(points) = k.points {
                    assert!(points <= t.points, "seed {seed} {side:?} {}", k.kind);
                }
            }
        }
    }
}

#[test]
fn sinking_one_more_enemy_ship_or_losing_one_fewer_never_lowers_a_margin() {
    // The scoring function on made-up counts, with the scenario's own
    // objectives: the Allied tally holds what the Allied side did to the
    // Japanese, and the Japanese tally what the Japanese side did to it.
    let world = run(SEED);
    let objectives = world.objectives().expect("objectives");
    let tally = |carriers: u32, cruisers: u32, crippled: u32, aircraft: u32| Tally {
        sunk: BTreeMap::from([(ShipKind::Carrier, carriers), (ShipKind::Cruiser, cruisers)]),
        crippled: BTreeMap::from([(ShipKind::Carrier, crippled)]),
        aircraft,
        ..Tally::default()
    };
    let margin = |allied: Tally, japanese: Tally| {
        let tallies = BTreeMap::from([(Side::Allied, allied), (Side::Japanese, japanese)]);
        score(objectives, &tallies).allied.margin
    };
    for a in 0..4 {
        for b in 0..4 {
            for (ac, jc) in [(0, 0), (5, 60), (60, 5)] {
                let before = margin(tally(a, 0, 0, ac), tally(b, 0, 0, jc));
                // One more enemy ship sunk.
                assert!(margin(tally(a + 1, 0, 0, ac), tally(b, 0, 0, jc)) >= before);
                assert!(margin(tally(a, 1, 0, ac), tally(b, 0, 0, jc)) >= before);
                // One own ship fewer sunk, or one own aircraft fewer lost.
                if b > 0 {
                    assert!(margin(tally(a, 0, 0, ac), tally(b - 1, 0, 0, jc)) >= before);
                }
                if jc > 0 {
                    assert!(margin(tally(a, 0, 0, ac), tally(b, 0, 0, jc - 1)) >= before);
                }
                // One more enemy ship crippled.
                assert!(margin(tally(a, 0, 1, ac), tally(b, 0, 0, jc)) >= before);
            }
        }
    }
}
