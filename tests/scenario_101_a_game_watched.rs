//! Scenario 101: a game watched. Two commanders are at the seats and a
//! third party, the watcher, holds the clock. He steps it, reads what a
//! commander reads, and can order nothing. The game played is the game
//! the same orders give with no watcher.
//!
//! Proves decision 0182. There is no arithmetic in it: the record changes
//! who moves the clock and nothing that happens when it moves. What
//! follows is stated by the architect before the code, and each statement
//! is a test. If the code disagrees, the code or the rule is wrong: a
//! mismatch goes back to the architect.
//!
//! It runs on existing scenarios, through a session opened by `sim-cli`'s
//! library on the live figures, as `serve` opens one. No new data.
//!
//! - **S78:** `78_orders_inside_a_day.toml`, three days, the Allied side
//!   alone, orders inside a day on.
//! - **ES:** `orders-inside-proof/05_eastern_solomons_on.toml`, one day,
//!   both sides, orders inside a day on.
//! - **T:** `01_transit.toml`, three days, orders inside a day off.
//!
//! # The games it is measured against
//!
//! **The plain game of a scenario:** that scenario with the Allied seat
//! alone played and no watcher, the seat making a fixed list of calls
//! that the test states. Its daily hashes and the answers of its calls
//! are the measure. For S78 the calls are those of scenario 78's hand
//! calculation.
//!
//! # Statements
//!
//! 1. **The same calls give the same game.** S78 watched, the Allied
//!    seat making the plain game's calls in order and the watcher
//!    stepping whenever the seat is ready: the same daily hashes, and
//!    each call answered as in the plain game, byte for byte.
//! 2. **Which seat speaks first changes nothing.** ES watched with both
//!    seats played, each giving one order at dusk and one at 0600: with
//!    the Allied seat calling first at each stop, and with the Japanese
//!    first, the same hash at the end and the same answers to each seat.
//! 3. **No step past a seat that has not finished.** At the start a
//!    `step` is refused, naming both seats. With the Allied seat ready
//!    and the Japanese not, it is refused naming the Japanese seat, and
//!    the clock has not moved.
//! 4. **Each seat's stop is its own.** On ES the Allied seat calls
//!    `advance` of one pulse and the Japanese seat `end_day`. After one
//!    step the Allied call is answered and the Japanese is not; a second
//!    step is refused naming the Allied seat; the Allied seat gives an
//!    order at 0600, taken in at the start of the morning, and calls
//!    `advance` of two pulses; after two more steps both are answered,
//!    the Allied first.
//! 5. **A stop on what a side was told.** On ES the Allied seat calls
//!    `advance` of three pulses to stop on a contact: it is answered
//!    after the step of the first pulse in which its side is told of
//!    one, with the answer the plain game of ES gives the same call.
//! 6. **A held seat cannot call.** Between a seat's call of time and
//!    its answer, any other call of that seat is refused, saying that it
//!    is ready and waits for the clock; the hash and the seat's queue
//!    are as they were, and the refusal is in the transcript.
//! 7. **A call of time refused for its arguments** (`advance` of no
//!    pulse) is answered at once, and the seat is not ready.
//! 8. **`run` beside a stop inside a day.** On S78 with both seats
//!    played: the Allied seat calls `run` of two days and the Japanese
//!    seat `advance` of one pulse, then at 0600 `end_day`. The Allied
//!    seat's orders queued at the first dusk go in then and not again;
//!    its call is answered at the second dusk with the plain answer of
//!    `run` of two days.
//! 9. **A held `advance` stopped by an interrupt** at a dusk is answered
//!    there, and the other seat's call is not.
//! 10. **`run` and `advance` are offered to both seats** in a watched
//!     game, and with two seats played and no watcher they are not, as
//!     before. A seat at a command has no `run`, watched or not.
//! 11. **The watcher reads as the seat reads.** For each of his reads,
//!     at dusk and at 0600, his answer for a side is the seat's own at
//!     that moment, byte for byte; the briefing is refused him at 0600
//!     as it is the seat.
//! 12. **The watcher can change nothing.** Each tool that orders,
//!     commands, previews, tries, acknowledges or moves time is refused
//!     him by name, as a tool he does not have. After any number of his
//!     reads the hashes, each seat's transcript and the number of the
//!     seat's next preview are what they would have been.
//! 13. **A seat's answers do not read the other seat's queue.** On ES
//!     with both seats played, at dusk: each read the Allied seat has,
//!     asked with the Japanese queue empty and again with it holding
//!     orders and a command, is the same byte for byte; and the same
//!     the other way. If one differs the build stops and reports it.
//! 14. **He is told where the game stands:** the day and the moment,
//!     each played seat as ready until a named stop or waited for, and
//!     at the end that the game is over. A scripted seat is always
//!     ready.
//! 15. **He is shown what a seat did:** its calls since its last stop,
//!     each with its tool, its arguments and whether it was refused; a
//!     call's `reason` where it gave one.
//! 16. **The step's answer** says the day and pulse resolved, where the
//!     clock stands and which seats' stops came.
//! 17. **At the game's end** every held call is answered, with the
//!     result where the tool gives one; a `step` is refused, the game
//!     being over. With the engine stopped on an error, made by the
//!     test, every held call is answered with the words of the stop.
//! 18. **Saved and replayed.** The saved game of 2 replays, with no
//!     watcher, to the same hash. A game left at 0600 is saved as
//!     stopped inside a day.
//! 19. **A day at a step where a scenario has no orders inside a day:**
//!     T watched with the Allied seat played, calling `end_day` each
//!     dusk: a step resolves a day, `advance` is not offered, and three
//!     steps give the plain game's hashes.
//! 20. **`serve --watched`,** in a test of `sim-cli`: a held call's
//!     answer line comes after the line of the step that made it due,
//!     with the call's own `id`; two due on one step come Allied first;
//!     a seat's tool definitions and opening words are answered; a
//!     request that is none of a seat's call, a step or a request of
//!     the watcher's is answered with an error and changes nothing; and
//!     without `--watched` a step and a watcher's request are refused.
//! 21. **A game not watched is as it was:** the recorded games replay
//!     as they did, and every tool listed for them is one listed before.

use crate::common;

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde_json::{json, Value};
use sim_agent::{
    CallError, Called, Post, SeatKind, Session, Stepped, COMMAND_TOOLS, TOOLS, TRIAL_TOOLS,
    WATCHER_TOOLS,
};
use sim_core::ids::{AirGroupId, TaskForceId};
use sim_core::pictures::CommandRef;
use sim_core::state::{Arrival, Side};
use sim_core::World;
use sim_data::Scenario;

const S78: &str = "78_orders_inside_a_day.toml";
const ES: &str = "orders-inside-proof/05_eastern_solomons_on.toml";
const T: &str = "01_transit.toml";
const A: Side = Side::Allied;
const J: Side = Side::Japanese;

