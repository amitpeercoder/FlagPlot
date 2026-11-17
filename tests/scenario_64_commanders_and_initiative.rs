//! Scenario 64: commanders and initiative. One carrier force at balanced
//! holds a cruiser it sighted itself. A steady commander stays; a bold one
//! closes; and a cautious one will not close even on a carrier.
//!
//! Proves decision 0137, step 4 of stage 2b, against a hand calculation.
//! The data is placeholder data: `data/scenarios/64_commanders_and_initiative.toml`
//! is scenario 58's file with command levels on, report figures of on
//! time 1 in every direction, intent 1 with no risk set, so balanced, and
//! the starting contact a cruiser that task force 1 saw: it is in task
//! force 1's picture alone, as its own. It runs on the frozen step 1
//! figures as scenario 58 does. There is no Japanese unit, so no die is
//! rolled.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Figures and inputs
//!
//! As scenario 58: task force 1, one carrier, at (0,0), with groups 1, 2
//! and 3; base 1 at (0,5); 3 days. The contact: 1 ship, a cruiser, at
//! (12,0), age 0, 480 nm away against a reach of 250 and a night at full
//! speed of 360. Balanced closes on a carrier; press closes on any kind;
//! preserve the force closes on nothing.
//!
//! ## The main case: a bold commander
//!
//! Task force 1's commander is "Placeholder Bold", bold.
//!
//! **Day 1.** The contact is his own. He weighs it at press, one level
//! bolder than balanced: press closes on a cruiser. He closes, as scenario
//! 58's force does at balanced on a carrier: the first hex within reach
//! is (6,0), full speed, there at hour 8. Everything else is balanced's:
//! a break-off level of 0.5, a patrol share of 0.5, and two orders to
//! group 3. Ten orders: the force, four (battle readiness, rules of
//! engagement, full speed, the route to (6,0)); group 1, two; group 2,
//! two; group 3, two. She launches nothing: the contact is a pulse old in
//! the morning against the figure of 0.
//!
//! The reason for the route names Placeholder Bold, bold, says the
//! contact is task force 1's own, and says it was weighed at press where
//! the task's risk is balanced.
//!
//! **Day 2.** She is at (6,0), having left her hex to close. The contact
//! is 3 pulses old and 240 nm away, in reach, and, weighed at press, of a
//! kind he closes on: she stays. No order. The contact is dropped at the
//! end of the morning, at 5 pulses.
//!
//! **Day 3.** She holds none and goes back: two orders, the route to
//! (0,0) and group 2's arc, the whole circle. She arrives at hour 56.
//!
//! Ten orders, none and two: twelve.
//!
//! ## The pictures
//!
//! Contact 1 is in task force 1's picture and in the side's own. The area
//! command and base 1 hold no contact at any time: a contact a scenario
//! says one command saw is sent to no one.
//!
//! # Properties
//!
//! - **Steady.** With no commander named: balanced does not close on a
//!   cruiser. She stays at (0,0) and is given eight orders on day 1 (the
//!   ten less the speed and the route), none on day 2, and one on day 3,
//!   group 2's arc, the whole circle, the contact having been dropped at
//!   the end of the morning of day 2. The reason for not closing is
//!   scenario 58's for a kind it does not close on, and names no
//!   commander.
//! - **Cautious, and a carrier.** With the contact a carrier and the
//!   commander "Placeholder Cautious", cautious: he weighs his own contact
//!   at preserve the force, which closes on nothing. She stays: eight
//!   orders on day 1. The reason names him, cautious, and the level. With
//!   no commander and the same carrier she closes: ten orders, as scenario
//!   58 at balanced.
//! - **A contact he was told of.** With the bold commander and the cruiser
//!   an ordinary starting contact, in every command's picture: it is not
//!   his own. He weighs it at balanced and stays: eight orders on day 1,
//!   and the reason names no rating.
//! - **Press stays press, preserve stays preserve.** With the bold
//!   commander and intent 1 at press: the orders are scenario 58's twelve
//!   on day 1. With a cautious commander and intent 1 at preserve the
//!   force: the orders are those of the same file with no commander.
//! - **Bold at preserve the force.** With the bold commander, intent 1 at
//!   preserve the force and the contact a carrier, his own: he weighs it
//!   at balanced, which closes on a carrier, and the speed is preserve's:
//!   always cruise. She is routed to (6,0) with no speed order and arrives
//!   at hour 15, in the morning of day 1.
//! - **Command levels off.** The main case's file with command levels off:
//!   the commander is not read. The contact is the side's, a cruiser, at
//!   balanced: she stays, eight orders on day 1.
//! - **A rating without its note is refused** by the loader, and so is a
//!   commander for a command the scenario does not have.
//! - **Nothing else reads a rating.** Scenarios 61, 62 and 63 give what
//!   they gave, and every scenario built before this one gives the events
//!   it gave.
//! - The same start gives the same game.

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::choices::Moment;
use sim_core::command::{Reason, StaffDecisions, StaffOrder};
use sim_core::commander::Initiative;
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, TaskForceId};
use sim_core::orders::{
    AirGroupOrder, AirMission, OrderKind, Readiness, SpeedSetting, Stance, StragglerInstruction,
    TaskForceOrder,
};
use sim_core::pictures::CommandRef;
use sim_core::refdata::ShipKind;
use sim_core::state::Side;
use sim_core::time::Pulse;
use sim_core::view::{CommandView, SideView};
use sim_core::World;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

