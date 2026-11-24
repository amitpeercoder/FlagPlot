use crate::source::{read_text, Disk, Source};
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
    load_map_from(&Disk, path)
}

/// As [`load_map`], reading through `source` (decision 0229, ruling 8).
pub(crate) fn load_map_from(source: &dyn Source, path: &Path) -> Result<Map, DataError> {
    let text = read_text(source, path)?;
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
#[allow(dead_code)]
pub fn load_land_links(path: &Path, map: &mut Map) -> Result<(), DataError> {
    load_land_links_from(&Disk, path, map)
}

/// As [`load_land_links`], reading through `source` (decision 0229, ruling 8).
pub(crate) fn load_land_links_from(
    source: &dyn Source,
    path: &Path,
    map: &mut Map,
) -> Result<(), DataError> {
    let text = read_text(source, path)?;
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

/// A generated map's frame (decision 0199): where it lies on the earth, as
/// its header says. Every quantity is a `Fixed`, read from the header's
/// decimal text exactly. Longitudes are in the frame's run: a map that
/// crosses the date line has its eastern edge above 180 (163 to 194 for 163 E
/// to 166 W), while its file's longitude column stays west negative, from
/// -180 to 180.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MapFrame {
    /// The latitude of the centre of hex (0,0), north positive.
    pub centre_lat: Fixed,
    /// The longitude of the centre of hex (0,0), in the run.
    pub centre_lon: Fixed,
    /// The factor for longitude: the cosine of the centre's latitude, as the
    /// header's literal.
    pub factor: Fixed,
    pub north: Fixed,
    pub south: Fixed,
    /// The western edge.
    pub west: Fixed,
    /// The eastern edge, in the run.
    pub east: Fixed,
}

impl MapFrame {
    /// A longitude, west negative, brought into the frame's run: raised by
    /// 360 when it is west of the western edge and the frame crosses the
    /// date line.
    pub fn in_run(&self, lon: Fixed) -> Fixed {
        if self.crosses_date_line() && lon < self.west {
            lon + Fixed::from_int(360)
        } else {
            lon
        }
    }

    /// Whether the frame crosses the date line: its eastern edge is above
    /// 180.
    pub fn crosses_date_line(&self) -> bool {
        self.east > Fixed::from_int(180)
    }
}

/// Reads a generated map's frame from its header (decision 0199, 18): the
/// lines "Projection: hex (0,0) is centred at ...", "east_nm = ...",
/// "north_nm = ..." and "Extent: every hex whose centre lies from ... to
/// ... and from ... to ...". `None` for a hand-written `.toml` map, or a map
/// whose header has none of these lines: it has no frame.
pub fn read_map_frame(path: &Path) -> Result<Option<MapFrame>, DataError> {
    read_map_frame_from(&Disk, path)
}

/// As [`read_map_frame`], reading through `source` (decision 0229, ruling 8).
pub(crate) fn read_map_frame_from(
    source: &dyn Source,
    path: &Path,
) -> Result<Option<MapFrame>, DataError> {
    if path.extension().and_then(|e| e.to_str()) != Some("csv") {
        return Ok(None);
    }
    let text = read_text(source, path)?;
    parse_map_frame(&text).map_err(|message| DataError::in_file(path, message))
}

/// A number of degrees with its hemisphere, as "12 S" or "178.5 E".
fn hemisphere_degrees(text: &str, positive: char, negative: char) -> Result<Fixed, String> {
    let text = text.trim();
    let (number, letter) = text
        .rsplit_once(' ')
        .ok_or_else(|| format!("\"{text}\" is not degrees and a hemisphere"))?;
    let value: Fixed = number
        .parse()
        .map_err(|e| format!("\"{number}\" is not a number of degrees: {e}"))?;
    if value.is_negative() {
        return Err(format!("\"{text}\": the hemisphere gives the sign"));
    }
    match letter.chars().collect::<Vec<_>>()[..] {
        [c] if c == positive => Ok(value),
        [c] if c == negative => Ok(-value),
        _ => Err(format!(
            "\"{text}\" does not end in {positive} or {negative}"
        )),
    }
}

/// Parses the frame from a generated map's text; see [`read_map_frame`].
/// The lines must agree: the centre of the projection line is the one the
/// two formulas use.
pub fn parse_map_frame(text: &str) -> Result<Option<MapFrame>, String> {
    let header: Vec<&str> = text
        .lines()
        .take_while(|line| line.starts_with('#'))
        .map(|line| line.trim_start_matches('#').trim())
        .collect();
    let find = |start: &str| header.iter().find(|line| line.starts_with(start)).copied();
    let projection = find("Projection: hex (0,0) is centred at ");
    let east_line = find("east_nm  = (lon - ");
    let north_line = find("north_nm = (lat ");
    let extent = find("Extent: every hex whose centre lies from ");
    let found = [projection, east_line, north_line, extent];
    if found.iter().all(Option::is_none) {
        return Ok(None);
    }
    let (Some(projection), Some(east_line), Some(north_line), Some(extent)) =
        (projection, east_line, north_line, extent)
    else {
        return Err(
            "the header has some of the lines of a frame (Projection, east_nm, north_nm, Extent) but not all"
                .to_string(),
        );
    };
    let bad = |line: &str| format!("the header line \"{line}\" is not as a frame writes it");

    // Projection: hex (0,0) is centred at 28.5 N, 178.5 E.
    let centre = projection
        .strip_prefix("Projection: hex (0,0) is centred at ")
        .and_then(|rest| rest.strip_suffix('.'))
        .ok_or_else(|| bad(projection))?;
    let (lat_text, lon_text) = centre.split_once(", ").ok_or_else(|| bad(projection))?;
    let centre_lat = hemisphere_degrees(lat_text, 'N', 'S')?;
    let centre_lon_given = hemisphere_degrees(lon_text, 'E', 'W')?;

    // east_nm  = (lon - 178.5) * 60 * 0.878817   (0.878817 is cos 28.5 degrees)
    let rest = east_line
        .strip_prefix("east_nm  = (lon - ")
        .ok_or_else(|| bad(east_line))?;
    let (centre_lon_text, rest) = rest.split_once(") * 60 * ").ok_or_else(|| bad(east_line))?;
    let (factor_text, rest) = rest.split_once("   (").ok_or_else(|| bad(east_line))?;
    let (factor_again, rest) = rest.split_once(" is cos ").ok_or_else(|| bad(east_line))?;
    let cos_of = rest
        .strip_suffix(" degrees)")
        .ok_or_else(|| bad(east_line))?;
    let number = |text: &str| -> Result<Fixed, String> {
        text.parse::<Fixed>()
            .map_err(|e| format!("\"{text}\" in the header is not a number: {e}"))
    };
    let centre_lon = number(centre_lon_text)?;
    let factor = number(factor_text)?;
    if factor_again != factor_text {
        return Err(bad(east_line));
    }
    let centre_magnitude = if centre_lat.is_negative() {
        -centre_lat
    } else {
        centre_lat
    };
    if number(cos_of)? != centre_magnitude {
        return Err(format!(
            "the factor is said to be the cosine of {cos_of} degrees, not of the centre's latitude"
        ));
    }
    if factor <= Fixed::ZERO || factor > Fixed::ONE {
        return Err(format!("the factor {factor_text} is not between 0 and 1"));
    }

    // north_nm = (lat + 12) * 60, or (lat - 28.5) * 60
    let rest = north_line
        .strip_prefix("north_nm = (lat ")
        .and_then(|rest| rest.strip_suffix(") * 60"))
        .ok_or_else(|| bad(north_line))?;
    let north_centre = if let Some(value) = rest.strip_prefix("+ ") {
        -number(value)?
    } else if let Some(value) = rest.strip_prefix("- ") {
        number(value)?
    } else {
        return Err(bad(north_line));
    };
    if north_centre != centre_lat {
        return Err(format!(
            "the north_nm line's centre is {north_centre}, the projection line's {centre_lat}"
        ));
    }

    // Extent: every hex whose centre lies from 1 S to 24 S and from 145 E to 172 E.
    let rest = extent
        .strip_prefix("Extent: every hex whose centre lies from ")
        .and_then(|rest| rest.strip_suffix('.'))
        .ok_or_else(|| bad(extent))?;
    let (lats, lons) = rest.split_once(" and from ").ok_or_else(|| bad(extent))?;
    let (north_text, south_text) = lats.split_once(" to ").ok_or_else(|| bad(extent))?;
    let (west_text, east_text) = lons.split_once(" to ").ok_or_else(|| bad(extent))?;
    let north = hemisphere_degrees(north_text, 'N', 'S')?;
    let south = hemisphere_degrees(south_text, 'N', 'S')?;
    let west = hemisphere_degrees(west_text, 'E', 'W')?;
    let east_given = hemisphere_degrees(east_text, 'E', 'W')?;
    if north <= south {
        return Err(format!(
            "the extent's north, {north}, is not north of its south, {south}"
        ));
    }
    let three_sixty = Fixed::from_int(360);
    let east = if east_given < west {
        east_given + three_sixty
    } else {
        east_given
    };
    let in_run = |lon: Fixed| {
        if east_given < west && lon < west {
            lon + three_sixty
        } else {
            lon
        }
    };
    if in_run(centre_lon_given) != centre_lon {
        return Err(format!(
            "the east_nm line's centre is {centre_lon}, the projection line's {centre_lon_given}"
        ));
    }
    let frame = MapFrame {
        centre_lat,
        centre_lon,
        factor,
        north,
        south,
        west,
        east,
    };
    if centre_lat < south || centre_lat > north || centre_lon < west || centre_lon > east {
        return Err("the centre of hex (0,0) is not inside the extent".to_string());
    }
    Ok(Some(frame))
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

    const PRESENT_FRAME: &str = "# Projection: hex (0,0) is centred at 12 S, 160 E.\n\
        #   east_nm  = (lon - 160) * 60 * 0.978148   (0.978148 is cos 12 degrees)\n\
        #   north_nm = (lat + 12) * 60\n\
        # Extent: every hex whose centre lies from 1 S to 24 S and from 145 E to 172 E.\n";

    const MIDWAY_FRAME: &str = "# Projection: hex (0,0) is centred at 28.5 N, 178.5 E.\n\
        #   east_nm  = (lon - 178.5) * 60 * 0.878817   (0.878817 is cos 28.5 degrees)\n\
        #   north_nm = (lat - 28.5) * 60\n\
        # Extent: every hex whose centre lies from 38 N to 18 N and from 163 E to 166 W.\n";

    fn fixed(text: &str) -> Fixed {
        text.parse().unwrap()
    }

    /// Decision 0199, 17 and 18: the header's lines are the frame, read
    /// exactly as fixed-point values, longitudes in the run.
    #[test]
    fn a_frame_is_read_from_the_header() {
        let present = parse_map_frame(&format!("# a title\n{PRESENT_FRAME}{HEADER}"))
            .unwrap()
            .unwrap();
        assert_eq!(
            present,
            MapFrame {
                centre_lat: fixed("-12"),
                centre_lon: fixed("160"),
                factor: fixed("0.978148"),
                north: fixed("-1"),
                south: fixed("-24"),
                west: fixed("145"),
                east: fixed("172"),
            }
        );
        assert!(!present.crosses_date_line());
        assert_eq!(present.in_run(fixed("-170")), fixed("-170"));
        let midway = parse_map_frame(MIDWAY_FRAME).unwrap().unwrap();
        assert_eq!(
            (midway.centre_lat, midway.centre_lon, midway.factor),
            (fixed("28.5"), fixed("178.5"), fixed("0.878817"))
        );
        assert_eq!(
            (midway.north, midway.south, midway.west, midway.east),
            (fixed("38"), fixed("18"), fixed("163"), fixed("194"))
        );
        assert!(midway.crosses_date_line());
        assert_eq!(midway.in_run(fixed("-177.4")), fixed("182.6"));
        assert_eq!(midway.in_run(fixed("170")), fixed("170"));
    }

    #[test]
    fn a_map_with_no_frame_lines_has_none_and_a_part_of_one_is_an_error() {
        assert_eq!(parse_map_frame(HEADER).unwrap(), None);
        let partial: String = PRESENT_FRAME
            .lines()
            .take(3)
            .map(|l| format!("{l}\n"))
            .collect();
        assert!(parse_map_frame(&partial).is_err());
        // The lines must agree with one another.
        for (from, to) in [
            ("(lon - 160)", "(lon - 161)"),
            ("(lat + 12)", "(lat - 12)"),
            ("is cos 12 degrees", "is cos 13 degrees"),
            ("0.978148   (0.978148", "0.978148   (0.97815"),
            ("from 1 S to 24 S", "from 24 S to 1 S"),
            ("12 S, 160 E.", "12 Q, 160 E."),
        ] {
            let text = PRESENT_FRAME.replace(from, to);
            assert!(parse_map_frame(&text).is_err(), "{text}");
        }
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
