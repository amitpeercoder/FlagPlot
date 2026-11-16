//! Scenario 53: the commander states one end, "deny the waters at (0,5)",
//! at press. The staff at the area's level gives base 1 the tasks of
//! striking ships and of searching those waters; at the base's level it
//! sets the bombers' standing orders; and each strike is sized at the
//! group's level.
//!
//! Proves decisions 0123 and 0118 together against a hand calculation,
//! with scenario 49 for decision 0118 alone. The data is placeholder data:
//! see `data/scenarios/53_deny_an_area.toml`. It runs on the frozen step 1
//! figures with the table of aircraft for each ship added, as below. The
//! upkeep, wear, fatigue and supply figures are not loaded. There is no
//! enemy unit: every strike finds nothing, the search sights nothing, and
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
//! As scenarios 46 and 49: `strike_contact_max_age_pulses` 0; a contact
//! gains a pulse of age at the end of each pulse and is dropped at 5; a
//! hex is 40 nm; the placeholder dive bomber (type 3) has a strike radius
//! of 250 nm; strikes are decided in a daylight pulse in group number
//! order; a group takes the kind first on its target list, then the
//! nearest. The placeholder flying boat (type 1) searches to 600 nm. From
//! (0,0) the hex (0,5) lies on 150, as scenario 47 has it.
//!
//! Aircraft for each ship reported, in this scenario's own figures:
//! cruiser 12, destroyer 6, transport 6, auxiliary 4, and no figure for a
//! carrier or a battleship, which need everything.
//!
//! ## Inputs
//!
//! A placeholder map of open sea. Allied base 1 at (0,0). The game is 2
//! days. There is no Japanese unit and no Japanese base.
//!
//! | Group | Type        | Ready aircraft | At the start |
//! | ----- | ----------- | -------------- | ------------ |
//! | 1     | dive bomber | 10             | resting, no setting |
//! | 2     | dive bomber | 10             | resting, no setting |
//! | 3     | dive bomber | 10             | resting, no setting |
//! | 4     | flying boat | 6              | resting, no arc |
//! | 5     | fighter     | 8              | resting, no setting |
//!
//! The Allied side starts holding three contacts, numbered from the oldest
//! down:
//!
//! | Contact | Hex    | Ships | Kind      | Age at the start | From base 1 | Need |
//! | ------- | ------ | ----- | --------- | ---------------- | ----------- | ---- |
//! | 1       | (0,4)  | 1     | carrier   | 2                | 160 nm      | everything |
//! | 2       | (3,0)  | 2     | cruiser   | 1                | 120 nm      | 24 |
//! | 3       | (-3,0) | 3     | transport | 0                | 120 nm      | 18 |
//!
//! The Allied side's commands of day 1: intent 1, deny the waters within 0
//! hexes of (0,5); its risk, press; leave to prepare and leave to launch
//! both given. No forces limit.
//!
//! ## The tree on the first morning
//!
//! The waters are the one hex (0,5), 200 nm from base 1. The bombers reach
//! it, 200 against 250, and the flying boats, 200 against 600. Base 1 has
//! no cover. So the staff forms two tasks:
//!
//! | Intent | Kind | Parent | Maker | Addressee |
//! | ------ | ---- | ------ | ----- | --------- |
//! | 1      | deny the waters at (0,5) | none | the commander | the area |
//! | 2      | strike ships from base 1 | 1 | the staff | base 1 |
//! | 3      | search the waters from base 1 | 1 | the staff | base 1 |
//!
//! ## The orders of day 1
//!
//! At press, for each of groups 1, 2 and 3, four orders serving intent 2:
//! the naval strike mission; the oldest contact, 2 (the figure, 0, and two
//! more); extended reach; and strike sizing on. The target list is every
//! kind and the held share none, which is what the groups have: no order.
//! The reason of each sizing order says the group decided, from its task;
//! the others say the base decided.
//!
//! For group 4, two orders serving intent 3: the search mission, and the
//! arc. The one threat is (0,5), on 150: the arcs of ten degrees that hold
//! it are 140 to 150 and 150 to 160, and the lower start is taken. Widened
//! by 30 at each end it is 110 to 180, at 600 nm.
//!
//! For group 5, two orders serving intent 2: combat air patrol, and a
//! patrol share of 0.5.
//!
//! Sixteen orders on day 1 and none on day 2.
//!
//! ## The morning of day 1
//!
//! After the night the contacts are 3, 2 and 1 pulses old. The groups
//! launch on a contact no older than 2: contact 1, the carrier, at 3, does
//! not qualify. Contacts 2 and 3 do, and each group's order among them is
//! contact 2, the cruisers, first.
//!
//! | Group | Can send | Contact | Its need | Sent there before | It sends |
//! | ----- | -------- | ------- | -------- | ----------------- | -------- |
//! | 1     | 10       | 2       | 24       | 0                 | 10       |
//! | 2     | 10       | 2       | 24       | 10                | 10       |
//! | 3     | 10       | 2       | 24       | 20                | 4        |
//!
//! One raid of 24 flies to (3,0), finds nothing, and contact 2 is dropped,
//! by group 1's strike. Nothing is sent at contact 3 in the morning: each
//! group launches once in a pulse.
//!
//! ## The afternoon of day 1
//!
//! Contact 1 is 4 pulses old and contact 3 is 2. Only contact 3 qualifies.
//!
//! | Group | Can send | Contact | Its need | Sent there before | It sends |
//! | ----- | -------- | ------- | -------- | ----------------- | -------- |
//! | 1     | 10       | 3       | 18       | 0                 | 10       |
//! | 2     | 10       | 3       | 18       | 10                | 8        |
//! | 3     | 10       | none    |          |                   | 0        |
//!
//! One raid of 18 flies to (-3,0), finds nothing, and contact 3 is
//! dropped. At the end of the afternoon contact 1 reaches 5 pulses and is
//! dropped.
//!
//! ## Day 2
//!
//! The Allied side holds no contact. Nothing is launched. Group 4 searches
//! in both daylight pulses of both days, with all 6, and sights nothing.
//! Group 5 keeps 4 overhead in every daylight pulse.
//!
//! ## At the end
//!
//! Five strikes were launched: 10, 10 and 4, then 10 and 8: 42 aircraft.
//! Every group has the aircraft it began with. The measure of intent 1
//! reads: 2 days in force; five strikes launched; no contact held in the
//! waters at the end of either day, so it was met on both. The contacts
//! struck lay outside the waters: a base given the task strikes what its
//! groups can reach.
//!
//! # Properties
//!
//! - **Balanced.** With no risk set: each bomber group is given two
//!   orders, the mission and sizing on. Group 4 is given three: the search
//!   mission, search once a day and the arc, in that order (restated by
//!   the architect on Nov 16, 2026, for decision 0131; it had two). Day 1
//!   has six orders for the bombers, three for group 4 and two for group
//!   5: eleven. Group 4 searches in the morning only. No strike is
//!   launched on either day: every contact is a pulse old or more in the
//!   morning, against the figure of 0.
//! - **Preserve the force.** Each bomber group is given four orders: the
//!   mission, sizing on, a held share of 0.333333 and the target list
//!   carrier, battleship, cruiser. No strike is launched.
//! - **The carrier needs everything.** At press, with contact 1 one pulse
//!   younger at the start, so that it qualifies in the morning: groups 1,
//!   2 and 3 each send their 10 at it, 30 in one raid, and nothing is sent
//!   at contact 2 in the morning.
//! - **A base with a cover has one arc.** With the commander also stating,
//!   first, intent 1, hold base 1, and the deny end as intent 2: the cover
//!   is intent 3 and has groups 4 and 5; the strike task is intent 4; no
//!   search task is formed; and group 4's arc, with (0,5) the only threat,
//!   is 110 to 180.
//! - **Out of reach.** With the waters at (0,7), 280 nm: no strike task is
//!   formed; the search task is, 280 against 600. With the waters 16 hexes
//!   away, 640 nm: neither is formed, and the staff hands in on day 1 that
//!   it cannot meet intent 1.
//! - **A commander's order overrides the size.** With the commander
//!   ordering sizing off on group 3 on day 2, when the strike task that
//!   may use it is in force: the order is taken, group 3 is held from
//!   then, and the staff does not set its sizing on again: it gives group
//!   3 no order on day 2. The same order on day 1 is taken before the
//!   task is formed and holds nothing: the staff gives group 3 its four
//!   orders that morning, sizing on among them, and it sends 4 at contact
//!   2 as in the main case. With a hold on group 3 on day 1, the staff
//!   gives it no order and it flies nothing, having no mission.
//!   (Restated by the architect on Nov 16, 2026, after the build: the
//!   first draft had the day 1 order hold the group.)
//! - **Withdrawing the end** on day 2 withdraws both tasks that day.
//! - The staff orders no group a task may not use; the same start gives
//!   the same orders; two worlds that differ only in what the Allied side
//!   cannot see give the same intents, orders and reasons.
//! - Every staffed scenario built before this one gives the orders it
//!   gave.

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_agent::{deny_measure, DenyMeasure};
use sim_core::command::{
    Addressee, CommandKind, IntentKind, Maker, NoticeKind, Reason, StaffDecisions, StaffOrder,
};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, IntentId};
use sim_core::orders::{AirGroupOrder, AirMission, OrderKind, Reach};
use sim_core::refdata::ShipKind;
use sim_core::state::{Side, TruthState};
use sim_core::time::Pulse;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

