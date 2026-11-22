//! The submarine classes and their torpedoes (decision 0218, stage 1).
//!
//! A list with `[[submarine]]` entries asks for two more kinds of output: a
//! class file of the submarine classes (a salvo, the salvoes carried, the
//! speed of passage and the torpedo each class carries) and a file of the
//! torpedoes themselves (the damage of a hit), each with a sources
//! companion. The written rules are decision 0218's rule 28 (the figures
//! and how each is read) and rule 29 (a figure still missing is estimated
//! from like boats by decision 0207 and marked; a class whose torpedo no row
//! states takes the torpedo stated for the boats of her group, or of the
//! group nearest in date, by the list's `torpedo_from` entry, and is
//! marked).
//!
//! Every fact is read as a ship's is: by [`crate::reading::resolve`] on the
//! list's date with the rules file's exceptions, so standing, dates,
//! `later = true`, a date judged to the year and a conflict between rows
//! behave exactly as they do for ships. Any problem anywhere means nothing
//! is written, as everywhere in this tool.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::Deserialize;

use crate::date::Day;
use crate::decimal::Dec;
use crate::derive::{Cite, Env, EstimateUse, Work};
use crate::input::{Exception, List, Rules};
use crate::output::{companion_names, csv_text, exception_line, idle_lines, one_line};
use crate::problem::{Kind, Problem};
use crate::reading::{resolve, Row, Table};

/// The figure of a submarine class that may rest on an estimate.
pub const PASSAGE_SPEED: &str = "passage_speed_kn";

/// The type of the classes this module derives, in ship_classes.csv.
const SUBMARINE_TYPE: &str = "submarine";

/// The only attribute of a submarine class a list may estimate (decision
/// 0218, rule 29).
const ESTIMABLE_ON_A_SUBMARINE: &str = "endurance_speed_kn";

const TUBES: &str = "torpedo_tubes_bow";
const CARRIED: &str = "torpedoes_carried";
const MODEL: &str = "torpedo_model";
const WARHEAD: &str = "warhead_kg";

/// The columns of the class file, after `id` and `name`, and the last one.
const CLASS_FIGURES: [&str; 4] = ["salvo", "salvoes_carried", PASSAGE_SPEED, "torpedo_id"];
const ESTIMATED_COLUMN: &str = "estimated";

const SALVO_RULE: &str =
    "torpedo_tubes_bow of the class: the tubes that bear forward are one salvo (decision 0218, rule 28)";
const SALVOES_RULE: &str =
    "torpedoes_carried of the class / salvo, rounded down (decision 0218, rule 28)";
const PASSAGE_RULE: &str =
    "endurance_speed_kn of the class: her speed of endurance on the surface (decision 0218, rule 28)";
const TORPEDO_RULE: &str =
    "torpedo_model of the class, a key of weapons.csv, written as the id the list pins for that torpedo (decision 0218, rule 28)";
const TORPEDO_FROM_RULE: &str =
    "taken from like boats, not read from a row of her class (decision 0218, rule 29): the torpedo stated for the boats of her group, or of the group nearest in date, by the list's torpedo_from entry, written as the id the list pins for it";
const DAMAGE_RULE: &str =
    "warhead_kg of the torpedo x torpedo_damage_per_warhead_kg (decision 0218, rule 28)";

/// The files a `[[submarine]]` list writes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubmarineFiles {
    pub class_csv: String,
    pub class_sources_csv: String,
    pub torpedo_csv: String,
    pub torpedo_sources_csv: String,
}

/// A row of engagement_events.csv, as much of it as a source note prints.
#[derive(Clone, Debug)]
pub(crate) struct EventRow {
    pub time: String,
    pub actor: String,
    pub means: String,
    pub standing: String,
}

