//! Scenario 47: the staff covers a base from the air. Two fighter groups,
//! a flying boat group and a dive bomber group at one base are given their
//! missions, patrol share, search arc and rest by the staff from one cover
//! intent, for 12 days.
//!
//! Proves decision 0115 against a hand calculation. The data is placeholder
//! data: see `data/scenarios/47_cover_from_the_air.toml`. It runs on the
//! frozen step 1 figures, with the fatigue figures loaded and the wear,
//! upkeep and supply figures not. The only Japanese thing is a base with
//! nothing at it, so nothing is sighted, no raid comes and no die is rolled.
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
//! Fatigue: a group that flew in a daylight pulse gains 3 and 2 for each
//! 100 nm of its longest sortie, there and back; a group that did not fly
//! in a daylight pulse loses 5; never below 0 or above 100. A patrol is a
//! sortie of no distance: 3. A search at 600 nm is a sortie of 1200 nm: 3 +
//! 24 = 27. A group flies its ready aircraft times 100 less its fatigue,
//! over 100, cut to a whole aircraft. A fighter group on patrol keeps its
//! share of those overhead, cut, and flies a patrol sortie in each daylight
//! pulse in which that is at least one aircraft. A contact gains a pulse of
//! age at the end of each pulse and is dropped when its age reaches 5.
//! Bearings are clockwise from north on a ten-degree grid; from a hex, the
//! hex 5 to its east at (5,0) lies on 090 and the hex at (0,5) on 150, and
//! an arc holds a hex that lies on its edge.
//!
//! The placeholder types: the fighter (type 4), the flying boat (type 1,
//! search range 600 nm) and the dive bomber (type 3, no search range).
//!
//! ## Inputs
//!
//! A placeholder map of open sea. Allied base 1 at (0,0). Japanese base 2
//! at (5,0), on the Allied map, with nothing at it. The game is 12 days.
//!
//! | Group | Type        | Ready aircraft | At the start |
//! | ----- | ----------- | -------------- | ------------ |
//! | 1     | fighter     | 12             | resting, no setting |
//! | 2     | fighter     | 8              | resting, no setting |
//! | 3     | flying boat | 6              | resting, no arc |
//! | 4     | dive bomber | 10             | resting |
//!
//! The Allied side starts holding one contact: 2 ships, a cruiser, at
//! (0,5), age 0.
//!
//! The Allied side's commands of day 1: intent 1, cover base 1; its forces
//! limit, groups 1, 2, 3 and 4; leave to prepare and leave to launch both
//! given. No risk is set: it is balanced. So the patrol share is 0.5 and a
//! group rests when its fatigue in the morning is above 45.
//!
//! ## The search arc
//!
//! - **Days 1 and 2.** The contact is 0 pulses old on the morning of day 1
//!   and 3 on the morning of day 2. The threats are base 2 at (5,0), on
//!   090, and the contact at (0,5), on 150. The narrowest arc that holds
//!   both runs from 090 to 150. Widened by 30 at each end it is 060 to 180.
//! - **From day 3.** The contact was dropped at the end of the morning of
//!   day 2, at 5 pulses. The one threat is base 2 on 090. The arcs of ten
//!   degrees that hold it are 080 to 090 and 090 to 100; the lower start is
//!   taken: 080 to 090. Widened it is 050 to 120.
//!
//! ## Day by day
//!
//! Fatigue is read in the morning, before the day's orders. "Flies" is a
//! day on patrol or on search; "rests" is a day on the rest mission.
//!
//! | Day | Group 1: fatigue, day | Group 2: fatigue, day | Group 3: fatigue, day | The staff's orders |
//! | --- | --------------------- | --------------------- | --------------------- | ------------------ |
//! | 1   | 0, flies              | 0, flies              | 0, flies              | Groups 1 and 2: patrol, share 0.5. Group 3: search, arc 060 to 180 at 600 nm |
//! | 2   | 6, flies              | 6, flies              | 54, rests             | Group 3: rest |
//! | 3   | 12, flies             | 12, flies             | 44, flies             | Group 3: search, arc 050 to 120 at 600 nm |
//! | 4   | 18, flies             | 18, flies             | 98, rests             | Group 3: rest |
//! | 5   | 24, flies             | 24, flies             | 88, rests             | none |
//! | 6   | 30, flies             | 30, flies             | 78, rests             | none |
//! | 7   | 36, flies             | 36, flies             | 68, rests             | none |
//! | 8   | 42, flies             | 42, flies             | 58, rests             | none |
//! | 9   | 48, rests             | 48, flies             | 48, rests             | Group 1: rest |
//! | 10  | 38, flies             | 54, rests             | 38, flies             | Group 1: patrol. Group 2: rest. Group 3: search |
//! | 11  | 44, flies             | 44, flies             | 92, rests             | Group 2: patrol. Group 3: rest |
//! | 12  | 50, rests             | 50, flies             | 82, rests             | Group 1: rest |
//!
//! At the end of day 12: group 1 has fatigue 40, group 2 has 56, group 3
//! has 72 and group 4 has 0.
//!
//! Why, where it is not plain:
//!
//! - **Day 1** has six orders: for each fighter group the patrol mission
//!   and the share; for group 3 the search mission and the arc. A mission
//!   and a setting are two orders. Group 4 is given none on any day: it is
//!   not a fighter and cannot search, and the day's reason says it is left
//!   for a strike intent.
//! - **Group 3's days in the air.** On day 1 it flies 6 in the morning and
//!   gains 27; in the afternoon 6 x 0.73, cut to 4, fly and it gains 27: 54.
//!   On day 3, at 44, 6 x 0.56, cut to 3, fly: 71; then 6 x 0.29, cut to 1:
//!   98. On day 10, at 38, 3 fly: 65; then 6 x 0.35, cut to 2: 92.
//! - **Day 3.** Group 3 is at 44, not above 45: it flies, and is given the
//!   search mission and the new arc. The arc's reason names the contact as
//!   gone.
//! - **Day 9.** Both fighter groups are at 48, above 45. They are taken
//!   most tired first, then by number: group 1 rests, and group 2, the last
//!   that can fly, is kept on patrol though it is above the level. Group 3
//!   is at 48 and rests: it has the rest mission already, so no order.
//! - **Day 10.** Group 1 is at 38 and flies. Group 2 is at 54 and may now
//!   rest, since group 1 flies.
//! - **Day 11.** Both fighters are at 44 and fly.
//! - **Day 12.** Both are at 50: group 1 rests and group 2 is kept.
//!
//! On every day at least one fighter group flies the patrol: both on days
//! 1 to 8 and 11, group 2 on days 9 and 12, group 1 on day 10. On a day a
//! fighter group flies it gains 6 and on a day it rests it loses 10. Group
//! 3 searches on days 1, 3 and 10 and no others.
//!
//! No staff order is refused. Every order and every morning without one
//! has a reason with these numbers: the fatigue against 45, the share, the
//! threats and the arc.
//!
//! # Properties
//!
//! - **The risk level.** At preserve the force the share is 0.75 and the
//!   level 30: both fighters first reach 36 on the morning of day 7, when
//!   group 1 rests. At press the share is 0.25 and the level 60: no
//!   fighter rests before day 12, when group 1 is at 66 and rests. Group 2,
//!   with 8 aircraft, keeps none overhead in a daylight pulse that begins
//!   with its fatigue above 50, since a quarter of the 3 it can then fly is
//!   cut to none, and so does not fly that pulse: its mornings from day 9
//!   are 48, 46, 52 and 50, and on day 12 it is not above the level and
//!   flies. (Restated by the architect on Nov 15, 2026, after the build: the
//!   first draft had both at 66.)
//! - **A raid raises the share.** In a variant with a Japanese bomber group
//!   at base 2 set to strike base 1 on day 3, the share of both fighter
//!   groups is 0.75 on days 4, 5 and 6 and 0.5 again on day 7.
//! - **No threat, the whole circle.** With base 2 off the Allied map and no
//!   contact, group 3's arc is the whole circle.
//! - **Nothing to cover with.** With both fighter groups at no ready
//!   aircraft, the staff hands in that it cannot meet the intent on day 1
//!   and the standing interrupt fires that day and on no later day.
//! - **A held group is left alone:** with group 2 held from day 5, it is
//!   given no order from then, and on day 9 group 1, now the only fighter
//!   the staff may order, is kept on patrol: the staff plans without what
//!   is held.
//! - **An override.** A commander's order to group 3 on day 2 holds it, and
//!   the staff gives it no order after.
//! - The staff never orders a group outside the intent's forces limit, and
//!   the same start gives the same orders.
//! - Two worlds that differ only in what the Allied side cannot see give
//!   the same orders and reasons.

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::command::PlanId;
use sim_core::command::{Reason, StaffDecisions, StaffOrder};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::ids::{AirGroupId, BaseId, IntentId};
use sim_core::orders::{AirGroupOrder, AirMission, OrderKind};
use sim_core::state::{Side, TruthState};
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

