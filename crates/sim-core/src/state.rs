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
use std::fmt;

use crate::fixed::Fixed;
use crate::hex::{Hex, HEX_NM};
use crate::ids::{
    AirGroupId, AircraftTypeId, BaseId, EventId, LandUnitId, OrderId, ShipClassId, ShipId,
    TaskForceId,
};
use crate::map::Map;
use crate::orders::{
    Activity, AirMission, CargoAction, Mission, Posture, Reach, Readiness, RejectReason,
    SpeedSetting, Stance, StragglerInstruction,
};
use crate::refdata::ShipKind;

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
    /// Away at a yard with the task force named (decision 0069). The ship is
    /// afloat but in no force's `ships` set; the force keeps its place and
    /// `yard_trip`, and takes her back on its day of return. Added at the
    /// end: the hash code is 4.
    Yard(TaskForceId),
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
    /// Fuel carried as cargo, in tons (decision 0046). Not the fuel the ship
    /// steams on: only landing it at a base makes it usable. Supply and fuel
    /// cargo together never exceed the class's cargo capacity.
    pub cargo_fuel_t: Fixed,
    pub location: ShipLocation,
    /// Flotation damage, 0 to 100. At 100 the ship sinks.
    pub flotation_damage: Fixed,
    /// Fire damage, 0 to 100. Any fire lights the ship at night.
    pub fire_damage: Fixed,
    /// Gun rounds left aboard. Never negative.
    pub gun_rounds: Fixed,
    /// True while the ship still has its torpedoes to fire.
    pub torpedoes_ready: bool,
    /// True once the ship has fired the first side of a class whose
    /// `torpedo_tubes` is above `torpedo_tubes_one_side`, and until she fires
    /// the other side (decision 0083). Set only while `torpedoes_ready` is
    /// set and only on such a class.
    pub torpedoes_one_side_fired: bool,
    /// The state of each named system (decision 0016).
    pub systems: ShipSystems,
    /// The crew's experience, a whole number from 0 to
    /// [`Ship::MAX_EXPERIENCE`] (decision 0064). Set when the scenario loads
    /// and written by no rule. Three chances of a dark surface fight read it.
    pub experience: u8,
    /// Points already put into the first degraded system, in system order,
    /// that a forward repair base has not finished (decision 0069). From 0
    /// to below `forward_repair_system_points`.
    pub system_repair_points: Fixed,
    /// The men aboard and the land unit they belong to (decision 0066). A
    /// ship carries men of one unit only, never more than its class's troop
    /// capacity, and `Some` always holds at least one man.
    pub troops: Option<Troops>,
}

/// Men of one land unit aboard a ship (decision 0066, 2).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Troops {
    pub unit: LandUnitId,
    /// At least 1.
    pub men: u32,
}

impl ShipLocation {
    /// Whether the ship is afloat and on the map: in a task force or a port.
    /// A ship at the yard is afloat but off the map (decision 0069), and a
    /// sunk ship is neither.
    pub const fn is_on_the_map(self) -> bool {
        matches!(self, ShipLocation::TaskForce(_) | ShipLocation::Port(_))
    }
}

impl Ship {
    /// A crew trained to its service's standard, with nothing in the record
    /// for or against it. A placeholder ship has this value.
    pub const DEFAULT_EXPERIENCE: u8 = 2;
    /// The highest experience a ship may have.
    pub const MAX_EXPERIENCE: u8 = 4;

    /// A ship with nothing aboard and nothing damaged: no fuel, supply or
    /// cargo, no gun rounds, torpedoes spent, every system working. Callers
    /// set the figures they care about; production code that builds a ship
    /// from data writes the full literal instead, so a new field is a
    /// compile error there.
    pub fn new(
        id: ShipId,
        name: String,
        class: ShipClassId,
        side: Side,
        location: ShipLocation,
    ) -> Ship {
        Ship {
            id,
            name,
            class,
            side,
            fuel_t: Fixed::ZERO,
            supply_t: Fixed::ZERO,
            cargo_fuel_t: Fixed::ZERO,
            location,
            flotation_damage: Fixed::ZERO,
            fire_damage: Fixed::ZERO,
            gun_rounds: Fixed::ZERO,
            torpedoes_ready: false,
            torpedoes_one_side_fired: false,
            systems: ShipSystems::ALL_WORKING,
            experience: Ship::DEFAULT_EXPERIENCE,
            system_repair_points: Fixed::ZERO,
            troops: None,
        }
    }
}

