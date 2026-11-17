//! Scenario 67: taking a place. The commander names a place, a transport
//! force, three units and the base they sail from. The staff picks the two
//! units the ships can carry, by what each is good at, makes the landing
//! ready, asks leave to launch, and on leave loads supply, embarks the
//! men, sails, lands them on attack and takes an airfield nobody defends.
//!
//! Proves decision 0142, step 3 of stage 3, against a hand calculation.
//! The data is placeholder data: see
//! `data/scenarios/67_taking_a_place.toml`. It runs on the frozen step 1
//! figures, with ground figures of its own that differ from the frozen
//! ones in five: `preparation_per_day` 25, `training_days_per_level` 3,
//! `assault_skill_strength` 0.25, `unprepared_landing_share` 0.5 and
//! `unprepared_landing_fatigue` 40. No enemy man is ashore anywhere, so no
//! die is rolled.
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
//! A force in port with its roster gains 25 of loading a day, to 100. A
//! unit that stands ready on that roster gains 25 of rehearsal a day times
//! its assault factor, to 100: the factor is 1 at assault skill 2 and
//! 1 + 0.25 x 2 = 1.5 at skill 4, which is 37.5 a day. A man weighs 0.25 t
//! to load or land and eats 0.005 t a day. A size 5 port has four berths
//! at 150 t an hour; a size 0 port works by lighter at 20 t an hour. A
//! force lands at the port's rate times 0.5 + 0.5 x its loading over 100.
//! A unit's first men ashore gain 40 x (100 - its rehearsal) / 100 of
//! fatigue. A day is 24 hours: a night pulse of 12 and two daylight pulses
//! of 6. Day d begins at hour 24 x (d - 1). In a pulse ships move before
//! cargo is worked. The men's supply is paid at the start of the night
//! pulse. Loading, rehearsal and a base that changes side are settled at
//! the end of the day.
//!
//! ## Inputs
//!
//! The placeholder map of scenario 27: A at (0,0), a coast hex with port
//! size 5, and B at (4,0), a coast hex with port size 0. A to B is 4
//! hexes: 160 nm, 16 hours at 10 kn. The game is 14 days.
//!
//! | Base | Side | Hex | Supply | Fuel |
//! | ---- | ---- | --- | ------ | ---- |
//! | 1, "Placeholder base A" | Allied | A | 1,000 t | 0 |
//! | 2, "Placeholder airfield B" | Japanese | B | 0 | 0 |
//!
//! Base 2 has no gun, no aircraft and no man. The Allied side holds no
//! estimate of the enemy's men at B.
//!
//! Task force 1, Allied, on the transport mission: ships 1 and 2,
//! placeholder troop transports (600 men, 3,000 t of cargo, 10 kn at
//! cruise, 0.1 t of fuel a mile, 1,000 t of fuel), stopped at base 1,
//! full of fuel, with nothing aboard.
//!
//! Ashore at A, all fighting units, every man fed in full:
//!
//! | Unit | Men | Experience | Assault skill |
//! | ---- | --- | ---------- | ------------- |
//! | 1, "Placeholder Raw" | 600 | 1 | 2 |
//! | 2, "Placeholder Sound" | 600 | 2 | 2 |
//! | 3, "Placeholder Assault" | 600 | 2 | 4 |
//!
//! The Allied side's commands of day 1: intent 1, take the place at B,
//! with a forces limit of task force 1, units 1, 2 and 3 and base 1 as
//! the source; no risk set, so balanced; no date. Leave to prepare is
//! given and leave to launch is withheld. On day 7 the commander gives
//! leave to launch.
//!
//! ## The choice
//!
//! The units in their order: unit 3 (assault 4), unit 2 (assault 2,
//! experience 2), unit 1 (assault 2, experience 1). Two ships of 600
//! berths: unit 3 takes one and unit 2 the other. No ship is left for
//! unit 1: it is left out. No estimate is held, so the staff takes every
//! unit the ships can carry and says that it goes with no count of the
//! enemy.
//!
//! The supply is 1,200 men x 0.005 x 15 days = 90 t. The days to embark
//! are 3: one for the supply and one for each unit. The passage is 16
//! hours: one day.
//!
//! ## Day by day
//!
//! | Day | Begins at hour | The staff's commands and orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 | 0 | Approves plan 1 to preparing. Forms intent 2, ready a landing by task force 1 at B, and sets its forces limit to units 2 and 3. Orders `prepare_landing` at B, roster 2 and 3 | Loading 25. Unit 2's rehearsal 25, unit 3's 37.5 |
//! | 2 | 24 | none | 50; 50; 75 |
//! | 3 | 48 | none | 75; 75; 100 |
//! | 4 | 72 | none | 100; 100; 100 |
//! | 5 | 96 | none. Notices: intent 2 is ready; plan 1 waits for leave to launch | |
//! | 6 | 120 | none. Notice: plan 1 waits for leave to launch | |
//! | 7 | 144 | Approves plan 1 to released. Orders a load of supply to a weight of 90 t | Ship 1 loads 90 t in 90 / 150 = 0.6 hours |
//! | 8 | 168 | Unit 2 set to attack. Load unit 2, no cargo | Ship 1 takes 600 men: 150 t in 1 hour. Complete at hour 169 |
//! | 9 | 192 | Unit 3 set to attack. Load unit 3, no cargo | Ship 2 takes 600 men. Complete at hour 193 |
//! | 10 | 216 | A route to B and back to A, acting at B, and an unload order | Sails at hour 216, arrives at hour 232. The landing, below. Base 2 changes side |
//! | 11 | 240 | Marks plan 1 complete. Notice: intent 1 is met | The last supply is landed at hour 244. The force steams for A at hour 252 |
//! | 12 | 264 | none | The force arrives at A at hour 268 |
//! | 13, 14 | | none | |
//!
//! Eight orders in all and five staff commands. Unit 1 is given no order
//! on any day. No staff order is refused.
//!
//! ## The landing
//!
//! The loading is 100 and for B: the factor is 0.5 + 0.5 x 1 = 1, and
//! each ship works by lighter at 20 t an hour.
//!
//! | Pulse | Hours | Ship 1 | Ship 2 |
//! | ----- | ----- | ------ | ------ |
//! | Day 10, morning | 2 | 40 t: 160 men of unit 2 | 40 t: 160 men of unit 3 |
//! | Day 10, afternoon | 6 | 440 men take 110 t; then 10 t of supply | 440 men take 110 t: done 5.5 hours in, at hour 239.5 |
//! | Day 11, night | 12 | 80 t of supply in 4 hours: complete at hour 244 | |
//!
//! - The first man forms base 3, "Landing at (4,0)", Allied.
//! - Each unit's first men land with a rehearsal of 100 for B: it gains
//!   40 x 0 = 0 fatigue. Its rehearsal is then 0 and it leaves the roster.
//! - At the end of day 10 units 2 and 3 are on attack at B, where the
//!   Japanese side has a base and no man: the Japanese side is beaten
//!   there without a fight and base 2 is Allied.
//! - When the unloading is complete the force's loading is 0 and its
//!   landing order is done.
//! - Each ship burns 16 t each way and has 968 t of fuel at the end. Base
//!   1 has no fuel to give.
//!
//! ## Supply
//!
//! Base 1 feeds the men ashore at A at the start of each night, before
//! that pulse's loading: 1,800 men, 9 t, on days 1 to 8; 1,200 men, 6 t,
//! on day 9; 600 men, 3 t, on days 10 to 14. That is 72 + 6 + 15 = 93 t.
//! With the 90 t loaded on day 7 it holds 847 t after day 7 and 817 t at
//! the end.
//!
//! At B the 10 t landed on the afternoon of day 10 go into base 3, the
//! side's only base there then. On the night of day 11 the 1,200 men are
//! charged 6 t: base 2, now Allied and the lower numbered, holds nothing,
//! and base 3 pays 6 t and holds 4 t. The 80 t landed in that pulse go
//! into base 2, the side's lowest-numbered base in the hex. Base 2 pays
//! 6 t on the nights of days 12, 13 and 14 and holds 62 t at the end.
//! Base 3 holds 4 t.
//!
//! ## At the end
//!
//! Units 2 and 3 are at B with 600 men each, on attack, fed in full, with
//! fatigue 0. Unit 1 is at A with 600 men, experience 1, never ordered.
//! Task force 1 is stopped at A, empty. Plan 1 is complete. Base 2 is
//! Allied.
//!
//! ## Reasons
//!
//! - The reason for the forces limit of day 1 names the units in their
//!   order with each one's assault skill, experience and men; the 1,200
//!   berths in two ships; unit 1 as left out because no ship is left for
//!   it; that no estimate of the enemy is held; the supply, 90 t; and the
//!   days: 3 to embark and 1 of passage.
//! - The reason of days 1 to 4 for giving intent 1 no order gives the
//!   loading, each unit's rehearsal and the morning it expects the
//!   landing to be ready, day 5.
//! - The reason of days 5 and 6 says that the landing is ready and waits
//!   for leave to launch, and that with leave on a morning it sails three
//!   days after and lands that day.
//!
//! # Properties
//!
//! - **Leave to launch given from day 1.** The plan is released on the
//!   morning of day 5 and no notice says it waits. The supply is loaded
//!   on day 5, unit 2 goes aboard on day 6 and unit 3 on day 7, the force
//!   sails on day 8 and arrives at hour 184, base 2 changes side on day 8,
//!   and the plan is marked complete on the morning of day 9.
//! - **Press.** With intent 1 at press and leave to launch given: the
//!   loading is 50, unit 2's rehearsal 50 and unit 3's 75 after day 2, so
//!   the landing is ready and the plan released on the morning of day 3.
//!   The supply is loaded on day 3, unit 2 goes aboard on day 4 and unit
//!   3 on day 5, and the force sails on day 6. The force and both units
//!   went on gaining in port: all three are at 100 by then, the landing
//!   is at 20 t an hour and neither unit gains fatigue. Base 2 changes
//!   side on day 6.
//! - **A third transport.** With ship 3, a third placeholder troop
//!   transport, in task force 1, and leave to launch given: all three
//!   units are chosen. Unit 1 trains on days 1 to 3, is at experience 2,
//!   is set to stand ready on the morning of day 4 and has a rehearsal of
//!   100 after day 7. The plan is released on the morning of day 8. The
//!   supply is 1,800 x 0.005 x 15 = 135 t, loaded on day 8; units 1, 2
//!   and 3 go aboard on days 9, 10 and 11; the force sails on day 12 and
//!   arrives at hour 280; base 2 changes side on day 12 and the plan is
//!   marked complete on the morning of day 13.
//! - **An estimate of the enemy.** With an opening estimate of 400
//!   Japanese men at B, and no man there:
//!   - at balanced the staff wants 800 men: unit 3's 600 are too few and
//!     with unit 2 it has 1,200: both are chosen, as in the main case;
//!   - at press it wants 400: unit 3 alone is chosen. The supply is
//!     600 x 0.005 x 15 = 45 t. With leave to launch given the plan is
//!     released on the morning of day 3, the supply is loaded that day,
//!     unit 3 goes aboard on day 4, the force sails on day 5 and base 2
//!     changes side on day 5.
//!
//!   With an opening estimate of 700 and preserve the force: it wants
//!   2,100 and the ships can carry 1,200. It forms no task, gives no
//!   order, asks no leave, and hands in the notice that it cannot meet
//!   intent 1 on day 1, with a reason that gives the 1,200 against the
//!   2,100.
//! - **A date it can make.** With a date of day 10: the task's date is
//!   the morning of day 10 - 3 - 1 + 1 = 7, the landing is ready on the
//!   morning of day 5, and no notice says it will be late.
//! - **A date it cannot make.** With a date of day 7: the task's date is
//!   the morning of day 4. On the morning of day 1 the staff hands in the
//!   notice that the landing cannot be ready by its date: 1 day late, and
//!   the reason names the loading.
//! - **A place that is the side's already.** With base 2 Allied at the
//!   start: no task is formed, no order is given, and the notice that
//!   intent 1 is met is handed in on day 1.
//! - **A bare shore.** With no base at B: the first man forms base 2,
//!   "Landing at (4,0)", on day 10, the plan is marked complete on the
//!   morning of day 11, and all 90 t go into that base: it holds 10 t
//!   after day 10, 84 t after day 11 and 66 t at the end.
//! - **Aboard already, and at sea.** With task force 1 stopped at sea at
//!   (2,0), units 2 and 3 aboard ships 1 and 2, 90 t of supply aboard
//!   ship 1, unit 1 ashore at A and leave to launch given: on day 1 the
//!   staff chooses units 2 and 3, who keep their ships, and leaves unit 1
//!   out. The landing is ready as it stands, with a loading of 0 and no
//!   rehearsal, and the reason says so. No supply is loaded and no unit
//!   is ordered aboard. The force is routed on day 1 and arrives at hour
//!   8. It lands at 20 x 0.5 = 10 t an hour: 160 men from each ship in
//!   the 4 hours of the night, 240 in the morning, and the last 200 in
//!   the afternoon, when ship 1 also lands 10 t. Each unit gains 40
//!   fatigue as its first men land. On the morning of day 2 both units
//!   are set to attack; base 2 changes side on day 2; the plan is marked
//!   complete on the morning of day 3. The last 80 t are landed in 8
//!   hours of the night of day 2, complete at hour 32.
//! - **An escort.** With task force 2, one placeholder fast transport on
//!   the escort mission, stopped at base 1 and named in the forces limit,
//!   preserve the force, and leave to launch given: on the morning of day
//!   8 the order that gives task force 2 task force 1 as its charge is
//!   handed in before the route order. At balanced, with no contact held,
//!   no escort order is given and the reason says the force sails alone.
//! - **Without leave to prepare** no command and no order is given, and
//!   the notice says the plan waits for leave to prepare.
//! - **A hold.** With task force 1 held from day 1: no task is formed, no
//!   order is given, and the reason names the hold.
//! - **Saved and resumed.** The game saved after day 6 and resumed, and
//!   saved again after day 9 and resumed, ends with the hash of the game
//!   played through.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see give the same orders and reasons;
//!   every scenario built before this one gives the events it gave.

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::command::{
    CommandKind, IntentKind, Leave, Limit, NoticeKind, PlanOutcome, PlanState, Reason,
    StaffDecisions, StaffOrder, Staffs,
};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, LandUnitId, ShipId, TaskForceId};
use sim_core::orders::{Activity, Cargo, LandUnitOrder, OrderKind, Posture, TaskForceOrder};
use sim_core::state::{Side, TaskForceStatus, TruthState};
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

