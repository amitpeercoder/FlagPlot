//! Scenario 51: an escort force. A force of warships with a charge keeps
//! station on it, and an enemy force must pass the escort to reach the
//! charge.
//!
//! Proves decision 0121 against a hand calculation. The data is placeholder
//! data: see `data/scenarios/51_an_escort_force.toml` and the variants
//! named below. It runs on the frozen step 1 figures.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything stated as a number
//! or a hex is asserted exactly. If the code disagrees, the code or the
//! rule is wrong: do not change a number here to make a test pass. A
//! mismatch goes back to the architect.
//!
//! ## Figures
//!
//! A hex is 40 nm. A day is a night pulse of 12 hours and two daylight
//! pulses of 6. The placeholder cargo ship: 10 kn at cruise. The
//! placeholder destroyer: 16 kn at cruise, 36 kn at most.
//!
//! ## Part 1: keeping station
//!
//! A placeholder map: S at (0,0), a coast hex with a port, and T at (5,0),
//! a coast hex with a port: 200 nm. There is no Japanese unit. The game is
//! 2 days.
//!
//! Task force 1, Allied, on the transport mission: one placeholder cargo
//! ship, at S. Task force 2, Allied, on the escort mission: two
//! placeholder destroyers, at S. The orders of day 1: force 1, a route to
//! T at 10 kn; force 2, escort force 1. Force 2 is given no route.
//!
//! Force 1 steams 200 nm at 10 kn: 20 hours. In the night pulse of day 1 it
//! makes 120 nm, which is 3 hexes: it ends the pulse at (3,0). It arrives
//! at T at hour 20, in the afternoon pulse, and ends day 1 at (5,0).
//!
//! Force 2 is in station at the start of every pulse and takes force 1's
//! route and speed:
//!
//! | End of | Force 1 | Force 2 |
//! | ------ | ------- | ------- |
//! | Day 1, night | (3,0) | (3,0) |
//! | Day 1, afternoon | (5,0), stopped at T | (5,0), stopped |
//! | Day 2, every pulse | (5,0) | (5,0) |
//!
//! Force 2 is in the hex of force 1 at the end of the morning pulse too.
//! It arrives at the hour force 1 does. It has burnt the fuel that a force
//! of the same two destroyers ordered along the same route at 10 kn burns:
//! the test runs that force beside it and compares.
//!
//! Properties of part 1:
//!
//! - **Out of station, it closes.** With force 2 starting at (0,2), 80 nm
//!   from S, and force 1 not sailing until day 2: on day 1 force 2 steams
//!   for S at 16 kn, 5 hours, and is at S and in station by the end of the
//!   night pulse. From day 2 it moves with force 1 as above.
//! - **A slow escort falls behind and follows.** With force 1 ordered at
//!   12 kn and force 2 made of a ship that can make no more than 10: force
//!   2 is not in force 1's hex at the end of the night pulse of day 1,
//!   steams for it, and is at T, stopped, by the end of day 2.
//! - **A route order to an escort with a charge is refused,** and names
//!   the charge. After the charge is cleared, a route order is taken.
//! - **The charge sunk or stood down:** force 2 has no charge from that
//!   moment and stops.
//! - An escort order naming a force of the other side, the force itself, a
//!   force that has a charge, or given to a force that is some force's
//!   charge, is refused.
//!
//! ## Part 2: who is met first
//!
//! No number of a fight is asserted here: a fight rolls dice. What is
//! asserted, on each of 200 seeds, is which fights begin and in what
//! order.
//!
//! The map of part 1. Task force 1, Allied, a cargo ship, and its escort
//! arrive at T in the night pulse of day 2 from one hex away, together. A
//! Japanese force lies stopped at T. The Allied forces are numbered so
//! that the charge has the lower number, which without the rule would put
//! its fight first.
//!
//! - **The escort holds.** Escort: four placeholder cruisers. Japanese
//!   force: one placeholder destroyer, set to break off at the first loss.
//!   On every seed a fight begins between the escort and the Japanese
//!   force before any other. On every seed on which that fight ends with
//!   the escort afloat and not the one that broke off, no fight begins
//!   between the cargo force and the Japanese force in that pulse, and the
//!   event of a meeting not held is recorded, caused by the escort order
//!   and the escort's fight. That is so on at least 150 of the 200 seeds.
//! - **The escort is passed.** Escort: one placeholder destroyer, set to
//!   break off at the first loss. Japanese force: four placeholder
//!   cruisers. On every seed the escort's fight begins first. On every
//!   seed on which it ends with the escort sunk or the one that broke off,
//!   a fight begins between the cargo force and the Japanese force in the
//!   same pulse, after it. That is so on at least 150 of the 200 seeds.
//! - **Unseen is not passed.** On every seed of either case on which the
//!   escort and the Japanese force pass unseen, no fight begins between
//!   the cargo force and the Japanese force.
//! - **Two escorts.** With a second escort force of one destroyer beside
//!   the four cruisers: the cargo force is met only on a seed on which
//!   both escorts' fights ended with the escort sunk or broken off.
//! - **Not in station, no screen.** With the escort a hex behind its
//!   charge when the charge arrives, the cargo force is met as chapter 4
//!   has it today.
//! - **No charge, no screen.** With the same forces and no escort order,
//!   the fights begin in task force number order, the cargo force first:
//!   the game as it was.
//! - **The enemy is told nothing** of the cargo force on a seed on which
//!   it was not met: nothing of it is in the Japanese side's events of the
//!   pulse.
//!
//! ## Part 3: against aircraft
//!
//! In a view built for it: a raid whose target is a cargo force with no
//! guns, with an escort of two placeholder destroyers in station. The
//! guns' shots at the raid are the two destroyers' anti-aircraft shots,
//! split before and after the attack by the share of chapter 5; with the
//! escort a hex away, or with no escort order, there are none. Every bomb
//! is aimed at a ship of the cargo force.
//!
//! # Properties
//!
//! - Every existing scenario's events and end states are what they were:
//!   no force in them has a charge.
//! - The same start and seed give the same events.

// ---------------------------------------------------------------- The harness

mod common;

use std::path::PathBuf;