// ------------------------------------------------------------------ the list

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RawSubmarine {
    class: String,
    #[serde(default)]
    torpedo_from: Option<RawTorpedoFrom>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawTorpedoFrom {
    weapon: String,
    #[serde(default)]
    group: Option<String>,
    rows: Vec<String>,
    by: String,
    date: String,
    reason: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RawLeftOut {
    #[serde(default)]
    class: Option<String>,
    #[serde(default)]
    weapon: Option<String>,
    reason: String,
}

/// The submarine keys of a list, as read from the file.
pub(crate) struct RawParts {
    pub output: Option<String>,
    pub torpedo_output: Option<String>,
    pub torpedoes: Vec<String>,
    pub entries: Vec<RawSubmarine>,
    pub left_out: Vec<RawLeftOut>,
    pub class_ids: Option<BTreeMap<String, String>>,
    pub torpedo_ids: Option<BTreeMap<String, String>>,
}

/// A row a `torpedo_from` entry cites: one of two files.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CitedRow {
    pub file: String,
    pub id: i64,
}

/// A `torpedo_from` entry, checked (decision 0218, rule 29).
#[derive(Clone, Debug)]
pub struct TorpedoFrom {
    pub weapon: String,
    pub group: Option<String>,
    pub rows: Vec<CitedRow>,
    pub by: String,
    pub date: String,
    pub reason: String,
}

/// A `[[submarine]]` entry.
#[derive(Clone, Debug)]
pub struct SubmarineEntry {
    pub class: String,
    pub torpedo_from: Option<TorpedoFrom>,
}

/// The submarine part of a list, checked as far as the list alone can be.
#[derive(Clone, Debug)]
pub struct SubmarineList {
    /// Path of the class CSV, relative to the list file.
    pub output: String,
    /// Path of the torpedo CSV, relative to the list file.
    pub torpedo_output: String,
    pub torpedoes: Vec<String>,
    pub entries: Vec<SubmarineEntry>,
    /// Class key and the reason it is left out.
    pub left_out_classes: Vec<(String, String)>,
    /// Weapon key and the reason it is left out.
    pub left_out_weapons: Vec<(String, String)>,
    /// The class ids pinned (decision 0077's way), id to class key.
    pub class_ids: BTreeMap<u32, String>,
    /// The torpedo ids pinned, id to weapon key.
    pub torpedo_ids: BTreeMap<u32, String>,
}

fn problem(figure: &str, message: impl Into<String>) -> Problem {
    Problem::new("list", Kind::List, figure, message)
}

fn csv_path_problem(field: &str, value: &str, problems: &mut Vec<Problem>) {
    let path = Path::new(value);
    if path.file_stem().is_none() || path.extension().is_none_or(|e| e != "csv") {
        problems.push(problem(
            field,
            format!("`{value}` is not the path of a .csv file"),
        ));
    }
}

/// The `file:id` of a cited row.
fn cited_row(text: &str) -> Result<CitedRow, String> {
    let bad = || {
        format!(
            "`{text}` is not file:id, with the file engagement_events.csv or ship_class_facts.csv and a whole number for the id"
        )
    };
    let (file, id) = text.split_once(':').ok_or_else(bad)?;
    if file != "engagement_events.csv" && file != "ship_class_facts.csv" {
        return Err(bad());
    }
    let id = id.trim().parse::<i64>().map_err(|_| bad())?;
    Ok(CitedRow {
        file: file.to_string(),
        id,
    })
}

/// Reads one pinned-id table: whole-number keys, each name once.
fn parse_pins(
    table: &BTreeMap<String, String>,
    field: &str,
    problems: &mut Vec<Problem>,
) -> BTreeMap<u32, String> {
    let mut pins = BTreeMap::new();
    let mut seen: BTreeMap<&str, u32> = BTreeMap::new();
    for (key, name) in table {
        let Ok(id) = key.trim().parse::<u32>() else {
            problems.push(problem(
                field,
                format!("`{key}` is not an id (a whole number)"),
            ));
            continue;
        };
        if let Some(first) = seen.insert(name.as_str(), id) {
            problems.push(problem(
                field,
                format!("`{name}` is pinned twice: to ids {first} and {id}"),
            ));
        }
        pins.insert(id, name.clone());
    }
    pins
}

/// Checks the submarine keys of a list. `None` when the list has none.
pub(crate) fn parse(parts: RawParts, problems: &mut Vec<Problem>) -> Option<SubmarineList> {
    let any = parts.output.is_some()
        || parts.torpedo_output.is_some()
        || !parts.entries.is_empty()
        || !parts.left_out.is_empty()
        || !parts.torpedoes.is_empty()
        || parts.class_ids.is_some()
        || parts.torpedo_ids.is_some();
    if !any {
        return None;
    }
    let mut ok = true;
    for (field, value) in [
        ("submarine_output", &parts.output),
        ("submarine_torpedo_output", &parts.torpedo_output),
    ] {
        match value {
            None => {
                problems.push(problem(
                    field,
                    "required when the list has submarine entries: submarine_output and submarine_torpedo_output go together, both or neither (decision 0218, rule 28)",
                ));
                ok = false;
            }
            Some(path) if path.trim().is_empty() => {
                problems.push(problem(field, "must not be empty"));
                ok = false;
            }
            Some(path) => csv_path_problem(field, path, problems),
        }
    }
    if parts.entries.is_empty() {
        problems.push(problem(
            "submarine",
            "the list asks for submarine output and has no [[submarine]] entry",
        ));
        ok = false;
    }
    if parts.torpedoes.is_empty() {
        problems.push(problem(
            "torpedoes",
            "required when the list has submarine entries: the torpedoes to derive, as keys of weapons.csv",
        ));
        ok = false;
    }
    let mut torpedoes: Vec<String> = Vec::new();
    for weapon in &parts.torpedoes {
        if torpedoes.contains(weapon) {
            problems.push(problem(
                "torpedoes",
                format!("torpedo `{weapon}` is listed more than once"),
            ));
        } else {
            torpedoes.push(weapon.clone());
        }
    }
    let mut entries: Vec<SubmarineEntry> = Vec::new();
    for raw in &parts.entries {
        let class = raw.class.as_str();
        if entries.iter().any(|e| e.class == class) {
            problems.push(problem(
                "submarine",
                format!("class `{class}` has more than one [[submarine]] entry"),
            ));
            continue;
        }
        let torpedo_from = raw.torpedo_from.as_ref().and_then(|from| {
            let mut good = true;
            let mut rows = Vec::new();
            for text in &from.rows {
                match cited_row(text) {
                    Ok(row) => rows.push(row),
                    Err(why) => {
                        problems.push(problem(
                            "submarine",
                            format!("class `{class}`, torpedo_from: row {why}"),
                        ));
                        good = false;
                    }
                }
            }
            if from.rows.is_empty() {
                problems.push(problem(
                    "submarine",
                    format!("class `{class}`, torpedo_from: `rows` is empty: a torpedo taken from like boats cites the rows that state it (decision 0218, rule 29)"),
                ));
                good = false;
            }
            for (field, value) in [("by", &from.by), ("reason", &from.reason)] {
                if value.trim().is_empty() {
                    problems.push(problem(
                        "submarine",
                        format!("class `{class}`, torpedo_from: `{field}` must not be empty"),
                    ));
                    good = false;
                }
            }
            if Day::parse_full(&from.date).is_none() {
                problems.push(problem(
                    "submarine",
                    format!(
                        "class `{class}`, torpedo_from: date `{}` is not a full date",
                        from.date
                    ),
                ));
                good = false;
            }
            good.then(|| TorpedoFrom {
                weapon: from.weapon.clone(),
                group: from.group.clone(),
                rows,
                by: from.by.clone(),
                date: from.date.clone(),
                reason: from.reason.clone(),
            })
        });
        let has_from = raw.torpedo_from.is_some();
        if has_from && torpedo_from.is_none() {
            ok = false;
            continue;
        }
        entries.push(SubmarineEntry {
            class: class.to_string(),
            torpedo_from,
        });
    }
    let mut left_out_classes: Vec<(String, String)> = Vec::new();
    let mut left_out_weapons: Vec<(String, String)> = Vec::new();
    for raw in &parts.left_out {
        if raw.reason.trim().is_empty() {
            problems.push(problem("left_out", "`reason` must not be empty"));
            ok = false;
            continue;
        }
        match (&raw.class, &raw.weapon) {
            (Some(class), None) => {
                if left_out_classes.iter().any(|(c, _)| c == class) {
                    problems.push(problem(
                        "left_out",
                        format!("class `{class}` is left out more than once"),
                    ));
                } else {
                    left_out_classes.push((class.clone(), raw.reason.clone()));
                }
            }
            (None, Some(weapon)) => {
                if left_out_weapons.iter().any(|(w, _)| w == weapon) {
                    problems.push(problem(
                        "left_out",
                        format!("weapon `{weapon}` is left out more than once"),
                    ));
                } else {
                    left_out_weapons.push((weapon.clone(), raw.reason.clone()));
                }
            }
            _ => {
                problems.push(problem(
                    "left_out",
                    "an entry names exactly one of `class` or `weapon`",
                ));
                ok = false;
            }
        }
    }
    let class_ids = match &parts.class_ids {
        Some(table) => parse_pins(table, "submarine_class_ids", problems),
        None => {
            problems.push(problem(
                "submarine_class_ids",
                "required when the list has submarine entries: a [submarine_class_ids] table giving each class id and its class key, so that ids never drift (decision 0077)",
            ));
            ok = false;
            BTreeMap::new()
        }
    };
    let torpedo_ids = match &parts.torpedo_ids {
        Some(table) => parse_pins(table, "submarine_torpedo_ids", problems),
        None => {
            problems.push(problem(
                "submarine_torpedo_ids",
                "required when the list has submarine entries: a [submarine_torpedo_ids] table giving each torpedo id and its weapon key (decision 0077)",
            ));
            ok = false;
            BTreeMap::new()
        }
    };
    if !ok {
        return None;
    }
    Some(SubmarineList {
        output: parts.output.unwrap_or_default(),
        torpedo_output: parts.torpedo_output.unwrap_or_default(),
        torpedoes,
        entries,
        left_out_classes,
        left_out_weapons,
        class_ids,
        torpedo_ids,
    })
}

// ------------------------------------------------------------- the derivation

/// One figure of a class or torpedo row, with what it rests on.
#[derive(Clone, Debug)]
pub struct SubFigure {
    pub value: String,
    pub cites: BTreeSet<Cite>,
    pub rule: String,
    pub note: String,
    /// The figure is estimated or taken from like boats (decision 0218,
    /// rule 29): it is named in the `estimated` column.
    pub marked: bool,
}

/// A submarine class, derived.
#[derive(Clone, Debug)]
pub struct ClassResult {
    pub class: String,
    pub name: String,
    pub id: u32,
    /// The four figures, in the order of [`CLASS_FIGURES`].
    pub figures: Vec<SubFigure>,
    pub used: Vec<(usize, Row)>,
    pub estimates: Vec<EstimateUse>,
    /// For the header, when the torpedo was taken from like boats.
    pub taken: Option<String>,
}

/// A torpedo, derived.
#[derive(Clone, Debug)]
pub struct TorpedoResult {
    pub name: String,
    pub id: u32,
    pub damage: SubFigure,
    pub used: Vec<(usize, Row)>,
}

/// Everything a list's submarine part derives.
#[derive(Clone, Debug)]
pub struct Derived {
    pub classes: Vec<ClassResult>,
    pub torpedoes: Vec<TorpedoResult>,
}

fn fig(
    value: String,
    cites: BTreeSet<Cite>,
    rule: &str,
    notes: Vec<String>,
    marked: bool,
) -> SubFigure {
    SubFigure {
        value,
        cites,
        rule: rule.to_string(),
        note: notes.join("; "),
        marked,
    }
}

impl Env<'_> {
    /// Derives the submarine classes and torpedoes of a list. All problems
    /// are returned, not the first.
    pub(crate) fn derive_submarines(&self, sub: &SubmarineList) -> Result<Derived, Vec<Problem>> {
        let mut problems = self.completeness_problems(sub);
        problems.extend(self.pin_problems(sub));
        let mut classes = Vec::new();
        for entry in &sub.entries {
            match self.derive_submarine_class(sub, entry) {
                Ok(result) => classes.push(result),
                Err(more) => problems.extend(more),
            }
        }
        let mut torpedoes = Vec::new();
        for weapon in &sub.torpedoes {
            match self.derive_torpedo(sub, weapon) {
                Ok(result) => torpedoes.push(result),
                Err(more) => problems.extend(more),
            }
        }
        // An entry no figure reads is a problem (decision 0207, rule 4):
        // the check is made with the ships' in lib.rs; here only an
        // attribute a submarine class may not have estimated is refused.
        for entry in &self.estimates {
            if let crate::input::Subject::Class(class) = &entry.subject {
                if sub.entries.iter().any(|e| e.class == *class)
                    && entry.attribute != ESTIMABLE_ON_A_SUBMARINE
                {
                    problems.push(Problem::new(
                        class.as_str(),
                        Kind::Estimate,
                        entry.attribute.clone(),
                        format!(
                            "class {class} {}: only {ESTIMABLE_ON_A_SUBMARINE} of a submarine class may be estimated (decision 0218, rule 29)",
                            entry.attribute
                        ),
                    ));
                }
            }
        }
        if !problems.is_empty() {
            return Err(problems);
        }
        classes.sort_by_key(|c| c.id);
        torpedoes.sort_by_key(|t| t.id);
        Ok(Derived { classes, torpedoes })
    }

    /// Every class of the type submarine is derived or left out, never both
    /// and never twice; a left-out weapon is a torpedo not derived (decision
    /// 0218, rule 1).
    fn completeness_problems(&self, sub: &SubmarineList) -> Vec<Problem> {
        let mut problems = Vec::new();
        for (key, info) in &self.classes {
            if info.kind_type != SUBMARINE_TYPE {
                continue;
            }
            let derived = sub.entries.iter().any(|e| e.class == *key);
            let left = sub.left_out_classes.iter().any(|(c, _)| c == key);
            if derived && left {
                problems.push(Problem::new(
                    key.as_str(),
                    Kind::List,
                    "left_out",
                    "the class is both a [[submarine]] entry and [[left_out]]: it is one or the other (decision 0218, rule 1)",
                ));
            } else if !derived && !left {
                problems.push(Problem::new(
                    key.as_str(),
                    Kind::List,
                    "submarine",
                    "a class of the type submarine in ship_classes.csv is neither a [[submarine]] entry nor [[left_out]] with its reason: every one is one or the other (decision 0218, rule 1)",
                ));
            }
        }
        for (class, _) in &sub.left_out_classes {
            match self.classes.get(class) {
                Some(info) if info.kind_type == SUBMARINE_TYPE => {}
                Some(info) => problems.push(Problem::new(
                    class.as_str(),
                    Kind::List,
                    "left_out",
                    format!(
                        "the class is of the type `{}`, not submarine, and is not the list's to leave out",
                        info.kind_type
                    ),
                )),
                None => problems.push(Problem::new(
                    class.as_str(),
                    Kind::NoClass,
                    "left_out",
                    "no such class key in ship_classes.csv",
                )),
            }
        }
        for (weapon, _) in &sub.left_out_weapons {
            if sub.torpedoes.contains(weapon) {
                problems.push(Problem::new(
                    weapon.as_str(),
                    Kind::List,
                    "left_out",
                    "the weapon is both in `torpedoes` and left out: it is one or the other",
                ));
            }
            if self.weapons.get(weapon).map(String::as_str) != Some("torpedo") {
                problems.push(Problem::new(
                    weapon.as_str(),
                    Kind::List,
                    "left_out",
                    "a weapon left out must be a weapon of kind torpedo in weapons.csv",
                ));
            }
        }
        problems
    }

    /// Each derived class and torpedo is pinned once, and nothing else is
    /// (decision 0077's way, for the submarine files).
    fn pin_problems(&self, sub: &SubmarineList) -> Vec<Problem> {
        let mut problems = Vec::new();
        let derived: BTreeSet<&str> = sub.entries.iter().map(|e| e.class.as_str()).collect();
        let pinned: BTreeSet<&str> = sub.class_ids.values().map(String::as_str).collect();
        for class in &derived {
            if !pinned.contains(class) {
                problems.push(problem(
                    "submarine_class_ids",
                    format!("class `{class}` has no id pinned"),
                ));
            }
        }
        for (id, class) in &sub.class_ids {
            if !derived.contains(class.as_str()) {
                problems.push(problem(
                    "submarine_class_ids",
                    format!("id {id} is pinned to `{class}`, which is not a [[submarine]] entry"),
                ));
            }
        }
        let torpedoes: BTreeSet<&str> = sub.torpedoes.iter().map(String::as_str).collect();
        let pinned: BTreeSet<&str> = sub.torpedo_ids.values().map(String::as_str).collect();
        for weapon in &torpedoes {
            if !pinned.contains(weapon) {
                problems.push(problem(
                    "submarine_torpedo_ids",
                    format!("torpedo `{weapon}` has no id pinned"),
                ));
            }
        }
        for (id, weapon) in &sub.torpedo_ids {
            if !torpedoes.contains(weapon.as_str()) {
                problems.push(problem(
                    "submarine_torpedo_ids",
                    format!("id {id} is pinned to `{weapon}`, which is not in `torpedoes`"),
                ));
            }
        }
        problems
    }

    fn class_id(sub: &SubmarineList, class: &str) -> u32 {
        sub.class_ids
            .iter()
            .find(|(_, c)| c.as_str() == class)
            .map_or(0, |(id, _)| *id)
    }

    /// A whole number above zero, or a problem naming the figure.
    fn whole_positive(
        w: &mut Work,
        got: &crate::derive::Got<Dec>,
        attribute: &str,
        figure: &str,
        class: &str,
    ) -> Option<i64> {
        if got.value.has_fraction() || !got.value.is_positive() {
            w.fail(
                Kind::BadValue,
                figure,
                format!(
                    "{attribute} of class {class} is {}: it must be a whole number above 0 (decision 0218, rule 28)",
                    got.value
                ),
            );
            return None;
        }
        Some(got.value.whole())
    }

    fn derive_submarine_class(
        &self,
        sub: &SubmarineList,
        entry: &SubmarineEntry,
    ) -> Result<ClassResult, Vec<Problem>> {
        let class = entry.class.as_str();
        let mut w = Work {
            subject: class,
            problems: Vec::new(),
            used: Vec::new(),
            estimated: Vec::new(),
        };
        let Some(info) = self.classes.get(class) else {
            w.fail(
                Kind::NoClass,
                "class",
                "no such class key in ship_classes.csv".to_string(),
            );
            return Err(w.problems);
        };
        if info.kind_type != SUBMARINE_TYPE {
            w.fail(
                Kind::List,
                "class",
                format!(
                    "ship_classes.csv:{} is of the type `{}`, not submarine",
                    info.row_id, info.kind_type
                ),
            );
            return Err(w.problems);
        }
        let keys = ("", class);

        let tubes = self.fact(&mut w, keys, TUBES, "salvo");
        let salvo = tubes
            .as_ref()
            .and_then(|got| Self::whole_positive(&mut w, got, TUBES, "salvo", class));
        let carried = self.fact(&mut w, keys, CARRIED, "salvoes_carried");
        let carried_whole = carried
            .as_ref()
            .and_then(|got| Self::whole_positive(&mut w, got, CARRIED, "salvoes_carried", class));
        let salvoes = match (salvo, carried_whole) {
            (Some(salvo), Some(carried)) => {
                let whole = carried / salvo;
                if whole == 0 {
                    w.fail(
                        Kind::BadValue,
                        "salvoes_carried",
                        format!(
                            "{carried} torpedoes carried over a salvo of {salvo} tubes is under one salvo (decision 0218, rule 28)"
                        ),
                    );
                    None
                } else {
                    Some((whole, carried, salvo))
                }
            }
            _ => None,
        };
        let passage = self
            .fact(&mut w, keys, ESTIMABLE_ON_A_SUBMARINE, PASSAGE_SPEED)
            .and_then(|got| {
                if got.value.is_positive() {
                    Some(got)
                } else {
                    w.fail(
                        Kind::BadValue,
                        PASSAGE_SPEED,
                        format!(
                            "{ESTIMABLE_ON_A_SUBMARINE} of class {class} is {}: it must be above 0",
                            got.value
                        ),
                    );
                    None
                }
            });
        let torpedo = self.class_torpedo(sub, entry, &mut w);

        if !w.problems.is_empty() {
            return Err(w.problems);
        }
        let (tubes, carried, passage) = (
            tubes.expect("no problem"),
            carried.expect("no problem"),
            passage.expect("no problem"),
        );
        let (salvoes, carried_whole, salvo) = salvoes.expect("no problem");
        let (torpedo_id, torpedo_cites, torpedo_note, from_like, taken) =
            torpedo.expect("no problem");

        let with_later = |cites: &BTreeSet<Cite>, mut notes: Vec<String>| {
            notes.extend(self.later_notes(cites));
            notes
        };
        let excepted = |got: &crate::derive::Got<Dec>, notes: &mut Vec<String>| {
            if got.excepted {
                notes.push("rests on an exception".to_string());
            }
        };
        let mut notes = Vec::new();
        excepted(&tubes, &mut notes);
        let salvo_figure = fig(
            salvo.to_string(),
            tubes.cites.clone(),
            SALVO_RULE,
            with_later(&tubes.cites, notes),
            false,
        );
        let mut notes = Vec::new();
        let exact = carried.value.div(tubes.value);
        if let Some(exact) = exact.filter(|e| e.has_fraction()) {
            notes.push(format!(
                "{carried_whole} torpedoes over a salvo of {salvo}: exact {exact}, rounded down"
            ));
        }
        excepted(&carried, &mut notes);
        let cites: BTreeSet<Cite> = tubes.cites.union(&carried.cites).cloned().collect();
        let salvoes_figure = fig(
            salvoes.to_string(),
            cites.clone(),
            SALVOES_RULE,
            with_later(&cites, notes),
            false,
        );
        let estimate = w
            .estimated
            .iter()
            .find(|u| u.figure == PASSAGE_SPEED)
            .cloned();
        let passage_figure = match &estimate {
            Some(used) => fig(
                passage.value.to_string(),
                passage.cites.clone(),
                &format!(
                    "{}; the figure is it (a speed in knots; decision 0218, rule 29)",
                    used.method_words
                ),
                with_later(&passage.cites, vec![used.note.clone()]),
                true,
            ),
            None => {
                let mut notes = Vec::new();
                excepted(&passage, &mut notes);
                fig(
                    passage.value.to_string(),
                    passage.cites.clone(),
                    PASSAGE_RULE,
                    with_later(&passage.cites, notes),
                    false,
                )
            }
        };
        let torpedo_figure = fig(
            Self::pin_of(sub, &torpedo_id).to_string(),
            torpedo_cites.clone(),
            if from_like {
                TORPEDO_FROM_RULE
            } else {
                TORPEDO_RULE
            },
            with_later(&torpedo_cites, torpedo_note),
            from_like,
        );
        Ok(ClassResult {
            class: class.to_string(),
            name: info.name.clone(),
            id: Self::class_id(sub, class),
            figures: vec![salvo_figure, salvoes_figure, passage_figure, torpedo_figure],
            used: w.used,
            estimates: w.estimated,
            taken,
        })
    }

    fn pin_of(sub: &SubmarineList, weapon: &str) -> u32 {
        sub.torpedo_ids
            .iter()
            .find(|(_, k)| k.as_str() == weapon)
            .map_or(0, |(id, _)| *id)
    }

    /// The torpedo of a class: the weapon key, the rows cited, notes, whether
    /// it was taken from like boats, and for that case the line for the
    /// header.
    #[allow(clippy::type_complexity)]
    fn class_torpedo(
        &self,
        sub: &SubmarineList,
        entry: &SubmarineEntry,
        w: &mut Work,
    ) -> Option<(String, BTreeSet<Cite>, Vec<String>, bool, Option<String>)> {
        let class = entry.class.as_str();
        let own = self.index.rows(Table::ClassFacts, class, MODEL);
        let (weapon, cites, notes, from_like, taken) = match &entry.torpedo_from {
            None => {
                let read = match resolve(
                    own,
                    &format!("{MODEL} of class {class}"),
                    self.date,
                    &self.rules.exceptions,
                ) {
                    Ok(read) => read,
                    Err(mut failure) => {
                        if failure.kind == Kind::Missing {
                            failure.message.push_str(
                                "; a class whose torpedo no row states takes a torpedo_from entry (decision 0218, rule 29)",
                            );
                        }
                        w.failed("torpedo_id", failure);
                        return None;
                    }
                };
                let weapon = read.rows[0].value.clone();
                let (cites, excepted, _) = w.take(&read);
                let mut notes = Vec::new();
                if excepted {
                    notes.push("rests on an exception".to_string());
                }
                (weapon, cites, notes, false, None)
            }
            Some(from) => {
                let (cites, notes, taken) = self.check_torpedo_from(entry, from, w)?;
                (from.weapon.clone(), cites, notes, true, Some(taken))
            }
        };
        if !sub.torpedoes.contains(&weapon) {
            w.fail(
                Kind::List,
                "torpedo_id",
                format!(
                    "the torpedo of class {class}, `{weapon}`, is not in the list's `torpedoes`, so it has no id: add it there"
                ),
            );
            return None;
        }
        if self.weapons.get(&weapon).map(String::as_str) != Some("torpedo") {
            w.fail(
                Kind::BadValue,
                "torpedo_id",
                format!("`{weapon}`, the torpedo of class {class}, is not a weapon of kind torpedo in weapons.csv"),
            );
            return None;
        }
        Some((weapon, cites, notes, from_like, taken))
    }

    /// Checks a `torpedo_from` entry against the facts (decision 0218, rule
    /// 29) and says how it is noted. `None` after a problem.
    #[allow(clippy::type_complexity)]
    fn check_torpedo_from(
        &self,
        entry: &SubmarineEntry,
        from: &TorpedoFrom,
        w: &mut Work,
    ) -> Option<(BTreeSet<Cite>, Vec<String>, String)> {
        let class = entry.class.as_str();
        let mut good = true;
        let own = self.index.rows(Table::ClassFacts, class, MODEL);
        if !own.is_empty() {
            w.fail(
                Kind::Estimate,
                "torpedo_id",
                format!(
                    "a {MODEL} row is filed for class {class} ({}), so a row drives the entry out: take out torpedo_from (decision 0218, rule 29)",
                    own.iter().map(Row::cite).collect::<Vec<_>>().join(", ")
                ),
            );
            good = false;
        }
        if let Some(group) = &from.group {
            match self.classes.get(group) {
                Some(info) if info.kind_type == SUBMARINE_TYPE && group != class => {}
                Some(info) if info.kind_type == SUBMARINE_TYPE => {
                    w.fail(
                        Kind::List,
                        "torpedo_id",
                        format!("torpedo_from names `{group}` as the group, which is the class itself: a group is another class"),
                    );
                    good = false;
                }
                Some(info) => {
                    w.fail(
                        Kind::List,
                        "torpedo_id",
                        format!(
                            "torpedo_from names `{group}` as the group, a class of the type `{}`, not submarine",
                            info.kind_type
                        ),
                    );
                    good = false;
                }
                None => {
                    w.fail(
                        Kind::NoClass,
                        "torpedo_id",
                        format!("torpedo_from names `{group}` as the group: no such class key in ship_classes.csv"),
                    );
                    good = false;
                }
            }
        }
        if self.weapons.get(&from.weapon).map(String::as_str) != Some("torpedo") {
            w.fail(
                Kind::BadValue,
                "torpedo_id",
                format!(
                    "torpedo_from names weapon `{}`, which is not a weapon of kind torpedo in weapons.csv",
                    from.weapon
                ),
            );
            good = false;
        }
        let whose = from.group.as_deref().unwrap_or(class);
        let mut cites = BTreeSet::new();
        let mut said = Vec::new();
        for cited in &from.rows {
            let cite = format!("{}:{}", cited.file, cited.id);
            if cited.file == "ship_class_facts.csv" {
                let Some(row) = self.index.class_fact(cited.id) else {
                    w.fail(
                        Kind::List,
                        "torpedo_id",
                        format!(
                            "torpedo_from cites {cite}, which is not a row of ship_class_facts.csv"
                        ),
                    );
                    good = false;
                    continue;
                };
                if row.name != MODEL || row.entity != whose || row.value != from.weapon {
                    w.fail(
                        Kind::List,
                        "torpedo_id",
                        format!(
                            "torpedo_from cites {cite}, which says {} {} of {}: it must be a {MODEL} row of {whose} whose value is {}",
                            row.name, row.value, row.entity, from.weapon
                        ),
                    );
                    good = false;
                    continue;
                }
                said.push(format!(
                    "{cite} ({}; {whose} {MODEL} {})",
                    row.standing, row.value
                ));
            } else {
                let Some(event) = self.events.get(&cited.id.to_string()) else {
                    w.fail(
                        Kind::List,
                        "torpedo_id",
                        format!("torpedo_from cites {cite}, which is not a row of engagement_events.csv"),
                    );
                    good = false;
                    continue;
                };
                said.push(format!(
                    "{cite} (time {}; actor {}; means {}; {})",
                    event.time, event.actor, event.means, event.standing
                ));
            }
            cites.insert((cited.file.clone(), u64::try_from(cited.id).unwrap_or(0)));
        }
        if !good {
            return None;
        }
        let method = match &from.group {
            None => "taken from the boats of her group".to_string(),
            Some(group) => format!("taken from the group nearest in date, {group}"),
        };
        let note = format!(
            "{method}: {}; rows cited: {}; entry by {}, {}; reason: {}",
            from.weapon,
            said.join(", "),
            from.by,
            from.date,
            one_line(&from.reason)
        );
        let taken = format!(
            "{class} torpedo_id: {method}: {} ({})",
            from.weapon,
            from.rows
                .iter()
                .map(|r| format!("{}:{}", r.file, r.id))
                .collect::<Vec<_>>()
                .join(", ")
        );
        Some((cites, vec![note], taken))
    }

    fn derive_torpedo(
        &self,
        sub: &SubmarineList,
        weapon: &str,
    ) -> Result<TorpedoResult, Vec<Problem>> {
        let mut w = Work {
            subject: weapon,
            problems: Vec::new(),
            used: Vec::new(),
            estimated: Vec::new(),
        };
        if self.weapons.get(weapon).map(String::as_str) != Some("torpedo") {
            w.fail(
                Kind::List,
                "torpedoes",
                "not a weapon of kind torpedo in weapons.csv".to_string(),
            );
            return Err(w.problems);
        }
        let read = match resolve(
            self.index.rows(Table::WeaponFacts, weapon, WARHEAD),
            &format!("{WARHEAD} of weapon {weapon}"),
            self.date,
            &self.rules.exceptions,
        ) {
            Ok(read) => read,
            Err(failure) => {
                w.failed("damage", failure);
                return Err(w.problems);
            }
        };
        let warhead = match crate::derive::number(&read, WARHEAD) {
            Ok(value) => value,
            Err(failure) => {
                w.failed("damage", failure);
                return Err(w.problems);
            }
        };
        let (cites, excepted, _) = w.take(&read);
        let Some(damage) = warhead.mul(self.rules.torpedo_damage_per_warhead_kg) else {
            w.failed("damage", crate::derive::overflow("the torpedo's damage"));
            return Err(w.problems);
        };
        let mut notes = Vec::new();
        if excepted {
            notes.push("rests on an exception".to_string());
        }
        notes.extend(self.later_notes(&cites));
        Ok(TorpedoResult {
            name: self.weapon_names.get(weapon).cloned().unwrap_or_default(),
            id: Self::pin_of(sub, weapon),
            damage: fig(damage.to_string(), cites, DAMAGE_RULE, notes, false),
            used: w.used,
        })
    }
}