mod common;

const COMMANDS_MARKER: &str = "# The Allied commander's commands";
const CONTACTS_MARKER: &str = "# The three contacts";

fn path() -> PathBuf {
    common::data_dir().join("scenarios/53_deny_an_area.toml")
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

/// The scenario's own figures: the table of aircraft for each ship, added
/// after the frozen file's empty `[strike_aircraft_per_ship]` table.
fn figures() -> sim_data::LoadOptions {
    common::altered_reference("s53", |dir| {
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
    scenario: Scenario,
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
        scenario,
    }
}

fn mission(m: AirMission) -> AirGroupOrder {
    AirGroupOrder::SetAirMission(m)
}

fn age(pulses: i64) -> AirGroupOrder {
    AirGroupOrder::SetStrikeContactAge { pulses }
}

fn held(s: &str) -> AirGroupOrder {
    AirGroupOrder::SetStrikeHeldShare { share: fx(s) }
}

fn sizing(on: bool) -> AirGroupOrder {
    AirGroupOrder::SetStrikeSizing { on }
}

fn arc(from_deg: i32, to_deg: i32) -> AirGroupOrder {
    AirGroupOrder::SetSearchArc {
        from_deg,
        to_deg,
        range_nm: n(600),
    }
}

const STRIKE: AirMission = AirMission::NavalStrike;
const CAP: AirMission = AirMission::CombatAirPatrol;

fn air_order(o: &StaffOrder) -> (u32, AirGroupOrder) {
    match &o.kind {
        OrderKind::AirGroup { target, order } => (target.raw(), order.clone()),
        other => panic!("the doctrine gave a non-air order: {other:?}"),
    }
}

impl Played {
    fn decisions(&self, day: u32) -> &StaffDecisions {
        &self.handed[&day]
    }

    fn orders(&self, day: u32) -> Vec<(u32, AirGroupOrder)> {
        self.decisions(day).orders.iter().map(air_order).collect()
    }

    fn orders_serving(&self, day: u32, served: u32) -> Vec<(u32, AirGroupOrder)> {
        self.decisions(day)
            .orders
            .iter()
            .filter(|o| o.intent == intent(served))
            .map(air_order)
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

    fn ready_at_end(&self, raw: u32) -> u32 {
        self.truth.last().expect("a state").air_groups[&group(raw)].ready
    }

    fn mission_at_end(&self, raw: u32) -> AirMission {
        self.truth.last().expect("a state").air_groups[&group(raw)].mission
    }

    fn measure(&self, raw: u32) -> Option<DenyMeasure> {
        deny_measure(&self.scenario.world.side_view(Side::Allied), intent(raw))
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

/// The orders of the main case on day 1 serving intent 2, at press: for each
/// bomber group the mission, the oldest contact (2), extended reach and
/// sizing on; then group 5's two.
fn strike_task_orders() -> Vec<(u32, AirGroupOrder)> {
    let mut v = Vec::new();
    for g in 1..=3 {
        v.push((g, mission(STRIKE)));
        v.push((g, age(2)));
        v.push((
            g,
            AirGroupOrder::SetReach {
                reach: Reach::Extended,
            },
        ));
        v.push((g, sizing(true)));
    }
    v.push((5, mission(CAP)));
    v.push((5, AirGroupOrder::SetPatrolShare { share: fx("0.5") }));
    v
}

fn search_task_orders() -> Vec<(u32, AirGroupOrder)> {
    vec![(4, mission(AirMission::Search)), (4, arc(110, 180))]
}

// ------------------------------------------------------- The hand calculation

#[test]
fn the_commands_of_the_file_are_the_ones_these_tests_vary() {
    let t = text();
    let at = t.find(COMMANDS_MARKER).expect("the marker");
    let c = &t[at..];
    assert_eq!(c.matches("[[command]]").count(), 4);
    assert!(c.contains("intent_kind = \"deny\"\nhex = [0, 5]\ndistance = 0\n"));
    assert!(c.contains("risk = \"press\""));
    assert!(!c.contains("set_leave\"\nleave = \"prepare\"\ngiven = false"));
    assert!(t.contains(CONTACTS_MARKER));
}

#[test]
fn the_tree_on_the_first_morning() {
    let played = play(&text());
    played.nothing_refused();
    let state = &played.state;
    assert_eq!(state.intents.len(), 3);
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
    assert_eq!(two.kind, IntentKind::StrikeShips { base: base(1) });
    assert_eq!(two.parent, Some(intent(1)));
    assert_eq!(two.maker, Maker::Staff);
    assert_eq!(two.addressee, Addressee::Base(base(1)));
    assert_eq!(two.plan, one.plan);
    assert_eq!(two.set_day, 1);
    assert_eq!(two.withdrawn_day, None);
    let three = &state.intents[&intent(3)];
    assert_eq!(three.kind, IntentKind::SearchWaters { base: base(1) });
    assert_eq!(three.parent, Some(intent(1)));
    assert_eq!(three.maker, Maker::Staff);
    assert_eq!(three.addressee, Addressee::Base(base(1)));
    assert_eq!(three.plan, one.plan);
    assert_eq!(three.set_day, 1);
    assert_eq!(three.withdrawn_day, None);
    // Both tasks are formed on day 1, by the staff, under intent 1.
    let formed = played.all(|k| matches!(k, EventKind::IntentFormed { .. }));
    assert_eq!(formed.len(), 2);
    for e in formed {
        assert_eq!(e.day, 1);
    }
}

#[test]
fn the_orders_of_day_1_are_sixteen_and_day_2_has_none() {
    let played = play(&text());
    played.nothing_refused();
    assert_eq!(played.orders(1).len(), 16, "{:?}", played.orders(1));
    assert!(played.orders(2).is_empty(), "{:?}", played.orders(2));
    assert_eq!(
        played.orders_serving(1, 2),
        strike_task_orders(),
        "the orders serving intent 2"
    );
    assert_eq!(
        played.orders_serving(1, 3),
        search_task_orders(),
        "the orders serving intent 3"
    );
    // No order serves any other intent.
    assert!(played.orders_serving(1, 1).is_empty());
    // The orders are in group number order within a task, and the strike
    // task's come before the search task's.
    let served: Vec<u32> = played
        .decisions(1)
        .orders
        .iter()
        .map(|o| o.intent.raw())
        .collect();
    let first_search = served.iter().position(|i| *i == 3).unwrap();
    assert!(served[..first_search].iter().all(|i| *i == 2));
    assert!(served[first_search..].iter().all(|i| *i == 3));
}

#[test]
fn every_order_serves_its_task_through_a_chain_one_intent_long_ending_with_the_commander() {
    let played = play(&text());
    for (day, order) in played.all_orders() {
        let reason = &order.reason;
        assert_eq!(reason.serves.intent, Some(order.intent), "day {day}");
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
fn the_sizing_order_is_decided_at_the_groups_level_and_the_others_at_the_bases() {
    let played = play(&text());
    let mut sizings = Vec::new();
    for (day, order) in played.all_orders() {
        let (g, what) = air_order(order);
        if what == sizing(true) {
            assert_eq!(level(&order.reason), Some(Addressee::AirGroup(group(g))));
            assert!(
                order
                    .reason
                    .serves
                    .to_string()
                    .contains("decided at the level of air_group"),
                "{}",
                order.reason.serves
            );
            sizings.push((day, g));
        } else {
            assert_eq!(
                level(&order.reason),
                Some(Addressee::Base(base(1))),
                "day {day}"
            );
            assert!(
                order
                    .reason
                    .serves
                    .to_string()
                    .contains("decided at the level of base:1"),
                "{}",
                order.reason.serves
            );
        }
    }
    assert_eq!(sizings, vec![(1, 1), (1, 2), (1, 3)]);
}

#[test]
fn the_morning_of_day_1_sends_10_10_and_4_at_contact_2() {
    let played = play(&text());
    assert_eq!(
        played.launches(1, Pulse::Morning),
        vec![(1, hex(3, 0), 10), (2, hex(3, 0), 10), (3, hex(3, 0), 4)]
    );
}

#[test]
fn the_afternoon_of_day_1_sends_10_and_8_at_contact_3() {
    let played = play(&text());
    assert_eq!(
        played.launches(1, Pulse::Afternoon),
        vec![(1, hex(-3, 0), 10), (2, hex(-3, 0), 8)]
    );
}

#[test]
fn nothing_is_launched_on_day_2_and_five_strikes_of_42_aircraft_in_all() {
    let played = play(&text());
    assert!(played.launches(2, Pulse::Morning).is_empty());
    assert!(played.launches(2, Pulse::Afternoon).is_empty());
    assert!(played.launches(2, Pulse::Night).is_empty());
    assert!(played.launches(1, Pulse::Night).is_empty());
    let all = played.all_launches();
    assert_eq!(all.len(), 5);
    assert_eq!(all.iter().map(|(_, _, a)| *a).sum::<u32>(), 42);
    // Only groups 1, 2 and 3 launch, and no group launches twice in a pulse.
    assert_eq!(played.launched_by(1), 20);
    assert_eq!(played.launched_by(2), 18);
    assert_eq!(played.launched_by(3), 4);
    assert_eq!(played.launched_by(4), 0);
    assert_eq!(played.launched_by(5), 0);
    // Each raid finds nothing: two raids, one at (3,0), one at (-3,0).
    let nothing = played.all(|k| matches!(k, EventKind::StrikeFoundNothing { .. }));
    let mut hexes: Vec<Hex> = nothing
        .iter()
        .map(|e| match &e.kind {
            EventKind::StrikeFoundNothing { hex, .. } => *hex,
            _ => unreachable!(),
        })
        .collect();
    hexes.sort();
    hexes.dedup();
    assert_eq!(hexes, vec![hex(-3, 0), hex(3, 0)]);
    assert!(played
        .all(|k| matches!(k, EventKind::AirAttack { .. }))
        .is_empty());
}

#[test]
fn every_group_has_the_aircraft_it_began_with() {
    let played = play(&text());
    for (raw, ready) in [(1, 10), (2, 10), (3, 10), (4, 6), (5, 8)] {
        assert_eq!(played.ready_at_end(raw), ready, "group {raw}");
    }
}

#[test]
fn group_4_searches_in_every_daylight_pulse_with_all_6_and_sights_nothing() {
    let played = play(&text());
    let flown: Vec<&Event> = played.all(|k| matches!(k, EventKind::SearchFlown { .. }));
    assert_eq!(flown.len(), 4, "{flown:?}");
    let mut pulses = Vec::new();
    for e in &flown {
        let EventKind::SearchFlown {
            air_group,
            from_deg,
            to_deg,
            range_nm,
            aircraft,
            ..
        } = &e.kind
        else {
            unreachable!()
        };
        assert_eq!(*air_group, group(4));
        assert_eq!((*from_deg, *to_deg), (110, 180));
        assert_eq!(*range_nm, n(600));
        assert_eq!(*aircraft, 6);
        pulses.push((e.day, e.pulse));
    }
    assert_eq!(
        pulses,
        vec![
            (1, Some(Pulse::Morning)),
            (1, Some(Pulse::Afternoon)),
            (2, Some(Pulse::Morning)),
            (2, Some(Pulse::Afternoon)),
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
fn group_5_is_on_patrol_with_a_share_of_a_half_so_it_keeps_4_overhead() {
    let played = play(&text());
    assert_eq!(played.mission_at_end(5), CAP);
    let g = &played.truth.last().expect("a state").air_groups[&group(5)];
    assert_eq!(g.patrol_share, fx("0.5"));
    assert_eq!(played.mission_at_end(4), AirMission::Search);
    for raw in 1..=3 {
        assert_eq!(played.mission_at_end(raw), STRIKE, "group {raw}");
    }
}

#[test]
fn no_notice_is_handed_in_and_nothing_is_refused() {
    let played = play(&text());
    played.nothing_refused();
    for day in 1..=2 {
        assert!(played.decisions(day).notices.is_empty(), "day {day}");
    }
}

#[test]
fn the_measure_of_intent_1_at_the_end() {
    let played = play(&text());
    let m = played.measure(1).expect("intent 1 is a deny intent");
    assert_eq!(
        m,
        DenyMeasure {
            days_in_force: 2,
            contacts_held: 0,
            strikes_launched: 5,
            claimed_sunk: 0,
            claimed_hits: 0,
            days_met: 2,
        }
    );
    // A task is not a deny intent.
    assert!(played.measure(2).is_none());
}

// Every staffed scenario built before this one gives the orders it gave. That
// is checked by the compare script (scripts/compare-scenarios.sh), not by a
// test.

// --------------------------------------------------------------- Properties

fn without_risk() -> String {
    changed(
        &text(),
        "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"press\"\n",
        "",
    )
}

#[test]
fn balanced_gives_each_bomber_the_mission_and_sizing_and_launches_nothing() {
    let played = play(&without_risk());
    played.nothing_refused();
    let mut expected = Vec::new();
    for g in 1..=3 {
        expected.push((g, mission(STRIKE)));
        expected.push((g, sizing(true)));
    }
    expected.push((5, mission(CAP)));
    expected.push((5, AirGroupOrder::SetPatrolShare { share: fx("0.5") }));
    assert_eq!(played.orders_serving(1, 2), expected);
    // At balanced group 4 is set to search once a day between its mission
    // and its arc (decision 0131).
    assert_eq!(
        played.orders_serving(1, 3),
        vec![
            (4, mission(AirMission::Search)),
            (4, AirGroupOrder::SetSearchOnce { on: true }),
            (4, arc(110, 180)),
        ]
    );
    // Six for the bombers, three for group 4, two for group 5.
    assert_eq!(played.orders(1).len(), 11);
    assert!(played.orders(2).is_empty());
    assert!(
        played.all_launches().is_empty(),
        "{:?}",
        played.all_launches()
    );
}

#[test]
fn preserve_the_force_holds_a_third_back_and_lists_three_kinds() {
    let text = with(&command_table(
        1,
        "kind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"preserve\"",
    ));
    // The file's own press is replaced: the later limit stands.
    let text = changed(
        &text,
        "kind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"press\"\n",
        "kind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"preserve\"\n",
    );
    let played = play(&text);
    played.nothing_refused();
    let mut expected = Vec::new();
    for g in 1..=3 {
        expected.push((g, mission(STRIKE)));
        expected.push((
            g,
            AirGroupOrder::SetTargetPriority {
                kinds: vec![ShipKind::Carrier, ShipKind::Battleship, ShipKind::Cruiser],
            },
        ));
        expected.push((g, held("0.333333")));
        expected.push((g, sizing(true)));
    }
    expected.push((5, mission(CAP)));
    expected.push((5, AirGroupOrder::SetPatrolShare { share: fx("0.5") }));
    assert_eq!(played.orders_serving(1, 2), expected);
    assert!(
        played.all_launches().is_empty(),
        "{:?}",
        played.all_launches()
    );
}

#[test]
fn the_carrier_needs_everything_so_the_three_groups_send_30_in_one_raid() {
    let text = changed(&text(), "age_pulses = 2", "age_pulses = 1");
    let played = play(&text);
    played.nothing_refused();
    assert_eq!(
        played.launches(1, Pulse::Morning),
        vec![(1, hex(0, 4), 10), (2, hex(0, 4), 10), (3, hex(0, 4), 10)]
    );
    assert!(played
        .launches(1, Pulse::Morning)
        .iter()
        .all(|(_, h, _)| *h != hex(3, 0)));
}

/// The commander states, first, hold base 1 as intent 1 and the deny end as
/// intent 2, with the risk limit on intent 2; the three starting contacts are
/// taken out, so that (0,5) is the only threat.
fn hold_and_deny() -> String {
    let t = text();
    let from = t.find(CONTACTS_MARKER).expect("the contacts");
    let to = t.find(COMMANDS_MARKER).expect("the commands");
    let t = format!("{}{}", &t[..from], &t[to..]);
    let t = changed(
        &t,
        "intent = 1\nintent_kind = \"deny\"",
        "intent = 1\nintent_kind = \"hold\"\nbase = 1\n\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_intent\"\nintent = 2\nintent_kind = \"deny\"",
    );
    changed(
        &t,
        "limit = \"risk\"\nintent = 1",
        "limit = \"risk\"\nintent = 2",
    )
}

#[test]
fn a_base_with_a_cover_has_one_arc() {
    let played = play(&hold_and_deny());
    played.nothing_refused();
    let cover = &played.state.intents[&intent(3)];
    assert_eq!(cover.kind, IntentKind::Cover { base: base(1) });
    assert_eq!(cover.maker, Maker::Staff);
    assert_eq!(cover.parent, Some(intent(1)));
    let task = &played.state.intents[&intent(4)];
    assert_eq!(task.kind, IntentKind::StrikeShips { base: base(1) });
    assert_eq!(task.parent, Some(intent(2)));
    assert_eq!(played.state.intents.len(), 4);
    assert!(!played.has_kind(&IntentKind::SearchWaters { base: base(1) }));
    // The cover has groups 4 and 5; group 4's arc is 110 to 180.
    let covering = played.orders_serving(1, 3);
    let groups: Vec<u32> = covering.iter().map(|(g, _)| *g).collect();
    assert!(groups.contains(&4) && groups.contains(&5), "{covering:?}");
    assert!(groups.iter().all(|g| *g == 4 || *g == 5), "{covering:?}");
    assert!(covering.contains(&(4, arc(110, 180))), "{covering:?}");
    // The strike task orders the three bombers and no one else.
    let striking = played.orders_serving(1, 4);
    assert!(!striking.is_empty());
    assert!(
        striking.iter().all(|(g, _)| (1..=3).contains(g)),
        "{striking:?}"
    );
    played.every_chain_ends_with_the_commander();
}

#[test]
fn waters_280_nm_away_are_in_the_flying_boats_reach_but_not_the_bombers() {
    let text = changed(&text(), "hex = [0, 5]", "hex = [0, 7]");
    let played = play(&text);
    played.nothing_refused();
    assert!(!played.has_kind(&IntentKind::StrikeShips { base: base(1) }));
    assert!(played.has_kind(&IntentKind::SearchWaters { base: base(1) }));
    assert_eq!(played.state.intents.len(), 2);
    // The staff gives no strike order: no group is on the strike mission.
    for (day, order) in played.all_orders() {
        let (g, what) = air_order(order);
        assert_ne!(what, mission(STRIKE), "day {day}");
        assert!(g == 4 || g == 5 || g == 0, "day {day}: group {g}");
    }
    assert!(played.all_launches().is_empty());
    assert!(played
        .decisions(1)
        .notices
        .iter()
        .all(|n| n.kind != NoticeKind::CannotMeet));
}

#[test]
fn waters_640_nm_away_are_out_of_reach_of_both_and_the_staff_cannot_meet_the_intent() {
    let text = changed(&text(), "hex = [0, 5]", "hex = [0, 16]");
    let played = play(&text);
    played.nothing_refused();
    assert_eq!(played.state.intents.len(), 1);
    for day in 1..=2 {
        assert!(played.decisions(day).orders.is_empty(), "day {day}");
        assert!(
            played
                .decisions(day)
                .commands
                .iter()
                .all(|c| !matches!(c.kind, CommandKind::FormIntent { .. })),
            "day {day}"
        );
    }
    assert!(
        played
            .decisions(1)
            .notices
            .iter()
            .any(|n| n.kind == NoticeKind::CannotMeet),
        "no cannot-meet notice on day 1"
    );
    assert!(played.all_launches().is_empty());
}

const SIZING_OFF_ON_GROUP_3: &str =
    "\n[[order]]\nday = 1\nside = \"allied\"\nair_group = 3\nkind = \"set_strike_sizing\"\non = false\n";

/// The property as the code has it: the commander's order of sizing off on
/// group 3 and a hold on the group, which is what makes the staff leave a
/// group alone (the command `hold`, as scenarios 48 and 50 use it).
#[test]
fn a_commanders_order_of_sizing_off_on_a_held_group_3_is_left_alone_and_it_flies_nothing() {
    let text = format!(
        "{}{}",
        with(&command_table(1, "kind = \"hold\"\nair_group = 3")),
        SIZING_OFF_ON_GROUP_3
    );
    let played = play(&text);
    played.nothing_refused();
    // The staff gives group 3 no order, on either day.
    for day in 1..=2 {
        assert!(
            played.orders(day).iter().all(|(g, _)| *g != 3),
            "day {day}: {:?}",
            played.orders(day)
        );
    }
    // It has no mission and flies nothing; its aircraft are all there.
    assert_eq!(played.launched_by(3), 0);
    assert_ne!(played.mission_at_end(3), STRIKE);
    assert_eq!(played.ready_at_end(3), 10);
    let g = &played.truth.last().expect("a state").air_groups[&group(3)];
    assert_eq!(g.mission, AirMission::Rest);
    // The other two bomber groups are given their orders as before.
    for raw in [1, 2] {
        assert_eq!(played.mission_at_end(raw), STRIKE, "group {raw}");
    }
}

/// The order on day 2, when the strike task that may use group 3 is in
/// force: it is taken, it holds the group, and the staff gives group 3 no
/// order on day 2, so its sizing is not set on again.
#[test]
fn a_commanders_order_of_sizing_off_on_day_2_holds_group_3_and_the_staff_leaves_it() {
    let played = play(&with(&SIZING_OFF_ON_GROUP_3.replace("day = 1", "day = 2")));
    played.nothing_refused();
    assert!(
        played.orders(2).iter().all(|(g, _)| *g != 3),
        "{:?}",
        played.orders(2)
    );
    assert!(
        played.state.hold_on_group(group(3)).is_some(),
        "group 3 is held from day 2"
    );
    let g = &played.truth.last().expect("a state").air_groups[&group(3)];
    assert!(!g.strike_sizing, "its sizing stays off");
}

/// The same order on day 1 is taken before the staff forms the strike task
/// and holds nothing: the staff gives group 3 its four orders, sizing on
/// among them, and it sends 4 at contact 2 in the morning, as in the main
/// case.
#[test]
fn a_commanders_order_of_sizing_off_on_day_1_holds_nothing() {
    let played = play(&with(SIZING_OFF_ON_GROUP_3));
    played.nothing_refused();
    let ordered: Vec<AirGroupOrder> = played
        .orders(1)
        .into_iter()
        .filter(|(g, _)| *g == 3)
        .map(|(_, order)| order)
        .collect();
    assert_eq!(ordered.len(), 4, "{ordered:?}");
    assert!(ordered.contains(&AirGroupOrder::SetStrikeSizing { on: true }));
    assert!(played.state.hold_on_group(group(3)).is_none());
    assert!(
        played
            .launches(1, Pulse::Morning)
            .contains(&(3, Hex::new(3, 0), 4)),
        "{:?}",
        played.launches(1, Pulse::Morning)
    );
}

#[test]
fn withdrawing_the_end_on_day_2_withdraws_both_tasks_that_day() {
    let text = with(&command_table(2, "kind = \"withdraw_intent\"\nintent = 1"));
    let played = play(&text);
    played.nothing_refused();
    for raw in [2, 3] {
        assert_eq!(
            played.state.intents[&intent(raw)].withdrawn_day,
            Some(2),
            "intent {raw}"
        );
    }
    let gone: Vec<&Event> = played.all(|k| matches!(k, EventKind::IntentWithdrawn { .. }));
    let tasks: Vec<u32> = gone
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::IntentWithdrawn { intent, .. } if *intent != self::intent(1) => {
                assert_eq!(e.day, 2);
                Some(intent.raw())
            }
            _ => None,
        })
        .collect();
    assert_eq!(tasks.len(), 2, "{tasks:?}");
    assert_eq!(played.orders(1).len(), 16);
    assert!(played.decisions(2).orders.is_empty());
    assert!(played.decisions(2).commands.is_empty());
}

#[test]
fn the_staff_orders_no_group_a_task_may_not_use() {
    // A forces limit on intent 1 that names every group but 3.
    let text = with(&command_table(
        1,
        "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\nair_groups = [1, 2, 4, 5]",
    ));
    let played = play(&text);
    played.nothing_refused();
    for (day, order) in played.all_orders() {
        let (g, _) = air_order(order);
        assert_ne!(g, 3, "group 3 was ordered on day {day}");
    }
    assert_eq!(played.launched_by(3), 0);
    assert_eq!(played.launched_by(1), 20);
    assert_eq!(played.launched_by(2), 18);
}

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
        "{}\n[[base]]\nid = 3\nname = \"Placeholder base 3\"\nside = \"japanese\"\nq = 4\nr = 15\nsupply_t = 50\non_map = false\n\n[[air_group]]\nid = 6\nname = \"Placeholder Japanese fighters 6\"\nside = \"japanese\"\nbase = 3\naircraft_type = 4\nready = 20\n",
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
