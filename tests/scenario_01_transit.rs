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

mod common;

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
    common::load(&path).expect("scenario 1 loads")
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

/// The hand calculation's table, row by row: the centre, the miles to (8,3)
/// from it, the charge of the step to it, and the miles steamed on arrival
/// at it (decision 0058).
const TABLE: [((i32, i32), u32, u32, u32); 12] = [
    ((0, 0), 394, 0, 0),
    ((1, 0), 356, 38, 38),
    ((2, 0), 317, 39, 77),
    ((3, 0), 280, 37, 114),
    ((4, 0), 243, 37, 151),
    ((5, 0), 208, 35, 186),
    ((6, 0), 174, 34, 220),
    ((7, 0), 144, 30, 250),
    ((8, 0), 120, 24, 274),
    ((8, 1), 80, 40, 314),
    ((8, 2), 40, 40, 354),
    ((8, 3), 0, 40, 394),
];

#[test]
fn the_map_charges_each_step_as_the_hand_calculation_says() {
    let scenario = load();
    let map = scenario.world.map();
    let target = Hex::new(8, 3);
    let path = map
        .sea_path(Hex::new(0, 0), target)
        .expect("a sea path exists");
    // 11 steps, along q to (8,0) and then up r: no land in the way.
    assert_eq!(path.len(), 11);
    assert_eq!(Hex::new(0, 0).distance(target), 11);
    let expected: Vec<Hex> = TABLE[1..]
        .iter()
        .map(|row| Hex::new(row.0 .0, row.0 .1))
        .collect();
    assert_eq!(path, expected);

    let field = map.sea_distances_to(target);
    let mut here = Hex::new(0, 0);
    let mut steamed = 0;
    for (row, next) in TABLE.iter().zip(TABLE.iter().skip(1)) {
        let (centre, to_go, _, _) = *row;
        let (next_centre, next_to_go, charge, total) = *next;
        let (centre, next_centre) = (
            Hex::new(centre.0, centre.1),
            Hex::new(next_centre.0, next_centre.1),
        );
        assert_eq!(here, centre);
        // Miles to (8,3) from each centre.
        assert_eq!(centre.air_distance_nm(target), to_go);
        assert_eq!(next_centre.air_distance_nm(target), next_to_go);
        // The charge of the step is the miles it brings the force closer.
        assert_eq!(to_go - next_to_go, charge);
        assert_eq!(
            map.step_charge_nm(&field, centre, next_centre, target),
            charge
        );
        steamed += charge;
        assert_eq!(steamed, total);
        here = next_centre;
    }
    // The transit is the straight distance, where it was 440.
    assert_eq!(steamed, 394);
    assert_eq!(map.sea_charged_nm(Hex::new(0, 0), target), Some(394));
    assert_eq!(
        map.sea_charged_nm_in(&field, Hex::new(0, 0), target),
        Some(394)
    );
    // The charged miles from every centre on the way are the miles to go.
    for (centre, to_go, _, _) in TABLE {
        assert_eq!(
            map.sea_charged_nm(Hex::new(centre.0, centre.1), target),
            Some(to_go)
        );
    }
}

