//! Preparing a landing and the experience of a land unit as the agent
//! interface tells them (decisions 0095 and 0097).
//!
//! Scenario 27 is read on the live figures (`data/reference/ground.toml`),
//! not the frozen ones, which switch the rule off: `preparation_per_day` 5,
//! `unprepared_landing_share` 0.5, `unprepared_landing_fatigue` 40. Five
//! Allied forces lie at base A (0,0); the side's `task_force:N` carries
//! `land_unit:N` aboard for N from 1 to 5, and `land_unit:6` is ashore at A.
//! B (4,0) and C (0,6) are coast hexes; (2,0) is open sea. None of the
//! scenario's own scripted orders is carried into a `Session`, so the orders
//! here are the player's.
//!
//! After one day with `prepare_landing` for B, force 1 has loading 5, so it
//! lands at 0.5 + 0.5 x 5/100 = 0.525 of the port's rate at B, and its unit
//! has rehearsal 5, so it would land with a fatigue of 40 x 95/100 = 38.
//!
//! Scenario 28: the Allied units are `land_unit:1` (id 1, experience 0, at
//! base 1), `land_unit:2` (id 2, experience 2 with 9 days of fighting, defends
//! at F against the Japanese attack), `land_unit:3` (id 4) and `land_unit:4`
//! (id 6, experience 1 at a base with 1 t). Unit 1 set to train for a day
//! counts day 1 of 30; unit 2 fires on day 1, counts its tenth day and gains
//! a step to experience 3.

mod support;

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::state::Side;
use sim_core::World;
use sim_data::{LoadOptions, Scenario};
use support::check::{check_view, valid};
use support::{scenario_path, scenario_text, schemas};

/// A scenario on the live figures, with its text changed by `edit`.
fn live(name: &str, edit: impl Fn(String) -> String) -> Scenario {
    Scenario::from_text_with(
        &edit(scenario_text(name)),
        &scenario_path(name),
        &LoadOptions::default(),
    )
    .expect("the scenario loads on the live figures")
    .with_seed(1942)
    .expect("a seed can be set")
}

fn session_of(scenario: Scenario) -> Session {
    Session::new(
        scenario.world,
        "Preparing",
        scenario.days,
        Seat::Player,
        Seat::Player,
    )
}

fn orders(session: &mut Session, side: Side, list: &[Value]) -> Vec<Value> {
    let answer = session
        .call(side, "issue_orders", &json!({ "orders": list }))
        .expect("issue_orders is answered");
    valid(&schemas(), "issue_orders.schema.json", &answer, "orders");
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
    valid(&schemas(), "briefing_text.schema.json", &text, "text");
    let data = session
        .call(side, "get_briefing", &json!({ "text": false }))
        .expect("the briefing");
    valid(&schemas(), "briefing.schema.json", &data, "briefing");
    (text["text"].as_str().expect("text").to_string(), data)
}

fn prepare(unit: &str, q: i32, r: i32, roster: &[&str]) -> Value {
    json!({ "unit": unit, "order": { "kind": "prepare_landing", "hex": [q, r], "roster": roster } })
}

fn activity(unit: &str, what: &str) -> Value {
    json!({ "unit": unit, "order": { "kind": "set_activity", "activity": what } })
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

fn force_line<'a>(data: &'a Value, handle: &str) -> &'a Value {
    by_handle(&data["task_forces"], "task_force", handle)
}

fn status(result: &Value) -> &str {
    result["status"].as_str().expect("a status")
}