fn path(name: &str) -> String {
    common::data_dir()
        .join("scenarios")
        .join(name)
        .to_str()
        .expect("a path in words")
        .to_string()
}

fn seats(players: &[Side]) -> BTreeMap<Side, SeatKind> {
    players.iter().map(|s| (*s, SeatKind::Player)).collect()
}

/// A game opened as `serve` opens one, on the live figures, with `players`
/// held by players and the other seat the script's.
fn opened(name: &str, players: &[Side]) -> (Session, sim_cli::game::Kept) {
    sim_cli::game::open(&path(name), None, seats(players), &BTreeMap::new())
        .expect("the game opens")
}

fn plain(name: &str, players: &[Side]) -> Session {
    opened(name, players).0
}

fn watched(name: &str, players: &[Side]) -> Session {
    opened(name, players).0.watched()
}

fn doc(outcome: &Result<Value, CallError>) -> Value {
    match outcome {
        Ok(value) => value.clone(),
        Err(error) => error.to_json(),
    }
}

/// A seat's call answered at once.
fn now(s: &mut Session, side: Side, tool: &str, args: Value) -> Value {
    match s.seat_call(side, tool, &args) {
        Called::Answered(outcome) => doc(&outcome),
        Called::Held => panic!("{tool} {args} was held"),
    }
}

/// A seat's call of time, held.
fn hold(s: &mut Session, side: Side, tool: &str, args: Value) {
    let called = s.seat_call(side, tool, &args);
    assert_eq!(called, Called::Held, "{tool} {args} is held");
}

/// A step taken: its answer, and the held calls it answered.
fn step(s: &mut Session) -> (Value, Vec<(Side, Value)>) {
    let Stepped { answer, answered } = s.step();
    let answer = answer.unwrap_or_else(|e| panic!("the step is refused: {e}"));
    (
        answer,
        answered.iter().map(|(side, o)| (*side, doc(o))).collect(),
    )
}

/// A step refused: its words.
fn refused_step(s: &mut Session) -> String {
    let Stepped { answer, answered } = s.step();
    assert!(answered.is_empty());
    answer.expect_err("the step is refused").message
}

fn order(unit: &str, order: Value) -> Value {
    json!({"unit": unit, "order": order})
}

fn transcript(s: &Session) -> Vec<Value> {
    s.transcript().iter().map(|e| e.to_json()).collect()
}

/// A seat's calls in turn, the watcher stepping whenever it is ready: the
/// answer of each call, the answer of a held call as its step gives it.
fn play_watched(s: &mut Session, side: Side, calls: &[(&str, Value)]) -> Vec<Value> {
    let mut answers = Vec::new();
    for (tool, args) in calls {
        match s.seat_call(side, tool, args) {
            Called::Answered(outcome) => answers.push(doc(&outcome)),
            Called::Held => loop {
                let (_, answered) = step(s);
                if let Some((_, answer)) = answered.into_iter().find(|(who, _)| *who == side) {
                    answers.push(answer);
                    break;
                }
            },
        }
    }
    answers
}

fn play_plain(s: &mut Session, side: Side, calls: &[(&str, Value)]) -> Vec<Value> {
    calls
        .iter()
        .map(|(tool, args)| doc(&s.call(side, tool, args)))
        .collect()
}

/// Scenario 78's calls, the hand calculation's orders: the route and the
/// cruise speed at dusk of day 1, a pulse, 8 knots at 0600, then each day
/// ended; with reads between.
fn s78_calls() -> Vec<(&'static str, Value)> {
    vec![
        ("get_briefing", json!({})),
        (
            "issue_orders",
            json!({"orders": [
                order("task_force:1", json!({"kind": "set_route", "waypoints": [[8, 3]]})),
                order("task_force:1", json!({"kind": "set_speed", "speed": "cruise"})),
            ], "reason": "To Espiritu Santo."}),
        ),
        ("advance", json!({"pulses": 1, "reason": "To 0600."})),
        ("events", json!({})),
        ("list_forces", json!({})),
        (
            "issue_orders",
            json!({"orders": [order("task_force:1", json!({"kind": "set_speed", "knots": "8"}))]}),
        ),
        ("end_day", json!({})),
        ("get_briefing", json!({})),
        ("end_day", json!({})),
        ("end_day", json!({})),
        ("get_briefing", json!({})),
    ]
}

// ---------------------------------------------------------------------
// 1. The same calls give the same game.
// ---------------------------------------------------------------------

#[test]
fn s1_the_same_calls_give_the_same_game() {
    let calls = s78_calls();
    let mut p = plain(S78, &[A]);
    let expected = play_plain(&mut p, A, &calls);
    let mut w = watched(S78, &[A]);
    let got = play_watched(&mut w, A, &calls);
    assert_eq!(got.len(), expected.len());
    for (i, (g, e)) in got.iter().zip(&expected).enumerate() {
        assert_eq!(
            serde_json::to_string(g).unwrap(),
            serde_json::to_string(e).unwrap(),
            "call {i} ({})",
            calls[i].0
        );
    }
    assert_eq!(w.day_hashes().len(), 3);
    assert_eq!(w.day_hashes(), p.day_hashes());
    // The seat's transcript is the plain game's.
    assert_eq!(transcript(&w), transcript(&p));
}

// ---------------------------------------------------------------------
// 2. Which seat speaks first changes nothing.
// ---------------------------------------------------------------------

fn dusk_orders(side: Side) -> Value {
    match side {
        Side::Allied => {
            json!({"orders": [order("task_force:1", json!({"kind": "set_readiness", "readiness": "battle"}))]})
        }
        Side::Japanese => {
            json!({"orders": [order("task_force:1", json!({"kind": "set_route", "waypoints": [[8, -11]]}))]})
        }
    }
}

fn morning_orders(side: Side) -> Value {
    match side {
        Side::Allied => {
            json!({"orders": [order("task_force:2", json!({"kind": "set_speed", "speed": "full"}))]})
        }
        Side::Japanese => {
            json!({"orders": [order("task_force:2", json!({"kind": "set_speed", "speed": "full"}))]})
        }
    }
}

