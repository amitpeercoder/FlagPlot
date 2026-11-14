//! Scenario 22: an air group is moved to another base by order.
//!
//! Proves decision 0091 (rebasing an air group by order) against a hand
//! calculation. The data is placeholder data: see
//! `data/scenarios/22_rebase.toml`. The test runs on the frozen step 1
//! figures (`data/reference/step1`).
//!
//! # Hand calculation
//!
//! Written by the architect before the code. The tests below assert these
//! numbers exactly. If the code disagrees, the code or the rule is wrong:
//! do not change a number here to make a test pass. A mismatch goes back to
//! the architect.
//!
//! ## Inputs
//!
//! A placeholder map with three Allied bases on one row: base 1 at (0,0),
//! base 2 at (5,0) and base 3 at (13,0). A hex is 40 nm, so base 2 is 200 nm
//! from base 1 and base 3 is 520 nm from base 1 and 320 nm from base 2. No
//! base repairs aircraft. There is no Japanese unit. The game is 3 days.
//!
//! - Air group 1: placeholder flying boat (search range 600 nm), at base 1,
//!   6 ready and 2 damaged, on a search mission that it can fly every day.
//! - Air group 2: placeholder fighter (strike range 250 nm, search range 0),
//!   at base 1, 12 ready.
//!
//! Transfer range is twice the longer of the two ranges: 1,200 nm for the
//! flying boat and 500 nm for the fighter.
//!
//! Orders, all on day 2:
//!
//! - Rebase group 1 to base 2.
//! - Rebase group 2 to base 3.
//! - Rebase group 2 to base 1.
//!
//! ## What happens
//!
//! Day 1. Group 1 flies its search from (0,0) in both daylight pulses. At
//! the end of the day it is at base 1 with 6 ready and 2 damaged.
//!
//! Day 2, at intake:
//!
//! - Group 1 to base 2: 200 nm is within 1,200. Accepted. The group is at
//!   base 2 with 6 ready and 0 damaged. 2 damaged aircraft are written off
//!   at base 1.
//! - Group 2 to base 3: 520 nm is beyond 500. Refused, as beyond its
//!   transfer range.
//! - Group 2 to base 1: refused, as already there.
//!
//! Group 1 flies no search in either daylight pulse of day 2. Group 2 is at
//! base 1 with 12 ready throughout.
//!
//! Day 3. Group 1 flies its search from (5,0) in both daylight pulses.
//!
//! At the end: group 1 at base 2, 6 ready, 0 damaged; group 2 at base 1, 12
//! ready; 2 aircraft written off in all, both of group 1 on day 2.
//!
//! # Properties
//!
//! - Aircraft are conserved: a group's ready and damaged aircraft before a
//!   move equal its ready aircraft after it plus those written off.
//! - The enemy's view is the same in two worlds that differ only in a rebase
//!   order.
//! - A scenario with no rebase order is unchanged, apart from the hash
//!   version.

mod common;

use std::path::PathBuf;

use sim_core::events::{Cause, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, EntityRef};
use sim_core::orders::RejectReason;
use sim_core::state::{AirGroup, AirGroupLocation, Side};
use sim_core::SimConfig;
use sim_data::Scenario;

fn path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/22_rebase.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn load() -> Scenario {
    common::load(&path()).expect("scenario 22 loads")
}

fn from(source: &str) -> Scenario {
    common::from_text(source, &path()).expect("the variant loads")
}

fn run_to(mut scenario: Scenario, day: u32) -> Scenario {
    while scenario.world.day() < day {
        scenario.next_day().expect("the day resolves");
    }
    scenario
}

fn after(day: u32) -> Scenario {
    run_to(load(), day)
}

fn run_text(source: &str) -> Scenario {
    run_to(from(source), 3)
}

fn fx(n: i64) -> Fixed {
    Fixed::from_int(n)
}

fn gid(n: u32) -> AirGroupId {
    AirGroupId::new(n)
}

fn bs(n: u32) -> BaseId {
    BaseId::new(n)
}

fn group(scenario: &Scenario, n: u32) -> &AirGroup {
    &scenario.world.umpire().air_groups[&gid(n)]
}

