//! Units that work, and what they build, in what a side is told (decision
//! 0099).
//!
//! Scenario 30 has working units. The Allied side's land units are, in its
//! own numbers, `land_unit:1` to `land_unit:10` for scenario ids 1 to 7, 9, 11
//! and 12, and its bases `base:N` for scenario base id N (bases keep their numbers). Scenario base 1 has no airfield and 17 000 of work done on it; scenario
//! base 6 (`base:5`) has none, 5 000 done and defences 30; scenario unit 1
//! (`land_unit:1`) is a working unit at base 1. A side is shown the airfield,
//! work and defences of its own bases and nothing of the other side's: the
//! Japanese bases keep their public line.

mod support;

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::orders::{Activity, Job, LandUnitOrder, OrderKind};
use sim_core::state::Side;
use support::check::valid;
use support::{load, schemas};

fn session() -> Session {
    let scenario = load("30_working_units.toml", 1942);
    Session::new(
        scenario.world,
        "Works",
        scenario.days,
        Seat::Player,
        Seat::Player,
    )
}

fn call(session: &mut Session, side: Side, tool: &str, args: &Value) -> Value {
    session.call(side, tool, args).expect("the tool answers")
}

fn item<'a>(list: &'a Value, key: &str, handle: &str) -> &'a Value {
    list.as_array()
        .expect("a list")
        .iter()
        .find(|x| x[key]["handle"] == handle)
        .unwrap_or_else(|| panic!("{handle} in {list}"))
}

#[test]
fn a_side_sees_the_works_of_its_own_bases_and_units_whole() {
    let mut s = session();
    let schemas = schemas();

    let briefing = call(
        &mut s,
        Side::Allied,
        "get_briefing",
        &json!({"text": false}),
    );
    valid(&schemas, "briefing.schema.json", &briefing, "works");
    let first = item(&briefing["bases"], "base", "base:1");
    let works = &first["works"];
    assert_eq!(works["airfield"], false, "{first}");
    assert_eq!(works["airfield_work"], "17000", "{first}");
    assert!(works["airfield_work_needed"].is_string(), "{first}");
    assert_eq!(works["defences"], "0", "{first}");
    let fifth = item(&briefing["bases"], "base", "base:6");
    assert_eq!(fifth["works"]["airfield_work"], "5000", "{fifth}");
    assert_eq!(fifth["works"]["defences"], "30", "{fifth}");
    // A base with an airfield, no work and no defences says nothing of it.
    let second = item(&briefing["bases"], "base", "base:2");
    assert!(second.get("works").is_none(), "{second}");

    let unit = item(&briefing["land_units"]["units"], "land_unit", "land_unit:1");
    assert_eq!(unit["working"], true, "{unit}");
    assert_eq!(unit["skills"]["works"], 2, "{unit}");
    // A unit with no job named works at the default one: the airfield here.
    assert_eq!(unit["job"], "airfield", "{unit}");
    assert_eq!(unit["job_is_default"], true, "{unit}");
    let fighting = item(&briefing["land_units"]["units"], "land_unit", "land_unit:4");
    assert!(fighting.get("working").is_none(), "{fighting}");

    // The same through list_bases, list_forces and get_entity.
    let bases = call(&mut s, Side::Allied, "list_bases", &json!({}));
    valid(&schemas, "list_bases.schema.json", &bases, "works");
    assert_eq!(
        item(&bases["bases"], "base", "base:1")["works"]["airfield"],
        false
    );
    let forces = call(&mut s, Side::Allied, "list_forces", &json!({}));
    valid(&schemas, "list_forces.schema.json", &forces, "works");
    let entity = call(
        &mut s,
        Side::Allied,
        "get_entity",
        &json!({"handle": "base:1"}),
    );
    valid(&schemas, "entity.schema.json", &entity, "works");

    let text = call(&mut s, Side::Allied, "get_briefing", &json!({"text": true}));
    let text = text["text"].as_str().expect("text");
    assert!(text.contains("no airfield, 17000 of"), "{text}");
    assert!(
        text.contains("works at airfield (the default job)"),
        "{text}"
    );
}

