//! Scenario 54: a surface force that stands by night. The commander states
//! "deny the waters at (3,0)" and names task force 1. The staff gives the
//! force the task of standing there, and at the force's level sends it
//! out on the nights it believes in a threat and brings it home at
//! morning.
//!
//! Proves decision 0124 against a hand calculation. The data is placeholder
//! data: see `data/scenarios/54_a_force_that_stands_by_night.toml`. It
//! runs on the frozen step 1 figures, which give no fuelling rate, so a
//! ship in port is filled in the pulse. There is no enemy unit, no search
//! and no random draw.
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
//! A hex is 40 nm. A day is a night pulse of 12 hours and two daylight
//! pulses of 6; day d begins at hour 24 x (d - 1), with its night. The
//! placeholder cruiser (class 1): 16 kn at cruise, 2000 t of fuel over
//! 8000 nm, which is 0.25 t a mile at cruise. A force's position is the
//! nearest hex centre. A contact gains a pulse of age at the end of each
//! pulse and is dropped at 5.
//!
//! ## Inputs
//!
//! A placeholder map of open sea, with S at (0,0) a coast hex with a port.
//! Allied base 1 at S, with 5000 t of fuel. The waters are the hex (3,0),
//! 120 nm from S. The game is 4 days. There is no Japanese unit.
//!
//! Task force 1, Allied: two placeholder cruisers, ships 1 and 2, stopped
//! at base 1, full of fuel, on the surface combat mission, at normal
//! readiness, with no home port and the stance avoid.
//!
//! The Allied side starts holding one contact: 3 ships, a cruiser, at
//! (10,0), age 0. It is 7 hexes from the waters.
//!
//! The Allied side's commands of day 1: intent 1, deny the waters within 0
//! hexes of (3,0); its forces limit, task force 1; leave to prepare and
//! leave to launch both given. No risk is set: it is balanced.
//!
//! ## The tree on the first morning
//!
//! | Intent | Kind | Parent | Maker | Addressee |
//! | ------ | ---- | ------ | ----- | --------- |
//! | 1      | deny the waters at (3,0) | none | the commander | the area |
//! | 2      | stand at (3,0), task force 1 | 1 | the staff | task force 1 |
//!
//! Base 1 has no air group, so no strike task and no search task is
//! formed, and no notice is handed in: a task was formed.
//!
//! ## The sortie
//!
//! S to the station is 120 nm: 7.5 hours at 16 kn, inside the night's 12.
//! Out and home is 240 nm: 60 t for each ship. At balanced the force needs
//! 60 x 1.2 = 72 t in each ship.
//!
//! A sortie ordered on day d: the force sails at the start of the night,
//! reaches the station 7.5 hours into it and halts to stand. At the start
//! of the morning it steams on for S: in the morning's 6 hours it makes 96
//! nm and is 24 nm from S, nearest the centre of (1,0). It arrives 1.5
//! hours into the afternoon, 19.5 hours after it sailed, and each ship is
//! filled in that pulse.
//!
//! ## Day by day
//!
//! | Day | The contact at the staff's hour | Threat | The staff's orders to force 1 | End of night | End of morning | End of afternoon |
//! | --- | ------------------------------- | ------ | ----------------------------- | ------------ | -------------- | ---------------- |
//! | 1   | held, age 0 | yes | Home port, base 1. Rules of engagement: engage, break-off level 0.5. Battle readiness. Route to (3,0) and S, (3,0) marked to stand at | (3,0), halted | (1,0), under way | S, stopped, 2000 t in each ship |
//! | 2   | held, age 3 | yes | Route, as day 1 | (3,0), halted | (1,0) | S, stopped, 2000 t |
//! | 3   | dropped at the end of the morning of day 2 | no | Normal readiness | S | S | S |
//! | 4   | none | no | none | S | S | S |
//!
//! The staff gives 4, 1, 1 and 0 orders: 6 in all. On day 1 no speed order
//! is given: the force is at cruise. The straggler instruction and the
//! keep share in the rules of engagement order are those the force had.
//!
//! Each ship burns 60 t on each sortie and takes 60 t at S on the
//! afternoon of days 1 and 2. Base 1 holds 4880 t at the end of day 1 and
//! 4760 t from the end of day 2.
//!
//! The force records a halt at (3,0) at hour 7.5 and at hour 31.5, each
//! with the night as its work, and steams on at hours 12 and 36.
//!
//! Every order serves intent 2 and was decided at the level of task force
//! 1, with a chain of one: intent 1, the commander's. The reasons of days
//! 1 and 2 give the contact, 7 hexes from the station, 2000 t aboard
//! against 72 needed, and 7.5 hours to the station. The reasons of days 3
//! and 4 say there is no threat. No staff order is refused.
//!
//! # Properties
//!
//! - **Press.** With the risk of intent 1 set to press on day 1: the force
//!   sorties on all four nights, the break-off level ordered is 0.75, and
//!   it is never set back to normal readiness. Orders: 4, 1, 1, 1.
//! - **Preserve the force.** The contact reports 3 ships and the force has
//!   2: there is no threat that counts, no sortie on any day, and the
//!   reason gives 3 against 2. Day 1 has two orders, the home port and the
//!   rules of engagement with a break-off level of 0.25. With the contact
//!   reporting 2 ships, the force sorties on days 1 and 2, needing 90 t in
//!   each ship.
//! - **Fuel comes first.** With each ship holding 70 t and base 1 holding
//!   5000 t: on day 1 there is no sortie, and the reason gives 70 against
//!   72 and day 2 as the day from which the force may be used. The ships
//!   are filled in the night of day 1, and the force sorties on day 2.
//!   With base 1 holding no fuel, there is no sortie on any day, the
//!   reason gives no day, and the staff hands in on day 1 that it cannot
//!   meet intent 2.
//! - **Full speed where it must.** With the waters at (6,0), 240 nm: 15
//!   hours at cruise, and 7.5 at the class's 32 kn. The force is ordered
//!   to full speed and reaches the station 7.5 hours into the night. With
//!   the waters at (13,0), 520 nm, 16.25 hours at 32 kn: no sortie, and
//!   the notice.
//! - **Cover from the sea.** With the commander stating "hold base 1" with
//!   a forces limit of task force 1 in place of the deny intent: the stand
//!   task's station is S. The force is given no route on any day; battle
//!   readiness on day 1, normal on day 3; the home port and the rules of
//!   engagement on day 1. The contact is 10 hexes from S.
//! - **No force named, no task.** With no forces limit on intent 1, no
//!   stand task is formed and force 1 is given no order.
//! - **The rule alone.** A force of any mission routed through a waypoint
//!   marked to stand at halts there if it reaches it in the night and
//!   steams on at morning; if it reaches it in a daylight pulse it passes;
//!   and a route with no such mark moves as it did. A side asking the
//!   choice beforehand is told the same.
//! - **A held force is left alone,** and a commander's route order to
//!   force 1 holds it.
//! - **One task a force.** With the commander also stating, on day 1,
//!   "stand at (3,0), task force 1" as intent 2, beside the deny end: the
//!   staff forms no stand task under intent 1, and its reason names intent
//!   2. The orders are the main case's, each serving intent 2 with an empty
//!   chain. (Added by the architect on Nov 16, 2026, with the ruling that a
//!   force has one task of a kind, whoever made it.)
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see give the same intents, orders and
//!   reasons.
//! - Every scenario built before this one gives the events and end states
//!   it gave.

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::choices::movement::{MovementAnswer, MovementQuestion};
use sim_core::choices::{Answer, Moment, Question};
use sim_core::command::{
    Addressee, IntentKind, Maker, NoticeKind, Reason, StaffDecisions, StaffOrder,
};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, ShipId, TaskForceId};
use sim_core::orders::{
    OrderKind, Readiness, SpeedSetting, Stance, StragglerInstruction, TaskForceOrder,
};
use sim_core::state::{HaltWork, Side, TaskForceStatus, TruthState};
use sim_core::time::Pulse;
use sim_data::Scenario;
use sim_staff::stand::{break_off, margin, threat, usable_day, Sighted, THREAT_HEXES};
use sim_staff::{as_staffs, staffs};

