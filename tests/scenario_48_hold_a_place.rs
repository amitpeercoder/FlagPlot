//! Scenario 48: the commander states one end, "hold base 1", and the staff
//! forms cover from the air under it and gives the orders of scenario 47.
//!
//! Proves decision 0117, the intent tree in the command state and the end
//! "hold a place", against a hand calculation. It stands on scenario 47's
//! hand calculation for the cover doctrine (decision 0115) and repeats none
//! of its arithmetic. The data is placeholder data: see
//! `data/scenarios/48_hold_a_place.toml`. It runs on the frozen step 1
//! figures, with the fatigue figures loaded and the wear, upkeep and supply
//! figures not. No die is rolled.
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
//! Scenario 47's, unchanged: the map, Allied base 1 at (0,0), Japanese base
//! 2 at (5,0) with nothing at it, the four groups at base 1 (fighters 1 and
//! 2, flying boats 3, dive bombers 4), the one contact at (0,5), 12 days.
//!
//! The Allied side's commands of day 1 are different: intent 1, hold base
//! 1; leave to prepare and leave to launch both given. No forces limit and
//! no risk are set, on anything.
//!
//! ## The tree on the first morning
//!
//! Plan 1 is drafted for intent 1 when it is set. The staff makes it ready
//! and releases it, by the two leaves. Intent 1 has no forces limit and
//! nothing above it, so it may use what its addressee commands: the air
//! groups at base 1, which are groups 1, 2, 3 and 4. Groups 1 and 2 are
//! fighters and group 3 can search, and no cover intent for base 1 is in
//! force, so the staff forms one.
//!
//! | Intent | Kind | Parent | Maker | Addressee | Plan | Set |
//! | ------ | ---- | ------ | ----- | --------- | ---- | --- |
//! | 1      | hold base 1 | none | the commander | base 1 | 1 | day 1 |
//! | 2      | cover base 1 from the air | 1 | the staff | base 1 | 1 | day 1 |
//!
//! Intent 2 takes the number 2: one above the highest the side has used.
//! There is one plan. The reason handed in with the forming serves intent
//! 1, and its chain is empty: intent 1 is the commander's own.
//!
//! Intent 2 has no limit of its own. Read up the tree, its forces are the
//! four groups and its risk is balanced: the share is 0.5 and a group rests
//! when its fatigue in the morning is above 45.
//!
//! ## Day by day
//!
//! The cover doctrine runs for intent 2 on the morning it is formed. From
//! there every order, every group's fatigue on every morning and at the end
//! of day 12, and the search arc of every day are those of scenario 47's
//! table. By day the staff gives 7, 0, 1, 1, 0, 0, 1, 1, 1, 3, 2 and 1
//! orders: 18 in all. (Scenario 47's table as the architect restated it on
//! Nov 16, 2026, for decision 0131; it was 6, 1, 2, 1, 0, 0, 0, 0, 1, 3, 2
//! and 1, 17 in all. By his ruling of the same day this scenario follows
//! it.)
//!
//! Every one of the 18, and the reason of every morning without an order,
//! serves intent 2 and plan 1, and its chain is one intent long: intent 1,
//! made by the commander, with its statement in words. No staff order and
//! no staff command is refused.
//!
//! ## The measure
//!
//! At the end of day 12 the measure of intent 1 reads: 12 days in force; 12
//! days on which base 1 was Allied and its airfield open; no air raid on
//! it and none fought. It was met on every day. Its days of supply are not
//! asserted: the supply figures are not loaded.
//!
//! # Properties
//!
//! - **A forces limit is read up the tree.** With a forces limit on intent
//!   1, set on day 1, that names groups 1 and 3: group 2 is given no order
//!   on any day and ends at fatigue 0. Group 1 is the only fighter the
//!   staff may order: it is given the patrol and the share on day 1 and no
//!   order after, it is kept on patrol on day 9 at 48 and on every later
//!   day, and it ends day 12 at 72. Group 3's days are those of scenario
//!   47's table. The staff gives 5, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1 and 0: 11
//!   orders (4, 1, 2, 1, then 1 on days 10 and 11, 10 in all, before
//!   decision 0131).
//! - **A risk level is read up the tree.** With the risk of intent 1 set to
//!   preserve the force on day 1, the share ordered on day 1 is 0.75, and
//!   both fighters first pass the level of 30 on the morning of day 7, at
//!   36, when group 1 rests and group 2 is kept.
//! - **A limit on a sub-intent may narrow.** In the main case, with the
//!   commander setting the risk of intent 2 to preserve the force on day
//!   2: it is taken. That morning the staff gives two orders: a share of
//!   0.75 to group 1 and to group 2. Group 3 is at 22, not above 30, and
//!   keeps its search, with search once a day as preserve has it too.
//!   Group 1 rests on day 7, as above. (Restated by the architect on Nov
//!   16, 2026, for decision 0131: it had three orders, with rest to group
//!   3 at 54.)
//! - **And may not widen.** In the main case, with the commander setting
//!   the risk of intent 2 to press on day 2: it is refused, as bolder than
//!   its parent's, and the orders are those of the main case. A forces
//!   limit on intent 2 that names a group intent 1 may not use is refused
//!   the same way: in the first property's variant, one naming group 2.
//! - **A staff-made intent is the staff's.** The commander's `set_intent`
//!   and `withdraw_intent` on intent 2 are each refused and change nothing.
//! - **Withdrawing a tree.** With intent 1 withdrawn on day 5: intent 2 is
//!   withdrawn on day 5 by an event caused by intent 1's, plan 1 ends
//!   complete, and the staff gives no order and no reason from day 5. The
//!   groups keep the missions they had: groups 1 and 2 the patrol, group 3
//!   rest.
//! - **Nothing before release.** With leave to launch not given: on day 1
//!   the staff makes plan 1 ready, hands in that it waits for leave to
//!   launch, forms no intent and gives no order, and none on day 2. With
//!   the commander approving plan 1 on day 3, the staff forms intent 2 that
//!   morning and gives seven orders: the patrol and a share of 0.5 to
//!   groups 1 and 2, and to group 3 the search mission, search once a day
//!   (decision 0131) and the arc 050 to 120 at 600 nm, the contact being
//!   gone.
//! - **A cover intent that stands.** With the commander also stating
//!   intent 2, cover base 1, on day 1: the staff forms nothing under intent
//!   1, and its reason names intent 2. The orders are scenario 47's, each
//!   serving intent 2 with an empty chain.
//! - **Nothing to cover with.** With a forces limit on intent 1 that names
//!   group 4 alone: the staff forms nothing, on every day, gives no order,
//!   and its reason says there is nothing at the base to cover it with.
//! - **The top of every chain is the commander's.** In every variant,
//!   every reason's chain, followed to its end, reaches an intent the
//!   commander made, and `why` on any staff order prints it.
//! - **Stage 1 is unchanged.** Every staffed scenario of stage 1 gives the
//!   orders it gave. Its intents are roots, made by the commander and
//!   addressed to the area.
//! - Two worlds that differ only in what the Allied side cannot see give
//!   the same intents, orders and reasons, and the same start gives the
//!   same.

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_agent::{hold_measure, HoldMeasure};
use sim_core::command::{
    Addressee, CommandKind, CommandRefusal, CommandState, IntentKind, Leave, Maker, NoticeKind,
    PlanId, PlanState, Reason, StaffDecisions, StaffOrder,
};
use sim_core::events::{Cause, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::ids::{AirGroupId, BaseId, IntentId};
use sim_core::orders::{AirGroupOrder, AirMission, OrderKind};
use sim_core::state::{Side, TruthState};
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

mod common;

const COMMANDS_MARKER: &str = "# The Allied commander's commands";

fn path() -> PathBuf {
    common::data_dir().join("scenarios/48_hold_a_place.toml")
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

fn plan(raw: u32) -> PlanId {
    PlanId::new(raw)
}

/// The commands of the file itself: the marker line and what follows it.
fn commands() -> String {
    let text = text();
    let at = text.find(COMMANDS_MARKER).expect("the marker");
    text[at..].to_string()
}

/// A command table of the Allied side as the scenario states it.
fn command_table(day: u32, body: &str) -> String {
    format!("\n[[command]]\nday = {day}\nside = \"allied\"\n{body}\n")
}

/// The scenario's text with `extra` after its commands.
fn with(extra: &str) -> String {
    format!("{}{}", text(), extra)
}

fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

/// A game played to its end with the doctrine's staffs: what the Allied staff
/// handed in each day, the truth at the end of each day (index 0 is the
/// start), the log, the side's command state at the end and the measure of
/// intent 1 at the end (if it is a hold intent).
struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    truth: Vec<TruthState>,
    events: Vec<Event>,
    state: CommandState,
    measure: Option<HoldMeasure>,
}

fn play(text: &str) -> Played {
    let mut scenario: Scenario = common::from_text(text, &path()).expect("the scenario loads");
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
    let measure = hold_measure(&scenario.world.side_view(Side::Allied), intent(1));
    Played {
        handed,
        truth,
        events: scenario.world.events().iter().cloned().collect(),
        state: scenario.world.command_state(Side::Allied).clone(),
        measure,
    }
}

impl Played {
    /// A group's fatigue on the morning of `day`, before the day's orders.
    fn morning(&self, day: u32, raw: u32) -> Fixed {
        self.truth[(day - 1) as usize].air_groups[&group(raw)].fatigue
    }

    fn at_end(&self, raw: u32) -> Fixed {
        self.truth.last().expect("a state").air_groups[&group(raw)].fatigue
    }

    fn mission_at_end(&self, raw: u32) -> AirMission {
        self.truth.last().expect("a state").air_groups[&group(raw)].mission
    }

    fn decisions(&self, day: u32) -> &StaffDecisions {
        &self.handed[&day]
    }

    /// The staff's air group orders of `day`, as (group, order), in the order
    /// handed in.
    fn air_orders(&self, day: u32) -> Vec<(u32, AirGroupOrder)> {
        self.decisions(day)
            .orders
            .iter()
            .map(|o| match &o.kind {
                OrderKind::AirGroup { target, order } => (target.raw(), order.clone()),
                other => panic!("the air doctrine gave a non-air order: {other:?}"),
            })
            .collect()
    }

    fn counts(&self) -> Vec<usize> {
        (1..=12).map(|d| self.decisions(d).orders.len()).collect()
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

    /// The commands the commander had refused, with the day and the reason.
    fn refusals(&self) -> Vec<(u32, CommandRefusal)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::CommandRefused { reason, .. } => Some((e.day, *reason)),
                _ => None,
            })
            .collect()
    }

    /// No staff order was refused and no staff command was refused.
    fn nothing_refused(&self) {
        let refused = self.all(|k| {
            matches!(
                k,
                EventKind::OrderRejected { .. } | EventKind::CommandRefused { .. }
            )
        });
        assert!(refused.is_empty(), "something was refused: {refused:?}");
    }

    /// Every reason the staff handed in, with the day.
    fn reasons(&self) -> Vec<(u32, &Reason)> {
        let mut found = Vec::new();
        for (day, d) in &self.handed {
            found.extend(d.orders.iter().map(|o| (*day, &o.reason)));
            found.extend(d.commands.iter().map(|c| (*day, &c.reason)));
            found.extend(d.no_orders.iter().map(|r| (*day, r)));
        }
        found
    }

    /// Every reason's chain, followed to its end, reaches an intent the
    /// commander made.
    fn every_chain_ends_with_the_commander(&self) {
        for (day, reason) in self.reasons() {
            let served = reason.serves.intent.expect("a reason serves an intent");
            match reason.serves.chain.last() {
                Some(link) => {
                    assert_eq!(link.maker, Maker::Commander, "day {day}: {reason:?}");
                    assert!(!link.statement.is_empty(), "day {day}");
                    // The chain is nearest first: its first link is the
                    // parent of the intent served.
                    assert_eq!(
                        Some(reason.serves.chain[0].intent),
                        self.state.intents[&served].parent,
                        "day {day}"
                    );
                    assert_eq!(
                        Some(link.intent),
                        Some(self.state.root_of(served)),
                        "day {day}"
                    );
                }
                None => assert_eq!(
                    self.state.intents[&served].maker,
                    Maker::Commander,
                    "day {day}: a reason with no chain serves a staff-made intent"
                ),
            }
        }
    }
}