/// A named system of a ship that a hit can knock down (decision 0016).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ShipSystem {
    Propulsion,
    FireControl,
    MainBattery,
    TorpedoMounts,
    /// Matters only on a ship whose class carries aircraft (decisions 13 to
    /// 15 of `docs/decisions/0026-air-strikes.md`).
    FlightDeck,
}

impl ShipSystem {
    /// Every system, in code order.
    pub const ALL: [ShipSystem; 5] = [
        ShipSystem::Propulsion,
        ShipSystem::FireControl,
        ShipSystem::MainBattery,
        ShipSystem::TorpedoMounts,
        ShipSystem::FlightDeck,
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
            ShipSystem::FlightDeck => 5,
        }
    }

    /// The system's name as the event log writes it.
    pub const fn name(self) -> &'static str {
        match self {
            ShipSystem::Propulsion => "propulsion",
            ShipSystem::FireControl => "fire control",
            ShipSystem::MainBattery => "main battery",
            ShipSystem::TorpedoMounts => "torpedo mounts",
            ShipSystem::FlightDeck => "flight deck",
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
    pub flight_deck: SystemState,
}

impl ShipSystems {
    /// Every system working.
    pub const ALL_WORKING: ShipSystems = ShipSystems {
        propulsion: SystemState::Working,
        fire_control: SystemState::Working,
        main_battery: SystemState::Working,
        torpedo_mounts: SystemState::Working,
        flight_deck: SystemState::Working,
    };

    pub const fn get(&self, system: ShipSystem) -> SystemState {
        match system {
            ShipSystem::Propulsion => self.propulsion,
            ShipSystem::FireControl => self.fire_control,
            ShipSystem::MainBattery => self.main_battery,
            ShipSystem::TorpedoMounts => self.torpedo_mounts,
            ShipSystem::FlightDeck => self.flight_deck,
        }
    }

    pub fn set(&mut self, system: ShipSystem, state: SystemState) {
        match system {
            ShipSystem::Propulsion => self.propulsion = state,
            ShipSystem::FireControl => self.fire_control = state,
            ShipSystem::MainBattery => self.main_battery = state,
            ShipSystem::TorpedoMounts => self.torpedo_mounts = state,
            ShipSystem::FlightDeck => self.flight_deck = state,
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
    /// Anti-aircraft shots the base fires at a raid on it (decision 5 of
    /// `docs/decisions/0034-raids-on-a-base.md`). The default is 0.
    pub anti_aircraft: u32,
    /// Damaged aircraft the base repairs each day, for the groups located
    /// there, at the start of the day (decision 9 of
    /// `docs/decisions/0035-wear-repair-and-operational-losses.md`). The
    /// default is 0.
    pub aircraft_repairs_per_day: u32,
    /// Fuel stock for ships, in tons (decision 0045). The default is 0: a
    /// base with none refuels nobody.
    pub fuel_t: Fixed,
    /// The points of damage the base mends in a day, shared by every force of
    /// its side stopped there (decision 0069). The default is 0.
    pub ship_repair_points_per_day: Fixed,
    /// `Some(n)` makes the base a port for the yard, with a passage there and
    /// back of n days, n at least 1 (decision 0069). The default is `None`.
    pub yard_passage_days: Option<u32>,
    /// Whether the base is on the map both sides are given (decision 0066,
    /// 12): true for every base a scenario starts with, false for one formed
    /// by a landing, which only its own side knows of.
    pub on_map: bool,
}

impl Base {
    /// A base with no supply or fuel, no anti-aircraft fire and no repairs.
    pub fn new(id: BaseId, name: String, side: Side, hex: Hex) -> Base {
        Base {
            id,
            name,
            side,
            hex,
            supply_t: Fixed::ZERO,
            anti_aircraft: 0,
            aircraft_repairs_per_day: 0,
            fuel_t: Fixed::ZERO,
            ship_repair_points_per_day: Fixed::ZERO,
            yard_passage_days: None,
            on_map: true,
        }
    }
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
    /// Stopped at a waypoint marked to act at, with work there (decision
    /// 0096, 2). [`TaskForce::halt`] says what it waits for. Added at the
    /// end.
    Halted,
}

impl TaskForceStatus {
    /// A stable number for the status. Part of the state hash.
    pub const fn code(self) -> u8 {
        match self {
            TaskForceStatus::Idle => 1,
            TaskForceStatus::Underway => 2,
            TaskForceStatus::Arrived => 3,
            TaskForceStatus::OutOfFuel => 4,
            TaskForceStatus::Halted => 5,
        }
    }
}

/// The waypoints a task force is steaming to. Between waypoints it follows
/// the shortest path over hexes ships can enter (decision 0017); the path
/// itself is not stored, only the waypoints and which one is next.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Route {
    pub waypoints: Vec<Hex>,
    /// Index of the waypoint being steamed toward. Equal to the number of
    /// waypoints once the route is complete.
    pub next_index: usize,
    /// The waypoints marked to act at (decision 0096, 1), by hex: every
    /// waypoint at one of these hexes is marked. Each is one of the
    /// waypoints. Empty for a route with no mark.
    pub act_at: BTreeSet<Hex>,
}

impl Route {
    /// Whether the waypoint at `index` is marked to act at. The last
    /// waypoint never is: a force stops there anyway (decision 0096, 1).
    pub fn marked(&self, index: usize) -> bool {
        index + 1 < self.waypoints.len()
            && self
                .waypoints
                .get(index)
                .is_some_and(|hex| self.act_at.contains(hex))
    }
}

/// What a halted force waits to do before it steams on (decision 0096, 2):
/// a bombardment, cargo, or both.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct HaltWork {
    pub bombard: bool,
    pub cargo: bool,
}