mod common;

const COMMANDS_MARKER: &str = "# The Allied commander's commands";

fn path() -> PathBuf {
    common::data_dir().join("scenarios/54_a_force_that_stands_by_night.toml")
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

fn base(raw: u32) -> BaseId {
    BaseId::new(raw)
}

fn force(raw: u32) -> TaskForceId {
    TaskForceId::new(raw)
}

fn ship(raw: u32) -> ShipId {
    ShipId::new(raw)
}

fn hex(q: i32, r: i32) -> Hex {
    Hex { q, r }
}

fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

/// A command table of the Allied side.
fn command_table(day: u32, body: &str) -> String {
    format!("\n[[command]]\nday = {day}\nside = \"allied\"\n{body}\n")
}

fn with(extra: &str) -> String {
    format!("{}{}", text(), extra)
}

fn risk(level: &str) -> String {
    command_table(
        1,
        &format!("kind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"{level}\""),
    )
}

/// The file without any of the commander's commands, for the tests of the
/// rule alone and of a commander's own intents.
fn without_commands() -> String {
    let t = text();
    let at = t.find(COMMANDS_MARKER).expect("the marker");
    t[..at].to_string()
}

/// A game played to its end with the doctrine's staffs: what the Allied staff
/// handed in each day, the truth at the end of each day (index 0 is the
/// start), the log and the side's command state at the end.
struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    truth: Vec<TruthState>,
    events: Vec<Event>,
    state: sim_core::command::CommandState,
}

fn play(text: &str) -> Played {
    play_watched(text, &mut |_, _| {})
}

fn play_watched(text: &str, watch: &mut dyn FnMut(&sim_core::world::World, Moment)) -> Played {
    let mut scenario: Scenario = common::from_text(text, &path()).expect("the scenario loads");
    let held = staffs();
    let mut handed = BTreeMap::new();
    let mut truth = vec![scenario.world.umpire().clone()];
    loop {
        let day = scenario.world.day() + 1;
        let summary = scenario
            .next_day_watched(as_staffs(&held), watch)
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
        state: scenario.world.command_state(Side::Allied).clone(),
    }
}

fn tf_order(o: &StaffOrder) -> TaskForceOrder {
    match &o.kind {
        OrderKind::TaskForce { target, order } => {
            assert_eq!(*target, force(1), "an order to another force");
            order.clone()
        }
        other => panic!("the doctrine gave a non-task-force order: {other:?}"),
    }
}

fn route(waypoints: &[(i32, i32)], stand: &[(i32, i32)]) -> TaskForceOrder {
    TaskForceOrder::SetRoute {
        waypoints: waypoints.iter().map(|(q, r)| hex(*q, *r)).collect(),
        act_at: Default::default(),
        stand_at: stand.iter().map(|(q, r)| hex(*q, *r)).collect(),
    }
}

/// The route of the main case: to the station and home, the station marked.
fn sortie() -> TaskForceOrder {
    route(&[(3, 0), (0, 0)], &[(3, 0)])
}

fn engage(break_off: &str) -> TaskForceOrder {
    TaskForceOrder::SetRulesOfEngagement {
        stance: Stance::Engage,
        break_off: fx(break_off),
        stragglers: StragglerInstruction::Keep,
        keep_share: Fixed::ZERO,
    }
}

const BATTLE: TaskForceOrder = TaskForceOrder::SetReadiness(Readiness::Battle);
const NORMAL: TaskForceOrder = TaskForceOrder::SetReadiness(Readiness::Normal);

fn home_port() -> TaskForceOrder {
    TaskForceOrder::SetHomePort(base(1))
}

/// One halt, as (day, pulse, hex, work, hours).
type Halt = (u32, Option<Pulse>, Hex, HaltWork, Fixed);

fn night_work() -> HaltWork {
    HaltWork {
        bombard: false,
        cargo: false,
        night: true,
    }
}

impl Played {
    fn decisions(&self, day: u32) -> &StaffDecisions {
        &self.handed[&day]
    }

    fn orders(&self, day: u32) -> Vec<TaskForceOrder> {
        self.decisions(day).orders.iter().map(tf_order).collect()
    }

    fn all_orders(&self) -> Vec<(u32, &StaffOrder)> {
        self.handed
            .iter()
            .flat_map(|(day, d)| d.orders.iter().map(move |o| (*day, o)))
            .collect()
    }

    fn order_count(&self) -> Vec<usize> {
        (1..=self.handed.len() as u32)
            .map(|d| self.decisions(d).orders.len())
            .collect()
    }

    fn all(&self, pick: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
        self.events.iter().filter(|e| pick(&e.kind)).collect()
    }

    fn nothing_refused(&self) {
        let refused = self.all(|k| {
            matches!(
                k,
                EventKind::OrderRejected { .. } | EventKind::CommandRefused { .. }
            )
        });
        assert!(refused.is_empty(), "something was refused: {refused:?}");
    }

