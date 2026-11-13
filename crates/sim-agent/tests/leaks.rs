//! What a side cannot know changes nothing it is given, and what it does not
//! know is answered in one way (decisions 9 and 10 of 0048).
//!
//! 1. Not known: asked about an enemy unit by the log's number, a number that
//!    does not exist, or an event the side may not see, `get_entity` and
//!    `explain` give the same JSON, and it carries nothing of the question.
//! 2. Leaks by difference, in the way of `tests/scenario_09a_side_events.rs`:
//!    two worlds that differ only in something the side cannot know (the
//!    enemy's fuel or supply, an order of the enemy's that is refused, a
//!    distant enemy force's course, the enemy's rest thresholds) give the side
//!    the same briefing, lists and entity answers on every day. Event
//!    numbers in the answers are compared by their place among the side's own
//!    events as well, and the cases where only they differ are counted and
//!    printed (decision 0047, "gaps in event numbers").
//! 3. The crate's library code names nothing of the truth.
//!
//! `measure` is in all three: its not-known answers are in
//! `tests/measure.rs`, and a sample of its answers is compared with the rest
//! under every difference.

mod support;

use std::collections::BTreeMap;

use serde_json::Value;
use sim_agent::{
    briefing, get_entity, list_bases, list_contacts, list_forces, measure, PlaceArg, Question,
};
use sim_core::hex::Hex;
use sim_core::state::Side;
use sim_core::World;
use support::check::{normalised_answers, sample_questions, without_event_numbers};
use support::{load, load_text, play, scenario_path, scenario_text};

const SIDES: [Side; 2] = [Side::Allied, Side::Japanese];

fn other(side: Side) -> Side {
    match side {
        Side::Allied => Side::Japanese,
        Side::Japanese => Side::Allied,
    }
}

// ---------------------------------------------------------------------
// 1. Not known.
// ---------------------------------------------------------------------

fn unknown_handles(world: &World, side: Side) -> Vec<String> {
    let truth = world.umpire();
    let mut handles: Vec<String> = Vec::new();
    handles.extend(
        truth
            .ships
            .values()
            .filter(|u| u.side != side)
            .map(|u| u.id.to_string()),
    );
    handles.extend(
        truth
            .task_forces
            .values()
            .filter(|u| u.side != side)
            .map(|u| u.id.to_string()),
    );
    handles.extend(
        truth
            .air_groups
            .values()
            .filter(|u| u.side != side)
            .map(|u| u.id.to_string()),
    );
    // An enemy base is not here: its number, name, place and holder are on
    // the map and the side may ask for them (decision 0050, as amended).
    for kind in [
        "ship",
        "task_force",
        "air_group",
        "base",
        "contact",
        "pilot",
        "land_unit",
    ] {
        handles.push(format!("{kind}:99999"));
        handles.push(format!("{kind}:0"));
    }
    handles
        .extend(["", "banana", "ship:", "ship:-1", "ship:x", ":3", "ship:1:2"].map(String::from));
    handles
}

#[test]
fn an_enemy_unit_a_missing_number_and_a_bad_handle_get_one_and_the_same_answer() {
    for name in [
        "05_eastern_solomons.toml",
        "07_full_slice.toml",
        "06a_base_raid.toml",
    ] {
        let mut scenario = load(name, 1942);
        scenario.run_to_end().expect("the run resolves");
        for side in SIDES {
            let view = scenario.world.side_view(side);
            let handles = unknown_handles(&scenario.world, side);
            let first = serde_json::to_string(&get_entity(&view, &handles[0])).unwrap();
            assert!(first.contains("not_known"), "{name}: {first}");
            for handle in &handles {
                let answer = serde_json::to_string(&get_entity(&view, handle)).unwrap();
                assert_eq!(answer, first, "{name} {side:?} {handle:?}");
            }
            // An own unit is known.
            let own = view
                .ships()
                .map(|u| u.id.to_string())
                .chain(view.task_forces().map(|u| u.id.to_string()))
                .chain(view.air_groups().map(|u| u.id.to_string()))
                .chain(view.bases().map(|u| u.id.to_string()))
                .next()
                .expect("an own unit");
            let known = serde_json::to_string(&get_entity(&view, &own)).unwrap();
            assert_ne!(known, first);
            // The answer says nothing of the question.
            assert!(
                !first.contains("99999") && !first.contains("ship:"),
                "{first}"
            );
        }
    }
}

