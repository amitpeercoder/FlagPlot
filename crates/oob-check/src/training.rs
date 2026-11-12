//! Rule 7: statements about training and experience by navy and period
//! (`training_facts.csv`).
//!
//! The ids, the source and the other source columns are checked by the
//! general rules (a file with a `standing` column is held to them). This
//! module checks what is particular to the file: the columns in order, the
//! closed lists of entities and attributes, the form of `value` for each
//! attribute and the form and order of the dates.
//!
//! The "earned preferred" rule (`standing.rs`, rule 6) is NOT applied to
//! this file in this stage. It would fail five rows that are lone
//! medium-confidence rows marked `unconfirmed` (ids 3, 30, 31, 33 and 79);
//! whether the rule applies, or the rows change, is for the research session
//! and the architect to settle. Nor is there a consistency rule: the
//! attributes have no declared cardinality.
//!
//! The entities are navies and arms of service, not keys of an identity
//! file, so they are a closed list here (`docs/research/oob/experience.md`).

use crate::table::Data;
use crate::value::{is_date, is_whole, looks_like_exponent_float};
use crate::Problem;

const FILE: &str = "training_facts.csv";

/// The columns of the file, in order.
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

pub const ENTITIES: [&str; 11] = [
    "ijn-aviators",
    "ijn-carrier-aviators",
    "ijn-land-based-aviators",
    "usn-aviators",
    "usmc-aviators",
    "usaaf-fighter-pilots",
    "ijn-cruiser-destroyer-crews",
    "ijn-surface-fleet",
    "usn-cruiser-destroyer-crews",
    "usn-surface-fleet",
    "ran-cruiser-crews",
];

/// Attributes whose value is a whole number.
const NUMERIC: [&str; 5] = [
    "flying_hours_on_joining",
    "flying_hours_average_in_unit",
    "training_course_months",
    "training_hours_in_course",
    "pilots_trained_per_year",
];

/// Attributes whose value is free text.
const NOTES: [&str; 7] = [
    "carrier_qualification_note",
    "night_training_note",
    "service_length_note",
    "gunnery_practice_note",
    "lookout_training_note",
    "damage_control_note",
    "training_note",
];

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
    for row in data.rows(FILE) {
        let entity = row.get("entity");
        if !ENTITIES.contains(&entity) {
            out.push(row.problem(format!("unknown entity `{entity}`")));
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
        } else if NOTES.contains(&attribute) {
            if value.trim().is_empty() {
                out.push(row.problem(format!("value of `{attribute}` is blank")));
            }
        } else {
            out.push(row.problem(format!("unknown attribute `{attribute}`")));
        }
        let (from, to) = (row.get("valid_from"), row.get("valid_to"));
        let mut dates_ok = true;
        for (column, text) in [("valid_from", from), ("valid_to", to)] {
            if !text.is_empty() && !is_date(text) {
                dates_ok = false;
                out.push(row.problem(format!("{column} `{text}` is not a date")));
            }
        }
        // Dates compare as text: a coarser date sorts before the finer dates
        // inside it, and the same date in both is a point (as in `value::span`).
        if dates_ok && !from.is_empty() && !to.is_empty() && from > to {
            out.push(row.problem(format!("valid_from `{from}` is after valid_to `{to}`")));
        }
    }
}
