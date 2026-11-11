//! Scenario 5: Eastern Solomons, 24 August 1942.
//!
//! Stage 5e of scenario 5, and the first air battle with real ships and
//! aircraft. Two American carrier forces lie east of Guadalcanal; the
//! Japanese carrier striking force and the Ryujo detachment come down from
//! the north.
//!
//! The figures are derived from sourced facts
//! (`data/reference/ship_classes_eastern_solomons.csv` and
//! `aircraft_types_eastern_solomons.csv`, decisions 0020 and 0029). What is
//! in the scenario and what is left out is
//! `docs/decisions/0031-eastern-solomons-scenario.md`. The search, air
//! combat and scaling figures are placeholders, so the losses are not a
//! historical check yet. This file asserts what the rules fix and prints the
//! spread beside what happened.
//!
//! # Hand calculation
//!
//! Written by the architect before the test code. The tests below assert
//! these numbers. If the code disagrees, the code or the rule is wrong: do
//! not change a number here to make a test pass.
//!
//! Quantities are cut to six decimal places at each step.
//!
//! ## The forces
//!
//! | Task force | Ships, by ID | Air groups, by ID |
//! | ---------- | ------------ | ----------------- |
//! | 1, Task Force 16, Allied | 1 Enterprise, 2 North Carolina, 3 Portland, 4 Atlanta, 5 to 10 destroyers | 1 VF-6, 27 F4F-4, patrol; 2 VB-6, 17 SBD-3, strike; 3 VS-5, 16 SBD-3, search; 4 VT-3, 14 TBF-1, strike |
//! | 2, Task Force 11, Allied | 11 Saratoga, 12 New Orleans, 13 Minneapolis, 14 to 18 destroyers | 5 VF-5, 27 F4F-4, patrol; 6 VB-3, 15 SBD-3, strike; 7 VS-3, 15 SBD-3, strike; 8 VT-8, 13 TBF-1, strike |
//! | 3, carrier striking force, Japanese | 19 Shokaku, 20 Zuikaku, 21 Chikuma, 22 Hiei, 23 Kirishima, 24 Kumano, 25 Suzuya, 26 Nagara, 27 to 32 destroyers | 11 fighters, 26 A6M2, patrol share 0.8; 12 bombers, 27 D3A1, strike; 13 attack, 18 B5N2, strike (Shokaku). 14 fighters, 25, share 0.8; 15 bombers, 27; 16 attack, 18 (Zuikaku). 17 floatplanes, 5 E13A1, search (Chikuma) |
//! | 4, Ryujo detachment, Japanese | 33 Ryujo, 34 Tone, 35 Amatsukaze, 36 Tokitsukaze | 18 fighters, 23 A6M2, patrol share 0.3; 19 attack, 6 B5N2, strike (Ryujo). 20 floatplanes, 5 E13A1, search (Tone) |
//!
//! At Henderson Field, base 1, in (2,-4): 9 VMF-223, 13 F4F-4, patrol; 10
//! VMSB-232, 11 SBD-3, strike.
//!
//! | Aircraft | Radius | Attacks with | Pilot armour | Self-sealing tanks |
//! | -------- | ------ | ------------ | ------------ | ------------------ |
//! | F4F-4    | 105 nm | nothing      | yes          | yes                |
//! | SBD-3    | 200 nm | a bomb, damage 75.6 | yes   | yes                |
//! | TBF-1    | 190 nm | a torpedo, damage 73.2 | yes | yes               |
//! | A6M2     | 261 nm | nothing      | no           | no                 |
//! | D3A1     | 252 nm | a bomb, damage 41.666666 | no | no               |
//! | B5N2     | 175 nm | a torpedo, damage 81.64 | no | no                |
//! | E13A1    | 253 nm | nothing      | no           | no                 |
//!
//! ## Where everyone is
//!
//! Both American forces lie stopped in (7,-5) all day.
//!
//! The carrier striking force starts in (9,-19) and steams for (8,-11), 8
//! hexes and 320 nm, at 17 knots. The Ryujo detachment starts in (10,-19)
//! and steams for (7,-10), 9 hexes and 360 nm, at 20 knots. A force's
//! position is the nearest hex centre.
//!
//! | At | Striking force | Steamed | Ryujo detachment | Steamed |
//! | -- | -------------- | ------- | ---------------- | ------- |
//! | 06:00 | (8,-14), 4 nm past its centre | 204 nm | (7,-13), at its centre | 240 nm |
//! | 12:00 | (8,-11), 14 nm short of its centre | 306 nm | (7,-10), arrived at 12:00, hour 18 | 360 nm |
//! | 18:00 | (8,-11), arrived at hour 18.823529 | 320 nm | (7,-10) | 360 nm |
//!
//! Distances between hex centres, in nm:
//!
//! | At | American carriers to striking force | American carriers to Ryujo | Henderson Field to Ryujo |
//! | -- | ----------------------------------- | -------------------------- | ------------------------ |
//! | 06:00 | 9 hexes, 360 | 8 hexes, 320 | 9 hexes, 360 |
//! | 12:00 | 6 hexes, 240 | 5 hexes, 200 | 6 hexes, 240 |
//!
//! ## Fuel
//!
//! Shokaku cruises at 18 knots, so at 17 she burns at her cruise rate, 4100
//! t over 9700 nm: 86.226804 t in the night, 43.113402 t in the morning and
//! 5.917525 t for the last 14 nm, 135.257731 t in all. Ryujo cruises at 14
//! knots, so at 20 her burn is multiplied by (20 / 14) squared: 1.428571
//! squared is 2.040815. She burns 240 x 2490 / 10000 = 59.76, times that,
//! 121.959104 t in the night, and 60.979552 t in the morning, and has
//! 2307.061344 t left.
//!
//! ## The morning
//!
//! Every search is flown. No force is within reach of any search, so there
//! is no attempt, no contact, no strike and no loss in the morning of any
//! run.
//!
//! ## The afternoon: search
//!
//! The chance of a sighting is 57.295779 x aircraft x 20 / (arc x distance).
//!
//! | Searcher | From | Arc | Target | Bearing | Distance | Chance |
//! | -------- | ---- | --- | ------ | ------- | -------- | ------ |
//! | Group 3, 16 SBD-3 | (7,-5) | 290 to 090, 160 degrees, to 200 nm | Ryujo detachment | 330 | 200 | 0.572957 |
//! | Group 3 | (7,-5) | | Striking force | | 240 | beyond 200 nm: no attempt |
//! | Group 17, 5 E13A1 | (8,-11) | 110 to 200, 90 degrees, to 253 nm | Each American force | about 159 | 240 | 0.265258 |
//! | Group 20, 5 E13A1 | (7,-10) | 110 to 200 | Each American force | 150 | 200 | 0.318309 |
//!
//! Each floatplane group rolls once for each of the two American forces, so
//! the Japanese make four attempts. The chance that none sights is
//! 0.734742 x 0.734742 x 0.681691 x 0.681691 = 0.250867, so the Japanese
//! side holds a contact in the afternoon with chance 0.749133. (First
//! written as 0.250869 and 0.749131, a slip in the multiplication. The mean
//! and the band are unchanged.)
//!
//! | Count of runs, over seeds 1 to 1000 | p        | Mean   | Std dev | Allowed    |
//! | ----------------------------------- | -------- | ------ | ------- | ---------- |
//! | The Allied side sights the Ryujo detachment | 0.572957 | 572.96 | 15.64 | 511 to 635 |
//! | The Japanese side sights an American force  | 0.749133 | 749.13 | 13.71 | 695 to 803 |
//!
//! ## The afternoon: who launches
//!
//! A group launches at a contact its side holds within its radius of where
//! the group is. A report names the true hex or one next to it.
//!
//! - **Groups 2, 6 and 7, the SBD-3s:** launch if the contact's hex is within
//!   200 nm of (7,-5), which the true hex (7,-10) is, exactly. A report that
//!   names a hex 240 nm away launches nothing.
//! - **Groups 4 and 8, the TBF-1s:** radius 190 nm. They launch only at a
//!   contact reported 160 nm away, a hex nearer than the truth, and find
//!   nothing there.
//! - **Group 10 at Henderson Field:** the Ryujo detachment's true hex is 240
//!   nm from (2,-4). It launches only at a contact reported 200 nm away or
//!   less, and finds nothing there.
//! - **Groups 12 and 15, the D3A1s:** launch if the contact's hex is within
//!   252 nm of (8,-11), which the true hex (7,-5) is.
//! - **Groups 13 and 16, the striking force's B5N2s:** radius 175 nm, and
//!   every hex a report of the American forces can name is 200 nm or more
//!   from (8,-11). They never launch.
//! - **Group 19, Ryujo's B5N2s:** the true hex is 200 nm from (7,-10). They
//!   launch only at a contact reported in (7,-6), 160 nm away, and find
//!   nothing there. Ryujo's 17 escorts go with them: the A6M2's radius
//!   covers it. (The first version of this list said no B5N2 ever launches.
//!   It missed the hex between the two forces.)
//!
//! A strike attacks only if it flew to the hex its target is really in.
//!
//! ## An American raid on the Ryujo detachment
//!
//! Enterprise's raid is group 2: 17 SBD-3. Saratoga's raid is groups 6 and
//! 7: 30 SBD-3. Neither has an escort: the target is 200 nm away and the
//! F4F-4's radius is 105 (decision 0030).
//!
//! Ryujo's fighters, 23 with a share of 0.3, keep 6 overhead (6.9 rounded
//! down). The other 17 are escorts, and fly only if group 19 launches. Each
//! American raid meets those 6, as they stood at the start of the pulse.
//!
//! A shot at an SBD-3 has both protection factors against it: a fighter's
//! 0.3 x 0.75 x 0.75 = 0.16875, a gun's 0.05 x 0.75 x 0.75 = 0.028125. The
//! detachment's ships have 13 + 11 + 1 + 1 = 26 anti-aircraft shots.
//!
//! | Step | Shots | Chance | Expected | Std dev |
//! | ---- | ----- | ------ | -------- | ------- |
//! | Patrol at the bombers | 6 | 0.16875 | 1.0125 | 0.917410 |
//! | Guns at the bombers | 26 | 0.028125 | 0.73125 | 0.843021 |
//!
//! Each shot is rolled on its own (decision 0033), so a roll scores from no
//! hit to its shots. Of Enterprise's 17 bombers 17 less both losses attack,
//! 15.25625 on average, and of Saratoga's 30, 28.25625 on average. Over the
//! runs the hits of each roll are within four standard deviations of the
//! expected figure times the number of such raids. (Before decision 0033
//! the patrol shot down 1 or 2 and the guns 0 or 1.)
//! A group whose bombers are all shot down attacks nothing, and records
//! neither an attack nor a strike that found nothing.
//! Saratoga's losses come off group 6 first. Every bomber attacks Ryujo,
//! ship 33, at 0.2 a bomb. A hit does 100 x 75.6 / 424 = 17.830188 points.
//!
//! ## A Japanese raid on the American carriers
//!
//! Shokaku's raid is group 12, 27 D3A1, and Zuikaku's is group 15, 27 D3A1.
//! They are two raids, because they leave from two ships. The target is 240
//! nm away and the A6M2's radius is 261, so the escorts go: Shokaku's 26
//! fighters keep 20 (20.8 rounded down) and send 6; Zuikaku's 25 keep 20 and
//! send 5.
//!
//! Each raid meets the whole American patrol over (7,-5) as it stood at the
//! start of the pulse: groups 1 and 5, 54 fighters. Henderson Field's
//! fighters are in another hex.
//!
//! A shot at an F4F-4 is at 0.16875. A shot at an A6M2 or a D3A1, which have
//! no protection, is at 0.3, and a gun's at 0.05. The strike attacks task
//! force 1, whose ships have 22 + 34 + 15 + 20 + 10 + 5 + 5 + 4 + 4 + 4 =
//! 123 anti-aircraft shots.
//!
//! | Step | Shokaku's raid | Zuikaku's raid |
//! | ---- | -------------- | -------------- |
//! | Escorts at the patrol | 6 shots x 0.16875 = 1.0125 expected | 5 x 0.16875 = 0.84375 |
//! | Patrol at the escorts | 6 x 0.3 = 1.8 | 5 x 0.3 = 1.5 |
//! | Patrol at the bombers | 48 x 0.3 = 14.4 | 49 x 0.3 = 14.7 |
//! | Guns at the bombers | 123 x 0.05 = 6.15 | 6.15 |
//! | Bombers that attack | 27 less both losses: 6.45 on average | 6.15 on average |
//!
//! Each shot is rolled on its own (decision 0033): a roll of n shots at
//! chance p scores from no hit to n, but no more than the aircraft it fires
//! at, with a standard deviation of the square root of n x p x (1 - p):
//! 0.917410 and 0.837477 for the escorts, 1.122497 and 1.024695 for the
//! patrol at the escorts, 3.174902 and 3.207803 for the patrol at the
//! bombers, 2.417126 for the guns. Over the runs the hits of each roll are
//! within four standard deviations of its expected figure times the number
//! of such raids. (Before decision 0033 each roll scored the whole part of
//! its expected figure or one more, and 5 to 7 bombers attacked.)
//!
//! Losses to the American patrol come off group 1 first. Every bomber
//! attacks Enterprise, ship 1, at 0.2 a bomb. A hit does 100 x 41.666666 /
//! 792 = 5.260942 points. Saratoga's force is never attacked.
//!
//! ## What cannot happen
//!
//! - No ship fires a gun or a torpedo at another ship: no two opposing
//!   forces are ever in one hex.
//! - No F4F-4 ever escorts a raid, and no B5N2 of the striking force ever
//!   launches. No B5N2 ever attacks.
//! - No aircraft at Henderson Field is ever lost, and no ship of task force
//!   2 or task force 3 is ever damaged.
//! - The striking force is never sighted and never attacked.
//!
//! # Properties
//!
//! - Every strike is launched at a contact its own side holds, reported in
//!   that pulse or the one before, within the launching type's radius.
//! - Every escort sent is within its fighter type's radius of the raid's
//!   target.
//! - Every group's aircraft at the end are its aircraft at the start less
//!   the aircraft its loss events record, and none goes below zero. A group
//!   aboard a ship that sinks has none.
//! - No ship has negative fuel, and no damage track is above 100.
//! - Every damage event is caused by an attack, every attack by a launch,
//!   and every launch by a report to the launching side.
//! - A replay of any seed reproduces the daily hash.
//!
//! # What happened, for comparison only
//!
//! Saratoga's 30 dive bombers and 8 torpedo planes sank Ryujo for no loss
//! or one. The Japanese first wave, 27 dive bombers and 10 fighters, hit
//! Enterprise with three bombs and lost 24 aircraft; a second wave of the
//! same size did not find the American force. No American ship was sunk.
//! The test prints how often each carrier is sunk and how many aircraft each
//! side loses over seeds 1 to 1000, and asserts nothing about those counts.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::OnceLock;

use sim_core::bearing;
use sim_core::events::{AirFirer, AirTarget, Cause, DamageTrack, Event, EventKind, Weapon};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, ContactId, EntityRef, EventId, OrderId, ShipId, TaskForceId};
use sim_core::map::Map;
use sim_core::orders::AirMission;
use sim_core::refdata::{
    AirCombatFigures, AircraftType, AttackWeapon, SearchFigures, ShipKind, SurfaceCombatFigures,
};
use sim_core::rng::{RngKey, RngStream, Subsystem};
use sim_core::state::{AirGroupLocation, ShipLocation, ShipSystem, Side, SystemState};
use sim_core::time::{self, Pulse};
use sim_data::Scenario;

const SEEDS: u64 = 1000;

// ---------------------------------------------------------------------
// The forces and groups, as the hand calculation's tables give them.
// ---------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Place {
    Ship(u32),
    Base(u32),
}

struct Spec {
    id: u32,
    side: Side,
    ready: u32,
    /// The aircraft type's ID.
    ty: u32,
    mission: AirMission,
    /// The patrol share in tenths.
    share: i64,
    place: Place,
}

const fn group(
    id: u32,
    side: Side,
    ready: u32,
    ty: u32,
    mission: AirMission,
    share: i64,
    place: Place,
) -> Spec {
    Spec {
        id,
        side,
        ready,
        ty,
        mission,
        share,
        place,
    }
}

const F4F4: u32 = 2000;
const SBD3: u32 = 2001;
const TBF1: u32 = 2002;
const A6M2: u32 = 2003;
const D3A1: u32 = 2004;
const B5N2: u32 = 2005;
const E13A1: u32 = 2006;

const PATROL: AirMission = AirMission::CombatAirPatrol;
const STRIKE: AirMission = AirMission::NavalStrike;
const SEARCH: AirMission = AirMission::Search;

