//! Scenario 8: the soak (decision 0044).
//!
//! `data/scenarios/08_soak.toml` is the full slice of scenario 7 carried on
//! for 180 days from 21 August 1942, with the orders of both sides scripted so
//! that every subsystem stays busy to the end and each kind of running out
//! happens. It is a machine test and not history. There is no hand
//! calculation of outcomes: nothing in it is compared with the record, and no
//! number of a result is asserted. What is claimed is below, and nothing else.
//!
//! 1. **It survives.** The scenario runs its 180 days without a panic and
//!    with every invariant the engine checks at the close of a day holding
//!    (`Scenario::next_day` fails a day that breaks one), for the scenario's
//!    own seed, 1942, and for seeds 1 to `OTHER_SEEDS` in the ordinary test
//!    run. A second test, `#[ignore]`d and run by hand, does the same for
//!    seeds 1 to 100.
//! 2. **Every subsystem stays in use** (decision 2). The 180 days are split
//!    into six windows of 30, and in the seed-1942 run each window has at
//!    least one event of each of these kinds, named by the engine's own
//!    event kinds:
//!
//!    | Activity | Event kinds |
//!    | -------- | ----------- |
//!    | a raid on a base | `BaseStrikeLaunched` |
//!    | an air search flown | `SearchFlown` |
//!    | a strike on ships launched, or found nothing | `StrikeLaunched`, `StrikeFoundNothing` |
//!    | a surface engagement begun | `EngagementBegun` |
//!    | a convoy unloading completed | `UnloadingComplete` |
//!    | an aircraft repair | `AircraftRepaired` |
//!    | an arrival or replacement received | `AircraftArrived`, `ReplacementsReceived` |
//!    | a group stood down | `AirGroupStoodDown` |
//!    | a group returned | `AirGroupReturned` |
//!
//!    Both sides also still have groups flying in the last window.
//! 3. **Things run out, and the run goes on** (decision 5). In the seed-1942
//!    run each of these happens at least once, and the run then carries on
//!    to day 180 with the invariants holding: a base with no supply at the
//!    start of a pulse (and then nothing flies from it); a ship with no
//!    fuel; an air group that had aircraft and has none; an air group with
//!    fewer pilots than aircraft, and one with aircraft and no pilots, each
//!    coming about in the run; a task force that lost every
//!    ship.
//! 4. **Nothing leaves its range** (decision 3). At the close of every day of
//!    every run, no supply, fuel or gun-round count is negative, no fatigue
//!    or rest threshold is outside 0 to 100, no damage track is outside 0 to
//!    100, an air group never has more aircraft or pilots than its strength,
//!    and fuel and gun rounds never rise. Aircraft and pilot counts are
//!    unsigned in the state, so "not negative" for them is held by the types
//!    and by the conservation below.
//! 5. **Conservation** (decision 3), computed from the events and the final
//!    state alone, as the scenario 6 test does, for every run:
//!    - for each air group, what it started with plus what arrived and what
//!      replacements brought, plus what landed elsewhere to it, equals what
//!      it has (ready and damaged) plus what was shot down, destroyed on the
//!      ground, written off, lost with a ship, came down in the sea for want
//!      of a deck (decisions 7 to 10 of 0059) and landed elsewhere from it,
//!      a group formed by a landing starting at 0; for each air group whose
//!      pilots are tracked, the pilots it started with plus those that
//!      arrived, were replaced and came with a landing equal those it has
//!      plus those lost and those that went with a landing;
//!    - for each base, supply started + unloaded = left + spent on flying and
//!      repair + destroyed by bombs + loaded out;
//!    - for each ship, the supply aboard is what it started with plus what it
//!      loaded less what it unloaded, and its fuel is what it started with
//!      less what the fuel events burned.
//! 6. **Determinism.** Seed 1942 run twice gives the same 180 daily hashes
//!    and the same events.
//! 7. **Measurements** (decision 6) are printed by `print_the_summary`:
//!    events in the log in total and by 30-day window, the wall time of one
//!    run, and how both compare with scenario 7's 14 days scaled by 180 / 14.
//! 8. **What the soak found about the engine, on record.** Orders for forces
//!    with no ship afloat and for air groups lost with their ship are rejected
//!    (`NoShipAfloat`, `AirGroupLost`). Ships cannot refuel in port or at sea.
//!    A stranded force ordered again records "ran out of fuel" again and
//!    changes nothing but its route and the order that set it. Two forces
//!    left in one hex with no home port fight in every pulse.
//!
//! The scenario file's comments say what each part of it is for.

mod common;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::time::Instant;

use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::state::{ShipLocation, TruthState};
use sim_data::Scenario;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
const SOAK: &str = "08_soak.toml";
const FULL_SLICE: &str = "07_full_slice.toml";
const DAYS: usize = 180;
const WINDOW: usize = 30;
const WINDOWS: usize = DAYS / WINDOW;
/// Seeds 1 to this are run in the ordinary test run, besides 1942.
const OTHER_SEEDS: u64 = 48;

/// The air groups of the two bases (scenario 7's and the soak's own).
const RABAUL_GROUPS: [u32; 4] = [1, 2, 3, 5];
const HENDERSON_GROUPS: [u32; 10] = [6, 7, 10, 11, 12, 13, 14, 15, 16, 17];

fn scenario_path(name: &str) -> PathBuf {
    Path::new(ROOT).join("data/scenarios").join(name)
}

