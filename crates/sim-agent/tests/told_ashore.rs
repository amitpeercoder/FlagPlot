//! What a player commanding ashore is told of its own affairs (decision
//! 0110, items 1 to 9): why a unit made no work, the arrivals that are due
//! and what they wait for, the rule of fatigue, whether a base counts as open
//! today, what a base without fuel means for the ships there, a force with
//! nothing left to do, a landing summed up, and the cause of aircraft written
//! off. Each is read from the side's own facts and events.
//!
//! The scenarios are the ones the rules were built on: 30 (working units; the
//! Allied side's `land_unit:6` fights at D, `land_unit:7` works there and the
//! Japanese attack it), 25 (a march), 31 (a field damaged above the level that
//! refuses aircraft), 21 (a landing), 06a with objectives added, and 02 with
//! a home port added.

mod support;

use serde_json::{json, Value};
use sim_agent::{briefing, Seat, Session};
use sim_core::state::Side;
use support::check::valid;
use support::play::{play_day, session_of};
use support::{load, load_text, scenario_text, schemas};

fn session(name: &str) -> Session {
    let scenario = load(name, 1942);
    Session::new(
        scenario.world,
        name,
        scenario.days,
        Seat::Player,
        Seat::Player,
    )
}

fn orders(session: &mut Session, list: &[Value]) {
    let answer = session
        .call(Side::Allied, "issue_orders", &json!({ "orders": list }))
        .expect("issue_orders answers");
    assert_eq!(answer["refused_in_call"], 0, "{answer}");
}

fn end_day(session: &mut Session) {
    for side in [Side::Allied, Side::Japanese] {
        session
            .call(side, "end_day", &json!({}))
            .expect("end_day answers");
    }
}

/// The Allied briefing as text and as data, both checked against their
/// schemas.
fn briefing_of(session: &mut Session) -> (String, Value) {
    let text = session
        .call(Side::Allied, "get_briefing", &json!({ "text": true }))
        .expect("the text");
    let data = session
        .call(Side::Allied, "get_briefing", &json!({ "text": false }))
        .expect("the data");
    valid(&schemas(), "briefing.schema.json", &data, "briefing");
    (text["text"].as_str().expect("text").to_string(), data)
}

fn line_with<'a>(text: &'a str, start: &str) -> &'a str {
    text.lines()
        .find(|l| l.starts_with(start))
        .unwrap_or_else(|| panic!("no line starting {start:?} in\n{text}"))
}

// ---- item 1: why a unit made no work ----

#[test]
fn a_working_unit_in_a_hex_the_enemy_attacked_is_told_the_hex_was_contested() {
    let mut s = session("30_working_units.toml");
    end_day(&mut s);
    let (text, data) = briefing_of(&mut s);
    let line = line_with(&text, "- No work yesterday: land_unit:7 ");
    assert!(
        line.contains("made no work: its hex was contested by the enemy's attack [event "),
        "{line}"
    );
    let none = &data["land_units"]["no_activity_yesterday"];
    let seventh = none
        .as_array()
        .expect("a list")
        .iter()
        .find(|n| n["land_unit"]["handle"] == "land_unit:7")
        .expect("land_unit:7");
    assert_eq!(seventh["activity"], "work");
    assert_eq!(seventh["reason"], "contested");
    assert_eq!(seventh["by"], "enemy_attack");
    // A working unit that worked is not named.
    assert!(none
        .as_array()
        .unwrap()
        .iter()
        .all(|n| n["land_unit"]["handle"] != "land_unit:1"));
}

#[test]
fn the_side_that_attacked_is_told_the_contest_was_its_own_attack() {
    let mut s = session("30_working_units.toml");
    // Land unit 6 fights at D beside land unit 7, which works there.
    orders(
        &mut s,
        &[json!({"unit": "land_unit:6",
            "order": {"kind": "set_posture", "posture": "attack"}})],
    );
    end_day(&mut s);
    let (text, _) = briefing_of(&mut s);
    let line = line_with(&text, "- No work yesterday: land_unit:7 ");
    assert!(
        line.contains("its hex was contested by your own side's attack"),
        "{line}"
    );
}