use sim_core::events::{Cause, EngagementEndReason, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::EntityRef;
use sim_core::ids::{ShipId, TaskForceId};
use sim_core::orders::RejectReason;
use sim_core::side_events::SideEventKind;
use sim_core::state::{Side, TaskForceStatus};
use sim_core::time::Pulse;
use sim_data::{LoadOptions, Scenario};

const S: Hex = Hex::new(0, 0);
const T: Hex = Hex::new(5, 0);

fn path() -> PathBuf {
    common::data_dir().join("scenarios/51_an_escort_force.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

fn force(raw: u32) -> TaskForceId {
    TaskForceId::new(raw)
}

fn ship(raw: u32) -> ShipId {
    ShipId::new(raw)
}

fn hex(q: i32, r: i32) -> Hex {
    Hex::new(q, r)
}

fn load(text: &str) -> Scenario {
    common::from_text(text, &path()).expect("the scenario loads")
}

fn load_with(text: &str, options: &LoadOptions) -> Scenario {
    common::from_text_with(text, &path(), options).expect("the scenario loads")
}

fn run(mut scenario: Scenario) -> Scenario {
    scenario.run_to_end().expect("the run resolves");
    scenario
}

fn events(scenario: &Scenario) -> Vec<Event> {
    scenario.world.events().iter().cloned().collect()
}

fn order_of(pulse: Pulse) -> u32 {
    match pulse {
        Pulse::Night => 0,
        Pulse::Morning => 1,
        Pulse::Afternoon => 2,
    }
}

/// The hex a force is in at the end of a pulse, from the log: where its last
/// steaming at or before the pulse left it, or `start`.
fn position_after(log: &[Event], start: Hex, id: u32, day: u32, pulse: Pulse) -> Hex {
    let mut at = start;
    for event in log {
        let Some(p) = event.pulse else { continue };
        if (event.day, order_of(p)) > (day, order_of(pulse)) {
            break;
        }
        if let EventKind::TaskForceSteamed { task_force, to, .. } = &event.kind {
            if *task_force == force(id) {
                at = *to;
            }
        }
    }
    at
}

const PULSES: [Pulse; 3] = [Pulse::Night, Pulse::Morning, Pulse::Afternoon];

fn rejections(log: &[Event]) -> Vec<(u32, RejectReason)> {
    log.iter()
        .filter_map(|e| match &e.kind {
            EventKind::OrderRejected { reason, .. } => Some((e.day, reason.clone())),
            _ => None,
        })
        .collect()
}

fn station_kept(log: &[Event]) -> Vec<&Event> {
    log.iter()
        .filter(|e| matches!(e.kind, EventKind::StationKept { .. }))
        .collect()
}

fn fuel(scenario: &Scenario, raw: u32) -> Fixed {
    scenario.world.umpire().ships[&ship(raw)].fuel_t
}

/// A `[[order]]` table of the Allied side.
fn order(day: u32, target: &str, body: &str) -> String {
    format!("\n[[order]]\nday = {day}\nside = \"allied\"\n{target}\n{body}\n")
}

/// The escort order of the main file, as it is written there.
const ESCORT_ORDER: &str =
    "[[order]]\nday = 1\nside = \"allied\"\ntask_force = 2\nkind = \"escort\"\ncharge = 1\n";

// ------------------------------------------------------------------ Part 1

#[test]
fn the_escort_keeps_station_over_the_passage() {
    let scenario = run(load(&text()));
    let log = events(&scenario);

    // Both forces end every pulse of both days in one hex: (3,0) after the
    // night of day 1 and (5,0) after its afternoon and from then on. The
    // morning's hex is the same for both.
    assert_eq!(position_after(&log, S, 1, 1, Pulse::Night), hex(3, 0));
    assert_eq!(position_after(&log, S, 2, 1, Pulse::Night), hex(3, 0));
    assert_eq!(position_after(&log, S, 1, 1, Pulse::Afternoon), T);
    assert_eq!(position_after(&log, S, 2, 1, Pulse::Afternoon), T);
    for day in 1..=2 {
        for pulse in PULSES {
            assert_eq!(
                position_after(&log, S, 1, day, pulse),
                position_after(&log, S, 2, day, pulse),
                "day {day} {pulse:?}"
            );
        }
    }
    for pulse in PULSES {
        assert_eq!(position_after(&log, S, 2, 2, pulse), T, "day 2 {pulse:?}");
    }

    // Both steam 120 nm in the night at 10 kn.
    for id in [1, 2] {
        let night: Vec<_> = log
            .iter()
            .filter(|e| e.day == 1 && e.pulse == Some(Pulse::Night))
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceSteamed {
                    task_force,
                    distance_nm,
                    speed_kn,
                    ..
                } if *task_force == force(id) => Some((*distance_nm, *speed_kn)),
                _ => None,
            })
            .collect();
        assert_eq!(night, vec![(fx("120"), fx("10"))], "force {id}");
    }

    // They arrive at T at the same hour, hour 20, in the afternoon pulse.
    let arrivals: Vec<(TaskForceId, Hex, Fixed, Option<Pulse>)> = log
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::TaskForceArrived {
                task_force,
                hex,
                elapsed_hours,
            } => Some((*task_force, *hex, *elapsed_hours, e.pulse)),
            _ => None,
        })
        .collect();
    assert_eq!(
        arrivals,
        vec![
            (force(1), T, fx("20"), Some(Pulse::Afternoon)),
            (force(2), T, fx("20"), Some(Pulse::Afternoon)),
        ]
    );

    // Force 2 was never given a route, kept station once, in station, and
    // nothing was refused.
    assert!(rejections(&log).is_empty());
    let kept = station_kept(&log);
    assert_eq!(kept.len(), 1);
    match &kept[0].kind {
        EventKind::StationKept {
            escort,
            charge,
            in_station,
            waypoints,
            knots,
        } => {
            assert_eq!((*escort, *charge, *in_station), (force(2), force(1), true));
            assert_eq!(waypoints, &vec![T]);
            assert_eq!(*knots, Some(fx("10")));
        }
        other => panic!("not a station kept: {other:?}"),
    }
    assert_eq!((kept[0].day, kept[0].pulse), (1, Some(Pulse::Night)));
    // It cites the escort order and the charge's route order.
    assert_eq!(kept[0].causes.len(), 2);
    assert!(kept[0].causes.iter().all(|c| matches!(c, Cause::Order(_))));

    // The state: force 2 has force 1 as its charge, and is stopped at T.
    let truth = scenario.world.umpire();
    assert_eq!(
        truth.task_forces[&force(2)].charge.map(|c| c.force),
        Some(force(1))
    );
    assert_eq!(truth.task_forces[&force(2)].position(), T);
    assert_eq!(
        truth.task_forces[&force(2)].status,
        TaskForceStatus::Arrived
    );
    assert_eq!(
        truth.task_forces[&force(1)].status,
        TaskForceStatus::Arrived
    );
}

#[test]
fn the_escort_burns_the_fuel_a_force_ordered_the_same_route_burns() {
    let station = run(load(&text()));
    // The same two destroyers, ordered along the same route at 10 kn.
    let route = format!(
        "{}{}",
        order(
            1,
            "task_force = 2",
            "kind = \"set_route\"\nwaypoints = [[5, 0]]"
        ),
        order(1, "task_force = 2", "kind = \"set_speed\"\nknots = \"10\"")
    );
    let ordered_text = changed(&text(), ESCORT_ORDER, "") + &route;
    let ordered = run(load(&ordered_text));
    for id in [2, 3] {
        assert_eq!(fuel(&station, id), fuel(&ordered, id), "ship {id}");
        assert!(fuel(&station, id) < fx("500"));
    }
    let log = events(&ordered);
    let station_log = events(&station);
    for pulse in PULSES {
        assert_eq!(
            position_after(&log, S, 2, 1, pulse),
            position_after(&station_log, S, 2, 1, pulse)
        );
    }
}

