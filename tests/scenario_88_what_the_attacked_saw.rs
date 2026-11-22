//! Scenario 88: what the attacked saw of a raid. A side whose ship or base
//! is raided from the air is told what the aircraft attacked with, and,
//! where its people recognised them, what type they were.
//!
//! Proves decision 0167 against a hand calculation. It has no scenario
//! file of its own: it plays scenario 19a (an air attack on a ship that is
//! certain to be made) and scenario 6a (a raid on a base), each as its own
//! test plays it, on the frozen step 1 figures with one figure altered by
//! `common::altered_reference`: `raid_recognised_chance`.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly, but for the one count over seeds, which has its allowed range.
//! If the code disagrees, the code or the rule is wrong: do not change a
//! number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## The figure
//!
//! `raid_recognised_chance` is the chance that the people of an attacked
//! ship or base recognise the aircraft. It is 0 in the frozen step 1
//! figures and 0.75 in the live figures. One draw is made for each air
//! attack on a ship and for each raid on a base, when it is resolved, from
//! a stream of its own; the true event records what it gave.
//!
//! ## Scenario 19a, the morning raid (b = 1, as its case 1)
//!
//! Japanese group 2, 12 Placeholder dive bombers (aircraft type 3, which
//! attacks with a bomb), strikes Allied task force 1 in the morning pulse
//! of day 1 and finds it. Task force 1 is one ship, ship 1, so the Allied
//! side has one event of an air attack against it in that pulse.
//!
//! | The figure | The weapon the Allied side is told | The type it is told |
//! | --- | --- | --- |
//! | 1 | bombs | aircraft type 3, "Placeholder dive bomber" |
//! | 0 | bombs | none: not recognised |
//!
//! The number of aircraft the event gives is what it gave before the
//! rule, on every seed.
//!
//! On the frozen figures as they stand, the figure 0, the event is that
//! of the second row.
//!
//! ## Scenario 6a, the raid of the morning of day 1
//!
//! Japanese group 1, Placeholder dive bombers (aircraft type 3), raids
//! Allied base 2 on the morning of day 1. The Allied side has one event of
//! a raid on its base in that pulse.
//!
//! | The figure | The weapon the Allied side is told | The type it is told |
//! | --- | --- | --- |
//! | 1 | bombs | aircraft type 3, "Placeholder dive bomber" |
//! | 0 | bombs | none: not recognised |
//!
//! Its shots, its hits and what they fell on are what the event gave
//! before the rule.
//!
//! ## The chance, over seeds
//!
//! Scenario 19a's morning raid with the figure at 0.75, seeds 1 to 1000.
//! The attack is made on every seed the raid reaches the ship with a
//! bomber left; a seed on which no attack is made counts as not
//! recognised.
//!
//! | Count of runs, over seeds 1 to 1000 | p    | Mean | Std dev | Allowed    |
//! | ----------------------------------- | ---- | ---- | ------- | ---------- |
//! | The type is recognised              | 0.75 | 750  | 13.69   | 696 to 804 |
//!
//! The standard deviation is the square root of 1000 x 0.75 x 0.25. The
//! allowed range is 3.94 of them either side, as scenario 5b's are.
//!
//! # Properties
//!
//! - **The attacker is told nothing new.** The Japanese side's events of
//!   the morning raid of scenario 19a, and of the first raid of scenario
//!   6a, are those it had before the rule, word for word, with the figure
//!   at 0 and at 1.
//! - **No fight is resolved differently.** On each of seeds 1 to 50, the
//!   ships, air groups and bases of scenario 19a and of scenario 6a end
//!   each day the same with the figure at 0, at 0.75 and at 1: the hits,
//!   the aircraft lost and the damage do not move.
//!   (Note by the architect, Nov 22, 2026, with decision 0210: the
//!   test compares the ships, air groups and bases, as this says, and no
//!   longer the daily hashes, since a claim's types are in the hash and
//!   move with the figure.)
//! - **It is the same twice.** The same seed gives the same recognition
//!   twice, and a game saved and replayed gives the same.
//! - **Nothing more is told.** Two worlds that differ only in what the
//!   Allied side cannot see and has not been told give it the same events
//!   of the raid.
//! - **The other raids of the scenarios.** In scenario 6a the raids of the
//!   mornings of days 2 and 3, where they fly, each have their own draw;
//!   with the figure at 1 each names type 3 and with it at 0 none does.