/// Where the frozen copy of a scenario is, `data/scenarios/step2-frozen`: the
/// runs on the frozen figures read the frozen copies of scenarios 7 and 8, as
/// decision 0073 and the folder's README say (frozen figures go with frozen
/// scenarios). The runs on the live figures read the live files.
fn frozen_scenario_path(name: &str) -> PathBuf {
    Path::new(ROOT)
        .join("data/scenarios/step2-frozen")
        .join(name)
}

/// Which figures a soak runs on. The tests that assert only that the run
/// completes, repeats, stays in range and conserves what it should (claims 1,
/// 4 and 5 and the repeat) run on the live figures, so that the figures step 2
/// tunes are held to them. The tests that assert what the soak's scripted
/// orders make happen at the figures of step 1 (every subsystem in use, each
/// kind of running out, the stranded forces, and the like) run on the frozen
/// figures of `data/reference/step1`.
#[derive(Clone, Copy)]
enum Figures {
    Frozen,
    Live,
}

fn load(name: &str, seed: u64) -> Scenario {
    load_on(name, seed, Figures::Frozen)
}

fn load_on(name: &str, seed: u64, figures: Figures) -> Scenario {
    match figures {
        Figures::Frozen => common::load(&frozen_scenario_path(name)),
        Figures::Live => common::load_live(&scenario_path(name)),
    }
    .expect("the scenario loads")
    .with_seed(seed)
    .expect("a seed can be set")
}

fn hundred() -> Fixed {
    Fixed::from_int(100)
}

/// A wall-clock stopwatch for the measurements of decision 6. The workspace
/// forbids `Instant::now` so that nothing in the simulation can read the clock
/// (clippy.toml). This is the one place a test reads it, to time a run from
/// outside: no rule, order or assertion depends on what it says.
struct Stopwatch(Instant);

impl Stopwatch {
    #[allow(clippy::disallowed_methods)]
    fn start() -> Stopwatch {
        Stopwatch(Instant::now())
    }

    fn seconds(&self) -> f64 {
        self.0.elapsed().as_secs_f64()
    }
}

fn day_of(event: &Event) -> usize {
    usize::try_from(event.day).expect("a day fits")
}

// ---------------------------------------------------------------------
// What one run leaves.
// ---------------------------------------------------------------------

/// An air group at the close of a day.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GroupState {
    ready: u32,
    damaged: u32,
    pilots: Option<u32>,
    strength: u32,
}

impl GroupState {
    fn aircraft(&self) -> u32 {
        self.ready + self.damaged
    }
}

/// The part of the truth that the checks read, at one moment.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
struct Picture {
    groups: BTreeMap<u32, GroupState>,
    base_supply: BTreeMap<u32, Fixed>,
    ship_fuel: BTreeMap<u32, Fixed>,
    ship_supply: BTreeMap<u32, Fixed>,
    /// Task forces with no ship.
    empty_forces: BTreeSet<u32>,
    /// Ships afloat with no fuel.
    dry_ships: BTreeSet<u32>,
}

fn picture(truth: &TruthState) -> Picture {
    Picture {
        groups: truth
            .air_groups
            .values()
            .map(|g| {
                (
                    g.id.raw(),
                    GroupState {
                        ready: g.ready,
                        damaged: g.damaged,
                        pilots: g.pilots,
                        strength: g.strength,
                    },
                )
            })
            .collect(),
        base_supply: truth
            .bases
            .values()
            .map(|b| (b.id.raw(), b.supply_t))
            .collect(),
        ship_fuel: truth
            .ships
            .values()
            .map(|s| (s.id.raw(), s.fuel_t))
            .collect(),
        ship_supply: truth
            .ships
            .values()
            .map(|s| (s.id.raw(), s.supply_t))
            .collect(),
        empty_forces: truth
            .task_forces
            .values()
            .filter(|t| t.ships.is_empty())
            .map(|t| t.id.raw())
            .collect(),
        dry_ships: truth
            .ships
            .values()
            .filter(|s| s.location != ShipLocation::Sunk && s.fuel_t.is_zero())
            .map(|s| s.id.raw())
            .collect(),
    }
}

/// What one run left: the day's hash and picture for every day, the whole
/// event log and the time the days took.
struct Run {
    seed: u64,
    start: Picture,
    hashes: Vec<u64>,
    days: Vec<Picture>,
    events: Vec<Event>,
    seconds: f64,
    /// The time each day took to resolve, in seconds.
    day_seconds: Vec<f64>,
}

impl Run {
    fn end(&self) -> &Picture {
        self.days.last().expect("a run has days")
    }
}

