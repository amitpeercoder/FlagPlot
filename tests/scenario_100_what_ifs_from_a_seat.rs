//! Scenario 100: what-ifs from a seat. A commander at his seat queues a
//! landing and asks three things of it before a day is resolved: what if
//! I give it later, what if I hold it a day, and what does it cost me.
//! He is answered with estimates set side by side; the game is as it was.
//!
//! Proves decision 0181, the second part of step 8 of step 3's stage 4,
//! against a hand calculation. The data is placeholder data: see
//! `data/scenarios/100_what_ifs_from_a_seat.toml`, which the build writes
//! to the inputs below. It runs on the frozen step 1 figures with
//! scenario 96's ground figures, through a session as `sim-cli` opens
//! one, with the Allied seat played and the Japanese seat scripted.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Figures
//!
//! Scenario 96's (see its hand calculation): one shot for each 100
//! firing men, every shot hits and takes 50 men; a side is beaten where
//! the other attacked and it has no man left, or fewer than 0.2 of the
//! other's, and beaten men at B surrender; a unit on the defence digs 10
//! of fieldworks at the end of each day; in a fight a side's loss is
//! held to the men it has, shared between its units, and each unit's
//! part cut by its cover over 100 times 0.5, to a whole man.
//!
//! ## Inputs
//!
//! Scenario 96's second case, "worlds that differ":
//!
//! - Task force 1, two troop transports, and base 1 at A, Allied. Unit
//!   1, 600 fighting men at A, usable from day 1. Unit 2, 600 fighting
//!   men at A, usable from day 4.
//! - Base 2 at B, (4,0), Japanese, with the live garrison scenario 96's
//!   file gives it.
//! - The Allied side holds no count of B. It believes 200 men are at B,
//!   standing present, and 200 more may be there, standing unknown.
//! - No commands in the scenario.
//! - The trial's figures: an enemy base is assumed to hold 200 t of
//!   supply and 0 t of fuel; the seed is 1942; the size `small` is 4
//!   worlds for 5 days, `standard` 8 worlds for 5 days and `full` 16
//!   worlds for 5 days.
//!
//! So in a trial's worlds 1 and 3 the garrison of B is 200 men, and in
//! worlds 2 and 4 two units of 200.
//!
//! ## What the commander queues
//!
//! On the evening before day 1, at the Allied seat:
//!
//! 1. `set_intent`: take the place at (4,0). It is his intent 1.
//! 2. `set_limit`, forces, on intent 1: task force 1, units 1 and 2, and
//!    base 1 as the source.
//! 3. `set_leave`: leave to launch, given.
//!
//! Leave to prepare is the side's from the start. The side has no other
//! intent.
//!
//! ## A. What if I give it later: the window
//!
//! He asks `trial` of intent 1 with `dates` chosen, at `small`.
//!
//! **What is handed in at each date:** commands 1 and 2, which set the
//! intent and its limit. Command 3, the leave, is a standing command,
//! handed in on day 1 at every date. The course gives no leave: the side
//! holds leave to prepare and the commander has queued leave to launch.
//!
//! **The dates.** The first is day 1. Of what the forces limit names,
//! unit 2 may first be used on day 4, within the 5 days: one later date,
//! day 4. The first date is played to day 5 and the second to day 8.
//!
//! **The answer** is of the kind `window`, and is scenario 96's window
//! of its second case:
//!
//! | | The first date, day 1 | The second date, day 4 |
//! | --- | --- | --- |
//! | Its course taken in on | day 1 | day 4 |
//! | What came due | nothing | unit 2, usable from day 4 |
//! | The men its take chose | 600, unit 1 | 1,200, units 1 and 2 |
//! | Men wanted and to hand | none: no count is held | none |
//! | Day expected ashore | day 4 | day 8 |
//! | Met and kept | 2 of 4 worlds, worlds 1 and 3, on day 4 | 4 of 4, on day 8 |
//! | Not met | 2, worlds 2 and 4, by day 5 | none |
//! | The side's men lost to the enemy | 100, 100, 200 | 100, 100, 200 |
//! | The trial's enemy's men lost to the enemy | 200, 200, 254 | 200, 200, 400 |
//! | Every other cost of either side | 0 | 0 |
//! | The enemy at (4,0) on the eve: men | 200, 200, 400 | 200, 200, 400 |
//! | The enemy at (4,0) on the eve: the least cover | 0, 0, 0 | 30, 30, 30 |
//!
//! - **World for world,** day 4 against day 1: met and kept at both in
//!   2 worlds, 1 and 3; at the first alone in none; at the later alone
//!   in 2, worlds 2 and 4; at neither in none.
//! - The second date's words say that nothing of the order was done
//!   before day 4.
//! - No second trial is played, and the answer names no other plan.
//!
//! ## B. What if I hold it a day: a what-if of the date
//!
//! On the same evening, with the same three commands queued, he asks
//! `preview` of a `set_limit`, "not before day 3", on intent 1. It is
//! his preview 1. He asks `trial` of preview 1 at `small`, and gives no
//! `compare`: for a preview of a limit it is true, and the second trial
//! is the plan as it stands.
//!
//! **The two trials.** Both are given the three standing commands; the
//! first has the draft besides. Neither is given a leave by the trial.
//!
//! **As it stands** is the first date of A: the place is taken on day 4
//! in worlds 1 and 3.
//!
//! **With the draft,** in every world (scenario 81's days, with one
//! unit): on day 1 the staff forms the landing and orders
//! `prepare_landing` with roster 1, the one unit it may use; on day 2
//! the landing is ready and the plan is not released, the reason naming
//! day 3; on day 3 the plan is released and 600 x 0.005 x 15 = 45 t of
//! supply are loaded; on day 4 unit 1 is set to attack and loaded; on
//! day 5 the force sails at hour 96, arrives at hour 112 and lands the
//! 600 men, and the fight is that evening. The garrison has dug at the
//! end of days 1 to 4: its fieldworks are 40.
//!
//! | The fight of day 5, worlds 1 and 3 | Allied | Japanese |
//! | --- | --- | --- |
//! | Men ashore | 600, on attack | 200, on defend, fieldworks 40 |
//! | Shots, all hits | 6 | 2 |
//! | Men lost in the fight | 2 x 50 = 100 | 6 x 50 = 300, held to the 200 there, cut by cover to 200 x (1 - 0.2) = 160 |
//! | Left | 500 | 40 |
//!
//! 40 is fewer than 0.2 x 500 = 100: beaten, and the 40 surrender. The
//! end is met on day 5. The side has lost 100 men and the enemy 200.
//!
//! | The fight of day 5, worlds 2 and 4 | Allied | Japanese |
//! | --- | --- | --- |
//! | Men ashore | 600, on attack | 200 and 200, on defend, fieldworks 40 |
//! | Shots, all hits | 6 | 4 |
//! | Men lost in the fight | 4 x 50 = 200 | 300, 150 from each unit, each cut to 150 x (1 - 0.2) = 120 |
//! | Left | 400 | 80 and 80: 160 |
//!
//! 160 is not fewer than 0.2 x 400 = 80: not beaten. Not met by day 5.
//! The side has lost 200 men and the enemy 240.
//!
//! **The answer** is of the kind `trial`, of preview 1, intent 1:
//!
//! | | With the draft | As it stands |
//! | --- | --- | --- |
//! | Met and kept | 2 of 4 worlds, worlds 1 and 3, the earliest, the middle and the latest day 5 | 2 of 4, worlds 1 and 3, on day 4 |
//! | Not met by day 5 | 2, worlds 2 and 4 | 2, worlds 2 and 4 |
//! | The side's men lost to the enemy | 100, 100, 200 | 100, 100, 200 |
//! | The trial's enemy's men lost to the enemy | 200, 200, 240 | 200, 200, 254 |
//! | Every other cost of either side | 0 | 0 |
//!
//! - **World for world,** the draft against the plan as it stands: met
//!   and kept in both in 2 worlds, 1 and 3; with the draft alone in
//!   none; as it stands alone in none; in neither in 2, worlds 2 and 4.
//! - **Where the worlds part with the draft,** by the 200 men believed,
//!   standing unknown: with them, worlds 2 and 4, met and kept in none,
//!   the side's men lost 200, 200, 200 and the enemy's 240, 240, 240;
//!   without them, worlds 1 and 3, met and kept in 2 on day 5, the
//!   side's men lost 100, 100, 100 and the enemy's 200, 200, 200.
//! - The side has no other plan to weigh, and the answer says so.
//! - The plan order's day expected ashore with the draft is reported by
//!   the build for the architect to state, and not asserted here.
//!
//! ## C. What does it cost me: without it
//!
//! On the same evening he asks `trial` of intent 1 with `compare` true,
//! at `small`.
//!
//! **The two trials.** The first has the three standing commands and an
//! empty course: the first date of A. The second has the standing
//! commands less commands 1 and 2, which leaves the leave: nothing is
//! ordered, no ship sails and no man is lost.
//!
//! **The answer:** how the intent went and what it cost are the first
//! date of A. Beside it, without the order: the side's men lost to the
//! enemy 0, 0, 0 and the trial's enemy's 0, 0, 0, and every other cost
//! 0. The intent is not there to have gone, and no count of worlds is
//! given for it. The side has no other plan to weigh.
//!
//! ## Nothing is changed
//!
//! - The session's hash of the game is the same before each call and
//!   after it. The seat's queue holds the same three commands.
//! - Preview 1 is not spent by its trial: `approve` of it afterwards
//!   queues the limit.
//! - `trials` lists three estimates, A, B and C, numbered 1 to 3 in the
//!   order asked, and gives each again unchanged.
//! - **The live enemy is not read.** The same scenario with the live
//!   garrison of B changed by 300 men, and the same calls: the three
//!   answers are the same, byte for byte.
//!
//! # Properties
//!
//! - **Asked twice, the same.** Each of A, B and C asked again on the
//!   same evening gives the same fields and words.
//! - **Where no `compare` is given.** `trial` of intent 1 with nothing
//!   more plays one trial: the side has no other plan. `trial` of
//!   preview 1 with `compare` false plays one trial, and its figures are
//!   B's "with the draft".
//! - **The same leaves in both trials.** Without command 3: the what-if
//!   of B has B's figures on both sides. Leave to launch is given by the
//!   trial in both trials, and the answer names it as given to the side
//!   by the trial. (Before this record the second trial was given no
//!   leave, and its landing would not have sailed.)
//! - **Another plan to weigh.** With a hold of base 1 queued as well,
//!   his intent 2: `trial` of intent 1 with no `compare` plays the second
//!   trial, without intent 1. Intent 2 is met and kept in 4 worlds with
//!   intent 1 and in 4 without, and is not named as displaced.
//! - **An order in force.** With the three commands taken in on day 1
//!   and the trial asked on the evening of day 1:
//!   - with no `compare`, one trial is played, and the answer says the
//!     order is in force;
//!   - with `compare` true, two are played, of days 2 to 6. The second
//!     is handed the withdrawal of intent 1 on the morning of day 2,
//!     before the staff is asked: nothing sails, the side's men lost
//!     are 0, 0, 0, and the answer says that the order is withdrawn on
//!     the coming morning, that what was already ordered stands, and
//!     that the withdrawal is the trial's. In the
//!     first the place is taken on day 4 in worlds 1 and 3. What worlds
//!     2 and 4 cost by day 6 is reported by the build for the architect
//!     to state, and not asserted here.
//! - **Dates given.** `trial` of intent 1 with `dates` of day 4 gives
//!   A's window, and so does `dates` of days 1 and 4.
//! - **A draft limit at later dates.** `trial` of preview 1 with `dates`
//!   chosen: the course is commands 1 and 2 and then the draft, and the
//!   dates are day 1 and day 4, chosen from the forces limit. Its first
//!   date has B's figures with the draft. The second date's figures are
//!   reported by the build, and not asserted here.
//! - **A draft at later dates.** On a fresh evening he previews the
//!   `set_intent` alone and asks `trial` of that preview with `dates`
//!   chosen. With no forces limit every transport force and fighting
//!   unit of the side counts, so day 4 is a date. It is answered with a
//!   window of two dates; its figures are reported by the build, and not
//!   asserted here.
//! - **Asked before it is played.** A with `describe` true: the answer
//!   gives the standing command, the two commands of the course, the
//!   dates day 1 and day 4 with unit 2 due on day 4, and 4 worlds for 5
//!   days at the first date and 8 at the second. B with `describe` true:
//!   two trials of 4 worlds for 5 days, the second the plan as it
//!   stands. Nothing
//!   is played, the hash is the same, and `trials` lists nothing more.
//! - **Refused, each with its own words and nothing played:**
//!   - `dates` with `compare` true;
//!   - `dates` of an intent in force, named by `intent` or by a preview
//!     of a limit on it;
//!   - `dates` of three later days; of days not ascending; of day 6,
//!     which is after the last day of the first date's play; and `dates`
//!     that is neither `chosen` nor a list of days;
//!   - with a second intent queued, a take of another place, and an
//!     "after intent 1" queued on it: `dates` of intent 1, and `compare`
//!     true of intent 1, each naming the "after";
//!   - a `compare` or a `describe` that is not true or false.
//! - **Nothing of the trial's enemy is named.** Every number of a unit,
//!   a task force or a base in each answer, fields and words, is one the
//!   seat's own tools give in the live game; no run of digits in it is
//!   900001 or 900002, with or without a comma. The enemy on the eve of
//!   a date is men and cover at a place.
//! - **The words** hold no per cent sign and none of "chance", "points",
//!   "better", "should", "recommend" or "advise", and none of the
//!   engine's own names decision 0178's hand calculation lists.
//! - **The answers are checked against their schemas.**
//! - **A game with no trial is as it was:** the recorded games replay
//!   as they did, but for the version of the agent's interface in each
//!   answer. Scenario 97's answers are as they were, but for that
//!   version.

