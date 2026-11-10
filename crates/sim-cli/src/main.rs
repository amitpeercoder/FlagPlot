//! Flag Plot command-line runner.
//!
//! So far there is one command: `run`. The spec also lists `batch`, `replay`,
//! `hash` and `serve`. They arrive with the scenarios that need them.

use std::env;
use std::path::Path;
use std::process::ExitCode;

use sim_core::state::ShipLocation;
use sim_data::Scenario;

const USAGE: &str = "\
Usage: sim-cli run <scenario.toml> [--events]

  run       Runs a scenario to its last day and prints the state hash of
            each day, then where every task force and ship ended up.
  --events  Also prints the full event log, with what caused each event.";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let mut scenario_path: Option<&str> = None;
    let mut show_events = false;

    let mut words = args.iter().map(String::as_str);
    match words.next() {
        Some("run") => {}
        Some("--help" | "-h") | None => {
            println!("{USAGE}");
            return Ok(());
        }
        Some(other) => return Err(format!("unknown command \"{other}\"\n\n{USAGE}")),
    }
    for word in words {
        match word {
            "--events" => show_events = true,
            flag if flag.starts_with('-') => {
                return Err(format!("unknown option \"{flag}\"\n\n{USAGE}"))
            }
            path if scenario_path.is_none() => scenario_path = Some(path),
            extra => return Err(format!("unexpected argument \"{extra}\"\n\n{USAGE}")),
        }
    }
    let scenario_path = scenario_path.ok_or_else(|| format!("no scenario given\n\n{USAGE}"))?;

    let mut scenario = Scenario::load(Path::new(scenario_path)).map_err(|e| e.to_string())?;
    println!("{}", scenario.name);
    if scenario.placeholder {
        println!("PLACEHOLDER DATA: units and geography are invented, not historical.");
    }
    println!("seed {}", scenario.world.seed());
    println!();

    let summaries = scenario.run_to_end().map_err(|e| e.to_string())?;
    for summary in &summaries {
        println!(
            "day {:>3}  hash {:016x}  {} events",
            summary.day, summary.state_hash, summary.event_count
        );
    }

    let world = &scenario.world;
    if show_events {
        println!();
        for event in world.events().iter() {
            println!("{event}");
        }
    }

    // The runner is the umpire: it is allowed to print the truth.
    let truth = world.umpire();
    println!();
    for task_force in truth.task_forces.values() {
        println!(
            "{} \"{}\" ({}): {:?} at {}, {} nm toward the next hex",
            task_force.id,
            task_force.name,
            task_force.side.name(),
            task_force.status,
            task_force.hex,
            task_force.progress_nm
        );
    }
    for ship in truth.ships.values() {
        let location = match ship.location {
            ShipLocation::TaskForce(id) => id.to_string(),
            ShipLocation::Port(id) => id.to_string(),
        };
        println!(
            "{} \"{}\": {} t of fuel, {} t of supply, in {}",
            ship.id, ship.name, ship.fuel_t, ship.supply_t, location
        );
    }
    for base in truth.bases.values() {
        println!(
            "{} \"{}\" ({}) at {}: {} t of supply in stock",
            base.id,
            base.name,
            base.side.name(),
            base.hex,
            base.supply_t
        );
    }
    Ok(())
}
