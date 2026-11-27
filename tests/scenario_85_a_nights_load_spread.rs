//! Scenario 85: a night's load spread over its ships. Three fast
//! transports keep a garrison fed at a place an enemy base overlooks.
//! They arrive with two hours of the night left. Each carries what it
//! can land in two hours, and the run lands three ships' worth where it
//! landed one.
//!
//! Proves decision 0162, the first part of the first mend of step 3's
//! stage 4, against a hand calculation. The data is placeholder data:
//! see `data/scenarios/85_a_nights_load_spread.toml`. It reads the same
//! figures as scenario 70. Nothing is sighted and nothing fights, so no
//! die changes anything.
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
//! As scenario 70's. A man eats 0.005 t a day, paid at the start of the
//! night pulse. A size 5 port works at 150 t an hour; a size 0 port by
//! lighter at 20 t an hour for each ship. Day d begins at hour
//! 24 x (d - 1): a night pulse of 12 hours and two daylight pulses of 6.
//! In a pulse ships move before cargo is worked. A force that has landed
//! its load steams on at the start of the next pulse. A ship burns its
//! cruising rate a mile at or below cruising speed, and that times the
//! square of the speed over the cruising speed above it.
//!
//! The supply doctrine: a run ordered on a morning loads that day, sails
//! the next and, with a passage of a day or less, arrives that same day;
//! it is ordered when the reading of the day after it would arrive is
//! below the floor's stock; its load is the floor's stock on the day it
//! arrives less that day's reading, plus the use of 15 days, cut to the
//! ships' room.
//!
//! ## Inputs
//!
//! The placeholder map of scenario 27: A at (0,0), port size 5; B at
//! (4,0), port size 0; C at (0,6), port size 0. A to B is 4 hexes,
//! 160 nm. B to C is 6 hexes. The game is 14 days.
//!
//! | Base | Side | Hex | Supply | Fuel |
//! | ---- | ---- | --- | ------ | ---- |
//! | 1, "Placeholder base A" | Allied | A | 1,000 t | 0 |
//! | 2, "Placeholder garrison B" | Allied | B | 100 t | 0 |
//! | 3, "Placeholder enemy base C" | Japanese | C | 0 | 0 |
//!
//! Base 3 is on the Allied map and has no ship, aircraft or man. Unit 1,
//! Allied, "Placeholder Garrison", a fighting unit of 2,000 men, is
//! ashore at B on the defence.
//!
//! A new placeholder class, 13, "Placeholder fast transport with 50 t of
//! cargo": class 12 with 50 t of cargo in place of 30 (16 kn at cruise,
//! 36 kn at full, 500 t of fuel over 4,000 nm, which is 0.125 t a mile at
//! cruise; 200 men). It is in a file of its own.
//!
//! Task force 1, Allied, on the transport mission: ships 1, 2 and 3 of
//! class 13, stopped at base 1 at cruising speed, each with 150 t of
//! fuel and nothing aboard. Base 1 holds no fuel, so no ship is fuelled.
//!
//! The Allied side's commands of day 1: intent 1, keep base 2 supplied,
//! with a floor of 5 days and no interval, and a forces limit of ships
//! 1, 2 and 3 and base 1 as the source. No risk is set, so balanced.
//! Leave to prepare and leave to launch are both given.
//!
//! ## By night
//!
//! Base 3 is 6 hexes from B, within 15, and not in B's hex: the run goes
//! by night. At 16 kn the passage is 160 / 16 = 10 hours: it arrives in
//! the night with 2 hours of it left. A ship burns 160 x 0.125 = 20 t
//! each way, 40 t out and home, and holds 150 t: it has the fuel. At
//! 36 kn the passage is 160 / 36 = 4 hours 26 minutes 40 seconds, and a
//! ship would burn 0.125 x (36 / 16) squared = 0.6328125 t a mile,
//! 202.5 t out and home: it has 150 t, so full speed is passed over, and
//! the reason gives 150 against 202.5. Cruising speed is the speed; the
//! force is at it already, so no speed order is given.
//!
//! A ship's limit is 20 x 2 = 40 t. The level is 40 t.
//!
//! ## The supply doctrine's sums
//!
//! The use at B is 2,000 x 0.005 = 10 t a day; the floor's stock is 50 t.
//!
//! | Morning | The stock at B | The day checked | Its reading | The staff |
//! | --- | --- | --- | --- | --- |
//! | 1 | 100 t | 3 | 80 t | No run |
//! | 2 | 90 t | 4 | 70 t | No run |
//! | 3 | 80 t | 5 | 60 t | No run |
//! | 4 | 70 t | 6 | 50 t | Not below 50: no run |
//! | 5 | 60 t | 7 | 40 t | A run |
//! | 7 | 160 t | 9 | 140 t | No run |
//! | 8 to 14 | 150 t, falling by 10 t a day to 90 t | | 130 t, falling to 70 t | No run |
//!
//! The run's load: the floor's stock on the day it arrives, 50 t, less
//! that day's reading, 50 t, which is 0; plus 15 days of use, 150 t; the
//! ships' room is 150 t, so 150 t. The night's limit: each of the three
//! ships takes the least of its room, 50 t, and the level, 40 t: 40 t
//! each, 120 t in all. The load of 150 t is cut to 120 t. With the run
//! landed the reading of the day checked is 40 + 120 = 160 t, above the
//! floor: no notice.
//!
//! As the rule stood before decision 0162 the load was cut to 40 t, all
//! in ship 1.
//!
//! ## Day by day
//!
//! | Day | Hour | The staff's commands and orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 | 0 | Approves plan 1 to preparing and to released | |
//! | 5 | 96 | `add_run`, `commit_ships`. Orders a load of supply to a weight of 120 t, no ship to hold more than 40 t | Ships 1, 2 and 3 load 40 t each |
//! | 6 | 120 | A route to B and back to A, acting at B, and an unload order | Sails at hour 120. Arrives at hour 130. Each ship lands 40 t by hour 132. The force steams for A at hour 132 and arrives at hour 142 |
//!
//! Three orders in all. On every other morning the staff gives one
//! reason for giving no order. No staff order or command is refused.
//!
//! The force is at B from hour 130 to hour 132, and at no hour of
//! daylight.
//!
//! ## Supply and fuel
//!
//! - **Base 2** pays 10 t at the start of each night: 90, 80, 70, 60 and
//!   50 t after the nights of days 1 to 5. On day 6 it pays 10 t and then
//!   takes 120 t: 160 t. Then it falls by 10 t a day: 80 t after day 14.
//!   No man goes unfed.
//! - **Base 1** loads out 120 t and ends with 880 t of supply.
//! - **Fuel.** Each ship burns 40 t and ends with 110 t. None is fuelled.
//!
//! ## The order alone
//!
//! A second game on the same map, with no staff: task force 1 of ships
//! 1, 2 and 3 of class 13 stopped at base 1, with 10 t, nothing and 45 t
//! of supply aboard. The Allied side orders a load of supply to a weight
//! of 100 t, no ship to hold more than 40 t. The force has 55 t aboard
//! and is to have 100 t: 45 t to take. Ship 1 takes the least of its
//! room, 40 t, the 45 t left and 40 - 10 = 30 t: 30 t. Ship 2 takes the
//! least of 50 t, the 15 t left and 40 t: 15 t. Ship 3 holds 45 t, above
//! the level: it takes nothing and gives nothing up. The ships end with
//! 40 t, 15 t and 45 t; base 1 ends with 955 t.
//!
//! # Properties
//!
//! - **The weight is not reached.** In the second game with nothing
//!   aboard any ship, a weight of 150 t and a level of 40 t: each ship
//!   loads 40 t, the force has 120 t and loads no more on any later day.
//! - **Refused.** A load with a level and no weight; with a level and a
//!   unit; with a level of 0. Each is refused by the engine, by the
//!   scenario's loader and by the agent's decoder.
//! - **No level.** A load to a weight of 100 t with no level, from
//!   nothing aboard: ships 1 and 2 load 50 t each and ship 3 none, as
//!   before.
//! - **Fuel as cargo** takes a level as supply does.
//! - **Press.** At press the run of the main case goes as before
//!   decision 0145: the load is 150 t with no level, 50 t in each ship;
//!   it sails at hour 120 and arrives at hour 130.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see give the same orders and reasons; a
//!   save on day 8 and a resume give the same game.

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde_json::{json, Value};
use sim_core::command::{CommandKind, NoticeKind, Reason, StaffDecisions, StaffOrder, Staffs};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, ShipId, TaskForceId};
use sim_core::orders::{
    Cargo, OrderKind, OrderRequest, Origin, RejectReason, SpeedSetting, TaskForceOrder,
};
use sim_core::state::{Side, TruthState};
use sim_data::{LoadOptions, Scenario};
use sim_staff::{as_staffs, staffs};

