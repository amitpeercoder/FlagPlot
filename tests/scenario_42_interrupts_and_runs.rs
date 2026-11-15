//! Scenario 42: interrupts, a run of days in one call, the command report,
//! and a save in the middle. A threshold already crossed when it is set, a
//! quiet week, a shortage that lasts (42a); two reasons on one day (42b);
//! an intent that cannot be met, as one matter with a history (42c); and a
//! game saved with a held ship, an amended plan and an acknowledged
//! interrupt, resumed, matching play that was not interrupted (42d).
//!
//! Proves step 4 of decision 0107 (step 3, stage 1: interrupts, `run`, the
//! command report, resuming; rulings 31 to 40 there) against a hand
//! calculation. The data is placeholder data: see
//! `data/scenarios/42a_a_shortage_stops_the_clock.toml`,
//! `42b_two_reasons_one_day.toml`, `42c_displaced_under_run.toml` and
//! `42d_saved_in_the_middle.toml`. It runs on the frozen step 1 figures.
//! There is no enemy, no aircraft and no random draw.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## The rules
//!
//! - **An interrupt is tested each morning,** when the staff is asked: on
//!   the reading of that day and on what the staff hands in that morning.
//! - **It fires on a change.** An interrupt is armed when it is set. When
//!   its condition holds and it is armed, it fires and is no longer armed.
//!   On a morning its condition does not hold, it is armed again.
//!   Acknowledging it does not arm it.
//! - **A threshold** is set for a base and a number of days: its condition
//!   holds when the hex's days of supply are below the number. With no
//!   figure it does not hold.
//! - **Three interrupts stand without being set:** the staff cannot meet an
//!   intent; a plan waits for the commander's leave; a ship of the side is
//!   lost. Each is armed apart for each plan, or each ship. The staff
//!   cannot meet an intent on a day it needs a run and can have none, or
//!   the run it can have leaves the reading it checks below the floor.
//! - **A run of days** resolves day after day, up to its number, and stops
//!   after a day on which any interrupt of the side fired, or at the end of
//!   the game. Everything that fired that day is listed together.
//! - **A matter** in the command report is a plan, a run where there is
//!   one, and a kind. It is open on each day the staff hands it in, and
//!   one matter however many days it lasts.
//!
//! # 42a: a threshold crossed when set, a quiet week, a shortage that lasts
//!
//! ## Inputs
//!
//! Scenario 36 whole: 18 days, both leaves given. One more command on day
//! 1: interrupt 1, base 2 below 35 days of supply.
//!
//! ## What happens
//!
//! The readings at T are scenario 36's. In days: 10 on day 1, 9, 16, then
//! 43 on day 4 falling by one a day to 30 on day 17, and 37 on day 18.
//!
//! | Call | Days resolved | Why it stops |
//! | ---- | ------------- | ------------ |
//! | `run` of 30 days | Day 1 | Interrupt 1 fired on day 1: 10 days, below 35. It was already true when it was set |
//! | `run` of 30 days | Days 2 to 13, twelve days | Interrupt 1 fired on day 13: 34 days |
//! | `acknowledge` interrupt 1, then `run` of 30 days | Days 14 to 18, five days | The game is over |
//!
//! - Days 2 and 3 read 9 and 16 days, below 35: the interrupt is not armed
//!   and does not fire. On day 4 the reading is 43 days: it is armed again.
//! - Days 5 to 12 read 42 down to 35 days. 35 is not below 35. The clock
//!   does not stop: a quiet week and more in one call.
//! - Day 13 reads 34 days: it fires. Days 14 to 17 read 33 down to 30: it
//!   does not fire again, acknowledged or not. Day 18 reads 37: armed.
//! - Interrupt 1 fired twice in the game, on days 1 and 13. No standing
//!   interrupt fired: the staff met the intent and waited for no leave.
//!
//! Every order, reading, stock and hour is scenario 36's, and the end
//! state is scenario 36's.
//!
//! The first call's answer lists the staff's orders for day 1 before the
//! day is resolved: the force formed and the load of 360 t.
//!
//! ## Properties of 42a
//!
//! - Played by `end_day` eighteen times in place of the three calls, the
//!   game has the same hash at the end of every day.
//! - With the acknowledgement left out, the third call resolves the same
//!   five days.
//! - With the threshold at 30 days in place of 35, it fires on day 1 only:
//!   the reading is never again below 30 after day 4.
//! - A `run` of 5 days from the start of day 2 resolves days 2 to 6 and
//!   stops for its number, having fired nothing.
//!
//! # 42b: two reasons on one day
//!
//! ## Inputs
//!
//! Scenario 36 with no leave set (leave to prepare is given by default,
//! leave to launch is not), 18 days. Commands of day 1: the intent, the
//! forces limit and the reserve of scenario 36, and interrupt 1, base 2
//! below 10 days of supply. On day 3 the commander approves the plan.
//!
//! ## What happens
//!
//! | Call | Days resolved | Why it stops |
//! | ---- | ------------- | ------------ |
//! | `run` of 30 days | Days 1 and 2 | Two fired on day 2, listed together: interrupt 1 (9 days, below 10), and the plan waits for leave to launch |
//! | `approve` the plan, then `run` of 30 days | Days 3 to 18, sixteen days | The game is over |
//!
//! - Day 1 reads 10 days, not below 10, and the staff makes the draft
//!   ready by the leave it has: nothing fires.
//! - Day 2 reads 9 days: interrupt 1 fires. Run 1 is loaded and the plan
//!   is not released: the standing interrupt for a plan that waits fires.
//!   The clock stops once.
//! - Day 3 reads 8 days and day 4 reads 15: interrupt 1 does not fire on
//!   day 3 and is armed again on day 4. The plan is released on day 3 and
//!   waits no more.
//! - The readings at T in tons: 100, 90, 80, 150, then 420 on day 5 falling
//!   by 10 a day to 310 on day 16 and 300 on day 17, and 370 on day 18. A
//!   second run of 150 t is ordered on day 16 and sails on day 17.
//!
//! At the end of day 18: base 2 holds 430 t; base 1 holds 4490 t of supply
//! and 1960 t of fuel; ship 1 is at sea, empty, 80 nm from S, with 968 t of
//! fuel; ship 2 has not moved.
//!
//! The command report asked after day 2 has one open matter, plan 1 waits
//! for leave to launch, opened on day 2, and interrupt 1 as fired and not
//! acknowledged. Asked at the end it has that matter closed, open on day 2
//! only.
//!
//! ## Property of 42b
//!
//! - **A convoy that waits a week is one matter.** With no approval on day
//!   3, the first call stops after day 2 and a second `run` of 7 days
//!   resolves days 3 to 9 and stops for its number, having fired nothing:
//!   the plan still waits and is not a new change. The report then has one
//!   open matter, plan 1 waits for leave to launch, opened on day 2, with a
//!   history of days 2 to 9, each leading to that day's reason.
//!
//! # 42c: an intent that cannot be met, under a run
//!
//! ## Inputs
//!
//! Scenario 40a whole, 16 days, with no command added. The standing
//! interrupts are on.
//!
//! ## What happens
//!
//! | Call | Days resolved | Why it stops |
//! | ---- | ------------- | ------------ |
//! | `run` of 30 days | Day 1 | The staff cannot meet intent 2: displaced by ship 1's run for intent 1 |
//! | `run` of 30 days | Days 2 to 10, nine days | The staff cannot meet intent 1: displaced by ship 1's run for intent 2 |
//! | `run` of 30 days | Days 11 to 16, six days | The game is over |
//!
//! - Intent 2 cannot be met on days 1 to 4: it fires on day 1 and not on
//!   days 2 to 4. On day 5 it is met and is armed again; it does not fire
//!   after.
//! - Intent 1 cannot be met on days 10 to 13: it fires on day 10 only.
//! - Every order, reading and stock is scenario 40a's.
//!
//! The command report at the end has two matters, both closed: intent 2's
//! plan could not be met, days 1 to 4; intent 1's plan could not be met,
//! days 10 to 13. Each day of each leads to that day's reason, which names
//! the commitment that displaced it.
//!
//! ## Property of 42c
//!
//! - On scenario 40b a `run` of 30 days stops after day 1 (the staff
//!   cannot meet intent 2: its load is cut to 14 t by the source, which
//!   leaves the reading of day 3 at 22 t, below the floor), and a second
//!   resolves days 2 to 5 to the end of the game, having fired nothing:
//!   intent 2 is short on every day. The report has one open matter with a
//!   history of days 1 to 5.
//! - A load cut that still leaves the reading it checks at or above the
//!   floor is not a matter: in 40a intent 1's first load is cut from 36 t
//!   to the ship's 30 t, the reading of day 3 is 38 t, and nothing fires
//!   for intent 1 on day 1.
//!
//! # 42d: saved in the middle
//!
//! ## Inputs
//!
//! Scenario 41a whole, 27 days, with two more commands: on day 1,
//! interrupt 1, base 2 below 38 days of supply; on day 11, interrupt 1 is
//! acknowledged.
//!
//! ## What happens
//!
//! The readings at T are scenario 41a's. In days of supply: 10, 9, 8 and
//! 15 on days 1 to 4; 42 on day 5 falling by one a day to 37 on day 10; 36
//! on day 11; 43 on day 12; 53 on day 13 falling by one a day to 40 on day
//! 26; 47 on day 27.
//!
//! - Interrupt 1 fires on day 1 (10 days). It is armed again on day 5 (42
//!   days), fires on day 10 (37 days), and is armed again on day 12. It
//!   does not fire after: the lowest later reading is 40 days.
//! - The plan waits for leave to launch on day 2: that fires on day 2.
//! - The game is saved at the end of day 12: ship 1 is held, the plan has
//!   one amendment, and interrupt 1 has fired twice and is acknowledged.
//! - Resumed from the save and played to day 27, the game has the same
//!   hash at the end of every day from 13 to 27 as the game played through
//!   without a save, and scenario 41a's end state.
//! - The command report asked on the resumed game after day 12 is, word
//!   for word, the report of the game that was not saved.
//!
//! ## Properties of 42d
//!
//! - A check of the saved game against the staff passes.
//! - Resuming the save for more play under another doctrine version is
//!   refused, and says which version the game was begun under. Replaying it
//!   is not refused.

