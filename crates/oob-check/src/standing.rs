//! Rules 5 and 6: the `standing` column is consistent with the other rows
//! about the same thing.
//!
//! Rule 5, consistency, for fact attributes with cardinality `one`: the rows
//! of one (entity, attribute) whose dates overlap may not be "chosen"
//! (`preferred` or `unconfirmed`) with different values, and a chosen row
//! may not sit beside a `disputed` row with a different value.
//!
//! Rule 6, "a value has to earn preferred" (decision 0015, item 9): a value
//! is supported by one row at medium or high confidence, or by rows from two
//! different sources of which one is not graded C. A `preferred` row needs a
//! supported value. An `unconfirmed` row must not have one.

use std::collections::{BTreeMap, BTreeSet};

use crate::schema::{ENGAGEMENT_FILES, FACT_FILES};
use crate::sourcing::reliability;
use crate::table::{Data, Rec};
use crate::value::{normalise, span, spans_overlap};
use crate::Problem;

pub fn check(data: &Data, out: &mut Vec<Problem>) {
    let reliability = reliability(data);
    let cardinality: BTreeMap<(&str, &str), &str> = data
        .rows("attributes.csv")
        .iter()
        .map(|a| ((a.get("key"), a.get("applies_to")), a.get("cardinality")))
        .collect();

    for (file, entity_type) in FACT_FILES {
        // Rows that state one value of one thing. Their support is counted
        // together, whatever their standing.
        let mut same_value: BTreeMap<Vec<String>, Vec<Rec>> = BTreeMap::new();
        // Cardinality `one` only: every row of one (entity, attribute).
        let mut single_valued: BTreeMap<(&str, &str), Vec<Rec>> = BTreeMap::new();
        for row in data.rows(file) {
            let (entity, attribute) = (row.get("entity"), row.get("attribute"));
            let value = normalise(row.get("value"));
            let mut key = vec![entity.to_string(), attribute.to_string(), value];
            match cardinality.get(&(attribute, entity_type)) {
                // An unknown attribute is reported by `values.rs`.
                None => continue,
                Some(&"many") => key.push(row.get("valid_from").to_string()),
                Some(_) => single_valued
                    .entry((entity, attribute))
                    .or_default()
                    .push(row),
            }
            same_value.entry(key).or_default().push(row);
        }
        for rows in single_valued.values() {
            consistent(rows, out);
        }
        for rows in same_value.values() {
            earned(rows, &reliability, out);
        }
    }

    // Fits have no cardinality. Rows that give the same item and count in
    // the same slot from the same date state one value.
    let mut same_fit: BTreeMap<Vec<&str>, Vec<Rec>> = BTreeMap::new();
    for row in data.rows("fits.csv") {
        let key = ["carrier", "slot", "item", "count", "valid_from"].map(|c| row.get(c));
        same_fit.entry(key.to_vec()).or_default().push(row);
    }
    for rows in same_fit.values() {
        earned(rows, &reliability, out);
    }

    // Engagement rows are not grouped: actors and positions are free text.
    // A low-confidence row cannot be preferred on its own.
    for file in ENGAGEMENT_FILES {
        for row in data.rows(file) {
            if row.get("standing") == "preferred" && row.get("confidence") == "low" {
                out.push(row.problem("a low-confidence engagement row cannot be preferred"));
            }
        }
    }
}

fn is_chosen(row: &Rec) -> bool {
    matches!(row.get("standing"), "preferred" | "unconfirmed")
}

/// Rule 5 over the rows of one (entity, attribute) with cardinality `one`.
fn consistent(rows: &[Rec], out: &mut Vec<Problem>) {
    let clash = |a: &Rec, b: &Rec| {
        normalise(a.get("value")) != normalise(b.get("value"))
            && spans_overlap(
                &span(a.get("valid_from"), a.get("valid_to")),
                &span(b.get("valid_from"), b.get("valid_to")),
            )
    };
    for (i, row) in rows.iter().enumerate() {
        if !is_chosen(row) {
            continue;
        }
        for other in rows {
            if other.get("standing") == "disputed" && clash(row, other) {
                out.push(row.problem(format!(
                    "{} row overlaps disputed row {} with a different value",
                    row.get("standing"),
                    other.label()
                )));
            }
        }
        for other in &rows[i + 1..] {
            if is_chosen(other) && clash(row, other) {
                out.push(row.problem(format!(
                    "{} row and {} row {} overlap with different values",
                    row.get("standing"),
                    other.get("standing"),
                    other.label()
                )));
            }
        }
    }
}

/// Rule 6 over the rows that state one value.
fn earned(rows: &[Rec], reliability: &BTreeMap<&str, &str>, out: &mut Vec<Problem>) {
    let sources: BTreeSet<&str> = rows.iter().map(|r| r.get("source")).collect();
    // A source that is not in sources.csv counts as graded C.
    let not_c = |source: &&str| reliability.get(source).is_some_and(|grade| *grade != "C");
    let supported = rows
        .iter()
        .any(|r| matches!(r.get("confidence"), "medium" | "high"))
        || (sources.len() >= 2 && sources.iter().any(not_c));
    for row in rows {
        match (row.get("standing"), supported) {
            ("preferred", false) => out.push(row.problem(
                "preferred, but the value has no medium or high row and no two agreeing \
                 sources of which one is not graded C: it should be unconfirmed",
            )),
            ("unconfirmed", true) => out.push(
                row.problem("unconfirmed, but the value is supported: it should be preferred"),
            ),
            _ => {}
        }
    }
}
