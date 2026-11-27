//! Scenario 44: refuelling that takes time. A ship in port takes fuel
//! through the hours of the day at the port's fuelling rate, at a berth or
//! by lighter, in the cargo rule's queue. Two forces share one berth, one
//! of them loading; a cruiser at a port with no berth sails short and
//! finishes fuelling where she arrives; and a reserve holds a destroyer
//! short.
//!
//! Proves decision 0112 against a hand calculation. The data is placeholder
//! data: see `data/scenarios/44_fuelling_takes_time.toml`. It runs on the
//! frozen step 1 figures, with the fuelling rates set in the scenario's own
//! port sizes, since the frozen figures give none (no limit). There is no
//! enemy, no search and no random draw.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Figures
//!
//! | Port size | Berths | Cargo at a berth | Cargo by lighter | Fuel at a berth | Fuel by lighter |
//! | --------- | ------ | ---------------- | ---------------- | --------------- | --------------- |
//! | 0         | 0      | none             | 20 t an hour     | none            | 40 t an hour    |
//! | 1         | 1      | 50 t an hour     | 20 t an hour     | 100 t an hour   | 40 t an hour    |
//!
//! The placeholder cargo ship (class 3): 3000 t of cargo, 1000 t of fuel.
//! The placeholder cruiser (class 1): 2000 t of fuel over 8000 nm, which is
//! 0.25 t a mile, 16 kn at cruise. The placeholder destroyer (class 2): 500
//! t of fuel. A day is a night pulse of 12 hours and two daylight pulses of
//! 6. In a pulse ships move first; then berths are given out, to forces in
//! number order, to a ship that has work to do, cargo to move or fuel to
//! take; then the work is done. A ship with no berth works by lighter. A
//! ship does its cargo first and then takes fuel in the hours of the pulse
//! that are left. A berth given in a pulse is that ship's for the pulse.
//!
//! ## Inputs
//!
//! A placeholder map: S at (0,0), a coast hex with port size 1; U at (5,0),
//! 200 nm from S, and V at (0,5), coast hexes with port size 0. The game is
//! 2 days. Everything is Allied.
//!
//! | Base | Hex | Supply | Fuel   | Fuel reserve |
//! | ---- | --- | ------ | ------ | ------------ |
//! | 1    | S   | 2000 t | 5000 t | 0            |
//! | 2    | U   | 0      | 3000 t | 0            |
//! | 3    | V   | 0      | 300 t  | 100 t        |
//!
//! | Force | Ship                     | Fuel aboard | Where             |
//! | ----- | ------------------------ | ----------- | ----------------- |
//! | 1     | 1, cargo ship            | 600 t       | stopped at base 1 |
//! | 2     | 2, cruiser               | 800 t       | stopped at base 1 |
//! | 3     | 3, cruiser               | 500 t       | stopped at base 2 |
//! | 4     | 4, destroyer             | 0           | stopped at base 3 |
//!
//! Orders: on day 1 force 1 loads supply up to 500 t. On day 2 force 3 is
//! routed to S.
//!
//! ## At S: one berth, cargo and fuel
//!
//! Ship 1 lacks 400 t of fuel and ship 2 lacks 1200 t.
//!
//! | Pulse              | Force 1, ship 1                                   | Force 2, ship 2                         | Base 1's fuel after |
//! | ------------------ | ------------------------------------------------- | --------------------------------------- | ------------------- |
//! | Day 1, night       | Has the berth. Loads 500 t in 10 hours, then takes 2 x 100 = 200 t of fuel | By lighter: 12 x 40 = 480 t | 4320 t |
//! | Day 1, morning     | Has the berth. Takes its last 200 t in 2 hours: full at hour 14 | By lighter: 6 x 40 = 240 t | 3880 t |
//! | Day 1, afternoon   | Nothing to do                                     | Has the berth. Takes its last 480 t in 4.8 hours: full at hour 22.8 | 3400 t |
//!
//! Base 1 holds 1500 t of supply from the night of day 1. The load is
//! complete at hour 10. At no start of a day is any ship refuelled: the
//! morning's refuelling is gone.
//!
//! ## At U: no berth, and sailing short
//!
//! Ship 3 lacks 1500 t. On day 1 she fuels by lighter in all three pulses:
//! 24 x 40 = 960 t. She has 1460 t at the end of day 1 and base 2 holds
//! 2040 t.
//!
//! Asked before day 2 is resolved, the list of forces says force 3 lacks
//! 540 t of fuel and that at base 2 it would take 13.5 hours.
//!
//! On day 2 she is ordered to S and sails at hour 24 with 1460 t: the order
//! is not held back. She steams 200 nm in 12.5 hours, burns 50 t and
//! arrives at S at hour 36.5 with 1410 t. She lacks 590 t.
//!
//! | Pulse            | Force 3, ship 3 at S                                             | Base 1's fuel after |
//! | ---------------- | ---------------------------------------------------------------- | ------------------- |
//! | Day 2, morning   | Arrives half an hour into the pulse. No other ship has work: she has the berth for the 5.5 hours left and takes 550 t | 2850 t |
//! | Day 2, afternoon | Has the berth. Takes her last 40 t in 0.4 hours: full at hour 42.4 | 2810 t             |
//!
//! ## At V: a reserve
//!
//! Base 3 holds 300 t of fuel and may give 200 t, down to its reserve of
//! 100. Ship 4 lacks 500 t. In the night of day 1 she takes 200 t by
//! lighter in 5 hours and base 3 is at its reserve. She is held at the
//! reserve, 300 t short: that is recorded once, in the night of day 1, and
//! not again in any later pulse while nothing changes.
//!
//! ## At the end of day 2
//!
//! | Ship | Fuel   | Where |
//! | ---- | ------ | ----- |
//! | 1    | 1000 t | at base 1, with 500 t of supply aboard |
//! | 2    | 2000 t | at base 1 |
//! | 3    | 2000 t | at base 1 |
//! | 4    | 200 t  | at base 3 |
//!
//! Base 1 holds 1500 t of supply and 2810 t of fuel; base 2 holds 2040 t of
//! fuel; base 3 holds 100 t of fuel.
//!
//! # Properties
//!
//! - **With no fuelling rate given,** as on the frozen figures, a ship in
//!   port takes all it lacks in the first pulse it lies there, and every
//!   scenario's stocks and fuel at the end of each day are what they were
//!   under the morning's refuelling, apart from a force that begins a game
//!   in port short of fuel and sails on day 1, which now sails as it began.
//! - Fuel is conserved: what the ships take is what the bases give, in
//!   every pulse.
//! - No ship takes more than the hours of a pulse at its rate, and none
//!   takes fuel in a pulse in which it moved for the whole of it.
//! - Refuelling never takes a base's fuel below its reserve.
//! - A force that ran out of fuel at a base's hex is stopped and idle again
//!   in the first pulse in which a ship of it takes fuel.
//! - With force 2 given the lower number, ship 2 has the berth in the night
//!   of day 1 and takes 1200 t in 12 hours; ship 1 loads 240 t by lighter
//!   and takes no fuel that pulse.

