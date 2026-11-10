//! Scenario 2: a convoy loads supply at Espiritu Santo and unloads it at
//! Guadalcanal.
//!
//! Proves that loading, port capacity limits and base supply levels match a
//! hand calculation, that the escort's fuel burn is floored at its cruise
//! rate, that supply is conserved, and that a replay reproduces every daily
//! hash.
//!
//! The data is placeholder data: see `data/scenarios/02_supply_run.toml`.
//! The rules are in `docs/decisions/0011-cargo-handling.md` and
//! `docs/decisions/0006-fuel-model.md`.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. The tests below assert these
//! numbers exactly. If the code disagrees, the code or the rule is wrong:
//! do not change a number here to make a test pass.
//!
//! Inputs, from the data files:
//!
//! - Task force 1: cargo ships 1 and 2, destroyer 3.
//! - Cargo ship: cruises at 10 kn, 1000 t of fuel lasts 10000 nm, so it burns
//!   0.1 t per nm. It carries 3000 t of supply.
//! - Destroyer: cruises at 16 kn, 500 t of fuel lasts 4000 nm, so it burns
//!   0.125 t per nm at cruise. It carries no supply.
//! - At the "cruise" setting the task force steams at its slowest ship's
//!   cruise speed: 10 kn.
//! - Espiritu Santo, base 1 at (0,0): port size 5, which is 4 berths at
//!   150 t per hour. Stock 10000 t.
//! - Guadalcanal, base 2 at (8,0): port size 1, which is 1 berth at 50 t per
//!   hour. Lighters move 20 t per hour. Stock 500 t.
//! - Route: (0,0) to (8,0), 8 hexes, 320 nm.
//! - A day is a 12-hour night pulse, then two 6-hour pulses. It starts at
//!   18:00. Cargo moves at the end of each pulse, after movement.
//! - Total supply in the world: 10000 + 500 = 10500 t, at all times.
//!
//! ## Day 1: loading at Espiritu Santo
//!
//! Order: load. Two ships have cargo to move and the port has four berths, so
//! both work at 150 t per hour.
//!
//! | Pulse     | Hours worked | Each ship loads | Each ship aboard | Base 1 stock |
//! | --------- | ------------ | --------------- | ---------------- | ------------ |
//! | Night     | 12           | 1800 t          | 1800 t           | 6400 t       |
//! | Morning   | 6            | 900 t           | 2700 t           | 4600 t       |
//! | Afternoon | 2            | 300 t           | 3000 t           | 4000 t       |
//!
//! Each ship is full two hours into the afternoon pulse. Loading is complete
//! 20 hours after the start, at 14:00 on day 1. The destroyer loads nothing.
//! The task force does not move, so no fuel is burned.
//!
//! ## Day 2: under way
//!
//! Orders: set route to (8,0), set speed cruise, unload. The unload order
//! waits for arrival.
//!
//! | Pulse     | Hours | Distance | Total  | Hex left, miles on | Position |
//! | --------- | ----- | -------- | ------ | ------------------ | -------- |
//! | Night     | 12    | 120 nm   | 120 nm | (3,0) plus 0 nm    | (3,0)    |
//! | Morning   | 6     | 60 nm    | 180 nm | (4,0) plus 20 nm   | (5,0)    |
//! | Afternoon | 6     | 60 nm    | 240 nm | (6,0) plus 0 nm    | (6,0)    |
//!
//! A task force is in the hex whose centre is nearest, and at exactly 20 nm
//! it is in the hex ahead (decision 0005). So at the end of the morning
//! pulse, 20 nm out of (4,0) toward (5,0), its position is (5,0). Hexes
//! entered: night (1,0), (2,0), (3,0); morning (4,0), (5,0); afternoon
//! (6,0). On day 3 it enters (7,0) and (8,0).
//!
//! Fuel burned on day 2:
//!
//! - Each cargo ship: 240 x 0.1 = 24 t (12, 6 and 6 by pulse). 976 t remain.
//! - Destroyer: 10 kn is below its 16 kn cruise speed, so the multiplier is
//!   floored at 1 and it burns its cruise rate: 240 x 0.125 = 30 t (15, 7.5
//!   and 7.5 by pulse). 470 t remain.
//!
//! No cargo moves on day 2. Base 1 holds 4000 t, base 2 holds 500 t and the
//! ships hold 6000 t.
//!
//! ## Day 3: arrival and the first unloading
//!
//! 80 nm remain. At 10 kn that takes 8 hours, so the task force arrives 56
//! hours after the start, at 02:00 on day 3, during the night pulse. Fuel
//! burned on the leg: each cargo ship 8 t (968 t remain), destroyer 10 t
//! (460 t remain).
//!
//! Fuel totals for the transit: each cargo ship 32 t, destroyer 40 t. Without
//! the floor the destroyer would have burned 320 x 0.125 x (10/16)^2 =
//! 15.625 t. The test asserts 40 t.
//!
//! Unloading starts in the same night pulse, for the 4 hours that are left of
//! it. Guadalcanal has one berth. Ship 1 has the lower ID, so it takes the
//! berth at 50 t per hour. Ship 2 works by lighter at 20 t per hour.
//!
//! ## Days 3 to 7: unloading at Guadalcanal
//!
//! | Day | Pulse     | Hours | Ship 1 unloads | Ship 1 aboard | Ship 2 unloads | Ship 2 aboard | Base 2 stock |
//! | --- | --------- | ----- | -------------- | ------------- | -------------- | ------------- | ------------ |
//! | 3   | Night     | 4     | 200 t          | 2800 t        | 80 t           | 2920 t        | 780 t        |
//! | 3   | Morning   | 6     | 300 t          | 2500 t        | 120 t          | 2800 t        | 1200 t       |
//! | 3   | Afternoon | 6     | 300 t          | 2200 t        | 120 t          | 2680 t        | 1620 t       |
//! | 4   | Night     | 12    | 600 t          | 1600 t        | 240 t          | 2440 t        | 2460 t       |
//! | 4   | Morning   | 6     | 300 t          | 1300 t        | 120 t          | 2320 t        | 2880 t       |
//! | 4   | Afternoon | 6     | 300 t          | 1000 t        | 120 t          | 2200 t        | 3300 t       |
//! | 5   | Night     | 12    | 600 t          | 400 t         | 240 t          | 1960 t        | 4140 t       |
//! | 5   | Morning   | 6     | 300 t          | 100 t         | 120 t          | 1840 t        | 4560 t       |
//! | 5   | Afternoon | 6     | 100 t          | 0 t           | 120 t          | 1720 t        | 4780 t       |
//! | 6   | Night     | 12    | none           | 0 t           | 600 t          | 1120 t        | 5380 t       |
//! | 6   | Morning   | 6     | none           | 0 t           | 300 t          | 820 t         | 5680 t       |
//! | 6   | Afternoon | 6     | none           | 0 t           | 300 t          | 520 t         | 5980 t       |
//! | 7   | Night     | 12    | none           | 0 t           | 520 t          | 0 t           | 6500 t       |
//!
//! Notes on the table:
//!
//! - Day 5, afternoon: ship 1 has 100 t left and empties in 2 hours. Ship 2
//!   stays on lighters for that whole pulse, because a berth assignment holds
//!   for the pulse.
//! - From day 6, ship 2 is the only ship with cargo, so it takes the berth at
//!   50 t per hour.
//! - Day 7, night: ship 2 has 520 t left. At 50 t per hour that takes 10.4
//!   hours. Unloading is complete 144 + 10.4 = 154.4 hours after the start,
//!   at 04:24 on day 7.
//!
//! End state: base 1 holds 4000 t, base 2 holds 6500 t, every ship is empty.
//! 4000 + 6500 = 10500 t.
//!
//! Base 2 stock at the end of each day: day 1 500 t, day 2 500 t, day 3
//! 1620 t, day 4 3300 t, day 5 4780 t, day 6 5980 t, day 7 6500 t, day 8
//! 6500 t.
//!
//! ## Day 8
//!
//! No new orders. Nothing moves, no cargo is left and no fuel is burned.
//!
//! # Properties
//!
//! Beside the exact numbers, the tests check what must always hold:
//!
//! - Supply is conserved: ships plus bases hold 10500 t at the end of every
//!   day.
//! - A larger destination port never finishes unloading later. With the
//!   destination at port size 0, 1, 2 and 3, the hour unloading completes
//!   never rises. Size 0 has no berth: both ships use lighters, 6000 t at
//!   40 t per hour is 150 hours, so unloading is complete 206 hours after the
//!   start and needs a longer run than 8 days.
//! - No ship ever holds more than its cargo capacity, and no stock goes
//!   negative. These are also end-of-day invariants.

