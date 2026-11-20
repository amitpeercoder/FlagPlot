//! Scenario 59: a search that can be kept up. A flying boat group under a
//! cover searches to 600 nm every morning for 12 days, where under the
//! rules before it would have been exhausted on the second.
//!
//! Proves decision 0131 against a hand calculation. The data is placeholder
//! data: see `data/scenarios/59_a_search_kept_up.toml`. It runs on the
//! frozen step 1 figures with the fatigue figures loaded and one figure
//! added in the scenario's own file: a search figure of 0.15 for each 100
//! nm. The wear, upkeep and supply figures are not loaded. The only
//! Japanese thing is a base with nothing at it: nothing is sighted and no
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
//! A group that flew in a daylight pulse gains 3 for the sortie and, for a
//! search, 0.15 for each 100 nm of it, there and back; for any other
//! sortie, 2. A group that did not fly in a daylight pulse loses 5; never
//! below 0. The placeholder flying boat (type 1) searches to 600 nm: a
//! sortie of 1200 nm, which gains 3 + 12 x 0.15 = 4.8. A group flies its
//! ready aircraft times 100 less its fatigue, over 100, cut to a whole
//! aircraft. From (0,0) the hex (5,0) lies on 090.
//!
//! ## Inputs
//!
//! Scenario 47's map: Allied base 1 at (0,0); Japanese base 2 at (5,0), on
//! the Allied map, with nothing at it. The game is 12 days.
//!
//! One group at base 1: group 1, placeholder flying boats, 6 ready,
//! resting, with no arc and the search-once setting off.
//!
//! The Allied side's commands of day 1: intent 1, cover base 1; leave to
//! prepare and leave to launch both given. No risk: it is balanced. A
//! group rests when its fatigue in the morning is above 45.
//!
//! ## The orders of day 1
//!
//! Three, serving intent 1: the search mission; search once a day, on;
//! and the arc. The one threat is base 2, on 090: the arc is 080 to 090,
//! widened to 050 to 120, at 600 nm, as scenario 47 has it from day 3.
//!
//! ## Day by day
//!
//! Every day is the same. In the morning the group is at fatigue 0 and
//! flies its 6. It gains 4.8. In the afternoon it does not fly and loses
//! 5: it is at 0 again.
//!
//! On every morning of the 12 days its fatigue is 0, it flies 6 aircraft
//! in the morning and none in the afternoon, and it never rests. The staff
//! gives three orders on day 1 and none after.
//!
//! # Properties
//!
//! - **In both pulses, at press.** With the risk of intent 1 set to press
//!   on day 1: the search-once order is not given, and the group searches
//!   in both daylight pulses, gaining 9.6 a day. Its mornings are 0, 9.6,
//!   19.2, 28.8, 38.4, 48, 57.6 and, on day 8, 67.2, above the 60 of
//!   press: it rests, losing 10. Then 57.2 on day 9, when it flies; 66.8
//!   on day 10, when it rests; 56.8 on day 11, when it flies; 66.4 on day
//!   12, when it rests. The staff gives 2 orders on day 1 and one on each
//!   of days 8 to 12: seven.
//! - **Without the search figure.** With no search figure in the file, a
//!   search costs what every sortie costs: 3 + 12 x 2 = 27. Once a day, at
//!   balanced, the group gains 22 a day: its mornings are 0, 22 and 44 on
//!   days 1 to 3, when it flies, and 66 on day 4, above 45. It rests on
//!   days 4, 5 and 6, at 66, 56 and 46; flies on day 7 at 36; rests on
//!   days 8 and 9 at 58 and 48; flies on day 10 at 38; rests on days 11
//!   and 12 at 60 and 50. It searches on days 1, 2, 3, 7 and 10.
//! - **The rule alone.** A group on the search mission with the setting on
//!   records a search in the morning and none in the afternoon; with it
//!   off, in both. A strike group's fatigue for a sortie of 1200 nm is 27
//!   whether or not the search figure is given.
//! - **The least tired bomber searches, on a carrier.** In a view built
//!   for it: a force with no group that can only search and two bomber
//!   groups that can search, at fatigue 10 and 4: the one at 4 searches,
//!   once a day at preserve the force, and the other strikes. (It read "at
//!   balanced" until rule 6 was amended on Nov 16, 2026: a carrier force
//!   searches once a day at preserve the force only.) At equal fatigue the
//!   lower numbered searches. A group that cannot fly is passed over.
//! - **The least tired bomber searches, at a base.** In a view built for
//!   it: a base with a cover that has no searcher and a strike task with
//!   two bomber groups that can search: the less tired flies the cover's
//!   arc and the other holds the strike. With one such group, it holds
//!   the strike and nobody searches. With a group at the base that can
//!   only search, it searches and both bombers strike.
//! - **An order to set search once a day** on a group that is not the
//!   side's is refused.
//! - Every hand-calculation test on the frozen figures built before this
//!   one passes untouched: none gives a search figure or the setting.

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::command::{Risk, StaffDecisions};
use sim_core::events::{Event, EventKind, SortieKind};
use sim_core::fixed::Fixed;
use sim_core::ids::AirGroupId;
use sim_core::orders::{
    AirGroupOrder, AirMission, OrderKind, OrderRequest, Origin, Reach, RejectReason,
};
use sim_core::refdata::ShipKind;
use sim_core::state::{Side, TruthState};
use sim_core::time::Pulse;
use sim_data::Scenario;
use sim_staff::carrier::{decide, roles, CarrierGroup, CarrierRole};
use sim_staff::search::{base_turn, least_tired, once_a_day, TurnCandidate};
use sim_staff::{as_staffs, staffs};

