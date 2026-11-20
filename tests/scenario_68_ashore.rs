//! Scenario 68: ashore. The staff lands two units on an airfield that its
//! estimate says 500 men hold. There are 1,200. The first day's fight
//! shows it, the staff stops the attack, rests the men, brings the unit it
//! left behind, and attacks again when the men are enough. The airfield
//! falls.
//!
//! Proves decision 0143, step 4 of stage 3, first part, against a hand
//! calculation. The data is placeholder data: see
//! `data/scenarios/68_ashore.toml`. It runs on the frozen step 1 figures,
//! with ground figures of its own that differ from the frozen ones in
//! eight: `preparation_per_day` 100, `hit_chance` 1, `defender_hit_chance`
//! 1, `men_per_hit` 50, `fatigue_per_attack_day` 50, `rest_per_day` 25,
//! `report_error_low` 1 and `report_error_high` 1. Every shot is a whole
//! shot and hits, and a report gives the true count, so no die changes
//! anything.
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
//! A unit fires one shot for each 100 firing men, and its firing men are
//! its men ashore times its fed share times 100 less its fatigue over
//! 100. Every shot hits and a hit takes 50 men, shared between the units
//! hit in proportion to their men. A unit on attack that fired gains 50
//! of fatigue; any other unit loses 25. A unit on attack whose fatigue is
//! above 60 goes back to the defence. A side is beaten where the other
//! attacked and it has no man left, or fewer than 0.2 of the other's. A
//! base that changes side keeps half its supply. A man weighs 0.25 t to
//! land and eats 0.005 t a day, paid at the start of the night pulse by
//! the side's bases in the hex in base order. A force with its roster in
//! port gains 100 of loading in a day and a unit ready on it 100 of
//! rehearsal. A size 5 port works at 150 t an hour; a size 0 port by
//! lighter at 20 t an hour for each ship. Day d begins at hour
//! 24 x (d - 1): a night pulse of 12 hours and two daylight pulses of 6.
//! In a pulse ships move before cargo is worked. The fight and the
//! reports are at the end of the day.
//!
//! The staff's figures: at balanced it wants 2 men for each man of its
//! estimate, and a unit can attack at a fatigue of 40 or less (two thirds
//! of 60) or if it is on attack already. A unit's fit men are its men
//! ashore times 100 less its fatigue over 100.
//!
//! ## Inputs
//!
//! The placeholder map of scenario 27: A at (0,0), a coast hex with port
//! size 5, and B at (4,0), a coast hex with port size 0, 160 nm apart: 16
//! hours at 10 kn. No land link. The game is 14 days.
//!
//! | Base | Side | Hex | Supply | Fuel |
//! | ---- | ---- | --- | ------ | ---- |
//! | 1, "Placeholder base A" | Allied | A | 1,000 t | 0 |
//! | 2, "Placeholder airfield B" | Japanese | B | 200 t | 0 |
//!
//! Task force 1, Allied, on the transport mission: ships 1 and 2,
//! placeholder troop transports (600 men, 3,000 t of cargo, 10 kn, 0.1 t
//! of fuel a mile, 1,000 t of fuel), stopped at base 1, full of fuel,
//! nothing aboard.
//!
//! | Unit | Side | Where | Men | Experience | Assault skill | Posture |
//! | ---- | ---- | ----- | --- | ---------- | ------------- | ------- |
//! | 1, "Placeholder First" | Allied | A | 600 | 2 | 2 | defend |
//! | 2, "Placeholder Second" | Allied | A | 600 | 2 | 2 | defend |
//! | 3, "Placeholder Third" | Allied | A | 600 | 2 | 2 | defend |
//! | 10, "Placeholder Garrison" | Japanese | B | 1,200 | 2 | | defend |
//!
//! All are fighting units, fed in full, with no fatigue. The Allied side
//! has an opening estimate of 500 enemy men at B. The Japanese side has
//! no staff intent and gives no order.
//!
//! The Allied side's commands of day 1: intent 1, take the place at base
//! 2, with a forces limit of task force 1, units 1, 2 and 3 and base 1 as
//! the source; no risk set, so balanced; no date. Leave to prepare and
//! leave to launch are both given.
//!
//! ## The choice
//!
//! The estimate is 500: the men wanted are 1,000. The units in their
//! order are 1, 2, 3. Unit 1 takes ship 1: 600 men, fewer than 1,000.
//! Unit 2 takes ship 2: 1,200, enough. Unit 3 is left behind. The supply
//! is 1,200 x 0.005 x 15 = 90 t.
//!
//! ## Day by day
//!
//! | Day | Hour | The staff's commands and orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 | 0 | Approves plan 1 to preparing. Forms intent 2, ready a landing by task force 1 at B, and sets its forces limit to units 1 and 2. Orders `prepare_landing` at B, roster 1 and 2 | Loading 100; each unit's rehearsal 100 |
//! | 2 | 24 | Notice: intent 2 is ready. Approves plan 1 to released. Orders a load of supply to a weight of 90 t | Ship 1 loads 90 t |
//! | 3 | 48 | Unit 1 set to attack. Load unit 1 | Ship 1 takes 600 men |
//! | 4 | 72 | Unit 2 set to attack. Load unit 2 | Ship 2 takes 600 men |
//! | 5 | 96 | A route to B and back to A, acting at B, and an unload order | Sails at hour 96, arrives at hour 112. Each ship lands 160 men in the morning and 440 in the afternoon; ship 1 then lands 10 t. Base 3, "Landing at (4,0)", is formed. The first fight |
//! | 6 | 120 | Units 1 and 2 set to defend | Ship 1 lands 80 t in 4 hours. The force steams for A at hour 132. No fight |
//! | 7 | 144 | none | The force arrives at A at hour 148 |
//! | 8 | 168 | Sets the forces limit of intent 2 to units 1, 2 and 3. Orders a load of supply to a weight of 45 t | Ship 1 loads 45 t |
//! | 9 | 192 | Load unit 3 | Ship 1 takes 600 men |
//! | 10 | 216 | A route to B and back to A, acting at B, and an unload order | Sails at hour 216, arrives at hour 232. Ship 1 lands 160 men in the morning, 440 in the afternoon and then 10 t |
//! | 11 | 240 | Units 1, 2 and 3 set to attack | Ship 1 lands 35 t in 1.75 hours. The force steams for A at hour 252. The second fight. Base 2 changes side |
//! | 12 | 264 | Marks plan 1 complete. Notice: intent 1 is met | The force arrives at A at hour 268 |
//! | 13, 14 | | none | |
//!
//! Seventeen orders in all and six staff commands. No staff order is
//! refused.
//!
//! ## The first fight, day 5
//!
//! | | Allied | Japanese |
//! | --- | --- | --- |
//! | Men ashore | 600 and 600 | 1,200 |
//! | Firing men | 600 and 600 | 1,200 |
//! | Shots, all hits | 6 and 6 | 12 |
//! | Men lost | 12 x 50 = 600: 300 each | 12 x 50 = 600 |
//! | Men left | 300 and 300 | 600 |
//! | Fatigue after | 50 and 50 | 0 |
//!
//! The Japanese side has 600 men against 0.2 x 600 = 120: it is not
//! beaten. The Allied report of that evening gives 600 enemy men at B.
//!
//! ## The judgement, morning by morning
//!
//! | Morning | The estimate | Men wanted | Units that can attack, with their fit men | The staff |
//! | --- | --- | --- | --- | --- |
//! | 6 | 600, of day 5 | 1,200 | 1 and 2, on attack, at fatigue 50: 150 each, 300 | Short by 900: both set to defend |
//! | 7 | 600, of day 6 | 1,200 | 1 and 2 at fatigue 25: 225 each, 450 | Short by 750: waits |
//! | 8 | 600 | 1,200 | 1 and 2 at fatigue 0: 300 each, 600 | Short by 600: waits. The force lies at A: unit 3 is to be sent |
//! | 9, 10 | 600 | 1,200 | 600 | Short by 600: waits |
//! | 11 | 600, of day 10 | 1,200 | 1, 2 and 3 at fatigue 0: 300, 300 and 600: 1,200 | Enough: all three set to attack |
//!
//! On the morning of day 8 the men ashore who can attack are 600 and unit
//! 3 has 600: together 1,200, the men wanted. Its supply is
//! 600 x 0.005 x 15 = 45 t, and it takes ship 1.
//!
//! ## The second fight, day 11
//!
//! | | Allied | Japanese |
//! | --- | --- | --- |
//! | Men ashore | 300, 300 and 600 | 600 |
//! | Shots, all hits | 3, 3 and 6 | 6 |
//! | Men lost | 6 x 50 = 300: 75, 75 and 150 | 12 x 50 = 600: all |
//! | Men left | 225, 225 and 450 | 0 |
//! | Fatigue after | 50, 50 and 50 | |
//!
//! Unit 10 is destroyed. The Japanese side has no man at B and is beaten:
//! base 2 is Allied. The Allied men find the hex empty and the estimate
//! is cleared. On the morning of day 12 the side holds a base at B and
//! the enemy none: the intent is met.
//!
//! ## Supply and fuel
//!
//! - **Base 1** feeds the men ashore at A at the start of each night,
//!   before that pulse's loading: 1,800 men, 9 t, on days 1 to 3; 1,200
//!   men, 6 t, on day 4; 600 men, 3 t, on days 5 to 9; none after. That
//!   is 27 + 6 + 15 = 48 t. It loads out 90 t and 45 t. It ends with
//!   1,000 - 48 - 135 = 817 t.
//! - **Base 2,** while Japanese, feeds 1,200 men, 6 t, on days 1 to 5 and
//!   600 men, 3 t, on days 6 to 11: 200 - 30 - 18 = 152 t. It changes
//!   side with half, 76 t. It then feeds the 900 Allied men, 4.5 t a
//!   night, being the lower numbered: 71.5 t, 67 t and 62.5 t after the
//!   nights of days 12, 13 and 14.
//! - **Base 3** is given 10 t on the afternoon of day 5. On the night of
//!   day 6 it pays 3 t for 600 men and then takes 80 t: 87 t. It pays 3 t
//!   on each of days 7 to 10: 75 t; takes 10 t on the afternoon of day
//!   10: 85 t; pays 6 t for 1,200 men on the night of day 11 and takes
//!   35 t: 114 t. Base 2 pays from day 12, so it ends with 114 t.
//! - **Each ship** steams 320 nm twice and burns 64 t: 936 t at the end.
//!
//! # Properties
//!
//! - **No estimate at the start.** With no opening estimate the staff
//!   takes every unit the ships can carry, units 1 and 2, and says it goes
//!   with no count of the enemy. Every order and every number after day 1
//!   is as in the main case: the report of day 5 gives the estimate.
//! - **A garrison the estimate was right about.** With unit 10 at 400
//!   men: on day 5 the twelve hits take all 400, the four shots of the
//!   garrison take 200, and units 1 and 2 are left with 500 each. Base 2
//!   changes side on day 5 with (200 - 2 x 5) / 2 = 95 t. The plan is
//!   marked complete and the notice handed in on the morning of day 6.
//!   Eight orders and five staff commands; unit 3 is never sent.
//! - **Nothing left to send.** With unit 3 held from day 1: the first six
//!   days are as in the main case. From the morning of day 6 the staff
//!   waits, says each morning that it is short and has no unit to bring,
//!   and gives no order after day 6: ten orders in all. The place is not
//!   taken by the end. The force lies at A from hour 148.
//! - **A hold is honoured.** With unit 2 held from day 7. (Restated by
//!   the architect on Nov 19, 2026, for decision 0169: before it unit 3
//!   was sent as in the main case, for fourteen orders in all, and on day
//!   11 the staff waited with 900 fit men, short by 300.) On the morning
//!   of day 8 the second lift, task force 1 with unit 3's 600 men, would
//!   leave the operation with 300 + 600 = 900 fit men against the 1,200
//!   wanted, no other force is in reach by day 15, and no hold of B with
//!   a source of supply is in force, so the men cannot be fed: at
//!   balanced the staff does not send it. It hands in "cannot bring
//!   enough" on day 8, once, and gives no order from day 7 on. The
//!   notices of the game are "ready" on day 2 and that one. On the
//!   morning of day 11 the unit that can attack is unit 1, 300 fit men
//!   against 1,200 wanted: the staff waits, short by 900, and names the
//!   hold. Ten orders in all, the main case's first ten, and the place
//!   is not taken.
//! - **Scenario 67 is not changed:** it gives the orders and the numbers
//!   it gave.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see (base 2 with 300 t of supply in one)
//!   give the same orders and reasons; a save on day 8 and a resume give
//!   the same game; every scenario built before this one gives the events
//!   it gave.

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
    common::data_dir().join("scenarios/68_ashore.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

/// A number in hundredths: c(24175) is 241.75.
fn c(hundredths: i64) -> Fixed {
    Fixed::from_ratio(hundredths, 100)
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
/// `data/reference/ground_placeholder_scenario_68.toml`, frozen beside the
/// others: the figures its hand calculation names.
fn figures() -> sim_data::LoadOptions {
    common::frozen()
}

fn load(text: &str) -> Scenario {
    common::from_text_with(text, &path(), &figures()).expect("the scenario loads")
}

/// A game played to its end with the doctrine's staffs: what the Allied staff
/// handed in each day, the truth at the end of each day (index 0 is the
/// start), the log and the hash of each day.
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

/// A staff command, plainly.
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

    /// Every notice of the game, by day, in the order handed in.
    fn all_notices(&self) -> Vec<(u32, NoticeKind)> {
        (1..=self.days())
            .flat_map(|d| self.handed[&d].notices.iter().map(move |nt| (d, nt.kind)))
            .collect()
    }

    /// The no-order reasons of `day` serving `of`.
    fn no_orders(&self, day: u32, of: u32) -> Vec<&Reason> {
        self.handed[&day]
            .no_orders
            .iter()
            .filter(|r| r.serves.intent == Some(intent(of)))
            .collect()
    }

    /// The reason of the morning's judgement: the reason of its first
    /// posture order, or else its reason for giving no order to the men.
    fn judgement(&self, day: u32) -> String {
        let posture = self.handed[&day].orders.iter().find(|o| {
            matches!(
                o.kind,
                OrderKind::LandUnit {
                    order: LandUnitOrder::SetPosture { .. },
                    ..
                }
            )
        });
        match posture {
            Some(order) => text_of(&order.reason),
            None => self
                .no_orders(day, 1)
                .into_iter()
                .map(text_of)
                .find(|t| t.contains("the units ashore"))
                .unwrap_or_else(|| panic!("day {day} has a judgement")),
        }
    }

    fn unit_state(&self, day: u32, u: u32) -> &sim_core::state::LandUnit {
        &self.truth[day as usize].land_units[&unit(u)]
    }

    fn base(&self, day: u32, b: u32) -> Option<&sim_core::state::Base> {
        self.truth[day as usize].bases.get(&BaseId::new(b))
    }

    fn base_side(&self, day: u32, b: u32) -> Side {
        self.base(day, b).expect("the base exists").side
    }

    fn supply(&self, day: u32, b: u32) -> Fixed {
        self.base(day, b).expect("the base exists").supply_t
    }

    fn ship(&self, day: u32, s: u32) -> &sim_core::state::Ship {
        &self.truth[day as usize].ships[&ShipId::new(s)]
    }

    fn plan_day(&self, to: PlanState) -> Option<u32> {
        self.events.iter().find_map(|e| match &e.kind {
            EventKind::PlanStateChanged {
                side, plan, to: t, ..
            } if *side == Side::Allied && plan.raw() == 1 && *t == to => Some(e.day),
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

    /// The Allied ground reports of B: (day, men).
    fn reports(&self) -> Vec<(u32, u32)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::GroundReportReceived {
                    side: Side::Allied,
                    hex,
                    men,
                    ..
                } if *hex == B => Some((e.day, *men)),
                _ => None,
            })
            .collect()
    }

    fn fight_days(&self) -> Vec<u32> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::GroundFight { hex, .. } if *hex == B => Some(e.day),
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

/// The text with `extra` added after the commander's commands.
fn with_commands(t: &str, extra: &str) -> String {
    format!("{t}\n{extra}")
}

fn hold_unit(day: u32, u: u32) -> String {
    format!("[[command]]\nday = {day}\nside = \"allied\"\nkind = \"hold\"\nland_unit = {u}\n")
}

fn main_game() -> &'static Played {
    static GAME: std::sync::OnceLock<Played> = std::sync::OnceLock::new();
    GAME.get_or_init(|| play(&text()))
}

fn main_orders() -> Vec<(u32, Given)> {
    vec![
        (1, Given::Landing(1, B, vec![1, 2])),
        (2, Given::LoadTo(n(90))),
        (3, Given::Attack(1)),
        (3, Given::LoadUnit(1)),
        (4, Given::Attack(2)),
        (4, Given::LoadUnit(2)),
        (5, route()),
        (5, Given::Unload),
        (6, Given::Defend(1)),
        (6, Given::Defend(2)),
        (8, Given::LoadTo(n(45))),
        (9, Given::LoadUnit(3)),
        (10, route()),
        (10, Given::Unload),
        (11, Given::Attack(1)),
        (11, Given::Attack(2)),
        (11, Given::Attack(3)),
    ]
}

// ---------------------------------------------------------------- The hand calculation

#[test]
fn the_choice_on_day_1() {
    let g = main_game();
    assert_eq!(g.days(), 14);
    assert_eq!(
        g.commands(1),
        vec![
            Cmd::Approve(1),
            Cmd::Form(1, landing_kind()),
            Cmd::Limit(2, vec![1, 2]),
        ]
    );
    let limit = text_of(g.command_reason(1, &Cmd::Limit(2, vec![1, 2])));
    assert_says(
        &limit,
        &[
            "500 enemy men",
            "1000 men",
            "land_unit:3 (Placeholder Third) is not needed",
            "90 t",
        ],
    );
}

#[test]
fn seventeen_orders_and_six_staff_commands() {
    let g = main_game();
    assert_eq!(g.all_orders(), main_orders());
    assert_eq!(
        g.all_commands(),
        vec![
            (1, Cmd::Approve(1)),
            (1, Cmd::Form(1, landing_kind())),
            (1, Cmd::Limit(2, vec![1, 2])),
            (2, Cmd::Approve(1)),
            (8, Cmd::Limit(2, vec![1, 2, 3])),
            (12, Cmd::Complete(1)),
        ]
    );
    assert!(g.refused().is_empty(), "{:?}", g.refused());
    assert_eq!(g.plan_day(PlanState::Preparing), Some(1));
    assert_eq!(g.plan_day(PlanState::Released), Some(2));
    assert_eq!(g.plan_day(PlanState::Complete), Some(12));
}

#[test]
fn the_notices() {
    let g = main_game();
    assert_eq!(
        g.all_notices(),
        vec![(2, NoticeKind::Ready), (12, NoticeKind::Met)]
    );
    // The notice that the place is taken says the intent's words (the ruling
    // on decision 0142's reading 4).
    let met = g
        .events
        .iter()
        .find(|e| {
            matches!(
                e.kind,
                EventKind::StaffNotice {
                    kind: NoticeKind::Met,
                    ..
                }
            )
        })
        .expect("a met notice");
    assert!(
        met.kind.to_string().ends_with("intent:1 is met"),
        "{}",
        met.kind
    );
}

#[test]
fn the_passages_and_the_landings() {
    let g = main_game();
    assert_eq!(
        g.arrivals(),
        vec![(B, n(112)), (A, n(148)), (B, n(232)), (A, n(268))]
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
            (10, Pulse::Morning, 3, 160),
            (10, Pulse::Afternoon, 3, 440),
        ]
    );
    let formed = g.base(5, 3).expect("base 3 is formed on day 5");
    assert_eq!(formed.name, "Landing at (4,0)");
    assert_eq!(formed.side, Side::Allied);
    assert!(g.base(4, 3).is_none());
    // Ship 1 lands 80 t in 4 hours of the night of day 6, and 35 t in 1.75
    // hours of the night of day 11.
    assert_eq!(g.unloading_complete(), vec![(3, n(124)), (3, c(24175))]);
    // Unit 3 takes ship 1 on day 9, with the 45 t loaded on day 8.
    assert_eq!(g.ship(8, 1).supply_t, n(45));
    assert_eq!(
        g.ship(9, 1).troops.map(|t| (t.unit.raw(), t.men)),
        Some((3, 600))
    );
    assert_eq!(g.ship(9, 2).troops, None);
    // A unit of the second lift is not set to attack before it sails.
    assert_eq!(g.unit_state(9, 3).posture, Posture::Defend);
    assert_eq!(g.unit_state(10, 3).posture, Posture::Defend);
    for s in [1, 2] {
        assert_eq!(g.ship(14, s).fuel_t, n(936));
    }
}