/// Every count and track of the truth is inside its range, and what only ever
/// falls has not risen since `before`. Returns the problems found.
fn out_of_range(truth: &TruthState, before: Option<&TruthState>) -> Vec<String> {
    let mut problems = Vec::new();
    for ship in truth.ships.values() {
        let id = ship.id;
        if ship.fuel_t.is_negative() {
            problems.push(format!("{id} has negative fuel {}", ship.fuel_t));
        }
        if ship.supply_t.is_negative() {
            problems.push(format!("{id} has negative supply {}", ship.supply_t));
        }
        if ship.gun_rounds.is_negative() {
            problems.push(format!("{id} has negative gun rounds {}", ship.gun_rounds));
        }
        for (name, track) in [
            ("flotation", ship.flotation_damage),
            ("fire", ship.fire_damage),
        ] {
            if track < Fixed::ZERO || track > hundred() {
                problems.push(format!("{id} has {name} damage {track}"));
            }
        }
        if let Some(old) = before.and_then(|b| b.ships.get(&ship.id)) {
            // A ship's fuel only falls, unless a base had fuel to give it the
            // day before (decision 0045). The soak's bases have none, so for
            // it nothing changes; scenario 7's two have, for its convoy.
            let a_base_had_fuel =
                before.is_some_and(|b| b.bases.values().any(|base| base.fuel_t > Fixed::ZERO));
            if ship.fuel_t > old.fuel_t && !a_base_had_fuel {
                problems.push(format!(
                    "{id} gained fuel: {} to {}",
                    old.fuel_t, ship.fuel_t
                ));
            }
            if ship.gun_rounds > old.gun_rounds {
                problems.push(format!(
                    "{id} gained gun rounds: {} to {}",
                    old.gun_rounds, ship.gun_rounds
                ));
            }
            if old.location == ShipLocation::Sunk && ship.location != ShipLocation::Sunk {
                problems.push(format!("{id} came back from the dead"));
            }
        }
    }
    for task_force in truth.task_forces.values() {
        if task_force.fatigue < Fixed::ZERO || task_force.fatigue > hundred() {
            problems.push(format!(
                "{} has fatigue {}",
                task_force.id, task_force.fatigue
            ));
        }
    }
    for base in truth.bases.values() {
        if base.supply_t.is_negative() {
            problems.push(format!("{} has negative supply {}", base.id, base.supply_t));
        }
    }
    for group in truth.air_groups.values() {
        let id = group.id;
        if group.fatigue < Fixed::ZERO || group.fatigue > hundred() {
            problems.push(format!("{id} has fatigue {}", group.fatigue));
        }
        if let Some(rest) = group.rest_above {
            if rest < Fixed::ZERO || rest > hundred() {
                problems.push(format!("{id} has rest threshold {rest}"));
            }
        }
        if group.ready + group.damaged > group.strength {
            problems.push(format!(
                "{id} has {} aircraft over its strength {}",
                group.ready + group.damaged,
                group.strength
            ));
        }
        if group.pilots.is_some_and(|p| p > group.strength) {
            problems.push(format!("{id} has more pilots than its strength"));
        }
    }
    problems
}

/// Runs a scenario for its days and checks the ranges at the close of each.
fn run(name: &str, seed: u64) -> Run {
    run_on(name, seed, Figures::Frozen)
}

fn run_on(name: &str, seed: u64, figures: Figures) -> Run {
    let mut scenario = load_on(name, seed, figures);
    let start = picture(scenario.world.umpire());
    let mut hashes = Vec::new();
    let mut days = Vec::new();
    let mut before: Option<TruthState> = None;
    let started = Stopwatch::start();
    let mut day_seconds = Vec::new();
    let mut day_started = Stopwatch::start();
    while let Some(summary) = scenario
        .next_day()
        .unwrap_or_else(|e| panic!("seed {seed}: the day does not resolve: {e}"))
    {
        day_seconds.push(day_started.seconds());
        hashes.push(summary.state_hash);
        let truth = scenario.world.umpire();
        let problems = out_of_range(truth, before.as_ref());
        assert!(
            problems.is_empty(),
            "seed {seed}, day {}: {problems:?}",
            summary.day
        );
        days.push(picture(truth));
        before = Some(truth.clone());
        day_started = Stopwatch::start();
    }
    let seconds = started.seconds();
    Run {
        seed,
        start,
        hashes,
        days,
        events: scenario.world.events().iter().cloned().collect(),
        seconds,
        day_seconds,
    }
}

// ---------------------------------------------------------------------
// Activities, window by window.
// ---------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Activity {
    RaidOnABase,
    AirSearchFlown,
    StrikeOnShips,
    SurfaceEngagementBegun,
    ConvoyUnloaded,
    AircraftRepaired,
    ArrivalOrReplacement,
    GroupStoodDown,
    GroupReturned,
}

const ACTIVITIES: [(Activity, &str); 9] = [
    (Activity::RaidOnABase, "raid on a base"),
    (Activity::AirSearchFlown, "air search flown"),
    (Activity::StrikeOnShips, "strike on ships"),
    (Activity::SurfaceEngagementBegun, "surface engagement"),
    (Activity::ConvoyUnloaded, "convoy unloaded"),
    (Activity::AircraftRepaired, "aircraft repair"),
    (Activity::ArrivalOrReplacement, "arrival or replacement"),
    (Activity::GroupStoodDown, "group stood down"),
    (Activity::GroupReturned, "group returned"),
];

fn activity(kind: &EventKind) -> Option<Activity> {
    Some(match kind {
        EventKind::BaseStrikeLaunched { .. } => Activity::RaidOnABase,
        EventKind::SearchFlown { .. } => Activity::AirSearchFlown,
        EventKind::StrikeLaunched { .. } | EventKind::StrikeFoundNothing { .. } => {
            Activity::StrikeOnShips
        }
        EventKind::EngagementBegun { .. } => Activity::SurfaceEngagementBegun,
        EventKind::UnloadingComplete { .. } => Activity::ConvoyUnloaded,
        EventKind::AircraftRepaired { .. } => Activity::AircraftRepaired,
        EventKind::AircraftArrived { .. } | EventKind::ReplacementsReceived { .. } => {
            Activity::ArrivalOrReplacement
        }
        EventKind::AirGroupStoodDown { .. } => Activity::GroupStoodDown,
        EventKind::AirGroupReturned { .. } => Activity::GroupReturned,
        _ => return None,
    })
}

