//! Scenario 72: reconnaissance. The commander asks for an airfield at
//! preserve the force. The staff has no count of the enemy there. It
//! sends an aircraft to look, learns that 150 men hold it, chooses one
//! unit where it had chosen two, and only then launches.
//!
//! Proves decision 0147, step 6 of stage 3, first part, against a hand
//! calculation. The data is placeholder data: see
//! `data/scenarios/72_reconnaissance.toml`. It runs on the frozen step 1
//! figures, with ground figures of its own that differ from the frozen
//! ones in five: `preparation_per_day` 100, `report_error_low` 1,
//! `report_error_high` 1, `reconnaissance_error_low` 1 and
//! `reconnaissance_error_high` 1. A report gives the true count, so no
//! die changes anything. The game ends before the landing.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly, except the supply and the fatigue the flights cost, which
//! are the search rule's and are not this scenario's to test. If the
//! code disagrees, the code or the rule is wrong: do not change a number
//! here to make a test pass. A mismatch goes back to the architect.
//!
//! ## Figures
//!
//! A force with its roster in port gains 100 of loading in a day and a
//! unit ready on it 100 of rehearsal, each to 100. A change of roster
//! keeps half the loading. At preserve the force the staff wants 3 men
//! for each man of its estimate, a landing is ready at a loading and a
//! rehearsal of 100, and a "take a place" may launch on a count seen
//! from the air or by men in contact no more than 3 days old. A man eats
//! 0.005 t a day.
//!
//! ## Inputs
//!
//! The placeholder map of scenario 27: A at (0,0), port size 5, and B at
//! (4,0), port size 0, 4 hexes away. The game is 4 days.
//!
//! Base 1, Allied, at A, with an airfield and 1,000 t of supply. Base 2,
//! Japanese, at B, on the Allied map, with 100 t of supply.
//!
//! Air group 1, Allied, at base 1: 6 ready aircraft of a placeholder type
//! of the frozen figures that can search and whose search range is 4
//! hexes or more, at rest.
//!
//! Task force 1, Allied, on the transport mission: two placeholder troop
//! transports of 600 berths, stopped at base 1.
//!
//! | Unit | Side | Where | Men | Experience | Assault skill |
//! | ---- | ---- | ----- | --- | ---------- | ------------- |
//! | 1, "Placeholder First" | Allied | A | 600 | 2 | 2 |
//! | 2, "Placeholder Second" | Allied | A | 600 | 2 | 2 |
//! | 10, "Placeholder Garrison" | Japanese | B | 150 | 2 | |
//!
//! The Allied side holds no estimate. Its commands of day 1: intent 1,
//! take the place at base 2, at preserve the force, with a forces limit
//! of task force 1, units 1 and 2, air group 1 and base 1 as the source.
//! Leave to prepare and leave to launch are both given.
//!
//! ## Day by day
//!
//! | Day | The estimate at the staff's hour | The staff's commands and orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 | none | Approves plan 1 to preparing. Forms intent 2, ready a landing by task force 1 at B, with a forces limit of units 1 and 2. Orders `prepare_landing` at B, roster 1 and 2. Sets air group 1 to reconnaissance of B | In the morning one aircraft flies 4 hexes and reports 150 men, seen from the air. Loading 100; each unit's rehearsal 100 |
//! | 2 | 150, seen from the air, of day 1 | Sets the forces limit of intent 2 to unit 1. Orders `prepare_landing` at B, roster 1. Sets air group 1 to rest | Unit 2 leaves the roster, which costs no loading (decision 0095, item 10): it stays 100 |
//! | 3 | the same | Notice: intent 2 is ready. Approves plan 1 to released. Orders a load of supply to a weight of 45 t | |
//! | 4 | the same | Unit 1 set to attack. Load unit 1 | |
//!
//! - **Day 1.** With no estimate the staff takes every unit the ships can
//!   carry, units 1 and 2, and says it goes with no count. The need is
//!   not met: it sends air group 1, the only group in the forces limit.
//! - **Day 2.** The men wanted are 150 x 3 = 450. Unit 1 has 600: it is
//!   enough, and unit 2 is left out. The need is met, by a count seen
//!   from the air one day old. The landing is not ready: it was ordered
//!   again this morning, and a landing ordered in the morning is not
//!   ready that morning. Its loading stays 100, since only a unit added
//!   to a roster costs loading.
//! - **Day 3.** The loading is 100 and unit 1's rehearsal is 100: ready,
//!   and the count is 2 days old. The supply is 600 x 0.005 x 15 = 45 t.
//!
//! Seven orders in all, and five staff commands. Air group 1 flies once,
//! on day 1. No staff order is refused.
//!
//! # Properties
//!
//! - **The grades.** With an opening estimate of 500 men at B: it is of
//!   the grade told, and at preserve the force it does not meet the need.
//!   On day 1 the men wanted are 500 x 3 = 1,500 and the ships carry
//!   1,200: at preserve the force no landing is formed and the staff
//!   hands in the notice that it cannot meet the intent (decision 0142,
//!   item 10); air group 1 is sent all the same and flies. On day 2 the
//!   estimate is 150, seen from the air: the landing is formed with unit
//!   1 alone and the group is set to rest. Restated by the architect on
//!   Nov 17, 2026: the first writing had two units chosen on day 1.
//! - **Balanced.** At balanced the men wanted on day 2 are 300, unit 1 is
//!   chosen alone, and the days are as in the main case.
//! - **Press.** At press no group is sent on any day. The landing is
//!   ready on the morning of day 2 with units 1 and 2: the plan is
//!   released and the reason says it goes on no count. The load is to
//!   90 t.
//! - **Nothing can look, at preserve the force.** With air group 1 left
//!   out of the forces limit: no group is sent. On the morning of day 2
//!   the landing is otherwise ready: the plan is not released, and the
//!   staff hands in the notice that it cannot meet intent 1, on that
//!   morning, and on each after gives the reason, that nothing it may use
//!   can look.
//! - **Nothing can look, at balanced.** The same, at balanced: on the
//!   morning of day 2 the plan is released with units 1 and 2, and the
//!   reason says it goes on no count.
//! - **Men in contact.** With an Allied fighting unit 3 of 100 men ashore
//!   at B on the defence from the start, fed from an Allied base 3 there:
//!   air group 1 is not sent on any day. The ground report of the end of
//!   day 1 gives 150 men, by men in contact, and day 2 is as in the main
//!   case but for the order to the group.
//! - **An empty place.** With unit 10 left out: the flight of day 1
//!   reports 0 men. On day 2 the staff chooses one unit, unit 1.
//! - **The frozen figures give no flight.** With the two reconnaissance
//!   figures left out, a group on the mission does not fly and no report
//!   is made.
//! - **The enemy sees nothing of the flight:** the Japanese side's events
//!   hold neither the flight nor the report.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see and has not been told give the same
//!   orders and reasons; a save on day 2 and a resume give the same game;
//!   every scenario built before this one gives the events it gave.

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::belief::GroundGrade;
use sim_core::command::{
    CommandKind, IntentKind, Limit, NoticeKind, PlanState, Reason, StaffDecisions, StaffOrder,
    Staffs,
};
use sim_core::events::{Cause, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, EntityRef, IntentId, LandUnitId, TaskForceId};
use sim_core::orders::{
    AirGroupOrder, AirMission, Cargo, LandUnitOrder, OrderKind, Posture, TaskForceOrder,
};
use sim_core::side_events::{SideEvent, SideEventKind};
use sim_core::state::{AirGroupLocation, Side, TruthState};
use sim_core::time::Pulse;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

