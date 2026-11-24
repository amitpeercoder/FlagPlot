//! The pack of a scenario: every file a load of it reads, by the path asked
//! for, with its text, written as one JSON object (decision 0229, rulings 9
//! and 10). It is made by `sim-data`, by loading the scenario through a
//! source that notes what it is asked for, and never by hand.
//!
//! The object has two keys: `versions`, the rules version, the state version
//! and the doctrine version of the program that made it, and `files`, each
//! path with its text. Paths are as `sim-cli` run from the top of the
//! repository names them, with their `..` parts left in.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use sim_data::{Disk, LoadOptions, Pack, Scenario};

/// The three versions a pack is made under.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Versions {
    pub rules: u32,
    pub state: String,
    pub doctrine: String,
}

/// The versions of this program.
pub fn versions() -> Versions {
    Versions {
        rules: sim_core::world::RULES_VERSION,
        state: crate::save::engine_state_version(),
        doctrine: sim_staff::DOCTRINE_VERSION.to_string(),
    }
}

/// The pack of the scenarios named, read from the disk: the files each load
/// reads, together.
pub fn pack_of(scenarios: &[&str], options: &LoadOptions) -> Result<Pack, String> {
    let mut pack = Pack::new();
    for scenario in scenarios {
        let one =
            Scenario::pack_of(&Disk, Path::new(scenario), options).map_err(|e| e.to_string())?;
        pack.merge(&one).map_err(|path| {
            format!(
                "{} is read as two different files by the scenarios named",
                path.display()
            )
        })?;
    }
    Ok(pack)
}

/// The text of a pack file: the versions of this program and the files.
pub fn pack_text(pack: &Pack) -> Result<String, String> {
    let versions = versions();
    let files: BTreeMap<String, &String> = pack
        .files()
        .iter()
        .map(|(path, text)| (path.to_string_lossy().into_owned(), text))
        .collect();
    let value = json!({
        "versions": {
            "rules": versions.rules,
            "state": versions.state,
            "doctrine": versions.doctrine,
        },
        "files": files,
    });
    serde_json::to_string(&value).map_err(|e| e.to_string())
}

/// A pack file read back: the versions it was made under and the pack.
pub fn read_pack(text: &str) -> Result<(Versions, Pack), String> {
    let value: Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
    let versions = value
        .get("versions")
        .ok_or_else(|| "the pack has no versions".to_string())?;
    let rules = versions
        .get("rules")
        .and_then(Value::as_u64)
        .and_then(|n| u32::try_from(n).ok())
        .ok_or_else(|| "the pack's rules version is missing or not a number".to_string())?;
    let word = |key: &str| {
        versions
            .get(key)
            .and_then(Value::as_str)
            .map(str::to_string)
            .ok_or_else(|| format!("the pack's {key} version is missing or not text"))
    };
    let versions = Versions {
        rules,
        state: word("state")?,
        doctrine: word("doctrine")?,
    };
    let files = value
        .get("files")
        .and_then(Value::as_object)
        .ok_or_else(|| "the pack has no files".to_string())?;
    let mut held = BTreeMap::new();
    for (path, text) in files {
        let text = text
            .as_str()
            .ok_or_else(|| format!("the file {path} is not text"))?;
        held.insert(PathBuf::from(path), text.to_string());
    }
    Ok((versions, Pack::from_files(held)))
}

/// `sim-cli pack SCENARIO... --out FILE`: writes the pack of the scenarios
/// named as one file.
pub fn command(args: &[String], usage: &str, options: &LoadOptions) -> Result<(), String> {
    let mut scenarios: Vec<&str> = Vec::new();
    let mut out: Option<&str> = None;
    let mut words = args.iter().map(String::as_str);
    while let Some(word) = words.next() {
        match word {
            "--out" => {
                out = Some(
                    words
                        .next()
                        .ok_or_else(|| format!("--out needs a file\n\n{usage}"))?,
                )
            }
            flag if flag.starts_with('-') => {
                return Err(format!("unknown option \"{flag}\"\n\n{usage}"))
            }
            scenario => scenarios.push(scenario),
        }
    }
    if scenarios.is_empty() {
        return Err(format!("pack needs at least one scenario\n\n{usage}"));
    }
    let out = out.ok_or_else(|| format!("pack needs --out FILE\n\n{usage}"))?;
    let pack = pack_of(&scenarios, options)?;
    let text = pack_text(&pack)?;
    std::fs::write(out, text).map_err(|e| format!("{out}: {e}"))?;
    println!(
        "wrote {out}: {} files from {} scenario{}",
        pack.files().len(),
        scenarios.len(),
        if scenarios.len() == 1 { "" } else { "s" }
    );
    Ok(())
}
