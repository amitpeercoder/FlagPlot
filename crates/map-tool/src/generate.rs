//! Generating the map (decision 0017, decisions 1, 4, 5, 6 and 7).

use std::collections::{BTreeMap, BTreeSet};

use sim_core::hex::Hex;

use crate::land::{LandIndex, Polygon};
use crate::projection::{
    centre_nm, centre_of, degrees_text, from_nm, hex_of, hex_of_nm, to_nm, NM_PER_DEG_LAT,
    NM_PER_DEG_LON,
};
use crate::water::{nearest_sea_point, SEARCH_NM};

/// Extent: every hex whose centre lies in these bounds, in degrees.
pub const LAT_MIN: f64 = -24.0;
pub const LAT_MAX: f64 = -1.0;
pub const LON_MIN: f64 = 145.0;
pub const LON_MAX: f64 = 172.0;
/// Distance between lattice points, in nm.
pub const LATTICE_NM: i32 = 2;
/// Inradius of a hexagon (centre to flat side), in nm.
pub const INRADIUS_NM: f64 = 20.0;
/// More than a hexagon's circumradius (23.09 nm), for bounding boxes.
const HALF_HEIGHT_BOUND_NM: f64 = 24.0;
/// Land percentage at which a hex is land.
pub const LAND_FROM_PERCENT: u32 = 80;
/// The date written in the file header. A literal: no wall clock.
pub const GENERATED_ON: &str = "2026-11-10";

/// What a hex is.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Land,
    Coast,
    DeepSea,
}

impl Kind {
    pub fn as_str(self) -> &'static str {
        match self {
            Kind::Land => "land",
            Kind::Coast => "coast",
            Kind::DeepSea => "deep_sea",
        }
    }
}

/// One generated hex.
#[derive(Clone, Debug, PartialEq)]
pub struct MapHex {
    pub q: i32,
    pub r: i32,
    pub kind: Kind,
    pub land_percent: u32,
    /// Lattice points that belong to this hex.
    pub points: u32,
    /// Lattice points of this hex that fell on land.
    pub land_points: u32,
    /// Bodies of water in the hex (decision 0017, decision 6).
    pub water_bodies: u32,
    /// Points in the main body of water; 0 when the hex has no sea point.
    pub main_body_points: u32,
    pub lat: f64,
    pub lon: f64,
    /// Closed sides, indexed like [`Hex::DIRECTIONS`] (decision 0017,
    /// decision 7).
    pub closed_sides: [bool; 6],
}

/// The generated map.
#[derive(Clone, Debug)]
pub struct GeneratedMap {
    pub hexes: Vec<MapHex>,
    /// Hexes with at least one land point but 0 percent after rounding down.
    pub zero_percent_coast: usize,
    /// What each anchorage did, in the order of the anchors file; empty when
    /// no anchors were given.
    pub anchors: Vec<AnchorNote>,
}

impl GeneratedMap {
    /// Hexes with more than one body of water.
    pub fn multi_body_hexes(&self) -> usize {
        self.hexes.iter().filter(|h| h.water_bodies > 1).count()
    }

    /// Closed sides between two hexes that are both not land, each pair
    /// counted once. The neighbour must be on the map.
    pub fn closed_sides_between_non_land(&self) -> usize {
        let by_hex: BTreeMap<(i32, i32), &MapHex> =
            self.hexes.iter().map(|h| ((h.q, h.r), h)).collect();
        let mut pairs = 0;
        for hex in &self.hexes {
            if hex.kind == Kind::Land {
                continue;
            }
            for (direction, (dq, dr)) in Hex::DIRECTIONS.iter().enumerate() {
                let other = (hex.q + dq, hex.r + dr);
                let Some(neighbor) = by_hex.get(&other) else {
                    continue;
                };
                if hex.closed_sides[direction]
                    && neighbor.kind != Kind::Land
                    && (hex.q, hex.r) < other
                {
                    pairs += 1;
                }
            }
        }
        pairs
    }
}

/// The projected position in nm of lattice point (i, j): east = 2i, north =
/// 2j. The lattice is global and has a point at the origin.
pub fn lattice_nm(i: i32, j: i32) -> (f64, f64) {
    (f64::from(i * LATTICE_NM), f64::from(j * LATTICE_NM))
}

/// The hex a lattice point belongs to: the hex whose centre is nearest, by
/// the cube rounding of `hex_of_nm` on the projected position. The only
/// exact ties among lattice points are points on a vertical side between two
/// hexes of one row (exactly 20 nm east of one centre and 20 nm west of the
/// other; the centres' east coordinates are multiples of 20, so this is exact
/// in `f64`). Cube rounding decides those by float noise, so the rule is
/// explicit: a point exactly on a vertical side belongs to the hex to its
/// east (larger q, same r).
pub fn lattice_hex(i: i32, j: i32) -> (i32, i32) {
    let (east, north) = lattice_nm(i, j);
    let (q, r) = hex_of_nm(east, north);
    let (centre_east, _) = centre_nm(q, r);
    if east - centre_east == INRADIUS_NM {
        // On the east side of the hex rounding picked: the hex to the east.
        (q + 1, r)
    } else {
        (q, r)
    }
}

/// Every lattice point (i, j) that belongs to hex (q, r), in ascending
/// order of i, then j.
pub fn hex_points(q: i32, r: i32) -> Vec<(i32, i32)> {
    let (centre_east, centre_north) = centre_nm(q, r);
    let step = f64::from(LATTICE_NM);
    let i_low = ((centre_east - INRADIUS_NM) / step).floor() as i32 - 1;
    let i_high = ((centre_east + INRADIUS_NM) / step).floor() as i32 + 1;
    let j_low = ((centre_north - HALF_HEIGHT_BOUND_NM) / step).floor() as i32 - 1;
    let j_high = ((centre_north + HALF_HEIGHT_BOUND_NM) / step).floor() as i32 + 1;
    let mut points = Vec::new();
    for i in i_low..=i_high {
        for j in j_low..=j_high {
            if lattice_hex(i, j) == (q, r) {
                points.push((i, j));
            }
        }
    }
    points
}

/// One body of water inside a hex: sea points joined north, south, east or
/// west, through points of the same hex only.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Body {
    pub size: u32,
    /// The point furthest north (largest j), and of those the furthest west
    /// (smallest i).
    pub top: (i32, i32),
}

/// Whether point `a` is further north than `b`, or equally north and
/// further west: `a` maximises (j, -i).
fn is_further_north_then_west(a: (i32, i32), b: (i32, i32)) -> bool {
    a.1 > b.1 || (a.1 == b.1 && a.0 < b.0)
}

impl Body {
    /// Whether this body beats `other` as a hex's main body: larger, or the
    /// same size and holding the northernmost (then westernmost) point.
    pub fn beats(&self, other: &Body) -> bool {
        self.size > other.size
            || (self.size == other.size && is_further_north_then_west(self.top, other.top))
    }
}

/// One hex's lattice points, land and bodies of water.
#[derive(Clone, Debug)]
pub struct HexAnalysis {
    pub q: i32,
    pub r: i32,
    pub points: Vec<(i32, i32)>,
    pub land_points: u32,
    pub bodies: Vec<Body>,
    /// Index into `bodies` of the main body, if the hex has a sea point.
    pub main: Option<usize>,
    /// The body each sea point is in.
    pub body_of: BTreeMap<(i32, i32), usize>,
}

impl HexAnalysis {
    /// Whether the point is in the main body.
    pub fn in_main(&self, point: (i32, i32)) -> bool {
        self.main.is_some() && self.body_of.get(&point).copied() == self.main
    }
}

/// Works out one hex's land points and bodies of water (decision 0017,
/// decision 6). `is_land(i, j)` says whether lattice point (i, j) is land.
/// Sea points are grouped by 4-neighbour adjacency on the global lattice,
/// but only through points of this hex. The main body is the largest; where
/// two are the same size it is the one holding the point furthest north,
/// then furthest west.
pub fn analyse_hex(q: i32, r: i32, is_land: &dyn Fn(i32, i32) -> bool) -> HexAnalysis {
    let points = hex_points(q, r);
    let mut sea: BTreeSet<(i32, i32)> = BTreeSet::new();
    let mut land_points = 0;
    for &(i, j) in &points {
        if is_land(i, j) {
            land_points += 1;
        } else {
            sea.insert((i, j));
        }
    }
    let mut body_of: BTreeMap<(i32, i32), usize> = BTreeMap::new();
    let mut bodies: Vec<Body> = Vec::new();
    for &start in &sea {
        if body_of.contains_key(&start) {
            continue;
        }
        let index = bodies.len();
        let mut body = Body {
            size: 0,
            top: start,
        };
        let mut stack = vec![start];
        body_of.insert(start, index);
        while let Some((i, j)) = stack.pop() {
            body.size += 1;
            if is_further_north_then_west((i, j), body.top) {
                body.top = (i, j);
            }
            for next in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
                if sea.contains(&next) && !body_of.contains_key(&next) {
                    body_of.insert(next, index);
                    stack.push(next);
                }
            }
        }
        bodies.push(body);
    }
    let mut main: Option<usize> = None;
    for (index, body) in bodies.iter().enumerate() {
        if main.is_none_or(|m| body.beats(&bodies[m])) {
            main = Some(index);
        }
    }
    HexAnalysis {
        q,
        r,
        points,
        land_points,
        bodies,
        main,
        body_of,
    }
}

