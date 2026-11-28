//! Scenario 135: men ashore beside a place. The commander says "control
//! the island" and nothing else. The island has one place, which the side
//! holds. One hex from it by a land link the side has a dump with 1,000
//! men and four days of food. On land joined to no place of the island it
//! has the twin of that dump. The staff feeds the men beside the place by
//! a run from the island's source, and does nothing for the twin.
//!
//! Proves decision 0231 as amended on review, against a hand calculation.
//! It stands on scenario 76's for an island's tree, its source of supply
//! and the supply doctrine's sums. The data is placeholder data: see
//! `data/scenarios/men-beside-a-place/135_men_beside_a_place.toml`. It
//! runs on the frozen step 1 figures with scenario 76's ground figures
//! file. No enemy man or ship is in reach of anything, so no die is
//! rolled.
//!
//! **Written before the rule is built.** Every test below is empty and
//! marked `#[ignore = "statement written, not built"]`. The build fills
//! each to the statement above it and takes the mark off. Until then the
//! scenario file plays, on the engine as it stands, the game "without the
//! rule": the men at D are lost as the men at T are.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! The supply doctrine's part of it (the day of the run, its load, its
//! hours and the stocks at D) was worked by hand and then checked on the
//! engine as it stood on Nov 24, 2026, before the rule existed, by a copy
//! of the scenario in which D is itself the island's place, so that the
//! supply of D is formed under a hold: the doctrine that serves the two is
//! one, and it gave these figures. The numbers of the intents and of the
//! plans, and everything about when the new intent is formed and
//! withdrawn, are the rule's own and were checked on nothing.
//!
//! ## Inputs
//!
//! The scenario's own map. A at (0,0), port size 5. D at (4,0), P at (5,0)
//! and N at (6,0), port size 0; land links join D to P and P to N. T at
//! (0,4) and J at (6,4), port size 0, joined to nothing. A to D is 160 nm
//! and A to T is 160 nm. The game is 14 days.
//!
//! | Base | Side | Hex | Supply |
//! | ---- | ---- | --- | ------ |
//! | 1, "Placeholder source A" | Allied | A | 1,000 t |
//! | 2, "Placeholder anchorage P" | Allied | P | 100 t |
//! | 3, "Placeholder dump D" | Allied | D | 20 t |
//! | 4, "Placeholder dump T" | Allied | T | 20 t |
//! | 5, "Placeholder shore J" | Japanese | J | 0 |
//!
//! Base 5 is not on the Allied map. No base has a reserve or fuel. The
//! Allied side holds no estimate and no contact.
//!
//! Island 1, "Placeholder island": base 2. Its land is P, D and N.
//!
//! Task force 1, Allied, on the transport mission: ships 1 and 2,
//! placeholder troop transports (10 kn, 3,000 t of cargo each), stopped
//! at base 1.
//!
//! | Unit | Where | Men |
//! | ---- | ----- | --- |
//! | 1, "Placeholder Beside" | D | 1,000 |
//! | 2, "Placeholder Garrison" | P | 300 |
//! | 3, "Placeholder Twin" | T | 1,000 |
//!
//! All are Allied fighting units, on the defence, experience 2, every
//! skill 2. A man eats 0.005 t a day; of the unfed one in twenty is lost
//! each night, cut to a whole man.
//!
//! The Allied side's commands of day 1: intent 1, control island 1. No
//! forces limit and no risk, so balanced. Both leaves are given.
//!
//! ## The tree on the first morning
//!
//! | Intent | Kind | Under | Made by |
//! | ------ | ---- | ----- | ------- |
//! | 1 | Control island 1 | none | the commander |
//! | 2 | Hold base 2 | 1 | the staff |
//! | 3 | Keep base 3 supplied, floor 5 days, no interval | 1 | the staff |
//! | 4 | Deny the waters within 3 hexes of (5,0) | 1 | the staff |
//! | 5 | Keep base 2 supplied, floor 5 days, no interval | 2 | the staff |
//!
//! The place is the side's, so no take is formed; no enemy base is on the
//! side's map, so nothing is neutralised. The source of supply is base 1,
//! which has the most supply of the side's bases in no named place's hex
//! (1,000 t against 20 t at base 3 and 20 t at base 4), with task force 1
//! stopped there and nothing aboard. The hold's forces limit is set to
//! base 1 and task force 1, as today. **Intent 3 is the rule's:** base 3
//! is on the island's land, in no named place's hex, and unit 1 is ashore
//! in its hex. It is formed after the hold and before the denial, with a
//! plan of its own, plan 2, and the staff sets its forces limit to base 1
//! and task force 1 that morning. Intent 5 then has plan 3. No intent
//! names base 4: T is joined to no place of the island. The denial finds
//! no force and no air group, forms nothing and hands in its notice each
//! morning, as it does without the rule.
//!
//! ## The run to D
//!
//! The supply doctrine is asked before the island's tree is formed, so
//! intent 3, formed on the morning of day 1, is first served on day 2.
//!
//! - **Day 2.** Plan 2 is approved and released on the leaves. The use at
//!   D is 1,000 x 0.005 = 5 t a day and the floor's stock 5 x 5 = 25 t.
//!   The stock at D is 15 t this morning. A run ordered today is due on
//!   day 3, and the reading checked is day 4's: 15 - 5 - 5 = 5 t, below
//!   25 t. A run is needed. Its load is the floor's stock on the day it
//!   arrives less the reading of that day, 25 - 10 = 15 t, plus 15 days
//!   of use, 75 t: 90 t. Ship 1's room covers it; the force goes whole.
//!   The staff adds run 1 of plan 2, 90 t, loading day 2 and due day 3,
//!   commits both ships from day 2 to day 5, and orders a load of supply
//!   to a weight of 90 t. Ship 1 loads 90 t that night.
//! - **Day 3.** The staff orders a route to D and back to A, acting at D,
//!   and an unload order. The force sails at hour 48 and arrives at hour
//!   64. Ship 1 lands 40 t in the morning and 50 t in the afternoon.
//! - **Day 4.** The force steams for A and lies stopped there at the end
//!   of the day.
//! - **No second run.** A run is next needed on the first morning the
//!   stock at D is below 35 t (25 t and two days' use). The stock on the
//!   morning of day 14 is 45 t.
//!
//! Three orders in all, each the staff's and each serving intent 3. No
//! staff order or command is refused.
//!
//! ## Supply, at the end of each day
//!
//! | Day | Base 1 | Base 2 | Base 3 (D) | Base 4 (T) |
//! | --- | ------ | ------ | ---------- | ---------- |
//! | 1 | 1,000 | 98.5 | 15 | 15 |
//! | 2 | 910 | 97 | 10 | 10 |
//! | 3 | 910 | 95.5 | 95 | 5 |
//! | 4 | 910 | 94 | 90 | 0 |
//! | 5 | 910 | 92.5 | 85 | 0 |
//! | 14 | 910 | 79 | 40 | 0 |
//!
//! Base 3 pays 5 t each night; on day 3 it pays 5 t in the night and takes
//! 90 t by day: 5 + 90 = 95 t; then 95 - 5 x 11 = 40 t at the end. Base 2
//! pays 300 x 0.005 = 1.5 t each night: 100 - 21 = 79 t, and its supply,
//! intent 5, needs no run on any day. Base 1 feeds no one and loads out
//! 90 t.
//!
//! ## The men
//!
//! - **Unit 1, at D,** is fed in full every night. It loses no man: 1,000
//!   at the end.
//! - **Unit 3, at T,** is the game without the rule. Base 4 feeds it on
//!   the nights of days 1 to 4 and is then empty. From the night of day 5
//!   every man is unfed, and one in twenty is lost each night, cut:
//!
//!   | Night of day | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 |
//!   | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
//!   | Lost | 50 | 47 | 45 | 42 | 40 | 38 | 36 | 35 | 33 | 31 |
//!   | Left | 950 | 903 | 858 | 816 | 776 | 738 | 702 | 667 | 634 | 603 |
//!
//!   397 men lost; 603 at the end. On the engine as it stood before the
//!   rule, unit 1 had these same figures.
//! - **Unit 2, at P,** is fed in full: 300 at the end.
//!
//! ## The measure, and what the plan order says
//!
//! The island's one place is the side's and holds more than 5 days of
//! supply on every day: the island is controlled on 14 of 14 days. Base 3
//! is no part of the measure. The plan order of intent 1 names base 3
//! among the bases fed beside its places, with its hex and the place it
//! is beside, and does not name base 4.
//!
//! # Properties
//!
//! - **It ends when the men go.** With a scenario order of day 6 for unit
//!   1 to march to P: the unit marches on days 6, 7 and 8 and is at D,
//!   fed there, until it arrives on day 8. It carries 5 days of its
//!   charge, 5 x 5 = 25 t, taken from base 3 and put into base 2. Base 3
//!   holds 80 t at the end of day 6, 75 t at the end of day 7, and on day
//!   8 pays 5 t in the night and gives 25 t to the march: 45 t, which it
//!   keeps to the end. Base 2 holds 100 - 1.5 x 8 + 25 = 113 t at the end
//!   of day 8 and then pays 1,300 x 0.005 = 6.5 t a night: 74 t at the
//!   end. On the mornings of days 7 and 8 intent 3 stands. On the morning
//!   of day 9 no man of the side is at D, none marches to it and no run
//!   of plan 2 is loaded or at sea: the staff withdraws intent 3, and plan
//!   2 is complete. No intent names base 3 after, and no run is added.
//!   The orders are the main case's three and the scenario's one.
//! - **Gone and come back.** With that order, and a scenario order of day
//!   10 for unit 1 to march to D: on the morning of day 9 the unit is at
//!   P with no march, and intent 3 is withdrawn, as above. A scenario's
//!   order is taken in before the staff is asked, so on the morning of
//!   day 10 unit 1 is marching to D: the staff forms a "keep base 3
//!   supplied" under intent 1 again, intent 6, with plan 4, and sets its
//!   forces limit. The unit marches on days 10, 11 and 12 and arrives on
//!   day 12 with 25 t taken from base 2. Base 2 holds 113 - 6.5 x 4 - 25
//!   = 62 t at the end of day 12 and pays 1.5 t on the nights of days 13
//!   and 14: 59 t at the end. Base 3 holds 45 + 25 = 70 t at the end of
//!   day 12 and pays 5 t on the nights of days 13 and 14: 60 t at the
//!   end. Intent 6 needs no run: on the mornings of days 11 and 12 no man
//!   is at D and there is no figure, and on the morning of day 13 the
//!   stock at D is 70 t and the reading checked 60 t, and on the morning
//!   of day 14 65 t and 55 t, against 25 t. No man is lost, and the one
//!   run of the game is the main case's. (Restated by the architect on
//!   Nov 24, 2026, on the build's finding: the first writing ordered the
//!   return on day 9 and said the day's orders are taken in after the
//!   staff is asked. A scenario's order is taken in before, so the unit
//!   was already marching back on the morning of day 9 and the supply was
//!   rightly kept and never withdrawn. The rule is unchanged; the return
//!   is ordered a day later so that the case is run. Checked on the
//!   built branch.)
//! - **Nobody there.** With unit 1 left out: no intent names base 3 on
//!   any day, the staff gives no order in the game, and base 3 holds 20 t
//!   at the end.
//! - **A hex with no base.** With a fourth unit, "Placeholder Bare", 200
//!   Allied men ashore at N: no intent names N; on each morning the staff
//!   records one reason for giving no order that names N, the unit and
//!   its 200 men (the men it then has), and says that no base of the side
//!   stands there to feed them through. The unit loses 10 men on the
//!   night of day 1 and is unfed every night. Everything of the main case
//!   at D, T and P is as written above.
//! - **More at the source is never worse.** With base 1 holding 2,000 t:
//!   base 3 holds at the end of each day what it holds in the main case.
//! - **At press.** With intent 1's risk set to press on day 1: intents 3
//!   and 5 are formed, each with a floor of 1 day, and neither is refused.
//!   Only the forming is asserted. (Restated by the architect on Nov 24,
//!   2026, for decision 0230: the first writing gave intent 3 a floor of
//!   0 days.)
//! - **What the side cannot see.** With base 5 holding 500 t and a
//!   Japanese unit of 400 men ashore at J: the Allied staff's commands,
//!   orders and reasons on every day are those of the main case, word for
//!   word.
//! - **The same twice.** The main case played twice gives the same daily
//!   hashes.
//!
//! # The intake's three changes
//!
//! Tested in `sim-core`'s own command tests, each way (decision 0231,
//! amended 1): a "keep supplied" of a base of the side is taken under
//! "control an island", and of a base of the enemy's is refused as
//! `CannotStandUnder`; so formed it has a plan of its own; and a floor of
//! 0 days is taken under "control an island" and still refused for a
//! "keep supplied" that stands alone.
#![allow(clippy::doc_lazy_continuation, clippy::doc_markdown)]

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::command::{
    CommandKind, CommandState, ForcesLimit, IntentKind, Limit, Maker, PlanOutcome, PlanState,
    Reason, StaffDecisions,
};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, LandUnitId, ShipId};
use sim_core::orders::{Cargo, LandUnitOrder, OrderKind, TaskForceOrder};
use sim_core::state::{Side, TruthState};
use sim_core::time::Pulse;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

