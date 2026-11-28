//! Scenario 78: scenario 1's transit, with an order given inside a day
//! (decision 0156, step 2).
//!
//! The scenario is scenario 1's setup with `orders_inside_a_day = true` and
//! one more order: at 0600 of day 1 the force slows to 8 knots. It is case A
//! below. Case B (the same order at 1200) and case C (the key on and no
//! order inside a day) are built here from the file by the API. The tests run
//! on the frozen figures of step 1, as scenario 1's do.
//!
//! The saved game's file itself, written by `sim-cli run --save` and read by
//! `sim-cli replay`, is tested in `crates/sim-cli/tests/saved_games.rs`
//! (`scenario_78_saves_and_replays_with_the_pulse_of_its_order`), since only
//! that crate's tests can run the runner; here the replay is the scenario
//! rebuilt from the orders a saved game keeps, with their pulses.
//!
//! # The hand calculation
//!
//! For `tests/scenario_78_orders_inside_a_day.rs`, whose doc comment takes this section word for word. Written by the architect on Nov 18, 2026, before any code. If the code disagrees, the code or the rule is wrong: no number here is changed to make a test pass.
//!
//! **The setup** is scenario 1's, with the key on: one Allied task force, a cruiser (2,000 t of fuel, 0.25 t a mile at cruise) and a destroyer (500 t, 0.125 t a mile), ordered at dusk of day 1 from (0,0) to (8,3) at cruise, 16 kn. Scenario 1's hand calculation gives the path and the miles steamed when the force enters each hex: (1,0) at 19, (2,0) at 57.5, (3,0) at 95.5, (4,0) at 132.5, (5,0) at 168.5, (6,0) at 203, (7,0) at 235, (8,0) at 262, (8,1) at 294, (8,2) at 334, (8,3) at 374; the transit is 394 miles. A ship below its cruise speed burns its cruise rate a mile (chapter 2, Fuel burn), so fuel is miles times the rate at either speed.
//!
//! **Case A: at 0600 of day 1 the commander orders `set_speed` of 8 kn.**
//!
//! | Pulse | Hours | Speed | Miles | Total | Hexes entered | Position at its end |
//! | --- | --- | --- | --- | --- | --- | --- |
//! | Night | 12 | 16 kn | 192 | 192 | (1,0) to (5,0) | (5,0) |
//! | Morning | 6 | 8 kn | 48 | 240 | (6,0) at 203, (7,0) at 235 | (7,0) |
//! | Afternoon | 6 | 8 kn | 48 | 288 | (8,0) at 262 | (8,0) |
//!
//! - The order is accepted in the morning pulse of day 1; no event of the night names it.
//! - End of day 1: 288 miles steamed, the force in (8,0). Fuel burned: cruiser 288 x 0.25 = 72 t, 1,928 t left; destroyer 288 x 0.125 = 36 t, 464 t left.
//! - Day 2: 394 - 288 = 106 miles remain, 13.25 hours at 8 kn. The night steams 96, to 384, entering (8,1), (8,2) and (8,3). The last 10 miles take 1.25 hours of the morning: the force arrives 37.25 hours after the start, at 07:15 on day 2. Fuel on day 2: cruiser 26.5 t, destroyer 13.25 t.
//! - Totals: cruiser 98.5 t, destroyer 49.25 t, as scenario 1's, since the miles are the same.
//!
//! **Case B: the same order at 1200 of day 1.**
//!
//! | Pulse | Hours | Speed | Miles | Total | Hexes entered | Position at its end |
//! | --- | --- | --- | --- | --- | --- | --- |
//! | Night | 12 | 16 kn | 192 | 192 | (1,0) to (5,0) | (5,0) |
//! | Morning | 6 | 16 kn | 96 | 288 | (6,0), (7,0), (8,0) | (8,0) |
//! | Afternoon | 6 | 8 kn | 48 | 336 | (8,1) at 294, (8,2) at 334 | (8,2) |
//!
//! - The order is accepted in the afternoon pulse of day 1.
//! - End of day 1: 336 miles steamed, the force in (8,2). Fuel burned: cruiser 84 t, 1,916 t left; destroyer 42 t, 458 t left.
//! - Day 2: 58 miles remain, 7.25 hours at 8 kn. The force enters (8,3) and arrives 31.25 hours after the start, at 01:15 on day 2, in the night pulse. Fuel on day 2: cruiser 14.5 t, destroyer 7.25 t. Totals as above.
//!
//! **Case C: the key on and no order inside a day.** Every daily hash is scenario 1's.
//!
//! **Beside the numbers,** tests that state what must always hold:
//!
//! - An order of a kind not among the seven, handed in at 0600, is refused as given at dusk and changes nothing but its event: the truth of each day is the truth without it, the arrival event's number set aside, and the daily hash is that of the same game with one order refused at dusk, since the hash counts orders and events (decision 0055).
//! - The later the order, the farther the force: the same order at dusk of day 1 (8 kn for 24 hours), at 0600 and at 1200 gives 192, 288 and 336 miles steamed on day 1.
//! - With a staff: in scenario 36, the one convoy, a `set_speed` at 0600 to the convoy's force, which is in the forces limit of the supply intent, records `HeldByOrder` in the morning pulse. At the next dusk the staff gives the held force no order: it sets it aside as chapter 8 has it, ends the run with a reason that names the hold, frees the ship and forms another run with another ship.
//! - A saved game of case A replays to the same daily hashes. The same file with the order's pulse changed to the afternoon replays to case B's.