/// Percentage (rounded down) and kind from a land point count.
pub fn classify(land_points: u32, points: u32) -> (u32, Kind) {
    let percent = 100 * land_points / points;
    let kind = if land_points == 0 {
        Kind::DeepSea
    } else if percent >= LAND_FROM_PERCENT {
        Kind::Land
    } else {
        Kind::Coast
    };
    (percent, kind)
}

/// Every hex whose centre lies in the extent, sorted by r then q.
pub fn extent_hexes() -> Vec<(i32, i32)> {
    let mut hexes = Vec::new();
    for r in -40..=40 {
        for q in -60..=60 {
            let (lat, lon) = centre_of(q, r);
            if (LAT_MIN..=LAT_MAX).contains(&lat) && (LON_MIN..=LON_MAX).contains(&lon) {
                hexes.push((q, r));
            }
        }
    }
    hexes
}

/// Keeps the polygons whose bounding box touches the extent padded by one
/// hex (40 nm of latitude and of longitude).
pub fn keep_near_extent(polygons: Vec<Polygon>) -> Vec<Polygon> {
    let pad_lat = 40.0 / NM_PER_DEG_LAT;
    let pad_lon = 40.0 / NM_PER_DEG_LON;
    polygons
        .into_iter()
        .filter(|polygon| {
            polygon.bbox().is_some_and(|b| {
                b.2 >= LON_MIN - pad_lon
                    && b.0 <= LON_MAX + pad_lon
                    && b.3 >= LAT_MIN - pad_lat
                    && b.1 <= LAT_MAX + pad_lat
            })
        })
        .collect()
}

/// Whether lattice point (i, j) is land in the index. The one place a
/// lattice point is turned into a position: projected nm to latitude and
/// longitude, never the other way.
pub fn lattice_point_is_land(index: &LandIndex, i: i32, j: i32) -> bool {
    let (east, north) = lattice_nm(i, j);
    let (lat, lon) = from_nm(east, north);
    index.contains(lon, lat)
}

/// The direction (index into [`Hex::DIRECTIONS`]) from one hex to another,
/// if they are neighbours.
fn direction_between(from: (i32, i32), to: (i32, i32)) -> Option<usize> {
    let offset = (to.0 - from.0, to.1 - from.1);
    Hex::DIRECTIONS.iter().position(|d| *d == offset)
}

/// The fewest lattice points a body of water needs to be chosen by an
/// anchorage (decision 0017, decision 6).
pub const MIN_ANCHOR_BODY_POINTS: u32 = 5;

/// The columns of the anchors file, exactly.
pub const ANCHORS_HEADER: [&str; 7] = [
    "place", "feature", "lat", "lon", "source", "locator", "accessed",
];

/// One anchorage: a place and its sourced position in degrees, south and
/// west negative. The source, locator and date columns stay in the file.
#[derive(Clone, Debug, PartialEq)]
pub struct Anchorage {
    pub place: String,
    pub feature: String,
    pub lat: f64,
    pub lon: f64,
}

impl Anchorage {
    fn describe(&self) -> String {
        format!(
            "anchorage {} ({}) at lat {}, lon {}",
            self.place,
            self.feature,
            degrees_text(self.lat),
            degrees_text(self.lon)
        )
    }
}

/// Reads the anchors file: CSV with `#` comment lines, the header
/// [`ANCHORS_HEADER`] exactly, quoted fields allowed.
pub fn parse_anchors(text: &str) -> Result<Vec<Anchorage>, String> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .comment(Some(b'#'))
        .from_reader(text.as_bytes());
    let mut records = reader.records();
    let header = records
        .next()
        .ok_or("the anchors file has no header line")?
        .map_err(|e| e.to_string())?;
    if header.iter().collect::<Vec<_>>() != ANCHORS_HEADER {
        return Err(format!(
            "the anchors file header must be exactly {}",
            ANCHORS_HEADER.join(",")
        ));
    }
    let mut anchors: Vec<Anchorage> = Vec::new();
    for record in records {
        let record = record.map_err(|e| e.to_string())?;
        let line = record.position().map_or(0, |p| p.line());
        if record.len() != ANCHORS_HEADER.len() {
            return Err(format!(
                "anchors file line {line}: {} fields, expected {}",
                record.len(),
                ANCHORS_HEADER.len()
            ));
        }
        let degrees = |index: usize, limit: f64| -> Result<f64, String> {
            let text = &record[index];
            match text.parse::<f64>() {
                Ok(v) if v.is_finite() && v.abs() <= limit => Ok(v),
                _ => Err(format!(
                    "anchors file line {line}: {} \"{text}\" is not a number of degrees",
                    ANCHORS_HEADER[index]
                )),
            }
        };
        let place = record[0].to_string();
        if place.is_empty() {
            return Err(format!("anchors file line {line}: the place is empty"));
        }
        if anchors.iter().any(|a| a.place == place) {
            return Err(format!("anchors file line {line}: {place} is listed twice"));
        }
        anchors.push(Anchorage {
            place,
            feature: record[1].to_string(),
            lat: degrees(2, 90.0)?,
            lon: degrees(3, 180.0)?,
        });
    }
    Ok(anchors)
}

/// What an anchorage did to its hex.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnchorOutcome {
    /// Its body became the hex's main body in place of another.
    Changed { from_points: u32, to_points: u32 },
    /// Its body was already the hex's main body.
    Unchanged { points: u32 },
    /// Its body has fewer than [`MIN_ANCHOR_BODY_POINTS`] points; the hex
    /// kept its largest body, of `main_points`.
    Ignored { points: u32, main_points: u32 },
}

/// One anchorage's effect.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnchorNote {
    pub place: String,
    pub hex: (i32, i32),
    pub outcome: AnchorOutcome,
}

impl AnchorNote {
    /// The line of the file header, without the `# `.
    pub fn text(&self) -> String {
        let what = match self.outcome {
            AnchorOutcome::Changed {
                from_points,
                to_points,
            } => format!("changed the main body of water, from {from_points} points to {to_points}"),
            AnchorOutcome::Unchanged { points } => format!(
                "its body of water ({points} points) was already the main body, no change"
            ),
            AnchorOutcome::Ignored {
                points,
                main_points,
            } => format!(
                "ignored: its body of water has {points} points, fewer than {MIN_ANCHOR_BODY_POINTS}; the main body stays at {main_points} points"
            ),
        };
        format!(
            "  {}: hex ({},{}): {what}",
            self.place, self.hex.0, self.hex.1
        )
    }
}

