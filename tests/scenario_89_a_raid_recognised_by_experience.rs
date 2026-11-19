//! Scenario 89: a raid recognised by the experience of the ship attacked.
//! The chance that an attacked ship's people recognise the aircraft is
//! moved by the ship's own experience; a base keeps the plain chance.
//!
//! Proves decision 0168, an amendment to decision 0167, against a hand
//! calculation. It has no scenario file of its own. It plays scenario 19a
//! (an air attack on a ship that is certain to be made) as scenario 88
//! plays it, with ship 1 given an experience by the scenario's own key
//! (`experience` in its `[[ship]]` entry, added to the scenario's text by
//! the test), and scenario 6a (a raid on a base). Both are played on the
//! frozen step 1 figures with two figures altered by
//! `common::altered_reference`: `raid_recognised_chance` and
//! `raid_recognised_experience_strength`.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly, but for the counts over seeds, which have their allowed
//! ranges. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## The chance
//!
//! For an air attack on a ship:
//!
//! `chance = min(1, raid_recognised_chance x factor)`
//! `factor = 1 + raid_recognised_experience_strength x (experience - 2)`
//!
//! with the experience of the ship attacked, a whole number from 0 to 4.
//! With the strength at 0 the factor is 1 and the chance is the plain
//! figure. For a raid on a base the chance is the plain figure at any
//! strength. One draw is made against the chance, the draw of decision
//! 0167, from its own stream: the same seed gives the same draw whatever
//! the experience and whatever the two figures.
//!
//! Both figures are 0 in the frozen step 1 figures. In the live figures
//! the chance is 0.75 and the strength 0.25.
//!
//! ## The chance at each experience (the figure 0.75, the strength 0.25)
//!
//! | Experience of ship 1 | Factor              | Chance                  |
//! | -------------------- | ------------------- | ----------------------- |
//! | 0                    | 1 - 0.5  = 0.5      | 0.75 x 0.5  = 0.375     |
//! | 1                    | 1 - 0.25 = 0.75     | 0.75 x 0.75 = 0.5625    |
//! | 2                    | 1                   | 0.75                    |
//! | 3                    | 1 + 0.25 = 1.25     | 0.75 x 1.25 = 0.9375    |
//! | 4                    | 1 + 0.5  = 1.5      | 1.125, cut to 1         |
//!
//! (Struck by the architect on Nov 19, 2026, ruling 1 of decision 0168: the
//! statement that the breakdown of the draw gives the plain chance, the
//! experience, the factor and the chance of the row. No breakdown of the
//! draw is recorded yet; it is owed.)
//!
//! ## Scenario 19a, the morning raid, over seeds 1 to 1000
//!
//! Japanese group 2, 12 Placeholder dive bombers (aircraft type 3),
//! strikes Allied task force 1, which is ship 1, in the morning pulse of
//! day 1. The count is of the seeds on which the Allied side's event of
//! the attack names type 3. A seed on which no attack is made counts as
//! not recognised, as in scenario 88.
//!
//! | Experience | p      | Mean  | Std dev | Allowed                        |
//! | ---------- | ------ | ----- | ------- | ------------------------------ |
//! | 0          | 0.375  | 375   | 15.31   | 314 to 436                     |
//! | 1          | 0.5625 | 562.5 | 15.69   | 500 to 625                     |
//! | 2          | 0.75   | 750   | 13.69   | 696 to 804, and see below      |
//! | 3          | 0.9375 | 937.5 | 7.65    | 907 to 968                     |
//! | 4          | 1      |       |         | every seed an attack is made   |
//!
//! The standard deviation is the square root of 1000 x p x (1 - p):
//! 234.375, 246.09375, 187.5 and 58.59375 under the root. The allowed
//! range is 3.94 of them either side, taken outward to whole seeds, as
//! scenario 88's is.
//!
//! Exactly, and not by a range:
//!
//! - **Experience 2 is decision 0167.** At experience 2 the seeds
//!   recognised are the very seeds recognised with the strength at 0, one
//!   for one.
//! - **Experience 4 is certain.** At experience 4 the type is named on
//!   every seed on which the attack is made, and the count is the count of
//!   those seeds.
//! - **More experience never sees less.** The draw is the same on a seed
//!   at every experience, so a seed recognised at an experience is
//!   recognised at every higher one: the seeds of 0 are among those of 1,
//!   those of 1 among those of 2, and so on to 4.
//!
//! ## The ends of the strength
//!
//! Scenario 19a's morning raid, seeds 1 to 50:
//!
//! | The figure | The strength | Experience | Factor | Chance | The type the Allied side is told |
//! | ---------- | ------------ | ---------- | ------ | ------ | -------------------------------- |
//! | 1          | 0.5          | 0          | 0      | 0      | none, on every seed              |
//! | 1          | 0.5          | 2          | 1      | 1      | type 3, on every seed            |
//! | 0.75       | 0            | 0          | 1      | 0.75   | as at experience 2, seed by seed |
//! | 0          | 0.5          | 4          | 2      | 0      | none, on every seed              |
//!
//! On every row the weapon is told: bombs.
//!
//! ## Scenario 6a, the raid of the morning of day 1: a base
//!
//! Japanese group 1, Placeholder dive bombers (aircraft type 3), raids
//! Allied base 2. A base has no experience and its chance is the plain
//! figure.
//!
//! | The figure | The strength | The type the Allied side is told |
//! | ---------- | ------------ | -------------------------------- |
//! | 1          | 0.5          | type 3, "Placeholder dive bomber" |
//! | 1          | 0            | type 3, "Placeholder dive bomber" |
//! | 0          | 0.5          | none: not recognised              |
//!
//! With the figure at 0.75, on each of seeds 1 to 50 the raid is
//! recognised with the strength at 0.5 if and only if it is with the
//! strength at 0.
//!
//! ## The figure's bounds
//!
//! A figures file with `raid_recognised_experience_strength` below 0, or
//! above 0.5, is refused when it is read. At 0 and at 0.5 it is read.
//!
//! # Properties
//!
//! - **No fight is resolved differently.** On each of seeds 1 to 50, the
//!   ships, air groups and bases of scenario 19a end each day the same at
//!   every experience of ship 1 from 0 to 4 with the strength at 0.25 as
//!   they do with it at 0: the hits, the aircraft lost and the damage do
//!   not move. (Scenario 19a is fought by day, and no other rule reads a
//!   ship's experience by day.)
//! - **The attacker is told nothing new.** The Japanese side's events of
//!   the morning raid of scenario 19a are the same, word for word, at
//!   every experience of ship 1 and at both strengths.
//! - **Experience is not written.** Ship 1 ends every game with the
//!   experience the scenario gave it.
//! - **It is the same twice.** The same seed gives the same recognition
//!   twice, and a game saved and replayed gives the same.
//! - **The frozen figures are as they were.** With both figures as the
//!   frozen step 1 files have them, every assertion of scenario 88 on
//!   those figures holds unchanged.

