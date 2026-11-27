//! Three of the four mends at the seat of decision 0172: an escort's charge
//! named by the side's number at every seat (2), a day that fails its
//! invariants ends the session's game (3), and preview numbers counted for
//! each side (4). The first mend, the game file pinned to its inputs, is in
//! `crates/sim-cli/tests/game_file_pin.rs`.
//!
//! # What the tests are to show
//!
//! Written by the architect in decision 0172; the tests are written to it.
//!
//! 2. **An escort's charge.** A copy of scenario 51 with its two Allied
//!    forces numbered 10 and 20. At the area's seat the side lists them as
//!    its forces 1 and 2. An escort order to force 1 with force 2 as its
//!    charge is queued, and the order the engine records has task force 10
//!    escorting task force 20. With a charge that names no force of the
//!    side, it is refused as not known. Scenario 51's own tests pass
//!    unchanged.
//! 3. **A day that failed.** In a session whose world is made to break an
//!    invariant inside a day (by a means of the test, not of the library),
//!    `end_day` answers that the game has stopped. A second `end_day`, a
//!    briefing, an order and a preview are each answered the same, and the
//!    world's day and its event count are what they were after the failure.
//! 4. **Preview numbers.** Two sessions of one scenario with a seat on each
//!    side, the same in everything the Allied seat does. In the second the
//!    Japanese seat asks for three previews first. The Allied seat's answers
//!    are the same in both, preview numbers and all, and it approves its
//!    preview 1 in both. In the second the Japanese seat's previews are 1, 2
//!    and 3.
//!
//! In the copy of scenario 51 the escort (two destroyers, on the escort
//! mission) is numbered 10 and the cargo force 20, so that the side's force 1
//! is the escort and its force 2 the charge: "task force 10 escorting task
//! force 20". Before the mend the area's seat read `task_force:2` as the
//! engine's force 2, which does not exist, and refused the order.
//!
//! The means of the test for 3 is the one `tests/leaks.rs` uses: scenario
//! 12c with no orders and an arrival, put in the world's starting state by
//! the test, that puts 100 more aircraft into Japanese group 3 aboard a
//! carrier with room for 36. The loader refuses such an arrival, so the
//! world is built directly; the arrival lands inside day 1 and the day's
//! close finds the carrier over its capacity. Nothing in the library is
//! added for it.
//!
//! Scenario 6a (a Japanese airfield 1 and an Allied airfield 2) gives each
//! side a base to keep supplied, so that each side's seat may ask for
//! previews of an intent of its own.

use crate::support;

use std::collections::BTreeMap;

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::ids::{AirGroupId, TaskForceId};
use sim_core::orders::{OrderKind, TaskForceOrder};
use sim_core::state::{Arrival, Side};
use sim_core::World;
use support::{load, load_text, scenario_text};

// ---------------------------------------------------------------------
// 2. An escort's charge.
// ---------------------------------------------------------------------

/// Scenario 51 with no orders and its forces renumbered: the escort, once
/// force 2, is force 10; the cargo force, once force 1, is force 20.
fn scenario_51_renumbered() -> sim_data::Scenario {
    let name = "51_an_escort_force.toml";
    let text = scenario_text(name);
    let (head, _orders) = text
        .split_once("\n# Day 1:")
        .expect("51's orders come last");
    let renumbered = head
        .replace(
            "id = 1\nname = \"Placeholder cargo force 1\"",
            "id = 20\nname = \"Placeholder cargo force 1\"",
        )
        .replace(
            "id = 2\nname = \"Placeholder escort force 2\"",
            "id = 10\nname = \"Placeholder escort force 2\"",
        )
        .replace("task_force = 1\n", "task_force = 20\n")
        .replace("task_force = 2\n", "task_force = 10\n");
    assert!(renumbered.contains("id = 20\n") && renumbered.contains("id = 10\n"));
    assert!(!renumbered.contains("task_force = 1\n") && !renumbered.contains("task_force = 2\n"));
    load_text(&renumbered, name, 1942)
}

fn allied_session(scenario: sim_data::Scenario) -> Session {
    let name = scenario.name.clone();
    Session::new(
        scenario.world,
        &name,
        scenario.days,
        Seat::Player,
        Seat::Script(BTreeMap::new()),
    )
}

