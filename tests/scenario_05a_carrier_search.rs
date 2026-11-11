//! Scenario 5a: a carrier steams east and its scouts search ahead of it.
//!
//! Stage 5a of scenario 5. It proves that an air group can be based aboard a
//! ship, that it is where the ship is, and that it searches from there by
//! the rule of scenario 3.
//!
//! The data is placeholder data: see `data/scenarios/05a_carrier_search.toml`.
//! The rules are `docs/decisions/0025-carrier-air-groups.md` and
//! `docs/decisions/0012-search-and-contacts.md`.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. The tests below assert these
//! numbers. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass.
//!
//! ## Where the carrier is
//!
//! The carrier force, task force 1, starts at (0,0) and steams east for
//! (10,0), 400 nm, at 16 knots. The slowest ship sets the speed, and both
//! ships cruise at 16 knots. A force's position is the nearest hex centre.
//! The Japanese force lies stopped at (14,0).
//!
//! | Pulse           | Steamed in it | Total  | Position at its end |
//! | --------------- | ------------- | ------ | ------------------- |
//! | Day 1 night     | 192 nm        | 192 nm | 32 nm past (4,0): (5,0)  |
//! | Day 1 morning   | 96 nm         | 288 nm | 8 nm past (7,0): (7,0)   |
//! | Day 1 afternoon | 96 nm         | 384 nm | 24 nm past (9,0): (10,0) |
//! | Day 2 night     | 16 nm         | 400 nm | (10,0), arrived at hour 25 |
//!
//! Search is flown before movement in a daylight pulse, and not at night. So
//! the scouts fly from where the carrier was at the start of each daylight
//! pulse:
//!
//! | Search          | Flown from | Distance to (14,0) |
//! | --------------- | ---------- | ------------------ |
//! | Day 1 morning   | (5,0)      | 9 hexes, 360 nm    |
//! | Day 1 afternoon | (7,0)      | 7 hexes, 280 nm    |
//! | Day 2 morning   | (10,0)     | 4 hexes, 160 nm    |
//! | Day 2 afternoon | (10,0)     | 4 hexes, 160 nm    |
//!
//! ## The chance of a sighting
//!
//! The group has 8 aircraft with a sweep width of 20 nm, and searches the
//! arc 040 to 130, 90 degrees, out to 300 nm. (First written as 045 to 135,
//! which an order may not give: bearings go in steps of 10 degrees, decision
//! 0012. No figure below changed.) The target bears 090, inside
//! the arc, in every search. The chance is
//! 57.295779 x aircraft x sweep width / (arc x distance), which here is
//! 57.295779 x 8 x 20 = 9167.32464, over 90 x distance.
//!
//! | Search          | Distance | Chance                         |
//! | --------------- | -------- | ------------------------------ |
//! | Day 1 morning   | 360 nm   | beyond 300 nm: no attempt      |
//! | Day 1 afternoon | 280 nm   | 9167.32464 / 25200 = 0.363782  |
//! | Day 2 morning   | 160 nm   | 9167.32464 / 14400 = 0.636619  |
//! | Day 2 afternoon | 160 nm   | 0.636619                       |
//!
//! So every run has four searches flown and exactly three attempts, with
//! those chances. Each is rolled on its own.
//!
//! | Count of runs, over seeds 1 to 1000  | p        | Mean   | Std dev | Allowed    |
//! | ------------------------------------ | -------- | ------ | ------- | ---------- |
//! | Sighted on day 1, afternoon          | 0.363782 | 363.78 | 15.21   | 303 to 424 |
//! | Sighted on day 2, morning            | 0.636619 | 636.62 | 15.21   | 576 to 697 |
//! | Sighted on day 2, afternoon          | 0.636619 | 636.62 | 15.21   | 576 to 697 |
//! | Sighted at least once                | 1 - 0.636218 x 0.363381 x 0.363381 = 0.915990 | 915.99 | 8.77 | 881 to 951 |
//!
//! ## Fuel
//!
//! At cruise speed the carrier burns 3000 / 9000 = 0.333333 t per nm and the
//! destroyer 500 / 4000 = 0.125 t per nm. Over 400 nm the destroyer burns
//! 50 t and ends with 450 t. The carrier's burn is worked pulse by pulse:
//! 192 x 3000 / 9000 = 64, then 32 and 32, then 16 x 3000 / 9000 = 5.333333,
//! which is 133.333333 t, leaving 2866.666667 t.
//!
//! # Properties
//!
//! - The air group is aboard ship 1 at the end of every day, and its
//!   position is task force 1's position.
//! - Every search is flown from the hex task force 1 was in at the start of
//!   that pulse.
//! - More aircraft never lower the chance: with 16 aircraft the three
//!   chances are 0.727565, 1 and 1 (9167.32464 x 2 = 18334.64928, over
//!   25200 and over 14400, the second capped at 1).
//! - The aircraft aboard a ship never exceed its class's capacity, 36 here.
//! - A replay of any seed reproduces the daily hash.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use sim_core::events::{Cause, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, ShipId, TaskForceId};
use sim_core::state::{AirGroupLocation, ShipLocation};
use sim_core::time::Pulse;
use sim_data::Scenario;

