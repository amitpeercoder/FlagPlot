//! The tools of decision 0156, step 3: `events`, `advance`, orders given at
//! 0600 and 1200, and the other tools there.
//!
//! These are not hand calculations: every number asserted here is a hash of
//! the same game played another way, or a fact read back from the game's own
//! answers. The games are on the live figures, as `sim-cli` runs them, since
//! the first test compares with scenario 78 run by `sim-cli`
//! (`Scenario::next_day_staffed` with the doctrine's staffs, which is what
//! `sim-cli run` calls).

mod support;

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde_json::{json, Value};
use sim_agent::{CallError, Seat, Session};
use sim_core::orders::OrderRequest;
use sim_core::state::Side;
use sim_data::Scenario;
use support::play::{script_of, served_json, session_of};
use support::ROOT;

fn live_path(name: &str) -> PathBuf {
    PathBuf::from(ROOT).join("data/scenarios").join(name)
}

fn live_text(name: &str) -> String {
    std::fs::read_to_string(live_path(name)).expect("the scenario reads")
}

/// The scenario's text with the key `orders_inside_a_day` set on, written
/// after its `days` line (the file itself is not edited).
fn with_key_on(text: &str) -> String {
    assert!(
        !text.contains("orders_inside_a_day"),
        "the key is set already"
    );
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
    assert!(done, "the scenario has a days line");
    out
}

fn scenario(name: &str, text: &str) -> Scenario {
    Scenario::from_text(text, &live_path(name)).expect("the scenario loads")
}

/// Scenario 78 with its order inside a day taken out: scenario 1 with the
/// key on (case C).
fn case_c_text() -> String {
    let text = live_text("78_orders_inside_a_day.toml");
    let marker = "# Day 1, at 0600";
    let cut = text
        .find(marker)
        .expect("the order inside the day is marked");
    let rest = &text[cut..];
    let end = rest.find("# Days 2 and 3").expect("the file goes on");
    format!("{}{}", &text[..cut], &rest[end..])
}

/// The daily hashes of the scenario run by days with the doctrine's staffs,
/// as `sim-cli run` runs it.
fn run_by_days(mut scenario: Scenario) -> Vec<u64> {
    let held = sim_staff::staffs();
    while scenario
        .next_day_staffed(sim_staff::as_staffs(&held))
        .expect("the day resolves")
        .is_some()
    {}
    scenario.world.day_hashes().to_vec()
}

/// A session as `sim-cli serve`, `mcp` and `play` open one: `player` held by
/// a player, the other side by the script, the doctrine's staffs, the
/// script's commands and the scenario's key and orders inside a day.
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

