//! Scenario 40: two supply intents and one pool. In 40a one ship serves two
//! bases: the higher intent has it, the lower's shortfall is stated with the
//! commitment that caused it, and a changed priority moves the ship when it
//! next comes free. In 40b two plans load from one stock on the same
//! morning and do not both count the same ton.
//!
//! Proves step 2 of decision 0107 (step 3, stage 1: priority, commitments
//! of stock, a second intent, displaced demand reported; rulings 10 to 16
//! there) against a hand calculation. The data is placeholder data: see
//! `data/scenarios/40a_one_pool_two_hexes.toml` and
//! `data/scenarios/40b_one_stock_two_plans.toml`. It runs on the frozen
//! step 1 figures. There is no enemy, no aircraft and no random draw.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
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
//! hour, and lighters at 20 t an hour; a size 0 port works by lighter at 20
//! t an hour. The placeholder fast transport with cargo (class 12): 30 t of
//! cargo, 16 kn at cruise, 500 t of fuel over 4000 nm, which is 0.125 t a
//! mile. A day is 24 hours: a night pulse of 12 and two daylight pulses of
//! 6. Day d begins at hour 24 x (d - 1). In a pulse ships move before cargo
//! is worked. The men's supply is paid at the start of the night pulse.
//! A ship in port takes the fuel it lacks in the pulse. A force that acts at a place
//! and steams on does so at the start of the first pulse after its work is
//! done (decision 0096).
//!
//! ## The rules the staff works by
//!
//! Those of scenario 36, and:
//!
//! - **Intents are served in priority order,** rank 1 first; at equal rank
//!   the lower intent number.
//! - **What the staff gives a higher intent in the morning is committed
//!   when it turns to the next:** the ships of a run it has just added, and
//!   the tons that run has still to load.
//! - **A free ship goes to the first intent, in that order, that needs a
//!   run today.** The staff does not keep a free ship idle for an intent
//!   that will need one later, and does not recall a run under way.
//! - **A ship is free** on the day after its commitment ends. A run's
//!   commitment is counted in whole days: the day it loads, the day it
//!   sails and arrives, a day to unload and a day back, so a ship ordered
//!   on day d is free on day d + 4 here.
//! - **An intent that needs a run and cannot have one names what displaced
//!   it:** its shortfall in days at the reading it checked, and the
//!   commitment that holds the ship or the tons (the plan, the run and the
//!   days).
//!
//! # 40a: one ship, two bases
//!
//! ## Inputs
//!
//! A placeholder map: S at (0,0), a coast hex with port size 1; T at (5,0)
//! and U at (0,5), coast hexes with port size 0. S to T and S to U are each
//! 5 hexes on one axis: 200 nm, 12.5 hours at cruise, 25 t of fuel. The
//! game is 16 days. Everything is Allied.
//!
//! | Base | Hex | Supply | Fuel   |
//! | ---- | --- | ------ | ------ |
//! | 1    | S   | 1000 t | 1000 t |
//! | 2    | T   | 10 t   | 0      |
//! | 3    | U   | 10 t   | 0      |
//!
//! Land unit 1, 200 men, at T on defend, and land unit 2, 200 men, at U on
//! defend. Each costs 1 t a day.
//!
//! Ship 1, a placeholder fast transport with cargo, lies in port at base 1
//! in no task force, full of fuel.
//!
//! The Allied side's commands:
//!
//! - Day 1. Intent 1: keep base 2 at a floor of 30 days, interval 15 days.
//!   Intent 2: keep base 3 at a floor of 30 days, interval 15 days. The
//!   floor is a stock of 30 t at each, and an interval's use is 15 t. The
//!   forces limit of each intent: ship 1, with base 1 as the source.
//!   Intent 1 has priority 1 and intent 2 priority 2. Leave to prepare and
//!   leave to launch are both given. No reserve is set.
//! - Day 3. Intent 2 is given priority 1, then intent 1 priority 2.
//!
//! ## A run, hour by hour
//!
//! A run ordered on day d: the load is aboard within the first hour of the
//! night of day d (30 t at 50 t an hour is 0.6 hours). On day d + 1 the
//! force sails at hour 24 x d, steams the 12 hours of the night and arrives
//! half an hour into the morning. It lands its load by lighter in the
//! morning (30 t in 1.5 hours), steams for S at the start of the afternoon,
//! and arrives at S 6.5 hours into the night of day d + 2, with 450 t of
//! fuel. It refuels 50 t in that pulse. It lies there on day d + 3 and is
//! free on day d + 4.
//!
//! ## Day by day
//!
//! Readings are at the start of the day, before its orders.
//!
//! | Day | Read at T | Read at U | The staff's orders | What happens |
//! | --- | --------- | --------- | ------------------ | ------------ |
//! | 1   | 10 t, 10 days | 10 t, 10 days | Intent 1: form a force of ship 1; load 30 t. Intent 2: none, displaced | Base 1 holds 970 t. T and U pay 1 t each, as on every night |
//! | 2   | 9 t       | 9 t       | Intent 1: route to T and back, acting at T, with an unload order. Intent 2: none, displaced | Sails at hour 24, arrives at hour 36.5, 30 t landed by hour 38, steams for S at hour 42 |
//! | 3   | 38 t      | 8 t       | none; intent 2 displaced | The priorities are changed. The force arrives at S at hour 54.5. Ship 1 refuels 50 t: base 1 holds 950 t of fuel |
//! | 4   | 37 t      | 7 t       | none; intent 2 displaced | |
//! | 5   | 36 t      | 6 t       | Intent 2: load 30 t. Intent 1: none, no run needed | Base 1 holds 940 t |
//! | 6   | 35 t      | 5 t       | Intent 2: route to U and back, acting at U, with an unload order | Sails at hour 120, arrives at hour 132.5, 30 t landed by hour 134, steams for S at hour 138 |
//! | 7   | 34 t      | 34 t      | none               | The force arrives at S at hour 150.5. Ship 1 refuels 50 t: 900 t of fuel at base 1 |
//! | 8   | 33 t      | 33 t      | none               | |
//! | 9   | 32 t      | 32 t      | none: ship 1 is free, and neither intent needs a run | |
//! | 10  | 31 t      | 31 t      | Intent 2: load 15 t. Intent 1: none, displaced | Base 1 holds 925 t |
//! | 11  | 30 t      | 30 t      | Intent 2: route to U and back, acting at U, with an unload order. Intent 1: none, displaced | Sails at hour 240, arrives at hour 252.5, 15 t landed by hour 253.25, steams for S at hour 258 |
//! | 12  | 29 t, 29 days | 44 t  | none; intent 1 displaced | The force arrives at S at hour 270.5. Ship 1 refuels 50 t: 850 t of fuel at base 1 |
//! | 13  | 28 t      | 43 t      | none; intent 1 displaced | |
//! | 14  | 27 t      | 42 t      | Intent 1: load 19 t. Intent 2: none, no run needed | Base 1 holds 906 t |
//! | 15  | 26 t, 26 days | 41 t  | Intent 1: route to T and back, acting at T, with an unload order | Sails at hour 336, arrives at hour 348.5, 19 t landed by hour 349.45, steams for S at hour 354 |
//! | 16  | 44 t      | 40 t      | none               | The force arrives at S at hour 366.5. Ship 1 refuels 50 t: 800 t of fuel at base 1 |
//!
//! Why each run, its size, and what is displaced:
//!
//! - **Day 1.** Intent 1 is first. A run ordered today arrives on day 2;
//!   without it the reading of day 3 at T is 8 t, below 30: a run is
//!   needed. The stock projected for the reading of day 2 is 9 t. The load
//!   is 30 - 9 + 15 = 36 t, cut to the ship's room of 30 t; the reason
//!   names the room as what cut it. Intent 2 needs a run by the same sum,
//!   and its one ship has just been given to intent 1: no ship is free
//!   until day 5. Its reason gives the shortfall, 30 - 8 = 22 days at the
//!   reading of day 3, and what displaced it: ship 1 committed to run 1 of
//!   intent 1's plan from day 1 to day 4.
//! - **Days 2 to 4.** Intent 2 is displaced on each day by the same
//!   commitment. On day d it checks the reading of day d + 2, which is 9 -
//!   d tons, and its shortfall is 21 + d days: 23, 24 and 25. The change
//!   of priority on day 3 recalls nothing: the run is under way.
//! - **Day 5.** Ship 1 is free. Intent 2 is now first. A run ordered today
//!   arrives on day 6; the reading of day 7 without it is 4 t: needed. The
//!   stock projected for day 6 is 5 t: 30 - 5 + 15 = 40 t, cut to 30 t by
//!   the room. Intent 1 needs none: the reading of day 7 at T is 34 t.
//! - **Day 9.** Ship 1 is free. A run ordered today would arrive on day
//!   10, and the reading of day 11 is 30 t at both bases, not below the
//!   floor: no run for either.
//! - **Day 10.** Both need a run: the reading of day 12 is 29 t at both.
//!   Intent 2 is first and has ship 1. The stock projected for day 11 is
//!   30 t: the load is 0 + 15 = 15 t. Intent 1 is displaced: no ship free
//!   until day 14, a shortfall of 1 day at the reading of day 12, by ship 1
//!   committed to run 2 of intent 2's plan from day 10 to day 13.
//! - **Days 11 to 13.** Intent 1 is displaced by the same commitment, with
//!   a shortfall of 2, 3 and 4 days, at the readings of days 13, 14 and
//!   15.
//! - **Day 14.** Ship 1 is free. Intent 2 needs none (the reading of day
//!   16 at U is 40 t). Intent 1: a run ordered today arrives on day 15, the
//!   reading of day 16 without it is 25 t: needed. The stock projected for
//!   day 15 is 26 t: the load is 30 - 26 + 15 = 19 t.
//!
//! At the end of day 16: base 2 holds 43 t (10 + 30 + 19 - 16) and base 3
//! holds 39 t (10 + 30 + 15 - 16). Base 1 holds 906 t of supply and 800 t
//! of fuel. Ship 1 is stopped at base 1, empty, with 500 t of fuel. Intent
//! 1's plan has made two runs, of 30 t and 19 t, ordered on days 1 and 14;
//! intent 2's plan two, of 30 t and 15 t, ordered on days 5 and 10. No
//! staff order was refused. The reading at T was below its floor on days 12
//! to 15, lowest on day 15 at 26 days; the reading at U was never below its
//! floor after day 6.
//!
//! The proposal for intent 1 asked for on day 10, before the day is ended,
//! says what it is displaced by: intent 2's run, ship 1, days 10 to 13.
//!
//! ## Properties of 40a
//!
//! - **Without the commands of day 3** the days to 9 are the same, and from
//!   day 10 T and U change places: intent 1 has ship 1 on day 10 with 15 t,
//!   intent 2 is displaced on days 10 to 13 and loads 19 t on day 14. At
//!   the end base 2 holds 39 t and base 3 holds 43 t.
//! - **With ship 1 in intent 1's protected limit** intent 2 is never given
//!   it, on any day, free or not: each day's reason for intent 2 says that
//!   no ship of its pool may be taken. Base 3 holds 0 t after the night of
//!   day 10 and its 200 men are unfed on the night of day 11.
//! - **At equal priority** (both intents with priority 1, or neither with
//!   any) the days are those of the first property: the lower intent
//!   number is first.
//! - The same start gives the same orders on every run, and a check of the
//!   saved game against the staff passes.
//! - Two worlds that differ only in what the Allied side cannot see give
//!   the same orders, commands, reasons and proposals.
//!
//! # 40b: one stock, two plans
//!
//! ## Inputs
//!
//! The map, bases 2 and 3 and the land units of 40a. Base 1 holds 1040 t of
//! supply and 1000 t of fuel. Ships 1 and 2, placeholder fast transports
//! with cargo, lie in port at base 1 in no task force, full of fuel. The
//! game is 5 days.
//!
//! Commands of day 1: intents 1 and 2 as in 40a, with priorities 1 and 2,
//! but intent 1's interval is 5 days; the forces limit of each is ships 1
//! and 2 with base 1 as the source; a reserve at base 1 of 1000 t of
//! supply; both leaves given.
//!
//! Corrected by the architect on Nov 15, 2026, after the build: the first
//! draft gave intent 1 an interval of 15 days and a load of 36 t, and had
//! it cut to ship 1's room. By decision 39 a run takes the free ships in
//! number order until their room covers the load, so that run took both
//! ships. Intent 1's interval is now 5 days, so that its load fits ship 1.
//!
//! ## Day by day
//!
//! - **Day 1.** Base 1 has 40 t above its reserve. Intent 1 is first: its
//!   load is 30 - 9 + 5 = 26 t, on ship 1, the first free ship, whose room
//!   covers it. The staff forms a force of ship 1. Intent 2: ship 2 is
//!   free. Its load is 30 - 9 + 15 = 36 t; the source has 40 t above its
//!   reserve, of which 26 t are still to be taken by intent 1's run: 14 t.
//!   It forms a force of ship 2 and orders a load of 14 t. Its reason names
//!   the source as what cut the load, with the three numbers: 1040 t of
//!   stock, the reserve of 1000 t, and 26 t still to be taken by run 1 of
//!   intent 1's plan. The two forces take the side's next two force
//!   numbers, ship 1's the lower.
//! - In the night ship 1's force has the berth and loads 26 t in 0.52
//!   hours; ship 2's force loads 14 t by lighter in 0.7 hours. Base 1
//!   holds 1000 t, its reserve, and never less on any day.
//! - **Day 2.** Both forces are routed, sail at hour 24 and arrive at hour
//!   36.5. T has 26 t landed by hour 37.8 and U 14 t by hour 37.2. Both
//!   steam for S at hour 42 and arrive at hour 54.5.
//! - **Readings.** At T: 10, 9, 34, 33, 32 t on days 1 to 5. At U: 10, 9,
//!   22, 21, 20 t.
//! - **Days 3 and 4.** Intent 1 needs no run (the readings it checks are
//!   32 t and 31 t). Intent 2 needs one (the reading it checks, of day 5
//!   and of day 6, is 20 t and 19 t) and has no ship free until day 5: its
//!   own run holds ship 2 and intent 1's holds ship 1. Its reason names
//!   both commitments.
//! - **Day 5.** Both ships are free. Intent 1 needs no run: the reading of
//!   day 7 is 30 t, not below the floor. Intent 2 needs one, the reading of
//!   day 7 being 18 t, a shortfall of 12 days; the source has nothing above
//!   its reserve, and its reason says so: 1000 t of stock, a reserve of
//!   1000 t, nothing still to be taken. It gives no order.
//!
//! At the end of day 5: base 1 holds 1000 t of supply and 900 t of fuel
//! (each ship refuelled 50 t on day 3, when it arrived); base 2 holds 31 t and base 3 holds
//! 19 t; both forces are stopped at base 1, empty.
//!
//! ## Property of 40b
//!
//! - With no reserve at base 1, intent 2's first load is 36 t cut to the
//!   30 t of ship 2's room, and with the reserve at 1014 t it is 0: intent
//!   2 gives no order on day 1 and names the source.

