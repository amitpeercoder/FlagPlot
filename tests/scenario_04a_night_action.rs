//! Scenario 4a: two cruiser forces meet at night. One is alert and one is not.
//!
//! Stage 4a of scenario 4. It proves the surface combat rule on placeholder
//! ships: when a fight starts, who sights whom, surprise, gun and torpedo
//! fire, damage, sinking, breaking off and what each side believes
//! afterwards. The real map (4b) and Savo Island (4c) follow.
//!
//! The data is placeholder data: see `data/scenarios/04a_night_action.toml`.
//! The rule is `docs/decisions/0014-surface-combat.md`.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. The tests below assert these
//! numbers. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass.
//!
//! Inputs, from the data files:
//!
//! - Force A, task force 1, Japanese: cruisers 1, 2 and 3. At battle readiness, so alert
//!   (decision 0021). It starts
//!   at (0,0) and steams east to (10,0) at 16 kn.
//! - Force B, task force 2, Allied: cruisers 4, 5 and 6. At normal readiness with no
//!   contact held, so not alert. It lies
//!   stopped at (5,0).
//! - Both are ordered to engage and to break off at 0.5.
//! - Placeholder cruiser: 8 guns, 3 rounds a gun a minute, 100 rounds a gun,
//!   gun damage 20; 8 torpedo tubes, torpedo damage 200; toughness 400; no
//!   radar.
//! - Combat figures: rounds of 5 minutes, 6 at most. At night a force sights
//!   with chance 0.5, times 0.5 if it is not alert. At night a gun shot hits
//!   with chance 0.03 and a torpedo with 0.04. A surprised target multiplies
//!   the chance by 1.5 and a target lit by fire by 2. A gun hit lands on
//!   flotation or fire with chances 0.75 and 0.25.
//! - Systems (decision 0016, which replaced the system damage track this
//!   scenario was first written for): a hit also knocks a system down a
//!   level with a chance of 0.06 for each point of damage it does, 1 at
//!   most (decision 0033). A cruiser's gun hit does 5 points here, so 0.3; a
//!   torpedo hit does 50, so it always does. A degraded
//!   system halves what it does. Fire control that is out gives a quarter.
//!   Propulsion, a main battery or torpedo mounts that are out give nothing.
//!
//! ## The meeting
//!
//! Force A's position is the nearest hex centre, so it enters (5,0), where
//! force B lies, 20 nm short of that centre: after 180 nm, 11.25 hours into
//! the night pulse. The two meet and fight at that moment (decision 0022).
//! It is dark. By then each cruiser of force A has burned 180 x 0.25 = 45 t.
//!
//! After the fight force A steams on for what is left of the night pulse and
//! for the 12 hours of the morning and afternoon pulses, at the speed of its
//! slowest ship afloat. At 16 kn it had 12 nm of the night left. At another
//! speed that distance is scaled by the new speed over the old. Both forces
//! keep their stragglers, which is the default.
//!
//! | Slowest ship's propulsion | Speed | Rest of the night | Night total | By day | Day total | Ends at | Fuel left |
//! | ------------------------- | ----- | ----------------- | ----------- | ------ | --------- | ------- | --------- |
//! | Working                   | 16 kn | 12 nm             | 192 nm      | 192 nm | 384 nm    | (10,0)  | 1904 t    |
//! | Degraded                  | 8 kn  | 6 nm              | 186 nm      | 96 nm  | 282 nm    | (7,0)   | 1929.5 t  |
//! | Out                       | 0 kn  | 0 nm              | 180 nm      | 0 nm   | 180 nm    | (5,0)   | 1955 t    |
//!
//! Every ship afloat burns 0.25 t per nm, at half speed too, because burn
//! never falls below the cruise rate: 96 t, 70.5 t and 45 t. Force B does
//! not move and burns nothing. The table holds when the propulsion state of
//! force A's slowest ship afloat does not change after the night fight.
//!
//! Corrected by the architect for decision 0022. Before it, the fight was at
//! the end of the night pulse and the night total was always 192 nm.
//!
//! A force that can move is leaving, so it does not fight again at the start
//! of the morning pulse although it is still in (5,0) then.
//!
//! If force A has a ship dead in the water it stays at (5,0), and the two
//! forces fight again at the start of the morning and afternoon pulses. If
//! that ship is sunk in one of those fights, force A steams on straight
//! after it, in the same pulse and for the whole of it, since the fight is
//! at the pulse's start and takes no time: 6 hours at the speed of its
//! slowest ship still afloat, and the same again in the afternoon if the
//! fight was in the morning. (Corrected for decision 0022 after the code was
//! run: before it, a fight came after the pulse's movement, so the force
//! steamed on only in the next pulse.) Those fights are
//! by day: both forces sight in round 1, nobody is surprised, a gun shot
//! hits with chance 0.05 and a torpedo with 0.02, and fire does not light a
//! target. Everything this file asserts about a salvo, damage and the end
//! of a fight holds for those fights too. The counts of how a fight opens,
//! below, are for the night fight only.
//!
//! ## Who sights whom
//!
//! Each round a force that has not sighted the enemy rolls: force A at 0.5,
//! force B at 0.5 x 0.5 = 0.25. The rolls are independent.
//!
//! | In a round where neither has yet sighted | Chance             |
//! | ---------------------------------------- | ------------------ |
//! | Both sight                               | 0.5 x 0.25 = 0.125 |
//! | Only A sights                            | 0.5 x 0.75 = 0.375 |
//! | Only B sights                            | 0.5 x 0.25 = 0.125 |
//! | Neither sights                           | 0.5 x 0.75 = 0.375 |
//!
//! If only one force sights, it fires that round on an enemy that cannot
//! answer: a free round. The enemy has sighted from the next round. With six
//! rounds at most, the chance that neither ever sights is 0.375^6 =
//! 0.002781. The chance that the fight has opened by then is 0.997219, and
//! it opens in one of three ways, in the proportions 0.375 : 0.125 : 0.125
//! out of 0.625.
//!
//! | How the fight opens      | Chance                              |
//! | ------------------------ | ----------------------------------- |
//! | A has a free round       | 0.997219 x 0.375 / 0.625 = 0.598331 |
//! | B has a free round       | 0.997219 x 0.125 / 0.625 = 0.199444 |
//! | Both fire from the start | 0.199444                            |
//! | They pass unseen         | 0.002781                            |
//!
//! ## Detection rates over 1,000 runs
//!
//! The scenario runs with seeds 1 to 1000. Each count below follows a
//! binomial distribution and must fall within four standard deviations of
//! its mean. The seeds are fixed, so the result never varies.
//!
//! | Count of runs where         | p        | Mean   | Std dev | Allowed    |
//! | --------------------------- | -------- | ------ | ------- | ---------- |
//! | A sights in round 1         | 0.5      | 500.00 | 15.81   | 437 to 563 |
//! | B sights in round 1         | 0.25     | 250.00 | 13.69   | 196 to 304 |
//! | A has a free round          | 0.598331 | 598.33 | 15.50   | 537 to 660 |
//! | B has a free round          | 0.199444 | 199.44 | 12.64   | 149 to 249 |
//! | Both fire from the start    | 0.199444 | 199.44 | 12.64   | 149 to 249 |
//! | They pass unseen            | 0.002781 | 2.78   | 1.67    | 0 to 9     |
//!
//! The four ways a fight can open add to 1000 runs exactly.
//!
//! ## A salvo
//!
//! A cruiser with its main battery working fires 8 x 3 x 5 = 120 gun shots in a
//! round. A gun hit does 100 x 20 / 400 = 5 points. A torpedo hit does 100 x
//! 200 / 400 = 50 points of flotation damage: one cripples a cruiser and two
//! sink it. Torpedoes are fired once, in the first round the ship fires.
//!
//! | Salvo                          | Shots | Chance to hit    | Expected | Std dev of the hits |
//! | ------------------------------ | ----- | ---------------- | -------- | ------------------- |
//! | Guns, target not surprised     | 120   | 0.03             | 3.6      | 1.87                |
//! | Guns, target surprised         | 120   | 0.03 x 1.5       | 5.4      | 2.27                |
//! | Guns, target lit               | 120   | 0.03 x 2         | 7.2      | 2.60                |
//! | Torpedoes, target not surprised| 8     | 0.04             | 0.32     | 0.55                |
//! | Torpedoes, target surprised    | 8     | 0.04 x 1.5       | 0.48     | 0.67                |
//! | Torpedoes, target lit          | 8     | 0.04 x 2         | 0.64     | 0.77                |
//! | Guns, main battery degraded    | 60    | 0.03             | 1.8      | 1.32                |
//! | Guns, fire control degraded    | 120   | 0.03 x 0.5       | 1.8      | 1.33                |
//! | Guns, fire control out         | 120   | 0.03 x 0.25      | 0.9      | 0.95                |
//! | Guns, main battery out         | 0     | no salvo         |          |                     |
//! | Torpedoes, mounts degraded     | 4     | 0.04             | 0.16     | 0.39                |
//! | Torpedoes, mounts out          | 0     | no salvo         |          |                     |
//!
//! Each shot is rolled on its own (decision 0033), so a salvo of n shots at
//! chance p scores anything from no hit to n, np on average, with a standard
//! deviation of the square root of np(1 - p). (Before decision 0033 a salvo
//! scored the whole part of the expected figure or one more, and this table
//! gave the two values. The shots, chances and expected figures are as they
//! were.)
//!
//! So in a free round every firing cruiser lands 5.4 gun hits on average on
//! a target that cannot answer and, with chance 1 - 0.94^8 = 0.390431, at
//! least one torpedo hit, any one of which cripples it. When both fire from
//! the start, each lands 3.6 gun hits on average and at least one torpedo
//! hit with chance 1 - 0.96^8 = 0.278610.
//!
//! In every run and every salvo the test asserts, from the salvo's
//! breakdown: gun shots = 120 x the firer's main battery factor (1, 0.5 or
//! 0), or the rounds aboard if that is fewer, and torpedo shots = 8 x its
//! torpedo mount factor; the chance is the
//! base times the factors that apply, and for guns also times the firer's
//! fire control factor (1, 0.5 or 0.25);
//! expected hits = shots x chance; and the hits are no more than the shots.
//! A ship fires 120 shots a round at most and has 800 rounds. One night
//! fight uses 720 at most, so ammunition never limits fire at night. A ship
//! that fights again by day can run short: it then fires the rounds it has
//! left. Each ship ends with 800 less the gun shots it fired, and never
//! below none.
//!
//! Corrected by the architect on review. The first version said ammunition
//! never limits fire, which forgot the second fights by day.
//!
//! ## Hits over 1,000 runs
//!
//! Each shot is one roll. Summed over every gun salvo in the 1,000 runs,
//! with H the hits, E the expected hits, and n and p each salvo's shots and
//! chance, the test asserts (H - E)^2 <= 16 x the sum of np(1 - p). The same
//! holds for torpedo salvoes.
//!
//! Of all gun hits, n, the numbers on flotation and fire must each be within
//! four standard deviations of 0.75n and 0.25n: (k - np)^2 <= 16np(1 - p).
//! Every torpedo hit is on flotation.
//!
//! ## System hits over 1,000 runs
//!
//! Each hit rolls once for a system. The chance of each outcome is the
//! chance of a system hit times that system's share.
//!
//! | Outcome         | Per gun hit         | Per torpedo hit    |
//! | --------------- | ------------------- | ------------------ |
//! | Propulsion      | 0.3 x 0.2 = 0.06    | 1 x 0.8 = 0.8      |
//! | Fire control    | 0.3 x 0.3 = 0.09    | 1 x 0.1 = 0.1      |
//! | Main battery    | 0.3 x 0.35 = 0.105  | 1 x 0.1 = 0.1      |
//! | Torpedo mounts  | 0.3 x 0.15 = 0.045  | none               |
//! | No system       | 0.7                 | none               |
//!
//! (The torpedo column was 0.4, 0.05, 0.05 and 0.5 before decision 0033,
//! when a torpedo hit knocked a system with a flat chance of 0.5. The gun
//! column is unchanged: 5 points x 0.06 is the 0.3 it was.)
//!
//! Each count must be within four standard deviations of its share of the
//! hits, by the same test. A hit on a system that is already out is counted
//! and changes nothing. A system goes from working to degraded to out, one
//! level a hit, and never comes back.
//!
//! ## Damage, sinking and breaking off
//!
//! - Each track is the sum of the hits recorded on it, and stops at 100.
//! - A ship is sunk exactly when its flotation damage reached 100. A sunk
//!   ship is in no task force, fires in the round it sank and never after,
//!   and is never a target after.
//! - Ships are matched in ID order among those afloat at the start of the
//!   round: with three afloat on each side, ship 1 fires at 4, 2 at 5 and 3
//!   at 6, and back again. With two targets afloat the third firer takes the
//!   first target.
//! - A force breaks off at the end of a round when at least half the ships
//!   it began with are sunk or have flotation damage of 50 or more. With
//!   three ships that is two of them. One is not enough: 1/3 is below 0.5.
//! - A fight ends when a force breaks off, when one force has no ship
//!   afloat, when they pass unseen, or after round 6. The test replays each
//!   run's events and asserts the fight ended in the round these rules give,
//!   and for the reason they give.
//!
//! ## What each side believes afterwards
//!
//! - A force that sighted the enemy, or was fired on, sends one sighting
//!   report when the fight ends. It makes one contact at (5,0) or a
//!   neighbouring hex, of a kind the confusion table allows for cruisers. Its
//!   count is the enemy ships afloat at the end of the fight, or one more or
//!   one fewer, and never below one. A force that neither sighted nor was
//!   fired on sends none.
//!
//!   Corrected by the architect on review, before the code was accepted. The
//!   first version said "2 to 4 ships", which forgot that a force can lose a
//!   ship, and "a force that never sighted sends none", which left a force
//!   that was surprised and broke off in the same round with no report of
//!   who had attacked it.
//! - A force that fired claims as sunk every enemy ship that sank plus every
//!   enemy ship left afloat with fire damage of 30 or more. The claim is
//!   never less than the truth.
//! - Force B is alert at the end of every run in which it sighted or was
//!   fired on, and not alert in the runs where the forces passed unseen.
//!
//! # Properties
//!
//! Beside the numbers above, the tests check what must always hold:
//!
//! - Firing first pays. Over the runs where A had a free round, the total
//!   flotation damage to force B is greater than the total to force A. Over
//!   the runs where B had a free round, it is the other way round.
//! - More guns never mean fewer expected hits, and a damaged system never
//!   means more: the expected gun hits against an unsurprised, unlit target
//!   with 4, 8 and 12 guns are 1.8, 3.6 and 5.4. With 8 guns and the main
//!   battery working, degraded and out they are 3.6, 1.8 and none. With fire
//!   control working, degraded and out they are 3.6, 1.8 and 0.9.
//! - A force is never faster than its slowest ship afloat, and a ship with
//!   damaged propulsion is never faster than one without.
//! - An alert force never sights less readily than one that is not: 0.5
//!   against 0.25 at night, and 0.7 against 0.35 with radar.
//! - No ship has negative ammunition, and no track is below 0 or above 100.
//!   Every ship is in exactly one task force or is sunk.
//! - Belief holds nothing the reports did not say.
//! - A replay of any seed reproduces the daily hash.

mod common;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use sim_core::events::{Cause, DamageTrack, EngagementEndReason, Event, EventKind, Weapon};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::TaskForceId;
use sim_core::refdata::ShipKind;
use sim_core::state::{ShipLocation, ShipSystem, ShipSystems, Side, SystemState};
use sim_core::time::Pulse;
use sim_data::Scenario;

