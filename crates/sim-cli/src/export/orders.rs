//! Orders as JSON.

use serde::Serialize;
use sim_core::ids::UmpireKey;
use sim_core::orders::{
    AirGroupOrder, AirMission, LandUnitOrder, Order, OrderKind, TaskForceOrder,
};

use super::names;

#[derive(Serialize)]
pub struct OrderOut {
    id: u32,
    side: &'static str,
    /// The day the order took effect.
    day: u32,
    issuer: &'static str,
    /// An entity string such as `task_force:1`.
    target: String,
    text: String,
}

pub fn order(order: &Order, key: &UmpireKey) -> OrderOut {
    OrderOut {
        id: order.id.umpire_raw(key),
        side: names::side(order.side),
        day: order.day_issued,
        issuer: names::issuer(order.origin.issuer),
        target: order.kind.target().to_string(),
        text: text(&order.kind),
    }
}

/// A short description of what the order asks for.
fn text(kind: &OrderKind) -> String {
    match kind {
        OrderKind::TaskForce { order, .. } => match order {
            TaskForceOrder::SetRoute { waypoints, act_at } => {
                let waypoints: Vec<String> = waypoints.iter().map(ToString::to_string).collect();
                let text = format!("route via {}", waypoints.join(", "));
                if act_at.is_empty() {
                    text
                } else {
                    // Decision 0096: the waypoints marked to act at.
                    let marks: Vec<String> = act_at.iter().map(ToString::to_string).collect();
                    format!("{text}, acting at {}", marks.join(", "))
                }
            }
            TaskForceOrder::SetSpeed(speed) => format!("speed {}", names::speed(*speed)),
            TaskForceOrder::SetMission(mission) => format!("mission {}", names::mission(*mission)),
            TaskForceOrder::Load => "load supply from the base here".to_string(),
            TaskForceOrder::LoadFuel => "load fuel cargo from the base here".to_string(),
            TaskForceOrder::Unload => "unload supply into the base here".to_string(),
            TaskForceOrder::SetRulesOfEngagement {
                stance,
                break_off,
                stragglers,
                keep_share,
            } => format!(
                "rules of engagement: {}, break off at {break_off}, stragglers {}, keep share {keep_share}",
                names::stance(*stance),
                stragglers.name()
            ),
            TaskForceOrder::SetHomePort(base) => format!("home port {base}"),
            TaskForceOrder::SetReadiness(readiness) => format!("readiness {}", readiness.name()),
            TaskForceOrder::SendToYard => "send to the yard".to_string(),
            TaskForceOrder::LoadUnit { unit, cargo } => match cargo {
                Some(cargo) => format!("load {unit} and {} from the base here", cargo.name()),
                None => format!("load {unit} (men only)"),
            },
            TaskForceOrder::DetachShips(ships) => {
                let ships: Vec<String> = ships.iter().map(ToString::to_string).collect();
                format!("detach {}", ships.join(", "))
            }
            TaskForceOrder::PrepareLanding { hex, roster } => {
                let units: Vec<String> = roster.iter().map(ToString::to_string).collect();
                if units.is_empty() {
                    format!("prepare a landing at {hex}")
                } else {
                    format!("prepare a landing at {hex} with {}", units.join(", "))
                }
            }
        },
        OrderKind::AirGroup { order, .. } => match order {
            AirGroupOrder::SetAirMission(AirMission::BaseStrike(base)) => {
                format!("air mission base_strike on {base}")
            }
            AirGroupOrder::SetAirMission(mission) => {
                format!("air mission {}", names::air_mission(*mission))
            }
            AirGroupOrder::SetSearchArc {
                from_deg,
                to_deg,
                range_nm,
            } => format!("search {from_deg:03}-{to_deg:03} out to {range_nm} nm"),
            AirGroupOrder::SetPatrolShare { share } => format!("patrol share {share}"),
            AirGroupOrder::SetRestThreshold { fatigue } => format!("rest above {fatigue}"),
            AirGroupOrder::SetReach { reach } => format!("reach {}", reach.name()),
            AirGroupOrder::SetTargetPriority { kinds } => {
                let kinds: Vec<&str> = kinds.iter().map(|kind| kind.name()).collect();
                format!("target priority {}", kinds.join(", "))
            }
            AirGroupOrder::Rebase { base } => format!("rebase to {base}"),
        },
        OrderKind::LandUnit { order, .. } => match order {
            LandUnitOrder::SetPosture {
                posture,
                when_spent: None,
            } => format!("posture {}", posture.name()),
            LandUnitOrder::SetPosture {
                posture,
                when_spent: Some(when_spent),
            } => format!(
                "posture {}, when spent {}",
                posture.name(),
                when_spent.name()
            ),
            LandUnitOrder::March(hex) => format!("march to {hex}"),
            LandUnitOrder::SetActivity(activity) => format!("activity {}", activity.name()),
        },
    }
}
