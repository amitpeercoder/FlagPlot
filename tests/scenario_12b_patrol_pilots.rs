//! The pilot of a patrol aircraft shot down over its own hex (decision 0059,
//! stage 1b).
//!
//! # Hand calculation
//!
//! Written by the architect before the test code. The tests below assert
//! these numbers. If the code disagrees, the code or the rule is wrong: do
//! not change a number here to make a test pass.
//!
//! The scenario is `data/scenarios/12b_patrol_pilots.toml`: scenario 6a, the
//! raid on a base, with pilots counted for the two fighter groups: group 2,
//! the raid's escorts, 12 pilots for its 12 aircraft, and group 3, the
//! patrol over base 2, 12 pilots for its 12 aircraft. Nothing else changes,
//! so on the scenario's own seed the three mornings' fights are those of 6a:
//!
//! | Day | Escorts shoot down of the patrol (group 3) | Patrol shoots down of the escorts (group 2) |
//! | --- | --- | --- |
//! | 1 | 3 | 1 |
//! | 2 | 2 | 1 |
//! | 3 | 0 | 1 |
//!
//! Group 2 is shot down away from home, so each aircraft takes its pilot
//! (decision 0038), at any setting of the figures: 12 aircraft and 12
//! pilots become 11 and 11, 10 and 10, 9 and 9.
//!
//! Group 3 is shot down over its own base. Each aircraft lost draws for its
//! pilot, recovered with `crew_recovered_over_own_base`. The two settings
//! that take the draw out:
//!
//! | `crew_recovered_over_own_base` | Group 3 after day 1 | After day 2 | After day 3 |
//! | --- | --- | --- | --- |
//! | 1 (the frozen figure: step 1's rule) | 9 aircraft, 12 pilots | 7 and 12 | 7 and 12 |
//! | 0 | 9 aircraft, 9 pilots | 7 and 7 | 7 and 7 |
//!
//! At the live figure, 0.6, of the 5 aircraft lost 3 pilots are expected to
//! be recovered; that is not asserted for one seed.
//!
//! The fight is the same at every setting: a patrol flies the smaller of
//! its ready aircraft and its pilots, and losing a pilot with each aircraft
//! leaves that number what losing the aircraft alone leaves it.
//!
//! # Properties
//!
//! - At 1, no pilot of a patrol over its own hex is lost, in this scenario
//!   and in every scenario that existed before the rule: their events are
//!   what they were.
//! - At any setting, a patrol group's pilots lost in a pulse are never more
//!   than its aircraft lost in that pulse, and its pilots never fall below
//!   zero.
//! - Over seeds 1 to 1000 at 0.6, the patrol pilots recovered sum to within
//!   10 in 100 (of the expected count) of 0.6 times the patrol aircraft
//!   lost.
//! - A carrier's patrol at sea draws with `crew_recovered_beside_own_force`
//!   and not with the base's figure: with the first at 0 and the second at
//!   1 a carrier's patrol loses a pilot with every aircraft, and the reverse
//!   loses none.

use std::fs;
use std::path::PathBuf;

use sim_core::events::{Event, EventKind};
use sim_core::ids::AirGroupId;
use sim_core::World;
use sim_data::Scenario;

mod common;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
const ESCORTS: AirGroupId = AirGroupId::new(2);
const PATROL: AirGroupId = AirGroupId::new(3);

fn scenario_path(name: &str) -> PathBuf {
    PathBuf::from(ROOT).join("data/scenarios").join(name)
}

/// A scenario of the repository on the frozen figures with the two patrol
/// pilot figures set (`None` leaves the frozen "1" for the base figure and
/// for the one beside a force), after the exact text replacements `edits`.
fn variant(name: &str, base: &str, force: &str, edits: &[(&str, &str)], seed: u64) -> Scenario {
    let options = common::altered_reference("patrol_pilots", |dir| {
        common::edit_file_by(
            dir,
            "air_combat.toml",
            &[
                (
                    "crew_recovered_over_own_base = \"1\"",
                    &format!("crew_recovered_over_own_base = \"{base}\""),
                ),
                (
                    "crew_recovered_beside_own_force = \"1\"",
                    &format!("crew_recovered_beside_own_force = \"{force}\""),
                ),
            ],
        );
    });
    let path = scenario_path(name);
    let mut text = fs::read_to_string(&path).expect("the scenario reads");
    for (from, to) in edits {
        text = common::replace_once(&text, from, to);
    }
    common::from_text_with(&text, &path, &options)
        .expect("the variant loads")
        .with_seed(seed)
        .expect("a seed can be set")
}

fn run(scenario: &mut Scenario) -> &World {
    scenario.run_to_end().expect("the run resolves");
    &scenario.world
}

