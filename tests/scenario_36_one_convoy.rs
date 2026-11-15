//! Scenario 36: one supply intent, one pool, no enemy. The staff keeps a
//! base at 30 days of supply: it forms a convoy, loads a stated weight,
//! sails, unloads, comes home, and sends the next run when the projection
//! calls for it. With no orders the base runs out.
//!
//! Proves the doctrine of decision 0107 (step 3, stage 1; decisions 37 to
//! 44, with the rulings the calculation needed, recorded there) against a
//! hand calculation. The data is placeholder data: see
//! `data/scenarios/36_one_convoy.toml`. It runs on the frozen step 1
//! figures. There is no enemy, no aircraft and no random draw.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! Corrected by the architect on Nov 15, 2026, after the build: a force
//! that acts at T steams on at the start of the next pulse (decision 0096),
//! not the hour its unloading ends. The rows of days 3, 4 and 18 and the
//! end of day 18 say so; nothing else moved.
//!
//! Restated by the architect on Nov 15, 2026, for decision 0112: a ship in
//! port takes fuel in the pulses of the day and no longer at its start. On
//! these figures no fuelling rate is given, so a ship that arrives in a
//! pulse is filled in that pulse. Each refuelling below is a day earlier
//! than first written; no other number moved.
//!
//! ## Figures
//!
//! `daily_supply_t_per_man` 0.005. A size 1 port has one berth, at 50 t an
//! hour; a size 0 port works by lighter at 20 t an hour. The placeholder
//! cargo ship: 3000 t of cargo, 10 kn at cruise, 0.1 t of fuel a mile, 1000
//! t of fuel. A day is 24 hours: a night pulse of 12 and two daylight
//! pulses of 6. Day d begins at hour 24 x (d - 1). In a pulse ships move
//! before cargo is worked. The men's supply is paid at the start of the
//! night pulse. A ship in port takes the fuel it lacks in the pulse. A force that
//! acts at a place and steams on does so at the start of the first pulse
//! after its work is done (decision 0096).
//!
//! ## Inputs
//!
//! A placeholder map: S at (0,0), a coast hex with port size 1, and T at
//! (5,0), a coast hex with port size 0. S to T is 5 hexes on one axis: 200
//! nm, 20 hours at cruise. The game is 18 days.
//!
//! | Base | Side   | Hex | Supply | Fuel   |
//! | ---- | ------ | --- | ------ | ------ |
//! | 1    | Allied | S   | 5000 t | 2000 t |
//! | 2    | Allied | T   | 100 t  | 0      |
//!
//! Land unit 1, Allied, 2000 men, at T on defend. It costs 10 t a day.
//!
//! Ships 1 and 2, placeholder cargo ships, lie in port at base 1 in no
//! task force, full of fuel.
//!
//! The Allied side's commands, in force from day 1:
//!
//! - An intent: keep base 2 at a floor of 30 days of supply, with an
//!   interval between shipments of 15 days. At 10 t a day the floor is a
//!   stock of 300 t.
//! - The forces limit of that intent: ships 1 and 2, with base 1 as the
//!   source.
//! - A reserve at base 1 of 1000 t of supply.
//! - Leave to prepare and leave to launch both given.
//!
//! There is no other order and no Japanese unit.
//!
//! ## The rules the staff works by
//!
//! - **When a run is needed.** A run ordered on day d is loaded that day,
//!   sails on day d + 1 and arrives that day. It is needed on day d if,
//!   without it, the figure read on the day after it would arrive, day d +
//!   2, would be below the intent's floor.
//! - **Its ships** are the pool's free ships in number order until their
//!   room covers the load. A ship is free on the morning after it is back
//!   at the source and has refuelled there.
//! - **Its load** is the floor's stock, less the stock projected for the
//!   reading on the day it arrives and not below 0, plus the planned use of
//!   one interval between shipments. The intent's interval is 15 days: 150
//!   t.
//! - **One run of the plan is loading at a time.**
//!
//! ## Day by day
//!
//! Readings are at the start of the day, before its orders. The game is 18
//! days.
//!
//! | Day | Read at T: stock, days | The staff's orders                    | What happens |
//! | --- | ---------------------- | ------------------------------------- | ------------ |
//! | 1   | 100 t, 10              | Form a force of ship 1. Load 360 t of supply at base 1 | Loaded in 360 / 50 = 7.2 hours. Base 1 holds 4640 t. T pays 10 t |
//! | 2   | 90 t, 9                | Route to T and back, acting at T, with an unload order | Sails at hour 24, arrives at hour 44. T pays 10 t. 4 hours of the afternoon at 20 t an hour: 80 t landed |
//! | 3   | 160 t, 16              | none                                  | T pays 10 t. The last 280 t take 14 hours: the 12 of the night and 2 of the morning. Unloaded at hour 62. The force steams for S at hour 66, the start of the next pulse |
//! | 4   | 430 t, 43              | none                                  | T pays 10 t. The force arrives at S at hour 86. Ship 1 refuels 40 t in that pulse: base 1 holds 1960 t of fuel |
//! | 5   | 420 t, 42              | none                                  | T pays 10 t. Ship 1 is free |
//! | 6 to 15 | 410 t falling to 320 t, 41 to 32 | none                    | T pays 10 t a day |
//! | 16  | 310 t, 31              | Load 150 t of supply at base 1        | Loaded in 3 hours. Base 1 holds 4490 t of supply. T pays 10 t |
//! | 17  | 300 t, 30              | Route to T and back, acting at T, with an unload order | Sails at hour 384, arrives at hour 404. T pays 10 t. 80 t landed |
//! | 18  | 370 t, 37              | none                                  | T pays 10 t. The last 70 t take 3.5 hours of the night: unloaded at hour 411.5. The force steams for S at hour 420, the start of the next pulse, and is at sea at the end of the day |
//!
//! Why each run, and its size:
//!
//! - **Day 1.** A run ordered today arrives on day 2. Without it the
//!   reading of day 3 is 80 t, below 300: a run is needed. The stock
//!   projected for the reading of day 2 is 90 t. The load is 300 - 90 +
//!   150 = 360 t. Ship 1 is the first free ship and has the room.
//! - **Days 2 to 15.** With the run ashore the reading falls by 10 t a day
//!   from 430 t on day 4: 310 t on day 16, 300 t on day 17 and 290 t on day
//!   18. On day 15 a run ordered then would arrive on day 16, and the
//!   reading of day 17 is 300 t, not below: no run. Ship 2 is free
//!   throughout and is not sent.
//! - **Day 16.** A run ordered today arrives on day 17, and without it the
//!   reading of day 18 is 290 t, below 300: a run is needed. The stock
//!   projected for the reading of day 17 is 300 t. The load is 0 + 150 =
//!   150 t. Ship 1 has been back and refuelled since day 4, and is the
//!   first free ship in number order.
//!
//! At the end of day 18: base 2 holds 430 t; base 1 holds 4490 t of supply
//! and 1960 t of fuel; ship 1 is at sea on its way back, empty, 12
//! hours out of T, with 968 t of fuel; it would arrive at hour 440, after
//! the game; ship 2 has not moved. The plan has made two
//! runs. No staff order was refused.
//!
//! Each order the staff gives, and each day it gives none, has a reason
//! with the numbers of this table in its "worked" part. On the days it
//! gives none, the reason says which reading the next run waits for.
//!
//! ## With no orders
//!
//! The same scenario with no command in force and no order, over 12 days.
//! Base 2 pays 10 t on each of the nights of days 1 to 10 and holds 0 t
//! after the night of day 10. On the night of day 11 its 2000 men are
//! unfed.
//!
//! ## What it shows
//!
//! From day 4, when the first run is ashore, the reading at T is never
//! below 30 days, and it never passes 45: the floor and one interval. A
//! run sails every 15 days and carries 15 days of use. The interval's
//! first value rests on one row, fuel oil shipped every 15 days
//! (`land_unit_facts` 1099), and on the doctrine of the period that a
//! shipment brings the use of one interval between shipments (1057, 1065).
//! It is a starting value, to be looked at again in stage 2, when a convoy
//! can be lost.
//!
//! # Properties
//!
//! - The staff never loads the source below its reserve: with base 1's
//!   supply set to 1100 t at the start, the first run loads 100 t and the
//!   reason names the reserve as what binds.
//! - With ship 1 out of the pool, the same runs are made by ship 2; and
//!   from step 3 of the plan, with ship 1 held. (The architect's edit, Nov
//!   15, 2026: holds are built in step 3.)
//! - The same start gives the same orders on every run.
//! - With the intent withdrawn on day 5 the staff gives no order from then,
//!   and a force at sea completes its run and comes home.

