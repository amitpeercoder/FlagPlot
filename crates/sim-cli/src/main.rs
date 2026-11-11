//! Flag Plot command-line runner.
//!
//! Commands: `run`, `export`, `batch`, `replay` and `hash`. The spec also
//! lists `serve` and `query`. They arrive with the steps that need them.

mod batch;
mod export;
mod save;

use std::env;
use std::path::Path;
use std::process::ExitCode;

use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::orders::{AirMission, Stance};
use sim_core::state::{AirGroupLocation, ShipLocation, ShipSystem, Side, SystemState, TruthState};
use sim_core::DaySummary;
use sim_data::Scenario;

const USAGE: &str = "\
Usage: sim-cli run <scenario.toml> [--events] [--seed N] [--save <game.json>]
       sim-cli replay <game.json>
       sim-cli hash <scenario.toml> [--seed N]
       sim-cli export <scenario.toml> --out <file.json> [--seed N] [--labels <anchors.csv>]
       sim-cli batch <scenario.toml> --out <file.json> [--runs N] [--first-seed S]

  run       Runs a scenario to its last day and prints the state hash of
            each day, then where every task force and ship ended up.
  replay    Runs a saved game: loads the scenario the file names, refuses it if
            the scenario file or any data file it names has changed, submits
            the saved orders day by day in place of the scenario's own, prints
            each day's hash and compares it with the saved one. Exits with an
            error on the first day that differs.
  hash      Runs a scenario and prints the state hash of each day, one a line
            as `day N <hash>`, and nothing else.
  export    Runs a scenario the same way and writes the finished run as one
            JSON replay file: the map, the starting forces, every event with
            its causes, and the final state.
  batch     Runs a scenario N times with seeds S, S+1, ... one after another
            and writes one JSON summary file: ships sunk per side (a
            histogram and totals), per ship, and the final hash and sunk
            ships of every run. Prints a short summary.
  --events  (run) Also prints the full event log, with what caused each event.
  --seed N  Runs with seed N instead of the scenario's own seed.
  --save F  (run) Also writes a saved game: the scenario's file and its hash
            and those of its data files, the seed, the orders of each day with
            their origin, and the daily hashes. It holds no state.
  --out F   (export, batch) The file to write.
  --runs N  (batch) Number of runs, at least 1. Default 1000.
  --first-seed S  (batch) Seed of the first run. Default 1.
  --labels F  (export) An anchors CSV (place, feature, lat, lon) to put place
            names on the replay's map.";

/// A scenario that has been loaded, given its seed and run to its last day.
/// Both commands build one with [`execute`].
struct Executed {
    scenario: Scenario,
    summaries: Vec<DaySummary>,
    /// The truth as it stood after loading and seeding, before day 1.
    initial: TruthState,
}

/// The one way a scenario is run: load it, apply the seed if one is given,
/// call `before_run` (the runner prints its header there, so a run that fails
/// partway still shows which scenario and seed it was), note the starting
/// truth, run every day.
fn execute(
    path: &str,
    seed: Option<u64>,
    before_run: impl FnOnce(&Scenario),
) -> Result<Executed, String> {
    let mut scenario = Scenario::load(Path::new(path)).map_err(|e| e.to_string())?;
    if let Some(seed) = seed {
        scenario = scenario.with_seed(seed).map_err(|e| e.to_string())?;
    }
    before_run(&scenario);
    let initial = scenario.world.umpire().clone();
    let summaries = scenario.run_to_end().map_err(|e| e.to_string())?;
    Ok(Executed {
        scenario,
        summaries,
        initial,
    })
}