use std::fs;
use std::path::{Path, PathBuf};

use sim_core::events::{Cause, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, EntityRef, ShipId, TaskForceId};
use sim_core::orders::{Issuer, OrderKind, TaskForceOrder};
use sim_core::state::{Side, TaskForceStatus};
use sim_core::time::{ClockTime, Pulse};
use sim_core::World;
use sim_data::Scenario;

const TASK_FORCE: TaskForceId = TaskForceId::new(1);
const CARGO_1: ShipId = ShipId::new(1);
const CARGO_2: ShipId = ShipId::new(2);
const DESTROYER: ShipId = ShipId::new(3);
const BASE_1: BaseId = BaseId::new(1);
const BASE_2: BaseId = BaseId::new(2);
const TOTAL_SUPPLY_T: i64 = 10_500;

fn scenario_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/02_supply_run.toml")
}

fn load() -> Scenario {
    Scenario::load(&scenario_path()).expect("scenario 2 loads")
}

fn tons(value: i64) -> Fixed {
    Fixed::from_int(value)
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

fn ship_supply(world: &World, ship: ShipId) -> Fixed {
    world.umpire().ships[&ship].supply_t
}

fn ship_fuel(world: &World, ship: ShipId) -> Fixed {
    world.umpire().ships[&ship].fuel_t
}

fn base_stock(world: &World, base: BaseId) -> Fixed {
    world.umpire().bases[&base].supply_t
}

/// The cargo-moved events as (pulse, ship, tons moved, aboard after).
fn cargo_moves(events: &[Event]) -> Vec<(Pulse, ShipId, Fixed, Fixed)> {
    events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::CargoLoaded {
                ship,
                loaded_t,
                aboard_t,
                ..
            } => Some((
                event.pulse.expect("cargo has a pulse"),
                *ship,
                *loaded_t,
                *aboard_t,
            )),
            EventKind::CargoUnloaded {
                ship,
                unloaded_t,
                aboard_t,
                ..
            } => Some((
                event.pulse.expect("cargo has a pulse"),
                *ship,
                *unloaded_t,
                *aboard_t,
            )),
            _ => None,
        })
        .collect()
}

