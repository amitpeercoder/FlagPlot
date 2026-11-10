//! Runs the checker over the repo's real `data/oob`, so CI enforces the
//! rules on every change to the data.

use std::path::Path;

/// Rows of the real data that are known to break a rule and are waiting for
/// a person to fix them, as (file, row label, part of the message). The
/// checker must not be weakened to let a bad row through, and this test must
/// not edit the data. Empty: the data passes every rule.
const KNOWN_EXCEPTIONS: [(&str, &str, &str); 0] = [];

#[test]
fn real_data_has_no_problems() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/oob");
    let report = oob_check::check_dir(&dir);
    let unexpected: Vec<String> = report
        .problems
        .iter()
        .filter(|p| {
            !KNOWN_EXCEPTIONS.iter().any(|(file, row, text)| {
                p.file == *file && p.row == *row && p.message.contains(text)
            })
        })
        .map(ToString::to_string)
        .collect();
    assert!(
        unexpected.is_empty(),
        "{} problems in data/oob:\n{}",
        unexpected.len(),
        unexpected.join("\n")
    );
}

#[test]
fn real_data_was_actually_read() {
    // Guards against the path being wrong and an empty directory "passing".
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/oob");
    assert!(dir.join("ship_facts.csv").is_file());
    let missing = oob_check::check_dir(&dir.join("no-such-directory"));
    assert!(!missing.is_clean());
}
