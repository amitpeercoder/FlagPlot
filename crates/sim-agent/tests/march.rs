//! Marching overland as the agent interface tells it (decision 0094).
//!
//! Scenario 25: four coast hexes in a row, P (0,0), Q (1,0), R (2,0) and
//! S (3,0), land links P-Q and R-S only. The Allied regiment (the side's
//! `land_unit:1`) marches from P to Q on day 1 with its base's supply and
//! forms a base there; the Japanese company at R (the Japanese side's
//! `land_unit:2`) marches to S from a shore with no base and carries nothing.
//! The Japanese battalion at Q is the Japanese side's `land_unit:1`.
//!
//! A march takes 3 days in these figures: day 1 and day 2 are marched in
//! place, and the unit arrives in day 3's ground combat phase. So the
//! briefing of the start of day 2 is told one day marched, of 3.

mod support;

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::state::Side;
use support::check::{check_view, valid};
use support::{load, schemas};

fn session() -> Session {
    let scenario = load("25_march.toml", 1942);
    Session::new(
        scenario.world,
        "Scenario 25",
        scenario.days,
        Seat::Player,
        Seat::Player,
    )
}

fn orders(session: &mut Session, side: Side, list: &[Value]) -> Vec<Value> {
    let answer = session
        .call(side, "issue_orders", &json!({ "orders": list }))
        .expect("issue_orders is answered");
    valid(&schemas(), "issue_orders.schema.json", &answer, "march");
    answer["results"].as_array().expect("results").clone()
}

fn end_day(session: &mut Session) {
    for side in [Side::Allied, Side::Japanese] {
        session
            .call(side, "end_day", &json!({}))
            .expect("end_day is answered");
    }
}

fn briefing(session: &mut Session, side: Side) -> (String, Value) {
    let text = session
        .call(side, "get_briefing", &json!({ "text": true }))
        .expect("the briefing");
    valid(&schemas(), "briefing_text.schema.json", &text, "march text");
    let data = session
        .call(side, "get_briefing", &json!({ "text": false }))
        .expect("the briefing");
    valid(&schemas(), "briefing.schema.json", &data, "march briefing");
    (text["text"].as_str().expect("text").to_string(), data)
}

fn march(unit: &str, q: i32, r: i32) -> Value {
    json!({ "unit": unit, "order": { "kind": "march", "hex": [q, r] } })
}

fn by_handle<'a>(list: &'a Value, key: &str, handle: &str) -> &'a Value {
    list.as_array()
        .expect("a list")
        .iter()
        .find(|x| x[key]["handle"] == handle)
        .unwrap_or_else(|| panic!("{handle} in {list}"))
}

fn unit_line<'a>(data: &'a Value, handle: &str) -> &'a Value {
    by_handle(&data["land_units"]["units"], "land_unit", handle)
}

fn coords(places: &Value) -> Vec<(i64, i64)> {
    places
        .as_array()
        .expect("places")
        .iter()
        .map(|p| (p["q"].as_i64().unwrap(), p["r"].as_i64().unwrap()))
        .collect()
}

/// Both sides march on day 1 and the days are played.
fn played(days: u32) -> Session {
    let mut session = session();
    let allied = orders(&mut session, Side::Allied, &[march("land_unit:1", 1, 0)]);
    assert_eq!(allied[0]["status"], "queued", "{allied:?}");
    let japanese = orders(&mut session, Side::Japanese, &[march("land_unit:2", 3, 0)]);
    assert_eq!(japanese[0]["status"], "queued", "{japanese:?}");
    for _ in 0..days {
        end_day(&mut session);
    }
    session
}

