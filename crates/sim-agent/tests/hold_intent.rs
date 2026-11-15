//! The measure of a hold intent (decision 0117, 14) and the tree's tools,
//! over scenario 31: base 2 starts with airfield damage 60, a unit mends 10 a
//! day from day 2 (the scenario's own comments: 50 on day 2, 40 on day 3), and
//! the level at which a field takes no aircraft is 50.
//!
//! Hand calculation: a hold on base 2 set for day 1 is in force 1 day after
//! day 1, and so on. The field is open at the end of a day when its damage is
//! below 50: day 1 (60) and day 2 (50) are not met; days 3 to 6 (40, 30, 20,
//! 10) are. So after day 6: 6 days in force, 4 met, no raid.

mod support;

use serde_json::{json, Value};
use sim_agent::{hold_measure, Seat, Session};
use sim_core::ids::IntentId;
use sim_core::state::Side;
use support::check::valid;
use support::{load_text, scenario_text, schemas};

const NAME: &str = "31_airfield_damage.toml";

fn text_with_hold(day: u32) -> String {
    format!(
        "{}\n[[command]]\nday = {day}\nside = \"allied\"\nkind = \"set_intent\"\nintent = 1\nintent_kind = \"hold\"\nbase = 2\n",
        scenario_text(NAME)
    )
}

#[test]
fn the_measure_counts_days_in_force_and_days_the_field_was_open() {
    let mut scenario = load_text(&text_with_hold(1), NAME, 1942);
    let expected = [(0, 0), (1, 0), (2, 0), (3, 1), (4, 2), (5, 3), (6, 4)];
    for (day, (in_force, met)) in expected.into_iter().enumerate() {
        let view = scenario.world.side_view(Side::Allied);
        assert_eq!(view.day() as usize, day);
        let measure = hold_measure(&view, IntentId::new(1));
        if day == 0 {
            // Scripted for day 1: not taken in before it begins.
            assert_eq!(measure, None);
        } else {
            let m = measure.expect("a hold intent");
            assert_eq!((m.days_in_force, m.days_met), (in_force, met), "day {day}");
            assert_eq!((m.raids, m.raids_fought), (0, 0));
        }
        scenario.next_day().expect("the day resolves");
    }
}

#[test]
fn a_hold_set_later_counts_from_its_own_day_and_the_other_side_has_none() {
    let mut scenario = load_text(&text_with_hold(4), NAME, 1942);
    for _ in 0..6 {
        scenario.next_day().expect("the day resolves");
    }
    let view = scenario.world.side_view(Side::Allied);
    let m = hold_measure(&view, IntentId::new(1)).expect("a hold intent");
    assert_eq!((m.days_in_force, m.days_met), (3, 3));
    // Not a hold intent, and not the side's: no measure.
    assert_eq!(hold_measure(&view, IntentId::new(2)), None);
    let japanese = scenario.world.side_view(Side::Japanese);
    assert_eq!(hold_measure(&japanese, IntentId::new(1)), None);
}

#[test]
fn the_tools_show_the_tree_the_measure_and_refuse_what_the_staff_made() {
    let scenario = load_text(&text_with_hold(1), NAME, 1942);
    let mut scenario = scenario;
    scenario.next_day().expect("day 1");
    let mut session = Session::new(
        scenario.world,
        "Hold",
        scenario.days,
        Seat::Player,
        Seat::Player,
    )
    .with_staffs(sim_staff::staffs());
    let schemas = schemas();
    let mut call = |tool: &str, args: Value| {
        let answer = session
            .call(Side::Allied, tool, &args)
            .expect("the tool answers");
        let kind = answer["kind"].as_str().expect("a kind").to_string();
        valid(&schemas, &format!("{kind}.schema.json"), &answer, tool);
        answer
    };
    let state = call("command_state", json!({}));
    let intent = &state["intents"][0];
    assert_eq!(intent["kind"], "hold");
    assert_eq!(intent["maker"], "commander");
    assert_eq!(intent["parent"], Value::Null);
    assert_eq!(intent["floor_days"], Value::Null);
    assert_eq!(intent["limits"]["risk"], Value::Null);
    assert_eq!(intent["limits_read_up"]["risk"], "balanced");
    assert_eq!(intent["limits_read_up"]["risk_set"], false);
    assert_eq!(intent["measure"]["days_in_force"], 1);

    // A cover intent, and a limit on groups.
    let a = call("set_intent", json!({"intent_kind": "cover", "base": 2}));
    assert_eq!(a["status"], "queued", "{a}");
    let a = call(
        "set_limit",
        json!({"limit": "forces", "intent": 2, "air_groups": [3]}),
    );
    assert_eq!(a["status"], "queued", "{a}");
    let a = call("hold", json!({"air_group": 3}));
    assert_eq!(a["status"], "queued", "{a}");
    assert_eq!(a["command"], json!({"kind": "hold", "air_group": 3}));
    // Days in force of a cover intent are refused as an argument.
    let error = session
        .call(
            Side::Allied,
            "set_intent",
            &json!({"intent_kind": "cover", "base": 2, "floor_days": 3}),
        )
        .expect_err("refused");
    assert!(error.message.contains("floor_days"), "{error}");
}
