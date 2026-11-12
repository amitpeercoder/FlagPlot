//! Claims from the air (decision 0056).
//!
//! # Hand calculation
//!
//! Written by the architect before the test code. The tests below assert
//! these numbers. If the code disagrees, the code or the rule is wrong: do
//! not change a number here to make a test pass.
//!
//! A claim is the real count plus a draw for each whole miss. Two settings
//! of the chances take the draws out: at 0 nothing false is claimed, and at
//! 1 every whole miss is. Both are fixed here from the event logs of the
//! scenarios as they run today with their own seeds; the fight itself is
//! the same at any setting (decision 10), which the tests also assert.
//!
//! ## The base raid, `06a_base_raid.toml`, three mornings
//!
//! Groups 1 (bombers) and 2 (escorts) raid base 2; group 3 patrols over it
//! and the base's guns fire. "Missed" is shots less hits.
//!
//! | Day | Firing | Shots | Hits | Missed | Really lost to it |
//! | --- | ------ | ----- | ---- | ------ | ----------------- |
//! | 1 | escorts at the patrol | 6 | 3 | 3 | 3 of group 3 |
//! | 1 | patrol at the escorts | 6 | 1 | 5 | 1 of group 2 |
//! | 1 | patrol at the bombers | 6 | 2 | 4 | 2 of group 1 |
//! | 1 | base guns | 20 | 0 | 20 | 0 |
//! | 1 | bombs | 16 | 1, none on aircraft | | 0 aircraft on the ground |
//! | 2 | escorts at the patrol | 6 | 2 | 4 | 2 of group 3 |
//! | 2 | patrol at the escorts | 6 | 1 | 5 | 1 of group 2 |
//! | 2 | patrol at the bombers | 3 | 1 | 2 | 1 of group 1 |
//! | 2 | base guns | 20 | 0 | 20 | 0 |
//! | 2 | bombs | 15 | 4, two on aircraft | | 2 of group 4 on the ground |
//! | 3 | escorts at the patrol | 5 | 0 | 5 | 0 |
//! | 3 | patrol at the escorts | 5 | 1 | 4 | 1 of group 2 |
//! | 3 | patrol at the bombers | 2 | 1 | 1 | 1 of group 1 |
//! | 3 | base guns | 20 | 3 | 17 | 3 of group 1 |
//! | 3 | bombs | 11 | 1, none on aircraft | | 0 |
//!
//! The raiding side's claim, each day. Shot down: what the escorts really
//! shot down, plus their misses at chance 1. On the ground: what the bombs
//! really destroyed, plus every other bomb at chance 1.
//!
//! | Day | Shot down, chances 0 | Shot down, chances 1 | On the ground, 0 | On the ground, 1 |
//! | --- | -------------------- | -------------------- | ---------------- | ---------------- |
//! | 1 | 3 | 3 + 3 = 6 | 0 | 0 + 16 = 16 |
//! | 2 | 2 | 2 + 4 = 6 | 2 | 2 + 13 = 15 |
//! | 3 | 0 | 0 + 5 = 5 | 0 | 0 + 11 = 11 |
//!
//! The defending side's claim, each day. Shot down: what the patrol and
//! the guns really shot down, plus all their misses at chance 1.
//!
//! | Day | Shot down, chances 0 | Shot down, chances 1 |
//! | --- | -------------------- | -------------------- |
//! | 1 | 1 + 2 + 0 = 3 | 3 + 5 + 4 + 20 = 32 |
//! | 2 | 1 + 1 + 0 = 2 | 2 + 5 + 2 + 20 = 29 |
//! | 3 | 1 + 1 + 3 = 5 | 5 + 4 + 1 + 17 = 27 |
//!
//! Neither side claims a ship hit or sunk: no ship was attacked. So there
//! are six claims in the three days, two a day, at any setting.
//!
//! At the placeholder chances (fighters 0.1, guns 0.05, bombs 0.1) the
//! defending side on day 1 expects 3 + 9 x 0.1 + 20 x 0.05 = 4.9 claimed
//! against 3 real. That is not asserted for one seed.
//!
//! ## The unopposed strike, `05b_strike.toml`
//!
//! Group 2 is launched at contact 1, taken for a carrier, and drops 18
//! bombs on ship 3 for 5 hits. Ship 3 ends with flotation 50 and fire 0; no
//! ship sinks and none burns. No fighter or gun fires.
//!
//! The Allied claim: hits on ships 5 at chances 0, and 5 + 13 = 18 at
//! chances 1; ships sunk 0 at both (nothing sank and no fire reaches 30);
//! the kind is carrier, what the contact was taken for at the launch,
//! although the report after the strike then reads it as a cruiser;
//! aircraft 0. The Japanese side fired nothing and gets no claim.
//!
//! # Properties
//!
//! - With claims in the game, every event that is not a claim is what it
//!   was before, and so is every ship, aircraft and base at the end: only
//!   claim events and the claims in belief are added. Checked on scenarios
//!   5, 6 and 7 against their event logs from before this change, with the
//!   claim events taken out.
//! - A claim is never below the real count it rests on, for every claim,
//!   over seeds 1 to 200 of scenario 5 and of the base raid.
//! - Over seeds 1 to 1000 of the base raid at the placeholder chances, the
//!   defending side's false claims (claimed less real, summed over all
//!   claims and seeds) are within 10 in 100 of the sum over the same runs of
//!   fighter misses x 0.1 + gun misses x 0.05.
//! - A side's claims are the same in two worlds that differ only in
//!   something that side cannot see and that did not change the firing: a
//!   claim holds no identity of an enemy unit.
//! - Run twice, and from a saved game, the claims are the same.