    /// Every halt, in log order.
    fn halts(&self) -> Vec<Halt> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceHalted {
                    hex,
                    work,
                    elapsed_hours,
                    ..
                } => Some((e.day, e.pulse, *hex, *work, *elapsed_hours)),
                _ => None,
            })
            .collect()
    }

    /// Every steaming on, as (day, pulse, hex).
    fn steamed_on(&self) -> Vec<(u32, Option<Pulse>, Hex)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceSteamedOn { hex, work, .. } => {
                    assert!(work.night, "steamed on from something else");
                    Some((e.day, e.pulse, *hex))
                }
                _ => None,
            })
            .collect()
    }

    /// The hex the force is in at the end of a pulse, from the log: where
    /// the last movement of or before the pulse left it (the start is S).
    fn position_after(&self, day: u32, pulse: Pulse) -> Hex {
        let order = |p: Pulse| match p {
            Pulse::Night => 0,
            Pulse::Morning => 1,
            Pulse::Afternoon => 2,
        };
        let mut at = self.truth[0].task_forces[&force(1)].position();
        for e in &self.events {
            let Some(p) = e.pulse else { continue };
            if (e.day, order(p)) > (day, order(pulse)) {
                break;
            }
            if let EventKind::TaskForceSteamed { to, .. } = &e.kind {
                at = *to;
            }
        }
        at
    }

    /// Whether the force is halted at the end of a pulse: its last halt or
    /// steaming on, at or before the pulse, was a halt.
    fn halted_after(&self, day: u32, pulse: Pulse) -> bool {
        let order = |p: Pulse| match p {
            Pulse::Night => 0,
            Pulse::Morning => 1,
            Pulse::Afternoon => 2,
        };
        let mut halted = false;
        for e in &self.events {
            let Some(p) = e.pulse else { continue };
            if (e.day, order(p)) > (day, order(pulse)) {
                break;
            }
            match &e.kind {
                EventKind::TaskForceHalted { .. } => halted = true,
                EventKind::TaskForceSteamedOn { .. } => halted = false,
                _ => {}
            }
        }
        halted
    }

    /// The pulses of a day in which the force steamed.
    fn steamed_in(&self, day: u32) -> Vec<Option<Pulse>> {
        self.events
            .iter()
            .filter(|e| e.day == day && matches!(e.kind, EventKind::TaskForceSteamed { .. }))
            .map(|e| e.pulse)
            .collect()
    }

    /// The arrivals of the force, as (day, pulse, hex, hours).
    fn arrivals(&self) -> Vec<(u32, Option<Pulse>, Hex, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceArrived {
                    hex, elapsed_hours, ..
                } => Some((e.day, e.pulse, *hex, *elapsed_hours)),
                _ => None,
            })
            .collect()
    }

    /// The ships' refuelling, as (day, pulse, ship, taken, fuel after, base
    /// fuel after).
    fn refuelled(&self) -> Vec<(u32, Option<Pulse>, u32, Fixed, Fixed, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::ShipRefuelled {
                    ship,
                    taken_t,
                    fuel_t,
                    base_fuel_t,
                    ..
                } => Some((e.day, e.pulse, ship.raw(), *taken_t, *fuel_t, *base_fuel_t)),
                _ => None,
            })
            .collect()
    }

    fn fuel_at_end_of(&self, day: usize, raw: u32) -> Fixed {
        self.truth[day].ships[&ship(raw)].fuel_t
    }

    fn base_fuel_at_end_of(&self, day: usize) -> Fixed {
        self.truth[day].bases[&base(1)].fuel_t
    }

    fn force_at_end_of(&self, day: usize) -> &sim_core::state::TaskForce {
        &self.truth[day].task_forces[&force(1)]
    }

    fn cannot_meet(&self, day: u32) -> usize {
        self.decisions(day)
            .notices
            .iter()
            .filter(|n| n.kind == NoticeKind::CannotMeet)
            .count()
    }

    fn has_kind(&self, kind: &IntentKind) -> bool {
        self.state.intents.values().any(|i| &i.kind == kind)
    }

    /// Every reason's chain, followed to its end, reaches an intent the
    /// commander made.
    fn every_chain_ends_with_the_commander(&self) {
        for (day, order) in self.all_orders() {
            let reason = &order.reason;
            let served = reason.serves.intent.expect("a reason serves an intent");
            match reason.serves.chain.last() {
                Some(link) => {
                    assert_eq!(link.maker, Maker::Commander, "day {day}: {reason:?}");
                    assert!(!link.statement.is_empty(), "day {day}");
                    assert_eq!(
                        Some(link.intent),
                        Some(self.state.root_of(served)),
                        "day {day}"
                    );
                }
                None => assert_eq!(
                    self.state.intents[&served].maker,
                    Maker::Commander,
                    "day {day}"
                ),
            }
        }
    }
}

fn level(reason: &Reason) -> Option<Addressee> {
    reason.serves.level
}

/// A reason's text: its calculation and its evidence.
fn said(reason: &Reason) -> String {
    let evidence: Vec<&str> = reason.evidence.iter().map(|e| e.what.as_str()).collect();
    format!("{} | {}", reason.worked, evidence.join(" | "))
}

/// The reason of the order of a day whose order is `wanted`.
fn reason_of<'a>(played: &'a Played, day: u32, wanted: &TaskForceOrder) -> &'a Reason {
    &played
        .decisions(day)
        .orders
        .iter()
        .find(|o| &tf_order(o) == wanted)
        .unwrap_or_else(|| panic!("day {day} has no order {wanted:?}"))
        .reason
}

/// Every reason the staff gave on a day: the orders' and the decisions to
/// give none.
fn reasons_of(played: &Played, day: u32) -> Vec<&Reason> {
    let d = played.decisions(day);
    d.orders
        .iter()
        .map(|o| &o.reason)
        .chain(d.no_orders.iter())
        .collect()
}

// ------------------------------------------------------- The hand calculation

/// What the Allied side holds at the staff's hour of each day: (hex, ships,
/// age) of each contact, read at the moment the day's orders are taken in.
fn contacts_at_the_staffs_hour(text: &str) -> BTreeMap<u32, Vec<(Hex, u32, u32)>> {
    let mut held: BTreeMap<u32, Vec<(Hex, u32, u32)>> = BTreeMap::new();
    play_watched(text, &mut |world, moment| {
        if let Moment::Orders { day } = moment {
            held.insert(
                day,
                world
                    .side_view(Side::Allied)
                    .contacts()
                    .map(|c| (c.estimated_hex, c.estimated_ship_count, c.age_pulses))
                    .collect(),
            );
        }
    });
    held
}

#[test]
fn the_commands_of_the_file_are_the_ones_these_tests_vary() {
    let t = text();
    let at = t.find(COMMANDS_MARKER).expect("the marker");
    let c = &t[at..];
    assert_eq!(c.matches("[[command]]").count(), 4);
    assert!(c.contains("intent_kind = \"deny\"\nhex = [3, 0]\ndistance = 0\n"));
    assert!(c.contains("limit = \"forces\"\nintent = 1\ntask_forces = [1]\n"));
    assert!(!c.contains("risk"));
    assert!(!c.contains("given = false"));
}

#[test]
fn the_contact_at_the_staffs_hour_of_each_day() {
    let held = contacts_at_the_staffs_hour(&text());
    // Day 1: held, age 0, 3 ships at (10,0). Day 2: age 3. Day 3 and 4: none
    // (dropped at 5, at the end of the morning of day 2).
    assert_eq!(held[&1], vec![(hex(10, 0), 3, 0)]);
    assert_eq!(held[&2], vec![(hex(10, 0), 3, 3)]);
    assert!(held[&3].is_empty(), "{:?}", held[&3]);
    assert!(held[&4].is_empty(), "{:?}", held[&4]);
    // It is 7 hexes from the waters and 10 from S.
    assert_eq!(hex(10, 0).distance(hex(3, 0)), 7);
    assert_eq!(hex(10, 0).distance(hex(0, 0)), 10);
}

#[test]
fn the_tree_on_the_first_morning() {
    let played = play(&text());
    played.nothing_refused();
    let state = &played.state;
    assert_eq!(state.intents.len(), 2);
    let one = &state.intents[&intent(1)];
    assert_eq!(
        one.kind,
        IntentKind::Deny {
            hex: hex(3, 0),
            distance: 0
        }
    );
    assert_eq!(one.parent, None);
    assert_eq!(one.maker, Maker::Commander);
    assert_eq!(one.addressee, Addressee::Area);
    assert_eq!(one.set_day, 1);
    let two = &state.intents[&intent(2)];
    assert_eq!(
        two.kind,
        IntentKind::StandAt {
            force: force(1),
            hex: hex(3, 0)
        }
    );
    assert_eq!(two.parent, Some(intent(1)));
    assert_eq!(two.maker, Maker::Staff);
    assert_eq!(two.addressee, Addressee::TaskForce(force(1)));
    assert_eq!(two.plan, one.plan);
    assert_eq!(two.set_day, 1);
    assert_eq!(two.withdrawn_day, None);
    // One task is formed, on day 1, by the staff, under intent 1.
    let formed = played.all(|k| matches!(k, EventKind::IntentFormed { .. }));
    assert_eq!(formed.len(), 1);
    assert_eq!(formed[0].day, 1);
    // Base 1 has no air group, so no strike task and no search task is
    // formed, and no notice is handed in: a task was formed.
    assert!(!played.has_kind(&IntentKind::StrikeShips { base: base(1) }));
    assert!(!played.has_kind(&IntentKind::SearchWaters { base: base(1) }));
    for day in 1..=4 {
        assert!(played.decisions(day).notices.is_empty(), "day {day}");
    }
}

