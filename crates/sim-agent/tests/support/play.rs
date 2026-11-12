//! Playing a scenario through a session: a test player that gives the
//! scenario's own orders through `issue_orders` and ends each day.

use std::collections::BTreeMap;

use serde_json::{json, Value};
use sim_agent::{order_to_json, seat_name, unit_text, Seat, Session};
use sim_core::orders::OrderRequest;
use sim_core::state::Side;
use sim_data::Scenario;

pub type ByDay = BTreeMap<u32, Vec<OrderRequest>>;

/// The orders the scenario's script gives for one side, by day.
pub fn script_of(scenario: &Scenario, side: Side) -> ByDay {
    let mut by_day = ByDay::new();
    for (day, list) in scenario.scripted_orders() {
        for (who, request) in list {
            if *who == side {
                by_day.entry(*day).or_default().push(request.clone());
            }
        }
    }
    by_day
}

/// An order as `issue_orders` takes it.
pub fn order_json(request: &OrderRequest) -> Value {
    json!({ "unit": unit_text(&request.kind), "order": order_to_json(&request.kind) })
}

/// A session over the scenario with the given sides held by players and the
/// others by the script. Also returns the script of each side, for the test
/// players.
pub fn session_of(scenario: Scenario, players: &[Side]) -> (Session, BTreeMap<Side, ByDay>) {
    let scripts: BTreeMap<Side, ByDay> = Side::ALL
        .into_iter()
        .map(|side| (side, script_of(&scenario, side)))
        .collect();
    let seat = |side: Side| {
        if players.contains(&side) {
            Seat::Player
        } else {
            Seat::Script(scripts[&side].clone())
        }
    };
    let (allied, japanese) = (seat(Side::Allied), seat(Side::Japanese));
    let name = scenario.name.clone();
    let session = Session::new(scenario.world, &name, scenario.days, allied, japanese);
    (session, scripts)
}

/// One side's test player gives a day's orders and ends the day. Returns the
/// reasons of the orders refused, with their index, and the `end_day` answer.
pub fn play_day(
    session: &mut Session,
    side: Side,
    orders: &[OrderRequest],
) -> (Vec<(usize, String)>, Value) {
    let list: Vec<Value> = orders.iter().map(order_json).collect();
    let answer = session
        .call(side, "issue_orders", &json!({ "orders": list }))
        .expect("issue_orders is answered");
    let refused = answer["results"]
        .as_array()
        .expect("results")
        .iter()
        .filter(|r| r["status"] == "refused")
        .map(|r| {
            (
                r["index"].as_u64().expect("index") as usize,
                r["reason"].as_str().expect("reason").to_string(),
            )
        })
        .collect();
    let ended = session
        .call(side, "end_day", &json!({}))
        .expect("end_day is answered");
    (refused, ended)
}

/// What a played game left: the daily hashes, every event as text, and the
/// orders the player's engine check refused, as `(day, side, index, reason)`.
pub struct Played {
    pub hashes: Vec<u64>,
    pub events: Vec<String>,
    pub refused: Vec<(u32, Side, usize, String)>,
    pub session_hashes: Vec<u64>,
    pub saved_orders: BTreeMap<u32, Vec<(Side, OrderRequest)>>,
}

/// Plays every day with test players on `players`, each issuing the scenario's
/// scripted orders for its side and ending the day, in the order of `players`.
pub fn play_as(scenario: Scenario, players: &[Side]) -> Played {
    let (mut session, scripts) = session_of(scenario, players);
    let mut refused = Vec::new();
    for day in 1..=session.days() {
        let mut last = Value::Null;
        for side in players {
            let orders = scripts[side].get(&day).cloned().unwrap_or_default();
            let (bad, ended) = play_day(&mut session, *side, &orders);
            refused.extend(bad.into_iter().map(|(i, r)| (day, *side, i, r)));
            last = ended;
        }
        assert_eq!(last["resolved"], true, "day {day}: {last}");
        assert_eq!(session.day(), day);
    }
    assert!(session.is_over());
    let session_hashes = session.day_hashes().to_vec();
    let saved_orders = session.handed_in().clone();
    let world = session.into_world();
    Played {
        hashes: world.day_hashes().to_vec(),
        events: world.events().iter().map(|e| format!("{e:?}")).collect(),
        refused,
        session_hashes,
        saved_orders,
    }
}

