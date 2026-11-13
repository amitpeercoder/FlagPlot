//! Whom the patrol fires at (decision 0080): the escorts draw off a share of
//! the patrol one for one, and the rest is split between the escorts not
//! drawn off and the bombers in proportion to their numbers.
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
//! ## The rule, as worked here
//!
//! For a raid: P its share of the patrol (decision 0033), E its escorts, B
//! its bombers at the start of the step, s = `patrol_drawn_off_share`.
//!
//! - D = floor(s × min(P, E)), drawn off one for one.
//! - R = P − D.
//! - X = min(floor(R × (E − D) / (E − D + B)), E − D), in whole numbers.
//! - Shots at the escorts: D + X. At the bombers: R − X.
//!
//! s is 1 in the frozen step 1 figures (step 1's rule) and is set to 0.5
//! and 0 by `common::altered_reference`. A patrol roll with no shots records
//! nothing.
//!
//! ## The scenario
//!
//! `data/scenarios/19b_whom_the_patrol_fires_at.toml`, one day, on the
//! frozen figures with `strike_contact_max_age_pulses` at 1 (as in 13a and
//! 19a), so the Japanese strike is launched in the morning at a contact the
//! side starts holding, with no search. Everything asserted by hand is of
//! the morning pulse of day 1.
//!
//! The Allied task force 1 is one "Placeholder carrier" (class 4, no guns)
//! with group 1, Placeholder fighters (type 4) on combat air patrol with a
//! patrol share of 1. The Japanese task force 2, within 250 nm, is one
//! "Placeholder carrier" (class 4) with group 2, Placeholder dive bombers
//! (type 3) on naval strike, and group 3, Placeholder fighters on combat air
//! patrol with a patrol share of 0, all of which escort. No ship has guns,
//! so the bombers are fired at by the patrol alone. No type has pilot armour
//! or self-sealing tanks: every chance at aircraft is `fighter_hit_chance`,
//! 0.3. The cases set the groups' ready aircraft; the scenario file is case
//! (a).
//!
//! Each line below is: shots at the escorts / shots at the bombers
//! (expected hits at 0.3).
//!
//! ## (a) More escorts than patrol: P = 4, E = 6, B = 10
//!
//! The escorts fire 6 shots at the patrol: 1.8 expected, at every s.
//!
//! | s | D | R | X | Shots | Expected hits |
//! | --- | --- | --- | --- | --- | --- |
//! | 1 | 4 | 0 | 0 | 4 / 0 | 1.2 / 0 |
//! | 0.5 | 2 | 2 | floor(2 × 4 / 14) = 0 | 2 / 2 | 0.6 / 0.6 |
//! | 0 | 0 | 4 | floor(4 × 6 / 16) = 1 | 1 / 3 | 0.3 / 0.9 |
//!
//! ## (b) Fewer escorts than patrol: P = 10, E = 4, B = 12
//!
//! The escorts fire 4 shots: 1.2 expected.
//!
//! | s | D | R | X | Shots | Expected hits |
//! | --- | --- | --- | --- | --- | --- |
//! | 1 | 4 | 6 | 0 | 4 / 6 | 1.2 / 1.8 |
//! | 0.5 | 2 | 8 | floor(8 × 2 / 14) = 1 | 3 / 7 | 0.9 / 2.1 |
//! | 0 | 0 | 10 | floor(10 × 4 / 16) = 2 | 2 / 8 | 0.6 / 2.4 |
//!
//! ## (c) No escort: P = 6, E = 0, B = 8
//!
//! At every s: 0 / 6, 1.8 expected at the bombers. No escort fires and no
//! patrol roll at escorts is recorded.
//!
//! ## (d) Two raids in one pulse share a patrol of 10
//!
//! The Japanese task force 2 has two carriers, each its own place and so its
//! own raid: raid 1 (the lower first strike group) B = 8, E = 2; raid 2
//! B = 6, E = 4. Both strike task force 1. Each raid's aircraft are 10, so
//! each share is 10 × 10 / 20 = 5, with none left over. The escorts' 6 shots
//! in all at the patrol: 1.8 expected, taken from the one patrol pool.
//!
//! | s | Raid 1: D, R, X, shots | Raid 2: D, R, X, shots |
//! | --- | --- | --- |
//! | 1 | 2, 3, 0: 2 / 3 | 4, 1, 0: 4 / 1 |
//! | 0.5 | 1, 4, floor(4 × 1 / 9) = 0: 1 / 4 | 2, 3, floor(3 × 2 / 8) = 0: 2 / 3 |
//! | 0 | 0, 5, floor(5 × 2 / 10) = 1: 1 / 4 | 0, 5, floor(5 × 4 / 10) = 2: 2 / 3 |
//!
//! ## (e) Henderson Field in small: a raid on a base, P = 13, E = 15, B = 6
//!
//! A Japanese base with 6 Placeholder dive bombers on base strike and 15
//! Placeholder fighters escorting (patrol share 0), against an Allied base
//! with 13 Placeholder fighters on patrol and no guns. The escorts fire 15
//! shots at the patrol: 4.5 expected.
//!
//! | s | D | R | X | Shots | Expected hits |
//! | --- | --- | --- | --- | --- | --- |
//! | 1 | 13 | 0 | 0 | 13 / 0 | 3.9 / 0 |
//! | 0.5 | floor(6.5) = 6 | 7 | floor(7 × 9 / 15) = 4 | 10 / 3 | 3.0 / 0.9 |
//! | 0 | 0 | 13 | floor(13 × 15 / 21) = 9 | 9 / 4 | 2.7 / 1.2 |
//!
//! At s = 1 the bombers cannot be touched, which is today's rule; the record
//! of Ryujo's raid on 24 August has 4 attack planes and 3 fighters lost
//! (`engagement_outcomes` 271).
//!
//! ## (f) A share larger than the raid: P = 20, E = 2, B = 3
//!
//! | s | D | R | X | Shots |
//! | --- | --- | --- | --- | --- |
//! | 1 | 2 | 18 | 0 | 2 / 18 |
//! | 0 | 0 | 20 | min(floor(20 × 2 / 5) = 8, 2) = 2 | 2 / 18 |
//!
//! The hits on the bombers are capped at the 3 there.
//!
//! **The breakdown.** Each patrol roll's breakdown lists the whole patrol,
//! the raid's aircraft, all the raids' aircraft and the raid's share, then
//! its shots and the chance 0.3, as now; when s is below 1 it also lists s,
//! D and X.
//!
//! # Properties
//!
//! - At s = 1 every seed's events, in every case, are those of the frozen
//!   figures before the rule, and every scenario that existed before the
//!   rule keeps its events on the frozen figures: every other hand
//!   calculation passes untouched.
//! - The shots in the tables above are the same on every seed (they are not
//!   drawn); the escorts' shots at the patrol are E at every s.
//! - Over seeds 1 to 1000, case (e), the hits summed are within four
//!   standard deviations of 1,000 times the expectation (a binomial of the
//!   shots at 0.3 for each roll; no pool is exhausted):
//!
//! | s | Roll | Expected total | Standard deviation | Allowed |
//! | --- | --- | --- | --- | --- |
//! | 0.5 | At the escorts (10) | 3,000 | sqrt(1,000 × 2.1) = 45.8 | 2,817 to 3,183 |
//! | 0.5 | At the bombers (3) | 900 | sqrt(1,000 × 0.63) = 25.1 | 800 to 1,000 |
//! | 0 | At the escorts (9) | 2,700 | sqrt(1,000 × 1.89) = 43.5 | 2,526 to 2,874 |
//! | 0 | At the bombers (4) | 1,200 | sqrt(1,000 × 0.84) = 29.0 | 1,084 to 1,316 |
//!
//! # Pinned outcomes
//!
//! What the scenario's own seed (1942) gives in each case at each s.
//! Pinned, not calculated: no hand calculation can say which draw succeeds.
//! A change here means a stream or the order of draws moved; they are
//! remade only with a deliberate change of the rule (docs/step2-spec.md,
//! "Tests while figures move", 7). Filled in by the build from the first
//! run, and checked against the properties above before they are pinned.
//!
//! Each row: the hits of every `AirFight` roll of the morning in the order
//! they are logged (a roll with no shots is not logged), then the aircraft
//! lost by groups 1 to 5.
//!
//! | Case | s | Hits of each roll, in order | Lost by groups 1 to 5 |
//! | --- | --- | --- | --- |
//! | (a) | 1 | 2, 1 | 2, 0, 1, 0, 0 |
//! | (a) | 0.5 | 2, 0, 1 | 2, 1, 0, 0, 0 |
//! | (a) | 0 | 2, 0, 1 | 2, 1, 0, 0, 0 |
//! | (b) | 1 | 2, 1, 4 | 2, 4, 1, 0, 0 |
//! | (b) | 0.5 | 2, 1, 4 | 2, 4, 1, 0, 0 |
//! | (b) | 0 | 2, 0, 5 | 2, 5, 0, 0, 0 |
//! | (c) | 1 | 2 | 0, 2, 0, 0, 0 |
//! | (c) | 0.5 | 2 | 0, 2, 0, 0, 0 |
//! | (c) | 0 | 2 | 0, 2, 0, 0, 0 |
//! | (d) | 1 | 1, 0, 2, 1, 1, 0 | 2, 2, 0, 0, 1 |
//! | (d) | 0.5 | 1, 0, 2, 1, 0, 1 | 2, 2, 0, 1, 0 |
//! | (d) | 0 | 1, 0, 2, 1, 0, 1 | 2, 2, 0, 1, 0 |
//! | (e) | 1 | 4, 3 | 0, 3, 4, 0, 0 |
//! | (e) | 0.5 | 4, 2, 1 | 1, 2, 4, 0, 0 |
//! | (e) | 0 | 4, 2, 1 | 1, 2, 4, 0, 0 |
//! | (f) | 1 | 1, 0, 9 | 1, 3, 0, 0, 0 |
//! | (f) | 0 | 1, 0, 9 | 1, 3, 0, 0, 0 |
//!
//! Checked against the properties before pinning: no roll has more hits than
//! shots; each group's losses are the hits on it (case (f)'s 9 hits at the 3
//! bombers are 3 lost); and (f) is the same at s = 0 and 1, the shots and
//! the stream being the same.

