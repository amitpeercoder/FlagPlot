//! Scenario 81: told to wait. The commander asks for an airfield and
//! tells the staff not to go before day 6. The staff makes the landing
//! ready at once, holds it, and releases it on the morning of day 6.
//! Other cases have it wait for a force, or for another place to be
//! taken first.
//!
//! Proves decision 0159, rule C (items 9 to 11), step 3 of the opening of
//! stage 4, against a hand calculation. The data is placeholder data: see
//! `data/scenarios/81_told_to_wait.toml`. It runs on the frozen step 1
//! figures with one ground figure of its own, `preparation_per_day` 100,
//! as scenario 69 does. No Japanese man is ashore anywhere and no
//! estimate is held, so no die is rolled and the staff takes every unit
//! its ships can carry.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## The three limits
//!
//! Each is a limit on a "take a place" or a "control an island" of the
//! commander's, read up the tree as the other limits are, and refused on
//! any other kind of intent.
//!
//! - **Not before** a day: unmet on a morning before that day.
//! - **Wait for** a task force or a land unit of the side: unmet until
//!   the side may use it and it lies at the source the intent's forces
//!   limit names (a force stopped there, a unit ashore there).
//! - **After** another "take a place" or "control an island" of the
//!   commander's: unmet until, on a morning, that intent's place is the
//!   side's (for an island, every place of it).
//!
//! While a limit is unmet the staff readies the landing and does not
//! release its plan: the plan stays preparing, leave to launch is not
//! asked, and each morning's reason names what is waited for. On the
//! first morning every limit is met, the take goes on as it does today.
//!
//! ## Figures
//!
//! As scenario 80's.
//!
//! ## Inputs
//!
//! The placeholder map of scenario 27: A at (0,0), port size 5; B at
//! (4,0), port size 0, 160 nm from A, 16 hours at 10 kn; C at (0,6), port
//! size 0, 240 nm from A, 24 hours. The game is 13 days.
//!
//! | Base | Side | Hex | Supply |
//! | ---- | ---- | --- | ------ |
//! | 1, "Placeholder base A" | Allied | A | 1,000 t |
//! | 2, "Placeholder airfield B" | Japanese | B | 0 |
//!
//! Base 2 has no gun, no aircraft and no man, and is on the Allied map.
//! No estimate is held.
//!
//! Task force 1, Allied, on the transport mission: ships 1 and 2,
//! placeholder troop transports (600 men, 3,000 t of cargo, 10 kn, 0.1 t
//! of fuel a mile, 1,000 t of fuel), stopped at base 1, nothing aboard.
//!
//! Units 1 and 2, Allied, fighting units of 600 men each, experience 2,
//! every skill 2, ashore at A on the defence, usable from day 1.
//!
//! The Allied side's commands of day 1: intent 1, take the place at base
//! 2, with a forces limit of task force 1, units 1 and 2 and base 1 as
//! the source; no risk, so balanced; no latest day; and the limit "not
//! before day 6" on intent 1. Leave to prepare and leave to launch are
//! both given.
//!
//! ## Day by day
//!
//! | Day | Hour | The staff's commands and orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 | 0 | Approves plan 1 to preparing. Forms intent 2, ready a landing by task force 1 at B, with a forces limit of units 1 and 2. Orders `prepare_landing` at B, roster 1 and 2 | Loading 100; each unit's rehearsal 100 |
//! | 2 | 24 | Notice: intent 2 is ready. The plan is not released: the reason says it is not to go before day 6 | |
//! | 3 to 5 | | none; the same reason; no notice | |
//! | 6 | 120 | Approves plan 1 to released. Orders a load of supply to a weight of 90 t | Ship 1 loads 90 t |
//! | 7 | 144 | Unit 1 set to attack. Load unit 1 | Ship 1 takes 600 men |
//! | 8 | 168 | Unit 2 set to attack. Load unit 2 | Ship 2 takes 600 men |
//! | 9 | 192 | A route to B and back to A, acting at B, and an unload order | Sails at hour 192, arrives at hour 208. Each ship lands 160 men in the morning and 440 in the afternoon; ship 1 then lands 10 t. Base 3, "Landing at (4,0)", is formed. Base 2 changes side at the end of the day |
//! | 10 | 216 | Marks plan 1 complete. Notice: intent 1 is met | Ship 1 lands 80 t in 4 hours: complete at hour 220. The force steams for A at hour 228 |
//! | 11 | 240 | none | The force arrives at A at hour 244 |
//! | 12, 13 | | none | |
//!
//! Eight orders in all, one on day 1 and none on days 2 to 5. No staff
//! order or command is refused. On days 2 to 5 the plan's state is
//! preparing and no notice that it waits for leave is handed in.
//!
//! ## Supply and fuel
//!
//! - **Base 1** feeds 1,200 men, 6 t, on the nights of days 1 to 7, and
//!   600 men, 3 t, on the night of day 8: 45 t. It loads out 90 t on day
//!   6. It holds 874 t at the end of day 6 and 865 t at the end.
//! - **Base 3** takes 10 t on the afternoon of day 9 and pays 6 t on the
//!   night of day 10: 4 t at the end.
//! - **Base 2** takes 80 t on the night of day 10, after that night's
//!   charge, and pays 6 t on the nights of days 11, 12 and 13: 62 t at
//!   the end. It is Allied from the end of day 9.
//! - **Each ship** steams 320 nm and burns 32 t: 968 t at the end.
//!
//! # Properties
//!
//! - **No limit.** With no "not before": day 1 as above; on day 2 the
//!   notice that intent 2 is ready, plan 1 approved to released, and a
//!   load of supply to a weight of 90 t. Days 1 and 2 are asserted.
//! - **The limit lifted.** As the main case, with the commander's command
//!   of day 3 that clears "not before" on intent 1: on day 3 plan 1 is
//!   approved to released and the load of 90 t is ordered. Days 1 to 3
//!   are asserted.
//! - **Leave withheld.** As the main case, with leave to launch withheld:
//!   on days 2 to 5 no notice that plan 1 waits for leave is handed in;
//!   on day 6 it is, and the plan is not released. Days 1 to 6 are
//!   asserted.
//! - **Wait for a force.** With task force 2, Allied, on the surface
//!   combat mission, one placeholder cruiser, ship 3, stopped at base 1
//!   and usable from day 5; no "not before"; and the limit "wait for task
//!   force 2" on intent 1. Task force 2 is not in intent 1's forces
//!   limit and is given no order. Day 1 as the main case. On days 2, 3
//!   and 4 the plan is not released and the reason names task force 2
//!   and day 5. On day 5 plan 1 is approved to released and the load of
//!   90 t is ordered. Days 1 to 5 are asserted.
//! - **After another place.** With base 4, Japanese, at C, on the Allied
//!   map, with nothing at it; task force 3, Allied, on the transport
//!   mission, ship 4, a placeholder troop transport, stopped at base 1
//!   with nothing aboard; unit 3, Allied, a fighting unit of 600 men
//!   ashore at A; intent 3, stated on day 1 after intent 1, take the
//!   place at base 4, with a forces limit of task force 3, unit 3 and
//!   base 1 as the source; no "not before"; and the limit "after intent
//!   3" on intent 1. Intent 3 goes as a take does: readied on day 1,
//!   released on day 2 with a load of 600 x 0.005 x 15 = 45 t, unit 3
//!   embarked on day 3, sailed at hour 72. It arrives at C at hour 96, as
//!   the afternoon of day 4 ends, with no hour left to work cargo. In the
//!   night of day 5 it lands 600 men in 7.5 hours and then 45 t in 2.25
//!   hours. Base 4 is Allied at the end of day 5. Intent 1: day 1 as the
//!   main case; on days 2 to 5 the plan is not released and the reason
//!   names intent 3; on day 6 plan 1 is approved to released and the load
//!   of 90 t is ordered. Days 1 to 6 are asserted for intent 1, and for
//!   intent 3 the hours and the day base 4 changes side.
//! - **A wait that can never end.** As "After another place", with the
//!   commander's command of day 2 that withdraws intent 3: a command of
//!   a day is taken in before the staff is asked that morning, so on day
//!   2 the staff hands in one notice that intent 1 cannot be met as it
//!   stands, naming the limit and the withdrawn intent; on days 3 and 4
//!   none. Plan 1 is preparing on every day to day 6 and is not
//!   released. Days 1 to 6 are asserted. (Restated by the architect on
//!   Nov 18, 2026, on reading the build: the first writing had the
//!   notice on day 3, a day after the command, which is not how a
//!   command is taken in, as "The limit lifted" above already has it.)
//! - **Wait for a unit.** With unit 3, Allied, a fighting unit of 600
//!   men ashore at A and usable from day 5, not in intent 1's forces
//!   limit; no "not before"; and the limit "wait for land unit 3" on
//!   intent 1. As "Wait for a force": on days 2, 3 and 4 the plan is not
//!   released and the reason names unit 3 and day 5; on day 5 plan 1 is
//!   approved to released and the load of 90 t is ordered. Unit 3 is
//!   given no order. Days 1 to 5 are asserted.
//! - **No ring of waits.** An "after" that would have two intents each
//!   wait for the other, directly or through a third, is refused at
//!   intake as an "after" naming its own intent is.
//! - **Refused at intake.** A "not before" on a "hold" is refused. An
//!   "after" that names its own intent, a "hold", or an intent of the
//!   staff's is refused. A "wait for" that names a task force of the
//!   other side is refused with the reason a forces limit naming it is
//!   refused with, so that the refusal tells nothing of the enemy.
//! - The same start gives the same orders; a save on day 3 and a resume
//!   give the same game; with no such limit set, scenarios 67 to 80 give
//!   the orders they gave.

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::command::{
    CommandKind, CommandRefusal, IntentKind, Leave, Limit, NoticeKind, PlanOutcome, PlanState,
    Reason, StaffDecisions, StaffOrder, Staffs,
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

use crate::common;

const A: Hex = Hex { q: 0, r: 0 };
const B: Hex = Hex { q: 4, r: 0 };
const C: Hex = Hex { q: 0, r: 6 };

fn path() -> PathBuf {
    common::data_dir().join("scenarios/81_told_to_wait.toml")
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

fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

/// The frozen step 1 figures. The scenario names scenario 69's ground
/// figures, `data/reference/ground_placeholder_scenario_69.toml`, frozen
/// beside the others: the figures its hand calculation names.
fn figures() -> sim_data::LoadOptions {
    common::frozen()
}

fn load(text: &str) -> Scenario {
    common::from_text_with(text, &path(), &figures()).expect("the scenario loads")
}

struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    japanese: BTreeMap<u32, StaffDecisions>,
    truth: Vec<TruthState>,
    events: Vec<Event>,
    hashes: Vec<u64>,
}

fn play(text: &str) -> Played {
    play_from(load(text), BTreeMap::new())
}

fn play_from(mut scenario: Scenario, mut handed: BTreeMap<u32, StaffDecisions>) -> Played {
    let held = staffs();
    let mut japanese = BTreeMap::new();
    let mut truth = vec![scenario.world.umpire().clone()];
    loop {
        let day = scenario.world.day() + 1;
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
        let of = |side: Side| {
            scenario
                .world
                .staff_handed_in()
                .get(&side)
                .cloned()
                .unwrap_or_default()
        };
        handed.insert(day, of(Side::Allied));
        japanese.insert(day, of(Side::Japanese));
        truth.push(scenario.world.umpire().clone());
    }
    Played {
        handed,
        japanese,
        truth,
        events: scenario.world.events().iter().cloned().collect(),
        hashes: scenario.world.day_hashes().to_vec(),
    }
}

/// An order of the staff, plainly, with the task force or unit it is for.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Given {
    Landing(u32, Hex, Vec<u32>),
    LoadTo(u32, Fixed),
    Attack(u32),
    LoadUnit(u32, u32),
    Route(u32, Vec<Hex>, Vec<Hex>),
    Unload(u32),
}

