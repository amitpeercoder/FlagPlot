//! The proof of decision 0156 (orders inside a day), step 5: the tests and
//! the measuring runs its conditions need. `docs/validation/
//! orders-inside-a-day-proof.md` says what each gave; the scenario files are
//! made by `scripts/orders-inside-proof/make_arrangements.py` into
//! `data/scenarios/orders-inside-proof/`.
//!
//! - Condition 2: [`condition_2_by_pulse_has_the_hashes_of_by_days`], marked
//!   `#[ignore]`, run in release over a range of seeds given by
//!   `PROOF_SEEDS=FIRST-LAST`. It prints the count that agree and the first
//!   seed that does not.
//! - Condition 5: [`condition_5_events_at_0600_and_1200_are_the_seats_view`],
//!   run with the rest.
//! - Condition 6: [`condition_6_the_six_days`], marked `#[ignore]`, writes the
//!   events of the six days to `PROOF_OUT`.
//! - Condition 8: [`condition_8_speed`], marked `#[ignore]`, times a day
//!   resolved in one call and in three, `PROOF_REPEATS` times (30 by
//!   default), and prints the medians.
//!
//! None of these is a hand calculation, and no number here is an expected
//! figure: each compares two ways of playing the same game, or a tool's
//! answer with the side's own view.

mod support;

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde_json::{json, Value};
use sim_agent::Session;
use sim_core::orders::OrderRequest;
use sim_core::state::Side;
use sim_data::Scenario;
use support::play::{script_of, served_json, session_of};
use support::ROOT;

const ALLIED: Side = Side::Allied;

fn proof_path(name: &str) -> PathBuf {
    PathBuf::from(ROOT)
        .join("data/scenarios/orders-inside-proof")
        .join(name)
}

fn live_path(name: &str) -> PathBuf {
    PathBuf::from(ROOT).join("data/scenarios").join(name)
}

/// A proof file on the live figures, as `sim-cli` loads it.
fn proof(name: &str, seed: Option<u64>) -> Scenario {
    let scenario = Scenario::load(&proof_path(name)).expect("the proof file loads");
    match seed {
        Some(seed) => scenario.with_seed(seed).expect("a seed can be set"),
        None => scenario,
    }
}

/// Scenario 78 with its order inside a day taken out (its case C): the text
/// cut as `tests/advance.rs` cuts it.
fn scenario_78_case_c() -> Scenario {
    let path = live_path("78_orders_inside_a_day.toml");
    let text = std::fs::read_to_string(&path).expect("scenario 78 reads");
    let cut = text.find("# Day 1, at 0600").expect("the order is marked");
    let rest = &text[cut..];
    let end = rest.find("# Days 2 and 3").expect("the file goes on");
    let text = format!("{}{}", &text[..cut], &rest[end..]);
    Scenario::from_text(&text, &path).expect("scenario 78 loads")
}

/// The scenario's daily hashes run by days with the doctrine's staffs, as
/// `sim-cli run` runs it.
fn run_by_days(mut scenario: Scenario) -> Vec<u64> {
    let held = sim_staff::staffs();
    while scenario
        .next_day_staffed(sim_staff::as_staffs(&held))
        .expect("the day resolves")
        .is_some()
    {}
    scenario.world.day_hashes().to_vec()
}

/// A session as `sim-cli play` opens one (as `tests/advance.rs` builds it).
fn session(scenario: Scenario, player: Side) -> (Session, BTreeMap<u32, Vec<OrderRequest>>) {
    let commands = scenario.scripted_commands().clone();
    let inside_on = scenario.orders_inside_a_day();
    let inside = scenario.scripted_orders_inside_day().clone();
    let script = script_of(&scenario, player);
    let (session, _) = session_of(scenario, &[player]);
    let session = session
        .with_staffs(sim_staff::staffs())
        .with_commands(commands)
        .with_orders_inside_a_day(inside_on, inside);
    (session, script)
}

fn call(session: &mut Session, side: Side, tool: &str, args: Value) -> Value {
    session
        .call(side, tool, &args)
        .unwrap_or_else(|e| panic!("{tool} {args} refused: {e}"))
}