mod common;

use std::fs;
use std::path::PathBuf;

use sim_core::events::{EventKind, Weapon};
use sim_core::fixed::Fixed;
use sim_core::ids::{AircraftTypeId, BaseId, EntityRef, ShipId};
use sim_core::rng::{Draw, RngKey, Subsystem};
use sim_core::side_events::{SideEvent, SideEventKind};
use sim_core::state::Side;
use sim_core::time::Pulse;
use sim_core::World;
use sim_data::{LoadOptions, Scenario};

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
const SCENARIO_19A: &str = "19a_guns_and_the_attack.toml";
const SCENARIO_6A: &str = "06a_base_raid.toml";
/// The figure as the frozen file writes it.
const FROZEN_FIGURE: &str = "raid_recognised_chance = \"0\"";
/// Type 3, the Placeholder dive bomber, of both raids.
const DIVE_BOMBER: AircraftTypeId = AircraftTypeId::new(3);
const DIVE_BOMBER_NAME: &str = "Placeholder dive bomber";
/// Ship 1, task force 1's one ship, in scenario 19a.
const SHIP_1: ShipId = ShipId::new(1);
/// Allied base 2, airfield B, in scenario 6a.
const BASE_2: BaseId = BaseId::new(2);
/// The seeds of the properties.
const PROPERTY_SEEDS: u64 = 50;
/// The count over seeds 1 to 1000 at 0.75 that the hand calculation allows.
const ALLOWED_RECOGNISED: (u64, u64) = (696, 804);

fn path(name: &str) -> PathBuf {
    PathBuf::from(ROOT).join("data/scenarios").join(name)
}

fn text(name: &str) -> String {
    fs::read_to_string(path(name)).expect("the scenario reads")
}

fn figure_line(figure: &str) -> String {
    format!("raid_recognised_chance = \"{figure}\"")
}

/// Scenario 19a as its own test plays it (a contact held at the start can be
/// struck in the morning; the guns' share is the frozen 1), with the figure
/// at `figure`.
fn options_19a(figure: &str) -> LoadOptions {
    let line = figure_line(figure);
    common::altered_reference("what_the_attacked_saw_19a", |dir| {
        let mut edits = vec![(
            "strike_contact_max_age_pulses = 0",
            "strike_contact_max_age_pulses = 1",
        )];
        if figure != "0" {
            edits.push((FROZEN_FIGURE, line.as_str()));
        }
        common::edit_file_by(dir, "air_combat.toml", &edits);
    })
}

fn load_19a_text(text: &str, figure: &str) -> Scenario {
    common::from_text_with(text, &path(SCENARIO_19A), &options_19a(figure))
        .expect("scenario 19a loads")
}

fn load_19a(figure: &str) -> Scenario {
    load_19a_text(&text(SCENARIO_19A), figure)
}

/// Scenario 6a as its own test plays it (the frozen figures), with the
/// figure at `figure`. At "0" it is the frozen figures as they stand.
fn load_6a_text(text: &str, figure: &str) -> Scenario {
    if figure == "0" {
        return common::from_text(text, &path(SCENARIO_6A)).expect("scenario 6a loads");
    }
    let line = figure_line(figure);
    let options = common::altered_reference("what_the_attacked_saw_6a", |dir| {
        common::edit_file_by(dir, "air_combat.toml", &[(FROZEN_FIGURE, line.as_str())]);
    });
    common::from_text_with(text, &path(SCENARIO_6A), &options).expect("scenario 6a loads")
}

