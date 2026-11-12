//! Saved games, `replay` and `hash` (decisions 6 to 9 of 0043).
//!
//! A saved game names the scenario file and records its hash and the hash of
//! every data file the scenario names, the seed, the orders of each day as
//! they were submitted (with their origin) and the daily hashes of the run
//! that saved it. It holds no state: the starting state is the scenario's.
//! Every quantity is a whole number, a decimal string or a plain string, so
//! the same run gives the same bytes. The hash of a file is the FNV-1a hash
//! the simulation uses for its state, over the file's bytes.
//!
//! Decision 11 of 0050 adds `"seats"`: who held each side, `"player"` or
//! `"script"`. The orders are those handed in to the engine, for both sides,
//! whoever gave them, so a game played by a player replays with none. The
//! key is an addition: a file without it (every game saved before) is read as
//! both seats held by the script, and the version is unchanged.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use sim_agent::{order_from_json, order_to_json, unit_text, SeatKind};
use sim_core::hash::StateHasher;
use sim_core::ids::IntentId;
use sim_core::orders::{Issuer, OrderRequest, Origin};
use sim_core::state::Side;
use sim_data::Scenario;

pub const FORMAT: &str = "flagplot-saved-game";
pub const VERSION: u32 = 1;

pub type Script = BTreeMap<u32, Vec<(Side, OrderRequest)>>;

/// What a run needs to remember to be saved: read before it starts, since the
/// scenario spends its script as it runs.
pub struct Remembered {
    script: Script,
    files: Vec<(String, String)>,
}

pub fn hash_hex(bytes: &[u8]) -> String {
    let mut hasher = StateHasher::new();
    hasher.bytes(bytes);
    format!("{:016x}", hasher.finish())
}

fn file_hash(path: &Path) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(hash_hex(&bytes))
}

/// What a run was made on, written into every file that holds results (a
/// batch summary, a replay), so that two files can be told to come from the
/// same inputs before their numbers are compared (decision 0057). It does not
/// name the engine's code: `batch --label` carries that, as the caller knows
/// it.
#[derive(serde::Serialize)]
pub struct Inputs {
    /// The version of the state hash's layout, which changes with the state.
    pub state_version: String,
    /// The hash of the scenario file.
    pub scenario_hash: String,
    /// Every data file the scenario names, with its hash, in the order the
    /// scenario names them.
    pub data: Vec<InputFile>,
}

#[derive(serde::Serialize)]
pub struct InputFile {
    pub file: String,
    pub hash: String,
}

/// The inputs of a run of `scenario`, loaded from `scenario_path`.
pub fn inputs(scenario_path: &str, scenario: &Scenario) -> Result<Inputs, String> {
    let mut data = Vec::new();
    for path in scenario.data_files() {
        data.push(InputFile {
            file: path.to_string_lossy().into_owned(),
            hash: file_hash(path)?,
        });
    }
    Ok(Inputs {
        state_version: String::from_utf8_lossy(sim_core::hash::STATE_VERSION).into_owned(),
        scenario_hash: file_hash(Path::new(scenario_path))?,
        data,
    })
}

/// Notes the scripted orders and the hash of every data file, before the run.
pub fn remember(scenario: &Scenario) -> Result<Remembered, String> {
    let mut files = Vec::new();
    for path in scenario.data_files() {
        files.push((path.to_string_lossy().into_owned(), file_hash(path)?));
    }
    Ok(Remembered {
        script: scenario.scripted_orders().clone(),
        files,
    })
}

/// The seats of a game with no player: both held by the script.
pub fn script_seats() -> BTreeMap<Side, SeatKind> {
    Side::ALL
        .into_iter()
        .map(|s| (s, SeatKind::Script))
        .collect()
}

impl Remembered {
    /// The scenario's own script, as it will be submitted.
    pub fn script(&self) -> &Script {
        &self.script
    }
}