const CARRIER_FORCE: TaskForceId = TaskForceId::new(1);
const CARRIER: ShipId = ShipId::new(1);
const DESTROYER: ShipId = ShipId::new(2);
const AIR_GROUP: AirGroupId = AirGroupId::new(1);
const SEEDS: u64 = 1000;
const CAPACITY: u32 = 36;

/// The four searches flown, in order: (day, pulse, hex flown from).
const FLIGHTS: [(u32, Pulse, Hex); 4] = [
    (1, Pulse::Morning, Hex::new(5, 0)),
    (1, Pulse::Afternoon, Hex::new(7, 0)),
    (2, Pulse::Morning, Hex::new(10, 0)),
    (2, Pulse::Afternoon, Hex::new(10, 0)),
];

/// The three attempts: (day, pulse, distance in nm, chance in millionths).
const ATTEMPTS: [(u32, Pulse, i64, i64); 3] = [
    (1, Pulse::Afternoon, 280, 363_782),
    (2, Pulse::Morning, 160, 636_619),
    (2, Pulse::Afternoon, 160, 636_619),
];

/// The allowed counts of runs, over seeds 1 to 1000, in which each attempt
/// sighted the force, and then of runs with at least one sighting.
const ALLOWED: [(i64, i64); 3] = [(303, 424), (576, 697), (576, 697)];
const ALLOWED_AT_LEAST_ONE: (i64, i64) = (881, 951);

fn nm(value: i64) -> Fixed {
    Fixed::from_int(value)
}

fn millionths(value: i64) -> Fixed {
    Fixed::from_raw(value)
}

fn scenario_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/05a_carrier_search.toml")
}

fn load() -> Scenario {
    Scenario::load(&scenario_path()).expect("scenario 5a loads")
}

fn pulse_of(event: &Event) -> Pulse {
    event.pulse.expect("this event has a pulse")
}

fn all_events(scenario: &Scenario) -> Vec<Event> {
    scenario.world.events().iter().cloned().collect()
}

/// The search attempts of a list of events as (day, pulse, distance, chance,
/// sighted).
fn attempts(events: &[Event]) -> Vec<(u32, Pulse, Fixed, Fixed, bool)> {
    events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::SearchAttempt {
                air_group,
                task_force,
                distance_nm,
                chance,
                sighted,
            } => {
                assert_eq!(*air_group, AIR_GROUP);
                assert_eq!(*task_force, TaskForceId::new(2));
                Some((event.day, pulse_of(event), *distance_nm, *chance, *sighted))
            }
            _ => None,
        })
        .collect()
}

/// The searches flown as (day, pulse, origin hex, origin).
fn flights(events: &[Event]) -> Vec<(u32, Pulse, Hex, AirGroupLocation)> {
    events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::SearchFlown {
                air_group,
                from,
                hex,
                from_deg,
                to_deg,
                range_nm,
                aircraft,
            } => {
                assert_eq!(*air_group, AIR_GROUP);
                assert_eq!(*range_nm, nm(300));
                assert_eq!((*from_deg, *to_deg), (40, 130));
                assert_eq!(*aircraft, 8);
                Some((event.day, pulse_of(event), *hex, *from))
            }
            _ => None,
        })
        .collect()
}

// ---------------------------------------------------------------------
// The single run with the file's own seed.
// ---------------------------------------------------------------------