#[test]
fn a_number_that_is_no_event_of_the_side_gets_one_and_the_same_answer() {
    // A side numbers the events it sees 1, 2, 3...; the log's numbers, which
    // would name events it may not see, are not in use. Beyond its last, and
    // 0, there is nothing, and the answer is one document.
    for name in [
        "05_eastern_solomons.toml",
        "07_full_slice.toml",
        "04a_night_action.toml",
    ] {
        let mut scenario = load(name, 1942);
        scenario.run_to_end().expect("the run resolves");
        for side in SIDES {
            let view = scenario.world.side_view(side);
            let seen = view.events().len() as u32;
            assert!(
                scenario.world.events().len() as u32 > seen,
                "{name}: the log holds events the side does not see"
            );
            let explainer = sim_agent::Explainer::new(&view);
            let first = serde_json::to_string(&explainer.explain(0)).unwrap();
            assert!(first.contains("not_known"), "{first}");
            for id in [
                seen + 1,
                seen + 2,
                scenario.world.events().len() as u32 + 1,
                u32::MAX,
                999_999,
            ] {
                let answer = serde_json::to_string(&explainer.explain(id)).unwrap();
                assert_eq!(answer, first, "{name} {side:?} event {id}");
                assert_eq!(
                    serde_json::to_string(&sim_agent::explain(&view, id)).unwrap(),
                    first
                );
            }
            assert!(!first.contains("999999"), "{first}");
            for known in [1, seen] {
                assert_ne!(
                    serde_json::to_string(&explainer.explain(known)).unwrap(),
                    first
                );
            }
        }
    }
}

#[test]
fn an_enemy_base_is_known_by_its_public_facts_and_a_missing_base_is_not() {
    for name in ["06_rabaul_guadalcanal.toml", "05_eastern_solomons.toml"] {
        let scenario = load(name, 1942);
        for side in SIDES {
            let view = scenario.world.side_view(side);
            let missing = serde_json::to_string(&get_entity(&view, "base:99999")).unwrap();
            assert!(missing.contains("not_known"), "{missing}");
            let enemy: Vec<_> = scenario
                .world
                .umpire()
                .bases
                .values()
                .filter(|b| b.side != side)
                .cloned()
                .collect();
            for base in enemy {
                let answer = serde_json::to_value(get_entity(&view, &base.id.to_string())).unwrap();
                assert_eq!(answer["kind"], "entity", "{name} {side:?}");
                let detail = &answer["entity"];
                assert_eq!(detail["entity_kind"], "enemy_base");
                assert_eq!(detail["summary"]["base"]["name"], base.name.as_str());
                assert_eq!(detail["summary"]["place"]["q"], base.hex.q);
                assert_eq!(detail["summary"]["held_by"], side_word(base.side));
                // Nothing else of it: the keys are exactly these.
                let keys: Vec<&String> = detail.as_object().unwrap().keys().collect();
                assert_eq!(keys, ["entity_kind", "known", "summary"]);
                let line: Vec<&String> = detail["summary"].as_object().unwrap().keys().collect();
                assert_eq!(line, ["base", "held_by", "place"]);
                assert_ne!(
                    serde_json::to_string(&answer).unwrap(),
                    missing,
                    "known and missing are deliberately distinguishable"
                );
            }
        }
    }
}

/// The text of the scenario with every private figure of the enemy's bases
/// changed: supply, fuel, anti-aircraft guns, repairs, and the aircraft and
/// pilots of the air groups at them.
fn enemy_bases_privately_different(doc: &Doc, enemy: Side) -> String {
    let side = side_word(enemy);
    let tables: Vec<String> = doc
        .tables
        .iter()
        .map(|table| {
            let mine = table_value(table, "side").as_deref() == Some(side);
            let mut table = table.clone();
            if mine && table_kind(&table) == "base" {
                for key in [
                    "supply_t",
                    "anti_aircraft",
                    "aircraft_repairs_per_day",
                    "fuel_t",
                ] {
                    match change_number(&table, key, |n| Some(n + 7)) {
                        Some(changed) => table = changed,
                        None => {
                            if !table.ends_with('\n') {
                                table.push('\n');
                            }
                            table.push_str(&format!("{key} = 7\n"));
                        }
                    }
                }
            }
            if mine && table_kind(&table) == "air_group" && table_value(&table, "base").is_some() {
                for key in ["ready", "pilots"] {
                    if let Some(changed) = change_number(&table, key, |n| Some(n + 3)) {
                        table = changed;
                    }
                }
            }
            table
        })
        .collect();
    Doc {
        head: doc.head.clone(),
        tables,
    }
    .text()
}