#[test]
fn the_first_fight_day_5() {
    let g = main_game();
    assert_eq!(g.fight_days(), vec![5, 11]);
    for u in [1, 2] {
        let s = g.unit_state(5, u);
        assert_eq!((s.hex, s.men_ashore), (Some(B), 300), "unit {u}");
        assert_eq!(s.fatigue, n(50));
        assert_eq!(s.posture, Posture::Attack);
    }
    let garrison = g.unit_state(5, 10);
    assert_eq!(garrison.men_ashore, 600);
    assert_eq!(garrison.fatigue, Fixed::ZERO);
    assert_eq!(g.base_side(5, 2), Side::Japanese);
    // The Allied report of that evening gives 600 enemy men at B.
    assert_eq!(g.reports().first(), Some(&(0, 500)), "the opening estimate");
    assert!(g.reports().contains(&(5, 600)), "{:?}", g.reports());
}

#[test]
fn the_judgement_morning_by_morning() {
    let g = main_game();
    // Morning 6: short by 900, both set to defend.
    assert_says(
        &g.judgement(6),
        &[
            "600 enemy men at (4,0), of day 5",
            "1200 men",
            "land_unit:1 (Placeholder First): 300 men ashore, on attack, fatigue 50 against the mark of 40, 150 fit men, can attack",
            "land_unit:2 (Placeholder Second): 300 men ashore, on attack, fatigue 50 against the mark of 40, 150 fit men, can attack",
            "have 300 fit men against the 1200 wanted, short by 900",
            "to defend",
        ],
    );
    assert_eq!(g.orders(6), vec![Given::Defend(1), Given::Defend(2)]);
    // Morning 7: fatigue 25, short by 750; waits.
    assert!(g.orders(7).is_empty());
    assert_says(
        &g.judgement(7),
        &[
            "of day 6",
            "fatigue 25 against the mark of 40, 225 fit men, can attack",
            "have 450 fit men against the 1200 wanted, short by 750",
        ],
    );
    // Morning 8: fatigue 0, short by 600; unit 3 is to be sent.
    assert_says(
        &g.judgement(8),
        &[
            "fatigue 0 against the mark of 40, 300 fit men",
            "have 600 fit men against the 1200 wanted, short by 600",
            "land_unit:3 (Placeholder Third) (600 men)",
        ],
    );
    for day in [9, 10] {
        assert_says(
            &g.judgement(day),
            &["have 600 fit men against the 1200 wanted, short by 600"],
        );
    }
    // Morning 11: 300, 300 and 600: enough.
    assert_says(
        &g.judgement(11),
        &[
            "of day 10",
            "land_unit:3 (Placeholder Third): 600 men ashore, on defend, fatigue 0 against the mark of 40, 600 fit men, can attack",
            "have 1200 fit men, at least the 1200 wanted",
        ],
    );
    assert_eq!(
        g.orders(11),
        vec![Given::Attack(1), Given::Attack(2), Given::Attack(3)]
    );
    // The rest between: fatigue 25 after day 6 and 0 after day 7.
    for u in [1, 2] {
        assert_eq!(g.unit_state(6, u).fatigue, n(25));
        assert_eq!(g.unit_state(7, u).fatigue, Fixed::ZERO);
        assert_eq!(g.unit_state(6, u).posture, Posture::Defend);
    }
}

