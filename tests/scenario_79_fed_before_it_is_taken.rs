//! Scenario 79: fed before it is taken. Men are ashore beside an airfield
//! the enemy still holds, living on what their landing brought. The
//! commander has asked that the airfield be held. The staff feeds the men
//! through the base their own side has in the hex, and sends the first run
//! of supply when it is needed, without waiting for the airfield to change
//! hands.
//!
//! Proves decision 0159, rule A (items 1 to 3), step 1 of the opening of
//! stage 4, against a hand calculation. The data is placeholder data: see
//! `data/scenarios/79_fed_before_it_is_taken.toml`. It runs on the frozen
//! step 1 figures. Nobody is on the attack, so nobody fights and no die is
//! rolled.
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
//! As scenario 69's. A man eats 0.005 t a day, paid at the start of the
//! night pulse by his side's bases in his hex, in base order. Supply
//! landed goes into the side's lowest-numbered base in the hex. A size 0
//! port works by lighter at 20 t an hour for each ship. Day d begins at
//! hour 24 x (d - 1): a night pulse of 12 hours and two daylight pulses
//! of 6. In a pulse ships move before cargo is worked.
//!
//! The staff's figures: at balanced the margin is 5 days; a supply intent
//! with no interval uses 15 days.
//!
//! ## Inputs
//!
//! The placeholder map of scenario 27: A at (0,0), a coast hex with port
//! size 5, and B at (4,0), a coast hex with port size 0, 160 nm apart: 16
//! hours at 10 kn. The game is 14 days.
//!
//! | Base | Side | Hex | Supply | Fuel |
//! | ---- | ---- | --- | ------ | ---- |
//! | 1, "Placeholder base A" | Allied | A | 1,000 t | 0 |
//! | 2, "Placeholder airfield B" | Japanese | B | 100 t | 0 |
//! | 3, "Placeholder beach at B" | Allied | B | 48 t | 0 |
//!
//! Base 2 has no gun and no aircraft and is on the Allied map. Base 3 is
//! the base a landing forms: no airfield, not on the map. No base has a
//! reserve. The Allied side holds no estimate of the enemy.
//!
//! Task force 1, Allied, on the transport mission: ships 1 and 2,
//! placeholder troop transports (600 men, 3,000 t of cargo, 10 kn, 0.1 t
//! of fuel a mile, 1,000 t of fuel), stopped at base 1, full of fuel,
//! nothing aboard.
//!
//! Units 1 and 2, Allied, "Placeholder First" and "Placeholder Second":
//! fighting units of 600 men each, ashore at B on the defence. Unit 3,
//! Japanese, "Placeholder garrison": a fighting unit of 400 men ashore at
//! B on the defence.
//!
//! The Allied side's command of day 1: intent 1, hold base 2, with a
//! forces limit of task force 1 and base 1 as the source. No risk is set,
//! so balanced. Leave to prepare and leave to launch are both given. The
//! Japanese side is given nothing.
//!
//! ## What the staff does
//!
//! Base 2 is not the Allied side's, and the Allied side has base 3 in the
//! same hex with 1,200 men ashore there. So on day 1 the staff forms
//! intent 2, keep base 3 supplied, under intent 1, with a floor of 5 days
//! (rule A, item 1). The hold's reason that day says that base 2 is not
//! the side's, that the staff waits for it, and that the men there are
//! fed through base 3 (item 3). It gives no order to unit 1 or unit 2.
//!
//! The use at B is the Allied men ashore, 1,200 x 0.005 = 6 t a day. The
//! floor's stock is 5 x 6 = 30 t. The stock the staff reads on a morning
//! is base 3's before that night's charge. A run ordered on a morning
//! loads that day, sails the next and arrives that same day; the reading
//! checked is the day after.
//!
//! | Morning | The stock at base 3 | The day checked | Its reading | The staff |
//! | --- | --- | --- | --- | --- |
//! | 2 | 42 t | 4 | 42 - 12 = 30 t | Not below 30: no run |
//! | 3 | 36 t | 5 | 36 - 12 = 24 t | Below 30: a run |
//!
//! On the morning of day 1 no run is ordered either: a supply intent is
//! first served on the morning after it is formed (and the reading would
//! be 48 - 12 = 36 t, not below 30).
//!
//! The run's load: the floor's stock on the day it arrives, day 4, 30 t,
//! less that day's reading, 30 t, which is 0; plus 15 days of use, 90 t.
//!
//! | Day | Hour | The staff's orders | What happens |
//! | --- | --- | --- | --- |
//! | 3 | 48 | A load of supply to a weight of 90 t | Ship 1 loads 90 t at A |
//! | 4 | 72 | A route to B and back to A, acting at B, and an unload order | Sails at hour 72, arrives at hour 88. Ship 1 lands 40 t in the morning and 50 t in the afternoon, into base 3 |
//! | 5 | 96 | none | The force steams for A at hour 96 and arrives at hour 112 |
//!
//! Three orders to task force 1 in all, and none to a land unit. No staff
//! order or command is refused. After the run, a morning's stock is
//! 114 - 6 x (d - 5) for day d from 5 to 14, never below 42 t, so no
//! second run is called in the game (a run is called when the morning's
//! stock is below 42 t).
//!
//! ## Supply, fuel and men
//!
//! - **Base 3** pays 6 t on each of the 14 nights. After the nights of
//!   days 1 to 4 it holds 42, 36, 30 and 24 t. It takes 90 t on day 4:
//!   114 t. It pays 6 t on the nights of days 5 to 14: 54 t at the end.
//! - **Base 1** feeds nobody. It loads out 90 t: 910 t at the end.
//! - **Base 2** feeds 400 men, 2 t a night, for 14 nights: 72 t at the
//!   end. It is Japanese at the end.
//! - **Each ship** steams 320 nm and burns 32 t: 968 t at the end.
//! - **Every man is fed every night.** Units 1 and 2 have 600 men each at
//!   the end and unit 3 has 400. No man is lost, landed or embarked.
//!
//! # Properties
//!
//! - **No hold stated.** With intent 1 left out, no intent is formed and
//!   no order is given in the game. Base 3 pays 6 t on the nights of days
//!   1 to 8 and then holds nothing. On the night of day 9 it is asked for
//!   6 t and pays none, and 1,200 Allied men are unfed. (What hunger then
//!   takes is the engine's rule and is not asserted here.)
//! - **No man ashore.** With units 1 and 2 left out and intent 1 stated,
//!   the staff forms no intent and gives no order: the hold waits for
//!   base 2 to be the side's, as before this decision.
//! - **No source.** With base 1 left out of intent 1's forces limit, no
//!   "keep supplied" is formed, and the hold's reason says that its
//!   forces limit names no source.
//! - **Formed once.** In the main case exactly one "keep supplied" intent
//!   is in force for the hex on every day from 2 to 14 (item 2).
//! - **Stated as before.** The Japanese side, given nothing, is given no
//!   order by its staff on any day.

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::command::{
    CommandKind, CommandState, IntentKind, Limit, Reason, StaffDecisions, StaffOrder,
};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, LandUnitId, ShipId};
use sim_core::orders::{Cargo, OrderKind, TaskForceOrder};
use sim_core::state::{Side, TruthState};
use sim_core::time::Pulse;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