#[test]
fn what_an_enemy_base_holds_changes_nothing_the_tools_say_of_it() {
    let mut compared = 0;
    for name in [
        "05_eastern_solomons.toml",
        "06a_base_raid.toml",
        "06_rabaul_guadalcanal.toml",
        "07_full_slice.toml",
    ] {
        let text = scenario_text(name);
        let doc = parse_doc(&text);
        for side in SIDES {
            let enemy = other(side);
            let changed = enemy_bases_privately_different(&doc, enemy);
            if changed == text {
                continue;
            }
            let (one, two) = (load_text(&text, name, 5), load_text(&changed, name, 5));
            // The truth does differ.
            let figures = |s: &sim_data::Scenario| {
                s.world
                    .umpire()
                    .bases
                    .values()
                    .filter(|b| b.side == enemy)
                    .map(|b| {
                        (
                            b.supply_t,
                            b.fuel_t,
                            b.anti_aircraft,
                            b.aircraft_repairs_per_day,
                        )
                    })
                    .collect::<Vec<_>>()
            };
            assert_ne!(
                figures(&one),
                figures(&two),
                "{name} {side:?}: the edit applied"
            );
            let ask = |s: &sim_data::Scenario| {
                let view = s.world.side_view(side);
                let mut out = vec![
                    serde_json::to_string(&list_bases(&view, None)).unwrap(),
                    serde_json::to_string(&list_bases(&view, Some((Hex::new(0, 0), 400)))).unwrap(),
                ];
                for base in view.enemy_bases() {
                    let handle = base.id.to_string();
                    out.push(serde_json::to_string(&get_entity(&view, &handle)).unwrap());
                    for force in view.task_forces() {
                        let from = force.id.to_string();
                        for question in [
                            Question::AirDistance {
                                from: PlaceArg::unit(&from),
                                to: PlaceArg::unit(&handle),
                            },
                            Question::SeaRoute {
                                from: PlaceArg::unit(&from),
                                to: PlaceArg::unit(&handle),
                            },
                            Question::Passage {
                                task_force: from.clone(),
                                to: PlaceArg::unit(&handle),
                                speed: None,
                            },
                        ] {
                            out.push(serde_json::to_string(&measure(&view, &question)).unwrap());
                        }
                    }
                    for group in view.air_groups() {
                        out.push(
                            serde_json::to_string(&measure(
                                &view,
                                &Question::AirReach {
                                    air_group: group.id.to_string(),
                                    to: vec![PlaceArg::unit(&handle)],
                                },
                            ))
                            .unwrap(),
                        );
                    }
                }
                out
            };
            let (a, b) = (ask(&one), ask(&two));
            assert!(a.len() >= 4, "{name} {side:?}: something was asked");
            assert_eq!(a, b, "{name} {side:?}");
            compared += 1;
        }
    }
    assert!(compared >= 4, "{compared} comparisons");
}

// ---------------------------------------------------------------------
// 2. Leaks by difference.
// ---------------------------------------------------------------------

/// A scenario file's text as its head and its `[[tables]]`.
struct Doc {
    head: String,
    tables: Vec<String>,
}

fn parse_doc(text: &str) -> Doc {
    let mut head = String::new();
    let mut tables: Vec<String> = Vec::new();
    for line in text.split_inclusive('\n') {
        if line.trim_end().starts_with("[[") {
            tables.push(line.to_string());
        } else if let Some(last) = tables.last_mut() {
            last.push_str(line);
        } else {
            head.push_str(line);
        }
    }
    Doc { head, tables }
}

impl Doc {
    fn text(&self) -> String {
        let mut text = self.head.clone();
        for table in &self.tables {
            text.push_str(table);
        }
        text
    }
}

