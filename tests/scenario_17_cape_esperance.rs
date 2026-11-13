//! Scenario 17: Cape Esperance, 11 to 12 October 1942. Loads, meets, and
//! nothing more.
//!
//! Decision 0072 (`docs/decisions/0072-cape-esperance-scenario.md`). The file
//! is `data/scenarios/17_cape_esperance.toml`. It stood in
//! `data/scenarios/not-run/` until Savo Island was tuned, and moved on Nov 13,
//! 2026, for the one check (decision 0084); it is never tuned on.
//!
//! **This test reads nothing of how the battle comes out.** It runs the one day
//! on the frozen figures (`tests/common`, `data/reference/step1`), because the
//! meeting happens inside the night's movement, and then reads only events up
//! to and including the first `EngagementBegun`, and the state as loaded. No
//! assertion, and no message of a failing assertion, names or prints a salvo,
//! a hit, a sinking, a claim, a report, a break-off, an end state, a hash or
//! an event after the meeting. A failure message says which number differed
//! and nothing else.
//!
//! # Hand calculation
//!
//! Written by the architect's team lead before the code, from decision 0072
//! ("Where and when", "Readiness", "For the next stage"), and checked against
//! the derived speeds: every class's top speed is above both ordered speeds
//! (the slowest, 32.5 knots, Brooklyn, St. Louis and Pensacola classes), so
//! the ordered 27 and 29 knots stand. If the code disagrees, the code or the
//! scenario is wrong: do not change a number here to make the test pass.
//!
//! **1. It loads, on the frozen figures,** with 14 ships, two task forces and
//! one base (Shortland, Japanese, at (-2,-8)).
//!
//! **2. Ship order and classes** (decision 0072, 1 and 12), as loaded:
//!
//! | Task force | Ships, in order | Classes |
//! | --- | --- | --- |
//! | 1, Japanese | 1 Aoba, 2 Furutaka, 3 Kinugasa, 4 Fubuki, 5 Hatsuyuki | 3000, 3001, 3000, 3002, 3002 |
//! | 2, Allied | 6 Farenholt, 7 Duncan, 8 Laffey, 9 San Francisco, 10 Boise, 11 Salt Lake City, 12 Helena, 13 Buchanan, 14 McCalla | 3003, 3003, 3004, 3005, 3006, 3007, 3008, 3003, 3003 |
//!
//! **3. Readiness as loaded:** task force 1 at normal readiness and not
//! alerted; task force 2 at battle readiness. Neither side holds a contact.
//!
//! **4. Task force 1's approach.** From (0,-8), for its first waypoint (2,-5),
//! the path is (0,-8), (1,-8), (1,-7), (2,-7), (2,-6), (2,-5), clear of land,
//! so each step is charged the miles it brings the force closer (decision
//! 0058), with air distances rounded to the nearest mile:
//!
//! | Step | air(from, (2,-5)) | air(to, (2,-5)) | Charge | Total |
//! | --- | --- | --- | --- | --- |
//! | (0,-8) to (1,-8) | 40 x sqrt(19) = 174.36, 174 | 40 x sqrt(13) = 144.22, 144 | 30 | 30 |
//! | (1,-8) to (1,-7) | 144 | 40 x sqrt(7) = 105.83, 106 | 38 | 68 |
//! | (1,-7) to (2,-7) | 106 | 80 | 26 | 94 |
//! | (2,-7) to (2,-6) | 80 | 40 | 40 | 134 |
//! | (2,-6) to (2,-5) | 40 | 0 | 40 | 174 |
//!
//! It enters (2,-5) halfway through the last step, after 134 + 20 = 154 miles.
//! At 27 knots that is 154 / 27 = 5.703703 hours (truncated at the sixth
//! place): 23:42 on 11 October. Its first steaming segment of the night runs
//! from (0,-8) and ends in (2,-5) at the meeting, 154 miles at 27 knots,
//! entering the five hexes (1,-8), (1,-7), (2,-7), (2,-6), (2,-5) (the hex it
//! starts in is not one it enters).
//!
//! **5. Task force 2's approach.** From (2,-2), for its first waypoint (1,-4):
//! (2,-3), (1,-3), (1,-4), charged air((2,-2),(1,-4)) = 40 x sqrt(7) = 106 in
//! all (37, 29 and 40). Then for (2,-5), one step of 40. It enters (2,-5)
//! after 106 + 20 = 126 miles, 126 / 29 = 4.344827 hours (22:20). Its first
//! steaming segment of the night, recorded before the meeting, runs from
//! (2,-2) and ends in (2,-5): 126 miles at 29 knots, entering (2,-3), (1,-3),
//! (1,-4), (2,-5). Entering (2,-5) is its last change of position, so it
//! stands in (2,-5) from 22:20, before the Japanese enter at 23:42. Its last
//! 20 miles to the centre of its last waypoint, and its arrival there (by the
//! clock at 146 / 29 = 5.034482 hours, 23:02), are steamed out after the
//! changes of position of every force, so they are recorded after the fight:
//! the second approximation of decision 0022, 6, which says a force met in
//! the hex of its last waypoint has its last 20 miles and its arrival recorded
//! after the fight with the arrival's own earlier hour. This test reads
//! nothing after the meeting, so it does not assert the arrival.