impl HaltWork {
    /// Either kind of work.
    pub fn any(self) -> bool {
        self.bombard || self.cargo
    }
}

/// A task force's halt at a marked waypoint (decision 0096). `Some` exactly
/// when the force's status is [`TaskForceStatus::Halted`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Halt {
    /// The work it halted for.
    pub work: HaltWork,
    /// Elapsed hours since the scenario began at which it halted. Its cargo
    /// is worked from this hour, as an arrived force's is from its arrival.
    pub at_hours: Fixed,
    /// The `TaskForceHalted` event.
    pub event: EventId,
    /// The force's `ShoreBombardment` since it halted, if it has bombarded.
    pub bombarded: Option<EventId>,
    /// The completion event of its cargo order since it halted, if one was
    /// recorded.
    pub cargo_completed: Option<EventId>,
}

/// A standing cargo order: what to do, and the order that set it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CargoOrder {
    pub action: CargoAction,
    pub order: OrderId,
    /// The land unit a load order takes aboard with its cargo (decision
    /// 0066, 5). `None` for a load with no unit and for an unload.
    pub unit: Option<LandUnitId>,
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
    ///
    /// A force is always in a navigable hex: this one, and `toward_hex` when
    /// set, are hexes ships can enter (decision 0017; an invariant checks it).
    pub from_hex: Hex,
    /// The hex the task force is steaming toward. `Some` exactly when
    /// `progress_nm` is above zero, and then always a neighbour of
    /// `from_hex` that ships can enter.
    pub toward_hex: Option<Hex>,
    /// Nautical miles covered from the centre of `from_hex` toward the centre
    /// of `toward_hex`. At least 0 and less than `leg_nm`.
    pub progress_nm: Fixed,
    /// What the step in progress is charged, in miles (decision 0058, 7):
    /// set when a step begins, from the map's charge of the step
    /// ([`Map::step_charge_nm`]). 1 to 40, and 40 for a force at a centre.
    /// `progress_nm` runs from 0 to this, and the force is in the hex ahead
    /// from half of it.
    pub leg_nm: Fixed,
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
    /// The standing readiness (decision 1 of
    /// `docs/decisions/0021-readiness-and-alarm.md`).
    pub readiness: Readiness,
    /// The standing order that set the readiness, if any.
    pub readiness_order: Option<OrderId>,
    /// Whether the task force is alerted (decisions 2 to 5 of
    /// `docs/decisions/0021-readiness-and-alarm.md`). Matters only at
    /// normal readiness; see [`TaskForce::is_alert`].
    pub alerted: bool,
    /// The event that alerted the force, if it is alerted.
    pub alerted_by: Option<EventId>,
    /// The standing rules of engagement.
    pub rules_of_engagement: RulesOfEngagement,
    /// The standing order that set the rules of engagement, if any.
    pub rules_order: Option<OrderId>,
    /// The base the force heads for when it retires or sends stragglers home
    /// (decision 0016). It must be a base of the force's own side.
    pub home_port: Option<BaseId>,
    /// The force's fatigue, 0 to 100 (decision 0037). Gained at the start of
    /// each pulse in which the force is alert and lost in each in which it is
    /// not; it cuts the force's chance to sight the enemy and does nothing
    /// else. Stays 0 where the scenario names no fatigue figures.
    pub fatigue: Fixed,
    /// `Some` while the force is away at the yard (decision 0069). Its ships
    /// are then at `ShipLocation::Yard(force)` and out of `ships`.
    pub yard_trip: Option<YardTrip>,
    /// `Some` while the force is halted at a marked waypoint (decision
    /// 0096), exactly when its status is [`TaskForceStatus::Halted`].
    pub halt: Option<Halt>,
    /// The landing the force is loaded for, by order (decision 0095).
    /// `None` until a prepare-landing order is given, and again when the
    /// landing is done.
    pub preparation: Option<Preparation>,
    /// The hex where the force has begun landing men and its unloading is
    /// not yet complete (decision 5 of 0095): from the first man landed
    /// until the unloading there is complete, the force works at the rate of
    /// a landing, whatever it still carries.
    pub landing_at: Option<Hex>,
}