fn load_6a(figure: &str) -> Scenario {
    load_6a_text(&text(SCENARIO_6A), figure)
}

fn run(base: &Scenario, seed: u64) -> World {
    let mut scenario = base.with_seed(seed).expect("a seed can be set");
    scenario.run_to_end().expect("the run resolves");
    scenario.world
}

fn morning_of(events: Vec<SideEvent>, day: u32) -> Vec<SideEvent> {
    events
        .into_iter()
        .filter(|e| e.day == day && e.pulse == Some(Pulse::Morning))
        .collect()
}

/// The Allied side's events of an air attack on its ship in the morning of
/// day 1: (shots, weapon, recognised).
fn attacks_told(world: &World) -> Vec<(Fixed, Weapon, Option<AircraftTypeId>)> {
    morning_of(world.side_view(Side::Allied).events(), 1)
        .into_iter()
        .filter_map(|e| match e.kind {
            SideEventKind::AirAttackAgainst {
                ship,
                shots,
                weapon,
                recognised,
                ..
            } => {
                assert_eq!(ship, SHIP_1);
                Some((shots, weapon, recognised))
            }
            _ => None,
        })
        .collect()
}

/// What the Allied side is told of each raid on its base, by day: (day,
/// event).
fn raids_told(world: &World) -> Vec<(u32, SideEventKind)> {
    world
        .side_view(Side::Allied)
        .events()
        .into_iter()
        .filter(|e| matches!(e.kind, SideEventKind::BaseAttackedAgainst { .. }))
        .map(|e| {
            assert_eq!(e.pulse, Some(Pulse::Morning), "a raid flies in the morning");
            (e.day, e.kind)
        })
        .collect()
}

/// The true event of each raid on base 2, by day.
fn raids_true(world: &World) -> Vec<(u32, EventKind)> {
    world
        .events()
        .iter()
        .filter(|e| matches!(e.kind, EventKind::BaseAttacked { .. }))
        .map(|e| (e.day, e.kind.clone()))
        .collect()
}

fn recognised_of(kind: &SideEventKind) -> Option<AircraftTypeId> {
    match kind {
        SideEventKind::BaseAttackedAgainst { recognised, .. }
        | SideEventKind::AirAttackAgainst { recognised, .. } => *recognised,
        _ => panic!("not an attack told to the attacked"),
    }
}

/// The first draw of the stream item 5 names: the day, the morning, the air
/// combat subsystem and the ship or base attacked.
fn drawn(seed: u64, day: u32, target: EntityRef, chance: Fixed) -> bool {
    RngKey {
        seed,
        day,
        pulse: Some(Pulse::Morning),
        subsystem: Subsystem::AirCombat,
        entity: target,
        other: None,
        draw: Draw::RaidRecognised,
        domain: None,
    }
    .stream()
    .chance(chance)
}

/// The words the agent gives the Allied side's event number `number`.
fn agent_words(world: &World, number: u32) -> String {
    let view = world.side_view(Side::Allied);
    match sim_agent::explain(&view, number) {
        sim_agent::ExplainAnswer::Found(answer) => {
            answer.nodes[0].text.clone().expect("an event has words")
        }
        sim_agent::ExplainAnswer::NotKnown(_) => panic!("the side knows its event {number}"),
    }
}

fn number_of_attack(world: &World) -> u32 {
    morning_of(world.side_view(Side::Allied).events(), 1)
        .into_iter()
        .find(|e| {
            matches!(
                e.kind,
                SideEventKind::AirAttackAgainst { .. } | SideEventKind::BaseAttackedAgainst { .. }
            )
        })
        .expect("an attack told")
        .number
}

// ## Scenario 19a, the morning raid