fn mission(m: AirMission) -> AirGroupOrder {
    AirGroupOrder::SetAirMission(m)
}

fn share(s: &str) -> AirGroupOrder {
    AirGroupOrder::SetPatrolShare { share: fx(s) }
}

/// Search once a day, on (decision 0131).
fn once() -> AirGroupOrder {
    AirGroupOrder::SetSearchOnce { on: true }
}

fn arc(from: i32, to: i32) -> AirGroupOrder {
    AirGroupOrder::SetSearchArc {
        from_deg: from,
        to_deg: to,
        range_nm: n(600),
    }
}

const CAP: AirMission = AirMission::CombatAirPatrol;
const REST: AirMission = AirMission::Rest;
const SEARCH: AirMission = AirMission::Search;

/// The staff's orders of scenario 47's table, by day.
fn table_orders() -> Vec<Vec<(u32, AirGroupOrder)>> {
    vec![
        vec![
            (1, mission(CAP)),
            (1, share("0.5")),
            (2, mission(CAP)),
            (2, share("0.5")),
            (3, mission(SEARCH)),
            (3, once()),
            (3, arc(60, 180)),
        ],
        vec![],
        vec![(3, arc(50, 120))],
        vec![(3, mission(REST))],
        vec![],
        vec![],
        vec![(3, mission(SEARCH))],
        vec![(3, mission(REST))],
        vec![(1, mission(REST))],
        vec![(1, mission(CAP)), (2, mission(REST)), (3, mission(SEARCH))],
        vec![(2, mission(CAP)), (3, mission(REST))],
        vec![(1, mission(REST))],
    ]
}

