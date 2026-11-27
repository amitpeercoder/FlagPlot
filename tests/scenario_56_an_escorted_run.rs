//! Scenario 56: an escorted run. Scenario 36's convoy, with an escort
//! force named in the forces limit and a contact near the destination on
//! the first days: the staff escorts the first run and not the second.
//!
//! Proves decision 0126 against a hand calculation, with decision 0121's
//! rule beneath it. It stands on scenario 36's hand calculation for every
//! reading, load, hour and stock of the supply plan, and repeats none of
//! it. The data is placeholder data: see
//! `data/scenarios/56_an_escorted_run.toml`. It runs on the frozen step 1
//! figures. There is no enemy unit and no random draw.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything stated is
//! asserted exactly. If the code disagrees, the code or the rule is
//! wrong: do not change a number here to make a test pass. A mismatch
//! goes back to the architect.
//!
//! ## Inputs
//!
//! Scenario 36's, unchanged: S at (0,0) and T at (5,0), bases 1 and 2,
//! land unit 1, ships 1 and 2 in port at base 1, the intent to keep base 2
//! at 30 days with an interval of 15, the reserve, both leaves, 18 days.
//!
//! Added: task force 1, Allied, two placeholder destroyers, ships 3 and 4,
//! stopped at base 1, full of fuel, on the escort mission, with no charge.
//! The forces limit of the intent names it beside ships 1 and 2 and the
//! source.
//!
//! Added: the Allied side starts holding one contact: 2 ships, a
//! destroyer, at (8,0), age 0. It is 3 hexes from T. A contact gains a
//! pulse of age at the end of each pulse and is dropped at 5: it is held,
//! at age 3, on the morning of day 2, and is gone from the end of the
//! morning of day 2.
//!
//! No risk is set: it is balanced. A run is escorted on a threat, where an
//! escort is free.
//!
//! ## The first run
//!
//! The supply plan is scenario 36's: on day 1 the staff forms a force of
//! ship 1 and loads 360 t; on day 2 it routes it to T and back.
//!
//! On the morning of day 2 the contact is held, 3 hexes from T: a threat.
//! Force 1 is free: it is stopped at base 1, has no charge and is full.
//! So day 2 has one more order than in scenario 36: force 1 is given the
//! run's force as its charge.
//!
//! Force 1 keeps station (decision 0121, 3). It is in the run's hex at the
//! end of every pulse from day 2 to day 4:
//!
//! | When | The run's force | Force 1 |
//! | ---- | --------------- | ------- |
//! | Hour 24, the start of day 2 | Sails from S | Sails with it |
//! | Hour 44 | Arrives at T and unloads | Arrives and stops with it |
//! | Hour 66, the start of the afternoon of day 3 | Steams for S | Steams with it |
//! | Hour 86, in the morning of day 4 | Arrives at S | Arrives at S; its ships are filled in that pulse |
//!
//! (Restated by the architect on Nov 16, 2026, after the build: the first
//! draft put hour 86 in the afternoon.)
//!
//! On the morning of day 5 the run's force is back at the source: force 1
//! has no charge from that morning, by the staff's order to clear it or,
//! if the run's force has ceased to be, by decision 0121's rule 7. The
//! test asserts that it has none, and that the staff gives it no other
//! order.
//!
//! ## The second run
//!
//! On day 16 the staff loads 150 t and on day 17 routes the run, as in
//! scenario 36. On the morning of day 17 the side holds no contact: no
//! threat. Force 1 is given no order, and the reason of the routing says
//! no contact of ships is held within 15 hexes of T. The run sails alone.
//! Force 1 does not move after day 4.
//!
//! ## At the end
//!
//! Every reading at T, every load, every stock and ship 1's fuel are
//! scenario 36's. Force 1 is stopped at base 1 with no charge and full of
//! fuel. Base 1's fuel is scenario 36's less what force 1 took on day 4,
//! which is what a force of the same two destroyers ordered to T and back
//! at 10 kn burns: the test runs that force beside it and compares. No
//! staff order is refused.
//!
//! # Properties
//!
//! - **Press.** No run is escorted. Force 1 is given no order on any day.
//! - **Preserve the force.** Both runs are escorted: force 1 is given the
//!   run's force as its charge on day 2 and again on day 17.
//! - **Preserve, a threat and no escort.** With force 1 named and held: on
//!   day 2 the run is not routed, the staff hands in that it cannot meet
//!   the intent, and its reason gives the contact. On day 3 the contact is
//!   gone and the run is routed, a day late.
//! - **Balanced, a threat and no escort.** With force 1 named and held:
//!   the run sails alone on day 2 as in scenario 36, and its reason says a
//!   threat was believed and no escort was free.
//! - **None named, the threat is not read.** With no task force in the
//!   forces limit the supply doctrine does not read the threat, at any
//!   risk level (decision 0126, 2). (The two properties above were
//!   restated by the architect on Nov 16, 2026, after the build: the first
//!   draft took force 1 out of the forces limit, which rule 2 answers.)
//! - **An escort short of fuel is not free.** With ships 3 and 4 holding
//!   10 t each: the run sails alone on day 2, and the reason gives the
//!   fuel aboard against the fuel needed.
//! - **A held escort is left alone.**
//! - **A covering force.** With task force 2, two placeholder cruisers on
//!   the surface combat mission stopped at base 1, also named: a stand
//!   task is formed on day 1 with (5,0) as its station. T is 200 nm from
//!   S: 12.5 hours at cruise, more than the night, and 6.25 hours at 32
//!   kn. On days 1 and 2, with the contact held 3 hexes from the station,
//!   force 2 is ordered to full speed and routed to (5,0) and back,
//!   standing at (5,0); from day 3 it stays in port.
//! - **None named, the game as it was.** With no task force in the forces
//!   limit, every order and event is scenario 36's.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see give the same orders and reasons.

