//! Scenario 26: a force bombards by night and is gone by day; a convoy
//! unloads and comes home by itself; and a marked waypoint with nothing to
//! do there is passed.
//!
//! Proves decision 0096 (a force that acts at a place and steams on) against
//! a hand calculation. The data is placeholder data: see
//! `data/scenarios/26_act_and_steam_on.toml`. The test runs on the frozen
//! step 1 figures (`data/reference/step1`).
//!
//! # Hand calculation
//!
//! Written by the architect before the code. What the dice decide (the hits
//! of the bombardment and what they destroy) is read from the run; the rest
//! is asserted exactly. If the code disagrees, the code or the rule is
//! wrong: do not change a number here to make a test pass. A mismatch goes
//! back to the architect.
//!
//! ## Inputs
//!
//! A placeholder map of open sea with three coast hexes: X at (0,0), Y at
//! (0,3) with port size 0, and Z at (4,3). The game is 2 days. Hour 0 is
//! 18:00 on day 1; the night pulse is 12 hours and each daylight pulse 6.
//!
//! Figures, frozen: `bombardment_minutes` 30,
//! `bombardment_reference_gun_damage` 20; a port of size 0 works cargo by
//! lighter at 20 t an hour. Classes, frozen: the placeholder cruiser (8 guns
//! at 3 rounds a minute, gun damage 20, 16 kn at cruise, 0.25 t of fuel a
//! mile), the placeholder destroyer (16 kn at cruise, 0.125 t a mile) and
//! the placeholder cargo ship (10 kn at cruise, 0.1 t a mile, 3000 t of
//! cargo).
//!
//! - Allied base 1 at X, supply 1000 t, with an air group of 20 ready
//!   aircraft and no mission. Allied base 2 at Y, supply 0. Allied base 3 at
//!   Z, supply 0.
//! - Japanese task force 1 at sea at (4,0), mission bombardment: ship 1, a
//!   placeholder cruiser with 800 rounds and 2000 t of fuel.
//! - Allied task force 2 at base 3: ship 2, a placeholder cargo ship with
//!   100 t of supply aboard and 1000 t of fuel.
//! - Allied task force 3 at sea at (0,5): ship 3, a placeholder destroyer
//!   with 500 t of fuel. It has no cargo order and its mission is surface
//!   combat.
//!
//! Orders, all on day 1, all at cruise:
//!
//! - Task force 1: route to (0,0), marked to act there, then (4,0).
//! - Task force 2: unload; route to (0,3), marked to act there, then (4,3).
//! - Task force 3: route to (2,5), marked to act there, then (4,5).
//!
//! ## Task force 1: a raid
//!
//! Day 1, night. 4 hexes, 160 nm, at 16 kn is 10 hours: it reaches (0,0) at
//! hour 10, 04:00 on day 1. The waypoint is marked and it has work there (a
//! bombardment mission, a ship that can fire, an enemy base on its map): it
//! halts. In the shore bombardment phase ship 1 fires 3 x 8 x 30 = 720
//! shells, which are 720 weighed shots, and has 80 rounds left.
//!
//! Day 1, morning. It has bombarded since it halted: it steams on, with the
//! pulse's 6 hours. 96 nm toward (4,0): two hexes and 16 nm, so its position
//! at the end of the pulse is (2,0).
//!
//! Day 1, afternoon. The last 64 nm take 4 hours. It arrives at (4,0) at
//! hour 22, 16:00 on day 1.
//!
//! Fuel: 320 nm at 0.25 t is 80 t. Ship 1 has 1920 t. On the night of day 2
//! it is at (4,0), where there is no enemy base, and does not bombard. It
//! ends with 80 rounds.
//!
//! In no daylight pulse is its position (0,0).
//!
//! ## Task force 2: a supply run
//!
//! Day 1. 4 hexes, 160 nm, at 10 kn is 16 hours: 120 nm in the night pulse
//! and the last 40 nm in the first 4 hours of the morning. It reaches (0,3)
//! at hour 16, 10:00 on day 1. The waypoint is marked and it has work there
//! (an unload order, supply aboard, a base of its side): it halts.
//!
//! | Pulse            | Hours worked | Unloaded          | Aboard | Base 2 |
//! | ---------------- | ------------ | ----------------- | ------ | ------ |
//! | Day 1, morning   | 2            | 2 x 20 = 40 t     | 60 t   | 40 t   |
//! | Day 1, afternoon | 6            | 60 t, the rest    | 0 t    | 100 t  |
//!
//! Unloading is complete 3 hours into the afternoon, at hour 21, 15:00 on
//! day 1.
//!
//! Day 2, night. Its cargo work is complete: it steams on. 120 nm in the 12
//! hours. Day 2, morning: the last 40 nm in 4 hours. It arrives at (4,3) at
//! hour 24 + 12 + 4 = 40, 10:00 on day 2.
//!
//! Fuel: 320 nm at 0.1 t is 32 t. Ship 2 has 968 t. Base 2 holds 100 t and
//! base 3 holds 0.
//!
//! ## Task force 3: nothing to do
//!
//! The waypoint (2,5) is marked, but the force has no bombardment mission
//! and no cargo order: it has no work and does not halt. 4 hexes, 160 nm, at
//! 16 kn is 10 hours. It arrives at (4,5) at hour 10, 04:00 on day 1, with
//! no halt recorded. Ship 3 burns 160 x 0.125 = 20 t and has 480 t.
//!
//! # Properties
//!
//! - A route with no mark gives the movement it gave before, apart from the
//!   hash version, in every existing scenario.
//! - A halted force never moves in a pulse in which its work is not done.
//! - The enemy's view is the same in two worlds that differ only in a mark
//!   on a waypoint where the force has no work.
//!
//! # The team's own extras
//!
//! The tests whose doc comments say "the team's own" are not the architect's:
//! they are cases the hand calculation does not reach, each with a short
//! calculation of its own on the same frozen figures. They assert behaviour
//! (which pulse a force halts, fires, steams on in; where it ends) and not
//! the dice.

mod common;

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use sim_core::events::{Cause, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, OrderId, ShipId, TaskForceId};
use sim_core::orders::{OrderKind, TaskForceOrder};
use sim_core::state::{HaltWork, Side, TaskForceStatus, TruthState};
use sim_core::time::Pulse;
use sim_data::Scenario;

// ------------------------------------------------------------------ Helpers

const X: Hex = Hex::new(0, 0);
const Y: Hex = Hex::new(0, 3);
const Z: Hex = Hex::new(4, 3);
const SEA: Hex = Hex::new(4, 0);
const PASSED: Hex = Hex::new(2, 5);
const HOME_3: Hex = Hex::new(4, 5);

