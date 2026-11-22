//! What the staff does with a unit held apart until its day (decision 0195,
//! 8 and 13): it gives a force named by an intent no order and says it
//! enters on its day, counts a force or a land unit held apart as coming
//! where a take counts what it may use and where it asks whether more is
//! coming, and on the day itself serves the force as any force. The games
//! are scenario 58 (a carrier force) and scenario 109 (men ashore), each
//! with `enters` put on a force or a unit by the test, on the frozen step 1
//! figures. The architect's hand calculation of the whole is
//! `tests/scenario_111_forces_that_arrive.rs`, which these do not repeat.

use std::path::{Path, PathBuf};

use sim_core::command::{CommandKind, ForcesLimit};
use sim_core::command::{PlanOutcome, Reason, StaffDecisions};
use sim_core::ids::{EntityRef, LandUnitId, TaskForceId};
use sim_core::orders::{LandUnitOrder, OrderKind, Posture};
use sim_core::state::Side;
use sim_data::{LoadOptions, Scenario};
use sim_staff::{as_staffs, counted_by_a_take, staffs};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load(path: &Path, text: &str) -> Scenario {
    let options = LoadOptions::frozen_step1(&root());
    Scenario::from_text_with(text, path, &options).expect("loads")
}

fn said(reason: &Reason) -> String {
    let mut all = reason.worked.clone();
    for e in &reason.evidence {
        all.push('\n');
        all.push_str(&e.what);
    }
    all
}

fn ordered(day: &StaffDecisions, unit: EntityRef) -> bool {
    day.orders.iter().any(|o| o.kind.target() == unit)
}

fn limit_of(force: Option<u32>, unit: Option<u32>) -> ForcesLimit {
    ForcesLimit {
        ships: Default::default(),
        task_forces: force.map(TaskForceId::new).into_iter().collect(),
        air_groups: Default::default(),
        source: None,
        land_units: unit.map(LandUnitId::new).into_iter().collect(),
        submarines: Default::default(),
    }
}

/// What `side`'s staff handed in on each day, day 1 first.
fn play(path: &Path, text: &str, side: Side) -> Vec<StaffDecisions> {
    let held = staffs();
    let mut scenario = load(path, text);
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
                .get(&side)
                .cloned()
                .unwrap_or_default(),
        );
    }
    handed
}

// ---------------------------------------------------------- A carrier force

fn carrier_path() -> PathBuf {
    root().join("data/scenarios/58_a_carrier_force_closes.toml")
}

fn carrier_text(enters: u32) -> String {
    let text = std::fs::read_to_string(carrier_path()).expect("scenario reads");
    let needle = "name = \"Placeholder carrier force 1\"\nside = \"allied\"";
    assert_eq!(text.matches(needle).count(), 1);
    text.replace(needle, &format!("{needle}\nenters = {enters}"))
}

#[test]
fn a_carrier_force_held_apart_is_given_no_order_and_its_day_is_named() {
    let handed = play(&carrier_path(), &carrier_text(3), Side::Allied);
    let force = EntityRef::TaskForce(TaskForceId::new(1));
    for (i, day) in handed.iter().enumerate().take(2) {
        assert!(day.orders.is_empty(), "day {}: {:?}", i + 1, day.orders);
        assert_eq!(day.no_orders.len(), 1, "day {}: one reason", i + 1);
        let reason = &day.no_orders[0];
        assert!(
            reason
                .evidence
                .iter()
                .any(|e| e.what == "task_force:1 enters on day 3"),
            "day {}: {}",
            i + 1,
            said(reason)
        );
        assert!(said(reason).contains("is to be served by task_force:1, which enters on day 3"));
        // The intent is neither failed nor finished.
        assert!(
            day.commands.iter().all(|c| !matches!(
                c.kind,
                CommandKind::MarkPlan {
                    outcome: PlanOutcome::Failed | PlanOutcome::Complete,
                    ..
                }
            )),
            "day {}: no plan is marked",
            i + 1
        );
    }
    // On its day it is on the map and served as any carrier force.
    assert!(ordered(&handed[2], force), "ordered on day 3");
    assert!(handed[2]
        .no_orders
        .iter()
        .all(|r| !said(r).contains("enters on day")));
}

