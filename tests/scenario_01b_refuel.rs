//! Scenario 1b: a task force steams from Noumea to Espiritu Santo, refuels,
//! steams back and refuels again.
//!
//! It proves refuelling in port (decision 0045). The data is
//! `data/scenarios/01b_refuel.toml`. Everything in it is a placeholder.
//! There is no chance in it: every seed gives the same events.
//!
//! # Hand calculation
//!
//! ## The setting
//!
//! It is scenario 1 (`tests/scenario_01_transit.rs`) carried on for two
//! more days. Noumea, base 1, is at (0,0) and has 100 t of fuel. Espiritu
//! Santo, base 2, is at (8,3) and has 120 t. Both are Allied. Ships are
//! charged straight-line miles (decision 0058): the route between them is
//! 11 hexes and 394 nm either way, charged 38, 39, 37, 37, 35, 34, 30, 24, 40,
//! 40, 40 going out (scenario 1) and 27, 25, 22, 40, 40, 40, 40, 40, 40, 40,
//! 40 coming back. (The way back is a different path: from (8,3) the first
//! direction that helps is (0,-1), so it goes (8,2), (8,1), (8,0) and then
//! west along r = 0. With (0,0) as the waypoint the air distances are 394,
//! 367, 342, 320, 280, ..., 40, 0: 394 - 367 = 27, 367 - 342 = 25, 342 - 320
//! = 22, then eight 40s.)
//!
//! Task force 1 is Allied and starts stopped at Noumea:
//!
//! | Ship | Class | Fuel capacity | Starts with | Burns at cruise |
//! | ---- | ----- | ------------- | ----------- | --------------- |
//! | 1 | Placeholder cruiser | 2,000 t | 2,000 t | 0.25 t a mile |
//! | 2 | Placeholder destroyer | 500 t | 500 t | 0.125 t a mile |
//!
//! It is ordered to Espiritu Santo on day 1 and back to Noumea on day 3, at
//! its cruising 16 knots. Each way it burns 394 x 0.25 = 98.5 t (cruiser) and
//! 394 x 0.125 = 49.25 t (destroyer). Together that is 147.75 t, which is why
//! Espiritu Santo has 120 t: under decision 0058 the transit is shorter, and
//! the 150 t it held before would have filled both ships. 120 t is less than
//! the 147.75 t the force lacks, so the base still cannot fill both.
//!
//! ## The rule
//!
//! At the start of each day, before the day's orders are taken in, a task
//! force stopped at the hex of a base its side holds refuels: each ship,
//! in order of its number, takes what it lacks up to its capacity, or what
//! the base has left if that is less.
//!
//! ## Day by day
//!
//! **Day 1.** At the start the force is stopped at Noumea with both ships
//! full. Neither lacks anything: nothing is taken and nothing is recorded.
//! Noumea still has 100 t. The force steams 384 nm in the day, as in
//! scenario 1: it ends the day 30 nm along the 40 nm step from (8,2) to
//! (8,3), so in (8,3). The cruiser has burned 96 t and has 1,904 t; the
//! destroyer 48 t and has 452 t.
//!
//! **Day 2.** At the start the force is under way, 30 nm along a step: it
//! does not refuel. It arrives at Espiritu Santo after 10 nm more, 0.625
//! hours into the night pulse, 24.625 hours after the start: D2 18:38 on the
//! clock. The cruiser has burned 98.5 t and has 1,901.5 t; the destroyer has
//! burned 49.25 t and has 450.75 t.
//!
//! **Day 3.** At the start the force is stopped at Espiritu Santo, which
//! has 120 t.
//!
//! | Ship | Lacks | Takes | Has | Espiritu Santo has left |
//! | ---- | ----- | ----- | --- | ----------------------- |
//! | 1, the cruiser | 98.5 t | 98.5 t | 2,000 t | 21.5 t |
//! | 2, the destroyer | 49.25 t | 21.5 t, all that is left | 472.25 t | none |
//!
//! Then the day's order is taken in and the force steams for Noumea: 384 nm
//! in the day, to 30 nm along the step from (1,0) to (0,0) (centres at 27,
//! 52, 74, 114, ..., 354 and 394 nm; 384 is 30 past the one at 354). The
//! cruiser has 2,000 - 96 = 1,904 t and the destroyer 472.25 - 48 = 424.25 t.
//!
//! **Day 4.** At the start the force is under way and does not refuel. It
//! arrives at Noumea after 10 nm more, at 18:38 on day 4 (72.625 hours after
//! the start), 394 nm from Espiritu Santo. The cruiser has 2,000 - 98.5 =
//! 1,901.5 t and the destroyer 472.25 - 49.25 = 423 t.
//!
//! **Day 5.** At the start the force is stopped at Noumea, which has 100 t.
//!
//! | Ship | Lacks | Takes | Has | Noumea has left |
//! | ---- | ----- | ----- | --- | --------------- |
//! | 1, the cruiser | 98.5 t | 98.5 t | 2,000 t | 1.5 t |
//! | 2, the destroyer | 77 t | 1.5 t, all there is | 424.5 t | none |
//!
//! No order is given on day 5 and the force does not move.
//!
//! ## At the end
//!
//! | | Fuel |
//! | - | ---- |
//! | The cruiser | 2,000 t |
//! | The destroyer | 424.5 t |
//! | Noumea | none |
//! | Espiritu Santo | none |
//!
//! The fuel in the ships and the bases at the start, 2,000 + 500 + 100 +
//! 120 = 2,720 t, less what the ships burned, 197 + 98.5 = 295.5 t, is the
//! 2,424.5 t there at the end: 2,000 + 424.5.
//!
//! Four ships are refuelled in the run, so four refuelling events are
//! recorded: two at the start of day 3 and two at the start of day 5. Each
//! names the ship, the base, the tons taken, what the ship then has and
//! what the base has left.
//!
//! ## Variants
//!
//! | Variant | What changes |
//! | ------- | ------------ |
//! | Neither base has fuel | Nothing is refuelled. Until the order to return on day 3 the log is scenario 1's, event for event, and the force steams back on what it has: 2,000 - 197 = 1,803 t and 500 - 98.5 = 401.5 t at the end |
//! | Espiritu Santo has 1,000 t | Both ships fill up on day 3: 98.5 t and 49.25 t, leaving 852.25 t. The destroyer returns to Noumea with 500 - 49.25 = 450.75 t, and at Noumea takes the 1.5 t left after the cruiser's 98.5 t: 452.25 t |
//! | Espiritu Santo is Japanese | The force does not refuel there. At Noumea on day 5 the cruiser has 1,803 t, lacks 197 t and takes the 100 t: 1,903 t. The destroyer, 401.5 t, takes nothing |
//! | A second Allied force, task force 2 with one cruiser at 1,900 t, lies stopped at Espiritu Santo from the start | It is there before task force 1 arrives: it lacks 100 t and takes it at the start of day 1, leaving 20 t, and lacks nothing after. On day 3 task force 1's cruiser takes the 20 t (1,921.5 t) and its destroyer nothing. At Noumea on day 5 the cruiser has 1,921.5 - 98.5 = 1,823 t, lacks 177 t and takes the 100 t: 1,923 t; the destroyer, 450.75 - 49.25 = 401.5 t, takes nothing |
//!
//! ## What cannot happen
//!
//! - No ship has more fuel than its class's capacity or less than none, and
//!   no base has less than none.
//! - No ship is refuelled while its force is under way, at a base of the
//!   other side, or in a hex with no base.
//! - Nothing is refuelled in the middle of a day.
//! - No fuel appears from nowhere: fuel is conserved as "At the end" has it.
//!
//! # Properties
//!
//! - Fuel is conserved in every variant.
//! - More fuel at a base never leaves a ship with less.
//! - Every seed gives the same events: nothing here is rolled. Each seed
//!   has its own hashes, because the state hash includes the seed, and
//!   repeats them. (The first version said the hashes are the same too.)
//! - Recording breakdowns changes neither the events nor the hashes.
//! - A scenario whose bases have no fuel has the hashes it had before this
//!   stage: no existing golden file changes.

