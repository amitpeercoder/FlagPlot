//! `sim-cli serve`: one game held open for play (decisions 13 to 15 of 0050).
//!
//! JSON lines on standard input and output, one request a line and one answer
//! a line, flushed. It opens no port and reads no clock. A request is
//! `{"id": .., "seat": "allied", "tool": "get_briefing", "arguments": {..}}`
//! and the answer is `{"id": .., "result": ..}` or `{"id": .., "error": ..}`
//! with `id` echoed as given. A request that is not a call (not JSON, not an
//! object, a seat that does not exist or is held by the script, no tool
//! named) is answered with an error and changes nothing, the transcript
//! included.
//!
//! # A watched game
//!
//! With `--watched` the game has a watcher, who holds the clock (decision
//! 0182). A seat's call is as above, but a call of time (`end_day`, `run`,
//! `advance`) whose arguments stand is held: no line answers it until a step
//! brings its stop, and then its answer line, with its own `id`, follows the
//! step's line (two due on one step, the Allied first). Beside a seat's call
//! there are these requests:
//!
//! * `{"id": .., "watcher": "step"}`: the watcher's step. Answered with the
//!   step's document (`kind` `step`: `resolved` with the `day` and its
//!   `pulses`, the `clock`, the `stops` that came, `game_over`, `summary`),
//!   or an error when a played seat is not ready, naming it.
//! * `{"id": .., "watcher": "read", "side": "allied", "tool": .., "arguments":
//!   {..}}`: a read of the watcher's, answered as that side's seat would be.
//! * `{"id": .., "watcher": "standing"}`: where the game stands (`kind`
//!   `standing`).
//! * `{"id": .., "watcher": "seat_calls", "side": "allied"}`: what a seat did
//!   since its last stop (`kind` `seat_calls`).
//! * `{"id": .., "seat": "allied", "request": "tools"}` and `"request":
//!   "instructions"`: the seat's tool definitions (`{"tools": [..]}`, as
//!   `tools/list` of `mcp` gives them) and the words an agent is given on
//!   taking the seat (`{"instructions": ".."}`, as `mcp` gives them).
//!
//! Without `--watched` none of these is a request, and each is answered as
//! any request that is not a call is.

use std::collections::BTreeMap;
use std::io::{BufRead, Write};

use serde_json::{json, Value};
use sim_agent::{parse_seat, CallError, Called, Post, SeatKind, Session};
use sim_core::state::Side;

pub use sim_cli::game::open;

/// What `serve` was asked to do.
struct Options<'a> {
    scenario: &'a str,
    seed: Option<u64>,
    seats: BTreeMap<Side, SeatKind>,
    /// Where each player seat sits; the area command where nothing is said.
    posts: BTreeMap<Side, Post>,
    save: Option<&'a str>,
    transcript: Option<&'a str>,
    /// The game is watched (decision 0182).
    watched: bool,
}

