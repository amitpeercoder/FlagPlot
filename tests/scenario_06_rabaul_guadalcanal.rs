//! Scenario 6: Rabaul against Guadalcanal in the air, 21 August to 17
//! September 1942.
//!
//! Stage 6e, which closes scenario 6 (decision 0042). The data is the frozen
//! copy `data/scenarios/step2-frozen/06_rabaul_guadalcanal.toml`, the
//! scenario as it stood before decision 0082 set its orders from the record,
//! run on the frozen figures with seeds 1 to 1000.
//! The air groups, aircraft types, arrivals and airfields are historical.
//! The combat, wear, fatigue and supply figures, each base's supply and
//! repairs a day, and the replacement rates are placeholders.
//!
//! It is the first scenario in which raids on a base (decision 0034), wear
//! and repair (0035), fatigue and rest (0036), pilots and replacements
//! (0038), base supply (0039), straight-line air distance (0040) and
//! arrivals (0041) all run together.
//!
//! # Hand calculation
//!
//! ## The setting
//!
//! Henderson Field, base 1, Allied, is in (2,-4): 4,000 t of supply, 8
//! anti-aircraft shots, 3 repairs a day. Rabaul, base 2, Japanese, is in
//! (-5,-13): 10,000 t, no guns, 4 repairs a day.
//!
//! The air distance between them is 40 x the square root of (7 x 7 + 9 x 9
//! + 7 x 9) = 40 x root 193 = 555.7, so 556 nm. A sortie from one to the
//! other is 1,112 nm.
//!
//! | Type | Radius | Weapon | Pilot armour | Protected tanks |
//! | ---- | ------ | ------ | ------------ | --------------- |
//! | F4F-4 | 105 nm | none | yes | yes |
//! | SBD-3 | 200 nm | a bomb | yes | yes |
//! | A6M2 | 556 nm | none | no | no |
//! | G4M1 | 611 nm | a bomb | no | no |
//!
//! So the G4M1s and the A6M2s can fly from Rabaul to Henderson Field, the
//! A6M2s with nothing to spare, and nothing at Henderson Field can reach
//! Rabaul.
//!
//! | Group | At | Type | Starts with | Mission | Rests above |
//! | ----- | -- | ---- | ----------- | ------- | ----------- |
//! | 1, the land attack bombers | Rabaul | G4M1 | 41 | base strike on Henderson Field | 45 |
//! | 2, Tainan Air Group | Rabaul | A6M2 | 24 | combat air patrol, share 0 | 45 |
//! | 3, 6th Air Group | Rabaul | A6M2 | none | combat air patrol, share 0 | 45 |
//! | 10, VMF-223 | Henderson | F4F-4 | 19 | combat air patrol, share 1 | 40 |
//! | 11, VMSB-232 | Henderson | SBD-3 | 12 | naval strike | never |
//! | 12, Flight 300 | Henderson | SBD-3 | none | naval strike | never |
//! | 13, VMF-224 | Henderson | F4F-4 | none | combat air patrol, share 1 | 40 |
//! | 14, VMSB-231 | Henderson | SBD-3 | none | naval strike | never |
//! | 15, VF-5 | Henderson | F4F-4 | none | combat air patrol, share 1 | 40 |
//! | 16, fighters from Hornet and Wasp | Henderson | F4F-4 | none | combat air patrol, share 1 | 40 |
//! | 17, VS-3 | Henderson | SBD-3 | none | naval strike | never |
//!
//! Every group has a pilot for each aircraft it starts with, and none
//! damaged. A group that starts with none has a strength of none.
//!
//! | Day | Arrival | Aircraft and pilots |
//! | --- | ------- | ------------------- |
//! | 4 | Group 12 | 11 |
//! | 10 | Group 13 | 19 |
//! | 10 | Group 14 | 12 |
//! | 11 | Group 3 | 13 |
//! | 22 | Group 15 | 24 |
//! | 24 | Group 16 | 18 |
//! | 24 | Group 17 | 12 |
//!
//! Rabaul receives two G4M1 and two A6M2 a day, each with a pilot.
//! Henderson Field receives nothing but the arrivals.
//!
//! ## The figures for one sortie
//!
//! | | A patrol | Rabaul to Henderson Field and back, 1,112 nm |
//! | - | -------- | -------------------------------------------- |
//! | Chance an aircraft is worn | 0.05 | 0.05 + 0.02 x 11.12 = 0.2724 |
//! | Fatigue the group gains | 3 | 3 + 2 x 11.12 = 25.24 |
//! | Supply for each aircraft sent | 0.5 t | 0.5 + 0.25 x 11.12 = 3.28 t |
//!
//! A worn aircraft is written off with chance 0.2. A group that did not fly
//! in a daylight pulse loses 5 fatigue. An aircraft at a base costs 0.2 t a
//! day and a repair 1 t.
//!
//! A fighter's shot hits an A6M2 or a G4M1 with chance 0.3, and an F4F-4
//! with 0.3 x 0.75 x 0.75 = 0.16875. A gun's shot hits a G4M1 with 0.05. A
//! bomb hits with 0.2, and lands on aircraft on the ground with 0.3, on
//! supply with 0.3, destroying 50 t, and on nothing with 0.4.
//!
//! A group with R ready aircraft, P pilots and fatigue f flies the smaller
//! of R and P, times (100 - f) / 100, rounded down, and nothing if it is
//! stood down or its base has no supply at the start of the pulse. Write
//! that fly(group).
//!
//! ## A day
//!
//! **Night, at the start of the day,** in this order:
//!
//! 1. Repair. Each base repairs damaged aircraft, the lowest-numbered
//!    group's first, up to its repairs a day and while it has 1 t for each,
//!    and pays 1 t for each.
//! 2. Arrivals due that day.
//! 3. Replacements, from day 2, in order of aircraft type (decision 0043,
//!    4e), so the fighters first: at Rabaul up to two A6M2 and two pilots
//!    to group 2 and then group 3, up to each one's strength; then up to
//!    two G4M1 and two pilots to group 1, up to its strength in each.
//! 4. The daily cost: 0.2 t for each aircraft at the base, ready or
//!    damaged.
//! 5. The staff stands down each group whose fatigue is above its
//!    threshold and returns each that is not.
//!
//! **Morning.** On the state at the start of the pulse:
//!
//! - Group 1 launches fly(1) bombers at Henderson Field, if that is at
//!   least one.
//! - Groups 2 and 3 keep nothing over Rabaul. If a raid launches, each sends
//!   every fighter it can fly as escort. If none does, they do not fly.
//! - Each Allied fighter group's patrol is all it can fly. The dive bombers
//!   do not fly: no contact is ever held.
//! - The raid is one raid and meets the whole patrol over Henderson Field,
//!   P fighters in all, with E escorts and B bombers:
//!   - the escorts fire E shots at the patrol, at 0.16875;
//!   - the patrol fires the smaller of P and E shots at the escorts and the
//!     rest of P at the bombers, at 0.3;
//!   - the guns fire 8 shots at the bombers left, at 0.05;
//!   - each bomber left drops a bomb at 0.2, and each hit lands by one
//!     draw.
//! - Losses to the patrol come off the lowest-numbered Allied fighter group
//!   first, and to the escorts off group 2 before group 3.
//! - The aircraft on the ground at Henderson Field are every ready aircraft
//!   there that is not flying and every damaged one: all the dive bombers,
//!   and whatever a fighter group leaves behind. Those destroyed are taken
//!   from the lowest-numbered group first, its damaged before its ready.
//! - A pilot is lost with every bomber and every escort shot down. No
//!   Allied pilot is ever lost: an Allied aircraft is lost only over its
//!   own field or on the ground.
//! - Then wear: each group that flew rolls for the aircraft that came back,
//!   at 0.2724 for the raid and the escort and 0.05 for a patrol.
//! - Then fatigue: 25.24 for groups 1, 2 and 3 if they flew, 3 for a patrol,
//!   and 5 off a group that did not fly.
//! - Then supply: Rabaul pays 3.28 t for each bomber and escort sent,
//!   Henderson Field 0.5 t for each fighter on patrol.
//!
//! **Afternoon.** No raid. The Japanese groups do not fly and rest. The
//! Allied fighter groups patrol, roll for wear at 0.05, gain 3, and
//! Henderson Field pays 0.5 t for each.
//!
//! ## The first day
//!
//! Night: nothing is damaged, nothing arrives, and nobody is stood down.
//! Rabaul pays (41 + 24) x 0.2 = 13 t and has 9,987 t. Henderson Field pays
//! (19 + 12) x 0.2 = 6.2 t and has 3,993.8 t.
//!
//! Morning: 41 bombers launch with 24 escorts, and VMF-223's patrol is 19.
//!
//! | Step | Shots | Chance | Expected | Std dev |
//! | ---- | ----- | ------ | -------- | ------- |
//! | Escorts at the patrol | 24 | 0.16875 | 4.05 | 1.834819 |
//! | Patrol at the escorts | 19 | 0.3 | 5.7 | 1.997498 |
//! | Patrol at the bombers | none: the 24 escorts hold all 19 | | | |
//! | Guns at the bombers | 8 | 0.05 | 0.4 | 0.616441 |
//!
//! The bombers that attack are 41 less the guns' hits, 40.6 on average, and
//! score 0.2 of their number in hits, 8.12 on average. The aircraft on the
//! ground are VMSB-232's twelve.
//!
//! Then, with e escorts and g bombers shot down:
//!
//! - Group 1 rolls 41 - g aircraft at 0.2724 and group 2 rolls 24 - e at
//!   0.2724. VMF-223 rolls its survivors at 0.05.
//! - Groups 1 and 2 are at a fatigue of 25.24, and VMF-223 at 3.
//! - Rabaul pays (41 + 24) x 3.28 = 213.2 t and has 9,773.8 t. Henderson
//!   Field pays 19 x 0.5 = 9.5 t, after 50 t for each bomb on its supply.
//!
//! Afternoon: groups 1 and 2 rest to 20.24. VMF-223 patrols with what it
//! can fly at a fatigue of 3 and ends the day at 6.
//!
//! Over seeds 1 to 1000 the first raid always flies, and the hits of each
//! of the three rolls, summed over the runs, are within four standard
//! deviations of the expected figure times 1,000. So are the bomb hits,
//! against 0.2 of the bombers that attacked.
//!
//! ## Fatigue and the days a raid flies
//!
//! Fatigue takes no roll. A group that is not stood down flies as long as
//! fly(group) is at least 1, and the tables below hold for every run in
//! which that is so on every day; the test works the rule for the others.
//!
//! Group 1, threshold 45. A day flown is +25.24 and -5. A day stood down is
//! -5 and -5.
//!
//! | Day | At the start | The staff | After the morning | After the afternoon |
//! | --- | ------------ | --------- | ----------------- | ------------------- |
//! | 1 | 0 | flies | 25.24 | 20.24 |
//! | 2 | 20.24 | flies | 45.48 | 40.48 |
//! | 3 | 40.48 | flies | 65.72 | 60.72 |
//! | 4 | 60.72 | stood down | 55.72 | 50.72 |
//! | 5 | 50.72 | stood down | 45.72 | 40.72 |
//! | 6 | 40.72 | returned, flies | 65.96 | 60.96 |
//! | 7 | 60.96 | stood down | 55.96 | 50.96 |
//! | 8 | 50.96 | stood down | 45.96 | 40.96 |
//! | 9 | 40.96 | returned, flies | 66.2 | 61.2 |
//!
//! From day 6 it flies every third day, starting each raid day 0.24 higher
//! than the last: 40.72 on day 6 and 40.72 + 0.24 x k on day 6 + 3 x k. So
//! raids fly on days 1, 2, 3, 6, 9, 12, 15, 18, 21, 24 and 27: eleven
//! raids. On day 1 it flies every bomber it has pilots for; on day 2, at
//! 20.24, 0.7976 of them; from day 3, at about 41, about 0.59 of them.
//!
//! Group 2 has the same threshold, gains the same on a raid day and flies
//! on no other day, so its fatigue is group 1's and it escorts every raid.
//!
//! Group 3 arrives on day 11 at a fatigue of 0 and escorts the raids from
//! day 12: it ends each raid day at 20.24 more than it began, less 5, and
//! rests 10 on each of the two days between, so it begins day 12 at 0, day
//! 15 at 0.24, day 18 at 0.48, and is never stood down.
//!
//! An Allied fighter group, threshold 40, gains 3 in each daylight pulse it
//! patrols: 6 a day. While it has an aircraft to fly in every pulse of
//! every day it is not stood down, its fatigue follows from the day it
//! first flies:
//!
//! | Group | First flies | Stood down on days |
//! | ----- | ----------- | ------------------ |
//! | 10, VMF-223 | 1 | 8, 11, 14, 16, 19, 22, 24, 27 |
//! | 13, VMF-224 | 10 | 17, 20, 23, 25, 28 |
//! | 15, VF-5 | 22 | none |
//! | 16, Hornet and Wasp | 24 | none |
//!
//! A group reaches 42 at the end of its seventh day of flying and is stood
//! down on the eighth. A day stood down takes 10 off and a day flown puts 6
//! on.
//!
//! The condition matters. Losses to the patrol come off the lowest-numbered
//! fighter group first, and Henderson Field receives no replacements, so
//! VMF-223 takes every loss until it has no aircraft. It runs out in every
//! one of the 1,000 runs, and its row of the table is then the recurrence
//! and not something a run shows. The test asserts each row in the runs
//! that meet the condition and the recurrence for the rest. (The first
//! version of this section said VMF-223 is on the ground for the raids of
//! days 24 and 27. It has no aircraft by then. No figure changed.)
//!
//! ## Over the four weeks
//!
//! The test works every pulse of every run from the state at its start, by
//! the rule above: repairs, arrivals, replacements, the daily cost, who is
//! stood down, what flies, the raid, the pilots lost, wear, fatigue and
//! supply.
//!
//! For every group in every run:
//!
//! - The aircraft it started with, those that arrived and those it received
//!   add up to its ready and damaged aircraft at the end, those shot down,
//!   those destroyed on the ground and those written off.
//! - The pilots it started with, those that arrived and those it received,
//!   less those lost, are its pilots at the end.
//! - Its strength is what it started with and what arrived.
//!
//! For each base, the supply at the end is the supply at the start less
//! what it paid by the day, for sorties and for repairs, and less what
//! bombs destroyed.
//!
//! ## Variants
//!
//! Each over seeds 1 to 100.
//!
//! | Variant | What changes |
//! | ------- | ------------ |
//! | The A6M2's radius is 261 nm, the figure from a carrier | No escort ever flies to Henderson Field: every raid is unescorted, and the whole patrol fires at the bombers. The fighters that cannot go are a patrol over Rabaul (decision 0030), which meets nobody |
//! | The G4M1's radius is 513 nm, a third of its other range row | No raid is ever launched. Nothing is lost at Henderson Field, which still patrols and pays for it |
//! | The scenario names no fatigue file | Nobody is stood down and everybody flies all they can: a raid on every morning group 1 has a bomber and a pilot |
//! | No arrivals | Groups 3 and 12 to 17 never have an aircraft. VMF-223 defends alone |
//! | No replacements at Rabaul | Groups 1, 2 and 3 receive nothing after their arrivals |
//!
//! ## What cannot happen
//!
//! - No Allied aircraft flies to Rabaul, nothing attacks Rabaul, and no
//!   Japanese aircraft is destroyed on the ground.
//! - No Allied pilot is lost.
//! - No raid flies on a day group 1 is stood down, in an afternoon or at
//!   night.
//! - No dive bomber flies, is worn or gains fatigue.
//! - No contact is held and no report is made: there are no ships.
//! - No group has aircraft before its arrival day.
//!
//! # Properties
//!
//! - Aircraft, pilots and supply are conserved, as "Over the four weeks"
//!   has it, in every run and every variant.
//! - No count and no stock is below nothing, no fatigue is outside 0 to
//!   100, and no group flies more than the smaller of its ready aircraft
//!   and its pilots.
//! - A raid is launched only at a base within the bombers' radius by the
//!   air distance, and escorts go only within theirs.
//! - Recording breakdowns changes neither the events nor the hashes.
//! - The same seed gives the same hashes twice.

// The architect's hand calculation above has a line that begins with a plus
// sign, which rustdoc reads as a list item, and the rebuild indexes arrays of
// two bases by number.
#![allow(clippy::doc_lazy_continuation, clippy::needless_range_loop)]

// The test code is written by a developer from the hand calculation above.

mod common;

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use sim_core::events::{
    AirFirer, AirTarget, Cause, Event, EventKind, FatigueChange, SortieKind, SupplyUse,
};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, AircraftTypeId, BaseId, EntityRef, EventId, OrderId};
use sim_core::orders::AirMission;
use sim_core::refdata::AttackWeapon;
use sim_core::rng::{Draw, RngKey, RngStream, Subsystem};
use sim_core::state::{AirGroupLocation, Side};
use sim_core::time::Pulse;
use sim_data::Scenario;

// ---------------------------------------------------------------------
// The setting, as constants. Index 0 of the arrays of two is Henderson
// Field, index 1 is Rabaul.
// ---------------------------------------------------------------------