// ---------------------------------------------------------------- The harness
//
// Played as scenario 36 plays: each day through `next_day_watched` with the
// doctrine's staffs, and what the staff handed in each day read from
// `staff_handed_in`. The figure at T is read at the staff's moment.

use crate::common;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use sim_core::choices::Moment;
use sim_core::command::{NoticeBasis, NoticeKind, Reason, StaffDecisions, StaffOrder};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, ShipId, TaskForceId};
use sim_core::orders::{OrderKind, TaskForceOrder};
use sim_core::state::{ShipLocation, Side, TruthState};
use sim_core::time::Pulse;
use sim_core::DaysOfSupply;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

const S: Hex = Hex::new(0, 0);
const T: Hex = Hex::new(5, 0);

/// The number the engine gives the run's force, formed on day 1.
const FORCE: u32 = 1_000_001;

const COMMANDS_MARKER: &str = "# The Allied commander's commands";

fn path() -> PathBuf {
    common::data_dir().join("scenarios/56_an_escorted_run.toml")
}

fn path_36() -> PathBuf {
    common::data_dir().join("scenarios/36_one_convoy.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn text_36() -> String {
    std::fs::read_to_string(path_36()).expect("scenario 36's file reads")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

fn ship(raw: u32) -> ShipId {
    ShipId::new(raw)
}

fn base(raw: u32) -> BaseId {
    BaseId::new(raw)
}

fn force(raw: u32) -> TaskForceId {
    TaskForceId::new(raw)
}

fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

/// A command table of the Allied side.
fn command_table(day: u32, body: &str) -> String {
    format!("\n[[command]]\nday = {day}\nside = \"allied\"\n{body}\n")
}

fn with(extra: &str) -> String {
    format!("{}{}", text(), extra)
}

fn risk(level: &str) -> String {
    command_table(
        1,
        &format!("kind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"{level}\""),
    )
}

/// The same game with force 1 out of the forces limit.
fn force_1_not_named(text: &str) -> String {
    changed(text, "task_forces = [1]", "task_forces = []")
}

/// Force 1 held by the commander from day 1.
fn force_1_held() -> String {
    command_table(1, "kind = \"hold\"\ntask_force = 1")
}

/// A game played to its end with the doctrine's staffs.
struct Played {
    readings: BTreeMap<u32, Option<DaysOfSupply>>,
    handed: BTreeMap<u32, StaffDecisions>,
    /// The truth at the end of each day; index 0 is the start.
    truth: Vec<TruthState>,
    events: Vec<Event>,
    state: sim_core::command::CommandState,
}

fn play(text: &str) -> Played {
    play_at(text, &path())
}

fn play_at(text: &str, at: &Path) -> Played {
    let mut scenario: Scenario = common::from_text(text, at).expect("the scenario loads");
    let held = staffs();
    let mut readings = BTreeMap::new();
    let mut handed = BTreeMap::new();
    let mut truth = vec![scenario.world.umpire().clone()];
    loop {
        let day = scenario.world.day() + 1;
        let mut reading = None;
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |world, moment| {
                if moment == (Moment::Orders { day }) {
                    reading = Some(world.side_view(Side::Allied).days_of_supply(T));
                }
            })
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
        readings.insert(day, reading.expect("the staff's moment came"));
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
    }
    Played {
        readings,
        handed,
        truth,
        events: scenario.world.events().iter().cloned().collect(),
        state: scenario.world.command_state(Side::Allied).clone(),
    }
}

fn order_rank(p: Pulse) -> u32 {
    match p {
        Pulse::Night => 0,
        Pulse::Morning => 1,
        Pulse::Afternoon => 2,
    }
}

impl Played {
    fn at(&self, day: u32) -> &TruthState {
        &self.truth[day as usize]
    }

    fn decisions(&self, day: u32) -> &StaffDecisions {
        &self.handed[&day]
    }

    fn orders(&self) -> Vec<(u32, &StaffOrder)> {
        self.handed
            .iter()
            .flat_map(|(day, d)| d.orders.iter().map(move |o| (*day, o)))
            .collect()
    }

    /// The task-force orders of a day to force `raw`.
    fn orders_to(&self, day: u32, raw: u32) -> Vec<&TaskForceOrder> {
        self.decisions(day)
            .orders
            .iter()
            .filter_map(|o| match &o.kind {
                OrderKind::TaskForce { target, order } if *target == force(raw) => Some(order),
                _ => None,
            })
            .collect()
    }

    /// Every staff order to force `raw`, by day.
    fn all_orders_to(&self, raw: u32) -> Vec<(u32, &StaffOrder)> {
        self.orders()
            .into_iter()
            .filter(|(_, o)| {
                matches!(&o.kind, OrderKind::TaskForce { target, .. } if *target == force(raw))
            })
            .collect()
    }

    fn reason_of_order_to(&self, day: u32, raw: u32, wanted: &TaskForceOrder) -> &Reason {
        &self
            .decisions(day)
            .orders
            .iter()
            .find(|o| {
                matches!(&o.kind, OrderKind::TaskForce { target, order }
                    if *target == force(raw) && order == wanted)
            })
            .unwrap_or_else(|| panic!("day {day} has no order {wanted:?} to force {raw}"))
            .reason
    }

    /// The reason of a day's route order to the run's force.
    fn route_reason(&self, day: u32) -> &Reason {
        &self
            .decisions(day)
            .orders
            .iter()
            .find(|o| {
                matches!(&o.kind, OrderKind::TaskForce { target, order: TaskForceOrder::SetRoute { .. } }
                    if *target == force(FORCE))
            })
            .unwrap_or_else(|| panic!("day {day} has no route order"))
            .reason
    }

    fn all(&self, pick: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
        self.events.iter().filter(|e| pick(&e.kind)).collect()
    }

    fn nothing_refused(&self) {
        let refused: Vec<&Event> = self.all(|k| {
            matches!(
                k,
                EventKind::OrderRejected { .. } | EventKind::CommandRefused { .. }
            )
        });
        assert!(refused.is_empty(), "something was refused: {refused:?}");
    }

    fn base_supply(&self, day: u32, raw: u32) -> Fixed {
        self.at(day).bases[&base(raw)].supply_t
    }

    fn base_fuel(&self, day: u32, raw: u32) -> Fixed {
        self.at(day).bases[&base(raw)].fuel_t
    }

    fn ship_fuel(&self, day: u32, raw: u32) -> Fixed {
        self.at(day).ships[&ship(raw)].fuel_t
    }

    /// The hex a force is in at the end of a pulse, from the log: where the
    /// last movement of or before the pulse left it.
    fn position_after(&self, raw: u32, day: u32, pulse: Pulse) -> Hex {
        // The run's force is formed on day 1, from ship 1 in port at S.
        let mut at = self.truth[0]
            .task_forces
            .get(&force(raw))
            .map_or(S, |f| f.position());
        for e in &self.events {
            let Some(p) = e.pulse else { continue };
            if (e.day, order_rank(p)) > (day, order_rank(pulse)) {
                break;
            }
            if let EventKind::TaskForceSteamed { task_force, to, .. } = &e.kind {
                if *task_force == force(raw) {
                    at = *to;
                }
            }
        }
        at
    }

    /// The hour (elapsed since the scenario began) a force arrived at a hex.
    fn arrived_at(&self, day: u32, raw: u32, hex: Hex) -> Option<Fixed> {
        self.events
            .iter()
            .filter(|e| e.day == day)
            .find_map(|e| match &e.kind {
                EventKind::TaskForceArrived {
                    task_force,
                    hex: h,
                    elapsed_hours,
                } if *task_force == force(raw) && *h == hex => Some(*elapsed_hours),
                _ => None,
            })
    }

    fn halted_at(&self, day: u32, raw: u32, hex: Hex) -> Option<Fixed> {
        self.events
            .iter()
            .filter(|e| e.day == day)
            .find_map(|e| match &e.kind {
                EventKind::TaskForceHalted {
                    task_force,
                    hex: h,
                    elapsed_hours,
                    ..
                } if *task_force == force(raw) && *h == hex => Some(*elapsed_hours),
                _ => None,
            })
    }

    /// The pulse and day each ship of a force was filled at a base.
    fn refuelled(&self, raw_ship: u32) -> Vec<(u32, Option<Pulse>, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::ShipRefuelled {
                    ship: s, taken_t, ..
                } if *s == ship(raw_ship) => Some((e.day, e.pulse, *taken_t)),
                _ => None,
            })
            .collect()
    }

    fn cannot_meet(&self, day: u32) -> Vec<&sim_core::command::StaffNotice> {
        self.decisions(day)
            .notices
            .iter()
            .filter(|n| n.kind == NoticeKind::CannotMeet)
            .collect()
    }
}