// The tests are written by the build, to the statements above.

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
    common::data_dir().join("scenarios/100_what_ifs_from_a_seat.toml")
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

const SET_INTENT: (&str, &str) = (
    "set_intent",
    r#"{"intent_kind": "take_place", "hex": [4, 0]}"#,
);
const SET_FORCES: (&str, &str) = (
    "set_limit",
    r#"{"limit": "forces", "intent": 1, "task_forces": [1], "land_units": [1, 2], "source": 1}"#,
);
const SET_LEAVE: (&str, &str) = ("set_leave", r#"{"leave": "launch", "given": true}"#);

fn queue(session: &mut Session, commands: &[(&str, &str)]) {
    for (tool, args) in commands {
        let a = ask(session, tool, serde_json::from_str(args).unwrap());
        assert_eq!(a["status"], "queued", "{tool}: {a}");
    }
}

/// The commander's three commands of the evening before day 1.
fn queue_the_three(session: &mut Session) {
    queue(session, &[SET_INTENT, SET_FORCES, SET_LEAVE]);
}

/// The preview of "not before day 3" on intent 1: its number.
fn preview_not_before(session: &mut Session) -> u64 {
    let p = ask(
        session,
        "preview",
        json!({"command": "set_limit", "limit": "not_before", "intent": 1, "from_day": 3}),
    );
    p["preview"]
        .as_u64()
        .unwrap_or_else(|| panic!("a preview: {p}"))
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

fn spread(lowest: u64, middle: u64, highest: u64) -> Value {
    json!({"lowest": lowest, "middle": middle, "highest": highest})
}

/// The five costs: men lost to the enemy as given, every other 0.
fn costs(men: Value) -> Value {
    json!({
        "men_lost_to_the_enemy": men,
        "men_lost_unfed": spread(0, 0, 0),
        "ships_sunk": spread(0, 0, 0),
        "ships_out_of_action": spread(0, 0, 0),
        "aircraft_lost": spread(0, 0, 0),
    })
}

fn cover(lowest: &str, middle: &str, highest: &str) -> Value {
    json!({"lowest": lowest, "middle": middle, "highest": highest})
}

fn went(met: &[u32], days: Option<(u32, u32, u32)>, not_met: &[u32]) -> Value {
    json!({
        "worlds": [1, 2, 3, 4],
        "met_and_kept": met,
        "met_days": days.map(|(a, b, c)| json!({"earliest": a, "middle": b, "latest": c})),
        "met_and_lost_again": [],
        "not_met": not_met,
    })
}

const THE_THREE_WORDS: [&str; 3] = [
    "set intent 1: take the place at (4,0)",
    "limit intent 1's forces to task_force:1 \"Placeholder transport force 1\", land_unit:1 \"Placeholder First\", land_unit:2 \"Placeholder Second\", from base:1 \"Placeholder base A\"",
    "give leave to launch",
];

/// The evening before day 1, the three commands queued, preview 1 shown,
/// and the three questions asked in order: A, the window; B, the what-if of
/// the date; C, without it. Each answer, the hash before and after each
/// call, and the session after.
struct Asked {
    a: Value,
    b: Value,
    c: Value,
    hashes: Vec<(u64, u64)>,
    preview: u64,
    session: Session,
}

fn ask_the_three() -> Asked {
    let mut s = session();
    queue_the_three(&mut s);
    let preview = preview_not_before(&mut s);
    let mut hashes = Vec::new();
    let mut asked = |s: &mut Session, args: Value| {
        let before = s.state_hash();
        let a = ask(s, "trial", args);
        hashes.push((before, s.state_hash()));
        a
    };
    let a = asked(
        &mut s,
        json!({"intent": 1, "dates": "chosen", "size": "small"}),
    );
    let b = asked(&mut s, json!({"preview": preview, "size": "small"}));
    let c = asked(
        &mut s,
        json!({"intent": 1, "compare": true, "size": "small"}),
    );
    Asked {
        a,
        b,
        c,
        hashes,
        preview,
        session: s,
    }
}

/// The main case's three answers, asked once.
fn answers() -> (Value, Value, Value) {
    static MAIN: OnceLock<(Value, Value, Value)> = OnceLock::new();
    MAIN.get_or_init(|| {
        let m = ask_the_three();
        (m.a, m.b, m.c)
    })
    .clone()
}

// ---------------------------------------------------------------- Inputs

#[test]
fn the_inputs_are_scenario_96s_second_case() {
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
        [("small", 4, 5), ("standard", 8, 5), ("full", 16, 5)]
    );
    assert_eq!(figures.seed, 1942);
    assert_eq!(
        figures.enemy_base_supply_t,
        sim_core::fixed::Fixed::from_int(200)
    );
    assert_eq!(figures.enemy_base_fuel_t, sim_core::fixed::Fixed::ZERO);
    // Scenario 96's file and its second case's believed men, but for the
    // name, the file of sides, the trial's figures and the notes.
    let ninety_six =
        std::fs::read_to_string(common::data_dir().join("scenarios/96_the_window.toml")).unwrap();
    let more = r#"
[[believed_men]]
side = "allied"
name = "More believed at B"
at_base = 2
men = 200
standing = "unknown"
placeholder = true
"#;
    let body = |t: &str| -> Vec<String> {
        t.lines()
            .filter(|l| !l.starts_with('#') && !l.starts_with("name = \"Placeholder scenario"))
            .filter(|l| !l.starts_with("class_sides") && !l.starts_with("trial ="))
            .filter(|l| !l.starts_with("note =") && !l.trim().is_empty())
            .map(str::to_string)
            .collect()
    };
    assert_eq!(body(&text()), body(&format!("{ninety_six}\n{more}")));
    // The Allied side holds no count of B, and believes 200 present and
    // 200 more of unknown standing.
    let men: Vec<(u32, sim_core::believed::Standing)> = s
        .believed_men(ALLIED)
        .map(|m| (m.men, m.standing))
        .collect();
    assert_eq!(
        men,
        vec![
            (200, sim_core::believed::Standing::Present),
            (200, sim_core::believed::Standing::Unknown)
        ]
    );
}

// ---------------------------------------------------------------- A. The window

#[test]
fn a_the_window_hands_in_commands_1_and_2_at_each_date() {
    let (a, _, _) = answers();
    checked(&a);
    assert_eq!(a["kind"], "window");
    assert_eq!(a["version"], sim_agent::VERSION);
    assert_eq!(a["estimate"], 1);
    let asked = &a["asked"];
    assert_eq!(asked["intent"], 1);
    assert_eq!(asked["dates"], "chosen");
    assert_eq!(asked["compare"], false);
    assert_eq!(asked["second_trial"], false);
    assert_eq!(asked["beside"], Value::Null);
    assert_eq!(asked["in_force"], false);
    assert_eq!(
        asked["course"],
        json!([THE_THREE_WORDS[0], THE_THREE_WORDS[1]]),
        "commands 1 and 2 at each date"
    );
    assert_eq!(
        asked["standing"],
        json!([THE_THREE_WORDS[2]]),
        "command 3 handed in on day 1 at every date"
    );
    assert_eq!(a["given"], false);
    for date in a["dates"].as_array().unwrap() {
        assert_eq!(
            date["leaves_given_to_the_side"],
            json!([]),
            "the course gives no leave"
        );
    }
}

#[test]
fn a_the_dates_are_day_1_and_day_4() {
    let (a, _, _) = answers();
    let dates = a["dates"].as_array().unwrap();
    assert_eq!(
        dates.iter().map(|d| d["day"].clone()).collect::<Vec<_>>(),
        vec![json!(1), json!(4)]
    );
    assert_eq!(dates[0]["due"], json!([]));
    assert_eq!(dates[1]["due"].as_array().unwrap().len(), 1);
    let due = &dates[1]["due"][0];
    assert_eq!(due["kind"], "land_unit");
    assert_eq!(due["unit"]["handle"], "land_unit:2");
    assert_eq!(due["day"], 4);
    assert_eq!(
        (a["first_day"].clone(), a["last_day"].clone()),
        (json!(1), json!(5))
    );
    assert_eq!(dates[0]["last_day"], 5, "the first date is played to day 5");
    assert_eq!(dates[1]["last_day"], 8, "the second to day 8");
    assert_eq!(a["not_tried"], json!([]));
}

#[test]
fn a_the_first_date_day_1() {
    let (a, _, _) = answers();
    let d = &a["dates"][0];
    assert_eq!(d["taken_in"], 1);
    assert_eq!(d["after_first"], 0);
    assert_eq!(d["chosen"]["men"], 600);
    assert_eq!(
        d["chosen"]["units"],
        json!([{"land_unit": {"handle": "land_unit:1", "name": "Placeholder First"}, "men": 600}])
    );
    assert_eq!(d["needs"]["men"], Value::Null, "no count is held");
    assert_eq!(d["needs"]["count"]["held"], Value::Null);
    assert_eq!(d["expected_ashore"], 4);
    assert_eq!(d["how_it_went"], went(&[1, 3], Some((4, 4, 4)), &[2, 4]));
    assert_eq!(d["cost"], costs(spread(100, 100, 200)));
    assert_eq!(d["enemy_lost"], costs(spread(200, 200, 254)));
    let eve = &d["enemy_on_the_eve"];
    assert_eq!(eve.as_array().unwrap().len(), 1);
    assert_eq!(
        (eve[0]["place"]["q"].clone(), eve[0]["place"]["r"].clone()),
        (json!(4), json!(0))
    );
    assert_eq!(eve[0]["men"], spread(200, 200, 400));
    assert_eq!(eve[0]["least_cover"], cover("0", "0", "0"));
    assert_eq!(d["against_first"], Value::Null);
}

#[test]
fn a_the_second_date_day_4() {
    let (a, _, _) = answers();
    let d = &a["dates"][1];
    assert_eq!(d["taken_in"], 4);
    assert_eq!(d["after_first"], 3);
    assert_eq!(d["chosen"]["men"], 1200);
    assert_eq!(
        d["chosen"]["units"],
        json!([
            {"land_unit": {"handle": "land_unit:1", "name": "Placeholder First"}, "men": 600},
            {"land_unit": {"handle": "land_unit:2", "name": "Placeholder Second"}, "men": 600}
        ])
    );
    assert_eq!(d["needs"]["men"], Value::Null, "none");
    assert_eq!(d["expected_ashore"], 8);
    assert_eq!(d["how_it_went"], went(&[1, 2, 3, 4], Some((8, 8, 8)), &[]));
    assert_eq!(d["cost"], costs(spread(100, 100, 200)));
    assert_eq!(d["enemy_lost"], costs(spread(200, 200, 400)));
    let eve = &d["enemy_on_the_eve"][0];
    assert_eq!(eve["men"], spread(200, 200, 400));
    assert_eq!(eve["least_cover"], cover("30", "30", "30"));
    assert_eq!(eve["eve"], 3);
}

#[test]
fn a_world_for_world_and_its_words() {
    let (a, _, _) = answers();
    assert_eq!(
        a["dates"][1]["against_first"],
        json!({"both": [1, 3], "first_alone": [], "later_alone": [2, 4], "neither": [], "apart": []})
    );
    let second = a["dates"][1]["text"]
        .as_array()
        .unwrap()
        .iter()
        .map(|l| l.as_str().unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        second.contains("Nothing of the order was done before day 4"),
        "{second}"
    );
    // No second trial, and no other plan named.
    for date in a["dates"].as_array().unwrap() {
        assert!(date.get("other_plans").is_none());
    }
    let w = words(&a);
    assert!(!w.contains("Your intent"), "{w}");
    assert!(!w.contains("Beside it"), "{w}");
    println!("A, THE WINDOW'S WORDS:\n{w}");
}

// ---------------------------------------------------------------- B. A what-if of the date

#[test]
fn b_the_two_trials_differ_by_the_draft_alone() {
    let (_, b, _) = answers();
    checked(&b);
    assert_eq!(b["kind"], "trial");
    let asked = &b["asked"];
    assert_eq!(asked["preview"], 1);
    assert_eq!(asked["tried_intent"], 1);
    assert_eq!(
        asked["compare"], true,
        "for a preview of a limit it is true"
    );
    assert_eq!(asked["beside"], "as_it_stands");
    assert_eq!(asked["second_trial"], true);
    assert_eq!(asked["standing"], json!(THE_THREE_WORDS));
    assert_eq!(
        asked["course"],
        json!(["intent:1 not before day 3"]),
        "the first has the draft besides"
    );
    assert_eq!(
        b["leaves_given_to_the_side"],
        json!([]),
        "neither is given a leave"
    );
}

#[test]
fn b_with_the_draft_and_as_it_stands() {
    let (_, b, _) = answers();
    // With the draft.
    assert_eq!(b["how_it_went"], went(&[1, 3], Some((5, 5, 5)), &[2, 4]));
    assert_eq!(b["last_day"], 5);
    assert_eq!(b["cost"], costs(spread(100, 100, 200)));
    assert_eq!(b["enemy_lost"], costs(spread(200, 200, 240)));
    // As it stands: the first date of A.
    let beside = &b["beside"];
    assert_eq!(beside["of"], "as_it_stands");
    assert_eq!(
        beside["how_it_went"],
        went(&[1, 3], Some((4, 4, 4)), &[2, 4])
    );
    assert_eq!(beside["cost"], costs(spread(100, 100, 200)));
    assert_eq!(beside["enemy_lost"], costs(spread(200, 200, 254)));
    let (a, _, _) = answers();
    for field in ["how_it_went", "cost", "enemy_lost"] {
        assert_eq!(beside[field], a["dates"][0][field], "{field}");
    }
    assert_eq!(
        beside["world_for_world"],
        json!({"both": [1, 3], "with_the_draft_alone": [], "as_it_stands_alone": [], "neither": [2, 4], "apart": []})
    );
}

#[test]
fn b_where_the_worlds_part_with_the_draft() {
    let (_, b, _) = answers();
    let partings = b["partings"].as_array().unwrap();
    assert_eq!(partings.len(), 1);
    let p = &partings[0];
    assert_eq!(p["by"]["kind"], "men");
    assert_eq!(p["by"]["men"], 200);
    assert_eq!(p["from"]["standing"], "unknown");
    assert_eq!(p["with"]["how_it_went"]["worlds"], json!([2, 4]));
    assert_eq!(p["with"]["how_it_went"]["met_and_kept"], json!([]));
    assert_eq!(p["with"]["cost"], costs(spread(200, 200, 200)));
    assert_eq!(p["with"]["enemy_lost"], costs(spread(240, 240, 240)));
    assert_eq!(p["without"]["how_it_went"]["worlds"], json!([1, 3]));
    assert_eq!(p["without"]["how_it_went"]["met_and_kept"], json!([1, 3]));
    assert_eq!(
        p["without"]["how_it_went"]["met_days"],
        json!({"earliest": 5, "middle": 5, "latest": 5})
    );
    assert_eq!(p["without"]["cost"], costs(spread(100, 100, 100)));
    assert_eq!(p["without"]["enemy_lost"], costs(spread(200, 200, 200)));
    // No other plan to weigh, and the answer says so.
    assert_eq!(b["other_plans"], json!([]));
    assert!(words(&b).contains("no other plan"), "{}", words(&b));
}

/// Reported, not asserted: the plan order's day expected ashore with the
/// draft.
#[test]
fn b_reported_the_day_expected_ashore_with_the_draft() {
    let (_, b, _) = answers();
    println!(
        "B, THE DAY EXPECTED ASHORE WITH THE DRAFT: {} (the plan order of world {} at the end of day {})",
        b["needs"]["expected_ashore"], b["needs"]["world"], b["needs"]["at_the_end_of_day"]
    );
    println!("B, ITS WORDS:\n{}", words(&b));
}

// ---------------------------------------------------------------- C. Without it

#[test]
fn c_without_the_order() {
    let (a, _, c) = answers();
    checked(&c);
    assert_eq!(c["kind"], "trial");
    let asked = &c["asked"];
    assert_eq!(asked["compare"], true);
    assert_eq!(asked["beside"], "without_it");
    assert_eq!(asked["standing"], json!(THE_THREE_WORDS));
    assert_eq!(asked["course"], json!([]), "an empty course");
    // How the intent went and what it cost are the first date of A.
    for field in ["how_it_went", "cost", "enemy_lost"] {
        assert_eq!(c[field], a["dates"][0][field], "{field}");
    }
    let beside = &c["beside"];
    assert_eq!(beside["of"], "without_it");
    assert_eq!(
        beside["how_it_went"],
        Value::Null,
        "the intent is not there to have gone"
    );
    assert_eq!(
        beside["world_for_world"],
        Value::Null,
        "no count of worlds for it"
    );
    assert_eq!(beside["cost"], costs(spread(0, 0, 0)));
    assert_eq!(beside["enemy_lost"], costs(spread(0, 0, 0)));
    assert_eq!(c["other_plans"], json!([]));
    let w = words(&c);
    assert!(w.contains("no other plan"), "{w}");
    assert!(w.contains("Beside it, your side without the order"), "{w}");
}

// ---------------------------------------------------------------- Nothing is changed

#[test]
fn nothing_is_changed_by_asking() {
    let mut m = ask_the_three();
    assert_eq!(m.hashes.len(), 3);
    for (before, after) in &m.hashes {
        assert_eq!(before, after);
    }
    let state = ask(&mut m.session, "command_state", json!({}));
    assert_eq!(
        state["queued"].as_array().map(Vec::len),
        Some(3),
        "the queue holds the same three commands"
    );
}

#[test]
fn the_preview_is_not_spent_and_trials_keeps_the_three() {
    let mut m = ask_the_three();
    let s = &mut m.session;
    let listed = ask(s, "trials", json!({}));
    checked(&listed);
    let estimates = listed["estimates"].as_array().unwrap();
    assert_eq!(estimates.len(), 3);
    assert_eq!(
        estimates
            .iter()
            .map(|e| e["estimate"].clone())
            .collect::<Vec<_>>(),
        vec![json!(1), json!(2), json!(3)]
    );
    assert_eq!(
        estimates
            .iter()
            .map(|e| e["kind"].clone())
            .collect::<Vec<_>>(),
        vec![json!("window"), json!("trial"), json!("trial")]
    );
    for (number, answer) in [(1, &m.a), (2, &m.b), (3, &m.c)] {
        let again = ask(s, "trials", json!({"estimate": number}));
        assert_eq!(&again, answer, "estimate {number} unchanged");
    }
    // The preview is not spent: approved afterwards, it queues the limit.
    let approved = ask(s, "approve", json!({"preview": m.preview}));
    assert_eq!(approved["status"], "queued", "{approved}");
}

#[test]
fn the_live_enemy_is_not_read() {
    let answers_of = |text: &str| -> Vec<String> {
        let mut s = session_with(load(text), &[ALLIED]);
        queue_the_three(&mut s);
        let p = preview_not_before(&mut s);
        [
            json!({"intent": 1, "dates": "chosen"}),
            json!({"preview": p}),
            json!({"intent": 1, "compare": true}),
        ]
        .into_iter()
        .map(|args| ask(&mut s, "trial", args).to_string())
        .collect()
    };
    let other = common::replace_once(&text(), "men = 1200", "men = 1500");
    assert_ne!(text(), other);
    assert_eq!(answers_of(&text()), answers_of(&other), "byte for byte");
}

// ---------------------------------------------------------------- Properties

/// An answer without its number among the estimates shown.
fn unnumbered(a: &Value) -> Value {
    let mut a = a.clone();
    a["estimate"] = Value::Null;
    a
}

#[test]
fn asked_twice_the_same() {
    let (a, b, c) = answers();
    let mut m = ask_the_three();
    let preview = m.preview;
    for (args, first) in [
        (json!({"intent": 1, "dates": "chosen", "size": "small"}), &a),
        (json!({"preview": preview, "size": "small"}), &b),
        (json!({"intent": 1, "compare": true, "size": "small"}), &c),
    ] {
        let again = ask(&mut m.session, "trial", args.clone());
        assert_eq!(unnumbered(&again), unnumbered(first), "{args}");
        assert_eq!(again["text"], first["text"], "{args}");
    }
}

#[test]
fn where_no_compare_is_given() {
    let mut s = session();
    queue_the_three(&mut s);
    // Of intent 1 with nothing more: one trial, for the side has no other
    // plan.
    let one = ask(&mut s, "trial", json!({"intent": 1}));
    checked(&one);
    assert_eq!(one["asked"]["second_trial"], false);
    assert_eq!(one["beside"], Value::Null);
    // Of preview 1 with compare false: one trial, B's "with the draft".
    let p = preview_not_before(&mut s);
    let alone = ask(&mut s, "trial", json!({"preview": p, "compare": false}));
    checked(&alone);
    assert_eq!(alone["asked"]["second_trial"], false);
    assert_eq!(alone["beside"], Value::Null);
    let (_, b, _) = answers();
    for field in [
        "how_it_went",
        "cost",
        "enemy_lost",
        "needs",
        "assumed",
        "partings",
    ] {
        assert_eq!(alone[field], b[field], "{field}");
    }
}

#[test]
fn the_same_leaves_in_both_trials() {
    let mut s = session();
    queue(&mut s, &[SET_INTENT, SET_FORCES]);
    let p = preview_not_before(&mut s);
    let a = ask(&mut s, "trial", json!({"preview": p}));
    checked(&a);
    assert_eq!(
        a["asked"]["course"],
        json!(["intent:1 not before day 3", "give leave to launch"])
    );
    let (_, b, _) = answers();
    for field in ["how_it_went", "cost", "enemy_lost", "beside", "partings"] {
        assert_eq!(a[field], b[field], "{field}");
    }
    assert_eq!(a["leaves_given_to_the_side"], json!(["launch"]));
    let w = words(&a);
    assert!(w.contains("given to the side by the trial"), "{w}");
    assert!(w.contains("It is given in both trials"), "{w}");
}

#[test]
fn another_plan_to_weigh() {
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
    assert_eq!(a["asked"]["beside"], "without_it");
    let plans = a["other_plans"].as_array().expect("other plans weighed");
    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0]["intent"], 2);
    assert_eq!(plans[0]["with"]["worlds"].as_array().unwrap().len(), 4);
    assert_eq!(plans[0]["without"]["worlds"].as_array().unwrap().len(), 4);
    assert_eq!(plans[0]["displaced"], Value::Null, "not named as displaced");
}

