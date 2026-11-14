//! Land units in the agent's orders, handles, queries and briefing (decision
//! 0066, "What a player sees"), and what the enemy is not told of them
//! (decisions 12 and 22).
//!
//! The scenario is 21, on the frozen figures: Allied task force 1 sails with
//! land units 1 (600 men) and 2 (400) and 14 t of supply to the empty shore
//! (8,0), where the first man landed forms base 2; task force 2 lies at (10,0),
//! where unit 3 (500 men) is ashore with no base. Task force 2 takes 200 of
//! unit 3's men aboard on day 1 (the scenario's own load order) and lands them
//! at (8,0) on day 3, where they become a new unit, since unit 3 has men
//! ashore elsewhere. The hand calculation of the engine's side is in
//! `tests/scenario_21_landing.rs`; what is checked here is what a player is
//! told: the same numbers, by the side's own handles.

mod support;

use serde_json::{json, Value};
use sim_agent::Session;
use sim_core::state::Side;
use support::check::valid;
use support::play::{play_day, session_of};
use support::{load_text, scenario_text, schemas};

/// Allied player, scripted Japanese seat, on the scenario as written.
fn landing(text: &str) -> (Session, support::play::ByDay) {
    let scenario = load_text(text, "21_landing.toml", 1942);
    let (session, mut scripts) = session_of(scenario, &[Side::Allied]);
    let allied = scripts.remove(&Side::Allied).expect("the Allied script");
    (session, allied)
}

fn call(session: &mut Session, side: Side, tool: &str, args: Value) -> Value {
    session
        .call(side, tool, &args)
        .unwrap_or_else(|e| panic!("{tool} was refused: {e}"))
}

/// Plays the days up to, not including, `until` with the scenario's own
/// orders.
fn play_until(session: &mut Session, script: &support::play::ByDay, until: u32) {
    for day in session.day() + 1..until {
        let orders = script.get(&day).cloned().unwrap_or_default();
        play_day(session, Side::Allied, &orders);
    }
    assert_eq!(session.day() + 1, until);
}

fn text_of(session: &mut Session, side: Side) -> String {
    call(session, side, "get_briefing", json!({ "text": true }))["text"]
        .as_str()
        .expect("the briefing text")
        .to_string()
}

fn unit_order(force: u32, body: Value) -> Value {
    json!({ "unit": format!("task_force:{force}"), "order": body })
}

fn results(answer: &Value) -> Vec<(String, Option<String>)> {
    answer["results"]
        .as_array()
        .expect("results")
        .iter()
        .map(|r| {
            (
                r["status"].as_str().expect("status").to_string(),
                r["reason"].as_str().map(str::to_string),
            )
        })
        .collect()
}

// ---- Orders ----