use crate::common;

const COMMANDS_MARKER: &str = "# The Allied commander's commands";
const B: Hex = Hex { q: 4, r: 0 };

fn path() -> PathBuf {
    common::data_dir().join("scenarios/72_reconnaissance.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn unit(raw: u32) -> LandUnitId {
    LandUnitId::new(raw)
}

fn intent(raw: u32) -> IntentId {
    IntentId::new(raw)
}

fn group(raw: u32) -> AirGroupId {
    AirGroupId::new(raw)
}

fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

/// The frozen step 1 figures. The scenario names its own ground figures,
/// `data/reference/ground_placeholder_scenario_72.toml`, frozen beside the
/// others: the figures its hand calculation names.
fn figures() -> sim_data::LoadOptions {
    common::frozen()
}

fn load_from(text: &str, at: &str) -> Scenario {
    let at = common::data_dir().join("scenarios").join(at);
    common::from_text_with(text, &at, &figures()).expect("the scenario loads")
}

fn load(text: &str) -> Scenario {
    common::from_text_with(text, &path(), &figures()).expect("the scenario loads")
}

/// The Allied side's estimate of the enemy at B: men, grade, day.
type Estimate = Option<(u32, GroundGrade, u32)>;

struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    truth: Vec<TruthState>,
    events: Vec<Event>,
    hashes: Vec<u64>,
    /// The Allied estimate at B as it stood after each day: index 0 before
    /// day 1, so the staff's hour of day `d` reads index `d - 1`.
    estimates: Vec<Estimate>,
    allied_events: Vec<SideEvent>,
    japanese_events: Vec<SideEvent>,
}

fn estimate_of(scenario: &Scenario) -> Estimate {
    scenario
        .world
        .side_view(Side::Allied)
        .ground_estimate(B)
        .map(|e| (e.men, e.grade, e.day))
}

fn play(text: &str) -> Played {
    play_from(load(text), BTreeMap::new())
}

fn play_from(mut scenario: Scenario, mut handed: BTreeMap<u32, StaffDecisions>) -> Played {
    let held = staffs();
    let mut truth = vec![scenario.world.umpire().clone()];
    let mut estimates = vec![estimate_of(&scenario)];
    loop {
        let day = scenario.world.day() + 1;
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
        handed.insert(
            day,
            scenario
                .world
                .staff_handed_in()
                .get(&Side::Allied)
                .cloned()
                .unwrap_or_default(),
        );
        truth.push(scenario.world.umpire().clone());
        estimates.push(estimate_of(&scenario));
    }
    Played {
        handed,
        truth,
        events: scenario.world.events().iter().cloned().collect(),
        hashes: scenario.world.day_hashes().to_vec(),
        estimates,
        allied_events: scenario.world.side_view(Side::Allied).events(),
        japanese_events: scenario.world.side_view(Side::Japanese).events(),
    }
}

/// An order of the staff, plainly.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Given {
    Landing(u32, Hex, Vec<u32>),
    Look(u32, Hex),
    Rest(u32),
    LoadTo(Fixed),
    Attack(u32),
    LoadUnit(u32),
}

fn given(o: &StaffOrder) -> Given {
    match &o.kind {
        OrderKind::TaskForce {
            target,
            order: TaskForceOrder::PrepareLanding { hex, roster },
        } => Given::Landing(target.raw(), *hex, roster.iter().map(|u| u.raw()).collect()),
        OrderKind::AirGroup {
            target,
            order: AirGroupOrder::SetAirMission(AirMission::Reconnaissance(hex)),
        } => Given::Look(target.raw(), *hex),
        OrderKind::AirGroup {
            target,
            order: AirGroupOrder::SetAirMission(AirMission::Rest),
        } => Given::Rest(target.raw()),
        OrderKind::TaskForce {
            order:
                TaskForceOrder::LoadTo {
                    cargo: Cargo::Supply,
                    weight_t,
                    ..
                },
            ..
        } => Given::LoadTo(*weight_t),
        OrderKind::LandUnit {
            target,
            order:
                LandUnitOrder::SetPosture {
                    posture: Posture::Attack,
                    when_spent: None,
                },
        } => Given::Attack(target.raw()),
        OrderKind::TaskForce {
            order: TaskForceOrder::LoadUnit { unit, cargo: None },
            ..
        } => Given::LoadUnit(unit.raw()),
        other => panic!("the doctrine gave an order of another kind: {other:?}"),
    }
}