fn table_kind(table: &str) -> &str {
    table
        .lines()
        .next()
        .and_then(|line| line.trim().strip_prefix("[["))
        .and_then(|rest| rest.strip_suffix("]]"))
        .unwrap_or("")
}

fn table_value(table: &str, key: &str) -> Option<String> {
    table.lines().find_map(|line| {
        line.strip_prefix(key)
            .and_then(|rest| rest.strip_prefix(" = "))
            .map(|v| v.trim().trim_matches('"').to_string())
    })
}

/// The table with the whole number under `key` changed by `change`, if it has
/// one and `change` gives a value.
fn change_number(table: &str, key: &str, change: impl Fn(i64) -> Option<i64>) -> Option<String> {
    let old = table_value(table, key)?.parse::<i64>().ok()?;
    let new = change(old)?;
    Some(table.replace(&format!("{key} = {old}\n"), &format!("{key} = {new}\n")))
}

fn side_word(side: Side) -> &'static str {
    match side {
        Side::Allied => "allied",
        Side::Japanese => "japanese",
    }
}

/// What a difference does to a scenario's text: the text of the first world
/// and of the second, or `None` if the difference does not apply to the
/// scenario.
type Make = fn(&Doc, Side, Side, &World) -> Option<(String, String)>;

/// Applies `change_number` to every table of `kind` of `side`.
fn edit_tables(
    doc: &Doc,
    side: Side,
    (kind, key): (&str, &str),
    only: impl Fn(&str) -> bool,
    change: impl Fn(i64) -> Option<i64>,
) -> Option<(String, String)> {
    let mut edited = false;
    let tables = doc
        .tables
        .iter()
        .map(|table| {
            if table_kind(table) == kind
                && table_value(table, "side").as_deref() == Some(side_word(side))
                && only(table)
            {
                if let Some(new) = change_number(table, key, &change) {
                    edited = true;
                    return new;
                }
            }
            table.clone()
        })
        .collect();
    edited.then(|| {
        (
            doc.text(),
            Doc {
                head: doc.head.clone(),
                tables,
            }
            .text(),
        )
    })
}

/// The enemy's ships carry a ton less fuel.
fn less_fuel(doc: &Doc, _side: Side, enemy: Side, _world: &World) -> Option<(String, String)> {
    edit_tables(
        doc,
        enemy,
        ("ship", "fuel_t"),
        |_| true,
        |n| (n >= 100).then_some(n - 1),
    )
}

/// The enemy's bases hold a ton more supply.
fn more_supply(doc: &Doc, _side: Side, enemy: Side, _world: &World) -> Option<(String, String)> {
    edit_tables(
        doc,
        enemy,
        ("base", "supply_t"),
        |_| true,
        |n| (n >= 10).then_some(n + 1),
    )
}

/// The enemy's air groups rest at a threshold one point higher.
fn higher_rest_threshold(
    doc: &Doc,
    _side: Side,
    enemy: Side,
    _world: &World,
) -> Option<(String, String)> {
    edit_tables(
        doc,
        enemy,
        ("order", "fatigue"),
        |table| table_value(table, "kind").as_deref() == Some("set_rest_threshold"),
        |n| (n < 99).then_some(n + 1),
    )
}

/// A control, which must be seen: the enemy gets one more aircraft a day. It
/// changes what the enemy can send, which the side meets.
fn more_replacements(
    doc: &Doc,
    _side: Side,
    enemy: Side,
    _world: &World,
) -> Option<(String, String)> {
    edit_tables(
        doc,
        enemy,
        ("replacement", "aircraft_per_day"),
        |_| true,
        |n| Some(n + 1),
    )
}

/// The enemy gives an order, on the last day, to a force of the side's. It is
/// rejected, and the rejection never reveals that the unit exists (decision
/// 0008).
fn rejected_order(doc: &Doc, side: Side, enemy: Side, _world: &World) -> Option<(String, String)> {
    let days = doc.head.lines().find_map(|l| {
        l.strip_prefix("days = ")
            .and_then(|v| v.trim().parse::<u32>().ok())
    })?;
    rejected_order_on(doc, side, enemy, days)
}

