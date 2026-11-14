//! Cover, fieldworks, the grades of defences, strain and wear, in what a
//! side is told (decision 0106, 14).
//!
//! Scenario 34, on the live figures (the frozen ones switch the rule off).
//! The Allied battalion at A digs in; the Japanese bombard base 6 at F on
//! day 2 and base 1 at A on day 3; the Allied battalion at A is set to
//! attack on day 4. The Allied side is told of its own cover, fieldworks,
//! the grade of its defences, its digging, the wear on its fieldworks and
//! defences and its strain. The Japanese side is told none of it.

mod support;

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::ids::LandUnitId;
use sim_core::state::Side;
use sim_data::Scenario;
use support::check::valid;
use support::{scenario_path, scenario_text, schemas};

const NAME: &str = "34_cover.toml";

fn session_of(text: &str) -> Session {
    let scenario = Scenario::from_text(text, &scenario_path(NAME))
        .expect("the scenario loads")
        .with_seed(1942)
        .expect("a seed can be set");
    Session::new(
        scenario.world,
        "Cover",
        scenario.days,
        Seat::Player,
        Seat::Player,
    )
}

fn call(session: &mut Session, side: Side, tool: &str, args: &Value) -> Value {
    session.call(side, tool, args).expect("the tool answers")
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

fn orders(session: &mut Session, side: Side, orders: Value) {
    let answer = call(session, side, "issue_orders", &json!({ "orders": orders }));
    assert_eq!(answer["refused_in_call"], 0, "{answer}");
}

/// The Japanese forces 1 and 2 are those of scenario 34's; the orders are
/// those of its script, given through the tool, since the session does not
/// play the scripted ones.
fn japanese_to_f(session: &mut Session) {
    orders(
        session,
        Side::Japanese,
        json!([
            {"unit": "task_force:1", "order": {"kind": "set_route", "waypoints": [[40, 0]]}},
            {"unit": "task_force:1", "order": {"kind": "set_speed", "speed": "cruise"}}
        ]),
    );
}

fn japanese_to_a(session: &mut Session) {
    orders(
        session,
        Side::Japanese,
        json!([
            {"unit": "task_force:1", "order": {"kind": "set_route", "waypoints": [[38, 0]]}},
            {"unit": "task_force:2", "order": {"kind": "set_route", "waypoints": [[0, 0]]}},
            {"unit": "task_force:2", "order": {"kind": "set_speed", "speed": "cruise"}}
        ]),
    );
}

fn unit_line(b: &Value, handle: &str) -> Value {
    b["land_units"]["units"]
        .as_array()
        .expect("units")
        .iter()
        .find(|u| u["land_unit"]["handle"] == handle)
        .cloned()
        .unwrap_or_else(|| panic!("{handle} in {b}"))
}

/// Plays scenario 34 through the tools, giving the script's orders, and
/// hands each day's briefing of `side` (and its text) to `look` after the
/// day's resolution. Allied unit 1 is the battalion at A; the Japanese
/// forces 1 and 2 are the bombarding forces.
fn play(mut look: impl FnMut(&mut Session, u32)) {
    let mut s = session_of(&scenario_text(NAME));
    for day in 1..=5 {
        match day {
            2 => japanese_to_f(&mut s),
            3 => japanese_to_a(&mut s),
            4 => {
                orders(
                    &mut s,
                    Side::Japanese,
                    json!([{"unit": "task_force:2", "order": {"kind": "set_route", "waypoints": [[2, 0]]}}]),
                );
                orders(
                    &mut s,
                    Side::Allied,
                    json!([{"unit": "land_unit:1", "order": {"kind": "set_posture", "posture": "attack"}}]),
                );
            }
            5 => orders(
                &mut s,
                Side::Allied,
                json!([{"unit": "land_unit:1", "order": {"kind": "set_posture", "posture": "defend"}}]),
            ),
            _ => {}
        }
        end_day(&mut s);
        look(&mut s, day);
    }
}

fn lines<'a>(b: &'a Value, key: &str) -> Vec<&'a Value> {
    b["land_units"][key]
        .as_array()
        .map(|a| a.iter().collect())
        .unwrap_or_default()
}