mod common;

const COMMANDS_MARKER: &str = "# The Allied commander's commands";
const FATIGUE_FILE: &str = "fatigue_placeholder_search_kept_up.toml";

fn path() -> PathBuf {
    common::data_dir().join("scenarios/59_a_search_kept_up.toml")
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

/// The scenario's text with `extra` after its commands.
fn with(extra: &str) -> String {
    format!("{}{}", text(), extra)
}

/// The scenario's text with no command of the commander's.
fn without_commands() -> String {
    let text = text();
    let at = text.find(COMMANDS_MARKER).expect("the marker");
    text[..at].to_string()
}

/// The frozen figures with the search figure taken out of the scenario's
/// fatigue file: with none, a search costs what every sortie costs.
fn no_search_figure() -> sim_data::LoadOptions {
    common::altered_reference("s59", |dir| {
        common::edit_file_by(
            dir,
            FATIGUE_FILE,
            &[("search_fatigue_per_100_nm = \"0.15\"\n", "")],
        );
    })
}

/// A game played to its end with the doctrine's staffs.
struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    truth: Vec<TruthState>,
    events: Vec<Event>,
}

fn play_with(text: &str, options: &sim_data::LoadOptions) -> Played {
    let mut scenario: Scenario =
        common::from_text_with(text, &path(), options).expect("the scenario loads");
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
    }
}

fn play(text: &str) -> Played {
    play_with(text, &common::frozen())
}

impl Played {
    /// Group 1's fatigue on the morning of `day`, before the day's orders.
    fn morning(&self, day: u32) -> Fixed {
        self.truth[(day - 1) as usize].air_groups[&group(1)].fatigue
    }

    fn mornings(&self) -> Vec<Fixed> {
        (1..=12).map(|d| self.morning(d)).collect()
    }

    /// The staff's orders of `day` to air groups, as (group, order).
    fn air_orders(&self, day: u32) -> Vec<(u32, AirGroupOrder)> {
        self.handed[&day]
            .orders
            .iter()
            .map(|o| match &o.kind {
                OrderKind::AirGroup { target, order } => (target.raw(), order.clone()),
                other => panic!("the air doctrine gave a non-air order: {other:?}"),
            })
            .collect()
    }

    fn counts(&self) -> Vec<usize> {
        (1..=12).map(|d| self.handed[&d].orders.len()).collect()
    }

    /// The aircraft group 1 sent on a search in `pulse` of `day`, if it
    /// searched.
    fn searched(&self, day: u32, pulse: Pulse) -> Option<u32> {
        self.events.iter().find_map(|e| match &e.kind {
            EventKind::SearchFlown {
                air_group,
                aircraft,
                ..
            } if *air_group == group(1) && e.day == day && e.pulse == Some(pulse) => {
                Some(*aircraft)
            }
            _ => None,
        })
    }

    /// The days on which group 1 searched in the morning.
    fn search_days(&self) -> Vec<u32> {
        (1..=12)
            .filter(|d| self.searched(*d, Pulse::Morning).is_some())
            .collect()
    }

    /// The days on which group 1 has the rest mission at the end of the
    /// staff's orders: the days it rests.
    fn rest_days(&self) -> Vec<u32> {
        (1..=12)
            .filter(|d| self.truth[*d as usize].air_groups[&group(1)].mission == AirMission::Rest)
            .collect()
    }