fn given(o: &StaffOrder) -> Given {
    match &o.kind {
        OrderKind::TaskForce {
            target,
            order: TaskForceOrder::PrepareLanding { hex, roster },
        } => Given::Landing(target.raw(), *hex, roster.iter().map(|u| u.raw()).collect()),
        OrderKind::TaskForce {
            target,
            order:
                TaskForceOrder::LoadTo {
                    cargo: Cargo::Supply,
                    weight_t,
                    ..
                },
        } => Given::LoadTo(target.raw(), *weight_t),
        OrderKind::LandUnit {
            target,
            order:
                LandUnitOrder::SetPosture {
                    posture: Posture::Attack,
                    when_spent: None,
                },
        } => Given::Attack(target.raw()),
        OrderKind::TaskForce {
            target,
            order: TaskForceOrder::LoadUnit { unit, cargo: None },
        } => Given::LoadUnit(target.raw(), unit.raw()),
        OrderKind::TaskForce {
            target,
            order:
                TaskForceOrder::SetRoute {
                    waypoints,
                    act_at,
                    stand_at,
                },
        } => {
            assert!(stand_at.is_empty(), "no stand mark");
            Given::Route(
                target.raw(),
                waypoints.clone(),
                act_at.iter().copied().collect(),
            )
        }
        OrderKind::TaskForce {
            target,
            order: TaskForceOrder::Unload,
        } => Given::Unload(target.raw()),
        other => panic!("the doctrine gave an order of another kind: {other:?}"),
    }
}