mod common;

const LEVELS_LINE: &str = "command_levels = true\n";
const REPORTS_LINE: &str = "reports = \"../reference/reports_placeholder_scenario_64.toml\"\n";
const SEEN_BY_LINE: &str = "seen_by = { task_force = 1 }\n";
const KIND_LINE: &str = "kind = \"cruiser\"\n";
const NAME_LINE: &str = "name = \"Placeholder Bold\"\n";
const RATING_LINE: &str = "initiative = \"bold\"\n";
const NOTE_LINE: &str = "note = \"Placeholder: invented for scenario 64's hand calculation; no source rates any commander\"\n";
const COMMANDER_MARKER: &str = "# Task force 1's commander.";
const COMMANDS_MARKER: &str = "# The Allied commander's commands";

fn path() -> PathBuf {
    common::data_dir().join("scenarios/64_commanders_and_initiative.toml")
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

fn force(raw: u32) -> TaskForceId {
    TaskForceId::new(raw)
}

fn hex(q: i32, r: i32) -> Hex {
    Hex { q, r }
}

fn tf(raw: u32) -> CommandRef {
    CommandRef::TaskForce(force(raw))
}

const AREA: CommandRef = CommandRef::Area;
const BASE_1: CommandRef = CommandRef::Base(BaseId::new(1));

/// The scenario's own figures: scenario 53's table of aircraft for each ship,
/// as scenario 58 has them.
fn figures() -> sim_data::LoadOptions {
    common::altered_reference("s64", |dir| {
        common::edit_file(dir, "air_combat.toml", |text| {
            format!("{text}\ncruiser = 12\ndestroyer = 6\ntransport = 6\nauxiliary = 4\n")
        })
    })
}

fn load(text: &str) -> Result<Scenario, sim_data::DataError> {
    common::from_text_with(text, &path(), &figures())
}

// ----------------------------------------------------------- The variants

/// The file with no commander: steady.
fn no_commander(text: &str) -> String {
    let from = text.find(COMMANDER_MARKER).expect("the commander");
    let to = text.find(COMMANDS_MARKER).expect("the commands");
    format!("{}{}", &text[..from], &text[to..])
}

/// The file with the commander's name and rating changed.
fn commander(text: &str, name: &str, rating: &str) -> String {
    let t = common::replace_once(text, NAME_LINE, &format!("name = \"{name}\"\n"));
    common::replace_once(&t, RATING_LINE, &format!("initiative = \"{rating}\"\n"))
}

fn carrier(text: &str) -> String {
    common::replace_once(text, KIND_LINE, "kind = \"carrier\"\n")
}

/// The contact an ordinary starting contact, in every command's picture.
fn told(text: &str) -> String {
    common::replace_once(text, SEEN_BY_LINE, "")
}

/// Intent 1 at `risk`: scenario 58's risk limit, given after the intent.
fn at_risk(text: &str, risk: &str) -> String {
    common::replace_once(
        text,
        "hex = [0, 0]\n",
        &format!(
            "hex = [0, 0]\n\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"{risk}\"\n"
        ),
    )
}

/// The file with command levels off: no command levels line and no report
/// figures.
fn off(text: &str) -> String {
    let t = common::replace_once(text, LEVELS_LINE, "");
    common::replace_once(&t, REPORTS_LINE, "")
}

// ----------------------------------------------------------- The game

/// A contact as (number, hex, ships, kind, age, own).
type Held = (u32, Hex, u32, ShipKind, u32, bool);

fn command_contacts(view: &CommandView<'_>) -> Vec<Held> {
    view.contacts()
        .map(|c| {
            (
                view.contact_number(c.id),
                c.estimated_hex,
                c.estimated_ship_count,
                c.estimated_kind,
                c.age_pulses,
                view.is_own(c.id),
            )
        })
        .collect()
}

fn side_contacts(view: &SideView<'_>) -> Vec<(u32, Hex, ShipKind, u32)> {
    view.contacts()
        .map(|c| {
            (
                view.contact_number(c.id),
                c.estimated_hex,
                c.estimated_kind,
                c.age_pulses,
            )
        })
        .collect()
}

/// The contacts of task force 1, the area command, base 1 and the side.
struct Pictures {
    tf1: Vec<Held>,
    area: Vec<Held>,
    base_1: Vec<Held>,
    side: Vec<(u32, Hex, ShipKind, u32)>,
}

fn pictures(world: &World) -> Pictures {
    let view = world.side_view(Side::Allied);
    let of = |c: CommandRef| command_contacts(&view.command(c).expect("a command"));
    Pictures {
        tf1: of(tf(1)),
        area: of(AREA),
        base_1: of(BASE_1),
        side: side_contacts(&view),
    }
}

/// A game played to its end with the doctrine's staffs.
struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    events: Vec<Event>,
    /// The pictures at the start, then at the start of each daylight pulse
    /// and at the end of each day: "1 Morning", "1 Afternoon", "1 End", ...
    pictures: Vec<(String, Pictures)>,
    hash: u64,
}

