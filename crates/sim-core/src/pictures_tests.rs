//! Commands and their pictures (decision 0134): where a command is made, a
//! picture copied and a command ended, in a world built for it. Scenario 61
//! (`tests/scenario_61_commands_and_their_pictures.rs`) holds the proof that
//! a whole game plays the same with command levels on.
//!
//! The world: open sea; Allied task force 1 (ships 1 and 2) at (0,0);
//! Allied task force 2 (ship 5) at (1,0); Allied base 1 at (0,0) with ships 3
//! and 4 in port there in no force; Japanese base 2 at (5,0) and Japanese
//! task force 9 (ship 9) at (8,0). The pictures are told apart by the
//! estimate of the enemy's men ashore each holds, which a test writes to one
//! picture alone.

use std::collections::BTreeSet;

use crate::belief::{GroundEstimate, GroundReport, Report};
use crate::events::EventKind;
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{BaseId, ShipClassId, ShipId, TaskForceId};
use crate::map::{HexData, Map};
use crate::orders::Mission;
use crate::pictures::{CommandRef, NamedSuperior};
use crate::refdata::{GroundFigures, PortSize, RefData, SearchFigures, ShipClass, ShipKind};
use crate::state::{Base, Ship, ShipLocation, Side, TaskForce, TruthState};
use crate::world::World;

const TF1: TaskForceId = TaskForceId::new(1);
const TF2: TaskForceId = TaskForceId::new(2);
const TF9: TaskForceId = TaskForceId::new(9);
const BASE1: BaseId = BaseId::new(1);
const BASE2: BaseId = BaseId::new(2);

fn ship(raw: u32) -> ShipId {
    ShipId::new(raw)
}

fn hx(q: i32, r: i32) -> Hex {
    Hex::new(q, r)
}

fn tf(id: TaskForceId) -> CommandRef {
    CommandRef::TaskForce(id)
}

fn world() -> World {
    let mut map = Map::new();
    for q in -1..=12 {
        for r in -1..=5 {
            map.insert(Hex::new(q, r), HexData::open_sea());
        }
    }
    let class = ShipClass {
        id: ShipClassId::new(1),
        name: "Test class".to_string(),
        kind: ShipKind::Cruiser,
        cruise_speed_kn: Fixed::from_int(16),
        max_speed_kn: Fixed::from_int(32),
        fuel_capacity_t: Fixed::from_int(2000),
        endurance_nm: Fixed::from_int(8000),
        cargo_capacity_t: Fixed::ZERO,
        main_guns: 0,
        gun_rounds_per_min: Fixed::ZERO,
        gun_rounds_per_gun: 0,
        gun_damage: Fixed::ZERO,
        torpedo_tubes: 0,
        torpedo_tubes_one_side: 0,
        torpedo_damage: Fixed::ZERO,
        toughness: Fixed::from_int(100),
        radar: false,
        aircraft_capacity: 0,
        anti_aircraft: 0,
        troop_capacity_men: 0,
    };
    let mut ref_data = RefData::default();
    ref_data.ship_classes.insert(class.id, class);
    ref_data.port_sizes.insert(
        0,
        PortSize {
            berths: 0,
            berth_rate_t_per_h: Fixed::ZERO,
            lighter_rate_t_per_h: Fixed::from_int(20),
            fuel_berth_rate_t_per_h: None,
            fuel_lighter_rate_t_per_h: None,
            ammunition_berth_rate_t_per_h: None,
            ammunition_lighter_rate_t_per_h: None,
        },
    );
    ref_data.search = Some(SearchFigures::from_search_toml());
    ref_data.ground = Some(GroundFigures::FIRST);

    let mut truth = TruthState::default();
    let mut add_force = |id: TaskForceId, side: Side, at: Hex, ships: &[u32]| {
        let mut force = TaskForce::at(
            id,
            format!("Force {}", id.raw()),
            side,
            at,
            Mission::Transport,
        );
        for raw in ships {
            force.ships.insert(ship(*raw));
        }
        truth.task_forces.insert(id, force);
        for raw in ships {
            truth.ships.insert(
                ship(*raw),
                Ship::new(
                    ship(*raw),
                    format!("Ship {raw}"),
                    ShipClassId::new(1),
                    side,
                    ShipLocation::TaskForce(id),
                ),
            );
        }
    };
    add_force(TF1, Side::Allied, hx(0, 0), &[1, 2]);
    add_force(TF2, Side::Allied, hx(1, 0), &[5]);
    add_force(TF9, Side::Japanese, hx(8, 0), &[9]);
    for raw in [3, 4] {
        truth.ships.insert(
            ship(raw),
            Ship::new(
                ship(raw),
                format!("Ship {raw}"),
                ShipClassId::new(1),
                Side::Allied,
                ShipLocation::Port(BASE1),
            ),
        );
    }
    truth.bases.insert(
        BASE1,
        Base::new(BASE1, "Base 1".to_string(), Side::Allied, hx(0, 0)),
    );
    truth.bases.insert(
        BASE2,
        Base::new(BASE2, "Base 2".to_string(), Side::Japanese, hx(5, 0)),
    );
    World::new(7, ref_data, map, truth).expect("valid setup")
}

