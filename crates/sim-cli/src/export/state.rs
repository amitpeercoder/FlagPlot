//! A snapshot of the truth (and, at the end, of each side's belief) as JSON.

use serde::Serialize;
use sim_core::hex::Hex;
use sim_core::orders::{AirMission, Reach};
use sim_core::refdata::{RefData, ShipKind};
use sim_core::state::{
    AirGroup, AirGroupLocation, Base, LandUnit, Ship, ShipLocation, ShipSystem, Side, TaskForce,
    TruthState,
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
    /// The land units (decision 0066). Left out when the scenario has none,
    /// so a file made before land units is unchanged.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub land_units: Vec<LandUnitOut>,
}

/// A land unit: its men ashore and the hex they are in (null with none
/// ashore). Men aboard ships are on the ship's `troops`.
#[derive(Serialize)]
pub struct LandUnitOut {
    id: u32,
    name: String,
    side: &'static str,
    men_ashore: u32,
    hex: Option<Pos>,
    /// "attack" (decision 0090). Left out on defend, so a file made where no
    /// unit attacks is unchanged.
    #[serde(skip_serializing_if = "is_defend")]
    posture: &'static str,
    /// Fatigue, 0 to 100 (decision 0090, 11). Left out when 0.
    #[serde(skip_serializing_if = "num_is_zero")]
    fatigue: Num,
    /// The share of its men fed at the last night's charge (decision 0090,
    /// 8). Left out when 1.
    #[serde(skip_serializing_if = "num_is_one")]
    fed_share: Num,
    /// The unit's march (decision 0094): the hex it marches to and the days
    /// marched. Left out when it has none, so a file made where no unit
    /// marches is unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    march: Option<MarchOut>,
}

/// A land unit's march in progress.
#[derive(Serialize)]
pub struct MarchOut {
    to: Pos,
    days: u32,
}

/// The men of one land unit aboard a ship.
#[derive(Serialize)]
struct TroopsOut {
    unit: u32,
    men: u32,
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
    /// toward (null at a centre), the miles covered between them and what the
    /// step is charged in all (decision 0058, 7).
    from_hex: Pos,
    toward_hex: Option<Pos>,
    progress_nm: Num,
    leg_nm: Num,
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
    /// Fuel carried as cargo (decision 0046). Left out when 0.
    #[serde(skip_serializing_if = "num_is_zero")]
    cargo_fuel: Num,
    flotation_damage: Num,
    fire_damage: Num,
    /// Only the systems the ship's class has.
    systems: Data,
    gun_rounds: Num,
    torpedoes_ready: bool,
    /// The men aboard (decision 0066). Left out when none.
    #[serde(skip_serializing_if = "Option::is_none")]
    troops: Option<TroopsOut>,
}

/// `{"task_force": n}`, `{"port": n}`, `"sunk"` or `{"yard": n}`, a ship
/// away at the yard with task force n (decision 0069).
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum LocationOut {
    TaskForce(u32),
    Port(u32),
    Sunk,
    Yard(u32),
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
    /// True for a group lost with the base it was at when the base changed
    /// side (decision 0090, 20): it has no base and no ship. Left out when
    /// false.
    #[serde(skip_serializing_if = "is_false")]
    lost_with_base: bool,
    aircraft_type: String,
    /// The type's number in `aircraft_types` (version 3): names need not be
    /// unique.
    aircraft_type_id: u32,
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
    /// "extended" for a group on extended reach (decision 0059). Left out on
    /// normal reach, so a replay of a game with no reach order is as it was.
    #[serde(skip_serializing_if = "Option::is_none")]
    reach: Option<&'static str>,
    /// The kinds the group goes for, in order, for a group whose list is not
    /// the default (decision 0063). Left out otherwise, so a replay of a game
    /// with no target order is as it was.
    #[serde(skip_serializing_if = "Option::is_none")]
    target_priority: Option<Vec<&'static str>>,
    arc: Option<ArcOut>,
}

#[derive(Serialize)]
struct ArcOut {
    from_deg: u32,
    to_deg: u32,
    range_nm: Num,
}

/// An aircraft type's reach figures, as every rule reads them: the strike
/// radius, the reach on extended reach (`RefData::reach_nm`, which is the
/// radius when the air combat figures are not loaded) and the search range.
#[derive(Serialize)]
pub struct AircraftTypeOut {
    id: u32,
    name: String,
    strike_radius_nm: Num,
    extended_reach_nm: Num,
    search_range_nm: Num,
}

pub fn aircraft_types(ref_data: &RefData) -> Vec<AircraftTypeOut> {
    ref_data
        .aircraft_types
        .iter()
        .map(|(id, aircraft)| AircraftTypeOut {
            id: id.raw(),
            name: aircraft.name.clone(),
            strike_radius_nm: Num(ref_data.reach_nm(*id, Reach::Normal)),
            extended_reach_nm: Num(ref_data.reach_nm(*id, Reach::Extended)),
            search_range_nm: Num(aircraft.search_range_nm),
        })
        .collect()
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
    /// Points of ship damage mended a day (decision 0069). Left out when 0.
    #[serde(skip_serializing_if = "num_is_zero")]
    ship_repair_points_per_day: Num,
    /// The days of passage to and from the yard, for a base that is a port
    /// for it (decision 0069). Left out when the base is not.
    #[serde(skip_serializing_if = "Option::is_none")]
    yard_passage_days: Option<u32>,
    /// False for a base a landing formed (decision 0066, 12): on its own
    /// side's map only. Left out when true.
    #[serde(skip_serializing_if = "is_true")]
    on_map: bool,
}

