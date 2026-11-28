//! Scenario 136: supply at press. Scenario 76's island, asked for at
//! press. The staff lands the men with the supply that lasts until the
//! first run can follow, and the first run follows: it is ordered so that
//! it lands on the day the stock runs out, and nobody goes unfed.
//!
//! Proves decision 0230 against a hand calculation. It stands on scenario
//! 76's, whose tree, landing and supply sums it repeats with one command
//! more. The data is placeholder data: see
//! `data/scenarios/supply-at-press/136_supply_at_press.toml`. It runs on
//! the frozen step 1 figures with scenario 76's ground figures file. No
//! enemy man is ashore anywhere, so no die is rolled.
//!
//! **Built.** Each test below is filled to the statement above it. The
//! scenario file played the game before the rule as the section "Before
//! the rule" gives.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! The run of day 7 and every stock below were worked by hand and then
//! checked on the engine as it stood on Nov 24, 2026, before the rule, by
//! a copy of the scenario in which the commander himself states "keep
//! base 2 supplied" with a floor of 1 day on day 5, so that the hold forms
//! none: the supply doctrine served it and gave these figures. The numbers
//! of the intents and plans in the game with the rule, where the staff
//! forms the supply, could be checked on nothing.
//!
//! ## Inputs
//!
//! Scenario 76's, whole: A at (0,0), port size 5; B at (4,0), port size
//! 0, 160 nm from A; C at (0,6), port size 0. Base 1 at A, Allied, 1,000
//! t. Base 2 at B, Japanese, on the Allied map, empty. Base 3 at C,
//! Allied, 100 t. Task force 1, ships 1 and 2, placeholder troop
//! transports, at base 1. Unit 1, 600 men, at A; unit 3, 300 men, at C.
//! Island 1: base 2, then base 3. The game is 14 days.
//!
//! What the sums rest on, each the engine's as it stands: a man eats
//! 0.005 t a day; of the unfed one in twenty is lost a night, cut to a
//! whole man; a load takes one day at A and the passage to B one day, so
//! a run ordered on a morning is due the next day and the reading the
//! doctrine checks is the day after that; a night's charge is paid before
//! anything lands that day; a projected reading is never below 0 t; in a
//! hex the lower-numbered base pays first, base 2 before base 4; and with
//! no interval set the staff sizes a run for 15 days of use.
//!
//! The Allied side's commands of day 1: intent 1, control island 1; the
//! risk of intent 1 set to press; both leaves given.
//!
//! ## The floor at press
//!
//! The margin of the check that a place can be held is 0 days at press,
//! as before. The floor of a "keep supplied" the staff forms is that
//! margin and never less than 1 day: 10 days at preserve the force, 5 at
//! balanced, 1 at press.
//!
//! ## The tree
//!
//! As scenario 76's, with the two supplies formed at a floor of 1 day:
//!
//! | Intent | Kind | Under | Formed |
//! | ------ | ---- | ----- | ------ |
//! | 1 | Control island 1, at press | none | the commander, day 1 |
//! | 2 | Take the place at base 2 | 1 | day 1 |
//! | 3 | Hold base 2 | 1 | day 1 |
//! | 4 | Hold base 3 | 1 | day 1 |
//! | 5 | Deny the waters within 3 hexes of (4,0) | 1 | day 1 |
//! | 6 | Neutralise base 2 | 1 | day 1; withdrawn on day 5 |
//! | 7 | Keep base 3 supplied, floor 1 day | 4 | day 1 |
//! | 8 | Ready a landing by task force 1 at B | 2 | day 1 |
//! | 9 | Keep base 2 supplied, floor 1 day | 3 | day 5 |
//!
//! The reason of each of the two commands that form a supply says "with a
//! floor of 1 day (the least floor, at press)", where before the rule it
//! said "with a floor of 0 days (the margin at press)".
//!
//! ## The landing
//!
//! The check that the place can be held: the need is 600 x 0.005 = 3 t a
//! day; supply follows in 4 days; the margin at press is 0; the supply
//! that lands with the men is 3 x 4 = 12 t. The landing is scenario 76's
//! but for that weight:
//!
//! | Day | The staff's orders | What happens |
//! | --- | --- | --- |
//! | 1 | `prepare_landing` at B, roster 1 | |
//! | 2 | A load of supply to a weight of 12 t | Ship 1 loads 12 t |
//! | 3 | Unit 1 set to attack. Load unit 1 | |
//! | 4 | A route to B and back to A, acting at B, and an unload order | Ship 1 lands 160 men in the morning, 440 in the afternoon and then 10 t. Base 4, "Landing at (4,0)", is formed. Base 2 changes side at the end of the day |
//! | 5 | The take's plan is marked complete. Unit 1 set to defend | Ship 1 lands 2 t at base 2 in the night. The force steams for A |
//! | 6 | | The force arrives at A in the night |
//!
//! ## The run that follows
//!
//! Intent 9 is formed on the morning of day 5 and first served on day 6.
//! The use at B is 3 t a day and the floor's stock 1 x 3 = 3 t. The stock
//! in the hex is that of bases 2 and 4 together.
//!
//! - **Day 6.** The stock is 9 t (2 t and 7 t). A run ordered today would
//!   be due on day 7, and the reading checked is day 8's: 9 - 3 - 3 = 3
//!   t, not below 3 t. No run is needed. (Task force 1 is at sea this
//!   morning; that binds nothing, since none is needed.)
//! - **Day 7.** The stock is 6 t. A run ordered today is due on day 8,
//!   and the reading checked is day 9's: 6 - 3 - 3 = 0 t, below 3 t. A
//!   run is needed. Its load is the floor's stock on the day it arrives,
//!   3 t, less the reading of that day, 3 t, which is 0, plus 15 days of
//!   use: 45 t. The staff adds run 1 of intent 9's plan, 45 t, loading
//!   day 7 and due day 8, commits both ships from day 7 to day 10, and
//!   orders a load of supply to a weight of 45 t. Ship 1 loads 45 t that
//!   night.
//! - **Day 8.** A route to B and back to A, acting at B, and an unload
//!   order. Ship 1 lands 40 t at base 2 in the morning and 5 t in the
//!   afternoon.
//! - **No second run.** One is next needed on the first morning the
//!   stock is below 9 t. It is 30 t on the morning of day 14.
//!
//! Ten orders in all: seven of the landing and three of the run. No staff
//! order or command is refused.
//!
//! ## Supply, at the end of each day
//!
//! | Day | Base 1 | Base 2 | Base 4 | Base 3 |
//! | --- | ------ | ------ | ------ | ------ |
//! | 1 | 997 | 0 | | 98.5 |
//! | 2 | 982 | 0 | | 97 |
//! | 3 | 979 | 0 | | 95.5 |
//! | 4 | 979 | 0 | 10 | 94 |
//! | 5 | 979 | 2 | 7 | 92.5 |
//! | 6 | 979 | 0 | 6 | 91 |
//! | 7 | 934 | 0 | 3 | 89.5 |
//! | 8 | 934 | 45 | 0 | 88 |
//! | 9 | 934 | 42 | 0 | 86.5 |
//! | 14 | 934 | 27 | 0 | 79 |
//!
//! Base 1 feeds unit 1 on the nights of days 1 to 3 and loads out 12 t
//! and 45 t: 1,000 - 9 - 57 = 934 t. At B the night's charge is paid by
//! base 2 and then base 4: on the night of day 5 base 2 holds nothing and
//! base 4 pays 3 t; on the night of day 6 base 2 pays its 2 t and base 4
//! 1 t; on the nights of days 7 and 8 base 4 pays 3 t and is empty. The
//! run lands on day 8, after that night's charge. Base 2 pays 3 t on the
//! nights of days 9 to 14: 45 - 18 = 27 t.
//!
//! ## The men
//!
//! Every man is fed on every night. Unit 1 has 600 men at the end and
//! unit 3 has 300. No `MenLostUnfed` is recorded in the game.
//!
//! ## Before the rule
//!
//! For the record, and not asserted: the game the scenario file plays on
//! the engine before decision 0230. The two supplies are formed with a
//! floor of 0 days; no run is ever needed, since no reading is below 0 t;
//! bases 2 and 4 are empty from the end of day 8; on the night of day 9
//! all 600 men are unfed, and unit 1 loses 30, 28, 27, 25, 24 and 23 men
//! on the nights of days 9 to 14: 157 lost, 443 left. Base 1 ends at 979
//! t. Seven orders.
//!
//! # Properties
//!
//! - **Never a night short, with a stock that is no whole number of
//!   days.** With base 2 holding 3 t at the start: half of it, 1.5 t, is
//!   taken with the base at the end of day 4. On the night of day 5 base
//!   2 pays its 1.5 t and base 4 1.5 t (8.5 t left), and base 2 then
//!   takes the 2 t from the ship; on the night of day 6 base 2 pays 2 t
//!   and base 4 1 t (7.5 t left). On the morning of day 6 the stock at B
//!   is 10.5 t and the reading checked 4.5 t, not below 3 t. On the
//!   morning of day 7 it is 7.5 t, two and a half days, and the reading
//!   checked 1.5 t, below 3 t: the run is ordered, 0 + 45 = 45 t, since
//!   the reading of the day it arrives, 4.5 t, is above the floor's
//!   stock. Base 4 pays 3 t on the nights of days 7 and 8, 4.5 t and
//!   then 1.5 t left, and the run lands on day 8. No man is unfed on any
//!   night. Base 4 holds 1.5 t and base 2 27 t at the end. (Restated on
//!   review, Nov 24, 2026, before anything was built: the first writing
//!   changed the unit's men, which leaves the stock at three days of use
//!   at every size and so never ran the case. Checked on the engine as
//!   the main case was.)
//! - **At balanced and at preserve nothing moves.** With the risk command
//!   left out, the game is scenario 76's main case: its ten orders on
//!   their days, the supplies formed with a floor of 5 days, and 919 t,
//!   35 t and 79 t at bases 1, 2 and 3 at the end. With the risk set to
//!   preserve, the supplies are formed with a floor of 10 days.
//! - **The commander's own floor is his.** With the commander's own
//!   intent "keep base 3 supplied, floor 3 days", with a forces limit of
//!   task force 1 and base 1, stated on day 1 beside intent 1: the staff
//!   forms no supply of base 3, and the commander's keeps its floor of 3
//!   days.
//! - **The same twice.** The main case played twice gives the same daily
//!   hashes.
#![allow(clippy::doc_lazy_continuation, clippy::doc_markdown)]
#![allow(dead_code)]

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::command::{
    CommandKind, CommandState, ForcesLimit, IntentKind, Limit, Maker, NoticeKind, PlanOutcome,
    PlanState, Reason, StaffDecisions, StaffOrder,
};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, LandUnitId, ShipId, TaskForceId};
use sim_core::orders::{
    AirGroupOrder, AirMission, Cargo, LandUnitOrder, OrderKind, Posture, TaskForceOrder,
};
use sim_core::state::{Side, TruthState};
use sim_core::time::Pulse;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