fn parse<'a>(args: &'a [String], usage: &str) -> Result<Options<'a>, String> {
    let mut scenario = None;
    let mut seed = None;
    let mut seats: BTreeMap<Side, SeatKind> = BTreeMap::new();
    let mut posts: BTreeMap<Side, Post> = BTreeMap::new();
    let mut save = None;
    let mut transcript = None;
    let mut watched = false;
    let mut words = args.iter().map(String::as_str);
    while let Some(word) = words.next() {
        let mut value = |what: &str| {
            words
                .next()
                .ok_or_else(|| format!("{what} needs a value\n\n{usage}"))
        };
        match word {
            "--seed" => {
                let v = value("--seed")?;
                seed = Some(
                    v.parse::<u64>()
                        .map_err(|_| format!("--seed \"{v}\" is not a whole number"))?,
                );
            }
            "--seat" => {
                let v = value("--seat")?;
                let (side, holder) = v
                    .split_once('=')
                    .ok_or_else(|| format!("--seat \"{v}\" is not side=player or side=script"))?;
                let side = parse_seat(side).ok_or_else(|| {
                    format!("--seat names no side \"{side}\": allied or japanese")
                })?;
                let holder = SeatKind::parse(holder).ok_or_else(|| {
                    format!("--seat holder \"{holder}\" is not \"player\" or \"script\"")
                })?;
                if seats.insert(side, holder).is_some() {
                    return Err(format!("--seat names the {} seat twice", side.name()));
                }
            }
            "--at" => {
                let v = value("--at")?;
                let (side, post) = v.split_once('=').ok_or_else(|| {
                    format!("--at \"{v}\" is not side=seat, for example allied=task_force:2")
                })?;
                let side = parse_seat(side)
                    .ok_or_else(|| format!("--at names no side \"{side}\": allied or japanese"))?;
                if posts.insert(side, Post::parse(post)?).is_some() {
                    return Err(format!("--at names the {} seat twice", side.name()));
                }
            }
            "--save" => save = Some(value("--save")?),
            "--transcript" => transcript = Some(value("--transcript")?),
            "--watched" => watched = true,
            flag if flag.starts_with('-') => {
                return Err(format!("unknown option \"{flag}\"\n\n{usage}"))
            }
            path if scenario.is_none() => scenario = Some(path),
            extra => return Err(format!("unexpected argument \"{extra}\"\n\n{usage}")),
        }
    }
    let scenario = scenario.ok_or_else(|| format!("no scenario given\n\n{usage}"))?;
    for side in Side::ALL {
        seats.entry(side).or_insert(SeatKind::Script);
    }
    // With no player there is nothing to serve: it is an error, not a game
    // that runs itself (`run` does that).
    if seats.values().all(|k| *k == SeatKind::Script) {
        return Err(format!(
            "serve needs at least one seat held by a player: --seat allied=player or --seat japanese=player\n\n{usage}"
        ));
    }
    if let Some(side) = posts.keys().find(|side| seats[side] != SeatKind::Player) {
        return Err(format!(
            "--at names the {} seat, which is held by the script: --seat {}=player first",
            side.name(),
            side.name()
        ));
    }
    Ok(Options {
        scenario,
        seed,
        seats,
        posts,
        save,
        transcript,
        watched,
    })
}

pub fn command(args: &[String], usage: &str) -> Result<(), String> {
    let options = parse(args, usage)?;
    let (mut session, kept) = open(
        options.scenario,
        options.seed,
        options.seats.clone(),
        &options.posts,
    )?;
    if options.watched {
        session = session.watched();
    }

    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    // The `id` of each seat's held call, answered when a step brings its stop.
    let mut held: BTreeMap<Side, Value> = BTreeMap::new();
    for line in stdin.lock().lines() {
        let line = line.map_err(|e| format!("standard input: {e}"))?;
        let answers = if options.watched {
            watched_lines(&mut session, &mut held, &line)
        } else {
            vec![answer_line(&mut session, &line)]
        };
        for answer in answers {
            writeln!(out, "{answer}")
                .and_then(|()| out.flush())
                .map_err(|e| format!("standard output: {e}"))?;
        }
    }
    kept.finish(&session, options.save, options.transcript)
}

fn result_line(id: Value, outcome: Result<Value, CallError>) -> String {
    match outcome {
        Ok(result) => serde_json::to_string(&json!({ "id": id, "result": result }))
            .expect("an answer serialises"),
        Err(error) => error_line(id, &error),
    }
}

/// The side a watcher's or a seat's request names under `key`.
fn side_of(object: &serde_json::Map<String, Value>, key: &str) -> Result<Side, CallError> {
    object
        .get(key)
        .and_then(Value::as_str)
        .and_then(parse_seat)
        .ok_or_else(|| {
            CallError::naming(
                format!("\"{key}\" must be one of the seats"),
                &["allied", "japanese"],
            )
        })
}

