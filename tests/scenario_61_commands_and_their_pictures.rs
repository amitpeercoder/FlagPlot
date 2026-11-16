//! Scenario 61: commands and their pictures. Scenario 58, the carrier
//! force that closes on a contact, played with command levels on: it
//! plays as scenario 58 does, and every Allied command holds the side's
//! picture.
//!
//! Proves decision 0134, step 1 of stage 2b, against a hand calculation.
//! The data is placeholder data: `data/scenarios/61_commands_and_their_pictures.toml`
//! is scenario 58's file with command levels turned on and nothing else
//! changed. It runs on the frozen step 1 figures as scenario 58 does.
//! There is no Japanese unit and no Japanese base, so no die is rolled.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Inputs
//!
//! As scenario 58 (`tests/scenario_58_a_carrier_force_closes.rs`): Allied
//! base 1 at (0,5); task force 1, one carrier, at (0,0); one contact held
//! at the start, 1 ship, a carrier, at (12,0), age 0; intent 1, operate
//! task force 1 from (0,0), at press, both leaves given; 3 days.
//!
//! ## The commands
//!
//! At the start the Allied side has three commands: its area command;
//! task force 1, under the area command; base 1, under the area command.
//! The Japanese side has one: its area command. Air groups 1, 2 and 3
//! belong to task force 1.
//!
//! ## The game
//!
//! Every order, every reason, every arrival and every event is scenario
//! 58's: twelve orders on day 1, two on day 2 and none on day 3; the
//! force at (6,0) at hour 8; the strike of 10 in the morning of day 1,
//! which finds nothing; the force back at (0,0) at hour 32.
//!
//! ## The pictures
//!
//! | When | The side's picture | Each of the three Allied commands' |
//! | --- | --- | --- |
//! | The start | Contact 1: 1 ship, a carrier, at (12,0), age 0 | The same |
//! | After the night of day 1 | Contact 1, age 1 | The same |
//! | After the morning of day 1 | No contact: the strike found nothing there and the contact is dropped | No contact |
//! | Every later pulse | No contact | No contact |
//!
//! The Japanese area command's picture holds nothing at any time, as its
//! side's does.
//!
//! The strike of the morning of day 1 is decided on task force 1's
//! picture: its reason, or the launch's record, names task force 1 as the
//! command whose picture was read. The staff's orders to task force 1 and
//! its groups are decided on task force 1's picture, and the reasons that
//! name contact 1 name it by the number it has there, which is 1.
//!
//! # Properties
//!
//! - **On is off.** The same file with command levels off gives the same
//!   events, the same orders and reasons, and the same state in every
//!   field but the commands.
//! - **Balanced.** The balanced property of scenario 58, with command
//!   levels on: ten orders on day 1, none on day 2, two on day 3. Contact
//!   1 is in all three Allied pictures at ages 1, 2 and 3 after the three
//!   pulses of day 1, at 4 after the night of day 2, and is dropped from
//!   all of them at the end of the morning of day 2, at 5 pulses, in the
//!   pulse the side's own drops it.
//! - **A force formed in the game.** With a commander's order on day 2
//!   that detaches nothing (the carrier is alone), no command is added.
//!   The forming of a command is tested where a force can be formed: in a
//!   view built for it, a force detached from task force 1 is a command
//!   under the area command, and its picture is a copy of task force 1's.
//! - **A base that changes side** stops being a command of the side that
//!   lost it and becomes one of the side that took it, with a copy of that
//!   side's area command's picture: in a view built for it.
//! - **Bigger games, on is off.** On live figures, with command levels
//!   turned on and nothing else changed: the Eastern Solomons from intents
//!   (`data/scenarios/second-proof/05a_eastern_solomons.toml`) on seeds 1
//!   to 20, Savo Island (scenario 4) on seeds 1 to 20, and the week of the
//!   slice from intents (`data/scenarios/second-proof/07a_*.toml`) on
//!   seeds 1 to 3, each give the events they give with command levels
//!   off, and at the end of every pulse every command's picture holds what
//!   its side's holds: the same contacts with the same numbers, places,
//!   counts, kinds and ages, the same claims and the same estimates.
//! - **Off is unchanged.** Every scenario built before this one gives the
//!   events it gave.
//! - Nothing that plays is given another command's picture or the truth:
//!   a command's view returns its own picture only.

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::Instant;