/// Whether the order is for task force 1 or one of units 1 and 2: intent
/// 1's.
fn of_intent_1(g: &Given) -> bool {
    match g {
        Given::Landing(f, ..)
        | Given::LoadTo(f, _)
        | Given::LoadUnit(f, _)
        | Given::Route(f, ..)
        | Given::Unload(f) => *f == 1,
        Given::Attack(u) => *u == 1 || *u == 2,
    }
}

fn route() -> Given {
    Given::Route(1, vec![B, A], vec![B])
}

/// A staff command, plainly, by kind.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Cmd {
    Approve(u32),
    Form(u32, IntentKind),
    Limit(u32, Vec<u32>),
    Complete(u32),
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

/// Day 1 of every case: plan 1 approved to preparing, intent 2 formed with
/// units 1 and 2, and the landing ordered.
fn day_1_commands() -> Vec<Cmd> {
    vec![
        Cmd::Approve(1),
        Cmd::Form(1, landing_kind()),
        Cmd::Limit(2, vec![1, 2]),
    ]
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

    /// The orders of `day` for intent 1's force and units.
    fn orders_1(&self, day: u32) -> Vec<Given> {
        self.orders(day).into_iter().filter(of_intent_1).collect()
    }

    fn commands(&self, day: u32) -> Vec<Cmd> {
        self.handed[&day]
            .commands
            .iter()
            .map(|c| cmd(&c.kind))
            .collect()
    }

    /// The "ready a landing" the staff formed under intent 1: intent 2 in
    /// the main case, the next free number where the commander stated more.
    fn landing_1(&self) -> Option<u32> {
        self.events.iter().find_map(|e| match &e.kind {
            EventKind::IntentFormed {
                side: Side::Allied,
                intent,
                parent,
            } if parent.raw() == 1 => Some(intent.raw()),
            _ => None,
        })
    }

    /// The staff's commands of `day` for plan 1 and the intents under it,
    /// the landing's number written as 2.
    fn commands_1(&self, day: u32) -> Vec<Cmd> {
        let landing = self.landing_1();
        self.commands(day)
            .into_iter()
            .filter_map(|c| match c {
                Cmd::Approve(p) | Cmd::Complete(p) if p == 1 => Some(c),
                Cmd::Form(1, _) => Some(c),
                Cmd::Limit(i, units) if Some(i) == landing => Some(Cmd::Limit(2, units)),
                _ => None,
            })
            .collect()
    }

    fn all_commands(&self) -> Vec<(u32, Cmd)> {
        (1..=self.days())
            .flat_map(|d| self.commands(d).into_iter().map(move |c| (d, c)))
            .collect()
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

    /// The notices of `day` for plan 1.
    fn notices_1(&self, day: u32) -> Vec<NoticeKind> {
        self.handed[&day]
            .notices
            .iter()
            .filter(|nt| nt.plan.raw() == 1)
            .map(|nt| nt.kind)
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

    fn unit_state(&self, day: u32, u: u32) -> &sim_core::state::LandUnit {
        &self.truth[day as usize].land_units[&unit(u)]
    }

    fn supply(&self, day: u32, b: u32) -> Fixed {
        self.truth[day as usize]
            .bases
            .get(&BaseId::new(b))
            .expect("the base exists")
            .supply_t
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

    fn formed_day(&self, b: u32) -> Option<u32> {
        self.events.iter().find_map(|e| match &e.kind {
            EventKind::BaseFormed { base, .. } if base.raw() == b => Some(e.day),
            _ => None,
        })
    }

    fn arrivals(&self, force: u32) -> Vec<(Hex, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceArrived {
                    task_force,
                    hex,
                    elapsed_hours,
                } if task_force.raw() == force => Some((*hex, *elapsed_hours)),
                EventKind::TaskForceHalted {
                    task_force,
                    hex,
                    elapsed_hours,
                    ..
                } if task_force.raw() == force => Some((*hex, *elapsed_hours)),
                _ => None,
            })
            .collect()
    }

    fn unloading_complete(&self, force: u32) -> Vec<(u32, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::UnloadingComplete {
                    task_force,
                    base,
                    elapsed_hours,
                } if task_force.raw() == force => Some((base.raw(), *elapsed_hours)),
                _ => None,
            })
            .collect()
    }

    fn landed(&self, units: &[u32]) -> Vec<(u32, Pulse, u32, u32)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::MenLanded { unit, men, .. } if units.contains(&unit.raw()) => {
                    Some((e.day, e.pulse?, unit.raw(), *men))
                }
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

    /// The commands refused, each with its day and the refusal.
    fn command_refusals(&self) -> Vec<(u32, CommandKind, CommandRefusal)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::CommandRefused {
                    command, reason, ..
                } => Some((e.day, command.clone(), *reason)),
                _ => None,
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

