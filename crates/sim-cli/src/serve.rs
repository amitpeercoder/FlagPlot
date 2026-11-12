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

use std::collections::BTreeMap;
use std::io::{BufRead, Write};
use std::path::Path;

use serde_json::{json, Value};
use sim_agent::{parse_seat, CallError, Seat, SeatKind, Session, TOOLS};
use sim_core::orders::OrderRequest;
use sim_core::state::Side;
use sim_data::Scenario;

use crate::save;

/// What `serve` was asked to do.
struct Options<'a> {
    scenario: &'a str,
    seed: Option<u64>,
    seats: BTreeMap<Side, SeatKind>,
    save: Option<&'a str>,
    transcript: Option<&'a str>,
}

fn parse<'a>(args: &'a [String], usage: &str) -> Result<Options<'a>, String> {
    let mut scenario = None;
    let mut seed = None;
    let mut seats: BTreeMap<Side, SeatKind> = BTreeMap::new();
    let mut save = None;
    let mut transcript = None;
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
            "--save" => save = Some(value("--save")?),
            "--transcript" => transcript = Some(value("--transcript")?),
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
    Ok(Options {
        scenario,
        seed,
        seats,
        save,
        transcript,
    })
}

/// The orders a script seat gives, by day.
fn script_of(
    script: &BTreeMap<u32, Vec<(Side, OrderRequest)>>,
    side: Side,
) -> BTreeMap<u32, Vec<OrderRequest>> {
    let mut by_day: BTreeMap<u32, Vec<OrderRequest>> = BTreeMap::new();
    for (day, list) in script {
        for (who, request) in list {
            if *who == side {
                by_day.entry(*day).or_default().push(request.clone());
            }
        }
    }
    by_day
}

pub fn command(args: &[String], usage: &str) -> Result<(), String> {
    let options = parse(args, usage)?;
    let mut scenario = Scenario::load(Path::new(options.scenario)).map_err(|e| e.to_string())?;
    if let Some(seed) = options.seed {
        scenario = scenario.with_seed(seed).map_err(|e| e.to_string())?;
    }
    let remembered = save::remember(&scenario)?;
    let seat = |side: Side| match options.seats[&side] {
        SeatKind::Player => Seat::Player,
        SeatKind::Script => Seat::Script(script_of(remembered.script(), side)),
    };
    let (allied, japanese) = (seat(Side::Allied), seat(Side::Japanese));
    let seed = scenario.world.seed();
    let (name, days) = (scenario.name.clone(), scenario.days);
    let mut session = Session::new(scenario.world, &name, days, allied, japanese);

    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    for line in stdin.lock().lines() {
        let line = line.map_err(|e| format!("standard input: {e}"))?;
        let answer = answer_line(&mut session, &line);
        writeln!(out, "{answer}")
            .and_then(|()| out.flush())
            .map_err(|e| format!("standard output: {e}"))?;
    }

    if let Some(file) = options.save {
        let hashes: Vec<(u32, u64)> = (1u32..).zip(session.day_hashes().iter().copied()).collect();
        save::write(
            file,
            options.scenario,
            seed,
            &remembered,
            session.handed_in(),
            &options.seats,
            &hashes,
        )?;
    }
    if let Some(file) = options.transcript {
        let mut text = String::new();
        for entry in session.transcript() {
            text.push_str(&serde_json::to_string(&entry.to_json()).map_err(|e| e.to_string())?);
            text.push('\n');
        }
        std::fs::write(file, text).map_err(|e| format!("{file}: {e}"))?;
    }
    Ok(())
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
        return error_line(id, &CallError::naming("\"tool\" must name a tool", &TOOLS));
    };
    let arguments = object.get("arguments").cloned().unwrap_or(Value::Null);
    match session.call(side, tool, &arguments) {
        Ok(result) => serde_json::to_string(&json!({ "id": id, "result": result }))
            .expect("an answer serialises"),
        Err(error) => error_line(id, &error),
    }
}