/// The base stock after the last cargo event of a pulse, if there was one.
fn stock_after_pulse(events: &[Event], pulse: Pulse) -> Option<Fixed> {
    events
        .iter()
        .filter(|event| event.pulse == Some(pulse))
        .filter_map(|event| match &event.kind {
            EventKind::CargoLoaded { base_stock_t, .. }
            | EventKind::CargoUnloaded { base_stock_t, .. } => Some(*base_stock_t),
            _ => None,
        })
        .next_back()
}

/// The fuel burned by a ship as (pulse, tons), in the order logged.
fn fuel_burns(events: &[Event], ship: ShipId) -> Vec<(Pulse, Fixed)> {
    events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::FuelBurned {
                ship: burned_ship,
                burned_t,
                ..
            } if *burned_ship == ship => Some((event.pulse.expect("burn has a pulse"), *burned_t)),
            _ => None,
        })
        .collect()
}

/// Movement events as (pulse, distance, position at the start, position at
/// the end, hexes entered).
fn steams(events: &[Event]) -> Vec<(Pulse, Fixed, Hex, Hex, Vec<Hex>)> {
    events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::TaskForceSteamed {
                distance_nm,
                from,
                to,
                hexes_entered,
                ..
            } => Some((
                event.pulse.expect("movement has a pulse"),
                *distance_nm,
                *from,
                *to,
                hexes_entered.clone(),
            )),
            _ => None,
        })
        .collect()
}

/// Asserts the stored leg state and the position of the task force.
fn assert_leg(world: &World, from: Hex, toward: Option<Hex>, progress: i64, position: Hex) {
    let task_force = &world.umpire().task_forces[&TASK_FORCE];
    assert_eq!(task_force.from_hex, from);
    assert_eq!(task_force.toward_hex, toward);
    assert_eq!(task_force.progress_nm, tons(progress));
    assert_eq!(task_force.position(), position);
}

fn completions(events: &[Event], loading: bool) -> Vec<(Option<Pulse>, Fixed)> {
    events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::LoadingComplete { elapsed_hours, .. } if loading => {
                Some((event.pulse, *elapsed_hours))
            }
            EventKind::UnloadingComplete { elapsed_hours, .. } if !loading => {
                Some((event.pulse, *elapsed_hours))
            }
            _ => None,
        })
        .collect()
}

