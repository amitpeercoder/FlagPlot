//! The briefing's two lists of arrivals (decision 0215, as amended on
//! review): `arrivals.came_today` and `arrivals.comes_tomorrow`, made from the
//! side's own view, with what is explicit of what the staff did with each
//! unit. No hand calculation: nothing is computed.
//!
//! The scenario is 72 with its commands cut off, six days long, and:
//! * task force 1 (two transports) enters on day 3 and may first be used on
//!   day 3 (`enters = 3`, `available_from = 3`): one line, not two;
//! * land unit 1 enters on day 3 (at base 1);
//! * land unit 2 may first be used on day 4, with no entry;
//! * air group 1 (at base 1, 6 aircraft) is due 2 aircraft on day 1, 2 on day
//!   3 and 3 on day 5.
//!
//! So day 3 has three arrivals together, the evening of day 1 has an arrival
//! that is not told (day 1 has none), and a briefing is "day D" in its heading
//! with the day just played D - 1: the briefing of day 3 is the evening of
//! day 2, and says what comes on day 3.

mod support;

use serde_json::{json, Value};
use sim_core::state::Side;
use support::check::valid;
use support::play::{play_day, session_of};
use support::validator::Schemas;
use support::{load_text, scenario_text};

/// `text` with `line` added after the `side` line of the table `kind` whose
/// `id` is `id`.
fn add_to_table(text: &str, kind: &str, id: u32, line: &str) -> String {
    let head = format!("[[{kind}]]\nid = {id}\n");
    let at = text.find(&head).unwrap_or_else(|| panic!("no {kind} {id}"));
    let side = at + text[at..].find("side = ").expect("a side line");
    let end = side + text[side..].find('\n').expect("a line end") + 1;
    format!("{}{line}\n{}", &text[..end], &text[end..])
}

fn call(session: &mut sim_agent::Session, side: Side, tool: &str, args: Value) -> Value {
    session
        .call(side, tool, &args)
        .unwrap_or_else(|e| panic!("{tool} was refused: {e}"))
}

fn variant(japanese_enters: bool) -> String {
    let mut text = scenario_text("72_reconnaissance.toml");
    text = text
        .split("# The Allied commander's commands")
        .next()
        .expect("the commands")
        .to_string();
    text = text.replace("days = 4", "days = 6");
    text = add_to_table(&text, "task_force", 1, "enters = 3\navailable_from = 3");
    text = add_to_table(&text, "land_unit", 1, "enters = 3");
    text = add_to_table(&text, "land_unit", 2, "available_from = 4");
    if japanese_enters {
        text = add_to_table(&text, "land_unit", 10, "enters = 3");
    }
    text.push_str(
        "\n[[arrival]]\nday = 1\nair_group = 1\naircraft = 2\n\
         \n[[arrival]]\nday = 3\nair_group = 1\naircraft = 2\n\
         \n[[arrival]]\nday = 5\nair_group = 1\naircraft = 3\n",
    );
    text
}

fn session(japanese_enters: bool) -> sim_agent::Session {
    let scenario = load_text(&variant(japanese_enters), "72_reconnaissance.toml", 1942);
    session_of(scenario, &[Side::Allied, Side::Japanese])
        .0
        .with_staffs(sim_staff::staffs())
}

fn end_day(s: &mut sim_agent::Session) {
    play_day(s, Side::Japanese, &[]);
    play_day(s, Side::Allied, &[]);
}

fn briefing(s: &mut sim_agent::Session, side: Side) -> Value {
    call(s, side, "get_briefing", json!({}))
}

fn lines(briefing: &Value, list: &str) -> Vec<Value> {
    briefing["arrivals"]
        .get(list)
        .map(|l| l.as_array().expect("a list").clone())
        .unwrap_or_default()
}

fn handles(lines: &[Value]) -> Vec<String> {
    lines
        .iter()
        .map(|l| l["unit"]["handle"].as_str().expect("a handle").to_string())
        .collect()
}