#[test]
fn the_two_orders_are_queued_and_refused_with_their_texts() {
    let mut session = session_of(live("27_preparing_a_landing.toml", |t| t));
    let results = orders(
        &mut session,
        Side::Allied,
        &[
            prepare("task_force:1", 4, 0, &["land_unit:1"]),
            // An empty roster is accepted; so is a second coast hex.
            prepare("task_force:2", 0, 6, &[]),
            // Open sea is no coast hex.
            prepare("task_force:3", 2, 0, &["land_unit:3"]),
            // A unit that is nothing, in a roster.
            prepare("task_force:4", 4, 0, &["land_unit:99"]),
            // A roster that is not one.
            json!({"unit": "task_force:5", "order": {"kind": "prepare_landing", "hex": [4, 0], "roster": ["ship:1"]}}),
            json!({"unit": "task_force:5", "order": {"kind": "prepare_landing", "hex": [4], "roster": []}}),
            activity("land_unit:1", "train"),
            activity("land_unit:6", "ready"),
            activity("land_unit:99", "train"),
            activity("land_unit:2", "dance"),
        ],
    );
    assert_eq!(status(&results[0]), "queued", "{results:?}");
    assert_eq!(status(&results[1]), "queued", "{results:?}");
    assert_eq!(status(&results[2]), "refused", "{results:?}");
    let reason = results[2]["reason"].as_str().expect("a reason");
    assert!(
        reason.starts_with(
            "(2,0) is not a coast hex on the map (prepare_landing names the coast hex"
        ),
        "{reason}"
    );
    assert_eq!(status(&results[3]), "refused", "{results:?}");
    let reason = results[3]["reason"].as_str().expect("a reason");
    assert!(
        reason.starts_with("the land unit is not one of your units (a load with a unit, and each unit of a prepare_landing roster"),
        "{reason}"
    );
    for refused in [&results[4], &results[5]] {
        assert_eq!(status(refused), "refused", "{results:?}");
        assert!(
            refused["reason"]
                .as_str()
                .expect("a reason")
                .starts_with("malformed order"),
            "{results:?}"
        );
    }
    // A unit still aboard may be set to train.
    assert_eq!(status(&results[6]), "queued", "{results:?}");
    assert_eq!(status(&results[7]), "queued", "{results:?}");
    // A unit that is nothing is `not known`, as for every other order.
    assert_eq!(status(&results[8]), "refused", "{results:?}");
    assert_eq!(results[8]["reason"], "not known", "{results:?}");
    assert_eq!(status(&results[9]), "refused", "{results:?}");
    assert!(
        results[9]["reason"]
            .as_str()
            .expect("a reason")
            .starts_with("malformed order"),
        "{results:?}"
    );
    // The other side has no unit by these numbers: the same one refusal.
    let results = orders(
        &mut session,
        Side::Japanese,
        &[
            prepare("task_force:1", 4, 0, &[]),
            activity("land_unit:1", "train"),
        ],
    );
    for r in &results {
        assert_eq!(r["reason"], "not known", "{results:?}");
    }
}