use sim_core::choices::Moment;
use sim_core::command::{Reason, StaffDecisions, StaffOrder};
use sim_core::events::{Event, EventKind};
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, TaskForceId};
use sim_core::pictures::CommandRef;
use sim_core::refdata::ShipKind;
use sim_core::state::Side;
use sim_core::time::Pulse;
use sim_core::view::SideView;
use sim_core::{Staffs, World};
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

mod common;

const LEVELS_LINE: &str = "command_levels = true\n";

fn path() -> PathBuf {
    common::data_dir().join("scenarios/61_commands_and_their_pictures.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

/// Scenario 58's file text. It sits in the same folder as scenario 61's, so
/// the files it names are read from the same places.
fn text_58() -> String {
    std::fs::read_to_string(common::data_dir().join("scenarios/58_a_carrier_force_closes.toml"))
        .expect("scenario 58's file reads")
}

/// The same text with command levels off.
fn off(text: &str) -> String {
    common::replace_once(text, LEVELS_LINE, "")
}

fn group(raw: u32) -> AirGroupId {
    AirGroupId::new(raw)
}

fn force(raw: u32) -> TaskForceId {
    TaskForceId::new(raw)
}

fn base(raw: u32) -> BaseId {
    BaseId::new(raw)
}

fn hex(q: i32, r: i32) -> Hex {
    Hex { q, r }
}

fn tf(raw: u32) -> CommandRef {
    CommandRef::TaskForce(force(raw))
}

fn bs(raw: u32) -> CommandRef {
    CommandRef::Base(base(raw))
}

/// The scenario's own figures: scenario 53's table of aircraft for each ship,
/// added after the frozen file's empty `[strike_aircraft_per_ship]` table, as
/// scenario 58 does.
fn figures() -> sim_data::LoadOptions {
    common::altered_reference("s61", |dir| {
        common::edit_file(dir, "air_combat.toml", |text| {
            format!("{text}\ncruiser = 12\ndestroyer = 6\ntransport = 6\nauxiliary = 4\n")
        })
    })
}

/// What a picture holds, as text: its contacts, claims and estimates, with
/// every field.
fn pic_text(
    contacts: Vec<&sim_core::belief::Contact>,
    claims: &[sim_core::belief::BattleClaim],
    estimates: Vec<&sim_core::belief::GroundEstimate>,
) -> String {
    format!("{contacts:?} | {claims:?} | {estimates:?}")
}

fn side_pic(view: &SideView<'_>) -> String {
    pic_text(
        view.contacts().collect(),
        view.claims(),
        view.ground_estimates().collect(),
    )
}

fn command_pic(view: &sim_core::view::CommandView<'_>) -> String {
    pic_text(
        view.contacts().collect(),
        view.claims(),
        view.ground_estimates().collect(),
    )
}

/// A contact as (number, hex, ships, kind, age).
type Held = (u32, Hex, u32, ShipKind, u32);

fn side_contacts(view: &SideView<'_>) -> Vec<Held> {
    view.contacts()
        .map(|c| {
            (
                c.id.umpire_raw(common::umpire_key()),
                c.estimated_hex,
                c.estimated_ship_count,
                c.estimated_kind,
                c.age_pulses,
            )
        })
        .collect()
}

fn command_contacts(view: &sim_core::view::CommandView<'_>) -> Vec<Held> {
    view.contacts()
        .map(|c| {
            (
                view.contact_number(c.id),
                c.estimated_hex,
                c.estimated_ship_count,
                c.estimated_kind,
                c.age_pulses,
            )
        })
        .collect()
}

/// What the Allied side and its commands held at one moment.
struct Pictures {
    side: Vec<Held>,
    side_text: String,
    commands: BTreeMap<CommandRef, (Vec<Held>, String)>,
    japanese: Vec<Held>,
    japanese_commands: BTreeMap<CommandRef, (Vec<Held>, String)>,
}

fn pictures(world: &World) -> Pictures {
    let commands_of = |side: Side| {
        let view = world.side_view(side);
        view.commands()
            .into_iter()
            .map(|c| {
                let v = view.command(c).expect("a listed command has a view");
                (c, (command_contacts(&v), command_pic(&v)))
            })
            .collect::<BTreeMap<_, _>>()
    };
    let allied = world.side_view(Side::Allied);
    Pictures {
        side: side_contacts(&allied),
        side_text: side_pic(&allied),
        commands: commands_of(Side::Allied),
        japanese: side_contacts(&world.side_view(Side::Japanese)),
        japanese_commands: commands_of(Side::Japanese),
    }
}

/// A game played to its end with the doctrine's staffs.
struct Played {
    /// What the Allied staff handed in each day.
    handed: BTreeMap<u32, StaffDecisions>,
    /// The truth, each side's belief, and each side's command state, as text,
    /// at the end of each day (index 0 is the start).
    truth: Vec<String>,
    beliefs: Vec<String>,
    states: Vec<String>,
    events: Vec<Event>,
    /// The pictures at the start, then at the end of each pulse in turn.
    ends: Vec<Pictures>,
    /// The command of group 3 when the strike decisions of the morning of
    /// day 1 had been made.
    launcher: Option<CommandRef>,
    /// Allied commands at the end.
    commands_at_end: Vec<CommandRef>,
    force_at: BTreeMap<String, Hex>,
}

fn belief_text(world: &World) -> String {
    Side::ALL
        .into_iter()
        .map(|side| side_pic(&world.side_view(side)))
        .collect::<Vec<_>>()
        .join(" || ")
}

fn state_text(world: &World) -> String {
    Side::ALL
        .into_iter()
        .map(|side| format!("{:?}", world.command_state(side)))
        .collect::<Vec<_>>()
        .join(" || ")
}

fn play(text: &str) -> Played {
    let mut scenario: Scenario =
        common::from_text_with(text, &path(), &figures()).expect("the scenario loads");
    let held = staffs();
    let mut played = Played {
        handed: BTreeMap::new(),
        truth: vec![format!("{:?}", scenario.world.umpire())],
        beliefs: vec![belief_text(&scenario.world)],
        states: vec![state_text(&scenario.world)],
        events: Vec::new(),
        ends: Vec::new(),
        launcher: None,
        commands_at_end: Vec::new(),
        force_at: BTreeMap::new(),
    };
    let mut ended_days: Vec<u32> = Vec::new();
    loop {
        let day = scenario.world.day() + 1;
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |world, moment| match moment {
                Moment::DayBegins { day: 1 } => played.ends.push(pictures(world)),
                Moment::PulseBegins { day, pulse } => {
                    if pulse != Pulse::Night {
                        played.ends.push(pictures(world));
                    }
                    played.force_at.insert(
                        format!("{day} {pulse:?}"),
                        world.umpire().task_forces[&force(1)].position(),
                    );
                }
                Moment::EndOfDay { day, .. } => {
                    if !ended_days.contains(&day) {
                        ended_days.push(day);
                        played.ends.push(pictures(world));
                    }
                }
                Moment::Phase { day, pulse, phase }
                    if day == 1
                        && pulse == Pulse::Morning
                        && format!("{phase:?}") == "AirCombat" =>
                {
                    played.launcher = world.side_view(Side::Allied).command_of_group(group(3));
                }
                _ => {}
            })
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
        played.handed.insert(
            day,
            scenario
                .world
                .staff_handed_in()
                .get(&Side::Allied)
                .cloned()
                .unwrap_or_default(),
        );
        played.truth.push(format!("{:?}", scenario.world.umpire()));
        played.beliefs.push(belief_text(&scenario.world));
        played.states.push(state_text(&scenario.world));
    }
    played.events = scenario.world.events().iter().cloned().collect();
    played.commands_at_end = scenario.world.side_view(Side::Allied).commands();
    played
}

impl Played {
    fn orders(&self, day: u32) -> Vec<&StaffOrder> {
        self.handed[&day].orders.iter().collect()
    }

    fn order_count(&self) -> Vec<usize> {
        (1..=self.handed.len() as u32)
            .map(|d| self.handed[&d].orders.len())
            .collect()
    }

    fn all_reasons(&self) -> Vec<String> {
        let mut out = Vec::new();
        for d in self.handed.values() {
            out.extend(d.orders.iter().map(|o| said(&o.reason)));
            out.extend(d.no_orders.iter().map(said));
        }
        out
    }

    /// Every order of the staff, with its reason's text, as text.
    fn orders_and_reasons(&self) -> Vec<String> {
        self.handed
            .iter()
            .flat_map(|(day, d)| {
                d.orders
                    .iter()
                    .map(move |o| format!("day {day}: {:?} | {}", o.kind, said(&o.reason)))
            })
            .collect()
    }

    fn launches(&self) -> Vec<(u32, Hex, u32)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::StrikeLaunched {
                    air_group,
                    hex,
                    aircraft,
                    ..
                } => Some((air_group.raw(), *hex, *aircraft)),
                _ => None,
            })
            .collect()
    }

    fn arrivals(&self) -> Vec<(Hex, sim_core::fixed::Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceArrived {
                    hex, elapsed_hours, ..
                } => Some((*hex, *elapsed_hours)),
                _ => None,
            })
            .collect()
    }
}

