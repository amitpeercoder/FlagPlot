//! Anti-aircraft fire and the attack (decision 0079): a share of the guns'
//! fire is worked before the attack and the rest after it.
//!
//! # Hand calculation
//!
//! Written by the architect before the test code, and checked by the
//! coordinating architect on Nov 13, 2026. The tests below assert these
//! numbers. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass.
//!
//! Quantities are cut to six decimal places at each step.
//!
//! ## The scenario
//!
//! `data/scenarios/19a_guns_and_the_attack.toml`, one day, on the frozen
//! step 1 figures (`data/reference/step1`), altered by
//! `common::altered_reference` where a case says so:
//!
//! - `strike_contact_max_age_pulses` is 1, not 0, in every case. A contact a
//!   side starts the game holding at age 0 is aged at the end of the night
//!   pulse, so it is 1 pulse old at the morning's decision; at 1 it can be
//!   struck in the morning with no search and no random report (as in
//!   scenario 13a). Everything asserted by hand is of the morning pulse of
//!   day 1.
//! - `anti_aircraft_before_attack_share`, b, is 1 in the frozen figures
//!   (step 1's rule) and is set to 0.5, 0.33 and 0 for the other cases.
//!
//! The Allied task force 1 is one "Placeholder carrier with guns" (class 5:
//! toughness 600, 20 anti-aircraft shots, capacity 48), stopped, with group
//! 1, 6 Placeholder fighters (type 4) on combat air patrol with a patrol
//! share of 1. The Japanese task force 2, within 250 nm of it and stopped, is
//! one "Placeholder carrier" (class 4, no guns) with group 2, 12 Placeholder
//! dive bombers (type 3: a bomb, damage 60, no protection) on naval strike,
//! and group 3, 4 Placeholder fighters on combat air patrol with a patrol
//! share of 0. The Japanese side starts holding a contact of task force 1 at
//! its true hex, one ship, carrier. The Allied side holds none and has no
//! strike group, so nothing strikes the Japanese.
//!
//! ## Case 1: the morning raid, at b = 1, 0.5, 0.33 and 0
//!
//! Group 2 launches all 12 at the contact; group 3 sends its 4 as escorts
//! (all it can fly: its patrol share is 0, and the raid is within its 250
//! nm). The raid finds task force 1. No type has pilot armour or
//! self-sealing tanks, so every chance at aircraft is its base figure.
//!
//! | Roll | Shots | Chance | Expected hits |
//! | --- | --- | --- | --- |
//! | Escorts at the patrol | 4 | 0.3 | 1.2 |
//! | Patrol at the escorts (min(6, 4)) | 4 | 0.3 | 1.2 |
//! | Patrol at the bombers (6 − 4) | 2 | 0.3 | 0.6 |
//! | Guns, in all | 20 | 0.05 | 1.0 |
//!
//! The guns' 20 shots split: k = floor(b × 20) before the attack, 20 − k
//! after.
//!
//! | b | Before / after | Expected attack shots, 12 − 0.6 − 0.05k | Expected bomb hits on the carrier (× 0.2) | Expected bombers lost to the guns |
//! | --- | --- | --- | --- | --- |
//! | 1 | 20 / 0 | 10.4 | 2.08 | 1.0 |
//! | 0.5 | 10 / 10 | 10.9 | 2.18 | 1.0 |
//! | 0.33 | 6 / 14 (floor 6.6) | 11.1 | 2.22 | 1.0 |
//! | 0 | 0 / 20 | 11.4 | 2.28 | 1.0 |
//!
//! The attack shots are exact expectations while the 12 bombers are not
//! exhausted; that needs more than 10 hits from 2 patrol shots and at most
//! 20 gun shots, which has a chance below one in a million. The bombers lost
//! in all are 1.6 expected at every b. A bomb hit does 100 × 60 / 600 = 10
//! points.
//!
//! **The breakdown.** Every gun event's breakdown has its shots and the
//! chance 0.05, with `expected` the shots times 0.05: 1.0 at b = 1 (20
//! shots); 0.5 and 0.5 at b = 0.5; 0.3 and 0.7 at b = 0.33; 1.0 at b = 0
//! (after the attack). When b is below 1 it also lists the guns' shots in
//! all, 20, and the share b. At b = 1 it is the breakdown of step 1.
//!
//! **The events, at b = 0.5,** in this order in the morning: the launch;
//! `EscortsSent`; the escorts' `AirFight` at the patrol; the patrol's at the
//! escorts and at the bombers; `AntiAircraftFire` of 10 shots (not after
//! the attack); the `AircraftLost` of those rolls; the `AirAttack` of group
//! 2; `AntiAircraftFire` of 10 shots after the attack, citing the launch and
//! the `AirAttack`; its `AircraftLost` of group 2, citing it; then the
//! damage, the report and the claims. At b = 1 there is no second gun event
//! and the log is step 1's. At b = 0 the only gun event is the one after the
//! attack. A part with no shots records nothing.
//!
//! ## Case 2: a lone bomber
//!
//! The scenario with group 2 at 1 dive bomber, group 3 at 0 and group 1 at 0,
//! so no escort and no patrol: the bomber meets the 20 guns alone.
//!
//! | b | Chance it attacks | Expected bomb hits | Chance it is lost |
//! | --- | --- | --- | --- |
//! | 1 | 0.95^20 = 0.358486 | 0.071697 | 0.641514 |
//! | 0.5 | 0.95^10 = 0.598737 | 0.119747 | 0.641514 |
//! | 0 | 1 | 0.2 | 0.641514 |
//!
//! ## Case 3: a raid on a base
//!
//! A base with 8 guns, no patrol, raided by 15 Placeholder dive bombers from
//! a base within their radius, at b = 0.5: k = 4 before and 4 after. The guns
//! expect 0.2 hits before and 0.2 after. The bombs expect (15 − 0.2) × 0.2 =
//! 2.96 hits, against 2.92 at b = 1 and 3.0 at b = 0, each landing by the
//! `[base_hit]` draws as now. The fire after the attack cites the launch and
//! the `BaseAttacked` event.
//!
//! # Properties
//!
//! Over seeds 1 to 1000, case 1, at b = 1, 0.5, 0.33 and 0, each seed run
//! four times:
//!
//! - The guns' total hits in the morning are equal at every b, and so is
//!   each group's total of aircraft lost, and so is the defending side's
//!   claim, and the raiding side's claim of aircraft shot down. (The raiders'
//!   claim of hits rests on the bombs dropped, which differ by b.)
//! - The attack's shots at b equal the bombers left by the patrol less the
//!   hits among the first k of the guns' draws, the hits of the before event,
//!   never below 0.
//! - At b = 1 the event log of every seed is the log with the figure as it
//!   was before the rule, byte for byte.
//! - The morning attack's shots summed over the 1,000 seeds are within four
//!   standard deviations of 1,000 times the expectation. Per seed the
//!   variance is 2 × 0.3 × 0.7 = 0.42 from the patrol's two shots at the
//!   bombers plus k × 0.05 × 0.95 = 0.0475k from the guns before the attack:
//!
//! | b | Expected total | Standard deviation of the total | Allowed |
//! | --- | --- | --- | --- |
//! | 1 | 10,400 | sqrt(1,000 × 1.37) = 37.0 | 10,252 to 10,548 |
//! | 0.5 | 10,900 | sqrt(1,000 × 0.895) = 29.9 | 10,780 to 11,020 |
//! | 0.33 | 11,100 | sqrt(1,000 × 0.705) = 26.6 | 10,994 to 11,206 |
//! | 0 | 11,400 | sqrt(1,000 × 0.42) = 20.5 | 11,318 to 11,482 |
//!
//! - In every scenario that existed before the rule, on the frozen figures,
//!   the events are what they were: every other hand calculation passes
//!   untouched.
//!
//! # Pinned outcomes
//!
//! What the scenario's own seed (1942) gives in case 1 at each b. Pinned,
//! not calculated: no hand calculation can say which draw succeeds. A change
//! here means a stream or the order of draws moved; they are remade only
//! with a deliberate change of the rule (docs/step2-spec.md, "Tests while
//! figures move", 7). Filled in by the build from the first run, and checked
//! against the properties above before they are pinned.
//!
//! PINNED (not calculated), case 1, morning of day 1. Gun hits are the hits of
//! the guns' events before and after the attack; the attack's shots and bomb
//! hits are group 2's; lost is by group 1, 2 and 3. On seed 1942 the guns miss
//! all 20 shots, so the outcome is the same at every b; seed 10 is the first
//! seed on which they hit and the share moves the hits across the attack.
//!
//! | Seed | b | Gun hits before | Gun hits after | Attack shots | Bomb hits | Lost (1, 2, 3) |
//! | --- | --- | --- | --- | --- | --- | --- |
//! | 1942 | 1 | 0 | 0 | 11 | 1 | 2, 1, 1 |
//! | 1942 | 0.5 | 0 | 0 | 11 | 1 | 2, 1, 1 |
//! | 1942 | 0.33 | 0 | 0 | 11 | 1 | 2, 1, 1 |
//! | 1942 | 0 | 0 | 0 | 11 | 1 | 2, 1, 1 |
//! | 10 | 1 | 3 | 0 | 8 | 1 | 1, 4, 1 |
//! | 10 | 0.5 | 2 | 1 | 9 | 2 | 1, 4, 1 |
//! | 10 | 0.33 | 1 | 2 | 10 | 2 | 1, 4, 1 |
//! | 10 | 0 | 0 | 3 | 11 | 2 | 1, 4, 1 |
//!
//! Checked against the properties before pinning: the total gun hits and the
//! losses by group are the same at every b; the hits before grow with the
//! first k of the 20 draws (0, 1, 2, 3 on seed 10); the attack's shots are 12
//! less the patrol's 1 hit at the bombers less the hits before (11 - 3 = 8,
//! 9, 10, 11).

