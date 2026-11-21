//! An estimate is never evidence (decision 0207, rule 12). A class file
//! that games load names, in its column `estimated`, the figures of each
//! class that rest on a figure estimated from like ships (rule 10). These
//! are the two checks that keep the mark in sight, for the tests that hold
//! them over every file on file:
//!
//! - a scenario file that loads such a class says so in its header;
//! - a measure of the validation whose scenario loads such a class names
//!   the class in its own notes (its `reason`) and says it is estimated.
//!
//! Both read the files as text and the class files through `sim-data`; no
//! rule of the game reads the mark.

use std::path::{Path, PathBuf};

use serde::Deserialize;
use sim_core::ids::ShipClassId;

/// A class a scenario loads with figures that rest on an estimate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EstimatedClass {
    pub id: ShipClassId,
    /// The class's name in its class file.
    pub name: String,
    /// The figures marked, by their column names.
    pub figures: Vec<String>,
    /// The class file it is in, as the scenario names it, made a path.
    pub file: PathBuf,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Files {
    One(String),
    Many(Vec<String>),
}

#[derive(Deserialize)]
struct ScenarioClasses {
    ship_classes: Files,
}

/// The classes with estimated figures that the scenario file at `path`
/// loads, in the order its class files are named and then by id.
pub fn estimated_classes(path: &Path) -> Result<Vec<EstimatedClass>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    estimated_classes_of(&text, path)
}

/// As [`estimated_classes`], for the text of a scenario file as if it sat
/// at `path`.
pub fn estimated_classes_of(text: &str, path: &Path) -> Result<Vec<EstimatedClass>, String> {
    let file: ScenarioClasses =
        toml::from_str(text).map_err(|e| format!("{}: {e}", path.display()))?;
    let names = match file.ship_classes {
        Files::One(name) => vec![name],
        Files::Many(names) => names,
    };
    let folder = path.parent().unwrap_or_else(|| Path::new("."));
    let mut classes = Vec::new();
    for name in names {
        let class_path = folder.join(&name);
        let (ref_data, marked) =
            sim_data::load_ship_classes_marked(&class_path).map_err(|e| e.to_string())?;
        for (id, figures) in marked {
            let name = ref_data
                .ship_classes
                .get(&id)
                .map(|c| c.name.clone())
                .unwrap_or_default();
            classes.push(EstimatedClass {
                id,
                name,
                figures,
                file: class_path.clone(),
            });
        }
    }
    Ok(classes)
}

/// The comment lines a file opens with, before its first key: its header.
pub fn header(text: &str) -> String {
    text.lines()
        .take_while(|l| {
            let l = l.trim_start();
            l.is_empty() || l.starts_with('#')
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// What is wrong with the header of the scenario file at `path`, if
/// anything: where it loads a class with estimated figures, its header must
/// say that figures are estimated and cite decision 0207 (rule 12). `None`
/// for a file that loads no such class, or says so.
pub fn header_problem(path: &Path) -> Result<Option<String>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    header_problem_of(&text, path)
}

/// As [`header_problem`], for the text of a scenario file as if it sat at
/// `path`.
pub fn header_problem_of(text: &str, path: &Path) -> Result<Option<String>, String> {
    let classes = estimated_classes_of(text, path)?;
    if classes.is_empty() {
        return Ok(None);
    }
    let head = header(text).to_lowercase();
    if head.contains("estimated") && head.contains("decision 0207") {
        return Ok(None);
    }
    Ok(Some(format!(
        "{} loads {} with estimated figures and its header does not say so (decision 0207, rule 12): it must say they are estimated, citing decision 0207",
        path.display(),
        classes
            .iter()
            .map(|c| format!("class {} ({}: {})", c.id, c.name, c.figures.join(" ")))
            .collect::<Vec<_>>()
            .join(", ")
    )))
}

#[derive(Deserialize)]
struct MeasuresFile {
    #[serde(default)]
    scenario: Option<String>,
    #[serde(default)]
    measure: Vec<Measure>,
}

#[derive(Deserialize)]
struct Measure {
    id: String,
    #[serde(default)]
    reason: String,
}

/// The measures of the measures file `text` that rest on an estimate
/// without saying so (decision 0207, rule 12): where its scenario (a path
/// from `root`, the top of the repository, as `sim-cli validate` takes it)
/// loads a class with estimated figures, every measure's `reason` must name
/// each such class and say it is estimated. One line a measure refused.
pub fn measure_problems(text: &str, root: &Path) -> Result<Vec<String>, String> {
    let file: MeasuresFile = toml::from_str(text).map_err(|e| e.to_string())?;
    let Some(scenario) = file.scenario else {
        return Ok(Vec::new());
    };
    let classes = estimated_classes(&root.join(&scenario))?;
    let mut problems = Vec::new();
    for measure in &file.measure {
        let reason = measure.reason.to_lowercase();
        let missing: Vec<&EstimatedClass> = classes
            .iter()
            .filter(|c| !(reason.contains(&c.name.to_lowercase()) && reason.contains("estimated")))
            .collect();
        if !missing.is_empty() {
            problems.push(format!(
                "measure {} rests on {scenario}, which loads {} with estimated figures, and its notes do not name the class and say it is estimated (decision 0207, rule 12)",
                measure.id,
                missing
                    .iter()
                    .map(|c| format!("class {} ({})", c.id, c.name))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
    }
    Ok(problems)
}
