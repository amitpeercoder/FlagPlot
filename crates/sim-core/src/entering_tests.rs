//! Forces that arrive (decision 0195): what is held apart until its day, in
//! a world built by hand. Scenario 111
//! (`tests/scenario_111_forces_that_arrive.rs`) holds the proof that a whole
//! game plays as the rules say.
//!
//! The world (the open sea of `pictures_tests`, with its Allied base 1 at
//! (0,0), Japanese base 2 at (5,0) and Japanese task force 9 at (8,0)), and:
//!
//! - Allied base 3 at (2,0), with no airfield;
//! - Allied task force 5 at (3,1), held until day 2: ship 6 carries air
//!   group 11 and ship 7 carries land unit 21 (300 men), ship 6 with 777 t
//!   of fuel and the force at 12 fatigue; it is also placed under base 1
//!   with a commander, and on radio silence, where those are asked for;
//! - Allied land unit 22 at base 1 (500 men), held until day 2;
//! - Allied air group 12 at base 1 (10 ready, 2 damaged), held until day 2;
//! - Allied land unit 23 at the Japanese base 2 (400 men), held until day 2:
//!   it waits for a base that is not its side's;
//! - Allied air group 13 at base 3 (8 ready), held until day 2: it waits,
//!   the base taking no aircraft;
//! - Allied land unit 20 on the map from the start, below every held number.

use crate::commander::{Commander, Initiative, NamedCommander};
use crate::entering::{EntryPlace, EntryWait};
use crate::events::EventKind;
use crate::fixed::Fixed;
use crate::hash::hash_world;
use crate::hex::Hex;
use crate::ids::{
    AirGroupId, AircraftTypeId, BaseId, EntityRef, LandUnitId, ShipClassId, ShipId, TaskForceId,
};
use crate::orders::Mission;
use crate::pictures::{CommandRef, NamedSuperior};
use crate::pictures_tests::{world as test_world, BASE1, TF9};
use crate::refdata::{AircraftRole, AircraftType, AttackWeapon};
use crate::side_events::SideEventKind;
use crate::state::{
    AirGroup, AirGroupLocation, Base, LandUnit, Ship, ShipLocation, Side, TaskForce, Troops,
    TruthState,
};
use crate::world::World;

const TF5: TaskForceId = TaskForceId::new(5);
const BASE2: BaseId = BaseId::new(2);
const BASE3: BaseId = BaseId::new(3);

fn hx(q: i32, r: i32) -> Hex {
    Hex::new(q, r)
}

fn unit(raw: u32) -> LandUnitId {
    LandUnitId::new(raw)
}

fn group(raw: u32) -> AirGroupId {
    AirGroupId::new(raw)
}

fn ship(raw: u32) -> ShipId {
    ShipId::new(raw)
}

fn commander() -> Commander {
    Commander {
        name: "Test Admiral".to_string(),
        initiative: Initiative::Bold,
        note: "a test".to_string(),
    }
}

/// The state before anything is held apart.
fn truth_before(base: &World) -> TruthState {
    let mut truth = base.truth.clone();
    truth.bases.insert(
        BASE1,
        Base {
            airfield: true,
            ..Base::new(BASE1, "Base 1".to_string(), Side::Allied, hx(0, 0))
        },
    );
    truth.bases.insert(
        BASE2,
        Base {
            airfield: true,
            ..Base::new(BASE2, "Base 2".to_string(), Side::Japanese, hx(5, 0))
        },
    );
    truth.bases.insert(
        BASE3,
        Base {
            airfield: false,
            ..Base::new(BASE3, "Base 3".to_string(), Side::Allied, hx(2, 0))
        },
    );
    // Task force 5, its ships and what is aboard.
    let mut force = TaskForce::at(
        TF5,
        "Force 5".to_string(),
        Side::Allied,
        hx(3, 1),
        Mission::Transport,
    );
    force.fatigue = Fixed::from_int(12);
    for raw in [6, 7] {
        force.ships.insert(ship(raw));
        let mut s = Ship::new(
            ship(raw),
            format!("Ship {raw}"),
            ShipClassId::new(1),
            Side::Allied,
            ShipLocation::TaskForce(TF5),
        );
        if raw == 6 {
            s.fuel_t = Fixed::from_int(777);
        }
        if raw == 7 {
            s.troops = Some(Troops {
                unit: unit(21),
                men: 300,
            });
        }
        truth.ships.insert(ship(raw), s);
    }
    truth.task_forces.insert(TF5, force);
    let mut carried = AirGroup::new(
        group(11),
        "Group 11".to_string(),
        Side::Allied,
        AirGroupLocation::Ship(ship(6)),
        AircraftTypeId::new(5),
    );
    carried.ready = 6;
    carried.strength = 6;
    truth.air_groups.insert(group(11), carried);
    truth.land_units.insert(
        unit(21),
        LandUnit::new(unit(21), "Unit 21".to_string(), Side::Allied, 0, None),
    );
    // Units at bases.
    for (raw, at, men) in [
        (22, hx(0, 0), 500),
        (23, hx(5, 0), 400),
        (20, hx(0, 0), 100),
    ] {
        truth.land_units.insert(
            unit(raw),
            LandUnit::new(
                unit(raw),
                format!("Unit {raw}"),
                Side::Allied,
                men,
                Some(at),
            ),
        );
    }
    // Air group 13, at base 3, which takes no aircraft, is added only where
    // it is held apart: on the map it would break an invariant.
    let mut at_base = AirGroup::new(
        group(12),
        "Group 12".to_string(),
        Side::Allied,
        AirGroupLocation::Base(BASE1),
        AircraftTypeId::new(5),
    );
    at_base.ready = 10;
    at_base.damaged = 2;
    at_base.strength = 12;
    truth.air_groups.insert(group(12), at_base);
    truth
}