mod common;

const COMMANDS_MARKER: &str = "# The Allied commander's commands";
const A: Hex = Hex { q: 0, r: 0 };
const B: Hex = Hex { q: 4, r: 0 };

fn path() -> PathBuf {
    common::data_dir().join("scenarios/67_taking_a_place.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

/// A number in tenths: x(375) is 37.5.
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

/// The scenario's own ground figures: the frozen ones with five changed.
fn figures() -> sim_data::LoadOptions {
    common::altered_reference("s67", |dir| {
        common::edit_file_by(
            dir,
            "ground.toml",
            &[
                ("preparation_per_day = 5\n", "preparation_per_day = 25\n"),
                (
                    "training_days_per_level = 30\n",
                    "training_days_per_level = 3\n",
                ),
                (
                    "assault_skill_strength = 0\n",
                    "assault_skill_strength = \"0.25\"\n",
                ),
                (
                    "unprepared_landing_share = 1\n",
                    "unprepared_landing_share = \"0.5\"\n",
                ),
                (
                    "unprepared_landing_fatigue = 0\n",
                    "unprepared_landing_fatigue = 40\n",
                ),
            ],
        );
    })
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

/// Plays `scenario` to its end with the staffs, adding to `handed`.
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
    Activity(u32, Activity),
    LoadTo(Fixed),
    Attack(u32),
    LoadUnit(u32),
    Route(Vec<Hex>, Vec<Hex>),
    Unload,
    Escort(u32, u32),
}

fn given(o: &StaffOrder) -> Given {
    match &o.kind {
        OrderKind::TaskForce {
            target,
            order: TaskForceOrder::PrepareLanding { hex, roster },
        } => Given::Landing(target.raw(), *hex, roster.iter().map(|u| u.raw()).collect()),
        OrderKind::LandUnit {
            target,
            order: LandUnitOrder::SetActivity(activity),
        } => Given::Activity(target.raw(), *activity),
        OrderKind::TaskForce {
            order:
                TaskForceOrder::LoadTo {
                    cargo: Cargo::Supply,
                    weight_t,
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
        OrderKind::TaskForce {
            target,
            order: TaskForceOrder::Escort(Some(charge)),
        } => Given::Escort(target.raw(), charge.raw()),
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

fn landing_kind(by_day: Option<u32>) -> IntentKind {
    IntentKind::ReadyLanding {
        force: TaskForceId::new(1),
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

    /// The days on which a notice of `kind` was handed in for plan 1.
    fn notice_days(&self, kind: NoticeKind) -> Vec<u32> {
        (1..=self.days())
            .filter(|d| {
                self.handed[d]
                    .notices
                    .iter()
                    .any(|nt| nt.kind == kind && nt.plan.raw() == 1)
            })
            .collect()
    }

    /// Every notice of the game, by day, in the order handed in.
    fn all_notices(&self) -> Vec<(u32, NoticeKind)> {
        (1..=self.days())
            .flat_map(|d| self.handed[&d].notices.iter().map(move |nt| (d, nt.kind)))
            .collect()
    }

    fn notice_reason(&self, day: u32, kind: NoticeKind) -> &Reason {
        let decisions = &self.handed[&day];
        let notice = decisions
            .notices
            .iter()
            .find(|nt| nt.kind == kind)
            .unwrap_or_else(|| panic!("day {day} has a {kind:?} notice"));
        match notice.rests_on {
            sim_core::command::NoticeBasis::NoOrder(i) => &decisions.no_orders[i],
            sim_core::command::NoticeBasis::Command(i) => &decisions.commands[i].reason,
        }
    }

    /// The no-order reasons of `day` serving `of`.
    fn no_orders(&self, day: u32, of: u32) -> Vec<&Reason> {
        self.handed[&day]
            .no_orders
            .iter()
            .filter(|r| r.serves.intent == Some(intent(of)))
            .collect()
    }

    fn loading(&self, day: u32) -> Fixed {
        self.truth[day as usize].task_forces[&TaskForceId::new(1)]
            .preparation
            .as_ref()
            .map_or(Fixed::ZERO, |p| p.loading)
    }

    fn unit_state(&self, day: u32, u: u32) -> &sim_core::state::LandUnit {
        &self.truth[day as usize].land_units[&unit(u)]
    }

    fn rehearsal(&self, day: u32, u: u32) -> Fixed {
        self.unit_state(day, u).rehearsal
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

    fn plan_state(&self) -> PlanState {
        let state = self.events.iter().rev().find_map(|e| match &e.kind {
            EventKind::PlanStateChanged { side, plan, to, .. }
                if *side == Side::Allied && plan.raw() == 1 =>
            {
                Some(*to)
            }
            _ => None,
        });
        state.unwrap_or(PlanState::Draft)
    }

    /// The day the plan went to `to`.
    fn plan_day(&self, to: PlanState) -> Option<u32> {
        self.events.iter().find_map(|e| match &e.kind {
            EventKind::PlanStateChanged {
                side, plan, to: t, ..
            } if *side == Side::Allied && plan.raw() == 1 && *t == to => Some(e.day),
            _ => None,
        })
    }

    /// The day base `b` changed side, if it did.
    fn changed_side_day(&self, b: u32) -> Option<u32> {
        self.events.iter().find_map(|e| match &e.kind {
            EventKind::BaseChangedSide { base, .. } if base.raw() == b => Some(e.day),
            _ => None,
        })
    }

    fn arrivals(&self, at: Hex) -> Vec<Fixed> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceArrived {
                    task_force,
                    hex,
                    elapsed_hours,
                } if task_force.raw() == 1 && *hex == at => Some(*elapsed_hours),
                EventKind::TaskForceHalted {
                    task_force,
                    hex,
                    elapsed_hours,
                    ..
                } if task_force.raw() == 1 && *hex == at => Some(*elapsed_hours),
                _ => None,
            })
            .collect()
    }

    fn unloading_complete(&self) -> Vec<(BaseId, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::UnloadingComplete {
                    task_force,
                    base,
                    elapsed_hours,
                } if task_force.raw() == 1 => Some((*base, *elapsed_hours)),
                _ => None,
            })
            .collect()
    }

    fn loading_complete(&self) -> Vec<(u32, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::LoadingComplete {
                    task_force,
                    elapsed_hours,
                    ..
                } if task_force.raw() == 1 => Some((e.day, *elapsed_hours)),
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

fn assert_says(reason: &Reason, parts: &[&str]) {
    let all = text_of(reason);
    for part in parts {
        assert!(all.contains(part), "the reason says {part:?}: {all}");
    }
}

/// The parts appear in the reason's working in this order.
fn assert_in_order(reason: &Reason, parts: &[&str]) {
    let all = &reason.worked;
    let mut from = 0;
    for part in parts {
        let at = all[from..]
            .find(part)
            .unwrap_or_else(|| panic!("the working says {part:?} after the last: {all}"));
        from += at + part.len();
    }
}

/// The scenario's text with leave to launch given on day 1 rather than 7.
fn launch_from_day_1(t: &str) -> String {
    changed(
        t,
        "day = 7\nside = \"allied\"\nkind = \"set_leave\"\nleave = \"launch\"",
        "day = 1\nside = \"allied\"\nkind = \"set_leave\"\nleave = \"launch\"",
    )
}

/// The scenario's text with `extra` added after the commander's commands.
fn with_commands(t: &str, extra: &str) -> String {
    format!("{t}\n{extra}")
}

fn risk(level: &str) -> String {
    format!(
        "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"{level}\"\n"
    )
}

/// The text with `extra` put before the commander's commands.
fn with_data(t: &str, extra: &str) -> String {
    changed(t, COMMANDS_MARKER, &format!("{extra}\n{COMMANDS_MARKER}"))
}

fn estimate(men: u32) -> String {
    format!("[[estimate]]\nside = \"allied\"\nq = 4\nr = 0\nmen = {men}\n")
}

fn main_game() -> &'static Played {
    static GAME: std::sync::OnceLock<Played> = std::sync::OnceLock::new();
    GAME.get_or_init(|| play(&text()))
}

// ---------------------------------------------------------------- The hand calculation

#[test]
fn the_choice_and_the_task_formed_on_day_1() {
    let g = main_game();
    assert_eq!(g.days(), 14);
    assert_eq!(
        g.commands(1),
        vec![
            Cmd::Approve(1),
            Cmd::Form(1, landing_kind(None)),
            Cmd::Limit(2, vec![2, 3]),
        ]
    );
    assert_eq!(g.orders(1), vec![Given::Landing(1, B, vec![2, 3])]);
    // Intent 2 is the staff's, under intent 1, in plan 1.
    let formed = g.events.iter().any(|e| {
        matches!(&e.kind, EventKind::IntentFormed { side: Side::Allied, intent: i, parent: p }
            if i.raw() == 2 && p.raw() == 1)
    });
    assert!(formed, "intent 2 is formed under intent 1");
}

#[test]
fn day_by_day_the_loading_and_the_rehearsals() {
    let g = main_game();
    let loading = [25, 50, 75, 100];
    let unit2 = [25, 50, 75, 100];
    let unit3 = [375, 750, 1000, 1000];
    for day in 1..=4u32 {
        let i = (day - 1) as usize;
        assert_eq!(g.loading(day), n(loading[i]), "loading after day {day}");
        assert_eq!(g.rehearsal(day, 2), n(unit2[i]), "unit 2 after day {day}");
        assert_eq!(g.rehearsal(day, 3), x(unit3[i]), "unit 3 after day {day}");
    }
    for day in 5..=9u32 {
        assert_eq!(g.loading(day), n(100), "loading after day {day}");
        assert_eq!(g.rehearsal(day, 2), n(100));
        assert_eq!(g.rehearsal(day, 3), n(100));
    }
    // The landing spends the rehearsal and the loading.
    assert_eq!(g.rehearsal(10, 2), Fixed::ZERO);
    assert_eq!(g.rehearsal(10, 3), Fixed::ZERO);
    assert_eq!(g.loading(11), Fixed::ZERO);
}

#[test]
fn eight_orders_in_all_and_five_staff_commands() {
    let g = main_game();
    assert_eq!(
        g.all_orders(),
        vec![
            (1, Given::Landing(1, B, vec![2, 3])),
            (7, Given::LoadTo(n(90))),
            (8, Given::Attack(2)),
            (8, Given::LoadUnit(2)),
            (9, Given::Attack(3)),
            (9, Given::LoadUnit(3)),
            (10, route()),
            (10, Given::Unload),
        ]
    );
    assert_eq!(
        g.all_commands(),
        vec![
            (1, Cmd::Approve(1)),
            (1, Cmd::Form(1, landing_kind(None))),
            (1, Cmd::Limit(2, vec![2, 3])),
            (7, Cmd::Approve(1)),
            (11, Cmd::Complete(1)),
        ]
    );
    // Unit 1 is given no order on any day; no staff order is refused.
    for (_, o) in g.all_orders() {
        assert!(
            !matches!(
                o,
                Given::Activity(1, _) | Given::Attack(1) | Given::LoadUnit(1)
            ),
            "unit 1 is given no order"
        );
    }
    assert!(g.refused().is_empty(), "{:?}", g.refused());
    assert_eq!(g.plan_day(PlanState::Preparing), Some(1));
    assert_eq!(g.plan_day(PlanState::Released), Some(7));
    assert_eq!(g.plan_day(PlanState::Complete), Some(11));
}

#[test]
fn the_notices() {
    let g = main_game();
    assert_eq!(
        g.all_notices(),
        vec![
            (5, NoticeKind::Ready),
            (5, NoticeKind::WaitsForLeave(Leave::Launch)),
            (6, NoticeKind::WaitsForLeave(Leave::Launch)),
            (11, NoticeKind::Met),
        ]
    );
}

#[test]
fn the_supply_loaded_and_the_men_embarked() {
    let g = main_game();
    // Day 7: ship 1 loads 90 t in 90 / 150 = 0.6 hours, from hour 144.
    assert_eq!(g.loading_complete(), vec![(7, x(1446))]);
    assert_eq!(g.ship(7, 1).supply_t, n(90));
    assert_eq!(g.ship(7, 2).supply_t, Fixed::ZERO);
    // Day 8: ship 1 takes unit 2's 600 men; day 9: ship 2 takes unit 3's.
    let troops = |day: u32, s: u32| g.ship(day, s).troops.map(|t| (t.unit.raw(), t.men));
    assert_eq!(troops(7, 1), None);
    assert_eq!(troops(8, 1), Some((2, 600)));
    assert_eq!(troops(8, 2), None);
    assert_eq!(troops(9, 2), Some((3, 600)));
    assert_eq!(g.unit_state(8, 2).posture, Posture::Attack);
    assert_eq!(g.unit_state(8, 3).posture, Posture::Defend);
    assert_eq!(g.unit_state(9, 3).posture, Posture::Attack);
    let embarked: Vec<(u32, u32, u32)> = g
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::MenEmbarked { unit, men, .. } => Some((e.day, unit.raw(), *men)),
            _ => None,
        })
        .collect();
    assert_eq!(embarked, vec![(8, 2, 600), (9, 3, 600)]);
}

#[test]
fn the_passage_and_the_landing() {
    let g = main_game();
    // Sails at hour 216, arrives at B at hour 232; back at A at hour 268.
    assert_eq!(g.arrivals(B), vec![n(232)]);
    assert_eq!(g.arrivals(A), vec![n(268)]);
    // It steams for A at the start of the morning of day 11, hour 252.
    let steamed: Vec<(u32, Option<sim_core::time::Pulse>)> = g
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::TaskForceSteamedOn {
                task_force, hex, ..
            } if task_force.raw() == 1 && *hex == B => Some((e.day, e.pulse)),
            _ => None,
        })
        .collect();
    assert_eq!(steamed, vec![(11, Some(sim_core::time::Pulse::Morning))]);
    // Men landed, by pulse, from each ship.
    let landed: Vec<(u32, sim_core::time::Pulse, u32, u32, u32)> = g
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::MenLanded {
                unit, ship, men, ..
            } => Some((e.day, e.pulse?, ship.raw(), unit.raw(), *men)),
            _ => None,
        })
        .collect();
    use sim_core::time::Pulse::{Afternoon, Morning};
    assert_eq!(
        landed,
        vec![
            (10, Morning, 1, 2, 160),
            (10, Morning, 2, 3, 160),
            (10, Afternoon, 1, 2, 440),
            (10, Afternoon, 2, 3, 440),
        ]
    );
    // The first man forms base 3, "Landing at (4,0)", Allied.
    let formed = g.base(10, 3).expect("base 3 is formed");
    assert_eq!(formed.name, "Landing at (4,0)");
    assert_eq!(formed.side, Side::Allied);
    assert_eq!(formed.hex, B);
    assert!(g.base(9, 3).is_none());
    // Each unit's first men land with a rehearsal of 100: no fatigue; it
    // leaves the roster.
    assert_eq!(g.unit_state(10, 2).fatigue, Fixed::ZERO);
    assert_eq!(g.unit_state(10, 3).fatigue, Fixed::ZERO);
    // Base 2 changes side at the end of day 10.
    assert_eq!(g.changed_side_day(2), Some(10));
    assert_eq!(g.base_side(9, 2), Side::Japanese);
    assert_eq!(g.base_side(10, 2), Side::Allied);
    // The last supply is landed at hour 244, into base 2.
    assert_eq!(g.unloading_complete(), vec![(BaseId::new(2), n(244))]);
    let preparation = &g.truth[11].task_forces[&TaskForceId::new(1)].preparation;
    assert!(preparation.is_none(), "the landing order is done");
    // Each ship burns 16 t each way and has 968 t at the end.
    for s in [1, 2] {
        assert_eq!(g.ship(14, s).fuel_t, n(968));
    }
}

