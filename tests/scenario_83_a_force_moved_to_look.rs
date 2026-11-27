//! Scenario 83: a force moved to look. The commander asks for an airfield
//! 400 nm away and names a carrier force with its scouts. Nothing the
//! staff may use can see the airfield from where it lies. The staff sends
//! the carrier forward at once, to the hex she would cover the landing
//! from, looks from there, learns the airfield is empty, takes one unit
//! in place of two, and goes two days later than it would have gone
//! blind.
//!
//! Proves decision 0159, rule D, item 14, step 5 of the opening of stage
//! 4, against a hand calculation. It stands on scenario 73's for the
//! carrier force and the landing and on scenario 72's for the looking.
//! The data is placeholder data: see
//! `data/scenarios/83_a_force_moved_to_look.toml`. It is scenario 73's
//! start with air group 2 named in the forces limit, and it runs on
//! scenario 73's figures with scenario 72's ground figures in place of
//! 73's, so that a reconnaissance flight is flown and reports the true
//! count. Base 2 has no gun, no aircraft and no man, so no aircraft is
//! lost and no die changes anything asserted.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly, except what each bomb hits, how many strikes a group launches
//! in a day, and the supply and fatigue a flight costs, which are the air
//! rules'. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## The rule, as it is built
//!
//! A "take a place" has a carrier task for each carrier force its forces
//! limit names: "operate the carrier force from a hex", the first hex on
//! her way from which her bombers reach the place (decision 0148). Until
//! now it was formed when the plan was released. Now:
//!
//! 1. It is formed while the plan is preparing, on a morning when the
//!    need to know is not met, the forces limit names an air group that
//!    can search and is aboard a ship of that force, and no group the
//!    limit names reaches the place from where it is.
//! 2. Its hex is the covering hex, where a group aboard that the limit
//!    names and that can search reaches the place from it; else the first
//!    hex on her way from which one does. So the force moves once.
//! 3. Before the plan is released the task gives every order it gives
//!    today but the strike on the place's base.
//! 4. A group that is on reconnaissance of the place is given no order by
//!    the carrier task, and when the need is met the looking leaves a
//!    group aboard such a force to the carrier task and does not rest it.
//! 5. At balanced the staff does this without asking. At preserve the
//!    force it does it only with leave to launch given; without it the
//!    reason says that the force could look from that hex and waits for
//!    leave. At press nothing need be known and the task is formed at
//!    release, as before.
//! 6. The force is sent home when the take is met, as before. It is not
//!    brought back when the need is met: it stays to cover.
//!
//! This differs from the decision's first wording in two things, ruled
//! by the architect on Nov 18, 2026: the hex is the covering hex and not
//! the nearest from which a scout reaches (here (4,0) and not (3,0)), so
//! that the force is not moved twice; and it stays there.
//!
//! ## Figures and inputs
//!
//! As scenario 73's: a hex is 40 nm; the placeholder dive bomber reaches
//! 250 nm; the placeholder carrier scout searches to 300 nm; the carrier
//! makes 16 kn and the transports 10 kn; A at (0,0) and B at (10,0),
//! 400 nm apart; base 1, Allied, at A; base 2, Japanese, at B, with no
//! gun, aircraft, supply or man; task force 1, the carrier, with group 1,
//! 12 fighters, group 2, 6 scouts, and group 3, 10 dive bombers; task
//! force 2, ships 2 and 3, troop transports; units 1 and 2, 600 men each,
//! at A. No contact and no estimate is held. A reconnaissance flight
//! reports the true count. The game is 9 days.
//!
//! The Allied side's commands of day 1: intent 1, take the place at base
//! 2, with a forces limit of task forces 1 and 2, units 1 and 2, air
//! group 2 and base 1 as the source. No risk is set, so balanced. Leave
//! to prepare and leave to launch are both given.
//!
//! ## The hex
//!
//! From (0,0) the scouts are 400 nm from B, beyond their 300. The
//! covering hex is (4,0), 240 nm from B: within the bombers' 250, and
//! within the scouts' 300. It is 160 nm from A, 10 hours at 16 kn.
//!
//! ## Day by day
//!
//! | Day | Hour | The landing | The carrier and the looking | What happens |
//! | --- | --- | --- | --- | --- |
//! | 1 | 0 | Approves plan 1 to preparing. Forms intent 2, ready a landing by task force 2 at B, with a forces limit of units 1 and 2. Orders `prepare_landing` at B, roster 1 and 2 | Forms intent 3, operate task force 1 from (4,0), under intent 1. The force: rules of engagement, avoid, break-off level 0.5; a route to (4,0). Group 1: combat air patrol; a patrol share of 0.5. Group 2: the search mission; the arc 050 to 120 at 300 nm. Group 3: the naval strike mission; strike sizing on | The carrier sails at hour 0 and arrives at hour 10. Loading 100; each unit's rehearsal 100 |
//! | 2 | 24 | Notice: intent 2 is ready. The plan is not released: a group is looking | The force: battle readiness. Group 2: reconnaissance of B | In the morning one scout flies 240 nm and reports 0 men, seen from the air |
//! | 3 | 48 | Sets the forces limit of intent 2 to unit 1. Orders `prepare_landing` at B, roster 1. The landing was ordered again and is not ready this morning | Group 2: the search mission | |
//! | 4 | 72 | Approves plan 1 to released. Orders a load of supply to a weight of 45 t | Group 3: base strike at base 2 | Group 3 strikes base 2 |
//! | 5 | 96 | Unit 1 set to attack. Load unit 1 | none | Group 3 strikes base 2 |
//! | 6 | 120 | A route to B and back to A, acting at B, and an unload order | none | The transports sail at hour 120. Group 3 strikes base 2 |
//! | 7 | 144 | none | none | The transports arrive at hour 160. Unit 1's ship lands 160 men in the morning and 440 in the afternoon, and then 10 t. Group 3 strikes base 2. Base 2 changes side at the end of the day |
//! | 8 | 168 | Marks plan 1 complete. Notice: intent 1 is met | The force: a route to A | The carrier sails at hour 168 and arrives at hour 178. The last 35 t is landed in 1.75 hours |
//! | 9 | 192 | none | none | |
//!
//! - **Day 1.** With no estimate the staff takes every unit the ships can
//!   carry. The need is not met; group 2 can search and cannot reach B
//!   from A; no other group is named. The carrier task is formed that
//!   morning with the orders scenario 73 gives on its day 2. The arc is
//!   worked from (0,0), where base 2 is on 090.
//! - **Day 2.** The force is at its hex. Group 2 reaches B, 240 nm, and
//!   is set to reconnaissance. The carrier task gives it no order and
//!   gives no strike on the base: the plan is not released.
//! - **Day 3.** The count is 0, seen from the air, of day 2: the need is
//!   met, at any age at balanced. The men wanted are 0 x 2 = 0 and the
//!   staff takes one unit, unit 1. Group 2 is no longer looking and its
//!   mission is not the search: the carrier task sets it. From (4,0)
//!   base 2 is still on 090: no arc order.
//! - **Day 4.** The landing is ready and the need is met: released. The
//!   supply is 600 x 0.005 x 15 = 45 t. The plan is released, B is within
//!   the bombers' reach and no contact is held: group 3 is given the base
//!   strike.
//! - **Day 6.** 400 nm at 10 kn is 40 hours.
//! - **The end of day 7.** Base 2 is Allied, and group 3's mission is
//!   cleared to rest by the rule of a base that changes side.
//!
//! Seven orders to the landing and thirteen to the carrier and her
//! groups, twenty in all. No staff order or command is refused. Group 2
//! flies the reconnaissance once, on day 2. Beside scenario 73, which
//! goes blind with two units and is ashore on day 6, the staff knows the
//! place is empty, sends one unit, and is ashore on day 7; the carrier is
//! forward from day 1 and not from day 2.
//!
//! # Properties
//!
//! - **The scouts not named.** With air group 2 left out of the forces
//!   limit: the game of scenario 73. No carrier task is formed on day 1;
//!   the plan is released on day 2 on no count, and the carrier task is
//!   formed that morning. Days 1 and 2 are asserted.
//! - **At press.** With the risk press: no carrier task on day 1 and no
//!   group set to reconnaissance on any day; the plan is released on day
//!   2 and the carrier task is formed that morning. Days 1 and 2 are
//!   asserted.
//! - **At preserve the force, with leave.** With the risk preserve: on
//!   day 1 intent 3, operate task force 1 from (4,0), is formed under
//!   intent 1 and the force is given a route to (4,0). Only that is
//!   asserted of day 1; the other orders are those of a carrier task at
//!   preserve the force.
//! - **At preserve the force, without leave.** The same with leave to
//!   launch withheld: no carrier task is formed on days 1 and 2, the
//!   force is given no order, and the reason of each morning says that
//!   task force 1 could look from (4,0) and that the staff waits for
//!   leave to launch.
//! - **Already in reach.** With task force 1 at sea, stopped, at (4,0)
//!   from the start: on day 1 group 2 is set to reconnaissance of B and
//!   no carrier task is formed that morning. Day 1 is asserted.
//! - **The enemy sees nothing of the flight.**
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see and has not been told give the same
//!   orders and reasons; a save on day 3 and a resume give the same game;
//!   scenarios 72 and 73 give the orders they gave.