//! **6. The meeting.** The first `EngagementBegun` of the game is in the night
//! pulse of day 1, between task forces 1 and 2 (lower first), in hex (2,-5),
//! at elapsed hours 5.703703, and it is dark. It is the only
//! `EngagementBegun` of the night pulse, and it comes after task force 2's
//! steaming segment of item 5.
//!
//! **7. Readiness at the meeting.** No `ForceBecameAlert`,
//! `ForceAlertedByContact` or `AlarmRoll` for task force 1 comes before the
//! meeting, so it meets the enemy at normal readiness, not alert and not
//! ready (chapter 4, "Ready, and the share of the guns"). Task force 2 is at
//! battle readiness, so alert and ready.

mod common;

// The code is written to the hand calculation above.

use std::path::PathBuf;

use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::orders::Readiness;
use sim_core::state::Side;
use sim_core::time::Pulse;
use sim_data::Scenario;

const SEED: u64 = 1942;
const MEETING: Hex = Hex::new(2, -5);

fn fx(value: i64) -> Fixed {
    Fixed::from_int(value)
}

fn millionths(value: i64) -> Fixed {
    Fixed::from_raw(value)
}

fn load() -> Scenario {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/17_cape_esperance.toml");
    common::load(&path).expect("the scenario loads")
}

/// The events up to and including the first `EngagementBegun`, and nothing
/// after it, plus how many `EngagementBegun` events the night pulse of day 1
/// holds. The one run; nothing else of it is read.
fn meeting_prefix(scenario: &Scenario) -> (Vec<Event>, usize) {
    let mut run = scenario.with_seed(SEED).expect("a seed can be set");
    run.run_to_end().expect("the run resolves");
    let mut prefix = Vec::new();
    let mut night_meetings = 0;
    let mut met = false;
    for event in run.world.events().iter() {
        let begun = matches!(event.kind, EventKind::EngagementBegun { .. });
        if begun && event.day == 1 && event.pulse == Some(Pulse::Night) {
            night_meetings += 1;
        }
        if !met {
            prefix.push(event.clone());
            met = begun;
        }
    }
    assert!(met, "the forces meet");
    (prefix, night_meetings)
}

fn naming_force_one(kind: &EventKind) -> bool {
    match kind {
        EventKind::ForceBecameAlert { task_force, .. }
        | EventKind::ForceAlertedByContact { task_force, .. }
        | EventKind::AlarmRoll { task_force, .. } => task_force.raw() == 1,
        _ => false,
    }
}

#[test]
fn it_loads_with_14_ships_2_forces_and_shortland() {
    let scenario = load();
    let truth = scenario.world.umpire();
    assert_eq!(truth.ships.len(), 14);
    assert_eq!(truth.task_forces.len(), 2);
    assert_eq!(truth.bases.len(), 1);
    let base = truth.bases.values().next().expect("one base");
    assert_eq!(base.name, "Shortland");
    assert_eq!(base.side, Side::Japanese);
    assert_eq!(base.hex, Hex::new(-2, -8));
}

#[test]
fn ships_are_in_order_with_their_classes() {
    let scenario = load();
    let truth = scenario.world.umpire();
    let expected: [(u32, Vec<u32>, Vec<u32>); 2] = [
        (1, vec![1, 2, 3, 4, 5], vec![3000, 3001, 3000, 3002, 3002]),
        (
            2,
            (6..=14).collect(),
            vec![3003, 3003, 3004, 3005, 3006, 3007, 3008, 3003, 3003],
        ),
    ];
    for (force, ships, classes) in expected {
        let task_force = truth
            .task_forces
            .iter()
            .find(|(id, _)| id.raw() == force)
            .map(|(_, f)| f)
            .expect("the task force exists");
        let held: Vec<u32> = task_force.ships.iter().map(|s| s.raw()).collect();
        assert_eq!(held, ships, "ships of force {force}");
        let held_classes: Vec<u32> = ships
            .iter()
            .map(|id| {
                truth
                    .ships
                    .iter()
                    .find(|(ship, _)| ship.raw() == *id)
                    .map(|(_, s)| s.class.raw())
                    .expect("the ship exists")
            })
            .collect();
        assert_eq!(held_classes, classes, "classes of force {force}");
    }
}