// ------------------------------------------------------------------ The tests
//
// As in scenario 36 the staff is asked as the engine asks it: each day
// through `next_day_watched(as_staffs(..))`, at the step after the
// commander's commands and the morning's refuelling. A watcher reads the
// figures at T and U at that moment (`Moment::Orders`), asks the staff twice
// on the same view, and asks for the proposal of each intent. What the staff
// handed in each day is read from `staff_handed_in`.

mod common;

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::process::Command;

use sim_core::choices::Moment;
use sim_core::command::{
    CommandKind, Committed, Displacement, PlanId, PlanState, Proposal, Reason, RunDays, RunSpec,
    StaffDecisions, StaffOrder,
};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, ShipId, TaskForceId};
use sim_core::orders::{BaseOrder, Cargo, OrderKind, TaskForceOrder};
use sim_core::state::{ShipLocation, Side, TaskForceStatus, TruthState};
use sim_core::{DaysOfSupply, Staff};
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs, SupplyStaff};

const S: Hex = Hex::new(0, 0);
const T: Hex = Hex::new(5, 0);
const U: Hex = Hex::new(0, 5);

/// The numbers the engine gives the forces formed on day 1: the first and
/// second of the Allied made series (decision 0086).
const FORCE: u32 = 1_000_001;
const FORCE_2: u32 = 1_000_002;

const COMMANDS_MARKER: &str = "# The Allied commander's commands";

const MAP: &str = "map/placeholder_one_pool_two_hexes.toml";

/// Which of the two scenario files.
#[derive(Clone, Copy)]
enum Which {
    A,
    B,
}

fn path(which: Which) -> PathBuf {
    let name = match which {
        Which::A => "scenarios/40a_one_pool_two_hexes.toml",
        Which::B => "scenarios/40b_one_stock_two_plans.toml",
    };
    common::data_dir().join(name)
}