/// The first table: the air groups, by ID.
const GROUPS: [Spec; 20] = [
    group(1, Side::Allied, 27, F4F4, PATROL, 10, Place::Ship(1)),
    group(2, Side::Allied, 17, SBD3, STRIKE, 10, Place::Ship(1)),
    group(3, Side::Allied, 16, SBD3, SEARCH, 10, Place::Ship(1)),
    group(4, Side::Allied, 14, TBF1, STRIKE, 10, Place::Ship(1)),
    group(5, Side::Allied, 27, F4F4, PATROL, 10, Place::Ship(11)),
    group(6, Side::Allied, 15, SBD3, STRIKE, 10, Place::Ship(11)),
    group(7, Side::Allied, 15, SBD3, STRIKE, 10, Place::Ship(11)),
    group(8, Side::Allied, 13, TBF1, STRIKE, 10, Place::Ship(11)),
    group(9, Side::Allied, 13, F4F4, PATROL, 10, Place::Base(1)),
    group(10, Side::Allied, 11, SBD3, STRIKE, 10, Place::Base(1)),
    group(11, Side::Japanese, 26, A6M2, PATROL, 8, Place::Ship(19)),
    group(12, Side::Japanese, 27, D3A1, STRIKE, 10, Place::Ship(19)),
    group(13, Side::Japanese, 18, B5N2, STRIKE, 10, Place::Ship(19)),
    group(14, Side::Japanese, 25, A6M2, PATROL, 8, Place::Ship(20)),
    group(15, Side::Japanese, 27, D3A1, STRIKE, 10, Place::Ship(20)),
    group(16, Side::Japanese, 18, B5N2, STRIKE, 10, Place::Ship(20)),
    group(17, Side::Japanese, 5, E13A1, SEARCH, 10, Place::Ship(21)),
    group(18, Side::Japanese, 23, A6M2, PATROL, 3, Place::Ship(33)),
    group(19, Side::Japanese, 6, B5N2, STRIKE, 10, Place::Ship(33)),
    group(20, Side::Japanese, 5, E13A1, SEARCH, 10, Place::Ship(34)),
];

/// The orders each group's standing orders came from, in the order they were
/// accepted (the mission first, then the arc or the share).
const ORDERS: [(u32, &[u32]); 20] = [
    (1, &[1]),
    (2, &[6]),
    (3, &[4, 5]),
    (4, &[7]),
    (5, &[2]),
    (6, &[8]),
    (7, &[9]),
    (8, &[10]),
    (9, &[3]),
    (10, &[11]),
    (11, &[16, 17]),
    (12, &[26]),
    (13, &[27]),
    (14, &[18, 19]),
    (15, &[28]),
    (16, &[29]),
    (17, &[22, 23]),
    (18, &[20, 21]),
    (19, &[30]),
    (20, &[24, 25]),
];

/// The second table: the aircraft. ID, name part, radius in nm, weapon,
/// damage in millionths, pilot armour, self-sealing tanks.
const AIRCRAFT: [(u32, &str, i64, AttackWeapon, i64, bool, bool); 7] = [
    (F4F4, "F4F-4", 105, AttackWeapon::None, 0, true, true),
    (
        SBD3,
        "SBD-3",
        200,
        AttackWeapon::Bomb,
        75_600_000,
        true,
        true,
    ),
    (
        TBF1,
        "TBF-1",
        190,
        AttackWeapon::Torpedo,
        73_200_000,
        true,
        true,
    ),
    (A6M2, "A6M2", 261, AttackWeapon::None, 0, false, false),
    (
        D3A1,
        "D3A1",
        252,
        AttackWeapon::Bomb,
        41_666_666,
        false,
        false,
    ),
    (
        B5N2,
        "B5N2",
        175,
        AttackWeapon::Torpedo,
        81_640_000,
        false,
        false,
    ),
    (E13A1, "E13A1", 253, AttackWeapon::None, 0, false, false),
];

/// The ships of each force, by the force's ID.
fn ships_of_force(force: u32) -> std::ops::RangeInclusive<u32> {
    match force {
        1 => 1..=10,
        2 => 11..=18,
        3 => 19..=32,
        4 => 33..=36,
        _ => panic!("no such force"),
    }
}

fn force_of(ship: u32) -> u32 {
    (1..=4)
        .find(|force| ships_of_force(*force).contains(&ship))
        .expect("a ship of a force")
}

fn side_of_force(force: u32) -> Side {
    if force <= 2 {
        Side::Allied
    } else {
        Side::Japanese
    }
}

/// Where each force is at the start of the afternoon, 12:00.
fn force_hex(force: u32) -> Hex {
    match force {
        1 | 2 => Hex::new(7, -5),
        3 => Hex::new(8, -11),
        4 => Hex::new(7, -10),
        _ => panic!("no such force"),
    }
}

const HENDERSON: Hex = Hex::new(2, -4);

fn place_hex(place: Place) -> Hex {
    match place {
        Place::Ship(ship) => force_hex(force_of(ship)),
        Place::Base(_) => HENDERSON,
    }
}

fn spec_of(group: u32) -> &'static Spec {
    &GROUPS[usize::try_from(group).expect("small") - 1]
}

fn orders_of(group: u32) -> Vec<Cause> {
    ORDERS
        .iter()
        .find(|(id, _)| *id == group)
        .expect("a group")
        .1
        .iter()
        .map(|order| Cause::Order(OrderId::new(*order)))
        .collect()
}

fn side_index(side: Side) -> usize {
    match side {
        Side::Allied => 0,
        Side::Japanese => 1,
    }
}

fn nm(hexes: u32) -> i64 {
    40 * i64::from(hexes)
}

// ---------------------------------------------------------------------
// Helpers.
// ---------------------------------------------------------------------

fn scenario_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/05_eastern_solomons.toml")
}

fn load() -> Scenario {
    Scenario::load(&scenario_path()).expect("scenario 5 (Eastern Solomons) loads")
}

fn millionths(value: i64) -> Fixed {
    Fixed::from_raw(value)
}

fn tenths(value: i64) -> Fixed {
    Fixed::from_ratio(value, 10)
}

fn hundredths(value: i64) -> Fixed {
    Fixed::from_ratio(value, 100)
}

fn int(value: u32) -> Fixed {
    Fixed::from_int(i64::from(value))
}

fn count(value: usize) -> i64 {
    i64::try_from(value).expect("small")
}

/// Whether k out of n is within four standard deviations of probability
/// `p` (in millionths), in exact integer arithmetic.
fn within_four_sigma(k: i64, n: i64, p: i64) -> bool {
    let (k, n, p) = (i128::from(k), i128::from(n), i128::from(p));
    let scale = 1_000_000;
    let off = k * scale - n * p;
    off * off <= 16 * n * p * (scale - p)
}

/// The hits a roll of `shots` at `chance` gives from its stream: one draw at
/// the chance for each shot (decision 0033), before any cap at what the
/// target has.
fn hits_from(stream: &mut RngStream, shots: u32, chance: Fixed) -> u32 {
    (0..shots).map(|_| u32::from(stream.chance(chance))).sum()
}

fn stream_of(
    seed: u64,
    pulse: Pulse,
    subsystem: Subsystem,
    entity: EntityRef,
    other: EntityRef,
) -> RngStream {
    RngKey {
        seed,
        day: 1,
        pulse: Some(pulse),
        subsystem,
        entity,
        other: Some(other),
    }
    .stream()
}

fn shots_text(shots: u32, chance: Fixed, expected: Fixed, hits: u32) -> String {
    format!(
        "{shots} shots x {chance} = {expected} expected, {hits} {}",
        if hits == 1 { "hit" } else { "hits" }
    )
}

fn scale() -> u64 {
    Fixed::SCALE.unsigned_abs()
}

fn raw_u64(value: Fixed) -> u64 {
    u64::try_from(value.raw()).expect("a chance is not negative")
}

fn event_ids(causes: &[EventId]) -> Vec<Cause> {
    causes.iter().copied().map(Cause::Event).collect()
}

/// The hand calculation's own arithmetic for a search chance: 57.295779 x
/// aircraft x 20 / (arc x distance), the numerator multiplied out first and
/// divided once.
fn search_chance(aircraft: i64, arc: i64, distance: i64) -> Fixed {
    Fixed::from_raw(57_295_779) * aircraft * Fixed::from_int(20) / (Fixed::from_int(distance) * arc)
}

// ---------------------------------------------------------------------
// The loaded reference data the rebuild reads.
// ---------------------------------------------------------------------

struct ShipInfo {
    name: String,
    kind: ShipKind,
    force: u32,
    aa: u32,
    toughness: Fixed,
    fuel: Fixed,
    cruise: Fixed,
    endurance: Fixed,
}

struct Ctx {
    map: Map,
    search: SearchFigures,
    air: AirCombatFigures,
    surface: SurfaceCombatFigures,
    types: BTreeMap<u32, AircraftType>,
    ships: BTreeMap<u32, ShipInfo>,
}

fn ctx() -> &'static Ctx {
    static CTX: OnceLock<Ctx> = OnceLock::new();
    CTX.get_or_init(|| {
        let scenario = load();
        let truth = scenario.world.umpire();
        let data = scenario.world.ref_data();
        let ships = truth
            .ships
            .values()
            .map(|ship| {
                let class = &data.ship_classes[&ship.class];
                let ShipLocation::TaskForce(force) = ship.location else {
                    panic!("every ship starts in a task force");
                };
                (
                    ship.id.raw(),
                    ShipInfo {
                        name: ship.name.clone(),
                        kind: class.kind,
                        force: force.raw(),
                        aa: class.anti_aircraft,
                        toughness: class.toughness,
                        fuel: ship.fuel_t,
                        cruise: class.cruise_speed_kn,
                        endurance: class.endurance_nm,
                    },
                )
            })
            .collect();
        Ctx {
            map: scenario.world.map().clone(),
            search: data.search.clone().expect("search figures"),
            air: data.air_combat.clone().expect("air combat figures"),
            surface: data.surface_combat.clone().expect("surface combat figures"),
            types: data
                .aircraft_types
                .values()
                .map(|t| (t.id.raw(), t.clone()))
                .collect(),
            ships,
        }
    })
}

// ---------------------------------------------------------------------
// What one run did, read from its events and final state.
// ---------------------------------------------------------------------

struct Final {
    flotation: Fixed,
    fire: Fixed,
    sunk: bool,
    systems: [SystemState; 5],
    fuel: Fixed,
}

struct Run {
    seed: u64,
    events: Vec<Event>,
    hashes: Vec<u64>,
    ships: BTreeMap<u32, Final>,
    /// The aircraft ready at the end of each group.
    ready: BTreeMap<u32, u32>,
    positions: BTreeMap<u32, Hex>,
}

fn run_of(base: &Scenario, seed: u64) -> Run {
    let mut run = base.with_seed(seed).expect("a seed can be set");
    run.run_to_end().expect("the run resolves");
    digest(&run, seed)
}

fn digest(run: &Scenario, seed: u64) -> Run {
    let truth = run.world.umpire();
    Run {
        seed,
        events: run.world.events().iter().cloned().collect(),
        hashes: run.world.day_hashes().to_vec(),
        ships: truth
            .ships
            .values()
            .map(|ship| {
                (
                    ship.id.raw(),
                    Final {
                        flotation: ship.flotation_damage,
                        fire: ship.fire_damage,
                        sunk: ship.location == ShipLocation::Sunk,
                        systems: ShipSystem::ALL.map(|system| ship.systems.get(system)),
                        fuel: ship.fuel_t,
                    },
                )
            })
            .collect(),
        ready: truth
            .air_groups
            .values()
            .map(|group| (group.id.raw(), group.ready))
            .collect(),
        positions: truth
            .task_forces
            .values()
            .map(|force| (force.id.raw(), force.position()))
            .collect(),
    }
}

fn batch() -> &'static Vec<Run> {
    static BATCH: OnceLock<Vec<Run>> = OnceLock::new();
    BATCH.get_or_init(|| {
        let base = load();
        (1..=SEEDS).map(|seed| run_of(&base, seed)).collect()
    })
}

/// Every run worked by the rule, asserting every event of the afternoon on
/// the way.
fn worked() -> &'static Vec<Worked> {
    static WORKED: OnceLock<Vec<Worked>> = OnceLock::new();
    WORKED.get_or_init(|| batch().iter().map(|run| work_the_run(run, ctx())).collect())
}

fn afternoon(run: &Run) -> Vec<&Event> {
    run.events
        .iter()
        .filter(|event| event.day == 1 && event.pulse == Some(Pulse::Afternoon))
        .collect()
}

fn morning(run: &Run) -> Vec<&Event> {
    run.events
        .iter()
        .filter(|event| event.day == 1 && event.pulse == Some(Pulse::Morning))
        .collect()
}

fn night(run: &Run) -> Vec<&Event> {
    run.events
        .iter()
        .filter(|event| event.day == 1 && event.pulse == Some(Pulse::Night))
        .collect()
}

// ---------------------------------------------------------------------
// Working the afternoon of a run from the state at its start.
// ---------------------------------------------------------------------

/// A contact a side holds: what its latest report said.
#[derive(Clone, Debug)]
struct Held {
    hex: Hex,
    kind: ShipKind,
    report: EventId,
}

/// A side's belief, as far as the afternoon needs it.
#[derive(Default)]
struct Belief {
    contacts: BTreeMap<u32, Held>,
    allocated: u32,
}

impl Belief {
    /// The contact a sighting at `hex` refreshes, else the next new ID (a
    /// contact's ID is never reused). Every contact in the afternoon is of
    /// age 0, so only the distance matters.
    fn contact_for(&self, cx: &Ctx, hex: Hex) -> (u32, bool) {
        let nearest = self
            .contacts
            .iter()
            .filter(|(_, held)| held.hex.distance(hex) <= cx.search.merge_distance_hexes)
            .min_by_key(|(_, held)| held.hex.distance(hex))
            .map(|(id, _)| *id);
        match nearest {
            Some(id) => (id, false),
            None => (self.allocated + 1, true),
        }
    }

    fn hold(&mut self, id: u32, held: Held) {
        self.allocated = self.allocated.max(id);
        self.contacts.insert(id, held);
    }
}

struct Cursor<'a> {
    events: &'a [&'a Event],
    at: usize,
    seed: u64,
}

impl<'a> Cursor<'a> {
    fn next(&mut self, what: &str) -> &'a Event {
        let event =
            self.events.get(self.at).copied().unwrap_or_else(|| {
                panic!("seed {}: the log ends where {what} should be", self.seed)
            });
        self.at += 1;
        event
    }

    fn peek(&self) -> Option<&'a Event> {
        self.events.get(self.at).copied()
    }
}

/// One launch, as the rule works it.
#[derive(Clone, Debug)]
struct Launch {
    group: u32,
    contact: u32,
    hex: Hex,
    aircraft: u32,
    distance_nm: i64,
    event: EventId,
    report: EventId,
}

/// One raid, as the rule works it.
#[derive(Clone, Debug)]
struct RaidWork {
    place: Place,
    side: Side,
    hex: Hex,
    /// The strike groups, in group order, and the aircraft each launched.
    strikes: Vec<(u32, u32)>,
    /// The fighter groups that sent escorts, and how many.
    escorts: Vec<(u32, u32)>,
    found: bool,
    target_force: Option<u32>,
    target_ship: Option<u32>,
    patrol: u32,
    escort_count: u32,
    /// (shots, hits) of each roll, or `None` for a roll never made.
    escorts_at_patrol: Option<(u32, u32)>,
    patrol_at_escorts: Option<(u32, u32)>,
    patrol_at_bombers: Option<(u32, u32)>,
    guns: Option<(u32, u32)>,
    bombers: u32,
    bombers_attacking: u32,
}

#[derive(Clone, Debug)]
struct AttackWork {
    group: u32,
    ship: u32,
    shots: u32,
    hits: u32,
    flotation_hits: u32,
    fire_hits: u32,
}

#[derive(Debug)]
struct Worked {
    seed: u64,
    /// Each search attempt: (group, force searched for, sighted).
    attempts: Vec<(u32, u32, bool)>,
    /// The hex each side's last report of the afternoon's search named.
    last_report: [Option<Hex>; 2],
    launches: Vec<Launch>,
    raids: Vec<RaidWork>,
    attacks: Vec<AttackWork>,
    /// Aircraft lost in the air, by group.
    lost: BTreeMap<u32, u32>,
    /// Ships sunk.
    sunk: Vec<u32>,
}

/// The report a sighting draws from the rest of its stream: position error,
/// then count, then kind (decision 0012).
fn report_from(
    cx: &Ctx,
    stream: &mut RngStream,
    true_hex: Hex,
    true_count: u32,
    true_kind: ShipKind,
) -> (Hex, u32, ShipKind) {
    let direction = if stream.chance(cx.search.position_error_chance) {
        Some(usize::try_from(stream.below(6)).expect("below 6"))
    } else {
        None
    };
    let hex = match direction {
        None => true_hex,
        Some(index) => {
            let (dq, dr) = Hex::DIRECTIONS[index];
            let neighbour = Hex::new(true_hex.q + dq, true_hex.r + dr);
            if cx.map.is_navigable(neighbour) {
                neighbour
            } else {
                true_hex
            }
        }
    };
    let v = stream.below(scale());
    let under = raw_u64(cx.search.count_under_chance);
    let over = raw_u64(cx.search.count_over_chance);
    let count = if v < under {
        true_count.saturating_sub(1).max(1)
    } else if v < under + over {
        true_count + 1
    } else {
        true_count
    };
    let v = stream.below(scale());
    let row = &cx.search.kind_confusion[&true_kind];
    let mut cumulative = 0u64;
    let mut kind = None;
    for candidate in ShipKind::ALL {
        cumulative += row.get(&candidate).map_or(0, |chance| raw_u64(*chance));
        if v < cumulative {
            kind = Some(candidate);
            break;
        }
    }
    (hex, count, kind.expect("a confusion row adds to 1"))
}