/// A staff command, plainly, by kind.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Cmd {
    Approve(u32),
    Form(u32, IntentKind),
    Limit(u32, Vec<u32>),
}

fn cmd(c: &CommandKind) -> Cmd {
    match c {
        CommandKind::Approve { plan } => Cmd::Approve(plan.raw()),
        CommandKind::FormIntent { parent, kind } => Cmd::Form(parent.raw(), *kind),
        CommandKind::SetLimit(Limit::Forces { intent, limit }) => {
            assert!(limit.ships.is_empty() && limit.task_forces.is_empty());
            assert!(limit.air_groups.is_empty() && limit.source.is_none());
            Cmd::Limit(
                intent.raw(),
                limit.land_units.iter().map(|u| u.raw()).collect(),
            )
        }
        other => panic!("the staff handed in another command: {other:?}"),
    }
}

fn landing_kind() -> IntentKind {
    IntentKind::ReadyLanding {
        force: TaskForceId::new(1),
        hex: B,
        by_day: None,
    }
}

impl Played {
    fn days(&self) -> u32 {
        u32::try_from(self.handed.len()).expect("few days")
    }

    fn orders(&self, day: u32) -> Vec<Given> {
        self.handed[&day].orders.iter().map(given).collect()
    }

    fn all_orders(&self) -> Vec<(u32, Given)> {
        (1..=self.days())
            .flat_map(|d| self.orders(d).into_iter().map(move |g| (d, g)))
            .collect()
    }

    fn commands(&self, day: u32) -> Vec<Cmd> {
        self.handed[&day]
            .commands
            .iter()
            .map(|c| cmd(&c.kind))
            .collect()
    }

    fn all_commands(&self) -> Vec<(u32, Cmd)> {
        (1..=self.days())
            .flat_map(|d| self.commands(d).into_iter().map(move |c| (d, c)))
            .collect()
    }

    fn command_reason(&self, day: u32, wanted: &Cmd) -> &Reason {
        &self.handed[&day]
            .commands
            .iter()
            .find(|c| cmd(&c.kind) == *wanted)
            .unwrap_or_else(|| panic!("day {day} has the command {wanted:?}"))
            .reason
    }

    fn order_reason(&self, day: u32, wanted: &Given) -> &Reason {
        &self.handed[&day]
            .orders
            .iter()
            .find(|o| given(o) == *wanted)
            .unwrap_or_else(|| panic!("day {day} has the order {wanted:?}"))
            .reason
    }

    fn all_notices(&self) -> Vec<(u32, NoticeKind, u32)> {
        (1..=self.days())
            .flat_map(|d| {
                self.handed[&d]
                    .notices
                    .iter()
                    .map(move |nt| (d, nt.kind, nt.plan.raw()))
            })
            .collect()
    }

    /// The no-order reasons of `day` serving `of`.
    fn no_orders(&self, day: u32, of: u32) -> Vec<String> {
        self.handed[&day]
            .no_orders
            .iter()
            .filter(|r| r.serves.intent == Some(intent(of)))
            .map(text_of)
            .collect()
    }

    fn plan_day(&self, plan: u32, to: PlanState) -> Option<u32> {
        self.events.iter().find_map(|e| match &e.kind {
            EventKind::PlanStateChanged {
                side,
                plan: p,
                to: t,
                ..
            } if *side == Side::Allied && p.raw() == plan && *t == to => Some(e.day),
            _ => None,
        })
    }

    fn flights(&self) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|e| matches!(e.kind, EventKind::ReconnaissanceFlown { .. }))
            .collect()
    }

    /// The Allied ground reports of B: day, pulse, men, grade, replaced.
    fn reports(&self) -> Vec<(u32, Option<Pulse>, u32, GroundGrade, bool)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::GroundReportReceived {
                    side: Side::Allied,
                    hex,
                    men,
                    replaced,
                    grade,
                } if *hex == B => Some((e.day, e.pulse, *men, *grade, *replaced)),
                _ => None,
            })
            .collect()
    }

    fn group_mission(&self, day: u32) -> AirMission {
        self.truth[day as usize].air_groups[&group(1)].mission
    }

    fn roster(&self, day: u32) -> Vec<u32> {
        self.truth[day as usize].task_forces[&TaskForceId::new(1)]
            .preparation
            .as_ref()
            .map(|p| p.roster.iter().map(|u| u.raw()).collect())
            .unwrap_or_default()
    }

    fn loading(&self, day: u32) -> Fixed {
        self.truth[day as usize].task_forces[&TaskForceId::new(1)]
            .preparation
            .as_ref()
            .map_or(Fixed::ZERO, |p| p.loading)
    }

    fn refused(&self) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|e| {
                matches!(
                    e.kind,
                    EventKind::OrderRejected { .. } | EventKind::CommandRefused { .. }
                )
            })
            .collect()
    }
}

fn text_of(reason: &Reason) -> String {
    format!(
        "{} | {} | {}",
        reason.worked,
        reason.binding.join("; "),
        reason
            .evidence
            .iter()
            .map(|e| e.what.clone())
            .collect::<Vec<_>>()
            .join("; ")
    )
}

fn assert_says(all: &str, parts: &[&str]) {
    for part in parts {
        assert!(all.contains(part), "the reason says {part:?}: {all}");
    }
}

fn at_risk(t: &str, level: &str) -> String {
    changed(t, "risk = \"preserve\"", &format!("risk = \"{level}\""))
}

fn without_the_group(t: &str) -> String {
    changed(t, "air_groups = [1]\n", "")
}

fn main_game() -> &'static Played {
    static GAME: std::sync::OnceLock<Played> = std::sync::OnceLock::new();
    GAME.get_or_init(|| play(&text()))
}