/// A reason's text: its calculation and its evidence.
fn said(reason: &Reason) -> String {
    let evidence: Vec<&str> = reason.evidence.iter().map(|e| e.what.as_str()).collect();
    format!("{} | {}", reason.worked, evidence.join(" | "))
}

fn route_run() -> TaskForceOrder {
    TaskForceOrder::SetRoute {
        waypoints: vec![T, S],
        act_at: [T].into_iter().collect(),
        stand_at: BTreeSet::new(),
    }
}

fn escort(charge: Option<u32>) -> TaskForceOrder {
    TaskForceOrder::Escort(charge.map(force))
}

// ------------------------------------------------------- The hand calculation

#[test]
fn the_first_run_is_escorted_with_one_more_order_than_scenario_36s() {
    let played = play(&text());
    let reference = play_at(&text_36(), &path_36());
    played.nothing_refused();
    reference.nothing_refused();

    // Day 1 is scenario 36's, order for order: the force 1 is not touched.
    assert_eq!(played.decisions(1).orders.len(), 2);
    assert_eq!(played.decisions(1).commands.len(), 4);
    assert_eq!(
        played
            .decisions(1)
            .orders
            .iter()
            .map(|o| &o.kind)
            .collect::<Vec<_>>(),
        reference
            .decisions(1)
            .orders
            .iter()
            .map(|o| &o.kind)
            .collect::<Vec<_>>()
    );

    // Day 2: scenario 36 gave 2 orders (the route and the unload); here
    // there are 3, the escort order first.
    assert_eq!(reference.decisions(2).orders.len(), 2);
    let d2 = played.decisions(2);
    assert_eq!(d2.orders.len(), 3);
    assert_eq!(
        d2.orders[0].kind,
        OrderKind::TaskForce {
            target: force(1),
            order: escort(Some(FORCE)),
        }
    );
    // The other two are scenario 36's, in order.
    assert_eq!(d2.orders[1].kind, reference.decisions(2).orders[0].kind);
    assert_eq!(d2.orders[2].kind, reference.decisions(2).orders[1].kind);
    assert_eq!(d2.orders[0].intent, IntentId::new(1));
    assert!(d2.commands.is_empty() && d2.no_orders.is_empty());
}