// ------------------------------------------------------------------ The tests
//
// The staff is asked as the engine asks it: each day through
// `next_day_watched(as_staffs(..))`, at the step after the commander's
// commands and the morning's refuelling. A watcher reads the figure at T at
// that moment (`Moment::Orders`), and asks the staff twice on the same view.
// What the staff handed in each day is read from `staff_handed_in`.

mod common;

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::process::Command;

use sim_core::choices::Moment;
use sim_core::command::{
    CommandKind, PlanId, PlanState, Reason, RunDays, RunSpec, StaffDecisions, StaffOrder,
};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, ShipId, TaskForceId};
use sim_core::orders::{BaseOrder, Cargo, OrderKind, TaskForceOrder};
use sim_core::side_events::SideEventKind;
use sim_core::state::{ShipLocation, Side, TaskForceStatus, TruthState};
use sim_core::{DaysOfSupply, Staff};
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs, SupplyStaff};

const S: Hex = Hex::new(0, 0);
const T: Hex = Hex::new(5, 0);

/// The number the engine gives the force formed on day 1: the first of the
/// Allied made series (decision 0086).
const FORCE: u32 = 1_000_001;

const COMMANDS_MARKER: &str = "# The Allied commander's commands";

fn path() -> PathBuf {
    common::data_dir().join("scenarios/36_one_convoy.toml")
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

fn ship(raw: u32) -> ShipId {
    ShipId::new(raw)
}

fn base(raw: u32) -> BaseId {
    BaseId::new(raw)
}

fn force(raw: u32) -> TaskForceId {
    TaskForceId::new(raw)
}

fn intent(raw: u32) -> IntentId {
    IntentId::new(raw)
}

fn plan(raw: u32) -> PlanId {
    PlanId::new(raw)
}

fn ships(raw: &[u32]) -> BTreeSet<ShipId> {
    raw.iter().map(|r| ship(*r)).collect()
}

/// The scenario's text before the commands.
fn head() -> String {
    let text = text();
    assert_eq!(text.matches(COMMANDS_MARKER).count(), 1);
    text.split(COMMANDS_MARKER)
        .next()
        .expect("a head")
        .to_string()
}

/// Text with one change, asserting that the original appears exactly once.
fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

fn days(text: &str, count: u32) -> String {
    changed(text, "days = 18", &format!("days = {count}"))
}

/// A command table as the scenario states it.
fn command_table(day: u32, body: &str) -> String {
    format!("\n[[command]]\nday = {day}\nside = \"allied\"\n{body}\n")
}

/// A game played to its end with the doctrine's staffs passed in: what the
/// staff handed in each day, the figure at T it was asked on, the truth at
/// the end of each day, the log and the hash of each day.
struct Played {
    scenario: Scenario,
    /// The figure at T at the staff's moment of each day.
    readings: BTreeMap<u32, Option<DaysOfSupply>>,
    handed: BTreeMap<u32, StaffDecisions>,
    /// Whether asking the staff twice on the view gave one answer, and the
    /// answer handed in was that answer.
    asked_twice_alike: bool,
    truth: Vec<TruthState>,
    hashes: Vec<u64>,
    events: Vec<Event>,
}

fn play(text: &str) -> Played {
    play_scenario(common::from_text(text, &path()).expect("the scenario loads"))
}

fn play_scenario(mut scenario: Scenario) -> Played {
    let held = staffs();
    let mut readings = BTreeMap::new();
    let mut handed = BTreeMap::new();
    let mut truth = vec![scenario.world.umpire().clone()];
    let mut hashes = Vec::new();
    let mut asked_twice_alike = true;
    loop {
        let mut asked: Option<StaffDecisions> = None;
        let mut reading = None;
        let mut again = true;
        let day = scenario.world.day() + 1;
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |world, moment| {
                if moment == (Moment::Orders { day }) {
                    let view = world.side_view(Side::Allied);
                    reading = Some(view.days_of_supply(T));
                    let first = SupplyStaff.decide(view);
                    again = first == SupplyStaff.decide(view);
                    asked = Some(first);
                }
            })
            .expect("a day resolves");
        let Some(summary) = summary else { break };
        asked_twice_alike &= again;
        readings.insert(day, reading.expect("the staff's moment came"));
        let given = scenario
            .world
            .staff_handed_in()
            .get(&Side::Allied)
            .cloned()
            .unwrap_or_default();
        asked_twice_alike &= asked.as_ref() == Some(&given);
        handed.insert(day, given);
        truth.push(scenario.world.umpire().clone());
        hashes.push(summary.state_hash);
    }
    Played {
        events: scenario.world.events().iter().cloned().collect(),
        scenario,
        readings,
        handed,
        asked_twice_alike,
        truth,
        hashes,
    }
}

impl Played {
    fn at(&self, day: u32) -> &TruthState {
        &self.truth[day as usize]
    }

    fn base_supply(&self, day: u32, raw: u32) -> Fixed {
        self.at(day).bases[&base(raw)].supply_t
    }

    fn base_fuel(&self, day: u32, raw: u32) -> Fixed {
        self.at(day).bases[&base(raw)].fuel_t
    }