mod common;

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use sim_core::belief::BattleClaim;
use sim_core::events::{Cause, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::refdata::ShipKind;
use sim_core::state::Side;
use sim_core::time::Pulse;
use sim_core::{SimConfig, World};
use sim_data::Scenario;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");

/// The three chances of decision 12 (fighters, guns, bombs), as the file
/// writes them, and the two settings that take the draws out.
const PLACEHOLDER: [&str; 3] = ["0.1", "0.05", "0.1"];
const NONE: [&str; 3] = ["0", "0", "0"];
const ALL: [&str; 3] = ["1", "1", "1"];
const CHANCE_KEYS: [&str; 3] = [
    "fighter_false_claim_chance",
    "anti_aircraft_false_claim_chance",
    "bomb_false_claim_chance",
];

// ---------------------------------------------------------------------
// Scenarios with the chances set, written under the build directory. The
// repository's data folder is never touched.
// ---------------------------------------------------------------------

/// The scenario `name` (a file under `data/scenarios`) with each exact text
/// replacement of `edits` made once, its air combat figures (a copy of the
/// frozen ones) with the three chances set to `chances`, and the seed `seed`
/// if there is one.
fn variant(name: &str, chances: [&str; 3], seed: Option<u64>, edits: &[(&str, &str)]) -> Scenario {
    let options = common::altered_reference("claims", |dir| {
        common::edit_file(dir, "air_combat.toml", |figures| {
            let mut lines: Vec<String> = figures.lines().map(str::to_string).collect();
            for (key, value) in CHANCE_KEYS.iter().zip(chances) {
                let mut found = 0;
                for line in &mut lines {
                    if line.starts_with(&format!("{key} = ")) {
                        *line = format!("{key} = \"{value}\"");
                        found += 1;
                    }
                }
                assert_eq!(found, 1, "{key} is set once in the file");
            }
            lines.join("\n")
        });
    });
    let scenario_path = PathBuf::from(ROOT).join("data/scenarios").join(name);
    let mut text = fs::read_to_string(&scenario_path).expect("the scenario reads");
    for (from, to) in edits {
        text = common::replace_once(&text, from, to);
    }
    let scenario =
        common::from_text_with(&text, &scenario_path, &options).expect("the variant loads");
    match seed {
        Some(seed) => scenario.with_seed(seed).expect("a seed can be set"),
        None => scenario,
    }
}

fn run(name: &str, chances: [&str; 3], seed: Option<u64>) -> World {
    run_edited(name, chances, seed, &[])
}

fn run_edited(name: &str, chances: [&str; 3], seed: Option<u64>, edits: &[(&str, &str)]) -> World {
    let mut scenario = variant(name, chances, seed, edits);
    scenario.run_to_end().expect("the run resolves");
    scenario.world
}

// ---------------------------------------------------------------------
// What the claims of a run say, read from its log.
// ---------------------------------------------------------------------

/// One claim from the air, from its event.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Claim {
    id: u32,
    day: u32,
    pulse: Pulse,
    side: Side,
    sunk: u32,
    hits: u32,
    kind: Option<ShipKind>,
    down: u32,
    ground: u32,
    causes: Vec<Cause>,
    text: String,
}

fn air_claims(world: &World) -> Vec<Claim> {
    world
        .events()
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::ClaimReceived {
                side,
                claimed_sunk,
                claimed_hits,
                claimed_kind,
                claimed_shot_down,
                claimed_on_ground,
                from_air: true,
                ..
            } => Some(Claim {
                id: event.id.raw(),
                day: event.day,
                pulse: event.pulse.expect("a claim has a pulse"),
                side: *side,
                sunk: *claimed_sunk,
                hits: *claimed_hits,
                kind: *claimed_kind,
                down: *claimed_shot_down,
                ground: *claimed_on_ground,
                causes: event.causes.clone(),
                text: event.kind.to_string(),
            }),
            _ => None,
        })
        .collect()
}

fn is_air_claim(event: &Event) -> bool {
    matches!(event.kind, EventKind::ClaimReceived { from_air: true, .. })
}

/// The events that are not claims from the air.
fn fight_events(world: &World) -> Vec<Event> {
    world
        .events()
        .iter()
        .filter(|event| !is_air_claim(event))
        .cloned()
        .collect()
}