mod common;

const A: Hex = Hex { q: 0, r: 0 };
const B: Hex = Hex { q: 4, r: 0 };

const HOLD: &str = "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_intent\"\nintent = 1\nintent_kind = \"hold\"\nbase = 2\n";
const HOLD_LIMIT: &str = "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\ntask_forces = [1]\nsource = 1\n";
const ALLIED_UNITS_START: &str =
    "# The Allied land units, ashore at B, fighting units on the defence.\n";
const ALLIED_UNITS_END: &str = "# End of the Allied land units\n";

fn path() -> PathBuf {
    common::data_dir().join("scenarios/79_fed_before_it_is_taken.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

/// The text with the Allied land units, units 1 and 2, left out.
fn without_allied_units(t: &str) -> String {
    let start = t.find(ALLIED_UNITS_START).expect("the start marker");
    let end = t.find(ALLIED_UNITS_END).expect("the end marker");
    format!("{}{}", &t[..start], &t[end..])
}

/// The frozen step 1 figures.
fn load(text: &str) -> Scenario {
    common::from_text_with(text, &path(), &common::frozen()).expect("the scenario loads")
}

struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    japanese: BTreeMap<u32, StaffDecisions>,
    /// The Allied command state at the end of each day, from day 1.
    command: BTreeMap<u32, CommandState>,
    truth: Vec<TruthState>,
    events: Vec<Event>,
}

fn play(text: &str) -> Played {
    let mut scenario = load(text);
    let held = staffs();
    let mut played = Played {
        handed: BTreeMap::new(),
        japanese: BTreeMap::new(),
        command: BTreeMap::new(),
        truth: vec![scenario.world.umpire().clone()],
        events: Vec::new(),
    };
    loop {
        let day = scenario.world.day() + 1;
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
        let handed = |side: Side| {
            scenario
                .world
                .staff_handed_in()
                .get(&side)
                .cloned()
                .unwrap_or_default()
        };
        played.handed.insert(day, handed(Side::Allied));
        played.japanese.insert(day, handed(Side::Japanese));
        played
            .command
            .insert(day, scenario.world.command_state(Side::Allied).clone());
        played.truth.push(scenario.world.umpire().clone());
    }
    played.events = scenario.world.events().iter().cloned().collect();
    played
}

/// An order of the staff, plainly.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Given {
    LoadTo(Fixed),
    Route(Vec<Hex>, Vec<Hex>),
    Unload,
}

