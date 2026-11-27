use crate::error::DataError;
use sim_core::command::CommandKind;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, IntentId, LandUnitId, ShipId, SubmarineId, TaskForceId};
use sim_core::orders::{
    Activity, AirGroupOrder, AirMission, BaseOrder, Cargo, Job, LandUnitOrder, OrderKind,
    OrderRequest, Origin, SpeedSetting, SubmarineOrder, TaskForceOrder,
};
use sim_core::refdata::RefData;
use sim_core::state::{Side, TruthState};
use sim_core::time::Pulse;
use std::collections::BTreeMap;
use std::path::Path;

use super::super::file::ScenarioFile;
use super::super::orders::{ActionSpec, ActivitySpec, AirMissionSpec, CargoSpec, PostureSpec};
use super::super::ScriptInsideDay;

/// The orders a scenario scripts, by day, and the ones inside a day.
pub(super) struct OrderScript {
    pub(super) orders_inside_a_day: bool,
    pub(super) script: BTreeMap<u32, Vec<(Side, OrderRequest)>>,
    pub(super) script_inside_day: ScriptInsideDay,
}

pub(super) fn build_orders(
    file: &ScenarioFile,
    path: &Path,
    truth: &TruthState,
    _ref_data: &RefData,
    _map: &sim_core::map::Map,
) -> Result<OrderScript, DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    let orders_inside_a_day = file.orders_inside_a_day == Some(true);
    let mut script: BTreeMap<u32, Vec<(Side, OrderRequest)>> = BTreeMap::new();
    let mut script_inside_day = ScriptInsideDay::new();
    for spec in &file.order {
        if spec.day == 0 || spec.day > file.days {
            return Err(fail(format!(
                "an order is set for day {}, outside days 1 to {}",
                spec.day, file.days
            )));
        }
        // An order given inside a day (decision 0156): only where the
        // scenario says, and only at 0600 or 1200.
        let pulse = match spec.pulse.as_deref() {
                None => None,
                Some(_) if !orders_inside_a_day => {
                    return Err(fail(format!(
                        "the order on day {} has a pulse, and orders inside a day need orders_inside_a_day = true at the top of the scenario",
                        spec.day
                    )))
                }
                Some("morning") => Some(Pulse::Morning),
                Some("afternoon") => Some(Pulse::Afternoon),
                Some(other) => {
                    return Err(fail(format!(
                        "the order on day {} has pulse \"{other}\": an order inside a day is given for the \"morning\" (at 0600) or the \"afternoon\" (at 1200)",
                        spec.day
                    )))
                }
            };
        let origin = Origin {
            issuer: spec.issuer.into(),
            intent: spec.intent.map(IntentId::new),
        };
        let day = spec.day;
        let wrong_target = |kind: &str, wanted: &str| {
            fail(format!(
                "the {kind} order on day {day} must be addressed to a {wanted}"
            ))
        };
        // `base` names the target of a base order, but for a home port
        // or a rebase it is the base the order names, not the target.
        let names_a_base = matches!(
            spec.action,
            ActionSpec::SetHomePort {} | ActionSpec::Rebase {}
        );
        let (target_base, named_base) = if names_a_base {
            (None, spec.base)
        } else {
            (spec.base, None)
        };
        let need_base = |kind: &str| {
            named_base.ok_or_else(|| fail(format!("the {kind} order on day {day} needs a base")))
        };
        let kind = match (
                &spec.task_force,
                &spec.air_group,
                &spec.land_unit,
                &target_base,
                &spec.submarine,
            ) {
                (Some(raw), None, None, None, None) => {
                    let order = match &spec.action {
                        ActionSpec::SetRoute {
                            waypoints,
                            act_at,
                            stand_at,
                        } => TaskForceOrder::SetRoute {
                            waypoints: waypoints.iter().map(|[q, r]| Hex::new(*q, *r)).collect(),
                            act_at: act_at.iter().map(|[q, r]| Hex::new(*q, *r)).collect(),
                            stand_at: stand_at.iter().map(|[q, r]| Hex::new(*q, *r)).collect(),
                        },
                        ActionSpec::SetSpeed { speed, knots } => {
                            TaskForceOrder::SetSpeed(match (speed, knots) {
                                (Some(speed), None) => (*speed).into(),
                                (None, Some(knots)) => SpeedSetting::Knots(
                                    knots
                                        .to_fixed()
                                        .map_err(|e| fail(format!("knots on day {day} {e}")))?,
                                ),
                                _ => {
                                    return Err(fail(format!(
                                        "the set_speed order on day {day} needs exactly one of speed and knots"
                                    )))
                                }
                            })
                        }
                        ActionSpec::SetMission { mission } => {
                            TaskForceOrder::SetMission((*mission).into())
                        }
                        ActionSpec::Load {
                            cargo,
                            unit,
                            weight_t: Some(weight),
                            each_t,
                        } => {
                            // A weight does not apply to men (decision 5 of
                            // 0111). Its sign is for intake to refuse.
                            if unit.is_some() {
                                return Err(fail(format!(
                                    "the load order on day {day} gives a weight_t and a unit: a weight does not apply to men"
                                )));
                            }
                            let cargo = match cargo {
                                None | Some(CargoSpec::Supply) => Cargo::Supply,
                                Some(CargoSpec::Fuel) => Cargo::Fuel,
                                Some(CargoSpec::NoCargo) => {
                                    return Err(fail(format!(
                                        "the load order on day {day} gives a weight_t with cargo none: a weight needs supply or fuel"
                                    )))
                                }
                            };
                            // The level is refused at 0 or below as it is
                            // read (decision 0162, 3).
                            let each_t = match each_t {
                                Some(level) => {
                                    let level = level
                                        .to_fixed()
                                        .map_err(|e| fail(format!("each_t on day {day} {e}")))?;
                                    if level <= Fixed::ZERO {
                                        return Err(fail(format!(
                                            "the load order on day {day} gives an each_t of {level}: the most a ship is to hold must be above 0"
                                        )));
                                    }
                                    Some(level)
                                }
                                None => None,
                            };
                            TaskForceOrder::LoadTo {
                                cargo,
                                weight_t: weight
                                    .to_fixed()
                                    .map_err(|e| fail(format!("weight_t on day {day} {e}")))?,
                                each_t,
                            }
                        }
                        ActionSpec::Load {
                            each_t: Some(_), ..
                        } => {
                            return Err(fail(format!(
                                "the load order on day {day} gives an each_t and no weight_t: the most a ship is to hold goes with a weight"
                            )))
                        }
                        ActionSpec::Load {
                            cargo,
                            unit,
                            weight_t: None,
                            each_t: None,
                        } => match (unit, cargo) {
                            (None, None | Some(CargoSpec::Supply)) => TaskForceOrder::Load,
                            (None, Some(CargoSpec::Fuel)) => TaskForceOrder::LoadFuel,
                            (None, Some(CargoSpec::NoCargo)) => {
                                return Err(fail(format!(
                                    "the load order on day {day}: a load with no unit must name supply or fuel"
                                )))
                            }
                            (Some(unit), cargo) => TaskForceOrder::LoadUnit {
                                unit: LandUnitId::new(*unit),
                                cargo: match cargo {
                                    None | Some(CargoSpec::Supply) => Some(Cargo::Supply),
                                    Some(CargoSpec::Fuel) => Some(Cargo::Fuel),
                                    Some(CargoSpec::NoCargo) => None,
                                },
                            },
                        },
                        ActionSpec::Unload {} => TaskForceOrder::Unload,
                        ActionSpec::Join { into } => {
                            TaskForceOrder::JoinTo(TaskForceId::new(*into))
                        }
                        ActionSpec::StandDown {} => TaskForceOrder::StandDown,
                        ActionSpec::NoCargoWork {} => TaskForceOrder::NoCargoWork,
                        ActionSpec::FuelFrom { giver } => {
                            TaskForceOrder::FuelFrom(TaskForceId::new(*giver))
                        }
                        ActionSpec::Escort { charge } => {
                            TaskForceOrder::Escort(charge.map(TaskForceId::new))
                        }
                        ActionSpec::FormTaskForce { .. } => {
                            return Err(wrong_target("form_task_force", "base"))
                        }
                        ActionSpec::SetRulesOfEngagement {
                            stance,
                            break_off,
                            stragglers,
                            keep_share,
                        } => TaskForceOrder::SetRulesOfEngagement {
                            stance: (*stance).into(),
                            break_off: break_off
                                .to_fixed()
                                .map_err(|e| fail(format!("break_off on day {day} {e}")))?,
                            stragglers: (*stragglers).into(),
                            keep_share: match keep_share {
                                Some(quantity) => quantity
                                    .to_fixed()
                                    .map_err(|e| fail(format!("keep_share on day {day} {e}")))?,
                                None => Fixed::ZERO,
                            },
                        },
                        ActionSpec::SetHomePort {} => {
                            TaskForceOrder::SetHomePort(BaseId::new(need_base("set_home_port")?))
                        }
                        ActionSpec::SetReadiness { readiness } => {
                            TaskForceOrder::SetReadiness((*readiness).into())
                        }
                        ActionSpec::Detach { ships } => TaskForceOrder::DetachShips(
                            ships.iter().map(|raw| ShipId::new(*raw)).collect(),
                        ),
                        ActionSpec::SendToYard {} => TaskForceOrder::SendToYard,
                        ActionSpec::SetAirMission { .. } => {
                            return Err(wrong_target("set_air_mission", "air_group"))
                        }
                        ActionSpec::SetSearchArc { .. } => {
                            return Err(wrong_target("set_search_arc", "air_group"))
                        }
                        ActionSpec::SetPatrolShare { .. } => {
                            return Err(wrong_target("set_patrol_share", "air_group"))
                        }
                        ActionSpec::SetRestThreshold { .. } => {
                            return Err(wrong_target("set_rest_threshold", "air_group"))
                        }
                        ActionSpec::SetReach { .. } => {
                            return Err(wrong_target("set_reach", "air_group"))
                        }
                        ActionSpec::SetTargetPriority { .. } => {
                            return Err(wrong_target("set_target_priority", "air_group"))
                        }
                        ActionSpec::SetStrikeContactAge { .. } => {
                            return Err(wrong_target("set_strike_contact_age", "air_group"))
                        }
                        ActionSpec::SetStrikeHeldShare { .. } => {
                            return Err(wrong_target("set_strike_held_share", "air_group"))
                        }
                        ActionSpec::SetStrikeSizing { .. } => {
                            return Err(wrong_target("set_strike_sizing", "air_group"))
                        }
                        ActionSpec::SetSearchOnce { .. } => {
                            return Err(wrong_target("set_search_once", "air_group"))
                        }
                        ActionSpec::SetStrikeFirstWaters { .. } => {
                            return Err(wrong_target("set_strike_first_waters", "air_group"))
                        }
                        ActionSpec::Rebase {} => {
                            return Err(wrong_target("rebase", "air_group"))
                        }
                        ActionSpec::SetPosture { .. } => {
                            return Err(wrong_target("set_posture", "land_unit"))
                        }
                        ActionSpec::March { .. } => return Err(wrong_target("march", "land_unit")),
                        ActionSpec::SetActivity { .. } => {
                            return Err(wrong_target("set_activity", "land_unit"))
                        }
                        ActionSpec::Station { .. } => {
                            return Err(wrong_target("station", "submarine"))
                        }
                        ActionSpec::PrepareLanding { hex: [q, r], roster } => {
                            TaskForceOrder::PrepareLanding {
                                hex: Hex::new(*q, *r),
                                roster: roster.iter().map(|raw| LandUnitId::new(*raw)).collect(),
                            }
                        }
                    };
                    OrderKind::TaskForce {
                        target: TaskForceId::new(*raw),
                        order,
                    }
                }
                (None, Some(raw), None, None, None) => {
                    let order = match &spec.action {
                        ActionSpec::SetAirMission {
                            mission,
                            target_base,
                            hex,
                        } => {
                            let mission = match (mission, target_base) {
                                (AirMissionSpec::Reconnaissance, None) => match hex {
                                    Some([q, r]) => AirMission::Reconnaissance(Hex::new(*q, *r)),
                                    None => {
                                        return Err(fail(format!(
                                            "the reconnaissance order on day {day} needs a hex"
                                        )))
                                    }
                                },
                                (_, None) if hex.is_some() => {
                                    return Err(fail(format!(
                                        "hex on day {day} is only for the reconnaissance mission"
                                    )))
                                }
                                (AirMissionSpec::BaseStrike, Some(raw)) => {
                                    let base = BaseId::new(*raw);
                                    let held_by = truth
                                        .bases
                                        .get(&base)
                                        .ok_or_else(|| {
                                            fail(format!(
                                                "the base_strike order on day {day} names unknown base {raw}"
                                            ))
                                        })?
                                        .side;
                                    // A base can change side in play (decision
                                    // 0090), so only a day-1 order is held to
                                    // the side the base has at the start; a
                                    // later one is checked when it is taken in
                                    // (`NotAnEnemyBase`). Decision 0101, as
                                    // built: the full slice orders raids on
                                    // Lunga, Japanese on day 1.
                                    if day == 1 && held_by == Side::from(spec.side) {
                                        return Err(fail(format!(
                                            "the base_strike order on day {day} names {base}, a base of the group's own side"
                                        )));
                                    }
                                    AirMission::BaseStrike(base)
                                }
                                (AirMissionSpec::BaseStrike, None) => {
                                    return Err(fail(format!(
                                        "the base_strike order on day {day} needs a target_base"
                                    )))
                                }
                                (_, Some(_)) => {
                                    return Err(fail(format!(
                                        "target_base on day {day} is only for the base_strike mission"
                                    )))
                                }
                                (AirMissionSpec::Rest, None) => AirMission::Rest,
                                (AirMissionSpec::Search, None) => AirMission::Search,
                                (AirMissionSpec::NavalStrike, None) => AirMission::NavalStrike,
                                (AirMissionSpec::CombatAirPatrol, None) => {
                                    AirMission::CombatAirPatrol
                                }
                            };
                            AirGroupOrder::SetAirMission(mission)
                        }
                        ActionSpec::SetSearchArc {
                            from_deg,
                            to_deg,
                            range_nm,
                        } => AirGroupOrder::SetSearchArc {
                            from_deg: *from_deg,
                            to_deg: *to_deg,
                            range_nm: range_nm
                                .to_fixed()
                                .map_err(|e| fail(format!("range_nm on day {day} {e}")))?,
                        },
                        ActionSpec::SetPatrolShare { share } => AirGroupOrder::SetPatrolShare {
                            share: share
                                .to_fixed()
                                .map_err(|e| fail(format!("share on day {day} {e}")))?,
                        },
                        ActionSpec::SetRestThreshold { fatigue } => {
                            AirGroupOrder::SetRestThreshold {
                                fatigue: fatigue
                                    .to_fixed()
                                    .map_err(|e| fail(format!("fatigue on day {day} {e}")))?,
                            }
                        }
                        ActionSpec::SetReach { reach } => AirGroupOrder::SetReach {
                            reach: (*reach).into(),
                        },
                        ActionSpec::SetTargetPriority { kinds } => {
                            let mut parsed = Vec::new();
                            for name in kinds {
                                parsed.push(crate::reference::parse_kind(name).ok_or_else(|| {
                                    fail(format!(
                                        "the set_target_priority order on day {day} names \"{name}\", which is not a kind of ship"
                                    ))
                                })?);
                            }
                            AirGroupOrder::SetTargetPriority { kinds: parsed }
                        }
                        ActionSpec::SetStrikeContactAge { pulses } => {
                            AirGroupOrder::SetStrikeContactAge { pulses: *pulses }
                        }
                        ActionSpec::SetStrikeHeldShare { share } => {
                            AirGroupOrder::SetStrikeHeldShare {
                                share: share
                                    .to_fixed()
                                    .map_err(|e| fail(format!("share on day {day} {e}")))?,
                            }
                        }
                        ActionSpec::SetStrikeSizing { on } => {
                            AirGroupOrder::SetStrikeSizing { on: *on }
                        }
                        ActionSpec::SetSearchOnce { on } => {
                            AirGroupOrder::SetSearchOnce { on: *on }
                        }
                        ActionSpec::SetStrikeFirstWaters { hexes } => {
                            AirGroupOrder::SetStrikeFirstWaters {
                                hexes: hexes.iter().map(|[q, r]| Hex::new(*q, *r)).collect(),
                            }
                        }
                        ActionSpec::Rebase {} => AirGroupOrder::Rebase {
                            base: BaseId::new(need_base("rebase")?),
                        },
                        _ => {
                            return Err(fail(format!(
                                "an order on day {day} for an air_group must be set_air_mission, set_search_arc, set_patrol_share, set_rest_threshold, set_reach, set_target_priority, set_strike_contact_age, set_strike_held_share, set_strike_sizing, set_search_once, set_strike_first_waters or rebase"
                            )))
                        }
                    };
                    OrderKind::AirGroup {
                        target: AirGroupId::new(*raw),
                        order,
                    }
                }
                (None, None, Some(raw), None, None) => {
                    let order = match &spec.action {
                        ActionSpec::SetPosture {
                            posture,
                            when_spent,
                        } => {
                            // Only an attack order says what the unit does
                            // when its attack is spent (decision 6 of 0105).
                            if matches!(posture, PostureSpec::Defend) && when_spent.is_some() {
                                return Err(fail(format!(
                                    "the set_posture order on day {day} gives when_spent, which only posture attack takes"
                                )));
                            }
                            LandUnitOrder::SetPosture {
                                posture: (*posture).into(),
                                when_spent: when_spent.map(Into::into),
                            }
                        }
                        ActionSpec::March { hex: [q, r] } => {
                            LandUnitOrder::March(Hex::new(*q, *r))
                        }
                        ActionSpec::SetActivity { activity, job } => {
                            let activity = match (activity, job) {
                                (ActivitySpec::Work, Some(name)) => {
                                    let job = Job::from_name(name).ok_or_else(|| {
                                        fail(format!(
                                            "the set_activity order on day {day} has job \"{name}\", which is not airfield, port, aircraft or defences"
                                        ))
                                    })?;
                                    Activity::Work(Some(job))
                                }
                                (ActivitySpec::Work, None) => Activity::Work(None),
                                (other, None) => (*other).into(),
                                (_, Some(_)) => {
                                    return Err(fail(format!(
                                        "the set_activity order on day {day} gives a job, which only the activity work takes"
                                    )))
                                }
                            };
                            LandUnitOrder::SetActivity(activity)
                        }
                        _ => {
                            return Err(fail(format!(
                                "an order on day {day} for a land_unit must be set_posture, march or set_activity"
                            )))
                        }
                    };
                    OrderKind::LandUnit {
                        target: LandUnitId::new(*raw),
                        order,
                    }
                }
                (None, None, None, Some(raw), None) => {
                    let order = match &spec.action {
                        ActionSpec::FormTaskForce { ships } => BaseOrder::FormTaskForce {
                            ships: ships.iter().map(|raw| ShipId::new(*raw)).collect(),
                        },
                        _ => {
                            return Err(fail(format!(
                                "an order on day {day} for a base must be form_task_force"
                            )))
                        }
                    };
                    OrderKind::Base {
                        target: BaseId::new(*raw),
                        order,
                    }
                }
                (None, None, None, None, Some(raw)) => {
                    let order = match &spec.action {
                        ActionSpec::Station { hex: [q, r] } => {
                            SubmarineOrder::Station(Hex::new(*q, *r))
                        }
                        _ => {
                            return Err(fail(format!(
                                "an order on day {day} for a submarine must be station"
                            )))
                        }
                    };
                    OrderKind::Submarine {
                        target: SubmarineId::new(*raw),
                        order,
                    }
                }
                _ => {
                    return Err(fail(format!(
                        "the order on day {day} needs exactly one of task_force, air_group, land_unit and base (or submarine, for a station order)"
                    )))
                }
            };
        let request = OrderRequest { kind, origin };
        match pulse {
            None => script
                .entry(spec.day)
                .or_default()
                .push((spec.side.into(), request)),
            Some(pulse) => script_inside_day.entry(spec.day).or_default().push((
                pulse,
                spec.side.into(),
                request,
            )),
        }
    }
    Ok(OrderScript {
        orders_inside_a_day,
        script,
        script_inside_day,
    })
}

pub(super) fn build_commands(
    file: &ScenarioFile,
    path: &Path,
) -> Result<BTreeMap<u32, Vec<(Side, CommandKind)>>, DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    let mut commands: BTreeMap<u32, Vec<(Side, CommandKind)>> = BTreeMap::new();
    for spec in &file.command {
        let (day, side, kind) = spec.build(file.days).map_err(&fail)?;
        commands.entry(day).or_default().push((side, kind));
    }
    Ok(commands)
}
