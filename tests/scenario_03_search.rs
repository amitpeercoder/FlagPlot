//! Scenario 3: flying boats search an arc while an enemy task force steams
//! toward their base.
//!
//! Proves that the chance of a sighting matches a hand calculation in every
//! pulse, that detection rates over 1,000 runs fall where that calculation
//! says, that reports are wrong as often as the figures say, and that
//! contacts age, merge and drop by the rules.
//!
//! The data is placeholder data: see `data/scenarios/03_search.toml`. The
//! rules are in `docs/decisions/0012-search-and-contacts.md` and
//! `docs/decisions/0013-random-keys-for-pairs.md`.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. The tests below assert these
//! numbers. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass.
//!
//! Inputs, from the data files:
//!
//! - Allied base 1 at (0,0). Air group 1: 4 flying boats, sweep width 40 nm,
//!   search range 600 nm.
//! - Days 1 to 3: the group searches the arc from 060 to 120, which is 60
//!   degrees wide and centred on due east, out to 600 nm. On day 4 it is
//!   ordered to search 240 to 300, due west.
//! - Japanese task force 1: two cruisers and a destroyer, 3 ships, the most
//!   important kind being cruiser. It starts at (22,0), 880 nm due east, and
//!   steams west along r = 0 to (4,0) at 16 kn, the cruise speed of both
//!   classes. Hexes on r = 0 east of the base lie on bearing 090 exactly, in
//!   the middle of the arc.
//! - A day is a 12-hour night pulse, then two 6-hour pulses. Search runs at
//!   the start of the morning and afternoon pulses, before movement. There
//!   is no search in the night pulse.
//! - Chance of a sighting = (57.295779 x aircraft x sweep) / (arc x
//!   distance), truncated to six decimals and capped at 1. With 4 aircraft
//!   and a 40 nm sweep the numerator is 57.295779 x 160 = 9167.32464.
//!
//! ## Where the task force is at each search
//!
//! It steams 192 nm in a night pulse and 96 nm in a day pulse. A task force
//! is in the hex whose centre is nearest (decision 0005).
//!
//! | Search          | Hour | Steamed | Hex left, miles on | Position | Distance |
//! | --------------- | ---- | ------- | ------------------ | -------- | -------- |
//! | Day 1 morning   | 12   | 192 nm  | (18,0) plus 32 nm  | (17,0)   | 680 nm   |
//! | Day 1 afternoon | 18   | 288 nm  | (15,0) plus 8 nm   | (15,0)   | 600 nm   |
//! | Day 2 morning   | 36   | 576 nm  | (8,0) plus 16 nm   | (8,0)    | 320 nm   |
//! | Day 2 afternoon | 42   | 672 nm  | (6,0) plus 32 nm   | (5,0)    | 200 nm   |
//! | Day 3 morning   | 60   | 720 nm  | (4,0), arrived     | (4,0)    | 160 nm   |
//! | Day 3 afternoon | 66   | 720 nm  | (4,0), arrived     | (4,0)    | 160 nm   |
//!
//! The route is 18 hexes, 720 nm, which takes 45 hours. The task force
//! arrives at (4,0) at hour 45, 15:00 on day 2, and stays there. Fuel for the
//! transit: each cruiser 720 x 0.25 = 180 t, the destroyer 720 x 0.125 =
//! 90 t.
//!
//! ## The chance of a sighting in each search
//!
//! | Search          | Distance | Arc x distance | Chance                         |
//! | --------------- | -------- | -------------- | ------------------------------ |
//! | Day 1 morning   | 680 nm   | beyond 600 nm  | none: not a target             |
//! | Day 1 afternoon | 600 nm   | 36000          | 9167.32464 / 36000 = 0.254647  |
//! | Day 2 morning   | 320 nm   | 19200          | 9167.32464 / 19200 = 0.477464  |
//! | Day 2 afternoon | 200 nm   | 12000          | 9167.32464 / 12000 = 0.763943  |
//! | Day 3 morning   | 160 nm   | 9600           | 9167.32464 / 9600 = 0.954929   |
//! | Day 3 afternoon | 160 nm   | 9600           | 0.954929                       |
//! | Day 4, both     | 160 nm   | outside 240-300 | none: not a target            |
//!
//! 600 nm is on the edge of the range and counts as inside. These chances
//! are exact and the same in every run, whatever the seed. The test reads
//! them from the search event's breakdown.
//!
//! ## Detection rates over 1,000 runs
//!
//! The scenario runs with seeds 1 to 1000. Each search is an independent
//! roll, so the number of runs with a sighting in a given search follows a
//! binomial distribution: mean 1000p, standard deviation sqrt(1000p(1-p)).
//! The test asserts each count is within four standard deviations of its
//! mean. The seeds are fixed, so the result never varies.
//!
//! | Search          | p        | Mean   | Std dev | Allowed count |
//! | --------------- | -------- | ------ | ------- | ------------- |
//! | Day 1 afternoon | 0.254647 | 254.65 | 13.78   | 200 to 309    |
//! | Day 2 morning   | 0.477464 | 477.46 | 15.80   | 415 to 540    |
//! | Day 2 afternoon | 0.763943 | 763.94 | 13.43   | 711 to 817    |
//! | Day 3 morning   | 0.954929 | 954.93 | 6.56    | 929 to 981    |
//! | Day 3 afternoon | 0.954929 | 954.93 | 6.56    | 929 to 981    |
//!
//! Sighted at least once by the end of day 2: the chance of three misses is
//! 0.745353 x 0.522536 x 0.236057 = 0.091938, so p = 0.908062. Mean 908.06,
//! standard deviation 9.14, allowed count 872 to 944.
//!
//! No run has a sighting on day 1 morning, in any night pulse, or on day 4.
//!
//! ## Report errors
//!
//! Every sighting makes one report. The truth is 3 ships, most important kind
//! cruiser. From `data/reference/search.toml`:
//!
//! - Position: the true hex with chance 0.7, a neighbouring hex with chance
//!   0.3. Every neighbour of the task force's positions is open sea, so a
//!   wrong position is always one hex from the true one.
//! - Count: 3 with chance 0.6, 4 with chance 0.2, 2 with chance 0.2.
//! - Kind: cruiser with chance 0.7, and battleship, destroyer and carrier
//!   with chance 0.1 each.
//!
//! About 3406 reports are expected across the 1,000 runs (the five means
//! added). For each of these chances p, with n the number of reports the
//! runs actually made and k the number that came out that way, the test
//! asserts (k - np)^2 <= 16np(1-p): within four standard deviations. Every
//! report must be one of the outcomes listed. No report names a hex more
//! than one from the truth, a count outside 2 to 4, or a kind outside the
//! four above.
//!
//! ## Contacts: ageing, merging and dropping
//!
//! At the end of each pulse a contact that was not created or refreshed in
//! that pulse ages by one. A contact that reaches an age of 5 pulses is
//! dropped.
//!
//! (Rewritten at the complexity review after scenario 5, decision 0032. A
//! contact used to carry a confidence, 0.8 when fresh and multiplied by 0.8
//! each pulse, and was dropped below 0.3: that is at an age of 5, so nothing
//! here happens at a different time. A sighted task force used to roll to
//! notice the aircraft; that roll is cut, and its section with it.)
//!
//! The scenario has 12 pulses. A contact last created or refreshed in pulse
//! k has age 12 - k at the end. The searches fall in pulses 3, 5, 6, 8 and 9.
//!
//! | Last report in  | Pulse | At the end of day 4                |
//! | --------------- | ----- | ---------------------------------- |
//! | Day 3 afternoon | 9     | age 3                              |
//! | Day 3 morning   | 8     | age 4                              |
//! | Day 2 afternoon | 6     | dropped at the end of pulse 11     |
//! | Day 2 morning   | 5     | dropped at the end of pulse 10     |
//! | Day 1 afternoon | 3     | dropped at the end of pulse 8      |
//!
//! So at the end of every run the Allied side holds at most two contacts,
//! and each has one of the two ages in the first two rows. The test replays
//! each run's reports through the rules below and asserts the contacts and
//! their ages exactly.
//!
//! Merging: a report refreshes the nearest contact that is within 2 hexes of
//! the reported hex and no more than 2 pulses old (the lowest ID on a tie).
//! Otherwise it makes a new contact. For example, a day 3 morning report
//! names (4,0) or a neighbour. A contact made on day 2 afternoon names (5,0)
//! or a neighbour and is 1 pulse old by then (it aged at the end of the day
//! 3 night pulse). The two hexes are 0 to 3 apart, so the report usually
//! refreshes that contact and sometimes makes a second one.
//!
//! The Japanese side has no search and holds no contacts in any run.
//!
//! # Properties
//!
//! Beside the numbers above, the tests check what must always hold:
//!
//! - More aircraft never lower the chance of a sighting: with 1, 2, 4 and 8
//!   aircraft the chance at 320 nm is 0.119366, 0.238732, 0.477464 and
//!   0.954929, and with 16 it is capped at 1.
//! - A wider arc never raises it: at 320 nm with 4 aircraft, arcs of 30, 60,
//!   120 and 360 degrees give 0.954929, 0.477464, 0.238732 and 0.079577.
//! - A target outside the arc or beyond the range is never sighted, in any
//!   of the 1,000 runs.
//! - Belief holds nothing the reports did not say: every contact's hex, count
//!   and kind are those of the report that last refreshed it.
//! - A replay of any seed reproduces every daily hash.