#[test]
fn scenario_19a_at_1_the_allied_side_is_told_bombs_and_type_3() {
    let base = load_19a("1");
    for seed in 1..=PROPERTY_SEEDS {
        let world = run(&base, seed);
        let told = attacks_told(&world);
        assert_eq!(told.len(), 1, "seed {seed}: one attack on ship 1");
        assert_eq!(told[0].1, Weapon::Bomb, "seed {seed}");
        assert_eq!(told[0].2, Some(DIVE_BOMBER), "seed {seed}");
    }
    let world = run(&base, 1942);
    let name = &world
        .side_view(Side::Allied)
        .aircraft_type(DIVE_BOMBER)
        .expect("type 3 is in the data")
        .name;
    assert_eq!(name, DIVE_BOMBER_NAME);
    let number = number_of_attack(&world);
    let words = agent_words(&world, number);
    assert!(
        words.ends_with(&format!(
            "aircraft with bombs, recognised as {DIVE_BOMBER_NAME}"
        )),
        "{words}"
    );
}

#[test]
fn scenario_19a_at_0_the_allied_side_is_told_bombs_and_no_type() {
    let base = load_19a("0");
    for seed in 1..=PROPERTY_SEEDS {
        let world = run(&base, seed);
        let told = attacks_told(&world);
        assert_eq!(told.len(), 1, "seed {seed}: one attack on ship 1");
        assert_eq!(told[0].1, Weapon::Bomb, "seed {seed}");
        assert_eq!(told[0].2, None, "seed {seed}");
    }
    let world = run(&base, 1942);
    let number = number_of_attack(&world);
    let words = agent_words(&world, number);
    assert!(
        words.ends_with("aircraft with bombs, type not made out"),
        "{words}"
    );
}

#[test]
fn scenario_19a_the_number_of_aircraft_told_does_not_move_with_the_figure() {
    let (at_0, at_1) = (load_19a("0"), load_19a("1"));
    for seed in 1..=PROPERTY_SEEDS {
        let (w0, w1) = (run(&at_0, seed), run(&at_1, seed));
        let (t0, t1) = (attacks_told(&w0), attacks_told(&w1));
        assert_eq!(t0.len(), 1, "seed {seed}");
        assert_eq!(t0[0].0, t1[0].0, "seed {seed}");
        // The shots told are the true event's.
        let true_shots = w0
            .events()
            .iter()
            .find_map(|e| match &e.kind {
                EventKind::AirAttack { shots, .. } if e.day == 1 => Some(*shots),
                _ => None,
            })
            .expect("the attack");
        assert_eq!(t0[0].0, true_shots, "seed {seed}");
    }
    // Scenario 19a's own pinned outcome at b = 1: 11 attack shots on seed
    // 1942.
    let told = attacks_told(&run(&at_0, 1942));
    assert_eq!(told[0].0, Fixed::from_int(11));
}

/// On the frozen figures as they stand, the event is that of the second row.
#[test]
fn scenario_19a_on_the_frozen_figures_no_type_is_named() {
    let original = fs::read_to_string(common::frozen_root().join("air_combat.toml"))
        .expect("the frozen file reads");
    assert_eq!(original.matches(FROZEN_FIGURE).count(), 1);
    let world = run(&load_19a("0"), 1942);
    let told = attacks_told(&world);
    assert_eq!(told.len(), 1);
    assert_eq!((told[0].1, told[0].2), (Weapon::Bomb, None));
}

// ## Scenario 6a, the raid of the morning of day 1

fn first_raid(world: &World) -> SideEventKind {
    let raids = raids_told(world);
    let (day, kind) = raids.first().expect("a raid on day 1").clone();
    assert_eq!(day, 1);
    assert_eq!(raids.iter().filter(|(d, _)| *d == 1).count(), 1);
    kind
}

#[test]
fn scenario_6a_at_1_the_allied_side_is_told_bombs_and_type_3() {
    let base = load_6a("1");
    for seed in 1..=PROPERTY_SEEDS {
        let world = run(&base, seed);
        let kind = first_raid(&world);
        assert_eq!(recognised_of(&kind), Some(DIVE_BOMBER), "seed {seed}");
        let words = kind.to_string();
        assert!(words.starts_with("bombs at base:2: "), "{words}");
        assert!(
            words.ends_with("; recognised as aircraft_type:3"),
            "{words}"
        );
    }
    let world = run(&base, 1942);
    let words = agent_words(&world, number_of_attack(&world));
    assert!(words.starts_with("bombs at base:2: "), "{words}");
    assert!(
        words.ends_with(&format!("; recognised as {DIVE_BOMBER_NAME}")),
        "{words}"
    );
}