// ---------------------------------------------------------------- The harness

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use sim_core::belief::GroundGrade;
use sim_core::choices::Moment;
use sim_core::command::{
    CommandKind, CommandState, CountRange, HeldCount, IntentKind, Knowledge, Limit, Looking, Maker,
    MovedToLook, NeedToKnow, NoticeKind, PlanId, PlanOutcome, PlanState, Reason, Staff,
    StaffDecisions, StaffOrder, Staffs,
};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, IntentId, LandUnitId, TaskForceId};
use sim_core::orders::{
    AirGroupOrder, AirMission, Cargo, LandUnitOrder, OrderKind, Posture, Readiness, Stance,
    StragglerInstruction, TaskForceOrder,
};
use sim_core::side_events::{SideEvent, SideEventKind};
use sim_core::state::{Side, TruthState};
use sim_core::time::Pulse;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs, SupplyStaff};

use crate::common;

const COMMANDS_MARKER: &str = "# The Allied commander's commands";
const A: Hex = Hex { q: 0, r: 0 };
const B: Hex = Hex { q: 10, r: 0 };
const COVER: Hex = Hex { q: 4, r: 0 };

fn path() -> PathBuf {
    common::data_dir().join("scenarios/83_a_force_moved_to_look.toml")
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

/// A command table of the Allied side, added to the end of the file.
fn command_table(day: u32, body: &str) -> String {
    format!("\n[[command]]\nday = {day}\nside = \"allied\"\n{body}\n")
}

/// Scenario 73's figures: scenario 53's table of aircraft for each ship,
/// added after the frozen file's empty `[strike_aircraft_per_ship]` table.
/// The file names scenario 72's ground figures itself.
fn figures() -> sim_data::LoadOptions {
    common::altered_reference("s83", |dir| {
        common::edit_file(dir, "air_combat.toml", |text| {
            format!("{text}\ncruiser = 12\ndestroyer = 6\ntransport = 6\nauxiliary = 4\n")
        })
    })
}

fn load(text: &str) -> Scenario {
    load_at(text, &path())
}

fn load_at(text: &str, at: &std::path::Path) -> Scenario {
    common::from_text_with(text, at, &figures()).expect("the scenario loads")
}

struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    /// The need to know of each morning, worked from the view the staff is
    /// asked on, as a seat's `plans` works it.
    needs: BTreeMap<u32, Vec<NeedToKnow>>,
    truth: Vec<TruthState>,
    events: Vec<Event>,
    hashes: Vec<u64>,
    state: CommandState,
    allied_events: Vec<SideEvent>,
    japanese_events: Vec<SideEvent>,
}

