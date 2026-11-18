//! A day in parts (decision 0156, step 1).
//!
//! The engine resolves a day in three parts: [`World::open_day`],
//! [`World::resolve_next_pulse`] for the night, the morning and the
//! afternoon, and [`World::close_day`]. [`World::resolve_day_watched`] is
//! those three called in turn. This step is a move and changes no rule, so
//! what must hold is that a day resolved by the parts is the day resolved by
//! [`Scenario::next_day_watched`], which calls `resolve_day_watched`:
//!
//! 1. the same daily hashes, the same summaries (day, hash, first event and
//!    event count) and the same event log, event for event, on scenario 1,
//!    scenario 5, the first three days of the full slice and scenario 36
//!    (the one convoy, whose staff acts), each with the staffs, as
//!    `run` plays them;
//! 2. the watcher is handed the same moments in the same order either way;
//! 3. calling a part out of its turn is [`SimError::DayPartOutOfTurn`],
//!    naming the part and the day in hand, and changes nothing.
//!
//! No expected number is written here: every comparison is of the two ways
//! of resolving the same day. No number depends on a figure, so the
//! scenarios are read on the live figures.

mod common;

use std::path::Path;

use sim_core::choices::Moment;
use sim_core::time::Pulse;
use sim_core::{DayInHand, DayPart, DaySummary, SimError, World};
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

fn load(name: &str) -> Scenario {
    let path = common::data_dir().join("scenarios").join(name);
    common::load_live(Path::new(&path)).expect("the scenario loads")
}

/// What one way of resolving gave: each day's summary, the moments the
/// watcher was handed, and the world at the end.
struct Run {
    summaries: Vec<DaySummary>,
    moments: Vec<Moment>,
    world: World,
}

/// Up to `days` days resolved whole, with the staffs.
fn by_days(name: &str, days: u32) -> Run {
    let mut scenario = load(name);
    let held = staffs();
    let mut moments = Vec::new();
    let mut summaries = Vec::new();
    for _ in 0..days {
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |_, moment| moments.push(moment))
            .expect("the day resolves");
        let Some(summary) = summary else { break };
        summaries.push(summary);
    }
    Run {
        summaries,
        moments,
        world: scenario.world,
    }
}

/// The same days resolved in parts. The scenario's commands and orders are
/// handed in before each day as [`Scenario::next_day_watched`] hands them
/// in: the commands first, then the orders, one at a time.
fn by_parts(name: &str, days: u32) -> Run {
    let mut scenario = load(name);
    let held = staffs();
    let mut moments = Vec::new();
    let mut summaries = Vec::new();
    let last = days.min(scenario.days);
    let commands = scenario.scripted_commands().clone();
    let script = scenario.scripted_orders().clone();
    let world = &mut scenario.world;
    for day in 1..=last {
        for (side, command) in commands.get(&day).cloned().unwrap_or_default() {
            world.submit_commands(side, vec![command]);
        }
        for (side, request) in script.get(&day).cloned().unwrap_or_default() {
            world.submit_orders(side, vec![request]);
        }
        let mut watch = |_: &World, moment: Moment| moments.push(moment);
        assert_eq!(world.day_in_hand(), None);
        let opened = world
            .open_day(as_staffs(&held), &mut watch)
            .expect("the day opens");
        assert_eq!(opened, day);
        for pulse in Pulse::ALL {
            assert_eq!(world.day_in_hand().map(|h| h.day), Some(day));
            assert_eq!(world.day_in_hand().and_then(|h| h.next_pulse), Some(pulse));
            let resolved = world
                .resolve_next_pulse(&mut watch)
                .expect("the pulse resolves");
            assert_eq!(resolved, pulse);
        }
        assert_eq!(world.day_in_hand().map(|h| h.next_pulse), Some(None));
        summaries.push(world.close_day(&mut watch).expect("the day closes"));
        assert_eq!(world.day_in_hand(), None);
    }
    Run {
        summaries,
        moments,
        world: scenario.world,
    }
}

fn assert_same(name: &str, days: u32) {
    let whole = by_days(name, days);
    let parts = by_parts(name, days);
    assert!(!whole.summaries.is_empty(), "{name}: no day resolved");
    assert_eq!(whole.summaries, parts.summaries, "{name}: the summaries");
    assert_eq!(
        whole.world.day_hashes(),
        parts.world.day_hashes(),
        "{name}: the daily hashes"
    );
    assert_eq!(whole.world.state_hash(), parts.world.state_hash(), "{name}");
    assert_eq!(
        whole.world.events().len(),
        parts.world.events().len(),
        "{name}: the event count"
    );
    assert!(
        whole.world.events() == parts.world.events(),
        "{name}: the event logs differ"
    );
    assert_eq!(whole.moments, parts.moments, "{name}: the moments");
    // The moments of each day: DayBegins, Orders, then for each pulse its
    // beginning and its phases, then the end-of-day phases.
    let begins = whole
        .moments
        .iter()
        .filter(|m| matches!(m, Moment::DayBegins { .. }))
        .count();
    assert_eq!(begins, whole.summaries.len(), "{name}");
}