/// Gives the script's orders of the coming day as the player's.
fn give_day(session: &mut Session, side: Side, script: &BTreeMap<u32, Vec<OrderRequest>>) {
    let day = session.day() + 1;
    let orders: Vec<Value> = script
        .get(&day)
        .into_iter()
        .flatten()
        .map(|o| served_json(side, o))
        .collect();
    if orders.is_empty() {
        return;
    }
    let answer = call(session, side, "issue_orders", json!({ "orders": orders }));
    assert_eq!(answer["refused_in_call"], 0, "{answer}");
}

/// The game played as `player`, the script's orders at each dusk, advanced
/// one pulse at a time with no order inside a day.
fn by_pulses(scenario: Scenario, player: Side) -> Vec<u64> {
    let days = scenario.days;
    let (mut s, script) = session(scenario, player);
    for _ in 1..=days {
        give_day(&mut s, player, &script);
        for _ in 0..3 {
            let answer = call(&mut s, player, "advance", json!({"pulses": 1}));
            if answer["game_over"] == true {
                break;
            }
        }
    }
    assert!(s.is_over());
    s.day_hashes().to_vec()
}

// ---------------------------------------------------------------------
// Condition 2.
// ---------------------------------------------------------------------

fn seed_range() -> (u64, u64) {
    let text = std::env::var("PROOF_SEEDS").unwrap_or_else(|_| "1-1000".to_string());
    let (a, b) = text.split_once('-').expect("PROOF_SEEDS is FIRST-LAST");
    (
        a.parse().expect("a first seed"),
        b.parse().expect("a last seed"),
    )
}

/// On each seed of `PROOF_SEEDS` (1-1000 by default), the Eastern Solomons
/// with the key on (`05_eastern_solomons_on.toml`) run by days, and played
/// through a session advanced a pulse at a time with no order inside a day,
/// once from each seat. Prints the count of seeds on which all three games
/// have the same daily hashes, and the first that does not; fails if any
/// does not.
#[test]
#[ignore = "the proof's run: PROOF_SEEDS=1-1000, in release"]
fn condition_2_by_pulse_has_the_hashes_of_by_days() {
    let (first, last) = seed_range();
    let mut agree = 0u64;
    let mut allied_agree = 0u64;
    let mut japanese_agree = 0u64;
    let mut differ: Vec<u64> = Vec::new();
    for seed in first..=last {
        let file = "05_eastern_solomons_on.toml";
        let by_days = run_by_days(proof(file, Some(seed)));
        let allied = by_pulses(proof(file, Some(seed)), Side::Allied) == by_days;
        let japanese = by_pulses(proof(file, Some(seed)), Side::Japanese) == by_days;
        allied_agree += u64::from(allied);
        japanese_agree += u64::from(japanese);
        if allied && japanese {
            agree += 1;
        } else {
            differ.push(seed);
        }
    }
    let runs = last - first + 1;
    println!(
        "condition 2: seeds {first} to {last}: {agree} of {runs} agree from both seats \
         (Allied seat {allied_agree}, Japanese seat {japanese_agree}); first that does not: {}",
        differ
            .first()
            .map_or("none".to_string(), |seed| seed.to_string())
    );
    assert!(differ.is_empty(), "seeds that differ: {differ:?}");
}

// ---------------------------------------------------------------------
// Condition 5.
// ---------------------------------------------------------------------