// The test code is written by a developer from the hand calculation above.

mod common;

use std::collections::BTreeSet;
use std::path::PathBuf;

use sim_core::events::{Cause, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, EventId, ShipId, TaskForceId};
use sim_core::state::{Side, TaskForceStatus};
use sim_core::time::{ClockTime, Pulse};
use sim_core::{DaySummary, SimConfig};
use sim_data::Scenario;

const FORCE: TaskForceId = TaskForceId::new(1);
const CRUISER: ShipId = ShipId::new(1);
const DESTROYER: ShipId = ShipId::new(2);
const NOUMEA: BaseId = BaseId::new(1);
const SANTO: BaseId = BaseId::new(2);

fn path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/01b_refuel.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn load() -> Scenario {
    common::load(&path()).expect("scenario 1b loads")
}

/// The scenario with its text edited before the loader sees it. Panics if
/// the edit changes nothing, so a variant cannot silently be the base case.
fn variant(edit: impl Fn(&str) -> String) -> Scenario {
    let original = text();
    let edited = edit(&original);
    assert_ne!(original, edited, "the edit changed nothing");
    common::from_text(&edited, &path()).expect("the variant loads")
}

fn tons(value: i64) -> Fixed {
    Fixed::from_int(value)
}

/// Every refuelling event in the log as (day, ship, base, taken, aboard,
/// left).
type Refuelling = (u32, ShipId, BaseId, i64, i64, i64);

fn refuellings(scenario: &Scenario) -> Vec<Refuelling> {
    scenario
        .world
        .events()
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::ShipRefuelled {
                ship,
                base,
                taken_t,
                fuel_t,
                base_fuel_t,
                ..
            } => Some((
                event.day,
                *ship,
                *base,
                taken_t.trunc(),
                fuel_t.trunc(),
                base_fuel_t.trunc(),
            )),
            _ => None,
        })
        .collect()
}

