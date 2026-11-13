//! Scenario 20a: ship repair, forward at a base and in a yard off the map.
//!
//! Decision 0069 (`docs/decisions/0069-ship-repair.md`, its decided text and
//! the section "As planned for the build"). A base may mend a number of
//! points of damage a day; the staff spends them, at the start of each day,
//! on the forces of its side stopped there. A base may be a port for the
//! yard; a force stopped there can be ordered to the yard, leaves the map and
//! comes back on a day worked out from its damage, repaired, refuelled and
//! rearmed. There is no random draw.
//!
//! # Hand calculation
//!
//! Drafted by the team lead, checked number by number and adopted by the
//! coordinating architect on Nov 13, 2026. The tests below assert these
//! numbers. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass; a mismatch goes back to the
//! architect.
//!
//! Quantities are cut to six decimal places at each step. Every quantity in
//! this scenario is exact at two places, so nothing is cut.
//!
//! ## The rule, as worked here
//!
//! **Forward repair**, at the start of each day, after returns from the
//! yard, refuelling and the day's orders. Each base's allowance for the day
//! is its `ship_repair_points_per_day`. Forces of the base's side stopped in
//! its hex (not under way, no `toward_hex`, not away) are served in number
//! order, ships in number order. One ship spends, each step taking the
//! smaller of the allowance left and what she needs:
//!
//! 1. fire damage, down toward 0;
//! 2. flotation damage, down toward 0, only if it is below
//!    `forward_repair_max_flotation`;
//! 3. the first degraded system in system order (propulsion, fire control,
//!    main battery, torpedo mounts, flight deck): it needs
//!    `forward_repair_system_points` less the points already banked on the
//!    ship (`system_repair_points`). If the allowance covers that, the system
//!    is working, the bank is 0, and the next degraded system is taken;
//!    otherwise the allowance left goes into the bank. A system out is passed
//!    over.
//!
//! A ship that takes points records one `ShipRepaired`; one that takes none
//! records nothing.
//!
//! **The yard.** Days of repair of a ship = ceil(2 x flotation + 5 x systems
//! degraded + 30 x systems out); fire adds nothing. Days away of a force =
//! the port's `yard_passage_days` + the largest days of repair of its ships.
//! Day of return = the day the order is taken in + the days away; the force
//! is back at the start of that day, before refuelling and orders. A force
//! whose day of return is after the last day never returns.
//!
//! ## Figures
//!
//! The frozen step 1 figures (`data/reference/step1`), with the five new
//! figures at their first values, the same in the live file:
//!
//! | Figure | Value |
//! | --- | --- |
//! | `forward_repair_max_flotation` | 10 |
//! | `forward_repair_system_points` | 10 |
//! | `yard_days_per_flotation_point` | 2 |
//! | `yard_days_per_system_degraded` | 5 |
//! | `yard_days_per_system_out` | 30 |
//! | `contact_drop_age_pulses` (search, unchanged) | 5 |
//!
//! Ship classes (frozen `ship_classes.csv`): class 1, "Placeholder cruiser":
//! fuel capacity 2,000 t, 8 main guns with 100 rounds a gun (a full load of
//! 800 rounds), 8 torpedo tubes, 8 bearing on one side; it has propulsion,
//! fire control, main battery and torpedo mounts. Class 2, "Placeholder
//! destroyer": 500 t, 4 guns with 150 rounds a gun (600), 8 tubes, 8 on one
//! side; the same four systems. Neither class has a flight deck, and neither
//! ever sets decision 0083's one-side flag.
//!
//! ## The scenario
//!
//! `data/scenarios/20a_ship_repair.toml`, 37 days, seed 1942, on a new
//! placeholder map `data/map/placeholder_repair.toml`: open sea, 40 nm hexes,
//! q from -1 to 10 and r from -1 to 5, with anchorages at (0,0), (4,0),
//! (0,4) and (8,0). The scenario names no fatigue figures, so no force's
//! fatigue changes by the fatigue rule. No base has fuel or supply, so
//! nothing is refuelled in the whole run.
//!
//! | Base | Side | Hex | `ship_repair_points_per_day` | `yard_passage_days` |
//! | --- | --- | --- | --- | --- |
//! | 1, Placeholder repair anchorage | Allied | (0,0) | 6 | none |
//! | 2, Placeholder yard port | Allied | (4,0) | none (0) | 4 |
//! | 3, Placeholder bare anchorage | Allied | (0,4) | none | none |
//! | 4, Placeholder enemy anchorage | Japanese | (8,0) | 6 | 4 |
//!
//! Every task force is Allied, mission transport, stopped at its place at
//! the start, readiness normal unless stated:
//!
//! | Force | At | Ships |
//! | --- | --- | --- |
//! | 1 | base 1 | 1, 2 |
//! | 2 | base 2, readiness battle, fatigue 40 | 3, 4 |
//! | 3 | base 2 | 5 |
//! | 4 | base 2 | 6 |
//! | 5 | base 3 | 7 |
//! | 6 | base 4 | 8 |
//! | 7 | base 1 | 9 |
//!
//! | Ship | Class | Fuel | Guns | Fire | Flotation | Systems not working |
//! | --- | --- | --- | --- | --- | --- | --- |
//! | 1 | cruiser | 2,000 | 800 | 4 | 6 | fire control degraded, main battery out |
//! | 2 | destroyer | 500 | 600 | 2.5 | 9.5 | torpedo mounts degraded |
//! | 3 | cruiser | 1,500 | 500 | 7 | 12.5 | fire control degraded |
//! | 4 | destroyer | 300 | 200 | 6 | 3.25 | torpedo mounts degraded |
//! | 5 | cruiser | 2,000 | 800 | 0 | 0 | propulsion out |
//! | 6 | destroyer | 500 | 600 | 3 | 0 | none |
//! | 7 | destroyer | 500 | 600 | 3 | 2 | none |
//! | 8 | destroyer | 500 | 600 | 3 | 2 | none |
//! | 9 | cruiser | 2,000 | 800 | 1 | 10 | propulsion degraded |
//!
//! The Japanese side has no task force. It starts holding one contact at
//! (4,0), two ships, cruiser, age 0.
//!
//! Orders (Allied), taken in at the start of their day:
//!
//! | Day | Force | Order |
//! | --- | --- | --- |
//! | 1 | 1 | `send_to_yard` |
//! | 1 | 4 | `set_speed` cruise, `set_route` to (2,2), then `send_to_yard` |
//! | 1 | 6 | `send_to_yard` |
//! | 2 | 2 | `send_to_yard` |
//! | 3 | 2 | `send_to_yard` |
//! | 3 | 4 | `send_to_yard` |
//! | 4 | 3 | `send_to_yard` |
//!
//! ## Case A: the orders and their answers
//!
//! | Day | Force | Answer | Why |
//! | --- | --- | --- | --- |
//! | 1 | 1 | refused, `NotAYardPort` | stopped at base 1, its own, which is not a port for the yard |
//! | 1 | 4 | speed and route accepted; yard refused, `UnderWay` | the route, taken in before it, put it under way |
//! | 1 | 6 | refused, `NoOwnBaseHere` | base 4 is Japanese |
//! | 2 | 2 | accepted; `ForceLeftForYard` | below |
//! | 3 | 2 | refused, `AwayAtYard` | away until day 36 |
//! | 3 | 4 | refused, `NoOwnBaseHere` | it arrived at (2,2), two hexes away, on day 1; no base there |
//! | 4 | 3 | accepted; `ForceLeftForYard` | below |
//!
//! A refused order changes nothing: force 1 is still stopped at base 1 and
//! is repaired on day 1 (case B).
//!
//! ## Case B: forward repair at base 1, 6 points a day
//!
//! Base 1 serves force 1 (ships 1 and 2) and then force 7 (ship 9). L is the
//! allowance left.
//!
//! | Day | Ship | Spent on | Points | After | L after |
//! | --- | --- | --- | --- | --- | --- |
//! | 1 | 1 | fire 4 to 0; flotation 6 < 10, 6 to 4 | 4 + 2 = 6 | fire 0, flotation 4 | 0 |
//! | 2 | 1 | flotation 4 to 0; fire control needs 10, banks 2 | 4 + 2 = 6 | flotation 0, bank 2 | 0 |
//! | 3 | 1 | fire control needs 10 - 2 = 8, banks 6 | 6 | bank 8 | 0 |
//! | 4 | 1 | fire control needs 2: working; main battery out, passed over | 2 | fire control working, bank 0 | 4 |
//! | 4 | 2 | fire 2.5 to 0; flotation 9.5 < 10, 9.5 to 8 | 2.5 + 1.5 = 4 | fire 0, flotation 8 | 0 |
//! | 5 | 2 | flotation 8 to 2 (ship 1 takes nothing and records nothing) | 6 | flotation 2 | 0 |
//! | 6 | 2 | flotation 2 to 0; torpedo mounts need 10, banks 4 | 2 + 4 = 6 | flotation 0, bank 4 | 0 |
//! | 7 | 2 | torpedo mounts need 10 - 4 = 6: working | 6 | torpedo mounts working, bank 0 | 0 |
//! | 8 | 9 | fire 1 to 0; flotation 10 is not below 10, untouched; propulsion needs 10, banks 5 | 1 + 5 = 6 | fire 0, bank 5 | 0 |
//! | 9 | 9 | propulsion needs 10 - 5 = 5: working | 5 | propulsion working, bank 0 | 1 |
//!
//! From day 10 nothing at base 1 takes a point and nothing is recorded:
//! ship 1's main battery is out and ship 9's flotation is 10.
//!
//! Points spent: 6 a day on days 1 to 8, 5 on day 9: 53. By ship: ship 1,
//! 4 + 6 + 10 = 20; ship 2, 2.5 + 9.5 + 10 = 22; ship 9, 1 + 10 = 11;
//! 20 + 22 + 11 = 53.
//!
//! `ShipRepaired` events, in order: day 1 ship 1; day 2 ship 1; day 3 ship 1;
//! day 4 ship 1 then ship 2; day 5 ship 2; day 6 ship 2; day 7 ship 2; day 8
//! ship 9; day 9 ship 9. Ten events. Day 4's shows the day's points shared
//! in ship order; days 8 and 9 show force 7, numbered after force 1, served
//! only once force 1 needs nothing. Each names its force and base 1, and
//! cites nothing (the forces began the scenario there and have no arrival
//! event). Day 8's breakdown names `forward_repair_max_flotation` 10.
//!
//! At the end: ship 1 fire 0, flotation 0, fire control working, main
//! battery out, bank 0. Ship 2 all 0, all working. Ship 9 fire 0, flotation
//! 10, all working, bank 0.
//!
//! ## Case C: no repair where it does not apply
//!
//! - Ship 6 (force 4): at base 2 on day 1, whose figure is 0; then at
//!   (2,2), where there is no base. Fire 3 to the end.
//! - Ship 7 (force 5): stopped at base 3, Allied, with no figure. Fire 3,
//!   flotation 2 to the end.
//! - Ship 8 (force 6): stopped at base 4, which mends 6 a day for the
//!   Japanese side only. Fire 3, flotation 2 to the end.
//! - Ships 3, 4 and 5: at base 2 (figure 0) until they leave; nothing while
//!   away.
//!
//! None of them records a `ShipRepaired`.
//!
//! ## Case D: force 2 to the yard and back
//!
//! On day 2, at the order's acceptance:
//!
//! | Ship | Flotation | Degraded | Out | Days of repair |
//! | --- | --- | --- | --- | --- |
//! | 3 | 12.5 | 1 | 0 | ceil(2 x 12.5 + 5 x 1) = ceil(30) = 30 |
//! | 4 | 3.25 | 1 | 0 | ceil(2 x 3.25 + 5 x 1) = ceil(11.5) = 12 |
//!
//! Ship 3's fire of 7 and ship 4's of 6 add nothing. Days away: 4 + 30 = 34.
//! Day of return: 2 + 34 = 36. `ForceLeftForYard`: force 2, base 2, passage
//! 4, ships [(3, 30), (4, 12)], days away 34, day of return 36; caused by
//! the order. Its breakdown: passage 4; ship 3 flotation 12.5, degraded 1,
//! out 0, days 30; ship 4 flotation 3.25, degraded 1, out 0, days 12; days
//! away 34.
//!
//! From then until the start of day 36: ships 3 and 4 are at the yard
//! (location `Yard(2)`), force 2 is at (4,0), stopped, with no ships, its
//! fatigue still 40 and its readiness battle; its ships keep the damage,
//! fuel and guns they left with.
//!
//! **What `measure` says**, asked `yard` of force 2 after day 1 resolves:
//! not refused; port base 2, passage 4; ship 3 flotation 12.5, degraded 1,
//! out 0, 30 days; ship 4 flotation 3.25, degraded 1, out 0, 12 days; days
//! away 34; leaves day 2; returns day 36; returns before the end, the last
//! day being 37. The same numbers as the event of day 2.
//!
//! **Orders the evening before.** `World::check_order` for a `set_speed`
//! to force 2: after day 34 resolves (the order's day would be 35, before
//! its return) `Err(AwayAtYard)`; after day 35 resolves (day 36, its return)
//! `Ok`.
//!
//! **At the start of day 36**, first of the day's steps,
//! `ForceReturnedFromYard`: force 2, base 2, ships [3, 4], caused by the
//! departure event of day 2. Afterwards:
//!
//! | | Ship 3 | Ship 4 |
//! | --- | --- | --- |
//! | Fire, flotation | 0, 0 | 0, 0 |
//! | Systems | all working | all working |
//! | Bank | 0 | 0 |
//! | Fuel | 1,500 to 2,000 | 300 to 500 |
//! | Guns | 500 to 800 | 200 to 600 |
//! | Torpedoes ready | true | true |
//! | One-side flag | false | false |
//! | Location | force 2 | force 2 |
//!
//! Force 2: ships {3, 4}, at (4,0), idle, no route, fatigue 40 to 0, not
//! alerted, readiness battle (kept), arrival event the return. Base 2 has
//! no fuel and no repair figure, so nothing else is recorded for it that
//! day or the next.
//!
//! ## Case E: force 3 does not come back
//!
//! On day 4: ship 5, flotation 0, degraded 0, out 1: ceil(30) = 30 days.
//! Days away 4 + 30 = 34; day of return 4 + 34 = 38, after day 37.
//! `ForceLeftForYard`: force 3, base 2, passage 4, ships [(5, 30)], days 34,
//! day of return 38. `measure` asked after day 3 says returns day 38, not
//! before the end. No `ForceReturnedFromYard` is ever recorded for it. At
//! the end ship 5 is afloat at the yard (`Yard(3)`) with propulsion out.
//!
//! ## Case F: what the Japanese side sees
//!
//! Nothing of the rule. Its contact at (4,0) is never refreshed: it ages at
//! the end of each pulse (1 after day 1's night, 2, 3, 4) and is dropped at
//! the end of day 2's morning, at age 5. Its side events are those four
//! `ContactAged` and one `ContactDropped`, and no event that names any
//! Allied unit; they are the same event for event as in the variant with
//! no Allied orders at all. Its briefings have no line on repair or the
//! yard.
//!
//! ## Variants
//!
//! | Variant | What changes |
//! | --- | --- |
//! | Base 1's figure is 0 | No `ShipRepaired` at all; ships 1, 2 and 9 end as they began |
//! | `forward_repair_max_flotation` 9 (an altered copy of the frozen figures) | Days 1 to 3 and ship 1 on day 4 as case B. Day 4, ship 2: fire 2.5 to 0, flotation 9.5 is not below 9, torpedo mounts bank 1.5 (L 0). Day 5: ship 2 needs 8.5, banks 6: bank 7.5. Day 6: ship 2 needs 2.5: working, L 3.5; ship 9: fire 1 to 0 (L 2.5), flotation untouched, propulsion banks 2.5. Day 7: ship 9 needs 7.5, banks 6: bank 8.5. Day 8: ship 9 needs 1.5: working, L 4.5. Nothing after. Ship 2 ends with flotation 9.5, ship 9 with 10. Points: 6 x 7 + 1.5 = 43.5 = ship 1's 20 + ship 2's 2.5 + 10 + ship 9's 1 + 10 |
//! | Force 2's order is given on day 1 | Same days, 34; returns day 35 |
//! | Base 2's `yard_passage_days` is 0 | The scenario is refused when loaded |
//!
//! ## Unit tests (where the scenario cannot reach)
//!
//! - A ship of the wing-tubes class with `torpedoes_ready` true and the
//!   one-side flag set returns with ready true and the flag false; one with
//!   torpedoes spent returns with them ready.
//! - Two bases of a side in one hex, both with a figure: a ship takes from
//!   the lower-numbered until its allowance is spent, and the next ship from
//!   the other.
//! - A degraded system hit to out while points are banked: the bank goes to
//!   the next degraded system.
//! - The same with no other degraded system: the bank is kept until forward
//!   repair next reaches step 3, which finds no degraded system, sets it to 0
//!   and spends nothing for it.
//! - Decision 0085's projection: a force that lost a ship in a fight that
//!   ended force-sunk by the enemy's loss, and then leaves for the yard, is
//!   still told its own true reason for that fight, not "force sunk".
//! - The objectives tally: a ship at the yard at the end with propulsion
//!   out counts as crippled (in this scenario, ship 5).
//! - A carrier with an air group aboard sent to the yard: the group flies
//!   nothing while away and returns with its aircraft, damaged aircraft and
//!   pilots as they were (pending the architect's choice).
//!
//! ## Properties
//!
//! - No draw: over seeds 1 to 50 the event log (every event's day, pulse,
//!   kind, data and causes) is the same; only the state hashes differ, by
//!   the seed. `rng::Draw::ALL` is unchanged.
//! - Determinism: one seed run twice gives the same hash every day.
//! - No damage track ever rises by this rule; no system out is ever mended
//!   forward; a bank is always from 0 to below 10.
//! - What must not move: with no base given a repair figure or a yard
//!   passage, every other scenario, on the frozen and the live figures,
//!   gives its event log byte for byte as before
//!   (`crates/sim-cli/tests/run_baseline/*.events.txt` unchanged); only the
//!   state hashes move, by the new fields and state version 9.
//!
//! This rule has no chance, so the three parts of `docs/step2-spec.md`,
//! "Tests while figures move", 7, reduce to the hand calculation above and
//! the properties: there is no chance to hand-calculate at a working value
//! and no seed to pin.