mod common;

use std::fs;
use std::path::PathBuf;

use sim_core::events::{AirFirer, AirTarget, Cause, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::ids::{AirGroupId, EntityRef, EventId, ShipId};
use sim_core::rng::{Draw, RngKey, Subsystem};
use sim_core::state::Side;
use sim_core::time::Pulse;
use sim_core::World;
use sim_data::{LoadOptions, Scenario};

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
const SCENARIO: &str = "19a_guns_and_the_attack.toml";
const SEEDS: u64 = 1000;
/// The shares of the cases, as written in the figures file, and the guns'
/// shots (before, after) each gives: k = floor(b x 20), then 20 - k.
const SHARES: [(&str, i64, i64); 4] =
    [("1", 20, 0), ("0.5", 10, 10), ("0.33", 6, 14), ("0", 0, 20)];
/// The attack's shots in all over 1,000 seeds that the hand calculation
/// allows at each share: expected within four standard deviations.
const ALLOWED_ATTACK_SHOTS: [(u64, u64); 4] = [
    (10_252, 10_548),
    (10_780, 11_020),
    (10_994, 11_206),
    (11_318, 11_482),
];

fn fx(n: i64) -> Fixed {
    Fixed::from_int(n)
}

fn hundredths(n: i64) -> Fixed {
    Fixed::from_ratio(n, 100)
}

fn scenario_path() -> PathBuf {
    PathBuf::from(ROOT).join("data/scenarios").join(SCENARIO)
}

fn scenario_text() -> String {
    fs::read_to_string(scenario_path()).expect("the scenario reads")
}

/// The frozen figures with the two changes the hand calculation names: a
/// contact the side starts with can be struck in the morning, and the share
/// is `share` (written as the figures file writes it).
fn options(share: &str) -> LoadOptions {
    common::altered_reference("guns_and_the_attack", |dir| {
        common::edit_file_by(
            dir,
            "air_combat.toml",
            &[
                (
                    "strike_contact_max_age_pulses = 0",
                    "strike_contact_max_age_pulses = 1",
                ),
                (
                    "anti_aircraft_before_attack_share = \"1\"",
                    &format!("anti_aircraft_before_attack_share = \"{share}\""),
                ),
            ],
        );
    })
}

fn load(text: &str, share: &str) -> Scenario {
    common::from_text_with(text, &scenario_path(), &options(share)).expect("the scenario loads")
}

/// Case 1: the scenario as the file gives it.
fn case_1(share: &str) -> Scenario {
    load(&scenario_text(), share)
}

/// Case 2: a lone bomber, no escort and no patrol.
fn case_2(share: &str) -> Scenario {
    let text = scenario_text();
    let text = common::replace_once(&text, "ready = 6", "ready = 0");
    let text = common::replace_once(&text, "ready = 12", "ready = 1");
    let text = common::replace_once(&text, "ready = 4", "ready = 0");
    load(&text, share)
}

/// Case 3: 15 dive bombers raid a base with 8 guns and no patrol.
fn case_3(share: &str) -> Scenario {
    let text = r#"name = "Scenario 19a, case 3: a raid on a base"
placeholder = true
seed = 1942
days = 1
map = "../map/placeholder_two_airfields.toml"
ship_classes = "../reference/ship_classes.csv"
port_sizes = "../reference/port_sizes.csv"
aircraft_types = "../reference/aircraft_types.csv"
search = "../reference/search.toml"
surface_combat = "../reference/surface_combat.toml"
air_combat = "../reference/air_combat.toml"

[[base]]
id = 1
name = "Placeholder airfield A"
side = "japanese"
q = 0
r = 0
supply_t = 0

[[base]]
id = 2
name = "Placeholder airfield B"
side = "allied"
q = 5
r = 0
supply_t = 1000
anti_aircraft = 8

[[air_group]]
id = 1
name = "Placeholder bomber group"
side = "japanese"
base = 1
aircraft_type = 3
ready = 15

[[air_group]]
id = 4
name = "Placeholder resting bombers"
side = "allied"
base = 2
aircraft_type = 3
ready = 10

[[order]]
day = 1
side = "japanese"
air_group = 1
kind = "set_air_mission"
mission = "base_strike"
target_base = 2
"#;
    load(text, share)
}

fn run_seed(base: &Scenario, seed: u64) -> World {
    let mut scenario = base.with_seed(seed).expect("a seed can be set");
    scenario.run_to_end().expect("the run resolves");
    scenario.world
}

fn morning(world: &World) -> Vec<Event> {
    world
        .events()
        .iter()
        .filter(|event| event.day == 1 && event.pulse == Some(Pulse::Morning))
        .cloned()
        .collect()
}

fn is_guns(kind: &EventKind) -> bool {
    matches!(
        kind,
        EventKind::AntiAircraftFire { .. } | EventKind::BaseAntiAircraftFire { .. }
    )
}

/// (shots, chance, expected, hits, after_attack) of a gun event.
fn gun_of(event: &Event) -> (Fixed, Fixed, Fixed, u32, bool) {
    match event.kind {
        EventKind::AntiAircraftFire {
            shots,
            chance,
            expected,
            hits,
            after_attack,
            ..
        }
        | EventKind::BaseAntiAircraftFire {
            shots,
            chance,
            expected,
            hits,
            after_attack,
            ..
        } => (shots, chance, expected, hits, after_attack),
        ref other => panic!("not a gun event: {other:?}"),
    }
}

fn guns(events: &[Event]) -> Vec<Event> {
    events
        .iter()
        .filter(|e| is_guns(&e.kind))
        .cloned()
        .collect()
}

fn attack_events(events: &[Event]) -> Vec<Event> {
    events
        .iter()
        .filter(|e| matches!(e.kind, EventKind::AirAttack { .. }))
        .cloned()
        .collect()
}

/// (shots, hits) of the morning's attack of group 2, none if it has none.
fn attack_of(events: &[Event]) -> (i64, u32) {
    attack_events(events)
        .first()
        .map_or((0, 0), |event| match event.kind {
            EventKind::AirAttack { shots, hits, .. } => (shots.trunc(), hits),
            _ => unreachable!(),
        })
}

fn lost_by(events: &[Event], group: u32) -> u32 {
    events
        .iter()
        .map(|event| match event.kind {
            EventKind::AircraftLost {
                air_group,
                aircraft,
                ..
            } if air_group.raw() == group => aircraft,
            _ => 0,
        })
        .sum()
}

/// The hits of the patrol's roll at the bombers, none if it has no roll.
fn patrol_hits_at_bombers(events: &[Event]) -> u32 {
    events
        .iter()
        .map(|event| match event.kind {
            EventKind::AirFight {
                firing: AirFirer::Patrol,
                at: AirTarget::Bombers,
                hits,
                ..
            } => hits,
            _ => 0,
        })
        .sum()
}

/// The 20 draws of the guns' stream of the morning, each True for a hit at
/// `chance`, as the rule rolls them: one stream keyed on the lowest ship of
/// the force and the raid's first group, one draw for each shot, in order
/// (decision 3 of 0079).
fn gun_draws(seed: u64, shots: usize, chance: Fixed) -> Vec<bool> {
    let mut stream = RngKey {
        seed,
        day: 1,
        pulse: Some(Pulse::Morning),
        subsystem: Subsystem::AirCombat,
        entity: EntityRef::Ship(ShipId::new(1)),
        other: Some(EntityRef::AirGroup(AirGroupId::new(2))),
        draw: Draw::Legacy,
    }
    .stream();
    (0..shots).map(|_| stream.chance(chance)).collect()
}

fn names(event: &Event) -> Vec<&'static str> {
    event
        .breakdown
        .as_ref()
        .expect("breakdowns are on")
        .factors
        .iter()
        .map(|factor| factor.name)
        .collect()
}