#[test]
fn the_force_enters_each_hex_at_half_the_charge_of_the_step() {
    // Miles steamed when each hex is entered, doubled so that they are whole:
    // the miles steamed on reaching the centre before it, plus half the
    // step's charge. 19, 57.5, 95.5, 132.5, 168.5, 203, 235, 262, 294, 334
    // and 374.
    let entered_at_doubled = [38, 115, 191, 265, 337, 406, 470, 524, 588, 668, 748];
    for (index, entry) in entered_at_doubled.iter().enumerate() {
        let before = TABLE[index].3;
        let charge = TABLE[index + 1].2;
        assert_eq!(2 * before + charge, *entry, "hex {}", index + 1);
    }
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
        assert_eq!(task_force.leg_nm, Fixed::from_int(40));
        assert_eq!(task_force.position(), Hex::new(0, 0));
        assert_eq!(task_force.status, TaskForceStatus::Idle);
        assert_eq!(truth.ships[&CRUISER].fuel_t, tons(2000));
        assert_eq!(truth.ships[&DESTROYER].fuel_t, tons(500));
    }

    // Day 1: 384 nm of 394.
    let day1 = scenario
        .next_day()
        .expect("day 1 resolves")
        .expect("day 1 exists");
    assert_eq!(day1.day, 1);
    {
        let truth = scenario.world.umpire();
        let task_force = &truth.task_forces[&TASK_FORCE];
        assert_eq!(task_force.status, TaskForceStatus::Underway);
        // Left (8,2), steaming toward (8,3), 30 nm along a step of 40: it
        // is in (8,3).
        assert_eq!(task_force.from_hex, Hex::new(8, 2));
        assert_eq!(task_force.toward_hex, Some(Hex::new(8, 3)));
        assert_eq!(task_force.progress_nm, Fixed::from_int(30));
        assert_eq!(task_force.leg_nm, Fixed::from_int(40));
        assert_eq!(task_force.position(), Hex::new(8, 3));
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
                hex(8, 0),
                vec![hex(6, 0), hex(7, 0), hex(8, 0)]
            ),
            (
                Pulse::Afternoon,
                Fixed::from_int(96),
                hex(8, 0),
                hex(8, 3),
                vec![hex(8, 1), hex(8, 2), hex(8, 3)]
            ),
        ]
    );

    // The night pulse began six steps: the charges it records are the
    // table's first six.
    let night = scenario
        .world
        .events()
        .from_index(day1.first_event)
        .iter()
        .find(|event| matches!(event.kind, EventKind::TaskForceSteamed { .. }))
        .expect("the night steaming is logged");
    let charges: Vec<Fixed> = night
        .breakdown
        .as_ref()
        .expect("breakdowns are on by default")
        .factors
        .iter()
        .filter(|factor| factor.name == "step_charge_nm")
        .map(|factor| factor.value)
        .collect();
    assert_eq!(
        charges,
        [38, 39, 37, 37, 35, 34].map(Fixed::from_int).to_vec()
    );

    // Day 2: the last 10 nm, arriving 24.625 hours after the start.
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
        assert_eq!(task_force.leg_nm, Fixed::from_int(40));
        assert_eq!(task_force.position(), Hex::new(8, 3));
        assert_eq!(
            truth.ships[&CRUISER].fuel_t,
            Fixed::from_ratio(2000 * 2 - 197, 2)
        );
        assert_eq!(
            truth.ships[&DESTROYER].fuel_t,
            Fixed::from_ratio(500 * 4 - 197, 4)
        );
    }

    // It was already in (8,3) at the start, so finishing the step enters
    // nothing.
    assert_eq!(
        steamed(&scenario, day2.first_event),
        vec![(
            Pulse::Night,
            Fixed::from_int(10),
            hex(8, 3),
            hex(8, 3),
            vec![]
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
    // 24 hours into the game, the start of day 2, and 10 nm at 16 kn is
    // 0.625 hours: 24.625 hours after the start. The clock rounds to the
    // nearest minute, half a minute up, so it reads 18:38 where the hand
    // calculation has 18:37 and a half.
    assert_eq!(elapsed_hours, Fixed::from_ratio(197, 8));
    assert_eq!(
        ClockTime::from_elapsed(elapsed_hours).to_string(),
        "D2 18:38"
    );

    // Day 3: no orders, nothing moves, nothing burns, and nothing is logged
    // but the two bases' counts at the end of the day (decision 0067).
    let day3 = scenario
        .next_day()
        .expect("day 3 resolves")
        .expect("day 3 exists");
    assert_eq!(day3.event_count, 2);
    assert!(scenario
        .world
        .events()
        .from_index(day3.first_event)
        .iter()
        .all(|event| matches!(event.kind, EventKind::BaseAtDayEnd { .. })));
    {
        let truth = scenario.world.umpire();
        let task_force = &truth.task_forces[&TASK_FORCE];
        assert_eq!(task_force.from_hex, Hex::new(8, 3));
        assert_eq!(task_force.toward_hex, None);
        assert_eq!(task_force.progress_nm, Fixed::ZERO);
        assert_eq!(task_force.leg_nm, Fixed::from_int(40));
        assert_eq!(task_force.position(), Hex::new(8, 3));
        // Totals for the transit: cruiser 98.5 t, destroyer 49.25 t.
        assert_eq!(
            truth.ships[&CRUISER].fuel_t,
            Fixed::from_ratio(2000 * 2 - 197, 2)
        );
        assert_eq!(
            truth.ships[&DESTROYER].fuel_t,
            Fixed::from_ratio(500 * 4 - 197, 4)
        );
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

// ---- Property tests on the real map (decision 0058) ----
//
// They state what must always hold of what a force is charged, whatever the
// route: the charged miles of a clear way are the straight distance, a detour
// costs more and never more than a hex a step, and no step is charged less
// than 1 or more than 40 miles.

fn real_map() -> sim_core::map::Map {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/map/south_pacific.csv");
    sim_data::load_map(&path).expect("the map loads")
}

/// A small deterministic generator, so the cases never depend on a crate.
struct Lcg(u64);

impl Lcg {
    fn below(&mut self, n: usize) -> usize {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        usize::try_from((self.0 >> 33) % n as u64).expect("fits")
    }
}

#[test]
fn on_the_real_map_a_clear_way_is_charged_the_air_distance_and_a_detour_more() {
    let map = real_map();
    let sea: Vec<Hex> = map
        .iter()
        .filter(|(hex, _)| map.is_navigable(**hex))
        .map(|(hex, _)| *hex)
        .collect();
    let mut rng = Lcg(58);
    let (mut clear, mut detours, mut pairs) = (0, 0, 0);
    // 60 targets, each with the field worked out once, and 8 sources each.
    for _ in 0..60 {
        let target = sea[rng.below(sea.len())];
        let field = map.sea_distances_to(target);
        for _ in 0..8 {
            let from = sea[rng.below(sea.len())];
            let Some(&steps) = field.get(&from) else {
                continue;
            };
            pairs += 1;
            let path = map.sea_path(from, target).expect("a route exists");
            assert_eq!(path.len() as u32, steps);
            let mut here = from;
            let mut sum = 0;
            for next in &path {
                let charge = map.step_charge_nm(&field, here, *next, target);
                // A step is charged from 1 to 40 miles.
                assert!((1..=40).contains(&charge), "{here} to {next} for {target}");
                sum += charge;
                here = *next;
            }
            // The charged miles of the path are the sum of its steps.
            assert_eq!(map.sea_charged_nm_in(&field, from, target), Some(sum));
            let air = from.air_distance_nm(target);
            if steps == from.distance(target) {
                // As many steps as hexes between them: the straight line.
                clear += 1;
                assert_eq!(sum, air, "{from} to {target}");
            } else {
                // Land forced a detour: more than the straight line, and at
                // most a hex a step.
                detours += 1;
                assert!(sum > air, "{from} to {target}: {sum} is not above {air}");
                assert!(sum <= 40 * steps, "{from} to {target}");
            }
        }
    }
    assert!(pairs >= 300, "only {pairs} pairs had a route");
    assert!(
        clear >= 50 && detours >= 20,
        "{clear} clear, {detours} detours"
    );
}

#[test]
fn on_the_real_map_a_step_along_a_hex_axis_is_charged_forty() {
    let map = real_map();
    let mut tested = 0;
    for (hex, _) in map
        .iter()
        .filter(|(hex, _)| map.is_navigable(**hex))
        .step_by(131)
    {
        for (dq, dr) in Hex::DIRECTIONS {
            for steps in 1..=6 {
                let target = Hex::new(hex.q + dq * steps, hex.r + dr * steps);
                let field = map.sea_distances_to(target);
                // Only a clear way along the axis counts.
                if field.get(hex) != Some(&(steps as u32)) {
                    continue;
                }
                let path = map.sea_path(*hex, target).expect("a route exists");
                let mut here = *hex;
                for next in &path {
                    assert_eq!(map.step_charge_nm(&field, here, *next, target), 40);
                    here = *next;
                }
                assert_eq!(map.sea_charged_nm(*hex, target), Some(40 * steps as u32));
                tested += 1;
            }
        }
    }
    assert!(tested >= 100, "only {tested} axis routes were clear");
}
