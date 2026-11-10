//! Derives the ship classes a scenario loads from the facts in `data/oob`
//! (decision 0020).
//!
//! Given a list of ships and a date, the tool reads only `preferred` fact
//! rows that hold on the date (plus rows the rules file names as
//! exceptions), applies the rules of decision 0020, and writes the class
//! CSV `sim-data` loads, with a sources file naming the rule and the fact
//! rows behind every figure, and a file saying which class each ship got.
//!
//! Any problem at all means nothing is written and every problem is
//! reported. The arithmetic is exact decimal arithmetic, truncating toward
//! zero; there are no floats.
//!
//! [`derive`] is a pure function over text. [`run`] does the file access.

mod date;
mod decimal;
mod derive;
mod input;
mod output;
mod problem;
mod reading;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use oob_check::table::{Data, Table};

pub use derive::NO_RADAR_ITEMS;
pub use output::{companion_names, Outputs};
pub use problem::{Kind, Problem};

/// Where the rules text comes from, when it is not simply given.
pub enum RulesSource<'a> {
    Text(&'a str),
    /// The file could not be read; the message says why.
    Unreadable(String),
    /// The list could not be read well enough to know where the rules are.
    Unknown,
}

/// Derives the three outputs from texts held in memory.
///
/// `list_name` is the list file's name, for the header. The facts are file
/// name to text, as for `oob_check::check_files`.
pub fn derive(
    list_name: &str,
    list_text: &str,
    rules_text: &str,
    facts: &BTreeMap<String, String>,
) -> Result<Outputs, Vec<Problem>> {
    derive_with(
        list_name,
        list_text,
        RulesSource::Text(rules_text),
        facts,
        &[],
    )
}

/// As [`derive`], for a caller that has already had trouble reading files:
/// `unread` are problems reading the facts directory.
pub fn derive_with(
    list_name: &str,
    list_text: &str,
    rules: RulesSource,
    facts: &BTreeMap<String, String>,
    unread: &[oob_check::Problem],
) -> Result<Outputs, Vec<Problem>> {
    let mut problems = Vec::new();
    let (list, list_problems) = input::parse_list(list_text);
    problems.extend(list_problems);
    let rules = match rules {
        RulesSource::Text(text) => {
            let (rules, rules_problems) = input::parse_rules(text);
            problems.extend(rules_problems);
            rules
        }
        RulesSource::Unreadable(why) => {
            problems.push(Problem::new("rules", Kind::Rules, "file", why));
            None
        }
        RulesSource::Unknown => None,
    };

    let mut data = Data::default();
    for (name, text) in facts {
        let mut ignored = Vec::new();
        data.tables
            .insert(name.clone(), Table::parse(name, text, &mut ignored));
    }
    let report = oob_check::check_files(facts);
    let facts_problems: Vec<Problem> = unread
        .iter()
        .chain(&report.problems)
        .map(|p| {
            let message = if p.line == 0 {
                p.message.clone()
            } else {
                format!("line {}: {}: {}", p.line, p.row, p.message)
            };
            Problem::new("facts", Kind::Facts, p.file.clone(), message)
        })
        .collect();
    let facts_clean = facts_problems.is_empty();
    problems.extend(facts_problems);

    let (Some(list), Some(rules), true) = (list, rules, facts_clean) else {
        return Err(problems);
    };

    let env = derive::Env::build(&data, list.date, &rules);
    problems.extend(env.exception_problems());
    let mut results = Vec::new();
    for ship in &list.ships {
        let assume = list.radar_unknown_means_none.contains(ship);
        match env.derive_ship(ship, assume) {
            Ok(result) => results.push(result),
            Err(ship_problems) => problems.extend(ship_problems),
        }
    }
    if !problems.is_empty() {
        return Err(problems);
    }
    let mut used = BTreeMap::new();
    for result in &results {
        for (index, row) in &result.used {
            used.insert((*index, row.file.clone(), row.cite_key().1), row.clone());
        }
    }
    let context = output::Context {
        list: &list,
        list_name,
        rules: &rules,
        results: &results,
        used: &used,
    };
    output::render(&context).map_err(|problem| vec![problem])
}

/// What [`run`] did.
#[derive(Debug)]
pub enum Outcome {
    /// Problems were found. Nothing was written.
    Problems(Vec<Problem>),
    /// No problems. With `written` false (a `--check`), these are the files
    /// that would be written; otherwise the files that were.
    Clean { files: Vec<PathBuf>, written: bool },
}

/// Reads the list, the rules it names and the facts directory, derives, and
/// writes the three files unless there is a problem or `check_only`.
///
/// Paths in the list are relative to the list file's directory. `Err` is
/// for a failure to write.
pub fn run(list_path: &Path, facts_dir: &Path, check_only: bool) -> Result<Outcome, String> {
    let list_name = list_path
        .file_name()
        .map_or_else(String::new, |n| n.to_string_lossy().into_owned());
    let list_text = match std::fs::read_to_string(list_path) {
        Ok(text) => text,
        Err(err) => {
            return Ok(Outcome::Problems(vec![Problem::new(
                "list",
                Kind::List,
                "file",
                format!("cannot read {}: {err}", list_path.display()),
            )]));
        }
    };
    let dir = list_path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let (list, _) = input::parse_list(&list_text);
    let rules_text;
    let rules = match &list {
        None => RulesSource::Unknown,
        Some(list) => {
            let path = dir.join(&list.rules);
            match std::fs::read_to_string(&path) {
                Ok(text) => {
                    rules_text = text;
                    RulesSource::Text(&rules_text)
                }
                Err(err) => {
                    RulesSource::Unreadable(format!("cannot read {}: {err}", path.display()))
                }
            }
        }
    };
    let (facts, unread) = oob_check::read_dir(facts_dir);
    let outputs = match derive_with(&list_name, &list_text, rules, &facts, &unread) {
        Ok(outputs) => outputs,
        Err(problems) => return Ok(Outcome::Problems(problems)),
    };
    let list = list.expect("a list that derives has parsed");
    let class_path = dir.join(&list.output);
    let (sources_name, ships_name) = companion_names(&list.output);
    let files = vec![
        (class_path.clone(), outputs.class_csv),
        (class_path.with_file_name(sources_name), outputs.sources_csv),
        (class_path.with_file_name(ships_name), outputs.ships_csv),
    ];
    let paths: Vec<PathBuf> = files.iter().map(|(p, _)| p.clone()).collect();
    if check_only {
        return Ok(Outcome::Clean {
            files: paths,
            written: false,
        });
    }
    let mut done: Vec<&Path> = Vec::new();
    for (path, text) in &files {
        if let Err(err) = std::fs::write(path, text) {
            for earlier in done {
                let _ = std::fs::remove_file(earlier);
            }
            return Err(format!("cannot write {}: {err}", path.display()));
        }
        done.push(path);
    }
    Ok(Outcome::Clean {
        files: paths,
        written: true,
    })
}
