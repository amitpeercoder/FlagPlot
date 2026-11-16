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
//!
//! Version 3 (decision 0107, "As built") adds, beside the orders, `"commands"`
//! (the commander's commands handed in, by day, each as
//! `sim_agent::command_to_json` writes it), `"staff"` (what each side's staff
//! handed in, by day and side, as `sim_agent::staff_decisions_to_json` writes
//! it; only the days and sides that handed in anything) and
//! `"doctrine_version"` (the doctrine whose staff was passed in). A replay
//! hands in the orders, the commands and the staff's decisions and asks no
//! staff; `replay --check-staff` asks the doctrine's staff as well and says
//! where it first differs.
//!
//! Version 4 (decisions 0115, 0116 and 0117) adds the intent tree: in
//! `"commands"` and in what the staff handed in, the commands `set_intent`
//! with `intent_kind` (`keep_supplied`, left out, `cover` or `hold`) and the
//! staff's `form_intent` (`parent`, `intent_kind`, `base`); a forces limit with
//! `air_groups` and a `source` that may be left out; a hold on a whole air
//! group (`hold` or `release` with `air_group` and no `setting`); and in a
//! reason's `serves` a `chain` of the intents above it. A game saved before
//! reads none of these and is refused, not converted.
//!
//! Version 5 (decision 0120) adds the intent kinds `neutralise` (`target`) and
//! `raid` (`base` and `target`), a `target` in the staff's `form_intent`, and
//! in a reason's `serves` a `level` (`"area"` or `{"base": n}`), the level
//! that decided; it is left out where a reason has none.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use sim_agent::{
    command_from_json, command_to_json, order_from_json, order_to_json, staff_decisions_from_json,
    staff_decisions_to_json, unit_text, SeatKind,
};
use sim_core::command::{CommandKind, StaffDecisions};
use sim_core::hash::StateHasher;
use sim_core::ids::IntentId;
use sim_core::orders::{Issuer, OrderRequest, Origin};
use sim_core::state::Side;
use sim_data::Scenario;

pub const FORMAT: &str = "flagplot-saved-game";
/// Version 2 (decision 0100, 8) adds `"state_version"` and `"rules_version"`:
/// what the game was played under. Version 3 (decision 0107) adds
/// `"commands"`, `"staff"` and `"doctrine_version"`. Version 4 adds the
/// intent tree (decisions 0115 to 0117): the intent kinds `cover` and `hold`,
/// the staff's `form_intent`, `air_groups` and an optional `source` in a
/// forces limit, a hold on a whole air group and a reason's `chain`.
/// Version 5 (decision 0120) adds the intent kinds `neutralise` and `raid`
/// (with their `target`, also in the staff's `form_intent`) and a reason's
/// `level` in `serves`. A game of an older version lacks them and is
/// refused, not converted.
pub const VERSION: u32 = 5;

pub type Script = BTreeMap<u32, Vec<(Side, OrderRequest)>>;

/// The commander's commands handed in, by day, in the order given.
pub type Commands = BTreeMap<u32, Vec<(Side, CommandKind)>>;

/// What each side's staff handed in, by day: only the days and sides that
/// handed in anything.
pub type StaffHandIns = BTreeMap<u32, BTreeMap<Side, StaffDecisions>>;

fn side_name(side: Side) -> &'static str {
    match side {
        Side::Allied => "allied",
        Side::Japanese => "japanese",
    }
}

fn side_from(name: &str) -> Result<Side, String> {
    match name {
        "allied" => Ok(Side::Allied),
        "japanese" => Ok(Side::Japanese),
        other => Err(format!("unknown side \"{other}\"")),
    }
}

