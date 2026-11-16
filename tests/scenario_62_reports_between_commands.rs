//! Scenario 62: reports between commands. Two carrier forces hold the same
//! contact. One strikes it and finds nothing there. The other is never
//! told, and for a day the two decide differently, each on what it holds.
//!
//! Proves decision 0135, step 2 of stage 2b, against a hand calculation.
//! The data is placeholder data: `data/scenarios/62_reports_between_commands.toml`
//! is scenario 58's with a second carrier force, command levels on and
//! report figures of its own. It runs on the frozen step 1 figures as
//! scenario 58 does. There is no Japanese unit and no Japanese base. Every
//! chance in the report figures is 0 or 1, so no outcome rests on a die.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Figures
//!
//! As scenario 58: a hex is 40 nm; the placeholder dive bomber's strike
//! radius is 250 nm; a contact gains a pulse of age at the end of each
//! pulse and is dropped when its age reaches 5. The report figures of this
//! scenario: up, on time 1; sideways, on time 0 and a pulse late 0, so
//! never; down, on time 0 and a pulse late 0, so never. Radio reach: 600
//! nm for a carrier, 1,000 nm for a base.
//!
//! ## Inputs
//!
//! Scenario 58's map, base 1 at (0,5), task force 1 with ship 1 and air
//! groups 1, 2 and 3 at (0,0), and the one contact the Allied side starts
//! with: 1 ship, a carrier, at (12,0), age 0. Added: task force 2, one
//! placeholder carrier, ship 2, stopped at sea at (7,0), full of fuel, on
//! the carrier mission, at normal readiness, at cruising speed, with the
//! stance engage and base 1 as its home port. Aboard her: group 4, 12
//! fighters; group 5, 6 carrier scouts; group 6, 10 dive bombers; all
//! resting. The game is 3 days.
//!
//! The Allied side's commands of day 1: intent 1, operate task force 1
//! from (0,0), its risk press; intent 2, operate task force 2 from (7,0),
//! with no risk set, so balanced; leave to prepare and leave to launch
//! given for both.
//!
//! The Allied commands: the area command, task force 1, task force 2 and
//! base 1. Each starts with contact 1 in its picture.
//!
//! ## Day 1
//!
//! Task force 1 does what it does in scenario 58: twelve orders, full
//! speed to (6,0), there at hour 8, and in the morning group 3 sends its
//! 10 at the contact, 240 nm away, and finds nothing.
//!
//! Task force 2 is at its hex. The contact is 200 nm away, within reach:
//! it does not close. It is given eight orders, as a carrier force at its
//! hex at balanced is (scenario 55): battle readiness; rules of engagement
//! with the stance avoid and a break-off level of 0.5; group 4, combat air
//! patrol and a patrol share of 0.5; group 5, the search mission and the
//! arc, 050 to 120 at 300 nm, the contact lying on 090; group 6, the naval
//! strike mission and strike sizing on. It launches nothing: the contact
//! is a pulse old in the morning, against the figure of 0.
//!
//! Twenty orders on day 1.
//!
//! **The report.** "Nothing there" at (12,0) is task force 1's. In the
//! morning it drops contact 1 from its picture and sends the report:
//!
//! - up, to the area command: on time. The area command drops contact 1 in
//!   the morning.
//! - sideways, to task force 2, 40 nm away, and to base 1, 223 nm away,
//!   both within 600 nm: never.
//! - The area command sends it down, to task force 2 and to base 1: never.
//!
//! Two "report sent" events in the morning of day 1, the first with three
//! listeners and the second with two.
//!
//! ## The pictures
//!
//! Contact 1, by its age, or "none":
//!
//! | After | Task force 1 | The area command | Task force 2 | Base 1 | The side's own |
//! | --- | --- | --- | --- | --- | --- |
//! | The start | 0 | 0 | 0 | 0 | 0 |
//! | The night of day 1 | 1 | 1 | 1 | 1 | 1 |
//! | The morning of day 1 | none | none | 2 | 2 | none |
//! | The afternoon of day 1 | none | none | 3 | 3 | none |
//! | The night of day 2 | none | none | 4 | 4 | none |
//! | The morning of day 2 | none | none | none | none | none |
//!
//! Task force 2 and base 1 drop it at 5 pulses, having never been told.
//!
//! ## Day 2
//!
//! Task force 1 holds no contact: two orders, the route back to (0,0) and
//! group 2's arc, the whole circle, as in scenario 58.
//!
//! Task force 2 holds contact 1, 3 pulses old, 200 nm away: no order. Its
//! reason names contact 1 by its number in task force 2's picture, its
//! age and its distance, and names task force 2 as the command that
//! holds it.
//!
//! Two orders on day 2.
//!
//! ## Day 3
//!
//! Task force 1 is at its hex: no order. Task force 2 holds no contact:
//! one order, group 5's arc, the whole circle.
//!
//! One order on day 3. Twenty-three in all.
//!
//! # Properties
//!
//! - **Told sideways, on time.** With sideways on time 1: task force 2
//!   and base 1 drop contact 1 in the morning of day 1. Task force 2 is
//!   given group 5's arc, the whole circle, on day 2 and nothing on day
//!   3. The orders are 20, 3 and 0.
//! - **Told down, a pulse late.** With sideways never and down a pulse
//!   late 1: task force 2 and base 1 hold contact 1 at age 2 after the
//!   morning of day 1 and drop it at the start of the afternoon. The
//!   orders are 20, 3 and 0.
//! - **Out of reach.** With sideways on time 1 and a carrier's reach 30
//!   nm: task force 2, 40 nm from task force 1, is not sent the report
//!   sideways, and base 1 is not; the game is the main case. With the
//!   reach 40 nm: task force 2 is sent it and drops the contact in the
//!   morning; base 1, 223 nm away, is not, and holds it to 5 pulses.
//! - **Heard once.** With sideways on time 1 and down on time 1: task
//!   force 2 is sent the report by both roads. It is applied once, and the
//!   game is that of "told sideways, on time".
//! - **A late sighting is as old as it is.** In a view built for it: a
//!   sighting made in pulse 4 that reaches a picture holding no contact at
//!   the start of pulse 5 makes a contact at age 1. It is 1 at the end of
//!   pulse 5 and 2 at the end of pulse 6, as it is in the picture of the
//!   command that made the sighting.
//! - **Newer word stands.** In a view built for it: a picture holds a
//!   contact made by a sighting of pulse 5 at (3,0). A sighting of pulse 4
//!   at (3,0) that arrives in pulse 5 after it is heard and changes
//!   nothing: the contact keeps its count, its kind and its age.
//! - **Every chance certain is step 1.** With on time 1 in every
//!   direction: in this scenario, and on live figures with these report
//!   figures in the Eastern Solomons from intents
//!   (`data/scenarios/second-proof/05a_eastern_solomons.toml`, seeds 1 to
//!   10) and the week of the slice from intents (seed 1), every command's
//!   picture holds what its side's holds at the end of every pulse, and
//!   the events are those of the same game with command levels off, but
//!   for the "report sent" events and the words of a staff reason that
//!   say which command holds a contact it names. (The second exception was
//!   added by the architect on Nov 16, 2026, after the build: decision
//!   0135's item 22 puts those words there, and the first draft of this
//!   property forgot them.)
//! - **No report figures.** Scenario 61 gives what it gave.
//! - **Off is unchanged.** Every scenario built before this one gives the
//!   events it gave.
//! - The Japanese side sees no Allied "report sent" event. The same start
//!   gives the same game. Two worlds that differ only in what the Allied
//!   side cannot see give the same orders and reasons.

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use sim_core::choices::Moment;
use sim_core::command::{Reason, StaffDecisions, StaffOrder};
use sim_core::events::{Arrival, Direction, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, TaskForceId};
use sim_core::orders::{
    AirGroupOrder, AirMission, OrderKind, Reach, Readiness, SpeedSetting, Stance,
    StragglerInstruction, TaskForceOrder,
};
use sim_core::pictures::CommandRef;
use sim_core::refdata::ShipKind;
use sim_core::side_events::SideEventKind;
use sim_core::state::Side;
use sim_core::time::Pulse;
use sim_core::view::SideView;
use sim_core::{Staffs, World};
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