#[test]
fn the_supply_at_both_ends() {
    let g = main_game();
    // Base 1: 9 t a night on days 1 to 8, 6 t on day 9, 3 t on days 10 to
    // 14, and the 90 t loaded on day 7.
    let base1 = [
        991, 982, 973, 964, 955, 946, 847, 838, 832, 829, 826, 823, 820, 817,
    ];
    for day in 1..=14u32 {
        assert_eq!(
            g.supply(day, 1),
            n(base1[(day - 1) as usize]),
            "base 1 after day {day}"
        );
    }
    assert_eq!(g.supply(7, 1), n(847));
    assert_eq!(g.supply(14, 1), n(817));
    // At B: 10 t into base 3 on day 10; base 3 pays 6 t on the night of day
    // 11 and holds 4 t; the 80 t landed then go into base 2, which pays 6 t
    // on each of the nights of days 12 to 14.
    assert_eq!(g.supply(10, 3), n(10));
    assert_eq!(g.supply(10, 2), Fixed::ZERO);
    assert_eq!(g.supply(11, 3), n(4));
    assert_eq!(g.supply(11, 2), n(80));
    assert_eq!(g.supply(12, 2), n(74));
    assert_eq!(g.supply(13, 2), n(68));
    assert_eq!(g.supply(14, 2), n(62));
    assert_eq!(g.supply(14, 3), n(4));
}

