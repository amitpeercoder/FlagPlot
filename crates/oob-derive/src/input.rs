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
    #[serde(default)]
    output: Option<String>,
    #[serde(default)]
    first_class_id: Option<u32>,
    ships: Vec<String>,
    #[serde(default)]
    radar_unknown_means_none: Vec<String>,
    #[serde(default)]
    main_gun_ammunition_not_limiting: Vec<String>,
    #[serde(default)]
    air_figures: bool,
    #[serde(default)]
    surface_weapons_not_modelled: Option<RawNotModelled>,
    #[serde(default)]
    aircraft: Vec<RawAircraft>,
    #[serde(default)]
    aircraft_output: Option<String>,
    #[serde(default)]
    first_aircraft_type_id: Option<u32>,
}

/// Either the string "all" or a list of ship names.
#[derive(Deserialize)]
#[serde(untagged)]
enum RawNotModelled {
    Text(String),
    Names(Vec<String>),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawAircraft {
    #[serde(rename = "type")]
    kind: String,
    #[serde(default)]
    load: Option<String>,
    #[serde(default)]
    combat_radius_row: Option<i64>,
    #[serde(default)]
    combat_radius_reason: Option<String>,
}

/// One aircraft type the list asks for (decision 0029, 7).
#[derive(Clone, Debug)]
pub struct AircraftEntry {
    /// Key in `aircraft_types.csv`.
    pub type_key: String,
    /// Key of the weapon it attacks with, if any.
    pub load: Option<String>,
    /// For this list only, the row of `aircraft_type_facts.csv` the combat
    /// radius is read from, and why (a choice of the list, not of the rules
    /// file).
    pub radius_choice: Option<RadiusChoice>,
}

/// A list's own choice of the row a type's radius is read from.
#[derive(Clone, Debug)]
pub struct RadiusChoice {
    pub row: i64,
    pub reason: String,
}

/// The list file, checked.
#[derive(Clone, Debug)]
pub struct List {
    pub name: String,
    pub date_text: String,
    pub date: Day,
    /// Path of the rules file, relative to the list file.
    pub rules: String,
    /// Path of the class CSV, relative to the list file; empty when the list
    /// has no ships.
    pub output: String,
    pub first_class_id: u32,
    /// Ship keys in list order, each once.
    pub ships: Vec<String>,
    pub radar_unknown_means_none: BTreeSet<String>,
    /// Ships whose main-gun ammunition is treated as not limiting while no
    /// `main_rounds_per_gun` row exists (decision 0023).
    pub main_gun_ammunition_not_limiting: BTreeSet<String>,
    /// Write `aircraft_capacity` and `anti_aircraft` columns (decision 0029).
    pub air_figures: bool,
    /// Ships whose surface weapons are not modelled (decision 0029, 4).
    pub surface_weapons_not_modelled: BTreeSet<String>,
    pub aircraft: Vec<AircraftEntry>,
    /// Path of the aircraft CSV, relative to the list file; with aircraft.
    pub aircraft_output: Option<String>,
    pub first_aircraft_type_id: u32,
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
    for (field, value) in [("name", &raw.name), ("rules", &raw.rules)] {
        if value.trim().is_empty() {
            problems.push(list_problem(field, "must not be empty"));
        }
    }
    if raw.ships.is_empty() {
        for (field, given) in [
            ("output", raw.output.is_some()),
            ("first_class_id", raw.first_class_id.is_some()),
        ] {
            if given {
                problems.push(list_problem(field, "the list has no ships"));
            }
        }
        if raw.aircraft.is_empty() {
            problems.push(list_problem(
                "ships",
                "the list has neither ships nor [[aircraft]] entries",
            ));
        }
    } else {
        match &raw.output {
            None => problems.push(list_problem("output", "required when the list has ships")),
            Some(output) if output.trim().is_empty() => {
                problems.push(list_problem("output", "must not be empty"));
            }
            Some(output) => {
                let path = std::path::Path::new(output);
                if path.file_stem().is_none() || path.extension().is_none_or(|e| e != "csv") {
                    problems.push(list_problem(
                        "output",
                        format!("`{output}` is not the path of a .csv file"),
                    ));
                }
            }
        }
        if raw.first_class_id.is_none() {
            problems.push(list_problem(
                "first_class_id",
                "required when the list has ships",
            ));
        }
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
    let mut not_modelled = BTreeSet::new();
    match &raw.surface_weapons_not_modelled {
        None => {}
        Some(RawNotModelled::Text(text)) if text == "all" => {
            not_modelled.extend(ships.iter().cloned());
        }
        Some(RawNotModelled::Text(text)) => problems.push(list_problem(
            "surface_weapons_not_modelled",
            format!("`{text}` is not \"all\" or a list of ship names"),
        )),
        Some(RawNotModelled::Names(names)) => {
            for ship in names {
                if ships.contains(ship) {
                    not_modelled.insert(ship.clone());
                } else {
                    problems.push(list_problem(
                        "surface_weapons_not_modelled",
                        format!("`{ship}` is not in ships"),
                    ));
                }
            }
        }
    }
    let mut aircraft = Vec::new();
    for entry in &raw.aircraft {
        if aircraft
            .iter()
            .any(|a: &AircraftEntry| a.type_key == entry.kind)
        {
            problems.push(list_problem(
                "aircraft",
                format!("type `{}` is listed more than once", entry.kind),
            ));
        } else {
            let radius_choice = match (entry.combat_radius_row, &entry.combat_radius_reason) {
                (None, None) => None,
                (Some(row), Some(reason)) if !reason.trim().is_empty() => Some(RadiusChoice {
                    row,
                    reason: reason.clone(),
                }),
                (Some(_), Some(_)) => {
                    problems.push(list_problem(
                        "aircraft",
                        format!(
                            "type `{}`: combat_radius_reason must not be empty",
                            entry.kind
                        ),
                    ));
                    None
                }
                _ => {
                    problems.push(list_problem(
                        "aircraft",
                        format!(
                            "type `{}`: combat_radius_row and combat_radius_reason go together: give both or neither",
                            entry.kind
                        ),
                    ));
                    None
                }
            };
            aircraft.push(AircraftEntry {
                type_key: entry.kind.clone(),
                load: entry.load.clone(),
                radius_choice,
            });
        }
    }
    if aircraft.is_empty() {
        for (field, given) in [
            ("aircraft_output", raw.aircraft_output.is_some()),
            (
                "first_aircraft_type_id",
                raw.first_aircraft_type_id.is_some(),
            ),
        ] {
            if given {
                problems.push(list_problem(field, "the list has no [[aircraft]] entries"));
            }
        }
    } else {
        match &raw.aircraft_output {
            None => problems.push(list_problem(
                "aircraft_output",
                "required when the list has [[aircraft]] entries",
            )),
            Some(path) => {
                let path = std::path::Path::new(path);
                if path.file_stem().is_none() || path.extension().is_none_or(|e| e != "csv") {
                    problems.push(list_problem(
                        "aircraft_output",
                        format!("`{}` is not the path of a .csv file", path.display()),
                    ));
                }
            }
        }
        if raw.first_aircraft_type_id.is_none() {
            problems.push(list_problem(
                "first_aircraft_type_id",
                "required when the list has [[aircraft]] entries",
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
        output: raw.output.unwrap_or_default(),
        first_class_id: raw.first_class_id.unwrap_or(0),
        ships,
        radar_unknown_means_none: raw.radar_unknown_means_none.into_iter().collect(),
        main_gun_ammunition_not_limiting: raw
            .main_gun_ammunition_not_limiting
            .into_iter()
            .collect(),
        air_figures: raw.air_figures,
        surface_weapons_not_modelled: not_modelled,
        aircraft,
        aircraft_output: raw.aircraft_output,
        first_aircraft_type_id: raw.first_aircraft_type_id.unwrap_or(0),
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
    anti_aircraft_heavy_weight: Option<toml::Value>,
    #[serde(default)]
    anti_aircraft_light_weight: Option<toml::Value>,
    #[serde(default)]
    strike_radius_fraction_of_range: Option<toml::Value>,
    #[serde(default)]
    search_sweep_width_nm: Option<toml::Value>,
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
    /// The constants of decision 0029; read only when the list needs them.
    pub anti_aircraft_heavy_weight: Option<Dec>,
    pub anti_aircraft_light_weight: Option<Dec>,
    pub strike_radius_fraction_of_range: Option<Dec>,
    pub search_sweep_width_nm: Option<Dec>,
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
    let optional = |name: &str, value: &Option<toml::Value>, problems: &mut Vec<Problem>| {
        value.as_ref().and_then(|v| constant(name, v, problems))
    };
    let aa_heavy = optional(
        "anti_aircraft_heavy_weight",
        &raw.anti_aircraft_heavy_weight,
        &mut problems,
    );
    let aa_light = optional(
        "anti_aircraft_light_weight",
        &raw.anti_aircraft_light_weight,
        &mut problems,
    );
    let fraction = optional(
        "strike_radius_fraction_of_range",
        &raw.strike_radius_fraction_of_range,
        &mut problems,
    );
    let sweep = optional(
        "search_sweep_width_nm",
        &raw.search_sweep_width_nm,
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
            anti_aircraft_heavy_weight: aa_heavy,
            anti_aircraft_light_weight: aa_light,
            strike_radius_fraction_of_range: fraction,
            search_sweep_width_nm: sweep,
            exceptions,
        }),
        _ => None,
    };
    (rules, problems)
}