fn ref_data_with_aircraft(base: &World) -> crate::refdata::RefData {
    let mut ref_data = base.ref_data.clone();
    // Room for what task force 5 carries.
    let class = ref_data
        .ship_classes
        .get_mut(&ShipClassId::new(1))
        .expect("the test class");
    class.aircraft_capacity = 10;
    class.troop_capacity_men = 1000;
    ref_data.aircraft_types.insert(
        AircraftTypeId::new(5),
        AircraftType {
            id: AircraftTypeId::new(5),
            name: "Test fighter".to_string(),
            role: AircraftRole::Fighter,
            search_range_nm: Fixed::ZERO,
            sweep_width_nm: Fixed::ZERO,
            strike_range_nm: Fixed::from_int(250),
            attack_weapon: AttackWeapon::None,
            attack_damage: Fixed::ZERO,
            pilot_armour: false,
            self_sealing_tanks: false,
        },
    );
    ref_data
}

/// What a world is built from, with nothing held apart.
fn parts() -> (crate::refdata::RefData, crate::map::Map, TruthState) {
    let base = test_world();
    (
        ref_data_with_aircraft(&base),
        base.map.clone(),
        truth_before(&base),
    )
}

/// The world with everything above held apart until day 2.
fn held_world() -> World {
    let (ref_data, map, mut truth) = parts();
    let mut waiting_group = AirGroup::new(
        group(13),
        "Group 13".to_string(),
        Side::Allied,
        AirGroupLocation::Base(BASE3),
        AircraftTypeId::new(5),
    );
    waiting_group.ready = 8;
    waiting_group.strength = 8;
    truth.air_groups.insert(group(13), waiting_group);
    for key in [
        EntityRef::TaskForce(TF5),
        EntityRef::LandUnit(unit(22)),
        EntityRef::AirGroup(group(12)),
        EntityRef::LandUnit(unit(23)),
        EntityRef::AirGroup(group(13)),
    ] {
        truth.hold_apart(key, 2).expect("held apart");
    }
    World::new(7, ref_data, map, truth).expect("valid setup")
}

fn open(world: &mut World) {
    world
        .open_day(crate::command::Staffs::none(), &mut |_, _| {})
        .expect("the day opens");
}

fn events_of(world: &World, keep: impl Fn(&EventKind) -> bool) -> Vec<crate::events::Event> {
    world
        .events()
        .from_index(0)
        .iter()
        .filter(|event| keep(&event.kind))
        .cloned()
        .collect()
}

#[test]
fn what_is_held_apart_is_not_in_the_state_on_day_1() {
    let mut world = held_world();
    assert_eq!(world.truth.entering.len(), 5);
    world.resolve_day().expect("day 1");
    let truth = &world.truth;
    assert!(!truth.task_forces.contains_key(&TF5));
    assert!(!truth.ships.contains_key(&ship(6)) && !truth.ships.contains_key(&ship(7)));
    assert!(!truth.air_groups.contains_key(&group(11)));
    assert!(!truth.air_groups.contains_key(&group(12)));
    assert!(!truth.land_units.contains_key(&unit(21)));
    assert!(!truth.land_units.contains_key(&unit(22)));
    assert_eq!(truth.entering.len(), 5);
    // Nothing was said of any of it.
    assert!(events_of(&world, |kind| matches!(
        kind,
        EventKind::ForceEntered { .. }
            | EventKind::UnitEntered { .. }
            | EventKind::EntryWaiting { .. }
    ))
    .is_empty());
}