/// A force's preparation for a landing (decision 0095).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Preparation {
    /// The coast hex the landing is for.
    pub hex: Hex,
    /// The land units that are to land from the force. A unit is on at most
    /// one roster.
    pub roster: BTreeSet<LandUnitId>,
    /// The force's loading, from 0 to 100.
    pub loading: Fixed,
    /// The prepare-landing order.
    pub order: OrderId,
}

/// A task force's trip to the yard (decision 0069).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct YardTrip {
    /// The port the force left from.
    pub base: BaseId,
    /// The day the force is back, at the start of that day.
    pub return_day: u32,
    /// The `ForceLeftForYard` event.
    pub departure: EventId,
}

impl TaskForce {
    /// Where the task force is: the hex whose centre is nearest. That is
    /// the hex ahead once it is half its leg's charge along it, and the hex
    /// it left before that (decision 0005, decision 1; decision 0058, 6).
    pub fn position(&self) -> Hex {
        match self.toward_hex {
            Some(toward) if self.progress_nm >= self.leg_nm / Fixed::from_int(2) => toward,
            _ => self.from_hex,
        }
    }

    /// Where a new route starts when it is accepted now, and whether the task
    /// force turns back to get there (decision 0005, decision 5, amended by
    /// decision 0017, decision 14).
    ///
    /// At a centre this is `(false, from_hex)`. Between centres the force
    /// goes on to the hex ahead or turns back to the hex it left, whichever
    /// is the shorter trip to `first_waypoint` by sea. Going on costs the
    /// rest of the leg plus the charged miles of the sea path from the hex
    /// ahead ([`Map::sea_charged_nm_in`]). Turning back costs the miles
    /// already covered plus the charged miles of the sea path from the hex
    /// it left (decision 0058). A tie goes ahead. If only one side can reach the
    /// waypoint, that side is taken. If neither can, the result is `None`
    /// and the order must be rejected. With no waypoint there is nothing to
    /// compare, so it goes ahead.
    ///
    /// Pure: used by both intake validation and applying the order, so they
    /// cannot disagree.
    pub fn reroute_centre(&self, map: &Map, first_waypoint: Option<Hex>) -> Option<(bool, Hex)> {
        let Some(toward) = self.toward_hex else {
            return Some((false, self.from_hex));
        };
        let Some(waypoint) = first_waypoint else {
            return Some((false, toward));
        };
        // Turning back never has to cross a closed side. A force between
        // centres is on an open step from `from_hex` to `toward_hex` (the
        // invariant `TaskForceLegCrossesClosedSide`), and `Map::can_step` is
        // symmetric, so the step back from `toward_hex` to `from_hex` is
        // open too. Both distances below are therefore reached by open steps
        // only, and the cost of turning back is always a real trip.
        let distances = map.sea_distances_to(waypoint);
        let cost_ahead = map
            .sea_charged_nm_in(&distances, toward, waypoint)
            .map(|miles| (self.leg_nm - self.progress_nm) + Fixed::from_int(i64::from(miles)));
        let cost_back = map
            .sea_charged_nm_in(&distances, self.from_hex, waypoint)
            .map(|miles| self.progress_nm + Fixed::from_int(i64::from(miles)));
        match (cost_ahead, cost_back) {
            (None, None) => None,
            (Some(_), None) => Some((false, toward)),
            (None, Some(_)) => Some((true, self.from_hex)),
            (Some(ahead), Some(back)) if back < ahead => Some((true, self.from_hex)),
            (Some(_), Some(_)) => Some((false, toward)),
        }
    }

