//! The five warnings of decision 6 and the least-fuelled ship of part 6.
//!
//! 1. A hand-built world: ranges and distances worked out by hand.
//! 2. Every scenario that raises a warning: each is recomputed from the
//!    view's own state and events, and between them all five kinds occur.

mod support;

use std::collections::BTreeSet;

use sim_agent::briefing;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, ShipClassId, ShipId, TaskForceId};
use sim_core::map::{HexData, Map};
use sim_core::orders::Mission;
use sim_core::refdata::{PortSize, RefData, ShipClass, ShipKind};
use sim_core::state::{Base, Ship, ShipLocation, ShipSystems, Side, TaskForce, TruthState};
use sim_core::World;
use support::{load, play};

fn class() -> ShipClass {
    ShipClass {
        id: ShipClassId::new(1),
        name: "Test cruiser".to_string(),
        kind: ShipKind::Cruiser,
        cruise_speed_kn: Fixed::from_int(16),
        max_speed_kn: Fixed::from_int(32),
        fuel_capacity_t: Fixed::from_int(2000),
        endurance_nm: Fixed::from_int(8000),
        cargo_capacity_t: Fixed::ZERO,
        main_guns: 8,
        gun_rounds_per_min: Fixed::from_int(3),
        gun_rounds_per_gun: 100,
        gun_damage: Fixed::from_int(20),
        torpedo_tubes: 0,
        torpedo_damage: Fixed::ZERO,
        toughness: Fixed::from_int(400),
        radar: false,
        aircraft_capacity: 0,
        anti_aircraft: 0,
    }
}

fn ship(id: u32, force: u32, fuel: i64) -> Ship {
    Ship {
        id: ShipId::new(id),
        name: format!("Ship {id}"),
        class: ShipClassId::new(1),
        side: Side::Allied,
        fuel_t: Fixed::from_int(fuel),
        supply_t: Fixed::ZERO,
        cargo_fuel_t: Fixed::ZERO,
        location: ShipLocation::TaskForce(TaskForceId::new(force)),
        flotation_damage: Fixed::ZERO,
        fire_damage: Fixed::ZERO,
        gun_rounds: Fixed::from_int(800),
        torpedoes_ready: false,
        systems: ShipSystems::ALL_WORKING,
    }
}

/// A line of sea from (0,0) to (12,0) with a base of the Allied side at
/// (0,0), and one more hex at (20,0) that no ship can reach the line from.
/// Four ships hold 4 nautical miles for every ton of fuel (2000 t, 8000 nm).
///
/// * Force 1 at (10,0), 400 nm from the base: ship 1 with 100 t (400 nm,
///   exactly enough) and ship 2 with 50 t (200 nm).
/// * Force 2 at (11,0), 440 nm from the base: ship 3 with 100 t (400 nm).
/// * Force 3 at (20,0), cut off from every base: ship 4 with 500 t.
fn world() -> World {
    let mut map = Map::new();
    for q in 0..=12 {
        map.insert(Hex::new(q, 0), HexData::open_sea());
    }
    map.insert(Hex::new(20, 0), HexData::open_sea());
    let mut ref_data = RefData::default();
    ref_data.port_sizes.insert(
        0,
        PortSize {
            berths: 0,
            berth_rate_t_per_h: Fixed::ZERO,
            lighter_rate_t_per_h: Fixed::from_int(20),
        },
    );
    ref_data.ship_classes.insert(ShipClassId::new(1), class());
    let mut truth = TruthState::default();
    truth.bases.insert(
        BaseId::new(1),
        Base {
            id: BaseId::new(1),
            name: "Home".to_string(),
            side: Side::Allied,
            hex: Hex::new(0, 0),
            supply_t: Fixed::from_int(100),
            anti_aircraft: 0,
            aircraft_repairs_per_day: 0,
            fuel_t: Fixed::ZERO,
        },
    );
    for (force, hex, ships) in [
        (1, 10, vec![(1, 100), (2, 50)]),
        (2, 11, vec![(3, 100)]),
        (3, 20, vec![(4, 500)]),
    ] {
        let mut f = TaskForce::at(
            TaskForceId::new(force),
            format!("Force {force}"),
            Side::Allied,
            Hex::new(hex, 0),
            Mission::SurfaceCombat,
        );
        for (id, fuel) in ships {
            f.ships.insert(ShipId::new(id));
            truth.ships.insert(ShipId::new(id), ship(id, force, fuel));
        }
        truth.task_forces.insert(TaskForceId::new(force), f);
    }
    World::new(1, ref_data, map, truth).expect("a valid setup")
}

