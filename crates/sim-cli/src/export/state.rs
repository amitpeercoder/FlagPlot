//! A snapshot of the truth (and, at the end, of each side's belief) as JSON.

use serde::Serialize;
use sim_core::hex::Hex;
use sim_core::orders::AirMission;
use sim_core::refdata::RefData;
use sim_core::state::{
    AirGroup, AirGroupLocation, Base, Ship, ShipLocation, ShipSystem, Side, TaskForce, TruthState,
};
use sim_core::World;

use super::data::Data;
use super::names;
use super::num::Num;

/// A hex as `{ "q": .., "r": .. }`.
#[derive(Serialize)]
pub struct Pos {
    q: i32,
    r: i32,
}

impl From<Hex> for Pos {
    fn from(hex: Hex) -> Pos {
        Pos { q: hex.q, r: hex.r }
    }
}

#[derive(Serialize)]
pub struct Snapshot {
    pub task_forces: Vec<TaskForceOut>,
    pub ships: Vec<ShipOut>,
    pub air_groups: Vec<AirGroupOut>,
    pub bases: Vec<BaseOut>,
}

#[derive(Serialize)]
pub struct TaskForceOut {
    id: u32,
    name: String,
    side: &'static str,
    mission: &'static str,
    /// The hex whose centre is nearest.
    position: Pos,
    /// The stored leg (decision 0005): the hex last left, the hex steamed
    /// toward (null at a centre) and the miles covered between them.
    from_hex: Pos,
    toward_hex: Option<Pos>,
    progress_nm: Num,
    status: &'static str,
    ships: Vec<u32>,
    home_port: Option<u32>,
    /// "normal" or "battle" (decision 0021).
    readiness: &'static str,
    alerted: bool,
    /// Fatigue (decision 0037). Left out when 0.
    #[serde(skip_serializing_if = "num_is_zero")]
    fatigue: Num,
    rules_of_engagement: RulesOut,
}

#[derive(Serialize)]
struct RulesOut {
    stance: &'static str,
    break_off: Num,
    stragglers: &'static str,
    keep_share: Num,
}

#[derive(Serialize)]
pub struct ShipOut {
    id: u32,
    name: String,
    class: String,
    kind: &'static str,
    side: &'static str,
    location: LocationOut,
    fuel: Num,
    supply: Num,
    flotation_damage: Num,
    fire_damage: Num,
    /// Only the systems the ship's class has.
    systems: Data,
    gun_rounds: Num,
    torpedoes_ready: bool,
}

/// `{"task_force": n}`, `{"port": n}` or `"sunk"`.
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum LocationOut {
    TaskForce(u32),
    Port(u32),
    Sunk,
}

#[derive(Serialize)]
pub struct AirGroupOut {
    id: u32,
    name: String,
    side: &'static str,
    /// Set for a group based at a base.
    #[serde(skip_serializing_if = "Option::is_none")]
    base: Option<u32>,
    /// Set for a group aboard a ship.
    #[serde(skip_serializing_if = "Option::is_none")]
    ship: Option<u32>,
    aircraft_type: String,
    ready: u32,
    /// Damaged aircraft (decision 0035). Left out when 0.
    #[serde(skip_serializing_if = "is_zero")]
    damaged: u32,
    mission: &'static str,
    /// Set for a group on combat air patrol (decision 0027).
    #[serde(skip_serializing_if = "Option::is_none")]
    patrol_share: Option<Num>,
    /// Fatigue (decision 0036). Left out when 0.
    #[serde(skip_serializing_if = "num_is_zero")]
    fatigue: Num,
    /// The fatigue above which the staff stands the group down. Left out
    /// when the group has no threshold.
    #[serde(skip_serializing_if = "Option::is_none")]
    rest_above: Option<Num>,
    /// Whether the staff stood the group down today. Left out when not.
    #[serde(skip_serializing_if = "is_false")]
    stood_down: bool,
    /// Pilots (decision 0038). Left out when the group's pilots are not
    /// tracked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pilots: Option<u32>,
    /// The ceiling for replacements; given only with pilots.
    #[serde(skip_serializing_if = "Option::is_none")]
    strength: Option<u32>,
    arc: Option<ArcOut>,
}