/// The orders of days 1 to 3, which are the same in several games; day 4's
/// two orders are compared apart, as a pair.
fn main_orders_to_day_3() -> Vec<(u32, Given)> {
    vec![
        (1, Given::Landing(1, B, vec![1, 2])),
        (1, Given::Look(1, B)),
        (2, Given::Landing(1, B, vec![1])),
        (2, Given::Rest(1)),
        (3, Given::LoadTo(n(45))),
    ]
}

fn assert_main_orders(g: &Played, orders: &[(u32, Given)]) {
    let all = g.all_orders();
    let (before, day_4): (Vec<_>, Vec<_>) = all.into_iter().partition(|(d, _)| *d < 4);
    assert_eq!(before, orders);
    let mut pair: Vec<Given> = day_4.into_iter().map(|(_, o)| o).collect();
    pair.sort_by_key(|o| format!("{o:?}"));
    let mut wanted = vec![Given::Attack(1), Given::LoadUnit(1)];
    wanted.sort_by_key(|o| format!("{o:?}"));
    assert_eq!(pair, wanted, "day 4: unit 1 set to attack, and loaded");
}

fn main_commands() -> Vec<(u32, Cmd)> {
    vec![
        (1, Cmd::Approve(1)),
        (1, Cmd::Form(1, landing_kind())),
        (1, Cmd::Limit(2, vec![1, 2])),
        (2, Cmd::Limit(2, vec![1])),
        (3, Cmd::Approve(1)),
    ]
}

// ---------------------------------------------------------------- The hand calculation

#[test]
fn seven_orders_and_five_staff_commands_and_none_refused() {
    let g = main_game();
    assert_eq!(g.days(), 4);
    assert_main_orders(g, &main_orders_to_day_3());
    assert_eq!(g.all_orders().len(), 7);
    assert_eq!(g.all_commands(), main_commands());
    assert_eq!(g.all_commands().len(), 5);
    assert!(g.refused().is_empty(), "{:?}", g.refused());
}

#[test]
fn the_estimate_at_the_staffs_hour_of_each_day() {
    let g = main_game();
    // Day 1: none. Days 2 to 4: 150, seen from the air, of day 1.
    assert_eq!(g.estimates[0], None);
    for hour in 1..=3 {
        assert_eq!(
            g.estimates[hour],
            Some((150, GroundGrade::SeenFromAir, 1)),
            "the estimate at the staff's hour of day {}",
            hour + 1
        );
    }
}

#[test]
fn day_1_the_flight_and_the_report() {
    let g = main_game();
    // Air group 1 flies once, on day 1, in the morning: 4 hexes, 160 nm.
    let flights = g.flights();
    assert_eq!(flights.len(), 1);
    let flight = flights[0];
    assert_eq!((flight.day, flight.pulse), (1, Some(Pulse::Morning)));
    let EventKind::ReconnaissanceFlown {
        air_group,
        from,
        hex,
        distance_nm,
        ..
    } = &flight.kind
    else {
        unreachable!()
    };
    assert_eq!(*air_group, group(1));
    assert_eq!(*from, AirGroupLocation::Base(BaseId::new(1)));
    assert_eq!(*hex, B);
    assert_eq!(*distance_nm, n(160));
    // It cites the mission order, the staff's order of day 1 to the group.
    let order = g
        .events
        .iter()
        .find_map(|e| match &e.kind {
            EventKind::OrderAccepted {
                order,
                target: EntityRef::AirGroup(target),
            } if *target == group(1) && e.day == 1 => Some(*order),
            _ => None,
        })
        .expect("the group's mission order was accepted on day 1");
    assert_eq!(flight.causes, vec![Cause::Order(order)]);
    // The report: 150 men, seen from the air, citing the flight.
    assert_eq!(
        g.reports(),
        vec![(
            1,
            Some(Pulse::Morning),
            150,
            GroundGrade::SeenFromAir,
            false
        )]
    );
    let report = g
        .events
        .iter()
        .find(|e| matches!(e.kind, EventKind::GroundReportReceived { .. }))
        .expect("the report");
    assert_eq!(report.causes, vec![Cause::Event(flight.id)]);
    // No base was found: base 2 was on the map.
    assert!(!g
        .events
        .iter()
        .any(|e| matches!(e.kind, EventKind::BaseFound { .. })));
    // Loading 100; each unit's rehearsal 100.
    assert_eq!(g.loading(1), n(100));
    assert_eq!(g.roster(1), vec![1, 2]);
    for u in [1, 2] {
        assert_eq!(g.truth[1].land_units[&unit(u)].rehearsal, n(100));
    }
    assert_eq!(g.group_mission(1), AirMission::Reconnaissance(B));
}

#[test]
fn day_1_the_staff_chooses_with_no_count_and_says_so() {
    let g = main_game();
    let form = text_of(g.command_reason(1, &Cmd::Form(1, landing_kind())));
    assert_says(
        &form,
        &[
            "holds no estimate of the enemy's men",
            "every unit the ships can carry",
            "no count of the enemy",
            "land_unit:1",
            "land_unit:2",
        ],
    );
    let look = text_of(g.order_reason(1, &Given::Look(1, B)));
    assert_says(
        &look,
        &[
            "the side holds no estimate of the enemy's men at (4,0)",
            "at preserve the force",
            "no more than 3 days before the morning",
            "air_group:1",
            "range 300 nm, 160 nm away",
            "reconnaissance of (4,0)",
        ],
    );
}

