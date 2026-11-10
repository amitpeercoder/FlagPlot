//! The two input files: the list of ships and the rules.

use std::collections::BTreeSet;

use serde::Deserialize;

use crate::date::Day;
use crate::decimal::Dec;
use crate::problem::{Kind, Problem};
use crate::reading::Row;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawList {
    name: String,
    date: String,
    rules: String,
    output: String,
    first_class_id: u32,
    ships: Vec<String>,
    #[serde(default)]
    radar_unknown_means_none: Vec<String>,
    #[serde(default)]
    main_gun_ammunition_not_limiting: Vec<String>,
}

/// The list file, checked.
#[derive(Clone, Debug)]
pub struct List {
    pub name: String,
    pub date_text: String,
    pub date: Day,
    /// Path of the rules file, relative to the list file.
    pub rules: String,
    /// Path of the class CSV, relative to the list file.
    pub output: String,
    pub first_class_id: u32,
    /// Ship keys in list order, each once.
    pub ships: Vec<String>,
    pub radar_unknown_means_none: BTreeSet<String>,
    /// Ships whose main-gun ammunition is treated as not limiting while no
    /// `main_rounds_per_gun` row exists (decision 0023).
    pub main_gun_ammunition_not_limiting: BTreeSet<String>,
}

fn list_problem(figure: &str, message: impl Into<String>) -> Problem {
    Problem::new("list", Kind::List, figure, message)
}

fn rules_problem(figure: &str, message: impl Into<String>) -> Problem {
    Problem::new("rules", Kind::Rules, figure, message)
}

