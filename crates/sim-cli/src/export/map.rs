//! The map, the direction table and the place labels as JSON.

use std::path::Path;

use serde::Serialize;
use sim_core::fixed::Fixed;
use sim_core::hex::{Hex, HEX_NM};
use sim_core::map::Map;

use super::names;
use super::num::Num;

#[derive(Serialize)]
pub struct MapOut {
    hex_nm: i64,
    /// True when every hex has a latitude and longitude, which only the
    /// generated CSV maps give.
    real: bool,
    /// Sorted by r, then q.
    hexes: Vec<HexOut>,
}

#[derive(Serialize)]
struct HexOut {
    q: i32,
    r: i32,
    kind: &'static str,
    land_percent: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    lat: Option<Num>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lon: Option<Num>,
    /// Indexes into `directions` of the sides ships cannot cross.
    closed: Vec<usize>,
    port_size: u8,
    airfield_size: u8,
    anchorage: bool,
    terrain: Option<&'static str>,
}

pub fn map(map: &Map) -> MapOut {
    let mut hexes: Vec<(&Hex, HexOut)> = map
        .iter()
        .map(|(hex, data)| {
            (
                hex,
                HexOut {
                    q: hex.q,
                    r: hex.r,
                    kind: names::hex_kind(data.kind),
                    land_percent: data.land_percent,
                    lat: data.lat.map(Num),
                    lon: data.lon.map(Num),
                    closed: data
                        .closed_sides
                        .iter()
                        .enumerate()
                        .filter(|(_, closed)| **closed)
                        .map(|(direction, _)| direction)
                        .collect(),
                    port_size: data.port_size,
                    airfield_size: data.airfield_size,
                    anchorage: data.anchorage,
                    terrain: data.terrain.map(names::terrain),
                },
            )
        })
        .collect();
    hexes.sort_by_key(|(hex, _)| (hex.r, hex.q));
    let hexes: Vec<HexOut> = hexes.into_iter().map(|(_, out)| out).collect();
    let real = !hexes.is_empty() && hexes.iter().all(|h| h.lat.is_some() && h.lon.is_some());
    MapOut {
        hex_nm: HEX_NM,
        real,
        hexes,
    }
}

/// The six neighbour offsets, read from `Hex::DIRECTIONS`.
pub fn directions() -> Vec<[i32; 2]> {
    Hex::DIRECTIONS.iter().map(|(dq, dr)| [*dq, *dr]).collect()
}

#[derive(Serialize)]
pub struct LabelOut {
    place: String,
    feature: String,
    lat: Num,
    lon: Num,
}

/// Reads an anchors CSV: `#` comment lines, a header row, then one row per
/// place. Only `place`, `feature`, `lat` and `lon` are kept. Latitude and
/// longitude must be plain decimals, and are kept exactly.
pub fn labels(path: &Path) -> Result<Vec<LabelOut>, String> {
    let fail = |message: String| format!("{}: {message}", path.display());
    let mut reader = csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .from_path(path)
        .map_err(|e| fail(e.to_string()))?;
    let headers = reader.headers().map_err(|e| fail(e.to_string()))?.clone();
    let column = |name: &str| {
        headers
            .iter()
            .position(|header| header.trim() == name)
            .ok_or_else(|| fail(format!("no \"{name}\" column")))
    };
    let (place, feature, lat, lon) = (
        column("place")?,
        column("feature")?,
        column("lat")?,
        column("lon")?,
    );
    let mut labels = Vec::new();
    for record in reader.records() {
        let record = record.map_err(|e| fail(e.to_string()))?;
        let field = |index: usize| record.get(index).unwrap_or("").trim();
        let number = |index: usize, name: &str| {
            field(index)
                .parse::<Fixed>()
                .map(Num)
                .map_err(|e| fail(format!("{} {name} \"{}\": {e}", field(place), field(index))))
        };
        labels.push(LabelOut {
            place: field(place).to_string(),
            feature: field(feature).to_string(),
            lat: number(lat, "lat")?,
            lon: number(lon, "lon")?,
        });
    }
    Ok(labels)
}