#[test]
fn at_the_end() {
    let g = main_game();
    for u in [2, 3] {
        let s = g.unit_state(14, u);
        assert_eq!((s.hex, s.men_ashore), (Some(B), 600), "unit {u}");
        assert_eq!(s.posture, Posture::Attack);
        assert_eq!(s.fed_share, Fixed::ONE);
        assert_eq!(s.fatigue, Fixed::ZERO);
    }
    let raw = g.unit_state(14, 1);
    assert_eq!((raw.hex, raw.men_ashore, raw.experience), (Some(A), 600, 1));
    assert_eq!(raw.posture, Posture::Defend);
    assert_eq!(raw.activity, Activity::Ready);
    let force = &g.truth[14].task_forces[&TaskForceId::new(1)];
    assert_eq!(force.from_hex, A);
    assert!(matches!(
        force.status,
        TaskForceStatus::Arrived | TaskForceStatus::Idle
    ));
    for s in [1, 2] {
        assert_eq!(g.ship(14, s).troops, None);
        assert_eq!(g.ship(14, s).supply_t, Fixed::ZERO);
    }
    assert_eq!(g.plan_state(), PlanState::Complete);
    assert_eq!(g.base_side(14, 2), Side::Allied);
}

#[test]
fn the_reasons_carry_the_numbers() {
    let g = main_game();
    let limit = g.command_reason(1, &Cmd::Limit(2, vec![2, 3]));
    assert_in_order(
        limit,
        &[
            "no estimate of the enemy",
            "land_unit:3 (Placeholder Assault): assault skill 4, experience 2, 600 men",
            "land_unit:2 (Placeholder Sound): assault skill 2, experience 2, 600 men",
            "land_unit:1 (Placeholder Raw): assault skill 2, experience 1, 600 men",
            "1200 berths in 2 ships",
            "land_unit:1 (Placeholder Raw) (600 men) is left out: no ship is left for it",
            "90 t",
            "3 to embark",
            "1 day of passage",
        ],
    );
    assert_says(limit, &["goes with no count of the enemy"]);
    for day in 1..=4 {
        let reasons = g.no_orders(day, 1);
        assert_eq!(reasons.len(), 1, "one reason under intent 1 on day {day}");
        assert_says(
            reasons[0],
            &[
                &format!("loading for (4,0) is {}", (day - 1) * 25),
                "land_unit:2 (Placeholder Sound) has rehearsal",
                "land_unit:3 (Placeholder Assault) has rehearsal",
                "ready on the morning of day 5",
            ],
        );
    }
    for day in [5, 6] {
        let waits = g.notice_reason(day, NoticeKind::WaitsForLeave(Leave::Launch));
        assert_says(
            waits,
            &[
                "is ready",
                "leave to launch",
                "it sails 3 days after",
                "lands that day",
            ],
        );
    }
}

