//! Scenario 41: a plan made ready that waits for leave to launch, a hold, a
//! changed intent, a week, a release (41a); an order of the commander's
//! that overrides the staff (41b); and a plan cancelled with its ship at
//! sea (41c).
//!
//! Proves step 3 of decision 0107 (step 3, stage 1: holds and release,
//! overrides, previews and amendments, the two leaves, cancelling; rulings
//! 17 to 30 there) against a hand calculation. The data is placeholder
//! data: see `data/scenarios/41a_hold_change_release.toml`,
//! `41b_an_override.toml` and `41c_cancelled_at_sea.toml`. It runs on the
//! frozen step 1 figures. There is no enemy, no aircraft and no random
//! draw.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Figures and the rules the staff works by
//!
//! Those of scenario 36: the placeholder cargo ship (3000 t of cargo, 10
//! kn at cruise, 0.1 t of fuel a mile, 1000 t of fuel), a berth at 50 t an
//! hour at S and lighters at 20 t an hour at T, 10 t a day for the 2000 men
//! at T, a day of a night pulse of 12 hours and two daylight pulses of 6,
//! and a force that steams on at the start of the first pulse after its
//! work is done. And:
//!
//! - **A plan that is preparing forms and loads its first run and does not
//!   sail it.** It sails on the first day the plan is released.
//! - **A held ship is not free,** and no ship of a force that has a held
//!   ship in it, or is itself held, is free. The staff gives no order to
//!   what is held.
//! - **A run not yet sailed whose ship is held is ended.** What is aboard
//!   stays aboard.
//! - **An order of the commander's to a force in an intent's forces limit
//!   holds that force,** before the staff is asked that morning.
//! - **A released ship is newly free.** The staff takes the free ships in
//!   number order, as it always does.
//!
//! # 41a: ready and not sailed, a hold, a changed intent, a release
//!
//! ## Inputs
//!
//! Scenario 36's map, bases, land unit and ships: S at (0,0) with base 1
//! (5000 t of supply, 2000 t of fuel), T at (5,0) with base 2 (100 t),
//! ships 1 and 2 in port at base 1 in no force. The game is 27 days.
//!
//! The Allied side's commands:
//!
//! - Day 1: intent 1, keep base 2 at a floor of 30 days, interval 15 days;
//!   its forces limit, ships 1 and 2 with base 1 as the source; a reserve
//!   at base 1 of 1000 t of supply. No leave is set: by default leave to
//!   prepare is given and leave to launch is not.
//! - Day 3: approve the plan (it is preparing: this releases it).
//! - Day 8: hold ship 1.
//! - Day 10: intent 1 changed to a floor of 40 days, interval 15 days.
//! - Day 17: release ship 1.
//!
//! ## Day by day
//!
//! | Day | Read at T | The staff | What happens |
//! | --- | --------- | --------- | ------------ |
//! | 1   | 100 t, 10 days | Makes the draft ready (preparing). Forms a force of ship 1; load 360 t | Loaded in 7.2 hours. Base 1 holds 4640 t. T pays 10 t, as on every night |
//! | 2   | 90 t      | No order: run 1 is loaded and waits for leave to launch | |
//! | 3   | 80 t      | The plan is released. Run 1 is timed again: sails day 3, due day 3, free day 6. Route to T and back, acting at T, with an unload order | Sails at hour 48, arrives at hour 68, 80 t landed |
//! | 4   | 150 t     | none      | The last 280 t take 14 hours: unloaded at hour 86. Steams for S at hour 90 |
//! | 5   | 420 t     | none      | Arrives at S at hour 110 |
//! | 6   | 410 t     | none      | Ship 1 refuels 40 t: 1960 t of fuel at base 1. Ship 1 is free |
//! | 7   | 400 t     | none      | |
//! | 8   | 390 t     | none      | Ship 1 is held |
//! | 9   | 380 t     | none      | |
//! | 10  | 370 t, 37 days | The floor is 40 days: amendment 1. Forms a force of ship 2; load 190 t | Loaded in 3.8 hours. Base 1 holds 4450 t |
//! | 11  | 360 t     | Route to T and back, acting at T, with an unload order | Sails at hour 240, arrives at hour 260, 80 t landed |
//! | 12  | 430 t     | none      | The last 110 t take 5.5 hours: unloaded at hour 269.5. Steams for S at hour 276 |
//! | 13  | 530 t     | none      | Arrives at S at hour 296 |
//! | 14  | 520 t     | none      | Ship 2 refuels 40 t: 1920 t of fuel at base 1 |
//! | 15 to 24 | 510 t falling to 420 t | none | Ship 1 is released on day 17 |
//! | 25  | 410 t     | Load 150 t on ship 1's force | Loaded in 3 hours. Base 1 holds 4300 t |
//! | 26  | 400 t     | Route to T and back, acting at T, with an unload order | Sails at hour 600, arrives at hour 620, 80 t landed |
//! | 27  | 470 t     | none      | The last 70 t take 3.5 hours: unloaded at hour 627.5. Steams for S at hour 636 and is at sea at the end of the day |
//!
//! Why, day by day:
//!
//! - **Day 1.** As scenario 36: the load is 300 - 90 + 150 = 360 t. Leave
//!   to prepare is given, so the staff makes the draft ready and loads its
//!   first run. The run's days as planned are load 1, sail 2, due 2, free 5.
//! - **Day 2.** The load is complete and the plan is not released. The
//!   staff gives no order; its reason says the run is loaded and waits for
//!   leave to launch, and that leave given, or the commander's approval,
//!   would change it.
//! - **Day 3.** The commander's approval releases the plan. The load is not
//!   sized again. The run sails a day later than planned, so the reading of
//!   day 5 is 420 t and not 430.
//! - **Day 8.** With ship 1 held the staff still needs no run: the reading
//!   of day 10 is 370 t. Its reason says the next run waits for the reading
//!   of day 18 (290 t) and is expected to be ordered on day 16.
//! - **Day 10.** The floor is now a stock of 400 t. A run ordered today
//!   arrives on day 11, and the reading of day 12 without it is 350 t,
//!   below 400: a run is needed. The stock projected for day 11 is 360 t:
//!   the load is 400 - 360 + 150 = 190 t. Ship 1 is held, so the run is
//!   ship 2's. The reason says it serves the plan's amendment 1, that ship
//!   1 is held, and that without the hold ship 1 would have been sent.
//! - **Day 17.** Ship 1 is released and is free. No run is needed: the
//!   readings from day 13 are 530 t less 10 t a day, 400 t on day 26 and
//!   390 t on day 27, so the next run is expected to be ordered on day 25.
//! - **Day 25.** A run ordered today arrives on day 26; the reading of day
//!   27 without it is 390 t: needed. The stock projected for day 26 is 400
//!   t: the load is 0 + 150 = 150 t. Ship 1 is the first free ship in
//!   number order, and lies in the force formed on day 1.
//!
//! At the end of day 27: base 2 holds 530 t (100 + 360 + 190 + 150 - 270).
//! Base 1 holds 4300 t of supply and 1920 t of fuel. Ship 2 is stopped at
//! base 1, empty, with 1000 t of fuel. Ship 1 is at sea, empty, 80 nm from
//! S, with 968 t of fuel. The plan has made three runs and has one
//! amendment. No staff order was refused.
//!
//! ## The preview
//!
//! Asked before day 10 is ended, with ship 1 held: a preview of intent 1
//! at a floor of 40 days says that as things stand the next run is 150 t,
//! expected to be ordered on day 16, and that with the change a run of 190
//! t is ordered on day 10 by ship 2; that ship 1 is held; that nothing made
//! ready is lost; and that no other plan gives anything up. It changes
//! nothing: the command state, the queue and the hash are as they were.
//! Approving the preview queues the changed intent for day 10, and the day
//! then runs as the table has it.
//!
//! If the commander queues a release of ship 1 after the preview, the
//! preview is void: approving it is refused and says why. Asked again, the
//! preview gives the run of 190 t to ship 1.
//!
//! ## Properties of 41a
//!
//! - **With both leaves given on day 1** and none of the later commands the
//!   game is scenario 36's, day for day.
//! - **With no approval on day 3** the run never sails: base 2 holds 0 t
//!   after the night of day 10 and its men are unfed on the night of day
//!   11, and on each day from day 2 the staff's reason says the run waits
//!   for leave to launch.
//! - **A run set aside.** With ship 1 held on day 2 in place of day 8, and
//!   released on day 8: on day 2 the staff ends run 1, with 360 t still
//!   aboard ship 1, and, the plan still preparing, loads ship 2 in its
//!   place: a run ordered on day 2 arrives on day 3, the stock projected
//!   for day 3 is 80 t, and the load is 300 - 80 + 150 = 370 t. It sails on
//!   day 3, when the plan is released, and the readings at T are those of
//!   41b. Run 1 is not taken up again when ship 1 is released. The next run
//!   is needed on day 17 (the reading of day 19 is 290 t) and is ship 1's:
//!   its load is 150 t by the sum, and it carries the 360 t it has aboard,
//!   with no load order; the reason says so. It sails on day 18.
//! - **A held force.** With the force formed on day 1 held on day 8 in
//!   place of ship 1, the days are the same.
//! - **A held ship in a force of two.** With ships 1 and 2 in one force
//!   from the start and ship 1 held on day 8, the staff has no free ship on
//!   day 10 and says the hold is what binds; a third ship in the pool,
//!   lying in no force, is sent.
//! - **A held setting.** A hold on the search arc of an air group is
//!   recorded, shown by `command_state`, and changes no order the supply
//!   staff gives.
//! - **The guard.** A staff that hands in an order to a held ship's force
//!   has it refused, with the hold as the reason.
//! - The staff never gives an order to what is held, on any day of any of
//!   these games.
//!
//! # 41b: an override
//!
//! ## Inputs
//!
//! Scenario 36 whole, both leaves given. The game is 5 days. On day 2 the
//! commander orders the force formed on day 1 to unload.
//!
//! ## Day by day
//!
//! - **Day 1.** As scenario 36: a force of ship 1 is formed and loads 360
//!   t. Base 1 holds 4640 t.
//! - **Day 2.** The commander's order is to a force in intent 1's forces
//!   limit: the force is held from that morning, as an override, before the
//!   staff is asked. The staff ends run 1 and frees ship 1's commitment. It
//!   needs a run: ordered today it arrives on day 3, the reading of day 4
//!   without it is 70 t, and the stock projected for day 3 is 80 t. The
//!   load is 300 - 80 + 150 = 370 t, on ship 2, in a new force. Its reason
//!   says ship 1's force is held by the commander's order.
//! - In the night of day 2 the first force has the berth and puts its 360
//!   t ashore in 7.2 hours; the second loads 240 t by lighter. In the
//!   morning the second has the berth and loads the other 130 t in 2.6
//!   hours. Base 1 holds 4630 t.
//! - **Day 3.** Ship 2's force sails at hour 48 and arrives at hour 68: 80
//!   t landed.
//! - **Day 4.** The last 290 t take 14.5 hours: unloaded at hour 86.5. It
//!   steams for S at hour 90.
//! - **Day 5.** It arrives at S at hour 110.
//!
//! Readings at T: 100, 90, 80, 150 and 430 t on days 1 to 5. At the end of
//! day 5: base 2 holds 420 t; base 1 holds 4630 t of supply and 2000 t of
//! fuel; ship 1 is at base 1, empty, with 1000 t of fuel, its force held;
//! ship 2 is at base 1, empty, with 960 t of fuel.
//!
//! ## Property of 41b
//!
//! - The override is recorded as an event that cites the commander's
//!   order, and `command_state` shows the hold and that an order made it.
//!
//! # 41c: cancelled with a ship at sea
//!
//! ## Inputs
//!
//! Scenario 36 with T at (9,0): 360 nm from S, 36 hours at cruise. Both
//! leaves given. The game is 5 days. On day 3 the commander cancels the
//! plan.
//!
//! ## Day by day
//!
//! - **Day 1.** The passage out is 2 days, so a run ordered today sails on
//!   day 2 and arrives on day 3. Without it the reading of day 4 is 70 t:
//!   needed. The stock projected for day 3 is 80 t: the load is 300 - 80 +
//!   150 = 370 t, on ship 1. Loaded in 7.4 hours. Base 1 holds 4630 t.
//! - **Day 2.** The force sails at hour 24.
//! - **Day 3.** At hour 48 it is 240 nm out, in hex (6,0). The plan is
//!   cancelled. The staff frees ship 1's commitment and gives the force a
//!   route home, to S, with a reason that names the cancelling. The force
//!   turns and is back at S at hour 72, the end of the day, with 370 t
//!   aboard and 952 t of fuel.
//! - **Day 4.** Ship 1 refuels 48 t: base 1 holds 1952 t of fuel. Its
//!   unload order stands, and it puts the 370 t ashore at S in 7.4 hours:
//!   base 1 holds 5000 t. The staff gives no order; its reason says the
//!   plan is cancelled.
//! - **Day 5.** No order.
//!
//! At the end of day 5: base 1 holds 5000 t of supply and 1952 t of fuel;
//! base 2 holds 50 t; ship 1 is stopped at base 1, empty, with 1000 t of
//! fuel; the plan is cancelled and intent 1 is still in force with no plan
//! that can serve it.
//!
//! ## Property of 41c
//!
//! - Cancelled on day 4 in place of day 3, when the force is at T and has
//!   begun to unload (it arrived at hour 60 and landed 240 t in the 12
//!   hours of daylight left), the force is given no new order: it lands the
//!   other 130 t and comes home by the route it has.