/// Fatigue on the morning of each day: (group 1, group 2, group 3).
const MORNINGS: [(i64, i64, i64); 12] = [
    (0, 0, 0),
    (6, 6, 22),
    (12, 12, 44),
    (18, 18, 66),
    (24, 24, 56),
    (30, 30, 46),
    (36, 36, 36),
    (42, 42, 58),
    (48, 48, 48),
    (38, 54, 38),
    (44, 44, 60),
    (50, 50, 50),
];

fn mentions(reason: &Reason, words: &[&str]) {
    for word in words {
        assert!(
            reason.worked.contains(word),
            "the reason's worked part has no \"{word}\": {}",
            reason.worked
        );
    }
}

/// A reason of the cover doctrine: it serves intent 2 and plan 1 and its
/// chain is one intent long, intent 1, the commander's, with its statement in
/// words.
fn serves_cover_under_the_hold(reason: &Reason) {
    assert_eq!(reason.serves.intent, Some(intent(2)));
    assert_eq!(reason.serves.plan, Some(plan(1)));
    assert_eq!(reason.serves.amendment, None);
    assert!(!reason.serves.statement.is_empty());
    assert_eq!(reason.serves.chain.len(), 1, "{:?}", reason.serves);
    let link = &reason.serves.chain[0];
    assert_eq!(link.intent, intent(1));
    assert_eq!(link.maker, Maker::Commander);
    assert!(
        link.statement.contains("hold") && link.statement.contains("base:1"),
        "{}",
        link.statement
    );
    assert!(!reason.worked.is_empty());
}

/// A reason of the hold doctrine: it serves intent 1 and plan 1, which is the
/// commander's own, with no chain.
fn serves_the_hold(reason: &Reason) {
    assert_eq!(reason.serves.intent, Some(intent(1)));
    assert_eq!(reason.serves.plan, Some(plan(1)));
    assert!(reason.serves.chain.is_empty());
    assert!(!reason.serves.statement.is_empty());
}

// ------------------------------------------------------- The hand calculation

#[test]
fn the_commands_of_the_file_are_the_ones_these_tests_vary() {
    let c = commands();
    assert_eq!(c.matches("[[command]]").count(), 3);
    assert!(c.contains("intent_kind = \"hold\""));
    assert!(!c.contains("set_limit"));
}

