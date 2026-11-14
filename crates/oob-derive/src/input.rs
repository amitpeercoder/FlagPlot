//! The two input files: the list of ships and the rules.

use std::collections::{BTreeMap, BTreeSet};

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
    #[serde(default)]
    class_ids: Option<BTreeMap<String, Vec<String>>>,
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
    #[serde(default)]
    experience: Option<crate::experience::RawExperience>,
    #[serde(default)]
    troop_figures: bool,
    #[serde(default)]
    troops: Option<RawTroops>,
}

/// The `[troops]` table of a list: the voyage whose loads are read and the
/// runs read for ships (decision 0093, 8).
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawTroops {
    #[serde(default)]
    voyage: Option<String>,
    #[serde(default)]
    run: Vec<RawRun>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRun {
    run: String,
    ships: Vec<String>,
    men_row: i64,
    ships_row: i64,
    ships_in_run: i64,
    reason: String,
}

/// One run read for some ships of the list.
#[derive(Clone, Debug)]
pub struct TroopRun {
    /// Key in `lift_entities.csv`, of kind run.
    pub run: String,
    pub ships: Vec<String>,
    /// Row of `troop_lift_facts.csv` giving the men carried.
    pub men_row: i64,
    /// Row of `troop_lift_facts.csv` giving the ships in the run.
    pub ships_row: i64,
    /// The list's reading of the ships row.
    pub ships_in_run: i64,
    pub reason: String,
}

/// The `[troops]` table, checked.
#[derive(Clone, Debug, Default)]
pub struct TroopsInput {
    /// The voyage whose loads are read, as written and as a day.
    pub voyage: Option<(String, Day)>,
    pub runs: Vec<TroopRun>,
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
    #[allow(dead_code)] // checked against the pins when the list is parsed
    pub first_class_id: u32,
    /// The class rows, pinned (decision 0077): each class id and the ships
    /// that have it. Required when the list generates class files.
    pub class_ids: Option<BTreeMap<u32, Vec<String>>>,
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
    /// Ships' experience values to derive (decision 0064).
    pub experience: Option<crate::experience::Experience>,
    /// Write the `troop_capacity_men` column (decision 0093, 8).
    pub troop_figures: bool,
    /// The voyage and the runs the troop figure is read from.
    pub troops: TroopsInput,
}

/// Checks the `[troops]` table against the list.
fn parse_troops(raw: &RawTroops, ships: &[String], problems: &mut Vec<Problem>) -> TroopsInput {
    let mut input = TroopsInput::default();
    if let Some(text) = &raw.voyage {
        match Day::parse_full(text) {
            Some(day) => input.voyage = Some((text.clone(), day)),
            None => problems.push(list_problem(
                "troops",
                format!("voyage `{text}` is not a full date, YYYY-MM-DD"),
            )),
        }
    }
    let mut in_run: BTreeMap<&str, &str> = BTreeMap::new();
    let mut seen_runs: Vec<&str> = Vec::new();
    for entry in &raw.run {
        let run = entry.run.as_str();
        let fail = |problems: &mut Vec<Problem>, message: String| {
            problems.push(Problem::new(run, Kind::List, "troops", message));
        };
        let mut ok = true;
        if seen_runs.contains(&run) {
            fail(problems, "the run is listed more than once".to_string());
            ok = false;
        }
        seen_runs.push(run);
        if run.trim().is_empty() {
            fail(problems, "`run` is empty".to_string());
            ok = false;
        }
        if entry.reason.trim().is_empty() {
            fail(problems, "`reason` must not be empty".to_string());
            ok = false;
        }
        if entry.ships.is_empty() {
            fail(problems, "`ships` is empty".to_string());
            ok = false;
        }
        if entry.ships_in_run < 1 {
            fail(
                problems,
                format!("ships_in_run {} is not at least 1", entry.ships_in_run),
            );
            ok = false;
        } else if usize::try_from(entry.ships_in_run).is_ok_and(|n| n < entry.ships.len()) {
            fail(
                problems,
                format!(
                    "ships_in_run {} is fewer than the {} ships named for the run",
                    entry.ships_in_run,
                    entry.ships.len()
                ),
            );
            ok = false;
        }
        let mut named: Vec<&str> = Vec::new();
        for ship in &entry.ships {
            if !ships.contains(ship) {
                fail(problems, format!("`{ship}` is not in ships"));
                ok = false;
            } else if named.contains(&ship.as_str()) {
                fail(
                    problems,
                    format!("ship `{ship}` is named twice for the run"),
                );
                ok = false;
            } else if let Some(other) = in_run.get(ship.as_str()) {
                if *other != run {
                    fail(
                        problems,
                        format!("ship `{ship}` is already read for run {other}: a ship is in at most one run"),
                    );
                    ok = false;
                }
            }
            named.push(ship);
            in_run.entry(ship).or_insert(run);
        }
        if ok {
            input.runs.push(TroopRun {
                run: entry.run.clone(),
                ships: entry.ships.clone(),
                men_row: entry.men_row,
                ships_row: entry.ships_row,
                ships_in_run: entry.ships_in_run,
                reason: entry.reason.clone(),
            });
        }
    }
    input
}

/// Checks the `[class_ids]` table against the list (decision 0077): it is
/// required when the list has ships, and pins every ship once, to an id at
/// or above `first_class_id`, each id once.
fn pinned_ids(
    raw: &RawList,
    ships: &[String],
    problems: &mut Vec<Problem>,
) -> Option<BTreeMap<u32, Vec<String>>> {
    let Some(table) = &raw.class_ids else {
        if !raw.ships.is_empty() {
            problems.push(list_problem(
                "class_ids",
                "required when the list has ships: a [class_ids] table giving each class id and the ships that have it, so that ids never drift (decision 0077)",
            ));
        }
        return None;
    };
    if raw.ships.is_empty() {
        problems.push(list_problem("class_ids", "the list has no ships"));
        return None;
    }
    let mut pins: BTreeMap<u32, Vec<String>> = BTreeMap::new();
    let mut pinned_to: BTreeMap<&str, u32> = BTreeMap::new();
    for (key, members) in table {
        let Ok(id) = key.trim().parse::<u32>() else {
            problems.push(list_problem(
                "class_ids",
                format!("`{key}` is not a class id (a whole number)"),
            ));
            continue;
        };
        if id < raw.first_class_id.unwrap_or(0) {
            problems.push(list_problem(
                "class_ids",
                format!(
                    "class id {id} is below first_class_id {}",
                    raw.first_class_id.unwrap_or(0)
                ),
            ));
        }
        if pins.contains_key(&id) {
            problems.push(list_problem(
                "class_ids",
                format!("class id {id} is pinned twice"),
            ));
            for ship in members {
                pinned_to.entry(ship.as_str()).or_insert(id);
            }
            continue;
        }
        if members.is_empty() {
            problems.push(list_problem(
                "class_ids",
                format!("class id {id} has no ships"),
            ));
        }
        for ship in members {
            if !ships.contains(ship) {
                problems.push(list_problem(
                    "class_ids",
                    format!("`{ship}`, pinned to class id {id}, is not in ships"),
                ));
            } else if let Some(first) = pinned_to.insert(ship.as_str(), id) {
                problems.push(list_problem(
                    "class_ids",
                    format!("ship `{ship}` is pinned twice: to class ids {first} and {id}"),
                ));
            }
        }
        pins.insert(id, members.clone());
    }
    for ship in ships {
        if !pinned_to.contains_key(ship.as_str()) {
            problems.push(list_problem(
                "class_ids",
                format!("ship `{ship}` has no class id pinned"),
            ));
        }
    }
    Some(pins)
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
        if raw.aircraft.is_empty() && raw.experience.is_none() {
            problems.push(list_problem(
                "ships",
                "the list has neither ships nor [[aircraft]] entries (nor an [experience] table)",
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
    let class_ids = pinned_ids(&raw, &ships, &mut problems);
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
    let experience = match raw.experience {
        Some(raw) => {
            let (experience, experience_problems) = crate::experience::parse_list(raw);
            problems.extend(experience_problems);
            experience
        }
        None => None,
    };
    let troops = match &raw.troops {
        Some(_) if !raw.troop_figures => {
            problems.push(list_problem(
                "troops",
                "a [troops] table needs troop_figures = true",
            ));
            TroopsInput::default()
        }
        Some(table) => parse_troops(table, &ships, &mut problems),
        None => TroopsInput::default(),
    };
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
        class_ids,
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
        experience,
        troop_figures: raw.troop_figures,
        troops,
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
    #[serde(default)]
    experience: Option<crate::experience::RawRulesExperience>,
    #[serde(default)]
    fire_rate: Option<crate::fire_rate::RawFireRate>,
    #[serde(default)]
    search_radius: Option<RawSearchRadius>,
}

/// The `[search_radius]` block of the rules file (decision 0075).
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawSearchRadius {
    divisor_of_scouting_range: toml::Value,
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

/// The `[search_radius]` block: a type's search radius is its range in the
/// scouting condition divided by this, rounded down (decision 0075).
#[derive(Clone, Debug)]
pub struct SearchRadiusRules {
    pub divisor: i64,
    /// Exceptions read only when the search radius is read.
    pub exceptions: Vec<Exception>,
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
    /// The questions' thresholds and the navies' rows (decision 0064).
    pub experience: Option<crate::experience::ExperienceRules>,
    /// The cases of achieved rates of fire (decision 0070).
    pub fire_rate: Option<crate::fire_rate::FireRateRules>,
    /// The search radius of an aircraft type (decision 0075); without the
    /// block the search range is the strike radius.
    pub search_radius: Option<SearchRadiusRules>,
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

/// Checks the exceptions of one list of them. `label` names the list in
/// messages.
fn parse_exceptions(
    raw: Vec<RawException>,
    label: &str,
    problems: &mut Vec<Problem>,
) -> Vec<Exception> {
    let mut exceptions = Vec::new();
    for (i, e) in raw.into_iter().enumerate() {
        let figure = format!("{label} {}", i + 1);
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
    exceptions
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
    let exceptions = parse_exceptions(raw.exception, "exception", &mut problems);
    let search_radius = raw.search_radius.map(|raw| {
        let divisor = match raw.divisor_of_scouting_range {
            toml::Value::Integer(n) if n > 0 => n,
            other => {
                problems.push(rules_problem(
                    "search_radius divisor_of_scouting_range",
                    format!("must be a whole number above zero, not `{other}`"),
                ));
                1
            }
        };
        SearchRadiusRules {
            divisor,
            exceptions: parse_exceptions(raw.exception, "search_radius exception", &mut problems),
        }
    });
    let experience = match raw.experience {
        Some(raw) => {
            let (experience, experience_problems) = crate::experience::parse_rules(raw);
            problems.extend(experience_problems);
            experience
        }
        None => None,
    };
    let fire_rate = match raw.fire_rate {
        Some(raw) => {
            let (fire_rate, fire_rate_problems) = crate::fire_rate::parse(raw);
            problems.extend(fire_rate_problems);
            fire_rate
        }
        None => None,
    };
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
            experience,
            fire_rate,
            search_radius,
        }),
        _ => None,
    };
    (rules, problems)
}