/// A reason's text: its calculation and its evidence.
fn said(reason: &Reason) -> String {
    let evidence: Vec<&str> = reason.evidence.iter().map(|e| e.what.as_str()).collect();
    format!("{} | {}", reason.worked, evidence.join(" | "))
}

/// The wall clock is not allowed in the simulation (clippy.toml). This is the
/// one place this test reads it, to time a run from outside: nothing depends
/// on what it says.
struct Stopwatch(Instant);

impl Stopwatch {
    #[allow(clippy::disallowed_methods)]
    fn start() -> Stopwatch {
        Stopwatch(Instant::now())
    }

    fn seconds(&self) -> f64 {
        self.0.elapsed().as_secs_f64()
    }
}

fn the_contact(age: u32) -> Vec<Held> {
    vec![(1, hex(12, 0), 1, ShipKind::Carrier, age)]
}

const ALLIED_COMMANDS: [CommandRef; 3] = [
    CommandRef::Area,
    CommandRef::TaskForce(TaskForceId::new(1)),
    CommandRef::Base(BaseId::new(1)),
];

// ------------------------------------------------------- The hand calculation

#[test]
fn the_file_is_scenario_58s_with_command_levels_on_and_a_name() {
    let t = text();
    assert!(t.contains(LEVELS_LINE));
    let normal = |s: &str| {
        s.lines()
            .filter(|l| {
                !l.starts_with('#') && !l.starts_with("name = ") && *l != "command_levels = true"
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    assert_eq!(normal(&t), normal(&text_58()));
}

#[test]
fn the_commands_at_the_start() {
    let scenario = common::from_text_with(&text(), &path(), &figures()).expect("loads");
    let world = &scenario.world;
    assert!(world.command_levels());
    let allied = world.side_view(Side::Allied);
    assert!(allied.command_levels());
    assert_eq!(allied.commands(), ALLIED_COMMANDS.to_vec());
    for c in ALLIED_COMMANDS {
        let v = allied.command(c).expect("a view");
        let expected = (c != CommandRef::Area).then_some(CommandRef::Area);
        assert_eq!(v.superior(), expected, "{c:?}");
        assert_eq!(v.side(), Side::Allied);
        assert_eq!(v.command(), c);
    }
    let japanese = world.side_view(Side::Japanese);
    assert_eq!(japanese.commands(), vec![CommandRef::Area]);
    assert_eq!(
        japanese
            .command(CommandRef::Area)
            .expect("a view")
            .superior(),
        None
    );
    for raw in [1, 2, 3] {
        assert_eq!(
            allied.command_of_group(group(raw)),
            Some(tf(1)),
            "group {raw}"
        );
    }
    // The umpire's table agrees.
    let table = world.commands(Side::Allied).expect("on");
    assert_eq!(
        table.keys().copied().collect::<Vec<_>>(),
        ALLIED_COMMANDS.to_vec()
    );
    assert_eq!(world.commands(Side::Japanese).expect("on").len(), 1);
}

#[test]
fn the_pictures_table_for_the_side_and_each_allied_command() {
    let played = play(&text());
    // The start, the end of each of nine pulses.
    assert_eq!(played.ends.len(), 10);
    let expectations: [Vec<Held>; 3] = [the_contact(0), the_contact(1), Vec::new()];
    for (when, pictures) in played.ends.iter().enumerate() {
        let expected = expectations.get(when).cloned().unwrap_or_default();
        assert_eq!(pictures.side, expected, "the side, at moment {when}");
        assert_eq!(
            pictures.commands.keys().copied().collect::<Vec<_>>(),
            ALLIED_COMMANDS.to_vec(),
            "at moment {when}"
        );
        for (c, (held, text)) in &pictures.commands {
            assert_eq!(held, &expected, "{c:?}, at moment {when}");
            assert_eq!(text, &pictures.side_text, "{c:?}, at moment {when}");
        }
        assert!(pictures.japanese.is_empty(), "moment {when}");
        assert_eq!(
            pictures
                .japanese_commands
                .keys()
                .copied()
                .collect::<Vec<_>>(),
            vec![CommandRef::Area]
        );
        for (held, _) in pictures.japanese_commands.values() {
            assert!(held.is_empty(), "moment {when}");
        }
    }
}

#[test]
fn the_game_is_scenario_58s_twelve_orders_two_orders_and_none() {
    let played = play(&text());
    assert_eq!(played.order_count(), vec![12, 2, 0]);
    assert_eq!(played.orders(1).len(), 12);
    assert_eq!(played.orders(2).len(), 2);
    assert!(played.orders(3).is_empty());
    let refused: Vec<_> = played
        .events
        .iter()
        .filter(|e| {
            matches!(
                e.kind,
                EventKind::OrderRejected { .. } | EventKind::CommandRefused { .. }
            )
        })
        .collect();
    assert!(refused.is_empty(), "{refused:?}");
    // Day 1's strike: 10 aircraft of group 3, at the contact, finding nothing.
    assert_eq!(played.launches(), vec![(3, hex(12, 0), 10)]);
    let launch = played
        .events
        .iter()
        .find(|e| matches!(e.kind, EventKind::StrikeLaunched { .. }))
        .expect("a launch");
    assert_eq!((launch.day, launch.pulse), (1, Some(Pulse::Morning)));
    assert!(played
        .events
        .iter()
        .any(|e| matches!(e.kind, EventKind::StrikeFoundNothing { .. })
            && e.day == 1
            && e.pulse == Some(Pulse::Morning)));
    assert_eq!(
        played.arrivals(),
        vec![
            (hex(6, 0), sim_core::fixed::Fixed::from_int(8)),
            (hex(0, 0), sim_core::fixed::Fixed::from_int(32)),
        ]
    );
    assert_eq!(played.force_at["1 Morning"], hex(6, 0));
    assert_eq!(played.force_at["2 Night"], hex(6, 0));
    assert_eq!(played.force_at["2 Morning"], hex(0, 0));
    assert_eq!(played.force_at["3 Night"], hex(0, 0));
    // No command was added or lost.
    assert_eq!(played.commands_at_end, ALLIED_COMMANDS.to_vec());
}

#[test]
fn every_order_reason_arrival_and_event_is_scenario_58s() {
    let on = play(&text());
    let plain = play(&text_58());
    assert_eq!(on.events, plain.events);
    assert_eq!(on.orders_and_reasons(), plain.orders_and_reasons());
    assert_eq!(on.order_count(), plain.order_count());
    assert_eq!(on.arrivals(), plain.arrivals());
    assert_eq!(on.truth, plain.truth);
}

#[test]
fn the_reasons_naming_contact_1_name_it_as_the_command_numbers_it() {
    let on = play(&text());
    let off_run = play(&off(&text()));
    assert_eq!(on.all_reasons(), off_run.all_reasons());
    assert!(
        on.all_reasons().iter().any(|r| r.contains("contact 1")),
        "{:?}",
        on.all_reasons()
    );
    // The command's number of the contact is the contact's, 1.
    let scenario = common::from_text_with(&text(), &path(), &figures()).expect("loads");
    let allied = scenario.world.side_view(Side::Allied);
    for c in ALLIED_COMMANDS {
        let v = allied.command(c).expect("a view");
        let id = v.contacts().next().expect("a contact").id;
        assert_eq!(v.contact_number(id), 1);
        assert!(v.contact_by_number(1).is_some());
        assert!(v.contact_by_number(2).is_none());
    }
}

#[test]
fn the_strike_of_day_1_is_launched_by_a_group_of_task_force_1() {
    let played = play(&text());
    assert!(!played.launches().is_empty());
    assert_eq!(played.launcher, Some(tf(1)));
}

#[test]
#[ignore = "no reason or launch record names the command whose picture was read: adding it changes the events with command levels on and off; the options are with the architect"]
fn the_launch_record_names_task_force_1() {
    let played = play(&text());
    let launch = played
        .events
        .iter()
        .find(|e| matches!(e.kind, EventKind::StrikeLaunched { .. }))
        .expect("a launch");
    let record = format!("{launch:?}");
    assert!(
        record.contains("task force 1")
            || record.contains("TaskForce(TaskForceId(1))")
            || record.contains("TaskForceId(1)"),
        "{record}"
    );
}

// --------------------------------------------------------------- Properties

#[test]
fn on_is_off() {
    let on = play(&text());
    let off_run = play(&off(&text()));
    assert_eq!(on.events.len(), off_run.events.len());
    assert_eq!(on.events, off_run.events);
    assert_eq!(on.orders_and_reasons(), off_run.orders_and_reasons());
    assert_eq!(on.truth, off_run.truth);
    assert_eq!(on.beliefs, off_run.beliefs);
    assert_eq!(on.states, off_run.states);
    // Off has no command at all.
    assert!(off_run.commands_at_end.is_empty());
    assert!(off_run.ends.iter().all(|p| p.commands.is_empty()));
}

fn without_risk(text: &str) -> String {
    common::replace_once(
        text,
        "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"press\"\n",
        "",
    )
}

#[test]
fn balanced_keeps_contact_1_in_all_three_pictures_until_the_morning_of_day_2() {
    let played = play(&without_risk(&text()));
    assert_eq!(played.order_count(), vec![10, 0, 2]);
    assert!(played.launches().is_empty());
    assert_eq!(played.ends.len(), 10);
    // Start, then ages 1, 2, 3 after the three pulses of day 1, 4 after the
    // night of day 2.
    for (when, pictures) in played.ends.iter().enumerate() {
        let expected = if when <= 4 {
            the_contact(when as u32)
        } else {
            Vec::new()
        };
        assert_eq!(pictures.side, expected, "the side, at moment {when}");
        assert_eq!(pictures.commands.len(), 3, "moment {when}");
        for (c, (held, text)) in &pictures.commands {
            assert_eq!(held, &expected, "{c:?}, at moment {when}");
            assert_eq!(text, &pictures.side_text, "{c:?}, at moment {when}");
        }
    }
    // Dropped in the pulse the side's own drops it: the morning of day 2, at
    // 5 pulses.
    let dropped: Vec<(u32, Option<Pulse>, u32)> = played
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::ContactDropped {
                side: Side::Allied,
                age_pulses,
                ..
            } => Some((e.day, e.pulse, *age_pulses)),
            _ => None,
        })
        .collect();
    assert_eq!(dropped, vec![(2, Some(Pulse::Morning), 5)]);
}

#[test]
fn a_commanders_order_on_day_2_that_detaches_nothing_adds_no_command() {
    // The carrier is alone: an order to detach her would leave the force
    // empty and is refused.
    let t = format!(
        "{}\n[[order]]\nday = 2\nside = \"allied\"\ntask_force = 1\nkind = \"detach\"\nships = [1]\n",
        text()
    );
    let played = play(&t);
    assert!(played
        .events
        .iter()
        .any(|e| e.day == 2 && matches!(e.kind, EventKind::OrderRejected { .. })));
    assert!(!played
        .events
        .iter()
        .any(|e| matches!(e.kind, EventKind::ShipsDetached { .. })));
    assert_eq!(played.commands_at_end, ALLIED_COMMANDS.to_vec());
    for p in &played.ends {
        assert_eq!(p.commands.len(), 3);
    }
}

// ------------------------------------------------ The first to see another's

#[test]
fn nothing_that_plays_is_given_another_commands_picture_or_the_truth() {
    for levels in [true, false] {
        let t = if levels { text() } else { off(&text()) };
        let scenario = common::from_text_with(&t, &path(), &figures()).expect("loads");
        let allied = scenario.world.side_view(Side::Allied);
        let japanese = scenario.world.side_view(Side::Japanese);
        // The Japanese side has no force and no base here; asking for the
        // Allied commands from its view gives nothing.
        for c in [tf(1), bs(1)] {
            assert!(japanese.command(c).is_none(), "levels {levels}: {c:?}");
        }
        // And for a force or base that does not exist, from either side.
        for c in [tf(2), bs(2), tf(99)] {
            assert!(allied.command(c).is_none(), "levels {levels}: {c:?}");
            assert!(japanese.command(c).is_none(), "levels {levels}: {c:?}");
        }
        assert!(japanese.command_of_group(group(1)).is_none());
    }
}

// ---------------------------------------------------------- Off is unchanged

#[test]
fn every_scenario_file_but_61_loads_with_command_levels_off() {
    let dir = common::data_dir().join("scenarios");
    let mut seen = 0;
    for entry in std::fs::read_dir(&dir).expect("the folder reads") {
        let path = entry.expect("an entry").path();
        if path.extension().is_none_or(|e| e != "toml") {
            continue;
        }
        let name = path
            .file_name()
            .expect("a name")
            .to_string_lossy()
            .to_string();
        let text = std::fs::read_to_string(&path).expect("reads");
        if name.starts_with("61_") {
            assert!(text.contains(LEVELS_LINE));
            continue;
        }
        seen += 1;
        assert!(
            !text.contains("command_levels"),
            "{name} names command levels"
        );
        let scenario = Scenario::load(&path).unwrap_or_else(|e| panic!("{name}: {e}"));
        assert!(!scenario.world.command_levels(), "{name}");
        assert!(scenario.command_levels().is_none(), "{name}");
        assert!(scenario.world.commands(Side::Allied).is_none(), "{name}");
        assert!(scenario.world.side_view(Side::Allied).commands().is_empty());
    }
    assert!(seen > 50, "{seen} scenario files");
}

// --------------------------------------------------------- Bigger games

/// Plays `text` (the file's text with command levels on or off) to its end
/// with `held`'s staffs (none for a scripted game) on `seed`, and returns the
/// events, the end truth as text and the number of pulse ends at which every
/// command's picture held exactly what the side's holds (checked as it goes
/// when command levels are on).
fn run_bigger(
    text: &str,
    path: &std::path::Path,
    seed: u64,
    staffed: bool,
    check_pictures: bool,
) -> (Vec<Event>, String, usize) {
    let mut scenario = Scenario::from_text(text, path)
        .expect("the scenario loads")
        .with_seed(seed)
        .expect("the seed is set");
    let held = staffs();
    let mut checks = 0usize;
    loop {
        let staffs: Staffs<'_> = if staffed {
            as_staffs(&held)
        } else {
            Staffs::none()
        };
        let summary = scenario
            .next_day_watched(staffs, &mut |world, moment| {
                let at_pulse_end = match moment {
                    Moment::DayBegins { .. } => true,
                    Moment::PulseBegins { pulse, .. } => pulse != Pulse::Night,
                    Moment::EndOfDay { phase, .. } => format!("{phase:?}") == "BaseCount",
                    _ => false,
                };
                if check_pictures && at_pulse_end {
                    for side in Side::ALL {
                        let view = world.side_view(side);
                        let own = side_pic(&view);
                        for c in view.commands() {
                            let v = view.command(c).expect("a listed command has a view");
                            assert_eq!(command_pic(&v), own, "{side:?} {c:?} at {moment:?}");
                            checks += 1;
                        }
                    }
                }
            })
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
    }
    (
        scenario.world.events().iter().cloned().collect(),
        format!("{:?}", scenario.world.umpire()),
        checks,
    )
}