fn is_true(b: &bool) -> bool {
    *b
}

fn is_zero(n: &u32) -> bool {
    *n == 0
}

fn num_is_zero(n: &Num) -> bool {
    n.0 == sim_core::fixed::Fixed::ZERO
}

fn num_is_one(n: &Num) -> bool {
    n.0 == sim_core::fixed::Fixed::ONE
}

fn is_defend(posture: &&'static str) -> bool {
    *posture == "defend"
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
        land_units: truth.land_units.values().map(land_unit).collect(),
    }
}

fn land_unit(unit: &LandUnit) -> LandUnitOut {
    LandUnitOut {
        id: unit.id.raw(),
        name: unit.name.clone(),
        side: names::side(unit.side),
        men_ashore: unit.men_ashore,
        hex: unit.hex.map(Pos::from),
        posture: unit.posture.name(),
        fatigue: Num(unit.fatigue),
        fed_share: Num(unit.fed_share),
        march: unit.march.map(|march| MarchOut {
            to: Pos::from(march.to),
            days: march.days,
        }),
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
        ship_repair_points_per_day: Num(base.ship_repair_points_per_day),
        yard_passage_days: base.yard_passage_days,
        on_map: base.on_map,
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
        leg_nm: Num(force.leg_nm),
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
            ShipLocation::Yard(id) => LocationOut::Yard(id.raw()),
        },
        fuel: Num(ship.fuel_t),
        supply: Num(ship.supply_t),
        cargo_fuel: Num(ship.cargo_fuel_t),
        flotation_damage: Num(ship.flotation_damage),
        fire_damage: Num(ship.fire_damage),
        systems,
        gun_rounds: Num(ship.gun_rounds),
        torpedoes_ready: ship.torpedoes_ready,
        troops: ship.troops.map(|troops| TroopsOut {
            unit: troops.unit.raw(),
            men: troops.men,
        }),
    }
}

fn air_group(group: &AirGroup, ref_data: &RefData) -> AirGroupOut {
    AirGroupOut {
        id: group.id.raw(),
        name: group.name.clone(),
        side: names::side(group.side),
        base: match group.location {
            AirGroupLocation::Base(base) => Some(base.raw()),
            AirGroupLocation::Ship(_) | AirGroupLocation::Lost => None,
        },
        ship: match group.location {
            AirGroupLocation::Base(_) | AirGroupLocation::Lost => None,
            AirGroupLocation::Ship(ship) => Some(ship.raw()),
        },
        lost_with_base: group.location == AirGroupLocation::Lost,
        aircraft_type: ref_data.aircraft_types[&group.aircraft_type].name.clone(),
        aircraft_type_id: group.aircraft_type.raw(),
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
        reach: (group.reach == Reach::Extended).then_some(group.reach.name()),
        target_priority: (group.target_priority != ShipKind::ALL).then(|| {
            group
                .target_priority
                .iter()
                .map(|kind| kind.name())
                .collect()
        }),
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
    #[serde(skip_serializing_if = "Vec::is_empty")]
    land_units: Vec<LandUnitOut>,
    contacts: PerSide<ContactOut>,
    claims: PerSide<ClaimOut>,
    /// What each side believes of the enemy's men ashore (decision 0092,
    /// 19), beside its contacts. Left out when neither side holds an
    /// estimate.
    #[serde(skip_serializing_if = "PerSide::is_empty")]
    ground_estimates: PerSide<GroundEstimateOut>,
}

#[derive(Serialize)]
pub struct PerSide<T> {
    allied: Vec<T>,
    japanese: Vec<T>,
}

impl<T> PerSide<T> {
    fn is_empty(&self) -> bool {
        self.allied.is_empty() && self.japanese.is_empty()
    }
}

/// A side's estimate of the enemy's men ashore in a hex: the last ground
/// report's count, its day and the report's event. It may be wrong; the truth
/// is in `land_units`.
#[derive(Serialize)]
struct GroundEstimateOut {
    hex: (i32, i32),
    men: u32,
    day: u32,
    report_event: u32,
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
        land_units,
    } = snapshot(world.umpire(), world.ref_data());
    let key = world.umpire_key();
    let contacts = |side: Side| -> Vec<ContactOut> {
        world
            .side_view(side)
            .contacts()
            .map(|contact| ContactOut {
                id: contact.id.umpire_raw(&key),
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
                received_event: claim.received_event.umpire_raw(&key),
            })
            .collect()
    };
    let ground_estimates = |side: Side| -> Vec<GroundEstimateOut> {
        world
            .side_view(side)
            .ground_estimates()
            .map(|estimate| GroundEstimateOut {
                hex: (estimate.hex.q, estimate.hex.r),
                men: estimate.men,
                day: estimate.day,
                report_event: estimate.report_event.umpire_raw(&key),
            })
            .collect()
    };
    FinalOut {
        task_forces,
        ships,
        air_groups,
        bases,
        land_units,
        contacts: PerSide {
            allied: contacts(Side::Allied),
            japanese: contacts(Side::Japanese),
        },
        claims: PerSide {
            allied: claims(Side::Allied),
            japanese: claims(Side::Japanese),
        },
        ground_estimates: PerSide {
            allied: ground_estimates(Side::Allied),
            japanese: ground_estimates(Side::Japanese),
        },
    }
}
