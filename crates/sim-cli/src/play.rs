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
use sim_cli::game;
use sim_cli::game_file::{read_calls, replay, Header, Outcome, Pin};
use sim_cli::mcp::{tool_list, tool_list_for};
use sim_staff::DOCTRINE_VERSION;

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
                let (session, _) = game::open(&scenario, seed, seats, &posts)?;
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
        header.check_doctrine(game)?;
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