fn line_of<'a>(lines: &'a [Value], handle: &str) -> &'a Value {
    lines
        .iter()
        .find(|l| l["unit"]["handle"] == handle)
        .unwrap_or_else(|| panic!("no line for {handle} in {lines:?}"))
}

fn text_lines(s: &mut sim_agent::Session, side: Side, start: &str) -> Vec<String> {
    let answer = call(s, side, "get_briefing", json!({"text": true}));
    answer["text"]
        .as_str()
        .expect("text")
        .lines()
        .filter(|l| l.trim_start_matches("- ").starts_with(start))
        .map(str::to_string)
        .collect()
}

#[test]
fn day_one_has_none_though_an_arrival_comes_on_it() {
    let mut s = session(false);
    // Before day 1: what comes on day 1 is what is there at the start.
    let b = briefing(&mut s, Side::Allied);
    assert!(lines(&b, "came_today").is_empty() && lines(&b, "comes_tomorrow").is_empty());
    assert!(
        b["arrivals"].get("came_today").is_none(),
        "left out when none"
    );
    assert!(b["arrivals"].get("comes_tomorrow").is_none());
    // The evening of day 1: air group 1 received its aircraft on day 1 (it has
    // 8 now), and the list does not tell it.
    end_day(&mut s);
    let b = briefing(&mut s, Side::Allied);
    assert_eq!(b["heading"]["day"], 2);
    let group = b["air_groups"]
        .as_array()
        .expect("groups")
        .iter()
        .find(|g| g["air_group"]["handle"] == "air_group:1")
        .expect("the group")
        .clone();
    assert_eq!(group["ready"], 8, "the arrival of day 1 was delivered");
    assert!(lines(&b, "came_today").is_empty(), "{b}");
    // The evening of day 2 says what comes on day 3, not before.
    end_day(&mut s);
    let b = briefing(&mut s, Side::Allied);
    assert!(lines(&b, "came_today").is_empty(), "{b}");
    assert_eq!(lines(&b, "comes_tomorrow").len(), 3, "{b}");
}

