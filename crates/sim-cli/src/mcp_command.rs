//! `sim-cli mcp`'s command: reads the server's requests on standard input
//! and answers them on standard output. The server itself is in the
//! library (`sim_cli::mcp`, decision 0229, ruling 2); this part reads
//! standard input and so stays in the program.

use std::collections::BTreeMap;

use sim_agent::{parse_seat, Post};
use sim_cli::mcp::Server;
use sim_core::state::Side;

// ---------------------------------------------------------------------
// The command.
// ---------------------------------------------------------------------

struct Options<'a> {
    scenario: &'a str,
    side: Side,
    /// Where the seat sits (decision 0138, 5): the area command by default.
    post: Post,
    seed: Option<u64>,
    save: Option<&'a str>,
    transcript: Option<&'a str>,
}

fn parse<'a>(args: &'a [String], usage: &str) -> Result<Options<'a>, String> {
    let mut scenario = None;
    let mut side = None;
    let mut post = Post::Area;
    let mut seed = None;
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
            "--seat" => {
                let v = value("--seat")?;
                side = Some(
                    parse_seat(v)
                        .ok_or_else(|| format!("--seat \"{v}\" is not allied or japanese"))?,
                );
            }
            "--at" => post = Post::parse(value("--at")?)?,
            "--seed" => {
                let v = value("--seed")?;
                seed = Some(
                    v.parse::<u64>()
                        .map_err(|_| format!("--seed \"{v}\" is not a whole number"))?,
                );
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
    Ok(Options {
        scenario: scenario.ok_or_else(|| format!("no scenario given\n\n{usage}"))?,
        side: side
            .ok_or_else(|| format!("--seat allied or --seat japanese is needed\n\n{usage}"))?,
        post,
        seed,
        save,
        transcript,
    })
}

pub fn command(args: &[String], usage: &str) -> Result<(), String> {
    use std::io::{BufRead, Write};
    let options = parse(args, usage)?;
    let seats = BTreeMap::from([(options.side, sim_agent::SeatKind::Player)]);
    let posts = BTreeMap::from([(options.side, options.post)]);
    let (session, kept) = sim_cli::game::open(options.scenario, options.seed, seats, &posts)?;
    let mut server = Server::new(session, options.side);
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    for line in stdin.lock().lines() {
        let line = line.map_err(|e| format!("standard input: {e}"))?;
        if line.trim().is_empty() {
            continue;
        }
        if let Some(answer) = server.handle_line(&line) {
            writeln!(out, "{answer}")
                .and_then(|()| out.flush())
                .map_err(|e| format!("standard output: {e}"))?;
        }
    }
    kept.finish(&server.session, options.save, options.transcript)
}
