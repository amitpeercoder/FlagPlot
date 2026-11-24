//! The door of one seat (decision 0229, rulings 1 to 3, 17, 19 and 22).
//!
//! A [`Door`] holds one game and answers one seat, as `sim-cli mcp` does: it
//! opens the game from a pack (the files `sim-data` read, made by
//! `sim-cli pack`), calls the seat's tools through the function the MCP
//! server calls, and keeps the seat's own calls as a game file `sim-cli
//! play` reads. It is plain Rust with no binding in it; a build for the
//! browser binds it at its edge, and the program `door` (`src/bin/door.rs`)
//! answers it from standard input.
//!
//! `sim-web` stands where `sim-cli mcp` stands: it is the umpire's side of a
//! seat's door. It holds the session, which holds the world. What plays is
//! given the answers of the seat's tools and nothing else. It never gives a
//! hash of the state while a game is in play: the hashes of the days reach
//! the outside only inside the saved game, when the game is over.

#[cfg(target_arch = "wasm32")]
mod binding;

use std::collections::BTreeMap;

use serde_json::{json, Value};
use sim_agent::{parse_seat, Post, SeatKind, Session, WATCHER_TOOLS};
use sim_cli::game::{self, Kept};
use sim_cli::game_file::{self, Header, Pin};
use sim_cli::mcp::{call_seat_tool, tool_list_for};
use sim_cli::pack::{self, Versions};
use sim_core::state::Side;
use sim_data::Pack;

/// The one game of a door.
struct Held {
    pack: Pack,
    session: Session,
    kept: Kept,
    side: Side,
    header: Header,
    /// The seat's calls the game answered that are not reads, in the order
    /// made (ruling 33).
    calls: Vec<(String, Value)>,
}

/// One seat's door: no game until `begin` or `resume`, then one for the
/// life of the door.
#[derive(Default)]
pub struct Door {
    held: Option<Held>,
}

/// The versions of this build as the text `versions` gives.
fn versions_text(versions: &Versions) -> String {
    json!({
        "rules": versions.rules,
        "state": versions.state,
        "doctrine": versions.doctrine,
    })
    .to_string()
}

/// The pack in `bytes`, refused in one sentence if it was made under
/// versions other than this build's (ruling 11).
fn read_pack(bytes: &[u8]) -> Result<Pack, String> {
    let text = std::str::from_utf8(bytes).map_err(|_| "the pack is not text".to_string())?;
    let (made, pack) = pack::read_pack(text)?;
    let own = pack::versions();
    if made != own {
        return Err(format!(
            "the pack was made under rules version {}, state version {} and doctrine version {}, and this build has rules version {}, state version {} and doctrine version {}",
            made.rules, made.state, made.doctrine, own.rules, own.state, own.doctrine
        ));
    }
    Ok(pack)
}

impl Door {
    /// A door with no game.
    pub fn new() -> Door {
        Door::default()
    }

    /// The rules, state and doctrine versions this build was made with, as
    /// a JSON object.
    pub fn versions(&self) -> String {
        versions_text(&pack::versions())
    }

