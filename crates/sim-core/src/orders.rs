//! Unit-level orders: the only way to influence the simulation.
//!
//! Only the orders the scenarios so far need exist: task force orders (route,
//! speed, mission, load, unload and rules of engagement) and air group orders
//! (mission and search arc). The spec lists the full set (form, disband and
//! so on). They arrive with the scenarios that use them.

use core::fmt;

use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{
    AirGroupId, BaseId, EntityRef, IntentId, LandUnitId, OrderId, ShipId, TaskForceId,
};
use crate::refdata::ShipKind;
use crate::state::Side;

/// Who issued an order.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Issuer {
    Player,
    /// The staff planner, acting on the player's intent between days.
    Staff,
    /// A rule acting for the commander on the spot inside a day, on the
    /// standing orders the unit already has: a damaged ship sent home, say.
    /// Kept apart from `Staff` so that "who decided this" can tell the
    /// planner's choice from doctrine firing (decision 0055). Added at the
    /// end.
    LocalCommander,
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
    /// Strikes enemy ships its side has a contact on (decision 0026).
    NavalStrike,
    /// Fights over its own hex and escorts strikes (decision 0027). Added at
    /// the end.
    CombatAirPatrol,
    /// Strikes the named enemy base (decision 0034). Added at the end.
    BaseStrike(BaseId),
}

impl AirMission {
    /// A stable number for the mission. Part of the state hash. Never
    /// renumber it.
    pub const fn code(self) -> u8 {
        match self {
            AirMission::Rest => 1,
            AirMission::Search => 2,
            AirMission::NavalStrike => 3,
            AirMission::CombatAirPatrol => 4,
            AirMission::BaseStrike(_) => 5,
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
    /// A speed in knots (decision 7 of
    /// `docs/decisions/0021-readiness-and-alarm.md`). Each ship steams at
    /// this speed or at its class top speed if that is lower, times its
    /// propulsion factor. Must be above zero. Added at the end.
    Knots(Fixed),
}

/// How far an air group's strikes and escorts may go (decision 1 of
/// `docs/decisions/0059-aircraft-that-cannot-land-where-they-took-off.md`).
/// Normal is the type's strike radius. Extended is the radius and a share
/// more (`air_combat.toml: extended_reach_share`), at the price of aircraft
/// that may run out of fuel on the way home.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub enum Reach {
    /// The type's strike radius. The default.
    #[default]
    Normal,
    /// The radius and a share more.
    Extended,
}

impl Reach {
    /// A stable number for the reach. Part of the state hash. Never
    /// renumber it; add new values at the end.
    pub const fn code(self) -> u8 {
        match self {
            Reach::Normal => 1,
            Reach::Extended => 2,
        }
    }

    /// The reach's name as scenario files, orders and the log write it.
    pub const fn name(self) -> &'static str {
        match self {
            Reach::Normal => "normal",
            Reach::Extended => "extended",
        }
    }
}

/// How ready a task force is for action (decision 1 of
/// `docs/decisions/0021-readiness-and-alarm.md`). A force at battle readiness
/// is alert.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub enum Readiness {
    /// Not expecting action. Alert only while alerted.
    #[default]
    Normal,
    /// At battle stations. Always alert.
    Battle,
}

impl Readiness {
    /// A stable number for the readiness. Part of the state hash. Never
    /// renumber it; add new levels at the end.
    pub const fn code(self) -> u8 {
        match self {
            Readiness::Normal => 1,
            Readiness::Battle => 2,
        }
    }

    /// The readiness's name as scenario files and the log write it.
    pub const fn name(self) -> &'static str {
        match self {
            Readiness::Normal => "normal",
            Readiness::Battle => "battle",
        }
    }
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

/// What a task force does about its stragglers, ships whose propulsion is
/// degraded or out (decisions 10 to 12 of
/// `docs/decisions/0016-systems-and-stragglers.md`).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub enum StragglerInstruction {
    /// Nothing happens.
    #[default]
    Keep,
    /// Send stragglers that can steam home with escorts.
    Detach,
    /// Leave every straggler where it is.
    Leave,
    /// The whole force heads home.
    Retire,
}

impl StragglerInstruction {
    /// A stable number for the instruction. Part of the state hash. Never
    /// renumber it; add new instructions at the end.
    pub const fn code(self) -> u8 {
        match self {
            StragglerInstruction::Keep => 1,
            StragglerInstruction::Detach => 2,
            StragglerInstruction::Leave => 3,
            StragglerInstruction::Retire => 4,
        }
    }