/// Applies the anchorages to the hexes' main bodies (decision 0017,
/// decision 6). `extent` and `analyses` run in step.
///
/// For each anchorage: its hex is the one whose centre is nearest; the body
/// is the one holding the nearest sea point of that hex within
/// [`SEARCH_NM`] (the search of the `water` command, ties north then west,
/// but only over the sea points of the anchorage's own hex). A body of
/// [`MIN_ANCHOR_BODY_POINTS`] or more becomes the hex's main body; a smaller
/// one is ignored. Two anchorages of one hex whose bodies both qualify and
/// differ are an error.
fn apply_anchors(
    extent: &[(i32, i32)],
    analyses: &mut [HexAnalysis],
    anchors: &[Anchorage],
) -> Result<Vec<AnchorNote>, String> {
    struct Pick {
        hex_index: usize,
        body: usize,
        size: u32,
    }
    let mut picks: Vec<Pick> = Vec::new();
    for anchor in anchors {
        let hex = hex_of(anchor.lat, anchor.lon);
        let Some(hex_index) = extent.iter().position(|e| *e == hex) else {
            return Err(format!(
                "{} is in hex ({},{}), outside the extent",
                anchor.describe(),
                hex.0,
                hex.1
            ));
        };
        let analysis = &analyses[hex_index];
        if classify(analysis.land_points, analysis.points.len() as u32).1 == Kind::Land {
            return Err(format!(
                "{} is in hex ({},{}), which is a land hex",
                anchor.describe(),
                hex.0,
                hex.1
            ));
        }
        let (east, north) = to_nm(anchor.lat, anchor.lon);
        let mut is_sea = |i: i32, j: i32| analysis.body_of.contains_key(&(i, j));
        let Some((point, _)) = nearest_sea_point(east, north, &mut is_sea) else {
            return Err(format!(
                "{} has no sea point of hex ({},{}) within {SEARCH_NM} nm",
                anchor.describe(),
                hex.0,
                hex.1
            ));
        };
        let body = analysis.body_of[&point];
        picks.push(Pick {
            hex_index,
            body,
            size: analysis.bodies[body].size,
        });
    }
    for (a, first) in picks.iter().enumerate() {
        for (b, second) in picks.iter().enumerate().skip(a + 1) {
            if first.hex_index == second.hex_index
                && first.body != second.body
                && first.size >= MIN_ANCHOR_BODY_POINTS
                && second.size >= MIN_ANCHOR_BODY_POINTS
            {
                let hex = extent[first.hex_index];
                return Err(format!(
                    "{} and {} are both in hex ({},{}) but pick different bodies of water ({} and {} points)",
                    anchors[a].describe(),
                    anchors[b].describe(),
                    hex.0,
                    hex.1,
                    first.size,
                    second.size
                ));
            }
        }
    }
    // The outcomes are told against the main bodies before any change.
    let before: Vec<Option<usize>> = analyses.iter().map(|a| a.main).collect();
    let mut notes = Vec::new();
    for (anchor, pick) in anchors.iter().zip(&picks) {
        let analysis = &mut analyses[pick.hex_index];
        let main_before = before[pick.hex_index].map(|m| analysis.bodies[m].size);
        let outcome = if pick.size < MIN_ANCHOR_BODY_POINTS {
            AnchorOutcome::Ignored {
                points: pick.size,
                main_points: main_before.unwrap_or(0),
            }
        } else {
            analysis.main = Some(pick.body);
            if before[pick.hex_index] == Some(pick.body) {
                AnchorOutcome::Unchanged { points: pick.size }
            } else {
                AnchorOutcome::Changed {
                    from_points: main_before.unwrap_or(0),
                    to_points: pick.size,
                }
            }
        };
        notes.push(AnchorNote {
            place: anchor.place.clone(),
            hex: extent[pick.hex_index],
            outcome,
        });
    }
    Ok(notes)
}

/// Generates the map from land polygons (longitude, latitude), without
/// anchorages.
pub fn generate(polygons: Vec<Polygon>) -> GeneratedMap {
    match generate_with_anchors(polygons, &[]) {
        Ok(map) => map,
        // With no anchorages there is nothing to reject.
        Err(message) => unreachable!("{message}"),
    }
}

/// Generates the map from land polygons (longitude, latitude), and the
/// anchorages that decide some hexes' main bodies of water (decision 0017,
/// decision 6). An anchorage that cannot be placed is an error.
pub fn generate_with_anchors(
    polygons: Vec<Polygon>,
    anchors: &[Anchorage],
) -> Result<GeneratedMap, String> {
    let index = LandIndex::new(&keep_near_extent(polygons));
    let is_land = |i: i32, j: i32| lattice_point_is_land(&index, i, j);
    let extent = extent_hexes();
    let mut analyses: Vec<HexAnalysis> = extent
        .iter()
        .map(|&(q, r)| analyse_hex(q, r, &is_land))
        .collect();
    let anchor_notes = apply_anchors(&extent, &mut analyses, anchors)?;

    // Closed sides (decision 0017, decision 7): a side is open when a point
    // of one hex's main body is next to (north, south, east or west on the
    // lattice) a point of the neighbour's main body. One pass over the
    // main-body points.
    let mut main_points: BTreeMap<(i32, i32), usize> = BTreeMap::new();
    for (h, analysis) in analyses.iter().enumerate() {
        for (&point, &body) in &analysis.body_of {
            if Some(body) == analysis.main {
                main_points.insert(point, h);
            }
        }
    }
    let mut open = vec![[false; 6]; extent.len()];
    for (&(i, j), &h) in &main_points {
        for next in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
            let Some(&other) = main_points.get(&next) else {
                continue;
            };
            if other == h {
                continue;
            }
            if let Some(direction) = direction_between(extent[h], extent[other]) {
                open[h][direction] = true;
                open[other][(direction + 3) % 6] = true;
            }
        }
    }
    let position: BTreeMap<(i32, i32), usize> = extent
        .iter()
        .enumerate()
        .map(|(h, hex)| (*hex, h))
        .collect();

    let mut hexes = Vec::new();
    let mut zero_percent_coast = 0;
    for (h, analysis) in analyses.iter().enumerate() {
        let (q, r) = extent[h];
        let points = analysis.points.len() as u32;
        let (land_percent, kind) = classify(analysis.land_points, points);
        if kind == Kind::Coast && land_percent == 0 {
            zero_percent_coast += 1;
        }
        let (lat, lon) = centre_of(q, r);
        // A side toward a hex outside the extent has no partner: closed.
        let closed_sides = open[h].map(|is_open| !is_open);
        for (direction, (dq, dr)) in Hex::DIRECTIONS.iter().enumerate() {
            if let Some(&other) = position.get(&(q + dq, r + dr)) {
                assert_eq!(
                    closed_sides[direction],
                    !open[other][(direction + 3) % 6],
                    "closed sides of ({q},{r}) and its neighbour in direction {direction} disagree"
                );
            }
        }
        hexes.push(MapHex {
            q,
            r,
            kind,
            land_percent,
            points,
            land_points: analysis.land_points,
            water_bodies: analysis.bodies.len() as u32,
            main_body_points: analysis.main.map_or(0, |m| analysis.bodies[m].size),
            lat,
            lon,
            closed_sides,
        });
    }
    Ok(GeneratedMap {
        hexes,
        zero_percent_coast,
        anchors: anchor_notes,
    })
}

/// The closed sides as text: direction indexes ascending, joined by `|`;
/// empty when none are closed.
pub fn closed_sides_text(closed: &[bool; 6]) -> String {
    closed
        .iter()
        .enumerate()
        .filter(|(_, c)| **c)
        .map(|(d, _)| d.to_string())
        .collect::<Vec<_>>()
        .join("|")
}