#[test]
fn the_carrier_steams_east_pulse_by_pulse_as_the_hand_calculation_says() {
    let mut scenario = load();
    assert!(scenario.placeholder);
    assert_eq!(scenario.days, 2);
    scenario.run_to_end().expect("the run resolves");
    let events = all_events(&scenario);

    // No order was rejected: the search order took effect.
    assert!(!events
        .iter()
        .any(|event| matches!(event.kind, EventKind::OrderRejected { .. })));

    // What each pulse's movement did: (day, pulse, nm steamed, position at
    // its end).
    let steamed: Vec<(u32, Pulse, Fixed, Hex)> = events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::TaskForceSteamed {
                task_force,
                to,
                distance_nm,
                ..
            } => {
                assert_eq!(*task_force, CARRIER_FORCE);
                Some((event.day, pulse_of(event), *distance_nm, *to))
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        steamed,
        vec![
            (1, Pulse::Night, nm(192), Hex::new(5, 0)),
            (1, Pulse::Morning, nm(96), Hex::new(7, 0)),
            (1, Pulse::Afternoon, nm(96), Hex::new(10, 0)),
            (2, Pulse::Night, nm(16), Hex::new(10, 0)),
        ]
    );
    let total = steamed.iter().fold(Fixed::ZERO, |sum, s| sum + s.2);
    assert_eq!(total, nm(400));

    // Arrival at (10,0) at hour 25.
    let arrivals: Vec<(Hex, Fixed)> = events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::TaskForceArrived {
                task_force,
                hex,
                elapsed_hours,
            } => {
                assert_eq!(*task_force, CARRIER_FORCE);
                Some((*hex, *elapsed_hours))
            }
            _ => None,
        })
        .collect();
    assert_eq!(arrivals, vec![(Hex::new(10, 0), nm(25))]);

    // The Japanese force lies stopped at (14,0).
    let truth = scenario.world.umpire();
    assert_eq!(
        truth.task_forces[&TaskForceId::new(2)].position(),
        Hex::new(14, 0)
    );
    assert!(!events.iter().any(|event| matches!(
        event.kind,
        EventKind::TaskForceSteamed { task_force, .. } if task_force == TaskForceId::new(2)
    )));
}

#[test]
fn every_search_is_flown_from_where_the_carrier_was_at_the_start_of_the_pulse() {
    let mut scenario = load();
    scenario.run_to_end().expect("the run resolves");
    let events = all_events(&scenario);

    // Four searches flown, none at night, each from the ship.
    let flown = flights(&events);
    assert_eq!(flown.len(), 4);
    for ((day, pulse, hex, from), (e_day, e_pulse, e_hex)) in flown.iter().zip(FLIGHTS) {
        assert_eq!((*day, *pulse, *hex), (e_day, e_pulse, e_hex));
        assert_eq!(*from, AirGroupLocation::Ship(CARRIER));
    }

    // The same hexes come from the movement events: the hex the force
    // steamed from in the pulse (or held, once arrived).
    let mut position = Hex::new(0, 0);
    let mut at_start = Vec::new();
    for day in 1..=2 {
        for pulse in [Pulse::Night, Pulse::Morning, Pulse::Afternoon] {
            at_start.push((day, pulse, position));
            for event in &events {
                if let EventKind::TaskForceSteamed { to, .. } = &event.kind {
                    if event.day == day && event.pulse == Some(pulse) {
                        position = *to;
                    }
                }
            }
        }
    }
    for (day, pulse, hex) in FLIGHTS {
        let start = at_start
            .iter()
            .find(|(d, p, _)| *d == day && *p == pulse)
            .expect("a pulse");
        assert_eq!(start.2, hex, "D{day} {pulse:?}");
    }

    // Each flight names the ship as an entity beside the group, and cites
    // the two orders.
    for event in events
        .iter()
        .filter(|e| matches!(e.kind, EventKind::SearchFlown { .. }))
    {
        assert_eq!(
            event.kind.entities(),
            vec![
                sim_core::ids::EntityRef::AirGroup(AIR_GROUP),
                sim_core::ids::EntityRef::Ship(CARRIER)
            ]
        );
        assert_eq!(event.causes.len(), 2);
        assert!(event.causes.iter().all(|c| matches!(c, Cause::Order(_))));
    }
    let first = events
        .iter()
        .find(|e| matches!(e.kind, EventKind::SearchFlown { .. }))
        .expect("a flight");
    assert_eq!(
        first.kind.to_string(),
        "air_group:1 searched 040-130 out to 300 nm from ship:1 at (5,0) with 8 aircraft"
    );
}