fn play(text: &str) -> Played {
    play_from(load(text), BTreeMap::new(), BTreeMap::new())
}

fn play_from(
    mut scenario: Scenario,
    mut handed: BTreeMap<u32, StaffDecisions>,
    mut needs: BTreeMap<u32, Vec<NeedToKnow>>,
) -> Played {
    let held = staffs();
    let mut truth = vec![scenario.world.umpire().clone()];
    loop {
        let day = scenario.world.day() + 1;
        let mut morning: Option<Vec<NeedToKnow>> = None;
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |world, moment| {
                if let Moment::Orders { .. } = moment {
                    morning = Some(SupplyStaff.needs_to_know(world.side_view(Side::Allied)));
                }
            })
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
        needs.insert(day, morning.expect("the staff's moment was passed"));
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
        needs,
        truth,
        events: scenario.world.events().iter().cloned().collect(),
        hashes: scenario.world.day_hashes().to_vec(),
        state: scenario.world.command_state(Side::Allied).clone(),
        allied_events: scenario.world.side_view(Side::Allied).events(),
        japanese_events: scenario.world.side_view(Side::Japanese).events(),
    }
}

/// An order of the staff, plainly: to a task force, an air group or a land
/// unit, by its number.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Given {
    Force(u32, TaskForceOrder),
    Air(u32, AirGroupOrder),
    Unit(u32, LandUnitOrder),
}

fn given(o: &StaffOrder) -> Given {
    match &o.kind {
        OrderKind::TaskForce { target, order } => Given::Force(target.raw(), order.clone()),
        OrderKind::AirGroup { target, order } => Given::Air(target.raw(), order.clone()),
        OrderKind::LandUnit { target, order } => Given::Unit(target.raw(), order.clone()),
        other => panic!("the staff gave an order of another kind: {other:?}"),
    }
}

/// Whether an order is the carrier's or the looking's: to task force 1 or to
/// an air group.
fn carrier_order(o: &StaffOrder) -> bool {
    matches!(
        &o.kind,
        OrderKind::TaskForce { target, .. } if target.raw() == 1
    ) || matches!(o.kind, OrderKind::AirGroup { .. })
}

fn mission(group: u32, m: AirMission) -> Given {
    Given::Air(group, AirGroupOrder::SetAirMission(m))
}

fn share(group: u32, s: &str) -> Given {
    Given::Air(group, AirGroupOrder::SetPatrolShare { share: fx(s) })
}

fn sizing(group: u32) -> Given {
    Given::Air(group, AirGroupOrder::SetStrikeSizing { on: true })
}

fn arc(group: u32, from_deg: i32, to_deg: i32) -> Given {
    Given::Air(
        group,
        AirGroupOrder::SetSearchArc {
            from_deg,
            to_deg,
            range_nm: n(300),
        },
    )
}

fn rules(break_off: &str) -> Given {
    Given::Force(
        1,
        TaskForceOrder::SetRulesOfEngagement {
            stance: Stance::Avoid,
            break_off: fx(break_off),
            stragglers: StragglerInstruction::Keep,
            keep_share: Fixed::ZERO,
        },
    )
}

fn route(force: u32, waypoints: Vec<Hex>, act_at: &[Hex]) -> Given {
    Given::Force(
        force,
        TaskForceOrder::SetRoute {
            waypoints,
            act_at: act_at.iter().copied().collect(),
            stand_at: BTreeSet::new(),
        },
    )
}

fn battle() -> Given {
    Given::Force(1, TaskForceOrder::SetReadiness(Readiness::Battle))
}

fn prepare(units: &[u32]) -> Given {
    Given::Force(
        2,
        TaskForceOrder::PrepareLanding {
            hex: B,
            roster: units.iter().map(|u| LandUnitId::new(*u)).collect(),
        },
    )
}