// ---------------------------------------------------------------- The properties

#[test]
fn leave_to_launch_given_from_day_1() {
    let g = play(&launch_from_day_1(&text()));
    assert_eq!(g.plan_day(PlanState::Released), Some(5));
    assert!(g
        .notice_days(NoticeKind::WaitsForLeave(Leave::Launch))
        .is_empty());
    assert_eq!(
        g.all_orders(),
        vec![
            (1, Given::Landing(1, B, vec![2, 3])),
            (5, Given::LoadTo(n(90))),
            (6, Given::Attack(2)),
            (6, Given::LoadUnit(2)),
            (7, Given::Attack(3)),
            (7, Given::LoadUnit(3)),
            (8, route()),
            (8, Given::Unload),
        ]
    );
    assert_eq!(g.arrivals(B), vec![n(184)]);
    assert_eq!(g.changed_side_day(2), Some(8));
    assert_eq!(g.plan_day(PlanState::Complete), Some(9));
    assert!(g.commands(9).contains(&Cmd::Complete(1)));
}

#[test]
fn press() {
    let t = with_commands(&launch_from_day_1(&text()), &risk("press"));
    let g = play(&t);
    assert_eq!(g.loading(2), n(50));
    assert_eq!(g.rehearsal(2, 2), n(50));
    assert_eq!(g.rehearsal(2, 3), n(75));
    assert_eq!(g.plan_day(PlanState::Released), Some(3));
    assert!(g.commands(3).contains(&Cmd::Approve(1)));
    assert_eq!(
        g.all_orders(),
        vec![
            (1, Given::Landing(1, B, vec![2, 3])),
            (3, Given::LoadTo(n(90))),
            (4, Given::Attack(2)),
            (4, Given::LoadUnit(2)),
            (5, Given::Attack(3)),
            (5, Given::LoadUnit(3)),
            (6, route()),
            (6, Given::Unload),
        ]
    );
    // The force and both units went on gaining in port: all three are at
    // 100 by the day it sails.
    assert_eq!(g.loading(5), n(100));
    assert_eq!(g.rehearsal(5, 2), n(100));
    assert_eq!(g.rehearsal(5, 3), n(100));
    assert_eq!(g.unit_state(6, 2).fatigue, Fixed::ZERO);
    assert_eq!(g.unit_state(6, 3).fatigue, Fixed::ZERO);
    assert_eq!(g.changed_side_day(2), Some(6));
}

