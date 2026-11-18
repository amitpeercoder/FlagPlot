//! Scenario 77: forces with men aboard. The commander says "control the
//! island". Its one place is an empty enemy airfield. The side's men are
//! at sea in their transports, as on 7 August 1942. The staff lands
//! them, takes the airfield, sends the builders who become usable on day
//! 4, and feeds the place with the ships that lie empty at the source.
//!
//! Proves decision 0152 against a hand calculation. It stands on
//! scenarios 67, 69 and 76. The data is placeholder data: see
//! `data/scenarios/77_forces_with_men_aboard.toml`. It runs on the frozen
//! step 1 figures. No enemy man is ashore anywhere, so no die is rolled.
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
//! A man weighs 0.25 t to land and eats 0.005 t a day, paid at the start
//! of the night pulse by the side's bases in the hex in base order.
//! Supply landed goes into the side's lowest-numbered base in the hex. A
//! size 0 port works by lighter at 20 t an hour for each ship, and on the
//! frozen figures a landing is not slowed by its loading. Day d begins at
//! hour 24 x (d - 1): a night pulse of 12 hours and two daylight pulses
//! of 6. In a pulse ships move before cargo is worked. A force that has
//! landed its load steams on at the start of the next pulse. A hex is
//! 40 nm and the transports make 10 kn.
//!
//! A working unit ashore at a base with nothing else to do works on its
//! port (decision 0099): each man gives 1 of work a day, and the rate of
//! unloading there is 1 + work / 1,000 times the lighter's 20 t an hour.
//! Unit 2, 400 men, works from day 2: 1.4, 28 t an hour. Unit 3, 300 men,
//! works from day 5: 1.7, 34 t an hour. (Restated by the architect on Nov
//! 18, 2026: the first writing left the work out and had 20 t an hour on
//! days 4 and 5.)
//!
//! ## Inputs
//!
//! The placeholder map of scenario 27: A at (0,0), port size 5, and B at
//! (4,0), port size 0. The game is 8 days.
//!
//! Base 1, Allied, at A, with 1,000 t of supply. Base 2, Japanese, at B,
//! on the Allied map, with no gun, aircraft, supply or man. No estimate
//! and no contact is held. Island 1, "Placeholder island": base 2.
//!
//! | Task force | Ships | Where | Home port | Aboard |
//! | --- | --- | --- | --- | --- |
//! | 1 | 1 and 2, placeholder troop transports | at sea at (2,0), stopped | base 1 | unit 1 in ship 1 with 30 t of supply; unit 2 in ship 2 |
//! | 2, usable from day 4 | 3, a placeholder troop transport | in port at base 1 | base 1 | unit 3, with 20 t of supply |
//! | 3 | 4, a placeholder troop transport | stopped at base 1 | base 1 | nothing |
//!
//! All are Allied and on the transport mission.
//!
//! | Unit | Kind | Men | Posture |
//! | ---- | ---- | --- | ------- |
//! | 1, "Placeholder Infantry" | fighting | 600 | defend |
//! | 2, "Placeholder Pioneers" | working | 400 | |
//! | 3, "Placeholder Builders" | working | 300 | |
//!
//! The Allied side's commands of day 1: intent 1, control island 1. No
//! forces limit and no risk, so balanced. Leave to prepare and leave to
//! launch are both given.
//!
//! ## Day by day
//!
//! | Day | Hour | The staff's orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 | 0 | Task force 1: `prepare_landing` at B, roster 1; a route to B and back to A, acting at B; an unload order | Sails at hour 0 and arrives at hour 8. In the night's last 4 hours each ship lands 320 men. In the morning ship 1 lands 280 men in 3.5 hours and then 30 t in 1.5 hours; ship 2 lands 80 men in an hour. Base 3, "Landing at (4,0)", is formed. The force steams for A at hour 18 |
//! | 2 | 24 | Unit 1 set to attack | The force arrives at A at hour 34. Base 2 changes side at the end of the day |
//! | 3 | 48 | Task force 3: a load of supply to a weight of 80 t. Unit 1 set to defend | Ship 4 loads 80 t at A |
//! | 4 | 72 | Task force 3: a route to B and back to A, acting at B, and an unload order. Task force 2: the same | Both sail at hour 72 and arrive at hour 88. With unit 2 at work on the port the rate is 28 t an hour for each ship. Ship 4 lands 56 t in the morning's 2 hours and 24 t in the afternoon. Ship 3 lands 224 men in the morning, and in the afternoon 76 men and then 20 t. Both steam for A at hour 96 |
//! | 5 | 96 | none | Task forces 2 and 3 arrive at A at hour 112 |
//! | 6 to 8 | | none | |
//!
//! Ten orders in all. No staff order or command is refused.
//!
//! - **Day 1.** The tree: "take the place at base 2", "hold base 2",
//!   "deny the waters within 3 hexes of (4,0)" and "neutralise base 2"
//!   under intent 1, in that order. The neutralising finds no bomber and
//!   no bombardment force, and the staff withdraws it on the morning of
//!   day 3, base 2 the side's. The take's first lift is task force 1, a lift as it stands:
//!   its fighting unit is unit 1, and unit 2 lands with it. The force
//!   does not lie at a base: its landing is ready as it stands, and it
//!   goes with the 30 t aboard. The plan of the take is released that
//!   morning and the force routed. The source of supply is base 1, where
//!   task force 3 lies empty: the hold's forces limit is base 1 and task
//!   force 3.
//! - **Day 2.** No estimate is held: the judgement is to attack, and unit
//!   1 is ashore and not on attack. The hold of base 2 still waits, and
//!   the side has base 3 in the hex with 1,000 men ashore: "keep base 3
//!   supplied", with a floor of 5 days, is formed under the hold
//!   (decision 0159, rule A). At the end of the day the Japanese side
//!   has a base and no man at B, and base 2 is Allied.
//! - **Day 3.** The take is met: its plan is marked complete. The hold of
//!   base 2 is no longer waiting: unit 1 is set to defend. No "keep base
//!   2 supplied" is formed: one is in force for the hex. Task force 1
//!   lies empty at base 1 beside task force 3: the hold's forces limit
//!   is set to base 1 and task forces 1 and 3. The supply intent is
//!   served for the first time, on the limit as it stood that morning,
//!   base 1 and task force 3: the stock at B is 25 t, the use is 1,000 x
//!   0.005 = 5 t a day, and the floor's stock is 25 t. A run ordered
//!   today arrives on day 4, and the reading of day 5 is 25 - 10 = 15 t,
//!   below 25: a run. Its load is the floor's stock on the day it
//!   arrives, 25 t, less that day's reading, 20 t, plus 15 days of use,
//!   75 t: 80 t. Ship 4's room covers it: task force 3 takes the run.
//! - **Day 4.** Task force 2 may be used. It has men aboard, all of a
//!   working unit, and no take is in force: the hold of base 2 sends it.
//!   No second run is called then or after: with the run due that day
//!   the hex holds 115 t at the end of day 4 against a use of 6.5 t.
//!
//! ## Supply
//!
//! - **Base 3** takes 30 t on the morning of day 1. It pays 5 t for 1,000
//!   men on the nights of days 2, 3 and 4: base 2, Allied from the end
//!   of day 2 and the lower numbered, holds nothing on those nights. It
//!   ends with 15 t.
//! - **Base 2** takes 80 t and 20 t on day 4: 100 t. It pays 6.5 t for
//!   1,300 men on the nights of days 5, 6, 7 and 8: 74 t at the end.
//! - **Base 1** loads out 80 t and feeds nobody: 920 t at the end.
//! - **Fuel.** Ships 1 and 2 steam 80 nm to B and 160 nm home, and burn
//!   24 t: 976 t at the end. Ships 3 and 4 steam 320 nm and burn 32 t:
//!   968 t.
//!
//! ## The measure
//!
//! A place that names a base counts on a day when that base was the
//! side's at the end of the day and the stock in its hex was at least
//! the floor, 5 days, of use. Base 2 is the side's from the end of day
//! 2; on day 1 the side has only base 3, formed by the landing, in the
//! hex, and the place is not its own. The measure keeps no record of
//! past days' use: it reads every past day at today's use (decision
//! 0151, ruling 5).
//!
//! | Read at the end of day | Use, t a day | Days that count | Controlled |
//! | --- | --- | --- | --- |
//! | 1 | 5 | none | 0 |
//! | 2 | 5 | 2 (25 t, 5 days) | 1 |
//! | 3 | 5 | 2 (day 3 has 20 t, 4 days) | 1 |
//! | 4 | 6.5 | 4 (115 t; days 2 and 3 hold less than 32.5 t) | 1 |
//! | 5 to 8 | 6.5 | 4 and each day since | 2, 3, 4, 5 |
//!
//! At the end of the game the island is controlled on 5 of 8 days in
//! force. The days of supply shown for the place are 6, 5 and 4 at the
//! end of days 1 to 3.
//!
//! Restated by the architect on Nov 18, 2026, for decision 0159's rule A
//! (doctrine supply-35): the men landed on day 1 are fed through base 3
//! from day 2, before the airfield is taken, so the supply intent is
//! formed a day sooner and for base 3, the run is called on day 3 and
//! not day 4, it is 80 t and not 85 t, and task force 3 carries it and
//! not task force 1. The orders are still ten. The measure's counts are
//! as they were.
//!
//! Restated by the architect on Nov 18, 2026, on reading the build: the
//! first writing counted day 2 to the end and gave 6 of 8, which a
//! measure that keeps no record cannot give; and the build counted day
//! 1, on which the airfield was the enemy's, which is mended (a place
//! that names a base is the side's when that base is).
//!
//! # Properties
//!
//! - **A later lift of fighting men.** With unit 3 a fighting unit of
//!   300 men, and unit 10, Japanese, 2,000 men, ashore at B: the take is
//!   not met on day 4, and no estimate or any estimate leaves the men
//!   wanted above the men ashore. On the morning of day 4 the take sends
//!   task force 2, a later lift as it stands, with a route to B and back
//!   and an unload order, unit 3 is added to the forces limit of the
//!   task "ready a landing", and unit 3 is given no posture order before
//!   it sails. Only the orders of days 1 and 4 are asserted: the fights
//!   are not this scenario's.
//! - **Two islands.** With a second island, "Placeholder island 2", of a
//!   Japanese base 4 at C, (0,6), on the Allied map with nothing at it,
//!   intent 2, control island 2, stated on day 1 after intent 1, unit 3
//!   a fighting unit of 300 men, and task force 2 usable from day 1: on the morning
//!   of day 1 the take of base 2 takes task force 1 as its first lift
//!   and sends no other, its first lift being on its way; the take of
//!   base 4 takes task force 2 as its first lift and routes it to C. No
//!   force but task force 1 is routed to B on day 1. Only the orders of
//!   day 1 are asserted.
//! - **The higher take first.** As "Two islands", but with task force 2
//!   usable from day 4 as in the main case, and unit 10, Japanese, 2,000
//!   men, ashore at B. On day 1 the take of base 2 takes task force 1 as
//!   its first lift; the take of base 4 has no lift it may take and
//!   forms nothing. On the morning of day 4 task force 2 may be used.
//!   The take of base 2 is not met, its first lift landed on day 1, and
//!   its 600 fit men are fewer than the men wanted whatever the
//!   estimate: it is the higher in priority and sends task force 2 as a
//!   later lift, with a route to B and back and an unload order, and
//!   unit 3 is added to the forces limit of its task "ready a landing".
//!   The take of base 4 forms nothing that morning and task force 2 is
//!   given no route to C on any day. Only the orders of days 1 and 4 are
//!   asserted. Added by the architect on Nov 18, 2026, with decision
//!   0152's item 5b.
//! - **Nothing aboard that can fight.** With unit 1 a working unit: no
//!   lift as it stands has a fighting unit, and no base has a transport
//!   force beside a fighting unit ashore. The take forms nothing, hands
//!   in the notice that it cannot meet its intent on day 1 and gives the
//!   reason each morning. No force is sent: the hold's base is not the
//!   side's.
//! - **A force that waits for its day.** Task force 2 is given no order
//!   before day 4 in any case, and each morning's reason names its day.
//! - **Held.** With task force 2 held from day 1: it is given no order,
//!   and the reason of day 4 names the hold.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see give the same orders and reasons; a
//!   save on day 3 and a resume give the same game; scenarios 67 to 76
//!   give the orders they gave.