    fn on_day(&self, day: u32, pick: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|e| e.day == day && pick(&e.kind))
            .collect()
    }

    fn all(&self, pick: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
        self.events.iter().filter(|e| pick(&e.kind)).collect()
    }

    fn decisions(&self, day: u32) -> &StaffDecisions {
        &self.handed[&day]
    }

    /// The tons unloaded on `day`.
    fn unloaded(&self, day: u32) -> Fixed {
        self.on_day(day, |k| matches!(k, EventKind::CargoUnloaded { .. }))
            .iter()
            .map(|e| match &e.kind {
                EventKind::CargoUnloaded { unloaded_t, .. } => *unloaded_t,
                _ => Fixed::ZERO,
            })
            .fold(Fixed::ZERO, |sum, t| sum + t)
    }

    /// The hour a task force arrived, from `TaskForceArrived`, on `day`.
    fn arrived_at(&self, day: u32, raw: u32, hex: Hex) -> Option<Fixed> {
        self.on_day(day, |k| {
            matches!(k, EventKind::TaskForceArrived { task_force, hex: h, .. }
                if *task_force == force(raw) && *h == hex)
        })
        .first()
        .and_then(|e| match &e.kind {
            EventKind::TaskForceArrived { elapsed_hours, .. } => Some(*elapsed_hours),
            _ => None,
        })
    }

    fn halted_at(&self, day: u32, raw: u32, hex: Hex) -> Option<Fixed> {
        self.on_day(day, |k| {
            matches!(k, EventKind::TaskForceHalted { task_force, hex: h, .. }
                if *task_force == force(raw) && *h == hex)
        })
        .first()
        .and_then(|e| match &e.kind {
            EventKind::TaskForceHalted { elapsed_hours, .. } => Some(*elapsed_hours),
            _ => None,
        })
    }

    fn loading_complete(&self, day: u32) -> Vec<Fixed> {
        self.on_day(day, |k| matches!(k, EventKind::LoadingComplete { .. }))
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::LoadingComplete { elapsed_hours, .. } => Some(*elapsed_hours),
                _ => None,
            })
            .collect()
    }

    fn unloading_complete(&self, day: u32) -> Vec<Fixed> {
        self.on_day(day, |k| matches!(k, EventKind::UnloadingComplete { .. }))
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::UnloadingComplete { elapsed_hours, .. } => Some(*elapsed_hours),
                _ => None,
            })
            .collect()
    }

    /// Every staff order of the game, by day.
    fn orders(&self) -> Vec<(u32, &StaffOrder)> {
        self.handed
            .iter()
            .flat_map(|(day, d)| d.orders.iter().map(move |o| (*day, o)))
            .collect()
    }

    /// No staff order was refused and no staff command was refused.
    fn nothing_refused(&self) {
        let refused: Vec<&Event> = self.all(|k| {
            matches!(
                k,
                EventKind::OrderRejected { .. } | EventKind::CommandRefused { .. }
            )
        });
        assert!(refused.is_empty(), "something was refused: {refused:?}");
    }
}

fn route_to(waypoints: &[Hex], act_at: &[Hex]) -> TaskForceOrder {
    TaskForceOrder::SetRoute {
        waypoints: waypoints.to_vec(),
        act_at: act_at.iter().copied().collect(),
    }
}

fn to_force(raw: u32, order: TaskForceOrder) -> OrderKind {
    OrderKind::TaskForce {
        target: force(raw),
        order,
    }
}

fn load_to(raw: u32, weight: i64) -> OrderKind {
    to_force(
        raw,
        TaskForceOrder::LoadTo {
            cargo: Cargo::Supply,
            weight_t: n(weight),
        },
    )
}

fn run_spec(raw_ships: &[u32], load: i64, days: RunDays) -> RunSpec {
    RunSpec {
        ships: ships(raw_ships),
        load_t: n(load),
        days,
    }
}

fn commit(plan_raw: u32, run: u32, raw_ships: &[u32], from: u32, to: u32) -> CommandKind {
    CommandKind::CommitShips {
        plan: plan(plan_raw),
        run: Some(run),
        ships: ships(raw_ships),
        from_day: from,
        to_day: to,
    }
}

fn mentions(reason: &Reason, words: &[&str]) {
    for word in words {
        assert!(
            reason.worked.contains(word),
            "the reason's worked part has no \"{word}\": {}",
            reason.worked
        );
    }
}

/// A reason serves intent 1 and plan 1, under the baseline.
fn serves_the_intent(reason: &Reason) {
    assert_eq!(reason.serves.intent, Some(intent(1)));
    assert_eq!(reason.serves.plan, Some(plan(1)));
    assert_eq!(reason.serves.amendment, None);
    assert!(!reason.worked.is_empty());
    assert!(!reason.evidence.is_empty() || !reason.binding.is_empty());
}

// ------------------------------------------------------- The hand calculation

#[test]
fn the_readings_at_t_follow_the_table() {
    let played = play(&text());
    let mut table: Vec<(u32, i64, i64)> = vec![
        (1, 100, 10),
        (2, 90, 9),
        (3, 160, 16),
        (4, 430, 43),
        (5, 420, 42),
    ];
    for day in 6..=15 {
        let stock = 410 - 10 * i64::from(day - 6);
        table.push((day, stock, stock / 10));
    }
    table.extend([(16, 310, 31), (17, 300, 30), (18, 370, 37)]);
    for (day, stock, figure) in table {
        let got = played.readings[&day].expect("a figure at T");
        assert_eq!(got.stock_t, n(stock), "stock at T, day {day}");
        assert_eq!(got.days, Some(n(figure)), "days at T, day {day}");
        assert_eq!(got.planned_use.total_t(), n(10), "use at T, day {day}");
    }
    assert!(played.asked_twice_alike, "the staff answers one view alike");
}

#[test]
fn the_staffs_orders_follow_the_table_day_by_day() {
    let played = play(&text());
    played.nothing_refused();

    // Day 1: ready the plan, release it, add the run, commit the ship; form
    // a force of ship 1 and load 360 t.
    let d1 = played.decisions(1);
    assert_eq!(d1.commands.len(), 4);
    assert_eq!(d1.commands[0].kind, CommandKind::Approve { plan: plan(1) });
    assert_eq!(d1.commands[1].kind, CommandKind::Approve { plan: plan(1) });
    assert_eq!(
        d1.commands[2].kind,
        CommandKind::AddRun {
            plan: plan(1),
            run: run_spec(
                &[1],
                360,
                RunDays {
                    load: 1,
                    sail: 2,
                    arrive: 2,
                    free: 5
                }
            ),
        }
    );
    assert_eq!(d1.commands[3].kind, commit(1, 1, &[1], 1, 4));
    assert_eq!(d1.orders.len(), 2);
    assert_eq!(
        d1.orders[0].kind,
        OrderKind::Base {
            target: base(1),
            order: BaseOrder::FormTaskForce {
                ships: vec![ship(1)]
            },
        }
    );
    assert_eq!(d1.orders[1].kind, load_to(FORCE, 360));
    assert!(d1.no_orders.is_empty());
    for order in &d1.orders {
        assert_eq!(order.intent, intent(1));
    }

    // Day 2: route to T and back, acting at T, with an unload order.
    let d2 = played.decisions(2);
    assert!(d2.commands.is_empty() && d2.no_orders.is_empty());
    assert_eq!(d2.orders.len(), 2);
    assert_eq!(d2.orders[0].kind, to_force(FORCE, route_to(&[T, S], &[T])));
    assert_eq!(d2.orders[1].kind, to_force(FORCE, TaskForceOrder::Unload));

    // Days 3 to 15 and 18: no order, and a reason.
    for day in (3..=15).chain([18]) {
        let d = played.decisions(day);
        assert!(d.orders.is_empty() && d.commands.is_empty(), "day {day}");
        assert_eq!(d.no_orders.len(), 1, "one no-order reason, day {day}");
    }

    // Day 16: add the run, commit the ship, load 150 t. The force of day 1
    // is at S and stopped: no forming.
    let d16 = played.decisions(16);
    assert_eq!(d16.commands.len(), 2);
    assert_eq!(
        d16.commands[0].kind,
        CommandKind::AddRun {
            plan: plan(1),
            run: run_spec(
                &[1],
                150,
                RunDays {
                    load: 16,
                    sail: 17,
                    arrive: 17,
                    free: 20
                }
            ),
        }
    );
    assert_eq!(d16.commands[1].kind, commit(1, 2, &[1], 16, 19));
    assert_eq!(d16.orders.len(), 1);
    assert_eq!(d16.orders[0].kind, load_to(FORCE, 150));
    assert!(d16.no_orders.is_empty());

    // Day 17: route and unload.
    let d17 = played.decisions(17);
    assert!(d17.commands.is_empty() && d17.no_orders.is_empty());
    assert_eq!(d17.orders.len(), 2);
    assert_eq!(d17.orders[0].kind, to_force(FORCE, route_to(&[T, S], &[T])));
    assert_eq!(d17.orders[1].kind, to_force(FORCE, TaskForceOrder::Unload));

    // Ship 2 is free throughout and is never sent: it is in no order.
    for (_, order) in played.orders() {
        if let OrderKind::Base {
            order: BaseOrder::FormTaskForce { ships },
            ..
        } = &order.kind
        {
            assert_eq!(ships, &vec![ship(1)]);
        }
    }
    // Seven staff orders, every one accepted.
    let accepted = played.all(|k| matches!(k, EventKind::OrderAccepted { .. }));
    assert_eq!(accepted.len(), 7);
}