    /// The instruction's name as scenario files and the log write it.
    pub const fn name(self) -> &'static str {
        match self {
            StragglerInstruction::Keep => "keep",
            StragglerInstruction::Detach => "detach",
            StragglerInstruction::Leave => "leave",
            StragglerInstruction::Retire => "retire",
        }
    }
}

/// A kind of cargo a ship carries (decision 0046). A ship carries supply and
/// fuel together, up to its class's cargo capacity.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Cargo {
    Supply,
    Fuel,
}

impl Cargo {
    /// The cargo's name as scenario files and the log write it.
    pub const fn name(self) -> &'static str {
        match self {
            Cargo::Supply => "supply",
            Cargo::Fuel => "fuel",
        }
    }
}

/// What a standing cargo order does at a port.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum CargoAction {
    /// Load supply.
    Load,
    /// Unload everything aboard: supply first, then fuel.
    Unload,
    /// Load fuel as cargo (decision 0046). Added at the end.
    LoadFuel,
    /// Load no cargo: the load order with a land unit that names none takes
    /// the unit's men aboard and nothing else (decision 0066). Added at the
    /// end.
    LoadNoCargo,
}

impl CargoAction {
    /// A stable number for the action. Part of the state hash. Never
    /// renumber it; add new actions at the end.
    pub const fn code(self) -> u8 {
        match self {
            CargoAction::Load => 1,
            CargoAction::Unload => 2,
            CargoAction::LoadFuel => 3,
            CargoAction::LoadNoCargo => 4,
        }
    }

    /// The cargo a load action takes aboard, or `None` for an unload and for
    /// a load that names no cargo.
    pub const fn loads(self) -> Option<Cargo> {
        match self {
            CargoAction::Load => Some(Cargo::Supply),
            CargoAction::LoadFuel => Some(Cargo::Fuel),
            CargoAction::Unload | CargoAction::LoadNoCargo => None,
        }
    }

    /// Whether the action is a load of any kind.
    pub const fn is_load(self) -> bool {
        !matches!(self, CargoAction::Unload)
    }

    /// The load action for a cargo, or for none.
    pub const fn load_of(cargo: Option<Cargo>) -> CargoAction {
        match cargo {
            Some(Cargo::Supply) => CargoAction::Load,
            Some(Cargo::Fuel) => CargoAction::LoadFuel,
            None => CargoAction::LoadNoCargo,
        }
    }
}