/// The three commands taken in on day 1, asked on the evening of day 1.
fn in_force_session() -> Session {
    let mut s = session();
    queue_the_three(&mut s);
    ask(&mut s, "end_day", json!({}));
    s
}

#[test]
fn an_order_in_force_with_no_compare() {
    let mut s = in_force_session();
    let a = ask(&mut s, "trial", json!({"intent": 1}));
    checked(&a);
    assert_eq!(a["asked"]["in_force"], true);
    assert_eq!(a["asked"]["second_trial"], false, "one trial");
    assert_eq!(a["beside"], Value::Null);
    let w = words(&a);
    assert!(w.contains("Your order is in force"), "{w}");
}

#[test]
fn an_order_in_force_compared_is_withdrawn_in_the_second() {
    let mut s = in_force_session();
    let before = s.state_hash();
    let a = ask(&mut s, "trial", json!({"intent": 1, "compare": true}));
    assert_eq!(s.state_hash(), before);
    checked(&a);
    assert_eq!(a["asked"]["second_trial"], true, "two are played");
    assert_eq!(a["asked"]["beside"], "withdrawn");
    assert_eq!(
        (a["first_day"].clone(), a["last_day"].clone()),
        (json!(2), json!(6))
    );
    let beside = &a["beside"];
    assert_eq!(beside["of"], "withdrawn");
    assert_eq!(beside["last_day"], 6);
    assert_eq!(beside["how_it_went"], Value::Null);
    assert_eq!(
        beside["cost"]["men_lost_to_the_enemy"],
        spread(0, 0, 0),
        "nothing sails"
    );
    let w = words(&a);
    for said in [
        "withdrawn on the coming morning",
        "what was already ordered stands",
        "The withdrawal is the trial's",
    ] {
        assert!(w.contains(said), "{said:?} in:\n{w}");
    }
    // In the first the place is taken on day 4 in worlds 1 and 3: the
    // worlds without the 200 of unknown standing.
    let without = &a["partings"][0]["without"]["how_it_went"];
    assert_eq!(without["worlds"], json!([1, 3]));
    assert_eq!(without["met_and_kept"], json!([1, 3]));
    assert_eq!(
        without["met_days"],
        json!({"earliest": 4, "middle": 4, "latest": 4})
    );
    // Reported, not asserted: what worlds 2 and 4 cost by day 6.
    println!(
        "IN FORCE, COMPARED: went {}; the side's men lost {}; the trial's enemy's {}; worlds 2 and 4: {}\nWORDS:\n{}",
        a["how_it_went"], a["cost"]["men_lost_to_the_enemy"], a["enemy_lost"]["men_lost_to_the_enemy"],
        a["partings"][0]["with"], words(&a)
    );
}