#[test]
fn force_1_is_in_the_runs_hex_at_the_end_of_every_pulse_from_day_2_to_day_4() {
    let played = play(&text());
    for day in 2..=4 {
        for pulse in [Pulse::Night, Pulse::Morning, Pulse::Afternoon] {
            assert_eq!(
                played.position_after(1, day, pulse),
                played.position_after(FORCE, day, pulse),
                "day {day}, {pulse:?}"
            );
        }
    }
    // Where that is: at T by the end of the morning of day 2, and on its way
    // home from the afternoon of day 3.
    assert_eq!(played.position_after(1, 2, Pulse::Night), Hex::new(3, 0));
    assert_eq!(played.position_after(1, 2, Pulse::Morning), T);
    assert_eq!(played.position_after(1, 2, Pulse::Afternoon), T);
    assert_eq!(played.position_after(1, 3, Pulse::Morning), T);
    assert_eq!(
        played.position_after(1, 3, Pulse::Afternoon),
        Hex::new(3, 0)
    );
    assert_eq!(played.position_after(1, 4, Pulse::Afternoon), S);
    // At the end of each day, from the truth.
    assert_eq!(played.at(1).task_forces[&force(1)].position(), S);
    assert_eq!(played.at(2).task_forces[&force(1)].position(), T);
    assert_eq!(
        played.at(3).task_forces[&force(1)].position(),
        Hex::new(3, 0)
    );
    assert_eq!(played.at(4).task_forces[&force(1)].position(), S);
    // It keeps station: the engine says so on each pulse it steams with it.
    let kept = played.all(|k| matches!(k, EventKind::StationKept { .. }));
    assert!(!kept.is_empty());
    for e in &kept {
        match &e.kind {
            EventKind::StationKept { escort, charge, .. } => {
                assert_eq!((*escort, *charge), (force(1), force(FORCE)));
            }
            _ => unreachable!(),
        }
    }
    assert!(played
        .all(|k| matches!(
            k,
            EventKind::ChargeLost { .. } | EventKind::MeetingNotHeld { .. }
        ))
        .is_empty());
}

#[test]
fn hours_44_and_86_force_1_arrives_with_the_run_and_its_ships_are_filled_at_s() {
    let played = play(&text());
    // Hour 44: the run's force arrives at T and unloads; force 1 arrives and
    // stops with it.
    assert_eq!(played.halted_at(2, FORCE, T), Some(n(44)));
    assert_eq!(played.arrived_at(2, 1, T), Some(n(44)));
    // Hour 86: the run's force arrives at S, and force 1 with it.
    assert_eq!(played.arrived_at(4, FORCE, S), Some(n(86)));
    assert_eq!(played.arrived_at(4, 1, S), Some(n(86)));
    // Its ships are filled at S on day 4, in the pulse it arrives.
    for raw in [3, 4] {
        let filled = played.refuelled(raw);
        assert_eq!(filled.len(), 1, "ship {raw} is filled once");
        assert_eq!(filled[0].0, 4, "ship {raw}");
        assert_eq!(filled[0].2, n(50), "ship {raw} takes 50 t");
        assert_eq!(played.ship_fuel(4, raw), n(500));
    }
}

/// The table: at hour 86, in the morning of day 4, force 1 arrives at S
/// and its ships are filled in that pulse.
#[test]
fn the_ships_are_filled_in_the_morning_of_day_4() {
    let played = play(&text());
    for raw in [3, 4] {
        let filled = played.refuelled(raw);
        assert_eq!(filled.len(), 1, "ship {raw}");
        assert_eq!(filled[0].0, 4, "ship {raw}");
        assert_eq!(filled[0].1, Some(Pulse::Morning), "ship {raw}");
    }
}

#[test]
fn on_day_5_force_1_has_no_charge_and_the_staffs_only_order_to_it_is_the_clear() {
    let played = play(&text());
    assert!(played.at(4).task_forces[&force(1)].charge.is_some());
    assert!(played.at(5).task_forces[&force(1)].charge.is_none());
    assert_eq!(played.orders_to(5, 1), vec![&escort(None)]);
    // It is the only order to force 1 on any day from 3 to 16.
    for day in 3..=16 {
        let orders = played.orders_to(day, 1);
        if day == 5 {
            assert_eq!(orders, vec![&escort(None)]);
        } else {
            assert!(orders.is_empty(), "day {day}: {orders:?}");
        }
    }
    // The orders to force 1 over the whole game: the escort of day 2 and
    // the clear of day 5.
    let to_1: Vec<(u32, &TaskForceOrder)> = played
        .all_orders_to(1)
        .into_iter()
        .map(|(d, o)| match &o.kind {
            OrderKind::TaskForce { order, .. } => (d, order),
            _ => unreachable!(),
        })
        .collect();
    assert_eq!(
        to_1,
        vec![(2, &escort(Some(FORCE))), (5, &escort(None))],
        "the orders to force 1"
    );
    let clear = played.reason_of_order_to(5, 1, &escort(None));
    assert!(clear.worked.contains("home"), "{}", clear.worked);
    assert!(clear.worked.contains("charge"), "{}", clear.worked);
}

#[test]
fn the_second_run_has_no_escort_and_the_reason_says_no_contact_is_held_within_15_hexes() {
    let played = play(&text());
    // No contact is held on the morning of day 17.
    let d17 = played.decisions(17);
    assert_eq!(d17.orders.len(), 2, "the route and the unload only");
    assert!(played.orders_to(17, 1).is_empty());
    assert_eq!(
        d17.orders[0].kind,
        OrderKind::TaskForce {
            target: force(FORCE),
            order: route_run(),
        }
    );
    let said = played.route_reason(17).worked.clone();
    assert!(
        said.contains("no contact of ships is held within 15 hexes"),
        "{said}"
    );
    // The run sails alone, and force 1 does not move after day 4.
    for day in 5..=18 {
        assert_eq!(
            played.at(day).task_forces[&force(1)].position(),
            S,
            "day {day}"
        );
    }
    assert_eq!(played.at(18).task_forces[&force(1)].position(), S);
    for e in played.events.iter().filter(|e| e.day >= 5) {
        if let EventKind::TaskForceSteamed { task_force, .. } = &e.kind {
            assert_ne!(*task_force, force(1), "force 1 steamed on day {}", e.day);
        }
    }
}