#[test]
fn scenario_1_in_parts_is_the_same() {
    assert_same("01_transit.toml", u32::MAX);
}

#[test]
fn scenario_5_in_parts_is_the_same() {
    assert_same("05_eastern_solomons.toml", u32::MAX);
}

#[test]
fn the_full_slice_first_three_days_in_parts_are_the_same() {
    let whole = by_days("07_full_slice.toml", 3);
    assert_eq!(whole.summaries.len(), 3);
    assert_same("07_full_slice.toml", 3);
}

#[test]
fn scenario_36_with_its_staff_in_parts_is_the_same() {
    assert_same("36_one_convoy.toml", u32::MAX);
}

fn out_of_turn(part: DayPart, in_hand: Option<DayInHand>) -> SimError {
    SimError::DayPartOutOfTurn { part, in_hand }
}

/// Each wrong call is the error, names the day in hand, and leaves the
/// world as it was: the same hash and the same events, and the day goes on
/// to the hash it has when resolved whole.
#[test]
fn a_part_out_of_its_turn_is_an_error_and_changes_nothing() {
    let mut world = load("01_transit.toml").world;
    let mut watch = |_: &World, _: Moment| {};
    let unchanged = |world: &mut World, part: DayPart, call: &dyn Fn(&mut World) -> SimError| {
        let hash = world.state_hash();
        let events = world.events().len();
        let in_hand = world.day_in_hand();
        assert_eq!(call(world), out_of_turn(part, in_hand));
        assert_eq!(world.state_hash(), hash);
        assert_eq!(world.events().len(), events);
        assert_eq!(world.day_in_hand(), in_hand);
    };
    let pulse = |w: &mut World| {
        w.resolve_next_pulse(&mut |_, _| {})
            .expect_err("out of turn")
    };
    let close = |w: &mut World| w.close_day(&mut |_, _| {}).expect_err("out of turn");
    let open = |w: &mut World| {
        w.open_day(sim_core::Staffs::none(), &mut |_, _| {})
            .expect_err("out of turn")
    };

    // No day open: no pulse and no close.
    unchanged(&mut world, DayPart::Pulse, &pulse);
    unchanged(&mut world, DayPart::Close, &close);

    let first_event = world.events().len();
    world
        .open_day(sim_core::Staffs::none(), &mut watch)
        .expect("opens");
    assert_eq!(
        world.day_in_hand(),
        Some(DayInHand {
            day: 1,
            next_pulse: Some(Pulse::Night),
            first_event,
        })
    );
    // A day open: no second opening, and no close before the afternoon.
    unchanged(&mut world, DayPart::Open, &open);
    unchanged(&mut world, DayPart::Close, &close);
    world.resolve_next_pulse(&mut watch).expect("night");
    unchanged(&mut world, DayPart::Close, &close);
    world.resolve_next_pulse(&mut watch).expect("morning");
    unchanged(&mut world, DayPart::Close, &close);
    world.resolve_next_pulse(&mut watch).expect("afternoon");
    // The afternoon resolved: no pulse after it, and no second opening.
    unchanged(&mut world, DayPart::Pulse, &pulse);
    unchanged(&mut world, DayPart::Open, &open);
    let summary = world.close_day(&mut watch).expect("closes");
    // Closed: no second close and no pulse.
    unchanged(&mut world, DayPart::Close, &close);
    unchanged(&mut world, DayPart::Pulse, &pulse);

    // The day with the wrong calls in it is the day resolved whole.
    let mut whole = load("01_transit.toml").world;
    assert_eq!(whole.resolve_day().expect("resolves"), summary);
    assert!(whole.events() == world.events());

    // resolve_day with a day open is the same error.
    world
        .open_day(sim_core::Staffs::none(), &mut watch)
        .expect("opens");
    let in_hand = world.day_in_hand();
    assert_eq!(
        world.resolve_day().expect_err("a day is open"),
        out_of_turn(DayPart::Open, in_hand)
    );
}