const SEEDS: u64 = 1000;
const MEETING: Hex = Hex::new(5, 0);
/// The ships of force A (task force 1) and force B (task force 2).
const SHIPS: [[u32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
/// The side of force A and of force B.
const SIDES: [Side; 2] = [Side::Japanese, Side::Allied];
const MAX_ROUNDS: u32 = 6;
/// Written out from the hand calculation, not read from the data files.
const GUN_BASE_NIGHT: Fixed = Fixed::from_raw(30_000);
const TORPEDO_BASE_NIGHT: Fixed = Fixed::from_raw(40_000);
const GUN_BASE_DAY: Fixed = Fixed::from_raw(50_000);
const TORPEDO_BASE_DAY: Fixed = Fixed::from_raw(20_000);
const SURPRISED: Fixed = Fixed::from_raw(1_500_000);
const LIT: Fixed = Fixed::from_raw(2_000_000);
const GUN_HIT_POINTS: i64 = 5;
const TORPEDO_HIT_POINTS: i64 = 50;
const CRIPPLED: i64 = 50;
const CLAIMED_FIRE: i64 = 30;
/// Nautical miles between two hex centres, and the speed of a cruiser at
/// cruise setting with propulsion working.
const HEX_NM: i64 = 40;
const CRUISE_KN: i64 = 16;
/// Force A enters (5,0), 20 nm short of its centre, after 180 nm of the
/// night pulse's 12 hours at 16 kn (decision 0022).
const NIGHT_ENTRY_NM: i64 = 180;
const NIGHT_HOURS: i64 = 12;
/// Fuel a cruiser burns a mile, in millionths of a ton.
const BURN_PER_NM: i64 = 250_000;

/// Chance of a system hit on a gun hit (5 points x 0.06 = 0.3) and on a
/// torpedo hit (50 points x 0.06, capped at 1), and the share of each system,
/// in millionths: propulsion, fire control, main battery, torpedo mounts.
/// Written out from the hand calculation (chance x share).
const GUN_SYSTEM_P: [i64; 4] = [60_000, 90_000, 105_000, 45_000];
const GUN_NO_SYSTEM_P: i64 = 700_000;
const TORPEDO_SYSTEM_P: [i64; 4] = [800_000, 100_000, 100_000, 0];
const TORPEDO_NO_SYSTEM_P: i64 = 0;

fn fx(value: i64) -> Fixed {
    Fixed::from_int(value)
}

fn millionths(value: i64) -> Fixed {
    Fixed::from_raw(value)
}

/// What a system in a state multiplies, written out from the hand
/// calculation: working 1, degraded 0.5, out 0 (fire control out: 0.25).
fn system_factor(system: ShipSystem, state: SystemState) -> Fixed {
    match (system, state) {
        (_, SystemState::Working) => Fixed::ONE,
        (_, SystemState::Degraded) => millionths(500_000),
        (ShipSystem::FireControl, SystemState::Out) => millionths(250_000),
        (_, SystemState::Out) => Fixed::ZERO,
    }
}

/// A cruiser's speed in knots at cruise setting: 16 x 1, 16 x 0.5 or 16 x 0.
fn ship_speed_kn(propulsion: SystemState) -> i64 {
    match propulsion {
        SystemState::Working => CRUISE_KN,
        SystemState::Degraded => CRUISE_KN / 2,
        SystemState::Out => 0,
    }
}

/// The centre nearest to a position `total` nm east of (0,0) along the row.
fn hex_at(total: i64) -> Hex {
    Hex::new(
        i32::try_from((total + HEX_NM / 2).div_euclid(HEX_NM)).expect("small"),
        0,
    )
}

fn pulse_hours(pulse: Pulse) -> i64 {
    match pulse {
        Pulse::Night => 12,
        Pulse::Morning | Pulse::Afternoon => 6,
    }
}

fn pulse_index(pulse: Pulse) -> usize {
    match pulse {
        Pulse::Night => 0,
        Pulse::Morning => 1,
        Pulse::Afternoon => 2,
    }
}

/// Which force a ship belongs to at the start: 0 for A, 1 for B.
fn force_of_ship(ship: u32) -> usize {
    assert!(
        (1..=6).contains(&ship),
        "ship {ship} is not in the scenario"
    );
    usize::from(ship > 3)
}

/// Index of a task force: 0 for task force 1, 1 for task force 2.
fn force_index(task_force: TaskForceId) -> usize {
    assert!((1..=2).contains(&task_force.raw()));
    usize::try_from(task_force.raw() - 1).expect("small")
}

fn scenario_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/04a_night_action.toml")
}

fn load() -> Scenario {
    common::load(&scenario_path()).expect("scenario 4a loads")
}

/// Whether the class of a ship has a system. All six ships are class 1, the
/// placeholder cruiser, which has all four.
fn class_has(ship: u32, system: ShipSystem) -> bool {
    static TABLE: OnceLock<BTreeMap<(u32, ShipSystem), bool>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let scenario = load();
        let truth = scenario.world.umpire();
        let classes = &scenario.world.ref_data().ship_classes;
        let mut table = BTreeMap::new();
        for (id, ship) in &truth.ships {
            for system in ShipSystem::ALL {
                table.insert((id.raw(), system), classes[&ship.class].has_system(system));
            }
        }
        table
    })[&(ship, system)]
}

// ---------------------------------------------------------------------
// What a run leaves behind: its events, and its final state.
// ---------------------------------------------------------------------

#[derive(Clone, Debug)]
struct ShipEnd {
    side: Side,
    location: ShipLocation,
    fuel: Fixed,
    flotation: Fixed,
    fire: Fixed,
    rounds: Fixed,
    torpedoes_ready: bool,
    systems: ShipSystems,
}

#[derive(Clone, Debug)]
struct ForceEnd {
    position: Hex,
    ships: BTreeSet<u32>,
    alert: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ContactEnd {
    id: u32,
    hex: Hex,
    count: u32,
    kind: ShipKind,
    age: u32,
    last_event: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ClaimEnd {
    hex: Hex,
    day: u32,
    pulse: Pulse,
    claimed_sunk: u32,
    event: u32,
}

#[derive(Clone, Debug)]
struct RunData {
    seed: u64,
    events: Vec<Event>,
    ships: BTreeMap<u32, ShipEnd>,
    forces: BTreeMap<u32, ForceEnd>,
    contacts: BTreeMap<Side, Vec<ContactEnd>>,
    claims: BTreeMap<Side, Vec<ClaimEnd>>,
}

fn snapshot(seed: u64, scenario: &Scenario) -> RunData {
    let world = &scenario.world;
    let truth = world.umpire();
    let mut contacts = BTreeMap::new();
    let mut claims = BTreeMap::new();
    for side in Side::ALL {
        let view = world.side_view(side);
        contacts.insert(
            side,
            view.contacts()
                .map(|contact| ContactEnd {
                    id: contact.id.umpire_raw(common::umpire_key()),
                    hex: contact.estimated_hex,
                    count: contact.estimated_ship_count,
                    kind: contact.estimated_kind,
                    age: contact.age_pulses,
                    last_event: contact.last_report_event.umpire_raw(common::umpire_key()),
                })
                .collect::<Vec<_>>(),
        );
        claims.insert(
            side,
            view.claims()
                .iter()
                .map(|claim| ClaimEnd {
                    hex: claim.hex,
                    day: claim.day,
                    pulse: claim.pulse,
                    claimed_sunk: claim.claimed_sunk,
                    event: claim.received_event.umpire_raw(common::umpire_key()),
                })
                .collect::<Vec<_>>(),
        );
    }
    RunData {
        seed,
        events: world.events().iter().cloned().collect(),
        ships: truth
            .ships
            .iter()
            .map(|(id, ship)| {
                (
                    id.raw(),
                    ShipEnd {
                        side: ship.side,
                        location: ship.location,
                        fuel: ship.fuel_t,
                        flotation: ship.flotation_damage,
                        fire: ship.fire_damage,
                        rounds: ship.gun_rounds,
                        torpedoes_ready: ship.torpedoes_ready,
                        systems: ship.systems,
                    },
                )
            })
            .collect(),
        forces: truth
            .task_forces
            .iter()
            .map(|(id, force)| {
                (
                    id.raw(),
                    ForceEnd {
                        position: force.position(),
                        ships: force.ships.iter().map(|ship| ship.raw()).collect(),
                        alert: force.is_alert(),
                    },
                )
            })
            .collect(),
        contacts,
        claims,
    }
}

struct Batch {
    runs: Vec<RunData>,
    elapsed: Duration,
}

/// The wall clock is banned from the simulation (`clippy.toml`). The test is
/// not the simulation: it only times how long the 1,000 runs take, and the
/// time never feeds back into any result.
#[allow(clippy::disallowed_methods)]
fn start_timer() -> Instant {
    Instant::now()
}

/// Seeds 1 to 1000, run once and shared by every test below. Breakdowns are
/// recorded (the default of `SimConfig`).
fn batch() -> &'static Batch {
    static BATCH: OnceLock<Batch> = OnceLock::new();
    BATCH.get_or_init(|| {
        let base = load();
        let timer = start_timer();
        let runs = (1..=SEEDS)
            .map(|seed| {
                let mut run = base.with_seed(seed).expect("a seed can be set");
                run.run_to_end().expect("the run resolves");
                snapshot(seed, &run)
            })
            .collect();
        Batch {
            runs,
            elapsed: timer.elapsed(),
        }
    })
}

// ---------------------------------------------------------------------
// The events of a run, sorted out by kind.
// ---------------------------------------------------------------------

#[derive(Clone, Debug)]
struct Roll {
    round: u32,
    force: usize,
    chance: Fixed,
    sighted: bool,
}

#[derive(Clone, Debug)]
struct SalvoRec {
    event: u32,
    round: u32,
    firer: u32,
    target: u32,
    weapon: Weapon,
    shots: Fixed,
    chance: Fixed,
    expected: Fixed,
    hits: u32,
    factors: Vec<(&'static str, Fixed)>,
    result: Fixed,
}

impl SalvoRec {
    fn factor(&self, name: &str) -> Fixed {
        self.factors
            .iter()
            .find(|(n, _)| *n == name)
            .unwrap_or_else(|| panic!("salvo #{} has no factor {name}", self.event))
            .1
    }
}

#[derive(Clone, Debug)]
struct DamageRec {
    round: u32,
    ship: u32,
    track: DamageTrack,
    hits: u32,
    points: Fixed,
    after: Fixed,
    cause: u32,
}

#[derive(Clone, Debug)]
struct SystemHitRec {
    event: u32,
    round: u32,
    ship: u32,
    system: ShipSystem,
    before: SystemState,
    after: SystemState,
    cause: u32,
}

#[derive(Clone, Debug)]
struct SunkRec {
    round: u32,
    ship: u32,
    force: usize,
    cause: u32,
}

#[derive(Clone, Debug)]
struct ReportRec {
    event: u32,
    side: Side,
    contact: u32,
    hex: Hex,
    count: u32,
    kind: ShipKind,
    created: bool,
    cause: u32,
}

#[derive(Clone, Debug)]
struct ClaimRec {
    event: u32,
    side: Side,
    hex: Hex,
    claimed_sunk: u32,
    cause: u32,
}

#[derive(Clone, Debug)]
struct Ended {
    event: u32,
    round: u32,
    hex: Hex,
    reason: EngagementEndReason,
}

/// One fight: everything between its `EngagementBegun` and the reports and
/// claims that follow its `EngagementEnded`.
#[derive(Clone, Debug)]
struct FightRec {
    pulse: Pulse,
    begun_event: u32,
    /// The two forces and the hex the `EngagementBegun` event names.
    begun_with: (TaskForceId, TaskForceId, Hex),
    /// The `elapsed_hours` the `EngagementBegun` event carries.
    elapsed_hours: Fixed,
    ended: Ended,
    rolls: Vec<Roll>,
    surprised: Vec<(u32, usize)>,
    salvos: Vec<SalvoRec>,
    damages: Vec<DamageRec>,
    system_hits: Vec<SystemHitRec>,
    sunk: Vec<SunkRec>,
    alerts: Vec<(u32, usize)>,
    broke_off: Vec<(u32, usize, Fixed)>,
    passed_unseen: Vec<u32>,
    slipped: usize,
    /// Reports and claims after the fight, in order: (is a claim, side).
    after_fight: Vec<(bool, Side)>,
    reports: Vec<ReportRec>,
    claims: Vec<ClaimRec>,
}

impl FightRec {
    fn dark(&self) -> bool {
        self.pulse == Pulse::Night
    }
}

/// One `TaskForceSteamed` event.
#[derive(Clone, Debug)]
struct StepRec {
    event: u32,
    pulse: Pulse,
    task_force: u32,
    from: Hex,
    to: Hex,
    distance: Fixed,
    speed: Fixed,
}

/// One `FuelBurned` event.
#[derive(Clone, Debug)]
struct BurnRec {
    event: u32,
    ship: u32,
    task_force: u32,
    burned: Fixed,
    remaining: Fixed,
}

/// A run's events sorted out: its fights in order, its steps and burns.
#[derive(Clone, Debug)]
struct Parsed {
    seed: u64,
    fights: Vec<FightRec>,
    steps: Vec<StepRec>,
    burns: Vec<BurnRec>,
    /// The event at which each sunk ship sank.
    sunk_at: BTreeMap<u32, u32>,
}

impl Parsed {
    fn system_hits(&self) -> impl Iterator<Item = &SystemHitRec> {
        self.fights.iter().flat_map(|f| f.system_hits.iter())
    }

    /// A ship's system as the events leave it just before `event`: every
    /// state change is a `SystemHit`, and the ship starts with all working.
    fn state_before(&self, ship: u32, system: ShipSystem, event: u32) -> SystemState {
        self.system_hits()
            .filter(|hit| hit.ship == ship && hit.system == system && hit.event < event)
            .last()
            .map_or(SystemState::Working, |hit| hit.after)
    }

    /// Whether a ship is afloat just before `event`.
    fn afloat_before(&self, ship: u32, event: u32) -> bool {
        self.sunk_at.get(&ship).is_none_or(|sunk| *sunk > event)
    }
}

fn cause_event(event: &Event) -> u32 {
    let ids: Vec<u32> = event
        .causes
        .iter()
        .filter_map(|cause| match cause {
            Cause::Event(id) => Some(id.umpire_raw(common::umpire_key())),
            Cause::Order(_) => None,
        })
        .collect();
    assert_eq!(
        ids.len(),
        1,
        "#{} has one event cause",
        event.id.umpire_raw(common::umpire_key())
    );
    ids[0]
}

/// The fight in progress, for an event that belongs inside it.
fn open_fight<'a>(parsed: &'a mut Parsed, open: bool, event: &Event) -> &'a mut FightRec {
    let (seed, id) = (parsed.seed, event.id.umpire_raw(common::umpire_key()));
    assert!(open, "seed {seed}: fight event #{id} outside a fight");
    let fight = parsed.fights.last_mut().expect("a fight is open");
    assert_eq!(
        (event.day, event.pulse),
        (1, Some(fight.pulse)),
        "seed {seed}: fight event #{id}"
    );
    fight
}

/// The fight that has just ended, for a report or claim that follows it.
fn closed_fight<'a>(parsed: &'a mut Parsed, open: bool, event: &Event) -> &'a mut FightRec {
    let (seed, id) = (parsed.seed, event.id.umpire_raw(common::umpire_key()));
    assert!(!open, "seed {seed}: #{id} inside a fight");
    let fight = parsed
        .fights
        .last_mut()
        .unwrap_or_else(|| panic!("seed {seed}: #{id} before any fight"));
    assert_eq!(
        (event.day, event.pulse),
        (1, Some(fight.pulse)),
        "seed {seed}: #{id} follows a fight in its pulse"
    );
    fight
}