#[test]
fn out_of_station_it_closes_on_its_charge_and_then_moves_with_it() {
    // Force 2 starts at (0,2), 80 nm from S; force 1 does not sail until day
    // 2.
    let moved_route = changed(
        &text(),
        "day = 1\nside = \"allied\"\ntask_force = 1\nkind = \"set_route\"",
        "day = 2\nside = \"allied\"\ntask_force = 1\nkind = \"set_route\"",
    );
    let moved_speed = changed(
        &moved_route,
        "day = 1\nside = \"allied\"\ntask_force = 1\nkind = \"set_speed\"",
        "day = 2\nside = \"allied\"\ntask_force = 1\nkind = \"set_speed\"",
    );
    let behind = changed(
        &moved_speed,
        "q = 0\nr = 0\nmission = \"escort\"",
        "q = 0\nr = 2\nmission = \"escort\"",
    );
    let scenario = run(load(&behind));
    let log = events(&scenario);
    let start = hex(0, 2);

    // Day 1: 80 nm at 16 kn, 5 hours, and it is at S and in station by the
    // end of the night.
    let steamed: Vec<_> = log
        .iter()
        .filter(|e| e.day == 1)
        .filter_map(|e| match &e.kind {
            EventKind::TaskForceSteamed {
                task_force,
                from,
                to,
                distance_nm,
                speed_kn,
                ..
            } if *task_force == force(2) => Some((e.pulse, *from, *to, *distance_nm, *speed_kn)),
            _ => None,
        })
        .collect();
    assert_eq!(
        steamed,
        vec![(Some(Pulse::Night), start, S, fx("80"), fx("16"))]
    );
    assert_eq!(position_after(&log, start, 2, 1, Pulse::Night), S);
    assert_eq!(position_after(&log, S, 1, 1, Pulse::Night), S);
    let arrived: Vec<_> = log
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::TaskForceArrived {
                task_force,
                hex,
                elapsed_hours,
            } if *task_force == force(2) && e.day == 1 => Some((*hex, *elapsed_hours)),
            _ => None,
        })
        .collect();
    assert_eq!(arrived, vec![(S, fx("5"))]);
    let kept = station_kept(&log);
    assert_eq!(kept.len(), 2);
    match &kept[0].kind {
        EventKind::StationKept {
            in_station,
            waypoints,
            knots,
            ..
        } => {
            assert!(!*in_station);
            assert_eq!(waypoints, &vec![S]);
            assert_eq!(*knots, None);
        }
        other => panic!("{other:?}"),
    }
    assert_eq!((kept[0].day, kept[0].pulse), (1, Some(Pulse::Night)));

    // From day 2 it moves with force 1 as in part 1.
    for pulse in PULSES {
        assert_eq!(
            position_after(&log, start, 2, 1, pulse),
            S,
            "day 1 {pulse:?}"
        );
    }
    assert_eq!(position_after(&log, S, 1, 2, Pulse::Night), hex(3, 0));
    assert_eq!(position_after(&log, start, 2, 2, Pulse::Night), hex(3, 0));
    assert_eq!(position_after(&log, S, 1, 2, Pulse::Afternoon), T);
    assert_eq!(position_after(&log, start, 2, 2, Pulse::Afternoon), T);
    match &kept[1].kind {
        EventKind::StationKept {
            in_station, knots, ..
        } => {
            assert!(*in_station);
            assert_eq!(*knots, Some(fx("10")));
        }
        other => panic!("{other:?}"),
    }
    assert_eq!((kept[1].day, kept[1].pulse), (2, Some(Pulse::Night)));
}

#[test]
fn a_slow_escort_falls_behind_and_follows() {
    // Force 1 at 12 kn; the destroyers can make no more than 10.
    let options = common::altered_reference("slow_escort", |dir| {
        common::edit_file_by(
            dir,
            "ship_classes.csv",
            &[("destroyer,16,36,500,4000", "destroyer,10,10,500,4000")],
        );
    });
    let fast_charge = changed(&text(), "knots = \"10\"", "knots = \"12\"");
    let scenario = run(load_with(&fast_charge, &options));
    let log = events(&scenario);

    // At the end of the night of day 1 force 2 is not in force 1's hex.
    let charge_at = position_after(&log, S, 1, 1, Pulse::Night);
    let escort_at = position_after(&log, S, 2, 1, Pulse::Night);
    assert_ne!(charge_at, escort_at);
    // It steams for it: a station kept out of station follows.
    let out: Vec<_> = station_kept(&log)
        .into_iter()
        .filter(|e| {
            matches!(
                e.kind,
                EventKind::StationKept {
                    in_station: false,
                    ..
                }
            )
        })
        .collect();
    assert!(!out.is_empty(), "it was routed for its charge's hex");
    assert_eq!((out[0].day, out[0].pulse), (1, Some(Pulse::Morning)));
    // And is at T, stopped, by the end of day 2.
    assert_eq!(position_after(&log, S, 2, 2, Pulse::Afternoon), T);
    let truth = scenario.world.umpire();
    assert_eq!(truth.task_forces[&force(2)].position(), T);
    assert_eq!(
        truth.task_forces[&force(2)].status,
        TaskForceStatus::Arrived
    );
    assert_eq!(truth.task_forces[&force(1)].position(), T);
}

#[test]
fn a_route_order_to_an_escort_with_a_charge_is_refused_until_it_is_cleared() {
    // Day 1: force 2 is given its charge (the file's order) and then a route
    // order. Day 2: its charge is cleared, then a route order is taken.
    let extra = format!(
        "{}{}{}",
        order(
            1,
            "task_force = 2",
            "kind = \"set_route\"\nwaypoints = [[0, 0]]"
        ),
        order(2, "task_force = 2", "kind = \"escort\""),
        order(
            2,
            "task_force = 2",
            "kind = \"set_route\"\nwaypoints = [[0, 0]]"
        ),
    );
    let scenario = run(load(&(text() + &extra)));
    let log = events(&scenario);
    assert_eq!(
        rejections(&log),
        vec![(1, RejectReason::KeepsStationOn(force(1)))]
    );
    // Force 2 kept station on day 1, and after the clearing it steamed home:
    // 200 nm at 16 kn, 12.5 hours, is at S by the end of the night's
    // 12 hours plus the morning.
    assert_eq!(position_after(&log, S, 2, 1, Pulse::Afternoon), T);
    let truth = scenario.world.umpire();
    assert_eq!(truth.task_forces[&force(2)].charge, None);
    assert_eq!(truth.task_forces[&force(2)].position(), S);
    assert_eq!(position_after(&log, S, 1, 2, Pulse::Afternoon), T);
}