const NOT_BEFORE_6: &str = "# The limit that tells the plan to wait (decision 0159, 9).\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"not_before\"\nintent = 1\nfrom_day = 6\n";

fn without_not_before(t: &str) -> String {
    changed(t, NOT_BEFORE_6, "")
}

fn main_game() -> &'static Played {
    static GAME: std::sync::OnceLock<Played> = std::sync::OnceLock::new();
    GAME.get_or_init(|| play(&text()))
}

fn main_orders() -> Vec<(u32, Given)> {
    vec![
        (1, Given::Landing(1, B, vec![1, 2])),
        (6, Given::LoadTo(1, n(90))),
        (7, Given::Attack(1)),
        (7, Given::LoadUnit(1, 1)),
        (8, Given::Attack(2)),
        (8, Given::LoadUnit(1, 2)),
        (9, route()),
        (9, Given::Unload(1)),
    ]
}

/// The morning of `day` of intent 1: no order, no command and no notice,
/// and its one reason says the plan is held and names `parts`.
fn assert_held(g: &Played, day: u32, parts: &[&str]) {
    assert!(g.orders_1(day).is_empty(), "day {day}: {:?}", g.orders(day));
    assert!(
        g.commands_1(day).is_empty(),
        "day {day}: {:?}",
        g.commands(day)
    );
    let reasons = g.no_orders(day, 1);
    assert_eq!(reasons.len(), 1, "day {day}: {reasons:?}");
    assert_says(&reasons[0], parts);
    assert_says(&reasons[0], &["does not release plan:1"]);
}