/// ES with both seats played, each giving one order at dusk and one at
/// 0600, `first` speaking first at each stop. The session, its kept part
/// and each seat's answers.
fn es_two_orders(first: Side) -> (Session, sim_cli::game::Kept, BTreeMap<Side, Vec<Value>>) {
    let (s, kept) = opened(ES, &[A, J]);
    let mut s = s.watched();
    let order_of = if first == A { [A, J] } else { [J, A] };
    let mut answers: BTreeMap<Side, Vec<Value>> = BTreeMap::new();
    for side in order_of {
        let a = now(&mut s, side, "issue_orders", dusk_orders(side));
        assert_eq!(a["queued_in_call"], 1, "{a}");
        answers.entry(side).or_default().push(a);
        hold(&mut s, side, "advance", json!({"pulses": 1}));
    }
    let (_, answered) = step(&mut s);
    assert_eq!(
        answered.iter().map(|(s, _)| *s).collect::<Vec<_>>(),
        vec![A, J]
    );
    for (side, a) in answered {
        answers.entry(side).or_default().push(a);
    }
    for side in order_of {
        let a = now(&mut s, side, "issue_orders", morning_orders(side));
        assert_eq!(a["queued_in_call"], 1, "{a}");
        answers.entry(side).or_default().push(a);
        hold(&mut s, side, "end_day", json!({}));
    }
    let (_, answered) = step(&mut s);
    assert!(answered.is_empty());
    let (_, answered) = step(&mut s);
    assert_eq!(
        answered.iter().map(|(s, _)| *s).collect::<Vec<_>>(),
        vec![A, J]
    );
    for (side, a) in answered {
        answers.entry(side).or_default().push(a);
    }
    assert!(s.is_over());
    (s, kept, answers)
}

#[test]
fn s2_which_seat_speaks_first_changes_nothing() {
    let (a_first, _, a_answers) = es_two_orders(A);
    let (j_first, _, j_answers) = es_two_orders(J);
    assert_eq!(a_first.state_hash(), j_first.state_hash());
    assert_eq!(a_first.day_hashes(), j_first.day_hashes());
    assert_eq!(a_answers, j_answers);
}

// ---------------------------------------------------------------------
// 3. No step past a seat that has not finished.
// ---------------------------------------------------------------------

#[test]
fn s3_no_step_past_a_seat_that_has_not_finished() {
    let mut s = watched(ES, &[A, J]);
    let (hash, clock) = (s.state_hash(), s.standing()["clock"].clone());
    let words = refused_step(&mut s);
    assert!(
        words.contains("allied") && words.contains("japanese"),
        "{words}"
    );
    hold(&mut s, A, "end_day", json!({}));
    let words = refused_step(&mut s);
    assert!(
        words.contains("japanese seat") && !words.contains("allied"),
        "{words}"
    );
    assert_eq!(s.state_hash(), hash);
    assert_eq!(s.standing()["clock"], clock);
    assert_eq!(s.open_day(), None);
    assert_eq!(s.day(), 0);
}

// ---------------------------------------------------------------------
// 4. Each seat's stop is its own.
// ---------------------------------------------------------------------

#[test]
fn s4_each_seats_stop_is_its_own() {
    let mut s = watched(ES, &[A, J]);
    hold(&mut s, A, "advance", json!({"pulses": 1}));
    hold(&mut s, J, "end_day", json!({}));
    let (answer, answered) = step(&mut s);
    assert_eq!(answer["stops"], json!(["allied"]));
    assert_eq!(answered.len(), 1);
    assert_eq!(answered[0].0, A);
    assert_eq!(answered[0].1["kind"], "advance");
    assert_eq!(answered[0].1["stopped"], "pulses");
    let words = refused_step(&mut s);
    assert!(
        words.contains("allied seat") && !words.contains("japanese"),
        "{words}"
    );
    let given = now(&mut s, A, "issue_orders", morning_orders(A));
    assert_eq!(given["for_pulse"], "morning");
    assert_eq!(given["queued_in_call"], 1);
    hold(&mut s, A, "advance", json!({"pulses": 2}));
    let (_, answered) = step(&mut s);
    assert!(answered.is_empty());
    // The order went in at the start of the morning.
    let inside = &s.handed_in_inside_day()[&1];
    assert_eq!(inside.len(), 1);
    assert_eq!(inside[0].0, sim_core::time::Pulse::Morning);
    assert_eq!(inside[0].1, A);
    let (answer, answered) = step(&mut s);
    assert_eq!(answer["stops"], json!(["allied", "japanese"]));
    assert_eq!(
        answered.iter().map(|(s, _)| *s).collect::<Vec<_>>(),
        vec![A, J]
    );
    assert_eq!(answered[0].1["kind"], "advance");
    assert_eq!(answered[1].1["kind"], "end_day");
    assert_eq!(answered[1].1["resolved"], true);
}

// ---------------------------------------------------------------------
// 5. A stop on what a side was told.
// ---------------------------------------------------------------------

/// The Allied orders the scenario gives on day 1, as the seat gives them.
fn scripted_allied_orders(name: &str) -> Vec<Value> {
    let scenario = Scenario::load(std::path::Path::new(&path(name))).expect("loads");
    let view = scenario.world.side_view(A);
    let number = |mut ids: Vec<u32>, raw: u32| -> u32 {
        ids.sort();
        u32::try_from(ids.iter().position(|i| *i == raw).expect("an own unit") + 1).unwrap()
    };
    scenario
        .scripted_orders()
        .get(&1)
        .into_iter()
        .flatten()
        .filter(|(side, _)| *side == A)
        .map(|(_, r)| {
            let unit = match &r.kind {
                sim_core::orders::OrderKind::TaskForce { target, .. } => format!(
                    "task_force:{}",
                    number(
                        view.task_forces().map(|f| f.id.raw()).collect(),
                        target.raw()
                    )
                ),
                sim_core::orders::OrderKind::AirGroup { target, .. } => format!(
                    "air_group:{}",
                    number(
                        view.air_groups().map(|g| g.id.raw()).collect(),
                        target.raw()
                    )
                ),
                other => panic!("no such order in the script: {other:?}"),
            };
            json!({"unit": unit, "order": sim_agent::order_to_json(&r.kind)})
        })
        .collect()
}

#[test]
fn s5_a_stop_on_what_a_side_was_told() {
    let calls = vec![
        (
            "issue_orders",
            json!({"orders": scripted_allied_orders(ES)}),
        ),
        ("advance", json!({"pulses": 3, "stop_on": ["contact"]})),
    ];
    let mut p = plain(ES, &[A]);
    let expected = play_plain(&mut p, A, &calls);
    // ES's first contact for the Allied side comes in the afternoon, its
    // last pulse: the plain answer stops there, on what it was told, and,
    // the game being over at that dusk, says so first (`advance` reads the
    // end of the game before what it was told).
    let told = expected[1]["told"].as_array().expect("told");
    assert_eq!(told.len(), 1, "{}", expected[1]);
    assert_eq!(told[0]["thing"], "contact");
    let told_after = expected[1]["pulses_resolved"].as_array().unwrap().len();
    assert_eq!(told_after, 3);
    assert_eq!(expected[1]["stopped"], "game_over");

    let mut w = watched(ES, &[A]);
    assert_eq!(now(&mut w, A, calls[0].0, calls[0].1.clone()), expected[0]);
    hold(&mut w, A, calls[1].0, calls[1].1.clone());
    let mut steps = 0;
    let answer = loop {
        let (_, answered) = step(&mut w);
        steps += 1;
        if let Some((_, answer)) = answered.into_iter().next() {
            break answer;
        }
    };
    assert_eq!(
        steps, told_after,
        "answered after the step of the pulse it was told in"
    );
    assert_eq!(
        serde_json::to_string(&answer).unwrap(),
        serde_json::to_string(&expected[1]).unwrap()
    );
}

