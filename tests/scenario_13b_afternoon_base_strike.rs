//! Scenario 13b: a carrier closes an enemy airfield and strikes it in the
//! afternoon of the first day and in the morning of the second.
//!
//! Stage 1 of decision 0063, decision 7. It proves that a base strike
//! launches in the first daylight pulse of the day in which it can, and
//! still once a day. Nobody opposes the strike.
//!
//! The data is placeholder data: see
//! `data/scenarios/13b_afternoon_base_strike.toml`. The rule is
//! `docs/decisions/0063-strike-target-priority.md`, which settles item 3 of
//! `docs/decisions/0034-raids-on-a-base.md`.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. The tests below assert these
//! numbers. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass.
//!
//! ## The setting
//!
//! The frozen step 1 figures, unchanged. The map is
//! `placeholder_two_airfields.toml`. Two days.
//!
//! Japanese base 1, an airfield, is at (0,0), with 1,000 t of supply, no
//! anti-aircraft fire and no aircraft.
//!
//! The Allied carrier force, task force 1, starts at (12,0): carrier A,
//! ship 1 (class 4), and a destroyer, ship 2. It is ordered on day 1 to
//! move to (3,0), and steams at 16 knots, the cruising speed of both
//! classes. Aboard carrier A is air group 1, 18 dive bombers (type 3:
//! radius 250 nm), ordered on day 1 to strike base 1.
//!
//! ## Where the carrier is
//!
//! The way is clear along the row, so each leg is charged 40 miles. A force
//! is in the hex whose centre is nearest: the hex ahead from 20 miles along
//! a leg. Movement comes after air combat in a daylight pulse, so a
//! decision reads the position the pulse began with.
//!
//! | Moment              | Miles steamed | Legs done and miles into the next | Hex    | To base 1 |
//! | ------------------- | ------------- | --------------------------------- | ------ | --------- |
//! | Day 1, 06:00        | 12 x 16 = 192 | 4 and 32                          | (7,0)  | 280 nm    |
//! | Day 1, 12:00        | 288           | 7 and 8                           | (5,0)  | 200 nm    |
//! | Day 1, 16:30        | 360           | 9: arrived                        | (3,0)  | 120 nm    |
//! | Day 2, every pulse  | 360           | Stopped                           | (3,0)  | 120 nm    |
//!
//! ## The launches
//!
//! | Pulse             | Carrier at | In reach of 250? | Launch                                           |
//! | ----------------- | ---------- | ---------------- | ------------------------------------------------ |
//! | Day 1, morning    | (7,0)      | No, 280          | None. Nothing recorded                           |
//! | Day 1, afternoon  | (5,0)      | Yes, 200         | 18 at base 1, 200 nm                             |
//! | Day 2, morning    | (3,0)      | Yes, 120         | 18 at base 1, 120 nm                             |
//! | Day 2, afternoon  | (3,0)      | Yes, 120         | None: the group has flown at its base that day   |
//!
//! Two `BaseStrikeLaunched` events in every run, and no more. Before this
//! rule there was one: day 2's.
//!
//! ## The attacks
//!
//! Each raid finds its base. 18 shots at a bomb's chance of 0.2: 3.6
//! expected hits, from 0 to 18. A hit lands on aircraft with chance 0.3,
//! and there are none; on supply with chance 0.3, destroying 50 t; on
//! nothing with chance 0.4. So a raid destroys 3.6 x 0.3 x 50 = 54 t on
//! average, and the base's stock at the end is 1,000 less 50 times the
//! supply hits of both raids.
//!
//! No aircraft is lost: the group has 18 at the end of every run.
//!
//! Over seeds 1 to 1000, each raid's hits in all are within four standard
//! deviations of 3,600. One raid's hits have a standard deviation of the
//! square root of (18 x 0.2 x 0.8) = 1.697056, so 1,000 raids have 53.67,
//! and the allowed range is 3,386 to 3,814.
//!
//! ## Variants
//!
//! | Change                                      | Launches                                        |
//! | ------------------------------------------- | ----------------------------------------------- |
//! | The force starts stopped at (5,0), no move order | Morning of day 1 and of day 2, at 200 nm. None in an afternoon: the rule as it was |
//! | The force starts stopped at (7,0), no move order | None on either day: 280 nm                |
//!
//! # Properties
//!
//! - A base strike group launches at most once in a game day.
//! - It never launches at night, and never at a base beyond its reach from
//!   where its ship is when the pulse begins.
//! - If it can launch in the morning it does, and then not in the
//!   afternoon.
//! - A replay of any seed reproduces the daily hash.