fn whole(value: Fixed) -> u32 {
    u32::try_from(value.trunc()).expect("a count")
}

/// Aircraft really lost to the firing events a claim cites: the
/// `AircraftLost` events caused by them.
fn real_shot_down(world: &World, claim: &Claim) -> u32 {
    world
        .events()
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::AircraftLost { aircraft, .. }
                if event
                    .causes
                    .iter()
                    .any(|cause| claim.causes.contains(cause)) =>
            {
                Some(*aircraft)
            }
            _ => None,
        })
        .sum()
}

/// The whole shots that missed in the firing events of a claim, split into
/// those of fighters and those of guns, read from the log.
fn misses_of(world: &World, claim: &Claim) -> (u32, u32) {
    let (mut fighters, mut guns) = (0, 0);
    for event in world.events().iter() {
        if !claim.causes.contains(&Cause::Event(event.id)) {
            continue;
        }
        match &event.kind {
            EventKind::AirFight { shots, hits, .. } => fighters += whole(*shots) - hits,
            EventKind::AntiAircraftFire { shots, hits, .. }
            | EventKind::BaseAntiAircraftFire { shots, hits, .. } => {
                guns += whole(*shots) - hits;
            }
            _ => {}
        }
    }
    (fighters, guns)
}

/// The claim of a side on a day (the base raid has one a side a day).
fn claim_on(claims: &[Claim], side: Side, day: u32) -> &Claim {
    let mut found = claims.iter().filter(|c| c.side == side && c.day == day);
    let claim = found.next().expect("the side claims on the day");
    assert!(found.next().is_none(), "one claim a side a day");
    claim
}

// ---------------------------------------------------------------------
// The base raid, 06a_base_raid.toml, three mornings.
// ---------------------------------------------------------------------

const RAID: &str = "06a_base_raid.toml";

/// The raid's firing, from its log, as (day, who fired, shots, hits): the
/// table of the hand calculation.
fn firing_of(world: &World) -> Vec<(u32, String, u32, u32)> {
    let mut rows = Vec::new();
    for event in world.events().iter() {
        match &event.kind {
            EventKind::AirFight {
                firing,
                at,
                shots,
                hits,
                ..
            } => rows.push((
                event.day,
                format!("{} at the {}", firing.name(), at.name()),
                whole(*shots),
                *hits,
            )),
            EventKind::BaseAntiAircraftFire { shots, hits, .. } => {
                rows.push((event.day, "base guns".to_string(), whole(*shots), *hits));
            }
            _ => {}
        }
    }
    rows
}

#[test]
fn the_firing_of_the_base_raid_is_the_table_of_the_hand_calculation() {
    let world = run(RAID, PLACEHOLDER, None);
    let row = |day: u32, who: &str, shots: u32, hits: u32| (day, who.to_string(), shots, hits);
    assert_eq!(
        firing_of(&world),
        vec![
            row(1, "escorts at the patrol", 6, 3),
            row(1, "patrol at the escorts", 6, 1),
            row(1, "patrol at the bombers", 6, 2),
            row(1, "base guns", 20, 0),
            row(2, "escorts at the patrol", 6, 2),
            row(2, "patrol at the escorts", 6, 1),
            row(2, "patrol at the bombers", 3, 1),
            row(2, "base guns", 20, 0),
            row(3, "escorts at the patrol", 5, 0),
            row(3, "patrol at the escorts", 5, 1),
            row(3, "patrol at the bombers", 2, 1),
            row(3, "base guns", 20, 3),
        ]
    );
}

#[test]
fn at_chances_0_and_1_the_aircraft_claimed_over_the_base_are_the_hand_calculation() {
    // The raiding side is the Japanese (group 1 bombs the Allied base).
    let (raiders, defenders) = (Side::Japanese, Side::Allied);
    for (chances, raiding, defending, on_ground) in [
        (NONE, [3, 2, 0], [3, 2, 5], [0, 2, 0]),
        (ALL, [6, 6, 5], [32, 29, 27], [16, 15, 11]),
    ] {
        let world = run(RAID, chances, None);
        let claims = air_claims(&world);
        assert_eq!(claims.len(), 6, "{chances:?}: two claims a day, three days");
        for day in 1..=3 {
            let raid = claim_on(&claims, raiders, day);
            let defence = claim_on(&claims, defenders, day);
            let at = day as usize - 1;
            assert_eq!(raid.down, raiding[at], "{chances:?} day {day}");
            assert_eq!(defence.down, defending[at], "{chances:?} day {day}");
            // The raiders claim aircraft destroyed on the ground; the
            // defenders, who dropped no bombs, none.
            assert_eq!(raid.ground, on_ground[at], "{chances:?} day {day}");
            assert_eq!(defence.ground, 0, "{chances:?} day {day}");
            // Neither side attacked a ship.
            for claim in [raid, defence] {
                assert_eq!((claim.sunk, claim.hits, claim.kind), (0, 0, None));
                assert_eq!(claim.pulse, Pulse::Morning);
            }
        }
    }
}

