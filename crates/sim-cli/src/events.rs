//! `sim-cli events`: the umpire's questions to the event log of a finished
//! run (decision 0052, item 9). "What happened to this unit" lists every
//! event that names it; "why" walks one event's causes down to orders. Like
//! `run`, it reads the truth. Nothing that plays the game may use it.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use sim_core::events::{Cause, Event, EventKind};
use sim_core::ids::{AirGroupId, BaseId, ContactId, EntityRef, ShipId, TaskForceId};
use sim_core::state::Side;
use sim_core::World;
use sim_data::Scenario;

use crate::{event_line, execute, parse_seed, save, Executed};

const EVENTS_USAGE: &str = "\
Usage: sim-cli events <scenario.toml | game.json> --unit HANDLE [--day N] [--seed N]
       sim-cli events <scenario.toml | game.json> --why EVENT [--seed N]
  HANDLE is ship:N, task_force:N, air_group:N, base:N or contact:N. Contact
  numbers are kept by each side, so a contact may be written allied:contact:N
  or japanese:contact:N; without a side it lists both sides' contact N.
  EVENT is an event number, 42 or #42.";

/// What the user asked about.
enum Subject {
    Unit {
        unit: EntityRef,
        /// Only for a contact: the side whose contact it is.
        side: Option<Side>,
    },
    Why(u32),
}

pub fn command(args: &[String], options: &sim_data::LoadOptions) -> Result<(), String> {
    let usage = EVENTS_USAGE;
    let mut source: Option<&str> = None;
    let mut unit: Option<&str> = None;
    let mut why: Option<&str> = None;
    let mut day: Option<u32> = None;
    let mut seed: Option<u64> = None;
    let mut words = args.iter().map(String::as_str);
    while let Some(word) = words.next() {
        match word {
            "--unit" | "--why" | "--day" | "--seed" => {
                let value = words
                    .next()
                    .ok_or_else(|| format!("{word} needs a value\n\n{usage}"))?;
                match word {
                    "--unit" => unit = Some(value),
                    "--why" => why = Some(value),
                    "--day" => {
                        day = Some(
                            value
                                .parse::<u32>()
                                .map_err(|_| format!("--day \"{value}\" is not a whole number"))?,
                        )
                    }
                    _ => seed = Some(parse_seed(Some(value))?),
                }
            }
            flag if flag.starts_with('-') => {
                return Err(format!("unknown option \"{flag}\"\n\n{usage}"))
            }
            path if source.is_none() => source = Some(path),
            extra => return Err(format!("unexpected argument \"{extra}\"\n\n{usage}")),
        }
    }
    let source = source.ok_or_else(|| format!("no scenario or saved game given\n\n{usage}"))?;
    let subject = match (unit, why) {
        (Some(_), Some(_)) | (None, None) => {
            return Err(format!("give exactly one of --unit and --why\n\n{usage}"))
        }
        (Some(text), None) => {
            let (unit, side) = parse_handle(text)?;
            Subject::Unit { unit, side }
        }
        (None, Some(text)) => {
            if day.is_some() {
                return Err(format!("--day goes with --unit only\n\n{usage}"));
            }
            Subject::Why(parse_event(text)?)
        }
    };

    let executed = run_source(source, seed, options)?;
    let world = &executed.scenario.world;
    let asked = match &subject {
        Subject::Unit { unit, side } => format!(
            "events of {}{}{}",
            side.map_or(String::new(), |s| format!("{} ", s.name())),
            unit,
            day.map_or(String::new(), |d| format!(" on day {d}"))
        ),
        Subject::Why(n) => format!("why event #{n}"),
    };
    let header = format!("{}, seed {}: {asked}", executed.scenario.name, world.seed());
    match subject {
        Subject::Unit { unit, side } => list_unit(&executed, unit, side, day, &header),
        Subject::Why(n) => why_tree(world, n, &header),
    }
}

