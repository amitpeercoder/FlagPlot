use crate::error::DataError;
use sim_core::hex::Hex;
use sim_core::ids::BaseId;
use sim_core::map::{HexKind, Island, IslandPlace};
use sim_core::state::TruthState;
use std::collections::BTreeSet;
use std::path::Path;

use super::super::file::ScenarioFile;

pub(super) fn build_islands(
    file: &ScenarioFile,
    path: &Path,
    truth: &TruthState,
    map: &mut sim_core::map::Map,
) -> Result<(), DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    // Islands are map data (decision 0151, 1); they are added before the
    // world is built, so both sides' maps have them.
    let mut islands_seen: BTreeSet<u32> = BTreeSet::new();
    for spec in &file.island {
        if !islands_seen.insert(spec.number) {
            return Err(fail(format!("island {} appears twice", spec.number)));
        }
        if spec.places.is_empty() {
            return Err(fail(format!("island {} has no place", spec.number)));
        }
        let mut places = Vec::new();
        for (index, place) in spec.places.iter().enumerate() {
            let what = format!("island {} place {}", spec.number, index + 1);
            match (place.base, place.hex) {
                (Some(raw), None) => {
                    let base_id = BaseId::new(raw);
                    let base = truth.bases.get(&base_id).ok_or_else(|| {
                        fail(format!(
                            "{what} names {base_id}, which is not a base of the scenario"
                        ))
                    })?;
                    places.push(IslandPlace {
                        base: Some(base_id),
                        hex: base.hex,
                    });
                }
                (None, Some([q, r])) => {
                    let hex = Hex::new(q, r);
                    let data = map
                        .get(hex)
                        .ok_or_else(|| fail(format!("{what} is at {q},{r}, off the map")))?;
                    if data.kind != HexKind::Coast {
                        return Err(fail(format!("{what} is at {q},{r}, which is not coast")));
                    }
                    places.push(IslandPlace { base: None, hex });
                }
                (Some(_), Some(_)) => {
                    return Err(fail(format!(
                        "{what} has both \"base\" and \"hex\"; it takes exactly one"
                    )));
                }
                (None, None) => {
                    return Err(fail(format!(
                        "{what} has neither \"base\" nor \"hex\"; it takes exactly one"
                    )));
                }
            }
        }
        map.add_island(Island {
            number: spec.number,
            name: spec.name.clone(),
            places,
        });
    }
    Ok(())
}
