use std::fs;
use std::path::Path;

use serde::Deserialize;
use sim_core::fixed::Fixed;
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
    /// Pairs of neighbouring hexes men can march between (decision 0094).
    #[serde(default)]
    land_link: Vec<LandLinkSpec>,
}

/// One land link (decision 1 of 0094).
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LandLinkSpec {
    a: [i32; 2],
    b: [i32; 2],
    /// Free text for the reader of the file. Not used by the loader.
    #[allow(dead_code)]
    note: Option<String>,
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
    /// Directions (0 to 5, indexed like `Hex::DIRECTIONS`) whose side is
    /// closed to ships. Mirrored onto the neighbour when it is on the map.
    #[serde(default)]
    closed_sides: Vec<i64>,
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

/// Loads a map from a file: a generated `.csv` map (see [`parse_csv_map`])
/// or a hand-written `.toml` map. The extension decides.
pub fn load_map(path: &Path) -> Result<Map, DataError> {
    let text = fs::read_to_string(path).map_err(|e| DataError::in_file(path, e.to_string()))?;
    if path.extension().and_then(|e| e.to_str()) == Some("csv") {
        return parse_csv_map(&text, path);
    }
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
    let mut closed: Vec<(Hex, usize)> = Vec::new();
    for hex in &file.hex {
        let place = format!("hex ({},{})", hex.q, hex.r);
        let mut seen = [false; 6];
        for side in &hex.closed_sides {
            let index = usize::try_from(*side).ok().filter(|i| *i <= 5);
            match index {
                None => {
                    return Err(DataError::in_file(
                        path,
                        format!("{place}: closed side {side} is not a direction from 0 to 5"),
                    ))
                }
                Some(i) if seen[i] => {
                    return Err(DataError::in_file(
                        path,
                        format!("{place}: closed side {i} is listed twice"),
                    ))
                }
                Some(i) => {
                    seen[i] = true;
                    closed.push((Hex::new(hex.q, hex.r), i));
                }
            }
        }
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
                ..HexData::open_sea()
            },
        );
    }
    // Closed sides are applied once everything is placed, so a hex entry
    // never overwrites a mark made from its neighbour. Each side is set on
    // the hex and mirrored onto the neighbour when it is on the map, so a
    // test map states each closed side once.
    for (hex, direction) in closed {
        let Some(data) = map.get(hex) else {
            continue;
        };
        let mut data = data.clone();
        data.closed_sides[direction] = true;
        map.insert(hex, data);
        let (dq, dr) = Hex::DIRECTIONS[direction];
        let neighbor = Hex::new(hex.q + dq, hex.r + dr);
        if let Some(other) = map.get(neighbor) {
            let mut other = other.clone();
            other.closed_sides[(direction + 3) % 6] = true;
            map.insert(neighbor, other);
        }
    }
    for link in &file.land_link {
        let a = Hex::new(link.a[0], link.a[1]);
        let b = Hex::new(link.b[0], link.b[1]);
        map.add_land_link(a, b)
            .map_err(|e| DataError::in_file(path, e))?;
    }
    if map.is_empty() {
        return Err(DataError::in_file(path, "the map has no hexes"));
    }
    Ok(map)
}

/// The columns of a land links file, in order.
const LAND_LINK_COLUMNS: [&str; 6] = ["q1", "r1", "q2", "r2", "landmass", "reason"];

/// Reads a land links file (decision 2 of 0094) and adds its links to `map`,
/// whatever format the map came from. See [`parse_land_links`].
pub fn load_land_links(path: &Path, map: &mut Map) -> Result<(), DataError> {
    let text = fs::read_to_string(path).map_err(|e| DataError::in_file(path, e.to_string()))?;
    parse_land_links(&text, path, map)
}

