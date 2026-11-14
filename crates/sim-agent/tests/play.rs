//! Playing through a session (decisions 0050).

mod support;

use sim_core::state::Side;
use support::load;
use support::play::{play_as, run_scripted};

const SCENARIOS: [&str; 6] = [
    "01_transit.toml",
    "02_supply_run.toml",
    "04a_night_action.toml",
    "05_eastern_solomons.toml",
    "06_rabaul_guadalcanal.toml",
    "07_full_slice.toml",
];

fn first_difference(a: &[String], b: &[String]) -> Option<String> {
    (0..a.len().max(b.len()))
        .find(|i| a.get(*i) != b.get(*i))
        .map(|i| {
            format!(
                "event {i}:\n  scripted: {:?}\n  played:   {:?}",
                a.get(i),
                b.get(i)
            )
        })
}

/// The one scripted order of the six scenarios the engine's check refuses when
/// it is asked early: 07_full_slice sends one Japanese order, on day 5, to a
/// force of the other side, to see the engine refuse it (decision 0043). A
/// player is refused it at `issue_orders` as "not known" and it is never
/// queued, so the engine never numbers it and never logs its rejection.
fn expected_refusals(name: &str, players: &[Side]) -> Vec<(u32, Side, usize, String)> {
    if name == "07_full_slice.toml" && players.contains(&Side::Japanese) {
        vec![(5, Side::Japanese, 1, "not known".to_string())]
    } else {
        Vec::new()
    }
}

#[test]
fn a_game_played_through_the_session_is_the_scripted_game() {
    for name in SCENARIOS {
        let (hashes, events) = run_scripted(load(name, 1942));
        for players in [
            vec![Side::Allied],
            vec![Side::Japanese],
            vec![Side::Allied, Side::Japanese],
            vec![Side::Japanese, Side::Allied],
        ] {
            let played = play_as(load(name, 1942), &players);
            println!(
                "{name} {players:?}: {} days, {} events, {} refused",
                played.hashes.len(),
                played.events.len(),
                played.refused.len()
            );
            assert_eq!(
                played.hashes, played.session_hashes,
                "{name}: the session's hashes"
            );
            assert_eq!(played.refused, expected_refusals(name, &players), "{name}");
            if played.refused.is_empty() {
                // Nothing was refused early, so the game is the scripted one:
                // the same hashes and the same log, event for event.
                if let Some(diff) = first_difference(&events, &played.events) {
                    panic!("{name} {players:?}: the log differs\n{diff}");
                }
                assert_eq!(hashes, played.hashes, "{name} {players:?}");
            } else {
                // The scripted game with that one order left out is the
                // played game, event for event.
                let mut scenario = load(name, 1942);
                scenario.replace_script(played_script(&played));
                let (hashes, events) = run_scripted(scenario);
                if let Some(diff) = first_difference(&events, &played.events) {
                    panic!("{name} {players:?}: the log differs from the script of the orders handed in\n{diff}");
                }
                assert_eq!(hashes, played.hashes, "{name} {players:?}");
                // And it is not the scripted game: the refused order is
                // missing from it.
                assert_eq!(events.len() + 1, played_len(name), "{name}");
            }
        }
    }
}

fn played_script(
    played: &support::play::Played,
) -> std::collections::BTreeMap<u32, Vec<(Side, sim_core::orders::OrderRequest)>> {
    played.saved_orders.clone()
}

fn played_len(name: &str) -> usize {
    run_scripted(load(name, 1942)).1.len()
}

// ---------------------------------------------------------------------
// The rest: orders, refusals, seats and time, the record.
// ---------------------------------------------------------------------

use std::collections::BTreeMap;

use serde_json::{json, Value};
use sim_agent::{
    is_orderable_unit, order_from_json, order_to_json, unit_text, CallError, SeatKind, Session,
    NOT_KNOWN,
};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, LandUnitId, ShipId, TaskForceId};
use sim_core::orders::{
    AirGroupOrder, AirMission, Cargo, LandUnitOrder, Mission, OrderKind, OrderRequest, Origin,
    Posture, Reach, Readiness, SpeedSetting, Stance, StragglerInstruction, TaskForceOrder,
};
use sim_core::refdata::ShipKind;
use support::play::{order_json, play_day, served_json, session_of, ByDay};
use support::schemas;

fn call(session: &mut Session, side: Side, tool: &str, args: Value) -> Value {
    session
        .call(side, tool, &args)
        .unwrap_or_else(|e| panic!("{tool} was refused: {e}"))
}