#[test]
fn a_force_shows_its_loading_and_the_rate_it_lands_at_and_a_unit_its_rehearsal() {
    let mut session = session_of(live("27_preparing_a_landing.toml", |t| t));
    let results = orders(
        &mut session,
        Side::Allied,
        &[
            prepare("task_force:1", 4, 0, &["land_unit:1"]),
            // Force 2 is given no order: it has no preparation to show.
            activity("land_unit:2", "train"),
        ],
    );
    assert!(results.iter().all(|r| status(r) == "queued"), "{results:?}");

    // Before the day: the order is only queued, nothing is prepared.
    let (_, data) = briefing(&mut session, Side::Allied);
    assert!(force_line(&data, "task_force:1")
        .get("preparation")
        .is_none());

    end_day(&mut session);
    let (text, data) = briefing(&mut session, Side::Allied);
    let force = force_line(&data, "task_force:1");
    let prep = &force["preparation"];
    assert_eq!(prep["hex"]["q"], 4, "{force}");
    assert_eq!(prep["hex"]["r"], 0, "{force}");
    assert_eq!(prep["roster"][0]["handle"], "land_unit:1", "{force}");
    assert_eq!(prep["loading"], "5", "{force}");
    assert_eq!(prep["landing_rate"], "0.525", "{force}");
    assert_eq!(prep["order"], 1, "{force}");
    assert!(
        text.contains("PREPARING a landing at (4,0)")
            && text.contains("roster land_unit:1: loading 5, lands at 0.525 of the port's rate there (by your order 1)"),
        "{text}"
    );
    assert!(
        force_line(&data, "task_force:2")
            .get("preparation")
            .is_none(),
        "no order, no preparation"
    );

    // The unit: its rehearsal for B, its roster, the fatigue it would land
    // with (40 x 95/100), and nothing of the activity it never had.
    let unit = unit_line(&data, "land_unit:1");
    assert_eq!(unit["rehearsal"]["hex"]["q"], 4, "{unit}");
    assert_eq!(unit["rehearsal"]["rehearsal"], "5", "{unit}");
    assert_eq!(unit["roster"]["task_force"]["handle"], "task_force:1");
    assert_eq!(unit["roster"]["landing_fatigue"], "38", "{unit}");
    assert_eq!(unit["activity"], "ready");
    assert_eq!(unit["experience"], 2);
    assert!(
        text.contains("rehearsed 5 for a landing at (4,0)")
            && text.contains("where it would gain fatigue 38 landing")
            && text.contains("; on the roster of task_force:1 "),
        "{text}"
    );
    // A unit on the roster of no force has neither.
    let other = unit_line(&data, "land_unit:3");
    assert!(other.get("rehearsal").is_none() && other.get("roster").is_none());

    // Yesterday's gains, each with its event.
    let land = &data["land_units"];
    let loading = &land["loading_yesterday"][0];
    assert_eq!(loading["task_force"]["handle"], "task_force:1", "{loading}");
    assert_eq!(loading["change"], "gained");
    assert_eq!(loading["loading"], "5");
    let rehearsal = &land["rehearsal_yesterday"][0];
    assert_eq!(rehearsal["land_unit"]["handle"], "land_unit:1");
    assert_eq!(rehearsal["change"], "gained");
    assert_eq!(rehearsal["rehearsal"], "5");
    assert!(
        text.contains("Loading yesterday: task_force:1 ")
            && text.contains("loading for (4,0) ")
            && text.contains(" is now 5")
            && text.contains("Rehearsal yesterday: land_unit:1 ")
            && text.contains("rehearsed for the landing at (4,0) ")
            && text.contains(", now 5"),
        "{text}"
    );

    // Unit 2 was set to train: at the standard 2 and the ceiling 2 it spends
    // the day and gains nothing; the line says what it does.
    let trained = unit_line(&data, "land_unit:2");
    assert_eq!(trained["activity"], "train", "{trained}");
    assert_eq!(trained["activity_order"], 2, "{trained}");
    assert!(
        text.contains("; activity train (by your order 2)"),
        "{text}"
    );

    // The same in list_forces and get_entity.
    let forces = session
        .call(Side::Allied, "list_forces", &json!({}))
        .expect("answered");
    valid(&schemas(), "list_forces.schema.json", &forces, "forces");
    assert_eq!(
        by_handle(&forces["task_forces"], "task_force", "task_force:1")["preparation"]
            ["landing_rate"],
        "0.525"
    );
    assert_eq!(
        by_handle(&forces["land_units"], "land_unit", "land_unit:1")["rehearsal"]["rehearsal"],
        "5"
    );
    let entity = session
        .call(
            Side::Allied,
            "get_entity",
            &json!({ "handle": "task_force:1" }),
        )
        .expect("answered");
    valid(&schemas(), "entity.schema.json", &entity, "force entity");
    assert_eq!(
        entity["entity"]["summary"]["preparation"]["loading"], "5",
        "{entity}"
    );
    let entity = session
        .call(
            Side::Allied,
            "get_entity",
            &json!({ "handle": "land_unit:1" }),
        )
        .expect("answered");
    valid(&schemas(), "entity.schema.json", &entity, "unit entity");
    assert_eq!(
        entity["entity"]["summary"]["roster"]["landing_fatigue"], "38",
        "{entity}"
    );
}

