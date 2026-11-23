//! Scenario 74: the force assembled by the staff. The commander says
//! "take that airfield" and names nothing else. The staff weighs two
//! bases it could sail from, takes the nearer, picks the unit, the
//! transports and a battleship to shell the place on the night of the
//! landing, and takes it.
//!
//! Proves decision 0149, step 7 of stage 3, against a hand calculation.
//! It stands on scenario 69's for the landing and scenario 71's for the
//! bombardment. The data is placeholder data: see
//! `data/scenarios/74_the_force_assembled.toml`. It runs on the frozen
//! step 1 figures, with one ground figure of its own,
//! `preparation_per_day` 100, and scenario 71's three bombardment figures
//! (5 minutes, no hit, no false claim). No enemy man is ashore anywhere,
//! so no die changes anything.
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
//! (4,0), port size 0; C at (0,6), port size 0. A to B is 4 hexes,
//! 160 nm; C to B is 6 hexes, 240 nm. The game is 6 days.
//!
//! | Base | Side | Hex | Supply | Fuel |
//! | ---- | ---- | --- | ------ | ---- |
//! | 1, "Placeholder base A" | Allied | A | 1,000 t | 5,000 t |
//! | 2, "Placeholder airfield B" | Japanese | B | 0 | 0 |
//! | 3, "Placeholder base C" | Allied | C | 1,000 t | 0 |
//!
//! Base 2 is on the Allied map and has no gun, aircraft or man. The
//! Allied side has an opening estimate of 300 enemy men at B.
//!
//! | Task force | Mission | Ships | Stopped at |
//! | --- | --- | --- | --- |
//! | 1 | transport | 1 and 2, placeholder troop transports, 600 berths each | base 1 |
//! | 2 | transport | 3, a placeholder troop transport | base 3 |
//! | 3 | bombardment | 4, a placeholder battleship, as scenario 71's | base 1 |
//!
//! | Unit | Where | Men | Experience | Assault skill |
//! | ---- | ----- | --- | ---------- | ------------- |
//! | 1, "Placeholder First" | A | 600 | 2 | 2 |
//! | 2, "Placeholder Second" | A | 600 | 2 | 2 |
//! | 3, "Placeholder Assault" | C | 600 | 2 | 4 |
//!
//! All are Allied fighting units, fed in full.
//!
//! The Allied side's commands of day 1: intent 1, take the place at base
//! 2, with no forces limit; no risk set, so balanced. Leave to prepare
//! and leave to launch are both given.
//!
//! ## The choice, on the morning of day 1
//!
//! - **The enemy.** 300 men, as told, of day 0. At balanced the men
//!   wanted are 600.
//! - **The source.** Base 1: task force 1 has 1,200 berths; unit 1 takes
//!   ship 1 and gives 600 men, the men wanted; the passage is 16 hours.
//!   Base 3: task force 2 has 600 berths; unit 3 gives 600 men; the
//!   passage is 21.2 hours: C to B is off the grid's axes and the route
//!   is charged its straight line, 212 nm, at 10 kn (restated by the
//!   architect on Nov 17, 2026: the first writing had 6 hexes and 24
//!   hours). Both give the men wanted, and base 1 is the
//!   nearer: the source is base 1.
//! - **The units.** Unit 1.
//! - **The transport force.** Task force 1.
//! - **The risk on the way.** Task force 3, on the bombardment mission,
//!   is taken. There is no carrier force and no force on the escort
//!   mission, and the reason says so.
//!
//! The limit the staff sets on the task it forms: task forces 1 and 3,
//! unit 1, and base 1 as the source.
//!
//! ## Day by day
//!
//! | Day | Hour | The staff's commands and orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 | 0 | Approves plan 1 to preparing. Forms intent 2, ready a landing by task force 1 at B, and sets its forces limit as above. Orders `prepare_landing` at B, roster 1 | Loading 100; unit 1's rehearsal 100 |
//! | 2 | 24 | Notice: intent 2 is ready. Approves plan 1 to released. Orders a load of supply to a weight of 45 t. Forms intent 3, bombard base 2 with task force 3, under intent 1. Task force 3: home port, base 1; rules of engagement, engage, break-off level 0.5 | Ship 1 loads 45 t |
//! | 3 | 48 | Unit 1 set to attack. Load unit 1 | Ship 1 takes 600 men |
//! | 4 | 72 | Task force 1: a route to B and back to A, acting at B, and an unload order. Task force 3: battle readiness; a route to B and A, B marked to act at | The battleship sails at hour 72, halts at B at hour 82 and fires 120 shells in the night. The transports sail at hour 72 and arrive at hour 88. Ship 1 lands 160 men in the morning and 440 in the afternoon, and then 10 t. Base 4, "Landing at (4,0)", is formed (restated by the architect: base 3 is base C). Base 2 changes side at the end of the day |
//! | 5 | 96 | Marks plan 1 complete; notice: intent 1 is met. Task force 3: normal readiness | Ship 1 lands 35 t in 1.75 hours. The transports steam for A at hour 108 and arrive at hour 124. The battleship steamed for A at hour 84 and arrived at hour 94 |
//! | 6 | 120 | none | |
//!
//! The landing force is due on the day it sails, day 4: the battleship
//! goes that morning. The supply is 600 x 0.005 x 15 = 45 t: no hold is
//! stated, so nothing follows.
//!
//! Eleven orders in all: six to the landing and five to the battleship.
//! Six staff commands. Units 2 and 3 and task force 2 are given no order.
//! The Allied men find B empty at the end of day 4 and the estimate is
//! cleared. No staff order is refused.
//!
//! # Properties
//!
//! - **The farther base, when the nearer cannot.** With task force 1
//!   held from day 1: base 1 has no transport force the staff may take.
//!   The source is base 3. On day 1 the staff forms "ready a landing" by
//!   task force 2 at B and sets its forces limit to task forces 2 and 3,
//!   unit 3 and base 3 as the source.
//! - **The commander names the source.** With a forces limit on intent 1
//!   that names base 3 as the source and nothing else: the choice of day
//!   1 is that of the property above, but the limit the staff sets names
//!   no source: the commander's stands.
//! - **The commander names a unit.** With a forces limit that names unit
//!   2 alone: the source is base 1, the unit is unit 2, and the limit the
//!   staff sets names task forces 1 and 3 and base 1 and no land unit.
//! - **A unit another intent has.** With a commander's intent of day 1,
//!   ranked above intent 1, whose forces limit names unit 1: the staff
//!   takes unit 2, and the reason says unit 1 serves the other intent.
//! - **Short of men, at balanced.** With unit 2 left out and an opening
//!   estimate of 500: the men wanted are 1,000. Base 1's lift gives 600
//!   and base 3's gives 600: neither is enough, and no unit or force is
//!   coming. The staff forms nothing and gives no order or command. On
//!   day 1 it hands in the notice that it cannot bring enough, with
//!   1,000 men wanted and 600 to hand, and on days 2 and 3 it gives the
//!   reason and no second notice. (Restated by the architect on Nov 18,
//!   2026, for decision 0159's rule B, item 7: before it, the staff went
//!   with unit 1 from base 1 and said it was short by 400 men. At press
//!   the men wanted are 500, base 1's lift gives them, and the staff
//!   goes with unit 1 from base 1 on day 1, short of nothing.)
//! - **Short of men, at preserve the force.** The same at preserve the
//!   force: the men wanted are 1,500. The staff forms nothing, hands in
//!   the notice that it cannot meet intent 1 on day 1 and gives the
//!   reason on each morning, with 600 men against 1,500.
//! - **Nothing to sail in.** With task forces 1 and 2 held from day 1:
//!   no base can be the source. The staff forms nothing, hands in the
//!   notice that it cannot meet intent 1 on day 1 and says on each
//!   morning that no transport force it may take lies at a base of the
//!   side.
//! - **With all three named** (task forces 1 and 3, unit 1, base 1): the
//!   orders and commands are those of the main case, and the limit the
//!   staff sets on intent 2 names unit 1 alone, as decision 0142 has it.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see and has not been told give the same
//!   orders and reasons; a save on day 3 and a resume give the same game;
//!   scenarios 67 to 73 give the orders they gave.

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::belief::GroundGrade;
use sim_core::command::{
    CommandKind, ForcesLimit, IntentKind, Limit, NoticeKind, PlanOutcome, PlanState, Reason,
    StaffDecisions, StaffOrder, Staffs,
};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, LandUnitId, ShipId, TaskForceId};
use sim_core::orders::{
    Cargo, LandUnitOrder, OrderKind, Posture, Readiness, Stance, TaskForceOrder,
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
    common::data_dir().join("scenarios/74_the_force_assembled.toml")
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

fn half() -> Fixed {
    Fixed::from_ratio(1, 2)
}

fn intent(raw: u32) -> IntentId {
    IntentId::new(raw)
}

fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

/// The frozen step 1 figures. The scenario names its own ground figures,
/// `data/reference/ground_placeholder_scenario_74.toml`, and scenario 71's
/// surface combat figures, both frozen beside the others.
fn load(text: &str) -> Scenario {
    common::from_text_with(text, &path(), &common::frozen()).expect("the scenario loads")
}

/// The Allied side's estimate of the enemy at B: men, grade, day.
type Estimate = Option<(u32, GroundGrade, u32)>;

fn estimate_of(scenario: &Scenario) -> Estimate {
    scenario
        .world
        .side_view(Side::Allied)
        .ground_estimate(B)
        .map(|e| (e.men, e.grade, e.day))
}

struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    truth: Vec<TruthState>,
    events: Vec<Event>,
    hashes: Vec<u64>,
    /// The Allied estimate at B after each day: index 0 before day 1.
    estimates: Vec<Estimate>,
}