/// The window, from 0, of a day from 1 (events before day 1 are in none).
fn window_of(day: usize) -> Option<usize> {
    (1..=DAYS).contains(&day).then(|| (day - 1) / WINDOW)
}

fn activity_counts(run: &Run) -> BTreeMap<Activity, [u32; WINDOWS]> {
    let mut counts: BTreeMap<Activity, [u32; WINDOWS]> =
        ACTIVITIES.iter().map(|(a, _)| (*a, [0; WINDOWS])).collect();
    for event in &run.events {
        if let (Some(a), Some(w)) = (activity(&event.kind), window_of(day_of(event))) {
            counts.get_mut(&a).expect("every activity is there")[w] += 1;
        }
    }
    counts
}

fn events_by_window(run: &Run) -> [usize; WINDOWS] {
    let mut counts = [0; WINDOWS];
    for event in &run.events {
        if let Some(w) = window_of(day_of(event)) {
            counts[w] += 1;
        }
    }
    counts
}

// ---------------------------------------------------------------------
// Conservation, from the events and the final state alone.
// ---------------------------------------------------------------------

fn assert_conserved(run: &Run) {
    let seed = run.seed;
    let end = run.end();
    // Every group at the end, those formed by a landing elsewhere (decisions 7
    // to 10 of 0059) included: a formed group starts at 0.
    for (g, now) in &end.groups {
        let now = *now;
        let start = run.start.groups.get(g).copied().unwrap_or(GroupState {
            ready: 0,
            damaged: 0,
            pilots: now.pilots.map(|_| 0),
            strength: 0,
        });
        let start = &start;
        let (mut arrived, mut received, mut shot, mut ground, mut written_off, mut with_ship) =
            (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        let (mut pilots_arrived, mut pilots_received, mut pilots_lost) = (0u32, 0u32, 0u32);
        // What ran out of fuel on the way home from a sortie beyond the
        // radius (decision 3 of 0059), what came down in the sea for want of a
        // deck, and what landed elsewhere from the group and to it, with the
        // pilots that went along.
        let mut fuel = 0u32;
        let (mut no_deck, mut landed_out, mut landed_in) = (0u32, 0u32, 0u32);
        let (mut pilots_out, mut pilots_in) = (0u32, 0u32);
        for event in &run.events {
            match event.kind {
                EventKind::AircraftArrived {
                    air_group,
                    aircraft,
                    pilots,
                    ..
                } if air_group.raw() == *g => {
                    arrived += aircraft;
                    pilots_arrived += pilots;
                }
                EventKind::ReplacementsReceived {
                    air_group,
                    aircraft,
                    pilots,
                    ..
                } if air_group.raw() == *g => {
                    received += aircraft;
                    pilots_received += pilots;
                }
                EventKind::AircraftLost {
                    air_group,
                    aircraft,
                    ..
                } if air_group.raw() == *g => shot += aircraft,
                EventKind::AircraftDestroyedOnGround {
                    air_group,
                    aircraft,
                    ..
                }
                | EventKind::DamagedAircraftDestroyedOnGround {
                    air_group,
                    aircraft,
                    ..
                } if air_group.raw() == *g => ground += aircraft,
                EventKind::AircraftWorn {
                    air_group,
                    written_off: w,
                    ..
                } if air_group.raw() == *g => written_off += w,
                EventKind::AirGroupLost {
                    air_group,
                    aircraft,
                    ..
                } if air_group.raw() == *g => with_ship += aircraft,
                EventKind::PilotsLost {
                    air_group, lost, ..
                } if air_group.raw() == *g => pilots_lost += lost,
                EventKind::AircraftDitched {
                    air_group, ditched, ..
                } if air_group.raw() == *g => fuel += ditched,
                EventKind::AircraftDitchedNoDeck {
                    air_group,
                    aircraft,
                    ..
                } if air_group.raw() == *g => no_deck += aircraft,
                EventKind::AircraftLandedElsewhere {
                    from,
                    to,
                    aircraft,
                    pilots,
                    ..
                } => {
                    if from.raw() == *g {
                        landed_out += aircraft;
                        pilots_out += pilots;
                    }
                    if to.raw() == *g {
                        landed_in += aircraft;
                        pilots_in += pilots;
                    }
                }
                _ => {}
            }
        }
        assert_eq!(
            start.aircraft() + arrived + received + landed_in,
            now.aircraft() + shot + ground + written_off + with_ship + fuel + no_deck + landed_out,
            "seed {seed}, air group {g}: aircraft started {} + arrived {arrived} + received \
             {received} + landed to it {landed_in} = has {} + shot {shot} + on the ground \
             {ground} + written off {written_off} + lost with a ship {with_ship} + out of fuel \
             {fuel} + no deck {no_deck} + landed from it {landed_out}",
            start.aircraft(),
            now.aircraft()
        );
        match (start.pilots, now.pilots) {
            (Some(p), Some(q)) => assert_eq!(
                p + pilots_arrived + pilots_received + pilots_in,
                q + pilots_lost + pilots_out,
                "seed {seed}, air group {g}: pilots"
            ),
            (None, None) => {}
            other => panic!("seed {seed}, air group {g}: pilots tracked changed: {other:?}"),
        }
    }

    for (b, start) in &run.start.base_supply {
        let (mut unloaded, mut loaded, mut paid, mut destroyed) =
            (Fixed::ZERO, Fixed::ZERO, Fixed::ZERO, Fixed::ZERO);
        for event in &run.events {
            match event.kind {
                EventKind::CargoUnloaded {
                    base, unloaded_t, ..
                } if base.raw() == *b => unloaded += unloaded_t,
                EventKind::CargoLoaded { base, loaded_t, .. } if base.raw() == *b => {
                    loaded += loaded_t;
                }
                EventKind::BaseSupplySpent { base, paid_t, .. } if base.raw() == *b => {
                    paid += paid_t;
                }
                EventKind::BaseSupplyDestroyed {
                    base, destroyed_t, ..
                } if base.raw() == *b => destroyed += destroyed_t,
                _ => {}
            }
        }
        assert_eq!(
            *start + unloaded,
            end.base_supply[b] + paid + destroyed + loaded,
            "seed {seed}, base {b}: supply started {start} + unloaded {unloaded} = left {} + \
             spent {paid} + destroyed {destroyed} + loaded out {loaded}",
            end.base_supply[b]
        );
    }

    for (s, start) in &run.start.ship_supply {
        let (mut loaded, mut unloaded) = (Fixed::ZERO, Fixed::ZERO);
        for event in &run.events {
            match event.kind {
                EventKind::CargoLoaded { ship, loaded_t, .. } if ship.raw() == *s => {
                    loaded += loaded_t;
                }
                EventKind::CargoUnloaded {
                    ship, unloaded_t, ..
                } if ship.raw() == *s => unloaded += unloaded_t,
                _ => {}
            }
        }
        assert_eq!(
            *start + loaded,
            end.ship_supply[s] + unloaded,
            "seed {seed}, ship {s}: supply aboard"
        );
    }
    for (s, start) in &run.start.ship_fuel {
        let mut burned = Fixed::ZERO;
        for event in &run.events {
            if let EventKind::FuelBurned { ship, burned_t, .. } = event.kind {
                if ship.raw() == *s {
                    burned += burned_t;
                }
            }
        }
        assert_eq!(
            *start,
            end.ship_fuel[s] + burned,
            "seed {seed}, ship {s}: fuel"
        );
    }
}

/// Every check that holds of every run of any seed.
fn check_every_run(run: &Run) {
    assert_eq!(run.hashes.len(), DAYS, "seed {}", run.seed);
    assert_eq!(run.days.len(), DAYS, "seed {}", run.seed);
    assert_conserved(run);
}

/// Runs seeds `first..=last` on as many threads as the machine has, each seed
/// being its own run (so the result does not depend on the threads).
fn run_seeds(first: u64, last: u64) -> Vec<Run> {
    let seeds: Vec<u64> = (first..=last).collect();
    let threads = std::thread::available_parallelism()
        .map_or(1, std::num::NonZeroUsize::get)
        .min(seeds.len().max(1));
    let mut runs: Vec<Run> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads)
            .map(|t| {
                let mine: Vec<u64> = seeds.iter().copied().skip(t).step_by(threads).collect();
                scope.spawn(move || {
                    mine.into_iter()
                        .map(|seed| {
                            let one = run_on(SOAK, seed, Figures::Live);
                            check_every_run(&one);
                            one
                        })
                        .collect::<Vec<Run>>()
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|h| h.join().expect("a seed's run does not panic"))
            .collect()
    });
    runs.sort_by_key(|r| r.seed);
    runs
}

// ---------------------------------------------------------------------
// The tests.
// ---------------------------------------------------------------------

#[test]
fn the_scenarios_own_seed_runs_180_days_with_everything_holding() {
    let one = run_on(SOAK, 1942, Figures::Live);
    check_every_run(&one);
    let distinct: BTreeSet<u64> = one.hashes.iter().copied().collect();
    assert_eq!(distinct.len(), DAYS, "every day changes the state");
}

#[test]
fn other_seeds_run_180_days_with_everything_holding() {
    let runs = run_seeds(1, OTHER_SEEDS);
    assert_eq!(runs.len(), usize::try_from(OTHER_SEEDS).expect("small"));
    let ends: BTreeSet<u64> = runs
        .iter()
        .map(|r| *r.hashes.last().expect("days"))
        .collect();
    assert!(ends.len() > 1, "the seed matters");
}

/// Seeds 1 to 100, in release mode by hand:
/// `cargo test --release -p scenario-tests --test scenario_08_soak -- --ignored`
#[test]
#[ignore = "a hundred 180-day runs; run by hand"]
fn a_hundred_seeds_run_180_days_with_everything_holding() {
    let started = Stopwatch::start();
    let runs = run_seeds(1, 100);
    let events: usize = runs.iter().map(|r| r.events.len()).sum();
    eprintln!("100 seeds, {events} events, {:.1} s", started.seconds());
    assert_eq!(runs.len(), 100);
}

#[test]
fn seed_1942_run_twice_gives_the_same_180_daily_hashes_and_events() {
    let a = run_on(SOAK, 1942, Figures::Live);
    let b = run_on(SOAK, 1942, Figures::Live);
    assert_eq!(a.hashes.len(), DAYS);
    assert_eq!(a.hashes, b.hashes, "the daily hashes differ");
    assert_eq!(a.events.len(), b.events.len(), "the logs differ in length");
    for (x, y) in a.events.iter().zip(&b.events) {
        assert_eq!(x, y, "events differ");
    }
    let other = run_on(SOAK, 1, Figures::Live);
    assert_ne!(a.hashes, other.hashes, "the seed matters");
}

#[test]
fn every_subsystem_is_in_use_in_every_30_days() {
    let one = run(SOAK, 1942);
    let counts = activity_counts(&one);
    let mut missing = Vec::new();
    for (activity, name) in ACTIVITIES {
        for (w, n) in counts[&activity].iter().enumerate() {
            if *n == 0 {
                missing.push(format!(
                    "no {name} in days {} to {}",
                    w * WINDOW + 1,
                    (w + 1) * WINDOW
                ));
            }
        }
    }
    assert!(missing.is_empty(), "{missing:#?}");
}

/// The first day, from 1, on which `holds` is true of the close of the day.
fn first_day(one: &Run, holds: impl Fn(&Picture) -> bool) -> Option<usize> {
    one.days.iter().position(holds).map(|i| i + 1)
}

#[test]
fn each_kind_of_running_out_happens_and_leaves_the_run_going() {
    let one = run(SOAK, 1942);

    // A base with no supply at the start of a pulse: its stock is nothing at
    // the close of a day. It has air groups, and nothing flies from it after
    // that day.
    let grounded_day = first_day(&one, |p| p.base_supply.get(&2).is_some_and(|s| s.is_zero()))
        .expect("Rabaul runs out of supply");
    assert!(
        grounded_day < DAYS,
        "Rabaul runs out on day {grounded_day}, before the last day"
    );
    for event in &one.events {
        if day_of(event) > grounded_day {
            match &event.kind {
                EventKind::BaseStrikeLaunched { air_group, .. }
                | EventKind::SearchFlown { air_group, .. } => {
                    assert!(
                        !RABAUL_GROUPS.contains(&air_group.raw()),
                        "day {}: a group at Rabaul flies after its stock is gone",
                        event.day
                    );
                }
                _ => {}
            }
        }
    }

    // A ship out of fuel: the engine says so, and a ship afloat has none.
    let out_of_fuel_day = one
        .events
        .iter()
        .find(|e| matches!(e.kind, EventKind::TaskForceOutOfFuel { .. }))
        .map(day_of)
        .expect("a task force runs out of fuel");
    let dry_day = first_day(&one, |p| !p.dry_ships.is_empty()).expect("a ship has no fuel");

    // An air group that had aircraft and has none.
    let destroyed_day = first_day(&one, |p| {
        p.groups.iter().any(|(g, now)| {
            one.start
                .groups
                .get(g)
                .is_some_and(|start| start.aircraft() > 0)
                && now.aircraft() == 0
        })
    })
    .expect("an air group is destroyed outright");
    // The same for a group ashore (pilots are tracked only ashore).
    let destroyed_ashore_day = first_day(&one, |p| {
        p.groups.iter().any(|(g, now)| {
            one.start
                .groups
                .get(g)
                .is_some_and(|start| start.pilots.is_some() && start.aircraft() > 0)
                && now.aircraft() == 0
        })
    })
    .expect("an air group ashore is destroyed outright");

    // An air group with fewer pilots than aircraft, and one with aircraft and
    // no pilots, each coming about in the run: a group that did not have that
    // at the close of the day before has it at the close of this one.
    let comes_about = |holds: &dyn Fn(&GroupState) -> bool| {
        (1..DAYS)
            .find(|&i| {
                one.days[i]
                    .groups
                    .iter()
                    .any(|(g, now)| holds(now) && !one.days[i - 1].groups.get(g).is_some_and(holds))
            })
            .map(|i| i + 1)
    };
    let fewer_pilots_day = comes_about(&|g| g.pilots.is_some_and(|n| n > 0 && n < g.aircraft()))
        .expect("an air group comes to have fewer pilots than aircraft");
    let no_pilots_day = comes_about(&|g| g.pilots == Some(0) && g.aircraft() > 0)
        .expect("an air group comes to have aircraft and no pilots");

    // A task force with every ship sunk.
    let wiped_day = first_day(&one, |p| {
        p.empty_forces
            .difference(&one.start.empty_forces)
            .next()
            .is_some()
    })
    .expect("a task force loses every ship");

    eprintln!(
        "first days: Rabaul out of supply {grounded_day}; task force out of fuel \
         {out_of_fuel_day}; a ship at 0 fuel {dry_day}; an air group with no aircraft \
         {destroyed_day}, ashore {destroyed_ashore_day}; fewer pilots than aircraft \
         {fewer_pilots_day}; aircraft and no pilots {no_pilots_day}; a task force with \
         every ship sunk {wiped_day}"
    );

    // The run goes on: after the last of these days there are days left, and
    // they have events.
    let last_first = [
        grounded_day,
        out_of_fuel_day,
        dry_day,
        destroyed_day,
        destroyed_ashore_day,
        fewer_pilots_day,
        no_pilots_day,
        wiped_day,
    ]
    .into_iter()
    .max()
    .expect("some days");
    assert!(last_first < DAYS, "the last of them is on day {last_first}");
    let after = one.events.iter().filter(|e| day_of(e) > last_first).count();
    assert!(after > 0, "nothing happens after day {last_first}");
}

#[test]
fn both_sides_still_have_aircraft_flying_in_the_last_30_days() {
    let one = run(SOAK, 1942);
    let in_last_window = |e: &&Event| window_of(day_of(e)) == Some(WINDOWS - 1);
    let flown_by = |groups: &[u32]| {
        one.events
            .iter()
            .filter(in_last_window)
            .filter(|e| match &e.kind {
                EventKind::BaseStrikeLaunched { air_group, .. }
                | EventKind::SearchFlown { air_group, .. }
                | EventKind::StrikeLaunched { air_group, .. } => groups.contains(&air_group.raw()),
                _ => false,
            })
            .count()
    };
    assert!(
        flown_by(&RABAUL_GROUPS) > 0,
        "Japanese groups at Rabaul fly in the last 30 days"
    );
    assert!(
        flown_by(&HENDERSON_GROUPS) > 0,
        "Allied groups at Henderson Field fly in the last 30 days"
    );
}

// ---------------------------------------------------------------------
// What the soak found about the engine, on record.
// ---------------------------------------------------------------------

/// Orders for units that have nothing left are rejected, with their own
/// reasons: forces whose every ship is sunk (forces 7 and 8 of scenario 7,
/// lost on day 7) and the air groups of Ryujo, which went down with her on
/// day 4. The soak's section E sends them orders on days 20 to 22 and 60.
#[test]
fn orders_to_units_with_nothing_left_are_rejected() {
    use sim_core::ids::EntityRef;
    use sim_core::orders::RejectReason;
    let one = run(SOAK, 1942);
    let rejected = |wanted: &RejectReason| -> Vec<(usize, EntityRef)> {
        one.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::OrderRejected { target, reason, .. } if reason == wanted => {
                    Some((day_of(e), *target))
                }
                _ => None,
            })
            .collect()
    };
    let force = |id: u32| EntityRef::TaskForce(sim_core::ids::TaskForceId::new(id));
    let group = |id: u32| EntityRef::AirGroup(sim_core::ids::AirGroupId::new(id));
    let forces = rejected(&RejectReason::NoShipAfloat);
    for wanted in [(20, 7), (21, 7), (22, 8), (60, 7)] {
        assert!(
            forces.contains(&(wanted.0, force(wanted.1))),
            "day {}: no rejection for task force {}",
            wanted.0,
            wanted.1
        );
    }
    let groups = rejected(&RejectReason::AirGroupLost);
    for wanted in [(21, 118), (21, 119)] {
        assert!(
            groups.contains(&(wanted.0, group(wanted.1))),
            "day {}: no rejection for air group {}",
            wanted.0,
            wanted.1
        );
    }
    // Every such rejection is for a unit that has nothing: a force with no
    // ship in the picture of the day before, a group aboard a ship that was
    // sunk. (Orders are taken in at the start of a day, so the day before is
    // the state they met.)
    for (day, target) in &forces {
        let EntityRef::TaskForce(id) = target else {
            panic!("a rejection for no ship afloat names a force")
        };
        assert!(
            one.days[day - 2].empty_forces.contains(&id.raw()),
            "day {day}: {id} had a ship afloat"
        );
    }
    // And nothing else accepted an order for one of them after: the standing
    // orders of the dead forces stay as they were (no route for force 7).
    let accepted_for_dead = one.events.iter().any(|e| match &e.kind {
        EventKind::OrderAccepted { target, .. } => {
            day_of(e) >= 20 && (*target == force(7) || *target == force(8))
        }
        _ => false,
    });
    assert!(!accepted_for_dead, "an order for a dead force was accepted");
}