#[test]
fn a_task_force_enters_at_its_hex_before_the_night_with_what_the_scenario_wrote() {
    let mut world = held_world();
    world.resolve_day().expect("day 1");
    open(&mut world);
    let truth = &world.truth;
    // On day 2, after the day opens and before any pulse, it is in the state.
    let force = &truth.task_forces[&TF5];
    assert_eq!(force.position(), hx(3, 1));
    assert_eq!(force.toward_hex, None);
    assert_eq!(force.fatigue, Fixed::from_int(12));
    assert_eq!(truth.ships[&ship(6)].fuel_t, Fixed::from_int(777));
    assert_eq!(
        truth.air_groups[&group(11)].location,
        AirGroupLocation::Ship(ship(6))
    );
    assert_eq!(truth.land_units[&unit(21)].men_ashore, 0);
    assert!(!truth.entering.contains_key(&EntityRef::TaskForce(TF5)));
    // The three units at bases.
    assert_eq!(truth.land_units[&unit(22)].hex, Some(hx(0, 0)));
    assert_eq!(truth.land_units[&unit(22)].men_ashore, 500);
    assert_eq!(
        truth.air_groups[&group(12)].location,
        AirGroupLocation::Base(BASE1)
    );
    // It is told on day 2, with no pulse and no cause.
    let entered = events_of(&world, |kind| {
        matches!(kind, EventKind::ForceEntered { .. })
    });
    assert_eq!(entered.len(), 1);
    assert_eq!(entered[0].day, 2);
    assert_eq!(entered[0].pulse, None);
    assert!(entered[0].causes.is_empty());
    assert_eq!(
        entered[0].kind,
        EventKind::ForceEntered {
            force: TF5,
            side: Side::Allied,
            hex: hx(3, 1),
            base: None,
            ships: vec![ship(6), ship(7)],
            air_groups: vec![group(11)],
            land_units: vec![unit(21)],
        }
    );
}

#[test]
fn entries_are_taken_in_the_order_of_their_keys_each_with_no_cause() {
    let mut world = held_world();
    world.resolve_day().expect("day 1");
    open(&mut world);
    // The force, then the air group at a base, then the land unit; the two
    // that cannot enter wait, each in its turn: air group 13 (an air group
    // before a land unit), then land unit 23.
    let words: Vec<String> = events_of(&world, |kind| {
        matches!(
            kind,
            EventKind::ForceEntered { .. }
                | EventKind::UnitEntered { .. }
                | EventKind::EntryWaiting { .. }
        )
    })
    .iter()
    .map(|event| {
        assert_eq!(event.day, 2);
        assert_eq!(event.pulse, None);
        assert!(event.causes.is_empty());
        match &event.kind {
            EventKind::ForceEntered { force, .. } => format!("force {}", force.raw()),
            EventKind::UnitEntered { unit, .. } => format!("entered {unit}"),
            EventKind::EntryWaiting { unit, .. } => format!("waits {unit}"),
            _ => unreachable!(),
        }
    })
    .collect();
    assert_eq!(
        words,
        vec![
            "force 5".to_string(),
            "entered air_group:12".to_string(),
            "waits air_group:13".to_string(),
            "entered land_unit:22".to_string(),
            "waits land_unit:23".to_string(),
        ]
    );
}

#[test]
fn a_unit_at_a_base_not_its_sides_waits_and_is_told_each_day() {
    let mut world = held_world();
    world.resolve_day().expect("day 1");
    world.resolve_day().expect("day 2");
    world.resolve_day().expect("day 3");
    let waits = |unit: EntityRef| {
        events_of(
            &world,
            |kind| matches!(kind, EventKind::EntryWaiting { unit: u, .. } if *u == unit),
        )
    };
    let land = waits(EntityRef::LandUnit(unit(23)));
    assert_eq!(land.iter().map(|e| e.day).collect::<Vec<_>>(), vec![2, 3]);
    assert_eq!(
        land[0].kind,
        EventKind::EntryWaiting {
            unit: EntityRef::LandUnit(unit(23)),
            side: Side::Allied,
            base: BASE2,
            day: 2,
            wait: EntryWait::NotTheSidesBase,
        }
    );
    // An air group at a base that takes no aircraft waits too.
    let air = waits(EntityRef::AirGroup(group(13)));
    assert_eq!(air.len(), 2);
    assert!(matches!(
        air[1].kind,
        EventKind::EntryWaiting {
            wait: EntryWait::TakesNoAircraft,
            day: 2,
            ..
        }
    ));
    // Neither is on the map; both are still held and marked as waiting.
    assert!(!world.truth.land_units.contains_key(&unit(23)));
    assert!(!world.truth.air_groups.contains_key(&group(13)));
    assert!(world.truth.entering[&EntityRef::LandUnit(unit(23))].waiting);
    assert!(world.truth.entering[&EntityRef::AirGroup(group(13))].waiting);
    // The day from which a waiting unit may be used moves on with the days.
    assert_eq!(
        world.truth.available_from(EntityRef::LandUnit(unit(23))),
        Some(4)
    );
}

#[test]
fn a_waiting_unit_enters_on_the_day_its_base_is_its_sides() {
    let mut world = held_world();
    world.resolve_day().expect("day 1");
    world.resolve_day().expect("day 2");
    // Base 3 gets an airfield; the next day the group enters.
    world.truth.bases.get_mut(&BASE3).expect("base").airfield = true;
    open(&mut world);
    assert!(world.truth.air_groups.contains_key(&group(13)));
    assert!(!world
        .truth
        .entering
        .contains_key(&EntityRef::AirGroup(group(13))));
    let entered = events_of(
        &world,
        |kind| matches!(kind, EventKind::UnitEntered { unit, .. } if *unit == EntityRef::AirGroup(group(13))),
    );
    assert_eq!(entered.len(), 1);
    assert_eq!(entered[0].day, 3);
    assert_eq!(
        entered[0].kind,
        EventKind::UnitEntered {
            unit: EntityRef::AirGroup(group(13)),
            side: Side::Allied,
            base: BASE3,
            hex: hx(2, 0),
            men: 0,
            aircraft: 8,
        }
    );
}

