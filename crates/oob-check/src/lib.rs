//! Checker for the order-of-battle facts files in `data/oob`.
//!
//! The format and its rules are in `docs/research/oob/database-design.md`
//! and decision 0015. This crate enforces them, one module per rule:
//!
//! | Module | Rule |
//! | --- | --- |
//! | `table`, `schema` | Files parse, rows have the header's field count, the expected files and columns exist |
//! | `structure` | Ids and keys are well formed and unique |
//! | `refs` | References between files resolve |
//! | `values` | Attributes are in the vocabulary and values have their declared form |
//! | `sourcing` | Sourced rows have a source, locator, date read, grades and, where needed, a note |
//! | `standing` | Chosen rows do not contradict each other, and `preferred` is earned |
//!
//! Sort order is NOT checked. The design asks for fact and fit files to be
//! sorted by entity, attribute or slot, date and source. Row order changes
//! no meaning, so it is left to a later decision.
//!
//! The checker reads files and nothing else. It does not feed the
//! simulation, which loads only `data/reference`.

mod refs;
mod schema;
mod sourcing;
mod standing;
mod structure;
pub mod table;
mod value;
mod values;

use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;

use table::{Data, Table};

/// One thing wrong with one row, or with a whole file.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Problem {
    /// File name within the checked directory.
    pub file: String,
    /// Line of the file the row starts on. 0 for a problem with the file.
    pub line: usize,
    /// How to find the row: `id 12`, `key chokai`, `line 40`, or empty for
    /// a problem with the whole file.
    pub row: String,
    pub message: String,
}

impl Problem {
    fn file(file: &str, message: impl Into<String>) -> Problem {
        Problem {
            file: file.to_string(),
            line: 0,
            row: String::new(),
            message: message.into(),
        }
    }
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.line == 0 {
            write!(f, "{}: {}", self.file, self.message)
        } else {
            write!(
                f,
                "{}:{}: {}: {}",
                self.file, self.line, self.row, self.message
            )
        }
    }
}

/// The result of a check: every problem found, sorted by file, then line.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Report {
    pub problems: Vec<Problem>,
}

impl Report {
    pub fn is_clean(&self) -> bool {
        self.problems.is_empty()
    }
}

/// Reads every `.csv` file in a directory: file name to text. Anything that
/// cannot be read is a problem, so the caller can still check what was read.
pub fn read_dir(path: &Path) -> (BTreeMap<String, String>, Vec<Problem>) {
    let mut files = BTreeMap::new();
    let mut unreadable = Vec::new();
    match std::fs::read_dir(path) {
        Err(err) => unreadable.push(Problem::file(
            &path.display().to_string(),
            format!("cannot read directory: {err}"),
        )),
        Ok(entries) => {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().into_owned();
                if !name.ends_with(".csv") {
                    continue;
                }
                match std::fs::read_to_string(entry.path()) {
                    Ok(text) => {
                        files.insert(name, text);
                    }
                    Err(err) => {
                        unreadable.push(Problem::file(&name, format!("cannot read: {err}")))
                    }
                }
            }
        }
    }
    (files, unreadable)
}

/// Checks every `.csv` file in a directory.
pub fn check_dir(path: &Path) -> Report {
    let (files, unreadable) = read_dir(path);
    let mut report = check_files(&files);
    report.problems.extend(unreadable);
    report.problems.sort();
    report
}

/// Checks a data set held in memory: file name to file text.
pub fn check_files(files: &BTreeMap<String, String>) -> Report {
    let mut problems = Vec::new();
    let mut data = Data::default();
    for (name, text) in files {
        let table = Table::parse(name, text, &mut problems);
        data.tables.insert(name.clone(), table);
    }
    let before = problems.len();
    schema::check(&data, &mut problems);
    // The rules read columns by name. With a file or column missing they
    // would report every row, so stop at the missing pieces.
    if problems.len() == before {
        structure::check(&data, &mut problems);
        refs::check(&data, &mut problems);
        values::check(&data, &mut problems);
        sourcing::check(&data, &mut problems);
        standing::check(&data, &mut problems);
    }
    problems.sort();
    problems.dedup();
    Report { problems }
}