/// What a run needs to remember to be saved: read before it starts, since the
/// scenario spends its script as it runs.
pub struct Remembered {
    script: Script,
    commands: Commands,
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

/// The version of the state's layout the engine has, in the text a batch file
/// and a saved game record it.
fn engine_state_version() -> String {
    String::from_utf8_lossy(sim_core::hash::STATE_VERSION).into_owned()
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
        state_version: engine_state_version(),
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
        commands: scenario.scripted_commands().clone(),
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

    /// The scenario's own commands, as they will be submitted.
    pub fn commands(&self) -> &Commands {
        &self.commands
    }
}

/// Writes a saved game. `orders` are the orders as handed in to the engine,
/// by day, for both sides; `commands` the commander's commands as handed in;
/// `staff` what each side's staff handed in; `hashes` are `(day, hash)` for
/// each day resolved.
#[allow(clippy::too_many_arguments)]
pub fn write(
    file: &str,
    scenario_path: &str,
    seed: u64,
    remembered: &Remembered,
    orders: &Script,
    commands: &Commands,
    staff: &StaffHandIns,
    seats: &BTreeMap<Side, SeatKind>,
    hashes: &[(u32, u64)],
) -> Result<(), String> {
    let scenario_hash = file_hash(Path::new(scenario_path))?;
    let mut order_list = Vec::new();
    for (day, list) in orders {
        for (side, request) in list {
            order_list.push(json!({
                "day": day,
                "side": side_name(*side),
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
    let command_list: Vec<Value> = commands
        .iter()
        .flat_map(|(day, list)| {
            list.iter().map(move |(side, command)| {
                json!({
                    "day": day,
                    "side": side_name(*side),
                    "command": command_to_json(command),
                })
            })
        })
        .collect();
    let staff_list: Vec<Value> = staff
        .iter()
        .flat_map(|(day, by_side)| {
            by_side
                .iter()
                .filter(|(_, decisions)| !decisions.is_empty())
                .map(move |(side, decisions)| {
                    json!({
                        "day": day,
                        "side": side_name(*side),
                        "decisions": staff_decisions_to_json(decisions),
                    })
                })
        })
        .collect();
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
        "state_version": engine_state_version(),
        "rules_version": sim_core::world::RULES_VERSION,
        "doctrine_version": sim_staff::DOCTRINE_VERSION,
        "seats": {
            "allied": seats.get(&Side::Allied).copied().unwrap_or(SeatKind::Script).name(),
            "japanese": seats.get(&Side::Japanese).copied().unwrap_or(SeatKind::Script).name(),
        },
        "orders": order_list,
        "commands": command_list,
        "staff": staff_list,
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
    /// The commander's commands handed in, by day.
    pub commands: Commands,
    /// What each side's staff handed in, by day.
    pub staff: StaffHandIns,
    /// The doctrine whose staff was passed in when the game was saved.
    pub doctrine_version: String,
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
    match value.get("version").and_then(Value::as_u64) {
        Some(v) if v == u64::from(VERSION) => {}
        Some(v) if v < 2 => {
            return Err(bad(format!(
                "this game was saved before saved games recorded the rules they were played under \
                 (saved game version {v}, now {VERSION}); it is not converted, so play it again"
            )));
        }
        Some(v) if v < u64::from(VERSION) => {
            return Err(bad(format!(
                "this game was saved before saved games kept the commander's commands, what the staff handed in, \
                 the doctrine and the intent tree (saved game version {v}, now {VERSION}); it is not converted, so play it again"
            )));
        }
        _ => return Err(bad(format!("saved game version is not {VERSION}"))),
    }
    let saved_state = text(&value, "state_version").map_err(&bad)?;
    let engine_state = engine_state_version();
    if saved_state != engine_state {
        return Err(bad(format!(
            "this game was saved under state layout {saved_state}, and this engine has {engine_state}; \
             it is not converted, so play it again"
        )));
    }
    let saved_rules = value
        .get("rules_version")
        .and_then(Value::as_u64)
        .ok_or_else(|| bad("\"rules_version\" is missing or not a whole number".into()))?;
    let engine_rules = u64::from(sim_core::world::RULES_VERSION);
    if saved_rules != engine_rules {
        return Err(bad(format!(
            "this game was saved under rules version {saved_rules}, and this engine has rules version {engine_rules}; \
             it is not converted, so play it again"
        )));
    }
    let doctrine_version = text(&value, "doctrine_version").map_err(&bad)?.to_string();
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
        let side = side_from(text(entry, "side").map_err(&bad)?).map_err(&bad)?;
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
    let mut commands: Commands = BTreeMap::new();
    for entry in value
        .get("commands")
        .and_then(Value::as_array)
        .ok_or_else(|| bad("no commands".into()))?
    {
        let day = small(entry, "day").map_err(&bad)?;
        let side = side_from(text(entry, "side").map_err(&bad)?).map_err(&bad)?;
        let command = command_from_json(
            entry
                .get("command")
                .ok_or_else(|| bad("a command entry has no command".into()))?,
        )
        .map_err(|e| bad(format!("command on day {day}: {e}")))?;
        commands.entry(day).or_default().push((side, command));
    }
    let mut staff: StaffHandIns = BTreeMap::new();
    for entry in value
        .get("staff")
        .and_then(Value::as_array)
        .ok_or_else(|| bad("no staff".into()))?
    {
        let day = small(entry, "day").map_err(&bad)?;
        let side = side_from(text(entry, "side").map_err(&bad)?).map_err(&bad)?;
        let decisions = staff_decisions_from_json(
            entry
                .get("decisions")
                .ok_or_else(|| bad("a staff entry has no decisions".into()))?,
        )
        .map_err(|e| bad(format!("staff hand-in on day {day}: {e}")))?;
        if staff
            .entry(day)
            .or_default()
            .insert(side, decisions)
            .is_some()
        {
            return Err(bad(format!(
                "the {} staff handed in twice on day {day}",
                side_name(side)
            )));
        }
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
        commands,
        staff,
        doctrine_version,
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

#[cfg(test)]
mod tests {
    use super::*;
    use sim_core::fixed::Fixed;
    use sim_core::ids::{BaseId, ShipId, TaskForceId};
    use sim_core::orders::{BaseOrder, Cargo, OrderKind, TaskForceOrder};

    fn request(kind: OrderKind) -> OrderRequest {
        OrderRequest {
            kind,
            origin: Origin {
                issuer: Issuer::Staff,
                intent: None,
            },
        }
    }

    /// Decision 0111: a load up to a weight (both cargos), a join, a stand
    /// down and a forming order at a base are written and read back as the
    /// same orders, with the weight exact to the millionth and the base as
    /// the target.
    #[test]
    fn the_orders_of_decision_0111_round_trip_through_a_saved_game() {
        let force = TaskForceId::new(3);
        let other = TaskForceId::new(7);
        let kinds = vec![
            OrderKind::TaskForce {
                target: force,
                order: TaskForceOrder::LoadTo {
                    cargo: Cargo::Supply,
                    weight_t: Fixed::from_int(1200),
                },
            },
            OrderKind::TaskForce {
                target: force,
                order: TaskForceOrder::LoadTo {
                    cargo: Cargo::Fuel,
                    weight_t: Fixed::from_raw(2_500_001),
                },
            },
            OrderKind::TaskForce {
                target: force,
                order: TaskForceOrder::JoinTo(other),
            },
            OrderKind::TaskForce {
                target: other,
                order: TaskForceOrder::StandDown,
            },
            OrderKind::Base {
                target: BaseId::new(2),
                order: BaseOrder::FormTaskForce {
                    ships: vec![ShipId::new(11), ShipId::new(4)],
                },
            },
        ];
        let mut script: Script = BTreeMap::new();
        script.insert(
            2,
            kinds
                .iter()
                .cloned()
                .map(|kind| (Side::Allied, request(kind)))
                .collect(),
        );
        let dir = std::env::temp_dir().join(format!("flagplot-save-0111-{}", std::process::id()));
        fs::create_dir_all(&dir).expect("make the folder");
        let file = dir.join("game.save.json");
        let file = file.to_string_lossy().into_owned();
        // Any file will do as the scenario: only its hash is written.
        let scenario = concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml");
        let remembered = Remembered {
            script: BTreeMap::new(),
            commands: BTreeMap::new(),
            files: Vec::new(),
        };
        write(
            &file,
            scenario,
            9,
            &remembered,
            &script,
            &BTreeMap::new(),
            &BTreeMap::new(),
            &script_seats(),
            &[],
        )
        .expect("writes");
        let saved = read(&file).expect("reads");
        let back: Vec<OrderKind> = saved
            .script
            .get(&2)
            .expect("day 2")
            .iter()
            .map(|(side, request)| {
                assert_eq!(*side, Side::Allied);
                request.kind.clone()
            })
            .collect();
        assert_eq!(back, kinds);
        fs::remove_dir_all(&dir).ok();
    }

    /// Decision 0107: the commander's commands handed in by day, and what a
    /// side's staff handed in (an order with its intent and its five-part
    /// reason, a command, a decision to give no order), are written and read
    /// back exactly, and the doctrine is recorded.
    #[test]
    fn commands_and_a_staffs_hand_in_round_trip_through_a_saved_game() {
        use sim_core::command::{
            Evidence, ForcesLimit, IntentKind, Leave, Limit, PlanId, Reason, Risk, RunDays,
            RunSpec, Serves, StaffCommand, StaffOrder,
        };
        use sim_core::fixed::Fixed;
        use sim_core::hex::Hex;
        use sim_core::ids::{BaseId, ShipId, TaskForceId};
        use sim_core::orders::{Cargo, OrderKind, TaskForceOrder};

        let intent = IntentId::new(1);
        let mut commands: Commands = BTreeMap::new();
        commands.insert(
            1,
            vec![
                (
                    Side::Allied,
                    CommandKind::SetIntent {
                        intent,
                        kind: IntentKind::KeepSupplied {
                            base: BaseId::new(2),
                            floor_days: 10,
                            interval_days: Some(15),
                        },
                    },
                ),
                (
                    Side::Allied,
                    CommandKind::SetLimit(Limit::Forces {
                        intent,
                        limit: ForcesLimit {
                            ships: [ShipId::new(3), ShipId::new(4)].into_iter().collect(),
                            task_forces: [TaskForceId::new(1)].into_iter().collect(),
                            air_groups: [sim_core::ids::AirGroupId::new(5)].into_iter().collect(),
                            source: Some(BaseId::new(1)),
                        },
                    }),
                ),
            ],
        );
        commands.insert(
            3,
            vec![
                (
                    Side::Japanese,
                    CommandKind::SetLimit(Limit::Area {
                        intent,
                        hexes: [Hex::new(2, -1), Hex::new(0, 0)].into_iter().collect(),
                    }),
                ),
                (
                    Side::Allied,
                    CommandKind::SetLimit(Limit::Reserve {
                        base: BaseId::new(1),
                        supply_t: Fixed::from_raw(400_500_001),
                        fuel_t: Fixed::from_int(300),
                    }),
                ),
                (
                    Side::Allied,
                    CommandKind::SetLimit(Limit::Risk {
                        intent,
                        risk: Risk::Preserve,
                    }),
                ),
                (
                    Side::Allied,
                    CommandKind::SetLeave {
                        leave: Leave::Launch,
                        given: true,
                    },
                ),
                (
                    Side::Allied,
                    CommandKind::Approve {
                        plan: PlanId::new(1),
                    },
                ),
                (
                    Side::Allied,
                    CommandKind::SetInterrupt(sim_core::command::InterruptSetting::Set {
                        interrupt: sim_core::ids::InterruptId::new(1),
                        condition: sim_core::command::InterruptCondition::SupplyBelow {
                            base: BaseId::new(2),
                            days: 35,
                        },
                    }),
                ),
                (
                    Side::Allied,
                    CommandKind::SetIntent {
                        intent: IntentId::new(7),
                        kind: IntentKind::Hold {
                            base: BaseId::new(2),
                        },
                    },
                ),
                (
                    Side::Allied,
                    CommandKind::Hold {
                        target: sim_core::command::HoldTarget::AirGroup(
                            sim_core::ids::AirGroupId::new(5),
                        ),
                    },
                ),
                (
                    Side::Allied,
                    CommandKind::Acknowledge {
                        interrupt: sim_core::command::InterruptRef::Standing {
                            kind: sim_core::command::StandingKind::CannotMeet,
                            subject: sim_core::command::StandingSubject::Plan(PlanId::new(1)),
                        },
                    },
                ),
            ],
        );
        let reason = Reason {
            serves: Serves {
                intent: Some(intent),
                plan: Some(PlanId::new(1)),
                amendment: None,
                statement: "keep base:2 at 30 days of supply, a run every 15 days".to_string(),
                chain: vec![sim_core::command::ChainLink {
                    intent: IntentId::new(7),
                    maker: sim_core::command::Maker::Commander,
                    statement: "hold base:2".to_string(),
                }],
                level: Some(sim_core::command::Addressee::Base(
                    sim_core::ids::BaseId::new(2),
                )),
            },
            evidence: vec![Evidence {
                what: "base 2 holds 40 t".to_string(),
                day: 2,
            }],
            binding: vec!["the pool holds ship 3".to_string()],
            worked: "10 days x 12 t = 120 t".to_string(),
            would_change: vec!["a second ship in the pool".to_string()],
        };
        let decisions = StaffDecisions {
            orders: vec![StaffOrder {
                kind: OrderKind::TaskForce {
                    target: TaskForceId::new(1),
                    order: TaskForceOrder::LoadTo {
                        cargo: Cargo::Supply,
                        weight_t: Fixed::from_raw(120_000_001),
                    },
                },
                intent,
                reason: reason.clone(),
            }],
            commands: vec![
                StaffCommand {
                    kind: CommandKind::AddRun {
                        plan: PlanId::new(1),
                        run: RunSpec {
                            ships: [ShipId::new(3)].into_iter().collect(),
                            load_t: Fixed::from_int(120),
                            days: RunDays {
                                load: 3,
                                sail: 4,
                                arrive: 9,
                                free: 14,
                            },
                        },
                    },
                    reason: reason.clone(),
                },
                StaffCommand {
                    kind: CommandKind::FormIntent {
                        parent: IntentId::new(7),
                        kind: IntentKind::Cover {
                            base: BaseId::new(2),
                        },
                    },
                    reason: reason.clone(),
                },
            ],
            no_orders: vec![reason],
            notices: vec![
                sim_core::command::StaffNotice {
                    kind: sim_core::command::NoticeKind::CannotMeet,
                    plan: PlanId::new(1),
                    run: None,
                    rests_on: sim_core::command::NoticeBasis::NoOrder(0),
                },
                sim_core::command::StaffNotice {
                    kind: sim_core::command::NoticeKind::RunEnded,
                    plan: PlanId::new(1),
                    run: Some(2),
                    rests_on: sim_core::command::NoticeBasis::Command(0),
                },
            ],
        };
        let mut staff: StaffHandIns = BTreeMap::new();
        staff
            .entry(3)
            .or_default()
            .insert(Side::Allied, decisions.clone());
        // A side that handed in nothing is not written.
        staff
            .entry(4)
            .or_default()
            .insert(Side::Japanese, StaffDecisions::default());

        let dir = std::env::temp_dir().join(format!("flagplot-save-0107-{}", std::process::id()));
        fs::create_dir_all(&dir).expect("make the folder");
        let file = dir.join("game.save.json");
        let file = file.to_string_lossy().into_owned();
        let scenario = concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml");
        let remembered = Remembered {
            script: BTreeMap::new(),
            commands: BTreeMap::new(),
            files: Vec::new(),
        };
        write(
            &file,
            scenario,
            9,
            &remembered,
            &BTreeMap::new(),
            &commands,
            &staff,
            &script_seats(),
            &[],
        )
        .expect("writes");
        let written: Value = serde_json::from_str(&fs::read_to_string(&file).expect("reads"))
            .expect("the file is JSON");
        assert_eq!(written["version"], 5);
        assert_eq!(written["doctrine_version"], sim_staff::DOCTRINE_VERSION);
        assert_eq!(written["commands"].as_array().expect("commands").len(), 11);
        assert_eq!(written["staff"].as_array().expect("staff").len(), 1);
        let saved = read(&file).expect("reads");
        assert_eq!(saved.commands, commands);
        assert_eq!(saved.staff.len(), 1, "an empty hand-in is not kept");
        assert_eq!(saved.staff[&3][&Side::Allied], decisions);
        assert_eq!(saved.doctrine_version, sim_staff::DOCTRINE_VERSION);
        // A second write of what was read is the same bytes.
        let again = dir.join("again.save.json");
        let again = again.to_string_lossy().into_owned();
        write(
            &again,
            scenario,
            9,
            &remembered,
            &BTreeMap::new(),
            &saved.commands,
            &saved.staff,
            &script_seats(),
            &[],
        )
        .expect("writes again");
        assert_eq!(
            fs::read_to_string(&file).expect("reads"),
            fs::read_to_string(&again).expect("reads")
        );
        // A hand-in saved before notices has no `notices` key and reads as
        // having none; the version stays 3.
        let mut before = written.clone();
        for entry in before["staff"].as_array_mut().expect("staff") {
            entry["decisions"]
                .as_object_mut()
                .expect("decisions")
                .remove("notices");
        }
        fs::write(&file, before.to_string()).expect("writes");
        let before = read(&file).expect("an older hand-in reads");
        assert!(before.staff[&3][&Side::Allied].notices.is_empty());
        assert_eq!(before.staff[&3][&Side::Allied].orders, decisions.orders);
        // An older version is refused, not converted.
        let mut old = written.clone();
        old["version"] = json!(2);
        fs::write(&file, old.to_string()).expect("writes");
        let error = read(&file).err().expect("version 2 is refused");
        assert!(error.contains("not converted"), "{error}");
        fs::remove_dir_all(&dir).ok();
    }
}