#[test]
fn the_second_lift_is_decided_on_day_8() {
    let g = main_game();
    let limit = text_of(g.command_reason(8, &Cmd::Limit(2, vec![1, 2, 3])));
    assert_says(
        &limit,
        &[
            "land_unit:3 (Placeholder Third) (600 men) takes 1 ship",
            "600 men chosen, 3 t a day, for 15 days = 45 t",
        ],
    );
    assert_says(
        &text_of(g.order_reason(10, &route())),
        &["It goes as it stands, with no landing made ready"],
    );
}

#[test]
fn the_second_fight_day_11() {
    let g = main_game();
    let men: Vec<u32> = [1, 2, 3]
        .iter()
        .map(|u| g.unit_state(11, *u).men_ashore)
        .collect();
    assert_eq!(men, vec![225, 225, 450]);
    for u in [1, 2, 3] {
        assert_eq!(g.unit_state(11, u).fatigue, n(50), "unit {u}");
    }
    let destroyed = g
        .events
        .iter()
        .any(|e| matches!(&e.kind, EventKind::LandUnitDestroyed { unit } if unit.raw() == 10));
    assert!(destroyed, "unit 10 is destroyed");
    assert_eq!(g.changed_side_day(2), Some(11));
    assert_eq!(g.base_side(11, 2), Side::Allied);
}

