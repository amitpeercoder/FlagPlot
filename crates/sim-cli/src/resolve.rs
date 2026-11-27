//! `sim-cli resolve FILE`: a scenario built on another, read whole
//! (decision 0234, item 12). It prints the resolved scenario as TOML on
//! standard output and the files read, the one asked for first, on standard
//! error, a path a line. It builds no world and reads no file but the chain.

use sim_data::{Disk, Scenario};
use std::path::Path;

pub fn command(args: &[String], usage: &str) -> Result<(), String> {
    let mut files: Vec<&str> = Vec::new();
    for word in args {
        match word.as_str() {
            flag if flag.starts_with('-') => {
                return Err(format!("unknown option \"{flag}\"\n\n{usage}"))
            }
            file => files.push(file),
        }
    }
    let [file] = files.as_slice() else {
        return Err(format!("resolve takes one scenario file\n\n{usage}"));
    };
    let path = Path::new(file);
    let text = Scenario::resolved_text(&Disk, path).map_err(|e| e.to_string())?;
    let chain = Scenario::built_on_chain(&Disk, path).map_err(|e| e.to_string())?;
    print!("{text}");
    for read in chain {
        eprintln!("{}", read.display());
    }
    Ok(())
}