#[test]
fn the_tree_on_the_first_morning() {
    let played = play(&text());
    let state = &played.state;
    assert_eq!(state.intents.len(), 2);
    assert_eq!(state.plans.len(), 1);
    let one = &state.intents[&intent(1)];
    assert_eq!(
        one.kind,
        IntentKind::Hold {
            base: BaseId::new(1)
        }
    );
    assert_eq!(one.parent, None);
    assert_eq!(one.maker, Maker::Commander);
    assert_eq!(one.addressee, Addressee::Base(BaseId::new(1)));
    assert_eq!(one.plan, plan(1));
    assert_eq!(one.set_day, 1);
    // Intent 2 takes the number one above the highest the side has used.
    let two = &state.intents[&intent(2)];
    assert_eq!(
        two.kind,
        IntentKind::Cover {
            base: BaseId::new(1)
        }
    );
    assert_eq!(two.parent, Some(intent(1)));
    assert_eq!(two.maker, Maker::Staff);
    assert_eq!(two.addressee, Addressee::Base(BaseId::new(1)));
    assert_eq!(two.plan, plan(1));
    assert_eq!(two.set_day, 1);
    assert_eq!(two.withdrawn_day, None);
    assert!(one.limits.forces.is_none() && two.limits.forces.is_none());
    // Plan 1 is released by the two leaves on the first morning.
    assert_eq!(state.plans[&plan(1)].state, PlanState::Released);
}

#[test]
fn on_the_first_morning_the_staff_makes_the_plan_ready_releases_it_and_forms_intent_2() {
    let played = play(&text());
    let commands = &played.decisions(1).commands;
    assert_eq!(commands.len(), 3);
    assert_eq!(commands[0].kind, CommandKind::Approve { plan: plan(1) });
    assert_eq!(commands[1].kind, CommandKind::Approve { plan: plan(1) });
    assert_eq!(
        commands[2].kind,
        CommandKind::FormIntent {
            parent: intent(1),
            kind: IntentKind::Cover {
                base: BaseId::new(1)
            },
        }
    );
    // The reason handed in with the forming serves intent 1, and its chain
    // is empty: intent 1 is the commander's own.
    serves_the_hold(&commands[2].reason);
    for day in 2..=12 {
        assert!(played.decisions(day).commands.is_empty(), "day {day}");
    }
    let formed = played.all(|k| matches!(k, EventKind::IntentFormed { .. }));
    assert_eq!(formed.len(), 1);
    assert_eq!(formed[0].day, 1);
    assert!(matches!(
        formed[0].kind,
        EventKind::IntentFormed { intent: i, parent: p, .. } if i == intent(2) && p == intent(1)
    ));
}

#[test]
fn the_staffs_orders_of_every_morning_are_those_of_scenario_47() {
    let played = play(&text());
    for (i, expected) in table_orders().iter().enumerate() {
        let day = i as u32 + 1;
        assert_eq!(&played.air_orders(day), expected, "the orders of day {day}");
    }
}

#[test]
fn the_staff_gives_seven_none_one_one_none_none_one_one_one_three_two_one_orders() {
    let played = play(&text());
    assert_eq!(played.counts(), vec![7, 0, 1, 1, 0, 0, 1, 1, 1, 3, 2, 1]);
    assert_eq!(played.counts().iter().sum::<usize>(), 18);
}

#[test]
fn every_groups_fatigue_on_every_morning_and_at_the_end_is_that_of_scenario_47() {
    let played = play(&text());
    for (i, (g1, g2, g3)) in MORNINGS.iter().enumerate() {
        let day = i as u32 + 1;
        assert_eq!(played.morning(day, 1), n(*g1), "group 1, day {day}");
        assert_eq!(played.morning(day, 2), n(*g2), "group 2, day {day}");
        assert_eq!(played.morning(day, 3), n(*g3), "group 3, day {day}");
        assert_eq!(played.morning(day, 4), n(0), "group 4, day {day}");
    }
    assert_eq!(played.at_end(1), n(40));
    assert_eq!(played.at_end(2), n(56));
    assert_eq!(played.at_end(3), n(40));
    assert_eq!(played.at_end(4), n(0));
}

#[test]
fn every_staff_order_serves_intent_2_and_plan_1_through_a_chain_one_intent_long() {
    let played = play(&text());
    for (day, order) in played.all_orders() {
        assert_eq!(order.intent, intent(2), "day {day}");
        serves_cover_under_the_hold(&order.reason);
    }
}

// READING OF THE HAND CALCULATION. "The reason of every morning without an
// order serves intent 2 and plan 1" is read as the reason of the cover
// doctrine for the groups it gives no order (the reason that names group 4,
// left for a strike intent): there is one on every morning, and it serves
// intent 2 with a chain of intent 1. The hold doctrine's own daily reason from
// day 2, that intent 2 stands so that no other is formed, is a reason of
// intent 1's (the intent it is made for, the commander's own, with no chain);
// the hand calculation does not count it among them. This is the reading
// closest to the record of decision 0117, 10: a reason serves the intent whose
// doctrine made it.
#[test]
fn the_reason_of_every_morning_without_an_order_serves_intent_2_and_plan_1() {
    let played = play(&text());
    for day in 1..=12 {
        let d = played.decisions(day);
        let cover: Vec<&Reason> = d
            .no_orders
            .iter()
            .filter(|r| r.serves.intent == Some(intent(2)))
            .collect();
        assert_eq!(cover.len(), 1, "day {day}");
        serves_cover_under_the_hold(cover[0]);
        mentions(
            cover[0],
            &[
                "air_group:4",
                "left for a strike intent",
                "The share is 0.5",
            ],
        );
        assert!(d.notices.is_empty(), "day {day}");
        // The others are the hold doctrine's: one a day from day 2.
        let hold: Vec<&Reason> = d
            .no_orders
            .iter()
            .filter(|r| r.serves.intent != Some(intent(2)))
            .collect();
        assert_eq!(hold.len(), if day == 1 { 0 } else { 1 }, "day {day}");
        for r in hold {
            serves_the_hold(r);
            mentions(r, &["intent:2", "the staff forms no other"]);
        }
    }
}