fn events_of(scenario: &Scenario) -> Vec<Event> {
    scenario.world.events().iter().cloned().collect()
}

fn on_day(scenario: &Scenario, day: u32) -> Vec<Event> {
    events_of(scenario)
        .into_iter()
        .filter(|e| e.day == day)
        .collect()
}

/// The scenario's text without its three day 2 orders.
fn without_rebase_orders() -> String {
    let source = text();
    let at = source
        .find("# Day 2: group 1 to base 2")
        .expect("the day 2 marker");
    source[..at].to_string()
}

/// The scenario's text with the given day 2 rebase orders (group, base) of
/// the Allied side in place of its own.
fn with_rebase_orders(orders: &[(u32, u32)]) -> String {
    let mut source = without_rebase_orders();
    for (group, base) in orders {
        source.push_str(&format!(
            "[[order]]\nday = 2\nside = \"allied\"\nair_group = {group}\nkind = \"rebase\"\nbase = {base}\n\n"
        ));
    }
    source
}

const JAPANESE_BASE: &str =
    "[[base]]\nid = 4\nname = \"Placeholder airfield 4\"\nside = \"japanese\"\nq = 15\nr = 0\n\n";

/// The text with a Japanese base 4 at (15,0) added.
fn with_japanese_base(source: &str) -> String {
    format!("{source}\n{JAPANESE_BASE}")
}

/// The answers to the orders of one day: accepted (event, group) and
/// rejected (event, group, reason).
type Accepted = (Event, AirGroupId);
type Rejected = (Event, AirGroupId, RejectReason);

fn answers(scenario: &Scenario, day: u32) -> (Vec<Accepted>, Vec<Rejected>) {
    let mut accepted = Vec::new();
    let mut rejected = Vec::new();
    for event in on_day(scenario, day) {
        match &event.kind {
            EventKind::OrderAccepted {
                target: EntityRef::AirGroup(g),
                ..
            } => accepted.push((event.clone(), *g)),
            EventKind::OrderRejected {
                target: EntityRef::AirGroup(g),
                reason,
                ..
            } => rejected.push((event.clone(), *g, reason.clone())),
            _ => {}
        }
    }
    (accepted, rejected)
}

/// The reasons of the rejected orders of day 2, with the group each is for.
fn day_two_reasons(source: &str) -> Vec<(u32, RejectReason)> {
    let scenario = run_to(from(source), 2);
    let (_, rejected) = answers(&scenario, 2);
    rejected
        .into_iter()
        .map(|(_, g, reason)| (g.raw(), reason))
        .collect()
}

fn searches(scenario: &Scenario, g: u32, day: u32) -> Vec<(AirGroupLocation, Hex, u32)> {
    on_day(scenario, day)
        .into_iter()
        .filter_map(|e| match e.kind {
            EventKind::SearchFlown {
                air_group,
                from,
                hex,
                aircraft,
                ..
            } if air_group == gid(g) => Some((from, hex, aircraft)),
            _ => None,
        })
        .collect()
}

fn rebased(scenario: &Scenario) -> Vec<Event> {
    events_of(scenario)
        .into_iter()
        .filter(|e| matches!(e.kind, EventKind::AirGroupRebased { .. }))
        .collect()
}

fn written_off(scenario: &Scenario) -> Vec<Event> {
    events_of(scenario)
        .into_iter()
        .filter(|e| matches!(e.kind, EventKind::AircraftWrittenOff { .. }))
        .collect()
}

fn here(scenario: &Scenario, g: u32) -> (AirGroupLocation, u32, u32, Option<u32>) {
    let group = group(scenario, g);
    (
        group.location,
        group.ready,
        group.damaged,
        group.rebased_day,
    )
}

// ------------------------------------------------------------------ Day 1

#[test]
fn day_1_group_1_searches_from_base_1_in_both_daylight_pulses() {
    let scenario = after(1);
    let flown = searches(&scenario, 1, 1);
    assert_eq!(
        flown,
        vec![
            (AirGroupLocation::Base(bs(1)), Hex::new(0, 0), 6),
            (AirGroupLocation::Base(bs(1)), Hex::new(0, 0), 6),
        ]
    );
    assert!(rebased(&scenario).is_empty());
    assert!(written_off(&scenario).is_empty());
}