#[test]
fn the_supply_run_matches_the_hand_calculation() {
    let mut scenario = load();
    assert!(scenario.placeholder);
    assert_eq!(scenario.days, 8);

    // Before day 1.
    {
        let world = &scenario.world;
        assert_eq!(base_stock(world, BASE_1), tons(10000));
        assert_eq!(base_stock(world, BASE_2), tons(500));
        assert_eq!(ship_fuel(world, CARGO_1), tons(1000));
        assert_eq!(ship_fuel(world, CARGO_2), tons(1000));
        assert_eq!(ship_fuel(world, DESTROYER), tons(500));
        assert_leg(world, Hex::new(0, 0), None, 0, Hex::new(0, 0));
    }

    let mut all_events: Vec<Event> = Vec::new();
    let mut base_2_by_day: Vec<Fixed> = Vec::new();

    // Day 1: loading at Espiritu Santo.
    let (day, events) = run_day(&mut scenario);
    assert_eq!(day, 1);
    assert_eq!(
        cargo_moves(&events),
        vec![
            (Pulse::Night, CARGO_1, tons(1800), tons(1800)),
            (Pulse::Night, CARGO_2, tons(1800), tons(1800)),
            (Pulse::Morning, CARGO_1, tons(900), tons(2700)),
            (Pulse::Morning, CARGO_2, tons(900), tons(2700)),
            (Pulse::Afternoon, CARGO_1, tons(300), tons(3000)),
            (Pulse::Afternoon, CARGO_2, tons(300), tons(3000)),
        ]
    );
    assert!(events
        .iter()
        .all(|event| !matches!(event.kind, EventKind::CargoUnloaded { .. })));
    assert_eq!(stock_after_pulse(&events, Pulse::Night), Some(tons(6400)));
    assert_eq!(stock_after_pulse(&events, Pulse::Morning), Some(tons(4600)));
    assert_eq!(
        stock_after_pulse(&events, Pulse::Afternoon),
        Some(tons(4000))
    );
    // The destroyer carries no supply and loads nothing.
    assert!(events.iter().all(|event| !matches!(
        event.kind,
        EventKind::CargoLoaded { ship, .. } if ship == DESTROYER
    )));
    let loading = completions(&events, true);
    assert_eq!(loading.len(), 1);
    assert_eq!(loading[0].0, Some(Pulse::Afternoon));
    assert_eq!(loading[0].1, tons(20));
    assert_eq!(
        ClockTime::from_elapsed(loading[0].1).to_string(),
        "D1 14:00"
    );
    // The task force stays in port: nothing steams, nothing burns.
    assert!(steams(&events).is_empty());
    assert!(events
        .iter()
        .all(|event| !matches!(event.kind, EventKind::FuelBurned { .. })));
    {
        let world = &scenario.world;
        assert_leg(world, Hex::new(0, 0), None, 0, Hex::new(0, 0));
        assert_eq!(base_stock(world, BASE_1), tons(4000));
        assert_eq!(ship_supply(world, CARGO_1), tons(3000));
        assert_eq!(ship_supply(world, CARGO_2), tons(3000));
        assert_eq!(ship_supply(world, DESTROYER), Fixed::ZERO);
        assert_eq!(ship_fuel(world, CARGO_1), tons(1000));
        base_2_by_day.push(base_stock(world, BASE_2));
    }
    all_events.extend(events);

    // Day 2: under way.
    let (day, events) = run_day(&mut scenario);
    assert_eq!(day, 2);
    let hex = Hex::new;
    assert_eq!(
        steams(&events),
        vec![
            (
                Pulse::Night,
                tons(120),
                hex(0, 0),
                hex(3, 0),
                vec![hex(1, 0), hex(2, 0), hex(3, 0)]
            ),
            // Ends 20 nm out of (4,0), so it is in (5,0).
            (
                Pulse::Morning,
                tons(60),
                hex(3, 0),
                hex(5, 0),
                vec![hex(4, 0), hex(5, 0)]
            ),
            // Already in (5,0): reaching its centre enters nothing.
            (
                Pulse::Afternoon,
                tons(60),
                hex(5, 0),
                hex(6, 0),
                vec![hex(6, 0)]
            ),
        ]
    );
    for cargo_ship in [CARGO_1, CARGO_2] {
        assert_eq!(
            fuel_burns(&events, cargo_ship),
            vec![
                (Pulse::Night, tons(12)),
                (Pulse::Morning, tons(6)),
                (Pulse::Afternoon, tons(6)),
            ]
        );
    }
    assert_eq!(
        fuel_burns(&events, DESTROYER),
        vec![
            (Pulse::Night, tons(15)),
            (Pulse::Morning, Fixed::from_ratio(15, 2)),
            (Pulse::Afternoon, Fixed::from_ratio(15, 2)),
        ]
    );
    assert!(cargo_moves(&events).is_empty());
    {
        let world = &scenario.world;
        let task_force = &world.umpire().task_forces[&TASK_FORCE];
        assert_eq!(task_force.status, TaskForceStatus::Underway);
        assert_eq!(task_force.from_hex, Hex::new(6, 0));
        assert_eq!(task_force.toward_hex, None);
        assert_eq!(task_force.progress_nm, Fixed::ZERO);
        assert_eq!(task_force.position(), Hex::new(6, 0));
        assert_eq!(ship_fuel(world, CARGO_1), tons(976));
        assert_eq!(ship_fuel(world, CARGO_2), tons(976));
        assert_eq!(ship_fuel(world, DESTROYER), tons(470));
        assert_eq!(base_stock(world, BASE_1), tons(4000));
        assert_eq!(base_stock(world, BASE_2), tons(500));
        assert_eq!(ship_supply(world, CARGO_1), tons(3000));
        assert_eq!(ship_supply(world, CARGO_2), tons(3000));
        assert_eq!(ship_supply(world, DESTROYER), Fixed::ZERO);
        base_2_by_day.push(base_stock(world, BASE_2));
    }
    all_events.extend(events);

    // Day 3: arrival 56 hours after the start, then the first unloading.
    let (day, events) = run_day(&mut scenario);
    assert_eq!(day, 3);
    let arrivals: Vec<_> = events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::TaskForceArrived {
                hex, elapsed_hours, ..
            } => Some((event.pulse, *hex, *elapsed_hours)),
            _ => None,
        })
        .collect();
    assert_eq!(
        arrivals,
        vec![(Some(Pulse::Night), Hex::new(8, 0), tons(56))]
    );
    assert_eq!(ClockTime::from_elapsed(tons(56)).to_string(), "D3 02:00");
    assert_eq!(
        steams(&events),
        vec![(
            Pulse::Night,
            tons(80),
            Hex::new(6, 0),
            Hex::new(8, 0),
            vec![Hex::new(7, 0), Hex::new(8, 0)]
        )]
    );
    assert_eq!(fuel_burns(&events, CARGO_1), vec![(Pulse::Night, tons(8))]);
    assert_eq!(fuel_burns(&events, CARGO_2), vec![(Pulse::Night, tons(8))]);
    assert_eq!(
        fuel_burns(&events, DESTROYER),
        vec![(Pulse::Night, tons(10))]
    );
    {
        let world = &scenario.world;
        let task_force = &world.umpire().task_forces[&TASK_FORCE];
        assert_eq!(task_force.status, TaskForceStatus::Arrived);
        assert_eq!(task_force.from_hex, Hex::new(8, 0));
        assert_eq!(task_force.toward_hex, None);
        assert_eq!(task_force.progress_nm, Fixed::ZERO);
        assert_eq!(task_force.position(), Hex::new(8, 0));
        assert_eq!(ship_fuel(world, CARGO_1), tons(968));
        assert_eq!(ship_fuel(world, CARGO_2), tons(968));
        assert_eq!(ship_fuel(world, DESTROYER), tons(460));
        // Destroyer: 500 t at the start, 460 t now, 40 t burned.
        assert_eq!(tons(500) - ship_fuel(world, DESTROYER), tons(40));
    }
    base_2_by_day.push(base_stock(&scenario.world, BASE_2));
    all_events.extend(events);

    // Days 3 to 7: the unloading table. Each row is the day, the pulse, then
    // ship 1's tons unloaded and aboard after, ship 2's tons unloaded and
    // aboard after, and base 2's stock after the pulse. `None` is no event.
    type Row = (u32, Pulse, Option<(i64, i64)>, Option<(i64, i64)>, i64);
    let table: [Row; 13] = [
        (3, Pulse::Night, Some((200, 2800)), Some((80, 2920)), 780),
        (
            3,
            Pulse::Morning,
            Some((300, 2500)),
            Some((120, 2800)),
            1200,
        ),
        (
            3,
            Pulse::Afternoon,
            Some((300, 2200)),
            Some((120, 2680)),
            1620,
        ),
        (4, Pulse::Night, Some((600, 1600)), Some((240, 2440)), 2460),
        (
            4,
            Pulse::Morning,
            Some((300, 1300)),
            Some((120, 2320)),
            2880,
        ),
        (
            4,
            Pulse::Afternoon,
            Some((300, 1000)),
            Some((120, 2200)),
            3300,
        ),
        (5, Pulse::Night, Some((600, 400)), Some((240, 1960)), 4140),
        (5, Pulse::Morning, Some((300, 100)), Some((120, 1840)), 4560),
        (5, Pulse::Afternoon, Some((100, 0)), Some((120, 1720)), 4780),
        (6, Pulse::Night, None, Some((600, 1120)), 5380),
        (6, Pulse::Morning, None, Some((300, 820)), 5680),
        (6, Pulse::Afternoon, None, Some((300, 520)), 5980),
        (7, Pulse::Night, None, Some((520, 0)), 6500),
    ];

    // Day 3's events are already in `all_events`; days 4 to 7 follow.
    let mut day_events: Vec<(u32, Vec<Event>)> = vec![(3, Vec::new())];
    day_events[0].1 = all_events.iter().filter(|e| e.day == 3).cloned().collect();
    for expected_day in 4..=7 {
        let (day, events) = run_day(&mut scenario);
        assert_eq!(day, expected_day);
        // No steaming, no burning, no arrival once the task force is in.
        assert!(steams(&events).is_empty());
        assert!(events
            .iter()
            .all(|event| !matches!(event.kind, EventKind::FuelBurned { .. })));
        day_events.push((day, events.clone()));
        all_events.extend(events);
        base_2_by_day.push(base_stock(&scenario.world, BASE_2));
    }

    let mut expected_moves: Vec<(u32, Pulse, ShipId, Fixed, Fixed)> = Vec::new();
    for (day, pulse, ship_1, ship_2, _) in &table {
        if let Some((moved, aboard)) = ship_1 {
            expected_moves.push((*day, *pulse, CARGO_1, tons(*moved), tons(*aboard)));
        }
        if let Some((moved, aboard)) = ship_2 {
            expected_moves.push((*day, *pulse, CARGO_2, tons(*moved), tons(*aboard)));
        }
    }
    let mut actual_moves: Vec<(u32, Pulse, ShipId, Fixed, Fixed)> = Vec::new();
    for (day, events) in &day_events {
        for (pulse, ship, moved, aboard) in cargo_moves(events) {
            actual_moves.push((*day, pulse, ship, moved, aboard));
        }
    }
    assert_eq!(actual_moves, expected_moves);

    // Base 2's stock after each pulse of the table.
    for (day, pulse, _, _, stock) in &table {
        let events = &day_events
            .iter()
            .find(|(d, _)| d == day)
            .expect("the day was run")
            .1;
        assert_eq!(
            stock_after_pulse(events, *pulse),
            Some(tons(*stock)),
            "base 2 stock after day {day} {}",
            pulse.name()
        );
    }
    // Ship 1 has no cargo event from day 6 on.
    assert!(all_events
        .iter()
        .filter(|event| event.day >= 6)
        .all(|event| !matches!(
            event.kind,
            EventKind::CargoUnloaded { ship, .. } if ship == CARGO_1
        )));

    // Unloading completes once: day 7, night pulse, 154.4 hours in.
    let unloading = completions(&all_events, false);
    assert_eq!(
        unloading,
        vec![(Some(Pulse::Night), Fixed::from_ratio(1544, 10))]
    );
    assert_eq!(
        ClockTime::from_elapsed(unloading[0].1).to_string(),
        "D7 04:24"
    );
    let completed_on: Vec<u32> = all_events
        .iter()
        .filter(|event| matches!(event.kind, EventKind::UnloadingComplete { .. }))
        .map(|event| event.day)
        .collect();
    assert_eq!(completed_on, vec![7]);

    // The destroyer burned 40 t over the whole run, in four events.
    let destroyer_total = fuel_burns(&all_events, DESTROYER)
        .into_iter()
        .fold(Fixed::ZERO, |sum, (_, burned)| sum + burned);
    assert_eq!(destroyer_total, tons(40));

    // Day 8: nothing at all.
    let (day, events) = run_day(&mut scenario);
    assert_eq!(day, 8);
    assert!(events.is_empty());
    base_2_by_day.push(base_stock(&scenario.world, BASE_2));

    // Base 2 at the end of each day.
    let expected_stock: Vec<Fixed> = [500, 500, 1620, 3300, 4780, 5980, 6500, 6500]
        .into_iter()
        .map(tons)
        .collect();
    assert_eq!(base_2_by_day, expected_stock);

    // End state.
    let world = &scenario.world;
    assert_eq!(base_stock(world, BASE_1), tons(4000));
    assert_eq!(base_stock(world, BASE_2), tons(6500));
    for ship in [CARGO_1, CARGO_2, DESTROYER] {
        assert_eq!(ship_supply(world, ship), Fixed::ZERO);
    }

    // The scenario is over.
    assert!(scenario
        .next_day()
        .expect("no error past the end")
        .is_none());
}