fn play(text: &str) -> Played {
    play_from(load(text), BTreeMap::new())
}

fn play_from(mut scenario: Scenario, mut handed: BTreeMap<u32, StaffDecisions>) -> Played {
    let held = staffs();
    let mut truth = vec![scenario.world.umpire().clone()];
    let mut estimates = vec![estimate_of(&scenario)];
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
        estimates.push(estimate_of(&scenario));
    }
    Played {
        handed,
        truth,
        events: scenario.world.events().iter().cloned().collect(),
        hashes: scenario.world.day_hashes().to_vec(),
        estimates,
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
    HomePort(u32, u32),
    Roe(u32, Stance, Fixed),
    Ready(u32, Readiness),
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
                } => {
                    assert!(stand_at.is_empty(), "no stand mark");
                    Given::Route(tf, waypoints.clone(), act_at.iter().copied().collect())
                }
                TaskForceOrder::Unload => Given::Unload(tf),
                TaskForceOrder::SetHomePort(base) => Given::HomePort(tf, base.raw()),
                TaskForceOrder::SetRulesOfEngagement {
                    stance, break_off, ..
                } => Given::Roe(tf, *stance, *break_off),
                TaskForceOrder::SetReadiness(r) => Given::Ready(tf, *r),
                other => panic!("the doctrine gave a task force another order: {other:?}"),
            }
        }
        OrderKind::LandUnit {
            target,
            order:
                LandUnitOrder::SetPosture {
                    posture: Posture::Attack,
                    when_spent: None,
                },
        } => Given::Attack(target.raw()),
        other => panic!("the doctrine gave an order of another kind: {other:?}"),
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

