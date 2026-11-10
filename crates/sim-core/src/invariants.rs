//! Invariants checked at the end of every day.
//!
//! A violation is always a bug in the rules, never a legal game state.
//!
//! The spec lists three more invariants. "Aircraft ready is never negative"
//! holds by construction: `ready` is unsigned. "Every air group is at exactly
//! one base" holds by construction too, since an air group has one `base`
//! field; the checks here are that the base exists and is the group's own
//! side's. (Carriers arrive later.) "No belief-state data was written from
//! anything except a report" holds by construction: see
//! [`crate::belief::BeliefState::apply_report`].

use core::fmt;
use std::collections::BTreeMap;

use crate::fixed::Fixed;
use crate::hex::{Hex, HEX_NM};
use crate::ids::{AirGroupId, BaseId, ShipId, TaskForceId};
use crate::map::Map;
use crate::refdata::RefData;
use crate::state::{ShipLocation, TruthState};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum InvariantViolation {
    NegativeFuel(ShipId),
    /// The ship says it is in a task force that does not list it.
    ShipNotListedByItsTaskForce(ShipId, TaskForceId),
    /// The ship says it is in a port that does not exist.
    ShipInUnknownPort(ShipId, BaseId),
    /// A task force lists a ship that is somewhere else or does not exist.
    TaskForceListsStrayShip(TaskForceId, ShipId),
    /// More than one task force lists the ship.
    ShipInSeveralTaskForces(ShipId),
    TaskForceHasShipOfOtherSide(TaskForceId, ShipId),
    TaskForceOffMap(TaskForceId, Hex),
    /// Progress toward the next hex is negative or a full hex or more.
    TaskForceProgressOutOfRange(TaskForceId),
    BaseOffMap(BaseId, Hex),
    NegativeSupplyAboard(ShipId),
    /// A ship carries more supply than its class's cargo capacity.
    ShipOverCargoCapacity(ShipId),
    NegativeBaseStock(BaseId),
    /// `toward_hex` is set with no progress, or progress is made with no
    /// `toward_hex`.
    TaskForceLegStateInconsistent(TaskForceId),
    /// `toward_hex` is not a neighbour of `from_hex`.
    TaskForceTowardNotAdjacent(TaskForceId, Hex),
    /// `toward_hex` is off the map.
    TaskForceTowardOffMap(TaskForceId, Hex),
    /// The air group is based at a base that does not exist.
    AirGroupAtUnknownBase(AirGroupId, BaseId),
    /// The air group is based at a base of the other side.
    AirGroupAtOtherSidesBase(AirGroupId, BaseId),
    /// A flotation, system or fire damage track is below 0 or above 100.
    ShipDamageOutOfRange(ShipId),
    NegativeGunRounds(ShipId),
    /// The ship is sunk but a task force still lists it.
    SunkShipInTaskForce(ShipId, TaskForceId),
    /// The ship has flotation damage of 100 and is not sunk.
    ShipNotSunkAtFullFlotationDamage(ShipId),
}

impl fmt::Display for InvariantViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvariantViolation::NegativeFuel(ship) => write!(f, "{ship} has negative fuel"),
            InvariantViolation::ShipNotListedByItsTaskForce(ship, task_force) => {
                write!(f, "{ship} claims {task_force}, which does not list it")
            }
            InvariantViolation::ShipInUnknownPort(ship, base) => {
                write!(f, "{ship} is in {base}, which does not exist")
            }
            InvariantViolation::TaskForceListsStrayShip(task_force, ship) => {
                write!(f, "{task_force} lists {ship}, which is not in it")
            }
            InvariantViolation::ShipInSeveralTaskForces(ship) => {
                write!(f, "{ship} is listed by more than one task force")
            }
            InvariantViolation::TaskForceHasShipOfOtherSide(task_force, ship) => {
                write!(f, "{task_force} holds {ship} of the other side")
            }
            InvariantViolation::TaskForceOffMap(task_force, hex) => {
                write!(f, "{task_force} is at {hex}, which is off the map")
            }
            InvariantViolation::TaskForceProgressOutOfRange(task_force) => {
                write!(f, "{task_force} has progress outside one hex")
            }
            InvariantViolation::BaseOffMap(base, hex) => {
                write!(f, "{base} is at {hex}, which is off the map")
            }
            InvariantViolation::NegativeSupplyAboard(ship) => {
                write!(f, "{ship} has negative supply aboard")
            }
            InvariantViolation::ShipOverCargoCapacity(ship) => {
                write!(f, "{ship} carries more supply than its class can hold")
            }
            InvariantViolation::NegativeBaseStock(base) => {
                write!(f, "{base} has negative supply stock")
            }
            InvariantViolation::TaskForceLegStateInconsistent(task_force) => {
                write!(
                    f,
                    "{task_force} has a hex ahead without progress, or progress without one"
                )
            }
            InvariantViolation::TaskForceTowardNotAdjacent(task_force, hex) => {
                write!(
                    f,
                    "{task_force} is steaming toward {hex}, which is not next to the hex it left"
                )
            }
            InvariantViolation::TaskForceTowardOffMap(task_force, hex) => {
                write!(
                    f,
                    "{task_force} is steaming toward {hex}, which is off the map"
                )
            }
            InvariantViolation::AirGroupAtUnknownBase(group, base) => {
                write!(f, "{group} is based at {base}, which does not exist")
            }
            InvariantViolation::AirGroupAtOtherSidesBase(group, base) => {
                write!(
                    f,
                    "{group} is based at {base}, which belongs to the other side"
                )
            }
            InvariantViolation::ShipDamageOutOfRange(ship) => {
                write!(f, "{ship} has a damage track outside 0 to 100")
            }
            InvariantViolation::NegativeGunRounds(ship) => {
                write!(f, "{ship} has negative gun rounds")
            }
            InvariantViolation::SunkShipInTaskForce(ship, task_force) => {
                write!(f, "{ship} is sunk but {task_force} still lists it")
            }
            InvariantViolation::ShipNotSunkAtFullFlotationDamage(ship) => {
                write!(f, "{ship} has flotation damage of 100 and is not sunk")
            }
        }
    }
}

