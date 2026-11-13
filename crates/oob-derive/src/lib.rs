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

mod aircraft;
mod date;
mod decimal;
mod derive;
mod experience;
mod fire_rate;
mod input;
mod output;
mod problem;
mod reading;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use oob_check::table::{Data, Table};

pub use derive::{AMMUNITION_NOT_LIMITING_ROUNDS, NO_RADAR_ITEMS};
pub use output::{aircraft_sources_name, companion_names, Outputs};
pub use problem::{Kind, Problem};

/// Where the rules text comes from, when it is not simply given.
pub enum RulesSource<'a> {
    Text(&'a str),
    /// The file could not be read; the message says why.
    Unreadable(String),
    /// The list could not be read well enough to know where the rules are.
    Unknown,
}

/// Derives the outputs (the three ship class files, empty for a list with no
/// ships, and the two aircraft files when the list has aircraft) from texts held in memory.
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
    problems.extend(env.fire_rate_problems.iter().cloned());
    let needs = |name: &str, given: bool, wanted: bool| {
        (wanted && !given).then(|| {
            Problem::new(
                "rules",
                Kind::Rules,
                name,
                "required by this list (decision 0029) and not in the rules file",
            )
        })
    };
    let air = list.air_figures;
    let planes = !list.aircraft.is_empty();
    problems.extend(
        [
            needs(
                "anti_aircraft_heavy_weight",
                rules.anti_aircraft_heavy_weight.is_some(),
                air,
            ),
            needs(
                "anti_aircraft_light_weight",
                rules.anti_aircraft_light_weight.is_some(),
                air,
            ),
            needs(
                "strike_radius_fraction_of_range",
                rules.strike_radius_fraction_of_range.is_some(),
                planes,
            ),
            needs(
                "search_sweep_width_nm",
                rules.search_sweep_width_nm.is_some(),
                planes,
            ),
        ]
        .into_iter()
        .flatten(),
    );
    if problems.iter().any(|p| p.kind == Kind::Rules) {
        // Nothing can be derived without the constants the list needs.
        return Err(problems);
    }
    let mut results = Vec::new();
    for ship in &list.ships {
        let opts = derive::ShipOpts {
            assume_no_radar: list.radar_unknown_means_none.contains(ship),
            assume_ammunition_not_limiting: list.main_gun_ammunition_not_limiting.contains(ship),
            air_figures: air,
            surface_weapons_not_modelled: list.surface_weapons_not_modelled.contains(ship),
        };
        match env.derive_ship(ship, opts) {
            Ok(result) => results.push(result),
            Err(ship_problems) => problems.extend(ship_problems),
        }
    }
    let mut aircraft_results = Vec::new();
    for entry in &list.aircraft {
        match env.derive_aircraft(entry) {
            Ok(result) => aircraft_results.push(result),
            Err(type_problems) => problems.extend(type_problems),
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
    let mut aircraft_used = BTreeMap::new();
    for result in &aircraft_results {
        for (index, row) in &result.used {
            aircraft_used.insert((*index, row.file.clone(), row.cite_key().1), row.clone());
        }
    }
    let context = output::Context {
        list: &list,
        list_name,
        rules: &rules,
        results: &results,
        used: &used,
        aircraft: &aircraft_results,
        aircraft_used: &aircraft_used,
        fire_rate: env.fire_rate.as_ref(),
    };
    let mut outputs = output::render(&context).map_err(|problem| vec![problem])?;
    if let Some(exp) = &list.experience {
        let Some(exp_rules) = &rules.experience else {
            return Err(vec![Problem::new(
                "rules",
                Kind::Rules,
                "experience",
                "required by this list's [experience] and not in the rules file",
            )]);
        };
        let names = experience::Names {
            list_file: list_name,
            list_title: &list.name,
            rules_file: &list.rules,
        };
        let derived = experience::derive(exp, exp_rules, &data, &names)?;
        outputs.experience_csv = Some(derived.text);
        outputs.experience_unread = derived.unread;
    }
    Ok(outputs)
}

/// What [`run`] did.
#[derive(Debug)]
pub enum Outcome {
    /// Problems were found. Nothing was written.
    Problems(Vec<Problem>),
    /// No problems. With `written` false (a `--check`), these are the files
    /// that would be written; otherwise the files that were.
    Clean {
        files: Vec<PathBuf>,
        written: bool,
        /// Rows of the experience ships that no question read, one line a
        /// ship. They are not in the generated file.
        unread: Vec<String>,
    },
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
    let mut outputs = match derive_with(&list_name, &list_text, rules, &facts, &unread) {
        Ok(outputs) => outputs,
        Err(problems) => return Ok(Outcome::Problems(problems)),
    };
    let list = list.expect("a list that derives has parsed");
    let unread = std::mem::take(&mut outputs.experience_unread);
    let mut files = Vec::new();
    if !list.ships.is_empty() {
        let class_path = dir.join(&list.output);
        let (sources_name, ships_name) = companion_names(&list.output);
        files.push((class_path.with_file_name(sources_name), outputs.sources_csv));
        files.push((class_path.with_file_name(ships_name), outputs.ships_csv));
        files.insert(0, (class_path, outputs.class_csv));
    }
    if let (Some(output), Some(csv), Some(sources)) = (
        &list.aircraft_output,
        outputs.aircraft_csv,
        outputs.aircraft_sources_csv,
    ) {
        let path = dir.join(output);
        let sources_path = path.with_file_name(aircraft_sources_name(output));
        files.push((path, csv));
        files.push((sources_path, sources));
    }
    if let (Some(exp), Some(csv)) = (&list.experience, outputs.experience_csv) {
        files.push((dir.join(&exp.output), csv));
    }
    let paths: Vec<PathBuf> = files.iter().map(|(p, _)| p.clone()).collect();
    if check_only {
        return Ok(Outcome::Clean {
            files: paths,
            written: false,
            unread,
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
        unread,
    })
}