#[test]
fn what_happens_follows_the_table_exactly() {
    let played = play(&text());
    played.nothing_refused();

    // Day 1: loaded in 360 / 50 = 7.2 hours. Base 1 holds 4640 t. T pays 10 t.
    assert_eq!(played.loading_complete(1), vec![fx("7.2")]);
    let loaded = played.on_day(1, |k| matches!(k, EventKind::CargoLoaded { .. }));
    assert_eq!(loaded.len(), 1);
    match &loaded[0].kind {
        EventKind::CargoLoaded {
            ship: s,
            task_force,
            base: b,
            loaded_t,
            aboard_t,
            base_stock_t,
            ..
        } => {
            assert_eq!((*s, *task_force, *b), (ship(1), force(FORCE), base(1)));
            assert_eq!(
                (*loaded_t, *aboard_t, *base_stock_t),
                (n(360), n(360), n(4640))
            );
        }
        _ => unreachable!(),
    }
    assert_eq!(played.base_supply(1, 1), n(4640));
    assert_eq!(played.base_supply(1, 2), n(90));

    // Day 2: sails at hour 24, arrives at hour 44; 4 hours at 20 t an hour:
    // 80 t landed. T pays 10 t.
    assert_eq!(played.halted_at(2, FORCE, T), Some(n(44)));
    assert_eq!(played.unloaded(2), n(80));
    assert_eq!(played.base_supply(2, 2), n(160));

    // Day 3: the last 280 t take 14 hours: unloaded at hour 62. T pays 10 t.
    assert_eq!(played.unloaded(3), n(280));
    assert_eq!(played.unloading_complete(3), vec![n(62)]);
    assert_eq!(played.base_supply(3, 2), n(430));

    // Day 4: the force arrives at S at hour 86 (see
    // `the_hours_at_which_the_force_comes_home`).
    assert!(played.arrived_at(4, FORCE, S).is_some());
    assert_eq!(played.base_supply(4, 2), n(420));
    // Day 5: ship 1 refuels 40 t: base 1 holds 1960 t of fuel.
    assert_eq!(played.base_fuel(4, 1), n(2000));
    assert_eq!(played.base_fuel(5, 1), n(1960));
    assert_eq!(played.at(4).ships[&ship(1)].fuel_t, n(960));
    assert_eq!(played.at(5).ships[&ship(1)].fuel_t, n(1000));
    // Days 6 to 15: T pays 10 t a day.
    for day in 4..=15 {
        assert_eq!(
            played.base_supply(day, 2),
            n(420 - 10 * i64::from(day - 4)),
            "day {day}"
        );
    }

    // Day 16: loaded in 3 hours. Base 1 holds 4490 t of supply.
    assert_eq!(played.loading_complete(16), vec![n(15 * 24 + 3)]);
    assert_eq!(played.base_supply(16, 1), n(4490));
    // Day 17: sails at hour 384, arrives at hour 404; 80 t landed.
    assert_eq!(played.halted_at(17, FORCE, T), Some(n(404)));
    assert_eq!(played.unloaded(17), n(80));
    // Day 18: the last 70 t take 3.5 hours of the night: unloaded at hour
    // 411.5; the force steams for S at hour 420 (see the test of the hours
    // below).
    assert_eq!(played.unloaded(18), n(70));
    assert_eq!(played.unloading_complete(18), vec![fx("411.5")]);

    // The end of day 18.
    assert_eq!(played.base_supply(18, 2), n(430));
    assert_eq!(played.base_supply(18, 1), n(4490));
    assert_eq!(played.base_fuel(18, 1), n(1960));
    let end = played.at(18);
    let two = &end.ships[&ship(2)];
    assert_eq!(two.location, ShipLocation::Port(base(1)));
    assert_eq!(two.fuel_t, n(1000));
    for day in 0..=18 {
        assert_eq!(
            played.at(day).ships[&ship(2)].location,
            ShipLocation::Port(base(1)),
            "ship 2 has not moved, day {day}"
        );
    }

    // The end state of ship 1 (at sea, empty, with 968 t of fuel) is in
    // the test of the hours below.

    // The plan has made two runs.
    let state = played.scenario.world.command_state(Side::Allied);
    let made = &state.plans[&plan(1)];
    assert_eq!(made.state, PlanState::Released);
    assert_eq!(made.runs.len(), 2);
    assert_eq!(made.runs[&1].load_t, n(360));
    assert_eq!(made.runs[&2].load_t, n(150));
    assert_eq!(made.runs[&1].source, base(1));
    assert_eq!(
        made.runs[&2].days,
        RunDays {
            load: 16,
            sail: 17,
            arrive: 17,
            free: 20
        }
    );
}

/// The hours at which the force comes home, and where ship 1 is at the end
/// of day 18, as the architect corrected the table: a force that acts at a
/// place steams on at the start of the first pulse after its work is done
/// (decision 0096). On day 3 the unloading ends at hour 62, in the morning
/// pulse; the force steams on at hour 66 and arrives at S at hour 86. On day
/// 18 the unloading ends at hour 411.5, in the night pulse; the force steams
/// on at hour 420 and is at sea at the end of the day, 12 hours out of T:
/// 120 nm from T, 80 nm from S, with 968 t of fuel.
#[test]
fn the_hours_at_which_the_force_comes_home() {
    let played = play(&text());
    assert_eq!(played.arrived_at(4, FORCE, S), Some(n(86)));
    assert_eq!(played.arrived_at(18, FORCE, S), None);
    let end = played.at(18);
    let one = &end.ships[&ship(1)];
    assert_eq!(one.location, ShipLocation::TaskForce(force(FORCE)));
    assert!(one.supply_t.is_zero() && one.cargo_fuel_t.is_zero());
    assert_eq!(one.fuel_t, n(968));
    let formed = &end.task_forces[&force(FORCE)];
    assert_eq!(formed.status, TaskForceStatus::Underway);
    assert_eq!(formed.route.waypoints.last(), Some(&S));
    // 12 hours at 10 kn from T: 120 nm from T, 80 nm from S, on the line
    // of hexes 40 nm apart: two hexes from S, and on the leg from there.
    let from_s = |hex: Hex| i64::from(hex.distance(S)) * 40;
    let left = from_s(formed.from_hex) - formed.progress_nm.trunc();
    assert_eq!(left, 80, "nm still to steam to S");
}