#[test]
fn at_the_end_everything_is_scenario_36s_save_the_fuel_force_1_took() {
    let played = play(&text());
    let reference = play_at(&text_36(), &path_36());
    // Every reading at T.
    assert_eq!(played.readings, reference.readings);
    // Every load and stock, every day: base 2 and base 1's supply.
    for day in 0..=18 {
        for raw in [1, 2] {
            assert_eq!(
                played.base_supply(day, raw),
                reference.base_supply(day, raw),
                "supply at base {raw}, day {day}"
            );
        }
        for raw in [1, 2] {
            assert_eq!(
                played.at(day).ships[&ship(raw)].fuel_t,
                reference.at(day).ships[&ship(raw)].fuel_t,
                "fuel of ship {raw}, day {day}"
            );
            assert_eq!(
                played.at(day).ships[&ship(raw)].supply_t,
                reference.at(day).ships[&ship(raw)].supply_t,
                "load of ship {raw}, day {day}"
            );
        }
    }
    assert_eq!(played.ship_fuel(18, 1), n(968));
    assert_eq!(played.base_supply(18, 2), n(430));
    assert_eq!(played.base_supply(18, 1), n(4490));
    // The runs are the same: the same loads and the same unloading.
    let tons = |p: &Played| -> Vec<(u32, Fixed)> {
        p.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::CargoUnloaded { unloaded_t, .. } => Some((e.day, *unloaded_t)),
                EventKind::CargoLoaded { loaded_t, .. } => Some((e.day, *loaded_t)),
                _ => None,
            })
            .collect()
    };
    assert_eq!(tons(&played), tons(&reference));
    // Force 1 stopped at base 1 with no charge, full.
    let one = &played.at(18).task_forces[&force(1)];
    assert!(one.charge.is_none());
    assert_eq!(one.position(), S);
    for raw in [3, 4] {
        let s = &played.at(18).ships[&ship(raw)];
        assert_eq!(s.fuel_t, n(500), "ship {raw}");
        assert_eq!(s.location, ShipLocation::TaskForce(force(1)));
    }
    // Base 1's fuel: scenario 36's less what two destroyers ordered to T and
    // back at 10 kn burn, which a comparison game runs beside it.
    let comparison = comparison_force();
    comparison.nothing_refused();
    let burned = n(100);
    for raw in [3, 4] {
        // 400 nm at 10 kn burns 50 t a ship, taken again at S.
        let taken: Fixed = comparison
            .refuelled(raw)
            .iter()
            .map(|r| r.2)
            .fold(Fixed::ZERO, |a, b| a + b);
        assert_eq!(taken, n(50), "ship {raw} of the comparison force");
    }
    for day in 5..=18 {
        assert_eq!(
            comparison.base_fuel(day, 1),
            reference.base_fuel(day, 1) - burned,
            "comparison, day {day}"
        );
        assert_eq!(
            played.base_fuel(day, 1),
            reference.base_fuel(day, 1) - burned,
            "base 1's fuel, day {day}"
        );
        assert_eq!(played.base_fuel(day, 1), comparison.base_fuel(day, 1));
    }
    assert_eq!(played.base_fuel(18, 1), n(1860));
    // And no staff order was refused.
    played.nothing_refused();
}

/// Scenario 36 with two placeholder destroyers, ships 3 and 4, in a force at
/// S, ordered on day 2 to T and back at 10 kn: what the escort burns.
fn comparison_force() -> Played {
    let extra = r#"
[[task_force]]
id = 1
name = "Placeholder comparison force 1"
side = "allied"
at_base = 1
mission = "escort"

[[ship]]
id = 3
name = "Placeholder destroyer 3"
class = 2
side = "allied"
fuel_t = 500
task_force = 1

[[ship]]
id = 4
name = "Placeholder destroyer 4"
class = 2
side = "allied"
fuel_t = 500
task_force = 1

[[order]]
day = 2
side = "allied"
task_force = 1
kind = "set_route"
waypoints = [[5, 0], [0, 0]]

[[order]]
day = 2
side = "allied"
task_force = 1
kind = "set_speed"
knots = "10"
"#;
    // The extra tables come before the commander's commands so that they
    // are not read as part of a command.
    let t = text_36();
    let at = t.find(COMMANDS_MARKER).expect("the marker");
    let source = format!("{}{}\n{}", &t[..at], extra, &t[at..]);
    play_at(&source, &path_36())
}

// ---------------------------------------------------------------- Properties

/// No order was given to force `raw` on any day.
fn no_order_to(played: &Played, raw: u32) {
    assert!(
        played.all_orders_to(raw).is_empty(),
        "orders to force {raw}: {:?}",
        played.all_orders_to(raw)
    );
}

/// The run's day-2 orders as scenario 36 has them: the route and the unload.
fn sails_as_in_36(played: &Played, day: u32) {
    let reference = play_at(&text_36(), &path_36());
    let d = played.decisions(day);
    assert_eq!(d.orders.len(), 2, "day {day}");
    assert_eq!(d.orders[0].kind, reference.decisions(day).orders[0].kind);
    assert_eq!(d.orders[1].kind, reference.decisions(day).orders[1].kind);
}