// ------------------------------------------------------------------ The tests
//
// The games are played through `sim_agent::Session`, the Allied seat a
// player's and the Japanese the script's, the doctrine's staffs passed in,
// the scenario's scripted commands given to the session as `session_at` of
// scenario 41 gives them, on the frozen step 1 figures. The readings the
// table names are read as the staff reads them, at the staff's moment of each
// day, in a game of its own (`readings`): the interrupts change no reading.

mod common;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::choices::Moment;
use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, ShipId, TaskForceId};
use sim_core::orders::OrderRequest;
use sim_core::state::{ShipLocation, Side, TaskForceStatus, TruthState};
use sim_core::World;
use sim_staff::{as_staffs, staffs};

const S: Hex = Hex::new(0, 0);
const T: Hex = Hex::new(5, 0);

/// The number the engine gives the force formed on day 1 (decision 0086).
const FORCE: u32 = 1_000_001;

fn scenario_path(file: &str) -> PathBuf {
    common::data_dir().join("scenarios").join(file)
}

fn path_a() -> PathBuf {
    scenario_path("42a_a_shortage_stops_the_clock.toml")
}

fn path_b() -> PathBuf {
    scenario_path("42b_two_reasons_one_day.toml")
}

fn path_c() -> PathBuf {
    scenario_path("42c_displaced_under_run.toml")
}

fn path_d() -> PathBuf {
    scenario_path("42d_saved_in_the_middle.toml")
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn base(raw: u32) -> BaseId {
    BaseId::new(raw)
}

fn ship(raw: u32) -> ShipId {
    ShipId::new(raw)
}

fn force(raw: u32) -> TaskForceId {
    TaskForceId::new(raw)
}

/// A session of the Allied seat (a player's) with the doctrine's staffs, the
/// scenario's scripted commands given to it, over the text of a scenario
/// loaded on the frozen figures.
fn session_of(text: &str, path: &Path) -> Session {
    let mut scenario = common::from_text(text, path).expect("the scenario loads");
    let commands = scenario.scripted_commands().clone();
    scenario.replace_commands(BTreeMap::new());
    let name = scenario.name.clone();
    Session::new(
        scenario.world,
        &name,
        scenario.days,
        Seat::Player,
        Seat::Script(BTreeMap::new()),
    )
    .with_staffs(staffs())
    .with_commands(commands)
}

fn session(path: &Path) -> Session {
    session_of(&read(path), path)
}

fn ask(session: &mut Session, tool: &str, args: Value) -> Value {
    match session.call(Side::Allied, tool, &args) {
        Ok(answer) => answer,
        Err(error) => error.to_json(),
    }
}

fn end_day(session: &mut Session) {
    let a = ask(session, "end_day", json!({}));
    assert_eq!(a["resolved"], true, "{a}");
}

/// A `run` of `days` days.
fn run(session: &mut Session, days: u32) -> Value {
    let a = ask(session, "run", json!({"days": days}));
    assert_eq!(a["kind"], "run", "{a}");
    a
}

fn resolved(a: &Value) -> (u64, u64, u64) {
    let d = &a["days_resolved"];
    (
        d["first"].as_u64().unwrap(),
        d["last"].as_u64().unwrap(),
        d["count"].as_u64().unwrap(),
    )
}

/// What fired in a call, as (day, interrupt number or standing kind, plan,
/// figure).
fn fired_of(a: &Value) -> Vec<(u64, String, Option<u64>, Option<String>)> {
    a["fired"]
        .as_array()
        .expect("fired")
        .iter()
        .map(|f| {
            let what = match f.get("standing") {
                Some(kind) => kind.as_str().unwrap().to_string(),
                None => format!("interrupt:{}", f["interrupt"]),
            };
            (
                f["day"].as_u64().unwrap(),
                what,
                f.get("plan").and_then(Value::as_u64),
                f.get("figure").map(|x| x.as_str().unwrap().to_string()),
            )
        })
        .collect()
}

/// Every firing of the game, from its events: (day, the interrupt in words).
fn fired_events(world: &World) -> Vec<(u32, String)> {
    world
        .events()
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::InterruptFired { interrupt, .. } => Some((e.day, interrupt.to_string())),
            _ => None,
        })
        .collect()
}