use crate::common;

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::state::{TaskForceStatus, TruthState};
use sim_core::time::{pulse_start, Pulse};
use sim_core::world::SimConfig;
use sim_data::Scenario;

// ------------------------------------------------------------------ Helpers

fn path() -> PathBuf {
    common::data_dir().join("scenarios/44_fuelling_takes_time.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

/// A decimal figure of the hand calculation, such as `"22.8"`.
fn d(text: &str) -> Fixed {
    text.parse().expect("a decimal figure")
}

struct Run {
    /// The truth at the start (index 0) and at the end of each day.
    days: Vec<TruthState>,
    events: Vec<Event>,
}

impl Run {
    /// Runs the whole game with breakdowns recorded, which changes nothing
    /// that happens.
    fn of(mut scenario: Scenario) -> Run {
        scenario.world.set_config(SimConfig {
            record_breakdowns: true,
        });
        let mut days = vec![scenario.world.umpire().clone()];
        while scenario.next_day().expect("a day resolves").is_some() {
            days.push(scenario.world.umpire().clone());
        }
        Run {
            events: scenario.world.events().iter().cloned().collect(),
            days,
        }
    }

    fn the_scenario() -> Run {
        Run::of(common::load(&path()).expect("the scenario loads"))
    }

    fn text(source: &str) -> Run {
        Run::of(common::from_text(source, &path()).expect("the variant loads"))
    }

    fn at(&self, day: u32) -> &TruthState {
        &self.days[day as usize]
    }

    fn fuel(&self, day: u32, ship: u32) -> Fixed {
        self.at(day).ships[&sim_core::ids::ShipId::new(ship)].fuel_t
    }

    fn base_fuel(&self, day: u32, base: u32) -> Fixed {
        self.at(day).bases[&sim_core::ids::BaseId::new(base)].fuel_t
    }

    fn base_supply(&self, day: u32, base: u32) -> Fixed {
        self.at(day).bases[&sim_core::ids::BaseId::new(base)].supply_t
    }

    /// The `ShipRefuelled` events of a pulse, in log order.
    fn fuelled(&self, day: u32, pulse: Pulse) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|event| {
                event.day == day
                    && event.pulse == Some(pulse)
                    && matches!(event.kind, EventKind::ShipRefuelled { .. })
            })
            .collect()
    }

    /// What `ship` took in the pulse: the tons, whether it had the berth,
    /// and what the base held after.
    fn took(&self, day: u32, pulse: Pulse, ship: u32) -> Option<(Fixed, bool, Fixed)> {
        self.fuelled(day, pulse)
            .into_iter()
            .find_map(|event| match event.kind {
                EventKind::ShipRefuelled {
                    ship: of,
                    taken_t,
                    base_fuel_t,
                    ..
                } if of.raw() == ship => {
                    Some((taken_t, factor(event, "on_berth") == n(1), base_fuel_t))
                }
                _ => None,
            })
    }

    fn held(&self) -> Vec<(u32, Option<Pulse>, u32, Fixed)> {
        self.events
            .iter()
            .filter_map(|event| match event.kind {
                EventKind::RefuellingHeldAtReserve { ship, short_t, .. } => {
                    Some((event.day, event.pulse, ship.raw(), short_t))
                }
                _ => None,
            })
            .collect()
    }
}

