//! What the staff does with a unit that may not be used yet (decision 0140,
//! 3): it leaves it out and names the day. The games are scenarios 36, 47
//! and 58 with a day set on a ship, a group or a force, played on the frozen
//! step 1 figures with the doctrine's staffs passed in. The architect's hand
//! calculation of the whole is
//! `tests/scenario_65_a_day_from_which_a_force_may_be_used.rs`, which these
//! do not repeat.

use std::path::{Path, PathBuf};

use sim_core::command::{CommandKind, PlanOutcome, Reason, StaffDecisions};
use sim_core::events::EventKind;
use sim_core::ids::{AirGroupId, EntityRef, ShipId, TaskForceId};
use sim_core::state::Side;
use sim_data::{LoadOptions, Scenario};
use sim_staff::{as_staffs, staffs};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn scenario_text(file: &str) -> String {
    std::fs::read_to_string(root().join("data/scenarios").join(file)).expect("scenario reads")
}

/// `text` with `needle` (once) followed by `available_from = day`.
fn with_day(text: &str, needle: &str, day: u32) -> String {
    assert_eq!(
        text.matches(needle).count(),
        1,
        "{needle} is once in the file"
    );
    text.replace(needle, &format!("{needle}\navailable_from = {day}"))
}

/// What the Allied staff handed in on each day, day 1 first, and whether
/// anything was refused.
fn play(file: &str, text: &str) -> (Vec<StaffDecisions>, bool) {
    let path = root().join("data/scenarios").join(file);
    let options = LoadOptions::frozen_step1(&root());
    let mut scenario = Scenario::from_text_with(text, &path, &options).expect("loads");
    let held = staffs();
    let mut handed = Vec::new();
    while scenario
        .next_day_watched(as_staffs(&held), &mut |_, _| {})
        .expect("a day resolves")
        .is_some()
    {
        handed.push(
            scenario
                .world
                .staff_handed_in()
                .get(&Side::Allied)
                .cloned()
                .unwrap_or_default(),
        );
    }
    let refused = scenario.world.events().iter().any(|e| {
        matches!(
            e.kind,
            EventKind::OrderRejected { .. } | EventKind::CommandRefused { .. }
        )
    });
    (handed, refused)
}

fn said(reason: &Reason) -> String {
    let mut all = reason.worked.clone();
    for e in &reason.evidence {
        all.push('\n');
        all.push_str(&e.what);
    }
    all
}

fn all_said(day: &StaffDecisions) -> Vec<String> {
    day.no_orders.iter().map(said).collect()
}

fn ordered(day: &StaffDecisions, unit: EntityRef) -> bool {
    day.orders.iter().any(|o| o.kind.target() == unit)
}

#[test]
fn a_pool_ship_before_its_day_is_left_out_of_a_supply_plan() {
    let base = scenario_text("36_one_convoy.toml").replace("days = 18", "days = 6");
    let text = with_day(
        &base,
        "name = \"Placeholder cargo ship 2\"\nclass = 3\nside = \"allied\"",
        5,
    );
    let (handed, refused) = play("36_one_convoy.toml", &text);
    assert!(!refused, "nothing the staff handed in was refused");
    for (i, day) in handed.iter().enumerate().take(4) {
        for command in &day.commands {
            if let CommandKind::AddRun { run, .. } = &command.kind {
                assert!(
                    !run.ships.contains(&ShipId::new(2)),
                    "day {}: ship 2 is in a run before its day",
                    i + 1
                );
            }
        }
    }
}

#[test]
fn a_pool_of_ships_before_their_day_gives_a_reason_with_the_day_and_fails_no_plan() {
    let base = scenario_text("36_one_convoy.toml")
        .replace("days = 18", "days = 3")
        .replace("ships = [1, 2]", "ships = [2]");
    let text = with_day(
        &base,
        "name = \"Placeholder cargo ship 2\"\nclass = 3\nside = \"allied\"",
        5,
    );
    let (handed, refused) = play("36_one_convoy.toml", &text);
    assert!(!refused);
    for day in &handed {
        assert!(
            day.commands.iter().all(|c| !matches!(
                c.kind,
                CommandKind::AddRun { .. }
                    | CommandKind::MarkPlan {
                        outcome: PlanOutcome::Failed,
                        ..
                    }
            )),
            "no run and no failed plan before the day"
        );
    }
    let reasons = all_said(&handed[0]);
    assert!(
        reasons
            .iter()
            .any(|r| r.contains("ship:2 may be used from day 5")),
        "the reason names the ship and its day: {reasons:?}"
    );
}

#[test]
fn an_air_group_before_its_day_is_not_used_by_a_bases_cover() {
    let base = scenario_text("47_cover_from_the_air.toml").replace("days = 12", "days = 5");
    let text = with_day(
        &base,
        "name = \"Placeholder flying boats 3\"\nside = \"allied\"",
        4,
    );
    let (handed, refused) = play("47_cover_from_the_air.toml", &text);
    assert!(!refused);
    let group3 = EntityRef::AirGroup(AirGroupId::new(3));
    let group1 = EntityRef::AirGroup(AirGroupId::new(1));
    for (i, day) in handed.iter().enumerate().take(3) {
        assert!(!ordered(day, group3), "day {}: group 3 is ordered", i + 1);
    }
    assert!(ordered(&handed[0], group1), "the others are served as ever");
    assert!(
        handed[0]
            .no_orders
            .iter()
            .any(|r| said(r).contains("group:3 may be used from day 4")),
        "a reason names group 3 with its day"
    );
    assert!(ordered(&handed[3], group3), "from its day it is served");
}

#[test]
fn a_task_force_intent_with_its_force_before_its_day_gives_one_reason_with_the_day() {
    let base = scenario_text("58_a_carrier_force_closes.toml");
    let text = with_day(
        &base,
        "name = \"Placeholder carrier force 1\"\nside = \"allied\"",
        2,
    );
    let (handed, refused) = play("58_a_carrier_force_closes.toml", &text);
    assert!(!refused);
    let force = EntityRef::TaskForce(TaskForceId::new(1));
    let day1 = &handed[0];
    assert!(
        day1.orders.is_empty(),
        "no order on day 1: {:?}",
        day1.orders
    );
    assert_eq!(day1.no_orders.len(), 1, "one reason: {:?}", all_said(day1));
    let reason = said(&day1.no_orders[0]);
    assert!(
        reason.contains("is to be served by") && reason.contains("may be used from day 2"),
        "{reason}"
    );
    assert!(ordered(&handed[1], force), "the force is ordered on day 2");
    assert!(!ordered(day1, force));
    assert!(!handed[1].orders.is_empty(), "the force is served on day 2");
}