use crate::common;
use crate::common::replace_once as changed;
use crate::common::values::{assert_says, command_table, fx, intent, n, text_of};

const COMMANDS_MARKER: &str = "# The Allied commander's commands";
const A: Hex = Hex { q: 0, r: 0 };
const B: Hex = Hex { q: 4, r: 0 };

fn path() -> PathBuf {
    common::data_dir().join("scenarios/supply-at-press/136_supply_at_press.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

/// The frozen step 1 figures. The scenario names its own ground figures,
/// `data/reference/ground_placeholder_scenario_76.toml`, frozen beside the
/// others.
fn load(text: &str) -> Scenario {
    common::from_text_with(text, &path(), &common::frozen()).expect("the scenario loads")
}

struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    truth: Vec<TruthState>,
    events: Vec<Event>,
    hashes: Vec<u64>,
    state: CommandState,
    /// The measure of intent 1 as the agent interface reads it, with its
    /// words, after each day: index 0 before day 1.
    measures: Vec<Option<sim_agent::IslandMeasure>>,
    words: Vec<String>,
}

fn play(text: &str) -> Played {
    play_from(load(text), BTreeMap::new())
}

fn words_of(scenario: &Scenario, staff: &dyn sim_core::command::Staff) -> String {
    let view = scenario.world.side_view(Side::Allied);
    let state = sim_agent::command_state(&view, staff, &[]);
    state["intents"]
        .as_array()
        .and_then(|all| all.iter().find(|i| i["number"] == 1))
        .and_then(|i| i["measure"]["text"].as_str())
        .unwrap_or("")
        .to_string()
}

fn play_from(mut scenario: Scenario, mut handed: BTreeMap<u32, StaffDecisions>) -> Played {
    let held = staffs();
    let mut truth = vec![scenario.world.umpire().clone()];
    let mut measures = vec![sim_agent::island_measure(
        &scenario.world.side_view(Side::Allied),
        intent(1),
    )];
    let mut words = vec![words_of(&scenario, held[&Side::Allied].as_ref())];
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
        measures.push(sim_agent::island_measure(
            &scenario.world.side_view(Side::Allied),
            intent(1),
        ));
        words.push(words_of(&scenario, held[&Side::Allied].as_ref()));
    }
    Played {
        handed,
        truth,
        events: scenario.world.events().iter().cloned().collect(),
        hashes: scenario.world.day_hashes().to_vec(),
        state: scenario.world.command_state(Side::Allied).clone(),
        measures,
        words,
    }
}