/// The answer lines to one request line of a watched game (decision 0182):
/// none for a seat's call that is held, else its own, and after a step's
/// own line the lines of the held calls it brought to their stops.
fn watched_lines(
    session: &mut Session,
    held: &mut BTreeMap<Side, Value>,
    line: &str,
) -> Vec<String> {
    let Ok(Value::Object(object)) = serde_json::from_str::<Value>(line) else {
        return vec![answer_line(session, line)];
    };
    let id = object.get("id").cloned().unwrap_or(Value::Null);
    if let Some(what) = object.get("watcher") {
        const REQUESTS: [&str; 4] = ["step", "read", "standing", "seat_calls"];
        let outcome = match what.as_str() {
            Some("step") => {
                let stepped = session.step();
                let mut lines = vec![result_line(id, stepped.answer)];
                for (side, outcome) in stepped.answered {
                    let call = held.remove(&side).unwrap_or(Value::Null);
                    lines.push(result_line(call, outcome));
                }
                return lines;
            }
            Some("read") => side_of(&object, "side").and_then(|side| {
                let tool = object.get("tool").and_then(Value::as_str).ok_or_else(|| {
                    CallError::naming(
                        "\"tool\" must name a tool of the watcher's",
                        &session.watcher_tools(side),
                    )
                })?;
                let arguments = object.get("arguments").cloned().unwrap_or(Value::Null);
                session.watch(side, tool, &arguments)
            }),
            Some("standing") => Ok(session.standing()),
            Some("seat_calls") => side_of(&object, "side").map(|side| session.seat_calls(side)),
            _ => Err(CallError::naming(
                "\"watcher\" must name one of the watcher's requests",
                &REQUESTS,
            )),
        };
        return vec![result_line(id, outcome)];
    }
    if let Some(what) = object.get("request") {
        let outcome = side_of(&object, "seat").and_then(|side| {
            if session.seat_kind(side) != SeatKind::Player {
                return Err(CallError::new(format!(
                    "the {} seat is held by the script: it has no tools to give",
                    side.name().to_lowercase()
                )));
            }
            match what.as_str() {
                Some("tools") => Ok(json!({
                    "tools": sim_cli::mcp::tool_list_for(&session.tools(side)),
                })),
                Some("instructions") => Ok(json!({
                    "instructions": sim_cli::mcp::instructions_for(session, side),
                })),
                _ => Err(CallError::naming(
                    "\"request\" must name one of a seat's requests",
                    &["tools", "instructions"],
                )),
            }
        });
        return vec![result_line(id, outcome)];
    }
    // A seat's call: held where it is a call of time whose arguments stand.
    let (Ok(side), Some(tool)) = (
        side_of(&object, "seat"),
        object.get("tool").and_then(Value::as_str),
    ) else {
        return vec![answer_line(session, line)];
    };
    let arguments = object.get("arguments").cloned().unwrap_or(Value::Null);
    match session.seat_call(side, tool, &arguments) {
        Called::Answered(outcome) => vec![result_line(id, outcome)],
        Called::Held => {
            held.insert(side, id);
            Vec::new()
        }
    }
}

fn error_line(id: Value, error: &CallError) -> String {
    serde_json::to_string(&json!({ "id": id, "error": error.to_json() }))
        .expect("an answer serialises")
}

/// The answer to one request line.
fn answer_line(session: &mut Session, line: &str) -> String {
    let request: Value = match serde_json::from_str(line) {
        Ok(value) => value,
        Err(e) => {
            return error_line(
                Value::Null,
                &CallError::new(format!("the request is not valid JSON: {e}")),
            )
        }
    };
    let id = request.get("id").cloned().unwrap_or(Value::Null);
    let Some(object) = request.as_object() else {
        return error_line(
            id,
            &CallError::new(
                "a request is a JSON object: {\"id\": .., \"seat\": .., \"tool\": .., \"arguments\": ..}",
            ),
        );
    };
    let seat = object.get("seat").and_then(Value::as_str);
    let Some(side) = seat.and_then(parse_seat) else {
        return error_line(
            id,
            &CallError::naming("\"seat\" must be one of the seats", &["allied", "japanese"]),
        );
    };
    let Some(tool) = object.get("tool").and_then(Value::as_str) else {
        return error_line(
            id,
            &CallError::naming("\"tool\" must name a tool", &session.tools(side)),
        );
    };
    let arguments = object.get("arguments").cloned().unwrap_or(Value::Null);
    match session.call(side, tool, &arguments) {
        Ok(result) => serde_json::to_string(&json!({ "id": id, "result": result }))
            .expect("an answer serialises"),
        Err(error) => error_line(id, &error),
    }
}