// The test code is written by a developer from the hand calculation above.
//
// The "Unit tests" above are in sim-core, where the scenario cannot reach:
// `crates/sim-core/src/repair/tests.rs` (the wing-tubes return, two bases in
// one hex, the bank handed on and kept, the projection of decision 0085),
// `crates/sim-core/src/side_events.rs` (who sees which repair event) and
// `crates/sim-core/src/objectives.rs` (a ship at the yard counts as crippled).
// The `measure` of `yard` is the staff tool's, tested with it in `sim-agent`.
// The air group at the yard awaits the architect's choice and is not tested.

mod common;

use std::collections::BTreeSet;
use std::path::PathBuf;

use sim_core::events::{Cause, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, EntityRef, ShipId, TaskForceId};
use sim_core::objectives::tallies;
use sim_core::orders::{
    OrderKind, OrderRequest, Origin, Readiness, RejectReason, SpeedSetting, TaskForceOrder,
};
use sim_core::refdata::ShipKind;
use sim_core::side_events::SideEventKind;
use sim_core::state::{Ship, ShipLocation, ShipSystem, Side, SystemState, TaskForceStatus};
use sim_data::Scenario;

fn path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/20a_ship_repair.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn load() -> Scenario {
    common::load(&path()).expect("scenario 20a loads")
}

