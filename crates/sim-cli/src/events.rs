//! `sim-cli events`: the umpire's questions to the event log of a finished
//! run (decision 0052, item 9). "What happened to this unit" lists every
//! event that names it; "why" walks one event's causes down to orders. Like
//! `run`, it reads the truth. Nothing that plays the game may use it.

use std::collections::{BTreeMap, BTreeSet};

use sim_core::choices::{Moment, PulsePhase};
use sim_core::events::{Cause, Event, EventKind};
use sim_core::ids::{
    AirGroupId, BaseId, ContactId, EntityRef, LandUnitId, ShipId, TaskForceId, UmpireKey,
};
use sim_core::pictures::CommandRef;
use sim_core::state::Side;
use sim_core::time::Pulse;
use sim_core::view::strike_review::StrikeReview;
use sim_core::World;

use crate::{event_line, execute_source, execute_source_watching, parse_seed, Executed};

mod report_road;
mod strike_why;

const EVENTS_USAGE: &str = "\
Usage: sim-cli events <scenario.toml | game.json> --unit HANDLE [--day N] [--seed N]
       sim-cli events <scenario.toml | game.json> --why EVENT [--seed N]
       sim-cli events <scenario.toml | game.json> --report EVENT [--command COMMAND] [--seed N] [--json]
       sim-cli events <scenario.toml | game.json> --strike air_group:N --day D --pulse night|morning|afternoon
                      [--report EVENT] [--seed N] [--json]
  HANDLE is ship:N, task_force:N, air_group:N, base:N, land_unit:N or contact:N. Contact
  numbers are kept by each side, so a contact may be written allied:contact:N
  or japanese:contact:N; without a side it lists both sides' contact N.
  EVENT is an event number, 42 or #42. With --report it is the event a report is known by
  (a sighting's, a claim's, a ground count's, a \"nothing there\"'s); the answer says when each
  command of the report's side first held it and by which road, or that it never did.
  COMMAND is area, task_force:N or base:N. --strike asks why a naval strike group did or did
  not launch in that pulse, from the picture of its command at the moment of the decision.";

/// What the user asked about.
enum Subject {
    Unit {
        unit: Handle,
        /// Only for a contact: the side whose contact it is.
        side: Option<Side>,
    },
    Why(u32),
    /// When a report reached a command (decision 0138, item 1).
    Report {
        event: u32,
        command: Option<CommandRef>,
    },
    /// Why a group did not launch (decision 0138, item 2).
    Strike {
        group: u32,
        day: u32,
        pulse: Pulse,
        report: Option<u32>,
    },
}

/// A unit as the user wrote it. A contact's number is read through the
/// umpire's key, which only a world gives, so the handle becomes an
/// `EntityRef` once the run is done.
#[derive(Clone, Copy)]
enum Handle {
    Ship(u32),
    TaskForce(u32),
    AirGroup(u32),
    Base(u32),
    LandUnit(u32),
    Contact(u32),
}

impl Handle {
    fn entity(self, key: &UmpireKey) -> EntityRef {
        match self {
            Handle::Ship(n) => EntityRef::Ship(ShipId::new(n)),
            Handle::TaskForce(n) => EntityRef::TaskForce(TaskForceId::new(n)),
            Handle::AirGroup(n) => EntityRef::AirGroup(AirGroupId::new(n)),
            Handle::Base(n) => EntityRef::Base(BaseId::new(n)),
            Handle::LandUnit(n) => EntityRef::LandUnit(LandUnitId::new(n)),
            Handle::Contact(n) => EntityRef::Contact(ContactId::for_umpire(n, key)),
        }
    }
}