fn base_grade(b: &Value, handle: &str) -> Option<String> {
    b["bases"]
        .as_array()
        .expect("bases")
        .iter()
        .find(|base| base["base"]["handle"] == handle)
        .and_then(|base| base["works"]["grade"].as_str().map(str::to_string))
}

#[test]
fn the_allied_side_sees_its_own_fieldworks_cover_grades_and_yesterdays_wear_and_strain() {
    let schemas = schemas();
    let mut dug = false;
    let mut worn_fieldworks = false;
    let mut lost = false;
    let mut dug_again = false;
    let mut defences_worn = false;
    let mut strained = false;
    play(|s, day| {
        let b = briefing(s, Side::Allied);
        valid(&schemas, "briefing.schema.json", &b, "allied");
        let t = text(s, Side::Allied);
        if day == 1 {
            // The battalion at A dug in on the first day: fieldworks 10,
            // and its cover is that (base A has no defences).
            let a = unit_line(&b, "land_unit:1");
            assert_eq!(a["fieldworks"], "10", "{a}");
            assert_eq!(a["cover"], "10", "{a}");
            assert!(t.contains("fieldworks 10; cover 10"), "{t}");
            // The battalion at D has fieldworks of 10 but is covered by its
            // base's defences of 80; a labour battalion digs nothing and is
            // sheltered by its base: cover and no fieldworks.
            let d = unit_line(&b, "land_unit:4");
            assert_eq!(
                (d["fieldworks"].as_str(), d["cover"].as_str()),
                (Some("10"), Some("80"))
            );
            let labour = unit_line(&b, "land_unit:2");
            assert!(labour.get("fieldworks").is_none(), "{labour}");
            assert_eq!(labour["cover"], "30", "{labour}");
            // The grades of the defences by the ground figures
            // (`prepared_from` 30, `fortified_from` 60).
            assert_eq!(base_grade(&b, "base:2").as_deref(), Some("prepared"));
            assert_eq!(base_grade(&b, "base:3").as_deref(), Some("prepared"));
            assert_eq!(base_grade(&b, "base:4").as_deref(), Some("fortified"));
            assert!(t.contains("defences 30 (prepared)"), "{t}");
            assert!(t.contains("defences 80 (fortified)"), "{t}");
            // Base A has no defences and no works line.
            assert_eq!(base_grade(&b, "base:1"), None);
        }
        for f in lines(&b, "fieldworks_yesterday") {
            assert!(f["event"].is_u64());
            let unit = f["land_unit"]["handle"].as_str().expect("handle");
            match f["change"].as_str().expect("change") {
                "dug" => {
                    dug = true;
                    if day == 5 && unit == "land_unit:1" {
                        // Back on defend with no fieldworks: digs from nothing.
                        assert_eq!(f["fieldworks"], "10", "{f}");
                        dug_again = true;
                    }
                }
                "worn" => worn_fieldworks = true,
                "lost" => {
                    assert_eq!((day, unit), (4, "land_unit:1"), "{f}");
                    assert_eq!(f["fieldworks"], "0", "{f}");
                    lost = true;
                }
                other => panic!("{other}"),
            }
        }
        if !lines(&b, "fieldworks_yesterday").is_empty() {
            assert!(t.contains("Fieldworks yesterday:"), "{t}");
        }
        for d in lines(&b, "defences_worn_yesterday") {
            // Base 6 at F, shelled on day 2.
            assert_eq!(d["base"]["handle"], "base:6", "{d}");
            assert_eq!(d["place"]["q"], 40, "{d}");
            assert!(d["hits"].as_u64().expect("hits") > 0);
            assert!(d["event"].is_u64());
            assert!(t.contains("Defences worn yesterday:"), "{t}");
            defences_worn = true;
        }
        for f in lines(&b, "fatigue_yesterday") {
            if f["change"] == "strained" {
                strained = true;
                assert!(t.contains("was under fire, did not rest, and tired"), "{t}");
            }
        }
        if day == 4 {
            // Set to attack, the battalion at A has no cover.
            let a = unit_line(&b, "land_unit:1");
            assert_eq!(a["posture"], "attack");
            assert!(
                a.get("cover").is_none() && a.get("fieldworks").is_none(),
                "{a}"
            );
        }
    });
    assert!(dug, "no unit was told it dug in");
    assert!(worn_fieldworks, "no unit was told its fieldworks were worn");
    assert!(lost, "no unit was told it left its fieldworks");
    assert!(
        dug_again,
        "the unit set to defend again was not told it dug in"
    );
    assert!(defences_worn, "no base was told its defences were worn");
    assert!(strained, "no unit was told it was strained");
}