    fn nothing_refused(&self) {
        let refused: Vec<&Event> = self
            .events
            .iter()
            .filter(|e| {
                matches!(
                    e.kind,
                    EventKind::OrderRejected { .. } | EventKind::CommandRefused { .. }
                )
            })
            .collect();
        assert!(refused.is_empty(), "something was refused: {refused:?}");
    }
}

fn mission(m: AirMission) -> AirGroupOrder {
    AirGroupOrder::SetAirMission(m)
}

fn arc() -> AirGroupOrder {
    AirGroupOrder::SetSearchArc {
        from_deg: 50,
        to_deg: 120,
        range_nm: n(600),
    }
}

fn press_text() -> String {
    with("\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"press\"\n")
}

// ------------------------------------------------------- The hand calculation

#[test]
fn the_scenarios_fatigue_file_gives_a_search_figure_of_0_15_and_the_rest_as_frozen() {
    let scenario = common::load(&path()).expect("the scenario loads");
    let figures = scenario
        .world
        .ref_data()
        .fatigue
        .expect("the fatigue figures");
    assert_eq!(figures.search_fatigue_per_100_nm, Some(fx("0.15")));
    assert_eq!(figures.air_fatigue_per_sortie, n(3));
    assert_eq!(figures.air_fatigue_per_100_nm, n(2));
    assert_eq!(figures.air_rest_per_pulse, n(5));
    // A search to 600 nm is a sortie of 1200 nm: 3 + 12 x 0.15 = 4.8.
    assert_eq!(figures.gain_for(SortieKind::Search, n(1200)), fx("4.8"));
}