/// A factor of an event's breakdown.
fn factor(event: &Event, name: &str) -> Fixed {
    event
        .breakdown
        .as_ref()
        .expect("breakdown recorded")
        .factors
        .iter()
        .find(|factor| factor.name == name)
        .unwrap_or_else(|| panic!("factor {name}"))
        .value
}

fn took(run: &Run, day: u32, pulse: Pulse, ship: u32) -> (i64, bool, i64) {
    let (taken, berth, base) = run
        .took(day, pulse, ship)
        .unwrap_or_else(|| panic!("ship {ship} took fuel on day {day} in {pulse:?}"));
    (taken.trunc(), berth, base.trunc())
}

// ------------------------------------------------------------------- Tests

#[test]
fn at_s_one_berth_cargo_and_fuel() {
    let run = Run::the_scenario();
    // Night: ship 1 has the berth, loads 500 t in 10 hours and takes 200 t
    // of fuel in the 2 hours left; ship 2 fuels by lighter, 12 x 40 = 480.
    assert_eq!(took(&run, 1, Pulse::Night, 1), (200, true, 4800));
    assert_eq!(took(&run, 1, Pulse::Night, 2), (480, false, 4320));
    let night = run.fuelled(1, Pulse::Night);
    let ship_1 = night
        .iter()
        .find(
            |event| matches!(event.kind, EventKind::ShipRefuelled { ship, .. } if ship.raw() == 1),
        )
        .expect("ship 1 fuelled");
    assert_eq!(factor(ship_1, "cargo_hours"), n(10));
    assert_eq!(factor(ship_1, "hours_left"), n(2));
    assert_eq!(factor(ship_1, "rate_t_per_h"), n(100));
    // The load is complete at hour 10 and puts 500 t aboard ship 1, leaving
    // 1500 t of supply at base 1.
    let loaded: Vec<_> = run
        .events
        .iter()
        .filter_map(|event| match event.kind {
            EventKind::CargoLoaded {
                ship,
                loaded_t,
                aboard_t,
                base_stock_t,
                ..
            } => Some((
                event.day,
                event.pulse,
                ship.raw(),
                loaded_t,
                aboard_t,
                base_stock_t,
            )),
            _ => None,
        })
        .collect();
    assert_eq!(
        loaded,
        vec![(1, Some(Pulse::Night), 1, n(500), n(500), n(1500))]
    );
    let complete: Vec<_> = run
        .events
        .iter()
        .filter_map(|event| match event.kind {
            EventKind::LoadingComplete { elapsed_hours, .. } => {
                Some((event.day, event.pulse, elapsed_hours))
            }
            _ => None,
        })
        .collect();
    assert_eq!(complete, vec![(1, Some(Pulse::Night), n(10))]);

    // Morning: ship 1 still has the berth and takes its last 200 t in 2
    // hours, full at hour 14; ship 2 takes 240 by lighter.
    assert_eq!(took(&run, 1, Pulse::Morning, 1), (200, true, 4120));
    assert_eq!(took(&run, 1, Pulse::Morning, 2), (240, false, 3880));
    let morning = run.fuelled(1, Pulse::Morning);
    let ship_1 = morning
        .iter()
        .find(
            |event| matches!(event.kind, EventKind::ShipRefuelled { ship, .. } if ship.raw() == 1),
        )
        .expect("ship 1 fuelled");
    assert_eq!(
        pulse_start(1, Pulse::Morning) + factor(ship_1, "lacks_t") / factor(ship_1, "rate_t_per_h"),
        n(14)
    );
    assert_eq!(run.fuel(1, 1), n(1000));

    // Afternoon: ship 1 has nothing to do; ship 2 has the berth and takes
    // its last 480 t in 4.8 hours, full at hour 22.8.
    assert!(run.took(1, Pulse::Afternoon, 1).is_none());
    assert_eq!(took(&run, 1, Pulse::Afternoon, 2), (480, true, 3400));
    let afternoon = run.fuelled(1, Pulse::Afternoon);
    assert_eq!(afternoon.len(), 2, "ship 2 at S and ship 3 at U");
    let ship_2 = afternoon
        .iter()
        .find(
            |event| matches!(event.kind, EventKind::ShipRefuelled { ship, .. } if ship.raw() == 2),
        )
        .expect("ship 2 fuelled");
    assert_eq!(
        pulse_start(1, Pulse::Afternoon)
            + factor(ship_2, "lacks_t") / factor(ship_2, "rate_t_per_h"),
        d("22.8")
    );
    assert_eq!(run.fuel(1, 2), n(2000));

    // Base 1 holds 1500 t of supply from the night of day 1 to the end.
    assert_eq!(run.base_supply(1, 1), n(1500));
    assert_eq!(run.base_supply(2, 1), n(1500));
    assert_eq!(run.base_fuel(1, 1), n(3400));
}

