//! Scenario 97: a seat's trial. A commander at his seat queues a landing
//! and, before a day is resolved, asks what it would cost. He is given
//! the estimate, in his own words and numbers; the game is as it was.
//!
//! Proves decision 0178, the first part of step 8 of step 3's stage 4,
//! against a hand calculation. The data is placeholder data: see
//! `data/scenarios/97_a_seats_trial.toml`, which the build writes to the
//! inputs below. It runs on the frozen step 1 figures, through a session
//! as `sim-cli` opens one, with the Allied seat played and the Japanese
//! seat scripted.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Inputs
//!
//! Scenario 95 (see its hand calculation), with one change: its file of
//! the trial's figures names the size `small` as 4 worlds for 8 days,
//! `standard` as 8 worlds for 8 days and `full` as 16 worlds for 8 days.
//! So: unit 1, 600 fighting men at A, and task force 1 of two troop
//! transports there; base 2 at B, Japanese, held in the live game by
//! 1,200 men; the Allied side's opening estimate of B is 200 men, and it
//! believes 400 more may be there, standing unknown; an enemy base is
//! assumed to hold 200 t of supply; scenario 68's ground figures, with
//! no die. The scenario has no commands.
//!
//! ## What the commander does
//!
//! On the evening before day 1, at the Allied seat, by the seat's own
//! tools:
//!
//! 1. `set_intent`: take the place at (4,0). It is his intent 1.
//! 2. `set_limit`, forces, on intent 1: task force 1, unit 1, and base 1
//!    as the source.
//! 3. `set_leave`: leave to launch, given.
//! 4. `trial`, of intent 1, at the size `small`.
//!
//! The three commands are queued for the morning and are not yet taken
//! in. Leave to prepare is the side's from the start.
//!
//! ## The course
//!
//! The standing commands are the three he queued, in the order queued;
//! the scenario has none of its own. Intent 1 is queued, so it may be
//! tried. Leave to prepare the side holds and leave to launch is among
//! the standing commands: the course is empty. The side has no other
//! intent, so no second trial is asked. The trial is trial 1, of 4
//! worlds for 8 days.
//!
//! This is scenario 95's trial: the same picture, the same beliefs and
//! figures, the same worlds, and the same four commands taken in on the
//! first morning (scenario 95's course gives leave to prepare as well,
//! which the side already has; there the launch leave was the course's
//! and was named as given by it, and here it is the commander's own).
//!
//! ## The answer
//!
//! Of the kind `trial`. Its estimate is number 1. What was asked:
//! intent 1, the size `small`, 4 worlds for 8 days. Its fields are
//! scenario 95's estimate of intent 1:
//!
//! - **How it went:** 4 of 4 worlds played; met and kept in 2, worlds 1
//!   and 3, the earliest, the middle and the latest day 4; met and lost
//!   again in none; not met by day 8 in 2, worlds 2 and 4.
//! - **What it cost the side:** men lost to the enemy 100, 100, 550;
//!   every other cost 0, 0, 0.
//! - **What the trial's enemy lost:** men lost to the enemy 200, 200,
//!   500; every other cost 0, 0, 0. (Restated by the architect on Nov
//!   20, 2026, for decision 0193: in scenario 95's worlds 2 and 4 unit 1
//!   attacks again on day 7 with nothing more coming, and the side loses
//!   500 and 550 there, the enemy 500 and 500. Before it these were 100,
//!   100, 300 and 200, 200, 300.)
//! - **What it needs and has:** men wanted 400, to hand 600; it waits
//!   for nothing; the day expected ashore is day 4; a count of the
//!   enemy at the place is asked for, and the one held is the opening
//!   estimate, 200 men, told on day 0, which does not meet it.
//! - **What its other plans lost:** the side has no other plan to
//!   weigh.
//! - **What was assumed:** the enemy's base at B with 200 t of supply
//!   and 0 t of fuel, in 4 worlds; 200 men at B from the side's
//!   estimate, in 4; 400 men believed at B, standing unknown, in 2 and
//!   not in 2; the enemy given no belief, in 4; the enemy given no
//!   intent, in 4. No leave is named as given by the course: the
//!   commander queued it himself.
//! - **Where the worlds part,** by the 400 men believed at B: with
//!   them, 2 worlds, met and kept in none, the side's men lost to the
//!   enemy 500, 500, 550; without them, 2 worlds, met and kept in 2 on
//!   day 4, the side's men lost 100, 100, 100. (Restated by the
//!   architect on Nov 20, 2026, for decision 0193: before it, with them,
//!   300, 300, 300.)
//!
//! In the answer the base is named as the seat's tools name a base, the
//! place as they give a place, the task under the plan order in the
//! words the tool `order` uses for it ("ready a landing at ... by ..."),
//! with no number of a task or a plan, and task force 1 and unit 1 by
//! the side's own numbers. The 200 men are "from your estimate of" the
//! place and the 400 "believed, standing unknown".
//!
//! Every number of a unit, a task force or a base in the answer, fields
//! and words, is one the seat's own tools give in the live game; no
//! run of digits in it is 900001 or 900002, with or without a comma.
//! Its words name "2 of 4 worlds"; they hold no per cent sign, neither
//! "chance" nor "points", and none of these marks of the engine's own
//! names: "TaskForceId", "LandUnitId", "PlanId", "Hex {", "Some(",
//! "None", "ReadyLanding", "TakePlace", "ready_landing", "take_place".
//!
//! ## Nothing is changed
//!
//! - The session's hash of the game is the same before the call and
//!   after it.
//! - The seat's queue holds the same three commands, and `command_state`
//!   and `staff_orders` answer as they did before the call.
//! - The day is then resolved as the commander meant, and days 1 to 4
//!   of the live game are those of the game in which `trial` was never
//!   called: the same daily hashes. (In the live game the 600 men meet
//!   1,200 on day 4, not the 200 or 600 of the trial: the trial was not
//!   told.)
//! - **The live enemy is not read.** The same scenario with 900 men at
//!   B in place of 1,200, and the same calls: the answer of `trial` is
//!   the same, byte for byte.
//! - **The other side's commands are not in it.** In a session with
//!   both seats played, the Japanese seat queues a command of its own
//!   before the Allied seat asks: the Allied answer is the same, byte
//!   for byte.
//!
//! # Properties
//!
//! - **Asked twice, the same.** A second `trial` of intent 1 at `small`
//!   on the same evening is answered with the same fields and words, as
//!   estimate 2. `trials` then lists two, each with the evening it
//!   was asked on and what was asked; and gives estimate 1 again by its
//!   number, equal to the first answer.
//! - **A larger size is more of the same worlds.** At `standard`, 8
//!   worlds: met and kept in 4, worlds 1, 3, 5 and 7; not met in 4;
//!   the side's men lost to the enemy 100, 100, 550, as the answer gives
//!   the spread (the answer gives no world's own figures, so which of
//!   worlds 6 and 8 lost 550, if either, is not stated here). Worlds 1
//!   to 4 of it have the figures they had at `small`. (Restated by the
//!   architect on Nov 20, 2026, for decision 0193: before it, 100, 100,
//!   300.)
//! - **A draft, tried.** On a fresh evening the commander asks
//!   `preview` of the `set_intent` alone, and then `trial` of that
//!   preview. It is tried and answered; nothing is queued; the preview
//!   is not spent and may still be approved. Its figures are reported
//!   by the build for the architect to state, and not asserted here:
//!   with no forces limit the staff assembles the force itself
//!   (decision 0149), and with leave to launch given by the course and
//!   not by the side, which the answer names.
//! - **An intent in force.** With the three commands taken in on day 1
//!   and the trial asked on the evening of day 1: the standing commands
//!   are none, the course is empty, and the trial is of days 2 to 9.
//!   The landing is ashore on day 4 in every world as before: met and
//!   kept in 2 of 4 on day 4, not met by day 9 in 2.
//! - **Refused, each with its own words and nothing played:**
//!   - while a day is open, after the night pulse of day 1 with the
//!     clock stopped (in a copy of the scenario with orders inside a
//!     day on);
//!   - after the last day, the game being over;
//!   - intent 1 once its withdrawal is queued;
//!   - with both `intent` and `preview`, and with neither;
//!   - intent 7, which the side does not have;
//!   - the staff's own intent, the landing it formed, on the evening of
//!     day 1;
//!   - a preview that is void, a command having been queued after it;
//!   - the size `vast`.
//! - **The tools `trial` and `trials` are listed** for a seat with a
//!   staff that plays a whole side in a game that has a trial. At a
//!   command's seat, and in a game that names no file of the trial's
//!   figures, they are not listed and a call is answered as a call of
//!   any tool the seat does not have.
//! - **Leave given by the course is the side's.** Without step 3: the
//!   course gives leave to launch, the answer names it as "leave given
//!   to the side", and the figures are those above.
//! - **The session is built in this test as `sim-cli` builds one,** on
//!   the frozen figures. A test of `sim-cli` proves that it hands a
//!   runner in exactly where the scenario names a trial's figures.
//! - **The agent's library names no picture and no world built from
//!   one:** its source test passes unchanged in what it bans.
//! - **A game with no trial is as it was:** the recorded games replay
//!   as they did, but for the version of the agent's interface in each
//!   answer; and every tool listed for them is one listed before.

