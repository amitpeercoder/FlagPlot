//! Scenario 58: a carrier force closes the range. A carrier at her hex
//! holds a contact of a carrier beyond her bombers' reach. At press she
//! steams at full speed in the night to the nearest hex from which they
//! reach it, strikes in the morning, and goes back when it is gone.
//!
//! Proves decision 0130 against a hand calculation. It stands on scenario
//! 55's for the orders to the groups. The data is placeholder data: see
//! `data/scenarios/58_a_carrier_force_closes.toml`. It runs on the frozen
//! step 1 figures with scenario 53's table of aircraft for each ship. The
//! upkeep, wear, fatigue and supply figures are not loaded. There is no
//! enemy unit: the strike finds nothing and no die is rolled.
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
//! pulses of 6. The placeholder carrier (class 4): 16 kn at cruise, 30 kn
//! at most. The placeholder dive bomber (type 3): a strike radius of 250
//! nm. The placeholder carrier scout (type 2) searches to 300 nm.
//! `strike_contact_max_age_pulses` 0; a contact gains a pulse of age at
//! the end of each pulse. From a hex, the hexes on its own row to the east
//! lie on 090. A carrier has no figure in the table of aircraft for each
//! ship: she needs everything.
//!
//! ## Inputs
//!
//! A placeholder map of open sea, with S at (0,5) a coast hex with a port:
//! Allied base 1, with 5000 t of fuel. The game is 3 days. There is no
//! Japanese unit and no Japanese base.
//!
//! Task force 1, Allied: one placeholder carrier, ship 1, stopped at sea
//! at (0,0), full of fuel, on the carrier mission, at normal readiness,
//! at cruising speed, with the stance engage and base 1 as its home port.
//! Aboard her, scenario 55's groups: group 1, 12 fighters; group 2, 6
//! carrier scouts; group 3, 10 dive bombers; all resting.
//!
//! The Allied side starts holding one contact: 1 ship, a carrier, at
//! (12,0), age 0. It is 480 nm from the force.
//!
//! The Allied side's commands of day 1: intent 1, operate task force 1
//! from (0,0); its risk, press; leave to prepare and leave to launch both
//! given.
//!
//! ## Day 1
//!
//! The contact is fresh, a carrier, and 480 nm away against a reach of
//! 250. A night at full speed is 360 nm: 480 is within 360 and 250. She
//! closes. Along the row, (5,0) is 280 nm from the contact and (6,0) is
//! 240: the first hex within reach is (6,0), 240 nm from her. At cruise
//! that is 15 hours, more than the night; at 30 kn it is 8.
//!
//! The staff's orders, all serving intent 1 and decided at the level of
//! task force 1, except the sizing order, whose reason says the group
//! decided:
//!
//! - **The force,** four: battle readiness; rules of engagement with the
//!   stance avoid and a break-off level of 0.75; full speed; a route to
//!   (6,0).
//! - **Group 1,** two: combat air patrol; a patrol share of 0.25.
//! - **Group 2,** two: the search mission; the arc. The one threat is the
//!   contact, on 090 from (0,0): the narrowest arc is 080 to 090, widened
//!   to 050 to 120, at 300 nm.
//! - **Group 3,** four, as scenario 55 at press: the naval strike mission;
//!   the oldest contact, 2; extended reach; strike sizing on.
//!
//! Twelve orders.
//!
//! She sails at the start of the night and reaches (6,0) at hour 8, where
//! she stops. In the morning the contact is 1 pulse old and 240 nm away:
//! group 3 launches its 10 at it. The raid finds nothing and the contact
//! is dropped. Nothing is launched in the afternoon.
//!
//! ## Day 2
//!
//! The side holds no contact: there is nothing in reach and nothing to
//! close on, and she goes back to her hex. (0,0) is 240 nm: full speed again, which she has. Two
//! orders: the force's route to (0,0); and group 2's arc, the whole
//! circle, there being no threat. She arrives at hour 32 and stops.
//!
//! ## Day 3
//!
//! She is at her hex. No order.
//!
//! ## At the end
//!
//! One strike of 10. Fourteen staff orders: 12, 2 and 0. The force is at
//! (0,0), stopped, having steamed 480 nm at 30 kn. Her fuel is not
//! asserted as a number: the test asserts that she burnt what a force of
//! the same ship ordered over the same two passages at full speed burns.
//!
//! # Properties
//!
//! - **Balanced.** A carrier is what balanced closes on: she closes as
//!   above, with a break-off level of 0.5, a patrol share of 0.5 and two
//!   orders to group 3: ten orders on day 1 (as first written; decision
//!   0131 made it eleven with search once a day for group 2, and the
//!   amendment of its rule 6 on Nov 16, 2026, that a carrier force
//!   searches in both pulses at balanced, brought it back). She launches
//!   nothing: the
//!   contact is a pulse old against the figure of 0. On the morning of
//!   day 2 the contact is still held, at age 3, and is in reach from
//!   (6,0), 240 nm: she stays, and the staff gives no order. The contact
//!   is dropped at the end of the morning of day 2, at 5 pulses. On day 3
//!   she holds none and goes back: two orders, the route to (0,0) and
//!   group 2's arc, the whole circle. She arrives at hour 56.
//! - **Preserve the force.** She does not close and is given no route or
//!   speed, and the reason says preserve the force closes on nothing.
//! - **A kind it does not close on.** With the contact a cruiser: at
//!   balanced she stays and the reason says so; at press she closes.
//! - **Already in reach.** With the contact at (5,0), 200 nm: no route.
//! - **Too far.** With the contact at (30,0), 1200 nm, against 610: she
//!   stays, and the reason gives the two distances.
//! - **Not the fuel.** With ship 1 holding 300 t she has the fuel for the
//!   passage home and not for the passage to (6,0) and from there home:
//!   she does not close, and the reason gives the fuel aboard against the
//!   fuel needed. With 50 t she has not the fuel for the passage home
//!   itself, which the engine's measure has her run dry short of: she is
//!   routed home to fuel, at cruise, and closing is not weighed. (Restated
//!   on Nov 16, 2026, by the architect, after the build found that the
//!   engine's measure gives no more burnt than is aboard: at 50 t she read
//!   as having the 50 t the passage home needed. A ship that runs dry
//!   short of a passage is short of the fuel for it.)
//! - **The first contact decides.** With a second contact, a destroyer at
//!   (20,0), 800 nm and beyond a night and the reach, the day is as above.
//!   With the carrier contact at (5,0), in reach, and a destroyer at
//!   (12,0): at press she does not close, the carrier being first and in
//!   reach. With a destroyer at (5,0) and the carrier at (12,0): at press
//!   she closes on the carrier, to (6,0).
//! - **Speed alone.** With no contact and the commander stating "operate
//!   task force 1 from (6,0)": at balanced she is ordered to full speed
//!   and arrives at hour 8; at preserve the force she is given no speed
//!   order and arrives at hour 15, in the morning. With the hex (2,0), 80
//!   nm, she goes at cruise at every risk and arrives at hour 5.
//! - A held force is left alone; the same start gives the same orders; two
//!   worlds that differ only in what the Allied side cannot see give the
//!   same orders and reasons.
//! - Every staffed scenario built before this one gives the orders it
//!   gave, scenario 55 among them: there the three contacts are in reach,
//!   or of a kind press closes on and in reach.

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::choices::Moment;
use sim_core::command::{Addressee, Maker, Reason, StaffDecisions, StaffOrder};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, IntentId, ShipId, TaskForceId};
use sim_core::orders::{
    AirGroupOrder, AirMission, OrderKind, Reach, Readiness, SpeedSetting, Stance,
    StragglerInstruction, TaskForceOrder,
};
use sim_core::state::{Side, TruthState};
use sim_core::time::Pulse;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