/// Parses a land links file: lines starting with `#` are comments; the
/// header is exactly `q1,r1,q2,r2,landmass,reason`; each row is one link
/// between two neighbouring hexes. `landmass` is the landmass the link is
/// for (a hex can hold two islands) and `reason` says why the two are
/// joined; both must be filled. A link the map refuses, or that is listed
/// twice, or that the map already has, is an error naming the row. `path` is
/// only used in messages.
fn parse_land_links(text: &str, path: &Path, map: &mut Map) -> Result<(), DataError> {
    let mut reader = csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .trim(csv::Trim::All)
        .from_reader(text.as_bytes());
    let fail = |message: String| DataError::in_file(path, message);

    let headers = reader.headers().map_err(|e| fail(e.to_string()))?;
    let found: Vec<&str> = headers.iter().collect();
    if found != LAND_LINK_COLUMNS {
        return Err(fail(format!(
            "the header must be exactly {}, found {}",
            LAND_LINK_COLUMNS.join(","),
            found.join(",")
        )));
    }

    for (index, row) in reader.records().enumerate() {
        let row = row.map_err(|e| fail(e.to_string()))?;
        let place = format!("land link row {}", index + 1);
        let number = |column: usize| -> Result<i32, DataError> {
            let text = row.get(column).unwrap_or("");
            text.parse().map_err(|_| {
                fail(format!(
                    "{place}: {} \"{text}\" is not a whole number",
                    LAND_LINK_COLUMNS[column]
                ))
            })
        };
        let a = Hex::new(number(0)?, number(1)?);
        let b = Hex::new(number(2)?, number(3)?);
        for column in [4, 5] {
            if row.get(column).unwrap_or("").is_empty() {
                return Err(fail(format!(
                    "{place} ({a} to {b}): {} is empty",
                    LAND_LINK_COLUMNS[column]
                )));
            }
        }
        map.add_land_link(a, b)
            .map_err(|e| fail(format!("{place}: {e}")))?;
    }
    Ok(())
}

/// The columns of a generated map, in order.
const CSV_COLUMNS: [&str; 8] = [
    "q",
    "r",
    "kind",
    "land_percent",
    "lat",
    "lon",
    "water_bodies",
    "closed_sides",
];