/// Takes `hits` from the pool, the lowest group first, each until it has none
/// left. Hits beyond what is there are wasted.
fn take_hits(hits: u32, pool: &mut [(u32, u32)]) -> Vec<(u32, u32)> {
    let mut left = hits;
    let mut taken = Vec::new();
    for (group, aircraft) in pool.iter_mut() {
        let now = left.min(*aircraft);
        if now > 0 {
            *aircraft -= now;
            left -= now;
            taken.push((*group, now));
        }
    }
    taken
}

/// One roll of fighters or guns, worked and not yet recorded.
struct Roll {
    event: EventKind,
    text: String,
    causes: Vec<Cause>,
    factors: Vec<(&'static str, Fixed)>,
    expected: Fixed,
    lost: Vec<(u32, u32)>,
}

/// The chance of one shot at aircraft of `group`'s type: the base times the
/// pilot armour factor if the type has armour, times the tanks factor if it
/// has self-sealing tanks (decision 11 of 0029). Returns the chance and the
/// factors applied.
fn shot_chance(cx: &Ctx, base: Fixed, group: u32) -> (Fixed, Vec<(&'static str, Fixed)>) {
    let ty = &cx.types[&spec_of(group).ty];
    let mut chance = base;
    let mut factors = vec![("hit_chance", base)];
    if ty.pilot_armour {
        chance = chance * cx.air.pilot_armour_factor;
        factors.push(("pilot_armour_factor", cx.air.pilot_armour_factor));
    }
    if ty.self_sealing_tanks {
        chance = chance * cx.air.self_sealing_tanks_factor;
        factors.push((
            "self_sealing_tanks_factor",
            cx.air.self_sealing_tanks_factor,
        ));
    }
    (chance, factors)
}

/// Asserts one recorded roll against the rule.
fn check_roll(seed: u64, event: &Event, roll: &Roll) {
    assert_eq!(event.kind, roll.event, "seed {seed}");
    assert_eq!(event.kind.to_string(), roll.text, "seed {seed}");
    assert_eq!(event.causes, roll.causes, "seed {seed}");
    let breakdown = event.breakdown.as_ref().expect("a breakdown");
    assert_eq!(breakdown.result, roll.expected, "seed {seed}");
    assert_eq!(
        breakdown
            .factors
            .iter()
            .map(|f| (f.name, f.value))
            .collect::<Vec<_>>(),
        roll.factors,
        "seed {seed}"
    );
}

/// What a strike group did, before damage.
enum Outcome {
    Nothing,
    Attack {
        left: u32,
        ship: u32,
        force: u32,
        hits: u32,
        tracks: [u32; 2],
        report: (Hex, u32, ShipKind),
    },
    /// No bomber left: it records nothing.
    Spent,
}

/// Works the afternoon of `run` by the rule of the hand calculation from the
/// state at its start, and asserts every event of it. The morning has no
/// events but search and movement, and is asserted in its own test.
#[allow(clippy::too_many_lines)]
fn work_the_run(run: &Run, cx: &Ctx) -> Worked {
    let seed = run.seed;
    let pulse = Pulse::Afternoon;
    let events = afternoon(run);
    let mut cur = Cursor {
        events: &events,
        at: 0,
        seed,
    };
    let mut work = Worked {
        seed,
        attempts: Vec::new(),
        last_report: [None, None],
        launches: Vec::new(),
        raids: Vec::new(),
        attacks: Vec::new(),
        lost: BTreeMap::new(),
        sunk: Vec::new(),
    };
    let mut beliefs = [Belief::default(), Belief::default()];
    let mut afloat: BTreeSet<u32> = (1..=36).collect();
    let mut now: BTreeMap<u32, u32> = GROUPS.iter().map(|s| (s.id, s.ready)).collect();

    // ---- The search: each searching group, on its own stream. ----
    // Group, ship, hex, arc, range, aircraft, and each attempt's force,
    // distance and chance: the afternoon search table.
    type Attempts = &'static [(u32, i64, i64)];
    type Search = (u32, u32, Hex, (u32, u32), i64, u32, Attempts);
    let searches: [Search; 3] = [
        (
            3,
            1,
            Hex::new(7, -5),
            (290, 90),
            200,
            16,
            &[(4, 200, 572_957)],
        ),
        (
            17,
            21,
            Hex::new(8, -11),
            (110, 200),
            253,
            5,
            &[(1, 240, 265_258), (2, 240, 265_258)],
        ),
        (
            20,
            34,
            Hex::new(7, -10),
            (110, 200),
            253,
            5,
            &[(1, 200, 318_309), (2, 200, 318_309)],
        ),
    ];
    let force_counts = [0u32, 10, 8, 14, 4];
    for (group, ship, hex, arc, range, aircraft, attempts) in searches {
        let flown = cur.next("a search flown");
        assert_eq!(
            flown.kind,
            EventKind::SearchFlown {
                air_group: AirGroupId::new(group),
                from: AirGroupLocation::Ship(ShipId::new(ship)),
                hex,
                from_deg: arc.0,
                to_deg: arc.1,
                range_nm: Fixed::from_int(range),
                aircraft,
            },
            "seed {seed}"
        );
        assert_eq!(flown.causes, orders_of(group), "seed {seed}");
        let side = spec_of(group).side;
        for &(force, distance, chance) in attempts {
            let attempt = cur.next("a search attempt");
            let mut stream = stream_of(
                seed,
                pulse,
                Subsystem::Search,
                EntityRef::AirGroup(AirGroupId::new(group)),
                EntityRef::TaskForce(TaskForceId::new(force)),
            );
            let sighted = stream.chance(millionths(chance));
            assert_eq!(
                attempt.kind,
                EventKind::SearchAttempt {
                    air_group: AirGroupId::new(group),
                    task_force: TaskForceId::new(force),
                    distance_nm: Fixed::from_int(distance),
                    chance: millionths(chance),
                    sighted,
                },
                "seed {seed}"
            );
            assert_eq!(attempt.causes, vec![Cause::Event(flown.id)], "seed {seed}");
            let breakdown = attempt.breakdown.as_ref().expect("a breakdown");
            assert_eq!(breakdown.result, millionths(chance));
            assert_eq!(
                breakdown
                    .factors
                    .iter()
                    .map(|f| (f.name, f.value))
                    .collect::<Vec<_>>(),
                vec![
                    ("degrees_per_radian", Fixed::from_raw(57_295_779)),
                    ("aircraft", Fixed::from_int(i64::from(aircraft))),
                    ("sweep_width_nm", Fixed::from_int(20)),
                    (
                        "arc_deg",
                        Fixed::from_int(i64::from(bearing::arc_degrees(arc.0, arc.1)))
                    ),
                    ("distance_nm", Fixed::from_int(distance)),
                ]
            );
            work.attempts.push((group, force, sighted));
            if !sighted {
                continue;
            }
            let (hex, ship_count, kind) = report_from(
                cx,
                &mut stream,
                force_hex(force),
                force_counts[usize::try_from(force).expect("small")],
                ShipKind::Carrier,
            );
            let belief = &mut beliefs[side_index(side)];
            let (contact, created) = belief.contact_for(cx, hex);
            let report = cur.next("a report");
            assert_eq!(
                report.kind,
                EventKind::ReportReceived {
                    side,
                    contact: ContactId::new(contact),
                    hex,
                    ship_count,
                    kind,
                    created,
                },
                "seed {seed}"
            );
            assert_eq!(report.causes, vec![Cause::Event(attempt.id)]);
            belief.hold(
                contact,
                Held {
                    hex,
                    kind,
                    report: report.id,
                },
            );
            work.last_report[side_index(side)] = Some(hex);
        }
    }

    // ---- The decisions: each strike group on its own side's belief. ----
    for spec in &GROUPS {
        let ty = &cx.types[&spec.ty];
        if spec.mission != STRIKE || ty.attack_weapon == AttackWeapon::None || spec.ready == 0 {
            continue;
        }
        let origin = place_hex(spec.place);
        let chosen = beliefs[side_index(spec.side)]
            .contacts
            .iter()
            .filter_map(|(id, held)| {
                let distance = nm(origin.distance(held.hex));
                (Fixed::from_int(distance) <= ty.strike_range_nm).then_some((
                    held.kind,
                    distance,
                    *id,
                    held.hex,
                    held.report,
                ))
            })
            .min_by_key(|(kind, distance, id, _, _)| (*kind, *distance, *id));
        let Some((_, distance, contact, hex, report)) = chosen else {
            continue;
        };
        let event = cur.next("a launch");
        assert_eq!(
            event.kind,
            EventKind::StrikeLaunched {
                air_group: AirGroupId::new(spec.id),
                contact: ContactId::new(contact),
                hex,
                aircraft: spec.ready,
                distance_nm: Fixed::from_int(distance),
            },
            "seed {seed}"
        );
        assert_eq!(
            event.kind.to_string(),
            format!(
                "air_group:{} launched {} aircraft at contact:{contact} at {hex}, {distance} nm",
                spec.id, spec.ready
            )
        );
        let mut causes = vec![orders_of(spec.id)[0]];
        causes.push(Cause::Event(report));
        assert_eq!(event.causes, causes, "seed {seed}");
        work.launches.push(Launch {
            group: spec.id,
            contact,
            hex,
            aircraft: spec.ready,
            distance_nm: distance,
            event: event.id,
            report,
        });
    }

    // ---- Raids: the strikes of one place at one hex. ----
    struct Raid {
        place: Place,
        hex: Hex,
        side: Side,
        strikes: Vec<usize>,
        escorts: Vec<usize>,
    }
    let mut raids: Vec<Raid> = Vec::new();
    for (index, launch) in work.launches.iter().enumerate() {
        let spec = spec_of(launch.group);
        match raids
            .iter_mut()
            .find(|raid| raid.place == spec.place && raid.hex == launch.hex)
        {
            Some(raid) => raid.strikes.push(index),
            None => raids.push(Raid {
                place: spec.place,
                hex: launch.hex,
                side: spec.side,
                strikes: vec![index],
                escorts: Vec::new(),
            }),
        }
    }
    // The fighters, each split into patrol and escorts.
    struct Fighter {
        group: u32,
        side: Side,
        place: Place,
        hex: Hex,
        patrol: u32,
        escorts: u32,
        radius: Fixed,
    }
    let mut fighters: Vec<Fighter> = GROUPS
        .iter()
        .filter(|s| s.mission == PATROL && s.ready > 0)
        .map(|s| {
            let patrol = (int(s.ready) * tenths(s.share)).trunc();
            let patrol = u32::try_from(patrol).expect("small");
            Fighter {
                group: s.id,
                side: s.side,
                place: s.place,
                hex: place_hex(s.place),
                patrol,
                escorts: s.ready - patrol,
                radius: cx.types[&s.ty].strike_range_nm,
            }
        })
        .collect();
    #[allow(clippy::needless_range_loop)]
    for index in 0..fighters.len() {
        if fighters[index].escorts == 0 {
            continue;
        }
        let chosen = raids
            .iter()
            .enumerate()
            .filter(|(_, raid)| raid.place == fighters[index].place)
            .max_by_key(|(_, raid)| {
                let bombers: u32 = raid
                    .strikes
                    .iter()
                    .map(|&i| work.launches[i].aircraft)
                    .sum();
                (bombers, Reverse(work.launches[raid.strikes[0]].group))
            })
            .map(|(raid_index, _)| raid_index);
        if let Some(raid_index) = chosen {
            let distance = nm(fighters[index].hex.distance(raids[raid_index].hex));
            if Fixed::from_int(distance) <= fighters[index].radius {
                raids[raid_index].escorts.push(index);
            } else {
                fighters[index].patrol += fighters[index].escorts;
                fighters[index].escorts = 0;
            }
        }
    }

    // ---- The air combat step, raid by raid, on the start-of-step counts. ----
    let mut outcomes: BTreeMap<u32, Outcome> = BTreeMap::new();
    let mut loss_ids: BTreeMap<u32, Vec<EventId>> = BTreeMap::new();
    for raid in &raids {
        let first = work.launches[raid.strikes[0]].clone();
        let mut raid_work = RaidWork {
            place: raid.place,
            side: raid.side,
            hex: raid.hex,
            strikes: raid
                .strikes
                .iter()
                .map(|&i| (work.launches[i].group, work.launches[i].aircraft))
                .collect(),
            escorts: raid
                .escorts
                .iter()
                .map(|&i| (fighters[i].group, fighters[i].escorts))
                .collect(),
            found: false,
            target_force: None,
            target_ship: None,
            patrol: 0,
            escort_count: 0,
            escorts_at_patrol: None,
            patrol_at_escorts: None,
            patrol_at_bombers: None,
            guns: None,
            bombers: raid_work_bombers(&work, &raid.strikes),
            bombers_attacking: 0,
        };
        // The escorts go with the raid, whether or not it finds anything.
        let mut escort_events: Vec<EventId> = Vec::new();
        for &index in &raid.escorts {
            let fighter = &fighters[index];
            let event = cur.next("escorts sent");
            assert_eq!(
                event.kind,
                EventKind::EscortsSent {
                    air_group: AirGroupId::new(fighter.group),
                    with_air_group: AirGroupId::new(first.group),
                    aircraft: fighter.escorts,
                },
                "seed {seed}"
            );
            assert_eq!(
                event.kind.to_string(),
                format!(
                    "air_group:{} sent {} escorts with air_group:{}",
                    fighter.group, fighter.escorts, first.group
                )
            );
            let mut causes = orders_of(fighter.group);
            causes.push(Cause::Event(first.event));
            assert_eq!(event.causes, causes, "seed {seed}");
            escort_events.push(event.id);
        }
        // The task force the raid finds: the one with the most important
        // ship, then the lowest number.
        let target = (1..=4u32)
            .filter(|&force| side_of_force(force) != raid.side && force_hex(force) == raid.hex)
            .filter_map(|force| {
                ships_of_force(force)
                    .filter(|ship| afloat.contains(ship))
                    .map(|ship| (cx.ships[&ship].kind, ship))
                    .min()
                    .map(|(kind, ship)| (kind, force, ship))
            })
            .min();
        let Some((_, force, ship)) = target else {
            for &(group, _) in &raid_work.strikes {
                outcomes.insert(group, Outcome::Nothing);
            }
            work.raids.push(raid_work);
            continue;
        };
        raid_work.found = true;
        raid_work.target_force = Some(force);
        raid_work.target_ship = Some(ship);
        let defending = side_of_force(force);

        // The pools the hits are taken from, each in group order.
        let mut bombers: Vec<(u32, u32)> = raid_work.strikes.clone();
        let mut escorts: Vec<(u32, u32)> = raid_work
            .escorts
            .iter()
            .copied()
            .filter(|(_, aircraft)| *aircraft > 0)
            .collect();
        let patrol: Vec<(u32, u32)> = fighters
            .iter()
            .filter(|f| f.side == defending && f.hex == raid.hex && f.patrol > 0)
            .map(|f| (f.group, f.patrol))
            .collect();
        let escort_count: u32 = escorts.iter().map(|(_, n)| n).sum();
        let patrol_count: u32 = patrol.iter().map(|(_, n)| n).sum();
        raid_work.patrol = patrol_count;
        raid_work.escort_count = escort_count;

        let mut rolls: Vec<Roll> = Vec::new();
        let escort_causes: Vec<Cause> = event_ids(&escort_events);
        let fight = |firer: u32,
                     target: u32,
                     firing: AirFirer,
                     at: AirTarget,
                     shots: u32,
                     chance: Fixed,
                     factors: Vec<(&'static str, Fixed)>,
                     hits: u32,
                     lost: Vec<(u32, u32)>|
         -> Roll {
            let expected = int(shots) * chance;
            let who = match firing {
                AirFirer::Escorts => "escorts",
                AirFirer::Patrol => "patrol",
            };
            let what = match at {
                AirTarget::Patrol => "patrol",
                AirTarget::Escorts => "escorts",
                AirTarget::Bombers => "bombers",
            };
            let mut causes = vec![Cause::Event(first.event)];
            if firing == AirFirer::Escorts || at == AirTarget::Escorts {
                causes.extend(escort_causes.iter().cloned());
            }
            let mut all = vec![("shots", int(shots))];
            all.extend(factors);
            Roll {
                event: EventKind::AirFight {
                    firer: AirGroupId::new(firer),
                    target: AirGroupId::new(target),
                    firing,
                    at,
                    shots: int(shots),
                    chance,
                    expected,
                    hits,
                },
                text: format!(
                    "air_group:{firer} {who} at the {what} of air_group:{target}: {}",
                    shots_text(shots, chance, expected, hits)
                ),
                causes,
                factors: all,
                expected,
                lost,
            }
        };
        if let (Some(&(first_escort, _)), Some(&(first_patrol, _))) =
            (escorts.first(), patrol.first())
        {
            let mut stream = stream_of(
                seed,
                pulse,
                Subsystem::AirCombat,
                EntityRef::AirGroup(AirGroupId::new(first_escort)),
                EntityRef::TaskForce(TaskForceId::new(force)),
            );
            let (chance, factors) = shot_chance(cx, cx.air.fighter_hit_chance, first_patrol);
            let hits = hits_from(&mut stream, escort_count, chance);
            raid_work.escorts_at_patrol = Some((escort_count, hits));
            rolls.push(fight(
                first_escort,
                first_patrol,
                AirFirer::Escorts,
                AirTarget::Patrol,
                escort_count,
                chance,
                factors,
                hits,
                take_hits(hits, &mut patrol.clone()),
            ));
        }
        if let Some(&(first_patrol, _)) = patrol.first() {
            let mut stream = stream_of(
                seed,
                pulse,
                Subsystem::AirCombat,
                EntityRef::AirGroup(AirGroupId::new(first_patrol)),
                EntityRef::AirGroup(AirGroupId::new(first.group)),
            );
            let at_escorts = patrol_count.min(escort_count);
            if let Some(&(first_escort, _)) = escorts.first() {
                let (chance, factors) = shot_chance(cx, cx.air.fighter_hit_chance, first_escort);
                let hits = hits_from(&mut stream, at_escorts, chance);
                raid_work.patrol_at_escorts = Some((at_escorts, hits));
                let lost = take_hits(hits, &mut escorts);
                rolls.push(fight(
                    first_patrol,
                    first_escort,
                    AirFirer::Patrol,
                    AirTarget::Escorts,
                    at_escorts,
                    chance,
                    factors,
                    hits,
                    lost,
                ));
            }
            let at_bombers = patrol_count - at_escorts;
            if at_bombers > 0 {
                let (chance, factors) = shot_chance(cx, cx.air.fighter_hit_chance, first.group);
                let hits = hits_from(&mut stream, at_bombers, chance);
                raid_work.patrol_at_bombers = Some((at_bombers, hits));
                let lost = take_hits(hits, &mut bombers);
                rolls.push(fight(
                    first_patrol,
                    first.group,
                    AirFirer::Patrol,
                    AirTarget::Bombers,
                    at_bombers,
                    chance,
                    factors,
                    hits,
                    lost,
                ));
            }
        }
        // The guns of every ship afloat in the force, at the bombers left.
        let gun_shots: u32 = ships_of_force(force)
            .filter(|ship| afloat.contains(ship))
            .map(|ship| cx.ships[&ship].aa)
            .sum();
        let bombers_left: u32 = bombers.iter().map(|(_, n)| n).sum();
        let lowest = ships_of_force(force).find(|ship| afloat.contains(ship));
        if let (true, true, Some(lowest)) = (gun_shots > 0, bombers_left > 0, lowest) {
            let mut stream = stream_of(
                seed,
                pulse,
                Subsystem::AirCombat,
                EntityRef::Ship(ShipId::new(lowest)),
                EntityRef::AirGroup(AirGroupId::new(first.group)),
            );
            let (chance, factors) = shot_chance(cx, cx.air.anti_aircraft_hit_chance, first.group);
            let hits = hits_from(&mut stream, gun_shots, chance);
            let expected = int(gun_shots) * chance;
            raid_work.guns = Some((gun_shots, hits));
            let mut all = vec![("shots", int(gun_shots))];
            all.extend(factors);
            rolls.push(Roll {
                event: EventKind::AntiAircraftFire {
                    task_force: TaskForceId::new(force),
                    air_group: AirGroupId::new(first.group),
                    shots: int(gun_shots),
                    chance,
                    expected,
                    hits,
                },
                text: format!(
                    "task_force:{force} guns at the raid of air_group:{}: {}",
                    first.group,
                    shots_text(gun_shots, chance, expected, hits)
                ),
                causes: vec![Cause::Event(first.event)],
                factors: all,
                expected,
                lost: take_hits(hits, &mut bombers),
            });
        }
        // Record the rolls, then the losses they cost.
        let mut recorded: Vec<EventId> = Vec::new();
        for roll in &rolls {
            let event = cur.next("a roll");
            check_roll(seed, event, roll);
            recorded.push(event.id);
        }
        for (roll, cause) in rolls.iter().zip(recorded) {
            for &(group, taken) in &roll.lost {
                let lost = taken.min(now[&group]);
                if lost == 0 {
                    continue;
                }
                let remaining = now[&group] - lost;
                now.insert(group, remaining);
                let event = cur.next("a loss");
                assert_eq!(
                    event.kind,
                    EventKind::AircraftLost {
                        air_group: AirGroupId::new(group),
                        aircraft: lost,
                        remaining,
                    },
                    "seed {seed}"
                );
                assert_eq!(
                    event.kind.to_string(),
                    format!("air_group:{group} lost {lost} aircraft, {remaining} left")
                );
                assert_eq!(event.causes, vec![Cause::Event(cause)], "seed {seed}");
                loss_ids.entry(group).or_default().push(event.id);
                *work.lost.entry(group).or_default() += lost;
            }
        }
        // What each strike group has left to attack with.
        let ship_count =
            u32::try_from(ships_of_force(force).filter(|s| afloat.contains(s)).count())
                .expect("few ships");
        raid_work.bombers_attacking = bombers.iter().map(|(_, n)| n).sum();
        for (&(group, left), launch_index) in bombers.iter().zip(&raid.strikes) {
            if left == 0 {
                outcomes.insert(group, Outcome::Spent);
                continue;
            }
            let launch = &work.launches[*launch_index];
            let ty = &cx.types[&spec_of(group).ty];
            let weapon_chance = match ty.attack_weapon {
                AttackWeapon::Bomb => cx.air.bomb_hit_chance,
                AttackWeapon::Torpedo => cx.air.torpedo_hit_chance,
                AttackWeapon::None => unreachable!(),
            };
            let mut stream = stream_of(
                seed,
                pulse,
                Subsystem::AirCombat,
                EntityRef::AirGroup(AirGroupId::new(group)),
                EntityRef::TaskForce(TaskForceId::new(force)),
            );
            let hits = hits_from(&mut stream, left, weapon_chance);
            let mut tracks = [0u32; 2];
            for _ in 0..hits {
                let track = if ty.attack_weapon == AttackWeapon::Bomb {
                    let draw = stream.below(scale());
                    usize::from(draw >= raw_u64(cx.surface.gun_hit_flotation))
                } else {
                    0
                };
                tracks[track] += 1;
                // One draw for the system, always.
                let _ = stream.below(scale());
            }
            let report = report_from(cx, &mut stream, launch.hex, ship_count, ShipKind::Carrier);
            outcomes.insert(
                group,
                Outcome::Attack {
                    left,
                    ship,
                    force,
                    hits,
                    tracks,
                    report,
                },
            );
        }
        work.raids.push(raid_work);
    }
    assert!(
        cur.peek().is_none_or(|e| !matches!(
            e.kind,
            EventKind::EscortsSent { .. }
                | EventKind::AirFight { .. }
                | EventKind::AntiAircraftFire { .. }
                | EventKind::AircraftLost { .. }
        )),
        "seed {seed}: nothing else in the air"
    );

    // ---- What each strike found, in group order. ----
    let mut finding: BTreeMap<u32, EventId> = BTreeMap::new();
    for launch in &work.launches {
        match &outcomes[&launch.group] {
            Outcome::Spent => {
                finding.insert(launch.group, launch.event);
            }
            Outcome::Nothing => {
                let event = cur.next("a strike that found nothing");
                assert_eq!(
                    event.kind,
                    EventKind::StrikeFoundNothing {
                        air_group: AirGroupId::new(launch.group),
                        hex: launch.hex,
                    },
                    "seed {seed}"
                );
                assert_eq!(
                    event.kind.to_string(),
                    format!("air_group:{} found nothing at {}", launch.group, launch.hex)
                );
                assert_eq!(event.causes, vec![Cause::Event(launch.event)]);
                finding.insert(launch.group, event.id);
            }
            Outcome::Attack {
                left,
                ship,
                force,
                hits,
                ..
            } => {
                let ty = &cx.types[&spec_of(launch.group).ty];
                assert_eq!(ty.attack_weapon, AttackWeapon::Bomb, "seed {seed}");
                let chance = cx.air.bomb_hit_chance;
                let expected = int(*left) * chance;
                let event = cur.next("an attack");
                assert_eq!(
                    event.kind,
                    EventKind::AirAttack {
                        air_group: AirGroupId::new(launch.group),
                        task_force: TaskForceId::new(*force),
                        ship: ShipId::new(*ship),
                        weapon: Weapon::Bomb,
                        shots: int(*left),
                        chance,
                        expected,
                        hits: *hits,
                    },
                    "seed {seed}"
                );
                assert_eq!(
                    event.kind.to_string(),
                    format!(
                        "air_group:{} bombs at ship:{ship}: {}",
                        launch.group,
                        shots_text(*left, chance, expected, *hits)
                    )
                );
                let mut causes = vec![Cause::Event(launch.event)];
                causes.extend(event_ids(
                    loss_ids
                        .get(&launch.group)
                        .map_or(&[][..], |v| v.as_slice()),
                ));
                assert_eq!(event.causes, causes, "seed {seed}");
                let breakdown = event.breakdown.as_ref().expect("a breakdown");
                assert_eq!(breakdown.result, expected);
                assert_eq!(
                    breakdown
                        .factors
                        .iter()
                        .map(|f| (f.name, f.value))
                        .collect::<Vec<_>>(),
                    vec![("aircraft", int(*left)), ("hit_chance", chance)]
                );
                finding.insert(launch.group, event.id);
            }
        }
    }

    // ---- Damage lands and the reports go out, in group order. ----
    let mut flotation: BTreeMap<u32, Fixed> = BTreeMap::new();
    let mut systems: BTreeMap<(u32, ShipSystem), SystemState> = BTreeMap::new();
    let mut fire: BTreeMap<u32, Fixed> = BTreeMap::new();
    for launch in &work.launches {
        let side = spec_of(launch.group).side;
        let own = side_index(side);
        match &outcomes[&launch.group] {
            Outcome::Spent => {}
            Outcome::Nothing => {
                let names = beliefs[own]
                    .contacts
                    .get(&launch.contact)
                    .is_some_and(|held| held.hex == launch.hex);
                if names {
                    beliefs[own].contacts.remove(&launch.contact);
                    let event = cur.next("a contact dropped");
                    assert_eq!(
                        event.kind,
                        EventKind::ContactNotThere {
                            side,
                            contact: ContactId::new(launch.contact),
                            hex: launch.hex,
                        },
                        "seed {seed}"
                    );
                    assert_eq!(event.causes, vec![Cause::Event(finding[&launch.group])]);
                }
            }
            Outcome::Attack {
                left,
                ship,
                force,
                hits,
                tracks,
                report,
            } => {
                let attack_id = finding[&launch.group];
                let info = &cx.ships[ship];
                let damage = cx.types[&spec_of(launch.group).ty].attack_damage;
                let per_hit = Fixed::from_int(100) * damage / info.toughness;
                let mut total = 0;
                for (slot, track) in DamageTrack::ALL.into_iter().enumerate() {
                    if tracks[slot] == 0 {
                        continue;
                    }
                    let values = match track {
                        DamageTrack::Flotation => &mut flotation,
                        DamageTrack::Fire => &mut fire,
                    };
                    let before = values.get(ship).copied().unwrap_or(Fixed::ZERO);
                    let points = per_hit * i64::from(tracks[slot]);
                    let after = (before + points).min(Fixed::from_int(100));
                    values.insert(*ship, after);
                    let event = cur.next("damage");
                    assert_eq!(
                        event.kind,
                        EventKind::ShipDamaged {
                            round: 0,
                            ship: ShipId::new(*ship),
                            track,
                            hits: tracks[slot],
                            points,
                            value_after: after,
                        },
                        "seed {seed}"
                    );
                    assert_eq!(event.causes, vec![Cause::Event(attack_id)]);
                    total += tracks[slot];
                    if track == DamageTrack::Flotation
                        && before < Fixed::from_int(100)
                        && after >= Fixed::from_int(100)
                    {
                        let sunk = cur.next("a sinking");
                        assert_eq!(
                            sunk.kind,
                            EventKind::ShipSunk {
                                round: 0,
                                ship: ShipId::new(*ship),
                                task_force: TaskForceId::new(*force),
                            },
                            "seed {seed}"
                        );
                        assert_eq!(sunk.causes, vec![Cause::Event(event.id)]);
                        afloat.remove(ship);
                        work.sunk.push(*ship);
                        for spec in GROUPS.iter().filter(|s| s.place == Place::Ship(*ship)) {
                            let lost = cur.next("an air group lost with its ship");
                            assert_eq!(
                                lost.kind,
                                EventKind::AirGroupLost {
                                    air_group: AirGroupId::new(spec.id),
                                    ship: ShipId::new(*ship),
                                    aircraft: now[&spec.id],
                                },
                                "seed {seed}"
                            );
                            assert_eq!(
                                lost.kind.to_string(),
                                format!(
                                    "air_group:{} lost with ship:{ship}: {} aircraft",
                                    spec.id, now[&spec.id]
                                )
                            );
                            assert_eq!(lost.causes, vec![Cause::Event(sunk.id)]);
                            now.insert(spec.id, 0);
                        }
                    }
                }
                assert_eq!(total, *hits, "seed {seed}: every hit is on a track");
                // The systems hit, in hit order.
                let mut system_hits = 0;
                while let Some(event) = cur.peek() {
                    let EventKind::SystemHit {
                        round,
                        ship: hit,
                        system,
                        before,
                        after,
                    } = event.kind
                    else {
                        break;
                    };
                    cur.next("a system hit");
                    assert_eq!(event.causes, vec![Cause::Event(attack_id)]);
                    assert_eq!(round, 0);
                    assert_eq!(hit, ShipId::new(*ship));
                    let held = systems
                        .get(&(*ship, system))
                        .copied()
                        .unwrap_or(SystemState::Working);
                    assert_eq!(before, held, "seed {seed}");
                    assert!(after == before || after == before.step_down());
                    systems.insert((*ship, system), after);
                    system_hits += 1;
                }
                assert!(system_hits <= *hits, "seed {seed}");
                // The report, citing the attack.
                let belief = &mut beliefs[own];
                let (contact, created) = belief.contact_for(cx, report.0);
                let received = cur.next("a report");
                assert_eq!(
                    received.kind,
                    EventKind::ReportReceived {
                        side,
                        contact: ContactId::new(contact),
                        hex: report.0,
                        ship_count: report.1,
                        kind: report.2,
                        created,
                    },
                    "seed {seed}"
                );
                assert_eq!(received.causes, vec![Cause::Event(attack_id)]);
                belief.hold(
                    contact,
                    Held {
                        hex: report.0,
                        kind: report.2,
                        report: received.id,
                    },
                );
                work.attacks.push(AttackWork {
                    group: launch.group,
                    ship: *ship,
                    shots: *left,
                    hits: *hits,
                    flotation_hits: tracks[0],
                    fire_hits: tracks[1],
                });
            }
        }
    }

    // ---- What is left is movement. ----
    while let Some(event) = cur.peek() {
        assert!(
            matches!(
                event.kind,
                EventKind::TaskForceSteamed { .. }
                    | EventKind::FuelBurned { .. }
                    | EventKind::TaskForceArrived { .. }
            ),
            "seed {seed}: unexpected {}",
            event.kind
        );
        cur.next("movement");
    }

    // ---- The end: the aircraft and damage the events leave. ----
    let ready_now: BTreeMap<u32, u32> = now;
    assert_eq!(run.ready, ready_now, "seed {seed}");
    for (id, last) in &run.ships {
        assert_eq!(
            (last.flotation, last.fire),
            (
                flotation.get(id).copied().unwrap_or(Fixed::ZERO),
                fire.get(id).copied().unwrap_or(Fixed::ZERO)
            ),
            "seed {seed}, ship {id}"
        );
        assert_eq!(last.sunk, !afloat.contains(id), "seed {seed}");
        for (slot, system) in ShipSystem::ALL.into_iter().enumerate() {
            assert_eq!(
                last.systems[slot],
                systems
                    .get(&(*id, system))
                    .copied()
                    .unwrap_or(SystemState::Working),
                "seed {seed}, ship {id}"
            );
        }
    }
    work
}

fn raid_work_bombers(work: &Worked, strikes: &[usize]) -> u32 {
    strikes.iter().map(|&i| work.launches[i].aircraft).sum()
}

// ---------------------------------------------------------------------
// The setting: the forces, the groups, the aircraft.
// ---------------------------------------------------------------------

/// The first table's ships: ID, a part of the name, the kind.
const SHIPS: [(u32, &str, ShipKind); 36] = [
    (1, "Enterprise", ShipKind::Carrier),
    (2, "North Carolina", ShipKind::Battleship),
    (3, "Portland", ShipKind::Cruiser),
    (4, "Atlanta", ShipKind::Cruiser),
    (5, "Balch", ShipKind::Destroyer),
    (6, "Benham", ShipKind::Destroyer),
    (7, "Ellet", ShipKind::Destroyer),
    (8, "Maury", ShipKind::Destroyer),
    (9, "Grayson", ShipKind::Destroyer),
    (10, "Monssen", ShipKind::Destroyer),
    (11, "Saratoga", ShipKind::Carrier),
    (12, "New Orleans", ShipKind::Cruiser),
    (13, "Minneapolis", ShipKind::Cruiser),
    (14, "Farragut", ShipKind::Destroyer),
    (15, "Macdonough", ShipKind::Destroyer),
    (16, "Worden", ShipKind::Destroyer),
    (17, "Dale", ShipKind::Destroyer),
    (18, "Phelps", ShipKind::Destroyer),
    (19, "Shokaku", ShipKind::Carrier),
    (20, "Zuikaku", ShipKind::Carrier),
    (21, "Chikuma", ShipKind::Cruiser),
    (22, "Hiei", ShipKind::Battleship),
    (23, "Kirishima", ShipKind::Battleship),
    (24, "Kumano", ShipKind::Cruiser),
    (25, "Suzuya", ShipKind::Cruiser),
    (26, "Nagara", ShipKind::Cruiser),
    (27, "Akigumo", ShipKind::Destroyer),
    (28, "Kazagumo", ShipKind::Destroyer),
    (29, "Makigumo", ShipKind::Destroyer),
    (30, "Yugumo", ShipKind::Destroyer),
    (31, "Akizuki", ShipKind::Destroyer),
    (32, "Hatsukaze", ShipKind::Destroyer),
    (33, "Ryujo", ShipKind::Carrier),
    (34, "Tone", ShipKind::Cruiser),
    (35, "Amatsukaze", ShipKind::Destroyer),
    (36, "Tokitsukaze", ShipKind::Destroyer),
];

#[test]
fn the_forces_and_air_groups_are_as_the_first_two_tables_say() {
    let cx = ctx();
    let scenario = load();
    assert_eq!(scenario.days, 1);
    let truth = scenario.world.umpire();
    // Four task forces: the first table's names and sides.
    assert_eq!(truth.task_forces.len(), 4);
    for (id, name, side) in [
        (1, "Task Force 16", Side::Allied),
        (2, "Task Force 11", Side::Allied),
        (3, "striking force", Side::Japanese),
        (4, "Ryujo", Side::Japanese),
    ] {
        let force = &truth.task_forces[&TaskForceId::new(id)];
        assert!(force.name.contains(name), "{}", force.name);
        assert_eq!(force.side, side);
        assert_eq!(
            force.ships.iter().map(|s| s.raw()).collect::<Vec<_>>(),
            ships_of_force(id).collect::<Vec<_>>()
        );
    }
    // 36 ships, by ID, with the names, kinds and forces of the table.
    assert_eq!(truth.ships.len(), 36);
    for (id, name, kind) in SHIPS {
        let info = &cx.ships[&id];
        assert!(info.name.contains(name), "ship {id} is {}", info.name);
        assert_eq!(info.kind, kind, "ship {id}");
        assert_eq!(info.force, force_of(id), "ship {id}");
        assert_eq!(
            truth.ships[&ShipId::new(id)].side,
            side_of_force(force_of(id))
        );
    }
    // Both American forces start in (7,-5), the Japanese in (9,-19) and
    // (10,-19); Henderson Field is base 1, in (2,-4).
    for (id, hex) in [
        (1, Hex::new(7, -5)),
        (2, Hex::new(7, -5)),
        (3, Hex::new(9, -19)),
        (4, Hex::new(10, -19)),
    ] {
        assert_eq!(truth.task_forces[&TaskForceId::new(id)].position(), hex);
    }
    assert_eq!(
        truth.bases[&sim_core::ids::BaseId::new(1)].hex,
        Hex::new(2, -4)
    );
    assert_eq!(truth.bases.len(), 1);
    // Twenty air groups: counts, types, places.
    assert_eq!(truth.air_groups.len(), 20);
    for spec in &GROUPS {
        let air = &truth.air_groups[&AirGroupId::new(spec.id)];
        assert_eq!(air.ready, spec.ready, "group {}", spec.id);
        assert_eq!(air.aircraft_type.raw(), spec.ty, "group {}", spec.id);
        assert_eq!(air.side, spec.side, "group {}", spec.id);
        assert_eq!(
            air.location,
            match spec.place {
                Place::Ship(ship) => AirGroupLocation::Ship(ShipId::new(ship)),
                Place::Base(base) => AirGroupLocation::Base(sim_core::ids::BaseId::new(base)),
            },
            "group {}",
            spec.id
        );
        assert_eq!(
            air.side,
            match spec.place {
                Place::Ship(ship) => side_of_force(force_of(ship)),
                Place::Base(_) => Side::Allied,
            }
        );
    }
    // The table's totals by force.
    let aboard = |ships: &[u32]| -> u32 {
        GROUPS
            .iter()
            .filter(|s| matches!(s.place, Place::Ship(ship) if ships.contains(&ship)))
            .map(|s| s.ready)
            .sum()
    };
    assert_eq!(aboard(&[1]), 27 + 17 + 16 + 14);
    assert_eq!(aboard(&[11]), 27 + 15 + 15 + 13);
    assert_eq!(aboard(&[19, 20, 21]), 26 + 27 + 18 + 25 + 27 + 18 + 5);
    assert_eq!(aboard(&[33, 34]), 23 + 6 + 5);

    // Missions, shares and arcs, as the orders leave them.
    let mut ran = load();
    ran.run_to_end().expect("runs");
    let truth = ran.world.umpire();
    for spec in &GROUPS {
        let air = &truth.air_groups[&AirGroupId::new(spec.id)];
        assert_eq!(air.mission, spec.mission, "group {}", spec.id);
        if spec.mission == PATROL {
            assert_eq!(air.patrol_share, tenths(spec.share), "group {}", spec.id);
        }
        let arc = air.search.map(|a| (a.from_deg, a.to_deg, a.range_nm));
        match spec.id {
            3 => assert_eq!(arc, Some((290, 90, Fixed::from_int(200)))),
            17 | 20 => assert_eq!(arc, Some((110, 200, Fixed::from_int(253)))),
            _ => assert_eq!(arc, None, "group {}", spec.id),
        }
    }
    // Patrol shares: the Japanese carriers keep 0.8, Ryujo 0.3, the American
    // fighters everything.
    for id in [1, 5, 9] {
        assert_eq!(
            truth.air_groups[&AirGroupId::new(id)].patrol_share,
            Fixed::ONE
        );
    }
}

#[test]
fn the_orders_are_the_ones_the_raids_cite() {
    for run in batch() {
        let mut seen: BTreeMap<EntityRef, Vec<u32>> = BTreeMap::new();
        for event in &run.events {
            if let EventKind::OrderAccepted { order, target } = event.kind {
                assert_eq!(event.causes, vec![Cause::Order(order)]);
                seen.entry(target).or_default().push(order.raw());
            }
        }
        for (group, orders) in ORDERS {
            assert_eq!(
                seen[&EntityRef::AirGroup(AirGroupId::new(group))],
                orders,
                "seed {}, group {group}",
                run.seed
            );
        }
        assert_eq!(seen[&EntityRef::TaskForce(TaskForceId::new(3))], [12, 13]);
        assert_eq!(seen[&EntityRef::TaskForce(TaskForceId::new(4))], [14, 15]);
    }
}

#[test]
fn the_aircraft_table_is_read_from_the_reference_data() {
    let cx = ctx();
    for (id, name, radius, weapon, damage, armour, tanks) in AIRCRAFT {
        let ty = &cx.types[&id];
        assert!(ty.name.contains(name), "{}", ty.name);
        assert_eq!(ty.strike_range_nm, Fixed::from_int(radius), "{name}");
        assert_eq!(ty.attack_weapon, weapon, "{name}");
        assert_eq!(ty.attack_damage, millionths(damage), "{name}");
        assert_eq!(ty.pilot_armour, armour, "{name}");
        assert_eq!(ty.self_sealing_tanks, tanks, "{name}");
    }
    // The floatplane's radius is its search range too.
    assert_eq!(cx.types[&E13A1].search_range_nm, Fixed::from_int(253));
    // The factors the hand calculation uses.
    assert_eq!(cx.air.fighter_hit_chance, tenths(3));
    assert_eq!(cx.air.anti_aircraft_hit_chance, hundredths(5));
    assert_eq!(cx.air.bomb_hit_chance, tenths(2));
    assert_eq!(cx.air.pilot_armour_factor, hundredths(75));
    assert_eq!(cx.air.self_sealing_tanks_factor, hundredths(75));
    assert_eq!(cx.air.strike_contact_max_age_pulses, 0);
    // A shot at an SBD-3 or an F4F-4: both factors. At an A6M2 or a D3A1:
    // none. A fighter's 0.3 x 0.75 x 0.75 = 0.16875, a gun's 0.05 x 0.75 x
    // 0.75 = 0.028125.
    for group in [1, 2, 5, 6, 7] {
        assert_eq!(
            shot_chance(cx, cx.air.fighter_hit_chance, group).0,
            millionths(168_750)
        );
        assert_eq!(
            shot_chance(cx, cx.air.anti_aircraft_hit_chance, group).0,
            millionths(28_125)
        );
    }
    for group in [11, 12, 14, 15, 18] {
        assert_eq!(
            shot_chance(cx, cx.air.fighter_hit_chance, group).0,
            tenths(3)
        );
        assert_eq!(
            shot_chance(cx, cx.air.anti_aircraft_hit_chance, group).0,
            hundredths(5)
        );
    }
    // The anti-aircraft shots: 123 for task force 1, 26 for task force 4.
    let aa = |ships: &[u32]| -> Vec<u32> { ships.iter().map(|s| cx.ships[s].aa).collect() };
    assert_eq!(
        aa(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
        [22, 34, 15, 20, 10, 5, 5, 4, 4, 4]
    );
    assert_eq!(
        aa(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]).iter().sum::<u32>(),
        123
    );
    assert_eq!(aa(&[33, 34, 35, 36]), [13, 11, 1, 1]);
    assert_eq!(aa(&[33, 34, 35, 36]).iter().sum::<u32>(), 26);
    // A hit does 100 x damage / toughness: 17.830188 on Ryujo, 5.260942 on
    // Enterprise.
    assert_eq!(cx.ships[&33].toughness, Fixed::from_int(424));
    assert_eq!(cx.ships[&1].toughness, Fixed::from_int(792));
    assert_eq!(
        Fixed::from_int(100) * cx.types[&SBD3].attack_damage / cx.ships[&33].toughness,
        millionths(17_830_188)
    );
    assert_eq!(
        Fixed::from_int(100) * cx.types[&D3A1].attack_damage / cx.ships[&1].toughness,
        millionths(5_260_942)
    );
    // How many keep station and how many escort: 26 x 0.8 = 20.8, 25 x 0.8,
    // 23 x 0.3 = 6.9, all rounded down.
    for (ready, share, keep) in [(26, 8, 20), (25, 8, 20), (23, 3, 6)] {
        assert_eq!((int(ready) * tenths(share)).trunc(), keep);
    }
}

// ---------------------------------------------------------------------
// Where everyone is.
// ---------------------------------------------------------------------

/// The steaming of one force in a pulse: from, to, distance, speed.
fn steamed(events: &[&Event], force: u32) -> Option<(Hex, Hex, Fixed, Fixed)> {
    let mut found = None;
    for event in events {
        if let EventKind::TaskForceSteamed {
            task_force,
            from,
            to,
            distance_nm,
            speed_kn,
            ..
        } = event.kind
        {
            if task_force == TaskForceId::new(force) {
                assert!(found.is_none(), "one steaming a pulse");
                found = Some((from, to, distance_nm, speed_kn));
            }
        }
    }
    found
}

#[test]
fn positions_distances_and_arrivals_are_as_the_tables_say() {
    let start3 = Hex::new(9, -19);
    let start4 = Hex::new(10, -19);
    let dest3 = Hex::new(8, -11);
    let dest4 = Hex::new(7, -10);
    // Eight hexes and 320 nm, nine hexes and 360 nm.
    assert_eq!(start3.distance(dest3), 8);
    assert_eq!(nm(start3.distance(dest3)), 320);
    assert_eq!(start4.distance(dest4), 9);
    assert_eq!(nm(start4.distance(dest4)), 360);
    let american = Hex::new(7, -5);
    // 06:00: (8,-14) and (7,-13).
    let (a3, a4) = (Hex::new(8, -14), Hex::new(7, -13));
    assert_eq!(nm(american.distance(a3)), 360);
    assert_eq!(american.distance(a3), 9);
    assert_eq!(nm(american.distance(a4)), 320);
    assert_eq!(american.distance(a4), 8);
    assert_eq!(nm(HENDERSON.distance(a4)), 360);
    assert_eq!(HENDERSON.distance(a4), 9);
    // 12:00: (8,-11) and (7,-10).
    assert_eq!(nm(american.distance(dest3)), 240);
    assert_eq!(american.distance(dest3), 6);
    assert_eq!(nm(american.distance(dest4)), 200);
    assert_eq!(american.distance(dest4), 5);
    assert_eq!(nm(HENDERSON.distance(dest4)), 240);
    assert_eq!(HENDERSON.distance(dest4), 6);

    for run in batch() {
        let night = night(run);
        let morning = morning(run);
        let afternoon = afternoon(run);
        let seed = run.seed;
        // The striking force: 204 nm in the night to (8,-14), 4 nm past its
        // centre, then 102 nm to (8,-11): 306 nm in all, 14 nm short of the
        // centre, then the last 14 nm: 320 nm, at 17 knots.
        let n3 = steamed(&night, 3).expect("steamed");
        let m3 = steamed(&morning, 3).expect("steamed");
        let f3 = steamed(&afternoon, 3).expect("steamed");
        assert_eq!(n3, (start3, a3, Fixed::from_int(204), Fixed::from_int(17)));
        assert_eq!(m3, (a3, dest3, Fixed::from_int(102), Fixed::from_int(17)));
        assert_eq!(f3, (dest3, dest3, Fixed::from_int(14), Fixed::from_int(17)));
        assert_eq!(nm(start3.distance(a3)) + 4, 204, "4 nm past its centre");
        assert_eq!(204 + 102, 306, "seed {seed}");
        assert_eq!(nm(start3.distance(dest3)) - 306, 14, "14 nm short");
        assert_eq!(204 + 102 + 14, 320);
        // The Ryujo detachment: 240 nm to (7,-13), at its centre; 120 nm to
        // (7,-10), 360 nm; at 20 knots; nothing in the afternoon.
        let n4 = steamed(&night, 4).expect("steamed");
        let m4 = steamed(&morning, 4).expect("steamed");
        assert_eq!(n4, (start4, a4, Fixed::from_int(240), Fixed::from_int(20)));
        assert_eq!(m4, (a4, dest4, Fixed::from_int(120), Fixed::from_int(20)));
        assert_eq!(nm(start4.distance(a4)), 240, "at its centre");
        assert_eq!(240 + 120, 360);
        assert!(steamed(&afternoon, 4).is_none(), "seed {seed}");
        // Arrivals: the Ryujo detachment at 12:00, hour 18; the striking
        // force at hour 18.823529.
        let arrivals: Vec<(TaskForceId, Hex, Fixed, Pulse)> = run
            .events
            .iter()
            .filter_map(|e| match e.kind {
                EventKind::TaskForceArrived {
                    task_force,
                    hex,
                    elapsed_hours,
                } => Some((task_force, hex, elapsed_hours, e.pulse.expect("a pulse"))),
                _ => None,
            })
            .collect();
        assert_eq!(
            arrivals,
            vec![
                (
                    TaskForceId::new(4),
                    dest4,
                    Fixed::from_int(18),
                    Pulse::Morning
                ),
                (
                    TaskForceId::new(3),
                    dest3,
                    millionths(18_823_529),
                    Pulse::Afternoon
                ),
            ],
            "seed {seed}"
        );
        assert_eq!(
            Fixed::from_int(18) + Fixed::from_int(14) / Fixed::from_int(17),
            millionths(18_823_529)
        );
        assert_eq!(time::pulse_start(1, Pulse::Afternoon), Fixed::from_int(18));
        // The searches fly from where the forces are: (7,-5) all day; the
        // floatplanes from (8,-14) and (7,-13) at 06:00 and from (8,-11) and
        // (7,-10) at 12:00.
        for (events, expected) in [
            (&morning, [(3, american), (17, a3), (20, a4)]),
            (&afternoon, [(3, american), (17, dest3), (20, dest4)]),
        ] {
            let flown: Vec<(u32, Hex)> = events
                .iter()
                .filter_map(|e| match e.kind {
                    EventKind::SearchFlown { air_group, hex, .. } => Some((air_group.raw(), hex)),
                    _ => None,
                })
                .collect();
            assert_eq!(flown, expected, "seed {seed}");
        }
        // The American forces never move; the end positions.
        assert!(run.events.iter().all(
            |e| !matches!(e.kind, EventKind::TaskForceSteamed { task_force, .. }
                if task_force.raw() <= 2)
        ));
        assert_eq!(run.positions[&1], american);
        assert_eq!(run.positions[&2], american);
        assert_eq!(run.positions[&3], dest3);
        assert_eq!(run.positions[&4], dest4);
    }
}

// ---------------------------------------------------------------------
// Fuel.
// ---------------------------------------------------------------------

#[test]
fn the_fuel_figures_are_exact() {
    let cx = ctx();
    // Shokaku cruises at 18 knots, so at 17 she burns at her cruise rate,
    // 4100 t over 9700 nm.
    let shokaku = &cx.ships[&19];
    assert_eq!(shokaku.cruise, Fixed::from_int(18));
    assert_eq!(shokaku.fuel, Fixed::from_int(4100));
    assert_eq!(shokaku.endurance, Fixed::from_int(9700));
    let burn = |distance: i64| shokaku.fuel * distance / shokaku.endurance;
    assert_eq!(burn(204), millionths(86_226_804));
    assert_eq!(burn(102), millionths(43_113_402));
    assert_eq!(burn(14), millionths(5_917_525));
    assert_eq!(
        millionths(86_226_804) + millionths(43_113_402) + millionths(5_917_525),
        millionths(135_257_731)
    );
    // Ryujo cruises at 14 knots, so at 20 her burn is multiplied by (20 /
    // 14) squared: 1.428571 squared is 2.040815. She burns 240 x 2490 /
    // 10000 = 59.76, times that, 121.959104 t in the night, and 60.979552 t
    // in the morning, and has 2307.061344 t left.
    let ryujo = &cx.ships[&33];
    assert_eq!(ryujo.cruise, Fixed::from_int(14));
    assert_eq!(ryujo.fuel, Fixed::from_int(2490));
    assert_eq!(ryujo.endurance, Fixed::from_int(10_000));
    let ratio = Fixed::from_int(20) / ryujo.cruise;
    assert_eq!(ratio, millionths(1_428_571));
    let factor = ratio * ratio;
    assert_eq!(factor, millionths(2_040_815));
    let at_cruise = Fixed::from_int(240) * ryujo.fuel / ryujo.endurance;
    assert_eq!(at_cruise, millionths(59_760_000));
    assert_eq!(at_cruise * factor, millionths(121_959_104));
    assert_eq!(
        at_cruise / Fixed::from_int(2) * factor,
        millionths(60_979_552)
    );
    assert_eq!(
        ryujo.fuel - millionths(121_959_104) - millionths(60_979_552),
        millionths(2_307_061_344)
    );

    for run in batch() {
        let seed = run.seed;
        let mut burns: BTreeMap<u32, Vec<(Pulse, Fixed, Fixed)>> = BTreeMap::new();
        for event in &run.events {
            if let EventKind::FuelBurned {
                ship,
                burned_t,
                remaining_t,
                ..
            } = event.kind
            {
                burns.entry(ship.raw()).or_default().push((
                    event.pulse.expect("a pulse"),
                    burned_t,
                    remaining_t,
                ));
            }
        }
        // Every ship: what it burned is what it had less what it has, none
        // is negative, and the American forces burn nothing.
        for (id, info) in &cx.ships {
            let mut fuel = info.fuel;
            for (_, burned, remaining) in burns.get(id).map_or(&[][..], |v| v.as_slice()) {
                assert!(*burned >= Fixed::ZERO);
                fuel -= *burned;
                assert_eq!(*remaining, fuel, "seed {seed}, ship {id}");
                assert!(*remaining >= Fixed::ZERO);
            }
            assert_eq!(run.ships[id].fuel, fuel, "seed {seed}, ship {id}");
            assert!(run.ships[id].fuel >= Fixed::ZERO);
            if info.force <= 2 {
                assert!(!burns.contains_key(id), "seed {seed}");
            }
        }
        // Shokaku: night, morning, afternoon; Ryujo: night, morning.
        assert_eq!(
            burns[&19],
            vec![
                (
                    Pulse::Night,
                    millionths(86_226_804),
                    millionths(4_100_000_000 - 86_226_804)
                ),
                (
                    Pulse::Morning,
                    millionths(43_113_402),
                    millionths(4_100_000_000 - 86_226_804 - 43_113_402)
                ),
                (
                    Pulse::Afternoon,
                    millionths(5_917_525),
                    millionths(4_100_000_000 - 135_257_731)
                ),
            ],
            "seed {seed}"
        );
        assert_eq!(run.ships[&19].fuel, millionths(3_964_742_269));
        assert_eq!(
            burns[&33],
            vec![
                (
                    Pulse::Night,
                    millionths(121_959_104),
                    millionths(2_490_000_000 - 121_959_104)
                ),
                (
                    Pulse::Morning,
                    millionths(60_979_552),
                    millionths(2_307_061_344)
                ),
            ],
            "seed {seed}"
        );
        assert_eq!(run.ships[&33].fuel, millionths(2_307_061_344));
    }
}

// ---------------------------------------------------------------------
// The morning.
// ---------------------------------------------------------------------

#[test]
fn the_morning_flies_every_search_and_finds_nothing_and_does_nothing() {
    let cx = ctx();
    // No force is within reach of any search: the searches are 200 nm and
    // 253 nm and every enemy is 320 nm or more away.
    let american = Hex::new(7, -5);
    let (a3, a4) = (Hex::new(8, -14), Hex::new(7, -13));
    for (from, range, enemies) in [
        (american, 200, [a3, a4]),
        (a3, 253, [american, american]),
        (a4, 253, [american, american]),
    ] {
        for enemy in enemies {
            assert!(nm(from.distance(enemy)) >= 320);
            assert!(nm(from.distance(enemy)) > range);
        }
    }
    assert_eq!(cx.types[&SBD3].search_range_nm, Fixed::from_int(200));
    for run in batch() {
        let seed = run.seed;
        let events = morning(run);
        let mut flown: BTreeMap<u32, u32> = BTreeMap::new();
        for event in &events {
            match event.kind {
                EventKind::SearchFlown {
                    air_group,
                    aircraft,
                    ..
                } => {
                    flown.insert(air_group.raw(), aircraft);
                }
                // The morning is movement, and the searches.
                EventKind::TaskForceSteamed { .. }
                | EventKind::FuelBurned { .. }
                | EventKind::TaskForceArrived { .. } => {}
                ref other => panic!("seed {seed}: the morning holds {other}"),
            }
        }
        // Every search is flown, with every aircraft of its group.
        assert_eq!(
            flown,
            BTreeMap::from([(3, 16), (17, 5), (20, 5)]),
            "seed {seed}"
        );
        // No attempt, no report, no contact, no strike and no loss: the
        // morning's kinds above allow none of them. The same from the whole
        // run's events before noon.
        for event in &run.events {
            if event.pulse == Some(Pulse::Afternoon) {
                break;
            }
            assert!(
                !matches!(
                    event.kind,
                    EventKind::SearchAttempt { .. }
                        | EventKind::ReportReceived { .. }
                        | EventKind::StrikeLaunched { .. }
                        | EventKind::AirAttack { .. }
                        | EventKind::AircraftLost { .. }
                        | EventKind::ContactAged { .. }
                ),
                "seed {seed}"
            );
        }
        // The aircraft at noon are the aircraft at the start.
        for spec in &GROUPS {
            assert!(run.ready.contains_key(&spec.id));
        }
    }
}

// ---------------------------------------------------------------------
// The afternoon: search.
// ---------------------------------------------------------------------

#[test]
fn the_afternoon_search_attempts_are_the_table_and_the_counts_are_in_their_bands() {
    let american = Hex::new(7, -5);
    let (striking, ryujo) = (Hex::new(8, -11), Hex::new(7, -10));
    // Group 3, from (7,-5), arc 290 to 090, 160 degrees, to 200 nm: the
    // Ryujo detachment at bearing 330 and 200 nm, and the striking force at
    // 240 nm, beyond 200: no attempt.
    assert_eq!(bearing::arc_degrees(290, 90), 160);
    assert!(bearing::in_arc(american, ryujo, 290, 90));
    // Bearing 330 exactly: on the edge of 320-330 and of 330-340 and in no
    // other ten-degree arc.
    assert!(bearing::in_arc(american, ryujo, 320, 330));
    assert!(bearing::in_arc(american, ryujo, 330, 340));
    assert!(!bearing::in_arc(american, ryujo, 340, 350));
    assert!(!bearing::in_arc(american, ryujo, 310, 320));
    assert_eq!(nm(american.distance(ryujo)), 200);
    assert_eq!(nm(american.distance(striking)), 240);
    assert!(nm(american.distance(striking)) > 200);
    // Group 17, from (8,-11), arc 110 to 200, 90 degrees, to 253 nm: about
    // bearing 159, 240 nm.
    assert_eq!(bearing::arc_degrees(110, 200), 90);
    assert!(bearing::in_arc(striking, american, 150, 160));
    assert!(!bearing::in_arc(striking, american, 160, 170));
    assert!(!bearing::in_arc(striking, american, 140, 150));
    assert!(bearing::in_arc(striking, american, 110, 200));
    assert_eq!(nm(striking.distance(american)), 240);
    // Group 20, from (7,-10): bearing 150 exactly, 200 nm.
    assert!(bearing::in_arc(ryujo, american, 150, 160));
    assert!(bearing::in_arc(ryujo, american, 140, 150));
    assert!(!bearing::in_arc(ryujo, american, 160, 170));
    assert!(!bearing::in_arc(ryujo, american, 130, 140));
    assert!(bearing::in_arc(ryujo, american, 110, 200));
    assert_eq!(nm(ryujo.distance(american)), 200);
    // The chances, by the formula and as the table gives them.
    assert_eq!(search_chance(16, 160, 200), millionths(572_957));
    assert_eq!(search_chance(5, 90, 240), millionths(265_258));
    assert_eq!(search_chance(5, 90, 200), millionths(318_309));
    // The chance that none of the four attempts sights, and its complement.
    // 0.734742 x 0.734742 x 0.681691 x 0.681691 = 0.250867 and 0.749133.
    let none = (Fixed::ONE - millionths(265_258))
        * (Fixed::ONE - millionths(265_258))
        * (Fixed::ONE - millionths(318_309))
        * (Fixed::ONE - millionths(318_309));
    assert_eq!(none, millionths(250_867));
    assert_eq!(Fixed::ONE - none, millionths(749_133));

    let mut allied = 0;
    let mut japanese = 0;
    for work in worked() {
        // Exactly the five attempts of the table.
        let pairs: Vec<(u32, u32)> = work.attempts.iter().map(|a| (a.0, a.1)).collect();
        assert_eq!(
            pairs,
            vec![(3, 4), (17, 1), (17, 2), (20, 1), (20, 2)],
            "seed {}",
            work.seed
        );
        allied += i64::from(work.attempts[0].2);
        japanese += i64::from(work.attempts[1..].iter().any(|a| a.2));
    }
    assert_eq!(count(worked().len()), 1000);
    eprintln!("afternoon sightings over 1000 seeds: Allied {allied}, Japanese {japanese}");
    // The two counts, in the bands the table allows (and four standard
    // deviations of the exact chance).
    assert!((511..=635).contains(&allied), "Allied sightings {allied}");
    assert!(
        (695..=803).contains(&japanese),
        "Japanese sightings {japanese}"
    );
    assert!(within_four_sigma(allied, 1000, 572_957));
    assert!(within_four_sigma(japanese, 1000, 749_133));
}

// ---------------------------------------------------------------------
// The afternoon: who launches.
// ---------------------------------------------------------------------

fn launched(work: &Worked, group: u32) -> Option<&Launch> {
    work.launches.iter().find(|l| l.group == group)
}

fn attacked(run: &Run, group: u32) -> bool {
    run.events.iter().any(
        |e| matches!(e.kind, EventKind::AirAttack { air_group, .. } if air_group.raw() == group),
    )
}

fn found_nothing(run: &Run, group: u32) -> bool {
    run.events.iter().any(|e| {
        matches!(e.kind, EventKind::StrikeFoundNothing { air_group, .. } if air_group.raw() == group)
    })
}

#[test]
fn each_group_launches_when_its_own_side_holds_a_contact_within_its_radius_and_not_otherwise() {
    let cx = ctx();
    let american = Hex::new(7, -5);
    let (striking, ryujo) = (Hex::new(8, -11), Hex::new(7, -10));
    // What the radii and the hexes say, before any run: a hex is reached if
    // its distance in nm is within the radius.
    let reaches = |from: Hex, to: Hex, ty: u32| {
        Fixed::from_int(nm(from.distance(to))) <= cx.types[&ty].strike_range_nm
    };
    assert!(reaches(american, ryujo, SBD3));
    assert!(!reaches(american, ryujo, TBF1));
    assert!(!reaches(HENDERSON, ryujo, SBD3));
    assert_eq!(nm(HENDERSON.distance(ryujo)), 240);
    assert!(reaches(striking, american, D3A1));
    assert!(!reaches(striking, american, B5N2));
    // A report of the American forces names (7,-5) or a navigable
    // neighbour. Every one of them is 200 nm or more from the striking
    // force's carriers, beyond the B5N2's 175.
    let mut named = vec![american];
    named.extend(
        american
            .neighbors()
            .into_iter()
            .filter(|h| cx.map.is_navigable(*h)),
    );
    for hex in &named {
        assert!(nm(striking.distance(*hex)) >= 200, "{hex}");
    }

    let mut counts: BTreeMap<&str, i64> = BTreeMap::new();
    let mut bump = |name: &'static str| *counts.entry(name).or_default() += 1;
    for (run, work) in batch().iter().zip(worked()) {
        let seed = run.seed;
        let allied_hex = work.last_report[0];
        let japanese_hex = work.last_report[1];
        // Groups 2, 6 and 7: launch if the contact's hex is within 200 nm of
        // (7,-5).
        for group in [2, 6, 7] {
            let within = allied_hex.is_some_and(|hex| nm(american.distance(hex)) <= 200);
            assert_eq!(launched(work, group).is_some(), within, "seed {seed}");
            if let Some(launch) = launched(work, group) {
                assert_eq!(Some(launch.hex), allied_hex);
                assert_eq!(launch.aircraft, spec_of(group).ready);
            }
        }
        // The true hex is exactly 200 nm; a hex 240 nm away launches nothing.
        if allied_hex == Some(ryujo) {
            assert_eq!(launched(work, 2).expect("launches").distance_nm, 200);
            bump("allied report names the true hex");
        }
        if allied_hex.is_some_and(|hex| nm(american.distance(hex)) == 240) {
            assert!(launched(work, 2).is_none() && launched(work, 6).is_none());
            assert!(launched(work, 7).is_none());
            bump("allied report 240 nm away, nothing launches");
        }
        // Groups 4 and 8: radius 190. They launch only at a contact reported
        // 160 nm away, a hex nearer than the truth, and find nothing there.
        for group in [4, 8] {
            let within = allied_hex.is_some_and(|hex| nm(american.distance(hex)) <= 190);
            assert_eq!(launched(work, group).is_some(), within, "seed {seed}");
            if let Some(launch) = launched(work, group) {
                assert_eq!(launch.distance_nm, 160, "seed {seed}");
                assert_eq!(nm(american.distance(ryujo)) - launch.distance_nm, 40);
                assert!(american.distance(launch.hex) < american.distance(ryujo));
                assert_ne!(launch.hex, ryujo);
                assert!(found_nothing(run, group), "seed {seed}");
                assert!(!attacked(run, group), "seed {seed}");
                bump(if group == 4 {
                    "group 4 launches and finds nothing"
                } else {
                    "group 8 launches and finds nothing"
                });
            }
        }
        // Group 10 at Henderson Field: the true hex is 240 nm away. It
        // launches only at a contact reported 200 nm away or less, and finds
        // nothing there.
        let within = allied_hex.is_some_and(|hex| nm(HENDERSON.distance(hex)) <= 200);
        assert_eq!(launched(work, 10).is_some(), within, "seed {seed}");
        if let Some(launch) = launched(work, 10) {
            assert!(launch.distance_nm <= 200);
            assert_ne!(launch.hex, ryujo);
            assert!(found_nothing(run, 10), "seed {seed}");
            assert!(!attacked(run, 10), "seed {seed}");
            bump("group 10 launches and finds nothing");
        }
        if allied_hex == Some(ryujo) {
            assert!(launched(work, 10).is_none(), "seed {seed}");
        }
        // Groups 12 and 15: launch if the contact's hex is within 252 nm of
        // (8,-11), which the true hex (7,-5) is.
        for group in [12, 15] {
            let within = japanese_hex.is_some_and(|hex| nm(striking.distance(hex)) <= 252);
            assert_eq!(launched(work, group).is_some(), within, "seed {seed}");
            if let Some(launch) = launched(work, group) {
                assert_eq!(Some(launch.hex), japanese_hex);
                assert_eq!(launch.aircraft, 27);
            }
        }
        if japanese_hex == Some(american) {
            assert_eq!(launched(work, 12).expect("launches").distance_nm, 240);
            bump("japanese report names the true hex");
        }
        // Groups 13 and 16, the B5N2s of the striking force, never launch.
        assert!(launched(work, 13).is_none(), "seed {seed}");
        assert!(launched(work, 16).is_none(), "seed {seed}");
        // A strike attacks only if it flew to the hex its target is really
        // in.
        for launch in &work.launches {
            let side = spec_of(launch.group).side;
            let enemy_hex = if side == Side::Allied {
                ryujo
            } else {
                american
            };
            let attack = attacked(run, launch.group);
            if launch.hex == enemy_hex {
                // At the right hex it attacks, unless every one of its
                // bombers was shot down on the way in (each shot is rolled
                // on its own, decision 0033): then it records nothing.
                let left = launch.aircraft - work.lost.get(&launch.group).copied().unwrap_or(0);
                assert_eq!(attack, left > 0, "seed {seed}");
                assert!(!found_nothing(run, launch.group), "seed {seed}");
            } else {
                assert!(!attack, "seed {seed}");
                assert!(found_nothing(run, launch.group), "seed {seed}");
            }
        }
    }
    eprintln!("launch cases over 1000 seeds: {counts:?}");
    for name in [
        "allied report names the true hex",
        "allied report 240 nm away, nothing launches",
        "group 4 launches and finds nothing",
        "group 8 launches and finds nothing",
        "group 10 launches and finds nothing",
        "japanese report names the true hex",
    ] {
        assert!(counts.get(name).copied().unwrap_or(0) > 0, "{name}");
    }
}

// ---------------------------------------------------------------------
// The raids.
// ---------------------------------------------------------------------

#[test]
fn an_american_raid_on_the_ryujo_detachment_is_as_the_table_says() {
    let cx = ctx();
    let mut raids = 0;
    let mut both = 0;
    // Shots and hits over all raids, for the bands: (shots, hits) of the
    // patrol at the bombers and of the guns.
    let mut patrol_total = (0i64, 0i64);
    let mut gun_total = (0i64, 0i64);
    for (run, work) in batch().iter().zip(worked()) {
        let seed = run.seed;
        let found: Vec<&RaidWork> = work
            .raids
            .iter()
            .filter(|r| r.found && r.side == Side::Allied)
            .collect();
        // Enterprise's raid is group 2, Saratoga's groups 6 and 7.
        let mut shapes: Vec<Vec<u32>> = found
            .iter()
            .map(|r| r.strikes.iter().map(|s| s.0).collect())
            .collect();
        shapes.sort();
        assert!(
            shapes.is_empty() || shapes == vec![vec![2], vec![6, 7]],
            "seed {seed}: {shapes:?}"
        );
        both += i64::from(found.len() == 2);
        for raid in found {
            raids += 1;
            let bombers: u32 = raid.strikes.iter().map(|s| s.1).sum();
            assert!(bombers == 17 || bombers == 30);
            assert_eq!(raid.bombers, bombers);
            assert_eq!(raid.target_force, Some(4));
            assert_eq!(raid.target_ship, Some(33));
            // Enterprise's raid leaves Enterprise, Saratoga's Saratoga.
            assert_eq!(
                raid.place,
                if bombers == 17 {
                    Place::Ship(1)
                } else {
                    Place::Ship(11)
                }
            );
            assert_eq!(raid.hex, Hex::new(7, -10));
            // No escort: the target is 200 nm away and the F4F-4's radius is
            // 105 (decision 0030).
            assert!(raid.escorts.is_empty(), "seed {seed}");
            assert_eq!(raid.escort_count, 0);
            assert!(raid.escorts_at_patrol.is_none() && raid.patrol_at_escorts.is_none());
            // Ryujo's fighters: 23 with a share of 0.3 keep 6 overhead; each
            // raid meets those 6, as they stood at the start of the pulse.
            assert_eq!(raid.patrol, 6, "seed {seed}");
            let (shots, patrol_hits) = raid.patrol_at_bombers.expect("a patrol roll");
            assert_eq!(shots, 6);
            // 6 shots at 0.16875 expect 1.0125; each is rolled on its own,
            // so from no hit to six, and the loss is no more than the
            // bombers there are.
            let first_group = raid.strikes[0].0;
            let (chance, _) = shot_chance(cx, cx.air.fighter_hit_chance, first_group);
            assert_eq!(chance, millionths(168_750));
            assert_eq!(int(shots) * chance, millionths(1_012_500));
            assert!(patrol_hits <= shots, "seed {seed}");
            patrol_total.0 += i64::from(shots);
            patrol_total.1 += i64::from(patrol_hits);
            let lost_to_patrol = patrol_hits.min(bombers);
            let (gun_shots, gun_hits) = raid.guns.expect("a gun roll");
            assert_eq!(gun_shots, 26);
            // 26 shots at 0.028125 expect 0.73125.
            let (chance, _) = shot_chance(cx, cx.air.anti_aircraft_hit_chance, first_group);
            assert_eq!(chance, millionths(28_125));
            assert_eq!(int(gun_shots) * chance, millionths(731_250));
            assert!(gun_hits <= gun_shots, "seed {seed}");
            gun_total.0 += i64::from(gun_shots);
            gun_total.1 += i64::from(gun_hits);
            let lost_to_guns = gun_hits.min(bombers - lost_to_patrol);
            // Of 17 (or 30) bombers, those less both losses attack.
            let attacking = bombers - lost_to_patrol - lost_to_guns;
            assert_eq!(raid.bombers_attacking, attacking, "seed {seed}");
        }
        // Every bomber that attacks attacks Ryujo, ship 33.
        let attacks: Vec<&AttackWork> = work
            .attacks
            .iter()
            .filter(|a| spec_of(a.group).side == Side::Allied)
            .collect();
        for attack in &attacks {
            assert_eq!(attack.ship, 33, "seed {seed}");
            assert_eq!(attack.flotation_hits + attack.fire_hits, attack.hits);
            assert!(attack.hits <= attack.shots);
        }
        // Saratoga's losses come off group 6 first: group 7 attacks with all
        // 15 unless the raid lost more than group 6 had, and group 6 with
        // what is left of it.
        if let Some(saratoga) = work
            .raids
            .iter()
            .find(|r| r.found && r.strikes.first().is_some_and(|s| s.0 == 6))
        {
            let lost = 30 - saratoga.bombers_attacking;
            let shots_of = |group: u32| {
                attacks
                    .iter()
                    .find(|a| a.group == group)
                    .map_or(0, |a| a.shots)
            };
            assert_eq!(shots_of(6), 15 - lost.min(15), "seed {seed}");
            assert_eq!(shots_of(7), 15 - lost.saturating_sub(15), "seed {seed}");
        }
    }
    eprintln!(
        "American raids on Ryujo {raids}, runs with both raids {both}, \
         patrol hits {} of {} shots, gun hits {} of {}",
        patrol_total.1, patrol_total.0, gun_total.1, gun_total.0
    );
    // Over the runs the hits of each roll are within four standard
    // deviations of the sum of the expected figures.
    assert!(within_four_sigma(patrol_total.1, patrol_total.0, 168_750));
    assert!(within_four_sigma(gun_total.1, gun_total.0, 28_125));
    assert!(raids > 500, "{raids} raids");
    assert!(both > 200, "{both} runs with both raids");
}

#[test]
fn a_japanese_raid_on_the_american_carriers_is_as_the_table_says() {
    let cx = ctx();
    let mut raids = 0;
    let mut both = 0;
    // (shots, hits) over all raids of each roll, for the bands.
    let mut totals = [(0i64, 0i64); 4];
    for (run, work) in batch().iter().zip(worked()) {
        let seed = run.seed;
        let found: Vec<&RaidWork> = work
            .raids
            .iter()
            .filter(|r| r.found && r.side == Side::Japanese)
            .collect();
        // Two raids, because they leave from two ships: Shokaku's is group
        // 12, Zuikaku's group 15. Neither is ever combined with another.
        let firsts: Vec<u32> = found.iter().map(|r| r.strikes[0].0).collect();
        assert!(
            firsts.is_empty() || firsts == [12, 15] || firsts == [12] || firsts == [15],
            "seed {seed}: {firsts:?}"
        );
        both += i64::from(found.len() == 2);
        for raid in found {
            raids += 1;
            assert_eq!(raid.strikes.len(), 1);
            assert_eq!(raid.strikes[0].1, 27, "27 D3A1");
            assert_eq!(raid.bombers, 27);
            // The target is 240 nm away and the A6M2's radius is 261, so the
            // escorts go: Shokaku's 26 fighters keep 20 and send 6;
            // Zuikaku's 25 keep 20 and send 5.
            let (escort_group, escorts) = match raid.strikes[0].0 {
                12 => (11, 6),
                15 => (14, 5),
                other => panic!("seed {seed}: raid of group {other}"),
            };
            assert_eq!(raid.escorts, vec![(escort_group, escorts)], "seed {seed}");
            assert_eq!(raid.escort_count, escorts);
            // Each raid meets the whole American patrol over (7,-5) as it
            // stood at the start of the pulse: groups 1 and 5, 54 fighters.
            // Henderson Field's fighters are in another hex.
            assert_eq!(raid.patrol, 54, "seed {seed}");
            assert_eq!(raid.target_force, Some(1));
            assert_eq!(raid.target_ship, Some(1));
            assert_eq!(
                raid.place,
                Place::Ship(if escort_group == 11 { 19 } else { 20 })
            );
            assert_eq!(raid.hex, Hex::new(7, -5));
            // Every roll is one draw for each shot (decision 0033): the
            // shots and chance are the table's, the expected figure is their
            // product, and the hits are from none to the shots.
            let group = raid.strikes[0].0;
            let fighter = |target: u32| shot_chance(cx, cx.air.fighter_hit_chance, target).0;
            // Escorts at the patrol: 6 x 0.16875 = 1.0125; 5 x 0.16875 =
            // 0.84375.
            let (shots, hits) = raid.escorts_at_patrol.expect("escorts fire");
            assert_eq!(shots, escorts);
            assert_eq!(fighter(1), millionths(168_750));
            assert_eq!(
                int(shots) * fighter(1),
                if escorts == 6 {
                    millionths(1_012_500)
                } else {
                    millionths(843_750)
                }
            );
            assert!(hits <= shots, "seed {seed}");
            totals[0].0 += i64::from(shots);
            totals[0].1 += i64::from(hits);
            // Patrol at the escorts: 6 x 0.3 = 1.8; 5 x 0.3 = 1.5.
            let (shots, at_escorts_hits) = raid.patrol_at_escorts.expect("patrol fires");
            assert_eq!(shots, escorts);
            assert_eq!(fighter(escort_group), millionths(300_000));
            assert_eq!(
                int(shots) * fighter(escort_group),
                if escorts == 6 {
                    millionths(1_800_000)
                } else {
                    millionths(1_500_000)
                }
            );
            assert!(at_escorts_hits <= shots, "seed {seed}");
            totals[1].0 += i64::from(shots);
            totals[1].1 += i64::from(at_escorts_hits);
            // Patrol at the bombers: 48 x 0.3 = 14.4; 49 x 0.3 = 14.7.
            let (shots, at_bombers_hits) = raid.patrol_at_bombers.expect("patrol fires");
            assert_eq!(shots, 54 - escorts);
            assert_eq!(shots, if escorts == 6 { 48 } else { 49 });
            assert_eq!(fighter(group), millionths(300_000));
            assert_eq!(
                int(shots) * fighter(group),
                if escorts == 6 {
                    millionths(14_400_000)
                } else {
                    millionths(14_700_000)
                }
            );
            assert!(at_bombers_hits <= shots, "seed {seed}");
            totals[2].0 += i64::from(shots);
            totals[2].1 += i64::from(at_bombers_hits);
            // The whole patrol is committed: shots at the escorts and at the
            // bombers add to 54.
            assert_eq!(raid.patrol_at_escorts.expect("fires").0 + shots, 54);
            // Guns at the bombers: 123 x 0.05 = 6.15.
            let (gun_shots, gun_hits) = raid.guns.expect("guns fire");
            assert_eq!(gun_shots, 123);
            let (chance, _) = shot_chance(cx, cx.air.anti_aircraft_hit_chance, group);
            assert_eq!(chance, millionths(50_000));
            assert_eq!(int(gun_shots) * chance, millionths(6_150_000));
            assert!(gun_hits <= gun_shots, "seed {seed}");
            totals[3].0 += i64::from(gun_shots);
            totals[3].1 += i64::from(gun_hits);
            // Bombers that attack: 27 less both losses, each loss no more
            // than the bombers there are.
            let lost_to_patrol = at_bombers_hits.min(27);
            let lost_to_guns = gun_hits.min(27 - lost_to_patrol);
            assert_eq!(
                raid.bombers_attacking,
                27 - lost_to_patrol - lost_to_guns,
                "seed {seed}"
            );
        }
        // Every bomber attacks Enterprise, ship 1.
        for attack in work
            .attacks
            .iter()
            .filter(|a| spec_of(a.group).side == Side::Japanese)
        {
            assert_eq!(attack.flotation_hits + attack.fire_hits, attack.hits);
            assert_eq!(attack.ship, 1, "seed {seed}");
            assert!(attack.shots <= 27, "seed {seed}");
        }
    }
    eprintln!("Japanese raids {raids}, runs with both raids {both}, (shots, hits) {totals:?}");
    assert!(raids > 0 && both > 0);
    // Over the runs the hits of each roll are within four standard
    // deviations of the sum of the expected figures.
    for (total, chance) in totals.iter().zip([168_750, 300_000, 300_000, 50_000]) {
        assert!(within_four_sigma(total.1, total.0, chance), "{total:?}");
    }
}

#[test]
fn the_shot_numbers_and_chances_of_the_tables_are_in_the_events() {
    // The first time each kind of roll appears, its event reads as the tables
    // say, with its figures.
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for run in batch() {
        for event in &run.events {
            let (name, shots, chance, expected) = match event.kind {
                EventKind::AirFight {
                    firer,
                    firing,
                    at,
                    shots,
                    chance,
                    expected,
                    ..
                } => {
                    let name = match (firer.raw(), firing, at) {
                        (18, AirFirer::Patrol, AirTarget::Bombers) => "ryujo patrol at bombers",
                        (11, AirFirer::Escorts, AirTarget::Patrol) => "shokaku escorts at patrol",
                        (14, AirFirer::Escorts, AirTarget::Patrol) => "zuikaku escorts at patrol",
                        (1, AirFirer::Patrol, AirTarget::Escorts) => "patrol at escorts",
                        (1, AirFirer::Patrol, AirTarget::Bombers) => "patrol at bombers",
                        _ => "other",
                    };
                    (name, shots, chance, expected)
                }
                EventKind::AntiAircraftFire {
                    task_force,
                    shots,
                    chance,
                    expected,
                    ..
                } => (
                    if task_force.raw() == 4 {
                        "guns of task force 4"
                    } else {
                        "guns of task force 1"
                    },
                    shots,
                    chance,
                    expected,
                ),
                _ => continue,
            };
            match name {
                "ryujo patrol at bombers" => {
                    assert_eq!(
                        (shots, chance, expected),
                        (int(6), millionths(168_750), millionths(1_012_500))
                    );
                }
                "guns of task force 4" => {
                    assert_eq!(
                        (shots, chance, expected),
                        (int(26), millionths(28_125), millionths(731_250))
                    );
                }
                "shokaku escorts at patrol" => {
                    assert_eq!(
                        (shots, chance, expected),
                        (int(6), millionths(168_750), millionths(1_012_500))
                    );
                }
                "zuikaku escorts at patrol" => {
                    assert_eq!(
                        (shots, chance, expected),
                        (int(5), millionths(168_750), millionths(843_750))
                    );
                }
                "patrol at escorts" => {
                    assert_eq!(chance, tenths(3));
                    assert!(shots == int(6) || shots == int(5));
                    assert_eq!(expected, shots * tenths(3));
                }
                "patrol at bombers" => {
                    assert_eq!(chance, tenths(3));
                    assert!(
                        (shots, expected) == (int(48), tenths(144))
                            || (shots, expected) == (int(49), tenths(147)),
                        "{shots} {expected}"
                    );
                }
                "guns of task force 1" => {
                    assert_eq!(
                        (shots, chance, expected),
                        (int(123), hundredths(5), millionths(6_150_000))
                    );
                }
                _ => {}
            }
            seen.insert(name);
        }
    }
    for name in [
        "ryujo patrol at bombers",
        "guns of task force 4",
        "shokaku escorts at patrol",
        "zuikaku escorts at patrol",
        "patrol at escorts",
        "patrol at bombers",
        "guns of task force 1",
    ] {
        assert!(seen.contains(name), "{name}");
    }
    assert_eq!(tenths(144), millionths(14_400_000));
}

// ---------------------------------------------------------------------
// What cannot happen.
// ---------------------------------------------------------------------

#[test]
fn what_cannot_happen_never_does() {
    for (run, work) in batch().iter().zip(worked()) {
        let seed = run.seed;
        // No ship fires a gun or a torpedo at another ship: no two opposing
        // forces are ever in one hex.
        for event in &run.events {
            assert!(
                !matches!(
                    event.kind,
                    EventKind::Salvo { .. }
                        | EventKind::EngagementBegun { .. }
                        | EventKind::SightingRoll { .. }
                        | EventKind::AlarmRoll { .. }
                        | EventKind::ForceBrokeOff { .. }
                ),
                "seed {seed}: {}",
                event.kind
            );
            if let EventKind::TaskForceSteamed {
                task_force,
                from,
                to,
                ref hexes_entered,
                ..
            } = event.kind
            {
                // Only the Japanese forces move, and none enters (7,-5) or
                // any hex south of row -10.
                assert!(task_force.raw() >= 3, "seed {seed}");
                for hex in std::iter::once(&from)
                    .chain(std::iter::once(&to))
                    .chain(hexes_entered)
                {
                    assert!(hex.r <= -10, "seed {seed}: {hex}");
                    assert_ne!(*hex, Hex::new(7, -5));
                }
            }
        }
        assert_ne!(run.positions[&1], run.positions[&3]);
        assert_ne!(run.positions[&1], run.positions[&4]);
        assert_ne!(run.positions[&2], run.positions[&3]);
        assert_ne!(run.positions[&2], run.positions[&4]);
        // No F4F-4 ever escorts a raid, and no B5N2 of the striking force
        // ever launches. Group 19, Ryujo's B5N2s, launches only at (7,-6),
        // 160 nm away, and always finds nothing; no B5N2 ever attacks.
        for event in &run.events {
            match event.kind {
                EventKind::EscortsSent { air_group, .. } => {
                    assert!(
                        matches!(air_group.raw(), 11 | 14 | 18),
                        "seed {seed}: group {} escorts",
                        air_group.raw()
                    );
                    for fighters in [1, 5, 9] {
                        assert_ne!(air_group.raw(), fighters);
                    }
                }
                EventKind::StrikeLaunched { air_group, .. } => {
                    assert!(!matches!(air_group.raw(), 13 | 16), "seed {seed}");
                }
                _ => {}
            }
        }
        // Group 19 launches only at (7,-6), 160 nm, and finds nothing; Ryujo's
        // 17 escorts are sent exactly when it launches.
        if let Some(l) = launched(work, 19) {
            assert_eq!(l.hex, Hex::new(7, -6), "seed {seed}");
            assert_eq!(l.distance_nm, 160);
            assert!(found_nothing(run, 19) && !attacked(run, 19), "seed {seed}");
            let sent: Vec<u32> = run
                .events
                .iter()
                .filter_map(|e| match e.kind {
                    EventKind::EscortsSent {
                        air_group,
                        with_air_group,
                        aircraft,
                    } if air_group.raw() == 18 => {
                        assert_eq!(with_air_group.raw(), 19);
                        Some(aircraft)
                    }
                    _ => None,
                })
                .collect();
            assert_eq!(sent, [17], "seed {seed}");
        }
        for e in &run.events {
            if let EventKind::AirAttack { air_group, .. } = e.kind {
                assert!(!matches!(air_group.raw(), 13 | 16 | 19), "seed {seed}");
            }
        }
        if launched(work, 19).is_none() {
            assert!(
                !run.events.iter().any(|e| matches!(
                    e.kind,
                    EventKind::EscortsSent { air_group, .. } if air_group.raw() == 18
                )),
                "seed {seed}"
            );
        }
        // No aircraft at Henderson Field is ever lost, and no ship of task
        // force 2 or task force 3 is ever damaged.
        for event in &run.events {
            match event.kind {
                EventKind::AircraftLost { air_group, .. }
                | EventKind::AirGroupLost { air_group, .. } => {
                    assert!(!matches!(air_group.raw(), 9 | 10), "seed {seed}");
                }
                EventKind::ShipDamaged { ship, .. }
                | EventKind::SystemHit { ship, .. }
                | EventKind::ShipSunk { ship, .. } => {
                    assert!(!(11..=32).contains(&ship.raw()), "seed {seed}");
                }
                EventKind::AirAttack { task_force, .. } => {
                    // Saratoga's force is never attacked; neither is the
                    // striking force.
                    assert!(!matches!(task_force.raw(), 2 | 3), "seed {seed}");
                }
                EventKind::SearchAttempt { task_force, .. } => {
                    // The striking force is never sighted.
                    assert_ne!(task_force.raw(), 3, "seed {seed}");
                }
                _ => {}
            }
        }
        assert_eq!(run.ready[&9], 13);
        assert_eq!(run.ready[&10], 11);
        for ship in 11..=32u32 {
            let last = &run.ships[&ship];
            assert_eq!(last.flotation, Fixed::ZERO);
            assert_eq!(last.fire, Fixed::ZERO);
            assert!(!last.sunk);
            assert!(last
                .systems
                .iter()
                .all(|state| *state == SystemState::Working));
        }
        // The striking force is never sighted and never attacked.
        assert!(work.attempts.iter().all(|a| a.1 != 3), "seed {seed}");
        assert!(work.attacks.iter().all(|a| force_of(a.ship) != 3));
    }
}

#[test]
fn losses_come_off_the_lower_group_first() {
    // Saratoga's losses come off group 6 before 7, and the American patrol's
    // off group 1 before 5: the losses are a few aircraft, so groups 7 and 5
    // never lose any.
    let mut six = 0;
    let mut one = 0;
    for run in batch() {
        for event in &run.events {
            if let EventKind::AircraftLost { air_group, .. } = event.kind {
                match air_group.raw() {
                    6 => six += 1,
                    1 => one += 1,
                    7 | 5 => panic!("seed {}: group {} lost aircraft", run.seed, air_group.raw()),
                    _ => {}
                }
            }
        }
        assert_eq!(run.ready[&7], 15, "seed {}", run.seed);
        assert_eq!(run.ready[&5], 27, "seed {}", run.seed);
        assert!(run.ready[&6] <= 15 && run.ready[&1] <= 27);
    }
    assert!(six > 100 && one > 100, "{six} {one}");
}

// ---------------------------------------------------------------------
// Properties.
// ---------------------------------------------------------------------

#[test]
fn every_strike_is_launched_at_a_contact_its_own_side_holds_within_its_types_radius() {
    let cx = ctx();
    let mut launches = 0;
    for (run, work) in batch().iter().zip(worked()) {
        let by_id: BTreeMap<EventId, &Event> = run.events.iter().map(|e| (e.id, e)).collect();
        for launch in &work.launches {
            launches += 1;
            let spec = spec_of(launch.group);
            // It cites a report of its own side, from this pulse's search
            // (there was nothing the pulse before).
            let report = by_id[&launch.report];
            let EventKind::ReportReceived { side, hex, .. } = report.kind else {
                panic!("a launch cites a report");
            };
            assert_eq!(side, spec.side);
            assert_eq!(hex, launch.hex);
            assert_eq!(report.pulse, Some(Pulse::Afternoon));
            assert!(report.id < launch.event);
            // The report was this side's own search.
            let Cause::Event(attempt) = report.causes[0] else {
                panic!("a report cites an event");
            };
            let EventKind::SearchAttempt { air_group, .. } = by_id[&attempt].kind else {
                panic!("an early report is a search's");
            };
            assert_eq!(spec_of(air_group.raw()).side, spec.side);
            // Within the launching type's radius of where the group is.
            let distance = nm(place_hex(spec.place).distance(launch.hex));
            assert_eq!(distance, launch.distance_nm);
            assert!(Fixed::from_int(distance) <= cx.types[&spec.ty].strike_range_nm);
        }
    }
    assert!(launches > 2000, "{launches} launches");
}

#[test]
fn every_escort_sent_is_within_its_types_radius_of_the_raids_target() {
    let cx = ctx();
    let mut sent = 0;
    for work in worked() {
        for raid in &work.raids {
            for &(group, _) in &raid.escorts {
                sent += 1;
                let spec = spec_of(group);
                let distance = nm(place_hex(spec.place).distance(raid.hex));
                assert!(
                    Fixed::from_int(distance) <= cx.types[&spec.ty].strike_range_nm,
                    "seed {}: group {group} escorts {distance} nm",
                    work.seed
                );
            }
        }
    }
    assert!(sent > 1000, "{sent} escort groups sent");
}

#[test]
fn every_groups_aircraft_at_the_end_are_those_at_the_start_less_its_recorded_losses() {
    for run in batch() {
        for spec in &GROUPS {
            let mut left = spec.ready;
            for event in &run.events {
                let lost = match event.kind {
                    EventKind::AircraftLost {
                        air_group,
                        aircraft,
                        remaining,
                    } if air_group.raw() == spec.id => {
                        left = left.checked_sub(aircraft).expect("none goes below zero");
                        assert_eq!(left, remaining, "seed {}", run.seed);
                        continue;
                    }
                    EventKind::AirGroupLost {
                        air_group,
                        aircraft,
                        ..
                    } if air_group.raw() == spec.id => aircraft,
                    _ => continue,
                };
                assert_eq!(
                    lost, left,
                    "seed {}: a sunk ship takes what is left",
                    run.seed
                );
                left = 0;
            }
            assert_eq!(
                run.ready[&spec.id], left,
                "seed {}, group {}",
                run.seed, spec.id
            );
            assert!(run.ready[&spec.id] <= spec.ready);
        }
        // A group aboard a ship that sinks has none.
        for spec in &GROUPS {
            if let Place::Ship(ship) = spec.place {
                if run.ships[&ship].sunk {
                    assert_eq!(run.ready[&spec.id], 0, "seed {}", run.seed);
                }
            }
        }
    }
}

#[test]
fn no_ship_has_negative_fuel_and_no_damage_track_is_above_100() {
    for run in batch() {
        for (id, last) in &run.ships {
            assert!(last.fuel >= Fixed::ZERO, "seed {}, ship {id}", run.seed);
            for damage in [last.flotation, last.fire] {
                assert!(damage >= Fixed::ZERO && damage <= Fixed::from_int(100));
            }
            // A ship is sunk exactly when its flotation reached 100.
            assert_eq!(last.sunk, last.flotation == Fixed::from_int(100));
        }
        for event in &run.events {
            match event.kind {
                EventKind::ShipDamaged { value_after, .. } => {
                    assert!(value_after <= Fixed::from_int(100));
                }
                EventKind::FuelBurned { remaining_t, .. } => {
                    assert!(remaining_t >= Fixed::ZERO);
                }
                _ => {}
            }
        }
    }
}

#[test]
fn every_damage_event_is_caused_by_an_attack_every_attack_by_a_launch_and_every_launch_by_a_report()
{
    let mut damage_events = 0;
    for run in batch() {
        let by_id: BTreeMap<EventId, &Event> = run.events.iter().map(|e| (e.id, e)).collect();
        let cause_of = |event: &Event| -> &Event {
            assert_eq!(event.causes.len(), 1, "seed {}", run.seed);
            let Cause::Event(id) = event.causes[0] else {
                panic!("seed {}: an event cites an event", run.seed);
            };
            by_id[&id]
        };
        for event in &run.events {
            match event.kind {
                EventKind::ShipDamaged { .. } | EventKind::SystemHit { .. } => {
                    assert!(matches!(cause_of(event).kind, EventKind::AirAttack { .. }));
                    damage_events += 1;
                }
                EventKind::ShipSunk { .. } => {
                    assert!(matches!(
                        cause_of(event).kind,
                        EventKind::ShipDamaged { .. }
                    ));
                }
                EventKind::AirGroupLost { .. } => {
                    assert!(matches!(cause_of(event).kind, EventKind::ShipSunk { .. }));
                }
                EventKind::AirAttack { air_group, .. } => {
                    let Cause::Event(launch) = event.causes[0] else {
                        panic!("an attack cites its launch");
                    };
                    assert!(matches!(
                        by_id[&launch].kind,
                        EventKind::StrikeLaunched { air_group: launched, .. } if launched == air_group
                    ));
                    for cause in &event.causes[1..] {
                        let Cause::Event(loss) = cause else {
                            panic!("an attack cites events");
                        };
                        assert!(matches!(
                            by_id[loss].kind,
                            EventKind::AircraftLost { air_group: lost, .. } if lost == air_group
                        ));
                    }
                }
                EventKind::StrikeLaunched { air_group, .. } => {
                    let Cause::Event(report) = event.causes[1] else {
                        panic!("a launch cites a report");
                    };
                    assert!(matches!(
                        by_id[&report].kind,
                        EventKind::ReportReceived { side, .. } if side == spec_of(air_group.raw()).side
                    ));
                }
                EventKind::ReportReceived { side, .. } => {
                    // A report is caused by a search sighting or an attack,
                    // both of its own side's groups.
                    match cause_of(event).kind {
                        EventKind::SearchAttempt {
                            air_group, sighted, ..
                        } => assert!(spec_of(air_group.raw()).side == side && sighted),
                        EventKind::AirAttack { air_group, .. } => {
                            assert_eq!(spec_of(air_group.raw()).side, side);
                        }
                        ref other => panic!("seed {}: a report cites {other}", run.seed),
                    }
                }
                _ => {}
            }
        }
    }
    assert!(damage_events > 3000, "{damage_events} damage events");
}

#[test]
fn a_replay_reproduces_every_daily_hash() {
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
        assert_eq!(hashes[0].len(), 1);
        assert_eq!(hashes[0], hashes[1], "seed {seed}");
        all_hashes.push(hashes[0].clone());
    }
    // The seed matters.
    assert!(all_hashes.windows(2).any(|pair| pair[0] != pair[1]));
    // Every run of the batch has the hash of a fresh run of its seed.
    for run in batch().iter().step_by(25) {
        let mut again = base.with_seed(run.seed).expect("a seed can be set");
        again.run_to_end().expect("the run resolves");
        assert_eq!(run.hashes, again.world.day_hashes(), "seed {}", run.seed);
        let events: Vec<Event> = again.world.events().iter().cloned().collect();
        assert_eq!(run.events, events, "seed {}", run.seed);
    }
    assert_eq!(batch()[76].seed, 77);
    // The file's own seed gives the same hashes as a seeded copy.
    let mut own = load();
    own.run_to_end().expect("the run resolves");
    let mut copy = base.with_seed(1942).expect("a seed can be set");
    copy.run_to_end().expect("the run resolves");
    assert_eq!(own.world.day_hashes(), copy.world.day_hashes());
}

#[test]
fn every_afternoon_of_every_run_is_worked_by_the_rule_from_its_start() {
    // `work_the_run` asserts every event of the afternoon, search to damage
    // and movement, and the end state, for every seed.
    assert_eq!(worked().len(), 1000);
    for (run, work) in batch().iter().zip(worked()) {
        assert_eq!(run.seed, work.seed);
    }
}

// ---------------------------------------------------------------------
// The summary, for the architect. It asserts nothing.
// ---------------------------------------------------------------------

#[test]
fn print_the_summary() {
    let runs = count(batch().len());
    let mut sunk: BTreeMap<u32, i64> = BTreeMap::new();
    let mut attacks = [0i64; 2];
    let mut both = 0i64;
    let mut lost_by_side = [Vec::new(), Vec::new()];
    let mut hits_on_enterprise = 0i64;
    let mut hits_on_ryujo = 0i64;
    let mut other_sunk = 0i64;
    for (run, work) in batch().iter().zip(worked()) {
        for ship in &work.sunk {
            if matches!(ship, 1 | 11 | 19 | 20 | 33) {
                *sunk.entry(*ship).or_default() += 1;
            } else {
                other_sunk += 1;
            }
        }
        for (slot, side) in [Side::Allied, Side::Japanese].into_iter().enumerate() {
            let attacked = work.attacks.iter().any(|a| spec_of(a.group).side == side);
            attacks[slot] += i64::from(attacked);
            let lost: u32 = GROUPS
                .iter()
                .filter(|s| s.side == side)
                .map(|s| s.ready - run.ready[&s.id])
                .sum();
            lost_by_side[slot].push(i64::from(lost));
        }
        both += i64::from(
            work.attacks
                .iter()
                .any(|a| spec_of(a.group).side == Side::Allied)
                && work
                    .attacks
                    .iter()
                    .any(|a| spec_of(a.group).side == Side::Japanese),
        );
        for attack in &work.attacks {
            match attack.ship {
                1 => hits_on_enterprise += i64::from(attack.hits),
                33 => hits_on_ryujo += i64::from(attack.hits),
                _ => {}
            }
        }
    }
    let mean = |values: &[i64]| -> String {
        let total: i64 = values.iter().sum();
        format!(
            "{}.{:02}",
            total / runs,
            (total * 100 / runs).rem_euclid(100)
        )
    };
    eprintln!("SUMMARY over seeds 1 to {SEEDS}");
    for (ship, name) in [
        (1, "Enterprise"),
        (11, "Saratoga"),
        (19, "Shokaku"),
        (20, "Zuikaku"),
        (33, "Ryujo"),
    ] {
        eprintln!(
            "  {name} ends sunk in {} of {runs} runs",
            sunk.get(&ship).copied().unwrap_or(0)
        );
    }
    eprintln!("  other ships sunk: {other_sunk}");
    eprintln!(
        "  runs in which the Allied side attacks {}, the Japanese side {}, both {both}",
        attacks[0], attacks[1]
    );
    for (slot, name) in ["Allied", "Japanese"].into_iter().enumerate() {
        let values = &lost_by_side[slot];
        eprintln!(
            "  {name} aircraft lost: mean {}, range {} to {}",
            mean(values),
            values.iter().min().expect("runs"),
            values.iter().max().expect("runs")
        );
    }
    eprintln!(
        "  hits on Enterprise: mean {}.{:02} a run; hits on Ryujo: mean {}.{:02} a run",
        hits_on_enterprise / runs,
        (hits_on_enterprise * 100 / runs) % 100,
        hits_on_ryujo / runs,
        (hits_on_ryujo * 100 / runs) % 100
    );
    eprintln!("  what happened, for comparison only:");
    eprintln!("    Saratoga's 30 dive bombers and 8 torpedo planes sank Ryujo for no loss or one.");
    eprintln!("    The Japanese first wave, 27 dive bombers and 10 fighters, hit Enterprise with");
    eprintln!("    three bombs and lost 24 aircraft; a second wave of the same size did not find");
    eprintln!("    the American force. No American ship was sunk.");
}