fn refused(session: &mut Session, side: Side, tool: &str, args: Value) -> CallError {
    session
        .call(side, tool, &args)
        .expect_err("the call is refused")
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

// ---- The order schema is complete ----

/// One index for each kind of task force order. The match has no wildcard: a
/// new kind of order breaks this build until it has an example below.
fn tf_index(order: &TaskForceOrder) -> usize {
    match order {
        TaskForceOrder::SetRoute { .. } => 0,
        TaskForceOrder::SetSpeed(_) => 1,
        TaskForceOrder::SetMission(_) => 2,
        TaskForceOrder::Load => 3,
        TaskForceOrder::Unload => 4,
        TaskForceOrder::SetRulesOfEngagement { .. } => 5,
        TaskForceOrder::SetHomePort(_) => 6,
        TaskForceOrder::DetachShips(_) => 7,
        TaskForceOrder::SetReadiness(_) => 8,
        TaskForceOrder::LoadFuel => 9,
        TaskForceOrder::SendToYard => 10,
        TaskForceOrder::LoadUnit { .. } => 11,
        TaskForceOrder::PrepareLanding { .. } => 12,
    }
}
const TF_KINDS: usize = 13;

fn ag_index(order: &AirGroupOrder) -> usize {
    match order {
        AirGroupOrder::SetAirMission(mission) => match mission {
            AirMission::Rest => 0,
            AirMission::Search => 1,
            AirMission::NavalStrike => 2,
            AirMission::CombatAirPatrol => 3,
            AirMission::BaseStrike(_) => 4,
        },
        AirGroupOrder::SetSearchArc { .. } => 5,
        AirGroupOrder::SetPatrolShare { .. } => 6,
        AirGroupOrder::SetRestThreshold { .. } => 7,
        AirGroupOrder::SetReach { .. } => 8,
        AirGroupOrder::SetTargetPriority { .. } => 9,
        AirGroupOrder::Rebase { .. } => 10,
    }
}
const AG_KINDS: usize = 11;

fn examples() -> Vec<OrderKind> {
    let tf = |order| OrderKind::TaskForce {
        target: TaskForceId::new(3),
        order,
    };
    let ag = |order| OrderKind::AirGroup {
        target: AirGroupId::new(12),
        order,
    };
    let mut out = vec![
        tf(TaskForceOrder::route(vec![Hex::new(2, -4), Hex::new(0, 7)])),
        // A marked route (decision 0096): a mark is a hex, and a hex that
        // appears twice on the route is one mark.
        tf(TaskForceOrder::SetRoute {
            waypoints: vec![Hex::new(2, -4), Hex::new(0, 7), Hex::new(2, -4)],
            act_at: [Hex::new(0, 7)].into_iter().collect(),
        }),
        tf(TaskForceOrder::SetSpeed(SpeedSetting::Cruise)),
        tf(TaskForceOrder::SetSpeed(SpeedSetting::Full)),
        tf(TaskForceOrder::SetSpeed(SpeedSetting::Knots(fx("17.5")))),
        tf(TaskForceOrder::Load),
        tf(TaskForceOrder::LoadFuel),
        tf(TaskForceOrder::Unload),
        tf(TaskForceOrder::SetRulesOfEngagement {
            stance: Stance::Avoid,
            break_off: fx("0.25"),
            stragglers: StragglerInstruction::Retire,
            keep_share: fx("0.5"),
        }),
        tf(TaskForceOrder::SetRulesOfEngagement {
            stance: Stance::Engage,
            break_off: fx("0"),
            stragglers: StragglerInstruction::Keep,
            keep_share: fx("1"),
        }),
        tf(TaskForceOrder::SetHomePort(BaseId::new(4))),
        tf(TaskForceOrder::DetachShips(vec![
            ShipId::new(5),
            ShipId::new(6),
        ])),
        tf(TaskForceOrder::SetReadiness(Readiness::Battle)),
        tf(TaskForceOrder::SetReadiness(Readiness::Normal)),
        tf(TaskForceOrder::SendToYard),
        tf(TaskForceOrder::PrepareLanding {
            hex: Hex::new(4, 0),
            roster: [LandUnitId::new(1), LandUnitId::new(3)]
                .into_iter()
                .collect(),
        }),
        tf(TaskForceOrder::PrepareLanding {
            hex: Hex::new(-2, 5),
            roster: std::collections::BTreeSet::new(),
        }),
        tf(TaskForceOrder::LoadUnit {
            unit: LandUnitId::new(2),
            cargo: Some(Cargo::Supply),
        }),
        tf(TaskForceOrder::LoadUnit {
            unit: LandUnitId::new(2),
            cargo: Some(Cargo::Fuel),
        }),
        tf(TaskForceOrder::LoadUnit {
            unit: LandUnitId::new(2),
            cargo: None,
        }),
        ag(AirGroupOrder::SetSearchArc {
            from_deg: 90,
            to_deg: 180,
            range_nm: fx("250"),
        }),
        ag(AirGroupOrder::SetPatrolShare { share: fx("0.75") }),
        ag(AirGroupOrder::SetRestThreshold { fatigue: fx("40") }),
        ag(AirGroupOrder::SetReach {
            reach: Reach::Extended,
        }),
        ag(AirGroupOrder::SetReach {
            reach: Reach::Normal,
        }),
        ag(AirGroupOrder::SetTargetPriority {
            kinds: vec![ShipKind::Transport, ShipKind::Cruiser],
        }),
        ag(AirGroupOrder::SetTargetPriority {
            kinds: vec![ShipKind::Carrier],
        }),
        ag(AirGroupOrder::Rebase {
            base: BaseId::new(3),
        }),
    ];
    for mission in [
        Mission::Transport,
        Mission::SurfaceCombat,
        Mission::Carrier,
        Mission::Bombardment,
        Mission::Escort,
    ] {
        out.push(tf(TaskForceOrder::SetMission(mission)));
    }
    for mission in [
        AirMission::Rest,
        AirMission::Search,
        AirMission::NavalStrike,
        AirMission::CombatAirPatrol,
        AirMission::BaseStrike(BaseId::new(9)),
    ] {
        out.push(ag(AirGroupOrder::SetAirMission(mission)));
    }
    for posture in [Posture::Defend, Posture::Attack] {
        out.push(OrderKind::LandUnit {
            target: sim_core::ids::LandUnitId::new(4),
            order: LandUnitOrder::SetPosture(posture),
        });
    }
    out.push(OrderKind::LandUnit {
        target: sim_core::ids::LandUnitId::new(4),
        order: LandUnitOrder::March(sim_core::hex::Hex::new(3, -2)),
    });
    for activity in [
        sim_core::orders::Activity::Ready,
        sim_core::orders::Activity::Train,
    ] {
        out.push(OrderKind::LandUnit {
            target: sim_core::ids::LandUnitId::new(4),
            order: LandUnitOrder::SetActivity(activity),
        });
    }
    out
}

#[test]
fn every_kind_of_order_the_engine_has_is_in_the_schema_and_round_trips() {
    let schemas = schemas();
    let mut seen_tf = [false; TF_KINDS];
    let mut seen_ag = [false; AG_KINDS];
    let mut seen_posture = false;
    let mut seen_march = false;
    let mut seen_activity = false;
    for kind in examples() {
        match &kind {
            OrderKind::TaskForce { order, .. } => seen_tf[tf_index(order)] = true,
            OrderKind::AirGroup { order, .. } => seen_ag[ag_index(order)] = true,
            OrderKind::LandUnit { order, .. } => match order {
                LandUnitOrder::SetPosture(_) => seen_posture = true,
                LandUnitOrder::March(_) => seen_march = true,
                LandUnitOrder::SetActivity(_) => seen_activity = true,
            },
        }
        let request = OrderRequest {
            kind: kind.clone(),
            origin: Origin::PLAYER,
        };
        let doc = order_json(&request);
        let errors = schemas.check("order.schema.json", &doc);
        assert!(errors.is_empty(), "{doc}: {errors:?}");
        let unit = unit_text(&kind);
        assert!(is_orderable_unit(&unit));
        let back = order_from_json(&unit, &doc["order"]).expect("the order decodes");
        assert_eq!(back, kind, "{doc}");
        assert_eq!(order_to_json(&back), doc["order"], "{doc}");
    }
    assert!(seen_tf.iter().all(|s| *s), "task force kinds: {seen_tf:?}");
    assert!(seen_ag.iter().all(|s| *s), "air group kinds: {seen_ag:?}");
    assert!(seen_posture, "the land unit order is in the examples");
    assert!(seen_march, "the march order is in the examples");
    assert!(seen_activity, "the activity order is in the examples");
    // Every scripted order of every scenario is in the schema too.
    for name in [
        "05_eastern_solomons.toml",
        "07_full_slice.toml",
        "06_rabaul_guadalcanal.toml",
    ] {
        let scenario = load(name, 1);
        for list in scenario.scripted_orders().values() {
            for (_, request) in list {
                let doc = order_json(request);
                let errors = schemas.check("order.schema.json", &doc);
                assert!(errors.is_empty(), "{name} {doc}: {errors:?}");
            }
        }
    }
}

#[test]
fn a_marked_route_writes_act_at_and_an_unmarked_one_writes_none() {
    let schemas = schemas();
    let unit = "task_force:3";
    let kind = |order| OrderKind::TaskForce {
        target: TaskForceId::new(3),
        order,
    };
    // No mark: the key is not there, so the order is written as it always was.
    let plain = kind(TaskForceOrder::route(vec![Hex::new(1, 0), Hex::new(3, 0)]));
    let body = order_to_json(&plain);
    assert_eq!(
        body,
        json!({"kind": "set_route", "waypoints": [[1, 0], [3, 0]]})
    );
    assert!(body.get("act_at").is_none(), "{body}");
    // A mark: written in hex order, read back to the same order.
    let marked = kind(TaskForceOrder::SetRoute {
        waypoints: vec![Hex::new(1, 0), Hex::new(3, 0), Hex::new(1, 0)],
        act_at: [Hex::new(3, 0), Hex::new(1, 0)].into_iter().collect(),
    });
    let body = order_to_json(&marked);
    assert_eq!(body["act_at"], json!([[1, 0], [3, 0]]), "{body}");
    let doc = json!({"unit": unit, "order": body});
    assert!(schemas.check("order.schema.json", &doc).is_empty(), "{doc}");
    assert_eq!(
        order_from_json(unit, &doc["order"]).expect("decodes"),
        marked
    );
    // act_at that is not a list of hexes is an error, and null is no mark.
    let bad = json!({"kind": "set_route", "waypoints": [[1, 0]], "act_at": 3});
    assert!(order_from_json(unit, &bad).is_err());
    let null = json!({"kind": "set_route", "waypoints": [[1, 0]], "act_at": null});
    assert_eq!(
        order_from_json(unit, &null).expect("null is no mark"),
        kind(TaskForceOrder::route(vec![Hex::new(1, 0)]))
    );
    // The schema refuses a mark that is not a hex pair of integers.
    let doc = json!({"unit": unit, "order": {"kind": "set_route", "waypoints": [[1, 0]], "act_at": [["x"]]}});
    assert!(!schemas.check("order.schema.json", &doc).is_empty());
}

#[test]
fn the_order_schema_refuses_what_is_not_an_order() {
    let schemas = schemas();
    for doc in [
        json!({"unit": "task_force:1", "order": {"kind": "set_course", "waypoints": []}}),
        json!({"unit": "task_force:1", "order": {"kind": "set_speed", "speed": "flank"}}),
        json!({"unit": "task_force:1", "order": {"kind": "set_speed", "knots": 17}}),
        json!({"unit": "task_force:1", "order": {"kind": "unload", "extra": 1}}),
        json!({"unit": "air_group:1", "order": {"kind": "set_air_mission", "mission": "base_strike"}}),
        json!({"unit": "air_group:1", "order": {"kind": "set_air_mission", "mission": "rest", "target_base": 1}}),
        json!({"order": {"kind": "unload"}}),
    ] {
        assert!(
            !schemas.check("order.schema.json", &doc).is_empty(),
            "{doc}"
        );
    }
}

// ---- Refusals ----

fn eastern_solomons() -> (Session, BTreeMap<Side, ByDay>, Units) {
    let scenario = load("05_eastern_solomons.toml", 1942);
    let truth = scenario.world.umpire();
    let first = |side: Side| {
        (
            truth
                .task_forces
                .values()
                .find(|f| f.side == side)
                .expect("a force")
                .id
                .raw(),
            truth
                .air_groups
                .values()
                .find(|g| g.side == side)
                .expect("a group")
                .id
                .raw(),
            truth
                .ships
                .values()
                .find(|s| s.side == side)
                .expect("a ship")
                .id
                .raw(),
        )
    };
    // A side has no handle for an enemy unit (decision 0086): the numbers
    // just above its own count of each kind stand for them.
    let view = scenario.world.side_view(Side::Allied);
    let counts = |n: usize| u32::try_from(n + 1).expect("few units");
    let units = Units {
        own: first(Side::Allied),
        enemy: (
            counts(view.task_forces().count()),
            counts(view.air_groups().count()),
            counts(view.ships().count()),
        ),
    };
    let (session, scripts) = session_of(scenario, &[Side::Allied]);
    (session, scripts, units)
}

/// (task force, air group, ship) numbers.
struct Units {
    own: (u32, u32, u32),
    enemy: (u32, u32, u32),
}

fn one_order(session: &mut Session, side: Side, unit: &str, body: Value) -> Value {
    call(
        session,
        side,
        "issue_orders",
        json!({ "orders": [{"unit": unit, "order": body}], "clear": true }),
    )
}

#[test]
fn an_order_for_an_enemy_unit_or_no_unit_is_refused_in_one_way() {
    let (mut session, _, units) = eastern_solomons();
    let tf = json!({"kind": "set_speed", "speed": "cruise"});
    let ag = json!({"kind": "set_air_mission", "mission": "rest"});
    let cases = [
        (format!("task_force:{}", units.enemy.0), tf.clone()),
        (format!("air_group:{}", units.enemy.1), ag.clone()),
        ("task_force:99999".to_string(), tf.clone()),
        ("air_group:99999".to_string(), ag.clone()),
        ("task_force:0".to_string(), tf.clone()),
        ("air_group:0".to_string(), ag),
        (format!("ship:{}", units.enemy.2), tf.clone()),
        (format!("ship:{}", units.own.2), tf.clone()),
        ("base:1".to_string(), tf.clone()),
        ("banana".to_string(), tf.clone()),
        (String::new(), tf),
    ];
    let mut answers = Vec::new();
    for (unit, body) in &cases {
        let answer = one_order(&mut session, Side::Allied, unit, body.clone());
        assert_eq!(
            answer["results"],
            json!([{"index": 0, "status": "refused", "reason": NOT_KNOWN}]),
            "{unit}"
        );
        answers.push(answer.to_string());
    }
    assert!(
        answers.windows(2).all(|w| w[0] == w[1]),
        "the refusals differ: {answers:?}"
    );
    // The answer says nothing of the question.
    assert!(!answers[0].contains("99999"));
    // Nothing was queued, and an own unit is accepted.
    assert_eq!(
        call(
            &mut session,
            Side::Allied,
            "issue_orders",
            json!({"orders": []})
        )["queued"],
        0
    );
    let own = one_order(
        &mut session,
        Side::Allied,
        &format!("task_force:{}", units.own.0),
        json!({"kind": "set_speed", "speed": "cruise"}),
    );
    assert_eq!(own["results"], json!([{"index": 0, "status": "queued"}]));
}

#[test]
fn a_route_through_land_or_to_nowhere_gets_the_engines_own_reason() {
    let scenario = load("05_eastern_solomons.toml", 1942);
    let map = scenario.world.map();
    let land = map
        .iter()
        .map(|(hex, _)| *hex)
        .find(|hex| !map.is_navigable(*hex))
        .expect("the map has land");
    let truth = scenario.world.umpire();
    let force = truth
        .task_forces
        .values()
        .find(|f| f.side == Side::Allied)
        .expect("a force")
        .id
        .raw();
    let world_for_check = scenario.world.clone();
    let (mut session, _) = session_of(scenario, &[Side::Allied]);
    let unit = format!("task_force:{force}");
    for waypoints in [
        json!([[land.q, land.r]]),
        json!([]),
        json!([[100_000, 100_000]]),
    ] {
        let body = json!({"kind": "set_route", "waypoints": waypoints});
        let answer = one_order(&mut session, Side::Allied, &unit, body.clone());
        let request = OrderRequest {
            kind: order_from_json(&unit, &body).expect("decodes"),
            origin: Origin::PLAYER,
        };
        let engine = world_for_check
            .check_order(Side::Allied, &request)
            .expect_err("the engine refuses it")
            .to_string();
        // The engine's own words come first; a hint of what the order needs
        // may follow in brackets (wording pass of Nov 12).
        let shown = answer["results"][0]["reason"].as_str().expect("a reason");
        assert_eq!(answer["results"][0]["status"], "refused", "{waypoints}");
        assert!(shown.starts_with(&engine), "{waypoints}: {shown}");
        assert!(
            shown == engine || shown[engine.len()..].starts_with(" ("),
            "{shown}"
        );
        assert_ne!(engine, NOT_KNOWN);
    }
    // The reasons are the engine's words.
    let reasons: Vec<String> = [json!([]), json!([[100_000, 100_000]])]
        .iter()
        .map(|w| {
            one_order(
                &mut session,
                Side::Allied,
                &unit,
                json!({"kind": "set_route", "waypoints": w}),
            )["results"][0]["reason"]
                .as_str()
                .unwrap()
                .to_string()
        })
        .collect();
    assert_eq!(reasons[0], "the route goes nowhere");
    assert!(reasons[1].contains("off the map"), "{}", reasons[1]);
}

#[test]
fn a_malformed_order_is_refused_and_the_others_in_the_call_stand() {
    let (mut session, _, units) = eastern_solomons();
    let unit = format!("task_force:{}", units.own.0);
    let answer = call(
        &mut session,
        Side::Allied,
        "issue_orders",
        json!({"orders": [
            {"unit": unit, "order": {"kind": "set_speed", "speed": "cruise"}},
            {"unit": unit, "order": {"kind": "set_speed", "speed": "warp"}},
            {"unit": unit},
            {"order": {"kind": "unload"}},
            "not an order",
            {"unit": unit, "order": {"kind": "set_readiness", "readiness": "battle"}},
        ]}),
    );
    let statuses: Vec<&str> = answer["results"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["status"].as_str().unwrap())
        .collect();
    assert_eq!(
        statuses,
        ["queued", "refused", "refused", "refused", "refused", "queued"]
    );
    for i in 1..=4 {
        let reason = answer["results"][i]["reason"].as_str().unwrap();
        assert!(reason.starts_with("malformed order"), "{reason}");
    }
    assert_eq!(answer["queued"], 2);
}

/// The engine's reject reasons and what a player is shown, checked against the
/// engine's own text, one by one.
#[test]
fn every_reject_reason_is_shown_as_it_is_except_the_one_that_is_folded() {
    let scenario = load("07_full_slice.toml", 1942);
    let truth = scenario.world.umpire();
    let allied_force = truth
        .task_forces
        .values()
        .find(|f| f.side == Side::Allied)
        .expect("tf");
    let force = allied_force.id.raw();
    let other_ship = truth
        .ships
        .values()
        .find(|s| s.side == Side::Japanese)
        .expect("ship")
        .id
        .raw();
    let (mut session, _) = session_of(scenario, &[Side::Allied]);
    let unit = format!("task_force:{force}");
    // Fixed reasons that need nothing but the order.
    let reasons: Vec<(Value, &str)> = vec![
        (
            json!({"kind": "set_route", "waypoints": []}),
            "the route goes nowhere",
        ),
        (json!({"kind": "set_speed", "knots": "0"}), "is not above 0"),
        (
            json!({"kind": "set_rules_of_engagement", "stance": "engage", "break_off": "2", "stragglers": "keep", "keep_share": "1"}),
            "break-off level",
        ),
        (
            json!({"kind": "set_rules_of_engagement", "stance": "engage", "break_off": "0", "stragglers": "keep", "keep_share": "2"}),
            "share to keep",
        ),
        (
            json!({"kind": "set_home_port", "base": 99999}),
            "the base is not one of your bases",
        ),
        (
            json!({"kind": "detach", "ships": []}),
            "no ships are listed to detach",
        ),
        (
            json!({"kind": "detach", "ships": [other_ship]}),
            "is not afloat in the task force",
        ),
    ];
    for (body, expect) in reasons {
        let answer = one_order(&mut session, Side::Allied, &unit, body.clone());
        let reason = answer["results"][0]["reason"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        assert!(reason.contains(expect), "{body}: {reason}");
    }
    // The air group orders.
    let group = session_group(&mut session);
    for (body, expect) in [
        (
            json!({"kind": "set_search_arc", "from_deg": 5, "to_deg": 90, "range_nm": "100"}),
            "is not a multiple of 10",
        ),
        (
            json!({"kind": "set_search_arc", "from_deg": 0, "to_deg": 90, "range_nm": "0"}),
            "search range",
        ),
        (
            json!({"kind": "set_patrol_share", "share": "2"}),
            "patrol share",
        ),
        (
            json!({"kind": "set_rest_threshold", "fatigue": "101"}),
            "rest threshold",
        ),
        (
            json!({"kind": "set_air_mission", "mission": "base_strike", "target_base": 99999}),
            "does not exist or is held by your side",
        ),
    ] {
        let answer = one_order(&mut session, Side::Allied, &group, body.clone());
        let reason = answer["results"][0]["reason"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        assert!(reason.contains(expect), "{body}: {reason}");
    }
}

fn session_group(session: &mut Session) -> String {
    let briefing = call(session, Side::Allied, "get_briefing", json!({}));
    let handles: Vec<String> = briefing["air_groups"]
        .as_array()
        .expect("groups")
        .iter()
        .map(|g| {
            g["air_group"]["handle"]
                .as_str()
                .expect("handle")
                .to_string()
        })
        .collect();
    // A group whose aircraft can strike.
    handles
        .into_iter()
        .find(|h| {
            let body = json!({"kind": "set_air_mission", "mission": "naval_strike"});
            one_order(session, Side::Allied, h, body)["results"][0]["status"] == "queued"
        })
        .expect("a group that can strike")
}

// ---- Seats and time ----

fn world_hash(session: &Session) -> (u64, u32, usize) {
    (
        session.state_hash(),
        session.day(),
        session.handed_in().len(),
    )
}

#[test]
fn a_script_seat_cannot_be_called_and_nothing_changes() {
    let (mut session, _, _) = eastern_solomons();
    let before = world_hash(&session);
    let error = refused(&mut session, Side::Japanese, "get_briefing", json!({}));
    assert!(error.message.contains("script"), "{error}");
    assert_eq!(error.allowed, ["allied"]);
    assert!(session.transcript().is_empty(), "not a call of a player");
    assert_eq!(world_hash(&session), before);
    assert_eq!(session.seat_kind(Side::Japanese), SeatKind::Script);
}

#[test]
fn an_unknown_tool_and_bad_arguments_change_nothing_and_name_what_is_allowed() {
    let (mut session, _, _) = eastern_solomons();
    let before = world_hash(&session);
    let e = refused(&mut session, Side::Allied, "get_truth", json!({}));
    assert_eq!(e.allowed.len(), 9);
    assert!(e.allowed.contains(&"issue_orders".to_string()));
    for (tool, args) in [
        ("get_entity", json!({})),
        ("get_entity", json!({"handle": 3})),
        ("explain", json!({"event": "x"})),
        ("measure", json!({"question": "teleport"})),
        ("list_forces", json!({"area": {"q": 1}})),
        ("list_forces", json!([1, 2])),
        ("issue_orders", json!({})),
        ("issue_orders", json!({"orders": "x"})),
        ("issue_orders", json!({"orders": [], "clear": "yes"})),
        ("get_briefing", json!({"text": 3})),
    ] {
        refused(&mut session, Side::Allied, tool, args);
    }
    assert_eq!(world_hash(&session), before);
    // A refused call is something the player was told, so it is in the
    // transcript, with its error as the answer.
    assert_eq!(session.transcript().len(), 11);
    assert!(session
        .transcript()
        .iter()
        .all(|t| t.answer["kind"] == "error"));
    // A bad `issue_orders` did not even clear the queue.
    let queue = call(
        &mut session,
        Side::Allied,
        "issue_orders",
        json!({"orders": []}),
    );
    assert_eq!(queue["queued"], 0);
}

/// Each unit's orders together, in their order, the units in reverse.
fn reversed_by_unit(orders: &[OrderRequest]) -> Vec<OrderRequest> {
    let mut groups: Vec<(sim_core::ids::EntityRef, Vec<OrderRequest>)> = Vec::new();
    for order in orders {
        let target = order.kind.target();
        match groups.iter_mut().find(|(t, _)| *t == target) {
            Some((_, list)) => list.push(order.clone()),
            None => groups.push((target, vec![order.clone()])),
        }
    }
    groups.reverse();
    groups.into_iter().flat_map(|(_, list)| list).collect()
}

#[test]
fn with_two_players_the_day_waits_for_both_and_the_order_does_not_matter() {
    let name = "05_eastern_solomons.toml";
    let (hashes, events) = run_scripted(load(name, 7));
    // 0: allied first, 1: japanese first, 2: interleaved with the orders of
    // each given in reverse unit order and the day ended by the other first.
    for variant in 0..3 {
        let (mut session, scripts) = session_of(load(name, 7), &[Side::Allied, Side::Japanese]);
        let allied = scripts[&Side::Allied].get(&1).cloned().unwrap_or_default();
        let japanese = scripts[&Side::Japanese]
            .get(&1)
            .cloned()
            .unwrap_or_default();
        assert!(!allied.is_empty() && !japanese.is_empty());
        let end = |s: &mut Session, side| call(s, side, "end_day", json!({}));
        let issue = |s: &mut Session, side, orders: &[OrderRequest]| {
            let list: Vec<Value> = orders.iter().map(|o| served_json(side, o)).collect();
            call(s, side, "issue_orders", json!({"orders": list}))
        };
        let last = match variant {
            0 => {
                issue(&mut session, Side::Allied, &allied);
                let first = end(&mut session, Side::Allied);
                assert_eq!(first["resolved"], false);
                assert_eq!(first["day"], 0);
                // Reads still work, orders do not.
                call(&mut session, Side::Allied, "get_briefing", json!({}));
                let closed = refused(
                    &mut session,
                    Side::Allied,
                    "issue_orders",
                    json!({"orders": []}),
                );
                assert_eq!(closed.message, "the day's orders are closed");
                issue(&mut session, Side::Japanese, &japanese);
                end(&mut session, Side::Japanese)
            }
            1 => {
                issue(&mut session, Side::Japanese, &japanese);
                assert_eq!(end(&mut session, Side::Japanese)["resolved"], false);
                issue(&mut session, Side::Allied, &allied);
                end(&mut session, Side::Allied)
            }
            _ => {
                issue(&mut session, Side::Japanese, &reversed_by_unit(&japanese));
                issue(&mut session, Side::Allied, &reversed_by_unit(&allied));
                assert_eq!(end(&mut session, Side::Allied)["resolved"], false);
                // Ending twice is the same as once.
                assert_eq!(end(&mut session, Side::Allied)["resolved"], false);
                end(&mut session, Side::Japanese)
            }
        };
        // The answer that reports the game over carries the result of the
        // scenario's objectives (decision 0054); the rest is as it was.
        let mut last = last;
        let result = last
            .as_object_mut()
            .expect("an object")
            .remove("result")
            .expect("the answer has a result");
        assert!(result["allied"]["margin"].is_string(), "{result}");
        assert_eq!(
            last,
            json!({"format": "flagplot-agent", "version": 3, "kind": "end_day", "resolved": true, "day": 1, "game_over": true})
        );
        assert_eq!(session.day_hashes(), hashes, "variant {variant}");
        let world = session.into_world();
        let played: Vec<String> = world.events().iter().map(|e| format!("{e:?}")).collect();
        assert_eq!(
            first_difference(&events, &played),
            None,
            "variant {variant}"
        );
    }
}

#[test]
fn clear_withdraws_what_is_queued_and_orders_after_end_day_are_refused() {
    let (mut session, scripts, _) = eastern_solomons();
    let orders: Vec<Value> = scripts[&Side::Allied][&1]
        .iter()
        .map(|o| served_json(Side::Allied, o))
        .collect();
    let n = orders.len();
    let first = call(
        &mut session,
        Side::Allied,
        "issue_orders",
        json!({"orders": orders}),
    );
    assert_eq!(first["queued"], n);
    let again = call(
        &mut session,
        Side::Allied,
        "issue_orders",
        json!({"orders": orders}),
    );
    assert_eq!(again["queued"], 2 * n, "orders add to the queue");
    let cleared = call(
        &mut session,
        Side::Allied,
        "issue_orders",
        json!({"orders": [], "clear": true}),
    );
    assert_eq!(cleared["queued"], 0);
    assert_eq!(cleared["cleared"], true);
    let end = call(&mut session, Side::Allied, "end_day", json!({}));
    assert_eq!(end["resolved"], true);
    assert!(
        session.handed_in()[&1]
            .iter()
            .all(|(side, _)| *side == Side::Japanese),
        "the allied seat handed in nothing"
    );
}

#[test]
fn the_last_day_ends_the_game_and_nothing_more_changes_it() {
    let scenario = load("01_transit.toml", 1);
    let (mut session, scripts) = session_of(scenario, &[Side::Allied]);
    for day in 1..=3 {
        let orders = scripts[&Side::Allied]
            .get(&day)
            .cloned()
            .unwrap_or_default();
        let (_, ended) = play_day(&mut session, Side::Allied, &orders);
        assert_eq!(ended["resolved"], true);
        assert_eq!(ended["day"], day);
        assert_eq!(ended["game_over"], day == 3);
    }
    assert!(session.is_over());
    let before = world_hash(&session);
    let over = call(&mut session, Side::Allied, "end_day", json!({}));
    assert_eq!(over["resolved"], false);
    assert_eq!(over["game_over"], true);
    assert_eq!(over["day"], 3);
    let e = refused(
        &mut session,
        Side::Allied,
        "issue_orders",
        json!({"orders": []}),
    );
    assert_eq!(e.message, "the game is over");
    // Reading still works.
    let b = call(&mut session, Side::Allied, "get_briefing", json!({}));
    assert_eq!(b["heading"]["day"], 4);
    assert_eq!(world_hash(&session), before);
}

// ---- A queued order can still be refused at intake (decision 4) ----

#[test]
fn an_order_acceptable_now_can_be_refused_when_the_day_begins() {
    let (mut session, _) = session_of(load("02_supply_run.toml", 1), &[Side::Allied]);
    let unit = "task_force:1";
    // At its base and stopped, the convoy may be told to load and to sail.
    // Each is acceptable now; but the engine takes the route first and the
    // load then finds the force under way.
    let answer = call(
        &mut session,
        Side::Allied,
        "issue_orders",
        json!({"orders": [
            {"unit": unit, "order": {"kind": "set_route", "waypoints": [[8, 0]]}},
            {"unit": unit, "order": {"kind": "load"}},
        ]}),
    );
    assert_eq!(
        answer["results"],
        json!([{"index": 0, "status": "queued"}, {"index": 1, "status": "queued"}])
    );
    call(&mut session, Side::Allied, "end_day", json!({}));
    let briefing = call(&mut session, Side::Allied, "get_briefing", json!({}));
    let refused = &briefing["orders"]["refused"];
    assert_eq!(refused.as_array().unwrap().len(), 1, "{refused}");
    let reason = refused[0]["reason"].as_str().expect("a reason");
    assert!(
        reason.starts_with("the task force is under way"),
        "{reason}"
    );
    assert!(
        reason.contains("load (of supply or fuel) and send_to_yard need the task force stopped at a base of your own side"),
        "{reason}"
    );
    assert_eq!(briefing["orders"]["accepted"], 1);
    // `explain` shows the same refusal in the same words.
    let event = refused[0]["event"].as_u64().expect("an event number");
    let explained = call(
        &mut session,
        Side::Allied,
        "explain",
        json!({ "event": event }),
    );
    let text = explained["nodes"][0]["text"].as_str().expect("text");
    assert!(
        text.contains(": the task force is under way (load (of supply or fuel) and send_to_yard need the task force stopped at a base of your own side"),
        "{text}"
    );
}

// ---- What the engine's verdict can tell a side ----

#[test]
fn a_base_strike_verdict_depends_only_on_the_bases_the_side_can_list() {
    // A base strike is accepted only on a base that exists and is the enemy's.
    // The bases of the other side are on the map (decision 0034, 2) and
    // `list_bases` and `get_entity` give them, so the verdict tells the side
    // nothing it cannot list: the numbers accepted are exactly the enemy bases
    // listed, and the same ones `get_entity` knows.
    let scenario = load("06_rabaul_guadalcanal.toml", 1);
    let (mut session, _) = session_of(scenario, &[Side::Japanese]);
    let listed: Vec<u32> = call(&mut session, Side::Japanese, "list_bases", json!({}))
        ["enemy_bases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|b| {
            b["base"]["handle"]
                .as_str()
                .unwrap()
                .strip_prefix("base:")
                .unwrap()
                .parse()
                .unwrap()
        })
        .collect();
    assert!(!listed.is_empty());
    let group = {
        let b = call(&mut session, Side::Japanese, "get_briefing", json!({}));
        let groups = b["air_groups"].as_array().unwrap();
        groups
            .iter()
            .map(|g| g["air_group"]["handle"].as_str().unwrap().to_string())
            .find(|handle| {
                let a = one_order(
                    &mut session,
                    Side::Japanese,
                    handle,
                    json!({"kind": "set_air_mission", "mission": "naval_strike"}),
                );
                a["results"][0]["status"] == "queued"
            })
            .expect("a group that can strike")
    };
    let mut queued = Vec::new();
    let mut known = Vec::new();
    for n in 1..=60u32 {
        let a = one_order(
            &mut session,
            Side::Japanese,
            &group,
            json!({"kind": "set_air_mission", "mission": "base_strike", "target_base": n}),
        );
        if a["results"][0]["status"] == "queued" {
            queued.push(n);
        }
        let e = call(
            &mut session,
            Side::Japanese,
            "get_entity",
            json!({"handle": format!("base:{n}")}),
        );
        if e["kind"] == "entity" && e["entity"]["entity_kind"] == "enemy_base" {
            known.push(n);
        }
    }
    assert_eq!(queued, listed);
    assert_eq!(known, listed);
}

#[test]
fn a_route_into_a_hex_with_an_unseen_enemy_is_judged_by_the_map_alone() {
    // Two worlds that differ only in where the enemy's second carrier force
    // sits: in the hex the side is ordered to, or one hex away. The verdict of
    // the route order is the same in both, and so is every verdict of the
    // battery.
    let name = "05_eastern_solomons.toml";
    let text = support::scenario_text(name);
    let target = Hex::new(7, -18);
    let beside = (target.q + 1, target.r);
    let moved = text.replacen(
        "name = \"Ryujo detachment\"\nside = \"japanese\"\nq = 7\nr = -18",
        &format!(
            "name = \"Ryujo detachment\"\nside = \"japanese\"\nq = {}\nr = {}",
            beside.0, beside.1
        ),
        1,
    );
    assert_ne!(text, moved, "the edit applied");
    let mut answers = Vec::new();
    for text in [&text, &moved] {
        let scenario = support::load_text(text, name, 1942);
        let (mut session, _) = session_of(scenario, &[Side::Allied]);
        let order = json!({"kind": "set_route", "waypoints": [[target.q, target.r]]});
        let a = one_order(&mut session, Side::Allied, "task_force:1", order);
        assert_eq!(a["results"][0]["status"], "queued", "{a}");
        answers.push(a.to_string());
    }
    assert_eq!(answers[0], answers[1]);
}

// ---- The record ----

#[test]
fn every_answer_and_the_transcript_are_valid_against_their_schemas() {
    let schemas = schemas();
    let (mut session, scripts, _) = eastern_solomons();
    let orders: Vec<Value> = scripts[&Side::Allied][&1]
        .iter()
        .map(|o| served_json(Side::Allied, o))
        .collect();
    let args = json!({"orders": orders, "clear": true});
    assert!(schemas
        .check("issue_orders_arguments.schema.json", &args)
        .is_empty());
    let answer = call(&mut session, Side::Allied, "issue_orders", args);
    assert!(
        schemas
            .check("issue_orders.schema.json", &answer)
            .is_empty(),
        "{answer}"
    );
    let text = call(
        &mut session,
        Side::Allied,
        "get_briefing",
        json!({"text": true}),
    );
    assert!(schemas.check("briefing_text.schema.json", &text).is_empty());
    let bad = session
        .call(Side::Allied, "nope", &json!({}))
        .expect_err("refused");
    assert!(schemas
        .check("error.schema.json", &bad.to_json())
        .is_empty());
    let end = call(&mut session, Side::Allied, "end_day", json!({}));
    assert!(
        schemas.check("end_day.schema.json", &end).is_empty(),
        "{end}"
    );
    call(
        &mut session,
        Side::Allied,
        "list_forces",
        json!({"area": {"q": 7, "r": -5, "radius_nm": 100}}),
    );
    call(&mut session, Side::Allied, "list_bases", json!({}));
    call(&mut session, Side::Allied, "list_contacts", json!({}));
    call(
        &mut session,
        Side::Allied,
        "get_entity",
        json!({"handle": "task_force:1"}),
    );
    call(
        &mut session,
        Side::Allied,
        "get_entity",
        json!({"handle": "task_force:3"}),
    );
    call(&mut session, Side::Allied, "explain", json!({"event": 1}));
    call(
        &mut session,
        Side::Allied,
        "measure",
        json!({"question": "air_distance", "from": "task_force:1", "to": {"q": 0, "r": 0}}),
    );
    let entries = session.transcript();
    assert_eq!(entries.len(), 11);
    for (i, entry) in entries.iter().enumerate() {
        let doc = entry.to_json();
        let errors = schemas.check("transcript_entry.schema.json", &doc);
        assert!(errors.is_empty(), "{doc}: {errors:?}");
        assert_eq!(entry.seq as usize, i + 1);
        // The tool's own schema fits the answer it was given.
        let file = match (entry.tool.as_str(), entry.answer["kind"].as_str().unwrap()) {
            (_, "error") => "error.schema.json".to_string(),
            (t, "not_known") => {
                let _ = t;
                "not_known.schema.json".to_string()
            }
            (_, kind) => format!("{kind}.schema.json"),
        };
        let errors = schemas.check(&file, &entry.answer);
        assert!(errors.is_empty(), "{file}: {errors:?}");
        // The scenario's history note says "several times the American": a
        // multiple, not a time of day.
        assert!(
            !doc.to_string()
                .replace("several times", "")
                .contains("time"),
            "no time of day"
        );
    }
    assert_eq!(entries[0].after_day, 0);
    assert_eq!(entries[1].tool, "get_briefing");
    assert_eq!(entries[3].after_day, 0, "the day was not yet resolved");
    assert_eq!(entries[4].after_day, 1, "calls after the day resolved");
}

#[test]
fn the_tools_give_what_the_functions_give() {
    use sim_agent::{briefing_for_game, get_entity, list_bases, list_contacts, list_forces};
    let (mut session, _, _) = eastern_solomons();
    let world = load("05_eastern_solomons.toml", 1942).world;
    let view = world.side_view(Side::Allied);
    let same = |a: Value, b: &dyn erased::Ser| {
        assert_eq!(a, serde_json::from_str::<Value>(&b.to_json()).unwrap())
    };
    same(
        call(&mut session, Side::Allied, "get_briefing", json!({})),
        &briefing_for_game(
            &view,
            "Scenario 5: Eastern Solomons, 24 August 1942",
            Some(1),
        ),
    );
    same(
        call(&mut session, Side::Allied, "list_forces", json!({})),
        &list_forces(&view, None),
    );
    same(
        call(&mut session, Side::Allied, "list_bases", json!({})),
        &list_bases(&view, None),
    );
    same(
        call(&mut session, Side::Allied, "list_contacts", json!({})),
        &list_contacts(&view, None),
    );
    same(
        call(
            &mut session,
            Side::Allied,
            "get_entity",
            json!({"handle": "task_force:1"}),
        ),
        &get_entity(&view, "task_force:1"),
    );
}

mod erased {
    pub trait Ser {
        fn to_json(&self) -> String;
    }
    impl<T: serde::Serialize> Ser for T {
        fn to_json(&self) -> String {
            serde_json::to_string(self).expect("serialises")
        }
    }
}