use crate::common;

const A: Hex = Hex { q: 0, r: 0 };
const B: Hex = Hex { q: 4, r: 0 };

fn path() -> PathBuf {
    common::data_dir().join("scenarios/85_a_nights_load_spread.toml")
}

fn path_b() -> PathBuf {
    common::data_dir().join("scenarios/85b_a_load_with_a_level.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn text_b() -> String {
    std::fs::read_to_string(path_b()).expect("the scenario file reads")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

/// The passage of 160 nm at 36 kn, by the engine's own arithmetic: the hand
/// calculation's "4 hours 26 minutes 40 seconds", 4.444444 hours.
fn fast() -> Fixed {
    n(160) / n(36)
}

fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

fn load_with(text: &str, options: &LoadOptions) -> Scenario {
    common::from_text_with(text, &path(), options).expect("the scenario loads")
}

struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    truth: Vec<TruthState>,
    events: Vec<Event>,
    hashes: Vec<u64>,
}

fn play(text: &str) -> Played {
    play_from(load_with(text, &common::frozen()), BTreeMap::new())
}

fn play_from(mut scenario: Scenario, mut handed: BTreeMap<u32, StaffDecisions>) -> Played {
    let held = staffs();
    let mut truth = vec![scenario.world.umpire().clone()];
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
    }
    Played {
        handed,
        truth,
        events: scenario.world.events().iter().cloned().collect(),
        hashes: scenario.world.day_hashes().to_vec(),
    }
}