#[test]
fn day_1_ends_with_group_1_at_base_1_6_ready_2_damaged_and_group_2_12_ready() {
    let scenario = after(1);
    assert_eq!(
        here(&scenario, 1),
        (AirGroupLocation::Base(bs(1)), 6, 2, None)
    );
    assert_eq!(
        here(&scenario, 2),
        (AirGroupLocation::Base(bs(1)), 12, 0, None)
    );
}

// ------------------------------------------------------------------ Day 2

#[test]
fn day_2_the_three_orders_are_answered_as_the_calculation_says() {
    let scenario = after(2);
    let (accepted, rejected) = answers(&scenario, 2);
    assert_eq!(accepted.len(), 1);
    assert_eq!(accepted[0].1, gid(1));
    assert_eq!(rejected.len(), 2);
    assert_eq!(rejected[0].1, gid(2));
    assert_eq!(
        rejected[0].2,
        RejectReason::BeyondTransferRange {
            distance_nm: fx(520),
            range_nm: fx(500)
        }
    );
    assert_eq!(rejected[1].1, gid(2));
    assert_eq!(rejected[1].2, RejectReason::AlreadyAtBase);
}

#[test]
fn day_2_the_refusal_words() {
    assert_eq!(
        RejectReason::BeyondTransferRange {
            distance_nm: fx(520),
            range_nm: fx(500)
        }
        .to_string(),
        "the base is 520 nm away, beyond the air group's transfer range of 500 nm"
    );
    assert_eq!(
        RejectReason::AlreadyAtBase.to_string(),
        "the air group is already at that base"
    );
}

#[test]
fn day_2_group_1_is_at_base_2_with_6_ready_and_0_damaged_and_2_are_written_off() {
    let scenario = after(2);
    assert_eq!(
        here(&scenario, 1),
        (AirGroupLocation::Base(bs(2)), 6, 0, Some(2))
    );
    let off = written_off(&scenario);
    assert_eq!(off.len(), 1);
    assert_eq!(
        off[0].kind,
        EventKind::AircraftWrittenOff {
            air_group: gid(1),
            base: bs(1),
            aircraft: 2
        }
    );
    let moved = rebased(&scenario);
    assert_eq!(moved.len(), 1);
    assert_eq!(
        moved[0].kind,
        EventKind::AirGroupRebased {
            air_group: gid(1),
            from: bs(1),
            to: bs(2),
            distance_nm: fx(200),
            transfer_range_nm: fx(1200),
            ready: 6,
            pilots: None
        }
    );
}

#[test]
fn day_2_both_events_cite_the_order_only_and_follow_its_acceptance() {
    let scenario = after(2);
    let (accepted, _) = answers(&scenario, 2);
    let acceptance = &accepted[0].0;
    let EventKind::OrderAccepted { order, .. } = acceptance.kind else {
        unreachable!()
    };
    let day = on_day(&scenario, 2);
    let at = day
        .iter()
        .position(|e| e.id == acceptance.id)
        .expect("the acceptance is in the log");
    assert!(matches!(
        day[at + 1].kind,
        EventKind::AircraftWrittenOff { .. }
    ));
    assert!(matches!(
        day[at + 2].kind,
        EventKind::AirGroupRebased { .. }
    ));
    assert_eq!(day[at + 1].causes, vec![Cause::Order(order)]);
    assert_eq!(day[at + 2].causes, vec![Cause::Order(order)]);
}

#[test]
fn day_2_group_1_flies_no_search_and_group_2_stays_at_base_1_with_12_ready() {
    let scenario = after(2);
    assert!(searches(&scenario, 1, 2).is_empty());
    assert_eq!(
        here(&scenario, 2),
        (AirGroupLocation::Base(bs(1)), 12, 0, None)
    );
}

#[test]
fn a_refused_order_changes_nothing() {
    let scenario = after(2);
    let group2 = group(&scenario, 2);
    assert_eq!(group2.location, AirGroupLocation::Base(bs(1)));
    assert_eq!(group2.ready, 12);
    assert_eq!(group2.damaged, 0);
    assert_eq!(group2.rebased_day, None);
}