/// Writes a saved game. `orders` are the orders as handed in to the engine,
/// by day, for both sides; `hashes` are `(day, hash)` for each day resolved.
pub fn write(
    file: &str,
    scenario_path: &str,
    seed: u64,
    remembered: &Remembered,
    orders: &Script,
    seats: &BTreeMap<Side, SeatKind>,
    hashes: &[(u32, u64)],
) -> Result<(), String> {
    let scenario_hash = file_hash(Path::new(scenario_path))?;
    let mut order_list = Vec::new();
    for (day, list) in orders {
        for (side, request) in list {
            order_list.push(json!({
                "day": day,
                "side": match side {
                    Side::Allied => "allied",
                    Side::Japanese => "japanese",
                },
                "unit": unit_text(&request.kind),
                "order": order_to_json(&request.kind),
                "origin": {
                    "issuer": match request.origin.issuer {
                        Issuer::Player => "player",
                        Issuer::Staff => "staff",
                        Issuer::LocalCommander => "local_commander",
                    },
                    "intent": request.origin.intent.map(|i| i.raw()),
                },
            }));
        }
    }
    let value = json!({
        "format": FORMAT,
        "version": VERSION,
        "scenario": { "file": scenario_path, "hash": scenario_hash },
        "data_files": remembered
            .files
            .iter()
            .map(|(path, hash)| json!({ "file": path, "hash": hash }))
            .collect::<Vec<_>>(),
        "seed": seed,
        "seats": {
            "allied": seats.get(&Side::Allied).copied().unwrap_or(SeatKind::Script).name(),
            "japanese": seats.get(&Side::Japanese).copied().unwrap_or(SeatKind::Script).name(),
        },
        "orders": order_list,
        "hashes": hashes
            .iter()
            .map(|(day, hash)| json!({ "day": day, "hash": format!("{hash:016x}") }))
            .collect::<Vec<_>>(),
    });
    let mut text = serde_json::to_string_pretty(&value).map_err(|e| e.to_string())?;
    text.push('\n');
    fs::write(file, text).map_err(|e| format!("{file}: {e}"))
}

fn text<'a>(value: &'a Value, key: &str) -> Result<&'a str, String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("\"{key}\" is missing or not text"))
}

fn small(value: &Value, key: &str) -> Result<u32, String> {
    value
        .get(key)
        .and_then(Value::as_i64)
        .ok_or_else(|| format!("\"{key}\" is missing or not a whole number"))
        .and_then(|n| u32::try_from(n).map_err(|_| format!("\"{key}\" is out of range")))
}

/// A saved game, read back.
pub struct Saved {
    pub scenario_file: String,
    scenario_hash: String,
    data_files: Vec<(String, String)>,
    pub seed: u64,
    pub seats: BTreeMap<Side, SeatKind>,
    pub script: Script,
    pub hashes: Vec<(u32, String)>,
}