/// The same on day 1, so the enemy's order is numbered before every order
/// the side gives after it. The side's own orders must keep the numbers it
/// is shown (decision 0055).
fn rejected_order_on_day_one(
    doc: &Doc,
    side: Side,
    enemy: Side,
    _world: &World,
) -> Option<(String, String)> {
    rejected_order_on(doc, side, enemy, 1)
}

fn rejected_order_on(doc: &Doc, side: Side, enemy: Side, days: u32) -> Option<(String, String)> {
    let target = doc
        .tables
        .iter()
        .find(|t| {
            table_kind(t) == "task_force"
                && table_value(t, "side").as_deref() == Some(side_word(side))
        })
        .and_then(|t| table_value(t, "id"))?;
    let mut text = doc.text();
    text.push_str(&format!(
        "\n[[order]]\nday = {days}\nside = \"{}\"\ntask_force = {target}\nkind = \"set_speed\"\nspeed = \"cruise\"\n",
        side_word(enemy)
    ));
    Some((doc.text(), text))
}

/// The enemy has a force far from every unit of the side, and outside every
/// arc its aircraft could search, in both worlds. In the first it steams one
/// way and in the second another. (Whether it is there at all is a different
/// question, see `a_new_force_takes_the_next_number`.)
fn far_unit_moves(doc: &Doc, side: Side, enemy: Side, world: &World) -> Option<(String, String)> {
    let truth = world.umpire();
    let mine: Vec<Hex> = truth
        .task_forces
        .values()
        .filter(|f| f.side == side)
        .map(|f| f.position())
        .chain(
            truth
                .bases
                .values()
                .filter(|b| b.side == side)
                .map(|b| b.hex),
        )
        .collect();
    let (hex, away) = world
        .map()
        .iter()
        .filter(|(hex, _)| world.map().is_navigable(**hex))
        .map(|(hex, _)| {
            let nearest = mine
                .iter()
                .map(|m| m.air_distance_nm(*hex))
                .min()
                .unwrap_or(0);
            (*hex, nearest)
        })
        .max_by_key(|(hex, away)| (*away, hex.q, hex.r))?;
    // Beyond the longest search range of the side's aircraft, with room to
    // spare.
    let longest = truth
        .air_groups
        .values()
        .filter(|g| g.side == side)
        .filter_map(|g| world.ref_data().aircraft_types.get(&g.aircraft_type))
        .map(|t| t.search_range_nm.trunc())
        .max()
        .unwrap_or(0);
    if i64::from(away) < longest + 100 {
        return None;
    }
    let ways: Vec<Hex> = (0..6)
        .filter(|d| world.map().can_step(hex, *d))
        .map(|d| hex.neighbors()[d])
        .collect();
    let (way_a, way_b) = (*ways.first()?, *ways.get(1)?);
    let class = truth.ships.values().find(|s| s.side == enemy)?.class.raw();
    let free_force = truth.task_forces.keys().map(|k| k.raw()).max()? + 1;
    let free_ship = truth.ships.keys().map(|k| k.raw()).max()? + 1000;
    let text = |way: Hex| {
        let mut text = doc.text();
        text.push_str(&format!(
            "\n[[task_force]]\nid = {free_force}\nname = \"Placeholder far unit\"\nside = \"{e}\"\nq = {q}\nr = {r}\nmission = \"escort\"\n\n[[ship]]\nid = {free_ship}\nname = \"Placeholder far ship\"\nclass = {class}\nside = \"{e}\"\nfuel_t = 100\ntask_force = {free_force}\n\n[[order]]\nday = 1\nside = \"{e}\"\ntask_force = {free_force}\nkind = \"set_route\"\nwaypoints = [[{wq}, {wr}]]\n",
            e = side_word(enemy),
            q = hex.q,
            r = hex.r,
            wq = way.q,
            wr = way.r
        ));
        text
    };
    Some((text(way_a), text(way_b)))
}

