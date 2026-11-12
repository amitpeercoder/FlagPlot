//! Scenario 1: one task force steams from Noumea to Espiritu Santo.
//!
//! Proves that movement, fuel burn and arrival time match a hand calculation,
//! that a replay reproduces every daily hash, and that the event log can say
//! why the task force arrived.
//!
//! The data is placeholder data: see `data/scenarios/01_transit.toml`.
//!
//! # Hand calculation
//!
//! Redone by the architect on Nov 12, 2026 for decision 0058 (ships are
//! charged straight-line miles). The tests below assert these numbers. If
//! the code disagrees, the code or the rule is wrong: do not change a number
//! here to make a test pass.
//!
//! Inputs, from the data files:
//!
//! - Route: (0,0) to (8,3). The path runs along q to (8,0) and then up r:
//!   (8,1), (8,2), (8,3). That is 11 steps. No land is in the way, so every
//!   step is charged the miles it brings the force closer to (8,3) in a
//!   straight line (decision 0058, 1).
//! - Both ship classes cruise at 16 kn, so the task force steams at 16 kn.
//! - Cruiser: 2000 t lasts 8000 nm at cruise, so it burns 0.25 t per nm.
//! - Destroyer: 500 t lasts 4000 nm at cruise, so it burns 0.125 t per nm.
//! - A day is a 12-hour night pulse, then two 6-hour pulses. It starts at
//!   18:00.
//!
//! The straight distance to (8,3), 40 times the root of dq^2 + dr^2 + dq x
//! dr to the nearest mile, from each centre on the path, and so the charge
//! of each step:
//!
//! | Centre | dq, dr | dq^2 + dr^2 + dq x dr | Miles to (8,3) | Step to it is charged | Miles steamed |
//! | ------ | ------ | --------------------- | -------------- | --------------------- | ------------- |
//! | (0,0)  | 8, 3   | 97                    | 394            |                       | 0             |
//! | (1,0)  | 7, 3   | 79                    | 356            | 38                    | 38            |
//! | (2,0)  | 6, 3   | 63                    | 317            | 39                    | 77            |
//! | (3,0)  | 5, 3   | 49                    | 280            | 37                    | 114           |
//! | (4,0)  | 4, 3   | 37                    | 243            | 37                    | 151           |
//! | (5,0)  | 3, 3   | 27                    | 208            | 35                    | 186           |
//! | (6,0)  | 2, 3   | 19                    | 174            | 34                    | 220           |
//! | (7,0)  | 1, 3   | 13                    | 144            | 30                    | 250           |
//! | (8,0)  | 0, 3   | 9                     | 120            | 24                    | 274           |
//! | (8,1)  | 0, 2   | 4                     | 80             | 40                    | 314           |
//! | (8,2)  | 0, 1   | 1                     | 40             | 40                    | 354           |
//! | (8,3)  | 0, 0   | 0                     | 0              | 40                    | 394           |
//!
//! The transit is 394 nm, the straight distance, where it was 440.
//!
//! A task force is in the hex whose centre is nearest, so it enters the
//! next hex when half the step's charge is steamed. The miles steamed when
//! it enters each hex: (1,0) at 19, (2,0) at 57.5, (3,0) at 95.5, (4,0) at
//! 132.5, (5,0) at 168.5, (6,0) at 203, (7,0) at 235, (8,0) at 262, (8,1)
//! at 294, (8,2) at 334, (8,3) at 374.
//!
//! Day 1:
//!
//! | Pulse     | Hours | Distance | Total  | Centre left, miles on | Position |
//! | --------- | ----- | -------- | ------ | --------------------- | -------- |
//! | Night     | 12    | 192 nm   | 192 nm | (5,0) plus 6 of 34    | (5,0)    |
//! | Morning   | 6     | 96 nm    | 288 nm | (8,0) plus 14 of 40   | (8,0)    |
//! | Afternoon | 6     | 96 nm    | 384 nm | (8,2) plus 30 of 40   | (8,3)    |
//!
//! The hexes it enters, by pulse:
//!
//! - Night: (1,0), (2,0), (3,0), (4,0), (5,0). The last at 168.5 nm.
//! - Morning: (6,0) at 203 nm, (7,0) at 235 nm, (8,0) at 262 nm.
//! - Afternoon: (8,1) at 294 nm, (8,2) at 334 nm, (8,3) at 374 nm.
//!
//! At the end of day 1 the task force has left (8,2), is steaming toward
//! (8,3) and is 30 nm along a step of 40, so its position is (8,3). Fuel
//! burned: cruiser 384 x 0.25 = 96 t, destroyer 384 x 0.125 = 48 t.
//!
//! Day 2: 394 - 384 = 10 nm remain. At 16 kn that takes 0.625 hours, so the
//! task force arrives 24.625 hours after the start, at 18:37 and a half on
//! day 2, during the night pulse. It is already in (8,3) and enters no hex.
//! Fuel burned on the leg: cruiser 2.5 t, destroyer 1.25 t.
//!
//! Totals for the transit: cruiser 98.5 t, destroyer 49.25 t.
//!
//! Day 3: no new orders. The task force has arrived, so nothing moves and no
//! fuel is burned.