#[test]
fn day_2_the_count_changes_the_choice() {
    let g = main_game();
    // 150 x 3 = 450 men wanted: unit 1 has 600, so unit 2 is left out.
    let limit = text_of(g.command_reason(2, &Cmd::Limit(2, vec![1])));
    assert_says(
        &limit,
        &[
            "150 men, seen from the air, of day 1",
            "wants 3 men for each, 450 men",
            "land_unit:2 (Placeholder Second) is not needed",
            "The roster of intent:2 was land_unit:1 and land_unit:2 and is now land_unit:1",
        ],
    );
    // The landing is ordered again with the new roster; unit 2 leaves it.
    assert_eq!(g.roster(2), vec![1]);
    assert!(g
        .events
        .iter()
        .any(|e| e.day == 2
            && matches!(&e.kind, EventKind::LeftRoster { unit: u, .. } if *u == unit(2))));
    // The need is met by a count seen from the air one day old: the group
    // is set to rest, by one order.
    assert_eq!(g.group_mission(2), AirMission::Rest);
    let rest = text_of(g.order_reason(2, &Given::Rest(1)));
    assert_says(&rest, &["the need is met", "to rest"]);
    // The loading after the day's gain is 100 again.
    assert_eq!(g.loading(2), n(100));
    // The landing is not ready on day 2: the order changed the roster.
    assert_eq!(g.plan_day(1, PlanState::Released), Some(3));
    let not_ready = g.no_orders(2, 1).join(" ");
    assert_says(&not_ready, &["The landing at (4,0) is not ready"]);
}

/// Day 2: the change of roster drops unit 2 and adds none, so no loading is
/// lost (decision 0095, item 10) and the loading stays 100.
#[test]
fn day_2_no_loading_is_lost_and_the_loading_stays_100() {
    let g = main_game();
    assert!(!g
        .events
        .iter()
        .any(|e| e.day == 2 && matches!(e.kind, EventKind::LoadingLost { .. })));
    assert_eq!(g.loading(1), n(100));
    assert_eq!(g.loading(2), n(100));
}

#[test]
fn day_3_ready_the_count_is_two_days_old_and_the_plan_is_released() {
    let g = main_game();
    // The notice that intent 2's landing is ready, on plan 1.
    assert_eq!(g.all_notices(), vec![(3, NoticeKind::Ready, 1)]);
    assert_eq!(g.plan_day(1, PlanState::Preparing), Some(1));
    assert_eq!(g.plan_day(1, PlanState::Released), Some(3));
    // Loading 100 and unit 1's rehearsal 100.
    assert_eq!(g.loading(2), n(100));
    assert_eq!(g.truth[2].land_units[&unit(1)].rehearsal, n(100));
    let release = text_of(g.command_reason(3, &Cmd::Approve(1)));
    assert_says(
        &release,
        &[
            "is ready",
            "the force's loading for (4,0) is 100",
            "It goes on the count held",
            "150 men, seen from the air, of day 1",
        ],
    );
    // The supply is 600 x 0.005 x 15 = 45 t, ordered to a weight of 45 t.
    assert_eq!(g.orders(3), vec![Given::LoadTo(n(45))]);
    let ships: Fixed = g.truth[3]
        .ships
        .values()
        .fold(Fixed::ZERO, |sum, s| sum + s.supply_t);
    assert_eq!(ships, n(45));
}

#[test]
fn day_4_unit_1_is_set_to_attack_and_loaded() {
    let g = main_game();
    let mut day = g.orders(4);
    day.sort_by_key(|o| format!("{o:?}"));
    let mut wanted = vec![Given::Attack(1), Given::LoadUnit(1)];
    wanted.sort_by_key(|o| format!("{o:?}"));
    assert_eq!(day, wanted);
    assert_eq!(g.truth[4].land_units[&unit(1)].posture, Posture::Attack);
    // The game ends before the landing: nothing is ashore at B but the
    // Japanese garrison.
    assert_eq!(g.truth[4].land_units[&unit(10)].men_ashore, 150);
    assert_eq!(g.flights().len(), 1, "air group 1 flew once, on day 1");
}

// ---------------------------------------------------------------- Properties

fn with_an_opening_estimate_of_500() -> String {
    changed(
        &text(),
        COMMANDS_MARKER,
        "[[estimate]]\nside = \"allied\"\nq = 4\nr = 0\nmen = 500\n\n# The Allied commander's commands",
    )
}

#[test]
fn the_grades() {
    // An opening estimate of 500 men at B is told: it does not meet the need
    // at preserve the force, so a group is sent on day 1.
    let g = play(&with_an_opening_estimate_of_500());
    assert_eq!(g.estimates[0], Some((500, GroundGrade::Told, 0)));
    assert!(
        g.orders(1).contains(&Given::Look(1, B)),
        "a told count is not enough"
    );
    // On day 2 the estimate is 150, seen from the air, of day 1: the flight's
    // report replaces the opening estimate whatever its grade.
    assert_eq!(g.estimates[1], Some((150, GroundGrade::SeenFromAir, 1)));
    assert_eq!(
        g.reports(),
        vec![
            (0, None, 500, GroundGrade::Told, false),
            (1, Some(Pulse::Morning), 150, GroundGrade::SeenFromAir, true)
        ]
    );
    assert_eq!(g.flights().len(), 1);
    assert!(g.refused().is_empty(), "{:?}", g.refused());
}

/// The told game, as restated: on day 1 no landing is formed (1,500 men
/// wanted against the 1,200 the ships carry, decision 0142, item 10), the
/// notice that intent 1 cannot be met is handed in, and air group 1 is sent
/// and flies; on day 2, on 150 seen from the air, the landing is formed with
/// unit 1 alone and the group is set to rest.
#[test]
fn the_grades_day_1_no_landing_and_day_2_unit_1_alone() {
    let g = play(&with_an_opening_estimate_of_500());
    // Day 1.
    assert!(!g.commands(1).iter().any(|c| matches!(c, Cmd::Form(..))));
    assert_eq!(g.orders(1), vec![Given::Look(1, B)]);
    let cannot: Vec<(u32, NoticeKind, u32)> = g
        .all_notices()
        .into_iter()
        .filter(|(_, k, _)| *k == NoticeKind::CannotMeet)
        .collect();
    assert_eq!(cannot, vec![(1, NoticeKind::CannotMeet, 1)]);
    assert_eq!(g.flights().len(), 1);
    assert_eq!(g.estimates[1], Some((150, GroundGrade::SeenFromAir, 1)));
    // Day 2.
    assert!(g.commands(2).contains(&Cmd::Form(1, landing_kind())));
    assert!(g.commands(2).contains(&Cmd::Limit(2, vec![1])));
    assert!(g.orders(2).contains(&Given::Landing(1, B, vec![1])));
    assert!(g.orders(2).contains(&Given::Rest(1)));
    assert!(g.refused().is_empty(), "{:?}", g.refused());
    // Ordered on day 2, with loading and rehearsal 100 by its end: ready,
    // and the plan released, on the morning of day 3.
    assert_eq!(g.plan_day(1, PlanState::Released), Some(3));
}