use crate::common;

const COMMANDS_MARKER: &str = "# The Allied commander's commands";
const CONTACT_MARKER: &str = "# The contact";

fn path() -> PathBuf {
    common::data_dir().join("scenarios/58_a_carrier_force_closes.toml")
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

fn group(raw: u32) -> AirGroupId {
    AirGroupId::new(raw)
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

/// The scenario's own figures: scenario 53's table of aircraft for each ship,
/// added after the frozen file's empty `[strike_aircraft_per_ship]` table.
fn figures() -> sim_data::LoadOptions {
    common::altered_reference("s58", |dir| {
        common::edit_file(dir, "air_combat.toml", |text| {
            format!("{text}\ncruiser = 12\ndestroyer = 6\ntransport = 6\nauxiliary = 4\n")
        })
    })
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
    let mut scenario: Scenario =
        common::from_text_with(text, &path(), &figures()).expect("the scenario loads");
    let held = staffs();
    let mut handed = BTreeMap::new();
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
        state: scenario.world.command_state(Side::Allied).clone(),
    }
}

fn mission(m: AirMission) -> AirGroupOrder {
    AirGroupOrder::SetAirMission(m)
}

fn age(pulses: i64) -> AirGroupOrder {
    AirGroupOrder::SetStrikeContactAge { pulses }
}

fn sizing(on: bool) -> AirGroupOrder {
    AirGroupOrder::SetStrikeSizing { on }
}

fn arc(from_deg: i32, to_deg: i32) -> AirGroupOrder {
    AirGroupOrder::SetSearchArc {
        from_deg,
        to_deg,
        range_nm: n(300),
    }
}

fn share(s: &str) -> AirGroupOrder {
    AirGroupOrder::SetPatrolShare { share: fx(s) }
}

const STRIKE: AirMission = AirMission::NavalStrike;
const CAP: AirMission = AirMission::CombatAirPatrol;
const SEARCH: AirMission = AirMission::Search;

/// An order of the staff: to a force or to an air group.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Given {
    Force(u32, TaskForceOrder),
    Air(u32, AirGroupOrder),
}