#[test]
fn the_reasons_name_the_fatigue_against_the_level_the_share_the_threats_and_the_arc() {
    let played = play(&text());
    for (day, order) in played.all_orders() {
        let OrderKind::AirGroup { target, order: o } = &order.kind else {
            unreachable!()
        };
        let raw = target.raw();
        let m = MORNINGS[(day - 1) as usize];
        let fatigue = match raw {
            1 => m.0,
            2 => m.1,
            _ => m.2,
        };
        let w = &order.reason.worked;
        match o {
            AirGroupOrder::SetAirMission(_) => assert!(
                w.contains(&format!(
                    "air_group:{raw}, a {}, has fatigue {fatigue} against the 45 of the balanced risk",
                    if raw == 3 { "search group" } else { "fighter group" }
                )),
                "day {day}: {w}"
            ),
            AirGroupOrder::SetPatrolShare { .. } => {
                assert!(w.contains("0.5 at balanced"), "day {day}: {w}")
            }
            AirGroupOrder::SetSearchOnce { on } => {
                assert!(on, "day {day}");
                assert!(
                    w.contains(&format!(
                        "air_group:3, a search group, has fatigue {fatigue} against the 45 of the balanced risk"
                    )),
                    "day {day}: {w}"
                );
            }
            AirGroupOrder::SetSearchArc { .. } => {}
            other => panic!("{other:?}"),
        }
    }
    let day1 = &played.decisions(1).orders[6].reason.worked;
    for word in [
        "060 to 180 out to 600 nm",
        "base:2 (Placeholder base 2) at (5,0)",
        "contact 1 at (0,5)",
        "the narrowest arc that holds them is 090 to 150",
    ] {
        assert!(day1.contains(word), "day 1: no \"{word}\": {day1}");
    }
    let day3 = &played.decisions(3).orders[0].reason.worked;
    for word in [
        "050 to 120 out to 600 nm",
        "the narrowest arc that holds them is 080 to 090",
        "contact 1 (dropped on day 2) held when its arc was set is gone",
    ] {
        assert!(day3.contains(word), "day 3: no \"{word}\": {day3}");
    }
    let day9 = &played
        .decisions(9)
        .no_orders
        .iter()
        .find(|r| r.serves.intent == Some(intent(2)))
        .expect("the cover reason")
        .worked;
    assert!(day9.contains("air_group:2, a fighter group, has fatigue 48 against the 45"));
    assert!(day9.contains("so it is never rested"));
}

#[test]
fn no_staff_order_and_no_staff_command_is_refused() {
    let played = play(&text());
    played.nothing_refused();
    assert!(played.refusals().is_empty());
}

#[test]
fn the_measure_of_intent_1_at_the_end_of_day_12() {
    let played = play(&text());
    let m = played.measure.expect("intent 1 is a hold intent");
    assert_eq!(m.days_in_force, 12);
    assert_eq!(m.days_met, 12);
    assert_eq!(m.raids, 0);
    assert_eq!(m.raids_fought, 0);
    // Its days of supply are not asserted: the supply figures are not loaded.
}

#[test]
fn the_top_of_every_chain_is_the_commanders_and_why_prints_it() {
    let played = play(&text());
    played.every_chain_ends_with_the_commander();

    let mut scenario: Scenario = common::from_text(&text(), &path()).expect("loads");
    let held = staffs();
    scenario
        .next_day_watched(as_staffs(&held), &mut |_, _| {})
        .expect("day 1");
    let view = scenario.world.side_view(Side::Allied);
    let events = view.events();
    // Every staff reason with an order of day 1 prints its chain.
    let mut seen = 0;
    for (i, e) in events.iter().enumerate() {
        if matches!(
            &e.kind,
            sim_core::side_events::SideEventKind::Whole(EventKind::StaffReason {
                order: Some(_),
                ..
            })
        ) {
            let said = sim_agent::why(&view, i as u32 + 1).to_string();
            assert!(said.contains("under intent 1 (the commander's)"), "{said}");
            assert!(said.contains("hold"), "{said}");
            seen += 1;
        }
    }
    assert_eq!(seen, 7);
}

// Stage 1 is unchanged: every staffed scenario of stage 1 gives the orders it
// gave, and its intents are roots, made by the commander and addressed to the
// area. That is checked by the compare script (scripts/compare.sh), not by a
// test.

// --------------------------------------------------------------- Properties

/// A forces limit on intent 1 that names groups 1 and 3.
fn limited() -> String {
    with(&command_table(
        1,
        "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\nair_groups = [1, 3]",
    ))
}

#[test]
fn a_forces_limit_is_read_up_the_tree() {
    let played = play(&limited());
    played.nothing_refused();
    for (day, order) in played.all_orders() {
        let OrderKind::AirGroup { target, .. } = &order.kind else {
            unreachable!()
        };
        assert_ne!(*target, group(2), "group 2 was ordered on day {day}");
        assert_ne!(*target, group(4), "group 4 was ordered on day {day}");
    }
    assert_eq!(played.at_end(2), n(0));
    // Group 1, the only fighter, is given the patrol and the share on day 1
    // and no order after.
    assert_eq!(
        played.air_orders(1),
        vec![
            (1, mission(CAP)),
            (1, share("0.5")),
            (3, mission(SEARCH)),
            (3, once()),
            (3, arc(60, 180)),
        ]
    );
    for day in 2..=12 {
        assert!(
            played.air_orders(day).iter().all(|(g, _)| *g != 1),
            "group 1 was ordered on day {day}"
        );
    }
    // It is kept on patrol on day 9, at 48, and on every later day.
    assert_eq!(played.morning(9, 1), n(48));
    mentions(
        played
            .decisions(9)
            .no_orders
            .iter()
            .find(|r| r.serves.intent == Some(intent(2)))
            .expect("the cover reason"),
        &[
            "air_group:1, a fighter group, has fatigue 48 against the 45",
            "so it is never rested",
        ],
    );
    assert_eq!(played.mission_at_end(1), CAP);
    assert_eq!(played.at_end(1), n(72));
    // Group 3's days are those of scenario 47 and the counts are 5, 0, 1, 1,
    // 0, 0, 1, 1, 0, 1, 1, 0: 11 orders.
    for day in 2..=12 {
        let expected: Vec<(u32, AirGroupOrder)> = table_orders()[(day - 1) as usize]
            .iter()
            .filter(|(g, _)| *g == 3)
            .cloned()
            .collect();
        assert_eq!(played.air_orders(day), expected, "day {day}");
    }
    assert_eq!(played.counts(), vec![5, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0]);
    assert_eq!(played.counts().iter().sum::<usize>(), 11);
    played.every_chain_ends_with_the_commander();
}