fn fuel(scenario: &Scenario, ship: ShipId) -> Fixed {
    scenario.world.umpire().ships[&ship].fuel_t
}

fn stock(scenario: &Scenario, base: BaseId) -> Fixed {
    scenario.world.umpire().bases[&base].fuel_t
}

/// Fuel in the ships and the bases together.
fn fuel_total(scenario: &Scenario) -> Fixed {
    let truth = scenario.world.umpire();
    let mut total = Fixed::ZERO;
    for ship in truth.ships.values() {
        total += ship.fuel_t;
    }
    for base in truth.bases.values() {
        total += base.fuel_t;
    }
    total
}

/// Every ton burned so far, from the log.
fn burned(scenario: &Scenario) -> Fixed {
    let mut total = Fixed::ZERO;
    for event in scenario.world.events().iter() {
        if let EventKind::FuelBurned { burned_t, .. } = &event.kind {
            total += *burned_t;
        }
    }
    total
}

/// Nothing is over capacity or below nothing.
fn assert_within_limits(scenario: &Scenario) {
    let truth = scenario.world.umpire();
    for ship in truth.ships.values() {
        let capacity = scenario.world.ref_data().ship_classes[&ship.class].fuel_capacity_t;
        assert!(ship.fuel_t >= Fixed::ZERO, "{} has less than none", ship.id);
        assert!(ship.fuel_t <= capacity, "{} is over capacity", ship.id);
    }
    for base in truth.bases.values() {
        assert!(base.fuel_t >= Fixed::ZERO, "{} has less than none", base.id);
    }
}

/// Runs the scenario to its end a day at a time. After each day it checks
/// what cannot happen: nothing out of range; fuel conserved; every
/// refuelling event is at the start of the day, before the day's orders and
/// pulses, by a force that was stopped at a centre (not under way) in the hex
/// of a base of its own side at the start of the day. Returns the fuel in
/// ships and bases together at the start.
fn run_checked(scenario: &mut Scenario) -> Fixed {
    let start_total = fuel_total(scenario);
    assert_within_limits(scenario);
    while scenario.world.day() < scenario.days {
        // What the day starts with.
        let before = scenario.world.umpire().clone();
        let summary = scenario
            .next_day()
            .expect("the day resolves")
            .expect("the day exists");
        assert_within_limits(scenario);
        assert_eq!(
            fuel_total(scenario) + burned(scenario),
            start_total,
            "fuel is conserved after day {}",
            summary.day
        );
        let day_events = scenario.world.events().from_index(summary.first_event);
        let mut seen_other = false;
        for event in day_events {
            let EventKind::ShipRefuelled {
                ship,
                task_force,
                base,
                ..
            } = &event.kind
            else {
                seen_other = true;
                continue;
            };
            // Nothing is refuelled in the middle of a day: the refuelling is
            // the first thing the day does.
            assert_eq!(event.pulse, None);
            assert!(!seen_other, "a refuelling after another event of the day");
            let force = &before.task_forces[task_force];
            assert!(force.ships.contains(ship));
            assert_ne!(force.status, TaskForceStatus::Underway);
            assert_eq!(force.toward_hex, None);
            let base = &before.bases[base];
            assert_eq!(base.side, force.side);
            assert_eq!(base.hex, force.position());
        }
    }
    start_total
}

fn arrival_events(scenario: &Scenario) -> Vec<(EventId, Hex)> {
    scenario
        .world
        .events()
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::TaskForceArrived { hex, .. } => Some((event.id, *hex)),
            _ => None,
        })
        .collect()
}

fn steamed_nm(scenario: &Scenario, day: &DaySummary) -> i64 {
    scenario.world.events().from_index(day.first_event)[..day.event_count]
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::TaskForceSteamed { distance_nm, .. } => Some(distance_nm.trunc()),
            _ => None,
        })
        .sum()
}