use std::path::PathBuf;

use sim_core::events::{Cause, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{EntityRef, ShipId, TaskForceId};
use sim_core::orders::{Issuer, OrderKind, TaskForceOrder};
use sim_core::state::{ShipLocation, Side, TaskForceStatus};
use sim_core::time::{ClockTime, Pulse};
use sim_data::Scenario;

const TASK_FORCE: TaskForceId = TaskForceId::new(1);
const CRUISER: ShipId = ShipId::new(1);
const DESTROYER: ShipId = ShipId::new(2);

fn load() -> Scenario {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/01_transit.toml");
    Scenario::load(&path).expect("scenario 1 loads")
}

fn tons(value: i64) -> Fixed {
    Fixed::from_int(value)
}

/// One movement event: pulse, distance, position at the start, position at
/// the end, and the hexes entered.
type Steam = (Pulse, Fixed, Hex, Hex, Vec<Hex>);

fn steamed(scenario: &Scenario, first_event: usize) -> Vec<Steam> {
    scenario
        .world
        .events()
        .from_index(first_event)
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

#[test]
fn the_transit_matches_the_hand_calculation() {
    let mut scenario = load();
    assert!(scenario.placeholder);
    assert_eq!(scenario.days, 3);

    // Before day 1: in port, full tanks.
    {
        let truth = scenario.world.umpire();
        let task_force = &truth.task_forces[&TASK_FORCE];
        assert_eq!(task_force.from_hex, Hex::new(0, 0));
        assert_eq!(task_force.toward_hex, None);
        assert_eq!(task_force.progress_nm, Fixed::ZERO);
        assert_eq!(task_force.position(), Hex::new(0, 0));
        assert_eq!(task_force.status, TaskForceStatus::Idle);
        assert_eq!(truth.ships[&CRUISER].fuel_t, tons(2000));
        assert_eq!(truth.ships[&DESTROYER].fuel_t, tons(500));
    }

    // Day 1: 384 nm, which is nine hexes and 24 nm.
    let day1 = scenario
        .next_day()
        .expect("day 1 resolves")
        .expect("day 1 exists");
    assert_eq!(day1.day, 1);
    {
        let truth = scenario.world.umpire();
        let task_force = &truth.task_forces[&TASK_FORCE];
        assert_eq!(task_force.status, TaskForceStatus::Underway);
        // Left (8,1), steaming toward (8,2), 24 nm along: it is in (8,2).
        assert_eq!(task_force.from_hex, Hex::new(8, 1));
        assert_eq!(task_force.toward_hex, Some(Hex::new(8, 2)));
        assert_eq!(task_force.progress_nm, Fixed::from_int(24));
        assert_eq!(task_force.position(), Hex::new(8, 2));
        assert_eq!(truth.ships[&CRUISER].fuel_t, tons(2000 - 96));
        assert_eq!(truth.ships[&DESTROYER].fuel_t, tons(500 - 48));
    }

    // Each pulse of day 1 covered the distance the table says, moved the
    // position as the table says and entered the hexes the notes list.
    let hex = Hex::new;
    assert_eq!(
        steamed(&scenario, day1.first_event),
        vec![
            (
                Pulse::Night,
                Fixed::from_int(192),
                hex(0, 0),
                hex(5, 0),
                vec![hex(1, 0), hex(2, 0), hex(3, 0), hex(4, 0), hex(5, 0)]
            ),
            (
                Pulse::Morning,
                Fixed::from_int(96),
                hex(5, 0),
                hex(7, 0),
                vec![hex(6, 0), hex(7, 0)]
            ),
            (
                Pulse::Afternoon,
                Fixed::from_int(96),
                hex(7, 0),
                hex(8, 2),
                vec![hex(8, 0), hex(8, 1), hex(8, 2)]
            ),
        ]
    );

    // Day 2: the last 56 nm, arriving 27.5 hours after the start.
    let day2 = scenario
        .next_day()
        .expect("day 2 resolves")
        .expect("day 2 exists");
    {
        let truth = scenario.world.umpire();
        let task_force = &truth.task_forces[&TASK_FORCE];
        assert_eq!(task_force.status, TaskForceStatus::Arrived);
        assert_eq!(task_force.from_hex, Hex::new(8, 3));
        assert_eq!(task_force.toward_hex, None);
        assert_eq!(task_force.progress_nm, Fixed::ZERO);
        assert_eq!(task_force.position(), Hex::new(8, 3));
        assert_eq!(truth.ships[&CRUISER].fuel_t, tons(2000 - 110));
        assert_eq!(truth.ships[&DESTROYER].fuel_t, tons(500 - 55));
    }

    // It was already in (8,2) at the start, so finishing that leg enters
    // nothing; (8,3) is entered at 420 nm.
    assert_eq!(
        steamed(&scenario, day2.first_event),
        vec![(
            Pulse::Night,
            Fixed::from_int(56),
            hex(8, 2),
            hex(8, 3),
            vec![hex(8, 3)]
        )]
    );

    let arrivals: Vec<_> = scenario
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
        .collect();
    assert_eq!(arrivals.len(), 1);
    let (pulse, hex, elapsed_hours) = arrivals[0];
    assert_eq!(pulse, Some(Pulse::Night));
    assert_eq!(hex, Hex::new(8, 3));
    assert_eq!(elapsed_hours, Fixed::from_ratio(55, 2));
    assert_eq!(
        ClockTime::from_elapsed(elapsed_hours).to_string(),
        "D2 21:30"
    );

    // Day 3: no orders, nothing moves, nothing burns, nothing is logged.
    let day3 = scenario
        .next_day()
        .expect("day 3 resolves")
        .expect("day 3 exists");
    assert_eq!(day3.event_count, 0);
    {
        let truth = scenario.world.umpire();
        let task_force = &truth.task_forces[&TASK_FORCE];
        assert_eq!(task_force.from_hex, Hex::new(8, 3));
        assert_eq!(task_force.toward_hex, None);
        assert_eq!(task_force.progress_nm, Fixed::ZERO);
        assert_eq!(task_force.position(), Hex::new(8, 3));
        assert_eq!(truth.ships[&CRUISER].fuel_t, tons(1890));
        assert_eq!(truth.ships[&DESTROYER].fuel_t, tons(445));
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

    assert_eq!(first_hashes.len(), 3);
    assert_eq!(first_hashes, second_hashes);
    assert_eq!(first.world.day_hashes(), first_hashes.as_slice());

    // The state changes on days 1 and 2, so their hashes differ.
    assert_ne!(first_hashes[0], first_hashes[1]);
    // Day 3 changes nothing but the day counter, which is part of the state.
    assert_ne!(first_hashes[1], first_hashes[2]);
}

#[test]
fn the_event_log_explains_the_arrival() {
    let mut scenario = load();
    scenario.run_to_end().expect("run resolves");
    let world = &scenario.world;
    let log = world.events();

    // What happened to the task force: found by entity, without re-running.
    let arrival = log
        .involving(EntityRef::TaskForce(TASK_FORCE))
        .find(|event| matches!(event.kind, EventKind::TaskForceArrived { .. }))
        .expect("the task force arrived");

    // Why: the arrival was caused by a movement event...
    assert_eq!(arrival.causes.len(), 1);
    let Cause::Event(steamed_id) = arrival.causes[0] else {
        panic!("the arrival should be caused by an event");
    };
    let steamed = log.get(steamed_id).expect("the cause is in the log");
    assert!(matches!(steamed.kind, EventKind::TaskForceSteamed { .. }));

    // ...which was caused by the two standing orders from day 1.
    let orders: Vec<_> = log
        .trace(arrival.id)
        .into_iter()
        .filter_map(|cause| match cause {
            Cause::Order(id) => Some(id),
            Cause::Event(_) => None,
        })
        .collect();
    assert_eq!(orders.len(), 2);

    let route_order = world.order(orders[0]).expect("the order is on record");
    assert_eq!(route_order.side, Side::Allied);
    assert_eq!(route_order.day_issued, 1);
    assert_eq!(route_order.origin.issuer, Issuer::Player);
    assert_eq!(route_order.origin.intent, None);
    assert_eq!(
        route_order.kind,
        OrderKind::TaskForce {
            target: TASK_FORCE,
            order: TaskForceOrder::SetRoute(vec![Hex::new(8, 3)]),
        }
    );
    let speed_order = world.order(orders[1]).expect("the order is on record");
    assert!(matches!(
        speed_order.kind,
        OrderKind::TaskForce {
            order: TaskForceOrder::SetSpeed(_),
            ..
        }
    ));

    // Every fuel figure records the factors that produced it.
    let burn = log
        .involving(EntityRef::Ship(CRUISER))
        .find(|event| matches!(event.kind, EventKind::FuelBurned { .. }))
        .expect("the cruiser burned fuel");
    let breakdown = burn
        .breakdown
        .as_ref()
        .expect("breakdowns are on by default");
    assert_eq!(breakdown.result, tons(48));
    let factor = |name: &str| {
        breakdown
            .factors
            .iter()
            .find(|factor| factor.name == name)
            .map(|factor| factor.value)
    };
    assert_eq!(factor("distance_nm"), Some(Fixed::from_int(192)));
    assert_eq!(factor("speed_multiplier"), Some(Fixed::ONE));
}

#[test]
fn each_side_sees_only_its_own_units() {
    let mut scenario = load();
    scenario.run_to_end().expect("run resolves");

    let allied = scenario.world.side_view(Side::Allied);
    assert_eq!(allied.ships().count(), 2);
    assert_eq!(allied.task_forces().count(), 1);
    assert_eq!(allied.bases().count(), 2);
    assert_eq!(allied.contacts().count(), 0);
    assert!(allied
        .ships()
        .all(|ship| ship.location == ShipLocation::TaskForce(TASK_FORCE)));

    // The Japanese side has no units here and has sighted nothing, so its
    // view is empty. It has no way to ask for the Allied task force.
    let japanese = scenario.world.side_view(Side::Japanese);
    assert_eq!(japanese.ships().count(), 0);
    assert_eq!(japanese.task_forces().count(), 0);
    assert_eq!(japanese.bases().count(), 0);
    assert_eq!(japanese.contacts().count(), 0);
}