// The tests are written by the build, to the statements above.
//
// One statement is not tested: "The breakdown of the draw, with breakdowns
// on, gives the plain chance, the experience and the factor of the row, and
// the chance of the row." The build records no breakdown of the draw (the
// record's "As built", reading 1), which goes to the architect.

mod common;

use std::fs;
use std::path::PathBuf;

use sim_core::events::Weapon;
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
/// The two figures as the frozen file writes them.
const FROZEN_CHANCE: &str = "raid_recognised_chance = \"0\"";
const FROZEN_STRENGTH: &str = "raid_recognised_experience_strength = \"0\"";
/// Type 3, the Placeholder dive bomber, of both raids.
const DIVE_BOMBER: AircraftTypeId = AircraftTypeId::new(3);
const DIVE_BOMBER_NAME: &str = "Placeholder dive bomber";
/// Ship 1, task force 1's one ship, in scenario 19a.
const SHIP_1: ShipId = ShipId::new(1);
/// Allied base 2, airfield B, in scenario 6a.
const BASE_2: BaseId = BaseId::new(2);
/// The seeds of the properties and of the ends of the strength.
const PROPERTY_SEEDS: u64 = 50;
/// The seeds of the counts.
const COUNT_SEEDS: u64 = 1000;
/// The live figures: the chance 0.75 and the strength 0.25.
const LIVE: (&str, &str) = ("0.75", "0.25");