#[test]
fn a_charge_stood_down_leaves_its_escort_with_no_charge_and_stopped() {
    // A base at T. Force 1 arrives on day 1; on day 2 it is stood down.
    let base = "\n[[base]]\nid = 2\nname = \"Placeholder base T\"\nside = \"allied\"\nq = 5\nr = 0\nsupply_t = 0\n";
    let stand_down = order(2, "task_force = 1", "kind = \"stand_down\"");
    let scenario = run(load(&(text() + base + &stand_down)));
    let log = events(&scenario);
    let lost: Vec<&Event> = log
        .iter()
        .filter(|e| matches!(e.kind, EventKind::ChargeLost { .. }))
        .collect();
    assert_eq!(lost.len(), 1);
    assert_eq!(
        lost[0].kind,
        EventKind::ChargeLost {
            escort: force(2),
            charge: force(1),
            hex: T,
        }
    );
    // At the intake of day 2's orders, where the stand-down is taken, citing
    // the escort order.
    assert_eq!((lost[0].day, lost[0].pulse), (2, None));
    assert!(lost[0].causes.iter().any(|c| matches!(c, Cause::Order(_))));
    let truth = scenario.world.umpire();
    let escort = &truth.task_forces[&force(2)];
    assert_eq!(escort.charge, None);
    assert_eq!(escort.position(), T);
    assert_ne!(escort.status, TaskForceStatus::Underway);
    assert!(escort.route.waypoints.is_empty());
    assert!(rejections(&log).is_empty());
}

#[test]
fn escort_orders_naming_the_wrong_force_are_refused() {
    // Added to the file: force 3 (escort mission), force 4 (escort mission,
    // to be a charge), force 5 (Japanese, far off) and force 6 (transport).
    let more = "
[[task_force]]
id = 3
name = \"Placeholder escort force 3\"
side = \"allied\"
q = 0
r = 0
mission = \"escort\"

[[task_force]]
id = 4
name = \"Placeholder escort force 4\"
side = \"allied\"
q = 0
r = 0
mission = \"escort\"

[[task_force]]
id = 5
name = \"Placeholder Japanese force 5\"
side = \"japanese\"
q = 7
r = 0
mission = \"surface_combat\"

[[task_force]]
id = 6
name = \"Placeholder transport force 6\"
side = \"allied\"
q = 0
r = 0
mission = \"transport\"

[[ship]]
id = 4
name = \"Placeholder destroyer 4\"
class = 2
side = \"allied\"
fuel_t = 500
task_force = 3

[[ship]]
id = 5
name = \"Placeholder destroyer 5\"
class = 2
side = \"allied\"
fuel_t = 500
task_force = 4

[[ship]]
id = 6
name = \"Placeholder destroyer 6\"
class = 2
side = \"japanese\"
fuel_t = 500
task_force = 5

[[ship]]
id = 7
name = \"Placeholder cargo ship 7\"
class = 3
side = \"allied\"
fuel_t = 1000
task_force = 6
";
    let orders = [
        // Naming itself.
        order(1, "task_force = 2", "kind = \"escort\"\ncharge = 2"),
        // Naming a force that has a charge: force 2 has force 1.
        order(1, "task_force = 3", "kind = \"escort\"\ncharge = 2"),
        // Naming a force of the other side.
        order(1, "task_force = 3", "kind = \"escort\"\ncharge = 5"),
        // Given to a force not on the escort mission.
        order(1, "task_force = 6", "kind = \"escort\"\ncharge = 1"),
        // Accepted: force 3 takes force 4 as its charge.
        order(1, "task_force = 3", "kind = \"escort\"\ncharge = 4"),
        // Given to a force that is some force's charge.
        order(1, "task_force = 4", "kind = \"escort\"\ncharge = 1"),
    ]
    .concat();
    let scenario = run(load(&(text() + more + &orders)));
    let log = events(&scenario);
    // Intake takes the orders by the force they are for, so the last two are
    // refused in the order of forces 4 and 6, not of the file.
    assert_eq!(
        rejections(&log),
        vec![
            (1, RejectReason::CannotEscortItself),
            (1, RejectReason::ChargeIsAnEscort(force(2))),
            (1, RejectReason::NotYourUnit),
            (1, RejectReason::IsACharge { escort: force(3) }),
            (1, RejectReason::NotOnEscortMission),
        ]
    );
    let truth = scenario.world.umpire();
    let charge = |raw: u32| truth.task_forces[&force(raw)].charge.map(|c| c.force);
    assert_eq!(charge(2), Some(force(1)));
    assert_eq!(charge(3), Some(force(4)));
    assert_eq!(charge(4), None);
    assert_eq!(charge(6), None);
}

// ------------------------------------------------------------------ Part 2

const SEEDS: u64 = 200;
const ESCORT_SCREENS_AT_LEAST: usize = 150;

/// The forces of one case of part 2. Task force 1 is the cargo force (one
/// cargo ship, ship 1), charge of the escort forces, numbered 2 and up, one
/// for each entry of `escorts` (the classes of its ships); the Japanese
/// force, stopped at T, has the next number. Every ship's id follows the last
/// force's, in force order.
struct Case {
    escorts: Vec<Vec<u32>>,
    japanese: Vec<u32>,
    /// Whether the escorts are given their charge, and on which day. A
    /// charge given on day 2 is given in time for the night of the arrival
    /// only: an escort that starts out of station has had no day to close.
    charged: bool,
    charge_day: u32,
    /// Whether each escort force is given a route to T at 10 kn on day 2, as
    /// the cargo force is: the same arrival with no charge.
    own_route: bool,
    /// Where each escort force starts. The cargo force starts at (4,0), one
    /// hex from T.
    escort_at: Hex,
}

impl Case {
    fn new(escorts: &[&[u32]], japanese: &[u32]) -> Case {
        Case {
            escorts: escorts.iter().map(|classes| classes.to_vec()).collect(),
            japanese: japanese.to_vec(),
            charged: true,
            charge_day: 1,
            own_route: false,
            escort_at: hex(4, 0),
        }
    }

    fn japanese_force(&self) -> u32 {
        2 + u32::try_from(self.escorts.len()).expect("few forces")
    }

