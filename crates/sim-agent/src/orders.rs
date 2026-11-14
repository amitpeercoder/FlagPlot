//! Orders as JSON, in the form saved games use (decision 7 of 0050).
//!
//! One encoding for the saved game and for `issue_orders`: an order is a unit
//! (`"task_force:3"` or `"air_group:12"`) and a body whose `"kind"` names the
//! order. Every quantity is a whole number or a decimal string, so the same
//! order always gives the same bytes. The saved-game file and the agent
//! interface both use these functions; there is no second encoding.

use std::collections::BTreeSet;

use serde_json::{json, Map, Value};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, LandUnitId, ShipId, TaskForceId};
use sim_core::orders::{
    Activity, AirGroupOrder, AirMission, Cargo, LandUnitOrder, Mission, OrderKind, Posture, Reach,
    Readiness, SpeedSetting, Stance, StragglerInstruction, TaskForceOrder, WhenSpent,
};
use sim_core::refdata::ShipKind;

/// The unit an order is for, as `"task_force:N"` or `"air_group:N"`.
pub fn unit_text(kind: &OrderKind) -> String {
    match kind {
        OrderKind::TaskForce { target, .. } => format!("task_force:{}", target.raw()),
        OrderKind::AirGroup { target, .. } => format!("air_group:{}", target.raw()),
        OrderKind::LandUnit { target, .. } => format!("land_unit:{}", target.raw()),
    }
}

fn fixed_text(value: Fixed) -> Value {
    Value::String(value.to_string())
}