/// The text of the CSV file: comment header, column header, one row per hex.
///
/// `anchors_file` is the anchors file's name when one was used: the header
/// then names it and gives one line per anchorage. Without it the header is
/// as it was before anchorages.
pub fn csv_text(map: &GeneratedMap, command_line: &str, anchors_file: Option<&str>) -> String {
    let count = |kind: Kind| map.hexes.iter().filter(|h| h.kind == kind).count();
    let smallest = map.hexes.iter().map(|h| h.points).min().unwrap_or(0);
    let largest = map.hexes.iter().map(|h| h.points).max().unwrap_or(0);
    let mut out = String::new();
    let mut line = |text: &str| {
        out.push_str("# ");
        out.push_str(text);
        out.push('\n');
    };
    line("Flag Plot map of the south-west Pacific slice (decision 0017).");
    line("Source: Natural Earth 1:10m land and minor islands, version 5.1.1, public domain, naturalearthdata.com");
    line("  ne_10m_land.zip          SHA-256 e547d749445eaa0964aba76738090ec88f5e63c4585122170f98c67a7ea922dc");
    line("  ne_10m_minor_islands.zip SHA-256 47c0b3ce26df7b4dbabdbb251023918cb2f5c83462a426288f1f253da7284db2");
    line("Projection: hex (0,0) is centred at 12 S, 160 E.");
    line("  east_nm  = (lon - 160) * 60 * 0.978148   (0.978148 is cos 12 degrees)");
    line("  north_nm = (lat + 12) * 60");
    line("  hex (q,r) centre: east = 40 * (q + r/2) nm, south = 34.641016 * r nm (north is decreasing r)");
    line("  a point belongs to the hex whose centre is nearest.");
    line("Extent: every hex whose centre lies from 1 S to 24 S and from 145 E to 172 E.");
    line("Lattice: one global square lattice, 2 nm apart in projected nm, with a point at the origin (east = 2i nm, north = 2j nm for whole i and j).");
    line(&format!(
        "  Each point belongs to the hex whose centre is nearest (cube rounding); a point exactly on a vertical side of a hexagon goes to the hex to its east. {smallest} to {largest} points per hex."
    ));
    line("  A lattice point is land when it is inside a Natural Earth polygon (even-odd rule over the rings of each polygon record; holes work).");
    line("Thresholds: land_percent = 100 * land points / points, rounded down. land: 80 or more; coast: 1 to 79; deep_sea: 0 land points.");
    line(&format!(
        "  A hex with at least one land point but 0 percent after rounding counts as coast with land_percent 0: {} such hexes.",
        map.zero_percent_coast
    ));
    line("Bodies of water: a hex's sea points joined N/S/E/W within the hex; the main body is the largest (tie: holds the northernmost, then westernmost point).");
    if let Some(file) = anchors_file {
        line(&format!(
            "Anchors: {file}. A hex that holds an anchorage stands for the body of water nearest it, if that body has at least {MIN_ANCHOR_BODY_POINTS} points (decision 0017, decision 6); the main bodies below are worked out after that."
        ));
        for note in &map.anchors {
            line(&note.text());
        }
    }
    line("closed_sides: directions (0..5 in the order (1,0) (1,-1) (0,-1) (-1,0) (-1,1) (0,1)) whose neighbour is outside the extent or whose main body touches no point of this hex's main body; ships cannot cross.");
    line(&format!(
        "Hexes: {} ({} land, {} coast, {} deep_sea).",
        map.hexes.len(),
        count(Kind::Land),
        count(Kind::Coast),
        count(Kind::DeepSea)
    ));
    line(&format!(
        "  Hexes with more than one body of water: {}. Closed sides between two hexes that are both not land: {} (each pair counted once).",
        map.multi_body_hexes(),
        map.closed_sides_between_non_land()
    ));
    line("lat and lon are the centre of the hex in degrees, south and west negative. water_bodies is the number of bodies of water in the hex.");
    line(&format!(
        "Generated {GENERATED_ON} by `cargo run --release -p map-tool -- {command_line}`"
    ));
    out.push_str("q,r,kind,land_percent,lat,lon,water_bodies,closed_sides\n");
    for hex in &map.hexes {
        out.push_str(&format!(
            "{},{},{},{},{},{},{},{}\n",
            hex.q,
            hex.r,
            hex.kind.as_str(),
            hex.land_percent,
            degrees_text(hex.lat),
            degrees_text(hex.lon),
            hex.water_bodies,
            closed_sides_text(&hex.closed_sides)
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::land::Ring;
    use sim_core::map::{HexData, HexKind, Map};

    /// A rectangle given in projected nm, as a polygon in lon/lat.
    fn rect_nm(east: (f64, f64), north: (f64, f64)) -> Ring {
        let corner = |e: f64, n: f64| {
            let (lat, lon) = from_nm(e, n);
            (lon, lat)
        };
        Ring {
            points: vec![
                corner(east.0, north.0),
                corner(east.1, north.0),
                corner(east.1, north.1),
                corner(east.0, north.1),
                corner(east.0, north.0),
            ],
        }
    }

    fn find(map: &GeneratedMap, q: i32, r: i32) -> &MapHex {
        map.hexes
            .iter()
            .find(|h| (h.q, h.r) == (q, r))
            .expect("the hex is in the extent")
    }

    /// The generated hexes as a `sim_core` map, closed sides included.
    fn to_map(generated: &GeneratedMap) -> Map {
        let mut map = Map::new();
        for hex in &generated.hexes {
            map.insert(
                Hex::new(hex.q, hex.r),
                HexData {
                    kind: match hex.kind {
                        Kind::Land => HexKind::Land,
                        Kind::Coast => HexKind::Coast,
                        Kind::DeepSea => HexKind::DeepSea,
                    },
                    land_percent: hex.land_percent as u8,
                    closed_sides: hex.closed_sides,
                    ..HexData::open_sea()
                },
            );
        }
        map
    }

    /// Hand count of the lattice for hex (0,0), whose centre is a lattice
    /// point. The points strictly inside the hexagon: per column dx = 0,
    /// +-2, ... +-18, the even dy with |dy| < (40 - |dx|) / sqrt 3
    /// (|dx|/2 + |dy| sqrt3/2 < 20 rearranged):
    ///
    /// | dx | (40-dx)/sqrt3 | dy                 | points |
    /// | -- | ------------- | ------------------ | ------ |
    /// | 0  | 23.09         | -22 to 22          | 23     |
    /// | 2  | 21.94         | -20 to 20          | 21     |
    /// | 4  | 20.78         | -20 to 20          | 21     |
    /// | 6  | 19.63         | -18 to 18          | 19     |
    /// | 8  | 18.48         | -18 to 18          | 19     |
    /// | 10 | 17.32         | -16 to 16          | 17     |
    /// | 12 | 16.17         | -16 to 16          | 17     |
    /// | 14 | 15.01         | -14 to 14          | 15     |
    /// | 16 | 13.86         | -12 to 12          | 13     |
    /// | 18 | 12.70         | -12 to 12          | 13     |
    ///
    /// Columns dx = 2 to 18 sum to 21+21+19+19+17+17+15+13+13 = 155, and
    /// there are two sides of dx = 0: 23 + 2 x 155 = 333 strictly inside.
    /// The vertical sides are dx = -20 and dx = +20, each with the even dy
    /// with |dy| < 20 / sqrt 3 = 11.55, that is -10 to 10: 11 points. A
    /// point on a vertical side is a tie with the hex across it and goes to
    /// the hex to the east. So the 11 points on the west side (dx = -20)
    /// belong to this hex, and the 11 on the east side (dx = +20) to the
    /// eastern neighbour. 333 + 11 = 344.
    #[test]
    fn a_hexagon_in_row_zero_has_344_lattice_points() {
        let points = hex_points(0, 0);
        assert_eq!(points.len(), 344);
        let column = |dx: i32| points.iter().filter(|p| p.0 * 2 == dx).count();
        let expected = [23, 21, 21, 19, 19, 17, 17, 15, 13, 13];
        for (i, count) in expected.iter().enumerate() {
            let dx = 2 * i as i32;
            assert_eq!(column(dx), *count, "column {dx}");
            assert_eq!(column(-dx), *count, "column -{dx}");
        }
        assert_eq!(column(-20), 11, "the west side belongs to this hex");
        assert_eq!(column(20), 0, "the east side belongs to the next hex");
        assert_eq!(points.iter().filter(|p| p.0 * 2 > -20).count(), 333);
        assert!(points.contains(&(0, 0)));
        // The west side is dy = -10 to 10.
        let side: Vec<i32> = points
            .iter()
            .filter(|p| p.0 == -10)
            .map(|p| p.1 * 2)
            .collect();
        assert_eq!(side, vec![-10, -8, -6, -4, -2, 0, 2, 4, 6, 8, 10]);
    }

    /// A lattice point exactly on a vertical side belongs to the hex to its
    /// east; the east and west neighbours of a hex in row 0 share the same
    /// 11-point side.
    #[test]
    fn a_point_on_a_vertical_side_goes_east() {
        // (east, north) = (20, 0) is lattice point (10, 0): 20 from the
        // centres of (0,0) and (1,0).
        assert_eq!(lattice_hex(10, 0), (1, 0));
        assert_eq!(lattice_hex(-10, 0), (0, 0));
        assert_eq!(lattice_hex(9, 0), (0, 0));
        assert_eq!(lattice_hex(11, 0), (1, 0));
    }

    /// Over a window of lattice points, the assigned hex is a nearest centre
    /// (within 1e-6 nm), and every exact tie went east.
    #[test]
    fn every_lattice_point_goes_to_a_nearest_centre_and_vertical_ties_go_east() {
        let mut ties = 0;
        for i in -60..=60 {
            for j in -60..=60 {
                let (east, north) = lattice_nm(i, j);
                let assigned = lattice_hex(i, j);
                let dist = |q: i32, r: i32| {
                    let (ce, cn) = centre_nm(q, r);
                    ((east - ce) * (east - ce) + (north - cn) * (north - cn)).sqrt()
                };
                let mut nearest = f64::MAX;
                for r in -8..=8 {
                    for q in -10..=10 {
                        nearest = nearest.min(dist(q, r));
                    }
                }
                let chosen = dist(assigned.0, assigned.1);
                assert!(
                    chosen - nearest < 1e-6,
                    "point ({i},{j}) went to {assigned:?}"
                );
                // Every hex as near (within 1e-6) as the assigned one.
                let mut as_near = Vec::new();
                for r in -8..=8 {
                    for q in -10..=10 {
                        if dist(q, r) - nearest < 1e-6 {
                            as_near.push((q, r));
                        }
                    }
                }
                if as_near.len() > 1 {
                    ties += 1;
                    // Only vertical-side ties: two hexes of one row, the
                    // assigned one the eastern (larger q).
                    assert_eq!(as_near.len(), 2, "point ({i},{j}): {as_near:?}");
                    assert_eq!(as_near[0].1, as_near[1].1);
                    assert_eq!(as_near[1].0, as_near[0].0 + 1);
                    assert_eq!(assigned, as_near[1], "point ({i},{j}) tie went west");
                    assert_eq!(east - centre_nm(assigned.0, assigned.1).0, -INRADIUS_NM);
                }
            }
        }
        // Some ties were seen: every hex in rows with a lattice-aligned
        // vertical side has them.
        assert!(ties > 20, "{ties}");
    }

    /// No lattice point is counted twice or lost. For a block of hexes: the
    /// points of all of them are distinct, and every point of a window that
    /// `lattice_hex` gives to a hex of the block is in that hex's list.
    #[test]
    fn hex_point_lists_tile_the_lattice_without_overlap_or_gaps() {
        let mut block = Vec::new();
        for r in -2..=2 {
            for q in -3..=3 {
                block.push((q, r));
            }
        }
        let mut seen: BTreeSet<(i32, i32)> = BTreeSet::new();
        let mut total = 0;
        for &(q, r) in &block {
            let points = hex_points(q, r);
            assert!((320..=360).contains(&points.len()), "({q},{r})");
            total += points.len();
            seen.extend(points);
        }
        assert_eq!(seen.len(), total, "a point is in two hexes");
        // Every window point whose hex is in the block is listed.
        let mut in_block = 0;
        for i in -90..=90 {
            for j in -90..=90 {
                if block.contains(&lattice_hex(i, j)) {
                    assert!(seen.contains(&(i, j)), "({i},{j}) is lost");
                    in_block += 1;
                }
            }
        }
        // The window covers the block completely: hexes reach at most 160 +
        // 20 + 40 = 220 nm east, and north 69.3 + 24 = 94 nm: i up to 110.
        // So only the listed points inside the window are compared.
        let inside_window = seen
            .iter()
            .filter(|p| (-90..=90).contains(&p.0) && (-90..=90).contains(&p.1))
            .count();
        assert_eq!(in_block, inside_window);
    }

    #[test]
    fn extent_hexes_have_point_counts_between_the_row_zero_extremes() {
        // Row 0 hexes have 344 points. Other rows differ by a few.
        let counts: Vec<usize> = [(0, 0), (0, 1), (1, 1), (5, -4), (-7, 5)]
            .iter()
            .map(|&(q, r)| hex_points(q, r).len())
            .collect();
        assert!(counts.iter().all(|c| (320..=360).contains(c)), "{counts:?}");
    }

    /// 344 points: 275 land points is 79.94 percent (79, coast) and 276 is
    /// 80.23 percent (80, land). One point is 0.29 percent: 0, still coast.
    #[test]
    fn thresholds_on_344_points() {
        assert_eq!(classify(0, 344), (0, Kind::DeepSea));
        assert_eq!(classify(1, 344), (0, Kind::Coast));
        assert_eq!(classify(3, 344), (0, Kind::Coast));
        assert_eq!(classify(4, 344), (1, Kind::Coast));
        assert_eq!(classify(275, 344), (79, Kind::Coast));
        assert_eq!(classify(276, 344), (80, Kind::Land));
        assert_eq!(classify(344, 344), (100, Kind::Land));
    }

    /// Synthetic land in projected nm, all of it inside hex (0,0), whose
    /// lattice is east = 2i, north = 2j.
    ///
    /// Polygon A: east in (1, 15), north in (-9, 9), with a hole east in
    /// (3, 9), north in (-3, 3). Edges are off the lattice by 1 nm.
    /// - Outer: columns east = 2, 4, ..., 14 (7 columns) times north =
    ///   -8, -6, ..., 8 (9 rows) = 63 points. All are inside the hexagon
    ///   (worst corner (14, 8): 7 + 6.93 < 20).
    /// - Hole: columns east = 4, 6, 8 (3) times north = -2, 0, 2 (3) = 9.
    /// - Land points 63 - 9 = 54 of 344: 5400 / 344 = 15.7, so 15 percent,
    ///   coast. The hole is sea inside the land: its own body of 9 points,
    ///   and the main body is the other 344 - 54 - 9 = 281.
    ///
    /// Polygon B: a 2 nm square around the centre of hex (3, 0), which is
    /// at east 120, north 0: east in (119, 121), north in (-1, 1) holds only
    /// that hex's centre point.
    /// 1 land point of 344: 0 percent, coast with land_percent 0.
    ///
    /// Nothing else touches any hex: every other hex is deep sea, 0 percent.
    /// The map has one zero-percent coast hex.
    #[test]
    fn a_synthetic_map_is_counted_by_hand() {
        let a = Polygon {
            rings: vec![
                rect_nm((1.0, 15.0), (-9.0, 9.0)),
                rect_nm((3.0, 9.0), (-3.0, 3.0)),
            ],
        };
        let b = Polygon {
            rings: vec![rect_nm((119.0, 121.0), (-1.0, 1.0))],
        };
        let map = generate(vec![a, b]);
        assert_eq!(map.hexes.len(), extent_hexes().len());
        assert_eq!(map.zero_percent_coast, 1);
        for hex in &map.hexes {
            match (hex.q, hex.r) {
                (0, 0) => {
                    assert_eq!((hex.points, hex.land_points), (344, 54));
                    assert_eq!((hex.land_percent, hex.kind), (15, Kind::Coast));
                    // The hole is sea shut in by land: a second body of its 9
                    // points. The main body is the rest: 344 - 54 - 9 = 281.
                    assert_eq!(hex.water_bodies, 2);
                    assert_eq!(hex.main_body_points, 281);
                }
                (3, 0) => {
                    assert_eq!(hex.land_points, 1);
                    assert_eq!((hex.land_percent, hex.kind), (0, Kind::Coast));
                }
                _ => {
                    assert_eq!(hex.land_points, 0, "hex ({},{})", hex.q, hex.r);
                    assert_eq!((hex.land_percent, hex.kind), (0, Kind::DeepSea));
                    assert_eq!(hex.water_bodies, 1);
                }
            }
        }
    }

    #[test]
    fn land_everywhere_makes_every_hex_land_with_no_water() {
        let all = Polygon {
            rings: vec![rect_nm((-3000.0, 3000.0), (-3000.0, 3000.0))],
        };
        let map = generate(vec![all]);
        assert!(map.hexes.iter().all(|h| h.kind == Kind::Land
            && h.land_percent == 100
            && h.land_points == h.points
            && h.water_bodies == 0
            && h.main_body_points == 0
            && h.closed_sides == [true; 6]));
        assert_eq!(map.zero_percent_coast, 0);
        assert_eq!(map.multi_body_hexes(), 0);
        assert_eq!(map.closed_sides_between_non_land(), 0);
    }

    #[test]
    fn open_sea_everywhere_is_open_inside_and_closed_at_the_extent_edge() {
        let map = generate(Vec::new());
        let extent: BTreeSet<(i32, i32)> = extent_hexes().into_iter().collect();
        for hex in &map.hexes {
            assert_eq!(hex.water_bodies, 1);
            assert_eq!(hex.main_body_points, hex.points);
            for (d, (dq, dr)) in Hex::DIRECTIONS.iter().enumerate() {
                let inside = extent.contains(&(hex.q + dq, hex.r + dr));
                assert_eq!(
                    hex.closed_sides[d], !inside,
                    "({},{}) side {d}",
                    hex.q, hex.r
                );
            }
        }
        // Hexes at the edge of the extent really have closed sides.
        assert!(map.hexes.iter().any(|h| h.closed_sides != [false; 6]));
    }

    #[test]
    fn polygons_far_from_the_extent_are_dropped() {
        let far = Polygon {
            rings: vec![Ring {
                points: vec![(10.0, 50.0), (11.0, 50.0), (11.0, 51.0), (10.0, 50.0)],
            }],
        };
        let near = Polygon {
            rings: vec![Ring {
                points: vec![
                    (172.5, -10.0),
                    (173.0, -10.0),
                    (173.0, -9.0),
                    (172.5, -10.0),
                ],
            }],
        };
        // 172.5 is within 40 / 58.68888 = 0.68 degrees of 172.
        assert_eq!(keep_near_extent(vec![far, near]).len(), 1);
    }

    #[test]
    fn extent_hexes_are_sorted_and_inside() {
        let hexes = extent_hexes();
        assert!(hexes
            .windows(2)
            .all(|w| (w[0].1, w[0].0) < (w[1].1, w[1].0)));
        for (q, r) in hexes {
            let (lat, lon) = centre_of(q, r);
            assert!((-24.0..=-1.0).contains(&lat) && (145.0..=172.0).contains(&lon));
        }
    }

    // ----- Bodies of water ------------------------------------------------

    /// An island strip across hex (0,0): the lattice column i = 0 (east 0)
    /// is land, all the way along. Hex (0,0) has 23 points in that column
    /// (from the hand count above). The sea on the west holds the 11 points
    /// of the west side (dx = -20) and columns dx = -18 to -2: 11 + 155 =
    /// 166. The sea on the east holds columns dx = 2 to 18: 155. Two bodies;
    /// the larger, the west, is the main body.
    #[test]
    fn an_island_strip_across_a_hex_gives_two_bodies_and_the_larger_is_main() {
        let analysis = analyse_hex(0, 0, &|i, _| i == 0);
        assert_eq!(analysis.points.len(), 344);
        assert_eq!(analysis.land_points, 23);
        assert_eq!(analysis.bodies.len(), 2);
        let mut sizes: Vec<u32> = analysis.bodies.iter().map(|b| b.size).collect();
        sizes.sort_unstable();
        assert_eq!(sizes, vec![155, 166]);
        let main = analysis.main.expect("there is sea");
        assert_eq!(analysis.bodies[main].size, 166);
        assert!(analysis.in_main((-5, 0)), "west of the strip");
        assert!(!analysis.in_main((5, 0)), "east of the strip");
        assert!(!analysis.in_main((0, 0)), "land is in no body");
        // The main body's top point: the northernmost row j = 11 holds
        // east = -2 to ... : north 22 is column dx = 0 only, which is land;
        // the west body's top is j = 10 (north 20), in column dx = -2 to
        // -4: the westernmost of the top row decides the top point.
        let (top_i, top_j) = analysis.bodies[main].top;
        assert!(top_i < 0 && top_j > 0);
    }

    /// Sea only on named points; every other point of hex (0,0) is land.
    fn sea_on(points: &[(i32, i32)]) -> HexAnalysis {
        let sea: BTreeSet<(i32, i32)> = points.iter().copied().collect();
        analyse_hex(0, 0, &|i, j| !sea.contains(&(i, j)))
    }

    #[test]
    fn equal_bodies_tie_to_the_one_with_the_northernmost_point() {
        // Two pairs of equal size. The north pair is at j = 5, the south
        // pair at j = -5.
        let north = [(-5, 5), (-4, 5)];
        let south = [(5, -5), (4, -5)];
        let both: Vec<(i32, i32)> = north.iter().chain(south.iter()).copied().collect();
        let analysis = sea_on(&both);
        assert_eq!(analysis.bodies.len(), 2);
        assert!(analysis.bodies.iter().all(|b| b.size == 2));
        assert!(analysis.in_main((-5, 5)) && analysis.in_main((-4, 5)));
        assert!(!analysis.in_main((5, -5)));
        // The order the points are listed in does not matter.
        let reversed: Vec<(i32, i32)> = south.iter().chain(north.iter()).copied().collect();
        assert!(sea_on(&reversed).in_main((-5, 5)));
        // A body is judged by its own northernmost point, not its size in
        // the north: this pair stands north-south and reaches j = 7, beating
        // the pair at j = 5 although half its points are further south.
        let tall = [(5, 6), (5, 7)];
        let both: Vec<(i32, i32)> = north.iter().chain(tall.iter()).copied().collect();
        let analysis = sea_on(&both);
        assert!(analysis.in_main((5, 7)) && !analysis.in_main((-5, 5)));
    }

    #[test]
    fn equal_bodies_level_in_the_north_tie_to_the_westernmost() {
        // Both pairs have their northernmost point at j = 3. The west pair
        // has it at i = -5 and the east pair at i = 4: -5 is further west.
        let west = [(-5, 3), (-4, 3)];
        let east = [(4, 3), (5, 3)];
        let both: Vec<(i32, i32)> = west.iter().chain(east.iter()).copied().collect();
        let analysis = sea_on(&both);
        assert_eq!(analysis.bodies.len(), 2);
        assert!(analysis.in_main((-5, 3)) && !analysis.in_main((4, 3)));
        let reversed: Vec<(i32, i32)> = east.iter().chain(west.iter()).copied().collect();
        assert!(sea_on(&reversed).in_main((-5, 3)));
    }

    #[test]
    fn a_bigger_body_beats_a_more_northerly_smaller_one() {
        // Three points at j = -2 against one at j = 9.
        let analysis = sea_on(&[(1, -2), (2, -2), (3, -2), (-6, 9)]);
        assert!(analysis.in_main((1, -2)));
        assert!(!analysis.in_main((-6, 9)));
    }

    #[test]
    fn diagonal_points_are_not_joined_and_a_hex_with_no_sea_has_no_main_body() {
        let analysis = sea_on(&[(0, 0), (1, 1)]);
        assert_eq!(analysis.bodies.len(), 2);
        let none = analyse_hex(0, 0, &|_, _| true);
        assert_eq!(none.bodies.len(), 0);
        assert_eq!(none.main, None);
        assert!(!none.in_main((0, 0)));
    }

    #[test]
    fn bodies_do_not_join_through_a_neighbouring_hex() {
        // Sea in hex (0,0) at both sides of its east vertical side's
        // neighbours would be joined only through hex (1,0). Use two
        // points of (0,0) that touch only across hex (1,0)'s points: (9,0)
        // and (9,1) are in (0,0) and adjacent to each other, so one body;
        // (9,0) and (11,0) are separated by (10,0), which is in (1,0).
        let analysis = sea_on(&[(9, 0), (11, 0), (10, 0)]);
        assert_eq!(lattice_hex(10, 0), (1, 0));
        assert_eq!(analysis.body_of.len(), 1, "(11,0) is not in hex (0,0)");
    }

    // ----- Closed sides -----------------------------------------------------

    /// A wall of land across the whole slice, east to west along north = 0
    /// (the row of hexes r = 0), 6 nm thick: north in (-3, 3), which catches
    /// the lattice rows j = -1, 0, 1 (north = -2, 0, 2). Each hex (q, 0) has
    /// centre north 0, so its points split evenly: j = 2..11 on the north
    /// (10 rows) and j = -11..-2 on the south, with the west side column
    /// split 5 and 5. The two bodies are the same size, so the north one is
    /// main (it holds the point furthest north). So, for hexes (q, 0):
    ///
    /// - directions 1 (1,-1) and 2 (0,-1), to row -1, are open: the north
    ///   half touches the sea of row -1;
    /// - directions 4 (-1,1) and 5 (0,1), to row 1, are closed: the hex's
    ///   main body is the north half, and the south half is not main;
    /// - directions 0 and 3, to (q+1,0) and (q-1,0), are open: north halves
    ///   touch.
    ///
    /// Row 1 hexes are all sea: (q,1) has direction 1 (to (q+1,0)) and
    /// direction 2 (to (q,0)) closed.
    ///
    /// The gap: the wall is in two parts, east in (-1501, 101) and (111,
    /// 1501), leaving the lattice columns east = 102 to 110 (5 columns, a
    /// gap of 10 nm) open. They lie in hex (3,0) (dx = -18 to -10: hex (3,0)
    /// is centred at east 120). There the north and south halves join, so
    /// (3,0) has one body, the main body spans both halves, and its sides
    /// to row 1, direction 4 (to (2,1)) and 5 (to (3,1)), are open. Its
    /// neighbours' main bodies are unchanged. Open between row 0 and row 1:
    /// (3,0)-(2,1) and (3,0)-(3,1), nothing else.
    fn wall(with_gap: bool) -> GeneratedMap {
        let mut rings = Vec::new();
        if with_gap {
            rings.push(rect_nm((-1501.0, 101.0), (-3.0, 3.0)));
            rings.push(rect_nm((111.0, 1501.0), (-3.0, 3.0)));
        } else {
            rings.push(rect_nm((-1501.0, 1501.0), (-3.0, 3.0)));
        }
        generate(
            rings
                .into_iter()
                .map(|ring| Polygon { rings: vec![ring] })
                .collect(),
        )
    }

    #[test]
    fn a_wall_with_a_gap_closes_every_side_across_it_except_at_the_gap() {
        let generated = wall(true);
        let columns = -10..=10;
        for q in columns.clone() {
            let hex = find(&generated, q, 0);
            assert_eq!(hex.kind, Kind::Coast, "({q},0) is coast");
            let wanted = if q == 3 {
                // The gap joins the halves: one body, nothing closed.
                assert_eq!(hex.water_bodies, 1);
                [false; 6]
            } else {
                assert_eq!(hex.water_bodies, 2, "({q},0)");
                [false, false, false, false, true, true]
            };
            assert_eq!(hex.closed_sides, wanted, "({q},0)");
        }
        // Hexes of rows -1 and 1 are all sea, one body, and closed only
        // across the wall: (q,1) has directions 1 and 2 closed, except the
        // two at the gap.
        for q in columns {
            let north = find(&generated, q, -1);
            assert_eq!((north.kind, north.water_bodies), (Kind::DeepSea, 1));
            assert_eq!(north.closed_sides, [false; 6], "({q},-1)");
            let south = find(&generated, q, 1);
            assert_eq!((south.kind, south.water_bodies), (Kind::DeepSea, 1));
            let wanted = match q {
                2 => [false, false, true, false, false, false],
                3 => [false, true, false, false, false, false],
                _ => [false, true, true, false, false, false],
            };
            assert_eq!(south.closed_sides, wanted, "({q},1)");
        }
    }

    #[test]
    fn the_wall_is_coast_with_the_hand_counted_land_points() {
        // Each hex (q,0) has, on each of the rows j = -1, 0, 1, the points
        // i = -10 to 9 relative to its centre (dx = -20 to 18; the east side
        // is the next hex's): 20 points a row, 60 land points of 344, 17
        // percent. Every such point is well inside the hexagon (|dy| <= 2).
        // Hex (3,0) loses the 5 gap columns east = 102 to 110 on each of the
        // 3 rows: 60 - 15 = 45 land points, 13 percent.
        let generated = wall(true);
        for q in -10..=10 {
            let hex = find(&generated, q, 0);
            if q == 3 {
                assert_eq!((hex.land_points, hex.land_percent), (45, 13));
            } else {
                assert_eq!((hex.land_points, hex.land_percent), (60, 17), "({q},0)");
            }
        }
        // The only coast hexes are the row of the wall.
        let coast: Vec<(i32, i32)> = generated
            .hexes
            .iter()
            .filter(|h| h.kind != Kind::DeepSea)
            .map(|h| (h.q, h.r))
            .collect();
        assert!(coast.iter().all(|c| c.1 == 0));
        assert!(coast.len() > 30);
    }

    #[test]
    fn a_path_from_one_side_of_the_wall_to_the_other_goes_through_the_gap() {
        let generated = wall(true);
        let map = to_map(&generated);
        // The only open steps between row 0 and row 1 are at the gap.
        let mut open = Vec::new();
        for q in -10..=10 {
            for direction in [4, 5] {
                if map.can_step(Hex::new(q, 0), direction) {
                    open.push((q, direction));
                }
            }
        }
        assert_eq!(open, vec![(3, 4), (3, 5)]);
        // (0,1) to (0,-1): 2 in a straight line. By sea: to the gap hex
        // (3,0) is 3 steps ((0,1) -> (1,1) -> (2,1) -> (3,0)), and (3,0) to
        // (0,-1) is 4 ((3,0) is 4 hexes from (0,-1): dq -3, dr -1, (3 + 1 +
        // 4) / 2), all over open sea or open sides: 7 in all.
        let (start, target) = (Hex::new(0, 1), Hex::new(0, -1));
        assert_eq!(start.distance(target), 2);
        assert_eq!(map.sea_distance(start, target), Some(7));
        assert_eq!(map.sea_distance(target, start), Some(7));
        let distances = map.sea_distances_to(target);
        let mut here = start;
        let mut path = vec![here];
        while let Some(next) = map.next_step_in(&distances, here) {
            path.push(next);
            here = next;
        }
        assert_eq!(path.len(), 8);
        assert!(path.contains(&Hex::new(3, 0)));
        // Every step of the path is open, and only one of them crosses
        // between rows 1 and 0, at the gap.
        let crossings = path
            .windows(2)
            .filter(|w| w[0].r == 1 && w[1].r == 0)
            .count();
        assert_eq!(crossings, 1);
        // Closed sides are symmetric over the whole map, as `can_step` sees
        // them.
        for hex in &generated.hexes {
            let here = Hex::new(hex.q, hex.r);
            for (direction, neighbor) in here.neighbors().into_iter().enumerate() {
                assert_eq!(
                    map.can_step(here, direction),
                    map.can_step(neighbor, (direction + 3) % 6),
                    "{here} direction {direction}"
                );
            }
        }
    }

    #[test]
    fn a_wall_with_no_gap_cuts_the_slice_in_two() {
        let generated = wall(false);
        let map = to_map(&generated);
        for q in -10..=10 {
            // Every hex of the wall's row has two bodies; the tie goes to the
            // north one, so the sides to row 1 are all closed.
            assert_eq!(
                find(&generated, q, 0).closed_sides,
                [false, false, false, false, true, true],
                "({q},0)"
            );
        }
        assert_eq!(map.sea_distance(Hex::new(0, 1), Hex::new(0, -1)), None);
        assert_eq!(map.sea_distance(Hex::new(0, -1), Hex::new(0, 1)), None);
        // Each side still reaches itself.
        let (a, b) = (Hex::new(0, 1), Hex::new(5, 3));
        assert_eq!(map.sea_distance(a, b), Some(a.distance(b)));
    }

    #[test]
    fn the_header_counts_closed_sides_once_per_pair() {
        // Each closed side between non-land hexes is seen from both hexes, so
        // counting every hex's closed sides gives twice the pairs.
        let generated = wall(true);
        let pairs = generated.closed_sides_between_non_land();
        let by_hex: BTreeMap<(i32, i32), &MapHex> =
            generated.hexes.iter().map(|h| ((h.q, h.r), h)).collect();
        let mut both_ways = 0;
        for hex in &generated.hexes {
            for (d, (dq, dr)) in Hex::DIRECTIONS.iter().enumerate() {
                if let Some(other) = by_hex.get(&(hex.q + dq, hex.r + dr)) {
                    if hex.closed_sides[d] && hex.kind != Kind::Land && other.kind != Kind::Land {
                        both_ways += 1;
                    }
                }
            }
        }
        assert_eq!(both_ways, 2 * pairs);
        assert!(pairs > 0);
    }

    #[test]
    fn closed_sides_text_lists_indexes_ascending() {
        assert_eq!(closed_sides_text(&[false; 6]), "");
        assert_eq!(
            closed_sides_text(&[true, false, false, false, true, true]),
            "0|4|5"
        );
        assert_eq!(closed_sides_text(&[true; 6]), "0|1|2|3|4|5");
    }

    // ----- Anchorages ---------------------------------------------------------

    fn anchor_at(place: &str, east: f64, north: f64) -> Anchorage {
        let (lat, lon) = from_nm(east, north);
        Anchorage {
            place: place.to_string(),
            feature: "test anchorage".to_string(),
            lat,
            lon,
        }
    }

    /// A north-south strip of land across hex (0,0), the lattice column
    /// east = 0 (see `an_island_strip_across_a_hex_gives_two_bodies_and_the_larger_is_main`):
    /// a west body of 166 points, the main body, and an east body of 155.
    fn strip() -> Vec<Polygon> {
        vec![Polygon {
            rings: vec![rect_nm((-1.0, 1.0), (-200.0, 200.0))],
        }]
    }

    #[test]
    fn an_anchorage_on_the_smaller_side_of_a_strip_moves_the_main_body_and_the_closed_sides() {
        // Without anchorages the west body (166) is main. Hex (1,0) is
        // across the east side (direction 0) and hex (-1,0) across the west
        // side (direction 3); (-1,0) is plain sea, joined to the west body
        // along the hex's west side. So direction 0 is closed, 3 is open.
        let plain = generate(strip());
        let hex = find(&plain, 0, 0);
        assert_eq!((hex.water_bodies, hex.main_body_points), (2, 166));
        assert!(hex.closed_sides[0] && !hex.closed_sides[3]);

        // The anchorage is 10 nm east of the strip: its nearest sea point is
        // (5,0), in the east body of 155 points, which becomes main. Now
        // direction 0 is open (the east body touches hex (1,0)) and
        // direction 3 is closed.
        let anchors = [anchor_at("east-side", 10.0, 0.0)];
        let moved = generate_with_anchors(strip(), &anchors).unwrap();
        let hex = find(&moved, 0, 0);
        assert_eq!((hex.water_bodies, hex.main_body_points), (2, 155));
        assert!(!hex.closed_sides[0] && hex.closed_sides[3]);
        assert_eq!(
            moved.anchors,
            vec![AnchorNote {
                place: "east-side".to_string(),
                hex: (0, 0),
                outcome: AnchorOutcome::Changed {
                    from_points: 166,
                    to_points: 155
                },
            }]
        );
        // The neighbours agree, and a ship can now step east but not west.
        let map = to_map(&moved);
        assert!(map.can_step(Hex::new(0, 0), 0));
        assert!(!map.can_step(Hex::new(0, 0), 3));

        // An anchorage on the side that is already main changes nothing.
        let west = [anchor_at("west-side", -10.0, 0.0)];
        let same = generate_with_anchors(strip(), &west).unwrap();
        assert_eq!(same.hexes, plain.hexes);
        assert_eq!(
            same.anchors[0].outcome,
            AnchorOutcome::Unchanged { points: 166 }
        );
    }

    /// Land over all of hex (0,0) with holes of sea: a main hole east in
    /// (-17, -1), north in (-11, 11), which is the lattice columns east =
    /// -16 to -2 (8) and rows north = -10 to 10 (11): 88 points; and, if
    /// asked for, a hole of 4 points (east 4 and 6, north 4 and 6) and one of
    /// 5 (east 4, 6, 8, 10, 12, north 0). The hex has 344 points, at most 97
    /// of them sea: coast, not land.
    fn pockets(four: bool, five: bool) -> Vec<Polygon> {
        let mut rings = vec![
            rect_nm((-30.0, 30.0), (-30.0, 30.0)),
            rect_nm((-17.0, -1.0), (-11.0, 11.0)),
        ];
        if four {
            rings.push(rect_nm((3.0, 7.0), (3.0, 7.0)));
        }
        if five {
            rings.push(rect_nm((3.0, 13.0), (-1.0, 1.0)));
        }
        vec![Polygon { rings }]
    }

    #[test]
    fn a_body_of_four_points_is_ignored_and_a_body_of_five_is_used() {
        let four =
            generate_with_anchors(pockets(true, false), &[anchor_at("four", 5.0, 5.0)]).unwrap();
        let hex = find(&four, 0, 0);
        assert_eq!(hex.kind, Kind::Coast);
        assert_eq!((hex.water_bodies, hex.main_body_points), (2, 88));
        assert_eq!(
            four.anchors[0].outcome,
            AnchorOutcome::Ignored {
                points: 4,
                main_points: 88
            }
        );
        assert!(four.anchors[0].text().contains("ignored"));

        let five =
            generate_with_anchors(pockets(false, true), &[anchor_at("five", 8.0, 0.0)]).unwrap();
        let hex = find(&five, 0, 0);
        assert_eq!((hex.water_bodies, hex.main_body_points), (2, 5));
        assert_eq!(
            five.anchors[0].outcome,
            AnchorOutcome::Changed {
                from_points: 88,
                to_points: 5
            }
        );
        assert_eq!(
            five.anchors[0].text(),
            "  five: hex (0,0): changed the main body of water, from 88 points to 5"
        );
    }

    #[test]
    fn two_anchorages_on_one_body_agree_and_on_different_bodies_conflict() {
        let both = [anchor_at("one", -8.0, 0.0), anchor_at("two", -6.0, 4.0)];
        let map = generate_with_anchors(pockets(false, true), &both).unwrap();
        assert!(map
            .anchors
            .iter()
            .all(|n| n.outcome == AnchorOutcome::Unchanged { points: 88 }));

        let apart = [anchor_at("big", -8.0, 0.0), anchor_at("small", 8.0, 0.0)];
        let error = generate_with_anchors(pockets(false, true), &apart).unwrap_err();
        assert!(error.contains("big") && error.contains("small"), "{error}");
        assert!(error.contains("different bodies"), "{error}");
        // The order in the file does not matter.
        let swapped = [apart[1].clone(), apart[0].clone()];
        assert!(generate_with_anchors(pockets(false, true), &swapped).is_err());

        // An ignored anchorage picks nothing, so it conflicts with nobody.
        let quiet = [anchor_at("big", -8.0, 0.0), anchor_at("tiny", 5.0, 5.0)];
        assert!(generate_with_anchors(pockets(true, false), &quiet).is_ok());
    }

    #[test]
    fn an_anchorage_outside_the_extent_on_land_or_far_from_sea_is_an_error() {
        // Outside the extent: 0 degrees north is beyond the northern edge.
        let outside = Anchorage {
            place: "nowhere".to_string(),
            feature: "off the map".to_string(),
            lat: 0.0,
            lon: 160.0,
        };
        let error = generate_with_anchors(Vec::new(), &[outside]).unwrap_err();
        assert!(
            error.contains("nowhere") && error.contains("outside the extent"),
            "{error}"
        );

        // In a land hex.
        let all = vec![Polygon {
            rings: vec![rect_nm((-3000.0, 3000.0), (-3000.0, 3000.0))],
        }];
        let error = generate_with_anchors(all, &[anchor_at("inland", 0.0, 0.0)]).unwrap_err();
        assert!(
            error.contains("inland") && error.contains("land hex"),
            "{error}"
        );

        // In a coast hex, on land, more than 6 nm from every sea point.
        let error =
            generate_with_anchors(pockets(true, false), &[anchor_at("stranded", 10.0, -15.0)])
                .unwrap_err();
        assert!(
            error.contains("stranded") && error.contains("within 6 nm"),
            "{error}"
        );

        // Sea 4 nm away is found.
        let near = anchor_at("near", -17.0 - 3.0, 0.0);
        assert!(generate_with_anchors(pockets(false, false), &[near]).is_ok());
    }

    #[test]
    fn without_anchors_the_output_is_what_it_was_and_with_them_the_header_gains_lines() {
        let command = "generate --x y";
        let plain = generate(pockets(true, false));
        let text = csv_text(&plain, command, None);
        assert!(!text.to_lowercase().contains("anchor"));
        assert_eq!(
            text,
            csv_text(
                &generate_with_anchors(pockets(true, false), &[]).unwrap(),
                command,
                None
            )
        );

        let anchored = generate_with_anchors(
            pockets(true, false),
            &[anchor_at("four", 5.0, 5.0), anchor_at("main", -8.0, 0.0)],
        )
        .unwrap();
        let with = csv_text(&anchored, command, Some("data/map/anchors.csv"));
        let header: Vec<&str> = with.lines().filter(|l| l.starts_with('#')).collect();
        let at = header
            .iter()
            .position(|l| l.starts_with("# Anchors: data/map/anchors.csv"))
            .expect("the anchors line");
        assert_eq!(
            &header[at + 1..at + 3],
            [
                "#   four: hex (0,0): ignored: its body of water has 4 points, fewer than 5; the main body stays at 88 points",
                "#   main: hex (0,0): its body of water (88 points) was already the main body, no change",
            ]
        );
        assert!(header[at + 3].starts_with("# closed_sides:"));
        // Here the anchorages change nothing, so the rows are the same.
        let rows = |t: &str| {
            t.lines()
                .filter(|l| !l.starts_with('#'))
                .collect::<Vec<_>>()
                .join("\n")
        };
        assert_eq!(rows(&with), rows(&text));
    }

    #[test]
    fn the_anchors_file_is_read_with_comments_and_quoted_fields() {
        let text = "# a comment, with, commas\n\
            place,feature,lat,lon,source,locator,accessed\n\
            \n\
            rabaul,\"Simpson Harbour, the anchorage\",-4.22224,152.16544,geonames,https://x/1,2026-11-10\n\
            # another\n\
            buka,Buka Passage,-5.44605,154.65681,geonames,https://x/2,2026-11-10\n";
        let anchors = parse_anchors(text).unwrap();
        assert_eq!(anchors.len(), 2);
        assert_eq!(anchors[0].place, "rabaul");
        assert_eq!(anchors[0].feature, "Simpson Harbour, the anchorage");
        assert_eq!((anchors[0].lat, anchors[0].lon), (-4.22224, 152.16544));
        assert_eq!(anchors[1].place, "buka");

        // A header that is not exactly right, a short row, a bad number, a
        // repeated place and an empty file are errors.
        let bad = [
            "place,feature,lat,lon,source,locator\nrabaul,x,-4,152,s,l\n",
            "place,feature,lat,lon,source,locator,accessed\nrabaul,x,-4,152,s\n",
            "place,feature,lat,lon,source,locator,accessed\nrabaul,x,north,152,s,l,d\n",
            "place,feature,lat,lon,source,locator,accessed\nrabaul,x,-4,152,s,l,d\nrabaul,y,-5,153,s,l,d\n",
            "# only comments\n",
        ];
        for text in bad {
            assert!(parse_anchors(text).is_err(), "{text}");
        }
    }
}