fn value_of(event: &Event, name: &str) -> Fixed {
    event
        .breakdown
        .as_ref()
        .expect("breakdowns are on")
        .factors
        .iter()
        .find(|factor| factor.name == name)
        .unwrap_or_else(|| panic!("no factor {name}"))
        .value
}

/// What case 1 gives in the morning of one run.
struct Morning {
    before_hits: u32,
    after_hits: u32,
    attack_shots: i64,
    attack_hits: u32,
    lost: [u32; 3],
    patrol_at_bombers: u32,
    claims: Vec<EventKind>,
}

fn summarize(world: &World) -> Morning {
    let events = morning(world);
    let parts = guns(&events);
    let hits = |after: bool| -> u32 {
        parts
            .iter()
            .map(gun_of)
            .filter(|part| part.4 == after)
            .map(|part| part.3)
            .sum()
    };
    let (attack_shots, attack_hits) = attack_of(&events);
    Morning {
        before_hits: hits(false),
        after_hits: hits(true),
        attack_shots,
        attack_hits,
        lost: [
            lost_by(&events, 1),
            lost_by(&events, 2),
            lost_by(&events, 3),
        ],
        patrol_at_bombers: patrol_hits_at_bombers(&events),
        claims: events
            .iter()
            .filter(|e| matches!(e.kind, EventKind::ClaimReceived { .. }))
            .map(|e| e.kind.clone())
            .collect(),
    }
}