/// Ships cannot refuel, anywhere: a ship's fuel only falls (checked for every
/// ship on every day of every run), and a cargo ship that has loaded and
/// unloaded in port again and again has less fuel at the end than at the
/// start, by exactly what it burned.
#[test]
fn ships_cannot_refuel_in_port_or_at_sea() {
    let one = run(SOAK, 1942);
    // Convoy 22's cargo ships are ships 82 and 83 (convoy 21's are 80 and
    // 81, the raiders' are 60 to 79).
    for ship in [80u32, 82] {
        let in_port = one
            .events
            .iter()
            .filter(|e| matches!(e.kind, EventKind::CargoLoaded { ship: s, .. } if s.raw() == ship))
            .count();
        assert!(in_port >= 8, "ship {ship} loaded only {in_port} times");
        assert!(one.end().ship_fuel[&ship] < one.start.ship_fuel[&ship]);
        // Day by day, never more than the day before.
        let mut before = one.start.ship_fuel[&ship];
        for (i, day) in one.days.iter().enumerate() {
            assert!(
                day.ship_fuel[&ship] <= before,
                "day {}: ship {ship} gained fuel",
                i + 1
            );
            before = day.ship_fuel[&ship];
        }
    }
}

/// A force that has run out of fuel and is ordered again has the order
/// refused as `OutOfFuel` (differences.md row 34). It records no second
/// out-of-fuel event, and nothing about it changes: its status, position,
/// leg, route and route order are the day before's.
#[test]
fn a_stranded_force_ordered_again_is_refused_as_out_of_fuel_and_nothing_changes() {
    use sim_core::ids::{EntityRef, TaskForceId};
    use sim_core::orders::RejectReason;
    use sim_core::state::TaskForceStatus;
    let convoy = TaskForceId::new(5);
    let mut scenario = load(SOAK, 1942);
    let mut stranded = None;
    let mut seen_out = 0;
    while let Some(summary) = scenario.next_day().expect("the day resolves") {
        let events = scenario.world.events();
        let out_of_fuel = events
            .iter()
            .filter(|e| matches!(e.kind, EventKind::TaskForceOutOfFuel { task_force, .. } if task_force == convoy))
            .count();
        let now = scenario.world.umpire().task_forces[&convoy].clone();
        if let Some(before) = &stranded {
            let refused = events.iter().any(|e| {
                day_of(e) == usize::try_from(summary.day).expect("a day")
                    && matches!(
                        &e.kind,
                        EventKind::OrderRejected {
                            target,
                            reason: RejectReason::OutOfFuel,
                            ..
                        } if *target == EntityRef::TaskForce(convoy)
                    )
            });
            if refused {
                // This day's order was refused and nothing was recorded or
                // changed for the force.
                let before: &sim_core::state::TaskForce = before;
                assert_eq!(out_of_fuel, seen_out, "a second out-of-fuel event");
                assert_eq!(now.status, TaskForceStatus::OutOfFuel);
                assert_eq!(&now, before, "the refused order changed the force");
                return;
            }
        }
        seen_out = out_of_fuel;
        stranded = (now.status == TaskForceStatus::OutOfFuel).then_some(now);
    }
    panic!("the convoy was never ordered again after running dry");
}