// ---------------------------------------------------------------------
// 6. A held seat cannot call.
// ---------------------------------------------------------------------

#[test]
fn s6_a_held_seat_cannot_call() {
    let mut s = watched(ES, &[A, J]);
    now(&mut s, A, "issue_orders", dusk_orders(A));
    hold(&mut s, A, "end_day", json!({}));
    let (hash, queue) = (s.state_hash(), s.queue_of(A));
    for (tool, args) in [
        ("issue_orders", morning_orders(A)),
        ("list_forces", json!({})),
        ("end_day", json!({})),
        ("set_leave", json!({"leave": "launch", "given": true})),
    ] {
        let before = s.transcript().len();
        let a = now(&mut s, A, tool, args.clone());
        assert_eq!(a["kind"], "error");
        let words = a["message"].as_str().unwrap();
        assert!(
            words.contains("is ready and waits for the clock"),
            "{words}"
        );
        assert_eq!(s.state_hash(), hash);
        assert_eq!(s.queue_of(A), queue);
        // The refusal is in the transcript.
        assert_eq!(s.transcript().len(), before + 1);
        let last = s.transcript().last().unwrap();
        assert_eq!((last.seat, last.tool.as_str()), (A, tool));
        assert_eq!(last.arguments, args);
        assert_eq!(last.answer, a);
    }
}

// ---------------------------------------------------------------------
// 7. A call of time refused for its arguments.
// ---------------------------------------------------------------------

#[test]
fn s7_a_call_of_time_refused_for_its_arguments() {
    let mut s = watched(ES, &[A, J]);
    for (tool, args) in [
        ("advance", json!({"pulses": 0})),
        ("advance", json!({"pulses": 1, "stop_on": ["nothing"]})),
        ("run", json!({"days": 0})),
        ("end_day", json!([1])),
    ] {
        let a = now(&mut s, A, tool, args);
        assert_eq!(a["kind"], "error", "{tool}");
        let standing = s.standing();
        assert_eq!(standing["seats"][0]["ready"], false);
        assert_eq!(standing["seats"][0]["waited_for"], true);
    }
    hold(&mut s, J, "end_day", json!({}));
    let words = refused_step(&mut s);
    assert!(words.contains("allied seat"), "{words}");
}

// ---------------------------------------------------------------------
// 8. `run` beside a stop inside a day.
// ---------------------------------------------------------------------

fn s78_dusk_orders() -> Value {
    json!({"orders": [
        order("task_force:1", json!({"kind": "set_route", "waypoints": [[8, 3]]})),
        order("task_force:1", json!({"kind": "set_speed", "speed": "cruise"})),
    ]})
}

#[test]
fn s8_run_beside_a_stop_inside_a_day() {
    let mut p = plain(S78, &[A]);
    let expected = play_plain(
        &mut p,
        A,
        &[
            ("issue_orders", s78_dusk_orders()),
            ("run", json!({"days": 2})),
        ],
    );
    assert_eq!(expected[1]["kind"], "run");

    let mut s = watched(S78, &[A, J]);
    assert_eq!(
        now(&mut s, A, "issue_orders", s78_dusk_orders()),
        expected[0]
    );
    hold(&mut s, A, "run", json!({"days": 2}));
    hold(&mut s, J, "advance", json!({"pulses": 1}));
    let (_, answered) = step(&mut s);
    assert_eq!(
        answered.iter().map(|(s, _)| *s).collect::<Vec<_>>(),
        vec![J]
    );
    assert_eq!(s.open_day().map(|(d, _)| d), Some(1));
    hold(&mut s, J, "end_day", json!({}));
    step(&mut s);
    let (_, answered) = step(&mut s);
    // The first dusk: the Japanese end_day is answered, the run is not.
    assert_eq!(
        answered.iter().map(|(s, _)| *s).collect::<Vec<_>>(),
        vec![J]
    );
    assert_eq!(s.day(), 1);
    assert!(refused_step(&mut s).contains("japanese seat"));
    hold(&mut s, J, "end_day", json!({}));
    let mut run_answer = None;
    for _ in 0..3 {
        let (_, answered) = step(&mut s);
        for (side, a) in answered {
            if side == A {
                run_answer = Some(a);
            }
        }
    }
    assert_eq!(s.day(), 2);
    let run_answer = run_answer.expect("the run is answered at the second dusk");
    assert_eq!(
        serde_json::to_string(&run_answer).unwrap(),
        serde_json::to_string(&expected[1]).unwrap()
    );
    // The Allied orders went in at the first dusk and not again.
    let allied_on = |day: u32| {
        s.handed_in()
            .get(&day)
            .into_iter()
            .flatten()
            .filter(|(side, _)| *side == A)
            .count()
    };
    assert_eq!((allied_on(1), allied_on(2)), (2, 0));
    assert_eq!(s.day_hashes(), &p.day_hashes()[..2]);
}

// ---------------------------------------------------------------------
// 9. A held `advance` stopped by an interrupt.
// ---------------------------------------------------------------------

/// A command that makes an interrupt of the Allied side fire on S78's day
/// 1 (found by the build, see the record's "As built").
fn s78_interrupt_command() -> (&'static str, Value) {
    ("set_intent", json!({"intent_kind": "cover", "base": 1}))
}

#[test]
fn s9_a_held_advance_stopped_by_an_interrupt() {
    let (tool, args) = s78_interrupt_command();
    // In the plain game the same advance stops on the interrupt at the
    // first dusk.
    let mut p = plain(S78, &[A]);
    now(&mut p, A, tool, args.clone());
    let plain_answer = now(&mut p, A, "advance", json!({"pulses": 9}));
    assert_eq!(plain_answer["stopped"], "interrupt", "{plain_answer}");

    let mut s = watched(S78, &[A, J]);
    let a = now(&mut s, A, tool, args);
    assert_ne!(a["kind"], "error", "{a}");
    hold(&mut s, A, "advance", json!({"pulses": 9}));
    hold(&mut s, J, "advance", json!({"pulses": 9}));
    let mut got = None;
    for _ in 0..3 {
        let (_, answered) = step(&mut s);
        assert!(answered.iter().all(|(side, _)| *side == A));
        if let Some((_, a)) = answered.into_iter().next() {
            got = Some(a);
        }
    }
    let got = got.expect("answered at the dusk");
    assert_eq!((s.day(), s.open_day()), (1, None));
    assert_eq!(got, plain_answer);
    // The Japanese call is still held.
    let standing = s.standing();
    assert_eq!(standing["seats"][1]["ready"], true);
    assert_eq!(standing["seats"][1]["until"]["tool"], "advance");
}

// ---------------------------------------------------------------------
// 10. `run` and `advance` offered.
// ---------------------------------------------------------------------

