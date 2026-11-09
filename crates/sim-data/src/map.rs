use std::fs;
use std::path::Path;

use serde::Deserialize;
use sim_core::hex::Hex;
use sim_core::map::{HexData, HexKind, Map, Terrain};

use crate::error::DataError;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct MapFile {
    /// Must be present, so nobody mistakes invented geography for the real
    /// thing. Not used by the loader itself.
    #[allow(dead_code)]
    placeholder: bool,
    #[serde(default)]
    region: Vec<RegionSpec>,
    #[serde(default)]
    hex: Vec<HexSpec>,
}

/// A rectangle of identical hexes. Both ranges are inclusive.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RegionSpec {
    kind: HexKindSpec,
    q: [i32; 2],
    r: [i32; 2],
}

/// One hex, overriding whatever a region put there.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct HexSpec {
    q: i32,
    r: i32,
    kind: HexKindSpec,
    terrain: Option<TerrainSpec>,
    #[serde(default)]
    port_size: u8,
    #[serde(default)]
    airfield_size: u8,
    #[serde(default)]
    anchorage: bool,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum HexKindSpec {
    DeepSea,
    ShallowSea,
    Coast,
    Land,
}

impl From<HexKindSpec> for HexKind {
    fn from(spec: HexKindSpec) -> HexKind {
        match spec {
            HexKindSpec::DeepSea => HexKind::DeepSea,
            HexKindSpec::ShallowSea => HexKind::ShallowSea,
            HexKindSpec::Coast => HexKind::Coast,
            HexKindSpec::Land => HexKind::Land,
        }
    }
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum TerrainSpec {
    Clear,
    Jungle,
    Mountain,
    Atoll,
}

impl From<TerrainSpec> for Terrain {
    fn from(spec: TerrainSpec) -> Terrain {
        match spec {
            TerrainSpec::Clear => Terrain::Clear,
            TerrainSpec::Jungle => Terrain::Jungle,
            TerrainSpec::Mountain => Terrain::Mountain,
            TerrainSpec::Atoll => Terrain::Atoll,
        }
    }
}

/// Loads a map from a TOML file.
pub fn load_map(path: &Path) -> Result<Map, DataError> {
    let text = fs::read_to_string(path).map_err(|e| DataError::in_file(path, e.to_string()))?;
    let file: MapFile =
        toml::from_str(&text).map_err(|e| DataError::in_file(path, e.to_string()))?;

    let mut map = Map::new();
    for region in &file.region {
        if region.q[0] > region.q[1] || region.r[0] > region.r[1] {
            return Err(DataError::in_file(
                path,
                "a region's ranges must run from low to high",
            ));
        }
        for q in region.q[0]..=region.q[1] {
            for r in region.r[0]..=region.r[1] {
                map.insert(
                    Hex::new(q, r),
                    HexData {
                        kind: region.kind.into(),
                        ..HexData::open_sea()
                    },
                );
            }
        }
    }
    for hex in &file.hex {
        if hex.port_size > 10 || hex.airfield_size > 10 {
            return Err(DataError::in_file(
                path,
                format!("hex ({},{}): site sizes run 0 to 10", hex.q, hex.r),
            ));
        }
        map.insert(
            Hex::new(hex.q, hex.r),
            HexData {
                kind: hex.kind.into(),
                terrain: hex.terrain.map(Into::into),
                port_size: hex.port_size,
                airfield_size: hex.airfield_size,
                anchorage: hex.anchorage,
            },
        );
    }
    if map.is_empty() {
        return Err(DataError::in_file(path, "the map has no hexes"));
    }
    Ok(map)
}