/// Every answer of one side at the start of each day (and after the last):
/// the briefing, the lists, and every own unit and contact.
fn answers(view: &sim_core::view::SideView<'_>, name: &str) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    let json = |v: &dyn erased::Ser| v.to_json();
    out.insert("briefing".to_string(), json(&briefing(view, name)));
    out.insert("list_forces".to_string(), json(&list_forces(view, None)));
    out.insert("list_bases".to_string(), json(&list_bases(view, None)));
    out.insert(
        "list_contacts".to_string(),
        json(&list_contacts(view, None)),
    );
    let handles: Vec<String> = view
        .ships()
        .map(|s| s.id.to_string())
        .chain(view.task_forces().map(|s| s.id.to_string()))
        .chain(view.air_groups().map(|s| s.id.to_string()))
        .chain(view.bases().map(|s| s.id.to_string()))
        .chain(view.contacts().map(|s| s.id.to_string()))
        .collect();
    for handle in handles {
        out.insert(format!("entity:{handle}"), json(&get_entity(view, &handle)));
    }
    // The bases of the other side: public facts, asked for as an entity and as
    // a place to measure to.
    for base in view.enemy_bases() {
        let handle = base.id.to_string();
        out.insert(format!("entity:{handle}"), json(&get_entity(view, &handle)));
        if let Some(force) = view.task_forces().next() {
            let from = force.id.to_string();
            out.insert(
                format!("measure:air_distance:{from}:{handle}"),
                json(&measure(
                    view,
                    &Question::AirDistance {
                        from: PlaceArg::unit(&from),
                        to: PlaceArg::unit(&handle),
                    },
                )),
            );
        }
    }
    // Measurements: passage, sea route, air distance and air reach, of a few
    // forces and groups to a few bases and contacts (decision 0049).
    for (key, question) in sample_questions(view, 3, 2) {
        out.insert(key, json(&measure(view, &question)));
    }
    out
}

mod erased {
    pub trait Ser {
        fn to_json(&self) -> String;
    }
    impl<T: serde::Serialize> Ser for T {
        fn to_json(&self) -> String {
            serde_json::to_string(self).expect("an answer serialises")
        }
    }
}

/// One side's answers for each day, raw and with event numbers taken out, and
/// its own orders at the end.
struct Run {
    raw: Vec<BTreeMap<String, String>>,
    plain: Vec<BTreeMap<String, Value>>,
    orders: Vec<(u32, u32)>,
    enemy_briefings: Vec<String>,
}

fn run(text: &str, name: &str, seed: u64, side: Side) -> Run {
    let mut scenario = load_text(text, name, seed);
    let title = scenario.name.clone();
    let mut raw = Vec::new();
    let mut plain = Vec::new();
    let mut enemy_briefings = Vec::new();
    play(&mut scenario, |s| {
        let view = s.world.side_view(side);
        let mut a = answers(&view, &title);
        // What the side is told of orders it might give (stage 9d): the same
        // probes in every world, on the first day, the middle one and the end.
        let day = s.world.day();
        if day == 0 || day == s.days / 2 || day == s.days {
            a.insert(
                "issue_orders".to_string(),
                support::play::probe_answer(&s.world, side, &title),
            );
        }
        plain.push(normalised_answers(&view, &a));
        raw.push(a);
        enemy_briefings.push(briefing(&s.world.side_view(other(side)), &title).to_text());
    });
    let orders = scenario
        .world
        .orders()
        .filter(|o| o.side == side)
        .enumerate()
        // By the side's own count, which is what it is shown (decision
        // 0055). The log's numbers run through both sides' orders, so they
        // move with what the enemy orders.
        .map(|(i, o)| (u32::try_from(i + 1).expect("few orders"), o.day_issued))
        .collect();
    Run {
        raw,
        plain,
        orders,
        enemy_briefings,
    }
}

#[derive(Default, Debug)]
struct Found {
    compared: usize,
    identical: usize,
    /// Equal once the log's event numbers are taken out: the numbers moved
    /// because of events the side does not see.
    only_event_numbers_differ: usize,
    /// The enemy's own briefings differ: the difference was real.
    enemy_saw_it: usize,
    changed: Vec<String>,
    inapplicable: usize,
}