fn given(o: &StaffOrder) -> Given {
    match &o.kind {
        OrderKind::TaskForce {
            target,
            order:
                TaskForceOrder::LoadTo {
                    cargo: Cargo::Supply,
                    weight_t,
                    ..
                },
        } if target.raw() == 1 => Given::LoadTo(*weight_t),
        OrderKind::TaskForce {
            target,
            order:
                TaskForceOrder::SetRoute {
                    waypoints,
                    act_at,
                    stand_at,
                },
        } if target.raw() == 1 => {
            assert!(stand_at.is_empty(), "no stand mark");
            Given::Route(waypoints.clone(), act_at.iter().copied().collect())
        }
        OrderKind::TaskForce {
            target,
            order: TaskForceOrder::Unload,
        } if target.raw() == 1 => Given::Unload,
        other => panic!("the doctrine gave an order of another kind: {other:?}"),
    }
}

/// A staff command, plainly, by kind.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Cmd {
    Approve(u32),
    Form(u32, IntentKind),
    /// A run of a plan: its ships, its load and its days to load, sail and
    /// arrive.
    AddRun(u32, Vec<u32>, Fixed, (u32, u32, u32)),
    Commit(u32, Vec<u32>),
}

fn cmd(c: &CommandKind) -> Cmd {
    match c {
        CommandKind::Approve { plan } => Cmd::Approve(plan.raw()),
        CommandKind::FormIntent { parent, kind } => Cmd::Form(parent.raw(), *kind),
        CommandKind::AddRun { plan, run } => Cmd::AddRun(
            plan.raw(),
            run.ships.iter().map(|s| s.raw()).collect(),
            run.load_t,
            (run.days.load, run.days.sail, run.days.arrive),
        ),
        CommandKind::CommitShips { plan, ships, .. } => {
            Cmd::Commit(plan.raw(), ships.iter().map(|s| s.raw()).collect())
        }
        CommandKind::SetLimit(Limit::Forces { .. }) | CommandKind::SetLeave { .. } => {
            panic!("the staff handed in a commander's command: {c:?}")
        }
        other => panic!("the staff handed in another command: {other:?}"),
    }
}

fn keep_base_3() -> IntentKind {
    IntentKind::KeepSupplied {
        base: BaseId::new(3),
        floor_days: 5,
        interval_days: None,
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

    /// The no-order reasons of `day` serving `of`.
    fn no_orders(&self, day: u32, of: u32) -> Vec<String> {
        self.handed[&day]
            .no_orders
            .iter()
            .filter(|r| r.serves.intent == Some(IntentId::new(of)))
            .map(text_of)
            .collect()
    }

    fn supply(&self, day: u32, b: u32) -> Fixed {
        self.truth[day as usize].bases[&BaseId::new(b)].supply_t
    }

    fn men_ashore(&self, day: u32, u: u32) -> u32 {
        self.truth[day as usize].land_units[&LandUnitId::new(u)].men_ashore
    }

    fn ship(&self, day: u32, s: u32) -> &sim_core::state::Ship {
        &self.truth[day as usize].ships[&ShipId::new(s)]
    }

    /// Day, pulse, side, base, men, fed, unfed of each feeding.
    fn feedings(&self) -> Vec<(u32, Side, Option<u32>, u32, u32, u32)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::MenFed {
                    side,
                    base,
                    men,
                    fed,
                    unfed,
                    ..
                } => Some((e.day, *side, base.map(|b| b.raw()), *men, *fed, *unfed)),
                _ => None,
            })
            .collect()
    }

    /// Day, base, tons asked and tons paid of each charge for men.
    fn charges(&self, b: u32) -> Vec<(u32, Fixed, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::BaseSupplySpent {
                    base,
                    asked_t,
                    paid_t,
                    men,
                    ..
                } if base.raw() == b && *men > 0 => Some((e.day, *asked_t, *paid_t)),
                _ => None,
            })
            .collect()
    }

    fn arrivals(&self) -> Vec<(Hex, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceArrived {
                    task_force,
                    hex,
                    elapsed_hours,
                } if task_force.raw() == 1 => Some((*hex, *elapsed_hours)),
                EventKind::TaskForceHalted {
                    task_force,
                    hex,
                    elapsed_hours,
                    ..
                } if task_force.raw() == 1 => Some((*hex, *elapsed_hours)),
                _ => None,
            })
            .collect()
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

    /// The "keep supplied" intents in force at the end of `day` whose base
    /// lies at B.
    fn keeping_b(&self, day: u32) -> usize {
        let truth = &self.truth[day as usize];
        self.command[&day]
            .intents_in_force()
            .filter(|i| match i.kind {
                IntentKind::KeepSupplied { base, .. } => {
                    truth.bases.get(&base).map(|b| b.hex) == Some(B)
                }
                _ => false,
            })
            .count()
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

fn main_game() -> &'static Played {
    static GAME: std::sync::OnceLock<Played> = std::sync::OnceLock::new();
    GAME.get_or_init(|| play(&text()))
}