#[test]
fn a_unit_ashore_lists_where_it_can_march_and_a_march_shows_its_days() {
    let mut session = session();
    // Before any order: no march, and the hexes it can march to.
    let (text, data) = briefing(&mut session, Side::Allied);
    let u = unit_line(&data, "land_unit:1");
    assert!(u.get("march").is_none(), "{u}");
    assert_eq!(coords(&u["march_to"]), vec![(1, 0)], "{u}");
    assert!(text.contains("can march to (1,0)"), "{text}");
    // The Japanese company at R can go to S only; the battalion at Q to P.
    let (_, data) = briefing(&mut session, Side::Japanese);
    assert_eq!(
        coords(&unit_line(&data, "land_unit:2")["march_to"]),
        vec![(3, 0)]
    );
    assert_eq!(
        coords(&unit_line(&data, "land_unit:1")["march_to"]),
        vec![(0, 0)]
    );

    let mut session = played(1);
    let (text, data) = briefing(&mut session, Side::Allied);
    let u = unit_line(&data, "land_unit:1");
    assert_eq!(u["march"]["to"]["q"], 1, "{u}");
    assert_eq!(u["march"]["days_marched"], 1, "{u}");
    assert_eq!(u["march"]["of_days"], 3, "{u}");
    assert_eq!(u["march"]["order"], 1, "{u}");
    assert!(
        text.contains("marching to (1,0), 1 of 3 days marched (1 counted) (by your order 1)"),
        "{text}"
    );
    let day = &data["land_units"]["marches_yesterday"];
    assert_eq!(day.as_array().unwrap().len(), 1, "{day}");
    assert_eq!(day[0]["land_unit"]["handle"], "land_unit:1");
    assert_eq!(
        (day[0]["days"].as_u64(), day[0]["of_days"].as_u64()),
        (Some(1), Some(3))
    );
    assert!(
        text.contains("marched from (0,0) Placeholder base P, Allied toward (1,0), 1 of 3 days"),
        "{text}"
    );
    // The same fields in get_entity and list_forces.
    let entity = session
        .call(
            Side::Allied,
            "get_entity",
            &json!({ "handle": "land_unit:1" }),
        )
        .expect("answered");
    valid(&schemas(), "entity.schema.json", &entity, "march entity");
    assert_eq!(
        entity["entity"]["summary"]["march"]["days_marched"], 1,
        "{entity}"
    );
    assert_eq!(
        coords(&entity["entity"]["summary"]["march_to"]),
        vec![(1, 0)],
        "{entity}"
    );
    let forces = session
        .call(Side::Allied, "list_forces", &json!({}))
        .expect("answered");
    valid(
        &schemas(),
        "list_forces.schema.json",
        &forces,
        "march forces",
    );
    let line = by_handle(&forces["land_units"], "land_unit", "land_unit:1");
    assert_eq!(line["march"]["of_days"], 3, "{line}");
}

#[test]
fn the_arrival_carries_its_supply_forms_its_base_and_tires_the_unit() {
    let mut session = played(3);
    let (text, data) = briefing(&mut session, Side::Allied);
    let land = &data["land_units"];

    let arrived = &land["march_arrivals_yesterday"];
    assert_eq!(arrived.as_array().unwrap().len(), 1, "{arrived}");
    assert_eq!(arrived[0]["men"], 1000);
    assert_eq!(arrived[0]["supply_t"], "25");
    assert_eq!(arrived[0]["base"]["handle"], "base:3");
    assert_eq!(arrived[0]["to"]["q"], 1);
    assert_eq!(arrived[0]["from"]["q"], 0);
    assert!(
        text.contains("Arrived yesterday: 1000 men of land_unit:1"),
        "{text}"
    );

    let formed = &land["bases_formed_by_march_yesterday"];
    assert_eq!(formed[0]["base"]["handle"], "base:3", "{formed}");
    assert_eq!(formed[0]["land_unit"]["handle"], "land_unit:1");
    assert!(
        text.contains("carrying 25 t of supply into base:3")
            && text.contains("by the march of land_unit:1")
            && text.contains("it is on your map only"),
        "{text}"
    );

    // Marched and fired on attack that day: both fatigue changes in one.
    let tired = by_handle(&land["fatigue_yesterday"], "land_unit", "land_unit:1");
    assert_eq!(tired["change"], "marched_and_attacked", "{tired}");
    assert!(text.contains("marched, attacked and tired"), "{text}");

    // The unit is at Q now, its march spent.
    let u = unit_line(&data, "land_unit:1");
    assert_eq!(u["place"]["q"], 1, "{u}");
    assert!(u.get("march").is_none(), "{u}");

    // The supply: out of base 1 for the march, into base 3 as stock.
    let bases = data["bases"].as_array().expect("bases");
    let supply = |handle: &str| {
        let b = bases
            .iter()
            .find(|b| b["base"]["handle"] == handle)
            .unwrap_or_else(|| panic!("{handle}"));
        b["stocks_yesterday"]["supply"].clone()
    };
    let p = supply("base:1");
    assert_eq!(p["taken_by_marches_t"], "25", "{p}");
    assert!(p.get("arrived_with_marches_t").is_none(), "{p}");
    let q = supply("base:3");
    assert_eq!(q["arrived_with_marches_t"], "25", "{q}");
    assert_eq!(q["start_t"], "0", "{q}");
    assert_eq!(q["end_t"], "25", "{q}");
    assert!(text.contains("taken by marching units 25 t"), "{text}");
    assert!(text.contains("brought by marching units 25 t"), "{text}");
}