fn compare(name: &str, seeds: std::ops::RangeInclusive<u64>, make: Make, found: &mut Found) {
    let text = scenario_text(name);
    let doc = parse_doc(&text);
    let world0 =
        sim_data::Scenario::from_text_with(&text, &scenario_path(name), &support::frozen())
            .expect("loads")
            .world;
    for side in SIDES {
        let enemy = other(side);
        let Some((first, second)) = make(&doc, side, enemy, &world0) else {
            found.inapplicable += 1;
            continue;
        };
        for seed in seeds.clone() {
            let one = run(&first, name, seed, side);
            let two = run(&second, name, seed, side);
            if one.orders != two.orders {
                found.inapplicable += 1;
                continue;
            }
            found.compared += 1;
            if one.enemy_briefings != two.enemy_briefings {
                found.enemy_saw_it += 1;
            }
            if one.raw == two.raw {
                found.identical += 1;
            } else if one.plain == two.plain {
                // The log's numbers would show here. A side's own numbers do not.
                found.only_event_numbers_differ += 1;
                found.changed.push(format!(
                    "{name} seed {seed} side {side:?}: only numbers differ"
                ));
            } else {
                let (day, label) = one
                    .plain
                    .iter()
                    .zip(&two.plain)
                    .enumerate()
                    .find_map(|(d, (a, b))| {
                        a.iter()
                            .find(|(k, v)| b.get(*k) != Some(v))
                            .map(|(k, _)| (d, k.clone()))
                            .or_else(|| {
                                (a.len() != b.len()).then(|| (d, "the set of answers".to_string()))
                            })
                    })
                    .unwrap_or((0, "?".to_string()));
                found.changed.push(format!(
                    "{name} seed {seed} side {side:?}: day {day}, {label}"
                ));
            }
        }
    }
}

const LEAK_SCENARIOS: [&str; 5] = [
    "04a_night_action.toml",
    "05_eastern_solomons.toml",
    "06a_base_raid.toml",
    "06_rabaul_guadalcanal.toml",
    "07_full_slice.toml",
];

fn found_by(make: Make, seeds: std::ops::RangeInclusive<u64>) -> Found {
    let mut found = Found::default();
    for name in LEAK_SCENARIOS {
        compare(name, seeds.clone(), make, &mut found);
    }
    found
}

#[test]
fn what_a_side_cannot_know_changes_nothing_it_is_given() {
    let differences: [(&str, Make); 6] = [
        ("enemy ships carry a ton less fuel", less_fuel),
        ("enemy bases hold a ton more supply", more_supply),
        (
            "enemy rest thresholds are a point higher",
            higher_rest_threshold,
        ),
        ("the enemy gives an order that is refused", rejected_order),
        (
            "the enemy gives one more order on day 1",
            rejected_order_on_day_one,
        ),
        (
            "an enemy force far away steams one way, not another",
            far_unit_moves,
        ),
    ];
    for (name, make) in differences {
        let found = found_by(make, 1..=1);
        println!("{name}: {found:?}");
        assert!(
            found.changed.is_empty(),
            "{name}: the side was given something else: {:?}",
            found.changed
        );
        assert!(
            found.compared >= 4,
            "{name}: only {} runs compared",
            found.compared
        );
        assert!(
            found.enemy_saw_it * 2 >= found.compared,
            "{name}: the enemy's briefing differed in only {} of {} runs",
            found.enemy_saw_it,
            found.compared
        );
    }
}

#[test]
fn a_difference_the_side_can_meet_does_change_what_it_is_given() {
    // The control: one more aircraft a day for the enemy reaches the side in
    // the raids it meets. If nothing changed, the comparisons above would
    // prove nothing.
    let found = found_by(more_replacements, 1..=1);
    println!("one more replacement aircraft a day: {found:?}");
    assert!(found.compared >= 2);
    assert!(
        !found.changed.is_empty(),
        "more aircraft for the enemy changed nothing the side is given"
    );
}

// ---------------------------------------------------------------------
// 3. The library names nothing of the truth.
// ---------------------------------------------------------------------