#[test]
fn at_no_start_of_a_day_is_any_ship_refuelled() {
    let run = Run::the_scenario();
    let refuellings: Vec<_> = run
        .events
        .iter()
        .filter(|event| matches!(event.kind, EventKind::ShipRefuelled { .. }))
        .collect();
    assert!(!refuellings.is_empty());
    assert!(refuellings.iter().all(|event| event.pulse.is_some()));
    // Every refuelling follows an event of the day's pulses: none sits
    // between the day's orders and the night pulse's movement.
    for day in 1..=2 {
        let first_pulse_event = run
            .events
            .iter()
            .position(|event| event.day == day && event.pulse == Some(Pulse::Night))
            .expect("an event in the night");
        assert!(run.events[..first_pulse_event]
            .iter()
            .filter(|event| event.day == day)
            .all(|event| !matches!(event.kind, EventKind::ShipRefuelled { .. })));
    }
}

#[test]
fn at_u_no_berth_and_sailing_short() {
    let run = Run::the_scenario();
    // Day 1: ship 3 fuels by lighter in all three pulses, 24 x 40 = 960 t.
    assert_eq!(took(&run, 1, Pulse::Night, 3), (480, false, 2520));
    assert_eq!(took(&run, 1, Pulse::Morning, 3), (240, false, 2280));
    assert_eq!(took(&run, 1, Pulse::Afternoon, 3), (240, false, 2040));
    assert_eq!(run.fuel(1, 3), n(1460));
    assert_eq!(run.base_fuel(1, 2), n(2040));

    // Day 2: she sails at hour 24 with 1460 t, steams 200 nm in 12.5 hours,
    // burns 50 t and arrives at S at hour 36.5 with 1410 t.
    let arrival = run
        .events
        .iter()
        .find_map(|event| match event.kind {
            EventKind::TaskForceArrived {
                task_force,
                elapsed_hours,
                ..
            } if task_force.raw() == 3 => Some((event.day, event.pulse, elapsed_hours)),
            _ => None,
        })
        .expect("force 3 arrived");
    assert_eq!(arrival, (2, Some(Pulse::Morning), d("36.5")));
    // Nothing of hers in the night of day 2: she moved the whole pulse.
    assert!(run.took(2, Pulse::Night, 3).is_none());
    // Morning: no other ship has work, so she has the berth for the 5.5
    // hours left and takes 550 t; afternoon: her last 40 t in 0.4 hours.
    let (taken, berth, base) = run.took(2, Pulse::Morning, 3).expect("took fuel");
    assert_eq!((taken, berth, base), (n(550), true, n(2850)));
    let morning = run.fuelled(2, Pulse::Morning);
    assert_eq!(morning.len(), 1);
    // She had 1410 t on arriving, and lacked 590 t.
    assert_eq!(factor(morning[0], "lacks_t"), n(590));
    assert_eq!(factor(morning[0], "hours_left"), d("5.5"));
    let EventKind::ShipRefuelled {
        fuel_t, taken_t, ..
    } = morning[0].kind
    else {
        unreachable!()
    };
    assert_eq!(fuel_t - taken_t, n(1410));
    assert_eq!(took(&run, 2, Pulse::Afternoon, 3), (40, true, 2810));
    let afternoon = run.fuelled(2, Pulse::Afternoon);
    assert_eq!(
        pulse_start(2, Pulse::Afternoon)
            + factor(afternoon[0], "lacks_t") / factor(afternoon[0], "rate_t_per_h"),
        d("42.4")
    );
    assert_eq!(run.fuel(2, 3), n(2000));
}