mod common;

const COMMANDS_MARKER: &str = "# The Allied commander's commands";

fn path() -> PathBuf {
    common::data_dir().join("scenarios/47_cover_from_the_air.toml")
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
/// start) and the log.
struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    truth: Vec<TruthState>,
    events: Vec<Event>,
    /// The days on which the Allied side's own events say base 1 was raided
    /// from the air.
    raid_days: Vec<u32>,
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
    let raid_days = scenario
        .world
        .side_view(Side::Allied)
        .air_raids_on(BaseId::new(1))
        .iter()
        .map(|raid| raid.day)
        .collect();
    Played {
        handed,
        truth,
        events: scenario.world.events().iter().cloned().collect(),
        raid_days,
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
}

fn mission(m: AirMission) -> AirGroupOrder {
    AirGroupOrder::SetAirMission(m)
}

fn share(s: &str) -> AirGroupOrder {
    AirGroupOrder::SetPatrolShare { share: fx(s) }
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

/// The staff's orders of the main case, by day, as the table has them.
fn table_orders() -> Vec<Vec<(u32, AirGroupOrder)>> {
    vec![
        // Day 1
        vec![
            (1, mission(CAP)),
            (1, share("0.5")),
            (2, mission(CAP)),
            (2, share("0.5")),
            (3, mission(SEARCH)),
            (3, arc(60, 180)),
        ],
        // Day 2
        vec![(3, mission(REST))],
        // Day 3
        vec![(3, mission(SEARCH)), (3, arc(50, 120))],
        // Day 4
        vec![(3, mission(REST))],
        vec![],
        vec![],
        vec![],
        vec![],
        // Day 9
        vec![(1, mission(REST))],
        // Day 10
        vec![(1, mission(CAP)), (2, mission(REST)), (3, mission(SEARCH))],
        // Day 11
        vec![(2, mission(CAP)), (3, mission(REST))],
        // Day 12
        vec![(1, mission(REST))],
    ]
}

/// Fatigue on the morning of each day: (group 1, group 2, group 3).
const MORNINGS: [(i64, i64, i64); 12] = [
    (0, 0, 0),
    (6, 6, 54),
    (12, 12, 44),
    (18, 18, 98),
    (24, 24, 88),
    (30, 30, 78),
    (36, 36, 68),
    (42, 42, 58),
    (48, 48, 48),
    (38, 54, 38),
    (44, 44, 92),
    (50, 50, 82),
];

// ------------------------------------------------------- The hand calculation

#[test]
fn the_commands_of_the_file_are_the_ones_these_tests_vary() {
    let c = commands();
    assert_eq!(c.matches("[[command]]").count(), 4);
    assert!(c.contains("intent_kind = \"cover\""));
    assert!(c.contains("air_groups = [1, 2, 3, 4]"));
}

#[test]
fn the_staffs_orders_of_every_morning_follow_the_table() {
    let played = play(&text());
    for (i, expected) in table_orders().iter().enumerate() {
        let day = i as u32 + 1;
        assert_eq!(&played.air_orders(day), expected, "the orders of day {day}");
    }
}

#[test]
fn the_staff_gives_six_one_two_one_none_none_none_none_one_three_two_one_orders() {
    let played = play(&text());
    assert_eq!(played.counts(), vec![6, 1, 2, 1, 0, 0, 0, 0, 1, 3, 2, 1]);
    assert_eq!(played.counts().iter().sum::<usize>(), 17);
}

#[test]
fn every_staff_order_is_an_air_group_order_and_none_is_to_group_4() {
    let played = play(&text());
    for (day, order) in played.all_orders() {
        match &order.kind {
            OrderKind::AirGroup { target, .. } => {
                assert_ne!(*target, group(4), "group 4 was ordered on day {day}")
            }
            other => panic!("day {day}: {other:?}"),
        }
        assert_eq!(order.intent, IntentId::new(1));
    }
}

#[test]
fn every_groups_fatigue_on_every_morning_and_at_the_end_follows_the_table() {
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
    assert_eq!(played.at_end(3), n(72));
    assert_eq!(played.at_end(4), n(0));
}

#[test]
fn a_fighter_group_gains_6_on_a_day_it_flies_and_loses_10_on_a_day_it_rests() {
    let played = play(&text());
    // The table's own words: group 1 rests on days 9 and 12, group 2 on day
    // 10; group 3 flies on days 1, 3 and 10 (+54, +54, +54) and rests
    // otherwise.
    for day in 1..=12u32 {
        for (raw, rests) in [(1u32, [9u32, 12]), (2, [10, 10])] {
            let before = played.morning(day, raw);
            let after = if day == 12 {
                played.at_end(raw)
            } else {
                played.morning(day + 1, raw)
            };
            let expected = if rests.contains(&day) {
                before - n(10)
            } else {
                before + n(6)
            };
            assert_eq!(after, expected, "group {raw}, day {day}");
        }
    }
}

#[test]
fn group_3_searches_on_days_1_3_and_10_and_no_others() {
    let played = play(&text());
    let mut searched = Vec::new();
    for day in 1..=12u32 {
        let before = played.morning(day, 3);
        let after = if day == 12 {
            played.at_end(3)
        } else {
            played.morning(day + 1, 3)
        };
        if after > before {
            searched.push(day);
        }
    }
    assert_eq!(searched, vec![1, 3, 10]);
}

#[test]
fn on_every_day_at_least_one_fighter_group_flies_the_patrol() {
    let played = play(&text());
    // Group 1 rests on days 9 and 12, group 2 on day 10: never both.
    for day in 1..=12u32 {
        let flies = |raw: u32| {
            let before = played.morning(day, raw);
            let after = if day == 12 {
                played.at_end(raw)
            } else {
                played.morning(day + 1, raw)
            };
            after > before
        };
        assert!(flies(1) || flies(2), "no fighter flew on day {day}");
        let both = ![9, 10, 12].contains(&day);
        assert_eq!(flies(1) && flies(2), both, "day {day}");
    }
}

#[test]
fn no_staff_order_and_no_staff_command_is_refused() {
    let played = play(&text());
    played.nothing_refused();
    // Both leaves: the staff made plan 1 ready and released it on day 1.
    assert_eq!(played.decisions(1).commands.len(), 2);
    for day in 2..=12 {
        assert!(played.decisions(day).commands.is_empty(), "day {day}");
    }
    // Every order was accepted.
    let accepted = played.all(|k| matches!(k, EventKind::OrderAccepted { .. }));
    assert!(accepted.len() >= 17, "{} accepted", accepted.len());
}

// -------------------------------------------------------------- The reasons

fn serves_the_intent(reason: &Reason) {
    assert_eq!(reason.serves.intent, Some(IntentId::new(1)));
    assert_eq!(reason.serves.plan, Some(PlanId::new(1)));
    assert_eq!(reason.serves.amendment, None);
    assert!(
        reason.serves.chain.is_empty(),
        "intent 1 is the commander's own: no chain"
    );
    assert!(!reason.serves.statement.is_empty());
    assert!(!reason.worked.is_empty());
    assert!(!reason.evidence.is_empty() || !reason.binding.is_empty());
}

fn mentions(reason: &Reason, words: &[&str]) {
    for word in words {
        assert!(
            reason.worked.contains(word),
            "the reason's worked part has no \"{word}\": {}",
            reason.worked
        );
    }
}

#[test]
fn every_order_and_every_morning_without_one_has_a_reason_serving_intent_1_and_plan_1() {
    let played = play(&text());
    for (day, order) in played.all_orders() {
        serves_the_intent(&order.reason);
        let _ = day;
    }
    for day in 1..=12 {
        let d = played.decisions(day);
        // Group 4 is left for a strike intent on every day, so every morning
        // has one reason of the doctrine: the one for groups it orders
        // nothing for. The staff gives no notice.
        assert_eq!(d.no_orders.len(), 1, "day {day}");
        assert!(d.notices.is_empty(), "day {day}");
        serves_the_intent(&d.no_orders[0]);
        mentions(
            &d.no_orders[0],
            &[
                "air_group:4",
                "left for a strike intent",
                "The share is 0.5",
            ],
        );
    }
}

#[test]
fn the_reasons_of_the_fighters_name_the_fatigue_against_the_level_and_the_share() {
    let played = play(&text());
    for (day, order) in played.all_orders() {
        let OrderKind::AirGroup { target, order: o } = &order.kind else {
            unreachable!()
        };
        let raw = target.raw();
        let fatigue = {
            let m = MORNINGS[(day - 1) as usize];
            match raw {
                1 => m.0,
                2 => m.1,
                _ => m.2,
            }
        };
        let w = &order.reason.worked;
        match o {
            AirGroupOrder::SetAirMission(_) => {
                assert!(
                    w.contains(&format!(
                        "air_group:{raw}, a {}, has fatigue {fatigue} against the 45 of the balanced risk",
                        if raw == 3 { "search group" } else { "fighter group" }
                    )),
                    "day {day}: {w}"
                );
            }
            AirGroupOrder::SetPatrolShare { .. } => {
                assert!(
                    w.contains(&format!(
                        "The patrol share of air_group:{raw} is set from 1 to 0.5"
                    )),
                    "day {day}: {w}"
                );
                assert!(w.contains("0.5 at balanced"), "day {day}: {w}");
            }
            AirGroupOrder::SetSearchArc { .. } => {}
            other => panic!("{other:?}"),
        }
    }
}

#[test]
fn the_rest_reasons_say_why_the_group_rests_or_is_kept() {
    let played = play(&text());
    let worked = |day: u32, index: usize| played.decisions(day).orders[index].reason.worked.clone();
    // Group 3 rests on day 2 at 54, day 4 at 98 and day 11 at 92.
    assert!(worked(2, 0).contains("above the level and a search group is never kept"));
    assert!(worked(4, 0).contains("fatigue 98 against the 45"));
    assert!(worked(11, 1).contains("fatigue 92 against the 45"));
    // Group 1 rests on days 9 and 12 because another fighter group can fly.
    assert!(worked(9, 0).contains("fatigue 48 against the 45"));
    assert!(worked(9, 0).contains("another fighter group can fly the patrol"));
    assert!(worked(12, 0).contains("fatigue 50 against the 45"));
    assert!(worked(12, 0).contains("another fighter group can fly the patrol"));
    // Group 2, the last that can fly, is kept on patrol on days 9 and 12: no
    // order, and the day's one reason says so.
    for (day, fatigue) in [(9u32, 48), (12, 50)] {
        let r = &played.decisions(day).no_orders[0];
        mentions(
            r,
            &[
                &format!("air_group:2, a fighter group, has fatigue {fatigue} against the 45"),
                "it is above the level but the last fighter group that can fly, so it is never rested",
            ],
        );
    }
    // Group 2 rests on day 10 at 54.
    assert!(worked(10, 1).contains("air_group:2, a fighter group, has fatigue 54 against the 45"));
}

#[test]
fn the_arc_reasons_name_the_threats_and_the_arc() {
    let played = play(&text());
    let day1 = &played.decisions(1).orders[5].reason.worked;
    for word in [
        "060 to 180 out to 600 nm",
        "base:2 (Placeholder base 2) at (5,0)",
        "contact 1 at (0,5)",
        "the narrowest arc that holds them is 090 to 150",
        "widened by 30 degrees at each end to 060 to 180",
    ] {
        assert!(day1.contains(word), "day 1: no \"{word}\": {day1}");
    }
    // Day 3: the contact was dropped at the end of the morning of day 2 and
    // the reason names it as gone.
    let day3 = &played.decisions(3).orders[1].reason.worked;
    for word in [
        "050 to 120 out to 600 nm",
        "base:2 (Placeholder base 2) at (5,0)",
        "the narrowest arc that holds them is 080 to 090",
        "widened by 30 degrees at each end to 050 to 120",
        "contact 1 (dropped on day 2) held when its arc was set is gone",
    ] {
        assert!(day3.contains(word), "day 3: no \"{word}\": {day3}");
    }
    assert!(!day3.contains("contact 1 at (0,5)"));
    // The mornings in which the contact is held name both threats in the
    // reason of no order; from day 3 only the base.
    for day in 1..=2 {
        mentions(
            &played.decisions(day).no_orders[0],
            &["contact 1 at (0,5)", "the arc is 060 to 180"],
        );
    }
    for day in 3..=12 {
        let r = &played.decisions(day).no_orders[0];
        mentions(
            r,
            &[
                "the arc is 050 to 120",
                "base:2 (Placeholder base 2) at (5,0)",
            ],
        );
        assert!(!r.worked.contains("contact 1 at (0,5)"), "day {day}");
    }
}

#[test]
fn the_morning_without_an_order_names_every_groups_fatigue_against_the_level() {
    let played = play(&text());
    for day in 5..=8u32 {
        let (g1, g2, g3) = MORNINGS[(day - 1) as usize];
        mentions(
            &played.decisions(day).no_orders[0],
            &[
                &format!("air_group:1, a fighter group, has fatigue {g1} against the 45"),
                &format!("air_group:2, a fighter group, has fatigue {g2} against the 45"),
                &format!("air_group:3, a search group, has fatigue {g3} against the 45"),
                "it is not above the level, so it flies",
                "it is above the level, so it rests (mission rest already)",
            ],
        );
    }
}

// --------------------------------------------------------------- Properties

#[test]
fn at_preserve_the_force_the_share_is_three_quarters_and_the_level_is_30() {
    let text = with(&command_table(
        1,
        "kind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"preserve\"",
    ));
    let played = play(&text);
    played.nothing_refused();
    assert_eq!(played.air_orders(1)[1], (1, share("0.75")));
    assert_eq!(played.air_orders(1)[3], (2, share("0.75")));
    // Both fighters first reach 36 on the morning of day 7; before it, none
    // is above 30 and none rests.
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
    assert!(played.air_orders(7).contains(&(1, mission(REST))));
    mentions(
        &played.decisions(7).orders[0].reason,
        &["air_group:1, a fighter group, has fatigue 36 against the 30 of the preserve risk"],
    );
}

fn press_text() -> String {
    with(&command_table(
        1,
        "kind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"press\"",
    ))
}

#[test]
fn at_press_the_force_the_share_is_a_quarter() {
    let played = play(&press_text());
    played.nothing_refused();
    assert_eq!(played.air_orders(1)[1], (1, share("0.25")));
    assert_eq!(played.air_orders(1)[3], (2, share("0.25")));
}

#[test]
fn at_press_the_force_no_fighter_rests_before_day_12() {
    let played = play(&press_text());
    for day in 1..=11 {
        assert!(
            played
                .air_orders(day)
                .iter()
                .all(|(g, o)| !(*g <= 2 && *o == mission(REST))),
            "a fighter rested on day {day}"
        );
    }
}

#[test]
fn at_press_the_force_group_1_is_at_66_on_day_12_and_rests() {
    let played = play(&press_text());
    assert_eq!(played.morning(12, 1), n(66));
    assert!(played.air_orders(12).contains(&(1, mission(REST))));
    let rest = played
        .decisions(12)
        .orders
        .iter()
        .find(|o| matches!(&o.kind, OrderKind::AirGroup { target, order: AirGroupOrder::SetAirMission(AirMission::Rest) } if *target == group(1)))
        .expect("group 1's rest order");
    mentions(
        &rest.reason,
        &["fatigue 66 against the 60 of the press risk"],
    );
}

// At press group 2 keeps none overhead in a pulse that begins above fatigue
// 50 (a quarter of the 3 aircraft it can then fly is cut to none), so it does
// not fly that pulse; the architect restated the property so after the build.
#[test]
fn at_press_group_2_has_mornings_48_46_52_50_and_flies_on_day_12() {
    let played = play(&press_text());
    let mornings: Vec<Fixed> = (9..=12).map(|day| played.morning(day, 2)).collect();
    assert_eq!(mornings, vec![n(48), n(46), n(52), n(50)]);
    // On day 12 it is not above the level of 60: no rest order, and it ends
    // the day on the patrol.
    assert!(
        !played
            .air_orders(12)
            .iter()
            .any(|(g, order)| *g == 2 && *order == AirGroupOrder::SetAirMission(AirMission::Rest)),
        "group 2 is rested on day 12"
    );
    assert_eq!(played.mission_at_end(2), AirMission::CombatAirPatrol);
}

/// A Japanese bomber group at base 2 set to strike base 1 on day 3 and to
/// rest on day 4, so that base 1 is raided on day 3 and on no other day.
const RAIDER: &str = r#"
[[air_group]]
id = 5
name = "Placeholder Japanese bombers 5"
side = "japanese"
base = 2
aircraft_type = 3
ready = 18

[[order]]
day = 3
side = "japanese"
air_group = 5
kind = "set_air_mission"
mission = "base_strike"
target_base = 1

[[order]]
day = 4
side = "japanese"
air_group = 5
kind = "set_air_mission"
mission = "rest"
"#;

#[test]
fn a_raid_raises_the_share_for_the_three_days_after_it() {
    let played = play(&with(RAIDER));
    played.nothing_refused();
    assert!(
        !played.raid_days.is_empty(),
        "the Japanese bombers did not raid"
    );
    assert!(
        played.raid_days.iter().all(|d| *d == 3),
        "raids only on day 3: {:?}",
        played.raid_days
    );
    // The share ordered each day, from what the group's share was before.
    for day in 1..=12u32 {
        let wanted = if (4..=6).contains(&day) {
            "0.75"
        } else {
            "0.5"
        };
        let shares: Vec<(u32, Fixed)> = played
            .air_orders(day)
            .into_iter()
            .filter_map(|(g, o)| match o {
                AirGroupOrder::SetPatrolShare { share } => Some((g, share)),
                _ => None,
            })
            .collect();
        match day {
            1 => assert_eq!(shares, vec![(1, fx("0.5")), (2, fx("0.5"))]),
            4 => assert_eq!(shares, vec![(1, fx(wanted)), (2, fx(wanted))]),
            7 => assert_eq!(shares, vec![(1, fx(wanted)), (2, fx(wanted))]),
            _ => assert!(shares.is_empty(), "day {day}: {shares:?}"),
        }
    }
    // The reasons say what raised it.
    let on4 = played.decisions(4).orders.iter().find(|o| {
        matches!(
            &o.kind,
            OrderKind::AirGroup {
                order: AirGroupOrder::SetPatrolShare { .. },
                ..
            }
        )
    });
    let w = &on4.expect("a share order on day 4").reason.worked;
    assert!(
        w.contains("0.75: 0.5 at balanced raised by 0.25 because base:1 (Placeholder base 1) was attacked from the air on day 3"),
        "{w}"
    );
}

#[test]
fn with_no_threat_group_3_searches_the_whole_circle() {
    let text = changed(
        &text(),
        "side = \"japanese\"\nq = 5\nr = 0\nsupply_t = 0",
        "side = \"japanese\"\nq = 5\nr = 0\nsupply_t = 0\non_map = false",
    );
    let text = changed(
        &text,
        "[[contact]]\nside = \"allied\"\nq = 0\nr = 5\nship_count = 2\nkind = \"cruiser\"\nage_pulses = 0\n",
        "",
    );
    let played = play(&text);
    played.nothing_refused();
    let day1 = played.air_orders(1);
    assert_eq!(day1.len(), 6);
    assert_eq!(day1[5], (3, arc(0, 0)));
    mentions(
        &played.decisions(1).orders[5].reason,
        &[
            "the whole circle out to 600 nm",
            "no enemy base on the map and no contact: the arc is the whole circle",
        ],
    );
    // No contact is ever dropped: the arc is never ordered again.
    for day in 2..=12 {
        assert!(
            played
                .air_orders(day)
                .iter()
                .all(|(_, o)| !matches!(o, AirGroupOrder::SetSearchArc { .. })),
            "day {day}"
        );
    }
}

#[test]
fn with_no_ready_fighters_the_staff_cannot_meet_the_intent_and_the_interrupt_fires_on_day_1_only() {
    let text = changed(&text(), "ready = 12", "ready = 0");
    let text = changed(&text, "ready = 8", "ready = 0");
    let played = play(&text);
    played.nothing_refused();
    let d1 = played.decisions(1);
    assert!(
        d1.notices
            .iter()
            .any(|n| n.kind == sim_core::command::NoticeKind::CannotMeet),
        "no cannot-meet notice on day 1"
    );
    assert!(d1.no_orders.iter().any(|r| r.worked.contains(
        "The staff cannot meet intent:1: no fighter group of it has an aircraft able to fly."
    )));
    let fired = played.all(|k| {
        matches!(
            k,
            EventKind::InterruptFired {
                interrupt: sim_core::command::InterruptRef::Standing {
                    kind: sim_core::command::StandingKind::CannotMeet,
                    ..
                },
                ..
            }
        )
    });
    assert_eq!(fired.len(), 1, "{fired:?}");
    assert_eq!(fired[0].day, 1);
}

#[test]
fn a_held_group_is_left_alone_and_the_staff_plans_without_it() {
    let text = with(&command_table(5, "kind = \"hold\"\nair_group = 2"));
    let played = play(&text);
    played.nothing_refused();
    for day in 5..=12 {
        assert!(
            played.air_orders(day).iter().all(|(g, _)| *g != 2),
            "group 2 was ordered on day {day}"
        );
    }
    // Before the hold the orders are those of the main case.
    for day in 1..=4 {
        assert_eq!(played.air_orders(day), table_orders()[(day - 1) as usize]);
    }
    // On day 9 group 1, now the only fighter the staff may order, is at 48
    // and is kept: no order, and the reason says so. It is never rested.
    assert!(played.air_orders(9).is_empty());
    mentions(
        &played.decisions(9).no_orders[0],
        &[
            "air_group:1, a fighter group, has fatigue 48 against the 45",
            "so it is never rested",
        ],
    );
    assert_eq!(played.mission_at_end(1), CAP);
    // Group 2 keeps the mission it had when it was held.
    assert_eq!(played.mission_at_end(2), CAP);
}

#[test]
fn an_override_holds_group_3_and_the_staff_gives_it_no_order_after() {
    let order = r#"
[[order]]
day = 2
side = "allied"
air_group = 3
kind = "set_air_mission"
mission = "search"
"#;
    let played = play(&with(order));
    played.nothing_refused();
    assert!(
        !played
            .all(|k| matches!(k, EventKind::HeldByOrder { .. }))
            .is_empty(),
        "the order did not hold the group"
    );
    assert_eq!(played.air_orders(1), table_orders()[0]);
    for day in 2..=12 {
        assert!(
            played.air_orders(day).iter().all(|(g, _)| *g != 3),
            "group 3 was ordered on day {day}"
        );
    }
    // It keeps the commander's mission to the end.
    assert_eq!(played.mission_at_end(3), SEARCH);
}

#[test]
fn the_staff_never_orders_a_group_outside_the_intents_forces_limit() {
    let text = changed(&text(), "air_groups = [1, 2, 3, 4]", "air_groups = [1, 3]");
    let played = play(&text);
    played.nothing_refused();
    for (day, order) in played.all_orders() {
        let OrderKind::AirGroup { target, .. } = &order.kind else {
            unreachable!()
        };
        assert!(
            [group(1), group(3)].contains(target),
            "group {target:?} ordered on day {day}"
        );
    }
    assert_eq!(played.at_end(2), n(0));
    assert_eq!(played.at_end(4), n(0));
}

#[test]
fn the_same_start_gives_the_same_orders_and_reasons() {
    let a = play(&text());
    let b = play(&text());
    assert_eq!(a.handed, b.handed);
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
}

#[test]
fn why_on_a_staff_order_prints_what_it_serves() {
    let mut scenario: Scenario = common::from_text(&text(), &path()).expect("the scenario loads");
    let held = staffs();
    scenario
        .next_day_watched(as_staffs(&held), &mut |_, _| {})
        .expect("day 1");
    let view = scenario.world.side_view(Side::Allied);
    let events = view.events();
    let found = events
        .iter()
        .position(|e| {
            matches!(
                &e.kind,
                sim_core::side_events::SideEventKind::Whole(EventKind::StaffReason {
                    order: Some(_),
                    ..
                })
            )
        })
        .expect("a staff reason with an order");
    let said = sim_agent::why(&view, found as u32 + 1);
    let text = said.to_string();
    assert!(text.contains("intent 1"), "{text}");
    assert!(text.contains("plan 1"), "{text}");
    assert!(text.contains("has fatigue 0 against the 45"), "{text}");
}
