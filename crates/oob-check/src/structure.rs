//! Rule 1: ids and keys. (Field counts are checked as the files are read.)

use std::collections::BTreeSet;

use crate::schema::{entity_file, is_identity_file};
use crate::table::Data;
use crate::value::{is_key, is_whole};
use crate::Problem;

pub fn check(data: &Data, out: &mut Vec<Problem>) {
    for (file, table) in &data.tables {
        if table.has("id") {
            let mut seen = BTreeSet::new();
            for row in data.rows(file) {
                let id = row.get("id");
                if !is_whole(id) {
                    out.push(row.problem(format!("id `{id}` is not a whole number")));
                }
                if !seen.insert(id) {
                    out.push(row.problem(format!("duplicate id `{id}`")));
                }
            }
        }
        if is_identity_file(file) {
            let mut seen = BTreeSet::new();
            for row in data.rows(file) {
                let key = row.get("key");
                if !is_key(key) {
                    out.push(row.problem(format!(
                        "key `{key}` must be lower-case letters, digits, `.` and `-`"
                    )));
                }
                if !seen.insert(key) {
                    out.push(row.problem(format!("duplicate key `{key}`")));
                }
            }
        }
    }
    attributes(data, out);
}

/// `attributes.csv`: an attribute name may be reused across entity types
/// (`max_speed_kn` for a ship class and for an aircraft type), so what is
/// unique is the pair (key, applies_to).
fn attributes(data: &Data, out: &mut Vec<Problem>) {
    let mut seen = BTreeSet::new();
    for row in data.rows("attributes.csv") {
        let (key, applies_to) = (row.get("key"), row.get("applies_to"));
        if key.is_empty() {
            out.push(row.problem("attribute has no key"));
        }
        if !seen.insert((key, applies_to)) {
            out.push(row.problem(format!("duplicate attribute `{key}` for `{applies_to}`")));
        }
        if entity_file(applies_to).is_none() {
            out.push(row.problem(format!("unknown entity type `{applies_to}`")));
        }
        let value_type = row.get("value_type");
        if !matches!(
            value_type,
            "integer"
                | "decimal"
                | "date"
                | "text"
                | "ref:ship_class"
                | "ref:aircraft_type"
                | "ref:place_or_ship"
        ) {
            out.push(row.problem(format!("unknown value type `{value_type}`")));
        }
        let cardinality = row.get("cardinality");
        if !matches!(cardinality, "one" | "many") {
            out.push(row.problem(format!("cardinality `{cardinality}` is not one or many")));
        }
    }
}