#[test]
fn what_comes_tomorrow_is_told_the_evening_before_and_what_came_the_evening_of_its_day() {
    let mut s = session(false);
    end_day(&mut s);
    end_day(&mut s);

    // Briefing of day 3: three arrivals on one day, a force, a land unit and
    // aircraft to a group. The force is also first usable on day 3, and has
    // one line.
    let b = briefing(&mut s, Side::Allied);
    let tomorrow = lines(&b, "comes_tomorrow");
    assert_eq!(
        handles(&tomorrow),
        ["task_force:1", "land_unit:1", "air_group:1"]
    );
    let force = line_of(&tomorrow, "task_force:1");
    assert_eq!(force["kind"], "task_force");
    assert_eq!(force["how"], json!(["enter", "usable"]));
    assert_eq!(force["place"]["q"], 0);
    assert_eq!(force["ships"], json!([{"kind": "transport", "ships": 2}]));
    assert!(force.get("staff").is_none(), "no staff part tomorrow");
    let unit = line_of(&tomorrow, "land_unit:1");
    assert_eq!(unit["kind"], "land_unit");
    assert_eq!(unit["how"], json!(["enter"]));
    assert_eq!(unit["at"]["handle"], "base:1");
    assert_eq!(unit["men"], 600);
    let group = line_of(&tomorrow, "air_group:1");
    assert_eq!(group["kind"], "air_group");
    assert_eq!(group["how"], json!(["aircraft"]));
    assert_eq!(group["aircraft_received"], 2);
    assert_eq!(group["aircraft"], 8);
    assert_eq!(group["mission"], "rest");
    assert_eq!(group["at"]["handle"], "base:1");
    assert!(lines(&b, "came_today").is_empty());
    // Land unit 2 has its day to be used on day 4: not listed yet.
    assert!(!handles(&tomorrow).contains(&"land_unit:2".to_string()));

    // Briefing of day 4: all three came, each with one line and, with no
    // intent and no order, the staff gave none an order.
    end_day(&mut s);
    let b = briefing(&mut s, Side::Allied);
    let came = lines(&b, "came_today");
    assert_eq!(
        handles(&came),
        ["task_force:1", "land_unit:1", "air_group:1"]
    );
    for line in &came {
        assert_eq!(line["staff"], json!({"no_order_today": true}), "{line}");
    }
    assert_eq!(
        line_of(&came, "task_force:1")["how"],
        json!(["enter", "usable"])
    );
    assert_eq!(line_of(&came, "land_unit:1")["place"]["q"], 0);
    assert_eq!(line_of(&came, "air_group:1")["aircraft_received"], 2);
    assert_eq!(line_of(&came, "air_group:1")["aircraft"], 10);
    // Land unit 2 comes into use tomorrow, with no entry.
    let tomorrow = lines(&b, "comes_tomorrow");
    assert_eq!(handles(&tomorrow), ["land_unit:2"]);
    assert_eq!(tomorrow[0]["how"], json!(["usable"]));
    assert_eq!(tomorrow[0]["men"], 600);

    // Briefing of day 5: land unit 2 came, on its day alone. Aircraft come
    // on day 5.
    end_day(&mut s);
    let b = briefing(&mut s, Side::Allied);
    let came = lines(&b, "came_today");
    assert_eq!(handles(&came), ["land_unit:2"]);
    assert_eq!(came[0]["how"], json!(["usable"]));
    assert_eq!(came[0]["staff"], json!({"no_order_today": true}));
    let tomorrow = lines(&b, "comes_tomorrow");
    assert_eq!(handles(&tomorrow), ["air_group:1"]);
    assert_eq!(tomorrow[0]["aircraft_received"], 3);

    // Briefing of day 6, then nothing more comes.
    end_day(&mut s);
    let b = briefing(&mut s, Side::Allied);
    let came = lines(&b, "came_today");
    assert_eq!(handles(&came), ["air_group:1"]);
    assert_eq!(came[0]["aircraft_received"], 3);
    assert_eq!(came[0]["aircraft"], 13);
    assert!(lines(&b, "comes_tomorrow").is_empty());
}

#[test]
fn the_text_says_each_line_in_words() {
    let mut s = session(false);
    end_day(&mut s);
    end_day(&mut s);
    let tomorrow = text_lines(&mut s, Side::Allied, "Comes tomorrow");
    assert_eq!(tomorrow.len(), 3, "{tomorrow:?}");
    assert!(
        tomorrow[0].contains("task_force:1") && tomorrow[0].contains("(enters, may first be used)")
    );
    assert!(tomorrow[0].contains("2 transport"), "{}", tomorrow[0]);
    assert!(tomorrow[1].contains("600 men"), "{}", tomorrow[1]);
    assert!(
        tomorrow[2].contains("receives 2 aircraft"),
        "{}",
        tomorrow[2]
    );
    end_day(&mut s);
    let came = text_lines(&mut s, Side::Allied, "Came today");
    assert_eq!(came.len(), 3, "{came:?}");
    assert!(
        came[0].contains("(entered, may first be used)"),
        "{}",
        came[0]
    );
    assert!(came[2].contains("received 2 aircraft"), "{}", came[2]);
    for line in &came {
        assert!(
            line.ends_with("the staff has given it no order today"),
            "{line}"
        );
        assert!(
            !line.contains("intent"),
            "the player's word is order: {line}"
        );
    }
}