/// Returns every invariant the state breaks. Empty means the state is sound.
pub(crate) fn check(truth: &TruthState, map: &Map, ref_data: &RefData) -> Vec<InvariantViolation> {
    let mut violations = Vec::new();
    let full_damage = Fixed::from_int(100);

    for ship in truth.ships.values() {
        if ship.fuel_t.is_negative() {
            violations.push(InvariantViolation::NegativeFuel(ship.id));
        }
        if ship.supply_t.is_negative() {
            violations.push(InvariantViolation::NegativeSupplyAboard(ship.id));
        }
        let tracks = [ship.flotation_damage, ship.system_damage, ship.fire_damage];
        if tracks
            .iter()
            .any(|track| *track < Fixed::ZERO || *track > full_damage)
        {
            violations.push(InvariantViolation::ShipDamageOutOfRange(ship.id));
        }
        if ship.gun_rounds.is_negative() {
            violations.push(InvariantViolation::NegativeGunRounds(ship.id));
        }
        if ship.flotation_damage >= full_damage && ship.location != ShipLocation::Sunk {
            violations.push(InvariantViolation::ShipNotSunkAtFullFlotationDamage(
                ship.id,
            ));
        }
        if let Some(class) = ref_data.ship_classes.get(&ship.class) {
            if ship.supply_t > class.cargo_capacity_t {
                violations.push(InvariantViolation::ShipOverCargoCapacity(ship.id));
            }
        }
        match ship.location {
            ShipLocation::TaskForce(task_force_id) => {
                let listed = truth
                    .task_forces
                    .get(&task_force_id)
                    .is_some_and(|task_force| task_force.ships.contains(&ship.id));
                if !listed {
                    violations.push(InvariantViolation::ShipNotListedByItsTaskForce(
                        ship.id,
                        task_force_id,
                    ));
                }
            }
            ShipLocation::Port(base_id) => {
                if !truth.bases.contains_key(&base_id) {
                    violations.push(InvariantViolation::ShipInUnknownPort(ship.id, base_id));
                }
            }
            // Sunk ships are in no set: the task force loop below checks it.
            ShipLocation::Sunk => {}
        }
    }

    let mut listings: BTreeMap<ShipId, u32> = BTreeMap::new();
    let one_hex = Fixed::from_int(HEX_NM);
    for task_force in truth.task_forces.values() {
        if !map.contains(task_force.from_hex) {
            violations.push(InvariantViolation::TaskForceOffMap(
                task_force.id,
                task_force.from_hex,
            ));
        }
        if task_force.toward_hex.is_some() != (task_force.progress_nm.raw() > 0) {
            violations.push(InvariantViolation::TaskForceLegStateInconsistent(
                task_force.id,
            ));
        }
        if let Some(toward) = task_force.toward_hex {
            if task_force.from_hex.distance(toward) != 1 {
                violations.push(InvariantViolation::TaskForceTowardNotAdjacent(
                    task_force.id,
                    toward,
                ));
            }
            if !map.contains(toward) {
                violations.push(InvariantViolation::TaskForceTowardOffMap(
                    task_force.id,
                    toward,
                ));
            }
        }
        if task_force.progress_nm.is_negative() || task_force.progress_nm >= one_hex {
            violations.push(InvariantViolation::TaskForceProgressOutOfRange(
                task_force.id,
            ));
        }
        for ship_id in &task_force.ships {
            *listings.entry(*ship_id).or_insert(0) += 1;
            match truth.ships.get(ship_id) {
                Some(ship) if ship.location == ShipLocation::Sunk => {
                    violations.push(InvariantViolation::SunkShipInTaskForce(
                        *ship_id,
                        task_force.id,
                    ));
                }
                Some(ship) => {
                    if ship.location != ShipLocation::TaskForce(task_force.id) {
                        violations.push(InvariantViolation::TaskForceListsStrayShip(
                            task_force.id,
                            *ship_id,
                        ));
                    }
                    if ship.side != task_force.side {
                        violations.push(InvariantViolation::TaskForceHasShipOfOtherSide(
                            task_force.id,
                            *ship_id,
                        ));
                    }
                }
                None => violations.push(InvariantViolation::TaskForceListsStrayShip(
                    task_force.id,
                    *ship_id,
                )),
            }
        }
    }
    for (ship_id, count) in listings {
        if count > 1 {
            violations.push(InvariantViolation::ShipInSeveralTaskForces(ship_id));
        }
    }

    for base in truth.bases.values() {
        if !map.contains(base.hex) {
            violations.push(InvariantViolation::BaseOffMap(base.id, base.hex));
        }
        if base.supply_t.is_negative() {
            violations.push(InvariantViolation::NegativeBaseStock(base.id));
        }
    }

    for group in truth.air_groups.values() {
        match truth.bases.get(&group.base) {
            None => violations.push(InvariantViolation::AirGroupAtUnknownBase(
                group.id, group.base,
            )),
            Some(base) if base.side != group.side => violations.push(
                InvariantViolation::AirGroupAtOtherSidesBase(group.id, group.base),
            ),
            Some(_) => {}
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::{AircraftTypeId, ShipClassId};
    use crate::map::HexData;
    use crate::orders::{AirMission, Mission};
    use crate::state::{AirGroup, Base, Ship, Side, TaskForce};

    fn setup() -> (TruthState, Map) {
        let mut map = Map::new();
        map.insert(Hex::new(0, 0), HexData::open_sea());
        let mut truth = TruthState::default();
        truth.bases.insert(
            BaseId::new(1),
            Base {
                id: BaseId::new(1),
                name: "Base".to_string(),
                side: Side::Allied,
                hex: Hex::new(0, 0),
                supply_t: Fixed::ZERO,
            },
        );
        truth.air_groups.insert(
            AirGroupId::new(1),
            AirGroup {
                id: AirGroupId::new(1),
                name: "Group".to_string(),
                side: Side::Allied,
                base: BaseId::new(1),
                aircraft_type: AircraftTypeId::new(1),
                ready: 2,
                mission: AirMission::Rest,
                search: None,
                mission_order: None,
                arc_order: None,
            },
        );
        (truth, map)
    }

    #[test]
    fn a_sound_air_group_breaks_nothing() {
        let (truth, map) = setup();
        assert_eq!(check(&truth, &map, &RefData::default()), Vec::new());
    }

    #[test]
    fn an_air_group_at_an_unknown_base_is_caught() {
        let (mut truth, map) = setup();
        truth
            .air_groups
            .get_mut(&AirGroupId::new(1))
            .expect("group")
            .base = BaseId::new(9);
        assert_eq!(
            check(&truth, &map, &RefData::default()),
            vec![InvariantViolation::AirGroupAtUnknownBase(
                AirGroupId::new(1),
                BaseId::new(9)
            )]
        );
    }

    #[test]
    fn an_air_group_at_the_other_sides_base_is_caught() {
        let (mut truth, map) = setup();
        truth
            .air_groups
            .get_mut(&AirGroupId::new(1))
            .expect("group")
            .side = Side::Japanese;
        assert_eq!(
            check(&truth, &map, &RefData::default()),
            vec![InvariantViolation::AirGroupAtOtherSidesBase(
                AirGroupId::new(1),
                BaseId::new(1)
            )]
        );
    }

    const SHIP: ShipId = ShipId::new(1);
    const TF: TaskForceId = TaskForceId::new(1);

    /// The setup plus Allied task force 1 at (0,0) holding ship 1.
    fn setup_with_ship() -> (TruthState, Map) {
        let (mut truth, map) = setup();
        let mut task_force = TaskForce::at(
            TF,
            "Force".to_string(),
            Side::Allied,
            Hex::new(0, 0),
            Mission::SurfaceCombat,
        );
        task_force.ships.insert(SHIP);
        truth.task_forces.insert(TF, task_force);
        truth.ships.insert(
            SHIP,
            Ship {
                id: SHIP,
                name: "Ship".to_string(),
                class: ShipClassId::new(1),
                side: Side::Allied,
                fuel_t: Fixed::from_int(100),
                supply_t: Fixed::ZERO,
                location: ShipLocation::TaskForce(TF),
                flotation_damage: Fixed::ZERO,
                system_damage: Fixed::ZERO,
                fire_damage: Fixed::ZERO,
                gun_rounds: Fixed::from_int(800),
                torpedoes_ready: true,
            },
        );
        (truth, map)
    }

    fn violations_of(truth: &TruthState, map: &Map) -> Vec<InvariantViolation> {
        check(truth, map, &RefData::default())
    }

    fn ship_mut(truth: &mut TruthState) -> &mut Ship {
        truth.ships.get_mut(&SHIP).expect("ship")
    }

    /// Takes the ship out of its task force and marks it sunk, as combat
    /// does.
    fn sink(truth: &mut TruthState) {
        let ship = ship_mut(truth);
        ship.location = ShipLocation::Sunk;
        ship.flotation_damage = Fixed::from_int(100);
        truth
            .task_forces
            .get_mut(&TF)
            .expect("task force")
            .ships
            .remove(&SHIP);
    }

    #[test]
    fn a_sound_ship_and_a_sunk_ship_break_nothing() {
        let (mut truth, map) = setup_with_ship();
        assert_eq!(violations_of(&truth, &map), Vec::new());
        // Damage short of 100 on every track, and 100 on the other two.
        let ship = ship_mut(&mut truth);
        ship.flotation_damage = Fixed::from_int(99);
        ship.system_damage = Fixed::from_int(100);
        ship.fire_damage = Fixed::ZERO;
        assert_eq!(violations_of(&truth, &map), Vec::new());
        sink(&mut truth);
        assert_eq!(violations_of(&truth, &map), Vec::new());
    }

    #[test]
    fn a_damage_track_outside_zero_to_one_hundred_is_caught() {
        for set in [
            (|s: &mut Ship| s.flotation_damage = Fixed::from_raw(-1)) as fn(&mut Ship),
            |s| s.system_damage = Fixed::from_int(101),
            |s| s.system_damage = Fixed::from_raw(-1),
            |s| s.fire_damage = Fixed::from_raw(100_000_001),
            |s| s.fire_damage = Fixed::from_int(-5),
        ] {
            let (mut truth, map) = setup_with_ship();
            set(ship_mut(&mut truth));
            assert_eq!(
                violations_of(&truth, &map),
                vec![InvariantViolation::ShipDamageOutOfRange(SHIP)]
            );
        }
    }

    #[test]
    fn negative_gun_rounds_are_caught() {
        let (mut truth, map) = setup_with_ship();
        ship_mut(&mut truth).gun_rounds = Fixed::from_raw(-1);
        assert_eq!(
            violations_of(&truth, &map),
            vec![InvariantViolation::NegativeGunRounds(SHIP)]
        );
        ship_mut(&mut truth).gun_rounds = Fixed::ZERO;
        assert_eq!(violations_of(&truth, &map), Vec::new());
    }

    #[test]
    fn a_sunk_ship_still_in_a_task_force_is_caught() {
        let (mut truth, map) = setup_with_ship();
        sink(&mut truth);
        truth
            .task_forces
            .get_mut(&TF)
            .expect("task force")
            .ships
            .insert(SHIP);
        assert_eq!(
            violations_of(&truth, &map),
            vec![InvariantViolation::SunkShipInTaskForce(SHIP, TF)]
        );
    }

    #[test]
    fn full_flotation_damage_on_a_ship_that_is_not_sunk_is_caught() {
        let (mut truth, map) = setup_with_ship();
        ship_mut(&mut truth).flotation_damage = Fixed::from_int(100);
        assert_eq!(
            violations_of(&truth, &map),
            vec![InvariantViolation::ShipNotSunkAtFullFlotationDamage(SHIP)]
        );
    }

    #[test]
    fn a_ship_must_be_in_exactly_one_place() {
        // In a task force that does not list it.
        let (mut truth, map) = setup_with_ship();
        truth
            .task_forces
            .get_mut(&TF)
            .expect("task force")
            .ships
            .remove(&SHIP);
        assert_eq!(
            violations_of(&truth, &map),
            vec![InvariantViolation::ShipNotListedByItsTaskForce(SHIP, TF)]
        );

        // Located in a port, and still listed by a task force.
        let (mut truth, map) = setup_with_ship();
        ship_mut(&mut truth).location = ShipLocation::Port(BaseId::new(1));
        assert_eq!(
            violations_of(&truth, &map),
            vec![InvariantViolation::TaskForceListsStrayShip(TF, SHIP)]
        );

        // Listed by two task forces.
        let (mut truth, map) = setup_with_ship();
        let mut second = truth.task_forces[&TF].clone();
        second.id = TaskForceId::new(2);
        truth.task_forces.insert(second.id, second);
        let found = violations_of(&truth, &map);
        assert!(found.contains(&InvariantViolation::ShipInSeveralTaskForces(SHIP)));
    }
}