/// Two forces left in one hex after a fight fight again in every pulse, for
/// ever, when no force has a home port to retire to (decision 0016, 14). The
/// soak's own pairs are ordered to part two days after they meet; with those
/// orders left out the same run has pairs that meet again and again.
#[test]
fn two_forces_left_in_one_hex_with_no_home_port_fight_in_every_pulse() {
    use sim_core::ids::TaskForceId;
    let most_meetings = |run: &Run| -> usize {
        let mut pairs: BTreeMap<(TaskForceId, TaskForceId), usize> = BTreeMap::new();
        for event in &run.events {
            if let EventKind::EngagementBegun { a, b, .. } = event.kind {
                if (50..70).contains(&a.raw()) && (50..70).contains(&b.raw()) {
                    *pairs.entry((a, b)).or_default() += 1;
                }
            }
        }
        pairs.values().copied().max().unwrap_or(0)
    };
    let parted = run(SOAK, 1942);
    // The pairs of part D meet the day they are ordered to, and part two days
    // later (days 35, 52, 65, 82, 95, 112, 125, 142, 155 and 172).
    let text = std::fs::read_to_string(frozen_scenario_path(SOAK)).expect("the file reads");
    let mut kept = String::new();
    let mut table = String::new();
    let flush = |table: &mut String, kept: &mut String| {
        let value = |key: &str| {
            table
                .lines()
                .find_map(|l| l.strip_prefix(key).and_then(|r| r.strip_prefix(" = ")))
                .and_then(|v| v.trim().parse::<u32>().ok())
        };
        let parting = table.starts_with("[[order]]")
            && value("task_force").is_some_and(|f| (50..70).contains(&f))
            && value("day")
                .is_some_and(|d| [35, 52, 65, 82, 95, 112, 125, 142, 155, 172].contains(&d));
        if !parting {
            kept.push_str(table);
        }
        table.clear();
    };
    for line in text.split_inclusive('\n') {
        if line.starts_with("[[") {
            flush(&mut table, &mut kept);
        }
        table.push_str(line);
    }
    flush(&mut table, &mut kept);
    assert_ne!(kept, text, "some orders were left out");
    let mut scenario =
        common::from_text(&kept, &frozen_scenario_path(SOAK)).expect("the variant loads");
    scenario.run_to_end().expect("the variant runs");
    let variant = Run {
        seed: 1942,
        start: Picture::default(),
        hashes: Vec::new(),
        days: Vec::new(),
        events: scenario.world.events().iter().cloned().collect(),
        seconds: 0.0,
        day_seconds: Vec::new(),
    };
    let (with, without) = (most_meetings(&parted), most_meetings(&variant));
    eprintln!("most meetings of one pair: parted {with}, never parted {without}");
    assert!(with < 10, "{with}");
    assert!(without >= 60, "{without}");
}