// ---------------------------------------------------------------------
// Case 1: the table, the breakdowns and the events.
// ---------------------------------------------------------------------

#[test]
fn the_rolls_before_the_guns_have_the_shots_and_chances_of_the_table_at_every_seed_and_share() {
    for (share, _, _) in SHARES {
        let base = case_1(share);
        for seed in 1..=SEEDS {
            let world = run_seed(&base, seed);
            let events = morning(&world);
            let fights: Vec<(AirFirer, AirTarget, Fixed, Fixed, Fixed)> = events
                .iter()
                .filter_map(|e| match e.kind {
                    EventKind::AirFight {
                        firing,
                        at,
                        shots,
                        chance,
                        expected,
                        ..
                    } => Some((firing, at, shots, chance, expected)),
                    _ => None,
                })
                .collect();
            // Escorts at the patrol: 4 x 0.3; the patrol at the escorts:
            // min(6, 4) = 4 x 0.3; at the bombers: 6 - 4 = 2 x 0.3.
            let point_three = hundredths(30);
            assert_eq!(
                fights,
                vec![
                    (
                        AirFirer::Escorts,
                        AirTarget::Patrol,
                        fx(4),
                        point_three,
                        hundredths(120)
                    ),
                    (
                        AirFirer::Patrol,
                        AirTarget::Escorts,
                        fx(4),
                        point_three,
                        hundredths(120)
                    ),
                    (
                        AirFirer::Patrol,
                        AirTarget::Bombers,
                        fx(2),
                        point_three,
                        hundredths(60)
                    ),
                ],
                "share {share} seed {seed}"
            );
        }
    }
}