#[test]
fn the_run_matches_the_hand_calculation_day_by_day() {
    let mut scenario = load();
    assert!(scenario.placeholder);
    assert_eq!(scenario.days, 5);

    // The setting: two Allied bases with fuel, one force stopped at Noumea,
    // both ships full.
    {
        let truth = scenario.world.umpire();
        assert_eq!(truth.bases[&NOUMEA].hex, Hex::new(0, 0));
        assert_eq!(truth.bases[&NOUMEA].side, Side::Allied);
        assert_eq!(truth.bases[&SANTO].hex, Hex::new(8, 3));
        assert_eq!(truth.bases[&SANTO].side, Side::Allied);
        assert_eq!(truth.bases[&NOUMEA].fuel_t, tons(100));
        assert_eq!(truth.bases[&SANTO].fuel_t, tons(120));
        let force = &truth.task_forces[&FORCE];
        assert_eq!(force.side, Side::Allied);
        assert_eq!(force.position(), Hex::new(0, 0));
        assert_eq!(force.status, TaskForceStatus::Idle);
        assert_eq!(fuel(&scenario, CRUISER), tons(2000));
        assert_eq!(fuel(&scenario, DESTROYER), tons(500));
        let classes = &scenario.world.ref_data().ship_classes;
        assert_eq!(
            classes[&truth.ships[&CRUISER].class].fuel_capacity_t,
            tons(2000)
        );
        assert_eq!(
            classes[&truth.ships[&DESTROYER].class].fuel_capacity_t,
            tons(500)
        );
    }
    let total_at_start = fuel_total(&scenario);
    assert_eq!(total_at_start, tons(2000 + 500 + 100 + 120));

    // Day 1: both ships are full. Nothing is taken, nothing is recorded,
    // Noumea keeps its 100 t. The force steams 384 nm as in scenario 1.
    let day1 = scenario.next_day().expect("day 1").expect("exists");
    assert!(refuellings(&scenario).is_empty());
    assert_eq!(stock(&scenario, NOUMEA), tons(100));
    assert_eq!(stock(&scenario, SANTO), tons(120));
    assert_eq!(steamed_nm(&scenario, &day1), 384);
    {
        let force = &scenario.world.umpire().task_forces[&FORCE];
        assert_eq!(force.status, TaskForceStatus::Underway);
        assert_eq!(force.from_hex, Hex::new(8, 2));
        assert_eq!(force.toward_hex, Some(Hex::new(8, 3)));
        assert_eq!(force.progress_nm, tons(30));
        assert_eq!(force.leg_nm, tons(40));
        assert_eq!(force.position(), Hex::new(8, 3));
    }
    assert_eq!(fuel(&scenario, CRUISER), tons(2000 - 96));
    assert_eq!(fuel(&scenario, DESTROYER), tons(500 - 48));

    // Day 2: under way at the start, so no refuelling. It arrives at
    // Espiritu Santo at 18:38 (24.625 hours) after 394 nm in all (10 nm
    // today).
    let day2 = scenario.next_day().expect("day 2").expect("exists");
    assert!(refuellings(&scenario).is_empty());
    assert_eq!(steamed_nm(&scenario, &day2), 10);
    let arrivals = scenario
        .world
        .events()
        .from_index(day2.first_event)
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::TaskForceArrived {
                hex, elapsed_hours, ..
            } => Some((event.pulse, *hex, *elapsed_hours)),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(arrivals.len(), 1);
    assert_eq!(arrivals[0].0, Some(Pulse::Night));
    assert_eq!(arrivals[0].1, Hex::new(8, 3));
    assert_eq!(
        ClockTime::from_elapsed(arrivals[0].2).to_string(),
        "D2 18:38"
    );
    assert_eq!(arrivals[0].2, Fixed::from_ratio(197, 8));
    {
        let force = &scenario.world.umpire().task_forces[&FORCE];
        assert_eq!(force.status, TaskForceStatus::Arrived);
        assert_eq!(force.position(), Hex::new(8, 3));
    }
    assert_eq!(fuel(&scenario, CRUISER), Fixed::from_ratio(3803, 2));
    assert_eq!(fuel(&scenario, DESTROYER), Fixed::from_ratio(1803, 4));
    assert_eq!(stock(&scenario, SANTO), tons(120));
    let arrival_day2 = scenario
        .world
        .events()
        .from_index(day2.first_event)
        .iter()
        .find(|event| matches!(event.kind, EventKind::TaskForceArrived { .. }))
        .expect("arrival")
        .id;

    // Day 3: stopped at Espiritu Santo with 120 t. The cruiser lacks 98.5 t
    // and takes it, leaving 21.5 t. The destroyer lacks 49.25 t and takes the
    // 21.5 t left. Then the order is taken in and the force steams 384 nm.
    let day3 = scenario.next_day().expect("day 3").expect("exists");
    assert_eq!(
        refuellings(&scenario),
        vec![
            (3, CRUISER, SANTO, 98, 2000, 21),
            (3, DESTROYER, SANTO, 21, 472, 0),
        ]
    );
    assert_eq!(stock(&scenario, SANTO), Fixed::ZERO);
    assert_eq!(steamed_nm(&scenario, &day3), 384);
    {
        // The refuelling comes first in the day, before the order is taken in.
        let events = scenario.world.events().from_index(day3.first_event);
        assert!(matches!(events[0].kind, EventKind::ShipRefuelled { .. }));
        assert!(matches!(events[1].kind, EventKind::ShipRefuelled { .. }));
        assert!(matches!(events[2].kind, EventKind::OrderAccepted { .. }));
        for event in &events[..2] {
            assert_eq!(event.day, 3);
            assert_eq!(event.pulse, None);
            assert_eq!(event.causes, vec![Cause::Event(arrival_day2)]);
        }
    }
    assert_eq!(fuel(&scenario, CRUISER), tons(2000 - 96));
    assert_eq!(fuel(&scenario, DESTROYER), Fixed::from_ratio(1697, 4));
    assert_eq!(
        scenario.world.umpire().task_forces[&FORCE].status,
        TaskForceStatus::Underway
    );
    {
        // 30 nm along the step from (1,0) to (0,0).
        let force = &scenario.world.umpire().task_forces[&FORCE];
        assert_eq!(force.from_hex, Hex::new(1, 0));
        assert_eq!(force.toward_hex, Some(Hex::new(0, 0)));
        assert_eq!(force.progress_nm, tons(30));
        assert_eq!(force.position(), Hex::new(0, 0));
    }

    // Day 4: under way at the start, no refuelling. It arrives at Noumea at
    // 18:38, 394 nm from Espiritu Santo, with 1,901.5 t and 423 t.
    let day4 = scenario.next_day().expect("day 4").expect("exists");
    assert_eq!(refuellings(&scenario).len(), 2);
    assert_eq!(steamed_nm(&scenario, &day4), 10);
    let arrival_day4 = scenario
        .world
        .events()
        .from_index(day4.first_event)
        .iter()
        .find_map(|event| match &event.kind {
            EventKind::TaskForceArrived {
                hex, elapsed_hours, ..
            } => Some((event.id, event.pulse, *hex, *elapsed_hours)),
            _ => None,
        })
        .expect("the force arrives on day 4");
    assert_eq!(arrival_day4.1, Some(Pulse::Night));
    assert_eq!(arrival_day4.2, Hex::new(0, 0));
    assert_eq!(
        ClockTime::from_elapsed(arrival_day4.3).to_string(),
        "D4 18:38"
    );
    assert_eq!(arrival_day4.3, Fixed::from_ratio(581, 8));
    assert_eq!(
        steamed_nm(&scenario, &day3) + steamed_nm(&scenario, &day4),
        384 + 10
    );
    assert_eq!(fuel(&scenario, CRUISER), Fixed::from_ratio(3803, 2));
    assert_eq!(fuel(&scenario, DESTROYER), tons(423));
    assert_eq!(stock(&scenario, NOUMEA), tons(100));

    // Day 5: stopped at Noumea with 100 t. The cruiser lacks 98.5 t and takes
    // it, leaving 1.5 t; the destroyer lacks 77 t and takes the 1.5 t. No
    // order is given and the force does not move.
    let day5 = scenario.next_day().expect("day 5").expect("exists");
    assert_eq!(
        refuellings(&scenario),
        vec![
            (3, CRUISER, SANTO, 98, 2000, 21),
            (3, DESTROYER, SANTO, 21, 472, 0),
            (5, CRUISER, NOUMEA, 98, 2000, 1),
            (5, DESTROYER, NOUMEA, 1, 424, 0),
        ]
    );
    // The two refuellings, and the two bases' counts at the end of the day
    // (decision 0067).
    assert_eq!(day5.event_count, 2 + 2);
    let last = scenario.world.events().from_index(day5.first_event);
    assert!(last[2..]
        .iter()
        .all(|event| matches!(event.kind, EventKind::BaseAtDayEnd { .. })));
    assert_eq!(last[0].causes, vec![Cause::Event(arrival_day4.0)]);
    assert_eq!(last[1].causes, vec![Cause::Event(arrival_day4.0)]);
    assert_eq!(steamed_nm(&scenario, &day5), 0);
    assert_eq!(
        scenario.world.umpire().task_forces[&FORCE].position(),
        Hex::new(0, 0)
    );

    // The end.
    assert_eq!(fuel(&scenario, CRUISER), tons(2000));
    assert_eq!(fuel(&scenario, DESTROYER), Fixed::from_ratio(849, 2));
    assert_eq!(stock(&scenario, NOUMEA), Fixed::ZERO);
    assert_eq!(stock(&scenario, SANTO), Fixed::ZERO);
    // 2,000 + 500 + 100 + 120 less 197 + 98.5 burned is 2,424.5.
    assert_eq!(burned(&scenario), Fixed::from_ratio(591, 2));
    assert_eq!(fuel_total(&scenario), Fixed::from_ratio(4849, 2));
    assert_eq!(fuel_total(&scenario) + burned(&scenario), total_at_start);
    assert_eq!(
        fuel(&scenario, CRUISER) + fuel(&scenario, DESTROYER),
        Fixed::from_ratio(4849, 2)
    );

    // Four ships refuelled, so four events: two at the start of day 3 and
    // two at the start of day 5.
    assert_eq!(refuellings(&scenario).len(), 4);
    assert!(scenario.next_day().expect("past the end").is_none());
}