/// Runs the scenario file, or replays the saved game, to its end.
fn run_source(
    source: &str,
    seed: Option<u64>,
    options: &sim_data::LoadOptions,
) -> Result<Executed, String> {
    let text = fs::read_to_string(source).map_err(|e| format!("{source}: {e}"))?;
    if !text.trim_start().starts_with('{') {
        return execute(source, seed, options, |_| {});
    }
    // A saved game, read as `replay` reads it.
    if seed.is_some() {
        return Err("--seed does not go with a saved game: it holds its own seed".to_string());
    }
    let saved = save::read(source)?;
    let scenario =
        Scenario::load_with(Path::new(&saved.scenario_file), options).map_err(|e| e.to_string())?;
    saved.check_files(&scenario)?;
    let mut scenario = scenario.with_seed(saved.seed).map_err(|e| e.to_string())?;
    scenario.replace_script(saved.script.clone());
    let initial = scenario.world.umpire().clone();
    let summaries = scenario.run_to_end().map_err(|e| e.to_string())?;
    for ((day, hash), summary) in saved.hashes.iter().zip(&summaries) {
        let now = format!("{:016x}", summary.state_hash);
        if summary.day != *day || &now != hash {
            return Err(format!(
                "replay differs on day {day}: saved hash {hash}, replayed hash {now}"
            ));
        }
    }
    Ok(Executed {
        scenario,
        summaries,
        initial,
    })
}

fn parse_number(text: &str) -> Option<u32> {
    text.parse::<u32>().ok()
}

/// `ship:33`, `allied:contact:3`.
fn parse_handle(text: &str) -> Result<(EntityRef, Option<Side>), String> {
    let lower = text.to_ascii_lowercase();
    let (side, rest) = if let Some(rest) = lower.strip_prefix("allied:") {
        (Some(Side::Allied), rest)
    } else if let Some(rest) = lower.strip_prefix("japanese:") {
        (Some(Side::Japanese), rest)
    } else {
        (None, lower.as_str())
    };
    let bad = || {
        format!(
            "\"{text}\" is not a unit handle: write ship:N, task_force:N, air_group:N, base:N or contact:N"
        )
    };
    let (kind, number) = rest.split_once(':').ok_or_else(bad)?;
    let n = parse_number(number).ok_or_else(bad)?;
    let unit = match kind {
        "ship" => EntityRef::Ship(ShipId::new(n)),
        "task_force" => EntityRef::TaskForce(TaskForceId::new(n)),
        "air_group" => EntityRef::AirGroup(AirGroupId::new(n)),
        "base" => EntityRef::Base(BaseId::new(n)),
        "contact" => EntityRef::Contact(ContactId::new(n)),
        _ => return Err(bad()),
    };
    if side.is_some() && !matches!(unit, EntityRef::Contact(_)) {
        return Err(format!(
            "\"{text}\": a side goes only before a contact (allied:contact:N)"
        ));
    }
    Ok((unit, side))
}

/// `42`, `#42`, `event:42`.
fn parse_event(text: &str) -> Result<u32, String> {
    let digits = text
        .strip_prefix('#')
        .or_else(|| text.strip_prefix("event:"))
        .unwrap_or(text);
    parse_number(digits).ok_or_else(|| format!("\"{text}\" is not an event number"))
}

/// The side an event concerns, for the events that name a contact: a
/// contact's number means something only inside one side's belief.
fn contact_side(world: &World, event: &Event) -> Option<Side> {
    match &event.kind {
        EventKind::ReportReceived { side, .. }
        | EventKind::ContactAged { side, .. }
        | EventKind::ContactDropped { side, .. }
        | EventKind::ContactNotThere { side, .. } => return Some(*side),
        _ => {}
    }
    let truth = world.umpire();
    event.kind.entities().into_iter().find_map(|e| match e {
        EntityRef::TaskForce(id) => truth.task_forces.get(&id).map(|t| t.side),
        EntityRef::AirGroup(id) => truth.air_groups.get(&id).map(|g| g.side),
        EntityRef::Ship(id) => truth.ships.get(&id).map(|s| s.side),
        _ => None,
    })
}

