//! Invariants checked at the end of every day.
//!
//! A violation is always a bug in the rules, never a legal game state.
//!
//! The spec lists two more invariants. "Every air group is at exactly one
//! base or on one carrier" arrives with air groups. "No belief-state data was
//! written from anything except a report" holds by construction: see
//! [`crate::belief::BeliefState::apply_report`].

use core::fmt;
use std::collections::BTreeMap;

use crate::fixed::Fixed;
use crate::hex::{Hex, HEX_NM};
use crate::ids::{BaseId, ShipId, TaskForceId};
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
        }
    }
}

/// Returns every invariant the state breaks. Empty means the state is sound.
pub(crate) fn check(truth: &TruthState, map: &Map, ref_data: &RefData) -> Vec<InvariantViolation> {
    let mut violations = Vec::new();

    for ship in truth.ships.values() {
        if ship.fuel_t.is_negative() {
            violations.push(InvariantViolation::NegativeFuel(ship.id));
        }
        if ship.supply_t.is_negative() {
            violations.push(InvariantViolation::NegativeSupplyAboard(ship.id));
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
        }
    }

    let mut listings: BTreeMap<ShipId, u32> = BTreeMap::new();
    let one_hex = Fixed::from_int(HEX_NM);
    for task_force in truth.task_forces.values() {
        if !map.contains(task_force.hex) {
            violations.push(InvariantViolation::TaskForceOffMap(
                task_force.id,
                task_force.hex,
            ));
        }
        if task_force.progress_nm.is_negative() || task_force.progress_nm >= one_hex {
            violations.push(InvariantViolation::TaskForceProgressOutOfRange(
                task_force.id,
            ));
        }
        for ship_id in &task_force.ships {
            *listings.entry(*ship_id).or_insert(0) += 1;
            match truth.ships.get(ship_id) {
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

    violations
}