#[test]
fn the_staff_part_says_only_what_is_explicit() {
    let mut s = session(false);
    end_day(&mut s);
    end_day(&mut s);
    // At the briefing of day 3 the commander states a cover order for base 1
    // and names the group, land unit 1 and force 1 in its forces, gives leave
    // to launch and holds task force 1 (named in the limit as well: the hold
    // is told).
    for (tool, args) in [
        ("set_intent", json!({"intent_kind": "cover", "base": 1})),
        (
            "set_limit",
            json!({"limit": "forces", "intent": 1, "air_groups": [1], "land_units": [1], "task_forces": [1]}),
        ),
        ("hold", json!({"task_force": 1})),
        ("set_leave", json!({"leave": "launch", "given": true})),
    ] {
        call(&mut s, Side::Allied, tool, args);
    }
    end_day(&mut s);

    // Day 4's briefing: the staff gave the group an order that day for order
    // 1; the unit that enters is named in the limit; the force is held.
    let b = briefing(&mut s, Side::Allied);
    let came = lines(&b, "came_today");
    assert_eq!(
        handles(&came),
        ["task_force:1", "land_unit:1", "air_group:1"]
    );
    assert_eq!(
        line_of(&came, "air_group:1")["staff"],
        json!({"ordered": {"intent": 1, "words": "cover base:1 (Placeholder base A) from the air"}})
    );
    assert_eq!(line_of(&came, "air_group:1")["mission"], "search");
    assert_eq!(
        line_of(&came, "land_unit:1")["staff"],
        json!({"named_in_limit": {"intent": 1, "words": "cover base:1 (Placeholder base A) from the air"}})
    );
    assert_eq!(
        line_of(&came, "task_force:1")["staff"],
        json!({"held": true})
    );
    let text = text_lines(&mut s, Side::Allied, "Came today");
    assert!(text[0].ends_with("you hold it"), "{}", text[0]);
    assert!(
        text[1].ends_with("named in the forces of order 1"),
        "{}",
        text[1]
    );
    assert!(
        text[2].ends_with(
            "the staff has put it to order 1 (cover base:1 (Placeholder base A) from the air)"
        ),
        "{}",
        text[2]
    );

    // Land unit 2 comes into use on day 4; the commander orders it that day.
    let a = call(
        &mut s,
        Side::Allied,
        "issue_orders",
        json!({"orders": [{"unit": "land_unit:2", "order": {"kind": "set_posture", "posture": "attack"}}]}),
    );
    assert_eq!(a["results"][0]["status"], "queued", "{a}");
    end_day(&mut s);
    let b = briefing(&mut s, Side::Allied);
    let came = lines(&b, "came_today");
    assert_eq!(handles(&came), ["land_unit:2"]);
    assert_eq!(came[0]["staff"], json!({"commander_ordered": true}));
    let text = text_lines(&mut s, Side::Allied, "Came today");
    assert!(text[0].ends_with("under your direct order"), "{}", text[0]);

    // The group's day-5 aircraft: no order of the staff that day, but the
    // limit still names it (the staff's order of day 3 stands as the group's
    // mission, which is not an order of day 5).
    end_day(&mut s);
    let b = briefing(&mut s, Side::Allied);
    let came = lines(&b, "came_today");
    assert_eq!(handles(&came), ["air_group:1"]);
    assert_eq!(
        came[0]["staff"],
        json!({"named_in_limit": {"intent": 1, "words": "cover base:1 (Placeholder base A) from the air"}})
    );
}