use crate::common;

use std::path::PathBuf;

use crate::common::values::{hex, n};
use sim_core::command::{CommandKind, HoldTarget};
use sim_core::events::{Cause, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, EntityRef, OrderId, ShipId, TaskForceId};
use sim_core::orders::{
    Issuer, OrderKind, OrderRequest, Origin, Readiness, RejectReason, SpeedSetting, TaskForceOrder,
};
use sim_core::state::{Side, TruthState};
use sim_core::time::{ClockTime, Pulse};
use sim_core::{DayInHand, DayPart, DaySummary, SimError, World};
use sim_data::{Scenario, ScriptInsideDay};
use sim_staff::{as_staffs, staffs};

const TASK_FORCE: TaskForceId = TaskForceId::new(1);
const CRUISER: ShipId = ShipId::new(1);
const DESTROYER: ShipId = ShipId::new(2);

fn path(name: &str) -> PathBuf {
    common::data_dir().join("scenarios").join(name)
}

/// Case A: the file as it is, on the frozen figures.
fn load() -> Scenario {
    common::load(&path("78_orders_inside_a_day.toml")).expect("scenario 78 loads")
}

/// The file with its order inside a day handed in for `pulse` (case B with
/// the afternoon).
fn with_pulse(pulse: Pulse) -> Scenario {
    let mut scenario = load();
    let moved = moved_to(scenario.scripted_orders_inside_day(), pulse);
    scenario.replace_script_inside_day(moved);
    scenario
}

/// `inside` with every order's pulse set to `pulse`.
fn moved_to(inside: &ScriptInsideDay, pulse: Pulse) -> ScriptInsideDay {
    inside
        .iter()
        .map(|(day, list)| {
            let list = list
                .iter()
                .map(|(_, side, request)| (pulse, *side, request.clone()))
                .collect();
            (*day, list)
        })
        .collect()
}

/// Case C: the key on and no order inside a day.
fn case_c() -> Scenario {
    let mut scenario = load();
    scenario.replace_script_inside_day(ScriptInsideDay::new());
    scenario
}

/// The 8-knot order the file gives at 0600.
fn the_order() -> OrderRequest {
    let inside = load().scripted_orders_inside_day().clone();
    assert_eq!(inside.len(), 1, "one day has orders inside it");
    let list = &inside[&1];
    assert_eq!(list.len(), 1, "day 1 has one order inside it");
    let (pulse, side, request) = &list[0];
    assert_eq!((*pulse, *side), (Pulse::Morning, Side::Allied));
    assert_eq!(
        request.kind,
        OrderKind::TaskForce {
            target: TASK_FORCE,
            order: TaskForceOrder::SetSpeed(SpeedSetting::Knots(Fixed::from_int(8))),
        }
    );
    request.clone()
}

/// The same order given at dusk of day 1, after the file's two orders, so
/// that it replaces the cruise speed: 8 kn for the whole day.
fn at_dusk() -> Scenario {
    let mut scenario = load();
    let mut script = scenario.scripted_orders().clone();
    script
        .entry(1)
        .or_default()
        .push((Side::Allied, the_order()));
    scenario.replace_script(script);
    scenario.replace_script_inside_day(ScriptInsideDay::new());
    scenario
}

/// Every day of `scenario` resolved, with the truth at the end of each.
struct Run {
    summaries: Vec<DaySummary>,
    truths: Vec<TruthState>,
    world: World,
}

