//! Truth state: the real position and condition of everything.
//!
//! Only the umpire reads this directly. A player, the staff planner or an
//! agent gets a [`crate::view::SideView`] instead.
//!
//! Only the fields the scenarios so far need exist: position, fuel and
//! route, supply aboard ships and in base stocks, air groups (base,
//! aircraft ready, mission and search arc), and for surface combat each
//! ship's damage tracks, named systems and ammunition and each task force's alertness and
//! rules of engagement. Pilots and land units arrive with the scenarios that
//! use them.

use std::collections::{BTreeMap, BTreeSet};

use crate::fixed::Fixed;
use crate::hex::{Hex, HEX_NM};
use crate::ids::{
    AirGroupId, AircraftTypeId, BaseId, EventId, OrderId, ShipClassId, ShipId, TaskForceId,
};
use crate::orders::{AirMission, CargoAction, Mission, SpeedSetting, Stance, StragglerInstruction};

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

/// Where a ship is. A ship is always in exactly one task force, in exactly
/// one port, or sunk.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ShipLocation {
    TaskForce(TaskForceId),
    Port(BaseId),
    /// Sunk. In no task force and no port, and out of every rule.
    Sunk,
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
    /// Flotation damage, 0 to 100. At 100 the ship sinks.
    pub flotation_damage: Fixed,
    /// Fire damage, 0 to 100. Any fire lights the ship at night.
    pub fire_damage: Fixed,
    /// Gun rounds left aboard. Never negative.
    pub gun_rounds: Fixed,
    /// True while the ship still has its torpedoes to fire.
    pub torpedoes_ready: bool,
    /// The state of each named system (decision 0016).
    pub systems: ShipSystems,
}

/// A named system of a ship that a hit can knock down (decision 0016).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ShipSystem {
    Propulsion,
    FireControl,
    MainBattery,
    TorpedoMounts,
}

impl ShipSystem {
    /// Every system, in code order.
    pub const ALL: [ShipSystem; 4] = [
        ShipSystem::Propulsion,
        ShipSystem::FireControl,
        ShipSystem::MainBattery,
        ShipSystem::TorpedoMounts,
    ];

    /// A stable number for the system. Part of the state hash and of the
    /// order of the system hit bands. Never renumber it; add new systems at
    /// the end.
    pub const fn code(self) -> u8 {
        match self {
            ShipSystem::Propulsion => 1,
            ShipSystem::FireControl => 2,
            ShipSystem::MainBattery => 3,
            ShipSystem::TorpedoMounts => 4,
        }
    }

    /// The system's name as the event log writes it.
    pub const fn name(self) -> &'static str {
        match self {
            ShipSystem::Propulsion => "propulsion",
            ShipSystem::FireControl => "fire control",
            ShipSystem::MainBattery => "main battery",
            ShipSystem::TorpedoMounts => "torpedo mounts",
        }
    }
}

/// How well a system works.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum SystemState {
    Working,
    Degraded,
    Out,
}

impl SystemState {
    /// A stable number for the state. Part of the state hash. Never
    /// renumber it.
    pub const fn code(self) -> u8 {
        match self {
            SystemState::Working => 1,
            SystemState::Degraded => 2,
            SystemState::Out => 3,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            SystemState::Working => "working",
            SystemState::Degraded => "degraded",
            SystemState::Out => "out",
        }
    }

    /// One level worse: working to degraded, degraded to out. Out stays out.
    pub const fn step_down(self) -> SystemState {
        match self {
            SystemState::Working => SystemState::Degraded,
            SystemState::Degraded | SystemState::Out => SystemState::Out,
        }
    }
}

/// The state of every named system of one ship. A system the ship's class
/// does not have stays working.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ShipSystems {
    pub propulsion: SystemState,
    pub fire_control: SystemState,
    pub main_battery: SystemState,
    pub torpedo_mounts: SystemState,
}

impl ShipSystems {
    /// Every system working.
    pub const ALL_WORKING: ShipSystems = ShipSystems {
        propulsion: SystemState::Working,
        fire_control: SystemState::Working,
        main_battery: SystemState::Working,
        torpedo_mounts: SystemState::Working,
    };

    pub const fn get(&self, system: ShipSystem) -> SystemState {
        match system {
            ShipSystem::Propulsion => self.propulsion,
            ShipSystem::FireControl => self.fire_control,
            ShipSystem::MainBattery => self.main_battery,
            ShipSystem::TorpedoMounts => self.torpedo_mounts,
        }
    }

    pub fn set(&mut self, system: ShipSystem, state: SystemState) {
        match system {
            ShipSystem::Propulsion => self.propulsion = state,
            ShipSystem::FireControl => self.fire_control = state,
            ShipSystem::MainBattery => self.main_battery = state,
            ShipSystem::TorpedoMounts => self.torpedo_mounts = state,
        }
    }
}

impl Default for ShipSystems {
    fn default() -> ShipSystems {
        ShipSystems::ALL_WORKING
    }
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

/// How a task force behaves in a surface fight: whether it fights or tries
/// to slip away, and how much loss makes it break off.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RulesOfEngagement {
    pub stance: Stance,
    /// The share of ships sunk or crippled, 0 to 1, at which the force
    /// breaks off.
    pub break_off: Fixed,
    /// What to do about stragglers (decision 11 of
    /// `docs/decisions/0016-systems-and-stragglers.md`).
    pub stragglers: StragglerInstruction,
    /// The share, 0 to 1, of the ships with working propulsion the force
    /// keeps when it detaches stragglers.
    pub keep_share: Fixed,
}