#[test]
fn an_unreachable_hex_is_refused_with_where_the_unit_can_march_and_the_unit_stops_when_told() {
    let mut session = played(3);
    let results = orders(&mut session, Side::Allied, &[march("land_unit:1", 2, 0)]);
    assert_eq!(results[0]["status"], "refused", "{results:?}");
    let reason = results[0]["reason"].as_str().expect("a reason");
    assert!(
        reason.starts_with("no land link joins the land unit's hex to (2,0)")
            && reason.contains("march_to"),
        "{reason}"
    );
    // A march of the unit's own hex is accepted (it stops a march if there is
    // one); and one from a unit that is not the side's own is refused alike.
    let results = orders(
        &mut session,
        Side::Allied,
        &[march("land_unit:1", 1, 0), march("land_unit:2", 3, 0)],
    );
    assert_eq!(results[0]["status"], "queued", "{results:?}");
    assert_eq!(results[1]["status"], "refused", "{results:?}");

    // A stop mid-march is told with its reason.
    let mut session = session_marching_one_day();
    let results = orders(&mut session, Side::Allied, &[march("land_unit:1", 0, 0)]);
    assert_eq!(results[0]["status"], "queued", "{results:?}");
    let (text, data) = briefing(&mut session, Side::Allied);
    let stopped = &data["land_units"]["marches_stopped_yesterday"];
    assert!(
        stopped.is_null(),
        "a stop in the coming day is not yesterday's: {stopped}"
    );
    end_day(&mut session);
    let (text2, data) = briefing(&mut session, Side::Allied);
    let stopped = &data["land_units"]["marches_stopped_yesterday"];
    assert_eq!(
        stopped[0]["reason"], "ordered",
        "{stopped}\n{text}\n{text2}"
    );
    assert_eq!(stopped[0]["days"], 1);
    assert!(
        text2.contains("stopped its march to (1,0) after 1 day: you ordered it to stay"),
        "{text2}"
    );
    let u = unit_line(&data, "land_unit:1");
    assert!(u.get("march").is_none(), "{u}");
    assert_eq!(u["place"]["q"], 0);
}

fn session_marching_one_day() -> Session {
    played(1)
}

#[test]
fn the_march_of_each_side_is_told_to_that_side_alone_on_every_day() {
    let schemas = schemas();
    let mut scenario = load("25_march.toml", 1942);
    let name = scenario.name.clone();
    let mut told_allied = String::new();
    let mut told_japanese = String::new();
    support::play(&mut scenario, |s| {
        for side in [Side::Allied, Side::Japanese] {
            let view = s.world.side_view(side);
            let checked = check_view(&view, &name, &schemas, true);
            let mut all = checked.text.clone();
            for answer in checked.answers.values() {
                all.push_str(answer);
            }
            let day = view.day();
            match side {
                Side::Allied => {
                    // The Japanese march (unit 3 from R to S, the Japanese
                    // side's land_unit:2) is told in nothing of the Allied
                    // side's: no unit, no hex S, no name.
                    for word in [
                        "land_unit:2",
                        "(3,0)",
                        "\"q\":3",
                        "Japanese company",
                        "toward (3,0)",
                        "(2,0) toward",
                    ] {
                        assert!(
                            !all.contains(word),
                            "day {day}: the Allies were told {word}"
                        );
                    }
                    told_allied.push_str(&checked.text);
                }
                Side::Japanese => {
                    // The Allied march (a unit of 1,000 men from P to Q, a
                    // base formed at Q) is told in nothing of the Japanese
                    // side's but what a ground report gives: an estimate of
                    // men at Q, and base:3 found on day 3.
                    for word in [
                        "land_unit:3",
                        "Allied regiment",
                        "(0,0) toward",
                        "1000 men",
                        "carrying 25 t",
                        "marching units",
                        "formed yesterday: base:3",
                        "taken_by_marches_t",
                        "arrived_with_marches_t",
                    ] {
                        assert!(
                            !all.contains(word),
                            "day {day}: the Japanese were told {word}"
                        );
                    }
                    if day < 3 {
                        assert!(
                            !all.contains("base:3"),
                            "day {day}: base:3 before it was found"
                        );
                    } else {
                        assert!(
                            all.contains("base:3"),
                            "day {day}: base:3 is known from day 3, by a ground report"
                        );
                    }
                    told_japanese.push_str(&checked.text);
                }
            }
        }
    });
    // What each side is told of its own march is there.
    assert!(told_allied.contains("Arrived yesterday: 1000 men of land_unit:1"));
    assert!(told_japanese.contains("Arrived yesterday: 87 men of land_unit:2"));
    assert!(told_japanese.contains("carrying no supply"));
    assert!(told_japanese.contains("Enemy base found yesterday: base:3"));
}