mod common;

use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, TaskForceId};
use sim_core::refdata::ShipKind;
use sim_core::state::Side;
use sim_core::time::Pulse;
use sim_data::Scenario;

const TASK_FORCE: TaskForceId = TaskForceId::new(1);
const AIR_GROUP: AirGroupId = AirGroupId::new(1);
const SEEDS: u64 = 1000;

/// The five searches that find the task force in range and arc, in order:
/// (day, pulse, position, distance in nm, chance in millionths, hour at the
/// start of the pulse).
const SEARCHES: [(u32, Pulse, Hex, i64, i64, i64); 5] = [
    (1, Pulse::Afternoon, Hex::new(15, 0), 600, 254_647, 18),
    (2, Pulse::Morning, Hex::new(8, 0), 320, 477_464, 36),
    (2, Pulse::Afternoon, Hex::new(5, 0), 200, 763_943, 42),
    (3, Pulse::Morning, Hex::new(4, 0), 160, 954_929, 60),
    (3, Pulse::Afternoon, Hex::new(4, 0), 160, 954_929, 66),
];

/// The allowed sighting counts over 1,000 runs, one per search above.
const ALLOWED: [(i64, i64); 5] = [(200, 309), (415, 540), (711, 817), (929, 981), (929, 981)];

fn scenario_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/03_search.toml")
}