#[test]
fn a_load_with_a_land_unit_is_queued_with_each_cargo_and_refused_when_it_cannot_be_done() {
    let schemas = schemas();
    let (mut session, _) = landing(&scenario_text("21_landing.toml"));
    let orders = vec![
        // Force 2 lies at (10,0), where unit 3 is ashore.
        unit_order(
            2,
            json!({"kind": "load", "unit": "land_unit:3", "cargo": "none"}),
        ),
        unit_order(2, json!({"kind": "load", "unit": "land_unit:3"})),
        unit_order(
            2,
            json!({"kind": "load", "unit": "land_unit:3", "cargo": "fuel"}),
        ),
        unit_order(
            2,
            json!({"kind": "load", "unit": "land_unit:3", "cargo": "supply"}),
        ),
        // Not the side's unit, or no unit at all: the same words.
        unit_order(2, json!({"kind": "load", "unit": "land_unit:9"})),
        unit_order(2, json!({"kind": "load", "unit": "land_unit:0"})),
        // Unit 1 is aboard, not ashore; force 1 is at base 1 with none of
        // unit 3's men there.
        unit_order(2, json!({"kind": "load", "unit": "land_unit:1"})),
        unit_order(1, json!({"kind": "load", "unit": "land_unit:3"})),
        // No cargo and no unit; a unit that is not one; a cargo that is not.
        unit_order(2, json!({"kind": "load", "cargo": "none"})),
        unit_order(2, json!({"kind": "load", "unit": "ship:1"})),
        unit_order(
            2,
            json!({"kind": "load", "unit": "land_unit:3", "cargo": "oil"}),
        ),
        // A load of supply is as it was.
        unit_order(1, json!({"kind": "load"})),
    ];
    let answer = call(
        &mut session,
        Side::Allied,
        "issue_orders",
        json!({ "orders": orders }),
    );
    valid(
        &schemas,
        "issue_orders.schema.json",
        &answer,
        "issue_orders",
    );
    let r = results(&answer);
    for (i, r) in r.iter().enumerate().take(4) {
        assert_eq!(r.0, "queued", "order {i}: {r:?}");
    }
    let reason = |i: usize| r[i].1.clone().expect("a reason");
    for (i, refusal) in r.iter().enumerate().take(r.len() - 1).skip(4) {
        assert_eq!(refusal.0, "refused", "order {i}: {refusal:?}");
    }
    // An unknown unit and a number that is nothing: one text, and it names
    // nothing of the question.
    assert_eq!(reason(4), reason(5));
    assert!(reason(4).starts_with("the land unit is not one of your units"));
    assert!(!reason(4).contains('9'));
    assert!(
        reason(6).starts_with("the land unit has no men ashore where the task force is"),
        "{}",
        reason(6)
    );
    assert_eq!(reason(6), reason(7));
    assert!(reason(8).starts_with("malformed order"), "{}", reason(8));
    assert!(reason(8).contains("needs a \"unit\""), "{}", reason(8));
    assert!(reason(9).starts_with("malformed order"), "{}", reason(9));
    assert!(reason(10).starts_with("malformed order"), "{}", reason(10));
    // The plain load of supply is queued: force 1 is at base 1.
    assert_eq!(r[11].0, "queued");
}

#[test]
fn a_load_with_a_unit_is_refused_when_no_ship_carries_men() {
    // Ship 3 is made a class with no berths for men.
    let text = scenario_text("21_landing.toml").replace("class = 11", "class = 1");
    let (mut session, _) = landing(&text);
    let answer = call(
        &mut session,
        Side::Allied,
        "issue_orders",
        json!({ "orders": [unit_order(2, json!({"kind": "load", "unit": "land_unit:3"}))] }),
    );
    let r = results(&answer);
    assert_eq!(r[0].0, "refused", "{answer}");
    let reason = r[0].1.clone().expect("a reason");
    assert!(
        reason.starts_with("no ship of the task force can carry men"),
        "{reason}"
    );
    assert!(reason.contains("(a load with a unit needs a ship"));
}

// ---- Handles and queries ----

#[test]
fn units_are_numbered_among_the_sides_own_and_a_unit_made_in_play_takes_the_next() {
    // A fourth unit of one man, ashore at (8,0): it wastes on the first night
    // and is destroyed, and keeps its number. The unit made by division is
    // the fifth.
    let text = format!(
        "{}\n[[land_unit]]\nid = 4\nname = \"Placeholder remnant\"\nside = \"allied\"\nmen = 1\nq = 8\nr = 0\n",
        scenario_text("21_landing.toml")
    );
    let schemas = schemas();
    let (mut session, script) = landing(&text);
    let start = call(&mut session, Side::Allied, "list_forces", json!({}));
    valid(&schemas, "list_forces.schema.json", &start, "list_forces");
    let names: Vec<&str> = start["land_units"]
        .as_array()
        .expect("land units")
        .iter()
        .map(|u| u["land_unit"]["handle"].as_str().expect("handle"))
        .collect();
    assert_eq!(
        names,
        ["land_unit:1", "land_unit:2", "land_unit:3", "land_unit:4"]
    );
    play_until(&mut session, &script, 7);
    let end = call(&mut session, Side::Allied, "list_forces", json!({}));
    valid(&schemas, "list_forces.schema.json", &end, "list_forces");
    let units = end["land_units"].as_array().expect("land units");
    let handles: Vec<&str> = units
        .iter()
        .map(|u| u["land_unit"]["handle"].as_str().expect("handle"))
        .collect();
    assert_eq!(
        handles,
        [
            "land_unit:1",
            "land_unit:2",
            "land_unit:3",
            "land_unit:4",
            "land_unit:5"
        ]
    );
    assert_eq!(units[3]["destroyed"], true);
    assert_eq!(units[3]["men_ashore"], 0);
    assert_eq!(units[3]["place"], Value::Null);
    assert_eq!(units[3]["fed_last_night"], "not_ashore");
    assert_eq!(units[4]["destroyed"], false);
    assert!(units[4]["land_unit"]["name"]
        .as_str()
        .expect("a name")
        .contains("detachment"));
    // The destroyed unit still answers, and the day's text says so.
    let entity = call(
        &mut session,
        Side::Allied,
        "get_entity",
        json!({"handle": "land_unit:4"}),
    );
    valid(&schemas, "entity.schema.json", &entity, "land_unit:4");
    assert_eq!(entity["entity"]["entity_kind"], "land_unit");
    assert_eq!(entity["entity"]["summary"]["destroyed"], true);
    let nothing = call(
        &mut session,
        Side::Allied,
        "get_entity",
        json!({"handle": "land_unit:6"}),
    );
    assert_eq!(nothing["kind"], "not_known");
    let text = text_of(&mut session, Side::Allied);
    assert!(
        text.contains("land_unit:4 \"Placeholder remnant\": destroyed, no man left"),
        "{text}"
    );
}

