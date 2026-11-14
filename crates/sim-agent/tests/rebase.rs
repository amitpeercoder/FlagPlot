//! Rebasing an air group in the agent's answers (decision 0091).
//!
//! A small scenario written here, on the two-airfields map and the
//! placeholder aircraft: Allied bases 1 at (0,0), 2 at (5,0) and 3 at (12,3),
//! and a Japanese base 4 at (7,0). One hex is 40 nm.
//!
//! The hand calculation, as in the decision's worked case:
//!
//! - Group 1, the placeholder flying boat (search range 600, strike 0), is at
//!   base 1 with 6 ready, 2 damaged and 8 pilots. Its transfer range is
//!   2 x 600 = 1,200 nm. Base 2 is 5 hexes, 200 nm away: the order is taken,
//!   the 2 damaged aircraft are written off at base 1, and the group is at
//!   base 2 with 6 ready, 0 damaged and 8 pilots.
//! - Group 2, the placeholder fighter (strike 250, search 0), is at base 1.
//!   Its transfer range is 2 x 250 = 500 nm. Base 3 at (12,3) is
//!   dq = 12, dr = 3, so n = 144 + 9 + 36 = 189 and the distance is
//!   40 x sqrt(189) = 549.9, whole miles 550, beyond 500: refused with the
//!   distance and the range. Base 1 is where it is: refused as already there.
//! - Base 4 is the Japanese side's: refused as not one of the side's bases.
//! - Group 3 is aboard the carrier, ship 1: refused as aboard a ship.
//! - Two rebase orders for group 1 in one day are both taken when given (the
//!   state has not changed); at the start of the day the first moves it and
//!   the second is refused as already moved that day.

mod support;

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::state::Side;
use support::check::valid;
use support::{load_text, schemas};

const SCENARIO: &str = r#"
name = "Placeholder rebase test"
placeholder = true
seed = 1942
days = 4
map = "../map/placeholder_two_airfields.toml"
ship_classes = "../reference/ship_classes.csv"
port_sizes = "../reference/port_sizes.csv"
aircraft_types = "../reference/aircraft_types.csv"
search = "../reference/search.toml"
surface_combat = "../reference/surface_combat.toml"
air_combat = "../reference/air_combat.toml"

[[base]]
id = 1
name = "Placeholder airfield 1"
side = "allied"
q = 0
r = 0
supply_t = 100

[[base]]
id = 2
name = "Placeholder airfield 2"
side = "allied"
q = 5
r = 0
supply_t = 100

[[base]]
id = 3
name = "Placeholder airfield 3"
side = "allied"
q = 12
r = 3
supply_t = 100

[[base]]
id = 4
name = "Placeholder airfield 4"
side = "japanese"
q = 7
r = 0
supply_t = 100

[[task_force]]
id = 1
name = "Placeholder carrier force"
side = "allied"
at_base = 1
mission = "carrier"

[[ship]]
id = 1
name = "Placeholder carrier 1"
class = 4
side = "allied"
fuel_t = 3000
task_force = 1

[[air_group]]
id = 1
name = "Placeholder flying boats"
side = "allied"
base = 1
aircraft_type = 1
ready = 6
damaged = 2
pilots = 8

[[air_group]]
id = 2
name = "Placeholder fighters"
side = "allied"
base = 1
aircraft_type = 4
ready = 4

[[air_group]]
id = 3
name = "Placeholder carrier scouts"
side = "allied"
ship = 1
aircraft_type = 2
ready = 4

[[air_group]]
id = 4
name = "Placeholder Japanese flying boats"
side = "japanese"
base = 4
aircraft_type = 1
ready = 5
"#;

fn session() -> Session {
    let scenario = load_text(SCENARIO, "01_transit.toml", 1942);
    Session::new(
        scenario.world,
        "Placeholder rebase test",
        scenario.days,
        Seat::Player,
        Seat::Player,
    )
}

fn rebase(group: &str, base: u32) -> Value {
    json!({ "unit": group, "order": { "kind": "rebase", "base": base } })
}