#[test]
fn the_staff_gives_4_1_1_and_0_orders_6_in_all() {
    let played = play(&text());
    played.nothing_refused();
    assert_eq!(played.order_count(), vec![4, 1, 1, 0]);
    assert_eq!(played.all_orders().len(), 6);
    // Day 1: the home port, the rules of engagement (engage, break-off level
    // 0.5; the straggler instruction and the keep share those the force
    // had), battle readiness, and the route to (3,0) and S with (3,0) marked
    // to stand at. No speed order: the force is at cruise.
    let before = &played.truth[0].task_forces[&force(1)];
    let had = before.rules_of_engagement;
    assert_eq!(
        played.orders(1),
        vec![
            home_port(),
            TaskForceOrder::SetRulesOfEngagement {
                stance: Stance::Engage,
                break_off: fx("0.5"),
                stragglers: had.stragglers,
                keep_share: had.keep_share,
            },
            BATTLE,
            sortie(),
        ]
    );
    assert_eq!(had.stance, Stance::Avoid);
    assert_eq!(had.break_off, fx("0.5"));
    assert_eq!(had.stragglers, StragglerInstruction::Keep);
    assert_eq!(before.home_port, None);
    assert_eq!(before.speed, SpeedSetting::Cruise);
    assert_eq!(before.readiness, Readiness::Normal);
    // Day 2: the route, as day 1. Day 3: normal readiness. Day 4: none.
    assert_eq!(played.orders(2), vec![sortie()]);
    assert_eq!(played.orders(3), vec![NORMAL]);
    assert!(played.orders(4).is_empty());
    for day in 1..=4 {
        for o in &played.decisions(day).orders {
            assert!(
                !matches!(
                    tf_order(o),
                    TaskForceOrder::SetSpeed(_) | TaskForceOrder::SetMission(_)
                ),
                "day {day}"
            );
        }
    }
}

#[test]
fn the_force_ends_with_the_orders_it_was_given() {
    let played = play(&text());
    let f = played.force_at_end_of(4);
    assert_eq!(f.home_port, Some(base(1)));
    assert_eq!(f.readiness, Readiness::Normal);
    assert_eq!(f.rules_of_engagement.stance, Stance::Engage);
    assert_eq!(f.rules_of_engagement.break_off, fx("0.5"));
    assert_eq!(f.speed, SpeedSetting::Cruise);
    // Battle readiness stood from day 1 to day 3.
    assert_eq!(played.force_at_end_of(1).readiness, Readiness::Battle);
    assert_eq!(played.force_at_end_of(2).readiness, Readiness::Battle);
    assert_eq!(played.force_at_end_of(3).readiness, Readiness::Normal);
}

#[test]
fn the_force_is_at_the_station_halted_at_the_end_of_each_night_of_days_1_and_2() {
    let played = play(&text());
    for day in [1, 2] {
        assert_eq!(
            played.position_after(day, Pulse::Night),
            hex(3, 0),
            "day {day}"
        );
        assert!(played.halted_after(day, Pulse::Night), "day {day}");
        // At the start of the morning it steams on: in the morning's 6
        // hours it makes 96 nm and is 24 nm from S, nearest the centre of
        // (1,0), still under way.
        assert_eq!(
            played.position_after(day, Pulse::Morning),
            hex(1, 0),
            "day {day}"
        );
        assert!(!played.halted_after(day, Pulse::Morning), "day {day}");
        // It arrives in the afternoon and is at S.
        assert_eq!(
            played.position_after(day, Pulse::Afternoon),
            hex(0, 0),
            "day {day}"
        );
        assert_eq!(
            played.steamed_in(day),
            vec![
                Some(Pulse::Night),
                Some(Pulse::Morning),
                Some(Pulse::Afternoon)
            ]
        );
    }
    // The distances of the three movements: 120, 96 and 24 nm at 16 knots.
    let miles: Vec<(u32, Option<Pulse>, Fixed, Fixed)> = played
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::TaskForceSteamed {
                distance_nm,
                speed_kn,
                ..
            } => Some((e.day, e.pulse, *distance_nm, *speed_kn)),
            _ => None,
        })
        .collect();
    let expected: Vec<(u32, Option<Pulse>, Fixed, Fixed)> = [1, 2]
        .into_iter()
        .flat_map(|d| {
            [
                (d, Some(Pulse::Night), n(120), n(16)),
                (d, Some(Pulse::Morning), n(96), n(16)),
                (d, Some(Pulse::Afternoon), n(24), n(16)),
            ]
        })
        .collect();
    assert_eq!(miles, expected);
}

#[test]
fn the_force_arrives_1_and_a_half_hours_into_the_afternoon_19_and_a_half_hours_after_it_sailed() {
    let played = play(&text());
    assert_eq!(
        played.arrivals(),
        vec![
            (1, Some(Pulse::Afternoon), hex(0, 0), fx("19.5")),
            (2, Some(Pulse::Afternoon), hex(0, 0), fx("43.5")),
        ]
    );
}

#[test]
fn it_halts_at_the_station_at_hour_7_and_a_half_and_31_and_a_half_and_steams_on_in_the_morning() {
    let played = play(&text());
    assert_eq!(
        played.halts(),
        vec![
            (1, Some(Pulse::Night), hex(3, 0), night_work(), fx("7.5")),
            (2, Some(Pulse::Night), hex(3, 0), night_work(), fx("31.5")),
        ]
    );
    // Steaming on at the start of the morning's movement, hours 12 and 36.
    assert_eq!(
        played.steamed_on(),
        vec![
            (1, Some(Pulse::Morning), hex(3, 0)),
            (2, Some(Pulse::Morning), hex(3, 0)),
        ]
    );
    // The halt is of the night alone: no bombardment, no cargo.
    for (_, _, _, work, _) in played.halts() {
        assert!(work.night && !work.bombard && !work.cargo);
    }
}

#[test]
fn each_ship_burns_60_t_a_sortie_and_takes_60_t_at_s_on_the_afternoon_and_base_1_ends_at_4760() {
    let played = play(&text());
    // Burnt in each of the three movements: 30, 24 and 6, which is 60 in all.
    let burned: Vec<(u32, Option<Pulse>, u32, Fixed)> = played
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::FuelBurned { ship, burned_t, .. } => {
                Some((e.day, e.pulse, ship.raw(), *burned_t))
            }
            _ => None,
        })
        .collect();
    for day in [1, 2] {
        for s in [1, 2] {
            let of: Vec<Fixed> = burned
                .iter()
                .filter(|(d, _, sh, _)| *d == day && *sh == s)
                .map(|(_, _, _, t)| *t)
                .collect();
            assert_eq!(of, vec![n(30), n(24), n(6)], "day {day} ship {s}");
        }
    }
    assert_eq!(burned.iter().filter(|(d, _, _, _)| *d > 2).count(), 0);
    // Filled in the afternoon pulse of each sortie.
    assert_eq!(
        played.refuelled(),
        vec![
            (1, Some(Pulse::Afternoon), 1, n(60), n(2000), n(4940)),
            (1, Some(Pulse::Afternoon), 2, n(60), n(2000), n(4880)),
            (2, Some(Pulse::Afternoon), 1, n(60), n(2000), n(4820)),
            (2, Some(Pulse::Afternoon), 2, n(60), n(2000), n(4760)),
        ]
    );
    // The end of each day: S, stopped, 2000 t in each ship; the base 4880,
    // 4760, 4760, 4760.
    for (day, base_fuel) in [(1, 4880), (2, 4760), (3, 4760), (4, 4760)] {
        let f = played.force_at_end_of(day);
        assert_eq!(f.position(), hex(0, 0), "day {day}");
        assert_eq!(f.toward_hex, None, "day {day}");
        assert!(
            !matches!(
                f.status,
                TaskForceStatus::Underway | TaskForceStatus::Halted
            ),
            "day {day}: {:?}",
            f.status
        );
        assert_eq!(played.fuel_at_end_of(day, 1), n(2000), "day {day}");
        assert_eq!(played.fuel_at_end_of(day, 2), n(2000), "day {day}");
        assert_eq!(played.base_fuel_at_end_of(day), n(base_fuel), "day {day}");
    }
    assert_eq!(played.base_fuel_at_end_of(0), n(5000));
}

