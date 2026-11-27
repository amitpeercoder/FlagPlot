//! Scenario 70: a run by night. Two fast transports keep a garrison fed
//! at a place an enemy base overlooks. At cruising speed they would
//! arrive by day. The staff sends them at full speed: they arrive in the
//! night, land their load in an hour and a half, and are gone at the
//! morning.
//!
//! Proves decision 0145, step 5 of stage 3, first part, against a hand
//! calculation. The data is placeholder data: see
//! `data/scenarios/70_a_run_by_night.toml`. It runs on the frozen step 1
//! figures. Nothing is sighted and nothing fights, so no die changes
//! anything.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If
//! the code disagrees, the code or the rule is wrong: do not change a
//! number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Figures
//!
//! A man eats 0.005 t a day, paid at the start of the night pulse. A size
//! 5 port works at 150 t an hour; a size 0 port by lighter at 20 t an
//! hour for each ship. Day d begins at hour 24 x (d - 1): a night pulse
//! of 12 hours and two daylight pulses of 6. In a pulse ships move before
//! cargo is worked. A force that has landed its load steams on at the
//! start of the next pulse. A ship burns its cruising rate a mile at or
//! below cruising speed, and that times the square of the speed over the
//! cruising speed above it. The frozen figures give no fuelling rate, so
//! a ship in port is filled in the pulse.
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
//! (4,0), port size 0; C at (0,6), port size 0. A to C is 6 hexes,
//! 240 nm. C to B is 6 hexes. The game is 14 days.
//!
//! | Base | Side | Hex | Supply | Fuel |
//! | ---- | ---- | --- | ------ | ---- |
//! | 1, "Placeholder base A" | Allied | A | 1,000 t | 2,000 t |
//! | 2, "Placeholder garrison C" | Allied | C | 100 t | 0 |
//! | 3, "Placeholder enemy base B" | Japanese | B | 0 | 0 |
//!
//! Base 3 is on the Allied map and has no ship, aircraft or man. Unit 1,
//! Allied, "Placeholder Garrison", a fighting unit of 2,000 men, is
//! ashore at C on the defence.
//!
//! Task force 1, Allied, on the transport mission: ships 1 and 2,
//! placeholder fast transports with cargo (class 12: 30 t of cargo,
//! 16 kn at cruise, 36 kn at full, 500 t of fuel over 4,000 nm, which is
//! 0.125 t a mile at cruise), stopped at base 1 at cruising speed, full
//! of fuel, nothing aboard.
//!
//! The Allied side's commands of day 1: intent 1, keep base 2 supplied,
//! with a floor of 5 days and no interval, and a forces limit of ships 1
//! and 2 and base 1 as the source. No risk is set, so balanced. Leave to
//! prepare and leave to launch are both given.
//!
//! ## By night
//!
//! Base 3 is 6 hexes from C, within 15, and not in C's hex: the run goes
//! by night. At 16 kn the passage is 240 / 16 = 15 hours: it would arrive
//! at hour 15 of the day, in daylight. At 36 kn it is 240 / 36 = 6 hours
//! 40 minutes: it arrives in the night with 5 hours 20 minutes left. A
//! ship's limit is 20 x 5 1/3 = 106 2/3 t; each carries 30 t, which it
//! lands in 1.5 hours.
//!
//! At 36 kn a ship burns 0.125 x (36 / 16) squared = 0.125 x 5.0625 t a
//! mile: 151.875 t each way and 303.75 t out and home, against 60 t at
//! cruise. Each ship holds 500 t: it has the fuel. Full speed is ordered.
//!
//! ## The supply doctrine's sums
//!
//! The use at C is 2,000 x 0.005 = 10 t a day; the floor's stock is 50 t.
//!
//! | Morning | The stock at C | The day checked | Its reading | The staff |
//! | --- | --- | --- | --- | --- |
//! | 1 | 100 t | 3 | 80 t | No run |
//! | 2 | 90 t | 4 | 70 t | No run |
//! | 3 | 80 t | 5 | 60 t | No run |
//! | 4 | 70 t | 6 | 50 t | Not below 50: no run |
//! | 5 | 60 t | 7 | 40 t | A run |
//! | 7 | 100 t | 9 | 80 t | No run |
//! | 8, 9, 10 | 90, 80, 70 t | | 70, 60, 50 t | No run |
//! | 11 | 60 t | 13 | 40 t | A run |
//! | 13, 14 | 100, 90 t | | | No run |
//!
//! Each run's load: the floor's stock on the day it arrives, 50 t, less
//! that day's reading, 50 t, which is 0; plus 15 days of use, 150 t; cut
//! to the ships' room, 60 t. With the run landed the reading of the day
//! checked is 40 + 60 = 100 t, above the floor: no notice.
//!
//! ## Day by day
//!
//! | Day | Hour | The staff's commands and orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 | 0 | Approves plan 1 to preparing and to released | |
//! | 5 | 96 | `add_run`, `commit_ships`. Orders a load of supply to a weight of 60 t | Each ship loads 30 t |
//! | 6 | 120 | Speed full. A route to C and back to A, acting at C, and an unload order | Sails at hour 120. Arrives at hour 126 and 40 minutes. Each ship lands 30 t by hour 128 and 10 minutes. The force steams for A at hour 132 and arrives at hour 138 and 40 minutes |
//! | 11 | 240 | `add_run`, `commit_ships`. Orders a load of supply to a weight of 60 t | |
//! | 12 | 264 | A route to C and back to A, acting at C, and an unload order | Sails at hour 264, arrives at hour 270 and 40 minutes, has landed 60 t by hour 272 and 10 minutes, steams at hour 276, arrives at hour 282 and 40 minutes |
//!
//! Seven orders in all. On every other morning the staff gives one reason
//! for giving no order. No staff order or command is refused.
//!
//! The force is at C from hour 126 and 40 minutes to hour 132 and from
//! hour 270 and 40 minutes to hour 276, and at no hour of daylight.
//!
//! ## Supply
//!
//! - **Base 2** pays 10 t at the start of each night: 90, 80, 70, 60 and
//!   50 t after the nights of days 1 to 5. On day 6 it pays 10 t and then
//!   takes 60 t: 100 t. Then 90, 80, 70, 60 and 50 t after days 7 to 11;
//!   on day 12 it pays 10 t and takes 60 t: 100 t; 90 t after day 13 and
//!   80 t after day 14.
//! - **Base 1** loads out 120 t and ends with 880 t of supply.
//! - **Fuel.** The force's speed stays at full, so it comes home at 36 kn
//!   too. Each ship burns 303.75 t on each run and is filled at A in the
//!   pulse it arrives. Base 1 gives 4 x 303.75 = 1,215 t and ends with
//!   785 t of fuel; each ship ends full.
//!
//! # Properties
//!
//! - **Press.** At press the run goes as before: no speed order is
//!   given, the force sails at hour 120 at 16 kn, arrives at hour 135, in
//!   the morning pulse, has landed 60 t by hour 136 and 30 minutes,
//!   steams at hour 138 and arrives at A at hour 153. Six orders in all.
//! - **No enemy base near.** With base 3 left out, at balanced: as at
//!   press.
//! - **The side's own fighters at the place.** With an airfield at base 2
//!   and a fighter group of the Allied side based there on no mission, at
//!   balanced: as at press. The group flies nothing and draws nothing, so
//!   the supply doctrine's sums are the main case's. (Item 2a, added by
//!   the architect after the build.)
//! - **The side's own fighters in a hex next to the place.** With an
//!   Allied base that has an airfield and a fighter group on no mission
//!   in a hex next to C, and none at C, at balanced: as at press. With
//!   that base two hexes from C instead: the main case's seven orders.
//!   The added base is under no intent and draws nothing. (Ruling 6.)
//! - **An enemy base in the place's own hex only.** With base 3 at C and
//!   not on the Allied side's way: as at press.
//! - **No speed arrives in the dark, at balanced.** With base 2 and unit 1
//!   at B, base 3 at C, and the ships placeholder troop transports (class
//!   10: 10 kn at cruise, 12 kn at full): the passage of 160 nm is 16
//!   hours at cruise and 13 hours 20 minutes at full, both by day. The
//!   run of day 5 sails on day 6 at cruising speed with no speed order,
//!   and the reason of its route gives both hours.
//! - **No speed arrives in the dark, at preserve the force.** The same,
//!   at preserve the force: the run is loaded on day 5 and does not sail.
//!   On the morning of day 6 and on each morning after, the staff gives
//!   no order and hands in the notice that it cannot meet intent 1.
//! - **The night's limit.** With the lighter rate of a size 0 port at
//!   4 t an hour: a ship's limit is 4 x 5 1/3 = 21 1/3 t. The load of
//!   60 t is cut to 42 2/3 t, 21 1/3 t in each ship (decision 0162; as
//!   the rule stood it was 21 1/3 t, all in ship 1). The reason gives
//!   the cut and the level.
//! - **Not the fuel for full speed.** With each ship holding 300 t and
//!   base 1 no fuel: at 36 kn a ship needs 303.75 t and has 300. Full
//!   speed is passed over, cruise arrives by day, and at balanced the run
//!   goes by day at cruising speed; the reason gives 300 against 303.75.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see give the same orders and reasons; a
//!   save on day 8 and a resume give the same game; scenarios 67, 68 and
//!   69 give the orders they gave.

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::command::{CommandKind, NoticeKind, Reason, StaffDecisions, StaffOrder, Staffs};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, ShipId};
use sim_core::orders::{Cargo, OrderKind, SpeedSetting, TaskForceOrder};
use sim_core::state::{Side, TruthState};
use sim_data::{LoadOptions, Scenario};
use sim_staff::{as_staffs, staffs};