#[test]
fn a_land_unit_and_the_ship_that_carries_it_are_told_in_full() {
    let schemas = schemas();
    let (mut session, script) = landing(&scenario_text("21_landing.toml"));
    // The start: unit 1 fills ship 1, unit 2 is in ship 2.
    let ship = call(
        &mut session,
        Side::Allied,
        "get_entity",
        json!({"handle": "ship:1"}),
    );
    valid(&schemas, "entity.schema.json", &ship, "ship:1");
    assert_eq!(
        ship["entity"]["troops"]["land_unit"]["handle"],
        "land_unit:1"
    );
    assert_eq!(ship["entity"]["troops"]["men"], 600);
    let force = call(
        &mut session,
        Side::Allied,
        "get_entity",
        json!({"handle": "task_force:1"}),
    );
    valid(&schemas, "entity.schema.json", &force, "task_force:1");
    assert_eq!(force["entity"]["ships"][1]["troops"]["men"], 400);
    let unit = call(
        &mut session,
        Side::Allied,
        "get_entity",
        json!({"handle": "land_unit:1"}),
    );
    valid(&schemas, "entity.schema.json", &unit, "land_unit:1");
    let line = &unit["entity"]["summary"];
    assert_eq!(line["men_ashore"], 0);
    assert_eq!(line["men_aboard"], 600);
    assert_eq!(line["aboard"][0]["ship"]["handle"], "ship:1");
    assert_eq!(line["aboard"][0]["men"], 600);
    assert_eq!(line["fed_last_night"], "not_ashore");
    // Unit 3 is ashore at (10,0), and nobody has fed it yet.
    let three = call(
        &mut session,
        Side::Allied,
        "get_entity",
        json!({"handle": "land_unit:3"}),
    );
    assert_eq!(three["entity"]["summary"]["men_ashore"], 500);
    assert_eq!(three["entity"]["summary"]["place"]["q"], 10);
    assert_eq!(three["entity"]["summary"]["fed_last_night"], "not_known");

    // Two days on: 200 of unit 3's men are aboard ship 3, the rest are
    // ashore, unfed (no base of the side in the hex).
    play_until(&mut session, &script, 3);
    let three = call(
        &mut session,
        Side::Allied,
        "get_entity",
        json!({"handle": "land_unit:3"}),
    );
    valid(&schemas, "entity.schema.json", &three, "land_unit:3");
    let line = &three["entity"]["summary"];
    assert_eq!(line["men_aboard"], 200);
    assert_eq!(line["aboard"][0]["ship"]["handle"], "ship:3");
    assert_eq!(line["fed_last_night"], "unfed");
    assert!(line["men_ashore"].as_u64().expect("men") < 500);
    // The list filters by area: (8,0) with radius 0 has units 1 and 2 only.
    let area = call(
        &mut session,
        Side::Allied,
        "list_forces",
        json!({"area": {"q": 8, "r": 0, "radius_nm": 0}}),
    );
    let handles: Vec<&str> = area["land_units"]
        .as_array()
        .expect("land units")
        .iter()
        .map(|u| u["land_unit"]["handle"].as_str().expect("handle"))
        .collect();
    assert_eq!(handles, ["land_unit:1", "land_unit:2"]);
    // The base the landing formed is among the side's own, and named.
    let bases = call(&mut session, Side::Allied, "list_bases", json!({}));
    valid(&schemas, "list_bases.schema.json", &bases, "list_bases");
    assert_eq!(bases["bases"].as_array().expect("bases").len(), 2);
    assert_eq!(bases["bases"][1]["place"]["q"], 8);
    assert_eq!(
        bases["enemy_bases"].as_array().expect("enemy bases").len(),
        0
    );
}