fn path(name: &str) -> PathBuf {
    PathBuf::from(ROOT).join("data/scenarios").join(name)
}

fn text(name: &str) -> String {
    fs::read_to_string(path(name)).expect("the scenario reads")
}

fn fixed(s: &str) -> Fixed {
    s.parse().expect("a number")
}

/// The frozen figures with the chance at `chance` and the strength at
/// `strength`, and, for scenario 19a, the strike contact age its own test
/// plays it with.
fn options(tag: &str, chance: &str, strength: &str, for_19a: bool) -> LoadOptions {
    let chance_line = format!("raid_recognised_chance = \"{chance}\"");
    let strength_line = format!("raid_recognised_experience_strength = \"{strength}\"");
    common::altered_reference(tag, |dir| {
        let mut edits = vec![
            (FROZEN_CHANCE, chance_line.as_str()),
            (FROZEN_STRENGTH, strength_line.as_str()),
        ];
        if for_19a {
            edits.push((
                "strike_contact_max_age_pulses = 0",
                "strike_contact_max_age_pulses = 1",
            ));
        }
        common::edit_file_by(dir, "air_combat.toml", &edits);
    })
}

/// Scenario 19a's text with ship 1 given `experience` by the scenario's own
/// key.
fn text_19a(experience: u8) -> String {
    common::replace_once(
        &text(SCENARIO_19A),
        "name = \"Placeholder carrier with guns\"\n",
        &format!("name = \"Placeholder carrier with guns\"\nexperience = {experience}\n"),
    )
}

/// Scenario 19a as scenario 88 plays it, ship 1 of `experience`, the two
/// figures as given.
fn load_19a(experience: u8, chance: &str, strength: &str) -> Scenario {
    let options = options("raid_experience_19a", chance, strength, true);
    common::from_text_with(&text_19a(experience), &path(SCENARIO_19A), &options)
        .expect("scenario 19a loads")
}

