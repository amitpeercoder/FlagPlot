//! Playing a scenario through a session: a test player that gives the
//! scenario's own orders through `issue_orders` and ends each day.

use std::cell::RefCell;
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

/// An order as a saved game holds it: its unit and its ships by the
/// scenario's numbers.
pub fn order_json(request: &OrderRequest) -> Value {
    json!({ "unit": unit_text(&request.kind), "order": order_to_json(&request.kind) })
}

/// The side's own number of each unit by the scenario's number (decision
/// 0086), as `session_of` last saw the scenario on this thread. A task
/// force, an air group or a ship belongs to one side, so the scenario's
/// number alone says which.
#[derive(Default)]
struct Numbering {
    ships: BTreeMap<u32, (Side, u32)>,
    forces: BTreeMap<u32, (Side, u32)>,
    groups: BTreeMap<u32, (Side, u32)>,
}

thread_local! {
    static NUMBERING: RefCell<Numbering> = RefCell::new(Numbering::default());
}

fn register_numbering(world: &sim_core::World) {
    let mut numbering = Numbering::default();
    for side in Side::ALL {
        let view = world.side_view(side);
        let number = |handle: String| -> u32 {
            handle
                .split_once(':')
                .and_then(|(_, n)| n.parse().ok())
                .expect("a handle is kind:number")
        };
        for ship in view.ships() {
            numbering.ships.insert(
                ship.id.raw(),
                (side, number(super::ship_handle(&view, ship.id))),
            );
        }
        for force in view.task_forces() {
            numbering.forces.insert(
                force.id.raw(),
                (side, number(super::force_handle(&view, force.id))),
            );
        }
        for group in view.air_groups() {
            numbering.groups.insert(
                group.id.raw(),
                (side, number(super::group_handle(&view, group.id))),
            );
        }
    }
    NUMBERING.with(|n| *n.borrow_mut() = numbering);
}

/// An order of `side` as `issue_orders` takes it: the unit and the ships of a
/// detach by the side's own numbers. The numbers are those of the scenario of
/// the last `session_of` on this thread. A scripted order for a unit of the
/// other side, which a player cannot name (the full slice's Japanese order of
/// day 5), is given a number above every number of the side's own, which no
/// unit has.
pub fn served_json(side: Side, request: &OrderRequest) -> Value {
    NUMBERING.with(|n| {
        let n = n.borrow();
        let mut doc = order_json(request);
        let (kind, raw) = match &request.kind {
            sim_core::orders::OrderKind::TaskForce { target, .. } => ("task_force", target.raw()),
            sim_core::orders::OrderKind::AirGroup { target, .. } => ("air_group", target.raw()),
        };
        let (owner, number) = if kind == "task_force" {
            n.forces[&raw]
        } else {
            n.groups[&raw]
        };
        let number = if owner == side { number } else { 90_000 + raw };
        doc["unit"] = json!(format!("{kind}:{number}"));
        if let Some(ships) = doc["order"].get_mut("ships").and_then(Value::as_array_mut) {
            for ship in ships {
                let raw = ship.as_u64().expect("a ship number") as u32;
                let (owner, number) = n.ships[&raw];
                *ship = json!(if owner == side { number } else { 90_000 + raw });
            }
        }
        doc
    })
}

/// A session over the scenario with the given sides held by players and the
/// others by the script. Also returns the script of each side, for the test
/// players.
pub fn session_of(scenario: Scenario, players: &[Side]) -> (Session, BTreeMap<Side, ByDay>) {
    register_numbering(&scenario.world);
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
    let list: Vec<Value> = orders.iter().map(|o| served_json(side, o)).collect();
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
        let unit = super::force_handle(&view, force.id);
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
                json!({ "kind": "detach", "ships": [super::ship_handle(&view, *ship)
                .strip_prefix("ship:")
                .and_then(|n| n.parse::<u32>().ok())
                .expect("a ship number")] }),
            ));
        }
        out.push(order(
            unit.clone(),
            json!({ "kind": "detach", "ships": [999_999] }),
        ));
    }
    let groups: Vec<_> = view.air_groups().collect();
    for group in groups.iter().take(1).chain(groups.last()) {
        let unit = super::group_handle(&view, group.id);
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
