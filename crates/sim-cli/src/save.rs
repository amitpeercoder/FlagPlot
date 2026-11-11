//! Saved games, `replay` and `hash` (decisions 6 to 9 of 0043).
//!
//! A saved game names the scenario file and records its hash and the hash of
//! every data file the scenario names, the seed, the orders of each day as
//! they were submitted (with their origin) and the daily hashes of the run
//! that saved it. It holds no state: the starting state is the scenario's.
//! Every quantity is a whole number, a decimal string or a plain string, so
//! the same run gives the same bytes. The hash of a file is the FNV-1a hash
//! the simulation uses for its state, over the file's bytes.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Map, Value};
use sim_core::fixed::Fixed;
use sim_core::hash::StateHasher;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, IntentId, ShipId, TaskForceId};
use sim_core::orders::{
    AirGroupOrder, AirMission, Issuer, Mission, OrderKind, OrderRequest, Origin, Readiness,
    SpeedSetting, Stance, StragglerInstruction, TaskForceOrder,
};
use sim_core::state::Side;
use sim_core::DaySummary;
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

pub fn write(
    file: &str,
    scenario_path: &str,
    seed: u64,
    remembered: &Remembered,
    summaries: &[DaySummary],
) -> Result<(), String> {
    let scenario_hash = file_hash(Path::new(scenario_path))?;
    let mut orders = Vec::new();
    for (day, list) in &remembered.script {
        for (side, request) in list {
            orders.push(json!({
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
        "orders": orders,
        "hashes": summaries
            .iter()
            .map(|s| json!({ "day": s.day, "hash": format!("{:016x}", s.state_hash) }))
            .collect::<Vec<_>>(),
    });
    let mut text = serde_json::to_string_pretty(&value).map_err(|e| e.to_string())?;
    text.push('\n');
    fs::write(file, text).map_err(|e| format!("{file}: {e}"))
}

fn unit_text(kind: &OrderKind) -> String {
    match kind {
        OrderKind::TaskForce { target, .. } => format!("task_force:{}", target.raw()),
        OrderKind::AirGroup { target, .. } => format!("air_group:{}", target.raw()),
    }
}

fn fixed_text(value: Fixed) -> Value {
    Value::String(value.to_string())
}

fn order_to_json(kind: &OrderKind) -> Value {
    match kind {
        OrderKind::TaskForce { order, .. } => match order {
            TaskForceOrder::SetRoute(waypoints) => json!({
                "kind": "set_route",
                "waypoints": waypoints.iter().map(|h| json!([h.q, h.r])).collect::<Vec<_>>(),
            }),
            TaskForceOrder::SetSpeed(speed) => match speed {
                SpeedSetting::Cruise => json!({ "kind": "set_speed", "speed": "cruise" }),
                SpeedSetting::Full => json!({ "kind": "set_speed", "speed": "full" }),
                SpeedSetting::Knots(knots) => {
                    json!({ "kind": "set_speed", "knots": fixed_text(*knots) })
                }
            },
            TaskForceOrder::SetMission(mission) => {
                json!({ "kind": "set_mission", "mission": mission_name(*mission) })
            }
            TaskForceOrder::Load => json!({ "kind": "load" }),
            TaskForceOrder::Unload => json!({ "kind": "unload" }),
            TaskForceOrder::SetRulesOfEngagement {
                stance,
                break_off,
                stragglers,
                keep_share,
            } => json!({
                "kind": "set_rules_of_engagement",
                "stance": match stance { Stance::Engage => "engage", Stance::Avoid => "avoid" },
                "break_off": fixed_text(*break_off),
                "stragglers": stragglers.name(),
                "keep_share": fixed_text(*keep_share),
            }),
            TaskForceOrder::SetHomePort(base) => {
                json!({ "kind": "set_home_port", "base": base.raw() })
            }
            TaskForceOrder::DetachShips(ships) => json!({
                "kind": "detach",
                "ships": ships.iter().map(|s| s.raw()).collect::<Vec<_>>(),
            }),
            TaskForceOrder::SetReadiness(readiness) => {
                json!({ "kind": "set_readiness", "readiness": readiness.name() })
            }
        },
        OrderKind::AirGroup { order, .. } => match order {
            AirGroupOrder::SetAirMission(mission) => {
                let mut map = Map::new();
                map.insert("kind".into(), json!("set_air_mission"));
                match mission {
                    AirMission::Rest => map.insert("mission".into(), json!("rest")),
                    AirMission::Search => map.insert("mission".into(), json!("search")),
                    AirMission::NavalStrike => map.insert("mission".into(), json!("naval_strike")),
                    AirMission::CombatAirPatrol => {
                        map.insert("mission".into(), json!("combat_air_patrol"))
                    }
                    AirMission::BaseStrike(base) => {
                        map.insert("mission".into(), json!("base_strike"));
                        map.insert("target_base".into(), json!(base.raw()))
                    }
                };
                Value::Object(map)
            }
            AirGroupOrder::SetSearchArc {
                from_deg,
                to_deg,
                range_nm,
            } => json!({
                "kind": "set_search_arc",
                "from_deg": from_deg,
                "to_deg": to_deg,
                "range_nm": fixed_text(*range_nm),
            }),
            AirGroupOrder::SetPatrolShare { share } => {
                json!({ "kind": "set_patrol_share", "share": fixed_text(*share) })
            }
            AirGroupOrder::SetRestThreshold { fatigue } => {
                json!({ "kind": "set_rest_threshold", "fatigue": fixed_text(*fatigue) })
            }
        },
    }
}

fn mission_name(mission: Mission) -> &'static str {
    match mission {
        Mission::Transport => "transport",
        Mission::SurfaceCombat => "surface_combat",
        Mission::Carrier => "carrier",
        Mission::Bombardment => "bombardment",
        Mission::Escort => "escort",
    }
}

fn mission_from(name: &str) -> Result<Mission, String> {
    Ok(match name {
        "transport" => Mission::Transport,
        "surface_combat" => Mission::SurfaceCombat,
        "carrier" => Mission::Carrier,
        "bombardment" => Mission::Bombardment,
        "escort" => Mission::Escort,
        other => return Err(format!("unknown mission \"{other}\"")),
    })
}

fn text<'a>(value: &'a Value, key: &str) -> Result<&'a str, String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("\"{key}\" is missing or not text"))
}

