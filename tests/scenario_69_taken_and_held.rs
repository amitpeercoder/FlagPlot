//! Scenario 69: taken and held. The commander asks for an airfield and
//! asks that it be held, from the base the men sail from. The staff works
//! out that the men can be fed, lands them with nine days of supply and
//! not fifteen, puts them on the defence when the airfield is theirs, and
//! sends the first run of supply after them when it is needed.
//!
//! Proves decision 0144, step 4 of stage 3, second part, against a hand
//! calculation. The data is placeholder data: see
//! `data/scenarios/69_taken_and_held.toml`. It runs on the frozen step 1
//! figures, with one ground figure of its own, `preparation_per_day` 100.
//! No enemy man is ashore anywhere, so no die is rolled.
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
//! A man weighs 0.25 t to load or land and eats 0.005 t a day, paid at
//! the start of the night pulse by the side's bases in the hex in base
//! order. Supply landed goes into the side's lowest-numbered base in the
//! hex. A size 5 port works at 150 t an hour; a size 0 port by lighter at
//! 20 t an hour for each ship. A force with its roster in port gains 100
//! of loading in a day and a unit ready on it 100 of rehearsal. Day d
//! begins at hour 24 x (d - 1): a night pulse of 12 hours and two
//! daylight pulses of 6. In a pulse ships move before cargo is worked. A
//! force that has acted steams on at the start of the next pulse.
//!
//! The staff's figures: at balanced the margin is 5 days; a supply intent
//! with no interval uses 15 days.
//!
//! ## Inputs
//!
//! The placeholder map of scenario 27: A at (0,0), a coast hex with port
//! size 5, and B at (4,0), a coast hex with port size 0, 160 nm apart: 16
//! hours at 10 kn, one day. The game is 14 days.
//!
//! | Base | Side | Hex | Supply | Fuel |
//! | ---- | ---- | --- | ------ | ---- |
//! | 1, "Placeholder base A" | Allied | A | 1,000 t | 0 |
//! | 2, "Placeholder airfield B" | Japanese | B | 0 | 0 |
//!
//! Base 2 has no gun, no aircraft and no man, and is on the Allied map.
//! No base has a reserve. The Allied side holds no estimate of the enemy.
//!
//! Task force 1, Allied, on the transport mission: ships 1 and 2,
//! placeholder troop transports (600 men, 3,000 t of cargo, 10 kn, 0.1 t
//! of fuel a mile, 1,000 t of fuel), stopped at base 1, full of fuel,
//! nothing aboard.
//!
//! Units 1 and 2, Allied, "Placeholder First" and "Placeholder Second":
//! fighting units of 600 men each, experience 2, every skill 2, ashore at
//! A on the defence, fed in full.
//!
//! The Allied side's commands of day 1: intent 1, take the place at base
//! 2, with a forces limit of task force 1, units 1 and 2 and base 1 as
//! the source; intent 2, hold base 2, with a forces limit of task force 1
//! and base 1 as the source. No risk is set on either, so balanced. Leave
//! to prepare and leave to launch are both given.
//!
//! ## The check, on the morning of day 2
//!
//! - The daily need: 1,200 men x 0.005 = 6 t.
//! - The days until supply follows: to land the men, 150 t in the fullest
//!   ship at 20 t an hour, 7.5 hours, 1 day; the passage back, 1; to
//!   load, 1; the passage out, 1. Four days.
//! - The margin: 5 days.
//! - The supply that lands with the men: 6 x (4 + 5) = 54 t.
//! - The first run: 6 x 15 = 90 t.
//! - Base 1 holds 994 t that morning, with no reserve: 994 is more than
//!   54 + 90 = 144. The force's room is 6,000 t. It can be held.
//!
//! ## Day by day
//!
//! | Day | Hour | The staff's commands and orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 | 0 | For intent 1: approves plan 1 to preparing; forms intent 3, ready a landing by task force 1 at B, with a forces limit of units 1 and 2; orders `prepare_landing` at B, roster 1 and 2. For intent 2: approves its plan to preparing and to released; no order: base 2 is not the side's | Loading 100; each unit's rehearsal 100 |
//! | 2 | 24 | Notice: intent 3 is ready. Approves plan 1 to released. Orders a load of supply to a weight of 54 t | Ship 1 loads 54 t |
//! | 3 | 48 | Unit 1 set to attack. Load unit 1 | Ship 1 takes 600 men |
//! | 4 | 72 | Unit 2 set to attack. Load unit 2 | Ship 2 takes 600 men |
//! | 5 | 96 | A route to B and back to A, acting at B, and an unload order | Sails at hour 96, arrives at hour 112. Each ship lands 160 men in the morning and 440 in the afternoon; ship 1 then lands 10 t. Base 3, "Landing at (4,0)", is formed. Base 2 changes side at the end of the day |
//! | 6 | 120 | Marks plan 1 complete; notice: intent 1 is met. For intent 2: units 1 and 2 set to defend; forms intent 4, keep base 2 supplied, under intent 2, with a floor of 5 days | Ship 1 lands 44 t in 2.2 hours. The force steams for A at hour 132 |
//! | 7 | 144 | Intent 4: no run is needed | The force arrives at A at hour 148 |
//! | 8 | 168 | Intent 4: no run is needed | |
//! | 9 | 192 | Intent 4: a run. `add_run`, `commit_ships`; orders a load of supply to a weight of 90 t | Ship 1 loads 90 t |
//! | 10 | 216 | A route to B and back to A, acting at B, and an unload order | Sails at hour 216, arrives at hour 232. Ship 1 lands 40 t in the morning and 50 t in the afternoon |
//! | 11 | 240 | none | The force steams for A at hour 240 and arrives at hour 256 |
//! | 12 to 14 | | none | |
//!
//! Thirteen orders in all. No staff order or command is refused. On each
//! morning from day 2 to day 5 intent 2 has one reason for giving no
//! order: it waits for base 2 to be the side's. From day 6 the hold's
//! reason for no cover is that there is nothing at the base to cover it
//! with, and task force 1 is given no task of its own.
//!
//! The commands that prepare and release the plan of an intent the staff
//! formed are as the tree's rules give them, and are not counted here.
//!
//! ## The supply doctrine's sums for intent 4
//!
//! The use at B is the men ashore, 6 t a day. The floor's stock is
//! 5 x 6 = 30 t. A run ordered on a morning loads that day, sails the
//! next and arrives that same day; the reading checked is the day after.
//!
//! | Morning | The stock at B | The day checked | Its reading | The staff |
//! | --- | --- | --- | --- | --- |
//! | 7 | 48 t | 9 | 48 - 12 = 36 t | Not below 30: no run |
//! | 8 | 42 t | 10 | 42 - 12 = 30 t | Not below 30: no run |
//! | 9 | 36 t | 11 | 36 - 12 = 24 t | Below 30: a run |
//!
//! The run's load: the floor's stock on the day it arrives, day 10, 30 t,
//! less that day's reading, 30 t, which is 0; plus 15 days of use, 90 t.
//! Ship 1's room covers it; ship 2 goes with its force.
//!
//! ## Supply and fuel
//!
//! - **Base 1** feeds 1,200 men, 6 t, on the nights of days 1 to 3, and
//!   600 men, 3 t, on day 4: 21 t. It loads out 54 t and 90 t. It ends
//!   with 1,000 - 21 - 144 = 835 t. On the morning of day 2 it holds
//!   1,000 - 6 = 994 t.
//! - **Base 3** is given 10 t on the afternoon of day 5. On the night of
//!   day 6 base 2, now Allied and the lower numbered, holds nothing, and
//!   base 3 pays 6 t: 4 t, which it keeps to the end.
//! - **Base 2** takes the 44 t landed on the night of day 6, after that
//!   night's charge. It pays 6 t on each night from day 7: 38, 32, 26 and
//!   20 t after the nights of days 7 to 10. It takes 90 t on day 10:
//!   110 t. It pays 6 t on the nights of days 11 to 14: 86 t at the end.
//! - **The stock at B** on the mornings of days 7, 8, 9 and 10 is 48, 42,
//!   36 and 30 t.
//! - **Each ship** steams 320 nm twice and burns 64 t: 936 t at the end.
//!
//! # Properties
//!
//! - **No hold stated.** With intent 2 left out: the check is not made
//!   and the load of day 2 is to a weight of 6 x 15 = 90 t. Days 1 to 5
//!   are otherwise as in the main case. On day 6 the plan is marked
//!   complete and nothing else is ordered then or after: eight orders in
//!   all, and units 1 and 2 are on attack at the end.
//! - **A hold with no source.** With intent 2's forces limit left out:
//!   the load of day 2 is 90 t. On day 6 units 1 and 2 are set to defend,
//!   no supply intent is formed, and the hold's reason says that no other
//!   base of the side can be the source (base 1 is the source of intent 1,
//!   higher in priority), so base 2 lives on what it has, and offers no
//!   source to name (decision 0222, rule 1). Ten orders in all.
//! - **A hold with no source and a base that could be one.** As above, with
//!   an Allied base 3 at C holding 500 t and no reserve: the same ten
//!   orders, and the hold's reason names base 3, with its 500 t above its
//!   reserve, as a base a source may be named among (decision 0222, rule
//!   1).
//! - **It cannot be held, at balanced.** With base 1 at 100 t of supply:
//!   on the morning of day 2 it holds 94 t against 144 t wanted, short by
//!   50 t. The staff says so, releases the plan and orders the load of
//!   54 t.
//! - **It cannot be held, at preserve the force.** With base 1 at 100 t
//!   and both intents at preserve the force: the margin is 10 days, the
//!   supply that lands with the men 6 x 14 = 84 t, and 84 + 90 = 174 t
//!   are wanted against 94 t, short by 80 t. The staff does not release
//!   the plan: it hands in the notice that it cannot meet intent 1 on the
//!   morning of day 2, gives the reason on each morning after, and
//!   nothing sails.
//! - **Press.** With both intents at press: the margin is 0, the load of
//!   day 2 is 6 x 4 = 24 t, and intent 4 is formed with a floor of 0
//!   days.
//! - **A hold of a base already the side's, with no man there,** plays as
//!   it did: every scenario built before this one gives the events it
//!   gave, scenarios 67 and 68 among them.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see give the same orders and reasons; a
//!   save on day 8 and a resume give the same game.

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::command::{
    CommandKind, IntentKind, Limit, NoticeKind, PlanOutcome, PlanState, Reason, StaffDecisions,
    StaffOrder, Staffs,
};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, LandUnitId, ShipId, TaskForceId};
use sim_core::orders::{Cargo, LandUnitOrder, OrderKind, Posture, TaskForceOrder};
use sim_core::state::{Side, TruthState};
use sim_core::time::Pulse;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