#[test]
fn readiness_as_loaded_and_no_contacts() {
    let scenario = load();
    let truth = scenario.world.umpire();
    for (id, force) in &truth.task_forces {
        match id.raw() {
            1 => {
                assert_eq!(force.readiness, Readiness::Normal);
                assert!(!force.alerted);
            }
            2 => assert_eq!(force.readiness, Readiness::Battle),
            _ => panic!("an unexpected task force"),
        }
    }
    for side in [Side::Japanese, Side::Allied] {
        assert!(scenario.world.side_view(side).contacts().next().is_none());
    }
}

#[test]
fn task_force_1_steams_154_nm_at_27_kn_into_2_m5() {
    let (prefix, _) = meeting_prefix(&load());
    let first = prefix
        .iter()
        .find_map(|event| match &event.kind {
            EventKind::TaskForceSteamed {
                task_force,
                from,
                to,
                distance_nm,
                speed_kn,
                hexes_entered,
            } if task_force.raw() == 1 => Some((
                event.pulse,
                *from,
                *to,
                *distance_nm,
                *speed_kn,
                hexes_entered.clone(),
            )),
            _ => None,
        })
        .expect("task force 1 steams before the meeting");
    assert_eq!(first.0, Some(Pulse::Night));
    assert_eq!(first.1, Hex::new(0, -8));
    assert_eq!(first.2, MEETING);
    assert_eq!(first.3, fx(154));
    assert_eq!(first.4, fx(27));
    assert_eq!(
        first.5,
        vec![
            Hex::new(1, -8),
            Hex::new(1, -7),
            Hex::new(2, -7),
            Hex::new(2, -6),
            Hex::new(2, -5)
        ]
    );
}

#[test]
fn task_force_2_steams_126_nm_at_29_kn_into_2_m5_before_the_meeting() {
    let (prefix, _) = meeting_prefix(&load());
    // Every event read is before the meeting's id, or is the meeting.
    let first = prefix
        .iter()
        .find_map(|event| match &event.kind {
            EventKind::TaskForceSteamed {
                task_force,
                from,
                to,
                distance_nm,
                speed_kn,
                hexes_entered,
            } if task_force.raw() == 2 => Some((
                event.pulse,
                *from,
                *to,
                *distance_nm,
                *speed_kn,
                hexes_entered.clone(),
            )),
            _ => None,
        })
        .expect("task force 2 steams before the meeting");
    assert_eq!(first.0, Some(Pulse::Night));
    assert_eq!(first.1, Hex::new(2, -2));
    assert_eq!(first.2, MEETING);
    assert_eq!(first.3, fx(126));
    assert_eq!(first.4, fx(29));
    assert_eq!(
        first.5,
        vec![
            Hex::new(2, -3),
            Hex::new(1, -3),
            Hex::new(1, -4),
            Hex::new(2, -5)
        ]
    );
}

#[test]
fn the_meeting_is_in_the_dark_at_5_703703_hours() {
    let (prefix, night_meetings) = meeting_prefix(&load());
    let meeting = prefix.last().expect("the meeting is the last event read");
    assert_eq!(meeting.day, 1);
    assert_eq!(meeting.pulse, Some(Pulse::Night));
    match &meeting.kind {
        EventKind::EngagementBegun {
            a,
            b,
            hex,
            elapsed_hours,
        } => {
            assert_eq!((a.raw(), b.raw()), (1, 2));
            assert_eq!(*hex, MEETING);
            assert_eq!(*elapsed_hours, millionths(5_703_703));
        }
        _ => panic!("the last event read is not the meeting"),
    }
    assert_eq!(night_meetings, 1, "one meeting in the night pulse");
}

#[test]
fn task_force_1_is_not_alerted_before_the_meeting() {
    let (prefix, _) = meeting_prefix(&load());
    assert!(
        !prefix.iter().any(|event| naming_force_one(&event.kind)),
        "force 1 was alerted or alarm-rolled before the meeting"
    );
}