    /// The scenario text. Two days: the forces lie one hex from T on day 1
    /// and the cargo force is ordered to T on day 2, so that the cargo force
    /// and its escorts arrive together in the night pulse of day 2, 4 hours
    /// into it, at 10 kn.
    fn text(&self) -> String {
        let mut text = String::from(
            "# Scenario 51, part 2: who is met first. Built by the test.\n\
             name = \"Placeholder scenario 51, part 2\"\n\
             placeholder = true\n\
             seed = 1942\n\
             days = 2\n\
             map = \"../map/placeholder_one_convoy.toml\"\n\
             ship_classes = \"../reference/ship_classes.csv\"\n\
             port_sizes = \"../reference/port_sizes.csv\"\n\
             aircraft_types = \"../reference/aircraft_types.csv\"\n\
             search = \"../reference/search.toml\"\n\
             surface_combat = \"../reference/surface_combat.toml\"\n",
        );
        let force_table = |id: u32, side: &str, mission: &str, at: Hex| {
            format!(
                "\n[[task_force]]\nid = {id}\nname = \"Placeholder force {id}\"\nside = \"{side}\"\nq = {}\nr = {}\nmission = \"{mission}\"\n",
                at.q, at.r
            )
        };
        let ship_table = |id: u32, class: u32, side: &str, force: u32, fuel: u32| {
            format!(
                "\n[[ship]]\nid = {id}\nname = \"Placeholder ship {id}\"\nclass = {class}\nside = \"{side}\"\nfuel_t = {fuel}\ntask_force = {force}\n"
            )
        };
        text += &force_table(1, "allied", "transport", hex(4, 0));
        let mut next_ship = 2;
        let mut ships = ship_table(1, 3, "allied", 1, 1000);
        for (index, classes) in self.escorts.iter().enumerate() {
            let id = 2 + u32::try_from(index).expect("few forces");
            text += &force_table(id, "allied", "escort", self.escort_at);
            for class in classes {
                ships += &ship_table(next_ship, *class, "allied", id, 500);
                next_ship += 1;
            }
        }
        let japanese = self.japanese_force();
        text += &force_table(japanese, "japanese", "surface_combat", T);
        for class in &self.japanese {
            ships += &ship_table(next_ship, *class, "japanese", japanese, 500);
            next_ship += 1;
        }
        text += &ships;
        if self.charged {
            for index in 0..self.escorts.len() {
                let id = 2 + u32::try_from(index).expect("few forces");
                text += &order(
                    self.charge_day,
                    &format!("task_force = {id}"),
                    "kind = \"escort\"\ncharge = 1",
                );
            }
        }
        // The one-ship forces break off at the first loss: of one ship, 0.5
        // is reached by one lost.
        let single: Vec<u32> = self
            .escorts
            .iter()
            .enumerate()
            .filter(|(_, classes)| classes.len() == 1)
            .map(|(index, _)| 2 + u32::try_from(index).expect("few forces"))
            .collect();
        for id in single {
            text += &order(
                1,
                &format!("task_force = {id}"),
                "kind = \"set_rules_of_engagement\"\nstance = \"engage\"\nbreak_off = \"0.5\"",
            );
        }
        if self.japanese.len() == 1 {
            text += &format!(
                "\n[[order]]\nday = 1\nside = \"japanese\"\ntask_force = {japanese}\nkind = \"set_rules_of_engagement\"\nstance = \"engage\"\nbreak_off = \"0.5\"\n"
            );
        }
        text += &order(
            2,
            "task_force = 1",
            "kind = \"set_route\"\nwaypoints = [[5, 0]]",
        );
        if self.own_route {
            for index in 0..self.escorts.len() {
                let id = 2 + u32::try_from(index).expect("few forces");
                text += &order(
                    2,
                    &format!("task_force = {id}"),
                    "kind = \"set_route\"\nwaypoints = [[5, 0]]",
                );
                text += &order(
                    2,
                    &format!("task_force = {id}"),
                    "kind = \"set_speed\"\nknots = \"10\"",
                );
            }
        }
        text
    }
}

/// One game of a case on one seed.
struct Game {
    seed: u64,
    log: Vec<Event>,
    scenario: Scenario,
    /// How many ships each force began with.
    size: std::collections::BTreeMap<u32, usize>,
}

fn play(case: &Case, seed: u64) -> Game {
    let base = load(&case.text());
    let size = base
        .world
        .umpire()
        .task_forces
        .iter()
        .map(|(id, f)| (id.raw(), f.ships.len()))
        .collect();
    let mut scenario = base.with_seed(seed).expect("a seed can be set");
    scenario.run_to_end().expect("the run resolves");
    Game {
        seed,
        log: events(&scenario),
        scenario,
        size,
    }
}

/// One fight of the night pulse of day 2, as the log has it.
#[derive(Clone, Debug)]
struct Fight {
    a: u32,
    b: u32,
    begun: usize,
    ended: usize,
    reason: EngagementEndReason,
}

impl Game {
    fn night(&self) -> impl Iterator<Item = (usize, &Event)> {
        self.log
            .iter()
            .enumerate()
            .filter(|(_, e)| e.day == 2 && e.pulse == Some(Pulse::Night))
    }

    /// The fights that begin in the night of day 2, in the order they begin.
    fn fights(&self) -> Vec<Fight> {
        let mut begun: Vec<(u32, u32, usize)> = Vec::new();
        let mut fights = Vec::new();
        for (index, event) in self.night() {
            match &event.kind {
                EventKind::EngagementBegun { a, b, .. } => begun.push((a.raw(), b.raw(), index)),
                EventKind::EngagementEnded { a, b, reason, .. } => {
                    let at = begun
                        .iter()
                        .position(|(x, y, _)| (*x, *y) == (a.raw(), b.raw()))
                        .expect("a fight that ended began");
                    let (x, y, start) = begun.remove(at);
                    fights.push(Fight {
                        a: x,
                        b: y,
                        begun: start,
                        ended: index,
                        reason: *reason,
                    });
                }
                _ => {}
            }
        }
        fights.sort_by_key(|f| f.begun);
        fights
    }

    /// The fights that begin in the night, as pairs, in order.
    fn pairs(&self) -> Vec<(u32, u32)> {
        self.fights().iter().map(|f| (f.a, f.b)).collect()
    }

    fn fought(&self, a: u32, b: u32) -> Option<Fight> {
        self.fights()
            .into_iter()
            .find(|f| (f.a, f.b) == (a.min(b), a.max(b)))
    }

    /// Whether force `id` had a ship afloat when the log reached `upto`.
    fn afloat_at(&self, id: u32, upto: usize) -> bool {
        let sunk = self.log[..upto]
            .iter()
            .filter(|e| {
                matches!(&e.kind, EventKind::ShipSunk { task_force, .. } if *task_force == force(id))
            })
            .count();
        sunk < self.size[&id]
    }

    /// Whether `id` left `fight`: it had no ship afloat when the fight
    /// ended, or it was the force that broke off or slipped away.
    fn left(&self, id: u32, fight: &Fight) -> bool {
        let went = self.log[fight.begun..=fight.ended].iter().any(|e| {
            matches!(
                &e.kind,
                EventKind::ForceBrokeOff { task_force, .. }
                | EventKind::ForceSlippedAway { task_force, .. }
                    if *task_force == force(id)
            )
        });
        went || !self.afloat_at(id, fight.ended)
    }

    fn not_held(&self) -> Vec<(usize, &Event)> {
        self.night()
            .filter(|(_, e)| matches!(e.kind, EventKind::MeetingNotHeld { .. }))
            .collect()
    }

    /// The order that gave force `id` its charge.
    fn charge_order(&self, id: u32) -> sim_core::ids::OrderId {
        self.scenario.world.umpire().task_forces[&force(id)]
            .charge
            .expect("it has a charge")
            .order
    }

    /// The events of the night of day 2 a side may see.
    fn side_night(&self, side: Side) -> Vec<sim_core::side_events::SideEvent> {
        self.scenario
            .world
            .side_view(side)
            .events()
            .into_iter()
            .filter(|e| e.day == 2 && e.pulse == Some(Pulse::Night))
            .collect()
    }
}