/// An order of the staff, plainly, with the task force or unit it is for.
/// Any order the main case does not give is `Other`, with its debug text.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Given {
    Landing(u32, Hex, Vec<u32>),
    LoadTo(u32, Fixed),
    Attack(u32),
    Defend(u32),
    LoadUnit(u32, u32),
    Route(u32, Vec<Hex>, Vec<Hex>),
    Unload(u32),
    /// An air group set to a mission.
    Mission(u32, AirMission),
    Other(String),
}

fn given(o: &StaffOrder) -> Given {
    match &o.kind {
        OrderKind::TaskForce { target, order } => {
            let tf = target.raw();
            match order {
                TaskForceOrder::PrepareLanding { hex, roster } => {
                    Given::Landing(tf, *hex, roster.iter().map(|u| u.raw()).collect())
                }
                TaskForceOrder::LoadTo {
                    cargo: Cargo::Supply,
                    weight_t,
                    ..
                } => Given::LoadTo(tf, *weight_t),
                TaskForceOrder::LoadUnit { unit, cargo: None } => Given::LoadUnit(tf, unit.raw()),
                TaskForceOrder::SetRoute {
                    waypoints,
                    act_at,
                    stand_at,
                } if stand_at.is_empty() => {
                    Given::Route(tf, waypoints.clone(), act_at.iter().copied().collect())
                }
                TaskForceOrder::Unload => Given::Unload(tf),
                other => Given::Other(format!("task force {tf}: {other:?}")),
            }
        }
        OrderKind::LandUnit {
            target,
            order:
                LandUnitOrder::SetPosture {
                    posture,
                    when_spent: None,
                },
        } => match posture {
            Posture::Attack => Given::Attack(target.raw()),
            Posture::Defend => Given::Defend(target.raw()),
        },
        OrderKind::AirGroup {
            target,
            order: AirGroupOrder::SetAirMission(m),
        } => Given::Mission(target.raw(), *m),
        other => Given::Other(format!("{other:?}")),
    }
}