fn world_with_levels(named: &[NamedSuperior]) -> World {
    let mut world = world();
    world
        .turn_command_levels_on(named)
        .expect("command levels turn on");
    world
}

/// The estimates a command's picture holds, from the umpire's table.
fn held(world: &World, side: Side, command: CommandRef) -> Vec<GroundEstimate> {
    world.commands(side).expect("levels on")[&command]
        .picture()
        .ground_estimates()
        .copied()
        .collect()
}

fn has_command(world: &World, side: Side, command: CommandRef) -> bool {
    world
        .commands(side)
        .expect("levels on")
        .contains_key(&command)
}

fn report(world: &World, at: Hex, men: u32) -> Report {
    Report::Ground(GroundReport {
        hex: at,
        men,
        day: 0,
        received_event: world.events.next_id(),
    })
}

/// Writes an estimate to one command's picture alone.
fn tell_only(world: &mut World, side: Side, command: CommandRef, at: Hex, men: u32) {
    let figures = world.ref_data.search.clone().expect("figures");
    let r = report(world, at, men);
    world
        .chain
        .as_mut()
        .expect("levels on")
        .get_mut(&side)
        .expect("a side")
        .get_mut(&command)
        .expect("a command")
        .picture
        .apply_report(&r, &figures);
}

fn superior(world: &World, side: Side, command: CommandRef) -> Option<CommandRef> {
    world.commands(side).expect("levels on")[&command].superior()
}

fn drain_ships(world: &mut World, force: TaskForceId, sunk: bool) {
    let ships: Vec<ShipId> = world.truth.task_forces[&force]
        .ships
        .iter()
        .copied()
        .collect();
    world
        .truth
        .task_forces
        .get_mut(&force)
        .expect("force")
        .ships
        .clear();
    if sunk {
        for s in ships {
            world.truth.ships.get_mut(&s).expect("ship").location = ShipLocation::Sunk;
        }
    }
}

// ------------------------------------------------------------ Forming