#[test]
fn only_a_sides_own_entries_are_told_to_it() {
    let mut world = held_world();
    world.resolve_day().expect("day 1");
    world.resolve_day().expect("day 2");
    let seen = |side: Side| {
        world
            .side_view(side)
            .events()
            .into_iter()
            .filter(|event| {
                matches!(
                    event.kind,
                    SideEventKind::Whole(
                        EventKind::ForceEntered { .. }
                            | EventKind::UnitEntered { .. }
                            | EventKind::EntryWaiting { .. }
                    )
                )
            })
            .count()
    };
    // Task force 5, groups 12 and 13 (13 waits), land units 22 and
    // 23 (waiting): five events on day 2, all of them the Allied side's.
    assert_eq!(seen(Side::Allied), 5);
    assert_eq!(seen(Side::Japanese), 0);
}

#[test]
fn a_world_with_nothing_held_hashes_as_before_and_one_with_something_held_differs() {
    let (ref_data, map, truth) = parts();
    let plain = World::new(7, ref_data, map, truth).expect("valid setup");
    // Nothing held: the hash does not read the (empty) list.
    let mut again = plain.clone();
    again.truth.entering.clear();
    assert_eq!(hash_world(&plain), hash_world(&again));
    // Held: a different hash, and the same one for the same state.
    let held = held_world();
    assert_ne!(hash_world(&held), hash_world(&plain));
    assert_eq!(hash_world(&held), hash_world(&held_world()));
    // Each field of an entry moves it.
    let mut other = held_world();
    other
        .truth
        .entering
        .get_mut(&EntityRef::LandUnit(unit(22)))
        .expect("entry")
        .day = 3;
    assert_ne!(hash_world(&held), hash_world(&other));
    let mut silent = held_world();
    silent
        .truth
        .entering
        .get_mut(&EntityRef::TaskForce(TF5))
        .expect("entry")
        .radio_silent = true;
    assert_ne!(hash_world(&held), hash_world(&silent));
}

#[test]
fn the_next_land_unit_number_is_above_the_held_one() {
    let world = held_world();
    // The state's highest is 20; the held units are 21, 22 and 23.
    assert_eq!(world.truth.land_units.keys().next_back(), Some(&unit(20)));
    assert_eq!(world.truth.next_land_unit_id(), unit(24));
    // With nothing held, one above the state's highest, as before.
    let (ref_data, map, truth) = parts();
    let plain = World::new(7, ref_data, map, truth).expect("valid setup");
    assert_eq!(plain.truth.next_land_unit_id(), unit(24));
    let mut none = plain.clone();
    none.truth.land_units.retain(|id, _| id.raw() < 21);
    none.truth.ships.retain(|_, s| s.troops.is_none());
    assert_eq!(none.truth.next_land_unit_id(), unit(21));
}

#[test]
fn an_arrival_to_a_held_group_waits_silently_and_is_delivered_after_it_enters() {
    let mut world = held_world();
    // An arrival of 4 aircraft due to group 12 on day 2, the day it enters.
    // It is delivered on the night of day 2, the group being on the map.
    world.truth.arrivals.push(crate::state::Arrival {
        day: 2,
        air_group: group(12),
        aircraft: 4,
        pilots: 0,
        waiting: false,
    });
    // And one due to group 13, which waits for its base: due day 2 too.
    world.truth.arrivals.push(crate::state::Arrival {
        day: 2,
        air_group: group(13),
        aircraft: 3,
        pilots: 0,
        waiting: false,
    });
    world.resolve_day().expect("day 1");
    world.resolve_day().expect("day 2");
    assert_eq!(world.truth.air_groups[&group(12)].ready, 14);
    // Group 13 never entered: its arrival waits, with no event of its own.
    assert!(world
        .truth
        .arrivals
        .iter()
        .any(|a| a.air_group == group(13) && a.waiting));
    assert!(events_of(&world, |kind| matches!(
        kind,
        EventKind::AircraftArrived { air_group, .. } if *air_group == group(13)
    ))
    .is_empty());
    assert!(events_of(&world, |kind| matches!(
        kind,
        EventKind::ArrivalWaiting { .. }
    ))
    .is_empty());
    // The base takes aircraft from day 3; group 13 enters and its arrival is
    // delivered that night.
    world.truth.bases.get_mut(&BASE3).expect("base").airfield = true;
    world.resolve_day().expect("day 3");
    assert_eq!(world.truth.air_groups[&group(13)].ready, 11);
}