#[test]
fn a_third_transport() {
    let ship3 = "[[ship]]\nid = 3\nname = \"Placeholder troop transport 3\"\nclass = 10\nside = \"allied\"\nfuel_t = 1000\ntask_force = 1\n";
    let t = with_data(&launch_from_day_1(&text()), ship3);
    let g = play(&t);
    assert_eq!(g.commands(1)[2], Cmd::Limit(2, vec![1, 2, 3]));
    // Unit 1 trains on days 1 to 3 and is at experience 2.
    assert!(g.orders(1).contains(&Given::Activity(1, Activity::Train)));
    assert_eq!(g.unit_state(3, 1).experience, 2);
    assert_eq!(g.unit_state(2, 1).experience, 1);
    assert!(g.orders(4).contains(&Given::Activity(1, Activity::Ready)));
    assert_eq!(g.rehearsal(7, 1), n(100));
    assert_eq!(g.plan_day(PlanState::Released), Some(8));
    let after_day_1: Vec<(u32, Given)> = g
        .all_orders()
        .into_iter()
        .filter(|(d, o)| *d >= 8 && !matches!(o, Given::Activity(..)))
        .collect();
    assert_eq!(
        after_day_1,
        vec![
            (8, Given::LoadTo(n(135))),
            (9, Given::Attack(1)),
            (9, Given::LoadUnit(1)),
            (10, Given::Attack(2)),
            (10, Given::LoadUnit(2)),
            (11, Given::Attack(3)),
            (11, Given::LoadUnit(3)),
            (12, route()),
            (12, Given::Unload),
        ]
    );
    assert_eq!(g.arrivals(B), vec![n(280)]);
    assert_eq!(g.changed_side_day(2), Some(12));
    assert_eq!(g.plan_day(PlanState::Complete), Some(13));
}

