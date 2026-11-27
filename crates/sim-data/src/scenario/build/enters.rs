use crate::error::DataError;
use sim_core::ids::{AirGroupId, EntityRef, LandUnitId, ShipId, TaskForceId};
use sim_core::state::{AirGroupLocation, Ship, ShipLocation, TruthState};
use std::collections::BTreeMap;
use std::path::Path;

use super::super::file::ScenarioFile;
use super::super::orders::ActionSpec;

pub(super) fn hold_units_that_enter_later(
    file: &ScenarioFile,
    path: &Path,
    truth: &mut TruthState,
    map: &sim_core::map::Map,
    _aboard: &BTreeMap<ShipId, u32>,
) -> Result<(), DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    // Units that enter later (decision 0195, 1, 16 to 19): checked here,
    // against the whole truth, then held apart before the world is made.
    let mut held: Vec<(EntityRef, u32)> = Vec::new();
    {
        let mut force_day: BTreeMap<u32, u32> = BTreeMap::new();
        let mut group_day: BTreeMap<u32, u32> = BTreeMap::new();
        let mut unit_day: BTreeMap<u32, u32> = BTreeMap::new();
        let check_day = |what: &str, day: u32| -> Result<(), DataError> {
            if day < 2 {
                return Err(fail(format!(
                        "{what} has enters = {day}: a unit there on day 1 needs no `enters`, and a unit enters on day 2 or later (decision 0195, 1)"
                    )));
            }
            if day > file.days {
                return Err(fail(format!(
                        "{what} has enters = {day}, after the last day of the scenario ({}): it would never enter",
                        file.days
                    )));
            }
            Ok(())
        };
        for spec in &file.ship {
            if spec.enters.is_some() {
                return Err(fail(format!(
                        "ship {} has `enters`: a ship enters with its task force, so put `enters` on the force (decision 0195, 1)",
                        spec.id
                    )));
            }
        }
        for spec in &file.task_force {
            let Some(day) = spec.enters else { continue };
            let what = format!("task force {}", spec.id);
            check_day(&what, day)?;
            let id = TaskForceId::new(spec.id);
            let hex = truth.task_forces[&id].position();
            match map.get(hex) {
                    None => {
                        return Err(fail(format!(
                            "{what} enters on day {day} at {},{}, which is off the map: a force enters at a sea or coast hex (decision 0195, 17)",
                            hex.q, hex.r
                        )))
                    }
                    Some(data) if !data.is_navigable() => {
                        return Err(fail(format!(
                            "{what} enters on day {day} at {},{}, which is land: a force enters at a sea or coast hex, or at a base of its side whose hex is sea or coast (decision 0195, 17)",
                            hex.q, hex.r
                        )))
                    }
                    Some(_) => {}
                }
            force_day.insert(spec.id, day);
        }
        for spec in &file.air_group {
            let Some(day) = spec.enters else { continue };
            let what = format!("air group {}", spec.id);
            check_day(&what, day)?;
            if spec.ship.is_some() {
                return Err(fail(format!(
                        "{what} is aboard a ship and has `enters`: it enters with its ship's task force, so put `enters` on the force (decision 0195, 1)"
                    )));
            }
            group_day.insert(spec.id, day);
        }
        for spec in &file.land_unit {
            let Some(day) = spec.enters else { continue };
            let what = format!("land unit {}", spec.id);
            check_day(&what, day)?;
            if spec.task_force.is_some() {
                return Err(fail(format!(
                        "{what} is aboard a task force and has `enters`: it enters with its force, so put `enters` on the force (decision 0195, 1)"
                    )));
            }
            unit_day.insert(spec.id, day);
        }
        // An arrival of aircraft dated before its group is there.
        for (index, spec) in file.arrival.iter().enumerate() {
            let group = AirGroupId::new(spec.air_group);
            let entered = match group_day.get(&spec.air_group) {
                Some(day) => Some(*day),
                None => match truth.air_groups.get(&group).map(|g| g.location) {
                    Some(AirGroupLocation::Ship(ship)) => match truth.ships.get(&ship) {
                        Some(Ship {
                            location: ShipLocation::TaskForce(force) | ShipLocation::Yard(force),
                            ..
                        }) => force_day.get(&force.raw()).copied(),
                        _ => None,
                    },
                    _ => None,
                },
            };
            if let Some(day) = entered {
                if spec.day < day {
                    return Err(fail(format!(
                            "arrival {} is on day {}, before air group {} enters on day {day}: an arrival is dated on its group's day or after (decision 0195, 19)",
                            index + 1,
                            spec.day,
                            spec.air_group
                        )));
                }
            }
        }
        // An escort and its charge enter on one day or both are there.
        for spec in &file.order {
            if let (
                Some(escort),
                ActionSpec::Escort {
                    charge: Some(charge),
                },
            ) = (spec.task_force, &spec.action)
            {
                let escort_day = force_day.get(&escort).copied();
                let charge_day = force_day.get(charge).copied();
                if escort_day != charge_day {
                    let says = |day: Option<u32>| match day {
                        Some(day) => format!("enters on day {day}"),
                        None => "is there from day 1".to_string(),
                    };
                    return Err(fail(format!(
                            "the escort order on day {} names task force {escort}, which {}, to escort task force {charge}, which {}: an escort and its charge enter on one day or neither enters (decision 0195, 19)",
                            spec.day,
                            says(escort_day),
                            says(charge_day)
                        )));
                }
            }
        }
        for (raw, day) in force_day {
            held.push((EntityRef::TaskForce(TaskForceId::new(raw)), day));
        }
        for (raw, day) in group_day {
            held.push((EntityRef::AirGroup(AirGroupId::new(raw)), day));
        }
        for (raw, day) in unit_day {
            held.push((EntityRef::LandUnit(LandUnitId::new(raw)), day));
        }
    }
    for (unit, day) in held {
        truth
            .hold_apart(unit, day)
            .map_err(|message| fail(format!("{message} (`enters`, decision 0195)")))?;
    }
    Ok(())
}