#[test]
fn balanced() {
    let g = play(&at_risk(&text(), "balanced"));
    // The men wanted on day 2 are 150 x 2 = 300: unit 1 alone.
    let limit = text_of(g.command_reason(2, &Cmd::Limit(2, vec![1])));
    assert_says(&limit, &["wants 2 men for each, 300 men"]);
    assert_main_orders(&g, &main_orders_to_day_3());
    assert_eq!(g.all_commands(), main_commands());
    assert!(g.refused().is_empty(), "{:?}", g.refused());
}

#[test]
fn press() {
    let g = play(&at_risk(&text(), "press"));
    // No group is sent on any day.
    assert!(!g
        .all_orders()
        .iter()
        .any(|(_, o)| matches!(o, Given::Look(..) | Given::Rest(..))));
    assert!(g.flights().is_empty());
    assert_eq!(g.group_mission(4), AirMission::Rest);
    // The landing is ready on the morning of day 2 with units 1 and 2: the
    // plan is released and the reason says it goes on no count.
    assert_eq!(g.orders(1), vec![Given::Landing(1, B, vec![1, 2])]);
    assert_eq!(g.plan_day(1, PlanState::Released), Some(2));
    assert_eq!(g.commands(2), vec![Cmd::Approve(1)]);
    let release = text_of(g.command_reason(2, &Cmd::Approve(1)));
    assert_says(&release, &["It goes on no count of the enemy at (4,0)"]);
    // The load is to 90 t: 1,200 men x 0.005 x 15.
    assert_eq!(g.orders(2), vec![Given::LoadTo(n(90))]);
    assert!(g.refused().is_empty(), "{:?}", g.refused());
}

#[test]
fn nothing_can_look_at_preserve_the_force() {
    let g = play(&without_the_group(&text()));
    // No group is sent on any day.
    assert!(!g
        .all_orders()
        .iter()
        .any(|(_, o)| matches!(o, Given::Look(..) | Given::Rest(..))));
    assert!(g.flights().is_empty());
    assert_eq!(g.orders(1), vec![Given::Landing(1, B, vec![1, 2])]);
    // On the morning of day 2 the landing is otherwise ready: the plan is not
    // released, and the staff hands in the notice that it cannot meet
    // intent 1, on that morning and on no other.
    assert_eq!(g.plan_day(1, PlanState::Released), None);
    let cannot: Vec<(u32, NoticeKind, u32)> = g
        .all_notices()
        .into_iter()
        .filter(|(_, k, _)| *k == NoticeKind::CannotMeet)
        .collect();
    assert_eq!(cannot, vec![(2, NoticeKind::CannotMeet, 1)]);
    // On each morning after it gives the reason: nothing it may use can look.
    for day in 2..=4 {
        let reasons = g.no_orders(day, 1).join(" ");
        assert_says(
            &reasons,
            &["nothing it may use can look", "does not release plan:1"],
        );
    }
    assert!(g.commands(2).is_empty() && g.commands(3).is_empty());
    assert_eq!(g.orders(2), vec![]);
    assert!(g.refused().is_empty(), "{:?}", g.refused());
}

#[test]
fn nothing_can_look_at_balanced() {
    let g = play(&without_the_group(&at_risk(&text(), "balanced")));
    assert!(g.flights().is_empty());
    // On the morning of day 2 the plan is released with units 1 and 2, and
    // the reason says it goes on no count.
    assert_eq!(g.plan_day(1, PlanState::Released), Some(2));
    assert_eq!(g.orders(1), vec![Given::Landing(1, B, vec![1, 2])]);
    let release = text_of(g.command_reason(2, &Cmd::Approve(1)));
    assert_says(&release, &["It goes on no count of the enemy at (4,0)"]);
    assert_eq!(g.orders(2), vec![Given::LoadTo(n(90))]);
    assert!(!g
        .all_notices()
        .iter()
        .any(|(_, k, _)| *k == NoticeKind::CannotMeet));
    assert!(g.refused().is_empty(), "{:?}", g.refused());
}

#[test]
fn men_in_contact() {
    // An Allied fighting unit 3 of 100 men ashore at B on the defence from the
    // start, fed from an Allied base 3 there.
    let extra = "[[base]]\nid = 3\nname = \"Placeholder base B\"\nside = \"allied\"\nq = 4\nr = 0\nsupply_t = 100\nfuel_t = 0\n\n[[land_unit]]\nid = 3\nname = \"Placeholder Landing party\"\nside = \"allied\"\nmen = 100\nq = 4\nr = 0\n\n# The Allied commander's commands";
    let t = changed(&text(), COMMANDS_MARKER, extra);
    let g = play(&t);
    // Air group 1 is not sent on any day.
    assert!(!g
        .all_orders()
        .iter()
        .any(|(_, o)| matches!(o, Given::Look(..) | Given::Rest(..))));
    assert!(g.flights().is_empty());
    // The ground report of the end of day 1 gives 150 men, by men in contact.
    assert_eq!(
        g.reports()[0],
        (1, None, 150, GroundGrade::MenInContact, false)
    );
    assert_eq!(g.estimates[1], Some((150, GroundGrade::MenInContact, 1)));
    // Day 2 is as in the main case but for the order to the group.
    let without_group: Vec<(u32, Given)> = main_orders_to_day_3()
        .into_iter()
        .filter(|(_, o)| !matches!(o, Given::Look(..) | Given::Rest(..)))
        .collect();
    assert_main_orders(&g, &without_group);
    assert_eq!(g.all_commands(), main_commands());
    assert!(g.refused().is_empty(), "{:?}", g.refused());
}