mod common;

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::OnceLock;

use serde_json::{json, Value};
use sim_agent::{CallError, Seat, Session};
use sim_core::state::Side;
use sim_data::Scenario;
use sim_staff::staffs;

#[allow(dead_code)]
#[path = "../crates/sim-agent/tests/support/validator.rs"]
mod validator;

const ALLIED: Side = Side::Allied;

fn path() -> PathBuf {
    common::data_dir().join("scenarios/97_a_seats_trial.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn load(text: &str) -> Scenario {
    common::from_text(text, &path()).expect("the scenario loads")
}

/// A session over `scenario` built as `sim-cli` builds one (`serve::open`):
/// the staffs, the scenario's commands, its orders inside a day, and what
/// plays a seat's trials where the scenario names a trial's figures. The
/// seats in `players` are played; the others are the script's.
fn session_with(mut scenario: Scenario, players: &[Side]) -> Session {
    let runner = sim_cli::seat_trial::SeatTrials::runner_of(&scenario);
    let commands = scenario.scripted_commands().clone();
    scenario.replace_commands(BTreeMap::new());
    let inside = scenario.orders_inside_a_day();
    let script_inside = scenario.scripted_orders_inside_day().clone();
    let name = scenario.name.clone();
    let seat = |side: Side| {
        if players.contains(&side) {
            Seat::Player
        } else {
            Seat::Script(BTreeMap::new())
        }
    };
    let mut session = Session::new(
        scenario.world,
        &name,
        scenario.days,
        seat(Side::Allied),
        seat(Side::Japanese),
    )
    .with_staffs(staffs())
    .with_commands(commands)
    .with_orders_inside_a_day(inside, script_inside);
    if let Some(runner) = runner {
        session = session.with_trial(runner);
    }
    session
}

fn session() -> Session {
    session_with(load(&text()), &[ALLIED])
}

fn call(session: &mut Session, tool: &str, args: Value) -> Result<Value, CallError> {
    session.call(ALLIED, tool, &args)
}

fn ask(session: &mut Session, tool: &str, args: Value) -> Value {
    call(session, tool, args.clone()).unwrap_or_else(|e| panic!("{tool} {args}: {}", e.message))
}

/// The commander's three commands of the evening before day 1.
fn queue_the_three(session: &mut Session) {
    for (tool, args) in [
        (
            "set_intent",
            json!({"intent_kind": "take_place", "hex": [4, 0]}),
        ),
        (
            "set_limit",
            json!({"limit": "forces", "intent": 1, "task_forces": [1], "land_units": [1], "source": 1}),
        ),
        ("set_leave", json!({"leave": "launch", "given": true})),
    ] {
        let a = ask(session, tool, args);
        assert_eq!(a["status"], "queued", "{tool}: {a}");
    }
}

/// The answer checked against its schema.
fn checked(answer: &Value) {
    let schemas = validator::Schemas::load(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../crates/sim-agent/schemas"),
    );
    let kind = answer["kind"].as_str().expect("a kind");
    let errors = schemas.check(&format!("{kind}.schema.json"), answer);
    assert!(errors.is_empty(), "{errors:?}\n{answer}");
}

/// The words of an answer, one string.
fn words(answer: &Value) -> String {
    answer["text"]
        .as_array()
        .expect("words")
        .iter()
        .map(|line| line.as_str().expect("a line"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The main case: the three commands queued, then `trial` of intent 1 at
/// `small`; the session after, and the answer.
struct Main {
    answer: Value,
    hash_before: u64,
    hash_after: u64,
    state_before: Value,
    state_after: Value,
    orders_before: Value,
    orders_after: Value,
}

fn main_case() -> &'static Main {
    static MAIN: OnceLock<Main> = OnceLock::new();
    MAIN.get_or_init(|| {
        let mut s = session();
        queue_the_three(&mut s);
        let hash_before = s.state_hash();
        let state_before = ask(&mut s, "command_state", json!({}));
        let orders_before = ask(&mut s, "staff_orders", json!({}));
        let answer = ask(&mut s, "trial", json!({"intent": 1, "size": "small"}));
        Main {
            answer,
            hash_before,
            hash_after: s.state_hash(),
            state_after: ask(&mut s, "command_state", json!({})),
            orders_after: ask(&mut s, "staff_orders", json!({})),
            state_before,
            orders_before,
        }
    })
}

fn spread(lowest: u64, middle: u64, highest: u64) -> Value {
    json!({"lowest": lowest, "middle": middle, "highest": highest})
}

fn costs(men: Value) -> Value {
    json!({
        "men_lost_to_the_enemy": men,
        "men_lost_unfed": spread(0, 0, 0),
        "ships_sunk": spread(0, 0, 0),
        "ships_out_of_action": spread(0, 0, 0),
        "aircraft_lost": spread(0, 0, 0),
    })
}

// ---------------------------------------------------------------- Inputs

#[test]
fn the_inputs_are_scenario_95_with_its_sizes() {
    let s = load(&text());
    assert!(
        s.scripted_commands().is_empty(),
        "the scenario has no commands"
    );
    let figures = s.trial_figures().expect("it names a trial's figures");
    let sizes: Vec<(&str, u32, u32)> = figures
        .sizes()
        .iter()
        .map(|(name, size)| (*name, size.worlds, size.days))
        .collect();
    assert_eq!(
        sizes,
        [("small", 4, 8), ("standard", 8, 8), ("full", 16, 8)]
    );
    assert_eq!(figures.seed, 1942);
    // Scenario 95's file, but for the name, the file of sides and the
    // trial's figures.
    let ninety_five =
        std::fs::read_to_string(common::data_dir().join("scenarios/95_the_estimate.toml")).unwrap();
    let body = |t: &str| -> Vec<String> {
        t.lines()
            .filter(|l| !l.starts_with('#') && !l.starts_with("name ="))
            .filter(|l| !l.starts_with("class_sides") && !l.starts_with("trial ="))
            .filter(|l| !l.starts_with("note ="))
            .map(str::to_string)
            .collect()
    };
    assert_eq!(body(&text()), body(&ninety_five));
}

// ---------------------------------------------------------------- The course

#[test]
fn the_standing_commands_are_the_three_queued_and_the_course_is_empty() {
    let a = &main_case().answer;
    let asked = &a["asked"];
    assert_eq!(asked["intent"], 1);
    assert_eq!(asked["preview"], Value::Null);
    assert_eq!(asked["tried_intent"], 1);
    assert_eq!(asked["size"], "small");
    assert_eq!(
        (asked["worlds"].clone(), asked["days"].clone()),
        (json!(4), json!(8))
    );
    assert_eq!(asked["for_day"], 1);
    assert_eq!(
        asked["standing"],
        json!([
            "set intent 1: take the place at (4,0)",
            "limit intent 1's forces to task_force:1 \"Placeholder transport force 1\", land_unit:1 \"Placeholder First\", from base:1 \"Placeholder base A\"",
            "give leave to launch"
        ])
    );
    assert_eq!(asked["course"], json!([]), "the course is empty");
    assert_eq!(
        asked["second_trial"], false,
        "no other intent: no second trial"
    );
    assert_eq!(
        (a["first_day"].clone(), a["last_day"].clone()),
        (json!(1), json!(8))
    );
}

// ---------------------------------------------------------------- The answer

#[test]
fn the_answer_is_scenario_95s_estimate_of_intent_1() {
    let a = &main_case().answer;
    checked(a);
    assert_eq!(a["kind"], "trial");
    assert_eq!(a["version"], sim_agent::VERSION);
    assert_eq!(a["estimate"], 1);
    assert_eq!(a["has_end"], true);
    assert_eq!(
        a["how_it_went"],
        json!({
            "worlds": [1, 2, 3, 4],
            "met_and_kept": [1, 3],
            "met_days": {"earliest": 4, "middle": 4, "latest": 4},
            "met_and_lost_again": [],
            "not_met": [2, 4],
        })
    );
    assert_eq!(a["could_not_be_played"], json!([]));
    assert_eq!(a["cost"], costs(spread(100, 100, 550)));
    assert_eq!(a["enemy_lost"], costs(spread(200, 200, 500)));
    let needs = &a["needs"];
    assert_eq!(needs["men"], json!({"wanted": 400, "to_hand": 600}));
    assert_eq!(needs["waits_for"], json!([]), "it waits for nothing");
    assert_eq!(needs["expected_ashore"], 4);
    assert_eq!(needs["count"]["asked"], "count");
    assert_eq!(
        needs["count"]["held"],
        json!({"men": 200, "came_from": "told", "day": 0})
    );
    assert_eq!(needs["count"]["met"], false);
    assert_eq!(
        (needs["world"].clone(), needs["at_the_end_of_day"].clone()),
        (json!(1), json!(1))
    );
    assert_eq!(needs["differs_in"], json!([]));
    assert_eq!(a["other_plans"], Value::Null, "no second trial");
    assert_eq!(a["leaves_given_to_the_side"], json!([]));
    assert_eq!(a["uneven"], false);
    let w = words(a);
    assert!(w.contains("Your side has no other plan to weigh"), "{w}");
    assert!(w.contains("No leave was given by the trial"), "{w}");
}

#[test]
fn what_was_assumed() {
    let a = &main_case().answer;
    let lines = a["assumed"].as_array().unwrap();
    let got: Vec<(Value, Value, Value, Value)> = lines
        .iter()
        .map(|l| {
            (
                l["thing"].clone(),
                l["from"]["kind"].clone(),
                l["made_in"].clone(),
                l["not_in"].clone(),
            )
        })
        .collect();
    let place = |q: i32, r: i32| json!({"q": q, "r": r, "lat": null, "lon": null, "name": null});
    assert_eq!(
        got,
        vec![
            (
                json!({"kind": "enemy_base", "base": {"handle": "base:2", "name": "Placeholder airfield B"}, "place": place(4, 0), "supply_t": "200", "fuel_t": "0"}),
                json!("map"),
                json!([1, 2, 3, 4]),
                json!([])
            ),
            (
                json!({"kind": "men", "place": place(4, 0), "men": 200}),
                json!("estimate"),
                json!([1, 2, 3, 4]),
                json!([])
            ),
            (
                json!({"kind": "men", "place": place(4, 0), "men": 400}),
                json!("believed"),
                json!([2, 4]),
                json!([1, 3])
            ),
            (
                json!({"kind": "no_belief"}),
                json!("trial"),
                json!([1, 2, 3, 4]),
                json!([])
            ),
            (
                json!({"kind": "no_intent"}),
                json!("trial"),
                json!([1, 2, 3, 4]),
                json!([])
            ),
        ]
    );
    assert_eq!(lines[2]["from"]["standing"], "unknown");
    let w = words(a);
    assert!(
        w.contains("200 men at (4,0), from your estimate of the place"),
        "{w}"
    );
    assert!(
        w.contains("400 men at (4,0), believed, standing unknown"),
        "{w}"
    );
}

#[test]
fn where_the_worlds_part() {
    let a = &main_case().answer;
    let partings = a["partings"].as_array().unwrap();
    assert_eq!(partings.len(), 1);
    let p = &partings[0];
    assert_eq!(p["by"]["men"], 400);
    assert_eq!(p["from"]["kind"], "believed");
    assert_eq!(p["with"]["how_it_went"]["worlds"], json!([2, 4]));
    assert_eq!(p["with"]["how_it_went"]["met_and_kept"], json!([]));
    assert_eq!(
        p["with"]["cost"]["men_lost_to_the_enemy"],
        spread(500, 500, 550)
    );
    assert_eq!(p["without"]["how_it_went"]["worlds"], json!([1, 3]));
    assert_eq!(p["without"]["how_it_went"]["met_and_kept"], json!([1, 3]));
    assert_eq!(
        p["without"]["how_it_went"]["met_days"],
        json!({"earliest": 4, "middle": 4, "latest": 4})
    );
    assert_eq!(
        p["without"]["cost"]["men_lost_to_the_enemy"],
        spread(100, 100, 100)
    );
}

#[test]
fn things_are_named_as_the_seats_tools_name_them() {
    let a = &main_case().answer;
    let task = &a["needs"]["tasks"][0];
    assert_eq!(a["needs"]["tasks"].as_array().unwrap().len(), 1);
    assert_eq!(
        task["text"],
        "ready a landing at (4,0) by task_force:1 \"Placeholder transport force 1\""
    );
    assert_eq!(task["force"]["task_force"]["handle"], "task_force:1");
    assert_eq!(task["force"]["formed"], false);
    assert_eq!(task["place"]["q"], 4);
    // The base as list_bases names a base; the place as the tools give one.
    let mut s = session();
    let bases = ask(&mut s, "list_bases", json!({})).to_string();
    assert!(bases.contains("\"base:2\""), "{bases}");
    let w = words(a);
    assert!(
        w.contains("base:2 \"Placeholder airfield B\" at (4,0)"),
        "{w}"
    );
    assert!(w.contains("land_unit:1 \"Placeholder First\""), "{w}");
    // No number of a task or a plan.
    assert!(task.get("intent").is_none() && task.get("plan").is_none());
}

/// Every handle written in a JSON value or a text, kind and number.
fn handles(answer: &Value) -> BTreeSet<(String, u32)> {
    let text = answer.to_string();
    let mut out = BTreeSet::new();
    for kind in ["task_force", "land_unit", "ship", "air_group", "base"] {
        let mark = format!("{kind}:");
        for (at, _) in text.match_indices(&mark) {
            let digits: String = text[at + mark.len()..]
                .chars()
                .take_while(char::is_ascii_digit)
                .collect();
            if let Ok(n) = digits.parse() {
                out.insert((kind.to_string(), n));
            }
        }
    }
    out
}

#[test]
fn every_number_in_the_answer_is_the_live_views() {
    let a = &main_case().answer;
    let live = load(&text());
    let view = live.world.side_view(ALLIED);
    let count = |n: usize| -> u32 { u32::try_from(n).unwrap() };
    for (kind, number) in handles(a) {
        let known = match kind.as_str() {
            "task_force" => (1..=count(view.task_forces().count())).contains(&number),
            "land_unit" => (1..=count(view.land_units().count())).contains(&number),
            "ship" => (1..=count(view.ships().count())).contains(&number),
            "air_group" => (1..=count(view.air_groups().count())).contains(&number),
            _ => view
                .bases()
                .map(|b| b.id.raw())
                .chain(view.enemy_bases().map(|b| b.id.raw()))
                .any(|b| b == number),
        };
        assert!(known, "{kind}:{number} is not a number the live view gives");
    }
    assert!(!handles(a).is_empty());
    // No run of digits is a number kept for units made for the enemy.
    let flat = a.to_string().replace(',', "");
    for made in ["900001", "900002"] {
        assert!(!flat.contains(made), "{made} in the answer");
    }
}

#[test]
fn its_words_count_worlds_and_hold_no_name_of_the_engines() {
    let w = words(&main_case().answer);
    assert!(w.contains("2 of 4 worlds"), "{w}");
    for banned in [
        "%",
        "chance",
        "points",
        "TaskForceId",
        "LandUnitId",
        "PlanId",
        "Hex {",
        "Some(",
        "None",
        "ReadyLanding",
        "TakePlace",
        "ready_landing",
        "take_place",
    ] {
        assert!(!w.contains(banned), "{banned:?} in the words:\n{w}");
    }
    // In the order of rule 17.
    let order = [
        "Tried:",
        "How it went",
        "What it cost your side",
        "What the trial's enemy lost",
        "What it needs and has",
        "no other plan",
        "What was assumed",
        "Where the worlds part",
    ];
    let at: Vec<usize> = order.iter().map(|o| w.find(o).expect(o)).collect();
    assert!(at.windows(2).all(|p| p[0] < p[1]), "{w}");
}

// ---------------------------------------------------------------- Nothing is changed

#[test]
fn nothing_is_changed_by_asking() {
    let m = main_case();
    assert_eq!(m.hash_before, m.hash_after);
    assert_eq!(m.state_before, m.state_after);
    assert_eq!(m.orders_before, m.orders_after);
    assert_eq!(m.state_after["queued"].as_array().map(Vec::len), Some(3));
}

#[test]
fn the_days_after_are_those_of_a_game_with_no_trial() {
    let play = |trial: bool| -> Vec<u64> {
        let mut s = session();
        queue_the_three(&mut s);
        if trial {
            ask(&mut s, "trial", json!({"intent": 1}));
        }
        for _ in 0..4 {
            ask(&mut s, "end_day", json!({}));
        }
        s.day_hashes().to_vec()
    };
    let with = play(true);
    assert_eq!(with.len(), 4);
    assert_eq!(with, play(false));
}

#[test]
fn the_live_enemy_is_not_read() {
    let answer_of = |text: &str| -> String {
        let mut s = session_with(load(text), &[ALLIED]);
        queue_the_three(&mut s);
        ask(&mut s, "trial", json!({"intent": 1})).to_string()
    };
    let other = common::replace_once(&text(), "men = 1200", "men = 900");
    assert_eq!(answer_of(&text()), answer_of(&other), "byte for byte");
}

#[test]
fn the_other_sides_commands_are_not_in_it() {
    let mut both = session_with(load(&text()), &[ALLIED, Side::Japanese]);
    let queued = both
        .call(
            Side::Japanese,
            "set_leave",
            &json!({"leave": "launch", "given": true}),
        )
        .expect("answered");
    assert_eq!(queued["status"], "queued", "{queued}");
    queue_the_three(&mut both);
    let with_both = ask(&mut both, "trial", json!({"intent": 1}));
    let mut one = session();
    queue_the_three(&mut one);
    let alone = ask(&mut one, "trial", json!({"intent": 1}));
    assert_eq!(with_both.to_string(), alone.to_string(), "byte for byte");
}

// ---------------------------------------------------------------- Properties

#[test]
fn asked_twice_the_same_and_kept() {
    let mut s = session();
    queue_the_three(&mut s);
    let first = ask(&mut s, "trial", json!({"intent": 1, "size": "small"}));
    let second = ask(&mut s, "trial", json!({"intent": 1, "size": "small"}));
    assert_eq!(second["estimate"], 2);
    let strip = |a: &Value| {
        let mut a = a.clone();
        a.as_object_mut().unwrap().remove("estimate");
        a
    };
    assert_eq!(strip(&first), strip(&second), "the same fields and words");
    let listed = ask(&mut s, "trials", json!({}));
    checked(&listed);
    let estimates = listed["estimates"].as_array().unwrap();
    assert_eq!(estimates.len(), 2);
    for (n, e) in (1..).zip(estimates) {
        assert_eq!(e["estimate"], n);
        assert_eq!(e["asked_on"], json!({"days_resolved": 0, "for_day": 1}));
        assert_eq!(e["asked"], first["asked"]);
        assert_eq!(e["standing"], "current");
    }
    assert_eq!(ask(&mut s, "trials", json!({"estimate": 1})), first);
    // Marked once the queue changes, and once a day is resolved.
    ask(
        &mut s,
        "set_leave",
        json!({"leave": "prepare", "given": true}),
    );
    let listed = ask(&mut s, "trials", json!({}));
    assert_eq!(listed["estimates"][0]["standing"], "queue_changed");
    ask(&mut s, "end_day", json!({}));
    let listed = ask(&mut s, "trials", json!({}));
    assert_eq!(listed["estimates"][1]["standing"], "earlier_evening");
    assert_eq!(
        ask(&mut s, "trials", json!({"estimate": 1})),
        first,
        "unchanged"
    );
}

#[test]
fn a_larger_size_is_more_of_the_same_worlds() {
    let mut s = session();
    queue_the_three(&mut s);
    let small = ask(&mut s, "trial", json!({"intent": 1}));
    let standard = ask(&mut s, "trial", json!({"intent": 1, "size": "standard"}));
    checked(&standard);
    assert_eq!(
        (
            standard["asked"]["worlds"].clone(),
            standard["asked"]["days"].clone()
        ),
        (json!(8), json!(8))
    );
    let went = &standard["how_it_went"];
    assert_eq!(went["met_and_kept"], json!([1, 3, 5, 7]));
    assert_eq!(went["not_met"], json!([2, 4, 6, 8]));
    assert_eq!(
        standard["cost"]["men_lost_to_the_enemy"],
        spread(100, 100, 550)
    );
    // Worlds 1 to 4 have the figures they had at small: what each world
    // assumed, and how each went, world by world.
    let believed = |a: &Value| a["assumed"][2]["made_in"].clone();
    assert_eq!(believed(&small), json!([2, 4]));
    assert_eq!(believed(&standard), json!([2, 4, 6, 8]));
    for p in standard["partings"].as_array().unwrap() {
        for side in ["with", "without"] {
            let worlds: Vec<u64> = p[side]["how_it_went"]["worlds"]
                .as_array()
                .unwrap()
                .iter()
                .map(|w| w.as_u64().unwrap())
                .collect();
            assert!(worlds.iter().all(|w| (w % 2 == 0) == (side == "with")));
        }
        assert_eq!(p["with"]["cost"], small["partings"][0]["with"]["cost"]);
        assert_eq!(
            p["without"]["cost"],
            small["partings"][0]["without"]["cost"]
        );
    }
}

#[test]
fn a_draft_tried() {
    let mut s = session();
    let preview = ask(
        &mut s,
        "preview",
        json!({"command": "set_intent", "intent_kind": "take_place", "hex": [4, 0]}),
    );
    let number = preview["preview"].as_u64().expect("a preview number");
    let hash = s.state_hash();
    let a = ask(&mut s, "trial", json!({"preview": number}));
    checked(&a);
    assert_eq!(a["asked"]["preview"], number);
    assert_eq!(a["asked"]["tried_intent"], 1);
    assert_eq!(a["leaves_given_to_the_side"], json!(["launch"]));
    assert!(words(&a).contains("Leave to launch given to the side by the trial"));
    assert_eq!(s.state_hash(), hash);
    let state = ask(&mut s, "command_state", json!({}));
    assert_eq!(state["queued"], json!([]), "nothing is queued: {state}");
    // Reported for the architect, not asserted.
    println!("A DRAFT TRIED:\n{}\n{}", words(&a), a["needs"]);
    let approved = ask(&mut s, "approve", json!({"preview": number}));
    assert_eq!(
        approved["status"], "queued",
        "the preview may still be approved: {approved}"
    );
}

#[test]
fn an_intent_in_force() {
    let mut s = session();
    queue_the_three(&mut s);
    ask(&mut s, "end_day", json!({}));
    let a = ask(&mut s, "trial", json!({"intent": 1}));
    checked(&a);
    assert_eq!(a["asked"]["standing"], json!([]));
    assert_eq!(a["asked"]["course"], json!([]));
    assert_eq!(
        (a["first_day"].clone(), a["last_day"].clone()),
        (json!(2), json!(9))
    );
    assert_eq!(
        a["how_it_went"]["met_and_kept"].as_array().unwrap().len(),
        2
    );
    assert_eq!(
        a["how_it_went"]["met_days"],
        json!({"earliest": 4, "middle": 4, "latest": 4})
    );
    assert_eq!(a["how_it_went"]["not_met"].as_array().unwrap().len(), 2);
    assert!(
        words(&a).contains("not met by day 9 in 2 of 4 worlds"),
        "{}",
        words(&a)
    );
}

fn refusal(s: &mut Session, args: Value) -> String {
    let before = s.state_hash();
    let shown = s.transcript().len();
    let e = call(s, "trial", args.clone()).expect_err(&format!("refused: {args}"));
    assert_eq!(s.state_hash(), before);
    assert_eq!(s.transcript().len(), shown + 1);
    e.message
}

#[test]
fn refused_each_with_its_own_words() {
    let mut s = session();
    queue_the_three(&mut s);
    let mut seen = Vec::new();
    seen.push(refusal(&mut s, json!({"intent": 1, "preview": 1})));
    seen.push(refusal(&mut s, json!({})));
    seen.push(refusal(&mut s, json!({"intent": 7})));
    seen.push(refusal(&mut s, json!({"intent": 1, "size": "vast"})));
    assert!(seen[0].contains("not both"), "{}", seen[0]);
    assert!(
        seen[1].contains("\"intent\" or \"preview\" is needed"),
        "{}",
        seen[1]
    );
    assert!(
        seen[2].contains("intent 7 is not an intent of yours"),
        "{}",
        seen[2]
    );
    assert!(seen[3].contains("no size \"vast\""), "{}", seen[3]);
    // A void preview: a command queued after it.
    let p = ask(
        &mut s,
        "preview",
        json!({"command": "set_intent", "intent_kind": "take_place", "hex": [4, 0]}),
    );
    ask(
        &mut s,
        "set_leave",
        json!({"leave": "prepare", "given": true}),
    );
    let void = refusal(&mut s, json!({"preview": p["preview"]}));
    assert!(void.contains("is void"), "{void}");
    // The staff's own intent, the landing it formed, on the evening of day 1.
    ask(&mut s, "end_day", json!({}));
    let state = ask(&mut s, "command_state", json!({}));
    let staffs = state["intents"]
        .as_array()
        .unwrap()
        .iter()
        .find(|i| i["maker"] == "staff")
        .unwrap_or_else(|| panic!("a staff intent: {state}"))["number"]
        .as_u64()
        .unwrap();
    let own = refusal(&mut s, json!({"intent": staffs}));
    assert!(own.contains("is the staff's own"), "{own}");
    // Intent 1 once its withdrawal is queued.
    ask(&mut s, "withdraw_intent", json!({"intent": 1}));
    let withdrawn = refusal(&mut s, json!({"intent": 1}));
    assert!(withdrawn.contains("withdrawal queued"), "{withdrawn}");
    seen.extend([void, own, withdrawn]);
    let distinct: BTreeSet<&String> = seen.iter().collect();
    assert_eq!(distinct.len(), seen.len(), "each with its own words");
}

#[test]
fn refused_after_the_last_day() {
    let mut s = session();
    while s.day() < s.days() {
        ask(&mut s, "end_day", json!({}));
    }
    assert!(refusal(&mut s, json!({"intent": 1})).contains("the game is over"));
}

#[test]
#[ignore = "superseded by decision 0204: a trial is answered inside a day, at 0600 and 1200 (scenario 118)"]
fn refused_while_a_day_is_open() {
    let open = format!("orders_inside_a_day = true\n{}", text());
    let mut s = session_with(load(&open), &[ALLIED]);
    queue_the_three(&mut s);
    let advanced = ask(&mut s, "advance", json!({"pulses": 1}));
    assert_eq!(advanced["clock"]["at"], "0600", "{advanced}");
    let words = refusal(&mut s, json!({"intent": 1}));
    assert!(words.contains("asked at dusk"), "{words}");
}

#[test]
fn the_tools_are_listed_only_where_there_is_a_trial() {
    let s = session();
    let tools = s.tools(ALLIED);
    assert!(
        tools.contains(&"trial") && tools.contains(&"trials"),
        "{tools:?}"
    );
    // In a game that names no file of the trial's figures.
    let none: String = text()
        .lines()
        .filter(|l| !l.starts_with("trial ="))
        .collect::<Vec<_>>()
        .join("\n");
    let mut plain = session_with(load(&none), &[ALLIED]);
    assert!(!plain.has_trial());
    let listed = plain.tools(ALLIED);
    assert!(!listed.contains(&"trial") && !listed.contains(&"trials"));
    let e = call(&mut plain, "trial", json!({"intent": 1})).expect_err("not a tool");
    assert!(
        e.message.contains("\"trial\" is not a tool"),
        "{}",
        e.message
    );
    // At a command's seat.
    let levels = format!("command_levels = true\n{}", text());
    let at_force = session_with(load(&levels), &[ALLIED])
        .with_post(
            ALLIED,
            sim_agent::Post::At(sim_core::pictures::CommandRef::TaskForce(
                sim_core::ids::TaskForceId::new(1),
            )),
        )
        .expect("a seat at task force 1");
    assert!(!at_force.tools(ALLIED).contains(&"trial"));
    let mut at_force = at_force;
    let e = call(&mut at_force, "trial", json!({"intent": 1})).expect_err("not a tool");
    assert!(
        e.message.contains("\"trial\" is not a tool"),
        "{}",
        e.message
    );
    // The area seat with command levels on is tried on the whole side's view.
    let mut area = session_with(load(&levels), &[ALLIED]);
    queue_the_three(&mut area);
    let a = ask(&mut area, "trial", json!({"intent": 1}));
    assert_eq!(a["how_it_went"]["worlds"], json!([1, 2, 3, 4]));
    println!("AREA SEAT, LEVELS ON: {}", a["how_it_went"]);
}

#[test]
fn leave_given_by_the_course_is_the_sides() {
    let mut s = session();
    for (tool, args) in [
        (
            "set_intent",
            json!({"intent_kind": "take_place", "hex": [4, 0]}),
        ),
        (
            "set_limit",
            json!({"limit": "forces", "intent": 1, "task_forces": [1], "land_units": [1], "source": 1}),
        ),
    ] {
        ask(&mut s, tool, args);
    }
    let a = ask(&mut s, "trial", json!({"intent": 1}));
    assert_eq!(a["asked"]["course"], json!(["give leave to launch"]));
    assert_eq!(a["leaves_given_to_the_side"], json!(["launch"]));
    assert!(words(&a).contains("given to the side"));
    let main = &main_case().answer;
    for field in [
        "how_it_went",
        "cost",
        "enemy_lost",
        "needs",
        "assumed",
        "partings",
    ] {
        assert_eq!(a[field], main[field], "{field}");
    }
}

/// Not a statement of the hand calculation: the second trial is asked where
/// the side has another intent of the commander's with an end (rule 11).
/// Its figures are reported, not asserted.
#[test]
fn a_second_trial_where_there_is_another_plan() {
    let mut s = session();
    queue_the_three(&mut s);
    let held = ask(
        &mut s,
        "set_intent",
        json!({"intent_kind": "hold", "base": 1}),
    );
    assert_eq!(held["status"], "queued", "{held}");
    let a = ask(&mut s, "trial", json!({"intent": 1}));
    checked(&a);
    assert_eq!(a["asked"]["second_trial"], true);
    let plans = a["other_plans"].as_array().expect("other plans weighed");
    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0]["intent"], 2);
    println!("A SECOND TRIAL: {}", words(&a));
}