#[test]
fn at_v_a_reserve_holds_a_destroyer_short_and_it_is_recorded_once() {
    let run = Run::the_scenario();
    // Night of day 1: 200 t by lighter in 5 hours, and base 3 is at its
    // reserve of 100 t.
    assert_eq!(took(&run, 1, Pulse::Night, 4), (200, false, 100));
    let night = run.fuelled(1, Pulse::Night);
    let ship_4 = night
        .iter()
        .find(
            |event| matches!(event.kind, EventKind::ShipRefuelled { ship, .. } if ship.raw() == 4),
        )
        .expect("ship 4 fuelled");
    assert_eq!(
        factor(ship_4, "base_above_reserve_t") / factor(ship_4, "rate_t_per_h"),
        n(5)
    );
    // She takes nothing in any later pulse.
    for (day, pulse) in [
        (1, Pulse::Morning),
        (1, Pulse::Afternoon),
        (2, Pulse::Night),
        (2, Pulse::Morning),
        (2, Pulse::Afternoon),
    ] {
        assert!(run.took(day, pulse, 4).is_none());
    }
    // Held, 300 t short, once: in the night of day 1.
    assert_eq!(run.held(), vec![(1, Some(Pulse::Night), 4, n(300))]);
    assert_eq!(run.base_fuel(2, 3), n(100));
}

#[test]
fn the_end_of_day_2() {
    let run = Run::the_scenario();
    let ships = &run.at(2).ships;
    let ship = |raw: u32| &ships[&sim_core::ids::ShipId::new(raw)];
    assert_eq!(ship(1).fuel_t, n(1000));
    assert_eq!(ship(1).supply_t, n(500));
    assert_eq!(ship(2).fuel_t, n(2000));
    assert_eq!(ship(3).fuel_t, n(2000));
    assert_eq!(ship(4).fuel_t, n(200));
    // Where they are: ships 1 and 2 at base 1, ship 3 arrived at base 1,
    // ship 4 at base 3.
    let forces = &run.at(2).task_forces;
    let at = |raw: u32| forces[&sim_core::ids::TaskForceId::new(raw)].position();
    let hex_of = |raw: u32| run.at(2).bases[&sim_core::ids::BaseId::new(raw)].hex;
    assert_eq!(at(1), hex_of(1));
    assert_eq!(at(2), hex_of(1));
    assert_eq!(at(3), hex_of(1));
    assert_eq!(at(4), hex_of(3));
    assert_eq!(run.base_supply(2, 1), n(1500));
    assert_eq!(run.base_fuel(2, 1), n(2810));
    assert_eq!(run.base_fuel(2, 2), n(2040));
    assert_eq!(run.base_fuel(2, 3), n(100));
}