#[test]
fn press_no_run_is_escorted_and_force_1_is_given_no_order_on_any_day() {
    let played = play(&with(&risk("press")));
    played.nothing_refused();
    no_order_to(&played, 1);
    sails_as_in_36(&played, 2);
    sails_as_in_36(&played, 17);
    // The reason says so: a threat is believed on day 2 and not on day 17,
    // and at press no run is escorted.
    let r2 = played.route_reason(2).worked.clone();
    assert!(r2.contains("press"), "{r2}");
    assert!(r2.contains("never escorted"), "{r2}");
    assert!(r2.contains("3 hexes"), "{r2}");
    // Force 1 never moves; the run is scenario 36's.
    for day in 0..=18 {
        assert_eq!(
            played.at(day).task_forces[&force(1)].position(),
            S,
            "day {day}"
        );
        assert!(played.at(day).task_forces[&force(1)].charge.is_none());
    }
    let reference = play_at(&text_36(), &path_36());
    for day in 0..=18 {
        assert_eq!(played.base_supply(day, 2), reference.base_supply(day, 2));
    }
    assert_eq!(played.base_fuel(18, 1), reference.base_fuel(18, 1));
}

#[test]
fn preserve_both_runs_are_escorted_on_day_2_and_again_on_day_17() {
    let played = play(&with(&risk("preserve")));
    played.nothing_refused();
    for day in [2, 17] {
        let d = played.decisions(day);
        assert_eq!(d.orders.len(), 3, "day {day}");
        assert_eq!(
            d.orders[0].kind,
            OrderKind::TaskForce {
                target: force(1),
                order: escort(Some(FORCE)),
            },
            "day {day}"
        );
        assert_eq!(played.orders_to(day, 1).len(), 1);
    }
    // The clear of day 5 and of day 18 or later; none between.
    assert_eq!(played.orders_to(5, 1), vec![&escort(None)]);
    // Escorted on day 17: it is in the run's hex at the end of day 17.
    assert_eq!(played.at(17).task_forces[&force(1)].position(), T);
    // The margin of preserve is 1.5: 50 t out and back at 10 kn, 75 t
    // needed against 500 t aboard.
    let reason = played.reason_of_order_to(2, 1, &escort(Some(FORCE)));
    let text = said(reason);
    assert!(text.contains("1.5 at preserve risk"), "{text}");
    assert!(
        text.contains("ship:3 has 500 t aboard against 75 t needed"),
        "{text}"
    );
    assert!(
        text.contains("ship:4 has 500 t aboard against 75 t needed"),
        "{text}"
    );
    // On day 17 no threat is held and the run is escorted all the same.
    let text17 = said(played.reason_of_order_to(17, 1, &escort(Some(FORCE))));
    assert!(
        text17.contains("no contact of ships is held within 15 hexes"),
        "{text17}"
    );
    // The run is scenario 36's, and force 1 is full at the end.
    let reference = play_at(&text_36(), &path_36());
    assert_eq!(played.readings, reference.readings);
    // Force 1 is at sea with the second run at the end of day 18, and was
    // full when it left on day 17.
    assert!(played.ship_fuel(16, 3) == n(500) && played.ship_fuel(16, 4) == n(500));
}

#[test]
fn preserve_a_threat_and_no_escort_free_force_1_named_and_held() {
    let played = play(&with(&format!("{}{}", risk("preserve"), force_1_held())));
    played.nothing_refused();
    // Day 2: the run is not routed, the staff cannot meet the intent, and
    // its reason gives the contact.
    let d2 = played.decisions(2);
    assert!(d2.orders.is_empty());
    let notices = played.cannot_meet(2);
    assert_eq!(notices.len(), 1);
    assert_eq!(notices[0].rests_on, NoticeBasis::NoOrder(0));
    let why = said(&d2.no_orders[0]);
    assert!(
        why.contains("contact 1 of 2 ship(s) is 3 hexes from (5,0)"),
        "{why}"
    );
    assert!(
        why.contains("a threatened run does not sail without an escort"),
        "{why}"
    );
    assert!(why.contains("no escort is free"), "{why}");
    assert!(why.contains("held by the commander"), "{why}");
    // Nothing sailed on day 2.
    for pulse in [Pulse::Night, Pulse::Morning, Pulse::Afternoon] {
        assert_eq!(played.position_after(FORCE, 2, pulse), S);
    }
    // Day 3: the contact is gone and the run is routed, a day late.
    assert!(played.cannot_meet(3).is_empty());
    let d3 = played.decisions(3);
    assert_eq!(d3.orders.len(), 2);
    assert_eq!(
        d3.orders[0].kind,
        OrderKind::TaskForce {
            target: force(FORCE),
            order: route_run(),
        }
    );
    let sails = &played.route_reason(3).worked;
    assert!(
        sails.contains("no contact of ships is held within 15 hexes"),
        "{sails}"
    );
    // A day late: the run lands a day after scenario 36's.
    assert_eq!(played.halted_at(3, FORCE, T), Some(n(68)));
    // Force 1, held, was never ordered or moved.
    no_order_to(&played, 1);
    assert_eq!(played.at(18).task_forces[&force(1)].position(), S);
}

#[test]
fn balanced_a_threat_and_no_escort_free_force_1_named_and_held() {
    let played = play(&with(&force_1_held()));
    played.nothing_refused();
    // The run sails alone on day 2, as in scenario 36.
    sails_as_in_36(&played, 2);
    let reason = &played.route_reason(2).worked;
    assert!(reason.contains("A threat is believed"), "{reason}");
    assert!(
        reason.contains("contact 1 of 2 ship(s) is 3 hexes from (5,0)"),
        "{reason}"
    );
    assert!(reason.contains("no escort is free"), "{reason}");
    assert!(reason.contains("so the run sails alone"), "{reason}");
    assert!(played.cannot_meet(2).is_empty());
    no_order_to(&played, 1);
    // The run is scenario 36's.
    let reference = play_at(&text_36(), &path_36());
    for day in 0..=18 {
        assert_eq!(played.base_supply(day, 2), reference.base_supply(day, 2));
    }
    assert_eq!(played.halted_at(2, FORCE, T), Some(n(44)));
}