/// The morning plan 1 is released: the approval and the load of 90 t.
fn assert_released_on(g: &Played, day: u32) {
    assert_eq!(g.commands_1(day), vec![Cmd::Approve(1)], "day {day}");
    assert_eq!(g.orders_1(day), vec![Given::LoadTo(1, n(90))], "day {day}");
    assert_eq!(g.plan_day(1, PlanState::Released), Some(day));
}

/// Day 1 as the main case.
fn assert_day_1(g: &Played) {
    assert_eq!(g.commands_1(1), day_1_commands());
    assert_eq!(g.orders_1(1), vec![Given::Landing(1, B, vec![1, 2])]);
    assert_eq!(g.plan_day(1, PlanState::Preparing), Some(1));
}

// ---------------------------------------------------------------- The hand calculation

#[test]
fn eight_orders_in_all_one_on_day_1_and_none_on_days_2_to_5() {
    let g = main_game();
    assert_eq!(g.days(), 13);
    assert_eq!(g.all_orders(), main_orders());
    assert!(g.refused().is_empty(), "{:?}", g.refused());
}

#[test]
fn the_commands_by_day() {
    let g = main_game();
    assert_eq!(
        g.all_commands(),
        vec![
            (1, Cmd::Approve(1)),
            (1, Cmd::Form(1, landing_kind())),
            (1, Cmd::Limit(2, vec![1, 2])),
            (6, Cmd::Approve(1)),
            (10, Cmd::Complete(1)),
        ]
    );
    assert_eq!(g.plan_day(1, PlanState::Preparing), Some(1));
    assert_eq!(g.plan_day(1, PlanState::Released), Some(6));
    assert_eq!(g.plan_day(1, PlanState::Complete), Some(10));
}

#[test]
fn the_notices() {
    let g = main_game();
    assert_eq!(
        g.all_notices(),
        vec![(2, NoticeKind::Ready, 1), (10, NoticeKind::Met, 1)]
    );
}

#[test]
fn held_on_days_2_to_5_and_no_leave_asked() {
    let g = main_game();
    for day in 2..=5 {
        assert_held(g, day, &["not to go before day 6"]);
    }
    for day in 3..=5 {
        assert!(g.notices_1(day).is_empty(), "day {day}");
    }
    assert!(!g
        .all_notices()
        .iter()
        .any(|(_, k, _)| *k == NoticeKind::WaitsForLeave(Leave::Launch)));
}

#[test]
fn the_landing_made_ready_on_day_1() {
    let g = main_game();
    let force = &g.truth[1].task_forces[&TaskForceId::new(1)];
    assert_eq!(
        force.preparation.as_ref().map(|p| (p.hex, p.loading)),
        Some((B, n(100)))
    );
    for u in [1, 2] {
        assert_eq!(g.unit_state(1, u).rehearsal, n(100), "unit {u}");
    }
}

#[test]
fn the_passage_and_the_landing() {
    let g = main_game();
    assert_eq!(g.arrivals(1), vec![(B, n(208)), (A, n(244))]);
    assert_eq!(
        g.landed(&[1, 2]),
        vec![
            (9, Pulse::Morning, 1, 160),
            (9, Pulse::Morning, 2, 160),
            (9, Pulse::Afternoon, 1, 440),
            (9, Pulse::Afternoon, 2, 440),
        ]
    );
    assert_eq!(g.formed_day(3), Some(9));
    assert_eq!(g.changed_side_day(2), Some(9));
    assert_eq!(g.unloading_complete(1), vec![(2, n(220))]);
}

#[test]
fn supply_fuel_and_men() {
    let g = main_game();
    let base1 = [
        994, 988, 982, 976, 970, 874, 868, 865, 865, 865, 865, 865, 865,
    ];
    for day in 1..=13u32 {
        assert_eq!(
            g.supply(day, 1),
            n(base1[(day - 1) as usize]),
            "base 1 after day {day}"
        );
    }
    assert_eq!(g.supply(9, 3), n(10));
    for day in 10..=13 {
        assert_eq!(g.supply(day, 3), n(4), "base 3 after day {day}");
    }
    let base2 = [80, 74, 68, 62];
    for day in 10..=13u32 {
        assert_eq!(
            g.supply(day, 2),
            n(base2[(day - 10) as usize]),
            "base 2 after day {day}"
        );
    }
    assert_eq!(g.truth[8].bases[&BaseId::new(2)].side, Side::Japanese);
    assert_eq!(g.truth[9].bases[&BaseId::new(2)].side, Side::Allied);
    for s in [1, 2] {
        assert_eq!(g.ship(13, s).fuel_t, n(968), "ship {s}");
    }
    for u in [1, 2] {
        let state = g.unit_state(13, u);
        assert_eq!(state.men_ashore, 600, "unit {u}");
        assert_eq!(state.hex, Some(B), "unit {u}");
    }
}