pub fn read(file: &str) -> Result<Saved, String> {
    let bad = |message: String| format!("{file}: {message}");
    let text_in = fs::read_to_string(file).map_err(|e| bad(e.to_string()))?;
    let value: Value = serde_json::from_str(&text_in).map_err(|e| bad(e.to_string()))?;
    if value.get("format").and_then(Value::as_str) != Some(FORMAT) {
        return Err(bad(format!(
            "not a Flag Plot saved game (format \"{FORMAT}\")"
        )));
    }
    if value.get("version").and_then(Value::as_u64) != Some(u64::from(VERSION)) {
        return Err(bad(format!("saved game version is not {VERSION}")));
    }
    let scenario = value
        .get("scenario")
        .ok_or_else(|| bad("no scenario".into()))?;
    let scenario_file = text(scenario, "file").map_err(&bad)?.to_string();
    let scenario_hash = text(scenario, "hash").map_err(&bad)?.to_string();
    let mut data_files = Vec::new();
    for entry in value
        .get("data_files")
        .and_then(Value::as_array)
        .ok_or_else(|| bad("no data_files".into()))?
    {
        data_files.push((
            text(entry, "file").map_err(&bad)?.to_string(),
            text(entry, "hash").map_err(&bad)?.to_string(),
        ));
    }
    let seed = value
        .get("seed")
        .and_then(Value::as_u64)
        .ok_or_else(|| bad("no seed".into()))?;
    let mut seats = script_seats();
    if let Some(held) = value.get("seats") {
        for side in Side::ALL {
            let name = if side == Side::Allied {
                "allied"
            } else {
                "japanese"
            };
            let kind = text(held, name).map_err(&bad)?;
            let kind = SeatKind::parse(kind)
                .ok_or_else(|| bad(format!("unknown seat holder \"{kind}\"")))?;
            seats.insert(side, kind);
        }
    }
    let mut script: Script = BTreeMap::new();
    for entry in value
        .get("orders")
        .and_then(Value::as_array)
        .ok_or_else(|| bad("no orders".into()))?
    {
        let day = small(entry, "day").map_err(&bad)?;
        let side = match text(entry, "side").map_err(&bad)? {
            "allied" => Side::Allied,
            "japanese" => Side::Japanese,
            other => return Err(bad(format!("unknown side \"{other}\""))),
        };
        let unit = text(entry, "unit").map_err(&bad)?;
        let order = entry
            .get("order")
            .ok_or_else(|| bad("an order has no order".into()))?;
        let kind =
            order_from_json(unit, order).map_err(|e| bad(format!("order on day {day}: {e}")))?;
        let origin = entry
            .get("origin")
            .ok_or_else(|| bad("an order has no origin".into()))?;
        let issuer = match text(origin, "issuer").map_err(&bad)? {
            "player" => Issuer::Player,
            "staff" => Issuer::Staff,
            "local_commander" => Issuer::LocalCommander,
            other => return Err(bad(format!("unknown issuer \"{other}\""))),
        };
        let intent = match origin.get("intent") {
            Some(Value::Null) | None => None,
            Some(n) => Some(IntentId::new(
                n.as_u64()
                    .and_then(|n| u32::try_from(n).ok())
                    .ok_or_else(|| bad("an intent is not a number".into()))?,
            )),
        };
        script.entry(day).or_default().push((
            side,
            OrderRequest {
                kind,
                origin: Origin { issuer, intent },
            },
        ));
    }
    let mut hashes = Vec::new();
    for entry in value
        .get("hashes")
        .and_then(Value::as_array)
        .ok_or_else(|| bad("no hashes".into()))?
    {
        hashes.push((
            small(entry, "day").map_err(&bad)?,
            text(entry, "hash").map_err(&bad)?.to_string(),
        ));
    }
    Ok(Saved {
        scenario_file,
        scenario_hash,
        data_files,
        seed,
        seats,
        script,
        hashes,
    })
}

impl Saved {
    /// Refuses a scenario file or data file that has changed since the save.
    pub fn check_files(&self, scenario: &Scenario) -> Result<(), String> {
        let now = file_hash(Path::new(&self.scenario_file))?;
        if now != self.scenario_hash {
            return Err(format!(
                "the scenario file {} has changed since the game was saved (saved hash {}, now {now})",
                self.scenario_file, self.scenario_hash
            ));
        }
        let current: Vec<PathBuf> = scenario.data_files().to_vec();
        let saved: Vec<&str> = self.data_files.iter().map(|(f, _)| f.as_str()).collect();
        let names: Vec<String> = current
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect();
        if names.iter().map(String::as_str).collect::<Vec<_>>() != saved {
            return Err(format!(
                "the scenario {} now names different data files than when the game was saved",
                self.scenario_file
            ));
        }
        for (file, saved_hash) in &self.data_files {
            let now = file_hash(Path::new(file))?;
            if &now != saved_hash {
                return Err(format!(
                    "the data file {file} has changed since the game was saved (saved hash {saved_hash}, now {now})"
                ));
            }
        }
        Ok(())
    }
}