#[test]
fn an_empty_place() {
    // With unit 10 left out the flight of day 1 reports 0 men.
    let t = changed(
        &text(),
        "[[land_unit]]\nid = 10\nname = \"Placeholder Garrison\"\nside = \"japanese\"\nmen = 150\nq = 4\nr = 0\n\n",
        "",
    );
    let g = play(&t);
    assert_eq!(
        g.reports(),
        vec![(1, Some(Pulse::Morning), 0, GroundGrade::SeenFromAir, false)]
    );
    assert_eq!(g.estimates[1], Some((0, GroundGrade::SeenFromAir, 1)));
    // On day 2 the staff chooses one unit, unit 1: an estimate of 0 asks for
    // one unit, the first in the order.
    assert_eq!(g.commands(2), vec![Cmd::Limit(2, vec![1])]);
    let limit = text_of(g.command_reason(2, &Cmd::Limit(2, vec![1])));
    assert_says(&limit, &["0 men, seen from the air, of day 1", "0 men"]);
    assert_eq!(g.roster(2), vec![1]);
}

#[test]
fn the_frozen_figures_give_no_flight() {
    // The frozen step 1 ground figures give neither reconnaissance figure:
    // a group on the mission does not fly and no report is made.
    let t = changed(
        &text(),
        "ground = \"../reference/ground_placeholder_scenario_72.toml\"",
        "ground = \"../reference/ground_placeholder_scenario_69.toml\"",
    );
    let g = play(&t);
    assert!(g.flights().is_empty());
    assert!(g.reports().is_empty());
    assert_eq!(g.group_mission(4), AirMission::Reconnaissance(B));
    assert!(g.estimates.iter().all(Option::is_none));
}

#[test]
fn the_enemy_sees_nothing_of_the_flight() {
    let g = main_game();
    let flown = |events: &[SideEvent]| {
        events
            .iter()
            .filter(|e| {
                matches!(
                    &e.kind,
                    SideEventKind::Whole(EventKind::ReconnaissanceFlown { .. })
                )
            })
            .count()
    };
    let reports = |events: &[SideEvent]| {
        events
            .iter()
            .filter(|e| {
                matches!(
                    &e.kind,
                    SideEventKind::Whole(EventKind::GroundReportReceived { .. })
                )
            })
            .count()
    };
    // The flying side sees both whole; the enemy sees neither.
    assert_eq!(flown(&g.allied_events), 1);
    assert_eq!(reports(&g.allied_events), 1);
    assert_eq!(flown(&g.japanese_events), 0);
    assert_eq!(reports(&g.japanese_events), 0);
    // The side's report carries no breakdown: the true count and the error
    // are the umpire's.
    for e in &g.allied_events {
        if let SideEventKind::Whole(EventKind::GroundReportReceived { .. }) = &e.kind {
            assert!(e.breakdown.is_none());
        }
    }
}

#[test]
fn every_scenario_built_before_this_one_flies_nothing() {
    // The frozen step 1 figures give no reconnaissance figure, so a game
    // built before this one makes no flight and gives no report from the air.
    for file in ["67_taking_a_place.toml", "69_taken_and_held.toml"] {
        let at = common::data_dir().join("scenarios").join(file);
        let t = std::fs::read_to_string(&at).expect("the scenario reads");
        let g = play_from(load_from(&t, file), BTreeMap::new());
        assert!(g.flights().is_empty(), "{file}");
        assert!(
            !g.events.iter().any(|e| matches!(
                &e.kind,
                EventKind::GroundReportReceived {
                    grade: GroundGrade::SeenFromAir,
                    ..
                }
            )),
            "{file}"
        );
    }
}

/// A game rebuilt from what was handed in to day `upto`, as a saved game
/// replays.
fn rebuilt(handed: &BTreeMap<u32, StaffDecisions>, upto: u32) -> Scenario {
    let mut scenario = load(&text());
    for day in 1..=upto {
        scenario
            .world
            .submit_staff(Side::Allied, handed[&day].clone());
        scenario
            .next_day_watched(Staffs::none(), &mut |_, _| {})
            .expect("a day resolves")
            .expect("a day to resolve");
    }
    scenario
}

#[test]
fn saved_on_day_2_and_resumed() {
    let through = main_game();
    let at_2 = rebuilt(&through.handed, 2);
    assert_eq!(at_2.world.day_hashes(), &through.hashes[..2]);
    let first: BTreeMap<u32, StaffDecisions> =
        (1..=2).map(|d| (d, through.handed[&d].clone())).collect();
    let resumed = play_from(at_2, first);
    assert_eq!(resumed.hashes, through.hashes);
    assert_eq!(resumed.handed, through.handed);
}

#[test]
fn the_same_start_gives_the_same_orders() {
    let a = play(&text());
    assert_eq!(&a.handed, &main_game().handed);
    assert_eq!(a.hashes, main_game().hashes);
}

#[test]
fn what_the_side_cannot_see_changes_nothing() {
    // An enemy unit away from B, far from anything the Allied side has seen
    // or been told: two worlds that differ only in its size.
    let enemy = |men: u32| {
        format!(
            "[[land_unit]]\nid = 9\nname = \"Placeholder enemy\"\nside = \"japanese\"\nmen = {men}\nq = 0\nr = 6\n"
        )
    };
    let with = |men: u32| {
        changed(
            &text(),
            COMMANDS_MARKER,
            &format!("{}\n{COMMANDS_MARKER}", enemy(men)),
        )
    };
    let a = play(&with(100));
    let b = play(&with(300));
    assert_eq!(a.handed, b.handed);
    assert_eq!(a.all_orders(), main_game().all_orders());
}

