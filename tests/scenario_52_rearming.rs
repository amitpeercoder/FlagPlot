//! Scenario 52: rearming. A ship in port takes main battery rounds and
//! torpedoes from the base's ammunition through the hours of the day, at
//! a berth or by lighter.
//!
//! Proves decision 0122 against a hand calculation. The data is placeholder
//! data: see `data/scenarios/52_rearming.toml`. It runs on the frozen step
//! 1 figures, with the rearming rates and weights set in the scenario's
//! own files, since the frozen figures give none. There is no enemy, no
//! search and no random draw.
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
//! | Port size | Berths | Ammunition at a berth | Ammunition by lighter |
//! | --------- | ------ | --------------------- | --------------------- |
//! | 0         | 0      | none                  | 2 t an hour           |
//! | 1         | 1      | 5 t an hour           | 2 t an hour           |
//!
//! A round weighs 0.1 t for a cruiser and 0.02 t for a destroyer. A
//! torpedo weighs 2 t. These are this scenario's own figures and not the
//! live ones.
//!
//! The placeholder cruiser (class 1): 8 main guns and 100 rounds a gun, an
//! outfit of 800 rounds; 8 torpedo tubes. The placeholder destroyer (class
//! 2): 4 main guns and 150 rounds a gun, an outfit of 600 rounds; 8 tubes.
//! A day is a night pulse of 12 hours and two daylight pulses of 6. A ship
//! does its cargo first, then takes its fuel, then takes ammunition in the
//! hours of the pulse that are left, rounds before torpedoes.
//!
//! So at a berth the cruiser takes 50 rounds an hour or 2.5 torpedoes an
//! hour, and by lighter a destroyer takes 100 rounds an hour.
//!
//! ## Inputs
//!
//! A placeholder map: S at (0,0), a coast hex with port size 1, and U at
//! (5,0), a coast hex with port size 0. The game is 1 day. Everything is
//! Allied, and every ship is full of fuel and carries no cargo.
//!
//! | Base | Hex | Ammunition |
//! | ---- | --- | ---------- |
//! | 1    | S   | 80 t       |
//! | 2    | U   | 3 t        |
//!
//! | Force | Ship | Rounds aboard | Tubes fired | Where | Lacks |
//! | ----- | ---- | ------------- | ----------- | ----- | ----- |
//! | 1     | 1, cruiser   | 300 of 800 | all 8 | stopped at base 1 | 500 rounds, 50 t; 8 torpedoes, 16 t: 66 t |
//! | 2     | 2, destroyer | 100 of 600 | none  | stopped at base 1 | 500 rounds, 10 t |
//! | 3     | 3, destroyer | 100 of 600 | none  | stopped at base 2 | 500 rounds, 10 t |
//!
//! There is no order.
//!
//! ## At S: one berth
//!
//! | Pulse | Force 1, ship 1 | Force 2, ship 2 | Base 1's ammunition after |
//! | ----- | --------------- | --------------- | ------------------------- |
//! | Night | Has the berth. Takes 500 rounds, 50 t, in 10 hours; then in the 2 hours left 5 torpedoes, 10 t | By lighter: 500 rounds, 10 t, in 5 hours: full at hour 5 | 80 - 60 - 10 = 10 t |
//! | Morning | Has the berth. Takes her last 3 torpedoes, 6 t, in 1.2 hours: full at hour 13.2 | Nothing to do | 4 t |
//! | Afternoon | Nothing to do | Nothing to do | 4 t |
//!
//! At the end of the night ship 1 has 800 rounds and tubes 1 to 5 loaded;
//! tubes 6, 7 and 8 are still fired. At the end of the morning all 8 are
//! loaded.
//!
//! Three events are recorded at S: ship 1 in the night, 500 rounds, 5
//! torpedoes, 60 t; ship 2 in the night, 500 rounds, no torpedo, 10 t;
//! ship 1 in the morning, no round, 3 torpedoes, 6 t.
//!
//! Asked before the day is resolved, the list of forces says force 1 lacks
//! 500 rounds and 8 torpedoes, 66 t, and that at base 1 at a berth it
//! would take 13.2 hours; and force 2 lacks 500 rounds, 10 t, 5 hours by
//! lighter.
//!
//! ## At U: the stock runs out
//!
//! Ship 3 takes by lighter 100 rounds an hour. Base 2 holds 3 t, which is
//! 150 rounds: she takes them in the first 1.5 hours of the night and no
//! more. She ends the day with 250 rounds, and base 2 with none. One
//! event: 150 rounds, 3 t. Nothing is recorded for her in the morning or
//! the afternoon.
//!
//! # Properties
//!
//! - **The game as it was.** With no ammunition at either base, or with no
//!   ammunition rate in the port sizes, no ship takes any and nothing is
//!   recorded.
//! - **After cargo and fuel.** With base 1 holding 500 t of supply, cargo
//!   worked at a berth at 50 t an hour, and ship 1 a ship that can carry
//!   it and is ordered to load the 500 t, which takes 10 hours of the
//!   night: she takes
//!   100 rounds in the 2 hours left; in the morning 300 rounds in 6 hours;
//!   in the afternoon her last 100 rounds in 2 hours and then, in the 4
//!   hours left, her 8 torpedoes in 3.2: full at hour 23.2.
//! - **Whole ones only.** With base 1 holding 61 t in place of 80, and
//!   force 2 absent: in the night ship 1 takes 60 t as above; in the
//!   morning 1 t is left, which is less than a torpedo, and she takes
//!   nothing. Base 1 keeps its 1 t.
//! - **Short stock, in the queue's order.** With base 1 holding 65 t: in
//!   the night ship 1, served first, takes her 60 t, and ship 2 takes the
//!   5 t that are left, 250 rounds, in 2.5 hours. In the morning there is
//!   none.
//! - **The marker.** A ship whose class has 9,999 rounds a gun lacks no
//!   rounds and takes none, and takes torpedoes as any ship does.
//! - **She can fire again.** In a fight built for it on the day after,
//!   ship 1 has torpedo shots to fire; with base 1 holding no ammunition
//!   she has none.
//! - **A ship that sails keeps what she took.** With force 1 routed to U
//!   on the morning of day 1, she leaves with 800 rounds and 5 tubes
//!   loaded.
//! - Every existing scenario's events and end states are what they were.