/// Scenario 6a on the frozen figures with the two figures as given.
fn load_6a(chance: &str, strength: &str) -> Scenario {
    let options = options("raid_experience_6a", chance, strength, false);
    common::from_text_with(&text(SCENARIO_6A), &path(SCENARIO_6A), &options)
        .expect("scenario 6a loads")
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

/// The Allied side's events of an air attack on ship 1 in the morning of
/// day 1: (weapon, recognised).
fn attacks_told(world: &World) -> Vec<(Weapon, Option<AircraftTypeId>)> {
    morning_of(world.side_view(Side::Allied).events(), 1)
        .into_iter()
        .filter_map(|e| match e.kind {
            SideEventKind::AirAttackAgainst {
                ship,
                weapon,
                recognised,
                ..
            } => {
                assert_eq!(ship, SHIP_1);
                Some((weapon, recognised))
            }
            _ => None,
        })
        .collect()
}

/// What the Allied side is told of the one attack on ship 1, if one was
/// made: Some(recognised), or None where no attack was made.
fn told_19a(world: &World) -> Option<Option<AircraftTypeId>> {
    let told = attacks_told(world);
    assert!(told.len() <= 1, "at most one attack on ship 1");
    told.first().map(|(weapon, seen)| {
        assert_eq!(*weapon, Weapon::Bomb, "the weapon is told: bombs");
        *seen
    })
}

/// What the Allied side is told of the first raid on base 2, on day 1.
fn first_raid(world: &World) -> SideEventKind {
    let raids: Vec<SideEventKind> = morning_of(world.side_view(Side::Allied).events(), 1)
        .into_iter()
        .filter(|e| matches!(e.kind, SideEventKind::BaseAttackedAgainst { .. }))
        .map(|e| e.kind)
        .collect();
    assert_eq!(raids.len(), 1, "one raid on day 1");
    raids.into_iter().next().expect("the raid")
}

fn recognised_of(kind: &SideEventKind) -> Option<AircraftTypeId> {
    match kind {
        SideEventKind::BaseAttackedAgainst { recognised, .. } => *recognised,
        _ => panic!("not a raid told to the attacked"),
    }
}

/// The first draw of decision 0167's stream for `target` in the morning of
/// day 1, against `chance`.
fn drawn(seed: u64, target: EntityRef, chance: Fixed) -> bool {
    RngKey {
        seed,
        day: 1,
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

// ## The chance at each experience

/// The chance of each row of the hand calculation's first table, at the
/// figure 0.75 and the strength 0.25.
/// A number as a fraction: (numerator, denominator).
type Ratio = (i64, i64);

const ROWS: [(u8, Ratio, Ratio); 5] = [
    (0, (1, 2), (375, 1000)),
    (1, (3, 4), (5625, 10000)),
    (2, (1, 1), (75, 100)),
    (3, (5, 4), (9375, 10000)),
    (4, (3, 2), (1, 1)),
];

#[test]
fn the_factor_and_the_chance_at_each_experience() {
    let scenario = load_19a(2, LIVE.0, LIVE.1);
    let air = scenario
        .world
        .ref_data()
        .air_combat
        .as_ref()
        .expect("the air combat figures");
    assert_eq!(air.raid_recognised_chance, Fixed::from_ratio(75, 100));
    assert_eq!(
        air.raid_recognised_experience_strength,
        Fixed::from_ratio(25, 100)
    );
    for (experience, (fn_, fd), (cn, cd)) in ROWS {
        assert_eq!(
            air.ship_recognised_factor(experience),
            Fixed::from_ratio(fn_, fd),
            "experience {experience}"
        );
        assert_eq!(
            air.ship_recognised_chance(experience),
            Fixed::from_ratio(cn, cd),
            "experience {experience}"
        );
    }
}

// ## Scenario 19a, the morning raid, over seeds 1 to 1000

/// The allowed counts of the hand calculation's second table.
const ALLOWED: [(u8, u64, u64); 4] = [(0, 314, 436), (1, 500, 625), (2, 696, 804), (3, 907, 968)];

/// For each experience 0 to 4, for each seed 1 to 1000: what the Allied side
/// was told (None where no attack was made).
fn told_over_seeds(strength: &str) -> Vec<Vec<Option<Option<AircraftTypeId>>>> {
    (0..=4)
        .map(|experience| {
            let base = load_19a(experience, LIVE.0, strength);
            (1..=COUNT_SEEDS)
                .map(|seed| told_19a(&run(&base, seed)))
                .collect()
        })
        .collect()
}

fn recognised_count(told: &[Option<Option<AircraftTypeId>>]) -> u64 {
    told.iter().filter(|t| matches!(t, Some(Some(_)))).count() as u64
}

#[test]
fn scenario_19a_over_1000_seeds_at_each_experience() {
    let told = told_over_seeds(LIVE.1);
    let at_strength_0 = {
        let base = load_19a(2, LIVE.0, "0");
        (1..=COUNT_SEEDS)
            .map(|seed| told_19a(&run(&base, seed)))
            .collect::<Vec<_>>()
    };
    // Whether an attack is made does not move with the experience.
    for experience in 1..=4 {
        for (i, seed_told) in told[experience].iter().enumerate() {
            assert_eq!(
                seed_told.is_some(),
                told[0][i].is_some(),
                "experience {experience}, seed {}",
                i + 1
            );
        }
    }
    let mut counts = Vec::new();
    for (experience, row) in told.iter().enumerate() {
        let chance = Fixed::from_ratio(ROWS[experience].2 .0, ROWS[experience].2 .1);
        for (i, seed_told) in row.iter().enumerate() {
            let seed = i as u64 + 1;
            if let Some(seen) = seed_told {
                // Any type named is the group's own.
                assert!(seen.is_none() || *seen == Some(DIVE_BOMBER), "seed {seed}");
                // The one draw of decision 0167, against the row's chance.
                let expected = drawn(seed, EntityRef::Ship(SHIP_1), chance).then_some(DIVE_BOMBER);
                assert_eq!(*seen, expected, "experience {experience}, seed {seed}");
            }
        }
        counts.push(recognised_count(row));
    }
    eprintln!("recognised in 1000 at experience 0 to 4: {counts:?}");
    for (experience, low, high) in ALLOWED {
        let count = counts[experience as usize];
        assert!(
            (low..=high).contains(&count),
            "experience {experience}: {count} recognised in 1000, allowed {low} to {high}"
        );
    }
    // Experience 2 is decision 0167: the very seeds recognised with the
    // strength at 0.
    assert_eq!(told[2], at_strength_0, "experience 2 is decision 0167");
    // Experience 4 is certain: named on every seed an attack is made.
    let attacked = told[4].iter().filter(|t| t.is_some()).count() as u64;
    assert!(told[4]
        .iter()
        .all(|t| t.is_none() || *t == Some(Some(DIVE_BOMBER))));
    assert_eq!(counts[4], attacked);
    // More experience never sees less.
    for experience in 0..4 {
        for (i, seed_told) in told[experience].iter().enumerate() {
            if matches!(seed_told, Some(Some(_))) {
                assert!(
                    matches!(told[experience + 1][i], Some(Some(_))),
                    "seed {} recognised at {experience} and not at {}",
                    i + 1,
                    experience + 1
                );
            }
        }
    }
}

// ## The ends of the strength

#[test]
fn scenario_19a_at_the_ends_of_the_strength() {
    let rows: [(&str, &str, u8, Option<AircraftTypeId>); 3] = [
        ("1", "0.5", 0, None),
        ("1", "0.5", 2, Some(DIVE_BOMBER)),
        ("0", "0.5", 4, None),
    ];
    for (chance, strength, experience, want) in rows {
        let base = load_19a(experience, chance, strength);
        for seed in 1..=PROPERTY_SEEDS {
            let told = attacks_told(&run(&base, seed));
            assert_eq!(told.len(), 1, "seed {seed}: one attack on ship 1");
            assert_eq!(
                told[0],
                (Weapon::Bomb, want),
                "figure {chance}, strength {strength}, experience {experience}, seed {seed}"
            );
        }
    }
    // The figure 0.75, the strength 0: experience 0 is as experience 2, seed
    // by seed.
    let (at_0, at_2) = (load_19a(0, "0.75", "0"), load_19a(2, "0.75", "0"));
    for seed in 1..=PROPERTY_SEEDS {
        let (t0, t2) = (
            attacks_told(&run(&at_0, seed)),
            attacks_told(&run(&at_2, seed)),
        );
        assert_eq!(t0.len(), 1, "seed {seed}");
        assert_eq!(t0[0].0, Weapon::Bomb, "seed {seed}");
        assert_eq!(t0, t2, "seed {seed}");
    }
}

// ## Scenario 6a, the raid of the morning of day 1: a base

#[test]
fn scenario_6a_a_base_keeps_the_plain_chance() {
    for (chance, strength, want) in [
        ("1", "0.5", Some(DIVE_BOMBER)),
        ("1", "0", Some(DIVE_BOMBER)),
        ("0", "0.5", None),
    ] {
        let base = load_6a(chance, strength);
        for seed in 1..=PROPERTY_SEEDS {
            let kind = first_raid(&run(&base, seed));
            assert_eq!(
                recognised_of(&kind),
                want,
                "figure {chance}, strength {strength}, seed {seed}"
            );
            let words = kind.to_string();
            assert!(words.starts_with("bombs at base:2: "), "{words}");
        }
        let world = run(&base, 1942);
        let kind = first_raid(&world);
        if want.is_some() {
            let name = &world
                .side_view(Side::Allied)
                .aircraft_type(DIVE_BOMBER)
                .expect("type 3 is in the data")
                .name;
            assert_eq!(name, DIVE_BOMBER_NAME);
            assert!(kind
                .to_string()
                .ends_with("; recognised as aircraft_type:3"));
        } else {
            assert!(kind.to_string().ends_with("; type not made out"));
        }
    }
    // At 0.75 the raid is recognised at the strength 0.5 if and only if at 0.
    let (at_half, at_0) = (load_6a("0.75", "0.5"), load_6a("0.75", "0"));
    let chance = Fixed::from_ratio(75, 100);
    for seed in 1..=PROPERTY_SEEDS {
        let (a, b) = (
            recognised_of(&first_raid(&run(&at_half, seed))),
            recognised_of(&first_raid(&run(&at_0, seed))),
        );
        assert_eq!(a, b, "seed {seed}");
        assert_eq!(
            a,
            drawn(seed, EntityRef::Base(BASE_2), chance).then_some(DIVE_BOMBER),
            "seed {seed}"
        );
    }
}

// ## The figure's bounds

#[test]
fn the_strength_is_refused_below_0_and_above_half() {
    for strength in ["-0.01", "-1", "0.51", "1"] {
        let options = options("raid_experience_bounds", "0.75", strength, true);
        let refused = common::from_text_with(&text_19a(2), &path(SCENARIO_19A), &options);
        let message = match refused {
            Ok(_) => panic!("{strength}: read, and should be refused"),
            Err(error) => error.to_string(),
        };
        assert!(
            message.contains("raid recognised experience strength"),
            "{strength}: {message}"
        );
    }
    for strength in ["0", "0.5"] {
        let scenario = load_19a(2, "0.75", strength);
        assert_eq!(
            scenario
                .world
                .ref_data()
                .air_combat
                .as_ref()
                .expect("the figures")
                .raid_recognised_experience_strength,
            fixed(strength)
        );
    }
}

// # Properties

/// What every ship, air group and base is at the end of a day.
fn end_of_day(world: &World) -> String {
    let truth = world.umpire();
    format!(
        "{:?}\n{:?}\n{:?}",
        truth.ships, truth.air_groups, truth.bases
    )
}

fn days_of(base: &Scenario, seed: u64) -> (Vec<String>, Vec<u64>, u8) {
    let mut scenario = base.with_seed(seed).expect("a seed can be set");
    let mut states = Vec::new();
    while scenario.next_day().expect("the day resolves").is_some() {
        states.push(end_of_day(&scenario.world));
    }
    let hashes = scenario.world.day_hashes().to_vec();
    let experience = scenario.world.umpire().ships[&SHIP_1].experience;
    (states, hashes, experience)
}

/// No fight is resolved differently, and experience is not written: at
/// every experience the ships, air groups and bases end each day the same at
/// the strength 0.25 as at 0, and ship 1 ends with the experience it was
/// given.
#[test]
fn no_fight_is_resolved_differently_and_experience_is_not_written() {
    for experience in 0..=4 {
        let (plain, moved) = (
            load_19a(experience, LIVE.0, "0"),
            load_19a(experience, LIVE.0, LIVE.1),
        );
        for seed in 1..=PROPERTY_SEEDS {
            let (states, hashes, kept) = days_of(&plain, seed);
            assert!(!states.is_empty());
            let (s, h, kept_moved) = days_of(&moved, seed);
            assert_eq!(s, states, "experience {experience}, seed {seed}");
            assert_eq!(h, hashes, "experience {experience}, seed {seed}");
            assert_eq!(kept, experience, "seed {seed}");
            assert_eq!(kept_moved, experience, "seed {seed}");
        }
    }
}

/// The attacker is told nothing new: the Japanese side's events of the
/// morning raid are the same, word for word, at every experience and at both
/// strengths.
#[test]
fn the_attacker_is_told_nothing_new() {
    let loads: Vec<(u8, &str, Scenario)> = (0..=4)
        .flat_map(|experience| {
            ["0", LIVE.1]
                .into_iter()
                .map(move |strength| (experience, strength, load_19a(experience, LIVE.0, strength)))
        })
        .collect();
    for seed in 1..=PROPERTY_SEEDS {
        let mut first: Option<(Vec<SideEvent>, Vec<String>)> = None;
        for (experience, strength, base) in &loads {
            let events = morning_of(run(base, seed).side_view(Side::Japanese).events(), 1);
            let words: Vec<String> = events.iter().map(|e| e.kind.to_string()).collect();
            assert!(
                events
                    .iter()
                    .any(|e| matches!(e.kind, SideEventKind::AirAttack { .. })),
                "seed {seed}: the attacker's attack"
            );
            assert!(words
                .iter()
                .all(|w| !w.contains("recognised") && !w.contains("not made out")));
            match &first {
                None => first = Some((events, words)),
                Some((e0, w0)) => {
                    assert_eq!(
                        &events, e0,
                        "experience {experience}, strength {strength}, seed {seed}"
                    );
                    assert_eq!(
                        &words, w0,
                        "experience {experience}, strength {strength}, seed {seed}"
                    );
                }
            }
        }
    }
}

/// It is the same twice: the same seed gives the same recognition twice, and
/// the game played again from a fresh load, as a replay is, gives the same
/// events and hashes.
#[test]
fn it_is_the_same_twice() {
    for experience in [0, 3] {
        let base = load_19a(experience, LIVE.0, LIVE.1);
        for seed in 1..=PROPERTY_SEEDS {
            let (a, b) = (run(&base, seed), run(&base, seed));
            assert_eq!(
                a.side_view(Side::Allied).events(),
                b.side_view(Side::Allied).events(),
                "experience {experience}, seed {seed}"
            );
            assert_eq!(a.day_hashes(), b.day_hashes());
        }
        let again = load_19a(experience, LIVE.0, LIVE.1);
        let (a, b) = (run(&base, 1942), run(&again, 1942));
        assert_eq!(
            a.side_view(Side::Allied).events(),
            b.side_view(Side::Allied).events()
        );
        assert_eq!(a.day_hashes(), b.day_hashes());
    }
}

/// The frozen figures are as they were: both figures are 0 in the frozen
/// file, so the chance is the plain 0 at every experience and no type is
/// named. (Scenario 88's own assertions on those figures run in its file.)
#[test]
fn the_frozen_figures_are_as_they_were() {
    let frozen = fs::read_to_string(common::frozen_root().join("air_combat.toml"))
        .expect("the frozen file reads");
    assert_eq!(frozen.matches(FROZEN_CHANCE).count(), 1);
    assert_eq!(frozen.matches(FROZEN_STRENGTH).count(), 1);
    for experience in [0, 4] {
        let scenario = common::from_text_with(
            &text_19a(experience),
            &path(SCENARIO_19A),
            &options("raid_experience_frozen", "0", "0", true),
        )
        .expect("scenario 19a loads");
        let air = scenario
            .world
            .ref_data()
            .air_combat
            .as_ref()
            .expect("figures");
        assert_eq!(air.ship_recognised_chance(experience), Fixed::ZERO);
        for seed in 1..=PROPERTY_SEEDS {
            assert_eq!(told_19a(&run(&scenario, seed)), Some(None), "seed {seed}");
        }
    }
}