fn refused(session: &mut Session, side: Side, tool: &str, args: Value) -> CallError {
    session
        .call(side, tool, &args)
        .expect_err("the call is refused")
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

fn check(file: &str, answer: &Value) {
    let found = support::schemas().check(file, answer);
    assert!(found.is_empty(), "{file}: {found:?}\n{answer}");
}

const ALLIED: Side = Side::Allied;

fn route_and_cruise() -> Value {
    json!({"orders": [
        {"unit": "task_force:1", "order": {"kind": "set_route", "waypoints": [[8, 3]]}},
        {"unit": "task_force:1", "order": {"kind": "set_speed", "speed": "cruise"}}
    ]})
}

fn eight_knots() -> Value {
    json!({"orders": [{"unit": "task_force:1", "order": {"kind": "set_speed", "knots": "8"}}]})
}

// ---------------------------------------------------------------------
// 1. Scenario 78's cases A and B played through a session.
// ---------------------------------------------------------------------

/// Plays scenario 1 with the key on as the Allied player: the dusk orders,
/// `before` pulses, the 8-knot order, then to the end.
fn played_with_order_after(pulses_before: u32) -> (Vec<u64>, Value) {
    let (mut s, _) = session(
        scenario("78_orders_inside_a_day.toml", &case_c_text()),
        ALLIED,
    );
    call(&mut s, ALLIED, "issue_orders", route_and_cruise());
    let first = call(&mut s, ALLIED, "advance", json!({"pulses": pulses_before}));
    check("advance.schema.json", &first);
    let given = call(&mut s, ALLIED, "issue_orders", eight_knots());
    check("issue_orders.schema.json", &given);
    let rest = call(&mut s, ALLIED, "advance", json!({"pulses": 20}));
    check("advance.schema.json", &rest);
    assert_eq!(rest["stopped"], "game_over", "{rest}");
    (s.day_hashes().to_vec(), given)
}

#[test]
fn case_a_through_a_session_has_scenario_78s_hashes() {
    let file = run_by_days(scenario(
        "78_orders_inside_a_day.toml",
        &live_text("78_orders_inside_a_day.toml"),
    ));
    let (hashes, given) = played_with_order_after(1);
    assert_eq!(given["results"][0]["status"], "queued", "{given}");
    assert_eq!(hashes, file);
    assert_eq!(given["for_pulse"], "morning");
    assert_eq!(given["for_day"], 1);
    assert_eq!(given["results"][0]["status"], "queued");
    assert_eq!(given["results"][0]["for"], "the morning pulse of day 1");
}

#[test]
fn case_b_through_a_session_has_the_hashes_of_the_order_at_1200() {
    let mut b = scenario(
        "78_orders_inside_a_day.toml",
        &live_text("78_orders_inside_a_day.toml"),
    );
    let moved = b
        .scripted_orders_inside_day()
        .iter()
        .map(|(day, list)| {
            let list = list
                .iter()
                .map(|(_, side, r)| (sim_core::time::Pulse::Afternoon, *side, r.clone()))
                .collect();
            (*day, list)
        })
        .collect();
    b.replace_script_inside_day(moved);
    let file = run_by_days(b);
    let (hashes, given) = played_with_order_after(2);
    assert_eq!(hashes, file);
    assert_eq!(given["for_pulse"], "afternoon");
    assert_eq!(given["results"][0]["for"], "the afternoon pulse of day 1");
    // And the order is not the morning's: case A's hashes differ.
    let (a, _) = played_with_order_after(1);
    assert_ne!(a[0], hashes[0]);
}

// ---------------------------------------------------------------------
// 2. Advancing pulse by pulse with no order inside a day.
// ---------------------------------------------------------------------

/// Plays the scenario as `player` with the script's orders given at each
/// dusk, advancing a pulse at a time, and checks the clock at each stop.
fn by_pulses(scenario: Scenario, player: Side) -> Vec<u64> {
    let days = scenario.days;
    let (mut s, script) = session(scenario, player);
    for day in 1..=days {
        give_day(&mut s, player, &script);
        for (pulse, at) in [
            ("night", "0600"),
            ("morning", "1200"),
            ("afternoon", "dusk"),
        ] {
            let answer = call(&mut s, player, "advance", json!({"pulses": 1}));
            assert_eq!(
                answer["pulses_resolved"],
                json!([{"day": day, "pulse": pulse}]),
                "{answer}"
            );
            assert_eq!(answer["clock"]["at"], at, "{answer}");
            if answer["game_over"] == true {
                assert_eq!(day, days);
                break;
            }
        }
    }
    assert!(s.is_over());
    s.day_hashes().to_vec()
}

#[test]
fn advancing_pulse_by_pulse_gives_the_hashes_of_the_days_on_scenario_78_without_its_order() {
    let text = case_c_text();
    let by_days = run_by_days(scenario("78_orders_inside_a_day.toml", &text));
    // Scenario 1's own hashes too (case C).
    let one = run_by_days(scenario("01_transit.toml", &live_text("01_transit.toml")));
    assert_eq!(by_days, one);
    assert_eq!(
        by_pulses(scenario("78_orders_inside_a_day.toml", &text), ALLIED),
        by_days
    );
}

#[test]
fn advancing_pulse_by_pulse_gives_the_hashes_of_the_days_on_scenario_36_with_its_staff() {
    let text = with_key_on(&live_text("36_one_convoy.toml"));
    let by_days = run_by_days(scenario("36_one_convoy.toml", &text));
    assert_eq!(
        by_pulses(scenario("36_one_convoy.toml", &text), ALLIED),
        by_days
    );
}

#[test]
fn advancing_pulse_by_pulse_gives_the_hashes_of_the_days_on_the_eastern_solomons() {
    let text = with_key_on(&live_text("05_eastern_solomons.toml"));
    let by_days = run_by_days(scenario("05_eastern_solomons.toml", &text));
    for player in Side::ALL {
        assert_eq!(
            by_pulses(scenario("05_eastern_solomons.toml", &text), player),
            by_days
        );
    }
}

// ---------------------------------------------------------------------
// 3. stop_on.
// ---------------------------------------------------------------------

/// The side's events of one pulse, as `events` gives them.
fn events_of(s: &mut Session, side: Side, day: u64, pulse: &str) -> Vec<Value> {
    let answer = call(s, side, "events", json!({"day": day, "pulse": pulse}));
    check("events.schema.json", &answer);
    answer["events"].as_array().cloned().unwrap_or_default()
}

/// Whether one of `events` is a thing of `stop_on`'s `thing`, by its kind
/// and words as the side reads them.
fn is_thing(thing: &str, event: &Value) -> bool {
    let kind = event["event_kind"].as_str().unwrap_or_default();
    let text = event["text"].as_str().unwrap_or_default();
    match thing {
        "contact" => kind == "report_received" && text.contains(": new contact:"),
        "attack" => matches!(
            kind,
            "air_attack_against"
                | "base_attacked_against"
                | "shore_bombardment_against"
                | "salvo_against"
                | "engagement_begun"
        ),
        "loss" => {
            kind == "ship_sunk"
                || (kind == "system_hit"
                    && (text.contains("propulsion hit") || text.contains("flight deck hit"))
                    && text.ends_with("-> out"))
                // A carrier set on fire among her aircraft flies nothing off
                // or on while it burns (decision 0212): told as a loss.
                || (kind == "aircraft_fire_roll" && text.contains(": set, fire"))
        }
        "strike" => matches!(kind, "strike_launched" | "base_strike_launched"),
        "landing" => matches!(kind, "men_landed" | "ground_fight"),
        _ => unreachable!(),
    }
}

/// Plays the scenario as `player`, the script's orders at each dusk, with
/// `advance {pulses: 3, stop_on: [thing]}` until it stops on what it was
/// told. Asserts that it stops at the end of the first pulse in which the
/// side was told that thing, and names those events. Returns the day and
/// pulse of the stop, or `None` if the game ended with the thing never told.
fn stops_on(name: &str, text: &str, player: Side, thing: &str) -> Option<(u64, String)> {
    let (mut s, script) = session(scenario(name, text), player);
    let mut seen: Vec<(u64, String)> = Vec::new();
    loop {
        if s.open_day().is_none() {
            give_day(&mut s, player, &script);
        }
        let answer = call(
            &mut s,
            player,
            "advance",
            json!({"pulses": 3, "stop_on": [thing]}),
        );
        check("advance.schema.json", &answer);
        for pulse in answer["pulses_resolved"].as_array().unwrap() {
            seen.push((
                pulse["day"].as_u64().unwrap(),
                pulse["pulse"].as_str().unwrap().to_string(),
            ));
        }
        let (day, pulse) = seen.last().cloned().unwrap();
        // No pulse before the last had the thing; the last has it if and
        // only if the call stopped on it.
        for (d, p) in &seen[..seen.len() - 1] {
            let mut events = events_of(&mut s, player, *d, p);
            if p == "afternoon" {
                events.extend(events_of(&mut s, player, *d, "end"));
            }
            assert!(
                !events.iter().any(|e| is_thing(thing, e)),
                "{thing} told on day {d} {p} with no stop"
            );
        }
        let mut last = events_of(&mut s, player, day, &pulse);
        if pulse == "afternoon" {
            last.extend(events_of(&mut s, player, day, "end"));
        }
        let told: Vec<u64> = last
            .iter()
            .filter(|e| is_thing(thing, e))
            .map(|e| e["number"].as_u64().unwrap())
            .collect();
        // A stop on what it was told; on the game's last pulse the game's end
        // is the reason given, and what it was told is still named.
        let stopped_on_it = answer["stopped"] == "told"
            || (answer["stopped"] == "game_over" && answer["told"] != json!([]));
        if stopped_on_it {
            assert_eq!(answer["told"][0]["thing"], thing, "{answer}");
            assert_eq!(answer["told"][0]["events"], json!(told), "{answer}");
            if answer["stopped"] == "told" {
                assert!(answer["summary"].as_str().unwrap().contains(thing));
            }
            return Some((day, pulse));
        }
        assert!(
            told.is_empty(),
            "{thing} told in day {day} {pulse} and no stop: {answer}"
        );
        if answer["game_over"] == true {
            return None;
        }
        seen.clear();
        seen.push((day, pulse));
    }
}

#[test]
fn stop_on_contact_stops_at_the_end_of_the_pulse_of_the_first_new_contact() {
    let text = with_key_on(&live_text("05_eastern_solomons.toml"));
    let (mut s, script) = session(scenario("05_eastern_solomons.toml", &text), ALLIED);
    give_day(&mut s, ALLIED, &script);
    let answer = call(
        &mut s,
        ALLIED,
        "advance",
        json!({"pulses": 3, "stop_on": ["contact"]}),
    );
    check("advance.schema.json", &answer);
    // The Eastern Solomons is one day long: a contact first made in the
    // afternoon stops the call at the game's end, which is the reason given;
    // what the side was told is named either way.
    assert!(
        answer["stopped"] == "told" || answer["stopped"] == "game_over",
        "{answer}"
    );
    assert_eq!(answer["told"][0]["thing"], "contact", "{answer}");
    let last = answer["pulses_resolved"]
        .as_array()
        .unwrap()
        .last()
        .cloned()
        .unwrap();
    let numbers = answer["told"][0]["events"].as_array().unwrap().clone();
    assert!(!numbers.is_empty());
    for n in &numbers {
        let explained = call(&mut s, ALLIED, "explain", json!({"event": n}));
        let node = &explained["nodes"][0];
        assert_eq!(node["event_kind"], "report_received");
        assert!(
            node["text"].as_str().unwrap().contains(": new contact:"),
            "{node}"
        );
        assert_eq!(node["day"], last["day"]);
        assert_eq!(node["pulse"], last["pulse"]);
    }
    // The same through the general walk, which also checks the earlier
    // pulses had no new contact.
    assert!(stops_on("05_eastern_solomons.toml", &text, ALLIED, "contact").is_some());
}

#[test]
fn stop_on_attack_strike_and_loss_stop_where_the_side_was_told() {
    let es = with_key_on(&live_text("05_eastern_solomons.toml"));
    let savo = with_key_on(&live_text("04_savo_island.toml"));
    // The Eastern Solomons' are in its afternoon, the game's last pulse;
    // Savo Island's are in the night, so the clock stops at 0600.
    for (name, text, side, thing, pulse) in [
        (
            "05_eastern_solomons.toml",
            &es,
            ALLIED,
            "strike",
            "afternoon",
        ),
        (
            "05_eastern_solomons.toml",
            &es,
            Side::Japanese,
            "strike",
            "afternoon",
        ),
        (
            "05_eastern_solomons.toml",
            &es,
            ALLIED,
            "attack",
            "afternoon",
        ),
        (
            "05_eastern_solomons.toml",
            &es,
            Side::Japanese,
            "loss",
            "afternoon",
        ),
        ("04_savo_island.toml", &savo, ALLIED, "contact", "night"),
        ("04_savo_island.toml", &savo, ALLIED, "attack", "night"),
        (
            "04_savo_island.toml",
            &savo,
            Side::Japanese,
            "attack",
            "night",
        ),
        ("04_savo_island.toml", &savo, ALLIED, "loss", "night"),
    ] {
        assert_eq!(
            stops_on(name, text, side, thing),
            Some((1, pulse.to_string())),
            "{name}: the {side:?} side on {thing}"
        );
    }
    // The Allied side's carrier is set on fire among her aircraft in the
    // Eastern Solomons' afternoon (decision 0212): a loss, told.
    assert_eq!(
        stops_on("05_eastern_solomons.toml", &es, ALLIED, "loss"),
        Some((1, "afternoon".to_string()))
    );
    // Where the thing is not told, the game ends with no stop on it.
    assert_eq!(
        stops_on("04_savo_island.toml", &savo, Side::Japanese, "loss"),
        None
    );
}

#[test]
fn stop_on_landing_stops_where_men_landed() {
    let text = with_key_on(&live_text("21_landing.toml"));
    assert!(stops_on("21_landing.toml", &text, ALLIED, "landing").is_some());
}

#[test]
fn nothing_told_is_no_stop() {
    // Scenario 1's transit with the key on: no contact, attack, loss, strike
    // or landing in three days, so advance never stops on what it was told.
    let (mut s, _) = session(
        scenario("78_orders_inside_a_day.toml", &case_c_text()),
        ALLIED,
    );
    call(&mut s, ALLIED, "issue_orders", route_and_cruise());
    let answer = call(
        &mut s,
        ALLIED,
        "advance",
        json!({"pulses": 4, "stop_on": ["contact", "attack", "loss", "strike", "landing"]}),
    );
    assert_eq!(answer["stopped"], "pulses", "{answer}");
    assert_eq!(answer["told"], json!([]));
    assert_eq!(answer["clock"]["at"], "0600");
    assert_eq!(answer["clock"]["day"], 2);
    let end = call(
        &mut s,
        ALLIED,
        "advance",
        json!({"pulses": 20, "stop_on": ["contact", "attack", "loss", "strike", "landing"]}),
    );
    assert_eq!(end["stopped"], "game_over", "{end}");
    // And for each thing alone, on games where it is never told.
    assert_eq!(
        stops_on(
            "78_orders_inside_a_day.toml",
            &case_c_text(),
            ALLIED,
            "attack"
        ),
        None
    );
    let savo = with_key_on(&live_text("04_savo_island.toml"));
    for thing in ["strike", "landing"] {
        assert_eq!(stops_on("04_savo_island.toml", &savo, ALLIED, thing), None);
    }
}

// ---------------------------------------------------------------------
// 4. The refusals.
// ---------------------------------------------------------------------

#[test]
fn the_refusals_inside_a_day() {
    let (mut s, _) = session(
        scenario("78_orders_inside_a_day.toml", &case_c_text()),
        ALLIED,
    );
    call(&mut s, ALLIED, "issue_orders", route_and_cruise());
    call(&mut s, ALLIED, "advance", json!({"pulses": 1}));
    // A kind given at dusk is kept for the next dusk, not refused (decision
    // 0219); the clear below removes it again.
    let given = call(
        &mut s,
        ALLIED,
        "issue_orders",
        json!({"orders": [{"unit": "task_force:1", "order": {"kind": "set_mission", "mission": "bombardment"}}]}),
    );
    check("issue_orders.schema.json", &given);
    assert_eq!(given["results"][0]["status"], "queued");
    assert_eq!(given["results"][0]["kept"], true);
    assert_eq!(given["kept"], 1);
    // The briefing at 0600.
    let e = refused(&mut s, ALLIED, "get_briefing", json!({}));
    assert!(e.message.contains("the briefing is made at dusk"), "{e}");
    assert!(e.message.contains("0600 of day 1"), "{e}");
    assert!(e.message.contains("events"), "{e}");
    // An order queued and then cleared is not handed in: the game is case C.
    let queued = call(&mut s, ALLIED, "issue_orders", eight_knots());
    assert_eq!(queued["queued"], 1);
    let cleared = call(
        &mut s,
        ALLIED,
        "issue_orders",
        json!({"orders": [], "clear": true}),
    );
    assert_eq!(cleared["cleared"], true);
    assert_eq!(cleared["queued"], 0);
    call(&mut s, ALLIED, "advance", json!({"pulses": 20}));
    let c = run_by_days(scenario("78_orders_inside_a_day.toml", &case_c_text()));
    assert_eq!(s.day_hashes(), c.as_slice());
    assert!(s.handed_in_inside_day().is_empty());
}

#[test]
fn with_the_key_off_advance_is_not_a_tool_and_events_is() {
    let (mut s, _) = session(
        scenario("01_transit.toml", &live_text("01_transit.toml")),
        ALLIED,
    );
    let tools = s.tools(ALLIED);
    assert!(!tools.contains(&"advance"));
    assert!(tools.contains(&"events"));
    assert_eq!(tools.len(), 31);
    let e = refused(&mut s, ALLIED, "advance", json!({"pulses": 1}));
    assert!(e.message.contains("is not a tool"), "{e}");
    assert!(!e.allowed.contains(&"advance".to_string()));
    let answer = call(&mut s, ALLIED, "events", json!({}));
    check("events.schema.json", &answer);
    // With the key on: one more tool, last.
    let (on, _) = session(
        scenario("78_orders_inside_a_day.toml", &case_c_text()),
        ALLIED,
    );
    let tools_on = on.tools(ALLIED);
    assert_eq!(tools_on.len(), 32);
    assert_eq!(tools_on.last(), Some(&"advance"));
    assert_eq!(&tools_on[..31], tools.as_slice());
    // With two players, as for run, no advance.
    let both = scenario("78_orders_inside_a_day.toml", &case_c_text());
    let name = both.name.clone();
    let two = Session::new(both.world, &name, 3, Seat::Player, Seat::Player)
        .with_orders_inside_a_day(true, Default::default());
    assert!(!two.tools(ALLIED).contains(&"advance"));
}

// ---------------------------------------------------------------------
// 5. events.
// ---------------------------------------------------------------------

#[test]
fn events_gives_each_event_as_explain_does() {
    let text = with_key_on(&live_text("05_eastern_solomons.toml"));
    let (mut s, script) = session(scenario("05_eastern_solomons.toml", &text), ALLIED);
    // Before day 1: not begun.
    let before = call(&mut s, ALLIED, "events", json!({}));
    check("events.schema.json", &before);
    assert_eq!(before["day"], 1);
    assert_eq!(before["begun"], false);
    assert_eq!(before["events"], json!([]));
    assert!(before["summary"]
        .as_str()
        .unwrap()
        .contains("has not begun"));
    give_day(&mut s, ALLIED, &script);
    call(&mut s, ALLIED, "advance", json!({"pulses": 2}));
    // At 1200: the day so far, by default.
    let so_far = call(&mut s, ALLIED, "events", json!({}));
    check("events.schema.json", &so_far);
    assert_eq!(so_far["day"], 1);
    assert_eq!(so_far["begun"], true);
    assert!(so_far["summary"]
        .as_str()
        .unwrap()
        .contains("1200 of day 1"));
    let list = so_far["events"].as_array().unwrap();
    assert!(!list.is_empty());
    for e in list {
        assert_ne!(e["pulse"], "afternoon", "{e}");
        let explained = call(&mut s, ALLIED, "explain", json!({"event": e["number"]}));
        let node = &explained["nodes"][0];
        for (field, of_node) in [
            ("number", "id"),
            ("day", "day"),
            ("pulse", "pulse"),
            ("event_kind", "event_kind"),
            ("text", "text"),
            ("entities", "entities"),
        ] {
            assert_eq!(e[field], node[of_node], "{field} of {e}");
        }
    }
    // Every event of the side so far, in order.
    let numbers: Vec<u64> = list.iter().map(|e| e["number"].as_u64().unwrap()).collect();
    assert_eq!(numbers, (1..=numbers.len() as u64).collect::<Vec<_>>());
    // The pulse filter: the parts together are the whole.
    let mut parts: Vec<u64> = Vec::new();
    for pulse in ["night", "morning", "afternoon", "end"] {
        let part = events_of(&mut s, ALLIED, 1, pulse);
        for e in &part {
            match pulse {
                "end" => assert_eq!(e["pulse"], Value::Null),
                p => assert_eq!(e["pulse"], p),
            }
        }
        if pulse == "afternoon" {
            assert!(part.is_empty());
        }
        parts.extend(part.iter().map(|e| e["number"].as_u64().unwrap()));
    }
    parts.sort_unstable();
    assert_eq!(parts, numbers);
    // A day not begun.
    let later = call(&mut s, ALLIED, "events", json!({"day": 2}));
    assert_eq!(later["begun"], false);
    assert_eq!(later["count"], 0);
    // Bad arguments.
    assert!(
        refused(&mut s, ALLIED, "events", json!({"pulse": "evening"}))
            .allowed
            .contains(&"end".to_string())
    );
    refused(&mut s, ALLIED, "events", json!({"day": 0}));
}

// ---------------------------------------------------------------------
// 6. A command at 0600, and the other tools there.
// ---------------------------------------------------------------------

/// Scenario 36 with the key on, the Allied seat a player's, at 0600 of day
/// 1 with the script's orders of day 1 given.
fn convoy_at_0600() -> (Session, BTreeMap<u32, Vec<OrderRequest>>) {
    let text = with_key_on(&live_text("36_one_convoy.toml"));
    let (mut s, script) = session(scenario("36_one_convoy.toml", &text), ALLIED);
    give_day(&mut s, ALLIED, &script);
    let at = call(&mut s, ALLIED, "advance", json!({"pulses": 1}));
    assert_eq!(at["clock"]["at"], "0600");
    (s, script)
}

#[test]
fn a_command_queued_at_0600_is_taken_in_at_the_next_dusk_and_not_before() {
    let (mut s, script) = convoy_at_0600();
    let answer = call(
        &mut s,
        ALLIED,
        "set_interrupt",
        json!({"base": 2, "below_days": 3}),
    );
    check("command.schema.json", &answer);
    assert_eq!(answer["status"], "queued", "{answer}");
    assert_eq!(answer["for_day"], 2, "{answer}");
    let note = answer["inside_day"].as_str().unwrap();
    assert!(
        note.contains("queued for dusk at the end of day 1"),
        "{note}"
    );
    assert!(note.contains("taken in at the start of day 2"), "{note}");
    assert!(
        note.contains("the staff is not asked again inside the day"),
        "{note}"
    );
    assert!(
        note.contains("checked again when it is taken in at dusk"),
        "{note}"
    );
    // Day 1 is the day it would have been without the command: the staff's
    // hand-in of day 1 and the hash of day 1 are those of the game without it.
    let (mut without, _) = convoy_at_0600();
    for game in [&mut s, &mut without] {
        give_day_after_dusk(game, &script);
    }
    assert_eq!(s.day_hashes()[0], without.day_hashes()[0]);
    assert_eq!(
        s.staff_handed_in().get(&1),
        without.staff_handed_in().get(&1)
    );
    assert_eq!(
        s.commands_handed_in().get(&1),
        without.commands_handed_in().get(&1)
    );
    // It is handed in at the dusk of day 2, the day the answer gave.
    let handed = s.commands_handed_in().get(&2).cloned().unwrap_or_default();
    assert_eq!(handed.len(), 1, "{handed:?}");
    let taken: Vec<Value> = events_of(&mut s, ALLIED, 2, "end")
        .into_iter()
        .filter(|e| e["event_kind"] == "command_taken")
        .collect();
    assert_eq!(taken.len(), 1, "{taken:?}");
    assert!(events_of(&mut s, ALLIED, 1, "morning")
        .iter()
        .chain(events_of(&mut s, ALLIED, 1, "afternoon").iter())
        .all(|e| e["event_kind"] != "command_taken"));
    assert_ne!(s.day_hashes()[1], without.day_hashes()[1]);
}

/// Resolves the rest of day 1, then gives day 2's orders and resolves day 2.
fn give_day_after_dusk(s: &mut Session, script: &BTreeMap<u32, Vec<OrderRequest>>) {
    call(s, ALLIED, "advance", json!({"pulses": 2}));
    assert_eq!(s.day(), 1);
    give_day(s, ALLIED, script);
    call(s, ALLIED, "end_day", json!({}));
    assert_eq!(s.day(), 2);
}

#[test]
fn the_staff_tools_at_0600_answer_on_the_state_as_it_stands_and_say_so() {
    let (mut s, _) = convoy_at_0600();
    for (tool, args, schema) in [
        ("command_state", json!({}), "command_state.schema.json"),
        ("plans", json!({}), "plans.schema.json"),
        ("staff_orders", json!({}), "staff_orders.schema.json"),
        ("propose", json!({"intent": 1}), "propose.schema.json"),
        ("order", json!({"intent": 1}), "plan_order.schema.json"),
        (
            "preview",
            json!({"command": "set_limit", "limit": "reserve", "base": 1, "supply_t": 900, "fuel_t": 0}),
            "preview.schema.json",
        ),
    ] {
        let answer = call(&mut s, ALLIED, tool, args);
        check(schema, &answer);
        let note = answer["inside_day"]
            .as_str()
            .unwrap_or_else(|| panic!("{tool}: {answer}"));
        assert!(
            note.contains("Worked at 0600 of day 1 on the state as it stands"),
            "{note}"
        );
        if let Some(day) = answer.get("for_day") {
            assert_eq!(day, 2, "{tool}");
        }
    }
    // The read tools answer at 0600 with no note, from the view as it is.
    for tool in ["list_forces", "list_bases", "list_contacts", "report"] {
        let answer = call(&mut s, ALLIED, tool, json!({}));
        assert!(answer.get("inside_day").is_none(), "{tool}");
    }
    let report = call(&mut s, ALLIED, "report", json!({}));
    assert_eq!(report["day"], 1);
    assert!(report["summary"].as_str().unwrap().starts_with("Day 1."));
    // At dusk there is no note.
    call(&mut s, ALLIED, "advance", json!({"pulses": 2}));
    let dusk = call(&mut s, ALLIED, "staff_orders", json!({}));
    assert!(dusk.get("inside_day").is_none());
    assert_eq!(dusk["for_day"], 2);
}

#[test]
fn a_preview_is_void_after_a_pulse() {
    let text = with_key_on(&live_text("36_one_convoy.toml"));
    let (mut s, script) = session(scenario("36_one_convoy.toml", &text), ALLIED);
    give_day(&mut s, ALLIED, &script);
    call(&mut s, ALLIED, "advance", json!({"pulses": 1}));
    let shown = call(
        &mut s,
        ALLIED,
        "preview",
        json!({"command": "set_limit", "limit": "reserve", "base": 1, "supply_t": 900, "fuel_t": 0}),
    );
    let number = shown["preview"].clone();
    call(&mut s, ALLIED, "advance", json!({"pulses": 1}));
    let approve = call(&mut s, ALLIED, "approve", json!({"preview": number}));
    assert_eq!(approve["status"], "refused", "{approve}");
    assert!(
        approve["summary"]
            .as_str()
            .unwrap()
            .contains("a pulse was resolved after it")
            || approve
                .to_string()
                .contains("a pulse was resolved after it")
    );
}

#[test]
fn measure_says_at_0600_and_1200_that_a_passage_is_worked_from_the_night() {
    let (mut s, _) = session(
        scenario("78_orders_inside_a_day.toml", &case_c_text()),
        ALLIED,
    );
    let ask = json!({"question": "passage", "task_force": "task_force:1", "to": {"q": 8, "r": 3}});
    let dusk = call(&mut s, ALLIED, "measure", ask.clone());
    check("measure.schema.json", &dusk);
    assert!(dusk.get("inside_day").is_none());
    call(&mut s, ALLIED, "issue_orders", route_and_cruise());
    for (pulses, hours) in [(1, "12 hours"), (1, "18 hours")] {
        call(&mut s, ALLIED, "advance", json!({"pulses": pulses}));
        let answer = call(&mut s, ALLIED, "measure", ask.clone());
        check("measure.schema.json", &answer);
        let note = answer["inside_day"].as_str().unwrap();
        assert!(note.contains("as if the force left at 1800"), "{note}");
        assert!(note.contains(hours), "{note}");
        // A distance needs no note.
        let distance = call(
            &mut s,
            ALLIED,
            "measure",
            json!({"question": "air_distance", "from": "task_force:1", "to": {"q": 8, "r": 3}}),
        );
        assert!(distance.get("inside_day").is_none());
    }
    call(&mut s, ALLIED, "advance", json!({"pulses": 1}));
    let again = call(&mut s, ALLIED, "measure", ask);
    assert!(again.get("inside_day").is_none());
}

#[test]
fn end_day_and_run_inside_a_day_first_resolve_the_rest_of_it() {
    let text = case_c_text();
    let (mut s, _) = session(scenario("78_orders_inside_a_day.toml", &text), ALLIED);
    call(&mut s, ALLIED, "issue_orders", route_and_cruise());
    call(&mut s, ALLIED, "advance", json!({"pulses": 1}));
    let ended = call(&mut s, ALLIED, "end_day", json!({}));
    assert_eq!(ended["day"], 1);
    call(&mut s, ALLIED, "advance", json!({"pulses": 2}));
    let run = call(&mut s, ALLIED, "run", json!({"days": 5}));
    check("run.schema.json", &run);
    assert_eq!(run["days_resolved"]["first"], 2);
    assert_eq!(run["days_resolved"]["count"], 2);
    assert_eq!(run["stopped"], "game_over");
    let by_days = run_by_days(scenario("78_orders_inside_a_day.toml", &text));
    assert_eq!(s.day_hashes(), by_days.as_slice());
}

// ---------------------------------------------------------------------
// The words of the read tools at 0600: the night just resolved.
// ---------------------------------------------------------------------

/// The land units' `fed_last_night` of the side's `list_forces`.
fn fed_of(s: &mut Session, side: Side) -> Vec<Value> {
    let answer = call(s, side, "list_forces", json!({}));
    answer["land_units"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|u| json!([u["land_unit"]["handle"], u["fed_last_night"]]))
        .collect()
}

#[test]
fn at_0600_a_land_units_fed_last_night_is_of_the_night_just_resolved() {
    let text = with_key_on(&live_text("23_fight_ashore.toml"));
    let (mut s, script) = session(scenario("23_fight_ashore.toml", &text), ALLIED);
    let before = fed_of(&mut s, ALLIED);
    assert!(!before.is_empty(), "the scenario has land units");
    give_day(&mut s, ALLIED, &script);
    call(&mut s, ALLIED, "advance", json!({"pulses": 1}));
    let at_0600 = fed_of(&mut s, ALLIED);
    call(&mut s, ALLIED, "advance", json!({"pulses": 2}));
    let at_dusk = fed_of(&mut s, ALLIED);
    // The night of day 1 is the last night at 0600 of day 1 and at its dusk.
    assert_eq!(at_0600, at_dusk);
    // And it is not the night before day 1, which had none.
    assert_ne!(at_0600, before, "{before:?}");
}

/// The `as_of` of each force line of the area seat's `list_forces`.
fn as_of_lines(s: &mut Session, side: Side) -> Vec<Value> {
    let answer = call(s, side, "list_forces", json!({}));
    answer["task_forces"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|f| f.get("as_of").cloned().unwrap_or(Value::Null))
        .collect()
}

#[test]
fn at_0600_the_area_seat_reads_as_of_against_the_night_just_resolved() {
    let text = with_key_on(&live_text("63_a_superiors_knowledge.toml"));
    let (mut s, script) = session(scenario("63_a_superiors_knowledge.toml", &text), ALLIED);
    let at_dusk = as_of_lines(&mut s, ALLIED);
    assert!(!at_dusk.is_empty());
    assert!(at_dusk.iter().all(Value::is_null), "{at_dusk:?}");
    give_day(&mut s, ALLIED, &script);
    call(&mut s, ALLIED, "advance", json!({"pulses": 1}));
    let at_0600 = as_of_lines(&mut s, ALLIED);
    // No line is marked as of the night just resolved, which is current.
    assert!(
        at_0600.iter().all(|a| a != "the night of day 1"),
        "{at_0600:?}"
    );
    // A force not reported in the night is held as of before it, and says so.
    assert!(at_0600.iter().any(|a| a == "the start"), "{at_0600:?}");
}
