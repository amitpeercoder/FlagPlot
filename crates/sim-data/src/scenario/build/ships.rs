use crate::error::DataError;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, ShipClassId, ShipId, TaskForceId};
use sim_core::orders::Readiness;
use sim_core::refdata::RefData;
use sim_core::state::{
    Ship, ShipLocation, ShipSystem, ShipSystems, SystemState, TaskForce, TruthState,
};
use sim_core::world::MADE_IDS_FLOOR;
use std::path::Path;

use super::super::available_day;
use super::super::file::ScenarioFile;

pub(super) fn build_task_forces(
    file: &ScenarioFile,
    path: &Path,
    truth: &mut TruthState,
) -> Result<(), DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    for spec in &file.task_force {
        let id = TaskForceId::new(spec.id);
        if spec.id >= MADE_IDS_FLOOR {
            return Err(fail(format!(
                    "{id} is numbered {MADE_IDS_FLOOR} or above: those numbers are kept for forces made in play (decision 0086)"
                )));
        }
        let hex = match (spec.at_base, spec.q, spec.r) {
            (Some(raw), None, None) => {
                truth
                    .bases
                    .get(&BaseId::new(raw))
                    .ok_or_else(|| fail(format!("{id} starts at unknown base {raw}")))?
                    .hex
            }
            (None, Some(q), Some(r)) => Hex::new(q, r),
            _ => {
                return Err(fail(format!(
                    "{id} needs exactly one of at_base, or both q and r"
                )))
            }
        };
        let mut task_force = TaskForce::at(
            id,
            spec.name.clone(),
            spec.side.into(),
            hex,
            spec.mission.into(),
        );
        task_force.readiness = spec.readiness.map_or(Readiness::Normal, Readiness::from);
        if let Some(stance) = spec.stance {
            task_force.rules_of_engagement.stance = stance.into();
        }
        if let Some(quantity) = &spec.fatigue {
            let fatigue = quantity
                .to_fixed()
                .map_err(|e| fail(format!("{id} fatigue {e}")))?;
            if fatigue < Fixed::ZERO || fatigue > Fixed::from_int(100) {
                return Err(fail(format!(
                    "{id} has a fatigue of {fatigue}, not between 0 and 100"
                )));
            }
            task_force.fatigue = fatigue;
        }
        if let Some(raw) = spec.home_port {
            let port = BaseId::new(raw);
            let base = truth
                .bases
                .get(&port)
                .ok_or_else(|| fail(format!("{id} has unknown home port {raw}")))?;
            if base.side != task_force.side {
                return Err(fail(format!(
                    "{id} is {} but its home port {port} is {}",
                    task_force.side.name(),
                    base.side.name()
                )));
            }
            task_force.home_port = Some(port);
        }
        task_force.available_from = available_day(spec.available_from, id).map_err(&fail)?;
        if truth.task_forces.insert(id, task_force).is_some() {
            return Err(fail(format!("{id} appears twice")));
        }
    }
    Ok(())
}