#[test]
fn the_fight_is_the_same_at_every_setting_of_the_chances() {
    // Decision 10: the claim's draws are their own and come after the raid, so
    // no event that is not a claim, no cause and no id moves, and the world
    // at the end is the same.
    for name in [
        RAID,
        "05b_strike.toml",
        "05c_air_combat.toml",
        "05d_carrier_duel.toml",
        "05_eastern_solomons.toml",
    ] {
        let placeholder = run(name, PLACEHOLDER, None);
        let none = run(name, NONE, None);
        let all = run(name, ALL, None);
        for other in [&none, &all] {
            assert_eq!(fight_events(&placeholder), fight_events(other), "{name}");
            assert_eq!(placeholder.umpire(), other.umpire(), "{name}");
            assert_eq!(
                air_claims(&placeholder).len(),
                air_claims(other).len(),
                "{name}: a claim is made whatever the chances"
            );
        }
    }
}

// ---------------------------------------------------------------------
// Properties of every claim.
// ---------------------------------------------------------------------

/// What must hold of every claim from the air in a run.
fn check_claims(world: &World, label: &str, chances_are_zero: bool) {
    let events: Vec<Event> = world.events().iter().cloned().collect();
    for claim in air_claims(world) {
        // A claim rests on firing events, earlier than it and of its day and
        // pulse, each cited once.
        assert!(!claim.causes.is_empty(), "{label}: {}", claim.text);
        let mut seen = Vec::new();
        for cause in &claim.causes {
            let Cause::Event(id) = cause else {
                panic!("{label}: a claim cites only events");
            };
            assert!(id.raw() < claim.id, "{label}");
            assert!(!seen.contains(id), "{label}: cited twice");
            seen.push(*id);
            let cited = &events[id.raw() as usize - 1];
            assert_eq!((cited.day, cited.pulse), (claim.day, Some(claim.pulse)));
            assert!(
                matches!(
                    cited.kind,
                    EventKind::AirFight { .. }
                        | EventKind::AntiAircraftFire { .. }
                        | EventKind::BaseAntiAircraftFire { .. }
                        | EventKind::AirAttack { .. }
                        | EventKind::BaseAttacked { .. }
                ),
                "{label}: {}",
                cited.kind
            );
        }
        // A claim is never below what really happened, and never above that
        // and every miss.
        let real = real_shot_down(world, &claim);
        assert!(
            claim.down >= real,
            "{label}: {} claims {} of {real}",
            claim.side.name(),
            claim.down
        );
        let (fighter, gun) = misses_of(world, &claim);
        assert!(
            claim.down <= real + fighter + gun,
            "{label}: only misses can be claimed"
        );
        if chances_are_zero {
            assert_eq!(claim.down, real, "{label}: at 0 nothing false is claimed");
        }
        // Hits on ships and aircraft destroyed on the ground, the same way.
        let strikes = strikes_of(world, &claim);
        assert!(claim.hits >= strikes.hits, "{label}: hits");
        assert!(
            claim.hits <= strikes.hits + strikes.missed,
            "{label}: only misses can be claimed as hits"
        );
        assert!(claim.ground >= strikes.destroyed, "{label}: on the ground");
        assert!(
            claim.ground <= strikes.bombs.max(strikes.destroyed),
            "{label}: no more than the bombs dropped"
        );
        assert_eq!(
            claim.kind.is_some(),
            strikes.attacked_ships,
            "{label}: the kind goes with an attack on ships"
        );
        if chances_are_zero {
            assert_eq!(claim.hits, strikes.hits, "{label}: at 0 hits are real");
            assert_eq!(claim.ground, strikes.destroyed, "{label}: at 0 as real");
        }
    }
}

/// What the attack events a claim cites say: bombs and torpedoes that hit
/// ships, that missed them, bombs dropped on a base, and the aircraft those
/// bombs destroyed on the ground.
struct Strikes {
    attacked_ships: bool,
    hits: u32,
    missed: u32,
    bombs: u32,
    destroyed: u32,
}

fn strikes_of(world: &World, claim: &Claim) -> Strikes {
    let mut strikes = Strikes {
        attacked_ships: false,
        hits: 0,
        missed: 0,
        bombs: 0,
        destroyed: 0,
    };
    for event in world.events().iter() {
        if claim.causes.contains(&Cause::Event(event.id)) {
            match &event.kind {
                EventKind::AirAttack { shots, hits, .. } => {
                    strikes.attacked_ships = true;
                    strikes.hits += hits;
                    strikes.missed += whole(*shots) - hits;
                }
                EventKind::BaseAttacked { shots, .. } => strikes.bombs += whole(*shots),
                _ => {}
            }
        } else if event
            .causes
            .iter()
            .any(|cause| claim.causes.contains(cause))
        {
            match &event.kind {
                EventKind::AircraftDestroyedOnGround { aircraft, .. }
                | EventKind::DamagedAircraftDestroyedOnGround { aircraft, .. } => {
                    strikes.destroyed += aircraft;
                }
                _ => {}
            }
        }
    }
    strikes
}