/// What the staff gave: a load to a weight and its level, a speed, a route
/// with its mark, an unload.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Given {
    LoadTo(Fixed, Option<Fixed>),
    Speed(SpeedSetting),
    Route(Vec<Hex>, Vec<Hex>),
    Unload,
}

fn given(o: &StaffOrder) -> Given {
    match &o.kind {
        OrderKind::TaskForce {
            order:
                TaskForceOrder::LoadTo {
                    cargo: Cargo::Supply,
                    weight_t,
                    each_t,
                },
            ..
        } => Given::LoadTo(*weight_t, *each_t),
        OrderKind::TaskForce {
            order: TaskForceOrder::SetSpeed(s),
            ..
        } => Given::Speed(*s),
        OrderKind::TaskForce {
            order:
                TaskForceOrder::SetRoute {
                    waypoints,
                    act_at,
                    stand_at,
                },
            ..
        } => {
            assert!(stand_at.is_empty());
            Given::Route(waypoints.clone(), act_at.iter().copied().collect())
        }
        OrderKind::TaskForce {
            order: TaskForceOrder::Unload,
            ..
        } => Given::Unload,
        other => panic!("another order: {other:?}"),
    }
}

fn route_to(place: Hex) -> Given {
    Given::Route(vec![place, A], vec![place])
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Cmd {
    Approve(u32),
    AddRun(Vec<u32>, Fixed, (u32, u32, u32)),
    Commit(Vec<u32>),
}

fn cmd(c: &CommandKind) -> Cmd {
    match c {
        CommandKind::Approve { plan } => Cmd::Approve(plan.raw()),
        CommandKind::AddRun { run, .. } => Cmd::AddRun(
            run.ships.iter().map(|s| s.raw()).collect(),
            run.load_t,
            (run.days.load, run.days.sail, run.days.arrive),
        ),
        CommandKind::CommitShips { ships, .. } => {
            Cmd::Commit(ships.iter().map(|s| s.raw()).collect())
        }
        other => panic!("another command: {other:?}"),
    }
}

impl Played {
    fn days(&self) -> u32 {
        self.handed.len() as u32
    }

    fn orders(&self, day: u32) -> Vec<Given> {
        self.handed[&day].orders.iter().map(given).collect()
    }

    fn all_orders(&self) -> Vec<(u32, Given)> {
        (1..=self.days())
            .flat_map(|d| self.orders(d).into_iter().map(move |g| (d, g)))
            .collect()
    }

    fn all_commands(&self) -> Vec<(u32, Cmd)> {
        (1..=self.days())
            .flat_map(|d| {
                self.handed[&d]
                    .commands
                    .iter()
                    .map(move |c| (d, cmd(&c.kind)))
            })
            .collect()
    }

    fn order_reason(&self, day: u32, wanted: &Given) -> &Reason {
        &self.handed[&day]
            .orders
            .iter()
            .find(|o| given(o) == *wanted)
            .unwrap_or_else(|| panic!("day {day} has {wanted:?}"))
            .reason
    }

    fn no_orders(&self, day: u32) -> Vec<String> {
        self.handed[&day]
            .no_orders
            .iter()
            .filter(|r| r.serves.intent == Some(IntentId::new(1)))
            .map(text_of)
            .collect()
    }

    fn notices(&self) -> Vec<(u32, NoticeKind)> {
        (1..=self.days())
            .flat_map(|d| self.handed[&d].notices.iter().map(move |nt| (d, nt.kind)))
            .collect()
    }

    /// The halts and arrivals of task force 1: the hex and the hour.
    fn stops(&self) -> Vec<(Hex, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceHalted {
                    task_force,
                    hex,
                    elapsed_hours,
                    ..
                }
                | EventKind::TaskForceArrived {
                    task_force,
                    hex,
                    elapsed_hours,
                } if task_force.raw() == 1 => Some((*hex, *elapsed_hours)),
                _ => None,
            })
            .collect()
    }

    fn unloaded(&self) -> Vec<Fixed> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::UnloadingComplete {
                    task_force,
                    elapsed_hours,
                    ..
                } if task_force.raw() == 1 => Some(*elapsed_hours),
                _ => None,
            })
            .collect()
    }

    /// The start of the pulse of each steaming on.
    fn steamed_on(&self) -> Vec<Fixed> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceSteamedOn { task_force, .. } if task_force.raw() == 1 => {
                    let start = match e.pulse? {
                        sim_core::time::Pulse::Night => 0,
                        sim_core::time::Pulse::Morning => 12,
                        sim_core::time::Pulse::Afternoon => 18,
                    };
                    Some(n(24 * (i64::from(e.day) - 1) + start))
                }
                _ => None,
            })
            .collect()
    }

    /// The tons each ship loaded and then held, in log order: (ship, loaded, aboard).
    fn loads(&self) -> Vec<(u32, Fixed, Fixed)> {
        loads_of(&self.events)
    }

    /// The tons each ship put ashore, in log order: (ship, unloaded, aboard).
    fn lands(&self) -> Vec<(u32, Fixed, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::CargoUnloaded {
                    ship,
                    unloaded_t,
                    aboard_t,
                    ..
                } => Some((ship.raw(), *unloaded_t, *aboard_t)),
                _ => None,
            })
            .collect()
    }

    fn supply(&self, day: u32, b: u32) -> Fixed {
        self.truth[day as usize].bases[&BaseId::new(b)].supply_t
    }

    fn refused(&self) -> Vec<&Event> {
        refused_in(&self.events)
    }
}