#[test]
fn the_guns_split_k_before_and_the_rest_after_with_the_breakdowns_of_the_hand_calculation() {
    for (share, before, after) in SHARES {
        let world = run_seed(&case_1(share), 1942);
        let parts = guns(&morning(&world));
        // A part with no shots records nothing.
        let want: Vec<(i64, bool)> = [(before, false), (after, true)]
            .into_iter()
            .filter(|(shots, _)| *shots > 0)
            .collect();
        let got: Vec<(i64, bool)> = parts
            .iter()
            .map(|e| (gun_of(e).0.trunc(), gun_of(e).4))
            .collect();
        assert_eq!(got, want, "share {share}");
        for part in &parts {
            let (shots, chance, expected, _, _) = gun_of(part);
            assert_eq!(chance, hundredths(5), "share {share}");
            assert_eq!(expected, shots * hundredths(5), "share {share}");
            let breakdown = part.breakdown.as_ref().expect("breakdowns are on");
            assert_eq!(breakdown.result, expected);
            assert_eq!(value_of(part, "shots"), shots);
            assert_eq!(value_of(part, "hit_chance"), hundredths(5));
            if share == "1" {
                // The breakdown of step 1.
                assert_eq!(names(part), vec!["shots", "hit_chance"]);
            } else {
                assert_eq!(
                    names(part),
                    vec![
                        "guns",
                        "anti_aircraft_before_attack_share",
                        "shots",
                        "hit_chance"
                    ]
                );
                assert_eq!(value_of(part, "guns"), fx(20));
                assert_eq!(
                    value_of(part, "anti_aircraft_before_attack_share"),
                    Fixed::from_ratio(
                        match share {
                            "0.5" => 50,
                            "0.33" => 33,
                            _ => 0,
                        },
                        100
                    )
                );
            }
        }
        // The expectations of the table: 1.0 at 1 and 0; 0.5 and 0.5 at 0.5;
        // 0.3 and 0.7 at 0.33.
        let expecteds: Vec<Fixed> = parts.iter().map(|e| gun_of(e).2).collect();
        let want: Vec<Fixed> = match share {
            "0.5" => vec![hundredths(50), hundredths(50)],
            "0.33" => vec![hundredths(30), hundredths(70)],
            _ => vec![hundredths(100)],
        };
        assert_eq!(expecteds, want, "share {share}");
    }
}

#[test]
fn at_a_share_of_a_half_the_events_come_in_the_order_and_cite_the_causes_of_the_hand_calculation() {
    let base = case_1("0.5");
    let mut seen_attack_and_loss_after = false;
    for seed in 1..=SEEDS {
        let world = run_seed(&base, seed);
        let events = morning(&world);
        let find = |keep: &dyn Fn(&EventKind) -> bool| -> Vec<&Event> {
            events.iter().filter(|e| keep(&e.kind)).collect()
        };
        let launch = find(&|k| matches!(k, EventKind::StrikeLaunched { .. }))[0].id;
        let sent = find(&|k| matches!(k, EventKind::EscortsSent { .. }))[0].id;
        let fights = find(&|k| matches!(k, EventKind::AirFight { .. }));
        let gun_events = find(&is_guns);
        assert_eq!(gun_events.len(), 2, "seed {seed}");
        let (before, after) = (gun_events[0], gun_events[1]);
        assert_eq!(
            (gun_of(before).0, gun_of(before).4),
            (fx(10), false),
            "seed {seed}"
        );
        assert_eq!(
            (gun_of(after).0, gun_of(after).4),
            (fx(10), true),
            "seed {seed}"
        );
        let attacks = find(&|k| matches!(k, EventKind::AirAttack { .. }));
        // Group 2 loses at most 2 to the patrol and at most 12 in all: the
        // attack happens unless the fire before and the patrol took all 12,
        // which they cannot (2 + 10 shots of the patrol and the guns' 10
        // before the attack could, in principle: allow it).
        let lost: Vec<&Event> = find(&|k| matches!(k, EventKind::AircraftLost { .. }));
        // Before the attack: launch, escorts, the three fights, the fire
        // before.
        let mut order = vec![launch, sent];
        order.extend(fights.iter().map(|e| e.id));
        order.push(before.id);
        assert!(order.windows(2).all(|p| p[0] < p[1]), "seed {seed}");
        // The fire before cites the launch alone and its losses cite it.
        assert_eq!(before.causes, vec![Cause::Event(launch)], "seed {seed}");
        assert_eq!(after.causes[0], Cause::Event(launch), "seed {seed}");
        let group_2_losses: Vec<&Event> = lost
            .iter()
            .copied()
            .filter(|e| matches!(e.kind, EventKind::AircraftLost { air_group, .. } if air_group.raw() == 2))
            .collect();
        match attacks.first() {
            Some(attack) => {
                // The attack comes after the fire before and its losses, and
                // before the fire after; it cites the launch and the losses
                // of group 2 recorded before it, and none after.
                assert!(before.id < attack.id && attack.id < after.id, "seed {seed}");
                let mut want = vec![Cause::Event(launch)];
                want.extend(
                    group_2_losses
                        .iter()
                        .filter(|e| e.id < attack.id)
                        .map(|e| Cause::Event(e.id)),
                );
                assert_eq!(attack.causes, want, "seed {seed}");
                // The fire after cites the launch and the attack.
                assert_eq!(
                    after.causes,
                    vec![Cause::Event(launch), Cause::Event(attack.id)],
                    "seed {seed}"
                );
                // A loss after the attack cites the fire after it and
                // comes after it and before the damage.
                let first_damage: Option<EventId> = find(&|k| {
                    matches!(
                        k,
                        EventKind::ShipDamaged { .. } | EventKind::ReportReceived { .. }
                    )
                })
                .first()
                .map(|e| e.id);
                for loss in group_2_losses.iter().filter(|e| e.id > attack.id) {
                    assert_eq!(loss.causes, vec![Cause::Event(after.id)], "seed {seed}");
                    assert!(after.id < loss.id, "seed {seed}");
                    assert!(first_damage.is_none_or(|id| loss.id < id), "seed {seed}");
                    seen_attack_and_loss_after = true;
                }
                // Nothing the attack cites is a loss after it.
                assert!(
                    after.id > attack.id && gun_of(after).4,
                    "the fire after is recorded after the attack"
                );
            }
            None => {
                // No bomber was left: the fire after cites the launch alone.
                assert_eq!(after.causes, vec![Cause::Event(launch)], "seed {seed}");
            }
        }
        // Every loss of group 2 cites one of the three rolls that took it.
        for loss in &group_2_losses {
            assert!(
                loss.causes == vec![Cause::Event(before.id)]
                    || loss.causes == vec![Cause::Event(after.id)]
                    || fights
                        .iter()
                        .any(|f| loss.causes == vec![Cause::Event(f.id)]),
                "seed {seed}"
            );
        }
    }
    assert!(seen_attack_and_loss_after, "some seed loses a bomber after");
}