#[test]
fn a_claim_is_never_below_the_real_count_over_seeds_1_to_200() {
    let base_chances = [
        variant(RAID, PLACEHOLDER, None, &[]),
        variant("05_eastern_solomons.toml", PLACEHOLDER, None, &[]),
    ];
    let zero_chances = [
        variant(RAID, NONE, None, &[]),
        variant("05_eastern_solomons.toml", NONE, None, &[]),
    ];
    for seed in 1..=200 {
        for (index, name) in [RAID, "05_eastern_solomons.toml"].iter().enumerate() {
            for (scenarios, zero) in [(&base_chances, false), (&zero_chances, true)] {
                let mut scenario = scenarios[index].with_seed(seed).expect("a seed");
                scenario.run_to_end().expect("the run resolves");
                check_claims(
                    &scenario.world,
                    &format!("{name} seed {seed} zero {zero}"),
                    zero,
                );
            }
        }
    }
}

#[test]
fn the_defences_false_claims_follow_the_misses_over_1000_seeds_of_the_base_raid() {
    // Over seeds 1 to 1000 at the placeholder chances, the defending side's
    // false claims (claimed less real) are within 10 in 100 of what the
    // chances expect: fighter misses x 0.1 + gun misses x 0.05, summed over
    // the same runs. One run is 3 days of some 15 draws at these chances, so
    // a thousand runs make a sum of some 14 000 with a spread near 1 in 100.
    let base = variant(RAID, PLACEHOLDER, None, &[]);
    let (mut false_claims, mut expected_hundredths) = (0i64, 0i64);
    for seed in 1..=1000 {
        let mut scenario = base.with_seed(seed).expect("a seed can be set");
        scenario.world.set_config(SimConfig {
            record_breakdowns: false,
        });
        scenario.run_to_end().expect("the run resolves");
        let world = scenario.world;
        for claim in air_claims(&world).iter().filter(|c| c.side == Side::Allied) {
            let (fighter, gun) = misses_of(&world, claim);
            false_claims += i64::from(claim.down) - i64::from(real_shot_down(&world, claim));
            expected_hundredths += i64::from(fighter) * 10 + i64::from(gun) * 5;
        }
    }
    let off = (false_claims * 100 - expected_hundredths).abs();
    assert!(
        expected_hundredths > 100_000,
        "a long run: {expected_hundredths}"
    );
    assert!(
        off * 10 <= expected_hundredths,
        "false claims {} against {expected_hundredths} expected (hundredths)",
        false_claims * 100
    );
}

#[test]
fn a_claim_is_the_same_in_a_world_that_differs_in_what_the_claimant_cannot_see() {
    // The Allied scouts are fewer in the second world. The raiding Japanese
    // cannot see that, the scouts do not fire, and so the Japanese claims are
    // the same, event for event.
    let few = [(
        "aircraft_type = 2\nready = 6",
        "aircraft_type = 2\nready = 4",
    )];
    let japanese = |world: &World| -> Vec<Claim> {
        air_claims(world)
            .into_iter()
            .filter(|c| c.side == Side::Japanese)
            .collect()
    };
    for seed in 1..=50 {
        let one = run_edited(RAID, PLACEHOLDER, Some(seed), &[]);
        let other = run_edited(RAID, PLACEHOLDER, Some(seed), &few);
        assert_eq!(japanese(&one), japanese(&other), "seed {seed}");
        assert!(!japanese(&one).is_empty());
    }
}

#[test]
fn a_claim_is_in_the_belief_of_the_side_that_made_it_and_in_nobody_elses() {
    let world = run(RAID, PLACEHOLDER, None);
    for side in Side::ALL {
        let held: Vec<(u32, u32, u32)> = world
            .side_view(side)
            .claims()
            .iter()
            .map(|c: &BattleClaim| (c.day, c.claimed_shot_down, c.claimed_sunk))
            .collect();
        let told: Vec<(u32, u32, u32)> = air_claims(&world)
            .iter()
            .filter(|c| c.side == side)
            .map(|c| (c.day, c.down, c.sunk))
            .collect();
        assert_eq!(held, told, "{side:?}");
        assert_eq!(held.len(), 3);
        assert!(world.side_view(side).claims().iter().all(|c| c.from_air));
    }
}

#[test]
fn run_twice_the_claims_and_the_hashes_are_the_same() {
    let first = run(RAID, PLACEHOLDER, None);
    let second = run(RAID, PLACEHOLDER, None);
    assert_eq!(air_claims(&first), air_claims(&second));
    assert_eq!(first.day_hashes(), second.day_hashes());
    // A claim is state: the hash has it, so worlds whose claims differ are
    // told apart.
    let all = run(RAID, ALL, None);
    assert_ne!(first.day_hashes(), all.day_hashes());
}

