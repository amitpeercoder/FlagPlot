//! Rule 4: every sourced row says where it came from and how firm it is.
//! A sourced row is a row of any file that has a `standing` column.

use std::collections::BTreeMap;

use crate::table::Data;
use crate::value::is_date;
use crate::Problem;

pub fn check(data: &Data, out: &mut Vec<Problem>) {
    let reliability = reliability(data);
    for (file, table) in &data.tables {
        if !table.has("standing") {
            continue;
        }
        for row in data.rows(file) {
            let source = row.get("source");
            if source.is_empty() {
                out.push(row.problem("no source"));
            }
            if row.get("locator").is_empty() {
                out.push(row.problem("no locator"));
            }
            let accessed = row.get("accessed");
            if !is_date(accessed) {
                out.push(row.problem(format!("accessed `{accessed}` is not a date")));
            }
            let confidence = row.get("confidence");
            if !matches!(confidence, "high" | "medium" | "low") {
                out.push(row.problem(format!("unknown confidence `{confidence}`")));
            }
            let basis = row.get("basis");
            if !matches!(basis, "stated" | "inferred") {
                out.push(row.problem(format!("unknown basis `{basis}`")));
            }
            let standing = row.get("standing");
            if !matches!(
                standing,
                "preferred" | "unconfirmed" | "alternative" | "disputed"
            ) {
                out.push(row.problem(format!("unknown standing `{standing}`")));
            }
            if reliability.get(source) == Some(&"C") && confidence != "low" {
                out.push(row.problem(format!(
                    "source `{source}` is graded C, so the row must be low confidence"
                )));
            }
            let needs_note = confidence == "low"
                || basis == "inferred"
                || matches!(standing, "alternative" | "unconfirmed");
            if needs_note && row.get("note").is_empty() {
                out.push(
                    row.problem("a low, inferred, alternative or unconfirmed row needs a note"),
                );
            }
        }
    }
}

/// Source key to its reliability grade.
pub fn reliability(data: &Data) -> BTreeMap<&str, &str> {
    data.rows("sources.csv")
        .iter()
        .map(|s| (s.get("key"), s.get("reliability")))
        .collect()
}
