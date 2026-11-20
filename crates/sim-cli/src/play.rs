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
use sim_agent::{parse_seat, seat_name, Post, SeatKind};
use sim_core::state::Side;

use crate::mcp::{tool_list, tool_list_for, Server};
use crate::save;
use crate::serve::{self, Kept};
use sim_staff::DOCTRINE_VERSION;

const FORMAT: &str = "flagplot-game";

pub const USAGE: &str = "\
Usage: sim-cli play --game FILE [--scenario SCENARIO --seat allied|japanese [--at SEAT] [--seed N]] [--unpinned] TOOL [ARGUMENTS_JSON]
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
                     schema, as the MCP server lists them. With a game file
                     that exists, or --scenario and --seat, the tools of
                     that game's seat (a game may offer more: a dry run,
                     orders inside a day); with neither, the common ones.
  --export-save F    Replay the game and write the saved game (which
                     `sim-cli replay` runs with no player). Only once the
                     game is over: a saved game holds both sides' orders.
  --export-transcript F  Replay the game and write the transcript: every call
                     with the whole answer it was given, one JSON line each.
  --seed N           (first call) Play with seed N instead of the scenario's.
  --unpinned         Continue a game file made before game files recorded
                     their inputs. A game file records the seed it is played
                     on, the hash of the scenario file and of each data file,
                     and the rules version; a call is refused, and nothing is
                     written, if any of them has changed since. A game file
                     with no such record is exported as before, but a new call
                     is made on it only with --unpinned.
  --at SEAT          (first call) Where the seat sits: area (the default),
                     area:easier (the side's own picture, every report at once,
                     and its forces exactly), or task_force:N or base:N, N the
                     number the scenario file gives it, for a seat that is given
                     only what that command holds and may order its own units
                     only (command levels on). Kept in FILE.";

/// What a game was begun on (decision 0172, 1): the seed it is played on,
/// the hash of the scenario file and of each data file, and the rules
/// version, as a saved game records them (`save.rs`).
#[derive(Clone, PartialEq, Debug)]
struct Pin {
    seed: u64,
    scenario_hash: String,
    data_files: Vec<(String, String)>,
    rules_version: u32,
}

impl Pin {
    /// The pin of a game opened now, from what the opening read.
    fn of(scenario: &str, kept: &Kept) -> Result<Pin, String> {
        Ok(Pin {
            seed: kept.seed(),
            scenario_hash: save::file_hash(Path::new(scenario))?,
            data_files: kept.data_files().to_vec(),
            rules_version: sim_core::world::RULES_VERSION,
        })
    }

    fn to_json(&self) -> Value {
        json!({
            "seed": self.seed,
            "scenario_hash": self.scenario_hash,
            "data_files": self
                .data_files
                .iter()
                .map(|(file, hash)| json!({"file": file, "hash": hash}))
                .collect::<Vec<_>>(),
            "rules_version": self.rules_version,
        })
    }

    fn from_json(v: &Value) -> Option<Pin> {
        let mut data_files = Vec::new();
        for entry in v.get("data_files")?.as_array()? {
            data_files.push((
                entry.get("file")?.as_str()?.to_string(),
                entry.get("hash")?.as_str()?.to_string(),
            ));
        }
        Some(Pin {
            seed: v.get("seed")?.as_u64()?,
            scenario_hash: v.get("scenario_hash")?.as_str()?.to_string(),
            data_files,
            rules_version: u32::try_from(v.get("rules_version")?.as_u64()?).ok()?,
        })
    }