#[test]
fn a_force_detached_from_task_force_1_is_a_command_under_the_area_with_a_copy_of_its_picture() {
    let mut world = world_with_levels(&[]);
    tell_only(&mut world, Side::Allied, tf(TF1), hx(3, 3), 120);
    tell_only(&mut world, Side::Allied, CommandRef::Area, hx(4, 4), 40);
    let new = world.split_task_force(TF1, &BTreeSet::from([ship(2)]), "Detached".to_string());
    assert!(has_command(&world, Side::Allied, tf(new)));
    assert_eq!(
        superior(&world, Side::Allied, tf(new)),
        Some(CommandRef::Area)
    );
    // A copy of task force 1's, not of the area command's.
    assert_eq!(
        held(&world, Side::Allied, tf(new)),
        held(&world, Side::Allied, tf(TF1))
    );
    assert_eq!(held(&world, Side::Allied, tf(new)).len(), 1);
    assert_eq!(held(&world, Side::Allied, tf(new))[0].hex, hx(3, 3));
    assert_ne!(
        held(&world, Side::Allied, tf(new)),
        held(&world, Side::Allied, CommandRef::Area)
    );
    // Task force 1 stays a command, and so does everything else.
    assert!(has_command(&world, Side::Allied, tf(TF1)));
    // It is a picture of its own from then on: a report reaches it as it
    // reaches the others, and one told to the parent alone does not.
    tell_only(&mut world, Side::Allied, tf(TF1), hx(2, 2), 7);
    assert_eq!(held(&world, Side::Allied, tf(new)).len(), 1);
    assert_eq!(held(&world, Side::Allied, tf(TF1)).len(), 2);
    // Nothing is made for the Japanese side.
    assert!(!has_command(&world, Side::Japanese, tf(new)));
}

#[test]
fn a_force_formed_in_port_gets_a_copy_of_the_area_commands_picture() {
    let mut world = world_with_levels(&[]);
    tell_only(&mut world, Side::Allied, CommandRef::Area, hx(4, 4), 40);
    tell_only(&mut world, Side::Allied, tf(TF1), hx(3, 3), 120);
    let EventKind::TaskForceFormed { task_force, .. } = world.form_task_force(BASE1, &[ship(3)])
    else {
        panic!("a forming gives TaskForceFormed");
    };
    assert!(has_command(&world, Side::Allied, tf(task_force)));
    assert_eq!(
        superior(&world, Side::Allied, tf(task_force)),
        Some(CommandRef::Area)
    );
    assert_eq!(
        held(&world, Side::Allied, tf(task_force)),
        held(&world, Side::Allied, CommandRef::Area)
    );
    assert_eq!(held(&world, Side::Allied, tf(task_force))[0].hex, hx(4, 4));
}

#[test]
fn a_force_formed_with_command_levels_off_is_no_command() {
    let mut world = world();
    let EventKind::TaskForceFormed { task_force, .. } = world.form_task_force(BASE1, &[ship(3)])
    else {
        panic!("a forming gives TaskForceFormed");
    };
    let new = world.split_task_force(TF1, &BTreeSet::from([ship(2)]), "Detached".to_string());
    assert!(world.commands(Side::Allied).is_none());
    assert!(world.side_view(Side::Allied).commands().is_empty());
    assert!(world.truth.task_forces.contains_key(&task_force));
    assert!(world.truth.task_forces.contains_key(&new));
}

// ------------------------------------------------------------ Ending

fn named(command: CommandRef, superior: CommandRef) -> NamedSuperior {
    NamedSuperior {
        side: Side::Allied,
        command,
        superior,
    }
}

#[test]
fn a_force_joined_to_another_stops_being_a_command_at_the_end_of_the_pulse_and_those_under_it_go_under_the_area(
) {
    let mut world = world_with_levels(&[
        named(tf(TF2), tf(TF1)),
        named(CommandRef::Base(BASE1), tf(TF1)),
    ]);
    assert_eq!(superior(&world, Side::Allied, tf(TF2)), Some(tf(TF1)));
    // Task force 2 is moved to base 1's hex so that both are stopped at one
    // base, as joining needs.
    world
        .truth
        .task_forces
        .get_mut(&TF2)
        .expect("force")
        .from_hex = hx(0, 0);
    let _ = world.join_task_force(TF1, TF2);
    // The join alone ends nothing: the commands end at the end of the pulse.
    assert!(has_command(&world, Side::Allied, tf(TF1)));
    world.end_gone_commands();
    assert!(!has_command(&world, Side::Allied, tf(TF1)));
    assert!(has_command(&world, Side::Allied, tf(TF2)));
    assert_eq!(
        superior(&world, Side::Allied, tf(TF2)),
        Some(CommandRef::Area)
    );
    assert_eq!(
        superior(&world, Side::Allied, CommandRef::Base(BASE1)),
        Some(CommandRef::Area)
    );
}