#[test]
fn the_other_side_is_given_nothing() {
    let g = main_game();
    for (day, handed) in &g.japanese {
        assert!(handed.orders.is_empty(), "day {day}");
    }
}

// ---------------------------------------------------------------- The properties

#[test]
fn no_limit() {
    let g = play(&without_not_before(&text()));
    assert_day_1(&g);
    assert_eq!(g.notices_1(2), vec![NoticeKind::Ready]);
    assert_released_on(&g, 2);
}

#[test]
fn the_limit_lifted() {
    let lift = "[[command]]\nday = 3\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"not_before\"\nintent = 1\n";
    let g = play(&with_commands(&text(), lift));
    assert_day_1(&g);
    assert_held(&g, 2, &["not to go before day 6"]);
    assert_released_on(&g, 3);
}

#[test]
fn leave_withheld() {
    let t = changed(
        &text(),
        "leave = \"launch\"\ngiven = true\n",
        "leave = \"launch\"\ngiven = false\n",
    );
    let g = play(&t);
    assert_day_1(&g);
    for day in 2..=5 {
        assert_held(&g, day, &["not to go before day 6"]);
        assert!(
            !g.notices_1(day)
                .contains(&NoticeKind::WaitsForLeave(Leave::Launch)),
            "day {day}"
        );
    }
    assert_eq!(
        g.notices_1(6),
        vec![NoticeKind::WaitsForLeave(Leave::Launch)]
    );
    assert!(g.commands_1(6).is_empty());
    assert!(g.orders_1(6).is_empty());
    assert_eq!(g.plan_day(1, PlanState::Released), None);
}

const FORCE_2: &str = "\n# Task force 2: one placeholder cruiser, stopped in port at base 1, usable\n# from day 5.\n[[task_force]]\nid = 2\nname = \"Placeholder cruiser force 2\"\nside = \"allied\"\nat_base = 1\navailable_from = 5\nmission = \"surface_combat\"\n\n[[ship]]\nid = 3\nname = \"Placeholder cruiser 3\"\nclass = 1\nside = \"allied\"\nfuel_t = 2000\ntask_force = 2\n";

const WAIT_FOR_2: &str = "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"wait_for\"\nintent = 1\ntask_force = 2\n";

#[test]
fn wait_for_a_force() {
    let t = with_commands(
        &changed(
            &without_not_before(&text()),
            "# The land units",
            &format!("{FORCE_2}\n# The land units"),
        ),
        WAIT_FOR_2,
    );
    let g = play(&t);
    assert_day_1(&g);
    for day in 2..=4 {
        assert_held(&g, day, &["task_force:2", "day 5"]);
    }
    assert_released_on(&g, 5);
    for day in 1..=g.days() {
        assert!(
            !g.handed[&day].orders.iter().any(|o| matches!(
                &o.kind,
                OrderKind::TaskForce { target, .. } if *target == TaskForceId::new(2)
            )),
            "day {day}"
        );
    }
}

const PLACE_C: &str = "\n# Japanese base 4 at C: nothing at it.\n[[base]]\nid = 4\nname = \"Placeholder airfield C\"\nside = \"japanese\"\nq = 0\nr = 6\nsupply_t = 0\nfuel_t = 0\n\n# Task force 3: one placeholder troop transport, stopped in port at base 1.\n[[task_force]]\nid = 3\nname = \"Placeholder transport force 3\"\nside = \"allied\"\nat_base = 1\nmission = \"transport\"\n\n[[ship]]\nid = 4\nname = \"Placeholder troop transport 4\"\nclass = 10\nside = \"allied\"\nfuel_t = 1000\ntask_force = 3\n\n[[land_unit]]\nid = 3\nname = \"Placeholder Third\"\nside = \"allied\"\nmen = 600\nq = 0\nr = 0\n";

const INTENT_3: &str = "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_intent\"\nintent = 3\nintent_kind = \"take_place\"\nhex = [0, 6]\n\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"forces\"\nintent = 3\ntask_forces = [3]\nland_units = [3]\nsource = 1\n\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"after\"\nintent = 1\nother_intent = 3\n";

/// "After another place": base 4 at C, task force 3 and unit 3, and intent
/// 3 stated after intent 1, with the limit "after intent 3" on intent 1 in
/// place of "not before".
fn after_text() -> String {
    let t = changed(
        &without_not_before(&text()),
        "# Task force 1:",
        &format!("{PLACE_C}\n# Task force 1:"),
    );
    // Intent 3 and its limits come after intent 1's forces limit, before
    // the leaves.
    changed(
        &t,
        "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_leave\"\nleave = \"prepare\"\n",
        &format!("{INTENT_3}\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_leave\"\nleave = \"prepare\"\n"),
    )
}