// ------------------------------------------------------------------ The tests
//
// The games are played as the engine plays them: each day through
// `next_day_watched(as_staffs(..))`, the staff asked at the step after the
// commander's commands and the morning's refuelling. A watcher reads, at that
// moment, the figure at T and what is held. What the staff handed in each day
// is read from `staff_handed_in`. 41b and 41c have their own files; the
// variants of 41a are the text of its file with other commands.

mod common;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use sim_core::choices::Moment;
use sim_core::command::{
    CommandKind, HeldSetting, HoldMaker, HoldTarget, PlanId, PlanState, Reason, RunDays,
    StaffDecisions, StaffOrder, Staffs,
};
use sim_core::events::{Cause, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, ShipId, TaskForceId};
use sim_core::orders::{BaseOrder, OrderKind, RejectReason, TaskForceOrder};
use sim_core::state::{ShipLocation, Side, TaskForceStatus, TruthState};
use sim_core::view::SideView;
use sim_core::{DaysOfSupply, Staff};
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs, SupplyStaff};

const S: Hex = Hex::new(0, 0);
const T: Hex = Hex::new(5, 0);
const FAR_T: Hex = Hex::new(9, 0);

/// The number the engine gives the force formed on day 1, and the one it
/// gives the force formed next (decision 0086).
const FORCE: u32 = 1_000_001;
const SECOND: u32 = 1_000_002;

const COMMANDS_MARKER: &str = "# The Allied commander's commands";

fn scenario_path(file: &str) -> PathBuf {
    common::data_dir().join("scenarios").join(file)
}

fn path_a() -> PathBuf {
    scenario_path("41a_hold_change_release.toml")
}

fn path_b() -> PathBuf {
    scenario_path("41b_an_override.toml")
}

fn path_c() -> PathBuf {
    scenario_path("41c_cancelled_at_sea.toml")
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).expect("the scenario file reads")
}