mod common;

use std::fs;
use std::path::PathBuf;

use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId};
use sim_core::time::Pulse;
use sim_core::World;
use sim_data::Scenario;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
const SCENARIO: &str = "13b_afternoon_base_strike.toml";
const SEEDS: u64 = 1000;
const GROUP: AirGroupId = AirGroupId::new(1);
const BASE: BaseId = BaseId::new(1);
const AIRFIELD: Hex = Hex::new(0, 0);

/// The carrier's own start and move order, as the file gives them.
const START: &str = "q = 12\nr = 0\nmission = \"carrier\"";
const MOVE_ORDER: &str =
    "[[order]]\nday = 1\nside = \"allied\"\ntask_force = 1\nkind = \"set_route\"\nwaypoints = [[3, 0]]\n\n";

fn text() -> (String, PathBuf) {
    let path = PathBuf::from(ROOT).join("data/scenarios").join(SCENARIO);
    (fs::read_to_string(&path).expect("the scenario reads"), path)
}

/// The scenario as the file gives it, on the frozen figures.
fn scenario() -> Scenario {
    let (text, path) = text();
    common::from_text(&text, &path).expect("the scenario loads")
}

/// The scenario with the carrier force stopped at (`q`,0) and no move order.
fn stopped_at(q: i32) -> Scenario {
    let (text, path) = text();
    let text = common::replace_once(
        &text,
        START,
        &format!("q = {q}\nr = 0\nmission = \"carrier\""),
    );
    let text = common::replace_once(&text, MOVE_ORDER, "");
    common::from_text(&text, &path).expect("the variant loads")
}

fn run_seed(base: &Scenario, seed: u64) -> World {
    let mut scenario = base.with_seed(seed).expect("a seed can be set");
    scenario.run_to_end().expect("the run resolves");
    scenario.world
}

fn runs(base: &Scenario) -> Vec<World> {
    (1..=SEEDS).map(|seed| run_seed(base, seed)).collect()
}

fn events_of(world: &World, keep: impl Fn(&Event) -> bool) -> Vec<Event> {
    world
        .events()
        .iter()
        .filter(|event| keep(event))
        .cloned()
        .collect()
}

/// (day, pulse, aircraft, distance in nm) of each base strike launch, in the
/// order of the log.
fn launches(world: &World) -> Vec<(u32, Pulse, u32, i64)> {
    events_of(world, |e| {
        matches!(e.kind, EventKind::BaseStrikeLaunched { .. })
    })
    .iter()
    .map(|event| match event.kind {
        EventKind::BaseStrikeLaunched {
            air_group,
            base,
            hex,
            aircraft,
            distance_nm,
        } => {
            assert_eq!((air_group, base, hex), (GROUP, BASE, AIRFIELD));
            (
                event.day,
                event.pulse.expect("a launch is in a pulse"),
                aircraft,
                distance_nm.trunc(),
            )
        }
        _ => unreachable!(),
    })
    .collect()
}

/// (hits, hits on aircraft, hits on supply) of each raid's attack, in order.
fn attacks(world: &World) -> Vec<(u32, u32, u32)> {
    events_of(world, |e| matches!(e.kind, EventKind::BaseAttacked { .. }))
        .iter()
        .map(|event| match event.kind {
            EventKind::BaseAttacked {
                shots,
                chance,
                expected,
                hits,
                hits_on_aircraft,
                hits_on_supply,
                ..
            } => {
                assert_eq!(shots, Fixed::from_int(18));
                assert_eq!(chance, Fixed::from_ratio(1, 5));
                assert_eq!(expected, Fixed::from_ratio(18, 5));
                (hits, hits_on_aircraft, hits_on_supply)
            }
            _ => unreachable!(),
        })
        .collect()
}

#[test]
fn the_carrier_ends_at_3_0_and_the_airfield_is_120_nm_from_there() {
    // Day 1 pulse by pulse: (7,0) 280 nm, (5,0) 200 nm, (3,0) 120 nm.
    assert_eq!(Hex::new(7, 0).air_distance_nm(AIRFIELD), 280);
    assert_eq!(Hex::new(5, 0).air_distance_nm(AIRFIELD), 200);
    assert_eq!(Hex::new(3, 0).air_distance_nm(AIRFIELD), 120);
    let world = run_seed(&scenario(), 1);
    let force = &world.umpire().task_forces[&sim_core::ids::TaskForceId::new(1)];
    assert_eq!(force.position(), Hex::new(3, 0));
    let base = &world.umpire().bases[&BASE];
    assert_eq!(base.hex, AIRFIELD);
}