#[test]
fn a_risk_level_is_read_up_the_tree() {
    let text = with(&command_table(
        1,
        "kind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"preserve\"",
    ));
    let played = play(&text);
    played.nothing_refused();
    assert_eq!(played.air_orders(1)[1], (1, share("0.75")));
    assert_eq!(played.air_orders(1)[3], (2, share("0.75")));
    assert_eq!(played.morning(6, 1), n(30));
    assert_eq!(played.morning(7, 1), n(36));
    assert_eq!(played.morning(7, 2), n(36));
    for day in 1..=6 {
        assert!(
            played
                .air_orders(day)
                .iter()
                .all(|(g, o)| !(*g <= 2 && *o == mission(REST))),
            "a fighter rested on day {day}"
        );
    }
    // Group 1 rests and group 2 is kept.
    assert!(played.air_orders(7).contains(&(1, mission(REST))));
    assert!(!played.air_orders(7).contains(&(2, mission(REST))));
    played.every_chain_ends_with_the_commander();
}

#[test]
fn a_limit_on_a_sub_intent_may_narrow() {
    let text = with(&command_table(
        2,
        "kind = \"set_limit\"\nlimit = \"risk\"\nintent = 2\nrisk = \"preserve\"",
    ));
    let played = play(&text);
    played.nothing_refused();
    assert_eq!(played.air_orders(1), table_orders()[0]);
    assert_eq!(
        played.air_orders(2),
        vec![(1, share("0.75")), (2, share("0.75"))]
    );
    assert!(played.air_orders(7).contains(&(1, mission(REST))));
    let two = &played.state.intents[&intent(2)];
    assert_eq!(two.limits.risk, Some(sim_core::command::Risk::Preserve));
    played.every_chain_ends_with_the_commander();
}

#[test]
fn a_limit_on_a_sub_intent_may_not_widen_a_risk() {
    let text = with(&command_table(
        2,
        "kind = \"set_limit\"\nlimit = \"risk\"\nintent = 2\nrisk = \"press\"",
    ));
    let played = play(&text);
    assert_eq!(
        played.refusals(),
        vec![(2, CommandRefusal::WiderThanParent(intent(1)))]
    );
    for (i, expected) in table_orders().iter().enumerate() {
        let day = i as u32 + 1;
        assert_eq!(&played.air_orders(day), expected, "day {day}");
    }
    assert_eq!(played.state.intents[&intent(2)].limits.risk, None);
}

#[test]
fn a_forces_limit_on_a_sub_intent_may_not_name_a_group_the_parent_may_not_use() {
    let text = format!(
        "{}{}",
        limited(),
        command_table(
            2,
            "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 2\nair_groups = [2]"
        )
    );
    let played = play(&text);
    assert_eq!(
        played.refusals(),
        vec![(2, CommandRefusal::WiderThanParent(intent(1)))]
    );
    assert_eq!(played.at_end(2), n(0));
    assert_eq!(played.counts().iter().sum::<usize>(), 11);
}

#[test]
fn a_staff_made_intent_is_the_staffs() {
    let text = format!(
        "{}{}{}",
        text(),
        command_table(
            2,
            "kind = \"set_intent\"\nintent = 2\nintent_kind = \"cover\"\nbase = 1"
        ),
        command_table(3, "kind = \"withdraw_intent\"\nintent = 2")
    );
    let played = play(&text);
    assert_eq!(
        played.refusals(),
        vec![
            (2, CommandRefusal::MadeByStaff(intent(2))),
            (3, CommandRefusal::MadeByStaff(intent(2))),
        ]
    );
    // Nothing changed.
    let two = &played.state.intents[&intent(2)];
    assert_eq!(two.withdrawn_day, None);
    assert_eq!(two.maker, Maker::Staff);
    assert_eq!(two.parent, Some(intent(1)));
    for (i, expected) in table_orders().iter().enumerate() {
        let day = i as u32 + 1;
        assert_eq!(&played.air_orders(day), expected, "day {day}");
    }
}