#[test]
fn an_estimate_at_balanced_takes_both() {
    let g = play(&with_data(&text(), &estimate(400)));
    assert_eq!(g.commands(1)[2], Cmd::Limit(2, vec![2, 3]));
    let limit = g.command_reason(1, &Cmd::Limit(2, vec![2, 3]));
    assert_says(limit, &["400 enemy men", "800 men"]);
    assert_eq!(g.all_orders(), main_game().all_orders());
}

#[test]
fn an_estimate_at_press_takes_one() {
    let t = with_data(
        &with_commands(&launch_from_day_1(&text()), &risk("press")),
        &estimate(400),
    );
    let g = play(&t);
    assert_eq!(g.commands(1)[2], Cmd::Limit(2, vec![3]));
    assert_says(
        g.command_reason(1, &Cmd::Limit(2, vec![3])),
        &["400 men", "45 t"],
    );
    assert_eq!(g.plan_day(PlanState::Released), Some(3));
    assert_eq!(
        g.all_orders(),
        vec![
            (1, Given::Landing(1, B, vec![3])),
            (3, Given::LoadTo(n(45))),
            (4, Given::Attack(3)),
            (4, Given::LoadUnit(3)),
            (5, route()),
            (5, Given::Unload),
        ]
    );
    assert_eq!(g.changed_side_day(2), Some(5));
}

#[test]
fn an_estimate_at_preserve_the_force_it_cannot_meet() {
    let t = with_data(&with_commands(&text(), &risk("preserve")), &estimate(700));
    let g = play(&t);
    assert!(g.all_orders().is_empty());
    assert_eq!(g.all_commands(), vec![(1, Cmd::Approve(1))]);
    assert_eq!(g.notice_days(NoticeKind::CannotMeet), vec![1]);
    assert!(g
        .notice_days(NoticeKind::WaitsForLeave(Leave::Launch))
        .is_empty());
    assert_says(
        g.notice_reason(1, NoticeKind::CannotMeet),
        &["1200", "2100"],
    );
}

fn dated(day: u32) -> String {
    changed(
        &text(),
        "intent_kind = \"take_place\"\nhex = [4, 0]\n",
        &format!("intent_kind = \"take_place\"\nhex = [4, 0]\nby_day = {day}\n"),
    )
}

#[test]
fn a_date_it_can_make() {
    let g = play(&dated(10));
    assert_eq!(g.commands(1)[1], Cmd::Form(1, landing_kind(Some(7))));
    assert_eq!(g.notice_days(NoticeKind::Ready), vec![5]);
    assert!(g.notice_days(NoticeKind::Late).is_empty());
}

#[test]
fn a_date_it_cannot_make() {
    let g = play(&dated(7));
    assert_eq!(g.commands(1)[1], Cmd::Form(1, landing_kind(Some(4))));
    assert_eq!(g.notice_days(NoticeKind::Late), vec![1]);
    assert_says(
        g.notice_reason(1, NoticeKind::Late),
        &["1 day late", "the force's loading"],
    );
}

#[test]
fn a_place_that_is_the_sides_already() {
    let t = changed(
        &text(),
        "name = \"Placeholder airfield B\"\nside = \"japanese\"",
        "name = \"Placeholder airfield B\"\nside = \"allied\"",
    );
    let g = play(&t);
    assert!(g.all_orders().is_empty());
    assert!(!g
        .all_commands()
        .iter()
        .any(|(_, c)| matches!(c, Cmd::Form(..) | Cmd::Limit(..))));
    assert_eq!(g.notice_days(NoticeKind::Met), vec![1]);
    assert_eq!(g.plan_day(PlanState::Complete), Some(1));
}

#[test]
fn a_bare_shore() {
    let japanese_base = "# Japanese base 2 at B: no gun, no aircraft, no man, no stock.\n[[base]]\nid = 2\nname = \"Placeholder airfield B\"\nside = \"japanese\"\nq = 4\nr = 0\nsupply_t = 0\nfuel_t = 0\n";
    let g = play(&changed(&text(), japanese_base, ""));
    let formed = g.base(10, 2).expect("base 2 is formed on day 10");
    assert_eq!(formed.name, "Landing at (4,0)");
    assert_eq!(formed.side, Side::Allied);
    assert!(g.base(9, 2).is_none());
    assert_eq!(g.plan_day(PlanState::Complete), Some(11));
    assert_eq!(g.supply(10, 2), n(10));
    assert_eq!(g.supply(11, 2), n(84));
    assert_eq!(g.supply(14, 2), n(66));
}

