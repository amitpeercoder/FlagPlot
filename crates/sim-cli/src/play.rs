//! `sim-cli play`: one call to the MCP server, then exit (decisions 5 and 6
//! of 0051).
//!
//! A game is a file of requests: a first line that names the scenario, the
//! seat and the seed, then one line for each tool call made, `{"tool": ..,
//! "arguments": ..}`. Each time, `play` opens the game afresh, sends the
//! requests of the file and then the new one through the same
//! [`Server::handle_line`] the held-open server uses, prints the new answer
//! and adds the call to the file. The game is deterministic, so the same
//! requests give the same game. It reads no clock.

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;

use serde_json::{json, Value};
use sim_agent::{parse_seat, seat_name, SeatKind};
use sim_core::state::Side;

use crate::mcp::{tool_list, Server};
use crate::serve::{self, Kept};

const FORMAT: &str = "flagplot-game";

pub const USAGE: &str = "\
Usage: sim-cli play --game FILE [--scenario SCENARIO --seat allied|japanese [--seed N]] TOOL [ARGUMENTS_JSON]
       sim-cli play --game FILE --tools
       sim-cli play --game FILE --export-save FILE --export-transcript FILE

  play makes ONE call to the Flag Plot game and prints the answer. The game is
  kept in FILE, a file of the calls made so far: the first call creates it
  (and then needs --scenario and --seat; later calls may leave them out) and
  every call that the game answers is added to it. Each call replays the file
  first, so a game of any length is a series of separate commands. The answer
  is printed on standard output. If the call is refused, the reason is printed
  on standard error and the exit code is 1. A call refused by the game (a bad
  order, an unknown handle) is still a call that was made and is kept; a
  mistake in the command line (a tool that does not exist, arguments that are
  not JSON) is not kept.

  Example (a first turn of Eastern Solomons as the Allied side):
    sim-cli play --game g.txt --scenario data/scenarios/05_eastern_solomons.toml --seat allied --tools
    sim-cli play --game g.txt --scenario data/scenarios/05_eastern_solomons.toml --seat allied get_briefing
    sim-cli play --game g.txt measure '{\"question\": \"air_distance\", \"from\": \"task_force:1\", \"to\": \"base:1\"}'
    sim-cli play --game g.txt issue_orders '{\"orders\": [{\"unit\": \"air_group:1\", \"order\": {\"kind\": \"set_air_mission\", \"mission\": \"combat_air_patrol\"}}]}'
    sim-cli play --game g.txt end_day

  --tools            Print the tools, each with its description and input
                     schema, as the MCP server lists them. Needs no game.
  --export-save F    Replay the game and write the saved game (which
                     `sim-cli replay` runs with no player).
  --export-transcript F  Replay the game and write the transcript: every call
                     with the whole answer it was given, one JSON line each.
  --seed N           (first call) Play with seed N instead of the scenario's.";

struct Header {
    scenario: String,
    seat: Side,
    seed: Option<u64>,
}

impl Header {
    fn to_line(&self) -> String {
        serde_json::to_string(&json!({
            "format": FORMAT,
            "scenario": self.scenario,
            "seat": seat_name(self.seat),
            "seed": self.seed,
        }))
        .expect("a header serialises")
    }

    fn from_line(line: &str, file: &str) -> Result<Header, String> {
        let bad = || format!("{file}: the first line is not the header of a game file");
        let v: Value = serde_json::from_str(line).map_err(|_| bad())?;
        if v.get("format") != Some(&json!(FORMAT)) {
            return Err(bad());
        }
        let seed = match v.get("seed") {
            Some(Value::Null) | None => None,
            Some(n) => Some(n.as_u64().ok_or_else(bad)?),
        };
        Ok(Header {
            scenario: v
                .get("scenario")
                .and_then(Value::as_str)
                .ok_or_else(bad)?
                .to_string(),
            seat: v
                .get("seat")
                .and_then(Value::as_str)
                .and_then(parse_seat)
                .ok_or_else(bad)?,
            seed,
        })
    }
}

/// A game opened and spoken to as a client would: each message is a line sent
/// to the server's own line handler.
struct Client {
    server: Server,
    kept: Kept,
    next_id: u64,
}