/// A staff command, plainly, by kind.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Cmd {
    Approve(u32),
    Form(u32, IntentKind),
    /// A forces limit: intent, task forces, land units, source.
    Limit(u32, Vec<u32>, Vec<u32>, Option<u32>),
    Complete(u32),
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
        other => panic!("the staff handed in another command: {other:?}"),
    }
}

fn landing_kind(force: u32) -> IntentKind {
    IntentKind::ReadyLanding {
        force: TaskForceId::new(force),
        hex: B,
        by_day: None,
    }
}

fn bombard_kind() -> IntentKind {
    IntentKind::Bombard {
        force: TaskForceId::new(3),
        target: BaseId::new(2),
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

    fn formed_landings(&self) -> Vec<(u32, IntentKind)> {
        self.all_commands()
            .into_iter()
            .filter_map(|(d, c)| match c {
                Cmd::Form(_, k @ IntentKind::ReadyLanding { .. }) => Some((d, k)),
                _ => None,
            })
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

    /// (day, ship, tons) of every landing of supply.
    fn supply_landed(&self) -> Vec<(u32, u32, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::CargoUnloaded {
                    ship,
                    cargo: Cargo::Supply,
                    unloaded_t,
                    ..
                } => Some((e.day, ship.raw(), *unloaded_t)),
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

    /// Where the battleship (task force 3) halted, the hour, and whether it
    /// bombarded.
    fn battleship_halts(&self) -> Vec<(Hex, Fixed, bool)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceHalted {
                    task_force,
                    hex,
                    work,
                    elapsed_hours,
                } if task_force.raw() == 3 => Some((*hex, *elapsed_hours, work.bombard)),
                _ => None,
            })
            .collect()
    }

    /// The hour each steaming on of the battleship came: the start of its
    /// pulse.
    fn battleship_steamed_on(&self) -> Vec<Fixed> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceSteamedOn { task_force, .. } if task_force.raw() == 3 => {
                    let start = match e.pulse? {
                        Pulse::Night => 0,
                        Pulse::Morning => 12,
                        Pulse::Afternoon => 18,
                    };
                    Some(n(24 * (i64::from(e.day) - 1) + start))
                }
                _ => None,
            })
            .collect()
    }

    fn battleship_shells(&self) -> Vec<(u32, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::ShoreBombardment {
                    task_force, shells, ..
                } if task_force.raw() == 3 => Some((e.day, *shells)),
                _ => None,
            })
            .collect()
    }

    fn battleship_home(&self) -> Vec<Fixed> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceArrived {
                    task_force,
                    hex,
                    elapsed_hours,
                } if task_force.raw() == 3 && *hex == A => Some(*elapsed_hours),
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

