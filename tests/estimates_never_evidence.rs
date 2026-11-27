//! An estimate is never evidence (decision 0207, rule 12), held over every
//! file on file. A class file games load names, in its column `estimated`,
//! the figures of each class that rest on a figure estimated from like
//! ships (rule 10). Then:
//!
//! - every scenario file under `data/scenarios` that loads such a class says
//!   so in its header (the comment lines before its first key): that figures
//!   are estimated, citing decision 0207;
//! - every measure of `data/validation` whose file's scenario loads such a
//!   class names each such class in its own notes (`reason`) and says it is
//!   estimated.
//!
//! The checks are `sim_cli::estimates`; scenario 121, statement 12, shows
//! each refusing a file that hides the mark. This file holds them over what
//! is on file, so that a scenario or a measure added later is held too.

use crate::common;

use std::path::{Path, PathBuf};

use sim_cli::estimates::{estimated_classes, header_problem, measure_problems};

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

/// Every `.toml` file under `dir` that names its class files, but in the
/// `tmp-` folders, which are someone's trials.
fn scenario_files(dir: &Path, into: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).expect("a folder") {
        let path = entry.expect("an entry").path();
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        if path.is_dir() {
            if !name.starts_with("tmp-") {
                scenario_files(&path, into);
            }
        } else if name.ends_with(".toml") {
            let text = std::fs::read_to_string(&path).expect("reads");
            // An overlay (decision 0234) need not name its class files.
            if text.lines().any(|l| {
                l.trim_start().starts_with("ship_classes") || l.trim_start().starts_with("built_on")
            }) {
                into.push(path);
            }
        }
    }
}

#[test]
fn every_scenario_file_that_loads_an_estimate_says_so_in_its_header() {
    let mut files = Vec::new();
    scenario_files(&common::data_dir().join("scenarios"), &mut files);
    files.sort();
    assert!(files.len() > 100, "{} files", files.len());
    let mut marked = Vec::new();
    let mut problems = Vec::new();
    for file in &files {
        if !estimated_classes(file)
            .unwrap_or_else(|e| panic!("{e}"))
            .is_empty()
        {
            marked.push(file.clone());
        }
        if let Some(problem) = header_problem(file).unwrap_or_else(|e| panic!("{e}")) {
            problems.push(problem);
        }
    }
    assert!(problems.is_empty(), "{problems:#?}");
    // The files that load a class with the mark today: the Coral Sea's with
    // the oilers (Neosho's cargo) and with both oilers (Toho Maru's light
    // displacement and cargo), their bases and the table's copies.
    assert!(
        marked
            .iter()
            .any(|f| f.ends_with("coral-sea/03a_coral_sea_both_oilers.toml")),
        "{marked:?}"
    );
    assert!(
        marked
            .iter()
            .any(|f| f.ends_with("coral-sea/02a_coral_sea_oilers.toml")),
        "{marked:?}"
    );
    eprintln!(
        "{} scenario files load an estimate: {marked:#?}",
        marked.len()
    );
}

#[test]
fn every_measure_that_rests_on_an_estimate_says_so_in_its_notes() {
    let folder = common::data_dir().join("validation");
    let mut checked = 0;
    for entry in std::fs::read_dir(&folder).expect("the measures") {
        let path = entry.expect("an entry").path();
        if path.extension().is_none_or(|e| e != "toml") {
            continue;
        }
        let text = std::fs::read_to_string(&path).expect("reads");
        if !text.contains("[[measure]]") {
            continue;
        }
        let problems =
            measure_problems(&text, &repo()).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        assert!(problems.is_empty(), "{}: {problems:#?}", path.display());
        checked += 1;
    }
    assert!(checked >= 5, "{checked} measures files");
}