// ------------------------------------------------------------------ rendering

fn facts_cell(cites: &BTreeSet<Cite>) -> String {
    cites
        .iter()
        .map(|(file, id)| format!("{file}:{id}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn header(lines: &[String]) -> String {
    lines.iter().map(|l| format!("# {l}\n")).collect()
}

/// The exceptions used, deduplicated and in rules order, as a header block.
fn exception_block(
    title: &str,
    used: &BTreeMap<(usize, String, u64), Row>,
    rules: &Rules,
) -> Vec<String> {
    if used.is_empty() {
        return vec![format!("{title}: none")];
    }
    let mut lines = vec![format!("{title}:")];
    for ((index, _, _), row) in used {
        let e: &Exception = &rules.exceptions[*index];
        lines.push(exception_line(e, row));
    }
    lines
}

fn used_of<'a>(
    uses: impl Iterator<Item = &'a Vec<(usize, Row)>>,
) -> BTreeMap<(usize, String, u64), Row> {
    let mut used = BTreeMap::new();
    for list in uses {
        for (index, row) in list {
            used.insert((*index, row.file.clone(), row.cite_key().1), row.clone());
        }
    }
    used
}

/// The four files of a list's submarine part, as text.
pub(crate) fn render(
    list: &List,
    list_name: &str,
    rules: &Rules,
    sub: &SubmarineList,
    derived: &Derived,
    idle_later: &[String],
) -> SubmarineFiles {
    let class_name = Path::new(&sub.output)
        .file_name()
        .map_or_else(String::new, |n| n.to_string_lossy().into_owned());
    let torpedo_name = Path::new(&sub.torpedo_output)
        .file_name()
        .map_or_else(String::new, |n| n.to_string_lossy().into_owned());
    let class_sources_name = companion_names(&sub.output).0;
    let torpedo_sources_name = companion_names(&sub.torpedo_output).0;
    let list_line = format!(
        "List: {list_name}: {} (as of {})",
        one_line(&list.name),
        list.date_text
    );
    let constant = format!(
        "  torpedo_damage_per_warhead_kg = {}",
        rules.torpedo_damage_per_warhead_kg
    );

    // The class file.
    let mut lines = vec![
        "GENERATED by oob-derive from the facts in data/oob. Do not edit.".to_string(),
        list_line.clone(),
        format!("Rules: {}", list.rules),
        format!("Companions: {class_sources_name} (the rule and fact rows behind every figure); the torpedo file is {torpedo_name}"),
        "Written rules (decision 0218, rule 28): salvo is the class's torpedo_tubes_bow; salvoes_carried is torpedoes_carried / salvo, rounded down; passage_speed_kn is endurance_speed_kn; torpedo_id is the id the list pins for the class's torpedo_model.".to_string(),
        "Written rule (decision 0218, rule 29): a figure still missing is estimated from like boats by decision 0207 ([[estimate]]) and marked; a class whose torpedo no row states takes the torpedo stated for the boats of her group, or of the group nearest in date, by the list's torpedo_from entry, and is marked. The column `estimated` names the marked figures of each class.".to_string(),
        "Constants, MODEL PARAMETERS AND PLACEHOLDERS, not history (decision 0020, 8; read by the torpedo file):".to_string(),
        constant.clone(),
        "Counts are whole numbers. The speed is an exact decimal of up to six places, trailing zeros trimmed; a division truncates toward zero at the sixth place.".to_string(),
    ];
    let class_used = used_of(derived.classes.iter().map(|c| &c.used));
    lines.extend(exception_block(
        "Exceptions used (decision 0020, 3)",
        &class_used,
        rules,
    ));
    lines.extend(
        idle_lines(idle_later)
            .into_iter()
            .map(|l| l.trim_end().to_string()),
    );
    let mut marked: Vec<String> = Vec::new();
    for class in &derived.classes {
        for used in &class.estimates {
            marked.push(format!("  {} ({})", one_line(&used.line), used.figure));
        }
        if let Some(taken) = &class.taken {
            marked.push(format!("  {}", one_line(taken)));
        }
    }
    if marked.is_empty() {
        lines.push(
            "Estimated or taken from like boats, not read (decision 0207; decision 0218, rule 29): none"
                .to_string(),
        );
    } else {
        lines.push("Estimated or taken from like boats, not read (decision 0207; decision 0218, rule 29): a figure worked by a written rule from the rows of like boats, or a torpedo taken from the boats of her group, where no row of the class states it. It is never evidence:".to_string());
        lines.extend(marked.iter().cloned());
    }
    if sub.left_out_classes.is_empty() {
        lines.push("Left out (decision 0218, rule 1): none".to_string());
    } else {
        lines.push("Left out (decision 0218, rule 1), each class of the type submarine in ship_classes.csv that is not derived, with its reason:".to_string());
        for (class, reason) in &sub.left_out_classes {
            lines.push(format!("  {class}: {}", one_line(reason)));
        }
    }
    let mut class_text = header(&lines);
    let mut body = Vec::new();
    let mut source_rows = Vec::new();
    for class in &derived.classes {
        let marked: Vec<&str> = CLASS_FIGURES
            .iter()
            .zip(&class.figures)
            .filter(|(_, f)| f.marked)
            .map(|(name, _)| *name)
            .collect();
        let mut cells = vec![class.id.to_string(), class.name.clone()];
        cells.extend(class.figures.iter().map(|f| f.value.clone()));
        cells.push(marked.join(" "));
        body.push(cells);
        for (name, figure) in CLASS_FIGURES.iter().zip(&class.figures) {
            source_rows.push(vec![
                class.id.to_string(),
                (*name).to_string(),
                figure.value.clone(),
                figure.rule.clone(),
                facts_cell(&figure.cites),
                figure.note.clone(),
            ]);
        }
    }
    let mut class_header: Vec<&str> = vec!["id", "name"];
    class_header.extend(CLASS_FIGURES);
    class_header.push(ESTIMATED_COLUMN);
    class_text.push_str(&csv_text(&class_header, &body));
    let mut class_sources = header(&[
        "GENERATED by oob-derive. Do not edit.".to_string(),
        format!("For each class row of {class_name} and each of its figures: the value as written there, the rule, the fact rows read, and a note."),
        "facts: file:id of each row read, files in data/oob. Constants and fixed values cite no row.".to_string(),
        "The note gives the exact value where salvoes_carried was rounded down, says where a figure rests on an exception, a row of a later date, an estimate (the working) or a torpedo taken from like boats (the method, each row cited with what it says, and the entry's by, date and reason).".to_string(),
    ]);
    class_sources.push_str(&csv_text(
        &["class_id", "figure", "value", "rule", "facts", "note"],
        &source_rows,
    ));

    // The torpedo file.
    let mut lines = vec![
        "GENERATED by oob-derive from the facts in data/oob. Do not edit.".to_string(),
        list_line,
        format!("Rules: {}", list.rules),
        format!("Companion: {torpedo_sources_name} (the rule and fact rows behind every figure)"),
        "Written rule (decision 0218, rule 28): the damage of a hit is the torpedo's warhead_kg x torpedo_damage_per_warhead_kg.".to_string(),
        "Constant, MODEL PARAMETER AND PLACEHOLDER, not history (decision 0020, 8):".to_string(),
        constant,
        "damage is an exact decimal of up to six places, trailing zeros trimmed.".to_string(),
    ];
    let torpedo_used = used_of(derived.torpedoes.iter().map(|t| &t.used));
    lines.extend(exception_block(
        "Exceptions used (decision 0020, 3)",
        &torpedo_used,
        rules,
    ));
    if sub.left_out_weapons.is_empty() {
        lines.push("Left out (decision 0218, rule 1): none".to_string());
    } else {
        lines.push("Left out (decision 0218, rule 1), each torpedo of weapons.csv not derived, with its reason:".to_string());
        for (weapon, reason) in &sub.left_out_weapons {
            lines.push(format!("  {weapon}: {}", one_line(reason)));
        }
    }
    let mut torpedo_text = header(&lines);
    let body: Vec<Vec<String>> = derived
        .torpedoes
        .iter()
        .map(|t| vec![t.id.to_string(), t.name.clone(), t.damage.value.clone()])
        .collect();
    torpedo_text.push_str(&csv_text(&["id", "name", "damage"], &body));
    let source_rows: Vec<Vec<String>> = derived
        .torpedoes
        .iter()
        .map(|t| {
            vec![
                t.id.to_string(),
                "damage".to_string(),
                t.damage.value.clone(),
                t.damage.rule.clone(),
                facts_cell(&t.damage.cites),
                t.damage.note.clone(),
            ]
        })
        .collect();
    let mut torpedo_sources = header(&[
        "GENERATED by oob-derive. Do not edit.".to_string(),
        format!("For each torpedo of {torpedo_name} and its figure: the value as written there, the rule, the fact rows read, and a note."),
        "facts: file:id of each row read, files in data/oob. Constants cite no row.".to_string(),
        "The note says where a figure rests on an exception or a row of a later date.".to_string(),
    ]);
    torpedo_sources.push_str(&csv_text(
        &["torpedo_id", "figure", "value", "rule", "facts", "note"],
        &source_rows,
    ));
    SubmarineFiles {
        class_csv: class_text,
        class_sources_csv: class_sources,
        torpedo_csv: torpedo_text,
        torpedo_sources_csv: torpedo_sources,
    }
}

/// The check's count of what is estimated or taken from like boats, for the
/// lines `oob-derive` prints (decision 0207, rule 13; decision 0218, rule
/// 29): a first line with the count and the classes, then a line each.
pub(crate) fn estimated_lines(derived: &Derived) -> Vec<String> {
    let mut lines = Vec::new();
    let mut classes = BTreeSet::new();
    for class in &derived.classes {
        for used in &class.estimates {
            classes.insert(class.class.as_str());
            lines.push(one_line(&used.line));
        }
        if let Some(taken) = &class.taken {
            classes.insert(class.class.as_str());
            lines.push(one_line(&format!(
                "taken from like boats (decision 0218, rule 29): {taken}"
            )));
        }
    }
    if lines.is_empty() {
        return lines;
    }
    let n = lines.len();
    let mut all = vec![format!(
        "{n} {} estimated or taken from like boats (decision 0207; decision 0218, rule 29), for {}",
        if n == 1 { "figure" } else { "figures" },
        classes.into_iter().collect::<Vec<_>>().join(", ")
    )];
    all.extend(lines);
    all
}