/// The scenario run as it always was: the hashes and every event as text.
pub fn run_scripted(mut scenario: Scenario) -> (Vec<u64>, Vec<String>) {
    scenario.run_to_end().expect("the run resolves");
    (
        scenario.world.day_hashes().to_vec(),
        scenario
            .world
            .events()
            .iter()
            .map(|e| format!("{e:?}"))
            .collect(),
    )
}

pub fn seat_label(side: Side) -> &'static str {
    seat_name(side)
}

/// A battery of orders made from a side's own view alone: every number of a
/// task force and of an air group up to 40 (the enemy's among them, though the
/// side cannot know which) with one order each; every kind of order on the
/// side's first own task force and first own air group; and a base strike on
/// every base number up to 40 from the first and the last own air group. A
/// base strike is judged by the public facts of the bases, which the side can
/// list (decision 0050, as amended).
pub fn probe_battery(world: &sim_core::World, side: Side) -> Vec<Value> {
    let view = world.side_view(side);
    let mut out = Vec::new();
    let order = |unit: String, body: Value| json!({ "unit": unit, "order": body });
    for n in 1..=40 {
        out.push(order(
            format!("task_force:{n}"),
            json!({ "kind": "set_speed", "speed": "cruise" }),
        ));
        out.push(order(
            format!("air_group:{n}"),
            json!({ "kind": "set_air_mission", "mission": "rest" }),
        ));
    }
    let mut places: Vec<(i32, i32)> = view.bases().map(|b| (b.hex.q, b.hex.r)).collect();
    places.extend(view.enemy_bases().map(|b| (b.hex.q, b.hex.r)));
    places.extend(
        view.contacts()
            .map(|c| (c.estimated_hex.q, c.estimated_hex.r)),
    );
    if let Some(force) = view.task_forces().next() {
        let unit = format!("task_force:{}", force.id.raw());
        let here = force.position();
        for (q, r) in places.iter().copied().chain([(here.q + 1, here.r)]) {
            out.push(order(
                unit.clone(),
                json!({ "kind": "set_route", "waypoints": [[q, r]] }),
            ));
        }
        for body in [
            json!({ "kind": "set_readiness", "readiness": "battle" }),
            json!({ "kind": "load" }),
            json!({ "kind": "load", "cargo": "fuel" }),
            json!({ "kind": "unload" }),
            json!({ "kind": "set_speed", "knots": "0" }),
            json!({ "kind": "set_mission", "mission": "escort" }),
        ] {
            out.push(order(unit.clone(), body));
        }
        for n in 1..=20 {
            out.push(order(
                unit.clone(),
                json!({ "kind": "set_home_port", "base": n }),
            ));
        }
        if let Some(ship) = force.ships.iter().next() {
            out.push(order(
                unit.clone(),
                json!({ "kind": "detach", "ships": [ship.raw()] }),
            ));
        }
        out.push(order(
            unit.clone(),
            json!({ "kind": "detach", "ships": [999_999] }),
        ));
    }
    let groups: Vec<_> = view.air_groups().collect();
    for group in groups.iter().take(1).chain(groups.last()) {
        let unit = format!("air_group:{}", group.id.raw());
        for mission in ["rest", "search", "naval_strike", "combat_air_patrol"] {
            out.push(order(
                unit.clone(),
                json!({ "kind": "set_air_mission", "mission": mission }),
            ));
        }
        out.push(order(
            unit.clone(),
            json!({ "kind": "set_search_arc", "from_deg": 0, "to_deg": 90, "range_nm": "100" }),
        ));
        out.push(order(
            unit.clone(),
            json!({ "kind": "set_patrol_share", "share": "0.5" }),
        ));
        out.push(order(
            unit.clone(),
            json!({ "kind": "set_rest_threshold", "fatigue": "50" }),
        ));
        for n in 1..=40 {
            out.push(order(
                unit.clone(),
                json!({ "kind": "set_air_mission", "mission": "base_strike", "target_base": n }),
            ));
        }
    }
    out
}

/// The answer to `issue_orders` with the battery, from a copy of the world,
/// as JSON text.
pub fn probe_answer(world: &sim_core::World, side: Side, title: &str) -> String {
    let battery = probe_battery(world, side);
    let mut session = Session::new(
        world.clone(),
        title,
        world.day() + 5,
        if side == Side::Allied {
            Seat::Player
        } else {
            Seat::Script(ByDay::new())
        },
        if side == Side::Japanese {
            Seat::Player
        } else {
            Seat::Script(ByDay::new())
        },
    );
    let answer = session
        .call(
            side,
            "issue_orders",
            &json!({ "orders": battery, "clear": true }),
        )
        .expect("the battery is answered");
    answer.to_string()
}