use crate::common;
use crate::common::replace_once as changed;
use crate::common::values::{assert_says, fx, intent, n, text_of};

const A: Hex = Hex { q: 0, r: 0 };
const D: Hex = Hex { q: 4, r: 0 };
const P: Hex = Hex { q: 5, r: 0 };

fn path() -> PathBuf {
    common::data_dir().join("scenarios/men-beside-a-place/135_men_beside_a_place.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

/// The frozen step 1 figures; the scenario names its own ground figures
/// file, scenario 76's, which is frozen beside the others.
fn load(text: &str) -> Scenario {
    common::from_text_with(text, &path(), &common::frozen()).expect("the scenario loads")
}

struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    truth: Vec<TruthState>,
    events: Vec<Event>,
    hashes: Vec<u64>,
    state: CommandState,
    /// The measure of intent 1 as the agent interface reads it, after each
    /// day: index 0 before day 1.
    measures: Vec<Option<sim_agent::IslandMeasure>>,
    /// The plan orders of the side after each day: index 0 before day 1.
    plan_orders: Vec<Vec<sim_core::command::PlanOrder>>,
    /// The agent interface's answer for the plan order of intent 1 after
    /// each day: index 0 before day 1.
    answers: Vec<Option<serde_json::Value>>,
}

fn play(text: &str) -> Played {
    let mut scenario = load(text);
    let mut handed: BTreeMap<u32, StaffDecisions> = BTreeMap::new();
    let held = staffs();
    let mut truth = vec![scenario.world.umpire().clone()];
    let view = scenario.world.side_view(Side::Allied);
    let mut measures = vec![sim_agent::island_measure(&view, intent(1))];
    let mut plan_orders = vec![sim_core::command::Staff::plan_orders(
        held[&Side::Allied].as_ref(),
        view,
    )];
    let mut answers = vec![sim_agent::plan_order(&view, held[&Side::Allied].as_ref(), 1).ok()];
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
        let view = scenario.world.side_view(Side::Allied);
        measures.push(sim_agent::island_measure(&view, intent(1)));
        plan_orders.push(sim_core::command::Staff::plan_orders(
            held[&Side::Allied].as_ref(),
            view,
        ));
        answers.push(sim_agent::plan_order(&view, held[&Side::Allied].as_ref(), 1).ok());
    }
    Played {
        handed,
        truth,
        events: scenario.world.events().iter().cloned().collect(),
        hashes: scenario.world.day_hashes().to_vec(),
        state: scenario.world.command_state(Side::Allied).clone(),
        measures,
        plan_orders,
        answers,
    }
}