#[test]
fn the_refuelling_events_say_what_the_hand_calculation_says() {
    let mut scenario = load();
    scenario.run_to_end().expect("the run resolves");
    let events: Vec<&Event> = scenario
        .world
        .events()
        .iter()
        .filter(|event| matches!(event.kind, EventKind::ShipRefuelled { .. }))
        .collect();
    assert_eq!(events.len(), 4);
    let fields = |event: &Event| match &event.kind {
        EventKind::ShipRefuelled {
            ship,
            task_force,
            base,
            taken_t,
            fuel_t,
            base_fuel_t,
            force_restored,
        } => (
            *ship,
            *task_force,
            *base,
            *taken_t,
            *fuel_t,
            *base_fuel_t,
            *force_restored,
        ),
        _ => unreachable!(),
    };
    let half = |twice: i64| Fixed::from_ratio(twice, 2);
    assert_eq!(
        fields(events[0]),
        (
            CRUISER,
            FORCE,
            SANTO,
            half(197),
            tons(2000),
            half(43),
            false
        )
    );
    assert_eq!(
        fields(events[1]),
        (
            DESTROYER,
            FORCE,
            SANTO,
            half(43),
            Fixed::from_ratio(1889, 4),
            Fixed::ZERO,
            false
        )
    );
    assert_eq!(
        fields(events[2]),
        (
            CRUISER,
            FORCE,
            NOUMEA,
            half(197),
            tons(2000),
            half(3),
            false
        )
    );
    assert_eq!(
        fields(events[3]),
        (
            DESTROYER,
            FORCE,
            NOUMEA,
            half(3),
            half(849),
            Fixed::ZERO,
            false
        )
    );
    // In the plain style of the fuel events, saying plainly when a base has
    // run out.
    assert!(events[0].to_string().ends_with(
        "ship:1 took 98.5 t of fuel at base:2, 2000 t aboard, base:2 has 21.5 t left  <- #17"
    ));
    assert!(events[1]
        .to_string()
        .contains("ship:2 took 21.5 t of fuel at base:2, 472.25 t aboard, base:2 is out of fuel"));
    assert!(events[2]
        .to_string()
        .contains("ship:1 took 98.5 t of fuel at base:1, 2000 t aboard, base:1 has 1.5 t left"));
    assert!(events[3]
        .to_string()
        .contains("ship:2 took 1.5 t of fuel at base:1, 424.5 t aboard, base:1 is out of fuel"));
    // They are numbered in the order they happen, with the arrivals the
    // force came to rest by as their causes.
    let arrivals = arrival_events(&scenario);
    assert_eq!(arrivals.len(), 2);
    assert!(
        events[0].id < events[1].id && events[1].id < events[2].id && events[2].id < events[3].id
    );
    assert_eq!(events[0].causes, vec![Cause::Event(arrivals[0].0)]);
    assert_eq!(events[1].causes, vec![Cause::Event(arrivals[0].0)]);
    assert_eq!(events[2].causes, vec![Cause::Event(arrivals[1].0)]);
    assert_eq!(events[3].causes, vec![Cause::Event(arrivals[1].0)]);
    assert_eq!(arrivals[0].1, Hex::new(8, 3));
    assert_eq!(arrivals[1].1, Hex::new(0, 0));
    // Their causes lead back through the movement to the orders.
    let trace = scenario.world.events().trace(events[0].id);
    assert!(trace.iter().any(|cause| matches!(cause, Cause::Order(_))));
    // A refuelling is recorded for no ship that took nothing, and no
    // breakdown is recorded for it.
    assert!(events.iter().all(|event| event.breakdown.is_none()));
}

