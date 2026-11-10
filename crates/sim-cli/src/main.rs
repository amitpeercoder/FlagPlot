//! Flag Plot command-line runner.
//!
//! So far there is one command: `run`. The spec also lists `batch`, `replay`,
//! `hash` and `serve`. They arrive with the scenarios that need them.

use std::env;
use std::path::Path;
use std::process::ExitCode;

use sim_core::state::{ShipLocation, Side};
use sim_core::time::ClockTime;
use sim_data::Scenario;

const USAGE: &str = "\
Usage: sim-cli run <scenario.toml> [--events] [--seed N]

  run       Runs a scenario to its last day and prints the state hash of
            each day, then where every task force and ship ended up.
  --events  Also prints the full event log, with what caused each event.
  --seed N  Runs with seed N instead of the scenario's own seed.";

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
    let mut seed: Option<u64> = None;

    let mut words = args.iter().map(String::as_str);
    match words.next() {
        Some("run") => {}
        Some("--help" | "-h") | None => {
            println!("{USAGE}");
            return Ok(());
        }
        Some(other) => return Err(format!("unknown command \"{other}\"\n\n{USAGE}")),
    }
    while let Some(word) = words.next() {
        match word {
            "--events" => show_events = true,
            "--seed" => {
                let value = words
                    .next()
                    .ok_or_else(|| format!("--seed needs a number\n\n{USAGE}"))?;
                seed = Some(
                    value
                        .parse::<u64>()
                        .map_err(|_| format!("--seed \"{value}\" is not a whole number"))?,
                );
            }
            flag if flag.starts_with('-') => {
                return Err(format!("unknown option \"{flag}\"\n\n{USAGE}"))
            }
            path if scenario_path.is_none() => scenario_path = Some(path),
            extra => return Err(format!("unexpected argument \"{extra}\"\n\n{USAGE}")),
        }
    }
    let scenario_path = scenario_path.ok_or_else(|| format!("no scenario given\n\n{USAGE}"))?;

    let mut scenario = Scenario::load(Path::new(scenario_path)).map_err(|e| e.to_string())?;
    if let Some(seed) = seed {
        scenario = scenario.with_seed(seed).map_err(|e| e.to_string())?;
    }
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
        let position = match task_force.toward_hex {
            Some(toward) => format!(
                "{}, {} nm from {} toward {}",
                task_force.position(),
                task_force.progress_nm,
                task_force.from_hex,
                toward
            ),
            None => task_force.position().to_string(),
        };
        let noticed = match task_force.noticed_sighted_at_hours {
            Some(hours) => format!(
                ", last noticed sighted at {}",
                ClockTime::from_elapsed(hours)
            ),
            None => String::new(),
        };
        println!(
            "{} \"{}\" ({}): {:?} at {}{}",
            task_force.id,
            task_force.name,
            task_force.side.name(),
            task_force.status,
            position,
            noticed
        );
    }
    for group in truth.air_groups.values() {
        let arc = match group.search {
            Some(arc) => format!(
                "arc {:03}-{:03} to {} nm",
                arc.from_deg, arc.to_deg, arc.range_nm
            ),
            None => "no arc".to_string(),
        };
        println!(
            "{} \"{}\" ({}) at {}: {} ready, mission {:?}, {}",
            group.id,
            group.name,
            group.side.name(),
            group.base,
            group.ready,
            group.mission,
            arc
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
    for side in [Side::Allied, Side::Japanese] {
        let view = world.side_view(side);
        let mut any = false;
        for contact in view.contacts() {
            any = true;
            println!(
                "{} {} at {}: {} ships, {}, age {} pulses, confidence {}",
                side.name(),
                contact.id,
                contact.estimated_hex,
                contact.estimated_ship_count,
                contact.estimated_kind.name(),
                contact.age_pulses,
                contact.confidence
            );
        }
        if !any {
            println!("{}: no contacts", side.name());
        }
    }
    Ok(())
}