/// Reads the number after `--seed`.
fn parse_seed(value: Option<&str>) -> Result<u64, String> {
    let value = value.ok_or_else(|| format!("--seed needs a number\n\n{USAGE}"))?;
    value
        .parse::<u64>()
        .map_err(|_| format!("--seed \"{value}\" is not a whole number"))
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        Some("export") => export::command(&args[1..], USAGE),
        Some("batch") => batch::command(&args[1..], USAGE),
        Some("replay") => replay(&args[1..]),
        Some("hash") => hash(&args[1..]),
        _ => run(&args),
    };
    match result {
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
    let mut save_path: Option<&str> = None;

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
            "--seed" => seed = Some(parse_seed(words.next())?),
            "--save" => {
                save_path = Some(
                    words
                        .next()
                        .ok_or_else(|| format!("--save needs a file\n\n{USAGE}"))?,
                )
            }
            flag if flag.starts_with('-') => {
                return Err(format!("unknown option \"{flag}\"\n\n{USAGE}"))
            }
            path if scenario_path.is_none() => scenario_path = Some(path),
            extra => return Err(format!("unexpected argument \"{extra}\"\n\n{USAGE}")),
        }
    }
    let scenario_path = scenario_path.ok_or_else(|| format!("no scenario given\n\n{USAGE}"))?;

    let mut remembered: Option<Result<save::Remembered, String>> = None;
    let Executed {
        scenario,
        summaries,
        initial: _,
    } = execute(scenario_path, seed, |scenario| {
        if save_path.is_some() {
            remembered = Some(save::remember(scenario));
        }
        println!("{}", scenario.name);
        if scenario.placeholder {
            println!("PLACEHOLDER DATA: some or all of the units, figures and geography are invented, not historical.");
        }
        println!("seed {}", scenario.world.seed());
        println!();
    })?;

    for summary in &summaries {
        println!(
            "day {:>3}  hash {:016x}  {} events",
            summary.day, summary.state_hash, summary.event_count
        );
    }

    if let (Some(file), Some(remembered)) = (save_path, remembered) {
        save::write(
            file,
            scenario_path,
            scenario.world.seed(),
            &remembered?,
            &summaries,
        )?;
    }

    let world = &scenario.world;
    if show_events {
        println!();
        for event in world.events().iter() {
            // An order the simulation issued itself is marked, so the log
            // shows who gave it (decision 0016, 14).
            let by_staff = match &event.kind {
                sim_core::events::EventKind::OrderAccepted { order, .. }
                | sim_core::events::EventKind::OrderRejected { order, .. } => world
                    .order(*order)
                    .is_some_and(|o| o.origin.issuer == sim_core::orders::Issuer::Staff),
                _ => false,
            };
            if by_staff {
                println!("{event}  [issued by the staff]");
            } else {
                println!("{event}");
            }
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
        // "Alert" is what the rules read. A force is alert by its readiness
        // or because it is alerted (decision 0021). Saying which avoids a
        // line that reads "battle readiness, not alerted".
        let readiness = format!(
            "readiness {}, {}",
            task_force.readiness.name(),
            match (task_force.is_alert(), task_force.alerted) {
                (true, true) if task_force.readiness.name() == "normal" => "alert (alerted)",
                (true, _) => "alert",
                (false, _) => "not alert",
            }
        );
        // Fatigue is shown only when there is some (decision 0037).
        let readiness = if task_force.fatigue != Fixed::ZERO {
            format!("{readiness}, fatigue {}", task_force.fatigue)
        } else {
            readiness
        };
        let stance = match task_force.rules_of_engagement.stance {
            Stance::Engage => "engage",
            Stance::Avoid => "avoid",
        };
        println!(
            "{} \"{}\" ({}): {:?} at {}; {}, rules of engagement: {}, break off at {}, stragglers: {}, keep {}; {}",
            task_force.id,
            task_force.name,
            task_force.side.name(),
            task_force.status,
            position,
            readiness,
            stance,
            task_force.rules_of_engagement.break_off,
            task_force.rules_of_engagement.stragglers.name(),
            task_force.rules_of_engagement.keep_share,
            match task_force.home_port {
                Some(port) => format!("home port {port}"),
                None => "no home port".to_string(),
            }
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
        let share = if group.mission == AirMission::CombatAirPatrol {
            format!(", patrol share {}", group.patrol_share)
        } else {
            String::new()
        };
        let mission = match group.mission {
            AirMission::BaseStrike(base) => format!("BaseStrike on {base}"),
            other => format!("{other:?}"),
        };
        // Damaged aircraft are shown only when there are some (decision 0035).
        let ready = if group.damaged > 0 {
            format!("{} ready, {} damaged", group.ready, group.damaged)
        } else {
            format!("{} ready", group.ready)
        };
        // Fatigue, the rest threshold and a stand-down are shown only when
        // there is something to show (decision 0036).
        let mut tired = String::new();
        if group.fatigue != Fixed::ZERO {
            tired.push_str(&format!(", fatigue {}", group.fatigue));
        }
        if let Some(level) = group.rest_above {
            tired.push_str(&format!(", rests above {level}"));
        }
        if group.stood_down {
            tired.push_str(", stood down");
        }
        // Pilots are shown only when the scenario tracks them (decision 0038).
        let ready = match group.pilots {
            Some(pilots) => format!("{ready}, {pilots} pilots"),
            None => ready,
        };
        println!(
            "{} \"{}\" ({}) {}: {}, mission {}{}{}, {}",
            group.id,
            group.name,
            group.side.name(),
            match group.location {
                AirGroupLocation::Base(base) => format!("at {base}"),
                AirGroupLocation::Ship(ship) => format!("aboard {ship}"),
            },
            ready,
            mission,
            share,
            tired,
            arc
        );
    }
    for ship in truth.ships.values() {
        let location = match ship.location {
            ShipLocation::TaskForce(id) => id.to_string(),
            ShipLocation::Port(id) => id.to_string(),
            ShipLocation::Sunk => "no task force or port".to_string(),
        };
        let state = if ship.location == ShipLocation::Sunk {
            "SUNK".to_string()
        } else {
            format!("afloat in {location}")
        };
        let class = &world.ref_data().ship_classes[&ship.class];
        let damaged: Vec<String> = ShipSystem::ALL
            .into_iter()
            .filter(|system| class.has_system(*system))
            .filter(|system| ship.systems.get(*system) != SystemState::Working)
            .map(|system| format!("{} {}", system.name(), ship.systems.get(system).name()))
            .collect();
        let systems = if damaged.is_empty() {
            "all systems working".to_string()
        } else {
            damaged.join(", ")
        };
        println!(
            "{} \"{}\": {state}; damage flotation {}, fire {}; {systems}; {} gun rounds, torpedoes {}; {} t of fuel, {} t of supply{}",
            ship.id,
            ship.name,
            ship.flotation_damage,
            ship.fire_damage,
            ship.gun_rounds,
            if ship.torpedoes_ready { "ready" } else { "gone" },
            ship.fuel_t,
            ship.supply_t,
            if ship.cargo_fuel_t > Fixed::ZERO {
                format!(", {} t of fuel cargo", ship.cargo_fuel_t)
            } else {
                String::new()
            }
        );
    }
    for base in truth.bases.values() {
        let guns = if base.anti_aircraft > 0 {
            format!(", {} anti-aircraft shots", base.anti_aircraft)
        } else {
            String::new()
        };
        let repairs = if base.aircraft_repairs_per_day > 0 {
            format!(", repairs {} aircraft a day", base.aircraft_repairs_per_day)
        } else {
            String::new()
        };
        // Fuel is shown only for a base that has ever been given some. A
        // stock only falls, so that is a stock above nothing now or one that
        // refuelled a ship (decision 0045).
        let had_fuel = base.fuel_t > Fixed::ZERO
            || world.events().iter().any(|event| {
                matches!(&event.kind, EventKind::ShipRefuelled { base: b, .. } if *b == base.id)
            });
        let fuel = if had_fuel {
            format!(", {} t of fuel in stock", base.fuel_t)
        } else {
            String::new()
        };
        println!(
            "{} \"{}\" ({}) at {}: {} t of supply in stock{}{}{}",
            base.id,
            base.name,
            base.side.name(),
            base.hex,
            base.supply_t,
            fuel,
            guns,
            repairs
        );
    }
    for side in [Side::Allied, Side::Japanese] {
        let view = world.side_view(side);
        let mut any = false;
        for contact in view.contacts() {
            any = true;
            println!(
                "{} {} at {}: {} ships, {}, age {} pulses",
                side.name(),
                contact.id,
                contact.estimated_hex,
                contact.estimated_ship_count,
                contact.estimated_kind.name(),
                contact.age_pulses
            );
        }
        if !any {
            println!("{}: no contacts", side.name());
        }
        for claim in view.claims() {
            println!(
                "{} claims {} enemy {} sunk at {}, day {} {}",
                side.name(),
                claim.claimed_sunk,
                if claim.claimed_sunk == 1 {
                    "ship"
                } else {
                    "ships"
                },
                claim.hex,
                claim.day,
                claim.pulse.name()
            );
        }
        if view.claims().is_empty() {
            println!("{}: no battle claims", side.name());
        }
    }
    Ok(())
}

/// `sim-cli hash <scenario> [--seed N]`: the daily hashes and nothing else.
fn hash(args: &[String]) -> Result<(), String> {
    let mut scenario_path: Option<&str> = None;
    let mut seed: Option<u64> = None;
    let mut words = args.iter().map(String::as_str);
    while let Some(word) = words.next() {
        match word {
            "--seed" => seed = Some(parse_seed(words.next())?),
            flag if flag.starts_with('-') => {
                return Err(format!("unknown option \"{flag}\"\n\n{USAGE}"))
            }
            path if scenario_path.is_none() => scenario_path = Some(path),
            extra => return Err(format!("unexpected argument \"{extra}\"\n\n{USAGE}")),
        }
    }
    let scenario_path = scenario_path.ok_or_else(|| format!("no scenario given\n\n{USAGE}"))?;
    let executed = execute(scenario_path, seed, |_| {})?;
    for summary in &executed.summaries {
        println!("day {} {:016x}", summary.day, summary.state_hash);
    }
    Ok(())
}

/// `sim-cli replay <game.json>`: runs a saved game and compares every day's
/// hash with the saved one.
fn replay(args: &[String]) -> Result<(), String> {
    let file = match args {
        [file] if !file.starts_with('-') => file.as_str(),
        _ => return Err(format!("replay needs one saved game\n\n{USAGE}")),
    };
    let saved = save::read(file)?;
    let scenario = Scenario::load(Path::new(&saved.scenario_file)).map_err(|e| e.to_string())?;
    saved.check_files(&scenario)?;
    let mut scenario = scenario.with_seed(saved.seed).map_err(|e| e.to_string())?;
    scenario.replace_script(saved.script.clone());
    println!("{}", scenario.name);
    println!("seed {}", saved.seed);
    for (day, hash) in &saved.hashes {
        let summary = scenario
            .next_day()
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("the scenario has no day {day}"))?;
        let now = format!("{:016x}", summary.state_hash);
        println!("day {} {now}", summary.day);
        if summary.day != *day || &now != hash {
            return Err(format!(
                "replay differs on day {day}: saved hash {hash}, replayed hash {now}"
            ));
        }
    }
    Ok(())
}