#[test]
fn scenario_6a_at_0_the_allied_side_is_told_bombs_and_no_type() {
    // The figure 0 is the frozen figures as they stand.
    let base = load_6a("0");
    for seed in 1..=PROPERTY_SEEDS {
        let world = run(&base, seed);
        let kind = first_raid(&world);
        assert_eq!(recognised_of(&kind), None, "seed {seed}");
        let words = kind.to_string();
        assert!(words.starts_with("bombs at base:2: "), "{words}");
        assert!(words.ends_with("; type not made out"), "{words}");
    }
}

#[test]
fn scenario_6a_the_shots_and_hits_told_do_not_move_with_the_figure() {
    let (at_0, at_1) = (load_6a("0"), load_6a("1"));
    for seed in 1..=PROPERTY_SEEDS {
        let (w0, w1) = (run(&at_0, seed), run(&at_1, seed));
        let strip = |kind: SideEventKind| match kind {
            SideEventKind::BaseAttackedAgainst {
                base,
                shots,
                chance,
                expected,
                hits,
                hits_on_aircraft,
                hits_on_supply,
                hits_on_men,
                ..
            } => (
                base,
                shots,
                chance,
                expected,
                hits,
                hits_on_aircraft,
                hits_on_supply,
                hits_on_men,
            ),
            _ => unreachable!(),
        };
        let (k0, k1) = (strip(first_raid(&w0)), strip(first_raid(&w1)));
        assert_eq!(k0, k1, "seed {seed}");
        // And they are the true event's.
        let (_, truth) = raids_true(&w0).first().cloned().expect("the raid");
        let EventKind::BaseAttacked {
            base,
            shots,
            chance,
            expected,
            hits,
            hits_on_aircraft,
            hits_on_supply,
            hits_on_men,
            ..
        } = truth
        else {
            unreachable!()
        };
        assert_eq!(
            k0,
            (
                base,
                shots,
                chance,
                expected,
                hits,
                hits_on_aircraft,
                hits_on_supply,
                hits_on_men
            ),
            "seed {seed}"
        );
    }
}

// ## The chance, over seeds

#[test]
fn scenario_19a_at_0_75_the_type_is_recognised_696_to_804_times_in_1000() {
    let base = load_19a("0.75");
    let chance = Fixed::from_ratio(75, 100);
    let mut recognised = 0_u64;
    for seed in 1..=1000 {
        let world = run(&base, seed);
        let told = attacks_told(&world);
        assert!(told.len() <= 1, "seed {seed}");
        if let Some((_, weapon, seen)) = told.first() {
            assert_eq!(*weapon, Weapon::Bomb);
            // The draw is the first of the ship's own stream (item 5).
            let expected = drawn(seed, 1, EntityRef::Ship(SHIP_1), chance).then_some(DIVE_BOMBER);
            assert_eq!(*seen, expected, "seed {seed}");
            if seen.is_some() {
                recognised += 1;
            }
        }
    }
    let (low, high) = ALLOWED_RECOGNISED;
    assert!(
        (low..=high).contains(&recognised),
        "{recognised} recognised in 1000, allowed {low} to {high}"
    );
}

// # Properties