#[test]
fn dates_may_be_given() {
    let (a, _, _) = answers();
    let strip = |w: &Value| -> Value {
        let mut w = w.clone();
        w["estimate"] = Value::Null;
        w["asked"]["dates"] = Value::Null;
        w["given"] = Value::Null;
        w["text"] = Value::Null;
        w
    };
    for dates in [json!([4]), json!([1, 4])] {
        let mut s = session();
        queue_the_three(&mut s);
        let given = ask(&mut s, "trial", json!({"intent": 1, "dates": dates}));
        checked(&given);
        assert_eq!(given["given"], true);
        assert_eq!(given["asked"]["dates"], json!([1, 4]));
        assert_eq!(strip(&given), strip(&a), "dates {dates}");
    }
}

#[test]
fn a_draft_limit_at_later_dates() {
    let mut s = session();
    queue_the_three(&mut s);
    let p = preview_not_before(&mut s);
    let w = ask(&mut s, "trial", json!({"preview": p, "dates": "chosen"}));
    checked(&w);
    assert_eq!(w["kind"], "window");
    assert_eq!(
        w["asked"]["course"],
        json!([
            THE_THREE_WORDS[0],
            THE_THREE_WORDS[1],
            "intent:1 not before day 3"
        ])
    );
    assert_eq!(w["asked"]["standing"], json!([THE_THREE_WORDS[2]]));
    let dates = w["dates"].as_array().unwrap();
    assert_eq!(
        dates.iter().map(|d| d["day"].clone()).collect::<Vec<_>>(),
        vec![json!(1), json!(4)]
    );
    let (_, b, _) = answers();
    for field in ["how_it_went", "cost", "enemy_lost", "partings"] {
        assert_eq!(dates[0][field], b[field], "{field}");
    }
    println!(
        "A DRAFT LIMIT AT LATER DATES, THE SECOND DATE: went {}; cost {}; enemy lost {}; expected ashore {}; chosen {}",
        dates[1]["how_it_went"], dates[1]["cost"]["men_lost_to_the_enemy"],
        dates[1]["enemy_lost"]["men_lost_to_the_enemy"], dates[1]["expected_ashore"], dates[1]["chosen"]
    );
}

