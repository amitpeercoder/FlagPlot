//! Scenario 80: a take that waits. The commander asks for an airfield the
//! enemy is believed to hold with 600 men. The staff wants two men for
//! each of them and has one unit of 600 to hand; a second may be used
//! from day 4. It does not sail short: it waits for day 4 and goes with
//! both. Where no day in reach brings enough, it says so once and waits
//! for the commander.
//!
//! Proves decision 0159, rule B (items 4 to 7), step 2 of the opening of
//! stage 4, against a hand calculation. The data is placeholder data: see
//! `data/scenarios/80_a_take_that_waits.toml`. It runs on the frozen step
//! 1 figures with one ground figure of its own, `preparation_per_day`
//! 100, as scenario 69 does. The enemy is believed to be at B and is not:
//! no Japanese man is ashore anywhere, so no die is rolled.
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
//! As scenarios 67 and 69. A force with its roster in port gains 100 of
//! loading in a day and a unit ready on it 100 of rehearsal. A man weighs
//! 0.25 t to load or land and eats 0.005 t a day, paid at the start of
//! the night pulse by his side's bases in his hex in base order. Supply
//! landed goes into the side's lowest-numbered base in the hex. A size 5
//! port works at 150 t an hour; a size 0 port by lighter at 20 t an hour
//! for each ship. Day d begins at hour 24 x (d - 1): a night pulse of 12
//! hours and two daylight pulses of 6. In a pulse ships move before cargo
//! is worked. A base that changes side does so at the end of the day.
//!
//! The staff's figures: for each man of the estimate it wants 3 at
//! preserve, 2 at balanced and 1 at press; a take with no supply of its
//! own lands 15 days of supply; it looks 7 days ahead for a day that
//! brings enough, where the commander gave no latest day.
//!
//! ## Inputs
//!
//! The placeholder map of scenario 27: A at (0,0), port size 5, and B at
//! (4,0), port size 0, 160 nm apart: 16 hours at 10 kn. The game is 12
//! days.
//!
//! | Base | Side | Hex | Supply | Fuel |
//! | ---- | ---- | --- | ------ | ---- |
//! | 1, "Placeholder base A" | Allied | A | 1,000 t | 0 |
//! | 2, "Placeholder airfield B" | Japanese | B | 0 | 0 |
//!
//! Base 2 has no gun, no aircraft and no man, and is on the Allied map.
//! The Allied side begins with an estimate, told, of 600 enemy men at B.
//! It has no air group, so nothing can look.
//!
//! Task force 1, Allied, on the transport mission: ships 1 and 2,
//! placeholder troop transports (600 men, 3,000 t of cargo, 10 kn, 0.1 t
//! of fuel a mile, 1,000 t of fuel), stopped at base 1, full of fuel,
//! nothing aboard.
//!
//! Ashore at A, fighting units on the defence, experience 2, every skill
//! 2, fed in full:
//!
//! | Unit | Men | May be used |
//! | ---- | --- | ----------- |
//! | 1, "Placeholder First" | 600 | from day 1 |
//! | 2, "Placeholder Second" | 600 | from day 4 |
//!
//! The Allied side's command of day 1: intent 1, take the place at base
//! 2, with a forces limit of task force 1, units 1 and 2 and base 1 as
//! the source; no risk, so balanced; no latest day. Leave to prepare and
//! leave to launch are both given.
//!
//! ## The wait
//!
//! The staff wants 600 x 2 = 1,200 men. On days 1, 2 and 3 it may use
//! unit 1 alone: 600. Unit 2 may be used from day 4, which is within 7
//! days of each of those mornings, and 600 + 600 = 1,200. So on each of
//! days 1, 2 and 3 the take forms nothing and gives no order, and its
//! reason says that it wants 1,200 men, has 600 to hand, that unit 2 may
//! be used from day 4, and that it waits for day 4 (rule B, item 6). It
//! hands in no notice. On day 4 it does what a take does.
//!
//! ## Day by day
//!
//! | Day | Hour | The staff's commands and orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 to 3 | | none; the reason above | |
//! | 4 | 72 | Approves plan 1 to preparing. Forms intent 2, ready a landing by task force 1 at B, with a forces limit of units 1 and 2. Orders `prepare_landing` at B, roster 1 and 2 | Loading 100; each unit's rehearsal 100 |
//! | 5 | 96 | Notice: intent 2 is ready. Approves plan 1 to released. Orders a load of supply to a weight of 90 t | Ship 1 loads 90 t |
//! | 6 | 120 | Unit 1 set to attack. Load unit 1 | Ship 1 takes 600 men |
//! | 7 | 144 | Unit 2 set to attack. Load unit 2 | Ship 2 takes 600 men |
//! | 8 | 168 | A route to B and back to A, acting at B, and an unload order | Sails at hour 168, arrives at hour 184. Each ship lands 160 men in the morning and 440 in the afternoon; ship 1 then lands 10 t. Base 3, "Landing at (4,0)", is formed. Base 2 changes side at the end of the day |
//! | 9 | 192 | Marks plan 1 complete. Notice: intent 1 is met | Ship 1 lands 80 t in 4 hours: complete at hour 196. The force steams for A at hour 204 |
//! | 10 | 216 | none | The force arrives at A at hour 220 |
//! | 11, 12 | | none | |
//!
//! Eight orders in all, none before day 4. No staff order or command is
//! refused. The supply is 1,200 x 0.005 x 15 = 90 t.
//!
//! ## Supply, fuel and men
//!
//! - **Base 1** feeds 1,200 men, 6 t, on the nights of days 1 to 6, and
//!   600 men, 3 t, on the night of day 7: 39 t. It loads out 90 t on day
//!   5. It holds 880 t at the end of day 5 and 871 t at the end.
//! - **Base 3** takes 10 t on the afternoon of day 8. On the night of
//!   day 9 base 2, now Allied and the lower numbered, holds nothing, and
//!   base 3 pays 6 t: 4 t, which it keeps to the end.
//! - **Base 2** takes the 80 t landed on the night of day 9, after that
//!   night's charge. It pays 6 t on the nights of days 10, 11 and 12:
//!   62 t at the end. It is Allied from the end of day 8.
//! - **Each ship** steams 320 nm and burns 32 t: 968 t at the end.
//! - Both units land with a rehearsal of 100 and gain no fatigue from the
//!   landing. Each has 600 men at the end.
//!
//! # Properties
//!
//! - **Enough to hand.** With the estimate 300 men: the staff wants 600
//!   and has them. On day 1 it orders `prepare_landing` at B with roster
//!   1 alone, as before this decision, and no reason speaks of waiting.
//!   Only day 1 is asserted.
//! - **At press it goes.** With the risk of intent 1 set to press: the
//!   staff wants 600 x 1 = 600. On day 1 it orders `prepare_landing` at B
//!   with roster 1 alone. Only day 1 is asserted.
//! - **A latest day that waiting can meet.** With intent 1 given a latest
//!   day of 8: a take begun on day 4 lands on day 8. The game is the main
//!   case, order for order, and no notice that it is late is handed in.
//! - **A latest day that waiting cannot meet.** With a latest day of 7:
//!   no day in reach brings 1,200 men. On day 1 the staff hands in one
//!   notice, that it cannot bring enough: 1,200 wanted, 600 to hand, unit
//!   2 from day 4, which is too late for day 7. It gives no order on days
//!   1 and 2 and hands in no second notice on day 2. With the commander's
//!   command of day 3, risk press on intent 1: on day 3 the staff orders
//!   `prepare_landing` at B with roster 1 alone. Days 1 to 3 are
//!   asserted.
//! - **Beyond the horizon.** With unit 2 usable from day 9 and no latest
//!   day: on day 1, day 9 is more than 7 days off, and the staff hands in
//!   the notice that it cannot bring enough and gives no order. On day 2
//!   day 9 is within 7 days: its reason says that it waits for day 9, and
//!   it hands in no notice. Days 1 and 2 are asserted.
//! - **Nothing coming.** With unit 2 left out of the scenario: the notice
//!   on day 1, once, and no order to task force 1 or unit 1 on any day of
//!   the game.
//! - **The other side.** The Japanese side, given nothing, is given no
//!   order by its staff on any day.
//! - The same start gives the same orders; a save on day 3 and a resume
//!   give the same game.

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::command::{
    CommandKind, IntentKind, InterruptRef, Limit, NoticeKind, PlanOutcome, PlanState, Reason,
    StaffDecisions, StaffOrder, Staffs, StandingKind,
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

const A: Hex = Hex { q: 0, r: 0 };
const B: Hex = Hex { q: 4, r: 0 };

fn path() -> PathBuf {
    common::data_dir().join("scenarios/80_a_take_that_waits.toml")
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

/// An order of the staff, plainly.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Given {
    Landing(u32, Hex, Vec<u32>),
    LoadTo(Fixed),
    Attack(u32),
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

/// Whether an order is given to task force 1 or to land unit 1.
fn to_force_or_unit_1(o: &StaffOrder) -> bool {
    match &o.kind {
        OrderKind::TaskForce { target, .. } => *target == TaskForceId::new(1),
        OrderKind::LandUnit { target, .. } => *target == unit(1),
        _ => false,
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

    fn notices(&self, day: u32) -> Vec<NoticeKind> {
        self.handed[&day].notices.iter().map(|nt| nt.kind).collect()
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

    /// Every reason of `day`, of orders, commands and no orders.
    fn every_reason(&self, day: u32) -> Vec<String> {
        let d = &self.handed[&day];
        d.orders
            .iter()
            .map(|o| text_of(&o.reason))
            .chain(d.commands.iter().map(|c| text_of(&c.reason)))
            .chain(d.no_orders.iter().map(text_of))
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

    /// The days on which the Allied standing interrupt "cannot bring
    /// enough" fired.
    fn cannot_bring_enough_fired(&self) -> Vec<u32> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::InterruptFired {
                    side: Side::Allied,
                    interrupt:
                        InterruptRef::Standing {
                            kind: StandingKind::CannotBringEnough,
                            ..
                        },
                    ..
                } => Some(e.day),
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

fn risk_on(day: u32, level: &str) -> String {
    format!(
        "[[command]]\nday = {day}\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"{level}\"\n"
    )
}

fn by_day(t: &str, day: u32) -> String {
    changed(
        t,
        "intent_kind = \"take_place\"\nhex = [4, 0]\n",
        &format!("intent_kind = \"take_place\"\nhex = [4, 0]\nby_day = {day}\n"),
    )
}

fn estimate(t: &str, men: u32) -> String {
    changed(
        t,
        "q = 4\nr = 0\nmen = 600\n",
        &format!("q = 4\nr = 0\nmen = {men}\n"),
    )
}

fn unit_2_from(t: &str, day: u32) -> String {
    changed(
        t,
        "available_from = 4\n",
        &format!("available_from = {day}\n"),
    )
}

const UNIT_2: &str = "# Unit 2\n[[land_unit]]\nid = 2\nname = \"Placeholder Second\"\nside = \"allied\"\nmen = 600\nq = 0\nr = 0\n# Unit 2 may be used from day 4 (decision 0140).\navailable_from = 4\n";

fn main_game() -> &'static Played {
    static GAME: std::sync::OnceLock<Played> = std::sync::OnceLock::new();
    GAME.get_or_init(|| play(&text()))
}

fn main_orders() -> Vec<(u32, Given)> {
    vec![
        (4, Given::Landing(1, B, vec![1, 2])),
        (5, Given::LoadTo(n(90))),
        (6, Given::Attack(1)),
        (6, Given::LoadUnit(1)),
        (7, Given::Attack(2)),
        (7, Given::LoadUnit(2)),
        (8, route()),
        (8, Given::Unload),
    ]
}

/// The reason of a waiting morning says the men wanted, the men to hand,
/// the unit that comes and its day, and that the take waits for it.
fn assert_waits(g: &Played, day: u32, for_day: u32) {
    let reasons = g.no_orders(day, 1);
    assert_eq!(reasons.len(), 1, "day {day}: {reasons:?}");
    assert_says(
        &reasons[0],
        &[
            "The staff wants 1200 men and has 600 to hand",
            &format!("land_unit:2 (Placeholder Second) may be used from day {for_day}"),
            &format!("on day {for_day} the lift gives 1200 men"),
            &format!("it waits for day {for_day}"),
            "forms nothing and gives no order under intent:1",
        ],
    );
    assert!(
        !reasons[0].contains("goes with what it has"),
        "day {day}: {reasons:?}"
    );
}

// ---------------------------------------------------------------- The hand calculation

#[test]
fn eight_orders_in_all_none_before_day_4() {
    let g = main_game();
    assert_eq!(g.days(), 12);
    assert_eq!(g.all_orders(), main_orders());
    assert!(g.refused().is_empty(), "{:?}", g.refused());
}

#[test]
fn the_take_waits_on_days_1_to_3() {
    let g = main_game();
    for day in 1..=3 {
        assert!(g.orders(day).is_empty(), "day {day}");
        assert!(g.commands(day).is_empty(), "day {day}");
        assert!(g.notices(day).is_empty(), "day {day}");
        assert_waits(g, day, 4);
    }
    // The plan stays a draft while it waits.
    assert_eq!(g.plan_day(1, PlanState::Preparing), Some(4));
}

#[test]
fn the_commands_by_day() {
    let g = main_game();
    assert_eq!(
        g.all_commands(),
        vec![
            (4, Cmd::Approve(1)),
            (4, Cmd::Form(1, landing_kind())),
            (4, Cmd::Limit(2, vec![1, 2])),
            (5, Cmd::Approve(1)),
            (9, Cmd::Complete(1)),
        ]
    );
    assert_eq!(g.plan_day(1, PlanState::Released), Some(5));
    assert_eq!(g.plan_day(1, PlanState::Complete), Some(9));
}

#[test]
fn the_notices() {
    let g = main_game();
    assert_eq!(
        g.all_notices(),
        vec![(5, NoticeKind::Ready, 1), (9, NoticeKind::Met, 1)]
    );
    assert!(g.cannot_bring_enough_fired().is_empty());
}

#[test]
fn the_landing_made_ready_on_day_4() {
    let g = main_game();
    let force = &g.truth[4].task_forces[&TaskForceId::new(1)];
    assert_eq!(
        force.preparation.as_ref().map(|p| (p.hex, p.loading)),
        Some((B, n(100)))
    );
    for u in [1, 2] {
        assert_eq!(g.unit_state(4, u).rehearsal, n(100), "unit {u}");
        assert_eq!(g.unit_state(7, u).rehearsal, n(100), "unit {u}");
    }
}

#[test]
fn the_passage_and_the_landing() {
    let g = main_game();
    assert_eq!(g.arrivals(), vec![(B, n(184)), (A, n(220))]);
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
            (8, Pulse::Morning, 1, 160),
            (8, Pulse::Morning, 2, 160),
            (8, Pulse::Afternoon, 1, 440),
            (8, Pulse::Afternoon, 2, 440),
        ]
    );
    assert_eq!(g.formed_day(3), Some(8));
    assert_eq!(g.changed_side_day(2), Some(8));
    assert_eq!(g.unloading_complete(), vec![(2, n(196))]);
}

#[test]
fn supply_fuel_and_men() {
    let g = main_game();
    let base1 = [994, 988, 982, 976, 880, 874, 871, 871, 871, 871, 871, 871];
    for day in 1..=12u32 {
        assert_eq!(
            g.supply(day, 1),
            n(base1[(day - 1) as usize]),
            "base 1 after day {day}"
        );
    }
    assert_eq!(g.supply(8, 3), n(10));
    for day in 9..=12 {
        assert_eq!(g.supply(day, 3), n(4), "base 3 after day {day}");
    }
    let base2 = [80, 74, 68, 62];
    for day in 9..=12u32 {
        assert_eq!(
            g.supply(day, 2),
            n(base2[(day - 9) as usize]),
            "base 2 after day {day}"
        );
    }
    assert_eq!(g.truth[8].bases[&BaseId::new(2)].side, Side::Allied);
    for s in [1, 2] {
        assert_eq!(g.ship(12, s).fuel_t, n(968), "ship {s}");
    }
    for u in [1, 2] {
        let state = g.unit_state(12, u);
        assert_eq!(state.men_ashore, 600, "unit {u}");
        assert_eq!(state.hex, Some(B), "unit {u}");
        assert_eq!(state.fatigue, Fixed::ZERO, "unit {u}");
    }
}

// ---------------------------------------------------------------- The properties

#[test]
fn enough_to_hand() {
    let g = play(&estimate(&text(), 300));
    assert_eq!(g.orders(1), vec![Given::Landing(1, B, vec![1])]);
    assert!(
        g.every_reason(1)
            .iter()
            .all(|r| !r.contains("waits for day")),
        "{:?}",
        g.every_reason(1)
    );
}

#[test]
fn at_press_it_goes() {
    let g = play(&with_commands(&text(), &risk_on(1, "press")));
    assert_eq!(g.orders(1), vec![Given::Landing(1, B, vec![1])]);
}

#[test]
fn a_latest_day_that_waiting_can_meet() {
    let g = play(&by_day(&text(), 8));
    assert_eq!(g.all_orders(), main_orders());
    assert!(
        !g.all_notices()
            .iter()
            .any(|(_, k, _)| *k == NoticeKind::Late),
        "{:?}",
        g.all_notices()
    );
    for day in 1..=3 {
        assert_waits(&g, day, 4);
    }
}

#[test]
fn a_latest_day_that_waiting_cannot_meet() {
    let t = with_commands(&by_day(&text(), 7), &risk_on(3, "press"));
    let g = play(&t);
    assert_eq!(g.notices(1), vec![NoticeKind::CannotBringEnough]);
    assert_eq!(
        g.handed[&1].notices[0].plan,
        sim_core::command::PlanId::new(1)
    );
    let day1 = g.no_orders(1, 1);
    assert_eq!(day1.len(), 1, "{day1:?}");
    assert_says(
        &day1[0],
        &[
            "The staff wants 1200 men and has 600 to hand",
            "land_unit:2 (Placeholder Second) may be used from day 4, too late to land by day 7",
            "hands in the notice that it cannot bring enough",
        ],
    );
    for day in 1..=2 {
        assert!(g.orders(day).is_empty(), "day {day}");
        assert!(g.commands(day).is_empty(), "day {day}");
    }
    assert!(g.notices(2).is_empty());
    let day2 = g.no_orders(2, 1);
    assert_eq!(day2.len(), 1, "{day2:?}");
    assert_says(&day2[0], &["The staff wants 1200 men and has 600 to hand"]);
    assert!(!day2[0].contains("hands in the notice"), "{day2:?}");
    assert_eq!(g.cannot_bring_enough_fired(), vec![1]);
    assert_eq!(g.orders(3), vec![Given::Landing(1, B, vec![1])]);
}

#[test]
fn beyond_the_horizon() {
    let g = play(&unit_2_from(&text(), 9));
    assert_eq!(g.notices(1), vec![NoticeKind::CannotBringEnough]);
    assert!(g.orders(1).is_empty());
    assert_says(
        &g.no_orders(1, 1)[0],
        &[
            "land_unit:2 (Placeholder Second) may be used from day 9, beyond day 8",
            "No day up to day 8",
        ],
    );
    assert!(g.notices(2).is_empty());
    assert!(g.orders(2).is_empty());
    assert_waits(&g, 2, 9);
}

#[test]
fn nothing_coming() {
    // Unit 2 is left out of the scenario, and so out of the forces limit.
    let t = changed(
        &changed(&text(), UNIT_2, ""),
        "land_units = [1, 2]\n",
        "land_units = [1]\n",
    );
    let g = play(&t);
    assert_eq!(g.all_notices(), vec![(1, NoticeKind::CannotBringEnough, 1)]);
    assert_says(
        &g.no_orders(1, 1)[0],
        &["No unit or force of the side is coming"],
    );
    for day in 1..=g.days() {
        assert!(
            !g.handed[&day].orders.iter().any(to_force_or_unit_1),
            "day {day}"
        );
    }
}

#[test]
fn the_other_side_is_given_nothing() {
    let g = main_game();
    for (day, handed) in &g.japanese {
        assert!(handed.orders.is_empty(), "day {day}");
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

#[test]
fn the_same_start_gives_the_same_orders() {
    let a = play(&text());
    assert_eq!(&a.handed, &main_game().handed);
    assert_eq!(a.hashes, main_game().hashes);
}
