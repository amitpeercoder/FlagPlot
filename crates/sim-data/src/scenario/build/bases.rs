use crate::error::DataError;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::BaseId;
use sim_core::state::{Base, TruthState};
use std::path::Path;

use super::super::file::ScenarioFile;

pub(super) fn build_bases(
    file: &ScenarioFile,
    path: &Path,
    truth: &mut TruthState,
) -> Result<(), DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    for spec in &file.base {
        let id = BaseId::new(spec.id);
        let airfield = spec.airfield.unwrap_or(true);
        let airfield_work = match &spec.airfield_work {
            Some(quantity) => {
                let work = quantity
                    .to_fixed()
                    .map_err(|e| fail(format!("{id}: airfield_work {e}")))?;
                if work.is_negative() {
                    return Err(fail(format!("{id}: airfield_work is negative")));
                }
                if airfield {
                    return Err(fail(format!(
                            "{id}: airfield_work is given for a base that has an airfield, which has nothing more to build; give it only with airfield = false"
                        )));
                }
                work
            }
            None => Fixed::ZERO,
        };
        let defences = match &spec.defences {
            Some(quantity) => {
                let defences = quantity
                    .to_fixed()
                    .map_err(|e| fail(format!("{id}: defences {e}")))?;
                if defences.is_negative() || defences > Fixed::from_int(100) {
                    return Err(fail(format!(
                        "{id}: defences {defences} is not between 0 and 100"
                    )));
                }
                defences
            }
            None => Fixed::ZERO,
        };
        let airfield_damage = match &spec.airfield_damage {
            Some(quantity) => {
                let damage = quantity
                    .to_fixed()
                    .map_err(|e| fail(format!("{id}: airfield_damage {e}")))?;
                if damage.is_negative() || damage > Fixed::from_int(100) {
                    return Err(fail(format!(
                        "{id}: airfield_damage {damage} is not between 0 and 100"
                    )));
                }
                if !airfield {
                    return Err(fail(format!(
                            "{id}: airfield_damage is given for a base that has no airfield, which has none to damage; give it only for a base with an airfield"
                        )));
                }
                damage
            }
            None => Fixed::ZERO,
        };
        let base = Base {
                id,
                name: spec.name.clone(),
                side: spec.side.into(),
                hex: Hex::new(spec.q, spec.r),
                supply_t: match &spec.supply_t {
                    Some(quantity) => quantity
                        .to_fixed()
                        .map_err(|e| fail(format!("{id}: supply_t {e}")))?,
                    None => Fixed::ZERO,
                },
                anti_aircraft: spec.anti_aircraft.unwrap_or(0),
                aircraft_repairs_per_day: spec.aircraft_repairs_per_day.unwrap_or(0),
                fuel_t: match &spec.fuel_t {
                    Some(quantity) => {
                        let fuel = quantity
                            .to_fixed()
                            .map_err(|e| fail(format!("{id}: fuel_t {e}")))?;
                        if fuel < Fixed::ZERO {
                            return Err(fail(format!("{id}: fuel_t is negative")));
                        }
                        fuel
                    }
                    None => Fixed::ZERO,
                },
                supply_reserve_t: match &spec.supply_reserve_t {
                    Some(quantity) => {
                        let reserve = quantity
                            .to_fixed()
                            .map_err(|e| fail(format!("{id}: supply_reserve_t {e}")))?;
                        if reserve < Fixed::ZERO {
                            return Err(fail(format!("{id}: supply_reserve_t is negative")));
                        }
                        reserve
                    }
                    None => Fixed::ZERO,
                },
                // No men's rations landed yet (decision 0205, 5).
                rations_landed: false,
                ammunition_t: match &spec.ammunition_t {
                    Some(quantity) => {
                        let tons = quantity
                            .to_fixed()
                            .map_err(|e| fail(format!("{id}: ammunition_t {e}")))?;
                        if tons < Fixed::ZERO {
                            return Err(fail(format!("{id}: ammunition_t is negative")));
                        }
                        tons
                    }
                    None => Fixed::ZERO,
                },
                fuel_reserve_t: match &spec.fuel_reserve_t {
                    Some(quantity) => {
                        let reserve = quantity
                            .to_fixed()
                            .map_err(|e| fail(format!("{id}: fuel_reserve_t {e}")))?;
                        if reserve < Fixed::ZERO {
                            return Err(fail(format!("{id}: fuel_reserve_t is negative")));
                        }
                        reserve
                    }
                    None => Fixed::ZERO,
                },
                ship_repair_points_per_day: match &spec.ship_repair_points_per_day {
                    Some(quantity) => {
                        let points = quantity
                            .to_fixed()
                            .map_err(|e| fail(format!("{id}: ship_repair_points_per_day {e}")))?;
                        if points < Fixed::ZERO {
                            return Err(fail(format!(
                                "{id}: ship_repair_points_per_day is negative"
                            )));
                        }
                        points
                    }
                    None => Fixed::ZERO,
                },
                yard_passage_days: match spec.yard_passage_days {
                    Some(days) => Some(
                        u32::try_from(days)
                            .ok()
                            .filter(|days| *days >= 1)
                            .ok_or_else(|| {
                                fail(format!(
                                    "{id}: yard_passage_days {days} is not a whole number of at least 1"
                                ))
                            })?,
                    ),
                    None => None,
                },
                on_map: spec.on_map.unwrap_or(true),
                airfield,
                airfield_work,
                defences,
                day_work: None,
                airfield_damage,
            };
        if truth.bases.insert(id, base).is_some() {
            return Err(fail(format!("{id} appears twice")));
        }
    }
    Ok(())
}