#[test]
fn the_run_obeys_what_cannot_happen() {
    let mut scenario = load();
    run_checked(&mut scenario);
}

/// What a run leaves: the hash of every day and the whole event log.
fn outcome(mut scenario: Scenario) -> (Vec<u64>, Vec<Event>) {
    let hashes: Vec<u64> = scenario
        .run_to_end()
        .expect("the run resolves")
        .iter()
        .map(|summary| summary.state_hash)
        .collect();
    let events: Vec<Event> = scenario.world.events().iter().cloned().collect();
    (hashes, events)
}

fn without_breakdowns(events: &[Event]) -> Vec<Event> {
    events
        .iter()
        .cloned()
        .map(|mut event| {
            event.breakdown = None;
            event
        })
        .collect()
}

#[test]
fn recording_breakdowns_changes_neither_the_events_nor_the_hashes() {
    let run = |record_breakdowns: bool| {
        let mut scenario = load();
        scenario.world.set_config(SimConfig { record_breakdowns });
        outcome(scenario)
    };
    let (with_hashes, with_events) = run(true);
    let (without_hashes, without_events) = run(false);
    assert_eq!(with_hashes, without_hashes);
    // The events are equal apart from the breakdowns of the steaming, which
    // are there in one and not the other.
    assert!(with_events.iter().any(|event| event.breakdown.is_some()));
    assert!(without_events.iter().all(|event| event.breakdown.is_none()));
    assert_eq!(without_breakdowns(&with_events), without_events);
    // A refuelling never has one either way.
    assert!(with_events
        .iter()
        .filter(|event| matches!(event.kind, EventKind::ShipRefuelled { .. }))
        .all(|event| event.breakdown.is_none()));
}