fn run(mut scenario: Scenario) -> Run {
    let mut summaries = Vec::new();
    let mut truths = Vec::new();
    while let Some(summary) = scenario.next_day().expect("the day resolves") {
        summaries.push(summary);
        truths.push(scenario.world.umpire().clone());
    }
    Run {
        summaries,
        truths,
        world: scenario.world,
    }
}

impl Run {
    fn hashes(&self) -> Vec<u64> {
        self.summaries.iter().map(|s| s.state_hash).collect()
    }

    /// The events of day `day`.
    fn day(&self, day: u32) -> &[Event] {
        let summary = &self.summaries[day as usize - 1];
        &self.world.events().from_index(summary.first_event)[..summary.event_count]
    }

    /// The movement of day `day`, pulse by pulse: the pulse, the miles, the
    /// hex at the start and at the end, the hexes entered.
    fn steamed(&self, day: u32) -> Vec<(Pulse, Fixed, Hex, Hex, Vec<Hex>)> {
        self.day(day)
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

    fn miles(&self, day: u32) -> Fixed {
        self.steamed(day)
            .iter()
            .fold(Fixed::ZERO, |sum, steam| sum + steam.1)
    }

    /// The fuel `ship` burned on day `day`.
    fn burned(&self, day: u32, ship: ShipId) -> Fixed {
        self.day(day)
            .iter()
            .filter_map(|event| match &event.kind {
                EventKind::FuelBurned {
                    ship: burner,
                    burned_t,
                    ..
                } if *burner == ship => Some(*burned_t),
                _ => None,
            })
            .fold(Fixed::ZERO, |sum, burned| sum + burned)
    }

    /// The fuel `ship` has at the end of day `day`.
    fn fuel(&self, day: u32, ship: ShipId) -> Fixed {
        self.truths[day as usize - 1].ships[&ship].fuel_t
    }

    /// Where the force is at the end of day `day`.
    fn position(&self, day: u32) -> Hex {
        self.truths[day as usize - 1].task_forces[&TASK_FORCE].position()
    }

    /// The arrival: its day, its pulse and the hours since the start.
    fn arrival(&self) -> (u32, Option<Pulse>, Fixed) {
        let arrivals: Vec<_> = self
            .world
            .events()
            .iter()
            .filter_map(|event| match &event.kind {
                EventKind::TaskForceArrived { elapsed_hours, .. } => {
                    Some((event.day, event.pulse, *elapsed_hours))
                }
                _ => None,
            })
            .collect();
        assert_eq!(arrivals.len(), 1, "the force arrives once");
        arrivals[0]
    }

    /// The order taken in inside a day: the one accepted or refused event
    /// of day 1 with a pulse. Its order and the event.
    fn order_inside(&self) -> (OrderId, &Event) {
        let found: Vec<(OrderId, &Event)> =
            self.day(1)
                .iter()
                .filter(|event| event.pulse.is_some())
                .filter_map(|event| match &event.kind {
                    EventKind::OrderAccepted { order, .. }
                    | EventKind::OrderRejected { order, .. } => Some((*order, event)),
                    _ => None,
                })
                .collect();
        assert_eq!(found.len(), 1, "one order is taken in inside day 1");
        found[0]
    }

    /// Every event that names `order`, by a cause or as its subject.
    fn naming(&self, order: OrderId) -> Vec<&Event> {
        self.world
            .events()
            .iter()
            .filter(|event| {
                event.causes.contains(&Cause::Order(order))
                    || matches!(
                        &event.kind,
                        EventKind::OrderAccepted { order: o, .. }
                            | EventKind::OrderRejected { order: o, .. }
                            | EventKind::HeldByOrder { order: o, .. } if *o == order
                    )
            })
            .collect()
    }
}

/// The miles steamed when the force enters each hex, from the setup,
/// doubled so that they are whole.
const ENTERED_AT_DOUBLED: [((i32, i32), i64); 11] = [
    ((1, 0), 38),
    ((2, 0), 115),
    ((3, 0), 191),
    ((4, 0), 265),
    ((5, 0), 337),
    ((6, 0), 406),
    ((7, 0), 470),
    ((8, 0), 524),
    ((8, 1), 588),
    ((8, 2), 668),
    ((8, 3), 748),
];

/// The hexes the setup says the force enters after `from` miles and by
/// `to` miles of the transit.
fn entered_between(from: Fixed, to: Fixed) -> Vec<Hex> {
    ENTERED_AT_DOUBLED
        .iter()
        .filter(|(_, doubled)| {
            let at = Fixed::from_ratio(*doubled, 2);
            at > from && at <= to
        })
        .map(|((q, r), _)| hex(*q, *r))
        .collect()
}

/// Each pulse's hexes entered are the setup's between the miles steamed
/// before it and after it.
fn assert_hexes_follow_the_setup(run: &Run, days: &[u32]) {
    let mut total = Fixed::ZERO;
    for day in days {
        for (pulse, miles, _, _, entered) in run.steamed(*day) {
            let before = total;
            total += miles;
            assert_eq!(
                entered,
                entered_between(before, total),
                "day {day}, {pulse:?}: {before} to {total} miles"
            );
        }
    }
}

#[test]
fn the_file_is_scenario_1_with_the_key_on_and_one_order_at_0600() {
    let scenario = load();
    let first = common::load(&path("01_transit.toml")).expect("scenario 1 loads");
    assert!(scenario.placeholder);
    assert!(scenario.orders_inside_a_day());
    assert!(!first.orders_inside_a_day());
    assert_eq!(scenario.days, first.days);
    assert_eq!(scenario.world.seed(), first.world.seed());
    assert_eq!(scenario.world.umpire(), first.world.umpire());
    assert_eq!(scenario.data_files(), first.data_files());
    assert_eq!(scenario.scripted_orders(), first.scripted_orders());
    assert!(first.scripted_orders_inside_day().is_empty());
    the_order();
}

#[test]
fn case_a_the_order_at_0600_is_taken_in_the_morning_and_the_force_slows() {
    let a = run(load());
    assert_eq!(a.summaries.len(), 3);

    // The order is accepted in the morning pulse of day 1; no event of the
    // night names it.
    let (order, answer) = a.order_inside();
    assert_eq!((answer.day, answer.pulse), (1, Some(Pulse::Morning)));
    assert!(matches!(
        answer.kind,
        EventKind::OrderAccepted {
            target: EntityRef::TaskForce(TASK_FORCE),
            ..
        }
    ));
    assert_eq!(answer.causes, vec![Cause::Order(order)]);
    assert_eq!(
        a.world.order(order).expect("on record").kind,
        the_order().kind
    );
    let naming = a.naming(order);
    assert!(!naming.is_empty());
    assert!(
        naming
            .iter()
            .all(|event| !(event.day == 1 && matches!(event.pulse, None | Some(Pulse::Night)))),
        "no event of dusk or the night of day 1 names the order"
    );

    // Day 1, pulse by pulse.
    assert_eq!(
        a.steamed(1),
        vec![
            (
                Pulse::Night,
                n(192),
                hex(0, 0),
                hex(5, 0),
                vec![hex(1, 0), hex(2, 0), hex(3, 0), hex(4, 0), hex(5, 0)]
            ),
            (
                Pulse::Morning,
                n(48),
                hex(5, 0),
                hex(7, 0),
                vec![hex(6, 0), hex(7, 0)]
            ),
            (
                Pulse::Afternoon,
                n(48),
                hex(7, 0),
                hex(8, 0),
                vec![hex(8, 0)]
            ),
        ]
    );
    // The morning and the afternoon steam at 8 kn by the order.
    for steam in a.day(1).iter().filter(|e| {
        matches!(e.kind, EventKind::TaskForceSteamed { .. }) && e.pulse != Some(Pulse::Night)
    }) {
        assert!(steam.causes.contains(&Cause::Order(order)));
    }

    // End of day 1: 288 miles, in (8,0); cruiser 72 t burned and 1,928 t
    // left, destroyer 36 t and 464 t.
    assert_eq!(a.miles(1), n(288));
    assert_eq!(a.position(1), hex(8, 0));
    assert_eq!(a.burned(1, CRUISER), n(72));
    assert_eq!(a.fuel(1, CRUISER), n(1928));
    assert_eq!(a.burned(1, DESTROYER), n(36));
    assert_eq!(a.fuel(1, DESTROYER), n(464));

    // Day 2: the night steams 96, to 384, entering (8,1), (8,2) and (8,3);
    // the last 10 miles take 1.25 hours of the morning.
    assert_eq!(
        a.steamed(2),
        vec![
            (
                Pulse::Night,
                n(96),
                hex(8, 0),
                hex(8, 3),
                vec![hex(8, 1), hex(8, 2), hex(8, 3)]
            ),
            (Pulse::Morning, n(10), hex(8, 3), hex(8, 3), vec![]),
        ]
    );
    assert_eq!(a.miles(1) + a.miles(2), n(394));
    // Arrival 37.25 hours after the start, at 07:15 on day 2, in the
    // morning pulse.
    let (day, pulse, hours) = a.arrival();
    assert_eq!((day, pulse), (2, Some(Pulse::Morning)));
    assert_eq!(hours, Fixed::from_ratio(149, 4));
    assert_eq!(ClockTime::from_elapsed(hours).to_string(), "D2 07:15");
    // Fuel on day 2: cruiser 26.5 t, destroyer 13.25 t.
    assert_eq!(a.burned(2, CRUISER), Fixed::from_ratio(53, 2));
    assert_eq!(a.burned(2, DESTROYER), Fixed::from_ratio(53, 4));
    // Totals: cruiser 98.5 t, destroyer 49.25 t.
    assert_eq!(n(2000) - a.fuel(3, CRUISER), Fixed::from_ratio(197, 2));
    assert_eq!(n(500) - a.fuel(3, DESTROYER), Fixed::from_ratio(197, 4));
    assert_eq!(a.burned(3, CRUISER), Fixed::ZERO);

    assert_hexes_follow_the_setup(&a, &[1, 2, 3]);
}

#[test]
fn case_b_the_order_at_1200_is_taken_in_the_afternoon() {
    let b = run(with_pulse(Pulse::Afternoon));

    // Accepted in the afternoon pulse of day 1.
    let (order, answer) = b.order_inside();
    assert_eq!((answer.day, answer.pulse), (1, Some(Pulse::Afternoon)));
    assert!(matches!(answer.kind, EventKind::OrderAccepted { .. }));
    assert!(b
        .naming(order)
        .iter()
        .all(|event| !(event.day == 1 && event.pulse != Some(Pulse::Afternoon))));

    assert_eq!(
        b.steamed(1),
        vec![
            (
                Pulse::Night,
                n(192),
                hex(0, 0),
                hex(5, 0),
                vec![hex(1, 0), hex(2, 0), hex(3, 0), hex(4, 0), hex(5, 0)]
            ),
            (
                Pulse::Morning,
                n(96),
                hex(5, 0),
                hex(8, 0),
                vec![hex(6, 0), hex(7, 0), hex(8, 0)]
            ),
            (
                Pulse::Afternoon,
                n(48),
                hex(8, 0),
                hex(8, 2),
                vec![hex(8, 1), hex(8, 2)]
            ),
        ]
    );

    // End of day 1: 336 miles, in (8,2); cruiser 84 t burned and 1,916 t
    // left, destroyer 42 t and 458 t.
    assert_eq!(b.miles(1), n(336));
    assert_eq!(b.position(1), hex(8, 2));
    assert_eq!(b.burned(1, CRUISER), n(84));
    assert_eq!(b.fuel(1, CRUISER), n(1916));
    assert_eq!(b.burned(1, DESTROYER), n(42));
    assert_eq!(b.fuel(1, DESTROYER), n(458));

    // Day 2: 58 miles remain, 7.25 hours at 8 kn; the force enters (8,3)
    // and arrives 31.25 hours after the start, at 01:15 on day 2, in the
    // night pulse.
    assert_eq!(
        b.steamed(2),
        vec![(Pulse::Night, n(58), hex(8, 2), hex(8, 3), vec![hex(8, 3)])]
    );
    let (day, pulse, hours) = b.arrival();
    assert_eq!((day, pulse), (2, Some(Pulse::Night)));
    assert_eq!(hours, Fixed::from_ratio(125, 4));
    assert_eq!(ClockTime::from_elapsed(hours).to_string(), "D2 01:15");
    // Fuel on day 2: cruiser 14.5 t, destroyer 7.25 t.
    assert_eq!(b.burned(2, CRUISER), Fixed::from_ratio(29, 2));
    assert_eq!(b.burned(2, DESTROYER), Fixed::from_ratio(29, 4));
    // The same totals.
    assert_eq!(n(2000) - b.fuel(3, CRUISER), Fixed::from_ratio(197, 2));
    assert_eq!(n(500) - b.fuel(3, DESTROYER), Fixed::from_ratio(197, 4));

    assert_hexes_follow_the_setup(&b, &[1, 2, 3]);
}

#[test]
fn case_c_with_the_key_on_and_no_order_inside_a_day_every_hash_is_scenario_1s() {
    let c = run(case_c());
    let first = run(common::load(&path("01_transit.toml")).expect("scenario 1 loads"));
    assert_eq!(c.hashes().len(), 3);
    assert_eq!(c.hashes(), first.hashes());
}

#[test]
fn an_order_of_another_kind_at_0600_is_refused_as_given_at_dusk() {
    let mut scenario = load();
    let home_port = OrderRequest {
        kind: OrderKind::TaskForce {
            target: TASK_FORCE,
            order: TaskForceOrder::SetHomePort(BaseId::new(2)),
        },
        origin: Origin {
            issuer: Issuer::Player,
            intent: None,
        },
    };
    // At dusk the same order would be accepted.
    assert_eq!(scenario.world.check_order(Side::Allied, &home_port), Ok(()));
    let mut inside = ScriptInsideDay::new();
    inside.insert(1, vec![(Pulse::Morning, Side::Allied, home_port)]);
    scenario.replace_script_inside_day(inside);
    let refused = run(scenario);
    let c = run(case_c());

    let (order, answer) = refused.order_inside();
    assert_eq!((answer.day, answer.pulse), (1, Some(Pulse::Morning)));
    assert_eq!(
        answer.kind,
        EventKind::OrderRejected {
            order,
            target: EntityRef::TaskForce(TASK_FORCE),
            reason: RejectReason::GivenAtDusk,
        }
    );
    assert!(RejectReason::GivenAtDusk
        .to_string()
        .contains("given at dusk"));
    // It changes nothing but its event.
    assert_eq!(refused.naming(order), vec![answer]);
    assert_eq!(refused.truths[0].task_forces[&TASK_FORCE].home_port, None);
    let kinds = |run: &Run, leave_out: Option<sim_core::ids::EventId>| {
        run.world
            .events()
            .iter()
            .filter(|event| Some(event.id) != leave_out)
            .map(|event| (event.day, event.pulse, event.kind.clone()))
            .collect::<Vec<_>>()
    };
    assert_eq!(kinds(&refused, Some(answer.id)), kinds(&c, None));
    // The truth, day by day, is case C's, but for the number of the event
    // that records the arrival, which the refusal moved on by one.
    let numbered_apart = |truths: &[TruthState]| -> Vec<TruthState> {
        truths
            .iter()
            .map(|truth| {
                let mut truth = truth.clone();
                for force in truth.task_forces.values_mut() {
                    force.arrival_event = None;
                }
                truth
            })
            .collect()
    };
    assert_eq!(numbered_apart(&refused.truths), numbered_apart(&c.truths));

    // The daily hash counts every order received and every event (decision
    // 0055), so it is not case C's: it is the hash of the same game with one
    // order refused at dusk instead, which counts one order and one event
    // more, as this one does. Reported to the architect: the plan's "the
    // day is the day it would have been" holds of the truth and the events,
    // and the hash, which counts them, differs by that count.
    let mut refused_at_dusk = case_c();
    let mut script = refused_at_dusk.scripted_orders().clone();
    script.entry(1).or_default().push((
        Side::Allied,
        OrderRequest {
            kind: OrderKind::TaskForce {
                target: TASK_FORCE,
                order: TaskForceOrder::SetSpeed(SpeedSetting::Knots(Fixed::ZERO)),
            },
            origin: Origin {
                issuer: Issuer::Player,
                intent: None,
            },
        },
    ));
    refused_at_dusk.replace_script(script);
    let at_dusk = run(refused_at_dusk);
    assert!(at_dusk.world.events().iter().any(|event| matches!(
        event.kind,
        EventKind::OrderRejected {
            reason: RejectReason::SpeedNotAboveZero(_),
            ..
        }
    )));
    assert_eq!(refused.hashes(), at_dusk.hashes());
}

#[test]
fn the_later_the_order_the_farther_the_force() {
    let dusk = run(at_dusk());
    let morning = run(load());
    let afternoon = run(with_pulse(Pulse::Afternoon));
    assert_eq!(
        [dusk.miles(1), morning.miles(1), afternoon.miles(1)],
        [n(192), n(288), n(336)]
    );
}

#[test]
fn with_a_staff_an_order_at_0600_holds_the_convoy_from_the_staff() {
    let mut scenario = common::load(&path("36_one_convoy.toml")).expect("scenario 36 loads");
    let held = staffs();
    // Day 1 in its parts, the scenario's commands and orders handed in as
    // `Scenario::next_day_watched` hands them in.
    let commands = scenario.scripted_commands().clone();
    let script = scenario.scripted_orders().clone();
    let world = &mut scenario.world;
    for (side, command) in commands.get(&1).cloned().unwrap_or_default() {
        world.submit_commands(side, vec![command]);
    }
    for (side, request) in script.get(&1).cloned().unwrap_or_default() {
        world.submit_orders(side, vec![request]);
    }
    world
        .open_day(as_staffs(&held), &mut |_, _| {})
        .expect("day 1 opens");
    // The staff formed the convoy at dusk: the side's only force.
    let convoy: Vec<TaskForceId> = world
        .umpire()
        .task_forces
        .values()
        .filter(|force| force.side == Side::Allied)
        .map(|force| force.id)
        .collect();
    assert_eq!(convoy.len(), 1, "the staff formed one force");
    let convoy = convoy[0];
    world.resolve_next_pulse(&mut |_, _| {}).expect("the night");
    let at = world
        .submit_orders_inside_day(
            Side::Allied,
            vec![OrderRequest {
                kind: OrderKind::TaskForce {
                    target: convoy,
                    order: TaskForceOrder::SetSpeed(SpeedSetting::Knots(n(8))),
                },
                origin: Origin {
                    issuer: Issuer::Player,
                    intent: None,
                },
            }],
        )
        .expect("handed in at 0600");
    // A second order to the same force at 0600: taken in after the first,
    // and the force, already held, is not held again.
    world
        .submit_orders_inside_day(
            Side::Allied,
            vec![OrderRequest {
                kind: OrderKind::TaskForce {
                    target: convoy,
                    order: TaskForceOrder::SetReadiness(Readiness::Battle),
                },
                origin: Origin {
                    issuer: Issuer::Player,
                    intent: None,
                },
            }],
        )
        .expect("handed in at 0600");
    assert_eq!(at, Pulse::Morning);
    world
        .resolve_next_pulse(&mut |_, _| {})
        .expect("the morning");
    world
        .resolve_next_pulse(&mut |_, _| {})
        .expect("the afternoon");
    let day1 = world.close_day(&mut |_, _| {}).expect("day 1 closes");

    // `HeldByOrder` in the morning pulse, citing the order and its
    // acceptance, which is in the same pulse.
    let day1_events = &world.events().from_index(day1.first_event)[..day1.event_count];
    let held_by: Vec<&Event> = day1_events
        .iter()
        .filter(|event| matches!(event.kind, EventKind::HeldByOrder { .. }))
        .collect();
    assert_eq!(held_by.len(), 1, "held once, by the first order");
    let accepted_in_morning = day1_events
        .iter()
        .filter(|event| {
            event.pulse == Some(Pulse::Morning)
                && matches!(
                    event.kind,
                    EventKind::OrderAccepted { target: EntityRef::TaskForce(t), .. } if t == convoy
                )
        })
        .count();
    assert_eq!(
        accepted_in_morning, 2,
        "both orders are taken in the morning"
    );
    let held_event = held_by[0];
    let EventKind::HeldByOrder {
        side,
        target,
        order,
    } = held_event.kind
    else {
        unreachable!()
    };
    assert_eq!(
        (side, target),
        (Side::Allied, HoldTarget::TaskForce(convoy))
    );
    assert_eq!(
        (held_event.day, held_event.pulse),
        (1, Some(Pulse::Morning))
    );
    let accepted = day1_events
        .iter()
        .find(|event| matches!(event.kind, EventKind::OrderAccepted { order: o, .. } if o == order))
        .expect("the order is accepted");
    assert_eq!(accepted.pulse, Some(Pulse::Morning));
    assert_eq!(
        held_event.causes,
        vec![Cause::Order(order), Cause::Event(accepted.id)]
    );
    assert!(world
        .command_state(Side::Allied)
        .holds
        .contains_key(&HoldTarget::TaskForce(convoy)));

    // The next dusk. The staff would have ordered the convoy on day 2 (the
    // route to base T and back and the unload, as scenario 36 has it). With
    // the convoy held its doctrine hands in no order to it, so none is
    // refused as held: it ends the convoy's run, saying the force is held,
    // frees its ship, and sends the other ship of the pool in a new force.
    let day2 = scenario
        .next_day_staffed(as_staffs(&held))
        .expect("day 2 resolves")
        .expect("day 2 exists");
    let day2_events = &scenario.world.events().from_index(day2.first_event)[..day2.event_count];
    assert!(
        !day2_events.iter().any(|event| matches!(
            &event.kind,
            EventKind::OrderAccepted { target: EntityRef::TaskForce(t), .. }
                | EventKind::OrderRejected { target: EntityRef::TaskForce(t), .. } if *t == convoy
        )),
        "no order to the held convoy is handed in"
    );
    assert!(
        !day2_events.iter().any(|event| matches!(
            &event.kind,
            EventKind::OrderRejected {
                reason: RejectReason::Held(_),
                ..
            }
        )),
        "no order is refused as held"
    );
    // The staff ends run 1, the convoy's, and says why: the hold.
    let ended = day2_events
        .iter()
        .find_map(|event| match &event.kind {
            EventKind::StaffReason {
                command: Some(command),
                reason,
                ..
            } if matches!(**command, CommandKind::EndRun { run: 1, .. }) => Some(reason),
            _ => None,
        })
        .expect("the staff ends run 1");
    let held_words = format!("{convoy} is held by the commander's order of day 1");
    assert!(ended.worked.contains(&held_words), "{}", ended.worked);
    assert!(ended.evidence.iter().any(|e| e.what == held_words));
    // It sends ship 2 instead, in a new force that it orders to load.
    let formed: Vec<TaskForceId> = day2_events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::TaskForceFormed {
                task_force, ships, ..
            } if ships == &vec![ShipId::new(2)] => Some(*task_force),
            _ => None,
        })
        .collect();
    assert_eq!(formed.len(), 1, "ship 2 is formed into a force");
    assert!(day2_events.iter().any(|event| matches!(
        &event.kind,
        EventKind::OrderAccepted { target: EntityRef::TaskForce(t), .. } if *t == formed[0]
    )));
    // The convoy is still held, in port, with its order of 8 kn.
    assert!(scenario
        .world
        .command_state(Side::Allied)
        .holds
        .contains_key(&HoldTarget::TaskForce(convoy)));
    assert_eq!(
        scenario.world.umpire().task_forces[&convoy].speed,
        SpeedSetting::Knots(n(8))
    );
}