fn loads_of(events: &[Event]) -> Vec<(u32, Fixed, Fixed)> {
    events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::CargoLoaded {
                ship,
                loaded_t,
                aboard_t,
                ..
            } => Some((ship.raw(), *loaded_t, *aboard_t)),
            _ => None,
        })
        .collect()
}

fn refused_in(events: &[Event]) -> Vec<&Event> {
    events
        .iter()
        .filter(|e| {
            matches!(
                e.kind,
                EventKind::OrderRejected { .. } | EventKind::CommandRefused { .. }
            )
        })
        .collect()
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

fn risk(level: &str) -> String {
    format!(
        "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"{level}\"\n"
    )
}

fn main_game() -> &'static Played {
    static GAME: std::sync::OnceLock<Played> = std::sync::OnceLock::new();
    GAME.get_or_init(|| play(&text()))
}

fn main_orders() -> Vec<(u32, Given)> {
    vec![
        (5, Given::LoadTo(n(120), Some(n(40)))),
        (6, route_to(B)),
        (6, Given::Unload),
    ]
}

// ---------------------------------------------------------------- The second game

/// A game of the order alone: no staff, scenario orders only.
struct Alone {
    truth: Vec<TruthState>,
    events: Vec<Event>,
}

fn alone_from(mut scenario: Scenario) -> Alone {
    let mut truth = vec![scenario.world.umpire().clone()];
    while scenario.next_day().expect("a day resolves").is_some() {
        truth.push(scenario.world.umpire().clone());
    }
    Alone {
        truth,
        events: scenario.world.events().iter().cloned().collect(),
    }
}

fn request(kind: OrderKind) -> OrderRequest {
    OrderRequest {
        kind,
        origin: Origin::PLAYER,
    }
}

fn scenario_b(text: &str) -> Scenario {
    common::from_text_with(text, &path_b(), &common::frozen()).expect("the scenario loads")
}

fn alone(text: &str) -> Alone {
    alone_from(scenario_b(text))
}

impl Alone {
    fn ship(&self, day: usize, id: u32) -> &sim_core::state::Ship {
        &self.truth[day].ships[&ShipId::new(id)]
    }

    fn base(&self, day: usize, id: u32) -> &sim_core::state::Base {
        &self.truth[day].bases[&BaseId::new(id)]
    }

    fn loads(&self) -> Vec<(u32, Fixed, Fixed)> {
        loads_of(&self.events)
    }
}

/// The three ships with nothing aboard.
fn empty_ships(t: &str) -> String {
    changed(&changed(t, "supply_t = 10\n", ""), "supply_t = 45\n", "")
}

const ORDER_LINE: &str = "weight_t = 100\neach_t = 40\n";

// ---------------------------------------------------------------- The hand calculation