#[derive(Serialize)]
struct ArcOut {
    from_deg: u32,
    to_deg: u32,
    range_nm: Num,
}

#[derive(Serialize)]
pub struct BaseOut {
    id: u32,
    name: String,
    side: &'static str,
    q: i32,
    r: i32,
    supply: Num,
    /// Anti-aircraft shots (decision 0034). Left out when 0.
    #[serde(skip_serializing_if = "is_zero")]
    anti_aircraft: u32,
    /// Aircraft repaired a day (decision 0035). Left out when 0.
    #[serde(skip_serializing_if = "is_zero")]
    aircraft_repairs_per_day: u32,
    /// Fuel stock for ships in tons (decision 0045). Left out when 0.
    #[serde(skip_serializing_if = "num_is_zero")]
    fuel: Num,
}

fn is_zero(n: &u32) -> bool {
    *n == 0
}

fn num_is_zero(n: &Num) -> bool {
    n.0 == sim_core::fixed::Fixed::ZERO
}

fn is_false(b: &bool) -> bool {
    !*b
}

pub fn snapshot(truth: &TruthState, ref_data: &RefData) -> Snapshot {
    Snapshot {
        task_forces: truth.task_forces.values().map(task_force).collect(),
        ships: truth
            .ships
            .values()
            .map(|ship| self::ship(ship, ref_data))
            .collect(),
        air_groups: truth
            .air_groups
            .values()
            .map(|group| air_group(group, ref_data))
            .collect(),
        bases: bases(truth),
    }
}

pub fn bases(truth: &TruthState) -> Vec<BaseOut> {
    truth.bases.values().map(base).collect()
}

fn base(base: &Base) -> BaseOut {
    BaseOut {
        id: base.id.raw(),
        name: base.name.clone(),
        side: names::side(base.side),
        q: base.hex.q,
        r: base.hex.r,
        supply: Num(base.supply_t),
        anti_aircraft: base.anti_aircraft,
        aircraft_repairs_per_day: base.aircraft_repairs_per_day,
        fuel: Num(base.fuel_t),
    }
}

fn task_force(force: &TaskForce) -> TaskForceOut {
    let roe = &force.rules_of_engagement;
    TaskForceOut {
        id: force.id.raw(),
        name: force.name.clone(),
        side: names::side(force.side),
        mission: names::mission(force.mission),
        position: force.position().into(),
        from_hex: force.from_hex.into(),
        toward_hex: force.toward_hex.map(Pos::from),
        progress_nm: Num(force.progress_nm),
        status: names::status(force.status),
        ships: force.ships.iter().map(|ship| ship.raw()).collect(),
        home_port: force.home_port.map(|port| port.raw()),
        readiness: force.readiness.name(),
        alerted: force.alerted,
        fatigue: Num(force.fatigue),
        rules_of_engagement: RulesOut {
            stance: names::stance(roe.stance),
            break_off: Num(roe.break_off),
            stragglers: roe.stragglers.name(),
            keep_share: Num(roe.keep_share),
        },
    }
}

fn ship(ship: &Ship, ref_data: &RefData) -> ShipOut {
    let class = &ref_data.ship_classes[&ship.class];
    let mut systems = Data::new();
    for system in ShipSystem::ALL {
        if class.has_system(system) {
            systems = systems.text(
                names::system(system),
                names::system_state(ship.systems.get(system)),
            );
        }
    }
    ShipOut {
        id: ship.id.raw(),
        name: ship.name.clone(),
        class: class.name.clone(),
        kind: class.kind.name(),
        side: names::side(ship.side),
        location: match ship.location {
            ShipLocation::TaskForce(id) => LocationOut::TaskForce(id.raw()),
            ShipLocation::Port(id) => LocationOut::Port(id.raw()),
            ShipLocation::Sunk => LocationOut::Sunk,
        },
        fuel: Num(ship.fuel_t),
        supply: Num(ship.supply_t),
        flotation_damage: Num(ship.flotation_damage),
        fire_damage: Num(ship.fire_damage),
        systems,
        gun_rounds: Num(ship.gun_rounds),
        torpedoes_ready: ship.torpedoes_ready,
    }
}

