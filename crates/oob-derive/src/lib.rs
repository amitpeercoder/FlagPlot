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
mod submarine;
mod troops;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use oob_check::table::{Data, Table};

pub use derive::{AMMUNITION_NOT_LIMITING_ROUNDS, NO_RADAR_ITEMS};
pub use output::{aircraft_sources_name, companion_names, Outputs};
pub use problem::{Kind, Problem};
pub use submarine::SubmarineFiles;

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
    derive_among(list_name, list_text, rules_text, facts, &BTreeMap::new())
}

/// As [`derive`], with the other lists of the same folder, file name to
/// text, so that the check looks across the lists: a class with an attribute
/// estimated in any of them is not a like class for it (decision 0207, rule
/// 8). Only their `[[estimate]]` entries are read.
pub fn derive_among(
    list_name: &str,
    list_text: &str,
    rules_text: &str,
    facts: &BTreeMap<String, String>,
    other_lists: &BTreeMap<String, String>,
) -> Result<Outputs, Vec<Problem>> {
    derive_with(
        list_name,
        list_text,
        RulesSource::Text(rules_text),
        facts,
        &[],
        other_lists,
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
    other_lists: &BTreeMap<String, String>,
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

    let mut env = derive::Env::build(&data, list.date, &rules);
    env.estimates = list.estimates.clone();
    env.list_name = list_name.to_string();
    if let Some(sub) = &list.submarines {
        env.not_like = sub
            .left_out_classes
            .iter()
            .filter(|(class, _)| !sub.like_classes_kept.iter().any(|(c, _)| c == class))
            .map(|(class, _)| class.clone())
            .collect();
    }
    env.all_estimates = list
        .estimates
        .iter()
        .map(|e| {
            (
                list_name.to_string(),
                e.subject.clone(),
                e.attribute.clone(),
            )
        })
        .collect();
    for (name, text) in other_lists {
        if name == list_name {
            continue;
        }
        env.all_estimates.extend(
            input::estimates_of_text(text)
                .into_iter()
                .map(|(subject, attribute)| (name.clone(), subject, attribute)),
        );
    }
    problems.extend(env.estimate_entry_problems());
    // The column of the mark is written only where the list has an oiler
    // (decision 0206, rule 1), so that no class file of warships changes.
    let mark_column = list.ships.iter().any(|ship| env.is_oiler(ship));
    if list.troop_figures {
        let (troops, troop_problems) = troops::build(&list.troops, &data);
        problems.extend(troop_problems);
        env.troops = Some(troops);
    }
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
            mark_column,
            assume_no_anti_aircraft: list.anti_aircraft_unknown_means_none.contains(ship),
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
    let mut submarine_derived = None;
    if let Some(sub) = &list.submarines {
        match env.derive_submarines(sub) {
            Ok(derived) => submarine_derived = Some(derived),
            Err(sub_problems) => problems.extend(sub_problems),
        }
    }
    // An entry no figure reads is a problem: nothing is estimated without
    // being used, and an entry is not left standing for nothing.
    if problems.is_empty() {
        for entry in &list.estimates {
            let used = results.iter().any(|r| {
                r.estimates
                    .iter()
                    .any(|u| u.subject == entry.subject && u.attribute == entry.attribute)
            }) || submarine_derived.as_ref().is_some_and(|d| {
                d.classes.iter().any(|c| {
                    c.estimates
                        .iter()
                        .any(|u| u.subject == entry.subject && u.attribute == entry.attribute)
                })
            });
            if !used {
                problems.push(Problem::new(
                    entry.subject.key(),
                    Kind::Estimate,
                    entry.attribute.clone(),
                    format!(
                        "the list's [[estimate]] entry for {} {} is read by no figure of a listed ship",
                        entry.subject.words(),
                        entry.attribute
                    ),
                ));
            }
        }
    }
    if problems.is_empty() {
        problems.extend(output::check_pins(&list, &results, mark_column));
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
    let idle_later = env.idle_later();
    let context = output::Context {
        mark_column,
        list: &list,
        list_name,
        rules: &rules,
        results: &results,
        used: &used,
        aircraft: &aircraft_results,
        aircraft_used: &aircraft_used,
        fire_rate: env.fire_rate.as_ref(),
        idle_later: &idle_later,
    };
    let mut outputs = output::render(&context).map_err(|problem| vec![problem])?;
    outputs.search_not_settled = aircraft_results
        .iter()
        .zip(&list.aircraft)
        .filter_map(|(result, entry)| {
            result
                .search_not_settled
                .as_ref()
                .map(|why| format!("{}: {why}", entry.type_key))
        })
        .collect();
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
    if let (Some(sub), Some(derived)) = (&list.submarines, &submarine_derived) {
        outputs.submarine = Some(submarine::render(
            &list,
            list_name,
            &rules,
            sub,
            derived,
            &idle_later,
        ));
        outputs
            .estimated
            .extend(submarine::estimated_lines(derived));
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
        /// Aircraft types whose search radius was not settled, so that the
        /// strike radius is kept (decision 0075), one line a type. They are
        /// also in the header of the aircraft file.
        not_settled: Vec<String>,
        /// Ships with tubes no readable row says how they bear (decision
        /// 0074, 3), one line a ship. They are also in the header of the
        /// class file.
        torpedo_not_split: Vec<String>,
        /// The attributes estimated (decision 0207, rule 13): the count and
        /// the ships, then a line each; empty where none is.
        estimated: Vec<String>,
    },
}

/// What the program says of what [`run`] did: whether it passed, and the
/// text it prints to standard output and to standard error.
#[derive(Debug)]
pub struct Said {
    pub passed: bool,
    pub out: String,
    pub err: String,
}

/// The program's report of `result`, with `list` the list as it was named.
/// The program prints this and nothing else, so a test that asks what the
/// program says calls this rather than running the program.
pub fn said(list: &str, result: Result<Outcome, String>) -> Said {
    use std::fmt::Write;
    let mut out = String::new();
    let mut line = |text: &dyn std::fmt::Display| {
        let _ = writeln!(out, "{text}");
    };
    let passed = match result {
        Err(message) => {
            return Said {
                passed: false,
                out: String::new(),
                err: format!("{message}\n"),
            };
        }
        Ok(Outcome::Problems(problems)) => {
            for problem in &problems {
                line(problem);
            }
            let n = problems.len();
            line(&format!(
                "{list}: {n} {}",
                if n == 1 { "problem" } else { "problems" }
            ));
            false
        }
        Ok(Outcome::Clean {
            files,
            written,
            unread,
            not_settled,
            torpedo_not_split,
            estimated,
        }) => {
            line(&format!("{list}: no problems"));
            line(&if written { "wrote:" } else { "would write:" });
            for file in files {
                line(&format!("  {}", file.display()));
            }
            // The count of the estimates (decision 0207, rule 13), printed
            // for every list.
            match estimated.split_first() {
                None => line(&"estimated (decision 0207): none"),
                Some((count, lines)) => {
                    line(&format!("{count}:"));
                    for l in lines {
                        line(&format!("  {l}"));
                    }
                }
            }
            if !unread.is_empty() {
                line(&"rows of the listed ships that no question read (not in the file):");
                for l in unread {
                    line(&format!("  {l}"));
                }
            }
            if !torpedo_not_split.is_empty() {
                line(&"torpedo tubes not split: no row says how the tubes bear, so the tubes on one side are all of them (decision 0074, 3):");
                for l in torpedo_not_split {
                    line(&format!("  {l}"));
                }
            }
            if !not_settled.is_empty() {
                line(&"aircraft types whose search radius is not settled (the strike radius is kept, decision 0075):");
                for l in not_settled {
                    line(&format!("  {l}"));
                }
            }
            true
        }
    };
    Said {
        passed,
        out,
        err: String::new(),
    }
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
    let other_lists = sibling_lists(dir, &list_name);
    let mut outputs =
        match derive_with(&list_name, &list_text, rules, &facts, &unread, &other_lists) {
            Ok(outputs) => outputs,
            Err(problems) => return Ok(Outcome::Problems(problems)),
        };
    let list = list.expect("a list that derives has parsed");
    let unread = std::mem::take(&mut outputs.experience_unread);
    let not_settled = std::mem::take(&mut outputs.search_not_settled);
    let torpedo_not_split = std::mem::take(&mut outputs.torpedo_not_split);
    let estimated = std::mem::take(&mut outputs.estimated);
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
    if let (Some(sub), Some(written)) = (&list.submarines, outputs.submarine) {
        for (output, csv, sources) in [
            (&sub.output, written.class_csv, written.class_sources_csv),
            (
                &sub.torpedo_output,
                written.torpedo_csv,
                written.torpedo_sources_csv,
            ),
        ] {
            let path = dir.join(output);
            let sources_path = path.with_file_name(companion_names(output).0);
            files.push((path, csv));
            files.push((sources_path, sources));
        }
    }
    let paths: Vec<PathBuf> = files.iter().map(|(p, _)| p.clone()).collect();
    if check_only {
        return Ok(Outcome::Clean {
            files: paths,
            written: false,
            unread,
            not_settled,
            torpedo_not_split,
            estimated,
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
        not_settled,
        torpedo_not_split,
        estimated,
    })
}

/// The other `.toml` files of the list's folder, name to text, for the
/// check across the lists (decision 0207, rule 8). A file that cannot be
/// read is left out; one that is not a list has no `[[estimate]]` entries.
fn sibling_lists(dir: &Path, list_name: &str) -> BTreeMap<String, String> {
    let mut lists = BTreeMap::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return lists;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        if name == list_name || path.extension().is_none_or(|e| e != "toml") {
            continue;
        }
        if let Ok(text) = std::fs::read_to_string(&path) {
            lists.insert(name, text);
        }
    }
    lists
}