#[test]
fn s10_run_and_advance_are_offered_to_both_seats_in_a_watched_game() {
    let w = watched(ES, &[A, J]);
    for side in [A, J] {
        let tools = w.tools(side);
        assert!(
            tools.contains(&"run") && tools.contains(&"advance"),
            "{tools:?}"
        );
    }
    let p = plain(ES, &[A, J]);
    for side in [A, J] {
        let tools = p.tools(side);
        assert!(
            !tools.contains(&"run") && !tools.contains(&"advance"),
            "{tools:?}"
        );
    }
    // A seat at a command has no `run`, watched or not: scenario 62, the
    // nearest existing scenario with command levels on (none of the three
    // has them).
    let at = BTreeMap::from([(A, Post::At(CommandRef::TaskForce(TaskForceId::new(1))))]);
    let (seat, _) = sim_cli::game::open(
        &path("62_reports_between_commands.toml"),
        None,
        seats(&[A]),
        &at,
    )
    .expect("the seat is placed");
    assert!(!seat.tools(A).contains(&"run"));
    let seat = seat.watched();
    assert!(!seat.tools(A).contains(&"run"));
}

// ---------------------------------------------------------------------
// 11. The watcher reads as the seat reads.
// ---------------------------------------------------------------------

/// Every read of the watcher's, with arguments, that the side's seat has.
fn reads(s: &Session, side: Side) -> Vec<(&'static str, Value)> {
    let all: Vec<(&'static str, Value)> = vec![
        ("get_briefing", json!({})),
        ("get_briefing", json!({"text": true})),
        ("list_forces", json!({})),
        ("list_bases", json!({})),
        ("list_contacts", json!({})),
        ("get_entity", json!({"handle": "task_force:1"})),
        ("get_entity", json!({"handle": "ship:1"})),
        ("get_entity", json!({"handle": "air_group:1"})),
        ("get_entity", json!({"handle": "base:1"})),
        ("get_entity", json!({"handle": "contact:1"})),
        (
            "measure",
            json!({"question": "air_distance", "from": "task_force:1", "to": {"q": 2, "r": -4}}),
        ),
        (
            "measure",
            json!({"question": "choices", "unit": "air_group:1"}),
        ),
        (
            "measure",
            json!({"question": "passage", "task_force": "task_force:1", "to": {"q": 2, "r": -4}}),
        ),
        ("explain", json!({"event": 1})),
        ("explain", json!({"event": 3})),
        ("events", json!({})),
        ("events", json!({"day": 1, "pulse": "night"})),
        ("command_state", json!({})),
        ("plans", json!({})),
        ("staff_orders", json!({})),
        ("propose", json!({"intent": 1})),
        ("order", json!({"intent": 1})),
        ("why", json!({"number": 1})),
        ("report", json!({})),
        ("trials", json!({})),
    ];
    let tools = s.watcher_tools(side);
    all.into_iter().filter(|(t, _)| tools.contains(t)).collect()
}

/// Each read of the watcher's is the seat's own, byte for byte.
fn assert_reads_as_the_seat(s: &mut Session, when: &str) {
    for side in [A, J] {
        let rs = reads(s, side);
        assert!(rs.len() >= 20, "{when}: {} reads", rs.len());
        for (tool, args) in rs {
            let watcher = doc(&s.watch(side, tool, &args));
            let seat = now(s, side, tool, args.clone());
            assert_eq!(
                serde_json::to_string(&watcher).unwrap(),
                serde_json::to_string(&seat).unwrap(),
                "{when}, {side:?}, {tool} {args}"
            );
        }
    }
}

#[test]
fn s11_the_watcher_reads_as_the_seat_reads() {
    let mut s = watched(ES, &[A, J]);
    for side in [A, J] {
        let expected: Vec<&str> = WATCHER_TOOLS
            .iter()
            .copied()
            .filter(|t| s.tools(side).contains(t))
            .collect();
        assert_eq!(s.watcher_tools(side), expected);
    }
    // Each side with an intent, so that the staff's tools have something
    // to say.
    now(
        &mut s,
        A,
        "set_intent",
        json!({"intent_kind": "cover", "base": 1}),
    );
    assert_reads_as_the_seat(&mut s, "at dusk");
    hold(&mut s, A, "advance", json!({"pulses": 1}));
    hold(&mut s, J, "advance", json!({"pulses": 1}));
    step(&mut s);
    assert_eq!(s.open_day().map(|(d, _)| d), Some(1));
    assert_reads_as_the_seat(&mut s, "at 0600");
    // The briefing is refused him at 0600 as it is the seat.
    for side in [A, J] {
        let watcher = doc(&s.watch(side, "get_briefing", &json!({})));
        assert_eq!(watcher["kind"], "error");
        assert!(watcher["message"]
            .as_str()
            .unwrap()
            .contains("made at dusk"));
    }
}

// ---------------------------------------------------------------------
// 12. The watcher can change nothing.
// ---------------------------------------------------------------------

#[test]
fn s12_the_watcher_can_change_nothing() {
    let mut s = watched(ES, &[A, J]);
    let not_his = [
        "issue_orders",
        "end_day",
        "run",
        "advance",
        "set_intent",
        "withdraw_intent",
        "set_limit",
        "set_leave",
        "set_pursuit",
        "answer_pursuit",
        "hold",
        "release",
        "preview",
        "approve",
        "cancel_plan",
        "set_interrupt",
        "acknowledge",
        "trial",
        "queued",
        "take_back",
    ];
    let mut every: Vec<&str> = TOOLS.to_vec();
    every.extend(COMMAND_TOOLS);
    every.extend(TRIAL_TOOLS);
    every.push("advance");
    for tool in &every {
        assert!(
            WATCHER_TOOLS.contains(tool) || not_his.contains(tool),
            "{tool}"
        );
    }
    let hash = s.state_hash();
    for side in [A, J] {
        for tool in not_his {
            let error = s
                .watch(side, tool, &json!({}))
                .expect_err("not a tool of the watcher's");
            assert!(
                error
                    .message
                    .contains(&format!("\"{tool}\" is not a tool of the watcher's")),
                "{}",
                error.message
            );
        }
    }
    assert_eq!(s.state_hash(), hash);
    assert!(s.transcript().is_empty());

    // Two games, the same seats' calls; in one the watcher reads all the
    // while.
    let preview = json!({"command": "set_intent", "intent_kind": "cover", "base": 1});
    let game = |reading: bool| -> (Vec<u64>, u64, Vec<Value>, Vec<Value>) {
        let mut s = watched(ES, &[A, J]);
        let read_all = |s: &mut Session| {
            if reading {
                for side in [A, J] {
                    for (tool, args) in reads(s, side) {
                        let _ = s.watch(side, tool, &args);
                    }
                    let _ = s.standing();
                    let _ = s.seat_calls(side);
                }
            }
        };
        let mut shown = Vec::new();
        read_all(&mut s);
        shown.push(now(&mut s, A, "preview", preview.clone()));
        read_all(&mut s);
        shown.push(now(&mut s, A, "preview", preview.clone()));
        hold(&mut s, A, "advance", json!({"pulses": 1}));
        read_all(&mut s);
        hold(&mut s, J, "advance", json!({"pulses": 1}));
        step(&mut s);
        read_all(&mut s);
        shown.push(now(&mut s, A, "preview", preview.clone()));
        hold(&mut s, A, "end_day", json!({}));
        hold(&mut s, J, "end_day", json!({}));
        read_all(&mut s);
        step(&mut s);
        read_all(&mut s);
        step(&mut s);
        read_all(&mut s);
        (
            s.day_hashes().to_vec(),
            s.state_hash(),
            transcript(&s),
            shown,
        )
    };
    let (quiet, read) = (game(false), game(true));
    assert_eq!(quiet.0, read.0);
    assert_eq!(quiet.1, read.1);
    assert_eq!(quiet.2, read.2);
    assert_eq!(quiet.3, read.3);
    // The seat's next preview is numbered as it would have been.
    assert_eq!(read.3[1]["preview"], 2, "{}", read.3[1]);
    assert_eq!(read.3[2]["preview"], 3, "{}", read.3[2]);
}