fn load_to(tons: i64) -> Given {
    Given::Force(
        2,
        TaskForceOrder::LoadTo {
            cargo: Cargo::Supply,
            weight_t: n(tons),
            each_t: None,
        },
    )
}

fn attack(unit: u32) -> Given {
    Given::Unit(
        unit,
        LandUnitOrder::SetPosture {
            posture: Posture::Attack,
            when_spent: None,
        },
    )
}

fn load_unit(unit: u32) -> Given {
    Given::Force(
        2,
        TaskForceOrder::LoadUnit {
            unit: LandUnitId::new(unit),
            cargo: None,
        },
    )
}

const STRIKE: AirMission = AirMission::NavalStrike;
const CAP: AirMission = AirMission::CombatAirPatrol;
const SEARCH: AirMission = AirMission::Search;

fn base_strike() -> AirMission {
    AirMission::BaseStrike(BaseId::new(2))
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
        force: TaskForceId::new(2),
        hex: B,
        by_day: None,
    }
}

fn cover_kind(hex: Hex) -> IntentKind {
    IntentKind::OperateCarriers {
        force: TaskForceId::new(1),
        hex,
    }
}

impl Played {
    fn days(&self) -> u32 {
        u32::try_from(self.handed.len()).expect("few days")
    }

    /// The orders of `day`, in the order the staff gave them.
    fn orders(&self, day: u32) -> Vec<Given> {
        self.handed[&day].orders.iter().map(given).collect()
    }

    /// The carrier's and the looking's orders of a day.
    fn carrier(&self, day: u32) -> Vec<Given> {
        self.handed[&day]
            .orders
            .iter()
            .filter(|o| carrier_order(o))
            .map(given)
            .collect()
    }

    /// The landing's orders of a day: every order that is not the carrier's.
    fn landing(&self, day: u32) -> Vec<Given> {
        self.handed[&day]
            .orders
            .iter()
            .filter(|o| !carrier_order(o))
            .map(given)
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

    fn command_reason(&self, day: u32, wanted: &Cmd) -> String {
        said(
            &self.handed[&day]
                .commands
                .iter()
                .find(|c| cmd(&c.kind) == *wanted)
                .unwrap_or_else(|| panic!("day {day} has the command {wanted:?}"))
                .reason,
        )
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

    /// The reasons for giving no order of a day that serve `of`.
    fn no_orders(&self, day: u32, of: u32) -> Vec<String> {
        self.handed[&day]
            .no_orders
            .iter()
            .filter(|r| r.serves.intent == Some(intent(of)))
            .map(said)
            .collect()
    }

    fn carrier_tasks(&self) -> Vec<(u32, IntentKind, Option<u32>, Maker)> {
        self.state
            .intents
            .values()
            .filter(|i| matches!(i.kind, IntentKind::OperateCarriers { .. }))
            .map(|i| (i.id.raw(), i.kind, i.parent.map(|p| p.raw()), i.maker))
            .collect()
    }

    /// The days on which a carrier task is formed.
    fn carrier_formed(&self) -> Vec<u32> {
        (1..=self.days())
            .filter(|d| {
                self.commands(*d)
                    .iter()
                    .any(|c| matches!(c, Cmd::Form(_, IntentKind::OperateCarriers { .. })))
            })
            .collect()
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

    /// The reconnaissance flights: day, pulse, group, distance.
    fn flights(&self) -> Vec<(u32, Option<Pulse>, u32, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::ReconnaissanceFlown {
                    air_group,
                    hex,
                    distance_nm,
                    ..
                } if *hex == B => Some((e.day, e.pulse, air_group.raw(), *distance_nm)),
                _ => None,
            })
            .collect()
    }

    /// The Allied ground reports of B: day, pulse, men, grade.
    fn reports(&self) -> Vec<(u32, Option<Pulse>, u32, GroundGrade)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::GroundReportReceived {
                    side: Side::Allied,
                    hex,
                    men,
                    grade,
                    ..
                } if *hex == B => Some((e.day, e.pulse, *men, *grade)),
                _ => None,
            })
            .collect()
    }

    /// The days on which group `raw` launched a base strike, with the base
    /// and the aircraft.
    fn base_strikes(&self, raw: u32) -> Vec<(u32, BaseId, u32)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::BaseStrikeLaunched {
                    air_group,
                    base,
                    aircraft,
                    ..
                } if air_group.raw() == raw => Some((e.day, *base, *aircraft)),
                _ => None,
            })
            .collect()
    }

    fn changed_side_day(&self, b: u32) -> Option<u32> {
        self.events.iter().find_map(|e| match &e.kind {
            EventKind::BaseChangedSide { base, .. } if base.raw() == b => Some(e.day),
            _ => None,
        })
    }

    fn landed(&self) -> Vec<(u32, Pulse, u32, u32)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::MenLanded { unit, men, .. } => Some((e.day, e.pulse?, unit.raw(), *men)),
                _ => None,
            })
            .collect()
    }

    /// The supply landed by task force 2: day, pulse, tons.
    fn supply_landed(&self) -> Vec<(u32, Pulse, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::CargoUnloaded {
                    task_force,
                    cargo: Cargo::Supply,
                    unloaded_t,
                    ..
                } if task_force.raw() == 2 => Some((e.day, e.pulse?, *unloaded_t)),
                _ => None,
            })
            .collect()
    }

    fn unloading_complete(&self) -> Vec<Fixed> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::UnloadingComplete {
                    task_force,
                    elapsed_hours,
                    ..
                } if task_force.raw() == 2 => Some(*elapsed_hours),
                _ => None,
            })
            .collect()
    }

    /// Where and at which hour a force arrived or halted.
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

    /// The steaming of a task force: (day, pulse, from, to, nm).
    fn steamed(&self, force: u32) -> Vec<(u32, Pulse, Hex, Hex, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceSteamed {
                    task_force,
                    from,
                    to,
                    distance_nm,
                    ..
                } if task_force.raw() == force => Some((e.day, e.pulse?, *from, *to, *distance_nm)),
                _ => None,
            })
            .collect()
    }

    /// The one need of a morning: plan 1's, where there is one.
    fn need(&self, day: u32) -> Option<&NeedToKnow> {
        let all = &self.needs[&day];
        assert!(all.len() <= 1, "day {day}: {all:?}");
        all.first()
    }

    fn group_at_end(&self, raw: u32) -> &sim_core::state::AirGroup {
        &self.truth.last().expect("a state").air_groups[&AirGroupId::new(raw)]
    }

    /// The days on which group `raw` was set to reconnaissance of B.
    fn sent_to_look(&self, raw: u32) -> Vec<u32> {
        (1..=self.days())
            .filter(|d| {
                self.orders(*d)
                    .contains(&mission(raw, AirMission::Reconnaissance(B)))
            })
            .collect()
    }
}