#[test]
fn the_rebased_event_has_the_four_factors_of_its_breakdown() {
    let mut scenario = load();
    scenario.world.set_config(SimConfig {
        record_breakdowns: true,
    });
    let scenario = run_to(scenario, 2);
    let moved = rebased(&scenario);
    let breakdown = moved[0].breakdown.as_ref().expect("a breakdown");
    let named: Vec<(&str, Fixed)> = breakdown
        .factors
        .iter()
        .map(|f| (f.name, f.value))
        .collect();
    assert_eq!(
        named,
        vec![
            ("search_range_nm", fx(600)),
            ("strike_range_nm", fx(0)),
            ("transfer_range_nm", fx(1200)),
            ("distance_nm", fx(200)),
        ]
    );
}

#[test]
fn recording_breakdowns_changes_no_days_hash() {
    let hashes = |on: bool| {
        let mut scenario = load();
        scenario.world.set_config(SimConfig {
            record_breakdowns: on,
        });
        let scenario = run_to(scenario, 3);
        scenario.world.day_hashes().to_vec()
    };
    let with = hashes(true);
    assert_eq!(with.len(), 3);
    assert_eq!(with, hashes(false));
}

// ------------------------------------------------------------------ Day 3

#[test]
fn day_3_group_1_searches_from_base_2_in_both_daylight_pulses() {
    let scenario = after(3);
    assert_eq!(
        searches(&scenario, 1, 3),
        vec![
            (AirGroupLocation::Base(bs(2)), Hex::new(5, 0), 6),
            (AirGroupLocation::Base(bs(2)), Hex::new(5, 0), 6),
        ]
    );
    assert_eq!(
        here(&scenario, 2),
        (AirGroupLocation::Base(bs(1)), 12, 0, None)
    );
}

// ------------------------------------------------------------------ End

#[test]
fn the_end_state_and_two_aircraft_written_off_in_all() {
    let scenario = after(3);
    assert_eq!(
        here(&scenario, 1),
        (AirGroupLocation::Base(bs(2)), 6, 0, Some(2))
    );
    assert_eq!(
        here(&scenario, 2),
        (AirGroupLocation::Base(bs(1)), 12, 0, None)
    );
    let mut total = 0;
    for event in events_of(&scenario) {
        match event.kind {
            EventKind::AircraftWrittenOff {
                air_group,
                aircraft,
                ..
            } => {
                assert_eq!((air_group, event.day), (gid(1), 2));
                total += aircraft;
            }
            EventKind::AircraftWorn { written_off, .. } => total += written_off,
            _ => {}
        }
    }
    assert_eq!(total, 2);
}

// ------------------------------------------------------------- Properties

#[test]
fn property_aircraft_are_conserved_by_a_move() {
    let before = after(1);
    let ready_and_damaged = group(&before, 1).ready + group(&before, 1).damaged;
    let after_move = run_to(before, 2);
    let off: u32 = written_off(&after_move)
        .iter()
        .map(|e| match e.kind {
            EventKind::AircraftWrittenOff { aircraft, .. } => aircraft,
            _ => 0,
        })
        .sum();
    assert_eq!(ready_and_damaged, group(&after_move, 1).ready + off);
}

#[test]
fn property_with_no_damaged_aircraft_nothing_is_written_off_and_the_move_happens() {
    let source = text().replacen("damaged = 2", "damaged = 0", 1);
    assert_ne!(source, text());
    let scenario = run_to(from(&source), 2);
    assert!(written_off(&scenario).is_empty());
    assert_eq!(rebased(&scenario).len(), 1);
    assert_eq!(
        here(&scenario, 1),
        (AirGroupLocation::Base(bs(2)), 6, 0, Some(2))
    );
}