#[test]
fn a_ship_that_cannot_reach_a_base_is_warned_with_its_two_figures() {
    let world = world();
    let view = world.side_view(Side::Allied);
    let b = briefing(&view, "hand-built");
    let warned: Vec<(String, Option<String>, Option<u32>, u32)> = b
        .warnings
        .ship_cannot_reach_base
        .iter()
        .map(|w| {
            (
                w.ship.handle.clone(),
                w.nearest_base.as_ref().map(|h| h.handle.clone()),
                w.distance_nm,
                w.range_nm,
            )
        })
        .collect();
    // Ship 1: 100 t x 4 = 400 nm to go 400 nm: no warning. Ship 2: 200 nm
    // against 400. Ship 3: 400 nm against 440. Ship 4: no base reachable.
    assert_eq!(
        warned,
        vec![
            (
                "ship:2".to_string(),
                Some("base:1".to_string()),
                Some(400),
                200
            ),
            (
                "ship:3".to_string(),
                Some("base:1".to_string()),
                Some(440),
                400
            ),
            ("ship:4".to_string(), None, None, 2000),
        ]
    );
    let text = b.to_text();
    assert!(text.contains(
        "- Cannot reach base: ship:2 \"Ship 2\" is 400 miles from base:1 \"Home\", range 200 miles at cruising speed"
    ), "{text}");
    assert!(text.contains(
        "- Cannot reach base: ship:4 \"Ship 4\" has no base it can reach by sea, range 2000 miles at cruising speed"
    ), "{text}");
}

#[test]
fn the_least_fuelled_ship_and_its_range_are_in_the_task_force_line() {
    let world = world();
    let view = world.side_view(Side::Allied);
    let b = briefing(&view, "hand-built");
    let force1 = &b.task_forces[0];
    let least = force1.least_fuelled.as_ref().expect("a ship afloat");
    // 50 t x 8000 / 2000 = 200 nm.
    assert_eq!(least.ship.handle, "ship:2");
    assert_eq!(least.fuel_t.0, Fixed::from_int(50));
    assert_eq!(least.range_nm, Some(200));
    assert_eq!(force1.ships_afloat, 2);
    assert!(b.to_text().contains(
        "  - Least fuelled: ship:2 \"Ship 2\" with 50 t, range 200 miles at cruising speed"
    ));
}