/// A reason's text: its calculation and its evidence.
fn said(reason: &Reason) -> String {
    let evidence: Vec<&str> = reason.evidence.iter().map(|e| e.what.as_str()).collect();
    format!("{} | {}", reason.worked, evidence.join(" | "))
}

fn assert_says(text: &str, words: &[&str]) {
    for w in words {
        assert!(text.contains(w), "{w:?} is not in: {text}");
    }
}

/// The main game, played once for the tests that read it.
fn main_game() -> &'static Played {
    static GAME: std::sync::OnceLock<Played> = std::sync::OnceLock::new();
    GAME.get_or_init(|| play(&text()))
}

/// The carrier task's orders of the morning it is formed: scenario 73's of
/// its day 2.
fn carrier_formed_orders() -> Vec<Given> {
    vec![
        rules("0.5"),
        route(1, vec![COVER], &[]),
        mission(1, CAP),
        share(1, "0.5"),
        mission(2, SEARCH),
        arc(2, 50, 120),
        mission(3, STRIKE),
        sizing(3),
    ]
}

// --------------------------------------------------------------------- Tests

#[test]
fn the_commands_of_the_file_are_the_ones_these_tests_vary() {
    let t = text();
    assert_eq!(t.matches(COMMANDS_MARKER).count(), 1);
    assert_eq!(t.matches("air_groups = [2]\n").count(), 1);
    assert_eq!(t.matches("leave = \"launch\"\ngiven = true").count(), 1);
    assert!(t.contains("task_forces = [1, 2]"));
    assert!(t.contains("land_units = [1, 2]"));
    assert!(t.contains("days = 9"));
    assert!(t.contains("ground_placeholder_scenario_72.toml"));
    assert!(!t.contains("risk ="));
}

#[test]
fn the_hex_is_the_covering_hex_4_0_and_not_the_nearest_from_which_a_scout_reaches() {
    // From (0,0) the scouts are 400 nm from B, beyond their 300. (3,0) is
    // 280 nm from B, within the scouts' 300 and beyond the bombers' 250;
    // (4,0) is 240 nm, within both, and 160 nm from A.
    assert_eq!(A.air_distance_nm(B), 400);
    assert_eq!(Hex { q: 3, r: 0 }.air_distance_nm(B), 280);
    assert_eq!(COVER.air_distance_nm(B), 240);
    assert_eq!(A.air_distance_nm(COVER), 160);
    let g = main_game();
    assert_eq!(
        g.carrier_tasks(),
        vec![(3, cover_kind(COVER), Some(1), Maker::Staff)]
    );
    assert_eq!(g.carrier_formed(), vec![1]);
    // The reason says where the force could look from, the passage and how
    // near the place it then is.
    let reason = g.command_reason(1, &Cmd::Form(1, cover_kind(COVER)));
    assert_says(
        &reason,
        &[
            "the side holds no estimate of the enemy's men at (10,0)",
            "air_group:2, which searches to 300 nm",
            "(4,0)",
            "250 nm",
            "10 hours",
            "within 240 nm of (10,0)",
        ],
    );
}

#[test]
fn the_staffs_commands_day_by_day() {
    let g = main_game();
    assert_eq!(
        g.all_commands(),
        vec![
            (1, Cmd::Approve(1)),
            (1, Cmd::Form(1, landing_kind())),
            (1, Cmd::Limit(2, vec![1, 2])),
            (1, Cmd::Form(1, cover_kind(COVER))),
            (3, Cmd::Limit(2, vec![1])),
            (4, Cmd::Approve(1)),
            (8, Cmd::Complete(1)),
        ]
    );
    assert_eq!(
        g.all_notices(),
        vec![(2, NoticeKind::Ready, 1), (8, NoticeKind::Met, 1)]
    );
    assert_eq!(g.plan_day(1, PlanState::Preparing), Some(1));
    assert_eq!(g.plan_day(1, PlanState::Released), Some(4));
    assert_eq!(g.plan_day(1, PlanState::Complete), Some(8));
}

