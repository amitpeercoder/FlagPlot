//! Objectives and results (decision 0054) for Savo Island, Rabaul against
//! Guadalcanal and the full slice.
//!
//! # Hand calculation
//!
//! Written by the architect before the test code, from the event log of
//! each scenario as it runs today with its own seed (1942). The tests below
//! assert these numbers. If the code disagrees, the code or the rule is
//! wrong: do not change a number here to make a test pass.
//!
//! Every scenario uses the placeholder scale of decision 0054: a ship sunk
//! is carrier 10, battleship 8, cruiser 4, destroyer 1, transport 2,
//! auxiliary 1; a ship crippled is half of that; an aircraft destroyed is
//! 0.05. Levels in all three: minor 3, major 8, decisive 16.
//!
//! ## Scenario 4, Savo Island (`04_savo_island.toml`)
//!
//! Both sides hold two objectives: ships sunk and ships crippled.
//!
//! | Fact | |
//! | ---- | - |
//! | Japanese ships sunk | ship 3 Kako and ship 5 Furutaka, cruisers |
//! | Allied ships sunk | ship 9 Canberra, a cruiser; ship 11 Patterson, a destroyer |
//! | Ships afloat with propulsion out | none on either side (Chokai, Aoba and Chicago have it degraded, which does not count) |
//!
//! Allied points: 2 cruisers x 4 = 8. Japanese points: 4 + 1 = 5. Allied
//! margin 8 - 5 = 3, which is exactly the minor level: a minor Allied
//! victory and a minor Japanese defeat.
//!
//! The night itself was a decisive Japanese victory. This result is the
//! scenario's known fault (decision 0024) set on the scale, not a claim
//! about history.
//!
//! ## Scenario 6, Rabaul against Guadalcanal (`06_rabaul_guadalcanal.toml`)
//!
//! The Allied side holds aircraft destroyed (0.05) and base 1, Henderson
//! Field, open at the end (10). The Japanese side holds aircraft destroyed
//! (0.05) and base 1 closed at the end (10).
//!
//! | Aircraft lost over the 28 days | Shot down | On the ground | Written off | Total |
//! | ------------------------------ | --------- | ------------- | ----------- | ----- |
//! | Japanese | 37 | 0 | 24 | 61 |
//! | Allied   | 38 | 6 | 8  | 52 |
//!
//! Henderson Field ends with 2747 t of supply and aircraft ready (groups 11
//! to 17 hold 9, 11, 1, 12, 13, 16 and 12), so it is open.
//!
//! Allied points: 61 x 0.05 = 3.05, and 10 for the open field: 13.05.
//! Japanese points: 52 x 0.05 = 2.6, and 0: 2.6. Allied margin 10.45, at
//! least 8 and under 16: a major Allied victory.
//!
//! ## Scenario 7, the full slice (`07_full_slice.toml`)
//!
//! The Allied side holds ships sunk, ships crippled, aircraft destroyed
//! (0.05), cargo landed at base 1 at 0.005 a ton, and base 1 open at the end
//! (10). The Japanese side holds ships sunk, ships crippled, aircraft
//! destroyed (0.05) and base 1 closed at the end (10).
//!
//! | Fact | |
//! | ---- | - |
//! | Japanese ships sunk | ship 33 Ryujo, a carrier |
//! | Allied ships sunk | ships 42 and 43, cruisers; ship 44, a destroyer |
//! | Allied ships crippled | ship 1 Enterprise, a carrier |
//! | Japanese ships crippled | none (ship 40 has propulsion degraded only) |
//! | Japanese aircraft lost | shot down 78, lost with Ryujo 29, written off 17: 124 |
//! | Allied aircraft lost | shot down 32, on the ground 4, written off 7: 43 |
//! | Cargo landed at base 1 by Allied ships | ships 37 and 38, each 40 + 120 + 240 + 80 = 480 t: 960 t |
//! | Henderson Field at the end | 4300.955 t of supply, aircraft ready: open |
//!
//! | Objective | Allied | Japanese |
//! | --------- | ------ | -------- |
//! | Ships sunk | 10 | 4 + 4 + 1 = 9 |
//! | Ships crippled | 0 | 5 |
//! | Aircraft destroyed | 124 x 0.05 = 6.2 | 43 x 0.05 = 2.15 |
//! | Cargo landed | 960 x 0.005 = 4.8 | |
//! | Base open / closed | 10 | 0 |
//! | Total | 31 | 16.15 |
//!
//! Allied margin 31 - 16.15 = 14.85, at least 8 and under 16: a major
//! Allied victory. The full slice is not a reconstruction and has no
//! history to set beside it.
//!
//! # Properties
//!
//! - Each scenario's daily hashes and events are what they were without the
//!   objectives section.
//! - In each, the two margins sum to zero and the levels mirror.
//! - The result from a saved game replayed is the same.

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::ids::{AirGroupId, BaseId, ShipId};
use sim_core::objectives::{GameResult, Level, Outcome, SideResult};
use sim_core::refdata::ShipKind;
use sim_core::state::{ShipLocation, ShipSystem, Side, SystemState};
use sim_core::World;
use sim_data::Scenario;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
const SEED: u64 = 1942;