mod common;

use std::fs;
use std::path::PathBuf;

use sim_core::events::{AirFirer, AirTarget, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::time::Pulse;
use sim_core::World;
use sim_data::{LoadOptions, Scenario};

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
const SCENARIO: &str = "19b_whom_the_patrol_fires_at.toml";
const SEEDS: u64 = 1000;
/// The shares of the cases, as written in the figures file.
const SHARES: [&str; 3] = ["1", "0.5", "0"];

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

/// The frozen figures with the changes the hand calculation names: a contact
/// the side starts with can be struck in the morning, and the share is
/// `share` (written as the figures file writes it), or untouched if `None`.
fn options(share: Option<&str>) -> LoadOptions {
    common::altered_reference("patrol_fires_at", |dir| {
        let mut edits = vec![(
            "strike_contact_max_age_pulses = 0".to_string(),
            "strike_contact_max_age_pulses = 1".to_string(),
        )];
        if let Some(share) = share {
            edits.push((
                "patrol_drawn_off_share = \"1\"".to_string(),
                format!("patrol_drawn_off_share = \"{share}\""),
            ));
        }
        let edits: Vec<(&str, &str)> = edits
            .iter()
            .map(|(a, b)| (a.as_str(), b.as_str()))
            .collect();
        common::edit_file_by(dir, "air_combat.toml", &edits);
    })
}

/// The scenario file with the three groups' ready aircraft changed: the
/// Allied fighters (group 1), the Japanese bombers (2) and fighters (3).
fn with_ready(patrol: u32, bombers: u32, escorts: u32) -> String {
    let text = scenario_text();
    let text = common::replace_once(&text, "ready = 4", "ready = @1");
    let text = common::replace_once(&text, "ready = 10", "ready = @2");
    let text = common::replace_once(&text, "ready = 6", "ready = @3");
    text.replace("@1", &patrol.to_string())
        .replace("@2", &bombers.to_string())
        .replace("@3", &escorts.to_string())
}

/// Case (d): two Japanese carriers in task force 2, each with its own
/// bombers and escorts, against a patrol of 10.
fn case_d_text() -> String {
    let mut text = with_ready(10, 8, 2);
    text.push_str(
        r#"
# The second Japanese carrier, with its own bombers (6) and escorts (4).
[[ship]]
id = 3
name = "Placeholder second carrier"
class = 4
side = "japanese"
fuel_t = 3000
task_force = 2

[[air_group]]
id = 4
name = "Placeholder bombing squadron 2"
side = "japanese"
ship = 3
aircraft_type = 3
ready = 6

[[air_group]]
id = 5
name = "Placeholder fighting squadron C"
side = "japanese"
ship = 3
aircraft_type = 4
ready = 4

[[order]]
day = 1
side = "japanese"
air_group = 4
kind = "set_air_mission"
mission = "naval_strike"

[[order]]
day = 1
side = "japanese"
air_group = 5
kind = "set_air_mission"
mission = "combat_air_patrol"

[[order]]
day = 1
side = "japanese"
air_group = 5
kind = "set_patrol_share"
share = "0"
"#,
    );
    text
}

/// Case (e): 6 dive bombers and 15 escorts of a Japanese base raid an
/// Allied base with 13 fighters on patrol and no guns.
fn case_e_text() -> String {
    r#"name = "Scenario 19b, case (e): Henderson Field in small"
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

[[air_group]]
id = 1
name = "Placeholder bomber group"
side = "japanese"
base = 1
aircraft_type = 3
ready = 6

[[air_group]]
id = 2
name = "Placeholder escort group"
side = "japanese"
base = 1
aircraft_type = 4
ready = 15

[[air_group]]
id = 3
name = "Placeholder defending fighters"
side = "allied"
base = 2
aircraft_type = 4
ready = 13

[[order]]
day = 1
side = "japanese"
air_group = 1
kind = "set_air_mission"
mission = "base_strike"
target_base = 2

[[order]]
day = 1
side = "japanese"
air_group = 2
kind = "set_air_mission"
mission = "combat_air_patrol"

[[order]]
day = 1
side = "japanese"
air_group = 2
kind = "set_patrol_share"
share = "0"

[[order]]
day = 1
side = "allied"
air_group = 3
kind = "set_air_mission"
mission = "combat_air_patrol"
"#
    .to_string()
}

fn case_text(case: char) -> String {
    match case {
        'a' => scenario_text(),
        'b' => with_ready(10, 12, 4),
        'c' => with_ready(6, 8, 0),
        'd' => case_d_text(),
        'e' => case_e_text(),
        'f' => with_ready(20, 3, 2),
        other => panic!("no case {other}"),
    }
}

fn load_with(case: char, options: &LoadOptions) -> Scenario {
    common::from_text_with(&case_text(case), &scenario_path(), options).expect("the case loads")
}

fn load(case: char, share: &str) -> Scenario {
    load_with(case, &options(Some(share)))
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

/// One logged `AirFight`: (firing, at, firer group, target group, shots,
/// chance, expected, hits).
type Fight = (AirFirer, AirTarget, u32, u32, Fixed, Fixed, Fixed, u32);

fn fights(events: &[Event]) -> Vec<(Fight, Event)> {
    events
        .iter()
        .filter_map(|event| match event.kind {
            EventKind::AirFight {
                firing,
                at,
                firer,
                target,
                shots,
                chance,
                expected,
                hits,
                ..
            } => Some((
                (
                    firing,
                    at,
                    firer.raw(),
                    target.raw(),
                    shots,
                    chance,
                    expected,
                    hits,
                ),
                event.clone(),
            )),
            _ => None,
        })
        .collect()
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

/// A raid of a case: its first strike group, its first escort group (none
/// if it has no escort), E, the first patrol group, the whole patrol, the
/// raid's aircraft, all the raids' aircraft and the raid's share P.
struct Raid {
    strike: u32,
    escort: Option<u32>,
    e: u32,
    patrol_first: u32,
    patrol: u32,
    raid_aircraft: u32,
    all_aircraft: u32,
    p: u32,
}

fn raids(case: char) -> Vec<Raid> {
    let raid = |strike, escort, e, patrol_first, patrol, raid_aircraft, all_aircraft, p| Raid {
        strike,
        escort,
        e,
        patrol_first,
        patrol,
        raid_aircraft,
        all_aircraft,
        p,
    };
    match case {
        'a' => vec![raid(2, Some(3), 6, 1, 4, 16, 16, 4)],
        'b' => vec![raid(2, Some(3), 4, 1, 10, 16, 16, 10)],
        'c' => vec![raid(2, None, 0, 1, 6, 8, 8, 6)],
        'd' => vec![
            raid(2, Some(3), 2, 1, 10, 10, 20, 5),
            raid(4, Some(5), 4, 1, 10, 10, 20, 5),
        ],
        'e' => vec![raid(1, Some(2), 15, 3, 13, 21, 21, 13)],
        'f' => vec![raid(2, Some(3), 2, 1, 20, 5, 5, 20)],
        other => panic!("no case {other}"),
    }
}

/// The hand calculation for each raid of a case at a share: (D, X, shots at
/// the escorts, shots at the bombers).
fn split_of(case: char, share: &str) -> Vec<(u32, u32, u32, u32)> {
    match (case, share) {
        ('a', "1") => vec![(4, 0, 4, 0)],
        ('a', "0.5") => vec![(2, 0, 2, 2)],
        ('a', "0") => vec![(0, 1, 1, 3)],
        ('b', "1") => vec![(4, 0, 4, 6)],
        ('b', "0.5") => vec![(2, 1, 3, 7)],
        ('b', "0") => vec![(0, 2, 2, 8)],
        ('c', _) => vec![(0, 0, 0, 6)],
        ('d', "1") => vec![(2, 0, 2, 3), (4, 0, 4, 1)],
        ('d', "0.5") => vec![(1, 0, 1, 4), (2, 0, 2, 3)],
        ('d', "0") => vec![(0, 1, 1, 4), (0, 2, 2, 3)],
        ('e', "1") => vec![(13, 0, 13, 0)],
        ('e', "0.5") => vec![(6, 4, 10, 3)],
        ('e', "0") => vec![(0, 9, 9, 4)],
        ('f', "1") => vec![(2, 0, 2, 18)],
        ('f', "0") => vec![(0, 2, 2, 18)],
        other => panic!("no row for {other:?}"),
    }
}

/// Every (case, share) the hand calculation gives.
fn every_row() -> Vec<(char, &'static str)> {
    let mut rows = Vec::new();
    for case in ['a', 'b', 'c', 'd', 'e'] {
        for share in SHARES {
            rows.push((case, share));
        }
    }
    rows.push(('f', "1"));
    rows.push(('f', "0"));
    rows
}

/// What the hand calculation says a roll is: (firing, at, firer group,
/// target group, shots).
type Wanted = (AirFirer, AirTarget, u32, u32, Fixed);

/// The fights of the hand calculation, in order, for a case and share:
/// for each raid the escorts at the patrol, the patrol at the escorts and
/// at the bombers, a roll with no shots left out.
fn wanted_fights(case: char, share: &str) -> Vec<Wanted> {
    let mut wanted = Vec::new();
    for (raid, (_, _, at_escorts, at_bombers)) in raids(case).iter().zip(split_of(case, share)) {
        if let Some(escort) = raid.escort {
            wanted.push((
                AirFirer::Escorts,
                AirTarget::Patrol,
                escort,
                raid.patrol_first,
                fx(i64::from(raid.e)),
            ));
            if at_escorts > 0 {
                wanted.push((
                    AirFirer::Patrol,
                    AirTarget::Escorts,
                    raid.patrol_first,
                    escort,
                    fx(i64::from(at_escorts)),
                ));
            }
        }
        if at_bombers > 0 {
            wanted.push((
                AirFirer::Patrol,
                AirTarget::Bombers,
                raid.patrol_first,
                raid.strike,
                fx(i64::from(at_bombers)),
            ));
        }
    }
    wanted
}

// ---------------------------------------------------------------------
// The shots and the chance, at every seed.
// ---------------------------------------------------------------------

#[test]
fn the_shots_of_each_roll_are_those_of_the_hand_calculation_at_every_seed_and_share() {
    for (case, share) in every_row() {
        let base = load(case, share);
        let wanted = wanted_fights(case, share);
        for seed in 1..=SEEDS {
            let world = run_seed(&base, seed);
            let got = fights(&morning(&world));
            let shots: Vec<_> = got
                .iter()
                .map(|((firing, at, firer, target, shots, ..), _)| {
                    (*firing, *at, *firer, *target, *shots)
                })
                .collect();
            assert_eq!(shots, wanted, "case {case} share {share} seed {seed}");
            for ((_, _, _, _, shots, chance, expected, _), _) in &got {
                // No type has pilot armour or self-sealing tanks.
                assert_eq!(*chance, hundredths(30), "case {case} share {share}");
                assert_eq!(*expected, *shots * hundredths(30));
            }
        }
    }
}

#[test]
fn a_roll_with_no_shots_records_nothing() {
    // (a) at s = 1: the patrol has no shot at the bombers.
    let got = fights(&morning(&run_seed(&load('a', "1"), 1942)));
    assert_eq!(got.len(), 2);
    assert!(got.iter().all(|((_, at, ..), _)| *at != AirTarget::Bombers));
    // (a) at s = 0 has all three rolls.
    assert_eq!(fights(&morning(&run_seed(&load('a', "0"), 1942))).len(), 3);
    // (c): no escort, so no roll by the escorts and none at them, at any s.
    for share in SHARES {
        let got = fights(&morning(&run_seed(&load('c', share), 1942)));
        assert_eq!(got.len(), 1, "share {share}");
        let ((firing, at, _, _, shots, ..), _) = &got[0];
        assert_eq!(
            (*firing, *at, *shots),
            (AirFirer::Patrol, AirTarget::Bombers, fx(6))
        );
    }
    // (e) at s = 1: 13 at the escorts and none at the bombers.
    let got = fights(&morning(&run_seed(&load('e', "1"), 1942)));
    assert_eq!(got.len(), 2);
}

// ---------------------------------------------------------------------
// The breakdowns.
// ---------------------------------------------------------------------

#[test]
fn the_patrol_rolls_breakdowns_list_the_share_and_below_1_also_the_split() {
    for (case, share) in every_row() {
        let world = run_seed(&load(case, share), 1942);
        let rolls = fights(&morning(&world));
        let raids = raids(case);
        let splits = split_of(case, share);
        for (fight, event) in &rolls {
            // The escorts' roll at the patrol has the breakdown of step 1.
            if fight.0 == AirFirer::Escorts {
                assert_eq!(names(event), vec!["shots", "hit_chance"]);
                continue;
            }
            // The raid is the one whose first strike group or first escort
            // the patrol fired at.
            let index = raids
                .iter()
                .position(|raid| fight.3 == raid.strike || Some(fight.3) == raid.escort)
                .expect("a raid is fired at");
            let (d, x, ..) = splits[index];
            let r = &raids[index];
            let mut want = vec![
                "patrol",
                "raid_aircraft",
                "all_raids_aircraft",
                "patrol_share",
            ];
            if share != "1" {
                want.extend(["patrol_drawn_off_share", "drawn_off", "rest_at_escorts"]);
            }
            want.extend(["shots", "hit_chance"]);
            assert_eq!(names(event), want, "case {case} share {share}");
            assert_eq!(value_of(event, "patrol"), fx(i64::from(r.patrol)));
            assert_eq!(
                value_of(event, "raid_aircraft"),
                fx(i64::from(r.raid_aircraft))
            );
            assert_eq!(
                value_of(event, "all_raids_aircraft"),
                fx(i64::from(r.all_aircraft))
            );
            assert_eq!(value_of(event, "patrol_share"), fx(i64::from(r.p)));
            assert_eq!(value_of(event, "shots"), fight.4);
            assert_eq!(value_of(event, "hit_chance"), hundredths(30));
            if share != "1" {
                let s = if share == "0.5" {
                    hundredths(50)
                } else {
                    Fixed::ZERO
                };
                assert_eq!(value_of(event, "patrol_drawn_off_share"), s);
                assert_eq!(value_of(event, "drawn_off"), fx(i64::from(d)));
                assert_eq!(value_of(event, "rest_at_escorts"), fx(i64::from(x)));
            }
            assert_eq!(
                event.breakdown.as_ref().expect("on").result,
                fight.6,
                "the result is the expected hits"
            );
        }
    }
}

// ---------------------------------------------------------------------
// Properties over seeds.
// ---------------------------------------------------------------------

#[test]
fn over_1000_seeds_case_e_hits_are_within_four_standard_deviations_of_the_expectation() {
    // (share, allowed at the escorts, allowed at the bombers).
    for (share, (esc_low, esc_high), (bomb_low, bomb_high)) in [
        ("0.5", (2_817, 3_183), (800, 1_000)),
        ("0", (2_526, 2_874), (1_084, 1_316)),
    ] {
        let base = load('e', share);
        let (mut at_escorts, mut at_bombers) = (0_u64, 0_u64);
        for seed in 1..=SEEDS {
            let world = run_seed(&base, seed);
            for ((_, at, _, _, _, _, _, hits), _) in fights(&morning(&world)) {
                match at {
                    AirTarget::Escorts => at_escorts += u64::from(hits),
                    AirTarget::Bombers => at_bombers += u64::from(hits),
                    AirTarget::Patrol => {}
                }
            }
        }
        assert!(
            (esc_low..=esc_high).contains(&at_escorts),
            "share {share}: {at_escorts} hits at the escorts"
        );
        assert!(
            (bomb_low..=bomb_high).contains(&at_bombers),
            "share {share}: {at_bombers} hits at the bombers"
        );
    }
}

#[test]
fn a_share_larger_than_the_raid_caps_the_hits_on_the_bombers_at_the_3_there() {
    let base = load('f', "1");
    let mut capped = false;
    for seed in 1..=SEEDS {
        let events = morning(&run_seed(&base, seed));
        let hits_at_bombers: u32 = fights(&events)
            .iter()
            .filter(|((_, at, ..), _)| *at == AirTarget::Bombers)
            .map(|((.., hits), _)| *hits)
            .sum();
        // The ships have no guns: only the patrol brings the bombers down.
        assert_eq!(lost_by(&events, 2), hits_at_bombers.min(3), "seed {seed}");
        capped |= hits_at_bombers > 3;
    }
    assert!(capped, "some seed has more than 3 hits at the 3 bombers");
}

#[test]
fn at_a_share_of_1_every_case_has_the_events_of_the_frozen_figures() {
    // The frozen figures, with only the contact age changed, are the figures
    // as step 1 had them: the share in them is 1.
    let frozen_only_the_age = options(None);
    let at_1 = options(Some("1"));
    for case in ['a', 'b', 'c', 'd', 'e', 'f'] {
        let (frozen, one) = (
            load_with(case, &frozen_only_the_age),
            load_with(case, &at_1),
        );
        for seed in 1..=200 {
            let (a, b) = (run_seed(&frozen, seed), run_seed(&one, seed));
            assert_eq!(
                a.events().len(),
                b.events().len(),
                "case {case} seed {seed}"
            );
            for (x, y) in a.events().iter().zip(b.events().iter()) {
                assert_eq!(x, y, "case {case} seed {seed}");
            }
            assert_eq!(a.day_hashes(), b.day_hashes(), "case {case} seed {seed}");
        }
    }
}

// ---------------------------------------------------------------------
// Pinned outcomes.
// ---------------------------------------------------------------------

/// What a run gives in the morning: the hits of each roll, in order, and the
/// aircraft lost by groups 1 to 5.
fn outcome(world: &World) -> (Vec<u32>, [u32; 5]) {
    let events = morning(world);
    let hits = fights(&events)
        .iter()
        .map(|((.., hits), _)| *hits)
        .collect();
    let lost = [1, 2, 3, 4, 5].map(|group| lost_by(&events, group));
    (hits, lost)
}

type Pinned = (char, &'static str, Vec<u32>, [u32; 5]);

/// PINNED, not calculated. (case, share, hits of each roll in order, aircraft
/// lost by groups 1 to 5) at the scenario's own seed, 1942.
fn pinned() -> Vec<Pinned> {
    vec![
        ('a', "1", vec![2, 1], [2, 0, 1, 0, 0]),
        ('a', "0.5", vec![2, 0, 1], [2, 1, 0, 0, 0]),
        ('a', "0", vec![2, 0, 1], [2, 1, 0, 0, 0]),
        ('b', "1", vec![2, 1, 4], [2, 4, 1, 0, 0]),
        ('b', "0.5", vec![2, 1, 4], [2, 4, 1, 0, 0]),
        ('b', "0", vec![2, 0, 5], [2, 5, 0, 0, 0]),
        ('c', "1", vec![2], [0, 2, 0, 0, 0]),
        ('c', "0.5", vec![2], [0, 2, 0, 0, 0]),
        ('c', "0", vec![2], [0, 2, 0, 0, 0]),
        ('d', "1", vec![1, 0, 2, 1, 1, 0], [2, 2, 0, 0, 1]),
        ('d', "0.5", vec![1, 0, 2, 1, 0, 1], [2, 2, 0, 1, 0]),
        ('d', "0", vec![1, 0, 2, 1, 0, 1], [2, 2, 0, 1, 0]),
        ('e', "1", vec![4, 3], [0, 3, 4, 0, 0]),
        ('e', "0.5", vec![4, 2, 1], [1, 2, 4, 0, 0]),
        ('e', "0", vec![4, 2, 1], [1, 2, 4, 0, 0]),
        ('f', "1", vec![1, 0, 9], [1, 3, 0, 0, 0]),
        ('f', "0", vec![1, 0, 9], [1, 3, 0, 0, 0]),
    ]
}

#[test]
fn pinned_the_outcomes_of_seed_1942_in_each_case_at_each_share() {
    let rows: Vec<Pinned> = every_row()
        .into_iter()
        .map(|(case, share)| {
            let (hits, lost) = outcome(&run_seed(&load(case, share), 1942));
            (case, share, hits, lost)
        })
        .collect();
    assert_eq!(rows, pinned());
}
