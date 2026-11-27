use crate::error::DataError;
use crate::quantity::Quantity;
use sim_core::fixed::Fixed;
use sim_core::ids::{BaseId, LandUnitId};
use sim_core::state::{Side, TruthState};
use std::collections::BTreeMap;
use std::path::Path;

use super::super::file::ScenarioFile;

pub(super) fn build_supply_lines(
    file: &ScenarioFile,
    path: &Path,
    truth: &mut TruthState,
) -> Result<(), DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    // Supply lines come after bases are known (decision 0196).
    for (index, spec) in file.supply_line.iter().enumerate() {
        let number = index + 1;
        let base_id = BaseId::new(spec.base);
        let base = truth.bases.get(&base_id).ok_or_else(|| {
            fail(format!(
                "supply line {number} is at unknown base {}",
                spec.base
            ))
        })?;
        let side: Side = spec.side.map_or(base.side, Into::into);
        let tons = |q: &Option<Quantity>, what: &str| -> Result<Fixed, DataError> {
            let t = match q {
                None => Fixed::ZERO,
                Some(q) => q
                    .to_fixed()
                    .map_err(|e| fail(format!("supply line {number} {what}: {e}")))?,
            };
            if t < Fixed::ZERO {
                return Err(fail(format!("supply line {number} has {what} below 0")));
            }
            Ok(t)
        };
        let supply_t = tons(&spec.supply_t, "supply_t")?;
        let fuel_t = tons(&spec.fuel_t, "fuel_t")?;
        if supply_t == Fixed::ZERO && fuel_t == Fixed::ZERO {
            return Err(fail(format!(
                "supply line {number} gives nothing: neither supply nor fuel is above 0"
            )));
        }
        let first_day = spec.first_day.unwrap_or(1);
        let last_day = spec.last_day.unwrap_or(file.days);
        if first_day < 1 {
            return Err(fail(format!(
                "supply line {number} has a first day below 1"
            )));
        }
        if first_day > file.days || last_day > file.days {
            return Err(fail(format!(
                    "supply line {number} has days {first_day} to {last_day}, past the game's last day {}",
                    file.days
                )));
        }
        if last_day < first_day {
            return Err(fail(format!(
                "supply line {number} has its last day {last_day} before its first day {first_day}"
            )));
        }
        truth.supply_lines.push(sim_core::state::SupplyLine {
            side,
            base: base_id,
            supply_t,
            fuel_t,
            first_day,
            last_day,
        });
    }
    Ok(())
}

pub(super) fn build_men_streams(
    file: &ScenarioFile,
    path: &Path,
    truth: &mut TruthState,
    establishments: BTreeMap<LandUnitId, u32>,
    named_establishment: bool,
) -> Result<(), DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    // Streams of replacement men come after bases are known (decision
    // 0197, 2).
    for (index, spec) in file.men_replacement.iter().enumerate() {
        let number = index + 1;
        let base_id = BaseId::new(spec.base);
        let base = truth.bases.get(&base_id).ok_or_else(|| {
            fail(format!(
                "men replacement {number} is at unknown base {}",
                spec.base
            ))
        })?;
        let side: Side = spec.side.map_or(base.side, Into::into);
        if spec.men == 0 {
            return Err(fail(format!(
                "men replacement {number} gives no men: men must be above 0"
            )));
        }
        let first_day = spec.first_day.unwrap_or(1);
        let last_day = spec.last_day.unwrap_or(file.days);
        if first_day < 1 {
            return Err(fail(format!(
                "men replacement {number} has a first day below 1"
            )));
        }
        if first_day > file.days || last_day > file.days {
            return Err(fail(format!(
                    "men replacement {number} has days {first_day} to {last_day}, past the game's last day {}",
                    file.days
                )));
        }
        if last_day < first_day {
            return Err(fail(format!(
                    "men replacement {number} has its last day {last_day} before its first day {first_day}"
                )));
        }
        truth
            .men_replacement
            .streams
            .push(sim_core::state::MenStream {
                side,
                base: base_id,
                men_per_day: spec.men,
                first_day,
                last_day,
            });
    }
    if !truth.men_replacement.streams.is_empty() || named_establishment {
        truth.men_replacement.units = establishments
            .into_iter()
            .map(|(id, establishment)| {
                (
                    id,
                    sim_core::state::Strength {
                        establishment,
                        joined: 0,
                        last_fought: None,
                    },
                )
            })
            .collect();
    }
    Ok(())
}