fn path(name: &str) -> PathBuf {
    PathBuf::from(ROOT).join("data/scenarios").join(name)
}

fn text(name: &str) -> String {
    std::fs::read_to_string(path(name)).expect("the scenario reads")
}

fn load_text(source: &str, name: &str) -> Scenario {
    Scenario::from_text(source, &path(name))
        .expect("the scenario loads")
        .with_seed(SEED)
        .expect("a seed can be set")
}

fn run_text(source: &str, name: &str) -> World {
    let mut scenario = load_text(source, name);
    scenario.run_to_end().expect("the run resolves");
    scenario.world
}

fn run(name: &str) -> World {
    run_text(&text(name), name)
}

fn fx(value: &str) -> Fixed {
    value.parse().expect("a decimal")
}

fn n(value: i64) -> Fixed {
    Fixed::from_int(value)
}

fn result_of(world: &World) -> GameResult {
    world.result().expect("the scenario has objectives")
}

fn line(side: &SideResult, i: usize) -> (&'static str, Fixed, Fixed) {
    let o = &side.objectives[i];
    (o.kind, o.count, o.points)
}

fn ship_ids(world: &World, side: Side, keep: impl Fn(&sim_core::state::Ship) -> bool) -> Vec<u32> {
    world
        .umpire()
        .ships
        .values()
        .filter(|s| s.side == side && keep(s))
        .map(|s| s.id.raw())
        .collect()
}

fn sunk(s: &sim_core::state::Ship) -> bool {
    s.location == ShipLocation::Sunk
}

fn propulsion_out(s: &sim_core::state::Ship) -> bool {
    s.location != ShipLocation::Sunk
        && (s.systems.get(ShipSystem::Propulsion) == SystemState::Out
            || s.systems.get(ShipSystem::FlightDeck) == SystemState::Out)
}

fn kind_of(world: &World, ship: u32) -> ShipKind {
    let truth = world.umpire();
    world.ref_data().ship_classes[&truth.ships[&ShipId::new(ship)].class].kind
}

/// Aircraft lost for good, as (shot down, lost with a ship, on the ground,
/// written off), by the side of the air group.
fn losses(world: &World, side: Side) -> (u32, u32, u32, u32) {
    let truth = world.umpire();
    let mine = |g: &AirGroupId| truth.air_groups[g].side == side;
    let (mut shot, mut ship, mut ground, mut wear) = (0, 0, 0, 0);
    for event in world.events().iter() {
        match &event.kind {
            EventKind::AircraftLost {
                air_group,
                aircraft,
                ..
            } if mine(air_group) => shot += aircraft,
            EventKind::AirGroupLost {
                air_group,
                aircraft,
                ..
            } if mine(air_group) => ship += aircraft,
            EventKind::AircraftDestroyedOnGround {
                air_group,
                aircraft,
                ..
            }
            | EventKind::DamagedAircraftDestroyedOnGround {
                air_group,
                aircraft,
                ..
            } if mine(air_group) => ground += aircraft,
            EventKind::AircraftWorn {
                air_group,
                written_off,
                ..
            } if mine(air_group) => wear += written_off,
            _ => {}
        }
    }
    (shot, ship, ground, wear)
}

// ---------------------------------------------------------------------
// Scenario 4, Savo Island.
// ---------------------------------------------------------------------