#[test]
fn the_briefing_reads_the_landing_and_the_days_of_supply() {
    let (mut session, script) = landing(&scenario_text("21_landing.toml"));
    let first = text_of(&mut session, Side::Allied);
    assert!(first.contains("LAND UNITS"), "{first}");
    assert!(
        first.contains("land_unit:1 \"Placeholder battalion 1\": 600 men aboard (ship:1"),
        "{first}"
    );
    assert!(
        first
            .contains("Supply at (10,0): no base of the side here, so the 500 men ashore get none"),
        "{first}"
    );
    // After the first two days: the landing, the base, the feeding.
    play_until(&mut session, &script, 3);
    let day3 = text_of(&mut session, Side::Allied);
    for expected in [
        "Landed yesterday: 320 men of land_unit:1",
        "Base formed yesterday: base:2",
        "Fed last night: 275 men ashore at (10,0): none fed, no base of the side in the hex, 275 unfed",
        "Men lost yesterday: land_unit:3 \"Placeholder battalion 3\" lost 13 men unfed",
        "holds 14 t, the daily charge is 5 t (0 aircraft and 1000 men), so it lasts 2.8 days at the present charge",
        "Stocks yesterday: supply 0 t to 14 t (landed 14 t)",
    ] {
        assert!(day3.contains(expected), "missing {expected:?}\n{day3}");
    }
    // The men's charge is its own line of the stock, not the aircraft's.
    play_until(&mut session, &script, 6);
    let day6 = text_of(&mut session, Side::Allied);
    assert!(
        day6.contains("Stocks yesterday: supply 3 t to 0 t (feeding men ashore 3 t)"),
        "{day6}"
    );
    assert!(!day6.contains("sorties and upkeep"), "{day6}");
}

// ---- What the enemy is not told ----

/// Scenario 21 with a Japanese base and a Japanese air group at the Allied
/// port's hex, and the air combat figures that a base strike order needs. The
/// Japanese side is thereby a side with a briefing of its own.
fn with_a_japanese_side(text: &str) -> String {
    format!(
        "{}\n{}",
        text.replace(
            "ground = \"../reference/ground.toml\"",
            "ground = \"../reference/ground.toml\"\nair_combat = \"../reference/air_combat.toml\""
        ),
        r#"
[[base]]
id = 2
name = "Placeholder Japanese airfield"
side = "japanese"
q = 0
r = 0
supply_t = 100

[[air_group]]
id = 1
name = "Placeholder Japanese bombers"
side = "japanese"
base = 2
aircraft_type = 3
ready = 6
"#
    )
}

