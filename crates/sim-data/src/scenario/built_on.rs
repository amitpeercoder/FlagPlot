//! A scenario built on another (decision 0234, "The format as planned"):
//! reading the chain of files a scenario names with `built_on` and laying
//! each table over the one below it.
//!
//! Nothing here knows what a ship or a base is. The lists of the scenario
//! format are named once, below, by how an overlay lays its rows on them;
//! the result is a table that the one loader of scenarios reads as it reads
//! a file written out in full.

use super::Scenario;
use crate::error::DataError;
use crate::source::{read_text, Source};
use std::path::{Component, Path, PathBuf};
use toml::{Table, Value};

/// The most files a chain may have, the file asked for among them (item 2).
const MOST_FILES: usize = 8;

/// The lists of units by id (item 5): a row of an overlay with the id of a
/// row below changes that row, and any other row is put after the rows
/// below.
const BY_ID: [&str; 7] = [
    "base",
    "ship",
    "task_force",
    "air_group",
    "land_unit",
    "submarine",
    "formation",
];

/// The other lists of tables (item 6): the overlay's rows are put after the
/// rows below.
const APPENDED: [&str; 14] = [
    "order",
    "command",
    "contact",
    "estimate",
    "replacement",
    "arrival",
    "supply_line",
    "men_replacement",
    "suspected_waters",
    "believed_force",
    "believed_intent",
    "believed_men",
    "commander",
    "island",
];

/// The keys that name a file (item 8). A key that names a file, added to
/// the scenario format later, is added here in the same change.
const FILE_KEYS: [&str; 18] = [
    "map",
    "land_links",
    "ship_classes",
    "port_sizes",
    "aircraft_types",
    "search",
    "surface_combat",
    "air_combat",
    "air_upkeep",
    "fatigue",
    "air_supply",
    "ground",
    "reports",
    "class_sides",
    "trial",
    "submarine_classes",
    "submarine_torpedoes",
    "submarines",
];

/// The files a scenario is built on, read: the file asked for first.
pub(super) struct Chain {
    /// The paths the files were read from, the file asked for first.
    pub(super) paths: Vec<PathBuf>,
    /// Each file's table, without its `built_on`, in the same order.
    tables: Vec<Table>,
    /// For each file, the path from the folder of the file asked for to its
    /// own folder, struck of `..` pairs (item 8). Empty for the first.
    prefixes: Vec<PathBuf>,
}

/// Takes the plain folder names followed by `..` out of `path`, and the
/// `.` parts, by its words alone.
pub(super) fn strike(path: &Path) -> PathBuf {
    let mut kept: Vec<Component> = Vec::new();
    for part in path.components() {
        match part {
            Component::CurDir => {}
            Component::ParentDir => {
                if matches!(kept.last(), Some(Component::Normal(_))) {
                    kept.pop();
                } else {
                    kept.push(part);
                }
            }
            other => kept.push(other),
        }
    }
    kept.iter().collect()
}

fn folder_of(path: &Path) -> &Path {
    path.parent().unwrap_or_else(|| Path::new("."))
}

fn kind_of(value: &Value) -> &'static str {
    match value {
        Value::String(_) => "a string",
        Value::Integer(_) => "an integer",
        Value::Float(_) => "a float",
        Value::Boolean(_) => "a boolean",
        Value::Datetime(_) => "a date",
        Value::Array(_) => "a list",
        Value::Table(_) => "a table",
    }
}

impl Chain {
    /// Reads the chain of the scenario whose text is `text`, as it would be
    /// found at `path`, through `source`. A file that states no `built_on`
    /// is a chain of one.
    pub(super) fn read(source: &dyn Source, text: &str, path: &Path) -> Result<Chain, DataError> {
        let mut paths: Vec<PathBuf> = vec![path.to_path_buf()];
        let mut tables: Vec<Table> = Vec::new();
        let mut prefixes: Vec<PathBuf> = vec![PathBuf::new()];
        let mut seen: Vec<PathBuf> = vec![strike(path)];
        let mut current_text = text.to_string();
        loop {
            let at = paths[paths.len() - 1].clone();
            let mut table: Table = toml::from_str(&current_text)
                .map_err(|e| DataError::in_file(&at, e.to_string()))?;
            let built_on = table.remove("built_on");
            tables.push(table);
            let Some(value) = built_on else { break };
            let Value::String(name) = value else {
                return Err(DataError::in_file(
                    &at,
                    format!(
                        "built_on names one file, as a string; it is {}",
                        kind_of(&value)
                    ),
                ));
            };
            if paths.len() >= MOST_FILES {
                return Err(DataError::in_file(
                    path,
                    format!(
                        "it is built on a chain of more than {MOST_FILES} files: {} names {name}",
                        at.display()
                    ),
                ));
            }
            let next = folder_of(&at).join(&name);
            if seen.contains(&strike(&next)) {
                return Err(DataError::in_file(
                    path,
                    format!(
                        "built on itself: {} names {}, which is already read",
                        at.display(),
                        next.display()
                    ),
                ));
            }
            current_text = source.read_text(&next).map_err(|message| {
                DataError::in_file(
                    &next,
                    format!("{message}; {} is built on it", path.display()),
                )
            })?;
            let below = strike(&prefixes[prefixes.len() - 1].join(folder_of(Path::new(&name))));
            prefixes.push(below);
            seen.push(strike(&next));
            paths.push(next);
        }
        Ok(Chain {
            paths,
            tables,
            prefixes,
        })
    }