#[test]
fn savo_island_is_a_minor_allied_victory_by_the_margin_of_exactly_three() {
    let world = run("04_savo_island.toml");
    // Japanese ships sunk: Kako (3) and Furutaka (5), cruisers.
    assert_eq!(ship_ids(&world, Side::Japanese, sunk), vec![3, 5]);
    assert!([3, 5]
        .iter()
        .all(|s| kind_of(&world, *s) == ShipKind::Cruiser));
    // Allied: Canberra (9), a cruiser, and Patterson (11), a destroyer.
    assert_eq!(ship_ids(&world, Side::Allied, sunk), vec![9, 11]);
    assert_eq!(kind_of(&world, 9), ShipKind::Cruiser);
    assert_eq!(kind_of(&world, 11), ShipKind::Destroyer);
    // No ship afloat with propulsion out.
    assert!(ship_ids(&world, Side::Japanese, propulsion_out).is_empty());
    assert!(ship_ids(&world, Side::Allied, propulsion_out).is_empty());

    let result = result_of(&world);
    assert_eq!(result.allied.objectives.len(), 2);
    assert_eq!(result.japanese.objectives.len(), 2);
    assert_eq!(line(&result.allied, 0), ("ships_sunk", n(2), n(8)));
    assert_eq!(line(&result.allied, 1), ("ships_crippled", n(0), n(0)));
    assert_eq!(line(&result.japanese, 0), ("ships_sunk", n(2), n(5)));
    assert_eq!(line(&result.japanese, 1), ("ships_crippled", n(0), n(0)));
    assert_eq!(result.allied.total, n(8));
    assert_eq!(result.japanese.total, n(5));
    // 8 - 5 = 3, exactly the minor level.
    assert_eq!(result.allied.margin, n(3));
    assert_eq!(result.japanese.margin, n(-3));
    assert_eq!(result.allied.outcome, Outcome::Victory(Level::Minor));
    assert_eq!(result.japanese.outcome, Outcome::Defeat(Level::Minor));
}

// ---------------------------------------------------------------------
// Scenario 6, Rabaul against Guadalcanal.
// ---------------------------------------------------------------------

#[test]
fn rabaul_against_guadalcanal_is_a_major_allied_victory() {
    let world = run("06_rabaul_guadalcanal.toml");
    // Aircraft lost over the 28 days: shot down, on the ground, written off.
    let japanese = losses(&world, Side::Japanese);
    let allied = losses(&world, Side::Allied);
    assert_eq!(japanese, (37, 0, 0, 24));
    assert_eq!(allied, (38, 0, 6, 8));

    // Henderson Field ends with 2747 t of supply and aircraft ready.
    let truth = world.umpire();
    let field = &truth.bases[&BaseId::new(1)];
    assert_eq!(field.supply_t, n(2747));
    let ready: Vec<(u32, u32)> = truth
        .air_groups
        .values()
        .filter(|g| g.location == sim_core::state::AirGroupLocation::Base(field.id))
        .filter(|g| g.ready > 0)
        .map(|g| (g.id.raw(), g.ready))
        .collect();
    assert_eq!(
        ready,
        vec![
            (11, 9),
            (12, 11),
            (13, 1),
            (14, 12),
            (15, 13),
            (16, 16),
            (17, 12)
        ]
    );

    let result = result_of(&world);
    assert_eq!(
        line(&result.allied, 0),
        ("aircraft_destroyed", n(61), fx("3.05"))
    );
    assert_eq!(line(&result.allied, 1), ("base_open", n(1), n(10)));
    assert_eq!(result.allied.total, fx("13.05"));
    assert_eq!(
        line(&result.japanese, 0),
        ("aircraft_destroyed", n(52), fx("2.6"))
    );
    assert_eq!(line(&result.japanese, 1), ("base_open", n(0), n(0)));
    assert!(result.japanese.objectives[1].closed);
    assert_eq!(result.japanese.total, fx("2.6"));
    assert_eq!(result.allied.margin, fx("10.45"));
    assert_eq!(result.japanese.margin, fx("-10.45"));
    assert_eq!(result.allied.outcome, Outcome::Victory(Level::Major));
    assert_eq!(result.japanese.outcome, Outcome::Defeat(Level::Major));
}

// ---------------------------------------------------------------------
// Scenario 7, the full slice.
// ---------------------------------------------------------------------