fn load() -> Scenario {
    common::load(&scenario_path()).expect("scenario 3 loads")
}

fn nm(value: i64) -> Fixed {
    Fixed::from_int(value)
}

fn millionths(value: i64) -> Fixed {
    Fixed::from_raw(value)
}

/// Resolves the next day and returns its number and its events.
fn run_day(scenario: &mut Scenario) -> (u32, Vec<Event>) {
    let summary = scenario
        .next_day()
        .expect("the day resolves")
        .expect("the day exists");
    let events = scenario.world.events().from_index(summary.first_event);
    assert_eq!(events.len(), summary.event_count);
    (summary.day, events.to_vec())
}

fn pulse_of(event: &Event) -> Pulse {
    event.pulse.expect("this event has a pulse")
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
                assert_eq!(*task_force, TASK_FORCE);
                Some((event.day, pulse_of(event), *distance_nm, *chance, *sighted))
            }
            _ => None,
        })
        .collect()
}

// ---------------------------------------------------------------------
// The single run with the file's own seed.
// ---------------------------------------------------------------------

#[test]
fn the_single_run_matches_the_hand_calculation() {
    let mut scenario = load();
    assert!(scenario.placeholder);
    assert_eq!(scenario.days, 4);

    let mut all_events: Vec<Event> = Vec::new();
    let mut steam_starts: Vec<(u32, Pulse, Hex)> = Vec::new();
    let mut position_after_day: Vec<Hex> = Vec::new();
    for expected_day in 1..=4 {
        let (day, events) = run_day(&mut scenario);
        assert_eq!(day, expected_day);
        for event in &events {
            if let EventKind::TaskForceSteamed { from, .. } = &event.kind {
                steam_starts.push((event.day, pulse_of(event), *from));
            }
        }
        position_after_day.push(scenario.world.umpire().task_forces[&TASK_FORCE].position());
        all_events.extend(events);
    }

    // The task force's position at the start of each searched pulse. Days 1
    // and 2 come from the movement events; on day 3 it has arrived and does
    // not move, so its position after day 2 holds.
    let start_position = |day: u32, pulse: Pulse| -> Hex {
        steam_starts
            .iter()
            .find(|(d, p, _)| *d == day && *p == pulse)
            .map(|(_, _, from)| *from)
            .unwrap_or(position_after_day[1])
    };
    assert_eq!(start_position(1, Pulse::Morning), Hex::new(17, 0));
    for (day, pulse, position, _, _, _) in SEARCHES {
        assert_eq!(start_position(day, pulse), position, "D{day} {pulse:?}");
    }
    assert_eq!(position_after_day[1], Hex::new(4, 0));
    assert_eq!(position_after_day[2], Hex::new(4, 0));
    assert!(steam_starts.iter().all(|(day, _, _)| *day <= 2));

    // Day 1 morning: the group flies, but the task force is 680 nm out,
    // beyond the 600 nm range, so there is no attempt.
    let flown: Vec<(u32, Pulse)> = all_events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::SearchFlown {
                air_group,
                from_deg,
                to_deg,
                range_nm,
                aircraft,
                ..
            } => {
                assert_eq!(*air_group, AIR_GROUP);
                assert_eq!(*aircraft, 4);
                assert_eq!(*range_nm, nm(600));
                if event.day <= 3 {
                    assert_eq!((*from_deg, *to_deg), (60, 120));
                } else {
                    assert_eq!((*from_deg, *to_deg), (240, 300));
                }
                Some((event.day, pulse_of(event)))
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        flown,
        vec![
            (1, Pulse::Morning),
            (1, Pulse::Afternoon),
            (2, Pulse::Morning),
            (2, Pulse::Afternoon),
            (3, Pulse::Morning),
            (3, Pulse::Afternoon),
            (4, Pulse::Morning),
            (4, Pulse::Afternoon),
        ],
        "a flight in every morning and afternoon pulse, none at night"
    );
    assert_eq!(
        Hex::new(0, 0).distance(Hex::new(17, 0)) * 40,
        680,
        "day 1 morning is 680 nm out"
    );

    // The five attempts, with their distance and chance, and their breakdown.
    let found = attempts(&all_events);
    assert_eq!(found.len(), 5, "no attempt on day 1 morning or on day 4");
    for ((day, pulse, distance, chance, _), expected) in found.iter().zip(SEARCHES) {
        let (e_day, e_pulse, position, e_distance, e_chance, _) = expected;
        assert_eq!((*day, *pulse), (e_day, e_pulse));
        assert_eq!(*distance, nm(e_distance));
        assert_eq!(
            i64::from(Hex::new(0, 0).distance(position)) * 40,
            e_distance
        );
        assert_eq!(*chance, millionths(e_chance));
    }
    for event in &all_events {
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
                    ("aircraft", nm(4)),
                    ("sweep_width_nm", nm(40)),
                    ("arc_deg", nm(60)),
                    ("distance_nm", *distance_nm),
                ]
            );
        }
    }
    // Day 4: the arc is 240 to 300 and the task force is on 090.
    assert!(all_events
        .iter()
        .filter(|event| event.day == 4)
        .all(|event| !matches!(event.kind, EventKind::SearchAttempt { .. })));

    // Fuel: each cruiser burned 180 t and the destroyer 90 t.
    for (ship, burned, left) in [(1, 180, 1820), (2, 180, 1820), (3, 90, 410)] {
        let total: Fixed = all_events
            .iter()
            .filter_map(|event| match &event.kind {
                EventKind::FuelBurned {
                    ship: burned_ship,
                    burned_t,
                    ..
                } if burned_ship.raw() == ship => Some(*burned_t),
                _ => None,
            })
            .fold(Fixed::ZERO, |sum, t| sum + t);
        assert_eq!(total, nm(burned), "ship {ship}");
        let fuel = scenario.world.umpire().ships[&sim_core::ids::ShipId::new(ship)].fuel_t;
        assert_eq!(fuel, nm(left), "ship {ship}");
    }

    // Arrival at (4,0) 45 hours in.
    let arrivals: Vec<(Hex, Fixed)> = all_events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::TaskForceArrived {
                task_force,
                hex,
                elapsed_hours,
            } => {
                assert_eq!(*task_force, TASK_FORCE);
                Some((*hex, *elapsed_hours))
            }
            _ => None,
        })
        .collect();
    assert_eq!(arrivals, vec![(Hex::new(4, 0), nm(45))]);
}