fn orders(session: &mut Session, list: &[Value]) -> Vec<Value> {
    let answer = session
        .call(Side::Allied, "issue_orders", &json!({ "orders": list }))
        .expect("issue_orders is answered");
    valid(&schemas(), "issue_orders.schema.json", &answer, "rebase");
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
    let data = session
        .call(side, "get_briefing", &json!({ "text": false }))
        .expect("the briefing");
    valid(&schemas(), "briefing.schema.json", &data, "rebase briefing");
    (text["text"].as_str().expect("text").to_string(), data)
}

fn group<'a>(data: &'a Value, handle: &str) -> &'a Value {
    data["air_groups"]
        .as_array()
        .expect("air groups")
        .iter()
        .find(|g| g["air_group"]["handle"] == handle)
        .unwrap_or_else(|| panic!("{handle} in {data}"))
}

#[test]
fn a_rebase_order_is_queued_then_done_and_the_next_briefing_shows_it() {
    let mut session = session();
    let results = orders(&mut session, &[rebase("air_group:1", 2)]);
    assert_eq!(results[0]["status"], "queued", "{results:?}");

    // Before the day nothing has moved.
    let (_, before) = briefing(&mut session, Side::Allied);
    assert_eq!(
        group(&before, "air_group:1")["location"]["handle"],
        "base:1"
    );
    assert!(group(&before, "air_group:1")
        .get("moved_yesterday")
        .is_none());

    end_day(&mut session);
    let (text, data) = briefing(&mut session, Side::Allied);

    let g = group(&data, "air_group:1");
    assert_eq!(g["location"]["handle"], "base:2");
    assert_eq!(g["ready"], 6);
    assert_eq!(g["damaged"], 0);
    assert_eq!(g["pilots"], 8);
    assert_eq!(g["moved_yesterday"], true);
    assert!(
        text.contains(", moved to this base yesterday and flew nothing yesterday"),
        "{text}"
    );

    let rebased = &data["losses"]["air_groups_rebased"][0];
    assert_eq!(rebased["air_group"]["handle"], "air_group:1");
    assert_eq!(rebased["from"]["handle"], "base:1");
    assert_eq!(rebased["to"]["handle"], "base:2");
    assert_eq!(rebased["distance_nm"], "200");
    assert_eq!(rebased["transfer_range_nm"], "1200");
    assert_eq!(rebased["ready"], 6);
    assert_eq!(rebased["pilots"], 8);
    assert!(
        text.contains("Rebased: air_group:1")
            && text.contains("200 miles (transfer range 1200 miles), with 6 ready and 8 pilots; it flew nothing that day"),
        "{text}"
    );

    // The 2 damaged aircraft are aircraft lost for good, as written off.
    let lost = data["losses"]["aircraft"]
        .as_array()
        .expect("aircraft")
        .iter()
        .find(|l| l["air_group"]["handle"] == "air_group:1")
        .expect("the group's loss");
    assert_eq!(lost["written_off"], 2);
    assert!(text.contains("2 written off"), "{text}");

    // The group's entity answer carries the same line.
    let entity = session
        .call(
            Side::Allied,
            "get_entity",
            &json!({ "handle": "air_group:1" }),
        )
        .expect("entity");
    valid(&schemas(), "entity.schema.json", &entity, "rebase entity");
    assert_eq!(entity["entity"]["summary"]["moved_yesterday"], true);

    // The two events are explained.
    let numbers: Vec<u64> = rebased["event"].as_u64().into_iter().collect();
    let explained = session
        .call(Side::Allied, "explain", &json!({ "event": numbers[0] }))
        .expect("explain");
    valid(
        &schemas(),
        "explain.schema.json",
        &explained,
        "rebase explain",
    );
    assert!(explained.to_string().contains("moved from"), "{explained}");

    // The next day it has moved no more.
    end_day(&mut session);
    let (_, later) = briefing(&mut session, Side::Allied);
    assert!(group(&later, "air_group:1")
        .get("moved_yesterday")
        .is_none());
    assert_eq!(group(&later, "air_group:1")["location"]["handle"], "base:2");
    assert!(later["losses"].get("air_groups_rebased").is_none());
}