#[test]
fn every_seed_gives_the_same_events_and_each_seed_repeats_its_hashes() {
    let (own_hashes, own_events) = outcome(load());
    assert_eq!(own_hashes.len(), 5);
    let mut seen_hashes = BTreeSet::new();
    for seed in 1..=20 {
        let (hashes, events) = outcome(load().with_seed(seed).expect("a seed can be set"));
        // Nothing here is rolled: the events are the scenario's own, to the
        // last field.
        assert_eq!(events, own_events, "seed {seed}");
        // The seed is part of the state hash (decision 0004), so the hashes
        // are the seed's own; they repeat for a seed run again.
        let (again, _) = outcome(load().with_seed(seed).expect("a seed can be set"));
        assert_eq!(hashes, again, "seed {seed}");
        assert!(
            seen_hashes.insert(hashes),
            "seed {seed} repeats a hash list"
        );
    }
}

#[test]
fn a_replay_reproduces_every_daily_hash() {
    let mut first = load();
    let hashes: Vec<u64> = first
        .run_to_end()
        .expect("first run")
        .iter()
        .map(|summary| summary.state_hash)
        .collect();
    let mut second = load();
    second.run_to_end().expect("second run");
    assert_eq!(second.world.day_hashes(), hashes.as_slice());
    // The fuel changes on days 3 and 5, so those hashes differ from the one
    // before, as every day's does.
    let distinct: BTreeSet<u64> = hashes.iter().copied().collect();
    assert_eq!(distinct.len(), 5);
}

// ---- Variants ----

#[test]
fn neither_base_has_fuel_so_nothing_is_refuelled() {
    let mut scenario = variant(|text| {
        text.replace("fuel_t = 100\n", "")
            .replace("fuel_t = 120\n", "")
    });
    // The bases give no fuel: the force steams back on what it has.
    assert_eq!(stock(&scenario, NOUMEA), Fixed::ZERO);
    assert_eq!(stock(&scenario, SANTO), Fixed::ZERO);
    let total = run_checked(&mut scenario);
    assert_eq!(total, tons(2500));
    assert!(refuellings(&scenario).is_empty());
    assert_eq!(fuel(&scenario, CRUISER), tons(1803));
    assert_eq!(fuel(&scenario, DESTROYER), Fixed::from_ratio(803, 2));

    // The first three days are scenario 1, event for event: its whole log is
    // the start of this one (the same numbers, causes and kinds), and what
    // follows it is the order to go back and the steaming of day 3 and 4.
    let mut transit = common::load(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/01_transit.toml"),
    )
    .expect("scenario 1 loads");
    let transit_hashes: Vec<u64> = transit
        .run_to_end()
        .expect("scenario 1 runs")
        .iter()
        .map(|summary| summary.state_hash)
        .collect();
    // Scenario 1 through day 2. Its day 3 holds only the bases' counts at
    // the end of the day (decision 0067), which come after this scenario's
    // own events of day 3.
    let transit_events: Vec<Event> = transit
        .world
        .events()
        .iter()
        .filter(|event| event.day <= 2)
        .cloned()
        .collect();
    let ours: Vec<Event> = scenario.world.events().iter().cloned().collect();
    assert_eq!(ours[..transit_events.len()], transit_events[..]);
    assert!(ours[transit_events.len()..]
        .iter()
        .all(|event| event.day >= 3));
    // And the state hash of day 1 and day 2 is scenario 1's: a stock of no
    // fuel changes no hash.
    assert_eq!(scenario.world.day_hashes()[..2], transit_hashes[..2]);
}

#[test]
fn espiritu_santo_with_a_thousand_tons_fills_both_ships() {
    let mut scenario = variant(|text| text.replace("fuel_t = 120\n", "fuel_t = 1000\n"));
    run_checked(&mut scenario);
    assert_eq!(
        refuellings(&scenario),
        vec![
            (3, CRUISER, SANTO, 98, 2000, 901),
            (3, DESTROYER, SANTO, 49, 500, 852),
            // At Noumea on day 5: the cruiser lacks 98.5 t and takes it,
            // leaving 1.5 t; the destroyer, 450.75 t on arrival, lacks 49.25
            // t and gets the 1.5 t.
            (5, CRUISER, NOUMEA, 98, 2000, 1),
            (5, DESTROYER, NOUMEA, 1, 452, 0),
        ]
    );
    assert_eq!(stock(&scenario, SANTO), Fixed::from_ratio(3409, 4));
    assert_eq!(fuel(&scenario, CRUISER), tons(2000));
    // The destroyer returns to Noumea with 450.75 t and ends with 452.25 t.
    assert_eq!(fuel(&scenario, DESTROYER), Fixed::from_ratio(1809, 4));
}

#[test]
fn a_force_does_not_refuel_at_a_base_of_the_other_side() {
    let mut scenario = variant(|text| {
        text.replace(
            "name = \"Espiritu Santo\"\nside = \"allied\"",
            "name = \"Espiritu Santo\"\nside = \"japanese\"",
        )
    });
    assert_eq!(scenario.world.umpire().bases[&SANTO].side, Side::Japanese);
    run_checked(&mut scenario);
    // Nothing at Espiritu Santo; at Noumea on day 5 the cruiser, 1,803 t,
    // lacks 197 t and takes the 100 t.
    assert_eq!(
        refuellings(&scenario),
        vec![(5, CRUISER, NOUMEA, 100, 1903, 0)]
    );
    assert_eq!(stock(&scenario, SANTO), tons(120));
    assert_eq!(fuel(&scenario, DESTROYER), Fixed::from_ratio(803, 2));
}