impl RulesOfEngagement {
    /// Engage, break off at half the ships lost or crippled, keep stragglers.
    pub const DEFAULT: RulesOfEngagement = RulesOfEngagement {
        stance: Stance::Engage,
        break_off: Fixed::from_raw(500_000),
        stragglers: StragglerInstruction::Keep,
        keep_share: Fixed::ZERO,
    };
}

impl Default for RulesOfEngagement {
    fn default() -> RulesOfEngagement {
        RulesOfEngagement::DEFAULT
    }
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
    /// The hex whose centre the task force last left, or is sitting on.
    /// Read only by movement, route intake, the hash and the leg invariants.
    /// Everything else asks [`TaskForce::position`].
    pub from_hex: Hex,
    /// The hex the task force is steaming toward. `Some` exactly when
    /// `progress_nm` is above zero, and then always a neighbour of
    /// `from_hex`.
    pub toward_hex: Option<Hex>,
    /// Nautical miles covered from the centre of `from_hex` toward the centre
    /// of `toward_hex`. At least 0 and less than one hex.
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
    /// Elapsed hours since the scenario began at the start of the pulse of
    /// the latest sighting this task force noticed (decision 17 of
    /// `docs/decisions/0012-search-and-contacts.md`).
    pub noticed_sighted_at_hours: Option<Fixed>,
    /// Whether the task force is alert (decision 6 of
    /// `docs/decisions/0014-surface-combat.md`).
    pub alert: bool,
    /// The standing rules of engagement.
    pub rules_of_engagement: RulesOfEngagement,
    /// The standing order that set the rules of engagement, if any.
    pub rules_order: Option<OrderId>,
    /// The base the force heads for when it retires or sends stragglers home
    /// (decision 0016). It must be a base of the force's own side.
    pub home_port: Option<BaseId>,
}

impl TaskForce {
    /// Where the task force is: the hex whose centre is nearest. That is
    /// the hex ahead once it is half a hex (20 nm) or more along its leg,
    /// and the hex it left before that (decision 0005, decision 1).
    pub fn position(&self) -> Hex {
        match self.toward_hex {
            Some(toward) if self.progress_nm >= Fixed::from_int(HEX_NM) / Fixed::from_int(2) => {
                toward
            }
            _ => self.from_hex,
        }
    }

    /// Where a new route starts when it is accepted now, and whether the task
    /// force turns back to get there (decision 0005, decision 5).
    ///
    /// At a centre this is `(false, from_hex)`. Between centres the force
    /// goes on to the hex ahead or turns back to the hex it left, whichever
    /// is the shorter trip to `first_waypoint`. Going on costs the rest of
    /// the leg plus the hexes from the hex ahead. Turning back costs the
    /// miles already covered plus the hexes from the hex it left. A tie goes
    /// ahead. With no waypoint there is nothing to compare, so it goes ahead.
    ///
    /// Pure: used by both intake validation and applying the order.
    pub fn reroute_centre(&self, first_waypoint: Option<Hex>) -> (bool, Hex) {
        let Some(toward) = self.toward_hex else {
            return (false, self.from_hex);
        };
        let Some(waypoint) = first_waypoint else {
            return (false, toward);
        };
        let one_hex = Fixed::from_int(HEX_NM);
        let cost_ahead =
            (one_hex - self.progress_nm) + one_hex * i64::from(toward.distance(waypoint));
        let cost_back = self.progress_nm + one_hex * i64::from(self.from_hex.distance(waypoint));
        if cost_back < cost_ahead {
            (true, self.from_hex)
        } else {
            (false, toward)
        }
    }

    /// Turns the task force round mid-leg: the two hexes swap and the miles
    /// covered become the miles still to go. Does nothing at a centre.
    pub fn turn_back(&mut self) {
        if let Some(toward) = self.toward_hex {
            self.toward_hex = Some(self.from_hex);
            self.from_hex = toward;
            self.progress_nm = Fixed::from_int(HEX_NM) - self.progress_nm;
        }
    }

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
            from_hex: hex,
            toward_hex: None,
            progress_nm: Fixed::ZERO,
            status: TaskForceStatus::Idle,
            route_order: None,
            speed_order: None,
            cargo_order: None,
            arrived_at_hours: None,
            arrival_event: None,
            noticed_sighted_at_hours: None,
            alert: true,
            rules_of_engagement: RulesOfEngagement::DEFAULT,
            rules_order: None,
            home_port: None,
        }
    }
}

/// The arc an air group searches: from one bearing clockwise to another, in
/// degrees, out to a range. Equal bearings mean the whole circle.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SearchArc {
    pub from_deg: u32,
    pub to_deg: u32,
    pub range_nm: Fixed,
}

/// A group of aircraft of one type, based at one base.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AirGroup {
    pub id: AirGroupId,
    pub name: String,
    pub side: Side,
    pub base: BaseId,
    pub aircraft_type: AircraftTypeId,
    /// Aircraft ready to fly. Unsigned so that it cannot go negative.
    pub ready: u32,
    pub mission: AirMission,
    pub search: Option<SearchArc>,
    /// The standing order that set the mission, if any.
    pub mission_order: Option<OrderId>,
    /// The standing order that set the search arc, if any.
    pub arc_order: Option<OrderId>,
}

/// The real state of the world.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct TruthState {
    /// The last day resolved. 0 before the first day.
    pub day: u32,
    pub ships: BTreeMap<ShipId, Ship>,
    pub task_forces: BTreeMap<TaskForceId, TaskForce>,
    pub bases: BTreeMap<BaseId, Base>,
    pub air_groups: BTreeMap<AirGroupId, AirGroup>,
}