#[test]
fn the_other_side_is_told_nothing_of_the_works() {
    let mut s = session();
    let briefing = call(
        &mut s,
        Side::Japanese,
        "get_briefing",
        &json!({"text": false}),
    );
    let all = briefing.to_string();
    for word in [
        "airfield_work",
        "defences",
        "\"works\":{",
        "\"working\"",
        "job",
    ] {
        assert!(
            !all.contains(word),
            "{word} in the Japanese briefing: {all}"
        );
    }
    let text = call(
        &mut s,
        Side::Japanese,
        "get_briefing",
        &json!({"text": true}),
    );
    let text = text["text"].as_str().expect("text");
    assert!(!text.contains("airfield"), "{text}");

    // The Allied bases as the Japanese side's list shows them: a name and a
    // place, no airfield and no work.
    let bases = call(&mut s, Side::Japanese, "list_bases", &json!({}));
    let enemy = bases["enemy_bases"].to_string();
    assert!(!enemy.contains("airfield"), "{enemy}");
    assert!(!enemy.contains("work"), "{enemy}");
    assert!(!enemy.contains("defences"), "{enemy}");
}

#[test]
fn set_activity_work_names_a_job_or_none() {
    let mut s = session();
    let order = call(
        &mut s,
        Side::Allied,
        "issue_orders",
        &json!({"orders": [
            {"unit": "land_unit:1", "order": {"kind": "set_activity", "activity": "work", "job": "defences"}},
            {"unit": "land_unit:4", "order": {"kind": "set_activity", "activity": "work"}},
            {"unit": "land_unit:4", "order": {"kind": "set_activity", "activity": "work", "job": "bridges"}},
            {"unit": "land_unit:1", "order": {"kind": "set_activity", "activity": "train"}}
        ]}),
    );
    let results = order["results"].as_array().expect("results");
    assert_eq!(results[0]["status"], "queued", "{order}");
    assert_eq!(results[1]["status"], "queued", "{order}");
    assert_ne!(results[2]["status"], "queued", "{order}");
    assert!(results[2].to_string().contains("bridges"), "{order}");
    // A working unit does not train: the reason is the engine's, with the
    // staff's hint.
    assert_eq!(results[3]["status"], "refused", "{order}");
    assert!(
        results[3].to_string().contains("a working unit works"),
        "{order}"
    );

    for side in [Side::Allied, Side::Japanese] {
        call(&mut s, side, "end_day", &json!({}));
    }
    let briefing = call(
        &mut s,
        Side::Allied,
        "get_briefing",
        &json!({"text": false}),
    );
    let unit = item(&briefing["land_units"]["units"], "land_unit", "land_unit:1");
    assert_eq!(unit["job"], "defences", "{unit}");
    assert!(
        briefing["land_units"]["work_yesterday"].is_array(),
        "{briefing}"
    );
    let text = call(&mut s, Side::Allied, "get_briefing", &json!({"text": true}));
    assert!(text["text"].as_str().unwrap().contains("Work yesterday:"));
    assert!(unit.get("job_is_default").is_none(), "{unit}");
}

#[test]
fn the_order_writes_and_parses_with_its_job() {
    let named = OrderKind::LandUnit {
        target: sim_core::ids::LandUnitId::new(3),
        order: LandUnitOrder::SetActivity(Activity::Work(Some(Job::Aircraft))),
    };
    let json = sim_agent::order_to_json(&named);
    assert_eq!(
        json,
        json!({"kind": "set_activity", "activity": "work", "job": "aircraft"})
    );
    assert_eq!(
        sim_agent::order_from_json("land_unit:3", &json).expect("parses"),
        named
    );
    let unnamed = OrderKind::LandUnit {
        target: sim_core::ids::LandUnitId::new(3),
        order: LandUnitOrder::SetActivity(Activity::Work(None)),
    };
    let json = sim_agent::order_to_json(&unnamed);
    assert_eq!(json, json!({"kind": "set_activity", "activity": "work"}));
    assert_eq!(
        sim_agent::order_from_json("land_unit:3", &json).expect("parses"),
        unnamed
    );
    assert!(sim_agent::order_from_json(
        "land_unit:3",
        &json!({"kind": "set_activity", "activity": "work", "job": "bridges"})
    )
    .is_err());
}