#[test]
fn withdrawing_a_tree_withdraws_what_the_staff_formed_under_it() {
    let text = with(&command_table(5, "kind = \"withdraw_intent\"\nintent = 1"));
    let played = play(&text);
    played.nothing_refused();
    // Intent 2 is withdrawn on day 5 by an event caused by intent 1's
    // withdrawal.
    let gone = played.all(|k| matches!(k, EventKind::IntentWithdrawn { .. }));
    assert_eq!(gone.len(), 1);
    assert_eq!(gone[0].day, 5);
    assert!(matches!(
        gone[0].kind,
        EventKind::IntentWithdrawn { intent: i, parent: p, .. } if i == intent(2) && p == intent(1)
    ));
    let taken = played
        .events
        .iter()
        .find(|e| {
            matches!(
                &e.kind,
                EventKind::CommandTaken {
                    command: CommandKind::WithdrawIntent { intent: i },
                    ..
                } if *i == intent(1)
            )
        })
        .expect("the withdrawal was taken");
    assert!(
        gone[0].causes.contains(&Cause::Event(taken.id)),
        "{:?} against {:?}",
        gone[0].causes,
        taken.id
    );
    assert_eq!(played.state.intents[&intent(2)].withdrawn_day, Some(5));
    assert_eq!(played.state.intents[&intent(1)].withdrawn_day, Some(5));
    assert_eq!(played.state.plans[&plan(1)].state, PlanState::Complete);
    // The staff gives no order, no command and no reason from day 5.
    for day in 1..=4 {
        assert_eq!(played.air_orders(day), table_orders()[(day - 1) as usize]);
    }
    for day in 5..=12 {
        let d = played.decisions(day);
        assert!(d.orders.is_empty(), "day {day}");
        assert!(d.commands.is_empty(), "day {day}");
        assert!(d.no_orders.is_empty(), "day {day}");
        assert!(d.notices.is_empty(), "day {day}");
    }
    // The groups keep the missions they had.
    assert_eq!(played.mission_at_end(1), CAP);
    assert_eq!(played.mission_at_end(2), CAP);
    assert_eq!(played.mission_at_end(3), REST);
}

fn without_leave_to_launch() -> String {
    changed(
        &text(),
        "leave = \"launch\"\ngiven = true",
        "leave = \"launch\"\ngiven = false",
    )
}

#[test]
fn nothing_is_formed_or_ordered_before_the_plan_is_released() {
    let played = play(&without_leave_to_launch());
    played.nothing_refused();
    // Day 1: plan 1 made ready, the staff says it waits for leave to launch,
    // forms no intent and gives no order.
    let d1 = played.decisions(1);
    assert_eq!(d1.commands.len(), 1);
    assert_eq!(d1.commands[0].kind, CommandKind::Approve { plan: plan(1) });
    assert!(d1.orders.is_empty());
    assert!(d1
        .notices
        .iter()
        .any(|n| n.kind == NoticeKind::WaitsForLeave(Leave::Launch)));
    assert!(d1
        .no_orders
        .iter()
        .any(|r| r.worked.contains("leave to launch")));
    for r in &d1.no_orders {
        serves_the_hold(r);
    }
    // Day 2: none.
    assert!(played.decisions(2).orders.is_empty());
    assert!(played
        .decisions(2)
        .commands
        .iter()
        .all(|c| !matches!(c.kind, CommandKind::FormIntent { .. })));
    assert_eq!(played.air_orders(2), vec![]);
}

#[test]
fn the_commander_approving_the_plan_on_day_3_releases_it_and_the_staff_forms_cover() {
    let text = format!(
        "{}{}",
        without_leave_to_launch(),
        command_table(3, "kind = \"approve\"\nplan = 1")
    );
    let played = play(&text);
    played.nothing_refused();
    assert!(played.air_orders(1).is_empty());
    assert!(played.air_orders(2).is_empty());
    // That morning the staff forms intent 2 and gives seven orders.
    let commands = &played.decisions(3).commands;
    assert_eq!(commands.len(), 1);
    assert_eq!(
        commands[0].kind,
        CommandKind::FormIntent {
            parent: intent(1),
            kind: IntentKind::Cover {
                base: BaseId::new(1)
            },
        }
    );
    assert_eq!(played.state.intents[&intent(2)].set_day, 3);
    assert_eq!(
        played.air_orders(3),
        vec![
            (1, mission(CAP)),
            (1, share("0.5")),
            (2, mission(CAP)),
            (2, share("0.5")),
            (3, mission(SEARCH)),
            (3, once()),
            (3, arc(50, 120)),
        ]
    );
    for (_, order) in played.all_orders() {
        serves_cover_under_the_hold(&order.reason);
    }
    played.every_chain_ends_with_the_commander();
}

#[test]
fn a_cover_intent_that_stands_is_served_and_the_staff_forms_nothing_under_the_hold() {
    let text = with(&command_table(
        1,
        "kind = \"set_intent\"\nintent = 2\nintent_kind = \"cover\"\nbase = 1",
    ));
    let played = play(&text);
    played.nothing_refused();
    assert_eq!(played.state.intents.len(), 2);
    let two = &played.state.intents[&intent(2)];
    assert_eq!(two.maker, Maker::Commander);
    assert_eq!(two.parent, None);
    for day in 1..=12 {
        let d = played.decisions(day);
        assert!(
            d.commands
                .iter()
                .all(|c| !matches!(c.kind, CommandKind::FormIntent { .. })),
            "day {day}"
        );
        // The hold's reason names intent 2.
        let hold = d
            .no_orders
            .iter()
            .find(|r| r.serves.intent == Some(intent(1)))
            .unwrap_or_else(|| panic!("day {day}: no reason of the hold"));
        mentions(hold, &["intent:2", "the staff forms no other"]);
        assert!(hold.serves.chain.is_empty());
    }
    // The orders are scenario 47's, each serving intent 2 with an empty
    // chain.
    for (i, expected) in table_orders().iter().enumerate() {
        let day = i as u32 + 1;
        assert_eq!(&played.air_orders(day), expected, "day {day}");
    }
    for (day, order) in played.all_orders() {
        assert_eq!(order.intent, intent(2), "day {day}");
        assert_eq!(order.reason.serves.intent, Some(intent(2)), "day {day}");
        assert!(order.reason.serves.chain.is_empty(), "day {day}");
    }
    played.every_chain_ends_with_the_commander();
}