#[test]
fn the_twenty_orders_day_by_day() {
    let g = main_game();
    assert!(g.refused().is_empty(), "{:?}", g.refused());
    // The landing's seven.
    let landing: Vec<Vec<Given>> = (1..=9).map(|d| g.landing(d)).collect();
    assert_eq!(
        landing,
        vec![
            vec![prepare(&[1, 2])],
            vec![],
            vec![prepare(&[1])],
            vec![load_to(45)],
            vec![attack(1), load_unit(1)],
            vec![
                route(2, vec![B, A], &[B]),
                Given::Force(2, TaskForceOrder::Unload)
            ],
            vec![],
            vec![],
            vec![],
        ]
    );
    // The carrier's and the looking's thirteen.
    let carrier: Vec<Vec<Given>> = (1..=9).map(|d| g.carrier(d)).collect();
    assert_eq!(
        carrier,
        vec![
            carrier_formed_orders(),
            vec![mission(2, AirMission::Reconnaissance(B)), battle()],
            vec![mission(2, SEARCH)],
            vec![mission(3, base_strike())],
            vec![],
            vec![],
            vec![],
            vec![route(1, vec![A], &[])],
            vec![],
        ]
    );
    let total: usize = (1..=9).map(|d| g.orders(d).len()).sum();
    assert_eq!(total, 20);
}

#[test]
fn who_gives_each_order_to_group_2() {
    let g = main_game();
    let serves = |day: u32, wanted: &Given| -> u32 {
        g.handed[&day]
            .orders
            .iter()
            .find(|o| given(o) == *wanted)
            .unwrap_or_else(|| panic!("day {day}: {wanted:?}"))
            .intent
            .raw()
    };
    // Day 1: the carrier task sets it to the search, from (0,0).
    assert_eq!(serves(1, &mission(2, SEARCH)), 3);
    // Day 2: the take sends it to look; the carrier task gives it nothing.
    assert_eq!(serves(2, &mission(2, AirMission::Reconnaissance(B))), 1);
    // Day 3: the need met, the look leaves it to the carrier task, which
    // sets it to the search, and does not rest it.
    assert_eq!(serves(3, &mission(2, SEARCH)), 3);
    assert!(!g.orders(3).contains(&mission(2, AirMission::Rest)));
    // Day 4: the base strike is the carrier task's.
    assert_eq!(serves(4, &mission(3, base_strike())), 3);
}

#[test]
fn day_2_the_plan_is_not_released_a_group_is_looking() {
    let g = main_game();
    let day_2 = g.no_orders(2, 1).join(" ");
    assert_says(
        &day_2,
        &[
            "The landing at (10,0) is ready",
            "air_group:2 is sent to (10,0) (range 300 nm, 240 nm away)",
            "does not release plan:1 until the need is met",
        ],
    );
    // Day 3: the landing was ordered again with unit 1 and is not ready.
    let day_3 = g.no_orders(3, 1).join(" ");
    assert_says(
        &day_3,
        &[
            "The landing at (10,0) is not ready",
            "ready on the morning of day 4",
        ],
    );
}

#[test]
fn the_need_each_morning() {
    let g = main_game();
    let need = |held, looking, moved, met| NeedToKnow {
        plan: PlanId::new(1),
        intent: intent(1),
        place: B,
        asked: Knowledge::Count,
        asked_from: None,
        held,
        looking,
        moved_to_look: moved,
        met,
    };
    let moved = Some(MovedToLook {
        task_force: TaskForceId::new(1),
        hex: COVER,
    });
    let seen_0 = |age_days| HeldCount {
        men: 0,
        grade: GroundGrade::SeenFromAir,
        day: 2,
        age_days,
        could_be: Some(CountRange {
            lowest: 0,
            highest: Some(0),
        }),
    };
    // Day 1: nothing held, nobody looking yet, the carrier moved to (4,0).
    assert_eq!(g.need(1), Some(&need(None, None, moved, false)));
    // Day 2: group 2 looking since day 2, from the carrier at (4,0).
    let group_2 = Some(Looking {
        air_group: AirGroupId::new(2),
        since_day: Some(2),
    });
    assert_eq!(g.need(2), Some(&need(None, group_2, moved, false)));
    // Days 3 and 4: 0 seen from the air of day 2, met; none looking.
    assert_eq!(g.need(3), Some(&need(Some(seen_0(1)), None, None, true)));
    assert_eq!(g.need(4), Some(&need(Some(seen_0(2)), None, None, true)));
    // Released on day 4: no need after.
    for day in 5..=9 {
        assert_eq!(g.need(day), None, "day {day}");
    }
}

#[test]
fn the_look_the_count_and_the_strikes() {
    let g = main_game();
    // Group 2 flies the reconnaissance once, on day 2, 240 nm, and reports
    // 0 men, seen from the air.
    assert_eq!(g.flights(), vec![(2, Some(Pulse::Morning), 2, n(240))]);
    assert_eq!(
        g.reports(),
        vec![(2, Some(Pulse::Morning), 0, GroundGrade::SeenFromAir)]
    );
    assert_eq!(g.sent_to_look(2), vec![2]);
    // Group 3 strikes base 2 with its 10 on days 4 to 7 and no other day.
    assert_eq!(
        g.base_strikes(3),
        (4..=7).map(|d| (d, BaseId::new(2), 10)).collect::<Vec<_>>()
    );
    assert!(g.base_strikes(1).is_empty());
    assert!(g.base_strikes(2).is_empty());
    // Base 2 changes side at the end of day 7; group 3's mission is cleared
    // to rest.
    assert_eq!(g.changed_side_day(2), Some(7));
    assert_eq!(g.group_at_end(3).mission, AirMission::Rest);
}