    /// Opens the game of a request, a JSON object with `scenario` (the path
    /// of its file in the pack), `seat` (`allied` or `japanese`) and `seed`
    /// (a string of digits, or null or left out for the scenario's own). A
    /// door that has a game refuses it.
    pub fn begin(&mut self, pack: &[u8], request: &str) -> Result<(), String> {
        self.refuse_second()?;
        let request: Value =
            serde_json::from_str(request).map_err(|e| format!("the request is not JSON: {e}"))?;
        let scenario = request
            .get("scenario")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                "the request needs \"scenario\": the path of the scenario".to_string()
            })?;
        let seat = request
            .get("seat")
            .and_then(Value::as_str)
            .and_then(parse_seat)
            .ok_or_else(|| "the request needs \"seat\": allied or japanese".to_string())?;
        let seed = match request.get("seed") {
            None | Some(Value::Null) => None,
            Some(Value::String(digits))
                if !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()) =>
            {
                Some(
                    digits
                        .parse::<u64>()
                        .map_err(|_| format!("the seed \"{digits}\" is too large"))?,
                )
            }
            Some(_) => return Err("\"seed\" is a string of digits".to_string()),
        };
        let pack = read_pack(pack)?;
        let (session, kept) = open(&pack, scenario, seed, seat, Post::Area)?;
        let pin = Pin::of_from(&pack, scenario, &kept)?;
        let header = Header {
            scenario: scenario.to_string(),
            seat,
            at: Post::Area,
            seed,
            doctrine_version: pack::versions().doctrine,
            pin: Some(pin),
        };
        self.held = Some(Held {
            pack,
            session,
            kept,
            side: seat,
            header,
            calls: Vec::new(),
        });
        Ok(())
    }

    /// The seat's tools, as `tools/list` gives them: `{"tools": [..]}`.
    pub fn tools(&self) -> Result<String, String> {
        let held = self.game()?;
        Ok(json!({"tools": tool_list_for(&held.session.tools(held.side))}).to_string())
    }

    /// One tool call, its arguments a JSON object as text (or nothing):
    /// `{"ok": the answer}` or `{"refused": the words}`, as the table's
    /// bridge answers a call.
    pub fn call(&mut self, tool: &str, arguments: &str) -> Result<String, String> {
        let held = self.game_mut()?;
        let arguments: Option<Value> = if arguments.trim().is_empty() {
            None
        } else {
            match serde_json::from_str(arguments) {
                Ok(value) => Some(value),
                Err(e) => {
                    return Ok(
                        json!({"refused": format!("the arguments are not JSON: {e}")}).to_string(),
                    )
                }
            }
        };
        let answer = match call_seat_tool(&mut held.session, held.side, tool, arguments.as_ref()) {
            Ok(answer) => answer,
            Err(words) => return Ok(json!({"refused": words}).to_string()),
        };
        // The call happened. A read is left out of the game file; every
        // other call is kept, a refused one among them (ruling 33).
        if !WATCHER_TOOLS.contains(&tool) {
            let kept_arguments = match arguments {
                None | Some(Value::Null) => json!({}),
                Some(value) => value,
            };
            held.calls.push((tool.to_string(), kept_arguments));
        }
        Ok(if answer.is_error {
            json!({"refused": answer.text})
        } else {
            match serde_json::from_str::<Value>(&answer.text) {
                Ok(value) => json!({"ok": value}),
                Err(_) => json!({"ok": {"text": answer.text}}),
            }
        }
        .to_string())
    }

    /// The game file of the game as it stands: a game file `sim-cli play`
    /// reads, its header and each call of the seat the game answered that
    /// is not a read, the calls it refused among them.
    pub fn game_file(&self) -> Result<String, String> {
        let held = self.game()?;
        let mut text = held.header.to_line();
        text.push('\n');
        for (tool, arguments) in &held.calls {
            text.push_str(&json!({"tool": tool, "arguments": arguments}).to_string());
            text.push('\n');
        }
        Ok(text)
    }

    /// Opens the game of a game file by replaying its calls, as `sim-cli
    /// play` does. A file begun under another doctrine, or on inputs the
    /// pack does not hold as they were, is refused. A door that has a game
    /// refuses it.
    pub fn resume(&mut self, pack: &[u8], game_file: &str) -> Result<(), String> {
        self.refuse_second()?;
        let pack = read_pack(pack)?;
        let name = "the game file";
        let (header, calls) = game_file::read_calls(game_file, name)?;
        header.check_doctrine(name)?;
        let Some(pin) = header.pin.clone() else {
            return Err(format!(
                "{name} was begun before game files recorded their inputs, so whether its scenario or a data file has changed since cannot be told; the game is not replayed, so begin it again"
            ));
        };
        pin.check_files_from(&pack, name, &header.scenario)?;
        let (mut session, kept) =
            open(&pack, &header.scenario, header.seed, header.seat, header.at)?;
        pin.check_opened(name, &kept)?;
        let mut held_calls = Vec::new();
        for (i, (tool, arguments)) in calls.into_iter().enumerate() {
            if let Err(why) = call_seat_tool(&mut session, header.seat, &tool, Some(&arguments)) {
                return Err(format!("call {} of the game file is refused: {why}", i + 1));
            }
            if !WATCHER_TOOLS.contains(&tool.as_str()) {
                held_calls.push((tool, arguments));
            }
        }
        self.held = Some(Held {
            pack,
            session,
            kept,
            side: header.seat,
            header,
            calls: held_calls,
        });
        Ok(())
    }

    /// The saved game, as `sim-cli play --export-save` writes it. Refused
    /// until the game is over, in the words `play` uses.
    pub fn saved_game(&self) -> Result<String, String> {
        let held = self.game()?;
        game_file::refuse_unfinished(&held.session)?;
        held.kept.saved_game_text(&held.pack, &held.session)
    }

    fn game(&self) -> Result<&Held, String> {
        self.held
            .as_ref()
            .ok_or_else(|| "no game has begun on this door".to_string())
    }

    fn game_mut(&mut self) -> Result<&mut Held, String> {
        self.held
            .as_mut()
            .ok_or_else(|| "no game has begun on this door".to_string())
    }

    fn refuse_second(&self) -> Result<(), String> {
        if self.held.is_some() {
            return Err(
                "this door has a game: it serves one seat for the life of the module, so begin it again on a new one"
                    .to_string(),
            );
        }
        Ok(())
    }
}

/// Opens the session of one player seat over the pack.
fn open(
    pack: &Pack,
    scenario: &str,
    seed: Option<u64>,
    side: Side,
    post: Post,
) -> Result<(Session, Kept), String> {
    let seats = BTreeMap::from([(side, SeatKind::Player)]);
    let posts = BTreeMap::from([(side, post)]);
    game::open_from(pack, scenario, seed, seats, &posts)
}