/// What a `tools/call` came to.
enum Outcome {
    /// The text of the answer and whether the game marked it an error.
    Answer { text: String, is_error: bool },
    /// The protocol refused the request.
    Refused(String),
}

impl Client {
    /// Opens the game and does what a client does first: `initialize` and
    /// `notifications/initialized`.
    fn open(header: &Header) -> Result<Client, String> {
        let seats = BTreeMap::from([(header.seat, SeatKind::Player)]);
        let (session, kept) = serve::open(&header.scenario, header.seed, seats)?;
        let mut client = Client {
            server: Server::new(session, header.seat),
            kept,
            next_id: 1,
        };
        let init = client.send(
            "initialize",
            json!({"protocolVersion": "2025-11-25", "capabilities": {}, "clientInfo": {"name": "sim-cli play", "version": "1"}}),
        );
        if init.get("result").is_none() {
            return Err(format!("the server refused initialize: {init}"));
        }
        let note = json!({"jsonrpc": "2.0", "method": "notifications/initialized"}).to_string();
        if client.server.handle_line(&note).is_some() {
            return Err("a notification was answered".to_string());
        }
        Ok(client)
    }

    fn send(&mut self, method: &str, params: Value) -> Value {
        let id = self.next_id;
        self.next_id += 1;
        let line = json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params});
        let answer = self
            .server
            .handle_line(&line.to_string())
            .expect("a request is answered");
        serde_json::from_str(&answer).expect("the server's answer is JSON")
    }

    fn call(&mut self, tool: &str, arguments: &Value) -> Outcome {
        let answer = self.send("tools/call", json!({"name": tool, "arguments": arguments}));
        if let Some(error) = answer.get("error") {
            return Outcome::Refused(
                error
                    .get("message")
                    .and_then(Value::as_str)
                    .unwrap_or("refused")
                    .to_string(),
            );
        }
        let result = &answer["result"];
        Outcome::Answer {
            text: result["content"][0]["text"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            is_error: result["isError"] == json!(true),
        }
    }
}

/// Opens a game and replays the calls of a game file.
fn replay(header: &Header, calls: &[(String, Value)]) -> Result<Client, String> {
    let mut client = Client::open(header)?;
    for (i, (tool, arguments)) in calls.iter().enumerate() {
        if let Outcome::Refused(why) = client.call(tool, arguments) {
            return Err(format!("call {} of the game file is refused: {why}", i + 1));
        }
    }
    Ok(client)
}

fn read_calls(text: &str, file: &str) -> Result<(Header, Vec<(String, Value)>), String> {
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let first = lines
        .next()
        .ok_or_else(|| format!("{file} is empty: it is not a game file"))?;
    let header = Header::from_line(first, file)?;
    let mut calls = Vec::new();
    for (i, line) in lines.enumerate() {
        let bad = || format!("{file}: line {} is not a call", i + 2);
        let v: Value = serde_json::from_str(line).map_err(|_| bad())?;
        let tool = v.get("tool").and_then(Value::as_str).ok_or_else(bad)?;
        calls.push((
            tool.to_string(),
            v.get("arguments").cloned().unwrap_or_else(|| json!({})),
        ));
    }
    Ok((header, calls))
}

impl Client {
    fn finish(&self, save: Option<&str>, transcript: Option<&str>) -> Result<(), String> {
        self.kept.finish(&self.server.session, save, transcript)
    }
}

