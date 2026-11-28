//! The shared file of checked positions (decision 0242, rule 18): what holds
//! the table's page, the viewer and the clipping script to this tool's
//! projection. `table/chart/plane_fixture.json` is written by the command
//! `plane-fixture` and never edited by hand; a test holds the committed file
//! to what the command gives (`tests/plane_fixture.rs`, which says what the
//! file must hold).
//!
//! One entry for each map. An entry names the map's file, gives its frame as
//! the table's bridge serves it, and lists points: for a map about a centre
//! every line of its file of located positions, and for a map of one factor
//! every row of the positions file inside its extent. A point is the row, the
//! latitude and longitude as the row gives them (text), the east and north of
//! the plane in nautical miles to three decimals, and the hex.
//!
//! The map tool is the one crate that may use floating point; the file's
//! numbers are its own.

use std::path::Path;

use crate::frame::{Frame, Kind};
use crate::located::rows_inside;
use crate::projection::hex_of_nm;

/// A map the fixture covers: its file's name, its frame, and for a map about a
/// centre the text of its file of located positions.
pub struct FixtureMap {
    pub file: String,
    pub frame: Frame,
    pub located: Option<String>,
}

/// The maps the fixture covers, in its order: the three generated maps.
pub const FIXTURE_MAPS: [&str; 3] = [
    "data/map/south_pacific.csv",
    "data/map/midway.csv",
    "data/map/papua_to_midway.csv",
];

/// Reads the three maps' frames from their headers, and the located file of a
/// map about a centre (the map's name with `_located` before `.csv`), from
/// the repository at `root`.
pub fn read_maps(root: &Path) -> Result<Vec<FixtureMap>, String> {
    let mut maps = Vec::new();
    for file in FIXTURE_MAPS {
        let path = root.join(file);
        let frame = Frame::of_map(&path)?
            .ok_or_else(|| format!("the map {file} has no frame in its header"))?;
        let located = if frame.is_about_a_centre() {
            let located_file = root.join(file.replace(".csv", "_located.csv"));
            Some(
                std::fs::read_to_string(&located_file)
                    .map_err(|e| format!("{}: {e}", located_file.display()))?,
            )
        } else {
            None
        };
        maps.push(FixtureMap {
            file: file.to_string(),
            frame,
            located,
        });
    }
    Ok(maps)
}

/// A number as the file writes it: three decimals, never "-0.000".
fn miles(value: f64) -> String {
    let text = format!("{value:.3}");
    if text == "-0.000" {
        "0.000".to_string()
    } else {
        text
    }
}

/// The frame as the table's bridge serves it: a frame of one factor has the
/// keys centre_lat, centre_lon, factor, west and east; a frame about a centre
/// has `kind` and `scale` in place of `factor`. Longitudes are in the run.
fn frame_json(frame: &Frame) -> String {
    let centre_lon = frame.projection().origin_lon;
    let (kind, number) = match &frame.kind {
        Kind::OneFactor(factor) => (String::new(), format!("\"factor\": {:?}", factor.value)),
        Kind::AboutCentre(scale) => (
            ", \"kind\": \"about a centre\"".to_string(),
            format!("\"scale\": {:?}", scale.value),
        ),
    };
    format!(
        "{{\"centre_lat\": {:?}, \"centre_lon\": {:?}{kind}, {number}, \"west\": {:?}, \"east\": {:?}}}",
        frame.centre_lat.value,
        centre_lon,
        frame.west_run(),
        frame.east_run()
    )
}

/// One point: the row, the text of its position, its east and north and its hex.
struct Point {
    row: String,
    lat: String,
    lon: String,
    east: f64,
    north: f64,
    q: i32,
    r: i32,
}

fn points(map: &FixtureMap, positions: &str) -> Result<Vec<Point>, String> {
    let projection = map.frame.projection();
    let mut out = Vec::new();
    match &map.located {
        Some(located) => {
            // A map about a centre: each line of the located file, at the hex it gives.
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(true)
                .comment(Some(b'#'))
                .from_reader(located.as_bytes());
            for record in reader.records() {
                let record = record.map_err(|e| format!("{}: {e}", map.file))?;
                let field = |i: usize| record.get(i).unwrap_or("").to_string();
                let (lat_text, lon_text) = (field(2), field(3));
                let lat: f64 = lat_text
                    .parse()
                    .map_err(|_| format!("{}: lat \"{lat_text}\" is not a number", map.file))?;
                let lon: f64 = lon_text
                    .parse()
                    .map_err(|_| format!("{}: lon \"{lon_text}\" is not a number", map.file))?;
                let (east, north) = projection.to_nm(lat, map.frame.lon_in_run(lon));
                let number = |i: usize| {
                    field(i).parse::<i32>().map_err(|_| {
                        format!("{}: \"{}\" is not a hex coordinate", map.file, field(i))
                    })
                };
                out.push(Point {
                    row: field(0),
                    lat: lat_text,
                    lon: lon_text,
                    east,
                    north,
                    q: number(4)?,
                    r: number(5)?,
                });
            }
        }
        None => {
            for row in rows_inside(&map.frame, positions)? {
                let (east, north) = projection.to_nm(row.lat, row.lon_run);
                let (q, r) = hex_of_nm(east, north);
                out.push(Point {
                    row: row.row,
                    lat: row.lat_text,
                    lon: row.lon_text,
                    east,
                    north,
                    q,
                    r,
                });
            }
        }
    }
    Ok(out)
}

/// The text of the fixture file for these maps and this positions file.
pub fn fixture_text(maps: &[FixtureMap], positions: &str) -> Result<String, String> {
    let mut text = String::new();
    text.push_str("{\n");
    text.push_str("  \"note\": \"Checked positions: what holds the table's page, the viewer and the clipping script to the map tool's projection (decision 0242, rule 18). Generated by `cargo run --release -p map-tool -- plane-fixture --out table/chart/plane_fixture.json`; never edited by hand: a test checks that this file is what the command gives. For each map: its file, its frame as the table is served it, and points. A point is a row of data/oob/place_positions.csv (row), its latitude and longitude as the row gives them (west negative), the east and north of the map's plane in nautical miles from the centre of hex (0,0) to three decimals, and the hex whose centre is nearest. For the map about a centre the points are the lines of its file of located positions; for a map of one factor, the rows inside its extent.\",\n");
    text.push_str("  \"maps\": [\n");
    for (index, map) in maps.iter().enumerate() {
        text.push_str("    {\n");
        text.push_str(&format!("      \"file\": \"{}\",\n", map.file));
        text.push_str(&format!("      \"frame\": {},\n", frame_json(&map.frame)));
        text.push_str("      \"points\": [\n");
        let list = points(map, positions)?;
        for (n, p) in list.iter().enumerate() {
            text.push_str(&format!(
                "        {{\"row\": \"{}\", \"lat\": \"{}\", \"lon\": \"{}\", \"east\": {}, \"north\": {}, \"q\": {}, \"r\": {}}}{}\n",
                p.row,
                p.lat,
                p.lon,
                miles(p.east),
                miles(p.north),
                p.q,
                p.r,
                if n + 1 < list.len() { "," } else { "" }
            ));
        }
        text.push_str("      ]\n");
        text.push_str(&format!(
            "    }}{}\n",
            if index + 1 < maps.len() { "," } else { "" }
        ));
    }
    text.push_str("  ]\n}\n");
    Ok(text)
}