// ---------------------------------------------------------------- The harness
//
// Modelled on scenario 76's. Everything above this line is the architect's
// hand calculation; nothing below it states a number the hand calculation
// does not.

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
use sim_core::orders::{Cargo, LandUnitOrder, OrderKind, Posture, TaskForceOrder};
use sim_core::state::{Side, TruthState};
use sim_core::time::Pulse;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

mod common;

const COMMANDS_MARKER: &str = "# The Allied commander's commands";
const ISLANDS_MARKER: &str = "# ISLANDS_MARKER\n";
const A: Hex = Hex { q: 0, r: 0 };
const B: Hex = Hex { q: 4, r: 0 };
const C: Hex = Hex { q: 0, r: 6 };

fn path() -> PathBuf {
    common::data_dir().join("scenarios/77_forces_with_men_aboard.toml")
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
/// `data/reference/ground_placeholder_scenario_77.toml`, frozen beside the
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
    Route(u32, Vec<Hex>, Vec<Hex>),
    Unload(u32),
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
            order:
                LandUnitOrder::SetPosture {
                    posture,
                    when_spent: None,
                },
        } => match posture {
            Posture::Attack => Given::Attack(target.raw()),
            Posture::Defend => Given::Defend(target.raw()),
        },
        other => Given::Other(format!("{other:?}")),
    }
}