#[test]
fn a_new_landing_place_a_dropped_unit_and_a_taken_unit_are_told_with_their_reasons() {
    let mut session = session_of(live("27_preparing_a_landing.toml", |t| t));
    orders(
        &mut session,
        Side::Allied,
        &[
            prepare("task_force:1", 4, 0, &["land_unit:1"]),
            prepare("task_force:2", 4, 0, &["land_unit:2"]),
        ],
    );
    end_day(&mut session);
    end_day(&mut session);
    // Day 3: force 1 changes its landing place; force 2 takes unit 1 and drops
    // unit 2 by naming another roster.
    orders(
        &mut session,
        Side::Allied,
        &[
            prepare("task_force:1", 0, 6, &["land_unit:1"]),
            prepare("task_force:2", 4, 0, &["land_unit:1", "land_unit:3"]),
        ],
    );
    end_day(&mut session);
    let (text, data) = briefing(&mut session, Side::Allied);
    let land = &data["land_units"];
    let lost = by_handle(&land["loading_yesterday"], "task_force", "task_force:1");
    assert_eq!(lost["change"], "lost", "{lost}");
    assert_eq!(lost["reason"], "new_landing_place", "{lost}");
    assert_eq!(lost["loading_before"], "10", "{lost}");
    // The unit that left force 1's roster by force 2's order, and the one force
    // 2's new order dropped.
    let taken: Vec<_> = land["roster_left_yesterday"]
        .as_array()
        .expect("roster changes")
        .iter()
        .map(|r| {
            (
                r["land_unit"]["handle"].as_str().unwrap().to_string(),
                r["task_force"]["handle"].as_str().unwrap().to_string(),
                r["reason"].as_str().unwrap().to_string(),
            )
        })
        .collect();
    assert!(
        taken.contains(&(
            "land_unit:1".into(),
            "task_force:1".into(),
            "taken_by_another_force".into()
        )),
        "{taken:?}"
    );
    assert!(
        taken.contains(&(
            "land_unit:2".into(),
            "task_force:2".into(),
            "dropped_by_order".into()
        )),
        "{taken:?}"
    );
    assert!(
        text.contains("Roster yesterday: land_unit:1 ")
            && text.contains(": another force's order took it")
            && text.contains("Roster yesterday: land_unit:2 ")
            && text.contains(": your order dropped it"),
        "{text}"
    );
}

#[test]
fn training_and_fighting_count_days_and_raise_experience_and_each_is_told() {
    let mut session = session_of(live("28_experience_and_training.toml", |t| t));
    let (_, data) = briefing(&mut session, Side::Allied);
    let u = unit_line(&data, "land_unit:1");
    assert_eq!(u["experience"], 0, "{u}");
    assert_eq!(u["training"]["days"], 0);
    assert_eq!(u["training"]["of_days"], 30);
    assert_eq!(u["training"]["up_to"], 2);
    assert_eq!(unit_line(&data, "land_unit:2")["fighting"]["days"], 9);
    assert_eq!(unit_line(&data, "land_unit:2")["fighting"]["of_days"], 10);
    let results = orders(
        &mut session,
        Side::Allied,
        &[activity("land_unit:1", "train")],
    );
    assert_eq!(status(&results[0]), "queued", "{results:?}");
    end_day(&mut session);
    let (text, data) = briefing(&mut session, Side::Allied);
    let land = &data["land_units"];

    let u = unit_line(&data, "land_unit:1");
    assert_eq!(u["activity"], "train", "{u}");
    assert_eq!(u["training"]["days"], 1, "{u}");
    assert_eq!(u["experience"], 0, "{u}");
    let day = by_handle(
        &land["experience_days_yesterday"],
        "land_unit",
        "land_unit:1",
    );
    assert_eq!(day["source"], "training", "{day}");
    assert_eq!(day["days"], 1);
    assert!(
        text.contains("Experience yesterday: land_unit:1 ")
            && text.contains("counted a day of training, 1 toward the next step from experience 0"),
        "{text}"
    );

    // Unit 2 fired in the Japanese attack on its tenth day: a step to 3, the
    // count back to 0.
    let u2 = unit_line(&data, "land_unit:2");
    assert_eq!(u2["experience"], 3, "{u2}");
    assert_eq!(u2["fighting"]["days"], 0, "{u2}");
    let raised = by_handle(
        &land["experience_raised_yesterday"],
        "land_unit",
        "land_unit:2",
    );
    assert_eq!(raised["experience"], 3, "{raised}");
    assert_eq!(raised["by"], "fighting", "{raised}");
    assert!(
        text.contains("Experience raised yesterday: land_unit:2 ")
            && text.contains("reached experience 3 by fighting"),
        "{text}"
    );
    assert!(
        text.contains(
            "experience 3 (0 of 30 days of training, up to experience 2; 0 of 10 days of fighting)"
        ),
        "{text}"
    );
}