// ---------------------------------------------------------------- The hand calculation

/// Three orders to task force 1 in all, and none to a land unit; no staff
/// order or command is refused.
#[test]
fn three_orders_in_all() {
    let g = main_game();
    assert_eq!(g.days(), 14);
    assert_eq!(
        g.all_orders(),
        vec![
            (3, Given::LoadTo(n(90))),
            (4, Given::Route(vec![B, A], vec![B])),
            (4, Given::Unload),
        ]
    );
    assert!(g.refused().is_empty(), "{:?}", g.refused());
}

/// Item 1: intent 2, keep base 3 supplied at a floor of 5 days, formed under
/// intent 1 on day 1; the run ordered on the morning of day 3.
#[test]
fn intent_2_is_formed_on_day_1_and_one_run_on_day_3() {
    let g = main_game();
    let others: Vec<(u32, Cmd)> = g
        .all_commands()
        .into_iter()
        .filter(|(_, c)| !matches!(c, Cmd::Approve(_)))
        .collect();
    assert_eq!(
        others,
        vec![
            (1, Cmd::Form(1, keep_base_3())),
            (3, Cmd::AddRun(1, vec![1, 2], n(90), (3, 4, 4))),
            (3, Cmd::Commit(1, vec![1, 2])),
        ]
    );
    let formed: Vec<(u32, u32, u32)> = g
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::IntentFormed {
                side: Side::Allied,
                intent,
                parent,
            } => Some((e.day, intent.raw(), parent.raw())),
            _ => None,
        })
        .collect();
    assert_eq!(formed, vec![(1, 2, 1)]);
    assert_says(
        &text_of(g.command_reason(1, &Cmd::Form(1, keep_base_3()))),
        &[
            "base:2 (Placeholder airfield B), which is not the side's",
            "fed through base:3 (Placeholder beach at B)",
            "keep base:3 (Placeholder beach at B) supplied from base:1",
            "a floor of 5 days",
            "15 days",
        ],
    );
}

/// Item 3: the hold's reason says base 2 is not the side's, that the staff
/// waits for it, and that the men are fed through base 3.
#[test]
fn the_hold_says_through_which_base_the_men_are_fed() {
    let g = main_game();
    for day in 1..=14 {
        let reasons = g.no_orders(day, 1);
        assert_eq!(reasons.len(), 1, "day {day}: {reasons:?}");
        assert_says(
            &reasons[0],
            &[
                "base:2 (Placeholder airfield B), which is not the side's",
                "waits for it to be the side's",
                "fed through base:3 (Placeholder beach at B)",
            ],
        );
    }
}