/// What the lint allows (stage 9d, decision 0050). Code lines of every source
/// file of the library, doc comments left out, may not contain `umpire`,
/// `EventLog`, `TruthState`, `BeliefState`, `std::fs`, `std::net`, the hash
/// collections, `f32`, `f64` or `unsafe`, and may not contain the word `World`
/// with one exception: `session.rs`, the module that holds the game, may. That
/// file is held to a further list, below, so that holding the game does not
/// become a way to read it: it may not call the world's truth (`umpire`), its
/// full event log or order list, its map or reference data, or its beliefs
/// (`world.events`, `world.orders`, `world.order(`, `world.map`,
/// `world.ref_data`, `world.beliefs`, `world.truth`), and a view is only taken
/// with `side_view`. Every tool implementation outside `session.rs` takes a
/// `&SideView` and cannot name the world at all.
#[test]
fn the_library_code_names_nothing_of_the_truth() {
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let manifest = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"),
    )
    .expect("the manifest reads");
    let normal: &str = manifest.split("[dev-dependencies]").next().unwrap();
    for banned in ["sim-data", "sim-cli"] {
        assert!(
            !normal.contains(banned),
            "{banned} is a dependency of the library"
        );
    }
    // Words the library's code may not use anywhere. Doc comments are left
    // out.
    let banned = [
        "umpire",
        "EventLog",
        "TruthState",
        "BeliefState",
        "std::fs",
        "std::net",
        "HashMap",
        "HashSet",
        "f32",
        "f64",
        "unsafe",
    ];
    // The module that holds the game: `World` may be named there and only
    // there, and these may not.
    let session_banned = [
        "world.events",
        "world.orders",
        "world.order(",
        "world.map",
        "world.ref_data",
        "world.beliefs",
        "world.truth",
        "SystemTime",
        "Instant",
    ];
    let mut holders = 0;
    for entry in std::fs::read_dir(src).expect("src reads") {
        let path = entry.expect("an entry").path();
        let is_session = path.file_name().and_then(|n| n.to_str()) == Some("session.rs");
        let text = std::fs::read_to_string(&path).expect("a source reads");
        let mut names_world = false;
        for (i, line) in text.lines().enumerate() {
            let code = line.trim_start();
            if code.starts_with("//") {
                continue;
            }
            for word in banned {
                assert!(
                    !code.contains(word),
                    "{}:{} uses {word}: {line}",
                    path.display(),
                    i + 1
                );
            }
            if code.contains("World") {
                assert!(
                    is_session,
                    "{}:{} names World outside session.rs: {line}",
                    path.display(),
                    i + 1
                );
                names_world = true;
            }
            if is_session {
                for word in session_banned {
                    assert!(
                        !code.contains(word),
                        "{}:{} uses {word}: {line}",
                        path.display(),
                        i + 1
                    );
                }
            }
        }
        if names_world {
            holders += 1;
        }
    }
    assert_eq!(holders, 1, "exactly one module holds the world");
    let _ = (
        without_event_numbers as fn(&Value, &BTreeMap<u64, u64>) -> Value,
        Hex::new(0, 0),
    );
}

// ---------------------------------------------------------------------
// Findings of the outside review of Nov 13, 2026 (prompt 11). Kept switched
// off until the architect rules on each; run with `--ignored` to see them.
// ---------------------------------------------------------------------

#[test]
#[ignore = "outside review finding 4: a day that cannot be resolved answers end_day with the engine's own words, which name an enemy ship"]
fn a_day_that_cannot_be_resolved_names_no_enemy_unit() {
    // Scenario 12c with no orders, and an arrival the loader accepts that
    // puts 100 more aircraft into Japanese group 3 aboard carrier B (ship 3),
    // which has room for 36. Nothing of it is ever reported to the Allied
    // side, which plays and ends the day.
    let name = "12c_no_deck.toml";
    let text = scenario_text(name);
    let (head, _orders) = text
        .split_once("\n[[order]]")
        .expect("12c's orders come last");
    let text = format!("{head}\n[[arrival]]\nday = 1\nair_group = 3\naircraft = 100\n");
    let scenario = load_text(&text, name, 1942);
    let japanese: Vec<String> = scenario
        .world
        .umpire()
        .ships
        .values()
        .filter(|s| s.side == Side::Japanese)
        .map(|s| s.id.to_string())
        .collect();
    let (mut session, _) = support::play::session_of(scenario, &[Side::Allied]);
    let answer = session.call(Side::Allied, "end_day", &serde_json::json!({}));
    let said = match answer {
        Ok(value) => value.to_string(),
        Err(error) => error.message,
    };
    let named: Vec<&String> = japanese
        .iter()
        .filter(|handle| {
            said.match_indices(handle.as_str())
                .any(|(at, _)| !said[at + handle.len()..].starts_with(|c: char| c.is_ascii_digit()))
        })
        .collect();
    assert!(
        named.is_empty(),
        "the Allied player's end_day names enemy ships {named:?}: {said}"
    );
}