fn play(text: &str) -> Played {
    let mut scenario = load(text).expect("the scenario loads");
    let held = staffs();
    let mut handed = BTreeMap::new();
    let mut seen = vec![("start".to_string(), pictures(&scenario.world))];
    loop {
        let day = scenario.world.day() + 1;
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |world, m| {
                if let Moment::PulseBegins { day, pulse } = m {
                    if pulse != Pulse::Night {
                        seen.push((format!("{day} {pulse:?}"), pictures(world)));
                    }
                }
            })
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
        seen.push((format!("{day} End"), pictures(&scenario.world)));
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
    Played {
        handed,
        events: scenario.world.events().iter().cloned().collect(),
        pictures: seen,
        hash: scenario.world.state_hash(),
    }
}

fn mission(m: AirMission) -> AirGroupOrder {
    AirGroupOrder::SetAirMission(m)
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

fn full() -> Given {
    Given::Force(1, TaskForceOrder::SetSpeed(SpeedSetting::Full))
}

fn is_speed(o: &Given) -> bool {
    matches!(o, Given::Force(_, TaskForceOrder::SetSpeed(_)))
}

fn is_route(o: &Given) -> bool {
    matches!(o, Given::Force(_, TaskForceOrder::SetRoute { .. }))
}

/// The ten orders of day 1 at balanced when she closes.
fn closing_day_1() -> Vec<Given> {
    vec![
        Given::Force(1, BATTLE),
        Given::Force(1, rules(Stance::Avoid, "0.5")),
        full(),
        Given::Force(1, route(&[(6, 0)])),
        Given::Air(1, mission(CAP)),
        Given::Air(1, share("0.5")),
        Given::Air(2, mission(SEARCH)),
        Given::Air(2, arc(50, 120)),
        Given::Air(3, mission(STRIKE)),
        Given::Air(3, AirGroupOrder::SetStrikeSizing { on: true }),
    ]
}

/// The eight orders of day 1 at balanced when she stays: the ten less the
/// speed and the route.
fn staying_day_1() -> Vec<Given> {
    closing_day_1()
        .into_iter()
        .filter(|o| !is_speed(o) && !is_route(o))
        .collect()
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

    fn launches(&self) -> usize {
        self.events
            .iter()
            .filter(|e| matches!(e.kind, EventKind::StrikeLaunched { .. }))
            .count()
    }

    /// The force's arrivals, as (hex, hours since the start of the game).
    fn arrivals(&self) -> Vec<(Hex, Fixed)> {
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

    /// The reason given with the route to (6,0) on day 1.
    fn route_reason(&self) -> String {
        self.handed[&1]
            .orders
            .iter()
            .find(|o| given(o) == Given::Force(1, route(&[(6, 0)])))
            .map(|o| said(&o.reason))
            .expect("a route to (6,0)")
    }

    /// Every reason of the staff on `day`, of an order or of a decision not
    /// to order.
    fn reasons(&self, day: u32) -> Vec<String> {
        let d = &self.handed[&day];
        d.orders
            .iter()
            .map(|o| said(&o.reason))
            .chain(d.no_orders.iter().map(said))
            .collect()
    }

    fn all_reasons(&self) -> Vec<String> {
        (1..=self.handed.len() as u32)
            .flat_map(|d| self.reasons(d))
            .collect()
    }

    /// The one reason of `day` that says whether she closes.
    fn closing_reason(&self, day: u32) -> String {
        let found: Vec<String> = self
            .reasons(day)
            .into_iter()
            .filter(|r| {
                r.contains("does not close")
                    || r.contains("is routed to (6,0)")
                    || r.contains("stays where it is")
            })
            .collect();
        assert_eq!(found.len(), 1, "{found:?}");
        found[0].clone()
    }

    /// Every order with its reason's text.
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

    fn event_kinds(&self) -> Vec<(u32, Option<Pulse>, EventKind)> {
        self.events
            .iter()
            .map(|e| (e.day, e.pulse, e.kind.clone()))
            .collect()
    }
}

/// A reason's text: its calculation, its evidence and its bindings.
fn said(reason: &Reason) -> String {
    let evidence: Vec<&str> = reason.evidence.iter().map(|e| e.what.as_str()).collect();
    let binding: Vec<&str> = reason.binding.iter().map(String::as_str).collect();
    format!(
        "{} | {} | {}",
        reason.worked,
        evidence.join(" | "),
        binding.join(" | ")
    )
}

// ------------------------------------------------------- The hand calculation

#[test]
fn the_file_is_scenario_58s_with_command_levels_a_commander_and_a_contact_of_his_own() {
    let t = text();
    for line in [
        LEVELS_LINE,
        REPORTS_LINE,
        SEEN_BY_LINE,
        KIND_LINE,
        NAME_LINE,
        RATING_LINE,
        NOTE_LINE,
    ] {
        assert!(t.contains(line), "{line}");
    }
    assert!(t.contains("placeholder = true"));
    assert!(t.contains("Placeholder scenario 64"));
    // No risk is set: intent 1 is balanced.
    assert!(!t.contains("limit = \"risk\""));
    let scenario = load(&t).expect("loads");
    let world = &scenario.world;
    assert!(world.command_levels());
    let allied = world.side_view(Side::Allied);
    assert!(allied.reports_travel());
    assert_eq!(allied.commands(), vec![AREA, tf(1), BASE_1]);
    let figures = world.ref_data().reports.expect("report figures");
    for chances in [figures.up, figures.sideways, figures.down] {
        assert_eq!(chances.on_time, Fixed::ONE);
        assert_eq!(chances.late, Fixed::ZERO);
    }
    // Task force 1's commander, and no other.
    let his = allied
        .command(tf(1))
        .expect("a command")
        .commander()
        .expect("a commander");
    assert_eq!(his.name, "Placeholder Bold");
    assert_eq!(his.initiative, Initiative::Bold);
    assert!(his.note.starts_with("Placeholder"));
    for other in [AREA, BASE_1] {
        assert!(allied
            .command(other)
            .expect("a command")
            .commander()
            .is_none());
    }
    // The Japanese side has no unit.
    let truth = world.umpire();
    assert!(truth.task_forces.values().all(|f| f.side == Side::Allied));
    assert!(truth.air_groups.values().all(|g| g.side == Side::Allied));
}

#[test]
fn the_contact_is_in_task_force_1s_picture_alone_as_its_own_and_in_the_sides() {
    let played = play(&text());
    let contact = |age: u32| (1, hex(12, 0), 1, ShipKind::Cruiser, age, true);
    let (first, start) = &played.pictures[0];
    assert_eq!(first, "start");
    assert_eq!(start.tf1, vec![contact(0)]);
    assert_eq!(start.side, vec![(1, hex(12, 0), ShipKind::Cruiser, 0)]);
    for (moment, p) in &played.pictures {
        // The area command and base 1 hold no contact at any time.
        assert!(p.area.is_empty(), "{moment}: {:?}", p.area);
        assert!(p.base_1.is_empty(), "{moment}: {:?}", p.base_1);
        // Task force 1 holds it as its own while it holds it, at the age
        // the side holds it.
        let side: Vec<(u32, Hex, ShipKind, u32)> = p
            .tf1
            .iter()
            .map(|(n, h, _, k, a, _)| (*n, *h, *k, *a))
            .collect();
        assert_eq!(side, p.side, "{moment}");
        assert!(p.tf1.iter().all(|c| c.5), "{moment}: {:?}", p.tf1);
    }
    let at = |m: &str| {
        &played
            .pictures
            .iter()
            .find(|(k, _)| k == m)
            .expect("the moment")
            .1
    };
    assert_eq!(at("1 Morning").tf1, vec![contact(1)]);
    assert_eq!(at("2 Morning").tf1, vec![contact(4)]);
    // Dropped at the end of the morning of day 2, at 5 pulses.
    assert!(at("2 Afternoon").tf1.is_empty());
    assert!(at("2 Afternoon").side.is_empty());
}

#[test]
fn day_1_bold_weighs_his_own_cruiser_at_press_and_closes_with_ten_orders() {
    let played = play(&text());
    played.nothing_refused();
    assert_eq!(played.orders(1), closing_day_1());
    assert_eq!(played.launches(), 0);
    // The reason for the route names him, his rating, that the contact is
    // task force 1's own, and the level it was weighed at.
    let why = played.route_reason();
    for needle in [
        "Placeholder Bold, commanding task_force:1, is bold",
        "contact 1 held by task_force:1 is task_force:1's own",
        "he weighs it at the press risk, one level bolder than the task's balanced risk",
        "at the press risk the force closes on any kind",
        "a bold commander weighs a contact of his own at the press risk",
        "routed to (6,0) at full speed",
    ] {
        assert!(why.contains(needle), "{needle}\n{why}");
    }
}

#[test]
fn day_2_she_stays_at_6_0_and_day_3_she_goes_back_twelve_orders_in_all() {
    let played = play(&text());
    played.nothing_refused();
    assert!(played.orders(2).is_empty(), "{:?}", played.orders(2));
    assert_eq!(
        played.orders(3),
        vec![Given::Force(1, route(&[(0, 0)])), Given::Air(2, arc(0, 0))]
    );
    assert_eq!(played.order_count(), vec![10, 0, 2]);
    assert_eq!(
        played.arrivals(),
        vec![(hex(6, 0), n(8)), (hex(0, 0), n(56))]
    );
    // On day 2 the contact is 3 pulses old and 240 nm away, and weighed at
    // press is of a kind he closes on: she stays, and says why.
    let why = played.closing_reason(2);
    for needle in [
        "stays where it is",
        "contact 1 held by task_force:1, a cruiser, 3 pulse(s) old, is 240 nm from (6,0)",
        "the first of the contacts it closes on (any kind)",
        "Placeholder Bold, commanding task_force:1, is bold",
        "he weighs it at the press risk",
    ] {
        assert!(why.contains(needle), "{needle}\n{why}");
    }
}

// --------------------------------------------------------------- Properties

#[test]
fn steady_with_no_commander_she_stays_and_the_reason_names_no_commander() {
    let played = play(&no_commander(&text()));
    played.nothing_refused();
    assert_eq!(played.orders(1), staying_day_1());
    assert!(played.orders(2).is_empty(), "{:?}", played.orders(2));
    assert_eq!(played.orders(3), vec![Given::Air(2, arc(0, 0))]);
    assert_eq!(played.order_count(), vec![8, 0, 1]);
    assert!(played.arrivals().is_empty());
    // Scenario 58's reason for a kind it does not close on.
    let why = played.closing_reason(1);
    assert!(
        why.contains(
            "task_force:1 does not close: at the balanced risk it closes on a carrier, and no contact of ships 3 pulses old or less is of that kind (contact 1 held by task_force:1, a cruiser)."
        ),
        "{why}"
    );
    for r in played.all_reasons() {
        assert!(
            !r.contains("Placeholder Bold") && !r.contains("weighs"),
            "{r}"
        );
    }
}

#[test]
fn a_steady_commander_named_plays_as_none_named() {
    let named = play(&commander(&text(), "Placeholder Steady", "steady"));
    let none = play(&no_commander(&text()));
    assert_eq!(named.orders_and_reasons(), none.orders_and_reasons());
    assert_eq!(named.event_kinds(), none.event_kinds());
}

#[test]
fn cautious_on_his_own_carrier_she_stays_and_with_no_commander_she_closes() {
    let cautious = play(&commander(
        &carrier(&text()),
        "Placeholder Cautious",
        "cautious",
    ));
    cautious.nothing_refused();
    assert_eq!(cautious.orders(1), staying_day_1());
    let why = cautious.closing_reason(1);
    for needle in [
        "does not close",
        "contact 1 held by task_force:1, a carrier, weighed at the preserve risk, at which it closes on nothing",
        "Placeholder Cautious, commanding task_force:1, is cautious",
        "he weighs it at the preserve risk, one level more careful than the task's balanced risk",
    ] {
        assert!(why.contains(needle), "{needle}\n{why}");
    }
    let none = play(&no_commander(&carrier(&text())));
    none.nothing_refused();
    assert_eq!(none.orders(1), closing_day_1());
    assert!(!none.route_reason().contains("weighs"));
}

#[test]
fn a_contact_he_was_told_of_is_weighed_at_his_tasks_risk() {
    let played = play(&told(&text()));
    played.nothing_refused();
    assert_eq!(played.orders(1), staying_day_1());
    let world = load(&told(&text())).expect("loads").world;
    let view = world.side_view(Side::Allied);
    for c in [AREA, tf(1), BASE_1] {
        let p = command_contacts(&view.command(c).expect("a command"));
        assert_eq!(
            p,
            vec![(1, hex(12, 0), 1, ShipKind::Cruiser, 0, false)],
            "{c}"
        );
    }
    for r in played.all_reasons() {
        assert!(
            !r.contains("Placeholder Bold") && !r.contains("weighs"),
            "{r}"
        );
    }
}

#[test]
fn press_stays_press_for_a_bold_commander() {
    let bold = play(&at_risk(&text(), "press"));
    bold.nothing_refused();
    // Scenario 58's twelve orders of day 1.
    assert_eq!(
        bold.orders(1),
        vec![
            Given::Force(1, BATTLE),
            Given::Force(1, rules(Stance::Avoid, "0.75")),
            full(),
            Given::Force(1, route(&[(6, 0)])),
            Given::Air(1, mission(CAP)),
            Given::Air(1, share("0.25")),
            Given::Air(2, mission(SEARCH)),
            Given::Air(2, arc(50, 120)),
            Given::Air(3, mission(STRIKE)),
            Given::Air(3, AirGroupOrder::SetStrikeContactAge { pulses: 2 }),
            Given::Air(
                3,
                AirGroupOrder::SetReach {
                    reach: sim_core::orders::Reach::Extended,
                },
            ),
            Given::Air(3, AirGroupOrder::SetStrikeSizing { on: true }),
        ]
    );
    let none = play(&no_commander(&at_risk(&text(), "press")));
    assert_eq!(bold.orders_and_reasons(), none.orders_and_reasons());
}

#[test]
fn preserve_stays_preserve_for_a_cautious_commander() {
    let cautious = play(&at_risk(
        &commander(&text(), "Placeholder Cautious", "cautious"),
        "preserve",
    ));
    cautious.nothing_refused();
    let none = play(&no_commander(&at_risk(&text(), "preserve")));
    assert_eq!(cautious.orders_and_reasons(), none.orders_and_reasons());
    assert_eq!(cautious.event_kinds(), none.event_kinds());
}

#[test]
fn bold_at_preserve_the_force_closes_on_his_own_carrier_at_cruise() {
    let played = play(&at_risk(&carrier(&text()), "preserve"));
    played.nothing_refused();
    let day_1 = played.orders(1);
    assert!(
        day_1.contains(&Given::Force(1, route(&[(6, 0)]))),
        "{day_1:?}"
    );
    assert!(!day_1.iter().any(is_speed), "{day_1:?}");
    assert_eq!(played.arrivals().first(), Some(&(hex(6, 0), n(15))));
    let why = played.route_reason();
    for needle in [
        "Placeholder Bold, commanding task_force:1, is bold",
        "he weighs it at the balanced risk, one level bolder than the task's preserve risk",
        "routed to (6,0) at cruise",
    ] {
        assert!(why.contains(needle), "{needle}\n{why}");
    }
}

#[test]
fn with_command_levels_off_the_commander_is_not_read() {
    let t = off(&text());
    let scenario = load(&t).expect("loads with command levels off");
    assert!(!scenario.world.command_levels());
    let view = scenario.world.side_view(Side::Allied);
    let tf1 = view.command(tf(1)).expect("a view");
    assert!(tf1.commander().is_none());
    assert_eq!(
        command_contacts(&tf1),
        vec![(1, hex(12, 0), 1, ShipKind::Cruiser, 0, false)]
    );
    let played = play(&t);
    played.nothing_refused();
    assert_eq!(played.orders(1), staying_day_1());
    // With them off it plays as the same file with no commander and no
    // contact of its own.
    let plain = play(&off(&told(&no_commander(&text()))));
    assert_eq!(played.orders_and_reasons(), plain.orders_and_reasons());
    assert_eq!(played.event_kinds(), plain.event_kinds());
    assert_eq!(played.hash, plain.hash);
}

#[test]
fn a_rating_without_its_note_is_refused() {
    for note in ["", "note = \"\"\n", "note = \"  \"\n"] {
        let t = common::replace_once(&text(), NOTE_LINE, note);
        let error = load(&t).expect_err("refused").to_string();
        assert!(error.contains("has no note of what it rests on"), "{error}");
    }
    // With command levels off too.
    let t = common::replace_once(&off(&text()), NOTE_LINE, "");
    assert!(load(&t).is_err());
}

#[test]
fn a_commander_for_a_command_the_scenario_does_not_have_is_refused() {
    let t = common::replace_once(
        &text(),
        "task_force = 1\nname = \"Placeholder Bold\"",
        "task_force = 2\nname = \"Placeholder Bold\"",
    );
    let error = load(&t).expect_err("refused").to_string();
    assert!(
        error.contains("not a task force of the Allied side"),
        "{error}"
    );
    let t = common::replace_once(
        &text(),
        "side = \"allied\"\ntask_force = 1\nname = \"Placeholder Bold\"",
        "side = \"japanese\"\ntask_force = 1\nname = \"Placeholder Bold\"",
    );
    assert!(load(&t).is_err());
    let t = common::replace_once(
        &text(),
        "task_force = 1\nname = \"Placeholder Bold\"",
        "base = 2\nname = \"Placeholder Bold\"",
    );
    assert!(load(&t).is_err());
    // An unknown rating, and two commanders for one command.
    let t = common::replace_once(&text(), RATING_LINE, "initiative = \"rash\"\n");
    assert!(load(&t).is_err());
    let block_from = text().find(COMMANDER_MARKER).expect("the commander");
    let block_to = text().find(COMMANDS_MARKER).expect("the commands");
    let block = text()[block_from..block_to].to_string();
    let t = format!("{}{block}{}", &text()[..block_to], &text()[block_to..]);
    let error = load(&t).expect_err("refused").to_string();
    assert!(error.contains("has a commander already"), "{error}");
    // A contact seen by a command the side does not have.
    let t = common::replace_once(&text(), SEEN_BY_LINE, "seen_by = { task_force = 2 }\n");
    assert!(load(&t).is_err());
}

#[test]
fn the_same_start_gives_the_same_game() {
    let a = play(&text());
    let b = play(&text());
    assert_eq!(a.orders_and_reasons(), b.orders_and_reasons());
    assert_eq!(a.event_kinds(), b.event_kinds());
    assert_eq!(a.hash, b.hash);
}