mod common;

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::events::{Event, EventKind, Weapon};
use sim_core::fixed::Fixed;
use sim_core::ids::{BaseId, ShipId};
use sim_core::state::TruthState;
use sim_core::time::Pulse;
use sim_core::world::SimConfig;
use sim_data::Scenario;

// ------------------------------------------------------------------ Helpers

fn path() -> PathBuf {
    common::data_dir().join("scenarios/52_rearming.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
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

    fn with(source: &str, options: &sim_data::LoadOptions) -> Run {
        Run::of(common::from_text_with(source, &path(), options).expect("the variant loads"))
    }

    fn at(&self, day: u32) -> &TruthState {
        &self.days[day as usize]
    }

    fn rounds(&self, day: u32, ship: u32) -> Fixed {
        self.at(day).ships[&ShipId::new(ship)].gun_rounds
    }

    fn base_ammunition(&self, day: u32, base: u32) -> Fixed {
        self.at(day).bases[&BaseId::new(base)].ammunition_t
    }

    /// The tubes of a ship that are fired and not loaded, at the end of a
    /// day (all 8 tubes of the placeholder cruiser and destroyer bear to
    /// both sides).
    fn tubes_fired(&self, day: u32, ship: u32) -> u32 {
        let ship = &self.at(day).ships[&ShipId::new(ship)];
        ship.tubes_fired(8, 8)
    }

    /// Every `ShipRearmed` event: (day, pulse, ship, rounds, torpedoes,
    /// taken t, rounds aboard, tubes fired, base's ammunition after).
    fn rearmed(&self) -> Vec<Rearmed> {
        self.events
            .iter()
            .filter_map(|event| match event.kind {
                EventKind::ShipRearmed {
                    ship,
                    rounds,
                    torpedoes,
                    taken_t,
                    rounds_aboard,
                    tubes_fired,
                    base_ammunition_t,
                    ..
                } => Some(Rearmed {
                    day: event.day,
                    pulse: event.pulse.expect("a pulse"),
                    ship: ship.raw(),
                    rounds,
                    torpedoes,
                    taken_t,
                    rounds_aboard,
                    tubes_fired,
                    base_ammunition_t,
                }),
                _ => None,
            })
            .collect()
    }

    fn rearmed_of(&self, day: u32, pulse: Pulse, ship: u32) -> Option<Rearmed> {
        self.rearmed()
            .into_iter()
            .find(|r| r.day == day && r.pulse == pulse && r.ship == ship)
    }
}

#[derive(Clone, Debug)]
struct Rearmed {
    day: u32,
    pulse: Pulse,
    ship: u32,
    rounds: u32,
    torpedoes: u32,
    taken_t: Fixed,
    rounds_aboard: Fixed,
    tubes_fired: u32,
    base_ammunition_t: Fixed,
}

/// What one event says: rounds, torpedoes and tons taken.
fn took(run: &Run, day: u32, pulse: Pulse, ship: u32) -> (u32, u32, i64) {
    let r = run
        .rearmed_of(day, pulse, ship)
        .unwrap_or_else(|| panic!("ship {ship} rearmed on day {day} in {pulse:?}"));
    (r.rounds, r.torpedoes, r.taken_t.trunc())
}

fn nothing_recorded(run: &Run, day: u32, pulse: Pulse, ship: u32) -> bool {
    run.rearmed_of(day, pulse, ship).is_none()
}

// ------------------------------------------------------------------- Tests

#[test]
fn at_s_one_berth_a_cruiser_and_a_destroyer_by_lighter() {
    let run = Run::the_scenario();
    // Night: ship 1 has the berth: 500 rounds, 50 t, in 10 hours; then in the
    // 2 hours left 5 torpedoes, 10 t. Ship 2 by lighter: 500 rounds, 10 t.
    let one = run.rearmed_of(1, Pulse::Night, 1).expect("ship 1");
    assert_eq!((one.rounds, one.torpedoes), (500, 5));
    assert_eq!(one.taken_t, n(60));
    assert_eq!(one.rounds_aboard, n(800));
    // Tubes 1 to 5 are loaded: 3 are still fired.
    assert_eq!(one.tubes_fired, 3);
    assert_eq!(one.base_ammunition_t, n(20));
    let two = run.rearmed_of(1, Pulse::Night, 2).expect("ship 2");
    assert_eq!((two.rounds, two.torpedoes), (500, 0));
    assert_eq!(two.taken_t, n(10));
    assert_eq!(two.rounds_aboard, n(600));
    assert_eq!(two.tubes_fired, 0);
    assert_eq!(two.base_ammunition_t, n(10));

    // Morning: ship 1 takes her last 3 torpedoes, 6 t: base 1 holds 4 t.
    let one = run.rearmed_of(1, Pulse::Morning, 1).expect("ship 1");
    assert_eq!((one.rounds, one.torpedoes), (0, 3));
    assert_eq!(one.taken_t, n(6));
    assert_eq!(one.rounds_aboard, n(800));
    assert_eq!(one.tubes_fired, 0);
    assert_eq!(one.base_ammunition_t, n(4));
    assert!(nothing_recorded(&run, 1, Pulse::Morning, 2));

    // Afternoon: nothing to do, and nothing recorded for ships 1 and 2.
    assert!(nothing_recorded(&run, 1, Pulse::Afternoon, 1));
    assert!(nothing_recorded(&run, 1, Pulse::Afternoon, 2));

    // The end of the day.
    assert_eq!(run.rounds(1, 1), n(800));
    assert_eq!(run.tubes_fired(1, 1), 0);
    assert!(run.at(1).ships[&ShipId::new(1)].torpedoes_ready);
    assert_eq!(run.rounds(1, 2), n(600));
    assert_eq!(run.base_ammunition(1, 1), n(4));
}

#[test]
fn the_hours_the_work_took_at_s() {
    // 500 rounds at 50 an hour is 10 hours; 5 torpedoes at 2.5 an hour is 2
    // hours; the last 3 torpedoes are 1.2 hours, full at hour 13.2; ship 2's
    // 500 rounds by lighter at 100 an hour are 5 hours. The hours are the
    // tons taken over the rate in the event's breakdown.
    let run = Run::the_scenario();
    let hours = |day: u32, pulse: Pulse, ship: u32| {
        let event = run
            .events
            .iter()
            .find(|e| {
                e.day == day
                    && e.pulse == Some(pulse)
                    && matches!(e.kind, EventKind::ShipRearmed { ship: s, .. } if s.raw() == ship)
            })
            .expect("the event");
        let EventKind::ShipRearmed { taken_t, .. } = event.kind else {
            unreachable!()
        };
        let rate = event
            .breakdown
            .as_ref()
            .expect("breakdown recorded")
            .factors
            .iter()
            .find(|f| f.name == "rate_t_per_h")
            .expect("the rate")
            .value;
        taken_t / rate
    };
    // Ship 1 in the night: 50 t of rounds in 10 hours and 10 t of torpedoes
    // in 2: 60 t at 5 t an hour is 12 hours, the whole night.
    assert_eq!(hours(1, Pulse::Night, 1), n(12));
    assert_eq!(hours(1, Pulse::Night, 2), n(5));
    let morning = hours(1, Pulse::Morning, 1);
    assert_eq!(morning, "1.2".parse::<Fixed>().unwrap());
    assert_eq!(
        sim_core::time::pulse_start(1, Pulse::Morning) + morning,
        "13.2".parse::<Fixed>().unwrap()
    );
}

#[test]
fn nothing_else_is_recorded_at_s_or_in_the_afternoon() {
    let run = Run::the_scenario();
    let all: Vec<(u32, Pulse, u32)> = run
        .rearmed()
        .iter()
        .map(|r| (r.day, r.pulse, r.ship))
        .collect();
    assert_eq!(
        all,
        vec![
            (1, Pulse::Night, 1),
            (1, Pulse::Night, 2),
            (1, Pulse::Night, 3),
            (1, Pulse::Morning, 1),
        ]
    );
}

#[test]
fn at_u_the_stock_runs_out() {
    let run = Run::the_scenario();
    // 3 t is 150 rounds at 0.02 t: taken in the first 1.5 hours of the night.
    let three = run.rearmed_of(1, Pulse::Night, 3).expect("ship 3");
    assert_eq!((three.rounds, three.torpedoes), (150, 0));
    assert_eq!(three.taken_t, n(3));
    assert_eq!(three.rounds_aboard, n(250));
    assert_eq!(three.base_ammunition_t, n(0));
    assert!(nothing_recorded(&run, 1, Pulse::Morning, 3));
    assert!(nothing_recorded(&run, 1, Pulse::Afternoon, 3));
    assert_eq!(run.rounds(1, 3), n(250));
    assert_eq!(run.base_ammunition(1, 2), n(0));
    assert_eq!(took(&run, 1, Pulse::Night, 3), (150, 0, 3));
}

#[test]
fn asked_before_the_day_is_resolved_the_view_says_what_each_force_lacks() {
    let scenario = common::load(&path()).expect("the scenario loads");
    let view = scenario.world.side_view(sim_core::state::Side::Allied);
    let one = view
        .ammunition_work(sim_core::ids::TaskForceId::new(1))
        .expect("force 1 is the side's own");
    assert_eq!(one.lacks(), (500, 8, n(66)));
    assert_eq!(one.hours_to_full(), Some("13.2".parse().unwrap()));
    let two = view
        .ammunition_work(sim_core::ids::TaskForceId::new(2))
        .expect("force 2 is the side's own");
    assert_eq!(two.lacks(), (500, 0, n(10)));
    assert_eq!(two.hours_to_full(), Some(n(5)));
    let three = view
        .ammunition_work(sim_core::ids::TaskForceId::new(3))
        .expect("force 3 is the side's own");
    assert_eq!(three.lacks(), (500, 0, n(10)));
}

#[test]
fn asked_before_the_day_is_resolved_the_list_of_forces_says_it_too() {
    use serde_json::json;
    use sim_agent::{Seat, Session};
    use sim_core::state::Side;

    let scenario = common::load(&path()).expect("the scenario loads");
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
    let line = |number: &str| {
        forces
            .iter()
            .find(|f| f["task_force"]["handle"] == format!("task_force:{number}"))
            .unwrap_or_else(|| panic!("force {number} is listed"))
            .clone()
    };
    let one = &line("1")["ammunition_lacked"];
    assert_eq!(one["rounds"], 500);
    assert_eq!(one["torpedoes"], 8);
    assert_eq!(one["tons"], "66");
    assert_eq!(one["hours"], "13.2");
    let two = &line("2")["ammunition_lacked"];
    assert_eq!(two["rounds"], 500);
    assert_eq!(two["torpedoes"], 0);
    assert_eq!(two["tons"], "10");
    assert_eq!(two["hours"], "5");
}

// -------------------------------------------------------------- Properties

#[test]
fn the_game_as_it_was_with_no_ammunition_at_either_base() {
    let source = common::replace_once(&text(), "ammunition_t = 80", "ammunition_t = 0");
    let source = common::replace_once(&source, "ammunition_t = 3", "ammunition_t = 0");
    let run = Run::text(&source);
    assert!(run.rearmed().is_empty());
    assert_eq!(run.rounds(1, 1), n(300));
    assert_eq!(run.tubes_fired(1, 1), 8);
    assert!(!run.at(1).ships[&ShipId::new(1)].torpedoes_ready);
    assert_eq!(run.rounds(1, 2), n(100));
    assert_eq!(run.rounds(1, 3), n(100));
}

#[test]
fn the_game_as_it_was_with_no_ammunition_rate_in_the_port_sizes() {
    let options = common::altered_reference("rearming_no_rate", |dir| {
        common::edit_file(dir, "port_sizes_placeholder_rearming.csv", |text| {
            text.lines()
                .map(|line| {
                    if line.starts_with('#') || line.starts_with("port_size") {
                        line.to_string()
                    } else {
                        let cells: Vec<&str> = line.split(',').take(6).collect();
                        format!("{},,", cells.join(","))
                    }
                })
                .collect::<Vec<_>>()
                .join("\n")
                + "\n"
        });
    });
    let run = Run::with(&text(), &options);
    assert!(run.rearmed().is_empty());
    assert_eq!(run.rounds(1, 1), n(300));
    assert_eq!(run.base_ammunition(1, 1), n(80));
    assert_eq!(run.base_ammunition(1, 2), n(3));
}

#[test]
fn after_cargo_and_fuel() {
    // Ship 1 is given a cargo capacity of 3000 t, base 1 holds 500 t of
    // supply, and she is ordered to load 500 t, which takes 10 hours of the
    // night at 50 t an hour at the berth.
    let options = common::altered_reference("rearming_cargo", |dir| {
        common::edit_file(dir, "ship_classes.csv", |text| {
            common::replace_once(
                &text,
                "1,Placeholder cruiser,cruiser,16,32,2000,8000,0,8,3,100,20,8,8,200,400,0,0,0",
                "1,Placeholder cruiser,cruiser,16,32,2000,8000,3000,8,3,100,20,8,8,200,400,0,0,0",
            )
        });
    });
    let source = common::replace_once(
        &text(),
        "ammunition_t = 80",
        "supply_t = 500\nammunition_t = 80",
    );
    let source = format!(
        "{source}
[[order]]
day = 1
side = \"allied\"
task_force = 1
kind = \"load\"
weight_t = 500
"
    );
    let run = Run::with(&source, &options);
    let loaded: Vec<_> = run
        .events
        .iter()
        .filter_map(|e| match e.kind {
            EventKind::LoadingComplete { elapsed_hours, .. } => Some((e.pulse, elapsed_hours)),
            _ => None,
        })
        .collect();
    assert_eq!(loaded, vec![(Some(Pulse::Night), n(10))]);
    // Night: 2 hours left: 100 rounds, 10 t.
    assert_eq!(took(&run, 1, Pulse::Night, 1), (100, 0, 10));
    // Morning: 300 rounds in 6 hours, 30 t.
    assert_eq!(took(&run, 1, Pulse::Morning, 1), (300, 0, 30));
    // Afternoon: her last 100 rounds in 2 hours, 10 t, then in the 4 hours
    // left her 8 torpedoes, 16 t, in 3.2: full at hour 23.2.
    assert_eq!(took(&run, 1, Pulse::Afternoon, 1), (100, 8, 26));
    let last = run
        .rearmed_of(1, Pulse::Afternoon, 1)
        .expect("the afternoon's event");
    assert_eq!(last.rounds_aboard, n(800));
    assert_eq!(last.tubes_fired, 0);
    let rate = n(5);
    assert_eq!(
        sim_core::time::pulse_start(1, Pulse::Afternoon) + last.taken_t / rate,
        "23.2".parse::<Fixed>().unwrap()
    );
    // Ship 2, by lighter, is as it was; the base gave 76 t.
    assert_eq!(took(&run, 1, Pulse::Night, 2), (500, 0, 10));
    assert_eq!(run.base_ammunition(1, 1), n(4));
    assert_eq!(run.rounds(1, 1), n(800));
}

#[test]
fn whole_ones_only() {
    // Base 1 holds 61 t, and ship 2 lacks nothing: 60 t in the night as
    // above, and in the morning 1 t, less than a torpedo of 2 t, is not
    // taken.
    let source = common::replace_once(&text(), "ammunition_t = 80", "ammunition_t = 61");
    let source = common::replace_once(
        &source,
        "gun_rounds = 100\ntask_force = 2",
        "gun_rounds = 600\ntask_force = 2",
    );
    let run = Run::text(&source);
    assert_eq!(took(&run, 1, Pulse::Night, 1), (500, 5, 60));
    assert!(nothing_recorded(&run, 1, Pulse::Morning, 1));
    assert!(nothing_recorded(&run, 1, Pulse::Night, 2));
    assert_eq!(run.base_ammunition(1, 1), n(1));
    assert_eq!(run.tubes_fired(1, 1), 3);
}

#[test]
fn a_short_stock_is_given_in_the_queues_order() {
    let source = common::replace_once(&text(), "ammunition_t = 80", "ammunition_t = 65");
    let run = Run::text(&source);
    assert_eq!(took(&run, 1, Pulse::Night, 1), (500, 5, 60));
    // Ship 2 takes the 5 t that are left, 250 rounds, in 2.5 hours.
    assert_eq!(took(&run, 1, Pulse::Night, 2), (250, 0, 5));
    assert_eq!(run.rounds(1, 2), n(350));
    // In the morning there is none.
    assert!(nothing_recorded(&run, 1, Pulse::Morning, 1));
    assert!(nothing_recorded(&run, 1, Pulse::Morning, 2));
    assert_eq!(run.base_ammunition(1, 1), n(0));
}

#[test]
fn the_marker_a_ship_with_9999_rounds_a_gun_lacks_no_rounds() {
    let options = common::altered_reference("rearming_marker", |dir| {
        common::edit_file(dir, "ship_classes.csv", |text| {
            common::replace_once(
                &text,
                "1,Placeholder cruiser,cruiser,16,32,2000,8000,0,8,3,100,20,8,8,200,400,0,0,0",
                "1,Placeholder cruiser,cruiser,16,32,2000,8000,0,8,3,9999,20,8,8,200,400,0,0,0",
            )
        });
    });
    let run = Run::with(&text(), &options);
    // Ship 1 takes her 8 torpedoes, 16 t, in 3.2 hours of the night and no
    // round.
    assert_eq!(took(&run, 1, Pulse::Night, 1), (0, 8, 16));
    assert_eq!(run.rounds(1, 1), n(300));
    assert_eq!(run.tubes_fired(1, 1), 0);
    assert!(nothing_recorded(&run, 1, Pulse::Morning, 1));
    // Ship 2 is as it was.
    assert_eq!(took(&run, 1, Pulse::Night, 2), (500, 0, 10));
    assert_eq!(run.base_ammunition(1, 1), n(54));
}

#[test]
fn a_ship_that_sails_keeps_what_she_took() {
    // Base 1 holds 60 t and ship 2 lacks nothing, so ship 1 takes her 60 t
    // in the night and the base is empty; base 2 holds none. On day 2 force
    // 1 is routed to U at cruise speed and arrives with 800 rounds and 5
    // tubes loaded.
    let source = common::replace_once(&text(), "days = 1", "days = 2");
    let source = common::replace_once(&source, "ammunition_t = 80", "ammunition_t = 60");
    let source = common::replace_once(&source, "ammunition_t = 3", "ammunition_t = 0");
    let source = common::replace_once(
        &source,
        "gun_rounds = 100\ntask_force = 2",
        "gun_rounds = 600\ntask_force = 2",
    );
    let source = format!(
        "{source}
[[order]]
day = 2
side = \"allied\"
task_force = 1
kind = \"set_route\"
waypoints = [[5, 0]]

[[order]]
day = 2
side = \"allied\"
task_force = 1
kind = \"set_speed\"
speed = \"cruise\"
"
    );
    let run = Run::text(&source);
    assert_eq!(took(&run, 1, Pulse::Night, 1), (500, 5, 60));
    assert_eq!(run.rounds(1, 1), n(800));
    assert_eq!(run.tubes_fired(1, 1), 3);
    // She sails on day 2 and takes nothing more.
    assert!(run.rearmed().iter().all(|r| r.day == 1));
    assert_eq!(run.rounds(2, 1), n(800));
    assert_eq!(run.tubes_fired(2, 1), 3);
    let forces = &run.at(2).task_forces;
    let base_u = run.at(2).bases[&BaseId::new(2)].hex;
    assert_eq!(
        forces[&sim_core::ids::TaskForceId::new(1)].position(),
        base_u
    );
}

/// The scenario with a Japanese destroyer lying stopped at (3,0) and force 1
/// ordered east on day 2, set to engage, over 2 days.
fn fight_text(base_ammunition_t: i64, seed: u32) -> String {
    let source = common::replace_once(&text(), "days = 1", "days = 2");
    let source = common::replace_once(&source, "seed = 1942", &format!("seed = {seed}"));
    let source = common::replace_once(
        &source,
        "ammunition_t = 80",
        &format!("ammunition_t = {base_ammunition_t}"),
    );
    format!(
        "{source}
[[task_force]]
id = 4
name = \"Placeholder Japanese force 4\"
side = \"japanese\"
q = 3
r = 0
mission = \"surface_combat\"
readiness = \"battle\"

[[ship]]
id = 4
name = \"Placeholder Japanese destroyer 4\"
class = 2
side = \"japanese\"
fuel_t = 500
task_force = 4

[[order]]
day = 2
side = \"allied\"
task_force = 1
kind = \"set_route\"
waypoints = [[6, 0]]

[[order]]
day = 2
side = \"allied\"
task_force = 1
kind = \"set_speed\"
speed = \"cruise\"

[[order]]
day = 2
side = \"allied\"
task_force = 1
kind = \"set_rules_of_engagement\"
stance = \"engage\"
break_off = \"0.5\"

[[order]]
day = 2
side = \"japanese\"
task_force = 4
kind = \"set_rules_of_engagement\"
stance = \"engage\"
break_off = \"0.5\"
"
    )
}

/// Ship 1's salvos of a weapon on day 2, with their shots.
fn salvos_of_ship_1(run: &Run, wanted: Weapon) -> Vec<Fixed> {
    run.events
        .iter()
        .filter(|e| e.day == 2)
        .filter_map(|e| match e.kind {
            EventKind::Salvo {
                firer,
                weapon,
                shots,
                ..
            } if firer.raw() == 1 && weapon == wanted => Some(shots),
            _ => None,
        })
        .collect()
}

#[test]
fn she_can_fire_again_with_the_base_holding_ammunition_and_not_without() {
    let mut fights = 0;
    for seed in 1..=10u32 {
        let rearmed = Run::text(&fight_text(80, seed));
        let without = Run::text(&fight_text(0, seed));
        // The same fight with no ammunition at base 1: ship 1 still has
        // every tube fired and no torpedo ready, so she fires none, in any
        // seed; her guns fire.
        assert!(
            salvos_of_ship_1(&without, Weapon::Torpedo).is_empty(),
            "seed {seed}: no torpedo without ammunition"
        );
        assert_eq!(without.tubes_fired(1, 1), 8, "seed {seed}");
        // With it she is full at the end of day 1 (all 8 tubes loaded), and
        // whenever she fights on day 2 she fires her 8 torpedo shots, once.
        assert_eq!(rearmed.tubes_fired(1, 1), 0, "seed {seed}");
        let guns = salvos_of_ship_1(&rearmed, Weapon::Gun);
        let torpedoes = salvos_of_ship_1(&rearmed, Weapon::Torpedo);
        if guns.is_empty() {
            // No fight this seed.
            assert!(torpedoes.is_empty(), "seed {seed}");
            continue;
        }
        fights += 1;
        assert_eq!(torpedoes, vec![n(8)], "seed {seed}: 8 torpedo shots, once");
        // And the torpedoes are spent as before: no torpedo ready after.
        assert_eq!(rearmed.tubes_fired(2, 1), 8, "seed {seed}");
        assert!(!salvos_of_ship_1(&without, Weapon::Gun).is_empty() || guns.is_empty());
    }
    assert!(fights >= 5, "the fight happens in most seeds: {fights}");
}