/// An order to a task force.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TaskForceOrder {
    /// Steam to each waypoint in turn and stop at the last, by the shortest
    /// path over hexes ships can enter (decision 0017). Rejected if a
    /// waypoint is off the map or cannot be reached by sea from the one
    /// before.
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
    /// replaces the rules of engagement and nothing else. `break_off` and
    /// `keep_share` must each be from 0 to 1.
    SetRulesOfEngagement {
        stance: Stance,
        break_off: Fixed,
        stragglers: StragglerInstruction,
        keep_share: Fixed,
    },
    /// Set the base the force heads for when it retires or sends stragglers
    /// home. It must be a base of the issuing side.
    SetHomePort(BaseId),
    /// Split the listed ships off into a new task force at the same spot,
    /// idle, with the same rules of engagement. At least one ship must stay.
    DetachShips(Vec<ShipId>),
    /// Set the readiness (decision 1 of
    /// `docs/decisions/0021-readiness-and-alarm.md`). A standing order of its
    /// own kind: it replaces the readiness and nothing else, and does not
    /// touch whether the force is alerted. Added at the end.
    SetReadiness(Readiness),
    /// Fill every ship to its remaining cargo capacity with fuel from the
    /// fuel stock of the base here (decision 0046). The load order that names
    /// fuel: `Load` names supply. A standing order of the "cargo" kind,
    /// accepted under the same conditions as `Load`. Added at the end.
    LoadFuel,
    /// Send the whole force to the yard, from the base it is stopped at
    /// (decision 0069). It goes with all its ships, is repaired and returns
    /// on its day of return. Added at the end.
    SendToYard,
    /// The load order with a land unit (decision 0066, 5 and 6): take the
    /// unit's men ashore in the force's hex aboard first, then the named
    /// cargo, supply or fuel, or none. A standing order of the "cargo" kind,
    /// like `Load`. Accepted where the side has no base, and then it loads
    /// men only. Added at the end.
    LoadUnit {
        unit: LandUnitId,
        cargo: Option<Cargo>,
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
    /// The share of its flyable aircraft a group on combat air patrol keeps
    /// over its own hex, from 0 to 1 (decision 2 of
    /// `docs/decisions/0027-fighters-and-anti-aircraft-fire.md`). A standing
    /// order of its own kind. Added at the end.
    SetPatrolShare {
        share: Fixed,
    },
    /// The fatigue above which the staff stands the group down at the start
    /// of the day, from 0 to 100 (decision 7 of
    /// `docs/decisions/0036-fatigue-and-rest.md`). A standing order of its
    /// own kind. Added at the end.
    SetRestThreshold {
        fatigue: Fixed,
    },
    /// How far the group's strikes and escorts may go: normal (the type's
    /// radius) or extended (decision 1 of 0059). A standing order of its own
    /// kind: it replaces the reach and nothing else. Added at the end.
    SetReach {
        reach: Reach,
    },
    /// The kinds of ship a strike group goes for, in order (decision 0063).
    /// A kind not on the list is never launched at. A standing order of its
    /// own kind: it replaces the list and nothing else. Added at the end.
    SetTargetPriority {
        kinds: Vec<ShipKind>,
    },
    /// Move the whole group to another base of its side (decision 0091). Not
    /// a standing order: it is done when it is accepted, at the start of the
    /// day, and is then spent. The group flies nothing that day. Added at the
    /// end.
    Rebase {
        base: BaseId,
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
    /// The order needs a task force that is stopped.
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
    /// A share to keep is below 0 or above 1.
    KeepShareOutOfRange(Fixed),
    /// The base does not exist or is not the issuing side's. One reason for
    /// both, so a rejection never reveals an enemy base.
    NotYourBase,
    /// A detach order lists no ships.
    NoShipsToDetach,
    /// A ship listed in a detach order is not afloat in the task force.
    ShipNotInTaskForce(ShipId),
    /// A detach order lists every ship of the task force.
    WouldLeaveTaskForceEmpty,
    /// A waypoint is on the map but ships cannot get to it from the one
    /// before: it is land, or it lies across land with no way round
    /// (decision 0017, decision 13). Added at the end of the enum.
    WaypointUnreachable(Hex),
    /// A speed in knots is zero or negative (decision 7 of
    /// `docs/decisions/0021-readiness-and-alarm.md`). Added at the end of
    /// the enum.
    SpeedNotAboveZero(Fixed),
    /// The naval strike mission needs the air combat figures, and the
    /// scenario has none (decision 0026). Added at the end of the enum.
    NoAirCombatFigures,
    /// The naval strike mission needs an aircraft type that attacks with
    /// something (decision 2 of 0026). Added at the end of the enum.
    AircraftAttackWithNothing,
    /// The combat air patrol mission is for fighter types only (decision 1
    /// of 0027). Added at the end of the enum.
    NotAFighter,
    /// A patrol share is not between 0 and 1 (decision 2 of 0027). Added at
    /// the end of the enum.
    PatrolShareOutOfRange(Fixed),
    /// The base strike mission names a base that does not exist or is held by
    /// the group's own side (decision 1 of 0034). Added at the end of the
    /// enum.
    NotAnEnemyBase,
    /// A rest threshold is not between 0 and 100 (decision 7 of 0036). Added
    /// at the end of the enum.
    FatigueOutOfRange(Fixed),
    /// The task force has no ship afloat: every ship it had is sunk, so there
    /// is nothing to give an order to. Added at the end of the enum.
    NoShipAfloat,
    /// The air group was lost with the ship it was aboard, so there is
    /// nothing to give an order to. Added at the end of the enum.
    AirGroupLost,
    /// A target list names no kind (decision 0063). Added at the end of the
    /// enum.
    TargetListEmpty,
    /// A target list names a kind twice (decision 0063). Added at the end of
    /// the enum.
    TargetKindRepeated(ShipKind),
    /// A load, load-fuel or set-route order to a task force that is out of
    /// fuel: it cannot steam and has nothing to load into, so the order is
    /// refused (rows 17 and 34 of `docs/rules/differences.md`). Added at the
    /// end of the enum.
    OutOfFuel,
    /// The task force is away at a yard until its day of return (decision
    /// 0069). Added at the end of the enum.
    AwayAtYard,
    /// No base of the side in the task force's hex is a port for the yard
    /// (decision 0069). Added at the end of the enum.
    NotAYardPort,
    /// A load order names a land unit that does not exist or is not the
    /// issuing side's. One reason for both (decision 0066, 6). Added at the
    /// end of the enum.
    NotYourLandUnit,
    /// A load order names a land unit with no men ashore at the task force's
    /// position (decision 0066, 6). Added at the end of the enum.
    NoMenAshoreHere,
    /// A load order names a land unit, and no ship of the task force can
    /// carry men (decision 0066, 6). Added at the end of the enum.
    NoTroopCapacity,
    /// A rebase order to an air group aboard a ship (decision 0091, 2).
    /// Added at the end of the enum.
    AirGroupAboardShip,
    /// A rebase order to an air group aboard a ship that is away at the yard
    /// (decision 0091, 2). Added at the end of the enum.
    AirGroupAwayAtYard,
    /// A rebase order names the base the air group is already at (decision
    /// 0091, 2). Added at the end of the enum.
    AlreadyAtBase,
    /// A rebase order names a base further from the air group's base than
    /// its type's transfer range (decisions 2 and 3 of 0091). `distance_nm`
    /// is the air distance between the two bases. Added at the end of the
    /// enum.
    BeyondTransferRange { distance_nm: Fixed, range_nm: Fixed },
    /// A second rebase order to an air group on the day it has already moved
    /// (decision 0091, "As built"). Added at the end of the enum.
    AlreadyMovedToday,
}

impl fmt::Display for RejectReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RejectReason::NotYourUnit => f.write_str("the target is not one of your units"),
            RejectReason::EmptyRoute => f.write_str("the route goes nowhere"),
            RejectReason::WaypointOffMap(hex) => write!(f, "waypoint {hex} is off the map"),
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
            RejectReason::KeepShareOutOfRange(share) => {
                write!(f, "share to keep {share} is not between 0 and 1")
            }
            RejectReason::NotYourBase => f.write_str("the base is not one of your bases"),
            RejectReason::NoShipsToDetach => f.write_str("no ships are listed to detach"),
            RejectReason::ShipNotInTaskForce(ship) => {
                write!(f, "{ship} is not afloat in the task force")
            }
            RejectReason::WouldLeaveTaskForceEmpty => {
                f.write_str("detaching these ships would leave the task force empty")
            }
            RejectReason::WaypointUnreachable(hex) => {
                write!(f, "waypoint {hex} cannot be reached by sea")
            }
            RejectReason::SpeedNotAboveZero(knots) => {
                write!(f, "speed {knots} kn is not above 0")
            }
            RejectReason::NoAirCombatFigures => {
                f.write_str("this scenario has no air combat figures, so no air group can strike")
            }
            RejectReason::AircraftAttackWithNothing => {
                f.write_str("the air group's aircraft type attacks with nothing")
            }
            RejectReason::NotAFighter => {
                f.write_str("the air group's aircraft type is not a fighter")
            }
            RejectReason::PatrolShareOutOfRange(share) => {
                write!(f, "patrol share {share} is not between 0 and 1")
            }
            RejectReason::NotAnEnemyBase => {
                f.write_str("the base does not exist or is held by your side")
            }
            RejectReason::FatigueOutOfRange(fatigue) => {
                write!(f, "rest threshold {fatigue} is not between 0 and 100")
            }
            RejectReason::NoShipAfloat => f.write_str("the task force has no ship afloat"),
            RejectReason::AirGroupLost => f.write_str("the air group was lost with its ship"),
            RejectReason::TargetListEmpty => f.write_str("the target list names no kind of ship"),
            RejectReason::OutOfFuel => f.write_str("the task force is out of fuel"),
            RejectReason::AwayAtYard => {
                f.write_str("the task force is away at a yard until its day of return")
            }
            RejectReason::NotAYardPort => {
                f.write_str("no base of your side in the task force's hex is a port for the yard")
            }
            RejectReason::NotYourLandUnit => f.write_str("the land unit is not one of your units"),
            RejectReason::NoMenAshoreHere => {
                f.write_str("the land unit has no men ashore where the task force is")
            }
            RejectReason::NoTroopCapacity => f.write_str("no ship of the task force can carry men"),
            RejectReason::AirGroupAboardShip => f.write_str("the air group is aboard a ship"),
            RejectReason::AirGroupAwayAtYard => {
                f.write_str("the air group is aboard a ship that is away at a yard")
            }
            RejectReason::AlreadyAtBase => f.write_str("the air group is already at that base"),
            RejectReason::BeyondTransferRange {
                distance_nm,
                range_nm,
            } => write!(
                f,
                "the base is {distance_nm} nm away, beyond the air group's transfer range of {range_nm} nm"
            ),
            RejectReason::AlreadyMovedToday => {
                f.write_str("the air group has already moved to another base today")
            }
            RejectReason::TargetKindRepeated(kind) => {
                write!(f, "the target list names {} twice", kind.name())
            }
        }
    }
}
