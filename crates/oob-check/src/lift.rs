//! Rule 8: men carried by sea (`lift_entities.csv`, `troop_lift_facts.csv`)
//! and the runs ships are named in (`troop_run` rows of `ship_facts.csv`).
//!
//! Both files are optional, so that the small data sets `oob-derive` checks
//! of its own still pass. If present they are held to their columns. The
//! ids, the source columns and the sourcing of `troop_lift_facts.csv` are
//! checked by the general rules (a file with a `standing` column is held to
//! them); this module checks what is particular to the files: the closed
//! lists of kinds, nations and attributes, that a statement is about a key
//! of `lift_entities.csv`, the form of the figures, and that a ship's
//! `troop_run` value names a run.
//!
//! `ships_in_run` and `lift_note` are statements in words (a source's
//! description of the ships, or a note), so their values are free text.

use std::collections::BTreeMap;

use crate::table::Data;
use crate::value::{is_key, is_whole, looks_like_exponent_float};
use crate::Problem;

const ENTITIES_FILE: &str = "lift_entities.csv";
const FACTS_FILE: &str = "troop_lift_facts.csv";
const SHIP_FACTS: &str = "ship_facts.csv";

/// The columns of `lift_entities.csv`, in order.
pub const ENTITY_COLUMNS: [&str; 5] = ["id", "key", "name", "kind", "nation"];

/// The columns of `troop_lift_facts.csv`, in order.
pub const COLUMNS: [&str; 14] = [
    "id",
    "entity",
    "attribute",
    "value",
    "as_stated",
    "valid_from",
    "valid_to",
    "source",
    "locator",
    "accessed",
    "confidence",
    "basis",
    "standing",
    "note",
];

/// The columns before the source columns, which `schema.rs` requires by name.
pub const OWN_COLUMNS: usize = 7;

const KINDS: [&str; 4] = ["ship", "ship-type", "transport-group", "run"];
const NATIONS: [&str; 2] = ["us", "jp"];

/// Attributes whose value is a whole number.
const NUMERIC: [&str; 6] = [
    "troop_capacity_men",
    "troops_carried",
    "troops_lost",
    "crew_complement",
    "troops_per_ship_estimated_by_enemy",
    "supply_tons_carried",
];

/// Attributes whose value is words.
const WORDS: [&str; 2] = ["ships_in_run", "lift_note"];

pub fn check(data: &Data, out: &mut Vec<Problem>) {
    // Key to kind, for the keys of the identity file.
    let mut kinds: BTreeMap<&str, &str> = BTreeMap::new();
    if let Some(table) = data.tables.get(ENTITIES_FILE) {
        if table.fields != ENTITY_COLUMNS {
            out.push(Problem {
                file: ENTITIES_FILE.to_string(),
                line: 0,
                row: String::new(),
                message: format!("columns must be, in order: {}", ENTITY_COLUMNS.join(",")),
            });
        }
        for row in data.rows(ENTITIES_FILE) {
            let key = row.get("key");
            if !is_key(key) {
                out.push(row.problem(format!(
                    "key `{key}` must be lower-case letters, digits, `.` and `-`"
                )));
            }
            if kinds.insert(key, row.get("kind")).is_some() {
                out.push(row.problem(format!("duplicate key `{key}`")));
            }
            let (kind, nation) = (row.get("kind"), row.get("nation"));
            if !KINDS.contains(&kind) {
                out.push(row.problem(format!("kind `{kind}` is not one of {}", KINDS.join(", "))));
            }
            if !NATIONS.contains(&nation) {
                out.push(row.problem(format!(
                    "nation `{nation}` is not one of {}",
                    NATIONS.join(", ")
                )));
            }
        }
    }
    if let Some(table) = data.tables.get(FACTS_FILE) {
        if table.fields != COLUMNS {
            out.push(Problem {
                file: FACTS_FILE.to_string(),
                line: 0,
                row: String::new(),
                message: format!("columns must be, in order: {}", COLUMNS.join(",")),
            });
        }
        for row in data.rows(FACTS_FILE) {
            let entity = row.get("entity");
            if !kinds.contains_key(entity) {
                out.push(row.problem(format!("entity `{entity}` is not a key of {ENTITIES_FILE}")));
            }
            let (attribute, value) = (row.get("attribute"), row.get("value"));
            if looks_like_exponent_float(value.trim()) {
                out.push(row.problem(format!(
                    "value `{value}` looks like a float with an exponent"
                )));
            }
            if NUMERIC.contains(&attribute) {
                if !is_whole(value) {
                    out.push(row.problem(format!(
                        "value `{value}` of `{attribute}` is not a whole number"
                    )));
                }
            } else if WORDS.contains(&attribute) {
                if value.trim().is_empty() {
                    out.push(row.problem(format!("value of `{attribute}` is blank")));
                }
            } else {
                out.push(row.problem(format!("unknown attribute `{attribute}`")));
            }
        }
    }
    for row in data.rows(SHIP_FACTS) {
        if row.get("attribute") != "troop_run" {
            continue;
        }
        let run = row.get("value");
        match kinds.get(run) {
            Some(&"run") => {}
            Some(kind) => out.push(row.problem(format!(
                "troop_run `{run}` is a key of {ENTITIES_FILE} of kind `{kind}`, not a run"
            ))),
            None => {
                out.push(row.problem(format!("troop_run `{run}` is not a key of {ENTITIES_FILE}")))
            }
        }
    }
}