#[test]
fn three_orders_in_all() {
    let g = main_game();
    assert_eq!(g.days(), 14);
    assert_eq!(g.all_orders(), main_orders());
    // No speed order is given: the force is at cruising speed already.
    assert!(!g
        .all_orders()
        .iter()
        .any(|(_, o)| matches!(o, Given::Speed(_))));
    // No staff order or command is refused.
    assert!(g.refused().is_empty(), "{:?}", g.refused());
    // On every other morning the staff gives one reason for no order.
    for day in [1, 2, 3, 4, 7, 8, 9, 10, 11, 12, 13, 14] {
        assert!(g.orders(day).is_empty(), "day {day}");
        assert_eq!(
            g.no_orders(day).len(),
            1,
            "day {day}: {:?}",
            g.no_orders(day)
        );
    }
}

#[test]
fn the_commands() {
    let g = main_game();
    // The run's load is read as the load the staff adds to its plan: the
    // 120 t the night allows, not the 150 t asked (the doc comment says only
    // "add_run, commit_ships").
    assert_eq!(
        g.all_commands(),
        vec![
            (1, Cmd::Approve(1)),
            (1, Cmd::Approve(1)),
            (5, Cmd::AddRun(vec![1, 2, 3], n(120), (5, 6, 6))),
            (5, Cmd::Commit(vec![1, 2, 3])),
        ]
    );
    assert!(g.notices().is_empty(), "{:?}", g.notices());
}

#[test]
fn the_load_of_150_t_is_cut_to_120_t_and_spread_over_three_ships() {
    let g = main_game();
    // The order: a weight of 120 t and a level of 40 t.
    assert_eq!(
        g.orders(5),
        vec![Given::LoadTo(n(120), Some(n(40)))],
        "the weight and the level"
    );
    // The load: 15 days of use, 150 t, less the floor's stock less that
    // day's reading, 50 - 50 = 0; the ships' room is 150 t.
    let all = text_of(g.order_reason(5, &Given::LoadTo(n(120), Some(n(40)))));
    assert_says(
        &all,
        &[
            "Load = max(0, 50 - 50) + 150 (15 days of use) = 150 t",
            "is cut to 120 t",
            "No ship is to hold more than 40 t",
            "ship:1 carries 40 t against a limit of 20 t an hour x 2 hours = 40 t",
            "ship:2 carries 40 t against a limit of 20 t an hour x 2 hours = 40 t",
            "ship:3 carries 40 t against a limit of 20 t an hour x 2 hours = 40 t",
            "The load of 150 t is cut to 120 t",
        ],
    );
    // Each ship loads 40 t; the three hold 40 t each.
    assert_eq!(
        g.loads(),
        vec![(1, n(40), n(40)), (2, n(40), n(40)), (3, n(40), n(40)),]
    );
    for ship in 1..=3 {
        assert_eq!(
            g.truth[5].ships[&ShipId::new(ship)].supply_t,
            n(40),
            "ship {ship} after day 5"
        );
    }
}

#[test]
fn by_night_at_cruising_speed_full_speed_passed_over() {
    let g = main_game();
    let all = text_of(g.order_reason(5, &Given::LoadTo(n(120), Some(n(40)))));
    assert_says(
        &all,
        &[
            "base:3 (Placeholder enemy base C) lies 6 hexes from (4,0), within 15",
            "at cruising speed the passage is 10 hours, arriving in the night with 2 hours of it left",
            &format!(
                "at full speed the passage is {} hours, arriving in the night with {} hours of it left",
                fast(),
                n(12) - fast()
            ),
            "ship:1 has 150 t aboard against 202.5 t needed for the passage out and home",
            "ship:2 has 150 t aboard against 202.5 t needed for the passage out and home",
            "ship:3 has 150 t aboard against 202.5 t needed for the passage out and home",
            "passed over",
            "It goes at cruising speed, with 2 hours of the night left on arrival",
        ],
    );
    assert!(!all.contains("It goes at full speed"), "{all}");
    // The same reasons are on the route order of day 6.
    let route = text_of(g.order_reason(6, &route_to(B)));
    assert_says(
        &route,
        &[
            "at cruising speed the passage is 10 hours, arriving in the night with 2 hours of it left",
            "202.5 t needed for the passage out and home",
            "It goes at cruising speed, with 2 hours of the night left on arrival",
            "ship:1 carries 40 t against a limit of 20 t an hour x 2 hours = 40 t",
        ],
    );
}