/// The 12b run on its own seed at the given base figure.
fn twelve_b(base: &str) -> Scenario {
    let mut scenario = variant("12b_patrol_pilots.toml", base, "1", &[], 1942);
    run(&mut scenario);
    scenario
}

fn events_of(world: &World, keep: impl Fn(&EventKind) -> bool) -> Vec<Event> {
    world
        .events()
        .iter()
        .filter(|event| keep(&event.kind))
        .cloned()
        .collect()
}

/// (aircraft lost, remaining) of each `AircraftLost` of `group`, by day.
fn losses(world: &World, group: AirGroupId) -> Vec<(u32, u32, u32)> {
    events_of(
        world,
        |k| matches!(k, EventKind::AircraftLost { air_group, .. } if *air_group == group),
    )
    .iter()
    .map(|event| match event.kind {
        EventKind::AircraftLost {
            aircraft,
            remaining,
            ..
        } => (event.day, aircraft, remaining),
        _ => unreachable!(),
    })
    .collect()
}

/// (day, pilots lost, pilots left) of each `PilotsLost` of `group`.
fn pilot_losses(world: &World, group: AirGroupId) -> Vec<(u32, u32, u32)> {
    events_of(
        world,
        |k| matches!(k, EventKind::PilotsLost { air_group, .. } if *air_group == group),
    )
    .iter()
    .map(|event| match event.kind {
        EventKind::PilotsLost { lost, left, .. } => (event.day, lost, left),
        _ => unreachable!(),
    })
    .collect()
}

/// The group's (aircraft, pilots) at the end of `day`, read from its events
/// (it starts with 12 and 12).
fn after_day(world: &World, group: AirGroupId, day: u32) -> (u32, u32) {
    let aircraft = losses(world, group)
        .iter()
        .rfind(|(d, ..)| *d <= day)
        .map_or(12, |(_, _, remaining)| *remaining);
    let pilots = pilot_losses(world, group)
        .iter()
        .rfind(|(d, ..)| *d <= day)
        .map_or(12, |(_, _, left)| *left);
    (aircraft, pilots)
}

fn lost_on(world: &World, group: AirGroupId, day: u32) -> u32 {
    losses(world, group)
        .iter()
        .filter(|(d, ..)| *d == day)
        .map(|(_, aircraft, _)| aircraft)
        .sum()
}

/// Whether `k` is within 10 in 100 of `expected` (both in hundredths): a
/// relative tolerance of 10 in 100 of the expected count.
fn within_ten_in_a_hundred(k: i64, expected_hundredths: i64) -> bool {
    let off = (k * 100 - expected_hundredths).abs();
    off * 10 <= expected_hundredths
}

// ---------------------------------------------------------------------
// The fights of 6a, and the pilots.
// ---------------------------------------------------------------------

#[test]
fn the_fights_of_the_three_mornings_are_those_of_6a() {
    let scenario = twelve_b("1");
    let world = &scenario.world;
    // Escorts shoot down of the patrol, and the patrol of the escorts.
    for (day, of_patrol, of_escorts) in [(1, 3, 1), (2, 2, 1), (3, 0, 1)] {
        assert_eq!(lost_on(world, PATROL, day), of_patrol, "day {day}");
        assert_eq!(lost_on(world, ESCORTS, day), of_escorts, "day {day}");
    }
}

#[test]
fn the_escorts_lose_a_pilot_with_each_aircraft_at_every_setting() {
    for base in ["1", "0", "0.6"] {
        let scenario = twelve_b(base);
        let world = &scenario.world;
        for (day, expected) in [(1, 11), (2, 10), (3, 9)] {
            assert_eq!(
                after_day(world, ESCORTS, day),
                (expected, expected),
                "{base}"
            );
        }
    }
}

#[test]
fn at_1_the_patrol_keeps_every_pilot_and_at_0_loses_one_with_each_aircraft() {
    let kept = twelve_b("1");
    for (day, aircraft) in [(1, 9), (2, 7), (3, 7)] {
        assert_eq!(
            after_day(&kept.world, PATROL, day),
            (aircraft, 12),
            "day {day}"
        );
    }
    assert!(pilot_losses(&kept.world, PATROL).is_empty());
    let lost = twelve_b("0");
    for (day, aircraft) in [(1, 9), (2, 7), (3, 7)] {
        assert_eq!(
            after_day(&lost.world, PATROL, day),
            (aircraft, aircraft),
            "day {day}"
        );
    }
    // Each PilotsLost follows the AircraftLost it is caused by.
    let log: Vec<Event> = lost.world.events().iter().cloned().collect();
    for (at, event) in log.iter().enumerate() {
        if let EventKind::PilotsLost {
            air_group, lost, ..
        } = event.kind
        {
            if air_group == PATROL {
                assert_eq!(
                    event.causes,
                    vec![sim_core::events::Cause::Event(log[at - 1].id)]
                );
                assert!(matches!(
                    log[at - 1].kind,
                    EventKind::AircraftLost { air_group: g, aircraft, .. }
                        if g == PATROL && aircraft == lost
                ));
            }
        }
    }
}

