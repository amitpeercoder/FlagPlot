//! Scenario 63: what a superior knows of his forces, and radio silence.
//! Scenario 58's carrier force, silent by the scenario, closes, strikes
//! and comes home, and its area command holds it where it last reported.
//!
//! Proves decision 0136, step 3 of stage 2b, against a hand calculation.
//! The data is placeholder data: `data/scenarios/63_a_superiors_knowledge.toml`
//! is scenario 58's file with command levels on, report figures of its
//! own and task force 1 set to radio silence. It runs on the frozen step 1
//! figures as scenario 58 does. There is no Japanese unit. Every chance in
//! the report figures is 0 or 1, so no outcome rests on a die.
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
//! As scenario 58. The report figures: on time 1 in every direction.
//!
//! ## The game
//!
//! Every order, arrival and launch is scenario 58's: twelve orders on day
//! 1, two on day 2, none on day 3. Task force 1, at press, is given no
//! order about radio silence: the staff leaves the setting as it finds
//! it. She is at (6,0) from hour 8 of day 1, strikes in the morning of
//! day 1, and is back at (0,0) from hour 32, in the night of day 2.
//!
//! ## What the area command holds of task force 1
//!
//! She is silent, so she sends a state report only in a pulse in which
//! she fights, is attacked or launches: the morning of day 1.
//!
//! | After | She is at | The area command holds her at | As of |
//! | --- | --- | --- | --- |
//! | The start | (0,0) | (0,0) | the start |
//! | The night of day 1 | (6,0) | (0,0) | the start |
//! | The morning of day 1 | (6,0) | (6,0) | the morning of day 1 |
//! | The afternoon of day 1 | (6,0) | (6,0) | the morning of day 1 |
//! | The night of day 2 | (0,0) | (6,0) | the morning of day 1 |
//! | Every later pulse | (0,0) | (6,0) | the morning of day 1 |
//!
//! What it holds of her ship's fuel and of her groups is what they were
//! at the end of the morning of day 1, in every later pulse: group 3 with
//! its 10 in the air or back as the truth then had them, and ship 1's
//! fuel after the 240 nm of the night and not after the 240 nm home.
//!
//! Base 1 is not silent: the area command holds it as of the pulse just
//! ended, after every pulse.
//!
//! ## The reports
//!
//! "Nothing there" at (12,0) is task force 1's, made in the morning of
//! day 1, a pulse in which she launched: she sends it. The area command
//! and base 1 drop contact 1 in the morning, as she does.
//!
//! No "state report late" and no "state report lost" event is recorded:
//! every one sent arrives on time, and those not sent are not sent.
//!
//! # Properties
//!
//! - **Not silent.** With the silence taken out: the area command holds
//!   task force 1 as of the pulse just ended, after every pulse: at (6,0)
//!   after the night of day 1 and at (0,0) after the night of day 2.
//! - **A pulse late.** With the silence taken out and up a pulse late 1:
//!   after each pulse the area command holds her as of the pulse before:
//!   after the night of day 1, as of the start, at (0,0); after the
//!   morning of day 1, as of the night, at (6,0); after the night of day
//!   2, as of the afternoon of day 1, at (6,0); after the morning of day
//!   2, as of the night of day 2, at (0,0). A "state report late" event in
//!   every pulse for each of task force 1 and base 1.
//! - **Never.** With the silence taken out and up never: the area command
//!   holds her at (0,0) as of the start in every pulse, and base 1
//!   likewise. A "state report lost" event in every pulse for each. The
//!   area command never hears "nothing there" and holds contact 1 to 5
//!   pulses.
//! - **A held report is sent when she breaks silence.** In a view built
//!   for it: a silent force that makes a sighting in pulse 3 and launches
//!   a strike in pulse 5 sends the sighting in pulse 5. Her superior's
//!   picture has no contact after pulses 3 and 4 and has it after pulse
//!   5, at age 2.
//! - **The staff sets silence at preserve the force.** With the scenario's
//!   silence taken out and intent 1 at preserve the force: the orders are
//!   those of the same file with command levels off, and one more on day
//!   1, radio silence on, to task force 1. With no report figures loaded
//!   the order is not given.
//! - **The commander's order.** `set_radio_silence` off, to task force 1
//!   on day 2, is taken: from the night of day 2 the area command holds
//!   her as of the pulse just ended.
//! - **The seat.** The area commander's list of his forces gives task
//!   force 1 at (6,0), "as of the morning of day 1", on day 3, and base 1
//!   with no such words.
//! - **An order that meets a different world.** In the main case, a
//!   commander's order on day 3 that routes task force 1 is taken or
//!   refused by the engine as it would be with command levels off; where
//!   it is refused for a cause the held state did not show, the refusal
//!   says what the area command held and that it was of the morning of
//!   day 1. Tested in a view built for it, with a force the area command
//!   holds afloat that is sunk.
//! - **Every chance certain and no silence is step 2.** Scenario 62 and
//!   scenario 61 give what they gave. On live figures with report figures
//!   of on time 1 in every direction, the Eastern Solomons from intents
//!   (seeds 1 to 10) and the week of the slice from intents (seed 1) give
//!   the orders and reasons they give with command levels off, but for
//!   the words decision 0135 added.
//! - **A supply run on stale word.** Scenario 36, keeping a base supplied,
//!   with command levels on and up never: what the area staff does is
//!   reported by the build, with its reasons, for the architect to read
//!   before the step is merged. It is not asserted here.
//! - **Off is unchanged.** Every scenario built before this one gives the
//!   events it gave.
//! - The same start gives the same game. The Japanese side sees none of
//!   the Allied side's state report events.

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::choices::Moment;
use sim_core::command::{Reason, StaffDecisions, StaffOrder};
use sim_core::events::{Arrival, Direction, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::held::{AsOf, HeldState};
use sim_core::hex::Hex;
use sim_core::ids::ShipId;
use sim_core::orders::{
    AirGroupOrder, AirMission, OrderKind, OrderRequest, Origin, Reach, Readiness, SpeedSetting,
    Stance, StragglerInstruction, TaskForceOrder,
};
use sim_core::pictures::CommandRef;
use sim_core::refdata::ShipKind;
use sim_core::side_events::SideEventKind;
use sim_core::state::{AirGroup, Side, TruthState};
use sim_core::time::Pulse;
use sim_core::view::SideView;
use sim_core::{Staffs, World};
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

use crate::common;
use crate::common::values::{base, force, fx, group, hex, n};

const LEVELS_LINE: &str = "command_levels = true\n";
const REPORTS_LINE: &str = "reports = \"../reference/reports_placeholder_scenario_63.toml\"\n";
const SILENCE_LINE: &str = "radio_silence = true\n";

fn path() -> PathBuf {
    common::data_dir().join("scenarios/63_a_superiors_knowledge.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
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
/// scenarios 58, 61 and 62 do.
fn figures() -> sim_data::LoadOptions {
    common::altered_reference("s63", |dir| {
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
/// directory (one for each call, for tests run in parallel): `up` as given,
/// sideways and down on time, as the scenario's own.
fn figures_file(up: Chances) -> PathBuf {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    std::fs::create_dir_all(&dir).expect("make the folder");
    let file = dir.join(format!(
        "reports_s63_{}_{}.toml",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let body = format!(
        "placeholder = true\n\n[up]\non_time = \"{}\"\nlate = \"{}\"\n\n[sideways]\non_time = \"1\"\nlate = \"0\"\n\n[down]\non_time = \"1\"\nlate = \"0\"\n\n[radio_reach_nm]\ncarrier = 600\nbattleship = 600\ncruiser = 600\nother_ship = 300\nbase = 1000\n",
        up.0, up.1
    );
    std::fs::write(&file, body).expect("write the report figures");
    file
}

fn reports_line(file: &std::path::Path) -> String {
    format!("reports = \"{}\"\n", file.display())
}

/// The scenario's text with `up` chances of its own.
fn with_up(text: &str, up: Chances) -> String {
    common::replace_once(text, REPORTS_LINE, &reports_line(&figures_file(up)))
}

fn without_silence(text: &str) -> String {
    common::replace_once(text, SILENCE_LINE, "")
}

fn without_reports(text: &str) -> String {
    common::replace_once(text, REPORTS_LINE, "")
}

/// The text with command levels, report figures and the silence all out: the
/// same file with command levels off.
fn off(text: &str) -> String {
    common::replace_once(&without_silence(&without_reports(text)), LEVELS_LINE, "")
}

/// A contact as (number, hex, ships, kind, age).
type Held = (u32, Hex, u32, ShipKind, u32);

fn contacts_of<'a>(
    contacts: impl Iterator<Item = &'a sim_core::belief::Contact>,
    number: impl Fn(sim_core::ids::ContactId) -> u32,
) -> Vec<Held> {
    contacts
        .map(|c| {
            (
                number(c.id),
                c.estimated_hex,
                c.estimated_ship_count,
                c.estimated_kind,
                c.age_pulses,
            )
        })
        .collect()
}

fn side_contacts(view: &SideView<'_>) -> Vec<Held> {
    contacts_of(view.contacts(), |id| view.contact_number(id))
}

fn command_contacts(view: &sim_core::view::CommandView<'_>) -> Vec<Held> {
    contacts_of(view.contacts(), |id| view.contact_number(id))
}

/// Contact 1's age in a list of contacts, or `None`.
fn age_of_contact_1(held: &[Held]) -> Option<u32> {
    match held {
        [] => None,
        [(1, h, 1, ShipKind::Carrier, age)] if *h == hex(12, 0) => Some(*age),
        other => panic!("unexpected contacts {other:?}"),
    }
}

/// The age of contact 1 in the picture of task force 1, the area command,
/// base 1 and the side's own.
type Ages = [Option<u32>; 4];

/// What the world is at the end of one pulse (or the start).
struct Sample {
    /// What the Allied area command holds, if reports travel.
    held: Option<BTreeMap<CommandRef, HeldState>>,
    /// The truth.
    truth: TruthState,
    /// Contact 1's age in [task force 1, the area command, base 1, the side].
    ages: Ages,
}

fn sample(world: &World) -> Sample {
    let view = world.side_view(Side::Allied);
    let of = |c: CommandRef| {
        let v = view.command(c).expect("a listed command has a view");
        age_of_contact_1(&command_contacts(&v))
    };
    Sample {
        held: world.held(Side::Allied).cloned(),
        truth: world.umpire().clone(),
        ages: [
            of(tf(1)),
            of(AREA),
            of(bs(1)),
            age_of_contact_1(&side_contacts(&view)),
        ],
    }
}

/// The moment the end of sample `i` is of: sample 0 is the start, then the
/// night, morning and afternoon of each day in turn.
fn moment(i: usize) -> AsOf {
    if i == 0 {
        return AsOf::Start;
    }
    AsOf::Pulse {
        day: ((i - 1) / 3 + 1) as u32,
        pulse: [Pulse::Night, Pulse::Morning, Pulse::Afternoon][(i - 1) % 3],
    }
}

const MORNING_1: AsOf = AsOf::Pulse {
    day: 1,
    pulse: Pulse::Morning,
};

/// A game played to its end with the doctrine's staffs.
struct Played {
    /// What the Allied staff handed in each day.
    handed: BTreeMap<u32, StaffDecisions>,
    events: Vec<Event>,
    /// The samples at the start, then at the end of each pulse in turn: 10.
    samples: Vec<Sample>,
    scenario: Scenario,
}

fn play(text: &str) -> Played {
    play_with(text, |_, _| {})
}

/// As [`play`], with `before` called with the day and the scenario before the
/// day is resolved: a commander's order goes in there.
fn play_with(text: &str, mut before: impl FnMut(u32, &mut Scenario)) -> Played {
    let mut scenario: Scenario =
        common::from_text_with(text, &path(), &figures()).expect("the scenario loads");
    let held = staffs();
    let mut samples: Vec<Sample> = Vec::new();
    let mut handed = BTreeMap::new();
    loop {
        let day = scenario.world.day() + 1;
        before(day, &mut scenario);
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |world, m| match m {
                Moment::DayBegins { day: 1 } => samples.push(sample(world)),
                Moment::PulseBegins { pulse, .. } if pulse != Pulse::Night => {
                    samples.push(sample(world));
                }
                _ => {}
            })
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
        // The afternoon's state reports are made at the end of the day.
        samples.push(sample(&scenario.world));
        handed.insert(
            day,
            scenario
                .world
                .staff_handed_in()
                .get(&Side::Allied)
                .cloned()
                .unwrap_or_default(),
        );
    }
    let events = scenario.world.events().iter().cloned().collect();
    assert_eq!(samples.len(), 10);
    Played {
        handed,
        events,
        samples,
        scenario,
    }
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

fn silence(on: bool) -> TaskForceOrder {
    TaskForceOrder::SetRadioSilence { on }
}

/// Task force 1's twelve orders of day 1, as scenario 58 has them.
fn day_1() -> Vec<Given> {
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

/// The two orders of day 2: the route back and group 2's arc, the whole
/// circle.
fn day_2() -> Vec<Given> {
    vec![Given::Force(1, route(&[(0, 0)])), Given::Air(2, arc(0, 0))]
}

impl Played {
    fn orders(&self, day: u32) -> Vec<Given> {
        self.handed[&day].orders.iter().map(given).collect()
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

    /// The state report events: (day, pulse, late or lost, command, as of).
    fn state_events(&self) -> Vec<(u32, Option<Pulse>, &'static str, CommandRef, AsOf)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::StateReportLate { command, as_of, .. } => {
                    Some((e.day, e.pulse, "late", *command, *as_of))
                }
                EventKind::StateReportLost { command, as_of, .. } => {
                    Some((e.day, e.pulse, "lost", *command, *as_of))
                }
                _ => None,
            })
            .collect()
    }

    /// Every order of the staff with its reason's text, as text.
    fn orders_and_reasons(&self) -> Vec<String> {
        orders_and_reasons(&self.handed)
    }

    /// What the area command holds of `command` after the pulse `i`.
    fn held(&self, i: usize, command: CommandRef) -> &HeldState {
        &self.samples[i].held.as_ref().expect("reports travel")[&command]
    }

    /// Where the area command holds task force 1 after the pulse `i`, and
    /// the moment that is of.
    fn tf1_held(&self, i: usize) -> (Hex, AsOf) {
        let state = self.held(i, tf(1));
        (
            state.task_force().expect("a force").position(),
            state.as_of(),
        )
    }

    /// Where task force 1 is, in truth, after the pulse `i`.
    fn tf1_truth(&self, i: usize) -> Hex {
        self.samples[i].truth.task_forces[&force(1)].position()
    }

    fn ship_1_fuel(&self, i: usize) -> Fixed {
        self.samples[i].truth.ships[&ShipId::new(1)].fuel_t
    }

    fn ship_1_fuel_held(&self, i: usize) -> Fixed {
        self.held(i, tf(1))
            .ships()
            .find(|s| s.id == ShipId::new(1))
            .expect("ship 1 is held")
            .fuel_t
    }

    fn group_3_held(&self, i: usize) -> &AirGroup {
        self.held(i, tf(1))
            .air_groups()
            .find(|g| g.id == group(3))
            .expect("group 3 is held")
    }
}

fn orders_and_reasons(handed: &BTreeMap<u32, StaffDecisions>) -> Vec<String> {
    handed
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

/// Asserts the area command holds task force 1, after each pulse `i`, where
/// and as of what `expected(i)` says.
fn assert_tf1_held(played: &Played, expected: impl Fn(usize) -> (Hex, AsOf)) {
    for i in 0..10 {
        assert_eq!(played.tf1_held(i), expected(i), "after pulse {i}");
    }
}

/// Where task force 1 is in truth after each pulse: at (0,0) at the start,
/// at (6,0) from the night of day 1 to the afternoon of day 1, home at (0,0)
/// from the night of day 2.
fn tf1_truth_hex(i: usize) -> Hex {
    match i {
        1..=3 => hex(6, 0),
        _ => hex(0, 0),
    }
}

// ------------------------------------------------------- The hand calculation

#[test]
fn the_file_is_scenario_58s_with_command_levels_report_figures_a_silent_force_and_a_name() {
    let t = text();
    assert!(t.contains(LEVELS_LINE));
    assert!(t.contains(REPORTS_LINE));
    assert!(t.contains(SILENCE_LINE));
    assert!(t.contains("placeholder = true"));
    assert!(t.contains("Placeholder scenario 63"));
    let scenario = common::from_text_with(&t, &path(), &figures()).expect("loads");
    let world = &scenario.world;
    assert!(world.command_levels());
    assert!(world.holds_as_reported());
    let allied = world.side_view(Side::Allied);
    assert!(allied.reports_travel());
    assert_eq!(allied.commands(), vec![AREA, tf(1), bs(1)]);
    assert_eq!(allied.radio_silence(force(1)), Some(true));
    assert!(world.radio_silence(force(1)));
    let truth = world.umpire();
    assert_eq!(truth.task_forces[&force(1)].position(), hex(0, 0));
    assert_eq!(truth.task_forces.len(), 1);
    // The figures of the report file: placeholders, on time in every
    // direction.
    let figures = world.ref_data().reports.expect("report figures");
    for chances in [figures.up, figures.sideways, figures.down] {
        assert_eq!(chances.on_time, Fixed::ONE);
        assert_eq!(chances.late, Fixed::ZERO);
    }
    // Everything is held exactly, as of the start.
    for command in [tf(1), bs(1)] {
        assert_eq!(
            world.held(Side::Allied).expect("holding")[&command].as_of(),
            AsOf::Start
        );
    }
    // The Japanese side has no unit.
    assert!(truth.task_forces.values().all(|f| f.side == Side::Allied));
}

#[test]
fn the_game_is_scenario_58s_twelve_orders_on_day_1_two_on_day_2_and_none_on_day_3() {
    let played = play(&text());
    played.nothing_refused();
    assert_eq!(played.order_count(), vec![12, 2, 0]);
    assert_eq!(played.orders(1), day_1());
    assert_eq!(played.orders(2), day_2());
    assert_eq!(played.orders(3), Vec::<Given>::new());
    // Task force 1 is at (6,0) from the first night and home at (0,0) from the
    // night of day 2; group 3 sends its 10 in the morning of day 1.
    for i in 0..10 {
        assert_eq!(played.tf1_truth(i), tf1_truth_hex(i), "after pulse {i}");
    }
    let launches: Vec<_> = played
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::StrikeLaunched {
                air_group,
                hex,
                aircraft,
                ..
            } => Some((e.day, e.pulse, air_group.raw(), *hex, *aircraft)),
            _ => None,
        })
        .collect();
    assert_eq!(launches, vec![(1, MORNING, 3, hex(12, 0), 10)]);
}

#[test]
fn the_area_command_holds_task_force_1_where_she_last_reported_and_as_of_then() {
    let played = play(&text());
    assert_tf1_held(&played, |i| match i {
        // The start, and the night of day 1: she sent nothing, so the area
        // command holds her where it began, though she is at (6,0).
        0 | 1 => (hex(0, 0), AsOf::Start),
        // She launched in the morning of day 1 and broke silence; she is
        // silent again after it, and the area command holds that report
        // from then on, to the end.
        _ => (hex(6, 0), MORNING_1),
    });
    // The hand calculation's table of where she is.
    assert_eq!(played.tf1_truth(1), hex(6, 0));
    assert_eq!(played.tf1_truth(3), hex(6, 0));
    assert_eq!(played.tf1_truth(4), hex(0, 0));
    for i in 4..10 {
        assert_eq!(played.tf1_truth(i), hex(0, 0));
        assert_eq!(played.tf1_held(i), (hex(6, 0), MORNING_1), "pulse {i}");
    }
}

#[test]
fn what_it_holds_of_her_ships_fuel_and_her_groups_is_as_at_the_end_of_the_morning_of_day_1() {
    let played = play(&text());
    // Ship 1's fuel: after the 240 nm of the night and not after the 240 nm
    // home, from the morning of day 1 on.
    let at_morning = played.ship_1_fuel(2);
    assert_eq!(at_morning, fx("2718.75"));
    assert_ne!(
        played.ship_1_fuel(9),
        at_morning,
        "she burned more going home"
    );
    for i in 2..10 {
        assert_eq!(played.ship_1_fuel_held(i), at_morning, "pulse {i}");
        // Group 3 as the truth had it at the end of the morning of day 1:
        // its 10, in the air or back.
        assert_eq!(
            *played.group_3_held(i),
            played.samples[2].truth.air_groups[&group(3)],
            "pulse {i}"
        );
    }
    // Until she reports, it holds what she was at the start.
    for i in 0..2 {
        assert_eq!(
            played.ship_1_fuel_held(i),
            played.ship_1_fuel(0),
            "pulse {i}"
        );
        assert_eq!(played.ship_1_fuel_held(i), n(3000));
        assert_eq!(
            *played.group_3_held(i),
            played.samples[0].truth.air_groups[&group(3)],
            "pulse {i}"
        );
    }
}

#[test]
fn base_1_is_held_as_of_the_pulse_just_ended_after_every_pulse() {
    let played = play(&text());
    for i in 0..10 {
        assert_eq!(played.held(i, bs(1)).as_of(), moment(i), "pulse {i}");
        assert!(played.held(i, bs(1)).base().is_some());
    }
}

#[test]
fn nothing_there_is_sent_and_dropped_by_the_area_command_and_base_1_in_the_morning() {
    let played = play(&text());
    // She launched in the morning, so she breaks silence and sends it: up to
    // the area command and sideways to base 1, and the area command passes it
    // down to base 1; all on time.
    assert_eq!(
        played.sent(),
        vec![
            (
                1,
                MORNING,
                tf(1),
                vec![
                    (AREA, Direction::Up, Arrival::OnTime),
                    (bs(1), Direction::Sideways, Arrival::OnTime),
                ],
            ),
            (
                1,
                MORNING,
                AREA,
                vec![(bs(1), Direction::Down, Arrival::OnTime)],
            ),
        ]
    );
    // Contact 1's age in task force 1, the area command, base 1 and the
    // side's own: all four drop it in the morning of day 1.
    let row = |a: u32| [Some(a); 4];
    assert_eq!(played.samples[0].ages, row(0));
    assert_eq!(played.samples[1].ages, row(1));
    for i in 2..10 {
        assert_eq!(played.samples[i].ages, [None; 4], "pulse {i}");
    }
}

#[test]
fn no_state_report_is_late_or_lost_in_the_main_case() {
    let played = play(&text());
    assert_eq!(played.state_events(), vec![]);
}

// --------------------------------------------------------------- Properties

#[test]
fn not_silent_the_area_command_holds_her_as_of_the_pulse_just_ended_after_every_pulse() {
    let played = play(&without_silence(&text()));
    played.nothing_refused();
    assert!(!played.scenario.world.radio_silence(force(1)));
    assert_tf1_held(&played, |i| (tf1_truth_hex(i), moment(i)));
    assert_eq!(played.tf1_held(1), (hex(6, 0), moment(1)));
    assert_eq!(played.tf1_held(4), (hex(0, 0), moment(4)));
    assert_eq!(played.state_events(), vec![]);
    // The game is the same: the orders are scenario 58's.
    assert_eq!(played.order_count(), vec![12, 2, 0]);
    // What she holds of fuel is the fuel of the pulse just ended.
    for i in 0..10 {
        assert_eq!(
            played.ship_1_fuel_held(i),
            played.ship_1_fuel(i),
            "pulse {i}"
        );
    }
}

#[test]
fn a_pulse_late_the_area_command_holds_her_as_of_the_pulse_before() {
    let played = play(&with_up(&without_silence(&text()), LATE));
    played.nothing_refused();
    // After pulse i it holds the state of the end of pulse i - 1: at the start
    // it holds the start.
    assert_tf1_held(&played, |i| {
        let before = i.saturating_sub(1);
        (tf1_truth_hex(before), moment(before))
    });
    assert_eq!(played.tf1_held(1), (hex(0, 0), AsOf::Start));
    assert_eq!(played.tf1_held(2), (hex(6, 0), moment(1)));
    assert_eq!(played.tf1_held(4), (hex(6, 0), moment(3)));
    assert_eq!(played.tf1_held(5), (hex(0, 0), moment(4)));
    for i in 0..10usize {
        let before = i.saturating_sub(1);
        assert_eq!(
            played.held(i, bs(1)).as_of(),
            moment(before),
            "base, pulse {i}"
        );
    }
    // A "state report late" event in every pulse for each of task force 1 and
    // base 1, each of the moment it is of.
    let events = played.state_events();
    assert_eq!(events.len(), 18);
    for i in 1..=9 {
        let (day, pulse) = moment(i).day_and_pulse().expect("a pulse");
        for command in [tf(1), bs(1)] {
            assert!(
                events.contains(&(day, Some(pulse), "late", command, moment(i))),
                "pulse {i} {command:?}: {events:?}"
            );
        }
    }
    assert!(events.iter().all(|e| e.2 == "late"));
}

#[test]
fn never_the_area_command_holds_the_start_and_never_hears_nothing_there() {
    let played = play(&with_up(&without_silence(&text()), NEVER));
    played.nothing_refused();
    assert_tf1_held(&played, |_| (hex(0, 0), AsOf::Start));
    for i in 0..10 {
        assert_eq!(
            played.held(i, bs(1)).as_of(),
            AsOf::Start,
            "base, pulse {i}"
        );
    }
    // A "state report lost" event in every pulse for each of them.
    let events = played.state_events();
    assert_eq!(events.len(), 18);
    for i in 1..=9 {
        let (day, pulse) = moment(i).day_and_pulse().expect("a pulse");
        for command in [tf(1), bs(1)] {
            assert!(
                events.contains(&(day, Some(pulse), "lost", command, moment(i))),
                "pulse {i} {command:?}: {events:?}"
            );
        }
    }
    assert!(events.iter().all(|e| e.2 == "lost"));
    // The area command holds contact 1 to 5 pulses; task force 1 and base 1
    // drop it in the morning of day 1, as she does.
    let area = |i: usize| played.samples[i].ages[1];
    for (i, expected) in [Some(0), Some(1), Some(2), Some(3), Some(4), None]
        .into_iter()
        .enumerate()
    {
        assert_eq!(area(i), expected, "pulse {i}");
    }
    assert_eq!(area(9), None);
    for i in 2..10 {
        assert_eq!(played.samples[i].ages[0], None, "task force 1, pulse {i}");
        assert_eq!(played.samples[i].ages[2], None, "base 1, pulse {i}");
        assert_eq!(played.samples[i].ages[3], None, "the side, pulse {i}");
    }
}

// A held report is sent when she breaks silence (a silent force that makes a
// sighting in pulse 3 and launches a strike in pulse 5 sends the sighting in
// pulse 5, and her superior's picture has no contact after pulses 3 and 4 and
// has it after pulse 5 at age 2) is proved in sim-core, in a view built for
// it: `held_tests::a_silent_force_sends_a_sighting_when_it_breaks_silence_as_old_as_it_is`
// in `crates/sim-core/src/held_tests.rs`. What a whole game shows of it is
// `nothing_there_is_sent_and_dropped_by_the_area_command_and_base_1_in_the_morning`.

#[test]
fn the_staff_sets_silence_at_preserve_the_force() {
    let preserve =
        |text: &str| common::replace_once(text, "risk = \"press\"", "risk = \"preserve\"");
    let on = play(&preserve(&without_silence(&text())));
    let plain = play(&preserve(&off(&text())));
    let silent_orders = |played: &Played, day: u32| -> Vec<Given> {
        played
            .orders(day)
            .into_iter()
            .filter(|g| matches!(g, Given::Force(_, TaskForceOrder::SetRadioSilence { .. })))
            .collect()
    };
    // One more order on day 1: radio silence on, to task force 1.
    assert_eq!(silent_orders(&on, 1), vec![Given::Force(1, silence(true))]);
    assert_eq!(silent_orders(&on, 2), vec![]);
    assert_eq!(silent_orders(&on, 3), vec![]);
    assert_eq!(silent_orders(&plain, 1), vec![]);
    for day in 1..=3 {
        let others: Vec<Given> = on
            .orders(day)
            .into_iter()
            .filter(|g| !matches!(g, Given::Force(_, TaskForceOrder::SetRadioSilence { .. })))
            .collect();
        assert_eq!(others, plain.orders(day), "day {day}");
    }
    assert_eq!(on.order_count()[0], plain.order_count()[0] + 1);
    assert!(on.scenario.world.radio_silence(force(1)));
    // With no report figures the order is not given.
    let no_figures = play(&preserve(&without_reports(&without_silence(&text()))));
    assert!(!no_figures
        .scenario
        .world
        .side_view(Side::Allied)
        .reports_travel());
    for day in 1..=3 {
        assert_eq!(no_figures.orders(day), plain.orders(day), "day {day}");
    }
}

#[test]
fn the_commanders_order_off_on_day_2_is_taken_and_she_is_held_as_of_the_pulse_just_ended_from_the_night(
) {
    let played = play_with(&text(), |day, scenario| {
        if day == 2 {
            scenario.world.submit_orders(
                Side::Allied,
                vec![OrderRequest {
                    kind: OrderKind::TaskForce {
                        target: force(1),
                        order: silence(false),
                    },
                    origin: Origin::PLAYER,
                }],
            );
        }
    });
    played.nothing_refused();
    assert!(!played.scenario.world.radio_silence(force(1)));
    assert!(played.events.iter().any(|e| matches!(
        e.kind,
        EventKind::OrderAccepted { target, .. } if target == sim_core::ids::EntityRef::TaskForce(force(1))
    ) && e.day == 2));
    assert_tf1_held(&played, |i| match i {
        0 | 1 => (hex(0, 0), AsOf::Start),
        2 | 3 => (hex(6, 0), MORNING_1),
        // From the night of day 2 she is not silent: the pulse just ended.
        _ => (hex(0, 0), moment(i)),
    });
}

/// A session of the Allied seat with the doctrine's staff, the scenario's
/// scripted commands submitted as the commander's, played to the morning of
/// `day`.
fn session_at(text: &str, day: u32) -> Session {
    let mut scenario =
        common::from_text_with(text, &path(), &figures()).expect("the scenario loads");
    let commands = scenario.scripted_commands().clone();
    scenario.replace_commands(BTreeMap::new());
    let name = scenario.name.clone();
    let mut session = Session::new(
        scenario.world,
        &name,
        scenario.days,
        Seat::Player,
        Seat::Script(BTreeMap::new()),
    )
    .with_staffs(staffs())
    .with_commands(commands);
    while session.day() + 1 < day {
        let a = session
            .call(Side::Allied, "end_day", &json!({}))
            .expect("a day ends");
        assert_eq!(a["resolved"], true);
    }
    session
}

#[test]
fn the_seat_on_day_3_is_given_task_force_1_as_of_the_morning_of_day_1_and_base_1_without() {
    let mut session = session_at(&text(), 3);
    assert_eq!(session.day(), 2);
    let list: Value = session
        .call(Side::Allied, "list_forces", &json!({}))
        .expect("list_forces");
    let forces = list["task_forces"].as_array().expect("forces");
    assert_eq!(forces.len(), 1, "{list}");
    let one = &forces[0];
    assert_eq!(one["task_force"]["handle"], "task_force:1", "{list}");
    assert_eq!(one["as_of"], "the morning of day 1", "{list}");
    assert_eq!(one["radio_silence"], true, "{list}");
    // She is at (6,0) as the area command holds her.
    let at = serde_json::to_string(&one["place"]).expect("place");
    assert!(
        at.contains("[6,0]") || at.contains("6,0") || at.contains("\"q\":6"),
        "{at}"
    );
    let bases: Value = session
        .call(Side::Allied, "list_bases", &json!({}))
        .expect("list_bases");
    let base_one = &bases["bases"][0];
    assert!(base_one.get("as_of").is_none(), "{bases}");
    assert!(!bases.to_string().contains("as of the morning of day 1"));
}

#[test]
fn an_order_that_meets_a_different_world_is_taken_or_refused_as_with_command_levels_off() {
    // On day 3 the area command holds task force 1 at (6,0) and she is at
    // (0,0). The commander routes her; the engine takes the order against the
    // truth, as it would with command levels off. (The part that needs a
    // world built for it, an order the truth refuses and the held state would
    // have accepted, is refused as "not as held" and says what the area
    // command held and the moment that was of: it is proved in
    // `held_tests::an_order_from_the_area_to_a_force_sunk_in_truth_but_held_afloat_is_refused_as_not_as_held`.)
    let routes = |day: u32, scenario: &mut Scenario| {
        if day == 3 {
            scenario.world.submit_orders(
                Side::Allied,
                vec![
                    OrderRequest {
                        kind: OrderKind::TaskForce {
                            target: force(1),
                            order: route(&[(3, 0)]),
                        },
                        origin: Origin::PLAYER,
                    },
                    OrderRequest {
                        kind: OrderKind::TaskForce {
                            target: force(1),
                            order: route(&[(500, 500)]),
                        },
                        origin: Origin::PLAYER,
                    },
                ],
            );
        }
    };
    let on = play_with(&text(), routes);
    let plain = play_with(&off(&text()), routes);
    let outcomes = |played: &Played| -> Vec<String> {
        played
            .events
            .iter()
            .filter(|e| {
                e.day == 3
                    && matches!(
                        e.kind,
                        EventKind::OrderAccepted { .. } | EventKind::OrderRejected { .. }
                    )
            })
            .map(|e| format!("{:?}", e.kind))
            .collect()
    };
    let on_outcomes = outcomes(&on);
    assert_eq!(on_outcomes, outcomes(&plain));
    assert!(!on_outcomes.is_empty());
    for e in &on.events {
        if let EventKind::OrderRejected { reason, .. } = &e.kind {
            assert!(
                !reason.is_not_as_held(),
                "the truth's refusal, not a held one: {reason:?}"
            );
        }
    }
}

// ------------------------------------- Every chance certain and no silence

/// An event as text with the clause "held by <command>" of a staff reason
/// taken out (decision 0135, 22), so that a game can be compared with the
/// same game with command levels off. The property allows exactly these
/// words and the "report sent" events (as scenario 62's test of the same
/// property has it).
fn plain_text(kind: &EventKind) -> String {
    plain_str(&format!("{kind:?}"))
}

fn plain_str(text: &str) -> String {
    let mut out = String::new();
    let mut rest = text;
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

#[test]
fn every_chance_certain_and_no_silence_in_scenario_63_is_step_2() {
    let on = play(&without_silence(&text()));
    let plain = play(&off(&text()));
    assert_same_events(
        &on.events_but_sent(),
        &plain.events_but_sent(),
        "scenario 63",
    );
    assert_eq!(
        format!("{:?}", on.scenario.world.umpire()),
        format!("{:?}", plain.scenario.world.umpire())
    );
    // The orders and reasons are the same but for the words of decision 0135.
    let (a, b) = (on.orders_and_reasons(), plain.orders_and_reasons());
    assert_eq!(a.len(), b.len());
    for (x, y) in a.iter().zip(&b) {
        assert_eq!(plain_str(x), plain_str(y));
    }
    assert!(plain.sent().is_empty());
    assert!(!on.sent().is_empty());
    // Every command holds what the side holds, and the area command holds
    // every command as of the pulse just ended.
    for i in 0..10 {
        let ages = on.samples[i].ages;
        assert_eq!(ages[0], ages[3], "pulse {i}");
        assert_eq!(ages[1], ages[3], "pulse {i}");
        assert_eq!(ages[2], ages[3], "pulse {i}");
    }
}

// Scenarios 61 and 62 give what they gave: their own tests prove it
// (`tests/scenario_61_commands_and_their_pictures.rs`,
// `tests/scenario_62_reports_between_commands.rs`), and nothing of this step
// changes a game with no silence and every chance certain.

/// Plays `text` to its end with the doctrine's staffs on `seed`, returning the
/// events, the end truth as text, the orders and reasons the Allied and
/// Japanese staffs handed in each day, as text, and the number of pulse ends
/// at which every command was held as of the pulse just ended (checked as it
/// goes when `check_held` is set).
fn run_bigger(
    text: &str,
    path: &std::path::Path,
    seed: u64,
    check_held: bool,
) -> (Vec<Event>, String, Vec<String>, usize) {
    let mut scenario = Scenario::from_text(text, path)
        .expect("the scenario loads")
        .with_seed(seed)
        .expect("the seed is set");
    let held = staffs();
    let mut checks = 0usize;
    let mut handed: Vec<String> = Vec::new();
    loop {
        let staffs: Staffs<'_> = as_staffs(&held);
        let summary = scenario
            .next_day_watched(staffs, &mut |world, moment| {
                let just_ended = match moment {
                    Moment::DayBegins { day } => Some(match day {
                        1 => AsOf::Start,
                        d => AsOf::Pulse {
                            day: d - 1,
                            pulse: Pulse::Afternoon,
                        },
                    }),
                    Moment::PulseBegins { day, pulse } => match pulse {
                        Pulse::Night => None,
                        Pulse::Morning => Some(AsOf::Pulse {
                            day,
                            pulse: Pulse::Night,
                        }),
                        Pulse::Afternoon => Some(AsOf::Pulse {
                            day,
                            pulse: Pulse::Morning,
                        }),
                    },
                    _ => None,
                };
                if let (true, Some(just_ended)) = (check_held, just_ended) {
                    for side in Side::ALL {
                        let held = world.held(side).expect("reports travel");
                        for (command, state) in held {
                            assert_eq!(
                                state.as_of(),
                                just_ended,
                                "{side:?} {command:?} at {moment:?}"
                            );
                            checks += 1;
                        }
                    }
                }
            })
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
        for side in Side::ALL {
            let decisions = scenario
                .world
                .staff_handed_in()
                .get(&side)
                .cloned()
                .unwrap_or_default();
            let day = scenario.world.day();
            handed.extend(
                orders_and_reasons(&BTreeMap::from([(day, decisions)]))
                    .into_iter()
                    .map(|line| format!("{side:?} {line}")),
            );
        }
    }
    (
        scenario.world.events().iter().cloned().collect(),
        format!("{:?}", scenario.world.umpire()),
        handed,
        checks,
    )
}

/// A bigger game on live figures: with command levels on and report figures
/// whose every chance is on time 1, and no silence, it gives the orders and
/// reasons, and the events, of the game with command levels off but for the
/// "report sent" events and the words decision 0135 added, and every command
/// is held as of the pulse just ended.
fn certain_is_step_2_in_a_bigger_game(file: &str, seeds: std::ops::RangeInclusive<u64>) {
    let path = common::data_dir().join(file);
    let off_text = std::fs::read_to_string(&path).expect("the scenario file reads");
    assert!(!off_text.contains("command_levels"));
    assert!(!off_text.contains("radio_silence"));
    let on_text = format!(
        "{LEVELS_LINE}{}{off_text}",
        reports_line(&figures_file(ON_TIME))
    );
    for seed in seeds {
        let (off_events, off_truth, off_handed, _) = run_bigger(&off_text, &path, seed, false);
        let (on_events, on_truth, on_handed, checks) = run_bigger(&on_text, &path, seed, true);
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
        assert_eq!(on_handed.len(), off_handed.len(), "{file} seed {seed}");
        for (a, b) in on_handed.iter().zip(&off_handed) {
            assert_eq!(plain_str(a), plain_str(b), "{file} seed {seed}");
        }
        assert!(
            !on_events.iter().any(|e| matches!(
                e.kind,
                EventKind::StateReportLate { .. } | EventKind::StateReportLost { .. }
            )),
            "{file} seed {seed}: a state report was late or lost"
        );
    }
}

#[test]
fn every_chance_certain_in_the_eastern_solomons_from_intents_on_seeds_1_to_10_is_step_2() {
    certain_is_step_2_in_a_bigger_game("scenarios/second-proof/05a_eastern_solomons.toml", 1..=10);
}

#[test]
fn every_chance_certain_in_the_week_of_the_slice_from_intents_on_seed_1_is_step_2() {
    certain_is_step_2_in_a_bigger_game("scenarios/second-proof/07a_week.toml", 1..=1);
}

// ---------------------------------------------------------- Off is unchanged

#[test]
fn every_scenario_file_but_61_62_and_63_loads_with_command_levels_off_and_nothing_held() {
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
        // Scenarios 61, 62 and 63 turn command levels on, and scenario 64
        // (decision 0137), built after this one, does too.
        if ["61_", "62_", "63_", "64_"]
            .iter()
            .any(|p| name.starts_with(p))
        {
            assert!(text.contains(LEVELS_LINE), "{name}");
            continue;
        }
        seen += 1;
        assert!(
            !text.contains("command_levels"),
            "{name} names command levels"
        );
        assert!(!text.contains("radio_silence"), "{name} sets silence");
        assert!(!text.contains("\nreports ="), "{name} names report figures");
        let scenario = Scenario::load(&path).unwrap_or_else(|e| panic!("{name}: {e}"));
        let world = &scenario.world;
        assert!(!world.command_levels(), "{name}");
        assert!(!world.holds_as_reported(), "{name}");
        assert!(world.held(Side::Allied).is_none(), "{name}");
        assert!(world.held(Side::Japanese).is_none(), "{name}");
        for force in world.umpire().task_forces.keys() {
            assert!(!world.radio_silence(*force), "{name}");
        }
    }
    assert!(seen > 50, "{seen} scenario files");
}

#[test]
fn with_command_levels_off_the_silence_key_is_no_state_report_and_the_game_is_the_plain_one() {
    // Silence set where reports do not travel has no effect: nothing is held,
    // nothing is sent, and the game is the plain one.
    let levels_off = common::replace_once(&text(), LEVELS_LINE, "");
    let played = play(&levels_off);
    assert!(played.scenario.world.held(Side::Allied).is_none());
    assert!(played.sent().is_empty());
    assert!(played.state_events().is_empty());
    let plain = play(&off(&text()));
    assert_eq!(played.events_but_sent(), plain.events_but_sent());
    assert_eq!(
        format!("{:?}", played.scenario.world.umpire()),
        format!("{:?}", plain.scenario.world.umpire())
    );
}

// ----------------------------------------------------------------- The rest

#[test]
fn the_same_start_gives_the_same_game() {
    let a = play(&text());
    let b = play(&text());
    assert_eq!(a.handed, b.handed);
    assert_eq!(a.events, b.events);
    assert_eq!(a.orders_and_reasons(), b.orders_and_reasons());
    for (x, y) in a.samples.iter().zip(&b.samples) {
        assert_eq!(x.held, y.held);
        assert_eq!(x.ages, y.ages);
    }
    assert_eq!(
        format!("{:?}", a.scenario.world.umpire()),
        format!("{:?}", b.scenario.world.umpire())
    );
}

#[test]
fn the_japanese_side_sees_none_of_the_allied_sides_state_report_events() {
    // Up never arrives and the silence is out, so the Allied side has 18
    // "state report lost" events; with up a pulse late it has 18 "late".
    for up in [NEVER, LATE] {
        let played = play(&with_up(&without_silence(&text()), up));
        assert_eq!(played.state_events().len(), 18);
        let seen = |side: Side| {
            played
                .scenario
                .world
                .side_view(side)
                .events()
                .iter()
                .filter(|e| {
                    matches!(
                        e.kind,
                        SideEventKind::Whole(
                            EventKind::StateReportLate { .. } | EventKind::StateReportLost { .. }
                        )
                    )
                })
                .count()
        };
        assert_eq!(seen(Side::Allied), 18);
        assert_eq!(seen(Side::Japanese), 0);
    }
}