/// What the staff read at `t` at its moment of each day: the stock and the
/// days of supply.
fn readings(text: &str, path: &Path, t: Hex) -> BTreeMap<u32, (Fixed, Option<Fixed>)> {
    let mut scenario = common::from_text(text, path).expect("the scenario loads");
    let held = staffs();
    let mut readings = BTreeMap::new();
    loop {
        let day = scenario.world.day() + 1;
        let mut reading = None;
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |world, moment| {
                if moment == (Moment::Orders { day }) {
                    let figure = world
                        .side_view(Side::Allied)
                        .days_of_supply(t)
                        .expect("a figure");
                    reading = Some((figure.stock_t, figure.days));
                }
            })
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
        readings.insert(day, reading.expect("the staff's moment came"));
    }
    readings
}

/// A figure that is a whole number, as one.
fn whole(figure: Fixed) -> i64 {
    assert_eq!(figure, n(figure.trunc()), "{figure} is not whole");
    figure.trunc()
}

/// The days of supply read at T on each day, in whole days.
fn days_at_t(text: &str, path: &Path) -> Vec<i64> {
    readings(text, path, T)
        .values()
        .map(|(_, days)| whole(days.expect("a figure")))
        .collect()
}

fn stocks_at_t(text: &str, path: &Path) -> Vec<i64> {
    readings(text, path, T)
        .values()
        .map(|(stock, _)| whole(*stock))
        .collect()
}

/// A text with `original` once replaced by `changed`.
fn varied(text: &str, original: &str, changed: &str) -> String {
    common::replace_once(text, original, changed)
}

/// The commander's approval of plan 1 on day 3, as a command table.
const APPROVAL_ON_3: &str =
    "\n[[command]]\nday = 3\nside = \"allied\"\nkind = \"approve\"\nplan = 1\n";

fn why_of(session: &mut Session, number: &Value) -> Value {
    let w = ask(session, "why", json!({ "number": number }));
    assert_eq!(w["kind"], "why", "{w}");
    w
}

/// The end of a game: its truth state.
fn truth_of(session: Session) -> TruthState {
    session.into_world().umpire().clone()
}

