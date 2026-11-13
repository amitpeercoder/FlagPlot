//! The patrol's fire before and after the attack (decision 0087): a share of
//! the patrol's shots at the bombers is worked before the attack and the rest
//! after it, by the split of decision 0079.
//!
//! # Hand calculation
//!
//! Written by the architect before the test code, and checked and adopted by the
//! coordinating architect on Nov 13, 2026. The tests below assert these
//! numbers. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass.
//!
//! Quantities are cut to six decimal places at each step.
//!
//! ## The rule, as worked here
//!
//! The patrol's roll at the bombers has n shots (decisions 0027, 0033 and
//! 0080: R − X). With p = `patrol_before_attack_share`, the first
//! k = floor(p × n) shots, one fixed-point multiplication cut to a whole
//! number, are worked before the attack and the other n − k after it, from
//! the patrol's one stream in order. The patrol's roll at the escorts and the
//! escorts' fire at the patrol are not split.
//!
//! - Before the attack, as now: the patrol's before-hits come off the
//!   bombers, lowest group first; then the guns fire if any bomber is left
//!   (their own split, decision 0079); what is left attacks.
//! - After the attack (step 4a), raid by raid: the patrol's after-roll, its
//!   hits coming off the bombers that are left, lowest group first, then the
//!   guns' after-roll, as decision 0079 has it. The attack stands.
//! - A part with no shots records nothing. The after-roll is the same
//!   `AirFight` with `after_attack` set, written only when true.
//!
//! p is 1 in the frozen step 1 figures (step 1's rule) and is set to 0.4
//! and 0 by `common::altered_reference`. Everything else is the frozen
//! figures: `fighter_hit_chance` 0.3, `bomb_hit_chance` 0.2,
//! `anti_aircraft_before_attack_share` 1 and `patrol_drawn_off_share` 1
//! unless a case says otherwise; no type has pilot armour or self-sealing
//! tanks, so every chance at aircraft is its base figure.
//!
//! ## The scenario
//!
//! `data/scenarios/19c_patrol_after_the_attack.toml`, one day, on the
//! frozen figures with `strike_contact_max_age_pulses` at 1 (as in 13a, 19a
//! and 19b), so the Japanese strike is launched in the morning at a contact
//! the side starts holding, with no search. Everything asserted by hand is of
//! the morning pulse of day 1.
//!
//! The Allied task force 1 is one "Placeholder carrier" (class 4, no guns)
//! with group 1, 13 Placeholder fighters (type 4) on combat air patrol with
//! a patrol share of 1. The Japanese task force 2, within 250 nm, is one
//! "Placeholder carrier" (class 4) with group 2, 12 Placeholder dive bombers
//! (type 3: a bomb, damage 60) on naval strike, and group 3, 4 Placeholder
//! fighters on combat air patrol with a patrol share of 0, all of which
//! escort. The Japanese side starts holding a contact of task force 1 at its
//! true hex, one ship, carrier. The scenario file is case 1; the other cases
//! change it as they say.
//!
//! ## Case 1: the morning raid, at p = 1, 0.4 and 0
//!
//! P = 13, E = 4, B = 12. At `patrol_drawn_off_share` 1 the escorts draw off
//! 4; the patrol fires 4 at the escorts and 9 at the bombers.
//!
//! | Roll | Shots | Chance | Expected hits | At every p |
//! | --- | --- | --- | --- | --- |
//! | Escorts at the patrol | 4 | 0.3 | 1.2 | yes |
//! | Patrol at the escorts | 4 | 0.3 | 1.2 | yes |
//! | Patrol at the bombers, in all | 9 | 0.3 | 2.7 | split |
//!
//! | p | Before / after (k = floor(9p)) | Expected attack shots, 12 − 0.3k | Expected bomb hits (× 0.2) | Expected bombers lost to the patrol |
//! | --- | --- | --- | --- | --- |
//! | 1 | 9 / 0 | 9.3 | 1.86 | 2.7 |
//! | 0.4 | 3 / 6 (floor 3.6) | 11.1 | 2.22 | 2.7 |
//! | 0 | 0 / 9 | 12 | 2.4 | 2.7 |
//!
//! The 12 bombers cannot be exhausted: 9 shots hit at most 9. A bomb hit on
//! the carrier does 100 × 60 / 600 = 10 points.
//!
//! **The escorts' roll is untouched.** At every p the escorts' 4 shots at
//! the patrol and the patrol's 4 at the escorts are the same events, with the
//! same draws and hits on every seed, and their breakdowns name no share.
//!
//! **The breakdown.** The patrol's roll at the bombers lists, as now, the
//! whole patrol, the raid's aircraft, all the raids' aircraft and the raid's
//! share; when p is below 1 it also lists the shots at the bombers in all
//! (9) and p; then the part's shots and the chance 0.3, with `expected` the
//! part's shots times 0.3: 2.7 at p = 1; 0.9 and 1.8 at p = 0.4; 2.7 after
//! the attack at p = 0. At p = 1 it is the breakdown of step 1.
//!
//! **The events, at p = 0.4,** in this order in the morning: the launch;
//! `EscortsSent`; the escorts' `AirFight` at the patrol; the patrol's at the
//! escorts (4 shots); the patrol's at the bombers (3 shots, not after the
//! attack); the `AircraftLost` of those rolls; the `AirAttack` of group 2;
//! the patrol's `AirFight` at the bombers after the attack (6 shots), citing
//! the launch and the `AirAttack`; its `AircraftLost` of group 2, citing it;
//! then the damage, the report and the claims. At p = 1 there is no
//! after-roll and the log is step 1's. At p = 0 the patrol's only roll at
//! the bombers is the one after the attack.
//!
//! ## Case 2: no bomber left after the fire before the attack
//!
//! The scenario with group 2 at 1 dive bomber, group 3 at 0 and group 1 at
//! 5: no escort, and the patrol fires 5 at the lone bomber.
//!
//! | p | Before / after | Chance it attacks | Expected bomb hits | Chance it is lost |
//! | --- | --- | --- | --- | --- |
//! | 1 | 5 / 0 | 0.7^5 = 0.16807 | 0.033614 | 0.83193 |
//! | 0.4 | 2 / 3 | 0.7^2 = 0.49 | 0.098 | 0.83193 |
//! | 0 | 0 / 5 | 1 | 0.2 | 0.83193 |
//!
//! When the fire before the attack takes the bomber (p = 0.4, chance 0.51),
//! no `AirAttack` is recorded, the after-roll is still rolled and recorded
//! (3 shots, citing the launch alone), and its hits are wasted. The patrol's
//! draws are the same 5 at every p.
//!
//! ## Case 3: a raid on a base
//!
//! A Japanese base with 15 Placeholder dive bombers on base strike and no
//! escort, against an Allied base with 10 Placeholder fighters on patrol and
//! no guns. The patrol fires 10 at the bombers.
//!
//! | p | Before / after | Expected bombs that drop, 15 − 0.3k | Expected bomb hits (× 0.2) |
//! | --- | --- | --- | --- |
//! | 1 | 10 / 0 | 12 | 2.4 |
//! | 0.4 | 4 / 6 | 13.8 | 2.76 |
//! | 0 | 0 / 10 | 15 | 3.0 |
//!
//! The after-roll cites the launch and the `BaseAttacked` event. Each hit
//! lands by the `[base_hit]` draws as now.
//!
//! ## Case 4: the patrol and the guns together
//!
//! The scenario with task force 1's carrier a "Placeholder carrier with
//! guns" (class 5: 20 anti-aircraft shots, toughness 600), group 1 at 6,
//! group 3 at 0: no escort, the patrol fires 6 at the 12 bombers and the
//! guns 20 at 0.05.
//!
//! | p | b (`anti_aircraft_before_attack_share`) | Patrol before / after | Guns before / after | Expected attack shots | Expected bomb hits |
//! | --- | --- | --- | --- | --- | --- |
//! | 1 | 1 | 6 / 0 | 20 / 0 | 12 − 1.8 − 1.0 = 9.2 | 1.84 |
//! | 0.4 | 1 | 2 / 4 (floor 2.4) | 20 / 0 | 12 − 0.6 − 1.0 = 10.4 | 2.08 |
//! | 0.4 | 0 | 2 / 4 | 0 / 20 | 12 − 0.6 = 11.4 | 2.28 |
//! | 0 | 0 | 0 / 6 | 0 / 20 | 12 | 2.4 |
//!
//! The bombers lost in all are 1.8 + 1.0 = 2.8 expected in every row (12
//! cannot be exhausted by 26 shots at these chances but with a chance below
//! one in a million). In step 4a the patrol's after-roll comes before the
//! guns' after-roll, and its hits come off first.
//!
//! # Properties
//!
//! Over seeds 1 to 1000:
//!
//! - Case 1 at p = 1, 0.4 and 0: the patrol's hits at the bombers in all,
//!   each group's total of aircraft lost and the defender's claims are equal
//!   at every p; the escorts' roll and the patrol's roll at the escorts are
//!   identical events; the attack's shots equal 12 less the hits among the
//!   first k of the patrol's draws at the bombers.
//! - Case 4, the four rows: the same equalities of hits and losses, and the
//!   attack's shots equal 12 less the patrol's before-hits less the guns'
//!   before-hits.
//!   The claims are not asserted in case 4: the defender's false claims are
//!   drawn for its missed shots in the order its firing events were noted,
//!   patrol, then guns, then the patrol after the attack, at two different
//!   chances (0.1 for fighters, 0.05 for guns), so they may differ by row.
//! - At p = 1 every seed's events, in every case, are those of the frozen
//!   figures before the rule, and every scenario that existed before the
//!   rule keeps its events on the frozen figures.
//! - The morning attack's shots in case 1, summed over the 1,000 seeds, are
//!   within four standard deviations of 1,000 times the expectation (the
//!   variance per seed is k × 0.3 × 0.7 = 0.21k):
//!
//! | p | Expected total | Standard deviation | Allowed |
//! | --- | --- | --- | --- |
//! | 1 | 9,300 | sqrt(1,000 × 1.89) = 43.5 | 9,126 to 9,474 |
//! | 0.4 | 11,100 | sqrt(1,000 × 0.63) = 25.1 | 11,000 to 11,200 |
//! | 0 | 12,000 | 0 | exactly 12,000 |
//!
//! # Pinned outcomes
//!
//! What the scenario's own seed (1942) gives in each case at each p, with a
//! second seed beside it where 1942 moves no hit across the attack.
//! Pinned, not calculated: no hand calculation can say which draw succeeds.
//! A change here means a stream or the order of draws moved; they are
//! remade only with a deliberate change of the rule (docs/step2-spec.md,
//! "Tests while figures move", 7). Filled in by the build from the first
//! run, and checked against the properties above before they are pinned.
//!
//! PINNED (not calculated), morning of day 1, by the build from the first run:
//! seed 1942 is the scenario's own, and seed 1 is beside it for a second
//! pattern. Seed 1942 moves patrol hits across the attack in every case, so no
//! seed is needed to show a share that moves none. Hits are the patrol's at the
//! bombers before and after the attack and, in case 4, the guns' before and
//! after; the attack's shots and bomb hits are those of the raid's attack (a
//! raid with no bomber left has none); lost is by groups 1, 2 and 3 (in case 3 the
//! bombers are group 1). Case 4's b is the guns' share, and is 1 elsewhere.
//!
//! | Case | Seed | p | b | Patrol hits before / after | Guns hits before / after | Attack shots | Bomb hits | Lost (1, 2, 3) |
//! | --- | --- | --- | --- | --- | --- | --- | --- | --- |
//! | 1 | 1942 | 1 | - | 6 / 0 | 0 / 0 | 6 | 0 | 2, 6, 1 |
//! | 1 | 1942 | 0.4 | - | 2 / 4 | 0 / 0 | 10 | 1 | 2, 6, 1 |
//! | 1 | 1942 | 0 | - | 0 / 6 | 0 / 0 | 12 | 2 | 2, 6, 1 |
//! | 1 | 1 | 1 | - | 3 / 0 | 0 / 0 | 9 | 2 | 1, 3, 2 |
//! | 1 | 1 | 0.4 | - | 1 / 2 | 0 / 0 | 11 | 2 | 1, 3, 2 |
//! | 1 | 1 | 0 | - | 0 / 3 | 0 / 0 | 12 | 2 | 1, 3, 2 |
//! | 2 | 1942 | 1 | - | 2 / 0 | 0 / 0 | 0 | 0 | 0, 1, 0 |
//! | 2 | 1942 | 0.4 | - | 0 / 2 | 0 / 0 | 1 | 0 | 0, 1, 0 |
//! | 2 | 1942 | 0 | - | 0 / 2 | 0 / 0 | 1 | 0 | 0, 1, 0 |
//! | 2 | 1 | 1 | - | 2 / 0 | 0 / 0 | 0 | 0 | 0, 1, 0 |
//! | 2 | 1 | 0.4 | - | 1 / 1 | 0 / 0 | 0 | 0 | 0, 1, 0 |
//! | 2 | 1 | 0 | - | 0 / 2 | 0 / 0 | 1 | 0 | 0, 1, 0 |
//! | 3 | 1942 | 1 | - | 5 / 0 | 0 / 0 | 10 | 1 | 5, 0, 0 |
//! | 3 | 1942 | 0.4 | - | 4 / 1 | 0 / 0 | 11 | 1 | 5, 0, 0 |
//! | 3 | 1942 | 0 | - | 0 / 5 | 0 / 0 | 15 | 1 | 5, 0, 0 |
//! | 3 | 1 | 1 | - | 5 / 0 | 0 / 0 | 10 | 1 | 5, 0, 0 |
//! | 3 | 1 | 0.4 | - | 1 / 4 | 0 / 0 | 14 | 2 | 5, 0, 0 |
//! | 3 | 1 | 0 | - | 0 / 5 | 0 / 0 | 15 | 2 | 5, 0, 0 |
//! | 4 | 1942 | 1 | 1 | 2 / 0 | 0 / 0 | 10 | 1 | 0, 2, 0 |
//! | 4 | 1942 | 0.4 | 1 | 0 / 2 | 0 / 0 | 12 | 2 | 0, 2, 0 |
//! | 4 | 1942 | 0.4 | 0 | 0 / 2 | 0 / 0 | 12 | 2 | 0, 2, 0 |
//! | 4 | 1942 | 0 | 0 | 0 / 2 | 0 / 0 | 12 | 2 | 0, 2, 0 |
//! | 4 | 1 | 1 | 1 | 3 / 0 | 1 / 0 | 8 | 2 | 0, 4, 0 |
//! | 4 | 1 | 0.4 | 1 | 1 / 2 | 1 / 0 | 10 | 2 | 0, 4, 0 |
//! | 4 | 1 | 0.4 | 0 | 1 / 2 | 0 / 1 | 11 | 2 | 0, 4, 0 |
//! | 4 | 1 | 0 | 0 | 0 / 3 | 0 / 1 | 12 | 2 | 0, 4, 0 |
//!
//! Checked against the properties above before pinning, by the test that
//! asserts the pins (it runs the hand calculation's checks on each pinned
//! run): the patrol's hits in all, and each group's losses, are the same at
//! every p (case 1 seed 1942: 6 hits, group 2 loses 6; seed 1: 3; case 2: the
//! lone bomber is lost on both seeds at every p; case 3 seed 1942: 5, seed 1: 5;
//! case 4 seed 1942: 2 and no gun hit, seed 1: 3 and 1, group 2 losing 4 in all
//! four rows); the attack's shots are the bombers less the hits before the
//! attack (case 1 seed 1942: 12 - 6 = 6, 12 - 2 = 10, 12; case 4 seed 1: 12 - 3 - 1
//! = 8, 12 - 1 - 1 = 10, 12 - 1 = 11, 12); and a seed's draws, taken from the
//! patrol's own stream by the test, give the hits of each part.

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
const SCENARIO: &str = "19c_patrol_after_the_attack.toml";
const SEEDS: u64 = 1000;
/// The shares of the cases, as written in the figures file.
const SHARES: [&str; 3] = ["1", "0.4", "0"];