/// The mornings of days 1 to 3: nothing served on day 1; on days 2 and 3
/// the stock read, 42 t and 36 t, and the reading of the day checked, 30 t
/// and 24 t, against a floor of 30 t.
#[test]
fn the_mornings_of_days_1_to_3() {
    let g = main_game();
    // The stock the staff reads on a morning is the stock at the end of the
    // day before.
    assert_eq!([g.supply(1, 3), g.supply(2, 3)], [n(42), n(36)]);
    let day2 = g.no_orders(2, 2);
    assert_eq!(day2.len(), 1, "{day2:?}");
    assert_says(
        &day2[0],
        &[
            "42 t over 6 t a day",
            "the reading of day 4 is projected at 30 t against 5 days x 6 t = 30 t",
        ],
    );
    assert!(g.orders(2).is_empty());
    // A supply intent is first served on the morning after it is formed: on
    // day 1 no run and no supply sum is handed in for intent 2.
    assert!(g.orders(1).is_empty());
    assert!(g.no_orders(1, 2).is_empty(), "{:?}", g.no_orders(1, 2));
    assert!(!g
        .commands(1)
        .iter()
        .any(|c| matches!(c, Cmd::AddRun(..) | Cmd::Commit(..))));
    let run = Cmd::AddRun(1, vec![1, 2], n(90), (3, 4, 4));
    assert_says(
        &text_of(g.command_reason(3, &run)),
        &["36 t over 6 t a day", "24 t"],
    );
}

/// The run: loaded at A on day 3, sails at hour 72, at B at hour 88, 40 t
/// landed in the morning and 50 t in the afternoon into base 3, steams for
/// A at hour 96 and arrives at hour 112.
#[test]
fn the_run() {
    let g = main_game();
    assert_eq!(g.arrivals(), vec![(B, n(88)), (A, n(112))]);
    let loaded: Vec<(u32, u32, Fixed)> = g
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::CargoLoaded {
                ship,
                base,
                loaded_t,
                ..
            } if base.raw() == 1 => Some((e.day, ship.raw(), *loaded_t)),
            _ => None,
        })
        .collect();
    assert_eq!(loaded, vec![(3, 1, n(90))]);
    let landed: Vec<(u32, Option<Pulse>, u32, u32, Fixed)> = g
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::CargoUnloaded {
                ship,
                base,
                cargo: Cargo::Supply,
                unloaded_t,
                ..
            } => Some((e.day, e.pulse, ship.raw(), base.raw(), *unloaded_t)),
            _ => None,
        })
        .collect();
    assert_eq!(
        landed,
        vec![
            (4, Some(Pulse::Morning), 1, 3, n(40)),
            (4, Some(Pulse::Afternoon), 1, 3, n(50)),
        ]
    );
    let steamed_on: Vec<(u32, Option<Pulse>)> = g
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::TaskForceSteamedOn { task_force, .. } if task_force.raw() == 1 => {
                Some((e.day, e.pulse))
            }
            _ => None,
        })
        .collect();
    assert_eq!(steamed_on, vec![(5, Some(Pulse::Night))]);
}

/// Supply, fuel and men.
#[test]
fn supply_fuel_and_men() {
    let g = main_game();
    // Base 3 pays 6 t on each of the 14 nights.
    assert_eq!(
        g.charges(3),
        (1..=14).map(|d| (d, n(6), n(6))).collect::<Vec<_>>()
    );
    let base3: Vec<Fixed> = (1..=4).map(|d| g.supply(d, 3)).collect();
    assert_eq!(base3, vec![n(42), n(36), n(30), n(114)]);
    // A morning's stock from day 5 to 14 is 114 - 6 x (d - 5), never below
    // 42 t.
    for day in 5..=14u32 {
        let morning = g.supply(day - 1, 3);
        assert_eq!(morning, n(114 - 6 * (i64::from(day) - 5)), "day {day}");
        assert!(morning >= n(42), "day {day}");
    }
    assert_eq!(g.supply(14, 3), n(54));
    // Base 1 feeds nobody and loads out 90 t.
    assert!(g.charges(1).is_empty());
    assert_eq!(g.supply(14, 1), n(910));
    // Base 2 feeds 400 men, 2 t a night, and is Japanese at the end.
    assert_eq!(
        g.charges(2),
        (1..=14).map(|d| (d, n(2), n(2))).collect::<Vec<_>>()
    );
    assert_eq!(g.supply(14, 2), n(72));
    assert_eq!(g.truth[14].bases[&BaseId::new(2)].side, Side::Japanese);
    // Each ship steams 320 nm and burns 32 t.
    for s in [1, 2] {
        let burned: Fixed = g
            .events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::FuelBurned { ship, burned_t, .. } if ship.raw() == s => Some(*burned_t),
                _ => None,
            })
            .fold(Fixed::ZERO, |a, b| a + b);
        assert_eq!(burned, n(32), "ship {s}");
        assert_eq!(g.ship(14, s).fuel_t, n(968), "ship {s}");
    }
    let steamed: Fixed = g
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::TaskForceSteamed {
                task_force,
                distance_nm,
                ..
            } if task_force.raw() == 1 => Some(*distance_nm),
            _ => None,
        })
        .fold(Fixed::ZERO, |a, b| a + b);
    assert_eq!(steamed, n(320));
    // Every man is fed every night.
    let feedings = g.feedings();
    assert_eq!(feedings.len(), 28);
    for day in 1..=14 {
        assert!(feedings.contains(&(day, Side::Allied, Some(3), 1200, 1200, 0)));
        assert!(feedings.contains(&(day, Side::Japanese, Some(2), 400, 400, 0)));
    }
    assert_eq!(
        [
            g.men_ashore(14, 1),
            g.men_ashore(14, 2),
            g.men_ashore(14, 3)
        ],
        [600, 600, 400]
    );
    assert!(!g.events.iter().any(|e| matches!(
        e.kind,
        EventKind::MenLostUnfed { .. } | EventKind::MenLanded { .. }
    )));
}