#[test]
fn a_unit_set_to_work_that_marched_is_told_it_marched() {
    let mut s = session("25_march.toml");
    orders(
        &mut s,
        &[
            json!({"unit": "land_unit:1", "order": {"kind": "set_activity", "activity": "work"}}),
            json!({"unit": "land_unit:1", "order": {"kind": "march", "hex": [1, 0]}}),
        ],
    );
    end_day(&mut s);
    let (text, data) = briefing_of(&mut s);
    let line = line_with(&text, "- No work yesterday: land_unit:1 ");
    assert!(line.contains("made no work: it marched [event "), "{line}");
    assert_eq!(
        data["land_units"]["no_activity_yesterday"][0]["reason"],
        "marched"
    );
}

#[test]
fn a_unit_set_to_train_at_the_ceiling_is_told_why_no_day_counted() {
    let mut s = session("25_march.toml");
    orders(
        &mut s,
        &[json!({"unit": "land_unit:1", "order": {"kind": "set_activity", "activity": "train"}})],
    );
    end_day(&mut s);
    let (text, data) = briefing_of(&mut s);
    let line = line_with(&text, "- No training yesterday: land_unit:1 ");
    assert!(
        line.contains(
            "counted no day of training: its experience is at the level training stops at (2)"
        ),
        "{line}"
    );
    assert_eq!(
        data["land_units"]["no_activity_yesterday"][0]["reason"],
        "at_ceiling"
    );
}

// ---- item 2: arrivals that are due ----

#[test]
fn an_arrival_at_a_field_too_damaged_says_what_holds_it_and_then_that_it_is_held_past_its_day() {
    let mut s = session("31_airfield_damage.toml");
    let (text, data) = briefing_of(&mut s);
    let due = line_with(&text, "- Due day 2: air_group:3 ");
    assert!(
        due.contains("it cannot be delivered now: the airfield's damage 60 is at or above 50, the level at which it takes no aircraft"),
        "{due}"
    );
    let first = &data["arrivals"]["due"][0];
    assert_eq!(first["can_be_delivered"], false, "{first}");
    assert_eq!(first["held_past_day"], false);
    assert_eq!(first["waits_for"]["reason"], "airfield_damage");
    assert_eq!(first["waits_for"]["damage"], "60");
    assert_eq!(first["waits_for"]["refused_at"], "50");
    end_day(&mut s);
    end_day(&mut s);
    let (text, data) = briefing_of(&mut s);
    let due = line_with(&text, "- Due day 2: air_group:3 ");
    assert!(
        due.contains("held past its day by: the airfield's damage 60"),
        "{due}"
    );
    assert_eq!(data["arrivals"]["due"][0]["held_past_day"], true);
}

#[test]
fn an_empty_group_says_which_arrival_it_waits_for_and_what_holds_it() {
    let scenario = load("06_rabaul_guadalcanal.toml", 1942);
    let view = scenario.world.side_view(Side::Allied);
    let made = briefing(&view, "Full");
    let text = made.to_text();
    // Flight 300 is empty at Henderson Field with an arrival due on day 4.
    let line = line_with(&text, "- air_group:3 ");
    assert!(
        line.contains(", empty: waiting for its arrival of 11 aircraft and 11 pilots due day 4"),
        "{line}"
    );
    let group = made
        .air_groups
        .iter()
        .find(|g| g.air_group.handle == "air_group:3")
        .expect("the group");
    let waiting = group.awaiting_arrival.as_ref().expect("an arrival");
    assert_eq!((waiting.day, waiting.aircraft, waiting.pilots), (4, 11, 11));
    // The ready groups say nothing of it.
    assert!(made
        .air_groups
        .iter()
        .filter(|g| g.ready > 0)
        .all(|g| g.awaiting_arrival.is_none()));
}