    /// Refuses a game whose inputs on disk are not those it was begun on,
    /// naming what changed. It reads the files only: nothing is loaded.
    fn check_files(&self, game: &str, scenario: &str) -> Result<(), String> {
        let refused = |what: String| {
            format!("{game} cannot be replayed: {what}; the game is not replayed on other inputs, so begin it again")
        };
        if self.rules_version != sim_core::world::RULES_VERSION {
            return Err(refused(format!(
                "it was begun under rules version {}, and this program has rules version {}",
                self.rules_version,
                sim_core::world::RULES_VERSION
            )));
        }
        let now = save::file_hash(Path::new(scenario))
            .map_err(|e| refused(format!("the scenario file {scenario} cannot be read ({e})")))?;
        if now != self.scenario_hash {
            return Err(refused(format!(
                "the scenario file {scenario} has changed since the game was begun (begun on hash {}, now {now})",
                self.scenario_hash
            )));
        }
        for (file, hash) in &self.data_files {
            let now = save::file_hash(Path::new(file))
                .map_err(|e| refused(format!("the data file {file} cannot be read ({e})")))?;
            if &now != hash {
                return Err(refused(format!(
                    "the data file {file} has changed since the game was begun (begun on hash {hash}, now {now})"
                )));
            }
        }
        Ok(())
    }

    /// Refuses a game opened on a seed or a list of data files other than
    /// the pinned: with the files unchanged neither can differ, but the game
    /// is not played if one does.
    fn check_opened(&self, game: &str, kept: &Kept) -> Result<(), String> {
        if kept.seed() != self.seed {
            return Err(format!(
                "{game} cannot be replayed: it was begun on seed {}, and the scenario now gives seed {}; begin it again",
                self.seed,
                kept.seed()
            ));
        }
        let pinned: Vec<&str> = self.data_files.iter().map(|(f, _)| f.as_str()).collect();
        let now: Vec<&str> = kept.data_files().iter().map(|(f, _)| f.as_str()).collect();
        if pinned != now {
            return Err(format!(
                "{game} cannot be replayed: the scenario now names other data files than when the game was begun; begin it again"
            ));
        }
        Ok(())
    }
}

struct Header {
    scenario: String,
    seat: Side,
    /// Where the seat sits (decision 0138, 5). A game file made before seats
    /// at a command has none, which is read as the area command.
    at: Post,
    seed: Option<u64>,
    /// The doctrine whose staff the game was begun under (decision 0107). A
    /// game file made before doctrines were recorded has none, which is read
    /// as `"none"`: no doctrine existed.
    doctrine_version: String,
    /// What the game was begun on (decision 0172, 1). A game file made
    /// before game files were pinned has none.
    pin: Option<Pin>,
}