#[test]
fn recording_the_breakdowns_changes_neither_the_claims_nor_the_hashes() {
    let with = run(RAID, PLACEHOLDER, None);
    let mut scenario = variant(RAID, PLACEHOLDER, None, &[]);
    scenario.world.set_config(SimConfig {
        record_breakdowns: false,
    });
    scenario.run_to_end().expect("the run resolves");
    let without = scenario.world;
    assert_eq!(with.day_hashes(), without.day_hashes());
    assert_eq!(air_claims(&with).len(), air_claims(&without).len());
    for (a, b) in air_claims(&with).iter().zip(air_claims(&without).iter()) {
        assert_eq!((a.down, a.side, a.day), (b.down, b.side, b.day));
    }
    // With them, a claim says how it was made.
    for event in with.events().iter().filter(|e| is_air_claim(e)) {
        let breakdown = event.breakdown.as_ref().expect("a claim has its breakdown");
        let factor = |name: &str| {
            breakdown
                .factors
                .iter()
                .find(|f| f.name == name)
                .unwrap_or_else(|| panic!("the factor {name}"))
                .value
        };
        let EventKind::ClaimReceived {
            claimed_shot_down,
            claimed_hits,
            claimed_on_ground,
            ..
        } = event.kind
        else {
            unreachable!("filtered to claims")
        };
        let count = |n: u32| Fixed::from_int(i64::from(n));
        assert_eq!(
            factor("real_shot_down") + factor("false_shot_down"),
            count(claimed_shot_down)
        );
        // The result counts what the claim says of aircraft and of hits.
        assert_eq!(
            breakdown.result,
            count(claimed_shot_down + claimed_hits + claimed_on_ground)
        );
        assert_eq!(
            factor("fighter_false_claim_chance"),
            "0.1".parse().expect("a decimal")
        );
        assert_eq!(
            factor("anti_aircraft_false_claim_chance"),
            "0.05".parse().expect("a decimal")
        );
    }
}

// ---------------------------------------------------------------------
// No battle result changed: the logs from before decision 0056.
// ---------------------------------------------------------------------

/// The ids of a line `#N ...` and of the causes after `<-`, mapped by
/// `map`. Only a `#` followed by digits is an id.
fn renumbered(line: &str, map: &BTreeMap<u32, u32>) -> String {
    let mut out = String::new();
    let mut rest = line;
    while let Some(at) = rest.find('#') {
        out.push_str(&rest[..at]);
        let digits: String = rest[at + 1..]
            .chars()
            .take_while(char::is_ascii_digit)
            .collect();
        if digits.is_empty() {
            out.push('#');
            rest = &rest[at + 1..];
            continue;
        }
        let old: u32 = digits.parse().expect("an id");
        let new = map
            .get(&old)
            .unwrap_or_else(|| panic!("event {old} is a claim, and a claim is cited by nobody"));
        out.push_str(&format!("#{new}"));
        rest = &rest[at + 1 + digits.len()..];
    }
    out.push_str(rest);
    out
}

/// Events that did not exist when the old logs were made, which a log read
/// against them takes out with the claims: the landing of aircraft elsewhere
/// and their ditching for want of a deck (decisions 7 to 10 of 0059). They
/// record a state change that was not made then (an aircraft in the air when
/// its deck was lost stayed aboard).
fn is_no_deck_event(event: &Event) -> bool {
    matches!(
        event.kind,
        EventKind::AircraftLandedElsewhere { .. } | EventKind::AircraftDitchedNoDeck { .. }
    )
}

/// A run's log as `sim-cli run --events` prints it, with the claims from the
/// air and the no-deck events taken out and the events that are left
/// numbered 1, 2, 3...
fn lines_without_claims(world: &World) -> Vec<String> {
    let kept = |e: &&Event| !is_air_claim(e) && !is_no_deck_event(e);
    let mut map = BTreeMap::new();
    for event in world.events().iter().filter(kept) {
        let next = u32::try_from(map.len() + 1).expect("few events");
        map.insert(event.id.raw(), next);
    }
    world
        .events()
        .iter()
        .filter(kept)
        .map(|event| renumbered(&event.to_string(), &map))
        .collect()
}