fn on_is_off_in_a_bigger_game(file: &str, staffed: bool, seeds: std::ops::RangeInclusive<u64>) {
    let path = common::data_dir().join(file);
    let off_text = std::fs::read_to_string(&path).expect("the scenario file reads");
    assert!(!off_text.contains("command_levels"));
    let on_text = format!("{LEVELS_LINE}{off_text}");
    for seed in seeds {
        let started = Stopwatch::start();
        let (off_events, off_truth, _) = run_bigger(&off_text, &path, seed, staffed, false);
        let (on_events, on_truth, checks) = run_bigger(&on_text, &path, seed, staffed, true);
        assert!(checks > 0);
        assert_eq!(on_events.len(), off_events.len(), "{file} seed {seed}");
        assert_eq!(on_events, off_events, "{file} seed {seed}");
        assert_eq!(on_truth, off_truth, "{file} seed {seed}");
        eprintln!(
            "{file} seed {seed}: {} events, {checks} picture checks, {:.1} s",
            on_events.len(),
            started.seconds()
        );
    }
}

#[test]
fn the_eastern_solomons_from_intents_on_seeds_1_to_20_on_is_off() {
    on_is_off_in_a_bigger_game(
        "scenarios/second-proof/05a_eastern_solomons.toml",
        true,
        1..=20,
    );
}

#[test]
fn savo_island_on_seeds_1_to_20_on_is_off() {
    on_is_off_in_a_bigger_game("scenarios/04_savo_island.toml", false, 1..=20);
}

#[test]
fn the_week_of_the_slice_from_intents_on_seeds_1_to_3_on_is_off() {
    on_is_off_in_a_bigger_game("scenarios/second-proof/07a_week.toml", true, 1..=3);
}