#[test]
fn an_escorts_charge_is_named_by_the_sides_number_at_the_areas_seat() {
    let scenario = scenario_51_renumbered();
    let forces: Vec<u32> = scenario
        .world
        .umpire()
        .task_forces
        .keys()
        .map(|id| id.raw())
        .collect();
    assert_eq!(forces, [10, 20]);
    let mut session = allied_session(scenario);

    // The side lists them as its forces 1 and 2: 1 the escort, 2 the charge.
    let listed = session
        .call(Side::Allied, "list_forces", &json!({}))
        .expect("list_forces is answered");
    let names: Vec<(String, String)> = listed["task_forces"]
        .as_array()
        .expect("task forces")
        .iter()
        .map(|f| {
            (
                f["task_force"]["handle"]
                    .as_str()
                    .expect("handle")
                    .to_string(),
                f["task_force"]["name"].as_str().expect("name").to_string(),
            )
        })
        .collect();
    assert_eq!(
        names,
        [
            (
                "task_force:1".to_string(),
                "Placeholder escort force 2".to_string()
            ),
            (
                "task_force:2".to_string(),
                "Placeholder cargo force 1".to_string()
            ),
        ],
        "{listed}"
    );

    // A charge that names no force of the side is refused as not known.
    let answer = session
        .call(
            Side::Allied,
            "issue_orders",
            &json!({"orders": [
                {"unit": "task_force:1", "order": {"kind": "escort", "charge": "task_force:7"}},
            ]}),
        )
        .expect("issue_orders is answered");
    assert_eq!(answer["results"][0]["status"], "refused", "{answer}");
    assert_eq!(answer["results"][0]["reason"], "not known", "{answer}");

    // Force 1 escorting force 2 is queued.
    let answer = session
        .call(
            Side::Allied,
            "issue_orders",
            &json!({"orders": [
                {"unit": "task_force:1", "order": {"kind": "escort", "charge": "task_force:2"}},
            ]}),
        )
        .expect("issue_orders is answered");
    assert_eq!(answer["results"][0]["status"], "queued", "{answer}");
    let ended = session
        .call(Side::Allied, "end_day", &json!({}))
        .expect("end_day is answered");
    assert_eq!(ended["resolved"], true, "{ended}");

    // The order the engine records: task force 10 escorting task force 20.
    let handed: Vec<OrderKind> = session.handed_in()[&1]
        .iter()
        .map(|(_, request)| request.kind.clone())
        .collect();
    assert_eq!(
        handed,
        [OrderKind::TaskForce {
            target: TaskForceId::new(10),
            order: TaskForceOrder::Escort(Some(TaskForceId::new(20))),
        }]
    );
    let world = session.into_world();
    let recorded: Vec<OrderKind> = world.orders().map(|o| o.kind.clone()).collect();
    assert_eq!(recorded, handed, "the engine records the order handed in");
    assert_eq!(
        world.umpire().task_forces[&TaskForceId::new(10)]
            .charge
            .map(|charge| charge.force),
        Some(TaskForceId::new(20))
    );
}

// ---------------------------------------------------------------------
// 3. A day that failed.
// ---------------------------------------------------------------------

/// Scenario 12c with no orders, an arrival that overfills carrier B inside
/// day 1, both seats' staffs, and the Allied seat held by a player.
fn session_that_fails_on_day_1() -> Session {
    let name = "12c_no_deck.toml";
    let text = scenario_text(name);
    let (head, _orders) = text
        .split_once("\n[[order]]")
        .expect("12c's orders come last");
    let mut scenario = load_text(head, name, 1942);
    let world = &scenario.world;
    let mut truth = world.umpire().clone();
    truth.arrivals.push(Arrival {
        day: 1,
        air_group: AirGroupId::new(3),
        aircraft: 100,
        pilots: 0,
        waiting: false,
    });
    scenario.world = World::new(
        world.seed(),
        world.ref_data().clone(),
        world.map().clone(),
        truth,
    )
    .expect("the world is valid at the start");
    allied_session(scenario).with_staffs(sim_staff::staffs())
}

