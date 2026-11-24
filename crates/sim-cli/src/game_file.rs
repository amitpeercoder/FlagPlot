//! The game file `sim-cli play` keeps, in the library (decision 0229,
//! ruling 2): its header, its pin, the reading of its calls and the replay
//! of them through the server of a seat. `play` is the program that writes
//! and reads the file on the disk; the door of `sim-web` makes and reads its
//! text and names no file.
//!
//! A game is a file of requests: a first line that names the scenario, the
//! seat and the seed, then one line for each tool call made, `{"tool": ..,
//! "arguments": ..}`. The game is deterministic, so the same requests give
//! the same game.

use std::collections::BTreeMap;
use std::path::Path;

use serde_json::{json, Value};
use sim_agent::{parse_seat, seat_name, Post, SeatKind, Session};
use sim_core::state::Side;
use sim_data::{Disk, Source};

use crate::game::{self, Kept};
use crate::mcp::Server;
use crate::save;

pub const FORMAT: &str = "flagplot-game";

/// What a game was begun on (decision 0172, 1): the seed it is played on,
/// the hash of the scenario file and of each data file, and the rules
/// version, as a saved game records them (`save.rs`).
#[derive(Clone, PartialEq, Debug)]
pub struct Pin {
    pub seed: u64,
    pub scenario_hash: String,
    pub data_files: Vec<(String, String)>,
    pub rules_version: u32,
}

impl Pin {
    /// The pin of a game opened now, from what the opening read.
    pub fn of(scenario: &str, kept: &Kept) -> Result<Pin, String> {
        Pin::of_from(&Disk, scenario, kept)
    }

    /// As [`Pin::of`], hashing the scenario file through `source`.
    pub fn of_from(source: &dyn Source, scenario: &str, kept: &Kept) -> Result<Pin, String> {
        Ok(Pin {
            seed: kept.seed(),
            scenario_hash: save::file_hash_from(source, Path::new(scenario))?,
            data_files: kept.data_files().to_vec(),
            rules_version: sim_core::world::RULES_VERSION,
        })
    }

    pub fn to_json(&self) -> Value {
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

    pub fn from_json(v: &Value) -> Option<Pin> {
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
    pub fn check_files(&self, game: &str, scenario: &str) -> Result<(), String> {
        self.check_files_from(&Disk, game, scenario)
    }

    /// As [`Pin::check_files`], reading the files through `source`.
    pub fn check_files_from(
        &self,
        source: &dyn Source,
        game: &str,
        scenario: &str,
    ) -> Result<(), String> {
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
        let now = save::file_hash_from(source, Path::new(scenario))
            .map_err(|e| refused(format!("the scenario file {scenario} cannot be read ({e})")))?;
        if now != self.scenario_hash {
            return Err(refused(format!(
                "the scenario file {scenario} has changed since the game was begun (begun on hash {}, now {now})",
                self.scenario_hash
            )));
        }
        for (file, hash) in &self.data_files {
            let now = save::file_hash_from(source, Path::new(file))
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
    pub fn check_opened(&self, game: &str, kept: &Kept) -> Result<(), String> {
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

pub struct Header {
    pub scenario: String,
    pub seat: Side,
    /// Where the seat sits (decision 0138, 5). A game file made before seats
    /// at a command has none, which is read as the area command.
    pub at: Post,
    pub seed: Option<u64>,
    /// The doctrine whose staff the game was begun under (decision 0107). A
    /// game file made before doctrines were recorded has none, which is read
    /// as `"none"`: no doctrine existed.
    pub doctrine_version: String,
    /// What the game was begun on (decision 0172, 1). A game file made
    /// before game files were pinned has none.
    pub pin: Option<Pin>,
}

impl Header {
    /// Refuses a game file begun under another doctrine than this program's:
    /// a game is resumed under the doctrine it was begun under, since the
    /// staff is rebuilt from its calls and another doctrine would play
    /// another game. It is not converted. `game` names the file in the words.
    pub fn check_doctrine(&self, game: &str) -> Result<(), String> {
        if self.doctrine_version != sim_staff::DOCTRINE_VERSION {
            return Err(format!(
                "{game} was begun under doctrine version \"{}\", and this program has doctrine version \"{}\"; the game is not converted, so begin it again",
                self.doctrine_version,
                sim_staff::DOCTRINE_VERSION
            ));
        }
        Ok(())
    }

    pub fn to_line(&self) -> String {
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

    pub fn from_line(line: &str, file: &str) -> Result<Header, String> {
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
pub struct Client {
    pub server: Server,
    pub kept: Kept,
    next_id: u64,
}

/// What a `tools/call` came to.
pub enum Outcome {
    /// The text of the answer and whether the game marked it an error.
    Answer { text: String, is_error: bool },
    /// The protocol refused the request.
    Refused(String),
}

impl Client {
    /// Opens the game and does what a client does first: `initialize` and
    /// `notifications/initialized`.
    pub fn open(header: &Header) -> Result<Client, String> {
        Client::open_from(&Disk, header)
    }

    /// As [`Client::open`], reading the game's files through `source`.
    pub fn open_from(source: &dyn Source, header: &Header) -> Result<Client, String> {
        let seats = BTreeMap::from([(header.seat, SeatKind::Player)]);
        let posts = BTreeMap::from([(header.seat, header.at)]);
        let (session, kept) =
            game::open_from(source, &header.scenario, header.seed, seats, &posts)?;
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

    pub fn send(&mut self, method: &str, params: Value) -> Value {
        let id = self.next_id;
        self.next_id += 1;
        let line = json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params});
        let answer = self
            .server
            .handle_line(&line.to_string())
            .expect("a request is answered");
        serde_json::from_str(&answer).expect("the server's answer is JSON")
    }

    pub fn call(&mut self, tool: &str, arguments: &Value) -> Outcome {
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
pub fn replay(header: &Header, calls: &[(String, Value)]) -> Result<Client, String> {
    let mut client = Client::open(header)?;
    for (i, (tool, arguments)) in calls.iter().enumerate() {
        if let Outcome::Refused(why) = client.call(tool, arguments) {
            return Err(format!("call {} of the game file is refused: {why}", i + 1));
        }
    }
    Ok(client)
}

pub fn read_calls(text: &str, file: &str) -> Result<(Header, Vec<(String, Value)>), String> {
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
    pub fn finish(&self, save: Option<&str>, transcript: Option<&str>) -> Result<(), String> {
        if save.is_some() {
            refuse_unfinished(&self.server.session)?;
        }
        self.kept.finish(&self.server.session, save, transcript)
    }
}

/// The words with which a saved game is refused while the game is not
/// over: `Ok` once it is over and did not stop inside a day. The saved game
/// holds both sides' orders, so it is given only when the game has ended.
pub fn refuse_unfinished(session: &Session) -> Result<(), String> {
    if let Some(day) = session.stopped_inside_day() {
        return Err(format!("the game stopped inside day {day} on an internal error and did not end: there is no saved game of it; the transcript says what each call was told"));
    }
    if !session.is_over() {
        return Err("the game is not over: a saved game holds both sides' orders and is given only when the game has ended".to_string());
    }
    Ok(())
}