#[test]
fn the_hours_and_the_men_landed() {
    let g = main_game();
    // The carrier sails at hour 0 and arrives at hour 10; sails home at
    // hour 168 and arrives at hour 178.
    assert_eq!(
        g.steamed(1),
        vec![
            (1, Pulse::Night, A, COVER, n(160)),
            (8, Pulse::Night, COVER, A, n(160)),
        ]
    );
    assert_eq!(g.arrivals(1), vec![(COVER, n(10)), (A, n(178))]);
    // The transports sail at hour 120, the night of day 6, and arrive at
    // hour 160: 400 nm at 10 kn.
    let transports = g.steamed(2);
    assert_eq!(
        (transports[0].0, transports[0].1, transports[0].2),
        (6, Pulse::Night, A)
    );
    let out: Fixed = transports
        .iter()
        .take_while(|s| s.2 != B)
        .fold(Fixed::ZERO, |sum, s| sum + s.4);
    assert_eq!(out, n(400));
    assert_eq!(g.arrivals(2)[0], (B, n(160)));
    // Unit 1's ship lands 160 men in the morning and 440 in the afternoon
    // of day 7, then 10 t; the last 35 t in 1.75 hours on day 8.
    assert_eq!(
        g.landed(),
        vec![(7, Pulse::Morning, 1, 160), (7, Pulse::Afternoon, 1, 440),]
    );
    assert_eq!(
        g.supply_landed(),
        vec![(7, Pulse::Afternoon, n(10)), (8, Pulse::Night, n(35))]
    );
    assert_eq!(g.unloading_complete(), vec![fx("169.75")]);
}

// ----------------------------------------------------------------- Properties

fn at_risk(text: &str, risk: &str) -> String {
    format!(
        "{text}{}",
        command_table(
            1,
            &format!("kind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"{risk}\"")
        )
    )
}

fn without_leave_to_launch(text: &str) -> String {
    changed(
        text,
        "leave = \"launch\"\ngiven = true",
        "leave = \"launch\"\ngiven = false",
    )
}

/// Days 1 and 2 of scenario 73's game: no carrier task on day 1; on day 2
/// the plan released on no count, and the carrier task formed that morning
/// with its orders.
fn days_1_and_2_are_scenario_73s(g: &Played) {
    assert!(g.refused().is_empty(), "{:?}", g.refused());
    assert_eq!(
        g.commands(1),
        vec![
            Cmd::Approve(1),
            Cmd::Form(1, landing_kind()),
            Cmd::Limit(2, vec![1, 2])
        ]
    );
    assert_eq!(g.orders(1), vec![prepare(&[1, 2])]);
    assert_eq!(
        g.commands(2),
        vec![Cmd::Approve(1), Cmd::Form(1, cover_kind(COVER))]
    );
    assert_eq!(g.plan_day(1, PlanState::Released), Some(2));
    assert_eq!(g.landing(2), vec![load_to(90)]);
    let release = g.no_orders(2, 1).join(" ");
    assert!(
        release.is_empty() || !release.contains("is sent to"),
        "{release}"
    );
    assert!(g.sent_to_look(2).is_empty());
}

#[test]
fn the_scouts_not_named_the_game_of_scenario_73() {
    let g = play(&changed(&text(), "air_groups = [2]\n", ""));
    days_1_and_2_are_scenario_73s(&g);
    assert_eq!(g.carrier(2), carrier_formed_orders());
    assert_eq!(g.need(1).map(|need| need.moved_to_look), Some(None));
}

#[test]
fn at_press_nothing_need_be_known_and_the_task_is_formed_at_release() {
    let g = play(&at_risk(&text(), "press"));
    assert!(g.refused().is_empty(), "{:?}", g.refused());
    assert_eq!(
        g.commands(1),
        vec![
            Cmd::Approve(1),
            Cmd::Form(1, landing_kind()),
            Cmd::Limit(2, vec![1, 2])
        ]
    );
    assert!(g.carrier(1).is_empty(), "{:?}", g.carrier(1));
    assert_eq!(g.plan_day(1, PlanState::Released), Some(2));
    assert!(g.commands(2).contains(&Cmd::Form(1, cover_kind(COVER))));
    assert_eq!(g.carrier_formed(), vec![2]);
    assert!(g.sent_to_look(2).is_empty());
    assert!(g.flights().is_empty());
}

#[test]
fn at_preserve_the_force_with_leave_the_force_is_moved_on_day_1() {
    let g = play(&at_risk(&text(), "preserve"));
    assert!(g.refused().is_empty(), "{:?}", g.refused());
    assert!(g.commands(1).contains(&Cmd::Form(1, cover_kind(COVER))));
    assert_eq!(g.carrier_formed(), vec![1]);
    assert_eq!(
        g.carrier_tasks()[0],
        (3, cover_kind(COVER), Some(1), Maker::Staff)
    );
    assert!(
        g.carrier(1).contains(&route(1, vec![COVER], &[])),
        "{:?}",
        g.carrier(1)
    );
}