#[test]
fn with_command_levels_on_a_force_becomes_a_command_when_it_enters() {
    let (ref_data, map, mut truth) = parts();
    truth
        .hold_apart(EntityRef::TaskForce(TF5), 2)
        .expect("held apart");
    let mut world = World::new(7, ref_data, map, truth).expect("valid setup");
    world
        .turn_command_levels_on(&[NamedSuperior {
            side: Side::Allied,
            command: CommandRef::TaskForce(TF5),
            superior: CommandRef::Base(BASE1),
        }])
        .expect("a held force may be placed under a command of the side");
    world
        .name_commanders(&[NamedCommander {
            side: Side::Allied,
            command: CommandRef::TaskForce(TF5),
            commander: commander(),
        }])
        .expect("a commander for a held force");
    world
        .keep_radio_silence_from_the_start(TF5)
        .expect("a held force may keep silence");
    // Held, it is not a command and is not on radio silence; its entry holds
    // the three things. Each can be said again.
    assert!(world
        .commands(Side::Allied)
        .expect("levels")
        .get(&CommandRef::TaskForce(TF5))
        .is_none());
    assert!(!world.radio_silence(TF5));
    let entry = &world.truth.entering[&EntityRef::TaskForce(TF5)];
    assert_eq!(entry.superior, Some(CommandRef::Base(BASE1)));
    assert_eq!(entry.commander, Some(commander()));
    assert!(entry.radio_silent);
    world
        .keep_radio_silence_from_the_start(TF5)
        .expect("again changes nothing");
    world
        .name_commanders(&[NamedCommander {
            side: Side::Allied,
            command: CommandRef::TaskForce(TF5),
            commander: commander(),
        }])
        .expect("the same commander again changes nothing");
    // A held force named as a superior is not a command at the start.
    let (ref_data, map, mut truth) = parts();
    truth
        .hold_apart(EntityRef::TaskForce(TF5), 2)
        .expect("held");
    let mut refused = World::new(7, ref_data, map, truth).expect("valid setup");
    assert!(refused
        .turn_command_levels_on(&[NamedSuperior {
            side: Side::Allied,
            command: CommandRef::TaskForce(TF9),
            superior: CommandRef::TaskForce(TF5),
        }])
        .is_err());

    world.resolve_day().expect("day 1");
    open(&mut world);
    let post = &world.commands(Side::Allied).expect("levels")[&CommandRef::TaskForce(TF5)];
    assert_eq!(post.superior(), Some(CommandRef::Base(BASE1)));
    assert_eq!(post.commander(), Some(&commander()));
    assert!(world.radio_silence(TF5));
}

#[test]
fn a_force_whose_superior_is_gone_enters_under_the_area_command() {
    let (ref_data, map, mut truth) = parts();
    truth
        .hold_apart(EntityRef::TaskForce(TF5), 2)
        .expect("held");
    let mut world = World::new(7, ref_data, map, truth).expect("valid setup");
    world
        .turn_command_levels_on(&[NamedSuperior {
            side: Side::Allied,
            command: CommandRef::TaskForce(TF5),
            superior: CommandRef::Base(BASE3),
        }])
        .expect("placed");
    // The base is the enemy's by day 2, and so no longer a command of the
    // side.
    world.truth.bases.get_mut(&BASE3).expect("base").side = Side::Japanese;
    world.base_changed_side(BASE3, Side::Allied);
    world.resolve_day().expect("day 1");
    open(&mut world);
    let post = &world.commands(Side::Allied).expect("levels")[&CommandRef::TaskForce(TF5)];
    assert_eq!(post.superior(), Some(CommandRef::Area));
}

#[test]
fn a_picture_carries_the_entries_and_a_world_from_it_brings_them_in() {
    let key = held_world().umpire_key();
    let mut world = held_world();
    world.resolve_day().expect("day 1");
    let picture = world.whole_picture(&key).expect("a picture");
    assert_eq!(picture.truth.entering.len(), 5);
    let mut rebuilt = World::from_picture(
        picture.clone(),
        world.ref_data.clone(),
        world.map.clone(),
        crate::rng::Stream {
            seed: 3,
            domain: crate::rng::Domain { trial: 1, world: 1 },
        },
    )
    .expect("a sound picture");
    assert_eq!(rebuilt.truth.entering.len(), 5);
    open(&mut rebuilt);
    assert!(rebuilt.truth.task_forces.contains_key(&TF5));

    // A number both in the state and held apart is refused, and named.
    let mut twice = picture;
    let copy = twice.truth.entering[&EntityRef::LandUnit(unit(22))].land_units[&unit(22)].clone();
    twice.truth.land_units.insert(unit(22), copy);
    let error = World::from_picture(
        twice,
        world.ref_data.clone(),
        world.map.clone(),
        crate::rng::Stream {
            seed: 3,
            domain: crate::rng::Domain { trial: 1, world: 1 },
        },
    )
    .expect_err("refused");
    assert!(error.to_string().contains("land_unit:22"), "{error}");
}