pub fn command(args: &[String]) -> Result<(), String> {
    let mut game = None;
    let mut scenario = None;
    let mut seat = None;
    let mut seed = None;
    let mut export_save = None;
    let mut export_transcript = None;
    let mut tools = false;
    let mut positional: Vec<&str> = Vec::new();
    let mut words = args.iter().map(String::as_str);
    while let Some(word) = words.next() {
        let mut value = |what: &str| {
            words
                .next()
                .ok_or_else(|| format!("{what} needs a value\n\n{USAGE}"))
        };
        match word {
            "--help" | "-h" => {
                println!("{USAGE}");
                return Ok(());
            }
            "--game" => game = Some(value("--game")?),
            "--scenario" => scenario = Some(value("--scenario")?),
            "--seat" => {
                let v = value("--seat")?;
                seat = Some(
                    parse_seat(v)
                        .ok_or_else(|| format!("--seat \"{v}\" is not allied or japanese"))?,
                );
            }
            "--seed" => {
                let v = value("--seed")?;
                seed = Some(
                    v.parse::<u64>()
                        .map_err(|_| format!("--seed \"{v}\" is not a whole number"))?,
                );
            }
            "--export-save" => export_save = Some(value("--export-save")?),
            "--export-transcript" => export_transcript = Some(value("--export-transcript")?),
            "--tools" => tools = true,
            flag if flag.starts_with("--") => {
                return Err(format!("unknown option \"{flag}\"\n\n{USAGE}"))
            }
            other => positional.push(other),
        }
    }
    let exporting = export_save.is_some() || export_transcript.is_some();
    if positional.len() > 2 {
        return Err(format!(
            "too many arguments: a tool and, in quotes, its arguments as one JSON object\n\n{USAGE}"
        ));
    }

    // Nothing to do but say how.
    if game.is_none() && !tools {
        println!("{USAGE}");
        return Ok(());
    }
    if tools {
        let mut text = String::new();
        for tool in tool_list() {
            text.push_str(&format!(
                "TOOL {}\n{}\n\ninput schema:\n{}\n\n",
                tool["name"].as_str().unwrap_or_default(),
                tool["description"].as_str().unwrap_or_default(),
                serde_json::to_string_pretty(&tool["inputSchema"]).expect("a schema serialises"),
            ));
        }
        print!("{text}");
        return Ok(());
    }
    let game = game.expect("checked above");
    if positional.is_empty() && !exporting {
        println!("{USAGE}");
        return Ok(());
    }

    let existing = Path::new(game).exists();
    let (header, calls) = if existing {
        let text = std::fs::read_to_string(game).map_err(|e| format!("{game}: {e}"))?;
        let (header, calls) = read_calls(&text, game)?;
        if let Some(s) = scenario {
            if s != header.scenario {
                return Err(format!(
                    "--scenario {s} is not the game's scenario {} (in {game})",
                    header.scenario
                ));
            }
        }
        if let Some(s) = seat {
            if s != header.seat {
                return Err(format!(
                    "--seat {} is not the game's seat {} (in {game})",
                    seat_name(s),
                    seat_name(header.seat)
                ));
            }
        }
        if let Some(n) = seed {
            if Some(n) != header.seed {
                return Err(format!("--seed {n} is not the game's seed (in {game})"));
            }
        }
        (header, calls)
    } else {
        if exporting {
            return Err(format!("{game} does not exist: there is no game to export"));
        }
        let header = Header {
            scenario: scenario
                .ok_or_else(|| {
                    format!("{game} is a new game: --scenario and --seat are needed\n\n{USAGE}")
                })?
                .to_string(),
            seat: seat.ok_or_else(|| {
                format!("{game} is a new game: --scenario and --seat are needed\n\n{USAGE}")
            })?,
            seed,
        };
        (header, Vec::new())
    };

    let mut client = replay(&header, &calls)?;

    if exporting {
        if !positional.is_empty() {
            return Err("--export-save and --export-transcript make no call".to_string());
        }
        return client.finish(export_save, export_transcript);
    }

    let tool = positional[0];
    let arguments: Value = match positional.get(1) {
        None => json!({}),
        Some(text) => {
            serde_json::from_str(text).map_err(|e| format!("the arguments are not JSON: {e}"))?
        }
    };
    match client.call(tool, &arguments) {
        Outcome::Refused(why) => Err(why),
        Outcome::Answer { text, is_error } => {
            // The call happened: keep it, then say what came of it.
            append(game, existing, &header, tool, &arguments)?;
            if is_error {
                Err(text)
            } else {
                println!("{text}");
                Ok(())
            }
        }
    }
}

fn append(
    file: &str,
    existing: bool,
    header: &Header,
    tool: &str,
    arguments: &Value,
) -> Result<(), String> {
    let mut text = String::new();
    if !existing {
        text.push_str(&header.to_line());
        text.push('\n');
    }
    text.push_str(
        &serde_json::to_string(&json!({"tool": tool, "arguments": arguments}))
            .expect("a call serialises"),
    );
    text.push('\n');
    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(file)
        .and_then(|mut f| f.write_all(text.as_bytes()))
        .map_err(|e| format!("{file}: {e}"))
}