// ---------------------------------------------------------------- The properties

/// No hold stated: nothing formed or ordered; base 3 runs dry after the
/// night of day 8, and on the night of day 9 1,200 Allied men are unfed.
#[test]
fn no_hold_stated() {
    let g = play(&changed(&changed(&text(), HOLD, ""), HOLD_LIMIT, ""));
    assert!(g.all_orders().is_empty());
    assert!(g.all_commands().is_empty(), "{:?}", g.all_commands());
    // Base 3 pays 6 t on the nights of days 1 to 8 and then holds nothing.
    assert_eq!(
        g.charges(3),
        (1..=8).map(|d| (d, n(6), n(6))).collect::<Vec<_>>()
    );
    for day in 1..=8u32 {
        assert_eq!(g.supply(day, 3), n(48 - 6 * i64::from(day)), "day {day}");
    }
    // On the night of day 9 base 3 is asked and pays none (with nothing in
    // stock the engine records no charge, only the feeding): 1,200 Allied
    // men are unfed.
    assert!(g
        .feedings()
        .contains(&(9, Side::Allied, Some(3), 1200, 0, 1200)));
}

/// No man ashore: the hold waits for base 2 to be the side's, as before.
#[test]
fn no_man_ashore() {
    let g = play(&without_allied_units(&text()));
    assert!(g.all_orders().is_empty());
    let others: Vec<(u32, Cmd)> = g
        .all_commands()
        .into_iter()
        .filter(|(_, c)| !matches!(c, Cmd::Approve(_)))
        .collect();
    assert!(others.is_empty(), "{others:?}");
    for day in 1..=14 {
        let reasons = g.no_orders(day, 1);
        assert_eq!(reasons.len(), 1, "day {day}: {reasons:?}");
        assert_says(
            &reasons[0],
            &["waits for it to be the side's, and forms nothing and orders nothing under intent:1 until then"],
        );
        assert!(!reasons[0].contains("fed through"), "day {day}");
    }
}

/// No source: no "keep supplied" is formed, and the hold's reason says that
/// its forces limit names no source.
#[test]
fn no_source() {
    let g = play(&changed(
        &text(),
        "task_forces = [1]\nsource = 1\n",
        "task_forces = [1]\n",
    ));
    assert!(g.all_orders().is_empty());
    assert!(!g
        .all_commands()
        .iter()
        .any(|(_, c)| matches!(c, Cmd::Form(..))));
    assert!(g.refused().is_empty(), "{:?}", g.refused());
    for day in 1..=14 {
        assert!(
            g.no_orders(day, 1)
                .iter()
                .any(|r| r.contains("its forces limit names no source")),
            "day {day}: {:?}",
            g.no_orders(day, 1)
        );
    }
}

/// Formed once: exactly one "keep supplied" in force for the hex on every
/// day from 2 to 14 (item 2).
#[test]
fn formed_once() {
    let g = main_game();
    for day in 2..=14 {
        assert_eq!(g.keeping_b(day), 1, "day {day}");
    }
}

/// Stated as before: the Japanese side, given nothing, is given no order by
/// its staff on any day.
#[test]
fn the_japanese_side_is_given_nothing() {
    let g = main_game();
    for day in 1..=14 {
        assert!(g.japanese[&day].orders.is_empty(), "day {day}");
        assert!(g.japanese[&day].commands.is_empty(), "day {day}");
    }
}