fn games(case: &Case) -> Vec<Game> {
    (1..=SEEDS).map(|seed| play(case, seed)).collect()
}

/// The four-cruiser escort against a Japanese destroyer: force 2 is the
/// escort, force 3 the Japanese force.
fn holding() -> Case {
    Case::new(&[&[1, 1, 1, 1]], &[2])
}

/// The one-destroyer escort against four Japanese cruisers.
fn passed() -> Case {
    Case::new(&[&[2]], &[1, 1, 1, 1])
}

#[test]
fn the_escort_holds_and_the_cargo_force_is_not_met() {
    let mut screened = 0;
    for game in games(&holding()) {
        let seed = game.seed;
        // The escort's fight begins before any other.
        assert_eq!(game.pairs()[0], (2, 3), "seed {seed}");
        let escort = game.fought(2, 3).expect("the escort's fight");
        if game.left(2, &escort) {
            // The escort was passed: the cargo force is met after it.
            let cargo = game.fought(1, 3).expect("the cargo force is met");
            assert!(cargo.begun > escort.ended, "seed {seed}");
        } else {
            // It was afloat and did not leave: no fight begins between the
            // cargo force and the Japanese force in that pulse.
            assert!(game.fought(1, 3).is_none(), "seed {seed}");
            screened += 1;
        }
    }
    assert!(
        screened >= ESCORT_SCREENS_AT_LEAST,
        "screened on {screened}"
    );
}

/// On every seed on which the escort's fight ends with the escort afloat and
/// not the one that broke off, the event of a meeting not held is recorded,
/// caused by the escort order and the escort's fight, on at least 150 of the
/// 200 seeds. That includes the seeds on which the escort sank the Japanese
/// force: the meeting with the cargo force is then one not held.
#[test]
fn a_meeting_not_held_is_recorded_with_the_escort_order_and_its_fight() {
    let mut recorded = 0;
    for game in games(&holding()) {
        let seed = game.seed;
        let escort = game.fought(2, 3).expect("the escort's fight");
        if game.left(2, &escort) {
            continue;
        }
        let held = game.not_held();
        assert_eq!(held.len(), 1, "seed {seed}");
        let (_, event) = held[0];
        assert_eq!(
            event.kind,
            EventKind::MeetingNotHeld {
                charge: force(1),
                enemy: force(3),
                hex: T,
                escorts: vec![force(2)],
            },
            "seed {seed}"
        );
        assert_eq!(
            event.causes,
            vec![
                Cause::Order(game.charge_order(2)),
                Cause::Event(game.log[escort.ended].id)
            ],
            "seed {seed}"
        );
        recorded += 1;
    }
    assert!(
        recorded >= ESCORT_SCREENS_AT_LEAST,
        "recorded on {recorded}"
    );
}

/// What the code does record: on every seed on which the escort's fight
/// leaves the Japanese force afloat and the escort has not left, the meeting
/// with the cargo force is not held, and the event cites the escort order and
/// the end of the escort's fight.
#[test]
fn a_meeting_not_held_cites_the_escort_order_and_its_fight() {
    let mut recorded = 0;
    for game in games(&holding()) {
        let seed = game.seed;
        let escort = game.fought(2, 3).expect("the escort's fight");
        if game.left(2, &escort) || !game.afloat_at(3, escort.ended) {
            continue;
        }
        let held = game.not_held();
        assert_eq!(held.len(), 1, "seed {seed}");
        let (index, event) = held[0];
        assert_eq!(
            event.kind,
            EventKind::MeetingNotHeld {
                charge: force(1),
                enemy: force(3),
                hex: T,
                escorts: vec![force(2)],
            },
            "seed {seed}"
        );
        assert_eq!(
            event.causes,
            vec![
                Cause::Order(game.charge_order(2)),
                Cause::Event(game.log[escort.ended].id)
            ],
            "seed {seed}"
        );
        assert!(index > escort.ended, "seed {seed}");
        recorded += 1;
    }
    assert!(recorded > 0);
}

#[test]
fn the_escort_is_passed_and_the_cargo_force_is_met_after_it() {
    let mut passed_on = 0;
    for game in games(&passed()) {
        let seed = game.seed;
        assert_eq!(game.pairs()[0], (2, 3), "seed {seed}");
        let escort = game.fought(2, 3).expect("the escort's fight");
        if game.left(2, &escort) {
            // Sunk or the one that broke off: the cargo force is met in the
            // same pulse, after it.
            let cargo = game.fought(1, 3).expect("the cargo force is met");
            assert!(cargo.begun > escort.ended, "seed {seed}");
            assert_eq!(game.pairs(), vec![(2, 3), (1, 3)], "seed {seed}");
            assert!(game.not_held().is_empty(), "seed {seed}");
            passed_on += 1;
        } else {
            assert!(game.fought(1, 3).is_none(), "seed {seed}");
        }
    }
    assert!(
        passed_on >= ESCORT_SCREENS_AT_LEAST,
        "passed on {passed_on}"
    );
}

/// The architect's ruling of Nov 16, 2026 on rule 4 of decision 0121: an
/// escort that ended its fight with no ship afloat, sunk to the last ship
/// and not the force that broke off, has been passed, and the charge is met
/// in the same pulse, after it. Asserted on every seed on which the
/// one-destroyer escort is sunk outright, which happens on some.
#[test]
fn an_escort_sunk_to_the_last_ship_is_passed() {
    let mut sunk_on = 0;
    for game in games(&passed()) {
        let seed = game.seed;
        let escort = game.fought(2, 3).expect("the escort's fight");
        let broke_off = game.log[escort.begun..=escort.ended].iter().any(|e| {
            matches!(
                &e.kind,
                EventKind::ForceBrokeOff { task_force, .. }
                | EventKind::ForceSlippedAway { task_force, .. }
                    if *task_force == force(2)
            )
        });
        if broke_off || game.afloat_at(2, escort.ended) {
            continue;
        }
        sunk_on += 1;
        let cargo = game.fought(1, 3).expect("the cargo force is met");
        assert!(cargo.begun > escort.ended, "seed {seed}");
        assert!(game.not_held().is_empty(), "seed {seed}");
    }
    assert!(sunk_on > 0, "no seed sank the escort outright");
}

#[test]
fn unseen_is_not_passed() {
    for case in [holding(), passed()] {
        let mut unseen = 0;
        for game in games(&case) {
            let seed = game.seed;
            let escort = game.fought(2, 3).expect("the escort's fight");
            if escort.reason != EngagementEndReason::PassedUnseen {
                continue;
            }
            unseen += 1;
            // Neither sighted the other: the escort has not been passed, the
            // charge is not met and the meeting is recorded as not held.
            assert!(game.fought(1, 3).is_none(), "seed {seed}");
            let held = game.not_held();
            assert_eq!(held.len(), 1, "seed {seed}");
            assert!(held[0].0 > escort.ended, "seed {seed}");
        }
        assert!(unseen > 0, "no seed passed unseen");
    }
}