/// Whether two lists hold the same orders, whatever the order they stand in.
fn same(a: &[Given], b: &[Given]) -> bool {
    let key = |v: &[Given]| {
        let mut k: Vec<String> = v.iter().map(|g| format!("{g:?}")).collect();
        k.sort();
        k
    };
    key(a) == key(b)
}

fn route(tf: u32) -> Given {
    Given::Route(tf, vec![B, A], vec![B])
}

/// A staff command, plainly, by kind. Any other is `Other`, with its debug
/// text.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Cmd {
    Approve(u32),
    Form(u32, IntentKind),
    /// A forces limit: intent, task forces, land units, source.
    Limit(u32, Vec<u32>, Vec<u32>, Option<u32>),
    Complete(u32),
    Withdraw(u32),
    Other(String),
}

fn cmd(c: &CommandKind) -> Cmd {
    match c {
        CommandKind::Approve { plan } => Cmd::Approve(plan.raw()),
        CommandKind::FormIntent { parent, kind } => Cmd::Form(parent.raw(), *kind),
        CommandKind::SetLimit(Limit::Forces { intent, limit }) => {
            let ForcesLimit {
                ships,
                task_forces,
                air_groups,
                source,
                land_units,
                submarines: _,
                none_of: _,
                in_place: _,
            } = limit;
            assert!(ships.is_empty() && air_groups.is_empty(), "{limit:?}");
            Cmd::Limit(
                intent.raw(),
                task_forces.iter().map(|t| t.raw()).collect(),
                land_units.iter().map(|u| u.raw()).collect(),
                source.map(|b| b.raw()),
            )
        }
        CommandKind::MarkPlan {
            plan,
            outcome: PlanOutcome::Complete,
        } => Cmd::Complete(plan.raw()),
        CommandKind::WithdrawIntent { intent } => Cmd::Withdraw(intent.raw()),
        other => Cmd::Other(format!("{other:?}")),
    }
}

fn take_kind(by_day: Option<u32>) -> IntentKind {
    IntentKind::TakePlace { hex: B, by_day }
}

fn hold_kind(base: u32) -> IntentKind {
    IntentKind::Hold {
        base: BaseId::new(base),
    }
}

fn deny_kind() -> IntentKind {
    IntentKind::Deny {
        hex: B,
        distance: 3,
    }
}