#[test]
fn a_side_half_carries_its_own_entries_and_two_halves_join_them() {
    let mut world = held_world();
    world.resolve_day().expect("day 1");
    let allied = world.side_view(Side::Allied).own_half().expect("a half");
    let japanese = world.side_view(Side::Japanese).own_half().expect("a half");
    assert_eq!(allied.entering.len(), 5);
    assert!(japanese.entering.is_empty());
    let picture =
        crate::world_picture::WorldPicture::from_halves(allied, japanese).expect("joined");
    assert_eq!(picture.truth.entering.len(), 5);
}

#[test]
fn the_setup_checks_read_what_is_held_apart() {
    // An air group held apart of a type the reference data does not know is
    // refused, as one on the map is; so is a land unit with no ground
    // figures.
    let (mut ref_data, map, mut truth) = parts();
    truth
        .hold_apart(EntityRef::AirGroup(group(12)), 2)
        .expect("held");
    ref_data.aircraft_types.clear();
    // The group on the map (11, aboard a ship) is checked first; take it
    // out so that only the held one is unknown.
    truth.air_groups.remove(&group(11));
    let error = World::new(7, ref_data, map, truth).expect_err("refused");
    assert!(error.to_string().contains("air_group:12"), "{error}");

    let (mut ref_data, map, mut truth) = parts();
    truth
        .hold_apart(EntityRef::LandUnit(unit(22)), 2)
        .expect("held");
    truth.land_units.clear();
    ref_data.ground = None;
    let error = World::new(7, ref_data, map, truth).expect_err("refused");
    assert!(error.to_string().contains("no ground figures"), "{error}");
}

#[test]
fn an_entry_names_its_place() {
    let world = held_world();
    assert_eq!(
        world.truth.entering[&EntityRef::TaskForce(TF5)].place,
        EntryPlace::Hex(hx(3, 1))
    );
    assert_eq!(
        world.truth.entering[&EntityRef::LandUnit(unit(23))].place,
        EntryPlace::Base(BASE2)
    );
}

// ---------------------------------------------------------------------------
// The side's view, the orders and the commands (decision 0195, 8, 13, 14, 15).
// ---------------------------------------------------------------------------

use crate::command::{
    Command, CommandKind, CommandRefusal, ForcesLimit, HoldTarget, IntentKind, Limit,
};
use crate::ids::{IntentId, OrderId};
use crate::orders::{
    AirGroupOrder, BaseOrder, LandUnitOrder, Order, OrderKind, Origin, Posture, RejectReason,
    SpeedSetting, TaskForceOrder,
};

fn order_of(side: Side, day: u32, kind: OrderKind) -> Order {
    Order {
        id: OrderId::new(1),
        side,
        day_issued: day,
        kind,
        origin: Origin::PLAYER,
    }
}

fn to_force() -> OrderKind {
    OrderKind::TaskForce {
        target: TF5,
        order: TaskForceOrder::SetSpeed(SpeedSetting::Cruise),
    }
}

fn not_yet(unit: EntityRef, day: u32, waiting: bool) -> RejectReason {
    RejectReason::NotYetEntered { unit, day, waiting }
}

/// The reason the last command refusal gives, or `None` if the command was
/// taken.
fn command_result(world: &mut World, side: Side, kind: CommandKind) -> Option<CommandRefusal> {
    let before = world.events().from_index(0).len();
    world.take_in_command(
        1,
        side,
        Command {
            kind,
            issuer: crate::orders::Issuer::Player,
        },
    );
    world
        .events()
        .from_index(before)
        .iter()
        .find_map(|e| match &e.kind {
            EventKind::CommandRefused { reason, .. } => Some(*reason),
            _ => None,
        })
}

#[test]
fn a_sides_view_lists_its_own_entries_and_answers_for_its_own_held_units() {
    let world = held_world();
    let allied = world.side_view(Side::Allied);
    let japanese = world.side_view(Side::Japanese);
    assert_eq!(allied.entering().count(), 5);
    assert_eq!(japanese.entering().count(), 0);
    let force = EntityRef::TaskForce(TF5);
    for held in [
        force,
        EntityRef::Ship(ship(6)),
        EntityRef::AirGroup(group(11)),
        EntityRef::LandUnit(unit(21)),
        EntityRef::LandUnit(unit(22)),
        EntityRef::AirGroup(group(12)),
    ] {
        assert!(allied.entry_of(held).is_some(), "{held}");
        assert!(japanese.entry_of(held).is_none(), "{held}");
        assert!(!allied.may_use(held), "{held}");
        assert!(!japanese.may_use(held), "{held}");
        assert_eq!(allied.available_from(held), Some(2), "{held}");
        assert_eq!(japanese.available_from(held), None, "{held}");
    }
    assert_eq!(allied.entry_of(force).map(|e| e.day), Some(2));
    // A unit on the map is in no entry, and may be used.
    assert!(allied.entry_of(EntityRef::LandUnit(unit(20))).is_none());
    assert!(allied.may_use(EntityRef::LandUnit(unit(20))));
    assert!(!japanese.may_use(EntityRef::LandUnit(unit(20))));
    // A held unit is in no hex: the map's answers are for the map.
    assert_eq!(allied.cover(unit(22)), None);
    assert_eq!(allied.men_aboard(unit(21)), None);
    assert_eq!(allied.march_destinations(unit(22)), None);
    assert!(allied.task_forces().all(|f| f.id != TF5));
    assert!(allied.land_units().all(|u| u.id != unit(22)));
}