#[test]
fn the_fight_is_the_same_at_every_setting_and_the_log_is_6as_apart_from_pilot_events() {
    // 6a's log on its own seed, as text: the three mornings.
    let mut six_a = variant("06a_base_raid.toml", "1", "1", &[], 1942);
    let six_a: Vec<String> = run(&mut six_a)
        .events()
        .iter()
        .map(|e| e.kind.to_string())
        .collect();
    assert!(!six_a.iter().any(|t| t.contains("pilot")));
    for base in ["1", "0", "0.6"] {
        let scenario = twelve_b(base);
        let without: Vec<String> = scenario
            .world
            .events()
            .iter()
            .filter(|e| !matches!(e.kind, EventKind::PilotsLost { .. }))
            .map(|e| e.kind.to_string())
            .collect();
        assert_eq!(without, six_a, "base figure {base}");
    }
}

// ---------------------------------------------------------------------
// Properties.
// ---------------------------------------------------------------------

#[test]
fn a_patrols_pilots_lost_in_a_pulse_are_never_more_than_its_aircraft_lost() {
    for base in ["0", "0.6", "1"] {
        for seed in 1..=200 {
            let mut scenario = variant("12b_patrol_pilots.toml", base, "1", &[], seed);
            let world = run(&mut scenario);
            for day in 1..=3 {
                let pilots: u32 = pilot_losses(world, PATROL)
                    .iter()
                    .filter(|(d, ..)| *d == day)
                    .map(|(_, lost, _)| lost)
                    .sum();
                assert!(
                    pilots <= lost_on(world, PATROL, day),
                    "seed {seed} day {day}"
                );
            }
            let group = &world.umpire().air_groups[&PATROL];
            assert!(group.pilots.expect("counted") <= 12);
        }
    }
}

#[test]
fn over_seeds_1_to_1000_at_0_6_the_patrol_pilots_recovered_follow_the_figure() {
    let base = variant("12b_patrol_pilots.toml", "0.6", "1", &[], 1);
    let (mut aircraft, mut recovered) = (0_i64, 0_i64);
    for seed in 1..=1000 {
        let mut scenario = base.with_seed(seed).expect("a seed");
        let world = run(&mut scenario);
        let lost: u32 = (1..=3).map(|day| lost_on(world, PATROL, day)).sum();
        let pilots: u32 = pilot_losses(world, PATROL).iter().map(|(_, l, _)| l).sum();
        aircraft += i64::from(lost);
        recovered += i64::from(lost - pilots);
    }
    assert!(aircraft > 1000, "{aircraft}");
    assert!(
        within_ten_in_a_hundred(recovered, aircraft * 60),
        "{recovered} recovered of {aircraft}"
    );
}

#[test]
fn a_carriers_patrol_at_sea_draws_at_the_figure_beside_its_own_force() {
    // Scenario 5c on its own seed (1942): group 4, 12 fighters aboard the
    // Japanese carrier (ship 3) on combat air patrol, loses 4 aircraft in the
    // afternoon of day 1 to the escorts of group 3. Give it 12 pilots.
    let give_pilots = [(
        "ship = 3\naircraft_type = 4\nready = 12\n",
        "ship = 3\naircraft_type = 4\nready = 12\npilots = 12\n",
    )];
    let group = AirGroupId::new(4);
    // The base figure 1 and the figure beside a force 0: a pilot lost with
    // every aircraft.
    let mut at_sea = variant("05c_air_combat.toml", "1", "0", &give_pilots, 1942);
    let world = run(&mut at_sea);
    let aircraft: u32 = losses(world, group).iter().map(|(_, a, _)| a).sum();
    assert_eq!(aircraft, 4);
    let pilots: u32 = pilot_losses(world, group).iter().map(|(_, l, _)| l).sum();
    assert_eq!(pilots, aircraft);
    // And the reverse: none.
    let mut kept = variant("05c_air_combat.toml", "0", "1", &give_pilots, 1942);
    let world = run(&mut kept);
    assert_eq!(
        losses(world, group).iter().map(|(_, a, _)| a).sum::<u32>(),
        4
    );
    assert!(pilot_losses(world, group).is_empty());
    assert_eq!(world.umpire().air_groups[&group].pilots, Some(12));
}