// ---------------------------------------------------------------------
// The 1,000-run batch, run once and shared by the tests below.
// ---------------------------------------------------------------------

#[derive(Clone, Debug)]
struct AttemptRecord {
    day: u32,
    pulse: Pulse,
    distance: Fixed,
    chance: Fixed,
    sighted: bool,
}

#[derive(Clone, Debug)]
struct ReportRecord {
    day: u32,
    pulse: Pulse,
    contact: u32,
    hex: Hex,
    count: u32,
    kind: ShipKind,
    created: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ContactRecord {
    id: u32,
    hex: Hex,
    count: u32,
    kind: ShipKind,
    age: u32,
}

#[derive(Clone, Debug)]
struct RunSummary {
    seed: u64,
    attempts: Vec<AttemptRecord>,
    reports: Vec<ReportRecord>,
    final_contacts: Vec<ContactRecord>,
    japanese_contacts: usize,
}

fn summarize(seed: u64, scenario: &Scenario) -> RunSummary {
    let world = &scenario.world;
    let mut summary = RunSummary {
        seed,
        attempts: Vec::new(),
        reports: Vec::new(),
        final_contacts: Vec::new(),
        japanese_contacts: world.side_view(Side::Japanese).contacts().count(),
    };
    for event in world.events().iter() {
        match &event.kind {
            EventKind::SearchAttempt {
                distance_nm,
                chance,
                sighted,
                ..
            } => summary.attempts.push(AttemptRecord {
                day: event.day,
                pulse: pulse_of(event),
                distance: *distance_nm,
                chance: *chance,
                sighted: *sighted,
            }),
            EventKind::ReportReceived {
                side,
                contact,
                hex,
                ship_count,
                kind,
                created,
            } => {
                assert_eq!(*side, Side::Allied);
                summary.reports.push(ReportRecord {
                    day: event.day,
                    pulse: pulse_of(event),
                    contact: contact.umpire_raw(common::umpire_key()),
                    hex: *hex,
                    count: *ship_count,
                    kind: *kind,
                    created: *created,
                });
            }
            _ => {}
        }
    }
    summary.final_contacts = world
        .side_view(Side::Allied)
        .contacts()
        .map(|contact| ContactRecord {
            id: contact.id.umpire_raw(common::umpire_key()),
            hex: contact.estimated_hex,
            count: contact.estimated_ship_count,
            kind: contact.estimated_kind,
            age: contact.age_pulses,
        })
        .collect();
    summary
}

fn batch() -> &'static Vec<RunSummary> {
    static BATCH: OnceLock<Vec<RunSummary>> = OnceLock::new();
    BATCH.get_or_init(|| {
        let base = load();
        (1..=SEEDS)
            .map(|seed| {
                let mut run = base.with_seed(seed).expect("a seed can be set");
                run.run_to_end().expect("the run resolves");
                summarize(seed, &run)
            })
            .collect()
    })
}