#[test]
fn the_supply_doctrines_sums() {
    let g = main_game();
    // (morning, stock at B, day checked, its reading, no run)
    let no_run = [
        (1u32, 100, 3, 80),
        (2, 90, 4, 70),
        (3, 80, 5, 60),
        (4, 70, 6, 50),
        (7, 160, 9, 140),
        (8, 150, 10, 130),
        (9, 140, 11, 120),
        (10, 130, 12, 110),
        (11, 120, 13, 100),
        (12, 110, 14, 90),
        (13, 100, 15, 80),
        (14, 90, 16, 70),
    ];
    for (morning, stock, checked, reading) in no_run {
        let said = g.no_orders(morning);
        assert_eq!(said.len(), 1, "day {morning}");
        assert_says(
            &said[0],
            &[
                &format!("Reading today at base:2: {stock} t over 10 t a day"),
                "against a floor of 5 days (50 t)",
                &format!("the reading of day {checked} is projected at {reading} t against 5 days x 10 t = 50 t"),
                "not below the floor, so no run is needed",
            ],
        );
    }
    // Morning 4: a reading of exactly 50 t is not below 50 t.
    assert_says(
        &g.no_orders(4)[0],
        &["projected at 50 t against 5 days x 10 t = 50 t: not below"],
    );
    // Morning 5: 60 t at B, day 7 read at 40 t: a run.
    let run = text_of(g.order_reason(5, &Given::LoadTo(n(120), Some(n(40)))));
    assert_says(
        &run,
        &[
            "Reading today at base:2: 60 t over 10 t a day = 6 days",
            "the reading of day 7 is 40 t against 5 days x 10 t = 50 t: below the floor, so a run is needed",
            "The stock projected for the reading of day 6 is 50 t",
        ],
    );
}

#[test]
fn the_passage_the_landing_and_the_hours() {
    let g = main_game();
    // Sails at hour 120; arrives at hour 130; each ship has landed its 40 t by
    // hour 132; the force steams for A at hour 132 and arrives at hour 142.
    let route = text_of(g.order_reason(6, &route_to(B)));
    assert_says(
        &route,
        &[
            "it sails today (day 6, hour 120)",
            "so it arrives at base:2 at hour 130 (day 6)",
        ],
    );
    assert_eq!(g.stops(), vec![(B, n(130)), (A, n(142))]);
    assert_eq!(g.unloaded(), vec![n(132)]);
    assert_eq!(g.steamed_on(), vec![n(132)]);
    assert_eq!(
        g.lands(),
        vec![(1, n(40), n(0)), (2, n(40), n(0)), (3, n(40), n(0))]
    );
    // At B from hour 130 to hour 132, and at no hour of daylight: the night
    // pulse of day 6 is hours 120 to 132.
    let night_ends = n(24 * 5 + 12);
    assert!(n(130) < night_ends);
    assert!(g.unloaded()[0] <= night_ends);
    for (hex, hour) in g.stops() {
        if hex == B {
            assert!(hour >= n(120) && hour < night_ends, "{hour}");
        }
    }
}

#[test]
fn supply_and_fuel_at_the_end() {
    let g = main_game();
    // Base 2: 90, 80, 70, 60, 50 after the nights of days 1 to 5; on day 6 it
    // pays 10 t and takes 120 t, 160 t; then 10 t a day less to 80 t.
    let c = [
        90, 80, 70, 60, 50, 160, 150, 140, 130, 120, 110, 100, 90, 80,
    ];
    for day in 1..=14u32 {
        assert_eq!(
            g.supply(day, 2),
            n(c[(day - 1) as usize]),
            "base 2 after day {day}"
        );
    }
    // Base 1 loads out 120 t and ends with 880 t.
    assert_eq!(g.supply(14, 1), n(880));
    assert_eq!(g.supply(4, 1), n(1000));
    // No man goes unfed.
    assert!(g
        .events
        .iter()
        .all(|e| !matches!(e.kind, EventKind::MenLostUnfed { .. })));
    // Fuel: each ship burns 40 t and ends with 110 t; none is fuelled.
    for s in 1..=3 {
        assert_eq!(g.truth[14].ships[&ShipId::new(s)].fuel_t, n(110));
        assert_eq!(g.truth[14].ships[&ShipId::new(s)].supply_t, n(0));
    }
    assert!(!g
        .events
        .iter()
        .any(|e| matches!(e.kind, EventKind::ShipRefuelled { .. })));
}

// ---------------------------------------------------------------- The order alone

#[test]
fn the_order_alone() {
    let a = alone(&text_b());
    // Ship 1 takes 30 t, ship 2 15 t; ship 3 holds 45 t, above the level: it
    // takes nothing and gives nothing up.
    assert_eq!(a.loads(), vec![(1, n(30), n(40)), (2, n(15), n(15))]);
    assert_eq!(a.ship(1, 1).supply_t, n(40));
    assert_eq!(a.ship(1, 2).supply_t, n(15));
    assert_eq!(a.ship(1, 3).supply_t, n(45));
    assert_eq!(a.base(1, 1).supply_t, n(955));
    // The force had 55 t and has 100 t.
    let held = a.ship(1, 1).supply_t + a.ship(1, 2).supply_t + a.ship(1, 3).supply_t;
    assert_eq!(held, n(100));
    assert!(a
        .events
        .iter()
        .all(|e| !matches!(e.kind, EventKind::OrderRejected { .. })));
}