/// Sorts a run's events. Asserts the shape every fight must have: each
/// `EngagementBegun` is followed by one `EngagementEnded`, every fight event
/// lies between the two and in the fight's own pulse, and every report and
/// claim follows the end of a fight.
fn parse(seed: u64, events: &[Event]) -> Parsed {
    let mut parsed = Parsed {
        seed,
        fights: Vec::new(),
        steps: Vec::new(),
        burns: Vec::new(),
        sunk_at: BTreeMap::new(),
    };
    let mut open = false;
    for (index, event) in events.iter().enumerate() {
        let id = event.id.umpire_raw(common::umpire_key());
        assert_eq!(usize::try_from(id).expect("small"), index + 1);
        match &event.kind {
            EventKind::EngagementBegun {
                a,
                b,
                hex,
                elapsed_hours,
            } => {
                assert!(!open, "seed {seed}: a fight inside a fight");
                assert_eq!(event.day, 1);
                open = true;
                parsed.fights.push(FightRec {
                    pulse: event.pulse.expect("a fight has a pulse"),
                    begun_event: id,
                    begun_with: (*a, *b, *hex),
                    elapsed_hours: *elapsed_hours,
                    ended: Ended {
                        event: 0,
                        round: 0,
                        hex: MEETING,
                        reason: EngagementEndReason::MaxRounds,
                    },
                    rolls: Vec::new(),
                    surprised: Vec::new(),
                    salvos: Vec::new(),
                    damages: Vec::new(),
                    system_hits: Vec::new(),
                    sunk: Vec::new(),
                    alerts: Vec::new(),
                    broke_off: Vec::new(),
                    passed_unseen: Vec::new(),
                    slipped: 0,
                    after_fight: Vec::new(),
                    reports: Vec::new(),
                    claims: Vec::new(),
                });
            }
            EventKind::SightingRoll {
                round,
                task_force,
                enemy,
                chance,
                sighted,
                ..
            } => {
                let fight = open_fight(&mut parsed, open, event);
                assert_eq!(force_index(*task_force) + force_index(*enemy), 1);
                let breakdown = event.breakdown.as_ref().expect("a breakdown is recorded");
                assert_eq!(breakdown.result, *chance);
                fight.rolls.push(Roll {
                    round: *round,
                    force: force_index(*task_force),
                    chance: *chance,
                    sighted: *sighted,
                });
            }
            EventKind::ForceSurprised {
                round, task_force, ..
            } => {
                open_fight(&mut parsed, open, event)
                    .surprised
                    .push((*round, force_index(*task_force)));
            }
            EventKind::Salvo {
                round,
                firer,
                target,
                weapon,
                shots,
                chance,
                expected,
                hits,
            } => {
                let fight = open_fight(&mut parsed, open, event);
                let breakdown = event.breakdown.as_ref().expect("a breakdown is recorded");
                fight.salvos.push(SalvoRec {
                    event: id,
                    round: *round,
                    firer: firer.raw(),
                    target: target.raw(),
                    weapon: *weapon,
                    shots: *shots,
                    chance: *chance,
                    expected: *expected,
                    hits: *hits,
                    factors: breakdown
                        .factors
                        .iter()
                        .map(|factor| (factor.name, factor.value))
                        .collect(),
                    result: breakdown.result,
                });
            }
            EventKind::ShipDamaged {
                round,
                ship,
                track,
                hits,
                points,
                value_after,
            } => {
                open_fight(&mut parsed, open, event)
                    .damages
                    .push(DamageRec {
                        round: *round,
                        ship: ship.raw(),
                        track: *track,
                        hits: *hits,
                        points: *points,
                        after: *value_after,
                        cause: cause_event(event),
                    });
            }
            EventKind::SystemHit {
                round,
                ship,
                system,
                before,
                after,
            } => {
                open_fight(&mut parsed, open, event)
                    .system_hits
                    .push(SystemHitRec {
                        event: id,
                        round: *round,
                        ship: ship.raw(),
                        system: *system,
                        before: *before,
                        after: *after,
                        cause: cause_event(event),
                    });
            }
            EventKind::ShipSunk {
                round,
                ship,
                task_force,
            } => {
                open_fight(&mut parsed, open, event).sunk.push(SunkRec {
                    round: *round,
                    ship: ship.raw(),
                    force: force_index(*task_force),
                    cause: cause_event(event),
                });
                assert!(parsed.sunk_at.insert(ship.raw(), id).is_none());
            }
            EventKind::ForceBecameAlert { round, task_force } => {
                open_fight(&mut parsed, open, event)
                    .alerts
                    .push((*round, force_index(*task_force)));
            }
            EventKind::ForceBrokeOff {
                round,
                task_force,
                share,
            } => {
                open_fight(&mut parsed, open, event).broke_off.push((
                    *round,
                    force_index(*task_force),
                    *share,
                ));
            }
            EventKind::ForceSlippedAway { .. } => {
                open_fight(&mut parsed, open, event).slipped += 1;
            }
            EventKind::ForcesPassedUnseen { round, .. } => {
                open_fight(&mut parsed, open, event)
                    .passed_unseen
                    .push(*round);
            }
            EventKind::EngagementEnded {
                round,
                a,
                b,
                hex,
                reason,
            } => {
                let fight = open_fight(&mut parsed, open, event);
                assert_eq!((*a, *b, *hex), fight.begun_with, "seed {seed}");
                fight.ended = Ended {
                    event: id,
                    round: *round,
                    hex: *hex,
                    reason: *reason,
                };
                open = false;
            }
            EventKind::ReportReceived {
                side,
                contact,
                hex,
                ship_count,
                kind,
                created,
            } => {
                let fight = closed_fight(&mut parsed, open, event);
                fight.after_fight.push((false, *side));
                fight.reports.push(ReportRec {
                    event: id,
                    side: *side,
                    contact: contact.umpire_raw(common::umpire_key()),
                    hex: *hex,
                    count: *ship_count,
                    kind: *kind,
                    created: *created,
                    cause: cause_event(event),
                });
                assert_eq!(
                    fight.reports.last().map(|r| r.cause),
                    Some(fight.ended.event)
                );
            }
            EventKind::ClaimReceived {
                side,
                hex,
                claimed_sunk,
                ..
            } => {
                let fight = closed_fight(&mut parsed, open, event);
                fight.after_fight.push((true, *side));
                fight.claims.push(ClaimRec {
                    event: id,
                    side: *side,
                    hex: *hex,
                    claimed_sunk: *claimed_sunk,
                    cause: cause_event(event),
                });
                assert_eq!(
                    fight.claims.last().map(|c| c.cause),
                    Some(fight.ended.event)
                );
            }
            EventKind::TaskForceSteamed {
                task_force,
                from,
                to,
                distance_nm,
                speed_kn,
                ..
            } => {
                assert!(!open, "seed {seed}: movement inside a fight");
                parsed.steps.push(StepRec {
                    event: id,
                    pulse: event.pulse.expect("a step has a pulse"),
                    task_force: task_force.raw(),
                    from: *from,
                    to: *to,
                    distance: *distance_nm,
                    speed: *speed_kn,
                });
            }
            EventKind::FuelBurned {
                ship,
                task_force,
                burned_t,
                remaining_t,
            } => {
                parsed.burns.push(BurnRec {
                    event: id,
                    ship: ship.raw(),
                    task_force: task_force.raw(),
                    burned: *burned_t,
                    remaining: *remaining_t,
                });
            }
            _ => {}
        }
    }
    assert!(!open, "seed {seed}: the last fight never ended");
    assert!(!parsed.fights.is_empty(), "seed {seed}: no fight");
    parsed
}

/// The model's tracks: damage per ship, built only from the `points` of the
/// `ShipDamaged` events, capped at 100, and each ship's systems, built only
/// from the `SystemHit` events.
#[derive(Clone, Debug)]
struct Tracks {
    flotation: BTreeMap<u32, Fixed>,
    fire: BTreeMap<u32, Fixed>,
    systems: BTreeMap<u32, ShipSystems>,
}

impl Tracks {
    /// The start of the scenario: no damage, every system working.
    fn new() -> Tracks {
        let zero = || (1..=6).map(|ship| (ship, Fixed::ZERO)).collect();
        Tracks {
            flotation: zero(),
            fire: zero(),
            systems: (1..=6)
                .map(|ship| (ship, ShipSystems::ALL_WORKING))
                .collect(),
        }
    }

    fn track(&mut self, track: DamageTrack) -> &mut BTreeMap<u32, Fixed> {
        match track {
            DamageTrack::Flotation => &mut self.flotation,
            DamageTrack::Fire => &mut self.fire,
        }
    }

    fn afloat(&self, force: usize) -> Vec<u32> {
        SHIPS[force]
            .iter()
            .copied()
            .filter(|ship| self.flotation[ship] < fx(100))
            .collect()
    }

    /// The worst propulsion among the ships of a force afloat, if any is.
    fn slowest_propulsion(&self, force: usize) -> Option<SystemState> {
        self.afloat(force)
            .iter()
            .map(|ship| self.systems[ship].propulsion)
            .max()
    }
}

/// One round as the model sees it.
#[derive(Clone, Debug)]
struct RoundState {
    round: u32,
    /// Tracks at the start of the round, and at its end.
    start: Tracks,
    end: Tracks,
    /// Ships afloat at the start of the round, in ID order.
    afloat: [Vec<u32>; 2],
    /// The force rolls this round: it had not sighted the enemy.
    rolls: [bool; 2],
    /// The force has sighted the enemy after the sighting step.
    sighted: [bool; 2],
    /// The force is fired on this round.
    fired_on: [bool; 2],
    /// The force is surprised: it has not sighted, and the enemy fires.
    surprised: [bool; 2],
}

struct Walk {
    rounds: Vec<RoundState>,
    /// The model's value after each `ShipDamaged` event, in event order.
    after: Vec<Fixed>,
    /// The round in which each ship's flotation reached 100.
    sunk_round: BTreeMap<u32, u32>,
    /// The tracks and systems when the fight ended.
    end: Tracks,
}

impl Walk {
    fn last(&self) -> &RoundState {
        self.rounds.last().expect("a fight has a round")
    }
}

/// Replays the damage of a fight round by round from the tracks it starts
/// with, with the sighting and surprise rules written out again here. A
/// system hit must find the system as the model has it, and step it down one
/// level unless it is out or the ship's class lacks it.
fn walk(seed: u64, f: &FightRec, start: &Tracks) -> Walk {
    let mut tracks = start.clone();
    let mut after = Vec::new();
    let mut rounds = Vec::new();
    let mut sunk_round = BTreeMap::new();
    let mut sighted_before = [false; 2];
    let mut fired_on_before = [false; 2];
    let (mut next_damage, mut next_hit) = (0, 0);
    for round in 1..=f.ended.round {
        let round_start = tracks.clone();
        let afloat = [round_start.afloat(0), round_start.afloat(1)];
        let rolls = [0, 1].map(|force| !sighted_before[force] && !fired_on_before[force]);
        let success = |force: usize| {
            f.rolls
                .iter()
                .any(|r| r.round == round && r.force == force && r.sighted)
        };
        let sighted =
            [0, 1].map(|force| sighted_before[force] || fired_on_before[force] || success(force));
        let salvos: Vec<&SalvoRec> = f.salvos.iter().filter(|s| s.round == round).collect();
        let fired_on = [0, 1].map(|force| salvos.iter().any(|s| force_of_ship(s.target) == force));
        let fired = [0, 1].map(|force| salvos.iter().any(|s| force_of_ship(s.firer) == force));
        let surprised = [!sighted[0] && fired[1], !sighted[1] && fired[0]];
        while next_damage < f.damages.len() && f.damages[next_damage].round == round {
            let damage = &f.damages[next_damage];
            let track = tracks.track(damage.track);
            let value = std::cmp::min(track[&damage.ship] + damage.points, fx(100));
            track.insert(damage.ship, value);
            after.push(value);
            if damage.track == DamageTrack::Flotation && value >= fx(100) {
                sunk_round.entry(damage.ship).or_insert(round);
            }
            next_damage += 1;
        }
        while next_hit < f.system_hits.len() && f.system_hits[next_hit].round == round {
            let hit = &f.system_hits[next_hit];
            let systems = tracks.systems.get_mut(&hit.ship).expect("a ship");
            let current = systems.get(hit.system);
            assert_eq!(hit.before, current, "seed {seed}: #{} before", hit.event);
            let expected = if class_has(hit.ship, hit.system) {
                current.step_down()
            } else {
                current
            };
            assert_eq!(hit.after, expected, "seed {seed}: #{} after", hit.event);
            systems.set(hit.system, hit.after);
            next_hit += 1;
        }
        rounds.push(RoundState {
            round,
            start: round_start,
            end: tracks.clone(),
            afloat,
            rolls,
            sighted,
            fired_on,
            surprised,
        });
        sighted_before = sighted;
        fired_on_before = fired_on;
    }
    assert_eq!(
        next_damage,
        f.damages.len(),
        "seed {seed}: damage after the fight ended"
    );
    assert_eq!(
        next_hit,
        f.system_hits.len(),
        "seed {seed}: system hits after the fight ended"
    );
    Walk {
        rounds,
        after,
        sunk_round,
        end: tracks,
    }
}

/// A run's events and the model's walk of each of its fights, each starting
/// from the tracks the one before left.
struct Analysed {
    p: Parsed,
    walks: Vec<Walk>,
}

fn analyse(p: Parsed) -> Analysed {
    let mut tracks = Tracks::new();
    let mut walks = Vec::new();
    for fight in &p.fights {
        let w = walk(p.seed, fight, &tracks);
        tracks = w.end.clone();
        walks.push(w);
    }
    Analysed { p, walks }
}

/// How a fight opens, from the sighting rolls alone.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Opening {
    AFreeRound,
    BFreeRound,
    BothFire,
    PassUnseen,
}

fn opening(f: &FightRec) -> Opening {
    let Some(first) = f.rolls.iter().filter(|r| r.sighted).map(|r| r.round).min() else {
        return Opening::PassUnseen;
    };
    let sighted = |force: usize| {
        f.rolls
            .iter()
            .any(|r| r.round == first && r.force == force && r.sighted)
    };
    match (sighted(0), sighted(1)) {
        (true, false) => Opening::AFreeRound,
        (false, true) => Opening::BFreeRound,
        (true, true) => Opening::BothFire,
        (false, false) => unreachable!("a round with a success"),
    }
}

fn analysed_runs() -> &'static Vec<Analysed> {
    static ANALYSED: OnceLock<Vec<Analysed>> = OnceLock::new();
    ANALYSED.get_or_init(|| {
        batch()
            .runs
            .iter()
            .map(|run| analyse(parse(run.seed, &run.events)))
            .collect()
    })
}

/// Every run with what was made of it.
fn runs() -> impl Iterator<Item = (&'static RunData, &'static Analysed)> {
    batch().runs.iter().zip(analysed_runs())
}

/// One fight of one run.
struct FightView {
    run: &'static RunData,
    index: usize,
    f: &'static FightRec,
    w: &'static Walk,
}

impl FightView {
    fn seed(&self) -> u64 {
        self.run.seed
    }
}

/// Every fight of every run, in order.
fn fight_views() -> Vec<FightView> {
    runs()
        .flat_map(|(run, an)| {
            an.p.fights
                .iter()
                .zip(&an.walks)
                .enumerate()
                .map(move |(index, (f, w))| FightView { run, index, f, w })
        })
        .collect()
}

/// The ships that came up for fire in a round of a fight where their force
/// had sighted the enemy: every ship afloat at the start of such a round.
/// Torpedoes are gone after the first such round, whatever the mounts gave.
fn attempted_firers(an: &Analysed) -> BTreeSet<u32> {
    let mut firers = BTreeSet::new();
    for w in &an.walks {
        for st in &w.rounds {
            for force in 0..2 {
                if st.sighted[force] {
                    firers.extend(st.afloat[force].iter().copied());
                }
            }
        }
    }
    firers
}

/// A gun salvo whose shots were fewer than 120 x the firer's main battery
/// factor because the ship had too little ammunition left.
#[derive(Clone, Debug)]
struct AmmoCut {
    seed: u64,
    event: u32,
    pulse: Pulse,
    round: u32,
    ship: u32,
    /// What the main battery would have fired, and the rounds aboard.
    full: Fixed,
    aboard: Fixed,
    shots: Fixed,
}

/// Every gun salvo of a run in which the ship's ammunition, not its main
/// battery, set the shots. Each ship starts with 800 rounds and uses up the
/// shots it fires, over every fight of the run.
fn ammo_cuts(an: &Analysed) -> Vec<AmmoCut> {
    let mut aboard: BTreeMap<u32, Fixed> = (1..=6).map(|ship| (ship, fx(800))).collect();
    let mut cuts = Vec::new();
    for (f, w) in an.p.fights.iter().zip(&an.walks) {
        for salvo in f.salvos.iter().filter(|s| s.weapon == Weapon::Gun) {
            let st = &w.rounds[usize::try_from(salvo.round - 1).expect("small")];
            let battery = system_factor(
                ShipSystem::MainBattery,
                st.start.systems[&salvo.firer].main_battery,
            );
            let full = fx(120) * battery;
            let left = aboard.get_mut(&salvo.firer).expect("a ship");
            if salvo.shots < full {
                cuts.push(AmmoCut {
                    seed: an.p.seed,
                    event: salvo.event,
                    pulse: f.pulse,
                    round: salvo.round,
                    ship: salvo.firer,
                    full,
                    aboard: *left,
                    shots: salvo.shots,
                });
            }
            *left -= salvo.shots;
        }
    }
    cuts
}

/// Whether k out of n is within four standard deviations of probability
/// `p` (in millionths), in exact integer arithmetic.
fn within_four_sigma(k: i64, n: i64, p: i64) -> bool {
    let (k, n, p) = (i128::from(k), i128::from(n), i128::from(p));
    let scale = 1_000_000;
    let off = k * scale - n * p;
    off * off <= 16 * n * p * (scale - p)
}

fn total_flotation(run: &RunData, force: usize) -> i64 {
    SHIPS[force]
        .iter()
        .map(|ship| run.ships[ship].flotation.raw())
        .sum()
}

fn ships_sunk(run: &RunData, force: usize) -> i64 {
    let count = SHIPS[force]
        .iter()
        .filter(|ship| run.ships[ship].location == ShipLocation::Sunk)
        .count();
    i64::try_from(count).expect("small")
}

// ---------------------------------------------------------------------
// The meeting, and what force A does afterwards.
// ---------------------------------------------------------------------

/// What the movement rules give a run, replayed here from the model's
/// tracks: the pulses in which the forces fight, the steps of force A, the
/// miles each of its ships steamed while afloat, and the total.
struct Movement {
    fights: Vec<Pulse>,
    steps: Vec<(Pulse, Hex, Hex, Fixed, Fixed)>,
    total_nm: i64,
    burned_nm: BTreeMap<u32, i64>,
}