// ---------------------------------------------------------------------
// Measurements (decision 6).
// ---------------------------------------------------------------------

/// Prints what a soak costs. Run with `--nocapture`; in release mode for the
/// time a player would see.
#[test]
fn print_the_summary() {
    let soak = run(SOAK, 1942);
    let slice = run(FULL_SLICE, 1942);
    let total = soak.events.len();
    eprintln!("== scenario 8, seed 1942 ==");
    eprintln!("events in the log: {total}");
    eprintln!("events by window of 30 days: {:?}", events_by_window(&soak));
    let seconds_by_window: Vec<String> = soak
        .day_seconds
        .chunks(WINDOW)
        .map(|c| format!("{:.3}", c.iter().sum::<f64>()))
        .collect();
    eprintln!("seconds by window of 30 days: {seconds_by_window:?}");
    eprintln!(
        "time for the 180 days, load excluded: {:.3} s",
        soak.seconds
    );
    let scale = 180.0 / 14.0;
    let scaled_events = slice.events.len() as f64 * scale;
    let scaled_seconds = slice.seconds * scale;
    eprintln!(
        "scenario 7, 14 days: {} events, {:.3} s; scaled by 180/14: {:.0} events, {:.3} s",
        slice.events.len(),
        slice.seconds,
        scaled_events,
        scaled_seconds
    );
    eprintln!(
        "scenario 8 against the scaled scenario 7: events x{:.2}, time x{:.2}",
        total as f64 / scaled_events,
        soak.seconds / scaled_seconds
    );
    let counts = activity_counts(&soak);
    eprintln!("window table (days 1-30, 31-60, ..., 151-180):");
    for (activity, name) in ACTIVITIES {
        eprintln!("  {name:<24} {:?}", counts[&activity]);
    }
    let per_day = |from: usize, to: usize| {
        soak.events
            .iter()
            .filter(|e| (from..=to).contains(&day_of(e)))
            .count() as f64
            / (to - from + 1) as f64
    };
    eprintln!(
        "events a day: first 30 days {:.1}, last 30 days {:.1}",
        per_day(1, 30),
        per_day(151, 180)
    );
    assert!(total > 0);
}