fn text(which: Which) -> String {
    std::fs::read_to_string(path(which)).expect("the scenario file reads")
}

fn text_a() -> String {
    text(Which::A)
}

fn text_b() -> String {
    text(Which::B)
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
fn head(which: Which) -> String {
    let text = text(which);
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

fn days(text: &str, from: u32, count: u32) -> String {
    changed(text, &format!("days = {from}"), &format!("days = {count}"))
}

/// A command table as the scenario states it.
fn command_table(day: u32, body: &str) -> String {
    format!("\n[[command]]\nday = {day}\nside = \"allied\"\n{body}\n")
}

/// The Allied commander's commands, as the scenario files state them, with
/// the changes the tests make.
struct Commands {
    /// The ships of both forces limits, as TOML.
    ships: &'static str,
    /// A reserve at base 1, in tons of supply.
    reserve: Option<i64>,
    /// The ranks given on day 1, `(intent, rank)`.
    day_1_ranks: Vec<(u32, u32)>,
    /// A protected limit given on day 1, `(intent, ships as TOML)`.
    protected: Option<(u32, &'static str)>,
    /// Ranks given later, `(day, intent, rank)`.
    later_ranks: Vec<(u32, u32, u32)>,
    /// Intent 1's interval in days (intent 2's is 15).
    interval_1: u32,
}

impl Commands {
    /// The commands of scenario 40a as the file has them.
    fn a() -> Commands {
        Commands {
            ships: "[1]",
            reserve: None,
            day_1_ranks: vec![(1, 1), (2, 2)],
            protected: None,
            later_ranks: vec![(3, 2, 1), (3, 1, 2)],
            interval_1: 15,
        }
    }

    /// The commands of scenario 40b as the file has them.
    fn b() -> Commands {
        Commands {
            ships: "[1, 2]",
            reserve: Some(1000),
            day_1_ranks: vec![(1, 1), (2, 2)],
            protected: None,
            later_ranks: vec![],
            interval_1: 5,
        }
    }

    fn render(&self) -> String {
        let mut out = String::new();
        for (intent, base, interval) in [(1, 2, self.interval_1), (2, 3, 15)] {
            out += &command_table(
                1,
                &format!(
                    "kind = \"set_intent\"\nintent = {intent}\nbase = {base}\nfloor_days = 30\ninterval_days = {interval}"
                ),
            );
        }
        for intent in [1, 2] {
            out += &command_table(
                1,
                &format!(
                    "kind = \"set_limit\"\nlimit = \"forces\"\nintent = {intent}\nships = {}\ntask_forces = []\nsource = 1",
                    self.ships
                ),
            );
        }
        for (intent, rank) in &self.day_1_ranks {
            out += &command_table(
                1,
                &format!(
                    "kind = \"set_limit\"\nlimit = \"priority\"\nintent = {intent}\nrank = {rank}"
                ),
            );
        }
        if let Some((intent, held)) = self.protected {
            out += &command_table(
                1,
                &format!(
                    "kind = \"set_limit\"\nlimit = \"protected\"\nintent = {intent}\nships = {held}"
                ),
            );
        }
        if let Some(supply) = self.reserve {
            out += &command_table(
                1,
                &format!(
                    "kind = \"set_limit\"\nlimit = \"reserve\"\nbase = 1\nsupply_t = {supply}\nfuel_t = 0"
                ),
            );
        }
        out += &command_table(1, "kind = \"set_leave\"\nleave = \"prepare\"\ngiven = true");
        out += &command_table(1, "kind = \"set_leave\"\nleave = \"launch\"\ngiven = true");
        for (day, intent, rank) in &self.later_ranks {
            out += &command_table(
                *day,
                &format!(
                    "kind = \"set_limit\"\nlimit = \"priority\"\nintent = {intent}\nrank = {rank}"
                ),
            );
        }
        out
    }

    /// The scenario's head with these commands.
    fn on(&self, which: Which) -> String {
        format!(
            "{}{COMMANDS_MARKER}, in force from day 1.\n{}",
            head(which),
            self.render()
        )
    }
}

/// What the staff handed in each day, the figures it was asked on, the
/// truth at the end of each day, the log and the hash of each day.
struct Played {
    scenario: Scenario,
    /// The figures at T and at U at the staff's moment of each day.
    readings: BTreeMap<u32, (Option<DaysOfSupply>, Option<DaysOfSupply>)>,
    /// The proposal for each intent at the staff's moment of each day, by day
    /// and intent number.
    proposals: BTreeMap<(u32, u32), Option<Proposal>>,
    handed: BTreeMap<u32, StaffDecisions>,
    /// Whether asking the staff twice on the view gave one answer, and the
    /// answer handed in was that answer, and a proposal asked twice gave one.
    asked_twice_alike: bool,
    truth: Vec<TruthState>,
    hashes: Vec<u64>,
    events: Vec<Event>,
}

fn play(which: Which, text: &str) -> Played {
    play_scenario(common::from_text(text, &path(which)).expect("the scenario loads"))
}

fn play_scenario(mut scenario: Scenario) -> Played {
    let held = staffs();
    let mut readings = BTreeMap::new();
    let mut proposals = BTreeMap::new();
    let mut handed = BTreeMap::new();
    let mut truth = vec![scenario.world.umpire().clone()];
    let mut hashes = Vec::new();
    let mut asked_twice_alike = true;
    loop {
        let mut asked: Option<StaffDecisions> = None;
        let mut reading = None;
        let mut proposed: Vec<(u32, Option<Proposal>)> = Vec::new();
        let mut again = true;
        let day = scenario.world.day() + 1;
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |world, moment| {
                if moment == (Moment::Orders { day }) {
                    let view = world.side_view(Side::Allied);
                    reading = Some((view.days_of_supply(T), view.days_of_supply(U)));
                    let first = SupplyStaff.decide(view);
                    again = first == SupplyStaff.decide(view);
                    asked = Some(first);
                    for raw in [1, 2] {
                        let proposal = SupplyStaff.propose(view, intent(raw));
                        again &= proposal == SupplyStaff.propose(view, intent(raw));
                        proposed.push((raw, proposal));
                    }
                }
            })
            .expect("a day resolves");
        let Some(summary) = summary else { break };
        asked_twice_alike &= again;
        readings.insert(day, reading.expect("the staff's moment came"));
        for (raw, proposal) in proposed {
            proposals.insert((day, raw), proposal);
        }
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
        proposals,
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

    /// The figure at T at the staff's moment.
    fn at_t(&self, day: u32) -> DaysOfSupply {
        self.readings[&day].0.expect("a figure at T")
    }

    /// The figure at U at the staff's moment.
    fn at_u(&self, day: u32) -> DaysOfSupply {
        self.readings[&day].1.expect("a figure at U")
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

    /// Whether force `raw` steamed from `from` on `day` (it sails then).
    fn steamed_from(&self, day: u32, raw: u32, from: Hex) -> bool {
        !self
            .on_day(day, |k| {
                matches!(k, EventKind::TaskForceSteamed { task_force, from: f, .. }
                    if *task_force == force(raw) && *f == from)
            })
            .is_empty()
    }

    fn loading_complete(&self, day: u32) -> Vec<Fixed> {
        let mut found: Vec<Fixed> = self
            .on_day(day, |k| matches!(k, EventKind::LoadingComplete { .. }))
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::LoadingComplete { elapsed_hours, .. } => Some(*elapsed_hours),
                _ => None,
            })
            .collect();
        found.sort();
        found
    }

    fn unloading_complete(&self, day: u32) -> Vec<Fixed> {
        let mut found: Vec<Fixed> = self
            .on_day(day, |k| matches!(k, EventKind::UnloadingComplete { .. }))
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::UnloadingComplete { elapsed_hours, .. } => Some(*elapsed_hours),
                _ => None,
            })
            .collect();
        found.sort();
        found
    }

    /// The `(fed, unfed)` of each `MenFed` event of the Allied side at `hex`
    /// on `day`.
    fn fed_at(&self, day: u32, hex: Hex) -> Vec<(u32, u32)> {
        self.on_day(
            day,
            |k| matches!(k, EventKind::MenFed { side: Side::Allied, hex: h, .. } if *h == hex),
        )
        .iter()
        .map(|e| match &e.kind {
            EventKind::MenFed { fed, unfed, .. } => (*fed, *unfed),
            _ => (0, 0),
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

    /// The staff orders of one intent on one day.
    fn orders_of(&self, day: u32, raw: u32) -> Vec<&StaffOrder> {
        self.decisions(day)
            .orders
            .iter()
            .filter(|o| o.intent == intent(raw))
            .collect()
    }

    /// The no-order reasons of one intent on one day.
    fn no_orders_of(&self, day: u32, raw: u32) -> Vec<&Reason> {
        self.decisions(day)
            .no_orders
            .iter()
            .filter(|r| r.serves.intent == Some(intent(raw)))
            .collect()
    }

    /// The runs added on one day, with the plan they are added to and the
    /// reason.
    fn add_runs(&self, day: u32) -> Vec<(PlanId, &RunSpec, &Reason)> {
        self.decisions(day)
            .commands
            .iter()
            .filter_map(|c| match &c.kind {
                CommandKind::AddRun { plan, run } => Some((*plan, run, &c.reason)),
                _ => None,
            })
            .collect()
    }

    /// The ships committed on one day: plan, run, ships, from, to.
    fn commits(&self, day: u32) -> Vec<Committal> {
        self.decisions(day)
            .commands
            .iter()
            .filter_map(|c| match &c.kind {
                CommandKind::CommitShips {
                    plan,
                    run,
                    ships,
                    from_day,
                    to_day,
                } => Some((*plan, *run, ships.clone(), *from_day, *to_day)),
                _ => None,
            })
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
        stand_at: std::collections::BTreeSet::new(),
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

fn form(raw_ship: u32) -> OrderKind {
    OrderKind::Base {
        target: base(1),
        order: BaseOrder::FormTaskForce {
            ships: vec![ship(raw_ship)],
        },
    }
}

/// The four days of a run ordered on `day`: it loads that day, sails and
/// arrives the next, and its ship is free on the fourth day after.
fn run_days(day: u32) -> RunDays {
    RunDays {
        load: day,
        sail: day + 1,
        arrive: day + 1,
        free: day + 4,
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

/// A reason serves intent `raw` and its plan, the plan of the same number.
fn serves(reason: &Reason, raw: u32) {
    assert_eq!(reason.serves.intent, Some(intent(raw)));
    assert_eq!(reason.serves.plan, Some(plan(raw)), "{}", reason.worked);
    // Not asserted: the amendment. Giving a rank after the plan is approved
    // is an amendment, so a reason of day 3 on serves amendment 1.
    assert!(!reason.worked.is_empty());
    assert!(!reason.evidence.is_empty() || !reason.binding.is_empty());
}

/// A commitment of ships: plan, run, ships, from day, to day.
type Committal = (PlanId, Option<u32>, BTreeSet<ShipId>, u32, u32);

/// One run of 40a the staff is expected to order.
struct Run {
    day: u32,
    intent: u32,
    /// The run's number in its plan.
    number: u32,
    load: i64,
    to: Hex,
}

/// The staff's orders, commands and runs of the whole game are those of
/// `runs` and no others: each on its day, a load; on the next, a route and
/// an unload. Force 1 is formed from ship 1 on the first run only.
fn check_the_runs(played: &Played, runs: &[Run], last_day: u32) {
    for day in 1..=last_day {
        let mut expected_orders = 0;
        let mut expected_runs = 0;
        for run in runs {
            let raw = run.intent;
            if run.day == day {
                expected_runs += 1;
                let mut kinds = vec![];
                if run.day == runs[0].day {
                    kinds.push(form(1));
                }
                kinds.push(load_to(FORCE, run.load));
                let got: Vec<OrderKind> = played
                    .orders_of(day, raw)
                    .iter()
                    .map(|o| o.kind.clone())
                    .collect();
                assert_eq!(got, kinds, "orders of intent {raw}, day {day}");
                expected_orders += kinds.len();
                let adds = played.add_runs(day);
                assert_eq!(adds.len(), 1, "one run added, day {day}");
                assert_eq!(adds[0].0, plan(raw), "plan of the run, day {day}");
                assert_eq!(
                    adds[0].1,
                    &RunSpec {
                        ships: ships(&[1]),
                        load_t: n(run.load),
                        days: run_days(day),
                    },
                    "the run, day {day}"
                );
                serves_command(adds[0].2, raw);
                assert_eq!(
                    played.commits(day),
                    vec![(plan(raw), Some(run.number), ships(&[1]), day, day + 3)],
                    "the commitment, day {day}"
                );
            }
            if run.day + 1 == day {
                let got: Vec<OrderKind> = played
                    .orders_of(day, raw)
                    .iter()
                    .map(|o| o.kind.clone())
                    .collect();
                assert_eq!(
                    got,
                    vec![
                        to_force(FORCE, route_to(&[run.to, S], &[run.to])),
                        to_force(FORCE, TaskForceOrder::Unload),
                    ],
                    "orders of intent {raw}, day {day}"
                );
                expected_orders += 2;
            }
        }
        let d = played.decisions(day);
        assert_eq!(d.orders.len(), expected_orders, "orders, day {day}");
        assert_eq!(
            played.add_runs(day).len(),
            expected_runs,
            "runs added, day {day}"
        );
        if expected_runs == 0 {
            assert!(played.commits(day).is_empty(), "commitments, day {day}");
        }
        for order in &d.orders {
            serves(&order.reason, order.intent.raw());
        }
    }
}

/// A command's reason serves intent `raw` and its plan.
fn serves_command(reason: &Reason, raw: u32) {
    serves(reason, raw);
}

/// Each intent, each day, either gives orders or gives one reason for none.
fn each_intent_decides_once(played: &Played, last_day: u32) {
    for day in 1..=last_day {
        for raw in [1, 2] {
            let orders = played.orders_of(day, raw).len();
            let reasons = played.no_orders_of(day, raw);
            if orders > 0 {
                assert!(reasons.is_empty(), "intent {raw}, day {day}");
            } else {
                assert_eq!(reasons.len(), 1, "one reason of intent {raw}, day {day}");
                serves(reasons[0], raw);
                assert!(
                    !reasons[0].would_change.is_empty(),
                    "intent {raw}, day {day}"
                );
            }
        }
        assert_eq!(
            played.decisions(day).no_orders.len(),
            played
                .decisions(day)
                .no_orders
                .iter()
                .filter(|r| r.serves.intent.is_some())
                .count()
        );
    }
}

/// What the staff did, without the words: orders with their intents,
/// commands, and the intents that gave no order.
type Signature = Vec<(
    u32,
    Vec<(u32, OrderKind)>,
    Vec<CommandKind>,
    Vec<Option<IntentId>>,
)>;

fn signature(played: &Played) -> Signature {
    played
        .handed
        .iter()
        .map(|(day, d)| {
            (
                *day,
                d.orders
                    .iter()
                    .map(|o| (o.intent.raw(), o.kind.clone()))
                    .collect(),
                d.commands.iter().map(|c| c.kind.clone()).collect(),
                {
                    let mut intents: Vec<Option<IntentId>> =
                        d.no_orders.iter().map(|r| r.serves.intent).collect();
                    intents.sort();
                    intents
                },
            )
        })
        .collect()
}

/// The runs of the table, with intent 2 given the first rank on day 3.
fn runs_of_the_table() -> Vec<Run> {
    vec![
        Run {
            day: 1,
            intent: 1,
            number: 1,
            load: 30,
            to: T,
        },
        Run {
            day: 5,
            intent: 2,
            number: 1,
            load: 30,
            to: U,
        },
        Run {
            day: 10,
            intent: 2,
            number: 2,
            load: 15,
            to: U,
        },
        Run {
            day: 14,
            intent: 1,
            number: 2,
            load: 19,
            to: T,
        },
    ]
}

/// The runs without the commands of day 3: T and U change places from day 10.
fn runs_without_day_3() -> Vec<Run> {
    vec![
        Run {
            day: 1,
            intent: 1,
            number: 1,
            load: 30,
            to: T,
        },
        Run {
            day: 5,
            intent: 2,
            number: 1,
            load: 30,
            to: U,
        },
        Run {
            day: 10,
            intent: 1,
            number: 2,
            load: 15,
            to: T,
        },
        Run {
            day: 14,
            intent: 2,
            number: 2,
            load: 19,
            to: U,
        },
    ]
}

// ------------------------------------------------------- The hand calculation, 40a

#[test]
fn the_scenario_files_state_the_commands_these_tests_vary() {
    for (which, commands) in [(Which::A, Commands::a()), (Which::B, Commands::b())] {
        let file = text(which);
        let rest = file.split(COMMANDS_MARKER).nth(1).expect("the commands");
        let tables: String = rest.lines().skip(1).collect::<Vec<_>>().join("\n");
        assert_eq!(tables.trim(), commands.render().trim());
    }
}

#[test]
fn the_map_has_five_hexes_from_s_to_t_and_u() {
    let map = std::fs::read_to_string(common::data_dir().join(MAP)).expect("the map reads");
    assert!(map.contains("PLACEHOLDER MAP"));
    assert_eq!(S.distance(T), 5);
    assert_eq!(S.distance(U), 5);
    // The engine's own route is 5 hexes, 200 nm, one way.
    let played = play(Which::A, &days(&text_a(), 16, 3));
    let state = played.at(2);
    let formed = &state.task_forces[&force(FORCE)];
    assert_eq!(formed.route.waypoints.first(), Some(&T));
    let steamed = played.all(
        |k| matches!(k, EventKind::TaskForceSteamed { from, to, .. } if *from == S || *to == T),
    );
    let miles = steamed
        .iter()
        .map(|e| match &e.kind {
            EventKind::TaskForceSteamed { distance_nm, .. } => *distance_nm,
            _ => Fixed::ZERO,
        })
        .fold(Fixed::ZERO, |sum, d| sum + d);
    assert!(miles >= n(200), "{miles} nm steamed to T");
}

#[test]
fn the_readings_at_t_and_u_follow_the_table() {
    let played = play(Which::A, &text_a());
    let t: [i64; 16] = [
        10, 9, 38, 37, 36, 35, 34, 33, 32, 31, 30, 29, 28, 27, 26, 44,
    ];
    let u: [i64; 16] = [10, 9, 8, 7, 6, 5, 34, 33, 32, 31, 30, 44, 43, 42, 41, 40];
    for day in 1..=16u32 {
        for (got, stock, label) in [
            (played.at_t(day), t[day as usize - 1], "T"),
            (played.at_u(day), u[day as usize - 1], "U"),
        ] {
            assert_eq!(got.stock_t, n(stock), "stock at {label}, day {day}");
            assert_eq!(got.days, Some(n(stock)), "days at {label}, day {day}");
            assert_eq!(got.planned_use.total_t(), n(1), "use at {label}, day {day}");
        }
    }
    // The floor is 30 days: T is below it on days 12 to 15, lowest on day 15;
    // U is never below it after day 6.
    for day in 3..=16u32 {
        let days_at_t = played.at_t(day).days.expect("a figure");
        assert_eq!(days_at_t < n(30), (12..=15).contains(&day), "T, day {day}");
    }
    let lowest = (3..=16u32)
        .map(|day| (played.at_t(day).days.unwrap(), day))
        .min()
        .unwrap();
    assert_eq!(lowest, (n(26), 15));
    for day in 7..=16u32 {
        assert!(played.at_u(day).days.unwrap() >= n(30), "U, day {day}");
    }
    assert!(played.asked_twice_alike, "the staff answers one view alike");
}

#[test]
fn the_staffs_orders_follow_the_table_day_by_day() {
    let played = play(Which::A, &text_a());
    played.nothing_refused();
    check_the_runs(&played, &runs_of_the_table(), 16);
    each_intent_decides_once(&played, 16);

    // The days of no order at all: 7, 8 and 9, and 16: both intents give a
    // reason and nothing else.
    for day in [7, 8, 9, 16] {
        let d = played.decisions(day);
        assert!(d.orders.is_empty() && d.commands.is_empty(), "day {day}");
        assert_eq!(d.no_orders.len(), 2, "day {day}");
    }
    // The forces: one, formed on day 1, loaded and sent four times. Nothing
    // else is formed.
    let forms: Vec<u32> = played
        .orders()
        .iter()
        .filter(|(_, o)| matches!(o.kind, OrderKind::Base { .. }))
        .map(|(day, _)| *day)
        .collect();
    assert_eq!(forms, vec![1]);
    // Every order is accepted: thirteen in all: four loads, the one forming,
    // four routes and four unloads.
    let accepted = played.all(|k| matches!(k, EventKind::OrderAccepted { .. }));
    assert_eq!(accepted.len(), 13);
}

#[test]
fn the_plans_hold_the_runs_the_table_gives() {
    let played = play(Which::A, &text_a());
    let state = played.scenario.world.command_state(Side::Allied);
    for (raw, runs) in [
        (1u32, vec![(1u32, 30i64, 1u32), (2, 19, 14)]),
        (2, vec![(1, 30, 5), (2, 15, 10)]),
    ] {
        let made = &state.plans[&plan(raw)];
        assert_eq!(made.intent, intent(raw));
        assert_eq!(made.state, PlanState::Released);
        assert_eq!(made.runs.len(), 2);
        for (number, load, day) in runs {
            let run = &made.runs[&number];
            assert_eq!(run.load_t, n(load), "plan {raw} run {number}");
            assert_eq!(run.ships, ships(&[1]));
            assert_eq!(run.source, base(1));
            assert_eq!(run.days, run_days(day), "plan {raw} run {number}");
        }
    }
    assert_eq!(state.plans.len(), 2);
}

#[test]
fn every_run_has_a_reason_with_the_numbers_of_the_table() {
    let played = play(Which::A, &text_a());
    // Day 1: 30 - 9 + 15 = 36 t, cut to the ship's room of 30 t.
    let (_, _, reason) = played.add_runs(1)[0];
    mentions(reason, &["36", "30", "9", "15"]);
    assert!(
        reason.worked.to_lowercase().contains("room"),
        "the room cut it: {}",
        reason.worked
    );
    // Day 5: 30 - 5 + 15 = 40 t, cut to 30 t.
    let (_, _, reason) = played.add_runs(5)[0];
    mentions(reason, &["40", "30", "5", "15"]);
    // Day 10: 0 + 15 = 15 t, the stock projected for day 11 being 30 t.
    let (_, _, reason) = played.add_runs(10)[0];
    mentions(reason, &["15", "30"]);
    // Day 14: 30 - 26 + 15 = 19 t.
    let (_, _, reason) = played.add_runs(14)[0];
    mentions(reason, &["19", "26", "30", "15"]);
    // Each order carries the same arithmetic as its run.
    for (day, order) in played.orders() {
        assert!(!order.reason.worked.is_empty(), "day {day}");
        assert!(
            !order.reason.evidence.is_empty() || !order.reason.binding.is_empty(),
            "day {day}"
        );
    }
}

#[test]
fn what_happens_follows_the_table_exactly() {
    let played = play(Which::A, &text_a());
    played.nothing_refused();

    // (day ordered, destination, loaded at, halted at, unloaded at, the tons,
    // arrives at S at, the day it does).
    let table = [
        (1u32, T, "0.6", "36.5", "38", 30i64, "54.5", 3u32),
        (5, U, "96.6", "132.5", "134", 30, "150.5", 7),
        (10, U, "216.3", "252.5", "253.25", 15, "270.5", 12),
        (14, T, "312.38", "348.5", "349.45", 19, "366.5", 16),
    ];
    for (day, to, loaded, halted, unloaded, tons, home, home_day) in table {
        assert_eq!(
            played.loading_complete(day),
            vec![fx(loaded)],
            "loading, day {day}"
        );
        // It sails at the start of the next day and halts at its destination.
        assert!(
            played.steamed_from(day + 1, FORCE, S),
            "sails, day {}",
            day + 1
        );
        let ashore = day + 1;
        assert_eq!(
            played.halted_at(ashore, FORCE, to),
            Some(fx(halted)),
            "halted, day {ashore}"
        );
        assert_eq!(played.unloaded(ashore), n(tons), "unloaded, day {ashore}");
        assert_eq!(
            played.unloading_complete(ashore),
            vec![fx(unloaded)],
            "unloading complete, day {ashore}"
        );
        assert_eq!(
            played.arrived_at(home_day, FORCE, S),
            Some(fx(home)),
            "home, day {home_day}"
        );
    }

    // Base 1's supply: 970 t from day 1, 940 t from day 5, 925 t from day 10,
    // 906 t from day 14.
    for day in 1..=16u32 {
        let supply = match day {
            1..=4 => 970,
            5..=9 => 940,
            10..=13 => 925,
            _ => 906,
        };
        assert_eq!(played.base_supply(day, 1), n(supply), "supply, day {day}");
    }
    // Its fuel: ship 1 refuels 50 t on days 3, 7, 12 and 16, each in the
    // pulse it arrives (decision 0112).
    for day in 1..=16u32 {
        let fuel = match day {
            1..=2 => 1000,
            3..=6 => 950,
            7..=11 => 900,
            12..=15 => 850,
            _ => 800,
        };
        assert_eq!(played.base_fuel(day, 1), n(fuel), "fuel, day {day}");
    }
    // Ship 1 comes home with 450 t and refuels to 500 t in that pulse.
    for (day, fuel) in [(3, 500), (7, 500), (12, 500), (16, 500)] {
        assert_eq!(
            played.at(day).ships[&ship(1)].fuel_t,
            n(fuel),
            "ship 1, day {day}"
        );
    }
    // The bases: T and U each pay 1 t a night; what lands shows from the day.
    let t_end: [i64; 16] = [
        9, 38, 37, 36, 35, 34, 33, 32, 31, 30, 29, 28, 27, 26, 44, 43,
    ];
    let u_end: [i64; 16] = [9, 8, 7, 6, 5, 34, 33, 32, 31, 30, 44, 43, 42, 41, 40, 39];
    for day in 1..=16u32 {
        assert_eq!(
            played.base_supply(day, 2),
            n(t_end[day as usize - 1]),
            "T, day {day}"
        );
        assert_eq!(
            played.base_supply(day, 3),
            n(u_end[day as usize - 1]),
            "U, day {day}"
        );
    }
    // Landed on the day after each run is ordered.
    assert_eq!(
        [2, 6, 11, 15].map(|day| played.unloaded(day)),
        [n(30), n(30), n(15), n(19)]
    );

    // The end of day 16.
    assert_eq!(played.base_supply(16, 2), n(43));
    assert_eq!(played.base_supply(16, 3), n(39));
    assert_eq!(played.base_supply(16, 1), n(906));
    assert_eq!(played.base_fuel(16, 1), n(800));
    let end = played.at(16);
    let one = &end.ships[&ship(1)];
    assert_eq!(one.location, ShipLocation::TaskForce(force(FORCE)));
    assert!(one.supply_t.is_zero() && one.cargo_fuel_t.is_zero());
    assert_eq!(one.fuel_t, n(500));
    let formed = &end.task_forces[&force(FORCE)];
    assert_eq!(formed.status, TaskForceStatus::Arrived);
    assert_eq!(formed.from_hex, S);
    assert_eq!(formed.position(), S);
}

#[test]
fn the_ship_is_in_the_pool_of_both_intents_and_never_idles_for_one() {
    // Ship 1 is committed from day 1 to 4, 5 to 8, 10 to 13 and 14 to 17 and
    // on no other day; the commitments are to the plans in the table.
    let played = play(Which::A, &text_a());
    let mut given: Vec<(u32, u32, u32, u32)> = Vec::new();
    for day in 1..=16 {
        for (plan, run, ships_, from, to) in played.commits(day) {
            assert_eq!(ships_, ships(&[1]));
            given.push((plan.raw(), run.expect("a run"), from, to));
        }
    }
    assert_eq!(
        given,
        vec![(1, 1, 1, 4), (2, 1, 5, 8), (2, 2, 10, 13), (1, 2, 14, 17)]
    );
}

// ----------------------------------------------------------- What is displaced

#[test]
fn the_reasons_of_the_displaced_intent_name_the_shortfall_and_the_commitment() {
    let played = play(Which::A, &text_a());
    // Days 1 to 4: intent 2 is displaced by ship 1 on run 1 of plan 1, days 1
    // to 4, by 22, 23, 24 and 25 days at the readings of days 3 to 6.
    for (day, short) in [(1u32, 22u32), (2, 23), (3, 24), (4, 25)] {
        let reasons = played.no_orders_of(day, 2);
        assert_eq!(reasons.len(), 1, "day {day}");
        let reason = reasons[0];
        serves(reason, 2);
        assert_eq!(reason.serves.plan, Some(plan(2)));
        mentions(
            reason,
            &[
                &format!("Short {short} {}", if short == 1 { "day" } else { "days" }),
                &format!("day {}", day + 2),
                "ship:1",
                "run 1 of plan:1",
                "day 1 to day 4",
            ],
        );
        assert!(!reason.would_change.is_empty(), "day {day}");
    }
    // Days 10 to 13: intent 1 is displaced by ship 1 on run 2 of plan 2, days
    // 10 to 13, by 1, 2, 3 and 4 days at the readings of days 12 to 15.
    for (day, short) in [(10u32, 1u32), (11, 2), (12, 3), (13, 4)] {
        let reasons = played.no_orders_of(day, 1);
        assert_eq!(reasons.len(), 1, "day {day}");
        let reason = reasons[0];
        serves(reason, 1);
        assert_eq!(reason.serves.plan, Some(plan(1)));
        mentions(
            reason,
            &[
                &format!("Short {short} {}", if short == 1 { "day" } else { "days" }),
                &format!("day {}", day + 2),
                "ship:1",
                "run 2 of plan:2",
                "day 10 to day 13",
            ],
        );
        assert!(!reason.would_change.is_empty(), "day {day}");
    }
    // Neither intent names a commitment on the other days.
    for day in 1..=16u32 {
        for raw in [1, 2] {
            let displaced = played
                .no_orders_of(day, raw)
                .iter()
                .any(|r| r.worked.contains("is displaced by"));
            let expected = (raw == 2 && day <= 4) || (raw == 1 && (10..=13).contains(&day));
            assert_eq!(displaced, expected, "intent {raw}, day {day}");
        }
    }
}

#[test]
fn the_proposal_for_intent_1_on_day_10_says_what_displaces_it() {
    let played = play(Which::A, &text_a());
    let displacement = Displacement {
        intent: intent(2),
        plan: plan(2),
        run: 2,
        committed: Committed::Ships(ships(&[1])),
        from_day: 10,
        to_day: 13,
    };
    let proposal = played.proposals[&(10, 1)].as_ref().expect("a proposal");
    assert_eq!(proposal.intent, intent(1));
    assert_eq!(proposal.plan, plan(1));
    let shortfall = proposal.shortfall.as_ref().expect("a shortfall");
    assert_eq!(shortfall.shortfall_days, n(1));
    assert_eq!(shortfall.check_day, 12);
    assert_eq!(shortfall.displaced_by, vec![displacement.clone()]);
    assert!(proposal.runs.is_empty(), "no run can be sent today");
    // Intent 2's proposal says whom it displaces, with the same displacement.
    let second = played.proposals[&(10, 2)].as_ref().expect("a proposal");
    assert_eq!(second.displaces.len(), 1);
    let displaced = &second.displaces[0];
    assert_eq!(displaced.intent, intent(1));
    assert_eq!(displaced.plan, plan(1));
    assert_eq!(displaced.shortfall.displaced_by, vec![displacement]);
    assert_eq!(displaced.shortfall.check_day, 12);
    assert_eq!(displaced.shortfall.shortfall_days, n(1));
    // Intent 2 itself is not displaced by anything.
    assert!(second.shortfall.is_none());
    assert!(proposal.displaces.is_empty());
}

// ---------------------------------------------------------------- Properties of 40a

#[test]
fn without_the_commands_of_day_3_t_and_u_change_places_from_day_10() {
    let source = Commands {
        later_ranks: vec![],
        ..Commands::a()
    }
    .on(Which::A);
    let played = play(Which::A, &source);
    played.nothing_refused();
    let reference = play(Which::A, &text_a());
    // The days to 9 are the same.
    for day in 1..=9 {
        assert_eq!(
            signature(&played).iter().find(|s| s.0 == day),
            signature(&reference).iter().find(|s| s.0 == day),
            "day {day}"
        );
        assert_eq!(played.readings[&day], reference.readings[&day], "day {day}");
    }
    check_the_runs(&played, &runs_without_day_3(), 16);
    each_intent_decides_once(&played, 16);
    // Intent 2 is displaced on days 10 to 13, by run 2 of plan 1.
    for (day, short) in [(10u32, 1u32), (11, 2), (12, 3), (13, 4)] {
        let reason = played.no_orders_of(day, 2)[0];
        mentions(
            reason,
            &[
                &format!("Short {short} {}", if short == 1 { "day" } else { "days" }),
                &format!("day {}", day + 2),
                "ship:1",
                "run 2 of plan:1",
                "day 10 to day 13",
            ],
        );
    }
    // The end: base 2 holds 39 t, base 3 holds 43 t.
    assert_eq!(played.base_supply(16, 2), n(39));
    assert_eq!(played.base_supply(16, 3), n(43));
    assert_eq!(played.base_supply(16, 1), n(906));
    assert_eq!(played.base_fuel(16, 1), n(800));
}

#[test]
fn with_ship_1_in_intent_1s_protected_limit_intent_2_never_has_it() {
    let source = Commands {
        protected: Some((1, "[1]")),
        ..Commands::a()
    }
    .on(Which::A);
    let played = play(Which::A, &source);
    played.nothing_refused();
    // Intent 2 gives no order and no run on any day, and says each day that
    // no ship of its pool may be taken.
    for day in 1..=16u32 {
        assert!(played.orders_of(day, 2).is_empty(), "day {day}");
        assert!(
            played.add_runs(day).iter().all(|(p, _, _)| *p == plan(1)),
            "day {day}"
        );
        assert!(
            played.commits(day).iter().all(|c| c.0 == plan(1)),
            "day {day}"
        );
        let reasons = played.no_orders_of(day, 2);
        assert_eq!(reasons.len(), 1, "day {day}");
        let all = format!("{} {}", reasons[0].worked, reasons[0].binding.join(" ")).to_lowercase();
        assert!(
            all.contains("no ship of the pool of intent:2 may be taken")
                || all.contains("no ship of the pool may be taken"),
            "day {day}: {all}"
        );
    }
    // Base 3 holds 0 t after the night of day 10 and its 200 men are unfed on
    // the night of day 11; they were fed on every night before.
    for day in 1..=10 {
        assert_eq!(
            played.base_supply(day, 3),
            n(10 - i64::from(day)),
            "day {day}"
        );
        assert_eq!(played.fed_at(day, U), vec![(200, 0)], "fed on day {day}");
    }
    assert!(played.base_supply(10, 3).is_zero());
    assert_eq!(played.fed_at(11, U), vec![(0, 200)]);
    assert!(played.base_supply(11, 3).is_zero());
    // Intent 1's runs are made by ship 1 and intent 1's first run is that of
    // the table.
    assert_eq!(played.add_runs(1)[0].0, plan(1));
}

#[test]
fn at_equal_priority_the_days_are_those_of_the_first_property() {
    let first = Commands {
        later_ranks: vec![],
        ..Commands::a()
    }
    .on(Which::A);
    let reference = play(Which::A, &first);
    for (label, commands) in [
        (
            "both rank 1",
            Commands {
                day_1_ranks: vec![(1, 1), (2, 1)],
                later_ranks: vec![],
                ..Commands::a()
            },
        ),
        (
            "neither ranked",
            Commands {
                day_1_ranks: vec![],
                later_ranks: vec![],
                ..Commands::a()
            },
        ),
    ] {
        let played = play(Which::A, &commands.on(Which::A));
        played.nothing_refused();
        assert_eq!(signature(&played), signature(&reference), "{label}");
        check_the_runs(&played, &runs_without_day_3(), 16);
        assert_eq!(played.base_supply(16, 2), n(39), "{label}");
        assert_eq!(played.base_supply(16, 3), n(43), "{label}");
    }
}

#[test]
fn the_same_start_gives_the_same_orders_on_every_run() {
    for which in [Which::A, Which::B] {
        let first = play(which, &text(which));
        let second = play(which, &text(which));
        assert_eq!(first.handed, second.handed);
        assert_eq!(first.hashes, second.hashes);
        assert_eq!(first.readings, second.readings);
        assert_eq!(first.proposals, second.proposals);
        assert!(first.asked_twice_alike, "the staff answers one view alike");
    }
}

#[test]
fn two_worlds_that_differ_only_in_what_the_allied_side_cannot_see_give_the_same_decisions() {
    // A third coast hex far from everything, with a Japanese base, a ship and
    // a unit on it.
    let map_text = std::fs::read_to_string(common::data_dir().join(MAP)).expect("the map reads")
        + "\n[[hex]]\nq = 7\nr = 7\nkind = \"coast\"\nport_size = 1\n";
    let dir = std::env::temp_dir().join(format!("flag_plot_s40_{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a directory");
    let map_path = dir.join("map.toml");
    std::fs::write(&map_path, map_text).expect("the map is written");
    let enemy = |supply: u32, fuel: u32, men: u32, ship_fuel: u32| {
        format!(
            r#"
[[base]]
id = 7
name = "Placeholder base V, Japanese"
side = "japanese"
q = 7
r = 7
supply_t = {supply}
fuel_t = {fuel}

[[land_unit]]
id = 9
name = "Placeholder unit 9"
side = "japanese"
men = {men}
q = 7
r = 7
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
    for which in [Which::A, Which::B] {
        let on_the_new_map = |text: &str| {
            changed(
                text,
                &format!("map = \"../{MAP}\""),
                &format!("map = \"{}\"", map_path.display()),
            )
        };
        let plain = on_the_new_map(&text(which));
        let one = format!("{}{}", plain, enemy(900, 400, 1500, 500));
        let two = format!("{}{}", plain, enemy(10, 5, 4000, 900));
        let alone = play(which, &plain);
        let with_one = play(which, &one);
        let with_two = play(which, &two);
        let last = alone.handed.len() as u32;
        for day in 1..=last {
            assert_eq!(alone.handed[&day], with_one.handed[&day], "day {day}");
            assert_eq!(alone.handed[&day], with_two.handed[&day], "day {day}");
            assert_eq!(alone.readings[&day], with_one.readings[&day]);
            assert_eq!(alone.readings[&day], with_two.readings[&day]);
            for raw in [1, 2] {
                assert_eq!(
                    alone.proposals[&(day, raw)],
                    with_one.proposals[&(day, raw)],
                    "proposal {raw}, day {day}"
                );
                assert_eq!(
                    alone.proposals[&(day, raw)],
                    with_two.proposals[&(day, raw)],
                    "proposal {raw}, day {day}"
                );
            }
        }
        assert_eq!(last, if matches!(which, Which::A) { 16 } else { 5 });
    }
    std::fs::remove_dir_all(&dir).ok();
}

// ------------------------------------------------------------------ 40b

#[test]
fn on_day_1_two_plans_load_from_one_stock_and_do_not_count_a_ton_twice() {
    let played = play(Which::B, &text_b());
    played.nothing_refused();
    let d1 = played.decisions(1);

    // Intent 1 forms ship 1 into the first force and loads 26 t.
    let one: Vec<OrderKind> = played
        .orders_of(1, 1)
        .iter()
        .map(|o| o.kind.clone())
        .collect();
    assert_eq!(one, vec![form(1), load_to(FORCE, 26)]);
    // Intent 2 forms ship 2 into the second and loads 14 t.
    let two: Vec<OrderKind> = played
        .orders_of(1, 2)
        .iter()
        .map(|o| o.kind.clone())
        .collect();
    assert_eq!(two, vec![form(2), load_to(FORCE_2, 14)]);
    assert!(d1.no_orders.is_empty());

    // The runs: plan 1 loads 26 t in ship 1, plan 2 14 t in ship 2.
    let adds = played.add_runs(1);
    assert_eq!(adds.len(), 2);
    assert_eq!(adds[0].0, plan(1));
    assert_eq!(
        adds[0].1,
        &RunSpec {
            ships: ships(&[1]),
            load_t: n(26),
            days: run_days(1),
        }
    );
    assert_eq!(adds[1].0, plan(2));
    assert_eq!(
        adds[1].1,
        &RunSpec {
            ships: ships(&[2]),
            load_t: n(14),
            days: run_days(1),
        }
    );
    assert_eq!(
        played.commits(1),
        vec![
            (plan(1), Some(1), ships(&[1]), 1, 4),
            (plan(2), Some(1), ships(&[2]), 1, 4),
        ]
    );
    serves(adds[0].2, 1);
    serves(adds[1].2, 2);

    // Intent 2's reason names the source with its three numbers: 1040 t of
    // stock, a reserve of 1000 t, 26 t still to be taken by run 1 of intent
    // 1's plan.
    let load = played.orders_of(1, 2)[1];
    for reason in [adds[1].2, &load.reason] {
        mentions(
            reason,
            &["1040 t", "1000 t", "26 t still to be taken", "36"],
        );
        assert!(reason.binding.iter().any(|b| b == "displaced by plan:1"));
    }
    assert!(
        adds[1].2.worked.contains("run 1 of plan:1")
            || load.reason.worked.contains("run 1 of plan:1"),
        "{} / {}",
        adds[1].2.worked,
        load.reason.worked
    );
    // Intent 1's load is 30 - 9 + 5 = 26 t, on ship 1, uncut.
    mentions(adds[0].2, &["Load = max(0, 30 - 9) + 5", "= 26 t"]);
    assert!(!adds[0].2.worked.contains("It is cut"));

    // In the night: 26 t by the berth in 0.52 hours, 14 t by lighter in 0.7
    // hours. Base 1 holds 1000 t, its reserve, and never less.
    let mut hours = played.loading_complete(1);
    hours.sort();
    assert_eq!(hours, vec![fx("0.52"), fx("0.7")]);
    for day in 1..=5 {
        assert_eq!(played.base_supply(day, 1), n(1000), "day {day}");
    }
}

#[test]
fn in_40b_the_forces_sail_land_and_come_home_at_the_hours_of_the_table() {
    let played = play(Which::B, &text_b());
    played.nothing_refused();
    // Day 2: both are routed and unloaded.
    let one: Vec<OrderKind> = played
        .orders_of(2, 1)
        .iter()
        .map(|o| o.kind.clone())
        .collect();
    assert_eq!(
        one,
        vec![
            to_force(FORCE, route_to(&[T, S], &[T])),
            to_force(FORCE, TaskForceOrder::Unload),
        ]
    );
    let two: Vec<OrderKind> = played
        .orders_of(2, 2)
        .iter()
        .map(|o| o.kind.clone())
        .collect();
    assert_eq!(
        two,
        vec![
            to_force(FORCE_2, route_to(&[U, S], &[U])),
            to_force(FORCE_2, TaskForceOrder::Unload),
        ]
    );
    assert!(played.steamed_from(2, FORCE, S));
    assert!(played.steamed_from(2, FORCE_2, S));
    // Sail at hour 24, arrive at hour 36.5; T has 26 t landed by hour 37.8
    // and U 14 t by hour 37.2; both steam for S at hour 42 and arrive at hour
    // 54.5.
    assert_eq!(played.halted_at(2, FORCE, T), Some(fx("36.5")));
    assert_eq!(played.halted_at(2, FORCE_2, U), Some(fx("36.5")));
    assert_eq!(played.unloaded(2), n(40));
    let mut landed = played.unloading_complete(2);
    landed.sort();
    assert_eq!(landed, vec![fx("37.2"), fx("37.8")]);
    assert_eq!(played.arrived_at(3, FORCE, S), Some(fx("54.5")));
    assert_eq!(played.arrived_at(3, FORCE_2, S), Some(fx("54.5")));

    // Readings at T and U on days 1 to 5.
    for (day, t, u) in [
        (1u32, 10i64, 10i64),
        (2, 9, 9),
        (3, 34, 22),
        (4, 33, 21),
        (5, 32, 20),
    ] {
        assert_eq!(played.at_t(day).stock_t, n(t), "T, day {day}");
        assert_eq!(played.at_t(day).days, Some(n(t)), "T, day {day}");
        assert_eq!(played.at_u(day).stock_t, n(u), "U, day {day}");
        assert_eq!(played.at_u(day).days, Some(n(u)), "U, day {day}");
    }

    // No order on days 3, 4 and 5, and no order on day 2 but the routes.
    for day in [3, 4, 5] {
        let d = played.decisions(day);
        assert!(d.orders.is_empty() && d.commands.is_empty(), "day {day}");
    }
    // Days 3 and 4: intent 2 needs a run and has no ship free; its reason
    // names both commitments.
    for day in [3u32, 4] {
        let reasons = played.no_orders_of(day, 2);
        assert_eq!(reasons.len(), 1, "day {day}");
        serves(reasons[0], 2);
        mentions(
            reasons[0],
            &["ship:1", "run 1 of plan:1", "ship:2", "run 1 of plan:2"],
        );
        // Intent 1 needs no run.
        assert_eq!(played.no_orders_of(day, 1).len(), 1, "day {day}");
    }
    // Day 5: both ships are free; intent 1 needs no run; intent 2 needs one,
    // the reading of day 7 being 18 t, a shortfall of 12 days, and the source
    // has nothing above its reserve. It gives no order.
    assert_eq!(played.no_orders_of(5, 1).len(), 1);
    let reasons = played.no_orders_of(5, 2);
    assert_eq!(reasons.len(), 1);
    serves(reasons[0], 2);
    mentions(
        reasons[0],
        &["Short 12 days", "day 7", "18 t", "1000 t", "0 t"],
    );
    assert!(
        reasons[0]
            .binding
            .iter()
            .any(|b| b.contains("reserve") || b.contains("source")),
        "the source binds: {:?}",
        reasons[0].binding
    );

    // The end of day 5.
    assert_eq!(played.base_supply(5, 1), n(1000));
    assert_eq!(played.base_fuel(5, 1), n(900));
    assert_eq!(played.base_fuel(3, 1), n(900));
    assert_eq!(played.base_fuel(4, 1), n(900));
    assert_eq!(played.base_supply(5, 2), n(31));
    assert_eq!(played.base_supply(5, 3), n(19));
    let end = played.at(5);
    for (raw, f) in [(1u32, FORCE), (2, FORCE_2)] {
        let one = &end.ships[&ship(raw)];
        assert_eq!(one.location, ShipLocation::TaskForce(force(f)));
        assert!(one.supply_t.is_zero() && one.cargo_fuel_t.is_zero());
        assert_eq!(one.fuel_t, n(500), "ship {raw} refuelled on day 3");
        let formed = &end.task_forces[&force(f)];
        assert_eq!(formed.status, TaskForceStatus::Arrived);
        assert_eq!(formed.position(), S);
    }
    // Only two runs were made, both on day 1.
    let state = played.scenario.world.command_state(Side::Allied);
    assert_eq!(state.plans[&plan(1)].runs.len(), 1);
    assert_eq!(state.plans[&plan(2)].runs.len(), 1);
}

#[test]
fn with_no_reserve_intent_2s_first_load_is_30_t_and_with_a_reserve_of_1014_t_it_is_none() {
    // No reserve: base 1 holds 1040 t, intent 1 takes 26 t and intent 2 has
    // 1014 t left: its load is 36 t, cut to the room of ship 2, 30 t.
    let none = Commands {
        reserve: None,
        ..Commands::b()
    }
    .on(Which::B);
    let played = play(Which::B, &none);
    played.nothing_refused();
    let two: Vec<OrderKind> = played
        .orders_of(1, 2)
        .iter()
        .map(|o| o.kind.clone())
        .collect();
    assert_eq!(two, vec![form(2), load_to(FORCE_2, 30)]);
    let adds = played.add_runs(1);
    assert_eq!(adds[1].0, plan(2));
    assert_eq!(adds[1].1.load_t, n(30));
    mentions(adds[1].2, &["the ships' room is 30 t"]);
    assert_eq!(played.base_supply(1, 1), n(984));

    // A reserve of 1014 t: 26 t are above it, and intent 1 has them.
    let more = Commands {
        reserve: Some(1014),
        ..Commands::b()
    }
    .on(Which::B);
    let played = play(Which::B, &more);
    played.nothing_refused();
    let one: Vec<OrderKind> = played
        .orders_of(1, 1)
        .iter()
        .map(|o| o.kind.clone())
        .collect();
    assert_eq!(one, vec![form(1), load_to(FORCE, 26)]);
    assert!(played.orders_of(1, 2).is_empty());
    let adds = played.add_runs(1);
    assert_eq!(adds.len(), 1);
    assert_eq!(adds[0].0, plan(1));
    let reasons = played.no_orders_of(1, 2);
    assert_eq!(reasons.len(), 1);
    serves(reasons[0], 2);
    mentions(reasons[0], &["1040 t", "1014 t", "26 t"]);
    assert_eq!(played.base_supply(1, 1), n(1014));
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
    let dir = std::env::temp_dir().join(format!("flag_plot_s40_save_{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a directory");
    let data = common::data_dir()
        .canonicalize()
        .expect("the data directory");
    let whole = |text: String| text.replace("\"../", &format!("\"{}/", data.display()));
    // Both scenarios whole, and 40a in the middle, ship 1 at sea.
    let middle = dir.join("40a_in_the_middle.toml");
    std::fs::write(&middle, whole(days(&text_a(), 16, 11))).expect("written");
    let full_b = dir.join("40b.toml");
    std::fs::write(&full_b, whole(text_b())).expect("written");
    let saves = [
        (path(Which::A), dir.join("a.json")),
        (middle, dir.join("a_half.json")),
        (full_b, dir.join("b.json")),
    ];
    for (scenario, save) in &saves {
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
    for (_, save) in &saves {
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