#[test]
fn every_decision_has_a_full_reason_with_the_numbers_of_the_table() {
    let played = play(&text());

    // Day 1.
    let d1 = played.decisions(1);
    for command in &d1.commands {
        serves_the_intent(&command.reason);
    }
    for order in &d1.orders {
        serves_the_intent(&order.reason);
    }
    // The run, and each order of it, carry the arithmetic: 300 - 90 + 150.
    mentions(&d1.commands[2].reason, &["360", "300", "90", "150"]);
    mentions(&d1.orders[0].reason, &["360", "300", "90", "150"]);
    mentions(&d1.orders[1].reason, &["360", "300", "90", "150", "7.2"]);
    mentions(&d1.commands[3].reason, &["day 4", "day 5"]);
    assert!(d1.commands[2]
        .reason
        .binding
        .iter()
        .any(|b| b.contains("floor of 30 days")));
    assert!(!d1.commands[2].reason.would_change.is_empty());
    assert!(!d1.commands[2].reason.evidence.is_empty());
    assert!(d1.commands[2].reason.evidence.iter().all(|e| e.day == 1));

    // Day 2.
    let d2 = played.decisions(2);
    for order in &d2.orders {
        serves_the_intent(&order.reason);
        mentions(&order.reason, &["hour 24", "hour 44", "80 t", "360"]);
    }

    // Day 16.
    let d16 = played.decisions(16);
    for command in &d16.commands {
        serves_the_intent(&command.reason);
        mentions(&command.reason, &["150", "300"]);
    }
    for order in &d16.orders {
        serves_the_intent(&order.reason);
        mentions(&order.reason, &["150", "300", "3 hours"]);
    }

    // Day 17.
    for order in &played.decisions(17).orders {
        serves_the_intent(&order.reason);
        mentions(&order.reason, &["hour 384", "hour 404", "80 t", "150"]);
    }

    // The days with no order: one reason that says which reading the next run
    // waits for, and the day it is expected to be ordered.
    for day in 3..=15 {
        let reason = &played.decisions(day).no_orders[0];
        serves_the_intent(reason);
        mentions(
            reason,
            &[
                "reading of day 18",
                "300 t",
                "expected to be ordered on day 16",
            ],
        );
        assert!(!reason.would_change.is_empty(), "day {day}");
    }
    let last = &played.decisions(18).no_orders[0];
    serves_the_intent(last);
    mentions(
        last,
        &[
            "The next run waits for the reading of day",
            "expected to be ordered on day",
        ],
    );
}

#[test]
fn every_decision_has_a_reason_event_that_resolves_by_its_number() {
    let played = play(&text());
    let view = played.scenario.world.side_view(Side::Allied);
    let reasons: Vec<_> = view
        .events()
        .into_iter()
        .filter(|e| matches!(&e.kind, SideEventKind::Whole(EventKind::StaffReason { .. })))
        .collect();
    let handed: usize = played
        .handed
        .values()
        .map(|d| d.orders.len() + d.commands.len() + d.no_orders.len())
        .sum();
    assert_eq!(reasons.len(), handed, "one reason event for each decision");
    for event in &reasons {
        let found = view
            .event_by_number(event.number)
            .expect("the number resolves");
        assert_eq!(&found, event);
    }
    // The enemy is given none of them.
    let japanese = played.scenario.world.side_view(Side::Japanese);
    assert!(japanese
        .events()
        .into_iter()
        .all(|e| !matches!(&e.kind, SideEventKind::Whole(EventKind::StaffReason { .. }))));
}

// ----------------------------------------------------------- With no orders

#[test]
fn with_no_orders_the_base_runs_out_and_its_men_go_unfed() {
    let source = days(&head(), 12);
    let played = play(&source);
    // The staff, passed in, has no intent and hands in nothing.
    for (day, decisions) in &played.handed {
        assert!(decisions.is_empty(), "day {day}");
    }
    // Base 2 pays 10 t on each of the nights of days 1 to 10 and holds 0 t
    // after the night of day 10.
    for day in 1..=10u32 {
        assert_eq!(
            played.base_supply(day, 2),
            n(100 - 10 * i64::from(day)),
            "day {day}"
        );
    }
    assert!(played.base_supply(10, 2).is_zero());
    // On the night of day 11 its 2000 men are unfed; every earlier night they
    // were fed.
    let fed = |day: u32| -> Vec<(u32, u32)> {
        played
            .on_day(day, |k| {
                matches!(
                    k,
                    EventKind::MenFed {
                        side: Side::Allied,
                        ..
                    }
                )
            })
            .iter()
            .map(|e| match &e.kind {
                EventKind::MenFed { fed, unfed, .. } => (*fed, *unfed),
                _ => (0, 0),
            })
            .collect()
    };
    for day in 1..=10 {
        assert_eq!(fed(day), vec![(2000, 0)], "fed on day {day}");
    }
    assert_eq!(fed(11), vec![(0, 2000)], "unfed on the night of day 11");
    assert!(played.base_supply(11, 2).is_zero());
    // No staff order: nothing was ordered but by the scenario, and it orders
    // nothing.
    assert!(played.orders().is_empty());
    assert!(played
        .all(|k| matches!(k, EventKind::OrderAccepted { .. }))
        .is_empty());
    // Both ships stay where they were.
    for day in 0..=12 {
        for raw in [1, 2] {
            assert_eq!(
                played.at(day).ships[&ship(raw)].location,
                ShipLocation::Port(base(1))
            );
        }
    }
}

// ---------------------------------------------------------------- Properties

#[test]
fn the_source_is_never_loaded_below_its_reserve_and_the_reason_names_the_reserve() {
    let source = changed(&text(), "supply_t = 5000", "supply_t = 1100");
    let played = play(&source);
    played.nothing_refused();
    // The first run loads 100 t, what the source holds above 1000 t.
    let d1 = played.decisions(1);
    let add_run = &d1.commands[2];
    match &add_run.kind {
        CommandKind::AddRun { run, .. } => assert_eq!(run.load_t, n(100)),
        other => panic!("not a run: {other:?}"),
    }
    assert_eq!(d1.orders[1].kind, load_to(FORCE, 100));
    for reason in [&add_run.reason, &d1.orders[0].reason, &d1.orders[1].reason] {
        assert!(
            reason.binding.iter().any(|b| b.contains("reserve")),
            "the reserve binds: {:?}",
            reason.binding
        );
        mentions(reason, &["100 t", "1000 t", "reserve"]);
    }
    // The base is never taken below 1000 t, on any day, and the staff goes
    // on saying that the reserve binds each day it would send a run.
    for day in 1..=18 {
        assert!(played.base_supply(day, 1) >= n(1000), "day {day}");
    }
    assert_eq!(played.base_supply(1, 1), n(1000));
    let blocked = (2..=18).any(|day| {
        played.decisions(day).no_orders.iter().any(|r| {
            r.binding
                .iter()
                .any(|b| b.contains("the source's stock above its reserve"))
        })
    });
    assert!(blocked, "a later day says the reserve binds");
}

#[test]
fn with_ship_1_out_of_the_pool_the_same_runs_are_made_by_ship_2() {
    // The pool is ship 2 alone. (From step 3 of the plan the ship would be
    // held rather than left out of the limit: holds are built in step 3, the
    // architect's edit of Nov 15, 2026. A held ship takes its force out of
    // the pool; this is the half that is built now.)
    let source = format!(
        "{}{}",
        head(),
        COMMANDS_TABLES.replace("ships = [1, 2]", "ships = [2]")
    );
    let played = play(&source);
    played.nothing_refused();
    let d1 = played.decisions(1);
    assert_eq!(
        d1.commands[2].kind,
        CommandKind::AddRun {
            plan: plan(1),
            run: run_spec(
                &[2],
                360,
                RunDays {
                    load: 1,
                    sail: 2,
                    arrive: 2,
                    free: 5
                }
            ),
        }
    );
    assert_eq!(
        d1.orders[0].kind,
        OrderKind::Base {
            target: base(1),
            order: BaseOrder::FormTaskForce {
                ships: vec![ship(2)]
            },
        }
    );
    assert_eq!(d1.orders[1].kind, load_to(FORCE, 360));
    assert_eq!(
        played.decisions(16).commands[0].kind,
        CommandKind::AddRun {
            plan: plan(1),
            run: run_spec(
                &[2],
                150,
                RunDays {
                    load: 16,
                    sail: 17,
                    arrive: 17,
                    free: 20
                }
            ),
        }
    );
    // The same readings at T, and ship 1 never moved.
    let reference = play(&text());
    for day in 1..=18 {
        assert_eq!(played.readings[&day], reference.readings[&day], "day {day}");
        assert_eq!(
            played.at(day).ships[&ship(1)].location,
            ShipLocation::Port(base(1))
        );
    }
}