fn air_group(group: &AirGroup, ref_data: &RefData) -> AirGroupOut {
    AirGroupOut {
        id: group.id.raw(),
        name: group.name.clone(),
        side: names::side(group.side),
        base: match group.location {
            AirGroupLocation::Base(base) => Some(base.raw()),
            AirGroupLocation::Ship(_) => None,
        },
        ship: match group.location {
            AirGroupLocation::Base(_) => None,
            AirGroupLocation::Ship(ship) => Some(ship.raw()),
        },
        aircraft_type: ref_data.aircraft_types[&group.aircraft_type].name.clone(),
        ready: group.ready,
        damaged: group.damaged,
        mission: names::air_mission(group.mission),
        patrol_share: (group.mission == AirMission::CombatAirPatrol)
            .then_some(Num(group.patrol_share)),
        fatigue: Num(group.fatigue),
        rest_above: group.rest_above.map(Num),
        stood_down: group.stood_down,
        pilots: group.pilots,
        strength: group.pilots.map(|_| group.strength),
        arc: group.search.map(|arc| ArcOut {
            from_deg: arc.from_deg,
            to_deg: arc.to_deg,
            range_nm: Num(arc.range_nm),
        }),
    }
}

/// The end of the run: the truth, and what each side believed.
#[derive(Serialize)]
pub struct FinalOut {
    task_forces: Vec<TaskForceOut>,
    ships: Vec<ShipOut>,
    air_groups: Vec<AirGroupOut>,
    bases: Vec<BaseOut>,
    contacts: PerSide<ContactOut>,
    claims: PerSide<ClaimOut>,
}

#[derive(Serialize)]
pub struct PerSide<T> {
    allied: Vec<T>,
    japanese: Vec<T>,
}

#[derive(Serialize)]
struct ContactOut {
    id: u32,
    hex: (i32, i32),
    ship_count: u32,
    kind: &'static str,
    age_pulses: u32,
}

#[derive(Serialize)]
struct ClaimOut {
    hex: (i32, i32),
    day: u32,
    pulse: &'static str,
    claimed_sunk: u32,
    received_event: u32,
}

/// The truth after the last day, with the contacts and claims each side
/// holds, read through `side_view`.
pub fn final_state(world: &World) -> FinalOut {
    let Snapshot {
        task_forces,
        ships,
        air_groups,
        bases,
    } = snapshot(world.umpire(), world.ref_data());
    let contacts = |side: Side| -> Vec<ContactOut> {
        world
            .side_view(side)
            .contacts()
            .map(|contact| ContactOut {
                id: contact.id.raw(),
                hex: (contact.estimated_hex.q, contact.estimated_hex.r),
                ship_count: contact.estimated_ship_count,
                kind: contact.estimated_kind.name(),
                age_pulses: contact.age_pulses,
            })
            .collect()
    };
    let claims = |side: Side| -> Vec<ClaimOut> {
        world
            .side_view(side)
            .claims()
            .iter()
            .map(|claim| ClaimOut {
                hex: (claim.hex.q, claim.hex.r),
                day: claim.day,
                pulse: claim.pulse.name(),
                claimed_sunk: claim.claimed_sunk,
                received_event: claim.received_event.raw(),
            })
            .collect()
    };
    FinalOut {
        task_forces,
        ships,
        air_groups,
        bases,
        contacts: PerSide {
            allied: contacts(Side::Allied),
            japanese: contacts(Side::Japanese),
        },
        claims: PerSide {
            allied: claims(Side::Allied),
            japanese: claims(Side::Japanese),
        },
    }
}