#[test]
fn the_japanese_side_is_told_none_of_the_allied_fieldworks_cover_defences_or_wear() {
    let schemas = schemas();
    let mut seen_own_digging = false;
    play(|s, day| {
        let b = briefing(s, Side::Japanese);
        valid(&schemas, "briefing.schema.json", &b, "japanese");
        let t = text(s, Side::Japanese);
        let json = b.to_string();
        // Its own unit's digging it is told; the Allied battalions are
        // not on its list, and no base of the other side has works.
        let units = b["land_units"]["units"].as_array().expect("units");
        assert_eq!(units.len(), 1, "day {day}: {units:?}");
        for f in lines(&b, "fieldworks_yesterday") {
            assert_eq!(f["land_unit"]["handle"], "land_unit:1", "day {day}: {f}");
            assert!(
                t.contains("Placeholder Japanese battalion at D\" dug in"),
                "{t}"
            );
            seen_own_digging = true;
        }
        for word in [
            "defences_worn_yesterday",
            "Defences worn",
            "improvised",
            "prepared",
            "fortified",
            "\"grade\"",
            "strained",
            "did not rest",
            "left its fieldworks",
            "worn by fire",
        ] {
            assert!(!json.contains(word), "day {day}: {word} in {json}");
            assert!(!t.contains(word), "day {day}: {word} in {t}");
        }
        for base in b["bases"].as_array().expect("bases") {
            assert!(base.get("works").is_none(), "day {day}: {base}");
        }
        // Not the Allied units' state through the tools that list things.
        let bases = call(s, Side::Japanese, "list_bases", &json!({}));
        let bases = bases.to_string();
        assert!(!bases.contains("defences"), "day {day}: {bases}");
        assert!(!bases.contains("grade"), "day {day}: {bases}");
        let forces = call(s, Side::Japanese, "list_forces", &json!({}));
        let own = forces["land_units"].as_array().expect("land units");
        assert_eq!(own.len(), 1, "day {day}: {forces}");
        assert_eq!(own[0]["land_unit"]["handle"], "land_unit:1");
        // The shells that wore the Allied defences and fieldworks are its
        // own; the wear and the men's state are not told.
        let contacts = call(s, Side::Japanese, "list_contacts", &json!({})).to_string();
        assert!(!contacts.contains("fieldworks"), "{contacts}");
        assert!(!contacts.contains("cover"), "{contacts}");
    });
    assert!(
        seen_own_digging,
        "the Japanese battalion's own digging was not told"
    );
}

#[test]
fn a_side_has_no_cover_to_ask_of_an_enemy_unit() {
    // Allied unit 1 is the battalion at A, Japanese unit 5 the one at D.
    let scenario = Scenario::from_text(&scenario_text(NAME), &scenario_path(NAME))
        .expect("the scenario loads");
    let japanese = scenario.world.side_view(Side::Japanese);
    let allied = scenario.world.side_view(Side::Allied);
    assert_eq!(japanese.cover(LandUnitId::new(1)), None);
    assert!(japanese.cover(LandUnitId::new(5)).is_some());
    assert_eq!(allied.cover(LandUnitId::new(5)), None);
    assert!(allied.cover(LandUnitId::new(1)).is_some());
}
