//! Scenario 76: controlling an island. The commander says "control the
//! island" and nothing else. The island has two places: an airfield the
//! enemy holds with nobody on it, and an anchorage the side holds with a
//! small garrison. The staff forms the tree, takes the airfield, holds
//! both, and feeds them.
//!
//! Proves decision 0151, step 8 of stage 3, second part, against a hand
//! calculation. It stands on scenario 69's for a place taken and held
//! and on scenario 74's for the force the staff assembles. The data is
//! placeholder data: see `data/scenarios/76_controlling_an_island.toml`.
//! It runs on the frozen step 1 figures, with one ground figure of its
//! own, `preparation_per_day` 100. No enemy man is ashore anywhere, so no
//! die is rolled.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Inputs
//!
//! The placeholder map of scenario 27: A at (0,0), port size 5; B at
//! (4,0), port size 0; C at (0,6), port size 0. The game is 14 days.
//!
//! | Base | Side | Hex | Supply |
//! | ---- | ---- | --- | ------ |
//! | 1, "Placeholder base A" | Allied | A | 1,000 t |
//! | 2, "Placeholder airfield B" | Japanese | B | 0 |
//! | 3, "Placeholder anchorage C" | Allied | C | 100 t |
//!
//! Base 2 is on the Allied map and has no gun, aircraft or man. No base
//! has a reserve or fuel. The Allied side holds no estimate and no
//! contact.
//!
//! Island 1, "Placeholder island": base 2, then base 3.
//!
//! Task force 1, Allied, on the transport mission: ships 1 and 2,
//! placeholder troop transports, stopped at base 1.
//!
//! | Unit | Kind | Where | Men |
//! | ---- | ---- | ----- | --- |
//! | 1, "Placeholder First" | fighting | A | 600 |
//! | 3, "Placeholder Garrison" | fighting | C | 300 |
//!
//! Both are Allied, on the defence, experience 2, every skill 2.
//!
//! The Allied side's commands of day 1: intent 1, control island 1. No
//! forces limit and no risk, so balanced. Leave to prepare and leave to
//! launch are both given.
//!
//! ## The tree on the first morning
//!
//! | Kind | Under | Made by |
//! | ---- | ----- | ------- |
//! | Control island 1 | none | the commander |
//! | Take the place at base 2 | control | the staff |
//! | Hold base 2 | control | the staff |
//! | Hold base 3 | control | the staff |
//! | Deny the waters within 3 hexes of (4,0) | control | the staff |
//! | Neutralise base 2 | control | the staff |
//! | Ready a landing by task force 1 at B | the take | the staff |
//! | Keep base 3 supplied, floor 5 days | the hold of base 3 | the staff |
//!
//! The first five formed are intents 2 to 6, in that order. The side
//! has no carrier force: no carrier task is formed. Base 2 is the
//! enemy's and at a named place: it is neutralised until it is taken,
//! and the neutralising finds no bomber and no bombardment force and
//! forms nothing under it. On the morning of day 5, base 2 the side's,
//! the staff withdraws intent 6. No enemy base lies off the island. The
//! source of
//! supply is base 1, the side's only base off the island, where task
//! force 1 lies: each hold's forces limit is set to task force 1 and
//! base 1. The denial finds no force and no air group and forms nothing
//! under it. The hold of base 2 waits. "Keep base 2 supplied" is formed
//! under the hold of base 2 on the morning of day 5.
//!
//! ## The airfield
//!
//! The take chooses by decision 0149: no estimate, so every unit the
//! ships can carry; the source is base 1, the only base with a transport
//! force and a unit; unit 1; task force 1. The check of decision 0144:
//! the need is 600 x 0.005 = 3 t a day; supply follows in 4 days; the
//! margin is 5; the supply that lands with the men is 3 x 9 = 27 t; the
//! first run is 45 t; base 1 holds 997 t on the morning of day 2. It can
//! be held.
//!
//! | Day | Hour | The staff's orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 | 0 | `prepare_landing` at B, roster 1 | Loading 100; rehearsal 100 |
//! | 2 | 24 | The plan of the take is released. A load of supply to a weight of 27 t | |
//! | 3 | 48 | Unit 1 set to attack. Load unit 1 | |
//! | 4 | 72 | A route to B and back to A, acting at B, and an unload order | Sails at hour 72, arrives at hour 88. Ship 1 lands 160 men in the morning, 440 in the afternoon and then 10 t. Base 4, "Landing at (4,0)", is formed. Base 2 changes side at the end of the day |
//! | 5 | 96 | The take's plan is marked complete; notice. Unit 1 set to defend | Ship 1 lands 17 t in 0.85 hours. The force steams for A at hour 108 and arrives at hour 124 |
//! | 8 | 168 | A run for base 2: a load of supply to a weight of 45 t | |
//! | 9 | 192 | A route to B and back to A, acting at B, and an unload order | Sails at hour 192, arrives at hour 208. Ship 1 lands 40 t in the morning and 5 t in the afternoon |
//!
//! Ten orders in all. No staff order or command is refused.
//!
//! ## The supply doctrine's sums
//!
//! - **Base 2.** The use is 3 t a day and the floor's stock 15 t. The
//!   stock at B is 24 t on the morning of day 6, 21 t on day 7 and 18 t
//!   on day 8. A run ordered on a morning arrives the next day, and the
//!   reading checked is the day after, 6 t lower: 18 t, 15 t and 12 t.
//!   The first below 15 is on the morning of day 8. Its load is 0 plus
//!   15 days of use, 45 t.
//! - **Base 3.** The use is 300 x 0.005 = 1.5 t a day and the floor's
//!   stock 7.5 t. The stock is 100 t and falls to 79 t by the end: no
//!   run is needed on any day.
//!
//! ## Supply
//!
//! - **Base 1** feeds 600 men, 3 t, on the nights of days 1 to 3, and
//!   loads out 27 t and 45 t: 1,000 - 9 - 72 = 919 t at the end.
//! - **Base 4** is given 10 t on the afternoon of day 4. On the night of
//!   day 5 base 2 holds nothing and base 4 pays 3 t: 7 t, which it keeps.
//! - **Base 2** takes 17 t on the night of day 5, after that night's
//!   charge. It pays 3 t on each night from day 6: 14, 11, 8 and 5 t
//!   after the nights of days 6 to 9; takes 45 t on day 9: 50 t; and
//!   pays 3 t on the nights of days 10 to 14: 35 t at the end.
//! - **Base 3** pays 1.5 t each night: 79 t at the end.
//!
//! ## The measure
//!
//! A place counts on a day when it holds a base of the side at the end
//! of the day and the days of supply in its hex are at least the floor,
//! 5 days. C holds 100 t falling to 79 t at 1.5 t a day: it counts on
//! every day. B is the side's from the end of day 4. At the end of day 4
//! its hex holds 10 t at 3 t a day, 3.33 days: it does not count. At the
//! end of days 5 to 8 it holds 24, 21, 18 and 15 t: 8, 7, 6 and 5 days.
//! At the end of day 9 it holds 57 t, and 54 t down to 42 t at the end
//! of days 10 to 14. The island is controlled on days 5 to 14: on 10 of
//! 14 days in force.
//!
//! Restated by the architect on Nov 17, 2026, with decision 0151's
//! amendment: the first writing counted any supply at all and gave 11 of
//! 14; it had no neutralising of base 2, and no property of a carrier or
//! of a date.
//!
//! # Properties
//!
//! - **Both places the side's already.** With base 2 Allied, holding
//!   100 t of supply, and unit 1 ashore there: no take and no
//!   neutralising is formed; both holds are formed and each forms its
//!   supply on day 1; the island is controlled on 14 of 14 days.
//! - **A commander's own hold stands.** With the commander's own intent
//!   "hold base 3" stated on day 1 beside intent 1: the staff forms no
//!   hold of base 3 and its reason names the commander's.
//! - **The commander names the source.** With a forces limit on intent 1
//!   that names base 1 as the source: the tree and the orders are those
//!   of the main case.
//! - **No source of the landing.** With task force 1 left out: no base
//!   can be the source of the landing; the take hands in the notice
//!   that it cannot meet its intent on day 1 and gives the reason each
//!   morning. Base 1 is the source of supply from day 1, with no force
//!   (decision 0163, item 1), and "keep base 3 supplied", with a floor
//!   of 5 days, is formed under the hold of its place (intent 4) on
//!   day 1, with no ship to send. The island is controlled on 0 of 14 days. (Restated
//!   by the architect on Nov 19, 2026, for decision 0163: before it no
//!   base could be the source of supply either, and the holds formed no
//!   supply.)
//! - **An enemy base near.** With a Japanese base 5 on the Allied map at
//!   (2,3), in no named place's hex: "neutralise base 2" and then
//!   "neutralise base 5" are formed on day 1, after the denial, and each
//!   finds no bomber and no bombardment force. Only the forming is
//!   asserted: base 5 lies 3 hexes from B, so the lift is a run by night
//!   (decision 0145) and its hours are not the main case's.
//! - **A carrier.** With scenario 73's carrier force, its ship and its
//!   three air groups as they are there, stopped at base 1 with base 1
//!   its home port: "operate task force 2 from (0,0)" is formed under
//!   the denial on day 1, the force lying within its bombers' reach of
//!   B, 160 nm. The take forms no carrier task. On the morning of day 2
//!   the take's plan is released and group 3 is set to the base strike
//!   at base 2: it launches at base 2 on days 2, 3 and 4 and on no other
//!   day. The force is given no route on any day: it is not sent home
//!   when the take is met. The landing is the main case's.
//! - **A bombardment force.** With scenario 74's battleship force, its
//!   ship as it is there, as task force 3 on the bombardment mission,
//!   stopped at base 1 with base 1 its home port: "bombard base 2 by
//!   task force 3" is formed on day 1 under the neutralising of base 2,
//!   and no bombard task is formed under the take, whose limit names no
//!   bombardment force. On the morning of day 5, base 2 the side's, the
//!   neutralising is withdrawn and the bombard task with it, and task
//!   force 3 is given no route on that morning or after. Only the
//!   forming and the withdrawal are asserted: the nights it fires are
//!   the bombard doctrine's (scenario 71). Added by the architect on Nov
//!   18, 2026, with decision 0151's item 8c.
//! - **By a date.** With the island asked for by day 10: the take is
//!   formed with the date of day 10, and the task "ready a landing" under
//!   it with day 8, the take's day less the 2 days to embark and the day
//!   of passage, plus 1 (decision 0142, item 13; restated by the architect
//!   on Nov 18, 2026: the first writing gave the task day 10). Only the
//!   forming is asserted.
//! - **Without leave to launch** the plan of intent 1 is approved to
//!   preparing, nothing is formed, and the notice that it waits for
//!   leave to launch is handed in each morning.
//! - **Withdrawn.** With intent 1 withdrawn on day 6: every intent under
//!   it is withdrawn with it, and no order is given from day 6.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see give the same orders and reasons; a
//!   save on day 7 and a resume give the same game; every scenario built
//!   before this one gives the events it gave.
//!
//! # Decision 0217
//!
//! The architect's statement 8: "In scenario 76's carrier variant the deny
//! the staff forms under the order to control the island has a carrier task
//! under it, and on no day of the 14 is the notice that it cannot be met
//! handed in for that deny; its reason for the bases names the carrier
//! force first and has no 'cannot meet'. Every order of every day is what
//! it was."

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::command::{
    CommandKind, CommandState, ForcesLimit, IntentKind, Limit, Maker, NoticeKind, PlanOutcome,
    PlanState, Reason, StaffDecisions, StaffOrder, Staffs,
};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, LandUnitId, ShipId, TaskForceId};
use sim_core::orders::{
    AirGroupOrder, AirMission, Cargo, LandUnitOrder, OrderKind, Posture, TaskForceOrder,
};
use sim_core::state::{Side, TruthState};
use sim_core::time::Pulse;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