#[test]
fn a_replay_reproduces_every_daily_hash() {
    let mut first = load();
    let first_hashes: Vec<u64> = first
        .run_to_end()
        .expect("first run resolves")
        .iter()
        .map(|summary| summary.state_hash)
        .collect();

    let mut second = load();
    let second_hashes: Vec<u64> = second
        .run_to_end()
        .expect("second run resolves")
        .iter()
        .map(|summary| summary.state_hash)
        .collect();

    assert_eq!(first_hashes.len(), 8);
    assert_eq!(first_hashes, second_hashes);
    assert_eq!(first.world.day_hashes(), first_hashes.as_slice());
}

#[test]
fn the_event_log_explains_an_unload() {
    let mut scenario = load();
    scenario.run_to_end().expect("run resolves");
    let world = &scenario.world;
    let log = world.events();

    // Ship 1's first unload: found by entity, without re-running.
    let unload = log
        .involving(EntityRef::Ship(CARGO_1))
        .find(|event| matches!(event.kind, EventKind::CargoUnloaded { .. }))
        .expect("ship 1 unloaded");
    assert_eq!(unload.day, 3);
    assert_eq!(unload.pulse, Some(Pulse::Night));

    // Its causes are the unload order and the arrival.
    assert_eq!(unload.causes.len(), 2);
    let arrival = log
        .involving(EntityRef::TaskForce(TASK_FORCE))
        .find(|event| matches!(event.kind, EventKind::TaskForceArrived { .. }))
        .expect("the task force arrived");
    assert!(unload.causes.contains(&Cause::Event(arrival.id)));
    let unload_order_id = unload
        .causes
        .iter()
        .find_map(|cause| match cause {
            Cause::Order(id) => Some(*id),
            Cause::Event(_) => None,
        })
        .expect("the unload order is a cause");
    let unload_order = world
        .order(unload_order_id)
        .expect("the order is on record");
    assert_eq!(
        unload_order.kind,
        OrderKind::TaskForce {
            target: TASK_FORCE,
            order: TaskForceOrder::Unload,
        }
    );
    assert_eq!(unload_order.day_issued, 2);
    assert_eq!(unload_order.side, Side::Allied);
    assert_eq!(unload_order.origin.issuer, Issuer::Player);

    // The trace leads back through the arrival to the movement, and from
    // there to the route and speed orders.
    let trace = log.trace(unload.id);
    assert!(trace.contains(&Cause::Event(arrival.id)));
    let Cause::Event(steamed_id) = arrival.causes[0] else {
        panic!("the arrival should be caused by an event");
    };
    assert!(matches!(
        log.get(steamed_id).expect("in the log").kind,
        EventKind::TaskForceSteamed { .. }
    ));
    assert!(trace.contains(&Cause::Event(steamed_id)));
    let traced_orders: Vec<&TaskForceOrder> = trace
        .iter()
        .filter_map(|cause| match cause {
            Cause::Order(id) => match &world.order(*id).expect("on record").kind {
                OrderKind::TaskForce { order, .. } => Some(order),
            },
            Cause::Event(_) => None,
        })
        .collect();
    assert!(traced_orders
        .iter()
        .any(|order| matches!(order, TaskForceOrder::SetRoute(_))));
    assert!(traced_orders
        .iter()
        .any(|order| matches!(order, TaskForceOrder::SetSpeed(_))));
    assert!(traced_orders
        .iter()
        .any(|order| matches!(order, TaskForceOrder::Unload)));

    // The unload records the factors behind its figure: 4 hours on the berth
    // at 50 t per hour.
    let breakdown = unload
        .breakdown
        .as_ref()
        .expect("breakdowns are on by default");
    let factor = |name: &str| {
        breakdown
            .factors
            .iter()
            .find(|factor| factor.name == name)
            .map(|factor| factor.value)
    };
    assert_eq!(factor("hours_worked"), Some(tons(4)));
    assert_eq!(factor("on_berth"), Some(Fixed::ONE));
    assert_eq!(factor("rate_t_per_h"), Some(tons(50)));
    assert_eq!(breakdown.result, tons(200));
}