/// The attacker is told nothing new: the Japanese side's events of the raid
/// are the same, word for word, at 0 and at 1.
#[test]
fn the_attacker_is_told_nothing_new() {
    for (name, at_0, at_1) in [
        ("19a", load_19a("0"), load_19a("1")),
        ("6a", load_6a("0"), load_6a("1")),
    ] {
        for seed in 1..=PROPERTY_SEEDS {
            let (w0, w1) = (run(&at_0, seed), run(&at_1, seed));
            // Since decision 0179 the raiders' claim on a base names the
            // types they made out, at this same chance (its rule 11): that
            // is the attacker's own claim, not what the attacked saw, and is
            // set aside here.
            let not_a_claim = |e: &SideEvent| {
                !matches!(
                    e.kind,
                    SideEventKind::Whole(EventKind::ClaimReceived { .. })
                )
            };
            let words = |w: &World| -> Vec<String> {
                morning_of(w.side_view(Side::Japanese).events(), 1)
                    .iter()
                    .filter(|e| not_a_claim(e))
                    .map(|e| e.kind.to_string())
                    .collect()
            };
            let told = |w: &World| -> Vec<SideEvent> {
                morning_of(w.side_view(Side::Japanese).events(), 1)
                    .into_iter()
                    .filter(not_a_claim)
                    .collect()
            };
            let (j0, j1) = (told(&w0), told(&w1));
            assert_eq!(j0, j1, "scenario {name}, seed {seed}");
            assert_eq!(words(&w0), words(&w1), "scenario {name}, seed {seed}");
            assert!(
                j0.iter().any(|e| matches!(
                    e.kind,
                    SideEventKind::AirAttack { .. } | SideEventKind::BaseAttacked { .. }
                )),
                "scenario {name}, seed {seed}: the attacker's attack"
            );
            assert!(
                words(&w1)
                    .iter()
                    .all(|w| !w.contains("recognised") && !w.contains("not made out")),
                "scenario {name}, seed {seed}"
            );
        }
    }
}

/// What every ship, air group and base is at the end of a day.
fn end_of_day(world: &World) -> String {
    let truth = world.umpire();
    format!(
        "{:?}\n{:?}\n{:?}",
        truth.ships, truth.air_groups, truth.bases
    )
}

fn days_of(base: &Scenario, seed: u64) -> (Vec<String>, Vec<u64>) {
    let mut scenario = base.with_seed(seed).expect("a seed can be set");
    let mut states = Vec::new();
    while scenario.next_day().expect("the day resolves").is_some() {
        states.push(end_of_day(&scenario.world));
    }
    let hashes = scenario.world.day_hashes().to_vec();
    (states, hashes)
}

/// No fight is resolved differently: the ships, air groups and bases end
/// each day the same at 0, 0.75 and 1.
#[test]
fn no_fight_is_resolved_differently() {
    for (name, loads) in [
        ("19a", [load_19a("0"), load_19a("0.75"), load_19a("1")]),
        ("6a", [load_6a("0"), load_6a("0.75"), load_6a("1")]),
    ] {
        for seed in 1..=PROPERTY_SEEDS {
            let (states, hashes) = days_of(&loads[0], seed);
            assert!(!states.is_empty());
            for other in &loads[1..] {
                let (s, h) = days_of(other, seed);
                assert_eq!(s, states, "scenario {name}, seed {seed}");
                // Since decision 0179 the raiders' claim on a base names the
                // types they made out, at this same chance (its rule 11), and
                // the hash counts a claim's types: scenario 6a's hashes move
                // with it, and no fight does.
                if name == "19a" {
                    assert_eq!(h, hashes, "scenario {name}, seed {seed}");
                }
            }
        }
    }
}

/// It is the same twice: the same seed gives the same recognition twice,
/// and a game played again from the same start, seed and orders (what a
/// saved game's replay does) gives the same events and the same hashes.
#[test]
fn it_is_the_same_twice() {
    for (name, base) in [("19a", load_19a("0.75")), ("6a", load_6a("0.75"))] {
        for seed in 1..=PROPERTY_SEEDS {
            let (first, second) = (run(&base, seed), run(&base, seed));
            assert_eq!(
                first.side_view(Side::Allied).events(),
                second.side_view(Side::Allied).events(),
                "scenario {name}, seed {seed}"
            );
            assert_eq!(first.day_hashes(), second.day_hashes());
        }
        // Played again from a fresh load, as a replay is.
        let again = match name {
            "19a" => load_19a("0.75"),
            _ => load_6a("0.75"),
        };
        let (a, b) = (run(&base, 1942), run(&again, 1942));
        assert_eq!(
            a.side_view(Side::Allied).events(),
            b.side_view(Side::Allied).events()
        );
        assert_eq!(a.day_hashes(), b.day_hashes());
    }
}