const N: usize = 11;
const GROUPS: [u32; N] = [1, 2, 3, 10, 11, 12, 13, 14, 15, 16, 17];
const START: [u32; N] = [41, 24, 0, 19, 12, 0, 0, 0, 0, 0, 0];
const BOMBERS: u32 = 1;
const TAINAN: u32 = 2;
const SIXTH: u32 = 3;
const VMF_223: u32 = 10;
const ALLIED_FIGHTERS: [u32; 4] = [10, 13, 15, 16];
const JAPANESE_FIGHTERS: [u32; 2] = [2, 3];
const HENDERSON: BaseId = BaseId::new(1);
const RABAUL: BaseId = BaseId::new(2);
const START_STOCK: [i64; 2] = [400_000, 1_000_000];
const REPAIRS: [u32; 2] = [3, 4];
const GUNS: u32 = 8;
const SEEDS: u64 = 1000;
const VARIANT_SEEDS: u64 = 100;
const DAYS: u32 = 28;
/// The air distance between the two bases, and a sortie there and back.
const RAID_NM: i64 = 556;
const SORTIE_NM: i64 = 1112;
/// Arrivals: day, group, aircraft (and as many pilots).
const ARRIVALS: [(u32, u32, u32); 7] = [
    (4, 12, 11),
    (10, 13, 19),
    (10, 14, 12),
    (11, 3, 13),
    (22, 15, 24),
    (24, 16, 18),
    (24, 17, 12),
];
const RAID_DAYS: [u32; 11] = [1, 2, 3, 6, 9, 12, 15, 18, 21, 24, 27];

/// The frozen copy of scenario 6 (`data/scenarios/step2-frozen`), as it stood
/// before decision 0082 set its raid days and the dive bombers' searches from
/// the record: this test runs on the frozen figures, and frozen figures go
/// with frozen scenarios (`docs/step2-spec.md`, "Tests while figures move", 9).
fn scenario_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../data/scenarios/step2-frozen/06_rabaul_guadalcanal.toml")
}

fn load() -> Scenario {
    common::load(&scenario_path()).expect("scenario 6 loads")
}

fn n(value: i64) -> Fixed {
    Fixed::from_int(value)
}

fn tenths(value: i64) -> Fixed {
    Fixed::from_ratio(value, 10)
}

fn hundredths(value: i64) -> Fixed {
    Fixed::from_ratio(value, 100)
}

fn group(id: u32) -> AirGroupId {
    AirGroupId::new(id)
}

/// The index of a group number in the arrays of eleven.
fn ix(g: u32) -> usize {
    GROUPS
        .iter()
        .position(|id| *id == g)
        .unwrap_or_else(|| panic!("group {g}"))
}

fn count(value: usize) -> i64 {
    i64::try_from(value).expect("small")
}

fn japanese(g: u32) -> bool {
    g <= 3
}

/// 0 for a group at Henderson Field, 1 for a group at Rabaul.
fn base_ix(g: u32) -> usize {
    usize::from(japanese(g))
}

fn base_id(bi: usize) -> BaseId {
    [HENDERSON, RABAUL][bi]
}

/// The fatigue above which the staff stands a group down.
fn threshold(g: u32) -> Option<i64> {
    match g {
        1..=3 => Some(45),
        10 | 13 | 15 | 16 => Some(40),
        _ => None,
    }
}

fn type_of(g: u32) -> u32 {
    match g {
        1 => 2103,
        2 | 3 => 2102,
        10 | 13 | 15 | 16 => 2100,
        _ => 2101,
    }
}

/// Supply is worked in hundredths of a ton, as whole numbers.
fn fx(h: i64) -> Fixed {
    Fixed::from_ratio(h, 100)
}

/// Hundredths of a ton as the event log writes tons.
fn t(h: i64) -> String {
    assert!(h >= 0);
    if h % 100 == 0 {
        format!("{}", h / 100)
    } else if h % 10 == 0 {
        format!("{}.{}", h / 100, (h % 100) / 10)
    } else {
        format!("{}.{:02}", h / 100, h % 100)
    }
}

/// Whether k out of n is within four standard deviations of probability `p`
/// (in millionths), in exact integer arithmetic.
fn within_four_sigma(k: i64, n: i64, p: i64) -> bool {
    let (k, n, p) = (i128::from(k), i128::from(n), i128::from(p));
    let scale = 1_000_000;
    let off = k * scale - n * p;
    off * off <= 16 * n * p * (scale - p)
}

// ---------------------------------------------------------------------
// The rule, written from the hand calculation and nothing in sim-core.
// ---------------------------------------------------------------------

/// fly(R, P, f): the smaller of R and P times (100 - f) / 100, rounded down,
/// in exact integer arithmetic on millionths.
fn fly(ready: u32, pilots: u32, fatigue: Fixed) -> u32 {
    let hundred = 100 * i128::from(Fixed::SCALE);
    let crewed = i128::from(ready.min(pilots));
    let flown = crewed * (hundred - i128::from(fatigue.raw())) / hundred;
    u32::try_from(flown).expect("a count of aircraft")
}

/// The chance of a wear roll for a sortie of `distance_nm`.
fn wear_chance(distance_nm: i64) -> Fixed {
    Fixed::from_raw((50_000 + 20_000 * distance_nm / 100).min(Fixed::SCALE))
}

/// The fatigue a sortie of `distance_nm` gains.
fn fatigue_gain(distance_nm: i64) -> Fixed {
    Fixed::from_raw(3_000_000 + 2_000_000 * distance_nm / 100)
}

/// What one aircraft's sortie of `distance_nm` costs its base, in hundredths
/// of a ton.
fn sortie_cost(distance_nm: i64) -> i64 {
    50 + 25 * distance_nm / 100
}

/// What a fighter's shot hits with: the base chance times 0.75 for pilot
/// armour and 0.75 for protected tanks, each step truncated.
fn shot_chance(base: Fixed, g: u32) -> Fixed {
    let mut chance = base;
    if type_of(g) == 2100 {
        chance = chance * hundredths(75);
        chance = chance * hundredths(75);
    }
    chance
}

/// The days a group is stood down if it flies in every daylight pulse of every
/// day it is not stood down: fatigue gains 6 a day flown and loses 10 a day
/// stood down, from 0 on its first day.
fn stood_days_if_it_always_flies(first_day: u32, level: i64) -> Vec<u32> {
    let mut fatigue = Fixed::ZERO;
    let mut days = Vec::new();
    for day in first_day..=DAYS {
        if fatigue > n(level) {
            days.push(day);
            fatigue = (fatigue - n(10)).max(Fixed::ZERO);
        } else {
            fatigue += n(6);
        }
    }
    days
}

/// What differs between the scenario and its variants.
#[derive(Clone, Copy)]
struct Cfg {
    /// The scenario names the fatigue file.
    fatigue: bool,
    /// The scenario lists arrivals.
    arrivals: bool,
    /// The scenario gives Rabaul replacements.
    replacements: bool,
    /// The A6M2's radius reaches Henderson Field.
    escorts_reach: bool,
    /// The G4M1's radius reaches Henderson Field.
    raid_reach: bool,
}

fn scenario_cfg() -> Cfg {
    Cfg {
        fatigue: true,
        arrivals: true,
        replacements: true,
        escorts_reach: true,
        raid_reach: true,
    }
}

// ---------------------------------------------------------------------
// What one run did, read from its events and final state.
// ---------------------------------------------------------------------

struct Run {
    seed: u64,
    events: Vec<Event>,
    /// The indices of the events of each pulse, by day and pulse.
    index: BTreeMap<(u32, u32), Vec<usize>>,
    hashes: Vec<u64>,
    ready: [u32; N],
    damaged: [u32; N],
    pilots: [u32; N],
    strength: [u32; N],
    fatigue: [Fixed; N],
    stood: [bool; N],
    supply: [Fixed; 2],
    /// The contacts the two sides hold at the end.
    contacts: usize,
    /// The orders accepted for each air group, in the order of their ids.
    orders: BTreeMap<u32, Vec<OrderId>>,
}

fn pulse_order(pulse: Pulse) -> u32 {
    match pulse {
        Pulse::Night => 0,
        Pulse::Morning => 1,
        Pulse::Afternoon => 2,
    }
}

impl Run {
    fn cursor(&self, day: u32, pulse: Pulse) -> Cursor<'_> {
        let events = self
            .index
            .get(&(day, pulse_order(pulse)))
            .map(|indices| indices.iter().map(|i| &self.events[*i]).collect())
            .unwrap_or_default();
        Cursor {
            events,
            at: 0,
            seed: self.seed,
            day,
            pulse,
        }
    }

    fn orders_of(&self, g: u32) -> Vec<OrderId> {
        self.orders.get(&g).cloned().unwrap_or_default()
    }

    /// The order that set the group's mission: the first of its orders.
    fn mission_order(&self, g: u32) -> OrderId {
        self.orders_of(g)[0]
    }

    /// The order that set the group's rest threshold: its last.
    fn threshold_order(&self, g: u32) -> OrderId {
        *self.orders_of(g).last().expect("a threshold order")
    }

    /// The orders that say a fighter group keeps a patrol: its mission order,
    /// and for a Japanese group its share.
    fn patrol_orders(&self, g: u32) -> Vec<Cause> {
        let wanted = if japanese(g) { 2 } else { 1 };
        self.orders_of(g)
            .into_iter()
            .take(wanted)
            .map(Cause::Order)
            .collect()
    }
}

fn digest(run: &Scenario, seed: u64) -> Run {
    let truth = run.world.umpire();
    // The claims from the air (decision 0056) are not part of what this
    // scenario works out: they come after each raid is resolved and are
    // asserted in scenario_11_claims.rs.
    let events: Vec<Event> = run
        .world
        .events()
        .iter()
        .filter(|e| {
            !matches!(
                e.kind,
                sim_core::events::EventKind::ClaimReceived { from_air: true, .. }
            )
        })
        .cloned()
        .collect();
    let mut orders: BTreeMap<u32, Vec<OrderId>> = BTreeMap::new();
    let mut index: BTreeMap<(u32, u32), Vec<usize>> = BTreeMap::new();
    for (i, event) in events.iter().enumerate() {
        if let EventKind::OrderAccepted {
            order,
            target: EntityRef::AirGroup(target),
        } = event.kind
        {
            orders.entry(target.raw()).or_default().push(order);
        }
        if let Some(pulse) = event.pulse {
            index
                .entry((event.day, pulse_order(pulse)))
                .or_default()
                .push(i);
        }
    }
    let of = |g: u32| &truth.air_groups[&group(g)];
    Run {
        seed,
        events,
        index,
        hashes: run.world.day_hashes().to_vec(),
        ready: GROUPS.map(|g| of(g).ready),
        damaged: GROUPS.map(|g| of(g).damaged),
        pilots: GROUPS.map(|g| of(g).pilots.expect("pilots are tracked")),
        strength: GROUPS.map(|g| of(g).strength),
        fatigue: GROUPS.map(|g| of(g).fatigue),
        stood: GROUPS.map(|g| of(g).stood_down),
        supply: [
            truth.bases[&HENDERSON].supply_t,
            truth.bases[&RABAUL].supply_t,
        ],
        contacts: [Side::Allied, Side::Japanese]
            .into_iter()
            .map(|side| run.world.side_view(side).contacts().count())
            .sum(),
        orders,
    }
}

fn run_of(base: &Scenario, seed: u64) -> Run {
    let mut run = base.with_seed(seed).expect("a seed can be set");
    run.run_to_end().expect("the run resolves");
    digest(&run, seed)
}

/// A walk along the events of one pulse: the rule says what comes next and the
/// cursor hands out the event that is there.
struct Cursor<'a> {
    events: Vec<&'a Event>,
    at: usize,
    seed: u64,
    day: u32,
    pulse: Pulse,
}