/// With ships 3 and 4 holding 10 t each. The ships of a force stopped at a
/// base are filled in the night of day 1 from the base's fuel, so 10 t each
/// is a filled force unless the base has no fuel: base 1 holds none here
/// (scenario 54 does the same for "fuel comes first"). Without that the
/// escort is full on day 2 and is given the charge.
#[test]
fn an_escort_short_of_fuel_is_not_free_the_run_sails_alone() {
    let short = text()
        .replace("fuel_t = 500", "fuel_t = 10")
        .replace("fuel_t = 2000", "fuel_t = 0");
    let played = play(&short);
    played.nothing_refused();
    assert_eq!(played.ship_fuel(1, 3), n(10));
    assert_eq!(played.ship_fuel(1, 4), n(10));
    sails_as_in_36(&played, 2);
    let reason = &played.route_reason(2).worked;
    // The reason gives the fuel aboard against the fuel needed: 400 nm at
    // 500 t over 4000 nm is 50 t, times 1.2 at balanced risk, 60 t.
    assert!(
        reason.contains("ship:3 has 10 t aboard against 60 t needed"),
        "{reason}"
    );
    assert!(
        reason.contains("ship:4 has 10 t aboard against 60 t needed"),
        "{reason}"
    );
    assert!(reason.contains("task_force:1 is short of fuel"), "{reason}");
    assert!(reason.contains("no escort is free"), "{reason}");
    assert!(reason.contains("so the run sails alone"), "{reason}");
    no_order_to(&played, 1);
    assert_eq!(played.at(18).task_forces[&force(1)].position(), S);
}

#[test]
fn a_held_escort_is_left_alone() {
    let played = play(&with(&force_1_held()));
    played.nothing_refused();
    no_order_to(&played, 1);
    for day in 0..=18 {
        let f = &played.at(day).task_forces[&force(1)];
        assert_eq!(f.position(), S, "day {day}");
        assert!(f.charge.is_none(), "day {day}");
    }
    for e in &played.events {
        if let EventKind::TaskForceSteamed { task_force, .. } = &e.kind {
            assert_ne!(*task_force, force(1));
        }
    }
    let reason = played.route_reason(2).worked.clone();
    assert!(reason.contains("held by the commander"), "{reason}");
    assert_eq!(played.ship_fuel(18, 3), n(500));
    assert_eq!(played.ship_fuel(18, 4), n(500));
}

const COVER: &str = r#"
[[task_force]]
id = 2
name = "Placeholder covering force 2"
side = "allied"
at_base = 1
mission = "surface_combat"

[[ship]]
id = 5
name = "Placeholder cruiser 5"
class = 1
side = "allied"
fuel_t = 2000
task_force = 2

[[ship]]
id = 6
name = "Placeholder cruiser 6"
class = 1
side = "allied"
fuel_t = 2000
task_force = 2
"#;

/// The file with a covering force, task force 2, added before the commands
/// and named beside force 1.
fn covered() -> String {
    let t = text();
    let at = t.find(COMMANDS_MARKER).expect("the marker");
    let source = format!("{}{}\n{}", &t[..at], COVER, &t[at..]);
    changed(&source, "task_forces = [1]", "task_forces = [1, 2]")
}

fn speed_full() -> TaskForceOrder {
    TaskForceOrder::SetSpeed(sim_core::orders::SpeedSetting::Full)
}

fn stand_at_t() -> TaskForceOrder {
    TaskForceOrder::SetRoute {
        waypoints: vec![T, S],
        act_at: BTreeSet::new(),
        stand_at: [T].into_iter().collect(),
    }
}

#[test]
fn a_covering_force_stands_at_t_on_days_1_and_2_and_stays_in_port_from_day_3() {
    let played = play(&covered());
    played.nothing_refused();
    // Day 1: the stand task is formed with (5,0) as its station: full speed
    // and the route to (5,0) and home, (5,0) marked to stand at.
    let d1: Vec<&TaskForceOrder> = played.orders_to(1, 2);
    assert!(d1.contains(&&speed_full()), "{d1:?}");
    assert!(d1.contains(&&stand_at_t()), "{d1:?}");
    let reason = said(played.reason_of_order_to(1, 2, &speed_full()));
    assert!(
        reason.contains("12.5 hours away at cruise and 6.25 at full speed"),
        "{reason}"
    );
    assert!(
        reason.contains("task_force:2 is set to full speed"),
        "{reason}"
    );
    let route = said(played.reason_of_order_to(1, 2, &stand_at_t()));
    assert!(
        route.contains("contact 1 of 2 ship(s) is 3 hexes from the station (5,0)"),
        "{route}"
    );
    assert!(
        route.contains("6.25 hours to the station at full speed"),
        "{route}"
    );
    // Day 2: routed again; the speed is still full.
    let d2 = played.orders_to(2, 2);
    assert!(d2.contains(&&stand_at_t()), "{d2:?}");
    assert!(played.at(2).task_forces[&force(2)].speed == sim_core::orders::SpeedSetting::Full);
    // Days 1 and 2: standing at (5,0) in the night and home by the end of
    // the day.
    for day in [1, 2] {
        assert_eq!(played.position_after(2, day, Pulse::Night), T, "day {day}");
        assert_eq!(
            played.position_after(2, day, Pulse::Afternoon),
            S,
            "day {day}"
        );
        assert_eq!(played.at(day).task_forces[&force(2)].position(), S);
    }
    let halts: Vec<Fixed> = played
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::TaskForceHalted {
                task_force,
                hex,
                elapsed_hours,
                ..
            } if *task_force == force(2) && *hex == T => Some(*elapsed_hours),
            _ => None,
        })
        .collect();
    assert_eq!(halts, vec![fx("6.25"), fx("30.25")]);
    // From day 3 it stays in port: no route, no movement.
    for day in 3..=18 {
        assert!(
            !played
                .orders_to(day, 2)
                .iter()
                .any(|o| matches!(o, TaskForceOrder::SetRoute { .. })),
            "day {day}"
        );
        assert_eq!(
            played.at(day).task_forces[&force(2)].position(),
            S,
            "day {day}"
        );
    }
    for e in played.events.iter().filter(|e| e.day >= 3) {
        if let EventKind::TaskForceSteamed { task_force, .. } = &e.kind {
            assert_ne!(*task_force, force(2), "day {}", e.day);
        }
    }
    // The escort still goes with the run on day 2, as without the cover.
    assert_eq!(played.orders_to(2, 1), vec![&escort(Some(FORCE))]);
    let alone = play(&text());
    assert_eq!(played.readings, alone.readings);
}