#[test]
fn at_preserve_the_force_without_leave_the_staff_says_where_it_could_look_from_and_waits() {
    let g = play(&without_leave_to_launch(&at_risk(&text(), "preserve")));
    assert!(g.refused().is_empty(), "{:?}", g.refused());
    for day in 1..=2 {
        assert!(
            !g.commands(day)
                .iter()
                .any(|c| matches!(c, Cmd::Form(_, IntentKind::OperateCarriers { .. }))),
            "day {day}"
        );
        assert!(
            !g.orders(day)
                .iter()
                .any(|o| matches!(o, Given::Force(1, _) | Given::Air(..))),
            "day {day}: {:?}",
            g.orders(day)
        );
        let reasons = g.no_orders(day, 1).join(" ");
        assert_says(
            &reasons,
            &[
                "task_force:1 could look from (4,0)",
                "waits for leave to launch",
            ],
        );
    }
    assert!(g.carrier_tasks().is_empty());
}

#[test]
fn already_in_reach_group_2_looks_on_day_1_and_no_carrier_task_is_formed() {
    let t = changed(
        &text(),
        "name = \"Placeholder carrier force 1\"\nside = \"allied\"\nat_base = 1\n",
        "name = \"Placeholder carrier force 1\"\nside = \"allied\"\nq = 4\nr = 0\n",
    );
    let g = play(&t);
    assert!(g.refused().is_empty(), "{:?}", g.refused());
    assert!(g
        .orders(1)
        .contains(&mission(2, AirMission::Reconnaissance(B))));
    assert!(
        !g.commands(1)
            .iter()
            .any(|c| matches!(c, Cmd::Form(_, IntentKind::OperateCarriers { .. }))),
        "{:?}",
        g.commands(1)
    );
    assert!(g.carrier(1).iter().all(|o| matches!(o, Given::Air(2, _))));
}

#[test]
fn the_enemy_sees_nothing_of_the_flight() {
    let g = main_game();
    let flown = |events: &[SideEvent]| {
        events
            .iter()
            .filter(|e| {
                matches!(
                    &e.kind,
                    SideEventKind::Whole(EventKind::ReconnaissanceFlown { .. })
                )
            })
            .count()
    };
    let reports = |events: &[SideEvent]| {
        events
            .iter()
            .filter(|e| {
                matches!(
                    &e.kind,
                    SideEventKind::Whole(EventKind::GroundReportReceived { .. })
                )
            })
            .count()
    };
    assert_eq!(flown(&g.allied_events), 1);
    assert_eq!(reports(&g.allied_events), 1);
    assert_eq!(flown(&g.japanese_events), 0);
    assert_eq!(reports(&g.japanese_events), 0);
}

#[test]
fn the_same_start_gives_the_same_orders() {
    let a = play(&text());
    assert_eq!(a.handed, main_game().handed);
    assert_eq!(a.needs, main_game().needs);
    assert_eq!(a.hashes, main_game().hashes);
    assert_eq!(a.state, main_game().state);
}

#[test]
fn what_the_side_cannot_see_and_has_not_been_told_changes_nothing() {
    let hidden = |ready: u32| {
        format!(
            "[[base]]\nid = 3\nname = \"Placeholder base 3\"\nside = \"japanese\"\nq = 10\nr = 2\nsupply_t = 50\non_map = false\n\n[[air_group]]\nid = 6\nname = \"Placeholder Japanese fighters 6\"\nside = \"japanese\"\nbase = 3\naircraft_type = 4\nready = {ready}\n"
        )
    };
    let with_enemy = |ready: u32| {
        changed(
            &text(),
            COMMANDS_MARKER,
            &format!("{}\n{COMMANDS_MARKER}", hidden(ready)),
        )
    };
    let a = play(&with_enemy(10));
    let b = play(&with_enemy(30));
    assert_ne!(
        a.truth.last(),
        b.truth.last(),
        "the two worlds do not differ"
    );
    assert_eq!(a.handed, b.handed);
    assert_eq!(a.needs, b.needs);
    for day in 1..=9 {
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
    let needs: BTreeMap<u32, Vec<NeedToKnow>> =
        (1..=3).map(|d| (d, through.needs[&d].clone())).collect();
    let resumed = play_from(at_3, first, needs);
    assert_eq!(resumed.hashes, through.hashes);
    assert_eq!(resumed.handed, through.handed);
    assert_eq!(resumed.needs, through.needs);
}

#[test]
fn scenario_73_forms_its_carrier_task_on_day_2_as_it_did() {
    // Scenario 73's own test asserts its game whole; here only that its
    // carrier is not moved before its plan is released: no group is named.
    let at = common::data_dir().join("scenarios/73_carrier_cover.toml");
    let text = std::fs::read_to_string(&at).expect("scenario 73 reads");
    let g = play_from(load_at(&text, &at), BTreeMap::new(), BTreeMap::new());
    assert!(g.carrier(1).is_empty());
    assert_eq!(g.carrier_formed(), vec![2]);
    assert_eq!(g.carrier(2), carrier_formed_orders());
}