#[test]
fn a_view_counts_a_carrier_force_held_apart_when_a_limit_names_it() {
    let scenario = load(&carrier_path(), &carrier_text(3));
    let view = scenario.world.side_view(Side::Allied);
    assert_eq!(
        view.task_forces().count(),
        0,
        "it is in none of the view's forces"
    );
    let limit = limit_of(Some(1), None);
    let counted = counted_by_a_take(&view, Some(&limit));
    let ids: Vec<_> = counted.task_forces.iter().map(|f| f.id).collect();
    assert_eq!(ids, vec![TaskForceId::new(1)]);
    // With no limit only transport forces count, and this is a carrier force.
    assert!(counted_by_a_take(&view, None).task_forces.is_empty());
    // The enemy's view counts nothing of it.
    let enemy = scenario.world.side_view(Side::Japanese);
    assert!(counted_by_a_take(&enemy, Some(&limit))
        .task_forces
        .is_empty());
}

// ------------------------------------------------------- Men ashore, scenario 109

fn men_path() -> PathBuf {
    root().join("data/scenarios/men-ashore-no-more-coming/109_men_ashore_with_no_more_coming.toml")
}

/// Scenario 109 with a land unit W (id 3, 2,000 men) held apart until `day`
/// at the source base (base 1 at (0,0)), in the take's forces limit, and the
/// take's last day `last`.
fn men_text(day: u32, last: u32) -> String {
    let text = std::fs::read_to_string(men_path()).expect("scenario reads");
    let mark = "# MARK: units\n";
    assert_eq!(text.matches(mark).count(), 1);
    let w = format!(
        "[[land_unit]]\nid = 3\nname = \"Placeholder W\"\nside = \"japanese\"\nmen = 2000\nq = 0\nr = 0\nenters = {day}\n\n"
    );
    let text = text.replace(mark, &format!("{w}{mark}"));
    let needle = "land_units = [1, 2]";
    assert_eq!(text.matches(needle).count(), 1);
    let text = text.replace(needle, "land_units = [1, 2, 3]");
    let needle = "intent_kind = \"take_place\"\nhex = [14, 0]\n";
    assert_eq!(text.matches(needle).count(), 1);
    text.replace(needle, &format!("{needle}by_day = {last}\n"))
}

/// The judgement ashore of the morning of `day`.
fn judgement(handed: &[StaffDecisions], day: usize) -> String {
    let d = &handed[day - 1];
    d.orders
        .iter()
        .map(|o| format!("{} | {}", o.reason.worked, said(&o.reason)))
        .chain(d.no_orders.iter().map(said))
        .collect::<Vec<_>>()
        .join("\n")
        .lines()
        .find(|l| l.contains("the units that can attack have"))
        .unwrap_or_else(|| panic!("a judgement ashore on day {day}"))
        .to_string()
}

#[test]
fn a_land_unit_held_apart_is_counted_by_a_take_from_its_day() {
    let scenario = load(&men_path(), &men_text(7, 10));
    let view = scenario.world.side_view(Side::Japanese);
    assert!(view.land_units().all(|u| u.id != LandUnitId::new(3)));
    let all = counted_by_a_take(&view, None);
    let ids: Vec<_> = all.land_units.iter().map(|u| u.id.raw()).collect();
    assert_eq!(ids, vec![1, 2, 3], "in number order, W among them");
    let limit = limit_of(None, Some(3));
    let named = counted_by_a_take(&view, Some(&limit));
    let ids: Vec<_> = named.land_units.iter().map(|u| u.id.raw()).collect();
    assert_eq!(ids, vec![3]);
    assert_eq!(
        view.available_from(EntityRef::LandUnit(LandUnitId::new(3))),
        Some(7)
    );
}