fn neutralise_kind(base: u32) -> IntentKind {
    IntentKind::Neutralise {
        target: BaseId::new(base),
    }
}

fn keep_kind(base: u32, floor_days: u32) -> IntentKind {
    IntentKind::KeepSupplied {
        base: BaseId::new(base),
        floor_days,
        interval_days: None,
    }
}

fn landing_kind(force: u32, by_day: Option<u32>) -> IntentKind {
    IntentKind::ReadyLanding {
        force: TaskForceId::new(force),
        hex: B,
        by_day,
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

    /// The forces limit the staff set for `of` on `day`: task forces, land
    /// units and source.
    fn staff_limit(&self, day: u32, of: u32) -> Option<(Vec<u32>, Vec<u32>, Option<u32>)> {
        self.commands(day).into_iter().find_map(|c| match c {
            Cmd::Limit(i, tfs, units, source) if i == of => Some((tfs, units, source)),
            _ => None,
        })
    }

    /// Every intent the staff formed: (day, parent, kind), in order.
    fn forms(&self) -> Vec<(u32, u32, IntentKind)> {
        self.all_commands()
            .into_iter()
            .filter_map(|(d, c)| match c {
                Cmd::Form(p, k) => Some((d, p, k)),
                _ => None,
            })
            .collect()
    }

    fn formed(&self, kind: IntentKind) -> Vec<(u32, u32)> {
        self.forms()
            .into_iter()
            .filter(|(_, _, k)| *k == kind)
            .map(|(d, p, _)| (d, p))
            .collect()
    }

    fn notices(&self) -> Vec<(u32, NoticeKind, u32)> {
        (1..=self.days())
            .flat_map(|d| {
                self.handed[&d]
                    .notices
                    .iter()
                    .map(move |nt| (d, nt.kind, nt.plan.raw()))
            })
            .collect()
    }

    /// The reasons for giving no order of `day` that serve `of`.
    fn no_orders(&self, day: u32, of: u32) -> Vec<String> {
        self.handed[&day]
            .no_orders
            .iter()
            .filter(|r| r.serves.intent == Some(intent(of)))
            .map(text_of)
            .collect()
    }

    /// Every reason of `day` (commands, orders and decisions to give no
    /// order) that serves intent `of`, as text.
    fn reasons_serving(&self, day: u32, of: u32) -> String {
        let d = &self.handed[&day];
        let mut all: Vec<&Reason> = d.commands.iter().map(|c| &c.reason).collect();
        all.extend(d.orders.iter().map(|o| &o.reason));
        all.extend(d.no_orders.iter());
        all.into_iter()
            .filter(|r| r.serves.intent == Some(intent(of)))
            .map(text_of)
            .collect::<Vec<_>>()
            .join(" || ")
    }

    #[allow(dead_code)]
    /// Every reason of `day`, whatever it serves, as text.
    fn reasons(&self, day: u32) -> String {
        let d = &self.handed[&day];
        let mut all: Vec<&Reason> = d.commands.iter().map(|c| &c.reason).collect();
        all.extend(d.orders.iter().map(|o| &o.reason));
        all.extend(d.no_orders.iter());
        all.into_iter()
            .map(text_of)
            .collect::<Vec<_>>()
            .join(" || ")
    }

    fn unit_state(&self, day: u32, u: u32) -> &sim_core::state::LandUnit {
        &self.truth[day as usize].land_units[&LandUnitId::new(u)]
    }

    fn base(&self, day: u32, b: u32) -> Option<&sim_core::state::Base> {
        self.truth[day as usize].bases.get(&BaseId::new(b))
    }

    /// The supply of base `b` at the end of `day`.
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

    /// (day, intent, parent) of every intent the staff formed.
    fn formed_intents(&self) -> Vec<(u32, u32, u32)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::IntentFormed {
                    side: Side::Allied,
                    intent: i,
                    parent: p,
                } => Some((e.day, i.raw(), p.raw())),
                _ => None,
            })
            .collect()
    }

    /// (day, intent, parent) of every intent withdrawn with the one above.
    fn withdrawn_with(&self) -> Vec<(u32, u32, u32)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::IntentWithdrawn {
                    side: Side::Allied,
                    intent: i,
                    parent: p,
                } => Some((e.day, i.raw(), p.raw())),
                _ => None,
            })
            .collect()
    }

    fn intent_state(&self, raw: u32) -> &sim_core::command::Intent {
        &self.state.intents[&intent(raw)]
    }

    /// The transports' passages: where task force 1 arrived or halted, and
    /// the hour.
    fn transport_arrivals(&self) -> Vec<(Hex, Fixed)> {
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

    fn transport_steaming(&self) -> Vec<(u32, Pulse, Hex)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceSteamed {
                    task_force, from, ..
                } if task_force.raw() == 1 => Some((e.day, e.pulse?, *from)),
                _ => None,
            })
            .collect()
    }

    /// (day, pulse, unit, ship, men) of every landing of men.
    fn landed(&self) -> Vec<(u32, Pulse, u32, u32, u32)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::MenLanded {
                    unit, ship, men, ..
                } => Some((e.day, e.pulse?, unit.raw(), ship.raw(), *men)),
                _ => None,
            })
            .collect()
    }

    /// (day, pulse, ship, tons) of every landing of supply.
    fn supply_landed(&self) -> Vec<(u32, Pulse, u32, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::CargoUnloaded {
                    ship,
                    cargo: Cargo::Supply,
                    unloaded_t,
                    ..
                } => Some((e.day, e.pulse?, ship.raw(), *unloaded_t)),
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

    /// The days on which group `raw` launched a base strike at base 2.
    fn base_strikes(&self, raw: u32) -> Vec<(u32, BaseId)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::BaseStrikeLaunched {
                    air_group, base, ..
                } if air_group.raw() == raw => Some((e.day, *base)),
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

    fn nothing_refused(&self) {
        assert!(self.refused().is_empty(), "{:?}", self.refused());
    }

    /// The days controlled after each day: index 0 before day 1.
    fn controlled(&self) -> Vec<u32> {
        self.measures
            .iter()
            .map(|m| m.as_ref().map_or(0, |m| m.days_controlled))
            .collect()
    }
}