pub(super) fn build_ships(
    file: &ScenarioFile,
    path: &Path,
    truth: &mut TruthState,
    ref_data: &RefData,
) -> Result<(), DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    for spec in &file.ship {
        let id = ShipId::new(spec.id);
        let class = ShipClassId::new(spec.class);
        if !ref_data.ship_classes.contains_key(&class) {
            return Err(fail(format!("{id} has unknown class {}", spec.class)));
        }
        let location = match (spec.task_force, spec.port) {
            (Some(raw), None) => {
                let task_force_id = TaskForceId::new(raw);
                truth
                    .task_forces
                    .get_mut(&task_force_id)
                    .ok_or_else(|| fail(format!("{id} is in unknown task force {raw}")))?
                    .ships
                    .insert(id);
                ShipLocation::TaskForce(task_force_id)
            }
            (None, Some(raw)) => {
                let base_id = BaseId::new(raw);
                if !truth.bases.contains_key(&base_id) {
                    return Err(fail(format!("{id} is in unknown port {raw}")));
                }
                ShipLocation::Port(base_id)
            }
            _ => {
                return Err(fail(format!(
                    "{id} needs exactly one of task_force and port"
                )))
            }
        };
        let ship_class = &ref_data.ship_classes[&class];
        let gun_rounds = match &spec.gun_rounds {
            Some(quantity) => quantity
                .to_fixed()
                .map_err(|e| fail(format!("{id}: gun_rounds {e}")))?,
            None => Fixed::from_int(
                i64::from(ship_class.main_guns) * i64::from(ship_class.gun_rounds_per_gun),
            ),
        };
        let experience = match spec.experience {
            None => Ship::DEFAULT_EXPERIENCE,
            Some(value) => u8::try_from(value)
                .ok()
                .filter(|value| *value <= Ship::MAX_EXPERIENCE)
                .ok_or_else(|| {
                    fail(format!(
                        "{id}: experience {value} is not a whole number from 0 to {}",
                        Ship::MAX_EXPERIENCE
                    ))
                })?,
        };
        let flotation_damage = match &spec.flotation_damage {
            Some(quantity) => {
                let damage = quantity
                    .to_fixed()
                    .map_err(|e| fail(format!("{id}: flotation_damage {e}")))?;
                if damage < Fixed::ZERO || damage >= Fixed::from_int(100) {
                    return Err(fail(format!(
                        "{id}: flotation_damage {damage} is not from 0 to below 100"
                    )));
                }
                damage
            }
            None => Fixed::ZERO,
        };
        let fire_damage = match &spec.fire_damage {
            Some(quantity) => {
                let damage = quantity
                    .to_fixed()
                    .map_err(|e| fail(format!("{id}: fire_damage {e}")))?;
                if damage < Fixed::ZERO || damage > Fixed::from_int(100) {
                    return Err(fail(format!(
                        "{id}: fire_damage {damage} is not from 0 to 100"
                    )));
                }
                damage
            }
            None => Fixed::ZERO,
        };
        let mut systems = ShipSystems::ALL_WORKING;
        for (system, wanted) in [
            (ShipSystem::Propulsion, spec.propulsion),
            (ShipSystem::FireControl, spec.fire_control),
            (ShipSystem::MainBattery, spec.main_battery),
            (ShipSystem::TorpedoMounts, spec.torpedo_mounts),
            (ShipSystem::FlightDeck, spec.flight_deck),
        ] {
            let Some(wanted) = wanted else {
                continue;
            };
            let state: SystemState = wanted.into();
            if state != SystemState::Working && !ship_class.has_system(system) {
                return Err(fail(format!(
                    "{id} is {} {} but its class {} has no {}",
                    system.name(),
                    state.name(),
                    spec.class,
                    system.name()
                )));
            }
            systems.set(system, state);
        }
        let fuel_t = spec
            .fuel_t
            .to_fixed()
            .map_err(|e| fail(format!("{id}: fuel_t {e}")))?;
        // A ship cannot start with more fuel than its class holds. This
        // catches a scenario left behind when a class's figures change.
        if fuel_t > ship_class.fuel_capacity_t {
            return Err(fail(format!(
                "{id} starts with {fuel_t} t of fuel but its class {} holds {} t",
                spec.class, ship_class.fuel_capacity_t
            )));
        }
        let supply_t = match &spec.supply_t {
            Some(quantity) => quantity
                .to_fixed()
                .map_err(|e| fail(format!("{id}: supply_t {e}")))?,
            None => Fixed::ZERO,
        };
        let cargo_fuel_t = match &spec.cargo_fuel_t {
            Some(quantity) => quantity
                .to_fixed()
                .map_err(|e| fail(format!("{id}: cargo_fuel_t {e}")))?,
            None => Fixed::ZERO,
        };
        if cargo_fuel_t.is_negative() {
            return Err(fail(format!("{id}: cargo_fuel_t is negative")));
        }
        // Supply and fuel cargo share the class's cargo capacity
        // (decision 0046).
        if supply_t + cargo_fuel_t > ship_class.cargo_capacity_t {
            return Err(fail(format!(
                    "{id} starts with {supply_t} t of supply and {cargo_fuel_t} t of fuel cargo but its class {} carries {} t",
                    spec.class, ship_class.cargo_capacity_t
                )));
        }
        let tubes_fired = spec.tubes_fired.unwrap_or(0);
        if tubes_fired > ship_class.torpedo_tubes {
            return Err(fail(format!(
                "{id}: tubes_fired {tubes_fired} is more than its class's {} tubes",
                ship_class.torpedo_tubes
            )));
        }
        let ship = Ship {
            id,
            name: spec.name.clone(),
            class,
            side: spec.side.into(),
            fuel_t,
            supply_t,
            cargo_fuel_t,
            location,
            flotation_damage,
            fire_damage,
            gun_rounds,
            torpedoes_ready: tubes_fired < ship_class.torpedo_tubes,
            torpedoes_one_side_fired: false,
            torpedoes_fired: tubes_fired,
            systems,
            experience,
            system_repair_points: Fixed::ZERO,
            troops: None,
            available_from: available_day(spec.available_from, id).map_err(&fail)?,
            aircraft_fire: None,
        };
        if truth.ships.insert(id, ship).is_some() {
            return Err(fail(format!("{id} appears twice")));
        }
    }
    Ok(())
}

pub(super) fn check_class_sides(
    _file: &ScenarioFile,
    path: &Path,
    truth: &TruthState,
    ref_data: &RefData,
) -> Result<(), DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    // No ship or air group is of a class or type the file of sides gives
    // to the other side alone (decision 0173, 2).
    if let Some(sides) = &ref_data.class_sides {
        for ship in truth.ships.values() {
            if let Some(side) = sides.ship_classes.get(&ship.class) {
                if !side.used_by(ship.side) {
                    return Err(fail(format!(
                            "ship {} ({}) is {} and of ship class {}, which the file of sides gives to the {} side alone",
                            ship.id.raw(),
                            ship.name,
                            ship.side.name(),
                            ship.class.raw(),
                            side.name()
                        )));
                }
            }
        }
        for group in truth.air_groups.values() {
            if let Some(side) = sides.aircraft_types.get(&group.aircraft_type) {
                if !side.used_by(group.side) {
                    return Err(fail(format!(
                            "air group {} ({}) is {} and of aircraft type {}, which the file of sides gives to the {} side alone",
                            group.id.raw(),
                            group.name,
                            group.side.name(),
                            group.aircraft_type.raw(),
                            side.name()
                        )));
                }
            }
        }
    }
    Ok(())
}