fn risk(level: &str) -> String {
    command_table(
        1,
        &format!("kind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"{level}\""),
    )
}

/// The text with unit 2 left out and the opening estimate at `men`.
fn short_of_men(men: u32) -> String {
    let t = changed(
        &text(),
        "[[land_unit]]\nid = 2\nname = \"Placeholder Second\"\nside = \"allied\"\nmen = 600\nq = 0\nr = 0\n\n",
        "",
    );
    changed(&t, "men = 300\n", &format!("men = {men}\n"))
}

fn main_game() -> &'static Played {
    static GAME: std::sync::OnceLock<Played> = std::sync::OnceLock::new();
    GAME.get_or_init(|| play(&text()))
}

fn main_orders() -> Vec<(u32, Vec<Given>)> {
    vec![
        (1, vec![Given::Landing(1, B, vec![1])]),
        (
            2,
            vec![
                Given::LoadTo(1, n(45)),
                Given::HomePort(3, 1),
                Given::Roe(3, Stance::Engage, half()),
            ],
        ),
        (3, vec![Given::Attack(1), Given::LoadUnit(1, 1)]),
        (
            4,
            vec![
                route(1),
                Given::Unload(1),
                Given::Ready(3, Readiness::Battle),
                route(3),
            ],
        ),
        (5, vec![Given::Ready(3, Readiness::Normal)]),
        (6, vec![]),
    ]
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
    assert!(t.contains("days = 6"));
    assert!(t.contains("men = 300\n"));
}

#[test]
fn the_staff_commands_and_notices() {
    let g = main_game();
    g.nothing_refused();
    assert_eq!(g.days(), 6);
    // The limit the staff sets on the task it forms: task forces 1 and 3,
    // unit 1, and base 1 as the source.
    assert_eq!(
        g.commands(1),
        vec![
            Cmd::Approve(1),
            Cmd::Form(1, landing_kind(1)),
            Cmd::Limit(2, vec![1, 3], vec![1], Some(1)),
        ]
    );
    assert_eq!(
        g.commands(2),
        vec![Cmd::Approve(1), Cmd::Form(1, bombard_kind())]
    );
    for day in [3, 4, 6] {
        assert!(g.commands(day).is_empty(), "day {day}");
    }
    assert_eq!(g.commands(5), vec![Cmd::Complete(1)]);
    // Six staff commands in all.
    assert_eq!(g.all_commands().len(), 6);
    // Intent 2 is the landing, intent 3 the bombardment, both under intent 1.
    assert_eq!(g.formed_intents(), vec![(1, 2, 1), (2, 3, 1)]);
    assert_eq!(g.plan_day(1, PlanState::Released), Some(2));
    assert_eq!(g.plan_day(1, PlanState::Complete), Some(5));
    // Intent 2 is ready on day 2, intent 1 is met on day 5.
    assert_eq!(
        g.notices(),
        vec![(2, NoticeKind::Ready, 1), (5, NoticeKind::Met, 1)]
    );
}