#[test]
fn asked_before_day_2_the_list_of_forces_says_what_force_3_lacks_and_how_long_it_would_take() {
    use serde_json::json;
    use sim_agent::{Seat, Session};
    use sim_core::state::Side;

    // Day 1 is resolved; day 2 is not, and its scripted order (force 3 to S)
    // is not given yet.
    let mut scenario = common::load(&path()).expect("the scenario loads");
    scenario.next_day().expect("day 1 resolves");
    assert_eq!(scenario.world.day(), 1);
    let name = scenario.name.clone();
    let mut session = Session::new(
        scenario.world,
        &name,
        scenario.days,
        Seat::Player,
        Seat::Script(BTreeMap::new()),
    );

    let list = session
        .call(Side::Allied, "list_forces", &json!({}))
        .expect("the forces are listed");
    let forces = list["task_forces"].as_array().expect("a list of forces");
    let line = |forces: &Vec<serde_json::Value>, number: &str| {
        forces
            .iter()
            .find(|f| f["task_force"]["handle"] == format!("task_force:{number}"))
            .unwrap_or_else(|| panic!("force {number} is listed"))
            .clone()
    };
    // Force 3: 500 t aboard at the start, 960 t taken by lighter: 1460 t of
    // 2000 t; the lack is 540 t, and with no berth at base 2 (port size 0)
    // it is 540 / 40 = 13.5 hours.
    let three = line(forces, "3");
    assert_eq!(three["fuel_lacked_t"], "540");
    assert_eq!(three["fuelling_hours"], "13.5");
    assert_eq!(three["fuelling_at"]["handle"], "base:2");
    // Force 4's destroyer is held at a reserve with 300 t short, at base 3
    // (port size 0, 40 t an hour): 300 / 40 = 7.5 hours.
    let four = line(forces, "4");
    assert_eq!(four["fuel_lacked_t"], "300");
    assert_eq!(four["fuelling_hours"], "7.5");
    // Force 1 and force 2 are full at S: they lack nothing and say nothing.
    for number in ["1", "2"] {
        let full = line(forces, number);
        assert!(full.get("fuel_lacked_t").is_none(), "force {number}");
        assert!(full.get("fuelling_hours").is_none(), "force {number}");
        assert!(full.get("fuelling_at").is_none(), "force {number}");
    }

    // The briefing's line for force 3 says it in words.
    let briefing = session
        .call(Side::Allied, "get_briefing", &json!({ "text": true }))
        .expect("the briefing is given");
    let text = briefing["text"]
        .as_str()
        .expect("the briefing has its text");
    let line = text
        .lines()
        .find(|line| line.contains("task_force:3"))
        .expect("the briefing has a line for force 3");
    assert!(
        line.contains("lacks 540 t of fuel: 13.5 hours at base:2"),
        "{line}"
    );
}

// -------------------------------------------------------------- Properties