/// A world as the tests below build it: the scenario text with a change.
fn world_of(name: &str, edit: impl Fn(String) -> String) -> World {
    live(name, edit).world
}

/// The whole of what `side` is told each day for `days` days in a world, with
/// `orders_for` given on their days: the briefing as text and as data and
/// the lists.
fn told(world: World, side: Side, days: u32, orders_for: &[(u32, Side, Vec<Value>)]) -> String {
    // A world in a session, so the same orders reach it as a player's.
    let mut out = String::new();
    let mut session = Session::new(world, "Leak", days + 1, Seat::Player, Seat::Player);
    for day in 1..=days {
        for (on_day, who, list) in orders_for {
            if *on_day == day {
                orders(&mut session, *who, list);
            }
        }
        end_day(&mut session);
        let (text, data) = briefing(&mut session, side);
        out.push_str(&text);
        out.push_str(&data.to_string());
        for tool in ["list_forces", "list_bases", "list_contacts"] {
            let answer = session
                .call(side, tool, &json!({}))
                .expect("a list is answered");
            out.push_str(&answer.to_string());
        }
    }
    out
}

#[test]
fn the_enemy_is_told_nothing_of_a_preparation_a_training_or_an_experience() {
    // Scenario 28 gives the Japanese side units of its own, at F and G far
    // from Allied unit 1 at base 1. Two worlds differ only in the Allied
    // side's unit 1: its experience, and whether it trains. The Japanese side
    // is told the same in both, whole.
    let plain = world_of("28_experience_and_training.toml", |t| t);
    let trained = world_of("28_experience_and_training.toml", |t| {
        t.replacen("experience = 0\n\n# Day 1", "experience = 4\n\n# Day 1", 1)
    });
    let order = (1, Side::Allied, vec![activity("land_unit:1", "train")]);
    let base = told(plain, Side::Japanese, 3, &[]);
    let with_orders = told(trained, Side::Japanese, 3, &[order]);
    assert_eq!(
        base, with_orders,
        "a unit's training and experience are told to its own side only"
    );

    // Scenario 27: two worlds differing only in an Allied prepare order give
    // the Japanese side, which has no unit there, the same answers.
    let a = told(
        world_of("27_preparing_a_landing.toml", |t| t),
        Side::Japanese,
        3,
        &[],
    );
    let b = told(
        world_of("27_preparing_a_landing.toml", |t| t),
        Side::Japanese,
        3,
        &[(
            1,
            Side::Allied,
            vec![prepare("task_force:1", 4, 0, &["land_unit:1"])],
        )],
    );
    assert_eq!(a, b, "a preparation is told to its own side only");
    // And the own side is told, so the test is not told nothing for want of
    // anything to tell.
    let own = told(
        world_of("27_preparing_a_landing.toml", |t| t),
        Side::Allied,
        1,
        &[(
            1,
            Side::Allied,
            vec![prepare("task_force:1", 4, 0, &["land_unit:1"])],
        )],
    );
    assert!(own.contains("PREPARING a landing at (4,0)"), "{own}");
}

#[test]
fn every_answer_of_both_sides_on_every_day_is_in_the_schemas() {
    let schemas = schemas();
    for (name, days) in [
        ("27_preparing_a_landing.toml", 12),
        ("28_experience_and_training.toml", 12),
    ] {
        let mut scenario = live(name, |t| t);
        let title = scenario.name.clone();
        let mut day = 0;
        support::play(&mut scenario, |s| {
            day += 1;
            if day > days {
                return;
            }
            for side in [Side::Allied, Side::Japanese] {
                let view = s.world.side_view(side);
                check_view(&view, &title, &schemas, true);
            }
        });
    }
}