#[test]
fn a_replay_of_the_orders_a_saved_game_keeps_gives_the_same_hashes() {
    // What a saved game keeps: the orders of dusk and the orders inside a
    // day with their pulses, read before the run.
    let fresh = load();
    let dusk = fresh.scripted_orders().clone();
    let inside = fresh.scripted_orders_inside_day().clone();
    let a = run(fresh);

    let mut replay = load();
    replay.replace_script(dusk.clone());
    replay.replace_script_inside_day(inside.clone());
    assert_eq!(run(replay).hashes(), a.hashes());

    // The same orders with the pulse changed to the afternoon replay to
    // case B's hashes.
    let b = run(with_pulse(Pulse::Afternoon));
    let mut changed = load();
    changed.replace_script(dusk);
    changed.replace_script_inside_day(moved_to(&inside, Pulse::Afternoon));
    assert_eq!(run(changed).hashes(), b.hashes());
    assert_ne!(a.hashes()[0], b.hashes()[0]);
}

#[test]
fn orders_are_handed_in_inside_a_day_only_at_0600_and_1200() {
    let mut scenario = load();
    let order = the_order();
    let world = &mut scenario.world;
    let refused = |world: &mut World, in_hand: Option<DayInHand>| {
        let before = world.clone();
        let error = world
            .submit_orders_inside_day(Side::Allied, vec![order.clone()])
            .expect_err("refused");
        assert_eq!(
            error,
            SimError::DayPartOutOfTurn {
                part: DayPart::Orders,
                in_hand,
            }
        );
        assert!(
            error.to_string().contains("only at 0600 or 1200"),
            "{error}"
        );
        assert!(world.orders_waiting_inside_day(Side::Allied).is_empty());
        assert_eq!(world.state_hash(), before.state_hash());
    };
    // No day open.
    refused(world, None);
    world
        .open_day(sim_core::Staffs::none(), &mut |_, _| {})
        .expect("opens");
    // Before the night.
    refused(world, world.day_in_hand());
    world.resolve_next_pulse(&mut |_, _| {}).expect("the night");
    assert_eq!(
        world.submit_orders_inside_day(Side::Allied, vec![order.clone()]),
        Ok(Pulse::Morning)
    );
    assert_eq!(
        world.orders_waiting_inside_day(Side::Allied),
        std::slice::from_ref(&order)
    );
    world
        .resolve_next_pulse(&mut |_, _| {})
        .expect("the morning");
    assert!(world.orders_waiting_inside_day(Side::Allied).is_empty());
    assert_eq!(
        world.submit_orders_inside_day(Side::Allied, Vec::new()),
        Ok(Pulse::Afternoon)
    );
    world
        .resolve_next_pulse(&mut |_, _| {})
        .expect("the afternoon");
    // After the afternoon.
    refused(world, world.day_in_hand());
    world.close_day(&mut |_, _| {}).expect("closes");
}