/// Two truth states alike in everything the game is about: ships, bases,
/// land units and air groups, and each force's ships, route, place and
/// status. The numbers of events and orders are left out: a firing is an
/// event, so the games that fire interrupts number the events after it
/// differently from the games that do not.
fn assert_same_world(left: &TruthState, right: &TruthState) {
    assert_eq!(left.day, right.day);
    assert_eq!(left.ships, right.ships);
    assert_eq!(left.bases, right.bases);
    assert_eq!(left.land_units, right.land_units);
    assert_eq!(left.air_groups, right.air_groups);
    let shape = |state: &TruthState| {
        state
            .task_forces
            .iter()
            .map(|(id, f)| {
                (
                    *id,
                    f.ships.clone(),
                    f.route.waypoints.clone(),
                    f.route.next_index,
                    f.from_hex,
                    f.progress_nm,
                    f.status,
                    f.arrived_at_hours,
                )
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(shape(left), shape(right));
}

// ------------------------------------------------------------------- 42a

/// The three calls of 42a: a `run` of 30, a `run` of 30, then, the interrupt
/// acknowledged (or not), a `run` of 30.
fn play_a(text: &str, acknowledge: bool) -> (Session, Vec<Value>) {
    let mut session = session_of(text, &path_a());
    let first = run(&mut session, 30);
    let second = run(&mut session, 30);
    if acknowledge {
        let a = ask(&mut session, "acknowledge", json!({"interrupt": 1}));
        assert_eq!(a["status"], "queued", "{a}");
    }
    let third = run(&mut session, 30);
    (session, vec![first, second, third])
}

#[test]
fn a_the_readings_at_t_are_scenario_36s() {
    let days = days_at_t(&read(&path_a()), &path_a());
    assert_eq!(days.len(), 18);
    assert_eq!(&days[..4], &[10, 9, 16, 43]);
    // 43 on day 4 falling by one a day to 30 on day 17, and 37 on day 18.
    for day in 4..=17 {
        assert_eq!(days[day - 1], 43 - (day as i64 - 4), "day {day}");
    }
    assert_eq!(days[16], 30);
    assert_eq!(days[17], 37);
    // 35 is read on day 12 and is not below 35; 34 on day 13.
    assert_eq!(days[11], 35);
    assert_eq!(days[12], 34);
}

#[test]
fn a_three_calls_follow_the_table() {
    let (session, calls) = play_a(&read(&path_a()), true);

    // Call 1: day 1 only. Interrupt 1 fired on day 1: 10 days, below 35.
    let first = &calls[0];
    assert_eq!(resolved(first), (1, 1, 1), "{first}");
    assert_eq!(first["stopped"], "interrupt");
    assert_eq!(first["game_over"], false);
    assert_eq!(
        fired_of(first),
        vec![(1, "interrupt:1".to_string(), None, Some("10".to_string()))]
    );
    assert_eq!(first["fired"][0]["below_days"], 35);
    assert_eq!(first["fired"][0]["base"]["handle"], "base:2");
    assert_eq!(first["fired"][0]["acknowledged"], false);

    // The staff's orders of day 1, before the day resolved: the force formed
    // and the load of 360 t.
    assert_eq!(first["staff_orders"]["for_day"], 1);
    let orders = first["staff_orders"]["orders"].as_array().unwrap();
    assert!(
        orders
            .iter()
            .any(|o| o["order"]["kind"] == "form_task_force"),
        "{}",
        first["staff_orders"]
    );
    let load = orders
        .iter()
        .find(|o| o["order"]["kind"] == "load")
        .unwrap_or_else(|| panic!("a load order: {}", first["staff_orders"]));
    assert_eq!(load["order"]["weight_t"], "360", "{load}");

    // Call 2: days 2 to 13, twelve days. Interrupt 1 fired on day 13: 34 days.
    let second = &calls[1];
    assert_eq!(resolved(second), (2, 13, 12), "{second}");
    assert_eq!(second["stopped"], "interrupt");
    assert_eq!(
        fired_of(second),
        vec![(13, "interrupt:1".to_string(), None, Some("34".to_string()))]
    );

    // Call 3, after the acknowledgement: days 14 to 18, five days; the game
    // is over and nothing fired.
    let third = &calls[2];
    assert_eq!(resolved(third), (14, 18, 5), "{third}");
    assert_eq!(third["stopped"], "game_over");
    assert_eq!(third["game_over"], true);
    assert_eq!(fired_of(third), vec![]);

    // Interrupt 1 fired twice in the game, on days 1 and 13; no standing
    // interrupt fired.
    let world = session.into_world();
    let fired = fired_events(&world);
    assert_eq!(fired.len(), 2, "{fired:?}");
    assert_eq!(fired[0].0, 1);
    assert_eq!(fired[1].0, 13);
    assert!(
        fired.iter().all(|(_, what)| what == "interrupt:1"),
        "{fired:?}"
    );
}

#[test]
fn a_the_command_state_and_the_report_say_the_interrupt_fired_and_was_seen() {
    let mut session = session_of(&read(&path_a()), &path_a());
    run(&mut session, 30);
    run(&mut session, 30);
    // After day 13 it has fired and is not acknowledged; no matter is open.
    let r = ask(&mut session, "report", json!({}));
    assert_eq!(r["day"], 13);
    assert_eq!(r["open_matters"], 0);
    assert_eq!(r["matters"], json!([]));
    assert_eq!(r["interrupts"]["set"][0]["last_fired"], 13);
    assert_eq!(r["interrupts"]["set"][0]["acknowledged"], false);
    assert_eq!(r["interrupts"]["standing"], json!([]));
    ask(&mut session, "acknowledge", json!({"interrupt": 1}));
    run(&mut session, 30);
    let r = ask(&mut session, "report", json!({}));
    assert_eq!(r["day"], 18);
    assert_eq!(r["interrupts"]["set"][0]["last_fired"], 13);
    assert_eq!(r["interrupts"]["set"][0]["acknowledged"], true);
    assert_eq!(r["interrupts"]["set"][0]["armed"], true);
    // No standing interrupt fired, so there is no matter at all.
    assert_eq!(r["matters"], json!([]));
    assert_eq!(r["interrupts"]["standing"], json!([]));
}

#[test]
fn a_the_end_state_and_every_order_are_scenario_36s() {
    // The same game with the interrupt left out: the file of scenario 36,
    // played by `end_day`. The staff's hand-ins, the orders and the truth at
    // the end are the same: the interrupt changes no order and no state of
    // the world but the command state.
    let (with, _) = play_a(&read(&path_a()), true);
    let plain_path = scenario_path("36_one_convoy.toml");
    let mut plain = session(&plain_path);
    for _ in 0..18 {
        end_day(&mut plain);
    }
    assert_eq!(with.staff_handed_in(), plain.staff_handed_in());
    assert_eq!(with.handed_in(), plain.handed_in());
    assert_same_world(&truth_of(with), &truth_of(plain));
}

#[test]
fn a_end_day_eighteen_times_gives_the_same_hash_every_day_as_the_three_calls() {
    let (calls, _) = play_a(&read(&path_a()), true);
    // The same game by `end_day`: the acknowledgement the third call was
    // given is queued before day 14 resolves.
    let mut by_hand = session(&path_a());
    for day in 1..=18 {
        if day == 14 {
            let a = ask(&mut by_hand, "acknowledge", json!({"interrupt": 1}));
            assert_eq!(a["status"], "queued", "{a}");
        }
        end_day(&mut by_hand);
    }
    assert_eq!(calls.day_hashes().len(), 18);
    assert_eq!(calls.day_hashes(), by_hand.day_hashes());
    assert_eq!(calls.state_hash(), by_hand.state_hash());

    // And with no acknowledgement in either, the same again.
    let (calls, _) = play_a(&read(&path_a()), false);
    let mut by_hand = session(&path_a());
    for _ in 0..18 {
        end_day(&mut by_hand);
    }
    assert_eq!(calls.day_hashes(), by_hand.day_hashes());
}

#[test]
fn a_without_the_acknowledgement_the_third_call_resolves_the_same_five_days() {
    let (session, calls) = play_a(&read(&path_a()), false);
    let third = &calls[2];
    assert_eq!(resolved(third), (14, 18, 5), "{third}");
    assert_eq!(third["stopped"], "game_over");
    assert_eq!(fired_of(third), vec![]);
    // Acknowledging does not arm it and changes no part of the world: the
    // days are the same days, and the hashes differ only by the command
    // state, from the day of the acknowledgement.
    let (acknowledged, _) = play_a(&read(&path_a()), true);
    assert_eq!(
        &session.day_hashes()[..13],
        &acknowledged.day_hashes()[..13]
    );
    assert_same_world(&truth_of(session), &truth_of(acknowledged));
}

#[test]
fn a_with_the_threshold_at_30_it_fires_on_day_1_only() {
    let text = varied(&read(&path_a()), "below_days = 35", "below_days = 30");
    let mut session = session_of(&text, &path_a());
    let first = run(&mut session, 30);
    assert_eq!(resolved(&first), (1, 1, 1), "{first}");
    assert_eq!(
        fired_of(&first),
        vec![(1, "interrupt:1".to_string(), None, Some("10".to_string()))]
    );
    // The reading is never again below 30 after day 4: the rest is one call.
    let second = run(&mut session, 30);
    assert_eq!(resolved(&second), (2, 18, 17), "{second}");
    assert_eq!(second["stopped"], "game_over");
    assert_eq!(fired_of(&second), vec![]);
    let fired = fired_events(&session.into_world());
    assert_eq!(fired.len(), 1, "{fired:?}");
    assert_eq!(fired[0].0, 1);
}

#[test]
fn a_a_run_of_5_from_the_start_of_day_2_stops_for_its_number_having_fired_nothing() {
    let mut session = session(&path_a());
    end_day(&mut session);
    assert_eq!(session.day(), 1);
    let a = run(&mut session, 5);
    assert_eq!(resolved(&a), (2, 6, 5), "{a}");
    assert_eq!(a["stopped"], "days");
    assert_eq!(a["game_over"], false);
    assert_eq!(fired_of(&a), vec![]);
}

// ------------------------------------------------------------------- 42b

/// 42b with the commander's approval on day 3, as the table's second row
/// has it, given by the tool before the second call.
fn play_b() -> (Session, Value, Value, Value, Value) {
    let mut session = session(&path_b());
    let first = run(&mut session, 30);
    let report_after_2 = ask(&mut session, "report", json!({}));
    let approved = ask(&mut session, "approve", json!({"plan": 1}));
    assert_eq!(approved["status"], "queued", "{approved}");
    let second = run(&mut session, 30);
    let report_at_end = ask(&mut session, "report", json!({}));
    (session, first, report_after_2, second, report_at_end)
}

#[test]
fn b_the_readings_at_t_follow_the_table() {
    // With the approval on day 3, as the game is played.
    let text = format!("{}{APPROVAL_ON_3}", read(&path_b()));
    let stocks = stocks_at_t(&text, &path_b());
    let mut table = vec![100, 90, 80, 150];
    for day in 5..=16 {
        table.push(420 - 10 * (day - 5));
    }
    table.extend([300, 370]);
    assert_eq!(stocks, table);
    let days: Vec<i64> = stocks.iter().map(|s| s / 10).collect();
    // 10 days on day 1, 9 on day 2, 8 on day 3, 15 on day 4.
    assert_eq!(&days[..4], &[10, 9, 8, 15]);
}

#[test]
fn b_two_fire_on_day_2_listed_together_and_the_second_call_runs_to_the_end() {
    let (session, first, _, second, _) = play_b();

    assert_eq!(resolved(&first), (1, 2, 2), "{first}");
    assert_eq!(first["stopped"], "interrupt");
    assert_eq!(first["game_over"], false);
    // Day 1 reads 10 days, not below 10: nothing fired. Day 2 fired two,
    // listed together: interrupt 1 (9 days, below 10) and the plan waits for
    // leave to launch.
    assert_eq!(
        fired_of(&first),
        vec![
            (2, "interrupt:1".to_string(), None, Some("9".to_string())),
            (2, "waits_for_leave".to_string(), Some(1), None),
        ]
    );
    assert_eq!(first["fired"][0]["below_days"], 10);
    assert_eq!(first["fired"][0]["base"]["handle"], "base:2");
    assert_eq!(first["fired"][1]["acknowledged"], false);

    // The staff's orders of day 1: a force formed and the load of 360 t.
    let orders = first["staff_orders"]["orders"].as_array().unwrap();
    assert!(orders
        .iter()
        .any(|o| o["order"]["kind"] == "form_task_force"));
    let load = orders
        .iter()
        .find(|o| o["order"]["kind"] == "load")
        .expect("a load order");
    assert_eq!(load["order"]["weight_t"], "360");

    // The second call, after the approval: days 3 to 18, sixteen days, and
    // the game is over. Interrupt 1 did not fire on day 3 (8 days: it is
    // not armed) and is armed again on day 4.
    assert_eq!(resolved(&second), (3, 18, 16), "{second}");
    assert_eq!(second["stopped"], "game_over");
    assert_eq!(second["game_over"], true);
    assert_eq!(fired_of(&second), vec![]);
    // The plan was released on day 3.
    assert_eq!(second["staff_orders"]["for_day"], 3);

    // Two events of firing, both on day 2, and no other.
    let world = session.into_world();
    let fired = fired_events(&world);
    assert_eq!(fired.len(), 2, "{fired:?}");
    assert!(fired.iter().all(|(day, _)| *day == 2), "{fired:?}");
}

#[test]
fn b_the_end_of_day_18() {
    let (session, ..) = play_b();
    let world = session.into_world();
    let end = world.umpire();
    // Base 2 holds 430 t; base 1 holds 4490 t of supply and 1960 t of fuel.
    assert_eq!(end.bases[&base(2)].supply_t, n(430));
    assert_eq!(end.bases[&base(1)].supply_t, n(4490));
    assert_eq!(end.bases[&base(1)].fuel_t, n(1960));
    // Ship 1 is at sea, empty, 80 nm from S, with 968 t of fuel.
    let one = &end.ships[&ship(1)];
    assert_eq!(one.location, ShipLocation::TaskForce(force(FORCE)));
    assert!(one.supply_t.is_zero() && one.cargo_fuel_t.is_zero());
    assert_eq!(one.fuel_t, n(968));
    let formed = &end.task_forces[&force(FORCE)];
    assert_eq!(formed.status, TaskForceStatus::Underway);
    let from_s = |hex: Hex| i64::from(hex.distance(S)) * 40;
    assert_eq!(from_s(formed.from_hex) - formed.progress_nm.trunc(), 80);
    // Ship 2 has not moved.
    let two = &end.ships[&ship(2)];
    assert_eq!(two.location, ShipLocation::Port(base(1)));
    assert_eq!(two.fuel_t, n(1000));
    assert!(two.supply_t.is_zero() && two.cargo_fuel_t.is_zero());
}

#[test]
fn b_the_report_after_day_2_has_one_open_matter_and_at_the_end_it_is_closed() {
    let (_, _, after_2, _, at_end) = play_b();

    assert_eq!(after_2["day"], 2);
    assert_eq!(after_2["open_matters"], 1);
    assert_eq!(after_2["closed_matters"], 0);
    let matters = after_2["matters"].as_array().unwrap();
    assert_eq!(matters.len(), 1, "{after_2}");
    let matter = &matters[0];
    assert_eq!(matter["plan"], 1);
    assert_eq!(matter["kind"], "waits_for_leave_to_launch");
    assert_eq!(matter["words"], "plan:1 waits for leave to launch");
    assert_eq!(matter["open"], true);
    assert_eq!(matter["opened"], 2);
    assert_eq!(matter["closed"], Value::Null);
    assert_eq!(matter["run"], Value::Null);
    let days: Vec<u64> = matter["history"]
        .as_array()
        .unwrap()
        .iter()
        .map(|h| h["day"].as_u64().unwrap())
        .collect();
    assert_eq!(days, vec![2]);
    // Interrupt 1 fired and not acknowledged.
    let set = &after_2["interrupts"]["set"][0];
    assert_eq!(set["interrupt"], 1);
    assert_eq!(set["last_fired"], 2);
    assert_eq!(set["acknowledged"], false);
    assert_eq!(set["needs_acknowledgement"], true);
    assert_eq!(after_2["fired_last_day"].as_array().unwrap().len(), 2);

    // At the end that matter is closed, open on day 2 only.
    assert_eq!(at_end["day"], 18);
    assert_eq!(at_end["open_matters"], 0);
    assert_eq!(at_end["closed_matters"], 1);
    let matters = at_end["matters"].as_array().unwrap();
    assert_eq!(matters.len(), 1, "{at_end}");
    assert_eq!(matters[0]["kind"], "waits_for_leave_to_launch");
    assert_eq!(matters[0]["open"], false);
    assert_eq!(matters[0]["opened"], 2);
    assert_eq!(matters[0]["closed"], 3);
    let days: Vec<u64> = matters[0]["history"]
        .as_array()
        .unwrap()
        .iter()
        .map(|h| h["day"].as_u64().unwrap())
        .collect();
    assert_eq!(days, vec![2]);
    // Interrupt 1 did not fire on day 3 and was armed again on day 4: it
    // fired on day 2 only, and is armed at the end.
    let set = &at_end["interrupts"]["set"][0];
    assert_eq!(set["last_fired"], 2);
    assert_eq!(set["armed"], true);
}

#[test]
fn b_a_convoy_that_waits_a_week_is_one_matter() {
    // No approval on day 3: the file as it is.
    let mut session = session(&path_b());
    let first = run(&mut session, 30);
    assert_eq!(resolved(&first), (1, 2, 2), "{first}");
    // A second run of 7 days resolves days 3 to 9 and stops for its number,
    // having fired nothing: the plan still waits and is not a new change.
    let second = run(&mut session, 7);
    assert_eq!(resolved(&second), (3, 9, 7), "{second}");
    assert_eq!(second["stopped"], "days");
    assert_eq!(fired_of(&second), vec![]);

    let r = ask(&mut session, "report", json!({}));
    assert_eq!(r["day"], 9);
    assert_eq!(r["open_matters"], 1, "{r}");
    assert_eq!(r["closed_matters"], 0);
    let matters = r["matters"].as_array().unwrap();
    assert_eq!(matters.len(), 1);
    let matter = &matters[0];
    assert_eq!(matter["plan"], 1);
    assert_eq!(matter["kind"], "waits_for_leave_to_launch");
    assert_eq!(matter["open"], true);
    assert_eq!(matter["opened"], 2);
    // A history of days 2 to 9, each leading to that day's reason.
    let history = matter["history"].as_array().unwrap();
    let days: Vec<u64> = history.iter().map(|h| h["day"].as_u64().unwrap()).collect();
    assert_eq!(days, (2..=9).collect::<Vec<u64>>());
    for entry in history {
        let w = why_of(&mut session, &entry["why"]);
        assert_eq!(w["day"], entry["day"], "{w}");
        assert_eq!(w["reason"]["serves"]["plan"], 1, "{w}");
        assert!(
            w["reason"]["worked"]
                .as_str()
                .unwrap()
                .contains("waits for leave to launch"),
            "{w}"
        );
    }
}

// ------------------------------------------------------------------- 42c

fn play_c() -> (Session, Vec<Value>) {
    let mut session = session(&path_c());
    let calls: Vec<Value> = (0..3).map(|_| run(&mut session, 30)).collect();
    (session, calls)
}

#[test]
fn c_three_calls_follow_the_table() {
    let (session, calls) = play_c();

    // Day 1: the staff cannot meet intent 2 (plan 2): displaced by ship 1's
    // run for intent 1.
    assert_eq!(resolved(&calls[0]), (1, 1, 1), "{}", calls[0]);
    assert_eq!(calls[0]["stopped"], "interrupt");
    assert_eq!(
        fired_of(&calls[0]),
        vec![(1, "cannot_meet".to_string(), Some(2), None)]
    );
    // Days 2 to 10: the staff cannot meet intent 1 (plan 1).
    assert_eq!(resolved(&calls[1]), (2, 10, 9), "{}", calls[1]);
    assert_eq!(calls[1]["stopped"], "interrupt");
    assert_eq!(
        fired_of(&calls[1]),
        vec![(10, "cannot_meet".to_string(), Some(1), None)]
    );
    // Days 11 to 16: the game is over.
    assert_eq!(resolved(&calls[2]), (11, 16, 6), "{}", calls[2]);
    assert_eq!(calls[2]["stopped"], "game_over");
    assert_eq!(fired_of(&calls[2]), vec![]);

    // Intent 2 fires on day 1 and not on days 2 to 4 or after; intent 1 on
    // day 10 only.
    let fired = fired_events(&session.into_world());
    assert_eq!(fired.len(), 2, "{fired:?}");
    assert_eq!(fired[0].0, 1);
    assert!(fired[0].1.contains("cannot_meet") && fired[0].1.contains("plan:2"));
    assert_eq!(fired[1].0, 10);
    assert!(fired[1].1.contains("cannot_meet") && fired[1].1.contains("plan:1"));
}

#[test]
fn c_every_order_and_stock_is_scenario_40as() {
    let (with, _) = play_c();
    let mut plain = session(&scenario_path("40a_one_pool_two_hexes.toml"));
    for _ in 0..16 {
        end_day(&mut plain);
    }
    // No command was added: the same game, hash for hash.
    assert_eq!(with.day_hashes(), plain.day_hashes());
    assert_eq!(with.staff_handed_in(), plain.staff_handed_in());
    assert_eq!(truth_of(with), truth_of(plain));
}

#[test]
fn c_the_report_has_two_closed_matters_each_day_leading_to_the_commitment_that_displaced_it() {
    let (mut session, _) = play_c();
    let r = ask(&mut session, "report", json!({}));
    assert_eq!(r["day"], 16);
    assert_eq!(r["open_matters"], 0);
    assert_eq!(r["closed_matters"], 2);
    let matters = r["matters"].as_array().unwrap();
    assert_eq!(matters.len(), 2, "{r}");

    // Intent 2's plan could not be met, days 1 to 4; intent 1's, days 10 to
    // 13. The plans are numbered by the intents they serve.
    for (matter, plan, other, days) in [
        (&matters[0], 2u64, 1u64, 1u64..=4),
        (&matters[1], 1u64, 2u64, 10u64..=13),
    ] {
        assert_eq!(matter["plan"], plan, "{matter}");
        assert_eq!(matter["intent"], plan);
        assert_eq!(matter["run"], Value::Null);
        assert_eq!(matter["kind"], "cannot_meet");
        assert_eq!(matter["open"], false);
        assert_eq!(matter["opened"], *days.start());
        assert_eq!(matter["closed"], *days.end() + 1);
        let history = matter["history"].as_array().unwrap();
        let shown: Vec<u64> = history.iter().map(|h| h["day"].as_u64().unwrap()).collect();
        assert_eq!(shown, days.collect::<Vec<_>>());
        for entry in history {
            let w = why_of(&mut session, &entry["why"]);
            assert_eq!(w["day"], entry["day"], "{w}");
            assert_eq!(w["reason"]["serves"]["plan"], plan, "{w}");
            // The reason names the commitment that displaced it: ship 1's
            // run of the other intent's plan.
            let worked = w["reason"]["worked"].as_str().unwrap();
            let named = "displaced by ship:1 committed to run";
            assert!(worked.contains(named), "day {}: {worked}", entry["day"]);
            assert!(
                worked.contains(&format!("of plan:{other} (intent:{other})")),
                "day {}: {worked}",
                entry["day"]
            );
        }
    }
}

#[test]
fn c_on_scenario_40b_a_short_intent_is_one_open_matter_of_five_days() {
    let path = scenario_path("40b_one_stock_two_plans.toml");
    let mut session = session(&path);
    // The staff cannot meet intent 2: its load is cut to 14 t by the source,
    // which leaves the reading of day 3 at 22 t, below the floor.
    let first = run(&mut session, 30);
    assert_eq!(resolved(&first), (1, 1, 1), "{first}");
    assert_eq!(first["stopped"], "interrupt");
    assert_eq!(
        fired_of(&first),
        vec![(1, "cannot_meet".to_string(), Some(2), None)]
    );
    // A second resolves days 2 to 5, to the end of the game, having fired
    // nothing: intent 2 is short on every day.
    let second = run(&mut session, 30);
    assert_eq!(resolved(&second), (2, 5, 4), "{second}");
    assert_eq!(second["stopped"], "game_over");
    assert_eq!(fired_of(&second), vec![]);
    // The report has one open matter with a history of days 1 to 5.
    let r = ask(&mut session, "report", json!({}));
    assert_eq!(r["open_matters"], 1, "{r}");
    assert_eq!(r["closed_matters"], 0);
    let matters = r["matters"].as_array().unwrap();
    assert_eq!(matters.len(), 1);
    assert_eq!(matters[0]["plan"], 2);
    assert_eq!(matters[0]["kind"], "cannot_meet");
    assert_eq!(matters[0]["open"], true);
    assert_eq!(matters[0]["opened"], 1);
    let days: Vec<u64> = matters[0]["history"]
        .as_array()
        .unwrap()
        .iter()
        .map(|h| h["day"].as_u64().unwrap())
        .collect();
    assert_eq!(days, vec![1, 2, 3, 4, 5]);
}

#[test]
fn c_a_load_cut_that_leaves_the_reading_at_or_above_the_floor_is_not_a_matter() {
    // 40a: intent 1's first load is cut from 36 t to the ship's 30 t, the
    // reading of day 3 is 38 t, and nothing fires for intent 1 on day 1.
    let text = read(&scenario_path("40a_one_pool_two_hexes.toml"));
    let path = scenario_path("40a_one_pool_two_hexes.toml");
    let (stock, _) = readings(&text, &path, T)[&3];
    assert_eq!(stock, n(38));
    let mut session = session_of(&read(&path_c()), &path_c());
    let first = run(&mut session, 30);
    assert_eq!(resolved(&first), (1, 1, 1));
    let fired = fired_of(&first);
    assert!(
        fired.iter().all(|(_, _, plan, _)| *plan != Some(1)),
        "nothing fires for intent 1 on day 1: {fired:?}"
    );
    // The staff's day-1 orders gave intent 1 a load of 30 t.
    let orders = first["staff_orders"]["orders"].as_array().unwrap();
    let load = orders
        .iter()
        .find(|o| o["order"]["kind"] == "load")
        .expect("a load order");
    assert_eq!(load["order"]["weight_t"], "30", "{load}");
    assert_eq!(load["intent"], 1);
}

// ------------------------------------------------------------------- 42d

#[test]
fn d_the_readings_at_t_are_scenario_41as() {
    let days = days_at_t(&read(&path_d()), &path_d());
    assert_eq!(days.len(), 27);
    assert_eq!(&days[..4], &[10, 9, 8, 15]);
    for day in 5..=10 {
        assert_eq!(days[day - 1], 42 - (day as i64 - 5), "day {day}");
    }
    assert_eq!(days[9], 37);
    assert_eq!(days[10], 36);
    assert_eq!(days[11], 43);
    for day in 13..=26 {
        assert_eq!(days[day - 1], 53 - (day as i64 - 13), "day {day}");
    }
    assert_eq!(days[25], 40);
    assert_eq!(days[26], 47);
}

#[test]
fn d_the_firings_are_interrupt_1_on_days_1_and_10_and_plan_1_waiting_on_day_2() {
    let mut session = session(&path_d());
    let first = run(&mut session, 30);
    assert_eq!(resolved(&first), (1, 1, 1), "{first}");
    assert_eq!(
        fired_of(&first),
        vec![(1, "interrupt:1".to_string(), None, Some("10".to_string()))]
    );
    let second = run(&mut session, 30);
    assert_eq!(resolved(&second), (2, 2, 1), "{second}");
    assert_eq!(
        fired_of(&second),
        vec![(2, "waits_for_leave".to_string(), Some(1), None)]
    );
    // Armed again on day 5 (42 days), it fires on day 10 (37 days).
    let third = run(&mut session, 30);
    assert_eq!(resolved(&third), (3, 10, 8), "{third}");
    assert_eq!(
        fired_of(&third),
        vec![(10, "interrupt:1".to_string(), None, Some("37".to_string()))]
    );
    // Acknowledged on day 11, armed again on day 12, and never after: the
    // lowest later reading is 40 days.
    let last = run(&mut session, 30);
    assert_eq!(resolved(&last), (11, 27, 17), "{last}");
    assert_eq!(last["stopped"], "game_over");
    assert_eq!(fired_of(&last), vec![]);

    let fired = fired_events(&session.into_world());
    assert_eq!(fired.len(), 3, "{fired:?}");
    assert_eq!(fired[0].0, 1);
    assert_eq!(fired[1].0, 2);
    assert!(fired[1].1.contains("waits_for_leave"), "{fired:?}");
    assert_eq!(fired[2].0, 10);
}

#[test]
fn d_the_game_is_scenario_41as_with_the_interrupts_on_top() {
    let mut with = session(&path_d());
    let mut plain = session(&scenario_path("41a_hold_change_release.toml"));
    for _ in 0..27 {
        end_day(&mut with);
        end_day(&mut plain);
    }
    // Scenario 41a's end state, and the same hand-ins. (The command state
    // holds the interrupts, so the whole hashes are not compared.)
    assert_eq!(with.staff_handed_in(), plain.staff_handed_in());
    let (with, plain) = (with.into_world(), plain.into_world());
    assert_same_world(with.umpire(), plain.umpire());
    // At the end of day 27: base 2 holds 530 t, base 1 holds 4300 t of
    // supply and 1920 t of fuel.
    let end = with.umpire();
    assert_eq!(end.bases[&base(2)].supply_t, n(530));
    assert_eq!(end.bases[&base(1)].supply_t, n(4300));
    assert_eq!(end.bases[&base(1)].fuel_t, n(1920));
}

/// The world of a saved game as a replay builds it: the scenario's world,
/// the orders, the commands and the staff's hand-ins of each day handed in
/// day by day (`World::submit_orders`, `submit_commands`, `submit_staff`),
/// each day resolved with no staff.
fn rebuilt(played: &Session, upto: u32) -> World {
    let mut scenario = common::load(&path_d()).expect("the scenario loads");
    scenario.replace_commands(BTreeMap::new());
    let mut world = scenario.world;
    for day in 1..=upto {
        for side in Side::ALL {
            let orders: Vec<OrderRequest> = played
                .handed_in()
                .get(&day)
                .into_iter()
                .flatten()
                .filter(|(s, _)| *s == side)
                .map(|(_, request)| request.clone())
                .collect();
            world.submit_orders(side, orders);
        }
        for (side, command) in played.commands_handed_in().get(&day).into_iter().flatten() {
            world.submit_commands(*side, vec![command.clone()]);
        }
        for (side, decisions) in played.staff_handed_in().get(&day).into_iter().flatten() {
            world.submit_staff(*side, decisions.clone());
        }
        world.resolve_day().expect("a day resolves");
    }
    world
}

/// The scripted commands of 42d from `from` day on.
fn commands_from(from: u32) -> BTreeMap<u32, Vec<(Side, sim_core::command::CommandKind)>> {
    let scenario = common::load(&path_d()).expect("the scenario loads");
    scenario
        .scripted_commands()
        .iter()
        .filter(|(day, _)| **day >= from)
        .map(|(day, commands)| (*day, commands.clone()))
        .collect()
}

#[test]
fn d_resumed_from_a_save_at_day_12_it_is_the_game_that_was_not_saved() {
    // The game played through, by `end_day`, and the same to day 12 for the
    // save.
    let mut through = session(&path_d());
    let mut report_through_12 = Value::Null;
    for day in 1..=27 {
        end_day(&mut through);
        if day == 12 {
            report_through_12 = ask(&mut through, "report", json!({}));
        }
    }
    assert_eq!(through.day_hashes().len(), 27);

    let mut saved = session(&path_d());
    for _ in 1..=12 {
        end_day(&mut saved);
    }
    assert_eq!(saved.day(), 12);

    // What a saved game keeps, replayed as `sim-cli replay` does.
    let world = rebuilt(&saved, 12);
    assert_eq!(world.day(), 12);
    assert_eq!(world.state_hash(), saved.state_hash());
    assert_eq!(world.day_hashes(), saved.day_hashes());

    // At day 12: ship 1 is held, the plan has one amendment, interrupt 1
    // has fired twice (days 1 and 10) and is acknowledged.
    let command_state = world.command_state(Side::Allied);
    assert!(command_state
        .holds
        .contains_key(&sim_core::command::HoldTarget::Ship(ship(1))));
    assert_eq!(command_state.plans.len(), 1);
    let plan = command_state.plans.values().next().unwrap();
    assert_eq!(plan.amendments.len(), 1);
    let fired: Vec<u32> = fired_events(&world)
        .iter()
        .filter(|(_, what)| what == "interrupt:1")
        .map(|(day, _)| *day)
        .collect();
    assert_eq!(fired, vec![1, 10]);
    let set = command_state
        .interrupts
        .values()
        .next()
        .expect("interrupt 1 is set");
    assert_eq!(set.status.last_fired, Some(10));
    assert!(set.status.acknowledged);

    // A new session over that world, with the staffs and the scripted
    // commands of days 13 to 27.
    let name = common::load(&path_d()).unwrap().name;
    let mut resumed = Session::new(
        world,
        &name,
        27,
        Seat::Player,
        Seat::Script(BTreeMap::new()),
    )
    .with_staffs(staffs())
    .with_commands(commands_from(13));
    assert_eq!(resumed.day(), 12);

    // The report after day 12 is, word for word, the uninterrupted game's.
    let report_resumed_12 = ask(&mut resumed, "report", json!({}));
    assert_eq!(report_resumed_12["day"], 12);
    assert_eq!(report_resumed_12, report_through_12);
    // At day 12 the command state is the same in what the tool says too.
    let held = ask(&mut resumed, "command_state", json!({}));
    assert_eq!(held["holds"][0]["target"], json!({"ship": 1}));
    assert_eq!(held["plans"][0]["amendments"].as_array().unwrap().len(), 1);
    assert_eq!(held["interrupts"]["set"][0]["acknowledged"], true);
    assert_eq!(held["interrupts"]["set"][0]["last_fired"], 10);

    // Played to day 27 by `end_day`: the same hash at the end of every day
    // from 13 to 27 as the game played through without a save.
    for _ in 13..=27 {
        end_day(&mut resumed);
    }
    assert_eq!(resumed.day(), 27);
    assert_eq!(resumed.day_hashes().len(), 27);
    assert_eq!(&resumed.day_hashes()[12..], &through.day_hashes()[12..]);
    assert_eq!(resumed.day_hashes(), through.day_hashes());
    // The report at the end is the uninterrupted game's too, and the end
    // state is scenario 41a's: 530 t at base 2.
    assert_eq!(
        ask(&mut resumed, "report", json!({})),
        ask(&mut through, "report", json!({}))
    );
    let end = truth_of(resumed);
    assert_eq!(end.bases[&base(2)].supply_t, n(530));
    assert_eq!(end, truth_of(through));
}

// ---------------------------------------------------- Through the binary
//
// `run`, `replay` and `hash` read the frozen figures when they are given
// `--reference`. `play` has no such option: it reads the live figures, so
// what it is asked here is only what the figures do not change (the header of
// its game file and whether it is refused).

fn cargo() -> Command {
    let mut command = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string()));
    command.current_dir(common::data_dir().join(".."));
    command
}

/// One run of the binary at a time: each is a `cargo run`, and they share a
/// build folder.
static BINARY: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn sim_cli(arguments: &[&str]) -> std::process::Output {
    let _one_at_a_time = BINARY.lock().unwrap_or_else(|e| e.into_inner());
    let mut command = cargo();
    command.args(["run", "--quiet", "-p", "sim-cli"]);
    if !cfg!(debug_assertions) {
        command.arg("--release");
    }
    command.arg("--").args(arguments);
    command.output().expect("the command runs")
}

fn temp_dir(label: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("flag_plot_s42_{label}_{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a directory");
    dir
}

/// The game run and saved on the frozen figures.
fn save(scenario: &Path, save: &Path) {
    let reference = common::frozen_root();
    let ran = sim_cli(&[
        "run",
        scenario.to_str().unwrap(),
        "--save",
        save.to_str().unwrap(),
        "--reference",
        reference.to_str().unwrap(),
    ]);
    assert!(
        ran.status.success(),
        "run {}: {}",
        scenario.display(),
        String::from_utf8_lossy(&ran.stderr)
    );
}

fn replay(save: &Path, check_staff: bool) -> std::process::Output {
    let reference = common::frozen_root();
    let mut args = vec!["replay", save.to_str().unwrap()];
    if check_staff {
        args.push("--check-staff");
    }
    args.extend(["--reference", reference.to_str().unwrap()]);
    sim_cli(&args)
}

#[test]
fn d_every_game_of_42_saves_and_the_staff_checks() {
    let dir = temp_dir("check");
    for (label, scenario) in [
        ("42a", path_a()),
        ("42b", path_b()),
        ("42c", path_c()),
        ("42d", path_d()),
    ] {
        let file = dir.join(format!("{label}.json"));
        save(&scenario, &file);
        let replayed = replay(&file, true);
        let said = String::from_utf8_lossy(&replayed.stdout).to_string();
        assert!(
            replayed.status.success(),
            "replay {label}: {said} {}",
            String::from_utf8_lossy(&replayed.stderr)
        );
        assert!(
            said.contains("the staff gave what was saved on every day"),
            "{label}: {said}"
        );
    }
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn d_a_save_under_another_doctrine_version_still_replays() {
    let dir = temp_dir("version");
    let file = dir.join("42d.json");
    save(&path_d(), &file);
    let text = std::fs::read_to_string(&file).unwrap();
    let held = sim_staff::DOCTRINE_VERSION;
    let edited: Vec<String> = text
        .lines()
        .map(|line| {
            if line.contains("\"doctrine_version\"") {
                line.replace(&format!("\"{held}\""), "\"supply-edited\"")
            } else {
                line.to_string()
            }
        })
        .collect();
    let edited = edited.join("\n");
    assert!(
        edited.contains("\"supply-edited\""),
        "the header was edited"
    );
    assert_ne!(edited, text);
    std::fs::write(&file, edited).unwrap();
    // Replaying it is not refused (no staff is asked).
    let replayed = replay(&file, false);
    assert!(
        replayed.status.success(),
        "{} {}",
        String::from_utf8_lossy(&replayed.stdout),
        String::from_utf8_lossy(&replayed.stderr)
    );
    std::fs::remove_dir_all(&dir).ok();
}

/// `play` reads the live figures (it has no `--reference`), so this asserts
/// nothing a figure decides: only that a game begun under another doctrine
/// version is refused, and the message names the version it was begun under.
#[test]
fn d_a_play_game_of_another_doctrine_version_is_refused_and_names_it() {
    let dir = temp_dir("play");
    let game = dir.join("game.json");
    let begun = sim_cli(&[
        "play",
        "--game",
        game.to_str().unwrap(),
        "--scenario",
        path_d().to_str().unwrap(),
        "--seat",
        "allied",
        "get_briefing",
    ]);
    assert!(
        begun.status.success(),
        "{}",
        String::from_utf8_lossy(&begun.stderr)
    );
    let text = std::fs::read_to_string(&game).unwrap();
    let held = sim_staff::DOCTRINE_VERSION;
    let header = format!("\"doctrine_version\":\"{held}\"");
    assert_eq!(
        text.matches(&header).count(),
        1,
        "the header names the doctrine"
    );
    std::fs::write(
        &game,
        text.replacen(&header, "\"doctrine_version\":\"supply-edited\"", 1),
    )
    .unwrap();

    let refused = sim_cli(&["play", "--game", game.to_str().unwrap(), "get_briefing"]);
    assert!(!refused.status.success());
    let said = String::from_utf8_lossy(&refused.stderr).to_string();
    assert!(
        said.contains("was begun under doctrine version \"supply-edited\""),
        "{said}"
    );
    std::fs::remove_dir_all(&dir).ok();
}