/// The Japanese side's events and contacts as text, from a variant that has
/// a Japanese base and a Japanese task force (so that it has something to
/// see), with `orders` as the Allied day 2 rebase orders.
fn japanese_view(orders: &[(u32, u32)]) -> (Vec<String>, Vec<String>) {
    let mut source = with_japanese_base(&with_rebase_orders(orders));
    source.push_str(
        "[[task_force]]\nid = 1\nname = \"Placeholder Japanese force\"\nside = \"japanese\"\nq = 15\nr = 0\nmission = \"surface_combat\"\n\n\
         [[ship]]\nid = 1\nname = \"Placeholder cruiser 1\"\nclass = 1\nside = \"japanese\"\nfuel_t = 2000\ntask_force = 1\n\n\
         [[order]]\nday = 1\nside = \"japanese\"\ntask_force = 1\nkind = \"set_route\"\nwaypoints = [[15, 2]]\n",
    );
    let scenario = run_text(&source);
    let view = scenario.world.side_view(Side::Japanese);
    let events = view
        .events()
        .into_iter()
        .map(|e| {
            format!(
                "{} D{} {:?} {:?} {:?} {:?}",
                e.number, e.day, e.pulse, e.kind, e.order_number, e.causes
            )
        })
        .collect();
    let contacts = view.contacts().map(|c| format!("{c:?}")).collect();
    (events, contacts)
}

#[test]
fn property_the_enemy_sees_the_same_with_and_without_the_rebase_order() {
    let (with_events, with_contacts) = japanese_view(&[(1, 2)]);
    let (bare_events, bare_contacts) = japanese_view(&[]);
    assert!(!with_events.is_empty());
    assert_eq!(with_events, bare_events);
    assert_eq!(with_contacts, bare_contacts);
}

#[test]
fn property_a_scenario_with_no_rebase_order_is_unchanged() {
    let bare_source = without_rebase_orders();
    let bare = run_text(&bare_source);
    let day_1 = after(1);
    let bare_day_1 = after_text(&bare_source, 1);
    // Day 1 is the same events and the same hash.
    assert_eq!(on_day(&day_1, 1), on_day(&bare_day_1, 1));
    assert_eq!(
        day_1.world.day_hashes()[0],
        bare_day_1.world.day_hashes()[0]
    );
    // Group 1 searches in both daylight pulses of all 3 days from (0,0).
    for day in 1..=3 {
        assert_eq!(
            searches(&bare, 1, day),
            vec![
                (AirGroupLocation::Base(bs(1)), Hex::new(0, 0), 6),
                (AirGroupLocation::Base(bs(1)), Hex::new(0, 0), 6),
            ],
            "day {day}"
        );
    }
    assert!(rebased(&bare).is_empty());
    assert!(written_off(&bare).is_empty());
    for g in bare.world.umpire().air_groups.values() {
        assert_eq!(g.rebased_day, None);
    }
}

fn after_text(source: &str, day: u32) -> Scenario {
    run_to(from(source), day)
}

// A group stood down stays stood down, and its fatigue is untouched.

fn fatigue_variant(rebase: bool) -> String {
    let mut source = if rebase {
        text()
    } else {
        without_rebase_orders()
    };
    source = source.replacen(
        "surface_combat = \"../reference/surface_combat.toml\"",
        "surface_combat = \"../reference/surface_combat.toml\"\nfatigue = \"../reference/fatigue.toml\"",
        1,
    );
    source = source.replacen("damaged = 2", "damaged = 2\nfatigue = 100", 1);
    source.push_str(
        "\n[[order]]\nday = 1\nside = \"allied\"\nair_group = 1\nkind = \"set_rest_threshold\"\nfatigue = 50\n",
    );
    source
}

#[test]
fn a_stood_down_group_stays_stood_down_and_its_fatigue_is_what_rest_alone_gives() {
    let with = after_text(&fatigue_variant(true), 2);
    let without = after_text(&fatigue_variant(false), 2);
    assert_eq!(group(&with, 1).location, AirGroupLocation::Base(bs(2)));
    assert!(group(&with, 1).stood_down);
    assert_eq!(group(&with, 1).stood_down, group(&without, 1).stood_down);
    assert_eq!(group(&with, 1).fatigue, group(&without, 1).fatigue);
    assert!(group(&with, 1).fatigue > fx(50));
}

// ----------------------------------------------------------------- Refusals

