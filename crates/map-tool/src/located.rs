//! The file of located positions of a map (decision 0242, rule 15): for each
//! row of `data/oob/place_positions.csv` whose position is inside the map's
//! extent, the hex whose centre is nearest by the map's own projection.
//!
//! The map tool may read `data/oob/place_positions.csv` for this command and
//! nothing else under `data/oob`, as the check of passages does. The text of
//! the file is given to [`located_text`]; the command line reads it.
//!
//! It works for a map of either kind. A row's `lat` and `lon` are written as
//! the row gives them, the same text, and the hex is worked out with floating
//! point here, once, so that nothing that plays a game does it.

use std::collections::BTreeSet;

use crate::frame::Frame;
use crate::generate::extent_hexes;

/// The columns of a located file, after its comment header.
pub const LOCATED_HEADER: &str = "row,place,lat,lon,q,r";

/// One line of a located file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Located {
    /// The row's id, as the positions file gives it.
    pub row: String,
    pub place: String,
    /// The row's latitude and longitude, the same text.
    pub lat: String,
    pub lon: String,
    pub q: i32,
    pub r: i32,
}

/// A row of the positions file whose position is inside a map's extent.
#[derive(Clone, Debug, PartialEq)]
pub struct RowInside {
    /// The line of the positions file the row is on.
    pub line: u64,
    /// The row's id as a number, and as the file gives it.
    pub number: u32,
    pub row: String,
    pub place: String,
    /// The row's latitude and longitude, the same text, and their values; the
    /// longitude value is in the frame's run.
    pub lat_text: String,
    pub lon_text: String,
    pub lat: f64,
    pub lon_run: f64,
}

/// Every row of a positions file whose position is inside the frame's extent,
/// in the order of the rows' ids. A row outside the extent is left out.
pub fn rows_inside(frame: &Frame, positions: &str) -> Result<Vec<RowInside>, String> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .comment(Some(b'#'))
        .from_reader(positions.as_bytes());
    let mut records = reader.records();
    let header = records
        .next()
        .ok_or("the positions file has no header line")?
        .map_err(|e| e.to_string())?;
    let column = |name: &str| {
        header
            .iter()
            .position(|field| field == name)
            .ok_or_else(|| format!("the positions file has no column \"{name}\""))
    };
    let (id, place, lat, lon) = (
        column("id")?,
        column("place")?,
        column("lat")?,
        column("lon")?,
    );
    let mut found: Vec<RowInside> = Vec::new();
    for record in records {
        let record = record.map_err(|e| e.to_string())?;
        let line = record.position().map_or(0, |p| p.line());
        let field = |index: usize| {
            record
                .get(index)
                .ok_or_else(|| format!("positions file line {line}: too few fields"))
        };
        let (row, name, lat_text, lon_text) = (field(id)?, field(place)?, field(lat)?, field(lon)?);
        let number = row
            .parse::<u32>()
            .map_err(|_| format!("positions file line {line}: id \"{row}\" is not a number"))?;
        if name.is_empty() || name.contains([',', '"', '\n']) {
            return Err(format!(
                "positions file line {line}: place \"{name}\" cannot be written in a located file"
            ));
        }
        let degrees = |what: &str, text: &str, limit: f64| match text.parse::<f64>() {
            Ok(v) if v.is_finite() && v.abs() <= limit => Ok(v),
            _ => Err(format!(
                "positions file line {line}: {what} \"{text}\" is not a number of degrees"
            )),
        };
        let (lat_value, lon_value) = (
            degrees("lat", lat_text, 90.0)?,
            degrees("lon", lon_text, 180.0)?,
        );
        let run = frame.lon_in_run(lon_value);
        let inside = lat_value >= frame.south.value
            && lat_value <= frame.north.value
            && run >= frame.west_run()
            && run <= frame.east_run();
        if !inside {
            continue;
        }
        found.push(RowInside {
            line,
            number,
            row: row.to_string(),
            place: name.to_string(),
            lat_text: lat_text.to_string(),
            lon_text: lon_text.to_string(),
            lat: lat_value,
            lon_run: run,
        });
    }
    found.sort_by_key(|row| row.number);
    if found.windows(2).any(|w| w[0].number == w[1].number) {
        return Err("the positions file has a row id twice".to_string());
    }
    Ok(found)
}

/// The located lines of a positions file on the map of `frame`: every row
/// whose position is inside the frame's extent, in the order of the rows'
/// ids. A row outside the extent has no line.
pub fn locate_rows(frame: &Frame, positions: &str) -> Result<Vec<Located>, String> {
    let projection = frame.projection();
    let on_the_map: BTreeSet<(i32, i32)> = extent_hexes(frame).into_iter().collect();
    let mut located = Vec::new();
    for found in rows_inside(frame, positions)? {
        let (q, r) = projection.hex_of(found.lat, found.lon_run);
        if !on_the_map.contains(&(q, r)) {
            return Err(format!(
                "positions file line {}: row {} is inside the extent but its nearest hex ({q},{r}) is not on the map",
                found.line, found.row
            ));
        }
        located.push(Located {
            row: found.row,
            place: found.place,
            lat: found.lat_text,
            lon: found.lon_text,
            q,
            r,
        });
    }
    Ok(located)
}

/// The text of a located file: its comment header, the column line and one
/// line for each located row.
pub fn located_text(frame: &Frame, positions: &str, command_line: &str) -> Result<String, String> {
    let rows = locate_rows(frame, positions)?;
    let mut text = String::new();
    text.push_str("# The positions of data/oob/place_positions.csv that fall inside a map, and the hex of the map each falls in (decision 0242, rule 15).\n");
    text.push_str("# One line for each row of that file whose position is inside the map's extent, in the order of the rows' ids; a row outside the extent has no line.\n");
    text.push_str("# lat and lon are the row's own text. (q, r) is the hex whose centre is nearest, by the projection in the map's header.\n");
    text.push_str("# Generated by the map tool: never edited by hand. A test checks that this file is what the command gives.\n");
    text.push_str(&format!(
        "# Generated by `cargo run --release -p map-tool -- {command_line}`\n"
    ));
    text.push_str(LOCATED_HEADER);
    text.push('\n');
    for row in rows {
        text.push_str(&format!(
            "{},{},{},{},{},{}\n",
            row.row, row.place, row.lat, row.lon, row.q, row.r
        ));
    }
    Ok(text)
}