/// The warnings, worked out again from what the view holds.
fn recomputed(world: &World, side: Side) {
    let view = world.side_view(side);
    let b = briefing(&view, "recomputed");
    let yesterday = view.day();
    let spent_of = |base: BaseId| -> Fixed {
        view.events_on_day(yesterday)
            .iter()
            .filter_map(|e| match &e.kind {
                sim_core::side_events::SideEventKind::Whole(
                    sim_core::events::EventKind::BaseSupplySpent {
                        base: b, paid_t, ..
                    },
                ) if *b == base => Some(*paid_t),
                _ => None,
            })
            .fold(Fixed::ZERO, |a, x| a + x)
    };

    let mut low = BTreeSet::new();
    for base in view.bases() {
        let spent = spent_of(base.id);
        let by_days = !spent.is_zero() && base.supply_t.raw() < 3 * spent.raw();
        if view.rules_in_play().supply && (base.supply_t.is_zero() || by_days) {
            low.insert(base.id.to_string());
        }
    }
    let got: BTreeSet<String> = b
        .warnings
        .base_supply_low
        .iter()
        .map(|w| w.base.handle.clone())
        .collect();
    assert_eq!(got, low);
    for w in &b.warnings.base_supply_low {
        let base = view
            .bases()
            .find(|x| x.id.to_string() == w.base.handle)
            .unwrap();
        let spent = spent_of(base.id);
        // Days of supply: tenths, rounded down, by integer arithmetic.
        let expected = (!spent.is_zero())
            .then(|| Fixed::from_raw(base.supply_t.raw() * 10 / spent.raw() * 100_000));
        assert_eq!(w.days_of_supply.map(|d| d.0), expected);
        assert_eq!(w.spent_yesterday_t.0, spent);
    }
    for base in &b.bases {
        let spent = spent_of(
            view.bases()
                .find(|x| x.id.to_string() == base.base.handle)
                .unwrap()
                .id,
        );
        assert_eq!(base.spent_yesterday_t.0, spent);
        let expected = (!spent.is_zero())
            .then(|| Fixed::from_raw(base.supply_t.0.raw() * 10 / spent.raw() * 100_000));
        assert_eq!(base.days_of_supply.map(|d| d.0), expected);
    }

    let mut no_fuel = BTreeSet::new();
    for base in view.bases().filter(|b| b.fuel_t.is_zero()) {
        let here = view.task_forces().any(|f| {
            f.position() == base.hex
                && f.status != sim_core::state::TaskForceStatus::Underway
                && f.ships.iter().any(|s| {
                    view.ships()
                        .any(|x| x.id == *s && x.location != ShipLocation::Sunk)
                })
        });
        if here {
            no_fuel.insert(base.id.to_string());
        }
    }
    let got: BTreeSet<String> = b
        .warnings
        .base_without_fuel_with_force
        .iter()
        .map(|w| w.base.handle.clone())
        .collect();
    assert_eq!(got, no_fuel);

    let short: BTreeSet<String> = view
        .air_groups()
        .filter(|g| g.pilots.is_some_and(|p| p < g.ready))
        .map(|g| g.id.to_string())
        .collect();
    let got: BTreeSet<String> = b
        .warnings
        .air_group_short_of_pilots
        .iter()
        .map(|w| w.air_group.handle.clone())
        .collect();
    assert_eq!(got, short);

    let tired: BTreeSet<String> = view
        .air_groups()
        .filter(|g| {
            view.rules_in_play().fatigue
                && (g.stood_down || g.rest_above.is_some_and(|t| g.fatigue > t))
        })
        .map(|g| g.id.to_string())
        .collect();
    let got: BTreeSet<String> = b
        .warnings
        .air_group_tired
        .iter()
        .map(|w| w.air_group.handle.clone())
        .collect();
    assert_eq!(got, tired);

    // Ships: each warned ship really has less range than its sea distance
    // to the nearest base, and no other ship afloat does.
    let warned: BTreeSet<String> = b
        .warnings
        .ship_cannot_reach_base
        .iter()
        .map(|w| w.ship.handle.clone())
        .collect();
    for w in &b.warnings.ship_cannot_reach_base {
        if let Some(d) = w.distance_nm {
            assert!(w.range_nm < d, "{w:?}");
        }
    }
    for s in view.ships().filter(|s| s.location != ShipLocation::Sunk) {
        if warned.contains(&s.id.to_string()) {
            continue;
        }
        let class = view.ship_class(s.class).unwrap();
        let range = s
            .fuel_t
            .mul_div(class.endurance_nm, class.fuel_capacity_t)
            .trunc();
        let hex = match s.location {
            ShipLocation::TaskForce(f) => {
                view.task_forces().find(|x| x.id == f).unwrap().position()
            }
            ShipLocation::Port(b) => view.bases().find(|x| x.id == b).unwrap().hex,
            ShipLocation::Sunk => unreachable!(),
        };
        let nearest = view
            .bases()
            .filter_map(|base| view.sea_distances_to(base.hex).get(&hex).copied())
            .min();
        if view.bases().next().is_some() {
            if let Some(steps) = nearest {
                assert!(i64::from(steps) * 40 <= range, "{} is not warned", s.id);
            } else {
                panic!("{} reaches no base and is not warned", s.id);
            }
        }
    }
}

#[test]
fn every_warning_is_recomputed_from_the_view_in_the_scenarios_that_raise_it() {
    let names = [
        "01_transit",
        "02_supply_run",
        "06c_fatigue_and_rest",
        "06d1_pilots_and_replacements",
        "06d2_base_supply",
        "07_full_slice",
    ];
    let mut kinds = [0usize; 5];
    for name in names {
        let mut scenario = load(&format!("{name}.toml"), 1942);
        play(&mut scenario, |s| {
            for side in [Side::Allied, Side::Japanese] {
                recomputed(&s.world, side);
                let w = briefing(&s.world.side_view(side), "x").warnings;
                kinds[0] += w.ship_cannot_reach_base.len();
                kinds[1] += w.base_supply_low.len();
                kinds[2] += w.base_without_fuel_with_force.len();
                kinds[3] += w.air_group_short_of_pilots.len();
                kinds[4] += w.air_group_tired.len();
            }
        });
    }
    println!("warnings raised: {kinds:?}");
    assert!(kinds.iter().all(|n| *n > 0), "{kinds:?}");
}
