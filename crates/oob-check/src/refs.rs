//! Rule 2: every reference from one file to another resolves.
//!
//! Fact values of a `ref:` type are references too. They are checked with
//! the other value types in `values.rs`.

use crate::schema::{ENGAGEMENT_FILES, FACT_FILES};
use crate::table::{Data, EntityKeys, Rec};
use crate::Problem;

pub fn check(data: &Data, out: &mut Vec<Problem>) {
    // Sourced rows, in whichever file: the source.
    let sources = data.keys("sources.csv");
    for (file, table) in &data.tables {
        if table.has("standing") {
            for row in data.rows(file) {
                let source = row.get("source");
                if !source.is_empty() && !sources.contains(source) {
                    out.push(row.problem(format!("unknown source `{source}`")));
                }
            }
        }
    }

    let keys = data.entity_keys();
    for (file, entity_type) in FACT_FILES {
        for row in data.rows(file) {
            typed(&keys, row, entity_type, row.get("entity"), "entity", out);
        }
    }

    for row in data.rows("fits.csv") {
        let (carrier_type, item_type) = (row.get("carrier_type"), row.get("item_type"));
        typed(&keys, row, carrier_type, row.get("carrier"), "carrier", out);
        typed(&keys, row, item_type, row.get("item"), "item", out);
    }

    let classes = data.keys("ship_classes.csv");
    for row in data.rows("ships.csv") {
        let class = row.get("class");
        if !class.is_empty() && !classes.contains(class) {
            out.push(row.problem(format!("unknown class `{class}`")));
        }
    }

    let air_units = data.keys("air_units.csv");
    for row in data.rows("air_units.csv") {
        let parent = row.get("parent");
        if !parent.is_empty() && !air_units.contains(parent) {
            out.push(row.problem(format!("unknown parent `{parent}`")));
        }
    }

    let engagements = data.keys("engagements.csv");
    for file in ENGAGEMENT_FILES {
        for row in data.rows(file) {
            let engagement = row.get("engagement");
            if !engagements.contains(engagement) {
                out.push(row.problem(format!("unknown engagement `{engagement}`")));
            }
        }
    }
    for row in data.rows("engagement_forces.csv") {
        let member_type = row.get("member_type");
        typed(&keys, row, member_type, row.get("member"), "member", out);
    }
}

/// Checks that `key` is listed in the identity file of `entity_type`.
fn typed(
    keys: &EntityKeys,
    row: Rec,
    entity_type: &str,
    key: &str,
    column: &str,
    out: &mut Vec<Problem>,
) {
    match keys.get(entity_type) {
        None => out.push(row.problem(format!("unknown {column} type `{entity_type}`"))),
        Some(known) => {
            if !known.contains(key) {
                out.push(row.problem(format!("unknown {column} `{key}` ({entity_type})")));
            }
        }
    }
}