/// Force A steams 180 nm at 16 kn to enter (5,0), 11.25 hours into the night
/// pulse, and fights force B there (decision 0022). It then steams on for the
/// rest of the night: the 12 nm it had left at 16 kn, scaled by its new speed
/// over 16, at the speed of its slowest ship afloat (16, 8 or 0 kn). Then, in
/// each daylight pulse, it steams at the speed of its slowest ship afloat for
/// 6 hours. Force B never moves. The forces fight in a pulse if A has a ship
/// afloat, B has a ship afloat and A is in B's hex after moving.
fn expected_movement(an: &Analysed) -> Movement {
    let mut m = Movement {
        fights: vec![Pulse::Night],
        steps: Vec::new(),
        total_nm: 0,
        burned_nm: SHIPS[0].iter().map(|ship| (*ship, 0)).collect(),
    };
    let step = |m: &mut Movement, pulse: Pulse, distance: i64, speed: i64, afloat: &[u32]| {
        let from = hex_at(m.total_nm);
        m.total_nm += distance;
        m.steps
            .push((pulse, from, hex_at(m.total_nm), fx(distance), fx(speed)));
        for ship in afloat {
            *m.burned_nm.get_mut(ship).expect("a ship of A") += distance;
        }
    };
    step(&mut m, Pulse::Night, NIGHT_ENTRY_NM, CRUISE_KN, &SHIPS[0]);
    // The rest of the night, at the speed after the night fight.
    let after_night = an.walks[0].end.afloat(0);
    if let Some(slowest) = an.walks[0].end.slowest_propulsion(0) {
        let speed = ship_speed_kn(slowest);
        if speed > 0 {
            let rest = (NIGHT_HOURS * CRUISE_KN - NIGHT_ENTRY_NM) * speed / CRUISE_KN;
            step(&mut m, Pulse::Night, rest, speed, &after_night);
        }
    }
    let mut tracks = an.walks[0].end.clone();
    let mut fights = 1;
    let slowest_kn = |tracks: &Tracks, a: &[u32]| {
        a.iter()
            .map(|ship| ship_speed_kn(tracks.systems[ship].propulsion))
            .min()
            .expect("a ship afloat")
    };
    for pulse in [Pulse::Morning, Pulse::Afternoon] {
        let mut a = tracks.afloat(0);
        if a.is_empty() {
            continue;
        }
        let mut speed = slowest_kn(&tracks, &a);
        // A force dead in the water in B's hex fights at the start of the
        // pulse; if it is freed by a sinking it steams the whole pulse
        // straight after.
        if speed == 0 && hex_at(m.total_nm) == MEETING && !tracks.afloat(1).is_empty() {
            m.fights.push(pulse);
            let Some(walk) = an.walks.get(fights) else {
                break;
            };
            tracks = walk.end.clone();
            fights += 1;
            a = tracks.afloat(0);
            if a.is_empty() {
                continue;
            }
            speed = slowest_kn(&tracks, &a);
        }
        if speed > 0 {
            step(&mut m, pulse, speed * pulse_hours(pulse), speed, &a);
        }
    }
    m
}

#[test]
fn the_forces_meet_at_five_zero_and_fight_again_only_while_a_is_dead_in_the_water() {
    let batch = batch();
    assert_eq!(batch.runs.len(), 1000);
    let own = load();
    assert!(own.placeholder);
    assert_eq!(own.days, 1);
    // After the night fight: A's slowest ship afloat is working, degraded,
    // out; or A has no ship afloat. And how many Out runs A got away in.
    let mut cases = [0usize; 4];
    let mut freed = 0;
    let mut second_fights = 0;
    for (run, an) in runs() {
        let seed = run.seed;
        let p = &an.p;
        let begun = run
            .events
            .iter()
            .filter(|e| matches!(e.kind, EventKind::EngagementBegun { .. }))
            .count();
        let ends = run
            .events
            .iter()
            .filter(|e| matches!(e.kind, EventKind::EngagementEnded { .. }))
            .count();
        assert_eq!(
            (begun, ends),
            (p.fights.len(), p.fights.len()),
            "seed {seed}"
        );
        for fight in &p.fights {
            let begun_event = &run.events[usize::try_from(fight.begun_event - 1).expect("small")];
            assert!(
                matches!(begun_event.kind, EventKind::EngagementBegun { .. }),
                "seed {seed}"
            );
            assert_eq!(
                fight.begun_with,
                (TaskForceId::new(1), TaskForceId::new(2), MEETING),
                "seed {seed}"
            );
            assert_eq!(fight.ended.hex, MEETING, "seed {seed}");
        }
        assert_eq!(
            p.fights[0].pulse,
            Pulse::Night,
            "seed {seed}: the first fight"
        );
        second_fights += p.fights.len() - 1;
        // A enters (5,0) 20 nm short of its centre, after 180 nm at 16 kn:
        // 11.25 hours into the night pulse, which starts at hour 0. Later
        // fights are at the start of their pulse: hours 12 and 18.
        for fight in &p.fights {
            let hours = match fight.pulse {
                Pulse::Night => millionths(11_250_000),
                Pulse::Morning => fx(12),
                Pulse::Afternoon => fx(18),
            };
            assert_eq!(fight.elapsed_hours, hours, "seed {seed}: fight hour");
        }

        // Force A steams as the model says, and fights again exactly when
        // the model says.
        let m = expected_movement(an);
        let pulses: Vec<Pulse> = p.fights.iter().map(|f| f.pulse).collect();
        assert_eq!(pulses, m.fights, "seed {seed}: pulses of the fights");
        let steps: Vec<(Pulse, Hex, Hex, Fixed, Fixed)> = p
            .steps
            .iter()
            .map(|s| {
                assert_eq!(s.task_force, 1, "seed {seed}: only force A moves");
                (s.pulse, s.from, s.to, s.distance, s.speed)
            })
            .collect();
        assert_eq!(steps, m.steps, "seed {seed}: steps of force A");
        assert_eq!(
            run.forces[&1].position,
            hex_at(m.total_nm),
            "seed {seed}: where A ends"
        );

        // Fuel: 0.25 t a nm for every ship while afloat, at any speed. Force
        // B burns nothing.
        for burn in &p.burns {
            assert_eq!(burn.task_force, 1, "seed {seed}: only force A burns fuel");
            // Each burn follows the step that caused it: 0.25 t a nm.
            let step = p
                .steps
                .iter()
                .rfind(|s| s.event < burn.event)
                .expect("a burn follows a step");
            assert_eq!(
                burn.burned,
                millionths(BURN_PER_NM) * step.distance.trunc(),
                "seed {seed}"
            );
        }
        for ship in SHIPS[0] {
            let nm = m.burned_nm[&ship];
            let burned = millionths(BURN_PER_NM * nm);
            let events: Vec<&BurnRec> = p.burns.iter().filter(|b| b.ship == ship).collect();
            let total = events.iter().fold(Fixed::ZERO, |sum, b| sum + b.burned);
            assert_eq!(total, burned, "seed {seed} ship {ship}: fuel burned");
            assert_eq!(
                run.ships[&ship].fuel,
                fx(2000) - burned,
                "seed {seed} ship {ship}"
            );
            if let Some(last) = events.last() {
                assert_eq!(last.remaining, run.ships[&ship].fuel, "seed {seed}");
            }
        }
        assert_eq!(run.forces[&2].position, MEETING, "seed {seed}");
        for ship in SHIPS[1] {
            assert_eq!(run.ships[&ship].fuel, fx(2000), "seed {seed} ship {ship}");
        }

        // The hand calculation's table, by the propulsion of A's slowest
        // ship afloat after the night fight.
        let night = (Pulse::Night, Hex::new(0, 0), MEETING, fx(180), fx(16));
        let a_afloat: Vec<u32> = an.walks[0].end.afloat(0);
        // Fuel left by every ship of A afloat at the end: the miles it
        // steamed, at 0.25 t a nm, and the figure the hand calculation gives.
        let fuel_of_afloat = |nm: i64, left: Fixed| {
            for ship in &a_afloat {
                assert_eq!(
                    run.ships[ship].fuel,
                    fx(2000) - millionths(BURN_PER_NM * nm),
                    "seed {seed} ship {ship}"
                );
                assert_eq!(run.ships[ship].fuel, left, "seed {seed} ship {ship}");
            }
        };
        let tail = |pulse, from: (i32, i32), to: (i32, i32), nm: i64, kn: i64| {
            (
                pulse,
                Hex::new(from.0, from.1),
                Hex::new(to.0, to.1),
                fx(nm),
                fx(kn),
            )
        };
        match an.walks[0].end.slowest_propulsion(0) {
            Some(SystemState::Working) => {
                cases[0] += 1;
                assert_eq!(pulses, vec![Pulse::Night], "seed {seed}: no second fight");
                assert_eq!(
                    steps,
                    vec![
                        night,
                        tail(Pulse::Night, (5, 0), (5, 0), 12, 16),
                        tail(Pulse::Morning, (5, 0), (7, 0), 96, 16),
                        tail(Pulse::Afternoon, (7, 0), (10, 0), 96, 16),
                    ],
                    "seed {seed}"
                );
                assert_eq!(run.forces[&1].position, Hex::new(10, 0), "seed {seed}");
                fuel_of_afloat(384, fx(1904));
            }
            Some(SystemState::Degraded) => {
                cases[1] += 1;
                assert_eq!(pulses, vec![Pulse::Night], "seed {seed}: no second fight");
                assert_eq!(
                    steps,
                    vec![
                        night,
                        tail(Pulse::Night, (5, 0), (5, 0), 6, 8),
                        tail(Pulse::Morning, (5, 0), (6, 0), 48, 8),
                        tail(Pulse::Afternoon, (6, 0), (7, 0), 48, 8),
                    ],
                    "seed {seed}"
                );
                assert_eq!(run.forces[&1].position, Hex::new(7, 0), "seed {seed}");
                fuel_of_afloat(282, millionths(1_929_500_000));
            }
            Some(SystemState::Out) => {
                cases[2] += 1;
                // Dead in the water at the end of the night: A does not
                // steam until a fight frees it. It fights again at the start
                // of the morning pulse if B has a ship afloat.
                let b_afloat = !an.walks[0].end.afloat(1).is_empty();
                assert_eq!(
                    pulses.contains(&Pulse::Morning),
                    b_afloat,
                    "seed {seed}: a second fight in the morning"
                );
                if steps == vec![night] {
                    assert_eq!(run.forces[&1].position, MEETING, "seed {seed}");
                    fuel_of_afloat(180, fx(1955));
                } else {
                    // The ship dead in the water sank in a day fight, and A
                    // steamed the whole pulse straight after it (decision
                    // 0022): in the morning if freed in the morning, and
                    // again in the afternoon.
                    freed += 1;
                    let first = steps.get(1).expect("a step after the night").0;
                    assert!(pulses.contains(&first), "seed {seed}");
                    assert!(steps[1..].iter().all(|s| s.4 > Fixed::ZERO), "seed {seed}");
                    let freed_in = pulses[1..]
                        .iter()
                        .copied()
                        .find(|pulse| *pulse == first)
                        .expect("fought in the pulse it was freed in");
                    let expected: Vec<Pulse> = if freed_in == Pulse::Morning {
                        vec![Pulse::Morning, Pulse::Afternoon]
                    } else {
                        vec![Pulse::Afternoon]
                    };
                    let got: Vec<Pulse> = steps[1..].iter().map(|s| s.0).collect();
                    assert_eq!(got, expected, "seed {seed}");
                    // Each step is 6 hours at its speed.
                    for step in &steps[1..] {
                        assert_eq!(step.3, step.4 * fx(6), "seed {seed}");
                    }
                }
            }
            None => {
                cases[3] += 1;
                assert_eq!(
                    steps,
                    vec![night],
                    "seed {seed}: a sunk force steams no more"
                );
                assert_eq!(pulses, vec![Pulse::Night], "seed {seed}");
            }
        }
    }
    eprintln!(
        "slowest ship of A after the night fight: working {}, degraded {}, out {}, none afloat {}; out and then freed by a sinking in a day fight and steaming on in the same pulse {freed}; second fights {second_fights}",
        cases[0], cases[1], cases[2], cases[3]
    );
    assert!(cases[..3].iter().all(|n| *n > 0), "every case is met");
}

// ---------------------------------------------------------------------
// Detection.
// ---------------------------------------------------------------------

#[test]
fn detection_falls_where_the_hand_calculation_says() {
    let (mut a1, mut b1) = (0i64, 0i64);
    let mut ways = [0i64; 4];
    // The counts are for the night fight only.
    for (_, an) in runs() {
        let (p, w) = (&an.p.fights[0], &an.walks[0]);
        let seed = an.p.seed;
        // A force rolls once a round until it sights, at 0.5 for A and 0.25
        // for B, and never rolls after.
        let mut rolled = [0u32; 2];
        for roll in &p.rolls {
            let expected = if roll.force == 0 {
                millionths(500_000)
            } else {
                millionths(250_000)
            };
            assert_eq!(roll.chance, expected, "seed {seed}");
            rolled[roll.force] += 1;
        }
        for (f, rolled_f) in rolled.iter().enumerate() {
            let expected_rolls = w.rounds.iter().filter(|r| r.rolls[f]).count();
            assert_eq!(
                usize::try_from(*rolled_f).expect("small"),
                expected_rolls,
                "seed {seed} force {f}"
            );
            for st in &w.rounds {
                let rolls_now = p
                    .rolls
                    .iter()
                    .filter(|r| r.round == st.round && r.force == f)
                    .count();
                assert_eq!(rolls_now, usize::from(st.rolls[f]), "seed {seed}");
            }
        }
        a1 += i64::from(
            p.rolls
                .iter()
                .any(|r| r.round == 1 && r.force == 0 && r.sighted),
        );
        b1 += i64::from(
            p.rolls
                .iter()
                .any(|r| r.round == 1 && r.force == 1 && r.sighted),
        );
        let way = opening(p);
        ways[match way {
            Opening::AFreeRound => 0,
            Opening::BFreeRound => 1,
            Opening::BothFire => 2,
            Opening::PassUnseen => 3,
        }] += 1;
        // Passing unseen is what the end of the fight says too.
        assert_eq!(
            way == Opening::PassUnseen,
            p.ended.reason == EngagementEndReason::PassedUnseen,
            "seed {seed}"
        );
    }
    eprintln!("A sights in round 1: {a1}");
    eprintln!("B sights in round 1: {b1}");
    eprintln!(
        "A has a free round: {}, B has a free round: {}, both fire from the start: {}, pass unseen: {}",
        ways[0], ways[1], ways[2], ways[3]
    );
    assert!((437..=563).contains(&a1), "A sights in round 1: {a1}");
    assert!((196..=304).contains(&b1), "B sights in round 1: {b1}");
    assert!((537..=660).contains(&ways[0]), "A free round: {}", ways[0]);
    assert!((149..=249).contains(&ways[1]), "B free round: {}", ways[1]);
    assert!((149..=249).contains(&ways[2]), "both fire: {}", ways[2]);
    assert!((0..=9).contains(&ways[3]), "pass unseen: {}", ways[3]);
    assert_eq!(ways.iter().sum::<i64>(), 1000);
}

#[test]
fn a_fight_by_day_opens_with_both_forces_sighting_in_round_one() {
    let mut day_fights = 0;
    for v in fight_views().iter().filter(|v| v.index > 0) {
        let (f, seed) = (v.f, v.seed());
        day_fights += 1;
        assert!(!f.dark(), "seed {seed}: a second fight by night");
        // Both forces are alert and the sighting chance by day is 1: both
        // roll in round 1, both sight, nobody is surprised.
        let rolls: Vec<(u32, usize, Fixed, bool)> = f
            .rolls
            .iter()
            .map(|r| (r.round, r.force, r.chance, r.sighted))
            .collect();
        assert_eq!(
            rolls,
            vec![(1, 0, fx(1), true), (1, 1, fx(1), true)],
            "seed {seed}"
        );
        assert!(f.surprised.is_empty(), "seed {seed}");
        assert!(f.alerts.is_empty(), "seed {seed}: both were alert already");
        assert!(f.passed_unseen.is_empty(), "seed {seed}");
        assert_eq!(opening(f), Opening::BothFire, "seed {seed}");
        assert!(v.w.rounds[0].sighted == [true, true]);
        // Fire does not light a target by day.
        for salvo in &f.salvos {
            assert_eq!(salvo.factor("lit_factor"), Fixed::ONE, "seed {seed}");
        }
    }
    eprintln!("fights by day: {day_fights}");
    assert!(day_fights > 0);
}

// ---------------------------------------------------------------------
// Salvos.
// ---------------------------------------------------------------------