fn text_a() -> String {
    read(&path_a())
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

fn plan(raw: u32) -> PlanId {
    PlanId::new(raw)
}

/// The text before the commands of the scenario.
fn head_of(text: &str) -> String {
    assert_eq!(text.matches(COMMANDS_MARKER).count(), 1);
    text.split(COMMANDS_MARKER)
        .next()
        .expect("a head")
        .to_string()
}

/// A command table as the scenario states it.
fn command_table(day: u32, body: &str) -> String {
    format!("\n[[command]]\nday = {day}\nside = \"allied\"\n{body}\n")
}

/// The commands of day 1 that 41a and 36 share: the intent, the forces limit
/// and the reserve.
fn intent_and_limits(ships: &str) -> String {
    format!(
        "{}{}{}",
        command_table(
            1,
            "kind = \"set_intent\"\nintent = 1\nbase = 2\nfloor_days = 30\ninterval_days = 15"
        ),
        command_table(
            1,
            &format!(
                "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\nships = {ships}\ntask_forces = []\nsource = 1"
            )
        ),
        command_table(
            1,
            "kind = \"set_limit\"\nlimit = \"reserve\"\nbase = 1\nsupply_t = 1000\nfuel_t = 0"
        ),
    )
}

fn leave(which: &str) -> String {
    command_table(
        1,
        &format!("kind = \"set_leave\"\nleave = \"{which}\"\ngiven = true"),
    )
}

fn change_floor() -> String {
    command_table(
        10,
        "kind = \"set_intent\"\nintent = 1\nbase = 2\nfloor_days = 40\ninterval_days = 15",
    )
}

/// 41a's text with `commands` after its head.
fn a_with(days: u32, commands: &str) -> String {
    let head = head_of(&text_a()).replace("days = 27", &format!("days = {days}"));
    format!("{head}\n{COMMANDS_MARKER}\n{commands}")
}

/// What is held on a morning, at the staff's moment: the ships, the forces,
/// each with the ships in it, and the settings.
#[derive(Clone, Debug, Default)]
struct Held {
    ships: BTreeSet<ShipId>,
    forces: BTreeSet<TaskForceId>,
    settings: usize,
    /// The ships of each force of the side.
    members: BTreeMap<TaskForceId, BTreeSet<ShipId>>,
}

impl Held {
    fn read(view: SideView<'_>) -> Held {
        let mut held = Held::default();
        for target in view.command_state().holds.keys() {
            match target {
                HoldTarget::Ship(s) => {
                    held.ships.insert(*s);
                }
                HoldTarget::TaskForce(f) => {
                    held.forces.insert(*f);
                }
                HoldTarget::Setting { .. } => held.settings += 1,
            }
        }
        for s in view.ships() {
            if let ShipLocation::TaskForce(f) = s.location {
                held.members.entry(f).or_default().insert(s.id);
            }
        }
        held
    }

    /// Whether an order of the staff reaches what is held.
    fn reaches(&self, kind: &OrderKind) -> bool {
        match kind {
            OrderKind::TaskForce { target, .. } => {
                self.forces.contains(target)
                    || self
                        .members
                        .get(target)
                        .is_some_and(|m| m.iter().any(|s| self.ships.contains(s)))
            }
            OrderKind::Base {
                order: BaseOrder::FormTaskForce { ships },
                ..
            } => ships.iter().any(|s| self.ships.contains(s)),
            _ => false,
        }
    }
}

/// A game played to its end with the doctrine's staffs passed in.
struct Played {
    scenario: Scenario,
    /// The figure at T at the staff's moment of each day.
    readings: BTreeMap<u32, Option<DaysOfSupply>>,
    /// What was held at the staff's moment of each day.
    held: BTreeMap<u32, Held>,
    handed: BTreeMap<u32, StaffDecisions>,
    asked_twice_alike: bool,
    truth: Vec<TruthState>,
    hashes: Vec<u64>,
    events: Vec<Event>,
}

fn play(text: &str, path: &Path) -> Played {
    play_scenario(
        common::from_text(text, path).expect("the scenario loads"),
        T,
    )
}

fn play_at(text: &str, path: &Path, t: Hex) -> Played {
    play_scenario(
        common::from_text(text, path).expect("the scenario loads"),
        t,
    )
}

fn play_scenario(mut scenario: Scenario, t: Hex) -> Played {
    let held_staffs = staffs();
    let mut readings = BTreeMap::new();
    let mut held = BTreeMap::new();
    let mut handed = BTreeMap::new();
    let mut truth = vec![scenario.world.umpire().clone()];
    let mut hashes = Vec::new();
    let mut asked_twice_alike = true;
    loop {
        let mut asked: Option<StaffDecisions> = None;
        let mut reading = None;
        let mut holds = Held::default();
        let mut again = true;
        let day = scenario.world.day() + 1;
        let summary = scenario
            .next_day_watched(as_staffs(&held_staffs), &mut |world, moment| {
                if moment == (Moment::Orders { day }) {
                    let view = world.side_view(Side::Allied);
                    reading = Some(view.days_of_supply(t));
                    holds = Held::read(view);
                    let first = SupplyStaff.decide(view);
                    again = first == SupplyStaff.decide(view);
                    asked = Some(first);
                }
            })
            .expect("a day resolves");
        let Some(summary) = summary else { break };
        asked_twice_alike &= again;
        readings.insert(day, reading.expect("the staff's moment came"));
        held.insert(day, holds);
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
        held,
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

    fn stock_read(&self, day: u32) -> Fixed {
        self.readings[&day].expect("a figure at T").stock_t
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

    fn unloaded(&self, day: u32) -> Fixed {
        self.on_day(day, |k| matches!(k, EventKind::CargoUnloaded { .. }))
            .iter()
            .map(|e| match &e.kind {
                EventKind::CargoUnloaded { unloaded_t, .. } => *unloaded_t,
                _ => Fixed::ZERO,
            })
            .fold(Fixed::ZERO, |sum, t| sum + t)
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

    /// The staff gave no order, on any day, to a ship held that morning, to
    /// the force it lies in or to a force held.
    fn staff_never_orders_what_is_held(&self) {
        for (day, order) in self.orders() {
            let held = &self.held[&day];
            assert!(
                !held.reaches(&order.kind),
                "day {day}: the staff ordered what is held: {:?}",
                order.kind
            );
        }
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
            cargo: sim_core::orders::Cargo::Supply,
            weight_t: n(weight),
        },
    )
}

fn form(raw_ships: &[u32]) -> OrderKind {
    OrderKind::Base {
        target: base(1),
        order: BaseOrder::FormTaskForce {
            ships: raw_ships.iter().map(|r| ship(*r)).collect(),
        },
    }
}

/// Every piece of a reason, as one text.
fn whole(reason: &Reason) -> String {
    let mut all = reason.worked.clone();
    for e in &reason.evidence {
        all.push('\n');
        all.push_str(&e.what);
    }
    for b in &reason.binding {
        all.push('\n');
        all.push_str(b);
    }
    for w in &reason.would_change {
        all.push('\n');
        all.push_str(w);
    }
    all
}

fn mentions(reason: &Reason, words: &[&str]) {
    let all = whole(reason);
    for word in words {
        assert!(all.contains(word), "the reason has no \"{word}\": {all}");
    }
}

fn add_runs(d: &StaffDecisions) -> Vec<(BTreeSet<ShipId>, Fixed, RunDays, &Reason)> {
    d.commands
        .iter()
        .filter_map(|c| match &c.kind {
            CommandKind::AddRun { run, .. } => {
                Some((run.ships.clone(), run.load_t, run.days, &c.reason))
            }
            _ => None,
        })
        .collect()
}

fn load_orders(d: &StaffDecisions) -> Vec<Fixed> {
    d.orders
        .iter()
        .filter_map(|o| match &o.kind {
            OrderKind::TaskForce {
                order: TaskForceOrder::LoadTo { weight_t, .. },
                ..
            } => Some(*weight_t),
            _ => None,
        })
        .collect()
}

// -------------------------------------------------------------------- 41a

/// 41a's readings at T at the staff's moment, by day: the table's column.
fn table_a() -> Vec<(u32, i64)> {
    let mut table = vec![
        (1, 100),
        (2, 90),
        (3, 80),
        (4, 150),
        (5, 420),
        (6, 410),
        (7, 400),
        (8, 390),
        (9, 380),
        (10, 370),
        (11, 360),
        (12, 430),
        (13, 530),
        (14, 520),
    ];
    for day in 15..=24 {
        table.push((day, 510 - 10 * i64::from(day - 15)));
    }
    table.extend([(25, 410), (26, 400), (27, 470)]);
    table
}

#[test]
fn a_the_readings_at_t_follow_the_table() {
    let played = play(&text_a(), &path_a());
    for (day, stock) in table_a() {
        let got = played.readings[&day].expect("a figure at T");
        assert_eq!(got.stock_t, n(stock), "stock at T, day {day}");
        assert_eq!(got.days, Some(n(stock / 10)), "days at T, day {day}");
        assert_eq!(got.planned_use.total_t(), n(10), "use at T, day {day}");
    }
    assert_eq!(played.readings.len(), 27);
    // 10 days read on day 1, 37 on day 10.
    assert_eq!(played.readings[&10].unwrap().days, Some(n(37)));
    assert!(played.asked_twice_alike, "the staff answers one view alike");
    // The stock at T at the end of a day is the reading of the next, and 530 t
    // at the end of day 27.
    for day in 1..=26 {
        assert_eq!(
            played.base_supply(day, 2),
            played.stock_read(day + 1),
            "{day}"
        );
    }
    assert_eq!(played.base_supply(27, 2), n(530));
}

#[test]
fn a_the_staffs_hand_in_follows_the_table_day_by_day() {
    let played = play(&text_a(), &path_a());
    played.nothing_refused();

    // Day 1: made ready, not released; run 1 added and committed; a force of
    // ship 1 formed and loaded with 360 t.
    let d1 = played.decisions(1);
    assert_eq!(d1.commands.len(), 3);
    assert_eq!(d1.commands[0].kind, CommandKind::Approve { plan: plan(1) });
    mentions(&d1.commands[0].reason, &["makes it ready (preparing)"]);
    let runs = add_runs(d1);
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].0, BTreeSet::from([ship(1)]));
    assert_eq!(runs[0].1, n(360));
    assert_eq!(
        runs[0].2,
        RunDays {
            load: 1,
            sail: 2,
            arrive: 2,
            free: 5
        }
    );
    mentions(runs[0].3, &["300 - 90", "150", "360 t"]);
    assert_eq!(d1.orders.len(), 2);
    assert_eq!(d1.orders[0].kind, form(&[1]));
    assert_eq!(d1.orders[1].kind, load_to(FORCE, 360));
    assert!(d1.no_orders.is_empty());

    // Day 2: no order; the run is loaded and waits for leave to launch.
    let d2 = played.decisions(2);
    assert!(d2.orders.is_empty() && d2.commands.is_empty());
    let [reason] = &d2.no_orders[..] else {
        panic!("one reason")
    };
    mentions(
        reason,
        &[
            "Run 1 is loaded (360 t aboard) and waits for leave to launch",
            "No new run",
        ]
        .map(|w| {
            if w == "No new run" {
                "no new run is considered"
            } else {
                w
            }
        }),
    );
    assert!(reason
        .binding
        .iter()
        .any(|b| b == "leave to launch is withheld"));
    assert!(reason
        .would_change
        .iter()
        .any(|w| w.contains("leave to launch given")));

    // Day 3: released; run 1 timed again; route to T and back, acting at T,
    // with an unload order.
    let d3 = played.decisions(3);
    assert!(d3.no_orders.is_empty());
    assert_eq!(
        d3.commands[0].kind,
        CommandKind::TimeRun {
            plan: plan(1),
            run: 1,
            days: RunDays {
                load: 1,
                sail: 3,
                arrive: 3,
                free: 6
            }
        }
    );
    assert_eq!(d3.orders.len(), 2);
    assert_eq!(d3.orders[0].kind, to_force(FORCE, route_to(&[T, S], &[T])));
    assert_eq!(d3.orders[1].kind, to_force(FORCE, TaskForceOrder::Unload));
    mentions(&d3.orders[0].reason, &["hour 48", "hour 68", "80 t", "360"]);

    // Days 4 to 9: no order, the next run waits for the reading of day 18.
    for day in 4..=9 {
        let d = played.decisions(day);
        assert!(d.orders.is_empty() && d.commands.is_empty(), "day {day}");
        assert_eq!(d.no_orders.len(), 1, "day {day}");
        mentions(
            &d.no_orders[0],
            &[
                "reading of day 18",
                "(300 t)",
                "expected to be ordered on day 16",
            ],
        );
    }
    // Day 8, with ship 1 held: still no run, and the reading of day 10 is
    // 370 t.
    mentions(
        &played.decisions(8).no_orders[0],
        &["the reading of day 10 is projected at 370 t"],
    );

    // Day 10: amendment 1; ship 2 forms a force and loads 190 t.
    let d10 = played.decisions(10);
    assert!(d10.no_orders.is_empty());
    let runs = add_runs(d10);
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].0, BTreeSet::from([ship(2)]));
    assert_eq!(runs[0].1, n(190));
    assert_eq!(
        runs[0].2,
        RunDays {
            load: 10,
            sail: 11,
            arrive: 11,
            free: 14
        }
    );
    assert_eq!(runs[0].3.serves.amendment, Some(1));
    let words = whole(runs[0].3);
    for word in [
        "400 - 360",
        "190 t",
        "ship:1 is held by the commander's hold of day 8",
        "Without the hold, ship:1 would have been sent.",
    ] {
        assert!(words.contains(word), "no \"{word}\": {words}");
    }
    assert_eq!(d10.orders.len(), 2);
    assert_eq!(d10.orders[0].kind, form(&[2]));
    assert_eq!(d10.orders[1].kind, load_to(SECOND, 190));
    for order in &d10.orders {
        assert_eq!(order.reason.serves.amendment, Some(1));
    }

    // Day 11: route to T and back, acting at T, with an unload order.
    let d11 = played.decisions(11);
    assert!(d11.commands.is_empty() && d11.no_orders.is_empty());
    assert_eq!(d11.orders.len(), 2);
    assert_eq!(
        d11.orders[0].kind,
        to_force(SECOND, route_to(&[T, S], &[T]))
    );
    assert_eq!(d11.orders[1].kind, to_force(SECOND, TaskForceOrder::Unload));
    mentions(
        &d11.orders[0].reason,
        &["hour 240", "hour 260", "80 t", "190"],
    );

    // Days 12 to 24 and 27: no order. Day 17, the release: the next run is
    // expected on day 25.
    for day in (12..=24).chain([27]) {
        let d = played.decisions(day);
        assert!(d.orders.is_empty() && d.commands.is_empty(), "day {day}");
        assert_eq!(d.no_orders.len(), 1, "day {day}");
    }
    for day in 12..=24 {
        mentions(
            &played.decisions(day).no_orders[0],
            &[
                "reading of day 27",
                "(400 t)",
                "expected to be ordered on day 25",
            ],
        );
    }
    mentions(
        &played.decisions(17).no_orders[0],
        &["expected to be ordered on day 25"],
    );

    // Day 25: ship 1, in the force formed on day 1, loads 150 t.
    let d25 = played.decisions(25);
    let runs = add_runs(d25);
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].0, BTreeSet::from([ship(1)]));
    assert_eq!(runs[0].1, n(150));
    assert_eq!(
        runs[0].2,
        RunDays {
            load: 25,
            sail: 26,
            arrive: 26,
            free: 29
        }
    );
    mentions(runs[0].3, &["400 - 400", "150", "(15 days of use)"]);
    assert_eq!(d25.orders.len(), 1);
    assert_eq!(d25.orders[0].kind, load_to(FORCE, 150));
    assert!(!whole(runs[0].3).contains("held"));
    assert_eq!(runs[0].3.serves.amendment, Some(1));

    // Day 26: route and unload.
    let d26 = played.decisions(26);
    assert_eq!(d26.orders.len(), 2);
    assert_eq!(d26.orders[0].kind, to_force(FORCE, route_to(&[T, S], &[T])));
    assert_eq!(d26.orders[1].kind, to_force(FORCE, TaskForceOrder::Unload));
    mentions(
        &d26.orders[0].reason,
        &["hour 600", "hour 620", "80 t", "150"],
    );

    // No staff order was refused, none reached what is held; the orders of
    // the staff are those of the table and no more.
    played.staff_never_orders_what_is_held();
    assert_eq!(played.orders().len(), 11);
    // The commands of the commander were all taken.
    assert!(played
        .all(|k| matches!(k, EventKind::CommandRefused { .. }))
        .is_empty());
}