fn given(o: &StaffOrder) -> Given {
    match &o.kind {
        OrderKind::TaskForce { target, order } => Given::Force(target.raw(), order.clone()),
        OrderKind::AirGroup { target, order } => Given::Air(target.raw(), order.clone()),
        other => panic!("the doctrine gave an order of another kind: {other:?}"),
    }
}

fn rules(stance: Stance, break_off: &str) -> TaskForceOrder {
    TaskForceOrder::SetRulesOfEngagement {
        stance,
        break_off: fx(break_off),
        stragglers: StragglerInstruction::Keep,
        keep_share: Fixed::ZERO,
    }
}

const BATTLE: TaskForceOrder = TaskForceOrder::SetReadiness(Readiness::Battle);

fn route(waypoints: &[(i32, i32)]) -> TaskForceOrder {
    TaskForceOrder::SetRoute {
        waypoints: waypoints.iter().map(|(q, r)| hex(*q, *r)).collect(),
        act_at: Default::default(),
        stand_at: Default::default(),
    }
}

impl Played {
    fn decisions(&self, day: u32) -> &StaffDecisions {
        &self.handed[&day]
    }

    fn orders(&self, day: u32) -> Vec<Given> {
        self.decisions(day).orders.iter().map(given).collect()
    }

    fn orders_to_force(&self, day: u32, raw: u32) -> Vec<TaskForceOrder> {
        self.orders(day)
            .into_iter()
            .filter_map(|g| match g {
                Given::Force(f, o) if f == raw => Some(o),
                _ => None,
            })
            .collect()
    }

    fn orders_serving(&self, day: u32, served: u32) -> Vec<Given> {
        self.decisions(day)
            .orders
            .iter()
            .filter(|o| o.intent == intent(served))
            .map(given)
            .collect()
    }

    fn order_count(&self) -> Vec<usize> {
        (1..=self.handed.len() as u32)
            .map(|d| self.decisions(d).orders.len())
            .collect()
    }