#[test]
fn the_group_launches_in_the_afternoon_of_day_1_and_the_morning_of_day_2_and_no_more() {
    for world in runs(&scenario()) {
        assert_eq!(
            launches(&world),
            vec![(1, Pulse::Afternoon, 18, 200), (2, Pulse::Morning, 18, 120)]
        );
    }
}

#[test]
fn each_raid_bombs_the_airfield_18_shots_at_0_2_and_no_aircraft_is_lost() {
    let mut hits_in_all = [0u32; 2];
    for world in runs(&scenario()) {
        let found = attacks(&world);
        assert_eq!(found.len(), 2);
        let mut supply_hits = 0;
        for (index, (hits, on_aircraft, on_supply)) in found.iter().enumerate() {
            assert!(*hits <= 18);
            // A hit lands on aircraft with chance 0.3 and there are none to
            // destroy: the hits of that kind are counted and wasted.
            assert!(on_aircraft + on_supply <= *hits);
            hits_in_all[index] += hits;
            supply_hits += on_supply;
        }
        // The base's stock is 1,000 less 50 for each hit on supply.
        let stock = world.umpire().bases[&BASE].supply_t;
        assert_eq!(stock, Fixed::from_int(1000 - 50 * i64::from(supply_hits)));
        // No aircraft is lost: the group has 18 at the end.
        assert_eq!(world.umpire().air_groups[&GROUP].ready, 18);
        assert!(events_of(&world, |e| matches!(
            e.kind,
            EventKind::AircraftLost { .. }
                | EventKind::AirFight { .. }
                | EventKind::AircraftDestroyedOnGround { .. }
        ))
        .is_empty());
    }
    // Each raid's hits over 1,000 seeds are within four standard deviations
    // of 3,600: 3,386 to 3,814.
    for (raid, total) in hits_in_all.iter().enumerate() {
        assert!(
            (3386..=3814).contains(total),
            "raid {}: {total} hits in 1,000 runs",
            raid + 1
        );
    }
}

#[test]
fn started_in_reach_it_flies_in_the_morning_of_each_day_and_never_in_an_afternoon() {
    // At (5,0), 200 nm: the rule as it was.
    for world in runs(&stopped_at(5)) {
        assert_eq!(
            launches(&world),
            vec![(1, Pulse::Morning, 18, 200), (2, Pulse::Morning, 18, 200)]
        );
    }
}

#[test]
fn started_out_of_reach_it_never_launches() {
    // At (7,0), 280 nm: beyond the radius of 250 on both days.
    for world in runs(&stopped_at(7)) {
        assert!(launches(&world).is_empty());
        assert!(attacks(&world).is_empty());
    }
}

#[test]
fn a_base_strike_launches_once_a_day_never_at_night_and_never_beyond_its_reach() {
    for base in [scenario(), stopped_at(5), stopped_at(7)] {
        for seed in 1..=100 {
            let world = run_seed(&base, seed);
            let found = launches(&world);
            for (day, pulse, _, distance) in &found {
                assert_ne!(*pulse, Pulse::Night);
                assert!(*distance <= 250, "a launch at {distance} nm");
                assert_eq!(
                    found.iter().filter(|(d, ..)| d == day).count(),
                    1,
                    "seed {seed}: one launch a day"
                );
            }
            // If it can launch in the morning it does, and then not in the
            // afternoon.
            for day in 1..=2 {
                let morning = found
                    .iter()
                    .any(|(d, pulse, ..)| *d == day && *pulse == Pulse::Morning);
                let afternoon = found
                    .iter()
                    .any(|(d, pulse, ..)| *d == day && *pulse == Pulse::Afternoon);
                assert!(!(morning && afternoon));
            }
        }
    }
}

#[test]
fn a_replay_of_any_seed_reproduces_the_daily_hash() {
    let base = scenario();
    for seed in [1, 2, 3, 77, 1000] {
        assert_eq!(
            run_seed(&base, seed).day_hashes(),
            run_seed(&base, seed).day_hashes()
        );
    }
    // The day of the last base strike is state: a game in which the group
    // launched on a day has another hash from one in which it did not.
    assert_ne!(
        run_seed(&base, 1).day_hashes(),
        run_seed(&stopped_at(7), 1).day_hashes()
    );
}