#[test]
fn the_full_slice_is_a_major_allied_victory() {
    let world = run("07_full_slice.toml");
    assert_eq!(ship_ids(&world, Side::Japanese, sunk), vec![33]);
    assert_eq!(kind_of(&world, 33), ShipKind::Carrier);
    assert_eq!(ship_ids(&world, Side::Allied, sunk), vec![42, 43, 44]);
    assert_eq!(kind_of(&world, 42), ShipKind::Cruiser);
    assert_eq!(kind_of(&world, 43), ShipKind::Cruiser);
    assert_eq!(kind_of(&world, 44), ShipKind::Destroyer);
    // Enterprise is crippled; nothing Japanese is (ship 40 has propulsion
    // degraded only).
    assert_eq!(ship_ids(&world, Side::Allied, propulsion_out), vec![1]);
    assert_eq!(kind_of(&world, 1), ShipKind::Carrier);
    assert!(ship_ids(&world, Side::Japanese, propulsion_out).is_empty());
    assert_eq!(
        world.umpire().ships[&ShipId::new(40)]
            .systems
            .get(ShipSystem::Propulsion),
        SystemState::Degraded
    );

    // Aircraft: Japanese shot down 78, lost with Ryujo 29, written off 17.
    // Allied shot down 32, on the ground 4, written off 7.
    assert_eq!(losses(&world, Side::Japanese), (78, 29, 0, 17));
    assert_eq!(losses(&world, Side::Allied), (32, 0, 4, 7));

    // Cargo landed at base 1 by Allied ships 37 and 38: 480 t each.
    let mut landed: BTreeMap<u32, Fixed> = BTreeMap::new();
    for event in world.events().iter() {
        if let EventKind::CargoUnloaded {
            ship,
            base,
            unloaded_t,
            ..
        } = &event.kind
        {
            if base.raw() == 1 {
                *landed.entry(ship.raw()).or_insert(Fixed::ZERO) += *unloaded_t;
            }
        }
    }
    assert_eq!(landed, BTreeMap::from([(37, n(480)), (38, n(480))]));
    assert_eq!(
        world.umpire().bases[&BaseId::new(1)].supply_t,
        fx("4300.955")
    );

    let result = result_of(&world);
    assert_eq!(result.allied.objectives.len(), 5);
    assert_eq!(result.japanese.objectives.len(), 4);
    assert_eq!(line(&result.allied, 0), ("ships_sunk", n(1), n(10)));
    assert_eq!(line(&result.allied, 1), ("ships_crippled", n(0), n(0)));
    assert_eq!(
        line(&result.allied, 2),
        ("aircraft_destroyed", n(124), fx("6.2"))
    );
    assert_eq!(line(&result.allied, 3), ("cargo_landed", n(960), fx("4.8")));
    assert_eq!(line(&result.allied, 4), ("base_open", n(1), n(10)));
    assert_eq!(result.allied.total, n(31));

    // Japanese: sunk 4 + 4 + 1 = 9, crippled 5, aircraft 43 x 0.05 = 2.15,
    // base shut 0.
    assert_eq!(line(&result.japanese, 0), ("ships_sunk", n(3), n(9)));
    assert_eq!(line(&result.japanese, 1), ("ships_crippled", n(1), n(5)));
    assert_eq!(
        line(&result.japanese, 2),
        ("aircraft_destroyed", n(43), fx("2.15"))
    );
    assert_eq!(line(&result.japanese, 3), ("base_open", n(0), n(0)));
    assert_eq!(result.japanese.total, fx("16.15"));

    assert_eq!(result.allied.margin, fx("14.85"));
    assert_eq!(result.japanese.margin, fx("-14.85"));
    assert_eq!(result.allied.outcome, Outcome::Victory(Level::Major));
    assert_eq!(result.japanese.outcome, Outcome::Defeat(Level::Major));
    assert_eq!(result.history, None);
}

// ---------------------------------------------------------------------
// Properties.
// ---------------------------------------------------------------------

const ALL: [&str; 3] = [
    "04_savo_island.toml",
    "06_rabaul_guadalcanal.toml",
    "07_full_slice.toml",
];

#[test]
fn the_objectives_change_no_hash_and_no_event() {
    for name in ALL {
        let source = text(name);
        let cut = source
            .find("# Missions and objectives")
            .expect("the objectives section");
        let without = run_text(&source[..cut], name);
        assert!(without.objectives().is_none());
        let with = run(name);
        assert_eq!(with.day_hashes(), without.day_hashes(), "{name}");
        assert_eq!(with.state_hash(), without.state_hash(), "{name}");
        assert_eq!(with.events().len(), without.events().len(), "{name}");
    }
}

#[test]
fn the_margins_sum_to_zero_and_the_levels_mirror() {
    for name in ALL {
        let result = result_of(&run(name));
        assert_eq!(result.allied.margin + result.japanese.margin, Fixed::ZERO);
        assert_eq!(result.allied.outcome.mirror(), result.japanese.outcome);
        assert_eq!(
            result.allied.margin,
            result.allied.total - result.japanese.total
        );
    }
}

#[test]
fn the_result_from_a_saved_game_replayed_is_the_same() {
    for name in ALL {
        let first = run(name);
        let saved = load_text(&text(name), name).scripted_orders().clone();
        let mut replay = load_text(&text(name), name);
        replay.replace_script(saved);
        replay.run_to_end().expect("the replay resolves");
        assert_eq!(replay.world.day_hashes(), first.day_hashes(), "{name}");
        assert_eq!(result_of(&replay.world), result_of(&first), "{name}");
    }
}