fn fx(n: i64) -> Fixed {
    Fixed::from_int(n)
}

fn hundredths(n: i64) -> Fixed {
    Fixed::from_ratio(n, 100)
}

/// One case of the hand calculation: the patrol's first group, the raid's
/// first strike group, the bombers B, the patrol's shots at them n, the
/// patrol's draws before its shots at the bombers (those at the escorts),
/// the whole patrol, and the raid's aircraft.
#[derive(Clone, Copy)]
struct Case {
    number: u8,
    patrol_first: u32,
    strike: u32,
    bombers: i64,
    n: i64,
    offset: usize,
    patrol: i64,
    raid_aircraft: i64,
}

const CASE_1: Case = Case {
    number: 1,
    patrol_first: 1,
    strike: 2,
    bombers: 12,
    n: 9,
    offset: 4,
    patrol: 13,
    raid_aircraft: 16,
};
const CASE_2: Case = Case {
    number: 2,
    patrol_first: 1,
    strike: 2,
    bombers: 1,
    n: 5,
    offset: 0,
    patrol: 5,
    raid_aircraft: 1,
};
const CASE_3: Case = Case {
    number: 3,
    patrol_first: 2,
    strike: 1,
    bombers: 15,
    n: 10,
    offset: 0,
    patrol: 10,
    raid_aircraft: 15,
};
const CASE_4: Case = Case {
    number: 4,
    patrol_first: 1,
    strike: 2,
    bombers: 12,
    n: 6,
    offset: 0,
    patrol: 6,
    raid_aircraft: 12,
};