/// The same world without the Allied land units and the order that names one.
fn without_the_land_units(text: &str) -> String {
    text.split("\n\n")
        .filter(|chunk| {
            !chunk.contains("[[land_unit]]")
                && !(chunk.contains("[[order]]") && chunk.contains("unit = 3"))
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// Everything the Japanese seat can ask at the start of a day, as served.
fn japanese_probe(session: &mut Session) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut ask = |session: &mut Session, label: String, tool: &str, args: Value| {
        let answer = session
            .call(Side::Japanese, tool, &args)
            .unwrap_or_else(|e| panic!("{tool} was refused: {e}"));
        out.push((label, answer.to_string()));
    };
    ask(session, "briefing".into(), "get_briefing", json!({}));
    ask(
        session,
        "briefing text".into(),
        "get_briefing",
        json!({"text": true}),
    );
    for tool in ["list_forces", "list_bases", "list_contacts"] {
        ask(session, tool.into(), tool, json!({}));
    }
    for handle in [
        "land_unit:1",
        "land_unit:2",
        "land_unit:3",
        "land_unit:4",
        "base:1",
        "base:2",
        "base:3",
        "air_group:1",
    ] {
        ask(
            session,
            format!("entity {handle}"),
            "get_entity",
            json!({"handle": handle}),
        );
    }
    for event in 1..=40 {
        ask(
            session,
            format!("explain {event}"),
            "explain",
            json!({"event": event}),
        );
    }
    out
}

fn play_both(text: &str) -> Vec<Vec<(String, String)>> {
    let scenario = load_text(text, "21_landing.toml", 1942);
    let (mut session, scripts) = session_of(scenario, &[Side::Allied, Side::Japanese]);
    let mut probes = Vec::new();
    for day in 1..=session.days() {
        probes.push(japanese_probe(&mut session));
        let orders = scripts[&Side::Allied]
            .get(&day)
            .cloned()
            .unwrap_or_default();
        let (refused, _) = play_day(&mut session, Side::Allied, &orders);
        // The Allied seat's own orders are fine in the world with the units;
        // in the other, the one that named a unit is not there.
        assert!(refused.is_empty(), "day {day}: {refused:?}");
        play_day(&mut session, Side::Japanese, &[]);
    }
    probes.push(japanese_probe(&mut session));
    probes
}

#[test]
fn the_enemy_is_told_nothing_of_a_land_unit_its_landing_or_the_base_it_formed() {
    let with = with_a_japanese_side(&scenario_text("21_landing.toml"));
    let without = without_the_land_units(&with);
    assert!(!without.contains("land_unit"));
    let a = play_both(&with);
    let b = play_both(&without);
    assert_eq!(a.len(), b.len());
    for (day, (x, y)) in a.iter().zip(&b).enumerate() {
        for ((label, one), (_, other)) in x.iter().zip(y) {
            assert_eq!(one, other, "day {} start, Japanese {label}", day + 1);
        }
    }
    // The last state: the Japanese side knows its own base and the Allied
    // port on the map, and not the base the landing formed.
    let last = a.last().expect("a last probe");
    let bases: Value = serde_json::from_str(
        &last
            .iter()
            .find(|(l, _)| l == "list_bases")
            .expect("list_bases")
            .1,
    )
    .expect("JSON");
    assert_eq!(bases["bases"].as_array().expect("bases").len(), 1);
    assert_eq!(bases["enemy_bases"].as_array().expect("bases").len(), 1);
    assert_eq!(bases["enemy_bases"][0]["held_by"], "allied");
    let text = &last
        .iter()
        .find(|(l, _)| l == "briefing text")
        .expect("text")
        .1;
    for word in ["land_unit", "LAND UNITS", "men ashore", "landed", "formed"] {
        assert!(!text.contains(word), "{word}: {text}");
    }
}

#[test]
fn the_world_with_the_units_is_not_the_same_to_the_side_that_has_them() {
    // The comparison above would pass if the units changed nothing for anyone.
    let with = with_a_japanese_side(&scenario_text("21_landing.toml"));
    let without = without_the_land_units(&with);
    let mut texts = Vec::new();
    for text in [&with, &without] {
        let (mut session, script) = landing(text);
        play_until(&mut session, &script, 4);
        texts.push(text_of(&mut session, Side::Allied));
    }
    assert!(texts[0].contains("LAND UNITS"));
    assert!(!texts[1].contains("LAND UNITS"));
    assert_ne!(texts[0], texts[1]);
}

#[test]
fn a_base_strike_on_a_base_formed_by_a_landing_is_refused_as_on_one_that_does_not_exist() {
    let text = with_a_japanese_side(&scenario_text("21_landing.toml"));
    // The number the engine gave the formed base: found on a copy of the
    // world, which the player never holds. Base 1 and the Japanese base 2 are
    // the scenario's; the landing's is the next free number.
    let mut copy = load_text(&text, "21_landing.toml", 1942);
    copy.run_to_end().expect("the run resolves");
    let formed: u32 = {
        let truth = copy.world.umpire();
        let base = truth
            .bases
            .values()
            .find(|b| b.side == Side::Allied && b.id.raw() > 2)
            .expect("the landing formed a base");
        base.id.raw()
    };
    assert_eq!(formed, 3);
    let scenario = load_text(&text, "21_landing.toml", 1942);
    let (mut session, scripts) = session_of(scenario, &[Side::Allied, Side::Japanese]);
    let strike = |base: u32| {
        json!({"orders": [{
            "unit": "air_group:1",
            "order": {"kind": "set_air_mission", "mission": "base_strike", "target_base": base}
        }]})
    };
    for day in 1..=3 {
        let orders = scripts[&Side::Allied]
            .get(&day)
            .cloned()
            .unwrap_or_default();
        play_day(&mut session, Side::Allied, &orders);
        play_day(&mut session, Side::Japanese, &[]);
    }
    // Base 1 is the Allied port, on the map: queued. Base 3 exists and is
    // not on the Japanese map; 77 does not exist.
    let on_map = call(&mut session, Side::Japanese, "issue_orders", strike(1));
    assert_eq!(results(&on_map)[0].0, "queued", "{on_map}");
    let formed_base = call(&mut session, Side::Japanese, "issue_orders", strike(formed));
    let nowhere = call(&mut session, Side::Japanese, "issue_orders", strike(77));
    let (a, b) = (results(&formed_base), results(&nowhere));
    assert_eq!(a[0].0, "refused", "{formed_base}");
    assert_eq!(a, b);
}

/// Scenario 21 with two Allied bases at the bare shore (10,0), where unit 3's
/// 500 men are ashore: base 3 with 10 t of supply and base 4 with 20 t.
fn with_two_bases_at_the_bare_shore(text: &str) -> String {
    format!(
        "{text}\n{}",
        r#"
[[base]]
id = 3
name = "Placeholder shore dump 1"
side = "allied"
q = 10
r = 0
supply_t = 10

[[base]]
id = 4
name = "Placeholder shore dump 2"
side = "allied"
q = 10
r = 0
supply_t = 20
"#
    )
}

#[test]
fn the_days_of_supply_of_a_hex_count_the_stock_of_every_base_of_the_side_there() {
    // The men are fed from every base of their side in the hex, in base
    // order (decision 0090, 24a), so the briefing counts the stock of both:
    // 10 + 20 = 30 t against 500 men at 0.005 t a man, 2.5 t a day, which
    // lasts 12 days. One base alone would read 10 t and 4 days.
    let (mut session, _) = landing(&with_two_bases_at_the_bare_shore(&scenario_text(
        "21_landing.toml",
    )));
    let first = text_of(&mut session, Side::Allied);
    let expected = ": base:3 \"Placeholder shore dump 1\" and base:4 \"Placeholder shore dump 2\" hold 30 t, the daily charge is 2.5 t (0 aircraft and 500 men), so it lasts 12 days at the present charge";
    assert!(first.contains(expected), "missing {expected:?}\n{first}");
    let data = call(&mut session, Side::Allied, "get_briefing", json!({}));
    let hexes = data["land_units"]["supply_by_hex"]
        .as_array()
        .expect("supply by hex");
    let shore = hexes
        .iter()
        .find(|h| h["place"]["q"] == json!(10) && h["place"]["r"] == json!(0))
        .unwrap_or_else(|| panic!("the bare shore's line in {hexes:?}"));
    assert_eq!(shore["base"]["handle"], json!("base:3"), "{shore}");
    assert_eq!(
        shore["other_bases"][0]["handle"],
        json!("base:4"),
        "{shore}"
    );
    // The scenario as written has one base in each hex and names no other.
    let (mut plain, _) = landing(&scenario_text("21_landing.toml"));
    let data = call(&mut plain, Side::Allied, "get_briefing", json!({}));
    for h in data["land_units"]["supply_by_hex"]
        .as_array()
        .expect("hexes")
    {
        assert!(h.get("other_bases").is_none(), "{h}");
    }
}