/// The scenario's commands, as the file states them.
const COMMANDS_TABLES: &str = r#"
[[command]]
day = 1
side = "allied"
kind = "set_intent"
intent = 1
base = 2
floor_days = 30
interval_days = 15

[[command]]
day = 1
side = "allied"
kind = "set_limit"
limit = "forces"
intent = 1
ships = [1, 2]
task_forces = []
source = 1

[[command]]
day = 1
side = "allied"
kind = "set_limit"
limit = "reserve"
base = 1
supply_t = 1000
fuel_t = 0

[[command]]
day = 1
side = "allied"
kind = "set_leave"
leave = "prepare"
given = true

[[command]]
day = 1
side = "allied"
kind = "set_leave"
leave = "launch"
given = true
"#;

#[test]
fn the_commands_of_the_file_are_the_ones_these_tests_vary() {
    let file = text();
    let rest = file.split(COMMANDS_MARKER).nth(1).expect("the commands");
    let tables: String = rest.lines().skip(1).collect::<Vec<_>>().join("\n");
    assert_eq!(tables.trim(), COMMANDS_TABLES.trim());
}

#[test]
fn the_same_start_gives_the_same_orders_on_every_run() {
    let first = play(&text());
    let second = play(&text());
    assert_eq!(first.handed, second.handed);
    assert_eq!(first.hashes, second.hashes);
    assert_eq!(first.readings, second.readings);
}

#[test]
fn with_the_intent_withdrawn_on_day_5_the_staff_gives_nothing_from_then() {
    let source = format!(
        "{}{}",
        text(),
        command_table(5, "kind = \"withdraw_intent\"\nintent = 1")
    );
    let played = play(&source);
    played.nothing_refused();
    // Days 1 to 4 are as they were; from day 5 no order, no command, no reason.
    let reference = play(&text());
    for day in 1..=4 {
        assert_eq!(played.decisions(day), reference.decisions(day), "day {day}");
    }
    for day in 5..=18 {
        assert!(played.decisions(day).is_empty(), "day {day}");
    }
    // The run had come home on day 4 and nothing else sails.
    for day in 5..=18 {
        assert_eq!(played.base_supply(day, 1), n(4640), "day {day}");
    }
    let state = played.scenario.world.command_state(Side::Allied);
    assert_eq!(state.plans[&plan(1)].state, PlanState::Complete);
}

#[test]
fn with_the_intent_withdrawn_while_the_force_is_at_sea_it_completes_its_run_and_comes_home() {
    let source = format!(
        "{}{}",
        text(),
        command_table(3, "kind = \"withdraw_intent\"\nintent = 1")
    );
    let played = play(&source);
    played.nothing_refused();
    // Day 3: the plan ends and the staff frees the ship it committed, with a
    // reason; it gives no order and no reason of an order withheld.
    let d3 = played.decisions(3);
    assert!(d3.orders.is_empty() && d3.no_orders.is_empty());
    assert_eq!(d3.commands.len(), 1);
    assert_eq!(
        d3.commands[0].kind,
        CommandKind::FreeShips {
            plan: plan(1),
            ships: ships(&[1]),
        }
    );
    assert_eq!(d3.commands[0].reason.serves.intent, Some(intent(1)));
    assert!(!d3.commands[0].reason.worked.is_empty());
    for day in 4..=18 {
        assert!(played.decisions(day).is_empty(), "day {day}");
    }
    // The force completes its run: unloads at T and comes home to S on day 4.
    assert_eq!(played.unloaded(3), n(280));
    assert_eq!(played.base_supply(3, 2), n(430));
    assert!(played.arrived_at(4, FORCE, S).is_some());
    assert_eq!(
        played.at(4).task_forces[&force(FORCE)].status,
        TaskForceStatus::Arrived
    );
    // The commitments are gone.
    let state = played.scenario.world.command_state(Side::Allied);
    assert!(state.commitments.is_empty());
}

#[test]
fn a_draft_with_an_empty_pool_is_given_no_order_and_a_reason() {
    let source = format!(
        "{}{}",
        head(),
        COMMANDS_TABLES.replace("ships = [1, 2]", "ships = []")
    );
    let played = play(&source);
    played.nothing_refused();
    for day in 1..=18 {
        let d = played.decisions(day);
        assert!(d.orders.is_empty() && d.commands.is_empty(), "day {day}");
        assert_eq!(d.no_orders.len(), 1, "day {day}");
        assert!(d.no_orders[0]
            .binding
            .iter()
            .any(|b| b.contains("no ship afloat")));
    }
    // It was never approved, so it is not marked failed.
    let state = played.scenario.world.command_state(Side::Allied);
    assert_eq!(state.plans[&plan(1)].state, PlanState::Draft);
}

#[test]
fn an_approved_plan_whose_pool_has_no_ship_afloat_is_marked_failed_and_never_complete() {
    // On day 3 the commander changes the forces limit to no ship at all
    // while the force is at sea.
    let change = command_table(
        3,
        "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\nships = []\ntask_forces = []\nsource = 1",
    );
    let source = format!("{}{}", text(), change);
    let played = play(&source);
    played.nothing_refused();
    let d3 = played.decisions(3);
    assert_eq!(d3.commands.len(), 1);
    assert_eq!(
        d3.commands[0].kind,
        CommandKind::MarkPlan {
            plan: plan(1),
            outcome: sim_core::command::PlanOutcome::Failed,
        }
    );
    // The limit was changed after the plan was approved: an amendment.
    assert_eq!(d3.commands[0].reason.serves.amendment, Some(1));
    assert!(d3.orders.is_empty());
    assert_eq!(d3.no_orders.len(), 1);
    let state = played.scenario.world.command_state(Side::Allied);
    assert_eq!(state.plans[&plan(1)].state, PlanState::Failed);
    // From day 4 the ended plan's ships are freed, once, with a reason, and
    // the staff gives the intent one reason a day.
    let d4 = played.decisions(4);
    assert_eq!(d4.commands.len(), 1);
    assert!(matches!(d4.commands[0].kind, CommandKind::FreeShips { .. }));
    for day in 5..=18 {
        let d = played.decisions(day);
        assert!(d.orders.is_empty() && d.commands.is_empty(), "day {day}");
        assert_eq!(d.no_orders.len(), 1, "day {day}");
    }
    // The force at sea completes its run.
    assert_eq!(played.unloaded(3), n(280));
}

#[test]
fn the_staff_never_orders_a_unit_outside_the_forces_limit() {
    // Ship 3 lies in port at S too but the pool is ship 2 alone.
    let extra = r#"
[[ship]]
id = 3
name = "Placeholder cargo ship 3"
class = 3
side = "allied"
fuel_t = 1000
port = 1
"#;
    let source = format!(
        "{}{}{}",
        head(),
        extra,
        COMMANDS_TABLES.replace("ships = [1, 2]", "ships = [2]")
    );
    let played = play(&source);
    played.nothing_refused();
    assert!(!played.orders().is_empty());
    for (day, order) in played.orders() {
        match &order.kind {
            OrderKind::Base {
                target,
                order: BaseOrder::FormTaskForce { ships },
            } => {
                assert_eq!(*target, base(1), "day {day}");
                assert_eq!(ships, &vec![ship(2)], "day {day}");
            }
            OrderKind::TaskForce { target, .. } => {
                // The formed force holds ship 2 and nothing else.
                let held = &played.at(day).task_forces.get(target);
                if let Some(force) = held {
                    assert_eq!(force.ships, ships(&[2]), "day {day}");
                }
                assert_eq!(target.raw(), FORCE);
            }
            other => panic!("an order the staff may not give: {other:?}"),
        }
    }
    for day in 0..=18 {
        assert_eq!(
            played.at(day).ships[&ship(3)].location,
            ShipLocation::Port(base(1)),
            "ship 3 never moved, day {day}"
        );
    }
    // The engine itself refuses a staff order outside the limit: a hand-in
    // naming ship 3 is refused. (Checked on the engine in sim-core's tests;
    // here, the staff gave none.)
    assert!(played
        .all(|k| matches!(k, EventKind::OrderRejected { .. }))
        .is_empty());
}

