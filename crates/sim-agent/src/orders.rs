//! Orders as JSON, in the form saved games use (decision 7 of 0050).
//!
//! One encoding for the saved game and for `issue_orders`: an order is a unit
//! (`"task_force:3"` or `"air_group:12"`) and a body whose `"kind"` names the
//! order. Every quantity is a whole number or a decimal string, so the same
//! order always gives the same bytes. The saved-game file and the agent
//! interface both use these functions; there is no second encoding.

use serde_json::{json, Map, Value};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, ShipId, TaskForceId};
use sim_core::orders::{
    AirGroupOrder, AirMission, Mission, OrderKind, Reach, Readiness, SpeedSetting, Stance,
    StragglerInstruction, TaskForceOrder,
};

/// The unit an order is for, as `"task_force:N"` or `"air_group:N"`.
pub fn unit_text(kind: &OrderKind) -> String {
    match kind {
        OrderKind::TaskForce { target, .. } => format!("task_force:{}", target.raw()),
        OrderKind::AirGroup { target, .. } => format!("air_group:{}", target.raw()),
    }
}

fn fixed_text(value: Fixed) -> Value {
    Value::String(value.to_string())
}

pub fn order_to_json(kind: &OrderKind) -> Value {
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
/// `task_force:N` or `air_group:N` with `N` a whole number.
pub fn is_orderable_unit(unit: &str) -> bool {
    match unit.split_once(':') {
        Some(("task_force" | "air_group", number)) => number.parse::<u32>().is_ok(),
        _ => false,
    }
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
                "load" => match value.get("cargo") {
                    None => TaskForceOrder::Load,
                    Some(_) => match text(value, "cargo")? {
                        "supply" => TaskForceOrder::Load,
                        "fuel" => TaskForceOrder::LoadFuel,
                        other => return Err(format!("unknown cargo \"{other}\"")),
                    },
                },
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
                "set_reach" => AirGroupOrder::SetReach {
                    reach: match text(value, "reach")? {
                        "normal" => Reach::Normal,
                        "extended" => Reach::Extended,
                        other => return Err(format!("unknown reach \"{other}\"")),
                    },
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
    fn an_unknown_cargo_is_refused() {
        let bad: Value = serde_json::from_str(r#"{"kind":"load","cargo":"oil"}"#).expect("JSON");
        assert!(load_of(&bad).is_err());
    }
}
