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
//! This is the pinned outcome of the scenario's own seed, and it follows the
//! scenario's inputs: it was a major Allied victory (margin 14.85) until the
//! Eastern Solomons orders were given the record's reading on Nov 12, 2026
//! (decisions 0059 and 0063 and the `ryujo-raid` change: Ryujo in (7,-9), her
//! raid on Henderson Field flown, the fleet carriers' dive bombers held for
//! heavy ships). A change of the orders moves it again, and this
//! calculation is then redone from the events; it is not a claim about
//! history.
//!
//! | Fact | |
//! | ---- | - |
//! | Japanese ships sunk | ship 33 Ryujo, a carrier |
//! | Allied ships sunk | ships 42 and 43, cruisers; ship 44, a destroyer |
//! | Allied ships crippled | none: groups 112 and 115 attack Enterprise on day 4 (13 and 12 bombers left, 5 and 2 hits) and her flight deck stays at working or degraded |
//! | Japanese ships crippled | none (ship 40 has propulsion degraded only) |
//! | Japanese aircraft lost | shot down 80; lost with Ryujo 1 (group 118's 23 aircraft less the 22 that could fly, 23 x 0.97 = 22.31 for its fatigue of 3, were aboard; group 119's 0); ran out of fuel on the way home from the raid 1 (group 119: 6 x 0.285714 = 1.714284 expected); came down in the sea for want of a deck 23 (group 118: 18, group 119: 5); written off 16: 80 + 1 + 1 + 23 + 16 = 121 |
//! | Allied aircraft lost | shot down 38, on the ground 4, written off 7: 49 |
//! | Cargo landed at base 1 by Allied ships | ships 37 and 38, each 62 + 120 + 240 + 58 = 480 t: 960 t (the convoy arrives a little sooner under decision 0058, so the tons fall in other pulses) |
//! | Henderson Field at the end | 4339.2 t of supply, aircraft ready: open. From the events: 4000 t at the start + 960 t landed - 350 t destroyed (three raids on the supply) - 270.8 t spent (13 on 13 repairs, 86 on the aircraft at the base for the day and 171.8 on aircraft sent on sorties) = 4339.2 t. (It was 4342.72 t before the change, and 4300.955 t while Flight 300 arrived by script: 41.765 t is what that group's 11 aircraft cost, 11 nights at 2.2 t on the 11 aircraft at the base for the day (24.2), 15.565 t on its strike of the afternoon of day 5 and 2 t on its 2 repairs, decision 11 of 0059) |
//!
//! | Objective | Allied | Japanese |
//! | --------- | ------ | -------- |
//! | Ships sunk | 10 | 4 + 4 + 1 = 9 |
//! | Ships crippled | 0 | 0 |
//! | Aircraft destroyed | 121 x 0.05 = 6.05 | 49 x 0.05 = 2.45 |
//! | Cargo landed | 960 x 0.005 = 4.8 | |
//! | Base open / closed | 10 | 0 |
//! | Total | 10 + 0 + 6.05 + 4.8 + 10 = 30.85 | 9 + 0 + 2.45 + 0 = 11.45 |
//!
//! Allied margin 30.85 - 11.45 = 19.4, at least 16: a decisive Allied
//! victory. (Before the change: 31 and 16.15, margin 14.85, major. With the target list carrier, battleship, cruiser the figures were 31.15 and 11.25, margin 19.9.) The full
//! slice is not a reconstruction and has no history to set beside it.
//!
//! # Properties
//!
//! - Each scenario's daily hashes and events are what they were without the
//!   objectives section.
//! - In each, the two margins sum to zero and the levels mirror.
//! - The result from a saved game replayed is the same.

mod common;

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
    common::from_text(source, &path(name))
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

/// The aircraft of `side` that came down in the sea for want of a deck.
fn no_deck_losses(world: &World, side: Side) -> u32 {
    let truth = world.umpire();
    world
        .events()
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::AircraftDitchedNoDeck {
                air_group,
                aircraft,
                ..
            } if truth.air_groups[air_group].side == side => Some(*aircraft),
            _ => None,
        })
        .sum()
}

/// The aircraft of `side` that ran out of fuel on the way home (decision 3 of
/// 0059).
fn fuel_losses(world: &World, side: Side) -> u32 {
    let truth = world.umpire();
    world
        .events()
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::AircraftDitched {
                air_group, ditched, ..
            } if truth.air_groups[air_group].side == side => Some(*ditched),
            _ => None,
        })
        .sum()
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