#[test]
fn a_draft_at_later_dates() {
    let mut s = session();
    let p = ask(
        &mut s,
        "preview",
        json!({"command": "set_intent", "intent_kind": "take_place", "hex": [4, 0]}),
    );
    let number = p["preview"].as_u64().unwrap();
    let w = ask(
        &mut s,
        "trial",
        json!({"preview": number, "dates": "chosen"}),
    );
    checked(&w);
    assert_eq!(w["kind"], "window");
    let dates = w["dates"].as_array().unwrap();
    assert_eq!(dates.len(), 2, "a window of two dates");
    assert_eq!(
        dates[1]["day"], 4,
        "every fighting unit counts: day 4 is a date"
    );
    for (i, d) in dates.iter().enumerate() {
        println!(
            "A DRAFT AT LATER DATES, DATE {} (day {}): went {}; cost {}; enemy lost {}; chosen {}; leaves {}",
            i + 1,
            d["day"],
            d["how_it_went"],
            d["cost"]["men_lost_to_the_enemy"],
            d["enemy_lost"]["men_lost_to_the_enemy"],
            d["chosen"],
            d["leaves_given_to_the_side"]
        );
    }
}

#[test]
fn asked_before_it_is_played() {
    let mut s = session();
    queue_the_three(&mut s);
    let before = s.state_hash();
    let a = ask(
        &mut s,
        "trial",
        json!({"intent": 1, "dates": "chosen", "describe": true}),
    );
    checked(&a);
    assert_eq!(a["kind"], "trial_described");
    assert_eq!(a["asked"]["standing"], json!([THE_THREE_WORDS[2]]));
    assert_eq!(
        a["asked"]["course"],
        json!([THE_THREE_WORDS[0], THE_THREE_WORDS[1]])
    );
    assert_eq!(a["dates"][0], json!({"day": 1, "due": []}));
    assert_eq!(a["dates"][1]["day"], 4);
    assert_eq!(a["dates"][1]["due"][0]["unit"]["handle"], "land_unit:2");
    assert_eq!(
        a["plays"],
        json!([
            {"trial": "first", "date": 1, "worlds": 4, "days": 5},
            {"trial": "first", "date": 4, "worlds": 4, "days": 8}
        ])
    );
    let p = preview_not_before(&mut s);
    let b = ask(&mut s, "trial", json!({"preview": p, "describe": true}));
    checked(&b);
    assert_eq!(b["asked"]["beside"], "as_it_stands");
    assert_eq!(
        b["plays"],
        json!([
            {"trial": "first", "date": 1, "worlds": 4, "days": 5},
            {"trial": "second", "date": 1, "worlds": 4, "days": 5}
        ])
    );
    assert!(words(&b).contains("the plan as it stands"), "{}", words(&b));
    assert_eq!(s.state_hash(), before, "the hash is the same");
    let listed = ask(&mut s, "trials", json!({}));
    assert_eq!(listed["estimates"], json!([]), "trials lists nothing more");
    println!("A, DESCRIBED:\n{}\nB, DESCRIBED:\n{}", words(&a), words(&b));
}