#[test]
fn three_attempts_are_made_at_the_stated_distances_and_chances() {
    let mut scenario = load();
    scenario.run_to_end().expect("the run resolves");
    let events = all_events(&scenario);

    let found = attempts(&events);
    assert_eq!(found.len(), 3, "no attempt on day 1 morning");
    for ((day, pulse, distance, chance, _), (e_day, e_pulse, e_distance, e_chance)) in
        found.iter().zip(ATTEMPTS)
    {
        assert_eq!((*day, *pulse), (e_day, e_pulse));
        assert_eq!(*distance, nm(e_distance));
        assert_eq!(*chance, millionths(e_chance));
    }
    // Day 1 morning: flown from (5,0), 9 hexes, 360 nm from (14,0), beyond
    // the 300 nm range.
    assert_eq!(Hex::new(5, 0).distance(Hex::new(14, 0)) * 40, 360);
    assert!(!found
        .iter()
        .any(|(day, pulse, ..)| *day == 1 && *pulse == Pulse::Morning));
    // Distances of the other searches, from the hexes flown from.
    for ((_, _, hex), distance) in [FLIGHTS[1], FLIGHTS[2], FLIGHTS[3]]
        .iter()
        .zip([280, 160, 160])
    {
        assert_eq!(i64::from(hex.distance(Hex::new(14, 0))) * 40, distance);
    }

    // The breakdown of each attempt.
    for event in &events {
        if let EventKind::SearchAttempt {
            chance,
            distance_nm,
            ..
        } = &event.kind
        {
            let breakdown = event.breakdown.as_ref().expect("a breakdown is recorded");
            assert_eq!(breakdown.result, *chance);
            let factors: Vec<(&str, Fixed)> = breakdown
                .factors
                .iter()
                .map(|factor| (factor.name, factor.value))
                .collect();
            assert_eq!(
                factors,
                vec![
                    ("degrees_per_radian", millionths(57_295_779)),
                    ("aircraft", nm(8)),
                    ("sweep_width_nm", nm(20)),
                    ("arc_deg", nm(90)),
                    ("distance_nm", *distance_nm),
                ]
            );
        }
    }
}

#[test]
fn the_fuel_burned_is_as_the_hand_calculation_says() {
    let mut scenario = load();
    scenario.run_to_end().expect("the run resolves");
    let events = all_events(&scenario);

    let burns = |ship: ShipId| -> Vec<Fixed> {
        events
            .iter()
            .filter_map(|event| match &event.kind {
                EventKind::FuelBurned {
                    ship: burned_ship,
                    burned_t,
                    ..
                } if *burned_ship == ship => Some(*burned_t),
                _ => None,
            })
            .collect()
    };
    // The carrier: 64, 32 and 32 t, then 16 x 3000 / 9000 = 5.333333 t.
    assert_eq!(
        burns(CARRIER),
        vec![nm(64), nm(32), nm(32), millionths(5_333_333)]
    );
    let carrier_total = burns(CARRIER).iter().fold(Fixed::ZERO, |sum, t| sum + *t);
    assert_eq!(carrier_total, millionths(133_333_333));
    // The destroyer: 400 nm x 0.125 t per nm = 50 t.
    let destroyer_total = burns(DESTROYER).iter().fold(Fixed::ZERO, |sum, t| sum + *t);
    assert_eq!(destroyer_total, nm(50));

    let truth = scenario.world.umpire();
    assert_eq!(truth.ships[&CARRIER].fuel_t, millionths(2_866_666_667));
    assert_eq!(truth.ships[&DESTROYER].fuel_t, nm(450));
}