#[test]
fn each_refusal_has_its_words() {
    let mut session = session();
    let results = orders(
        &mut session,
        &[
            rebase("air_group:2", 3),
            rebase("air_group:2", 1),
            rebase("air_group:1", 4),
            rebase("air_group:3", 2),
        ],
    );
    let reasons: Vec<&str> = results
        .iter()
        .map(|r| {
            assert_eq!(r["status"], "refused", "{r}");
            r["reason"].as_str().expect("reason")
        })
        .collect();
    assert!(
        reasons[0].starts_with(
            "the base is 550 nm away, beyond the air group's transfer range of 500 nm ("
        ) && reasons[0].contains("measure with question air_reach gives it"),
        "{}",
        reasons[0]
    );
    assert!(
        reasons[1].starts_with("the air group is already at that base (rebase needs another base"),
        "{}",
        reasons[1]
    );
    assert!(
        reasons[2].starts_with("the base is not one of your bases"),
        "{}",
        reasons[2]
    );
    assert!(
        reasons[3].starts_with("the air group is aboard a ship (rebase is for a group at a base"),
        "{}",
        reasons[3]
    );
}

#[test]
fn a_second_move_in_one_day_is_refused_when_the_day_begins() {
    let mut session = session();
    let results = orders(
        &mut session,
        &[rebase("air_group:1", 2), rebase("air_group:1", 3)],
    );
    assert_eq!(results[0]["status"], "queued");
    assert_eq!(results[1]["status"], "queued");
    end_day(&mut session);
    let (text, data) = briefing(&mut session, Side::Allied);
    assert_eq!(group(&data, "air_group:1")["location"]["handle"], "base:2");
    assert!(
        text.contains(
            "the air group has already moved to another base today (a group moves once a day"
        ),
        "{text}"
    );
}

#[test]
fn the_enemy_has_no_trace_of_it() {
    // The same game twice: with the rebase, and without any order. The
    // Japanese seat is told the same in every tool.
    let probe = |with_order: bool| -> Vec<String> {
        let mut session = session();
        if with_order {
            let results = orders(
                &mut session,
                &[rebase("air_group:1", 2), rebase("air_group:2", 3)],
            );
            assert_eq!(results[0]["status"], "queued");
            assert_eq!(results[1]["status"], "refused");
        }
        end_day(&mut session);
        let mut out = Vec::new();
        for tool in ["list_forces", "list_bases", "list_contacts"] {
            out.push(
                session
                    .call(Side::Japanese, tool, &json!({}))
                    .unwrap()
                    .to_string(),
            );
        }
        for text in [false, true] {
            out.push(
                session
                    .call(Side::Japanese, "get_briefing", &json!({ "text": text }))
                    .unwrap()
                    .to_string(),
            );
        }
        for handle in [
            "base:1",
            "base:2",
            "air_group:1",
            "air_group:2",
            "air_group:3",
        ] {
            out.push(
                session
                    .call(Side::Japanese, "get_entity", &json!({ "handle": handle }))
                    .unwrap()
                    .to_string(),
            );
        }
        out
    };
    let with = probe(true);
    let without = probe(false);
    assert_eq!(with, without);
    for answer in &with {
        let lower = answer.to_lowercase();
        assert!(
            !lower.contains("rebased")
                && !lower.contains("moved to this base")
                && !lower.contains("written off")
                && !lower.contains("transfer range"),
            "{answer}"
        );
    }
}

#[test]
fn measure_air_reach_gives_the_transfer_range() {
    let mut session = session();
    let answer = session
        .call(
            Side::Allied,
            "measure",
            &json!({ "question": "air_reach", "air_group": "air_group:2", "to": ["base:3", "base:2"] }),
        )
        .expect("measure");
    valid(&schemas(), "measure.schema.json", &answer, "air reach");
    assert_eq!(answer["transfer_range_nm"], "500");
    let flying_boat = session
        .call(
            Side::Allied,
            "measure",
            &json!({ "question": "air_reach", "air_group": "air_group:1", "to": ["base:3"] }),
        )
        .expect("measure");
    assert_eq!(flying_boat["transfer_range_nm"], "1200");
}