// ---------------------------------------------------------------- The properties

#[test]
fn the_weight_is_not_reached() {
    // Nothing aboard any ship, a weight of 150 t and a level of 40 t, and
    // two more days on which nothing more is ordered.
    let t = changed(
        &changed(&empty_ships(&text_b()), "weight_t = 100", "weight_t = 150"),
        "days = 1\n",
        "days = 3\n",
    );
    let a = alone(&t);
    assert_eq!(
        a.loads(),
        vec![(1, n(40), n(40)), (2, n(40), n(40)), (3, n(40), n(40))]
    );
    for day in 1..=3 {
        let held = a.ship(day, 1).supply_t + a.ship(day, 2).supply_t + a.ship(day, 3).supply_t;
        assert_eq!(held, n(120), "day {day}");
        assert_eq!(a.base(day, 1).supply_t, n(880), "day {day}");
    }
}

#[test]
fn refused_by_the_engine() {
    // The engine's order cannot say a level with no weight or with a unit
    // (a LoadTo always has a weight and no unit): the engine refuses a level
    // of 0 or below, and those two are for the loader and the decoder.
    for level in [n(0), n(-5)] {
        let kind = OrderKind::TaskForce {
            target: TaskForceId::new(1),
            order: TaskForceOrder::LoadTo {
                cargo: Cargo::Supply,
                weight_t: n(100),
                each_t: Some(level),
            },
        };
        let mut scenario = scenario_b(&text_b());
        assert_eq!(
            scenario
                .world
                .check_order(Side::Allied, &request(kind.clone())),
            Err(RejectReason::LevelNotAboveZero(level))
        );
        scenario
            .world
            .submit_orders(Side::Allied, vec![request(kind)]);
        scenario.run_to_end().expect("the run resolves");
        let events: Vec<Event> = scenario.world.events().iter().cloned().collect();
        let reasons: Vec<RejectReason> = refused_in(&events)
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::OrderRejected { reason, .. } => Some(reason.clone()),
                _ => None,
            })
            .collect();
        assert_eq!(reasons, vec![RejectReason::LevelNotAboveZero(level)]);
    }
    // The level of 40 t is accepted.
    let kind = OrderKind::TaskForce {
        target: TaskForceId::new(1),
        order: TaskForceOrder::LoadTo {
            cargo: Cargo::Supply,
            weight_t: n(100),
            each_t: Some(n(40)),
        },
    };
    assert_eq!(
        scenario_b(&text_b())
            .world
            .check_order(Side::Allied, &request(kind)),
        Ok(())
    );
}

#[test]
fn refused_by_the_scenarios_loader() {
    let t = text_b();
    let refuses = |variant: String, says: &str| {
        let error = match common::from_text_with(&variant, &path_b(), &common::frozen()) {
            Ok(_) => panic!("the loader refuses it"),
            Err(error) => error.to_string(),
        };
        assert!(error.contains(says), "{error}");
    };
    // A level and no weight.
    refuses(changed(&t, "weight_t = 100\n", ""), "each_t");
    // A level and a unit (with a weight, and without).
    refuses(
        changed(&t, ORDER_LINE, "weight_t = 100\neach_t = 40\nunit = 1\n"),
        "unit",
    );
    refuses(changed(&t, ORDER_LINE, "each_t = 40\nunit = 1\n"), "each_t");
    // A level of 0 and one below.
    refuses(changed(&t, "each_t = 40", "each_t = 0"), "each_t");
    refuses(changed(&t, "each_t = 40", "each_t = -5"), "each_t");
    // And the same order with a level above 0 loads.
    assert!(common::from_text_with(&t, &path_b(), &common::frozen()).is_ok());
}

#[test]
fn refused_by_the_agents_decoder() {
    let decode = |body: Value| sim_agent::order_from_json("task_force:1", &body);
    // A level above 0 with a weight reads.
    assert_eq!(
        decode(json!({"kind":"load","cargo":"supply","weight_t":"100","each_t":"40"})),
        Ok(OrderKind::TaskForce {
            target: TaskForceId::new(1),
            order: TaskForceOrder::LoadTo {
                cargo: Cargo::Supply,
                weight_t: n(100),
                each_t: Some(n(40)),
            },
        })
    );
    // A level and no weight.
    assert!(decode(json!({"kind":"load","cargo":"supply","each_t":"40"})).is_err());
    // A level and a unit, with a weight and without.
    assert!(decode(json!({"kind":"load","unit":"land_unit:3","each_t":"40"})).is_err());
    assert!(
        decode(json!({"kind":"load","unit":"land_unit:3","weight_t":"100","each_t":"40"})).is_err()
    );
    // A level of 0, and one below.
    assert!(decode(json!({"kind":"load","weight_t":"100","each_t":"0"})).is_err());
    assert!(decode(json!({"kind":"load","weight_t":"100","each_t":"-5"})).is_err());
}