#[test]
fn the_group_is_aboard_the_carrier_and_where_its_force_is_at_the_end_of_every_day() {
    let mut scenario = load();
    let mut positions = Vec::new();
    for _ in 1..=2 {
        scenario
            .next_day()
            .expect("the day resolves")
            .expect("the day exists");
        let truth = scenario.world.umpire();
        let group = &truth.air_groups[&AIR_GROUP];
        assert_eq!(group.location, AirGroupLocation::Ship(CARRIER));
        assert_eq!(group.ready, 8);
        assert_eq!(
            truth.ships[&CARRIER].location,
            ShipLocation::TaskForce(CARRIER_FORCE)
        );
        let force_position = truth.task_forces[&CARRIER_FORCE].position();
        assert_eq!(truth.air_group_hex(group), Some(force_position));
        positions.push(force_position);
    }
    assert_eq!(positions, vec![Hex::new(10, 0), Hex::new(10, 0)]);
}

// ---------------------------------------------------------------------
// The 1,000-run batch, run once and shared by the tests below.
// ---------------------------------------------------------------------

struct RunSummary {
    seed: u64,
    /// (day, pulse, distance, chance, sighted) of each attempt.
    attempts: Vec<(u32, Pulse, Fixed, Fixed, bool)>,
    flown: Vec<(u32, Pulse, Hex, AirGroupLocation)>,
    hashes: Vec<u64>,
}

fn batch() -> &'static Vec<RunSummary> {
    static BATCH: OnceLock<Vec<RunSummary>> = OnceLock::new();
    BATCH.get_or_init(|| {
        let base = load();
        (1..=SEEDS)
            .map(|seed| {
                let mut run = base.with_seed(seed).expect("a seed can be set");
                run.run_to_end().expect("the run resolves");
                let events = all_events(&run);
                RunSummary {
                    seed,
                    attempts: attempts(&events),
                    flown: flights(&events),
                    hashes: run.world.day_hashes().to_vec(),
                }
            })
            .collect()
    })
}

#[test]
fn sightings_fall_where_the_hand_calculation_says() {
    let runs = batch();
    assert_eq!(runs.len(), 1000);
    let mut sighted_counts = [0i64; 3];
    let mut at_least_one = 0i64;
    for run in runs {
        // Every run has four searches flown and exactly three attempts,
        // with the stated distances and chances.
        assert_eq!(run.flown.len(), 4, "seed {}", run.seed);
        assert_eq!(run.attempts.len(), 3, "seed {}", run.seed);
        for ((day, pulse, distance, chance, _), (e_day, e_pulse, e_distance, e_chance)) in
            run.attempts.iter().zip(ATTEMPTS)
        {
            assert_eq!((*day, *pulse), (e_day, e_pulse), "seed {}", run.seed);
            assert_eq!(*distance, nm(e_distance));
            assert_eq!(*chance, millionths(e_chance));
        }
        for (count, attempt) in sighted_counts.iter_mut().zip(&run.attempts) {
            *count += i64::from(attempt.4);
        }
        at_least_one += i64::from(run.attempts.iter().any(|attempt| attempt.4));
    }
    eprintln!("sighting counts per attempt: {sighted_counts:?}");
    eprintln!("sighted at least once: {at_least_one}");
    for (count, (low, high)) in sighted_counts.iter().zip(ALLOWED) {
        assert!(
            (low..=high).contains(count),
            "{count} not in {low}..={high}"
        );
    }
    assert!(
        (ALLOWED_AT_LEAST_ONE.0..=ALLOWED_AT_LEAST_ONE.1).contains(&at_least_one),
        "{at_least_one} runs sighted at least once"
    );
}

// ---------------------------------------------------------------------
// Properties.
// ---------------------------------------------------------------------

#[test]
fn every_search_of_every_seed_is_flown_from_the_ship_and_its_forces_hex() {
    for run in batch() {
        for (flight, expected) in run.flown.iter().zip(FLIGHTS) {
            assert_eq!(
                (flight.0, flight.1, flight.2),
                expected,
                "seed {}",
                run.seed
            );
            assert_eq!(flight.3, AirGroupLocation::Ship(CARRIER));
        }
    }
}

/// Copies the scenario and its data into a folder under the build directory,
/// makes each exact, asserted-unique text replacement in the copied scenario
/// file, and loads it. Nothing in the repository's data folder is touched.
fn variant(tag: &str, edits: &[(&str, &str)]) -> Scenario {
    try_variant(tag, edits).expect("the variant loads")
}