#[test]
fn every_salvo_follows_the_figures() {
    let mut count = 0usize;
    let mut by_day = 0usize;
    for v in fight_views() {
        let (f, w, seed) = (v.f, v.w, v.seed());
        let dark = f.dark();

        // ForceSurprised events are exactly the surprised forces.
        for st in &w.rounds {
            for force in 0..2 {
                let events = f
                    .surprised
                    .iter()
                    .filter(|(round, fo)| *round == st.round && *fo == force)
                    .count();
                assert_eq!(events, usize::from(st.surprised[force]), "seed {seed}");
            }
        }
        assert_eq!(
            f.surprised.len(),
            w.rounds
                .iter()
                .map(|st| st.surprised.iter().filter(|s| **s).count())
                .sum::<usize>()
        );

        for salvo in &f.salvos {
            count += 1;
            by_day += usize::from(!dark);
            let st = &w.rounds[usize::try_from(salvo.round - 1).expect("small")];
            let target_force = force_of_ship(salvo.target);
            assert_eq!(force_of_ship(salvo.firer), 1 - target_force);
            let surprised = st.surprised[target_force];
            // By day nobody is surprised and fire lights nothing.
            assert!(dark || !surprised, "seed {seed}: surprised by day");
            let lit = dark && st.start.fire[&salvo.target] > Fixed::ZERO;
            let firer = st.start.systems[&salvo.firer];
            let surprised_factor = if surprised { SURPRISED } else { Fixed::ONE };
            let lit_factor = if lit { LIT } else { Fixed::ONE };

            // The breakdown's factors are named, in this order, and carry
            // the figures the hand calculation gives.
            let names: Vec<&str> = match salvo.weapon {
                Weapon::Gun => vec![
                    "guns",
                    "rate",
                    "minutes",
                    "main_battery_factor",
                    "base_chance",
                    "surprised_factor",
                    "lit_factor",
                    "fire_control_factor",
                ],
                Weapon::Torpedo => vec![
                    "tubes",
                    "rate",
                    "minutes",
                    "torpedo_mount_factor",
                    "base_chance",
                    "surprised_factor",
                    "lit_factor",
                ],
                Weapon::Bomb => unreachable!("scenario 4a fires no bombs"),
            };
            let listed: Vec<&str> = salvo.factors.iter().map(|(name, _)| *name).collect();
            assert_eq!(listed, names, "seed {seed}");
            let base = match (salvo.weapon, dark) {
                (Weapon::Gun, true) => GUN_BASE_NIGHT,
                (Weapon::Gun, false) => GUN_BASE_DAY,
                (Weapon::Torpedo, true) => TORPEDO_BASE_NIGHT,
                (Weapon::Torpedo, false) => TORPEDO_BASE_DAY,
                (Weapon::Bomb, _) => unreachable!("scenario 4a fires no bombs"),
            };
            assert_eq!(salvo.factor("base_chance"), base, "seed {seed}");
            assert_eq!(
                salvo.factor("surprised_factor"),
                surprised_factor,
                "seed {seed}"
            );
            assert_eq!(salvo.factor("lit_factor"), lit_factor, "seed {seed}");
            assert_eq!(salvo.result, salvo.expected);

            // Shots, and the factor each firer's system gives: read from the
            // breakdown and checked against the state at the start of the
            // round, which the model got from the SystemHit events.
            let mut chance_factors = base * surprised_factor * lit_factor;
            match salvo.weapon {
                Weapon::Gun => {
                    assert_eq!(salvo.factor("guns"), fx(8));
                    assert_eq!(salvo.factor("rate"), fx(3));
                    assert_eq!(salvo.factor("minutes"), fx(5));
                    // A ship whose main battery is out fires no gun salvo.
                    assert!(
                        firer.main_battery != SystemState::Out,
                        "seed {seed}: ship {} fires guns with its main battery out",
                        salvo.firer
                    );
                    let battery = system_factor(ShipSystem::MainBattery, firer.main_battery);
                    assert_eq!(salvo.factor("main_battery_factor"), battery, "seed {seed}");
                    // Ammunition can cut this short in a later fight: see
                    // `ammunition_only_ever_cuts_a_salvo_to_the_rounds_aboard`.
                    assert!(salvo.shots <= fx(120) * battery, "seed {seed}: gun shots");
                    let control = system_factor(ShipSystem::FireControl, firer.fire_control);
                    assert_eq!(salvo.factor("fire_control_factor"), control, "seed {seed}");
                    chance_factors = chance_factors * control;
                }
                Weapon::Torpedo => {
                    assert_eq!(salvo.factor("tubes"), fx(8));
                    assert_eq!(salvo.factor("rate"), fx(1));
                    assert_eq!(salvo.factor("minutes"), fx(1));
                    // A ship whose mounts are out fires no torpedo salvo.
                    assert!(
                        firer.torpedo_mounts != SystemState::Out,
                        "seed {seed}: ship {} fires torpedoes with its mounts out",
                        salvo.firer
                    );
                    let mounts = system_factor(ShipSystem::TorpedoMounts, firer.torpedo_mounts);
                    assert_eq!(salvo.factor("torpedo_mount_factor"), mounts, "seed {seed}");
                    assert_eq!(salvo.shots, fx(8) * mounts, "seed {seed}: torpedo shots");
                }
                Weapon::Bomb => unreachable!("scenario 4a fires no bombs"),
            }
            assert!(salvo.shots > Fixed::ZERO);

            // Chance, expected hits, hits.
            assert_eq!(
                salvo.chance, chance_factors,
                "seed {seed} salvo #{}",
                salvo.event
            );
            assert_eq!(salvo.expected, salvo.shots * salvo.chance, "seed {seed}");
            // Each shot is rolled on its own (decision 0033): the hits are
            // no more than the shots. The spread is checked over the runs in
            // `hits_and_where_they_land_stay_within_four_sigma`.
            assert!(
                Fixed::from_int(i64::from(salvo.hits)) <= salvo.shots,
                "seed {seed}"
            );
        }
    }
    eprintln!("salvos checked: {count}, of them by day: {by_day}");
    assert!(count > 10_000);
}

#[test]
fn each_ship_ends_with_800_rounds_less_the_gun_shots_it_fired() {
    for (run, an) in runs() {
        let attempted = attempted_firers(an);
        for ship in 1..=6u32 {
            let shots: Fixed =
                an.p.fights
                    .iter()
                    .flat_map(|f| f.salvos.iter())
                    .filter(|s| s.firer == ship && s.weapon == Weapon::Gun)
                    .fold(Fixed::ZERO, |sum, s| sum + s.shots);
            let end = &run.ships[&ship];
            assert_eq!(end.rounds, fx(800) - shots, "seed {} ship {ship}", run.seed);
            assert!(end.rounds >= Fixed::ZERO);
            // Torpedoes are fired at most once per ship over all the fights.
            let salvos =
                an.p.fights
                    .iter()
                    .flat_map(|f| f.salvos.iter())
                    .filter(|s| s.firer == ship && s.weapon == Weapon::Torpedo)
                    .count();
            assert!(salvos <= 1, "seed {} ship {ship}", run.seed);
            // They are gone exactly when a torpedo salvo was fired. A ship
            // whose mounts were out when it came up to fire keeps them.
            assert_eq!(
                end.torpedoes_ready,
                salvos == 0,
                "seed {} ship {ship}",
                run.seed
            );
            if !attempted.contains(&ship) {
                assert!(end.torpedoes_ready, "seed {} ship {ship}", run.seed);
            }
        }
    }
}

// ---------------------------------------------------------------------
// Hits over 1,000 runs.
// ---------------------------------------------------------------------

/// Checks (H - E)^2 <= 16 x the sum of np(1 - p), in raw millionths.
fn salvo_sums_hold(salvos: &[&SalvoRec]) -> (i128, i128, i128, bool) {
    let mut hits = 0i128;
    let mut expected = 0i128;
    let mut variance = 0i128;
    for salvo in salvos {
        hits += i128::from(salvo.hits) * 1_000_000;
        expected += i128::from(salvo.expected.raw());
        // Every salvo of scenario 4a has a whole number of shots.
        assert_eq!(salvo.shots.raw() % 1_000_000, 0);
        let p = i128::from(salvo.chance.raw());
        variance += i128::from(salvo.shots.trunc()) * p * (1_000_000 - p);
    }
    let off = hits - expected;
    (hits, expected, variance, off * off <= 16 * variance)
}

#[test]
fn hits_and_where_they_land_stay_within_four_sigma() {
    let views = fight_views();
    let all: Vec<&SalvoRec> = views.iter().flat_map(|v| v.f.salvos.iter()).collect();
    let guns: Vec<&SalvoRec> = all
        .iter()
        .copied()
        .filter(|s| s.weapon == Weapon::Gun)
        .collect();
    let torpedoes: Vec<&SalvoRec> = all
        .iter()
        .copied()
        .filter(|s| s.weapon == Weapon::Torpedo)
        .collect();
    for (name, salvos) in [("gun", &guns), ("torpedo", &torpedoes)] {
        let (hits, expected, variance, holds) = salvo_sums_hold(salvos);
        eprintln!(
            "{name} salvos {}: hits {} expected {} (millionths: H {hits}, E {expected}, sum np(1-p) {variance})",
            salvos.len(),
            hits / 1_000_000,
            Fixed::from_raw(i64::try_from(expected).expect("fits"))
        );
        assert!(holds, "{name} hits are outside four sigma");
    }

    // Where the hits land. Hits are counted from the damage events and tied
    // back to the weapon of the salvo that caused them.
    let mut gun_hits = 0i64;
    let mut by_track = [0i64; 2];
    let mut torpedo_hits = 0i64;
    let mut torpedo_on_flotation = 0i64;
    for v in &views {
        let (f, seed) = (v.f, v.seed());
        let weapon_of: BTreeMap<u32, Weapon> =
            f.salvos.iter().map(|s| (s.event, s.weapon)).collect();
        for damage in &f.damages {
            let weapon = weapon_of[&damage.cause];
            let hits = i64::from(damage.hits);
            match weapon {
                Weapon::Gun => {
                    gun_hits += hits;
                    by_track[match damage.track {
                        DamageTrack::Flotation => 0,
                        DamageTrack::Fire => 1,
                    }] += hits;
                }
                Weapon::Torpedo => {
                    torpedo_hits += hits;
                    if damage.track == DamageTrack::Flotation {
                        torpedo_on_flotation += hits;
                    }
                }
                Weapon::Bomb => unreachable!("scenario 4a fires no bombs"),
            }
        }
        // Each salvo's hits are all accounted for on the tracks.
        for salvo in &f.salvos {
            let landed: u32 = f
                .damages
                .iter()
                .filter(|d| d.cause == salvo.event)
                .map(|d| d.hits)
                .sum();
            assert_eq!(landed, salvo.hits, "seed {seed} salvo #{}", salvo.event);
        }
    }
    let salvo_gun_hits: i64 = guns.iter().map(|s| i64::from(s.hits)).sum();
    let salvo_torpedo_hits: i64 = torpedoes.iter().map(|s| i64::from(s.hits)).sum();
    assert_eq!(gun_hits, salvo_gun_hits);
    assert_eq!(torpedo_hits, salvo_torpedo_hits);
    eprintln!(
        "gun hits {gun_hits}: flotation {}, fire {}; torpedo hits {torpedo_hits}, on flotation {torpedo_on_flotation}",
        by_track[0], by_track[1]
    );
    assert_eq!(by_track.iter().sum::<i64>(), gun_hits);
    for (name, k, p) in [
        ("flotation", by_track[0], 750_000),
        ("fire", by_track[1], 250_000),
    ] {
        assert!(
            within_four_sigma(k, gun_hits, p),
            "{name}: {k} of {gun_hits}"
        );
    }
    assert_eq!(torpedo_on_flotation, torpedo_hits);
}

/// The system hits of every fight in every run, counted by the weapon of the
/// salvo each cites and the system it landed on: [gun, torpedo][system in
/// code order]. Also the hits of each weapon.
fn system_hit_counts() -> ([[i64; 4]; 2], [i64; 2]) {
    let mut counts = [[0i64; 4]; 2];
    let mut hits = [0i64; 2];
    for v in fight_views() {
        let weapon_of: BTreeMap<u32, &SalvoRec> = v.f.salvos.iter().map(|s| (s.event, s)).collect();
        for salvo in &v.f.salvos {
            hits[usize::from(salvo.weapon == Weapon::Torpedo)] += i64::from(salvo.hits);
            let landed =
                v.f.system_hits
                    .iter()
                    .filter(|h| h.cause == salvo.event)
                    .count();
            assert!(
                landed <= usize::try_from(salvo.hits).expect("small"),
                "seed {}: a hit rolls once for a system",
                v.seed()
            );
        }
        for hit in &v.f.system_hits {
            // The event cites the salvo whose hit it was.
            let salvo = weapon_of[&hit.cause];
            assert_eq!((salvo.target, salvo.round), (hit.ship, hit.round));
            let slot = ShipSystem::ALL
                .iter()
                .position(|s| *s == hit.system)
                .expect("a system");
            counts[usize::from(salvo.weapon == Weapon::Torpedo)][slot] += 1;
        }
    }
    (counts, hits)
}

#[test]
fn system_hits_land_where_the_hand_calculation_says() {
    let (counts, hits) = system_hit_counts();
    let tables = [
        (GUN_SYSTEM_P, GUN_NO_SYSTEM_P, "gun"),
        (TORPEDO_SYSTEM_P, TORPEDO_NO_SYSTEM_P, "torpedo"),
    ];
    for (weapon, (shares, none_p, name)) in tables.into_iter().enumerate() {
        let n = hits[weapon];
        let none = n - counts[weapon].iter().sum::<i64>();
        // The tables of this scenario cover the four systems that existed
        // when it was written; the flight deck (decision 0026) is never hit.
        for (slot, system) in ShipSystem::ALL.iter().take(4).enumerate() {
            let k = counts[weapon][slot];
            assert!(
                within_four_sigma(k, n, shares[slot]),
                "{name} hits on {}: {k} of {n}",
                system.name()
            );
        }
        assert!(
            within_four_sigma(none, n, none_p),
            "{name} hits on no system: {none} of {n}"
        );
        // The shares and no system make up every hit.
        assert_eq!(shares.iter().sum::<i64>() + none_p, 1_000_000);
    }
    // A torpedo hit never lands on torpedo mounts.
    assert_eq!(counts[1][3], 0);
}

#[test]
fn a_system_hit_steps_a_system_down_one_level_and_nothing_recovers() {
    let mut steps = 0;
    let mut no_change = 0;
    for (run, an) in runs() {
        let seed = run.seed;
        // Every hit, in event order, finds the system as the model has it
        // and leaves it one level lower, unless it was out already or the
        // class has no such system (the walk asserts this, hit by hit).
        let mut last: BTreeMap<(u32, ShipSystem), SystemState> = BTreeMap::new();
        for hit in an.p.system_hits() {
            let was = last
                .get(&(hit.ship, hit.system))
                .copied()
                .unwrap_or(SystemState::Working);
            assert_eq!(hit.before, was, "seed {seed}: #{}", hit.event);
            assert!(hit.after >= hit.before, "seed {seed}: a system recovered");
            if hit.before == SystemState::Out || !class_has(hit.ship, hit.system) {
                assert_eq!(hit.after, hit.before, "seed {seed}");
                no_change += 1;
            } else {
                assert!(hit.after > hit.before, "seed {seed}");
                assert_eq!(hit.after, hit.before.step_down(), "seed {seed}");
                steps += 1;
            }
            last.insert((hit.ship, hit.system), hit.after);
        }
        // The ships end with the states the hits left them in, and the
        // systems no hit touched are working.
        let end = &an.walks.last().expect("a fight").end;
        for ship in 1..=6u32 {
            assert_eq!(run.ships[&ship].systems, end.systems[&ship], "seed {seed}");
            for system in ShipSystem::ALL {
                let expected = last
                    .get(&(ship, system))
                    .copied()
                    .unwrap_or(SystemState::Working);
                assert_eq!(
                    run.ships[&ship].systems.get(system),
                    expected,
                    "seed {seed} ship {ship} {}",
                    system.name()
                );
            }
        }
    }
    eprintln!("system hits that stepped a system down: {steps}, with no change: {no_change}");
    assert!(steps > 0 && no_change > 0);
}

// ---------------------------------------------------------------------
// Damage, sinking, matching.
// ---------------------------------------------------------------------