// ---------------------------------------------------------------------
// Case 1: the properties.
// ---------------------------------------------------------------------

#[test]
fn over_1000_seeds_the_guns_hits_losses_and_claims_are_the_same_at_every_share() {
    let runs: Vec<Vec<Morning>> = SHARES
        .iter()
        .map(|(share, _, _)| {
            let base = case_1(share);
            (1..=SEEDS)
                .map(|seed| summarize(&run_seed(&base, seed)))
                .collect()
        })
        .collect();
    let at_1 = &runs[0];
    for index in 0..SEEDS as usize {
        let seed = index as u64 + 1;
        let total = |run: &Morning| run.before_hits + run.after_hits;
        for (which, run) in runs.iter().enumerate() {
            let run = &run[index];
            assert_eq!(total(run), total(&at_1[index]), "seed {seed} share {which}");
            assert_eq!(run.lost, at_1[index].lost, "seed {seed} share {which}");
            // The patrol's rolls do not depend on the share.
            assert_eq!(run.patrol_at_bombers, at_1[index].patrol_at_bombers);
            // The attack's shots: the bombers the patrol left, less the hits
            // before the attack, never below 0.
            let left = 12_i64 - i64::from(run.patrol_at_bombers);
            assert_eq!(
                run.attack_shots,
                (left - i64::from(run.before_hits)).max(0),
                "seed {seed} share {which}"
            );
        }
        // The first k of 20 draws: the hits before grow with k and the
        // total is the same.
        let before: Vec<u32> = runs.iter().map(|r| r[index].before_hits).collect();
        assert!(
            before[3] == 0
                && before[3] <= before[2]
                && before[2] <= before[1]
                && before[1] <= before[0],
            "seed {seed}: {before:?}"
        );
        // The defender's claim is the same, and so is what the raiders claim
        // to have shot down (their claimed hits follow the attack).
        let claims = |run: &Morning| -> (Vec<EventKind>, Vec<u32>) {
            let allied: Vec<EventKind> = run
                .claims
                .iter()
                .filter(|k| {
                    matches!(
                        k,
                        EventKind::ClaimReceived {
                            side: Side::Allied,
                            ..
                        }
                    )
                })
                .cloned()
                .collect();
            let japanese_down: Vec<u32> = run
                .claims
                .iter()
                .filter_map(|k| match k {
                    EventKind::ClaimReceived {
                        side: Side::Japanese,
                        claimed_shot_down,
                        ..
                    } => Some(*claimed_shot_down),
                    _ => None,
                })
                .collect();
            (allied, japanese_down)
        };
        for run in &runs {
            assert_eq!(claims(&run[index]), claims(&at_1[index]), "seed {seed}");
        }
    }
    // The attack's shots in all are within the hand calculation's range.
    for ((share_runs, (low, high)), (share, _, _)) in
        runs.iter().zip(ALLOWED_ATTACK_SHOTS).zip(SHARES)
    {
        let total: i64 = share_runs.iter().map(|run| run.attack_shots).sum();
        let total = u64::try_from(total).expect("not negative");
        assert!(
            (low..=high).contains(&total),
            "share {share}: {total} not in {low}..={high}"
        );
    }
}

#[test]
fn the_guns_hits_are_the_first_k_draws_of_the_one_stream_before_the_attack_and_the_rest_after() {
    for (share, k, _) in SHARES {
        let base = case_1(share);
        for seed in 1..=SEEDS {
            let run = summarize(&run_seed(&base, seed));
            let draws = gun_draws(seed, 20, hundredths(5));
            let k = k as usize;
            let before = draws[..k].iter().filter(|hit| **hit).count();
            let after = draws[k..].iter().filter(|hit| **hit).count();
            assert_eq!(
                (run.before_hits as usize, run.after_hits as usize),
                (before, after),
                "share {share} seed {seed}"
            );
        }
    }
}