#[test]
fn each_side_sees_only_its_own_units() {
    let mut scenario = load();
    scenario.run_to_end().expect("run resolves");

    let allied = scenario.world.side_view(Side::Allied);
    assert_eq!(allied.ships().count(), 3);
    assert_eq!(allied.task_forces().count(), 1);
    assert_eq!(allied.bases().count(), 2);
    assert_eq!(allied.contacts().count(), 0);
    let stock = |id: BaseId| {
        allied
            .bases()
            .find(|base| base.id == id)
            .expect("the base is visible")
            .supply_t
    };
    assert_eq!(stock(BASE_1), tons(4000));
    assert_eq!(stock(BASE_2), tons(6500));
    assert!(allied.ships().all(|ship| ship.supply_t == Fixed::ZERO));

    let japanese = scenario.world.side_view(Side::Japanese);
    assert_eq!(japanese.ships().count(), 0);
    assert_eq!(japanese.task_forces().count(), 0);
    assert_eq!(japanese.bases().count(), 0);
    assert_eq!(japanese.contacts().count(), 0);
}

#[test]
fn supply_is_conserved_every_day() {
    let mut scenario = load();
    let mut days = 0;
    while scenario.next_day().expect("day resolves").is_some() {
        days += 1;
        let truth = scenario.world.umpire();
        let ships = truth
            .ships
            .values()
            .fold(Fixed::ZERO, |sum, ship| sum + ship.supply_t);
        let bases = truth
            .bases
            .values()
            .fold(Fixed::ZERO, |sum, base| sum + base.supply_t);
        assert_eq!(ships + bases, tons(TOTAL_SUPPLY_T), "after day {days}");
    }
    assert_eq!(days, 8);
}