/// As `variant`, returning the loader's error instead of panicking.
fn try_variant(tag: &str, edits: &[(&str, &str)]) -> Result<Scenario, sim_data::DataError> {
    let data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data");
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("carrier_search_{tag}"));
    for folder in ["scenarios", "map", "reference"] {
        fs::create_dir_all(root.join(folder)).expect("make the variant folder");
    }
    for file in [
        "map/placeholder_search.toml",
        "reference/ship_classes.csv",
        "reference/port_sizes.csv",
        "reference/aircraft_types.csv",
        "reference/search.toml",
        "reference/surface_combat.toml",
    ] {
        fs::copy(data.join(file), root.join(file)).expect("copy a data file");
    }
    let mut text = fs::read_to_string(data.join("scenarios/05a_carrier_search.toml"))
        .expect("read the scenario");
    for (original, changed) in edits {
        assert_eq!(
            text.matches(original).count(),
            1,
            "the text {original:?} appears exactly once"
        );
        text = text.replace(original, changed);
    }
    fs::write(root.join("scenarios/05a_carrier_search.toml"), text).expect("write the variant");
    Scenario::load(&root.join("scenarios/05a_carrier_search.toml"))
}

/// The chances of the attempts of a scenario run with its own seed.
fn chances_of(mut scenario: Scenario) -> Vec<Fixed> {
    scenario.run_to_end().expect("the variant runs");
    attempts(&all_events(&scenario))
        .into_iter()
        .map(|attempt| attempt.3)
        .collect()
}

#[test]
fn more_aircraft_never_lower_the_chance() {
    let chances: Vec<Vec<Fixed>> = [8, 16]
        .iter()
        .map(|ready| {
            chances_of(variant(
                &format!("ready_{ready}"),
                &[("ready = 8", &format!("ready = {ready}"))],
            ))
        })
        .collect();
    assert_eq!(
        chances[0],
        vec![
            millionths(363_782),
            millionths(636_619),
            millionths(636_619)
        ]
    );
    // 16 aircraft: 18334.64928 / 25200 = 0.727565, then both others capped
    // at 1.
    assert_eq!(
        chances[1],
        vec![millionths(727_565), Fixed::ONE, Fixed::ONE]
    );
    for (fewer, more) in chances[0].iter().zip(&chances[1]) {
        assert!(fewer <= more);
    }
}

#[test]
fn the_aircraft_aboard_a_ship_never_exceed_its_class_capacity() {
    let scenario = load();
    let truth = scenario.world.umpire();
    let capacity =
        scenario.world.ref_data().ship_classes[&truth.ships[&CARRIER].class].aircraft_capacity;
    assert_eq!(capacity, CAPACITY);
    let aboard: u32 = truth
        .air_groups
        .values()
        .filter(|group| group.location == AirGroupLocation::Ship(CARRIER))
        .map(|group| group.ready)
        .sum();
    assert!(aboard <= capacity);

    // A variant with the group one aircraft over the capacity does not load,
    // and one exactly at it does.
    let at_capacity = variant("at_capacity", &[("ready = 8", "ready = 36")]);
    assert_eq!(
        at_capacity.world.umpire().air_groups[&AIR_GROUP].ready,
        CAPACITY
    );
    let over = match try_variant("over_capacity", &[("ready = 8", "ready = 37")]) {
        Ok(_) => panic!("37 aircraft do not fit aboard"),
        Err(error) => error,
    };
    assert!(over.message.contains("carries 36"), "{}", over.message);
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
        assert_eq!(hashes[0].len(), 2);
        assert_eq!(hashes[0], hashes[1], "seed {seed}");
        all_hashes.push(hashes[0].clone());
    }
    // The seed matters: different seeds do not all give the same history.
    assert!(all_hashes.windows(2).any(|pair| pair[0] != pair[1]));

    // The batch holds the same hashes as a fresh run of the same seed.
    let mut again = base.with_seed(77).expect("a seed can be set");
    again.run_to_end().expect("the run resolves");
    assert_eq!(batch()[76].seed, 77);
    assert_eq!(batch()[76].hashes, again.world.day_hashes());

    // The file's own seed gives the same hashes as a seeded copy.
    let mut own = load();
    own.run_to_end().expect("the run resolves");
    let mut copy = base.with_seed(1942).expect("a seed can be set");
    copy.run_to_end().expect("the run resolves");
    assert_eq!(own.world.day_hashes(), copy.world.day_hashes());
}
