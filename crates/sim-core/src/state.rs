//! Truth state: the real position and condition of everything.
//!
//! Only the umpire reads this directly. A player, the staff planner or an
//! agent gets a [`crate::view::SideView`] instead.
//!
//! Only the fields scenarios 1 and 2 need exist so far: position, fuel and
//! route, and supply aboard ships and in base stocks. Damage, ammunition, air
//! groups, pilots and land units arrive with the scenarios that use them.

use std::collections::{BTreeMap, BTreeSet};

use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{BaseId, EventId, OrderId, ShipClassId, ShipId, TaskForceId};
use crate::orders::{CargoAction, Mission, SpeedSetting};

/// One of the two sides.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Side {
    Allied,
    Japanese,
}

impl Side {
    /// Both sides, in the order their orders are processed.
    pub const ALL: [Side; 2] = [Side::Allied, Side::Japanese];

    /// A stable number for the side. Part of the state hash.
    pub const fn code(self) -> u8 {
        match self {
            Side::Allied => 1,
            Side::Japanese => 2,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Side::Allied => "Allied",
            Side::Japanese => "Japanese",
        }
    }
}

/// Where a ship is. A ship is always in exactly one task force or one port.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ShipLocation {
    TaskForce(TaskForceId),
    Port(BaseId),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Ship {
    pub id: ShipId,
    pub name: String,
    pub class: ShipClassId,
    pub side: Side,
    /// Fuel aboard, in tons.
    pub fuel_t: Fixed,
    /// Supply aboard, in tons.
    pub supply_t: Fixed,
    pub location: ShipLocation,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Base {
    pub id: BaseId,
    pub name: String,
    pub side: Side,
    pub hex: Hex,
    /// Supply stock, in tons.
    pub supply_t: Fixed,
}

/// What a task force is doing about its route.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum TaskForceStatus {
    /// No route.
    Idle,
    /// Following its route.
    Underway,
    /// Reached the last waypoint.
    Arrived,
    /// Stopped short because a ship ran dry.
    OutOfFuel,
}

impl TaskForceStatus {
    /// A stable number for the status. Part of the state hash.
    pub const fn code(self) -> u8 {
        match self {
            TaskForceStatus::Idle => 1,
            TaskForceStatus::Underway => 2,
            TaskForceStatus::Arrived => 3,
            TaskForceStatus::OutOfFuel => 4,
        }
    }
}

/// The waypoints a task force is steaming to.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Route {
    pub waypoints: Vec<Hex>,
    /// Index of the waypoint being steamed toward. Equal to the number of
    /// waypoints once the route is complete.
    pub next_index: usize,
}

/// A standing cargo order: what to do, and the order that set it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CargoOrder {
    pub action: CargoAction,
    pub order: OrderId,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TaskForce {
    pub id: TaskForceId,
    pub name: String,
    pub side: Side,
    pub ships: BTreeSet<ShipId>,
    pub mission: Mission,
    pub speed: SpeedSetting,
    pub route: Route,
    /// The hex the task force is in.
    pub hex: Hex,
    /// Nautical miles steamed from the centre of `hex` toward the next one.
    /// Always at least 0 and less than one hex.
    pub progress_nm: Fixed,
    pub status: TaskForceStatus,
    /// The standing order that set the route, if any.
    pub route_order: Option<OrderId>,
    /// The standing order that set the speed, if any.
    pub speed_order: Option<OrderId>,
    /// The standing cargo order and the order that set it, if any.
    pub cargo_order: Option<CargoOrder>,
    /// Elapsed hours since the scenario began at which the task force last
    /// arrived at the end of its route. Cleared by a new route.
    pub arrived_at_hours: Option<Fixed>,
    /// The `TaskForceArrived` event of that arrival.
    pub arrival_event: Option<EventId>,
}

impl TaskForce {
    /// A task force sitting at a hex with no route.
    pub fn at(id: TaskForceId, name: String, side: Side, hex: Hex, mission: Mission) -> TaskForce {
        TaskForce {
            id,
            name,
            side,
            ships: BTreeSet::new(),
            mission,
            speed: SpeedSetting::Cruise,
            route: Route::default(),
            hex,
            progress_nm: Fixed::ZERO,
            status: TaskForceStatus::Idle,
            route_order: None,
            speed_order: None,
            cargo_order: None,
            arrived_at_hours: None,
            arrival_event: None,
        }
    }
}

/// The real state of the world.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct TruthState {
    /// The last day resolved. 0 before the first day.
    pub day: u32,
    pub ships: BTreeMap<ShipId, Ship>,
    pub task_forces: BTreeMap<TaskForceId, TaskForce>,
    pub bases: BTreeMap<BaseId, Base>,
}