mod common;

const COMMANDS_MARKER: &str = "# The Allied commander's commands";
const A: Hex = Hex { q: 0, r: 0 };
const B: Hex = Hex { q: 4, r: 0 };

fn path() -> PathBuf {
    common::data_dir().join("scenarios/69_taken_and_held.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

/// A number in tenths: x(1222) is 122.2.
fn x(tenths: i64) -> Fixed {
    Fixed::from_ratio(tenths, 10)
}

fn unit(raw: u32) -> LandUnitId {
    LandUnitId::new(raw)
}

fn intent(raw: u32) -> IntentId {
    IntentId::new(raw)
}

fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

/// The frozen step 1 figures. The scenario names its own ground figures,
/// `data/reference/ground_placeholder_scenario_69.toml`, frozen beside the
/// others: the figures its hand calculation names.
fn figures() -> sim_data::LoadOptions {
    common::frozen()
}

fn load(text: &str) -> Scenario {
    common::from_text_with(text, &path(), &figures()).expect("the scenario loads")
}

struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    truth: Vec<TruthState>,
    events: Vec<Event>,
    hashes: Vec<u64>,
}

fn play(text: &str) -> Played {
    play_from(load(text), BTreeMap::new())
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

/// An order of the staff, plainly.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Given {
    Landing(u32, Hex, Vec<u32>),
    LoadTo(Fixed),
    Attack(u32),
    Defend(u32),
    LoadUnit(u32),
    Route(Vec<Hex>, Vec<Hex>),
    Unload,
}

fn given(o: &StaffOrder) -> Given {
    match &o.kind {
        OrderKind::TaskForce {
            target,
            order: TaskForceOrder::PrepareLanding { hex, roster },
        } => Given::Landing(target.raw(), *hex, roster.iter().map(|u| u.raw()).collect()),
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
        OrderKind::LandUnit {
            target,
            order:
                LandUnitOrder::SetPosture {
                    posture: Posture::Defend,
                    when_spent: None,
                },
        } => Given::Defend(target.raw()),
        OrderKind::TaskForce {
            order: TaskForceOrder::LoadUnit { unit, cargo: None },
            ..
        } => Given::LoadUnit(unit.raw()),
        OrderKind::TaskForce {
            order:
                TaskForceOrder::SetRoute {
                    waypoints,
                    act_at,
                    stand_at,
                },
            ..
        } => {
            assert!(stand_at.is_empty(), "no stand mark");
            Given::Route(waypoints.clone(), act_at.iter().copied().collect())
        }
        OrderKind::TaskForce {
            order: TaskForceOrder::Unload,
            ..
        } => Given::Unload,
        other => panic!("the doctrine gave an order of another kind: {other:?}"),
    }
}

fn route() -> Given {
    Given::Route(vec![B, A], vec![B])
}

/// A staff command, plainly, by kind.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Cmd {
    Approve(u32),
    Form(u32, IntentKind),
    Limit(u32, Vec<u32>),
    Complete(u32),
    /// A run of a plan: its ships, its load and its days to load, sail and
    /// arrive.
    AddRun(u32, Vec<u32>, Fixed, (u32, u32, u32)),
    Commit(u32, Vec<u32>),
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
        CommandKind::MarkPlan {
            plan,
            outcome: PlanOutcome::Complete,
        } => Cmd::Complete(plan.raw()),
        CommandKind::AddRun { plan, run } => Cmd::AddRun(
            plan.raw(),
            run.ships.iter().map(|s| s.raw()).collect(),
            run.load_t,
            (run.days.load, run.days.sail, run.days.arrive),
        ),
        CommandKind::CommitShips { plan, ships, .. } => {
            Cmd::Commit(plan.raw(), ships.iter().map(|s| s.raw()).collect())
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

fn keep_kind(floor_days: u32) -> IntentKind {
    IntentKind::KeepSupplied {
        base: BaseId::new(2),
        floor_days,
        interval_days: None,
    }
}

/// Whether a command prepares or releases a plan: the commands of an
/// intent's plan, as the tree's rules give them.
fn approves(c: &Cmd) -> bool {
    matches!(c, Cmd::Approve(_))
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

    /// What would change the no-order reasons of `day` serving `of`.
    fn would_change(&self, day: u32, of: u32) -> Vec<String> {
        self.handed[&day]
            .no_orders
            .iter()
            .filter(|r| r.serves.intent == Some(intent(of)))
            .flat_map(|r| r.would_change.iter().cloned())
            .collect()
    }

    fn unit_state(&self, day: u32, u: u32) -> &sim_core::state::LandUnit {
        &self.truth[day as usize].land_units[&unit(u)]
    }

    fn base(&self, day: u32, b: u32) -> Option<&sim_core::state::Base> {
        self.truth[day as usize].bases.get(&BaseId::new(b))
    }

    fn supply(&self, day: u32, b: u32) -> Fixed {
        self.base(day, b).expect("the base exists").supply_t
    }

    fn ship(&self, day: u32, s: u32) -> &sim_core::state::Ship {
        &self.truth[day as usize].ships[&ShipId::new(s)]
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

    fn changed_side_day(&self, b: u32) -> Option<u32> {
        self.events.iter().find_map(|e| match &e.kind {
            EventKind::BaseChangedSide { base, .. } if base.raw() == b => Some(e.day),
            _ => None,
        })
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

    fn unloading_complete(&self) -> Vec<(u32, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::UnloadingComplete {
                    task_force,
                    base,
                    elapsed_hours,
                } if task_force.raw() == 1 => Some((base.raw(), *elapsed_hours)),
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

fn risk(intent: u32, level: &str) -> String {
    format!(
        "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"risk\"\nintent = {intent}\nrisk = \"{level}\"\n"
    )
}

const HOLD: &str = "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_intent\"\nintent = 2\nintent_kind = \"hold\"\nbase = 2\n";
const HOLD_LIMIT: &str = "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"forces\"\nintent = 2\ntask_forces = [1]\nsource = 1\n";

fn base_1_at(t: &str, supply: u32) -> String {
    changed(
        t,
        "name = \"Placeholder base A\"\nside = \"allied\"\nq = 0\nr = 0\nsupply_t = 1000\n",
        &format!(
            "name = \"Placeholder base A\"\nside = \"allied\"\nq = 0\nr = 0\nsupply_t = {supply}\n"
        ),
    )
}

fn main_game() -> &'static Played {
    static GAME: std::sync::OnceLock<Played> = std::sync::OnceLock::new();
    GAME.get_or_init(|| play(&text()))
}

fn main_orders() -> Vec<(u32, Given)> {
    vec![
        (1, Given::Landing(1, B, vec![1, 2])),
        (2, Given::LoadTo(n(54))),
        (3, Given::Attack(1)),
        (3, Given::LoadUnit(1)),
        (4, Given::Attack(2)),
        (4, Given::LoadUnit(2)),
        (5, route()),
        (5, Given::Unload),
        (6, Given::Defend(1)),
        (6, Given::Defend(2)),
        (9, Given::LoadTo(n(90))),
        (10, route()),
        (10, Given::Unload),
    ]
}

// ---------------------------------------------------------------- The hand calculation

#[test]
fn thirteen_orders_in_all() {
    let g = main_game();
    assert_eq!(g.days(), 14);
    assert_eq!(g.all_orders(), main_orders());
    assert!(g.refused().is_empty(), "{:?}", g.refused());
}

#[test]
fn the_commands_by_kind_and_day() {
    let g = main_game();
    // The commands that prepare and release a plan are as the tree's rules
    // give them: on day 1, plan 1 to preparing and plan 2 to preparing and
    // to released; on day 2, plan 1 to released. They are listed apart.
    let approvals: Vec<(u32, Cmd)> = g
        .all_commands()
        .into_iter()
        .filter(|(_, c)| approves(c))
        .collect();
    assert_eq!(
        approvals,
        vec![
            (1, Cmd::Approve(1)),
            (1, Cmd::Approve(2)),
            (1, Cmd::Approve(2)),
            (2, Cmd::Approve(1)),
        ]
    );
    let others: Vec<(u32, Cmd)> = g
        .all_commands()
        .into_iter()
        .filter(|(_, c)| !approves(c))
        .collect();
    assert_eq!(
        others,
        vec![
            (1, Cmd::Form(1, landing_kind())),
            (1, Cmd::Limit(3, vec![1, 2])),
            (6, Cmd::Form(2, keep_kind(5))),
            (6, Cmd::Complete(1)),
            (9, Cmd::AddRun(2, vec![1, 2], n(90), (9, 10, 10))),
            (9, Cmd::Commit(2, vec![1, 2])),
        ]
    );
    // Intent 3 is the landing under intent 1; intent 4 the supply under
    // intent 2.
    let formed: Vec<(u32, u32, u32)> = g
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::IntentFormed {
                side: Side::Allied,
                intent: i,
                parent: p,
            } => Some((e.day, i.raw(), p.raw())),
            _ => None,
        })
        .collect();
    assert_eq!(formed, vec![(1, 3, 1), (6, 4, 2)]);
    assert_eq!(g.plan_day(1, PlanState::Released), Some(2));
    assert_eq!(g.plan_day(1, PlanState::Complete), Some(6));
    assert_eq!(g.plan_day(2, PlanState::Released), Some(1));
}

#[test]
fn the_notices() {
    let g = main_game();
    assert_eq!(
        g.all_notices(),
        vec![(2, NoticeKind::Ready, 1), (6, NoticeKind::Met, 1)]
    );
}

#[test]
fn the_check_on_the_morning_of_day_2() {
    let g = main_game();
    let release = text_of(g.command_reason(2, &Cmd::Approve(1)));
    assert_says(
        &release,
        &[
            "the daily need of the 1200 men chosen is 6 t",
            "1 to land the men (150 t in the fullest ship at 20 t an hour = 7.5 hours)",
            "1 back to base:1",
            "1 to load and 1 out: 4 days",
            "the margin at balanced is 5 days",
            "6 t x (4 + 5) = 54 t",
            "the first run is 6 t x 15 = 90 t",
            "base:1 holds 994 t above its reserve against 54 t + 90 t = 144 t",
            "room is 6000 t",
            "it can be held",
        ],
    );
    assert_says(
        &text_of(g.order_reason(2, &Given::LoadTo(n(54)))),
        &["for 9 days"],
    );
}

#[test]
fn the_hold_waits_and_then_holds() {
    let g = main_game();
    for day in 1..=5 {
        let reasons = g.no_orders(day, 2);
        assert_eq!(reasons.len(), 1, "day {day}: {reasons:?}");
        assert_says(&reasons[0], &["waits for it to be the side's"]);
    }
    for day in 6..=14 {
        let reasons = g.no_orders(day, 2);
        assert!(
            reasons.iter().any(|r| r
                .contains("There is nothing at base:2 (Placeholder airfield B) to cover it with")),
            "day {day}: {reasons:?}"
        );
        // Task force 1 is given no task of its own.
        assert!(
            !g.commands(day)
                .iter()
                .any(|c| matches!(c, Cmd::Form(2, IntentKind::StandAt { .. }))),
            "day {day}"
        );
    }
    assert_says(
        &text_of(g.order_reason(6, &Given::Defend(1))),
        &["intent:2 holds base:2", "to defend"],
    );
    assert_says(
        &text_of(g.command_reason(6, &Cmd::Form(2, keep_kind(5)))),
        &["a floor of 5 days", "15 days"],
    );
    for day in 7..=14 {
        assert_eq!(g.unit_state(day, 1).posture, Posture::Defend);
        assert_eq!(g.unit_state(day, 2).posture, Posture::Defend);
    }
}

#[test]
fn the_supply_doctrine_sums_for_intent_4() {
    let g = main_game();
    for day in [7, 8] {
        let reasons = g.no_orders(day, 4);
        assert_eq!(reasons.len(), 1, "day {day}: {reasons:?}");
    }
    // The stock at B on the mornings of days 7 to 10: 48, 42, 36 and 30 t.
    let at_b = |day: u32| g.supply(day, 2) + g.supply(day, 3);
    assert_eq!(
        [at_b(6), at_b(7), at_b(8), at_b(9)],
        [n(48), n(42), n(36), n(30)]
    );
}

#[test]
fn the_passages_and_the_landings() {
    let g = main_game();
    assert_eq!(
        g.arrivals(),
        vec![(B, n(112)), (A, n(148)), (B, n(232)), (A, n(256))]
    );
    let landed: Vec<(u32, Pulse, u32, u32)> = g
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::MenLanded { unit, men, .. } => Some((e.day, e.pulse?, unit.raw(), *men)),
            _ => None,
        })
        .collect();
    assert_eq!(
        landed,
        vec![
            (5, Pulse::Morning, 1, 160),
            (5, Pulse::Morning, 2, 160),
            (5, Pulse::Afternoon, 1, 440),
            (5, Pulse::Afternoon, 2, 440),
        ]
    );
    assert_eq!(g.changed_side_day(2), Some(5));
    // 44 t in 2.2 hours of the night of day 6; 40 t in the morning of day 10
    // and 50 t in the afternoon, complete 2.5 hours in.
    assert_eq!(g.unloading_complete(), vec![(2, x(1222)), (2, x(2365))]);
    assert_eq!(g.ship(9, 1).supply_t, n(90));
    assert_eq!(g.ship(9, 2).supply_t, Fixed::ZERO);
    for s in [1, 2] {
        assert_eq!(g.ship(14, s).fuel_t, n(936));
    }
}

#[test]
fn supply_and_fuel() {
    let g = main_game();
    let base1 = [
        994, 934, 928, 925, 925, 925, 925, 925, 835, 835, 835, 835, 835, 835,
    ];
    for day in 1..=14u32 {
        assert_eq!(
            g.supply(day, 1),
            n(base1[(day - 1) as usize]),
            "base 1 after day {day}"
        );
    }
    assert_eq!(g.supply(5, 3), n(10));
    for day in 6..=14 {
        assert_eq!(g.supply(day, 3), n(4), "base 3 after day {day}");
    }
    let base2 = [44, 38, 32, 26, 110, 104, 98, 92, 86];
    for day in 6..=14u32 {
        assert_eq!(
            g.supply(day, 2),
            n(base2[(day - 6) as usize]),
            "base 2 after day {day}"
        );
    }
}

// ---------------------------------------------------------------- The properties

#[test]
fn no_hold_stated() {
    let t = changed(&changed(&text(), HOLD, ""), HOLD_LIMIT, "");
    let g = play(&t);
    assert_eq!(g.orders(2), vec![Given::LoadTo(n(90))]);
    let mut wanted = main_orders()[..8].to_vec();
    wanted[1] = (2, Given::LoadTo(n(90)));
    assert_eq!(g.all_orders(), wanted);
    assert!(g.commands(6).contains(&Cmd::Complete(1)));
    assert!(!text_of(g.command_reason(2, &Cmd::Approve(1))).contains("can be held"));
    for u in [1, 2] {
        assert_eq!(g.unit_state(14, u).posture, Posture::Attack);
    }
}

#[test]
fn a_hold_with_no_source() {
    let g = play(&changed(&text(), HOLD_LIMIT, ""));
    let mut wanted = main_orders()[..10].to_vec();
    wanted[1] = (2, Given::LoadTo(n(90)));
    assert_eq!(g.all_orders(), wanted);
    assert!(!g
        .all_commands()
        .iter()
        .any(|(_, c)| matches!(c, Cmd::Form(2, _))));
    let reasons = g.no_orders(6, 2);
    assert!(
        reasons.iter().any(|r| r.contains(
            "No other base of the side can be the source (base:1 (Placeholder base A) is the source of intent:1, an intent higher in priority), so base:2 (Placeholder airfield B) lives on what it has"
        )),
        "{reasons:?}"
    );
    assert!(
        g.would_change(6, 2)
            .contains(&"a base of the side with supply above its reserve".to_string()),
        "{:?}",
        g.would_change(6, 2)
    );
    assert!(
        !reasons
            .iter()
            .any(|r| r.contains("name a source") || r.contains("may be named")),
        "{reasons:?}"
    );
}

/// Decision 0222, rule 1: with a base of the side that could be the source,
/// the hold's reason names it, with its supply above its reserve, and the
/// staff's orders are those of the hold with no source.
#[test]
fn a_hold_with_no_source_and_a_base_that_could_be_one() {
    let base_3 = "[[base]]\nid = 3\nname = \"Placeholder base C\"\nside = \"allied\"\nq = 0\nr = 6\nsupply_t = 500\nfuel_t = 0\n\n# Japanese base 2 at B";
    let t = changed(
        &changed(&text(), HOLD_LIMIT, ""),
        "# Japanese base 2 at B",
        base_3,
    );
    let g = play(&t);
    let mut wanted = main_orders()[..10].to_vec();
    wanted[1] = (2, Given::LoadTo(n(90)));
    assert_eq!(g.all_orders(), wanted);
    let reasons = g.no_orders(6, 2);
    assert!(
        reasons.iter().any(|r| r.contains(
            "A source may be named in the forces limit among the bases of the side that could be one: base:3 (Placeholder base C), 500 t of supply above its reserve."
        ) && r.contains("base:3 (Placeholder base C) could be the source: 500 t of supply above its reserve")),
        "{reasons:?}"
    );
    assert!(
        g.would_change(6, 2)
            .contains(&"a source named among base:3 (Placeholder base C)".to_string()),
        "{:?}",
        g.would_change(6, 2)
    );
}

#[test]
fn it_cannot_be_held_at_balanced() {
    let g = play(&base_1_at(&text(), 100));
    assert_says(
        &text_of(g.command_reason(2, &Cmd::Approve(1))),
        &[
            "base:1 has 94 t above its reserve against the 54 t + 90 t = 144 t wanted, short by 50 t",
            "it cannot be held",
            "At balanced the staff goes all the same",
        ],
    );
    assert_eq!(g.plan_day(1, PlanState::Released), Some(2));
    assert_eq!(g.orders(2), vec![Given::LoadTo(n(54))]);
}

#[test]
fn it_cannot_be_held_at_preserve_the_force() {
    let t = with_commands(
        &with_commands(&base_1_at(&text(), 100), &risk(1, "preserve")),
        &risk(2, "preserve"),
    );
    let g = play(&t);
    assert_eq!(
        g.all_notices()
            .into_iter()
            .filter(|(_, k, _)| *k == NoticeKind::CannotMeet)
            .collect::<Vec<_>>(),
        vec![(2, NoticeKind::CannotMeet, 1)]
    );
    let day2 = g.no_orders(2, 1);
    assert!(
        day2.iter().any(
            |r| r.contains("the 84 t + 90 t = 174 t wanted, short by 80 t")
                && r.contains("the margin at preserve is 10 days")
        ),
        "{day2:?}"
    );
    for day in 3..=14 {
        assert!(
            g.no_orders(day, 1)
                .iter()
                .any(|r| r.contains("it cannot be held")),
            "day {day}"
        );
    }
    assert_eq!(g.plan_day(1, PlanState::Released), None);
    assert_eq!(g.all_orders(), vec![(1, Given::Landing(1, B, vec![1, 2]))]);
}

#[test]
fn press() {
    let t = with_commands(
        &with_commands(&text(), &risk(1, "press")),
        &risk(2, "press"),
    );
    let g = play(&t);
    assert_eq!(g.orders(2), vec![Given::LoadTo(n(24))]);
    assert!(g.commands(6).contains(&Cmd::Form(2, keep_kind(0))));
    assert!(g.refused().is_empty(), "{:?}", g.refused());
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