/// The task force an order is for, where it is a task force's.
fn force_of(g: &Given) -> Option<u32> {
    match g {
        Given::Landing(tf, ..)
        | Given::LoadTo(tf, _)
        | Given::Route(tf, ..)
        | Given::Unload(tf) => Some(*tf),
        Given::Other(text) => text
            .strip_prefix("task force ")
            .and_then(|rest| rest.split(':').next())
            .and_then(|raw| raw.parse().ok()),
        _ => None,
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

/// The route to B and back to A, acting at B.
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

fn take_kind_at(hex: Hex) -> IntentKind {
    IntentKind::TakePlace { hex, by_day: None }
}

fn take_kind() -> IntentKind {
    take_kind_at(B)
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

fn landing_kind_at(force: u32, hex: Hex) -> IntentKind {
    IntentKind::ReadyLanding {
        force: TaskForceId::new(force),
        hex,
        by_day: None,
    }
}

fn landing_kind(force: u32) -> IntentKind {
    landing_kind_at(force, B)
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

    #[allow(dead_code)]
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

    fn intent_state(&self, raw: u32) -> &sim_core::command::Intent {
        &self.state.intents[&intent(raw)]
    }

    /// The passages of task force `tf`: where it arrived or halted, and the
    /// hour.
    fn arrivals_of(&self, tf: u32) -> Vec<(Hex, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceArrived {
                    task_force,
                    hex,
                    elapsed_hours,
                } if task_force.raw() == tf => Some((*hex, *elapsed_hours)),
                EventKind::TaskForceHalted {
                    task_force,
                    hex,
                    elapsed_hours,
                    ..
                } if task_force.raw() == tf => Some((*hex, *elapsed_hours)),
                _ => None,
            })
            .collect()
    }

    fn steaming_of(&self, tf: u32) -> Vec<(u32, Pulse, Hex)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceSteamed {
                    task_force, from, ..
                } if task_force.raw() == tf => Some((e.day, e.pulse?, *from)),
                _ => None,
            })
            .collect()
    }

    /// Every landing of men as text "day pulse unit u ship s men m", sorted.
    fn landed(&self) -> Vec<String> {
        let mut all: Vec<String> = self
            .events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::MenLanded {
                    unit, ship, men, ..
                } => Some(format!(
                    "{} {:?} unit {} ship {} men {}",
                    e.day,
                    e.pulse?,
                    unit.raw(),
                    ship.raw(),
                    men
                )),
                _ => None,
            })
            .collect();
        all.sort();
        all
    }

    /// Every landing of supply as text "day pulse ship s t tons", sorted.
    fn supply_landed(&self) -> Vec<String> {
        let mut all: Vec<String> = self
            .events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::CargoUnloaded {
                    ship,
                    cargo: Cargo::Supply,
                    unloaded_t,
                    ..
                } => Some(format!(
                    "{} {:?} ship {} t {}",
                    e.day,
                    e.pulse?,
                    ship.raw(),
                    unloaded_t
                )),
                _ => None,
            })
            .collect();
        all.sort();
        all
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

    /// The orders of days `from` to `to` that are for task force `tf`.
    fn orders_for_force(&self, tf: u32, from: u32, to: u32) -> Vec<(u32, Given)> {
        (from..=to)
            .flat_map(|d| self.orders(d).into_iter().map(move |g| (d, g)))
            .filter(|(_, g)| force_of(g) == Some(tf))
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

/// The text with `extra` tables added before the commands.
fn before_commands(extra: &str) -> String {
    changed(
        &text(),
        COMMANDS_MARKER,
        &format!("{extra}{COMMANDS_MARKER}"),
    )
}

/// Unit 3 a fighting unit of 300 men.
fn unit_3_fights(text: &str) -> String {
    changed(
        text,
        "name = \"Placeholder Builders\"\nside = \"allied\"\nworking = true\nmen = 300",
        "name = \"Placeholder Builders\"\nside = \"allied\"\nmen = 300",
    )
}

/// Task force 2 usable from day 1.
fn force_2_from_day_1(text: &str) -> String {
    changed(text, "available_from = 4\n", "")
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
    assert_eq!(t.matches(ISLANDS_MARKER).count(), 1);
    assert!(
        !t.contains("limit = \"forces\""),
        "no forces limit is named"
    );
    assert!(t.contains("days = 8"));
    assert!(t.contains("intent_kind = \"control_island\"\nisland = 1\n"));
    assert!(!t.contains("by_day"));
    assert_eq!(t.matches("leave = \"launch\"\ngiven = true").count(), 1);
    assert_eq!(t.matches("available_from = 4\n").count(), 1);
    assert_eq!(t.matches("working = true").count(), 2);
    // No order of any kind, set-activity included, is in the file.
    assert!(!t.contains("[[order]]"));
}

#[test]
fn the_tree_of_the_first_morning() {
    let g = main_game();
    assert_eq!(g.days(), 8);
    // Intents 2 to 5 are the first four formed, in that order, then the
    // landing under the take (intent 6); all on day 1.
    assert_eq!(
        g.formed_intents()
            .into_iter()
            .filter(|(d, ..)| *d == 1)
            .collect::<Vec<_>>(),
        vec![(1, 2, 1), (1, 3, 1), (1, 4, 1), (1, 5, 1), (1, 6, 2)]
    );
    let expect: [(u32, IntentKind, u32); 5] = [
        (2, take_kind(), 1),
        (3, hold_kind(2), 1),
        (4, deny_kind(), 1),
        (5, neutralise_kind(2), 1),
        (6, landing_kind(1), 2),
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
    // The neutralising finds no bomber and no bombardment force and forms
    // nothing under it; nor does the denial.
    assert!(!g
        .formed_intents()
        .iter()
        .any(|(_, _, p)| *p == 4 || *p == 5));
    // The side has no carrier force: no carrier task is formed.
    assert!(!g
        .forms()
        .iter()
        .any(|(_, _, k)| matches!(k, IntentKind::OperateCarriers { .. })));
}

#[test]
fn the_first_lift_of_the_take_is_task_force_1() {
    let g = main_game();
    assert_eq!(g.formed(landing_kind(1)), vec![(1, 2)]);
    // The landing task's forces limit names task force 1 and the fighting
    // unit aboard it, unit 1 (the hand calculation gives no more than that).
    let (tfs, units, _) = g.staff_limit(1, 6).expect("a limit on day 1");
    assert_eq!(tfs, vec![1]);
    assert!(units.contains(&1), "{units:?}");
    // The take has a plan of its own, and the landing task is in it.
    let plan_of_one = g.intent_state(1).plan.raw();
    let plan = g.intent_state(2).plan.raw();
    assert_ne!(plan, plan_of_one);
    assert_eq!(g.intent_state(6).plan.raw(), plan);
    // It is released that morning and complete on day 3.
    assert_eq!(g.plan_day(plan, PlanState::Released), Some(1));
    assert_eq!(g.plan_day(plan, PlanState::Complete), Some(3));
}

#[test]
fn the_holds_forces_limit_is_base_1_and_task_force_3_then_task_forces_1_and_3() {
    let g = main_game();
    // Day 1: the source of supply is base 1, where task force 3 lies empty.
    assert_eq!(g.staff_limit(1, 3), Some((vec![3], vec![], Some(1))));
    // Day 3: task force 1 lies empty at base 1 beside task force 3.
    assert_eq!(g.staff_limit(3, 3), Some((vec![1, 3], vec![], Some(1))));
    // Task force 2 is in the limit on no day before day 4.
    for day in 1..=3 {
        if let Some((tfs, ..)) = g.staff_limit(day, 3) {
            assert!(!tfs.contains(&2), "day {day}: {tfs:?}");
        }
    }
}

#[test]
fn the_neutralising_is_withdrawn_on_day_3_by_the_staff() {
    let g = main_game();
    assert!(
        g.commands(3).contains(&Cmd::Withdraw(5)),
        "{:?}",
        g.commands(3)
    );
    assert_eq!(g.intent_state(5).withdrawn_day, Some(3));
    for raw in [1, 2, 3, 4, 6, 7] {
        assert_eq!(g.intent_state(raw).withdrawn_day, None, "intent {raw}");
    }
}

#[test]
fn keep_base_3_supplied_with_floor_5_is_formed_on_day_2_under_the_hold() {
    let g = main_game();
    // Decision 0159, rule A: the hold of base 2 still waits on day 2, and the
    // side has base 3 in the hex with its men ashore.
    assert_eq!(g.formed(keep_kind(3)), vec![(2, 3)]);
    assert_eq!(g.formed_intents().last(), Some(&(2, 7, 3)));
    assert_eq!(g.intent_state(7).kind, keep_kind(3));
    // None is formed for base 2 on day 3, or on any day: one is in force
    // for the hex.
    assert!(g.formed(keep_kind(2)).is_empty(), "{:?}", g.forms());
    // It is served for the first time on day 3: nothing serves it on day 2.
    assert!(g.reasons_serving(2, 7).is_empty(), "{}", g.reasons_serving(2, 7));
    assert_says(
        &g.reasons_serving(3, 7),
        &["25 t over 5 t a day", "15 t"],
    );
}

#[test]
fn the_orders_of_days_1_to_8_ten_in_all_none_refused() {
    let g = main_game();
    let wanted: Vec<(u32, Vec<Given>)> = vec![
        (
            1,
            vec![Given::Landing(1, B, vec![1]), route(1), Given::Unload(1)],
        ),
        (2, vec![Given::Attack(1)]),
        (3, vec![Given::LoadTo(3, n(80)), Given::Defend(1)]),
        (
            4,
            vec![route(3), Given::Unload(3), route(2), Given::Unload(2)],
        ),
        (5, vec![]),
        (6, vec![]),
        (7, vec![]),
        (8, vec![]),
    ];
    for (day, want) in wanted {
        assert!(
            same(&g.orders(day), &want),
            "day {day}: {:?} against {want:?}",
            g.orders(day)
        );
    }
    // Ten in all, none refused, and no set-activity order.
    assert_eq!(g.all_orders().len(), 10);
    assert!(!g
        .all_orders()
        .iter()
        .any(|(_, o)| matches!(o, Given::Other(_))));
    g.nothing_refused();
}

#[test]
fn the_hours_of_the_landing_and_of_task_forces_2_and_3() {
    let g = main_game();
    // Task force 1 sails at hour 0 and arrives at B at hour 8, steams for A
    // at hour 18 (the afternoon of day 1) and arrives at A at hour 34. It
    // carries no run.
    assert_eq!(g.arrivals_of(1), vec![(B, n(8)), (A, n(34))]);
    let steaming = g.steaming_of(1);
    assert!(
        steaming.contains(&(1, Pulse::Afternoon, B)),
        "{steaming:?}"
    );
    // Task forces 2 and 3 sail at hour 72, arrive at B at hour 88, steam for
    // A at hour 96 and arrive at A at hour 112.
    for tf in [2, 3] {
        assert_eq!(g.arrivals_of(tf), vec![(B, n(88)), (A, n(112))], "{tf}");
        let steaming_tf = g.steaming_of(tf);
        for want in [(4, Pulse::Night, A), (5, Pulse::Night, B)] {
            assert!(
                steaming_tf.contains(&want),
                "task force {tf}: {want:?} in {steaming_tf:?}"
            );
        }
    }
}

#[test]
fn the_men_and_supply_landed() {
    let g = main_game();
    // Day 1: in the night's last 4 hours each ship lands 320 men; in the
    // morning ship 1 lands 280 and ship 2 lands 80. Day 4: task force 2's
    // ship lands 224 men in the morning and 76 in the afternoon, at 28 t an hour.
    let mut want: Vec<String> = [
        (1, "Night", 1, 1, 320),
        (1, "Night", 2, 2, 320),
        (1, "Morning", 1, 1, 280),
        (1, "Morning", 2, 2, 80),
        (4, "Morning", 3, 3, 224),
        (4, "Afternoon", 3, 3, 76),
    ]
    .iter()
    .map(|(d, p, u, s, m)| format!("{d} {p} unit {u} ship {s} men {m}"))
    .collect();
    want.sort();
    assert_eq!(g.landed(), want);
    // Supply: 30 t in the morning of day 1; on day 4, at 28 t an hour for
    // each ship, ship 4 lands 56 t in the morning and 24 t in the afternoon,
    // and ship 3 lands 20 t in the afternoon.
    let mut want_t: Vec<String> = [
        (1, "Morning", 1, 30),
        (4, "Afternoon", 3, 20),
        (4, "Morning", 4, 56),
        (4, "Afternoon", 4, 24),
    ]
    .iter()
    .map(|(d, p, s, t)| format!("{d} {p} ship {s} t {}", n(*t)))
    .collect();
    want_t.sort();
    assert_eq!(g.supply_landed(), want_t);
}

#[test]
fn base_3_landing_at_4_0_is_formed_and_base_2_changes_side_at_the_end_of_day_2() {
    let g = main_game();
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
    assert_eq!(formed[0].0, 1);
    assert_eq!(formed[0].1.raw(), 3);
    assert_eq!(g.base(1, 3).expect("formed").name, "Landing at (4,0)");
    assert_eq!(g.changed_side_day(2), Some(2));
    assert_eq!(g.base(1, 2).expect("base 2").side, Side::Japanese);
    assert_eq!(g.base(2, 2).expect("base 2").side, Side::Allied);
}

#[test]
fn the_postures_of_unit_1() {
    let g = main_game();
    // Day 2: set to attack. Day 3: set to defend.
    assert_eq!(g.unit_state(2, 1).posture, Posture::Attack);
    assert_eq!(g.unit_state(3, 1).posture, Posture::Defend);
}

#[test]
fn the_supply_at_every_day_end() {
    let g = main_game();
    // Base 3 takes 30 t on the morning of day 1 and pays 5 t on the nights
    // of days 2, 3 and 4: 30, 25, 20, 15, and keeps 15 t.
    for (day, t) in [(1, 30), (2, 25), (3, 20), (4, 15), (5, 15), (8, 15)] {
        assert_eq!(g.supply(day, 3), n(t), "base 3, day {day}");
    }
    // Base 2: nothing until day 4, 80 t and 20 t on day 4: 100 t; then
    // 6.5 t paid for 1,300 men on the nights of days 5 to 8: 74 t at the end.
    assert_eq!(g.supply(1, 2), n(0));
    assert_eq!(g.supply(2, 2), n(0));
    assert_eq!(g.supply(3, 2), n(0));
    assert_eq!(g.supply(4, 2), n(100));
    assert_eq!(g.supply(5, 2), fx("93.5"));
    assert_eq!(g.supply(6, 2), n(87));
    assert_eq!(g.supply(7, 2), fx("80.5"));
    assert_eq!(g.supply(8, 2), n(74));
    // The hex holds 115 t at the end of day 4.
    assert_eq!(g.supply(4, 2) + g.supply(4, 3), n(115));
    // Base 1 loads out 80 t on day 3 and feeds nobody: 920 t at the end.
    assert_eq!(g.supply(2, 1), n(1000));
    assert_eq!(g.supply(3, 1), n(920));
    assert_eq!(g.supply(8, 1), n(920));
    // Fuel: ships 1 and 2 steam 80 nm to B and 160 nm home and burn 24 t,
    // 976 t at the end; ships 3 and 4 steam 320 nm and burn 32 t, 968 t.
    for (s, t) in [(1, 976), (2, 976), (3, 968), (4, 968)] {
        assert_eq!(g.ship(8, s).fuel_t, n(t), "ship {s}");
    }
}

#[test]
fn the_island_is_controlled_on_5_of_8_days() {
    let g = main_game();
    // The measure as read at the end of each day (the table of the hand
    // calculation).
    assert_eq!(g.controlled(), vec![0, 0, 1, 1, 1, 2, 3, 4, 5]);
    let m = g.measures[8].as_ref().expect("a measure");
    assert_eq!((m.days_in_force, m.days_controlled), (8, 5));
    let at = |day: usize| {
        g.measures[day]
            .as_ref()
            .expect("a measure")
            .places
            .iter()
            .map(|p| (p.ours, p.days_of_supply))
            .collect::<Vec<_>>()
    };
    // End of day 2: 25 t at 5 t a day, 5 days. End of day 3: 20 t, 4 days.
    assert_eq!(at(1)[0], (false, Some(n(6))));
    assert_eq!(at(2)[0], (true, Some(n(5))));
    assert_eq!(at(3)[0], (true, Some(n(4))));
    assert_says(
        g.words.last().expect("words"),
        &["controlled on 5 of 8 days in force"],
    );
}

#[test]
fn the_supply_run_is_ordered_on_day_3_only() {
    let g = main_game();
    // Task force 3 takes the run of 80 t on day 3; no run is ordered on any
    // other day.
    let loads: Vec<(u32, Given)> = g
        .all_orders()
        .into_iter()
        .filter(|(_, o)| matches!(o, Given::LoadTo(..)))
        .collect();
    assert_eq!(loads, vec![(3, Given::LoadTo(3, n(80)))]);
}

// ---------------------------------------------------------------- The properties

/// A later lift of fighting men: unit 3 a fighting unit of 300 men, and
/// unit 10, Japanese, 2,000 men, ashore at B.
fn later_lift_text() -> String {
    let japanese = "# Unit 10: Japanese, ashore at B.\n[[land_unit]]\nid = 10\nname = \"Placeholder Garrison\"\nside = \"japanese\"\nmen = 2000\nq = 4\nr = 0\n\n";
    let t = unit_3_fights(&text());
    changed(&t, ISLANDS_MARKER, &format!("{japanese}{ISLANDS_MARKER}"))
}

#[test]
fn a_later_lift_of_fighting_men() {
    let g = play(&later_lift_text());
    // The take is not met on day 4.
    let plan = g.intent_state(2).plan.raw();
    assert!(
        g.plan_day(plan, PlanState::Complete)
            .is_none_or(|day| day > 4),
        "{:?}",
        g.plan_day(plan, PlanState::Complete)
    );
    // Day 1 is the main case's.
    assert!(g.orders(1).contains(&Given::Landing(1, B, vec![1])));
    // On the morning of day 4 the take sends task force 2: a route to B and
    // back and an unload order.
    assert!(g.orders(4).contains(&route(2)), "{:?}", g.orders(4));
    assert!(g.orders(4).contains(&Given::Unload(2)), "{:?}", g.orders(4));
    // Unit 3 is added to the forces limit of the task "ready a landing".
    let (_, units, _) = g.staff_limit(4, 6).expect("a limit on day 4");
    assert!(units.contains(&3), "{units:?}");
    // Unit 3 is given no posture order before it sails.
    for day in 1..=4 {
        assert!(
            !g.orders(day).contains(&Given::Attack(3))
                && !g.orders(day).contains(&Given::Defend(3)),
            "day {day}: {:?}",
            g.orders(day)
        );
    }
    // Task force 2 is given no order before day 4.
    assert_eq!(g.orders_for_force(2, 1, 3), vec![]);
}

/// Two islands: a Japanese base 4 at C, island 2 of it, intent 2 stated on
/// day 1 after intent 1, unit 3 a fighting unit and task force 2 usable from
/// day 1.
fn two_islands_text() -> String {
    let base_4 = "# Japanese base 4 at C: nothing at it.\n[[base]]\nid = 4\nname = \"Placeholder airfield C\"\nside = \"japanese\"\nq = 0\nr = 6\nsupply_t = 0\nfuel_t = 0\n\n";
    let island_2 =
        "[[island]]\nnumber = 2\nname = \"Placeholder island 2\"\nplaces = [{ base = 4 }]\n\n";
    let t = unit_3_fights(&force_2_from_day_1(&text()));
    let t = changed(
        &t,
        ISLANDS_MARKER,
        &format!("{base_4}{island_2}{ISLANDS_MARKER}"),
    );
    format!(
        "{t}{}",
        command_table(
            1,
            "kind = \"set_intent\"\nintent = 2\nintent_kind = \"control_island\"\nisland = 2"
        )
    )
}

fn two_islands() -> &'static Played {
    static GAME: std::sync::OnceLock<Played> = std::sync::OnceLock::new();
    GAME.get_or_init(|| play(&two_islands_text()))
}

#[test]
fn two_islands_each_take_has_its_own_first_lift() {
    let g = two_islands();
    // The take of base 2 takes task force 1 as its first lift.
    assert_eq!(g.formed(take_kind_at(B)).len(), 1, "{:?}", g.forms());
    assert_eq!(g.formed(landing_kind_at(1, B)).len(), 1, "{:?}", g.forms());
    // The take of base 4 takes task force 2 as its first lift.
    assert_eq!(g.formed(take_kind_at(C)).len(), 1, "{:?}", g.forms());
    assert_eq!(g.formed(landing_kind_at(2, C)).len(), 1, "{:?}", g.forms());
    // No other force is a first lift: no landing task for task force 2 at B
    // or task force 1 at C.
    assert!(g.formed(landing_kind_at(2, B)).is_empty());
    assert!(g.formed(landing_kind_at(1, C)).is_empty());
    // Day 1's orders: task force 1 readies a landing at B and is routed to
    // B; task force 2 readies a landing at C and is routed to C. No force
    // but task force 1 is routed to B.
    let day_1 = g.orders(1);
    assert!(day_1.contains(&Given::Landing(1, B, vec![1])), "{day_1:?}");
    assert!(
        day_1
            .iter()
            .any(|o| matches!(o, Given::Landing(2, hex, _) if *hex == C)),
        "{day_1:?}"
    );
    assert!(day_1.contains(&route(1)), "{day_1:?}");
    assert!(
        day_1.contains(&Given::Route(2, vec![C, A], vec![C])),
        "{day_1:?}"
    );
    for o in &day_1 {
        if let Given::Route(tf, waypoints, act) = o {
            if *tf != 1 {
                assert!(
                    !waypoints.contains(&B) && !act.contains(&B),
                    "task force {tf} is routed to B: {o:?}"
                );
            }
        }
    }
}

/// Not part of the hand calculation: the team lead's check, decision 0151
/// ruling 6. Over the whole 8-day game of the two islands, no staff command
/// and no staff order of the Allied side is refused on any day, and no run
/// of one island's supply intent is read as the other's: every route order
/// under island 1's intents acts at B (and goes to B or A), and under
/// island 2's at C (and goes to C or A); a keep-supplied intent's base is a
/// base of the side lying in the hex of its island's place, and it stands
/// under the hold of that island's place. Restated for decision 0159's rule
/// A: the keep-supplied intent may now name the side's own base in the
/// place's hex (a base a landing formed) rather than the held base itself.
#[test]
fn two_islands_no_refusal_and_no_run_is_read_as_the_other_islands() {
    let g = two_islands();
    g.nothing_refused();
    // The root of an intent: the commander's island intent it stands under.
    let root_of = |mut raw: u32| -> u32 {
        while let Some(parent) = g.intent_state(raw).parent {
            raw = parent.raw();
        }
        raw
    };
    let mut seen_route_under = (false, false);
    for day in 1..=g.days() {
        for order in &g.handed[&day].orders {
            let Some(served) = order.reason.serves.intent else {
                continue;
            };
            let root = root_of(served.raw());
            let (hex, base) = match root {
                1 => (B, 2),
                2 => (C, 4),
                other => panic!("day {day}: an order serves intent {served:?}, root {other}"),
            };
            if let IntentKind::KeepSupplied { base: kept, .. } = g.intent_state(served.raw()).kind {
                let found = g.base(day, kept.raw()).expect("the kept base");
                assert_eq!(found.side, Side::Allied, "day {day}: {served:?}");
                assert_eq!(found.hex, hex, "day {day}: {served:?} (island of base {base})");
            }
            if let Given::Route(_, waypoints, act) = given(order) {
                assert!(
                    waypoints.iter().all(|w| *w == hex || *w == A)
                        && act.iter().all(|w| *w == hex),
                    "day {day}: a route under island {root}'s intents: {waypoints:?} acting at {act:?}"
                );
                if root == 1 {
                    seen_route_under.0 = true;
                } else {
                    seen_route_under.1 = true;
                }
            }
        }
    }
    assert!(
        seen_route_under.0 && seen_route_under.1,
        "{seen_route_under:?}"
    );
    // Every keep-supplied intent stands under the hold of its island's
    // place, and its base is a base of the side in that place's hex.
    let last = g.days();
    for intent_state in g.state.intents.values() {
        if let IntentKind::KeepSupplied { base, .. } = intent_state.kind {
            let parent = intent_state.parent.expect("a parent");
            let island = match g.intent_state(root_of(parent.raw())).kind {
                IntentKind::ControlIsland { island, .. } => island,
                other => panic!("{other:?}"),
            };
            let (place, hex) = if island == 1 { (2, B) } else { (4, C) };
            assert_eq!(g.intent_state(parent.raw()).kind, hold_kind(place));
            let found = g.base(last, base.raw()).expect("the kept base");
            assert_eq!((found.side, found.hex), (Side::Allied, hex), "{base:?}");
        }
    }
}

/// The higher take first: as two islands, but with task force 2 usable from
/// day 4 as in the main case, and unit 10, Japanese, 2,000 men, ashore at B.
fn higher_take_first_text() -> String {
    let base_4 = "# Japanese base 4 at C: nothing at it.\n[[base]]\nid = 4\nname = \"Placeholder airfield C\"\nside = \"japanese\"\nq = 0\nr = 6\nsupply_t = 0\nfuel_t = 0\n\n";
    let island_2 =
        "[[island]]\nnumber = 2\nname = \"Placeholder island 2\"\nplaces = [{ base = 4 }]\n\n";
    let japanese = "# Unit 10: Japanese, ashore at B.\n[[land_unit]]\nid = 10\nname = \"Placeholder Garrison\"\nside = \"japanese\"\nmen = 2000\nq = 4\nr = 0\n\n";
    let t = unit_3_fights(&text());
    let t = changed(
        &t,
        ISLANDS_MARKER,
        &format!("{japanese}{base_4}{island_2}{ISLANDS_MARKER}"),
    );
    format!(
        "{t}{}",
        command_table(
            1,
            "kind = \"set_intent\"\nintent = 2\nintent_kind = \"control_island\"\nisland = 2"
        )
    )
}

#[test]
fn the_higher_take_first() {
    let g = play(&higher_take_first_text());
    let id_of = |kind: IntentKind| -> u32 {
        let found: Vec<u32> = g
            .state
            .intents
            .values()
            .filter(|i| i.kind == kind)
            .map(|i| i.id.raw())
            .collect();
        assert_eq!(found.len(), 1, "{kind:?}: {:?}", g.forms());
        found[0]
    };
    let take_b = id_of(take_kind_at(B));
    let take_c = id_of(take_kind_at(C));
    // Day 1: the take of base 2 takes task force 1 as its first lift.
    assert_eq!(g.formed(landing_kind_at(1, B)), vec![(1, take_b)]);
    assert!(
        g.orders(1).contains(&Given::Landing(1, B, vec![1])),
        "{:?}",
        g.orders(1)
    );
    // The take of base 4 has no lift it may take and forms nothing.
    let under_c = |day: u32| -> Vec<IntentKind> {
        g.forms()
            .into_iter()
            .filter(|(d, p, _)| *d == day && *p == take_c)
            .map(|(_, _, k)| k)
            .collect()
    };
    assert_eq!(under_c(1), vec![], "{:?}", g.forms());
    // Day 4: the take of base 2 is not met.
    let plan = g.intent_state(take_b).plan.raw();
    assert!(
        g.plan_day(plan, PlanState::Complete)
            .is_none_or(|day| day > 4),
        "{:?}",
        g.plan_day(plan, PlanState::Complete)
    );
    // It sends task force 2 as a later lift: a route to B and back and an
    // unload order.
    assert!(g.orders(4).contains(&route(2)), "{:?}", g.orders(4));
    assert!(g.orders(4).contains(&Given::Unload(2)), "{:?}", g.orders(4));
    // Unit 3 is added to the forces limit of its task "ready a landing".
    let landing_b = id_of(landing_kind_at(1, B));
    let (_, units, _) = g.staff_limit(4, landing_b).expect("a limit on day 4");
    assert!(units.contains(&3), "{units:?}");
    // The take of base 4 forms nothing that morning.
    assert_eq!(under_c(4), vec![], "{:?}", g.forms());
    // Task force 2 is given no route to C on any day.
    for (day, o) in g.all_orders() {
        if let Given::Route(2, waypoints, act) = &o {
            assert!(
                !waypoints.contains(&C) && !act.contains(&C),
                "day {day}: task force 2 is routed to C: {o:?}"
            );
        }
    }
}

/// Not part of the hand calculation: the team lead's test of the
/// architect's ruling of Nov 18, 2026 on decision 0152, reading 7 (a first
/// lift with no ship afloat is lost, and so no longer on its way). As "A
/// later lift of fighting men", but with 1,500 t of supply in ship 1, so that
/// task force 1 is still landing it at B after its men are ashore, and a
/// Japanese force of two placeholder cruisers, task force 10, stopped at
/// (5,-1) and ordered on day 2 to B, where it sinks both transports of task
/// force 1. The take sends task force 2, its next free lift, as a later lift
/// on day 4, the first morning it may be used.
fn first_lift_sunk_text() -> String {
    let japanese_force = "# Task force 10: Japanese, two placeholder cruisers.\n[[task_force]]\nid = 10\nname = \"Placeholder cruiser force\"\nside = \"japanese\"\nq = 5\nr = -1\nmission = \"surface_combat\"\n\n[[ship]]\nid = 10\nname = \"Placeholder cruiser 1\"\nclass = 1\nside = \"japanese\"\nfuel_t = 2000\ntask_force = 10\n\n[[ship]]\nid = 11\nname = \"Placeholder cruiser 2\"\nclass = 1\nside = \"japanese\"\nfuel_t = 2000\ntask_force = 10\n\n";
    let t = changed(&later_lift_text(), "supply_t = 30\n", "supply_t = 1500\n");
    let t = changed(
        &t,
        ISLANDS_MARKER,
        &format!("{japanese_force}{ISLANDS_MARKER}"),
    );
    format!(
        "{t}\n[[order]]\nday = 2\nside = \"japanese\"\ntask_force = 10\nkind = \"set_route\"\nwaypoints = [[4, 0]]\n"
    )
}

#[test]
fn a_first_lift_with_no_ship_afloat_is_no_longer_on_its_way() {
    let g = play(&first_lift_sunk_text());
    // Task force 1 is the first lift and lands unit 1's 600 men at B on day 1.
    assert!(g.orders(1).contains(&Given::Landing(1, B, vec![1])));
    assert_eq!(g.unit_state(1, 1).men_ashore, 600);
    assert_eq!(g.unit_state(1, 1).hex, Some(B));
    // Then it has no ship afloat: both its ships sink at B on day 2, while it
    // is landing its supply.
    let sunk: Vec<(u32, u32)> = g
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::ShipSunk {
                ship, task_force, ..
            } if task_force.raw() == 1 => Some((e.day, ship.raw())),
            _ => None,
        })
        .collect();
    assert_eq!(sunk, vec![(2, 2), (2, 1)]);
    assert_eq!(
        g.truth[2].task_forces[&TaskForceId::new(1)].landing_at,
        Some(B)
    );
    // The take is not met by day 4.
    let plan = g.intent_state(2).plan.raw();
    assert!(g
        .plan_day(plan, PlanState::Complete)
        .is_none_or(|day| day > 4));
    // Task force 2 is given no order before its day, and on day 4, the
    // first morning it may be used, it is sent as a later lift: a route to B
    // and back, and an unload order.
    assert_eq!(g.orders_for_force(2, 1, 3), vec![]);
    assert!(g.orders(4).contains(&route(2)), "{:?}", g.orders(4));
    assert!(g.orders(4).contains(&Given::Unload(2)), "{:?}", g.orders(4));
}