fn with(extra: &str) -> String {
    format!("{}{}", text(), extra)
}

fn hold_force(day: u32, tf: u32) -> String {
    command_table(day, &format!("kind = \"hold\"\ntask_force = {tf}"))
}

fn forces_limit(body: &str) -> String {
    command_table(
        1,
        &format!("kind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\n{body}"),
    )
}

/// The text with `extra` tables added before the commands.
fn before_commands(extra: &str) -> String {
    changed(
        &text(),
        COMMANDS_MARKER,
        &format!("{extra}{COMMANDS_MARKER}"),
    )
}

fn main_game() -> &'static Played {
    static GAME: std::sync::OnceLock<Played> = std::sync::OnceLock::new();
    GAME.get_or_init(|| play(&text()))
}

// ---------------------------------------------------------------- The hand calculation

const PRESS_WORDS: &str = "with a floor of 1 day (the least floor, at press)";

const RISK_BLOCK: &str = "# The island is asked for at press.\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"press\"\n";

fn all_reasons(g: &Played) -> String {
    (1..=g.days())
        .map(|d| g.reasons(d))
        .collect::<Vec<_>>()
        .join(" || ")
}

fn want_orders(g: &Played, wanted: Vec<(u32, Vec<Given>)>) {
    for (day, want) in wanted {
        assert!(
            same(&g.orders(day), &want),
            "day {day}: {:?} against {want:?}",
            g.orders(day)
        );
    }
}

/// The tree: the intents of the table, and the floors of intents 7 and 9,
/// each 1 day.
#[test]
fn the_tree_and_the_floor_at_press() {
    let g = main_game();
    assert_eq!(g.days(), 14);
    assert_eq!(
        g.formed_intents()
            .into_iter()
            .filter(|(d, ..)| *d == 1)
            .collect::<Vec<_>>(),
        vec![
            (1, 2, 1),
            (1, 3, 1),
            (1, 4, 1),
            (1, 5, 1),
            (1, 6, 1),
            (1, 7, 4),
            (1, 8, 2)
        ]
    );
    let expect: [(u32, IntentKind, u32, u32); 8] = [
        (2, take_kind(None), 1, 1),
        (3, hold_kind(2), 1, 1),
        (4, hold_kind(3), 1, 1),
        (5, deny_kind(), 1, 1),
        (6, neutralise_kind(2), 1, 1),
        (7, keep_kind(3, 1), 4, 1),
        (8, landing_kind(1, None), 2, 1),
        (9, keep_kind(2, 1), 3, 5),
    ];
    for (raw, kind, parent, day) in expect {
        let i = g.intent_state(raw);
        assert_eq!(i.kind, kind, "intent {raw}");
        assert_eq!(i.parent, Some(intent(parent)), "intent {raw}");
        assert_eq!(i.maker, Maker::Staff, "intent {raw}");
        assert_eq!(i.set_day, day, "intent {raw}");
    }
    assert_eq!(g.formed(keep_kind(3, 1)), vec![(1, 4)]);
    assert_eq!(g.formed(keep_kind(2, 1)), vec![(5, 3)]);
    assert_eq!(g.formed_intents().last(), Some(&(5, 9, 3)));
    assert_eq!(g.intent_state(6).withdrawn_day, Some(5));
    // The reason of each of the two commands that forms a supply.
    assert_says(&g.reasons_serving(1, 4), &[PRESS_WORDS]);
    assert_says(&g.reasons_serving(5, 3), &[PRESS_WORDS]);
    assert!(
        !all_reasons(g).contains("0 days (the margin at press)"),
        "no floor of 0 days is formed"
    );
}