    /// Whether a route order with these waypoints would be accepted at
    /// intake, as far as the route goes (the order's other checks are the
    /// world's). The one check: `validate_order` and the passage forecast
    /// (decision 0049) both make it.
    ///
    /// A waypoint off the map is rejected first, even for the first one,
    /// before any turn-back choice. The new route starts from the centre the
    /// force reaches first ([`TaskForce::reroute_centre`], the choice
    /// `apply_order` makes); if neither side can reach the first waypoint by
    /// sea the order is rejected. A force between centres always has
    /// somewhere to go: the leg it must finish, so "stop at the hex ahead"
    /// and "go back to the hex I left" are real routes. Each leg must be
    /// reachable by sea from the one before (decision 0017, decision 13).
    pub fn check_route(&self, map: &Map, waypoints: &[Hex]) -> Result<(), RejectReason> {
        let first = waypoints.first().copied();
        if let Some(first) = first {
            if !map.contains(first) {
                return Err(RejectReason::WaypointOffMap(first));
            }
        }
        let Some((_, mut current)) = self.reroute_centre(map, first) else {
            return Err(RejectReason::WaypointUnreachable(
                first.unwrap_or(self.from_hex),
            ));
        };
        let mut goes_somewhere = self.toward_hex.is_some() && !waypoints.is_empty();
        for waypoint in waypoints {
            if !map.contains(*waypoint) {
                return Err(RejectReason::WaypointOffMap(*waypoint));
            }
            match map.sea_distance(current, *waypoint) {
                None => return Err(RejectReason::WaypointUnreachable(*waypoint)),
                Some(steps) => goes_somewhere |= steps > 0,
            }
            current = *waypoint;
        }
        if goes_somewhere {
            Ok(())
        } else {
            Err(RejectReason::EmptyRoute)
        }
    }

    /// Whether the task force is alert: at battle readiness, or alerted
    /// (decisions 1 and 2 of `docs/decisions/0021-readiness-and-alarm.md`).
    pub fn is_alert(&self) -> bool {
        self.readiness == Readiness::Battle || self.alerted
    }

    /// Turns the task force round mid-leg: the two hexes swap and the miles
    /// covered become the miles still to go. Does nothing at a centre.
    pub fn turn_back(&mut self) {
        if let Some(toward) = self.toward_hex {
            self.toward_hex = Some(self.from_hex);
            self.from_hex = toward;
            self.progress_nm = self.leg_nm - self.progress_nm;
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
            leg_nm: Fixed::from_int(HEX_NM),
            status: TaskForceStatus::Idle,
            route_order: None,
            speed_order: None,
            cargo_order: None,
            arrived_at_hours: None,
            arrival_event: None,
            readiness: Readiness::Normal,
            readiness_order: None,
            alerted: false,
            alerted_by: None,
            rules_of_engagement: RulesOfEngagement::DEFAULT,
            rules_order: None,
            home_port: None,
            fatigue: Fixed::ZERO,
            yard_trip: None,
            halt: None,
            preparation: None,
            landing_at: None,
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

/// Where an air group is based: at one base, or aboard one ship (decision 1
/// of `docs/decisions/0025-carrier-air-groups.md`). Never both.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AirGroupLocation {
    Base(BaseId),
    /// Aboard a ship. It is where the ship is, and is lost if the ship sinks.
    Ship(ShipId),
    /// Lost with the base it was at when the base changed side (decision
    /// 0090, 20). The group has no place: it flies nothing, receives
    /// nothing and takes no order. Added at the end: the hash code is 3.
    Lost,
}

impl fmt::Display for AirGroupLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AirGroupLocation::Base(base) => fmt::Display::fmt(base, f),
            AirGroupLocation::Ship(ship) => fmt::Display::fmt(ship, f),
            AirGroupLocation::Lost => f.write_str("lost"),
        }
    }
}