#[test]
fn aboard_already_and_at_sea() {
    let mut t = launch_from_day_1(&text());
    t = changed(
        &t,
        "name = \"Placeholder transport force 1\"\nside = \"allied\"\nat_base = 1\n",
        "name = \"Placeholder transport force 1\"\nside = \"allied\"\nq = 2\nr = 0\n",
    );
    t = changed(
        &t,
        "name = \"Placeholder troop transport 1\"\nclass = 10\nside = \"allied\"\nfuel_t = 1000\n",
        "name = \"Placeholder troop transport 1\"\nclass = 10\nside = \"allied\"\nfuel_t = 1000\nsupply_t = 90\n",
    );
    t = changed(
        &t,
        "name = \"Placeholder Sound\"\nside = \"allied\"\nmen = 600\nq = 0\nr = 0\n",
        "name = \"Placeholder Sound\"\nside = \"allied\"\nmen = 600\ntask_force = 1\n",
    );
    t = changed(
        &t,
        "name = \"Placeholder Assault\"\nside = \"allied\"\nmen = 600\nq = 0\nr = 0\n",
        "name = \"Placeholder Assault\"\nside = \"allied\"\nmen = 600\ntask_force = 1\n",
    );
    let g = play(&t);
    assert_eq!(g.commands(1)[2], Cmd::Limit(2, vec![2, 3]));
    assert_says(
        g.command_reason(1, &Cmd::Limit(2, vec![2, 3])),
        &["land_unit:1 (Placeholder Raw) (600 men) is left out: no ship is left for it"],
    );
    // Ready as it stands, and the reason says so.
    assert_eq!(g.notice_days(NoticeKind::Ready), vec![1]);
    assert_says(
        g.notice_reason(1, NoticeKind::Ready),
        &[
            "does not lie at a base of its side",
            "loading for (4,0) is 0",
        ],
    );
    assert_eq!(g.plan_day(PlanState::Released), Some(1));
    // No supply loaded, no unit ordered aboard; routed on day 1.
    let orders = g.all_orders();
    assert!(!orders
        .iter()
        .any(|(_, o)| matches!(o, Given::LoadTo(_) | Given::LoadUnit(_))));
    assert!(orders.contains(&(1, route())));
    assert!(orders.contains(&(1, Given::Unload)));
    assert_eq!(g.arrivals(B), vec![n(8)]);
    // 10 t an hour: 160 men from each ship in the night, 240 in the morning,
    // the last 200 in the afternoon.
    use sim_core::time::Pulse::{Afternoon, Morning, Night};
    let landed: Vec<(u32, sim_core::time::Pulse, u32, u32)> = g
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
            (1, Night, 2, 160),
            (1, Night, 3, 160),
            (1, Morning, 2, 240),
            (1, Morning, 3, 240),
            (1, Afternoon, 2, 200),
            (1, Afternoon, 3, 200),
        ]
    );
    assert_eq!(g.ship(1, 1).supply_t, n(80));
    // Each unit gains 40 fatigue as its first men land.
    let landed_fatigue: Vec<(u32, Fixed)> = g
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::LandUnitFatigue {
                unit,
                change: sim_core::events::LandFatigueChange::Landed,
                fatigue,
            } => Some((unit.raw(), *fatigue)),
            _ => None,
        })
        .collect();
    assert_eq!(landed_fatigue, vec![(2, n(40)), (3, n(40))]);
    assert_eq!(
        g.orders(2)
            .into_iter()
            .filter(|o| matches!(o, Given::Attack(_)))
            .collect::<Vec<_>>(),
        vec![Given::Attack(2), Given::Attack(3)]
    );
    assert_eq!(g.changed_side_day(2), Some(2));
    assert_eq!(g.plan_day(PlanState::Complete), Some(3));
    let complete = g.unloading_complete();
    assert_eq!(complete.len(), 1);
    assert_eq!(complete[0].1, n(32));
}

fn escort_text(level: &str) -> String {
    let force2 = "[[task_force]]\nid = 2\nname = \"Placeholder escort force 2\"\nside = \"allied\"\nat_base = 1\nmission = \"escort\"\n\n[[ship]]\nid = 3\nname = \"Placeholder fast transport 3\"\nclass = 11\nside = \"allied\"\nfuel_t = 500\ntask_force = 2\n";
    let t = with_data(&launch_from_day_1(&text()), force2);
    let t = changed(&t, "task_forces = [1]\n", "task_forces = [1, 2]\n");
    with_commands(&t, &risk(level))
}

#[test]
fn an_escort_at_preserve_the_force() {
    let g = play(&escort_text("preserve"));
    assert_eq!(
        g.orders(8),
        vec![Given::Escort(2, 1), route(), Given::Unload]
    );
}

#[test]
fn no_escort_at_balanced_with_no_contact() {
    let g = play(&escort_text("balanced"));
    assert!(!g
        .all_orders()
        .iter()
        .any(|(_, o)| matches!(o, Given::Escort(..))));
    assert_eq!(g.orders(8), vec![route(), Given::Unload]);
    assert_says(g.order_reason(8, &route()), &["sails alone"]);
}

#[test]
fn without_leave_to_prepare() {
    let t = changed(
        &text(),
        "leave = \"prepare\"\ngiven = true\n",
        "leave = \"prepare\"\ngiven = false\n",
    );
    let g = play(&t);
    assert!(g.all_orders().is_empty(), "{:?}", g.all_orders());
    assert!(g.all_commands().is_empty());
    assert_eq!(
        g.notice_days(NoticeKind::WaitsForLeave(Leave::Prepare)),
        (1..=14).collect::<Vec<_>>()
    );
}

#[test]
fn a_hold() {
    let hold = "[[command]]\nday = 1\nside = \"allied\"\nkind = \"hold\"\ntask_force = 1\n";
    let g = play(&with_commands(&text(), hold));
    assert!(g.all_orders().is_empty());
    assert!(!g
        .all_commands()
        .iter()
        .any(|(_, c)| matches!(c, Cmd::Form(..) | Cmd::Limit(..))));
    let reasons = g.no_orders(1, 1);
    assert!(
        reasons
            .iter()
            .any(|r| text_of(r).contains("task_force:1") && text_of(r).contains("held")),
        "{:?}",
        reasons.iter().map(|r| text_of(r)).collect::<Vec<_>>()
    );
}

/// A game rebuilt from what was handed in to day `upto`, as a saved game
/// replays: the scenario's commands and the staff's hand-ins of each day,
/// each day resolved with no staff asked.
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
fn saved_and_resumed() {
    let through = main_game();
    // Saved after day 6 and resumed.
    let at_6 = rebuilt(&through.handed, 6);
    assert_eq!(at_6.world.day_hashes(), &through.hashes[..6]);
    let first: BTreeMap<u32, StaffDecisions> =
        (1..=6).map(|d| (d, through.handed[&d].clone())).collect();
    let resumed = play_from(at_6, first);
    // Saved again after day 9, from the resumed game's hand-ins, and resumed.
    let at_9 = rebuilt(&resumed.handed, 9);
    assert_eq!(at_9.world.day_hashes(), &through.hashes[..9]);
    let second: BTreeMap<u32, StaffDecisions> =
        (1..=9).map(|d| (d, resumed.handed[&d].clone())).collect();
    let again = play_from(at_9, second);
    assert_eq!(again.hashes, through.hashes);
    assert_eq!(again.handed, through.handed);
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
    let a = play(&with_data(&text(), &enemy(100)));
    let b = play(&with_data(&text(), &enemy(300)));
    assert_eq!(a.handed, b.handed);
    assert_eq!(a.all_orders(), main_game().all_orders());
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