#[test]
fn supply_and_fuel() {
    let g = main_game();
    let base1 = [
        991, 892, 883, 877, 874, 871, 868, 820, 817, 817, 817, 817, 817, 817,
    ];
    for day in 1..=14u32 {
        assert_eq!(
            g.supply(day, 1),
            n(base1[(day - 1) as usize]),
            "base 1 after day {day}"
        );
    }
    // Base 2 while Japanese: 6 t a night on days 1 to 5 and 3 t on days 6
    // to 11; then half, 76 t; then 4.5 t a night for the 900 Allied men.
    let base2 = [194, 188, 182, 176, 170, 167, 164, 161, 158, 155];
    for day in 1..=10u32 {
        assert_eq!(
            g.supply(day, 2),
            n(base2[(day - 1) as usize]),
            "base 2 after day {day}"
        );
    }
    assert_eq!(g.supply(11, 2), n(76));
    assert_eq!(g.supply(12, 2), c(7150));
    assert_eq!(g.supply(13, 2), n(67));
    assert_eq!(g.supply(14, 2), c(6250));
    // Base 3: 10, 87, 84, 81, 78, 85, 114, and 114 to the end.
    let base3 = [10, 87, 84, 81, 78, 85, 114, 114, 114, 114];
    for day in 5..=14u32 {
        assert_eq!(
            g.supply(day, 3),
            n(base3[(day - 5) as usize]),
            "base 3 after day {day}"
        );
    }
}