#[test]
fn the_log_without_the_claims_is_the_log_from_before_decision_0056() {
    // The logs under tests/claims_before are what `sim-cli run --events`
    // printed for these scenarios before the claims from the air were built
    // (the baselines of that day). Every event that is not a claim is the
    // same, with the same text and causes, once the claims are taken out and
    // the rest renumbered: the fights, hits, losses and movements did not
    // change, and every change of state records an event, so neither did
    // any ship, air group or base.
    //
    // Eastern Solomons (05) and the full slice (07) are not in the list. Their
    // fixtures recorded the game before the claims and also before decision
    // 0058, which charges ships straight-line miles and so moved their ships
    // (the carrier forces steam off the grid's axes): the movement events and
    // the pulse in which a force arrives are no longer the ones in those
    // files, and the count of events differs. The two fixture files are
    // removed. That the fights are the same at every setting of the chances
    // still holds for them, and is checked by
    // `the_fight_is_the_same_at_every_setting_of_the_chances`.
    let mut claims_added = 0;
    for name in [
        "05b_strike",
        "05c_air_combat",
        "05d_carrier_duel",
        "06_rabaul_guadalcanal",
        "06a_base_raid",
    ] {
        let before_path = PathBuf::from(ROOT)
            .join("tests/claims_before")
            .join(format!("{name}.events.txt"));
        let before_text = fs::read_to_string(&before_path).expect("the old log reads");
        let before: Vec<String> = before_text
            .lines()
            .filter(|line| line.starts_with('#'))
            .map(|line| line.trim_end_matches("  [issued by the staff]").to_string())
            .collect();
        let mut scenario =
            common::load(&PathBuf::from(ROOT).join(format!("data/scenarios/{name}.toml")))
                .expect("the scenario loads");
        scenario.run_to_end().expect("the run resolves");
        let after: Vec<String> = lines_without_claims(&scenario.world)
            .into_iter()
            .map(|line| line.trim_end_matches("  [issued by the staff]").to_string())
            .collect();
        assert_eq!(
            after.len(),
            before.len(),
            "{name}: as many events as before"
        );
        for (index, (a, b)) in after.iter().zip(&before).enumerate() {
            assert_eq!(a.trim_end(), b.trim_end(), "{name}: event {}", index + 1);
        }
        claims_added += air_claims(&scenario.world).len();
    }
    assert!(claims_added > 0, "the scenarios do make claims");
}

// ---------------------------------------------------------------------
// The unopposed strike, 05b_strike.toml.
// ---------------------------------------------------------------------

const STRIKE: &str = "05b_strike.toml";

#[test]
fn the_unopposed_strike_claims_what_the_hand_calculation_says() {
    // Group 2 drops 18 bombs on ship 3 for 5 hits; no ship sinks or burns; no
    // fighter or gun fires; the contact was taken for a carrier at the launch.
    for (chances, hits) in [(NONE, Some(5)), (ALL, Some(18)), (PLACEHOLDER, None)] {
        // At the placeholder chances the draws give what they give for this
        // seed: only the bounds are asserted for it.
        let world = run(STRIKE, chances, None);
        let truth = world.umpire();
        let ship = &truth.ships[&sim_core::ids::ShipId::new(3)];
        assert_eq!(ship.flotation_damage, Fixed::from_int(50));
        assert_eq!(ship.fire_damage, Fixed::ZERO);
        let attacks: Vec<(u32, u32)> = world
            .events()
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::AirAttack { shots, hits, .. } => Some((whole(*shots), *hits)),
                _ => None,
            })
            .collect();
        assert_eq!(attacks, vec![(18, 5)]);
        let claims = air_claims(&world);
        assert_eq!(claims.len(), 1, "the Japanese side fired nothing");
        let claim = &claims[0];
        assert_eq!(claim.side, Side::Allied);
        assert_eq!(claim.kind, Some(ShipKind::Carrier));
        assert_eq!((claim.sunk, claim.down, claim.ground), (0, 0, 0));
        match hits {
            Some(hits) => assert_eq!(claim.hits, hits, "{chances:?}"),
            None => assert!((5..=18).contains(&claim.hits), "{}", claim.hits),
        }
        // The report after the strike reads the force as a cruiser, and the
        // claim still says carrier: it is what the contact was taken for at
        // the launch.
        let attack = world
            .events()
            .iter()
            .find(|e| matches!(e.kind, EventKind::AirAttack { .. }))
            .expect("the attack")
            .id;
        let report = world
            .events()
            .iter()
            .find_map(|e| match &e.kind {
                EventKind::ReportReceived { kind, .. }
                    if e.causes.contains(&Cause::Event(attack)) =>
                {
                    Some(*kind)
                }
                _ => None,
            })
            .expect("a report after the strike");
        assert_eq!(report, ShipKind::Cruiser);
        assert!(
            claim.text.contains("hits on a carrier, 0 sunk"),
            "{}",
            claim.text
        );
        check_claims(&world, "05b", chances == NONE);
    }
}

// ---------------------------------------------------------------------
// Ships sunk from the air: decision 14 of 0014, again.
// ---------------------------------------------------------------------