#[test]
fn eleven_orders_day_by_day() {
    let g = main_game();
    for (day, wanted) in main_orders() {
        assert!(
            same(&g.orders(day), &wanted),
            "day {day}: {:?} against {wanted:?}",
            g.orders(day)
        );
    }
    // Eleven in all: six to the landing and five to the battleship.
    let all = g.all_orders();
    assert_eq!(all.len(), 11);
    let to_battleship = all
        .iter()
        .filter(|(_, o)| {
            matches!(
                o,
                Given::HomePort(3, _)
                    | Given::Roe(3, ..)
                    | Given::Ready(3, _)
                    | Given::Route(3, ..)
            )
        })
        .count();
    assert_eq!(to_battleship, 5);
    // Units 2 and 3 and task force 2 are given no order.
    for (day, o) in &all {
        match o {
            Given::Landing(tf, ..)
            | Given::LoadTo(tf, _)
            | Given::LoadUnit(tf, _)
            | Given::Route(tf, ..)
            | Given::Unload(tf)
            | Given::HomePort(tf, _)
            | Given::Roe(tf, ..)
            | Given::Ready(tf, _) => assert_ne!(*tf, 2, "day {day}: {o:?}"),
            Given::Attack(_) => {}
        }
        match o {
            Given::Attack(u) | Given::LoadUnit(_, u) => assert_eq!(*u, 1, "day {day}: {o:?}"),
            _ => {}
        }
    }
    g.nothing_refused();
}