/// The landing: the seven orders and their days; 12 t loaded on day 2; the
/// men and the 10 t landed on day 4 and the 2 t on day 5.
#[test]
fn the_landing_at_press() {
    let g = main_game();
    want_orders(
        g,
        vec![
            (1, vec![Given::Landing(1, B, vec![1])]),
            (2, vec![Given::LoadTo(1, n(12))]),
            (3, vec![Given::Attack(1), Given::LoadUnit(1, 1)]),
            (4, vec![route(1), Given::Unload(1)]),
            (5, vec![Given::Defend(1)]),
            (6, vec![]),
        ],
    );
    assert_eq!(g.ship(2, 1).supply_t, n(12));
    assert_eq!(
        g.landed(),
        vec![
            (4, Pulse::Morning, 1, 1, 160),
            (4, Pulse::Afternoon, 1, 1, 440)
        ]
    );
    let landed = g.supply_landed();
    assert_eq!(
        landed[..2],
        [(4, Pulse::Afternoon, 1, n(10)), (5, Pulse::Night, 1, n(2))]
    );
    assert_eq!(g.changed_side_day(2), Some(4));
    assert_eq!(g.base(4, 2).expect("base 2").side, Side::Allied);
    assert_eq!(g.base(4, 4).expect("base 4").name, "Landing at (4,0)");
}

/// The run that follows: none needed on day 6; run 1 of 45 t added on day
/// 7, loading day 7 and due day 8; its three orders; 40 t and 5 t landed
/// at base 2 on day 8; no second run.
#[test]
fn the_run_that_follows() {
    let g = main_game();
    want_orders(
        g,
        vec![
            (6, vec![]),
            (7, vec![Given::LoadTo(1, n(45))]),
            (8, vec![route(1), Given::Unload(1)]),
            (9, vec![]),
            (10, vec![]),
            (11, vec![]),
            (12, vec![]),
            (13, vec![]),
            (14, vec![]),
        ],
    );
    assert_eq!(g.all_orders().len(), 10);
    g.nothing_refused();
    let landed = g.supply_landed();
    assert_eq!(landed.len(), 4, "{landed:?}");
    assert_eq!(landed[2], (8, Pulse::Morning, 1, n(40)));
    assert_eq!(landed[3], (8, Pulse::Afternoon, 1, n(5)));
    assert_eq!(g.ship(7, 1).supply_t, n(45));
    // No run is ordered on day 6, and the reason says it is not needed.
    assert!(g.orders(6).is_empty());
    assert!(g.reasons_serving(6, 9).contains("not below the floor"));
}

/// Supply at the end of each day at the four Allied bases, as the table
/// has it and at every day between.
#[test]
fn supply_at_the_end_of_each_day() {
    let g = main_game();
    let base1 = [997, 982, 979, 979, 979, 979, 934];
    for day in 1..=14u32 {
        let want1 = base1[(day as usize - 1).min(6)];
        assert_eq!(g.supply(day, 1), n(want1), "base 1, day {day}");
        assert_eq!(
            g.supply(day, 3),
            fx("100") - fx("1.5") * Fixed::from_int(i64::from(day)),
            "base 3, day {day}"
        );
        let want2 = match day {
            1..=4 => n(0),
            5 => n(2),
            6 | 7 => n(0),
            d => n(45) - n(3) * Fixed::from_int(i64::from(d) - 8),
        };
        assert_eq!(g.supply(day, 2), want2, "base 2, day {day}");
    }
    let base4 = [10, 7, 6, 3, 0, 0, 0, 0, 0, 0, 0];
    for day in 4..=14u32 {
        assert_eq!(
            g.supply(day, 4),
            n(base4[day as usize - 4]),
            "base 4, day {day}"
        );
    }
    assert_eq!(g.supply(8, 2), n(45));
    assert_eq!(g.supply(9, 2), n(42));
    assert_eq!(g.supply(14, 2), n(27));
    assert_eq!(g.supply(14, 1), n(934));
}