/// Copies the scenario and its data into a fresh folder under the build
/// directory, with the destination port (8,0) at `port_size`, and loads it.
/// Nothing in the repository's data folder is touched.
fn variant(tag: &str, port_size: u8) -> Scenario {
    let data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data");
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("supply_run_{tag}_{port_size}"));
    for folder in ["scenarios", "map", "reference"] {
        fs::create_dir_all(root.join(folder)).expect("make the variant folder");
    }
    let copy = |from: &str, to: &str| {
        fs::copy(data.join(from), root.join(to)).expect("copy a data file");
    };
    copy(
        "scenarios/02_supply_run.toml",
        "scenarios/02_supply_run.toml",
    );
    copy("reference/ship_classes.csv", "reference/ship_classes.csv");
    copy("reference/port_sizes.csv", "reference/port_sizes.csv");

    let map_text = fs::read_to_string(data.join("map/placeholder_supply_run.toml"))
        .expect("read the map file");
    let original = "q = 8\nr = 0\nkind = \"coast\"\nport_size = 1\n";
    assert_eq!(
        map_text.matches(original).count(),
        1,
        "the (8,0) hex block appears exactly once"
    );
    let changed = original.replace("port_size = 1", &format!("port_size = {port_size}"));
    fs::write(
        root.join("map/placeholder_supply_run.toml"),
        map_text.replace(original, &changed),
    )
    .expect("write the variant map");

    Scenario::load(&root.join("scenarios/02_supply_run.toml")).expect("the variant loads")
}