// ---------------------------------------------------------------------
// 13. A seat's answers do not read the other seat's queue.
// ---------------------------------------------------------------------

/// Every read a seat has at dusk, preview and trial among them.
fn every_read(s: &Session, side: Side) -> Vec<(&'static str, Value)> {
    let mut all = reads(s, side);
    all.push((
        "preview",
        json!({"command": "set_intent", "intent_kind": "cover", "base": 1}),
    ));
    all.push((
        "preview",
        json!({"command": "set_intent", "intent_kind": "deny", "hex": [8, -11], "distance": 2}),
    ));
    if s.tools(side).contains(&"trial") {
        all.push(("trial", json!({"intent": 1})));
    }
    all
}

/// The other seat's queue: orders and a command, or nothing.
fn queue_for(s: &mut Session, side: Side, full: bool) {
    let own = match side {
        Side::Allied => json!({"intent_kind": "cover", "base": 1}),
        Side::Japanese => json!({"intent_kind": "deny", "hex": [8, -11], "distance": 2}),
    };
    // Each seat's own intent, in both games.
    let a = now(s, side, "set_intent", own);
    assert_ne!(a["kind"], "error", "{a}");
    if full {
        let a = now(s, side, "issue_orders", dusk_orders(side));
        assert_eq!(a["queued_in_call"], 1, "{a}");
        let a = now(
            s,
            side,
            "issue_orders",
            json!({"orders": [
                order("task_force:2", json!({"kind": "set_speed", "speed": "full"})),
                order("air_group:1", json!({"kind": "set_air_mission", "mission": "search"})),
            ]}),
        );
        assert_eq!(a["queued_in_call"], 2, "{a}");
        let a = now(
            s,
            side,
            "set_leave",
            json!({"leave": "launch", "given": true}),
        );
        assert_ne!(a["kind"], "error", "{a}");
    }
}

#[test]
fn s13_a_seats_answers_do_not_read_the_other_seats_queue() {
    let mut differ = Vec::new();
    for (reader, other) in [(A, J), (J, A)] {
        let answers = |full: bool| -> Vec<(String, String)> {
            let mut s = watched(ES, &[A, J]);
            queue_for(&mut s, reader, false);
            queue_for(&mut s, other, full);
            every_read(&s, reader)
                .into_iter()
                .map(|(tool, args)| {
                    let a = now(&mut s, reader, tool, args.clone());
                    (format!("{tool} {args}"), serde_json::to_string(&a).unwrap())
                })
                .collect()
        };
        let (empty, full) = (answers(false), answers(true));
        assert_eq!(empty.len(), full.len());
        for ((what, e), (_, f)) in empty.iter().zip(&full) {
            if e != f {
                differ.push(format!("{reader:?} {what}"));
            }
        }
    }
    assert!(
        differ.is_empty(),
        "reads that differ with the other seat's queue: {differ:?}"
    );
}

// ---------------------------------------------------------------------
// 14. Where the game stands.
// ---------------------------------------------------------------------

#[test]
fn s14_he_is_told_where_the_game_stands() {
    let mut s = watched(ES, &[A, J]);
    let st = s.standing();
    assert_eq!(st["kind"], "standing");
    assert_eq!(st["clock"]["day"], 1);
    assert_eq!(st["clock"]["at"], "dusk");
    assert_eq!(st["game_over"], false);
    assert_eq!(st["waiting_for"], json!(["allied", "japanese"]));
    for seat in st["seats"].as_array().unwrap() {
        assert_eq!(seat["holder"], "player");
        assert_eq!(seat["ready"], false);
        assert_eq!(seat["waited_for"], true);
        assert_eq!(seat["until"], Value::Null);
    }
    hold(
        &mut s,
        A,
        "advance",
        json!({"pulses": 2, "stop_on": ["contact"]}),
    );
    let st = s.standing();
    assert_eq!(st["waiting_for"], json!(["japanese"]));
    assert_eq!(st["seats"][0]["ready"], true);
    assert_eq!(st["seats"][0]["waited_for"], false);
    assert_eq!(st["seats"][0]["until"]["tool"], "advance");
    assert_eq!(st["seats"][0]["until"]["pulses"], 2);
    assert!(st["seats"][0]["until"]["words"]
        .as_str()
        .unwrap()
        .contains("2 more pulses"));
    hold(&mut s, J, "end_day", json!({}));
    let st = s.standing();
    assert_eq!(st["seats"][1]["until"]["tool"], "end_day");
    assert!(st["seats"][1]["until"]["words"]
        .as_str()
        .unwrap()
        .contains("end of day 1"));
    assert_eq!(st["waiting_for"], json!([]));
    step(&mut s);
    assert_eq!(s.standing()["clock"]["at"], "0600");
    while !s.is_over() {
        if s.standing()["seats"][0]["ready"] == false {
            hold(&mut s, A, "end_day", json!({}));
        }
        step(&mut s);
    }
    let st = s.standing();
    assert_eq!(st["game_over"], true);
    assert_eq!(st["waiting_for"], json!([]));
    assert_eq!(st["seats"][0]["waited_for"], false);

    // A scripted seat is always ready.
    let s = watched(ES, &[A]);
    let st = s.standing();
    assert_eq!(st["seats"][1]["holder"], "script");
    assert_eq!(st["seats"][1]["ready"], true);
    assert_eq!(st["seats"][1]["waited_for"], false);
    assert_eq!(st["waiting_for"], json!(["allied"]));
}

// ---------------------------------------------------------------------
// 15. What a seat did.
// ---------------------------------------------------------------------