/// The orders to task force 3 or unit 3, by day.
fn orders_3(g: &Played) -> Vec<(u32, Given)> {
    g.all_orders()
        .into_iter()
        .filter(|(_, o)| match o {
            Given::Landing(f, ..)
            | Given::LoadTo(f, _)
            | Given::LoadUnit(f, _)
            | Given::Route(f, ..)
            | Given::Unload(f) => *f == 3,
            Given::Attack(u) => *u == 3,
        })
        .collect()
}

#[test]
fn after_another_place() {
    let g = play(&after_text());
    assert!(g.refused().is_empty(), "{:?}", g.refused());
    // Intent 1.
    assert_day_1(&g);
    for day in 2..=5 {
        assert_held(&g, day, &["intent:3"]);
    }
    assert_released_on(&g, 6);
    // Intent 3 goes as a take does.
    assert_eq!(
        orders_3(&g)
            .into_iter()
            .filter(|(d, _)| *d <= 5)
            .collect::<Vec<_>>(),
        vec![
            (1, Given::Landing(3, C, vec![3])),
            (2, Given::LoadTo(3, n(45))),
            (3, Given::Attack(3)),
            (3, Given::LoadUnit(3, 3)),
            (4, Given::Route(3, vec![C, A], vec![C])),
            (4, Given::Unload(3)),
        ]
    );
    assert_eq!(g.plan_day(2, PlanState::Released), Some(2));
    assert_eq!(g.arrivals(3).first(), Some(&(C, n(96))));
    assert_eq!(g.landed(&[3]), vec![(5, Pulse::Night, 3, 600)]);
    assert_eq!(
        g.unloading_complete(3).first(),
        Some(&(5, "105.75".parse::<Fixed>().expect("a decimal")))
    );
    assert_eq!(g.changed_side_day(4), Some(5));
}

#[test]
fn a_wait_that_can_never_end() {
    let withdraw =
        "[[command]]\nday = 2\nside = \"allied\"\nkind = \"withdraw_intent\"\nintent = 3\n";
    let g = play(&with_commands(&after_text(), withdraw));
    assert_day_1(&g);
    let cannot: Vec<(u32, NoticeKind, u32)> = g
        .all_notices()
        .into_iter()
        .filter(|(_, k, p)| *k == NoticeKind::CannotMeet && *p == 1)
        .collect();
    assert_eq!(cannot, vec![(2, NoticeKind::CannotMeet, 1)]);
    let day2 = g.no_orders(2, 1);
    assert_eq!(day2.len(), 1, "{day2:?}");
    assert_says(
        &day2[0],
        &[
            "after intent:3",
            "intent:3 is withdrawn",
            "hands in the notice",
        ],
    );
    for day in 3..=4 {
        assert!(
            !g.notices_1(day).contains(&NoticeKind::CannotMeet),
            "day {day}"
        );
        assert!(g.notices_1(day).is_empty(), "day {day}");
    }
    for day in 2..=6 {
        assert!(g.commands_1(day).is_empty(), "day {day}");
        assert!(g.orders_1(day).is_empty(), "day {day}");
    }
    assert_eq!(g.plan_day(1, PlanState::Preparing), Some(1));
    assert_eq!(g.plan_day(1, PlanState::Released), None);
}

const UNIT_3: &str = "\n# Unit 3, usable from day 5, not in intent 1's forces limit.\n[[land_unit]]\nid = 3\nname = \"Placeholder Third\"\nside = \"allied\"\nmen = 600\nq = 0\nr = 0\navailable_from = 5\n";

const WAIT_FOR_UNIT_3: &str = "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"wait_for\"\nintent = 1\nland_unit = 3\n";

#[test]
fn wait_for_a_unit() {
    let t = with_commands(
        &changed(
            &without_not_before(&text()),
            "# The Allied commander's commands",
            &format!("{UNIT_3}\n# The Allied commander's commands"),
        ),
        WAIT_FOR_UNIT_3,
    );
    let g = play(&t);
    assert!(g.refused().is_empty(), "{:?}", g.refused());
    assert_day_1(&g);
    for day in 2..=4 {
        assert_held(&g, day, &["land_unit:3", "day 5"]);
    }
    assert_released_on(&g, 5);
    for day in 1..=g.days() {
        assert!(
            !g.handed[&day].orders.iter().any(|o| matches!(
                &o.kind,
                OrderKind::LandUnit { target, .. } if *target == unit(3)
            ) || matches!(
                &o.kind,
                OrderKind::TaskForce {
                    order: TaskForceOrder::LoadUnit { unit: u, .. },
                    ..
                } if *u == unit(3)
            )),
            "day {day}"
        );
    }
}