#[test]
fn the_landing_and_the_hours() {
    let g = main_game();
    // Day 1: loading 100; unit 1's rehearsal 100, and unit 2 has none.
    assert_eq!(g.unit_state(1, 1).rehearsal, n(100));
    assert_eq!(g.unit_state(1, 2).rehearsal, Fixed::ZERO);
    // Day 2: ship 1 loads 45 t.
    assert_eq!(g.ship(2, 1).supply_t, n(45));
    // Day 3: unit 1 is on attack.
    assert_eq!(g.unit_state(3, 1).posture, Posture::Attack);
    // The transports sail at hour 72, the night of day 4, from A, and arrive
    // at hour 88; they are home at hour 124.
    assert_eq!(
        g.transport_steaming().first().copied(),
        Some((4, Pulse::Night, A))
    );
    assert_eq!(g.transport_arrivals(), vec![(B, n(88)), (A, n(124))]);
    // Ship 1 lands 160 men in the morning and 440 in the afternoon.
    assert_eq!(
        g.landed(),
        vec![
            (4, Pulse::Morning, 1, 1, 160),
            (4, Pulse::Afternoon, 1, 1, 440)
        ]
    );
    // Then 10 t on day 4 and 35 t on day 5, in 1.75 hours.
    assert_eq!(g.supply_landed(), vec![(4, 1, n(10)), (5, 1, n(35))]);
    assert_eq!(g.unloading_complete(), vec![(2, fx("97.75"))]);
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
fn the_battleship_halts_at_b_at_hour_82_and_fires_120_shells() {
    let g = main_game();
    assert_eq!(g.battleship_halts(), vec![(B, n(82), true)]);
    assert_eq!(g.battleship_shells(), vec![(4, n(120))]);
    // She steams for A at hour 84 and is home at hour 94.
    assert_eq!(g.battleship_steamed_on(), vec![n(84)]);
    assert_eq!(g.battleship_home(), vec![n(94)]);
}

#[test]
fn the_estimate_is_told_at_first_and_cleared_after_day_4() {
    let g = main_game();
    assert_eq!(g.estimates[0], Some((300, GroundGrade::Told, 0)));
    for day in 4..=6 {
        assert_eq!(g.estimates[day], None, "after day {day}");
    }
}

#[test]
fn the_reason_of_the_choice_names_base_1_and_what_it_lacks() {
    let g = main_game();
    let day_1 = g.reasons(1);
    assert_says(
        &day_1,
        &[
            "the source is base:1",
            "no force on the carrier mission",
            "no force on the escort mission",
        ],
    );
}

#[test]
fn the_source_base_1_has_1200_berths_and_16_hours_and_base_3_600_berths() {
    let g = main_game();
    assert_says(
        &g.reasons_serving(1, 1),
        &[
            "base:1 (Placeholder base A): task_force:1 with 1200 berths, its lift gives 600 men, 16 hours to (4,0)",
            "base:3 (Placeholder base C): task_force:2 with 600 berths, its lift gives 600 men,",
            "the source is base:1 (Placeholder base A): its lift gives the men wanted with the shortest passage",
        ],
    );
}

#[test]
fn the_passage_from_base_3_is_21_2_hours() {
    let g = main_game();
    assert_says(
        &g.reasons_serving(1, 1),
        &["task_force:2 with 600 berths, its lift gives 600 men, 21.2 hours to (4,0)"],
    );
}

#[test]
fn the_landing_forms_base_4() {
    let g = main_game();
    assert_eq!(g.base(4, 4).expect("base 4").name, "Landing at (4,0)");
}

// ---------------------------------------------------------------- The properties

#[test]
fn the_farther_base_when_the_nearer_cannot() {
    let g = play(&with(&hold_force(1, 1)));
    assert_eq!(g.formed_landings().first(), Some(&(1, landing_kind(2))));
    assert_eq!(
        g.staff_limit(1, 2),
        Some((vec![2, 3], vec![3], Some(3))),
        "{:?}",
        g.commands(1)
    );
    assert!(
        g.orders(1).contains(&Given::Landing(2, B, vec![3])),
        "{:?}",
        g.orders(1)
    );
    assert_says(&g.reasons(1), &["the source is base:3"]);
}

#[test]
fn the_commander_names_the_source() {
    let g = play(&format!(
        "{}{}",
        with(&hold_force(1, 1)),
        forces_limit("source = 3")
    ));
    assert_eq!(g.formed_landings().first(), Some(&(1, landing_kind(2))));
    // The staff's limit names no source: the commander's stands.
    assert_eq!(
        g.staff_limit(1, 2),
        Some((vec![2, 3], vec![3], None)),
        "{:?}",
        g.commands(1)
    );
}

#[test]
fn the_commander_names_a_unit() {
    let g = play(&with(&forces_limit("land_units = [2]")));
    assert_eq!(g.formed_landings().first(), Some(&(1, landing_kind(1))));
    assert_eq!(
        g.staff_limit(1, 2),
        Some((vec![1, 3], vec![], Some(1))),
        "{:?}",
        g.commands(1)
    );
    assert!(
        g.orders(1).contains(&Given::Landing(1, B, vec![2])),
        "{:?}",
        g.orders(1)
    );
}

#[test]
fn a_unit_another_intent_has() {
    let extra = format!(
        "{}{}{}",
        command_table(
            1,
            "kind = \"set_intent\"\nintent = 2\nintent_kind = \"hold\"\nbase = 1"
        ),
        command_table(
            1,
            "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 2\nland_units = [1]"
        ),
        command_table(
            1,
            "kind = \"set_limit\"\nlimit = \"priority\"\nintent = 2\nrank = 1"
        ),
    );
    let g = play(&with(&extra));
    // The staff forms intent 3, the landing, with unit 2.
    assert_eq!(g.formed_landings().first(), Some(&(1, landing_kind(1))));
    assert_eq!(
        g.staff_limit(1, 3).map(|(_, units, _)| units),
        Some(vec![2]),
        "{:?}",
        g.commands(1)
    );
    assert_says(
        &g.reasons_serving(1, 1),
        &["land_unit:1", "serves intent:2"],
    );
}

#[test]
fn short_of_men_at_balanced() {
    let g = play(&short_of_men(500));
    // Decision 0159, rule B, item 7: no unit or force is coming, so at
    // balanced the staff forms nothing and asks the commander.
    assert!(g.formed_landings().is_empty(), "{:?}", g.formed_landings());
    for day in 1..=3 {
        assert!(g.orders(day).is_empty(), "day {day}: {:?}", g.orders(day));
        assert!(
            g.commands(day).is_empty(),
            "day {day}: {:?}",
            g.commands(day)
        );
    }
    assert_eq!(g.notices(), vec![(1, NoticeKind::CannotBringEnough, 1)]);
    for day in 1..=3 {
        let reasons = g.no_orders(day, 1);
        assert_eq!(reasons.len(), 1, "day {day}: {reasons:?}");
        assert_says(
            &reasons[0],
            &["The staff wants 1000 men and has 600 to hand"],
        );
        assert_eq!(
            reasons[0].contains("hands in the notice that it cannot bring enough"),
            day == 1,
            "day {day}: {reasons:?}"
        );
    }
}

#[test]
fn short_of_men_at_press_it_goes() {
    // At press the men wanted are 500, base 1's lift gives them, and the
    // staff goes with unit 1 from base 1 on day 1, short of nothing.
    let g = play(&format!("{}{}", short_of_men(500), risk("press")));
    assert_eq!(g.formed_landings().first(), Some(&(1, landing_kind(1))));
    let (_, units, source) = g.staff_limit(1, 2).expect("a limit on day 1");
    assert_eq!((units, source), (vec![1], Some(1)), "{:?}", g.commands(1));
    assert!(!g.reasons(1).contains("short by"), "{}", g.reasons(1));
}

#[test]
fn short_of_men_at_preserve_the_force() {
    let g = play(&format!("{}{}", short_of_men(500), risk("preserve")));
    assert!(g.formed_landings().is_empty(), "{:?}", g.formed_landings());
    assert!(!g
        .all_commands()
        .iter()
        .any(|(_, c)| matches!(c, Cmd::Form(..))));
    assert_eq!(g.notices(), vec![(1, NoticeKind::CannotMeet, 1)]);
    for day in 1..=6 {
        let reasons = g.no_orders(day, 1).join(" || ");
        assert_says(&reasons, &["600 men against the 1500 wanted"]);
    }
}

#[test]
fn nothing_to_sail_in() {
    let g = play(&format!("{}{}", with(&hold_force(1, 1)), hold_force(1, 2)));
    assert!(g.formed_landings().is_empty(), "{:?}", g.formed_landings());
    assert!(!g
        .all_commands()
        .iter()
        .any(|(_, c)| matches!(c, Cmd::Form(..))));
    assert_eq!(g.notices(), vec![(1, NoticeKind::CannotMeet, 1)]);
    for day in 1..=6 {
        let reasons = g.no_orders(day, 1).join(" || ");
        assert_says(
            &reasons,
            &["no transport force the staff may take lies stopped at a base of the side"],
        );
    }
}

#[test]
fn with_all_three_named() {
    let g = play(&with(&forces_limit(
        "task_forces = [1, 3]\nland_units = [1]\nsource = 1",
    )));
    g.nothing_refused();
    let m = main_game();
    for day in 1..=6 {
        assert!(
            same(&g.orders(day), &m.orders(day)),
            "day {day}: {:?}",
            g.orders(day)
        );
    }
    // The commands are the main case's but for the limit of day 1, which
    // names unit 1 alone.
    for day in 1..=6 {
        let mut wanted = m.commands(day);
        if day == 1 {
            wanted[2] = Cmd::Limit(2, vec![], vec![1], None);
        }
        assert_eq!(g.commands(day), wanted, "day {day}");
    }
    assert_eq!(g.notices(), m.notices());
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
    let with_enemy = |supply: u32| {
        changed(
            &text(),
            COMMANDS_MARKER,
            &format!("{}{COMMANDS_MARKER}", hidden(supply)),
        )
    };
    let a = play(&with_enemy(50));
    let b = play(&with_enemy(400));
    assert_ne!(
        a.truth.last(),
        b.truth.last(),
        "the two worlds do not differ"
    );
    assert_eq!(a.handed, b.handed);
    for day in 1..=6 {
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