    fn all_orders(&self) -> Vec<(u32, &StaffOrder)> {
        self.handed
            .iter()
            .flat_map(|(day, d)| d.orders.iter().map(move |o| (*day, o)))
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

    /// The `StrikeLaunched` events of a pulse of a day, as (group, hex,
    /// aircraft), in log order.
    fn launches(&self, day: u32, pulse: Pulse) -> Vec<(u32, Hex, u32)> {
        self.events
            .iter()
            .filter(|e| e.day == day && e.pulse == Some(pulse))
            .filter_map(|e| match &e.kind {
                EventKind::StrikeLaunched {
                    air_group,
                    hex,
                    aircraft,
                    ..
                } => Some((air_group.raw(), *hex, *aircraft)),
                _ => None,
            })
            .collect()
    }

    fn all_launches(&self) -> Vec<(u32, Hex, u32)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::StrikeLaunched {
                    air_group,
                    hex,
                    aircraft,
                    ..
                } => Some((air_group.raw(), *hex, *aircraft)),
                _ => None,
            })
            .collect()
    }

    fn launched_by(&self, raw: u32) -> u32 {
        self.all_launches()
            .iter()
            .filter(|(g, _, _)| *g == raw)
            .map(|(_, _, a)| *a)
            .sum()
    }

    fn mission_at_end(&self, raw: u32) -> AirMission {
        self.truth.last().expect("a state").air_groups[&group(raw)].mission
    }

    fn force_at_end(&self) -> &sim_core::state::TaskForce {
        &self.truth.last().expect("a state").task_forces[&force(1)]
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

// ------------------------------------------------------- The hand calculation

/// Where the force is, and whether it is between hex centres, at some
/// moments of the game: "1 Morning" is the start of the morning of day 1.
fn force_by_moment(text: &str) -> BTreeMap<String, Hex> {
    let mut held: BTreeMap<String, Hex> = BTreeMap::new();
    let mut scenario: Scenario =
        common::from_text_with(text, &path(), &figures()).expect("the scenario loads");
    let staffs = staffs();
    loop {
        let summary = scenario
            .next_day_watched(as_staffs(&staffs), &mut |world, moment| {
                if let Moment::PulseBegins { day, pulse } = moment {
                    held.insert(
                        format!("{day} {pulse:?}"),
                        world.umpire().task_forces[&force(1)].position(),
                    );
                }
            })
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
    }
    held
}

fn day_1_orders() -> Vec<Given> {
    vec![
        Given::Force(1, BATTLE),
        Given::Force(1, rules(Stance::Avoid, "0.75")),
        Given::Force(1, TaskForceOrder::SetSpeed(SpeedSetting::Full)),
        Given::Force(1, route(&[(6, 0)])),
        Given::Air(1, mission(CAP)),
        Given::Air(1, share("0.25")),
        Given::Air(2, mission(SEARCH)),
        Given::Air(2, arc(50, 120)),
        Given::Air(3, mission(STRIKE)),
        Given::Air(3, age(2)),
        Given::Air(
            3,
            AirGroupOrder::SetReach {
                reach: Reach::Extended,
            },
        ),
        Given::Air(3, sizing(true)),
    ]
}

fn full() -> Given {
    Given::Force(1, TaskForceOrder::SetSpeed(SpeedSetting::Full))
}

fn is_speed(o: &Given) -> bool {
    matches!(o, Given::Force(_, TaskForceOrder::SetSpeed(_)))
}

fn is_route(o: &Given) -> bool {
    matches!(o, Given::Force(_, TaskForceOrder::SetRoute { .. }))
}

/// Every reason of the staff, of an order or of a decision not to order.
fn all_reasons(played: &Played) -> Vec<String> {
    let mut out = Vec::new();
    for d in played.handed.values() {
        out.extend(d.orders.iter().map(|o| said(&o.reason)));
        out.extend(d.no_orders.iter().map(said));
    }
    out
}

fn any_reason_has(played: &Played, needles: &[&str]) -> bool {
    all_reasons(played)
        .iter()
        .any(|r| needles.iter().all(|n| r.contains(n)))
}

#[test]
fn the_commands_of_the_file_are_the_ones_these_tests_vary() {
    let t = text();
    let at = t.find(COMMANDS_MARKER).expect("the marker");
    let c = &t[at..];
    assert_eq!(c.matches("[[command]]").count(), 4);
    assert!(c.contains("intent_kind = \"operate_carriers\"\ntask_force = 1\nhex = [0, 0]\n"));
    assert!(c.contains("limit = \"risk\"\nintent = 1\nrisk = \"press\"\n"));
    assert!(!c.contains("given = false"));
    assert!(t.contains(CONTACT_MARKER));
    assert_eq!(t.matches("[[contact]]").count(), 1);
}

#[test]
fn the_figures_are_those_of_the_hand_calculation() {
    // A hex is 40 nm: 480 nm to the contact, 240 to (6,0), 280 from (5,0).
    assert_eq!(hex(0, 0).distance(hex(12, 0)) * 40, 480);
    assert_eq!(hex(6, 0).distance(hex(12, 0)) * 40, 240);
    assert_eq!(hex(5, 0).distance(hex(12, 0)) * 40, 280);
    assert_eq!(hex(0, 0).distance(hex(6, 0)) * 40, 240);
    let types = std::fs::read_to_string(common::frozen_root().join("aircraft_types.csv"))
        .expect("the frozen aircraft types read");
    let row = |id: &str| -> Vec<String> {
        types
            .lines()
            .find(|l| l.starts_with(&format!("{id},")))
            .unwrap_or_else(|| panic!("no row {id}"))
            .split(',')
            .map(str::to_string)
            .collect()
    };
    assert_eq!(row("2")[3], "300");
    assert_eq!(row("3")[5], "250");
}

#[test]
fn day_1_has_twelve_orders_in_the_order_of_the_doctrine() {
    let played = play(&text());
    played.nothing_refused();
    assert_eq!(played.orders(1).len(), 12, "{:?}", played.orders(1));
    assert_eq!(played.orders(1), day_1_orders());
    assert_eq!(played.orders_serving(1, 1), day_1_orders());
    for day in 1..=3 {
        assert!(played.decisions(day).notices.is_empty(), "day {day}");
    }
}

#[test]
fn day_2_sends_her_back_and_day_3_has_no_order() {
    let played = play(&text());
    assert_eq!(
        played.orders(2),
        vec![Given::Force(1, route(&[(0, 0)])), Given::Air(2, arc(0, 0))]
    );
    assert!(played.orders(3).is_empty(), "{:?}", played.orders(3));
    assert_eq!(played.order_count(), vec![12, 2, 0]);
    assert_eq!(played.all_orders().len(), 14);
    for (_, o) in played.all_orders() {
        assert_eq!(o.intent, intent(1));
    }
}

#[test]
fn every_order_is_decided_at_the_level_of_the_force_except_the_sizing_order() {
    let played = play(&text());
    let mut sizings = Vec::new();
    for (day, order) in played.all_orders() {
        if given(order) == Given::Air(3, sizing(true)) {
            assert_eq!(level(&order.reason), Some(Addressee::AirGroup(group(3))));
            sizings.push(day);
        } else {
            assert_eq!(
                level(&order.reason),
                Some(Addressee::TaskForce(force(1))),
                "day {day}"
            );
        }
    }
    assert_eq!(sizings, vec![1]);
    played.every_chain_ends_with_the_commander();
}

#[test]
fn the_morning_of_day_1_sends_group_3s_10_at_the_contact_and_nothing_else_is_launched() {
    let played = play(&text());
    assert_eq!(
        played.launches(1, Pulse::Morning),
        vec![(3, hex(12, 0), 10)]
    );
    assert!(played.launches(1, Pulse::Night).is_empty());
    assert!(played.launches(1, Pulse::Afternoon).is_empty());
    assert_eq!(played.all_launches(), vec![(3, hex(12, 0), 10)]);
    assert_eq!(played.launched_by(3), 10);
}

#[test]
fn she_reaches_6_0_at_hour_8_and_0_0_at_hour_32_and_stops() {
    let played = play(&text());
    played.nothing_refused();
    let arrivals: Vec<(Hex, Fixed)> = played
        .all(|k| matches!(k, EventKind::TaskForceArrived { .. }))
        .iter()
        .map(|e| match &e.kind {
            EventKind::TaskForceArrived {
                hex, elapsed_hours, ..
            } => (*hex, *elapsed_hours),
            _ => unreachable!(),
        })
        .collect();
    assert_eq!(arrivals, vec![(hex(6, 0), n(8)), (hex(0, 0), n(32))]);
    let at = force_by_moment(&text());
    assert_eq!(at["1 Morning"], hex(6, 0));
    assert_eq!(at["1 Afternoon"], hex(6, 0));
    assert_eq!(at["2 Night"], hex(6, 0));
    assert_eq!(at["2 Morning"], hex(0, 0));
    assert_eq!(at["3 Night"], hex(0, 0));
    let f = played.force_at_end();
    assert_eq!(f.position(), hex(0, 0));
    assert_eq!(f.toward_hex, None);
    assert_eq!(f.speed, SpeedSetting::Full);
    // Stopped: her route is done and she is neither under way nor halted.
    assert_eq!(f.route.next_index, f.route.waypoints.len(), "{:?}", f.route);
    assert!(
        !matches!(
            f.status,
            sim_core::state::TaskForceStatus::Underway | sim_core::state::TaskForceStatus::Halted
        ),
        "{:?}",
        f.status
    );
}

#[test]
fn she_burns_what_the_same_ship_ordered_over_the_same_passages_burns() {
    let by_staff = play(&text());
    let t = text();
    let at = t.find(COMMANDS_MARKER).expect("the marker");
    let plain = format!(
        "{}{}",
        &t[..at],
        "[[order]]\nday = 1\nside = \"allied\"\ntask_force = 1\nkind = \"set_speed\"\nspeed = \"full\"\n\n[[order]]\nday = 1\nside = \"allied\"\ntask_force = 1\nkind = \"set_route\"\nwaypoints = [[6, 0]]\n\n[[order]]\nday = 2\nside = \"allied\"\ntask_force = 1\nkind = \"set_route\"\nwaypoints = [[0, 0]]\n"
    );
    let by_orders = play(&plain);
    by_orders.nothing_refused();
    assert_eq!(by_orders.force_at_end().position(), hex(0, 0));
    let a = by_staff.truth.last().expect("a state").ships[&ship(1)].fuel_t;
    let b = by_orders.truth.last().expect("a state").ships[&ship(1)].fuel_t;
    assert_eq!(a, b);
    assert!(a < n(3000), "{a}");
}

#[test]
fn the_group_ends_as_ordered_and_no_order_is_refused() {
    let played = play(&text());
    played.nothing_refused();
    assert_eq!(played.mission_at_end(1), CAP);
    assert_eq!(played.mission_at_end(2), SEARCH);
    assert_eq!(played.mission_at_end(3), STRIKE);
    let f = played.force_at_end();
    assert_eq!(f.readiness, Readiness::Battle);
    assert_eq!(f.rules_of_engagement.stance, Stance::Avoid);
    assert_eq!(f.rules_of_engagement.break_off, fx("0.75"));
    assert_eq!(f.home_port, Some(base(1)));
}

// --------------------------------------------------------------- Properties

fn without_risk(text: &str) -> String {
    changed(
        text,
        "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"press\"\n",
        "",
    )
}

fn at_risk(text: &str, risk: &str) -> String {
    changed(text, "risk = \"press\"", &format!("risk = \"{risk}\""))
}

fn balanced_air_orders() -> Vec<Given> {
    vec![
        Given::Air(1, mission(CAP)),
        Given::Air(1, share("0.5")),
        Given::Air(2, mission(SEARCH)),
        Given::Air(2, arc(50, 120)),
        Given::Air(3, mission(STRIKE)),
        Given::Air(3, sizing(true)),
    ]
}

#[test]
fn balanced_closes_with_ten_orders_launches_nothing_stays_and_goes_back_on_day_3() {
    let played = play(&without_risk(&text()));
    played.nothing_refused();
    assert_eq!(
        played.orders(1),
        [
            vec![
                Given::Force(1, BATTLE),
                Given::Force(1, rules(Stance::Avoid, "0.5")),
                full(),
                Given::Force(1, route(&[(6, 0)])),
            ],
            balanced_air_orders(),
        ]
        .concat()
    );
    assert_eq!(played.orders(1).len(), 10);
    assert!(played.all_launches().is_empty());
    assert!(played.orders(2).is_empty(), "{:?}", played.orders(2));
    assert_eq!(
        played.orders(3),
        vec![Given::Force(1, route(&[(0, 0)])), Given::Air(2, arc(0, 0))]
    );
    assert_eq!(played.order_count(), vec![10, 0, 2]);
    assert_eq!(played.force_at_end().position(), hex(0, 0));
    assert_eq!(played.force_at_end().toward_hex, None);
    let at = force_by_moment(&without_risk(&text()));
    assert_eq!(at["1 Morning"], hex(6, 0));
    assert_eq!(at["3 Morning"], hex(0, 0));
    assert_eq!(
        arrivals(&played),
        vec![(hex(6, 0), n(8)), (hex(0, 0), n(56))]
    );
    // On day 2 the contact is held at age 3, 240 nm from (6,0): she stays.
    assert!(
        any_reason_has(
            &played,
            &[
                "stays where it is",
                "contact 1, a carrier, 3 pulse(s) old, is 240 nm from (6,0)"
            ]
        ),
        "{:?}",
        all_reasons(&played)
    );
    played.every_chain_ends_with_the_commander();
}

#[test]
fn preserve_the_force_does_not_close_and_says_it_closes_on_nothing() {
    let played = play(&at_risk(&text(), "preserve"));
    played.nothing_refused();
    for day in 1..=3 {
        let orders = played.orders(day);
        assert!(!orders.iter().any(is_route), "day {day}: {orders:?}");
        assert!(!orders.iter().any(is_speed), "day {day}: {orders:?}");
    }
    assert!(
        any_reason_has(&played, &["closes on nothing"]),
        "{:?}",
        all_reasons(&played)
    );
    assert!(played.all_launches().is_empty());
    assert_eq!(played.force_at_end().position(), hex(0, 0));
}

fn cruiser_contact(text: &str) -> String {
    changed(text, "kind = \"carrier\"", "kind = \"cruiser\"")
}

#[test]
fn a_cruiser_is_not_closed_on_at_balanced_and_is_at_press() {
    let balanced = play(&cruiser_contact(&without_risk(&text())));
    balanced.nothing_refused();
    assert!(
        !balanced.orders(1).iter().any(is_route),
        "{:?}",
        balanced.orders(1)
    );
    assert!(!balanced.orders(1).iter().any(is_speed));
    assert!(
        any_reason_has(&balanced, &["cruiser"]),
        "{:?}",
        all_reasons(&balanced)
    );
    let press = play(&cruiser_contact(&text()));
    press.nothing_refused();
    assert!(press.orders(1).contains(&full()));
    assert!(press.orders(1).contains(&Given::Force(1, route(&[(6, 0)]))));
}

#[test]
fn a_contact_already_in_reach_gets_no_route() {
    let played = play(&changed(&text(), "q = 12\nr = 0", "q = 5\nr = 0"));
    played.nothing_refused();
    assert!(
        !played.orders(1).iter().any(is_route),
        "{:?}",
        played.orders(1)
    );
    assert!(!played.orders(1).iter().any(is_speed));
}

#[test]
fn a_contact_too_far_is_not_closed_on_and_the_reason_gives_the_distances() {
    let played = play(&changed(&text(), "q = 12\nr = 0", "q = 30\nr = 0"));
    played.nothing_refused();
    assert!(
        !played.orders(1).iter().any(is_route),
        "{:?}",
        played.orders(1)
    );
    assert!(!played.orders(1).iter().any(is_speed));
    assert!(
        any_reason_has(&played, &["1200 nm", "610 nm"]),
        "{:?}",
        all_reasons(&played)
    );
}

#[test]
fn not_the_fuel_with_50_t_she_is_routed_home_at_cruise_and_closing_is_not_weighed() {
    let played = play(&changed(&text(), "fuel_t = 3000", "fuel_t = 50"));
    // Day 1: she is short of the passage home itself, which the engine's
    // measure has her run dry short of, and is routed home at cruise, which
    // she has: no speed order.
    let forces = played.orders_to_force(1, 1);
    assert!(forces.contains(&route(&[(0, 5)])), "{forces:?}");
    assert!(!forces.contains(&route(&[(6, 0)])), "{forces:?}");
    assert!(
        !forces
            .iter()
            .any(|o| matches!(o, TaskForceOrder::SetSpeed(_))),
        "{forces:?}"
    );
    let r = said(reason_of(&played, 1, &Given::Force(1, route(&[(0, 5)]))));
    assert!(r.contains("ship:1 has 50 t aboard against"), "{r}");
    assert!(r.contains("running dry short of the passage"), "{r}");
    assert!(r.contains("routed home to base:1"), "{r}");
    assert!(r.contains("at cruise"), "{r}");
    // Closing is not weighed on day 1. (From day 2 she lies dry at (0,4),
    // which the engine's measure gives no line for: decision 0125's
    // doctrine as built, which the hand calculation does not reach.)
    let day_1: Vec<String> = played
        .decisions(1)
        .orders
        .iter()
        .map(|o| said(&o.reason))
        .chain(played.decisions(1).no_orders.iter().map(said))
        .collect();
    assert!(
        !day_1.iter().any(|r| r.contains("does not close")),
        "{day_1:?}"
    );
    for day in 1..=3 {
        assert!(
            !played
                .orders(day)
                .contains(&Given::Force(1, route(&[(6, 0)]))),
            "day {day}: {:?}",
            played.orders(day)
        );
    }
}

/// The architect's ruling of Nov 16, 2026, recorded under decision 0131,
/// "As built": a carrier force that cannot move for want of fuel is given no
/// route and no speed order, the staff hands in that it cannot meet the task
/// with a reason naming the ship out of fuel and where the force lies, and
/// its air groups are given their orders as on any morning. The 50 t case
/// carried to days 2 and 3: no staff order is refused on any day, and the
/// notice is handed in from the morning she lies dry.
#[test]
fn not_the_fuel_with_50_t_she_lies_dry_and_is_given_no_route_from_that_morning() {
    use sim_core::command::NoticeKind;
    let played = play(&changed(&text(), "fuel_t = 3000", "fuel_t = 50"));
    played.nothing_refused();
    // The mornings on which ship 1 has no fuel aboard.
    let dry_from = (1..=3u32)
        .find(|day| played.truth[(day - 1) as usize].ships[&ship(1)].fuel_t <= Fixed::ZERO)
        .expect("she lies dry on some morning");
    for day in 1..=3u32 {
        let dry = day >= dry_from;
        let notices = &played.decisions(day).notices;
        assert_eq!(
            notices.iter().any(|n| n.kind == NoticeKind::CannotMeet),
            dry,
            "day {day}: {notices:?}"
        );
        if dry {
            let forces = played.orders_to_force(day, 1);
            assert!(
                !forces.iter().any(|o| matches!(
                    o,
                    TaskForceOrder::SetRoute { .. } | TaskForceOrder::SetSpeed(_)
                )),
                "day {day}: {forces:?}"
            );
            let lies =
                played.truth[(day - 1) as usize].task_forces[&TaskForceId::new(1)].position();
            assert!(
                played
                    .decisions(day)
                    .no_orders
                    .iter()
                    .map(said)
                    .any(|r| r.contains("ship:1 has no fuel aboard")
                        && r.contains(&format!("lies at {lies}"))),
                "day {day}: {:?}",
                played
                    .decisions(day)
                    .no_orders
                    .iter()
                    .map(said)
                    .collect::<Vec<_>>()
            );
        }
    }
    // She has 50 t on the morning of day 1 and lies dry from day 2.
    assert_eq!(dry_from, 2);
}

/// Rule 4 of the hand calculation ("Not the fuel", 300 t): she has the fuel
/// for the passage home from (0,0) at cruise (66.666666 t at press) but not
/// for the passage to (6,0) and from there home at full speed.
#[test]
fn not_the_fuel_with_300_t_she_has_the_passage_home_but_does_not_close() {
    let played = play(&changed(&text(), "fuel_t = 3000", "fuel_t = 300"));
    played.nothing_refused();
    assert!(played.orders_to_force(1, 1).iter().all(|o| !matches!(
        o,
        TaskForceOrder::SetRoute { .. } | TaskForceOrder::SetSpeed(_)
    )));
    assert!(
        any_reason_has(
            &played,
            &[
                "does not close on contact 1",
                "ship:1 has 300 t aboard against"
            ]
        ),
        "{:?}",
        all_reasons(&played)
    );
    assert!(played.all_launches().is_empty());
}

/// The force's arrivals, as (hex, hours since the start of the game).
fn arrivals(played: &Played) -> Vec<(Hex, Fixed)> {
    played
        .all(|k| matches!(k, EventKind::TaskForceArrived { .. }))
        .iter()
        .map(|e| match &e.kind {
            EventKind::TaskForceArrived {
                hex, elapsed_hours, ..
            } => (*hex, *elapsed_hours),
            _ => unreachable!(),
        })
        .collect()
}

fn without_contact(text: &str) -> String {
    let from = text.find(CONTACT_MARKER).expect("the contact");
    let to = text.find(COMMANDS_MARKER).expect("the commands");
    format!("{}{}", &text[..from], &text[to..])
}

fn commander_hex(text: &str, q: i32) -> String {
    changed(
        &without_contact(text),
        "hex = [0, 0]",
        &format!("hex = [{q}, 0]"),
    )
}

#[test]
fn speed_alone_at_balanced_a_distant_hex_is_gone_to_at_full_speed_and_reached_at_hour_8() {
    let t = commander_hex(&without_risk(&text()), 6);
    let played = play(&t);
    played.nothing_refused();
    let orders = played.orders_to_force(1, 1);
    assert!(orders.contains(&TaskForceOrder::SetSpeed(SpeedSetting::Full)));
    assert!(orders.contains(&route(&[(6, 0)])));
    let at = force_by_moment(&t);
    assert_eq!(at["1 Morning"], hex(6, 0));
    assert_eq!(arrivals(&played), vec![(hex(6, 0), n(8))]);
}

#[test]
fn speed_alone_at_press_a_distant_hex_is_gone_to_at_full_speed() {
    let played = play(&commander_hex(&text(), 6));
    played.nothing_refused();
    let orders = played.orders_to_force(1, 1);
    assert!(orders.contains(&TaskForceOrder::SetSpeed(SpeedSetting::Full)));
    assert!(orders.contains(&route(&[(6, 0)])));
    assert_eq!(arrivals(&played), vec![(hex(6, 0), n(8))]);
}

#[test]
fn speed_alone_at_preserve_the_force_goes_at_cruise_and_arrives_at_hour_15() {
    let t = commander_hex(&at_risk(&text(), "preserve"), 6);
    let played = play(&t);
    played.nothing_refused();
    let orders = played.orders_to_force(1, 1);
    assert!(!orders
        .iter()
        .any(|o| matches!(o, TaskForceOrder::SetSpeed(_))));
    assert!(orders.contains(&route(&[(6, 0)])));
    let at = force_by_moment(&t);
    assert_ne!(at["1 Morning"], hex(6, 0));
    assert_eq!(at["1 Afternoon"], hex(6, 0));
    assert_eq!(arrivals(&played), vec![(hex(6, 0), n(15))]);
}

#[test]
fn speed_alone_a_near_hex_is_gone_to_at_cruise_at_every_risk_and_reached_by_morning() {
    for risk in ["balanced", "press", "preserve"] {
        let t = commander_hex(&at_risk(&text(), risk), 2);
        let played = play(&t);
        played.nothing_refused();
        let orders = played.orders_to_force(1, 1);
        assert!(
            !orders
                .iter()
                .any(|o| matches!(o, TaskForceOrder::SetSpeed(_))),
            "{risk}: {orders:?}"
        );
        assert!(orders.contains(&route(&[(2, 0)])), "{risk}: {orders:?}");
        let at = force_by_moment(&t);
        assert_eq!(at["1 Morning"], hex(2, 0), "{risk}");
        assert_eq!(arrivals(&played), vec![(hex(2, 0), n(5))], "{risk}");
    }
}

#[test]
fn a_held_force_is_left_alone() {
    let played = play(&with(&command_table(1, "kind = \"hold\"\ntask_force = 1")));
    played.nothing_refused();
    assert!(played.all_orders().is_empty());
    assert!(played.all_launches().is_empty());
    assert_eq!(played.force_at_end().position(), hex(0, 0));
}

#[test]
fn the_same_start_gives_the_same_orders() {
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

/// The reason of the order of a day that is `wanted`.
fn reason_of<'a>(played: &'a Played, day: u32, wanted: &Given) -> &'a Reason {
    &played
        .decisions(day)
        .orders
        .iter()
        .find(|o| &given(o) == wanted)
        .unwrap_or_else(|| panic!("day {day} has no order {wanted:?}"))
        .reason
}