fn search_index(day: u32, pulse: Pulse) -> Option<usize> {
    SEARCHES.iter().position(|s| s.0 == day && s.1 == pulse)
}

#[test]
fn sightings_fall_where_the_hand_calculation_says() {
    let runs = batch();
    assert_eq!(runs.len(), 1000);
    let mut sighted_counts = [0i64; 5];
    let mut by_end_of_day_2 = 0i64;
    let mut total_sightings = 0i64;
    for run in runs {
        // Only the five listed searches ever roll, and each rolls once with
        // the distance and chance of the hand calculation.
        assert_eq!(run.attempts.len(), 5, "seed {}", run.seed);
        for (attempt, search) in run.attempts.iter().zip(SEARCHES) {
            assert_eq!((attempt.day, attempt.pulse), (search.0, search.1));
            assert_eq!(attempt.distance, nm(search.3));
            assert_eq!(attempt.chance, millionths(search.4));
        }
        let mut sighted_by_day_2 = false;
        for attempt in &run.attempts {
            // Never on day 1 morning, in a night pulse, or on day 4.
            assert_ne!(attempt.pulse, Pulse::Night);
            assert!(attempt.day <= 3);
            assert!(!(attempt.day == 1 && attempt.pulse == Pulse::Morning));
            let index = search_index(attempt.day, attempt.pulse).expect("a listed search");
            if attempt.sighted {
                sighted_counts[index] += 1;
                total_sightings += 1;
                sighted_by_day_2 |= attempt.day <= 2;
            }
        }
        by_end_of_day_2 += i64::from(sighted_by_day_2);
        // No report outside a sighted search, and one report for each.
        assert_eq!(
            run.reports.len(),
            run.attempts.iter().filter(|a| a.sighted).count()
        );
        for report in &run.reports {
            let attempt = run
                .attempts
                .iter()
                .find(|a| a.day == report.day && a.pulse == report.pulse)
                .expect("a report follows an attempt");
            assert!(attempt.sighted);
        }
    }
    eprintln!("sighting counts per search: {sighted_counts:?}");
    eprintln!("sighted at least once by end of day 2: {by_end_of_day_2}");
    eprintln!("total sightings (reports): {total_sightings}");
    for (count, (low, high)) in sighted_counts.iter().zip(ALLOWED) {
        assert!(
            (low..=high).contains(count),
            "{count} not in {low}..={high}"
        );
    }
    assert!((872..=944).contains(&by_end_of_day_2));
}