fn list_unit(
    executed: &Executed,
    unit: EntityRef,
    side: Option<Side>,
    day: Option<u32>,
    header: &str,
) -> Result<(), String> {
    let world = &executed.scenario.world;
    let truth = world.umpire();
    let names_unit = |event: &Event| {
        event.kind.entities().contains(&unit)
            && (side.is_none() || contact_side(world, event) == side)
    };
    let known = match unit {
        EntityRef::Ship(id) => truth.ships.contains_key(&id),
        EntityRef::TaskForce(id) => truth.task_forces.contains_key(&id),
        EntityRef::AirGroup(id) => truth.air_groups.contains_key(&id),
        EntityRef::Base(id) => truth.bases.contains_key(&id),
        _ => false,
    } || world.events().iter().any(names_unit);
    if !known {
        return Err(format!(
            "no {unit} in this run{}",
            side.map_or(String::new(), |s| format!(" for the {} side", s.name()))
        ));
    }

    println!("{header}");
    // For a ship: the task force it is in, event by event. A ship starts in
    // the force the scenario put it in and changes force only when ships are
    // detached; a sunk ship has none.
    let mut membership: BTreeMap<ShipId, TaskForceId> = BTreeMap::new();
    for force in executed.initial.task_forces.values() {
        for ship in &force.ships {
            membership.insert(*ship, force.id);
        }
    }

    let mut count = 0usize;
    for event in world.events().iter() {
        let direct = names_unit(event);
        let via = match unit {
            EntityRef::Ship(ship) if !direct => membership.get(&ship).copied().filter(|force| {
                event
                    .kind
                    .entities()
                    .contains(&EntityRef::TaskForce(*force))
            }),
            _ => None,
        };
        if (direct || via.is_some()) && day.is_none_or(|d| d == event.day) {
            count += 1;
            match via {
                Some(force) => println!("via {force}  {}", event_line(world, event)),
                None => println!("{}", event_line(world, event)),
            }
        }
        match &event.kind {
            EventKind::ShipsDetached {
                new_force,
                ships,
                escorts,
                ..
            } => {
                for ship in ships.iter().chain(escorts) {
                    membership.insert(*ship, *new_force);
                }
            }
            EventKind::ShipSunk { ship, .. } => {
                membership.remove(ship);
            }
            _ => {}
        }
    }
    // The reconstruction is checked against the truth at the end.
    if let EntityRef::Ship(ship) = unit {
        let actual = truth
            .task_forces
            .values()
            .find_map(|f| f.ships.contains(&ship).then_some(f.id));
        if actual != membership.get(&ship).copied() {
            println!(
                "note: the task force of {ship} over time could not be rebuilt exactly from the events; lines marked via may be missing"
            );
        }
    }
    if count == 0 {
        println!(
            "{unit} is in this run, but no event{} names it",
            if day.is_some() { " on that day" } else { "" }
        );
    }
    println!("{count} events");
    Ok(())
}

fn why_tree(world: &World, number: u32, header: &str) -> Result<(), String> {
    let find = |n: u32| -> Option<&Event> {
        usize::try_from(n)
            .ok()
            .and_then(|n| n.checked_sub(1))
            .and_then(|i| world.events().iter().nth(i))
    };
    if find(number).is_none() {
        return Err(format!(
            "no event #{number} in this run (it has {} events)",
            world.events().len()
        ));
    }
    println!("{header}");
    let mut shown: BTreeSet<u32> = BTreeSet::new();
    print_event(world, &find, number, 0, "", &mut shown);
    Ok(())
}

fn print_event<'a>(
    world: &'a World,
    find: &dyn Fn(u32) -> Option<&'a Event>,
    number: u32,
    depth: usize,
    lead: &str,
    shown: &mut BTreeSet<u32>,
) {
    let indent = "    ".repeat(depth);
    let Some(event) = find(number) else {
        println!("{indent}{lead}#{number} (not in the log)");
        return;
    };
    if !shown.insert(number) {
        println!("{indent}{lead}#{number} (shown above)");
        return;
    }
    println!("{indent}{lead}{}", event_line(world, event));
    if let Some(breakdown) = &event.breakdown {
        let factors: Vec<String> = breakdown
            .factors
            .iter()
            .map(|f| format!("{} {}", f.name, f.value))
            .collect();
        println!(
            "{indent}{}breakdown: {} = {}",
            " ".repeat(lead.len()),
            factors.join(", "),
            breakdown.result
        );
    }
    for cause in &event.causes {
        match cause {
            Cause::Event(id) => print_event(world, find, id.raw(), depth + 1, "<- ", shown),
            Cause::Order(id) => match world.order(*id) {
                Some(order) => println!(
                    "{indent}    <- {id} ({}, day {}, issued by {:?}{}): {:?}",
                    order.side.name(),
                    order.day_issued,
                    order.origin.issuer,
                    order
                        .origin
                        .intent
                        .map_or(String::new(), |i| format!(" for {i}")),
                    order.kind
                ),
                None => println!("{indent}    <- {id} (not in the log)"),
            },
        }
    }
}
