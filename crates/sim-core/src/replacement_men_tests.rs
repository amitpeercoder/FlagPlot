//! Men who replace the lost (decision 0197), in a world built by hand: the
//! open sea of `pictures_tests`, with its Allied base 1 at (0,0) and ship 3
//! in port there. Here a unit can have men ashore and aboard at once, or no
//! men at all, which no scenario file can say (scenario 113's statements 6
//! and 6a).

use crate::events::EventKind;
use crate::hash::hash_world;
use crate::ids::{LandUnitId, ShipId};
use crate::invariants::{check, InvariantViolation};
use crate::pictures_tests::{world as test_world, BASE1};
use crate::state::{LandUnit, MenStream, Side, Strength, Troops};
use crate::time::Pulse;
use crate::world::World;

const U1: LandUnitId = LandUnitId::new(1);
const U2: LandUnitId = LandUnitId::new(2);

fn stream(men: u32) -> MenStream {
    MenStream {
        side: Side::Allied,
        base: BASE1,
        men_per_day: men,
        first_day: 1,
        last_day: 10,
    }
}

/// The world with an Allied unit `id` of `ashore` men at base 1, of
/// `experience`, with an establishment of `establishment`.
fn with_unit(world: &mut World, id: LandUnitId, ashore: u32, establishment: u32, experience: u8) {
    let hex = world.truth.bases[&BASE1].hex;
    let mut unit = LandUnit::new(id, format!("Unit {}", id.raw()), Side::Allied, ashore, None);
    if ashore > 0 {
        unit.hex = Some(hex);
    }
    unit.experience = experience;
    world.truth.land_units.insert(id, unit);
    world.truth.men_replacement.units.insert(
        id,
        Strength {
            establishment,
            joined: 0,
            last_fought: None,
        },
    );
}

fn joined(world: &World) -> Vec<(u32, LandUnitId, u32)> {
    world
        .events
        .iter()
        .filter_map(|e| match e.kind {
            EventKind::MenJoined { unit, men, .. } => Some((e.day, unit, men)),
            _ => None,
        })
        .collect()
}

#[test]
fn statement_6a_men_aboard_count_toward_its_establishment() {
    // Unit 1 with 400 ashore at base 1 and 150 aboard ship 3 there, an
    // establishment of 600: it is 50 short and takes 50, not 200.
    let mut world = test_world();
    world.truth.men_replacement.streams.push(stream(100));
    with_unit(&mut world, U1, 400, 600, 2);
    world
        .truth
        .ships
        .get_mut(&ShipId::new(3))
        .expect("ship 3")
        .troops = Some(Troops { unit: U1, men: 150 });
    world.receive_replacement_men(2, Pulse::Night);
    assert_eq!(joined(&world), vec![(2, U1, 50)]);
    assert_eq!(world.truth.land_units[&U1].men_ashore, 450);
    match world
        .events
        .iter()
        .find(|e| matches!(e.kind, EventKind::MenJoined { .. }))
        .map(|e| &e.kind)
    {
        Some(EventKind::MenJoined { men_after, .. }) => assert_eq!(*men_after, 600),
        other => panic!("{other:?}"),
    }
    // Its men are at its establishment and no more. (The test world's
    // ships carry no men by class, so only this invariant is read.)
    assert!(!check(&world.truth, &world.map, &world.ref_data)
        .contains(&InvariantViolation::StrengthUnsound(U1)));
}

#[test]
fn statement_6_a_unit_destroyed_is_not_rebuilt() {
    let mut world = test_world();
    world.truth.men_replacement.streams.push(stream(100));
    with_unit(&mut world, U1, 0, 600, 2);
    with_unit(&mut world, U2, 100, 150, 2);
    world.receive_replacement_men(2, Pulse::Night);
    // Unit 1 has no men and gets none; unit 2 takes its 50.
    assert_eq!(joined(&world), vec![(2, U2, 50)]);
    assert_eq!(world.truth.land_units[&U1].men_ashore, 0);
}

#[test]
fn nothing_on_day_1_nor_to_a_base_of_the_other_side() {
    let mut world = test_world();
    world.truth.men_replacement.streams.push(stream(100));
    with_unit(&mut world, U1, 100, 600, 2);
    world.receive_replacement_men(1, Pulse::Night);
    assert!(joined(&world).is_empty());
    world.truth.bases.get_mut(&BASE1).expect("base").side = Side::Japanese;
    world.receive_replacement_men(2, Pulse::Night);
    assert!(joined(&world).is_empty());
}

#[test]
fn the_count_begins_again_when_the_unit_gains_a_level() {
    let mut world = test_world();
    with_unit(&mut world, U1, 100, 600, 3);
    world
        .truth
        .men_replacement
        .units
        .get_mut(&U1)
        .expect("u1")
        .joined = 250;
    world.note_experience_raised(U1);
    assert_eq!(world.truth.men_replacement.units[&U1].joined, 0);
}

#[test]
fn division_moves_the_establishment_and_keeps_the_count() {
    let mut world = test_world();
    with_unit(&mut world, U1, 400, 600, 3);
    world
        .truth
        .men_replacement
        .units
        .get_mut(&U1)
        .expect("u1")
        .joined = 70;
    world.note_division(U1, U2, 200);
    let units = &world.truth.men_replacement.units;
    assert_eq!((units[&U1].establishment, units[&U1].joined), (400, 70));
    assert_eq!(
        units[&U2],
        Strength {
            establishment: 200,
            joined: 0,
            last_fought: None
        }
    );
    // With nothing kept for the parent, nothing is kept for the new unit.
    let mut plain = test_world();
    plain.note_division(U1, U2, 200);
    assert!(plain.truth.men_replacement.is_empty());
}

#[test]
fn nothing_kept_hashes_as_before_and_anything_kept_changes_the_hash() {
    let world = test_world();
    let before = hash_world(&world);
    let mut kept = test_world();
    with_unit(&mut kept, U1, 100, 600, 2);
    let mut also = test_world();
    let hex = also.truth.bases[&BASE1].hex;
    also.truth.land_units.insert(
        U1,
        LandUnit {
            hex: Some(hex),
            ..LandUnit::new(U1, "Unit 1".to_string(), Side::Allied, 100, None)
        },
    );
    // The same unit with no strength kept hashes as a world without the rule.
    assert_ne!(hash_world(&also), before);
    assert_ne!(hash_world(&kept), hash_world(&also));
    let mut fought = kept.clone();
    fought.note_fought(3, &[U1]);
    assert_ne!(hash_world(&fought), hash_world(&kept));
}

#[test]
fn the_invariants_refuse_a_unit_above_its_establishment_and_a_stream_of_nothing() {
    let mut world = test_world();
    with_unit(&mut world, U1, 700, 600, 2);
    world.truth.men_replacement.streams.push(stream(0));
    let violations = check(&world.truth, &world.map, &world.ref_data);
    assert!(violations.contains(&InvariantViolation::StrengthUnsound(U1)));
    assert!(violations.contains(&InvariantViolation::MenStreamUnsound(BASE1)));
    // A strength kept for a unit that is not there.
    let mut gone = test_world();
    gone.truth.men_replacement.units.insert(
        U2,
        Strength {
            establishment: 1,
            joined: 0,
            last_fought: None,
        },
    );
    assert!(check(&gone.truth, &gone.map, &gone.ref_data)
        .contains(&InvariantViolation::StrengthUnsound(U2)));
}
