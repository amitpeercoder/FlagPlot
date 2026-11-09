use std::fs;
use std::path::Path;

use serde::Deserialize;
use sim_core::fixed::Fixed;
use sim_core::ids::ShipClassId;
use sim_core::refdata::{RefData, ShipClass, ShipKind};

use crate::error::DataError;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ShipClassRow {
    id: u32,
    name: String,
    kind: String,
    cruise_speed_kn: String,
    max_speed_kn: String,
    fuel_capacity_t: String,
    endurance_nm: String,
}

fn parse_kind(text: &str) -> Option<ShipKind> {
    Some(match text {
        "carrier" => ShipKind::Carrier,
        "battleship" => ShipKind::Battleship,
        "cruiser" => ShipKind::Cruiser,
        "destroyer" => ShipKind::Destroyer,
        "transport" => ShipKind::Transport,
        "auxiliary" => ShipKind::Auxiliary,
        _ => return None,
    })
}

/// Loads ship classes from a CSV file.
pub fn load_ship_classes(path: &Path) -> Result<RefData, DataError> {
    let text = fs::read_to_string(path).map_err(|e| DataError::in_file(path, e.to_string()))?;
    let mut reader = csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .trim(csv::Trim::All)
        .from_reader(text.as_bytes());

    let mut ref_data = RefData::default();
    for row in reader.deserialize::<ShipClassRow>() {
        let row = row.map_err(|e| DataError::in_file(path, e.to_string()))?;
        let id = ShipClassId::new(row.id);
        let number = |field: &str, text: &str| -> Result<Fixed, DataError> {
            text.parse::<Fixed>().map_err(|e| {
                DataError::in_file(path, format!("{id}: {field} \"{text}\" is invalid: {e}"))
            })
        };
        let class = ShipClass {
            id,
            kind: parse_kind(&row.kind).ok_or_else(|| {
                DataError::in_file(path, format!("{id}: unknown kind \"{}\"", row.kind))
            })?,
            cruise_speed_kn: number("cruise_speed_kn", &row.cruise_speed_kn)?,
            max_speed_kn: number("max_speed_kn", &row.max_speed_kn)?,
            fuel_capacity_t: number("fuel_capacity_t", &row.fuel_capacity_t)?,
            endurance_nm: number("endurance_nm", &row.endurance_nm)?,
            name: row.name,
        };
        if let Some(problem) = class.problem() {
            return Err(DataError::in_file(path, problem));
        }
        if ref_data.ship_classes.insert(id, class).is_some() {
            return Err(DataError::in_file(path, format!("{id} appears twice")));
        }
    }
    Ok(ref_data)
}