/// A group of aircraft of one type, based at one base or aboard one ship.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AirGroup {
    pub id: AirGroupId,
    pub name: String,
    pub side: Side,
    pub location: AirGroupLocation,
    pub aircraft_type: AircraftTypeId,
    /// Aircraft ready to fly. Unsigned so that it cannot go negative.
    pub ready: u32,
    /// Aircraft that came back worn and are not fit to fly until repaired
    /// (decision 0035). Everything that counts the aircraft a group can fly
    /// counts `ready` only. The default is 0.
    pub damaged: u32,
    pub mission: AirMission,
    pub search: Option<SearchArc>,
    /// The standing order that set the mission, if any.
    pub mission_order: Option<OrderId>,
    /// The standing order that set the search arc, if any.
    pub arc_order: Option<OrderId>,
    /// The share of its flyable aircraft the group keeps over its own hex
    /// when it is on combat air patrol, from 0 to 1 (decision 2 of
    /// `docs/decisions/0027-fighters-and-anti-aircraft-fire.md`). The
    /// default is 1.
    pub patrol_share: Fixed,
    /// The standing order that set the share, if any.
    pub share_order: Option<OrderId>,
    /// How tired the group is, from 0 to 100 (decision 0036). It cuts the
    /// aircraft the group can fly. The default is 0.
    pub fatigue: Fixed,
    /// The fatigue above which the staff stands the group down at the start
    /// of the day (decision 7 of 0036). `None`: never stood down.
    pub rest_above: Option<Fixed>,
    /// The standing order that set the threshold, if any.
    pub rest_order: Option<OrderId>,
    /// Whether the staff stood the group down today: it flies nothing
    /// (decision 9 of 0036). The default is false.
    pub stood_down: bool,
    /// The group's pilots, one for each aircraft it can fly, or `None` if
    /// the scenario does not track them for this group (decision 0038). A
    /// tracked group flies no more aircraft than it has pilots.
    pub pilots: Option<u32>,
    /// The ceiling for replacements, in aircraft (ready and damaged
    /// together) and in pilots: what the group started with, or the figure
    /// the scenario gives (decisions 7 and 10 of 0038). It is read by
    /// nothing else.
    pub strength: u32,
    /// How far the group's strikes and escorts may go (decision 1 of
    /// `docs/decisions/0059-aircraft-that-cannot-land-where-they-took-off.md`).
    /// The default is normal.
    pub reach: Reach,
    /// The standing order that set the reach, if any.
    pub reach_order: Option<OrderId>,
    /// The kinds of ship the group's strikes go for, in order (decision 0063).
    /// A kind not on the list is never launched at. The default is every
    /// kind, in the order of importance ([`ShipKind::ALL`]).
    pub target_priority: Vec<ShipKind>,
    /// The standing order that set the target list, if any.
    pub target_order: Option<OrderId>,
    /// The game day of the group's last base strike launch (decision 7 of
    /// 0063): a base strike launches once in a game day. `None` until it
    /// has launched one.
    pub last_base_strike_day: Option<u32>,
    /// The game day on which the group last moved to another base by order
    /// (decision 0091). On that day it flies nothing. `None` until it has
    /// moved.
    pub rebased_day: Option<u32>,
}

impl AirGroup {
    /// Where a kind of ship comes in the group's order of preference (decision
    /// 4 of 0063): the kinds on its target list first, in list order, then the
    /// kinds not on it, in the order of importance. Lower is better.
    pub fn target_rank(&self, kind: ShipKind) -> (u8, u8) {
        match self
            .target_priority
            .iter()
            .position(|listed| *listed == kind)
        {
            Some(index) => (
                0,
                u8::try_from(index).expect("a list has six kinds at most"),
            ),
            None => (1, kind.code()),
        }
    }

    /// An air group at rest with no aircraft, no fatigue, no pilot tracking
    /// and no standing orders; the whole patrol share. Set `ready` and
    /// `strength` together.
    pub fn new(
        id: AirGroupId,
        name: String,
        side: Side,
        location: AirGroupLocation,
        aircraft_type: AircraftTypeId,
    ) -> AirGroup {
        AirGroup {
            id,
            name,
            side,
            location,
            aircraft_type,
            ready: 0,
            damaged: 0,
            mission: AirMission::Rest,
            search: None,
            mission_order: None,
            arc_order: None,
            patrol_share: Fixed::ONE,
            share_order: None,
            fatigue: Fixed::ZERO,
            rest_above: None,
            rest_order: None,
            stood_down: false,
            pilots: None,
            strength: 0,
            reach: Reach::Normal,
            reach_order: None,
            target_priority: ShipKind::ALL.to_vec(),
            target_order: None,
            last_base_strike_day: None,
            rebased_day: None,
        }
    }
}

/// A stream of replacements a scenario gives a side at a base (decision 6 of
/// `docs/decisions/0038-pilots-and-replacements.md`): each day, for one
/// aircraft type, a number of aircraft and a number of pilots. Static: it is
/// never changed by the rules.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Replacement {
    pub side: Side,
    pub base: BaseId,
    pub aircraft_type: AircraftTypeId,
    pub aircraft_per_day: u32,
    pub pilots_per_day: u32,
}