#[test]
fn nothing_moves_on_days_3_and_4() {
    let played = play(&text());
    for day in [3, 4] {
        assert!(played.steamed_in(day).is_empty(), "day {day}");
        for pulse in [Pulse::Night, Pulse::Morning, Pulse::Afternoon] {
            assert_eq!(played.position_after(day, pulse), hex(0, 0));
            assert!(!played.halted_after(day, pulse));
        }
    }
}

#[test]
fn every_order_serves_intent_2_decided_at_the_level_of_task_force_1_with_a_chain_of_one() {
    let played = play(&text());
    for (day, order) in played.all_orders() {
        let reason = &order.reason;
        assert_eq!(order.intent, intent(2), "day {day}");
        assert_eq!(reason.serves.intent, Some(intent(2)), "day {day}");
        assert!(reason.serves.plan.is_some(), "day {day}");
        assert_eq!(
            level(reason),
            Some(Addressee::TaskForce(force(1))),
            "day {day}"
        );
        assert!(
            reason
                .serves
                .to_string()
                .contains("decided at the level of task_force:1"),
            "{}",
            reason.serves
        );
        assert_eq!(reason.serves.chain.len(), 1, "{:?}", reason.serves);
        let link = &reason.serves.chain[0];
        assert_eq!(link.intent, intent(1));
        assert_eq!(link.maker, Maker::Commander);
        assert!(link.statement.contains("deny"), "{}", link.statement);
        assert!(!reason.worked.is_empty());
        assert!(!reason.serves.statement.is_empty());
    }
    played.every_chain_ends_with_the_commander();
}

#[test]
fn the_reasons_of_days_1_and_2_give_the_contact_the_fuel_and_the_hours() {
    let played = play(&text());
    for day in [1, 2] {
        let route = said(reason_of(&played, day, &sortie()));
        // The contact, 7 hexes from the station.
        assert!(
            route.contains("contact 1 of 3 ship(s) is 7 hexes from the station (3,0)"),
            "{route}"
        );
        // 2000 t aboard against 72 needed, for each ship.
        assert!(
            route.contains("ship:1 has 2000 t aboard against 72 t needed"),
            "{route}"
        );
        assert!(
            route.contains("ship:2 has 2000 t aboard against 72 t needed"),
            "{route}"
        );
        // 7.5 hours to the station.
        assert!(route.contains("7.5 hours to the station"), "{route}");
        // Its evidence, too, carries the three figures.
        let evidence: Vec<&str> = reason_of(&played, day, &sortie())
            .evidence
            .iter()
            .map(|e| e.what.as_str())
            .collect();
        assert!(
            evidence.iter().any(|w| w.contains("7 hexes")),
            "{evidence:?}"
        );
        assert!(
            evidence
                .iter()
                .any(|w| w.contains("2000 t aboard against 72 t")),
            "{evidence:?}"
        );
        assert!(
            evidence.iter().any(|w| w.contains("7.5 hours")),
            "{evidence:?}"
        );
        // Readiness is set on day 1, and says the contact too.
        if day == 1 {
            let battle = said(reason_of(&played, 1, &BATTLE));
            assert!(battle.contains("7 hexes"), "{battle}");
        }
    }
}

#[test]
fn the_reasons_of_days_3_and_4_say_there_is_no_threat() {
    let played = play(&text());
    let on_3 = said(reason_of(&played, 3, &NORMAL));
    assert!(
        on_3.contains("no contact of ships within 15 hexes"),
        "{on_3}"
    );
    assert!(on_3.contains("there is no threat"), "{on_3}");
    // Day 4 gives no order; the decision to give none carries the reason.
    let on_4: Vec<String> = reasons_of(&played, 4).into_iter().map(said).collect();
    assert!(
        on_4.iter().any(|r| r.contains("there is no threat")),
        "{on_4:?}"
    );
    // The decision that says so serves intent 2, at the level of task force 1.
    let said_so = reasons_of(&played, 4)
        .into_iter()
        .find(|r| said(r).contains("there is no threat"))
        .expect("a reason");
    assert_eq!(said_so.serves.intent, Some(intent(2)));
    assert_eq!(level(said_so), Some(Addressee::TaskForce(force(1))));
}

#[test]
fn no_staff_order_is_refused() {
    play(&text()).nothing_refused();
}

#[test]
fn the_pure_parts_of_the_doctrine_are_the_tables_of_the_decision() {
    use sim_core::command::Risk;
    assert_eq!(THREAT_HEXES, 15);
    assert_eq!(margin(Risk::Preserve), fx("1.5"));
    assert_eq!(margin(Risk::Balanced), fx("1.2"));
    assert_eq!(margin(Risk::Press), n(1));
    assert_eq!(break_off(Risk::Preserve), fx("0.25"));
    assert_eq!(break_off(Risk::Balanced), fx("0.5"));
    assert_eq!(break_off(Risk::Press), fx("0.75"));
    // 60 t out and home, times the margin: 90, 72 and 60.
    assert_eq!(n(60) * margin(Risk::Preserve), n(90));
    assert_eq!(n(60) * margin(Risk::Balanced), n(72));
    assert_eq!(n(60) * margin(Risk::Press), n(60));
    // The contact: 3 ships, 7 hexes. It is a threat at balanced and at
    // press, and at preserve the force only against a force of 3 or more.
    let three = Sighted {
        number: 1,
        hexes: 7,
        ships: 3,
    };
    assert_eq!(threat(Risk::Balanced, 2, &[three]), Some(three));
    assert_eq!(threat(Risk::Press, 2, &[three]), Some(three));
    assert_eq!(threat(Risk::Preserve, 2, &[three]), None);
    assert_eq!(threat(Risk::Preserve, 3, &[three]), Some(three));
    let two = Sighted { ships: 2, ..three };
    assert_eq!(threat(Risk::Preserve, 2, &[two]), Some(two));
    // Beyond 15 hexes it is none.
    let far = Sighted { hexes: 16, ..three };
    assert_eq!(threat(Risk::Balanced, 2, &[far]), None);
    // The day from which a force that fuels in the night may be used.
    assert_eq!(usable_day(1, n(12)), 2);
}

// --------------------------------------------------------------- Properties

