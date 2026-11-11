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
use crate::state::{AirGroupLocation, ShipLocation, ShipSystem, SystemState, TruthState};

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
    /// The air group is aboard a ship that does not exist.
    AirGroupOnUnknownShip(AirGroupId, ShipId),
    /// The air group is aboard a ship of the other side.
    AirGroupOnOtherSidesShip(AirGroupId, ShipId),
    /// The ready and damaged aircraft of the groups aboard a ship exceed its class's
    /// aircraft capacity (decision 2 of 0025).
    ShipOverAircraftCapacity(ShipId),
    /// A flotation, system or fire damage track is below 0 or above 100.
    ShipDamageOutOfRange(ShipId),
    NegativeGunRounds(ShipId),
    /// The ship is sunk but a task force still lists it.
    SunkShipInTaskForce(ShipId, TaskForceId),
    /// The ship has flotation damage of 100 and is not sunk.
    ShipNotSunkAtFullFlotationDamage(ShipId),
    /// The ship's class lacks the system, and it is not working.
    ShipSystemNotInClass(ShipId, ShipSystem),
    /// The task force's home port is not an existing base.
    TaskForceHomePortUnknown(TaskForceId, BaseId),
    /// The task force's home port is a base of the other side.
    TaskForceHomePortOfOtherSide(TaskForceId, BaseId),
    /// The task force's share to keep is below 0 or above 1.
    TaskForceKeepShareOutOfRange(TaskForceId),
    /// The task force's `from_hex`, or its `toward_hex` when set, is on the
    /// map but ships cannot enter it. A force is always in a navigable hex.
    TaskForceNotNavigable(TaskForceId, Hex),
    /// A force between centres: the step from `from_hex` to `toward_hex`
    /// crosses a closed side (decision 0017, decision 7). Reported only when
    /// the hexes are adjacent, on the map and navigable.
    TaskForceLegCrossesClosedSide(TaskForceId, Hex, Hex),
    /// The air group's fatigue, or its rest threshold, is below 0 or above
    /// 100 (decision 0036). Added at the end of the enum.
    AirGroupFatigueOutOfRange(AirGroupId),
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
            InvariantViolation::TaskForceNotNavigable(task_force, hex) => {
                write!(f, "{task_force} is at {hex}, which ships cannot enter")
            }
            InvariantViolation::AirGroupFatigueOutOfRange(group) => {
                write!(
                    f,
                    "{group} has a fatigue or a rest threshold outside 0 to 100"
                )
            }
            InvariantViolation::TaskForceLegCrossesClosedSide(task_force, from, toward) => {
                write!(
                    f,
                    "{task_force} is between {from} and {toward}, and the side between them is closed to ships"
                )
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
            InvariantViolation::AirGroupOnUnknownShip(group, ship) => {
                write!(f, "{group} is aboard {ship}, which does not exist")
            }
            InvariantViolation::AirGroupOnOtherSidesShip(group, ship) => {
                write!(
                    f,
                    "{group} is aboard {ship}, which belongs to the other side"
                )
            }
            InvariantViolation::ShipOverAircraftCapacity(ship) => {
                write!(
                    f,
                    "{ship} carries more aircraft than its class's aircraft capacity"
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
            InvariantViolation::ShipSystemNotInClass(ship, system) => {
                write!(
                    f,
                    "{ship} has {} damaged but its class does not have one",
                    system.name()
                )
            }
            InvariantViolation::TaskForceHomePortUnknown(task_force, base) => {
                write!(f, "{task_force} has home port {base}, which does not exist")
            }
            InvariantViolation::TaskForceKeepShareOutOfRange(task_force) => {
                write!(f, "{task_force} has a share to keep outside 0 to 1")
            }
            InvariantViolation::TaskForceHomePortOfOtherSide(task_force, base) => {
                write!(
                    f,
                    "{task_force} has home port {base}, which belongs to the other side"
                )
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
        let tracks = [ship.flotation_damage, ship.fire_damage];
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
            for system in ShipSystem::ALL {
                if !class.has_system(system) && ship.systems.get(system) != SystemState::Working {
                    violations.push(InvariantViolation::ShipSystemNotInClass(ship.id, system));
                }
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
        // A force is always in a navigable hex (decision 0017): it is only
        // ever placed in one, and movement only steps onto one.
        if map.contains(task_force.from_hex) && !map.is_navigable(task_force.from_hex) {
            violations.push(InvariantViolation::TaskForceNotNavigable(
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
            let adjacent = task_force.from_hex.distance(toward) == 1;
            if !adjacent {
                violations.push(InvariantViolation::TaskForceTowardNotAdjacent(
                    task_force.id,
                    toward,
                ));
            }
            let mut both_navigable = map.is_navigable(task_force.from_hex);
            if !map.contains(toward) {
                violations.push(InvariantViolation::TaskForceTowardOffMap(
                    task_force.id,
                    toward,
                ));
                both_navigable = false;
            } else if !map.is_navigable(toward) {
                violations.push(InvariantViolation::TaskForceNotNavigable(
                    task_force.id,
                    toward,
                ));
                both_navigable = false;
            }
            // The leg itself must be an open step. Only reported when the
            // checks above passed, so a bad leg is not reported twice.
            if adjacent && both_navigable {
                let open = Hex::DIRECTIONS
                    .iter()
                    .position(|(dq, dr)| {
                        task_force.from_hex.q + dq == toward.q
                            && task_force.from_hex.r + dr == toward.r
                    })
                    .is_some_and(|direction| map.can_step(task_force.from_hex, direction));
                if !open {
                    violations.push(InvariantViolation::TaskForceLegCrossesClosedSide(
                        task_force.id,
                        task_force.from_hex,
                        toward,
                    ));
                }
            }
        }
        let share = task_force.rules_of_engagement.keep_share;
        if share < Fixed::ZERO || share > Fixed::ONE {
            violations.push(InvariantViolation::TaskForceKeepShareOutOfRange(
                task_force.id,
            ));
        }
        if let Some(port) = task_force.home_port {
            match truth.bases.get(&port) {
                None => violations.push(InvariantViolation::TaskForceHomePortUnknown(
                    task_force.id,
                    port,
                )),
                Some(base) if base.side != task_force.side => violations.push(
                    InvariantViolation::TaskForceHomePortOfOtherSide(task_force.id, port),
                ),
                Some(_) => {}
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

    let mut aboard: BTreeMap<ShipId, u32> = BTreeMap::new();
    for group in truth.air_groups.values() {
        let in_range = |value: Fixed| value >= Fixed::ZERO && value <= full_damage;
        if !in_range(group.fatigue) || group.rest_above.is_some_and(|level| !in_range(level)) {
            violations.push(InvariantViolation::AirGroupFatigueOutOfRange(group.id));
        }
        match group.location {
            AirGroupLocation::Base(base_id) => match truth.bases.get(&base_id) {
                None => {
                    violations.push(InvariantViolation::AirGroupAtUnknownBase(group.id, base_id))
                }
                Some(base) if base.side != group.side => violations.push(
                    InvariantViolation::AirGroupAtOtherSidesBase(group.id, base_id),
                ),
                Some(_) => {}
            },
            AirGroupLocation::Ship(ship_id) => match truth.ships.get(&ship_id) {
                None => {
                    violations.push(InvariantViolation::AirGroupOnUnknownShip(group.id, ship_id))
                }
                Some(ship) => {
                    if ship.side != group.side {
                        violations.push(InvariantViolation::AirGroupOnOtherSidesShip(
                            group.id, ship_id,
                        ));
                    }
                    // Damaged aircraft take room aboard like ready ones (0035).
                    *aboard.entry(ship_id).or_insert(0) += group.ready + group.damaged;
                }
            },
        }
    }
    for (ship_id, aircraft) in aboard {
        // A ship of an unknown class is reported by `World::new`.
        let capacity = truth
            .ships
            .get(&ship_id)
            .and_then(|ship| ref_data.ship_classes.get(&ship.class))
            .map_or(0, |class| class.aircraft_capacity);
        if aircraft > capacity {
            violations.push(InvariantViolation::ShipOverAircraftCapacity(ship_id));
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
    use crate::refdata::{ShipClass, ShipKind};
    use crate::state::{AirGroup, AirGroupLocation, Base, Ship, ShipSystems, Side, TaskForce};

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
                anti_aircraft: 0,
                aircraft_repairs_per_day: 0,
            },
        );
        truth.air_groups.insert(
            AirGroupId::new(1),
            AirGroup {
                id: AirGroupId::new(1),
                name: "Group".to_string(),
                side: Side::Allied,
                location: AirGroupLocation::Base(BaseId::new(1)),
                aircraft_type: AircraftTypeId::new(1),
                ready: 2,
                damaged: 0,
                mission: AirMission::Rest,
                search: None,
                mission_order: None,
                arc_order: None,
                patrol_share: Fixed::ONE,
                share_order: None,
                fatigue: Fixed::ZERO,
                rest_above: None,
                rest_order: None,
                stood_down: false,
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
    fn a_fatigue_or_threshold_outside_0_to_100_is_caught() {
        for (fatigue, threshold, bad) in [
            (Fixed::ZERO, None, false),
            (Fixed::from_int(100), Some(Fixed::from_int(100)), false),
            (Fixed::from_int(-1), None, true),
            (Fixed::from_raw(100_000_001), None, true),
            (Fixed::ZERO, Some(Fixed::from_int(101)), true),
            (Fixed::ZERO, Some(Fixed::from_int(-1)), true),
        ] {
            let (mut truth, map) = setup();
            let group = truth
                .air_groups
                .get_mut(&AirGroupId::new(1))
                .expect("group");
            group.fatigue = fatigue;
            group.rest_above = threshold;
            let expected = if bad {
                vec![InvariantViolation::AirGroupFatigueOutOfRange(
                    AirGroupId::new(1),
                )]
            } else {
                Vec::new()
            };
            assert_eq!(check(&truth, &map, &RefData::default()), expected);
        }
    }

    #[test]
    fn an_air_group_at_an_unknown_base_is_caught() {
        let (mut truth, map) = setup();
        truth
            .air_groups
            .get_mut(&AirGroupId::new(1))
            .expect("group")
            .location = AirGroupLocation::Base(BaseId::new(9));
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
                fire_damage: Fixed::ZERO,
                gun_rounds: Fixed::from_int(800),
                torpedoes_ready: true,
                systems: ShipSystems::ALL_WORKING,
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
        ship.systems.propulsion = SystemState::Out;
        ship.fire_damage = Fixed::ZERO;
        assert_eq!(violations_of(&truth, &map), Vec::new());
        sink(&mut truth);
        assert_eq!(violations_of(&truth, &map), Vec::new());
    }

    #[test]
    fn a_damage_track_outside_zero_to_one_hundred_is_caught() {
        for set in [
            (|s: &mut Ship| s.flotation_damage = Fixed::from_raw(-1)) as fn(&mut Ship),
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

    #[test]
    fn a_damaged_system_the_class_lacks_is_caught() {
        let (mut truth, map) = setup_with_ship();
        let mut ref_data = RefData::default();
        // A class with no guns and no tubes: only propulsion exists.
        ref_data.ship_classes.insert(
            ShipClassId::new(1),
            ShipClass {
                id: ShipClassId::new(1),
                name: "Auxiliary".to_string(),
                kind: ShipKind::Auxiliary,
                cruise_speed_kn: Fixed::from_int(10),
                max_speed_kn: Fixed::from_int(12),
                fuel_capacity_t: Fixed::from_int(100),
                endurance_nm: Fixed::from_int(1000),
                cargo_capacity_t: Fixed::from_int(100),
                main_guns: 0,
                gun_rounds_per_min: Fixed::ZERO,
                gun_rounds_per_gun: 0,
                gun_damage: Fixed::ZERO,
                torpedo_tubes: 0,
                torpedo_damage: Fixed::ZERO,
                toughness: Fixed::from_int(100),
                radar: false,
                aircraft_capacity: 0,
                anti_aircraft: 0,
            },
        );
        // Propulsion may be damaged.
        ship_mut(&mut truth).systems.propulsion = SystemState::Out;
        assert_eq!(check(&truth, &map, &ref_data), Vec::new());
        ship_mut(&mut truth).systems.main_battery = SystemState::Degraded;
        ship_mut(&mut truth).systems.torpedo_mounts = SystemState::Out;
        assert_eq!(
            check(&truth, &map, &ref_data),
            vec![
                InvariantViolation::ShipSystemNotInClass(SHIP, ShipSystem::MainBattery),
                InvariantViolation::ShipSystemNotInClass(SHIP, ShipSystem::TorpedoMounts),
            ]
        );
    }

    #[test]
    fn a_keep_share_must_be_from_zero_to_one() {
        let (mut truth, map) = setup_with_ship();
        let set = |truth: &mut TruthState, share: Fixed| {
            truth
                .task_forces
                .get_mut(&TF)
                .expect("task force")
                .rules_of_engagement
                .keep_share = share;
        };
        for ok in [Fixed::ZERO, Fixed::from_ratio(1, 2), Fixed::ONE] {
            set(&mut truth, ok);
            assert_eq!(violations_of(&truth, &map), Vec::new());
        }
        for bad in [Fixed::from_ratio(-1, 100), Fixed::from_ratio(101, 100)] {
            set(&mut truth, bad);
            assert_eq!(
                violations_of(&truth, &map),
                vec![InvariantViolation::TaskForceKeepShareOutOfRange(TF)]
            );
        }
    }

    /// The setup's map gains the sea hex (1,0), east of (0,0), and the force
    /// is on a leg from (0,0) toward it, 10 nm in. `closed_here` closes the
    /// side of (0,0) toward (1,0), direction 0; `closed_there` closes the
    /// side of (1,0) toward (0,0), direction 3.
    fn setup_on_a_leg(closed_here: bool, closed_there: bool) -> (TruthState, Map) {
        let (mut truth, mut map) = setup_with_ship();
        let mut here = HexData::open_sea();
        here.closed_sides[0] = closed_here;
        map.insert(Hex::new(0, 0), here);
        let mut there = HexData::open_sea();
        there.closed_sides[3] = closed_there;
        map.insert(Hex::new(1, 0), there);
        let task_force = truth.task_forces.get_mut(&TF).expect("task force");
        task_force.toward_hex = Some(Hex::new(1, 0));
        task_force.progress_nm = Fixed::from_int(10);
        (truth, map)
    }

    #[test]
    fn a_leg_across_a_closed_side_is_caught_and_the_same_leg_on_an_open_side_is_not() {
        let (truth, map) = setup_on_a_leg(false, false);
        assert_eq!(violations_of(&truth, &map), Vec::new());

        // Closed in either hex's data, or both, the leg is reported once,
        // naming the hex left and the hex ahead.
        for (here, there) in [(true, false), (false, true), (true, true)] {
            let (truth, map) = setup_on_a_leg(here, there);
            assert_eq!(
                violations_of(&truth, &map),
                vec![InvariantViolation::TaskForceLegCrossesClosedSide(
                    TF,
                    Hex::new(0, 0),
                    Hex::new(1, 0)
                )],
                "closed here {here}, closed there {there}"
            );
        }
    }

    #[test]
    fn a_home_port_must_be_a_base_of_the_forces_own_side() {
        let (mut truth, map) = setup_with_ship();
        let set = |truth: &mut TruthState, port: Option<BaseId>| {
            truth
                .task_forces
                .get_mut(&TF)
                .expect("task force")
                .home_port = port;
        };
        set(&mut truth, Some(BaseId::new(1)));
        assert_eq!(violations_of(&truth, &map), Vec::new());
        set(&mut truth, Some(BaseId::new(9)));
        assert_eq!(
            violations_of(&truth, &map),
            vec![InvariantViolation::TaskForceHomePortUnknown(
                TF,
                BaseId::new(9)
            )]
        );
        truth.bases.get_mut(&BaseId::new(1)).expect("base").side = Side::Japanese;
        set(&mut truth, Some(BaseId::new(1)));
        assert!(violations_of(&truth, &map).contains(
            &InvariantViolation::TaskForceHomePortOfOtherSide(TF, BaseId::new(1))
        ));
        set(&mut truth, None);
        truth.bases.get_mut(&BaseId::new(1)).expect("base").side = Side::Allied;
        assert_eq!(violations_of(&truth, &map), Vec::new());
    }

    // ---- air groups aboard ships (decisions 1 and 2 of 0025) ----

    /// The setup with Allied ship 1 (class 1, capacity 10) in task force 1,
    /// and the air group aboard it instead of at the base.
    fn setup_aboard() -> (TruthState, Map, RefData) {
        let (mut truth, map) = setup_with_ship();
        let mut ref_data = RefData::default();
        ref_data.ship_classes.insert(
            ShipClassId::new(1),
            ShipClass {
                id: ShipClassId::new(1),
                name: "Carrier".to_string(),
                kind: ShipKind::Carrier,
                cruise_speed_kn: Fixed::from_int(16),
                max_speed_kn: Fixed::from_int(30),
                fuel_capacity_t: Fixed::from_int(100),
                endurance_nm: Fixed::from_int(1000),
                cargo_capacity_t: Fixed::ZERO,
                main_guns: 0,
                gun_rounds_per_min: Fixed::ZERO,
                gun_rounds_per_gun: 0,
                gun_damage: Fixed::ZERO,
                torpedo_tubes: 0,
                torpedo_damage: Fixed::ZERO,
                toughness: Fixed::from_int(100),
                radar: false,
                aircraft_capacity: 10,
                anti_aircraft: 0,
            },
        );
        truth
            .air_groups
            .get_mut(&AirGroupId::new(1))
            .expect("group")
            .location = AirGroupLocation::Ship(SHIP);
        (truth, map, ref_data)
    }

    #[test]
    fn a_group_aboard_a_ship_within_capacity_breaks_nothing() {
        let (mut truth, map, ref_data) = setup_aboard();
        assert_eq!(check(&truth, &map, &ref_data), Vec::new());
        // Exactly at capacity: 8 and 2 aircraft in two groups.
        truth
            .air_groups
            .get_mut(&AirGroupId::new(1))
            .expect("group")
            .ready = 8;
        let mut second = truth.air_groups[&AirGroupId::new(1)].clone();
        second.id = AirGroupId::new(2);
        second.ready = 2;
        truth.air_groups.insert(second.id, second);
        assert_eq!(check(&truth, &map, &ref_data), Vec::new());
    }

    #[test]
    fn aircraft_aboard_a_ship_over_its_capacity_are_caught() {
        let (mut truth, map, ref_data) = setup_aboard();
        // 8 and 3 aircraft against a capacity of 10.
        truth
            .air_groups
            .get_mut(&AirGroupId::new(1))
            .expect("group")
            .ready = 8;
        let mut second = truth.air_groups[&AirGroupId::new(1)].clone();
        second.id = AirGroupId::new(2);
        second.ready = 3;
        truth.air_groups.insert(second.id, second);
        assert_eq!(
            check(&truth, &map, &ref_data),
            vec![InvariantViolation::ShipOverAircraftCapacity(SHIP)]
        );
    }

    #[test]
    fn damaged_aircraft_take_room_aboard_a_ship_like_ready_ones() {
        let (mut truth, map, ref_data) = setup_aboard();
        // 8 ready and 2 damaged fill a capacity of 10; one more damaged does
        // not fit.
        let group = truth
            .air_groups
            .get_mut(&AirGroupId::new(1))
            .expect("group");
        group.ready = 8;
        group.damaged = 2;
        assert!(check(&truth, &map, &ref_data).is_empty());
        truth
            .air_groups
            .get_mut(&AirGroupId::new(1))
            .expect("group")
            .damaged = 3;
        assert_eq!(
            check(&truth, &map, &ref_data),
            vec![InvariantViolation::ShipOverAircraftCapacity(SHIP)]
        );
    }

    #[test]
    fn a_group_aboard_a_ship_of_no_capacity_is_caught() {
        let (truth, map, mut ref_data) = setup_aboard();
        ref_data
            .ship_classes
            .get_mut(&ShipClassId::new(1))
            .expect("class")
            .aircraft_capacity = 0;
        assert_eq!(
            check(&truth, &map, &ref_data),
            vec![InvariantViolation::ShipOverAircraftCapacity(SHIP)]
        );
    }

    #[test]
    fn a_group_aboard_an_unknown_or_other_sides_ship_is_caught() {
        let (mut truth, map, ref_data) = setup_aboard();
        truth.ships.get_mut(&ShipId::new(1)).expect("ship").side = Side::Japanese;
        assert!(check(&truth, &map, &ref_data).contains(
            &InvariantViolation::AirGroupOnOtherSidesShip(AirGroupId::new(1), SHIP)
        ));
        truth.ships.get_mut(&SHIP).expect("ship").side = Side::Allied;
        truth
            .air_groups
            .get_mut(&AirGroupId::new(1))
            .expect("group")
            .location = AirGroupLocation::Ship(ShipId::new(9));
        assert_eq!(
            check(&truth, &map, &ref_data),
            vec![InvariantViolation::AirGroupOnUnknownShip(
                AirGroupId::new(1),
                ShipId::new(9)
            )]
        );
    }
}