/// An order of the staff, plainly, with the task force or unit it is for.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Given {
    LoadTo(u32, Fixed),
    Route(u32, Vec<Hex>, Vec<Hex>),
    Unload(u32),
    Other(String),
}

fn given(o: &sim_core::command::StaffOrder) -> Given {
    match &o.kind {
        OrderKind::TaskForce { target, order } => {
            let tf = target.raw();
            match order {
                TaskForceOrder::LoadTo {
                    cargo: Cargo::Supply,
                    weight_t,
                    ..
                } => Given::LoadTo(tf, *weight_t),
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
            order: LandUnitOrder::SetPosture { .. },
        } => Given::Other(format!("unit {}: posture", target.raw())),
        other => Given::Other(format!("{other:?}")),
    }
}

fn route(tf: u32) -> Given {
    Given::Route(tf, vec![D, A], vec![D])
}

/// A staff command, plainly, by kind. Any other is `Other`, with its debug
/// text.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Cmd {
    Approve(u32),
    Form(u32, IntentKind),
    /// A forces limit: intent, task forces, source.
    Limit(u32, Vec<u32>, Option<u32>),
    /// A run added: plan, tons, loading day, sail day, due day.
    AddRun(u32, Fixed, u32, u32, u32),
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
                ..
            } = limit;
            assert!(
                ships.is_empty() && air_groups.is_empty() && land_units.is_empty(),
                "{limit:?}"
            );
            Cmd::Limit(
                intent.raw(),
                task_forces.iter().map(|t| t.raw()).collect(),
                source.map(|b| b.raw()),
            )
        }
        CommandKind::AddRun { plan, run } => Cmd::AddRun(
            plan.raw(),
            run.load_t,
            run.days.load,
            run.days.sail,
            run.days.arrive,
        ),
        CommandKind::MarkPlan {
            plan,
            outcome: PlanOutcome::Complete,
        } => Cmd::Complete(plan.raw()),
        CommandKind::WithdrawIntent { intent } => Cmd::Withdraw(intent.raw()),
        other => Cmd::Other(format!("{other:?}")),
    }
}