#[test]
fn press_sorties_on_all_four_nights_with_a_break_off_level_of_three_quarters() {
    let played = play(&with(&risk("press")));
    played.nothing_refused();
    assert_eq!(played.order_count(), vec![4, 1, 1, 1]);
    assert_eq!(
        played.orders(1),
        vec![home_port(), engage("0.75"), BATTLE, sortie()]
    );
    for day in 2..=4 {
        assert_eq!(played.orders(day), vec![sortie()], "day {day}");
    }
    // It is never set back to normal readiness.
    for (day, order) in played.all_orders() {
        assert_ne!(tf_order(order), NORMAL, "day {day}");
    }
    assert_eq!(played.force_at_end_of(4).readiness, Readiness::Battle);
    assert_eq!(
        played.halts(),
        vec![
            (1, Some(Pulse::Night), hex(3, 0), night_work(), fx("7.5")),
            (2, Some(Pulse::Night), hex(3, 0), night_work(), fx("31.5")),
            (3, Some(Pulse::Night), hex(3, 0), night_work(), fx("55.5")),
            (4, Some(Pulse::Night), hex(3, 0), night_work(), fx("79.5")),
        ]
    );
    assert_eq!(played.steamed_on().len(), 4);
    // At press the fuel needed is the passage once: 60 t.
    let on_1 = said(reason_of(&played, 1, &sortie()));
    assert!(on_1.contains("aboard against 60 t needed"), "{on_1}");
    assert!(on_1.contains("the force sorties every night"), "{on_1}");
    // On the nights of days 3 and 4 the contact is gone and the force goes
    // all the same.
    let on_3 = said(reason_of(&played, 3, &sortie()));
    assert!(on_3.contains("there is no threat"), "{on_3}");
    assert!(on_3.contains("sorties every night"), "{on_3}");
    played.every_chain_ends_with_the_commander();
}

#[test]
fn preserve_the_force_does_not_sortie_against_3_ships_with_2() {
    let played = play(&with(&risk("preserve")));
    played.nothing_refused();
    // The contact reports 3 ships and the force has 2: no threat that
    // counts, no sortie on any day.
    assert!(played.halts().is_empty());
    assert!(played.arrivals().is_empty());
    for day in 1..=4 {
        assert!(played.steamed_in(day).is_empty(), "day {day}");
    }
    // Day 1 has two orders, the home port and the rules of engagement with a
    // break-off level of 0.25. Nothing after.
    assert_eq!(played.order_count(), vec![2, 0, 0, 0]);
    assert_eq!(played.orders(1), vec![home_port(), engage("0.25")]);
    // The reason gives 3 against 2.
    let none: Vec<String> = played.decisions(1).no_orders.iter().map(said).collect();
    assert!(
        none.iter()
            .any(|r| r.contains("reports 3 ship(s) against the 2 task_force:1 has afloat")),
        "{none:?}"
    );
    assert_eq!(played.force_at_end_of(4).readiness, Readiness::Normal);
}

#[test]
fn preserve_the_force_sorties_against_2_ships_needing_90_t_in_each_ship() {
    let text = changed(&with(&risk("preserve")), "ship_count = 3", "ship_count = 2");
    let played = play(&text);
    played.nothing_refused();
    assert_eq!(
        played.orders(1),
        vec![home_port(), engage("0.25"), BATTLE, sortie()]
    );
    assert_eq!(played.orders(2), vec![sortie()]);
    for day in [1, 2] {
        let r = said(reason_of(&played, day, &sortie()));
        assert!(
            r.contains("ship:1 has 2000 t aboard against 90 t needed"),
            "{r}"
        );
        assert!(
            r.contains("ship:2 has 2000 t aboard against 90 t needed"),
            "{r}"
        );
    }
    assert_eq!(
        played
            .halts()
            .iter()
            .map(|h| (h.0, h.4))
            .collect::<Vec<_>>(),
        vec![(1, fx("7.5")), (2, fx("31.5"))]
    );
}

fn seventy_t() -> String {
    // The ships' fuel only: the base's 5000 t is a different line.
    text().replace("fuel_t = 2000", "fuel_t = 70")
}

#[test]
fn fuel_comes_first_70_t_against_72_on_day_1_and_the_force_sorties_on_day_2() {
    let played = play(&seventy_t());
    played.nothing_refused();
    // Day 1: no sortie. The reason gives 70 against 72 and day 2 as the day
    // from which the force may be used.
    assert_eq!(played.orders(1), vec![home_port(), engage("0.5")]);
    let none: Vec<String> = played.decisions(1).no_orders.iter().map(said).collect();
    assert!(
        none.iter()
            .any(|r| r.contains("ship:1 has 70 t aboard against 72 t needed")
                && r.contains("ship:2 has 70 t aboard against 72 t needed")
                && r.contains("may be used from day 2")),
        "{none:?}"
    );
    assert_eq!(played.cannot_meet(1), 0);
    // The ships are filled in the night of day 1.
    let night: Vec<_> = played
        .refuelled()
        .into_iter()
        .filter(|r| r.0 == 1)
        .collect();
    assert_eq!(night.len(), 2);
    for (_, pulse, _, taken, fuel, _) in &night {
        assert_eq!(*pulse, Some(Pulse::Night));
        assert_eq!(*taken, n(1930));
        assert_eq!(*fuel, n(2000));
    }
    assert_eq!(played.fuel_at_end_of(1, 1), n(2000));
    // The force sorties on day 2, and not before.
    assert_eq!(played.orders(2), vec![BATTLE, sortie()]);
    assert_eq!(
        played.halts(),
        vec![(2, Some(Pulse::Night), hex(3, 0), night_work(), fx("31.5"))]
    );
    assert_eq!(played.steamed_on().len(), 1);
    assert!(played.steamed_in(1).is_empty());
}

#[test]
fn fuel_comes_first_base_1_with_no_fuel_gives_no_sortie_and_the_notice() {
    let played = play(&changed(&seventy_t(), "fuel_t = 5000", "fuel_t = 0"));
    played.nothing_refused();
    assert!(played.halts().is_empty());
    for day in 1..=4 {
        assert!(played.steamed_in(day).is_empty(), "day {day}");
    }
    assert!(played.refuelled().is_empty());
    // The reason gives no day, and the staff hands in on day 1 that it cannot
    // meet intent 2.
    let none: Vec<String> = played.decisions(1).no_orders.iter().map(said).collect();
    assert!(
        none.iter()
            .any(|r| r.contains("ship:1 has 70 t aboard against 72 t needed")
                && r.contains("there is no day from which it may be used")
                && r.contains("cannot meet intent:2")),
        "{none:?}"
    );
    assert_eq!(played.cannot_meet(1), 1);
    // The notice rests on that decision to give no order.
    let notice = &played.decisions(1).notices[0];
    assert_eq!(notice.kind, NoticeKind::CannotMeet);
    assert_eq!(notice.rests_on, sim_core::command::NoticeBasis::NoOrder(0));
}

#[test]
fn full_speed_where_it_must_waters_at_6_0_240_nm_reach_the_station_7_5_hours_into_the_night() {
    let played = play(&changed(&text(), "hex = [3, 0]", "hex = [6, 0]"));
    played.nothing_refused();
    // Fifteen hours at cruise, 7.5 at the class's 32 knots.
    let r = said(reason_of(
        &played,
        1,
        &TaskForceOrder::SetSpeed(SpeedSetting::Full),
    ));
    assert!(
        r.contains("15 hours away at cruise and 7.5 at full speed"),
        "{r}"
    );
    let to = route(&[(6, 0), (0, 0)], &[(6, 0)]);
    assert_eq!(
        played.orders(1),
        vec![
            home_port(),
            engage("0.5"),
            BATTLE,
            TaskForceOrder::SetSpeed(SpeedSetting::Full),
            to.clone(),
        ]
    );
    assert_eq!(played.orders(2), vec![to]);
    assert_eq!(
        played.halts(),
        vec![
            (1, Some(Pulse::Night), hex(6, 0), night_work(), fx("7.5")),
            (2, Some(Pulse::Night), hex(6, 0), night_work(), fx("31.5")),
        ]
    );
    assert_eq!(played.position_after(1, Pulse::Night), hex(6, 0));
    assert!(played.halted_after(1, Pulse::Night));
    assert_eq!(played.force_at_end_of(1).speed, SpeedSetting::Full);
    played.every_chain_ends_with_the_commander();
}