#[test]
fn the_staff_never_plans_outside_a_set_area() {
    // A detour: S, (0,1), (1,1), (2,1), (3,1), (4,0), T. The straight path
    // runs along r = 0.
    let area = "hexes = [[0, 0], [0, 1], [1, 1], [2, 1], [3, 1], [4, 0], [5, 0]]";
    let command = command_table(
        1,
        &format!("kind = \"set_limit\"\nlimit = \"area\"\nintent = 1\n{area}"),
    );
    let source = format!("{}{}", text(), command);
    let played = play(&source);
    played.nothing_refused();
    let allowed: BTreeSet<Hex> = [(0, 0), (0, 1), (1, 1), (2, 1), (3, 1), (4, 0), (5, 0)]
        .into_iter()
        .map(|(q, r)| Hex::new(q, r))
        .collect();
    // Every route the staff gives has waypoints in the area, and every hex
    // the force enters is.
    let mut routes = 0;
    for (_, order) in played.orders() {
        if let OrderKind::TaskForce {
            order: TaskForceOrder::SetRoute { waypoints, .. },
            ..
        } = &order.kind
        {
            routes += 1;
            for waypoint in waypoints {
                assert!(
                    allowed.contains(waypoint),
                    "{waypoint:?} is outside the area"
                );
            }
        }
    }
    assert!(routes >= 1, "a run was sent");
    let mut entered = 0;
    for event in played.all(|k| matches!(k, EventKind::TaskForceSteamed { .. })) {
        if let EventKind::TaskForceSteamed { hexes_entered, .. } = &event.kind {
            for hex in hexes_entered {
                entered += 1;
                assert!(allowed.contains(hex), "the force entered {hex:?}");
            }
        }
    }
    assert!(entered > 0);
    // The convoy did deliver.
    assert!(played.base_supply(18, 2) > n(300));
}

#[test]
fn a_set_area_with_no_route_in_it_gives_no_route_in_the_area_as_what_binds() {
    // S and T are in the area and nothing between them.
    let command = command_table(
        1,
        "kind = \"set_limit\"\nlimit = \"area\"\nintent = 1\nhexes = [[0, 0], [5, 0]]",
    );
    let source = format!("{}{}", text(), command);
    let played = play(&source);
    played.nothing_refused();
    assert!(
        played.orders().is_empty(),
        "no order is given with no route"
    );
    for day in 1..=18 {
        let d = played.decisions(day);
        assert!(
            !d.commands
                .iter()
                .any(|c| matches!(c.kind, CommandKind::AddRun { .. })),
            "day {day}"
        );
        assert_eq!(d.no_orders.len(), 1, "day {day}");
        let reason = &d.no_orders[0];
        assert!(
            reason.binding.iter().any(|b| b == "no route in the area"),
            "day {day}: {:?}",
            reason.binding
        );
        mentions(reason, &["no route in the area"]);
    }
    for day in 0..=18 {
        for raw in [1, 2] {
            assert_eq!(
                played.at(day).ships[&ship(raw)].location,
                ShipLocation::Port(base(1))
            );
        }
    }
    // The proposal says it too.
    let mut scenario = common::from_text(&source, &path()).expect("loads");
    scenario
        .next_day_staffed(as_staffs(&staffs()))
        .expect("day 1");
    let view = scenario.world.side_view(Side::Allied);
    let proposal = SupplyStaff.propose(view, intent(1)).expect("a proposal");
    let shortfall = proposal.shortfall.expect("a shortfall");
    assert_eq!(shortfall.binds, "no route in the area");
    assert!(proposal.runs.is_empty());
}

#[test]
fn a_staff_with_no_intent_hands_in_nothing_for_either_side() {
    let source = head();
    let mut scenario = common::from_text(&source, &path()).expect("loads");
    let held = staffs();
    while scenario
        .next_day_staffed(as_staffs(&held))
        .expect("a day resolves")
        .is_some()
    {
        for side in Side::ALL {
            assert!(scenario
                .world
                .staff_handed_in()
                .get(&side)
                .is_none_or(StaffDecisions::is_empty));
        }
    }
    assert!(scenario.world.command_state(Side::Allied).is_empty());
}

#[test]
fn deciding_twice_on_one_view_gives_one_answer() {
    // `play` asks twice at the staff's moment of every day and compares with
    // what was handed in.
    assert!(play(&text()).asked_twice_alike);
}

#[test]
fn a_reason_still_resolves_after_its_intent_is_replaced_or_withdrawn() {
    // Day 6: intent 1 is withdrawn and intent 2 keeps base 2 at 20 days with
    // the same pool; both are in the log.
    let more = format!(
        "{}{}{}{}",
        command_table(6, "kind = \"withdraw_intent\"\nintent = 1"),
        command_table(6, "kind = \"set_intent\"\nintent = 2\nbase = 2\nfloor_days = 20\ninterval_days = 10"),
        command_table(
            6,
            "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 2\nships = [1, 2]\ntask_forces = []\nsource = 1"
        ),
        command_table(7, "kind = \"set_intent\"\nintent = 2\nbase = 2\nfloor_days = 25\ninterval_days = 10"),
    );
    let source = format!("{}{}", text(), more);
    let mut scenario = common::from_text(&source, &path()).expect("loads");
    let held = staffs();
    let mut numbers: Vec<(u32, String)> = Vec::new();
    while scenario
        .next_day_staffed(as_staffs(&held))
        .expect("a day resolves")
        .is_some()
    {
        let view = scenario.world.side_view(Side::Allied);
        if scenario.world.day() == 4 {
            for event in view.events() {
                if let SideEventKind::Whole(EventKind::StaffReason { reason, .. }) = &event.kind {
                    numbers.push((event.number, reason.worked.clone()));
                }
            }
        }
    }
    assert!(numbers.len() > 5);
    let view = scenario.world.side_view(Side::Allied);
    for (number, worked) in numbers {
        let event = view.event_by_number(number).expect("it still resolves");
        match &event.kind {
            SideEventKind::Whole(EventKind::StaffReason { reason, .. }) => {
                assert_eq!(reason.worked, worked);
                assert_eq!(reason.serves.intent, Some(intent(1)));
            }
            other => panic!("not a reason: {other:?}"),
        }
    }
    // The state holds both intents, the first withdrawn.
    let state = scenario.world.command_state(Side::Allied);
    assert!(!state.intents[&intent(1)].in_force());
    assert!(state.intents[&intent(2)].in_force());
    // The second intent's plan has an amendment from day 7, and its reasons
    // name it.
    let amended = state
        .plans
        .values()
        .find(|p| p.intent == intent(2))
        .expect("its plan");
    assert_eq!(amended.amendments.len(), 1);
}