fn whole(value: &Value, key: &str) -> Result<i64, String> {
    value
        .get(key)
        .and_then(Value::as_i64)
        .ok_or_else(|| format!("\"{key}\" is missing or not a whole number"))
}

fn small(value: &Value, key: &str) -> Result<u32, String> {
    u32::try_from(whole(value, key)?).map_err(|_| format!("\"{key}\" is out of range"))
}

fn fixed(value: &Value, key: &str) -> Result<Fixed, String> {
    text(value, key)?
        .parse::<Fixed>()
        .map_err(|e| format!("\"{key}\": {e}"))
}

fn order_from_json(unit: &str, value: &Value) -> Result<OrderKind, String> {
    let (kind_of_unit, raw) = unit
        .split_once(':')
        .ok_or_else(|| format!("unit \"{unit}\" is not kind:number"))?;
    let raw: u32 = raw
        .parse()
        .map_err(|_| format!("unit \"{unit}\" has no number"))?;
    let kind = text(value, "kind")?;
    match kind_of_unit {
        "task_force" => {
            let order = match kind {
                "set_route" => {
                    let list = value
                        .get("waypoints")
                        .and_then(Value::as_array)
                        .ok_or("\"waypoints\" is missing")?;
                    let mut hexes = Vec::new();
                    for pair in list {
                        let pair = pair.as_array().ok_or("a waypoint is not a pair")?;
                        let get = |i: usize| {
                            pair.get(i)
                                .and_then(Value::as_i64)
                                .and_then(|n| i32::try_from(n).ok())
                                .ok_or("a waypoint is not a pair of whole numbers")
                        };
                        hexes.push(Hex::new(get(0)?, get(1)?));
                    }
                    TaskForceOrder::SetRoute(hexes)
                }
                "set_speed" => TaskForceOrder::SetSpeed(if value.get("knots").is_some() {
                    SpeedSetting::Knots(fixed(value, "knots")?)
                } else {
                    match text(value, "speed")? {
                        "cruise" => SpeedSetting::Cruise,
                        "full" => SpeedSetting::Full,
                        other => return Err(format!("unknown speed \"{other}\"")),
                    }
                }),
                "set_mission" => TaskForceOrder::SetMission(mission_from(text(value, "mission")?)?),
                "load" => TaskForceOrder::Load,
                "unload" => TaskForceOrder::Unload,
                "set_rules_of_engagement" => TaskForceOrder::SetRulesOfEngagement {
                    stance: match text(value, "stance")? {
                        "engage" => Stance::Engage,
                        "avoid" => Stance::Avoid,
                        other => return Err(format!("unknown stance \"{other}\"")),
                    },
                    break_off: fixed(value, "break_off")?,
                    stragglers: match text(value, "stragglers")? {
                        "keep" => StragglerInstruction::Keep,
                        "detach" => StragglerInstruction::Detach,
                        "leave" => StragglerInstruction::Leave,
                        "retire" => StragglerInstruction::Retire,
                        other => return Err(format!("unknown straggler instruction \"{other}\"")),
                    },
                    keep_share: fixed(value, "keep_share")?,
                },
                "set_home_port" => TaskForceOrder::SetHomePort(BaseId::new(small(value, "base")?)),
                "detach" => {
                    let list = value
                        .get("ships")
                        .and_then(Value::as_array)
                        .ok_or("\"ships\" is missing")?;
                    let mut ships = Vec::new();
                    for ship in list {
                        let raw = ship
                            .as_u64()
                            .and_then(|n| u32::try_from(n).ok())
                            .ok_or("a ship is not a whole number")?;
                        ships.push(ShipId::new(raw));
                    }
                    TaskForceOrder::DetachShips(ships)
                }
                "set_readiness" => TaskForceOrder::SetReadiness(match text(value, "readiness")? {
                    "normal" => Readiness::Normal,
                    "battle" => Readiness::Battle,
                    other => return Err(format!("unknown readiness \"{other}\"")),
                }),
                other => return Err(format!("unknown task force order \"{other}\"")),
            };
            Ok(OrderKind::TaskForce {
                target: TaskForceId::new(raw),
                order,
            })
        }
        "air_group" => {
            let order = match kind {
                "set_air_mission" => AirGroupOrder::SetAirMission(match text(value, "mission")? {
                    "rest" => AirMission::Rest,
                    "search" => AirMission::Search,
                    "naval_strike" => AirMission::NavalStrike,
                    "combat_air_patrol" => AirMission::CombatAirPatrol,
                    "base_strike" => {
                        AirMission::BaseStrike(BaseId::new(small(value, "target_base")?))
                    }
                    other => return Err(format!("unknown air mission \"{other}\"")),
                }),
                "set_search_arc" => AirGroupOrder::SetSearchArc {
                    from_deg: i32::try_from(whole(value, "from_deg")?)
                        .map_err(|_| "from_deg is out of range".to_string())?,
                    to_deg: i32::try_from(whole(value, "to_deg")?)
                        .map_err(|_| "to_deg is out of range".to_string())?,
                    range_nm: fixed(value, "range_nm")?,
                },
                "set_patrol_share" => AirGroupOrder::SetPatrolShare {
                    share: fixed(value, "share")?,
                },
                "set_rest_threshold" => AirGroupOrder::SetRestThreshold {
                    fatigue: fixed(value, "fatigue")?,
                },
                other => return Err(format!("unknown air group order \"{other}\"")),
            };
            Ok(OrderKind::AirGroup {
                target: AirGroupId::new(raw),
                order,
            })
        }
        other => Err(format!("unknown unit kind \"{other}\"")),
    }
}

/// A saved game, read back.
pub struct Saved {
    pub scenario_file: String,
    scenario_hash: String,
    data_files: Vec<(String, String)>,
    pub seed: u64,
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