// ---- item 3: the rule of fatigue ----

#[test]
fn the_land_units_part_gives_the_rule_of_fatigue_and_each_fighting_units_men_who_would_fire() {
    let mut s = session("30_working_units.toml");
    let (text, data) = briefing_of(&mut s);
    assert!(
        text.contains("\n- Fatigue: attack day +20, march day +10, rest -5 a day; above 60 a unit on attack goes back to defend; fatigue cuts the men who fire\n"),
        "{text}"
    );
    let rule = &data["land_units"]["fatigue_rule"];
    assert_eq!(rule["attack_day"], "20");
    assert_eq!(rule["rest_per_day"], "5");
    assert_eq!(rule["attack_limit"], "60");
    // Land unit 6 is a fighting unit at fatigue 0, 1000 men, fully fed.
    let unit = line_with(&text, "- land_unit:6 ");
    assert!(
        unit.contains("1000 men would fire today at its fatigue; another day of attack would make its fatigue 20;"),
        "{unit}"
    );
    // A working unit's men do not fire, and its line says nothing of it.
    let working = line_with(&text, "- land_unit:1 ");
    assert!(!working.contains("would fire today"), "{working}");
    let units = data["land_units"]["units"].as_array().expect("units");
    let sixth = units
        .iter()
        .find(|u| u["land_unit"]["handle"] == "land_unit:6")
        .unwrap();
    assert_eq!(sixth["firing_men_now"], 1000);
    assert_eq!(sixth["after_another_attack_day"]["fatigue"], "20");
    assert_eq!(sixth["after_another_attack_day"]["stops_attacking"], false);
}

