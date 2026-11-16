//! Scenario 55: a carrier force played from an end. The commander states
//! "deny the waters at (0,5)", at press, and names task force 1, a
//! carrier. The staff gives the force the task of operating from its hex,
//! and at the force's level flies its patrol, searches and sets its
//! bombers to strike.
//!
//! Proves decision 0125 against a hand calculation. It stands on scenario
//! 53's for the strike orders and the sizes of the strikes. The data is
//! placeholder data: see `data/scenarios/55_a_carrier_force_from_an_end.toml`.
//! It runs on the frozen step 1 figures with scenario 53's table of
//! aircraft for each ship. The upkeep, wear, fatigue and supply figures
//! are not loaded. There is no enemy unit: every strike finds nothing,
//! the search sights nothing, and no die is rolled.
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
//! Scenario 53's. The placeholder carrier scout (type 2) searches to 300
//! nm and carries no weapon. From a hex, the hexes on its own row to the
//! east lie on 090 and to the west on 270, and (0,4) and (0,5) from (0,0)
//! lie on 150. The placeholder carrier (class 4): 3000 t of fuel over 9000
//! nm, 16 kn at cruise.
//!
//! ## Inputs
//!
//! A placeholder map of open sea, with S at (5,0) a coast hex with a port:
//! Allied base 1, with 5000 t of fuel. The game is 2 days. There is no
//! Japanese unit and no Japanese base.
//!
//! Task force 1, Allied: one placeholder carrier, ship 1, stopped at sea
//! at (0,0), full of fuel, on the carrier mission, at normal readiness,
//! with the stance engage and base 1 as its home port. Aboard her:
//!
//! | Group | Type          | Ready aircraft | At the start |
//! | ----- | ------------- | -------------- | ------------ |
//! | 1     | fighter       | 12             | resting, no setting |
//! | 2     | carrier scout | 6              | resting, no arc |
//! | 3     | dive bomber   | 10             | resting, no setting |
//!
//! The Allied side starts holding scenario 53's three contacts: contact 1,
//! a carrier at (0,4), age 2; contact 2, 2 cruisers at (3,0), age 1;
//! contact 3, 3 transports at (-3,0), age 0.
//!
//! The Allied side's commands of day 1: intent 1, deny the waters within 0
//! hexes of (0,5); its risk, press; its forces limit, task force 1; leave
//! to prepare and leave to launch both given.
//!
//! ## The tree on the first morning
//!
//! | Intent | Kind | Parent | Maker | Addressee |
//! | ------ | ---- | ------ | ----- | --------- |
//! | 1      | deny the waters at (0,5) | none | the commander | the area |
//! | 2      | operate task force 1 from (0,0) | 1 | the staff | task force 1 |
//!
//! Base 1 has no air group, so no task is formed for it.
//!
//! ## The orders of day 1
//!
//! All serve intent 2 and were decided at the level of task force 1,
//! except the sizing order, whose reason says the group decided.
//!
//! - **The force,** two orders: battle readiness; and rules of engagement
//!   with the stance avoid and a break-off level of 0.75. It is in its hex
//!   and is given no route. Fuel: the passage home is 200 nm, 66.666666 t
//!   at press against 3000 aboard.
//! - **Group 1,** two orders: combat air patrol, and a patrol share of
//!   0.25. It keeps 12 x 0.25 = 3 overhead in each daylight pulse.
//! - **Group 2,** two orders: the search mission, and the arc. It can
//!   search and cannot attack a ship. The threats are the three contacts,
//!   on 150, 090 and 270, and the waters on 150. The narrowest arc that
//!   holds them runs from 090 to 270. Widened by 30 at each end it is 060
//!   to 300, at 300 nm.
//! - **Group 3,** four orders, as scenario 53's bombers: the naval strike
//!   mission; the oldest contact, 2; extended reach; strike sizing on.
//!
//! Ten orders on day 1.
//!
//! ## The strikes of day 1
//!
//! As scenario 53, with one group. In the morning contact 1 is 3 pulses
//! old and does not qualify; group 3 sends its 10 at contact 2, whose need
//! is 24. The raid finds nothing and contact 2 is dropped. In the
//! afternoon contact 3 is 2 pulses old; group 3 sends its 10 at it, whose
//! need is 18. It finds nothing and contact 3 is dropped. At the end of
//! the afternoon contact 1 reaches 5 pulses and is dropped.
//!
//! ## Day 2
//!
//! The Allied side holds no contact. The one threat is the waters, on 150:
//! the arc is 110 to 180. One order: group 2's arc, 110 to 180 at 300 nm.
//! Nothing is launched.
//!
//! ## At the end
//!
//! Two strikes of 10. Eleven staff orders. The force has not moved and has
//! burnt no fuel under way. Group 2 searched in all four daylight pulses
//! with its 6 and sighted nothing.
//!
//! # Properties
//!
//! - **Balanced.** Group 1's share is 0.5 and it keeps 6 overhead. Group 3
//!   is given two orders, the mission and sizing on, and launches nothing.
//!   The rules of engagement order has a break-off level of 0.5. Day 1 has
//!   8 orders.
//! - **No scout.** With no group 2 and a second dive bomber group, group 4,
//!   of 10: no group can only search, so group 3, the lowest that can do
//!   both, is set to search and group 4 to strike. The placeholder dive
//!   bomber has no search range, so this is a test of the pure decision in
//!   a view built for it, with a type that has both.
//! - **Fuel takes her home.** At balanced, with ship 1 holding 79 t: the
//!   passage home needs 66.666666 x 1.2 = 79.999999 t, in the engine's
//!   fixed point. On day 1 the force is routed to base 1 and the reason
//!   gives 79 against 79.999999. Its groups are given their orders as
//!   above. With 81 t it stays. (Restated by the architect on Nov 16,
//!   2026, after the build: the first draft rounded the need to 80.)
//! - **A commander's hex.** With the commander stating "operate task force
//!   1 from (2,0)" as intent 1 and nothing else: on day 1 the force is
//!   routed to (2,0) at cruise, and the air orders are those of balanced.
//! - **A raid on the force raises the patrol.** In a view built for it,
//!   with an air attack on the force two days before: the share ordered at
//!   balanced is 0.75.
//! - **Nothing to fly.** With no air group aboard, no task is formed. With
//!   every group at no ready aircraft, the staff hands in on day 1 that it
//!   cannot meet intent 2.
//! - **A surface force and a carrier force named together.** With task
//!   force 2, two cruisers on the surface combat mission, also in the
//!   forces limit: the staff forms a stand task for it (decision 0124) and
//!   the carrier task for force 1, and orders each by its own doctrine.
//! - **One task a force.** With the commander also stating, on day 1,
//!   "operate task force 1 from (0,0)" as intent 2, beside the deny end:
//!   the staff forms no task for force 1 under intent 1, and its reason
//!   names intent 2. Every order to the force and to its groups is given
//!   once, serving intent 2, whose risk is balanced: day 1 has 8 orders.
//!   (Added by the architect on Nov 16, 2026, with the ruling that a force
//!   has one task of a kind, whoever made it.)
//! - A held force or group is left alone; the same start gives the same
//!   orders; two worlds that differ only in what the Allied side cannot
//!   see give the same intents, orders and reasons.
//! - Every staffed scenario built before this one gives the orders it
//!   gave.

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::choices::Moment;
use sim_core::command::{
    Addressee, IntentKind, Maker, NoticeKind, Reason, StaffDecisions, StaffOrder,
};
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