#[test]
fn damage_adds_up_and_ships_sink_exactly_when_flotation_reaches_100() {
    for (run, an) in runs() {
        let seed = run.seed;
        let p = &an.p;
        let end = &an.walks.last().expect("a fight").end;

        for (f, w) in p.fights.iter().zip(&an.walks) {
            // Each damage event: its cause is a salvo at that ship, its
            // points are hits x the damage of one hit, and its value_after
            // is the running total, capped at 100.
            let salvo_of: BTreeMap<u32, &SalvoRec> =
                f.salvos.iter().map(|s| (s.event, s)).collect();
            for (index, damage) in f.damages.iter().enumerate() {
                let salvo = salvo_of[&damage.cause];
                assert_eq!(salvo.target, damage.ship, "seed {seed}");
                assert_eq!(salvo.round, damage.round, "seed {seed}");
                let per_hit = match salvo.weapon {
                    Weapon::Gun => GUN_HIT_POINTS,
                    Weapon::Torpedo => {
                        assert_eq!(damage.track, DamageTrack::Flotation);
                        TORPEDO_HIT_POINTS
                    }
                    Weapon::Bomb => unreachable!("scenario 4a fires no bombs"),
                };
                assert!(damage.hits > 0);
                assert_eq!(damage.points, fx(per_hit * i64::from(damage.hits)));
                assert_eq!(damage.after, w.after[index], "seed {seed} damage {index}");
                assert!(damage.after >= Fixed::ZERO && damage.after <= fx(100));
            }

            // The ShipSunk events: one per ship that sank in this fight, in
            // the round its flotation reached 100, caused by the damage
            // event that took it there.
            assert_eq!(f.sunk.len(), w.sunk_round.len(), "seed {seed}");
            for sunk in &f.sunk {
                assert_eq!(
                    w.sunk_round.get(&sunk.ship),
                    Some(&sunk.round),
                    "seed {seed}"
                );
                assert_eq!(sunk.force, force_of_ship(sunk.ship));
                let event = run
                    .events
                    .get(usize::try_from(sunk.cause - 1).expect("small"))
                    .expect("the cause exists");
                match &event.kind {
                    EventKind::ShipDamaged {
                        ship,
                        track,
                        value_after,
                        round,
                        ..
                    } => {
                        assert_eq!(ship.raw(), sunk.ship);
                        assert_eq!(*track, DamageTrack::Flotation);
                        assert_eq!(*value_after, fx(100));
                        assert_eq!(*round, sunk.round);
                    }
                    other => panic!("seed {seed}: a ship sank because of {other:?}"),
                }
            }

            // Nothing fires from or at a ship in a round after it sank, and
            // nothing is damaged after.
            for salvo in &f.salvos {
                for ship in [salvo.firer, salvo.target] {
                    if let Some(round) = w.sunk_round.get(&ship) {
                        assert!(salvo.round <= *round, "seed {seed}: ship {ship}");
                    }
                }
            }
            for damage in &f.damages {
                if let Some(round) = w.sunk_round.get(&damage.ship) {
                    assert!(damage.round <= *round, "seed {seed}");
                }
            }
            for salvo in &f.salvos {
                let st = &w.rounds[usize::try_from(salvo.round - 1).expect("small")];
                assert!(st.afloat[force_of_ship(salvo.firer)].contains(&salvo.firer));
                assert!(st.afloat[force_of_ship(salvo.target)].contains(&salvo.target));
            }
        }

        // A ship sunk in one fight is in none after: it is not afloat at
        // the start of a later one, and nothing is aimed at it.
        for (index, w) in an.walks.iter().enumerate().skip(1) {
            for ship in w.rounds[0].afloat.iter().flatten() {
                assert!(
                    !an.walks[..index]
                        .iter()
                        .any(|earlier| earlier.sunk_round.contains_key(ship)),
                    "seed {seed}: sunk ship {ship} fights again"
                );
            }
        }

        // Final tracks equal the model's: min(100, sum of points over every
        // fight).
        for ship in 1..=6u32 {
            let ship_end = &run.ships[&ship];
            for (track, model, name) in [
                (ship_end.flotation, &end.flotation, "flotation"),
                (ship_end.fire, &end.fire, "fire"),
            ] {
                let points: i64 = p
                    .fights
                    .iter()
                    .flat_map(|f| f.damages.iter())
                    .filter(|d| d.ship == ship && d.track.name() == name)
                    .map(|d| d.points.raw())
                    .sum();
                let capped = points.min(100_000_000);
                assert_eq!(track.raw(), capped, "seed {seed} ship {ship} {name}");
                assert_eq!(track, model[&ship], "seed {seed} ship {ship} {name}");
            }
            // Sunk exactly when flotation reached 100.
            let sunk = ship_end.flotation == fx(100);
            assert_eq!(
                ship_end.location == ShipLocation::Sunk,
                sunk,
                "seed {seed} ship {ship}"
            );
            assert_eq!(
                an.walks.iter().any(|w| w.sunk_round.contains_key(&ship)),
                sunk
            );
            assert_eq!(p.sunk_at.contains_key(&ship), sunk);
            let in_a_force = run.forces.values().any(|f| f.ships.contains(&ship));
            assert_eq!(in_a_force, !sunk, "seed {seed} ship {ship}");
        }
    }
}

#[test]
fn ships_are_matched_in_id_order_among_those_afloat() {
    for (run, an) in runs() {
        let seed = run.seed;
        // Whether a ship has come up for fire in a round of any fight so
        // far, which is when its torpedoes are gone.
        let mut fired_before: BTreeSet<u32> = BTreeSet::new();
        for (f, w) in an.p.fights.iter().zip(&an.walks) {
            for st in &w.rounds {
                let salvos: Vec<&SalvoRec> =
                    f.salvos.iter().filter(|s| s.round == st.round).collect();

                // Order: lower force first, ship ID order, guns before
                // torpedoes.
                let keys: Vec<(usize, u32, u8)> = salvos
                    .iter()
                    .map(|s| {
                        (
                            force_of_ship(s.firer),
                            s.firer,
                            u8::from(s.weapon == Weapon::Torpedo),
                        )
                    })
                    .collect();
                let mut sorted = keys.clone();
                sorted.sort_unstable();
                assert_eq!(keys, sorted, "seed {seed} round {}", st.round);

                for force in 0..2 {
                    let own = &st.afloat[force];
                    let enemy = &st.afloat[1 - force];
                    let firers: BTreeSet<u32> = salvos
                        .iter()
                        .filter(|s| force_of_ship(s.firer) == force)
                        .map(|s| s.firer)
                        .collect();
                    if !st.sighted[force] {
                        assert!(firers.is_empty(), "seed {seed}: fires before it sighted");
                        continue;
                    }
                    for (index, ship) in own.iter().enumerate() {
                        let target = enemy[index % enemy.len()];
                        let guns: Vec<&&SalvoRec> = salvos
                            .iter()
                            .filter(|s| s.firer == *ship && s.weapon == Weapon::Gun)
                            .collect();
                        let torpedoes: Vec<&&SalvoRec> = salvos
                            .iter()
                            .filter(|s| s.firer == *ship && s.weapon == Weapon::Torpedo)
                            .collect();
                        let systems = st.start.systems[ship];
                        // A gun salvo exists unless the main battery is out.
                        let silent = systems.main_battery == SystemState::Out;
                        assert_eq!(guns.len(), usize::from(!silent), "seed {seed} ship {ship}");
                        // Torpedoes go in the first round the ship fires,
                        // unless its mounts are out.
                        let first = !fired_before.contains(ship);
                        let mounted = systems.torpedo_mounts != SystemState::Out;
                        assert_eq!(
                            torpedoes.len(),
                            usize::from(first && mounted),
                            "seed {seed} ship {ship}"
                        );
                        for salvo in guns.iter().chain(torpedoes.iter()) {
                            assert_eq!(
                                salvo.target, target,
                                "seed {seed} round {} ship {ship}",
                                st.round
                            );
                        }
                    }
                    fired_before.extend(own.iter().copied());
                    // Only ships afloat at the start fired.
                    assert!(firers.iter().all(|s| own.contains(s)));
                }
            }
        }
    }
}

// ---------------------------------------------------------------------
// Ending.
// ---------------------------------------------------------------------

/// Ships a force began a fight with that are sunk or crippled at the end of a
/// round.
fn lost(tracks: &Tracks, began: &[u32]) -> i64 {
    let count = began
        .iter()
        .filter(|ship| tracks.flotation[ship] >= fx(CRIPPLED))
        .count();
    i64::try_from(count).expect("small")
}

/// The end the rules give, found by replaying the rounds: the round, the
/// reason and the forces that break off. A force breaks off when at least
/// half the ships it began the fight with are sunk or crippled. Written out
/// again here from decision 14 and the hand calculation, using only the
/// model's tracks.
fn expected_end(f: &FightRec, w: &Walk) -> Option<(u32, EngagementEndReason, Vec<usize>)> {
    let ever_sighted = f.rolls.iter().any(|r| r.sighted) || !f.salvos.is_empty();
    let began = &w.rounds[0].afloat;
    for st in &w.rounds {
        // (1) A force with no ship afloat.
        if st.end.afloat(0).is_empty() || st.end.afloat(1).is_empty() {
            return Some((st.round, EngagementEndReason::ForceSunk, Vec::new()));
        }
        // (2) Break-off: at least half the ships it began with sunk or
        // crippled.
        let broke: Vec<usize> = (0..2)
            .filter(|force| {
                let ships = i64::try_from(began[*force].len()).expect("small");
                2 * lost(&st.end, &began[*force]) >= ships
            })
            .collect();
        if !broke.is_empty() {
            return Some((st.round, EngagementEndReason::BrokeOff, broke));
        }
        // (3) The last round.
        if st.round == MAX_ROUNDS {
            let reason = if ever_sighted {
                EngagementEndReason::MaxRounds
            } else {
                EngagementEndReason::PassedUnseen
            };
            return Some((st.round, reason, Vec::new()));
        }
    }
    None
}

#[test]
fn each_fight_ends_in_the_round_and_for_the_reason_the_rules_give() {
    for v in fight_views() {
        let (p, w, seed) = (v.f, v.w, v.seed());
        let (round, reason, broke) =
            expected_end(p, w).unwrap_or_else(|| panic!("seed {seed}: the model never ends"));
        assert_eq!(p.ended.round, round, "seed {seed}");
        assert_eq!(p.ended.reason, reason, "seed {seed}");
        assert_eq!(p.ended.hex, MEETING);
        assert_eq!(p.slipped, 0, "nobody avoids in this scenario");

        // Nothing happens in a round after the end: every rolled, fired or
        // damaged round is within the fight.
        for roll in &p.rolls {
            assert!(roll.round <= round, "seed {seed}");
        }

        // The ForceBrokeOff events name exactly the forces the model says,
        // in the last round, with the share of ships lost.
        let named: Vec<usize> = p.broke_off.iter().map(|(_, force, _)| *force).collect();
        assert_eq!(named, broke, "seed {seed}");
        for (event_round, force, share) in &p.broke_off {
            assert_eq!(*event_round, round);
            let began = &w.rounds[0].afloat[*force];
            assert_eq!(
                *share,
                Fixed::from_ratio(
                    lost(&w.last().end, began),
                    i64::try_from(began.len()).expect("small")
                ),
                "seed {seed}"
            );
        }

        // ForcesPassedUnseen: only in a fight that passes unseen, in round 6.
        let unseen = reason == EngagementEndReason::PassedUnseen;
        assert_eq!(
            p.passed_unseen,
            if unseen { vec![MAX_ROUNDS] } else { vec![] }
        );
        if unseen {
            assert!(p.salvos.is_empty());
            assert!(p.rolls.iter().all(|r| !r.sighted));
            assert_eq!(p.rolls.len(), 12, "two forces rolling in six rounds");
        }
    }
}

// ---------------------------------------------------------------------
// Reports and claims.
// ---------------------------------------------------------------------

#[test]
fn reports_and_claims_say_what_the_fight_says() {
    let base = load();
    let figures = base
        .world
        .ref_data()
        .search
        .clone()
        .expect("search figures are loaded");
    let cruiser_row: BTreeSet<ShipKind> = figures.kind_confusion[&ShipKind::Cruiser]
        .keys()
        .copied()
        .collect();
    assert_eq!(
        cruiser_row,
        BTreeSet::from([
            ShipKind::Cruiser,
            ShipKind::Battleship,
            ShipKind::Destroyer,
            ShipKind::Carrier
        ])
    );
    let mut outside_two_to_four = 0;
    for (run, an) in runs() {
        let seed = run.seed;
        // The k-th claim a side holds is the k-th claim it sent.
        let mut claims_sent = [0usize; 2];
        for (f, w) in an.p.fights.iter().zip(&an.walks) {
            let last = w.last();

            // Order after the fight: for force A then B, the sighting
            // report if it sighted, then the claim if it fired.
            let mut expected_order: Vec<(bool, Side)> = Vec::new();
            for (force, side) in SIDES.iter().enumerate() {
                if last.sighted[force] || last.fired_on[force] {
                    expected_order.push((false, *side));
                }
                if f.salvos.iter().any(|s| force_of_ship(s.firer) == force) {
                    expected_order.push((true, *side));
                }
            }
            assert_eq!(f.after_fight, expected_order, "seed {seed}");

            for (force, side) in SIDES.iter().copied().enumerate() {
                let enemy = 1 - force;
                let began_enemy = w.rounds[0].afloat[enemy].len();
                let afloat_end = last.end.afloat(enemy);
                let reports: Vec<&ReportRec> =
                    f.reports.iter().filter(|r| r.side == side).collect();
                assert_eq!(
                    reports.len(),
                    usize::from(last.sighted[force] || last.fired_on[force]),
                    "seed {seed}"
                );

                // The report says the enemy ships still afloat, or one more
                // or one fewer, and never fewer than one.
                if let Some(report) = reports.first() {
                    let truth = if afloat_end.is_empty() {
                        began_enemy
                    } else {
                        afloat_end.len()
                    };
                    let truth = u32::try_from(truth).expect("small");
                    let allowed = [truth.saturating_sub(1).max(1), truth, truth + 1];
                    assert!(
                        allowed.contains(&report.count),
                        "seed {seed}: {}",
                        report.count
                    );
                    if !(2..=4).contains(&report.count) {
                        outside_two_to_four += 1;
                    }
                    assert!(
                        report.hex.distance(MEETING) <= 1,
                        "seed {seed}: report at {:?}",
                        report.hex
                    );
                    assert!(cruiser_row.contains(&report.kind), "seed {seed}");
                    assert_eq!(report.cause, f.ended.event);
                }

                // Claims: a force that fired claims the enemy ships that
                // sank in the fight, and those left afloat with fire damage
                // of 30 or more.
                let fired = f.salvos.iter().any(|s| force_of_ship(s.firer) == force);
                let claims: Vec<&ClaimRec> = f.claims.iter().filter(|c| c.side == side).collect();
                assert_eq!(claims.len(), usize::from(fired), "seed {seed}");
                if let Some(claim) = claims.first() {
                    let sunk = began_enemy - afloat_end.len();
                    let burning = afloat_end
                        .iter()
                        .filter(|ship| last.end.fire[ship] >= fx(CLAIMED_FIRE))
                        .count();
                    let expected = u32::try_from(sunk + burning).expect("small");
                    assert_eq!(claim.claimed_sunk, expected, "seed {seed}");
                    // The claim is never less than the truth.
                    assert!(claim.claimed_sunk >= u32::try_from(sunk).expect("small"));
                    assert_eq!(claim.hex, MEETING);
                    assert_eq!(claim.cause, f.ended.event);
                    // Through the side's view.
                    let seen = &run.claims[&side][claims_sent[force]];
                    assert_eq!(
                        *seen,
                        ClaimEnd {
                            hex: MEETING,
                            day: 1,
                            pulse: f.pulse,
                            claimed_sunk: expected,
                            event: claim.event,
                        },
                        "seed {seed}"
                    );
                    claims_sent[force] += 1;
                }
            }
        }

        // What each side holds at the end of the run.
        for (force, side) in SIDES.iter().copied().enumerate() {
            let reports: Vec<&ReportRec> =
                an.p.fights
                    .iter()
                    .flat_map(|f| f.reports.iter())
                    .filter(|r| r.side == side)
                    .collect();
            let contacts = &run.contacts[&side];
            assert_eq!(run.claims[&side].len(), claims_sent[force], "seed {seed}");
            assert_eq!(contacts.is_empty(), reports.is_empty(), "seed {seed}");
            assert!(contacts.len() <= reports.len(), "seed {seed}");
            if let Some(first) = reports.first() {
                assert!(first.created, "seed {seed}");
            }
            // A side with one report holds that contact, aged by the pulses
            // left in the day.
            if let [report] = reports[..] {
                let pulse =
                    an.p.fights
                        .iter()
                        .find(|f| f.reports.iter().any(|r| r.event == report.event))
                        .expect("a fight sent it")
                        .pulse;
                let age = 2 - u32::try_from(pulse_index(pulse)).expect("small");
                let contact = &contacts[0];
                assert_eq!(contacts.len(), 1, "seed {seed}");
                assert_eq!(contact.id, report.contact);
                assert_eq!(contact.hex, report.hex);
                assert_eq!(contact.count, report.count);
                assert_eq!(contact.kind, report.kind);
                assert_eq!(contact.last_event, report.event);
                assert_eq!(contact.age, age, "seed {seed}");
            }
        }
    }
    eprintln!("reports whose count is outside 2 to 4: {outside_two_to_four}");
}

