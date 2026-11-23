//! Scenario 86: fed with the lift. The commander says "control the
//! island". The side's men are at sea in transports whose home is a base
//! with no supply. The staff names the base that has supply as the source
//! on the first morning, sends the transports on to it when the men are
//! ashore, and feeds the place with those same ships.
//!
//! Proves decision 0163, the second part of the first mend of step 3's
//! stage 4, against a hand calculation. It is scenario 77 with task
//! forces 2 and 3 taken away and the lift's home changed. The data is
//! placeholder data: see `data/scenarios/86_fed_with_the_lift.toml`. It
//! runs on the frozen step 1 figures. No enemy man is ashore anywhere, so
//! no die is rolled.
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
//! As scenario 77's. A man weighs 0.25 t to land and eats 0.005 t a day,
//! paid at the start of the night pulse by the side's bases in the hex in
//! base order. Supply landed goes into the side's lowest-numbered base in
//! the hex. A size 0 port works by lighter at 20 t an hour for each ship,
//! and on the frozen figures a landing is not slowed by its loading. Day
//! d begins at hour 24 x (d - 1): a night pulse of 12 hours and two
//! daylight pulses of 6. In a pulse ships move before cargo is worked. A
//! force that has landed its load steams on at the start of the next
//! pulse. A hex is 40 nm and the transports make 10 kn and burn 0.1 t of
//! fuel a mile.
//!
//! A working unit ashore at a base with nothing else to do works on its
//! port: unit 2, 400 men, works from day 2, and the rate of unloading
//! there is 1.4 times the lighter's: 28 t an hour for each ship.
//!
//! The staff's figures: at balanced the margin is 5 days; a supply intent
//! with no interval uses 15 days. A run ordered on a morning loads that
//! day, sails the next and arrives that same day; it is ordered when the
//! reading of the day after it would arrive is below the floor's stock.
//! A supply intent is first served on the morning after it is formed. A
//! force found empty at the source on a morning is in the pool from the
//! next morning.
//!
//! ## Inputs
//!
//! The placeholder map of scenario 27: A at (0,0), port size 5; B at
//! (4,0), port size 0; C at (0,6), port size 0. The game is 8 days.
//!
//! | Base | Side | Hex | Supply | Fuel |
//! | ---- | ---- | --- | ------ | ---- |
//! | 1, "Placeholder base A" | Allied | A | 1,000 t | 0 |
//! | 2, "Placeholder airfield B" | Japanese | B | 0 | 0 |
//! | 4, "Placeholder staging C" | Allied | C | 0 | 0 |
//!
//! Base 2 is on the Allied map, with no gun, aircraft or man. No base
//! has a reserve. No estimate and no contact is held. Island 1,
//! "Placeholder island": base 2.
//!
//! Task force 1, Allied, on the transport mission: ships 1 and 2,
//! placeholder troop transports (600 men, 3,000 t of cargo, 1,000 t of
//! fuel, full), at sea at (2,0), stopped. Its home port is base 4. Unit 1
//! is in ship 1 with 30 t of supply; unit 2 is in ship 2. There is no
//! other force.
//!
//! | Unit | Kind | Men | Posture |
//! | ---- | ---- | --- | ------- |
//! | 1, "Placeholder Infantry" | fighting | 600 | defend |
//! | 2, "Placeholder Pioneers" | working | 400 | |
//!
//! The Allied side's commands of day 1: intent 1, control island 1. No
//! forces limit and no risk, so balanced. Leave to prepare and leave to
//! launch are both given.
//!
//! ## Day by day
//!
//! | Day | Hour | The staff's orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 | 0 | Task force 1: `prepare_landing` at B, roster 1; a route to B and on to A, acting at B; an unload order | Sails at hour 0 and arrives at hour 8. In the night's last 4 hours each ship lands 320 men. In the morning ship 1 lands 280 men in 3.5 hours and then 30 t in 1.5 hours; ship 2 lands 80 men in an hour. Base 5, "Landing at (4,0)", is formed. The force steams for A at hour 18 |
//! | 2 | 24 | Unit 1 set to attack | The force arrives at A at hour 34. Base 2 changes side at the end of the day |
//! | 3 | 48 | Unit 1 set to defend | |
//! | 4 | 72 | Task force 1: a load of supply to a weight of 85 t | Ship 1 loads 85 t at A |
//! | 5 | 96 | Task force 1: a route to B and back to A, acting at B, and an unload order | Sails at hour 96 and arrives at hour 112. At 28 t an hour ship 1 lands 56 t in the morning's 2 hours and 29 t in the afternoon. The force steams for A at hour 120 and arrives at hour 136 |
//! | 6 to 8 | | none | |
//!
//! Eight orders in all. No staff order or command is refused.
//!
//! - **Day 1.** The tree is scenario 77's. The take's first lift is task
//!   force 1, a lift as it stands, and its plan is released. The source
//!   of supply is base 1: 1,000 t above its reserve, against nothing at
//!   base 4, and no transport force lies there. The hold's forces limit
//!   is base 1 and no force. The lift is routed to B and on to base 1,
//!   the source of the hold of the place: each ship has 1,000 t of fuel
//!   for a passage of 80 nm and 160 nm, 24 t. The route's reason names
//!   base 1 and says it is the source of supply.
//! - **Day 2.** The judgement is to attack. The hold of base 2 still
//!   waits, and the side has base 5 in the hex with 1,000 men ashore:
//!   "keep base 5 supplied", with a floor of 5 days, is formed under the
//!   hold.
//! - **Day 3.** The take is met and unit 1 is set to defend. Task force 1
//!   lies empty at base 1: the hold's forces limit is set to base 1 and
//!   task force 1. The supply intent is served for the first time, on
//!   the limit as it stood that morning, base 1 and no force. The pool
//!   has no ship afloat: the staff gives no order for it, and its one
//!   reason says that no ship of the pool is afloat and there is nothing
//!   to send. The stock and the run are not reckoned on a morning with
//!   no ship. (Restated by the architect on Nov 19, 2026: the first
//!   writing had the run reckoned and then found to have no ship.)
//! - **Day 4.** The limit is base 1 and task force 1. The stock at B is
//!   20 t. A run ordered today arrives on day 5, and the reading of day
//!   6 is 20 - 10 = 10 t, below 25: a run. Its load is the floor's stock
//!   on the day it arrives, 25 t, less that day's reading, 15 t, which
//!   is 10 t; plus 15 days of use, 75 t: 85 t. Ship 1's room covers it.
//! - **Day 5 and after.** No second run is called: with the run landed
//!   the hex holds 95 t at the end of day 5 against a use of 5 t.
//!
//! ## Supply, fuel and men
//!
//! - **Base 5**, the base the landing forms (bases 1, 2 and 4 are in
//!   the scenario, so it takes the next number), takes 30 t on the morning of day 1. It pays 5 t for 1,000
//!   men on the nights of days 2, 3, 4 and 5: base 2, Allied from the
//!   end of day 2 and the lower numbered, holds nothing on those nights.
//!   It holds 25, 20, 15 and 10 t after them and ends with 10 t.
//! - **Base 2** takes 85 t on day 5. It pays 5 t on the nights of days
//!   6, 7 and 8: 70 t at the end.
//! - **Base 1** loads out 85 t and feeds nobody: 915 t at the end.
//!   **Base 4** holds nothing throughout, and no force comes to it.
//! - **Fuel.** Each ship steams 80 nm to B and 160 nm to A, then 320 nm
//!   on the run: 560 nm, 56 t. Each ends with 944 t.
//! - **Every man is fed every night.** Unit 1 has 600 men at the end and
//!   unit 2 has 400.
//!
//! As the rule stood before decision 0163 the lift was routed back to
//! base 4, and base 4, the first base at which a force lay empty, was
//! named the source, with no supply: no run was ever sent.
//!
//! # Properties
//!
//! - **A base with nothing cannot be the source.** With base 1's supply
//!   0: no base can be the source, and the reason of day 1 says so. The
//!   hold's forces limit names no source on any day, no "keep supplied"
//!   is formed, and the lift is routed to B and back to base 4, as
//!   before.
//! - **Not the fuel to go on.** With base 1's supply 0, base 4's
//!   1,000 t, task force 1's home port base 1, and each ship holding
//!   26 t of fuel: the source is base 4. The passage to B burns 8 t and
//!   the passage on to base 4, at C, more than the 18 t left; the
//!   passage back to base 1 burns 16 t, 24 t out and back against 26 t.
//!   The lift is routed to B and back to base 1, as before, and the
//!   route's reason says it does not go on to base 4 and gives each
//!   ship's fuel against the burn to B and on. Only the orders and the
//!   route's reason of day 1 are asserted. (Restated by the architect on
//!   Nov 19, 2026: in the first writing the way home was the longer, so
//!   a ship short for the way on was short for the way home too.)
//! - **The commander's source.** With intent 1 given a forces limit that
//!   names base 4 as the source: the source is base 4, as before, and
//!   the lift is routed to B and back to base 4.
//! - **Scenario 77 gives the orders it gave:** its lift's home is the
//!   source, and a transport lies empty there on day 1.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see give the same orders and reasons; a
//!   save on day 4 and a resume give the same game.

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::command::{
    CommandKind, CommandState, ForcesLimit, IntentKind, Limit, Maker, PlanOutcome, PlanState,
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

mod common;

const COMMANDS_MARKER: &str = "# The Allied commander's commands";
const ISLANDS_MARKER: &str = "# ISLANDS_MARKER\n";
const A: Hex = Hex { q: 0, r: 0 };
const B: Hex = Hex { q: 4, r: 0 };
const C: Hex = Hex { q: 0, r: 6 };

fn path() -> PathBuf {
    common::data_dir().join("scenarios/86_fed_with_the_lift.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn intent(raw: u32) -> IntentId {
    IntentId::new(raw)
}

fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

/// The frozen step 1 figures.
fn load(text: &str) -> Scenario {
    common::from_text_with(text, &path(), &common::frozen()).expect("the scenario loads")
}

struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    truth: Vec<TruthState>,
    events: Vec<Event>,
    hashes: Vec<u64>,
    state: CommandState,
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
        state: scenario.world.command_state(Side::Allied).clone(),
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

/// The base formed at B on the morning of day 1, "Landing at (4,0)".
fn landing(g: &Played) -> u32 {
    g.events
        .iter()
        .find_map(|e| match &e.kind {
            EventKind::BaseFormed {
                base,
                side: Side::Allied,
                hex,
                ..
            } if *hex == B => Some(base.raw()),
            _ => None,
        })
        .expect("a base is formed at B")
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
    assert_eq!(t.matches("working = true").count(), 1);
    assert_eq!(t.matches("fuel_t = 1000").count(), 2);
    assert_eq!(t.matches("supply_t = 1000").count(), 1);
    assert_eq!(t.matches("home_port = 4").count(), 1);
    // Only task force 1 and no order of any kind is in the file.
    assert_eq!(t.matches("[[task_force]]").count(), 1);
    assert!(!t.contains("[[order]]"));
}

#[test]
fn the_tree_of_the_first_morning() {
    let g = main_game();
    assert_eq!(g.days(), 8);
    // The tree is scenario 77's: intents 2 to 5, then the landing under the
    // take (intent 6); all on day 1.
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
    assert_eq!(one.maker, Maker::Commander);
}

#[test]
fn the_first_lift_of_the_take_is_task_force_1_and_its_plan_is_released_on_day_1() {
    let g = main_game();
    assert_eq!(g.formed(landing_kind(1)), vec![(1, 2)]);
    let (tfs, units, _) = g.staff_limit(1, 6).expect("a limit on day 1");
    assert_eq!(tfs, vec![1]);
    assert!(units.contains(&1), "{units:?}");
    let plan = g.intent_state(2).plan.raw();
    assert_eq!(g.plan_day(plan, PlanState::Released), Some(1));
    // The take is met on day 3.
    assert_eq!(g.plan_day(plan, PlanState::Complete), Some(3));
}

#[test]
fn the_source_of_supply_is_base_1_named_on_day_1_with_no_force() {
    let g = main_game();
    // Day 1: the hold's forces limit is base 1 and no force.
    assert_eq!(g.staff_limit(1, 3), Some((vec![], vec![], Some(1))));
    // Day 3: task force 1 lies empty at base 1: base 1 and task force 1.
    assert_eq!(g.staff_limit(3, 3), Some((vec![1], vec![], Some(1))));
    // No other day names another source.
    for day in 1..=8 {
        if let Some((_, _, source)) = g.staff_limit(day, 3) {
            assert_eq!(source, Some(1), "day {day}");
        }
    }
    let day_1 = g.reasons(1);
    assert_says(
        &day_1,
        &[
            "base:1 (Placeholder base A): 1000 t of supply above its reserve",
            "no transport force stopped there with nothing aboard",
            "base:4 (Placeholder staging C): nothing above its reserve, so it cannot be the source",
            "the source is base:1 (Placeholder base A)",
            "to base:1 (Placeholder base A) and no force",
        ],
    );
}

#[test]
fn the_lift_is_routed_to_b_and_on_to_base_1_and_the_reason_says_why() {
    let g = main_game();
    let route_reason = g.handed[&1]
        .orders
        .iter()
        .filter(|o| matches!(given(o), Given::Route(..)))
        .map(|o| text_of(&o.reason))
        .collect::<Vec<_>>()
        .join(" || ");
    assert_says(
        &route_reason,
        &[
            "80 nm out at 10 kn = 8 hours, so it arrives at (4,0) at hour 8; 160 nm on to base:1",
            "It goes on to base:1 (Placeholder base A), the source of supply of intent:3, the hold of (4,0)",
            "every ship has the fuel",
            "ship:1 has 1000 t against 8 t to (4,0) and 16 t on to base:1 (Placeholder base A), 24 t",
            "ship:2 has 1000 t against 8 t to (4,0) and 16 t on to base:1 (Placeholder base A), 24 t",
            "out to the place and on to the source of supply of the hold of the place",
        ],
    );
}

#[test]
fn the_take_run_the_same_morning_sees_the_source() {
    // The check of decision 0144 is made on the morning the island names the
    // source: it is in the release's reason, and it names base 1.
    let g = main_game();
    assert_says(
        &g.reasons(1),
        &[
            "The check that the place can be held, for intent:3 of base:2 (Placeholder airfield B)",
            "base:1 holds 1000 t above its reserve against 45 t + 75 t = 120 t",
            "it can be held",
        ],
    );
}

#[test]
fn the_neutralising_is_withdrawn_on_day_3_by_the_staff() {
    let g = main_game();
    assert!(
        g.commands(3).contains(&Cmd::Withdraw(5)),
        "{:?}",
        g.commands(3)
    );
}

#[test]
fn base_5_landing_at_4_0_is_formed_and_base_2_changes_side_at_the_end_of_day_2() {
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
    // Bases 1, 2 and 4 are in the scenario: the landing's is base 5.
    assert_eq!(formed[0].1.raw(), 5);
    assert_eq!(
        g.base(1, landing(g)).expect("formed").name,
        "Landing at (4,0)"
    );
    assert_eq!(g.changed_side_day(2), Some(2));
    assert_eq!(g.base(1, 2).expect("base 2").side, Side::Japanese);
    assert_eq!(g.base(2, 2).expect("base 2").side, Side::Allied);
}

#[test]
fn keep_the_landing_base_supplied_with_floor_5_is_formed_on_day_2_under_the_hold() {
    let g = main_game();
    let ld = landing(g);
    assert_eq!(g.formed(keep_kind(ld)), vec![(2, 3)]);
    assert_eq!(g.formed_intents().last(), Some(&(2, 7, 3)));
    assert_eq!(g.intent_state(7).kind, keep_kind(ld));
    assert!(g.formed(keep_kind(2)).is_empty(), "{:?}", g.forms());
    // It is served for the first time on day 3.
    assert!(
        g.reasons_serving(2, 7).is_empty(),
        "{}",
        g.reasons_serving(2, 7)
    );
}

#[test]
fn day_3_the_supply_intent_is_served_on_base_1_and_no_force_with_no_ship_afloat() {
    let g = main_game();
    // The pool has no ship afloat: no order for it, and its one reason says
    // no ship of the pool is afloat and there is nothing to send. The stock
    // and the run are not reckoned.
    let reasons = g.reasons_serving(3, 7);
    assert_says(
        &reasons,
        &[
            "No ship of the pool of intent:7 is afloat, so there is nothing to send",
            "no ship of the pool is afloat",
        ],
    );
    assert!(!reasons.contains("a day"), "{reasons}");
    assert_eq!(reasons.split(" || ").count(), 1, "one reason: {reasons}");
    assert_eq!(g.orders(3), vec![Given::Defend(1)]);
}

#[test]
fn the_orders_of_days_1_to_8_eight_in_all_none_refused() {
    let g = main_game();
    let wanted: Vec<(u32, Vec<Given>)> = vec![
        (
            1,
            vec![Given::Landing(1, B, vec![1]), route(1), Given::Unload(1)],
        ),
        (2, vec![Given::Attack(1)]),
        (3, vec![Given::Defend(1)]),
        (4, vec![Given::LoadTo(1, n(85))]),
        (5, vec![route(1), Given::Unload(1)]),
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
    assert_eq!(g.all_orders().len(), 8);
    assert!(!g
        .all_orders()
        .iter()
        .any(|(_, o)| matches!(o, Given::Other(_))));
    g.nothing_refused();
}

#[test]
fn the_hours_of_the_lift_and_of_the_run() {
    let g = main_game();
    // Sails at hour 0, arrives at B at hour 8, steams for A at hour 18 and
    // arrives at A at hour 34; the run sails at hour 96, arrives at B at hour
    // 112, steams for A at hour 120 and arrives at hour 136.
    assert_eq!(
        g.arrivals_of(1),
        vec![(B, n(8)), (A, n(34)), (B, n(112)), (A, n(136))]
    );
    let steaming = g.steaming_of(1);
    for want in [
        (1, Pulse::Afternoon, B),
        (5, Pulse::Night, A),
        (6, Pulse::Night, B),
    ] {
        assert!(steaming.contains(&want), "{want:?} in {steaming:?}");
    }
}

#[test]
fn the_men_and_supply_landed() {
    let g = main_game();
    let mut want: Vec<String> = [
        (1, "Night", 1, 1, 320),
        (1, "Night", 2, 2, 320),
        (1, "Morning", 1, 1, 280),
        (1, "Morning", 2, 2, 80),
    ]
    .iter()
    .map(|(d, p, u, s, m)| format!("{d} {p} unit {u} ship {s} men {m}"))
    .collect();
    want.sort();
    assert_eq!(g.landed(), want);
    // 30 t in the morning of day 1; on day 5, at 28 t an hour, ship 1 lands
    // 56 t in the morning and 29 t in the afternoon.
    let mut want_t: Vec<String> = [(1, "Morning", 30), (5, "Morning", 56), (5, "Afternoon", 29)]
        .iter()
        .map(|(d, p, t)| format!("{d} {p} ship 1 t {}", n(*t)))
        .collect();
    want_t.sort();
    assert_eq!(g.supply_landed(), want_t);
}

#[test]
fn the_postures_of_unit_1() {
    let g = main_game();
    assert_eq!(g.unit_state(2, 1).posture, Posture::Attack);
    assert_eq!(g.unit_state(3, 1).posture, Posture::Defend);
}

#[test]
fn the_supply_at_every_day_end() {
    let g = main_game();
    let ld = landing(g);
    // The landing base takes 30 t on the morning of day 1 and pays 5 t on the
    // nights of days 2, 3, 4 and 5: 30, 25, 20, 15, 10, and keeps 10 t.
    for (day, t) in [
        (1, 30),
        (2, 25),
        (3, 20),
        (4, 15),
        (5, 10),
        (6, 10),
        (7, 10),
        (8, 10),
    ] {
        assert_eq!(g.supply(day, ld), n(t), "landing base, day {day}");
    }
    // Base 2: nothing until day 5, 85 t on day 5, then 5 t a night on the
    // nights of days 6, 7 and 8: 70 t at the end.
    for (day, t) in [
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 85),
        (6, 80),
        (7, 75),
        (8, 70),
    ] {
        assert_eq!(g.supply(day, 2), n(t), "base 2, day {day}");
    }
    // The hex holds 95 t at the end of day 5.
    assert_eq!(g.supply(5, 2) + g.supply(5, ld), n(95));
    // Base 1 loads out 85 t on day 4 and feeds nobody: 915 t at the end.
    for (day, t) in [(1, 1000), (2, 1000), (3, 1000), (4, 915), (8, 915)] {
        assert_eq!(g.supply(day, 1), n(t), "base 1, day {day}");
    }
    // Base 4 holds nothing throughout.
    for day in 0..=8 {
        assert_eq!(g.supply(day, 4), n(0), "base 4, day {day}");
    }
    // Fuel: each ship steams 80 nm, 160 nm and 320 nm, 560 nm, 56 t.
    for s in [1, 2] {
        assert_eq!(g.ship(8, s).fuel_t, n(944), "ship {s}");
    }
    // Every man is fed every night.
    assert_eq!(g.unit_state(8, 1).men_ashore, 600);
    assert_eq!(g.unit_state(8, 2).men_ashore, 400);
    for day in 1..=8 {
        for u in [1, 2] {
            assert_eq!(g.unit_state(day, u).fed_share, n(1), "unit {u}, day {day}");
        }
    }
}

#[test]
fn the_supply_run_is_ordered_on_day_4_only() {
    let g = main_game();
    let loads: Vec<(u32, Given)> = g
        .all_orders()
        .into_iter()
        .filter(|(_, o)| matches!(o, Given::LoadTo(..)))
        .collect();
    assert_eq!(loads, vec![(4, Given::LoadTo(1, n(85)))]);
}

// ---------------------------------------------------------------- The properties

#[test]
fn a_base_with_nothing_cannot_be_the_source() {
    let g = play(&changed(&text(), "supply_t = 1000", "supply_t = 0"));
    let day_1 = g.reasons(1);
    assert_says(
        &day_1,
        &[
            "base:1 (Placeholder base A): nothing above its reserve, so it cannot be the source",
            "so the staff sets no forces limit on",
            "and it forms no supply",
        ],
    );
    for day in 1..=8 {
        assert_eq!(g.staff_limit(day, 3), None, "day {day}");
    }
    // No "keep supplied" is formed.
    assert!(!g
        .forms()
        .iter()
        .any(|(_, _, k)| matches!(k, IntentKind::KeepSupplied { .. })));
    // The lift is routed to B and back to base 4, as before.
    assert!(
        g.orders(1).contains(&Given::Route(1, vec![B, C], vec![B])),
        "{:?}",
        g.orders(1)
    );
}

#[test]
fn not_the_fuel_to_go_on() {
    // Base 1's supply 0, base 4's 1,000 t, the home port base 1 and 26 t of
    // fuel a ship: the source is base 4.
    let t = changed(&text(), "supply_t = 1000", "supply_t = 0");
    let t = replace_base_4_supply(&t);
    let t = changed(&t, "home_port = 4", "home_port = 1");
    let t = t.replace("fuel_t = 1000", "fuel_t = 26");
    let g = play(&t);
    assert_eq!(g.staff_limit(1, 3).map(|(_, _, s)| s), Some(Some(4)));
    // The lift is routed to B and back to base 1, as before.
    assert!(g.orders(1).contains(&route(1)), "{:?}", g.orders(1));
    // The route's reason says it does not go on to base 4 and gives each
    // ship's fuel against the burn to B (8 t) and on, more than the 18 t
    // left.
    let reasons = g.reasons(1);
    assert_says(
        &reasons,
        &[
            "It does not go on to base:4 (Placeholder staging C)",
            "not every ship has the fuel",
            "ship:1 has 26 t against 8 t to (4,0) and ",
            "ship:2 has 26 t against 8 t to (4,0) and ",
            "so it goes back to base:1, where it came from",
        ],
    );
    let on = reasons
        .split("ship:1 has 26 t against 8 t to (4,0) and ")
        .nth(1)
        .and_then(|rest| rest.split(" t on to base:4").next())
        .expect("the burn on");
    let on: Fixed = on.parse().expect("a figure");
    assert!(on > n(18), "the burn on to base 4 is {on}");
    eprintln!("the burn on from B to base 4: {on} t");
}

/// Base 4's supply set to 1,000 t.
fn replace_base_4_supply(text: &str) -> String {
    let at = text.find("id = 4\n").expect("base 4");
    let (head, tail) = text.split_at(at);
    format!(
        "{head}{}",
        tail.replacen("supply_t = 0", "supply_t = 1000", 1)
    )
}

#[test]
fn the_commanders_source() {
    let g = play(&with(&command_table(
        1,
        "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\nsource = 4",
    )));
    assert_eq!(g.staff_limit(1, 3).map(|(_, _, s)| s), Some(Some(4)));
    // The lift is routed to B and back to base 4, as before.
    assert!(
        g.orders(1).contains(&Given::Route(1, vec![B, C], vec![B])),
        "{:?}",
        g.orders(1)
    );
}

#[test]
fn scenario_77_gives_the_orders_it_gave() {
    let file = common::data_dir().join("scenarios/77_forces_with_men_aboard.toml");
    let source = std::fs::read_to_string(&file).expect("scenario 77 reads");
    let scenario = common::from_text_with(&source, &file, &common::frozen()).expect("it loads");
    let g = play_from(scenario, BTreeMap::new());
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
    assert_eq!(g.all_orders().len(), 10);
    g.nothing_refused();
    // Its source is its lift's home, base 1, with task force 3 empty there.
    assert_eq!(g.staff_limit(1, 3), Some((vec![3], vec![], Some(1))));
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
fn saved_on_day_4_and_resumed() {
    let through = main_game();
    let at_4 = rebuilt(&through.handed, 4);
    assert_eq!(at_4.world.day_hashes(), &through.hashes[..4]);
    let first: BTreeMap<u32, StaffDecisions> =
        (1..=4).map(|d| (d, through.handed[&d].clone())).collect();
    let resumed = play_from(at_4, first);
    assert_eq!(resumed.hashes, through.hashes);
    assert_eq!(resumed.handed, through.handed);
}