/// The elapsed hour at which unloading completed, if it did.
fn unloading_completed_at(scenario: &Scenario) -> Option<Fixed> {
    let hours: Vec<Fixed> = scenario
        .world
        .events()
        .iter()
        .filter_map(|event| match event.kind {
            EventKind::UnloadingComplete { elapsed_hours, .. } => Some(elapsed_hours),
            _ => None,
        })
        .collect();
    assert!(hours.len() <= 1, "unloading completes at most once");
    hours.first().copied()
}

#[test]
fn a_larger_destination_port_never_finishes_later() {
    let mut finished: Vec<(u8, Fixed)> = Vec::new();
    for size in 0..=3u8 {
        let mut scenario = variant("monotone", size);
        scenario.days = 12;
        scenario.run_to_end().expect("the variant runs");
        let hours = unloading_completed_at(&scenario)
            .unwrap_or_else(|| panic!("size {size} should finish unloading"));
        finished.push((size, hours));
    }
    assert_eq!(finished[0], (0, tons(206)));
    assert_eq!(finished[1], (1, Fixed::from_ratio(1544, 10)));
    for pair in finished.windows(2) {
        assert!(
            pair[1].1 <= pair[0].1,
            "size {} finished at {} after size {} at {}",
            pair[1].0,
            pair[1].1,
            pair[0].0,
            pair[0].1
        );
    }
}

fn assert_limits(world: &World, context: &str) {
    let truth = world.umpire();
    for ship in truth.ships.values() {
        let capacity = world.ref_data().ship_classes[&ship.class].cargo_capacity_t;
        assert!(
            ship.supply_t >= Fixed::ZERO && ship.supply_t <= capacity,
            "{context}: {} holds {} t, capacity {capacity} t",
            ship.id,
            ship.supply_t
        );
    }
    for base in truth.bases.values() {
        assert!(
            base.supply_t >= Fixed::ZERO,
            "{context}: {} holds {} t",
            base.id,
            base.supply_t
        );
    }
}

#[test]
fn no_ship_over_capacity_and_no_negative_supply() {
    let mut scenario = load();
    while let Some(summary) = scenario.next_day().expect("day resolves") {
        assert_limits(&scenario.world, &format!("scenario, day {}", summary.day));
    }

    let mut small_port = variant("limits", 0);
    small_port.days = 12;
    while let Some(summary) = small_port.next_day().expect("day resolves") {
        assert_limits(&small_port.world, &format!("size 0, day {}", summary.day));
    }
}