// ---------------------------------------------------------------------
// Alertness.
// ---------------------------------------------------------------------

#[test]
fn force_b_is_alert_when_it_sighted_or_was_fired_on() {
    let (mut alert_runs, mut unseen_runs) = (0, 0);
    for (run, an) in runs() {
        let seed = run.seed;
        // A is alert from the start and stays so.
        assert!(run.forces[&1].alert, "seed {seed}");
        let b_sighted =
            an.p.fights
                .iter()
                .any(|f| f.rolls.iter().any(|r| r.force == 1 && r.sighted));
        let b_fired_on =
            an.p.fights
                .iter()
                .any(|f| f.salvos.iter().any(|s| force_of_ship(s.target) == 1));
        let expected = b_sighted || b_fired_on;
        assert_eq!(run.forces[&2].alert, expected, "seed {seed}");
        if an.p.fights[0].ended.reason == EngagementEndReason::PassedUnseen {
            assert!(!run.forces[&2].alert, "seed {seed}");
            unseen_runs += 1;
        }
        alert_runs += i64::from(expected);

        // One ForceBecameAlert for B, in the first round of the first fight
        // in which it sighted or was fired on, and none for A and none
        // after that.
        let mut b_alert = false;
        for (f, w) in an.p.fights.iter().zip(&an.walks) {
            let first = w
                .rounds
                .iter()
                .find(|st| {
                    st.fired_on[1]
                        || f.rolls
                            .iter()
                            .any(|r| r.round == st.round && r.force == 1 && r.sighted)
                })
                .map(|st| st.round);
            let expected_events: Vec<(u32, usize)> = match first {
                Some(round) if !b_alert => vec![(round, 1)],
                _ => Vec::new(),
            };
            assert_eq!(f.alerts, expected_events, "seed {seed}");
            b_alert |= first.is_some();
        }
    }
    eprintln!("runs in which B ends alert: {alert_runs}, passed unseen (not alert): {unseen_runs}");
}

// ---------------------------------------------------------------------
// Properties.
// ---------------------------------------------------------------------

#[test]
fn firing_first_pays() {
    let (mut a_free, mut b_free) = ([0i64; 2], [0i64; 2]);
    for (run, an) in runs() {
        let damage = [total_flotation(run, 0), total_flotation(run, 1)];
        match opening(&an.p.fights[0]) {
            Opening::AFreeRound => {
                a_free[0] += damage[0];
                a_free[1] += damage[1];
            }
            Opening::BFreeRound => {
                b_free[0] += damage[0];
                b_free[1] += damage[1];
            }
            _ => {}
        }
    }
    eprintln!(
        "A free round: flotation damage to A {}, to B {}; B free round: to A {}, to B {}",
        Fixed::from_raw(a_free[0]),
        Fixed::from_raw(a_free[1]),
        Fixed::from_raw(b_free[0]),
        Fixed::from_raw(b_free[1])
    );
    assert!(a_free[1] > a_free[0], "A's free rounds damage B more");
    assert!(b_free[0] > b_free[1], "B's free rounds damage A more");
}

/// Loads the scenario on the frozen figures with each exact, asserted-unique
/// text replacement made in the named file. A change to the scenario is made
/// in its text; a change to a figures file is made in a copy of the frozen
/// reference folder (`common::altered_reference`). Nothing in the repository's
/// data folder is touched. Each edit is (file under data/, original text,
/// changed text).
fn variant(tag: &str, edits: &[(&str, &str, &str)]) -> Scenario {
    const SCENARIO: &str = "scenarios/04a_night_action.toml";
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../data")
        .join(SCENARIO);
    for (target, ..) in edits {
        assert!(
            [
                SCENARIO,
                "reference/ship_classes.csv",
                "reference/surface_combat.toml"
            ]
            .contains(target),
            "unknown file {target}"
        );
    }
    let edits_of = |file: &str| -> Vec<(&str, &str)> {
        edits
            .iter()
            .filter(|(target, ..)| *target == file)
            .map(|(_, original, changed)| (*original, *changed))
            .collect()
    };
    let mut text = fs::read_to_string(&path).expect("read the scenario");
    for (original, changed) in edits_of(SCENARIO) {
        text = common::replace_once(&text, original, changed);
    }
    let options = common::altered_reference(tag, |dir| {
        for file in ["ship_classes.csv", "surface_combat.toml"] {
            let these = edits_of(&format!("reference/{file}"));
            if !these.is_empty() {
                common::edit_file_by(dir, file, &these);
            }
        }
    });
    common::from_text_with(&text, &path, &options).expect("the variant loads")
}

const CLASSES: &str = "reference/ship_classes.csv";
const SCENARIO: &str = "scenarios/04a_night_action.toml";
const CRUISER_ROW: &str =
    "1,Placeholder cruiser,cruiser,16,32,2000,8000,0,8,3,100,20,8,8,200,400,0";

/// The cruiser row with another gun count, gun damage and radar flag.
fn cruiser_row(guns: u32, gun_damage: u32, radar: u32) -> String {
    format!(
        "1,Placeholder cruiser,cruiser,16,32,2000,8000,0,{guns},3,100,{gun_damage},8,8,200,400,{radar}"
    )
}

/// Edits that put one more line, such as `main_battery = "degraded"`, into
/// each of the six ships of the scenario file. The line goes right after the
/// ship's name, inside its `[[ship]]` table.
fn every_ship_has(line: &str) -> Vec<(String, String)> {
    ["A1", "A2", "A3", "B1", "B2", "B3"]
        .iter()
        .map(|name| {
            let original = format!("name = \"Placeholder cruiser {name}\"");
            let changed = format!("{original}\n{line}");
            (original, changed)
        })
        .collect()
}

fn run_events(scenario: &Scenario, seed: u64) -> Vec<Event> {
    let mut run = scenario.with_seed(seed).expect("a seed can be set");
    run.run_to_end().expect("the variant runs");
    run.world.events().iter().cloned().collect()
}

/// The first seed in 1 to 1000 in which both forces sight in round 1, so
/// both fire in round 1 at an unsurprised target that is not yet lit.
fn both_sight_in_round_one() -> u64 {
    analysed_runs()
        .iter()
        .find(|an| {
            [0, 1].iter().all(|force| {
                an.p.fights[0]
                    .rolls
                    .iter()
                    .any(|r| r.round == 1 && r.force == *force && r.sighted)
            })
        })
        .map(|an| an.p.seed)
        .expect("some seed has both sighting in round 1")
}

/// Round 1 of the night fight of a variant, where every ship fires at an
/// unsurprised, unlit target: its salvos of one weapon.
fn round_one_salvos(scenario: &Scenario, seed: u64, weapon: Weapon) -> Vec<SalvoRec> {
    let p = parse(seed, &run_events(scenario, seed));
    p.fights[0]
        .salvos
        .iter()
        .filter(|s| s.round == 1 && s.weapon == weapon)
        .cloned()
        .collect()
}

#[test]
fn more_guns_never_mean_fewer_expected_hits() {
    let seed = both_sight_in_round_one();
    let mut found = Vec::new();
    for guns in [4u32, 8, 12] {
        let row = cruiser_row(guns, 20, 0);
        let scenario = variant(&format!("guns_{guns}"), &[(CLASSES, CRUISER_ROW, &row)]);
        // Round 1: every cruiser fires at an unsurprised, unlit target.
        let round_one = round_one_salvos(&scenario, seed, Weapon::Gun);
        assert_eq!(round_one.len(), 6);
        for salvo in &round_one {
            assert_eq!(salvo.factor("surprised_factor"), Fixed::ONE);
            assert_eq!(salvo.factor("lit_factor"), Fixed::ONE);
            assert_eq!(salvo.factor("main_battery_factor"), Fixed::ONE);
            assert_eq!(salvo.factor("fire_control_factor"), Fixed::ONE);
            assert_eq!(salvo.factor("guns"), fx(i64::from(guns)));
            found.push((guns, salvo.expected));
        }
    }
    let expected = |guns: u32| -> Vec<Fixed> {
        found
            .iter()
            .filter(|(g, _)| *g == guns)
            .map(|(_, e)| *e)
            .collect()
    };
    assert_eq!(expected(4), vec![millionths(1_800_000); 6]);
    assert_eq!(expected(8), vec![millionths(3_600_000); 6]);
    assert_eq!(expected(12), vec![millionths(5_400_000); 6]);
}