// ---------------------------------------------------------------- The properties

#[test]
fn no_estimate_at_the_start() {
    let estimate = "[[estimate]]\nside = \"allied\"\nq = 4\nr = 0\nmen = 500\n";
    let g = play(&changed(&text(), estimate, ""));
    assert_says(
        &text_of(g.command_reason(1, &Cmd::Limit(2, vec![1, 2]))),
        &[
            "goes with no count of the enemy",
            "land_unit:3 (Placeholder Third) (600 men) is left out: no ship is left for it",
        ],
    );
    assert_eq!(g.all_orders(), main_orders());
    assert_eq!(g.all_commands(), main_game().all_commands());
    assert_eq!(g.supply(14, 1), n(817));
    assert_eq!(g.supply(14, 3), n(114));
}

#[test]
fn a_garrison_the_estimate_was_right_about() {
    let t = changed(
        &text(),
        "name = \"Placeholder Garrison\"\nside = \"japanese\"\nmen = 1200\n",
        "name = \"Placeholder Garrison\"\nside = \"japanese\"\nmen = 400\n",
    );
    let g = play(&t);
    for u in [1, 2] {
        assert_eq!(g.unit_state(5, u).men_ashore, 500, "unit {u}");
    }
    assert_eq!(g.changed_side_day(2), Some(5));
    assert_eq!(g.supply(5, 2), n(95));
    assert_eq!(g.plan_day(PlanState::Complete), Some(6));
    assert!(g.all_notices().contains(&(6, NoticeKind::Met)));
    assert_eq!(g.all_orders(), main_orders()[..8].to_vec());
    assert_eq!(
        g.all_commands(),
        vec![
            (1, Cmd::Approve(1)),
            (1, Cmd::Form(1, landing_kind())),
            (1, Cmd::Limit(2, vec![1, 2])),
            (2, Cmd::Approve(1)),
            (6, Cmd::Complete(1)),
        ]
    );
    assert_eq!(g.unit_state(14, 3).hex, Some(A));
}