/// The table's day 2 says the reason names the commander's approval as what
/// would change it, with leave given.
#[test]
fn a_day_2_reason_names_the_approval_as_what_would_change_it() {
    let played = play(&text_a(), &path_a());
    let would = played.decisions(2).no_orders[0].would_change.join(" | ");
    assert!(would.contains("approv"), "{would}");
}

#[test]
fn a_what_happens_follows_the_table_exactly() {
    let played = play(&text_a(), &path_a());
    played.nothing_refused();

    // Day 1: loaded in 7.2 hours; base 1 holds 4640 t.
    assert_eq!(played.loading_complete(1), vec![fx("7.2")]);
    assert_eq!(played.base_supply(1, 1), n(4640));
    // Day 2: nothing moves: the run is loaded and waits.
    assert!(played
        .on_day(2, |k| matches!(
            k,
            EventKind::TaskForceHalted { .. }
                | EventKind::TaskForceArrived { .. }
                | EventKind::TaskForceSteamed { .. }
        ))
        .is_empty());
    assert!(played.unloaded(2).is_zero());
    assert_eq!(played.at(2).ships[&ship(1)].supply_t, n(360));
    assert_eq!(played.base_supply(2, 1), n(4640));
    // Day 3: sails at hour 48, arrives at hour 68, 80 t landed.
    assert_eq!(played.halted_at(3, FORCE, T), Some(n(68)));
    assert_eq!(played.unloaded(3), n(80));
    // Day 4: the last 280 t take 14 hours: unloaded at hour 86.
    assert_eq!(played.unloaded(4), n(280));
    assert_eq!(played.unloading_complete(4), vec![n(86)]);
    // Day 5: arrives at S at hour 110.
    assert_eq!(played.arrived_at(5, FORCE, S), Some(n(110)));
    // Day 6: ship 1 refuels 40 t: 1960 t of fuel at base 1.
    assert_eq!(played.base_fuel(5, 1), n(2000));
    assert_eq!(played.base_fuel(6, 1), n(1960));
    assert_eq!(played.at(5).ships[&ship(1)].fuel_t, n(960));
    assert_eq!(played.at(6).ships[&ship(1)].fuel_t, n(1000));
    // Day 8: ship 1 is held, and the hold is recorded.
    assert!(played.held[&8].ships.contains(&ship(1)));
    assert!(!played.held[&7].ships.contains(&ship(1)));
    // Day 10: loaded in 3.8 hours; base 1 holds 4450 t.
    assert_eq!(played.loading_complete(10), vec![n(9 * 24) + fx("3.8")]);
    assert_eq!(played.base_supply(10, 1), n(4450));
    // Day 11: sails at hour 240, arrives at hour 260, 80 t landed.
    assert_eq!(played.halted_at(11, SECOND, T), Some(n(260)));
    assert_eq!(played.unloaded(11), n(80));
    // Day 12: the last 110 t take 5.5 hours: unloaded at hour 269.5.
    assert_eq!(played.unloaded(12), n(110));
    assert_eq!(played.unloading_complete(12), vec![fx("269.5")]);
    // Day 13: arrives at S at hour 296.
    assert_eq!(played.arrived_at(13, SECOND, S), Some(n(296)));
    // Day 14: ship 2 refuels 40 t: 1920 t of fuel at base 1.
    assert_eq!(played.base_fuel(13, 1), n(1960));
    assert_eq!(played.base_fuel(14, 1), n(1920));
    // Day 17: ship 1 is released.
    assert!(played.held[&16].ships.contains(&ship(1)));
    assert!(played.held[&17].ships.is_empty());
    // Day 25: loaded in 3 hours; base 1 holds 4300 t.
    assert_eq!(played.loading_complete(25), vec![n(24 * 24 + 3)]);
    assert_eq!(played.base_supply(25, 1), n(4300));
    // Day 26: sails at hour 600, arrives at hour 620, 80 t landed.
    assert_eq!(played.halted_at(26, FORCE, T), Some(n(620)));
    assert_eq!(played.unloaded(26), n(80));
    // Day 27: the last 70 t take 3.5 hours: unloaded at hour 627.5.
    assert_eq!(played.unloaded(27), n(70));
    assert_eq!(played.unloading_complete(27), vec![fx("627.5")]);

    // The end of day 27.
    assert_eq!(played.base_supply(27, 2), n(530));
    assert_eq!(played.base_supply(27, 1), n(4300));
    assert_eq!(played.base_fuel(27, 1), n(1920));
    let end = played.at(27);
    // Ship 2 is stopped at base 1 in the force formed on day 10, empty.
    let two = &end.ships[&ship(2)];
    assert_eq!(two.location, ShipLocation::TaskForce(force(SECOND)));
    assert!(two.supply_t.is_zero() && two.cargo_fuel_t.is_zero());
    assert_eq!(two.fuel_t, n(1000));
    let stopped = &end.task_forces[&force(SECOND)];
    assert_eq!(stopped.position(), S);
    assert_ne!(stopped.status, TaskForceStatus::Underway);
    let one = &end.ships[&ship(1)];
    assert_eq!(one.location, ShipLocation::TaskForce(force(FORCE)));
    assert!(one.supply_t.is_zero() && one.cargo_fuel_t.is_zero());
    assert_eq!(one.fuel_t, n(968));
    let formed = &end.task_forces[&force(FORCE)];
    assert_eq!(formed.status, TaskForceStatus::Underway);
    assert_eq!(formed.route.waypoints.last(), Some(&S));
    let from_s = |hex: Hex| i64::from(hex.distance(S)) * 40;
    assert_eq!(from_s(formed.from_hex) - formed.progress_nm.trunc(), 80);

    // Three runs and one amendment; the plan is released.
    let state = played.scenario.world.command_state(Side::Allied);
    let made = &state.plans[&plan(1)];
    assert_eq!(made.state, PlanState::Released);
    assert_eq!(made.runs.len(), 3);
    assert_eq!(made.amendments.len(), 1);
    assert_eq!(made.runs[&1].load_t, n(360));
    assert_eq!(made.runs[&2].load_t, n(190));
    assert_eq!(made.runs[&3].load_t, n(150));
    assert!(made.runs.values().all(|r| r.ended.is_none()));
    // The release lifted the hold: nothing is held at the end.
    assert!(state.holds.is_empty());
    // The hold of day 8 is an event of its own? No: a command's hold is
    // recorded by its command event, not `HeldByOrder`.
    assert!(played
        .all(|k| matches!(k, EventKind::HeldByOrder { .. }))
        .is_empty());
}

// -------------------------------------------------------------------- 41b

fn played_b() -> Played {
    play(&read(&path_b()), &path_b())
}

#[test]
fn b_the_readings_and_the_hand_in_follow_the_table() {
    let played = played_b();
    played.nothing_refused();
    for (day, stock) in [(1, 100), (2, 90), (3, 80), (4, 150), (5, 430)] {
        let got = played.readings[&day].expect("a figure at T");
        assert_eq!(got.stock_t, n(stock), "stock at T, day {day}");
        assert_eq!(got.days, Some(n(stock / 10)), "days at T, day {day}");
    }
    assert_eq!(played.readings.len(), 5);
    assert!(played.asked_twice_alike);

    // Day 1: as scenario 36: a force of ship 1, 360 t.
    let d1 = played.decisions(1);
    let runs = add_runs(d1);
    assert_eq!(runs[0].0, BTreeSet::from([ship(1)]));
    assert_eq!(runs[0].1, n(360));
    assert_eq!(d1.orders.len(), 2);
    assert_eq!(d1.orders[0].kind, form(&[1]));
    assert_eq!(d1.orders[1].kind, load_to(FORCE, 360));
    // The leaves are given: a run is added and sails on day 2, but the
    // commander's order of day 2 comes first.
    assert_eq!(runs[0].2.sail, 2);

    // Day 2: the force is held before the staff is asked.
    assert!(played.held[&2].forces.contains(&force(FORCE)));
    assert!(!played.held[&1].forces.contains(&force(FORCE)));
    let d2 = played.decisions(2);
    let ended: Vec<_> = d2
        .commands
        .iter()
        .filter(|c| matches!(c.kind, CommandKind::EndRun { .. }))
        .collect();
    assert_eq!(ended.len(), 1);
    assert!(matches!(
        ended[0].kind,
        CommandKind::EndRun { plan: p, run: 1 } if p == plan(1)
    ));
    assert!(
        ended[0]
            .reason
            .worked
            .contains("task_force:1000001 is held by the commander's order of day 2"),
        "{}",
        ended[0].reason.worked
    );
    assert!(d2.commands.iter().any(
        |c| matches!(&c.kind, CommandKind::FreeShips { ships, .. } if ships.contains(&ship(1)))
    ));
    let runs = add_runs(d2);
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].0, BTreeSet::from([ship(2)]));
    assert_eq!(runs[0].1, n(370));
    assert_eq!(runs[0].2.sail, 3);
    assert_eq!(runs[0].2.arrive, 3);
    mentions(
        runs[0].3,
        &[
            "max(0, 300 - 80) + 150",
            "370 t",
            "the reading of day 4 is 70 t",
            "The stock projected for the reading of day 3 is 80 t",
            "task_force:1000001 is held by the commander's order of day 2",
        ],
    );
    assert_eq!(d2.orders.len(), 2);
    assert_eq!(d2.orders[0].kind, form(&[2]));
    assert_eq!(d2.orders[1].kind, load_to(SECOND, 370));
    // No order of the staff reaches the held force.
    for order in &d2.orders {
        if let OrderKind::TaskForce { target, .. } = &order.kind {
            assert_ne!(*target, force(FORCE));
        }
    }

    // Day 3: route and unload for the second force. Days 4 and 5: nothing.
    let d3 = played.decisions(3);
    assert_eq!(d3.orders.len(), 2);
    assert_eq!(d3.orders[0].kind, to_force(SECOND, route_to(&[T, S], &[T])));
    assert_eq!(d3.orders[1].kind, to_force(SECOND, TaskForceOrder::Unload));
    mentions(&d3.orders[0].reason, &["hour 48", "hour 68", "80 t", "370"]);
    for day in [4, 5] {
        let d = played.decisions(day);
        assert!(d.orders.is_empty() && d.commands.is_empty(), "day {day}");
        assert_eq!(d.no_orders.len(), 1);
    }
    played.staff_never_orders_what_is_held();
    assert_eq!(played.orders().len(), 2 + 2 + 2);
}