/// Whether k out of n is within four standard deviations of probability
/// `p` (in millionths), in exact integer arithmetic.
fn within_four_sigma(k: i64, n: i64, p: i64) -> bool {
    let (k, n, p) = (i128::from(k), i128::from(n), i128::from(p));
    let scale = 1_000_000;
    let off = k * scale - n * p;
    off * off <= 16 * n * p * (scale - p)
}

#[test]
fn report_errors_occur_as_often_as_the_figures_say() {
    let runs = batch();
    let mut n = 0i64;
    let (mut exact, mut neighbour) = (0i64, 0i64);
    let (mut count_3, mut count_4, mut count_2) = (0i64, 0i64, 0i64);
    let (mut cruiser, mut battleship, mut destroyer, mut carrier) = (0i64, 0i64, 0i64, 0i64);
    for run in runs {
        for report in &run.reports {
            n += 1;
            let search = search_index(report.day, report.pulse).expect("a listed search");
            let truth = SEARCHES[search].2;
            let off_by = hex_distance(report.hex, truth);
            match off_by {
                0 => exact += 1,
                1 => neighbour += 1,
                _ => panic!("report names a hex {off_by} from the truth"),
            }
            match report.count {
                3 => count_3 += 1,
                4 => count_4 += 1,
                2 => count_2 += 1,
                other => panic!("report counts {other} ships"),
            }
            match report.kind {
                ShipKind::Cruiser => cruiser += 1,
                ShipKind::Battleship => battleship += 1,
                ShipKind::Destroyer => destroyer += 1,
                ShipKind::Carrier => carrier += 1,
                other => panic!("report names kind {other:?}"),
            }
        }
    }
    eprintln!("reports n = {n}");
    eprintln!("position exact {exact}, neighbour {neighbour}");
    eprintln!("count 3 {count_3}, count 4 {count_4}, count 2 {count_2}");
    eprintln!(
        "kind cruiser {cruiser}, battleship {battleship}, destroyer {destroyer}, carrier {carrier}"
    );
    assert_eq!(exact + neighbour, n);
    assert_eq!(count_3 + count_4 + count_2, n);
    assert_eq!(cruiser + battleship + destroyer + carrier, n);
    for (name, k, p) in [
        ("position exact", exact, 700_000),
        ("position neighbour", neighbour, 300_000),
        ("count 3", count_3, 600_000),
        ("count 4", count_4, 200_000),
        ("count 2", count_2, 200_000),
        ("cruiser", cruiser, 700_000),
        ("battleship", battleship, 100_000),
        ("destroyer", destroyer, 100_000),
        ("carrier", carrier, 100_000),
    ] {
        assert!(within_four_sigma(k, n, p), "{name}: {k} of {n}");
    }
}