/// At 0600 and at 1200 of the Eastern Solomons with the key on, from each
/// seat: the `events` tool's count for the open day is the number of the
/// side's own `SideView` events of that day at that moment, its events are
/// those, by number, and none is of a pulse not yet resolved.
#[test]
fn condition_5_events_at_0600_and_1200_are_the_seats_view() {
    for side in Side::ALL {
        for (pulses, at, allowed) in [
            (1, "0600", vec![Value::Null, json!("night")]),
            (
                2,
                "1200",
                vec![Value::Null, json!("night"), json!("morning")],
            ),
        ] {
            let (mut s, script) = session(proof("05_eastern_solomons_on.toml", None), side);
            give_day(&mut s, side, &script);
            let stop = call(&mut s, side, "advance", json!({"pulses": pulses}));
            assert_eq!(stop["clock"]["at"], at, "{stop}");
            let answer = call(&mut s, side, "events", json!({}));
            assert_eq!(answer["day"], 1, "{answer}");
            assert_eq!(answer["begun"], true, "{answer}");
            let listed = answer["events"].as_array().cloned().unwrap_or_default();
            let count = answer["count"].as_u64().expect("a count");
            assert_eq!(count, listed.len() as u64, "{answer}");
            for event in &listed {
                assert!(
                    allowed.contains(&event["pulse"]),
                    "{side:?} at {at}: an event of a pulse not resolved: {event}"
                );
            }
            let numbers: Vec<u64> = listed
                .iter()
                .map(|e| e["number"].as_u64().expect("a number"))
                .collect();
            // The side's own view of the world at that moment.
            let world = s.into_world();
            let view = world.side_view(side);
            let own: Vec<u64> = view
                .events_on_day(1)
                .iter()
                .map(|e| u64::from(e.number))
                .collect();
            assert_eq!(count, own.len() as u64, "{side:?} at {at}");
            assert_eq!(numbers, own, "{side:?} at {at}");
            assert!(count > 0, "{side:?} at {at}: the day has events");
            println!(
                "condition 5: {side:?} at {at}: events counts {count}, the view holds {}",
                own.len()
            );
        }
    }
}

// ---------------------------------------------------------------------
// Condition 6: the six days.
// ---------------------------------------------------------------------

/// One case of condition 6: the game, the seat, the pulses advanced before
/// the order and the order.
struct Case {
    letter: &'static str,
    scenario: fn() -> Scenario,
    side: Side,
    dusk_orders: Option<Value>,
    pulses_before: u32,
    order: Value,
}

fn case_78() -> Scenario {
    scenario_78_case_c()
}

fn case_36() -> Scenario {
    proof("36_one_convoy_on.toml", None)
}

fn case_es() -> Scenario {
    proof("05_eastern_solomons_on.toml", None)
}

fn route_and_cruise() -> Value {
    json!({"orders": [
        {"unit": "task_force:1", "order": {"kind": "set_route", "waypoints": [[8, 3]]}},
        {"unit": "task_force:1", "order": {"kind": "set_speed", "speed": "cruise"}}
    ]})
}