#[test]
fn nothing_aboard_that_can_fight() {
    // Unit 1 a working unit: no lift as it stands has a fighting unit.
    let g = play(&changed(
        &text(),
        "name = \"Placeholder Infantry\"\nside = \"allied\"\nmen = 600",
        "name = \"Placeholder Infantry\"\nside = \"allied\"\nworking = true\nmen = 600",
    ));
    // The take forms nothing, hands in the notice that it cannot meet its
    // intent on day 1 and gives the reason each morning.
    assert!(g.formed(landing_kind(1)).is_empty(), "{:?}", g.forms());
    assert!(!g
        .forms()
        .iter()
        .any(|(_, _, k)| matches!(k, IntentKind::ReadyLanding { .. })));
    let plan = g.intent_state(2).plan.raw();
    let notices: Vec<(u32, NoticeKind, u32)> = g
        .notices()
        .into_iter()
        .filter(|(_, _, p)| *p == plan)
        .collect();
    assert_eq!(notices, vec![(1, NoticeKind::CannotMeet, plan)]);
    for day in 1..=8 {
        assert!(
            !g.reasons_serving(day, 2).is_empty(),
            "day {day}: no reason of the take"
        );
    }
    // No force is sent: the hold's base is not the side's. No order is
    // given at all.
    assert!(g.all_orders().is_empty(), "{:?}", g.all_orders());
    g.nothing_refused();
}