// ------------------------------------------------------ The first contact

/// The scenario with a second contact entered after the first.
fn with_second_contact(text: &str, q: i32, kind: &str) -> String {
    let at = text.find(COMMANDS_MARKER).expect("the commands");
    format!(
        "{}[[contact]]\nside = \"allied\"\nq = {q}\nr = 0\nship_count = 1\nkind = \"{kind}\"\nage_pulses = 0\n\n{}",
        &text[..at],
        &text[at..]
    )
}

#[test]
fn the_first_contact_decides_a_destroyer_beyond_a_night_leaves_day_1_as_above() {
    let played = play(&with_second_contact(&text(), 20, "destroyer"));
    played.nothing_refused();
    assert_eq!(played.orders(1), day_1_orders());
    assert_eq!(
        played.launches(1, Pulse::Morning),
        vec![(3, hex(12, 0), 10)]
    );
}

#[test]
fn the_first_contact_decides_the_carrier_in_reach_and_a_destroyer_beyond_no_close() {
    let t = with_second_contact(
        &changed(&text(), "q = 12\nr = 0", "q = 5\nr = 0"),
        12,
        "destroyer",
    );
    let played = play(&t);
    played.nothing_refused();
    assert!(
        !played.orders(1).iter().any(is_route),
        "{:?}",
        played.orders(1)
    );
    assert!(
        !played.orders(1).iter().any(is_speed),
        "{:?}",
        played.orders(1)
    );
    assert!(
        any_reason_has(&played, &["does not close: contact 1, a carrier"]),
        "{:?}",
        all_reasons(&played)
    );
}

#[test]
fn the_first_contact_decides_a_destroyer_in_reach_and_the_carrier_beyond_she_closes() {
    let t = with_second_contact(
        &changed(
            &changed(&text(), "q = 12\nr = 0", "q = 5\nr = 0"),
            "kind = \"carrier\"",
            "kind = \"destroyer\"",
        ),
        12,
        "carrier",
    );
    let played = play(&t);
    played.nothing_refused();
    assert!(played.orders(1).contains(&full()), "{:?}", played.orders(1));
    assert!(
        played
            .orders(1)
            .contains(&Given::Force(1, route(&[(6, 0)]))),
        "{:?}",
        played.orders(1)
    );
}