// ---------------------------------------------------------------- Refused

fn refusal(s: &mut Session, args: Value) -> String {
    let before = s.state_hash();
    let shown = ask(s, "trials", json!({}))["estimates"]
        .as_array()
        .unwrap()
        .len();
    let e = call(s, "trial", args.clone()).expect_err(&format!("refused: {args}"));
    assert_eq!(s.state_hash(), before);
    let after = ask(s, "trials", json!({}))["estimates"]
        .as_array()
        .unwrap()
        .len();
    assert_eq!(after, shown, "nothing played: {args}");
    e.message
}

#[test]
fn refused_each_with_its_own_words() {
    let mut s = session();
    queue_the_three(&mut s);
    let mut seen = Vec::new();
    let mut expect = |s: &mut Session, args: Value, says: &str| {
        let words = refusal(s, args.clone());
        assert!(words.contains(says), "{args}: {words}");
        seen.push(words);
    };
    expect(
        &mut s,
        json!({"intent": 1, "dates": "chosen", "compare": true}),
        "no second trial",
    );
    expect(
        &mut s,
        json!({"intent": 1, "dates": [2, 3, 4]}),
        "gives 3 days",
    );
    expect(
        &mut s,
        json!({"intent": 1, "dates": [4, 3]}),
        "not each after the one before",
    );
    expect(
        &mut s,
        json!({"intent": 1, "dates": [6]}),
        "after the last day of the first date's play",
    );
    expect(
        &mut s,
        json!({"intent": 1, "dates": "soon"}),
        "\"dates\" must be",
    );
    // Neither "chosen" nor a list of days, in another form: the same words.
    let object = refusal(&mut s, json!({"intent": 1, "dates": {"day": 4}}));
    assert!(object.contains("\"dates\" must be"), "{object}");
    expect(
        &mut s,
        json!({"intent": 1, "compare": "yes"}),
        "\"compare\" must be true or false",
    );
    expect(
        &mut s,
        json!({"intent": 1, "describe": 1}),
        "\"describe\" must be true or false",
    );
    // An order in force, by intent and by a preview of a limit on it.
    let mut f = in_force_session();
    expect(
        &mut f,
        json!({"intent": 1, "dates": "chosen"}),
        "is in force",
    );
    let p = preview_not_before(&mut f);
    let by_preview = refusal(&mut f, json!({"preview": p, "dates": [4]}));
    assert!(by_preview.contains("is in force"), "{by_preview}");
    // An "after intent 1" on a second take.
    let mut g = session();
    queue_the_three(&mut g);
    let second = ask(
        &mut g,
        "set_intent",
        json!({"intent_kind": "take_place", "hex": [0, 6]}),
    );
    assert_eq!(second["status"], "queued", "{second}");
    let after = ask(
        &mut g,
        "set_limit",
        json!({"limit": "after", "intent": 2, "other_intent": 1}),
    );
    assert_eq!(after["status"], "queued", "{after}");
    let moved = refusal(&mut g, json!({"intent": 1, "dates": "chosen"}));
    let left_out = refusal(&mut g, json!({"intent": 1, "compare": true}));
    for words in [&moved, &left_out] {
        assert!(words.contains("names intent 1"), "{words}");
        assert!(words.contains("after"), "{words}");
    }
    seen.extend([moved, left_out]);
    let distinct: BTreeSet<&String> = seen.iter().collect();
    assert_eq!(
        distinct.len(),
        seen.len(),
        "each with its own words: {seen:#?}"
    );
}