/// The pinned outcome of the scenario's own seed: it follows the inputs, and is
/// redone from the events when the orders change (see the hand calculation).
#[test]
fn the_full_slice_on_its_own_seed_ends_in_a_decisive_allied_victory() {
    let world = run("07_full_slice.toml");
    assert_eq!(ship_ids(&world, Side::Japanese, sunk), vec![33]);
    assert_eq!(kind_of(&world, 33), ShipKind::Carrier);
    assert_eq!(ship_ids(&world, Side::Allied, sunk), vec![42, 43, 44]);
    assert_eq!(kind_of(&world, 42), ShipKind::Cruiser);
    assert_eq!(kind_of(&world, 43), ShipKind::Cruiser);
    assert_eq!(kind_of(&world, 44), ShipKind::Destroyer);
    // Nothing is crippled: Enterprise is attacked but keeps her deck, and
    // nothing Japanese is (ship 40 has propulsion degraded only).
    assert!(ship_ids(&world, Side::Allied, propulsion_out).is_empty());
    assert_eq!(kind_of(&world, 1), ShipKind::Carrier);
    assert!(ship_ids(&world, Side::Japanese, propulsion_out).is_empty());
    assert_eq!(
        world.umpire().ships[&ShipId::new(40)]
            .systems
            .get(ShipSystem::Propulsion),
        SystemState::Degraded
    );

    // Aircraft: Japanese shot down 80, lost with Ryujo 1, written off 16,
    // 1 out of fuel and 23 in the sea for want of a deck (80 + 1 + 1 + 23 + 16
    // = 121). Allied shot down 38, on the ground 4, written off 7 (49).
    assert_eq!(losses(&world, Side::Japanese), (80, 1, 0, 16));
    assert_eq!(fuel_losses(&world, Side::Japanese), 1);
    assert_eq!(no_deck_losses(&world, Side::Japanese), 23);
    assert_eq!(no_deck_losses(&world, Side::Allied), 0);
    assert_eq!(fuel_losses(&world, Side::Allied), 0);
    assert_eq!(losses(&world, Side::Allied), (38, 0, 4, 7));

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
    // 4000 + 960 landed - 350 destroyed - 270.8 spent = 4339.2.
    assert_eq!(world.umpire().bases[&BaseId::new(1)].supply_t, fx("4339.2"));

    let result = result_of(&world);
    assert_eq!(result.allied.objectives.len(), 5);
    assert_eq!(result.japanese.objectives.len(), 4);
    assert_eq!(line(&result.allied, 0), ("ships_sunk", n(1), n(10)));
    assert_eq!(line(&result.allied, 1), ("ships_crippled", n(0), n(0)));
    assert_eq!(
        line(&result.allied, 2),
        ("aircraft_destroyed", n(121), fx("6.05"))
    );
    assert_eq!(line(&result.allied, 3), ("cargo_landed", n(960), fx("4.8")));
    assert_eq!(line(&result.allied, 4), ("base_open", n(1), n(10)));
    // 10 + 0 + 6.05 + 4.8 + 10 = 30.85.
    assert_eq!(result.allied.total, fx("30.85"));

    // Japanese: sunk 4 + 4 + 1 = 9, crippled 0, aircraft 49 x 0.05 = 2.45,
    // base shut 0.
    assert_eq!(line(&result.japanese, 0), ("ships_sunk", n(3), n(9)));
    assert_eq!(line(&result.japanese, 1), ("ships_crippled", n(0), n(0)));
    assert_eq!(
        line(&result.japanese, 2),
        ("aircraft_destroyed", n(49), fx("2.45"))
    );
    assert_eq!(line(&result.japanese, 3), ("base_open", n(0), n(0)));
    assert_eq!(result.japanese.total, fx("11.45"));

    // 30.85 - 11.45 = 19.4, at least 16: a decisive Allied victory.
    assert_eq!(result.allied.margin, fx("19.4"));
    assert_eq!(result.japanese.margin, fx("-19.4"));
    assert_eq!(result.allied.outcome, Outcome::Victory(Level::Decisive));
    assert_eq!(result.japanese.outcome, Outcome::Defeat(Level::Decisive));
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
