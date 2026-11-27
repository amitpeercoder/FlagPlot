use crate::error::DataError;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, SubmarineClassId, SubmarineId};
use sim_core::refdata::RefData;
use sim_core::state::{Submarine, SubmarineCondition, SubmarinePassage, TruthState};
use sim_core::time::Pulse;
use std::path::Path;

use super::super::file::ScenarioFile;

pub(super) fn build_submarines(
    file: &ScenarioFile,
    path: &Path,
    truth: &mut TruthState,
    ref_data: &RefData,
) -> Result<(), DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    // Submarines (decision 0218): built against the data and the bases;
    // the world's own checks then see that each is whole.
    for spec in &file.submarine {
        let id = SubmarineId::new(spec.id);
        if truth.submarines.contains_key(&id) {
            return Err(fail(format!("submarine {} is given twice", spec.id)));
        }
        let what = format!("submarine {}", spec.id);
        let class_id = SubmarineClassId::new(spec.class);
        let class = ref_data
            .submarines
            .as_ref()
            .and_then(|data| data.classes.get(&class_id))
            .ok_or_else(|| {
                fail(format!(
                    "{what} has class {}, which the submarine class file does not give",
                    spec.class
                ))
            })?;
        let hex = Hex::new(spec.q, spec.r);
        let station = match (spec.station_q, spec.station_r) {
            (None, None) => hex,
            (Some(q), Some(r)) => Hex::new(q, r),
            _ => {
                return Err(fail(format!(
                    "{what} gives one of station_q and station_r: give both or neither"
                )))
            }
        };
        let salvoes = spec.salvoes.unwrap_or(class.salvoes_carried);
        if salvoes == 0 {
            return Err(fail(format!(
                    "{what} has salvoes = 0: a boat with no salvo is sent home, so a scenario gives 1 to her class's {} (decision 0218, rule 43)",
                    class.salvoes_carried
                )));
        }
        if salvoes > class.salvoes_carried {
            return Err(fail(format!(
                "{what} has salvoes = {salvoes}, more than her class carries ({})",
                class.salvoes_carried
            )));
        }
        let condition = match spec.condition.as_deref() {
            None | Some("sound") => SubmarineCondition::Sound,
            Some("damaged") => SubmarineCondition::Damaged,
            Some(other) => {
                return Err(fail(format!(
                    "{what} has condition \"{other}\": sound or damaged"
                )))
            }
        };
        for (key, day) in [
            ("leave_day", spec.leave_day),
            ("available_from", spec.available_from),
        ] {
            if day == Some(0) {
                return Err(fail(format!("{what} has {key} = 0: days count from 1")));
            }
        }
        if let Some(day) = spec.enters {
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
        }
        let home = BaseId::new(spec.home);
        // A damaged boat starts going home; a boat whose station is not
        // her hex starts on passage to it (decision 0218, 43).
        let passage = match (condition, truth.bases.get(&home)) {
            (SubmarineCondition::Damaged, Some(base)) => Some(SubmarinePassage {
                to: base.hex,
                home: true,
                toward_hex: None,
                progress_nm: Fixed::ZERO,
                leg_nm: Fixed::ZERO,
                arrived: false,
                order: None,
            }),
            (SubmarineCondition::Damaged, None) => None,
            _ if station != hex => Some(SubmarinePassage {
                to: station,
                home: false,
                toward_hex: None,
                progress_nm: Fixed::ZERO,
                leg_nm: Fixed::ZERO,
                arrived: false,
                order: None,
            }),
            _ => None,
        };
        // Held down for that many pulses from the first, the night
        // counted as the first: the last pulse she is held is that many
        // pulses after the night of day 1, the night included.
        let held_down_until = match spec.held_down_pulses.unwrap_or(0) {
            0 => None,
            pulses => {
                let place = usize::try_from(pulses - 1).expect("a count of pulses");
                let day = 1 + u32::try_from(place / 3).expect("a count of days");
                Some((day, Pulse::ALL[place % 3]))
            }
        };
        truth.submarines.insert(
            id,
            Submarine {
                id,
                name: spec.name.clone(),
                side: spec.side.into(),
                class: class_id,
                hex,
                station,
                passage,
                salvoes,
                held_down_until,
                condition,
                home,
                leave_day: spec.leave_day,
                enters_day: spec.enters,
                available_from: spec.available_from,
                station_order: None,
            },
        );
    }
    Ok(())
}