fn run() -> Scenario {
    let mut scenario = load();
    scenario.run_to_end().expect("the scenario runs");
    scenario
}

/// The scenario run through `day` days.
fn after(day: u32) -> Scenario {
    let mut scenario = load();
    while scenario.world.day() < day {
        scenario.next_day().expect("the day resolves");
    }
    scenario
}

/// The scenario's text edited. Panics if the edit changes nothing.
fn edited(edit: impl Fn(&str) -> String) -> String {
    let original = text();
    let changed = edit(&original);
    assert_ne!(original, changed, "the edit changed nothing");
    changed
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

fn sh(n: u32) -> ShipId {
    ShipId::new(n)
}

fn tf(n: u32) -> TaskForceId {
    TaskForceId::new(n)
}

fn bs(n: u32) -> BaseId {
    BaseId::new(n)
}

fn events_of(scenario: &Scenario) -> Vec<Event> {
    scenario.world.events().iter().cloned().collect()
}

/// One `ShipRepaired` event: day, ship, force, base, points, fire,
/// flotation, systems restored, bank.
type Repaired = (
    u32,
    ShipId,
    TaskForceId,
    BaseId,
    Fixed,
    Fixed,
    Fixed,
    Vec<ShipSystem>,
    Fixed,
);

fn repaired(scenario: &Scenario) -> Vec<Repaired> {
    events_of(scenario)
        .into_iter()
        .filter_map(|event| match event.kind {
            EventKind::ShipRepaired {
                ship,
                task_force,
                base,
                points,
                fire_damage,
                flotation_damage,
                restored,
                system_repair_points,
            } => Some((
                event.day,
                ship,
                task_force,
                base,
                points,
                fire_damage,
                flotation_damage,
                restored,
                system_repair_points,
            )),
            _ => None,
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn row(
    day: u32,
    ship: u32,
    force: u32,
    points: &str,
    fire: &str,
    flotation: &str,
    restored: &[ShipSystem],
    bank: &str,
) -> Repaired {
    (
        day,
        sh(ship),
        tf(force),
        bs(1),
        fx(points),
        fx(fire),
        fx(flotation),
        restored.to_vec(),
        fx(bank),
    )
}

fn ship(scenario: &Scenario, n: u32) -> &Ship {
    &scenario.world.umpire().ships[&sh(n)]
}

fn all_working(ship: &Ship) -> bool {
    ShipSystem::ALL
        .iter()
        .all(|system| ship.systems.get(*system) == SystemState::Working)
}

fn case_b_rows() -> Vec<Repaired> {
    use ShipSystem::*;
    vec![
        row(1, 1, 1, "6", "0", "4", &[], "0"),
        row(2, 1, 1, "6", "0", "0", &[], "2"),
        row(3, 1, 1, "6", "0", "0", &[], "8"),
        row(4, 1, 1, "2", "0", "0", &[FireControl], "0"),
        row(4, 2, 1, "4", "0", "8", &[], "0"),
        row(5, 2, 1, "6", "0", "2", &[], "0"),
        row(6, 2, 1, "6", "0", "0", &[], "4"),
        row(7, 2, 1, "6", "0", "0", &[TorpedoMounts], "0"),
        row(8, 9, 7, "6", "0", "10", &[], "5"),
        row(9, 9, 7, "5", "0", "10", &[Propulsion], "0"),
    ]
}

// ---------------------------------------------------------------- Case A

#[test]
fn case_a_each_order_is_answered_as_the_table_says() {
    let scenario = run();
    let mut accepted = Vec::new();
    let mut rejected = Vec::new();
    for event in events_of(&scenario) {
        match event.kind {
            EventKind::OrderAccepted { target, .. } => accepted.push((event.day, target)),
            EventKind::OrderRejected { target, reason, .. } => {
                rejected.push((event.day, target, reason))
            }
            _ => {}
        }
    }
    let force = EntityRef::TaskForce;
    // Force 4's speed and route on day 1, then the yard order of force 2 on
    // day 2 and of force 3 on day 4.
    assert_eq!(
        accepted,
        vec![
            (1, force(tf(4))),
            (1, force(tf(4))),
            (2, force(tf(2))),
            (4, force(tf(3))),
        ]
    );
    assert_eq!(
        rejected,
        vec![
            (1, force(tf(1)), RejectReason::NotAYardPort),
            (1, force(tf(4)), RejectReason::UnderWay),
            (1, force(tf(6)), RejectReason::NoOwnBaseHere),
            (3, force(tf(2)), RejectReason::AwayAtYard),
            (3, force(tf(4)), RejectReason::NoOwnBaseHere),
        ]
    );
}

#[test]
fn case_a_a_refused_order_changes_nothing() {
    // Force 1 stays stopped at base 1 and is repaired on day 1.
    let scenario = after(1);
    let force = &scenario.world.umpire().task_forces[&tf(1)];
    assert_eq!(force.position(), Hex::new(0, 0));
    assert_eq!(force.status, TaskForceStatus::Idle);
    assert_eq!(force.ships, BTreeSet::from([sh(1), sh(2)]));
    assert_eq!(repaired(&scenario).len(), 1);
    // Force 4 is at (2,2), two hexes from base 2, and not at the yard.
    let force4 = &scenario.world.umpire().task_forces[&tf(4)];
    assert_eq!(force4.position(), Hex::new(2, 2));
    assert_eq!(ship(&scenario, 6).location, ShipLocation::TaskForce(tf(4)));
}

// ---------------------------------------------------------------- Case B

#[test]
fn case_b_the_ten_ship_repaired_events_are_exactly_the_table() {
    let scenario = run();
    assert_eq!(repaired(&scenario), case_b_rows());
}

#[test]
fn case_b_no_event_has_a_cause_and_day_eight_names_the_flotation_limit() {
    let scenario = run();
    let mut count = 0;
    for event in events_of(&scenario) {
        if matches!(event.kind, EventKind::ShipRepaired { .. }) {
            count += 1;
            assert!(event.causes.is_empty(), "event {:?} has causes", event.id);
            let breakdown = event.breakdown.as_ref().expect("a breakdown");
            let limit = breakdown
                .factors
                .iter()
                .find(|factor| factor.name == "forward_repair_max_flotation");
            if event.day == 8 {
                assert_eq!(limit.expect("day 8 names the limit").value, fx("10"));
            }
        }
    }
    assert_eq!(count, 10);
}

#[test]
fn case_b_the_total_is_53_and_the_end_state_is_as_the_table_says() {
    let scenario = run();
    let mut total = Fixed::ZERO;
    let mut by_ship = std::collections::BTreeMap::new();
    for (_, ship, _, _, points, ..) in repaired(&scenario) {
        total += points;
        *by_ship.entry(ship).or_insert(Fixed::ZERO) += points;
    }
    assert_eq!(total, fx("53"));
    assert_eq!(by_ship[&sh(1)], fx("20"));
    assert_eq!(by_ship[&sh(2)], fx("22"));
    assert_eq!(by_ship[&sh(9)], fx("11"));

    let one = ship(&scenario, 1);
    assert_eq!((one.fire_damage, one.flotation_damage), (fx("0"), fx("0")));
    assert_eq!(
        one.systems.get(ShipSystem::FireControl),
        SystemState::Working
    );
    assert_eq!(one.systems.get(ShipSystem::MainBattery), SystemState::Out);
    assert_eq!(one.system_repair_points, fx("0"));

    let two = ship(&scenario, 2);
    assert_eq!((two.fire_damage, two.flotation_damage), (fx("0"), fx("0")));
    assert!(all_working(two));
    assert_eq!(two.system_repair_points, fx("0"));

    let nine = ship(&scenario, 9);
    assert_eq!(
        (nine.fire_damage, nine.flotation_damage),
        (fx("0"), fx("10"))
    );
    assert!(all_working(nine));
    assert_eq!(nine.system_repair_points, fx("0"));
}

// ---------------------------------------------------------------- Case C

#[test]
fn case_c_nothing_is_repaired_where_it_does_not_apply() {
    let scenario = run();
    let touched: BTreeSet<ShipId> = repaired(&scenario)
        .into_iter()
        .map(|(_, ship, ..)| ship)
        .collect();
    assert_eq!(touched, BTreeSet::from([sh(1), sh(2), sh(9)]));
    for n in 6..=8 {
        assert_eq!(ship(&scenario, n).fire_damage, fx("3"), "ship {n}");
    }
    assert_eq!(ship(&scenario, 6).flotation_damage, fx("0"));
    assert_eq!(ship(&scenario, 7).flotation_damage, fx("2"));
    assert_eq!(ship(&scenario, 8).flotation_damage, fx("2"));
    // Ship 6 left base 2 for (2,2) and stayed there.
    assert_eq!(ship(&scenario, 6).location, ShipLocation::TaskForce(tf(4)));
    // Ships 3 and 4 were mended in the yard, not forward.
    assert!(all_working(ship(&scenario, 3)));
    assert!(all_working(ship(&scenario, 4)));
}

// ---------------------------------------------------------------- Case D

fn left_for_yard(scenario: &Scenario, force: u32) -> Option<Event> {
    events_of(scenario).into_iter().find(|event| {
        matches!(&event.kind, EventKind::ForceLeftForYard { task_force, .. } if *task_force == tf(force))
    })
}

#[test]
fn case_d_force_two_leaves_for_the_yard_with_the_stated_fields_and_breakdown() {
    let scenario = run();
    let left = left_for_yard(&scenario, 2).expect("force 2 leaves");
    assert_eq!(left.day, 2);
    assert_eq!(
        left.kind,
        EventKind::ForceLeftForYard {
            task_force: tf(2),
            base: bs(2),
            passage_days: 4,
            ships: vec![(sh(3), 30), (sh(4), 12)],
            days_away: 34,
            return_day: 36,
        }
    );
    // Caused by the order, the one accepted on day 2 for force 2.
    let order = events_of(&scenario)
        .into_iter()
        .find_map(|event| match event.kind {
            EventKind::OrderAccepted {
                order,
                target: EntityRef::TaskForce(target),
            } if event.day == 2 && target == tf(2) => Some(order),
            _ => None,
        })
        .expect("the order is accepted");
    assert_eq!(left.causes, vec![Cause::Order(order)]);

    let breakdown = left.breakdown.expect("a breakdown");
    let named: Vec<(&str, Fixed)> = breakdown
        .factors
        .iter()
        .map(|factor| (factor.name, factor.value))
        .collect();
    assert_eq!(
        named,
        vec![
            ("passage_days", fx("4")),
            ("flotation_damage", fx("12.5")),
            ("systems_degraded", fx("1")),
            ("systems_out", fx("0")),
            ("days_of_repair", fx("30")),
            ("flotation_damage", fx("3.25")),
            ("systems_degraded", fx("1")),
            ("systems_out", fx("0")),
            ("days_of_repair", fx("12")),
            ("days_away", fx("34")),
        ]
    );
    assert_eq!(breakdown.result, fx("34"));
}

#[test]
fn case_d_the_yard_plan_before_the_order_reads_as_the_measure_does() {
    // After day 1 resolves, the plan the order would make.
    let scenario = after(1);
    let plan = scenario.world.yard_plan(tf(2)).expect("a plan");
    assert_eq!(plan.base, bs(2));
    assert_eq!(plan.passage_days, 4);
    assert_eq!(plan.ships, vec![(sh(3), 30), (sh(4), 12)]);
    assert_eq!(plan.days_away, 34);
    assert_eq!(plan.return_day(2), 36);
}

#[test]
fn case_d_while_away_the_ships_keep_what_they_left_with() {
    let mut scenario = after(2);
    let order = OrderRequest {
        kind: OrderKind::TaskForce {
            target: tf(2),
            order: TaskForceOrder::SetSpeed(SpeedSetting::Cruise),
        },
        origin: Origin::PLAYER,
    };
    for day in 2..=35 {
        assert_eq!(scenario.world.day(), day);
        let truth = scenario.world.umpire();
        let force = &truth.task_forces[&tf(2)];
        assert!(force.ships.is_empty(), "day {day}");
        assert_eq!(force.position(), Hex::new(4, 0));
        assert_eq!(force.status, TaskForceStatus::Idle);
        assert_eq!(force.fatigue, fx("40"));
        assert_eq!(force.readiness, Readiness::Battle);
        assert!(force.toward_hex.is_none());
        assert!(!force.alerted);
        for n in [3, 4] {
            assert_eq!(ship(&scenario, n).location, ShipLocation::Yard(tf(2)));
        }
        let (three, four) = (ship(&scenario, 3), ship(&scenario, 4));
        assert_eq!(
            (three.fire_damage, three.flotation_damage),
            (fx("7"), fx("12.5"))
        );
        assert_eq!(
            (four.fire_damage, four.flotation_damage),
            (fx("6"), fx("3.25"))
        );
        assert_eq!(
            three.systems.get(ShipSystem::FireControl),
            SystemState::Degraded
        );
        assert_eq!(
            four.systems.get(ShipSystem::TorpedoMounts),
            SystemState::Degraded
        );
        assert_eq!((three.fuel_t, three.gun_rounds), (fx("1500"), fx("500")));
        assert_eq!((four.fuel_t, four.gun_rounds), (fx("300"), fx("200")));
        // No yard return before day 36.
        assert!(!events_of(&scenario)
            .iter()
            .any(|e| matches!(e.kind, EventKind::ForceReturnedFromYard { .. })));
        if day == 34 {
            // The order's day would be 35, before the return.
            assert_eq!(
                scenario.world.check_order(Side::Allied, &order),
                Err(RejectReason::AwayAtYard)
            );
        }
        if day == 35 {
            // The order's day would be 36, the day of return.
            assert_eq!(scenario.world.check_order(Side::Allied, &order), Ok(()));
        }
        if day < 35 {
            scenario.next_day().expect("the day resolves");
        }
    }
}

#[test]
fn case_d_force_two_is_back_at_the_start_of_day_36() {
    let scenario = run();
    let events = events_of(&scenario);
    let left = left_for_yard(&scenario, 2).expect("departure");
    let returned: Vec<&Event> = events
        .iter()
        .filter(|e| matches!(e.kind, EventKind::ForceReturnedFromYard { .. }))
        .collect();
    assert_eq!(returned.len(), 1, "only force 2 returns");
    let back = returned[0];
    assert_eq!(back.day, 36);
    assert_eq!(
        back.kind,
        EventKind::ForceReturnedFromYard {
            task_force: tf(2),
            base: bs(2),
            ships: vec![sh(3), sh(4)],
        }
    );
    assert_eq!(back.causes, vec![Cause::Event(left.id)]);
    // The first of the day's steps.
    let first_of_day = events
        .iter()
        .find(|e| e.day == 36)
        .expect("day 36 has events");
    assert_eq!(first_of_day.id, back.id);

    for (n, fuel, guns) in [(3, "2000", "800"), (4, "500", "600")] {
        let mended = ship(&scenario, n);
        assert_eq!(
            (mended.fire_damage, mended.flotation_damage),
            (fx("0"), fx("0"))
        );
        assert!(all_working(mended));
        assert_eq!(mended.system_repair_points, fx("0"));
        assert_eq!((mended.fuel_t, mended.gun_rounds), (fx(fuel), fx(guns)));
        assert!(mended.torpedoes_ready);
        assert!(!mended.torpedoes_one_side_fired);
        assert_eq!(mended.location, ShipLocation::TaskForce(tf(2)));
    }
    let force = &scenario.world.umpire().task_forces[&tf(2)];
    assert_eq!(force.ships, BTreeSet::from([sh(3), sh(4)]));
    assert_eq!(force.position(), Hex::new(4, 0));
    assert_eq!(force.status, TaskForceStatus::Idle);
    assert!(force.route.waypoints.is_empty());
    assert_eq!(force.fatigue, fx("0"));
    assert_eq!(force.readiness, Readiness::Battle);
    assert_eq!(force.arrival_event, Some(back.id));
    assert!(force.yard_trip.is_none());
}

// ---------------------------------------------------------------- Case E

#[test]
fn case_e_force_three_goes_and_does_not_come_back() {
    let scenario = run();
    let left = left_for_yard(&scenario, 3).expect("force 3 leaves");
    assert_eq!(left.day, 4);
    assert_eq!(
        left.kind,
        EventKind::ForceLeftForYard {
            task_force: tf(3),
            base: bs(2),
            passage_days: 4,
            ships: vec![(sh(5), 30)],
            days_away: 34,
            return_day: 38,
        }
    );
    assert!(!events_of(&scenario).iter().any(|e| matches!(
        e.kind,
        EventKind::ForceReturnedFromYard { task_force, .. } if task_force == tf(3)
    )));
    let five = ship(&scenario, 5);
    assert_eq!(five.location, ShipLocation::Yard(tf(3)));
    assert_eq!(five.systems.get(ShipSystem::Propulsion), SystemState::Out);
}

#[test]
fn case_e_the_tally_reads_the_ship_at_the_yard_as_afloat_and_crippled() {
    let scenario = run();
    let world = &scenario.world;
    let tallies = tallies(world.umpire(), world.events(), world.ref_data());
    // A side's tally holds the enemy's losses: ship 5 is Allied.
    let japanese = &tallies[&Side::Japanese];
    assert_eq!(japanese.sunk.values().sum::<u32>(), 0);
    assert_eq!(japanese.crippled.get(&ShipKind::Cruiser), Some(&1));
    assert_eq!(japanese.crippled.values().sum::<u32>(), 1);
    assert_eq!(tallies[&Side::Allied].crippled.values().sum::<u32>(), 0);
}

// ---------------------------------------------------------------- Case F

/// Every side event the Japanese side may see, as text: its own number, day,
/// pulse, kind, order number and causes.
fn japanese_side_events(scenario: &Scenario) -> Vec<String> {
    scenario
        .world
        .side_view(Side::Japanese)
        .events()
        .into_iter()
        .map(|e| {
            format!(
                "{} D{} {:?} {:?} {:?} {:?}",
                e.number, e.day, e.pulse, e.kind, e.order_number, e.causes
            )
        })
        .collect()
}

/// The scenario with every order taken out (everything after the marker).
fn without_allied_orders() -> Scenario {
    let source = edited(|t| {
        let at = t.find("\n# Orders (case A).").expect("the orders marker");
        t[..at].to_string()
    });
    common::from_text(&source, &path()).expect("the variant loads")
}

#[test]
fn case_f_the_japanese_side_sees_only_its_contact_age_and_drop() {
    let scenario = run();
    let view = scenario.world.side_view(Side::Japanese);
    let mut aged = Vec::new();
    let mut dropped = Vec::new();
    for event in view.events() {
        let SideEventKind::Whole(kind) = &event.kind else {
            continue;
        };
        match kind {
            EventKind::ContactAged { age_pulses, .. } => aged.push((event.day, *age_pulses)),
            EventKind::ContactDropped { .. } => dropped.push(event.day),
            _ => {}
        }
        // Nothing names a unit of the other side.
        for entity in kind.entities() {
            assert!(
                matches!(entity, EntityRef::Contact(_) | EntityRef::Base(_)),
                "{kind:?} names {entity:?}"
            );
        }
    }
    assert_eq!(aged, vec![(1, 1), (1, 2), (1, 3), (2, 4)]);
    assert_eq!(dropped, vec![2]);
}

#[test]
fn case_f_the_japanese_side_sees_the_same_as_with_no_allied_orders() {
    let with_orders = japanese_side_events(&run());
    let mut bare = without_allied_orders();
    bare.run_to_end().expect("runs");
    assert_eq!(with_orders, japanese_side_events(&bare));
    assert!(!with_orders.is_empty());
}

// -------------------------------------------------------------- Variants

#[test]
fn variant_base_one_with_no_figure_repairs_nothing() {
    let source = edited(|t| {
        t.replacen(
            "ship_repair_points_per_day = 6",
            "ship_repair_points_per_day = 0",
            1,
        )
    });
    let mut scenario = common::from_text(&source, &path()).expect("loads");
    scenario.run_to_end().expect("runs");
    assert!(repaired(&scenario).is_empty());
    let one = ship(&scenario, 1);
    assert_eq!((one.fire_damage, one.flotation_damage), (fx("4"), fx("6")));
    assert_eq!(
        one.systems.get(ShipSystem::FireControl),
        SystemState::Degraded
    );
    assert_eq!(one.systems.get(ShipSystem::MainBattery), SystemState::Out);
    let two = ship(&scenario, 2);
    assert_eq!(
        (two.fire_damage, two.flotation_damage),
        (fx("2.5"), fx("9.5"))
    );
    assert_eq!(
        two.systems.get(ShipSystem::TorpedoMounts),
        SystemState::Degraded
    );
    let nine = ship(&scenario, 9);
    assert_eq!(
        (nine.fire_damage, nine.flotation_damage),
        (fx("1"), fx("10"))
    );
    assert_eq!(
        nine.systems.get(ShipSystem::Propulsion),
        SystemState::Degraded
    );
    for n in [1, 2, 9] {
        assert_eq!(ship(&scenario, n).system_repair_points, fx("0"));
    }
}

#[test]
fn variant_a_lower_flotation_limit_of_nine() {
    let options = common::altered_reference("flotation9", |dir| {
        common::edit_file_by(
            dir,
            "surface_combat.toml",
            &[(
                "forward_repair_max_flotation = \"10\"",
                "forward_repair_max_flotation = \"9\"",
            )],
        );
    });
    let mut scenario = common::from_text_with(&text(), &path(), &options).expect("loads");
    scenario.run_to_end().expect("runs");
    use ShipSystem::*;
    assert_eq!(
        repaired(&scenario),
        vec![
            row(1, 1, 1, "6", "0", "4", &[], "0"),
            row(2, 1, 1, "6", "0", "0", &[], "2"),
            row(3, 1, 1, "6", "0", "0", &[], "8"),
            row(4, 1, 1, "2", "0", "0", &[FireControl], "0"),
            row(4, 2, 1, "4", "0", "9.5", &[], "1.5"),
            row(5, 2, 1, "6", "0", "9.5", &[], "7.5"),
            row(6, 2, 1, "2.5", "0", "9.5", &[TorpedoMounts], "0"),
            row(6, 9, 7, "3.5", "0", "10", &[], "2.5"),
            row(7, 9, 7, "6", "0", "10", &[], "8.5"),
            row(8, 9, 7, "1.5", "0", "10", &[Propulsion], "0"),
        ]
    );
    let total = repaired(&scenario)
        .iter()
        .fold(Fixed::ZERO, |sum, r| sum + r.4);
    assert_eq!(total, fx("43.5"));
    assert_eq!(ship(&scenario, 2).flotation_damage, fx("9.5"));
    assert_eq!(ship(&scenario, 9).flotation_damage, fx("10"));
}

#[test]
fn variant_force_two_ordered_on_day_one_returns_on_day_35() {
    let source = edited(|t| {
        t.replacen(
            "day = 2\nside = \"allied\"\ntask_force = 2\n",
            "day = 1\nside = \"allied\"\ntask_force = 2\n",
            1,
        )
    });
    let mut scenario = common::from_text(&source, &path()).expect("loads");
    scenario.run_to_end().expect("runs");
    let left = left_for_yard(&scenario, 2).expect("force 2 leaves");
    assert_eq!(left.day, 1);
    assert!(matches!(
        left.kind,
        EventKind::ForceLeftForYard {
            days_away: 34,
            return_day: 35,
            ..
        }
    ));
    let back = events_of(&scenario)
        .into_iter()
        .find(|e| matches!(e.kind, EventKind::ForceReturnedFromYard { .. }))
        .expect("it returns");
    assert_eq!(back.day, 35);
}

#[test]
fn variant_a_yard_passage_of_zero_is_refused_when_loaded() {
    let source = edited(|t| {
        t.replacen(
            "q = 4\nr = 0\nyard_passage_days = 4",
            "q = 4\nr = 0\nyard_passage_days = 0",
            1,
        )
    });
    let error = common::from_text(&source, &path()).expect_err("refused");
    assert!(error.to_string().contains("yard_passage_days"), "{error}");
}

// ------------------------------------------------------------ Properties

/// Every event's id, day, pulse, kind, causes and breakdown, as text.
fn log_text(scenario: &Scenario) -> Vec<String> {
    events_of(scenario)
        .into_iter()
        .map(|e| {
            format!(
                "{:?} {} {:?} {:?} {:?} {:?}",
                e.id, e.day, e.pulse, e.kind, e.causes, e.breakdown
            )
        })
        .collect()
}

#[test]
fn property_no_draw_the_log_is_the_same_for_seeds_1_to_50() {
    let base = load();
    let mut reference = base.with_seed(1942).expect("a seed");
    reference.run_to_end().expect("runs");
    let expected = log_text(&reference);
    for seed in 1..=50u64 {
        let mut scenario = base.with_seed(seed).expect("a seed");
        scenario.run_to_end().expect("runs");
        assert_eq!(log_text(&scenario), expected, "seed {seed}");
    }
}

#[test]
fn property_one_seed_twice_gives_the_same_hash_every_day() {
    let a = run();
    let b = run();
    assert_eq!(a.world.day_hashes().len(), 37);
    assert_eq!(a.world.day_hashes(), b.world.day_hashes());
}

#[test]
fn property_no_damage_track_rises_and_the_bank_stays_below_ten() {
    fn rank(state: SystemState) -> u8 {
        match state {
            SystemState::Working => 0,
            SystemState::Degraded => 1,
            SystemState::Out => 2,
        }
    }
    let mut scenario = load();
    let mut before: Vec<Ship> = scenario.world.umpire().ships.values().cloned().collect();
    while scenario.next_day().expect("a day").is_some() {
        let now: Vec<Ship> = scenario.world.umpire().ships.values().cloned().collect();
        for (old, new) in before.iter().zip(&now) {
            assert!(new.fire_damage <= old.fire_damage, "{} fire rose", new.id);
            assert!(
                new.flotation_damage <= old.flotation_damage,
                "{} flotation rose",
                new.id
            );
            let back_from_yard = matches!(old.location, ShipLocation::Yard(_))
                && !matches!(new.location, ShipLocation::Yard(_));
            for system in ShipSystem::ALL {
                let (was, is) = (rank(old.systems.get(system)), rank(new.systems.get(system)));
                assert!(is <= was, "{} {} got worse", new.id, system.name());
                // An out system is mended only by the yard, never forward.
                if was == 2 && is < 2 {
                    assert!(
                        back_from_yard,
                        "{} {} mended forward",
                        new.id,
                        system.name()
                    );
                }
            }
            assert!(new.system_repair_points >= Fixed::ZERO);
            assert!(new.system_repair_points < fx("10"));
        }
        before = now;
    }
}