#[test]
fn a_day_that_fails_its_invariants_ends_the_sessions_game() {
    let mut session = session_that_fails_on_day_1();
    let failed = session
        .call(Side::Allied, "end_day", &json!({}))
        .expect_err("the day cannot be resolved");
    assert!(
        failed.message.contains("the game has stopped"),
        "{}",
        failed.message
    );

    assert_eq!(session.stopped_inside_day(), Some(1));
    assert!(!session.is_over());
    let (day, hash) = (session.day(), session.state_hash());
    assert_eq!(day, 0, "the failed day is not counted");

    let calls: [(&str, Value); 4] = [
        ("end_day", json!({})),
        ("get_briefing", json!({})),
        (
            "issue_orders",
            json!({"orders": [{"unit": "task_force:1", "order": {"kind": "set_speed", "speed": "full"}}]}),
        ),
        (
            "preview",
            json!({"command": "set_intent", "base": 1, "floor_days": 30}),
        ),
    ];
    for (tool, arguments) in &calls {
        let answer = session
            .call(Side::Allied, tool, arguments)
            .expect_err("a stopped game answers every call with the stop");
        assert_eq!(answer.message, failed.message, "{tool}");
        assert_eq!(session.day(), day, "{tool}");
        assert_eq!(session.state_hash(), hash, "{tool}");
    }
    // Every call was told, and is in the transcript.
    assert_eq!(session.transcript().len(), 1 + calls.len());

    // The world's event count is what it was after the failure: the same as
    // a session that made the failing call and nothing after it.
    let mut alone = session_that_fails_on_day_1();
    alone
        .call(Side::Allied, "end_day", &json!({}))
        .expect_err("the day cannot be resolved");
    let after = session.into_world();
    let reference = alone.into_world();
    assert_eq!(after.day(), 0);
    assert_eq!(after.events().len(), reference.events().len());
    assert_eq!(after.state_hash(), reference.state_hash());
}

// ---------------------------------------------------------------------
// 4. Preview numbers.
// ---------------------------------------------------------------------

/// Scenario 6a with both seats held by players and both sides' staffs.
fn two_seat_session() -> Session {
    let scenario = load("06a_base_raid.toml", 1942);
    let name = scenario.name.clone();
    Session::new(
        scenario.world,
        &name,
        scenario.days,
        Seat::Player,
        Seat::Player,
    )
    .with_staffs(sim_staff::staffs())
}

/// What the Allied seat does in both sessions, and what it is told.
fn allied_plays(session: &mut Session) -> Vec<Value> {
    let preview = session
        .call(
            Side::Allied,
            "preview",
            &json!({"command": "set_intent", "base": 2, "floor_days": 30}),
        )
        .expect("the preview is answered");
    let approved = session
        .call(Side::Allied, "approve", &json!({"preview": 1}))
        .expect("the approval is answered");
    let state = session
        .call(Side::Allied, "command_state", &json!({}))
        .expect("command_state is answered");
    vec![preview, approved, state]
}

#[test]
fn previews_are_numbered_for_each_side() {
    let mut first = two_seat_session();
    let alone = allied_plays(&mut first);

    let mut second = two_seat_session();
    let japanese: Vec<Value> = [30, 35, 40]
        .into_iter()
        .map(|floor| {
            second
                .call(
                    Side::Japanese,
                    "preview",
                    &json!({"command": "set_intent", "base": 1, "floor_days": floor}),
                )
                .expect("the preview is answered")
        })
        .collect();
    for (i, preview) in japanese.iter().enumerate() {
        assert_eq!(preview["kind"], "preview", "{preview}");
        assert_eq!(preview["preview"], i + 1, "{preview}");
    }
    let beside = allied_plays(&mut second);

    assert_eq!(alone[0]["kind"], "preview", "{}", alone[0]);
    assert_eq!(alone[0]["preview"], 1, "{}", alone[0]);
    assert_eq!(alone[1]["status"], "queued", "{}", alone[1]);
    assert_eq!(alone[1]["preview"], 1, "{}", alone[1]);
    assert_eq!(alone, beside, "the Allied seat is told the same in both");
}