fn keep_kind(base: u32) -> IntentKind {
    IntentKind::KeepSupplied {
        base: BaseId::new(base),
        floor_days: 5,
        interval_days: None,
    }
}

fn hold_kind(base: u32) -> IntentKind {
    IntentKind::Hold {
        base: BaseId::new(base),
    }
}

fn deny_kind() -> IntentKind {
    IntentKind::Deny {
        hex: P,
        distance: 3,
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

    /// The forces limit the staff set for `of` on `day`: task forces and
    /// source.
    fn staff_limit(&self, day: u32, of: u32) -> Option<(Vec<u32>, Option<u32>)> {
        self.commands(day).into_iter().find_map(|c| match c {
            Cmd::Limit(i, tfs, source) if i == of => Some((tfs, source)),
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

    /// The runs the staff added: (day, plan, tons, loading, sailing, due).
    fn runs(&self) -> Vec<(u32, u32, Fixed, u32, u32, u32)> {
        self.all_commands()
            .into_iter()
            .filter_map(|(d, c)| match c {
                Cmd::AddRun(p, t, l, s, a) => Some((d, p, t, l, s, a)),
                _ => None,
            })
            .collect()
    }

    /// The reasons for giving no order of `day`, as text, each with the
    /// intent it serves.
    fn no_orders(&self, day: u32) -> Vec<(Option<u32>, String)> {
        self.handed[&day]
            .no_orders
            .iter()
            .map(|r| (r.serves.intent.map(|i| i.raw()), text_of(r)))
            .collect()
    }

    /// Every reason of `day` that serves intent `of`, as text.
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

    /// Everything the staff handed in on `day`, word for word.
    fn transcript(&self, day: u32) -> String {
        let d = &self.handed[&day];
        let mut all: Vec<&Reason> = d.commands.iter().map(|c| &c.reason).collect();
        all.extend(d.orders.iter().map(|o| &o.reason));
        all.extend(d.no_orders.iter());
        format!(
            "{:?} | {:?} | {}",
            self.commands(day),
            self.orders(day),
            all.into_iter()
                .map(text_of)
                .collect::<Vec<_>>()
                .join(" || ")
        )
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

    /// The men of unit `u` at the end of `day`.
    fn men(&self, day: u32, u: u32) -> u32 {
        self.unit_state(day, u).men_ashore
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

    fn intent_state(&self, raw: u32) -> &sim_core::command::Intent {
        &self.state.intents[&intent(raw)]
    }

    /// The transports' passages: where task force 1 arrived, and the hour.
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

/// A command or order table of the Allied side, added before the marker.
fn march_order(day: u32, unit: u32, hex: (i32, i32)) -> String {
    format!(
        "\n[[order]]\nday = {day}\nside = \"allied\"\nland_unit = {unit}\nkind = \"march\"\nhex = [{}, {}]\n",
        hex.0, hex.1
    )
}

fn with_orders(extra: &str) -> String {
    changed(
        &text(),
        "# MARKER: orders",
        &format!("{extra}\n# MARKER: orders"),
    )
}

fn main_game() -> &'static Played {
    static GAME: std::sync::OnceLock<Played> = std::sync::OnceLock::new();
    GAME.get_or_init(|| play(&text()))
}

// ---------------------------------------------------------------- Variants

/// The scenario with unit 1 left out.
fn without_unit_1() -> String {
    let t = text();
    let from = t.find("# MARKER: unit 1").expect("marker");
    let to = t.find("# MARKER: end of unit 1").expect("marker");
    format!("{}{}", &t[..from], &t[to..])
}

// ---------------------------------------------------------------- The hand calculation

#[test]
fn the_file_is_the_one_these_tests_vary() {
    let t = text();
    assert_eq!(t.matches("# MARKER: orders").count(), 1);
    assert_eq!(t.matches("# MARKER: more land units").count(), 1);
    assert!(t.contains("days = 14"));
    assert!(t.contains("intent_kind = \"control_island\"\nisland = 1\n"));
    assert!(!t.contains("limit = \"forces\""));
    assert!(!t.contains("limit = \"risk\""));
}

/// The tree on the first morning: intents 2 to 5 as the table has them,
/// plans 2 and 3, intent 3's forces limit, and no intent that names base 4.
#[test]
fn the_tree_on_the_first_morning() {
    let g = main_game();
    assert_eq!(g.days(), 14);
    // The four intents of the table, formed in that order on day 1, and
    // nothing else formed in the game: no take (the place is the side's),
    // no neutralising, nothing under the denial.
    assert_eq!(
        g.formed_intents(),
        vec![(1, 2, 1), (1, 3, 1), (1, 4, 1), (1, 5, 2)]
    );
    let expect: [(u32, IntentKind, u32, u32); 4] = [
        (2, hold_kind(2), 1, 1),
        (3, keep_kind(3), 1, 2),
        (4, deny_kind(), 1, 1),
        (5, keep_kind(2), 2, 3),
    ];
    for (raw, kind, parent, plan) in expect {
        let i = g.intent_state(raw);
        assert_eq!(i.kind, kind, "intent {raw}");
        assert_eq!(i.parent, Some(intent(parent)), "intent {raw}");
        assert_eq!(i.maker, Maker::Staff, "intent {raw}");
        assert_eq!(i.set_day, 1, "intent {raw}");
        assert_eq!(i.plan.raw(), plan, "plan of intent {raw}");
        assert_eq!(i.withdrawn_day, None, "intent {raw}");
    }
    assert_eq!(g.intent_state(1).plan.raw(), 1);
    // Intent 3 has a plan of its own, plan 2, and its forces limit is set to
    // base 1 and task force 1 on the morning it is formed; so is the hold's.
    assert_eq!(g.staff_limit(1, 3), Some((vec![1], Some(1))));
    assert_eq!(g.staff_limit(1, 2), Some((vec![1], Some(1))));
    let limits = g
        .all_commands()
        .into_iter()
        .filter(|(_, c)| matches!(c, Cmd::Limit(..)))
        .count();
    assert_eq!(limits, 2, "no other limit is set by the staff");
    // No intent names base 4: T is joined to no place of the island.
    assert!(g
        .state
        .intents
        .values()
        .all(|i| !format!("{:?}", i.kind).contains("BaseId(4)")));
    assert!(!g.forms().iter().any(|(_, _, k)| matches!(
        k,
        IntentKind::TakePlace { .. } | IntentKind::Neutralise { .. }
    )));
    // The denial finds no force and no air group, forms nothing and hands in
    // its reason each morning, as it does without the rule.
    for day in 1..=14 {
        let said = g.reasons_serving(day, 4);
        assert_says(&said, &["forms no task and cannot meet intent:4"]);
    }
    // Plan 2 is the supply of base 3's: its leaves are taken on day 2, the
    // supply doctrine being asked before the island's tree is formed.
    assert_eq!(g.plan_day(2, PlanState::Preparing), Some(2));
    assert_eq!(g.plan_day(2, PlanState::Released), Some(2));
    assert!(!g.commands(1).contains(&Cmd::Approve(2)));
    // The reason of the forming names the base, its hex, the men by unit,
    // the supply it holds and its days at today's use, the place it is
    // beside and what would change it.
    let said = g.reasons_serving(1, 1);
    assert_says(
        &said,
        &[
            "base:3 (Placeholder dump D) stands at (4,0)",
            "1 hex(es) from the place at (5,0)",
            "land_unit:1 (Placeholder Beside), 1000 men",
            "20 t of supply, 4 days at today's use",
            "a floor of 5 days (the margin at balanced)",
        ],
    );
    let formed = g.handed[&1]
        .commands
        .iter()
        .find(|c| matches!(c.kind, CommandKind::FormIntent { kind, .. } if kind == keep_kind(3)))
        .expect("the forming of intent 3");
    assert!(formed
        .reason
        .would_change
        .iter()
        .any(|w| w == "no man of the side at the base"));
}

/// The run to D: served first on day 2; run 1 of plan 2, 90 t, loading day
/// 2 and due day 3; the three orders, their days and what they serve; the
/// hours of sailing and arrival; 40 t landed in the morning and 50 t in the
/// afternoon of day 3; the force at A at the end of day 4; no second run.
#[test]
fn the_run_to_d() {
    let g = main_game();
    // Run 1 of plan 2: 90 t, loading on day 2, sailing on day 3, due day 3.
    assert_eq!(g.runs(), vec![(2, 2, n(90), 2, 3, 3)]);
    assert!(g.commands(2).contains(&Cmd::AddRun(2, n(90), 2, 3, 3)));
    // Both ships are committed from day 2 to day 5.
    assert!(g.commands(2).iter().any(|c| matches!(
        c,
        Cmd::Other(text) if text.contains("CommitShips")
            && text.contains("from_day: 2, to_day: 5")
            && text.contains("ShipId(1)")
            && text.contains("ShipId(2)")
    )));
    // Three orders in all, each the staff's and each serving intent 3.
    assert_eq!(
        g.all_orders(),
        vec![
            (2, Given::LoadTo(1, n(90))),
            (3, route(1)),
            (3, Given::Unload(1)),
        ]
    );
    for day in [2, 3] {
        for order in &g.handed[&day].orders {
            assert_eq!(order.reason.serves.intent, Some(intent(3)), "day {day}");
        }
    }
    g.nothing_refused();
    // Ship 1 loads 90 t that night.
    assert_eq!(g.ship(2, 1).supply_t, n(90));
    // The force sails at hour 48 from A, arrives at D at hour 64, steams for
    // A at hour 72 and lies stopped there at the end of day 4 (hour 88).
    let steaming = g.transport_steaming();
    assert_eq!(
        steaming.first(),
        Some(&(3, Pulse::Night, A)),
        "{steaming:?}"
    );
    assert!(steaming.contains(&(4, Pulse::Night, D)), "{steaming:?}");
    assert!(steaming.iter().all(|(day, _, _)| *day <= 4), "{steaming:?}");
    let arrivals = g.transport_arrivals();
    assert_eq!(arrivals, vec![(D, n(64)), (A, n(88))]);
    let force = &g.truth[4].task_forces[&sim_core::ids::TaskForceId::new(1)];
    assert_eq!(force.from_hex, A);
    assert_eq!(force.toward_hex, None);
    // Ship 1 lands 40 t in the morning and 50 t in the afternoon of day 3.
    assert_eq!(
        g.supply_landed(),
        vec![
            (3, Pulse::Morning, 1, n(40)),
            (3, Pulse::Afternoon, 1, n(50))
        ]
    );
    // No second run: the stock at D is 45 t on the morning of day 14, above
    // 35 t.
    assert_eq!(g.supply(13, 3), n(45));
    assert_eq!(g.runs().len(), 1);
}

/// Supply at the end of each day, at the four Allied bases, as the table
/// has it and at every day between.
#[test]
fn supply_at_the_end_of_each_day() {
    let g = main_game();
    // Base 1 feeds no one and loads out 90 t on day 2.
    assert_eq!(g.supply(1, 1), n(1000));
    for day in 2..=14 {
        assert_eq!(g.supply(day, 1), n(910), "base 1, day {day}");
    }
    // Base 2 pays 1.5 t each night.
    for day in 1..=14u32 {
        assert_eq!(
            g.supply(day, 2),
            n(100) - fx("1.5") * Fixed::from_int(i64::from(day)),
            "base 2, day {day}"
        );
    }
    assert_eq!(g.supply(1, 2), fx("98.5"));
    assert_eq!(g.supply(2, 2), n(97));
    assert_eq!(g.supply(3, 2), fx("95.5"));
    assert_eq!(g.supply(4, 2), n(94));
    assert_eq!(g.supply(5, 2), fx("92.5"));
    assert_eq!(g.supply(14, 2), n(79));
    // Base 3 pays 5 t each night, and on day 3 takes 90 t by day.
    let base_3 = [(1, 15), (2, 10), (3, 95), (4, 90), (5, 85), (14, 40)];
    for (day, t) in base_3 {
        assert_eq!(g.supply(day, 3), n(t), "base 3, day {day}");
    }
    for day in 3..=14u32 {
        assert_eq!(
            g.supply(day, 3),
            n(95 - 5 * i64::from(day - 3)),
            "base 3, day {day}"
        );
    }
    // Base 4, the twin, is fed for four nights and is then empty.
    let base_4 = [(1, 15), (2, 10), (3, 5), (4, 0), (5, 0), (14, 0)];
    for (day, t) in base_4 {
        assert_eq!(g.supply(day, 4), n(t), "base 4, day {day}");
    }
}

/// The men: unit 1 loses none; unit 3 loses the men of the table, night by
/// night, 603 left; unit 2 loses none.
#[test]
fn the_men_beside_the_place_are_fed_and_the_twin_is_not() {
    let g = main_game();
    for day in 1..=14 {
        assert_eq!(g.men(day, 1), 1000, "unit 1, day {day}");
        assert_eq!(g.men(day, 2), 300, "unit 2, day {day}");
    }
    let left = [
        (4, 1000),
        (5, 950),
        (6, 903),
        (7, 858),
        (8, 816),
        (9, 776),
        (10, 738),
        (11, 702),
        (12, 667),
        (13, 634),
        (14, 603),
    ];
    for (day, men) in left {
        assert_eq!(g.men(day, 3), men, "unit 3, day {day}");
    }
    assert_eq!(1000 - g.men(14, 3), 397);
}

/// The measure is 14 of 14 days, and the plan order of intent 1 names base
/// 3 as fed beside the place and not base 4.
#[test]
fn the_measure_and_the_plan_order() {
    let g = main_game();
    assert_eq!(g.controlled(), (0..=14).collect::<Vec<u32>>());
    assert_eq!(g.measures[14].as_ref().map(|m| m.days_in_force), Some(14));
    // After day 1, where intent 3 is in force: the staff's plan order lists
    // intent 3 among the tasks of intent 1 and no task for base 4.
    let order = g.plan_orders[1]
        .iter()
        .find(|o| o.intent == intent(1))
        .expect("the island's plan order");
    let tasks: Vec<(u32, IntentKind)> = order
        .tasks
        .iter()
        .map(|t| (t.intent.raw(), t.kind))
        .collect();
    assert!(tasks.contains(&(3, keep_kind(3))), "{tasks:?}");
    assert!(
        tasks
            .iter()
            .all(|(_, k)| !format!("{k:?}").contains("BaseId(4)")),
        "{tasks:?}"
    );
    // The agent interface's answer says so with the base's hex and the place
    // it is beside, and does not name base 4.
    let answer = g.answers[1].as_ref().expect("the plan order answer");
    let all = answer.to_string();
    assert_says(
        &all,
        &["It feeds beside its places: base:3 (Placeholder dump D) at (4,0), beside the place at (5,0)."],
    );
    assert!(!all.contains("base:4"), "{all}");
    assert!(answer["tasks"]
        .as_array()
        .expect("tasks")
        .iter()
        .any(|t| t["intent"] == 3 && t["kind"] == "keep_supplied"));
}

/// Property: it ends when the men go.
#[test]
fn it_ends_when_the_men_go() {
    let g = play(&with_orders(&march_order(6, 1, (5, 0))));
    // The unit marches on days 6, 7 and 8 and is at D until it arrives on
    // day 8, fed there.
    assert_eq!(g.unit_state(5, 1).hex, Some(D));
    assert_eq!(g.unit_state(6, 1).hex, Some(D));
    assert_eq!(g.unit_state(7, 1).hex, Some(D));
    assert_eq!(g.unit_state(8, 1).hex, Some(P));
    // Base 3 holds 80 t at the end of day 6, 75 t at the end of day 7, and on
    // day 8 pays 5 t in the night and gives 25 t to the march: 45 t, which it
    // keeps. Base 2 holds 113 t at the end of day 8 and 74 t at the end.
    assert_eq!(g.supply(6, 3), n(80));
    assert_eq!(g.supply(7, 3), n(75));
    for day in 8..=14 {
        assert_eq!(g.supply(day, 3), n(45), "base 3, day {day}");
    }
    assert_eq!(g.supply(8, 2), n(113));
    assert_eq!(g.supply(14, 2), n(74));
    // On the mornings of days 7 and 8 intent 3 stands; on the morning of day
    // 9 the staff withdraws it and plan 2 is complete.
    for day in 1..=8 {
        assert!(!g.commands(day).contains(&Cmd::Withdraw(3)), "day {day}");
    }
    assert!(g.commands(9).contains(&Cmd::Withdraw(3)));
    assert_eq!(g.intent_state(3).withdrawn_day, Some(9));
    assert_eq!(g.plan_day(2, PlanState::Complete), Some(9));
    // No intent names base 3 after, and no run is added.
    assert!(!g
        .all_commands()
        .iter()
        .any(|(d, c)| *d >= 9 && matches!(c, Cmd::Form(_, _))));
    assert_eq!(g.runs().len(), 1);
    // The orders are the main case's three; the scenario's one is not the
    // staff's.
    assert_eq!(g.all_orders(), main_game().all_orders());
    g.nothing_refused();
    assert_eq!(g.men(14, 1), 1000);
}

/// Property: gone and come back.
#[test]
fn gone_and_come_back() {
    let g = play(&with_orders(&format!(
        "{}{}",
        march_order(6, 1, (5, 0)),
        march_order(10, 1, (4, 0))
    )));
    // On the morning of day 9 the unit is at P with no march, and intent 3
    // is withdrawn.
    assert_eq!(g.unit_state(8, 1).hex, Some(P));
    assert!(g.unit_state(8, 1).march.is_none());
    assert!(g.commands(9).contains(&Cmd::Withdraw(3)));
    assert_eq!(g.intent_state(3).withdrawn_day, Some(9));
    assert!(!g.commands(9).iter().any(|c| matches!(c, Cmd::Form(..))));
    // The order is taken in before the staff is asked: on the morning of
    // day 10 unit 1 is marching to D, and the staff forms a new supply of
    // base 3 under intent 1, intent 6, with plan 4, and sets its forces
    // limit.
    assert_eq!(g.formed_intents().last(), Some(&(10, 6, 1)));
    assert!(g.commands(10).contains(&Cmd::Form(1, keep_kind(3))));
    assert_eq!(g.intent_state(6).kind, keep_kind(3));
    assert_eq!(g.intent_state(6).plan.raw(), 4);
    assert_eq!(g.intent_state(6).withdrawn_day, None);
    assert_eq!(g.staff_limit(10, 6), Some((vec![1], Some(1))));
    assert_says(&g.reasons_serving(10, 1), &["marching here from (5,0)"]);
    // The unit marches on days 10, 11 and 12 and arrives on day 12 with 25 t
    // taken from base 2.
    assert_eq!(g.unit_state(11, 1).hex, Some(P));
    assert_eq!(g.unit_state(12, 1).hex, Some(D));
    assert_eq!(g.supply(12, 2), n(62));
    assert_eq!(g.supply(14, 2), n(59));
    assert_eq!(g.supply(12, 3), n(70));
    assert_eq!(g.supply(14, 3), n(60));
    // Intent 6 needs no run: the stock at D is 70 t on the morning of day 13
    // and 65 t on day 14, and the one run of the game is the main case's.
    assert_eq!(g.supply(13, 3), n(65));
    assert_eq!(g.runs().len(), 1);
    assert_eq!(g.men(14, 1), 1000);
    g.nothing_refused();
}

/// Property: nobody there.
#[test]
fn nobody_there() {
    let g = play(&without_unit_1());
    assert!(!g
        .forms()
        .iter()
        .any(|(_, _, k)| matches!(k, IntentKind::KeepSupplied { base, .. } if base.raw() == 3)));
    assert!(g.all_orders().is_empty(), "{:?}", g.all_orders());
    assert!(g.runs().is_empty());
    assert_eq!(g.supply(14, 3), n(20));
    g.nothing_refused();
}

/// Property: a hex with no base.
#[test]
fn a_hex_with_no_base() {
    let extra = r#"
[[land_unit]]
id = 4
name = "Placeholder Bare"
side = "allied"
men = 200
q = 6
r = 0
"#;
    let g = play(&changed(
        &text(),
        "# MARKER: more land units",
        &format!("{extra}\n# MARKER: more land units"),
    ));
    // No intent names N, and everything of the main case at D, T and P is as
    // written.
    assert_eq!(
        g.formed_intents(),
        vec![(1, 2, 1), (1, 3, 1), (1, 4, 1), (1, 5, 2)]
    );
    for day in 1..=14 {
        assert_eq!(g.supply(day, 3), main_game().supply(day, 3));
        assert_eq!(g.men(day, 1), 1000);
        assert_eq!(g.men(day, 3), main_game().men(day, 3));
    }
    // Each morning one reason names N, the unit and the men it then has, and
    // says that no base of the side stands there to feed them through.
    for day in 1..=14u32 {
        let said: Vec<String> = g
            .no_orders(day)
            .into_iter()
            .map(|(_, text)| text)
            .filter(|text| text.contains("(6,0)"))
            .collect();
        assert_eq!(said.len(), 1, "day {day}: {said:?}");
        let men = if day == 1 { 200 } else { g.men(day - 1, 4) };
        assert_says(
            &said[0],
            &[
                &format!("land_unit:4 (Placeholder Bare), {men} men"),
                "no base of the side stands there to feed them through",
            ],
        );
    }
    // The unit loses 10 men on the night of day 1 and is unfed every night.
    assert_eq!(g.men(1, 4), 190);
    for day in 2..=14 {
        assert!(g.men(day, 4) < g.men(day - 1, 4), "day {day}");
    }
}

/// Property: more at the source is never worse.
#[test]
fn more_at_the_source_is_never_worse() {
    let g = play(&changed(
        &text(),
        "# MARKER: the source's supply\nsupply_t = 1000",
        "# MARKER: the source's supply\nsupply_t = 2000",
    ));
    for day in 1..=14 {
        let (more, main) = (g.supply(day, 3), main_game().supply(day, 3));
        assert!(more >= main, "day {day}: {more} against {main}");
        assert_eq!(more, main, "day {day}");
    }
    assert_eq!(g.men(14, 1), 1000);
}

/// Property: at press.
#[test]
fn at_press() {
    let command = r#"
[[command]]
day = 1
side = "allied"
kind = "set_limit"
limit = "risk"
intent = 1
risk = "press"
"#;
    let g = play(&format!("{}{command}", text()));
    // Intents 3 and 5 are formed, each with a floor of 1 day, and neither is
    // refused. Only the forming is asserted.
    for (raw, base) in [(3, 3), (5, 2)] {
        assert_eq!(
            g.intent_state(raw).kind,
            IntentKind::KeepSupplied {
                base: BaseId::new(base),
                floor_days: 1,
                interval_days: None
            },
            "intent {raw}"
        );
    }
    assert_eq!(g.intent_state(3).parent, Some(intent(1)));
    assert!(g
        .refused()
        .iter()
        .all(|e| !matches!(e.kind, EventKind::CommandRefused { .. })));
}

/// Property: what the side cannot see.
#[test]
fn what_the_side_cannot_see() {
    let t = changed(
        &text(),
        "# MARKER: base 5's supply\nsupply_t = 0",
        "# MARKER: base 5's supply\nsupply_t = 500",
    );
    let japanese = r#"
[[land_unit]]
id = 5
name = "Placeholder Japanese company"
side = "japanese"
men = 400
q = 6
r = 4
"#;
    let g = play(&changed(
        &t,
        "# MARKER: more land units",
        &format!("{japanese}\n# MARKER: more land units"),
    ));
    let main = main_game();
    assert_eq!(g.days(), main.days());
    for day in 1..=14 {
        assert_eq!(g.transcript(day), main.transcript(day), "day {day}");
    }
}

/// Property: the same twice.
#[test]
fn the_same_twice() {
    let again = play(&text());
    assert_eq!(again.hashes, main_game().hashes);
    assert_eq!(again.hashes.len(), 14);
}