mod common;

const COMMANDS_MARKER: &str = "# The Allied commander's commands";
const A: Hex = Hex { q: 0, r: 0 };
const B: Hex = Hex { q: 4, r: 0 };

fn path() -> PathBuf {
    common::data_dir().join("scenarios/76_controlling_an_island.toml")
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

fn intent(raw: u32) -> IntentId {
    IntentId::new(raw)
}

fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

/// The frozen step 1 figures. The scenario names its own ground figures,
/// `data/reference/ground_placeholder_scenario_76.toml`, frozen beside the
/// others.
fn load(text: &str) -> Scenario {
    common::from_text_with(text, &path(), &common::frozen()).expect("the scenario loads")
}

/// The figures of the carrier property: the frozen step 1 figures with
/// scenario 53's table of aircraft for each ship, added after the frozen
/// file's empty `[strike_aircraft_per_ship]` table, as scenario 73 has it.
fn carrier_figures() -> sim_data::LoadOptions {
    common::altered_reference("s76", |dir| {
        common::edit_file(dir, "air_combat.toml", |text| {
            format!("{text}\ncruiser = 12\ndestroyer = 6\ntransport = 6\nauxiliary = 4\n")
        })
    })
}

fn load_carrier(text: &str) -> Scenario {
    common::from_text_with(text, &path(), &carrier_figures()).expect("the scenario loads")
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

fn play_carrier(text: &str) -> Played {
    play_from(load_carrier(text), BTreeMap::new())
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

fn keep_kind(base: u32) -> IntentKind {
    IntentKind::KeepSupplied {
        base: BaseId::new(base),
        floor_days: 5,
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

/// A command table of the Allied side, added to the end of the file.
fn command_table(day: u32, body: &str) -> String {
    format!("\n[[command]]\nday = {day}\nside = \"allied\"\n{body}\n")
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

#[test]
fn the_commands_of_the_file_are_the_ones_these_tests_vary() {
    let t = text();
    assert_eq!(t.matches(COMMANDS_MARKER).count(), 1);
    assert!(
        !t.contains("limit = \"forces\""),
        "no forces limit is named"
    );
    assert!(t.contains("days = 14"));
    assert!(t.contains("intent_kind = \"control_island\"\nisland = 1\n"));
    assert!(!t.contains("by_day"));
    assert_eq!(t.matches("leave = \"launch\"\ngiven = true").count(), 1);
}

#[test]
fn the_tree_of_the_first_morning() {
    let g = main_game();
    assert_eq!(g.days(), 14);
    // Intents 2 to 6 are the first five formed, in that order, then the
    // supply of base 3 under the hold of base 3 (intent 7) and the landing
    // under the take (intent 8); all on day 1.
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
    let expect: [(u32, IntentKind, u32); 7] = [
        (2, take_kind(None), 1),
        (3, hold_kind(2), 1),
        (4, hold_kind(3), 1),
        (5, deny_kind(), 1),
        (6, neutralise_kind(2), 1),
        (7, keep_kind(3), 4),
        (8, landing_kind(1, None), 2),
    ];
    for (raw, kind, parent) in expect {
        let i = g.intent_state(raw);
        assert_eq!(i.kind, kind, "intent {raw}");
        assert_eq!(i.parent, Some(intent(parent)), "intent {raw}");
        assert_eq!(i.maker, Maker::Staff, "intent {raw}");
        assert_eq!(i.set_day, 1, "intent {raw}");
    }
    let one = g.intent_state(1);
    assert_eq!(
        one.kind,
        IntentKind::ControlIsland {
            island: 1,
            by_day: None
        }
    );
    assert_eq!(one.parent, None);
    assert_eq!(one.maker, Maker::Commander);
    // The side has no carrier force: no carrier task is formed.
    assert!(!g
        .forms()
        .iter()
        .any(|(_, _, k)| matches!(k, IntentKind::OperateCarriers { .. })));
    // The neutralising finds no bomber and no bombardment force and forms
    // nothing under it; nor does the denial.
    assert!(!g
        .formed_intents()
        .iter()
        .any(|(_, _, p)| *p == 5 || *p == 6));
}

#[test]
fn the_holds_are_limited_to_task_force_1_and_base_1() {
    let g = main_game();
    assert_eq!(g.staff_limit(1, 3), Some((vec![1], vec![], Some(1))));
    assert_eq!(g.staff_limit(1, 4), Some((vec![1], vec![], Some(1))));
    // The take chooses by decision 0149: unit 1, task force 1, source base 1.
    assert_eq!(g.staff_limit(1, 8), Some((vec![1], vec![1], Some(1))));
    // No other limit is set by the staff.
    let limits = g
        .all_commands()
        .into_iter()
        .filter(|(_, c)| matches!(c, Cmd::Limit(..)))
        .count();
    assert_eq!(limits, 3);
}

#[test]
fn keep_base_3_supplied_stands_under_the_hold_of_base_3_with_floor_5() {
    let g = main_game();
    assert_eq!(g.formed(keep_kind(3)), vec![(1, 4)]);
    assert_eq!(g.formed(landing_kind(1, None)), vec![(1, 2)]);
}

#[test]
fn the_take_has_a_plan_of_its_own() {
    let g = main_game();
    let plan_of_one = g.intent_state(1).plan.raw();
    let plan = g.intent_state(2).plan.raw();
    assert_ne!(plan, plan_of_one);
    assert_eq!(g.intent_state(8).plan.raw(), plan);
    assert_eq!(g.plan_day(plan, PlanState::Preparing), Some(1));
    assert_eq!(g.plan_day(plan, PlanState::Released), Some(2));
    assert_eq!(g.plan_day(plan, PlanState::Complete), Some(5));
}

#[test]
fn intent_6_is_withdrawn_on_day_5_by_the_staff() {
    let g = main_game();
    assert_eq!(g.commands(5).first(), Some(&Cmd::Withdraw(6)));
    assert_eq!(g.intent_state(6).withdrawn_day, Some(5));
    // Nothing else is withdrawn.
    for raw in [1, 2, 3, 4, 5, 7, 8, 9] {
        assert_eq!(g.intent_state(raw).withdrawn_day, None, "intent {raw}");
    }
}

#[test]
fn keep_base_2_supplied_is_formed_on_day_5_under_the_hold_of_base_2() {
    let g = main_game();
    assert_eq!(g.formed(keep_kind(2)), vec![(5, 3)]);
    assert_eq!(g.formed_intents().last(), Some(&(5, 9, 3)));
    assert_eq!(g.intent_state(9).kind, keep_kind(2));
    // Nothing of the sort before the base is the side's.
    for day in 1..=4 {
        assert!(!g.commands(day).contains(&Cmd::Form(3, keep_kind(2))));
    }
}

#[test]
fn the_orders_of_days_1_to_5() {
    let g = main_game();
    let wanted: Vec<(u32, Vec<Given>)> = vec![
        (1, vec![Given::Landing(1, B, vec![1])]),
        (2, vec![Given::LoadTo(1, n(27))]),
        (3, vec![Given::Attack(1), Given::LoadUnit(1, 1)]),
        (4, vec![route(1), Given::Unload(1)]),
        (5, vec![Given::Defend(1)]),
        (6, vec![]),
        (7, vec![]),
    ];
    for (day, want) in wanted {
        assert!(
            same(&g.orders(day), &want),
            "day {day}: {:?} against {want:?}",
            g.orders(day)
        );
    }
}

#[test]
fn the_orders_of_days_8_and_9_and_ten_in_all_none_refused() {
    let g = main_game();
    let wanted: Vec<(u32, Vec<Given>)> = vec![
        (8, vec![Given::LoadTo(1, n(45))]),
        (9, vec![route(1), Given::Unload(1)]),
        (10, vec![]),
        (11, vec![]),
        (12, vec![]),
        (13, vec![]),
        (14, vec![]),
    ];
    for (day, want) in wanted {
        assert!(
            same(&g.orders(day), &want),
            "day {day}: {:?} against {want:?}",
            g.orders(day)
        );
    }
    // Ten in all, none refused. Unit 3 and task force 2 are given none.
    assert_eq!(g.all_orders().len(), 10);
    g.nothing_refused();
}

#[test]
fn the_landing_and_the_hours() {
    let g = main_game();
    // Day 1: loading 100; unit 1's rehearsal 100.
    assert_eq!(g.unit_state(1, 1).rehearsal, n(100));
    // Day 2: ship 1 loads 27 t.
    assert_eq!(g.ship(2, 1).supply_t, n(27));
    // Day 3: unit 1 is on attack.
    assert_eq!(g.unit_state(3, 1).posture, Posture::Attack);
    // The transports sail at hour 72, the night of day 4, from A, and arrive
    // at hour 88; they steam for A at hour 108 and arrive at hour 124.
    assert_eq!(
        g.transport_steaming().first().copied(),
        Some((4, Pulse::Night, A))
    );
    let arrivals = g.transport_arrivals();
    assert_eq!(arrivals[..2], [(B, n(88)), (A, n(124))]);
    // Ship 1 lands 160 men in the morning and 440 in the afternoon.
    assert_eq!(
        g.landed(),
        vec![
            (4, Pulse::Morning, 1, 1, 160),
            (4, Pulse::Afternoon, 1, 1, 440)
        ]
    );
    // Then 10 t on day 4 and 17 t on day 5, in 0.85 hours.
    let landed = g.supply_landed();
    assert_eq!(
        landed[..2],
        [(4, Pulse::Afternoon, 1, n(10)), (5, Pulse::Night, 1, n(17))]
    );
    assert_eq!(g.unloading_complete().first(), Some(&(2, fx("96.85"))));
    // The first man forms a base of the side, "Landing at (4,0)".
    let formed: Vec<(u32, BaseId)> = g
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::BaseFormed {
                base,
                side: Side::Allied,
                hex,
                ..
            } if *hex == B => Some((e.day, *base)),
            _ => None,
        })
        .collect();
    assert_eq!(formed.len(), 1, "{formed:?}");
    assert_eq!(formed[0].0, 4);
    assert_eq!(formed[0].1.raw(), 4);
    assert_eq!(
        g.base(4, formed[0].1.raw()).expect("formed").name,
        "Landing at (4,0)"
    );
    // Base 2 changes side at the end of day 4.
    assert_eq!(g.changed_side_day(2), Some(4));
    assert_eq!(g.base(3, 2).expect("base 2").side, Side::Japanese);
    assert_eq!(g.base(4, 2).expect("base 2").side, Side::Allied);
}

#[test]
fn the_second_run_sails_at_hour_192_and_lands_40_t_and_5_t() {
    let g = main_game();
    let arrivals = g.transport_arrivals();
    assert_eq!(arrivals.len(), 4, "{arrivals:?}");
    assert_eq!(arrivals[2], (B, n(208)));
    let landed = g.supply_landed();
    assert_eq!(landed.len(), 4, "{landed:?}");
    assert_eq!(landed[2], (9, Pulse::Morning, 1, n(40)));
    assert_eq!(landed[3], (9, Pulse::Afternoon, 1, n(5)));
    // It sails at hour 192, the night of day 9: the first steaming after
    // the force came home on day 6 is from A then.
    let steaming: Vec<(u32, Pulse, Hex)> = g.transport_steaming();
    let after_home = steaming.iter().find(|(day, _, _)| *day >= 7);
    assert_eq!(after_home, Some(&(9, Pulse::Night, A)), "{steaming:?}");
}

#[test]
fn the_supply_of_days_1_to_5() {
    let g = main_game();
    // Base 1 feeds 600 men, 3 t, on the nights of days 1 to 3, and loads out
    // 27 t on day 2.
    assert_eq!(g.supply(1, 1), n(997));
    assert_eq!(g.supply(2, 1), n(967));
    assert_eq!(g.supply(3, 1), n(964));
    // Base 3 pays 1.5 t each night.
    for day in 1..=5 {
        assert_eq!(
            g.supply(day, 3),
            fx("100") - fx("1.5") * Fixed::from_int(i64::from(day)),
            "day {day}"
        );
    }
    // Base 4 is given 10 t on the afternoon of day 4; on the night of day 5
    // base 2 holds nothing and base 4 pays 3 t: 7 t, which it keeps.
    assert_eq!(g.supply(4, 4), n(10));
    assert_eq!(g.supply(5, 4), n(7));
    // Base 2 takes 17 t on the night of day 5, after that night's charge.
    assert_eq!(g.supply(4, 2), Fixed::ZERO);
    assert_eq!(g.supply(5, 2), n(17));
}

#[test]
fn the_supply_of_days_6_to_14() {
    let g = main_game();
    // Base 2 pays 3 t on each night from day 6: 14, 11, 8 and 5 t after the
    // nights of days 6 to 9 before the run lands; takes 45 t on day 9: 50 t;
    // and pays 3 t on the nights of days 10 to 14: 35 t at the end.
    assert_eq!(g.supply(6, 2), n(14));
    assert_eq!(g.supply(7, 2), n(11));
    assert_eq!(g.supply(8, 2), n(8));
    assert_eq!(g.supply(9, 2), n(50));
    for day in 10..=14 {
        assert_eq!(
            g.supply(day, 2),
            n(50) - n(3) * Fixed::from_int(i64::from(day) - 9),
            "day {day}"
        );
    }
    assert_eq!(g.supply(14, 2), n(35));
    // Base 1: 1,000 - 9 - 72 = 919 t at the end.
    assert_eq!(g.supply(14, 1), n(919));
    // Base 4 keeps its 7 t.
    assert_eq!(g.supply(14, 4), n(7));
    // Base 3: 79 t at the end.
    assert_eq!(g.supply(14, 3), n(79));
}

#[test]
fn the_measure_to_day_8() {
    let g = main_game();
    // C counts on every day. B is the side's from the end of day 4; at the
    // end of day 4 its hex holds 10 t at 3 t a day, 3.33 days: it does not
    // count. At the end of days 5 to 8 it holds 24, 21, 18 and 15 t: 8, 7,
    // 6 and 5 days. The island is controlled on days 5 to 8.
    assert_eq!(g.controlled()[..9], [0, 0, 0, 0, 0, 1, 2, 3, 4]);
    let at = |day: usize| {
        g.measures[day]
            .as_ref()
            .expect("a measure")
            .places
            .iter()
            .map(|p| (p.ours, p.days_of_supply))
            .collect::<Vec<_>>()
    };
    assert_eq!(at(3)[0], (false, None));
    assert_eq!(at(4)[0], (true, Some(fx("3.333333"))));
    for (day, days) in [(5, 8), (6, 7), (7, 6), (8, 5)] {
        assert_eq!(at(day)[0], (true, Some(n(days))), "day {day}");
    }
    for day in 1..=8 {
        assert!(at(day)[1].0, "C is the side's");
    }
}

#[test]
fn the_island_is_controlled_on_10_of_14_days() {
    let g = main_game();
    // The island is controlled on days 5 to 14: at the end of day 9 B holds
    // 57 t, and 54 t down to 42 t at the end of days 10 to 14.
    let want: Vec<u32> = (0..=14).map(|d: u32| d.saturating_sub(4)).collect();
    assert_eq!(g.controlled(), want);
    let m = g.measures[14].as_ref().expect("a measure");
    assert_eq!((m.days_in_force, m.days_controlled), (14, 10));
    // The report's words.
    assert_says(
        g.words.last().expect("words"),
        &["controlled on 10 of 14 days in force"],
    );
}

// ---------------------------------------------------------------- The properties

/// The text with base 2 the side's, holding 100 t, and unit 1 ashore there.
fn both_the_sides() -> String {
    let t = changed(
        &text(),
        "side = \"japanese\"\nq = 4\nr = 0\nsupply_t = 0",
        "side = \"allied\"\nq = 4\nr = 0\nsupply_t = 100",
    );
    changed(
        &t,
        "name = \"Placeholder First\"\nside = \"allied\"\nmen = 600\nq = 0\nr = 0",
        "name = \"Placeholder First\"\nside = \"allied\"\nmen = 600\nq = 4\nr = 0",
    )
}

#[test]
fn both_places_the_sides_already() {
    let g = play(&both_the_sides());
    // No take and no neutralising is formed.
    assert!(g.formed(take_kind(None)).is_empty());
    assert!(g.formed(neutralise_kind(2)).is_empty());
    assert!(!g.forms().iter().any(|(_, _, k)| matches!(
        k,
        IntentKind::TakePlace { .. } | IntentKind::Neutralise { .. }
    )));
    // Both holds are formed and each forms its supply on day 1.
    assert_eq!(g.formed(hold_kind(2)), vec![(1, 1)]);
    assert_eq!(g.formed(hold_kind(3)), vec![(1, 1)]);
    assert_eq!(g.formed(keep_kind(2)).len(), 1);
    assert_eq!(g.formed(keep_kind(2))[0].0, 1);
    assert_eq!(g.formed(keep_kind(3)).len(), 1);
    assert_eq!(g.formed(keep_kind(3))[0].0, 1);
    // The island is controlled on 14 of 14 days.
    let m = g.measures[14].as_ref().expect("a measure");
    assert_eq!((m.days_in_force, m.days_controlled), (14, 14));
}

#[test]
fn a_commanders_own_hold_stands() {
    let g = play(&with(&command_table(
        1,
        "kind = \"set_intent\"\nintent = 2\nintent_kind = \"hold\"\nbase = 3",
    )));
    assert!(g.formed(hold_kind(3)).is_empty(), "{:?}", g.forms());
    // The staff still holds base 2.
    assert_eq!(g.formed(hold_kind(2)), vec![(1, 1)]);
    // Its reason names the commander's intent.
    assert_says(
        &g.reasons_serving(1, 1),
        &[
            "intent:2, the commander's intent to hold base:3",
            "so the staff forms none beside it",
        ],
    );
}

#[test]
fn the_commander_names_the_source() {
    let g = play(&with(&forces_limit("source = 1")));
    let m = main_game();
    assert_eq!(g.forms(), m.forms());
    for day in 1..=14 {
        assert!(
            same(&g.orders(day), &m.orders(day)),
            "day {day}: {:?} against {:?}",
            g.orders(day),
            m.orders(day)
        );
    }
}

#[test]
fn no_source_of_the_landing() {
    let g = play(&with(&hold_force(1, 1)));
    // Base 1 is the source of supply from day 1, with no force (decision
    // 0163, item 1).
    assert_eq!(g.staff_limit(1, 3), Some((vec![], vec![], Some(1))));
    assert_eq!(g.staff_limit(1, 4), Some((vec![], vec![], Some(1))));
    assert_says(
        &g.reasons_serving(1, 1),
        &[
            "the source is base:1 (Placeholder base A)",
            "with no transport force stopped there with nothing aboard",
            "to base:1 (Placeholder base A) and no force",
        ],
    );
    let all: String = (1..=14)
        .map(|d| g.reasons(d))
        .collect::<Vec<_>>()
        .join(" || ");
    assert_says(&all, &["is afloat, so there is nothing to send"]);
    // The take hands in the notice that it cannot meet its intent on day 1
    // and gives the reason each morning.
    let plan = g.intent_state(2).plan.raw();
    let notices: Vec<(u32, NoticeKind, u32)> = g
        .notices()
        .into_iter()
        .filter(|(_, _, p)| *p == plan)
        .collect();
    assert_eq!(notices, vec![(1, NoticeKind::CannotMeet, plan)]);
    for day in 1..=14 {
        let reasons = g.no_orders(day, 2).join(" || ");
        assert_says(
            &reasons,
            &["no transport force the staff may take lies stopped at a base of the side"],
        );
    }
    // The island is controlled on 0 of 14 days.
    let m = g.measures[14].as_ref().expect("a measure");
    assert_eq!((m.days_in_force, m.days_controlled), (14, 0));
    // "Keep base 3 supplied", floor 5 days, is formed under the hold of its
    // place, intent 4, on day 1, with no ship to send.
    assert_eq!(g.formed(keep_kind(3)), vec![(1, 4)], "{:?}", g.forms());
}

#[test]
fn an_enemy_base_near() {
    let base_5 = "[[base]]\nid = 5\nname = \"Placeholder enemy base 5\"\nside = \"japanese\"\nq = 2\nr = 3\nsupply_t = 0\nfuel_t = 0\n\n";
    let g = play(&before_commands(base_5));
    // "Neutralise base 2" and then "neutralise base 5" are formed on day 1,
    // after the denial.
    let day_1: Vec<IntentKind> = g
        .forms()
        .into_iter()
        .filter(|(d, p, _)| *d == 1 && *p == 1)
        .map(|(_, _, k)| k)
        .collect();
    let at = |k: &IntentKind| day_1.iter().position(|x| x == k);
    let (deny, n2, n5) = (
        at(&deny_kind()),
        at(&neutralise_kind(2)),
        at(&neutralise_kind(5)),
    );
    assert!(deny.is_some() && n2.is_some() && n5.is_some(), "{day_1:?}");
    assert!(deny < n2 && n2 < n5, "{day_1:?}");
}

/// The text with scenario 73's carrier force added: task force 2 on the
/// carrier mission, stopped at base 1, its home port, with ship 3 and the
/// three air groups as scenario 73 has them.
fn with_a_carrier() -> String {
    let carrier = "# Task force 2: one placeholder carrier, stopped in port at base 1, its home\n# port, as scenario 73 has it.\n[[task_force]]\nid = 2\nname = \"Placeholder carrier force 2\"\nside = \"allied\"\nat_base = 1\nmission = \"carrier\"\nhome_port = 1\n\n[[ship]]\nid = 3\nname = \"Placeholder carrier 3\"\nclass = 4\nside = \"allied\"\nfuel_t = 3000\ntask_force = 2\n\n[[air_group]]\nid = 1\nname = \"Placeholder fighters 1\"\nside = \"allied\"\nship = 3\naircraft_type = 4\nready = 12\n\n[[air_group]]\nid = 2\nname = \"Placeholder carrier scouts 2\"\nside = \"allied\"\nship = 3\naircraft_type = 2\nready = 6\n\n[[air_group]]\nid = 3\nname = \"Placeholder dive bombers 3\"\nside = \"allied\"\nship = 3\naircraft_type = 3\nready = 10\n\n";
    before_commands(carrier)
}

#[test]
fn a_carrier() {
    let g = play_carrier(&with_a_carrier());
    let m = main_game();
    // "Operate task force 2 from (0,0)" is formed under the denial on day 1.
    let carrier = IntentKind::OperateCarriers {
        force: TaskForceId::new(2),
        hex: A,
    };
    assert_eq!(g.formed(carrier), vec![(1, 5)]);
    // The take forms no carrier task.
    assert!(g
        .forms()
        .iter()
        .filter(|(_, p, _)| *p == 2)
        .all(|(_, _, k)| matches!(k, IntentKind::ReadyLanding { .. })));
    assert_eq!(
        g.forms()
            .iter()
            .filter(|(_, _, k)| matches!(k, IntentKind::OperateCarriers { .. }))
            .count(),
        1
    );
    // On the morning of day 2 group 3 is set to the base strike at base 2.
    let strike = AirMission::BaseStrike(BaseId::new(2));
    assert!(
        g.orders(2).contains(&Given::Mission(3, strike)),
        "{:?}",
        g.orders(2)
    );
    // It launches at base 2 on days 2, 3 and 4 and on no other day.
    let strikes = g.base_strikes(3);
    let days: Vec<u32> = strikes.iter().map(|(d, _)| *d).collect();
    let mut once: Vec<u32> = days.clone();
    once.dedup();
    assert_eq!(once, vec![2, 3, 4], "{strikes:?}");
    assert!(strikes.iter().all(|(_, b)| b.raw() == 2));
    // The force is given no route on any day.
    for (day, o) in g.all_orders() {
        if let Given::Route(2, ..) = o {
            panic!("day {day}: the carrier force is given a route");
        }
        if let Given::Other(text) = &o {
            assert!(
                !text.contains("SetRoute") || !text.starts_with("task force 2"),
                "{text}"
            );
        }
    }
    // The landing is the main case's: the orders to task force 1 and the
    // unit, days 1 to 5, and the hours.
    for day in 1..=5 {
        let landing: Vec<Given> = g
            .orders(day)
            .into_iter()
            .filter(|o| match o {
                Given::Mission(..) => false,
                Given::Other(text) => {
                    !text.starts_with("task force 2") && !text.contains("AirGroup")
                }
                Given::Landing(tf, ..)
                | Given::LoadTo(tf, _)
                | Given::LoadUnit(tf, _)
                | Given::Route(tf, ..)
                | Given::Unload(tf) => *tf == 1,
                _ => true,
            })
            .collect();
        assert!(
            same(&landing, &m.orders(day)),
            "day {day}: {landing:?} against {:?}",
            m.orders(day)
        );
    }
    assert_eq!(g.transport_arrivals()[..2], m.transport_arrivals()[..2]);
    assert_eq!(g.landed(), m.landed());
    assert_eq!(g.changed_side_day(2), Some(4));
}

/// The text with scenario 74's battleship force added as task force 3, on
/// the bombardment mission, stopped at base 1 with base 1 its home port: the
/// ship as scenario 74 has it, and its class file named. `extra` is added to
/// the task force's table.
fn with_a_bombardment_force(extra: &str) -> String {
    let force = format!("# Task force 3: scenario 74's placeholder battleship, stopped in port at\n# base 1, its home port, on the bombardment mission.\n[[task_force]]\nid = 3\nname = \"Placeholder bombardment force 3\"\nside = \"allied\"\nat_base = 1\nmission = \"bombardment\"\nstance = \"avoid\"\nhome_port = 1\n{extra}\n[[ship]]\nid = 4\nname = \"Placeholder battleship 4\"\nclass = 9\nside = \"allied\"\nfuel_t = 2000\ntask_force = 3\n\n");
    changed(
        &before_commands(&force),
        "    \"../reference/ship_classes_placeholder_transports.csv\",\n]",
        "    \"../reference/ship_classes_placeholder_transports.csv\",\n    \"../reference/ship_classes_placeholder_battleship.csv\",\n]",
    )
}

fn bombard_kind(force: u32, target: u32) -> IntentKind {
    IntentKind::Bombard {
        force: TaskForceId::new(force),
        target: BaseId::new(target),
    }
}

#[test]
fn a_bombardment_force() {
    let g = play(&with_a_bombardment_force(""));
    // "Bombard base 2 by task force 3" is formed on day 1 under the
    // neutralising of base 2, intent 6.
    assert_eq!(g.formed(neutralise_kind(2)), vec![(1, 1)]);
    assert_eq!(g.formed(bombard_kind(3, 2)), vec![(1, 6)]);
    // No bombard task is formed under the take, intent 2, whose limit names
    // no bombardment force.
    assert!(
        g.forms()
            .iter()
            .all(|(_, p, k)| *p != 2 || !matches!(k, IntentKind::Bombard { .. })),
        "{:?}",
        g.forms()
    );
    for (day, c) in g.all_commands() {
        if let Cmd::Limit(i, tfs, ..) = &c {
            assert!(
                !tfs.contains(&3),
                "day {day}: the limit of {i} names task force 3"
            );
        }
    }
    let task = g
        .formed_intents()
        .into_iter()
        .find(|(_, _, p)| *p == 6)
        .map(|(_, i, _)| i)
        .expect("the bombard task is formed");
    assert!(matches!(
        g.intent_state(task).kind,
        IntentKind::Bombard { .. }
    ));
    // On the morning of day 5, base 2 the side's, the neutralising is
    // withdrawn and the bombard task with it.
    assert!(
        g.commands(5).contains(&Cmd::Withdraw(6)),
        "{:?}",
        g.commands(5)
    );
    assert_eq!(g.intent_state(6).withdrawn_day, Some(5));
    assert_eq!(g.intent_state(task).withdrawn_day, Some(5));
    assert!(
        g.withdrawn_with().contains(&(5, task, 6)),
        "{:?}",
        g.withdrawn_with()
    );
    // Task force 3 is given no route on that morning or after.
    for day in 5..=g.days() {
        for o in g.orders(day) {
            let routed = match &o {
                Given::Route(3, ..) => true,
                Given::Other(text) => text.starts_with("task force 3") && text.contains("SetRoute"),
                _ => false,
            };
            assert!(!routed, "day {day}: task force 3 is given a route: {o:?}");
        }
    }
}

/// Not part of the hand calculation: the team's own test of decision 0151,
/// item 8c, the case of the proof, whose Japanese bombardment force may be
/// used only from a later day. With scenario 74's battleship force as in the
/// property "A bombardment force", may be used from day 3: it is taken on
/// the morning of day 3, its first, and not before.
#[test]
fn a_bombardment_force_used_from_a_later_day_is_taken_on_its_first_morning() {
    let g = play(&with_a_bombardment_force("available_from = 3\n"));
    assert_eq!(g.formed(bombard_kind(3, 2)), vec![(3, 6)]);
}

#[test]
fn by_a_date() {
    let g = play(&changed(
        &text(),
        "intent_kind = \"control_island\"\nisland = 1\n",
        "intent_kind = \"control_island\"\nisland = 1\nby_day = 10\n",
    ));
    // The take is formed with the date of day 10 and the task "ready a
    // landing" under it has that day.
    assert_eq!(g.formed(take_kind(Some(10))), vec![(1, 1)]);
    assert!(g.formed(take_kind(None)).is_empty());
}

#[test]
fn by_a_date_the_landing_task_has_day_8() {
    let g = play(&changed(
        &text(),
        "intent_kind = \"control_island\"\nisland = 1\n",
        "intent_kind = \"control_island\"\nisland = 1\nby_day = 10\n",
    ));
    assert_eq!(g.formed(landing_kind(1, Some(8))), vec![(1, 2)]);
}

#[test]
fn without_leave_to_launch() {
    let g = play(&changed(
        &text(),
        "leave = \"launch\"\ngiven = true",
        "leave = \"launch\"\ngiven = false",
    ));
    // The plan of intent 1 is approved to preparing and not released;
    // nothing is formed.
    assert_eq!(g.plan_day(1, PlanState::Preparing), Some(1));
    assert_eq!(g.plan_day(1, PlanState::Released), None);
    assert!(g.forms().is_empty(), "{:?}", g.forms());
    // The notice that it waits for leave to launch is handed in each
    // morning.
    for day in 1..=14 {
        assert!(
            g.handed[&day].notices.iter().any(|nt| nt.plan.raw() == 1
                && nt.kind == NoticeKind::WaitsForLeave(sim_core::command::Leave::Launch)),
            "day {day}: {:?}",
            g.notices()
        );
    }
}

#[test]
fn withdrawn() {
    let g = play(&with(&command_table(
        6,
        "kind = \"withdraw_intent\"\nintent = 1",
    )));
    // Every intent under intent 1 is withdrawn with it on day 6; intent 6
    // was withdrawn by the staff on day 5.
    assert_eq!(g.intent_state(1).withdrawn_day, Some(6));
    for raw in [2, 3, 4, 5, 7, 8, 9] {
        assert_eq!(g.intent_state(raw).withdrawn_day, Some(6), "intent {raw}");
    }
    assert_eq!(g.intent_state(6).withdrawn_day, Some(5));
    let mut with_it: Vec<u32> = g.withdrawn_with().iter().map(|(_, i, _)| *i).collect();
    with_it.sort();
    assert!(with_it.iter().all(|i| *i != 6), "{with_it:?}");
    assert_eq!(with_it, vec![2, 3, 4, 5, 7, 8, 9]);
    assert!(g.withdrawn_with().iter().all(|(d, ..)| *d == 6));
    // No order is given from day 6.
    for day in 6..=14 {
        assert!(g.orders(day).is_empty(), "day {day}: {:?}", g.orders(day));
    }
}

#[test]
fn the_same_start_gives_the_same_orders() {
    let a = play(&text());
    assert_eq!(a.handed, main_game().handed);
    assert_eq!(a.hashes, main_game().hashes);
}

#[test]
fn what_the_side_cannot_see_changes_nothing() {
    let hidden = |supply: u32| {
        format!(
            "[[base]]\nid = 9\nname = \"Placeholder base 9\"\nside = \"japanese\"\nq = 3\nr = 4\nsupply_t = {supply}\non_map = false\n\n"
        )
    };
    let a = play(&before_commands(&hidden(50)));
    let b = play(&before_commands(&hidden(400)));
    assert_ne!(
        a.truth.last(),
        b.truth.last(),
        "the two worlds do not differ"
    );
    assert_eq!(a.handed, b.handed);
    for day in 1..=14 {
        assert_eq!(a.orders(day), main_game().orders(day), "day {day}");
    }
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
fn saved_on_day_7_and_resumed() {
    let through = main_game();
    let at_7 = rebuilt(&through.handed, 7);
    assert_eq!(at_7.world.day_hashes(), &through.hashes[..7]);
    let first: BTreeMap<u32, StaffDecisions> =
        (1..=7).map(|d| (d, through.handed[&d].clone())).collect();
    let resumed = play_from(at_7, first);
    assert_eq!(resumed.hashes, through.hashes);
    assert_eq!(resumed.handed, through.handed);
}

// "Every scenario built before this one gives the events it gave" is checked
// elsewhere: the golden files and the two run-baseline tests, which leave this
// scenario out because its staff gives orders.

/// Not part of the hand calculation: the architect's ruling on decision
/// 0151, item 4 (Nov 17, 2026). Under "control an island" a forces limit
/// that names no air group does not bind the air groups: the hold of base 3,
/// whose forces limit the staff sets to task force 1 and base 1, still covers
/// its base with the fighter group based there, as decision 0117's doctrine
/// does with no limit. Placeholder data: scenario 73's fighter type, one
/// group of 12 at base 3.
#[test]
fn a_hold_under_the_island_covers_its_base_with_the_groups_there() {
    let fighters = "# Air group 1: placeholder fighters based at base 3.\n[[air_group]]\nid = 1\nname = \"Placeholder fighters 1\"\nside = \"allied\"\nbase = 3\naircraft_type = 4\nready = 12\n\n";
    let g = play_carrier(&before_commands(fighters));
    // The hold of base 3 is intent 4, its limit task force 1 and base 1.
    let cover = IntentKind::Cover {
        base: BaseId::new(3),
    };
    assert_eq!(g.formed(cover), vec![(1, 4)]);
    // The fighter group is given its patrol on day 1.
    assert!(
        g.orders(1)
            .contains(&Given::Mission(1, AirMission::CombatAirPatrol)),
        "{:?}",
        g.orders(1)
    );
}

/// Decision 0217, statement 8 (the architect's): in the carrier variant the
/// deny the staff forms under the order to control the island has a carrier
/// task under it, and on no day of the 14 is the notice that it cannot be
/// met handed in for that deny; its reason for the bases names the carrier
/// force first and has no "cannot meet". Every order of every day is what
/// it was (the orders and commands of the 14 days were dumped before the
/// change and after it and compared: no line differs).
#[test]
fn decision_0217_statement_8_the_islands_deny_is_met_by_its_carrier_task() {
    let g = play_carrier(&with_a_carrier());
    // The deny is intent 5; the carrier task under it is intent 6.
    let carrier = IntentKind::OperateCarriers {
        force: TaskForceId::new(2),
        hex: A,
    };
    assert_eq!(g.formed(carrier), vec![(1, 5)]);
    assert_eq!(g.days(), 14);
    for day in 1..=14 {
        let handed = &g.handed[&day];
        let about_the_deny = |r: &Reason| r.worked.contains("No base intent:5 may use");
        // No notice that the order cannot be met rests on a reason of the
        // deny.
        for notice in &handed.notices {
            if notice.kind != sim_core::command::NoticeKind::CannotMeet {
                continue;
            }
            if let sim_core::command::NoticeBasis::NoOrder(i) = notice.rests_on {
                let reason = &handed.no_orders[i];
                assert!(
                    !reason.worked.contains("intent:5"),
                    "day {day}: the notice that intent 5 cannot be met: {}",
                    reason.worked
                );
            }
        }
        // The deny's reason for the bases names the carrier force first.
        let reasons: Vec<&Reason> = handed
            .no_orders
            .iter()
            .filter(|r| about_the_deny(r))
            .collect();
        assert_eq!(reasons.len(), 1, "day {day}");
        let worked = &reasons[0].worked;
        assert!(
            worked.starts_with(
                "intent:6, to operate the carrier force task_force:2 from (0,0), serves intent:5."
            ),
            "day {day}: {worked}"
        );
        assert!(
            !worked.to_lowercase().contains("cannot meet"),
            "day {day}: {worked}"
        );
        assert!(
            worked.find("task_force:2") < worked.find("base:1"),
            "day {day}: {worked}"
        );
    }
    // The island's neutralise, intent 7, is not in the ruling: it still
    // says it cannot be met, on the days it had.
    assert!(g.handed[&1].notices.iter().any(|n| {
        n.kind == sim_core::command::NoticeKind::CannotMeet
            && matches!(n.rests_on, sim_core::command::NoticeBasis::NoOrder(i)
                if g.handed[&1].no_orders[i].worked.contains("intent:7"))
    }));
}