mod common;

const LEVELS_LINE: &str = "command_levels = true\n";
const REPORTS_LINE: &str = "reports = \"../reference/reports_placeholder_scenario_62.toml\"\n";

fn path() -> PathBuf {
    common::data_dir().join("scenarios/62_reports_between_commands.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
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

const AREA: CommandRef = CommandRef::Area;

/// The scenario's own figures: scenario 53's table of aircraft for each ship,
/// added after the frozen file's empty `[strike_aircraft_per_ship]` table, as
/// scenarios 58 and 61 do.
fn figures() -> sim_data::LoadOptions {
    common::altered_reference("s62", |dir| {
        common::edit_file(dir, "air_combat.toml", |text| {
            format!("{text}\ncruiser = 12\ndestroyer = 6\ntransport = 6\nauxiliary = 4\n")
        })
    })
}

/// A pair of chances, each 0 or 1: `(on_time, late)`.
type Chances = (u32, u32);

const NEVER: Chances = (0, 0);
const ON_TIME: Chances = (1, 0);
const LATE: Chances = (0, 1);

/// A report figures file of this test's own, written under the build
/// directory (one for each call, for tests run in parallel), and its path.
/// Nothing in the repository's data folder is touched.
fn figures_file(up: Chances, sideways: Chances, down: Chances, carrier_reach: u32) -> PathBuf {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    std::fs::create_dir_all(&dir).expect("make the folder");
    let file = dir.join(format!(
        "reports_s62_{}_{}.toml",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let pair = |c: Chances| format!("on_time = \"{}\"\nlate = \"{}\"\n", c.0, c.1);
    let body = format!(
        "placeholder = true\n\n[up]\n{}\n[sideways]\n{}\n[down]\n{}\n[radio_reach_nm]\ncarrier = {carrier_reach}\nbattleship = 600\ncruiser = 600\nother_ship = 300\nbase = 1000\n",
        pair(up),
        pair(sideways),
        pair(down),
    );
    std::fs::write(&file, body).expect("write the report figures");
    file
}

fn reports_line(file: &std::path::Path) -> String {
    format!("reports = \"{}\"\n", file.display())
}

/// The scenario's text with other report figures: up, sideways, down, and
/// the radio reach of a carrier in nautical miles.
fn with_figures(up: Chances, sideways: Chances, down: Chances, carrier_reach: u32) -> String {
    common::replace_once(
        &text(),
        REPORTS_LINE,
        &reports_line(&figures_file(up, sideways, down, carrier_reach)),
    )
}

fn without_reports(text: &str) -> String {
    common::replace_once(text, REPORTS_LINE, "")
}

fn off(text: &str) -> String {
    common::replace_once(&without_reports(text), LEVELS_LINE, "")
}

/// A contact as (number, hex, ships, kind, age).
type Held = (u32, Hex, u32, ShipKind, u32);

fn side_contacts(view: &SideView<'_>) -> Vec<Held> {
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

/// What a picture holds: its contacts, and how many claims and estimates.
type Picture = (Vec<Held>, usize, usize);

fn side_picture(view: &SideView<'_>) -> Picture {
    (
        side_contacts(view),
        view.claims().len(),
        view.ground_estimates().count(),
    )
}

fn command_picture(view: &sim_core::view::CommandView<'_>) -> Picture {
    (
        command_contacts(view),
        view.claims().len(),
        view.ground_estimates().count(),
    )
}

/// What the Allied side and its commands held at one moment, and how many
/// reports each command had heard.
struct Pictures {
    side: Picture,
    commands: BTreeMap<CommandRef, Picture>,
    heard: BTreeMap<CommandRef, usize>,
}

fn pictures(world: &World) -> Pictures {
    let view = world.side_view(Side::Allied);
    Pictures {
        side: side_picture(&view),
        commands: view
            .commands()
            .into_iter()
            .map(|c| {
                let v = view.command(c).expect("a listed command has a view");
                (c, command_picture(&v))
            })
            .collect(),
        heard: world
            .commands(Side::Allied)
            .map(|posts| posts.iter().map(|(c, p)| (*c, p.heard().len())).collect())
            .unwrap_or_default(),
    }
}

/// A game played to its end with the doctrine's staffs.
struct Played {
    /// What the Allied staff handed in each day.
    handed: BTreeMap<u32, StaffDecisions>,
    /// The truth at the end of each day (index 0 is the start), as text.
    truth: Vec<String>,
    events: Vec<Event>,
    /// The pictures at the start, then at the end of each pulse in turn: the
    /// start, the nights, mornings and afternoons of the days (10 in all).
    ends: Vec<Pictures>,
    /// The "report sent" events the Allied side and the Japanese side may see.
    allied_saw_sent: usize,
    japanese_saw_sent: usize,
    reports_travel: bool,
}

fn play(text: &str) -> Played {
    let mut scenario: Scenario =
        common::from_text_with(text, &path(), &figures()).expect("the scenario loads");
    let held = staffs();
    let mut played = Played {
        handed: BTreeMap::new(),
        truth: vec![format!("{:?}", scenario.world.umpire())],
        events: Vec::new(),
        ends: Vec::new(),
        allied_saw_sent: 0,
        japanese_saw_sent: 0,
        reports_travel: scenario.world.side_view(Side::Allied).reports_travel(),
    };
    let mut ended_days: Vec<u32> = Vec::new();
    loop {
        let day = scenario.world.day() + 1;
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |world, moment| match moment {
                Moment::DayBegins { day: 1 } => played.ends.push(pictures(world)),
                Moment::PulseBegins { pulse, .. } => {
                    if pulse != Pulse::Night {
                        played.ends.push(pictures(world));
                    }
                }
                Moment::EndOfDay { day, .. } if !ended_days.contains(&day) => {
                    ended_days.push(day);
                    played.ends.push(pictures(world));
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
    }
    played.events = scenario.world.events().iter().cloned().collect();
    let saw = |side: Side| {
        scenario
            .world
            .side_view(side)
            .events()
            .iter()
            .filter(|e| matches!(e.kind, SideEventKind::Whole(EventKind::ReportSent { .. })))
            .count()
    };
    played.allied_saw_sent = saw(Side::Allied);
    played.japanese_saw_sent = saw(Side::Japanese);
    played
}

fn mission(m: AirMission) -> AirGroupOrder {
    AirGroupOrder::SetAirMission(m)
}

fn age(pulses: i64) -> AirGroupOrder {
    AirGroupOrder::SetStrikeContactAge { pulses }
}

fn sizing(on: bool) -> AirGroupOrder {
    AirGroupOrder::SetStrikeSizing { on }
}

fn arc(from_deg: i32, to_deg: i32) -> AirGroupOrder {
    AirGroupOrder::SetSearchArc {
        from_deg,
        to_deg,
        range_nm: n(300),
    }
}

fn share(s: &str) -> AirGroupOrder {
    AirGroupOrder::SetPatrolShare { share: fx(s) }
}

const STRIKE: AirMission = AirMission::NavalStrike;
const CAP: AirMission = AirMission::CombatAirPatrol;
const SEARCH: AirMission = AirMission::Search;
const BATTLE: TaskForceOrder = TaskForceOrder::SetReadiness(Readiness::Battle);

/// An order of the staff: to a force or to an air group.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Given {
    Force(u32, TaskForceOrder),
    Air(u32, AirGroupOrder),
}

fn given(o: &StaffOrder) -> Given {
    match &o.kind {
        OrderKind::TaskForce { target, order } => Given::Force(target.raw(), order.clone()),
        OrderKind::AirGroup { target, order } => Given::Air(target.raw(), order.clone()),
        other => panic!("the doctrine gave an order of another kind: {other:?}"),
    }
}

fn rules(stance: Stance, break_off: &str) -> TaskForceOrder {
    TaskForceOrder::SetRulesOfEngagement {
        stance,
        break_off: fx(break_off),
        stragglers: StragglerInstruction::Keep,
        keep_share: Fixed::ZERO,
    }
}

fn route(waypoints: &[(i32, i32)]) -> TaskForceOrder {
    TaskForceOrder::SetRoute {
        waypoints: waypoints.iter().map(|(q, r)| hex(*q, *r)).collect(),
        act_at: Default::default(),
        stand_at: Default::default(),
    }
}

/// Task force 1's twelve orders of day 1, as scenario 58 has them.
fn tf1_day_1() -> Vec<Given> {
    vec![
        Given::Force(1, BATTLE),
        Given::Force(1, rules(Stance::Avoid, "0.75")),
        Given::Force(1, TaskForceOrder::SetSpeed(SpeedSetting::Full)),
        Given::Force(1, route(&[(6, 0)])),
        Given::Air(1, mission(CAP)),
        Given::Air(1, share("0.25")),
        Given::Air(2, mission(SEARCH)),
        Given::Air(2, arc(50, 120)),
        Given::Air(3, mission(STRIKE)),
        Given::Air(3, age(2)),
        Given::Air(
            3,
            AirGroupOrder::SetReach {
                reach: Reach::Extended,
            },
        ),
        Given::Air(3, sizing(true)),
    ]
}

/// Task force 2's eight orders of day 1, as a carrier force at its hex at
/// balanced has them (scenario 55): the contact lying on 090 in the middle of
/// the arc, 050 to 120.
fn tf2_day_1() -> Vec<Given> {
    vec![
        Given::Force(2, BATTLE),
        Given::Force(2, rules(Stance::Avoid, "0.5")),
        Given::Air(4, mission(CAP)),
        Given::Air(4, share("0.5")),
        Given::Air(5, mission(SEARCH)),
        Given::Air(5, arc(50, 120)),
        Given::Air(6, mission(STRIKE)),
        Given::Air(6, sizing(true)),
    ]
}

impl Played {
    fn orders(&self, day: u32) -> Vec<Given> {
        self.handed[&day].orders.iter().map(given).collect()
    }

    /// The orders of a day to the forces and groups given, in the order the
    /// staff gave them.
    fn orders_to(&self, day: u32, forces: &[u32], groups: &[u32]) -> Vec<Given> {
        self.orders(day)
            .into_iter()
            .filter(|g| match g {
                Given::Force(f, _) => forces.contains(f),
                Given::Air(a, _) => groups.contains(a),
            })
            .collect()
    }

    fn order_count(&self) -> Vec<usize> {
        (1..=self.handed.len() as u32)
            .map(|d| self.handed[&d].orders.len())
            .collect()
    }

    fn nothing_refused(&self) {
        let refused: Vec<_> = self
            .events
            .iter()
            .filter(|e| {
                matches!(
                    e.kind,
                    EventKind::OrderRejected { .. } | EventKind::CommandRefused { .. }
                )
            })
            .collect();
        assert!(refused.is_empty(), "something was refused: {refused:?}");
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

    /// The "report sent" events: (day, pulse, sender, listeners).
    fn sent(&self) -> Vec<Sent> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::ReportSent {
                    sender, listeners, ..
                } => Some((
                    e.day,
                    e.pulse,
                    *sender,
                    listeners
                        .iter()
                        .map(|l| (l.command, l.direction, l.arrival))
                        .collect(),
                )),
                _ => None,
            })
            .collect()
    }

    /// The events but the "report sent" ones, as (day, pulse, kind).
    fn events_but_sent(&self) -> Vec<(u32, Option<Pulse>, EventKind)> {
        self.events
            .iter()
            .filter(|e| !matches!(e.kind, EventKind::ReportSent { .. }))
            .map(|e| (e.day, e.pulse, e.kind.clone()))
            .collect()
    }

    /// Every reason of the staff, of an order or of a decision not to order.
    fn reasons(&self, day: u32) -> Vec<String> {
        let d = &self.handed[&day];
        let mut out: Vec<String> = d.orders.iter().map(|o| said(&o.reason)).collect();
        out.extend(d.no_orders.iter().map(said));
        out
    }

    /// Every order of the staff with its reason's text, as text.
    fn orders_and_reasons(&self) -> Vec<String> {
        self.handed
            .iter()
            .flat_map(|(day, d)| {
                d.orders
                    .iter()
                    .map(move |o| format!("day {day}: {:?} | {}", o.kind, said(&o.reason)))
                    .chain(
                        d.no_orders
                            .iter()
                            .map(move |r| format!("day {day}: none | {}", said(r))),
                    )
            })
            .collect()
    }

    /// Contact 1's age in the picture of each of the five holders, at one
    /// moment: task force 1, the area command, task force 2, base 1 and the
    /// side's own; `None` where it is not held.
    fn ages_at(&self, moment: usize) -> [Option<u32>; 5] {
        let p = &self.ends[moment];
        let of = |held: &Vec<Held>| -> Option<u32> {
            match held.as_slice() {
                [] => None,
                [(1, h, 1, ShipKind::Carrier, age)] if *h == hex(12, 0) => Some(*age),
                other => panic!("moment {moment}: unexpected contacts {other:?}"),
            }
        };
        [
            of(&p.commands[&tf(1)].0),
            of(&p.commands[&AREA].0),
            of(&p.commands[&tf(2)].0),
            of(&p.commands[&bs(1)].0),
            of(&p.side.0),
        ]
    }
}

/// A "report sent" event: (day, pulse, sender, listeners).
type Sent = (
    u32,
    Option<Pulse>,
    CommandRef,
    Vec<(CommandRef, Direction, Arrival)>,
);

/// A reason's text: its calculation and its evidence.
fn said(reason: &Reason) -> String {
    let evidence: Vec<&str> = reason.evidence.iter().map(|e| e.what.as_str()).collect();
    format!("{} | {}", reason.worked, evidence.join(" | "))
}

const MORNING: Option<Pulse> = Some(Pulse::Morning);

/// The first "report sent" of the morning of day 1: task force 1's own
/// report, with its listeners.
fn first(listeners: Vec<(CommandRef, Direction, Arrival)>) -> Sent {
    (1, MORNING, tf(1), listeners)
}

/// The second: the area command passing it down.
fn second(arrival: Arrival) -> Sent {
    (
        1,
        MORNING,
        AREA,
        vec![
            (tf(2), Direction::Down, arrival),
            (bs(1), Direction::Down, arrival),
        ],
    )
}

fn up_on_time() -> (CommandRef, Direction, Arrival) {
    (AREA, Direction::Up, Arrival::OnTime)
}

/// The five holders' ages of contact 1 in each of the ten moments (the start,
/// then the end of each pulse in turn to the afternoon of day 3: the
/// night, morning, afternoon of day 1, ...). A row is [task force 1, the area
/// command, task force 2, base 1, the side's own]. Moments past the table's
/// end hold nothing.
type Table = Vec<[Option<u32>; 5]>;

fn assert_table(played: &Played, table: &Table) {
    assert_eq!(played.ends.len(), 10);
    for moment in 0..10 {
        let expected = table.get(moment).copied().unwrap_or([None; 5]);
        assert_eq!(played.ages_at(moment), expected, "moment {moment}");
    }
}

fn row(a: u32, b: u32, c: u32, d: u32, e: u32) -> [Option<u32>; 5] {
    [Some(a), Some(b), Some(c), Some(d), Some(e)]
}

const NONE5: [Option<u32>; 5] = [None; 5];

/// The pictures of the main case.
fn main_table() -> Table {
    vec![
        row(0, 0, 0, 0, 0),
        row(1, 1, 1, 1, 1),
        [None, None, Some(2), Some(2), None],
        [None, None, Some(3), Some(3), None],
        [None, None, Some(4), Some(4), None],
        NONE5,
    ]
}

// ------------------------------------------------------- The hand calculation

#[test]
fn the_file_is_scenario_61s_with_task_force_2_report_figures_and_a_name() {
    let t = text();
    assert!(t.contains(LEVELS_LINE));
    assert!(t.contains(REPORTS_LINE));
    assert!(t.contains("ship_classes = \"../reference/ship_classes.csv\""));
    let scenario = common::from_text_with(&t, &path(), &figures()).expect("loads");
    let world = &scenario.world;
    assert!(world.command_levels());
    let allied = world.side_view(Side::Allied);
    assert!(allied.reports_travel());
    assert_eq!(allied.commands(), vec![AREA, tf(1), tf(2), bs(1)]);
    for raw in [1, 2, 3] {
        assert_eq!(allied.command_of_group(group(raw)), Some(tf(1)));
    }
    for raw in [4, 5, 6] {
        assert_eq!(allied.command_of_group(group(raw)), Some(tf(2)));
    }
    let truth = world.umpire();
    assert_eq!(truth.task_forces[&force(2)].position(), hex(7, 0));
    assert_eq!(truth.task_forces[&force(1)].position(), hex(0, 0));
    assert_eq!(truth.air_groups[&group(4)].ready, 12);
    assert_eq!(truth.air_groups[&group(5)].ready, 6);
    assert_eq!(truth.air_groups[&group(6)].ready, 10);
    // 40 nm between the forces' places in the morning, 200 nm to the contact.
    assert_eq!(hex(6, 0).distance(hex(7, 0)) * 40, 40);
    assert_eq!(hex(7, 0).distance(hex(12, 0)) * 40, 200);
    // The figures of the report file: placeholders, up on time, the rest never.
    let figures = world.ref_data().reports.expect("report figures");
    assert_eq!(figures.up.on_time, Fixed::ONE);
    assert_eq!(figures.up.late, Fixed::ZERO);
    for none in [figures.sideways, figures.down] {
        assert_eq!(none.on_time, Fixed::ZERO);
        assert_eq!(none.late, Fixed::ZERO);
    }
    assert_eq!(figures.reach_carrier_nm, n(600));
    assert_eq!(figures.reach_base_nm, n(1000));
}

#[test]
fn twenty_orders_on_day_1_twelve_for_task_force_1_and_eight_for_task_force_2() {
    let played = play(&text());
    played.nothing_refused();
    assert_eq!(played.orders(1).len(), 20, "{:?}", played.orders(1));
    assert_eq!(played.orders_to(1, &[1], &[1, 2, 3]), tf1_day_1());
    assert_eq!(played.orders_to(1, &[2], &[4, 5, 6]), tf2_day_1());
    for day in 1..=3 {
        assert!(played.handed[&day].notices.is_empty(), "day {day}");
    }
}

#[test]
fn task_force_1_strikes_in_the_morning_and_task_force_2_launches_nothing() {
    let played = play(&text());
    assert_eq!(played.launches(), vec![(3, hex(12, 0), 10)]);
    let launch = played
        .events
        .iter()
        .find(|e| matches!(e.kind, EventKind::StrikeLaunched { .. }))
        .expect("a launch");
    assert_eq!((launch.day, launch.pulse), (1, MORNING));
    assert!(played
        .events
        .iter()
        .any(|e| matches!(e.kind, EventKind::StrikeFoundNothing { .. })
            && e.day == 1
            && e.pulse == MORNING));
}

#[test]
fn two_reports_sent_in_the_morning_of_day_1_with_three_listeners_and_two() {
    let played = play(&text());
    assert_eq!(
        played.sent(),
        vec![
            first(vec![
                up_on_time(),
                (tf(2), Direction::Sideways, Arrival::Never),
                (bs(1), Direction::Sideways, Arrival::Never),
            ]),
            second(Arrival::Never),
        ]
    );
}

#[test]
fn the_pictures_of_contact_1_by_command_and_moment() {
    let played = play(&text());
    assert_table(&played, &main_table());
    // Task force 2 and base 1 never heard the report: they heard none at all
    // (the contact they started with was never reported to them).
    // (A command forgets what it heard after as long as a contact is kept.)
    for moment in 2..=5 {
        let heard = &played.ends[moment].heard;
        assert_eq!(heard[&AREA], 1, "moment {moment}");
        assert_eq!(heard[&tf(1)], 1, "moment {moment}");
        assert_eq!(heard[&tf(2)], 0, "moment {moment}");
        assert_eq!(heard[&bs(1)], 0, "moment {moment}");
    }
}

#[test]
fn day_2_has_two_orders_and_task_force_2s_reason_names_contact_1_age_3_200_nm_and_itself() {
    let played = play(&text());
    assert_eq!(
        played.orders(2),
        vec![Given::Force(1, route(&[(0, 0)])), Given::Air(2, arc(0, 0))]
    );
    assert_eq!(played.orders_to(2, &[2], &[4, 5, 6]), Vec::<Given>::new());
    let reasons = played.reasons(2);
    let r = reasons
        .iter()
        .find(|r| r.contains("contact 1 held by task_force:2"))
        .unwrap_or_else(|| panic!("no reason names contact 1 as task force 2's: {reasons:#?}"));
    assert!(r.contains(" 3 "), "its age 3: {r}");
    assert!(r.contains("200"), "its distance 200 nm: {r}");
    // None of day 2's reasons names a contact as another command's.
    assert!(
        !reasons.iter().any(|r| r.contains("held by task_force:1")),
        "{reasons:#?}"
    );
}

#[test]
fn day_3_has_one_order_group_5s_arc_the_whole_circle_and_twenty_three_in_all() {
    let played = play(&text());
    assert_eq!(played.orders(3), vec![Given::Air(5, arc(0, 0))]);
    assert_eq!(played.order_count(), vec![20, 2, 1]);
    assert_eq!(played.order_count().iter().sum::<usize>(), 23);
}

// --------------------------------------------------------------- Properties

/// The game of "told sideways, on time": the sideways road on time.
fn told_sideways_game() -> Played {
    play(&with_figures(ON_TIME, ON_TIME, NEVER, 600))
}

fn assert_told_sideways_game(played: &Played, first_sent: Sent, second_sent: Sent) {
    played.nothing_refused();
    assert_eq!(played.sent(), vec![first_sent, second_sent]);
    // Task force 2 and base 1 drop it in the morning of day 1.
    let table: Table = vec![row(0, 0, 0, 0, 0), row(1, 1, 1, 1, 1), NONE5];
    assert_table(played, &table);
    // Task force 2 is given group 5's arc, the whole circle, on day 2 and
    // nothing on day 3: 20, 3 and 0.
    assert_eq!(played.order_count(), vec![20, 3, 0]);
    assert_eq!(played.orders_to(1, &[1], &[1, 2, 3]), tf1_day_1());
    assert_eq!(played.orders_to(1, &[2], &[4, 5, 6]), tf2_day_1());
    assert_eq!(
        played.orders_to(2, &[2], &[4, 5, 6]),
        vec![Given::Air(5, arc(0, 0))]
    );
    assert_eq!(
        played.orders_to(2, &[1], &[1, 2, 3]),
        vec![Given::Force(1, route(&[(0, 0)])), Given::Air(2, arc(0, 0))]
    );
}

#[test]
fn told_sideways_on_time() {
    let played = told_sideways_game();
    assert_told_sideways_game(
        &played,
        first(vec![
            up_on_time(),
            (tf(2), Direction::Sideways, Arrival::OnTime),
            (bs(1), Direction::Sideways, Arrival::OnTime),
        ]),
        second(Arrival::Never),
    );
}

#[test]
fn told_down_a_pulse_late() {
    let played = play(&with_figures(ON_TIME, NEVER, LATE, 600));
    played.nothing_refused();
    assert_eq!(
        played.sent(),
        vec![
            first(vec![
                up_on_time(),
                (tf(2), Direction::Sideways, Arrival::Never),
                (bs(1), Direction::Sideways, Arrival::Never),
            ]),
            second(Arrival::Late),
        ]
    );
    // They hold it at age 2 after the morning of day 1 and drop it at the
    // start of the afternoon, when the report arrives (the moment after the
    // morning is that start, before the report is applied).
    let table: Table = vec![
        row(0, 0, 0, 0, 0),
        row(1, 1, 1, 1, 1),
        [None, None, Some(2), Some(2), None],
        NONE5,
    ];
    assert_table(&played, &table);
    assert_eq!(played.order_count(), vec![20, 3, 0]);
    assert_eq!(played.orders_to(1, &[2], &[4, 5, 6]), tf2_day_1());
    assert_eq!(
        played.orders_to(2, &[2], &[4, 5, 6]),
        vec![Given::Air(5, arc(0, 0))]
    );
}

#[test]
fn out_of_reach_with_a_carriers_reach_of_30_nm_the_game_is_the_main_case() {
    let played = play(&with_figures(ON_TIME, ON_TIME, NEVER, 30));
    played.nothing_refused();
    // Task force 2, 40 nm away, and base 1 are not sent the report sideways.
    assert_eq!(
        played.sent(),
        vec![first(vec![up_on_time()]), second(Arrival::Never)]
    );
    assert_table(&played, &main_table());
    assert_eq!(played.order_count(), vec![20, 2, 1]);
    let main = play(&text());
    assert_eq!(played.orders_and_reasons(), main.orders_and_reasons());
    assert_eq!(played.events_but_sent(), main.events_but_sent());
}

#[test]
fn out_of_reach_with_a_reach_of_40_nm_task_force_2_is_told_and_base_1_is_not() {
    let played = play(&with_figures(ON_TIME, ON_TIME, NEVER, 40));
    played.nothing_refused();
    assert_eq!(
        played.sent(),
        vec![
            first(vec![
                up_on_time(),
                (tf(2), Direction::Sideways, Arrival::OnTime),
            ]),
            second(Arrival::Never),
        ]
    );
    // Task force 2 drops it in the morning; base 1, 223 nm away, holds it to
    // 5 pulses.
    let table: Table = vec![
        row(0, 0, 0, 0, 0),
        row(1, 1, 1, 1, 1),
        [None, None, None, Some(2), None],
        [None, None, None, Some(3), None],
        [None, None, None, Some(4), None],
        NONE5,
    ];
    assert_table(&played, &table);
    assert_eq!(played.order_count(), vec![20, 3, 0]);
}

#[test]
fn heard_once_by_two_roads_the_game_is_that_of_told_sideways() {
    let both = play(&with_figures(ON_TIME, ON_TIME, ON_TIME, 600));
    assert_told_sideways_game(
        &both,
        first(vec![
            up_on_time(),
            (tf(2), Direction::Sideways, Arrival::OnTime),
            (bs(1), Direction::Sideways, Arrival::OnTime),
        ]),
        second(Arrival::OnTime),
    );
    // The game is that of "told sideways": the same events but the "report
    // sent" ones, orders, reasons, pictures and truth, and each command has
    // heard as many reports (the one, once).
    let sideways = told_sideways_game();
    assert_eq!(both.events_but_sent(), sideways.events_but_sent());
    assert_eq!(both.orders_and_reasons(), sideways.orders_and_reasons());
    assert_eq!(both.truth, sideways.truth);
    for (moment, (a, b)) in both.ends.iter().zip(&sideways.ends).enumerate() {
        assert_eq!(a.commands, b.commands, "moment {moment}");
        assert_eq!(a.heard, b.heard, "moment {moment}");
    }
    assert_eq!(both.ends[2].heard[&tf(2)], 1);
    assert_eq!(both.ends[2].heard[&bs(1)], 1);
}

// The two properties of a late report's effect on a picture are proved in
// sim-core, where a picture and a report can be built directly: the
// functions that apply a report and age a picture are crate-private, and no
// scenario here holds a sighting. They are
// `passing_tests::a_late_sighting_is_one_pulse_old_then_one_and_then_two_as_the_makers_is`
// (a sighting made in pulse 4 that reaches a picture holding no contact at
// the start of pulse 5 makes a contact of age 1, 1 at the end of pulse 5 and
// 2 at the end of pulse 6, as the maker's is) and
// `passing_tests::newer_word_stands` (a sighting of an earlier event that
// arrives after a later one is heard and changes nothing: count, kind, age
// and last report event stay), in `crates/sim-core/src/passing_tests.rs`.
// What a scenario can show is that a report arriving late changes a picture
// at the start of the pulse after it was sent: `told_down_a_pulse_late`.

// ------------------------------------------- Every chance certain is step 1

/// An event as text with one change made to what a staff reason says: with
/// command levels on and report figures, a reason that names a contact says
/// which command holds it (decision 0135, 22), "contact 1 held by
/// task_force:2"; that clause is taken out, so that a game can be compared
/// with the same game with command levels off. The property "every chance
/// certain is step 1", as the architect restated it after the build, allows
/// exactly these words and the "report sent" events.
fn plain_text(kind: &EventKind) -> String {
    let text = format!("{kind:?}");
    let mut out = String::new();
    let mut rest = text.as_str();
    while let Some(at) = rest.find(" held by ") {
        out.push_str(&rest[..at]);
        rest = &rest[at + " held by ".len()..];
        if let Some(after) = rest.strip_prefix("the area command") {
            rest = after;
            continue;
        }
        let end = rest
            .find(|c: char| !(c.is_ascii_alphanumeric() || c == ':' || c == '_'))
            .unwrap_or(rest.len());
        rest = &rest[end..];
    }
    out.push_str(rest);
    out
}

/// Asserts two lists of events (day, pulse, kind) are the same but for the
/// clause "held by <command>" in a reason, saying where they first differ.
fn assert_same_events(
    a: &[(u32, Option<Pulse>, EventKind)],
    b: &[(u32, Option<Pulse>, EventKind)],
    what: &str,
) {
    assert_eq!(a.len(), b.len(), "{what}: the number of events");
    for i in 0..a.len() {
        let (x, y) = (&a[i], &b[i]);
        assert_eq!((x.0, x.1), (y.0, y.1), "{what}: event {i}");
        assert_eq!(plain_text(&x.2), plain_text(&y.2), "{what}: event {i}");
    }
}

/// The figures of a file in which every chance is on time 1: written under
/// the build directory, for a scenario text that names it by absolute path.
fn on_time_everywhere() -> PathBuf {
    figures_file(ON_TIME, ON_TIME, ON_TIME, 600)
}

/// Plays `text` to its end with `staffed`'s staffs on `seed`, returning the
/// events, the end truth as text, and the number of pulse ends at which every
/// command's picture held what its side's holds (checked as it goes when
/// `check_pictures` is set).
fn run_bigger(
    text: &str,
    path: &std::path::Path,
    seed: u64,
    check_pictures: bool,
) -> (Vec<Event>, String, usize) {
    let mut scenario = Scenario::from_text(text, path)
        .expect("the scenario loads")
        .with_seed(seed)
        .expect("the seed is set");
    let held = staffs();
    let mut checks = 0usize;
    loop {
        let staffs: Staffs<'_> = as_staffs(&held);
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
                        let own = side_picture(&view);
                        for c in view.commands() {
                            let v = view.command(c).expect("a listed command has a view");
                            assert_eq!(command_picture(&v), own, "{side:?} {c:?} at {moment:?}");
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

/// A bigger game: with command levels on and report figures with every
/// chance 1, it gives the events of the game with command levels off but for
/// the "report sent" events, and at the end of every pulse every command's
/// picture holds what its side's holds.
fn certain_is_step_1_in_a_bigger_game(file: &str, seeds: std::ops::RangeInclusive<u64>) {
    let path = common::data_dir().join(file);
    let off_text = std::fs::read_to_string(&path).expect("the scenario file reads");
    assert!(!off_text.contains("command_levels"));
    let on_text = format!(
        "{LEVELS_LINE}{}{off_text}",
        reports_line(&on_time_everywhere())
    );
    for seed in seeds {
        let (off_events, off_truth, _) = run_bigger(&off_text, &path, seed, false);
        let (on_events, on_truth, checks) = run_bigger(&on_text, &path, seed, true);
        assert!(checks > 0);
        let stripped: Vec<(u32, Option<Pulse>, EventKind)> = on_events
            .iter()
            .filter(|e| !matches!(e.kind, EventKind::ReportSent { .. }))
            .map(|e| (e.day, e.pulse, e.kind.clone()))
            .collect();
        let plain: Vec<(u32, Option<Pulse>, EventKind)> = off_events
            .iter()
            .map(|e| (e.day, e.pulse, e.kind.clone()))
            .collect();
        assert_same_events(&stripped, &plain, &format!("{file} seed {seed}"));
        assert_eq!(on_truth, off_truth, "{file} seed {seed}");
    }
}

#[test]
fn every_chance_certain_in_scenario_62_is_step_1() {
    let on = play(&with_figures(ON_TIME, ON_TIME, ON_TIME, 600));
    let plain = play(&off(&text()));
    assert!(on.reports_travel);
    for (moment, p) in on.ends.iter().enumerate() {
        for (c, picture) in &p.commands {
            assert_eq!(picture, &p.side, "{c:?} at moment {moment}");
        }
    }
    assert_same_events(
        &on.events_but_sent(),
        &plain.events_but_sent(),
        "scenario 62",
    );
    assert_eq!(on.truth, plain.truth);
    assert!(plain.sent().is_empty());
    assert!(!on.sent().is_empty());
}

#[test]
fn every_chance_certain_in_the_eastern_solomons_from_intents_on_seeds_1_to_10_is_step_1() {
    certain_is_step_1_in_a_bigger_game("scenarios/second-proof/05a_eastern_solomons.toml", 1..=10);
}

#[test]
fn every_chance_certain_in_the_week_of_the_slice_from_intents_on_seed_1_is_step_1() {
    certain_is_step_1_in_a_bigger_game("scenarios/second-proof/07a_week.toml", 1..=1);
}

// ------------------------------------------------- Report figures and levels

#[test]
fn with_no_report_figures_scenario_61_gives_what_it_gave_every_picture_is_the_sides() {
    let played = play(&without_reports(&text()));
    played.nothing_refused();
    assert!(!played.reports_travel);
    assert!(played.sent().is_empty());
    assert_eq!(played.allied_saw_sent, 0);
    for (moment, p) in played.ends.iter().enumerate() {
        assert_eq!(p.commands.len(), 4, "moment {moment}");
        for (c, picture) in &p.commands {
            assert_eq!(picture, &p.side, "{c:?} at moment {moment}");
        }
    }
    // Every command drops contact 1 in the morning of day 1, with the side's.
    let table: Table = vec![row(0, 0, 0, 0, 0), row(1, 1, 1, 1, 1), NONE5];
    assert_table(&played, &table);
}

#[test]
fn off_is_unchanged_with_command_levels_off_nothing_is_sent() {
    // The scenarios built before this one give the events they gave: the run
    // baselines and the scenarios' own tests cover that. Here, with command
    // levels off and the report figures still named, nothing is sent and no
    // command exists.
    let levels_off = common::replace_once(&text(), LEVELS_LINE, "");
    let played = play(&levels_off);
    assert!(played.sent().is_empty());
    assert_eq!(played.allied_saw_sent, 0);
    assert!(played.ends.iter().all(|p| p.commands.is_empty()));
    assert!(!played.reports_travel);
    let plain = play(&off(&text()));
    assert_eq!(played.events_but_sent(), plain.events_but_sent());
    assert_eq!(played.truth, plain.truth);
}

// ------------------------------------------------------------- The other side

#[test]
fn the_allied_side_sees_its_own_reports_sent_and_the_japanese_side_none() {
    let played = play(&text());
    assert_eq!(played.sent().len(), 2);
    assert_eq!(played.allied_saw_sent, 2);
    assert_eq!(played.japanese_saw_sent, 0);
}

#[test]
fn the_same_start_gives_the_same_game() {
    let a = play(&text());
    let b = play(&text());
    assert_eq!(a.handed, b.handed);
    assert_eq!(a.events, b.events);
    assert_eq!(a.truth, b.truth);
    assert_eq!(a.orders_and_reasons(), b.orders_and_reasons());
    for (x, y) in a.ends.iter().zip(&b.ends) {
        assert_eq!(x.commands, y.commands);
        assert_eq!(x.side, y.side);
    }
}

#[test]
fn two_worlds_that_differ_only_in_what_the_allied_side_cannot_see_give_the_same_decisions() {
    let hidden = format!(
        "{}\n[[base]]\nid = 3\nname = \"Placeholder base 3\"\nside = \"japanese\"\nq = 10\nr = 5\nsupply_t = 50\non_map = false\n\n[[air_group]]\nid = 7\nname = \"Placeholder Japanese fighters 7\"\nside = \"japanese\"\nbase = 3\naircraft_type = 4\nready = 20\n",
        text()
    );
    let plain = play(&text());
    let other = play(&hidden);
    assert_ne!(
        plain.truth.last().map(|t| t.len()),
        other.truth.last().map(|t| t.len()),
        "the two worlds do not differ"
    );
    assert_eq!(plain.handed, other.handed);
    assert_eq!(plain.orders_and_reasons(), other.orders_and_reasons());
    assert_eq!(plain.sent(), other.sent());
}