#[test]
fn no_ring_of_waits() {
    // Intent 1 waits for intent 3; an "after" on intent 3 naming intent 1
    // would have each wait for the other.
    let ring = "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"after\"\nintent = 3\nother_intent = 1\n";
    let own = "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"after\"\nintent = 3\nother_intent = 3\n";
    let t = changed(&after_text(), "days = 13\n", "days = 1\n");
    let g = play(&with_commands(&with_commands(&t, ring), own));
    let refusals: Vec<CommandRefusal> = g
        .command_refusals()
        .into_iter()
        .map(|(_, _, reason)| reason)
        .collect();
    assert_eq!(
        refusals,
        vec![
            CommandRefusal::NotAnEndToWaitFor(intent(1)),
            CommandRefusal::NotAnEndToWaitFor(intent(3)),
        ]
    );
}

#[test]
fn refused_at_intake() {
    let japanese = "\n# A Japanese task force, far from everything.\n[[task_force]]\nid = 9\nname = \"Placeholder Japanese force 9\"\nside = \"japanese\"\nq = 5\nr = 7\nmission = \"surface_combat\"\n\n[[ship]]\nid = 9\nname = \"Placeholder destroyer 9\"\nclass = 2\nside = \"japanese\"\nfuel_t = 500\ntask_force = 9\n";
    let t = changed(
        &changed(&text(), "days = 13\n", "days = 2\n"),
        "# The land units",
        &format!("{japanese}\n# The land units"),
    );
    let set = |day: u32, body: &str| {
        format!("[[command]]\nday = {day}\nside = \"allied\"\nkind = \"set_limit\"\n{body}\n")
    };
    let extra = [
        "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_intent\"\nintent = 5\nintent_kind = \"hold\"\nbase = 1\n".to_string(),
        set(1, "limit = \"not_before\"\nintent = 5\nfrom_day = 3"),
        set(1, "limit = \"after\"\nintent = 1\nother_intent = 1"),
        set(1, "limit = \"after\"\nintent = 1\nother_intent = 5"),
        set(1, "limit = \"wait_for\"\nintent = 1\ntask_force = 9"),
        set(1, "limit = \"forces\"\nintent = 5\ntask_forces = [9]"),
        set(1, "limit = \"wait_for\"\nintent = 1\ntask_force = 99"),
    ]
    .join("\n");
    let g = play(&with_commands(&t, &extra));
    let refusals: Vec<(u32, CommandRefusal)> = g
        .command_refusals()
        .into_iter()
        .map(|(day, _, reason)| (day, reason))
        .collect();
    assert_eq!(
        refusals,
        vec![
            (1, CommandRefusal::NoWaitLimitOn(intent(5))),
            (1, CommandRefusal::NotAnEndToWaitFor(intent(1))),
            (1, CommandRefusal::NotAnEndToWaitFor(intent(5))),
            (1, CommandRefusal::NotYourForce(TaskForceId::new(9))),
            (1, CommandRefusal::NotYourForce(TaskForceId::new(9))),
            (1, CommandRefusal::NotYourForce(TaskForceId::new(99))),
        ]
    );
    // The same words for a force of the enemy's as for a forces limit
    // naming it, and as for one that does not exist but its number.
    let words: Vec<String> = refusals[3..6].iter().map(|(_, r)| r.to_string()).collect();
    assert_eq!(words[0], words[1]);
    assert_eq!(
        words[0].replace("task_force:9", "X"),
        words[2].replace("task_force:99", "X")
    );

    // An "after" naming an intent of the staff's: intent 2, the landing
    // formed on day 1.
    let staffs_own = format!(
        "{}\n{}",
        changed(&text(), "days = 13\n", "days = 2\n"),
        set(2, "limit = \"after\"\nintent = 1\nother_intent = 2")
    );
    let g = play(&staffs_own);
    assert_eq!(g.landing_1(), Some(2));
    let refusals: Vec<(u32, CommandRefusal)> = g
        .command_refusals()
        .into_iter()
        .map(|(day, _, reason)| (day, reason))
        .collect();
    assert_eq!(
        refusals,
        vec![(2, CommandRefusal::NotAnEndToWaitFor(intent(2)))]
    );
}

// ---------------------------------------------------------------- Determinism

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
fn saved_on_day_3_and_resumed() {
    let through = main_game();
    let at_3 = rebuilt(&through.handed, 3);
    assert_eq!(at_3.world.day_hashes(), &through.hashes[..3]);
    let first: BTreeMap<u32, StaffDecisions> =
        (1..=3).map(|d| (d, through.handed[&d].clone())).collect();
    let resumed = play_from(at_3, first);
    assert_eq!(resumed.hashes, through.hashes);
    assert_eq!(resumed.handed, through.handed);
}

#[test]
fn the_same_start_gives_the_same_orders() {
    let a = play(&text());
    assert_eq!(&a.handed, &main_game().handed);
    assert_eq!(a.hashes, main_game().hashes);
}