fn hex_distance(a: Hex, b: Hex) -> u32 {
    let dq = a.q - b.q;
    let dr = a.r - b.r;
    (dq.unsigned_abs() + dr.unsigned_abs() + (dq + dr).unsigned_abs()) / 2
}

/// A contact in the test's own model of belief.
#[derive(Clone, Debug)]
struct ModelContact {
    id: u32,
    hex: Hex,
    count: u32,
    kind: ShipKind,
    age: u32,
}

/// Replays a run's reports through the merge and ageing rules, written out
/// here from decisions 13 and 14 and the hand calculation. Asserts that each
/// report's contact ID and created flag are what the model decides.
fn replay_belief(reports: &[ReportRecord]) -> Vec<ModelContact> {
    let drop_age = 5;
    let mut contacts: Vec<ModelContact> = Vec::new();
    let mut next_id = 1u32;
    for day in 1..=4u32 {
        for pulse in Pulse::ALL {
            let mut touched: Vec<u32> = Vec::new();
            for report in reports.iter().filter(|r| r.day == day && r.pulse == pulse) {
                // Contacts are kept in ID order, so the first nearest one
                // is the lowest ID.
                let mut best: Option<(u32, u32)> = None;
                for contact in &contacts {
                    let distance = hex_distance(contact.hex, report.hex);
                    if distance <= 2 && contact.age <= 2 {
                        let better = match best {
                            Some((best_distance, _)) => distance < best_distance,
                            None => true,
                        };
                        if better {
                            best = Some((distance, contact.id));
                        }
                    }
                }
                let (id, created) = match best {
                    Some((_, id)) => (id, false),
                    None => {
                        let id = next_id;
                        next_id += 1;
                        (id, true)
                    }
                };
                assert_eq!(report.contact, id, "contact ID at D{day} {pulse:?}");
                assert_eq!(report.created, created, "created at D{day} {pulse:?}");
                let fresh = ModelContact {
                    id,
                    hex: report.hex,
                    count: report.count,
                    kind: report.kind,
                    age: 0,
                };
                match contacts.iter_mut().find(|c| c.id == id) {
                    Some(existing) => *existing = fresh,
                    None => contacts.push(fresh),
                }
                touched.push(id);
            }
            // End of the pulse: age what was not touched, drop what reaches
            // the drop age.
            for contact in contacts.iter_mut().filter(|c| !touched.contains(&c.id)) {
                contact.age += 1;
            }
            contacts.retain(|c| c.age < drop_age);
        }
    }
    contacts
}