impl<'a> Cursor<'a> {
    fn next(&mut self) -> &'a Event {
        let event = *self.events.get(self.at).unwrap_or_else(|| {
            panic!(
                "seed {}, day {}, {:?}: the rule makes more events than the log has",
                self.seed, self.day, self.pulse
            )
        });
        self.at += 1;
        event
    }

    fn finish(&self) {
        assert!(
            self.at == self.events.len(),
            "seed {}, day {}, {:?}: the log has events the rule does not make: {:?}",
            self.seed,
            self.day,
            self.pulse,
            self.events[self.at..]
                .iter()
                .map(|e| e.kind.to_string())
                .collect::<Vec<_>>()
        );
    }

    /// Asserts the kind and the causes of an event.
    fn check(&self, e: &Event, kind: EventKind, causes: Vec<Cause>) {
        assert_eq!(
            e.kind, kind,
            "seed {}, day {}, {:?}",
            self.seed, self.day, self.pulse
        );
        assert_eq!(
            e.causes, causes,
            "seed {}, day {}, {:?}: the causes of {}",
            self.seed, self.day, self.pulse, e.kind
        );
    }

    fn text(&self, e: &Event, text: String) {
        assert_eq!(
            e.kind.to_string(),
            text,
            "seed {}, day {}, {:?}",
            self.seed,
            self.day,
            self.pulse
        );
    }

    fn breakdown(&self, e: &Event, factors: &[(&'static str, Fixed)], result: Fixed) {
        let breakdown = e.breakdown.as_ref().expect("a breakdown");
        assert_eq!(
            breakdown.result, result,
            "seed {}, day {}, {:?}: the result of {}",
            self.seed, self.day, self.pulse, e.kind
        );
        assert_eq!(
            breakdown
                .factors
                .iter()
                .map(|f| (f.name, f.value))
                .collect::<Vec<_>>(),
            factors.to_vec(),
            "seed {}, day {}, {:?}: the factors of {}",
            self.seed,
            self.day,
            self.pulse,
            e.kind
        );
    }
}

fn stream_of(
    seed: u64,
    day: u32,
    pulse: Pulse,
    subsystem: Subsystem,
    entity: EntityRef,
    other: Option<EntityRef>,
) -> RngStream {
    RngKey {
        seed,
        day,
        pulse: Some(pulse),
        subsystem,
        entity,
        other,
        draw: Draw::Legacy,
    }
    .stream()
}

/// The hits a roll of `shots` at `chance` gives from its stream: one draw
/// for each shot (decision 0033).
fn hits_from(stream: &mut RngStream, shots: u32, chance: Fixed) -> u32 {
    (0..shots).map(|_| u32::from(stream.chance(chance))).sum()
}

/// Takes `hits` from the pool, the lowest group first, each until it has none
/// left. Returns what was taken from each group.
fn take(hits: u32, pool: &mut [(u32, u32)]) -> Vec<(u32, u32)> {
    let mut left = hits;
    let mut taken = Vec::new();
    for (group, count) in pool.iter_mut() {
        let now = left.min(*count);
        if now > 0 {
            *count -= now;
            left -= now;
            taken.push((*group, now));
        }
    }
    taken
}

/// Takes `hits` from the ground at Henderson Field: (group, is it its damaged
/// aircraft, how many), the lowest group first, a group's damaged before its
/// ready aircraft.
fn take_ground(hits: u32, pool: &mut [(u32, bool, u32)]) -> Vec<(u32, bool, u32)> {
    let mut left = hits;
    let mut taken = Vec::new();
    for (group, damaged, count) in pool.iter_mut() {
        let now = left.min(*count);
        if now > 0 {
            *count -= now;
            left -= now;
            taken.push((*group, *damaged, now));
        }
    }
    taken
}

// ---------------------------------------------------------------------
// Working every pulse of a run from the state at the start of it.
// ---------------------------------------------------------------------

/// What the run's state is, by the rule, as the pulses are worked.
#[derive(Clone)]
struct State {
    ready: [u32; N],
    damaged: [u32; N],
    pilots: [u32; N],
    strength: [u32; N],
    fatigue: [Fixed; N],
    stood: [bool; N],
    stood_event: [Option<EventId>; N],
    /// The latest wear roll that left an aircraft damaged, by group.
    damaging_wear: [Option<EventId>; N],
    /// Supply of Henderson Field and Rabaul, in hundredths of a ton.
    stock: [i64; 2],
}

impl State {
    fn new() -> State {
        State {
            ready: START,
            damaged: [0; N],
            pilots: START,
            strength: START,
            fatigue: [Fixed::ZERO; N],
            stood: [false; N],
            stood_event: [None; N],
            damaging_wear: [None; N],
            stock: START_STOCK,
        }
    }

    /// fly(group): what a group can fly in a daylight pulse that starts now.
    fn able(&self, cfg: &Cfg, g: u32, grounded: [bool; 2]) -> u32 {
        let i = ix(g);
        if grounded[base_ix(g)] || (cfg.fatigue && self.stood[i]) {
            return 0;
        }
        fly(self.ready[i], self.pilots[i], self.fatigue[i])
    }

    fn aircraft_at(&self, bi: usize) -> u32 {
        GROUPS
            .iter()
            .filter(|g| base_ix(**g) == bi)
            .map(|g| self.ready[ix(*g)] + self.damaged[ix(*g)])
            .sum()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum RollKind {
    EscortsAtPatrol,
    PatrolAtEscorts,
    PatrolAtBombers,
    Guns,
    Bombs,
}

/// One roll of the morning's raid as the rule works it.
#[derive(Clone, Copy, Debug)]
struct RollRec {
    kind: RollKind,
    shots: u32,
    chance: Fixed,
    hits: u32,
}

/// One wear roll as the rule works it.
#[derive(Clone, Copy, Debug)]
struct WearRec {
    group: u32,
    sortie: SortieKind,
    aircraft: u32,
    chance: Fixed,
    worn: u32,
    written_off: u32,
}

/// One day of a run, by the rule.
#[derive(Clone, Default)]
struct Day {
    /// After the night: who is stood down, the fatigue at the start of the
    /// morning, the stocks and the ready aircraft.
    stood: [bool; N],
    fatigue_start: [Fixed; N],
    ready_start: [u32; N],
    pilots_start: [u32; N],
    stock_night: [i64; 2],
    /// After the morning and the afternoon.
    fatigue_morning: [Fixed; N],
    fatigue_end: [Fixed; N],
    stock_morning: [i64; 2],
    stock_end: [i64; 2],
    ready_end: [u32; N],
    damaged_end: [u32; N],
    pilots_end: [u32; N],
    /// What each group can fly in each daylight pulse.
    able_morning: [u32; N],
    able_afternoon: [u32; N],
    /// The raid: launched, bombers launched, escorts sent by each group,
    /// Allied fighters on patrol in all and by each group.
    launched: bool,
    bombers: u32,
    escorts_by: [u32; N],
    patrol_by: [u32; N],
    /// The ready aircraft of each group on the ground at Henderson Field at
    /// the start of the air combat step, not flying; and the damaged.
    ground_ready: [u32; N],
    ground_damaged: [u32; N],
    /// The raid's rolls in order, bombs last.
    rolls: Vec<RollRec>,
    /// Aircraft shot down in the air, by group; Japanese bombers shot down
    /// by fighters and by guns; destroyed on the ground, by group.
    shot: [u32; N],
    bombers_by_fighters: u32,
    bombers_by_guns: u32,
    on_the_ground: [u32; N],
    pilots_lost: [u32; N],
    attacking: u32,
    hits_on_supply: u32,
    /// Tons of supply destroyed, in hundredths.
    supply_destroyed: i64,
    /// The wear rolls of the day, in order.
    wear: Vec<WearRec>,
    written_off: [u32; N],
    repaired: [u32; N],
    arrived: [u32; N],
    received_aircraft: [u32; N],
    received_pilots: [u32; N],
}

/// What working one run found, and what its events add up to.
struct Worked {
    seed: u64,
    days: Vec<Day>,
    hashes: Vec<u64>,
    /// The state at the end.
    ready: [u32; N],
    damaged: [u32; N],
    pilots: [u32; N],
    supply: [Fixed; 2],
    /// Totals over the run, by group, from the events.
    shot: [u32; N],
    on_the_ground: [u32; N],
    written_off: [u32; N],
    arrived: [u32; N],
    received_aircraft: [u32; N],
    received_pilots: [u32; N],
    pilots_lost: [u32; N],
    /// Tons destroyed at Henderson Field, in hundredths.
    supply_destroyed: i64,
}

impl Worked {
    fn raid_days(&self) -> Vec<u32> {
        (1..=DAYS)
            .filter(|day| self.days[(*day - 1) as usize].launched)
            .collect()
    }

    fn day(&self, day: u32) -> &Day {
        &self.days[(day - 1) as usize]
    }
}

/// A payment the rule expects: base, reason, aircraft, asked, paid, left.
type Payment = (BaseId, SupplyUse, u32, i64, i64, i64);

/// Takes what the stock covers off it and returns the payment.
fn pay(stock: &mut i64, base: BaseId, reason: SupplyUse, aircraft: u32, asked: i64) -> Payment {
    let paid = asked.min(*stock);
    *stock -= paid;
    (base, reason, aircraft, asked, paid, *stock)
}

/// Asserts that `e` is `payment`, in its fields, its words, its causes and its
/// breakdown.
fn assert_payment(
    cur: &Cursor,
    e: &Event,
    payment: &Payment,
    causes: Vec<Cause>,
    factors: &[(&'static str, Fixed)],
) {
    let (base, reason, aircraft, asked, paid, left) = *payment;
    cur.check(
        e,
        EventKind::BaseSupplySpent {
            base,
            reason,
            aircraft,
            men: 0,
            asked_t: fx(asked),
            paid_t: fx(paid),
            left_t: fx(left),
        },
        causes,
    );
    let what = match reason {
        SupplyUse::Sorties => format!("{aircraft} aircraft sent on sorties"),
        SupplyUse::Upkeep => format!("{aircraft} aircraft at the base for the day"),
        SupplyUse::Repairs => format!("{aircraft} repairs"),
        SupplyUse::Men => "men fed".to_string(),
        // Decision 0094: these scenarios have no march.
        SupplyUse::March => "men marching".to_string(),
    };
    let mut text = if paid < asked {
        format!(
            "{base} spent {} t of supply on {what}: {} t was asked and the stock could not cover it, {} t left",
            t(paid),
            t(asked),
            t(left)
        )
    } else {
        format!(
            "{base} spent {} t of supply on {what}, {} t left",
            t(paid),
            t(left)
        )
    };
    if left == 0 {
        text.push_str(&format!(": {base} is out of supply"));
    }
    cur.text(e, text);
    cur.breakdown(e, factors, fx(asked));
}

fn plural(count: u32, one: &str, many: &str) -> String {
    format!("{count} {}", if count == 1 { one } else { many })
}

fn work_the_run(run: &Run, cfg: &Cfg) -> Worked {
    let seed = run.seed;
    let mut state = State::new();
    let mut days: Vec<Day> = Vec::new();
    for day in 1..=DAYS {
        let mut rec = Day::default();
        work_the_night(run, cfg, day, &mut state, &mut rec);
        rec.stood = state.stood;
        rec.fatigue_start = state.fatigue;
        rec.ready_start = state.ready;
        rec.pilots_start = state.pilots;
        rec.stock_night = state.stock;
        assert_within_limits(&state, seed, day);
        work_the_morning(run, cfg, day, &mut state, &mut rec);
        rec.fatigue_morning = state.fatigue;
        rec.stock_morning = state.stock;
        assert_within_limits(&state, seed, day);
        work_the_afternoon(run, cfg, day, &mut state, &mut rec);
        rec.fatigue_end = state.fatigue;
        rec.stock_end = state.stock;
        rec.ready_end = state.ready;
        rec.damaged_end = state.damaged;
        rec.pilots_end = state.pilots;
        assert_within_limits(&state, seed, day);
        days.push(rec);
    }

    // The end of the run.
    assert_eq!(run.ready, state.ready, "seed {seed}: ready at the end");
    assert_eq!(
        run.damaged, state.damaged,
        "seed {seed}: damaged at the end"
    );
    assert_eq!(run.pilots, state.pilots, "seed {seed}: pilots at the end");
    assert_eq!(run.strength, state.strength, "seed {seed}: strength");
    assert_eq!(
        run.fatigue, state.fatigue,
        "seed {seed}: fatigue at the end"
    );
    assert_eq!(run.stood, state.stood, "seed {seed}: who is stood down");
    for bi in 0..2 {
        assert_eq!(
            run.supply[bi],
            fx(state.stock[bi]),
            "seed {seed}: the supply of base {} at the end",
            bi + 1
        );
    }
    let mut worked = Worked {
        seed,
        days,
        hashes: run.hashes.clone(),
        ready: state.ready,
        damaged: state.damaged,
        pilots: state.pilots,
        supply: run.supply,
        shot: [0; N],
        on_the_ground: [0; N],
        written_off: [0; N],
        arrived: [0; N],
        received_aircraft: [0; N],
        received_pilots: [0; N],
        pilots_lost: [0; N],
        supply_destroyed: 0,
    };
    for d in &worked.days {
        for i in 0..N {
            worked.shot[i] += d.shot[i];
            worked.on_the_ground[i] += d.on_the_ground[i];
            worked.written_off[i] += d.written_off[i];
            worked.arrived[i] += d.arrived[i];
            worked.received_aircraft[i] += d.received_aircraft[i];
            worked.received_pilots[i] += d.received_pilots[i];
            worked.pilots_lost[i] += d.pilots_lost[i];
        }
        worked.supply_destroyed += d.supply_destroyed;
    }
    worked
}

/// No group flies more than the smaller of its ready aircraft and its pilots.
fn assert_no_group_flies_more_than_it_can(seed: u64, day: u32, st: &State, able: &[u32; N]) {
    for g in GROUPS {
        let i = ix(g);
        assert!(
            able[i] <= st.ready[i].min(st.pilots[i]),
            "seed {seed}, day {day}, group {g}: flies {} of {} ready and {} pilots",
            able[i],
            st.ready[i],
            st.pilots[i]
        );
    }
}

/// No count and no stock is below nothing, no fatigue is outside 0 to 100, no
/// group has more aircraft or pilots than its strength.
fn assert_within_limits(state: &State, seed: u64, day: u32) {
    for g in GROUPS {
        let i = ix(g);
        assert!(
            state.fatigue[i] >= Fixed::ZERO && state.fatigue[i] <= n(100),
            "seed {seed}, day {day}, group {g}: fatigue {}",
            state.fatigue[i]
        );
        assert!(
            state.ready[i] + state.damaged[i] <= state.strength[i],
            "seed {seed}, day {day}, group {g}: aircraft over strength"
        );
        assert!(
            state.pilots[i] <= state.strength[i],
            "seed {seed}, day {day}, group {g}: pilots over strength"
        );
    }
    assert!(state.stock[0] >= 0 && state.stock[1] >= 0, "seed {seed}");
}

/// Night, at the start of the day: repair, arrivals, replacements, the daily
/// cost, the staff's stand-down decisions.
fn work_the_night(run: &Run, cfg: &Cfg, day: u32, st: &mut State, rec: &mut Day) {
    let mut cur = run.cursor(day, Pulse::Night);

    // 1. Repair, Henderson Field first, then Rabaul.
    for bi in 0..2 {
        let base = base_id(bi);
        let mut left = REPAIRS[bi];
        let mut paid_for = 0;
        let mut repair_events: Vec<Cause> = Vec::new();
        let damaged_groups: Vec<u32> = GROUPS
            .iter()
            .copied()
            .filter(|g| base_ix(*g) == bi && st.damaged[ix(*g)] > 0)
            .collect();
        for g in damaged_groups {
            if left == 0 {
                break;
            }
            let i = ix(g);
            let affordable = u32::try_from(st.stock[bi] / 100).expect("small");
            let repaired = left.min(st.damaged[i]).min(affordable);
            if repaired == 0 {
                break;
            }
            st.stock[bi] -= 100 * i64::from(repaired);
            paid_for += repaired;
            left -= repaired;
            st.damaged[i] -= repaired;
            st.ready[i] += repaired;
            rec.repaired[i] += repaired;
            let e = cur.next();
            cur.check(
                e,
                EventKind::AircraftRepaired {
                    air_group: group(g),
                    at: AirGroupLocation::Base(base),
                    repaired,
                    ready: st.ready[i],
                    damaged: st.damaged[i],
                },
                st.damaging_wear[i].into_iter().map(Cause::Event).collect(),
            );
            cur.text(
                e,
                format!(
                    "air_group:{g} had {repaired} aircraft repaired at {base}, {} ready and {} damaged",
                    st.ready[i], st.damaged[i]
                ),
            );
            assert!(e.breakdown.is_none());
            repair_events.push(Cause::Event(e.id));
        }
        if paid_for > 0 {
            let asked = 100 * i64::from(paid_for);
            // The stock was checked before each repair, so it is paid in full;
            // what is left is the stock after the last of them.
            let payment: Payment = (
                base,
                SupplyUse::Repairs,
                paid_for,
                asked,
                asked,
                st.stock[bi],
            );
            let e = cur.next();
            assert_payment(
                &cur,
                e,
                &payment,
                repair_events,
                &[
                    ("repairs", n(i64::from(paid_for))),
                    ("repair_supply_t", fx(100)),
                ],
            );
        }
    }

    // 2. Arrivals due today.
    if cfg.arrivals {
        for (arrival_day, g, aircraft) in ARRIVALS {
            if arrival_day != day {
                continue;
            }
            let i = ix(g);
            st.ready[i] += aircraft;
            st.strength[i] += aircraft;
            st.pilots[i] += aircraft;
            rec.arrived[i] += aircraft;
            let e = cur.next();
            cur.check(
                e,
                EventKind::AircraftArrived {
                    air_group: group(g),
                    aircraft,
                    pilots: aircraft,
                    ready: st.ready[i],
                    pilots_now: st.pilots[i],
                    strength: st.strength[i],
                },
                Vec::new(),
            );
            cur.text(
                e,
                format!(
                    "air_group:{g} received {aircraft} aircraft and {} by arrival, {} ready, {} and strength {}",
                    plural(aircraft, "pilot", "pilots"),
                    st.ready[i],
                    plural(st.pilots[i], "pilot", "pilots"),
                    st.strength[i]
                ),
            );
        }
    }

    // 3. Replacements from day 2, fighters first (decision 0043, 4e): two
    // fighters and two pilots to group 2, then group 3; then two bombers and
    // two pilots to group 1, each up to its strength.
    if cfg.replacements && day >= 2 {
        for groups in [&JAPANESE_FIGHTERS[..], &[BOMBERS][..]] {
            let mut aircraft_left = 2u32;
            let mut pilots_left = 2u32;
            for &g in groups {
                let i = ix(g);
                let aircraft =
                    aircraft_left.min(st.strength[i].saturating_sub(st.ready[i] + st.damaged[i]));
                let pilots = pilots_left.min(st.strength[i].saturating_sub(st.pilots[i]));
                if aircraft == 0 && pilots == 0 {
                    continue;
                }
                aircraft_left -= aircraft;
                pilots_left -= pilots;
                st.ready[i] += aircraft;
                st.pilots[i] += pilots;
                rec.received_aircraft[i] += aircraft;
                rec.received_pilots[i] += pilots;
                let e = cur.next();
                cur.check(
                    e,
                    EventKind::ReplacementsReceived {
                        air_group: group(g),
                        base: RABAUL,
                        aircraft,
                        pilots,
                        ready: st.ready[i],
                        pilots_now: st.pilots[i],
                    },
                    Vec::new(),
                );
                cur.text(
                    e,
                    format!(
                        "air_group:{g} received {aircraft} aircraft and {pilots} pilots at {RABAUL}, {} ready and {} pilots",
                        st.ready[i], st.pilots[i]
                    ),
                );
            }
        }
    }

    // 4. The daily cost: 0.2 t for each aircraft at the base, ready or
    // damaged, as much as the stock covers; a base with no stock pays nothing.
    for bi in 0..2 {
        let aircraft = st.aircraft_at(bi);
        let asked = 20 * i64::from(aircraft);
        if asked == 0 || st.stock[bi] == 0 {
            continue;
        }
        let payment = pay(
            &mut st.stock[bi],
            base_id(bi),
            SupplyUse::Upkeep,
            aircraft,
            asked,
        );
        let e = cur.next();
        assert_payment(
            &cur,
            e,
            &payment,
            Vec::new(),
            &[
                ("aircraft_at_base", n(i64::from(aircraft))),
                ("daily_supply_t_per_aircraft", fx(20)),
            ],
        );
    }

    // 5. The staff stands down each group above its threshold and returns
    // each that is not.
    if cfg.fatigue {
        for g in GROUPS {
            let Some(level) = threshold(g) else {
                continue;
            };
            let i = ix(g);
            let above = st.fatigue[i] > n(level);
            if above == st.stood[i] {
                continue;
            }
            st.stood[i] = above;
            let fatigue = st.fatigue[i];
            let e = cur.next();
            let (kind, text) = if above {
                (
                    EventKind::AirGroupStoodDown {
                        air_group: group(g),
                        fatigue,
                        threshold: n(level),
                    },
                    format!(
                        "air_group:{g} stood down: fatigue {fatigue} is above its threshold of {level}"
                    ),
                )
            } else {
                (
                    EventKind::AirGroupReturned {
                        air_group: group(g),
                        fatigue,
                        threshold: n(level),
                    },
                    format!(
                        "air_group:{g} returned to flying: fatigue {fatigue} is not above its threshold of {level}"
                    ),
                )
            };
            cur.check(e, kind, vec![Cause::Order(run.threshold_order(g))]);
            cur.text(e, text);
            cur.breakdown(
                e,
                &[("fatigue", fatigue), ("rest_threshold", n(level))],
                fatigue,
            );
            if above {
                st.stood_event[i] = Some(e.id);
            }
        }
    }
    cur.finish();
}

/// What one group flew in a daylight pulse, for the wear, fatigue and supply
/// steps.
struct Sortie {
    g: u32,
    kind: SortieKind,
    /// The aircraft that flew, and those that came back.
    sent: u32,
    aircraft: u32,
    distance_nm: i64,
    causes: Vec<Cause>,
}

/// What the raid left for the steps after the air combat.
struct RaidOut {
    launch: EventId,
    sent: Vec<(u32, EventId)>,
    /// Aircraft lost in the air: group, what the sortie was, how many.
    losses: Vec<(u32, SortieKind, u32)>,
}

fn work_the_morning(run: &Run, cfg: &Cfg, day: u32, st: &mut State, rec: &mut Day) {
    let mut cur = run.cursor(day, Pulse::Morning);
    // A base with no supply at the start of the pulse flies nothing in it.
    let grounded = [st.stock[0] == 0, st.stock[1] == 0];
    let able = GROUPS.map(|g| st.able(cfg, g, grounded));
    assert_no_group_flies_more_than_it_can(run.seed, day, st, &able);
    rec.able_morning = able;
    let b = able[ix(BOMBERS)];
    let raid = b >= 1 && cfg.raid_reach;
    // The Japanese fighters keep nothing over Rabaul and are ready to escort;
    // the Allied fighters keep all they can fly as patrol.
    let mut patrol = [0u32; N];
    let mut escorts = [0u32; N];
    let mut sent = [false; N];
    for g in JAPANESE_FIGHTERS {
        escorts[ix(g)] = able[ix(g)];
    }
    for g in ALLIED_FIGHTERS {
        patrol[ix(g)] = able[ix(g)];
    }
    if raid {
        for g in JAPANESE_FIGHTERS {
            let i = ix(g);
            if escorts[i] == 0 {
                continue;
            }
            if cfg.escorts_reach {
                sent[i] = true;
            } else {
                // Out of radius: they stay and keep all they can fly as
                // patrol over Rabaul (decision 0030).
                patrol[i] += escorts[i];
                escorts[i] = 0;
            }
        }
    }
    rec.launched = raid;
    rec.bombers = if raid { b } else { 0 };
    for g in GROUPS {
        let i = ix(g);
        rec.patrol_by[i] = patrol[i];
        rec.escorts_by[i] = if sent[i] { escorts[i] } else { 0 };
    }

    let mut sorties: Vec<Sortie> = Vec::new();
    for g in JAPANESE_FIGHTERS.into_iter().chain(ALLIED_FIGHTERS) {
        let i = ix(g);
        if patrol[i] > 0 {
            sorties.push(Sortie {
                g,
                kind: SortieKind::Patrol,
                sent: patrol[i],
                aircraft: patrol[i],
                distance_nm: 0,
                causes: run.patrol_orders(g),
            });
        }
    }
    if raid {
        let out = work_the_raid(run, day, st, rec, &mut cur, b, [&patrol, &escorts], &sent);
        sorties.push(Sortie {
            g: BOMBERS,
            kind: SortieKind::BaseStrike,
            sent: b,
            aircraft: b,
            distance_nm: SORTIE_NM,
            causes: vec![Cause::Event(out.launch)],
        });
        for (g, event) in &out.sent {
            sorties.push(Sortie {
                g: *g,
                kind: SortieKind::Escort,
                sent: escorts[ix(*g)],
                aircraft: escorts[ix(*g)],
                distance_nm: SORTIE_NM,
                causes: vec![Cause::Event(*event)],
            });
        }
        for (g, lost_from, lost) in &out.losses {
            let sortie = sorties
                .iter_mut()
                .find(|s| s.g == *g && s.kind == *lost_from)
                .expect("a loss comes off a sortie that flew");
            sortie.aircraft -= lost;
        }
    }
    work_the_upkeep(run, cfg, day, Pulse::Morning, st, rec, &mut cur, sorties);
}

fn work_the_afternoon(run: &Run, cfg: &Cfg, day: u32, st: &mut State, rec: &mut Day) {
    let mut cur = run.cursor(day, Pulse::Afternoon);
    let grounded = [st.stock[0] == 0, st.stock[1] == 0];
    let able = GROUPS.map(|g| st.able(cfg, g, grounded));
    assert_no_group_flies_more_than_it_can(run.seed, day, st, &able);
    rec.able_afternoon = able;
    // No raid flies in the afternoon. The Japanese groups keep no patrol; the
    // Allied fighter groups patrol with all they can fly.
    let mut sorties = Vec::new();
    for g in ALLIED_FIGHTERS {
        let flying = able[ix(g)];
        if flying > 0 {
            sorties.push(Sortie {
                g,
                kind: SortieKind::Patrol,
                sent: flying,
                aircraft: flying,
                distance_nm: 0,
                causes: run.patrol_orders(g),
            });
        }
    }
    work_the_upkeep(run, cfg, day, Pulse::Afternoon, st, rec, &mut cur, sorties);
}

/// The morning's raid: the launch, the escorts, the fights, the guns, the
/// losses, the pilots, the bombs and what they hit.
#[allow(clippy::too_many_arguments)]
fn work_the_raid(
    run: &Run,
    day: u32,
    st: &mut State,
    rec: &mut Day,
    cur: &mut Cursor<'_>,
    b: u32,
    flying: [&[u32; N]; 2],
    sent_flag: &[bool; N],
) -> RaidOut {
    let seed = run.seed;
    let pulse = Pulse::Morning;
    let [patrol, escorts] = flying;
    let launch = cur.next();
    cur.check(
        launch,
        EventKind::BaseStrikeLaunched {
            air_group: group(BOMBERS),
            base: HENDERSON,
            hex: Hex::new(2, -4),
            aircraft: b,
            distance_nm: n(RAID_NM),
        },
        vec![Cause::Order(run.mission_order(BOMBERS))],
    );
    assert!(
        !st.stood[ix(BOMBERS)],
        "seed {seed}, day {day}: no raid flies on a day group 1 is stood down"
    );

    // What is flying, and the aircraft on the ground at Henderson Field.
    let mut flying_now = [0u32; N];
    for g in GROUPS {
        let i = ix(g);
        flying_now[i] = patrol[i] + if sent_flag[i] { escorts[i] } else { 0 };
    }
    flying_now[ix(BOMBERS)] = b;
    let mut ground_pool: Vec<(u32, bool, u32)> = Vec::new();
    for g in GROUPS.into_iter().filter(|g| !japanese(*g)) {
        let i = ix(g);
        if st.damaged[i] > 0 {
            ground_pool.push((g, true, st.damaged[i]));
        }
        let resting = st.ready[i] - flying_now[i];
        if resting > 0 {
            ground_pool.push((g, false, resting));
        }
        rec.ground_damaged[i] = st.damaged[i];
        rec.ground_ready[i] = resting;
    }
    let on_ground: u32 = ground_pool.iter().map(|(_, _, count)| count).sum();

    // The escorts sent, in order of group.
    let sent_groups: Vec<u32> = JAPANESE_FIGHTERS
        .into_iter()
        .filter(|g| sent_flag[ix(*g)])
        .collect();
    let mut sent_events: Vec<(u32, EventId)> = Vec::new();
    for &g in &sent_groups {
        let e = cur.next();
        let mut causes = run.patrol_orders(g);
        causes.push(Cause::Event(launch.id));
        cur.check(
            e,
            EventKind::EscortsSent {
                air_group: group(g),
                with_air_group: group(BOMBERS),
                aircraft: escorts[ix(g)],
            },
            causes,
        );
        sent_events.push((g, e.id));
    }
    let e_total: u32 = sent_groups.iter().map(|g| escorts[ix(*g)]).sum();
    let patrol_groups: Vec<u32> = ALLIED_FIGHTERS
        .into_iter()
        .filter(|g| patrol[ix(*g)] > 0)
        .collect();
    let p_total: u32 = patrol_groups.iter().map(|g| patrol[ix(*g)]).sum();

    // The fights, on the counts at the start. The raid is alone, so it meets
    // the whole patrol.
    struct Planned {
        kind: RollKind,
        firer: u32,
        target: u32,
        shots: u32,
        chance: Fixed,
        factors: Vec<(&'static str, Fixed)>,
        hits: u32,
        lost: Vec<(u32, u32)>,
    }
    let mut escort_pool: Vec<(u32, u32)> =
        sent_groups.iter().map(|g| (*g, escorts[ix(*g)])).collect();
    let mut patrol_pool: Vec<(u32, u32)> =
        patrol_groups.iter().map(|g| (*g, patrol[ix(*g)])).collect();
    let mut bomber_pool = [(BOMBERS, b)];
    let mut planned: Vec<Planned> = Vec::new();
    let raid_aircraft = b + e_total;
    let patrol_factors = |shots: u32, chance: Fixed| -> Vec<(&'static str, Fixed)> {
        vec![
            ("patrol", n(i64::from(p_total))),
            ("raid_aircraft", n(i64::from(raid_aircraft))),
            ("all_raids_aircraft", n(i64::from(raid_aircraft))),
            ("patrol_share", n(i64::from(p_total))),
            ("shots", n(i64::from(shots))),
            ("hit_chance", chance),
        ]
    };
    if p_total > 0 {
        let first_patrol = patrol_groups[0];
        if let Some(&first_escort) = sent_groups.first() {
            let chance = shot_chance(tenths(3), first_patrol);
            let mut stream = stream_of(
                seed,
                day,
                pulse,
                Subsystem::AirCombat,
                EntityRef::AirGroup(group(first_escort)),
                Some(EntityRef::Base(HENDERSON)),
            );
            let hits = hits_from(&mut stream, e_total, chance);
            planned.push(Planned {
                kind: RollKind::EscortsAtPatrol,
                firer: first_escort,
                target: first_patrol,
                shots: e_total,
                chance,
                factors: vec![
                    ("shots", n(i64::from(e_total))),
                    ("hit_chance", tenths(3)),
                    ("pilot_armour_factor", hundredths(75)),
                    ("self_sealing_tanks_factor", hundredths(75)),
                ],
                hits,
                lost: take(hits, &mut patrol_pool),
            });
        }
        let mut stream = stream_of(
            seed,
            day,
            pulse,
            Subsystem::AirCombat,
            EntityRef::AirGroup(group(first_patrol)),
            Some(EntityRef::AirGroup(group(BOMBERS))),
        );
        let at_escorts = p_total.min(e_total);
        if let Some(&first_escort) = sent_groups.first() {
            let chance = shot_chance(tenths(3), first_escort);
            let hits = hits_from(&mut stream, at_escorts, chance);
            planned.push(Planned {
                kind: RollKind::PatrolAtEscorts,
                firer: first_patrol,
                target: first_escort,
                shots: at_escorts,
                chance,
                factors: patrol_factors(at_escorts, chance),
                hits,
                lost: take(hits, &mut escort_pool),
            });
        }
        let at_bombers = p_total - at_escorts;
        if at_bombers > 0 {
            let chance = shot_chance(tenths(3), BOMBERS);
            let hits = hits_from(&mut stream, at_bombers, chance);
            planned.push(Planned {
                kind: RollKind::PatrolAtBombers,
                firer: first_patrol,
                target: BOMBERS,
                shots: at_bombers,
                chance,
                factors: patrol_factors(at_bombers, chance),
                hits,
                lost: take(hits, &mut bomber_pool),
            });
        }
    }
    // The guns of Henderson Field at the bombers left.
    if bomber_pool[0].1 > 0 {
        let mut stream = stream_of(
            seed,
            day,
            pulse,
            Subsystem::AirCombat,
            EntityRef::Base(HENDERSON),
            Some(EntityRef::AirGroup(group(BOMBERS))),
        );
        let chance = hundredths(5);
        let hits = hits_from(&mut stream, GUNS, chance);
        planned.push(Planned {
            kind: RollKind::Guns,
            firer: 0,
            target: BOMBERS,
            shots: GUNS,
            chance,
            factors: vec![("shots", n(i64::from(GUNS))), ("hit_chance", chance)],
            hits,
            lost: take(hits, &mut bomber_pool),
        });
    }

    // The rolls are recorded first, in order.
    let mut roll_events: Vec<EventId> = Vec::new();
    for roll in &planned {
        let e = cur.next();
        let expected = n(i64::from(roll.shots)) * roll.chance;
        let (kind, causes) = if roll.kind == RollKind::Guns {
            (
                EventKind::BaseAntiAircraftFire {
                    base: HENDERSON,
                    air_group: group(BOMBERS),
                    shots: n(i64::from(roll.shots)),
                    chance: roll.chance,
                    expected,
                    hits: roll.hits,
                    after_attack: false,
                },
                vec![Cause::Event(launch.id)],
            )
        } else {
            let (firing, at) = match roll.kind {
                RollKind::EscortsAtPatrol => (AirFirer::Escorts, AirTarget::Patrol),
                RollKind::PatrolAtEscorts => (AirFirer::Patrol, AirTarget::Escorts),
                _ => (AirFirer::Patrol, AirTarget::Bombers),
            };
            let mut causes = vec![Cause::Event(launch.id)];
            // A fight with escorts in it is explained by them.
            if firing == AirFirer::Escorts || at == AirTarget::Escorts {
                causes.extend(sent_events.iter().map(|(_, id)| Cause::Event(*id)));
            }
            (
                EventKind::AirFight {
                    firer: group(roll.firer),
                    target: group(roll.target),
                    firing,
                    at,
                    shots: n(i64::from(roll.shots)),
                    chance: roll.chance,
                    expected,
                    hits: roll.hits,
                    after_attack: false,
                },
                causes,
            )
        };
        cur.check(e, kind, causes);
        cur.breakdown(e, &roll.factors, expected);
        roll_events.push(e.id);
        rec.rolls.push(RollRec {
            kind: roll.kind,
            shots: roll.shots,
            chance: roll.chance,
            hits: roll.hits,
        });
    }

    // Then the aircraft lost, roll by roll, each followed by the pilots that
    // went with it, if it was lost away from home.
    let mut losses: Vec<(u32, SortieKind, u32)> = Vec::new();
    let mut loss_events: Vec<(u32, EventId)> = Vec::new();
    for (roll, cause) in planned.iter().zip(&roll_events) {
        let lost_from = match roll.kind {
            RollKind::EscortsAtPatrol => SortieKind::Patrol,
            RollKind::PatrolAtEscorts => SortieKind::Escort,
            _ => SortieKind::BaseStrike,
        };
        for &(g, lost) in &roll.lost {
            let i = ix(g);
            st.ready[i] -= lost;
            rec.shot[i] += lost;
            match roll.kind {
                RollKind::PatrolAtBombers => rec.bombers_by_fighters += lost,
                RollKind::Guns => rec.bombers_by_guns += lost,
                _ => {}
            }
            let e = cur.next();
            cur.check(
                e,
                EventKind::AircraftLost {
                    air_group: group(g),
                    aircraft: lost,
                    remaining: st.ready[i],
                },
                vec![Cause::Event(*cause)],
            );
            cur.text(
                e,
                format!("air_group:{g} lost {lost} aircraft, {} left", st.ready[i]),
            );
            losses.push((g, lost_from, lost));
            loss_events.push((g, e.id));
            if lost_from != SortieKind::Patrol {
                let taken = lost.min(st.pilots[i]);
                if taken > 0 {
                    st.pilots[i] -= taken;
                    rec.pilots_lost[i] += taken;
                    let p = cur.next();
                    cur.check(
                        p,
                        EventKind::PilotsLost {
                            air_group: group(g),
                            lost: taken,
                            left: st.pilots[i],
                        },
                        vec![Cause::Event(e.id)],
                    );
                    cur.text(
                        p,
                        format!(
                            "air_group:{g} lost {}, {} left",
                            plural(taken, "pilot", "pilots"),
                            st.pilots[i]
                        ),
                    );
                }
            }
        }
    }
    assert!(
        patrol_groups.iter().all(|g| !japanese(*g)),
        "the patrol over Henderson Field is Allied"
    );

    // The bombers left each drop a bomb, and each hit lands by one draw.
    let bombers_left = bomber_pool[0].1;
    rec.attacking = bombers_left;
    if bombers_left > 0 {
        let mut stream = stream_of(
            seed,
            day,
            pulse,
            Subsystem::AirCombat,
            EntityRef::AirGroup(group(BOMBERS)),
            Some(EntityRef::Base(HENDERSON)),
        );
        let chance = tenths(2);
        let hits = hits_from(&mut stream, bombers_left, chance);
        let (mut on_aircraft, mut on_supply) = (0u32, 0u32);
        for _ in 0..hits {
            let draw = stream.below(Fixed::SCALE.unsigned_abs());
            if draw < 300_000 {
                on_aircraft += 1;
            } else if draw < 600_000 {
                on_supply += 1;
            }
        }
        rec.hits_on_supply = on_supply;
        rec.rolls.push(RollRec {
            kind: RollKind::Bombs,
            shots: bombers_left,
            chance,
            hits,
        });
        let expected = n(i64::from(bombers_left)) * chance;
        let attacked = cur.next();
        let mut causes = vec![Cause::Event(launch.id)];
        causes.extend(
            loss_events
                .iter()
                .filter(|(g, _)| *g == BOMBERS)
                .map(|(_, id)| Cause::Event(*id)),
        );
        cur.check(
            attacked,
            EventKind::BaseAttacked {
                air_group: group(BOMBERS),
                base: HENDERSON,
                shots: n(i64::from(bombers_left)),
                chance,
                expected,
                hits,
                hits_on_aircraft: on_aircraft,
                hits_on_supply: on_supply,
                // No men ashore at the base (decision 0092, 8).
                hits_on_men: 0,
            },
            causes,
        );
        cur.breakdown(
            attacked,
            &[
                ("aircraft", n(i64::from(bombers_left))),
                ("hit_chance", chance),
                ("hit_on_aircraft_chance", tenths(3)),
                ("hit_on_supply_chance", tenths(3)),
            ],
            expected,
        );

        // Hits on aircraft destroy aircraft on the ground, the lowest group
        // first, its damaged before its ready, no more than were there.
        for (g, damaged, destroyed) in take_ground(on_aircraft, &mut ground_pool) {
            let i = ix(g);
            let e = cur.next();
            let kind = if damaged {
                st.damaged[i] -= destroyed;
                EventKind::DamagedAircraftDestroyedOnGround {
                    air_group: group(g),
                    base: HENDERSON,
                    aircraft: destroyed,
                    remaining: st.damaged[i],
                }
            } else {
                st.ready[i] -= destroyed;
                EventKind::AircraftDestroyedOnGround {
                    air_group: group(g),
                    base: HENDERSON,
                    aircraft: destroyed,
                    remaining: st.ready[i],
                }
            };
            rec.on_the_ground[i] += destroyed;
            cur.check(e, kind, vec![Cause::Event(attacked.id)]);
            cur.breakdown(
                e,
                &[
                    ("hits_on_aircraft", n(i64::from(on_aircraft))),
                    ("aircraft_on_ground", n(i64::from(on_ground))),
                ],
                n(i64::from(on_aircraft.min(on_ground))),
            );
        }
        // Hits on supply destroy 50 t each, no more than the base has.
        let destroyed = (5000 * i64::from(on_supply)).min(st.stock[0]);
        if destroyed > 0 {
            let before = st.stock[0];
            st.stock[0] -= destroyed;
            rec.supply_destroyed += destroyed;
            let e = cur.next();
            cur.check(
                e,
                EventKind::BaseSupplyDestroyed {
                    base: HENDERSON,
                    destroyed_t: fx(destroyed),
                    supply_t: fx(st.stock[0]),
                },
                vec![Cause::Event(attacked.id)],
            );
            cur.breakdown(
                e,
                &[
                    ("hits_on_supply", n(i64::from(on_supply))),
                    ("tons_per_hit", n(50)),
                    ("supply_before", fx(before)),
                ],
                fx(destroyed),
            );
        }
    }
    RaidOut {
        launch: launch.id,
        sent: sent_events,
        losses,
    }
}

/// Wear, fatigue and supply after the air combat of a daylight pulse: each
/// sortie that came back rolls for wear; each group fatigues or rests; each
/// base pays for the sorties flown from it.
#[allow(clippy::too_many_arguments)]
fn work_the_upkeep(
    run: &Run,
    cfg: &Cfg,
    day: u32,
    pulse: Pulse,
    st: &mut State,
    rec: &mut Day,
    cur: &mut Cursor<'_>,
    mut sorties: Vec<Sortie>,
) {
    let seed = run.seed;
    // In order of group; a group's patrol before its escorts.
    sorties.sort_by_key(|sortie| sortie.g);

    // Wear: each aircraft that came back, then each worn one, from one stream
    // of the group.
    let mut streams: BTreeMap<u32, RngStream> = BTreeMap::new();
    for sortie in sorties.iter().filter(|s| s.aircraft > 0) {
        let g = sortie.g;
        let i = ix(g);
        let chance = wear_chance(sortie.distance_nm);
        let stream = streams.entry(g).or_insert_with(|| {
            stream_of(
                seed,
                day,
                pulse,
                Subsystem::Upkeep,
                EntityRef::AirGroup(group(g)),
                None,
            )
        });
        let worn = hits_from(stream, sortie.aircraft, chance);
        let written_off = hits_from(stream, worn, tenths(2));
        st.ready[i] -= worn;
        st.damaged[i] += worn - written_off;
        rec.written_off[i] += written_off;
        rec.wear.push(WearRec {
            group: g,
            sortie: sortie.kind,
            aircraft: sortie.aircraft,
            chance,
            worn,
            written_off,
        });
        let expected = n(i64::from(sortie.aircraft)) * chance;
        let e = cur.next();
        cur.check(
            e,
            EventKind::AircraftWorn {
                air_group: group(g),
                sortie: sortie.kind,
                distance_nm: n(sortie.distance_nm),
                aircraft: sortie.aircraft,
                chance,
                expected,
                worn,
                written_off,
                ready: st.ready[i],
                damaged: st.damaged[i],
            },
            sortie.causes.clone(),
        );
        cur.text(
            e,
            format!(
                "air_group:{g} {} sortie of {} nm: {} aircraft x {chance} = {expected} expected worn, {worn} worn, {written_off} written off, {} ready and {} damaged left",
                sortie.kind.name(),
                sortie.distance_nm,
                sortie.aircraft,
                st.ready[i],
                st.damaged[i]
            ),
        );
        cur.breakdown(
            e,
            &[
                ("aircraft", n(i64::from(sortie.aircraft))),
                ("wear_base_chance", hundredths(5)),
                ("wear_chance_per_100_nm", hundredths(2)),
                ("distance_nm", n(sortie.distance_nm)),
                ("chance", chance),
                ("write_off_share", tenths(2)),
            ],
            expected,
        );
        if worn > written_off {
            st.damaging_wear[i] = Some(e.id);
        }
    }

    // Fatigue: a group that flew gains for its longest sortie, once; any
    // other rests.
    if cfg.fatigue {
        for g in GROUPS {
            let i = ix(g);
            let before = st.fatigue[i];
            let flown: Vec<&Sortie> = sorties.iter().filter(|s| s.g == g && s.sent > 0).collect();
            let (change, after, causes, factors) = if flown.is_empty() {
                let after = (before - n(5)).max(Fixed::ZERO);
                let causes: Vec<Cause> = if st.stood[i] {
                    st.stood_event[i].map(Cause::Event).into_iter().collect()
                } else {
                    vec![Cause::Order(run.mission_order(g))]
                };
                (
                    FatigueChange::Rested,
                    after,
                    causes,
                    vec![("fatigue_before", before), ("air_rest_per_pulse", n(5))],
                )
            } else {
                // The longest sortie; the first of equal ones.
                let mut longest = flown[0];
                for sortie in &flown {
                    if sortie.distance_nm > longest.distance_nm {
                        longest = sortie;
                    }
                }
                let gain = fatigue_gain(longest.distance_nm);
                let after = (before + gain).min(n(100));
                let mut causes: Vec<Cause> = Vec::new();
                for sortie in &flown {
                    for cause in &sortie.causes {
                        if !causes.contains(cause) {
                            causes.push(*cause);
                        }
                    }
                }
                (
                    FatigueChange::Gained {
                        sortie: longest.kind,
                        distance_nm: n(longest.distance_nm),
                    },
                    after,
                    causes,
                    vec![
                        ("fatigue_before", before),
                        ("air_fatigue_per_sortie", n(3)),
                        ("air_fatigue_per_100_nm", n(2)),
                        ("distance_nm", n(longest.distance_nm)),
                        ("gain", gain),
                    ],
                )
            };
            if after == before {
                continue;
            }
            let amount = if after > before {
                after - before
            } else {
                before - after
            };
            st.fatigue[i] = after;
            let e = cur.next();
            cur.check(
                e,
                EventKind::AirGroupFatigue {
                    air_group: group(g),
                    change,
                    amount,
                    fatigue: after,
                },
                causes,
            );
            cur.text(
                e,
                match change {
                    FatigueChange::Gained {
                        sortie,
                        distance_nm,
                    } => format!(
                        "air_group:{g} gained {amount} fatigue from its {} sortie of {distance_nm} nm, now {after}",
                        sortie.name()
                    ),
                    FatigueChange::Rested => {
                        format!("air_group:{g} rested {amount} fatigue, now {after}")
                    }
                },
            );
            cur.breakdown(e, &factors, after);
        }
    }

    // Supply: each base pays once for the sorties flown from it, on the
    // aircraft sent.
    for bi in 0..2 {
        let flown: Vec<&Sortie> = sorties
            .iter()
            .filter(|s| s.sent > 0 && base_ix(s.g) == bi)
            .collect();
        let asked: i64 = flown
            .iter()
            .map(|s| i64::from(s.sent) * sortie_cost(s.distance_nm))
            .sum();
        if asked == 0 {
            continue;
        }
        let aircraft: u32 = flown.iter().map(|s| s.sent).sum();
        let mut causes: Vec<Cause> = Vec::new();
        for sortie in &flown {
            for cause in &sortie.causes {
                if !causes.contains(cause) {
                    causes.push(*cause);
                }
            }
        }
        let mut kinds: BTreeMap<SortieKind, (u32, i64)> = BTreeMap::new();
        for sortie in &flown {
            let entry = kinds.entry(sortie.kind).or_insert((0, 0));
            entry.0 += sortie.sent;
            entry.1 += i64::from(sortie.sent) * sortie_cost(sortie.distance_nm);
        }
        let mut factors: Vec<(&'static str, Fixed)> = Vec::new();
        for (kind, (sent, cost)) in &kinds {
            let (sent_name, cost_name) = match kind {
                SortieKind::BaseStrike => ("base_strike_aircraft_sent", "base_strike_t"),
                SortieKind::Patrol => ("patrol_aircraft_sent", "patrol_t"),
                SortieKind::Escort => ("escort_aircraft_sent", "escort_t"),
                other => panic!("a sortie the scenario does not fly: {other:?}"),
            };
            factors.push((sent_name, n(i64::from(*sent))));
            factors.push((cost_name, fx(*cost)));
        }
        let payment = pay(
            &mut st.stock[bi],
            base_id(bi),
            SupplyUse::Sorties,
            aircraft,
            asked,
        );
        let e = cur.next();
        assert_payment(cur, e, &payment, causes, &factors);
    }
    // Nothing else is in the log of the pulse.
    cur.finish();
}

// ---------------------------------------------------------------------
// What the events add up to, and what cannot happen, asserted for every run.
// ---------------------------------------------------------------------

/// The day a group's first aircraft arrive, if the scenario lists arrivals.
fn first_arrival(g: u32, cfg: &Cfg) -> Option<u32> {
    cfg.arrivals
        .then(|| ARRIVALS.iter().find(|(_, id, _)| *id == g).map(|a| a.0))
        .flatten()
}

/// The aircraft, the pilots, the strength and the supply are conserved in one
/// run, from its events and its final state alone: no rule in it.
fn assert_conserved(run: &Run) {
    let seed = run.seed;
    for g in GROUPS {
        let i = ix(g);
        let (mut shot, mut ground, mut written_off) = (0u32, 0u32, 0u32);
        let (mut arrived, mut received) = (0u32, 0u32);
        let (mut pilots_arrived, mut pilots_received, mut pilots_lost) = (0u32, 0u32, 0u32);
        for event in &run.events {
            match event.kind {
                EventKind::AircraftLost {
                    air_group,
                    aircraft,
                    ..
                } if air_group == group(g) => shot += aircraft,
                EventKind::AircraftDestroyedOnGround {
                    air_group,
                    aircraft,
                    ..
                }
                | EventKind::DamagedAircraftDestroyedOnGround {
                    air_group,
                    aircraft,
                    ..
                } if air_group == group(g) => ground += aircraft,
                EventKind::AircraftWorn {
                    air_group,
                    written_off: w,
                    ..
                } if air_group == group(g) => written_off += w,
                EventKind::AircraftArrived {
                    air_group,
                    aircraft,
                    pilots,
                    ..
                } if air_group == group(g) => {
                    arrived += aircraft;
                    pilots_arrived += pilots;
                }
                EventKind::ReplacementsReceived {
                    air_group,
                    aircraft,
                    pilots,
                    ..
                } if air_group == group(g) => {
                    received += aircraft;
                    pilots_received += pilots;
                }
                EventKind::PilotsLost {
                    air_group, lost, ..
                } if air_group == group(g) => pilots_lost += lost,
                _ => {}
            }
        }
        assert_eq!(
            START[i] + arrived + received,
            run.ready[i] + run.damaged[i] + shot + ground + written_off,
            "seed {seed}, group {g}: the aircraft it started with, those that arrived and those it received are those it has and those lost"
        );
        assert_eq!(
            START[i] + pilots_arrived + pilots_received - pilots_lost,
            run.pilots[i],
            "seed {seed}, group {g}: the pilots"
        );
        assert_eq!(
            START[i] + arrived,
            run.strength[i],
            "seed {seed}, group {g}: the strength is what it started with and what arrived"
        );
    }
    for bi in 0..2 {
        let base = base_id(bi);
        let (mut paid, mut destroyed) = (Fixed::ZERO, Fixed::ZERO);
        for event in &run.events {
            match event.kind {
                EventKind::BaseSupplySpent {
                    base: b, paid_t, ..
                } if b == base => paid += paid_t,
                EventKind::BaseSupplyDestroyed {
                    base: b,
                    destroyed_t,
                    ..
                } if b == base => destroyed += destroyed_t,
                _ => {}
            }
        }
        assert_eq!(
            fx(START_STOCK[bi]) - paid - destroyed,
            run.supply[bi],
            "seed {seed}, base {}: the supply at the end is the supply at the start less what was paid and what bombs destroyed",
            bi + 1
        );
        assert!(run.supply[bi] >= Fixed::ZERO, "seed {seed}");
    }
}

/// What cannot happen, in the events of one run.
fn assert_what_cannot_happen(run: &Run, worked: &Worked, cfg: &Cfg) {
    let seed = run.seed;
    assert_eq!(run.contacts, 0, "seed {seed}: no contact is ever held");
    for event in &run.events {
        let day = event.day;
        match &event.kind {
            EventKind::BaseStrikeLaunched { air_group, .. } => {
                assert_eq!(*air_group, group(BOMBERS), "seed {seed}");
                assert_eq!(
                    event.pulse,
                    Some(Pulse::Morning),
                    "seed {seed}: no raid in an afternoon or at night"
                );
                assert!(
                    !worked.day(day).stood[ix(BOMBERS)],
                    "seed {seed}, day {day}: a raid on a day group 1 is stood down"
                );
            }
            EventKind::EscortsSent { air_group, .. } => {
                assert!(
                    japanese(air_group.raw()),
                    "seed {seed}: no Allied aircraft flies to Rabaul"
                );
            }
            EventKind::BaseAttacked { base, .. }
            | EventKind::BaseAntiAircraftFire { base, .. }
            | EventKind::BaseSupplyDestroyed { base, .. } => {
                assert_eq!(*base, HENDERSON, "seed {seed}: nothing attacks Rabaul");
            }
            EventKind::AircraftDestroyedOnGround {
                air_group, base, ..
            }
            | EventKind::DamagedAircraftDestroyedOnGround {
                air_group, base, ..
            } => {
                assert!(
                    !japanese(air_group.raw()) && *base == HENDERSON,
                    "seed {seed}: no Japanese aircraft is destroyed on the ground"
                );
            }
            EventKind::PilotsLost { air_group, .. } => {
                assert!(
                    japanese(air_group.raw()),
                    "seed {seed}: no Allied pilot is lost"
                );
            }
            EventKind::AircraftWorn { air_group, .. }
            | EventKind::AirGroupFatigue { air_group, .. }
            | EventKind::AircraftLost { air_group, .. } => {
                assert!(
                    ![11, 12, 14, 17].contains(&air_group.raw()),
                    "seed {seed}: no dive bomber flies, is worn or gains fatigue"
                );
            }
            EventKind::SearchFlown { .. }
            | EventKind::SearchAttempt { .. }
            | EventKind::ReportReceived { .. }
            | EventKind::ContactNotThere { .. }
            | EventKind::StrikeLaunched { .. }
            | EventKind::StrikeFoundNothing { .. }
            | EventKind::AirAttack { .. }
            | EventKind::AntiAircraftFire { .. } => {
                panic!(
                    "seed {seed}: no contact is held and no report is made, {}",
                    event.kind
                )
            }
            _ => {}
        }
        // No group has aircraft before its arrival day: nothing but its
        // orders mentions it.
        if !matches!(event.kind, EventKind::OrderAccepted { .. }) {
            for entity in event.kind.entities() {
                if let EntityRef::AirGroup(id) = entity {
                    let g = id.raw();
                    if START[ix(g)] == 0 {
                        assert!(
                            first_arrival(g, cfg).is_some_and(|d| day >= d),
                            "seed {seed}, day {day}: group {g} has no aircraft yet: {}",
                            event.kind
                        );
                    }
                }
            }
        }
    }
    for g in GROUPS.into_iter().filter(|g| START[ix(*g)] == 0) {
        let first = first_arrival(g, cfg).unwrap_or(DAYS + 1);
        for day in 1..first.min(DAYS + 1) {
            let d = worked.day(day);
            assert!(
                d.ready_start[ix(g)] == 0
                    && d.ready_end[ix(g)] == 0
                    && d.damaged_end[ix(g)] == 0
                    && d.pilots_end[ix(g)] == 0,
                "seed {seed}, day {day}: group {g} has aircraft before its arrival day"
            );
        }
    }
}

/// Runs a seed, works every pulse of it by the rule, and asserts what the
/// events add up to and what cannot happen.
fn analyse(base: &Scenario, seed: u64, cfg: &Cfg) -> Worked {
    let run = run_of(base, seed);
    let worked = work_the_run(&run, cfg);
    assert_conserved(&run);
    assert_what_cannot_happen(&run, &worked, cfg);
    worked
}

/// Analyses seeds 1 to `seeds` of the scenario file at `path`, on as many
/// threads as the machine has (each seed is its own run, so the result does
/// not depend on it).
fn analyse_batch(source: &Source, seeds: u64, cfg: Cfg) -> Vec<Worked> {
    let all: Vec<u64> = (1..=seeds).collect();
    let threads = std::thread::available_parallelism()
        .map_or(1, usize::from)
        .min(8);
    let size = all.len().div_ceil(threads);
    std::thread::scope(|scope| {
        let handles: Vec<_> = all
            .chunks(size)
            .map(|part| {
                scope.spawn(move || {
                    let base = common::from_text_with(&source.text, &source.path, &source.options)
                        .expect("the scenario loads");
                    part.iter()
                        .map(|seed| analyse(&base, *seed, &cfg))
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|handle| handle.join().expect("a worker finishes"))
            .collect()
    })
}

/// Every pulse of every run of the scenario, worked and asserted, once.
fn batch() -> &'static Vec<Worked> {
    static BATCH: OnceLock<Vec<Worked>> = OnceLock::new();
    BATCH.get_or_init(|| analyse_batch(&Source::plain(), SEEDS, scenario_cfg()))
}

// ---------------------------------------------------------------------
// Variants: the scenario with one thing changed, in its text or in a copy of
// the frozen figures under the build directory. Nothing in the repository's
// data folder is touched.
// ---------------------------------------------------------------------

/// A scenario's text, where it would sit and how its reference files are
/// read: what a worker thread needs to build its own copy.
struct Source {
    text: String,
    path: PathBuf,
    options: sim_data::LoadOptions,
}

impl Source {
    /// The scenario as it is, on the frozen figures.
    fn plain() -> Source {
        Source {
            text: fs::read_to_string(scenario_path()).expect("read the scenario"),
            path: scenario_path(),
            options: common::frozen(),
        }
    }
}

/// Replaces `original` by `changed`, which must appear exactly once.
fn once(text: String, original: &str, changed: &str) -> String {
    common::replace_once(&text, original, changed)
}

/// The text without the blocks that begin with `header`, each up to the blank
/// line that ends it.
fn without_blocks(text: &str, header: &str) -> String {
    let mut out = String::new();
    let mut skipping = false;
    for line in text.lines() {
        if line == header {
            skipping = true;
            continue;
        }
        if skipping {
            if line.trim().is_empty() {
                skipping = false;
            }
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

const TYPES_FILE: &str = "aircraft_types_rabaul_guadalcanal.csv";

/// The scenario with `edit` applied to its text and, if given, `types`
/// applied to a copy of the aircraft types the scenario names (in a copy of
/// the frozen reference folder).
fn variant(
    tag: &str,
    edit: &dyn Fn(String) -> String,
    types: Option<&dyn Fn(String) -> String>,
) -> Source {
    let mut source = Source::plain();
    source.text = edit(source.text);
    if let Some(types) = types {
        source.options = common::altered_reference(tag, |dir| {
            common::edit_file(dir, TYPES_FILE, types);
        });
    }
    source
}

/// The scenario's own text, unchanged.
fn same(text: String) -> String {
    text
}

fn radius_of_the_a6m2(nm: i64) -> Source {
    variant(
        &format!("a6m2_{nm}"),
        &same,
        Some(&|text| {
            once(
                text,
                ",fighter,556,20,556,none,",
                &format!(",fighter,{nm},20,{nm},none,"),
            )
        }),
    )
}

fn radius_of_the_g4m1(nm: i64) -> Source {
    variant(
        &format!("g4m1_{nm}"),
        &same,
        Some(&|text| {
            once(
                text,
                ",bomber,611,20,611,bomb,",
                &format!(",bomber,{nm},20,{nm},bomb,"),
            )
        }),
    )
}

fn without_the_fatigue_file() -> Source {
    variant(
        "no_fatigue",
        &|text| once(text, "fatigue = \"../../reference/fatigue.toml\"\n", ""),
        None,
    )
}

fn without_arrivals() -> Source {
    variant(
        "no_arrivals",
        &|text| without_blocks(&text, "[[arrival]]"),
        None,
    )
}

fn without_replacements() -> Source {
    variant(
        "no_replacements",
        &|text| without_blocks(&text, "[[replacement]]"),
        None,
    )
}

fn fix(text: &str) -> Fixed {
    text.parse().expect("a decimal quantity")
}

// ---------------------------------------------------------------------
// The tests.
// ---------------------------------------------------------------------

#[test]
fn the_setting_is_as_the_hand_calculation_says() {
    let scenario = load();
    let truth = scenario.world.umpire();
    assert_eq!(truth.bases.len(), 2);
    let henderson = &truth.bases[&HENDERSON];
    let rabaul = &truth.bases[&RABAUL];
    assert_eq!(henderson.side, Side::Allied);
    assert_eq!(henderson.hex, Hex::new(2, -4));
    assert_eq!(henderson.supply_t, n(4000));
    assert_eq!(henderson.anti_aircraft, 8);
    assert_eq!(henderson.aircraft_repairs_per_day, 3);
    assert_eq!(rabaul.side, Side::Japanese);
    assert_eq!(rabaul.hex, Hex::new(-5, -13));
    assert_eq!(rabaul.supply_t, n(10_000));
    assert_eq!(rabaul.anti_aircraft, 0);
    assert_eq!(rabaul.aircraft_repairs_per_day, 4);

    // The air distance: 40 x root of (7 x 7 + 9 x 9 + 7 x 9) = 40 x root 193 =
    // 555.7, so 556 nm, and a sortie there and back is 1,112.
    assert_eq!(7 * 7 + 9 * 9 + 7 * 9, 193);
    assert_eq!((40.0 * 193f64.sqrt()).round(), 556.0);
    assert_eq!(henderson.hex.air_distance_nm(rabaul.hex), 556);
    assert_eq!(rabaul.hex.air_distance_nm(henderson.hex), 556);
    assert_eq!(2 * 556, SORTIE_NM);

    // The type table, from the loaded reference data.
    let data = scenario.world.ref_data();
    let types = [
        (2100, 105, AttackWeapon::None, true, true),
        (2101, 200, AttackWeapon::Bomb, true, true),
        (2102, 556, AttackWeapon::None, false, false),
        (2103, 611, AttackWeapon::Bomb, false, false),
    ];
    for (id, radius, weapon, armour, tanks) in types {
        let aircraft = &data.aircraft_types[&AircraftTypeId::new(id)];
        assert_eq!(aircraft.strike_range_nm, n(radius), "type {id}");
        assert_eq!(aircraft.attack_weapon, weapon, "type {id}");
        assert_eq!(aircraft.pilot_armour, armour, "type {id}");
        assert_eq!(aircraft.self_sealing_tanks, tanks, "type {id}");
    }
    // The G4M1s and the A6M2s can fly from Rabaul to Henderson Field, the
    // A6M2s with nothing to spare, and nothing at Henderson Field can reach
    // Rabaul.
    let distance = n(556);
    let radius = |id: u32| data.aircraft_types[&AircraftTypeId::new(id)].strike_range_nm;
    assert!(radius(2103) >= distance && radius(2102) >= distance);
    assert_eq!(radius(2102), distance);
    assert!(radius(2100) < distance && radius(2101) < distance);

    // The groups. Their missions, shares and rest thresholds are the day-1
    // orders of the scenario, so those are read from the state at the end of a
    // run, where they still stand.
    let mut finished = load().with_seed(1942).expect("a seed can be set");
    finished.run_to_end().expect("the run resolves");
    let after = finished.world.umpire();
    for g in GROUPS {
        let i = ix(g);
        let air_group = &truth.air_groups[&group(g)];
        let ordered = &after.air_groups[&group(g)];
        assert_eq!(
            air_group.side,
            if japanese(g) {
                Side::Japanese
            } else {
                Side::Allied
            },
            "group {g}"
        );
        assert_eq!(
            air_group.location,
            AirGroupLocation::Base(base_id(base_ix(g))),
            "group {g}"
        );
        assert_eq!(air_group.aircraft_type, AircraftTypeId::new(type_of(g)));
        assert_eq!(air_group.ready, START[i], "group {g}");
        assert_eq!(air_group.damaged, 0);
        assert_eq!(
            air_group.pilots,
            Some(START[i]),
            "a pilot for each aircraft"
        );
        assert_eq!(
            air_group.strength, START[i],
            "a group with none has a strength of none"
        );
        assert_eq!(air_group.fatigue, Fixed::ZERO);
        assert!(!air_group.stood_down);
        assert_eq!(ordered.rest_above, threshold(g).map(n), "group {g}");
        let (mission, share) = match g {
            1 => (AirMission::BaseStrike(HENDERSON), Fixed::ONE),
            2 | 3 => (AirMission::CombatAirPatrol, Fixed::ZERO),
            10 | 13 | 15 | 16 => (AirMission::CombatAirPatrol, Fixed::ONE),
            _ => (AirMission::NavalStrike, Fixed::ONE),
        };
        assert_eq!(ordered.mission, mission, "group {g}");
        assert_eq!(ordered.patrol_share, share, "group {g}");
    }
    assert_eq!(GROUPS.len(), 11);

    // Arrivals: day, group, aircraft and pilots.
    assert_eq!(truth.arrivals.len(), ARRIVALS.len());
    for (arrival, (day, g, aircraft)) in truth.arrivals.iter().zip(ARRIVALS) {
        assert_eq!(
            (
                arrival.day,
                arrival.air_group,
                arrival.aircraft,
                arrival.pilots
            ),
            (day, group(g), aircraft, aircraft)
        );
    }
    // Rabaul receives two G4M1 and two A6M2 a day, each with a pilot, and
    // Henderson Field nothing.
    assert_eq!(truth.replacements.len(), 2);
    for (stream, id) in truth.replacements.iter().zip([2102, 2103]) {
        assert_eq!(stream.side, Side::Japanese);
        assert_eq!(stream.base, RABAUL);
        assert_eq!(stream.aircraft_type, AircraftTypeId::new(id));
        assert_eq!((stream.aircraft_per_day, stream.pilots_per_day), (2, 2));
    }
    // All the figures are loaded.
    assert!(data.air_upkeep.is_some());
    assert!(data.fatigue.is_some());
    assert!(data.air_supply.is_some());
    assert!(data.air_combat.is_some());
    assert_eq!(scenario.days, DAYS);
}

#[test]
fn the_figures_for_one_sortie_and_fly_are_as_the_hand_calculation_says() {
    let scenario = load();
    let data = scenario.world.ref_data();
    let upkeep = data.air_upkeep.expect("wear figures");
    let fatigue = data.fatigue.expect("fatigue figures");
    let supply = data.air_supply.expect("supply figures");
    let air = data.air_combat.clone().expect("air combat figures");

    // A patrol, and Rabaul to Henderson Field and back, 1,112 nm.
    for nm in [0, SORTIE_NM] {
        assert_eq!(upkeep.wear_chance(n(nm)), wear_chance(nm));
        assert_eq!(fatigue.gain(n(nm)), fatigue_gain(nm));
        assert_eq!(supply.sortie_cost(n(nm)), fx(sortie_cost(nm)));
    }
    assert_eq!(wear_chance(0), hundredths(5));
    assert_eq!(wear_chance(SORTIE_NM), fix("0.2724"));
    assert_eq!(fatigue_gain(0), n(3));
    assert_eq!(fatigue_gain(SORTIE_NM), fix("25.24"));
    assert_eq!(fx(sortie_cost(0)), fix("0.5"));
    assert_eq!(fx(sortie_cost(SORTIE_NM)), fix("3.28"));
    // A worn aircraft is written off with chance 0.2. A group that did not fly
    // in a daylight pulse loses 5 fatigue. An aircraft at a base costs 0.2 t a
    // day and a repair 1 t.
    assert_eq!(upkeep.write_off_share, tenths(2));
    assert_eq!(fatigue.air_rest_per_pulse, n(5));
    assert_eq!(supply.daily_supply_t_per_aircraft, fix("0.2"));
    assert_eq!(supply.repair_supply_t, n(1));
    // A fighter's shot hits an A6M2 or a G4M1 with chance 0.3, and an F4F-4
    // with 0.3 x 0.75 x 0.75 = 0.16875. A gun's shot hits a G4M1 with 0.05. A
    // bomb hits with 0.2, and lands on aircraft on the ground with 0.3, on
    // supply with 0.3, destroying 50 t, and on nothing with 0.4.
    assert_eq!(air.fighter_hit_chance, tenths(3));
    assert_eq!(air.pilot_armour_factor, hundredths(75));
    assert_eq!(air.self_sealing_tanks_factor, hundredths(75));
    assert_eq!(shot_chance(air.fighter_hit_chance, 10), fix("0.16875"));
    assert_eq!(shot_chance(air.fighter_hit_chance, 2), tenths(3));
    assert_eq!(shot_chance(air.fighter_hit_chance, 1), tenths(3));
    assert_eq!(air.anti_aircraft_hit_chance, hundredths(5));
    assert_eq!(air.bomb_hit_chance, tenths(2));
    assert_eq!(air.base_hit_aircraft, tenths(3));
    assert_eq!(air.base_hit_supply, tenths(3));
    assert_eq!(
        n(1) - air.base_hit_aircraft - air.base_hit_supply,
        tenths(4)
    );
    assert_eq!(air.base_supply_destroyed_per_hit_t, n(50));

    // fly(group): the smaller of R and P, times (100 - f) / 100, rounded down.
    assert_eq!(fly(41, 41, Fixed::ZERO), 41);
    assert_eq!(fly(38, 41, fix("20.24")), 30);
    assert_eq!(fly(41, 38, fix("20.24")), 30);
    assert_eq!(fly(37, 41, fix("40.48")), 22);
    assert_eq!(fly(19, 19, n(3)), 18);
    assert_eq!(fly(0, 5, Fixed::ZERO), 0);
    assert_eq!(fly(5, 5, n(100)), 0);
    assert_eq!(fix("100") - fix("20.24"), fix("79.76"));
}

#[test]
fn every_pulse_of_every_run_is_worked_by_the_rule_from_the_state_at_the_start_of_it() {
    let runs = batch();
    assert_eq!(runs.len(), 1000);
    for (i, worked) in runs.iter().enumerate() {
        assert_eq!(worked.seed, i as u64 + 1);
        assert_eq!(worked.days.len(), DAYS as usize, "seed {}", worked.seed);
        assert_eq!(worked.hashes.len(), DAYS as usize);
    }
}

#[test]
fn the_first_day_is_as_the_hand_calculation_says() {
    // The expected figures and standard deviations of the table.
    let table = [
        (24u32, "0.16875", "4.05", "1.834819"),
        (19, "0.3", "5.7", "1.997498"),
        (8, "0.05", "0.4", "0.616441"),
    ];
    for (shots, chance, expected, sd) in table {
        let chance = fix(chance);
        assert_eq!(n(i64::from(shots)) * chance, fix(expected));
        let p = chance.raw() as f64 / 1e6;
        let sigma = (f64::from(shots) * p * (1.0 - p)).sqrt();
        assert_eq!(format!("{sigma:.6}"), sd);
    }
    // The bombers that attack are 41 less the guns' hits, 40.6 on average, and
    // score 0.2 of their number in hits, 8.12 on average.
    assert_eq!(n(41) - fix("0.4"), fix("40.6"));
    assert_eq!(fix("40.6") * tenths(2), fix("8.12"));
    // Night: Rabaul pays (41 + 24) x 0.2 = 13 t and Henderson Field (19 + 12)
    // x 0.2 = 6.2 t.
    assert_eq!(65 * 20, 1300);
    assert_eq!(31 * 20, 620);
    assert_eq!(t(1300), "13");
    assert_eq!(t(620), "6.2");
    assert_eq!(t(998_700), "9987");
    assert_eq!(t(399_380), "3993.8");
    // Rabaul pays 65 x 3.28 = 213.2 t.
    assert_eq!(65 * sortie_cost(SORTIE_NM), 21_320);
    assert_eq!(t(998_700 - 21_320), "9773.8");

    let hit_roll = |d: &Day, kind: RollKind| d.rolls.iter().find(|r| r.kind == kind).copied();
    let mut first_morning = [(0i64, 0i64); 4];
    for w in batch() {
        let seed = w.seed;
        let d = w.day(1);
        // The first raid always flies.
        assert!(d.launched && d.bombers == 41, "seed {seed}");
        // Night: nothing is damaged, nothing arrives, and nobody is stood
        // down. Rabaul has 9,987 t and Henderson Field 3,993.8 t.
        assert_eq!(d.stock_night, [399_380, 998_700], "seed {seed}");
        assert!(d.stood.iter().all(|s| !s), "seed {seed}");
        assert!(d.repaired.iter().all(|r| *r == 0));
        assert!(d.arrived.iter().all(|a| *a == 0));
        assert!(d.received_aircraft.iter().all(|a| *a == 0));
        // Morning: 41 bombers launch with 24 escorts, and VMF-223's patrol is
        // 19. The aircraft on the ground are VMSB-232's twelve.
        assert_eq!(d.escorts_by[ix(TAINAN)], 24, "seed {seed}");
        assert_eq!(d.escorts_by[ix(SIXTH)], 0);
        assert_eq!(d.patrol_by[ix(VMF_223)], 19, "seed {seed}");
        for g in GROUPS.into_iter().filter(|g| !japanese(*g)) {
            assert_eq!(
                d.ground_ready[ix(g)],
                if g == 11 { 12 } else { 0 },
                "seed {seed}, group {g}"
            );
            assert_eq!(d.ground_damaged[ix(g)], 0);
        }
        let escorts = hit_roll(d, RollKind::EscortsAtPatrol).expect("escorts fire");
        let patrol = hit_roll(d, RollKind::PatrolAtEscorts).expect("the patrol fires");
        let guns = hit_roll(d, RollKind::Guns).expect("the guns fire");
        let bombs = hit_roll(d, RollKind::Bombs).expect("the bombers attack");
        assert_eq!((escorts.shots, escorts.chance), (24, fix("0.16875")));
        assert_eq!((patrol.shots, patrol.chance), (19, tenths(3)));
        assert_eq!((guns.shots, guns.chance), (8, hundredths(5)));
        // The patrol at the bombers: none, the 24 escorts hold all 19.
        assert!(
            hit_roll(d, RollKind::PatrolAtBombers).is_none(),
            "seed {seed}"
        );
        assert_eq!(d.bombers_by_fighters, 0);
        assert_eq!(d.attacking, 41 - guns.hits, "seed {seed}");
        assert_eq!((bombs.shots, bombs.chance), (d.attacking, tenths(2)));
        // Then, with e escorts and g bombers shot down: group 1 rolls 41 - g
        // aircraft at 0.2724 and group 2 rolls 24 - e at 0.2724; VMF-223 rolls
        // its survivors at 0.05.
        let morning: Vec<&WearRec> = d.wear.iter().take(3).collect();
        assert_eq!(morning.len(), 3);
        assert_eq!(
            [morning[0].sortie, morning[1].sortie, morning[2].sortie],
            [
                SortieKind::BaseStrike,
                SortieKind::Escort,
                SortieKind::Patrol
            ]
        );
        assert!(morning.iter().all(|r| r.written_off <= r.worn));
        assert_eq!(
            (morning[0].group, morning[0].aircraft, morning[0].chance),
            (1, 41 - guns.hits, fix("0.2724")),
            "seed {seed}"
        );
        assert_eq!(
            (morning[1].group, morning[1].aircraft, morning[1].chance),
            (2, 24 - patrol.hits, fix("0.2724")),
            "seed {seed}"
        );
        assert_eq!(
            (morning[2].group, morning[2].aircraft, morning[2].chance),
            (10, 19 - escorts.hits, hundredths(5)),
            "seed {seed}"
        );
        // Groups 1 and 2 are at a fatigue of 25.24, and VMF-223 at 3.
        assert_eq!(d.fatigue_morning[ix(1)], fix("25.24"));
        assert_eq!(d.fatigue_morning[ix(2)], fix("25.24"));
        assert_eq!(d.fatigue_morning[ix(10)], n(3));
        // Rabaul pays 213.2 t and has 9,773.8 t. Henderson Field pays 19 x 0.5
        // = 9.5 t, after 50 t for each bomb on its supply.
        assert_eq!(d.stock_morning[1], 977_380, "seed {seed}");
        assert_eq!(
            d.stock_morning[0],
            399_380 - 5000 * i64::from(d.hits_on_supply) - 950,
            "seed {seed}"
        );
        // Afternoon: groups 1 and 2 rest to 20.24. VMF-223 patrols with what it
        // can fly at a fatigue of 3 and ends the day at 6.
        assert_eq!(d.fatigue_end[ix(1)], fix("20.24"));
        assert_eq!(d.fatigue_end[ix(2)], fix("20.24"));
        assert_eq!(
            d.able_afternoon[ix(10)],
            fly(19 - escorts.hits - morning[2].worn, 19, n(3)),
            "seed {seed}"
        );
        assert!(d.able_afternoon[ix(10)] >= 1);
        assert_eq!(d.fatigue_end[ix(10)], n(6));
        for (slot, roll) in first_morning.iter_mut().zip([escorts, patrol, guns, bombs]) {
            slot.0 += i64::from(roll.hits);
            slot.1 += i64::from(roll.shots);
        }
    }
    // Over seeds 1 to 1000 the hits of each of the three rolls, summed over
    // the runs, are within four standard deviations of the expected figure
    // times 1,000. So are the bomb hits, against 0.2 of the bombers that
    // attacked.
    for ((hits, shots), p) in first_morning
        .into_iter()
        .zip([168_750, 300_000, 50_000, 200_000])
    {
        assert!(
            within_four_sigma(hits, shots, p),
            "{hits} hits in {shots} shots at {p}"
        );
    }
    assert_eq!(first_morning[0].1, 24_000);
    assert_eq!(first_morning[1].1, 19_000);
    assert_eq!(first_morning[2].1, 8_000);
}

/// The staff's decision on a day, as the table words it.
fn staff(w: &Worked, day: u32, g: u32) -> &'static str {
    let stood = w.day(day).stood[ix(g)];
    let before = day > 1 && w.day(day - 1).stood[ix(g)];
    match (stood, before) {
        (true, _) => "stood down",
        (false, true) => "returned, flies",
        (false, false) => "flies",
    }
}

fn launched_on(w: &Worked, days: &[u32]) -> bool {
    days.iter().all(|day| w.day(*day).launched)
}

/// The fatigue tables, group by group, for the runs that meet the stated
/// condition. Returns the number of runs of each.
fn the_fatigue_tables() -> [usize; 4] {
    // Group 1, threshold 45. A day flown is +25.24 and -5. A day stood down is
    // -5 and -5.
    let group_1 = [
        (1, "0", "flies", "25.24", "20.24"),
        (2, "20.24", "flies", "45.48", "40.48"),
        (3, "40.48", "flies", "65.72", "60.72"),
        (4, "60.72", "stood down", "55.72", "50.72"),
        (5, "50.72", "stood down", "45.72", "40.72"),
        (6, "40.72", "returned, flies", "65.96", "60.96"),
        (7, "60.96", "stood down", "55.96", "50.96"),
        (8, "50.96", "stood down", "45.96", "40.96"),
        (9, "40.96", "returned, flies", "66.2", "61.2"),
    ];
    let first_nine = [1, 2, 3, 6, 9];
    let (mut met_table, mut met_eleven, mut met_escorts, mut met_sixth) = (0, 0, 0, 0);
    for w in batch() {
        let seed = w.seed;
        // The table holds in every run in which group 1 flies on the days it
        // is not stood down, as far as it goes.
        if launched_on(w, &first_nine) {
            met_table += 1;
            for (day, start, action, morning, end) in group_1 {
                let d = w.day(day);
                assert_eq!(d.fatigue_start[ix(1)], fix(start), "seed {seed}, day {day}");
                assert_eq!(staff(w, day, 1), action, "seed {seed}, day {day}");
                assert_eq!(
                    d.fatigue_morning[ix(1)],
                    fix(morning),
                    "seed {seed}, day {day}"
                );
                assert_eq!(d.fatigue_end[ix(1)], fix(end), "seed {seed}, day {day}");
            }
        }
        // From day 6 it flies every third day, starting each raid day 0.24
        // higher than the last: raids on days 1, 2, 3, 6, 9, ... 27.
        if launched_on(w, &RAID_DAYS) {
            met_eleven += 1;
            assert_eq!(w.raid_days(), RAID_DAYS, "seed {seed}: eleven raids");
            for k in 0..8u32 {
                let day = 6 + 3 * k;
                assert_eq!(
                    w.day(day).fatigue_start[ix(1)],
                    fix("40.72") + hundredths(24) * i64::from(k),
                    "seed {seed}, day {day}"
                );
            }
            // On day 1 it flies every bomber it has pilots for; on day 2, at
            // 20.24, 0.7976 of them; from day 3, at about 41, about 0.59 of
            // them.
            assert_eq!(w.day(1).able_morning[ix(1)], 41);
            assert_eq!(n(100) - w.day(2).fatigue_start[ix(1)], fix("79.76"));
            for day in RAID_DAYS.into_iter().filter(|d| *d >= 3) {
                let factor = n(100) - w.day(day).fatigue_start[ix(1)];
                assert!(
                    factor > n(57) && factor < n(60),
                    "seed {seed}, day {day}: factor {factor}"
                );
            }
        }
        // Group 2 has the same threshold, gains the same on a raid day and
        // flies on no other day, so its fatigue is group 1's and it escorts
        // every raid.
        for day in 1..=DAYS {
            let d = w.day(day);
            assert!(
                d.escorts_by[ix(TAINAN)] == 0 || d.launched,
                "seed {seed}, day {day}: group 2 flies on no day without a raid"
            );
        }
        let escorts_every_raid = w
            .raid_days()
            .iter()
            .all(|day| w.day(*day).escorts_by[ix(TAINAN)] >= 1);
        if escorts_every_raid {
            met_escorts += 1;
            for day in 1..=DAYS {
                let d = w.day(day);
                assert_eq!(
                    d.fatigue_start[ix(2)],
                    d.fatigue_start[ix(1)],
                    "seed {seed}, day {day}"
                );
                assert_eq!(d.fatigue_morning[ix(2)], d.fatigue_morning[ix(1)]);
                assert_eq!(d.fatigue_end[ix(2)], d.fatigue_end[ix(1)]);
                assert_eq!(d.stood[ix(2)], d.stood[ix(1)]);
            }
        }
        // Group 3 arrives on day 11 at a fatigue of 0 and escorts the raids
        // from day 12: it ends each raid day at 20.24 more than it began, less
        // 5, and rests 10 on each of the two days between, so it begins day 12
        // at 0, day 15 at 0.24, day 18 at 0.48, and is never stood down.
        let late: Vec<u32> = w.raid_days().into_iter().filter(|d| *d >= 11).collect();
        let sixth_days = [12, 15, 18, 21, 24, 27];
        if late == sixth_days
            && sixth_days
                .iter()
                .all(|day| w.day(*day).escorts_by[ix(SIXTH)] >= 1)
        {
            met_sixth += 1;
            for (k, day) in sixth_days.into_iter().enumerate() {
                let d = w.day(day);
                assert_eq!(
                    d.fatigue_start[ix(SIXTH)],
                    hundredths(24) * count(k),
                    "seed {seed}, day {day}"
                );
                assert_eq!(
                    d.fatigue_morning[ix(SIXTH)] - d.fatigue_start[ix(SIXTH)],
                    fix("25.24")
                );
                assert_eq!(
                    d.fatigue_end[ix(SIXTH)] - d.fatigue_start[ix(SIXTH)],
                    fix("20.24")
                );
            }
            for day in 1..=DAYS {
                assert!(
                    !w.day(day).stood[ix(SIXTH)],
                    "seed {seed}, day {day}: never stood down"
                );
            }
        }
    }
    [met_table, met_eleven, met_escorts, met_sixth]
}

#[test]
fn the_tables_of_the_bombers_and_the_escorts_hold_in_the_runs_that_meet_their_conditions() {
    let [table, eleven, escorts, sixth] = the_fatigue_tables();
    eprintln!(
        "runs in which group 1 flies on days 1, 2, 3, 6 and 9: {table}; on all eleven raid days: {eleven}; group 2 escorts every raid: {escorts}; group 3 escorts days 12 to 27 and the raids are those: {sixth}"
    );
    assert!(table > 0 && eleven > 0 && escorts > 0);
    assert!(eleven <= table);
}

#[test]
fn the_raid_days_and_the_fatigue_of_the_bombers_follow_from_the_rule_alone() {
    // The same arithmetic with no aircraft in it: a day flown is +25.24 and
    // -5, a day stood down -5 and -5, and a group above 45 is stood down.
    let mut fatigue = Fixed::ZERO;
    let mut raids = Vec::new();
    let mut starts = Vec::new();
    for day in 1..=DAYS {
        starts.push(fatigue);
        if fatigue > n(45) {
            fatigue = (fatigue - n(10)).max(Fixed::ZERO);
        } else {
            raids.push(day);
            fatigue = fatigue + fix("25.24") - n(5);
        }
    }
    assert_eq!(raids, RAID_DAYS, "the eleven raid days");
    assert_eq!(starts[0], Fixed::ZERO);
    assert_eq!(starts[1], fix("20.24"));
    assert_eq!(starts[2], fix("40.48"));
    assert_eq!(starts[5], fix("40.72"));
    assert_eq!(starts[26], fix("40.72") + hundredths(24) * 7);
    // Group 3 from day 12: 0, 0.24, 0.48 ... at the start of each raid day.
    let mut sixth = Fixed::ZERO;
    for (k, day) in (12..=DAYS).enumerate() {
        if RAID_DAYS.contains(&day) {
            assert_eq!(sixth, hundredths(24) * count(k / 3));
            sixth = sixth + fix("25.24") - n(5);
        } else {
            sixth = (sixth - n(10)).max(Fixed::ZERO);
        }
        assert!(sixth <= n(45));
    }
}

#[test]
fn the_allied_stood_down_days_are_those_of_the_table() {
    // A day stood down takes 10 off and a day flown puts 6 on: VMF-223 reaches
    // 42 at the end of day 7 and is stood down on day 8.
    let table: [(u32, u32, Vec<u32>); 4] = [
        (10, 1, vec![8, 11, 14, 16, 19, 22, 24, 27]),
        (13, 10, vec![17, 20, 23, 25, 28]),
        (15, 22, vec![]),
        (16, 24, vec![]),
    ];
    let mut fatigue = Fixed::ZERO;
    for _ in 1..=7 {
        fatigue += n(6);
    }
    assert_eq!(fatigue, n(42));
    assert!(fatigue > n(40));
    for (g, first, days) in &table {
        assert_eq!(
            &stood_days_if_it_always_flies(*first, 40),
            days,
            "group {g} by the rule alone"
        );
    }
    // By the recurrence its stood-down days include two raid days, 24 and
    // 27. No run shows it: the group has no aircraft by then.
    assert!(RAID_DAYS.contains(&24) && RAID_DAYS.contains(&27));
    assert!(table[0].2.contains(&24) && table[0].2.contains(&27));

    // In the runs in which a group flies in both daylight pulses of every day
    // it is not stood down, from its first day, its stood-down days are those.
    let mut met = [0usize; 4];
    let mut ground_checked = 0;
    for w in batch() {
        let seed = w.seed;
        for (k, (g, first, days)) in table.iter().enumerate() {
            let i = ix(*g);
            let flies = (*first..=DAYS).all(|day| {
                let d = w.day(day);
                d.stood[i] || (d.able_morning[i] >= 1 && d.able_afternoon[i] >= 1)
            });
            if flies {
                met[k] += 1;
                let stood: Vec<u32> = (1..=DAYS).filter(|d| w.day(*d).stood[i]).collect();
                assert_eq!(&stood, days, "seed {seed}, group {g}");
            }
        }
        // Whenever a fighter group is stood down on a raid day, all its ready
        // aircraft are on the ground, where bombs can find them.
        for day in w.raid_days() {
            let d = w.day(day);
            for g in ALLIED_FIGHTERS {
                let i = ix(g);
                if d.stood[i] {
                    ground_checked += 1;
                    assert_eq!(
                        d.ground_ready[i], d.ready_start[i],
                        "seed {seed}, day {day}, group {g}: a stood-down group is on the ground"
                    );
                }
            }
        }
    }
    eprintln!(
        "runs in which the Allied fighter groups 10, 13, 15 and 16 fly on every day they are not stood down: {met:?}; stood-down fighter groups on a raid day checked: {ground_checked}"
    );
    assert!(ground_checked > 0);
}

#[test]
fn over_the_four_weeks_aircraft_pilots_strength_and_supply_are_conserved() {
    // The event sums are asserted for every run as it is analysed; here the
    // rebuilt totals agree with the final state too.
    for w in batch() {
        let seed = w.seed;
        for g in GROUPS {
            let i = ix(g);
            assert_eq!(
                START[i] + w.arrived[i] + w.received_aircraft[i],
                w.ready[i] + w.damaged[i] + w.shot[i] + w.on_the_ground[i] + w.written_off[i],
                "seed {seed}, group {g}"
            );
            assert_eq!(
                START[i] + w.arrived[i] + w.received_pilots[i] - w.pilots_lost[i],
                w.pilots[i],
                "seed {seed}, group {g}: pilots"
            );
        }
        // Supply: what it started with, less what was paid by the day, for
        // sorties and for repairs (as the rebuild paid them), less what bombs
        // destroyed, is what Henderson Field and Rabaul have.
        assert_eq!(fx(w.day(DAYS).stock_end[0]), w.supply[0], "seed {seed}");
        assert!(w.supply_destroyed % 5000 == 0, "seed {seed}");
        assert!(w.supply[0] <= fx(START_STOCK[0]) && w.supply[1] <= fx(START_STOCK[1]));
    }
}

#[test]
fn what_cannot_happen_does_not_in_any_run() {
    for w in batch() {
        let seed = w.seed;
        // No Allied pilot is lost, and no Japanese aircraft is destroyed on the
        // ground.
        for g in GROUPS {
            let i = ix(g);
            if japanese(g) {
                assert_eq!(w.on_the_ground[i], 0, "seed {seed}, group {g}");
            } else {
                assert_eq!(w.pilots_lost[i], 0, "seed {seed}, group {g}");
                assert_eq!(
                    w.pilots[i],
                    START[i] + w.arrived[i],
                    "seed {seed}, group {g}: the Allies keep every pilot"
                );
            }
        }
        for day in 1..=DAYS {
            let d = w.day(day);
            // No raid flies on a day group 1 is stood down.
            assert!(!(d.launched && d.stood[ix(1)]), "seed {seed}, day {day}");
            // No dive bomber flies, is worn or gains fatigue.
            for g in [11, 12, 14, 17] {
                let i = ix(g);
                assert!(
                    d.fatigue_end[i] == Fixed::ZERO,
                    "seed {seed}, day {day}, group {g}"
                );
                assert!(d.patrol_by[i] == 0 && d.escorts_by[i] == 0 && d.shot[i] == 0);
                assert!(d.wear.iter().all(|r| r.group != g), "seed {seed}");
            }
            // The dive bombers are on the ground at every raid.
            if d.launched {
                for g in [11, 12, 14, 17] {
                    assert_eq!(d.ground_ready[ix(g)], d.ready_start[ix(g)]);
                }
            }
        }
    }
}

#[test]
fn the_variant_with_a_radius_of_261_for_the_a6m2_sends_no_escort_and_the_whole_patrol_fires_at_the_bombers(
) {
    let path = radius_of_the_a6m2(261);
    let cfg = Cfg {
        escorts_reach: false,
        ..scenario_cfg()
    };
    let runs = analyse_batch(&path, VARIANT_SEEDS, cfg);
    let (mut raids, mut at_bombers, mut over_rabaul, mut lost_by_fighters) = (0, 0, 0, 0);
    for w in &runs {
        for day in 1..=DAYS {
            let d = w.day(day);
            // No escort ever flies to Henderson Field.
            assert!(d.escorts_by.iter().all(|e| *e == 0), "seed {}", w.seed);
            if !d.launched {
                continue;
            }
            raids += 1;
            // Every raid is unescorted: the whole patrol fires at the bombers.
            assert!(d.rolls.iter().all(
                |r| r.kind != RollKind::EscortsAtPatrol && r.kind != RollKind::PatrolAtEscorts
            ));
            let patrol: u32 = ALLIED_FIGHTERS.iter().map(|g| d.patrol_by[ix(*g)]).sum();
            let fired = d
                .rolls
                .iter()
                .find(|r| r.kind == RollKind::PatrolAtBombers)
                .map_or(0, |r| r.shots);
            assert_eq!(fired, patrol, "seed {}, day {day}", w.seed);
            if patrol > 0 {
                at_bombers += 1;
            }
            lost_by_fighters += d.bombers_by_fighters;
            // The fighters that cannot go are a patrol over Rabaul, which
            // meets nobody.
            for g in JAPANESE_FIGHTERS {
                over_rabaul += d.patrol_by[ix(g)];
            }
        }
        for g in JAPANESE_FIGHTERS {
            assert_eq!(
                w.shot[ix(g)],
                0,
                "seed {}: nobody fights over Rabaul",
                w.seed
            );
        }
    }
    eprintln!(
        "A6M2 at 261 nm: {raids} raids in {VARIANT_SEEDS} runs, {at_bombers} met a patrol, {lost_by_fighters} bombers shot down by fighters, {over_rabaul} fighter sorties over Rabaul"
    );
    assert!(raids > 0 && at_bombers > 0 && over_rabaul > 0 && lost_by_fighters > 0);
}

#[test]
fn the_variant_with_a_radius_of_513_for_the_g4m1_launches_no_raid_and_loses_nothing_at_henderson_field(
) {
    let path = radius_of_the_g4m1(513);
    let cfg = Cfg {
        raid_reach: false,
        ..scenario_cfg()
    };
    let runs = analyse_batch(&path, VARIANT_SEEDS, cfg);
    let mut patrols = 0;
    for w in &runs {
        let seed = w.seed;
        assert!(
            w.raid_days().is_empty(),
            "seed {seed}: no raid is ever launched"
        );
        for g in GROUPS.into_iter().filter(|g| !japanese(*g)) {
            // Nothing is lost at Henderson Field.
            assert_eq!(w.shot[ix(g)], 0, "seed {seed}");
            assert_eq!(w.on_the_ground[ix(g)], 0);
        }
        assert_eq!(w.supply_destroyed, 0);
        // It still patrols, and pays for it.
        for day in 1..=DAYS {
            let d = w.day(day);
            patrols += d.wear.iter().filter(|r| r.group == VMF_223).count();
            assert!(
                d.wear.iter().all(|r| !japanese(r.group)),
                "seed {seed}: Japanese aircraft fly"
            );
        }
        assert!(w.supply[0] < fx(START_STOCK[0]), "seed {seed}");
    }
    eprintln!("G4M1 at 513 nm: {patrols} patrol wear rolls of VMF-223 in {VARIANT_SEEDS} runs");
    assert!(patrols > 0);
}

#[test]
fn the_variant_with_no_fatigue_file_stands_nobody_down_and_flies_a_raid_every_morning_group_1_has_a_bomber_and_a_pilot(
) {
    let path = without_the_fatigue_file();
    let cfg = Cfg {
        fatigue: false,
        ..scenario_cfg()
    };
    let runs = analyse_batch(&path, VARIANT_SEEDS, cfg);
    let mut raids = 0;
    for w in &runs {
        let seed = w.seed;
        for day in 1..=DAYS {
            let d = w.day(day);
            assert!(d.stood.iter().all(|s| !s), "seed {seed}, day {day}");
            assert!(d.fatigue_end.iter().all(|f| f.is_zero()), "seed {seed}");
            // Everybody flies all they can.
            for g in GROUPS {
                let i = ix(g);
                let can = d.ready_start[i].min(d.pilots_start[i]);
                assert_eq!(d.able_morning[i], can, "seed {seed}, day {day}, group {g}");
            }
            // A raid on every morning group 1 has a bomber and a pilot.
            assert_eq!(
                d.launched,
                d.ready_start[ix(1)].min(d.pilots_start[ix(1)]) >= 1,
                "seed {seed}, day {day}"
            );
            raids += u32::from(d.launched);
        }
    }
    eprintln!("no fatigue file: {raids} raids in {VARIANT_SEEDS} runs");
    assert!(raids > 0);
}

#[test]
fn the_variant_with_no_arrivals_never_gives_groups_3_and_12_to_17_an_aircraft_and_vmf_223_defends_alone(
) {
    let path = without_arrivals();
    let cfg = Cfg {
        arrivals: false,
        ..scenario_cfg()
    };
    let runs = analyse_batch(&path, VARIANT_SEEDS, cfg);
    for w in &runs {
        let seed = w.seed;
        for g in [3, 12, 13, 14, 15, 16, 17] {
            let i = ix(g);
            for day in 1..=DAYS {
                let d = w.day(day);
                assert!(
                    d.ready_start[i] == 0 && d.ready_end[i] == 0 && d.damaged_end[i] == 0,
                    "seed {seed}, day {day}, group {g}"
                );
                assert_eq!(d.able_morning[i], 0);
            }
            assert_eq!(w.arrived[i], 0);
        }
        for day in 1..=DAYS {
            let d = w.day(day);
            for g in ALLIED_FIGHTERS.into_iter().filter(|g| *g != VMF_223) {
                assert_eq!(d.patrol_by[ix(g)], 0, "seed {seed}, day {day}");
            }
            assert_eq!(d.patrol_by[ix(SIXTH)], 0);
        }
    }
}

#[test]
fn the_variant_with_no_replacements_gives_groups_1_2_and_3_nothing_after_their_arrivals() {
    let path = without_replacements();
    let cfg = Cfg {
        replacements: false,
        ..scenario_cfg()
    };
    let runs = analyse_batch(&path, VARIANT_SEEDS, cfg);
    let mut bombers_at_the_end = 0;
    for w in &runs {
        let seed = w.seed;
        for g in [1, 2, 3] {
            let i = ix(g);
            assert_eq!(w.received_aircraft[i], 0, "seed {seed}, group {g}");
            assert_eq!(w.received_pilots[i], 0, "seed {seed}, group {g}");
            assert!(
                w.ready[i] + w.damaged[i] <= START[i] + w.arrived[i],
                "seed {seed}, group {g}"
            );
        }
        bombers_at_the_end += w.ready[ix(1)] + w.damaged[ix(1)];
    }
    eprintln!("no replacements: group 1 has {bombers_at_the_end} aircraft at the end over {VARIANT_SEEDS} runs");
}

#[test]
fn the_air_distance_is_one_function_and_a_raid_goes_only_within_the_radius() {
    // The raid is launched at a base within the bombers' radius by the air
    // distance, and escorts go only within theirs: the rebuild of every run
    // asserts each launch event's distance is 556, and the radii are those of
    // the table.
    let scenario = load();
    let data = scenario.world.ref_data();
    let radius = |id: u32| data.aircraft_types[&AircraftTypeId::new(id)].strike_range_nm;
    let distance = scenario.world.umpire().bases[&HENDERSON]
        .hex
        .air_distance_nm(scenario.world.umpire().bases[&RABAUL].hex);
    assert!(n(i64::from(distance)) <= radius(2103));
    assert!(n(i64::from(distance)) <= radius(2102));
    let mut launches = 0;
    for w in batch() {
        launches += w.raid_days().len();
    }
    assert!(launches > 0);
}

#[test]
fn recording_breakdowns_changes_neither_the_events_nor_the_hashes() {
    let base = load();
    for seed in [1u64, 2, 77, 500, 1942] {
        let mut with = base.with_seed(seed).expect("a seed can be set");
        let mut without = base.with_seed(seed).expect("a seed can be set");
        without.world.set_config(sim_core::world::SimConfig {
            record_breakdowns: false,
        });
        with.run_to_end().expect("runs");
        without.run_to_end().expect("runs");
        assert_eq!(
            with.world.day_hashes(),
            without.world.day_hashes(),
            "seed {seed}"
        );
        let (a, b): (Vec<&Event>, Vec<&Event>) = (
            with.world.events().iter().collect(),
            without.world.events().iter().collect(),
        );
        assert_eq!(a.len(), b.len());
        assert!(a.iter().any(|e| e.breakdown.is_some()));
        for (x, y) in a.iter().zip(&b) {
            assert_eq!(
                (x.id, x.day, x.pulse, &x.kind, &x.causes),
                (y.id, y.day, y.pulse, &y.kind, &y.causes)
            );
            assert!(y.breakdown.is_none());
        }
    }
}

#[test]
fn the_same_seed_gives_the_same_hashes_twice() {
    let base = load();
    let mut all_hashes: Vec<Vec<u64>> = Vec::new();
    for seed in [1u64, 2, 77, 1942] {
        let hashes: Vec<Vec<u64>> = (0..2)
            .map(|_| {
                let mut run = base.with_seed(seed).expect("a seed can be set");
                run.run_to_end().expect("the run resolves");
                run.world.day_hashes().to_vec()
            })
            .collect();
        assert_eq!(hashes[0].len(), DAYS as usize);
        assert_eq!(hashes[0], hashes[1], "seed {seed}");
        all_hashes.push(hashes[0].clone());
    }
    assert!(all_hashes.windows(2).any(|pair| pair[0] != pair[1]));
    // The batch holds the same hashes as a fresh run of the same seed.
    let mut again = base.with_seed(77).expect("a seed can be set");
    again.run_to_end().expect("the run resolves");
    assert_eq!(batch()[76].seed, 77);
    assert_eq!(batch()[76].hashes, again.world.day_hashes());
}

// ---------------------------------------------------------------------
// The spread, printed for the reader. Not an assertion about the rule.
// ---------------------------------------------------------------------

fn mean_range(values: impl Iterator<Item = i64>, scale: i64) -> String {
    let values: Vec<i64> = values.collect();
    let total: i64 = values.iter().sum();
    let (lo, hi) = (
        values.iter().min().copied().unwrap_or(0),
        values.iter().max().copied().unwrap_or(0),
    );
    let mean = total * 100 / (count(values.len()) * scale);
    format!(
        "mean {}.{:02}, range {} to {}",
        mean / 100,
        mean % 100,
        lo / scale,
        hi / scale
    )
}

fn mean2(total: i64, runs: i64) -> String {
    let m = total * 100 / runs;
    format!("{}.{:02}", m / 100, m % 100)
}

#[test]
fn print_the_summary() {
    let runs = batch();
    let total = count(runs.len());
    let raids: Vec<usize> = runs.iter().map(|w| w.raid_days().len()).collect();
    eprintln!("SEEDS 1 to {total}");
    eprintln!(
        "raids flown per run: {}; runs with all eleven: {}; runs with eleven on the eleven raid days: {}",
        mean_range(raids.iter().map(|r| count(*r)), 1),
        raids.iter().filter(|r| **r == 11).count(),
        runs.iter().filter(|w| w.raid_days() == RAID_DAYS).count()
    );
    eprintln!("by day, among the runs with a raid that day: runs, mean bombers launched, mean escorts, mean Allied fighters on patrol");
    for day in 1..=DAYS {
        let with: Vec<&Day> = runs
            .iter()
            .map(|w| w.day(day))
            .filter(|d| d.launched)
            .collect();
        if with.is_empty() {
            continue;
        }
        let k = count(with.len());
        let bombers: i64 = with.iter().map(|d| i64::from(d.bombers)).sum();
        let escorts: i64 = with
            .iter()
            .map(|d| i64::from(d.escorts_by[ix(2)] + d.escorts_by[ix(3)]))
            .sum();
        let patrol: i64 = with
            .iter()
            .map(|d| {
                ALLIED_FIGHTERS
                    .iter()
                    .map(|g| i64::from(d.patrol_by[ix(*g)]))
                    .sum::<i64>()
            })
            .sum();
        let by_fighters: i64 = with.iter().map(|d| i64::from(d.bombers_by_fighters)).sum();
        let by_guns: i64 = with.iter().map(|d| i64::from(d.bombers_by_guns)).sum();
        eprintln!(
            "day {day:2}: {k:4} runs, bombers {}, escorts {}, patrol {}; bombers shot down by fighters {}, by guns {}",
            mean2(bombers, k),
            mean2(escorts, k),
            mean2(patrol, k),
            mean2(by_fighters, k),
            mean2(by_guns, k)
        );
    }
    let sum = |f: &dyn Fn(u32) -> bool, a: &[u32; N]| -> i64 {
        GROUPS
            .iter()
            .filter(|g| f(**g))
            .map(|g| i64::from(a[ix(*g)]))
            .sum()
    };
    let allied = |g: u32| !japanese(g);
    let jap_fighters = |g: u32| g == 2 || g == 3;
    let bombers_only = |g: u32| g == 1;
    let all_japanese = |g: u32| japanese(g);
    let allied_fighters = |g: u32| ALLIED_FIGHTERS.contains(&g);
    eprintln!("totals per run, Japanese then Allied:");
    eprintln!(
        "  shot down in the air: bombers {}",
        mean_range(runs.iter().map(|w| sum(&bombers_only, &w.shot)), 1)
    );
    eprintln!(
        "  shot down in the air: escorts {}",
        mean_range(runs.iter().map(|w| sum(&jap_fighters, &w.shot)), 1)
    );
    eprintln!(
        "  shot down in the air: Allied fighters {}",
        mean_range(runs.iter().map(|w| sum(&allied_fighters, &w.shot)), 1)
    );
    eprintln!(
        "  destroyed on the ground: Japanese {}; Allied {}",
        mean_range(runs.iter().map(|w| sum(&all_japanese, &w.on_the_ground)), 1),
        mean_range(runs.iter().map(|w| sum(&allied, &w.on_the_ground)), 1)
    );
    eprintln!(
        "  written off: Japanese {}; Allied {}",
        mean_range(runs.iter().map(|w| sum(&all_japanese, &w.written_off)), 1),
        mean_range(runs.iter().map(|w| sum(&allied, &w.written_off)), 1)
    );
    eprintln!(
        "  damaged at the end: Japanese {}; Allied {}",
        mean_range(runs.iter().map(|w| sum(&all_japanese, &w.damaged)), 1),
        mean_range(runs.iter().map(|w| sum(&allied, &w.damaged)), 1)
    );
    eprintln!(
        "  pilots lost: Japanese {}; Allied {}",
        mean_range(runs.iter().map(|w| sum(&all_japanese, &w.pilots_lost)), 1),
        mean_range(runs.iter().map(|w| sum(&allied, &w.pilots_lost)), 1)
    );
    for g in ALLIED_FIGHTERS {
        let out: Vec<i64> = runs
            .iter()
            .filter_map(|w| {
                (1..=DAYS)
                    .find(|d| w.day(*d).ready_end[ix(g)] == 0 && w.day(*d).ready_start[ix(g)] > 0)
                    .map(i64::from)
            })
            .collect();
        eprintln!(
            "group {g}: runs in which it is left with no ready aircraft at the end of a day: {}; mean first such day {}",
            out.len(),
            mean2(out.iter().sum(), count(out.len()).max(1))
        );
    }
    eprintln!("mean ready aircraft and pilots at the end, by group:");
    for g in GROUPS {
        let ready: i64 = runs.iter().map(|w| i64::from(w.ready[ix(g)])).sum();
        let pilots: i64 = runs.iter().map(|w| i64::from(w.pilots[ix(g)])).sum();
        eprintln!(
            "  group {g:2}: ready {}, pilots {}",
            mean2(ready, total),
            mean2(pilots, total)
        );
    }
    for (bi, name) in ["Henderson Field", "Rabaul"].into_iter().enumerate() {
        let left: i64 = runs.iter().map(|w| w.day(DAYS).stock_end[bi]).sum();
        eprintln!(
            "supply left at {name}: mean {} t of {}",
            mean2(left, total * 100),
            START_STOCK[bi] / 100
        );
    }
    let destroyed: i64 = runs.iter().map(|w| w.supply_destroyed).sum();
    eprintln!(
        "supply destroyed by bombs at Henderson Field: mean {} t a run",
        mean2(destroyed, total * 100)
    );
    let runs_dry = runs
        .iter()
        .filter(|w| w.days.iter().any(|d| d.stock_end[0] == 0))
        .count();
    eprintln!("runs in which Henderson Field runs out of supply: {runs_dry}");
    // Fighters against the bombers.
    let by_fighters: i64 = runs
        .iter()
        .flat_map(|w| w.days.iter())
        .map(|d| i64::from(d.bombers_by_fighters))
        .sum();
    let by_guns: i64 = runs
        .iter()
        .flat_map(|w| w.days.iter())
        .map(|d| i64::from(d.bombers_by_guns))
        .sum();
    let all_raids = runs.iter().map(|w| w.raid_days().len()).sum::<usize>();
    let reached = runs
        .iter()
        .flat_map(|w| w.days.iter())
        .filter(|d| {
            d.launched
                && d.rolls
                    .iter()
                    .any(|r| r.kind == RollKind::PatrolAtBombers && r.shots > 0)
        })
        .count();
    eprintln!(
        "Japanese bombers shot down by fighters in all {total} runs: {by_fighters}; by guns: {by_guns}; raids in all: {all_raids}; raids in which any patrol shot reached the bombers: {reached} ({} percent)",
        mean2(count(reached) * 100, count(all_raids))
    );
}