#[test]
fn b_what_happens_follows_the_table_exactly() {
    let played = played_b();
    played.nothing_refused();

    // Day 1: loaded in 7.2 hours; base 1 holds 4640 t.
    assert_eq!(played.loading_complete(1), vec![fx("7.2")]);
    assert_eq!(played.base_supply(1, 1), n(4640));
    // Day 2, the night: the first force has the berth and puts its 360 t
    // ashore in 7.2 hours (hour 31.2); in the morning the second has the
    // berth and loads the last 130 t in 2.6 hours. Base 1 holds 4630 t.
    assert_eq!(played.unloaded(2), n(360));
    assert_eq!(played.unloading_complete(2), vec![n(24) + fx("7.2")]);
    assert_eq!(played.loading_complete(2), vec![n(36) + fx("2.6")]);
    assert_eq!(played.base_supply(2, 1), n(4630));
    let loaded = played.on_day(2, |k| matches!(k, EventKind::CargoLoaded { .. }));
    let by_lighter: Fixed = loaded
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::CargoLoaded {
                task_force,
                loaded_t,
                ..
            } if *task_force == force(SECOND) => Some(*loaded_t),
            _ => None,
        })
        .fold(Fixed::ZERO, |a, t| a + t);
    assert_eq!(by_lighter, n(370));
    // Day 3: sails at hour 48, arrives at hour 68, 80 t landed.
    assert_eq!(played.halted_at(3, SECOND, T), Some(n(68)));
    assert_eq!(played.unloaded(3), n(80));
    // Day 4: the last 290 t take 14.5 hours: unloaded at hour 86.5.
    assert_eq!(played.unloaded(4), n(290));
    assert_eq!(played.unloading_complete(4), vec![fx("86.5")]);
    // Day 5: arrives at S at hour 110.
    assert_eq!(played.arrived_at(5, SECOND, S), Some(n(110)));

    // The end of day 5.
    assert_eq!(played.base_supply(5, 2), n(420));
    assert_eq!(played.base_supply(5, 1), n(4630));
    assert_eq!(played.base_fuel(5, 1), n(2000));
    let end = played.at(5);
    let one = &end.ships[&ship(1)];
    assert_eq!(one.location, ShipLocation::TaskForce(force(FORCE)));
    assert!(one.supply_t.is_zero() && one.cargo_fuel_t.is_zero());
    assert_eq!(one.fuel_t, n(1000));
    assert_eq!(end.task_forces[&force(FORCE)].position(), S);
    let two = &end.ships[&ship(2)];
    assert_eq!(two.location, ShipLocation::TaskForce(force(SECOND)));
    assert!(two.supply_t.is_zero() && two.cargo_fuel_t.is_zero());
    assert_eq!(two.fuel_t, n(960));
    assert_eq!(end.task_forces[&force(SECOND)].position(), S);
    // The first force is held at the end.
    let state = played.scenario.world.command_state(Side::Allied);
    assert!(state
        .holds
        .contains_key(&HoldTarget::TaskForce(force(FORCE))));
}

#[test]
fn b_the_override_is_an_event_citing_the_order_and_command_state_shows_it() {
    let played = played_b();
    let events = played.all(|k| matches!(k, EventKind::HeldByOrder { .. }));
    assert_eq!(events.len(), 1);
    let event = events[0];
    assert_eq!(event.day, 2);
    let EventKind::HeldByOrder {
        side,
        target,
        order,
    } = &event.kind
    else {
        unreachable!()
    };
    assert_eq!(*side, Side::Allied);
    assert_eq!(*target, HoldTarget::TaskForce(force(FORCE)));
    // It cites the commander's order and that order's acceptance.
    assert!(
        event.causes.contains(&Cause::Order(*order)),
        "{:?}",
        event.causes
    );
    let accepted: Vec<&Event> =
        played.all(|k| matches!(k, EventKind::OrderAccepted { order: o, .. } if o == order));
    assert_eq!(accepted.len(), 1);
    assert!(
        event.causes.contains(&Cause::Event(accepted[0].id)),
        "{:?}",
        event.causes
    );
    // The command state shows the hold, made by that order, on day 2.
    let state = played.scenario.world.command_state(Side::Allied);
    let hold = state.holds[&HoldTarget::TaskForce(force(FORCE))];
    assert_eq!(hold.day, 2);
    assert_eq!(hold.by, HoldMaker::Order(*order));
    assert_eq!(state.holds.len(), 1);
}

// -------------------------------------------------------------------- 41c

fn played_c() -> Played {
    play_at(&read(&path_c()), &path_c(), FAR_T)
}

#[test]
fn c_the_readings_and_the_hand_in_follow_the_table() {
    let played = played_c();
    played.nothing_refused();
    // Nothing is landed before the plan is cancelled: T only pays.
    for (day, stock) in [(1, 100), (2, 90), (3, 80), (4, 70), (5, 60)] {
        assert_eq!(played.stock_read(day), n(stock), "day {day}");
    }

    // Day 1: a run ordered today sails on day 2 and arrives on day 3; the
    // load is 300 - 80 + 150 = 370 t, on ship 1.
    let d1 = played.decisions(1);
    let runs = add_runs(d1);
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].0, BTreeSet::from([ship(1)]));
    assert_eq!(runs[0].1, n(370));
    assert_eq!(runs[0].2.load, 1);
    assert_eq!(runs[0].2.sail, 2);
    assert_eq!(runs[0].2.arrive, 3);
    mentions(
        runs[0].3,
        &[
            "max(0, 300 - 80) + 150",
            "370 t",
            "the reading of day 4 is 70 t",
            "The stock projected for the reading of day 3 is 80 t",
        ],
    );
    assert_eq!(d1.orders.len(), 2);
    assert_eq!(d1.orders[0].kind, form(&[1]));
    assert_eq!(d1.orders[1].kind, load_to(FORCE, 370));

    // Day 2: the route out and back, acting at T, with an unload order.
    let d2 = played.decisions(2);
    assert_eq!(d2.orders.len(), 2);
    assert_eq!(
        d2.orders[0].kind,
        to_force(FORCE, route_to(&[FAR_T, S], &[FAR_T]))
    );
    assert_eq!(d2.orders[1].kind, to_force(FORCE, TaskForceOrder::Unload));
    mentions(&d2.orders[0].reason, &["hour 24", "370"]);

    // Day 3: the plan is cancelled; the staff frees ship 1's commitment and
    // gives the force a route home, to S, with a reason that names the
    // cancelling.
    let d3 = played.decisions(3);
    assert!(d3.commands.iter().any(
        |c| matches!(&c.kind, CommandKind::FreeShips { ships, .. } if ships.contains(&ship(1)))
    ));
    assert_eq!(d3.orders.len(), 1);
    assert_eq!(d3.orders[0].kind, to_force(FORCE, route_to(&[S], &[])));
    assert_eq!(
        d3.orders[0].reason.worked,
        "plan:1 is cancelled: task_force:1000001 on passage out to base:2 is sent home to \
         base:1; its cargo stays aboard and its unload order stands, so it is put back \
         ashore at base:1."
    );
    // The force is not given a second unload order: its own stands.
    assert!(!d3.orders.iter().any(|o| matches!(
        o.kind,
        OrderKind::TaskForce {
            order: TaskForceOrder::Unload,
            ..
        }
    )));

    // Days 4 and 5: no order; the reason says the plan is cancelled.
    for day in [4, 5] {
        let d = played.decisions(day);
        assert!(d.orders.is_empty() && d.commands.is_empty(), "day {day}");
        let [reason] = &d.no_orders[..] else {
            panic!("one reason on day {day}")
        };
        assert!(
            reason
                .worked
                .contains("plan:1 is cancelled, so the staff gives no order for intent:1"),
            "{}",
            reason.worked
        );
    }
    played.staff_never_orders_what_is_held();
    assert_eq!(played.orders().len(), 2 + 2 + 1);
}

#[test]
fn c_what_happens_follows_the_table_exactly() {
    let played = played_c();
    played.nothing_refused();

    // Day 1: loaded in 7.4 hours; base 1 holds 4630 t.
    assert_eq!(played.loading_complete(1), vec![fx("7.4")]);
    assert_eq!(played.base_supply(1, 1), n(4630));
    // Day 2: the force sails at hour 24. At hour 48 it is 240 nm out, in hex
    // (6,0), at the end of day 2.
    let out = &played.at(2).task_forces[&force(FORCE)];
    assert_eq!(out.status, TaskForceStatus::Underway);
    assert_eq!(out.position(), Hex::new(6, 0));
    assert!(played.at(2).ships[&ship(1)].supply_t == n(370));
    // Day 3: the force turns and is back at S at hour 72, with 370 t aboard
    // and 952 t of fuel, at the end of the day.
    let back = &played.at(3).task_forces[&force(FORCE)];
    assert_eq!(back.position(), S);
    assert!(back.toward_hex.is_none());
    assert_ne!(back.status, TaskForceStatus::Underway);
    let one = &played.at(3).ships[&ship(1)];
    assert_eq!(one.supply_t, n(370));
    assert_eq!(one.fuel_t, n(952));
    assert!(played.unloaded(3).is_zero());
    // Day 4: ship 1 refuels 48 t, and its unload order stands: 370 t ashore
    // in 7.4 hours at S.
    assert_eq!(played.base_fuel(3, 1), n(2000));
    assert_eq!(played.base_fuel(4, 1), n(1952));
    assert_eq!(played.unloaded(4), n(370));
    assert_eq!(played.unloading_complete(4), vec![n(72) + fx("7.4")]);
    assert_eq!(played.base_supply(4, 1), n(5000));
    // Day 5: nothing more.
    assert!(played.unloaded(5).is_zero());

    // The end of day 5.
    assert_eq!(played.base_supply(5, 1), n(5000));
    assert_eq!(played.base_fuel(5, 1), n(1952));
    assert_eq!(played.base_supply(5, 2), n(50));
    let end = played.at(5);
    let one = &end.ships[&ship(1)];
    assert_eq!(one.location, ShipLocation::TaskForce(force(FORCE)));
    assert!(one.supply_t.is_zero() && one.cargo_fuel_t.is_zero());
    assert_eq!(one.fuel_t, n(1000));
    assert_eq!(end.task_forces[&force(FORCE)].position(), S);
    assert_ne!(
        end.task_forces[&force(FORCE)].status,
        TaskForceStatus::Underway
    );
    let state = played.scenario.world.command_state(Side::Allied);
    assert_eq!(state.plans[&plan(1)].state, PlanState::Cancelled);
    assert!(state.intents[&sim_core::ids::IntentId::new(1)].in_force());
    assert_eq!(state.plans.len(), 1);
}