/// Nothing more is told: two worlds that differ only in what the Allied side
/// cannot see (the names of the enemy's units) give it the same events.
#[test]
fn nothing_more_is_told() {
    let t19 = text(SCENARIO_19A);
    let t19 = common::replace_once(
        &t19,
        "name = \"Placeholder enemy carrier force\"",
        "name = \"Another name for the enemy force\"",
    );
    let t19 = common::replace_once(
        &t19,
        "name = \"Placeholder bombing squadron\"",
        "name = \"Another name for the bombers\"",
    );
    let t19 = common::replace_once(
        &t19,
        "name = \"Placeholder fighting squadron B\"",
        "name = \"Another name for the escorts\"",
    );
    let t6 = text(SCENARIO_6A);
    let t6 = common::replace_once(
        &t6,
        "name = \"Placeholder bomber group\"",
        "name = \"Another name for the bombers\"",
    );
    let t6 = common::replace_once(
        &t6,
        "name = \"Placeholder escort group\"",
        "name = \"Another name for the escorts\"",
    );
    for figure in ["0.75", "1"] {
        for (name, plain, other) in [
            ("19a", load_19a(figure), load_19a_text(&t19, figure)),
            ("6a", load_6a(figure), load_6a_text(&t6, figure)),
        ] {
            for seed in 1..=PROPERTY_SEEDS {
                let (a, b) = (run(&plain, seed), run(&other, seed));
                let (ea, eb) = (
                    a.side_view(Side::Allied).events(),
                    b.side_view(Side::Allied).events(),
                );
                assert_eq!(ea, eb, "scenario {name}, figure {figure}, seed {seed}");
                let words = |events: &[SideEvent]| -> Vec<String> {
                    events.iter().map(|e| e.kind.to_string()).collect()
                };
                assert_eq!(words(&ea), words(&eb));
            }
        }
    }
}

/// The other raids of scenario 6a: the raids of the mornings of days 2 and
/// 3, where they fly, each have their own draw; at 1 each names type 3 and
/// at 0 none does.
#[test]
fn scenario_6a_the_other_raids_each_have_their_own_draw() {
    let (at_0, at_1, at_3_4) = (load_6a("0"), load_6a("1"), load_6a("0.75"));
    let chance = Fixed::from_ratio(75, 100);
    let mut later_raids = 0;
    for seed in 1..=PROPERTY_SEEDS {
        for (raids, want) in [
            (raids_told(&run(&at_0, seed)), None),
            (raids_told(&run(&at_1, seed)), Some(DIVE_BOMBER)),
        ] {
            for (day, kind) in &raids {
                assert_eq!(recognised_of(kind), want, "seed {seed}, day {day}");
            }
        }
        // At 0.75 each raid's outcome is the first draw of the base's own
        // stream of that day's morning.
        let raids = raids_told(&run(&at_3_4, seed));
        let mut days: Vec<u32> = raids.iter().map(|(d, _)| *d).collect();
        days.dedup();
        assert_eq!(days.len(), raids.len(), "seed {seed}: one raid a day");
        for (day, kind) in &raids {
            let expected =
                drawn(seed, *day, EntityRef::Base(BASE_2), chance).then_some(DIVE_BOMBER);
            assert_eq!(recognised_of(kind), expected, "seed {seed}, day {day}");
            if *day > 1 {
                later_raids += 1;
            }
        }
    }
    assert!(
        later_raids > 0,
        "the raids of days 2 and 3 fly on some seeds"
    );
}