#[test]
fn fuel_is_conserved_and_never_below_a_reserve_in_every_pulse() {
    let run = Run::the_scenario();
    // Walk the refuellings in order: what a base holds after each is what it
    // held before less what the ship took, and the ship's fuel after is its
    // fuel before plus the same.
    let mut stock: BTreeMap<u32, Fixed> = run
        .at(0)
        .bases
        .iter()
        .map(|(id, base)| (id.raw(), base.fuel_t))
        .collect();
    let reserve: BTreeMap<u32, Fixed> = run
        .at(0)
        .bases
        .iter()
        .map(|(id, base)| (id.raw(), base.fuel_reserve_t))
        .collect();
    let mut taken_in_all = Fixed::ZERO;
    for event in &run.events {
        if let EventKind::ShipRefuelled {
            base,
            taken_t,
            base_fuel_t,
            ..
        } = event.kind
        {
            let before = stock[&base.raw()];
            assert_eq!(before - taken_t, base_fuel_t, "{event:?}");
            assert!(base_fuel_t >= reserve[&base.raw()], "{event:?}");
            stock.insert(base.raw(), base_fuel_t);
            taken_in_all += taken_t;
        }
    }
    // What the bases gave in all is what the ships took: 8300 t at the start,
    // 4950 t at the end.
    let given: Fixed = run
        .at(0)
        .bases
        .values()
        .map(|base| base.fuel_t)
        .fold(Fixed::ZERO, |sum, fuel| sum + fuel)
        - run
            .at(2)
            .bases
            .values()
            .map(|base| base.fuel_t)
            .fold(Fixed::ZERO, |sum, fuel| sum + fuel);
    assert_eq!(given, taken_in_all);
    assert_eq!(given, n(3350));
    for day in 0..=2 {
        for (id, base) in &run.at(day).bases {
            assert!(base.fuel_t >= reserve[&id.raw()]);
        }
    }
}

#[test]
fn no_ship_takes_more_than_the_hours_of_a_pulse_at_its_rate() {
    let run = Run::the_scenario();
    for event in &run.events {
        let EventKind::ShipRefuelled { taken_t, .. } = event.kind else {
            continue;
        };
        let pulse = event.pulse.expect("a pulse");
        let hours = factor(event, "hours_worked");
        assert!(hours <= n(pulse.hours()), "{event:?}");
        let rate = factor(event, "rate_t_per_h");
        assert!(taken_t <= rate * (factor(event, "hours_left")), "{event:?}");
        assert!(factor(event, "hours_left") <= hours, "{event:?}");
        assert!(taken_t > Fixed::ZERO);
    }
    // And none takes fuel in the pulse in which it moved the whole of it:
    // force 3 in the night of day 2.
    assert!(run.took(2, Pulse::Night, 3).is_none());
}

#[test]
fn with_force_2_given_the_lower_number_ship_2_has_the_berth_in_the_night() {
    // Force 1 and force 2 swap numbers; the load is still ordered to the
    // cargo ship's force.
    let source = text();
    let source = common::replace_once(
        &source,
        "id = 1\nname = \"Placeholder cargo force 1\"",
        "id = 2\nname = \"Placeholder cargo force 1\"",
    );
    let source = common::replace_once(
        &source,
        "id = 2\nname = \"Placeholder cruiser force 2\"",
        "id = 1\nname = \"Placeholder cruiser force 2\"",
    );
    let source = common::replace_once(
        &source,
        "fuel_t = 600\ntask_force = 1",
        "fuel_t = 600\ntask_force = 2",
    );
    let source = common::replace_once(
        &source,
        "fuel_t = 800\ntask_force = 2",
        "fuel_t = 800\ntask_force = 1",
    );
    let source = common::replace_once(
        &source,
        "day = 1\nside = \"allied\"\ntask_force = 1\nkind = \"load\"",
        "day = 1\nside = \"allied\"\ntask_force = 2\nkind = \"load\"",
    );
    let run = Run::text(&source);
    // Ship 2 has the berth and takes 1200 t in 12 hours.
    assert_eq!(took(&run, 1, Pulse::Night, 2), (1200, true, 3800));
    // Ship 1 loads 240 t by lighter and takes no fuel that pulse: its cargo
    // took the 12 hours.
    let loaded: Vec<_> = run
        .events
        .iter()
        .filter(|event| event.day == 1 && event.pulse == Some(Pulse::Night))
        .filter_map(|event| match event.kind {
            EventKind::CargoLoaded { ship, loaded_t, .. } => Some((ship.raw(), loaded_t)),
            _ => None,
        })
        .collect();
    assert_eq!(loaded, vec![(1, n(240))]);
    assert!(run.took(1, Pulse::Night, 1).is_none());
}