pub fn command(args: &[String], options: &sim_data::LoadOptions) -> Result<(), String> {
    let usage = EVENTS_USAGE;
    let mut source: Option<&str> = None;
    let mut unit: Option<&str> = None;
    let mut why: Option<&str> = None;
    let mut report: Option<&str> = None;
    let mut command_text: Option<&str> = None;
    let mut strike: Option<&str> = None;
    let mut pulse: Option<&str> = None;
    let mut json = false;
    let mut day: Option<u32> = None;
    let mut seed: Option<u64> = None;
    let mut words = args.iter().map(String::as_str);
    while let Some(word) = words.next() {
        match word {
            "--json" => json = true,
            "--unit" | "--why" | "--day" | "--seed" | "--report" | "--command" | "--strike"
            | "--pulse" => {
                let value = words
                    .next()
                    .ok_or_else(|| format!("{word} needs a value\n\n{usage}"))?;
                match word {
                    "--unit" => unit = Some(value),
                    "--why" => why = Some(value),
                    "--report" => report = Some(value),
                    "--command" => command_text = Some(value),
                    "--strike" => strike = Some(value),
                    "--pulse" => pulse = Some(value),
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
    let subject = match (unit, why, strike, report) {
        (Some(text), None, None, None) => {
            if command_text.is_some() || pulse.is_some() || json {
                return Err(format!(
                    "--command, --pulse and --json do not go with --unit\n\n{usage}"
                ));
            }
            let (unit, side) = parse_handle(text)?;
            Subject::Unit { unit, side }
        }
        (None, Some(text), None, None) => {
            if day.is_some() {
                return Err(format!(
                    "--day goes with --unit and --strike only\n\n{usage}"
                ));
            }
            if command_text.is_some() || pulse.is_some() || json {
                return Err(format!(
                    "--command, --pulse and --json do not go with --why\n\n{usage}"
                ));
            }
            Subject::Why(parse_event(text)?)
        }
        (None, None, None, Some(text)) => {
            if day.is_some() || pulse.is_some() {
                return Err(format!(
                    "--day and --pulse go with --strike, not with --report alone\n\n{usage}"
                ));
            }
            Subject::Report {
                event: parse_event(text)?,
                command: command_text.map(parse_command).transpose()?,
            }
        }
        (None, None, Some(text), report) => {
            if command_text.is_some() {
                return Err(format!(
                    "--command does not go with --strike: the command is the group's\n\n{usage}"
                ));
            }
            let group = match parse_handle(text)? {
                (Handle::AirGroup(n), None) => n,
                _ => {
                    return Err(format!(
                        "--strike takes an air group, air_group:N\n\n{usage}"
                    ))
                }
            };
            let day = day.ok_or_else(|| format!("--strike needs --day\n\n{usage}"))?;
            let pulse =
                parse_pulse(pulse.ok_or_else(|| format!("--strike needs --pulse\n\n{usage}"))?)?;
            Subject::Strike {
                group,
                day,
                pulse,
                report: report.map(parse_event).transpose()?,
            }
        }
        _ => {
            return Err(format!(
                "give exactly one of --unit, --why, --report and --strike\n\n{usage}"
            ))
        }
    };

    // What the questions that need the state, and not the log alone, watch
    // for while the game runs (decision 0138): the first moment each command
    // holds a report, the review of a strike decision, and which command
    // decided each launch and alert.
    let mut watch = Watched::default();
    match &subject {
        Subject::Report { event, .. } => watch.reports = Some(BTreeSet::from([*event])),
        Subject::Strike {
            group,
            day,
            pulse,
            report,
        } => {
            watch.reports = report.map(|n| BTreeSet::from([n]));
            watch.strike = Some((AirGroupId::new(*group), *day, *pulse));
        }
        Subject::Why(_) => watch.record_deciders = true,
        Subject::Unit { .. } => {}
    }
    let (executed, _) = if matches!(subject, Subject::Unit { .. }) {
        execute_source(source, seed, options)?
    } else {
        execute_source_watching(source, seed, options, &mut |world, moment| {
            watch.at(world, moment)
        })?
    };
    let world = &executed.scenario.world;
    let key = world.umpire_key();
    let asked = match &subject {
        Subject::Unit { unit, side } => format!(
            "events of {}{}{}",
            side.map_or(String::new(), |s| format!("{} ", s.name())),
            unit.entity(&key),
            day.map_or(String::new(), |d| format!(" on day {d}"))
        ),
        Subject::Why(n) => format!("why event #{n}"),
        Subject::Report { event, command } => format!(
            "when report #{event} reached {}",
            command.map_or("each command".to_string(), |c| c.to_string())
        ),
        Subject::Strike {
            group, day, pulse, ..
        } => format!(
            "why air_group:{group} did or did not launch in the {} of day {day}",
            pulse.name()
        ),
    };
    let header = format!("{}, seed {}: {asked}", executed.scenario.name, world.seed());
    match subject {
        Subject::Unit { unit, side } => list_unit(&executed, unit.entity(&key), side, day, &header),
        Subject::Why(n) => why_tree(world, n, &header, &watch),
        Subject::Report { event, command } => {
            report_road_answer(world, event, command, &watch, &header, json)
        }
        Subject::Strike {
            group,
            day,
            pulse,
            report,
        } => strike_why::answer(
            world,
            &watch,
            strike_why::Asked {
                group: AirGroupId::new(group),
                day,
                pulse,
                report,
            },
            &header,
            json,
        ),
    }
}

/// What the run is watched for: the state at the moments the log does not
/// keep (decision 0138). The umpire's: it reads the truth and the pictures of
/// the commands of both sides.
#[derive(Default)]
pub(crate) struct Watched {
    /// The reports asked about, by event number (all of them in a test): the
    /// first moment each command's heard set holds each.
    reports: Option<BTreeSet<u32>>,
    all_reports: bool,
    first_held: BTreeMap<(Side, CommandRef, u32), Moment>,
    /// Every command of each side at any moment of the game.
    seen: BTreeSet<(Side, CommandRef)>,
    /// A strike decision to review: the group, the day and the pulse.
    strike: Option<(AirGroupId, u32, Pulse)>,
    review: Option<(Side, StrikeReview)>,
    /// For `--why`: the command each group's strike decision and each force's
    /// alert was made by, at each pulse.
    record_deciders: bool,
    launch_command: BTreeMap<(u32, u64, u32), CommandRef>,
    alert_command: BTreeMap<(u32, u64, u32), CommandRef>,
}

impl Watched {
    fn at(&mut self, world: &World, moment: Moment) {
        let key = world.umpire_key();
        for side in Side::ALL {
            let Some(posts) = world.commands(side) else {
                continue;
            };
            for (command, post) in posts {
                self.seen.insert((side, *command));
                for report in post.heard().keys() {
                    let number = report.umpire_raw(&key);
                    if self.all_reports
                        || self
                            .reports
                            .as_ref()
                            .is_some_and(|asked| asked.contains(&number))
                    {
                        self.first_held
                            .entry((side, *command, number))
                            .or_insert(moment);
                    }
                }
            }
        }
        if let Some((group, day, pulse)) = self.strike {
            if moment
                == (Moment::Phase {
                    day,
                    pulse,
                    phase: PulsePhase::StrikeDecisions,
                })
                && self.review.is_none()
            {
                if let Some(side) = world.umpire().air_groups.get(&group).map(|g| g.side) {
                    self.review = world
                        .side_view(side)
                        .strike_review(group, pulse)
                        .map(|review| (side, review));
                }
            }
        }
        if self.record_deciders && world.command_levels() {
            match moment {
                Moment::Phase {
                    day,
                    pulse,
                    phase: PulsePhase::StrikeDecisions,
                } => {
                    for group in world.umpire().air_groups.values() {
                        if let Some(command) =
                            world.side_view(group.side).command_of_group(group.id)
                        {
                            self.launch_command
                                .insert((day, pulse.code(), group.id.raw()), command);
                        }
                    }
                }
                Moment::PulseBegins { day, pulse } => {
                    for force in world.umpire().task_forces.values() {
                        if let Some(command) =
                            world.side_view(force.side).command_of_task_force(force.id)
                        {
                            self.alert_command
                                .insert((day, pulse.code(), force.id.raw()), command);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

/// `area`, `task_force:N`, `base:N`.
fn parse_command(text: &str) -> Result<CommandRef, String> {
    let lower = text.to_ascii_lowercase();
    let bad = || format!("\"{text}\" is not a command: write area, task_force:N or base:N");
    if lower == "area" {
        return Ok(CommandRef::Area);
    }
    let (kind, number) = lower.split_once(':').ok_or_else(bad)?;
    let n = parse_number(number).ok_or_else(bad)?;
    match kind {
        "task_force" => Ok(CommandRef::TaskForce(TaskForceId::new(n))),
        "base" => Ok(CommandRef::Base(BaseId::new(n))),
        _ => Err(bad()),
    }
}

fn parse_pulse(text: &str) -> Result<Pulse, String> {
    match text.to_ascii_lowercase().as_str() {
        "night" => Ok(Pulse::Night),
        "morning" => Ok(Pulse::Morning),
        "afternoon" => Ok(Pulse::Afternoon),
        _ => Err(format!(
            "--pulse \"{text}\" is not night, morning or afternoon"
        )),
    }
}

fn parse_number(text: &str) -> Option<u32> {
    text.parse::<u32>().ok()
}

/// `ship:33`, `allied:contact:3`.
fn parse_handle(text: &str) -> Result<(Handle, Option<Side>), String> {
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
            "\"{text}\" is not a unit handle: write ship:N, task_force:N, air_group:N, base:N, land_unit:N or contact:N"
        )
    };
    let (kind, number) = rest.split_once(':').ok_or_else(bad)?;
    let n = parse_number(number).ok_or_else(bad)?;
    let unit = match kind {
        "ship" => Handle::Ship(n),
        "task_force" => Handle::TaskForce(n),
        "air_group" => Handle::AirGroup(n),
        "base" => Handle::Base(n),
        "land_unit" => Handle::LandUnit(n),
        "contact" => Handle::Contact(n),
        _ => return Err(bad()),
    };
    if side.is_some() && !matches!(unit, Handle::Contact(_)) {
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
        EntityRef::LandUnit(id) => truth.land_units.contains_key(&id),
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
            // Decision 0111: ships formed into a force, moved into another by a
            // join, or left in port in no force by a stand-down.
            EventKind::TaskForceFormed {
                task_force, ships, ..
            } => {
                for ship in ships {
                    membership.insert(*ship, *task_force);
                }
            }
            EventKind::TaskForceJoined { into, ships, .. } => {
                for ship in ships {
                    membership.insert(*ship, *into);
                }
            }
            EventKind::TaskForceStoodDownInPort { ships, .. } => {
                for ship in ships {
                    membership.remove(ship);
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

fn why_tree(world: &World, number: u32, header: &str, watch: &Watched) -> Result<(), String> {
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
    let sent = report_sent_index(world.events().iter(), &world.umpire_key());
    let travels = world.side_view(Side::Allied).reports_travel();
    let ctx = WhyContext {
        world,
        sent: &sent,
        travels,
        watch,
    };
    print_event(&ctx, &find, number, 0, "", None, &mut shown);
    Ok(())
}

/// For each event that a `ReportSent` passed on (decision 0135, 22), the
/// numbers of the `ReportSent` events, in the order of the log.
fn report_sent_index<'a>(
    events: impl Iterator<Item = &'a Event>,
    key: &UmpireKey,
) -> BTreeMap<u32, Vec<u32>> {
    let mut index: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    for event in events {
        if let EventKind::ReportSent { report, .. } = &event.kind {
            index
                .entry(report.umpire_raw(key))
                .or_default()
                .push(event.id.umpire_raw(key));
        }
    }
    index
}

/// What walking one event's causes reads besides the log: the passings each
/// report had, whether reports travel between commands, and which command
/// decided each launch and alert (`Watched`).
struct WhyContext<'a> {
    world: &'a World,
    sent: &'a BTreeMap<u32, Vec<u32>>,
    travels: bool,
    watch: &'a Watched,
}

impl WhyContext<'_> {
    /// The command that decided the event, when it is a launch or an alert
    /// and reports travel between commands: the command of the launching
    /// group at the strike decision, the force (or the area command, if it
    /// had ceased to be) at the alert. Under such an event a report is
    /// followed only along the road that brought it to that command
    /// (decision 0138, item 1).
    fn decider(&self, event: &Event) -> Option<CommandRef> {
        if !self.travels {
            return None;
        }
        let pulse = event.pulse?.code();
        match &event.kind {
            EventKind::StrikeLaunched { air_group, .. } => self
                .watch
                .launch_command
                .get(&(event.day, pulse, air_group.raw()))
                .copied(),
            EventKind::ForceAlertedByContact { task_force, .. } => self
                .watch
                .alert_command
                .get(&(event.day, pulse, task_force.raw()))
                .copied(),
            _ => None,
        }
    }
}

/// Under a report event that a launch or an alert cites: the road that
/// brought the report to the command that decided, and nothing else of how
/// it was passed.
fn print_road_to_decider<'a>(
    ctx: &WhyContext<'a>,
    find: &dyn Fn(u32) -> Option<&'a Event>,
    indent: &str,
    number: u32,
    event: &Event,
    command: CommandRef,
) {
    let key = ctx.world.umpire_key();
    let sents = report_road::sents_of(ctx.world, &key, number);
    if sents.is_empty() {
        return;
    }
    let road = report_road::road(command, report_road::made_when(event), &sents, true);
    println!(
        "{indent}    -> {}  [{command} decided]",
        report_road::line(command, number, &road)
    );
    if let report_road::Road::Told {
        event: passing,
        up_behind,
        ..
    } = &road
    {
        for n in std::iter::once(passing).chain(up_behind.iter().map(|(up, ..)| up)) {
            if let Some(passing) = find(*n) {
                println!("{indent}    -> {}", event_line(ctx.world, passing));
            }
        }
    }
}

fn print_event<'a>(
    ctx: &WhyContext<'a>,
    find: &dyn Fn(u32) -> Option<&'a Event>,
    number: u32,
    depth: usize,
    lead: &str,
    narrow: Option<CommandRef>,
    shown: &mut BTreeSet<u32>,
) {
    let world = ctx.world;
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
    // How the report this event is was passed between commands, and when
    // it arrived; the passings' own causes lead back to this event. Under a
    // launch or an alert, only the road to the command that decided.
    match narrow {
        Some(command) => print_road_to_decider(ctx, find, &indent, number, event, command),
        None => {
            for passing in ctx.sent.get(&number).into_iter().flatten() {
                if let Some(passing) = find(*passing) {
                    println!("{indent}    -> {}", event_line(world, passing));
                }
            }
        }
    }
    let key = world.umpire_key();
    let decider = ctx.decider(event);
    for cause in &event.causes {
        match cause {
            Cause::Event(id) => print_event(
                ctx,
                find,
                id.umpire_raw(&key),
                depth + 1,
                "<- ",
                decider,
                shown,
            ),
            Cause::Order(id) => match world.order(*id) {
                Some(order) => println!(
                    "{indent}    <- order:{} ({}, day {}, issued by {:?}{}): {:?}",
                    id.umpire_raw(&key),
                    order.side.name(),
                    order.day_issued,
                    order.origin.issuer,
                    order
                        .origin
                        .intent
                        .map_or(String::new(), |i| format!(" for {i}")),
                    order.kind
                ),
                None => println!(
                    "{indent}    <- order:{} (not in the log)",
                    id.umpire_raw(&key)
                ),
            },
        }
    }
}

/// `--report`: when each command of the report's side first held it and by
/// which road, or that it never did (decision 0138, item 1).
fn report_road_answer(
    world: &World,
    number: u32,
    only: Option<CommandRef>,
    watch: &Watched,
    header: &str,
    json: bool,
) -> Result<(), String> {
    let answers = report_answers(world, number, only, watch)?;
    if json {
        println!("{}", report_road::answers_json(number, &answers));
        return Ok(());
    }
    println!("{header}");
    for line in report_road::answers_text(&answers) {
        println!("{line}");
    }
    Ok(())
}

/// The answer for the report whose event number is `number`: the side, a line
/// for each command asked about and the notes that go with them.
pub(super) fn report_answers(
    world: &World,
    number: u32,
    only: Option<CommandRef>,
    watch: &Watched,
) -> Result<report_road::Answers, String> {
    let key = world.umpire_key();
    let event = usize::try_from(number)
        .ok()
        .and_then(|n| n.checked_sub(1))
        .and_then(|i| world.events().iter().nth(i))
        .ok_or_else(|| {
            format!(
                "no event #{number} in this run (it has {} events)",
                world.events().len()
            )
        })?;
    let sents = report_road::sents_of(world, &key, number);
    let held_by: BTreeSet<(Side, CommandRef)> = watch
        .first_held
        .keys()
        .filter(|(_, _, report)| *report == number)
        .map(|(side, command, _)| (*side, *command))
        .collect();
    let side = sents
        .first()
        .map(|sent| sent.side)
        .or_else(|| held_by.iter().next().map(|(side, _)| *side))
        .or_else(|| contact_side(world, event));
    let not_a_report = || format!("event #{number} is not a report passed between commands");
    if !world.command_levels() {
        // Without commands every report is the side's own at once.
        return match side {
            Some(_) => Ok(report_road::Answers::Everyone {
                side: side.expect("matched"),
                text: "command levels are off: the side held it when it was made".to_string(),
            }),
            None => Err(not_a_report()),
        };
    }
    if !world.side_view(Side::Allied).reports_travel() {
        return match side {
            Some(side) => Ok(report_road::Answers::Everyone {
                side,
                text: "reports do not travel in this game: every command of the side held it when it was made"
                    .to_string(),
            }),
            None => Err(not_a_report()),
        };
    }
    let Some(side) = side.filter(|_| !sents.is_empty() || !held_by.is_empty()) else {
        return Err(not_a_report());
    };
    let commands: Vec<CommandRef> = match only {
        Some(command) => {
            if !watch.seen.contains(&(side, command)) {
                return Err(format!(
                    "no {command} of the {} side in this run",
                    side.name()
                ));
            }
            vec![command]
        }
        None => watch
            .seen
            .iter()
            .filter(|(s, _)| *s == side)
            .map(|(_, command)| *command)
            .collect(),
    };
    let made = report_road::made_when(event);
    let rows = commands
        .into_iter()
        .map(|command| {
            let first = watch.first_held.get(&(side, command, number)).copied();
            let mut road = report_road::road(command, made, &sents, first.is_some());
            // A passing a pulse late that fell due after the game's last day.
            if let (
                None,
                report_road::Road::Told {
                    event,
                    sender,
                    direction,
                    arrival: sim_core::events::Arrival::Late,
                    when,
                    ..
                },
            ) = (first, &road)
            {
                let game_ended = when.0 > world.umpire().day;
                let ceased = world
                    .commands(side)
                    .is_some_and(|posts| !posts.contains_key(&command));
                if game_ended || ceased {
                    road = report_road::Road::Overtaken {
                        event: *event,
                        sender: *sender,
                        direction: *direction,
                        due: *when,
                        game_ended,
                    };
                }
            }
            report_road::Row {
                command,
                first_seen: first,
                road,
            }
        })
        .collect();
    Ok(report_road::Answers::PerCommand {
        side,
        report: number,
        rows,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use sim_core::events::{Arrival, Direction, ReportListener};
    use sim_core::ids::EventId;
    use sim_core::pictures::CommandRef;

    fn event(raw: u32, kind: EventKind, key: &UmpireKey) -> Event {
        Event {
            id: EventId::for_umpire(raw, key),
            day: 1,
            pulse: None,
            kind,
            causes: Vec::new(),
            breakdown: None,
        }
    }

    #[test]
    fn a_report_is_indexed_to_the_passings_that_name_it() {
        let (executed, _) = execute_source(
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../data/scenarios/01_transit.toml"
            ),
            None,
            &sim_data::LoadOptions::default(),
        )
        .expect("scenario 1 runs");
        let key = executed.scenario.world.umpire_key();
        let sent = |report: u32| EventKind::ReportSent {
            side: Side::Allied,
            sender: CommandRef::Area,
            report: EventId::for_umpire(report, &key),
            listeners: vec![ReportListener {
                command: CommandRef::Area,
                direction: Direction::Down,
                arrival: Arrival::OnTime,
            }],
        };
        let events = [
            event(2, sent(1), &key),
            event(3, sent(1), &key),
            event(4, sent(9), &key),
        ];
        let index = report_sent_index(events.iter(), &key);
        assert_eq!(index.get(&1), Some(&vec![2, 3]));
        assert_eq!(index.get(&9), Some(&vec![4]));
        assert_eq!(index.get(&2), None);
    }

    /// The Eastern Solomons from intents (decision 0128) with command levels
    /// on and the live report figures, as a temporary file whose data paths
    /// are made absolute: made here and not kept as a file.
    fn from_intents_with_levels(name: &str) -> std::path::PathBuf {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let root = root.canonicalize().expect("the repository root");
        let text = std::fs::read_to_string(root.join("data/scenarios/second-proof").join(name))
            .expect("the scenario reads");
        let text = text.replace("\"../../", &format!("\"{}/data/", root.display()));
        let text = format!(
            "command_levels = true\nreports = \"{}/data/reference/reports.toml\"\n{text}",
            root.display()
        );
        let file = std::env::temp_dir().join(format!(
            "flag_plot_events_levels_{}_{}",
            std::process::id(),
            name
        ));
        std::fs::write(&file, text).expect("the temporary scenario writes");
        file
    }

    /// Every report the commands of a game passed has a road in the log that
    /// the state agrees with: a command the state shows holding it has a road
    /// to it, one it never shows holding has none, and the moment the state
    /// first shows it is one the road allows. Scenarios 62, 63 and 64 on the
    /// frozen figures, and the Eastern Solomons from intents with command
    /// levels on, seeds 1 to 5. Also: every road is met somewhere.
    #[test]
    fn every_report_has_a_road_the_state_agrees_with() {
        use super::report_road::{Answers, Road};
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let frozen = sim_data::LoadOptions::frozen_step1(&root);
        let eastern = from_intents_with_levels("05a_eastern_solomons.toml");
        let week = from_intents_with_levels("07a_week.toml");
        let mut cases: Vec<(String, sim_data::LoadOptions, u64)> = Vec::new();
        for name in [
            "62_reports_between_commands",
            "63_a_superiors_knowledge",
            "64_commanders_and_initiative",
        ] {
            cases.push((
                format!("{}/data/scenarios/{name}.toml", root.display()),
                frozen.clone(),
                1942,
            ));
        }
        cases.push((
            week.display().to_string(),
            sim_data::LoadOptions::default(),
            1,
        ));
        for seed in 1..=5 {
            cases.push((
                eastern.display().to_string(),
                sim_data::LoadOptions::default(),
                seed,
            ));
        }
        let (mut own, mut up, mut sideways, mut down, mut late, mut never) = (0, 0, 0, 0, 0, 0);
        for (path, options, seed) in &cases {
            let mut watch = Watched {
                all_reports: true,
                ..Watched::default()
            };
            let (executed, _) = execute_source_watching(
                path,
                (!path.contains("scenarios/6")).then_some(*seed),
                options,
                &mut |world, moment| watch.at(world, moment),
            )
            .expect("the game runs");
            let world = &executed.scenario.world;
            let key = world.umpire_key();
            let reports: BTreeSet<u32> = world
                .events()
                .iter()
                .filter_map(|event| match &event.kind {
                    EventKind::ReportSent { report, .. } => Some(report.umpire_raw(&key)),
                    _ => None,
                })
                .collect();
            for report in reports {
                let answers = report_answers(world, report, None, &watch)
                    .unwrap_or_else(|e| panic!("{path} seed {seed} report #{report}: {e}"));
                let lines = report_road::answers_text(&answers);
                assert!(
                    lines.iter().all(|line| !line.starts_with("note:")),
                    "{path} seed {seed} report #{report}: {lines:?}"
                );
                let Answers::PerCommand { rows, .. } = answers else {
                    panic!("reports travel in these games");
                };
                for row in rows {
                    assert_eq!(
                        matches!(row.road, Road::Never { .. } | Road::Overtaken { .. }),
                        row.first_seen.is_none(),
                        "{path} seed {seed} report #{report} {}",
                        row.command
                    );
                    match &row.road {
                        Road::Own { .. } => own += 1,
                        Road::Never { .. } | Road::Overtaken { .. } => never += 1,
                        Road::Told {
                            direction, arrival, ..
                        } => {
                            match direction {
                                sim_core::events::Direction::Up => up += 1,
                                sim_core::events::Direction::Sideways => sideways += 1,
                                sim_core::events::Direction::Down => down += 1,
                            }
                            if *arrival == Arrival::Late {
                                late += 1;
                            }
                        }
                    }
                }
            }
        }
        let _ = std::fs::remove_file(&eastern);
        let _ = std::fs::remove_file(&week);
        for (name, count) in [
            ("own", own),
            ("up", up),
            ("sideways", sideways),
            ("down", down),
            ("late", late),
            ("never", never),
        ] {
            assert!(count > 0, "no {name} road was met in these games");
        }
    }
}
