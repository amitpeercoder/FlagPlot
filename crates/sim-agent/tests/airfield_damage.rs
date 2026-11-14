//! An airfield damaged and mended, in what a side is told (decision 0103,
//! 11).
//!
//! Scenario 31: the Allied side's bases 1 and 2 start with airfield damage 30
//! and 60, the Japanese side's base 3 with none. Group 1 and group 2 (12
//! ready each) are at base 1; group 3 (4 ready, 4 pilots) and groups 4 and 5
//! (20 ready each) at base 2. Work mends base 1 by 10 points a day. An
//! arrival of 6 aircraft for group 3 is due at base 2 on day 2 and waits,
//! because 60 is above the level, 50. A side is shown the damage of its own
//! fields whole and the aircraft they keep on the ground; the other side is
//! shown none of it.

mod support;

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::state::Side;
use support::check::valid;
use support::{load_text, scenario_text, schemas};

const NAME: &str = "31_airfield_damage.toml";

fn session_of(text: &str) -> Session {
    let scenario = load_text(text, NAME, 1942);
    Session::new(
        scenario.world,
        "Field",
        scenario.days,
        Seat::Player,
        Seat::Player,
    )
}

fn session() -> Session {
    session_of(&scenario_text(NAME))
}

/// The scenario with the Japanese base 3 damaged to `damage`.
fn text_with_base_3_damaged(damage: u32) -> String {
    let text = scenario_text(NAME);
    let at = text.find("id = 3\n").expect("base 3");
    let line = text[at..].find("supply_t = 5000\n").expect("its supply") + at;
    let end = line + "supply_t = 5000\n".len();
    format!(
        "{}airfield_damage = {damage}\n{}",
        &text[..end],
        &text[end..]
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

fn end_day(session: &mut Session) {
    for side in [Side::Allied, Side::Japanese] {
        call(session, side, "end_day", &json!({}));
    }
}

fn briefing(session: &mut Session, side: Side) -> Value {
    call(session, side, "get_briefing", &json!({"text": false}))
}

fn text(session: &mut Session, side: Side) -> String {
    call(session, side, "get_briefing", &json!({"text": true}))["text"]
        .as_str()
        .expect("text")
        .to_string()
}

#[test]
fn a_side_sees_the_damage_of_its_own_fields_and_the_aircraft_they_keep() {
    let mut s = session();
    let schemas = schemas();
    let b = briefing(&mut s, Side::Allied);
    valid(&schemas, "briefing.schema.json", &b, "airfield damage");

    let first = &item(&b["bases"], "base", "base:1")["airfield_damage"];
    assert_eq!(first["damage"], "30");
    assert_eq!(first["share_that_flies"], "70");
    assert_eq!(first["takes_aircraft"], true);
    assert_eq!(first["refused_at"], "50");
    let second = &item(&b["bases"], "base", "base:2")["airfield_damage"];
    assert_eq!(second["damage"], "60");
    assert_eq!(second["share_that_flies"], "40");
    assert_eq!(second["takes_aircraft"], false);

    // Of 12 ready at damage 30, 70 in 100 may fly, 8; 4 stay on the ground.
    let groups = &b["air_groups"];
    for (handle, kept, damage) in [
        ("air_group:1", 4, "30"),
        ("air_group:2", 4, "30"),
        // 4 crewed at 60: 40 in 100 is 1.6, cut to 1; 3 stay.
        ("air_group:3", 3, "60"),
        // 20 at 60: 8 may fly, 12 stay.
        ("air_group:4", 12, "60"),
    ] {
        let line = &item(groups, "air_group", handle)["kept_on_ground_by_field"];
        assert_eq!(line["aircraft"], kept, "{handle}");
        assert_eq!(line["damage"], damage, "{handle}");
    }

    // The same through list_bases and get_entity.
    let bases = call(&mut s, Side::Allied, "list_bases", &json!({}));
    valid(
        &schemas,
        "list_bases.schema.json",
        &bases,
        "airfield damage",
    );
    assert_eq!(
        item(&bases["bases"], "base", "base:2")["airfield_damage"]["damage"],
        "60"
    );
    let entity = call(
        &mut s,
        Side::Allied,
        "get_entity",
        &json!({"handle": "base:1"}),
    );
    valid(&schemas, "entity.schema.json", &entity, "airfield damage");
    assert_eq!(
        entity["entity"]["summary"]["airfield_damage"]["share_that_flies"],
        "70"
    );
    let group = call(
        &mut s,
        Side::Allied,
        "get_entity",
        &json!({"handle": "air_group:1"}),
    );
    valid(&schemas, "entity.schema.json", &group, "airfield damage");
    assert_eq!(
        group["entity"]["summary"]["kept_on_ground_by_field"]["aircraft"], 4,
        "{group}"
    );

    let t = text(&mut s, Side::Allied);
    assert!(
        t.contains("Airfield damage 30: 70 of every 100 aircraft may fly; it takes aircraft"),
        "{t}"
    );
    assert!(
        t.contains("Airfield damage 60: 40 of every 100 aircraft may fly; it takes no aircraft"),
        "{t}"
    );
    assert!(
        t.contains("4 of its aircraft kept on the ground by the damage (30) to the airfield"),
        "{t}"
    );
}

#[test]
fn a_field_with_no_damage_says_nothing_of_it() {
    let mut s = session();
    let b = briefing(&mut s, Side::Japanese);
    let own = item(&b["bases"], "base", "base:3");
    assert!(own.get("airfield_damage").is_none(), "{own}");
    assert!(own.get("airfield_yesterday").is_none(), "{own}");
    let all = b.to_string();
    assert!(!all.contains("airfield_damage"), "{all}");
    assert!(!all.contains("kept_on_ground_by_field"), "{all}");
}

#[test]
fn the_day_s_mending_and_the_waiting_arrival_are_in_the_briefing() {
    let mut s = session();
    let schemas = schemas();
    // A rebase to base 2 is refused, and the refusal says why.
    let order = call(
        &mut s,
        Side::Allied,
        "issue_orders",
        &json!({"orders": [
            {"unit": "air_group:2", "order": {"kind": "rebase", "base": 2}}
        ]}),
    );
    let refusal = order["results"][0].to_string();
    assert!(refusal.contains("refused"), "{order}");
    assert!(
        refusal
            .contains("the airfield's damage is 60, and a field at 50 or more takes no aircraft"),
        "{order}"
    );
    assert!(
        refusal.contains("once units working on its airfield"),
        "{order}"
    );
    end_day(&mut s);
    let t = text(&mut s, Side::Allied);
    let b = briefing(&mut s, Side::Allied);
    valid(&schemas, "briefing.schema.json", &b, "day 2");
    let first = item(&b["bases"], "base", "base:1");
    assert_eq!(first["airfield_damage"]["damage"], "20", "{first}");
    let mended = &first["airfield_yesterday"]["mended"][0];
    assert_eq!(mended["land_unit"]["handle"], "land_unit:1");
    assert_eq!(mended["points"], "10");
    assert_eq!(mended["damage"], "20");
    assert!(t.contains("Airfield mended yesterday: land_unit:1"), "{t}");

    // Day 2: the arrival due at base 2 waits, and nothing mends base 2.
    end_day(&mut s);
    let b = briefing(&mut s, Side::Allied);
    valid(&schemas, "briefing.schema.json", &b, "day 3");
    let second = item(&b["bases"], "base", "base:2");
    assert_eq!(second["airfield_damage"]["damage"], "60", "{second}");
    let waiting = &second["airfield_yesterday"]["arrivals_waiting"][0];
    assert_eq!(waiting["air_group"]["handle"], "air_group:3");
    assert_eq!(waiting["aircraft"], 6);
    assert_eq!(waiting["damage"], "60");
    assert!(second["airfield_yesterday"].get("mended").is_none());
    let t = text(&mut s, Side::Allied);
    assert!(
        t.contains("Arrival held back yesterday: 6 aircraft due at air_group:3"),
        "{t}"
    );
}

#[test]
fn a_side_is_told_the_damage_of_its_own_field_and_nothing_of_the_enemy_s() {
    let mut s = session_of(&text_with_base_3_damaged(40));
    let schemas = schemas();
    // The Japanese side sees base 3's damage and none of the Allied fields'.
    for _ in 0..3 {
        let b = briefing(&mut s, Side::Japanese);
        valid(&schemas, "briefing.schema.json", &b, "japanese");
        let own = item(&b["bases"], "base", "base:3");
        assert_eq!(own["airfield_damage"]["damage"], "40", "{own}");
        assert_eq!(own["airfield_damage"]["share_that_flies"], "60");
        assert_eq!(own["airfield_damage"]["takes_aircraft"], true);
        let t = text(&mut s, Side::Japanese);
        assert!(
            t.contains("Airfield damage 40: 60 of every 100 aircraft may fly"),
            "{t}"
        );
        assert!(!t.contains("Airfield damage 30"), "{t}");
        assert!(!t.contains("Airfield damage 60"), "{t}");
        assert!(!t.contains("mended"), "{t}");
        assert!(!t.contains("kept on the ground by the damage"), "{t}");
        let bases = call(&mut s, Side::Japanese, "list_bases", &json!({}));
        valid(&schemas, "list_bases.schema.json", &bases, "japanese");
        assert_eq!(
            item(&bases["bases"], "base", "base:3")["airfield_damage"]["damage"],
            "40"
        );
        let enemy = bases["enemy_bases"].to_string();
        assert!(!enemy.contains("damage"), "{enemy}");
        let entity = call(
            &mut s,
            Side::Japanese,
            "get_entity",
            &json!({"handle": "base:3"}),
        );
        assert_eq!(
            entity["entity"]["summary"]["airfield_damage"]["damage"],
            "40"
        );
        // An Allied base is an enemy base to this side: its public line only.
        let enemy_entity = call(
            &mut s,
            Side::Japanese,
            "get_entity",
            &json!({"handle": "base:1"}),
        );
        assert!(
            !enemy_entity.to_string().contains("damage"),
            "{enemy_entity}"
        );
        end_day(&mut s);
    }
}

/// What a side is given over the six days: the briefing as data and as text,
/// the lists and its base entities.
fn given(text: &str, side: Side) -> Vec<String> {
    let mut s = session_of(text);
    let mut out = Vec::new();
    for _ in 0..6 {
        out.push(briefing(&mut s, side).to_string());
        out.push(self::text(&mut s, side));
        out.push(call(&mut s, side, "list_bases", &json!({})).to_string());
        out.push(call(&mut s, side, "list_forces", &json!({})).to_string());
        for handle in ["base:1", "base:2", "base:3", "base:4"] {
            out.push(call(&mut s, side, "get_entity", &json!({"handle": handle})).to_string());
        }
        end_day(&mut s);
    }
    out
}

#[test]
fn two_worlds_that_differ_only_in_the_enemy_s_field_give_the_same_output() {
    let plain = scenario_text(NAME);
    for damage in [10, 40, 70] {
        let damaged = text_with_base_3_damaged(damage);
        assert_eq!(
            given(&plain, Side::Allied),
            given(&damaged, Side::Allied),
            "the Allied side was told of base 3's damage {damage}"
        );
    }
    // The control: the Japanese side meets the difference, its own field.
    assert_ne!(
        given(&plain, Side::Japanese),
        given(&text_with_base_3_damaged(40), Side::Japanese)
    );
}