// ---------------------------------------------------------------- Naming and words

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

/// Every answer of the main case and the properties that name things.
fn every_answer() -> Vec<Value> {
    let (a, b, c) = answers();
    let mut all = vec![a, b, c];
    let mut s = session();
    queue_the_three(&mut s);
    all.push(ask(
        &mut s,
        "trial",
        json!({"intent": 1, "dates": "chosen", "describe": true}),
    ));
    let mut f = in_force_session();
    all.push(ask(&mut f, "trial", json!({"intent": 1})));
    all.push(ask(&mut f, "trial", json!({"intent": 1, "compare": true})));
    all
}

#[test]
fn nothing_of_the_trials_enemy_is_named() {
    let live = load(&text());
    let view = live.world.side_view(ALLIED);
    let count = |n: usize| -> u32 { u32::try_from(n).unwrap() };
    for a in every_answer() {
        for (kind, number) in handles(&a) {
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
        assert!(!handles(&a).is_empty());
        let flat = a.to_string().replace(',', "");
        for made in ["900001", "900002"] {
            assert!(!flat.contains(made), "{made} in the answer");
        }
    }
    // The enemy on the eve of a date is men and cover at a place.
    let (a, _, _) = answers();
    for date in a["dates"].as_array().unwrap() {
        for eve in date["enemy_on_the_eve"].as_array().unwrap() {
            let keys: BTreeSet<&str> = eve
                .as_object()
                .unwrap()
                .keys()
                .map(String::as_str)
                .collect();
            assert_eq!(keys, BTreeSet::from(["place", "eve", "men", "least_cover"]));
        }
    }
}

#[test]
fn the_words_are_a_commanders() {
    for a in every_answer() {
        let w = words(&a);
        for banned in [
            "%",
            "chance",
            "points",
            "better",
            "should",
            "recommend",
            "advise",
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
            assert!(
                !w.contains(banned),
                "{banned:?} in the words of {}:\n{w}",
                a["kind"]
            );
        }
    }
}

#[test]
fn the_answers_are_checked_against_their_schemas() {
    for a in every_answer() {
        checked(&a);
    }
    let mut m = ask_the_three();
    let listed = ask(&mut m.session, "trials", json!({}));
    checked(&listed);
}

// ---------------------------------------------------------------- B in its worlds

use sim_core::command::CommandKind;
use sim_core::events::EventKind;
use sim_core::hex::Hex;
use sim_core::orders::{Cargo, LandUnitOrder, OrderKind, Posture, TaskForceOrder};
use sim_trial::{play_window, Believed, PlayedWorld, WindowAsked};

const A: Hex = Hex::new(0, 0);
const B: Hex = Hex::new(4, 0);

/// The three commands and the draft as scenario text, read by the loader.
const THE_THREE: &str = r#"
[[command]]
day = 1
side = "allied"
kind = "set_intent"
intent = 1
intent_kind = "take_place"
hex = [4, 0]

[[command]]
day = 1
side = "allied"
kind = "set_limit"
limit = "forces"
intent = 1
task_forces = [1]
land_units = [1, 2]
source = 1

[[command]]
day = 1
side = "allied"
kind = "set_leave"
leave = "launch"
given = true
"#;

const THE_DRAFT: &str = r#"
[[command]]
day = 1
side = "allied"
kind = "set_limit"
limit = "not_before"
intent = 1
from_day = 3
"#;

fn commands_of(extra: &str) -> Vec<CommandKind> {
    let with = load(&format!("{}\n{extra}", text()));
    with.scripted_commands()
        .get(&1)
        .map(|day| day.iter().map(|(_, c)| c.clone()).collect())
        .unwrap_or_default()
}

/// B's first trial, played in its worlds as the runner plays it: the three
/// standing commands, and the draft on day 1 of the course.
fn b_with_the_draft() -> Vec<PlayedWorld> {
    let s = load(&text());
    let draft = commands_of(THE_DRAFT);
    let played = play_window(
        &s.world.side_view(ALLIED),
        &Believed {
            forces: s.believed_forces(ALLIED).cloned().collect(),
            intents: s.believed_intents(ALLIED).cloned().collect(),
            men: s.believed_men(ALLIED).cloned().collect(),
        },
        s.trial_figures().unwrap(),
        s.world.ref_data(),
        s.world.map(),
        WindowAsked {
            trial: 1,
            worlds: 4,
            days: 5,
            intent: sim_core::ids::IntentId::new(1),
            standing: commands_of(THE_THREE),
            course: draft
                .into_iter()
                .map(|command| sim_trial::CourseCommand { day: 1, command })
                .collect(),
            dates: Some(vec![1]),
        },
    )
    .expect("played");
    played.played.into_iter().next().unwrap().played.worlds
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Given {
    Landing(Hex, Vec<u32>),
    LoadTo(sim_core::fixed::Fixed),
    Attack(u32),
    LoadUnit(u32),
    Route(Vec<Hex>, Vec<Hex>),
    Unload,
    Other(String),
}

fn given(o: &sim_core::command::StaffOrder) -> Given {
    match &o.kind {
        OrderKind::TaskForce { order, .. } => match order {
            TaskForceOrder::PrepareLanding { hex, roster } => {
                Given::Landing(*hex, roster.iter().map(|u| u.raw()).collect())
            }
            TaskForceOrder::LoadTo {
                cargo: Cargo::Supply,
                weight_t,
                ..
            } => Given::LoadTo(*weight_t),
            TaskForceOrder::LoadUnit { unit, cargo: None } => Given::LoadUnit(unit.raw()),
            TaskForceOrder::SetRoute {
                waypoints,
                act_at,
                stand_at,
            } if stand_at.is_empty() => {
                Given::Route(waypoints.clone(), act_at.iter().copied().collect())
            }
            TaskForceOrder::Unload => Given::Unload,
            other => Given::Other(format!("{other:?}")),
        },
        OrderKind::LandUnit {
            target,
            order:
                LandUnitOrder::SetPosture {
                    posture: Posture::Attack,
                    when_spent: None,
                },
        } => Given::Attack(target.raw()),
        other => Given::Other(format!("{other:?}")),
    }
}

/// The Allied staff's orders of each day of a world, each day's sorted.
fn orders(world: &PlayedWorld) -> Vec<(u32, Vec<Given>)> {
    world
        .days
        .iter()
        .map(|d| {
            let mut day: Vec<Given> = d
                .staff
                .get(&ALLIED)
                .map(|s| s.orders.iter().map(given).collect())
                .unwrap_or_default();
            day.sort_by_key(|g| format!("{g:?}"));
            (d.day, day)
        })
        .collect()
}

fn sorted(mut days: Vec<(u32, Vec<Given>)>) -> Vec<(u32, Vec<Given>)> {
    for (_, day) in &mut days {
        day.sort_by_key(|g| format!("{g:?}"));
    }
    days
}

type ByUnit = Vec<(u32, u32, u32)>;

fn fight_losses(world: &PlayedWorld) -> (ByUnit, ByUnit) {
    let mut lost = Vec::new();
    let mut surrendered = Vec::new();
    for e in world.world.as_ref().unwrap().events().iter() {
        match &e.kind {
            EventKind::MenLostInFight {
                unit, lost: men, ..
            } => {
                lost.push((e.day, unit.raw(), *men));
            }
            EventKind::UnitSurrendered { unit, men, .. } => {
                surrendered.push((e.day, unit.raw(), *men));
            }
            _ => {}
        }
    }
    (lost, surrendered)
}

#[test]
fn b_with_the_draft_in_every_world() {
    let n = sim_core::fixed::Fixed::from_int;
    for w in b_with_the_draft() {
        assert_eq!(
            orders(&w),
            sorted(vec![
                (1, vec![Given::Landing(B, vec![1])]),
                (2, vec![]),
                (3, vec![Given::LoadTo(n(45))]),
                (4, vec![Given::Attack(1), Given::LoadUnit(1)]),
                (5, vec![Given::Route(vec![B, A], vec![B]), Given::Unload]),
            ]),
            "world {}",
            w.number
        );
        // On day 2 the landing is ready and the plan is not released, the
        // reason naming day 3.
        let day2 = &w.days[1].staff[&ALLIED];
        let reasons: Vec<String> = day2
            .no_orders
            .iter()
            .map(|r| format!("{} {}", r.worked, r.binding.join("; ")))
            .collect();
        assert!(
            reasons.iter().any(|r| r.contains("day 3")),
            "world {}: {reasons:?}",
            w.number
        );
        let (lost, surrendered) = fight_losses(&w);
        if w.number % 2 == 1 {
            // 600 against 200 at fieldworks 40: 100 lost; 300 held to the
            // 200, cut to 160; the 40 left surrender.
            assert_eq!(
                lost,
                vec![(5, 1, 100), (5, 900_001, 160)],
                "world {}",
                w.number
            );
            assert_eq!(surrendered, vec![(5, 900_001, 40)], "world {}", w.number);
        } else {
            // 600 against 200 and 200: 200 lost; 150 from each, cut to 120.
            assert_eq!(
                lost,
                vec![(5, 1, 200), (5, 900_001, 120), (5, 900_002, 120)],
                "world {}",
                w.number
            );
            assert!(surrendered.is_empty(), "world {}: not beaten", w.number);
        }
    }
}