#[test]
fn a_waiting_unit_is_still_not_usable_and_its_day_is_the_next_try() {
    let mut world = held_world();
    world.resolve_day().expect("day 1");
    world.resolve_day().expect("day 2");
    // Land unit 23 waits for a base that is not its side's.
    let allied = world.side_view(Side::Allied);
    let waiting = EntityRef::LandUnit(unit(23));
    assert!(allied.entry_of(waiting).is_some_and(|e| e.waiting));
    assert!(!allied.may_use(waiting));
    assert_eq!(allied.available_from(waiting), Some(3));
}

#[test]
fn arrivals_to_a_held_group_are_in_its_sides_view_only() {
    let mut world = held_world();
    world.truth.arrivals.push(crate::state::Arrival {
        day: 1,
        air_group: group(12),
        aircraft: 2,
        pilots: 0,
        waiting: false,
    });
    assert_eq!(world.side_view(Side::Allied).arrivals().count(), 1);
    assert_eq!(world.side_view(Side::Japanese).arrivals().count(), 0);
}

#[test]
fn an_order_to_a_held_unit_of_the_side_is_refused_with_its_day() {
    let world = held_world();
    let refused = |kind: OrderKind| world.validate_order(&order_of(Side::Allied, 1, kind));
    // The force.
    assert_eq!(
        refused(to_force()),
        Err(not_yet(EntityRef::TaskForce(TF5), 2, false))
    );
    // A ship of the force, formed into a force by a base.
    assert_eq!(
        refused(OrderKind::Base {
            target: BASE1,
            order: BaseOrder::FormTaskForce {
                ships: vec![ship(6)]
            },
        }),
        Err(not_yet(EntityRef::Ship(ship(6)), 2, false))
    );
    // A group aboard.
    assert_eq!(
        refused(OrderKind::AirGroup {
            target: group(11),
            order: AirGroupOrder::SetAirMission(crate::orders::AirMission::Search),
        }),
        Err(not_yet(EntityRef::AirGroup(group(11)), 2, false))
    );
    // A land unit aboard and a land unit at a base.
    for raw in [21, 22] {
        assert_eq!(
            refused(OrderKind::LandUnit {
                target: unit(raw),
                order: LandUnitOrder::SetPosture {
                    posture: Posture::Defend,
                    when_spent: None,
                },
            }),
            Err(not_yet(EntityRef::LandUnit(unit(raw)), 2, false))
        );
    }
    // A load order naming a held land unit, to a force on the map.
    let mut on_map = held_world();
    on_map.resolve_day().expect("day 1");
    open(&mut on_map);
    // Held unit 23 waits for its base; the force 5 is on the map.
    let load = OrderKind::TaskForce {
        target: TF5,
        order: TaskForceOrder::LoadUnit {
            unit: unit(23),
            cargo: None,
        },
    };
    assert_eq!(
        on_map.validate_order(&order_of(Side::Allied, 2, load)),
        Err(not_yet(EntityRef::LandUnit(unit(23)), 2, true))
    );
}

#[test]
fn a_waiting_unit_gives_the_waiting_words() {
    let mut world = held_world();
    world.resolve_day().expect("day 1");
    world.resolve_day().expect("day 2");
    let reason = world
        .validate_order(&order_of(
            Side::Allied,
            3,
            OrderKind::AirGroup {
                target: group(13),
                order: AirGroupOrder::SetAirMission(crate::orders::AirMission::Search),
            },
        ))
        .expect_err("group 13 waits");
    assert_eq!(reason, not_yet(EntityRef::AirGroup(group(13)), 2, true));
    assert_eq!(
        reason.to_string(),
        "air group 13 is not on the map yet: it was to enter on day 2 and waits for its base"
            .replace("air group 13", &EntityRef::AirGroup(group(13)).to_string())
    );
    assert!(not_yet(EntityRef::TaskForce(TF5), 2, false)
        .to_string()
        .ends_with("is not on the map yet: it enters on day 2"));
}

#[test]
fn the_same_order_on_the_entry_day_is_taken() {
    let mut world = held_world();
    world.resolve_day().expect("day 1");
    open(&mut world);
    assert_eq!(
        world.validate_order(&order_of(Side::Allied, 2, to_force())),
        Ok(())
    );
}