// ------------------------------------------------------------- The preview

use serde_json::{json, Value};
use sim_agent::{Seat, Session};

/// 41a's text as far as the morning of day 10: the commands of days 1 to 9
/// only (the intent, the limits, the approval and the hold).
fn until_day_10() -> String {
    let commands = format!(
        "{}{}{}",
        intent_and_limits("[1, 2]"),
        command_table(3, "kind = \"approve\"\nplan = 1"),
        command_table(8, "kind = \"hold\"\nship = 1"),
    );
    a_with(27, &commands)
}

/// A session of the Allied seat with the doctrine's staff, the scenario's
/// scripted commands submitted as the commander's, played to the morning of
/// `day`.
fn session_at(text: &str, day: u32) -> Session {
    let mut scenario = common::from_text(text, &path_a()).expect("the scenario loads");
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

fn ask(session: &mut Session, tool: &str, args: Value) -> Value {
    match session.call(Side::Allied, tool, &args) {
        Ok(answer) => answer,
        Err(error) => error.to_json(),
    }
}

const CHANGE: fn() -> Value = || json!({"command": "set_intent", "intent": 1, "base": 2, "floor_days": 40, "interval_days": 15});

fn runs_of(side: &Value) -> Vec<(Vec<u64>, String, [u64; 4])> {
    side["runs"]
        .as_array()
        .expect("runs")
        .iter()
        .map(|r| {
            (
                r["ships"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|s| s.as_u64().unwrap())
                    .collect(),
                r["load_t"].as_str().unwrap().to_string(),
                ["load", "sail", "arrive", "free"].map(|k| r["days"][k].as_u64().unwrap()),
            )
        })
        .collect()
}

#[test]
fn a_the_preview_says_what_the_change_would_do_and_changes_nothing() {
    let mut session = session_at(&until_day_10(), 10);
    assert_eq!(session.day(), 9);
    let hash = session.state_hash();
    let state = ask(&mut session, "command_state", json!({}));
    let orders = ask(&mut session, "staff_orders", json!({}));
    let queued = session.commands_handed_in().clone();

    let v = ask(&mut session, "preview", CHANGE());
    assert_eq!(v["kind"], "preview", "{v}");
    assert_eq!(v["preview"], 1);
    assert_eq!(v["day"], 10);
    assert_eq!(v["intent"], 1);
    assert_eq!(v["valid"], true);

    // As things stand: no order today, the next run is 150 t, the run of day
    // 16, ship 1 being held so ship 2's.
    let stands = &v["as_things_stand"];
    assert_eq!(stands["orders"], json!([]));
    assert_eq!(stands["commands"], json!([]));
    let next = runs_of(stands);
    assert_eq!(next[0], (vec![2], "150".to_string(), [16, 17, 17, 20]));
    assert_eq!(stands["next_run"]["load_t"], "150");
    assert_eq!(stands["next_run"]["days"]["load"], 16);
    assert_eq!(stands["first_day_below"], 18);
    let reason = stands["no_orders"][0]["worked"].as_str().unwrap();
    assert!(
        reason.contains("expected to be ordered on day 16"),
        "{reason}"
    );

    // With the change: a run of 190 t ordered on day 10, by ship 2.
    let with = &v["with_change"];
    let next = runs_of(with);
    assert_eq!(next[0], (vec![2], "190".to_string(), [10, 11, 11, 14]));
    assert_eq!(with["next_run"]["load_t"], "190");
    assert_eq!(with["first_day_below"], 10);
    assert_eq!(with["no_orders"], json!([]));
    let given = with["orders"].as_array().expect("orders");
    assert_eq!(given[0]["order"]["kind"], "form_task_force");
    assert_eq!(given[0]["order"]["ships"], json!([2]));
    assert_eq!(given[1]["order"]["weight_t"], "190");
    let words = with["commands"][0]["reason"]["worked"].as_str().unwrap();
    assert!(
        words.contains("Without the hold, ship:1 would have been sent."),
        "{words}"
    );

    // Ship 1 is held; nothing made ready is lost; no other plan gives up
    // anything.
    assert_eq!(
        v["held"],
        json!(["ship 1 is held by your hold command of day 8"])
    );
    assert!(v["made_ready_lost"]
        .as_str()
        .unwrap()
        .starts_with("nothing"));
    assert_eq!(v["other_plans_give_up"], json!([]));
    assert_eq!(v["runs_in_hand"], json!([]));

    // Nothing is changed: the hash, the command state, the staff's answer and
    // the queue are as they were.
    assert_eq!(session.state_hash(), hash);
    assert_eq!(ask(&mut session, "command_state", json!({})), state);
    assert_eq!(ask(&mut session, "staff_orders", json!({})), orders);
    assert_eq!(session.commands_handed_in(), &queued);
}

#[test]
fn a_approving_the_preview_queues_the_changed_intent_and_the_day_runs_as_the_table_has_it() {
    let mut session = session_at(&until_day_10(), 10);
    let v = ask(&mut session, "preview", CHANGE());
    let a = ask(&mut session, "approve", json!({"preview": 1}));
    assert_eq!(a["status"], "queued", "{a}");
    assert_eq!(a["command"], v["command"]);
    assert_eq!(a["for_day"], 10);
    let end = ask(&mut session, "end_day", json!({}));
    assert_eq!(end["resolved"], true, "{end}");

    // The changed intent was handed in for day 10, and the staff did what the
    // preview said: ship 2 forms a force and loads 190 t.
    let queued = &session.commands_handed_in()[&10];
    assert_eq!(queued.len(), 1);
    assert_eq!(queued[0].1.name(), "set_intent");
    let handed = &session.staff_handed_in()[&10][&Side::Allied];
    let runs = add_runs(handed);
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].0, BTreeSet::from([ship(2)]));
    assert_eq!(runs[0].1, n(190));
    assert_eq!(handed.orders[0].kind, form(&[2]));
    assert_eq!(handed.orders[1].kind, load_to(SECOND, 190));
}

#[test]
fn a_a_release_queued_after_the_preview_voids_it_and_the_next_gives_the_run_to_ship_1() {
    let mut session = session_at(&until_day_10(), 10);
    let first = ask(&mut session, "preview", CHANGE());
    assert_eq!(first["preview"], 1);
    let a = ask(&mut session, "release", json!({"ship": 1}));
    assert_eq!(a["status"], "queued", "{a}");

    // Approving the first is refused, and says why.
    let refused = ask(&mut session, "approve", json!({"preview": 1}));
    assert_eq!(refused["status"], "refused", "{refused}");
    let reason = refused["reason"].as_str().unwrap();
    assert!(
        reason.starts_with(
            "preview 1 is void: something was queued after it (you queued: release ship:1)"
        ),
        "{reason}"
    );
    // The refusal queued nothing: only the release stands for the day.
    let state = ask(&mut session, "command_state", json!({}));
    assert_eq!(state["queued"].as_array().unwrap().len(), 1, "{state}");

    // Asked again, the preview gives the run of 190 t to ship 1.
    let second = ask(&mut session, "preview", CHANGE());
    assert_eq!(second["preview"], 2);
    let next = runs_of(&second["with_change"]);
    assert_eq!(next[0], (vec![1], "190".to_string(), [10, 11, 11, 14]));
    assert_eq!(second["held"], json!([]));
    // Ship 1 lies in the force formed on day 1: no forming, a load order.
    let given = second["with_change"]["orders"].as_array().unwrap();
    assert_eq!(given.len(), 1);
    assert_eq!(given[0]["order"]["weight_t"], "190");
    // The one asked second can be approved.
    let a = ask(&mut session, "approve", json!({"preview": 2}));
    assert_eq!(a["status"], "queued", "{a}");
}

// --------------------------------------------------------- Properties of 41a

/// The commands of 41a itself: the table's.
fn commands_of_a() -> String {
    commands_of_a_to(27)
}

/// The same as far as `last` day: a shorter game may not hold a command of a
/// later day.
fn commands_of_a_to(last: u32) -> String {
    let mut commands = intent_and_limits("[1, 2]");
    for (day, body) in [
        (3, "kind = \"approve\"\nplan = 1"),
        (8, "kind = \"hold\"\nship = 1"),
        (10, ""),
        (17, "kind = \"release\"\nship = 1"),
    ] {
        if day > last {
            continue;
        }
        commands.push_str(&if day == 10 {
            change_floor()
        } else {
            command_table(day, body)
        });
    }
    commands
}

/// The commands the variants are made from are the commands of the file.
#[test]
fn a_the_commands_of_the_file_are_the_ones_these_tests_vary() {
    let file = common::from_text(&text_a(), &path_a()).expect("loads");
    let built = common::from_text(&a_with(27, &commands_of_a()), &path_a()).expect("loads");
    assert_eq!(file.scripted_commands(), built.scripted_commands());
    assert_eq!(file.days, 27);
}