#[test]
fn no_level() {
    // A weight of 100 t with no level, from nothing aboard: ships 1 and 2
    // load 50 t each and ship 3 none, as before.
    let t = changed(&empty_ships(&text_b()), "each_t = 40\n", "");
    let a = alone(&t);
    assert_eq!(a.loads(), vec![(1, n(50), n(50)), (2, n(50), n(50))]);
    assert_eq!(a.ship(1, 3).supply_t, n(0));
    assert_eq!(a.base(1, 1).supply_t, n(900));
}

#[test]
fn fuel_as_cargo_takes_a_level() {
    // As supply does. Nothing aboard, a weight of 100 t and a level of 40 t:
    // ships 1 and 2 take 40 t of fuel cargo and ship 3 the 20 t left of the
    // weight (the doc comment says only that a level is taken as for supply;
    // these tons are that rule read on the three ships of 50 t).
    let fuel = changed(
        &empty_ships(&text_b()),
        "cargo = \"supply\"",
        "cargo = \"fuel\"",
    );
    let a = alone(&fuel);
    assert_eq!(
        a.loads(),
        vec![(1, n(40), n(40)), (2, n(40), n(40)), (3, n(20), n(20))]
    );
    for (ship, tons) in [(1, 40), (2, 40), (3, 20)] {
        assert_eq!(a.ship(1, ship).cargo_fuel_t, n(tons), "ship {ship}");
        assert_eq!(a.ship(1, ship).supply_t, n(0), "ship {ship}");
    }
    assert_eq!(a.base(1, 1).fuel_t, n(900));
    assert_eq!(a.base(1, 1).supply_t, n(1000));
    // With no level the first two ships are filled: 50 t, 50 t.
    let plain = alone(&changed(&fuel, "each_t = 40\n", ""));
    assert_eq!(plain.loads(), vec![(1, n(50), n(50)), (2, n(50), n(50))]);
}

#[test]
fn press() {
    // At press the run goes as before decision 0145: the load is 150 t with
    // no level, 50 t in each ship; it sails at hour 120 and arrives at hour
    // 130.
    let t = format!("{}\n{}", text(), risk("press"));
    let g = play(&t);
    assert_eq!(
        g.all_orders(),
        vec![
            (5, Given::LoadTo(n(150), None)),
            (6, route_to(B)),
            (6, Given::Unload),
        ]
    );
    assert_eq!(
        g.loads(),
        vec![(1, n(50), n(50)), (2, n(50), n(50)), (3, n(50), n(50))]
    );
    let route = text_of(g.order_reason(6, &route_to(B)));
    assert_says(&route, &["it sails today (day 6, hour 120)"]);
    assert!(!route.contains("By night"), "{route}");
    assert_eq!(g.stops()[0], (B, n(130)));
    assert!(!g
        .all_orders()
        .iter()
        .any(|(_, o)| matches!(o, Given::Speed(_))));
    assert!(g.refused().is_empty(), "{:?}", g.refused());
}

// ---------------------------------------------------------------- The same game

/// A game rebuilt from what was handed in to day `upto`.
fn rebuilt(handed: &BTreeMap<u32, StaffDecisions>, upto: u32) -> Scenario {
    let mut scenario = load_with(&text(), &common::frozen());
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
fn saved_on_day_8_and_resumed() {
    let through = main_game();
    let at_8 = rebuilt(&through.handed, 8);
    assert_eq!(at_8.world.day_hashes(), &through.hashes[..8]);
    let first: BTreeMap<u32, StaffDecisions> =
        (1..=8).map(|d| (d, through.handed[&d].clone())).collect();
    let resumed = play_from(at_8, first);
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
    // A Japanese land unit at the enemy base, of 100 men and of 300: the
    // Allied side cannot see it.
    let with = |men: u32| {
        changed(
            &text(),
            "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_intent\"",
            &format!(
                "[[land_unit]]\nid = 9\nname = \"Placeholder enemy\"\nside = \"japanese\"\nmen = {men}\nq = 0\nr = 6\n\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_intent\""
            ),
        )
    };
    let a = play(&with(100));
    let b = play(&with(300));
    assert_eq!(a.handed, b.handed);
    assert_eq!(a.all_orders(), main_orders());
}
