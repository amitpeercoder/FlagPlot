//! `oob-derive <list.toml> [--facts <dir>] [--check]`: derives the ship
//! classes a scenario loads from the facts in `data/oob`.

use std::path::Path;
use std::process::ExitCode;

const USAGE: &str = "usage: oob-derive <list.toml> [--facts <dir>] [--check]";

fn main() -> ExitCode {
    let mut list = None;
    let mut facts = None;
    let mut check = false;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--check" => check = true,
            "--facts" => match args.next() {
                Some(dir) if facts.is_none() => facts = Some(dir),
                _ => return usage(),
            },
            flag if flag.starts_with("--") => return usage(),
            _ if list.is_none() => list = Some(arg),
            _ => return usage(),
        }
    }
    let Some(list) = list else {
        return usage();
    };
    let facts = facts.unwrap_or_else(|| "data/oob".to_string());
    let said = oob_derive::said(
        &list,
        oob_derive::run(Path::new(&list), Path::new(&facts), check),
    );
    print!("{}", said.out);
    eprint!("{}", said.err);
    if said.passed {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn usage() -> ExitCode {
    eprintln!("{USAGE}");
    ExitCode::from(2)
}