/// Plays each case to the dusk of day 1 and writes the side's events of day
/// 1, as the `events` tool gives them then, to `PROOF_OUT/case_X.json`, with
/// the answer to the order inside the day beside it (for the record, not for
/// the readers).
#[test]
#[ignore = "the proof's run: PROOF_OUT=folder"]
fn condition_6_the_six_days() {
    let out = PathBuf::from(std::env::var("PROOF_OUT").expect("PROOF_OUT is set"));
    std::fs::create_dir_all(&out).expect("the folder is made");
    let eight =
        json!({"orders": [{"unit": "task_force:1", "order": {"kind": "set_speed", "knots": "8"}}]});
    let cases = [
        Case {
            letter: "A",
            scenario: case_78,
            side: ALLIED,
            dusk_orders: Some(route_and_cruise()),
            pulses_before: 1,
            order: eight.clone(),
        },
        Case {
            letter: "B",
            scenario: case_78,
            side: ALLIED,
            dusk_orders: Some(route_and_cruise()),
            pulses_before: 2,
            order: eight,
        },
        Case {
            letter: "C",
            scenario: case_36,
            side: ALLIED,
            dusk_orders: None,
            pulses_before: 1,
            order: Value::Null, // the convoy's force, found at 0600
        },
        Case {
            letter: "D",
            scenario: case_es,
            side: ALLIED,
            dusk_orders: None,
            pulses_before: 1,
            order: json!({"orders": [{"unit": "task_force:2", "order": {"kind": "set_readiness", "readiness": "battle"}}]}),
        },
        Case {
            letter: "E",
            scenario: case_es,
            side: ALLIED,
            dusk_orders: None,
            pulses_before: 2,
            order: json!({"orders": [{"unit": "air_group:6", "order": {"kind": "set_air_mission", "mission": "rest"}}]}),
        },
        Case {
            letter: "F",
            scenario: case_78,
            side: ALLIED,
            dusk_orders: Some(route_and_cruise()),
            pulses_before: 2,
            order: json!({"orders": [{"unit": "task_force:1", "order": {"kind": "set_route", "waypoints": [[5, 0]]}}]}),
        },
    ];
    for case in cases {
        let (mut s, script) = session((case.scenario)(), case.side);
        match &case.dusk_orders {
            Some(orders) => {
                call(&mut s, case.side, "issue_orders", orders.clone());
            }
            None => give_day(&mut s, case.side, &script),
        }
        call(
            &mut s,
            case.side,
            "advance",
            json!({"pulses": case.pulses_before}),
        );
        let order = if case.order.is_null() {
            // Case C: the force the staff formed at dusk, the side's only one.
            let forces = call(&mut s, case.side, "list_forces", json!({}));
            let list = forces["task_forces"].as_array().expect("forces");
            assert_eq!(list.len(), 1, "{forces}");
            let handle = list[0]["task_force"]["handle"]
                .as_str()
                .or_else(|| list[0]["handle"].as_str())
                .expect("a handle")
                .to_string();
            json!({"orders": [{"unit": handle, "order": {"kind": "set_speed", "knots": "8"}}]})
        } else {
            case.order.clone()
        };
        let given = call(&mut s, case.side, "issue_orders", order.clone());
        assert_eq!(given["refused_in_call"], 0, "{given}");
        let rest = 3 - case.pulses_before;
        call(&mut s, case.side, "advance", json!({"pulses": rest}));
        assert_eq!(s.day(), 1);
        let events = call(&mut s, case.side, "events", json!({"day": 1}));
        let record = json!({
            "case": case.letter,
            "side": format!("{:?}", case.side),
            "order": order,
            "answer": given,
            "events": events,
        });
        let path = out.join(format!("case_{}.json", case.letter));
        std::fs::write(
            &path,
            serde_json::to_string_pretty(&record).expect("json") + "\n",
        )
        .expect("written");
        println!(
            "condition 6: case {} written to {}",
            case.letter,
            path.display()
        );
    }
}

// ---------------------------------------------------------------------
// Condition 8: speed.
// ---------------------------------------------------------------------

/// The one place this file reads the clock. What it returns is only
/// subtracted from a later reading to print a duration; nothing of a game is
/// made from it.
#[allow(clippy::disallowed_methods)]
fn clock() -> std::time::Instant {
    std::time::Instant::now()
}

fn median(mut values: Vec<f64>) -> f64 {
    values.sort_by(|a, b| a.partial_cmp(b).expect("no NaN"));
    let n = values.len();
    if n % 2 == 1 {
        values[n / 2]
    } else {
        (values[n / 2 - 1] + values[n / 2]) / 2.0
    }
}

/// The text of a scenario with the key on, written after its `days` line.
fn with_key_on(path: &PathBuf) -> Scenario {
    let text = std::fs::read_to_string(path).expect("the scenario reads");
    if text.contains("orders_inside_a_day") {
        return Scenario::from_text(&text, path).expect("the scenario loads");
    }
    let mut out = String::new();
    let mut done = false;
    for line in text.lines() {
        out.push_str(line);
        out.push('\n');
        if !done && line.starts_with("days = ") {
            out.push_str("orders_inside_a_day = true\n");
            done = true;
        }
    }
    assert!(done);
    Scenario::from_text(&out, path).expect("the scenario loads")
}