#[test]
fn a_damaged_system_never_means_more_expected_hits() {
    let seed = both_sight_in_round_one();
    // Variants: all six ships with one system working, degraded or out. In
    // round 1 each fires at an unsurprised, unlit target: the expected gun
    // hits are 3.6 with the main battery working, 1.8 degraded and no salvo
    // out; with fire control working, degraded and out 3.6, 1.8 and 0.9;
    // and the expected torpedo hits are 0.32, 0.16 and no salvo with the
    // mounts working, degraded and out.
    type Case = (&'static str, Weapon, [Option<i64>; 3]);
    let cases: [Case; 3] = [
        (
            "main_battery",
            Weapon::Gun,
            [Some(3_600_000), Some(1_800_000), None],
        ),
        (
            "fire_control",
            Weapon::Gun,
            [Some(3_600_000), Some(1_800_000), Some(900_000)],
        ),
        (
            "torpedo_mounts",
            Weapon::Torpedo,
            [Some(320_000), Some(160_000), None],
        ),
    ];
    for (system, weapon, wanted) in cases {
        let mut seen: Vec<Option<Fixed>> = Vec::new();
        for (state, wanted) in ["working", "degraded", "out"].iter().zip(wanted) {
            let owned = every_ship_has(&format!("{system} = \"{state}\""));
            let edits: Vec<(&str, &str, &str)> = owned
                .iter()
                .map(|(original, changed)| (SCENARIO, original.as_str(), changed.as_str()))
                .collect();
            let scenario = variant(&format!("{system}_{state}"), &edits);
            let salvos = round_one_salvos(&scenario, seed, weapon);
            match wanted {
                Some(expected) => {
                    assert_eq!(salvos.len(), 6, "{system} {state}");
                    for salvo in &salvos {
                        assert_eq!(salvo.factor("surprised_factor"), Fixed::ONE);
                        assert_eq!(salvo.factor("lit_factor"), Fixed::ONE);
                        assert_eq!(salvo.expected, millionths(expected), "{system} {state}");
                    }
                    seen.push(Some(salvos[0].expected));
                }
                None => {
                    assert!(
                        salvos.is_empty(),
                        "{system} {state}: a salvo from a system out"
                    );
                    seen.push(None);
                }
            }
        }
        // Never more with the system worse: working, degraded, out.
        let value = |slot: &Option<Fixed>| slot.unwrap_or(Fixed::ZERO);
        assert!(
            value(&seen[0]) > value(&seen[1]) && value(&seen[1]) > value(&seen[2]),
            "{system}: {seen:?}"
        );
    }

    // From the 1,000 runs: every gun salvo, grouped by whether it was by
    // night, whether the target was surprised and lit and the firer's two
    // systems. Each group has one expected figure, and a worse system never
    // gives more.
    type Key = (bool, bool, bool, SystemState, SystemState);
    let mut groups: BTreeMap<Key, BTreeSet<Fixed>> = BTreeMap::new();
    // Salvos cut short by too little ammunition are left out: they are a
    // different matter (see `every_salvo_follows_the_figures`).
    let cut: BTreeSet<(u64, u32)> = analysed_runs()
        .iter()
        .flat_map(ammo_cuts)
        .map(|c| (c.seed, c.event))
        .collect();
    for v in fight_views() {
        for salvo in v.f.salvos.iter().filter(|s| s.weapon == Weapon::Gun) {
            if cut.contains(&(v.seed(), salvo.event)) {
                continue;
            }
            let st = &v.w.rounds[usize::try_from(salvo.round - 1).expect("small")];
            let firer = st.start.systems[&salvo.firer];
            let key = (
                v.f.dark(),
                salvo.factor("surprised_factor") != Fixed::ONE,
                salvo.factor("lit_factor") != Fixed::ONE,
                firer.main_battery,
                firer.fire_control,
            );
            groups.entry(key).or_default().insert(salvo.expected);
        }
    }
    let single: Vec<(Key, Fixed)> = groups
        .iter()
        .map(|(key, set)| {
            assert_eq!(set.len(), 1, "{key:?}: {set:?}");
            (*key, *set.iter().next().expect("one"))
        })
        .collect();
    for (a, expected_a) in &single {
        for (b, expected_b) in &single {
            let same_conditions = (a.0, a.1, a.2) == (b.0, b.1, b.2);
            if same_conditions && a.3 <= b.3 && a.4 <= b.4 {
                assert!(
                    expected_a >= expected_b,
                    "{a:?} gives {expected_a}, worse {b:?} gives {expected_b}"
                );
            }
        }
    }
    // The unsurprised, unlit night figures are the table's.
    let night = |main: SystemState, control: SystemState| {
        single
            .iter()
            .find(|(key, _)| *key == (true, false, false, main, control))
            .map(|(_, expected)| *expected)
    };
    for (main, control, expected) in [
        (SystemState::Working, SystemState::Working, 3_600_000),
        (SystemState::Degraded, SystemState::Working, 1_800_000),
        (SystemState::Working, SystemState::Degraded, 1_800_000),
        (SystemState::Working, SystemState::Out, 900_000),
    ] {
        assert_eq!(night(main, control), Some(millionths(expected)));
    }
}

#[test]
fn a_force_is_never_faster_than_its_slowest_ship_afloat() {
    // For each speed A steamed at, the propulsion of its slowest ship afloat.
    let mut speeds: BTreeMap<SystemState, BTreeSet<Fixed>> = BTreeMap::new();
    for (run, an) in runs() {
        let seed = run.seed;
        for step in &an.p.steps {
            let ships: Vec<(u32, SystemState)> = SHIPS[0]
                .iter()
                .copied()
                .filter(|ship| an.p.afloat_before(*ship, step.event))
                .map(|ship| {
                    (
                        ship,
                        an.p.state_before(ship, ShipSystem::Propulsion, step.event),
                    )
                })
                .collect();
            assert!(
                !ships.is_empty(),
                "seed {seed}: a force with no ship steams"
            );
            // Never faster than any ship afloat, and as fast as the slowest.
            for (_, state) in &ships {
                assert!(step.speed <= fx(ship_speed_kn(*state)), "seed {seed}");
            }
            let slowest = ships.iter().map(|(_, state)| *state).max().expect("a ship");
            assert_eq!(step.speed, fx(ship_speed_kn(slowest)), "seed {seed}");
            speeds.entry(slowest).or_default().insert(step.speed);
            // A ship with damaged propulsion is never faster than one
            // without.
            for (_, damaged) in &ships {
                for (_, sound) in &ships {
                    if damaged > sound {
                        assert!(ship_speed_kn(*damaged) <= ship_speed_kn(*sound));
                    }
                }
            }
        }
    }
    // A force steams at 16 kn when its slowest ship has working propulsion,
    // at 8 kn when it is degraded, and does not steam when it is out.
    assert_eq!(
        speeds,
        BTreeMap::from([
            (SystemState::Working, BTreeSet::from([fx(16)])),
            (SystemState::Degraded, BTreeSet::from([fx(8)])),
        ])
    );
    assert!(ship_speed_kn(SystemState::Working) > ship_speed_kn(SystemState::Degraded));
    assert!(ship_speed_kn(SystemState::Degraded) > ship_speed_kn(SystemState::Out));
}

#[test]
fn an_alert_force_never_sights_less_readily() {
    let seed = both_sight_in_round_one();
    // (A not alert, B alert, radar) -> round 1 chances of A and B. In the
    // scenario file A is at battle readiness and B at normal readiness. A is
    // made not alert by setting its readiness to normal; B is made alert by
    // giving it battle readiness (its block has no readiness line).
    let a_not_alert = ("readiness = \"battle\"", "readiness = \"normal\"");
    let b_alert = (
        "q = 5\nr = 0\nmission = \"surface_combat\"",
        "q = 5\nr = 0\nmission = \"surface_combat\"\nreadiness = \"battle\"",
    );
    let mut table: Vec<(bool, bool, [Fixed; 2])> = Vec::new();
    for (flipped, radar) in [(false, false), (true, false), (false, true), (true, true)] {
        let mut edits: Vec<(&str, &str, &str)> = Vec::new();
        let row = cruiser_row(8, 20, 1);
        if flipped {
            // A's edit goes first: B's edit adds a second
            // `readiness = "battle"` line to the text, and A's original
            // line must still appear exactly once when A's edit is made.
            edits.push((SCENARIO, a_not_alert.0, a_not_alert.1));
            edits.push((SCENARIO, b_alert.0, b_alert.1));
        }
        if radar {
            edits.push((CLASSES, CRUISER_ROW, &row));
        }
        let scenario = variant(&format!("sight_{flipped}_{radar}"), &edits);
        let events = run_events(&scenario, seed);
        let p = parse(seed, &events);
        let chance = |force: usize| {
            p.fights[0]
                .rolls
                .iter()
                .find(|r| r.round == 1 && r.force == force)
                .expect("every force rolls in round 1")
                .chance
        };
        table.push((flipped, radar, [chance(0), chance(1)]));
    }
    // Radar adds 0.2 to the base, then not being alert halves the chance.
    assert_eq!(
        table,
        vec![
            (false, false, [millionths(500_000), millionths(250_000)]),
            (true, false, [millionths(250_000), millionths(500_000)]),
            (false, true, [millionths(700_000), millionths(350_000)]),
            (true, true, [millionths(350_000), millionths(700_000)]),
        ]
    );
    for (_, _, [a, b]) in &table {
        assert!(a != b);
    }
}

#[test]
fn no_ammunition_is_negative_no_track_leaves_0_to_100_and_every_ship_has_one_place() {
    for (run, an) in runs() {
        let seed = run.seed;
        let p = &an.p;
        for (ship, end) in &run.ships {
            assert!(
                end.rounds >= Fixed::ZERO && end.rounds <= fx(800),
                "seed {seed}"
            );
            for track in [end.flotation, end.fire] {
                assert!(track >= Fixed::ZERO && track <= fx(100), "seed {seed}");
            }
            let homes: Vec<u32> = run
                .forces
                .iter()
                .filter(|(_, force)| force.ships.contains(ship))
                .map(|(id, _)| *id)
                .collect();
            match end.location {
                ShipLocation::TaskForce(task_force) => {
                    assert_eq!(homes, vec![task_force.raw()], "seed {seed} ship {ship}");
                    assert_eq!(task_force.raw(), if *ship <= 3 { 1 } else { 2 });
                }
                ShipLocation::Sunk => assert!(homes.is_empty(), "seed {seed} ship {ship}"),
                ShipLocation::Port(_) => panic!("seed {seed}: ship {ship} in port"),
                ShipLocation::Yard(_) => panic!("seed {seed}: ship {ship} at the yard"),
            }
            assert_eq!(end.side, SIDES[force_of_ship(*ship)]);
        }
        // Along the way: the model never holds a track outside 0 to 100 or
        // ammunition below 0.
        for w in &an.walks {
            for st in &w.rounds {
                for tracks in [&st.start, &st.end] {
                    for map in [&tracks.flotation, &tracks.fire] {
                        assert!(map.values().all(|v| *v >= Fixed::ZERO && *v <= fx(100)));
                    }
                }
            }
        }
        let mut rounds: BTreeMap<u32, Fixed> = (1..=6).map(|ship| (ship, fx(800))).collect();
        for salvo in p
            .fights
            .iter()
            .flat_map(|f| f.salvos.iter())
            .filter(|s| s.weapon == Weapon::Gun)
        {
            let left = rounds.get_mut(&salvo.firer).expect("a ship");
            *left -= salvo.shots;
            assert!(*left >= Fixed::ZERO, "seed {seed}: negative ammunition");
        }
        // Belief holds nothing the reports did not say: each contact traces
        // to a report and each claim to a claim event.
        for side in Side::ALL {
            for contact in &run.contacts[&side] {
                let report = p
                    .fights
                    .iter()
                    .flat_map(|f| f.reports.iter())
                    .find(|r| r.event == contact.last_event)
                    .expect("a contact comes from a report");
                assert_eq!(
                    (
                        report.side,
                        report.contact,
                        report.hex,
                        report.count,
                        report.kind
                    ),
                    (side, contact.id, contact.hex, contact.count, contact.kind)
                );
            }
            for claim in &run.claims[&side] {
                let sent = p
                    .fights
                    .iter()
                    .flat_map(|f| f.claims.iter())
                    .find(|c| c.event == claim.event)
                    .expect("a claim comes from a claim event");
                assert_eq!(
                    (sent.side, sent.hex, sent.claimed_sunk),
                    (side, claim.hex, claim.claimed_sunk)
                );
            }
        }
    }
}

#[test]
fn a_replay_reproduces_the_daily_hash() {
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
    // The seed matters: different seeds do not all give the same history.
    assert!(all_hashes.windows(2).any(|pair| pair[0] != pair[1]));

    // The file's own seed gives the same hashes as a seeded copy.
    let mut own = load();
    own.run_to_end().expect("the run resolves");
    let mut copy = base.with_seed(1942).expect("a seed can be set");
    copy.run_to_end().expect("the run resolves");
    assert_eq!(own.world.day_hashes(), copy.world.day_hashes());
}

// ---------------------------------------------------------------------
// What the designer needs to see: run with --nocapture.
// ---------------------------------------------------------------------

#[test]
fn ammunition_only_ever_cuts_a_salvo_to_the_rounds_aboard() {
    // A gun salvo fires 120 x the main battery factor, or the rounds aboard
    // if that is fewer. So every salvo that fired fewer fired exactly what
    // the ship had left, and only in a fight by day: one night fight uses
    // 720 of a ship's 800 rounds at most.
    for an in analysed_runs() {
        for cut in ammo_cuts(an) {
            assert_eq!(
                cut.shots, cut.aboard,
                "seed {} ship {}: a short salvo is the rounds aboard",
                cut.seed, cut.ship
            );
            assert!(cut.aboard < cut.full, "seed {}", cut.seed);
            assert_ne!(cut.pulse, Pulse::Night, "seed {}", cut.seed);
        }
    }
}

#[test]
fn print_the_summary() {
    let batch = batch();
    let runs_n = i64::try_from(batch.runs.len()).expect("small");
    let views = fight_views();
    eprintln!("== Scenario 4a summary over seeds 1 to {SEEDS} ==");
    eprintln!("the {runs_n} runs took {:?} in this build", batch.elapsed);

    // The six counts, for the night fight.
    let mut a1 = 0;
    let mut b1 = 0;
    let mut ways = [0i64; 4];
    for an in analysed_runs() {
        let p = &an.p.fights[0];
        a1 += i64::from(
            p.rolls
                .iter()
                .any(|r| r.round == 1 && r.force == 0 && r.sighted),
        );
        b1 += i64::from(
            p.rolls
                .iter()
                .any(|r| r.round == 1 && r.force == 1 && r.sighted),
        );
        ways[match opening(p) {
            Opening::AFreeRound => 0,
            Opening::BFreeRound => 1,
            Opening::BothFire => 2,
            Opening::PassUnseen => 3,
        }] += 1;
    }
    eprintln!("A sights in round 1: {a1} (437..=563)");
    eprintln!("B sights in round 1: {b1} (196..=304)");
    eprintln!("A has a free round: {} (537..=660)", ways[0]);
    eprintln!("B has a free round: {} (149..=249)", ways[1]);
    eprintln!("both fire from the start: {} (149..=249)", ways[2]);
    eprintln!("pass unseen: {} (0..=9)", ways[3]);

    // Fights: how many runs had a second fight, and how many fights in all.
    let mut by_count = [0i64; 4];
    for an in analysed_runs() {
        by_count[an.p.fights.len().min(3)] += 1;
    }
    eprintln!(
        "fights in all: {} in {runs_n} runs; runs with 1 fight: {}, with 2: {}, with 3: {}; runs with a second fight: {}",
        views.len(),
        by_count[1],
        by_count[2],
        by_count[3],
        by_count[2] + by_count[3]
    );

    // Reason by round, for the night fights and then the fights by day.
    let reasons = [
        EngagementEndReason::ForceSunk,
        EngagementEndReason::BrokeOff,
        EngagementEndReason::MaxRounds,
        EngagementEndReason::PassedUnseen,
        EngagementEndReason::SlippedAway,
    ];
    for (title, night) in [("night fights", true), ("fights by day", false)] {
        eprintln!("how the {title} ended, by reason and round:");
        eprintln!("  {:<24} r1   r2   r3   r4   r5   r6  total", "reason");
        for reason in reasons {
            let mut by_round = [0i64; 6];
            for v in views
                .iter()
                .filter(|v| v.f.dark() == night && v.f.ended.reason == reason)
            {
                by_round[usize::try_from(v.f.ended.round - 1).expect("small")] += 1;
            }
            eprintln!(
                "  {:<24} {:>3}  {:>3}  {:>3}  {:>3}  {:>3}  {:>3}  {:>5}",
                reason.name(),
                by_round[0],
                by_round[1],
                by_round[2],
                by_round[3],
                by_round[4],
                by_round[5],
                by_round.iter().sum::<i64>()
            );
        }
    }

    // Ships sunk per side, runs with a sinking.
    for (f, side) in SIDES.iter().enumerate() {
        let sunk: i64 = batch.runs.iter().map(|r| ships_sunk(r, f)).sum();
        let with_sinking = batch.runs.iter().filter(|r| ships_sunk(r, f) > 0).count();
        eprintln!(
            "ships sunk, force {} ({}): {sunk} in {with_sinking} runs",
            if f == 0 { "A" } else { "B" },
            side.name()
        );
    }

    // System hits of each kind per weapon against the expected count.
    let (counts, hits) = system_hit_counts();
    for (weapon, (shares, none_p, name)) in [
        (GUN_SYSTEM_P, GUN_NO_SYSTEM_P, "gun"),
        (TORPEDO_SYSTEM_P, TORPEDO_NO_SYSTEM_P, "torpedo"),
    ]
    .into_iter()
    .enumerate()
    {
        let n = hits[weapon];
        let none = n - counts[weapon].iter().sum::<i64>();
        let mut parts: Vec<String> = ShipSystem::ALL
            .iter()
            .take(4)
            .enumerate()
            .map(|(slot, system)| {
                format!(
                    "{} {} (expected {})",
                    system.name(),
                    counts[weapon][slot],
                    Fixed::from_raw(n * shares[slot])
                )
            })
            .collect();
        parts.push(format!(
            "no system {none} (expected {})",
            Fixed::from_raw(n * none_p)
        ));
        eprintln!(
            "system hits per {name} hit, {n} {name} hits: {}",
            parts.join(", ")
        );
    }

    // Ships that ended with each system degraded and out, of the 6,000.
    for system in ShipSystem::ALL {
        let count = |state: SystemState| {
            batch
                .runs
                .iter()
                .flat_map(|r| r.ships.values())
                .filter(|ship| ship.systems.get(system) == state)
                .count()
        };
        eprintln!(
            "ships ending with {} degraded: {}, out: {}",
            system.name(),
            count(SystemState::Degraded),
            count(SystemState::Out)
        );
    }

    // Force A's speed after the night fight.
    let mut speed = [0i64; 4];
    for an in analysed_runs() {
        speed[match an.walks[0].end.slowest_propulsion(0) {
            Some(SystemState::Working) => 0,
            Some(SystemState::Degraded) => 1,
            Some(SystemState::Out) => 2,
            None => 3,
        }] += 1;
    }
    eprintln!(
        "force A after the night fight: left at 16 kn {}, at 8 kn {}, at 0 kn {}, no ship afloat {}",
        speed[0], speed[1], speed[2], speed[3]
    );

    // Average flotation damage per side by how the fight opened.
    for (name, way) in [
        ("A free round", Opening::AFreeRound),
        ("B free round", Opening::BFreeRound),
        ("both fire from the start", Opening::BothFire),
    ] {
        let members: Vec<&RunData> = runs()
            .filter(|(_, an)| opening(&an.p.fights[0]) == way)
            .map(|(r, _)| r)
            .collect();
        let n = i64::try_from(members.len()).expect("small");
        let totals = [0, 1].map(|f| members.iter().map(|r| total_flotation(r, f)).sum::<i64>());
        eprintln!(
            "average flotation damage, {name} ({n} runs): force A {} / {n} = {}, force B {} / {n} = {}",
            Fixed::from_raw(totals[0]),
            Fixed::from_raw(totals[0] / n),
            Fixed::from_raw(totals[1]),
            Fixed::from_raw(totals[1] / n)
        );
    }

    // One seed where both fired from the start and the fight lasted 3+ rounds.
    let example = analysed_runs().iter().find(|an| {
        opening(&an.p.fights[0]) == Opening::BothFire && an.p.fights[0].ended.round >= 3
    });
    match example {
        Some(an) => eprintln!(
            "example: seed {} - both fired from the start, the fight ended in round {} ({})",
            an.p.seed,
            an.p.fights[0].ended.round,
            an.p.fights[0].ended.reason.name()
        ),
        None => eprintln!("example: no such seed"),
    }

    // Claims and reports, over every fight.
    let mut claims = [0i64; 2];
    let mut truth = [0i64; 2];
    let mut over = 0;
    let mut burning_claimed = 0i64;
    let mut sighted_not_reported = 0;
    let mut reports = [0i64; 2];
    let mut contacts_at_end = 0;
    for (run, an) in runs() {
        for (p, w) in an.p.fights.iter().zip(&an.walks) {
            let last = w.last();
            for f in 0..2 {
                let enemy = 1 - f;
                let sunk_now =
                    i64::try_from(w.rounds[0].afloat[enemy].len() - last.end.afloat(enemy).len())
                        .expect("small");
                for claim in p.claims.iter().filter(|c| c.side == SIDES[f]) {
                    claims[f] += i64::from(claim.claimed_sunk);
                    truth[f] += sunk_now;
                    over += i64::from(i64::from(claim.claimed_sunk) > sunk_now);
                    burning_claimed += i64::try_from(
                        last.end
                            .afloat(enemy)
                            .iter()
                            .filter(|s| last.end.fire[s] >= fx(CLAIMED_FIRE))
                            .count(),
                    )
                    .expect("small");
                }
                reports[f] +=
                    i64::try_from(p.reports.iter().filter(|r| r.side == SIDES[f]).count())
                        .expect("small");
                let fired_on = p.salvos.iter().any(|s| force_of_ship(s.target) == f);
                let rolled_success = p.rolls.iter().any(|r| r.force == f && r.sighted);
                if (fired_on || rolled_success) && !last.sighted[f] {
                    sighted_not_reported += 1;
                }
            }
        }
        for side in SIDES {
            contacts_at_end += run.contacts[&side].len();
        }
    }
    eprintln!(
        "claims: A claimed {} sunk (truth {}), B claimed {} sunk (truth {}); fights where a claim exceeded the truth: {over}",
        claims[0], truth[0], claims[1], truth[1]
    );
    eprintln!("enemy ships claimed only for fire damage of 30 or more: {burning_claimed}");
    eprintln!(
        "sighting reports: A sent {}, B sent {}; contacts held at the end: {contacts_at_end}",
        reports[0], reports[1]
    );
    eprintln!(
        "force-fights fired on but with no round left to sight in, so no report: {sighted_not_reported}"
    );
    // Gun salvos cut short by the ammunition left.
    let cuts: Vec<AmmoCut> = analysed_runs().iter().flat_map(ammo_cuts).collect();
    let gun_salvos: usize = views
        .iter()
        .map(|v| {
            v.f.salvos
                .iter()
                .filter(|s| s.weapon == Weapon::Gun)
                .count()
        })
        .sum();
    let cut_runs: BTreeSet<u64> = cuts.iter().map(|c| c.seed).collect();
    let by_day = cuts.iter().filter(|c| c.pulse != Pulse::Night).count();
    let min_rule = cuts.iter().filter(|c| c.shots == c.aboard).count();
    eprintln!(
        "gun salvos with fewer shots than 120 x the main battery factor, because of the ammunition left: {} of {gun_salvos}, in {} runs, by day {by_day}, shots equal to the rounds aboard in {min_rule}",
        cuts.len(),
        cut_runs.len()
    );
    for cut in cuts.iter().take(10) {
        eprintln!(
            "  seed {} salvo #{} ({} pulse, round {}) ship {}: would fire {}, rounds aboard {}, fired {}",
            cut.seed,
            cut.event,
            ["night", "morning", "afternoon"][pulse_index(cut.pulse)],
            cut.round,
            cut.ship,
            cut.full,
            cut.aboard,
            cut.shots
        );
    }
    let surprised_rounds: usize = views.iter().map(|v| v.f.surprised.len()).sum();
    let salvos: usize = views.iter().map(|v| v.f.salvos.len()).sum();
    let sunk_total: usize = views.iter().map(|v| v.f.sunk.len()).sum();
    eprintln!("salvos {salvos}, surprised forces {surprised_rounds}, ships sunk {sunk_total}");
}