#[test]
fn full_speed_where_it_must_waters_at_13_0_520_nm_are_out_of_reach_and_the_notice_is_handed_in() {
    let played = play(&changed(&text(), "hex = [3, 0]", "hex = [13, 0]"));
    played.nothing_refused();
    assert!(played.halts().is_empty());
    for day in 1..=4 {
        assert!(played.steamed_in(day).is_empty(), "day {day}");
        assert!(
            !played.orders(day).iter().any(|o| matches!(
                o,
                TaskForceOrder::SetRoute { .. } | TaskForceOrder::SetSpeed(_)
            )),
            "day {day}"
        );
    }
    // 32.5 hours at cruise, 16.25 at the class's 32 knots.
    let none: Vec<String> = played.decisions(1).no_orders.iter().map(said).collect();
    assert!(
        none.iter()
            .any(|r| r.contains("32.5 hours away at cruise and 16.25 at full speed")),
        "{none:?}"
    );
    assert_eq!(played.cannot_meet(1), 1);
}

fn hold_base_1() -> String {
    changed(
        &text(),
        "intent_kind = \"deny\"\nhex = [3, 0]\ndistance = 0",
        "intent_kind = \"hold\"\nbase = 1",
    )
}

#[test]
fn cover_from_the_sea_the_stand_tasks_station_is_s_and_the_force_is_given_no_route() {
    let played = play(&hold_base_1());
    played.nothing_refused();
    let one = &played.state.intents[&intent(1)];
    assert_eq!(one.kind, IntentKind::Hold { base: base(1) });
    assert_eq!(one.maker, Maker::Commander);
    let two = &played.state.intents[&intent(2)];
    assert_eq!(
        two.kind,
        IntentKind::StandAt {
            force: force(1),
            hex: hex(0, 0)
        }
    );
    assert_eq!(two.parent, Some(intent(1)));
    assert_eq!(two.maker, Maker::Staff);
    assert_eq!(two.addressee, Addressee::TaskForce(force(1)));
    // No route on any day; battle readiness on day 1, normal on day 3; the
    // home port and the rules of engagement on day 1.
    assert_eq!(played.orders(1), vec![home_port(), engage("0.5"), BATTLE]);
    assert!(played.orders(2).is_empty());
    assert_eq!(played.orders(3), vec![NORMAL]);
    assert!(played.orders(4).is_empty());
    for (day, order) in played.all_orders() {
        assert!(
            !matches!(tf_order(order), TaskForceOrder::SetRoute { .. }),
            "day {day}"
        );
    }
    // The contact is 10 hexes from S.
    let r = said(reason_of(&played, 1, &BATTLE));
    assert!(r.contains("10 hexes from the station (0,0)"), "{r}");
    assert!(played.halts().is_empty());
    for day in 1..=4_u32 {
        assert!(played.steamed_in(day).is_empty(), "day {day}");
        assert_eq!(played.force_at_end_of(day as usize).position(), hex(0, 0));
    }
    assert_eq!(played.force_at_end_of(1).readiness, Readiness::Battle);
    assert_eq!(played.force_at_end_of(3).readiness, Readiness::Normal);
    played.every_chain_ends_with_the_commander();
}

#[test]
fn no_force_named_no_task() {
    let text = changed(
        &text(),
        "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\ntask_forces = [1]\n",
        "",
    );
    let played = play(&text);
    played.nothing_refused();
    assert_eq!(played.state.intents.len(), 1);
    assert!(!played
        .state
        .intents
        .values()
        .any(|i| matches!(i.kind, IntentKind::StandAt { .. })));
    // Force 1 is given no order, and does not move.
    assert!(played.all_orders().is_empty());
    assert!(played.halts().is_empty());
    for day in 1..=4 {
        assert!(played.steamed_in(day).is_empty(), "day {day}");
    }
}

// ----------------------------------------------------------- The rule alone

fn commander_route(day: u32, waypoints: &str, marks: &str) -> String {
    format!(
        "\n[[order]]\nday = {day}\nside = \"allied\"\ntask_force = 1\nkind = \"set_route\"\nwaypoints = {waypoints}\n{marks}\n"
    )
}

/// Scenario 54's forces with no command and no staff intent, and the
/// commander's own route order on day 1.
fn rule_alone(mission: &str, waypoints: &str, marks: &str) -> String {
    let t = without_commands().replace(
        "mission = \"surface_combat\"",
        &format!("mission = \"{mission}\""),
    );
    format!("{t}{}", commander_route(1, waypoints, marks))
}

#[test]
fn the_rule_alone_a_force_of_any_mission_halts_at_a_mark_reached_in_the_night_and_steams_on() {
    for mission in ["surface_combat", "escort", "transport", "bombardment"] {
        let played = play(&rule_alone(
            mission,
            "[[3, 0], [0, 0]]",
            "stand_at = [[3, 0]]",
        ));
        played.nothing_refused();
        assert!(played.all_orders().is_empty());
        assert_eq!(
            played.halts(),
            vec![(1, Some(Pulse::Night), hex(3, 0), night_work(), fx("7.5"))],
            "{mission}"
        );
        assert_eq!(
            played.steamed_on(),
            vec![(1, Some(Pulse::Morning), hex(3, 0))],
            "{mission}"
        );
        assert_eq!(played.position_after(1, Pulse::Night), hex(3, 0));
        assert!(played.halted_after(1, Pulse::Night));
        assert_eq!(played.position_after(1, Pulse::Morning), hex(1, 0));
        assert!(!played.halted_after(1, Pulse::Morning));
        assert_eq!(
            played.arrivals(),
            vec![(1, Some(Pulse::Afternoon), hex(0, 0), fx("19.5"))],
            "{mission}"
        );
    }
}

#[test]
fn the_rule_alone_a_mark_reached_in_a_daylight_pulse_is_passed() {
    // (6,0) is 240 nm: 15 hours at 16 knots, in the morning pulse.
    let marked = play(&rule_alone(
        "surface_combat",
        "[[6, 0], [0, 0]]",
        "stand_at = [[6, 0]]",
    ));
    let plain = play(&rule_alone("surface_combat", "[[6, 0], [0, 0]]", ""));
    marked.nothing_refused();
    assert!(marked.halts().is_empty(), "{:?}", marked.halts());
    assert!(marked.steamed_on().is_empty());
    // The route moves as it does with no mark.
    let moves = |p: &Played| -> Vec<(u32, Option<Pulse>, Hex, Hex, Fixed)> {
        p.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceSteamed {
                    from,
                    to,
                    distance_nm,
                    ..
                } => Some((e.day, e.pulse, *from, *to, *distance_nm)),
                _ => None,
            })
            .collect()
    };
    assert!(!moves(&marked).is_empty());
    assert_eq!(moves(&marked), moves(&plain));
    assert_eq!(marked.arrivals(), plain.arrivals());
    // Out 15 hours, home 15 more: 30 hours, six hours into the night of day 2.
    assert_eq!(
        marked.arrivals(),
        vec![(2, Some(Pulse::Night), hex(0, 0), n(30))]
    );
}

#[test]
fn the_rule_alone_a_route_with_no_such_mark_moves_as_it_did() {
    let played = play(&rule_alone("surface_combat", "[[3, 0], [0, 0]]", ""));
    played.nothing_refused();
    assert!(played.halts().is_empty());
    assert!(played.steamed_on().is_empty());
    // Out 120 nm and home 120 nm: 15 hours, 3 hours into the morning.
    assert_eq!(
        played.arrivals(),
        vec![(1, Some(Pulse::Morning), hex(0, 0), n(15))]
    );
}