#[test]
fn s15_he_is_shown_what_a_seat_did() {
    let mut s = watched(ES, &[A, J]);
    let mut with_reason = dusk_orders(A);
    with_reason["reason"] = json!("Battle readiness before the carriers close.");
    now(&mut s, A, "list_forces", json!({}));
    now(&mut s, A, "issue_orders", with_reason.clone());
    now(&mut s, A, "get_truth", json!({}));
    now(&mut s, J, "list_bases", json!({}));
    hold(
        &mut s,
        A,
        "advance",
        json!({"pulses": 1, "reason": "To 0600."}),
    );
    let did = s.seat_calls(A);
    assert_eq!(did["kind"], "seat_calls");
    assert_eq!(did["seat"], "allied");
    let calls = did["calls"].as_array().unwrap();
    assert_eq!(calls.len(), 3, "the other seat's call is not among them");
    assert_eq!(calls[0]["tool"], "list_forces");
    assert_eq!(calls[0]["refused"], false);
    assert_eq!(calls[0]["reason"], Value::Null);
    assert_eq!(calls[1]["tool"], "issue_orders");
    assert_eq!(calls[1]["arguments"], with_reason);
    assert_eq!(
        calls[1]["reason"],
        "Battle readiness before the carriers close."
    );
    assert_eq!(calls[2]["tool"], "get_truth");
    assert_eq!(calls[2]["refused"], true);
    assert_eq!(did["held"]["tool"], "advance");
    assert_eq!(did["held"]["reason"], "To 0600.");
    hold(&mut s, J, "end_day", json!({}));
    step(&mut s);
    // Its stop came: nothing since it.
    let did = s.seat_calls(A);
    assert_eq!(did["calls"], json!([]));
    assert_eq!(did["held"], Value::Null);
    now(&mut s, A, "events", json!({}));
    let did = s.seat_calls(A);
    assert_eq!(did["calls"].as_array().unwrap().len(), 1);
    assert_eq!(did["calls"][0]["tool"], "events");
    // The Japanese seat has not stopped: its calls since the start.
    let did = s.seat_calls(J);
    assert_eq!(did["calls"].as_array().unwrap().len(), 1);
    assert_eq!(did["held"]["tool"], "end_day");
}

// ---------------------------------------------------------------------
// 16. The step's answer.
// ---------------------------------------------------------------------

#[test]
fn s16_the_steps_answer() {
    let mut s = watched(ES, &[A, J]);
    hold(&mut s, A, "advance", json!({"pulses": 1}));
    hold(&mut s, J, "end_day", json!({}));
    let (answer, _) = step(&mut s);
    assert_eq!(answer["kind"], "step");
    assert_eq!(answer["resolved"], json!({"day": 1, "pulses": ["night"]}));
    assert_eq!(answer["clock"]["at"], "0600");
    assert_eq!(answer["clock"]["day"], 1);
    assert_eq!(answer["stops"], json!(["allied"]));
    assert_eq!(answer["game_over"], false);
    assert!(answer["summary"]
        .as_str()
        .unwrap()
        .contains("night pulse of day 1"));
    hold(&mut s, A, "end_day", json!({}));
    let (answer, _) = step(&mut s);
    assert_eq!(answer["resolved"], json!({"day": 1, "pulses": ["morning"]}));
    assert_eq!(answer["clock"]["at"], "1200");
    assert_eq!(answer["stops"], json!([]));
    let (answer, _) = step(&mut s);
    assert_eq!(
        answer["resolved"],
        json!({"day": 1, "pulses": ["afternoon"]})
    );
    assert_eq!(answer["clock"]["at"], "dusk");
    assert_eq!(answer["stops"], json!(["allied", "japanese"]));
    assert_eq!(answer["game_over"], true);
}

// ---------------------------------------------------------------------
// 17. At the game's end, and on an error.
// ---------------------------------------------------------------------

#[test]
fn s17_at_the_games_end_every_held_call_is_answered() {
    let mut s = watched(ES, &[A, J]);
    hold(&mut s, A, "run", json!({"days": 5}));
    hold(&mut s, J, "advance", json!({"pulses": 9}));
    let mut answered = Vec::new();
    while !s.is_over() {
        answered.extend(step(&mut s).1);
    }
    assert_eq!(answered.len(), 2);
    assert_eq!(answered[0].1["kind"], "run");
    assert_eq!(answered[0].1["stopped"], "game_over");
    assert_eq!(answered[1].1["kind"], "advance");
    assert_eq!(answered[1].1["stopped"], "game_over");
    for (_, a) in &answered {
        assert!(
            a["result"].is_object(),
            "the result where the tool gives one: {a}"
        );
    }
    assert!(refused_step(&mut s).contains("the game is over"));
    // A call of time after the end is answered at once.
    assert_eq!(now(&mut s, A, "end_day", json!({}))["resolved"], false);
    assert_eq!(
        now(&mut s, A, "advance", json!({"pulses": 1}))["kind"],
        "error"
    );
}

#[test]
fn s17_with_the_engine_stopped_every_held_call_is_answered_with_its_words() {
    // ES with an arrival put in the world's start that overfills the
    // Enterprise inside day 1, as `crates/sim-agent/tests/leaks.rs` makes a
    // day fail: a means of the test.
    let mut scenario = Scenario::load(std::path::Path::new(&path(ES))).expect("loads");
    let world = &scenario.world;
    let mut truth = world.umpire().clone();
    truth.arrivals.push(Arrival {
        day: 1,
        air_group: AirGroupId::new(1),
        aircraft: 500,
        pilots: 0,
        waiting: false,
    });
    scenario.world = World::new(
        world.seed(),
        world.ref_data().clone(),
        world.map().clone(),
        truth,
    )
    .expect("valid at the start");
    let (s, _) = sim_cli::game::open_loaded(&path(ES), scenario, seats(&[A, J]), &BTreeMap::new())
        .expect("opens");
    let mut s = s.watched();
    hold(&mut s, A, "run", json!({"days": 1}));
    hold(&mut s, J, "advance", json!({"pulses": 3}));
    let stopped = loop {
        let Stepped { answer, answered } = s.step();
        match answer {
            Ok(_) => assert!(answered.is_empty()),
            Err(error) => break (error, answered),
        }
    };
    let (error, answered) = stopped;
    assert!(
        error.message.contains("the game has stopped inside day 1"),
        "{}",
        error.message
    );
    assert_eq!(answered.len(), 2);
    for (side, outcome) in &answered {
        assert_eq!(outcome, &Err(error.clone()), "{side:?}");
    }
    assert_eq!(s.stopped_inside_day(), Some(1));
    assert_eq!(refused_step(&mut s), error.message);
    // The held calls are in the transcript, answered with the stop.
    let last: Vec<&str> = s
        .transcript()
        .iter()
        .rev()
        .take(2)
        .map(|e| e.tool.as_str())
        .collect();
    assert_eq!(last, vec!["advance", "run"]);
}

// ---------------------------------------------------------------------
// 18. Saved and replayed.
// ---------------------------------------------------------------------

fn temp(label: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("flagplot-101-{}-{label}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a temp dir");
    dir
}