/// The engine, `days` days: each day by one call (`next_day_staffed`, which
/// is `resolve_day_watched`) or by the parts called one by one from outside
/// (`open_day`, `resolve_next_pulse` three times, `close_day`), the orders
/// and commands of the file handed in first as `next_day_watched` hands
/// them. Returns the milliseconds of the days, summed.
fn engine_days(make: &dyn Fn() -> Scenario, days: u32, parts: bool) -> f64 {
    let mut scenario = make();
    let held = sim_staff::staffs();
    let mut total = 0.0;
    for day in 1..=days {
        if parts {
            let commands = scenario.scripted_commands().get(&day).cloned();
            let orders = scenario.scripted_orders().get(&day).cloned();
            let world = &mut scenario.world;
            for (side, command) in commands.unwrap_or_default() {
                world.submit_commands(side, vec![command]);
            }
            for (side, request) in orders.unwrap_or_default() {
                world.submit_orders(side, vec![request]);
            }
            let start = clock();
            world
                .open_day(sim_staff::as_staffs(&held), &mut |_, _| {})
                .expect("opens");
            for _ in 0..3 {
                world.resolve_next_pulse(&mut |_, _| {}).expect("a pulse");
            }
            world.close_day(&mut |_, _| {}).expect("closes");
            total += start.elapsed().as_secs_f64() * 1000.0;
        } else {
            // `next_day_staffed` hands in the day's orders itself; they are a
            // few vector pushes, inside the reading.
            let start = clock();
            scenario
                .next_day_staffed(sim_staff::as_staffs(&held))
                .expect("the day")
                .expect("a day is left");
            total += start.elapsed().as_secs_f64() * 1000.0;
        }
    }
    total
}

/// The session, `days` days, the Allied seat a player's with the script's
/// orders given at each dusk (outside the reading): each day by `end_day`
/// (one call) or by `advance {pulses: 1}` three times. Returns the
/// milliseconds of the calls, summed.
fn session_days(make: &dyn Fn() -> Scenario, days: u32, three: bool) -> f64 {
    let (mut s, script) = session(make(), ALLIED);
    let mut total = 0.0;
    for _ in 1..=days {
        give_day(&mut s, ALLIED, &script);
        let start = clock();
        if three {
            for _ in 0..3 {
                std::hint::black_box(call(&mut s, ALLIED, "advance", json!({"pulses": 1})));
            }
        } else {
            std::hint::black_box(call(&mut s, ALLIED, "end_day", json!({})));
        }
        total += start.elapsed().as_secs_f64() * 1000.0;
    }
    total
}

/// Times a day resolved in one call and in three, on the Eastern Solomons
/// (its one day) and on the first ten days of the staff's setup of the full
/// slice, each way `PROOF_REPEATS` times (30 by default), the two ways taken
/// in turn. Prints the medians in milliseconds and their ratio. Both
/// scenarios are read with the key on (the Eastern Solomons from the proof
/// folder; the slice with the key written in, its file not edited), and the
/// same game is resolved both ways.
#[test]
#[ignore = "the proof's run: in release, PROOF_REPEATS=30"]
fn condition_8_speed() {
    let repeats: usize = std::env::var("PROOF_REPEATS")
        .ok()
        .map_or(30, |r| r.parse().expect("a count"));
    let es: Box<dyn Fn() -> Scenario> = Box::new(|| proof("05_eastern_solomons_on.toml", None));
    let slice_path = PathBuf::from(ROOT).join("data/scenarios/staff/07_full_slice.toml");
    let slice: Box<dyn Fn() -> Scenario> = Box::new(move || with_key_on(&slice_path));
    for (name, make, days) in [
        ("the Eastern Solomons, 1 day", &es, 1u32),
        (
            "the staff's setup of the full slice, days 1 to 10",
            &slice,
            10u32,
        ),
    ] {
        // One run of each way first, unmeasured, to warm the caches.
        engine_days(make.as_ref(), days, false);
        engine_days(make.as_ref(), days, true);
        session_days(make.as_ref(), days, false);
        session_days(make.as_ref(), days, true);
        let (mut e1, mut e3, mut s1, mut s3) = (vec![], vec![], vec![], vec![]);
        for _ in 0..repeats {
            e1.push(engine_days(make.as_ref(), days, false));
            e3.push(engine_days(make.as_ref(), days, true));
            s1.push(session_days(make.as_ref(), days, false));
            s3.push(session_days(make.as_ref(), days, true));
        }
        let (me1, me3, ms1, ms3) = (median(e1), median(e3), median(s1), median(s3));
        println!(
            "condition 8: {name}, {repeats} times each way, medians of the days summed:\n  \
             engine: one call {me1:.3} ms, open + three pulses + close {me3:.3} ms, ratio {:.3}\n  \
             session: end_day {ms1:.3} ms, advance x3 {ms3:.3} ms, ratio {:.3}",
            me3 / me1,
            ms3 / ms1
        );
    }
}