#[test]
fn two_escorts_the_cargo_force_is_met_only_past_both() {
    // The four cruisers (force 2) and a destroyer (force 3); the Japanese
    // force is force 4.
    let case = Case::new(&[&[1, 1, 1, 1], &[2]], &[2]);
    for game in games(&case) {
        let seed = game.seed;
        let both_left = [2, 3].iter().all(|id| {
            game.fought(*id, 4)
                .is_some_and(|fight| game.left(*id, &fight))
        });
        assert_eq!(game.fought(1, 4).is_some(), both_left, "seed {seed}");
    }
    // The same with two destroyers against four cruisers, where the escorts
    // are passed one after the other and the cargo force is met last.
    let case = Case::new(&[&[2], &[2]], &[1, 1, 1, 1]);
    let mut passed_both = 0;
    for game in games(&case) {
        let seed = game.seed;
        let both_left = [2, 3].iter().all(|id| {
            game.fought(*id, 4)
                .is_some_and(|fight| game.left(*id, &fight))
        });
        assert_eq!(game.fought(1, 4).is_some(), both_left, "seed {seed}");
        if both_left {
            assert_eq!(game.pairs(), vec![(2, 4), (3, 4), (1, 4)], "seed {seed}");
            passed_both += 1;
        }
    }
    assert!(passed_both > 0);
}

#[test]
fn not_in_station_there_is_no_screen() {
    // The escort is a hex behind its charge when the charge arrives.
    let mut case = holding();
    case.escort_at = hex(3, 0);
    case.charge_day = 2;
    for game in games(&case) {
        let seed = game.seed;
        assert_eq!(game.pairs()[0], (1, 3), "seed {seed}");
        assert!(game.fought(2, 3).is_none(), "seed {seed}");
        assert!(game.not_held().is_empty(), "seed {seed}");
    }
}

#[test]
fn with_no_charge_the_fights_begin_in_task_force_number_order() {
    // The same forces and no escort order; the escort is ordered to T at 10
    // kn on day 2 so that it arrives with the cargo force.
    let mut case = holding();
    case.charged = false;
    case.own_route = true;
    for game in games(&case) {
        let seed = game.seed;
        let pairs = game.pairs();
        assert_eq!(pairs[0], (1, 3), "seed {seed}");
        let cargo = game.fought(1, 3).expect("the cargo force's fight");
        if game.afloat_at(3, cargo.ended) {
            assert_eq!(pairs, vec![(1, 3), (2, 3)], "seed {seed}");
        }
        assert!(game.not_held().is_empty(), "seed {seed}");
    }
}

#[test]
fn the_enemy_is_told_nothing_of_a_force_it_did_not_meet() {
    let mut not_met = 0;
    let mut told_to_the_charges_side = 0;
    for game in games(&holding()) {
        let seed = game.seed;
        let escort = game.fought(2, 3).expect("the escort's fight");
        if game.left(2, &escort) {
            continue;
        }
        assert!(game.fought(1, 3).is_none(), "seed {seed}");
        not_met += 1;
        // Nothing of the cargo force, its ship or the meeting is in the
        // Japanese side's events of the pulse.
        for event in game.side_night(Side::Japanese) {
            let entities = event.kind.entities();
            assert!(
                !entities.contains(&EntityRef::TaskForce(force(1)))
                    && !entities.contains(&EntityRef::Ship(ship(1))),
                "seed {seed}: {event:?}"
            );
            assert!(
                !matches!(
                    &event.kind,
                    SideEventKind::Whole(EventKind::MeetingNotHeld { .. })
                ),
                "seed {seed}"
            );
        }
        // The charge's side is told of a meeting not held.
        let allied: Vec<_> = game
            .side_night(Side::Allied)
            .into_iter()
            .filter(|e| {
                matches!(
                    &e.kind,
                    SideEventKind::Whole(EventKind::MeetingNotHeld { .. })
                )
            })
            .collect();
        assert_eq!(allied.len(), game.not_held().len(), "seed {seed}");
        told_to_the_charges_side += allied.len();
    }
    assert_eq!(not_met, usize::try_from(SEEDS).expect("small"));
    assert!(told_to_the_charges_side > 0);
}

#[test]
fn the_same_start_and_seed_give_the_same_events() {
    for case in [holding(), passed()] {
        let a = play(&case, 7);
        let b = play(&case, 7);
        assert_eq!(a.log, b.log);
        assert_eq!(a.scenario.world.day_hashes(), b.scenario.world.day_hashes());
    }
}

#[test]
fn a_charge_sunk_leaves_its_escort_with_no_charge_from_that_moment() {
    let mut checked = 0;
    for game in games(&passed()) {
        let seed = game.seed;
        let sunk = game
            .log
            .iter()
            .position(|e| matches!(e.kind, EventKind::ShipSunk { ship: s, .. } if s == ship(1)));
        let Some(sunk) = sunk else { continue };
        let lost: Vec<usize> = game
            .log
            .iter()
            .enumerate()
            .filter(|(_, e)| matches!(e.kind, EventKind::ChargeLost { .. }))
            .map(|(i, _)| i)
            .collect();
        assert_eq!(lost.len(), 1, "seed {seed}");
        let event = &game.log[lost[0]];
        assert_eq!(
            event.kind,
            EventKind::ChargeLost {
                escort: force(2),
                charge: force(1),
                hex: T,
            },
            "seed {seed}"
        );
        // In the pulse the last ship of the charge was sunk, after it, citing
        // the escort order.
        assert!(lost[0] > sunk, "seed {seed}");
        assert_eq!(
            (event.day, event.pulse),
            (game.log[sunk].day, game.log[sunk].pulse),
            "seed {seed}"
        );
        assert!(
            event.causes.iter().any(|c| matches!(c, Cause::Order(_))),
            "seed {seed}"
        );
        assert_eq!(
            game.scenario.world.umpire().task_forces[&force(2)].charge,
            None,
            "seed {seed}"
        );
        checked += 1;
    }
    assert!(checked > 0);
}

// ------------------------------------------------------------------ Part 3

