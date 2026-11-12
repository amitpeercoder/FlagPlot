//! Rule 5: recorded passages (`passages.csv`).
//!
//! The ids, the source and the other source columns are checked by the
//! general rules. This module checks what is particular to a passage: the
//! columns that may not be blank, the forms of coordinates, times and
//! speeds, the list columns, the places, and that each end of a passage is
//! located or says it is not.

use std::collections::BTreeSet;

use crate::table::{Data, Rec};
use crate::value::{is_decimal, is_time, looks_like_exponent_float};
use crate::Problem;

const FILE: &str = "passages.csv";

/// The columns of the file, in order.
pub const COLUMNS: [&str; 25] = [
    "id",
    "subject",
    "ships",
    "from_place",
    "from_lat",
    "from_lon",
    "depart_time",
    "to_place",
    "to_lat",
    "to_lon",
    "arrive_time",
    "zone",
    "speed_kn",
    "speed_note",
    "distance_nm_stated",
    "steady",
    "flags",
    "as_stated",
    "source",
    "locator",
    "accessed",
    "confidence",
    "basis",
    "standing",
    "note",
];

/// The columns before the source columns, which `schema.rs` requires by name.
pub const OWN_COLUMNS: usize = 18;

/// Columns that may never be blank. `id` and the source columns are held by
/// the general rules.
const NEVER_BLANK: [&str; 4] = ["subject", "ships", "steady", "as_stated"];

pub fn check(data: &Data, out: &mut Vec<Problem>) {
    let Some(table) = data.tables.get(FILE) else {
        return;
    };
    if table.fields != COLUMNS {
        out.push(Problem {
            file: FILE.to_string(),
            line: 0,
            row: String::new(),
            message: format!("columns must be, in order: {}", COLUMNS.join(",")),
        });
    }
    let places = data.keys("places.csv");
    for row in data.rows(FILE) {
        for column in NEVER_BLANK {
            if row.get(column).trim().is_empty() {
                out.push(row.problem(format!("{column} is blank")));
            }
        }
        let steady = row.get("steady");
        if !steady.is_empty() && !matches!(steady, "yes" | "no" | "unknown") {
            out.push(row.problem(format!("steady `{steady}` is not yes, no or unknown")));
        }
        let ships = row.get("ships");
        if !ships.is_empty() && ships.split('|').any(|s| s.trim().is_empty()) {
            out.push(row.problem("ships has an empty member"));
        }
        let flags: Vec<&str> = if row.get("flags").is_empty() {
            Vec::new()
        } else {
            row.get("flags").split('|').collect()
        };
        // The flag list is open (`passages.md`: "and others in plain words"),
        // so only an empty member is a problem.
        if flags.iter().any(|f| f.trim().is_empty()) {
            out.push(row.problem("flags has an empty member"));
        }
        for end in ["from", "to"] {
            located(row, end, &flags, &places, out);
        }
        for column in ["depart_time", "arrive_time"] {
            let time = row.get(column);
            if !time.is_empty() && !is_time(time) {
                out.push(row.problem(format!(
                    "{column} `{time}` is not a date or YYYY-MM-DDTHH:MM"
                )));
            }
        }
        for column in ["speed_kn", "distance_nm_stated"] {
            let value = row.get(column);
            if looks_like_exponent_float(value.trim()) {
                out.push(row.problem(format!(
                    "{column} `{value}` looks like a float with an exponent"
                )));
            } else if !value.is_empty() && !above_zero(value) {
                out.push(row.problem(format!("{column} `{value}` is not a number above zero")));
            }
        }
    }
}

/// A decimal that is not negative and not zero.
fn above_zero(value: &str) -> bool {
    is_decimal(value) && !value.starts_with('-') && value.bytes().any(|b| matches!(b, b'1'..=b'9'))
}

/// One end of a passage: a place, or a latitude and longitude, or the flag
/// `<end>-not-located`.
fn located(row: Rec, end: &str, flags: &[&str], places: &BTreeSet<&str>, out: &mut Vec<Problem>) {
    let place = row.get(&format!("{end}_place"));
    let (lat_column, lon_column) = (format!("{end}_lat"), format!("{end}_lon"));
    let (lat, lon) = (row.get(&lat_column), row.get(&lon_column));
    if !place.is_empty() && !places.contains(place) {
        out.push(row.problem(format!("unknown {end}_place `{place}`")));
    }
    if lat.is_empty() != lon.is_empty() {
        out.push(row.problem(format!(
            "{lat_column} and {lon_column} come in pairs: one is blank"
        )));
    }
    for (column, value) in [(&lat_column, lat), (&lon_column, lon)] {
        if !value.is_empty() && !is_decimal(value) {
            out.push(row.problem(format!("{column} `{value}` is not a decimal")));
        }
    }
    let flag = format!("{end}-not-located");
    if place.is_empty() && lat.is_empty() && lon.is_empty() && !flags.contains(&flag.as_str()) {
        out.push(row.problem(format!(
            "the {end} end is not located: give {end}_place or {end}_lat and {end}_lon, or the flag `{flag}`"
        )));
    }
}