#[test]
fn a_force_that_waits_for_its_day() {
    let g = main_game();
    // Task force 2 is given no order before day 4.
    assert_eq!(g.orders_for_force(2, 1, 3), vec![]);
    // Each morning of days 1 to 3 a reason of the staff names its day.
    for day in 1..=3 {
        assert_says(&g.reasons(day), &["task_force:2 may be used from day 4"]);
    }
    // And with unit 3 a fighting unit, a lift for the take: the same.
    let v = play(&unit_3_fights(&text()));
    assert_eq!(v.orders_for_force(2, 1, 3), vec![]);
    for day in 1..=3 {
        assert_says(&v.reasons(day), &["task_force:2 may be used from day 4"]);
    }
}

#[test]
fn held() {
    // Task force 2 held from day 1: it is given no order, and the reason of
    // day 4 names the hold.
    let g = play(&with(&hold_force(1, 2)));
    assert_eq!(g.orders_for_force(2, 1, 8), vec![]);
    assert_says(
        &g.reasons(4),
        &["task_force:2 is held by the commander's hold of day 1"],
    );
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
    for day in 1..=8 {
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

/// A digest of what the Allied staff handed in over a game: every order and
/// every command by day, as debug text, hashed with FNV-1a.
fn staff_digest(played: &Played) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for day in 1..=played.days() {
        let line = format!(
            "{day}:{:?}|{:?}\n",
            played.orders(day),
            played.commands(day)
        );
        for byte in line.bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    hash
}

/// The digests of the orders and commands the Allied staff gave in each of
/// scenarios 67 to 76 before decision 0152, by file stem. To be recorded from
/// the code before the change (main at 4dec6155, recorded on Nov 18, 2026).
const EARLIER_STAFF_DIGESTS: &[(&str, u64)] = &[
    ("67_taking_a_place", 0xdea9_757f_ee2b_c26a),
    ("68_ashore", 0x8516_67b9_e76c_6a40),
    ("69_taken_and_held", 0xcc29_faa6_30b0_c5c9),
    ("70_a_run_by_night", 0xbd10_3628_2291_7359),
    ("71_bombardment_from_the_sea", 0xd2b5_e82d_1480_2860),
    ("72_reconnaissance", 0x163b_5e8b_e252_dc55),
    ("73_carrier_cover", 0x9aad_9d81_144e_f297),
    ("74_the_force_assembled", 0xf276_c8e9_4258_f593),
    ("76_controlling_an_island", 0xc126_875e_eb5b_934b),
];

#[test]
fn scenarios_67_to_76_give_the_orders_they_gave() {
    let dir = common::data_dir().join("scenarios");
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("the scenarios folder reads")
        .map(|entry| entry.expect("an entry").path())
        .filter(|p| p.extension().is_some_and(|e| e == "toml"))
        .filter(|p| {
            let stem = p.file_stem().expect("a stem").to_string_lossy().to_string();
            stem.get(..2)
                .and_then(|num| num.parse::<u32>().ok())
                .is_some_and(|num| (67..=76).contains(&num))
        })
        .collect();
    files.sort();
    let mut digests: Vec<(String, u64)> = Vec::new();
    for file in &files {
        let stem = file
            .file_stem()
            .expect("a stem")
            .to_string_lossy()
            .to_string();
        let source = std::fs::read_to_string(file).expect("the scenario reads");
        let scenario = common::from_text_with(&source, file, &common::frozen())
            .unwrap_or_else(|e| panic!("{stem} loads: {e:?}"));
        digests.push((stem, staff_digest(&play_from(scenario, BTreeMap::new()))));
    }
    for (stem, digest) in &digests {
        println!("(\"{stem}\", {digest:#018x}),");
    }
    assert!(!EARLIER_STAFF_DIGESTS.is_empty(), "no digests recorded");
    for (stem, digest) in &digests {
        let recorded = EARLIER_STAFF_DIGESTS
            .iter()
            .find(|(s, _)| s == stem)
            .unwrap_or_else(|| panic!("{stem} has no recorded digest"));
        assert_eq!(recorded.1, *digest, "{stem}");
    }
}