#[test]
fn two_worlds_that_differ_only_in_what_the_allied_side_cannot_see_give_the_same_decisions() {
    // A third coast hex, U, with a Japanese base, a ship and a unit on it.
    let map_text =
        std::fs::read_to_string(common::data_dir().join("map/placeholder_one_convoy.toml"))
            .expect("the map reads")
            + "\n[[hex]]\nq = 3\nr = 2\nkind = \"coast\"\nport_size = 1\n";
    let dir = std::env::temp_dir().join(format!("flag_plot_s36_{}", std::process::id()));
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
side = "japanese"
fuel_t = {ship_fuel}
port = 7
"#
        )
    };
    let on_the_new_map = |text: &str| {
        changed(
            text,
            "map = \"../map/placeholder_one_convoy.toml\"",
            &format!("map = \"{}\"", map_path.display()),
        )
    };
    let plain = on_the_new_map(&text());
    let one = format!("{}{}", plain, enemy(900, 400, 1500, 500));
    let two = format!("{}{}", plain, enemy(10, 5, 4000, 900));
    let alone = play(&plain);
    let with_one = play(&one);
    let with_two = play(&two);
    for day in 1..=18 {
        assert_eq!(alone.handed[&day], with_one.handed[&day], "day {day}");
        assert_eq!(alone.handed[&day], with_two.handed[&day], "day {day}");
        assert_eq!(alone.readings[&day], with_one.readings[&day]);
    }
    // The proposal is the same too.
    let mut proposals = Vec::new();
    for source in [&plain, &one, &two] {
        let mut scenario = common::from_text(source, &path()).expect("loads");
        let held = staffs();
        let mut taken = None;
        scenario
            .next_day_watched(as_staffs(&held), &mut |world, moment| {
                if moment == (Moment::Orders { day: 1 }) {
                    taken = SupplyStaff.propose(world.side_view(Side::Allied), intent(1));
                }
            })
            .expect("day 1");
        proposals.push(taken.expect("a proposal"));
    }
    assert_eq!(proposals[0], proposals[1]);
    assert_eq!(proposals[0], proposals[2]);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn the_proposal_is_what_the_staff_would_do_and_the_runs_of_thirty_days() {
    let mut scenario = common::from_text(&text(), &path()).expect("loads");
    let held = staffs();
    let mut proposal = None;
    let mut decided = None;
    scenario
        .next_day_watched(as_staffs(&held), &mut |world, moment| {
            if moment == (Moment::Orders { day: 1 }) {
                let view = world.side_view(Side::Allied);
                proposal = SupplyStaff.propose(view, intent(1));
                decided = Some(SupplyStaff.decide(view));
            }
        })
        .expect("day 1");
    let proposal = proposal.expect("a proposal");
    let decided = decided.expect("a decision");
    assert_eq!(proposal.intent, intent(1));
    assert_eq!(proposal.plan, plan(1));
    assert_eq!(proposal.plan_state, PlanState::Draft);
    // What it would hand in today as if the plan were released: the same
    // orders, and the same run, without the approvals.
    assert_eq!(proposal.decisions.orders, decided.orders);
    assert_eq!(proposal.decisions.commands, decided.commands[2..].to_vec());
    // The runs over 30 days: 360 t on day 1 and 150 t on day 16.
    assert_eq!(proposal.runs.len(), 2);
    assert_eq!(proposal.runs[0].load_t, n(360));
    assert_eq!(proposal.runs[0].ships, ships(&[1]));
    assert_eq!(
        proposal.runs[0].days,
        RunDays {
            load: 1,
            sail: 2,
            arrive: 2,
            free: 5
        }
    );
    assert_eq!(proposal.runs[1].load_t, n(150));
    assert_eq!(
        proposal.runs[1].days,
        RunDays {
            load: 16,
            sail: 17,
            arrive: 17,
            free: 20
        }
    );
    assert!(proposal.shortfall.is_none());
    let projection = proposal.projection.expect("a projection");
    assert_eq!(projection.hex, T);
    assert_eq!(projection.floor_days, 30);
    assert_eq!(projection.readings[0].stock_t, n(100));
    assert_eq!(projection.readings[0].days, Some(n(10)));
    assert_eq!(projection.first_day_below, Some(1));
    assert!(!proposal.reason.worked.is_empty());
    assert_eq!(proposal.reason.serves.intent, Some(intent(1)));
}

#[test]
fn against_no_orders_the_staff_keeps_the_figure_near_the_floor_over_thirty_days() {
    let source = days(&text(), 30);
    let played = play(&source);
    played.nothing_refused();
    // From the day the first run lands (day 3; the figure shows it on day 4)
    // the days of supply never fall below the floor less the days one round
    // trip takes: 30 - 4 = 26.
    let round_trip = 4;
    let mut lowest = n(1000);
    for day in 4..=30 {
        let figure = played.readings[&day]
            .expect("a figure")
            .days
            .expect("a use");
        assert!(figure >= n(30 - round_trip), "day {day}: {figure}");
        lowest = lowest.min(figure);
    }
    // In this case it holds the floor itself.
    assert_eq!(lowest, n(30));
    // And never passes 45: the floor and one interval.
    for day in 4..=30 {
        let figure = played.readings[&day].unwrap().days.unwrap();
        assert!(figure <= n(45), "day {day}: {figure}");
    }
    // With no orders it has run out by day 11 (see above).
    let without = play(&days(&head(), 30));
    assert!(without.base_supply(10, 2).is_zero());
}

// --------------------------------------------------- A save in the middle

fn cargo() -> Command {
    let mut command = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string()));
    command.current_dir(common::data_dir().join(".."));
    command
}

fn sim_cli(arguments: &[&str]) -> std::process::Output {
    let mut command = cargo();
    command.args(["run", "--quiet", "-p", "sim-cli"]);
    if !cfg!(debug_assertions) {
        command.arg("--release");
    }
    command.arg("--").args(arguments);
    command.output().expect("the command runs")
}

#[test]
fn a_game_saved_in_the_middle_replays_and_the_staff_checks() {
    let reference = common::frozen_root();
    let reference = reference.to_str().expect("a path");
    let dir = std::env::temp_dir().join(format!("flag_plot_s36_save_{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a directory");
    // The game saved in the middle: ten days of the scenario, with its paths
    // made whole so the copy can sit anywhere.
    let data = common::data_dir()
        .canonicalize()
        .expect("the data directory");
    let middle = days(&text(), 10).replace("\"../", &format!("\"{}/", data.display()));
    let middle_path = dir.join("36_in_the_middle.toml");
    std::fs::write(&middle_path, middle).expect("the scenario is written");
    let full = dir.join("full.json");
    let half = dir.join("half.json");
    for (scenario, save) in [(path(), &full), (middle_path.clone(), &half)] {
        let ran = sim_cli(&[
            "run",
            scenario.to_str().unwrap(),
            "--save",
            save.to_str().unwrap(),
            "--reference",
            reference,
        ]);
        assert!(
            ran.status.success(),
            "run: {}",
            String::from_utf8_lossy(&ran.stderr)
        );
    }
    for save in [&half, &full] {
        let replayed = sim_cli(&[
            "replay",
            save.to_str().unwrap(),
            "--check-staff",
            "--reference",
            reference,
        ]);
        let said = String::from_utf8_lossy(&replayed.stdout).to_string();
        assert!(
            replayed.status.success(),
            "replay: {} {}",
            said,
            String::from_utf8_lossy(&replayed.stderr)
        );
        assert!(
            said.contains("the staff gave what was saved on every day"),
            "{said}"
        );
        assert!(said.contains(sim_staff::DOCTRINE_VERSION), "{said}");
    }
    std::fs::remove_dir_all(&dir).ok();
}
