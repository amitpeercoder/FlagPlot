//! Unit-level orders: the only way to influence the simulation.
//!
//! Only the orders the scenarios so far need exist: task force orders (route,
//! speed, mission, load, unload and rules of engagement) and air group orders
//! (mission and search arc). The spec lists the full set (form, disband and
//! so on). They arrive with the scenarios that use them.

use core::fmt;

use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{AirGroupId, EntityRef, IntentId, OrderId, TaskForceId};
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

/// What an air group is doing.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub enum AirMission {
    /// Stays on the ground.
    #[default]
    Rest,
    /// Searches its arc.
    Search,
}

impl AirMission {
    /// A stable number for the mission. Part of the state hash. Never
    /// renumber it.
    pub const fn code(self) -> u8 {
        match self {
            AirMission::Rest => 1,
            AirMission::Search => 2,
        }
    }
}

/// How fast a task force steams.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum SpeedSetting {
    /// The cruise speed of the slowest ship.
    Cruise,
    /// The top speed of the slowest ship.
    Full,
}

/// Whether a task force fights or tries to slip away from a surface fight.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Stance {
    /// Fights when it can.
    Engage,
    /// Slips away from an enemy that has not sighted it, and breaks off at
    /// the end of the first round in which it is fired on.
    Avoid,
}

impl Stance {
    /// A stable number for the stance. Part of the state hash. Never
    /// renumber it; add new stances at the end.
    pub const fn code(self) -> u8 {
        match self {
            Stance::Engage => 1,
            Stance::Avoid => 2,
        }
    }
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
    /// Set the rules of engagement. A standing order of its own kind: it
    /// replaces the rules of engagement and nothing else. `break_off` must
    /// be from 0 to 1.
    SetRulesOfEngagement {
        stance: Stance,
        break_off: Fixed,
    },
}

/// An order to an air group. Each is a standing order of its own kind: a new
/// mission order replaces the mission and leaves the arc alone, and a new arc
/// order replaces the arc and leaves the mission alone.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AirGroupOrder {
    SetAirMission(AirMission),
    /// Search from `from_deg` clockwise to `to_deg`, out to `range_nm`.
    /// Bearings are signed so that a negative one reaches intake and is
    /// rejected there.
    SetSearchArc {
        from_deg: i32,
        to_deg: i32,
        range_nm: Fixed,
    },
}

/// An order together with the unit it is for.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum OrderKind {
    TaskForce {
        target: TaskForceId,
        order: TaskForceOrder,
    },
    AirGroup {
        target: AirGroupId,
        order: AirGroupOrder,
    },
}

impl OrderKind {
    /// The unit the order is addressed to.
    pub fn target(&self) -> EntityRef {
        match self {
            OrderKind::TaskForce { target, .. } => EntityRef::TaskForce(*target),
            OrderKind::AirGroup { target, .. } => EntityRef::AirGroup(*target),
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
    /// The target is not a unit of the issuing side. The same reason covers
    /// a unit that does not exist and one that belongs to the enemy, so a
    /// rejection never reveals that an enemy unit exists (decision 0008).
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
    /// A search arc bearing is not a multiple of 10 from 0 to 350.
    InvalidBearing(i32),
    /// A search range is zero, negative, or beyond the aircraft type's
    /// search range.
    InvalidSearchRange { range_nm: Fixed, max_nm: Fixed },
    /// A break-off level is below 0 or above 1.
    BreakOffOutOfRange(Fixed),
}

impl fmt::Display for RejectReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RejectReason::NotYourUnit => f.write_str("the target is not one of your units"),
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
            RejectReason::InvalidBearing(bearing) => {
                write!(f, "bearing {bearing} is not a multiple of 10 from 0 to 350")
            }
            RejectReason::InvalidSearchRange { range_nm, max_nm } => write!(
                f,
                "search range {range_nm} nm is not above 0 and at most the aircraft's {max_nm} nm"
            ),
            RejectReason::BreakOffOutOfRange(level) => {
                write!(f, "break-off level {level} is not between 0 and 1")
            }
        }
    }
}