/// The men: no man unfed on any night, 600 and 300 at the end, and no
/// `MenLostUnfed` in the game.
#[test]
fn every_man_is_fed() {
    let g = main_game();
    assert_eq!(g.unit_state(14, 1).men_ashore, 600);
    assert_eq!(g.unit_state(14, 3).men_ashore, 300);
    assert!(!g
        .events
        .iter()
        .any(|e| matches!(e.kind, EventKind::MenLostUnfed { .. })));
}

/// Property: never a night short.
#[test]
fn never_a_night_short() {
    let t = changed(
        &text(),
        "side = \"japanese\"\nq = 4\nr = 0\nsupply_t = 0",
        "side = \"japanese\"\nq = 4\nr = 0\nsupply_t = 3",
    );
    let g = play(&t);
    // Morning of day 7: 7.5 t at B, after the nights of days 5 and 6.
    assert_eq!(g.supply(6, 4), fx("7.5"));
    assert_eq!(g.supply(6, 2) + g.supply(6, 4), fx("7.5"));
    assert!(g.orders(6).is_empty());
    assert!(
        same(&g.orders(7), &[Given::LoadTo(1, n(45))]),
        "{:?}",
        g.orders(7)
    );
    assert_eq!(g.supply(7, 4), fx("4.5"));
    assert_eq!(g.supply(8, 4), fx("1.5"));
    assert_eq!(g.supply(14, 4), fx("1.5"));
    assert_eq!(g.supply(14, 2), n(27));
    assert!(!g
        .events
        .iter()
        .any(|e| matches!(e.kind, EventKind::MenLostUnfed { .. })));
    assert_eq!(g.unit_state(14, 1).men_ashore, 600);
}

/// Property: at balanced and at preserve nothing moves.
#[test]
fn at_balanced_and_at_preserve_nothing_moves() {
    let t = changed(&text(), RISK_BLOCK, "");
    let g = play(&t);
    want_orders(
        &g,
        vec![
            (1, vec![Given::Landing(1, B, vec![1])]),
            (2, vec![Given::LoadTo(1, n(27))]),
            (3, vec![Given::Attack(1), Given::LoadUnit(1, 1)]),
            (4, vec![route(1), Given::Unload(1)]),
            (5, vec![Given::Defend(1)]),
            (8, vec![Given::LoadTo(1, n(45))]),
            (9, vec![route(1), Given::Unload(1)]),
        ],
    );
    assert_eq!(g.all_orders().len(), 10);
    assert_eq!(g.formed(keep_kind(3, 5)), vec![(1, 4)]);
    assert_eq!(g.formed(keep_kind(2, 5)), vec![(5, 3)]);
    assert_eq!(g.supply(14, 1), n(919));
    assert_eq!(g.supply(14, 2), n(35));
    assert_eq!(g.supply(14, 3), n(79));
    assert_says(&g.reasons_serving(1, 4), &["(the margin at balanced)"]);
    let p = changed(&text(), "risk = \"press\"", "risk = \"preserve\"");
    let g = play(&p);
    assert_eq!(g.formed(keep_kind(3, 10)), vec![(1, 4)]);
    assert_says(
        &g.reasons_serving(1, 4),
        &["with a floor of 10 days (the margin at preserve)"],
    );
}

/// Property: the commander's own floor is his.
#[test]
fn the_commanders_own_floor_is_his() {
    let extra = format!(
        "{}{}",
        command_table(
            1,
            "kind = \"set_intent\"\nintent = 2\nintent_kind = \"keep_supplied\"\nbase = 3\nfloor_days = 3"
        ),
        command_table(
            1,
            "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 2\ntask_forces = [1]\nsource = 1"
        )
    );
    let g = play(&with(&extra));
    assert!(g.formed(keep_kind(3, 1)).is_empty(), "{:?}", g.forms());
    assert!(g.formed(keep_kind(3, 5)).is_empty(), "{:?}", g.forms());
    assert_eq!(g.intent_state(2).kind, keep_kind(3, 3));
    assert_eq!(g.intent_state(2).maker, Maker::Commander);
}

/// Property: the same twice.
#[test]
fn the_same_twice() {
    let a = play(&text());
    let b = play(&text());
    assert_eq!(a.hashes, b.hashes);
    assert_eq!(a.hashes.len(), 14);
}