#[test]
fn aboard_a_ship_is_refused_and_nothing_changes() {
    let mut source = with_rebase_orders(&[(3, 2)]);
    source.push_str(
        "[[task_force]]\nid = 1\nname = \"Placeholder carrier force\"\nside = \"allied\"\nq = 0\nr = 0\nmission = \"carrier\"\n\n\
         [[ship]]\nid = 1\nname = \"Placeholder carrier 1\"\nclass = 4\nside = \"allied\"\nfuel_t = 3000\ntask_force = 1\n\n\
         [[air_group]]\nid = 3\nname = \"Placeholder scouting squadron\"\nside = \"allied\"\nship = 1\naircraft_type = 2\nready = 8\ndamaged = 1\n",
    );
    let scenario = run_to(from(&source), 2);
    let (accepted, rejected) = answers(&scenario, 2);
    assert!(accepted.is_empty());
    assert_eq!(rejected.len(), 1);
    assert_eq!(rejected[0].2, RejectReason::AirGroupAboardShip);
    assert_eq!(
        RejectReason::AirGroupAboardShip.to_string(),
        "the air group is aboard a ship"
    );
    let aboard = group(&scenario, 3);
    assert!(matches!(aboard.location, AirGroupLocation::Ship(_)));
    assert_eq!(
        (aboard.ready, aboard.damaged, aboard.rebased_day),
        (8, 1, None)
    );
    assert!(written_off(&scenario).is_empty());
    assert!(rebased(&scenario).is_empty());
}

#[test]
fn a_base_of_the_other_side_and_a_base_that_does_not_exist_are_both_not_your_base() {
    let japanese = with_japanese_base(&with_rebase_orders(&[(1, 4)]));
    assert_eq!(
        day_two_reasons(&japanese),
        vec![(1, RejectReason::NotYourBase)]
    );
    let missing = with_rebase_orders(&[(1, 99)]);
    assert_eq!(
        day_two_reasons(&missing),
        vec![(1, RejectReason::NotYourBase)]
    );
    assert_eq!(
        RejectReason::NotYourBase.to_string(),
        "the base is not one of your bases"
    );
    // The group did not move, lose a damaged aircraft or take the day.
    let scenario = run_to(from(&missing), 2);
    assert_eq!(
        here(&scenario, 1),
        (AirGroupLocation::Base(bs(1)), 6, 2, None)
    );
    assert!(written_off(&scenario).is_empty());
    assert!(rebased(&scenario).is_empty());
}

#[test]
fn an_enemy_air_group_is_not_your_unit() {
    let mut source = with_japanese_base(&with_rebase_orders(&[(3, 2)]));
    source.push_str(
        "[[air_group]]\nid = 3\nname = \"Placeholder Japanese fighters\"\nside = \"japanese\"\nbase = 4\naircraft_type = 4\nready = 6\n",
    );
    assert_eq!(
        day_two_reasons(&source),
        vec![(3, RejectReason::NotYourUnit)]
    );
    assert_eq!(
        RejectReason::NotYourUnit.to_string(),
        "the target is not one of your units"
    );
    let scenario = run_to(from(&source), 2);
    assert_eq!(
        here(&scenario, 3),
        (AirGroupLocation::Base(bs(4)), 6, 0, None)
    );
}

#[test]
fn a_second_rebase_order_on_the_day_of_a_move_is_refused() {
    let source = with_rebase_orders(&[(1, 2), (1, 3)]);
    assert_eq!(
        day_two_reasons(&source),
        vec![(1, RejectReason::AlreadyMovedToday)]
    );
    assert_eq!(
        RejectReason::AlreadyMovedToday.to_string(),
        "the air group has already moved to another base today"
    );
    let scenario = run_to(from(&source), 2);
    assert_eq!(
        here(&scenario, 1),
        (AirGroupLocation::Base(bs(2)), 6, 0, Some(2))
    );
    assert_eq!(rebased(&scenario).len(), 1);
}

#[test]
fn beyond_range_and_already_there_in_the_scenario_have_their_words() {
    let scenario = after(2);
    let (_, rejected) = answers(&scenario, 2);
    assert_eq!(
        rejected[0].2.to_string(),
        "the base is 520 nm away, beyond the air group's transfer range of 500 nm"
    );
    assert_eq!(
        rejected[1].2.to_string(),
        "the air group is already at that base"
    );
}