#[test]
fn a_tired_unit_is_told_another_day_of_attack_would_stop_its_attacks() {
    let mut s = session("30_working_units.toml");
    // Four days of attack at 20 a day, from 0, is 80, above the limit of 60;
    // a unit at 60 would go to 80 and stop. Put land unit 6 on attack and
    // play until its fatigue is above 40.
    orders(
        &mut s,
        &[json!({"unit": "land_unit:6",
            "order": {"kind": "set_posture", "posture": "attack"}})],
    );
    for _ in 0..2 {
        end_day(&mut s);
    }
    let (text, data) = briefing_of(&mut s);
    let sixth = data["land_units"]["units"]
        .as_array()
        .unwrap()
        .iter()
        .find(|u| u["land_unit"]["handle"] == "land_unit:6")
        .unwrap()
        .clone();
    let fatigue: i64 = sixth["fatigue"].as_str().unwrap().parse().unwrap();
    let after: i64 = sixth["after_another_attack_day"]["fatigue"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(after, (fatigue + 20).min(100), "{sixth}");
    assert_eq!(
        sixth["after_another_attack_day"]["stops_attacking"],
        after > 60
    );
    let line = line_with(&text, "- land_unit:6 ");
    assert_eq!(
        line.contains("and it would then stop attacking"),
        after > 60,
        "{line}"
    );
}

// ---- item 4: whether a base counts as open today ----

const RAID: &str = "06a_base_raid.toml";

const OBJECTIVES: &str = r#"
[objectives]
minor = 1
major = 2
decisive = 3

[objectives.allied]
mission = "Keep your airfield open."

[[objectives.allied.objective]]
kind = "base_open"
base = 2
points = 1

[[objectives.allied.objective]]
kind = "base_open"
base = 1
points = 1

[objectives.japanese]
mission = "Shut the enemy airfield."

[[objectives.japanese.objective]]
kind = "base_open"
base = 2
closed = true
points = 1
"#;

fn raid_view_text(supply_in_base_2: Option<&str>) -> String {
    let text = format!("{}{OBJECTIVES}", scenario_text(RAID));
    match supply_in_base_2 {
        Some(supply) => text.replacen("supply_t = 1000", &format!("supply_t = {supply}"), 1),
        None => text,
    }
}

#[test]
fn a_base_objective_says_whether_today_would_count_and_what_is_missing() {
    let scenario = load_text(&raid_view_text(None), RAID, 1);
    let view = scenario.world.side_view(Side::Allied);
    let made = briefing(&view, "Raid");
    let text = made.to_text();
    // Base 2 is the Allied side's: supply and aircraft are there.
    assert!(
        text.contains(
            "Today, if the day ended now: would count as open: the base is open (supply "
        ),
        "{text}"
    );
    // Base 1 is the Japanese side's: it counts only once it is the Allies'.
    assert!(
        text.contains("  - Today: it counts only once the base is yours"),
        "{text}"
    );
    let json = serde_json::to_value(&made).expect("serialises");
    valid(&schemas(), "briefing.schema.json", &json, "raid briefing");
    let lines = json["objectives"]["objectives"].as_array().unwrap();
    assert_eq!(lines[0]["open_now"]["yours"], true);
    assert_eq!(lines[0]["open_now"]["open"], true);
    assert_eq!(lines[0]["open_now"]["counts"], true);
    assert_eq!(lines[1]["open_now"], json!({"yours": false}));
    // Nothing of the kind for the other kinds.
    assert!(lines.iter().all(|l| l["kind"] == "base_open"));
}

#[test]
fn a_base_with_no_supply_says_supply_is_what_is_missing() {
    let scenario = load_text(&raid_view_text(Some("0")), RAID, 1);
    let view = scenario.world.side_view(Side::Allied);
    let made = briefing(&view, "Raid");
    let text = made.to_text();
    assert!(
        text.contains("would not count as open: the base is not open: no supply at the base ("),
        "{text}"
    );
    let json = serde_json::to_value(&made).expect("serialises");
    assert_eq!(
        json["objectives"]["objectives"][0]["open_now"]["missing"],
        json!(["supply"])
    );
    // The enemy side's own objective for the same base is not shown to it.
    let other = briefing(&scenario.world.side_view(Side::Japanese), "Raid").to_text();
    assert!(!other.contains("no supply at the base"), "{other}");
}

// ---- item 5: a base without fuel ----

fn supply_run(home_port: bool, destroyer_fuel: u32) -> sim_data::Scenario {
    let name = "02_supply_run.toml";
    let mut text = scenario_text(name);
    if home_port {
        text = text.replacen(
            "mission = \"transport\"\n",
            "mission = \"transport\"\nhome_port = 2\n",
            1,
        );
    }
    text = text.replacen(
        "name = \"Placeholder destroyer 1\"\nclass = 2\nside = \"allied\"\nfuel_t = 500",
        &format!(
            "name = \"Placeholder destroyer 1\"\nclass = 2\nside = \"allied\"\nfuel_t = {destroyer_fuel}"
        ),
        1,
    );
    load_text(&text, name, 1942)
}

#[test]
fn a_force_stopped_at_a_base_without_fuel_is_told_whether_it_can_reach_its_home_port() {
    // Ample fuel: the passage to base 2 is made.
    let scenario = supply_run(true, 500);
    let made = briefing(&scenario.world.side_view(Side::Allied), "Run");
    let text = made.to_text();
    let line = line_with(&text, "- No fuel: base:1 ");
    assert!(
        line.contains("task_force:1 \"Placeholder supply convoy\" is stopped there; its least fuelled ship ship:3 \"Placeholder destroyer 1\" has 500 t; the passage to its home port base:2 \"Guadalcanal\""),
        "{line}"
    );
    assert!(line.ends_with(": it can get there"), "{line}");
    assert_eq!(made.warnings.base_without_fuel_with_force.len(), 1);
    let warning = &made.warnings.base_without_fuel_with_force[0];
    assert_eq!(warning.home_passage.as_ref().unwrap().outcome, "arrives");
    assert!(warning.home_passage.as_ref().unwrap().needs_t.is_some());

    // A ship with too little fuel runs dry on the way.
    let scenario = supply_run(true, 5);
    let made = briefing(&scenario.world.side_view(Side::Allied), "Run");
    let text = made.to_text();
    let line = line_with(&text, "- No fuel: base:1 ");
    assert!(
        line.contains("has 5 t; the passage to its home port"),
        "{line}"
    );
    assert!(
        line.contains("runs a ship dry after") && line.ends_with("it cannot get there"),
        "{line}"
    );
    let json = serde_json::to_value(&made).expect("serialises");
    valid(&schemas(), "briefing.schema.json", &json, "supply run");

    // With no home port it says so.
    let scenario = supply_run(false, 500);
    let text = briefing(&scenario.world.side_view(Side::Allied), "Run").to_text();
    let line = line_with(&text, "- No fuel: base:1 ");
    assert!(
        line.ends_with("it has no home port of yours to go to"),
        "{line}"
    );
}

// ---- item 6 and item 7: a force with nothing left to do, a landing summed up ----

#[test]
fn a_force_that_landed_everything_is_named_free_and_its_landing_is_one_line() {
    let scenario = load("21_landing.toml", 1942);
    let (mut s, mut scripts) = session_of(scenario, &[Side::Allied]);
    let script = scripts.remove(&Side::Allied).expect("the Allied script");
    for day in 1..=2 {
        let orders = script.get(&day).cloned().unwrap_or_default();
        play_day(&mut s, Side::Allied, &orders);
    }
    let (text, data) = briefing_of(&mut s);
    let free = line_with(&text, "- Free: task_force:1 ");
    assert!(
        free.contains("has finished unloading at base:2 \"Landing at (8,0)\"; it still carries nothing [event "),
        "{free}"
    );
    let landing = line_with(&text, "- Landing yesterday: task_force:1 ");
    assert!(
        landing.contains("put ashore 1000 men, 14 t supply and 0 t fuel; 0 men aboard, 0 t supply and 0 t fuel; finished [event "),
        "{landing}"
    );
    // The ships' lines are not in the text, and are in the data.
    assert!(!text.contains("Landed yesterday"), "{text}");
    assert!(!data["land_units"]["landed_yesterday"]
        .as_array()
        .unwrap()
        .is_empty());
    let summary = &data["land_units"]["landings_yesterday"][0];
    assert_eq!(summary["men_landed"], 1000);
    assert_eq!(summary["finished"], true);
    assert_eq!(data["free_forces"][0]["finished"], "unloading");
}

#[test]
fn a_landing_in_progress_says_when_it_will_finish_at_yesterdays_rate() {
    let scenario = load("30_working_units.toml", 1942);
    let (mut s, mut scripts) = session_of(scenario, &[Side::Allied]);
    let script = scripts.remove(&Side::Allied).expect("the Allied script");
    play_day(
        &mut s,
        Side::Allied,
        &script.get(&1).cloned().unwrap_or_default(),
    );
    let (text, data) = briefing_of(&mut s);
    // Force 1 unloads 1500 t of supply at base 2; 480 t went ashore, 1020 t
    // are aboard, which is three days at that rate.
    let landing = line_with(&text, "- Landing yesterday: task_force:1 ");
    assert!(
        landing.contains("put ashore 0 men, 480 t supply and 0 t fuel; 0 men aboard, 1020 t supply and 0 t fuel; finishes in about 3 days at yesterday's rate"),
        "{landing}"
    );
    assert_eq!(
        data["land_units"]["landings_yesterday"][0]["finishes_in_days"],
        3
    );
    // It is not free while cargo is aboard.
    assert!(!text.contains("- Free: task_force:1 "), "{text}");
}

// ---- item 9: losses name their cause ----

#[test]
fn aircraft_written_off_say_to_what() {
    let scenario = load("06b_wear_and_repair.toml", 1942);
    let (mut s, mut scripts) = session_of(scenario, &[Side::Allied, Side::Japanese]);
    let mut found = None;
    for day in 1..=s.days() {
        for side in [Side::Allied, Side::Japanese] {
            let orders = scripts[&side].get(&day).cloned().unwrap_or_default();
            play_day(&mut s, side, &orders);
        }
        let _ = &mut scripts;
        let (text, data) = {
            let text = s
                .call(Side::Allied, "get_briefing", &json!({ "text": true }))
                .unwrap();
            let data = s
                .call(Side::Allied, "get_briefing", &json!({ "text": false }))
                .unwrap();
            (text["text"].as_str().unwrap().to_string(), data)
        };
        valid(&schemas(), "briefing.schema.json", &data, "wear");
        if let Some(line) = text
            .lines()
            .find(|l| l.starts_with("- Aircraft lost:") && l.contains("written off ("))
        {
            found = Some((line.to_string(), data));
            break;
        }
    }
    let (line, data) = found.expect("a day with aircraft written off");
    assert!(line.contains("worn out on a "), "{line}");
    assert!(line.contains(" sortie"), "{line}");
    let loss = data["losses"]["aircraft"]
        .as_array()
        .unwrap()
        .iter()
        .find(|l| l["written_off"].as_u64().unwrap_or(0) > 0)
        .unwrap();
    let total: u64 = loss["written_off_by"]
        .as_array()
        .unwrap()
        .iter()
        .map(|w| w["aircraft"].as_u64().unwrap())
        .sum();
    assert_eq!(total, loss["written_off"].as_u64().unwrap(), "{loss}");
}

// ---- item 8: one hex, shown together ----

#[test]
fn a_hex_with_two_of_the_sides_bases_is_one_place_in_the_briefing_and_the_list() {
    // Scenario 30 with a second Allied base in the hex of base 1, (0,0).
    let name = "30_working_units.toml";
    let text = format!(
        "{}\n[[base]]\nid = 8\nname = \"Placeholder second base\"\nside = \"allied\"\nq = 0\nr = 0\nsupply_t = 100\n",
        scenario_text(name)
    );
    let scenario = load_text(&text, name, 1942);
    let view = scenario.world.side_view(Side::Allied);
    let made = briefing(&view, "Works");
    let text = made.to_text();
    assert!(
        text.contains("\n- (0,0) Placeholder base 1 / Placeholder second base: 2 bases of yours, supply 5100 t, fuel 0 t in all"),
        "{text}"
    );
    // The hex's days of supply once, under the hex and not under each base:
    // one line for (0,0) and one for each of the other hexes.
    let hexes = made
        .bases
        .iter()
        .map(|b| (b.place.q, b.place.r))
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        text.matches("Days of supply for the hex").count(),
        hexes.len(),
        "{text}"
    );
    assert!(
        text.contains("\n  - base:1 \"Placeholder base 1\": supply 5000 t"),
        "{text}"
    );
    assert!(
        text.contains("\n  - base:8 \"Placeholder second base\": supply 100 t"),
        "{text}"
    );
    let json = serde_json::to_value(&made).expect("serialises");
    valid(&schemas(), "briefing.schema.json", &json, "shared hex");
    assert_eq!(
        json["shared_hexes"][0]["bases"].as_array().unwrap().len(),
        2
    );
    assert!(json["bases"][0].get("planned_days_of_supply").is_none());

    let list = serde_json::to_value(sim_agent::list_bases(&view, None)).expect("serialises");
    valid(&schemas(), "list_bases.schema.json", &list, "shared hex");
    assert_eq!(list["shared_hexes"][0]["supply_t"], "5100");
    assert!(list["bases"][0].get("days_of_supply").is_none(), "{list}");
    // A base alone in its hex keeps its own figure.
    assert!(list["bases"][1]["days_of_supply"].is_object(), "{list}");
}