/// The orders that were refused when one day of `text` resolved with
/// `kinds` handed in by the player.
fn refused_when_ordered(
    text: &str,
    missions: Vec<AirMission>,
) -> Vec<sim_core::orders::RejectReason> {
    use sim_core::orders::{OrderRequest, Origin};
    let mut scenario = load(text);
    scenario.world.submit_orders(
        Side::Allied,
        missions
            .into_iter()
            .map(|mission| OrderRequest {
                kind: OrderKind::AirGroup {
                    target: group(1),
                    order: AirGroupOrder::SetAirMission(mission),
                },
                origin: Origin::PLAYER,
            })
            .collect(),
    );
    scenario.world.resolve_day().expect("the day resolves");
    scenario
        .world
        .events()
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::OrderRejected { reason, .. } => Some(reason.clone()),
            _ => None,
        })
        .collect()
}

#[test]
fn a_hex_off_the_map_is_refused_and_a_hex_on_it_is_not() {
    use sim_core::orders::RejectReason;
    let off = Hex::new(90, 90);
    let refused = refused_when_ordered(
        &text(),
        vec![
            AirMission::Reconnaissance(B),
            AirMission::Reconnaissance(off),
        ],
    );
    assert_eq!(refused, vec![RejectReason::WaypointOffMap(off)]);
}

#[test]
fn a_type_that_cannot_search_is_refused() {
    use sim_core::orders::RejectReason;
    // Aircraft type 4 is a fighter with no search range.
    let t = changed(&text(), "aircraft_type = 2", "aircraft_type = 4");
    let refused = refused_when_ordered(&t, vec![AirMission::Reconnaissance(B)]);
    assert_eq!(refused, vec![RejectReason::AircraftCannotSearch]);
}

// The flight's own refusals (items 5 and 6), by an order of the commander's
// to the group, since the staff never sends a group it could not send.

fn commander_orders_the_group_to_look_at(hex: [i32; 2]) -> String {
    format!(
        "[[order]]\nday = 1\nside = \"allied\"\nair_group = 1\nkind = \"set_air_mission\"\nmission = \"reconnaissance\"\nhex = [{}, {}]\n\n{COMMANDS_MARKER}",
        hex[0], hex[1]
    )
}

/// Plays one day with no staff and returns the flights and the reports.
fn one_day_with_no_staff(text: &str) -> Vec<Event> {
    let mut scenario = load(text);
    scenario
        .next_day_watched(Staffs::none(), &mut |_, _| {})
        .expect("a day resolves")
        .expect("a day to resolve");
    scenario.world.events().iter().cloned().collect()
}

#[test]
fn a_commanders_group_on_the_mission_flies_once_and_reports() {
    let t = changed(
        &text(),
        COMMANDS_MARKER,
        &commander_orders_the_group_to_look_at([4, 0]),
    );
    let events = one_day_with_no_staff(&t);
    let flights: Vec<&Event> = events
        .iter()
        .filter(|e| matches!(e.kind, EventKind::ReconnaissanceFlown { .. }))
        .collect();
    assert_eq!(flights.len(), 1);
    assert_eq!(
        (flights[0].day, flights[0].pulse),
        (1, Some(Pulse::Morning))
    );
}

#[test]
fn a_hex_beyond_the_types_search_range_is_not_flown_to() {
    // (5,7) is 417 nm from A; the type searches to 300 nm.
    let t = changed(
        &text(),
        COMMANDS_MARKER,
        &commander_orders_the_group_to_look_at([5, 7]),
    );
    let events = one_day_with_no_staff(&t);
    assert!(!events
        .iter()
        .any(|e| matches!(e.kind, EventKind::ReconnaissanceFlown { .. })));
    assert!(!events
        .iter()
        .any(|e| matches!(e.kind, EventKind::GroundReportReceived { .. })));
}

#[test]
fn a_hex_where_the_side_has_fighting_men_ashore_is_not_flown_to() {
    // Item 6: their report is the better one. Unit 3 and base 3 are the
    // side's own at B.
    let extra = "[[base]]\nid = 3\nname = \"Placeholder base B\"\nside = \"allied\"\nq = 4\nr = 0\nsupply_t = 100\nfuel_t = 0\n\n[[land_unit]]\nid = 3\nname = \"Placeholder Landing party\"\nside = \"allied\"\nmen = 100\nq = 4\nr = 0\n\n";
    let t = changed(
        &text(),
        COMMANDS_MARKER,
        &format!("{extra}{}", commander_orders_the_group_to_look_at([4, 0])),
    );
    let events = one_day_with_no_staff(&t);
    assert!(!events
        .iter()
        .any(|e| matches!(e.kind, EventKind::ReconnaissanceFlown { .. })));
    // Their daily report is the only one, by men in contact.
    let grades: Vec<GroundGrade> = events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::GroundReportReceived {
                side: Side::Allied,
                grade,
                ..
            } => Some(*grade),
            _ => None,
        })
        .collect();
    assert_eq!(grades, vec![GroundGrade::MenInContact]);
}

#[test]
fn a_flight_finds_a_hidden_base_in_the_hex() {
    // A Japanese base formed by a landing, off the Allied map, in the hex:
    // the report puts it on the map, with a BaseFound citing the report.
    let t = changed(
        &text(),
        "q = 4\nr = 0\nsupply_t = 100\nfuel_t = 0\n",
        "q = 4\nr = 0\nsupply_t = 100\nfuel_t = 0\non_map = false\n",
    );
    let t = changed(
        &t,
        COMMANDS_MARKER,
        &commander_orders_the_group_to_look_at([4, 0]),
    );
    let events = one_day_with_no_staff(&t);
    let report = events
        .iter()
        .find(|e| matches!(e.kind, EventKind::GroundReportReceived { .. }))
        .expect("the report");
    let found = events
        .iter()
        .find(|e| matches!(e.kind, EventKind::BaseFound { .. }))
        .expect("the base is found");
    assert_eq!(found.causes, vec![Cause::Event(report.id)]);
    assert_eq!((found.day, found.pulse), (1, Some(Pulse::Morning)));
}