/// Parses a generated map (decision 0017, decision 9): lines starting with
/// `#` are comments; the header is exactly
/// `q,r,kind,land_percent,lat,lon,water_bodies,closed_sides`; each row is one
/// hex, in any order. `kind` is `land`, `coast` or `deep_sea` and must agree
/// with `land_percent`: land 80 to 100, coast 0 to 79 (a coast hex whose
/// land share rounds down to 0 is allowed), deep sea 0. The percentage is
/// checked and kept. `water_bodies` is a whole number, checked and dropped.
/// `closed_sides` is empty, or direction indexes 0 to 5 joined by `|` (for
/// example `0|4|5`), with no repeats. After all rows are read the closed
/// sides must be symmetric: if a hex closes the side toward a neighbour
/// that is on the map, the neighbour must close the side back. Latitude and
/// longitude of the centre are decimal text, kept exactly as `Fixed`. `path`
/// is only used in messages.
fn parse_csv_map(text: &str, path: &Path) -> Result<Map, DataError> {
    let mut reader = csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .trim(csv::Trim::All)
        .from_reader(text.as_bytes());
    let fail = |message: String| DataError::in_file(path, message);

    let headers = reader.headers().map_err(|e| fail(e.to_string()))?;
    let found: Vec<&str> = headers.iter().collect();
    if found != CSV_COLUMNS {
        return Err(fail(format!(
            "the header must be exactly {}, found {}",
            CSV_COLUMNS.join(","),
            found.join(",")
        )));
    }

    let mut map = Map::new();
    for row in reader.records() {
        let row = row.map_err(|e| fail(e.to_string()))?;
        let field = |index: usize| row.get(index).unwrap_or("");
        let place = format!("hex ({},{})", field(0), field(1));
        let q: i32 = field(0)
            .parse()
            .map_err(|_| fail(format!("q \"{}\" is not a whole number", field(0))))?;
        let r: i32 = field(1)
            .parse()
            .map_err(|_| fail(format!("r \"{}\" is not a whole number", field(1))))?;
        let kind = match field(2) {
            "land" => HexKind::Land,
            "coast" => HexKind::Coast,
            "deep_sea" => HexKind::DeepSea,
            other => {
                return Err(fail(format!(
                    "{place}: kind \"{other}\" is not land, coast or deep_sea"
                )))
            }
        };
        let percent: u8 = field(3).parse().ok().filter(|p| *p <= 100).ok_or_else(|| {
            fail(format!(
                "{place}: land_percent \"{}\" is not a whole number from 0 to 100",
                field(3)
            ))
        })?;
        let agrees = match kind {
            HexKind::Land => percent >= 80,
            HexKind::Coast => percent <= 79,
            _ => percent == 0,
        };
        if !agrees {
            return Err(fail(format!(
                "{place}: kind {} does not agree with land_percent {percent}",
                field(2)
            )));
        }
        let degrees = |name: &str, text: &str| -> Result<Fixed, DataError> {
            text.parse::<Fixed>()
                .map_err(|e| fail(format!("{place}: {name} \"{text}\" is invalid: {e}")))
        };
        let lat = degrees("lat", field(4))?;
        let lon = degrees("lon", field(5))?;
        field(6).parse::<u32>().map_err(|_| {
            fail(format!(
                "{place}: water_bodies \"{}\" is not a whole number",
                field(6)
            ))
        })?;
        let mut closed_sides = [false; 6];
        if !field(7).is_empty() {
            for part in field(7).split('|') {
                let index: usize = part.parse().ok().filter(|i| *i <= 5).ok_or_else(|| {
                    fail(format!(
                        "{place}: closed side \"{part}\" is not a direction from 0 to 5"
                    ))
                })?;
                if closed_sides[index] {
                    return Err(fail(format!(
                        "{place}: closed side {index} is listed twice"
                    )));
                }
                closed_sides[index] = true;
            }
        }
        let hex = Hex::new(q, r);
        if map.contains(hex) {
            return Err(fail(format!("{place} appears twice")));
        }
        map.insert(
            hex,
            HexData {
                kind,
                // Terrain is not in the source data (coastlines only), so
                // every land hex is Jungle for now.
                terrain: (kind == HexKind::Land).then_some(Terrain::Jungle),
                lat: Some(lat),
                lon: Some(lon),
                land_percent: percent,
                closed_sides,
                ..HexData::open_sea()
            },
        );
    }
    if map.is_empty() {
        return Err(fail("the map has no hexes".to_string()));
    }
    // Symmetry: a side closed in one hex must be closed in the neighbour
    // that is on the map.
    for (hex, data) in map.iter() {
        for (direction, neighbor) in hex.neighbors().into_iter().enumerate() {
            if !data.closed_sides[direction] {
                continue;
            }
            if let Some(other) = map.get(neighbor) {
                if !other.closed_sides[(direction + 3) % 6] {
                    return Err(fail(format!(
                        "hex ({},{}) closes direction {direction} toward ({},{}), which does not close direction {} back",
                        hex.q,
                        hex.r,
                        neighbor.q,
                        neighbor.r,
                        (direction + 3) % 6
                    )));
                }
            }
        }
    }
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEADER: &str = "# a comment\nq,r,kind,land_percent,lat,lon,water_bodies,closed_sides\n";

    fn parse(body: &str) -> Result<Map, DataError> {
        parse_csv_map(&format!("{HEADER}{body}"), Path::new("test.csv"))
    }

    #[test]
    fn a_small_csv_loads_with_exact_centres() {
        let map = parse(
            "# another comment\n0,0,deep_sea,0,-12.0000,160.0000,1,\n\
             1,0,coast,12,-12.0000,160.6814,1,\n\
             5,-4,land,100,-9.6906,162.0447,0,\n",
        )
        .unwrap();
        assert_eq!(map.len(), 3);
        let sea = map.get(Hex::new(0, 0)).unwrap();
        assert_eq!(sea.kind, HexKind::DeepSea);
        assert_eq!(sea.terrain, None);
        assert_eq!(sea.lat, Some(Fixed::from_int(-12)));
        let coast = map.get(Hex::new(1, 0)).unwrap();
        assert_eq!(coast.kind, HexKind::Coast);
        assert_eq!(coast.lon, Some(Fixed::from_raw(160_681_400)));
        let land = map.get(Hex::new(5, -4)).unwrap();
        assert_eq!(land.kind, HexKind::Land);
        assert_eq!(land.terrain, Some(Terrain::Jungle));
        assert_eq!(land.lat, Some(Fixed::from_raw(-9_690_600)));
        assert_eq!(land.lon, Some(Fixed::from_raw(162_044_700)));
        assert_eq!(
            (land.port_size, land.airfield_size, land.anchorage),
            (0, 0, false)
        );
    }

    #[test]
    fn the_extension_picks_the_format() {
        let dir = std::env::temp_dir().join("flag_plot_csv_map_test");
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("tiny.csv");
        std::fs::write(
            &file,
            format!("{HEADER}0,0,deep_sea,0,-12.0000,160.0000,1,\n"),
        )
        .unwrap();
        assert_eq!(load_map(&file).unwrap().len(), 1);
        std::fs::remove_file(&file).unwrap();
    }

    fn message(body: &str) -> String {
        parse(body).unwrap_err().message
    }

    #[test]
    fn an_unknown_or_missing_column_is_an_error() {
        let extra = "q,r,kind,land_percent,lat,lon,depth\n0,0,deep_sea,0,-12,160,3\n";
        assert!(parse_csv_map(extra, Path::new("t.csv"))
            .unwrap_err()
            .message
            .contains("header"));
        let missing = "q,r,kind,land_percent,lat\n0,0,deep_sea,0,-12\n";
        assert!(parse_csv_map(missing, Path::new("t.csv"))
            .unwrap_err()
            .message
            .contains("header"));
    }

    #[test]
    fn a_bad_kind_is_an_error() {
        assert!(message("0,0,swamp,0,-12,160,1,\n").contains("swamp"));
    }

    #[test]
    fn kind_and_percent_must_agree() {
        assert!(message("0,0,land,79,-12,160,0,\n").contains("does not agree"));
        assert!(message("0,0,coast,80,-12,160,1,\n").contains("does not agree"));
        assert!(message("0,0,deep_sea,1,-12,160,1,\n").contains("does not agree"));
        assert!(message("0,0,land,101,-12,160,0,\n").contains("0 to 100"));
        // A coast hex with land points but under 1 percent has 0.
        assert!(
            parse("0,0,land,80,-12,160,0,\n1,0,coast,79,-12,160,1,\n2,0,coast,0,-12,160,1,\n")
                .is_ok()
        );
    }

    #[test]
    fn a_duplicate_hex_is_an_error() {
        assert!(message("0,0,deep_sea,0,-12,160,1,\n0,0,land,100,-12,160,0,\n").contains("twice"));
    }

    #[test]
    fn a_bad_latitude_is_an_error() {
        assert!(message("0,0,deep_sea,0,-1e1,160,1,\n").contains("lat"));
    }

    #[test]
    fn closed_sides_and_land_percent_are_read() {
        // (0,0) closes directions 0 and 4 and (1,0) closes 3 back; (-1,1) is
        // the neighbour in direction 4 of (0,0) and closes 1 back.
        let map = parse(
            "0,0,coast,40,-12.0000,160.0000,2,0|4\n\
             1,0,coast,12,-12.0000,160.6814,1,3\n\
             -1,1,coast,7,-12.0000,159.0000,1,1\n",
        )
        .unwrap();
        let first = map.get(Hex::new(0, 0)).unwrap();
        assert_eq!(first.land_percent, 40);
        assert_eq!(first.closed_sides, [true, false, false, false, true, false]);
        assert_eq!(map.get(Hex::new(1, 0)).unwrap().land_percent, 12);
        assert!(!map.can_step(Hex::new(0, 0), 0));
        assert!(!map.can_step(Hex::new(1, 0), 3));
        assert!(!map.can_step(Hex::new(0, 0), 4));
        assert!(map.sea_distance(Hex::new(0, 0), Hex::new(1, 0)).is_none());
    }

    #[test]
    fn a_bad_closed_side_is_an_error() {
        let bad_index = message("2,3,coast,5,-12,160,1,6\n");
        assert!(bad_index.contains("(2,3)") && bad_index.contains("0 to 5"));
        assert!(message("0,0,coast,5,-12,160,1,x\n").contains("0 to 5"));
        assert!(message("0,0,coast,5,-12,160,1,0||1\n").contains("0 to 5"));
        let repeated = message("2,3,coast,5,-12,160,1,0|4|0\n");
        assert!(repeated.contains("(2,3)") && repeated.contains("twice"));
        assert!(message("0,0,coast,5,-12,160,x,\n").contains("water_bodies"));
    }

    #[test]
    fn an_asymmetric_file_is_an_error() {
        // (0,0) closes direction 0, toward (1,0), which is on the map and
        // does not close direction 3 back.
        let text = message("0,0,coast,5,-12,160,1,0\n1,0,coast,5,-12,160.6,1,\n");
        assert!(text.contains("(0,0)") && text.contains("(1,0)"));
        assert!(text.contains("direction 0") && text.contains("direction 3"));
        // The other order gives the same error.
        assert!(parse("1,0,coast,5,-12,160.6,1,\n0,0,coast,5,-12,160,1,0\n").is_err());
        // A side toward a hex that is not on the map needs no partner.
        assert!(parse("0,0,coast,5,-12,160,1,0|1|2|3|4|5\n").is_ok());
    }

    fn load_toml(text: &str) -> Result<Map, DataError> {
        let dir = std::env::temp_dir().join("flag_plot_toml_map_test");
        std::fs::create_dir_all(&dir).unwrap();
        // A name of its own, since tests run in parallel.
        static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let number = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let file = dir.join(format!("{}_{number}.toml", std::process::id()));
        std::fs::write(&file, text).unwrap();
        let result = load_map(&file);
        std::fs::remove_file(&file).unwrap();
        result
    }

    #[test]
    fn a_toml_closed_side_is_mirrored_onto_the_neighbour() {
        let map = load_toml(
            "placeholder = true\n\
             [[region]]\nkind = \"deep_sea\"\nq = [0, 2]\nr = [0, 0]\n\
             [[hex]]\nq = 1\nr = 0\nkind = \"deep_sea\"\nclosed_sides = [0, 3]\n",
        )
        .unwrap();
        let middle = map.get(Hex::new(1, 0)).unwrap();
        assert_eq!(
            middle.closed_sides,
            [true, false, false, true, false, false]
        );
        // Mirrored: (2,0) closes direction 3, (0,0) closes direction 0.
        assert!(map.get(Hex::new(2, 0)).unwrap().closed_sides[3]);
        assert!(map.get(Hex::new(0, 0)).unwrap().closed_sides[0]);
        assert_eq!(map.sea_distance(Hex::new(0, 0), Hex::new(2, 0)), None);
        // Directions that lead off the map are fine.
        assert!(load_toml(
            "placeholder = true\n[[hex]]\nq = 0\nr = 0\nkind = \"deep_sea\"\nclosed_sides = [1]\n"
        )
        .is_ok());
    }

    #[test]
    fn a_bad_toml_closed_side_is_an_error() {
        let head = "placeholder = true\n[[hex]]\nq = 0\nr = 0\nkind = \"deep_sea\"\n";
        assert!(load_toml(&format!("{head}closed_sides = [6]\n"))
            .unwrap_err()
            .message
            .contains("0 to 5"));
        assert!(load_toml(&format!("{head}closed_sides = [-1]\n")).is_err());
        assert!(load_toml(&format!("{head}closed_sides = [2, 2]\n"))
            .unwrap_err()
            .message
            .contains("twice"));
    }

    // ------------------------------------------------------------------
    // Land links (decision 0094).
    // ------------------------------------------------------------------

    const LINK_HEADER: &str = "# a comment\nq1,r1,q2,r2,landmass,reason\n";

    /// A map of coast hexes (0,0), (1,0) and (2,0), land (3,0), and deep sea
    /// at (4,0).
    fn link_map() -> Map {
        parse(
            "0,0,coast,12,-12.0000,160.0000,1,\n\
             1,0,coast,12,-12.0000,160.6814,1,\n\
             2,0,coast,12,-12.0000,161.3628,1,\n\
             3,0,land,100,-12.0000,162.0442,0,\n\
             4,0,deep_sea,0,-12.0000,162.7256,1,\n",
        )
        .unwrap()
    }

    fn links(body: &str) -> Result<Map, DataError> {
        let mut map = link_map();
        parse_land_links(
            &format!("{LINK_HEADER}{body}"),
            Path::new("l.csv"),
            &mut map,
        )?;
        Ok(map)
    }

    #[test]
    fn a_land_links_file_adds_its_links_to_the_map() {
        let map = links(
            "# another comment\n0,0,1,0,Placeholder island,one landmass\n\
             3,0,2,0,Placeholder island,a shore walk\n",
        )
        .unwrap();
        assert!(map.has_land_link(Hex::new(1, 0), Hex::new(0, 0)));
        assert!(map.has_land_link(Hex::new(2, 0), Hex::new(3, 0)));
        assert!(!map.has_land_link(Hex::new(1, 0), Hex::new(2, 0)));
        // An empty file of links is allowed.
        assert_eq!(links("").unwrap().land_links().count(), 0);
    }

    fn link_message(body: &str) -> String {
        links(body).unwrap_err().message
    }

    #[test]
    fn a_bad_land_links_file_is_an_error_naming_the_row() {
        let wrong = "q1,r1,q2,r2,landmass\n0,0,1,0,x\n";
        let mut map = link_map();
        assert!(parse_land_links(wrong, Path::new("l.csv"), &mut map)
            .unwrap_err()
            .message
            .contains("header"));
        let extra = "q1,r1,q2,r2,landmass,reason,more\n0,0,1,0,x,y,z\n";
        assert!(parse_land_links(extra, Path::new("l.csv"), &mut map)
            .unwrap_err()
            .message
            .contains("header"));
        assert!(link_message("0,0,1,0,x\n").contains("record"));
        assert!(
            link_message("0,x,1,0,a,b\n").contains("row 1")
                && link_message("0,x,1,0,a,b\n").contains("r1")
        );
        assert!(link_message("0,0,1,0,a,b\n0,0,1.5,0,a,b\n").contains("row 2"));
        let empty_landmass = link_message("0,0,1,0,,reason\n");
        assert!(empty_landmass.contains("landmass is empty") && empty_landmass.contains("row 1"));
        assert!(link_message("0,0,1,0,island,\n").contains("reason is empty"));
    }

    #[test]
    fn a_land_link_the_map_refuses_is_an_error_naming_the_row() {
        assert!(link_message("0,0,2,0,a,b\n").contains("not neighbours"));
        assert!(link_message("0,0,1,0,a,b\n3,0,4,0,a,b\n").contains("row 2"));
        assert!(link_message("3,0,4,0,a,b\n").contains("neither land nor coast"));
        assert!(link_message("-1,0,0,0,a,b\n").contains("not on the map"));
        let twice = link_message("0,0,1,0,a,b\n1,0,0,0,a,b\n");
        assert!(twice.contains("twice") && twice.contains("row 2"));
    }

    #[test]
    fn a_link_the_toml_map_has_is_refused_in_the_links_file() {
        let mut map = load_toml(
            "placeholder = true\n\
             [[hex]]\nq = 0\nr = 0\nkind = \"coast\"\n\
             [[hex]]\nq = 1\nr = 0\nkind = \"coast\"\n\
             [[land_link]]\na = [0, 0]\nb = [1, 0]\nnote = \"a bridge\"\n",
        )
        .unwrap();
        assert!(map.has_land_link(Hex::new(0, 0), Hex::new(1, 0)));
        let text = format!("{LINK_HEADER}1,0,0,0,a,b\n");
        let error = parse_land_links(&text, Path::new("l.csv"), &mut map).unwrap_err();
        assert!(error.message.contains("twice"));
    }

    #[test]
    fn toml_land_links_are_added_and_refusals_name_the_link() {
        let head = "placeholder = true\n\
             [[region]]\nkind = \"deep_sea\"\nq = [0, 3]\nr = [0, 0]\n\
             [[hex]]\nq = 0\nr = 0\nkind = \"coast\"\n\
             [[hex]]\nq = 1\nr = 0\nkind = \"coast\"\n";
        let map = load_toml(&format!("{head}[[land_link]]\na = [0, 0]\nb = [1, 0]\n")).unwrap();
        assert!(map.has_land_link(Hex::new(1, 0), Hex::new(0, 0)));
        // No link list, no links.
        assert_eq!(load_toml(head).unwrap().land_links().count(), 0);
        let refused = |link: &str| load_toml(&format!("{head}{link}")).unwrap_err().message;
        assert!(refused("[[land_link]]\na = [0, 0]\nb = [2, 0]\n").contains("not neighbours"));
        assert!(
            refused("[[land_link]]\na = [1, 0]\nb = [2, 0]\n").contains("neither land nor coast")
        );
        assert!(refused("[[land_link]]\na = [1, 0]\nb = [1, 1]\n").contains("not on the map"));
        assert!(refused(
            "[[land_link]]\na = [0, 0]\nb = [1, 0]\n[[land_link]]\na = [1, 0]\nb = [0, 0]\n"
        )
        .contains("twice"));
        assert!(refused("[[land_link]]\na = [0, 0]\nb = [1, 0]\nextra = 1\n").contains("extra"));
    }
}