/// The answer the side's view gives to a movement question, asked at the
/// start of the pulse it names.
fn asked(text: &str, at: Moment, question: MovementQuestion) -> (Option<Answer>, Played) {
    let mut answer: Option<Answer> = None;
    let played = play_watched(text, &mut |world, moment| {
        if moment == at {
            answer = world
                .side_view(Side::Allied)
                .ask(&Question::Movement(question.clone()));
        }
    });
    (answer, played)
}

#[test]
fn the_rule_alone_a_side_asking_the_choice_beforehand_is_told_the_same() {
    // Marked, reached in the night: it would halt at the waypoint, index 0.
    let marked = rule_alone("surface_combat", "[[3, 0], [0, 0]]", "stand_at = [[3, 0]]");
    let (halt, played) = asked(
        &marked,
        Moment::PulseBegins {
            day: 1,
            pulse: Pulse::Night,
        },
        MovementQuestion::Halt {
            force: force(1),
            pulse: Pulse::Night,
        },
    );
    assert_eq!(halt, Some(Answer::Movement(MovementAnswer::Halt(Some(0)))));
    assert_eq!(played.halts().len(), 1);
    // Halted, it steams on at the start of the morning.
    let (steam, _) = asked(
        &marked,
        Moment::PulseBegins {
            day: 1,
            pulse: Pulse::Morning,
        },
        MovementQuestion::SteamOn {
            force: force(1),
            pulse: Pulse::Morning,
        },
    );
    assert_eq!(steam, Some(Answer::Movement(MovementAnswer::SteamOn(true))));
    // In a daylight pulse it would halt nowhere, and the engine passes.
    let (day, played) = asked(
        &rule_alone("surface_combat", "[[6, 0], [0, 0]]", "stand_at = [[6, 0]]"),
        Moment::PulseBegins {
            day: 1,
            pulse: Pulse::Morning,
        },
        MovementQuestion::Halt {
            force: force(1),
            pulse: Pulse::Morning,
        },
    );
    assert_eq!(day, Some(Answer::Movement(MovementAnswer::Halt(None))));
    assert!(played.halts().is_empty());
    // A route with no mark halts nowhere either.
    let (none, _) = asked(
        &rule_alone("surface_combat", "[[3, 0], [0, 0]]", ""),
        Moment::PulseBegins {
            day: 1,
            pulse: Pulse::Night,
        },
        MovementQuestion::Halt {
            force: force(1),
            pulse: Pulse::Night,
        },
    );
    assert_eq!(none, Some(Answer::Movement(MovementAnswer::Halt(None))));
}

// ------------------------------------------------------------- Held forces

#[test]
fn one_task_a_force_the_commanders_stand_task_stands_and_the_staff_forms_none() {
    let played = play(&with(&command_table(
        1,
        "kind = \"set_intent\"\nintent = 2\nintent_kind = \"stand_at\"\ntask_force = 1\nhex = [3, 0]",
    )));
    played.nothing_refused();
    // No stand task is formed under intent 1.
    assert_eq!(played.state.intents.len(), 2, "{:?}", played.state.intents);
    let two = &played.state.intents[&intent(2)];
    assert_eq!(two.maker, Maker::Commander);
    assert_eq!(two.parent, None);
    assert!(played
        .all(|k| matches!(k, EventKind::IntentFormed { .. }))
        .is_empty());
    // Its reason names intent 2.
    let none: Vec<String> = played.decisions(1).no_orders.iter().map(said).collect();
    assert!(
        none.iter()
            .any(|r| r.contains("intent:2") && r.contains("forms no other for task_force:1")),
        "{none:?}"
    );
    // The orders are the main case's, each serving intent 2 with an empty
    // chain.
    let main = play(&text());
    assert_eq!(played.order_count(), main.order_count());
    for day in 1..=4 {
        assert_eq!(played.orders(day), main.orders(day), "day {day}");
    }
    for (day, order) in played.all_orders() {
        assert_eq!(order.intent, intent(2), "day {day}");
        assert!(
            order.reason.serves.chain.is_empty(),
            "{:?}",
            order.reason.serves
        );
    }
}

#[test]
fn a_held_force_is_left_alone() {
    let played = play(&with(&command_table(1, "kind = \"hold\"\ntask_force = 1")));
    played.nothing_refused();
    assert!(played.all_orders().is_empty());
    assert!(played.halts().is_empty());
    for day in 1..=4 {
        assert!(played.steamed_in(day).is_empty(), "day {day}");
    }
    let none: Vec<String> = played.decisions(1).no_orders.iter().map(said).collect();
    assert!(
        none.iter()
            .any(|r| r.contains("task_force:1 is held by the commander")),
        "{none:?}"
    );
    assert!(played
        .state
        .hold_on_force(force(1), &played.truth[0].task_forces[&force(1)].ships)
        .is_some());
}

#[test]
fn a_commanders_route_order_to_force_1_holds_it() {
    // On day 1, the order is taken in and the force is held from then: the
    // staff gives it no order that day or after.
    let on_1 = play(&with(&commander_route(1, "[[1, 0]]", "")));
    on_1.nothing_refused();
    assert!(on_1.all_orders().is_empty());
    assert!(on_1.halts().is_empty());
    assert_eq!(on_1.force_at_end_of(1).position(), hex(1, 0));
    assert_eq!(on_1.force_at_end_of(4).position(), hex(1, 0));
    assert!(on_1
        .state
        .hold_on_force(force(1), &on_1.truth[0].task_forces[&force(1)].ships)
        .is_some());
    // On day 2 it holds the force from day 2: day 1 is the sortie of the
    // main case, and the staff gives no order on day 2 or after.
    let on_2 = play(&with(&commander_route(2, "[[1, 0]]", "")));
    on_2.nothing_refused();
    assert_eq!(on_2.order_count(), vec![4, 0, 0, 0]);
    assert_eq!(on_2.halts().len(), 1);
    assert_eq!(on_2.force_at_end_of(1).position(), hex(0, 0));
    assert_eq!(on_2.force_at_end_of(2).position(), hex(1, 0));
}

// ------------------------------------------------------------ Determinism

// Every scenario built before this one gives the events and end states it
// gave. That is checked by the lead with a script, not by a test.

#[test]
fn the_same_start_gives_the_same_intents_orders_and_reasons() {
    let a = play(&text());
    let b = play(&text());
    assert_eq!(a.handed, b.handed);
    assert_eq!(a.state, b.state);
    assert_eq!(a.events.len(), b.events.len());
}

#[test]
fn two_worlds_that_differ_only_in_what_the_allied_side_cannot_see_give_the_same_decisions() {
    let hidden = format!(
        "{}\n[[base]]\nid = 3\nname = \"Placeholder base 3\"\nside = \"japanese\"\nq = 10\nr = 5\nsupply_t = 50\non_map = false\n\n[[air_group]]\nid = 6\nname = \"Placeholder Japanese fighters 6\"\nside = \"japanese\"\nbase = 3\naircraft_type = 4\nready = 20\n",
        text()
    );
    let plain = play(&text());
    let other = play(&hidden);
    assert_ne!(
        plain.truth.last().map(|t| t.bases.len()),
        other.truth.last().map(|t| t.bases.len()),
        "the two worlds do not differ"
    );
    assert_eq!(plain.handed, other.handed);
    assert_eq!(plain.state, other.state);
}