/// The ships of 41a's head in a force of two (and, if asked, a third ship
/// in port in no force): the Allied force 1 at base 1.
fn head_with_force_of_two(days: u32, third: bool) -> String {
    let head = head_of(&text_a()).replace("days = 27", &format!("days = {days}"));
    let mut head = common::replace_once(
        &head,
        "[[ship]]\nid = 1\nname = \"Placeholder cargo ship 1\"\nclass = 3\nside = \"allied\"\nfuel_t = 1000\nport = 1",
        "[[task_force]]\nid = 1\nname = \"Placeholder convoy\"\nside = \"allied\"\nat_base = 1\nmission = \"transport\"\n\n[[ship]]\nid = 1\nname = \"Placeholder cargo ship 1\"\nclass = 3\nside = \"allied\"\nfuel_t = 1000\ntask_force = 1",
    );
    head = common::replace_once(
        &head,
        "[[ship]]\nid = 2\nname = \"Placeholder cargo ship 2\"\nclass = 3\nside = \"allied\"\nfuel_t = 1000\nport = 1",
        "[[ship]]\nid = 2\nname = \"Placeholder cargo ship 2\"\nclass = 3\nside = \"allied\"\nfuel_t = 1000\ntask_force = 1",
    );
    if third {
        head.push_str(
            "\n[[ship]]\nid = 3\nname = \"Placeholder cargo ship 3\"\nclass = 3\nside = \"allied\"\nfuel_t = 1000\nport = 1\n",
        );
    }
    head
}

/// A hand-in of what the staff gave, without the words of its reasons.
fn bare(d: &StaffDecisions) -> (Vec<OrderKind>, Vec<CommandKind>, usize) {
    (
        d.orders.iter().map(|o| o.kind.clone()).collect(),
        d.commands.iter().map(|c| c.kind.clone()).collect(),
        d.no_orders.len(),
    )
}

#[test]
fn a_with_both_leaves_given_and_no_later_commands_it_is_scenario_36_day_for_day() {
    let commands = format!(
        "{}{}{}",
        intent_and_limits("[1, 2]"),
        leave("prepare"),
        leave("launch")
    );
    let mine = play(&a_with(27, &commands), &path_a());
    mine.nothing_refused();
    let path_36 = scenario_path("36_one_convoy.toml");
    let text_36 = common::replace_once(&read(&path_36), "days = 18", "days = 27");
    let theirs = play(&text_36, &path_36);
    assert_eq!(mine.handed.len(), 27);
    assert_eq!(mine.handed, theirs.handed, "the staff hands in the same");
    assert_eq!(mine.readings, theirs.readings);
    assert_eq!(mine.truth, theirs.truth, "the truth is the same each day");
    assert_eq!(mine.hashes, theirs.hashes, "and so is its hash");
    mine.staff_never_orders_what_is_held();
    for day in 1..=27 {
        assert!(mine.held[&day].ships.is_empty() && mine.held[&day].forces.is_empty());
    }
}

#[test]
fn a_with_no_approval_on_day_3_the_run_never_sails() {
    let played = play(&a_with(12, &intent_and_limits("[1, 2]")), &path_a());
    played.nothing_refused();
    // Day 1: the run is loaded, and nothing more is ordered all game: the
    // only orders are the day-1 forming and loading.
    let d1 = played.decisions(1);
    assert_eq!(d1.orders.len(), 2);
    assert_eq!(d1.orders[1].kind, load_to(FORCE, 360));
    assert_eq!(played.orders().len(), 2);
    // Each day from 2 the reason says the run waits for leave to launch.
    for day in 2..=12 {
        let d = played.decisions(day);
        assert!(d.orders.is_empty() && d.commands.is_empty(), "day {day}");
        let [reason] = &d.no_orders[..] else {
            panic!("one reason on day {day}")
        };
        assert!(
            reason.worked.contains("waits for leave to launch"),
            "day {day}: {}",
            reason.worked
        );
    }
    // The ship never moves; base 2 pays 10 t a night and holds 0 t after the
    // night of day 10; its men are unfed on the night of day 11.
    for day in 1..=12 {
        let one = &played.at(day).ships[&ship(1)];
        assert_eq!(one.location, ShipLocation::TaskForce(force(FORCE)));
        assert_eq!(one.supply_t, n(360));
        assert_eq!(played.at(day).task_forces[&force(FORCE)].position(), S);
    }
    for day in 1..=10 {
        assert_eq!(
            played.base_supply(day, 2),
            n(100 - 10 * i64::from(day)),
            "{day}"
        );
    }
    assert!(played.base_supply(10, 2).is_zero());
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
    assert_eq!(fed(10), vec![(2000, 0)]);
    assert_eq!(fed(11), vec![(0, 2000)], "unfed on the night of day 11");
    assert!(played.unloaded(3).is_zero());
}

/// 41a with ship 1 held on day 2 in place of day 8, released on day 8, over
/// 19 days.
fn set_aside_text() -> String {
    let commands = format!(
        "{}{}{}{}",
        intent_and_limits("[1, 2]"),
        command_table(2, "kind = \"hold\"\nship = 1"),
        command_table(3, "kind = \"approve\"\nplan = 1"),
        command_table(8, "kind = \"release\"\nship = 1"),
    );
    a_with(19, &commands)
}

#[test]
fn a_a_run_set_aside_is_not_taken_up_again_and_its_cargo_is_carried_by_the_next() {
    let played = play(&set_aside_text(), &path_a());
    played.nothing_refused();

    // Day 1: as the table has it.
    assert_eq!(add_runs(played.decisions(1))[0].1, n(360));

    // Day 2: the staff ends run 1, with 360 t still aboard ship 1, and frees
    // its commitment; the plan still preparing, it loads ship 2 in its place:
    // a run ordered on day 2 arrives on day 3, the stock projected for day 3
    // is 80 t, the load 300 - 80 + 150 = 370 t.
    let d2 = played.decisions(2);
    let ended: Vec<_> = d2
        .commands
        .iter()
        .filter(|c| matches!(c.kind, CommandKind::EndRun { .. }))
        .collect();
    assert_eq!(ended.len(), 1);
    assert!(matches!(ended[0].kind, CommandKind::EndRun { run: 1, .. }));
    assert_eq!(
        ended[0].reason.worked,
        "Run 1 of plan:1 is ended: ship:1 is held by the commander's hold of day 2; \
         the 360 t aboard stay aboard."
    );
    assert!(d2.commands.iter().any(
        |c| matches!(&c.kind, CommandKind::FreeShips { ships, .. } if ships.contains(&ship(1)))
    ));
    let runs = add_runs(d2);
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].0, BTreeSet::from([ship(2)]));
    assert_eq!(runs[0].1, n(370));
    assert_eq!(runs[0].2.arrive, 3);
    mentions(
        runs[0].3,
        &[
            "max(0, 300 - 80) + 150",
            "The stock projected for the reading of day 3 is 80 t",
            "Without the hold, ship:1 would have been sent.",
        ],
    );
    assert_eq!(d2.orders.len(), 2);
    assert_eq!(d2.orders[0].kind, form(&[2]));
    assert_eq!(d2.orders[1].kind, load_to(SECOND, 370));
    // The 360 t stay aboard ship 1 through the days of the hold and after.
    for day in 2..=17 {
        assert_eq!(played.at(day).ships[&ship(1)].supply_t, n(360), "day {day}");
        assert_eq!(
            played.at(day).ships[&ship(1)].location,
            ShipLocation::TaskForce(force(FORCE))
        );
    }
    // Run 1 is ended in the command state and never taken up again.
    let state = played.scenario.world.command_state(Side::Allied);
    assert_eq!(state.plans[&plan(1)].runs[&1].ended, Some(2));
    assert!(state.holds.is_empty(), "released on day 8");

    // It sails on day 3, when the plan is released: the readings at T are
    // those of 41b.
    let d3 = played.decisions(3);
    assert_eq!(d3.orders[0].kind, to_force(SECOND, route_to(&[T, S], &[T])));
    assert_eq!(d3.orders[1].kind, to_force(SECOND, TaskForceOrder::Unload));
    for (day, stock) in [(1, 100), (2, 90), (3, 80), (4, 150), (5, 430)] {
        assert_eq!(played.stock_read(day), n(stock), "day {day}");
    }
    assert_eq!(played.halted_at(3, SECOND, T), Some(n(68)));
    assert_eq!(played.unloaded(3), n(80));
    assert_eq!(played.unloaded(4), n(290));

    // The release on day 8 gives the staff nothing to do with run 1.
    let d8 = played.decisions(8);
    assert!(d8.orders.is_empty() && d8.commands.is_empty());
    assert!(played.held[&7].ships.contains(&ship(1)));
    assert!(played.held[&8].ships.is_empty());
    for day in 4..=16 {
        let d = played.decisions(day);
        assert!(d.orders.is_empty() && d.commands.is_empty(), "day {day}");
    }

    // The next run is needed on day 17 (the reading of day 19 is 290 t) and
    // is ship 1's: its load is 150 t by the sum and it carries the 360 t it
    // has aboard, with no load order.
    let d17 = played.decisions(17);
    let runs = add_runs(d17);
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].0, BTreeSet::from([ship(1)]));
    assert_eq!(runs[0].1, n(360));
    assert!(load_orders(d17).is_empty(), "no load order");
    assert!(d17.orders.is_empty());
    mentions(
        runs[0].3,
        &[
            "Without it the reading of day 19 is 290 t",
            "carries the 360 t it has aboard",
            "its load is 150 t by the sum",
            "no load order is needed",
        ],
    );
    // It sails on day 18.
    let d18 = played.decisions(18);
    assert_eq!(d18.orders.len(), 2);
    assert_eq!(d18.orders[0].kind, to_force(FORCE, route_to(&[T, S], &[T])));
    assert_eq!(d18.orders[1].kind, to_force(FORCE, TaskForceOrder::Unload));
    let state = played.scenario.world.command_state(Side::Allied);
    assert_eq!(state.plans[&plan(1)].runs.len(), 3);
    played.staff_never_orders_what_is_held();
}

#[test]
fn a_a_held_force_gives_the_same_days_as_a_held_ship() {
    // The force formed on day 1 held on day 8 in place of ship 1.
    let by_ship = play(&text_a(), &path_a());
    let commands = commands_of_a()
        .replace(
            "kind = \"hold\"\nship = 1",
            "kind = \"hold\"\ntask_force = 1000001",
        )
        .replace(
            "kind = \"release\"\nship = 1",
            "kind = \"release\"\ntask_force = 1000001",
        );
    assert_ne!(commands, commands_of_a());
    let by_force = play(&a_with(27, &commands), &path_a());
    by_force.nothing_refused();
    assert!(by_force.held[&8].forces.contains(&force(FORCE)));
    assert!(by_force.held[&8].ships.is_empty());
    assert!(by_force.held[&16].forces.contains(&force(FORCE)));
    assert!(by_force.held[&17].forces.is_empty(), "released on day 17");
    for day in 1..=27 {
        assert_eq!(
            bare(by_force.decisions(day)),
            bare(by_ship.decisions(day)),
            "day {day}"
        );
    }
    assert_eq!(by_force.truth, by_ship.truth);
    // The reason on day 10 names the hold of the force.
    let runs = add_runs(by_force.decisions(10));
    assert_eq!(runs[0].0, BTreeSet::from([ship(2)]));
    let words = whole(runs[0].3);
    assert!(
        words.contains("task_force:1000001 is held by the commander's hold of day 8"),
        "{words}"
    );
    by_force.staff_never_orders_what_is_held();
}