#[test]
fn a_bomb_hit_does_10_points_and_the_attack_stands_whatever_the_guns_do_after_it() {
    for (share, _, _) in SHARES {
        let base = case_1(share);
        for seed in 1..=SEEDS {
            let world = run_seed(&base, seed);
            let events = morning(&world);
            let run = summarize(&world);
            // The points of the morning's bomb hits on the carrier: 10 a
            // hit, however many bombers the guns shot down after the attack.
            let points: Fixed = events
                .iter()
                .map(|e| match e.kind {
                    EventKind::ShipDamaged { points, .. } => points,
                    _ => Fixed::ZERO,
                })
                .fold(Fixed::ZERO, |a, b| a + b);
            assert_eq!(
                points,
                fx(10 * i64::from(run.attack_hits)),
                "share {share} seed {seed}"
            );
        }
    }
}

#[test]
fn at_a_share_of_1_the_log_has_no_part_after_the_attack_and_is_the_log_of_the_frozen_figures() {
    // The frozen figures, with only the contact age changed, are the figures
    // as step 1 had them: the share in them is 1. Every event, cause and
    // breakdown of every seed is the same as at "1" written out.
    let frozen_only_the_age = common::altered_reference("guns_frozen", |dir| {
        common::edit_file_by(
            dir,
            "air_combat.toml",
            &[(
                "strike_contact_max_age_pulses = 0",
                "strike_contact_max_age_pulses = 1",
            )],
        );
    });
    let text = scenario_text();
    let frozen =
        common::from_text_with(&text, &scenario_path(), &frozen_only_the_age).expect("loads");
    let at_1 = case_1("1");
    for seed in 1..=SEEDS {
        let (a, b) = (run_seed(&frozen, seed), run_seed(&at_1, seed));
        assert_eq!(a.events().len(), b.events().len(), "seed {seed}");
        for (x, y) in a.events().iter().zip(b.events().iter()) {
            assert_eq!(x, y, "seed {seed}");
            assert!(!x.kind.to_string().contains("after the attack"));
        }
        assert_eq!(a.day_hashes(), b.day_hashes(), "seed {seed}");
        // One gun event in the morning, as in step 1, and the breakdown of
        // step 1.
        let parts = guns(&morning(&a));
        assert_eq!(parts.len(), 1, "seed {seed}");
        assert_eq!(names(&parts[0]), vec!["shots", "hit_chance"]);
    }
}

// ---------------------------------------------------------------------
// Case 2: a lone bomber.
// ---------------------------------------------------------------------

#[test]
fn a_lone_bomber_attacks_with_0_95_to_the_k_and_is_lost_with_the_same_chance_at_every_share() {
    // (share, k, chance it attacks, expected bomb hits): 0.95^20, 0.95^10, 1.
    let cases = [
        (
            "1",
            20_usize,
            Fixed::from_raw(358_486),
            Fixed::from_raw(71_697),
        ),
        (
            "0.5",
            10,
            Fixed::from_raw(598_737),
            Fixed::from_raw(119_747),
        ),
        ("0", 0, Fixed::ONE, hundredths(20)),
    ];
    let mut lost_at = Vec::new();
    for (share, k, attacks, hits) in cases {
        let base = case_2(share);
        let (mut attacked, mut bomb_hits, mut lost) = (0_u64, 0_u64, Vec::new());
        for seed in 1..=SEEDS {
            let world = run_seed(&base, seed);
            let run = summarize(&world);
            let draws = gun_draws(seed, 20, hundredths(5));
            // It attacks when none of the first k shots hit it, and it is
            // lost when any of the 20 does.
            let before = draws[..k].iter().any(|hit| *hit);
            let any = draws.iter().any(|hit| *hit);
            assert_eq!(
                run.attack_shots,
                i64::from(!before),
                "share {share} seed {seed}"
            );
            assert_eq!(run.lost[1], u32::from(any), "share {share} seed {seed}");
            attacked += u64::from(run.attack_shots == 1);
            bomb_hits += u64::from(run.attack_hits);
            lost.push(run.lost[1]);
            // Nothing else is lost: no escort, no patrol.
            assert_eq!((run.lost[0], run.lost[2]), (0, 0));
        }
        lost_at.push(lost);
        let within = |count: u64, p: Fixed| {
            let n = SEEDS as f64;
            let p = p.raw() as f64 / 1_000_000.0;
            let mean = n * p;
            let sd = (n * p * (1.0 - p)).sqrt();
            ((count as f64) - mean).abs() <= 4.0 * sd + 1e-9
        };
        assert!(
            within(attacked, attacks),
            "share {share}: {attacked} attacked"
        );
        assert!(
            within(bomb_hits, hits),
            "share {share}: {bomb_hits} bomb hits"
        );
    }
    // Lost by the same seeds at every share, with chance 1 - 0.95^20.
    assert_eq!(lost_at[0], lost_at[1]);
    assert_eq!(lost_at[0], lost_at[2]);
    let lost: u64 = lost_at[0].iter().map(|n| u64::from(*n)).sum();
    let (n, p) = (SEEDS as f64, 0.641_514_f64);
    assert!(
        ((lost as f64) - n * p).abs() <= 4.0 * (n * p * (1.0 - p)).sqrt(),
        "{lost} lost"
    );
}

// ---------------------------------------------------------------------
// Case 3: a raid on a base.
// ---------------------------------------------------------------------