pub fn order_to_json(kind: &OrderKind) -> Value {
    match kind {
        OrderKind::TaskForce { order, .. } => match order {
            TaskForceOrder::SetRoute { waypoints, act_at } => {
                let mut body = json!({
                    "kind": "set_route",
                    "waypoints": waypoints.iter().map(|h| json!([h.q, h.r])).collect::<Vec<_>>(),
                });
                // Written only when a waypoint is marked (decision 0096), so
                // a route with no mark is written as it always was.
                if !act_at.is_empty() {
                    body["act_at"] = act_at.iter().map(|h| json!([h.q, h.r])).collect();
                }
                body
            }
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
            // A load of supply is written as it always was: only fuel names
            // its cargo, so a save of an old game is unchanged (decision 0046).
            TaskForceOrder::LoadFuel => json!({ "kind": "load", "cargo": "fuel" }),
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
            TaskForceOrder::SendToYard => json!({ "kind": "send_to_yard" }),
            // A load with a land unit always names its cargo, `"none"`
            // included (decision 0066, 5): the unit is the engine's number
            // of it, as `set_home_port` and `detach` hold the engine's.
            TaskForceOrder::LoadUnit { unit, cargo } => json!({
                "kind": "load",
                "unit": format!("land_unit:{}", unit.raw()),
                "cargo": match cargo {
                    Some(cargo) => cargo.name(),
                    None => "none",
                },
            }),
            // The roster is written as the engine's land unit handles
            // (decision 0095, 1). The agent interface proper comes later.
            TaskForceOrder::PrepareLanding { hex, roster } => json!({
                "kind": "prepare_landing",
                "hex": [hex.q, hex.r],
                "roster": roster
                    .iter()
                    .map(|unit| format!("land_unit:{}", unit.raw()))
                    .collect::<Vec<_>>(),
            }),
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
            AirGroupOrder::SetReach { reach } => {
                json!({ "kind": "set_reach", "reach": reach.name() })
            }
            AirGroupOrder::SetTargetPriority { kinds } => json!({
                "kind": "set_target_priority",
                "kinds": kinds.iter().map(|kind| kind.name()).collect::<Vec<_>>(),
            }),
            AirGroupOrder::Rebase { base } => json!({
                "kind": "rebase",
                "base": base.raw(),
            }),
        },
        OrderKind::LandUnit { order, .. } => match order {
            LandUnitOrder::SetPosture {
                posture,
                when_spent,
            } => match when_spent {
                Some(when_spent) => json!({
                    "kind": "set_posture",
                    "posture": posture.name(),
                    "when_spent": when_spent.name(),
                }),
                None => json!({
                    "kind": "set_posture",
                    "posture": posture.name(),
                }),
            },
            LandUnitOrder::March(hex) => json!({
                "kind": "march",
                "hex": [hex.q, hex.r],
            }),
            LandUnitOrder::SetActivity(activity) => match activity.job() {
                Some(job) => json!({
                    "kind": "set_activity",
                    "activity": activity.name(),
                    "job": job.name(),
                }),
                None => json!({
                    "kind": "set_activity",
                    "activity": activity.name(),
                }),
            },
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

/// Whether `unit` is written as a unit an order can be for:
/// `task_force:N`, `air_group:N` or `land_unit:N` with `N` a whole number.
pub fn is_orderable_unit(unit: &str) -> bool {
    match unit.split_once(':') {
        Some(("task_force" | "air_group" | "land_unit", number)) => number.parse::<u32>().is_ok(),
        _ => false,
    }
}

/// A land unit written `land_unit:N`: `N` whole, nothing after it.
pub fn land_unit_number(handle: &str) -> Option<u32> {
    handle.strip_prefix("land_unit:")?.parse().ok()
}

/// The body of a load order (decisions 5 and 6 of 0066). Without a land unit
/// it is as it always was: supply, or fuel when named. With one, the cargo is
/// supply when none is named, or `"supply"`, `"fuel"` or `"none"`.
fn load_from(value: &Value) -> Result<TaskForceOrder, String> {
    let cargo = match value.get("cargo") {
        None => None,
        Some(_) => Some(text(value, "cargo")?),
    };
    match value.get("unit") {
        None => match cargo {
            None | Some("supply") => Ok(TaskForceOrder::Load),
            Some("fuel") => Ok(TaskForceOrder::LoadFuel),
            Some("none") => Err(
                "\"cargo\": \"none\" needs a \"unit\": a load that names no land unit loads supply or fuel"
                    .to_string(),
            ),
            Some(other) => Err(format!("unknown cargo \"{other}\"")),
        },
        Some(unit) => {
            let unit = unit
                .as_str()
                .and_then(land_unit_number)
                .ok_or("\"unit\" must be a land unit written \"land_unit:N\"")?;
            let cargo = match cargo {
                None | Some("supply") => Some(Cargo::Supply),
                Some("fuel") => Some(Cargo::Fuel),
                Some("none") => None,
                Some(other) => return Err(format!("unknown cargo \"{other}\"")),
            };
            Ok(TaskForceOrder::LoadUnit {
                unit: LandUnitId::new(unit),
                cargo,
            })
        }
    }
}

/// Reads a list of `[q, r]` pairs; `what` names one in an error.
fn hex_pairs(list: &[Value], what: &str) -> Result<Vec<Hex>, String> {
    let mut hexes = Vec::new();
    for pair in list {
        let pair = pair
            .as_array()
            .ok_or_else(|| format!("a {what} is not a pair"))?;
        let get = |i: usize| {
            pair.get(i)
                .and_then(Value::as_i64)
                .and_then(|n| i32::try_from(n).ok())
                .ok_or_else(|| format!("a {what} is not a pair of whole numbers"))
        };
        hexes.push(Hex::new(get(0)?, get(1)?));
    }
    Ok(hexes)
}

/// Reads an order from its unit and its body.
pub fn order_from_json(unit: &str, value: &Value) -> Result<OrderKind, String> {
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
                    let waypoints = hex_pairs(list, "waypoint")?;
                    // Optional (decision 0096): the waypoints to act at.
                    let act_at = match value.get("act_at") {
                        None | Some(Value::Null) => Vec::new(),
                        Some(marks) => hex_pairs(
                            marks
                                .as_array()
                                .ok_or("\"act_at\" is not a list of hexes")?,
                            "mark",
                        )?,
                    };
                    TaskForceOrder::SetRoute {
                        waypoints,
                        act_at: act_at.into_iter().collect(),
                    }
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
                "load" => load_from(value)?,
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
                "send_to_yard" => TaskForceOrder::SendToYard,
                "prepare_landing" => {
                    let hex = hex_pairs(
                        std::slice::from_ref(value.get("hex").ok_or("\"hex\" is missing")?),
                        "hex",
                    )?[0];
                    let list = value
                        .get("roster")
                        .and_then(Value::as_array)
                        .ok_or("\"roster\" is missing")?;
                    let mut roster = BTreeSet::new();
                    for item in list {
                        let unit = item
                            .as_str()
                            .and_then(land_unit_number)
                            .ok_or("a roster unit must be a land unit written \"land_unit:N\"")?;
                        roster.insert(LandUnitId::new(unit));
                    }
                    TaskForceOrder::PrepareLanding { hex, roster }
                }
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
                "set_reach" => AirGroupOrder::SetReach {
                    reach: match text(value, "reach")? {
                        "normal" => Reach::Normal,
                        "extended" => Reach::Extended,
                        other => return Err(format!("unknown reach \"{other}\"")),
                    },
                },
                "set_target_priority" => {
                    let list = value
                        .get("kinds")
                        .and_then(Value::as_array)
                        .ok_or("\"kinds\" is missing")?;
                    let mut kinds = Vec::new();
                    for item in list {
                        let name = item.as_str().ok_or("a kind is not text")?;
                        kinds.push(
                            ShipKind::ALL
                                .into_iter()
                                .find(|kind| kind.name() == name)
                                .ok_or_else(|| format!("unknown kind of ship \"{name}\""))?,
                        );
                    }
                    AirGroupOrder::SetTargetPriority { kinds }
                }
                "rebase" => AirGroupOrder::Rebase {
                    base: BaseId::new(small(value, "base")?),
                },
                other => return Err(format!("unknown air group order \"{other}\"")),
            };
            Ok(OrderKind::AirGroup {
                target: AirGroupId::new(raw),
                order,
            })
        }
        "land_unit" => {
            let order = match kind {
                "set_posture" => LandUnitOrder::SetPosture {
                    posture: match text(value, "posture")? {
                        "defend" => Posture::Defend,
                        "attack" => Posture::Attack,
                        other => return Err(format!("unknown posture \"{other}\"")),
                    },
                    // What the unit does when its attack is spent (decision 6
                    // of 0105); left out for none.
                    when_spent: match value.get("when_spent") {
                        None | Some(Value::Null) => None,
                        Some(_) => Some(match text(value, "when_spent")? {
                            "hold" => WhenSpent::Hold,
                            "fall_back" => WhenSpent::FallBack,
                            other => {
                                return Err(format!(
                                    "unknown when_spent \"{other}\": hold or fall_back"
                                ))
                            }
                        }),
                    },
                },
                "march" => {
                    let pair = value
                        .get("hex")
                        .and_then(Value::as_array)
                        .ok_or("\"hex\" is missing")?;
                    let get = |i: usize| {
                        pair.get(i)
                            .and_then(Value::as_i64)
                            .and_then(|n| i32::try_from(n).ok())
                            .ok_or("\"hex\" is not a pair of whole numbers")
                    };
                    if pair.len() != 2 {
                        return Err("\"hex\" is not a pair of whole numbers".to_string());
                    }
                    LandUnitOrder::March(Hex::new(get(0)?, get(1)?))
                }
                "set_activity" => LandUnitOrder::SetActivity(match text(value, "activity")? {
                    "ready" => Activity::Ready,
                    "train" => Activity::Train,
                    "work" => Activity::Work(match value.get("job") {
                        None | Some(Value::Null) => None,
                        Some(job) => {
                            let name = job.as_str().ok_or("\"job\" is not text")?;
                            Some(sim_core::orders::Job::from_name(name).ok_or_else(|| {
                                format!(
                                    "unknown job \"{name}\" (airfield, port, aircraft or defences)"
                                )
                            })?)
                        }
                    }),
                    other => return Err(format!("unknown activity \"{other}\"")),
                }),
                other => return Err(format!("unknown land unit order \"{other}\"")),
            };
            Ok(OrderKind::LandUnit {
                target: LandUnitId::new(raw),
                order,
            })
        }
        other => Err(format!("unknown unit kind \"{other}\"")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_of(value: &Value) -> Result<TaskForceOrder, String> {
        match order_from_json("task_force:1", value)? {
            OrderKind::TaskForce { order, .. } => Ok(order),
            other => Err(format!("not a task force order: {other:?}")),
        }
    }

    fn written(order: TaskForceOrder) -> Value {
        order_to_json(&OrderKind::TaskForce {
            target: TaskForceId::new(1),
            order,
        })
    }

    #[test]
    fn a_load_written_before_fuel_was_a_cargo_still_reads_as_supply() {
        // The encoding of a load order in every saved game made before
        // decision 0046: a kind and nothing else.
        let old: Value = serde_json::from_str(r#"{"kind":"load"}"#).expect("JSON");
        assert_eq!(load_of(&old), Ok(TaskForceOrder::Load));
        let named: Value =
            serde_json::from_str(r#"{"kind":"load","cargo":"supply"}"#).expect("JSON");
        assert_eq!(load_of(&named), Ok(TaskForceOrder::Load));
    }

    #[test]
    fn a_load_of_supply_is_still_written_as_it_was_and_fuel_names_its_cargo() {
        assert_eq!(written(TaskForceOrder::Load), json!({ "kind": "load" }));
        assert_eq!(
            written(TaskForceOrder::LoadFuel),
            json!({ "kind": "load", "cargo": "fuel" })
        );
        assert_eq!(
            load_of(&written(TaskForceOrder::LoadFuel)),
            Ok(TaskForceOrder::LoadFuel)
        );
        assert_eq!(written(TaskForceOrder::Unload), json!({ "kind": "unload" }));
    }

    #[test]
    fn a_load_with_a_land_unit_names_its_cargo_and_reads_back() {
        let unit = LandUnitId::new(4);
        for cargo in [Some(Cargo::Supply), Some(Cargo::Fuel), None] {
            let order = TaskForceOrder::LoadUnit { unit, cargo };
            assert_eq!(load_of(&written(order.clone())), Ok(order));
        }
        assert_eq!(
            written(TaskForceOrder::LoadUnit { unit, cargo: None }),
            json!({ "kind": "load", "unit": "land_unit:4", "cargo": "none" })
        );
        // No cargo key with a unit is supply.
        let bare: Value =
            serde_json::from_str(r#"{"kind":"load","unit":"land_unit:4"}"#).expect("JSON");
        assert_eq!(
            load_of(&bare),
            Ok(TaskForceOrder::LoadUnit {
                unit,
                cargo: Some(Cargo::Supply)
            })
        );
    }

    #[test]
    fn no_cargo_without_a_unit_is_refused_and_a_bad_unit_is_malformed() {
        let none: Value = serde_json::from_str(r#"{"kind":"load","cargo":"none"}"#).expect("JSON");
        let error = load_of(&none).expect_err("refused");
        assert!(error.contains("needs a \"unit\""), "{error}");
        for bad in [
            r#"{"kind":"load","unit":"ship:1"}"#,
            r#"{"kind":"load","unit":3}"#,
        ] {
            let bad: Value = serde_json::from_str(bad).expect("JSON");
            assert!(load_of(&bad).is_err());
        }
    }

    #[test]
    fn an_unknown_cargo_is_refused() {
        let bad: Value = serde_json::from_str(r#"{"kind":"load","cargo":"oil"}"#).expect("JSON");
        assert!(load_of(&bad).is_err());
    }
}