#[test]
fn with_no_fuelling_rate_given_a_ship_takes_all_it_lacks_in_the_first_pulse() {
    // The scenario's own port sizes give rates; the frozen step 1 port sizes
    // give none. On them every ship fills in the night of day 1, and nothing
    // is taken in any later pulse of the day. (That every other scenario's
    // stocks at the end of each day are as they were under the morning's
    // refuelling is checked by the comparison of every scenario, which the
    // team lead runs, not here.)
    let source = common::replace_once(
        &text(),
        "port_sizes_placeholder_fuelling.csv",
        "port_sizes.csv",
    );
    let run = Run::text(&source);
    assert_eq!(run.fuelled(1, Pulse::Night).len(), 4);
    assert_eq!(took(&run, 1, Pulse::Night, 1).0, 400);
    assert_eq!(took(&run, 1, Pulse::Night, 2).0, 1200);
    assert_eq!(took(&run, 1, Pulse::Night, 3).0, 1500);
    // Base 3 gives down to its reserve only.
    assert_eq!(took(&run, 1, Pulse::Night, 4).0, 200);
    assert!(run.fuelled(1, Pulse::Morning).is_empty());
    assert!(run.fuelled(1, Pulse::Afternoon).is_empty());
    // No rate, no factor of a rate in the breakdown.
    for event in run.fuelled(1, Pulse::Night) {
        let names: Vec<_> = event
            .breakdown
            .as_ref()
            .expect("breakdown")
            .factors
            .iter()
            .map(|factor| factor.name)
            .collect();
        assert!(!names.contains(&"rate_t_per_h"));
    }
    // And fuel alone claims no berth: ship 1 had the berth for its cargo.
    assert_eq!(run.fuel(1, 1), n(1000));
}

#[test]
fn a_force_that_ran_out_of_fuel_at_a_base_is_idle_again_in_the_first_pulse_a_ship_takes_fuel() {
    // Force 3 sails from U on day 1 with 10 t, which is 40 nm: it runs dry in
    // the centre of the hex where a base of the side lies (a variant: base 4
    // at (1,0) beside S) and takes no fuel in the pulse it ran dry, but
    // fuels, and is restored, in the next.
    let source = text();
    let source = common::replace_once(
        &source,
        "fuel_t = 500\ntask_force = 3",
        "fuel_t = 10\ntask_force = 3",
    );
    let source = common::replace_once(
        &source,
        "# Day 1: force 1 loads supply up to 500 t.",
        "[[base]]\nid = 4\nname = \"Placeholder base W, Allied\"\nside = \"allied\"\nq = 4\nr = 0\nfuel_t = 700\n\n# Day 1: force 1 loads supply up to 500 t.",
    );
    let source = common::replace_once(
        &source,
        "day = 2\nside = \"allied\"\ntask_force = 3\nkind = \"set_route\"",
        "day = 1\nside = \"allied\"\ntask_force = 3\nkind = \"set_route\"",
    );
    let source = common::replace_once(
        &source,
        "day = 2\nside = \"allied\"\ntask_force = 3\nkind = \"set_speed\"",
        "day = 1\nside = \"allied\"\ntask_force = 3\nkind = \"set_speed\"",
    );
    let run = Run::text(&source);
    // It ran dry in the night of day 1.
    let dry = run
        .events
        .iter()
        .find(|event| matches!(event.kind, EventKind::TaskForceOutOfFuel { task_force, .. } if task_force.raw() == 3))
        .expect("force 3 ran out of fuel");
    assert_eq!((dry.day, dry.pulse), (1, Some(Pulse::Night)));
    // Nothing in that pulse; in the morning the first ship takes fuel, and
    // the event says the force is restored.
    assert!(run.took(1, Pulse::Night, 3).is_none());
    let morning = run.fuelled(1, Pulse::Morning);
    let event = morning
        .iter()
        .find(
            |event| matches!(event.kind, EventKind::ShipRefuelled { ship, .. } if ship.raw() == 3),
        )
        .expect("ship 3 took fuel in the morning");
    assert!(matches!(
        event.kind,
        EventKind::ShipRefuelled {
            force_restored: true,
            ..
        }
    ));
    let force = &run.at(1).task_forces[&sim_core::ids::TaskForceId::new(3)];
    assert_eq!(force.status, TaskForceStatus::Idle);
}