#[test]
fn none_named_the_game_as_it_was_every_order_and_event_is_scenario_36s() {
    let none = force_1_not_named(&text());
    let played = play(&none);
    let reference = play_at(&text_36(), &path_36());
    played.nothing_refused();
    // Every order, command, reason and notice of every day.
    assert_eq!(played.handed, reference.handed);
    // Every event, by what it says, leaving out the contact's own events
    // (scenario 36 has no contact).
    let story = |p: &Played| -> Vec<String> {
        p.events
            .iter()
            .map(|e| format!("D{} {:?} {}", e.day, e.pulse, e.kind))
            .filter(|line| !line.contains("contact"))
            .collect()
    };
    let (a, b) = (story(&played), story(&reference));
    assert_eq!(a.len(), b.len());
    assert_eq!(a, b);
    assert_eq!(played.readings, reference.readings);
    for day in 0..=18 {
        assert_eq!(played.base_fuel(day, 1), reference.base_fuel(day, 1));
        assert_eq!(played.base_supply(day, 2), reference.base_supply(day, 2));
    }
    // Force 1 never moves.
    no_order_to(&played, 1);
}

/// None named, the threat is not read, at any risk level: with no task
/// force in the forces limit, at preserve and at press as at balanced, every
/// order, command, reason and notice is scenario 36's at that risk.
#[test]
fn none_named_the_threat_is_not_read_at_any_risk_level() {
    for level in ["preserve", "press"] {
        let played = play(&force_1_not_named(&with(&risk(level))));
        let reference = play_at(&format!("{}{}", text_36(), risk(level)), &path_36());
        played.nothing_refused();
        assert_eq!(played.handed, reference.handed, "{level}");
        assert_eq!(played.readings, reference.readings, "{level}");
        no_order_to(&played, 1);
    }
}

#[test]
fn the_same_start_gives_the_same_orders_and_reasons() {
    let a = play(&text());
    let b = play(&text());
    assert_eq!(a.handed, b.handed);
    assert_eq!(a.state, b.state);
    assert_eq!(a.events.len(), b.events.len());
    assert_eq!(a.readings, b.readings);
}

#[test]
fn two_worlds_that_differ_only_in_what_the_allied_side_cannot_see_give_the_same_orders() {
    // A third coast hex, U, with a Japanese base, a ship and a unit on it.
    let map_text =
        std::fs::read_to_string(common::data_dir().join("map/placeholder_one_convoy.toml"))
            .expect("the map reads")
            + "\n[[hex]]\nq = 3\nr = 2\nkind = \"coast\"\nport_size = 1\n";
    let dir = std::env::temp_dir().join(format!("flag_plot_s56_{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a directory");
    let map_path = dir.join("map.toml");
    std::fs::write(&map_path, map_text).expect("the map is written");
    let enemy = |supply: u32, fuel: u32, men: u32, ship_fuel: u32| {
        format!(
            r#"
[[base]]
id = 7
name = "Placeholder base U, Japanese"
side = "japanese"
q = 3
r = 2
supply_t = {supply}
fuel_t = {fuel}
on_map = false

[[land_unit]]
id = 9
name = "Placeholder unit 9"
side = "japanese"
men = {men}
q = 3
r = 2
posture = "defend"

[[ship]]
id = 11
name = "Placeholder cargo ship 11"
class = 3
side = "allied"
fuel_t = {ship_fuel}
port = 7
"#
        )
        .replace("side = \"allied\"", "side = \"japanese\"")
    };
    let on_the_new_map = changed(
        &text(),
        "map = \"../map/placeholder_one_convoy.toml\"",
        &format!("map = \"{}\"", map_path.display()),
    );
    let one = format!("{}{}", on_the_new_map, enemy(900, 400, 1500, 500));
    let two = format!("{}{}", on_the_new_map, enemy(10, 5, 4000, 900));
    let alone = play(&on_the_new_map);
    let with_one = play(&one);
    let with_two = play(&two);
    assert_ne!(
        alone.at(0).bases.len(),
        with_one.at(0).bases.len(),
        "the worlds differ"
    );
    for day in 1..=18 {
        assert_eq!(alone.handed[&day], with_one.handed[&day], "day {day}");
        assert_eq!(alone.handed[&day], with_two.handed[&day], "day {day}");
        assert_eq!(alone.readings[&day], with_one.readings[&day]);
    }
    assert_eq!(alone.state, with_one.state);
    std::fs::remove_dir_all(&dir).ok();
}