/// The commands of 41a to the day the hold is made, the change of floor,
/// and no release: for the games of the force of two.
fn two_ship_commands(ships: &str) -> String {
    format!(
        "{}{}{}{}",
        intent_and_limits(ships),
        command_table(3, "kind = \"approve\"\nplan = 1"),
        command_table(8, "kind = \"hold\"\nship = 1"),
        change_floor(),
    )
}

#[test]
fn a_a_held_ship_in_a_force_of_two_binds_and_a_third_ship_is_sent() {
    // Ships 1 and 2 lie in force 1 from the start. Ship 1 held on day 8: the
    // staff has no free ship on day 10 and says the hold is what binds.
    let text = format!(
        "{}\n{COMMANDS_MARKER}\n{}",
        head_with_force_of_two(11, false),
        two_ship_commands("[1, 2]")
    );
    let played = play(&text, &path_a());
    played.nothing_refused();
    // Day 1: the force goes as a whole.
    let runs = add_runs(played.decisions(1));
    assert_eq!(runs[0].0, BTreeSet::from([ship(1), ship(2)]));
    assert_eq!(runs[0].1, n(360));
    assert_eq!(played.decisions(1).orders[0].kind, load_to(1, 360));
    assert!(played.held[&8].ships.contains(&ship(1)));
    for day in [10, 11] {
        let d = played.decisions(day);
        assert!(d.orders.is_empty() && d.commands.is_empty(), "day {day}");
        let [reason] = &d.no_orders[..] else {
            panic!("one reason on day {day}")
        };
        let words = whole(reason);
        assert!(words.contains("no ship of the pool is free"), "{words}");
        assert!(
            words.contains("Without the hold, ship:1, ship:2 would have been sent."),
            "{words}"
        );
        assert!(
            reason
                .binding
                .iter()
                .any(|b| b.contains("the hold on ship:1")),
            "{:?}",
            reason.binding
        );
    }
    // Nothing was ordered to the force on those days.
    played.staff_never_orders_what_is_held();
    // The force did not move: it lies at S.
    assert_eq!(played.at(11).task_forces[&force(1)].position(), S);

    // A third ship in the pool, lying in no force, is sent.
    let text = format!(
        "{}\n{COMMANDS_MARKER}\n{}",
        head_with_force_of_two(11, true),
        two_ship_commands("[1, 2, 3]")
    );
    let played = play(&text, &path_a());
    played.nothing_refused();
    let d10 = played.decisions(10);
    let runs = add_runs(d10);
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].0, BTreeSet::from([ship(3)]));
    assert_eq!(runs[0].1, n(190));
    assert_eq!(d10.orders[0].kind, form(&[3]));
    assert_eq!(d10.orders[1].kind, load_to(FORCE, 190));
    let words = whole(runs[0].3);
    assert!(
        words.contains("Without the hold, ship:1, ship:2 would have been sent."),
        "{words}"
    );
    assert!(
        words.contains("ship:1 is held by the commander's hold of day 8"),
        "{words}"
    );
    assert!(runs[0]
        .3
        .binding
        .iter()
        .any(|b| b.contains("the hold on ship:1")));
    played.staff_never_orders_what_is_held();
}

/// 41a's head with an air group at base 1.
fn head_with_air_group(days: u32) -> String {
    let mut head = head_of(&text_a()).replace("days = 27", &format!("days = {days}"));
    head.push_str(
        "\n[[air_group]]\nid = 1\nname = \"Placeholder patrol squadron\"\nside = \"allied\"\nbase = 1\naircraft_type = 1\nready = 4\n",
    );
    head
}

#[test]
fn a_a_held_setting_is_recorded_shown_and_changes_no_order() {
    let without = format!(
        "{}\n{COMMANDS_MARKER}\n{}",
        head_with_air_group(12),
        commands_of_a_to(12)
    );
    let with = format!(
        "{without}{}",
        command_table(
            5,
            "kind = \"hold\"\nair_group = 1\nsetting = \"search_arc\""
        )
    );
    let before = play(&without, &path_a());
    let after = play(&with, &path_a());
    before.nothing_refused();
    after.nothing_refused();
    // The hold is recorded and shown by the command state from day 5.
    let setting_holds = |p: &Played, day: u32| p.held[&day].settings;
    assert_eq!(setting_holds(&before, 12), 0);
    assert_eq!(setting_holds(&after, 4), 0);
    for day in 5..=12 {
        assert_eq!(setting_holds(&after, day), 1, "day {day}");
    }
    let state = after.scenario.world.command_state(Side::Allied);
    let settings: Vec<_> = state
        .holds
        .iter()
        .filter_map(|(t, h)| match t {
            HoldTarget::Setting { setting, .. } => Some((*setting, h.day, h.by)),
            _ => None,
        })
        .collect();
    assert_eq!(
        settings,
        vec![(HeldSetting::SearchArc, 5, HoldMaker::Command)]
    );
    // It changes no order the supply staff gives, on any day.
    for day in 1..=12 {
        assert_eq!(
            bare(after.decisions(day)),
            bare(before.decisions(day)),
            "day {day}"
        );
    }
    assert_eq!(after.base_supply(12, 2), before.base_supply(12, 2));
    after.staff_never_orders_what_is_held();
}

/// A staff that hands in, besides what the doctrine gives, an order to the
/// force formed on day 1 on every day ship 1 is held.
struct Guarded;

impl Staff for Guarded {
    fn decide(&self, view: SideView<'_>) -> StaffDecisions {
        let mut decisions = SupplyStaff.decide(view);
        if view
            .command_state()
            .holds
            .contains_key(&HoldTarget::Ship(ship(1)))
        {
            decisions.orders.push(StaffOrder {
                kind: to_force(FORCE, TaskForceOrder::Unload),
                intent: IntentId::new(1),
                reason: Reason::default(),
            });
        }
        decisions
    }
}

#[test]
fn a_the_guard_refuses_a_staff_order_to_a_held_ships_force_with_the_hold_as_the_reason() {
    let mut scenario =
        common::from_text(&a_with(10, &commands_of_a_to(10)), &path_a()).expect("loads");
    let staff = Guarded;
    let mut refused_days = Vec::new();
    for day in 1..=10u32 {
        let summary = scenario
            .next_day_watched(Staffs::none().with(Side::Allied, &staff), &mut |_, _| {})
            .expect("a day resolves");
        assert!(summary.is_some());
        let rejected: Vec<_> = scenario
            .world
            .events()
            .iter()
            .filter(|e| e.day == day)
            .filter_map(|e| match &e.kind {
                EventKind::OrderRejected { reason, target, .. } => Some((reason.clone(), *target)),
                _ => None,
            })
            .collect();
        if !rejected.is_empty() {
            assert_eq!(rejected.len(), 1, "day {day}");
            let RejectReason::Held(target) = &rejected[0].0 else {
                panic!("day {day}: refused for another reason: {:?}", rejected[0].0)
            };
            // The hold on the ship in the force, or on the force: ship 1's.
            assert_eq!(*target, HoldTarget::Ship(ship(1)), "day {day}");
            refused_days.push(day);
        }
    }
    // Ship 1 is held from day 8: refused on days 8, 9 and 10, and never
    // before.
    assert_eq!(refused_days, vec![8, 9, 10]);
    // The refused order did nothing: ship 1's force is still at S.
    let state = scenario.world.umpire();
    assert_eq!(state.task_forces[&force(FORCE)].position(), S);
}

#[test]
fn a_every_game_of_41a_saves_and_the_staff_checks() {
    save_and_check(&path_a(), "41a");
    save_and_check(&path_b(), "41b");
    save_and_check(&path_c(), "41c");
}

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

/// The game saved and replayed, and the staff checked against what was saved.
fn save_and_check(scenario: &Path, label: &str) {
    let reference = common::frozen_root();
    let reference = reference.to_str().expect("a path");
    let dir = std::env::temp_dir().join(format!("flag_plot_s41_{label}_{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a directory");
    let save = dir.join("game.json");
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
        "run {label}: {}",
        String::from_utf8_lossy(&ran.stderr)
    );
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
        "replay {label}: {said} {}",
        String::from_utf8_lossy(&replayed.stderr)
    );
    assert!(
        said.contains("the staff gave what was saved on every day"),
        "{said}"
    );
    std::fs::remove_dir_all(&dir).ok();
}

// --------------------------------------------------------- Property of 41c

#[test]
fn c_cancelled_on_day_4_the_force_at_t_is_given_no_new_order() {
    let text = common::replace_once(
        &read(&path_c()),
        "day = 3\nside = \"allied\"\nkind = \"cancel_plan\"",
        "day = 4\nside = \"allied\"\nkind = \"cancel_plan\"",
    );
    let played = play_at(&text, &path_c(), FAR_T);
    played.nothing_refused();

    // The force arrived at T at hour 60 and landed 240 t in the 12 hours of
    // daylight left of day 3.
    assert_eq!(played.halted_at(3, FORCE, FAR_T), Some(n(60)));
    assert_eq!(played.unloaded(3), n(240));
    assert_eq!(played.at(3).task_forces[&force(FORCE)].position(), FAR_T);
    // Day 4: the plan is cancelled, and the force is given no new order.
    let d4 = played.decisions(4);
    assert!(d4.orders.is_empty(), "{:?}", d4.orders);
    let [reason] = &d4.no_orders[..] else {
        panic!("one reason")
    };
    assert!(
        reason.worked.contains("plan:1 is cancelled"),
        "{}",
        reason.worked
    );
    assert!(played.decisions(5).orders.is_empty());
    // It lands the other 130 t and comes home by the route it has.
    assert_eq!(played.unloaded(4), n(130));
    assert_eq!(played.unloading_complete(4), vec![n(72) + fx("6.5")]);
    let route = &played.at(5).task_forces[&force(FORCE)].route;
    assert_eq!(route.waypoints.last(), Some(&S));
    // 370 t were landed in all, at T.
    assert_eq!(played.base_supply(5, 2), n(100 - 50 + 370));
    played.staff_never_orders_what_is_held();
}
