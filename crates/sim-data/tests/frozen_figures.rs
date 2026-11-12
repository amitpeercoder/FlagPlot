//! The frozen figures (`data/reference/step1`) keep up with the live ones in
//! shape, though not in value (docs/step2-spec.md, "Tests while figures
//! move").
//!
//! A hand-calculation test reads its figures from the frozen directory. A rule
//! that adds a figure adds it to the live file; if the frozen file did not get
//! it too, the loader would fail the test with a missing key, or worse, read a
//! default. These tests fail first and say which file and which key.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

fn reference_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/reference")
}

/// The figures files of `data/reference`: its CSV and TOML files that are not
/// the provenance files of a derivation (`.sources.csv`, `.ships.csv`).
fn figures_files(dir: &Path) -> BTreeSet<String> {
    fs::read_dir(dir)
        .expect("the directory reads")
        .map(|entry| {
            entry
                .expect("an entry reads")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .filter(|name| {
            (name.ends_with(".csv") || name.ends_with(".toml"))
                && !name.ends_with(".sources.csv")
                && !name.ends_with(".ships.csv")
        })
        .collect()
}

/// Every key of a TOML document as a dotted path, tables walked and anything
/// else a leaf.
fn toml_keys(prefix: &str, value: &toml::Value, into: &mut BTreeSet<String>) {
    if let toml::Value::Table(table) = value {
        for (key, inner) in table {
            let path = if prefix.is_empty() {
                key.clone()
            } else {
                format!("{prefix}.{key}")
            };
            into.insert(path.clone());
            toml_keys(&path, inner, into);
        }
    }
}

/// The columns of a CSV file: its first line that is neither blank nor a
/// comment.
fn csv_columns(text: &str) -> BTreeSet<String> {
    text.lines()
        .find(|line| !line.trim().is_empty() && !line.starts_with('#'))
        .expect("the file has a header")
        .split(',')
        .map(|column| column.trim().to_string())
        .collect()
}

/// What `file` has in shape: its TOML keys or its CSV columns.
fn shape(file: &str, text: &str) -> BTreeSet<String> {
    if file.ends_with(".toml") {
        let value: toml::Value = toml::from_str(text).expect("the TOML parses");
        let mut keys = BTreeSet::new();
        toml_keys("", &value, &mut keys);
        keys
    } else {
        csv_columns(text)
    }
}

#[test]
fn every_live_figures_file_has_a_frozen_counterpart_and_the_other_way_round() {
    let live = figures_files(&reference_dir());
    let frozen = figures_files(&reference_dir().join("step1"));
    let missing: Vec<&String> = live.difference(&frozen).collect();
    assert!(
        missing.is_empty(),
        "data/reference has figures files that data/reference/step1 lacks (freeze them, or \
         say in its README why not): {missing:?}"
    );
    let stale: Vec<&String> = frozen.difference(&live).collect();
    assert!(
        stale.is_empty(),
        "data/reference/step1 has files with no live counterpart: {stale:?}"
    );
}

#[test]
fn a_frozen_file_has_every_key_and_column_its_live_counterpart_has() {
    let live_dir = reference_dir();
    let frozen_dir = live_dir.join("step1");
    for file in figures_files(&live_dir) {
        let Ok(frozen_text) = fs::read_to_string(frozen_dir.join(&file)) else {
            continue; // the test above says so
        };
        let live_text = fs::read_to_string(live_dir.join(&file)).expect("the live file reads");
        let frozen = shape(&file, &frozen_text);
        let lacking: Vec<String> = shape(&file, &live_text)
            .difference(&frozen)
            .cloned()
            .collect();
        assert!(
            lacking.is_empty(),
            "data/reference/step1/{file} lacks what data/reference/{file} has: {lacking:?}. A \
             rule that adds a figure adds it to the frozen file too, with its first value."
        );
    }
}

#[test]
fn a_frozen_file_says_it_is_frozen() {
    let frozen_dir = reference_dir().join("step1");
    for file in figures_files(&frozen_dir) {
        let text = fs::read_to_string(frozen_dir.join(&file)).expect("the file reads");
        assert!(
            text.starts_with("# FROZEN:"),
            "data/reference/step1/{file} has lost its header"
        );
    }
}