/// Parses the list file. `None` when it cannot be used at all (it does not
/// parse, or its date is not a full date); other problems leave it usable.
pub fn parse_list(text: &str) -> (Option<List>, Vec<Problem>) {
    let mut problems = Vec::new();
    let raw: RawList = match toml::from_str(text) {
        Ok(raw) => raw,
        Err(err) => {
            problems.push(list_problem("file", err.message().replace('\n', " ")));
            return (None, problems);
        }
    };
    for (field, value) in [
        ("name", &raw.name),
        ("rules", &raw.rules),
        ("output", &raw.output),
    ] {
        if value.trim().is_empty() {
            problems.push(list_problem(field, "must not be empty"));
        }
    }
    let output = std::path::Path::new(&raw.output);
    if output.file_stem().is_none() || output.extension().is_none_or(|e| e != "csv") {
        problems.push(list_problem(
            "output",
            format!("`{}` is not the path of a .csv file", raw.output),
        ));
    }
    let date = Day::parse_full(&raw.date);
    if date.is_none() {
        problems.push(list_problem(
            "date",
            format!("`{}` is not a full date, YYYY-MM-DD", raw.date),
        ));
    }
    let mut ships = Vec::new();
    for ship in &raw.ships {
        if ships.contains(ship) {
            problems.push(list_problem(
                "ships",
                format!("ship `{ship}` is listed more than once"),
            ));
        } else {
            ships.push(ship.clone());
        }
    }
    for ship in &raw.radar_unknown_means_none {
        if !ships.contains(ship) {
            problems.push(list_problem(
                "radar_unknown_means_none",
                format!("`{ship}` is not in ships"),
            ));
        }
    }
    for ship in &raw.main_gun_ammunition_not_limiting {
        if !ships.contains(ship) {
            problems.push(list_problem(
                "main_gun_ammunition_not_limiting",
                format!("`{ship}` is not in ships"),
            ));
        }
    }
    let Some(date) = date else {
        return (None, problems);
    };
    let list = List {
        name: raw.name,
        date_text: raw.date,
        date,
        rules: raw.rules,
        output: raw.output,
        first_class_id: raw.first_class_id,
        ships,
        radar_unknown_means_none: raw.radar_unknown_means_none.into_iter().collect(),
        main_gun_ammunition_not_limiting: raw
            .main_gun_ammunition_not_limiting
            .into_iter()
            .collect(),
    };
    (Some(list), problems)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRules {
    constants_are_placeholders: bool,
    gun_shell_kg_per_damage: toml::Value,
    torpedo_damage_per_warhead_kg: toml::Value,
    displacement_lt_per_toughness: toml::Value,
    #[serde(default)]
    exception: Vec<RawException>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawException {
    entity: String,
    fact: String,
    row: i64,
    reason: String,
    agreed: String,
}

/// One agreed exception to "read only preferred rows".
#[derive(Clone, Debug)]
pub struct Exception {
    pub entity: String,
    pub fact: String,
    pub row: i64,
    pub reason: String,
    pub agreed: String,
}

impl Exception {
    /// Whether this exception names the row: same entity, same attribute or
    /// slot, same id.
    pub fn matches(&self, row: &Row) -> bool {
        row.entity == self.entity && row.name == self.fact && row.id == self.row.to_string()
    }
}

/// The rules file, checked.
#[derive(Clone, Debug)]
pub struct Rules {
    pub gun_shell_kg_per_damage: Dec,
    pub torpedo_damage_per_warhead_kg: Dec,
    pub displacement_lt_per_toughness: Dec,
    pub exceptions: Vec<Exception>,
}

fn constant(name: &str, value: &toml::Value, problems: &mut Vec<Problem>) -> Option<Dec> {
    let parsed = match value {
        toml::Value::Integer(n) => Dec::from_int(*n).ok_or_else(|| "too large".to_string()),
        toml::Value::String(text) => Dec::parse(text).map_err(|e| format!("`{text}`: {e}")),
        toml::Value::Float(_) => Err(
            "a TOML float is not allowed: write a whole number or a decimal string such as \"0.4\""
                .to_string(),
        ),
        other => Err(format!(
            "must be a whole number or a decimal string, not a {}",
            other.type_str()
        )),
    };
    match parsed {
        Ok(dec) if dec.is_positive() => Some(dec),
        Ok(_) => {
            problems.push(rules_problem(name, "must be greater than zero"));
            None
        }
        Err(message) => {
            problems.push(rules_problem(name, message));
            None
        }
    }
}

/// Parses the rules file. `None` when the constants cannot be used.
pub fn parse_rules(text: &str) -> (Option<Rules>, Vec<Problem>) {
    let mut problems = Vec::new();
    let raw: RawRules = match toml::from_str(text) {
        Ok(raw) => raw,
        Err(err) => {
            problems.push(rules_problem("file", err.message().replace('\n', " ")));
            return (None, problems);
        }
    };
    if !raw.constants_are_placeholders {
        problems.push(rules_problem(
            "constants_are_placeholders",
            "must be true: the constants are model parameters (decision 0020)",
        ));
    }
    let gun = constant(
        "gun_shell_kg_per_damage",
        &raw.gun_shell_kg_per_damage,
        &mut problems,
    );
    let torpedo = constant(
        "torpedo_damage_per_warhead_kg",
        &raw.torpedo_damage_per_warhead_kg,
        &mut problems,
    );
    let toughness = constant(
        "displacement_lt_per_toughness",
        &raw.displacement_lt_per_toughness,
        &mut problems,
    );
    let mut exceptions = Vec::new();
    for (i, e) in raw.exception.into_iter().enumerate() {
        let figure = format!("exception {}", i + 1);
        let mut ok = true;
        for (field, value) in [
            ("entity", &e.entity),
            ("fact", &e.fact),
            ("reason", &e.reason),
            ("agreed", &e.agreed),
        ] {
            if value.trim().is_empty() {
                problems.push(rules_problem(&figure, format!("`{field}` is empty")));
                ok = false;
            }
        }
        if !e.agreed.trim().is_empty() && Day::parse_full(&e.agreed).is_none() {
            problems.push(rules_problem(
                &figure,
                format!("`agreed` is `{}`, not a full date YYYY-MM-DD", e.agreed),
            ));
            ok = false;
        }
        if ok {
            exceptions.push(Exception {
                entity: e.entity,
                fact: e.fact,
                row: e.row,
                reason: e.reason,
                agreed: e.agreed,
            });
        }
    }
    let rules = match (gun, torpedo, toughness) {
        (Some(gun), Some(torpedo), Some(toughness)) => Some(Rules {
            gun_shell_kg_per_damage: gun,
            torpedo_damage_per_warhead_kg: torpedo,
            displacement_lt_per_toughness: toughness,
            exceptions,
        }),
        _ => None,
    };
    (rules, problems)
}