#[test]
fn s18_saved_and_replayed() {
    let (s, kept, _) = es_two_orders(A);
    let file = temp("saved").join("game.json");
    let file = file.to_str().unwrap();
    kept.finish(&s, Some(file), None).expect("saved");
    let saved = sim_cli::save::read(file).expect("read back");
    // Replayed with no watcher, as `replay` replays it.
    let mut scenario = Scenario::load(std::path::Path::new(&saved.scenario_file))
        .expect("loads")
        .with_seed(saved.seed)
        .expect("seeded");
    scenario.replace_script(saved.script.clone());
    scenario.replace_script_inside_day(saved.script_inside_day.clone());
    scenario.replace_commands(saved.commands.clone());
    assert!(!saved.script_inside_day.is_empty());
    for (day, hash) in &saved.hashes {
        for (side, decisions) in saved.staff.get(day).into_iter().flatten() {
            scenario.world.submit_staff(*side, decisions.clone());
        }
        let summary = scenario
            .next_day_watched(sim_core::Staffs::none(), &mut |_, _| {})
            .expect("the day resolves")
            .expect("the day exists");
        assert_eq!(&format!("{:016x}", summary.state_hash), hash);
    }
    assert_eq!(saved.hashes.len(), 1);
    assert_eq!(scenario.world.day_hashes(), s.day_hashes());

    // A watched game left at 0600 is saved as stopped inside a day.
    let (s, kept) = opened(ES, &[A, J]);
    let mut s = s.watched();
    hold(&mut s, A, "advance", json!({"pulses": 1}));
    hold(&mut s, J, "advance", json!({"pulses": 1}));
    step(&mut s);
    let file = temp("left").join("game.json");
    kept.finish(&s, Some(file.to_str().unwrap()), None)
        .expect("saved");
    let value: Value = serde_json::from_str(&std::fs::read_to_string(&file).unwrap()).unwrap();
    assert_eq!(value["stopped_inside_day"], 1);
    // A game not watched, left at 0600, is saved as before.
    let (mut s, kept) = opened(ES, &[A]);
    s.call(A, "advance", &json!({"pulses": 1}))
        .expect("a pulse");
    let file = temp("plain-left").join("game.json");
    kept.finish(&s, Some(file.to_str().unwrap()), None)
        .expect("saved");
    let value: Value = serde_json::from_str(&std::fs::read_to_string(&file).unwrap()).unwrap();
    assert_eq!(value.get("stopped_inside_day"), None);
}

// ---------------------------------------------------------------------
// 19. A day at a step where a scenario has no orders inside a day.
// ---------------------------------------------------------------------

#[test]
fn s19_a_day_at_a_step_without_orders_inside_a_day() {
    let calls: Vec<(&str, Value)> = vec![
        ("issue_orders", s78_dusk_orders()),
        ("end_day", json!({})),
        ("end_day", json!({})),
        ("end_day", json!({})),
    ];
    let mut p = plain(T, &[A]);
    let expected = play_plain(&mut p, A, &calls);
    let mut w = watched(T, &[A]);
    assert!(!w.tools(A).contains(&"advance"));
    assert!(w.tools(A).contains(&"run"));
    assert_eq!(now(&mut w, A, calls[0].0, calls[0].1.clone()), expected[0]);
    for (i, (tool, args)) in calls.iter().enumerate().skip(1) {
        hold(&mut w, A, tool, args.clone());
        let (answer, answered) = step(&mut w);
        assert_eq!(
            answer["resolved"],
            json!({"day": i, "pulses": ["night", "morning", "afternoon"]})
        );
        assert_eq!(answer["clock"]["at"], "dusk");
        assert_eq!(answered, vec![(A, expected[i].clone())]);
    }
    assert_eq!(w.day_hashes().len(), 3);
    assert_eq!(w.day_hashes(), p.day_hashes());
    assert!(refused_step(&mut w).contains("over"));
}

// ---------------------------------------------------------------------
// 21. A game not watched is as it was.
// ---------------------------------------------------------------------

#[test]
fn s21_a_game_not_watched_lists_its_tools_as_before() {
    // The tools as they were listed before decision 0182: `run` only where
    // one seat is a player's, `advance` only there and with orders inside a
    // day, `trial` and `trials` where the game has a trial.
    for (name, inside) in [(S78, true), (ES, true), (T, false)] {
        for players in [vec![A], vec![J], vec![A, J]] {
            let s = plain(name, &players);
            assert!(!s.is_watched());
            for side in &players {
                let mut before: Vec<&str> = TOOLS.to_vec();
                before.extend(
                    COMMAND_TOOLS
                        .iter()
                        .filter(|t| **t != "run" || players.len() == 1),
                );
                if s.has_trial() {
                    before.extend(TRIAL_TOOLS);
                }
                if inside && players.len() == 1 {
                    before.push("advance");
                }
                assert_eq!(s.tools(*side), before, "{name} {players:?}");
            }
        }
    }
    // A game not watched has no watcher and no step.
    let mut s = plain(ES, &[A]);
    assert!(s.step().answer.unwrap_err().message.contains("not watched"));
    assert!(s.watch(A, "list_forces", &json!({})).is_err());
    assert_eq!(s.left_inside_day(), None);
}

/// The recorded games, replayed through a session opened as `mcp` opens
/// one, answer every call as their transcripts kept it, and every tool a
/// recorded game called is one its seat is listed.
#[test]
fn s21_the_recorded_games_replay_as_they_did() {
    let root =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../crates/sim-cli/tests/recorded_games");
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(&root)
        .expect("the recorded games")
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.join("game.jsonl").exists() && p.join("transcript.jsonl").exists())
        .collect();
    dirs.sort();
    assert!(!dirs.is_empty());
    for dir in dirs {
        let game = std::fs::read_to_string(dir.join("game.jsonl")).unwrap();
        let header: Value = serde_json::from_str(game.lines().next().unwrap()).unwrap();
        let seat = sim_agent::parse_seat(header["seat"].as_str().unwrap()).unwrap();
        let scenario = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join(header["scenario"].as_str().unwrap());
        let (mut s, _) = sim_cli::game::open(
            scenario.to_str().unwrap(),
            header["seed"].as_u64(),
            seats(&[seat]),
            &BTreeMap::new(),
        )
        .expect("opens");
        let tools = s.tools(seat);
        for line in std::fs::read_to_string(dir.join("transcript.jsonl"))
            .unwrap()
            .lines()
        {
            let entry: Value = serde_json::from_str(line).unwrap();
            let tool = entry["tool"].as_str().unwrap();
            let answer = doc(&s.call(seat, tool, &entry["arguments"]));
            assert_eq!(
                answer,
                entry["answer"],
                "{} call {}",
                dir.display(),
                entry["seq"]
            );
            if answer["kind"] != "error" {
                assert!(tools.contains(&tool), "{tool}");
            }
        }
    }
}