#[test]
fn the_error_says_what_was_called_and_what_is_open() {
    let none = out_of_turn(DayPart::Pulse, None).to_string();
    assert_eq!(none, "cannot resolve a pulse: no day is open");
    let open = out_of_turn(
        DayPart::Close,
        Some(DayInHand {
            day: 2,
            next_pulse: Some(Pulse::Morning),
            first_event: 0,
        }),
    )
    .to_string();
    assert_eq!(
        open,
        "cannot close a day: day 2 is open and its morning pulse is next"
    );
    let done = out_of_turn(
        DayPart::Open,
        Some(DayInHand {
            day: 2,
            next_pulse: None,
            first_event: 0,
        }),
    )
    .to_string();
    assert_eq!(
        done,
        "cannot open a day: day 2 is open and its afternoon is resolved"
    );
}

// ---------------------------------------------------------------------
// Step 3 of decision 0156: the copy with the open day treated as closed,
// and the clock on a side's view.
// ---------------------------------------------------------------------

#[test]
fn a_copy_of_a_closed_day_is_a_plain_clone() {
    let mut scenario = load("36_one_convoy.toml");
    let held = staffs();
    scenario
        .next_day_staffed(as_staffs(&held))
        .expect("the day resolves");
    let world = &scenario.world;
    let copy = world.as_if_day_closed();
    assert_eq!(copy.state_hash(), world.state_hash());
    assert_eq!(copy.day(), world.day());
    assert_eq!(copy.day_in_hand(), None);
    assert_eq!(copy.events().len(), world.events().len());
}

#[test]
fn a_copy_of_an_open_day_is_that_day_with_no_day_in_hand_and_the_world_is_untouched() {
    let mut scenario = load("36_one_convoy.toml");
    let held = staffs();
    scenario
        .next_day_staffed(as_staffs(&held))
        .expect("the day resolves");
    let world = &mut scenario.world;
    world
        .open_day(as_staffs(&held), &mut |_, _| {})
        .expect("the day opens");
    for _ in 0..2 {
        let before = world.state_hash();
        let in_hand = world.day_in_hand();
        let copy = world.as_if_day_closed();
        assert_eq!(copy.day(), 2);
        assert_eq!(copy.day_in_hand(), None);
        assert!(copy
            .orders_waiting_inside_day(sim_core::state::Side::Allied)
            .is_empty());
        // The copy's staff step is that of day 3.
        let staff_step = copy.at_staff_step();
        assert_eq!(staff_step.day(), 2);
        assert_eq!(world.state_hash(), before);
        assert_eq!(world.day_in_hand(), in_hand);
        assert_eq!(world.day(), 1);
        world
            .resolve_next_pulse(&mut |_, _| {})
            .expect("the pulse resolves");
    }
}

#[test]
fn the_clock_on_a_sides_view_at_every_moment() {
    use sim_core::held::AsOf;
    use sim_core::state::Side;
    let mut scenario = load("36_one_convoy.toml");
    let held = staffs();
    let world = &mut scenario.world;
    let at = |world: &World| {
        let view = world.side_view(Side::Allied);
        (
            view.day_in_hand().map(|h| (h.day, h.next_pulse)),
            view.pulse_last_resolved(),
            view.pulse_just_ended(),
        )
    };
    let of = |day, pulse| AsOf::Pulse { day, pulse };
    // Day 1, as the start is the moment before it.
    assert_eq!(at(world), (None, AsOf::Start, AsOf::Start));
    world
        .open_day(as_staffs(&held), &mut |_, _| {})
        .expect("the day opens");
    assert_eq!(
        at(world),
        (Some((1, Some(Pulse::Night))), AsOf::Start, AsOf::Start)
    );
    world.resolve_next_pulse(&mut |_, _| {}).expect("the night");
    assert_eq!(
        at(world),
        (
            Some((1, Some(Pulse::Morning))),
            of(1, Pulse::Night),
            AsOf::Start
        )
    );
    world
        .resolve_next_pulse(&mut |_, _| {})
        .expect("the morning");
    assert_eq!(
        at(world),
        (
            Some((1, Some(Pulse::Afternoon))),
            of(1, Pulse::Morning),
            AsOf::Start
        )
    );
    world
        .resolve_next_pulse(&mut |_, _| {})
        .expect("the afternoon");
    assert_eq!(
        at(world),
        (Some((1, None)), of(1, Pulse::Afternoon), AsOf::Start)
    );
    world.close_day(&mut |_, _| {}).expect("the day closes");
    let ended = of(1, Pulse::Afternoon);
    assert_eq!(at(world), (None, ended, ended));
    // Day 2: with night next the last resolved is day 1's afternoon, which
    // is what pulse_just_ended has been and stays inside the day.
    world
        .open_day(as_staffs(&held), &mut |_, _| {})
        .expect("the day opens");
    assert_eq!(at(world), (Some((2, Some(Pulse::Night))), ended, ended));
    world.resolve_next_pulse(&mut |_, _| {}).expect("the night");
    assert_eq!(
        at(world),
        (Some((2, Some(Pulse::Morning))), of(2, Pulse::Night), ended)
    );
}