#[test]
fn the_staff_gives_the_search_mission_search_once_on_and_the_arc_on_day_1_and_nothing_after() {
    let played = play(&text());
    played.nothing_refused();
    assert_eq!(
        played.air_orders(1),
        vec![
            (1, mission(AirMission::Search)),
            (1, AirGroupOrder::SetSearchOnce { on: true }),
            (1, arc()),
        ]
    );
    assert_eq!(played.counts(), vec![3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn every_morning_the_group_is_at_0_flies_6_in_the_morning_and_none_in_the_afternoon() {
    let played = play(&text());
    for day in 1..=12 {
        assert_eq!(played.morning(day), Fixed::ZERO, "day {day}");
        assert_eq!(played.searched(day, Pulse::Morning), Some(6), "day {day}");
        assert_eq!(played.searched(day, Pulse::Afternoon), None, "day {day}");
    }
    assert!(played.rest_days().is_empty(), "{:?}", played.rest_days());
}

#[test]
fn it_gains_4_8_in_the_morning_and_loses_it_in_the_afternoon() {
    let played = play(&text());
    let changes: Vec<(u32, Pulse, Fixed, Fixed)> = played
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::AirGroupFatigue {
                air_group,
                amount,
                fatigue,
                ..
            } if *air_group == group(1) => {
                Some((e.day, e.pulse.expect("a pulse"), *amount, *fatigue))
            }
            _ => None,
        })
        .collect();
    let mut wanted = Vec::new();
    for day in 1..=12 {
        wanted.push((day, Pulse::Morning, fx("4.8"), fx("4.8")));
        // The amount of a loss is given as its size: 4.8, all it had.
        wanted.push((day, Pulse::Afternoon, fx("4.8"), Fixed::ZERO));
    }
    assert_eq!(changes, wanted);
}

#[test]
fn the_search_once_order_says_why_with_the_risk() {
    let played = play(&text());
    let reason = played.handed[&1]
        .orders
        .iter()
        .find(|o| {
            matches!(
                o.kind,
                OrderKind::AirGroup {
                    order: AirGroupOrder::SetSearchOnce { .. },
                    ..
                }
            )
        })
        .map(|o| o.reason.worked.clone())
        .expect("the order");
    assert!(reason.contains("balanced"), "{reason}");
    assert!(reason.contains("once a day"), "{reason}");
    assert!(reason.contains("fatigue 0"), "{reason}");
    assert!(reason.contains("45"), "{reason}");
}

// ----------------------------------------------------------------- Properties

#[test]
fn at_press_the_search_once_order_is_not_given_and_it_searches_in_both_pulses() {
    let played = play(&press_text());
    played.nothing_refused();
    assert_eq!(
        played.air_orders(1),
        vec![(1, mission(AirMission::Search)), (1, arc())]
    );
    let mornings: Vec<Fixed> = [
        "0", "9.6", "19.2", "28.8", "38.4", "48", "57.6", "67.2", "57.2", "66.8", "56.8", "66.4",
    ]
    .iter()
    .map(|s| fx(s))
    .collect();
    assert_eq!(played.mornings(), mornings);
    // Days 1 to 7: searches in both pulses.
    for day in 1..=7 {
        assert!(played.searched(day, Pulse::Morning).is_some(), "day {day}");
        assert!(
            played.searched(day, Pulse::Afternoon).is_some(),
            "day {day}"
        );
    }
    assert_eq!(played.rest_days(), vec![8, 10, 12]);
    assert_eq!(played.search_days(), vec![1, 2, 3, 4, 5, 6, 7, 9, 11]);
    assert_eq!(played.counts(), vec![2, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1]);
    assert_eq!(played.counts().iter().sum::<usize>(), 7);
    assert!(played
        .handed
        .values()
        .flat_map(|d| d.orders.iter())
        .all(|o| !matches!(
            o.kind,
            OrderKind::AirGroup {
                order: AirGroupOrder::SetSearchOnce { .. },
                ..
            }
        )));
}

#[test]
fn without_the_search_figure_a_search_costs_27_and_it_searches_on_days_1_2_3_7_and_10() {
    let played = play_with(&text(), &no_search_figure());
    played.nothing_refused();
    let mornings: Vec<Fixed> = [0, 22, 44, 66, 56, 46, 36, 58, 48, 38, 60, 50]
        .iter()
        .map(|v| n(*v))
        .collect();
    assert_eq!(played.mornings(), mornings);
    assert_eq!(played.search_days(), vec![1, 2, 3, 7, 10]);
    assert_eq!(played.rest_days(), vec![4, 5, 6, 8, 9, 11, 12]);
    for day in 1..=12 {
        assert_eq!(played.searched(day, Pulse::Afternoon), None, "day {day}");
    }
}

/// The scenario with no commands and the group's orders given by the file:
/// the search mission, the arc and search once a day `on`.
fn ordered(on: bool) -> String {
    format!(
        "{}\n[[order]]\nday = 1\nside = \"allied\"\nair_group = 1\nkind = \"set_air_mission\"\nmission = \"search\"\n\n[[order]]\nday = 1\nside = \"allied\"\nair_group = 1\nkind = \"set_search_arc\"\nfrom_deg = 50\nto_deg = 120\nrange_nm = 600\n\n[[order]]\nday = 1\nside = \"allied\"\nair_group = 1\nkind = \"set_search_once\"\non = {on}\n",
        without_commands()
    )
}

#[test]
fn the_rule_alone_on_it_searches_in_the_morning_only_and_off_in_both() {
    let on = play(&ordered(true));
    on.nothing_refused();
    let off = play(&ordered(false));
    off.nothing_refused();
    for day in 1..=12 {
        assert!(on.searched(day, Pulse::Morning).is_some(), "on, day {day}");
        assert_eq!(on.searched(day, Pulse::Afternoon), None, "on, day {day}");
    }
    assert!(off.searched(1, Pulse::Morning).is_some());
    assert!(off.searched(1, Pulse::Afternoon).is_some());
}

#[test]
fn a_strike_sortie_of_1200_nm_costs_27_whether_or_not_the_search_figure_is_given() {
    let with_figure = common::load(&path())
        .expect("the scenario loads")
        .world
        .ref_data()
        .fatigue
        .expect("the fatigue figures");
    let without = common::from_text_with(&text(), &path(), &no_search_figure())
        .expect("the scenario loads")
        .world
        .ref_data()
        .fatigue
        .expect("the fatigue figures");
    assert_eq!(without.search_fatigue_per_100_nm, None);
    assert_eq!(with_figure.gain_for(SortieKind::Strike, n(1200)), n(27));
    assert_eq!(without.gain_for(SortieKind::Strike, n(1200)), n(27));
    assert_eq!(without.gain_for(SortieKind::Search, n(1200)), n(27));
}

/// A group aboard as the carrier decision reads it.
fn aboard(
    id: u32,
    fighter: bool,
    can_search: bool,
    can_strike: bool,
    fatigue: i64,
    able: u32,
) -> CarrierGroup {
    CarrierGroup {
        id: group(id),
        fighter,
        can_search,
        can_strike,
        fatigue: n(fatigue),
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
        search_once: false,
        waters: Default::default(),
    }
}

fn role_of(roles: &[(AirGroupId, CarrierRole)], id: u32) -> CarrierRole {
    roles
        .iter()
        .find(|(g, _)| *g == group(id))
        .map(|(_, r)| *r)
        .expect("a role")
}

#[test]
fn on_a_carrier_the_least_tired_bomber_searches_once_a_day_and_the_other_strikes() {
    let groups = vec![
        aboard(1, true, false, false, 0, 12),
        aboard(3, false, true, true, 10, 10),
        aboard(4, false, true, true, 4, 10),
    ];
    let air = decide(
        Risk::Preserve,
        0,
        false,
        50,
        120,
        &Default::default(),
        &groups,
    );
    assert_eq!(role_of(&air.roles, 4), CarrierRole::Searcher);
    assert_eq!(role_of(&air.roles, 3), CarrierRole::Striker);
    let to_4: Vec<&AirGroupOrder> = air
        .orders
        .iter()
        .filter(|(g, _)| *g == group(4))
        .map(|(_, o)| o)
        .collect();
    assert_eq!(to_4[0], &mission(AirMission::Search));
    assert_eq!(to_4[1], &AirGroupOrder::SetSearchOnce { on: true });
    assert!(air
        .orders
        .contains(&(group(3), mission(AirMission::NavalStrike))));
    // At equal fatigue the lower numbered searches.
    let even = vec![
        aboard(3, false, true, true, 4, 10),
        aboard(4, false, true, true, 4, 10),
    ];
    let r = roles(&even);
    assert_eq!(role_of(&r, 3), CarrierRole::Searcher);
    assert_eq!(role_of(&r, 4), CarrierRole::Striker);
    // A group that cannot fly is passed over.
    let grounded = vec![
        aboard(3, false, true, true, 10, 10),
        aboard(4, false, true, true, 4, 0),
    ];
    let r = roles(&grounded);
    assert_eq!(role_of(&r, 3), CarrierRole::Searcher);
    assert_eq!(role_of(&r, 4), CarrierRole::Striker);
}

fn bomber(id: u32, fatigue: i64, able: u32) -> TurnCandidate {
    TurnCandidate {
        id: group(id),
        fatigue: n(fatigue),
        able,
    }
}

#[test]
fn at_a_base_the_less_tired_bomber_flies_the_covers_arc_and_with_one_it_holds_the_strike() {
    // A cover with no searcher and a strike task with two bomber groups that
    // can search: the less tired takes the search.
    assert_eq!(
        base_turn(false, &[bomber(3, 10, 10), bomber(4, 4, 10)]),
        Some(group(4))
    );
    assert_eq!(
        least_tired(&[bomber(3, 4, 10), bomber(4, 4, 10)]),
        Some(group(3))
    );
    // With one such group, it holds the strike and nobody searches.
    assert_eq!(base_turn(false, &[bomber(3, 10, 10)]), None);
    // Two, one of which cannot fly: one that can fly, so it holds the
    // strike.
    assert_eq!(
        base_turn(false, &[bomber(3, 10, 10), bomber(4, 4, 0)]),
        None
    );
    // With a group at the base that can only search, it searches and both
    // bombers strike.
    assert_eq!(
        base_turn(true, &[bomber(3, 10, 10), bomber(4, 4, 10)]),
        None
    );
}

#[test]
fn search_once_a_day_at_preserve_and_balanced_and_both_pulses_at_press() {
    assert!(once_a_day(Risk::Preserve));
    assert!(once_a_day(Risk::Balanced));
    assert!(!once_a_day(Risk::Press));
}

#[test]
fn an_order_to_set_search_once_on_a_group_that_is_not_the_sides_is_refused() {
    let scenario = common::load(&path()).expect("the scenario loads");
    let request = OrderRequest {
        kind: OrderKind::AirGroup {
            target: group(1),
            order: AirGroupOrder::SetSearchOnce { on: true },
        },
        origin: Origin::PLAYER,
    };
    assert_eq!(
        scenario.world.check_order(Side::Japanese, &request),
        Err(RejectReason::NotYourUnit)
    );
    assert_eq!(scenario.world.check_order(Side::Allied, &request), Ok(()));
}

#[test]
fn the_same_start_gives_the_same_orders_and_reasons() {
    let a = play(&text());
    let b = play(&text());
    assert_eq!(a.handed, b.handed);
    assert_eq!(a.truth.last(), b.truth.last());
}