impl Header {
    fn to_line(&self) -> String {
        let mut header = json!({
            "format": FORMAT,
            "scenario": self.scenario,
            "seat": seat_name(self.seat),
            "seed": self.seed,
            "doctrine_version": self.doctrine_version,
        });
        if self.at != Post::Area {
            header["at"] = json!(self.at.text());
        }
        if let Some(pin) = &self.pin {
            header["inputs"] = pin.to_json();
        }
        serde_json::to_string(&header).expect("a header serialises")
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
        let doctrine_version = match v.get("doctrine_version") {
            None => "none".to_string(),
            Some(version) => version.as_str().ok_or_else(bad)?.to_string(),
        };
        let at = match v.get("at") {
            None => Post::Area,
            Some(at) => at
                .as_str()
                .and_then(|at| Post::parse(at).ok())
                .ok_or_else(bad)?,
        };
        let pin = match v.get("inputs") {
            None => None,
            Some(inputs) => Some(Pin::from_json(inputs).ok_or_else(bad)?),
        };
        Ok(Header {
            at,
            doctrine_version,
            pin,
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
        let posts = BTreeMap::from([(header.seat, header.at)]);
        let (session, kept) = serve::open(&header.scenario, header.seed, seats, &posts)?;
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
        if let (Some(_), Some(day)) = (save, self.server.session.stopped_inside_day()) {
            return Err(format!("the game stopped inside day {day} on an internal error and did not end: there is no saved game of it; the transcript says what each call was told"));
        }
        if save.is_some() && !self.server.session.is_over() {
            return Err("the game is not over: a saved game holds both sides' orders and is given only when the game has ended".to_string());
        }
        self.kept.finish(&self.server.session, save, transcript)
    }
}

pub fn command(args: &[String]) -> Result<(), String> {
    let mut game = None;
    let mut scenario = None;
    let mut seat = None;
    let mut at = None;
    let mut seed = None;
    let mut export_save = None;
    let mut export_transcript = None;
    let mut tools = false;
    let mut unpinned = false;
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
            "--at" => at = Some(Post::parse(value("--at")?)?),
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
            "--unpinned" => unpinned = true,
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
        // The tools of this game's seat, where a game is named: a game
        // file that exists, or a scenario and a seat. A seat is offered
        // tools by its game (a trial, orders inside a day, a seat at a
        // command), so the list that needs no game leaves those out.
        let header = match game {
            Some(file) if Path::new(file).exists() => {
                let text = std::fs::read_to_string(file).map_err(|e| format!("{file}: {e}"))?;
                Some(read_calls(&text, file)?.0)
            }
            _ => None,
        };
        let opened = match (&header, scenario, seat) {
            (Some(header), _, _) => {
                Some((header.scenario.clone(), header.seat, header.at, header.seed))
            }
            (None, Some(scenario), Some(seat)) => {
                Some((scenario.to_string(), seat, at.unwrap_or(Post::Area), seed))
            }
            _ => None,
        };
        let list = match opened {
            Some((scenario, seat, at, seed)) => {
                let seats = BTreeMap::from([(seat, SeatKind::Player)]);
                let posts = BTreeMap::from([(seat, at)]);
                let (session, _) = serve::open(&scenario, seed, seats, &posts)?;
                tool_list_for(&session.tools(seat))
            }
            None => tool_list(),
        };
        let mut text = String::new();
        for tool in list {
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
        if let Some(post) = at {
            if post != header.at {
                return Err(format!(
                    "--at {} is not the game's seat {} (in {game})",
                    post.text(),
                    header.at.text()
                ));
            }
        }
        if let Some(n) = seed {
            if Some(n) != header.seed {
                return Err(format!("--seed {n} is not the game's seed (in {game})"));
            }
        }
        // A game is resumed under the doctrine it was begun under: the staff
        // is rebuilt from its calls, and another doctrine would play another
        // game. It is not converted.
        if header.doctrine_version != DOCTRINE_VERSION {
            return Err(format!(
                "{game} was begun under doctrine version \"{}\", and this program has doctrine version \"{DOCTRINE_VERSION}\"; the game is not converted, so begin it again",
                header.doctrine_version
            ));
        }
        // A game is replayed only on the inputs it was begun on (decision
        // 0172, 1); one begun before game files recorded them is exported
        // as before, and given a new call only on the caller's word.
        match &header.pin {
            Some(pin) => {
                if unpinned {
                    return Err(format!(
                        "--unpinned is for a game file made before game files recorded their inputs, and {game} records them"
                    ));
                }
                pin.check_files(game, &header.scenario)?;
            }
            None if !exporting && !unpinned => {
                return Err(format!(
                    "{game} was begun before game files recorded their inputs, so whether its scenario or a data file has changed since cannot be told; to continue it as it is, say so with --unpinned"
                ));
            }
            None => {}
        }
        (header, calls)
    } else {
        if unpinned {
            return Err(format!(
                "{game} is a new game, and a new game records its inputs: --unpinned is for a game file made before"
            ));
        }
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
            at: at.unwrap_or_default(),
            seed,
            doctrine_version: DOCTRINE_VERSION.to_string(),
            pin: None,
        };
        (header, Vec::new())
    };

    let mut client = replay(&header, &calls)?;
    // A new game is pinned to what it was opened on; a pinned one is checked
    // against what it was opened on now, before any call is kept.
    let header = match (&header.pin, existing) {
        (Some(pin), _) => {
            pin.check_opened(game, &client.kept)?;
            header
        }
        (None, false) => Header {
            pin: Some(Pin::of(&header.scenario, &client.kept)?),
            ..header
        },
        (None, true) => header,
    };

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