#[test]
fn contacts_age_merge_and_drop_by_the_rules() {
    let runs = batch();
    let mut by_final_state = [0i64; 3]; // none, one, two contacts
    let mut states = [0i64; 4]; // age 3 only, age 4 only, both, other
    for run in runs {
        let model = replay_belief(&run.reports);
        let expected: Vec<ContactRecord> = model
            .iter()
            .map(|c| ContactRecord {
                id: c.id,
                hex: c.hex,
                count: c.count,
                kind: c.kind,
                age: c.age,
            })
            .collect();
        assert_eq!(run.final_contacts, expected, "seed {}", run.seed);

        // At most two contacts, each in one of the two states.
        assert!(run.final_contacts.len() <= 2);
        by_final_state[run.final_contacts.len()] += 1;
        let mut seen = [false, false];
        for contact in &run.final_contacts {
            if contact.age == 3 {
                seen[0] = true;
            } else if contact.age == 4 {
                seen[1] = true;
            } else {
                panic!("seed {}: contact in state {contact:?}", run.seed);
            }
        }
        states[match seen {
            [true, false] => 0,
            [false, true] => 1,
            [true, true] => 2,
            [false, false] => 3,
        }] += 1;

        // Belief holds nothing the reports did not say: each contact's
        // hex, count and kind are those of the last report that named it.
        for contact in &run.final_contacts {
            let last = run
                .reports
                .iter()
                .rev()
                .find(|r| r.contact == contact.id)
                .expect("a contact comes from a report");
            assert_eq!(
                (contact.hex, contact.count, contact.kind),
                (last.hex, last.count, last.kind)
            );
        }
        // The Japanese side has no search and no contacts.
        assert_eq!(run.japanese_contacts, 0);
    }
    eprintln!("runs ending with 0, 1, 2 contacts: {by_final_state:?}");
    eprintln!("final states [age 3 only, age 4 only, both, none]: {states:?}");
}

// ---------------------------------------------------------------------
// Properties.
// ---------------------------------------------------------------------

/// Loads the scenario on the frozen figures with each exact, asserted-unique
/// text replacement made in its text. Nothing in the repository's data folder
/// is touched.
fn variant(_tag: &str, edits: &[(&str, &str)]) -> Scenario {
    let path = scenario_path();
    let mut text = fs::read_to_string(&path).expect("read the scenario");
    for (original, changed) in edits {
        text = common::replace_once(&text, original, changed);
    }
    common::from_text(&text, &path).expect("the variant loads")
}

/// The chance in the day 2 morning search, 320 nm out, of a scenario run
/// with its own seed.
fn day_2_morning_chance(mut scenario: Scenario) -> Fixed {
    scenario.run_to_end().expect("the variant runs");
    let events: Vec<Event> = scenario.world.events().iter().cloned().collect();
    let found: Vec<_> = attempts(&events)
        .into_iter()
        .filter(|(day, pulse, ..)| *day == 2 && *pulse == Pulse::Morning)
        .collect();
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].2, nm(320));
    found[0].3
}

#[test]
fn more_aircraft_never_lower_the_chance() {
    let chances: Vec<Fixed> = [1, 2, 4, 8, 16]
        .iter()
        .map(|ready| {
            let scenario = variant(
                &format!("ready_{ready}"),
                &[("ready = 4", &format!("ready = {ready}"))],
            );
            day_2_morning_chance(scenario)
        })
        .collect();
    assert_eq!(
        chances,
        vec![
            millionths(119_366),
            millionths(238_732),
            millionths(477_464),
            millionths(954_929),
            Fixed::ONE,
        ]
    );
    assert!(chances.windows(2).all(|pair| pair[0] <= pair[1]));
}

#[test]
fn a_wider_arc_never_raises_the_chance() {
    // Every arc keeps bearing 090 inside. A full circle has equal bearings.
    let arcs = [(80, 110), (60, 120), (30, 150), (0, 0)];
    let chances: Vec<Fixed> = arcs
        .iter()
        .map(|(from, to)| {
            let scenario = variant(
                &format!("arc_{from}_{to}"),
                &[(
                    "from_deg = 60\nto_deg = 120",
                    &format!("from_deg = {from}\nto_deg = {to}"),
                )],
            );
            day_2_morning_chance(scenario)
        })
        .collect();
    assert_eq!(
        chances,
        vec![
            millionths(954_929),
            millionths(477_464),
            millionths(238_732),
            millionths(79_577),
        ]
    );
    assert!(chances.windows(2).all(|pair| pair[0] >= pair[1]));
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
        assert_eq!(hashes[0].len(), 4);
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