#[test]
fn a_force_whose_ships_are_all_sunk_stops_being_a_command() {
    let mut world = world_with_levels(&[named(tf(TF2), tf(TF1))]);
    drain_ships(&mut world, TF1, true);
    world.end_gone_commands();
    assert!(!has_command(&world, Side::Allied, tf(TF1)));
    assert!(has_command(&world, Side::Allied, tf(TF2)));
    assert_eq!(
        superior(&world, Side::Allied, tf(TF2)),
        Some(CommandRef::Area)
    );
    // Its picture is gone with it, and the others hold theirs.
    assert!(has_command(&world, Side::Allied, CommandRef::Area));
    assert!(has_command(&world, Side::Allied, CommandRef::Base(BASE1)));
}

#[test]
fn a_force_whose_ships_are_at_a_yard_is_not_ended() {
    let mut world = world_with_levels(&[]);
    // Both ships away at the yard, to come back to the force (decision 0069).
    drain_ships(&mut world, TF1, false);
    for raw in [1, 2] {
        world
            .truth
            .ships
            .get_mut(&ship(raw))
            .expect("ship")
            .location = ShipLocation::Yard(TF1);
    }
    world.end_gone_commands();
    assert!(has_command(&world, Side::Allied, tf(TF1)));
    // With the yard emptied too, it is ended.
    for raw in [1, 2] {
        world
            .truth
            .ships
            .get_mut(&ship(raw))
            .expect("ship")
            .location = ShipLocation::Sunk;
    }
    world.end_gone_commands();
    assert!(!has_command(&world, Side::Allied, tf(TF1)));
}

// ------------------------------------------------------------ Bases

#[test]
fn a_base_that_changes_side_becomes_a_command_of_the_side_that_took_it() {
    let mut world = world_with_levels(&[]);
    // The Japanese area command holds an estimate; the Allied base 1 holds
    // another; base 1 is taken by the Japanese side.
    tell_only(&mut world, Side::Japanese, CommandRef::Area, hx(6, 6), 30);
    tell_only(
        &mut world,
        Side::Allied,
        CommandRef::Base(BASE1),
        hx(2, 5),
        9,
    );
    assert!(has_command(&world, Side::Allied, CommandRef::Base(BASE1)));
    assert!(!has_command(
        &world,
        Side::Japanese,
        CommandRef::Base(BASE1)
    ));
    let figures = GroundFigures::FIRST;
    world.change_side(1, BASE1, Side::Japanese, Vec::new(), &figures);
    assert!(!has_command(&world, Side::Allied, CommandRef::Base(BASE1)));
    assert!(has_command(&world, Side::Japanese, CommandRef::Base(BASE1)));
    assert_eq!(
        superior(&world, Side::Japanese, CommandRef::Base(BASE1)),
        Some(CommandRef::Area)
    );
    // A copy of the Japanese area command's picture, not the Allied base's.
    assert_eq!(
        held(&world, Side::Japanese, CommandRef::Base(BASE1)),
        held(&world, Side::Japanese, CommandRef::Area)
    );
    assert_eq!(
        held(&world, Side::Japanese, CommandRef::Base(BASE1))[0].hex,
        hx(6, 6)
    );
    // The commands of the side that lost it are otherwise whole.
    assert!(has_command(&world, Side::Allied, CommandRef::Area));
    assert!(has_command(&world, Side::Allied, tf(TF1)));
}

#[test]
fn a_base_that_changes_side_with_command_levels_off_makes_no_command() {
    let mut world = world();
    world.change_side(1, BASE1, Side::Japanese, Vec::new(), &GroundFigures::FIRST);
    assert!(world.commands(Side::Allied).is_none());
    assert!(world.commands(Side::Japanese).is_none());
    assert_eq!(world.truth.bases[&BASE1].side, Side::Japanese);
}