/// Aircraft and pilots a scenario flies in to an air group on a named day
/// (decision 0041). Static: it is never changed by the rules.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Arrival {
    pub day: u32,
    pub air_group: AirGroupId,
    pub aircraft: u32,
    pub pilots: u32,
}

/// A land unit: a count of men with a side and a name (decision 0066, 1).
/// Its men are ashore in one hex, aboard ships ([`Ship::troops`]), or part
/// each. It is never removed: a unit with no men anywhere is destroyed, and
/// keeps its number.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LandUnit {
    pub id: LandUnitId,
    pub name: String,
    pub side: Side,
    /// Men ashore, in `hex`.
    pub men_ashore: u32,
    /// The hex the men ashore are in. `Some` exactly when `men_ashore` is
    /// above 0.
    pub hex: Option<Hex>,
    /// Defend or attack (decision 0090, 1). Defend unless the scenario or a
    /// posture order says otherwise.
    pub posture: Posture,
    /// The standing order that set the posture, if any. Spent when a tired
    /// unit goes back to defend (decision 0090, 14).
    pub posture_order: Option<OrderId>,
    /// How tired the unit is, from 0 to 100 (decision 0090, 11). It cuts the
    /// men who fire and nothing else.
    pub fatigue: Fixed,
    /// The men of the unit's side fed in its hex at the last night's charge
    /// over the men charged, from 0 to 1 (decision 0090, 8). 1 for a unit
    /// none of whose men were ashore at that charge, and 1 until the first.
    pub fed_share: Fixed,
    /// The unit's march, if it has a march order standing (decision 0094, 3
    /// and 4). `None` unless a march order is given.
    pub march: Option<March>,
    /// How well the unit has rehearsed the landing at `rehearsal_hex`, from
    /// 0 to 100 (decision 3 of 0095). 0 until it rehearses.
    pub rehearsal: Fixed,
    /// The hex the rehearsal is for: `Some` exactly when `rehearsal` is above
    /// 0.
    pub rehearsal_hex: Option<Hex>,
    /// What the unit does on a day it does not march or fight (decision 4 of
    /// 0099). Ready unless an order says otherwise.
    pub activity: Activity,
    /// The order that set the activity, if any.
    pub activity_order: Option<OrderId>,
    /// The unit's experience, from 0 to 4 (decision 1 of 0097). 2 is its
    /// service's standard.
    pub experience: u8,
    /// Days of training counted toward the next step of experience, below
    /// `training_days_per_level` (decision 3 of 0097).
    pub training_days: u32,
    /// Days of firing in a fight counted toward the next step of experience,
    /// below `fight_days_per_level` (decision 5 of 0097).
    pub fight_days: u32,
}

/// A land unit's march to a neighbouring hex (decision 0094, 3 and 4). It
/// stands until the unit arrives or the order is replaced.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct March {
    /// The hex the unit marches to: a neighbour of its hex joined to it by a
    /// land link.
    pub to: Hex,
    /// The days marched so far, below the figure `march_days_per_hex`.
    pub days: u32,
    /// The march order.
    pub order: OrderId,
}

impl LandUnit {
    /// The experience of a unit a scenario does not rate: its service's
    /// standard (decision 1 of 0097).
    pub const DEFAULT_EXPERIENCE: u8 = 2;
    /// The highest experience (decision 1 of 0097).
    pub const MAX_EXPERIENCE: u8 = 4;