#[test]
#[ignore = "decision 0193 changes it: with unit 3 held nothing more is coming, so on the morning of day 8 units 1 and 2 (600 fit men) are at least the estimate of 600 and are set to attack at balanced (orders (8, Attack(1)) and (8, Attack(2)) after the ten expected); the architect's to restate"]
fn nothing_left_to_send() {
    let g = play(&with_commands(&text(), &hold_unit(1, 3)));
    assert_eq!(g.all_orders(), main_orders()[..10].to_vec());
    for day in 7..=14 {
        assert_says(
            &g.judgement(day),
            &["short by", "no unit is left behind to bring"],
        );
    }
    assert_eq!(g.plan_day(PlanState::Complete), None);
    assert_eq!(g.base_side(14, 2), Side::Japanese);
    assert_eq!(g.arrivals().last(), Some(&(A, n(148))));
}

#[test]
#[ignore = "decision 0193, rule 11, changes it: with unit 2 held from day 7 nothing more is coming and the men are too few, so `cannot_bring_enough` is handed in on each morning from day 8 to day 14, not on day 8 alone; the architect's to restate"]
fn a_hold_is_honoured() {
    let g = play(&with_commands(&text(), &hold_unit(7, 2)));
    // Restated for decision 0169: the second lift of day 8 is not sent.
    let wanted: Vec<(u32, Given)> = main_orders()[..10].to_vec();
    assert_eq!(g.all_orders(), wanted);
    assert!((7..=g.days()).all(|day| g.orders(day).is_empty()));
    assert_eq!(
        g.all_notices(),
        vec![(2, NoticeKind::Ready), (8, NoticeKind::CannotBringEnough)]
    );
    assert_says(
        &g.judgement(11),
        &[
            "have 300 fit men against the 1200 wanted, short by 900",
            "land_unit:2 (Placeholder Second) (300 men ashore) is left out: land_unit:2 is held",
        ],
    );
    assert_eq!(g.plan_day(PlanState::Complete), None);
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
    let more = changed(
        &text(),
        "name = \"Placeholder airfield B\"\nside = \"japanese\"\nq = 4\nr = 0\nsupply_t = 200\n",
        "name = \"Placeholder airfield B\"\nside = \"japanese\"\nq = 4\nr = 0\nsupply_t = 300\n",
    );
    let a = play(&more);
    let reasons = |g: &Played| -> Vec<(u32, Vec<Given>, Vec<String>)> {
        (1..=14)
            .map(|d| {
                (
                    d,
                    g.orders(d),
                    g.handed[&d]
                        .orders
                        .iter()
                        .map(|o| o.reason.worked.clone())
                        .chain(g.handed[&d].no_orders.iter().map(|r| r.worked.clone()))
                        .collect(),
                )
            })
            .collect()
    };
    assert_eq!(reasons(&a), reasons(main_game()));
}

#[test]
fn a_game_with_no_intent_gives_nothing() {
    let cut = text()
        .split(COMMANDS_MARKER)
        .next()
        .expect("a part")
        .to_string();
    let g = play(&cut);
    for day in 1..=14 {
        assert!(g.handed[&day].is_empty(), "nothing handed in on day {day}");
    }
}