/// k = floor(p x n) for the shares of the table, in whole numbers.
fn before_of(n: i64, p: &str) -> i64 {
    match p {
        "1" => n,
        "0.4" => n * 4 / 10,
        "0" => 0,
        other => panic!("no share {other}"),
    }
}

fn scenario_path() -> PathBuf {
    PathBuf::from(ROOT).join("data/scenarios").join(SCENARIO)
}

fn scenario_text() -> String {
    fs::read_to_string(scenario_path()).expect("the scenario reads")
}

/// The frozen figures with the changes the hand calculation names: a contact
/// the side starts with can be struck in the morning, the patrol's share is
/// `p`, and the guns' share is `b` if given (written as the figures file
/// writes them).
fn options(p: &str, b: Option<&str>) -> LoadOptions {
    common::altered_reference("patrol_after_attack", |dir| {
        let mut edits = vec![
            (
                "strike_contact_max_age_pulses = 0".to_string(),
                "strike_contact_max_age_pulses = 1".to_string(),
            ),
            (
                "patrol_before_attack_share = \"1\"".to_string(),
                format!("patrol_before_attack_share = \"{p}\""),
            ),
        ];
        if let Some(b) = b {
            edits.push((
                "anti_aircraft_before_attack_share = \"1\"".to_string(),
                format!("anti_aircraft_before_attack_share = \"{b}\""),
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
    let text = common::replace_once(&text, "ready = 13", "ready = @1");
    let text = common::replace_once(&text, "ready = 12", "ready = @2");
    let text = common::replace_once(&text, "ready = 4", "ready = @3");
    text.replace("@1", &patrol.to_string())
        .replace("@2", &bombers.to_string())
        .replace("@3", &escorts.to_string())
}

/// Case 3: 15 dive bombers of a Japanese base, with no escort, raid an
/// Allied base with 10 fighters on patrol and no guns.
fn case_3_text() -> String {
    r#"name = "Scenario 19c, case 3: a raid on a base"
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
ready = 15

[[air_group]]
id = 2
name = "Placeholder defending fighters"
side = "allied"
base = 2
aircraft_type = 4
ready = 10

[[order]]
day = 1
side = "japanese"
air_group = 1
kind = "set_air_mission"
mission = "base_strike"
target_base = 2

[[order]]
day = 1
side = "allied"
air_group = 2
kind = "set_air_mission"
mission = "combat_air_patrol"
"#
    .to_string()
}

fn case_text(case: Case) -> String {
    match case.number {
        1 => scenario_text(),
        // One bomber against a patrol of 5 and no escort.
        2 => with_ready(5, 1, 0),
        3 => case_3_text(),
        // The carrier has 20 anti-aircraft shots (class 5); the patrol is 6.
        4 => common::replace_once(
            &with_ready(6, 12, 0),
            "id = 1\nname = \"Placeholder carrier\"\nclass = 4",
            "id = 1\nname = \"Placeholder carrier with guns\"\nclass = 5",
        ),
        other => panic!("no case {other}"),
    }
}

fn load(case: Case, p: &str, b: Option<&str>) -> Scenario {
    common::from_text_with(&case_text(case), &scenario_path(), &options(p, b))
        .expect("the case loads")
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

/// The patrol's rolls at the bombers of the morning, in the order logged.
fn bomber_parts(events: &[Event]) -> Vec<Event> {
    events
        .iter()
        .filter(|e| {
            matches!(
                e.kind,
                EventKind::AirFight {
                    firing: AirFirer::Patrol,
                    at: AirTarget::Bombers,
                    ..
                }
            )
        })
        .cloned()
        .collect()
}

/// (shots, chance, expected, hits, after_attack) of an `AirFight`.
fn part_of(event: &Event) -> (Fixed, Fixed, Fixed, u32, bool) {
    match event.kind {
        EventKind::AirFight {
            shots,
            chance,
            expected,
            hits,
            after_attack,
            ..
        } => (shots, chance, expected, hits, after_attack),
        ref other => panic!("not a fight: {other:?}"),
    }
}

/// The patrol's hits at the bombers before and after the attack.
fn patrol_hits(events: &[Event]) -> (u32, u32) {
    let mut hits = (0, 0);
    for part in bomber_parts(events) {
        let part = part_of(&part);
        if part.4 {
            hits.1 += part.3;
        } else {
            hits.0 += part.3;
        }
    }
    hits
}

fn is_guns(kind: &EventKind) -> bool {
    matches!(
        kind,
        EventKind::AntiAircraftFire { .. } | EventKind::BaseAntiAircraftFire { .. }
    )
}

/// (shots, hits, after_attack) of each gun event, in order.
fn gun_parts(events: &[Event]) -> Vec<(i64, u32, bool)> {
    events
        .iter()
        .filter_map(|e| match e.kind {
            EventKind::AntiAircraftFire {
                shots,
                hits,
                after_attack,
                ..
            } => Some((shots.trunc(), hits, after_attack)),
            _ => None,
        })
        .collect()
}

fn gun_hits(events: &[Event]) -> (u32, u32) {
    let mut hits = (0, 0);
    for part in gun_parts(events) {
        if part.2 {
            hits.1 += part.1;
        } else {
            hits.0 += part.1;
        }
    }
    hits
}

/// The attack events of the morning: `AirAttack` or `BaseAttacked`.
fn attack_events(events: &[Event]) -> Vec<Event> {
    events
        .iter()
        .filter(|e| {
            matches!(
                e.kind,
                EventKind::AirAttack { .. } | EventKind::BaseAttacked { .. }
            )
        })
        .cloned()
        .collect()
}

/// (shots, chance, expected, hits) of the morning's attack, none if there
/// is no attack.
fn attack_of(events: &[Event]) -> Option<(Fixed, Fixed, Fixed, u32)> {
    attack_events(events).first().map(|event| match event.kind {
        EventKind::AirAttack {
            shots,
            chance,
            expected,
            hits,
            ..
        }
        | EventKind::BaseAttacked {
            shots,
            chance,
            expected,
            hits,
            ..
        } => (shots, chance, expected, hits),
        _ => unreachable!(),
    })
}

fn attack_shots(events: &[Event]) -> i64 {
    attack_of(events).map_or(0, |attack| attack.0.trunc())
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

/// The aircraft lost by `group` in `AircraftLost` events that cite `cause`.
fn lost_citing(events: &[Event], group: u32, cause: EventId) -> u32 {
    events
        .iter()
        .map(|event| match event.kind {
            EventKind::AircraftLost {
                air_group,
                aircraft,
                ..
            } if air_group.raw() == group && event.causes.contains(&Cause::Event(cause)) => {
                aircraft
            }
            _ => 0,
        })
        .sum()
}

fn launch_of(events: &[Event]) -> EventId {
    events
        .iter()
        .find(|e| {
            matches!(
                e.kind,
                EventKind::StrikeLaunched { .. } | EventKind::BaseStrikeLaunched { .. }
            )
        })
        .expect("the raid launches")
        .id
}

/// The defender's (Allied) claims of the morning.
fn allied_claims(events: &[Event]) -> Vec<EventKind> {
    events
        .iter()
        .filter(|e| {
            matches!(
                e.kind,
                EventKind::ClaimReceived {
                    side: Side::Allied,
                    ..
                }
            )
        })
        .map(|e| e.kind.clone())
        .collect()
}

/// The draws of the patrol's stream of the morning, each True for a hit at
/// 0.3, as the rule rolls them: one stream keyed on the patrol's first
/// group and the raid's first strike group, one draw for each shot, in order:
/// first the roll at the escorts, if any, then the shots at the bombers
/// (decision 0087, 3).
fn patrol_draws(case: Case, seed: u64, count: usize) -> Vec<bool> {
    let mut stream = RngKey {
        seed,
        day: 1,
        pulse: Some(Pulse::Morning),
        subsystem: Subsystem::AirCombat,
        entity: EntityRef::AirGroup(AirGroupId::new(case.patrol_first)),
        other: Some(EntityRef::AirGroup(AirGroupId::new(case.strike))),
        draw: Draw::Legacy,
    }
    .stream();
    (0..count).map(|_| stream.chance(hundredths(30))).collect()
}

/// The 20 draws of the guns' stream of the morning (case 4), True for a hit
/// at 0.05.
fn gun_draws(seed: u64) -> Vec<bool> {
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
    (0..20).map(|_| stream.chance(hundredths(5))).collect()
}

/// The patrol's hits at the bombers in the first k draws and in the rest,
/// from the stream.
fn draws_split(case: Case, seed: u64, p: &str) -> (u32, u32) {
    let draws = patrol_draws(case, seed, case.offset + case.n as usize);
    let k = case.offset + before_of(case.n, p) as usize;
    let count = |part: &[bool]| u32::try_from(part.iter().filter(|hit| **hit).count()).unwrap();
    (count(&draws[case.offset..k]), count(&draws[k..]))
}

// ---------------------------------------------------------------------
// Every case: shots, chances, breakdowns, draws, events, causes, losses.
// ---------------------------------------------------------------------

/// Everything the hand calculation says of one morning of one case at p (and
/// b, in case 4), asserted for one seed.
fn check_morning(case: Case, p: &str, b: Option<&str>, seed: u64, world: &World) {
    let at = format!("case {} p {p} b {b:?} seed {seed}", case.number);
    let events = morning(world);
    let launch = launch_of(&events);
    let k = before_of(case.n, p);
    let parts = bomber_parts(&events);
    let mut want_shots = Vec::new();
    if k > 0 {
        want_shots.push((fx(k), false));
    }
    if case.n - k > 0 {
        want_shots.push((fx(case.n - k), true));
    }
    // A part with no shots records nothing; the shots are k, then n - k.
    let got_shots: Vec<(Fixed, bool)> = parts
        .iter()
        .map(|part| (part_of(part).0, part_of(part).4))
        .collect();
    assert_eq!(got_shots, want_shots, "{at}");
    // The hits of each part are those of the draws of the one stream: the
    // first k before the attack, the rest after.
    let (draws_before, draws_after) = draws_split(case, seed, p);
    assert_eq!(patrol_hits(&events), (draws_before, draws_after), "{at}");
    for part in &parts {
        let (shots, chance, expected, hits, _) = part_of(part);
        assert_eq!(chance, hundredths(30), "{at}");
        assert_eq!(expected, shots * hundredths(30), "{at}");
        assert!(fx(i64::from(hits)) <= shots, "{at}");
        // The breakdown: the patrol's factors, below p = 1 the shots at the
        // bombers in all and p, then the part's shots and the chance.
        let mut want = vec![
            "patrol",
            "raid_aircraft",
            "all_raids_aircraft",
            "patrol_share",
        ];
        if p != "1" {
            want.extend(["patrol_shots_at_bombers", "patrol_before_attack_share"]);
        }
        want.extend(["shots", "hit_chance"]);
        assert_eq!(names(part), want, "{at}");
        assert_eq!(value_of(part, "patrol"), fx(case.patrol), "{at}");
        assert_eq!(
            value_of(part, "raid_aircraft"),
            fx(case.raid_aircraft),
            "{at}"
        );
        assert_eq!(
            value_of(part, "all_raids_aircraft"),
            fx(case.raid_aircraft),
            "{at}"
        );
        assert_eq!(value_of(part, "patrol_share"), fx(case.patrol), "{at}");
        if p != "1" {
            assert_eq!(
                value_of(part, "patrol_shots_at_bombers"),
                fx(case.n),
                "{at}"
            );
            let share = if p == "0.4" {
                hundredths(40)
            } else {
                Fixed::ZERO
            };
            assert_eq!(value_of(part, "patrol_before_attack_share"), share, "{at}");
        }
        assert_eq!(value_of(part, "shots"), shots, "{at}");
        assert_eq!(value_of(part, "hit_chance"), hundredths(30), "{at}");
        assert_eq!(part.breakdown.as_ref().expect("on").result, expected);
        // Only the after part says so, in the text.
        assert_eq!(
            part.kind.to_string().ends_with(" after the attack"),
            part_of(part).4,
            "{at}"
        );
    }
    // The guns, in case 4: 20 shots at 0.05, k = floor(20 b) before.
    let guns: Vec<Event> = events
        .iter()
        .filter(|e| is_guns(&e.kind))
        .cloned()
        .collect();
    let (guns_before, guns_after) = gun_hits(&events);
    if case.number == 4 {
        let b = b.expect("case 4 names b");
        let mut want = Vec::new();
        if b == "1" {
            want.push((20, false));
        } else {
            want.push((20, true));
        }
        let got: Vec<(i64, bool)> = gun_parts(&events).iter().map(|g| (g.0, g.2)).collect();
        assert_eq!(got, want, "{at}");
        let draws = gun_draws(seed);
        let total = u32::try_from(draws.iter().filter(|hit| **hit).count()).unwrap();
        assert_eq!(guns_before + guns_after, total, "{at}");
    } else {
        assert!(guns.is_empty(), "{at}");
    }
    // The attack's shots: the bombers less the patrol's hits before the
    // attack and the guns' hits before it, never below 0; its chance is 0.2.
    let bombers_left = (case.bombers - i64::from(patrol_hits(&events).0)).max(0);
    let bombers_left = (bombers_left - i64::from(guns_before)).max(0);
    assert_eq!(attack_shots(&events), bombers_left, "{at}");
    let attack = attack_events(&events);
    assert_eq!(attack.len(), usize::from(bombers_left > 0), "{at}");
    if let Some((shots, chance, expected, hits)) = attack_of(&events) {
        assert_eq!(chance, hundredths(20), "{at}");
        assert_eq!(expected, shots * hundredths(20), "{at}");
        assert!(fx(i64::from(hits)) <= shots, "{at}");
    }
    // Causes. The part before the attack cites the launch alone; the part
    // after it the launch and the raid's attack (the launch alone if none
    // attacked). The attack stands: the losses after are not its causes.
    let attack_ids: Vec<Cause> = attack.iter().map(|e| Cause::Event(e.id)).collect();
    let after_causes: Vec<Cause> = std::iter::once(Cause::Event(launch))
        .chain(attack_ids)
        .collect();
    for part in &parts {
        if part_of(part).4 {
            assert_eq!(part.causes, after_causes, "{at}");
            if let Some(attack) = attack.first() {
                assert!(attack.id < part.id, "{at}");
                assert!(!attack.causes.contains(&Cause::Event(part.id)), "{at}");
            }
        } else {
            assert_eq!(part.causes, vec![Cause::Event(launch)], "{at}");
            if let Some(attack) = attack.first() {
                assert!(part.id < attack.id, "{at}");
            }
        }
    }
    // The losses: the hits of each part come off the bombers that are left,
    // the part before from all of them, the part after from those the fire
    // before, and the guns before, left.
    let pool = case.bombers;
    let before_taken = i64::from(patrol_hits(&events).0).min(pool);
    let mut left = pool - before_taken;
    for part in &parts {
        let (_, _, _, hits, after) = part_of(part);
        let citing = i64::from(lost_citing(&events, case.strike, part.id));
        if after {
            // Taken after the attack, after the guns' fire before it.
            let guns_before_taken = i64::from(guns_before).min(left);
            let available = left - guns_before_taken;
            let taken = i64::from(hits).min(available);
            assert_eq!(citing, taken, "{at}");
            left = available - taken;
        } else {
            assert_eq!(citing, i64::from(hits).min(pool), "{at}");
            for loss in events.iter().filter(|e| {
                matches!(e.kind, EventKind::AircraftLost { .. })
                    && e.causes.contains(&Cause::Event(part.id))
            }) {
                assert!(loss.id > part.id, "{at}");
                assert_eq!(loss.causes, vec![Cause::Event(part.id)], "{at}");
            }
        }
        if after {
            assert!(
                events
                    .iter()
                    .filter(|e| matches!(e.kind, EventKind::AircraftLost { .. })
                        && e.causes.contains(&Cause::Event(part.id)))
                    .all(|e| e.id > part.id),
                "{at}"
            );
        }
    }
    // In all, the bombers lost are the hits of the patrol and the guns, no
    // more than the bombers there were.
    let hits_in_all = i64::from(patrol_hits(&events).0 + patrol_hits(&events).1)
        + i64::from(guns_before + guns_after);
    assert_eq!(
        i64::from(lost_by(&events, case.strike)),
        hits_in_all.min(pool),
        "{at}"
    );
    // The order of the log: the launch, the fire before, the attack, the
    // fire after; in case 4 the guns' parts as the table has them, the
    // patrol's after-roll before the guns'.
    let mut order: Vec<EventId> = vec![launch];
    order.extend(
        parts
            .iter()
            .filter(|part| !part_of(part).4)
            .map(|part| part.id),
    );
    order.extend(
        guns.iter()
            .filter(|g| match g.kind {
                EventKind::AntiAircraftFire { after_attack, .. } => !after_attack,
                _ => false,
            })
            .map(|g| g.id),
    );
    order.extend(attack.iter().map(|e| e.id));
    order.extend(
        parts
            .iter()
            .filter(|part| part_of(part).4)
            .map(|part| part.id),
    );
    order.extend(
        guns.iter()
            .filter(|g| match g.kind {
                EventKind::AntiAircraftFire { after_attack, .. } => after_attack,
                _ => false,
            })
            .map(|g| g.id),
    );
    assert!(
        order.windows(2).all(|pair| pair[0] < pair[1]),
        "{at}: {order:?}"
    );
    // Guns after the attack cite the launch and the attack, as the
    // patrol's fire after does.
    for gun in guns.iter().filter(|g| {
        matches!(
            g.kind,
            EventKind::AntiAircraftFire {
                after_attack: true,
                ..
            }
        )
    }) {
        assert_eq!(gun.causes, after_causes, "{at}");
    }
}

fn rows(case: Case) -> Vec<(&'static str, Option<&'static str>)> {
    match case.number {
        4 => vec![
            ("1", Some("1")),
            ("0.4", Some("1")),
            ("0.4", Some("0")),
            ("0", Some("0")),
        ],
        _ => SHARES.iter().map(|p| (*p, None)).collect(),
    }
}

#[test]
fn every_case_has_the_shots_chances_breakdowns_events_and_losses_of_the_hand_calculation() {
    for case in [CASE_1, CASE_2, CASE_3, CASE_4] {
        for (p, b) in rows(case) {
            let base = load(case, p, b);
            for seed in 1..=SEEDS {
                check_morning(case, p, b, seed, &run_seed(&base, seed));
            }
        }
    }
}

#[test]
fn case_1_the_escorts_roll_and_the_patrols_roll_at_the_escorts_are_not_split() {
    for p in SHARES {
        let base = load(CASE_1, p, None);
        for seed in 1..=SEEDS {
            let events = morning(&run_seed(&base, seed));
            let fights: Vec<&Event> = events
                .iter()
                .filter(|e| matches!(e.kind, EventKind::AirFight { .. }))
                .collect();
            // Escorts at the patrol, the patrol at the escorts, then the
            // bombers' parts.
            assert!(matches!(
                fights[0].kind,
                EventKind::AirFight {
                    firing: AirFirer::Escorts,
                    at: AirTarget::Patrol,
                    shots,
                    after_attack: false,
                    ..
                } if shots == fx(4)
            ));
            assert!(matches!(
                fights[1].kind,
                EventKind::AirFight {
                    firing: AirFirer::Patrol,
                    at: AirTarget::Escorts,
                    shots,
                    after_attack: false,
                    ..
                } if shots == fx(4)
            ));
            // Their breakdowns name no share of the attack: the escorts' is
            // step 1's, the patrol's the factors of its raid's share.
            assert_eq!(names(fights[0]), vec!["shots", "hit_chance"]);
            assert_eq!(
                names(fights[1]),
                vec![
                    "patrol",
                    "raid_aircraft",
                    "all_raids_aircraft",
                    "patrol_share",
                    "shots",
                    "hit_chance"
                ]
            );
            assert_eq!(fights.len(), 2 + bomber_parts(&events).len());
        }
    }
}

// ---------------------------------------------------------------------
// The properties.
// ---------------------------------------------------------------------

/// What case 1 gives in the morning of one run.
struct Morning {
    events: Vec<Event>,
    before_hits: u32,
    after_hits: u32,
    attack_shots: i64,
    lost: [u32; 3],
}

fn summarize(world: &World) -> Morning {
    let events = morning(world);
    let (before_hits, after_hits) = patrol_hits(&events);
    Morning {
        before_hits,
        after_hits,
        attack_shots: attack_shots(&events),
        lost: [
            lost_by(&events, 1),
            lost_by(&events, 2),
            lost_by(&events, 3),
        ],
        events,
    }
}

/// The escorts' fire at the patrol and the patrol's at the escorts, whole.
fn escort_rolls(events: &[Event]) -> Vec<Event> {
    events
        .iter()
        .filter(|e| match e.kind {
            EventKind::AirFight { firing, at, .. } => {
                firing == AirFirer::Escorts || at == AirTarget::Escorts
            }
            _ => false,
        })
        .cloned()
        .collect()
}

#[test]
fn over_1000_seeds_case_1_the_patrols_hits_losses_and_claims_are_the_same_at_every_share() {
    let runs: Vec<Vec<Morning>> = SHARES
        .iter()
        .map(|p| {
            let base = load(CASE_1, p, None);
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
            let p = SHARES[which];
            // The patrol's hits at the bombers in all, the losses of each
            // group and the defender's claims.
            assert_eq!(total(run), total(&at_1[index]), "seed {seed} p {p}");
            assert_eq!(run.lost, at_1[index].lost, "seed {seed} p {p}");
            assert_eq!(
                allied_claims(&run.events),
                allied_claims(&at_1[index].events),
                "seed {seed} p {p}"
            );
            // The escorts' roll and the patrol's roll at the escorts are the
            // same events, draws and hits.
            assert_eq!(
                escort_rolls(&run.events),
                escort_rolls(&at_1[index].events),
                "seed {seed} p {p}"
            );
            // The attack's shots: 12 less the hits among the first k of the
            // patrol's draws at the bombers.
            let draws = patrol_draws(CASE_1, seed, 13);
            let k = before_of(9, p) as usize;
            let among_first_k = draws[4..4 + k].iter().filter(|hit| **hit).count() as i64;
            assert_eq!(run.attack_shots, 12 - among_first_k, "seed {seed} p {p}");
        }
    }
    // The attack's shots in all are within four standard deviations of the
    // hand calculation's expectation.
    for (p, (low, high)) in
        SHARES
            .iter()
            .zip([(9_126_i64, 9_474_i64), (11_000, 11_200), (12_000, 12_000)])
    {
        let which = SHARES.iter().position(|share| share == p).unwrap();
        let total: i64 = runs[which].iter().map(|run| run.attack_shots).sum();
        assert!(
            (low..=high).contains(&total),
            "p {p}: {total} not in {low}..={high}"
        );
    }
}

#[test]
fn over_1000_seeds_case_2_the_lone_bomber_is_lost_as_often_at_every_share_and_attacks_as_the_table_says(
) {
    // (p, attacks allowed, bomb hits allowed): the chance the bomber lives
    // through the fire before the attack, 0.7^5, 0.7^2 and 1; four standard
    // deviations of the count of a seed's Bernoulli.
    let allowed = [
        ("1", (121, 215), (11, 56)),
        ("0.4", (427, 553), (60, 136)),
        ("0", (1000, 1000), (150, 250)),
    ];
    let mut lost_at = Vec::new();
    for (p, attacks, bomb_hits) in allowed {
        let base = load(CASE_2, p, None);
        let (mut attacked, mut hit, mut lost) = (0_i64, 0_i64, 0_i64);
        let mut lost_per_seed = Vec::new();
        for seed in 1..=SEEDS {
            let events = morning(&run_seed(&base, seed));
            let (before, after) = patrol_hits(&events);
            // It attacks if the fire before the attack took no bomber.
            assert_eq!(attack_events(&events).len(), usize::from(before == 0));
            attacked += attack_events(&events).len() as i64;
            hit += i64::from(attack_of(&events).map_or(0, |a| a.3));
            // Lost to one hit or more of the 5 draws, whenever they fall.
            let any = before + after > 0;
            assert_eq!(lost_by(&events, 2), u32::from(any), "p {p} seed {seed}");
            lost += i64::from(lost_by(&events, 2));
            lost_per_seed.push(lost_by(&events, 2));
        }
        assert!(
            (attacks.0..=attacks.1).contains(&attacked),
            "p {p}: {attacked} attacks"
        );
        assert!(
            (bomb_hits.0..=bomb_hits.1).contains(&hit),
            "p {p}: {hit} bomb hits"
        );
        lost_at.push((lost, lost_per_seed));
    }
    // 0.83193 lost: 832 expected (sd 11.8); the same on every seed at every p.
    assert!((785..=879).contains(&lost_at[0].0), "{} lost", lost_at[0].0);
    assert_eq!(lost_at[0].1, lost_at[1].1);
    assert_eq!(lost_at[0].1, lost_at[2].1);
}

#[test]
fn over_1000_seeds_case_3_the_bombs_that_drop_are_15_less_the_hits_before_the_attack() {
    for (p, (low, high)) in
        SHARES
            .iter()
            .zip([(11_817_i64, 12_183_i64), (13_684, 13_916), (15_000, 15_000)])
    {
        let base = load(CASE_3, p, None);
        let mut total = 0_i64;
        let mut lost_per_seed = Vec::new();
        for seed in 1..=SEEDS {
            let events = morning(&run_seed(&base, seed));
            let attacked = events
                .iter()
                .find(|e| matches!(e.kind, EventKind::BaseAttacked { .. }))
                .expect("the raid attacks");
            total += attack_shots(&events);
            // The after-roll cites the launch and the BaseAttacked event.
            for part in bomber_parts(&events).iter().filter(|p| part_of(p).4) {
                assert_eq!(
                    part.causes,
                    vec![Cause::Event(launch_of(&events)), Cause::Event(attacked.id)],
                    "p {p} seed {seed}"
                );
            }
            lost_per_seed.push(lost_by(&events, 1));
        }
        assert!(
            (low..=high).contains(&total),
            "p {p}: {total} bombs in all, not in {low}..={high}"
        );
        // 10 hits at most of 15: never exhausted, and the same at every p.
        let at_1 = load(CASE_3, "1", None);
        for (index, lost) in lost_per_seed.iter().enumerate() {
            let events = morning(&run_seed(&at_1, index as u64 + 1));
            assert_eq!(*lost, lost_by(&events, 1), "p {p} seed {}", index + 1);
        }
    }
}

#[test]
fn over_1000_seeds_case_4_the_hits_and_losses_are_the_same_in_the_four_rows() {
    // The attack's shots in all, in the order of the table's rows: 9.2, 10.4,
    // 11.4 and 12 expected, four standard deviations (the variance per seed
    // is 0.21 k for the patrol's before part and 0.95 for the guns' fire
    // before, which only the rows with b = 1 have).
    let allowed = [
        (9_012_i64, 9_388_i64),
        (10_252, 10_548),
        (11_318, 11_482),
        (12_000, 12_000),
    ];
    let mut totals: Vec<(Vec<u32>, Vec<u32>, Vec<u32>)> = Vec::new();
    for ((p, b), (low, high)) in rows(CASE_4).into_iter().zip(allowed) {
        let base = load(CASE_4, p, b);
        let (mut patrol, mut guns, mut lost) = (Vec::new(), Vec::new(), Vec::new());
        let mut shots_in_all = 0_i64;
        for seed in 1..=SEEDS {
            let events = morning(&run_seed(&base, seed));
            let (before, after) = patrol_hits(&events);
            let (guns_before, guns_after) = gun_hits(&events);
            patrol.push(before + after);
            guns.push(guns_before + guns_after);
            lost.push(lost_by(&events, 2));
            // 12 less the hits before the attack, the patrol's and the guns'.
            assert_eq!(
                attack_shots(&events),
                (12 - i64::from(before) - i64::from(guns_before)).max(0),
                "p {p} b {b:?} seed {seed}"
            );
            shots_in_all += attack_shots(&events);
        }
        assert!(
            (low..=high).contains(&shots_in_all),
            "p {p} b {b:?}: {shots_in_all} not in {low}..={high}"
        );
        totals.push((patrol, guns, lost));
    }
    for row in &totals[1..] {
        assert_eq!(row.0, totals[0].0, "the patrol's hits in all");
        assert_eq!(row.1, totals[0].1, "the guns' hits in all");
        assert_eq!(row.2, totals[0].2, "the bombers lost by seed");
    }
}

// ---------------------------------------------------------------------
// Pinned outcomes.
// ---------------------------------------------------------------------

/// (case, seed, p, b, patrol hits before, after, guns hits before, after,
/// the attack's shots, its hits, aircraft lost by groups 1, 2 and 3.)
type Pin = (
    u8,
    u64,
    &'static str,
    &'static str,
    u32,
    u32,
    u32,
    u32,
    i64,
    u32,
    [u32; 3],
);

fn case_of(number: u8) -> Case {
    [CASE_1, CASE_2, CASE_3, CASE_4][usize::from(number) - 1]
}

fn pin_of(number: u8, seed: u64, p: &'static str, b: &'static str) -> Pin {
    let case = case_of(number);
    let b_option = (number == 4).then_some(b);
    let world = run_seed(&load(case, p, b_option), seed);
    let events = morning(&world);
    let (patrol_before, patrol_after) = patrol_hits(&events);
    let (guns_before, guns_after) = gun_hits(&events);
    let bomb_hits = attack_of(&events).map_or(0, |a| a.3);
    (
        number,
        seed,
        p,
        b,
        patrol_before,
        patrol_after,
        guns_before,
        guns_after,
        attack_shots(&events),
        bomb_hits,
        [
            lost_by(&events, 1),
            lost_by(&events, 2),
            lost_by(&events, 3),
        ],
    )
}

/// PINNED, not calculated: see the table at the head of the file.
const PINNED: [Pin; 26] = [
    (1, 1942, "1", "1", 6, 0, 0, 0, 6, 0, [2, 6, 1]),
    (1, 1942, "0.4", "1", 2, 4, 0, 0, 10, 1, [2, 6, 1]),
    (1, 1942, "0", "1", 0, 6, 0, 0, 12, 2, [2, 6, 1]),
    (1, 1, "1", "1", 3, 0, 0, 0, 9, 2, [1, 3, 2]),
    (1, 1, "0.4", "1", 1, 2, 0, 0, 11, 2, [1, 3, 2]),
    (1, 1, "0", "1", 0, 3, 0, 0, 12, 2, [1, 3, 2]),
    (2, 1942, "1", "1", 2, 0, 0, 0, 0, 0, [0, 1, 0]),
    (2, 1942, "0.4", "1", 0, 2, 0, 0, 1, 0, [0, 1, 0]),
    (2, 1942, "0", "1", 0, 2, 0, 0, 1, 0, [0, 1, 0]),
    (2, 1, "1", "1", 2, 0, 0, 0, 0, 0, [0, 1, 0]),
    (2, 1, "0.4", "1", 1, 1, 0, 0, 0, 0, [0, 1, 0]),
    (2, 1, "0", "1", 0, 2, 0, 0, 1, 0, [0, 1, 0]),
    (3, 1942, "1", "1", 5, 0, 0, 0, 10, 1, [5, 0, 0]),
    (3, 1942, "0.4", "1", 4, 1, 0, 0, 11, 1, [5, 0, 0]),
    (3, 1942, "0", "1", 0, 5, 0, 0, 15, 1, [5, 0, 0]),
    (3, 1, "1", "1", 5, 0, 0, 0, 10, 1, [5, 0, 0]),
    (3, 1, "0.4", "1", 1, 4, 0, 0, 14, 2, [5, 0, 0]),
    (3, 1, "0", "1", 0, 5, 0, 0, 15, 2, [5, 0, 0]),
    (4, 1942, "1", "1", 2, 0, 0, 0, 10, 1, [0, 2, 0]),
    (4, 1942, "0.4", "1", 0, 2, 0, 0, 12, 2, [0, 2, 0]),
    (4, 1942, "0.4", "0", 0, 2, 0, 0, 12, 2, [0, 2, 0]),
    (4, 1942, "0", "0", 0, 2, 0, 0, 12, 2, [0, 2, 0]),
    (4, 1, "1", "1", 3, 0, 1, 0, 8, 2, [0, 4, 0]),
    (4, 1, "0.4", "1", 1, 2, 1, 0, 10, 2, [0, 4, 0]),
    (4, 1, "0.4", "0", 1, 2, 0, 1, 11, 2, [0, 4, 0]),
    (4, 1, "0", "0", 0, 3, 0, 1, 12, 2, [0, 4, 0]),
];

#[test]
fn pinned_the_outcomes_of_seed_1942_and_a_second_seed_in_each_case_at_each_share() {
    // Each pinned run also passes the hand calculation's checks.
    for pin in &PINNED {
        let case = case_of(pin.0);
        let b = (pin.0 == 4).then_some(pin.3);
        let world = run_seed(&load(case, pin.2, b), pin.1);
        check_morning(case, pin.2, b, pin.1, &world);
    }
    let got: Vec<Pin> = PINNED
        .iter()
        .map(|pin| pin_of(pin.0, pin.1, pin.2, pin.3))
        .collect();
    assert_eq!(got, PINNED.to_vec());
}