    /// A unit on defend with no fatigue, fully fed and no standing order:
    /// how every unit starts unless the scenario says otherwise, and how a
    /// unit divided off another starts.
    pub fn new(
        id: LandUnitId,
        name: String,
        side: Side,
        men_ashore: u32,
        hex: Option<Hex>,
    ) -> LandUnit {
        LandUnit {
            id,
            name,
            side,
            men_ashore,
            hex,
            posture: Posture::Defend,
            posture_order: None,
            fatigue: Fixed::ZERO,
            fed_share: Fixed::ONE,
            march: None,
            rehearsal: Fixed::ZERO,
            rehearsal_hex: None,
            activity: Activity::Ready,
            activity_order: None,
            experience: LandUnit::DEFAULT_EXPERIENCE,
            training_days: 0,
            fight_days: 0,
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
    pub air_groups: BTreeMap<AirGroupId, AirGroup>,
    /// The scenario's replacement streams, in the order they are applied
    /// (decision 0038). Empty in a scenario without replacements.
    pub replacements: Vec<Replacement>,
    /// The scenario's arrivals, in the order they are delivered (decision
    /// 0041). Empty in a scenario without any.
    pub arrivals: Vec<Arrival>,
    /// Land units (decision 0066). Empty in a scenario without any.
    pub land_units: BTreeMap<LandUnitId, LandUnit>,
}

impl TruthState {
    /// The men of a land unit aboard ships that are afloat (decision 0066).
    pub fn men_aboard(&self, unit: LandUnitId) -> u32 {
        self.ships
            .values()
            .filter_map(|ship| ship.troops)
            .filter(|troops| troops.unit == unit)
            .map(|troops| troops.men)
            .sum()
    }

    /// Puts a land unit's men aboard a task force at the start of a game
    /// (decision 0066, 3, and "Settled before the build"): over the force's
    /// ships in ship order, each up to its troop capacity, into ships that
    /// are empty of men or already hold this unit. For the loader. Fails,
    /// and changes nothing, if the men do not all fit, the force does not
    /// exist or is not the unit's side's, or the unit does not exist.
    pub fn put_aboard_at_start(
        &mut self,
        unit: LandUnitId,
        task_force: TaskForceId,
        men: u32,
        troop_capacity: impl Fn(ShipClassId) -> u32,
    ) -> Result<(), String> {
        let side = self
            .land_units
            .get(&unit)
            .ok_or_else(|| format!("{unit} does not exist"))?
            .side;
        let force = self
            .task_forces
            .get(&task_force)
            .ok_or_else(|| format!("{unit} is aboard {task_force}, which does not exist"))?;
        if force.side != side {
            return Err(format!(
                "{unit} is aboard {task_force}, which is not its side's"
            ));
        }
        let mut plan: Vec<(ShipId, u32)> = Vec::new();
        let mut left = men;
        for ship_id in &force.ships {
            if left == 0 {
                break;
            }
            let ship = &self.ships[ship_id];
            let (aboard, other) = match ship.troops {
                Some(troops) => (troops.men, troops.unit != unit),
                None => (0, false),
            };
            if other {
                continue;
            }
            let room = troop_capacity(ship.class).saturating_sub(aboard);
            let take = room.min(left);
            if take > 0 {
                plan.push((*ship_id, aboard + take));
                left -= take;
            }
        }
        if left > 0 {
            return Err(format!(
                "{unit} has {men} men aboard {task_force}, but its ships have berths for {} of them",
                men - left
            ));
        }
        for (ship_id, total) in plan {
            self.ships
                .get_mut(&ship_id)
                .expect("the ship exists")
                .troops = Some(Troops { unit, men: total });
        }
        Ok(())
    }

    /// Whether an air group is aboard a ship that is away at the yard
    /// (decision 0069). Such a group flies nothing, wears nothing, is not
    /// repaired, gets no replacement or arrival, and tires and rests not at
    /// all while the ship is away.
    pub fn air_group_at_yard(&self, group: &AirGroup) -> bool {
        match group.location {
            AirGroupLocation::Ship(ship) => self
                .ships
                .get(&ship)
                .is_some_and(|ship| matches!(ship.location, ShipLocation::Yard(_))),
            AirGroupLocation::Base(_) | AirGroupLocation::Lost => false,
        }
    }

    /// The hex an air group is in (decision 3 of
    /// `docs/decisions/0025-carrier-air-groups.md`): its base's hex, or the
    /// position of the task force its ship is in, or the hex of the base its
    /// ship is in port at. A group aboard a sunk ship, or aboard a ship or at
    /// a base that does not exist, or lost with its base (decision 0090), has
    /// no position.
    pub fn air_group_hex(&self, group: &AirGroup) -> Option<Hex> {
        match group.location {
            AirGroupLocation::Base(base) => self.bases.get(&base).map(|base| base.hex),
            AirGroupLocation::Ship(ship) => match self.ships.get(&ship)?.location {
                ShipLocation::TaskForce(task_force) => {
                    self.task_forces.get(&task_force).map(TaskForce::position)
                }
                ShipLocation::Port(base) => self.bases.get(&base).map(|base| base.hex),
                ShipLocation::Sunk => None,
                // A ship at the yard is off the map (decision 0069). Callers
                // must not read this as "lost": the ship is afloat.
                ShipLocation::Yard(_) => None,
            },
            AirGroupLocation::Lost => None,
        }
    }
}