    /// The scenario as one table: each file laid over the one below it.
    pub(super) fn laid(self) -> Result<Table, DataError> {
        let mut top: Vec<(Table, PathBuf, PathBuf)> = self
            .tables
            .into_iter()
            .zip(self.prefixes)
            .zip(self.paths)
            .map(|((table, prefix), path)| (table, prefix, path))
            .collect();
        let (table, prefix, _) = top.pop().expect("a chain has a file");
        let mut result = named_for_the_first_folder(table, &prefix);
        while let Some((table, prefix, path)) = top.pop() {
            let over = named_for_the_first_folder(table, &prefix);
            result = lay(over, result, &path)?;
        }
        Ok(result)
    }
}

/// The names of files `table` states, written for the folder of the file
/// asked for (item 8): `prefix` is the way from that folder to the table's
/// own.
fn named_for_the_first_folder(mut table: Table, prefix: &Path) -> Table {
    if prefix.as_os_str().is_empty() {
        return table;
    }
    let rewrite = |name: &str| -> String {
        if Path::new(name).is_absolute() {
            name.to_string()
        } else {
            strike(&prefix.join(name)).to_string_lossy().into_owned()
        }
    };
    for key in FILE_KEYS {
        match table.get_mut(key) {
            Some(Value::String(name)) => *name = rewrite(name),
            Some(Value::Array(names)) => {
                for name in names {
                    if let Value::String(name) = name {
                        *name = rewrite(name);
                    }
                }
            }
            _ => {}
        }
    }
    table
}

/// The rows of `value` as tables, or the error that says the list is not
/// one.
fn rows(value: Value, key: &str, path: &Path) -> Result<Vec<Table>, DataError> {
    let not_rows = |value: &Value| {
        DataError::in_file(
            path,
            format!(
                "{key} is a list of tables in a scenario; it is {}",
                kind_of(value)
            ),
        )
    };
    match value {
        Value::Array(items) => items
            .into_iter()
            .map(|item| match item {
                Value::Table(row) => Ok(row),
                other => Err(not_rows(&other)),
            })
            .collect(),
        other => Err(not_rows(&other)),
    }
}

/// Lays `over`, the table of the file at `path`, over `under`.
fn lay(over: Table, mut under: Table, path: &Path) -> Result<Table, DataError> {
    for (key, value) in over {
        let by_id = BY_ID.contains(&key.as_str());
        if !by_id && !APPENDED.contains(&key.as_str()) {
            // A plain key: the whole value takes the place of the one below
            // (item 4).
            under.insert(key, value);
            continue;
        }
        let new_rows = rows(value, &key, path)?;
        let mut kept = match under.remove(&key) {
            Some(below) => rows(below, &key, path)?,
            None => Vec::new(),
        };
        if !by_id {
            kept.extend(new_rows);
        } else {
            let mut stated: Vec<Value> = Vec::new();
            for (n, row) in new_rows.into_iter().enumerate() {
                let Some(id) = row.get("id").cloned() else {
                    return Err(DataError::in_file(
                        path,
                        format!(
                            "[[{key}]] row {} states no id: an overlay's rows of {key} are laid by id",
                            n + 1
                        ),
                    ));
                };
                if stated.contains(&id) {
                    return Err(DataError::in_file(
                        path,
                        format!("[[{key}]] has two rows of id {id}"),
                    ));
                }
                stated.push(id.clone());
                match kept.iter_mut().find(|r| r.get("id") == Some(&id)) {
                    Some(below) => {
                        for (name, value) in row {
                            below.insert(name, value);
                        }
                    }
                    None => kept.push(row),
                }
            }
        }
        under.insert(
            key,
            Value::Array(kept.into_iter().map(Value::Table).collect()),
        );
    }
    Ok(under)
}

/// Whether `text` could state `built_on`: a cheap test, so that a file that
/// cannot is read as it always was.
pub(super) fn may_state_built_on(text: &str) -> bool {
    text.contains("built_on")
}

impl Scenario {
    /// The scenario at `path`, read through `source`, as one TOML table
    /// (decision 0234, item 12): no `built_on`, and every file name written
    /// for the folder of `path`, so that the text, put in that folder or
    /// handed to [`Scenario::from_text`] with that path, loads as the file
    /// does. Builds no world and reads no file but the chain.
    pub fn resolved_text(source: &dyn Source, path: &Path) -> Result<String, DataError> {
        let text = read_text(source, path)?;
        Scenario::resolved_text_of(source, &text, path)
    }

    /// As [`Scenario::resolved_text`], for text that is not (or no longer)
    /// the file's own bytes, as it would be found at `path`.
    pub fn resolved_text_of(
        source: &dyn Source,
        text: &str,
        path: &Path,
    ) -> Result<String, DataError> {
        let table = Chain::read(source, text, path)?.laid()?;
        toml::to_string(&table).map_err(|e| DataError::in_file(path, e.to_string()))
    }

    /// The files read for the scenario at `path`, the one asked for first
    /// and then each file it is built on, nearest first (decision 0234,
    /// item 12). Reads no file but those.
    pub fn built_on_chain(source: &dyn Source, path: &Path) -> Result<Vec<PathBuf>, DataError> {
        let text = read_text(source, path)?;
        Scenario::built_on_chain_of(source, &text, path)
    }

    /// As [`Scenario::built_on_chain`], for text as it would be found at
    /// `path`.
    pub fn built_on_chain_of(
        source: &dyn Source,
        text: &str,
        path: &Path,
    ) -> Result<Vec<PathBuf>, DataError> {
        Ok(Chain::read(source, text, path)?.paths)
    }
}