/// The ships of `force` that a claim made against it should count, worked
/// from the log alone: those its cited attacks sank, and those of the force
/// afloat when the claim was made with fire damage at the claim level.
fn ships_a_claim_should_count(world: &World, claim: &Claim) -> u32 {
    let events: Vec<Event> = world.events().iter().cloned().collect();
    let level = world
        .ref_data()
        .surface_combat
        .as_ref()
        .expect("surface combat figures")
        .claimed_sunk_fire;
    // The attacks it cites, and the force they attacked.
    let mut force = None;
    let mut attacks = Vec::new();
    for cause in &claim.causes {
        let Cause::Event(id) = cause else { continue };
        if let EventKind::AirAttack { task_force, .. } = events[id.raw() as usize - 1].kind {
            force = Some(task_force);
            attacks.push(*id);
        }
    }
    let force = force.expect("a claim of ships cites an attack on a force");
    // Ships those attacks sank: ShipSunk, caused by ShipDamaged, caused by
    // an attack.
    let damaged: Vec<_> = events
        .iter()
        .filter(|e| {
            matches!(e.kind, EventKind::ShipDamaged { .. })
                && e.causes
                    .iter()
                    .any(|c| matches!(c, Cause::Event(id) if attacks.contains(id)))
        })
        .map(|e| e.id)
        .collect();
    let mut sunk: Vec<_> = Vec::new();
    for event in &events {
        if let EventKind::ShipSunk { ship, .. } = event.kind {
            if event
                .causes
                .iter()
                .any(|c| matches!(c, Cause::Event(id) if damaged.contains(id)))
                && !sunk.contains(&ship)
            {
                sunk.push(ship);
            }
        }
    }
    // The ships of the force afloat at the claim: those in it at the end and
    // those that sank after it.
    let mut members: Vec<_> = world.umpire().task_forces[&force]
        .ships
        .iter()
        .copied()
        .collect();
    for event in &events {
        if let EventKind::ShipSunk {
            ship, task_force, ..
        } = event.kind
        {
            if task_force == force && event.id.raw() > claim.id {
                members.push(ship);
            }
        }
    }
    let burning = members
        .iter()
        .filter(|ship| {
            let fire = events
                .iter()
                .take_while(|e| e.id.raw() < claim.id)
                .filter_map(|e| match e.kind {
                    EventKind::ShipDamaged {
                        ship: s,
                        track: sim_core::events::DamageTrack::Fire,
                        value_after,
                        ..
                    } if s == **ship => Some(value_after),
                    _ => None,
                })
                .last()
                .unwrap_or(Fixed::ZERO);
            fire >= level
        })
        .count();
    u32::try_from(sunk.len() + burning).expect("few ships")
}

#[test]
fn a_claim_of_ships_sunk_from_the_air_is_the_ships_sunk_and_those_burning() {
    let mut with_a_ship_claimed = 0;
    for seed in 1..=60 {
        for name in ["05_eastern_solomons.toml", "05d_carrier_duel.toml"] {
            let world = run(name, PLACEHOLDER, Some(seed));
            assert!(
                !world
                    .events()
                    .iter()
                    .any(|e| matches!(e.kind, EventKind::ShipsDetached { .. })),
                "no force splits in these runs, so final membership is membership at the claim"
            );
            check_claims(&world, &format!("{name} seed {seed}"), false);
            for claim in air_claims(&world).iter().filter(|c| c.kind.is_some()) {
                let counted = ships_a_claim_should_count(&world, claim);
                assert_eq!(claim.sunk, counted, "{name} seed {seed}: {}", claim.text);
                with_a_ship_claimed += u32::from(claim.sunk > 0);
            }
        }
    }
    assert!(with_a_ship_claimed > 0, "some run sinks or burns a ship");
}

#[test]
fn a_strike_claims_the_kind_it_was_sent_at_whatever_was_there() {
    // In every run of scenarios 5 and 5d each claim with a kind names the
    // kind the contact was taken for when its strike was launched: the kind
    // of the report that the launch cites.
    for seed in 1..=30 {
        for name in ["05_eastern_solomons.toml", "05d_carrier_duel.toml"] {
            let world = run(name, PLACEHOLDER, Some(seed));
            let events: Vec<Event> = world.events().iter().cloned().collect();
            for claim in air_claims(&world).iter().filter(|c| c.kind.is_some()) {
                let attack = claim
                    .causes
                    .iter()
                    .find_map(|cause| match cause {
                        Cause::Event(id)
                            if matches!(
                                events[id.raw() as usize - 1].kind,
                                EventKind::AirAttack { .. }
                            ) =>
                        {
                            Some(&events[id.raw() as usize - 1])
                        }
                        _ => None,
                    })
                    .expect("an attack");
                // The attack cites its launch, which cites the report that
                // made the contact.
                let launch = attack
                    .causes
                    .iter()
                    .find_map(|cause| match cause {
                        Cause::Event(id)
                            if matches!(
                                events[id.raw() as usize - 1].kind,
                                EventKind::StrikeLaunched { .. }
                            ) =>
                        {
                            Some(&events[id.raw() as usize - 1])
                        }
                        _ => None,
                    })
                    .expect("a launch");
                let report = launch
                    .causes
                    .iter()
                    .find_map(|cause| match cause {
                        Cause::Event(id) => match events[id.raw() as usize - 1].kind {
                            EventKind::ReportReceived { kind, .. } => Some(kind),
                            _ => None,
                        },
                        Cause::Order(_) => None,
                    })
                    .expect("the launch cites the report it was sent on");
                assert_eq!(claim.kind, Some(report), "seed {seed} {name}");
            }
        }
    }
}