#[test]
fn an_order_naming_the_enemys_held_unit_is_refused_as_a_unit_that_is_not_there() {
    let world = held_world();
    // The Japanese side names the Allied held force, a group and a land unit:
    // each is refused as the same order naming a unit that does not exist
    // is, with no day anywhere.
    let posture = |target| OrderKind::LandUnit {
        target,
        order: LandUnitOrder::SetPosture {
            posture: Posture::Defend,
            when_spent: None,
        },
    };
    let search = |target| OrderKind::AirGroup {
        target,
        order: AirGroupOrder::SetAirMission(crate::orders::AirMission::Search),
    };
    let speed = |target| OrderKind::TaskForce {
        target,
        order: TaskForceOrder::SetSpeed(SpeedSetting::Cruise),
    };
    let refused = |kind| world.validate_order(&order_of(Side::Japanese, 1, kind));
    for (held, nowhere) in [
        (speed(TF5), speed(TaskForceId::new(99))),
        (search(group(11)), search(group(99))),
        (posture(unit(22)), posture(unit(99))),
        (posture(unit(21)), posture(unit(99))),
    ] {
        let reason = refused(held).expect_err("not the Japanese side's");
        assert_eq!(Err(reason.clone()), refused(nowhere));
        assert!(!matches!(
            reason,
            RejectReason::NotYetEntered { .. } | RejectReason::NotYetAvailable { .. }
        ));
    }
    assert_eq!(refused(speed(TF5)), Err(RejectReason::NotYourUnit));
}

#[test]
fn a_unit_with_a_later_day_than_its_entry_is_not_available_after_it_has_entered() {
    let (ref_data, map, mut truth) = parts();
    truth
        .task_forces
        .get_mut(&TF5)
        .expect("force 5")
        .available_from = Some(4);
    truth
        .hold_apart(EntityRef::TaskForce(TF5), 2)
        .expect("held apart");
    let mut world = World::new(7, ref_data, map, truth).expect("valid setup");
    let force = EntityRef::TaskForce(TF5);
    assert_eq!(world.side_view(Side::Allied).available_from(force), Some(4));
    assert_eq!(
        world.validate_order(&order_of(Side::Allied, 1, to_force())),
        Err(not_yet(force, 2, false))
    );
    world.resolve_day().expect("day 1");
    open(&mut world);
    // Entered on day 2; its own day, 4, has not come.
    assert!(world.truth.task_forces.contains_key(&TF5));
    for day in [2, 3] {
        assert_eq!(
            world.validate_order(&order_of(Side::Allied, day, to_force())),
            Err(RejectReason::NotYetAvailable {
                unit: force,
                day: 4
            })
        );
    }
    assert_eq!(
        world.validate_order(&order_of(Side::Allied, 4, to_force())),
        Ok(())
    );
}

#[test]
fn a_forces_limit_an_intent_and_a_hold_may_name_the_sides_own_held_units() {
    let mut world = held_world();
    let intent = IntentId::new(1);
    // An intent given to the held force.
    assert_eq!(
        command_result(
            &mut world,
            Side::Allied,
            CommandKind::SetIntent {
                intent,
                kind: IntentKind::OperateCarriers {
                    force: TF5,
                    hex: hx(2, 1),
                },
            },
        ),
        None
    );
    // A forces limit naming a held ship, force, group and land unit.
    let limit = ForcesLimit {
        ships: [ship(6)].into_iter().collect(),
        task_forces: [TF5].into_iter().collect(),
        air_groups: [group(11), group(12)].into_iter().collect(),
        source: None,
        land_units: [unit(21), unit(22)].into_iter().collect(),
        submarines: Default::default(),
        none_of: Default::default(),
        in_place: false,
    };
    assert_eq!(
        command_result(
            &mut world,
            Side::Allied,
            CommandKind::SetLimit(Limit::Forces { intent, limit }),
        ),
        None
    );
    // Holds.
    for target in [
        HoldTarget::Ship(ship(6)),
        HoldTarget::TaskForce(TF5),
        HoldTarget::AirGroup(group(11)),
        HoldTarget::LandUnit(unit(22)),
    ] {
        assert_eq!(
            command_result(&mut world, Side::Allied, CommandKind::Hold { target }),
            None,
            "{target:?}"
        );
    }
}

#[test]
fn the_other_side_may_not_name_a_held_unit_in_a_command() {
    let mut world = held_world();
    let intent = IntentId::new(1);
    assert_eq!(
        command_result(
            &mut world,
            Side::Japanese,
            CommandKind::SetIntent {
                intent,
                kind: IntentKind::OperateCarriers {
                    force: TF5,
                    hex: hx(2, 1),
                },
            },
        ),
        Some(CommandRefusal::NotYourForce(TF5))
    );
    for (target, refusal) in [
        (
            HoldTarget::Ship(ship(6)),
            CommandRefusal::NotYourShip(ship(6)),
        ),
        (
            HoldTarget::TaskForce(TF5),
            CommandRefusal::NotYourForce(TF5),
        ),
        (
            HoldTarget::AirGroup(group(11)),
            CommandRefusal::NotYourGroup(group(11)),
        ),
        (
            HoldTarget::LandUnit(unit(22)),
            CommandRefusal::NotYourLandUnit(unit(22)),
        ),
    ] {
        assert_eq!(
            command_result(&mut world, Side::Japanese, CommandKind::Hold { target }),
            Some(refusal)
        );
    }
}
