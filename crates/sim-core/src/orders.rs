//! Unit-level orders: the only way to influence the simulation.
//!
//! Only the task force orders scenarios 1 and 2 need exist so far (route,
//! speed, mission, load and unload). The spec lists the full set (form, disband, rules of engagement, air group orders and so
//! on). They arrive with the scenarios that use them.

use core::fmt;

use crate::hex::Hex;
use crate::ids::{EntityRef, IntentId, OrderId, TaskForceId};
use crate::state::Side;

/// Who issued an order.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Issuer {
    Player,
    Staff,
}

/// Where an order came from. Stored and logged in step 1. The staff planner
/// and the UI depend on it later to answer "why did this happen".
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Origin {
    pub issuer: Issuer,
    /// The intent this order serves, once intents exist.
    pub intent: Option<IntentId>,
}

impl Origin {
    /// An order given directly by the player, serving no recorded intent.
    pub const PLAYER: Origin = Origin {
        issuer: Issuer::Player,
        intent: None,
    };
}

/// What a task force is out to do.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Mission {
    Transport,
    SurfaceCombat,
    Carrier,
    Bombardment,
    Escort,
}

/// How fast a task force steams.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum SpeedSetting {
    /// The cruise speed of the slowest ship.
    Cruise,
    /// The top speed of the slowest ship.
    Full,
}

/// What a standing cargo order does at a port.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum CargoAction {
    Load,
    Unload,
}

impl CargoAction {
    /// A stable number for the action. Part of the state hash. Never
    /// renumber it.
    pub const fn code(self) -> u8 {
        match self {
            CargoAction::Load => 1,
            CargoAction::Unload => 2,
        }
    }
}

/// An order to a task force.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TaskForceOrder {
    /// Steam to each waypoint in turn and stop at the last.
    SetRoute(Vec<Hex>),
    SetSpeed(SpeedSetting),
    SetMission(Mission),
    /// Fill every ship to its cargo capacity from the stock of the base here.
    /// A standing order of the "cargo" kind: it replaces an unload order, and
    /// neither cancels the route or the speed.
    Load,
    /// Move all supply aboard into the stock of the base where the task force
    /// stops. A standing order of the "cargo" kind: it replaces a load order,
    /// and neither cancels the route or the speed.
    Unload,
}

/// An order together with the unit it is for.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum OrderKind {
    TaskForce {
        target: TaskForceId,
        order: TaskForceOrder,
    },
}

impl OrderKind {
    /// The unit the order is addressed to.
    pub fn target(&self) -> EntityRef {
        match self {
            OrderKind::TaskForce { target, .. } => EntityRef::TaskForce(*target),
        }
    }
}

/// An order as a side submits it, before the simulation has accepted it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OrderRequest {
    pub kind: OrderKind,
    pub origin: Origin,
}

/// An order the simulation has received and numbered. Both accepted and
/// rejected orders are kept. The event log says which it was.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Order {
    pub id: OrderId,
    pub side: Side,
    /// The day the order took effect.
    pub day_issued: u32,
    pub kind: OrderKind,
    pub origin: Origin,
}

/// Why an order was rejected at intake.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RejectReason {
    /// The target does not exist.
    UnknownTarget,
    /// The target belongs to the other side.
    NotYourUnit,
    /// The route has no waypoint other than where the task force already is.
    EmptyRoute,
    /// A waypoint is not on the map.
    WaypointOffMap(Hex),
    /// The route would pass through a hex ships cannot enter.
    RouteBlocked(Hex),
    /// A load order needs a task force that is stopped.
    UnderWay,
    /// A load order needs a base of the side in the task force's hex.
    NoOwnBaseHere,
}

impl fmt::Display for RejectReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RejectReason::UnknownTarget => f.write_str("the target does not exist"),
            RejectReason::NotYourUnit => f.write_str("the target belongs to the other side"),
            RejectReason::EmptyRoute => f.write_str("the route goes nowhere"),
            RejectReason::WaypointOffMap(hex) => write!(f, "waypoint {hex} is off the map"),
            RejectReason::RouteBlocked(hex) => {
                write!(
                    f,
                    "the route passes through {hex}, which ships cannot enter"
                )
            }
            RejectReason::UnderWay => f.write_str("the task force is under way"),
            RejectReason::NoOwnBaseHere => f.write_str("there is no base of your side here"),
        }
    }
}