use crate::common;

const COMMANDS_MARKER: &str = "# The Allied commander's commands";
const A: Hex = Hex { q: 0, r: 0 };
const B: Hex = Hex { q: 4, r: 0 };
const C: Hex = Hex { q: 0, r: 6 };

fn path() -> PathBuf {
    common::data_dir().join("scenarios/70_a_run_by_night.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

/// Hours and tenths: x(1365) is 136.5.
fn x(tenths: i64) -> Fixed {
    Fixed::from_ratio(tenths, 10)
}

/// The passage of 240 nm at 36 kn, by the engine's own arithmetic: the
/// hand calculation's "6 hours 40 minutes", 6.666666 hours.
fn fast() -> Fixed {
    n(240) / n(36)
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
    play_with(text, &common::frozen())
}

fn play_with(text: &str, options: &LoadOptions) -> Played {
    play_from(load_with(text, options), BTreeMap::new())
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

#[derive(Clone, PartialEq, Eq, Debug)]
enum Given {
    LoadTo(Fixed),
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
                    ..
                },
            ..
        } => Given::LoadTo(*weight_t),
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

    fn supply(&self, day: u32, b: u32) -> Fixed {
        self.truth[day as usize].bases[&BaseId::new(b)].supply_t
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

fn with_commands(t: &str, extra: &str) -> String {
    format!("{t}\n{extra}")
}

fn risk(level: &str) -> String {
    format!(
        "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"{level}\"\n"
    )
}

const BASE_3: &str =
    "[[base]]\nid = 3\nname = \"Placeholder enemy base B\"\nside = \"japanese\"\nq = 4\nr = 0\n";

fn main_game() -> &'static Played {
    static GAME: std::sync::OnceLock<Played> = std::sync::OnceLock::new();
    GAME.get_or_init(|| play(&text()))
}

fn main_orders() -> Vec<(u32, Given)> {
    vec![
        (5, Given::LoadTo(n(60))),
        (6, Given::Speed(SpeedSetting::Full)),
        (6, route_to(C)),
        (6, Given::Unload),
        (11, Given::LoadTo(n(60))),
        (12, route_to(C)),
        (12, Given::Unload),
    ]
}

/// The orders of a run that goes as before, at cruise by day.
fn as_before() -> Vec<(u32, Given)> {
    vec![
        (5, Given::LoadTo(n(60))),
        (6, route_to(C)),
        (6, Given::Unload),
        (11, Given::LoadTo(n(60))),
        (12, route_to(C)),
        (12, Given::Unload),
    ]
}

// ---------------------------------------------------------------- The hand calculation

#[test]
fn seven_orders_in_all() {
    let g = main_game();
    assert_eq!(g.days(), 14);
    assert_eq!(g.all_orders(), main_orders());
    assert!(g.refused().is_empty(), "{:?}", g.refused());
    for day in [1, 2, 3, 4, 7, 8, 9, 10, 13, 14] {
        assert!(g.orders(day).is_empty());
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
    assert_eq!(
        g.all_commands(),
        vec![
            (1, Cmd::Approve(1)),
            (1, Cmd::Approve(1)),
            (5, Cmd::AddRun(vec![1, 2], n(60), (5, 6, 6))),
            (5, Cmd::Commit(vec![1, 2])),
            (11, Cmd::AddRun(vec![1, 2], n(60), (11, 12, 12))),
            (11, Cmd::Commit(vec![1, 2])),
        ]
    );
    assert!(g.notices().is_empty(), "{:?}", g.notices());
}

#[test]
fn the_passages_by_night() {
    let g = main_game();
    assert_eq!(
        g.stops(),
        vec![
            (C, n(120) + fast()),
            (A, n(132) + fast()),
            (C, n(264) + fast()),
            (A, n(276) + fast()),
        ]
    );
    // Each ship lands 30 t at 20 t an hour: 1.5 hours.
    assert_eq!(
        g.unloaded(),
        vec![n(120) + fast() + x(15), n(264) + fast() + x(15)]
    );
    assert_eq!(g.steamed_on(), vec![n(132), n(276)]);
    // At C only in the night: from 126 h 40 m to 132, and 270 h 40 m to 276.
    for (hex, hour) in g.stops() {
        if hex == C {
            let into = hour - n(24) * ((hour / n(24)).trunc());
            assert!(into < n(12), "{hour}");
        }
    }
}

#[test]
fn the_reasons_of_the_run_by_night() {
    let g = main_game();
    let all = text_of(g.order_reason(6, &route_to(C)));
    assert_says(
        &all,
        &[
            "base:3 (Placeholder enemy base B) lies 6 hexes from (0,6)",
            "at cruising speed the passage is 15 hours, arriving by day",
            &format!(
                "at full speed the passage is {} hours, arriving in the night with {} hours of it left",
                fast(),
                n(12) - fast()
            ),
            "It goes at full speed",
            &format!(
                "ship:1 carries 30 t against a limit of 20 t an hour x {} hours = {} t",
                n(12) - fast(),
                n(20) * (n(12) - fast())
            ),
        ],
    );
    assert_says(
        &text_of(g.order_reason(6, &Given::Speed(SpeedSetting::Full))),
        &["full speed"],
    );
    assert_says(
        &text_of(g.order_reason(5, &Given::LoadTo(n(60)))),
        &["By night:", "It goes at full speed"],
    );
}

#[test]
fn supply_and_fuel() {
    let g = main_game();
    let c = [90, 80, 70, 60, 50, 100, 90, 80, 70, 60, 50, 100, 90, 80];
    for day in 1..=14u32 {
        assert_eq!(
            g.supply(day, 2),
            n(c[(day - 1) as usize]),
            "base 2 after day {day}"
        );
    }
    assert_eq!(g.supply(14, 1), n(880));
    assert_eq!(g.truth[14].bases[&BaseId::new(1)].fuel_t, n(785));
    for s in [1, 2] {
        assert_eq!(g.truth[14].ships[&ShipId::new(s)].fuel_t, n(500));
    }
    let burned: Vec<Fixed> = g
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::ShipRefuelled { ship, taken_t, .. } if ship.raw() == 1 => Some(*taken_t),
            _ => None,
        })
        .collect();
    let each = Fixed::from_ratio(30375, 100);
    assert_eq!(burned, vec![each, each]);
}

// ---------------------------------------------------------------- The properties

/// A run that goes as before: no speed order, at 16 kn, arriving at hour
/// 135, landed by 136.5, steaming on at 138 and home at 153.
fn assert_as_before(g: &Played) {
    assert_eq!(g.all_orders(), as_before());
    let stops = g.stops();
    assert_eq!(stops[0], (C, n(135)));
    assert_eq!(stops[1], (A, n(153)));
    assert_eq!(g.unloaded()[0], x(1365));
    assert_eq!(g.steamed_on()[0], n(138));
    assert!(g.refused().is_empty(), "{:?}", g.refused());
}

#[test]
fn press() {
    assert_as_before(&play(&with_commands(&text(), &risk("press"))));
}

#[test]
fn no_enemy_base_near() {
    let t = changed(&text(), &format!("{BASE_3}supply_t = 0\nfuel_t = 0\n"), "");
    assert_as_before(&play(&t));
}

#[test]
fn an_enemy_base_in_the_places_own_hex_only() {
    let t = changed(
        &text(),
        BASE_3,
        "[[base]]\nid = 3\nname = \"Placeholder enemy base B\"\nside = \"japanese\"\nq = 0\nr = 6\n",
    );
    assert_as_before(&play(&t));
}

#[test]
fn the_sides_own_fighters_at_the_place() {
    let t = changed(
        &text(),
        "name = \"Placeholder garrison C\"\nside = \"allied\"\nq = 0\nr = 6\nsupply_t = 100\nfuel_t = 0\n",
        "name = \"Placeholder garrison C\"\nside = \"allied\"\nq = 0\nr = 6\nsupply_t = 100\nfuel_t = 0\nairfield = true\n",
    );
    let group = "[[air_group]]\nid = 1\nname = \"Placeholder fighters 1\"\nside = \"allied\"\nbase = 2\naircraft_type = 4\nready = 12\n\n";
    let t = changed(&t, COMMANDS_MARKER, &format!("{group}{COMMANDS_MARKER}"));
    assert_as_before(&play(&t));
}

/// The text with an Allied base with an airfield and a fighter group at
/// `(q, r)`, a coast hex added to a copy of the map, and the options that
/// read it.
fn with_fighter_base(q: i32, r: i32) -> String {
    let map_text =
        std::fs::read_to_string(common::data_dir().join("map/placeholder_preparing.toml"))
            .expect("the map reads")
            + &format!("\n[[hex]]\nq = {q}\nr = {r}\nkind = \"coast\"\nport_size = 0\n");
    let dir = std::env::temp_dir().join(format!("flag_plot_s70_{}_{q}_{r}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a directory");
    let map_path = dir.join("map.toml");
    std::fs::write(&map_path, map_text).expect("the map is written");
    let t = changed(
        &text(),
        "map = \"../map/placeholder_preparing.toml\"",
        &format!("map = \"{}\"", map_path.display()),
    );
    let base = format!(
        "[[base]]\nid = 4\nname = \"Placeholder airfield\"\nside = \"allied\"\nq = {q}\nr = {r}\nsupply_t = 0\nfuel_t = 0\nairfield = true\n\n[[air_group]]\nid = 1\nname = \"Placeholder fighters 1\"\nside = \"allied\"\nbase = 4\naircraft_type = 4\nready = 12\n\n"
    );
    changed(&t, COMMANDS_MARKER, &format!("{base}{COMMANDS_MARKER}"))
}

#[test]
fn the_sides_own_fighters_in_a_hex_next_to_the_place() {
    // (-1,6) is next to C at (0,6): as at press.
    assert_eq!(C.distance(Hex { q: -1, r: 6 }), 1);
    assert_as_before(&play(&with_fighter_base(-1, 6)));
    // (-2,6) is two hexes from C: the main case's seven orders.
    assert_eq!(C.distance(Hex { q: -2, r: 6 }), 2);
    let g = play(&with_fighter_base(-2, 6));
    assert_eq!(g.all_orders(), main_orders());
}

/// Base 2 and unit 1 at B, base 3 at C, and the ships troop transports.
fn no_speed_in_the_dark(t: &str) -> String {
    let t = changed(
        t,
        "name = \"Placeholder garrison C\"\nside = \"allied\"\nq = 0\nr = 6\n",
        "name = \"Placeholder garrison C\"\nside = \"allied\"\nq = 4\nr = 0\n",
    );
    let t = changed(
        &t,
        BASE_3,
        "[[base]]\nid = 3\nname = \"Placeholder enemy base B\"\nside = \"japanese\"\nq = 0\nr = 6\n",
    );
    let t = changed(
        &t,
        "men = 2000\nq = 0\nr = 6\n",
        "men = 2000\nq = 4\nr = 0\n",
    );
    t.replace("class = 12\n", "class = 10\n")
}

#[test]
fn no_speed_arrives_in_the_dark_at_balanced() {
    let g = play(&no_speed_in_the_dark(&text()));
    assert!(!g.orders(6).iter().any(|o| matches!(o, Given::Speed(_))));
    assert!(g.orders(6).contains(&route_to(B)));
    assert_says(
        &text_of(g.order_reason(6, &route_to(B))),
        &[
            "at cruising speed the passage is 16 hours, arriving by day",
            &format!(
                "at full speed the passage is {} hours, arriving by day",
                n(160) / n(12)
            ),
            "neither speed brings it there in the night",
        ],
    );
}

#[test]
fn no_speed_arrives_in_the_dark_at_preserve_the_force() {
    let g = play(&with_commands(
        &no_speed_in_the_dark(&text()),
        &risk("preserve"),
    ));
    assert!(matches!(g.orders(5)[..], [Given::LoadTo(_)]));
    for day in 6..=14 {
        assert!(g.orders(day).is_empty(), "day {day}: {:?}", g.orders(day));
    }
    let cannot: Vec<u32> = g
        .notices()
        .into_iter()
        .filter(|(_, k)| *k == NoticeKind::CannotMeet)
        .map(|(d, _)| d)
        .collect();
    assert_eq!(cannot, (6..=14).collect::<Vec<_>>());
}

#[test]
fn the_nights_limit() {
    let options = common::altered_reference("scenario_70_limit", |dir| {
        common::edit_file(dir, "port_sizes.csv", |t| {
            common::replace_once(&t, "\n0,0,0,20,", "\n0,0,0,4,")
        });
    });
    let g = play_with(&text(), &options);
    let limit = n(4) * (n(12) - fast());
    // As the doc comment restates it for decision 0162: the load of 60 t is
    // cut to 42 2/3 t, 21 1/3 t in each ship, and the order names the level.
    let load = limit + limit;
    assert_eq!(g.orders(5), vec![Given::LoadTo(load)]);
    let level = g.handed[&5].orders.iter().find_map(|o| match &o.kind {
        OrderKind::TaskForce {
            order: TaskForceOrder::LoadTo { each_t, .. },
            ..
        } => Some(*each_t),
        _ => None,
    });
    assert_eq!(level, Some(Some(limit)));
    assert_eq!(g.truth[5].ships[&ShipId::new(1)].supply_t, limit);
    assert_eq!(g.truth[5].ships[&ShipId::new(2)].supply_t, limit);
    assert_says(
        &text_of(g.order_reason(5, &Given::LoadTo(load))),
        &[
            &format!("is cut to {load} t"),
            &format!("No ship is to hold more than {limit} t"),
        ],
    );
}

#[test]
fn not_the_fuel_for_full_speed() {
    let t = text().replace("fuel_t = 500\n", "fuel_t = 300\n").replacen(
        "supply_t = 1000\nfuel_t = 2000\n",
        "supply_t = 1000\nfuel_t = 0\n",
        1,
    );
    let g = play(&t);
    assert!(!g.orders(6).iter().any(|o| matches!(o, Given::Speed(_))));
    assert_says(
        &text_of(g.order_reason(6, &route_to(C))),
        &[
            "ship:1 has 300 t aboard against 303.75 t needed",
            "neither speed brings it there in the night",
        ],
    );
}

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
}

#[test]
fn what_the_side_cannot_see_changes_nothing() {
    let enemy = |men: u32| {
        format!(
            "[[land_unit]]\nid = 9\nname = \"Placeholder enemy\"\nside = \"japanese\"\nmen = {men}\nq = 4\nr = 0\n"
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
    assert_eq!(a.all_orders(), main_orders());
}