#[test]
fn the_enemys_briefing_has_none_of_the_sides_lines_and_the_sides_none_of_the_enemys() {
    let mut s = session(false);
    for day in 1..=6 {
        let enemy = briefing(&mut s, Side::Japanese);
        assert!(
            enemy["arrivals"].get("came_today").is_none()
                && enemy["arrivals"].get("comes_tomorrow").is_none(),
            "day {day}: {enemy}"
        );
        for start in ["Came today", "Comes tomorrow"] {
            assert!(
                text_lines(&mut s, Side::Japanese, start).is_empty(),
                "day {day}"
            );
        }
        end_day(&mut s);
    }

    // With the Japanese garrison entering on day 3, it is told to the
    // Japanese side alone.
    let mut s = session(true);
    end_day(&mut s);
    end_day(&mut s);
    let japanese = briefing(&mut s, Side::Japanese);
    let theirs = lines(&japanese, "comes_tomorrow");
    assert_eq!(theirs.len(), 1, "{japanese}");
    assert_eq!(theirs[0]["kind"], "land_unit");
    assert_eq!(theirs[0]["men"], 150);
    let allied = briefing(&mut s, Side::Allied);
    let ours = lines(&allied, "comes_tomorrow");
    assert_eq!(ours.len(), 3, "{allied}");
    assert!(ours.iter().all(|l| l["men"] != 150), "{allied}");
    end_day(&mut s);
    let japanese = briefing(&mut s, Side::Japanese);
    let came = lines(&japanese, "came_today");
    assert_eq!(came.len(), 1, "{japanese}");
    assert_eq!(came[0]["staff"], json!({"no_order_today": true}));
    let allied = briefing(&mut s, Side::Allied);
    assert_eq!(lines(&allied, "came_today").len(), 3);
}

#[test]
fn every_briefing_with_the_lists_is_as_the_schema_says() {
    let schemas =
        Schemas::load(&std::path::Path::new(support::ROOT).join("crates/sim-agent/schemas"));
    let mut s = session(true);
    let mut told = 0;
    for day in 1..=6 {
        if day == 3 {
            for (tool, args) in [
                ("set_intent", json!({"intent_kind": "cover", "base": 1})),
                (
                    "set_limit",
                    json!({"limit": "forces", "intent": 1, "air_groups": [1], "land_units": [1], "task_forces": [1]}),
                ),
                ("hold", json!({"task_force": 1})),
                ("set_leave", json!({"leave": "launch", "given": true})),
            ] {
                call(&mut s, Side::Allied, tool, args);
            }
        }
        for side in [Side::Allied, Side::Japanese] {
            let b = briefing(&mut s, side);
            told += lines(&b, "came_today").len() + lines(&b, "comes_tomorrow").len();
            valid(&schemas, "briefing.schema.json", &b, &format!("day {day}"));
        }
        end_day(&mut s);
    }
    assert!(
        told >= 10,
        "the lists were in the answers that were checked"
    );
}

#[test]
fn what_a_force_carries_is_in_the_forces_line_and_has_none_of_its_own() {
    let mut text = scenario_text("21_landing.toml");
    text = add_to_table(&text, "task_force", 1, "enters = 3");
    let scenario = load_text(&text, "21_landing.toml", 1942);
    let mut s = session_of(scenario, &[Side::Allied]).0;
    let end = |s: &mut sim_agent::Session| {
        play_day(s, Side::Allied, &[]);
    };
    end(&mut s);
    end(&mut s);
    let tomorrow = lines(&briefing(&mut s, Side::Allied), "comes_tomorrow");
    assert_eq!(handles(&tomorrow), ["task_force:1"], "{tomorrow:?}");
    let carried = tomorrow[0]["land_units"].as_array().expect("land units");
    assert_eq!(carried.len(), 2, "{tomorrow:?}");
    assert!(carried.iter().all(|u| u["men"].as_u64().unwrap() > 0));
    let words = text_lines(&mut s, Side::Allied, "Comes tomorrow");
    assert_eq!(words.len(), 1);
    assert!(words[0].contains("aboard"), "{}", words[0]);
    end(&mut s);
    let b = briefing(&mut s, Side::Allied);
    let came = lines(&b, "came_today");
    assert_eq!(handles(&came), ["task_force:1"], "{came:?}");
    assert_eq!(came[0]["land_units"], tomorrow[0]["land_units"]);
    assert!(came[0]["staff"].is_object());
    let schemas =
        Schemas::load(&std::path::Path::new(support::ROOT).join("crates/sim-agent/schemas"));
    valid(&schemas, "briefing.schema.json", &b, "carrying force");
}