#[test]
fn with_nothing_at_the_base_to_cover_it_with_the_staff_forms_nothing() {
    let text = with(&command_table(
        1,
        "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\nair_groups = [4]",
    ));
    let played = play(&text);
    played.nothing_refused();
    assert_eq!(played.state.intents.len(), 1);
    for day in 1..=12 {
        let d = played.decisions(day);
        assert!(d.orders.is_empty(), "day {day}");
        assert!(
            d.commands
                .iter()
                .all(|c| !matches!(c.kind, CommandKind::FormIntent { .. })),
            "day {day}"
        );
        assert_eq!(d.no_orders.len(), 1, "day {day}");
        serves_the_hold(&d.no_orders[0]);
        mentions(
            &d.no_orders[0],
            &["There is nothing at base:1 (Placeholder base 1) to cover it with"],
        );
    }
    played.every_chain_ends_with_the_commander();
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
    let hidden = changed(
        &text(),
        "side = \"japanese\"\nq = 5\nr = 0\nsupply_t = 0",
        "side = \"japanese\"\nq = 5\nr = 0\nsupply_t = 777",
    );
    let hidden = format!(
        "{hidden}\n[[base]]\nid = 3\nname = \"Placeholder base 3\"\nside = \"japanese\"\nq = 7\nr = 7\nsupply_t = 50\non_map = false\n\n[[air_group]]\nid = 5\nname = \"Placeholder Japanese fighters 5\"\nside = \"japanese\"\nbase = 3\naircraft_type = 4\nready = 20\n"
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

// ------------------------------------------- Rulings after the build
//
// The architect's rulings on pull request 350, after the build (decision
// 0117, "As built"): read up the tree, the narrowest binds, so a limit set on
// a parent after one on its child binds the child too; and a commander's
// `set_intent` on an intent in force may change its figures and not its kind.

#[test]
fn a_parents_risk_set_after_its_childs_binds_the_child() {
    // Intent 2's risk set to balanced on day 2 (taken: not bolder than its
    // parent's, which is unset and so balanced), then intent 1's set to
    // preserve on day 3: from day 3 the share ordered is 0.75 and the level
    // 30.
    let text = with(&format!(
        "{}{}",
        command_table(
            2,
            "kind = \"set_limit\"\nlimit = \"risk\"\nintent = 2\nrisk = \"balanced\""
        ),
        command_table(
            3,
            "kind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"preserve\""
        ),
    ));
    let played = play(&text);
    played.nothing_refused();
    assert_eq!(played.air_orders(1), table_orders()[0]);
    assert_eq!(played.air_orders(2), table_orders()[1]);
    // Day 3: both fighters at 12 are given the share 0.75; group 3, which
    // searched on day 2 and is at 44, above 30, is given rest. (Restated by
    // the architect on Nov 16, 2026, for decision 0131: before it, group 3
    // was resting already and day 3 had the shares alone.)
    assert_eq!(
        played.air_orders(3),
        vec![(1, share("0.75")), (2, share("0.75")), (3, mission(REST))]
    );
    // Both fighters first pass 30 on the morning of day 7, at 36: group 1
    // rests and group 2 is kept.
    assert_eq!(played.morning(7, 1), n(36));
    assert!(played.air_orders(7).contains(&(1, mission(REST))));
    assert!(!played.air_orders(7).contains(&(2, mission(REST))));
    assert_eq!(
        played.state.intents[&intent(2)].limits.risk,
        Some(sim_core::command::Risk::Balanced)
    );
    assert_eq!(
        played.state.risk_read_up(intent(2)),
        sim_core::command::Risk::Preserve
    );
    played.every_chain_ends_with_the_commander();
}

#[test]
fn a_parents_forces_limit_set_after_its_childs_binds_the_child() {
    // Intent 2's forces limit names groups 1, 2 and 3 on day 2 (taken: all
    // are at base 1), then intent 1's is set to groups 1 and 3 on day 3: group
    // 2 is given no staff order after.
    let text = with(&format!(
        "{}{}",
        command_table(
            2,
            "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 2\nair_groups = [1, 2, 3]"
        ),
        command_table(
            3,
            "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\nair_groups = [1, 3]"
        ),
    ));
    let played = play(&text);
    played.nothing_refused();
    assert_eq!(played.air_orders(1), table_orders()[0]);
    assert_eq!(played.air_orders(2), table_orders()[1]);
    for day in 3..=12 {
        assert!(
            played.air_orders(day).iter().all(|(g, _)| *g != 2),
            "group 2 is given an order on day {day}"
        );
    }
    // Group 1 is the only fighter the staff may order: it is kept on patrol
    // on day 9, at 48.
    assert_eq!(played.morning(9, 1), n(48));
    assert!(!played.air_orders(9).contains(&(1, mission(REST))));
    assert_eq!(
        played.state.groups_on_chain(intent(2)),
        Some([group(1), group(3)].into_iter().collect())
    );
}

#[test]
fn a_set_intent_may_not_change_the_kind_of_an_intent_in_force() {
    // On day 2 the commander sets intent 1, a hold, to cover base 1: refused;
    // the orders are those of the main case.
    let text = with(&command_table(
        2,
        "kind = \"set_intent\"\nintent = 1\nintent_kind = \"cover\"\nbase = 1",
    ));
    let played = play(&text);
    assert_eq!(
        played.refusals(),
        vec![(2, CommandRefusal::KindCannotChange(intent(1)))]
    );
    for (i, expected) in table_orders().iter().enumerate() {
        let day = i as u32 + 1;
        assert_eq!(&played.air_orders(day), expected, "day {day}");
    }
    assert!(matches!(
        played.state.intents[&intent(1)].kind,
        sim_core::command::IntentKind::Hold { .. }
    ));
}