const BOMBARD: HaltWork = HaltWork {
    bombard: true,
    cargo: false,
};
const CARGO: HaltWork = HaltWork {
    bombard: false,
    cargo: true,
};
const BOTH: HaltWork = HaltWork {
    bombard: true,
    cargo: true,
};

fn path() -> PathBuf {
    common::data_dir().join("scenarios/26_act_and_steam_on.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn from(source: &str) -> Scenario {
    common::from_text(source, &path()).expect("the variant loads")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn tf(raw: u32) -> TaskForceId {
    TaskForceId::new(raw)
}

fn ship(raw: u32) -> ShipId {
    ShipId::new(raw)
}

fn base(raw: u32) -> BaseId {
    BaseId::new(raw)
}

/// The text of the order that gives task force 1 its speed.
const SPEED_1: &str =
    "[[order]]\nday = 1\nside = \"japanese\"\ntask_force = 1\nkind = \"set_speed\"\nspeed = \"cruise\"\n";
const MARK_1: &str = "act_at = [[0, 0]]\n";
const MARK_2: &str = "act_at = [[0, 3]]\n";
const MARK_3: &str = "act_at = [[2, 5]]\n";

/// A run of a scenario: the truth at the start (day 0) and at the end of
/// each day, the Japanese side's view at the end of each day, the log and
/// the orders.
struct Run {
    days: Vec<TruthState>,
    japanese: Vec<String>,
    events: Vec<Event>,
    orders: BTreeMap<OrderId, OrderKind>,
}

/// What the Japanese side holds at this moment: every event it may see, as
/// its view gives them, and its contacts.
fn japanese_digest(scenario: &Scenario) -> String {
    let view = scenario.world.side_view(Side::Japanese);
    let mut out = String::new();
    for event in view.events() {
        out += &format!("{event:?}\n");
    }
    for contact in view.contacts() {
        out += &format!("{contact:?}\n");
    }
    out
}

impl Run {
    fn of(mut scenario: Scenario) -> Run {
        let mut days = vec![scenario.world.umpire().clone()];
        let mut japanese = vec![japanese_digest(&scenario)];
        while scenario.next_day().expect("a day resolves").is_some() {
            days.push(scenario.world.umpire().clone());
            japanese.push(japanese_digest(&scenario));
        }
        Run {
            events: scenario.world.events().iter().cloned().collect(),
            orders: scenario
                .world
                .orders()
                .map(|order| (order.id, order.kind.clone()))
                .collect(),
            days,
            japanese,
        }
    }

    fn text(source: &str) -> Run {
        Run::of(from(source))
    }

    fn len(&self) -> u32 {
        u32::try_from(self.days.len() - 1).expect("a few days")
    }

    /// The truth at the end of `day`; day 0 is the start.
    fn at(&self, day: u32) -> &TruthState {
        &self.days[day as usize]
    }

    fn all(&self, pick: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
        self.events.iter().filter(|e| pick(&e.kind)).collect()
    }

    /// The events of `day` and `pulse` that satisfy `pick`, in log order.
    fn on(&self, day: u32, pulse: Pulse, pick: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|e| e.day == day && e.pulse == Some(pulse) && pick(&e.kind))
            .collect()
    }

    /// The one such event; panics unless exactly one.
    fn one(&self, day: u32, pulse: Pulse, pick: impl Fn(&EventKind) -> bool) -> &Event {
        let found = self.on(day, pulse, pick);
        assert_eq!(
            found.len(),
            1,
            "day {day} {pulse:?}: exactly one such event: {found:?}"
        );
        found[0]
    }

    /// The route order of a task force: the one `SetRoute` order for it.
    fn route_order(&self, raw: u32) -> OrderId {
        let found: Vec<OrderId> = self
            .orders
            .iter()
            .filter(|(_, kind)| {
                matches!(
                    kind,
                    OrderKind::TaskForce { target, order: TaskForceOrder::SetRoute { .. } }
                        if target.raw() == raw
                )
            })
            .map(|(id, _)| *id)
            .collect();
        assert_eq!(found.len(), 1, "one route order for task force {raw}");
        found[0]
    }

    fn halted(&self, raw: u32) -> Vec<&Event> {
        self.all(|k| matches!(k, EventKind::TaskForceHalted { task_force, .. } if task_force.raw() == raw))
    }

    fn steamed_on(&self, raw: u32) -> Vec<&Event> {
        self.all(|k| matches!(k, EventKind::TaskForceSteamedOn { task_force, .. } if task_force.raw() == raw))
    }

    fn steamed(&self, raw: u32) -> Vec<&Event> {
        self.all(|k| matches!(k, EventKind::TaskForceSteamed { task_force, .. } if task_force.raw() == raw))
    }

    fn arrived(&self, raw: u32) -> Vec<&Event> {
        self.all(|k| matches!(k, EventKind::TaskForceArrived { task_force, .. } if task_force.raw() == raw))
    }

    fn bombardments(&self) -> Vec<&Event> {
        self.all(|k| matches!(k, EventKind::ShoreBombardment { .. }))
    }

    fn position(&self, day: u32, raw: u32) -> Hex {
        self.at(day).task_forces[&tf(raw)].position()
    }

    fn status(&self, day: u32, raw: u32) -> TaskForceStatus {
        self.at(day).task_forces[&tf(raw)].status
    }
}

/// A `TaskForceSteamed` event's (from, to, distance, speed).
fn leg(e: &Event) -> (Hex, Hex, Fixed, Fixed) {
    match &e.kind {
        EventKind::TaskForceSteamed {
            from,
            to,
            distance_nm,
            speed_kn,
            ..
        } => (*from, *to, *distance_nm, *speed_kn),
        other => panic!("not a steaming event: {other:?}"),
    }
}

/// A `FuelBurned` event's (burned, remaining) of the ship, on `day` in `pulse`.
fn fuel(run: &Run, day: u32, pulse: Pulse, raw: u32) -> (Fixed, Fixed) {
    let e = run.one(
        day,
        pulse,
        |k| matches!(k, EventKind::FuelBurned { ship, .. } if ship.raw() == raw),
    );
    match e.kind {
        EventKind::FuelBurned {
            burned_t,
            remaining_t,
            ..
        } => (burned_t, remaining_t),
        _ => unreachable!(),
    }
}

/// A `TaskForceHalted` event's (hex, work, elapsed hours).
fn halt_of(e: &Event) -> (Hex, HaltWork, Fixed) {
    match e.kind {
        EventKind::TaskForceHalted {
            hex,
            work,
            elapsed_hours,
            ..
        } => (hex, work, elapsed_hours),
        _ => panic!("not a halt: {e:?}"),
    }
}

/// A `TaskForceSteamedOn` event's (hex, work, fired).
fn steam_on_of(e: &Event) -> (Hex, HaltWork, bool) {
    match e.kind {
        EventKind::TaskForceSteamedOn {
            hex, work, fired, ..
        } => (hex, work, fired),
        _ => panic!("not a steaming on: {e:?}"),
    }
}

/// A `TaskForceArrived` event's (hex, elapsed hours).
fn arrival_of(e: &Event) -> (Hex, Fixed) {
    match e.kind {
        EventKind::TaskForceArrived {
            hex, elapsed_hours, ..
        } => (hex, elapsed_hours),
        _ => panic!("not an arrival: {e:?}"),
    }
}

/// A log as comparable text: each event's number, day, pulse, kind and
/// causes.
fn digest(run: &Run) -> Vec<String> {
    run.events
        .iter()
        .map(|e| {
            format!(
                "{:?} {} {:?} {:?} {:?}",
                e.id, e.day, e.pulse, e.kind, e.causes
            )
        })
        .collect()
}

/// The movement of a task force as text, with no event numbers or causes:
/// its steaming and arrival, and the fuel burned by its ships.
fn movement_of(run: &Run, raw: u32, ships: &[u32]) -> Vec<String> {
    run.events
        .iter()
        .filter(|e| match &e.kind {
            EventKind::TaskForceSteamed { task_force, .. }
            | EventKind::TaskForceArrived { task_force, .. } => task_force.raw() == raw,
            EventKind::FuelBurned { ship, .. } => ships.contains(&ship.raw()),
            _ => false,
        })
        .map(|e| format!("{} {:?} {:?}", e.day, e.pulse, e.kind))
        .collect()
}

// ------------------------------------------------------- The hand calculation

#[test]
fn the_raid_the_supply_run_and_the_passed_mark_follow_the_calculation() {
    let run = Run::text(&text());
    assert_eq!(run.len(), 2);

    // ---- Task force 1: a raid.
    let route_1 = run.route_order(1);
    let steamed = run.steamed(1);
    assert_eq!(steamed.len(), 3, "night, morning and afternoon of day 1");

    // Day 1, night: 4 hexes, 160 nm, at 16 kn is 10 hours: it reaches (0,0)
    // at hour 10 and halts, to bombard.
    let night = run.one(
        1,
        Pulse::Night,
        |k| matches!(k, EventKind::TaskForceSteamed { task_force, .. } if task_force.raw() == 1),
    );
    assert_eq!(leg(night), (SEA, X, n(160), n(16)));
    match &night.kind {
        EventKind::TaskForceSteamed { hexes_entered, .. } => assert_eq!(
            hexes_entered,
            &vec![Hex::new(3, 0), Hex::new(2, 0), Hex::new(1, 0), X]
        ),
        _ => unreachable!(),
    }
    assert_eq!(fuel(&run, 1, Pulse::Night, 1), (n(40), n(1960)));
    let halt = run.one(
        1,
        Pulse::Night,
        |k| matches!(k, EventKind::TaskForceHalted { task_force, .. } if task_force.raw() == 1),
    );
    assert_eq!(halt_of(halt), (X, BOMBARD, n(10)));
    assert_eq!(
        halt.causes,
        vec![Cause::Order(route_1), Cause::Event(night.id)]
    );
    assert_eq!(run.halted(1).len(), 1);
    assert_eq!(run.status(0, 1), TaskForceStatus::Idle);

    // In the shore bombardment phase ship 1 fires 3 x 8 x 30 = 720 shells,
    // which are 720 weighed shots, and has 80 rounds left.
    let fire = run.one(1, Pulse::Night, |k| {
        matches!(k, EventKind::BombardmentFire { .. })
    });
    let (hits, on_aircraft, on_supply, on_men) = match fire.kind {
        EventKind::BombardmentFire {
            ship: firing,
            base: target,
            shells,
            weighed_shots,
            hits,
            hits_on_aircraft,
            hits_on_supply,
            hits_on_men,
            rounds_left,
            ..
        } => {
            assert_eq!(firing, ship(1));
            assert_eq!(target, base(1));
            assert_eq!(shells, n(720));
            assert_eq!(weighed_shots, n(720));
            assert_eq!(rounds_left, n(80));
            (hits, hits_on_aircraft, hits_on_supply, hits_on_men)
        }
        _ => unreachable!(),
    };
    let bombardment = run.one(1, Pulse::Night, |k| {
        matches!(k, EventKind::ShoreBombardment { .. })
    });
    match bombardment.kind {
        EventKind::ShoreBombardment {
            task_force,
            base: target,
            hex,
            shells,
            weighed_shots,
            hits: total,
            hits_on_aircraft,
            hits_on_supply,
            hits_on_men,
        } => {
            assert_eq!((task_force, target, hex), (tf(1), base(1), X));
            assert_eq!((shells, weighed_shots), (n(720), n(720)));
            assert_eq!(
                (total, hits_on_aircraft, hits_on_supply, hits_on_men),
                (hits, on_aircraft, on_supply, on_men)
            );
        }
        _ => unreachable!(),
    }
    // What the dice decided, read from the run.
    let aircraft_lost: u32 = run
        .all(|k| matches!(k, EventKind::AircraftDestroyedOnGround { .. }))
        .iter()
        .map(|e| match e.kind {
            EventKind::AircraftDestroyedOnGround { aircraft, .. } => aircraft,
            _ => unreachable!(),
        })
        .sum();
    let supply_lost = run
        .all(|k| matches!(k, EventKind::BaseSupplyDestroyed { .. }))
        .iter()
        .map(|e| match e.kind {
            EventKind::BaseSupplyDestroyed { destroyed_t, .. } => destroyed_t,
            _ => unreachable!(),
        })
        .fold(Fixed::ZERO, |sum, t| sum + t);
    eprintln!(
        "scenario 26, frozen figures: {hits} hits of 720 weighed shots: {on_aircraft} on aircraft, \
         {on_supply} on supply, {on_men} on men, {} on nothing; the base lost {aircraft_lost} \
         aircraft and {supply_lost} t of supply",
        hits - on_aircraft - on_supply - on_men
    );
    // The events are consistent: what the base lost is what the hits landed.
    assert_eq!(on_men, 0, "no men ashore at base 1");
    assert_eq!(on_aircraft == 0, aircraft_lost == 0);
    assert!(aircraft_lost <= on_aircraft);
    assert_eq!(on_supply == 0, supply_lost == Fixed::ZERO);
    let group = &run.at(2).air_groups[&sim_core::ids::AirGroupId::new(1)];
    assert_eq!(group.ready, 20 - aircraft_lost);
    assert_eq!(run.at(2).bases[&base(1)].supply_t, n(1000) - supply_lost);
    for e in run.all(|k| {
        matches!(
            k,
            EventKind::AircraftDestroyedOnGround { .. } | EventKind::BaseSupplyDestroyed { .. }
        )
    }) {
        assert!(
            e.causes.contains(&Cause::Event(bombardment.id)),
            "the loss cites the bombardment"
        );
    }

    // Day 1, morning: it has bombarded, so it steams on, with the pulse's 6
    // hours: 96 nm toward (4,0), two hexes and 16 nm, to (2,0).
    let on = run.one(
        1,
        Pulse::Morning,
        |k| matches!(k, EventKind::TaskForceSteamedOn { task_force, .. } if task_force.raw() == 1),
    );
    assert_eq!(steam_on_of(on), (X, BOMBARD, true));
    assert_eq!(
        on.causes,
        vec![
            Cause::Order(route_1),
            Cause::Event(halt.id),
            Cause::Event(bombardment.id)
        ]
    );
    assert_eq!(run.steamed_on(1).len(), 1);
    let morning = run.one(
        1,
        Pulse::Morning,
        |k| matches!(k, EventKind::TaskForceSteamed { task_force, .. } if task_force.raw() == 1),
    );
    assert_eq!(leg(morning), (X, Hex::new(2, 0), n(96), n(16)));
    assert_eq!(fuel(&run, 1, Pulse::Morning, 1), (n(24), n(1936)));

    // Day 1, afternoon: the last 64 nm take 4 hours. It arrives at (4,0) at
    // hour 22.
    let afternoon = run.one(
        1,
        Pulse::Afternoon,
        |k| matches!(k, EventKind::TaskForceSteamed { task_force, .. } if task_force.raw() == 1),
    );
    assert_eq!(leg(afternoon), (Hex::new(2, 0), SEA, n(64), n(16)));
    assert_eq!(fuel(&run, 1, Pulse::Afternoon, 1), (n(16), n(1920)));
    assert_eq!(run.arrived(1).len(), 1);
    assert_eq!(arrival_of(run.arrived(1)[0]), (SEA, n(22)));
    assert_eq!(run.arrived(1)[0].pulse, Some(Pulse::Afternoon));

    // Fuel: 320 nm at 0.25 t is 80 t, and 1920 t is left. On the night of day
    // 2 it is at (4,0), where there is no enemy base, and does not bombard:
    // it ends with 80 rounds.
    assert_eq!(run.bombardments().len(), 1);
    assert!(run.events.iter().all(|e| e.day == 1
        || !matches!(
            e.kind,
            EventKind::BombardmentFire { .. } | EventKind::ShoreBombardment { .. }
        )));
    assert!(run.steamed(1).iter().all(|e| e.day == 1));
    assert_eq!(run.position(2, 1), SEA);
    assert_eq!(run.status(2, 1), TaskForceStatus::Arrived);
    assert_eq!(run.at(2).ships[&ship(1)].fuel_t, n(1920));
    assert_eq!(run.at(2).ships[&ship(1)].gun_rounds, n(80));

    // In no daylight pulse is its position (0,0): it is at (0,0) only at the
    // end of the night, and ends the morning at (2,0) and the afternoon at (4,0).
    for e in run.steamed(1) {
        if matches!(e.pulse, Some(Pulse::Morning | Pulse::Afternoon)) {
            assert_ne!(leg(e).1, X, "{e:?}");
        }
    }
    assert_eq!(run.halted(1)[0].pulse, Some(Pulse::Night));

    // ---- Task force 2: a supply run.
    let route_2 = run.route_order(2);
    // Day 1: 4 hexes, 160 nm, at 10 kn is 16 hours: 120 nm in the night pulse
    // and the last 40 nm in the first 4 hours of the morning.
    let night = run.one(
        1,
        Pulse::Night,
        |k| matches!(k, EventKind::TaskForceSteamed { task_force, .. } if task_force.raw() == 2),
    );
    assert_eq!(leg(night), (Z, Hex::new(1, 3), n(120), n(10)));
    assert_eq!(fuel(&run, 1, Pulse::Night, 2), (n(12), n(988)));
    let morning = run.one(
        1,
        Pulse::Morning,
        |k| matches!(k, EventKind::TaskForceSteamed { task_force, .. } if task_force.raw() == 2),
    );
    assert_eq!(leg(morning), (Hex::new(1, 3), Y, n(40), n(10)));
    assert_eq!(fuel(&run, 1, Pulse::Morning, 2), (n(4), n(984)));
    // It reaches (0,3) at hour 16 and halts, to work cargo.
    let halt_2 = run.one(
        1,
        Pulse::Morning,
        |k| matches!(k, EventKind::TaskForceHalted { task_force, .. } if task_force.raw() == 2),
    );
    assert_eq!(halt_of(halt_2), (Y, CARGO, n(16)));
    assert_eq!(
        halt_2.causes,
        vec![Cause::Order(route_2), Cause::Event(morning.id)]
    );
    assert_eq!(run.halted(2).len(), 1);

    // Unloading: 2 hours in the morning at 20 t, then the rest.
    let unloaded = |day, pulse| {
        let e = run.one(
            day,
            pulse,
            |k| matches!(k, EventKind::CargoUnloaded { ship, .. } if ship.raw() == 2),
        );
        match e.kind {
            EventKind::CargoUnloaded {
                base: at,
                unloaded_t,
                aboard_t,
                base_stock_t,
                ..
            } => (at, unloaded_t, aboard_t, base_stock_t),
            _ => unreachable!(),
        }
    };
    assert_eq!(unloaded(1, Pulse::Morning), (base(2), n(40), n(60), n(40)));
    assert_eq!(
        unloaded(1, Pulse::Afternoon),
        (base(2), n(60), n(0), n(100))
    );
    let complete = run.one(
        1,
        Pulse::Afternoon,
        |k| matches!(k, EventKind::UnloadingComplete { task_force, .. } if task_force.raw() == 2),
    );
    match complete.kind {
        EventKind::UnloadingComplete {
            base: at,
            elapsed_hours,
            ..
        } => assert_eq!((at, elapsed_hours), (base(2), n(21))),
        _ => unreachable!(),
    }
    assert_eq!(run.status(1, 2), TaskForceStatus::Halted);
    assert_eq!(run.position(1, 2), Y);

    // Day 2, night: its cargo work is complete, so it steams on: 120 nm.
    // Morning: the last 40 nm. It arrives at (4,3) at hour 40.
    let on = run.one(
        2,
        Pulse::Night,
        |k| matches!(k, EventKind::TaskForceSteamedOn { task_force, .. } if task_force.raw() == 2),
    );
    assert_eq!(steam_on_of(on), (Y, CARGO, false));
    assert_eq!(
        on.causes,
        vec![
            Cause::Order(route_2),
            Cause::Event(halt_2.id),
            Cause::Event(complete.id)
        ]
    );
    assert_eq!(run.steamed_on(2).len(), 1);
    let night = run.one(
        2,
        Pulse::Night,
        |k| matches!(k, EventKind::TaskForceSteamed { task_force, .. } if task_force.raw() == 2),
    );
    assert_eq!(leg(night), (Y, Hex::new(3, 3), n(120), n(10)));
    assert_eq!(fuel(&run, 2, Pulse::Night, 2), (n(12), n(972)));
    let morning = run.one(
        2,
        Pulse::Morning,
        |k| matches!(k, EventKind::TaskForceSteamed { task_force, .. } if task_force.raw() == 2),
    );
    assert_eq!(leg(morning), (Hex::new(3, 3), Z, n(40), n(10)));
    assert_eq!(fuel(&run, 2, Pulse::Morning, 2), (n(4), n(968)));
    assert_eq!(run.arrived(2).len(), 1);
    assert_eq!(arrival_of(run.arrived(2)[0]), (Z, n(40)));
    assert_eq!(run.arrived(2)[0].day, 2);
    assert_eq!(run.arrived(2)[0].pulse, Some(Pulse::Morning));
    assert_eq!(run.steamed(2).len(), 4);
    // Fuel: 320 nm at 0.1 t is 32 t, so 968 t is left. Base 2 holds 100 t and
    // base 3 holds 0.
    assert_eq!(run.at(2).ships[&ship(2)].fuel_t, n(968));
    assert_eq!(run.at(2).ships[&ship(2)].supply_t, n(0));
    assert_eq!(run.at(2).bases[&base(2)].supply_t, n(100));
    assert_eq!(run.at(2).bases[&base(3)].supply_t, n(0));
    assert_eq!(run.position(2, 2), Z);
    assert_eq!(run.status(2, 2), TaskForceStatus::Arrived);

    // ---- Task force 3: nothing to do. The waypoint (2,5) is marked, but the
    // force has no work there: no halt. 160 nm at 16 kn is 10 hours: it
    // arrives at (4,5) at hour 10, with no halt recorded.
    assert!(run.halted(3).is_empty());
    assert!(run.steamed_on(3).is_empty());
    let night = run.one(
        1,
        Pulse::Night,
        |k| matches!(k, EventKind::TaskForceSteamed { task_force, .. } if task_force.raw() == 3),
    );
    assert_eq!(leg(night), (Hex::new(0, 5), HOME_3, n(160), n(16)));
    // Its route passed the marked waypoint.
    match &night.kind {
        EventKind::TaskForceSteamed { hexes_entered, .. } => {
            assert!(hexes_entered.contains(&PASSED))
        }
        _ => unreachable!(),
    }
    assert_eq!(run.steamed(3).len(), 1);
    assert_eq!(fuel(&run, 1, Pulse::Night, 3), (n(20), n(480)));
    assert_eq!(run.arrived(3).len(), 1);
    assert_eq!(arrival_of(run.arrived(3)[0]), (HOME_3, n(10)));
    assert_eq!(run.arrived(3)[0].pulse, Some(Pulse::Night));
    assert_eq!(run.at(2).ships[&ship(3)].fuel_t, n(480));
    assert_eq!(run.status(1, 3), TaskForceStatus::Arrived);

    // Only the two forces with work halt, and only once each.
    assert_eq!(
        run.all(|k| matches!(k, EventKind::TaskForceHalted { .. }))
            .len(),
        2
    );
    assert_eq!(
        run.all(|k| matches!(k, EventKind::TaskForceSteamedOn { .. }))
            .len(),
        2
    );
}

// ---------------------------------------------------------------- Properties

/// The scenario files of the repository other than scenario 26.
fn other_scenarios() -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(common::data_dir().join("scenarios"))
        .expect("the scenarios folder reads")
        .map(|entry| entry.expect("an entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "toml"))
        .filter(|path| {
            !path
                .file_name()
                .expect("a name")
                .to_string_lossy()
                .starts_with("26_")
        })
        .collect();
    files.sort();
    files
}

/// Property: a route with no mark gives the movement it gave before. In
/// every other scenario with no `act_at` in its file no route has a mark, so
/// the run records no halt, no steaming on, and no force is ever halted. The
/// full slice marks its destroyer runs' landing hexes from decision 0101 and
/// is left out by that test. (The run baselines of
/// `sim-cli`'s golden-file test cover that the movement is the same as it
/// was.)
#[test]
fn property_in_no_other_scenario_does_a_force_halt_or_steam_on() {
    let files = other_scenarios();
    assert!(files.len() > 40, "{} scenario files", files.len());
    for file in files {
        let source = std::fs::read_to_string(&file).expect("the file reads");
        if source.contains("act_at") {
            continue;
        }
        let mut scenario = common::load_live(&file)
            .unwrap_or_else(|error| panic!("{} loads: {error}", file.display()));
        loop {
            let more = scenario.next_day().expect("a day resolves").is_some();
            assert!(
                scenario
                    .world
                    .umpire()
                    .task_forces
                    .values()
                    .all(|force| force.status != TaskForceStatus::Halted && force.halt.is_none()),
                "{}: a force is halted",
                file.display()
            );
            if !more {
                break;
            }
        }
        assert!(
            scenario.world.events().iter().all(|e| !matches!(
                e.kind,
                EventKind::TaskForceHalted { .. } | EventKind::TaskForceSteamedOn { .. }
            )),
            "{}: a halt or a steaming on",
            file.display()
        );
    }
}

/// Property: a mark with no work behind it changes no movement. Scenario 26
/// with every mark removed gives task force 3, whose mark is on a waypoint
/// where it has no work, exactly the same movement.
#[test]
fn property_a_mark_where_the_force_has_no_work_changes_no_movement() {
    let marked = Run::text(&text());
    let source = [MARK_1, MARK_2, MARK_3]
        .iter()
        .fold(text(), |source, mark| {
            common::replace_once(&source, mark, "")
        });
    assert!(!source.contains("act_at"));
    let bare = Run::text(&source);
    // The marks were the only difference: the bare run has no halt at all.
    assert!(bare
        .all(|k| matches!(
            k,
            EventKind::TaskForceHalted { .. } | EventKind::TaskForceSteamedOn { .. }
        ))
        .is_empty());
    let with_mark = movement_of(&marked, 3, &[3]);
    assert_eq!(with_mark.len(), 3, "steamed, burned, arrived");
    assert_eq!(with_mark, movement_of(&bare, 3, &[3]));
    // And without the marks the other two forces steam straight on.
    assert!(bare.arrived(1).len() == 1 && bare.arrived(2).len() == 1);
    assert_eq!(bare.position(1, 1), SEA);
    assert!(bare.bombardments().is_empty());
}

/// Whether a run ever has a halted force move in a pulse in which its work
/// is not done: over its log, a force that is halted at the start of a pulse
/// and has no `TaskForceSteamedOn` in it has no `TaskForceSteamed` in it.
/// The halted set read from the log is checked against the truth at the end
/// of each day.
fn check_halted_never_moves_unworked(name: &str, run: &Run) {
    let truth_halted = |day: u32| -> BTreeSet<TaskForceId> {
        run.at(day)
            .task_forces
            .values()
            .filter(|force| force.status == TaskForceStatus::Halted)
            .map(|force| force.id)
            .collect()
    };
    let mut halted: BTreeSet<TaskForceId> = BTreeSet::new();
    let mut at_start: BTreeSet<TaskForceId> = BTreeSet::new();
    let mut stood_on: BTreeSet<TaskForceId> = BTreeSet::new();
    let mut key: (u32, Option<Pulse>) = (1, None);
    for e in &run.events {
        if e.day != key.0 {
            assert_eq!(
                halted,
                truth_halted(key.0),
                "{name}: halted at the end of day {}",
                key.0
            );
        }
        if (e.day, e.pulse) != key && e.pulse.is_some() {
            at_start = halted.clone();
            stood_on.clear();
        }
        if e.pulse.is_some() || e.day != key.0 {
            key = (e.day, e.pulse);
        }
        match &e.kind {
            EventKind::OrderAccepted {
                order,
                target: sim_core::ids::EntityRef::TaskForce(id),
            } => {
                if matches!(
                    run.orders[order],
                    OrderKind::TaskForce {
                        order: TaskForceOrder::SetRoute { .. },
                        ..
                    }
                ) {
                    halted.remove(id);
                }
            }
            EventKind::TaskForceSteamedOn { task_force, .. } => {
                stood_on.insert(*task_force);
                halted.remove(task_force);
            }
            EventKind::TaskForceSteamed { task_force, .. } => {
                assert!(
                    !at_start.contains(task_force) || stood_on.contains(task_force),
                    "{name}: {task_force:?} moved on day {} in {:?} while halted with work to do",
                    e.day,
                    e.pulse
                );
            }
            EventKind::TaskForceHalted { task_force, .. } => {
                halted.insert(*task_force);
            }
            _ => {}
        }
    }
    assert_eq!(halted, truth_halted(run.len()), "{name}: halted at the end");
}

/// Property: a halted force never moves in a pulse in which its work is not
/// done. Over the log of every scenario 26 run in this file.
#[test]
fn property_a_halted_force_never_moves_in_a_pulse_in_which_its_work_is_not_done() {
    let runs = all_runs();
    let mut halts = 0;
    for (name, run) in &runs {
        check_halted_never_moves_unworked(name, run);
        halts += run
            .all(|k| matches!(k, EventKind::TaskForceHalted { .. }))
            .len();
    }
    // The property was not vacuous: halts were made and left.
    assert!(halts >= 8, "{halts} halts over {} runs", runs.len());
}

/// Property: the enemy's view is the same in two worlds that differ only in
/// a mark on a waypoint where the force has no work. Scenario 26 with and
/// without task force 3's mark: the Japanese side's view (the events it is
/// given, as its view gives them, and its contacts) is the same on every day.
#[test]
fn property_the_enemys_view_is_the_same_with_and_without_a_mark_where_the_force_has_no_work() {
    let marked = Run::text(&text());
    let unmarked = Run::text(&common::replace_once(&text(), MARK_3, ""));
    // The worlds differ in the order only.
    let marks = |run: &Run, raw: u32| -> usize {
        match &run.orders[&run.route_order(raw)] {
            OrderKind::TaskForce {
                order: TaskForceOrder::SetRoute { act_at, .. },
                ..
            } => act_at.len(),
            other => panic!("not a route order: {other:?}"),
        }
    };
    assert_eq!(marks(&marked, 3), 1);
    assert_eq!(marks(&unmarked, 3), 0);
    assert_eq!(marked.len(), unmarked.len());
    assert!(marked.halted(3).is_empty() && unmarked.halted(3).is_empty());
    for day in 0..=marked.len() {
        assert_eq!(
            marked.japanese[day as usize], unmarked.japanese[day as usize],
            "day {day}"
        );
    }
    // The Japanese side sees something: its own raid.
    assert!(marked.japanese[1].contains("Halted") || marked.japanese[1].contains("halted"));
}

// ------------------------------------------------------------ The extra cases

/// The raid with task force 1 at 10 knots, to arrive by day, and `days` days.
fn slow_raid(days: u32) -> String {
    let source = common::replace_once(
        &text(),
        SPEED_1,
        "[[order]]\nday = 1\nside = \"japanese\"\ntask_force = 1\nkind = \"set_speed\"\nknots = \"10\"\n",
    );
    common::replace_once(&source, "days = 2", &format!("days = {days}"))
}

/// The team's own. A force that halts by day to bombard waits and fires that
/// night. Task force 1 at 10 knots: 160 nm is 16 hours, so it reaches (0,0)
/// at hour 16, 10:00 on day 1, in the morning pulse, and halts. It lies there
/// through the afternoon, does not move, bombards in the night shore
/// bombardment phase of day 2 and steams on in the morning of day 2, at 6
/// hours of 10 kn, 60 nm, which takes it past the first hex.
#[test]
fn extra_a_force_that_halts_by_day_to_bombard_waits_and_fires_that_night() {
    let run = Run::text(&slow_raid(2));
    let route = run.route_order(1);
    let halt = run.halted(1);
    assert_eq!(halt.len(), 1);
    assert_eq!(halt[0].day, 1);
    assert_eq!(halt[0].pulse, Some(Pulse::Morning));
    assert_eq!(halt_of(halt[0]), (X, BOMBARD, n(16)));
    // Halted through the afternoon: no movement and no fire.
    assert!(run
        .on(1, Pulse::Afternoon, |k| matches!(
            k,
            EventKind::TaskForceSteamed { task_force, .. } if task_force.raw() == 1
        ))
        .is_empty());
    assert!(run
        .on(1, Pulse::Afternoon, |k| matches!(
            k,
            EventKind::BombardmentFire { .. } | EventKind::ShoreBombardment { .. }
        ))
        .is_empty());
    assert_eq!(run.status(1, 1), TaskForceStatus::Halted);
    assert_eq!(run.position(1, 1), X);
    assert!(run.steamed_on(1).is_empty() || run.steamed_on(1)[0].day == 2);
    // On day 2 night it does not move before it has fired, and it fires.
    assert!(run
        .on(2, Pulse::Night, |k| matches!(
            k,
            EventKind::TaskForceSteamed { task_force, .. } if task_force.raw() == 1
        ))
        .is_empty());
    let bombardment = run.one(2, Pulse::Night, |k| {
        matches!(k, EventKind::ShoreBombardment { .. })
    });
    assert_eq!(run.bombardments().len(), 1);
    assert_eq!(run.status(1, 1), TaskForceStatus::Halted);
    // Then it steams on in the morning of day 2 and is gone from (0,0).
    let on = run.one(
        2,
        Pulse::Morning,
        |k| matches!(k, EventKind::TaskForceSteamedOn { task_force, .. } if task_force.raw() == 1),
    );
    assert_eq!(steam_on_of(on), (X, BOMBARD, true));
    assert_eq!(
        on.causes,
        vec![
            Cause::Order(route),
            Cause::Event(halt[0].id),
            Cause::Event(bombardment.id)
        ]
    );
    assert_eq!(run.steamed_on(1).len(), 1);
    let moved = run.one(
        2,
        Pulse::Morning,
        |k| matches!(k, EventKind::TaskForceSteamed { task_force, .. } if task_force.raw() == 1),
    );
    assert_eq!(leg(moved).0, X);
    assert_ne!(leg(moved).1, X);
    assert_ne!(run.position(2, 1), X);
    assert_eq!(run.status(2, 1), TaskForceStatus::Underway);
}

/// The team's own. A new route order ends a halt. The slow raid, with a day 2
/// route order for task force 1 back to (4,0): the order is taken in at the
/// start of day 2, the halt ends, it leaves at the start of day 2 night
/// without bombarding (120 nm in the night, and the last 40 nm in 4 hours of
/// the morning of day 2), and it never fires. No `TaskForceSteamedOn` is
/// recorded for it, as the new order, not the work, ended the halt.
#[test]
fn extra_a_new_route_order_ends_a_halt() {
    let source = format!(
        "{}\n[[order]]\nday = 2\nside = \"japanese\"\ntask_force = 1\nkind = \"set_route\"\nwaypoints = [[4, 0]]\n",
        slow_raid(2)
    );
    let run = Run::text(&source);
    assert_eq!(run.halted(1).len(), 1);
    assert_eq!(run.halted(1)[0].pulse, Some(Pulse::Morning));
    assert_eq!(run.status(1, 1), TaskForceStatus::Halted);
    assert!(run.at(1).task_forces[&tf(1)].halt.is_some());
    // The order ended the halt: it is under way at once, and leaves in the
    // night pulse, from (0,0).
    let night = run.one(
        2,
        Pulse::Night,
        |k| matches!(k, EventKind::TaskForceSteamed { task_force, .. } if task_force.raw() == 1),
    );
    assert_eq!(leg(night), (X, Hex::new(3, 0), n(120), n(10)));
    assert!(run.steamed_on(1).is_empty());
    assert!(run.bombardments().is_empty());
    assert!(run
        .all(|k| matches!(k, EventKind::BombardmentFire { .. }))
        .is_empty());
    assert_eq!(run.halted(1).len(), 1, "it halts once only");
    // It arrives at (4,0) in the morning of day 2.
    assert_eq!(run.arrived(1).len(), 1);
    assert_eq!(run.arrived(1)[0].day, 2);
    assert_eq!(run.arrived(1)[0].pulse, Some(Pulse::Morning));
    assert_eq!(arrival_of(run.arrived(1)[0]).0, SEA);
    assert_eq!(run.position(2, 1), SEA);
    assert_eq!(run.status(2, 1), TaskForceStatus::Arrived);
    assert!(run.at(2).task_forces[&tf(1)].halt.is_none());
}

/// The team's own. A force with both a bombardment and cargo to work does
/// both before it goes. Task force 1 also carries a placeholder cargo ship
/// (ship 4, 100 t of supply, 10 kn) and has an unload order, and Japanese
/// base 4 stands at (0,0) in the hex of Allied base 1 (two sides' bases in
/// one hex). The force goes at the cargo ship's 10 kn: 160 nm is 16 hours,
/// so it reaches (0,0) at hour 16 in the morning of day 1 and halts for both.
/// At 20 t an hour or faster the 100 t are unloaded in the first pulses (the
/// cargo is done in the morning or afternoon of day 1), but the bombardment
/// waits for night, so it is the bombardment that is last: it fires in the
/// night of day 2 and steams on in the morning of day 2, with `work` showing
/// both and `fired` true, citing both the bombardment and the unloading's
/// completion. It then goes on to (4,0): 160 nm at 10 kn is 16 hours, so it
/// arrives in the night of day 3.
#[test]
fn extra_a_force_with_a_bombardment_and_cargo_does_both_before_it_goes() {
    let source = common::replace_once(&text(), "days = 2", "days = 3");
    let source = format!(
        "{source}\n\
[[base]]\nid = 4\nname = \"Placeholder base X, Japanese\"\nside = \"japanese\"\nq = 0\nr = 0\nsupply_t = 0\n\n\
[[ship]]\nid = 4\nname = \"Placeholder cargo ship, Japanese\"\nclass = 3\nside = \"japanese\"\nfuel_t = 1000\nsupply_t = 100\ntask_force = 1\n\n\
[[order]]\nday = 1\nside = \"japanese\"\ntask_force = 1\nkind = \"unload\"\n"
    );
    let run = Run::text(&source);
    let route = run.route_order(1);
    let halt = run.halted(1);
    assert_eq!(halt.len(), 1);
    assert_eq!(halt[0].day, 1);
    assert_eq!(halt[0].pulse, Some(Pulse::Morning));
    assert_eq!(halt_of(halt[0]), (X, BOTH, n(16)));
    // The unloading is complete before the bombardment can be fired.
    let complete = run.all(
        |k| matches!(k, EventKind::UnloadingComplete { task_force, .. } if task_force.raw() == 1),
    );
    assert_eq!(complete.len(), 1);
    assert_eq!(complete[0].day, 1);
    assert_eq!(run.at(1).bases[&base(4)].supply_t, n(100));
    assert_eq!(run.at(1).ships[&ship(4)].supply_t, n(0));
    assert_eq!(run.status(1, 1), TaskForceStatus::Halted);
    assert_eq!(run.position(1, 1), X);
    // It stays: no movement and no steaming on until it has fired.
    assert!(run
        .steamed(1)
        .iter()
        .all(|e| (e.day, e.pulse) != (1, Some(Pulse::Afternoon))
            && (e.day, e.pulse) != (2, Some(Pulse::Night))));
    // It fires in the night of day 2, and steams on that morning.
    let bombardment = run.one(2, Pulse::Night, |k| {
        matches!(k, EventKind::ShoreBombardment { .. })
    });
    assert_eq!(run.bombardments().len(), 1);
    assert!(run
        .on(2, Pulse::Night, |k| matches!(
            k,
            EventKind::TaskForceSteamed { task_force, .. } if task_force.raw() == 1
        ))
        .is_empty());
    let on = run.one(
        2,
        Pulse::Morning,
        |k| matches!(k, EventKind::TaskForceSteamedOn { task_force, .. } if task_force.raw() == 1),
    );
    assert_eq!(steam_on_of(on), (X, BOTH, true));
    assert_eq!(
        on.causes,
        vec![
            Cause::Order(route),
            Cause::Event(halt[0].id),
            Cause::Event(bombardment.id),
            Cause::Event(complete[0].id)
        ]
    );
    assert_eq!(run.steamed_on(1).len(), 1);
    assert_ne!(run.position(2, 1), X);
    assert_eq!(run.position(3, 1), SEA);
    assert_eq!(run.arrived(1)[0].day, 3);
    assert_eq!(run.arrived(1)[0].pulse, Some(Pulse::Night));
}

/// The team's own. A marked last waypoint behaves as an unmarked one. Task
/// force 1 and 2 with their last waypoints marked as well as the waypoint
/// they act at: the events are the same, event for event, as in scenario 26,
/// and neither halts at its last waypoint, (4,0) and (4,3), but arrives there.
#[test]
fn extra_a_marked_last_waypoint_behaves_as_an_unmarked_one() {
    let marked = common::replace_once(&text(), MARK_1, "act_at = [[0, 0], [4, 0]]\n");
    let marked = common::replace_once(&marked, MARK_2, "act_at = [[0, 3], [4, 3]]\n");
    let run = Run::text(&marked);
    let plain = Run::text(&text());
    assert_eq!(digest(&run), digest(&plain));
    for e in run.all(|k| matches!(k, EventKind::TaskForceHalted { .. })) {
        assert!([X, Y].contains(&halt_of(e).0), "{e:?}");
    }
    assert_eq!(run.halted(1).len(), 1);
    assert_eq!(run.halted(2).len(), 1);
    assert_eq!(arrival_of(run.arrived(1)[0]), (SEA, n(22)));
    assert_eq!(arrival_of(run.arrived(2)[0]), (Z, n(40)));
    assert_eq!(run.position(2, 1), SEA);
    assert_eq!(run.position(2, 2), Z);
}

/// The team's own. A mark on the last waypoint and no other: with the only
/// mark on (4,0) for task force 1 and (4,3) for task force 2, no force halts,
/// the cruiser passes (0,0) and fires nothing, and the events are the same as
/// with no mark at all.
#[test]
fn extra_a_mark_on_the_last_waypoint_alone_changes_nothing() {
    let last_only = common::replace_once(&text(), MARK_1, "act_at = [[4, 0]]\n");
    let last_only = common::replace_once(&last_only, MARK_2, "act_at = [[4, 3]]\n");
    let last_only = Run::text(&last_only);
    let source = [MARK_1, MARK_2].iter().fold(text(), |source, mark| {
        common::replace_once(&source, mark, "")
    });
    let bare = Run::text(&source);
    assert_eq!(digest(&last_only), digest(&bare));
    assert!(last_only
        .all(|k| matches!(
            k,
            EventKind::TaskForceHalted { .. } | EventKind::TaskForceSteamedOn { .. }
        ))
        .is_empty());
    assert!(last_only.bombardments().is_empty());
    assert_eq!(last_only.arrived(1).len(), 1);
    assert_eq!(last_only.arrived(2).len(), 1);
    // The cruiser steams straight through (0,0) and back: still with its
    // 800 rounds.
    assert_eq!(last_only.at(2).ships[&ship(1)].gun_rounds, n(800));
    assert_eq!(last_only.position(2, 1), SEA);
}

/// Every scenario 26 run in this file: the file itself and each variant.
fn all_runs() -> Vec<(&'static str, Run)> {
    let bare = [MARK_1, MARK_2, MARK_3]
        .iter()
        .fold(text(), |source, mark| {
            common::replace_once(&source, mark, "")
        });
    let mut runs = vec![
        ("scenario 26", Run::text(&text())),
        ("no marks", Run::text(&bare)),
        (
            "task force 3 unmarked",
            Run::text(&common::replace_once(&text(), MARK_3, "")),
        ),
        ("slow raid", Run::text(&slow_raid(3))),
        (
            "slow raid recalled",
            Run::text(&format!(
                "{}\n[[order]]\nday = 2\nside = \"japanese\"\ntask_force = 1\nkind = \"set_route\"\nwaypoints = [[4, 0]]\n",
                slow_raid(2)
            )),
        ),
        (
            "last waypoints marked",
            Run::text(&common::replace_once(
                &text(),
                MARK_1,
                "act_at = [[0, 0], [4, 0]]\n",
            )),
        ),
    ];
    // Task force 1 on a longer game, to fire again and again: halted, then
    // sent back to the base on day 2 with a route of its own.
    runs.push((
        "back to the base",
        Run::text(&format!(
            "{}\n[[order]]\nday = 2\nside = \"japanese\"\ntask_force = 1\nkind = \"set_route\"\nwaypoints = [[0, 0], [4, 0]]\nact_at = [[0, 0]]\n",
            text()
        )),
    ));
    runs
}