/// The raid's scenario text: a cargo force (force 1, ship 1, no guns) and
/// its escort of two destroyers (force 2, ships 2 and 3), at S; a Japanese
/// carrier (force 3, ship 4) 200 nm east at T with 12 dive bombers (group 1)
/// on naval strike, going for transports only (so that the cargo force is the
/// raid's target, not its escort), and a contact of the cargo force. The
/// escort force starts at `escort_at`. One day.
fn raid_text(escort_order: bool, escort_at: Hex) -> String {
    let mut text = String::from(
        "# Scenario 51, part 3: against aircraft. Built by the test.\n\
         name = \"Placeholder scenario 51, part 3\"\n\
         placeholder = true\n\
         seed = 1942\n\
         days = 1\n\
         map = \"../map/placeholder_one_convoy.toml\"\n\
         ship_classes = \"../reference/ship_classes.csv\"\n\
         port_sizes = \"../reference/port_sizes.csv\"\n\
         aircraft_types = \"../reference/aircraft_types.csv\"\n\
         search = \"../reference/search.toml\"\n\
         surface_combat = \"../reference/surface_combat.toml\"\n\
         air_combat = \"../reference/air_combat.toml\"\n",
    );
    text += &format!(
        "
[[task_force]]
id = 1
name = \"Placeholder cargo force 1\"
side = \"allied\"
q = 0
r = 0
mission = \"transport\"

[[task_force]]
id = 2
name = \"Placeholder escort force 2\"
side = \"allied\"
q = {}
r = {}
mission = \"escort\"

[[task_force]]
id = 3
name = \"Placeholder carrier force 3\"
side = \"japanese\"
q = 5
r = 0
mission = \"carrier\"

[[ship]]
id = 1
name = \"Placeholder cargo ship 1\"
class = 3
side = \"allied\"
fuel_t = 1000
task_force = 1

[[ship]]
id = 2
name = \"Placeholder destroyer 2\"
class = 2
side = \"allied\"
fuel_t = 500
task_force = 2

[[ship]]
id = 3
name = \"Placeholder destroyer 3\"
class = 2
side = \"allied\"
fuel_t = 500
task_force = 2

[[ship]]
id = 4
name = \"Placeholder carrier 4\"
class = 4
side = \"japanese\"
fuel_t = 3000
task_force = 3

[[contact]]
side = \"japanese\"
q = 0
r = 0
ship_count = 1
kind = \"transport\"

[[air_group]]
id = 1
name = \"Placeholder bombing squadron\"
side = \"japanese\"
ship = 4
aircraft_type = 3
ready = 12

[[order]]
day = 1
side = \"japanese\"
air_group = 1
kind = \"set_air_mission\"
mission = \"naval_strike\"

[[order]]
day = 1
side = \"japanese\"
air_group = 1
kind = \"set_target_priority\"
kinds = [\"transport\"]
",
        escort_at.q, escort_at.r
    );
    if escort_order {
        text += &order(1, "task_force = 2", "kind = \"escort\"\ncharge = 1");
    }
    text
}

/// Figures for part 3: a contact the side starts with can be struck in the
/// morning, the guns' share before the attack is a half, and the destroyer
/// has `shots` anti-aircraft shots and the speed `knots`.
fn raid_options(shots: u32, knots: u32) -> LoadOptions {
    common::altered_reference("escort_guns", |dir| {
        common::edit_file_by(
            dir,
            "air_combat.toml",
            &[
                (
                    "strike_contact_max_age_pulses = 0",
                    "strike_contact_max_age_pulses = 1",
                ),
                (
                    "anti_aircraft_before_attack_share = \"1\"",
                    "anti_aircraft_before_attack_share = \"0.5\"",
                ),
            ],
        );
        common::edit_file_by(
            dir,
            "ship_classes.csv",
            &[(
                "destroyer,16,36,500,4000,0,4,6,150,5,8,8,200,100,0,0,0",
                &format!("destroyer,{knots},{knots},500,4000,0,4,6,150,5,8,8,200,100,0,0,{shots}"),
            )],
        );
    })
}

fn raid(escort_order: bool, escort_at: Hex, shots: u32, knots: u32, seed: u64) -> Scenario {
    let base = load_with(
        &raid_text(escort_order, escort_at),
        &raid_options(shots, knots),
    );
    let mut scenario = base.with_seed(seed).expect("a seed can be set");
    scenario.run_to_end().expect("the run resolves");
    scenario
}

/// The raid's events of the morning of day 1: the guns' fire, in order, and
/// the attacks.
fn raid_events(scenario: &Scenario) -> (Vec<EventKind>, Vec<EventKind>) {
    let morning: Vec<EventKind> = scenario
        .world
        .events()
        .iter()
        .filter(|e| e.day == 1 && e.pulse == Some(Pulse::Morning))
        .map(|e| e.kind.clone())
        .collect();
    let guns = morning
        .iter()
        .filter(|k| matches!(k, EventKind::AntiAircraftFire { .. }))
        .cloned()
        .collect();
    let attacks = morning
        .iter()
        .filter(|k| matches!(k, EventKind::AirAttack { .. }))
        .cloned()
        .collect();
    (guns, attacks)
}

/// Every attack of the morning's raid is on a ship of the cargo force. (The
/// afternoon's raid, if the morning's sank the cargo ship, has no transport
/// to go for.)
fn every_bomb_is_aimed_at_the_cargo_force(scenario: &Scenario, seed: u64) {
    let attacks: Vec<_> = scenario
        .world
        .events()
        .iter()
        .filter(|e| e.day == 1 && e.pulse == Some(Pulse::Morning))
        .filter_map(|e| match &e.kind {
            EventKind::AirAttack {
                task_force, ship, ..
            } => Some((*task_force, *ship)),
            _ => None,
        })
        .collect();
    assert!(!attacks.is_empty(), "seed {seed}: a raid");
    assert!(
        attacks.iter().all(|a| *a == (force(1), ship(1))),
        "seed {seed}: {attacks:?}"
    );
}

#[test]
fn the_escorts_guns_fire_with_the_charges_at_a_raid() {
    // Two destroyers of 6 shots each: 12 shots, split by the share of
    // chapter 5 (one half) into 6 before the attack and 6 after it. The cargo
    // force has no guns of its own.
    for seed in 1..=20 {
        let scenario = raid(true, S, 6, 16, seed);
        let (guns, _) = raid_events(&scenario);
        assert_eq!(guns.len(), 2, "seed {seed}");
        for (gun, before) in guns.iter().zip([true, false]) {
            match gun {
                EventKind::AntiAircraftFire {
                    task_force,
                    shots,
                    chance,
                    after_attack,
                    ..
                } => {
                    assert_eq!(*task_force, force(1), "seed {seed}");
                    assert_eq!(*shots, fx("6"), "seed {seed}");
                    assert_eq!(*chance, fx("0.05"), "seed {seed}");
                    assert_eq!(*after_attack, !before, "seed {seed}");
                }
                other => panic!("{other:?}"),
            }
        }
        every_bomb_is_aimed_at_the_cargo_force(&scenario, seed);
    }
}

#[test]
fn an_escort_a_hex_away_or_with_no_order_adds_no_guns() {
    for seed in 1..=20 {
        // The escort is a hex away (and too slow to close in the night).
        let away = raid(true, hex(1, 0), 6, 1, seed);
        let (guns, attacks) = raid_events(&away);
        assert!(guns.is_empty(), "seed {seed}: {guns:?}");
        assert_eq!(attacks.len(), 1, "seed {seed}");
        every_bomb_is_aimed_at_the_cargo_force(&away, seed);
        // No escort order, the destroyers in the hex.
        let none = raid(false, S, 6, 16, seed);
        let (guns, attacks) = raid_events(&none);
        assert!(guns.is_empty(), "seed {seed}: {guns:?}");
        assert_eq!(attacks.len(), 1, "seed {seed}");
        every_bomb_is_aimed_at_the_cargo_force(&none, seed);
    }
}