mod common;

const COMMANDS_MARKER: &str = "# The Allied commander's commands";
const CONTACTS_MARKER: &str = "# The three contacts";

fn path() -> PathBuf {
    common::data_dir().join("scenarios/55_a_carrier_force_from_an_end.toml")
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
    common::altered_reference("s55", |dir| {
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

/// One search flown: (day, pulse, group, from, to, range, aircraft).
type Search = (u32, Option<Pulse>, u32, u32, u32, Fixed, u32);

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

    fn orders_to_group(&self, day: u32, raw: u32) -> Vec<AirGroupOrder> {
        self.orders(day)
            .into_iter()
            .filter_map(|g| match g {
                Given::Air(f, o) if f == raw => Some(o),
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

    /// The searches flown, as (day, pulse, group, from, to, range, aircraft).
    fn searches(&self) -> Vec<Search> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::SearchFlown {
                    air_group,
                    from_deg,
                    to_deg,
                    range_nm,
                    aircraft,
                    ..
                } => Some((
                    e.day,
                    e.pulse,
                    air_group.raw(),
                    *from_deg,
                    *to_deg,
                    *range_nm,
                    *aircraft,
                )),
                _ => None,
            })
            .collect()
    }

    fn ready_at_end(&self, raw: u32) -> u32 {
        self.truth.last().expect("a state").air_groups[&group(raw)].ready
    }

    fn mission_at_end(&self, raw: u32) -> AirMission {
        self.truth.last().expect("a state").air_groups[&group(raw)].mission
    }

    fn air_group_at_end(&self, raw: u32) -> &sim_core::state::AirGroup {
        &self.truth.last().expect("a state").air_groups[&group(raw)]
    }

    fn force_at_end(&self) -> &sim_core::state::TaskForce {
        &self.truth.last().expect("a state").task_forces[&force(1)]
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

// ------------------------------------------------------- The hand calculation

/// What the Allied side holds at some moments of day 1 and day 2: the hex,
/// the ships reported and the age in pulses of each contact, by moment.
fn contacts_by_moment(text: &str) -> BTreeMap<String, Vec<(Hex, u32, u32)>> {
    let mut held: BTreeMap<String, Vec<(Hex, u32, u32)>> = BTreeMap::new();
    let mut scenario: Scenario =
        common::from_text_with(text, &path(), &figures()).expect("the scenario loads");
    let staffs = staffs();
    loop {
        let summary = scenario
            .next_day_watched(as_staffs(&staffs), &mut |world, moment| {
                let key = match moment {
                    Moment::Orders { day } => Some(format!("orders {day}")),
                    Moment::PulseBegins { day, pulse } => Some(format!("{day} {pulse:?}")),
                    _ => None,
                };
                if let Some(key) = key {
                    held.insert(
                        key,
                        world
                            .side_view(Side::Allied)
                            .contacts()
                            .map(|c| (c.estimated_hex, c.estimated_ship_count, c.age_pulses))
                            .collect(),
                    );
                }
            })
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
    }
    held.insert(
        "end".to_string(),
        scenario
            .world
            .side_view(Side::Allied)
            .contacts()
            .map(|c| (c.estimated_hex, c.estimated_ship_count, c.age_pulses))
            .collect(),
    );
    held
}

/// The orders of the main case on day 1, in the order the staff gives them.
fn day_1_orders() -> Vec<Given> {
    vec![
        // The force: battle readiness, then the stance avoid at 0.75.
        Given::Force(1, BATTLE),
        Given::Force(1, rules(Stance::Avoid, "0.75")),
        // Group 1: combat air patrol and a patrol share of 0.25.
        Given::Air(1, mission(CAP)),
        Given::Air(1, share("0.25")),
        // Group 2: the search mission and the arc, 060 to 300 at 300 nm.
        Given::Air(2, mission(SEARCH)),
        Given::Air(2, arc(60, 300)),
        // Group 3: the naval strike mission, the oldest contact 2, extended
        // reach, strike sizing on.
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

#[test]
fn the_commands_of_the_file_are_the_ones_these_tests_vary() {
    let t = text();
    let at = t.find(COMMANDS_MARKER).expect("the marker");
    let c = &t[at..];
    assert_eq!(c.matches("[[command]]").count(), 5);
    assert!(c.contains("intent_kind = \"deny\"\nhex = [0, 5]\ndistance = 0\n"));
    assert!(c.contains("limit = \"risk\"\nintent = 1\nrisk = \"press\"\n"));
    assert!(c.contains("limit = \"forces\"\nintent = 1\ntask_forces = [1]\n"));
    assert!(!c.contains("given = false"));
    assert!(t.contains(CONTACTS_MARKER));
}

#[test]
fn the_aircraft_are_those_of_the_hand_calculation() {
    // The placeholder carrier scout (type 2) searches to 300 nm and carries
    // no weapon; the placeholder dive bomber (type 3) has no search range.
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
    // id, name, role, search_range_nm, sweep_width_nm, strike_range_nm,
    // attack_weapon, attack_damage.
    let scout = row("2");
    assert_eq!(scout[3], "300");
    assert_eq!(scout[6], "none");
    let bomber = row("3");
    assert_eq!(bomber[3], "0");
    assert_eq!(bomber[5], "250");
    assert_eq!(bomber[6], "bomb");
    // A hex is 40 nm: the passage home from (0,0) to (5,0) is 200 nm.
    assert_eq!(hex(0, 0).distance(hex(5, 0)) * 40, 200);
}

#[test]
fn the_contacts_by_pulse() {
    let held = contacts_by_moment(&text());
    // Scenario 53's three contacts, at the staff's hour of day 1.
    assert_eq!(
        held["orders 1"],
        vec![(hex(0, 4), 1, 2), (hex(3, 0), 2, 1), (hex(-3, 0), 3, 0)]
    );
    // The morning: contact 1 is 3 pulses old and does not qualify.
    assert_eq!(
        held["1 Morning"],
        vec![(hex(0, 4), 1, 3), (hex(3, 0), 2, 2), (hex(-3, 0), 3, 1)]
    );
    // The afternoon: contact 2 is dropped by the raid that found nothing;
    // contact 3 is 2 pulses old.
    assert_eq!(
        held["1 Afternoon"],
        vec![(hex(0, 4), 1, 4), (hex(-3, 0), 3, 2)]
    );
    // At the end of the afternoon contact 1 reaches 5 pulses and is dropped,
    // and contact 3 with the raid on it. The Allied side holds no contact on
    // day 2.
    assert!(held["orders 2"].is_empty(), "{:?}", held["orders 2"]);
    assert!(held["end"].is_empty());
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
            hex: hex(0, 5),
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
        IntentKind::OperateCarriers {
            force: force(1),
            hex: hex(0, 0)
        }
    );
    assert_eq!(two.parent, Some(intent(1)));
    assert_eq!(two.maker, Maker::Staff);
    assert_eq!(two.addressee, Addressee::TaskForce(force(1)));
    assert_eq!(two.plan, one.plan);
    assert_eq!(two.set_day, 1);
    assert_eq!(two.withdrawn_day, None);
    let formed = played.all(|k| matches!(k, EventKind::IntentFormed { .. }));
    assert_eq!(formed.len(), 1);
    assert_eq!(formed[0].day, 1);
    // Base 1 has no air group, so no task is formed for it.
    assert!(!played.has_kind(&IntentKind::StrikeShips { base: base(1) }));
    assert!(!played.has_kind(&IntentKind::SearchWaters { base: base(1) }));
    assert!(!played.has_kind(&IntentKind::Cover { base: base(1) }));
}

#[test]
fn the_orders_of_day_1_are_ten_and_day_2_has_one() {
    let played = play(&text());
    played.nothing_refused();
    assert_eq!(played.orders(1).len(), 10, "{:?}", played.orders(1));
    assert_eq!(played.orders(1), day_1_orders());
    // The force has two orders, group 1 two, group 2 two and group 3 four,
    // and the force is given no route.
    assert_eq!(played.orders_to_force(1, 1).len(), 2);
    assert_eq!(played.orders_to_group(1, 1).len(), 2);
    assert_eq!(played.orders_to_group(1, 2).len(), 2);
    assert_eq!(played.orders_to_group(1, 3).len(), 4);
    assert!(!played
        .orders_to_force(1, 1)
        .iter()
        .any(|o| matches!(o, TaskForceOrder::SetRoute { .. })));
    // Day 2: one order, group 2's arc, 110 to 180 at 300 nm.
    assert_eq!(played.orders(2), vec![Given::Air(2, arc(110, 180))]);
    // Eleven in all, and every one serves intent 2.
    assert_eq!(played.order_count(), vec![10, 1]);
    assert_eq!(played.all_orders().len(), 11);
    assert_eq!(played.orders_serving(1, 2), day_1_orders());
    assert!(played.orders_serving(1, 1).is_empty());
    // No notice is handed in.
    for day in 1..=2 {
        assert!(played.decisions(day).notices.is_empty(), "day {day}");
    }
}

#[test]
fn every_order_serves_intent_2_through_a_chain_of_one_ending_with_the_commander() {
    let played = play(&text());
    for (day, order) in played.all_orders() {
        let reason = &order.reason;
        assert_eq!(order.intent, intent(2), "day {day}");
        assert_eq!(reason.serves.intent, Some(intent(2)), "day {day}");
        assert!(reason.serves.plan.is_some(), "day {day}");
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
fn every_order_is_decided_at_the_level_of_the_force_except_the_sizing_order() {
    let played = play(&text());
    let mut sizings = Vec::new();
    for (day, order) in played.all_orders() {
        if given(order) == Given::Air(3, sizing(true)) {
            // The group decided.
            assert_eq!(level(&order.reason), Some(Addressee::AirGroup(group(3))));
            assert!(
                order
                    .reason
                    .serves
                    .to_string()
                    .contains("decided at the level of air_group:3"),
                "{}",
                order.reason.serves
            );
            sizings.push(day);
        } else {
            assert_eq!(
                level(&order.reason),
                Some(Addressee::TaskForce(force(1))),
                "day {day}"
            );
            assert!(
                order
                    .reason
                    .serves
                    .to_string()
                    .contains("decided at the level of task_force:1"),
                "{}",
                order.reason.serves
            );
        }
    }
    assert_eq!(sizings, vec![1]);
}

#[test]
fn the_reasons_give_the_fuel_the_share_the_threats_and_the_arc() {
    let played = play(&text());
    // The force: the passage home is 200 nm, 66.666666 t at press against
    // 3000 aboard; it stays.
    let stays: Vec<String> = played.decisions(1).no_orders.iter().map(said).collect();
    assert!(
        stays.iter().any(
            |r| r.contains("ship:1 has 3000 t aboard against 66.666666 t needed")
                && r.contains("the 1 of the press risk")
                && r.contains("stays on its task")
        ),
        "{stays:?}"
    );
    // Battle readiness at its hex, and the stance avoid with the break-off
    // level of the risk.
    let r = said(reason_of(&played, 1, &Given::Force(1, BATTLE)));
    assert!(r.contains("at (0,0)"), "{r}");
    let r = said(reason_of(
        &played,
        1,
        &Given::Force(1, rules(Stance::Avoid, "0.75")),
    ));
    assert!(
        r.contains("the stance avoid and the break-off level of the press risk, 0.75"),
        "{r}"
    );
    // Group 1: the share 0.25 at press, the force not attacked.
    let r = said(reason_of(&played, 1, &Given::Air(1, share("0.25"))));
    assert!(
        r.contains("fatigue 0 not above the 60 of the press risk"),
        "{r}"
    );
    assert!(
        r.contains("a share of 0.25 (0.25 at press, the force not attacked from the air"),
        "{r}"
    );
    // Group 2: the threats, the narrowest arc and the widening.
    let r = said(reason_of(&played, 1, &Given::Air(2, arc(60, 300))));
    assert!(
        r.contains("a group that can search and cannot attack a ship"),
        "{r}"
    );
    assert!(r.contains("060 to 300 out to 300 nm from (0,0)"), "{r}");
    for threat in [
        "contact 1 at (0,4)",
        "contact 2 at (3,0)",
        "contact 3 at (-3,0)",
        "the waters to deny at (0,5)",
    ] {
        assert!(r.contains(threat), "{threat}: {r}");
    }
    assert!(r.contains("the narrowest arc that holds them is 090 to 270, widened by 30 degrees at each end to 060 to 300"), "{r}");
    // Group 3: the table of the risk, with sizing.
    let r = said(reason_of(&played, 1, &Given::Air(3, age(2))));
    assert!(
        r.contains("strikes ships by the table of the press risk, with its strikes sized"),
        "{r}"
    );
    // Day 2: the one threat is the waters, on 150: the arc is 110 to 180.
    let r = said(reason_of(&played, 2, &Given::Air(2, arc(110, 180))));
    assert!(r.contains("110 to 180 out to 300 nm from (0,0)"), "{r}");
    assert!(r.contains("(threats: the waters to deny at (0,5);"), "{r}");
    assert!(
        r.contains("widened by 30 degrees at each end to 110 to 180"),
        "{r}"
    );
}

#[test]
fn the_morning_of_day_1_sends_group_3s_10_at_contact_2() {
    let played = play(&text());
    assert_eq!(played.launches(1, Pulse::Morning), vec![(3, hex(3, 0), 10)]);
}

#[test]
fn the_afternoon_of_day_1_sends_group_3s_10_at_contact_3() {
    let played = play(&text());
    assert_eq!(
        played.launches(1, Pulse::Afternoon),
        vec![(3, hex(-3, 0), 10)]
    );
}

#[test]
fn two_strikes_of_10_nothing_on_day_2_and_each_raid_finds_nothing() {
    let played = play(&text());
    assert!(played.launches(1, Pulse::Night).is_empty());
    for pulse in [Pulse::Night, Pulse::Morning, Pulse::Afternoon] {
        assert!(played.launches(2, pulse).is_empty());
    }
    let all = played.all_launches();
    assert_eq!(all, vec![(3, hex(3, 0), 10), (3, hex(-3, 0), 10)]);
    assert_eq!(played.launched_by(3), 20);
    assert_eq!(played.launched_by(1), 0);
    assert_eq!(played.launched_by(2), 0);
    let nothing: Vec<Hex> = played
        .all(|k| matches!(k, EventKind::StrikeFoundNothing { .. }))
        .iter()
        .map(|e| match &e.kind {
            EventKind::StrikeFoundNothing { hex, .. } => *hex,
            _ => unreachable!(),
        })
        .collect();
    assert_eq!(nothing, vec![hex(3, 0), hex(-3, 0)]);
    assert!(played
        .all(|k| matches!(k, EventKind::AirAttack { .. }))
        .is_empty());
    // Contact 1 never qualified: no strike went to (0,4).
    assert!(!all.iter().any(|(_, h, _)| *h == hex(0, 4)));
}

#[test]
fn group_2_searches_in_all_four_daylight_pulses_with_its_6_and_sights_nothing() {
    let played = play(&text());
    assert_eq!(
        played.searches(),
        vec![
            (1, Some(Pulse::Morning), 2, 60, 300, n(300), 6),
            (1, Some(Pulse::Afternoon), 2, 60, 300, n(300), 6),
            (2, Some(Pulse::Morning), 2, 110, 180, n(300), 6),
            (2, Some(Pulse::Afternoon), 2, 110, 180, n(300), 6),
        ]
    );
    // There is no enemy to sight.
    assert!(played
        .all(|k| matches!(
            k,
            EventKind::SearchAttempt { .. } | EventKind::ReportReceived { .. }
        ))
        .iter()
        .all(|e| e.day == 0));
}

#[test]
fn the_force_has_not_moved_and_has_burnt_no_fuel_under_way() {
    let played = play(&text());
    assert!(played
        .all(|k| matches!(
            k,
            EventKind::TaskForceSteamed { .. }
                | EventKind::FuelBurned { .. }
                | EventKind::TaskForceArrived { .. }
                | EventKind::TaskForceHalted { .. }
        ))
        .is_empty());
    for t in &played.truth {
        assert_eq!(t.task_forces[&force(1)].position(), hex(0, 0));
        assert_eq!(t.ships[&ship(1)].fuel_t, n(3000));
    }
    assert_eq!(played.force_at_end().position(), hex(0, 0));
}

#[test]
fn the_groups_end_as_they_were_ordered_and_keep_their_aircraft() {
    let played = play(&text());
    for (raw, ready) in [(1, 12), (2, 6), (3, 10)] {
        assert_eq!(played.ready_at_end(raw), ready, "group {raw}");
    }
    // Group 1 on patrol, with a share of 0.25: 12 x 0.25 = 3 overhead in
    // each daylight pulse.
    assert_eq!(played.mission_at_end(1), CAP);
    let g1 = played.air_group_at_end(1);
    assert_eq!(g1.patrol_share, fx("0.25"));
    assert_eq!(n(i64::from(g1.ready)) * g1.patrol_share, n(3));
    // Group 2 searches, group 3 strikes.
    assert_eq!(played.mission_at_end(2), SEARCH);
    let arc = played.air_group_at_end(2).search.expect("an arc");
    assert_eq!((arc.from_deg, arc.to_deg, arc.range_nm), (110, 180, n(300)));
    assert_eq!(played.mission_at_end(3), STRIKE);
    let g3 = played.air_group_at_end(3);
    assert!(g3.strike_sizing);
    assert_eq!(g3.reach, Reach::Extended);
    // The force: battle readiness, the stance avoid at 0.75, home port 1.
    let f = played.force_at_end();
    assert_eq!(f.readiness, Readiness::Battle);
    assert_eq!(f.rules_of_engagement.stance, Stance::Avoid);
    assert_eq!(f.rules_of_engagement.break_off, fx("0.75"));
    assert_eq!(f.home_port, Some(base(1)));
}

#[test]
fn no_staff_order_is_refused_and_the_commands_are_taken() {
    let played = play(&text());
    played.nothing_refused();
    assert!(played
        .all(|k| matches!(k, EventKind::CommandRefused { .. }))
        .is_empty());
}

// Every staffed scenario built before this one gives the orders it gave.
// That is checked by the lead with a script, not by a test.

// --------------------------------------------------------------- Properties

fn without_risk(text: &str) -> String {
    changed(
        text,
        "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"press\"\n",
        "",
    )
}

/// The air orders of a balanced morning: the same whatever the force does.
fn balanced_air_orders() -> Vec<Given> {
    vec![
        Given::Air(1, mission(CAP)),
        Given::Air(1, share("0.5")),
        Given::Air(2, mission(SEARCH)),
        Given::Air(2, arc(60, 300)),
        Given::Air(3, mission(STRIKE)),
        Given::Air(3, sizing(true)),
    ]
}

fn air_orders(played: &Played, day: u32) -> Vec<Given> {
    played
        .orders(day)
        .into_iter()
        .filter(|g| matches!(g, Given::Air(..)))
        .collect()
}

#[test]
fn balanced_gives_group_1_a_half_and_group_3_two_orders_and_launches_nothing() {
    let played = play(&without_risk(&text()));
    played.nothing_refused();
    // Eight orders on day 1.
    assert_eq!(played.orders(1).len(), 8, "{:?}", played.orders(1));
    assert_eq!(
        played.orders(1),
        [
            vec![
                Given::Force(1, BATTLE),
                Given::Force(1, rules(Stance::Avoid, "0.5")),
            ],
            balanced_air_orders(),
        ]
        .concat()
    );
    // Group 1's share is 0.5 and it keeps 6 overhead.
    let g1 = played.air_group_at_end(1);
    assert_eq!(g1.patrol_share, fx("0.5"));
    assert_eq!(n(i64::from(g1.ready)) * g1.patrol_share, n(6));
    // Group 3 is given two orders, the mission and sizing on, and launches
    // nothing.
    assert_eq!(
        played.orders_to_group(1, 3),
        vec![mission(STRIKE), sizing(true)]
    );
    assert!(played.all_launches().is_empty());
    assert_eq!(played.launched_by(3), 0);
    // The break-off level is 0.5.
    assert_eq!(
        played.force_at_end().rules_of_engagement.break_off,
        fx("0.5")
    );
    // Group 2 still searches in all four daylight pulses.
    assert_eq!(played.searches().len(), 4);
}

fn the_scenario_with_fuel(tons: u32) -> String {
    changed(
        &without_risk(&text()),
        "fuel_t = 3000",
        &format!("fuel_t = {tons}"),
    )
}

#[test]
fn fuel_takes_her_home_79_t_against_79_999999_and_she_is_routed_home() {
    let played = play(&the_scenario_with_fuel(79));
    played.nothing_refused();
    // On day 1 the force is routed to base 1, which is (5,0).
    assert!(
        played.orders_to_force(1, 1).contains(&route(&[(5, 0)])),
        "{:?}",
        played.orders_to_force(1, 1)
    );
    // Its groups are given their orders as above.
    assert_eq!(air_orders(&played, 1), balanced_air_orders());
    // The reason gives 79 against the need: 66.666666 x 1.2 = 79.999999 in
    // the engine's fixed point (restated by the architect).
    let r = said(reason_of(&played, 1, &Given::Force(1, route(&[(5, 0)]))));
    assert!(
        r.contains("ship:1 has 79 t aboard against 79.999999 t needed"),
        "{r}"
    );
    assert!(r.contains("the 1.2 of the balanced risk"), "{r}");
    assert!(r.contains("routed home to base:1"), "{r}");
    // And the force sails: it is under way, not at its hex.
    assert!(!played
        .all(|k| matches!(k, EventKind::TaskForceSteamed { .. }))
        .is_empty());
    played.every_chain_ends_with_the_commander();
}

#[test]
fn fuel_takes_her_home_with_81_t_she_stays() {
    let played = play(&the_scenario_with_fuel(81));
    played.nothing_refused();
    assert!(!played
        .orders_to_force(1, 1)
        .iter()
        .any(|o| matches!(o, TaskForceOrder::SetRoute { .. })));
    assert_eq!(
        played.orders_to_force(1, 1),
        vec![BATTLE, rules(Stance::Avoid, "0.5")]
    );
    assert_eq!(air_orders(&played, 1), balanced_air_orders());
    assert!(played
        .all(|k| matches!(k, EventKind::TaskForceSteamed { .. }))
        .is_empty());
    assert_eq!(played.force_at_end().position(), hex(0, 0));
}

fn commander_hex() -> String {
    let t = text();
    let at = t.find(COMMANDS_MARKER).expect("the marker");
    format!(
        "{}{}{}{}",
        &t[..at],
        command_table(
            1,
            "kind = \"set_intent\"\nintent = 1\nintent_kind = \"operate_carriers\"\ntask_force = 1\nhex = [2, 0]"
        ),
        command_table(1, "kind = \"set_leave\"\nleave = \"prepare\"\ngiven = true"),
        command_table(1, "kind = \"set_leave\"\nleave = \"launch\"\ngiven = true"),
    )
}

#[test]
fn a_commanders_hex_the_force_is_routed_to_2_0_at_cruise_and_the_air_orders_are_balanced() {
    let played = play(&commander_hex());
    played.nothing_refused();
    let one = &played.state.intents[&intent(1)];
    assert_eq!(
        one.kind,
        IntentKind::OperateCarriers {
            force: force(1),
            hex: hex(2, 0)
        }
    );
    assert_eq!(one.parent, None);
    assert_eq!(one.maker, Maker::Commander);
    assert_eq!(one.addressee, Addressee::TaskForce(force(1)));
    assert_eq!(played.state.intents.len(), 1);
    // The force is routed to (2,0), and no speed order is given: it is at
    // cruise.
    let forces = played.orders_to_force(1, 1);
    assert!(forces.contains(&route(&[(2, 0)])), "{forces:?}");
    assert!(!forces
        .iter()
        .any(|o| matches!(o, TaskForceOrder::SetSpeed(_))));
    assert_eq!(played.force_at_end().speed, SpeedSetting::Cruise);
    assert_eq!(played.force_at_end().position(), hex(2, 0));
    // The air orders are those of balanced.
    assert_eq!(air_orders(&played, 1), balanced_air_orders());
    played.every_chain_ends_with_the_commander();
    for (_, order) in played.all_orders() {
        assert_eq!(order.intent, intent(1));
    }
}

#[test]
fn nothing_to_fly_with_no_air_group_aboard_no_task_is_formed() {
    let t = text();
    let from = t.find("# The groups aboard").expect("the groups");
    let to = t.find(CONTACTS_MARKER).expect("the contacts");
    let played = play(&format!("{}{}", &t[..from], &t[to..]));
    played.nothing_refused();
    assert_eq!(played.state.intents.len(), 1);
    assert!(!played
        .state
        .intents
        .values()
        .any(|i| matches!(i.kind, IntentKind::OperateCarriers { .. })));
    assert!(played.all_orders().is_empty());
}

#[test]
fn nothing_to_fly_with_every_group_at_no_ready_aircraft_the_staff_cannot_meet_the_intent() {
    let zero = text()
        .replace("ready = 12", "ready = 0")
        .replace("ready = 6", "ready = 0")
        .replace("ready = 10", "ready = 0");
    let played = play(&zero);
    played.nothing_refused();
    assert!(played.all_orders().is_empty());
    assert_eq!(played.cannot_meet(1), 1);
    let none: Vec<String> = played.decisions(1).no_orders.iter().map(said).collect();
    assert!(
        none.iter().any(|r| r
            .contains("no air group aboard task_force:1 has an aircraft able to fly")
            && r.contains("cannot meet intent:2")),
        "{none:?}"
    );
    assert!(played.all_launches().is_empty());
}

fn with_force_2() -> String {
    let t = format!(
        "{}\n[[task_force]]\nid = 2\nname = \"Placeholder cruiser force 2\"\nside = \"allied\"\nat_base = 1\nmission = \"surface_combat\"\n\n[[ship]]\nid = 2\nname = \"Placeholder cruiser 2\"\nclass = 1\nside = \"allied\"\nfuel_t = 2000\ntask_force = 2\n\n[[ship]]\nid = 3\nname = \"Placeholder cruiser 3\"\nclass = 1\nside = \"allied\"\nfuel_t = 2000\ntask_force = 2\n",
        text()
    );
    changed(&t, "task_forces = [1]", "task_forces = [1, 2]")
}

#[test]
fn a_surface_force_and_a_carrier_force_named_together_each_get_their_own_task() {
    let played = play(&with_force_2());
    played.nothing_refused();
    assert_eq!(played.state.intents.len(), 3);
    // Task force number order: the carrier task for force 1, the stand task
    // (decision 0124) for force 2.
    let carrier = &played.state.intents[&intent(2)];
    assert_eq!(
        carrier.kind,
        IntentKind::OperateCarriers {
            force: force(1),
            hex: hex(0, 0)
        }
    );
    assert_eq!(carrier.parent, Some(intent(1)));
    assert_eq!(carrier.maker, Maker::Staff);
    assert_eq!(carrier.addressee, Addressee::TaskForce(force(1)));
    let stand = &played.state.intents[&intent(3)];
    assert_eq!(
        stand.kind,
        IntentKind::StandAt {
            force: force(2),
            hex: hex(0, 5)
        }
    );
    assert_eq!(stand.parent, Some(intent(1)));
    assert_eq!(stand.maker, Maker::Staff);
    assert_eq!(stand.addressee, Addressee::TaskForce(force(2)));
    // Force 1 is ordered by the carrier doctrine: the orders of the main
    // case.
    let carrier_orders: Vec<Given> = played.orders_serving(1, 2);
    assert_eq!(carrier_orders, day_1_orders());
    // Force 2 is ordered by the stand doctrine: it serves intent 3, at the
    // level of force 2, and the orders are to force 2 alone.
    let stand_orders = played.orders_serving(1, 3);
    assert!(!stand_orders.is_empty());
    assert!(
        stand_orders.iter().all(|g| matches!(g, Given::Force(2, _))),
        "{stand_orders:?}"
    );
    assert!(stand_orders.contains(&Given::Force(2, TaskForceOrder::SetHomePort(base(1)))));
    assert!(stand_orders.contains(&Given::Force(2, BATTLE)));
    assert!(stand_orders.iter().any(|g| matches!(
        g,
        Given::Force(2, TaskForceOrder::SetRoute { stand_at, .. }) if stand_at.contains(&hex(0, 5))
    )));
    for (day, order) in played.all_orders() {
        if order.intent == intent(3) {
            assert_eq!(
                level(&order.reason),
                Some(Addressee::TaskForce(force(2))),
                "day {day}"
            );
        }
    }
    // Nothing else is ordered: 10 for the carrier task and the rest the
    // stand task's.
    assert_eq!(played.orders(1).len(), 10 + stand_orders.len());
    // The carrier force's game is unchanged: its strikes and searches.
    assert_eq!(
        played.all_launches(),
        vec![(3, hex(3, 0), 10), (3, hex(-3, 0), 10)]
    );
    assert_eq!(played.searches().len(), 4);
    played.every_chain_ends_with_the_commander();
}

#[test]
fn one_task_a_force_the_commanders_task_stands_and_the_staff_forms_none() {
    let played = play(&with(&command_table(
        1,
        "kind = \"set_intent\"\nintent = 2\nintent_kind = \"operate_carriers\"\ntask_force = 1\nhex = [0, 0]",
    )));
    played.nothing_refused();
    // No task is formed for force 1 under intent 1: the tree is the two
    // roots the commander stated.
    assert_eq!(played.state.intents.len(), 2, "{:?}", played.state.intents);
    let two = &played.state.intents[&intent(2)];
    assert_eq!(two.maker, Maker::Commander);
    assert_eq!(two.parent, None);
    assert!(played
        .state
        .intents
        .values()
        .all(|i| i.parent != Some(intent(1))));
    // Its reason names intent 2.
    let none: Vec<String> = played.decisions(1).no_orders.iter().map(said).collect();
    assert!(
        none.iter()
            .any(|r| r.contains("intent:2") && r.contains("forms no other for task_force:1")),
        "{none:?}"
    );
    // Every order is given once, serving intent 2, at balanced: 8 on day 1.
    assert_eq!(played.orders(1).len(), 8, "{:?}", played.orders(1));
    assert_eq!(
        played.orders(1),
        [
            vec![
                Given::Force(1, BATTLE),
                Given::Force(1, rules(Stance::Avoid, "0.5")),
            ],
            balanced_air_orders(),
        ]
        .concat()
    );
    for (_, order) in played.all_orders() {
        assert_eq!(order.intent, intent(2));
    }
}

#[test]
fn a_held_force_is_left_alone_with_its_groups() {
    let played = play(&with(&command_table(1, "kind = \"hold\"\ntask_force = 1")));
    played.nothing_refused();
    assert!(played.all_orders().is_empty());
    assert!(played.all_launches().is_empty());
    assert!(played.searches().is_empty());
    let none: Vec<String> = played.decisions(1).no_orders.iter().map(said).collect();
    assert!(
        none.iter()
            .any(|r| r.contains("task_force:1 is held by the commander")),
        "{none:?}"
    );
}

#[test]
fn a_held_group_is_left_alone() {
    // Group 3 held on day 1: the staff gives it no order and it flies
    // nothing; the others are ordered as in the main case.
    let played = play(&with(&command_table(1, "kind = \"hold\"\nair_group = 3")));
    played.nothing_refused();
    assert!(played.orders_to_group(1, 3).is_empty());
    assert_eq!(
        played.orders_to_group(1, 1),
        vec![mission(CAP), share("0.25")]
    );
    assert_eq!(
        played.orders_to_group(1, 2),
        vec![mission(SEARCH), arc(60, 300)]
    );
    assert_eq!(played.orders(1).len(), 6);
    assert_eq!(played.launched_by(3), 0);
    assert_eq!(played.mission_at_end(3), AirMission::Rest);
    assert_eq!(played.ready_at_end(3), 10);
    // Group 2 held on day 2: its arc is not narrowed that day.
    let played = play(&with(&command_table(2, "kind = \"hold\"\nair_group = 2")));
    played.nothing_refused();
    assert_eq!(played.orders(1), day_1_orders());
    assert!(played.orders(2).is_empty());
    assert!(played.searches().iter().all(|s| (s.3, s.4) == (60, 300)));
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

// ----------------------------------------------- Pure decisions of the force

use sim_core::command::Risk;
use sim_core::refdata::ShipKind;
use sim_staff::carrier::{decide, patrol_share, roles, CarrierGroup, CarrierRole};

/// A group as the decision reads it: resting, with no setting, 6 aircraft
/// able to fly (12 for a fighter group, 10 for a bomber group below).
fn view_of(id: u32, fighter: bool, can_search: bool, can_strike: bool, able: u32) -> CarrierGroup {
    CarrierGroup {
        id: group(id),
        fighter,
        can_search,
        can_strike,
        fatigue: Fixed::ZERO,
        able,
        mission: AirMission::Rest,
        patrol_share: Fixed::ONE,
        search: None,
        search_range_nm: n(300),
        contact_age: 0,
        target_priority: ShipKind::ALL.to_vec(),
        reach: Reach::Normal,
        held_share: Fixed::ZERO,
        sizing: false,
    }
}

#[test]
fn no_scout_the_lowest_group_that_can_do_both_searches_and_the_other_strikes() {
    // The placeholder dive bomber has no search range (above), so a view is
    // built for the decision with a type that has both: group 1 the fighter,
    // groups 3 and 4 two bomber groups of 10 that can search and attack.
    let groups = vec![
        view_of(1, true, false, false, 12),
        view_of(3, false, true, true, 10),
        view_of(4, false, true, true, 10),
    ];
    // No group can only search: group 3, the lowest that can do both, is
    // the searcher, and group 4 strikes.
    assert_eq!(
        roles(&groups),
        vec![
            (group(1), CarrierRole::Fighter),
            (group(3), CarrierRole::Searcher),
            (group(4), CarrierRole::Striker),
        ]
    );
    let air = decide(Risk::Press, 2, false, 90, 270, &groups);
    assert_eq!(air.roles, roles(&groups));
    let of = |raw: u32| -> Vec<AirGroupOrder> {
        air.orders
            .iter()
            .filter(|(g, _)| g.raw() == raw)
            .map(|(_, o)| o.clone())
            .collect()
    };
    // Group 3 is set to search, over the arc, and never to strike.
    assert_eq!(
        of(3),
        vec![
            mission(SEARCH),
            AirGroupOrder::SetSearchArc {
                from_deg: 90,
                to_deg: 270,
                range_nm: n(300)
            }
        ]
    );
    // Group 4 is set to strike, by the table, with sizing on.
    let four = of(4);
    assert_eq!(four.first(), Some(&mission(STRIKE)));
    assert!(four.contains(&sizing(true)), "{four:?}");
    assert!(!four.contains(&mission(SEARCH)));
    // Group 1 flies the patrol at press: a quarter.
    assert_eq!(of(1), vec![mission(CAP), share("0.25")]);
    assert_eq!(air.share, fx("0.25"));
    assert!(air.rested.is_empty());
    // With a scout in the force (group 2), it searches and both bombers
    // strike: the main case's roles.
    let with_scout = vec![
        view_of(1, true, false, false, 12),
        view_of(2, false, true, false, 6),
        view_of(3, false, false, true, 10),
    ];
    assert_eq!(
        roles(&with_scout),
        vec![
            (group(1), CarrierRole::Fighter),
            (group(2), CarrierRole::Searcher),
            (group(3), CarrierRole::Striker),
        ]
    );
}

#[test]
fn a_raid_on_the_force_raises_the_patrol_to_three_quarters_at_balanced() {
    // The patrol share at balanced is a half; raised by a quarter after an
    // air attack on the force in the three days before: three quarters.
    assert_eq!(patrol_share(Risk::Balanced, false), fx("0.5"));
    assert_eq!(patrol_share(Risk::Balanced, true), fx("0.75"));
    let groups = vec![
        view_of(1, true, false, false, 12),
        view_of(2, false, true, false, 6),
        view_of(3, false, false, true, 10),
    ];
    let calm = decide(Risk::Balanced, 0, false, 60, 300, &groups);
    assert_eq!(calm.share, fx("0.5"));
    assert!(calm.orders.contains(&(group(1), share("0.5"))));
    let raided = decide(Risk::Balanced, 0, true, 60, 300, &groups);
    assert_eq!(raided.share, fx("0.75"));
    assert!(raided.orders.contains(&(group(1), share("0.75"))));
    assert!(!raided.orders.contains(&(group(1), share("0.5"))));
    // 12 x 0.75 = 9 overhead.
    assert_eq!(n(12) * raided.share, n(9));
}