#[test]
fn a_base_formed_by_a_landing_is_a_command_with_a_copy_of_the_landing_forces_picture() {
    // The hook `cargo.rs` calls when a landing forms a base, with the base
    // it made: a landing is not played here.
    let mut world = world_with_levels(&[]);
    tell_only(&mut world, Side::Allied, tf(TF1), hx(3, 3), 120);
    tell_only(&mut world, Side::Allied, CommandRef::Area, hx(4, 4), 40);
    let id = BaseId::new(3);
    world.truth.bases.insert(
        id,
        Base::new(id, "Landing at (3,3)".to_string(), Side::Allied, hx(3, 3)),
    );
    world.base_formed(id, tf(TF1));
    assert!(has_command(&world, Side::Allied, CommandRef::Base(id)));
    assert_eq!(
        superior(&world, Side::Allied, CommandRef::Base(id)),
        Some(CommandRef::Area)
    );
    assert_eq!(
        held(&world, Side::Allied, CommandRef::Base(id)),
        held(&world, Side::Allied, tf(TF1))
    );
}

// ------------------------------------------------------------ The view

#[test]
fn with_command_levels_off_there_is_no_command_and_a_view_gives_the_sides_own_belief() {
    let mut world = world();
    assert!(world.commands(Side::Allied).is_none());
    assert!(world.commands(Side::Japanese).is_none());
    assert!(!world.command_levels());
    let figures = world.ref_data.search.clone().expect("figures");
    let r = report(&world, hx(3, 3), 120);
    world.apply_report(Side::Allied, &r, &figures);
    let view = world.side_view(Side::Allied);
    assert!(!view.command_levels());
    assert!(view.commands().is_empty());
    let own: Vec<GroundEstimate> = view.ground_estimates().copied().collect();
    assert_eq!(own.len(), 1);
    for command in [CommandRef::Area, tf(TF1), tf(TF2), CommandRef::Base(BASE1)] {
        let v = view.command(command).expect("an own command has a view");
        assert_eq!(v.superior(), None);
        assert_eq!(v.ground_estimates().copied().collect::<Vec<_>>(), own);
        assert!(v.contacts().eq(view.contacts()));
        assert_eq!(v.claims(), view.claims());
    }
    // An enemy force or base, or one that does not exist, has none.
    for command in [tf(TF9), CommandRef::Base(BASE2), tf(TaskForceId::new(77))] {
        assert!(view.command(command).is_none(), "{command:?}");
    }
}

#[test]
fn a_side_is_given_no_view_of_the_other_sides_commands_with_levels_on_or_off() {
    for levels in [true, false] {
        let world = if levels {
            world_with_levels(&[])
        } else {
            world()
        };
        let allied = world.side_view(Side::Allied);
        let japanese = world.side_view(Side::Japanese);
        for command in [tf(TF9), CommandRef::Base(BASE2)] {
            assert!(
                allied.command(command).is_none(),
                "levels {levels}: {command:?}"
            );
        }
        for command in [tf(TF1), tf(TF2), CommandRef::Base(BASE1)] {
            assert!(
                japanese.command(command).is_none(),
                "levels {levels}: {command:?}"
            );
        }
        assert!(allied.command(CommandRef::Area).is_some());
        assert!(japanese.command(CommandRef::Area).is_some());
    }
}

#[test]
fn with_command_levels_on_a_report_reaches_every_picture_of_the_side_and_no_other() {
    let mut world = world_with_levels(&[]);
    let figures = world.ref_data.search.clone().expect("figures");
    let r = report(&world, hx(3, 3), 120);
    world.apply_report(Side::Allied, &r, &figures);
    let own: Vec<GroundEstimate> = world
        .side_view(Side::Allied)
        .ground_estimates()
        .copied()
        .collect();
    assert_eq!(own.len(), 1);
    for command in world.side_view(Side::Allied).commands() {
        assert_eq!(held(&world, Side::Allied, command), own, "{command:?}");
    }
    for command in world.side_view(Side::Japanese).commands() {
        assert!(
            held(&world, Side::Japanese, command).is_empty(),
            "{command:?}"
        );
    }
}