#[test]
fn a_raid_on_a_base_with_8_guns_splits_them_4_and_4_and_the_fire_after_cites_the_base_attack() {
    let base = case_3("0.5");
    let mut total_shots = 0_u64;
    for seed in 1..=SEEDS {
        let world = run_seed(&base, seed);
        let events = morning(&world);
        let launch = events
            .iter()
            .find(|e| matches!(e.kind, EventKind::BaseStrikeLaunched { .. }))
            .expect("the raid launches")
            .id;
        let parts = guns(&events);
        assert_eq!(parts.len(), 2, "seed {seed}");
        for (part, after) in parts.iter().zip([false, true]) {
            assert_eq!(
                gun_of(part),
                (fx(4), hundredths(5), hundredths(20), gun_of(part).3, after),
                "seed {seed}"
            );
            assert!(matches!(part.kind, EventKind::BaseAntiAircraftFire { .. }));
        }
        let attacked = events
            .iter()
            .find(|e| matches!(e.kind, EventKind::BaseAttacked { .. }))
            .expect("the raid attacks");
        let EventKind::BaseAttacked {
            shots,
            chance,
            expected,
            hits,
            hits_on_aircraft,
            hits_on_supply,
            ..
        } = attacked.kind
        else {
            unreachable!()
        };
        // 15 less the hits before; each landing by the base hit draws as now.
        assert_eq!(
            shots,
            fx(15 - i64::from(gun_of(&parts[0]).3)),
            "seed {seed}"
        );
        assert_eq!(chance, hundredths(20));
        assert_eq!(expected, shots * hundredths(20));
        assert!(hits_on_aircraft + hits_on_supply <= hits);
        total_shots += u64::try_from(shots.trunc()).expect("shots");
        // Order and causes: launch, the fire before, the attack, the fire
        // after (launch and the attack), then what the bombs did.
        assert_eq!(parts[0].causes, vec![Cause::Event(launch)]);
        assert_eq!(
            parts[1].causes,
            vec![Cause::Event(launch), Cause::Event(attacked.id)]
        );
        assert!(launch < parts[0].id && parts[0].id < attacked.id && attacked.id < parts[1].id);
        assert_eq!(
            attacked.causes[0],
            Cause::Event(launch),
            "the attack cites the launch first"
        );
        assert!(
            !attacked.causes.contains(&Cause::Event(parts[1].id)),
            "seed {seed}"
        );
    }
    // 15 less 0.2 expected hits before the attack: 14,800 over 1,000 seeds,
    // standard deviation sqrt(1000 x 4 x 0.05 x 0.95) = 13.8.
    assert!(
        (14_745..=14_855).contains(&total_shots),
        "{total_shots} shots in all"
    );
    // At 1 it is 8 before and none after: 14,600 expected (sd 19.5). At 0 it
    // is 15 every time.
    for (share, low, high, parts) in [("1", 14_522, 14_678, 1), ("0", 15_000, 15_000, 1)] {
        let base = case_3(share);
        let mut total = 0_u64;
        for seed in 1..=SEEDS {
            let world = run_seed(&base, seed);
            let events = morning(&world);
            assert_eq!(guns(&events).len(), parts, "share {share} seed {seed}");
            assert_eq!(
                gun_of(&guns(&events)[0]).4,
                share == "0",
                "share {share} seed {seed}"
            );
            let attacked = events
                .iter()
                .find_map(|e| match e.kind {
                    EventKind::BaseAttacked { shots, .. } => Some(shots.trunc()),
                    _ => None,
                })
                .expect("the raid attacks");
            total += u64::try_from(attacked).expect("shots");
        }
        assert!(
            (low..=high).contains(&total),
            "share {share}: {total} shots in all"
        );
    }
}

// ---------------------------------------------------------------------
// Pinned outcomes.
// ---------------------------------------------------------------------

/// PINNED, not calculated. (seed, share, gun hits before, gun hits after,
/// attack shots, bomb hits, aircraft lost by group 1, 2 and 3.) Seed 1942 is
/// the scenario's own: the guns miss all 20 shots on it, so it is the same at
/// every share. Seed 10 is the first seed on which the guns hit and the share
/// moves the hits across the attack.
#[allow(clippy::type_complexity)]
const PINNED: [(u64, &str, u32, u32, i64, u32, [u32; 3]); 8] = [
    (1942, "1", 0, 0, 11, 1, [2, 1, 1]),
    (1942, "0.5", 0, 0, 11, 1, [2, 1, 1]),
    (1942, "0.33", 0, 0, 11, 1, [2, 1, 1]),
    (1942, "0", 0, 0, 11, 1, [2, 1, 1]),
    (10, "1", 3, 0, 8, 1, [1, 4, 1]),
    (10, "0.5", 2, 1, 9, 2, [1, 4, 1]),
    (10, "0.33", 1, 2, 10, 2, [1, 4, 1]),
    (10, "0", 0, 3, 11, 2, [1, 4, 1]),
];

#[test]
fn pinned_the_outcomes_of_seeds_1942_and_10_in_case_1_at_each_share() {
    let rows: Vec<_> = PINNED
        .iter()
        .map(|(seed, share, ..)| {
            let run = summarize(&run_seed(&case_1(share), *seed));
            (
                *seed,
                *share,
                run.before_hits,
                run.after_hits,
                run.attack_shots,
                run.attack_hits,
                run.lost,
            )
        })
        .collect();
    assert_eq!(rows, PINNED.to_vec());
}