#[test]
fn a_force_does_not_refuel_in_a_hex_with_no_base() {
    // Espiritu Santo's base is a hex short of where the force stops.
    let mut scenario = variant(|text| {
        text.replace(
            "name = \"Espiritu Santo\"\nside = \"allied\"\nq = 8\nr = 3\n",
            "name = \"Espiritu Santo\"\nside = \"allied\"\nq = 8\nr = 2\n",
        )
    });
    run_checked(&mut scenario);
    assert_eq!(
        refuellings(&scenario),
        vec![(5, CRUISER, NOUMEA, 100, 1903, 0)]
    );
    assert_eq!(stock(&scenario, SANTO), tons(120));
}

#[test]
fn a_second_force_already_at_espiritu_santo_is_served_first() {
    let mut scenario = variant(|text| {
        format!(
            "{text}\n[[task_force]]\nid = 2\nname = \"Placeholder second force\"\nside = \"allied\"\nat_base = 2\nmission = \"transport\"\n\n[[ship]]\nid = 3\nname = \"Placeholder cruiser 2\"\nclass = 1\nside = \"allied\"\nfuel_t = 1900\ntask_force = 2\n"
        )
    });
    let ship3 = ShipId::new(3);
    let total = run_checked(&mut scenario);
    assert_eq!(total, tons(2720 + 1900));
    // It lacks 100 t and takes it at the start of day 1, leaving 20 t; then
    // it lacks nothing. On day 3 force 1's cruiser takes the 20 t (1,921.5 t)
    // and its destroyer nothing.
    assert_eq!(
        refuellings(&scenario),
        vec![
            (1, ship3, SANTO, 100, 2000, 20),
            (3, CRUISER, SANTO, 20, 1921, 0),
            // On day 5 at Noumea: the cruiser, 1,823 t, lacks 177 t and takes
            // the 100 t; the destroyer nothing.
            (5, CRUISER, NOUMEA, 100, 1923, 0),
        ]
    );
    // No arrival, no order: the first refuelling has no cause.
    let first = scenario
        .world
        .events()
        .iter()
        .find(|event| matches!(event.kind, EventKind::ShipRefuelled { .. }))
        .expect("an event");
    assert_eq!(first.day, 1);
    assert!(first.causes.is_empty());
    assert_eq!(fuel(&scenario, ship3), tons(2000));
    assert_eq!(fuel(&scenario, DESTROYER), Fixed::from_ratio(803, 2));
}

// ---- Properties ----

/// The scenario with the two bases' stocks set, run to its end.
fn with_stocks(noumea: i64, santo: i64) -> Scenario {
    let mut scenario = variant(|text| {
        text.replace("fuel_t = 100\n", &format!("fuel_t = {noumea}\n"))
            .replace("fuel_t = 120\n", &format!("fuel_t = {santo}\n"))
    });
    run_checked(&mut scenario);
    scenario
}

#[test]
fn fuel_is_conserved_and_more_fuel_never_leaves_a_ship_with_less() {
    // Espiritu Santo's stock rising, Noumea's fixed; then the other way.
    let mut last: Option<(Fixed, Fixed)> = None;
    for santo in [0, 50, 147, 148, 300, 1000, 5000] {
        let scenario = with_stocks(101, santo);
        let now = (fuel(&scenario, CRUISER), fuel(&scenario, DESTROYER));
        if let Some(before) = last {
            assert!(now.0 >= before.0, "cruiser less with {santo} t at Santo");
            assert!(now.1 >= before.1, "destroyer less with {santo} t at Santo");
        }
        last = Some(now);
    }
    let mut last: Option<(Fixed, Fixed)> = None;
    for noumea in [0, 50, 99, 101, 110, 500, 5000] {
        let scenario = with_stocks(noumea, 120);
        let now = (fuel(&scenario, CRUISER), fuel(&scenario, DESTROYER));
        if let Some(before) = last {
            assert!(now.0 >= before.0, "cruiser less with {noumea} t at Noumea");
            assert!(
                now.1 >= before.1,
                "destroyer less with {noumea} t at Noumea"
            );
        }
        last = Some(now);
    }
}

#[test]
fn scenarios_whose_bases_have_no_fuel_have_no_refuelling() {
    // The golden files of every other scenario are compared by the runner's
    // tests; here, two of them directly.
    for name in ["01_transit.toml", "02_supply_run.toml"] {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../data/scenarios")
            .join(name);
        let mut scenario = common::load(&path).expect("the scenario loads");
        assert!(scenario
            .world
            .umpire()
            .bases
            .values()
            .all(|base| base.fuel_t == Fixed::ZERO));
        scenario.run_to_end().expect("the run resolves");
        assert!(refuellings(&scenario).is_empty(), "{name}");
    }
}