#[test]
fn a_land_unit_entering_at_the_source_is_more_coming_with_its_day() {
    // W enters at the source on day 7, inside the take's last day, 10: more
    // is coming, the men wanted are those of while more is coming, the staff
    // waits (statement 9a's reading for a unit that is not on the map yet).
    let handed = play(&men_path(), &men_text(7, 10), Side::Japanese);
    for day in 4..=6 {
        let j = judgement(&handed, day);
        assert!(!j.contains("nothing more is coming"), "day {day}: {j}");
        assert!(
            j.contains("coming: no unit is left behind to bring (land_unit:3 enters on day 7)"),
            "day {day}: {j}"
        );
        assert!(
            handed[day - 1].orders.iter().all(|o| !matches!(
                &o.kind,
                OrderKind::LandUnit {
                    order: LandUnitOrder::SetPosture {
                        posture: Posture::Attack,
                        ..
                    },
                    ..
                }
            )),
            "day {day}: no unit is set to attack"
        );
    }
}

#[test]
fn a_land_unit_entering_out_of_time_is_not_coming_and_is_named_with_its_day() {
    let handed = play(&men_path(), &men_text(7, 5), Side::Japanese);
    let j = judgement(&handed, 4);
    assert!(
        j.contains("nothing more is coming (by day 5, the latest day)"),
        "{j}"
    );
    assert!(
        j.contains("land_unit:3 enters on day 7, after the latest day"),
        "{j}"
    );
}

/// Scenario 109 with a second lift, task force 5 (four troop transports with
/// W, 2,000 men, aboard) held apart until `day` at (2,0), in the take's
/// forces limit, and the take's last day `last`.
fn lift_text(day: u32, last: u32) -> String {
    let text = std::fs::read_to_string(men_path()).expect("scenario reads");
    let mut ships = String::new();
    for i in 11..=14 {
        ships.push_str(&format!(
            "[[ship]]\nid = {i}\nname = \"Placeholder troop transport {i}\"\nclass = 10\nside = \"japanese\"\nfuel_t = 1000\ntask_force = 5\n\n"
        ));
    }
    let mark = "# MARK: units\n";
    assert_eq!(text.matches(mark).count(), 1);
    let added = format!(
        "[[task_force]]\nid = 5\nname = \"Placeholder second lift\"\nside = \"japanese\"\nq = 2\nr = 0\nmission = \"transport\"\nenters = {day}\n\n{ships}[[land_unit]]\nid = 3\nname = \"Placeholder W\"\nside = \"japanese\"\nmen = 2000\ntask_force = 5\n\n"
    );
    let text = text.replace(mark, &format!("{added}{mark}"));
    let text = text.replace("land_units = [1, 2]", "land_units = [1, 2, 3]");
    let text = text.replace("task_forces = [1]", "task_forces = [1, 5]");
    let needle = "intent_kind = \"take_place\"\nhex = [14, 0]\n";
    assert_eq!(text.matches(needle).count(), 1);
    text.replace(needle, &format!("{needle}by_day = {last}\n"))
}

#[test]
fn a_lift_held_apart_with_fighting_men_aboard_is_more_coming_from_its_day() {
    // In time: more is coming, so the staff does not set the men to attack.
    let handed = play(&men_path(), &lift_text(7, 7), Side::Japanese);
    let j = judgement(&handed, 4);
    assert!(!j.contains("nothing more is coming"), "{j}");
    // Out of time (the take's last day is 5): it is named with its day.
    let handed = play(&men_path(), &lift_text(7, 5), Side::Japanese);
    let j = judgement(&handed, 4);
    assert!(
        j.contains("nothing more is coming (by day 5, the latest day)"),
        "{j}"
    );
    assert!(
        j.contains(
            "task_force:5 enters on day 7 with 2000 fighting men aboard, after the latest day"
        ),
        "{j}"
    );
}
