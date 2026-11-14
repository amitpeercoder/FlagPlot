use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{
    AirGroupId, AircraftTypeId, BaseId, IntentId, LandUnitId, ShipClassId, ShipId, TaskForceId,
};
use sim_core::map::HexKind;
use sim_core::objectives::{Levels, Objective, Objectives, ShipPoints, SideObjectives};
use sim_core::orders::{
    Activity, AirGroupOrder, AirMission, Cargo, Issuer, Job, LandUnitOrder, Mission, OrderKind,
    OrderRequest, Origin, Posture, Reach, Readiness, SpeedSetting, Stance, StragglerInstruction,
    TaskForceOrder, WhenSpent,
};
use sim_core::refdata::{RefData, ShipKind};
use sim_core::state::{
    AirGroup, AirGroupLocation, Arrival, Base, LandUnit, Replacement, Ship, ShipLocation,
    ShipSystem, ShipSystems, Side, Skills, SystemState, TaskForce, TruthState,
};
use sim_core::world::MADE_IDS_FLOOR;
use sim_core::{DaySummary, StartingContact, StartingEstimate, World};

use crate::error::DataError;
use crate::map::{load_land_links, load_map};
use crate::quantity::Quantity;
use crate::reference::{
    load_air_combat_figures, load_air_supply_figures, load_air_upkeep_figures, load_aircraft_types,
    load_fatigue_figures, load_ground_figures, load_port_sizes, load_search_figures,
    load_ship_classes, load_surface_combat_figures, parse_kind,
};

/// One file name or a list of them (decision 0043).
#[derive(Deserialize)]
#[serde(untagged)]
enum FileList {
    One(String),
    Many(Vec<String>),
}

impl FileList {
    fn names(&self) -> Vec<&str> {
        match self {
            FileList::One(name) => vec![name.as_str()],
            FileList::Many(names) => names.iter().map(String::as_str).collect(),
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ScenarioFile {
    name: String,
    /// True when the scenario's units and geography are invented.
    placeholder: bool,
    seed: u64,
    /// How many days the scenario runs.
    days: u32,
    /// Path to the map file, relative to the scenario file.
    map: String,
    /// Path to a land links file, relative to the scenario file. Optional:
    /// its links are added to the map, whatever its format (decision 0094).
    land_links: Option<String>,
    /// Path to the ship class file, or a list of them, relative to the
    /// scenario file. Ids must not repeat across files.
    ship_classes: FileList,
    /// Path to the port size file, relative to the scenario file.
    port_sizes: String,
    /// Path to the aircraft type file, or a list of them, relative to the
    /// scenario file. Ids must not repeat across files.
    aircraft_types: FileList,
    /// Path to the search figures file, relative to the scenario file.
    search: String,
    /// Path to the surface combat figures file, relative to the scenario file.
    surface_combat: String,
    /// Path to the air combat figures file, relative to the scenario file.
    /// Optional: without it no air group can be ordered to strike.
    air_combat: Option<String>,
    /// Path to the wear and repair figures, relative to the scenario file.
    /// Optional: without it no aircraft is worn or repaired (decision 0035).
    air_upkeep: Option<String>,
    /// Path to the fatigue figures, relative to the scenario file. Optional:
    /// without it no air group tires or rests (decision 0036).
    fatigue: Option<String>,
    /// Path to the base supply figures, relative to the scenario file.
    /// Optional: without it nothing is spent and no base is out of supply
    /// (decision 0039).
    air_supply: Option<String>,
    /// Path to the ground figures, relative to the scenario file. Optional,
    /// but a scenario with a land unit must name it (decision 0066, 3).
    ground: Option<String>,
    #[serde(default)]
    base: Vec<BaseSpec>,
    #[serde(default)]
    land_unit: Vec<LandUnitSpec>,
    #[serde(default)]
    air_group: Vec<AirGroupSpec>,
    #[serde(default)]
    replacement: Vec<ReplacementSpec>,
    #[serde(default)]
    arrival: Vec<ArrivalSpec>,
    #[serde(default)]
    task_force: Vec<TaskForceSpec>,
    #[serde(default)]
    ship: Vec<ShipSpec>,
    #[serde(default)]
    order: Vec<OrderSpec>,
    #[serde(default)]
    contact: Vec<ContactSpec>,
    /// Opening estimates of the enemy's men ashore (decision 0092, 16).
    #[serde(default)]
    estimate: Vec<EstimateSpec>,
    /// Missions and objectives (decision 0054). Optional.
    objectives: Option<ObjectivesSpec>,
}

/// The `[objectives]` table (decision 0054, "As built"): the three levels,
/// an optional note of how the day went in history, and a table for each
/// side.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ObjectivesSpec {
    minor: Quantity,
    major: Quantity,
    decisive: Quantity,
    /// The days the result is judged over (decision 0088): a whole number,
    /// at least the scenario's days. Defaults to the scenario's days.
    horizon_days: Option<i64>,
    history: Option<String>,
    allied: SideObjectivesSpec,
    japanese: SideObjectivesSpec,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SideObjectivesSpec {
    mission: String,
    #[serde(default)]
    objective: Vec<ObjectiveSpec>,
}

/// One `[[objectives.<side>.objective]]`, by its `kind`.
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum ObjectiveSpec {
    /// `points` is a table from a ship kind (`carrier`, `battleship`,
    /// `cruiser`, `destroyer`, `transport`, `auxiliary`) to points.
    ShipsSunk {
        points: BTreeMap<String, Quantity>,
    },
    ShipsCrippled {
        points: BTreeMap<String, Quantity>,
    },
    /// Decision 0088: points for a ship of each kind away for the whole
    /// horizon.
    ShipsOutOfAction {
        points: BTreeMap<String, Quantity>,
    },
    AircraftDestroyed {
        points: Quantity,
    },
    CargoLanded {
        base: u32,
        points_per_t: Quantity,
    },
    BaseOpen {
        base: u32,
        points: Quantity,
        closed: Option<bool>,
    },
    /// Decision 0090, 25: points if the base is the side's at the end.
    BaseHeld {
        base: u32,
        points: Quantity,
    },
    /// Decision 0090, 26: points for each 100 enemy men lost.
    MenLost {
        points_per_100_men: Quantity,
    },
}

/// A contact a side holds when the game begins (decision 8 of
/// `docs/decisions/0021-readiness-and-alarm.md`).
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ContactSpec {
    side: SideSpec,
    q: i32,
    r: i32,
    ship_count: u32,
    /// A ship kind spelt as in the ship class file: `carrier`, `battleship`,
    /// `cruiser`, `destroyer`, `transport` or `auxiliary`.
    kind: String,
    /// How many pulses old the contact is at the start. Defaults to 0.
    age_pulses: Option<u32>,
}

/// An estimate of the enemy's men ashore in a hex that a side holds when the
/// game begins (decision 16 of 0092).
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EstimateSpec {
    side: SideSpec,
    q: i32,
    r: i32,
    men: u32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BaseSpec {
    id: u32,
    name: String,
    side: SideSpec,
    q: i32,
    r: i32,
    /// Supply stock at the start, in tons. Defaults to 0.
    supply_t: Option<Quantity>,
    /// Fuel stock for ships at the start, in tons (decision 0045). Defaults
    /// to 0. May not be negative.
    fuel_t: Option<Quantity>,
    /// Anti-aircraft shots at a raid on the base (decision 0034). Defaults
    /// to 0.
    anti_aircraft: Option<u32>,
    /// Damaged aircraft the base repairs each day (decision 0035). Defaults
    /// to 0.
    aircraft_repairs_per_day: Option<u32>,
    /// The points of ship damage the base mends in a day, shared by the
    /// forces of its side stopped there (decision 0069). Defaults to 0. May
    /// not be negative.
    ship_repair_points_per_day: Option<Quantity>,
    /// Makes the base a port for the yard, with a passage there and back of
    /// this many days (decision 0069). A whole number, at least 1. Defaults
    /// to none.
    yard_passage_days: Option<i64>,
    /// Whether the base is on the enemy's map at the start (decision 0092,
    /// "As built"). Defaults to true. A base off it cannot be bombarded or
    /// struck until a ground report finds it.
    on_map: Option<bool>,
    /// Whether the base has an airfield (decision 5 of 0099). Defaults to
    /// true.
    airfield: Option<bool>,
    /// The work already done on the airfield, 0 or more (decision 5 of
    /// 0099). Only for a base with `airfield = false`. Defaults to 0.
    airfield_work: Option<Quantity>,
    /// The base's defences at the start, 0 to 100 (decision 4 of 0099).
    /// Defaults to 0.
    defences: Option<Quantity>,
    /// The damage to the airfield at the start, 0 to 100 (decision 0103).
    /// Only for a base with an airfield. Defaults to 0.
    airfield_damage: Option<Quantity>,
}

/// A land unit at the start (decision 0066, 3): exactly one of a hex, where
/// its men are ashore, and a task force, where they are aboard.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LandUnitSpec {
    id: u32,
    name: String,
    side: SideSpec,
    men: u32,
    q: Option<i32>,
    r: Option<i32>,
    task_force: Option<u32>,
    /// The posture at the start, `defend` or `attack` (decision 0090, 1).
    /// Defaults to defend.
    posture: Option<PostureSpec>,
    /// Fatigue at the start, 0 to 100 (decision 0090, 11). Defaults to 0.
    fatigue: Option<Quantity>,
    /// Experience at the start, a whole 0 to 4 (decision 0097). Defaults to 2.
    experience: Option<u32>,
    /// Days of training counted at the start, a whole number below the
    /// ground figure `training_days_per_level` (decision 0097). Defaults to
    /// 0.
    training_days: Option<u32>,
    /// Days of fighting counted at the start, a whole number below the
    /// ground figure `fight_days_per_level` (decision 0097). Defaults to 0.
    fight_days: Option<u32>,
    /// What the unit is good at (decision 0098): each skill a whole number
    /// from 0 to 4; any not given is 2.
    skills: Option<SkillsSpec>,
    /// Marks a working unit (decision 1 of 0099): one whose men do not
    /// fight but work. Defaults to false. Not with `posture = "attack"`.
    working: Option<bool>,
    /// The fieldworks the unit has dug at the start (decision 0106): from 0
    /// to the ground figure `fieldworks_limit`, and 0 for a working unit,
    /// one on attack or one with no men ashore. Defaults to 0.
    fieldworks: Option<Quantity>,
}

/// A land unit's skills (decision 0098), each optional, defaulting to 2:
/// `assault`, `holding`, `weight`, and one movement skill for each kind of
/// terrain, `clear`, `jungle`, `mountain` and `atoll`.
#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct SkillsSpec {
    assault: Option<u32>,
    holding: Option<u32>,
    weight: Option<u32>,
    clear: Option<u32>,
    jungle: Option<u32>,
    mountain: Option<u32>,
    atoll: Option<u32>,
    works: Option<u32>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AirGroupSpec {
    id: u32,
    name: String,
    side: SideSpec,
    /// Exactly one of `base` and `ship` must be given.
    base: Option<u32>,
    ship: Option<u32>,
    aircraft_type: u32,
    /// Aircraft ready to fly at the start.
    ready: u32,
    /// Damaged aircraft at the start (decision 0035). Defaults to 0.
    damaged: Option<u32>,
    /// Fatigue at the start, 0 to 100 (decision 0036). Defaults to 0.
    fatigue: Option<Quantity>,
    /// Pilots at the start (decision 0038). Without it the group's pilots are
    /// not tracked and it flies as before.
    pilots: Option<u32>,
    /// The ceiling for replacements, in aircraft and in pilots (decision 10
    /// of 0038). Defaults to the aircraft the group starts with, ready and
    /// damaged.
    strength: Option<u32>,
}

/// A stream of replacements for a side at a base (decision 6 of 0038).
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ReplacementSpec {
    side: SideSpec,
    base: u32,
    aircraft_type: u32,
    aircraft_per_day: u32,
    pilots_per_day: u32,
}

/// Aircraft and pilots flown in to an air group on a day (decision 0041).
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ArrivalSpec {
    day: u32,
    air_group: u32,
    aircraft: u32,
    /// Defaults to 0. Only for a group whose pilots are tracked.
    pilots: Option<u32>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TaskForceSpec {
    id: u32,
    name: String,
    side: SideSpec,
    /// Where the task force starts: exactly one of `at_base`, or both `q`
    /// and `r` for a position at sea.
    at_base: Option<u32>,
    q: Option<i32>,
    r: Option<i32>,
    mission: MissionSpec,
    /// The task force's readiness at the start (decision 0021): `normal` or
    /// `battle`. Defaults to normal.
    readiness: Option<ReadinessSpec>,
    /// Fatigue at the start, 0 to 100 (decision 0037). Defaults to 0.
    fatigue: Option<Quantity>,
    /// The base the force heads for when it retires or sends stragglers
    /// home. It must be a base of the force's own side. Defaults to none.
    home_port: Option<u32>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ShipSpec {
    id: u32,
    name: String,
    class: u32,
    side: SideSpec,
    fuel_t: Quantity,
    /// Supply aboard at the start, in tons. Defaults to 0.
    supply_t: Option<Quantity>,
    /// Fuel carried as cargo at the start, in tons (decision 0046). Defaults
    /// to 0. With the supply aboard it must fit the class's cargo capacity.
    cargo_fuel_t: Option<Quantity>,
    /// Exactly one of `task_force` and `port` must be given.
    task_force: Option<u32>,
    port: Option<u32>,
    /// Gun rounds aboard at the start. Defaults to the guns times the
    /// rounds each carries.
    gun_rounds: Option<Quantity>,
    /// The crew's experience, a whole number from 0 to 4 (decision 0064).
    /// Defaults to 2. Set here and written by no rule.
    experience: Option<i64>,
    /// Flotation damage at the start, from 0 to below 100 (decision 0069): a
    /// ship sunk is not given this way. Defaults to 0.
    flotation_damage: Option<Quantity>,
    /// Fire damage at the start, from 0 to 100 (decision 0069). Defaults to 0.
    fire_damage: Option<Quantity>,
    /// The state of each named system at the start. Each defaults to
    /// working, and anything else needs a class that has the system.
    propulsion: Option<SystemStateSpec>,
    fire_control: Option<SystemStateSpec>,
    main_battery: Option<SystemStateSpec>,
    torpedo_mounts: Option<SystemStateSpec>,
    flight_deck: Option<SystemStateSpec>,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum SystemStateSpec {
    Working,
    Degraded,
    Out,
}

impl From<SystemStateSpec> for SystemState {
    fn from(spec: SystemStateSpec) -> SystemState {
        match spec {
            SystemStateSpec::Working => SystemState::Working,
            SystemStateSpec::Degraded => SystemState::Degraded,
            SystemStateSpec::Out => SystemState::Out,
        }
    }
}

#[derive(Deserialize)]
struct OrderSpec {
    /// The day the order takes effect. Day 1 is the first day.
    day: u32,
    side: SideSpec,
    /// Exactly one of `task_force`, `air_group` and `land_unit` must be
    /// given.
    task_force: Option<u32>,
    air_group: Option<u32>,
    /// A land unit, for a `set_posture`, `march` or `set_activity` order.
    land_unit: Option<u32>,
    #[serde(default)]
    issuer: IssuerSpec,
    intent: Option<u32>,
    #[serde(flatten)]
    action: ActionSpec,
}

// The variant names are the `kind` strings in scenario files. `OrderSpec`
// flattens this enum, which turns off its own unknown field check, so the
// check lives here instead. It only works on struct variants, which is why
// `Load` and `Unload` have empty braces.
/// What a load order takes aboard.
#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum CargoSpec {
    Supply,
    Fuel,
    /// Only with a unit (decision 0066): the men alone.
    #[serde(rename = "none")]
    NoCargo,
}

#[allow(clippy::enum_variant_names)]
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum ActionSpec {
    /// `act_at` marks waypoints to act at, by hex (decision 0096, 1); each
    /// must be one of the waypoints, which the order's intake checks.
    SetRoute {
        waypoints: Vec<[i32; 2]>,
        #[serde(default)]
        act_at: Vec<[i32; 2]>,
    },
    /// Exactly one of `speed` and `knots` (decision 7 of 0021).
    SetSpeed {
        speed: Option<SpeedSpec>,
        knots: Option<Quantity>,
    },
    SetMission {
        mission: MissionSpec,
    },
    /// `cargo` is "supply" or "fuel" and defaults to supply (decision 0046).
    /// With a `unit` (decision 0066) it may also be "none".
    Load {
        cargo: Option<CargoSpec>,
        unit: Option<u32>,
    },
    Unload {},
    /// `target_base` is required for `base_strike` and given for no other
    /// mission (decision 0034).
    SetAirMission {
        mission: AirMissionSpec,
        target_base: Option<u32>,
    },
    SetSearchArc {
        from_deg: i32,
        to_deg: i32,
        range_nm: Quantity,
    },
    /// The share of its aircraft a group on combat air patrol keeps over
    /// its hex (decision 0027).
    SetPatrolShare {
        share: Quantity,
    },
    /// The fatigue above which the staff stands the group down (decision
    /// 0036).
    SetRestThreshold {
        fatigue: Quantity,
    },
    /// `reach` is "normal" or "extended" (decision 0059).
    SetReach {
        reach: ReachSpec,
    },
    /// `kinds` are ship kinds by name, in the order the group goes for them
    /// (decision 0063). An empty list or a kind named twice is refused when
    /// the order reaches intake, as an out-of-range value is.
    SetTargetPriority {
        kinds: Vec<String>,
    },
    /// Move the air group to the named base of its side (decision 0091). A
    /// base that is not the side's, or out of reach, is refused at intake.
    Rebase {
        base: u32,
    },
    SetRulesOfEngagement {
        stance: StanceSpec,
        break_off: Quantity,
        #[serde(default)]
        stragglers: StragglersSpec,
        keep_share: Option<Quantity>,
    },
    SetHomePort {
        base: u32,
    },
    SetReadiness {
        readiness: ReadinessSpec,
    },
    Detach {
        ships: Vec<u32>,
    },
    /// Send the whole force to the yard from the base it is stopped at
    /// (decision 0069). No other key.
    SendToYard {},
    /// A land unit's posture, `defend` or `attack` (decision 0090). An
    /// attack order may carry `when_spent`, `hold` or `fall_back`: what the
    /// unit does when its attack is spent (decision 6 of 0105).
    SetPosture {
        posture: PostureSpec,
        #[serde(default)]
        when_spent: Option<WhenSpentSpec>,
    },
    /// March a land unit to a neighbouring hex (decision 0094). The hex is
    /// `[q, r]`.
    March {
        hex: [i32; 2],
    },
    /// Prepare a landing at a coast hex with the listed land units as the
    /// roster (decision 0095). The hex is `[q, r]`; the roster may be empty.
    PrepareLanding {
        hex: [i32; 2],
        roster: Vec<u32>,
    },
    /// A land unit's activity, `ready` or `train` (decisions 0095 and 0097).
    /// With `work`, an optional `job`: `airfield`, `port`, `aircraft` or
    /// `defences` (decision 0099).
    SetActivity {
        activity: ActivitySpec,
        job: Option<String>,
    },
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum ActivitySpec {
    Ready,
    Train,
    Work,
}

impl From<ActivitySpec> for Activity {
    fn from(spec: ActivitySpec) -> Activity {
        match spec {
            ActivitySpec::Ready => Activity::Ready,
            ActivitySpec::Train => Activity::Train,
            ActivitySpec::Work => Activity::Work(None),
        }
    }
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum PostureSpec {
    Defend,
    Attack,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum WhenSpentSpec {
    Hold,
    FallBack,
}

impl From<WhenSpentSpec> for WhenSpent {
    fn from(spec: WhenSpentSpec) -> WhenSpent {
        match spec {
            WhenSpentSpec::Hold => WhenSpent::Hold,
            WhenSpentSpec::FallBack => WhenSpent::FallBack,
        }
    }
}

impl From<PostureSpec> for Posture {
    fn from(spec: PostureSpec) -> Posture {
        match spec {
            PostureSpec::Defend => Posture::Defend,
            PostureSpec::Attack => Posture::Attack,
        }
    }
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum ReachSpec {
    Normal,
    Extended,
}

impl From<ReachSpec> for Reach {
    fn from(spec: ReachSpec) -> Reach {
        match spec {
            ReachSpec::Normal => Reach::Normal,
            ReachSpec::Extended => Reach::Extended,
        }
    }
}

#[derive(Deserialize, Clone, Copy, Default)]
#[serde(rename_all = "snake_case")]
enum StragglersSpec {
    #[default]
    Keep,
    Detach,
    Leave,
    Retire,
}

impl From<StragglersSpec> for StragglerInstruction {
    fn from(spec: StragglersSpec) -> StragglerInstruction {
        match spec {
            StragglersSpec::Keep => StragglerInstruction::Keep,
            StragglersSpec::Detach => StragglerInstruction::Detach,
            StragglersSpec::Leave => StragglerInstruction::Leave,
            StragglersSpec::Retire => StragglerInstruction::Retire,
        }
    }
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum ReadinessSpec {
    Normal,
    Battle,
}

impl From<ReadinessSpec> for Readiness {
    fn from(spec: ReadinessSpec) -> Readiness {
        match spec {
            ReadinessSpec::Normal => Readiness::Normal,
            ReadinessSpec::Battle => Readiness::Battle,
        }
    }
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum StanceSpec {
    Engage,
    Avoid,
}

impl From<StanceSpec> for Stance {
    fn from(spec: StanceSpec) -> Stance {
        match spec {
            StanceSpec::Engage => Stance::Engage,
            StanceSpec::Avoid => Stance::Avoid,
        }
    }
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum AirMissionSpec {
    Rest,
    Search,
    NavalStrike,
    CombatAirPatrol,
    BaseStrike,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum SideSpec {
    Allied,
    Japanese,
}

impl From<SideSpec> for Side {
    fn from(spec: SideSpec) -> Side {
        match spec {
            SideSpec::Allied => Side::Allied,
            SideSpec::Japanese => Side::Japanese,
        }
    }
}

#[derive(Deserialize, Clone, Copy, Default)]
#[serde(rename_all = "snake_case")]
enum IssuerSpec {
    #[default]
    Player,
    Staff,
}

impl From<IssuerSpec> for Issuer {
    fn from(spec: IssuerSpec) -> Issuer {
        match spec {
            IssuerSpec::Player => Issuer::Player,
            IssuerSpec::Staff => Issuer::Staff,
        }
    }
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum MissionSpec {
    Transport,
    SurfaceCombat,
    Carrier,
    Bombardment,
    Escort,
}

impl From<MissionSpec> for Mission {
    fn from(spec: MissionSpec) -> Mission {
        match spec {
            MissionSpec::Transport => Mission::Transport,
            MissionSpec::SurfaceCombat => Mission::SurfaceCombat,
            MissionSpec::Carrier => Mission::Carrier,
            MissionSpec::Bombardment => Mission::Bombardment,
            MissionSpec::Escort => Mission::Escort,
        }
    }
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum SpeedSpec {
    Cruise,
    Full,
}

impl From<SpeedSpec> for SpeedSetting {
    fn from(spec: SpeedSpec) -> SpeedSetting {
        match spec {
            SpeedSpec::Cruise => SpeedSetting::Cruise,
            SpeedSpec::Full => SpeedSetting::Full,
        }
    }
}

fn points_of(quantity: &Quantity, what: &str) -> Result<Fixed, String> {
    let points = quantity.to_fixed().map_err(|e| format!("{what} {e}"))?;
    if points.is_negative() {
        return Err(format!("{what} is negative"));
    }
    Ok(points)
}

fn ship_points(table: &BTreeMap<String, Quantity>, what: &str) -> Result<ShipPoints, String> {
    let mut points = BTreeMap::new();
    for (name, quantity) in table {
        let kind: ShipKind =
            parse_kind(name).ok_or_else(|| format!("{what} has unknown ship kind \"{name}\""))?;
        points.insert(kind, points_of(quantity, &format!("{what} {name}"))?);
    }
    Ok(ShipPoints(points))
}

/// Builds the objectives from their table. Named bases are checked against
/// the game by `World::set_objectives`.
///
/// `days` is the scenario's length, which is the horizon unless the table
/// states a longer one. `has_surface_combat` says whether the days of repair
/// can be worked out: `ships_out_of_action` reads the yard figures in the
/// surface combat file (decision 0088, "As built": the loader refuses the
/// objective without them).
fn build_objectives(
    spec: &ObjectivesSpec,
    days: u32,
    has_surface_combat: bool,
) -> Result<Objectives, String> {
    let horizon_days = match spec.horizon_days {
        None => days,
        Some(stated) => {
            let stated = u32::try_from(stated)
                .ok()
                .filter(|stated| *stated >= 1)
                .ok_or_else(|| {
                    format!("objectives horizon_days {stated} is not a whole number of at least 1")
                })?;
            if stated < days {
                return Err(format!(
                    "objectives horizon_days {stated} is shorter than the scenario's {days} days"
                ));
            }
            stated
        }
    };
    let uses = |wanted: &str| {
        [&spec.allied, &spec.japanese]
            .into_iter()
            .flat_map(|side| side.objective.iter())
            .any(|objective| {
                matches!(
                    (wanted, objective),
                    (
                        "ships_out_of_action",
                        ObjectiveSpec::ShipsOutOfAction { .. }
                    ) | ("ships_crippled", ObjectiveSpec::ShipsCrippled { .. })
                )
            })
    };
    if uses("ships_out_of_action") {
        if uses("ships_crippled") {
            return Err(
                "a scenario counts damaged ships one way: it may not have both ships_out_of_action and ships_crippled (decision 0088)"
                    .to_string(),
            );
        }
        if !has_surface_combat {
            return Err(
                "ships_out_of_action needs the surface combat figures, which hold the yard figures (decision 0088)"
                    .to_string(),
            );
        }
    }
    let level = |quantity: &Quantity, name: &str| {
        quantity
            .to_fixed()
            .map_err(|e| format!("objectives level {name} {e}"))
    };
    let (minor, major, decisive) = (
        level(&spec.minor, "minor")?,
        level(&spec.major, "major")?,
        level(&spec.decisive, "decisive")?,
    );
    let levels = Levels::new(minor, major, decisive).ok_or_else(|| {
        format!(
            "objectives levels must be above zero and rising, but minor is {minor}, major {major} and decisive {decisive}"
        )
    })?;
    let side = |name: &str, spec: &SideObjectivesSpec| -> Result<SideObjectives, String> {
        if spec.mission.trim().is_empty() {
            return Err(format!("the {name} mission is empty"));
        }
        let mut objectives = Vec::new();
        for (index, objective) in spec.objective.iter().enumerate() {
            let what = format!("{name} objective {}", index + 1);
            objectives.push(match objective {
                ObjectiveSpec::ShipsSunk { points } => {
                    Objective::ShipsSunk(ship_points(points, &what)?)
                }
                ObjectiveSpec::ShipsCrippled { points } => {
                    Objective::ShipsCrippled(ship_points(points, &what)?)
                }
                ObjectiveSpec::ShipsOutOfAction { points } => {
                    Objective::ShipsOutOfAction(ship_points(points, &what)?)
                }
                ObjectiveSpec::AircraftDestroyed { points } => Objective::AircraftDestroyed {
                    points: points_of(points, &format!("{what} points"))?,
                },
                ObjectiveSpec::CargoLanded { base, points_per_t } => Objective::CargoLanded {
                    base: BaseId::new(*base),
                    points_per_t: points_of(points_per_t, &format!("{what} points_per_t"))?,
                },
                ObjectiveSpec::BaseOpen {
                    base,
                    points,
                    closed,
                } => Objective::BaseOpen {
                    base: BaseId::new(*base),
                    closed: closed.unwrap_or(false),
                    points: points_of(points, &format!("{what} points"))?,
                },
                ObjectiveSpec::BaseHeld { base, points } => Objective::BaseHeld {
                    base: BaseId::new(*base),
                    points: points_of(points, &format!("{what} points"))?,
                },
                ObjectiveSpec::MenLost { points_per_100_men } => Objective::MenLost {
                    points_per_100_men: points_of(
                        points_per_100_men,
                        &format!("{what} points_per_100_men"),
                    )?,
                },
            });
        }
        Ok(SideObjectives {
            mission: spec.mission.clone(),
            objectives,
        })
    };
    Ok(Objectives {
        allied: side("allied", &spec.allied)?,
        japanese: side("japanese", &spec.japanese)?,
        levels,
        horizon_days,
        history: spec.history.clone(),
    })
}

/// A scenario ready to run: a game at its starting state plus the orders each
/// side will give on each day.
#[derive(Clone, Debug)]
pub struct Scenario {
    pub name: String,
    /// True when the scenario's units and geography are invented.
    pub placeholder: bool,
    /// How many days the scenario runs.
    pub days: u32,
    pub world: World,
    script: BTreeMap<u32, Vec<(Side, OrderRequest)>>,
    /// The contacts the sides start with, kept so that a world built afresh
    /// for another seed holds them too.
    contacts: Vec<StartingContact>,
    /// The opening estimates, kept for the same reason (decision 0092, 16).
    estimates: Vec<StartingEstimate>,
    /// Every file the scenario names, as the paths it reads them from, in a
    /// fixed order (decision 0043, 9).
    data_files: Vec<PathBuf>,
}

/// How a scenario is loaded, beyond the file itself.
///
/// The default reads every file where the scenario names it.
#[derive(Clone, Debug, Default)]
pub struct LoadOptions {
    /// A directory to read the scenario's reference (figures) files from, in
    /// place of the places it names. Each reference file the scenario names
    /// (ship classes, port sizes, aircraft types, and the search, surface
    /// combat, air combat, air upkeep, fatigue and base supply figures) is
    /// looked up by its file name directly under this directory. A file that
    /// is not there is an error that names it: there is no fallback to the
    /// file the scenario names, which would defeat the purpose. The map is
    /// not a reference file and is read where the scenario names it.
    ///
    /// The hand-calculation tests set this to `data/reference/step1`, the
    /// figures as they stood at the end of step 1 (`docs/step2-spec.md`,
    /// "Tests while figures move").
    pub reference_root: Option<PathBuf>,
}

/// Where the frozen figures of step 1 are, relative to the repository's root.
/// The one place the directory is named in code: the tests that read them
/// (`tests/common`, the agent's tests, the runner's tests) all go through
/// [`LoadOptions::frozen_step1`].
pub const FROZEN_REFERENCE_DIR: &str = "data/reference/step1";

impl LoadOptions {
    /// Options that read the reference files from `root`.
    pub fn with_reference_root(root: impl Into<PathBuf>) -> LoadOptions {
        LoadOptions {
            reference_root: Some(root.into()),
        }
    }

    /// Options that read the frozen figures of step 1, as the hand-calculation
    /// tests do, given the repository's root (`FROZEN_REFERENCE_DIR` under it).
    pub fn frozen_step1(repo_root: &Path) -> LoadOptions {
        LoadOptions::with_reference_root(repo_root.join(FROZEN_REFERENCE_DIR))
    }
}

impl Scenario {
    /// Loads a scenario and the map and reference files it names.
    pub fn load(path: &Path) -> Result<Scenario, DataError> {
        Scenario::load_with(path, &LoadOptions::default())
    }

    /// Loads a scenario as [`Scenario::load`] does, with `options`. The files
    /// the scenario records as its inputs ([`Scenario::data_files`]) are the
    /// ones actually read, so a saved game made with a reference root names
    /// that root's files and its hashes are theirs.
    pub fn load_with(path: &Path, options: &LoadOptions) -> Result<Scenario, DataError> {
        let fail = |message: String| DataError::in_file(path, message);

        let text = fs::read_to_string(path).map_err(|e| fail(e.to_string()))?;
        Scenario::parse_with(&text, path, options)
    }

    /// Builds a scenario from text that is not (or no longer) the file's own
    /// bytes, for tests that load a file with its tables in another order
    /// (decision 0043, 2). `path` says where the text would sit: the files it
    /// names are read relative to it.
    pub fn from_text(text: &str, path: &Path) -> Result<Scenario, DataError> {
        Scenario::parse(text, path)
    }

    /// As [`Scenario::from_text`], with `options` as in [`Scenario::load_with`].
    pub fn from_text_with(
        text: &str,
        path: &Path,
        options: &LoadOptions,
    ) -> Result<Scenario, DataError> {
        Scenario::parse_with(text, path, options)
    }

    /// Builds a scenario from its text. `path` says where the text came from:
    /// the files it names are read relative to it.
    fn parse(text: &str, path: &Path) -> Result<Scenario, DataError> {
        Scenario::parse_with(text, path, &LoadOptions::default())
    }

    /// As `parse`, reading the reference files as `options` say.
    fn parse_with(text: &str, path: &Path, options: &LoadOptions) -> Result<Scenario, DataError> {
        let fail = |message: String| DataError::in_file(path, message);

        let file: ScenarioFile = toml::from_str(text).map_err(|e| fail(e.to_string()))?;

        let folder = path.parent().unwrap_or_else(|| Path::new("."));
        // Where a named reference file is read from: where the scenario names
        // it, or by file name under the options' reference root.
        let reference = |name: &str| -> Result<PathBuf, DataError> {
            match &options.reference_root {
                None => Ok(folder.join(name)),
                Some(root) => {
                    let file_name = Path::new(name).file_name().ok_or_else(|| {
                        fail(format!("\"{name}\" does not name a reference file"))
                    })?;
                    let found = root.join(file_name);
                    if found.is_file() {
                        Ok(found)
                    } else {
                        Err(fail(format!(
                            "the reference file {} is not in the reference root {}",
                            file_name.to_string_lossy(),
                            root.display()
                        )))
                    }
                }
            }
        };
        let mut map = load_map(&folder.join(&file.map))?;
        let land_links_file = file.land_links.as_ref().map(|name| folder.join(name));
        if let Some(links) = &land_links_file {
            load_land_links(links, &mut map)?;
        }
        let class_files = file.ship_classes.names();
        if class_files.is_empty() {
            return Err(fail("ship_classes names no file".to_string()));
        }
        let type_files = file.aircraft_types.names();
        if type_files.is_empty() {
            return Err(fail("aircraft_types names no file".to_string()));
        }
        let mut ref_data = RefData::default();
        let mut class_owner: BTreeMap<ShipClassId, PathBuf> = BTreeMap::new();
        for name in &class_files {
            let file_path = reference(name)?;
            for (id, class) in load_ship_classes(&file_path)?.ship_classes {
                if let Some(first) = class_owner.insert(id, file_path.clone()) {
                    return Err(fail(format!(
                        "ship class {id} is in both {} and {}",
                        first.display(),
                        file_path.display()
                    )));
                }
                ref_data.ship_classes.insert(id, class);
            }
        }
        ref_data.port_sizes = load_port_sizes(&reference(&file.port_sizes)?)?;
        let mut type_owner: BTreeMap<AircraftTypeId, PathBuf> = BTreeMap::new();
        for name in &type_files {
            let file_path = reference(name)?;
            for (id, aircraft_type) in load_aircraft_types(&file_path)? {
                if let Some(first) = type_owner.insert(id, file_path.clone()) {
                    return Err(fail(format!(
                        "aircraft type {id} is in both {} and {}",
                        first.display(),
                        file_path.display()
                    )));
                }
                ref_data.aircraft_types.insert(id, aircraft_type);
            }
        }
        ref_data.search = Some(load_search_figures(&reference(&file.search)?)?);
        ref_data.surface_combat = Some(load_surface_combat_figures(&reference(
            &file.surface_combat,
        )?)?);
        if let Some(air_combat) = &file.air_combat {
            ref_data.air_combat = Some(load_air_combat_figures(&reference(air_combat)?)?);
        }
        if let Some(air_upkeep) = &file.air_upkeep {
            ref_data.air_upkeep = Some(load_air_upkeep_figures(&reference(air_upkeep)?)?);
        }
        if let Some(fatigue) = &file.fatigue {
            ref_data.fatigue = Some(load_fatigue_figures(&reference(fatigue)?)?);
        }
        if let Some(air_supply) = &file.air_supply {
            ref_data.air_supply = Some(load_air_supply_figures(&reference(air_supply)?)?);
        }
        if let Some(ground) = &file.ground {
            ref_data.ground = Some(load_ground_figures(&reference(ground)?)?);
        }

        let mut data_files: Vec<PathBuf> = vec![folder.join(&file.map)];
        data_files.extend(land_links_file);
        for name in &class_files {
            data_files.push(reference(name)?);
        }
        data_files.push(reference(&file.port_sizes)?);
        for name in &type_files {
            data_files.push(reference(name)?);
        }
        data_files.push(reference(&file.search)?);
        data_files.push(reference(&file.surface_combat)?);
        for name in [
            &file.air_combat,
            &file.air_upkeep,
            &file.fatigue,
            &file.air_supply,
            &file.ground,
        ]
        .into_iter()
        .flatten()
        {
            data_files.push(reference(name)?);
        }

        let mut truth = TruthState::default();

        for spec in &file.base {
            let id = BaseId::new(spec.id);
            let airfield = spec.airfield.unwrap_or(true);
            let airfield_work = match &spec.airfield_work {
                Some(quantity) => {
                    let work = quantity
                        .to_fixed()
                        .map_err(|e| fail(format!("{id}: airfield_work {e}")))?;
                    if work.is_negative() {
                        return Err(fail(format!("{id}: airfield_work is negative")));
                    }
                    if airfield {
                        return Err(fail(format!(
                            "{id}: airfield_work is given for a base that has an airfield, which has nothing more to build; give it only with airfield = false"
                        )));
                    }
                    work
                }
                None => Fixed::ZERO,
            };
            let defences = match &spec.defences {
                Some(quantity) => {
                    let defences = quantity
                        .to_fixed()
                        .map_err(|e| fail(format!("{id}: defences {e}")))?;
                    if defences.is_negative() || defences > Fixed::from_int(100) {
                        return Err(fail(format!(
                            "{id}: defences {defences} is not between 0 and 100"
                        )));
                    }
                    defences
                }
                None => Fixed::ZERO,
            };
            let airfield_damage = match &spec.airfield_damage {
                Some(quantity) => {
                    let damage = quantity
                        .to_fixed()
                        .map_err(|e| fail(format!("{id}: airfield_damage {e}")))?;
                    if damage.is_negative() || damage > Fixed::from_int(100) {
                        return Err(fail(format!(
                            "{id}: airfield_damage {damage} is not between 0 and 100"
                        )));
                    }
                    if !airfield {
                        return Err(fail(format!(
                            "{id}: airfield_damage is given for a base that has no airfield, which has none to damage; give it only for a base with an airfield"
                        )));
                    }
                    damage
                }
                None => Fixed::ZERO,
            };
            let base = Base {
                id,
                name: spec.name.clone(),
                side: spec.side.into(),
                hex: Hex::new(spec.q, spec.r),
                supply_t: match &spec.supply_t {
                    Some(quantity) => quantity
                        .to_fixed()
                        .map_err(|e| fail(format!("{id}: supply_t {e}")))?,
                    None => Fixed::ZERO,
                },
                anti_aircraft: spec.anti_aircraft.unwrap_or(0),
                aircraft_repairs_per_day: spec.aircraft_repairs_per_day.unwrap_or(0),
                fuel_t: match &spec.fuel_t {
                    Some(quantity) => {
                        let fuel = quantity
                            .to_fixed()
                            .map_err(|e| fail(format!("{id}: fuel_t {e}")))?;
                        if fuel < Fixed::ZERO {
                            return Err(fail(format!("{id}: fuel_t is negative")));
                        }
                        fuel
                    }
                    None => Fixed::ZERO,
                },
                ship_repair_points_per_day: match &spec.ship_repair_points_per_day {
                    Some(quantity) => {
                        let points = quantity
                            .to_fixed()
                            .map_err(|e| fail(format!("{id}: ship_repair_points_per_day {e}")))?;
                        if points < Fixed::ZERO {
                            return Err(fail(format!(
                                "{id}: ship_repair_points_per_day is negative"
                            )));
                        }
                        points
                    }
                    None => Fixed::ZERO,
                },
                yard_passage_days: match spec.yard_passage_days {
                    Some(days) => Some(
                        u32::try_from(days)
                            .ok()
                            .filter(|days| *days >= 1)
                            .ok_or_else(|| {
                                fail(format!(
                                    "{id}: yard_passage_days {days} is not a whole number of at least 1"
                                ))
                            })?,
                    ),
                    None => None,
                },
                on_map: spec.on_map.unwrap_or(true),
                airfield,
                airfield_work,
                defences,
                day_work: None,
                airfield_damage,
            };
            if truth.bases.insert(id, base).is_some() {
                return Err(fail(format!("{id} appears twice")));
            }
        }

        for spec in &file.task_force {
            let id = TaskForceId::new(spec.id);
            if spec.id >= MADE_IDS_FLOOR {
                return Err(fail(format!(
                    "{id} is numbered {MADE_IDS_FLOOR} or above: those numbers are kept for forces made in play (decision 0086)"
                )));
            }
            let hex = match (spec.at_base, spec.q, spec.r) {
                (Some(raw), None, None) => {
                    truth
                        .bases
                        .get(&BaseId::new(raw))
                        .ok_or_else(|| fail(format!("{id} starts at unknown base {raw}")))?
                        .hex
                }
                (None, Some(q), Some(r)) => Hex::new(q, r),
                _ => {
                    return Err(fail(format!(
                        "{id} needs exactly one of at_base, or both q and r"
                    )))
                }
            };
            let mut task_force = TaskForce::at(
                id,
                spec.name.clone(),
                spec.side.into(),
                hex,
                spec.mission.into(),
            );
            task_force.readiness = spec.readiness.map_or(Readiness::Normal, Readiness::from);
            if let Some(quantity) = &spec.fatigue {
                let fatigue = quantity
                    .to_fixed()
                    .map_err(|e| fail(format!("{id} fatigue {e}")))?;
                if fatigue < Fixed::ZERO || fatigue > Fixed::from_int(100) {
                    return Err(fail(format!(
                        "{id} has a fatigue of {fatigue}, not between 0 and 100"
                    )));
                }
                task_force.fatigue = fatigue;
            }
            if let Some(raw) = spec.home_port {
                let port = BaseId::new(raw);
                let base = truth
                    .bases
                    .get(&port)
                    .ok_or_else(|| fail(format!("{id} has unknown home port {raw}")))?;
                if base.side != task_force.side {
                    return Err(fail(format!(
                        "{id} is {} but its home port {port} is {}",
                        task_force.side.name(),
                        base.side.name()
                    )));
                }
                task_force.home_port = Some(port);
            }
            if truth.task_forces.insert(id, task_force).is_some() {
                return Err(fail(format!("{id} appears twice")));
            }
        }

        for spec in &file.ship {
            let id = ShipId::new(spec.id);
            let class = ShipClassId::new(spec.class);
            if !ref_data.ship_classes.contains_key(&class) {
                return Err(fail(format!("{id} has unknown class {}", spec.class)));
            }
            let location = match (spec.task_force, spec.port) {
                (Some(raw), None) => {
                    let task_force_id = TaskForceId::new(raw);
                    truth
                        .task_forces
                        .get_mut(&task_force_id)
                        .ok_or_else(|| fail(format!("{id} is in unknown task force {raw}")))?
                        .ships
                        .insert(id);
                    ShipLocation::TaskForce(task_force_id)
                }
                (None, Some(raw)) => {
                    let base_id = BaseId::new(raw);
                    if !truth.bases.contains_key(&base_id) {
                        return Err(fail(format!("{id} is in unknown port {raw}")));
                    }
                    ShipLocation::Port(base_id)
                }
                _ => {
                    return Err(fail(format!(
                        "{id} needs exactly one of task_force and port"
                    )))
                }
            };
            let ship_class = &ref_data.ship_classes[&class];
            let gun_rounds = match &spec.gun_rounds {
                Some(quantity) => quantity
                    .to_fixed()
                    .map_err(|e| fail(format!("{id}: gun_rounds {e}")))?,
                None => Fixed::from_int(
                    i64::from(ship_class.main_guns) * i64::from(ship_class.gun_rounds_per_gun),
                ),
            };
            let experience = match spec.experience {
                None => Ship::DEFAULT_EXPERIENCE,
                Some(value) => u8::try_from(value)
                    .ok()
                    .filter(|value| *value <= Ship::MAX_EXPERIENCE)
                    .ok_or_else(|| {
                        fail(format!(
                            "{id}: experience {value} is not a whole number from 0 to {}",
                            Ship::MAX_EXPERIENCE
                        ))
                    })?,
            };
            let flotation_damage = match &spec.flotation_damage {
                Some(quantity) => {
                    let damage = quantity
                        .to_fixed()
                        .map_err(|e| fail(format!("{id}: flotation_damage {e}")))?;
                    if damage < Fixed::ZERO || damage >= Fixed::from_int(100) {
                        return Err(fail(format!(
                            "{id}: flotation_damage {damage} is not from 0 to below 100"
                        )));
                    }
                    damage
                }
                None => Fixed::ZERO,
            };
            let fire_damage = match &spec.fire_damage {
                Some(quantity) => {
                    let damage = quantity
                        .to_fixed()
                        .map_err(|e| fail(format!("{id}: fire_damage {e}")))?;
                    if damage < Fixed::ZERO || damage > Fixed::from_int(100) {
                        return Err(fail(format!(
                            "{id}: fire_damage {damage} is not from 0 to 100"
                        )));
                    }
                    damage
                }
                None => Fixed::ZERO,
            };
            let mut systems = ShipSystems::ALL_WORKING;
            for (system, wanted) in [
                (ShipSystem::Propulsion, spec.propulsion),
                (ShipSystem::FireControl, spec.fire_control),
                (ShipSystem::MainBattery, spec.main_battery),
                (ShipSystem::TorpedoMounts, spec.torpedo_mounts),
                (ShipSystem::FlightDeck, spec.flight_deck),
            ] {
                let Some(wanted) = wanted else {
                    continue;
                };
                let state: SystemState = wanted.into();
                if state != SystemState::Working && !ship_class.has_system(system) {
                    return Err(fail(format!(
                        "{id} is {} {} but its class {} has no {}",
                        system.name(),
                        state.name(),
                        spec.class,
                        system.name()
                    )));
                }
                systems.set(system, state);
            }
            let fuel_t = spec
                .fuel_t
                .to_fixed()
                .map_err(|e| fail(format!("{id}: fuel_t {e}")))?;
            // A ship cannot start with more fuel than its class holds. This
            // catches a scenario left behind when a class's figures change.
            if fuel_t > ship_class.fuel_capacity_t {
                return Err(fail(format!(
                    "{id} starts with {fuel_t} t of fuel but its class {} holds {} t",
                    spec.class, ship_class.fuel_capacity_t
                )));
            }
            let supply_t = match &spec.supply_t {
                Some(quantity) => quantity
                    .to_fixed()
                    .map_err(|e| fail(format!("{id}: supply_t {e}")))?,
                None => Fixed::ZERO,
            };
            let cargo_fuel_t = match &spec.cargo_fuel_t {
                Some(quantity) => quantity
                    .to_fixed()
                    .map_err(|e| fail(format!("{id}: cargo_fuel_t {e}")))?,
                None => Fixed::ZERO,
            };
            if cargo_fuel_t.is_negative() {
                return Err(fail(format!("{id}: cargo_fuel_t is negative")));
            }
            // Supply and fuel cargo share the class's cargo capacity
            // (decision 0046).
            if supply_t + cargo_fuel_t > ship_class.cargo_capacity_t {
                return Err(fail(format!(
                    "{id} starts with {supply_t} t of supply and {cargo_fuel_t} t of fuel cargo but its class {} carries {} t",
                    spec.class, ship_class.cargo_capacity_t
                )));
            }
            let ship = Ship {
                id,
                name: spec.name.clone(),
                class,
                side: spec.side.into(),
                fuel_t,
                supply_t,
                cargo_fuel_t,
                location,
                flotation_damage,
                fire_damage,
                gun_rounds,
                torpedoes_ready: ship_class.torpedo_tubes > 0,
                torpedoes_one_side_fired: false,
                systems,
                experience,
                system_repair_points: Fixed::ZERO,
                troops: None,
            };
            if truth.ships.insert(id, ship).is_some() {
                return Err(fail(format!("{id} appears twice")));
            }
        }

        // Land units come after ships, since a unit may be aboard some
        // (decision 0066, 3).
        if !file.land_unit.is_empty() && file.ground.is_none() {
            return Err(fail(
                "the scenario has a land unit but names no ground file (decision 0066, 3)"
                    .to_string(),
            ));
        }
        let mut unit_specs: Vec<&LandUnitSpec> = file.land_unit.iter().collect();
        unit_specs.sort_by_key(|spec| spec.id);
        // Units are put aboard in unit order, each over the force's ships in
        // ship order.
        let mut to_put_aboard: Vec<(LandUnitId, TaskForceId, u32)> = Vec::new();
        for spec in unit_specs {
            let id = LandUnitId::new(spec.id);
            if spec.id == 0 || spec.id >= MADE_IDS_FLOOR {
                return Err(fail(format!(
                    "{id} is numbered 0 or {MADE_IDS_FLOOR} or above: 0 is no unit and those numbers are kept for units made in play (decision 0066)"
                )));
            }
            if truth.land_units.contains_key(&id) {
                return Err(fail(format!("{id} appears twice")));
            }
            if spec.men == 0 {
                return Err(fail(format!("{id} has no men")));
            }
            let side: Side = spec.side.into();
            let mut unit = LandUnit::new(id, spec.name.clone(), side, 0, None);
            if let Some(posture) = spec.posture {
                unit.posture = posture.into();
            }
            if spec.working.unwrap_or(false) {
                if unit.posture == Posture::Attack {
                    return Err(fail(format!(
                        "{id} is a working unit and has posture attack: a working unit does not attack, it is always on defend"
                    )));
                }
                unit.working = true;
                unit.activity = Activity::Work(None);
            }
            if let Some(fatigue) = &spec.fatigue {
                let fatigue = fatigue
                    .to_fixed()
                    .map_err(|e| fail(format!("{id} fatigue {e}")))?;
                if fatigue.is_negative() || fatigue > Fixed::from_int(100) {
                    return Err(fail(format!(
                        "{id} has fatigue {fatigue}, which is not between 0 and 100"
                    )));
                }
                unit.fatigue = fatigue;
            }
            if let Some(fieldworks) = &spec.fieldworks {
                let fieldworks = fieldworks
                    .to_fixed()
                    .map_err(|e| fail(format!("{id} fieldworks {e}")))?;
                let limit = ref_data
                    .ground
                    .as_ref()
                    .map_or(Fixed::ZERO, |g| g.fieldworks_limit);
                if fieldworks.is_negative() || fieldworks > limit {
                    return Err(fail(format!(
                        "{id} has fieldworks {fieldworks}, which is not between 0 and the fieldworks_limit {limit}"
                    )));
                }
                if fieldworks > Fixed::ZERO
                    && (unit.working
                        || unit.posture == Posture::Attack
                        || spec.task_force.is_some())
                {
                    return Err(fail(format!(
                        "{id} has fieldworks but is a working unit, on attack or has no men ashore: only a fighting unit on defend with men ashore has fieldworks"
                    )));
                }
                unit.fieldworks = fieldworks;
            }
            if let Some(experience) = spec.experience {
                if experience > u32::from(LandUnit::MAX_EXPERIENCE) {
                    return Err(fail(format!(
                        "{id} has experience {experience}, which is not a whole number from 0 to {}",
                        LandUnit::MAX_EXPERIENCE
                    )));
                }
                unit.experience = experience as u8;
            }
            if let Some(days) = spec.training_days {
                let per_level = ref_data
                    .ground
                    .as_ref()
                    .map_or(u32::MAX, |g| g.training_days_per_level);
                if days >= per_level {
                    return Err(fail(format!(
                        "{id} has {days} days of training counted, which is not below the {per_level} days a step takes"
                    )));
                }
                unit.training_days = days;
            }
            if let Some(days) = spec.fight_days {
                let per_level = ref_data
                    .ground
                    .as_ref()
                    .map_or(u32::MAX, |g| g.fight_days_per_level);
                if days >= per_level {
                    return Err(fail(format!(
                        "{id} has {days} days of fighting counted, which is not below the {per_level} days a step takes"
                    )));
                }
                unit.fight_days = days;
            }
            if let Some(skills) = &spec.skills {
                let standard = Skills::STANDARD_ALL;
                let mut set = Skills::STANDARD_ALL;
                for (name, given, slot, default) in [
                    (
                        "assault",
                        skills.assault,
                        &mut set.assault,
                        standard.assault,
                    ),
                    (
                        "holding",
                        skills.holding,
                        &mut set.holding,
                        standard.holding,
                    ),
                    ("weight", skills.weight, &mut set.weight, standard.weight),
                    ("clear", skills.clear, &mut set.clear, standard.clear),
                    ("jungle", skills.jungle, &mut set.jungle, standard.jungle),
                    (
                        "mountain",
                        skills.mountain,
                        &mut set.mountain,
                        standard.mountain,
                    ),
                    ("atoll", skills.atoll, &mut set.atoll, standard.atoll),
                    ("works", skills.works, &mut set.works, standard.works),
                ] {
                    let value = given.unwrap_or(u32::from(default));
                    if value > u32::from(Skills::MAX) {
                        return Err(fail(format!(
                            "{id} has a {name} skill of {value}, which is not a whole number from 0 to {}",
                            Skills::MAX
                        )));
                    }
                    *slot = value as u8;
                }
                unit.skills = set;
            }
            match (spec.q, spec.r, spec.task_force) {
                (Some(q), Some(r), None) => {
                    let hex = Hex::new(q, r);
                    let data = map
                        .get(hex)
                        .ok_or_else(|| fail(format!("{id} is ashore at {q},{r}, off the map")))?;
                    if !matches!(data.kind, HexKind::Coast | HexKind::Land) {
                        return Err(fail(format!(
                            "{id} is ashore at {q},{r}, which is not coast or land"
                        )));
                    }
                    unit.men_ashore = spec.men;
                    unit.hex = Some(hex);
                }
                (None, None, Some(raw)) => {
                    let force_id = TaskForceId::new(raw);
                    let force = truth
                        .task_forces
                        .get(&force_id)
                        .ok_or_else(|| fail(format!("{id} is aboard unknown task force {raw}")))?;
                    if force.side != side {
                        return Err(fail(format!(
                            "{id} is {} but its task force {force_id} is {}",
                            side.name(),
                            force.side.name()
                        )));
                    }
                    to_put_aboard.push((id, force_id, spec.men));
                }
                _ => {
                    return Err(fail(format!(
                        "{id} needs exactly one of a hex (both q and r) and a task_force"
                    )))
                }
            }
            truth.land_units.insert(id, unit);
        }
        for (id, force_id, men) in to_put_aboard {
            truth
                .put_aboard_at_start(id, force_id, men, |class| {
                    ref_data
                        .ship_classes
                        .get(&class)
                        .map_or(0, |class| class.troop_capacity_men)
                })
                .map_err(&fail)?;
        }

        // Air groups come after ships, since a group may be aboard one.
        let mut aboard: BTreeMap<ShipId, u32> = BTreeMap::new();
        for spec in &file.air_group {
            let id = AirGroupId::new(spec.id);
            if spec.id >= MADE_IDS_FLOOR {
                return Err(fail(format!(
                    "{id} is numbered {MADE_IDS_FLOOR} or above: those numbers are kept for groups made in play (decision 0086)"
                )));
            }
            let side: Side = spec.side.into();
            let location = match (spec.base, spec.ship) {
                (Some(raw), None) => {
                    let base_id = BaseId::new(raw);
                    let base = truth
                        .bases
                        .get(&base_id)
                        .ok_or_else(|| fail(format!("{id} is at unknown base {raw}")))?;
                    // A group with no aircraft and no pilots may wait at a base that
                    // is not its side's or has no airfield (decision 0101).
                    let empty = spec.ready == 0
                        && spec.damaged.unwrap_or(0) == 0
                        && spec.pilots.unwrap_or(0) == 0;
                    if base.side != side && !empty {
                        return Err(fail(format!(
                            "{id} is {} but its base {base_id} is {}",
                            side.name(),
                            base.side.name()
                        )));
                    }
                    if !base.airfield && !empty {
                        return Err(fail(format!(
                            "{id} is at {base_id}, which has no airfield (decision 6 of 0099)"
                        )));
                    }
                    AirGroupLocation::Base(base_id)
                }
                (None, Some(raw)) => {
                    let ship_id = ShipId::new(raw);
                    let ship = truth
                        .ships
                        .get(&ship_id)
                        .ok_or_else(|| fail(format!("{id} is aboard unknown ship {raw}")))?;
                    if ship.side != side {
                        return Err(fail(format!(
                            "{id} is {} but its ship {ship_id} is {}",
                            side.name(),
                            ship.side.name()
                        )));
                    }
                    let capacity = ref_data.ship_classes[&ship.class].aircraft_capacity;
                    let total = aboard.entry(ship_id).or_insert(0);
                    *total = total
                        .saturating_add(spec.ready)
                        .saturating_add(spec.damaged.unwrap_or(0));
                    if *total > capacity {
                        return Err(fail(format!(
                            "{id} puts {} aircraft aboard {ship_id}, whose class carries {capacity}",
                            *total
                        )));
                    }
                    AirGroupLocation::Ship(ship_id)
                }
                _ => return Err(fail(format!("{id} needs exactly one of base and ship"))),
            };
            let aircraft_type = AircraftTypeId::new(spec.aircraft_type);
            if !ref_data.aircraft_types.contains_key(&aircraft_type) {
                return Err(fail(format!(
                    "{id} has unknown aircraft type {}",
                    spec.aircraft_type
                )));
            }
            let group = AirGroup {
                id,
                name: spec.name.clone(),
                side,
                location,
                aircraft_type,
                ready: spec.ready,
                damaged: spec.damaged.unwrap_or(0),
                mission: AirMission::Rest,
                search: None,
                mission_order: None,
                arc_order: None,
                patrol_share: Fixed::ONE,
                share_order: None,
                fatigue: match &spec.fatigue {
                    Some(quantity) => {
                        let fatigue = quantity
                            .to_fixed()
                            .map_err(|e| fail(format!("{id} fatigue {e}")))?;
                        if fatigue < Fixed::ZERO || fatigue > Fixed::from_int(100) {
                            return Err(fail(format!(
                                "{id} has a fatigue of {fatigue}, not between 0 and 100"
                            )));
                        }
                        fatigue
                    }
                    None => Fixed::ZERO,
                },
                rest_above: None,
                rest_order: None,
                stood_down: false,
                pilots: spec.pilots,
                strength: spec
                    .strength
                    .unwrap_or_else(|| spec.ready.saturating_add(spec.damaged.unwrap_or(0))),
                reach: Reach::Normal,
                reach_order: None,
                target_priority: ShipKind::ALL.to_vec(),
                target_order: None,
                last_base_strike_day: None,
                rebased_day: None,
            };
            if truth.air_groups.insert(id, group).is_some() {
                return Err(fail(format!("{id} appears twice")));
            }
        }

        // Replacements come after bases and air groups' types are known.
        for (index, spec) in file.replacement.iter().enumerate() {
            let side: Side = spec.side.into();
            let base_id = BaseId::new(spec.base);
            let base = truth.bases.get(&base_id).ok_or_else(|| {
                fail(format!(
                    "replacement {} is at unknown base {}",
                    index + 1,
                    spec.base
                ))
            })?;
            if base.side != side {
                return Err(fail(format!(
                    "replacement {} is {} but its base {base_id} is {}",
                    index + 1,
                    side.name(),
                    base.side.name()
                )));
            }
            let aircraft_type = AircraftTypeId::new(spec.aircraft_type);
            if !ref_data.aircraft_types.contains_key(&aircraft_type) {
                return Err(fail(format!(
                    "replacement {} has unknown aircraft type {}",
                    index + 1,
                    spec.aircraft_type
                )));
            }
            truth.replacements.push(Replacement {
                side,
                base: base_id,
                aircraft_type,
                aircraft_per_day: spec.aircraft_per_day,
                pilots_per_day: spec.pilots_per_day,
            });
        }

        // Arrivals come after the air groups are known (decision 0041).
        for (index, spec) in file.arrival.iter().enumerate() {
            let number = index + 1;
            let group_id = AirGroupId::new(spec.air_group);
            let group = truth.air_groups.get(&group_id).ok_or_else(|| {
                fail(format!(
                    "arrival {number} is for unknown air group {}",
                    spec.air_group
                ))
            })?;
            if spec.day == 0 || spec.day > file.days {
                return Err(fail(format!(
                    "arrival {number} is on day {}, outside days 1 to {}",
                    spec.day, file.days
                )));
            }
            let pilots = spec.pilots.unwrap_or(0);
            if spec.aircraft == 0 && pilots == 0 {
                return Err(fail(format!("arrival {number} brings nothing")));
            }
            if pilots > 0 && group.pilots.is_none() {
                return Err(fail(format!(
                    "arrival {number} brings pilots to {group_id}, whose pilots are not tracked"
                )));
            }
            // An arrival to a group aboard a ship must not overfill the ship:
            // the aircraft the ship's groups start with plus every arrival to
            // them by this day. Losses in play are ignored, so the check is
            // conservative (it mirrors `ShipOverAircraftCapacity`).
            if let AirGroupLocation::Ship(ship_id) = group.location {
                let mut total = aboard.get(&ship_id).copied().unwrap_or(0);
                for other in &file.arrival {
                    let other_on_ship = truth
                        .air_groups
                        .get(&AirGroupId::new(other.air_group))
                        .is_some_and(|g| g.location == AirGroupLocation::Ship(ship_id));
                    if other_on_ship && other.day <= spec.day {
                        total = total.saturating_add(other.aircraft);
                    }
                }
                let capacity =
                    ref_data.ship_classes[&truth.ships[&ship_id].class].aircraft_capacity;
                if total > capacity {
                    return Err(fail(format!(
                        "arrival {number} would put {total} aircraft aboard {ship_id}, whose class carries {capacity}"
                    )));
                }
            }
            truth.arrivals.push(Arrival {
                day: spec.day,
                air_group: group_id,
                aircraft: spec.aircraft,
                pilots,
                waiting: false,
            });
        }

        let mut script: BTreeMap<u32, Vec<(Side, OrderRequest)>> = BTreeMap::new();
        for spec in &file.order {
            if spec.day == 0 || spec.day > file.days {
                return Err(fail(format!(
                    "an order is set for day {}, outside days 1 to {}",
                    spec.day, file.days
                )));
            }
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
            let kind = match (&spec.task_force, &spec.air_group, &spec.land_unit) {
                (Some(raw), None, None) => {
                    let order = match &spec.action {
                        ActionSpec::SetRoute { waypoints, act_at } => TaskForceOrder::SetRoute {
                            waypoints: waypoints.iter().map(|[q, r]| Hex::new(*q, *r)).collect(),
                            act_at: act_at.iter().map(|[q, r]| Hex::new(*q, *r)).collect(),
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
                        ActionSpec::Load { cargo, unit } => match (unit, cargo) {
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
                        ActionSpec::SetHomePort { base } => {
                            TaskForceOrder::SetHomePort(BaseId::new(*base))
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
                        ActionSpec::Rebase { .. } => {
                            return Err(wrong_target("rebase", "air_group"))
                        }
                        ActionSpec::SetPosture { .. } => {
                            return Err(wrong_target("set_posture", "land_unit"))
                        }
                        ActionSpec::March { .. } => return Err(wrong_target("march", "land_unit")),
                        ActionSpec::SetActivity { .. } => {
                            return Err(wrong_target("set_activity", "land_unit"))
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
                (None, Some(raw), None) => {
                    let order = match &spec.action {
                        ActionSpec::SetAirMission {
                            mission,
                            target_base,
                        } => {
                            let mission = match (mission, target_base) {
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
                        ActionSpec::Rebase { base } => AirGroupOrder::Rebase {
                            base: BaseId::new(*base),
                        },
                        _ => {
                            return Err(fail(format!(
                                "an order on day {day} for an air_group must be set_air_mission, set_search_arc, set_patrol_share, set_rest_threshold, set_reach, set_target_priority or rebase"
                            )))
                        }
                    };
                    OrderKind::AirGroup {
                        target: AirGroupId::new(*raw),
                        order,
                    }
                }
                (None, None, Some(raw)) => {
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
                _ => {
                    return Err(fail(format!(
                        "the order on day {day} needs exactly one of task_force, air_group and land_unit"
                    )))
                }
            };
            let request = OrderRequest { kind, origin };
            script
                .entry(spec.day)
                .or_default()
                .push((spec.side.into(), request));
        }

        // Replacements and arrivals are lists in the state and the hash, so
        // they are kept in one order whatever order the file lists them in
        // (decision 0043, 4c). Orders are put in order where they are taken
        // in, in `sim-core`.
        truth.replacements.sort_by_key(|r| {
            (
                r.side.code(),
                r.base,
                r.aircraft_type,
                r.aircraft_per_day,
                r.pilots_per_day,
            )
        });
        truth
            .arrivals
            .sort_by_key(|a| (a.day, a.air_group, a.aircraft, a.pilots));

        let mut world =
            World::new(file.seed, ref_data, map, truth).map_err(|e| fail(e.to_string()))?;
        if let Some(spec) = &file.objectives {
            let objectives =
                build_objectives(spec, file.days, world.ref_data().surface_combat.is_some())
                    .map_err(&fail)?;
            world
                .set_objectives(objectives)
                .map_err(|e| fail(e.to_string()))?;
        }

        let mut contacts = Vec::new();
        for (index, spec) in file.contact.iter().enumerate() {
            let kind = parse_kind(&spec.kind).ok_or_else(|| {
                fail(format!(
                    "contact {} has unknown kind \"{}\"",
                    index + 1,
                    spec.kind
                ))
            })?;
            contacts.push(StartingContact {
                side: spec.side.into(),
                hex: Hex::new(spec.q, spec.r),
                ship_count: spec.ship_count,
                kind,
                age_pulses: spec.age_pulses.unwrap_or(0),
            });
        }
        world
            .hold_starting_contacts(contacts.clone())
            .map_err(|e| fail(e.to_string()))?;
        let estimates: Vec<StartingEstimate> = file
            .estimate
            .iter()
            .map(|spec| StartingEstimate {
                side: spec.side.into(),
                hex: Hex::new(spec.q, spec.r),
                men: spec.men,
            })
            .collect();
        world
            .hold_starting_estimates(estimates.clone())
            .map_err(|e| fail(e.to_string()))?;

        Ok(Scenario {
            name: file.name,
            placeholder: file.placeholder,
            days: file.days,
            world,
            script,
            contacts,
            estimates,
            data_files,
        })
    }

    /// The files the scenario names (map, ship classes, port sizes, aircraft
    /// types, then the figures files it has), as the paths they are read from.
    pub fn data_files(&self) -> &[PathBuf] {
        &self.data_files
    }

    /// The orders still to be given, by day, as they will be submitted.
    pub fn scripted_orders(&self) -> &BTreeMap<u32, Vec<(Side, OrderRequest)>> {
        &self.script
    }

    /// Replaces the scripted orders, for replaying a saved game: the orders
    /// given are submitted day by day in place of the scenario's own.
    pub fn replace_script(&mut self, script: BTreeMap<u32, Vec<(Side, OrderRequest)>>) {
        self.script = script;
    }

    /// A copy of this scenario with a different seed, for a run of its own.
    /// The world is built afresh, so the scenario must not have started: it
    /// is an error once any day has been resolved. The script, name and
    /// length are kept. Used by the 1,000-seed test and by `--seed`.
    pub fn with_seed(&self, seed: u64) -> Result<Scenario, DataError> {
        if self.world.day() != 0 {
            return Err(DataError {
                path: None,
                message: format!(
                    "the seed can only change before day 1, and day {} has been resolved",
                    self.world.day()
                ),
            });
        }
        let mut world = World::new(
            seed,
            self.world.ref_data().clone(),
            self.world.map().clone(),
            self.world.umpire().clone(),
        )?;
        world.hold_starting_contacts(self.contacts.clone())?;
        world.hold_starting_estimates(self.estimates.clone())?;
        if let Some(objectives) = self.world.objectives() {
            world.set_objectives(objectives.clone())?;
        }
        Ok(Scenario {
            name: self.name.clone(),
            placeholder: self.placeholder,
            days: self.days,
            world,
            script: self.script.clone(),
            contacts: self.contacts.clone(),
            estimates: self.estimates.clone(),
            data_files: self.data_files.clone(),
        })
    }

    /// Submits the scripted orders for the next day and resolves it. Returns
    /// `None` once the scenario has run all its days.
    pub fn next_day(&mut self) -> Result<Option<DaySummary>, DataError> {
        let day = self.world.day() + 1;
        if day > self.days {
            return Ok(None);
        }
        for (side, request) in self.script.remove(&day).unwrap_or_default() {
            self.world.submit_orders(side, vec![request]);
        }
        Ok(Some(self.world.resolve_day()?))
    }

    /// Runs every remaining day and returns their summaries.
    pub fn run_to_end(&mut self) -> Result<Vec<DaySummary>, DataError> {
        let mut summaries = Vec::new();
        while let Some(summary) = self.next_day()? {
            summaries.push(summary);
        }
        Ok(summaries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sim_core::state::RulesOfEngagement;

    #[derive(Deserialize)]
    struct Holder {
        value: Quantity,
    }

    fn read(text: &str) -> Result<Fixed, String> {
        let holder: Holder = toml::from_str(text).map_err(|e| e.to_string())?;
        holder.value.to_fixed()
    }

    #[test]
    fn quantities_are_whole_numbers_or_decimal_strings() {
        assert_eq!(read("value = 2000"), Ok(Fixed::from_int(2000)));
        assert_eq!(read("value = \"0.25\""), Ok(Fixed::from_ratio(1, 4)));
    }

    #[test]
    fn a_toml_float_is_rejected() {
        assert!(read("value = 0.25").is_err());
        assert!(read("value = \"a quarter\"").is_err());
    }

    const HEAD: &str = r#"
name = "Test"
placeholder = true
seed = 5
days = 2
map = "../map/placeholder_search.toml"
ship_classes = "../reference/ship_classes.csv"
port_sizes = "../reference/port_sizes.csv"
aircraft_types = "../reference/aircraft_types.csv"
search = "../reference/search.toml"
surface_combat = "../reference/surface_combat.toml"

[[base]]
id = 1
name = "Allied base"
side = "allied"
q = 0
r = 0

[[base]]
id = 2
name = "Japanese base"
side = "japanese"
q = 5
r = 0
"#;

    /// Parses `HEAD` plus `extra`, as if it sat in the scenarios folder.
    fn parse_with(extra: &str) -> Result<Scenario, DataError> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        Scenario::parse(&format!("{HEAD}{extra}"), &path)
    }

    /// Parses `HEAD` with its two reference lines replaced, as if it sat in
    /// the scenarios folder (decision 0043: a file or a list of files).
    fn parse_files(classes: &str, types: &str) -> Result<Scenario, DataError> {
        let head = HEAD
            .replace(
                "ship_classes = \"../reference/ship_classes.csv\"",
                &format!("ship_classes = {classes}"),
            )
            .replace(
                "aircraft_types = \"../reference/aircraft_types.csv\"",
                &format!("aircraft_types = {types}"),
            );
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        Scenario::parse(&head, &path)
    }

    fn temp_copy(from: &str, name: &str) -> String {
        let dir = std::env::temp_dir().join(format!("flagplot-sim-data-{}", std::process::id()));
        fs::create_dir_all(&dir).expect("make the temp dir");
        let source = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../data/scenarios")
            .join(from);
        let target = dir.join(name);
        fs::copy(source, &target).expect("copy the file");
        target.to_string_lossy().into_owned()
    }

    #[test]
    fn ship_classes_and_aircraft_types_may_be_lists_of_files() {
        let scenario = parse_files(
            "[\"../reference/ship_classes.csv\", \"../reference/ship_classes_eastern_solomons.csv\"]",
            "[\"../reference/aircraft_types.csv\", \"../reference/aircraft_types_rabaul_guadalcanal.csv\"]",
        )
        .expect("two files each load");
        let data = scenario.world.ref_data();
        // Placeholder class 1 and a real class from the second file.
        assert!(data.ship_classes.contains_key(&ShipClassId::new(1)));
        assert!(data.ship_classes.contains_key(&ShipClassId::new(2020)));
        assert!(data.aircraft_types.contains_key(&AircraftTypeId::new(1)));
        assert!(data.aircraft_types.contains_key(&AircraftTypeId::new(2100)));
        // The files it names are listed in a fixed order.
        let names: Vec<String> = scenario
            .data_files()
            .iter()
            .map(|p| {
                p.file_name()
                    .expect("a name")
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();
        assert_eq!(
            names,
            [
                "placeholder_search.toml",
                "ship_classes.csv",
                "ship_classes_eastern_solomons.csv",
                "port_sizes.csv",
                "aircraft_types.csv",
                "aircraft_types_rabaul_guadalcanal.csv",
                "search.toml",
                "surface_combat.toml",
            ]
        );
    }

    #[test]
    fn a_list_of_one_file_is_the_same_as_the_file() {
        let one = parse_files(
            "\"../reference/ship_classes.csv\"",
            "\"../reference/aircraft_types.csv\"",
        )
        .expect("loads");
        let listed = parse_files(
            "[\"../reference/ship_classes.csv\"]",
            "[\"../reference/aircraft_types.csv\"]",
        )
        .expect("loads");
        assert_eq!(one.world.state_hash(), listed.world.state_hash());
        assert_eq!(one.data_files(), listed.data_files());
    }

    #[test]
    fn an_id_in_two_files_is_an_error_naming_both() {
        let copy = temp_copy("../reference/ship_classes.csv", "classes_again.csv");
        let error = parse_files(
            &format!("[\"../reference/ship_classes.csv\", {copy:?}]"),
            "\"../reference/aircraft_types.csv\"",
        )
        .expect_err("class 1 is in both");
        let text = error.to_string();
        assert!(text.contains("ship class"), "{text}");
        assert!(text.contains("ship_classes.csv"), "{text}");
        assert!(text.contains("classes_again.csv"), "{text}");

        let copy = temp_copy("../reference/aircraft_types.csv", "types_again.csv");
        let error = parse_files(
            "\"../reference/ship_classes.csv\"",
            &format!("[\"../reference/aircraft_types.csv\", {copy:?}]"),
        )
        .expect_err("type 1 is in both");
        let text = error.to_string();
        assert!(text.contains("aircraft type"), "{text}");
        assert!(text.contains("aircraft_types.csv"), "{text}");
        assert!(text.contains("types_again.csv"), "{text}");
    }

    #[test]
    fn an_empty_list_of_files_is_an_error() {
        let error = parse_files("[]", "\"../reference/aircraft_types.csv\"")
            .expect_err("no ship classes")
            .to_string();
        assert!(error.contains("ship_classes names no file"), "{error}");
        let error = parse_files("\"../reference/ship_classes.csv\"", "[]")
            .expect_err("no aircraft types")
            .to_string();
        assert!(error.contains("aircraft_types names no file"), "{error}");
    }

    fn error_of(extra: &str) -> String {
        parse_with(extra)
            .expect_err("the scenario is rejected")
            .message
    }

    fn task_force(placement: &str) -> String {
        format!(
            "[[task_force]]\nid = 1\nname = \"TF\"\nside = \"allied\"\n{placement}\nmission = \"escort\"\n"
        )
    }

    fn air_group(base: u32, aircraft_type: u32) -> String {
        format!(
            "[[air_group]]\nid = 1\nname = \"AG\"\nside = \"allied\"\nbase = {base}\naircraft_type = {aircraft_type}\nready = 3\n"
        )
    }

    #[test]
    fn task_force_placement_is_one_of_base_or_hex() {
        let at_base = parse_with(&task_force("at_base = 2")).expect("at a base loads");
        let hex = at_base.world.umpire().task_forces[&TaskForceId::new(1)].position();
        assert_eq!(hex, Hex::new(5, 0));
        let at_sea = parse_with(&task_force("q = 3\nr = -1")).expect("at sea loads");
        let hex = at_sea.world.umpire().task_forces[&TaskForceId::new(1)].position();
        assert_eq!(hex, Hex::new(3, -1));
        for bad in [
            "at_base = 1\nq = 3\nr = 0",
            "at_base = 1\nq = 3",
            "at_base = 1\nr = 3",
            "q = 3",
            "r = 3",
            "",
        ] {
            let message = error_of(&task_force(bad));
            assert!(message.contains("task_force:1"), "{bad}: {message}");
            assert!(
                message.contains("exactly one of at_base"),
                "{bad}: {message}"
            );
        }
    }

    #[test]
    fn a_unit_numbered_in_the_range_kept_for_units_made_in_play_is_refused() {
        // Decision 0086: 1,000,000 and above belong to the sides' series.
        let high_force = task_force("at_base = 2").replace("id = 1", "id = 1000000");
        let error = error_of(&high_force);
        assert!(error.contains("decision 0086"), "{error}");
        assert!(error.contains("task_force:1000000"), "{error}");
        let high_group = air_group(1, 1).replace("id = 1", "id = 2000001");
        let error = error_of(&high_group);
        assert!(error.contains("decision 0086"), "{error}");
        assert!(error.contains("air_group:2000001"), "{error}");
        // The number just below the floor loads.
        parse_with(&task_force("at_base = 2").replace("id = 1", "id = 999999"))
            .expect("999999 loads");
        parse_with(&air_group(1, 1).replace("id = 1", "id = 999999")).expect("999999 loads");
    }

    #[test]
    fn air_groups_are_checked() {
        let scenario = parse_with(&air_group(1, 1)).expect("a good air group loads");
        let group = &scenario.world.umpire().air_groups[&AirGroupId::new(1)];
        assert_eq!(group.ready, 3);
        assert_eq!(group.mission, AirMission::Rest);
        assert_eq!(group.search, None);
        assert!(error_of(&air_group(9, 1)).contains("unknown base"));
        assert!(error_of(&air_group(2, 1)).contains("its base"));
        assert!(error_of(&air_group(1, 9)).contains("unknown aircraft type"));
        let twice = format!("{}{}", air_group(1, 1), air_group(1, 1));
        assert!(error_of(&twice).contains("appears twice"));
    }

    #[test]
    fn pilots_and_strength_load_and_a_group_without_them_is_untracked() {
        let plain = parse_with(&air_group(1, 1)).expect("loads");
        let group = &plain.world.umpire().air_groups[&AirGroupId::new(1)];
        assert_eq!((group.pilots, group.strength), (None, 3));
        let with = |extra: &str| parse_with(&format!("{}{extra}", air_group(1, 1))).expect("loads");
        let tracked = with("pilots = 2\n");
        let group = &tracked.world.umpire().air_groups[&AirGroupId::new(1)];
        assert_eq!((group.pilots, group.strength), (Some(2), 3));
        let given = with("pilots = 5\nstrength = 9\n");
        let group = &given.world.umpire().air_groups[&AirGroupId::new(1)];
        assert_eq!((group.pilots, group.strength), (Some(5), 9));
        // The default strength counts damaged aircraft too.
        let damaged = with("damaged = 4\n");
        let group = &damaged.world.umpire().air_groups[&AirGroupId::new(1)];
        assert_eq!(group.strength, 7);
        assert!(parse_with(&format!("{}pilots = 1.5\n", air_group(1, 1))).is_err());
    }

    fn replacement(side: &str, base: u32, aircraft_type: u32) -> String {
        format!(
            "[[replacement]]\nside = \"{side}\"\nbase = {base}\naircraft_type = {aircraft_type}\naircraft_per_day = 2\npilots_per_day = 1\n"
        )
    }

    #[test]
    fn replacement_streams_are_kept_in_one_order_whatever_the_file_gives() {
        let a = format!(
            "{}{}{}",
            replacement("japanese", 2, 2),
            replacement("allied", 1, 2),
            replacement("allied", 1, 1)
        );
        let b = format!(
            "{}{}{}",
            replacement("allied", 1, 1),
            replacement("japanese", 2, 2),
            replacement("allied", 1, 2)
        );
        let (a, b) = (
            parse_with(&a).expect("loads"),
            parse_with(&b).expect("loads"),
        );
        let key = |s: &Scenario| -> Vec<(u8, u32, u32)> {
            s.world
                .umpire()
                .replacements
                .iter()
                .map(|r| (r.side.code(), r.base.raw(), r.aircraft_type.raw()))
                .collect()
        };
        // Side, then base, then type (decision 0043, 4c).
        assert_eq!(key(&a), vec![(1, 1, 1), (1, 1, 2), (2, 2, 2)]);
        assert_eq!(key(&a), key(&b));
        assert_eq!(a.world.state_hash(), b.world.state_hash());
    }

    #[test]
    fn replacement_tables_load_in_order_and_are_checked() {
        let none = parse_with("").expect("loads");
        assert!(none.world.umpire().replacements.is_empty());
        let both = format!(
            "{}{}",
            replacement("allied", 1, 1),
            replacement("japanese", 2, 2)
        );
        let scenario = parse_with(&both).expect("loads");
        let streams = &scenario.world.umpire().replacements;
        assert_eq!(streams.len(), 2);
        assert_eq!(
            (
                streams[0].side,
                streams[0].base.raw(),
                streams[0].aircraft_type.raw(),
                streams[0].aircraft_per_day,
                streams[0].pilots_per_day
            ),
            (Side::Allied, 1, 1, 2, 1)
        );
        assert_eq!(
            (streams[1].side, streams[1].base.raw()),
            (Side::Japanese, 2)
        );
        assert!(error_of(&replacement("allied", 9, 1)).contains("unknown base"));
        assert!(error_of(&replacement("allied", 2, 1)).contains("its base"));
        assert!(error_of(&replacement("allied", 1, 9)).contains("unknown aircraft type"));
        assert!(parse_with(&replacement("allied", 1, 1).replace("= 2\n", "= 2.5\n")).is_err());
        assert!(parse_with(&format!("{}extra = 1\n", replacement("allied", 1, 1))).is_err());
    }

    fn arrival(day: u32, group: u32, aircraft: u32, pilots: &str) -> String {
        format!("[[arrival]]\nday = {day}\nair_group = {group}\naircraft = {aircraft}\n{pilots}")
    }

    #[test]
    fn arrival_tables_load_in_one_order_whatever_the_file_gives_and_are_checked() {
        let none = parse_with("").expect("loads");
        assert!(none.world.umpire().arrivals.is_empty());
        let tracked = air_group(1, 1).replace("ready = 3", "ready = 0\npilots = 0");
        let untracked = air_group(1, 1).replace("ready = 3", "ready = 0");
        let text = format!(
            "{tracked}{}{}",
            arrival(2, 1, 6, "pilots = 5\n"),
            arrival(1, 1, 2, "")
        );
        let scenario = parse_with(&text).expect("loads");
        let truth = scenario.world.umpire();
        let group = &truth.air_groups[&AirGroupId::new(1)];
        assert_eq!((group.ready, group.pilots, group.strength), (0, Some(0), 0));
        let listed: Vec<_> = truth
            .arrivals
            .iter()
            .map(|a| (a.day, a.air_group.raw(), a.aircraft, a.pilots))
            .collect();
        // Listed as day 2 then day 1; kept by day (decision 0043, 4c).
        assert_eq!(listed, vec![(1, 1, 2, 0), (2, 1, 6, 5)]);
        // A group with no aircraft and no pilots key is loaded, untracked.
        let loaded = parse_with(&untracked).expect("loads");
        let group = &loaded.world.umpire().air_groups[&AirGroupId::new(1)];
        assert_eq!((group.ready, group.pilots), (0, None));
        // Aircraft alone, to an untracked group.
        assert!(parse_with(&format!("{untracked}{}", arrival(1, 1, 4, ""))).is_ok());
        let with = |arrival: String| error_of(&format!("{tracked}{arrival}"));
        assert!(with(arrival(1, 9, 1, "")).contains("unknown air group"));
        assert!(with(arrival(0, 1, 1, "")).contains("outside days"));
        assert!(with(arrival(3, 1, 1, "")).contains("outside days"));
        assert!(with(arrival(1, 1, 0, "")).contains("brings nothing"));
        assert!(with(arrival(1, 1, 0, "pilots = 0\n")).contains("brings nothing"));
        assert!(
            error_of(&format!("{untracked}{}", arrival(1, 1, 2, "pilots = 1\n")))
                .contains("not tracked")
        );
        assert!(parse_with(&format!("{tracked}{}extra = 1\n", arrival(1, 1, 1, ""))).is_err());
        assert!(parse_with(
            &format!("{tracked}{}", arrival(1, 1, 1, ""))
                .replace("= 1\naircraft", "= 1.5\naircraft")
        )
        .is_err());
    }

    #[test]
    fn an_arrival_that_would_overfill_a_ship_is_refused() {
        // Class 4 carries 36: 30 aboard, then 6 on day 1 fits, 7 does not.
        let base = format!(
            "{}{}",
            force_with_ship(4),
            carried_group(1, "ship = 1", 30, "allied")
        );
        parse_with(&format!("{base}{}", arrival(1, 1, 6, ""))).expect("30 and 6 fit in 36");
        let message = error_of(&format!("{base}{}", arrival(1, 1, 7, "")));
        assert!(
            message.contains("arrival 1 would put 37 aircraft aboard ship:1")
                && message.contains("class carries 36"),
            "{message}"
        );
        // Arrivals add up by day, across the ship's groups: the second, on
        // day 2, is the one that overfills it (30 + 3 + 4 = 37).
        let two = format!(
            "{base}{}{}{}",
            carried_group(2, "ship = 1", 0, "allied"),
            arrival(1, 1, 3, ""),
            arrival(2, 2, 4, "")
        );
        let message = error_of(&two);
        assert!(message.contains("arrival 2 would put 37"), "{message}");
        // An arrival to a group at a base is not a ship's concern.
        parse_with(&format!("{}{}", air_group(1, 1), arrival(1, 1, 500, "")))
            .expect("a base takes any number");
    }

    /// An Allied group aboard a ship, `extra` replacing the placement.
    fn carried_group(id: u32, placement: &str, ready: u32, side: &str) -> String {
        format!(
            "[[air_group]]\nid = {id}\nname = \"AG\"\nside = \"{side}\"\n{placement}\naircraft_type = 1\nready = {ready}\n"
        )
    }

    /// Task force 1 holding Allied ship 1 of the given class.
    fn force_with_ship(class: u32) -> String {
        with_force(&ship("").replace("class = 1", &format!("class = {class}")))
    }

    #[test]
    fn an_air_group_may_be_aboard_a_ship_with_the_capacity() {
        // Class 4 is the placeholder carrier, which carries 36.
        let text = format!(
            "{}{}",
            force_with_ship(4),
            carried_group(1, "ship = 1", 36, "allied")
        );
        let scenario = parse_with(&text).expect("a carried group loads");
        let group = &scenario.world.umpire().air_groups[&AirGroupId::new(1)];
        assert_eq!(group.location, AirGroupLocation::Ship(ShipId::new(1)));
        assert_eq!(group.ready, 36);
        // The group is where the ship's task force is.
        assert_eq!(
            scenario.world.umpire().air_group_hex(group),
            Some(Hex::new(0, 0))
        );
        // Two groups may share a ship while they fit.
        let two = format!(
            "{}{}{}",
            force_with_ship(4),
            carried_group(1, "ship = 1", 30, "allied"),
            carried_group(2, "ship = 1", 6, "allied")
        );
        parse_with(&two).expect("30 and 6 fit in 36");
    }

    #[test]
    fn an_air_group_needs_exactly_one_of_base_and_ship() {
        for placement in ["base = 1\nship = 1", ""] {
            let text = format!(
                "{}{}",
                force_with_ship(4),
                carried_group(1, placement, 3, "allied")
            );
            let message = error_of(&text);
            assert!(message.contains("air_group:1"), "{placement}: {message}");
            assert!(
                message.contains("exactly one of base and ship"),
                "{placement}: {message}"
            );
        }
    }

    #[test]
    fn a_carried_air_group_is_checked_against_its_ship() {
        let carried = |class: u32, placement: &str, ready: u32, side: &str| {
            error_of(&format!(
                "{}{}",
                force_with_ship(class),
                carried_group(1, placement, ready, side)
            ))
        };
        // A ship that is not in the scenario.
        let message = carried(4, "ship = 9", 3, "allied");
        assert!(
            message.contains("air_group:1") && message.contains("unknown ship"),
            "{message}"
        );
        // A ship of the other side.
        let message = carried(4, "ship = 1", 3, "japanese");
        assert!(
            message.contains("air_group:1") && message.contains("its ship"),
            "{message}"
        );
        // A ship whose class carries nothing (class 1 is a cruiser).
        let message = carried(1, "ship = 1", 1, "allied");
        assert!(
            message.contains("air_group:1") && message.contains("carries 0"),
            "{message}"
        );
        // One aircraft over the capacity of 36.
        let message = carried(4, "ship = 1", 37, "allied");
        assert!(
            message.contains("air_group:1") && message.contains("carries 36"),
            "{message}"
        );
        // Two groups that together are over it.
        let text = format!(
            "{}{}{}",
            force_with_ship(4),
            carried_group(1, "ship = 1", 30, "allied"),
            carried_group(2, "ship = 1", 7, "allied")
        );
        let message = error_of(&text);
        assert!(
            message.contains("air_group:2") && message.contains("37 aircraft"),
            "{message}"
        );
    }

    fn order(target: &str, body: &str) -> String {
        format!(
            "{}{}[[order]]\nday = 1\nside = \"allied\"\n{target}\n{body}\n",
            air_group(1, 1),
            task_force("at_base = 1")
        )
    }

    #[test]
    fn orders_are_addressed_to_one_kind_of_unit() {
        let arc = "kind = \"set_search_arc\"\nfrom_deg = 60\nto_deg = 120\nrange_nm = 600";
        let mission = "kind = \"set_air_mission\"\nmission = \"search\"";
        parse_with(&order("air_group = 1", arc)).expect("arc loads");
        parse_with(&order("air_group = 1", mission)).expect("mission loads");
        parse_with(&order("task_force = 1", "kind = \"load\"")).expect("load loads");
        assert!(error_of(&order("task_force = 1", arc)).contains("must be addressed to a"));
        assert!(error_of(&order("task_force = 1", mission)).contains("air_group"));
        assert!(error_of(&order("air_group = 1", "kind = \"load\"")).contains("air_group"));
        assert!(
            error_of(&order("task_force = 1\nair_group = 1", "kind = \"load\""))
                .contains("exactly one of task_force, air_group and land_unit")
        );
        assert!(error_of(&order("", "kind = \"load\"")).contains("exactly one"));
        let float = arc.replace("range_nm = 600", "range_nm = 600.5");
        assert!(parse_with(&order("air_group = 1", &float)).is_err());
    }

    #[test]
    fn the_naval_strike_mission_loads_for_an_air_group() {
        let mission = "kind = \"set_air_mission\"\nmission = \"naval_strike\"";
        parse_with(&order("air_group = 1", mission)).expect("naval_strike loads");
        let typo = mission.replace("naval_strike", "naval strike");
        assert!(parse_with(&order("air_group = 1", &typo)).is_err());
    }

    #[test]
    fn the_combat_air_patrol_mission_and_the_patrol_share_load_for_an_air_group() {
        let mission = "kind = \"set_air_mission\"\nmission = \"combat_air_patrol\"";
        parse_with(&order("air_group = 1", mission)).expect("combat_air_patrol loads");
        let typo = mission.replace("combat_air_patrol", "combat air patrol");
        assert!(parse_with(&order("air_group = 1", &typo)).is_err());

        let share = "kind = \"set_patrol_share\"\nshare = \"0.5\"";
        parse_with(&order("air_group = 1", share)).expect("share loads");
        // A share is a decimal string, never a float, and has a field.
        let float = share.replace("\"0.5\"", "0.5");
        assert!(parse_with(&order("air_group = 1", &float)).is_err());
        assert!(parse_with(&order("air_group = 1", "kind = \"set_patrol_share\"")).is_err());
        // It is for an air group, not a task force.
        assert!(error_of(&order("task_force = 1", share)).contains("air_group"));
        // The value is checked when the order reaches intake, not here: a
        // share above 1 loads.
        parse_with(&order(
            "air_group = 1",
            "kind = \"set_patrol_share\"\nshare = \"1.5\"",
        ))
        .expect("a share out of range is rejected by the world, not the loader");
    }

    #[test]
    fn the_base_strike_mission_needs_its_target_base_and_no_other_mission_has_one() {
        let strike = |target: &str| {
            format!("kind = \"set_air_mission\"\nmission = \"base_strike\"\n{target}")
        };
        // A base of the other side loads, and the order is addressed to a
        // group.
        parse_with(&order("air_group = 1", &strike("target_base = 2"))).expect("loads");
        let message = error_of(&order("air_group = 1", &strike("")));
        assert!(message.contains("needs a target_base"), "{message}");
        let message = error_of(&order("air_group = 1", &strike("target_base = 9")));
        assert!(message.contains("unknown base 9"), "{message}");
        let message = error_of(&order("air_group = 1", &strike("target_base = 1")));
        assert!(message.contains("own side"), "{message}");
        // A later day's order on the same base loads: the base may have
        // changed side by then, and intake checks it (decision 0101, as built).
        parse_with(
            &order("air_group = 1", &strike("target_base = 1")).replace("day = 1\n", "day = 2\n"),
        )
        .expect("a later day's strike on a base of the side at the start loads");
        let message = error_of(&order(
            "air_group = 1",
            "kind = \"set_air_mission\"\nmission = \"naval_strike\"\ntarget_base = 2",
        ));
        assert!(message.contains("only for the base_strike"), "{message}");
        // A target is a base number, not a float.
        assert!(parse_with(&order("air_group = 1", &strike("target_base = 2.5"))).is_err());
        // The order is for an air group.
        assert!(
            error_of(&order("task_force = 1", &strike("target_base = 2"))).contains("air_group")
        );
    }

    #[test]
    fn a_base_strike_order_sets_the_mission_with_its_target() {
        let mut scenario = with_air_combat(&format!(
            "{}[[order]]\nday = 1\nside = \"allied\"\nair_group = 1\nkind = \"set_air_mission\"\nmission = \"base_strike\"\ntarget_base = 2\n",
            air_group(1, 3)
        ))
        .expect("loads");
        scenario.next_day().expect("day runs");
        assert_eq!(
            scenario.world.umpire().air_groups[&AirGroupId::new(1)].mission,
            AirMission::BaseStrike(BaseId::new(2))
        );
    }

    #[test]
    fn a_base_takes_an_optional_whole_number_of_anti_aircraft_shots() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        let guns = |extra: &str| {
            let head = HEAD.replace("q = 5\nr = 0\n", &format!("q = 5\nr = 0\n{extra}"));
            Scenario::parse(&head, &path)
        };
        let shots =
            |scenario: Scenario| scenario.world.umpire().bases[&BaseId::new(2)].anti_aircraft;
        assert_eq!(shots(guns("").expect("loads")), 0);
        assert_eq!(shots(guns("anti_aircraft = 12\n").expect("loads")), 12);
        assert!(guns("anti_aircraft = 1.5\n").is_err());
        assert!(guns("anti_aircraft = -1\n").is_err());
    }

    #[test]
    fn a_base_takes_an_optional_fuel_stock_in_tons() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        let fuel = |extra: &str| {
            let head = HEAD.replace("q = 5\nr = 0\n", &format!("q = 5\nr = 0\n{extra}"));
            Scenario::parse(&head, &path)
        };
        let stock = |scenario: Scenario| scenario.world.umpire().bases[&BaseId::new(2)].fuel_t;
        assert_eq!(stock(fuel("").expect("loads")), Fixed::ZERO);
        assert_eq!(
            stock(fuel("fuel_t = 150\n").expect("loads")),
            Fixed::from_int(150)
        );
        assert_eq!(
            stock(fuel("fuel_t = \"0.25\"\n").expect("loads")),
            Fixed::from_ratio(1, 4)
        );
        // A TOML float and a negative stock are both refused.
        assert!(fuel("fuel_t = 1.5\n").is_err());
        assert!(fuel("fuel_t = -1\n").is_err());
        assert!(fuel("fuel_t = \"-0.5\"\n").is_err());
    }

    #[test]
    fn a_base_takes_an_optional_whole_number_of_repairs_a_day() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        let repairs = |extra: &str| {
            let head = HEAD.replace("q = 5\nr = 0\n", &format!("q = 5\nr = 0\n{extra}"));
            Scenario::parse(&head, &path)
        };
        let figure = |scenario: Scenario| {
            scenario.world.umpire().bases[&BaseId::new(2)].aircraft_repairs_per_day
        };
        assert_eq!(figure(repairs("").expect("loads")), 0);
        assert_eq!(
            figure(repairs("aircraft_repairs_per_day = 3\n").expect("loads")),
            3
        );
        assert!(repairs("aircraft_repairs_per_day = 1.5\n").is_err());
        assert!(repairs("aircraft_repairs_per_day = -1\n").is_err());
    }

    #[test]
    fn a_base_takes_repair_points_a_day_and_a_yard_passage_of_at_least_a_day() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        let base = |extra: &str| {
            let head = HEAD.replace("q = 5\nr = 0\n", &format!("q = 5\nr = 0\n{extra}"));
            Scenario::parse(&head, &path)
        };
        let figures = |scenario: Scenario| {
            let base = scenario.world.umpire().bases[&BaseId::new(2)].clone();
            (base.ship_repair_points_per_day, base.yard_passage_days)
        };
        assert_eq!(figures(base("").expect("loads")), (Fixed::ZERO, None));
        assert_eq!(
            figures(
                base("ship_repair_points_per_day = 6\nyard_passage_days = 4\n").expect("loads")
            ),
            (Fixed::from_int(6), Some(4))
        );
        assert_eq!(
            figures(base("ship_repair_points_per_day = \"2.5\"\n").expect("loads")).0,
            Fixed::from_ratio(5, 2)
        );
        assert_eq!(
            figures(base("ship_repair_points_per_day = 0\n").expect("loads")).0,
            Fixed::ZERO
        );
        // Negative points, a float, and a passage of 0, below 0 or not whole.
        assert!(base("ship_repair_points_per_day = -1\n").is_err());
        assert!(base("ship_repair_points_per_day = \"-0.5\"\n").is_err());
        assert!(base("ship_repair_points_per_day = 1.5\n").is_err());
        assert!(base("yard_passage_days = 0\n").is_err());
        assert!(base("yard_passage_days = -3\n").is_err());
        assert!(base("yard_passage_days = 2.5\n").is_err());
        assert!(base("yard_passage_days = \"4\"\n").is_err());
    }

    #[test]
    fn a_ship_may_start_with_flotation_and_fire_damage_within_range() {
        let damage = |extra: &str| {
            parse_with(&with_force(&ship(extra))).map(|scenario| {
                let ship = scenario.world.umpire().ships[&ShipId::new(1)].clone();
                (ship.flotation_damage, ship.fire_damage)
            })
        };
        assert_eq!(damage("").expect("loads"), (Fixed::ZERO, Fixed::ZERO));
        assert_eq!(
            damage("flotation_damage = 12\nfire_damage = \"2.5\"").expect("loads"),
            (Fixed::from_int(12), Fixed::from_ratio(5, 2))
        );
        assert_eq!(
            damage("flotation_damage = \"99.999999\"\nfire_damage = 100").expect("loads"),
            (Fixed::from_raw(99_999_999), Fixed::from_int(100))
        );
        // Flotation: not negative, below 100, no float. Fire: not negative,
        // at most 100, no float.
        assert!(damage("flotation_damage = -1").is_err());
        assert!(damage("flotation_damage = 100").is_err());
        assert!(damage("flotation_damage = 101").is_err());
        assert!(damage("flotation_damage = 12.5").is_err());
        assert!(damage("fire_damage = -1").is_err());
        assert!(damage("fire_damage = 101").is_err());
        assert!(damage("fire_damage = 2.5").is_err());
    }

    #[test]
    fn a_send_to_yard_order_takes_no_other_key() {
        let orders = loaded_orders("kind = \"send_to_yard\"").expect("loads");
        assert_eq!(orders, vec![TaskForceOrder::SendToYard]);
        assert!(loaded_orders("kind = \"send_to_yard\"\nbase = 1").is_err());
        assert!(error_of(&order("air_group = 1", "kind = \"send_to_yard\"")).contains("air_group"));
    }

    #[test]
    fn an_air_group_takes_an_optional_whole_number_of_damaged_aircraft() {
        let damaged = |extra: &str| {
            let text = air_group(1, 3).replace("ready = ", &format!("{extra}ready = "));
            parse_with(&text)
                .map(|scenario| scenario.world.umpire().air_groups[&AirGroupId::new(1)].damaged)
        };
        assert_eq!(damaged("").expect("loads"), 0);
        assert_eq!(damaged("damaged = 4\n").expect("loads"), 4);
        assert!(damaged("damaged = 1.5\n").is_err());
        assert!(damaged("damaged = -1\n").is_err());
    }

    /// `HEAD` with the air upkeep key added after the surface combat key.
    fn with_air_upkeep(key: &str) -> Result<Scenario, DataError> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        let head = HEAD.replace(
            "surface_combat = \"../reference/surface_combat.toml\"\n",
            &format!("surface_combat = \"../reference/surface_combat.toml\"\n{key}"),
        );
        Scenario::parse(&head, &path)
    }

    #[test]
    fn the_air_upkeep_key_is_optional_and_loads_the_figures() {
        assert!(parse_with("")
            .expect("loads")
            .world
            .ref_data()
            .air_upkeep
            .is_none());
        let with =
            with_air_upkeep("air_upkeep = \"../reference/air_upkeep.toml\"\n").expect("loads");
        let figures = with
            .world
            .ref_data()
            .air_upkeep
            .expect("the figures are loaded");
        assert_eq!(figures.wear_base_chance, Fixed::from_ratio(5, 100));
        assert_eq!(figures.carrier_repairs_per_day, 2);
        let error = with_air_upkeep("air_upkeep = \"../reference/nothing.toml\"\n")
            .expect_err("a missing file is an error");
        assert!(error.to_string().contains("nothing.toml"), "{error}");
    }

    /// `HEAD` with the fatigue key added after the surface combat key.
    fn with_fatigue(key: &str) -> Result<Scenario, DataError> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        let head = HEAD.replace(
            "surface_combat = \"../reference/surface_combat.toml\"\n",
            &format!("surface_combat = \"../reference/surface_combat.toml\"\n{key}"),
        );
        Scenario::parse(&head, &path)
    }

    #[test]
    fn the_fatigue_key_is_optional_and_loads_the_figures() {
        assert!(parse_with("")
            .expect("loads")
            .world
            .ref_data()
            .fatigue
            .is_none());
        let with = with_fatigue("fatigue = \"../reference/fatigue.toml\"\n").expect("loads");
        let figures = with
            .world
            .ref_data()
            .fatigue
            .expect("the figures are loaded");
        assert_eq!(figures.air_fatigue_per_sortie, Fixed::from_int(3));
        assert_eq!(figures.air_rest_per_pulse, Fixed::from_int(5));
        assert_eq!(figures.force_fatigue_per_pulse_alert, Fixed::from_int(5));
        assert_eq!(figures.force_rest_per_pulse, Fixed::from_int(3));
        let error = with_fatigue("fatigue = \"../reference/nothing.toml\"\n")
            .expect_err("a missing file is an error");
        assert!(error.to_string().contains("nothing.toml"), "{error}");
    }

    #[test]
    fn a_task_force_takes_an_optional_fatigue_in_whole_numbers_or_decimal_strings() {
        let fatigue = |extra: &str| {
            parse_with(&task_force(&format!("at_base = 1\n{extra}")))
                .map(|scenario| scenario.world.umpire().task_forces[&TaskForceId::new(1)].fatigue)
        };
        assert_eq!(fatigue("").expect("loads"), Fixed::ZERO);
        assert_eq!(fatigue("fatigue = 35").expect("loads"), Fixed::from_int(35));
        assert_eq!(
            fatigue("fatigue = \"2.5\"").expect("loads"),
            "2.5".parse::<Fixed>().expect("fixed")
        );
        assert_eq!(
            fatigue("fatigue = 100").expect("loads"),
            Fixed::from_int(100)
        );
        assert!(fatigue("fatigue = 101").is_err());
        assert!(fatigue("fatigue = -1").is_err());
        assert!(
            fatigue("fatigue = 3.5").is_err(),
            "a TOML float is rejected"
        );
    }

    #[test]
    fn the_air_supply_key_is_optional_and_loads_the_figures() {
        assert!(parse_with("")
            .expect("loads")
            .world
            .ref_data()
            .air_supply
            .is_none());
        let with = with_fatigue("air_supply = \"../reference/air_supply.toml\"\n").expect("loads");
        let figures = with
            .world
            .ref_data()
            .air_supply
            .expect("the figures are loaded");
        assert_eq!(figures.repair_supply_t, Fixed::from_int(1));
        assert_eq!(figures.daily_supply_t_per_aircraft, Fixed::from_ratio(1, 5));
        let error = with_fatigue("air_supply = \"../reference/nothing.toml\"\n")
            .expect_err("a missing file is an error");
        assert!(error.to_string().contains("nothing.toml"), "{error}");
        // The key is spelled exactly: a near miss is an unknown key.
        assert!(with_fatigue("air_supplies = \"../reference/air_supply.toml\"\n").is_err());
    }

    #[test]
    fn an_air_group_takes_an_optional_fatigue_in_whole_numbers_or_decimal_strings() {
        let fatigue = |extra: &str| {
            let text = air_group(1, 3).replace("ready = ", &format!("{extra}ready = "));
            parse_with(&text)
                .map(|scenario| scenario.world.umpire().air_groups[&AirGroupId::new(1)].fatigue)
        };
        assert_eq!(fatigue("").expect("loads"), Fixed::ZERO);
        assert_eq!(
            fatigue("fatigue = 40\n").expect("loads"),
            Fixed::from_int(40)
        );
        assert_eq!(
            fatigue("fatigue = \"6.2\"\n").expect("loads"),
            "6.2".parse::<Fixed>().expect("fixed")
        );
        assert!(fatigue("fatigue = 6.2\n").is_err());
        assert!(fatigue("fatigue = -1\n").is_err());
        assert!(fatigue("fatigue = 101\n").is_err());
        assert_eq!(
            fatigue("fatigue = 100\n").expect("loads"),
            Fixed::from_int(100)
        );
    }

    #[test]
    fn the_rest_threshold_order_loads_for_an_air_group_and_is_checked_at_intake() {
        let rest = "kind = \"set_rest_threshold\"\nfatigue = 30";
        parse_with(&order("air_group = 1", rest)).expect("a whole number loads");
        parse_with(&order(
            "air_group = 1",
            "kind = \"set_rest_threshold\"\nfatigue = \"12.5\"",
        ))
        .expect("a decimal string loads");
        // Never a float, always a value, and for an air group.
        let float = rest.replace("= 30", "= 30.5");
        assert!(parse_with(&order("air_group = 1", &float)).is_err());
        assert!(parse_with(&order("air_group = 1", "kind = \"set_rest_threshold\"")).is_err());
        assert!(error_of(&order("task_force = 1", rest)).contains("air_group"));
        // The range is checked when the order reaches intake, not here.
        parse_with(&order(
            "air_group = 1",
            "kind = \"set_rest_threshold\"\nfatigue = 150",
        ))
        .expect("a threshold out of range is rejected by the world, not the loader");
        // An unknown key is rejected.
        assert!(parse_with(&order("air_group = 1", &format!("{rest}\nextra = 1"))).is_err());
    }

    #[test]
    fn the_reach_order_loads_for_an_air_group_with_a_name_and_nothing_else() {
        let reach = |name: &str| format!("kind = \"set_reach\"\nreach = \"{name}\"");
        parse_with(&order("air_group = 1", &reach("normal"))).expect("normal loads");
        parse_with(&order("air_group = 1", &reach("extended"))).expect("extended loads");
        assert!(parse_with(&order("air_group = 1", &reach("long"))).is_err());
        assert!(parse_with(&order("air_group = 1", "kind = \"set_reach\"")).is_err());
        assert!(error_of(&order("task_force = 1", &reach("extended"))).contains("air_group"));
        // An unknown key is rejected.
        assert!(parse_with(&order(
            "air_group = 1",
            &format!("{}\nextra = 1", reach("extended"))
        ))
        .is_err());
    }

    #[test]
    fn the_target_priority_order_loads_for_an_air_group_with_kind_names_and_nothing_else() {
        let list = |kinds: &str| format!("kind = \"set_target_priority\"\nkinds = {kinds}");
        parse_with(&order(
            "air_group = 1",
            &list("[\"transport\", \"cruiser\"]"),
        ))
        .expect("a list of kinds loads");
        // An empty list or a repeated kind is the world's to refuse, at
        // intake.
        parse_with(&order("air_group = 1", &list("[]"))).expect("an empty list loads");
        parse_with(&order("air_group = 1", &list("[\"cruiser\", \"cruiser\"]")))
            .expect("a repeated kind loads");
        assert!(error_of(&order("air_group = 1", &list("[\"frigate\"]"))).contains("frigate"));
        assert!(parse_with(&order("air_group = 1", "kind = \"set_target_priority\"")).is_err());
        assert!(error_of(&order("task_force = 1", &list("[\"cruiser\"]"))).contains("air_group"));
        assert!(parse_with(&order(
            "air_group = 1",
            &format!("{}\nextra = 1", list("[\"cruiser\"]"))
        ))
        .is_err());
    }

    /// `HEAD` with the air combat key added after the surface combat key.
    fn with_air_combat(extra: &str) -> Result<Scenario, DataError> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        let head = HEAD.replace(
            "surface_combat = \"../reference/surface_combat.toml\"\n",
            "surface_combat = \"../reference/surface_combat.toml\"\nair_combat = \"../reference/air_combat.toml\"\n",
        );
        Scenario::parse(&format!("{head}{extra}"), &path)
    }

    #[test]
    fn the_air_combat_key_is_optional_and_loads_the_figures() {
        let without = parse_with("").expect("loads");
        assert!(without.world.ref_data().air_combat.is_none());
        let with = with_air_combat("").expect("loads");
        let figures = with
            .world
            .ref_data()
            .air_combat
            .as_ref()
            .expect("the figures are loaded");
        assert_eq!(figures.strike_contact_max_age_pulses, 0);
        assert_eq!(figures.bomb_hit_chance, Fixed::from_ratio(2, 10));
        let missing = HEAD.replace(
            "surface_combat = \"../reference/surface_combat.toml\"\n",
            "surface_combat = \"../reference/surface_combat.toml\"\nair_combat = \"../reference/nothing.toml\"\n",
        );
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        let error = Scenario::parse(&missing, &path).expect_err("a missing file is an error");
        assert!(error.to_string().contains("nothing.toml"), "{error}");
    }

    #[test]
    fn a_flight_deck_state_needs_a_class_that_carries_aircraft() {
        // Class 4 is the placeholder carrier, class 1 a cruiser.
        let carrier = |extra: &str| ship(extra).replace("class = 1", "class = 4");
        let deck = |extra: &str| {
            parse_with(&with_force(&carrier(extra)))
                .expect("loads")
                .world
                .umpire()
                .ships[&ShipId::new(1)]
                .systems
                .flight_deck
        };
        assert_eq!(deck(""), SystemState::Working);
        assert_eq!(deck("flight_deck = \"degraded\""), SystemState::Degraded);
        assert_eq!(deck("flight_deck = \"out\""), SystemState::Out);
        parse_with(&with_force(&ship("flight_deck = \"working\""))).expect("working loads");
        let message = error_of(&with_force(&ship("flight_deck = \"out\"")));
        assert!(message.contains("ship:1"), "{message}");
        assert!(
            message.contains("flight deck") && message.contains("has no"),
            "{message}"
        );
        assert!(parse_with(&with_force(&carrier("flight_deck = \"broken\""))).is_err());
    }

    #[test]
    fn a_misspelled_order_field_is_an_error() {
        let typo =
            "kind = \"set_search_arc\"\nfrom_deg = 60\nto_deg = 120\nrange_nm = 600\nrange_nmm = 5";
        assert!(parse_with(&order("air_group = 1", typo)).is_err());
        let typo = "kind = \"set_speed\"\nspeed = \"full\"\nspeeed = 5";
        assert!(parse_with(&order("task_force = 1", typo)).is_err());
        assert!(parse_with(&order("task_force = 1", "kind = \"load\"\nspeeed = 5")).is_err());
        let typo = "kind = \"load\"\nintnet = 1";
        assert!(parse_with(&order("task_force = 1", typo)).is_err());
    }

    fn ship(extra: &str) -> String {
        format!(
            "[[ship]]\nid = 1\nname = \"S\"\nclass = 1\nside = \"allied\"\nfuel_t = 100\ntask_force = 1\n{extra}\n"
        )
    }

    fn with_force(extra: &str) -> String {
        format!("{}{}", task_force("at_base = 1"), extra)
    }

    #[test]
    fn a_task_force_is_at_normal_readiness_unless_the_scenario_says_otherwise() {
        let truth = |scenario: Scenario| scenario.world.umpire().clone();
        let default = truth(parse_with(&task_force("at_base = 1")).expect("loads"));
        let force = &default.task_forces[&TaskForceId::new(1)];
        assert_eq!(force.readiness, Readiness::Normal);
        assert_eq!(force.readiness_order, None);
        assert!(!force.alerted);
        assert_eq!(force.alerted_by, None);
        assert!(!force.is_alert());
        assert_eq!(force.rules_of_engagement, RulesOfEngagement::DEFAULT);
        assert_eq!(force.rules_order, None);
        for (text, expected) in [
            ("readiness = \"normal\"", Readiness::Normal),
            ("readiness = \"battle\"", Readiness::Battle),
        ] {
            let loaded =
                truth(parse_with(&task_force(&format!("at_base = 1\n{text}"))).expect("loads"));
            let force = &loaded.task_forces[&TaskForceId::new(1)];
            assert_eq!(force.readiness, expected);
            assert_eq!(force.is_alert(), expected == Readiness::Battle);
        }
        assert!(parse_with(&task_force("at_base = 1\nreadiness = \"ready\"")).is_err());
        assert!(parse_with(&task_force("at_base = 1\nreadiness = true")).is_err());
        // The old key is gone: unknown fields are errors.
        assert!(parse_with(&task_force("at_base = 1\nalert = false")).is_err());
        assert!(parse_with(&task_force("at_base = 1\nalert = true")).is_err());
    }

    #[test]
    fn a_set_speed_order_gives_a_named_speed_or_knots_but_not_both_or_neither() {
        let speed_of = |body: &str| {
            let scenario = parse_with(&order("task_force = 1", body)).expect("loads");
            let (_, request) = &scenario.script[&1][0];
            match &request.kind {
                OrderKind::TaskForce {
                    order: TaskForceOrder::SetSpeed(speed),
                    ..
                } => *speed,
                other => panic!("not a speed order: {other:?}"),
            }
        };
        assert_eq!(
            speed_of("kind = \"set_speed\"\nspeed = \"full\""),
            SpeedSetting::Full
        );
        assert_eq!(
            speed_of("kind = \"set_speed\"\nknots = \"24\""),
            SpeedSetting::Knots(Fixed::from_int(24))
        );
        assert_eq!(
            speed_of("kind = \"set_speed\"\nknots = \"22.5\""),
            SpeedSetting::Knots(Fixed::from_ratio(45, 2))
        );
        let both = "kind = \"set_speed\"\nspeed = \"full\"\nknots = \"24\"";
        let message = error_of(&order("task_force = 1", both));
        assert!(
            message.contains("exactly one of speed and knots"),
            "{message}"
        );
        let message = error_of(&order("task_force = 1", "kind = \"set_speed\""));
        assert!(
            message.contains("exactly one of speed and knots"),
            "{message}"
        );
        let message = error_of(&order(
            "task_force = 1",
            "kind = \"set_speed\"\nknots = \"fast\"",
        ));
        assert!(message.contains("knots on day 1"), "{message}");
    }

    #[test]
    fn a_readiness_order_loads_for_a_task_force_only() {
        let scenario = parse_with(&order(
            "task_force = 1",
            "kind = \"set_readiness\"\nreadiness = \"battle\"",
        ))
        .expect("loads");
        let (_, request) = &scenario.script[&1][0];
        assert_eq!(
            request.kind,
            OrderKind::TaskForce {
                target: TaskForceId::new(1),
                order: TaskForceOrder::SetReadiness(Readiness::Battle),
            }
        );
        let scenario = parse_with(&order(
            "task_force = 1",
            "kind = \"set_readiness\"\nreadiness = \"normal\"",
        ))
        .expect("loads");
        assert!(matches!(
            &scenario.script[&1][0].1.kind,
            OrderKind::TaskForce {
                order: TaskForceOrder::SetReadiness(Readiness::Normal),
                ..
            }
        ));
        assert!(error_of(&order(
            "air_group = 1",
            "kind = \"set_readiness\"\nreadiness = \"battle\""
        ))
        .contains("air_group"));
        assert!(parse_with(&order("task_force = 1", "kind = \"set_readiness\"")).is_err());
        assert!(parse_with(&order(
            "task_force = 1",
            "kind = \"set_readiness\"\nreadiness = \"ready\""
        ))
        .is_err());
        assert!(parse_with(&order(
            "task_force = 1",
            "kind = \"set_readiness\"\nreadiness = \"battle\"\nextra = 1"
        ))
        .is_err());
    }

    #[test]
    fn a_ship_starts_with_the_guns_it_carries_and_its_torpedoes() {
        let scenario = parse_with(&with_force(&ship(""))).expect("loads");
        let truth = scenario.world.umpire();
        let cruiser = &truth.ships[&ShipId::new(1)];
        // Class 1: 8 guns x 100 rounds, 8 tubes.
        assert_eq!(cruiser.gun_rounds, Fixed::from_int(800));
        assert!(cruiser.torpedoes_ready);
        assert_eq!(cruiser.flotation_damage, Fixed::ZERO);
        assert_eq!(cruiser.systems, ShipSystems::ALL_WORKING);
        assert_eq!(cruiser.fire_damage, Fixed::ZERO);

        // Class 3 is a cargo ship: no guns and no tubes.
        let cargo = ship("").replace("class = 1", "class = 3");
        let scenario = parse_with(&with_force(&cargo)).expect("loads");
        let ship = &scenario.world.umpire().ships[&ShipId::new(1)];
        assert_eq!(ship.gun_rounds, Fixed::ZERO);
        assert!(!ship.torpedoes_ready);
    }

    #[test]
    fn a_ship_has_experience_2_unless_the_scenario_gives_0_to_4() {
        let experience = |text: &str| {
            parse_with(&with_force(&ship(text)))
                .map(|s| s.world.umpire().ships[&ShipId::new(1)].experience)
        };
        assert_eq!(experience("").expect("loads"), 2);
        for value in 0..=4u8 {
            assert_eq!(
                experience(&format!("experience = {value}")).expect("loads"),
                value
            );
        }
        // Outside 0 to 4, below 0, a float or a string: refused, naming the
        // ship and the figure.
        for bad in ["5", "-1", "100", "2.0", "\"3\"", "\"high\""] {
            let result = experience(&format!("experience = {bad}"));
            assert!(result.is_err(), "experience = {bad}");
        }
        let message = experience("experience = 5").expect_err("refused").message;
        assert!(message.contains("experience 5"), "{message}");
    }

    #[test]
    fn gun_rounds_are_a_whole_number_or_a_decimal_string() {
        let rounds = |text: &str| {
            parse_with(&with_force(&ship(&format!("gun_rounds = {text}"))))
                .map(|s| s.world.umpire().ships[&ShipId::new(1)].gun_rounds)
        };
        assert_eq!(rounds("120").expect("loads"), Fixed::from_int(120));
        assert_eq!(
            rounds("\"120.5\"").expect("loads"),
            Fixed::from_ratio(241, 2)
        );
        assert_eq!(rounds("0").expect("loads"), Fixed::ZERO);
        assert!(rounds("120.5").is_err());
        assert!(rounds("\"many\"").is_err());
        // A negative count starts the game in a state that breaks an
        // invariant.
        let message = rounds("-1").expect_err("rejected").message;
        assert!(message.contains("negative gun rounds"), "{message}");
    }

    #[test]
    fn a_ship_starts_with_the_system_states_the_scenario_gives() {
        // Default: everything working. Class 1 has guns and tubes, so all
        // four systems may be set.
        let truth = |extra: &str| {
            parse_with(&with_force(&ship(extra)))
                .expect("loads")
                .world
                .umpire()
                .ships[&ShipId::new(1)]
                .systems
        };
        assert_eq!(truth(""), ShipSystems::ALL_WORKING);
        let systems = truth(
            "propulsion = \"degraded\"\nfire_control = \"out\"\nmain_battery = \"working\"\ntorpedo_mounts = \"out\"",
        );
        assert_eq!(systems.propulsion, SystemState::Degraded);
        assert_eq!(systems.fire_control, SystemState::Out);
        assert_eq!(systems.main_battery, SystemState::Working);
        assert_eq!(systems.torpedo_mounts, SystemState::Out);
    }

    #[test]
    fn a_system_state_must_be_a_known_word_on_a_class_that_has_the_system() {
        assert!(parse_with(&with_force(&ship("propulsion = \"broken\""))).is_err());
        assert!(parse_with(&with_force(&ship("propulsion = 2"))).is_err());
        assert!(parse_with(&with_force(&ship("propulsions = \"out\""))).is_err());
        // Class 3 is a cargo ship with no guns and no tubes: only propulsion.
        let cargo = |extra: &str| ship(extra).replace("class = 1", "class = 3");
        parse_with(&with_force(&cargo("propulsion = \"out\""))).expect("propulsion loads");
        // A system the class lacks may be given as working, and nothing else.
        parse_with(&with_force(&cargo("main_battery = \"working\""))).expect("working loads");
        for (key, state) in [
            ("main_battery", "degraded"),
            ("fire_control", "out"),
            ("torpedo_mounts", "degraded"),
        ] {
            let message = error_of(&with_force(&cargo(&format!("{key} = \"{state}\""))));
            assert!(message.contains("ship:1"), "{message}");
            assert!(
                message.contains(&key.replace('_', " ")) && message.contains("has no"),
                "{message}"
            );
        }
    }

    #[test]
    fn a_task_force_may_have_a_home_port_of_its_own_side() {
        let force = |extra: &str| {
            parse_with(&task_force(&format!("at_base = 1\n{extra}")))
                .expect("loads")
                .world
                .umpire()
                .task_forces[&TaskForceId::new(1)]
                .home_port
        };
        assert_eq!(force(""), None);
        assert_eq!(force("home_port = 1"), Some(BaseId::new(1)));
        // Base 9 does not exist; base 2 is Japanese and the force is Allied.
        let message = error_of(&task_force("at_base = 1\nhome_port = 9"));
        assert!(message.contains("task_force:1") && message.contains("home port"));
        let message = error_of(&task_force("at_base = 1\nhome_port = 2"));
        assert!(
            message.contains("home port") && message.contains("Japanese"),
            "{message}"
        );
        assert!(parse_with(&task_force("at_base = 1\nhome_port = \"one\"")).is_err());
        assert!(parse_with(&task_force("at_base = 1\nhome_ports = 1")).is_err());
    }

    fn roe(body: &str) -> String {
        order(
            "task_force = 1",
            &format!("kind = \"set_rules_of_engagement\"\n{body}"),
        )
    }

    #[test]
    fn a_rules_of_engagement_order_loads() {
        let scenario = parse_with(&roe("stance = \"avoid\"\nbreak_off = \"0.25\"")).expect("loads");
        let (_, request) = &scenario.script[&1][0];
        assert_eq!(
            request.kind,
            OrderKind::TaskForce {
                target: TaskForceId::new(1),
                order: TaskForceOrder::SetRulesOfEngagement {
                    stance: Stance::Avoid,
                    break_off: Fixed::from_ratio(1, 4),
                    stragglers: StragglerInstruction::Keep,
                    keep_share: Fixed::ZERO,
                },
            }
        );
        let scenario = parse_with(&roe("stance = \"engage\"\nbreak_off = \"1\"")).expect("loads");
        let (_, request) = &scenario.script[&1][0];
        assert_eq!(
            request.kind,
            OrderKind::TaskForce {
                target: TaskForceId::new(1),
                order: TaskForceOrder::SetRulesOfEngagement {
                    stance: Stance::Engage,
                    break_off: Fixed::ONE,
                    stragglers: StragglerInstruction::Keep,
                    keep_share: Fixed::ZERO,
                },
            }
        );
        // A whole number works as for any quantity.
        parse_with(&roe("stance = \"engage\"\nbreak_off = 1")).expect("loads");
    }

    #[test]
    fn a_rules_of_engagement_order_loads_stragglers_and_keep_share() {
        for (text, wanted) in [
            ("keep", StragglerInstruction::Keep),
            ("detach", StragglerInstruction::Detach),
            ("leave", StragglerInstruction::Leave),
            ("retire", StragglerInstruction::Retire),
        ] {
            let scenario = parse_with(&roe(&format!(
                "stance = \"engage\"\nbreak_off = \"0.5\"\nstragglers = \"{text}\"\nkeep_share = \"0.5\""
            )))
            .expect("loads");
            let (_, request) = &scenario.script[&1][0];
            assert_eq!(
                request.kind,
                OrderKind::TaskForce {
                    target: TaskForceId::new(1),
                    order: TaskForceOrder::SetRulesOfEngagement {
                        stance: Stance::Engage,
                        break_off: Fixed::from_ratio(1, 2),
                        stragglers: wanted,
                        keep_share: Fixed::from_ratio(1, 2),
                    },
                }
            );
        }
        // A bad name, a float and a misspelled field are errors.
        assert!(parse_with(&roe(
            "stance = \"engage\"\nbreak_off = \"0.5\"\nstragglers = \"abandon\""
        ))
        .is_err());
        assert!(parse_with(&roe(
            "stance = \"engage\"\nbreak_off = \"0.5\"\nkeep_share = 0.5"
        ))
        .is_err());
        let message = error_of(&roe(
            "stance = \"engage\"\nbreak_off = \"0.5\"\nkeep_share = \"most\"",
        ));
        assert!(message.contains("keep_share on day 1"), "{message}");
        assert!(parse_with(&roe(
            "stance = \"engage\"\nbreak_off = \"0.5\"\nstraggler = \"detach\""
        ))
        .is_err());
        assert!(parse_with(&roe(
            "stance = \"engage\"\nbreak_off = \"0.5\"\nkeepshare = \"0.5\""
        ))
        .is_err());
    }

    #[test]
    fn a_home_port_order_loads_for_a_task_force_only() {
        let scenario = parse_with(&order(
            "task_force = 1",
            "kind = \"set_home_port\"\nbase = 2",
        ))
        .expect("loads");
        let (_, request) = &scenario.script[&1][0];
        assert_eq!(
            request.kind,
            OrderKind::TaskForce {
                target: TaskForceId::new(1),
                order: TaskForceOrder::SetHomePort(BaseId::new(2)),
            }
        );
        assert!(error_of(&order(
            "air_group = 1",
            "kind = \"set_home_port\"\nbase = 1"
        ))
        .contains("air_group"));
        assert!(parse_with(&order(
            "task_force = 1",
            "kind = \"set_home_port\"\nbase = 1.5"
        ))
        .is_err());
        assert!(parse_with(&order("task_force = 1", "kind = \"set_home_port\"")).is_err());
        assert!(parse_with(&order(
            "task_force = 1",
            "kind = \"set_home_port\"\nbse = 1"
        ))
        .is_err());
        assert!(parse_with(&order(
            "task_force = 1",
            "kind = \"set_home_port\"\nbase = 1\nextra = 1"
        ))
        .is_err());
    }

    #[test]
    fn a_detach_order_loads_for_a_task_force_only() {
        let scenario = parse_with(&order(
            "task_force = 1",
            "kind = \"detach\"\nships = [3, 1, 3]",
        ))
        .expect("loads");
        let (_, request) = &scenario.script[&1][0];
        assert_eq!(
            request.kind,
            OrderKind::TaskForce {
                target: TaskForceId::new(1),
                order: TaskForceOrder::DetachShips(vec![
                    ShipId::new(3),
                    ShipId::new(1),
                    ShipId::new(3)
                ]),
            }
        );
        // An empty list loads: intake rejects it, as a rule of the game.
        parse_with(&order("task_force = 1", "kind = \"detach\"\nships = []")).expect("loads");
        assert!(
            error_of(&order("air_group = 1", "kind = \"detach\"\nships = [1]"))
                .contains("air_group")
        );
        assert!(parse_with(&order("task_force = 1", "kind = \"detach\"")).is_err());
        assert!(parse_with(&order("task_force = 1", "kind = \"detach\"\nship = [1]")).is_err());
        assert!(parse_with(&order("task_force = 1", "kind = \"detach\"\nships = [1.5]")).is_err());
    }

    #[test]
    fn rules_of_engagement_order_errors_are_rejected() {
        // A float, text that is no number, an unknown stance, a missing or
        // misspelled field.
        assert!(parse_with(&roe("stance = \"engage\"\nbreak_off = 0.5")).is_err());
        assert!(parse_with(&roe("stance = \"engage\"\nbreak_off = \"half\"")).is_err());
        let message = error_of(&roe("stance = \"engage\"\nbreak_off = \"half\""));
        assert!(message.contains("break_off on day 1"), "{message}");
        assert!(parse_with(&roe("stance = \"charge\"\nbreak_off = \"0.5\"")).is_err());
        assert!(parse_with(&roe("stance = \"engage\"")).is_err());
        assert!(parse_with(&roe("break_off = \"0.5\"")).is_err());
        assert!(parse_with(&roe(
            "stance = \"engage\"\nbreak_off = \"0.5\"\nbreak_offf = 1"
        ))
        .is_err());
        // It is addressed to a task force, not an air group.
        let to_group = order(
            "air_group = 1",
            "kind = \"set_rules_of_engagement\"\nstance = \"engage\"\nbreak_off = \"0.5\"",
        );
        assert!(error_of(&to_group).contains("air_group"));
    }

    /// The task force orders scripted for day 1 of a scenario with one load
    /// order whose body is `body`.
    fn loaded_orders(body: &str) -> Result<Vec<TaskForceOrder>, DataError> {
        let scenario = parse_with(&order("task_force = 1", body))?;
        Ok(scenario.scripted_orders()[&1]
            .iter()
            .filter_map(|(_, request)| match &request.kind {
                OrderKind::TaskForce { order, .. } => Some(order.clone()),
                _ => None,
            })
            .collect())
    }

    #[test]
    fn a_load_order_names_its_cargo_and_defaults_to_supply() {
        assert_eq!(
            loaded_orders("kind = \"load\"").expect("loads"),
            vec![TaskForceOrder::Load]
        );
        assert_eq!(
            loaded_orders("kind = \"load\"\ncargo = \"supply\"").expect("loads"),
            vec![TaskForceOrder::Load]
        );
        assert_eq!(
            loaded_orders("kind = \"load\"\ncargo = \"fuel\"").expect("loads"),
            vec![TaskForceOrder::LoadFuel]
        );
        for bad in ["\"oil\"", "\"Fuel\"", "1", "\"\""] {
            assert!(
                loaded_orders(&format!("kind = \"load\"\ncargo = {bad}")).is_err(),
                "cargo = {bad} should not load"
            );
        }
        // An unload names no cargo.
        assert!(loaded_orders("kind = \"unload\"\ncargo = \"fuel\"").is_err());
    }

    #[test]
    fn a_ship_may_start_with_fuel_cargo_that_fits_beside_its_supply() {
        // Class 3 carries 3000 t.
        let cargo = |extra: &str| with_force(&ship(extra).replace("class = 1", "class = 3"));
        let aboard = |scenario: Scenario| {
            let ship = scenario.world.umpire().ships[&ShipId::new(1)].clone();
            (ship.supply_t, ship.cargo_fuel_t)
        };
        assert_eq!(
            aboard(parse_with(&cargo("")).expect("loads")),
            (Fixed::ZERO, Fixed::ZERO)
        );
        assert_eq!(
            aboard(parse_with(&cargo("cargo_fuel_t = 3000")).expect("loads")),
            (Fixed::ZERO, Fixed::from_int(3000))
        );
        assert_eq!(
            aboard(parse_with(&cargo("supply_t = 2000\ncargo_fuel_t = 1000")).expect("loads")),
            (Fixed::from_int(2000), Fixed::from_int(1000))
        );
        // Over the capacity together, though each is under it.
        let message = error_of(&cargo("supply_t = 2000\ncargo_fuel_t = 1001"));
        assert!(
            message.contains("fuel cargo but its class"),
            "unexpected message: {message}"
        );
        assert!(parse_with(&cargo("cargo_fuel_t = 3001")).is_err());
        // Not negative, not a float.
        assert!(parse_with(&cargo("cargo_fuel_t = -1")).is_err());
        assert!(parse_with(&cargo("cargo_fuel_t = 1.5")).is_err());
        // A class with no cargo space carries none.
        assert!(parse_with(&with_force(&ship("cargo_fuel_t = 1"))).is_err());
    }

    #[test]
    fn a_ship_with_more_fuel_than_its_class_holds_is_rejected() {
        // The helper's ship has 100 t, which its class holds. One with far
        // more does not load.
        assert!(parse_with(&with_force(&ship(""))).is_ok());
        let overfull = ship("").replace("fuel_t = 100", "fuel_t = 999999");
        let message = error_of(&with_force(&overfull));
        assert!(
            message.contains("of fuel but its class"),
            "unexpected message: {message}"
        );
    }

    #[test]
    fn the_surface_combat_path_is_required_and_the_file_is_loaded() {
        let scenario = parse_with(&task_force("at_base = 1")).expect("loads");
        let figures = scenario
            .world
            .ref_data()
            .surface_combat
            .as_ref()
            .expect("the loader supplies the figures");
        assert_eq!(figures.max_rounds, 6);

        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        let text = HEAD.replace(
            "surface_combat = \"../reference/surface_combat.toml\"\n",
            "",
        );
        let message = Scenario::parse(&text, &path).expect_err("rejected").message;
        assert!(message.contains("surface_combat"), "{message}");

        let text = HEAD.replace("../reference/surface_combat.toml", "../reference/nope.toml");
        assert!(Scenario::parse(&text, &path).is_err());
    }

    #[test]
    fn the_night_action_scenario_loads_and_builds_a_world() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../data/scenarios/04a_night_action.toml");
        let scenario = Scenario::load(&path).expect("scenario 4a loads");
        let truth = scenario.world.umpire();
        assert_eq!(truth.ships.len(), 6);
        assert_eq!(
            truth.task_forces[&TaskForceId::new(1)].readiness,
            Readiness::Battle
        );
        assert_eq!(
            truth.task_forces[&TaskForceId::new(2)].readiness,
            Readiness::Normal
        );
        assert!(truth
            .ships
            .values()
            .all(|ship| ship.gun_rounds == Fixed::from_int(800) && ship.torpedoes_ready));
        assert_eq!(scenario.script[&1].len(), 4);
    }

    #[test]
    fn with_seed_changes_only_the_seed() {
        let scenario = parse_with(&air_group(1, 1)).expect("loads");
        assert_eq!(scenario.world.seed(), 5);
        let other = scenario.with_seed(77).expect("copies");
        assert_eq!(other.world.seed(), 77);
        assert_eq!(other.name, scenario.name);
        assert_eq!(other.days, scenario.days);
        assert_eq!(other.world.umpire(), scenario.world.umpire());
        assert_eq!(other.script.len(), scenario.script.len());
        let mut started = scenario.clone();
        started.next_day().expect("runs");
        assert!(started.with_seed(1).is_err());
    }

    fn contact(body: &str) -> String {
        format!("[[contact]]\nside = \"allied\"\nq = 3\nr = 0\nship_count = 3\n{body}\n")
    }

    #[test]
    fn a_starting_contact_is_held_at_the_given_age() {
        // Age 2: two pulses old.
        let scenario = parse_with(&contact("kind = \"cruiser\"\nage_pulses = 2")).expect("loads");
        let view = scenario.world.side_view(Side::Allied);
        let held: Vec<_> = view.contacts().collect();
        assert_eq!(held.len(), 1);
        assert_eq!(held[0].estimated_hex, Hex::new(3, 0));
        assert_eq!(held[0].estimated_ship_count, 3);
        assert_eq!(held[0].age_pulses, 2);
        // One report and two ageing events, all on day 0.
        assert_eq!(scenario.world.events().len(), 3);

        // The age is optional and defaults to 0.
        let scenario = parse_with(&contact("kind = \"destroyer\"")).expect("loads");
        let view = scenario.world.side_view(Side::Allied);
        assert_eq!(view.contacts().next().expect("held").age_pulses, 0);
        assert_eq!(scenario.world.events().len(), 1);
    }

    #[test]
    fn a_bad_starting_contact_is_a_load_error() {
        let message = error_of(&contact("kind = \"dreadnought\""));
        assert!(
            message.contains("unknown kind \"dreadnought\""),
            "{message}"
        );
        let message = error_of(&contact("kind = \"cruiser\"\nspeed = 3"));
        assert!(message.contains("speed"), "{message}");
        // Age 6 is past the drop age of 5.
        let message = error_of(&contact("kind = \"cruiser\"\nage_pulses = 6"));
        assert!(message.contains("would be dropped"), "{message}");
        let message = error_of(&format!(
            "{}{}",
            contact("kind = \"cruiser\""),
            contact("kind = \"cruiser\"")
        ));
        assert!(message.contains("too close"), "{message}");
    }

    #[test]
    fn a_scenario_for_another_seed_holds_the_same_contacts() {
        let scenario = parse_with(&contact("kind = \"cruiser\"\nage_pulses = 1")).expect("loads");
        let other = scenario.with_seed(77).expect("copies");
        assert_eq!(other.world.events().len(), scenario.world.events().len());
        assert_eq!(other.world.state_hash(), {
            let mut again = scenario.clone();
            again = again.with_seed(77).expect("copies");
            again.world.state_hash()
        });
        assert_eq!(other.world.side_view(Side::Allied).contacts().count(), 1);
    }

    // -----------------------------------------------------------------
    // Objectives (decision 0054, "As built").
    // -----------------------------------------------------------------

    /// An `[objectives]` section with the given levels line and the given
    /// body for the allied objectives. The Japanese side has a mission and
    /// no objective.
    fn objectives(levels: &str, allied: &str) -> String {
        format!(
            "\n[objectives]\n{levels}\nhistory = \"A note.\"\n\n[objectives.allied]\nmission = \"Hold base one.\"\n{allied}\n[objectives.japanese]\nmission = \"Take base one.\"\n"
        )
    }

    const LEVELS: &str = "minor = 3\nmajor = 8\ndecisive = 16";

    const ALL_KINDS: &str = r#"
[[objectives.allied.objective]]
kind = "ships_sunk"
points = { carrier = 10, destroyer = 1 }

[[objectives.allied.objective]]
kind = "ships_crippled"
points = { carrier = 5, destroyer = "0.5" }

[[objectives.allied.objective]]
kind = "aircraft_destroyed"
points = "0.05"

[[objectives.allied.objective]]
kind = "cargo_landed"
base = 1
points_per_t = "0.1"

[[objectives.allied.objective]]
kind = "base_open"
base = 2
closed = true
points = 4
"#;

    #[test]
    fn a_scenario_may_give_each_side_a_mission_and_objectives_of_the_five_kinds() {
        let scenario = parse_with(&objectives(LEVELS, ALL_KINDS)).expect("loads");
        let found = scenario.world.objectives().expect("has objectives");
        assert_eq!(found.allied.mission, "Hold base one.");
        assert_eq!(found.japanese.mission, "Take base one.");
        assert_eq!(found.japanese.objectives.len(), 0);
        assert_eq!(found.history.as_deref(), Some("A note."));
        assert_eq!(
            (
                found.levels.minor,
                found.levels.major,
                found.levels.decisive
            ),
            (Fixed::from_int(3), Fixed::from_int(8), Fixed::from_int(16))
        );
        let kinds: Vec<&str> = found
            .allied
            .objectives
            .iter()
            .map(Objective::name)
            .collect();
        assert_eq!(
            kinds,
            [
                "ships_sunk",
                "ships_crippled",
                "aircraft_destroyed",
                "cargo_landed",
                "base_open"
            ]
        );
        match &found.allied.objectives[1] {
            Objective::ShipsCrippled(points) => {
                assert_eq!(points.of(ShipKind::Carrier), Fixed::from_int(5));
                assert_eq!(points.of(ShipKind::Destroyer), Fixed::from_ratio(1, 2));
                // A kind not listed is worth nothing.
                assert_eq!(points.of(ShipKind::Cruiser), Fixed::ZERO);
            }
            other => panic!("{other:?}"),
        }
        assert_eq!(
            found.allied.objectives[4],
            Objective::BaseOpen {
                base: BaseId::new(2),
                closed: true,
                points: Fixed::from_int(4)
            }
        );
        // Whole numbers and decimal strings both read; `closed` defaults to
        // false and `history` is optional.
        let plain = parse_with(
            "\n[objectives]\nminor = \"0.5\"\nmajor = 1\ndecisive = 2\n\n[objectives.allied]\nmission = \"A.\"\n\n[[objectives.allied.objective]]\nkind = \"base_open\"\nbase = 1\npoints = 1\n\n[objectives.japanese]\nmission = \"B.\"\n",
        )
        .expect("loads");
        let found = plain.world.objectives().expect("has objectives");
        assert_eq!(found.history, None);
        assert_eq!(
            found.allied.objectives[0],
            Objective::BaseOpen {
                base: BaseId::new(1),
                closed: false,
                points: Fixed::ONE
            }
        );
    }

    #[test]
    fn a_scenario_without_objectives_has_none() {
        let scenario = parse_with("").expect("loads");
        assert!(scenario.world.objectives().is_none());
        assert!(scenario.world.result().is_none());
        // A copy for another seed keeps them.
        let with = parse_with(&objectives(LEVELS, ALL_KINDS)).expect("loads");
        let other = with.with_seed(9).expect("copies");
        assert_eq!(other.world.objectives(), with.world.objectives());
    }

    // -----------------------------------------------------------------
    // Decision 0088: the horizon and ships out of action.
    // -----------------------------------------------------------------

    const OUT_OF_ACTION: &str = r#"
[[objectives.allied.objective]]
kind = "ships_out_of_action"
points = { carrier = 5, destroyer = "0.5" }
"#;

    fn horizon_of(levels: &str, allied: &str) -> Result<u32, String> {
        parse_with(&objectives(levels, allied))
            .map(|scenario| {
                scenario
                    .world
                    .objectives()
                    .expect("objectives")
                    .horizon_days
            })
            .map_err(|e| e.to_string())
    }

    #[test]
    fn the_horizon_is_the_scenarios_days_unless_a_longer_one_is_stated() {
        // HEAD has two days.
        assert_eq!(horizon_of(LEVELS, OUT_OF_ACTION), Ok(2));
        assert_eq!(horizon_of(LEVELS, ""), Ok(2));
        let stated = |value: &str| format!("{LEVELS}\nhorizon_days = {value}");
        assert_eq!(horizon_of(&stated("2"), OUT_OF_ACTION), Ok(2));
        assert_eq!(horizon_of(&stated("60"), OUT_OF_ACTION), Ok(60));
        // A horizon may be stated where nothing uses it.
        assert_eq!(horizon_of(&stated("60"), ""), Ok(60));
        let scenario = parse_with(&objectives(&stated("60"), OUT_OF_ACTION)).expect("loads");
        match &scenario
            .world
            .objectives()
            .expect("objectives")
            .allied
            .objectives[0]
        {
            Objective::ShipsOutOfAction(points) => {
                assert_eq!(points.of(ShipKind::Carrier), Fixed::from_int(5));
                assert_eq!(points.of(ShipKind::Destroyer), Fixed::from_ratio(1, 2));
                assert_eq!(points.of(ShipKind::Cruiser), Fixed::ZERO);
            }
            other => panic!("{other:?}"),
        }
        // A copy for another seed keeps the horizon.
        let other = scenario.with_seed(9).expect("copies");
        assert_eq!(
            other.world.objectives().expect("objectives").horizon_days,
            60
        );
    }

    #[test]
    fn a_horizon_that_is_zero_short_or_not_a_whole_number_is_refused() {
        let stated = |value: &str| format!("{LEVELS}\nhorizon_days = {value}");
        for (value, wanted) in [
            ("0", "horizon_days 0 is not a whole number of at least 1"),
            ("-4", "horizon_days -4 is not a whole number of at least 1"),
            ("1", "horizon_days 1 is shorter than the scenario's 2 days"),
        ] {
            let message = error_of(&objectives(&stated(value), OUT_OF_ACTION));
            assert!(message.contains(wanted), "{value}: {message}");
        }
        for value in ["2.5", "\"60\""] {
            let message = error_of(&objectives(&stated(value), OUT_OF_ACTION));
            assert!(message.contains("horizon_days"), "{value}: {message}");
        }
    }

    #[test]
    fn a_scenario_counts_damaged_ships_one_way_out_of_action_or_crippled() {
        let crippled = "\n[[objectives.allied.objective]]\nkind = \"ships_crippled\"\npoints = { carrier = 5 }\n";
        // On the same side, in either order.
        for both in [
            format!("{OUT_OF_ACTION}{crippled}"),
            format!("{crippled}{OUT_OF_ACTION}"),
        ] {
            let message = error_of(&objectives(LEVELS, &both));
            assert!(
                message.contains("counts damaged ships one way"),
                "{message}"
            );
        }
        // On different sides, in either order.
        let split = |allied: &str, japanese_kind: &str| {
            format!(
                "\n[objectives]\n{LEVELS}\n\n[objectives.allied]\nmission = \"A.\"\n{allied}\n[objectives.japanese]\nmission = \"B.\"\n\n[[objectives.japanese.objective]]\nkind = \"{japanese_kind}\"\npoints = {{ carrier = 5 }}\n"
            )
        };
        for text in [
            split(OUT_OF_ACTION, "ships_crippled"),
            split(crippled, "ships_out_of_action"),
        ] {
            let message = error_of(&text);
            assert!(
                message.contains("counts damaged ships one way"),
                "{message}"
            );
        }
        // Crippled alone, and out of action on both sides, are fine.
        assert!(parse_with(&objectives(LEVELS, crippled)).is_ok());
        assert!(parse_with(&split(OUT_OF_ACTION, "ships_out_of_action")).is_ok());
    }

    #[test]
    fn ships_out_of_action_needs_the_surface_combat_figures() {
        // The surface combat file is a required key, so a scenario without
        // it is refused whatever its objectives; the check in
        // `build_objectives` is the second line of that.
        let text = format!("{HEAD}{}", objectives(LEVELS, OUT_OF_ACTION)).replace(
            "surface_combat = \"../reference/surface_combat.toml\"\n",
            "",
        );
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        let message = Scenario::parse(&text, &path)
            .expect_err("refused")
            .to_string();
        assert!(message.contains("surface_combat"), "{message}");
        let spec: ObjectivesSpec = toml::from_str(
            "minor = 3\nmajor = 8\ndecisive = 16\n[allied]\nmission = \"A.\"\n[[allied.objective]]\nkind = \"ships_out_of_action\"\npoints = { carrier = 5 }\n[japanese]\nmission = \"B.\"\n",
        )
        .expect("parses");
        let message = build_objectives(&spec, 2, false).expect_err("refused");
        assert!(
            message.contains("needs the surface combat figures"),
            "{message}"
        );
        assert!(build_objectives(&spec, 2, true).is_ok());
    }

    #[test]
    fn objectives_are_checked_with_clear_errors() {
        let bad_levels = [
            "minor = 0\nmajor = 8\ndecisive = 16",
            "minor = 3\nmajor = 3\ndecisive = 16",
            "minor = 3\nmajor = 8\ndecisive = 8",
            "minor = 9\nmajor = 8\ndecisive = 16",
            "minor = -3\nmajor = 8\ndecisive = 16",
        ];
        for levels in bad_levels {
            let message = error_of(&objectives(levels, ""));
            assert!(
                message.contains("levels must be above zero and rising"),
                "{levels}: {message}"
            );
        }
        // A float level or points are rejected, as everywhere.
        let message = error_of(&objectives("minor = 0.5\nmajor = 8\ndecisive = 16", ""));
        assert!(message.contains("minor"), "{message}");
        let message = error_of(&objectives(
            LEVELS,
            "[[objectives.allied.objective]]\nkind = \"aircraft_destroyed\"\npoints = 0.05\n",
        ));
        assert!(!message.is_empty());
        let message = error_of(&objectives(
            LEVELS,
            "[[objectives.allied.objective]]\nkind = \"ships_sunk\"\npoints = { carrier = 0.5 }\n",
        ));
        assert!(!message.is_empty());
        // Negative points.
        let message = error_of(&objectives(
            LEVELS,
            "[[objectives.allied.objective]]\nkind = \"aircraft_destroyed\"\npoints = -1\n",
        ));
        assert!(
            message.contains("allied objective 1 points is negative"),
            "{message}"
        );
        let message = error_of(&objectives(
            LEVELS,
            "[[objectives.allied.objective]]\nkind = \"ships_sunk\"\npoints = { carrier = -10 }\n",
        ));
        assert!(message.contains("carrier is negative"), "{message}");
        let message = error_of(&objectives(
            LEVELS,
            "[[objectives.allied.objective]]\nkind = \"cargo_landed\"\nbase = 1\npoints_per_t = \"-0.1\"\n",
        ));
        assert!(message.contains("points_per_t is negative"), "{message}");
        // A named base that does not exist.
        for kind in [
            "cargo_landed\"\npoints_per_t = 1\nbase = 77\n#",
            "base_open\"\npoints = 1\nbase = 77\n#",
        ] {
            let message = error_of(&objectives(
                LEVELS,
                &format!("[[objectives.allied.objective]]\nkind = \"{kind}\n"),
            ));
            assert!(message.contains("unknown base:77"), "{message}");
        }
        // An unknown kind of objective, of ship, or field.
        let message = error_of(&objectives(
            LEVELS,
            "[[objectives.allied.objective]]\nkind = \"islands_taken\"\npoints = 1\n",
        ));
        assert!(message.contains("islands_taken"), "{message}");
        let message = error_of(&objectives(
            LEVELS,
            "[[objectives.allied.objective]]\nkind = \"ships_sunk\"\npoints = { dreadnought = 1 }\n",
        ));
        assert!(
            message.contains("unknown ship kind \"dreadnought\""),
            "{message}"
        );
        let message = error_of(&objectives(
            LEVELS,
            "[[objectives.allied.objective]]\nkind = \"aircraft_destroyed\"\npoints = 1\nbase = 1\n",
        ));
        assert!(message.contains("base"), "{message}");
        // An empty mission, a side missing.
        let message = error_of(&objectives(LEVELS, "").replace("Take base one.", "  "));
        assert!(message.contains("japanese mission is empty"), "{message}");
        let message = error_of(
            "\n[objectives]\nminor = 3\nmajor = 8\ndecisive = 16\n\n[objectives.allied]\nmission = \"A.\"\n",
        );
        assert!(message.contains("japanese"), "{message}");
    }

    // Land units (decision 0066).

    /// A small scenario on the landing map: base 1 at (0,0), a force of two
    /// troop transports (600 men each) at it, a force of one fast transport
    /// (200 men) at sea at (10,0), a Japanese force, and `extra` after them.
    /// `ground` says whether the ground figures are named.
    fn landing(ground: bool, extra: &str) -> Result<Scenario, DataError> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        let ground_key = if ground {
            "ground = \"../reference/ground.toml\"\n"
        } else {
            ""
        };
        let text = format!(
            r#"
name = "Test"
placeholder = true
seed = 5
days = 4
map = "../map/placeholder_landing.toml"
ship_classes = ["../reference/ship_classes.csv", "../reference/ship_classes_placeholder_transports.csv"]
port_sizes = "../reference/port_sizes.csv"
aircraft_types = "../reference/aircraft_types.csv"
search = "../reference/search.toml"
surface_combat = "../reference/surface_combat.toml"
{ground_key}
[[base]]
id = 1
name = "Port"
side = "allied"
q = 0
r = 0

[[task_force]]
id = 1
name = "Convoy"
side = "allied"
at_base = 1
mission = "transport"

[[task_force]]
id = 2
name = "Fast group"
side = "allied"
q = 10
r = 0
mission = "transport"

[[task_force]]
id = 3
name = "Japanese group"
side = "japanese"
q = 5
r = 0
mission = "transport"

[[ship]]
id = 1
name = "Troop ship 1"
class = 10
side = "allied"
fuel_t = 1000
task_force = 1

[[ship]]
id = 2
name = "Troop ship 2"
class = 10
side = "allied"
fuel_t = 1000
task_force = 1

[[ship]]
id = 3
name = "Fast ship"
class = 11
side = "allied"
fuel_t = 500
task_force = 2

[[ship]]
id = 4
name = "Japanese cargo ship"
class = 3
side = "japanese"
fuel_t = 1000
task_force = 3
{extra}"#
        );
        Scenario::parse(&text, &path)
    }

    fn landing_error(ground: bool, extra: &str) -> String {
        match landing(ground, extra) {
            Ok(_) => panic!("the scenario loads"),
            Err(error) => error.to_string(),
        }
    }

    fn unit_aboard(id: u32, men: u32, task_force: u32) -> String {
        format!(
            "\n[[land_unit]]\nid = {id}\nname = \"Unit {id}\"\nside = \"allied\"\nmen = {men}\ntask_force = {task_force}\n"
        )
    }

    fn unit_ashore(id: u32, men: u32, q: i32, r: i32) -> String {
        format!(
            "\n[[land_unit]]\nid = {id}\nname = \"Unit {id}\"\nside = \"allied\"\nmen = {men}\nq = {q}\nr = {r}\n"
        )
    }

    #[test]
    fn land_units_are_built_ashore_or_put_aboard_in_unit_order() {
        // Listed out of order: unit 2 is put aboard after unit 1 all the same.
        let extra = format!(
            "{}{}{}",
            unit_aboard(2, 400, 1),
            unit_ashore(3, 500, 10, 0),
            unit_aboard(1, 600, 1)
        );
        let scenario = landing(true, &extra).expect("loads");
        let truth = scenario.world.umpire();
        let ground = scenario.world.ref_data().ground.expect("the figures load");
        assert_eq!(ground.handling_t_per_man, Fixed::from_ratio(1, 4));
        assert_eq!(truth.land_units.len(), 3);
        let first = &truth.land_units[&LandUnitId::new(1)];
        assert_eq!((first.men_ashore, first.hex), (0, None));
        assert_eq!(first.side, Side::Allied);
        assert_eq!(first.name, "Unit 1");
        let third = &truth.land_units[&LandUnitId::new(3)];
        assert_eq!((third.men_ashore, third.hex), (500, Some(Hex::new(10, 0))));
        let troops = |ship: u32| truth.ships[&ShipId::new(ship)].troops;
        let aboard = |unit: u32, men: u32| {
            Some(sim_core::state::Troops {
                unit: LandUnitId::new(unit),
                men,
            })
        };
        assert_eq!(troops(1), aboard(1, 600));
        assert_eq!(troops(2), aboard(2, 400));
        assert_eq!(troops(3), None);
        assert_eq!(troops(4), None);
        assert!(truth.bases.values().all(|base| base.on_map));
    }

    #[test]
    fn a_unit_larger_than_a_ship_is_spread_over_the_ships_in_ship_order() {
        let scenario = landing(true, &unit_aboard(1, 900, 1)).expect("loads");
        let truth = scenario.world.umpire();
        assert_eq!(
            truth.ships[&ShipId::new(1)].troops.map(|t| t.men),
            Some(600)
        );
        assert_eq!(
            truth.ships[&ShipId::new(2)].troops.map(|t| t.men),
            Some(300)
        );
    }

    #[test]
    fn a_land_unit_without_a_ground_file_is_refused() {
        let message = landing_error(false, &unit_ashore(1, 100, 10, 0));
        assert!(message.contains("land unit"), "{message}");
        assert!(message.contains("ground"), "{message}");
        // A scenario with no land unit needs no ground file, and has none.
        let scenario = landing(false, "").expect("loads");
        assert!(scenario.world.ref_data().ground.is_none());
        assert!(scenario.world.umpire().land_units.is_empty());
        // The key names a file that must be there.
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        let text = "name = \"T\"\nground = \"../reference/nothing.toml\"\n";
        assert!(Scenario::parse(text, &path).is_err());
    }

    #[test]
    fn a_land_unit_that_does_not_fit_aboard_is_refused() {
        // Two transports hold 1,200 men.
        let message = landing_error(true, &unit_aboard(1, 1300, 1));
        assert!(message.contains("land_unit:1"), "{message}");
        assert!(message.contains("berths for 1200"), "{message}");
        // Unit 1 takes ship 1, unit 2 takes ship 2, and unit 3 has no room.
        let extra = format!(
            "{}{}{}",
            unit_aboard(1, 600, 1),
            unit_aboard(2, 600, 1),
            unit_aboard(3, 1, 1)
        );
        let message = landing_error(true, &extra);
        assert!(message.contains("land_unit:3"), "{message}");
        // A force whose ships have no troop capacity holds no man.
        let message = landing_error(true, &unit_aboard(1, 1, 3));
        assert!(message.contains("land_unit:1"), "{message}");
    }

    #[test]
    fn a_land_unit_needs_exactly_one_place() {
        let both = "\n[[land_unit]]\nid = 1\nname = \"U\"\nside = \"allied\"\nmen = 5\nq = 10\nr = 0\ntask_force = 1\n";
        let neither = "\n[[land_unit]]\nid = 1\nname = \"U\"\nside = \"allied\"\nmen = 5\n";
        let half = "\n[[land_unit]]\nid = 1\nname = \"U\"\nside = \"allied\"\nmen = 5\nq = 10\n";
        for extra in [both, neither, half] {
            let message = landing_error(true, extra);
            assert!(message.contains("land_unit:1"), "{message}");
            assert!(message.contains("exactly one of"), "{message}");
        }
    }

    #[test]
    fn a_land_unit_with_no_men_a_bad_number_or_a_duplicate_is_refused() {
        let message = landing_error(true, &unit_ashore(1, 0, 10, 0));
        assert!(message.contains("land_unit:1 has no men"), "{message}");
        let message = landing_error(true, &unit_ashore(0, 5, 10, 0));
        assert!(message.contains("land_unit:0"), "{message}");
        let message = landing_error(true, &unit_ashore(MADE_IDS_FLOOR, 5, 10, 0));
        assert!(message.contains(&MADE_IDS_FLOOR.to_string()), "{message}");
        let extra = format!("{}{}", unit_ashore(1, 5, 10, 0), unit_ashore(1, 6, 8, 0));
        let message = landing_error(true, &extra);
        assert!(message.contains("land_unit:1 appears twice"), "{message}");
    }

    #[test]
    fn a_land_unit_ashore_must_be_on_the_map_on_coast_or_land() {
        let message = landing_error(true, &unit_ashore(1, 5, 50, 50));
        assert!(message.contains("land_unit:1"), "{message}");
        assert!(message.contains("off the map"), "{message}");
        // (5,0) is deep sea.
        let message = landing_error(true, &unit_ashore(1, 5, 5, 0));
        assert!(message.contains("not coast or land"), "{message}");
        assert!(landing(true, &unit_ashore(1, 5, 8, 0)).is_ok());
    }

    #[test]
    fn a_land_unit_aboard_needs_a_force_of_its_own_side() {
        let message = landing_error(true, &unit_aboard(1, 5, 9));
        assert!(message.contains("land_unit:1"), "{message}");
        assert!(message.contains("unknown task force 9"), "{message}");
        // Task force 3 is Japanese.
        let message = landing_error(true, &unit_aboard(1, 5, 3));
        assert!(message.contains("land_unit:1"), "{message}");
        assert!(message.contains("Allied but its task force"), "{message}");
    }

    #[test]
    fn a_land_unit_has_no_unknown_keys() {
        let extra = format!("{}rank = 3\n", unit_ashore(1, 5, 10, 0));
        assert!(landing_error(true, &extra).contains("rank"));
    }

    fn load_orders(extra: &str) -> Result<Vec<TaskForceOrder>, DataError> {
        let orders: String = extra
            .split('|')
            .map(|body| {
                format!("\n[[order]]\nday = 1\nside = \"allied\"\ntask_force = 2\nkind = \"load\"\n{body}\n")
            })
            .collect();
        let scenario = landing(true, &orders)?;
        Ok(scenario.scripted_orders()[&1]
            .iter()
            .map(|(_, request)| match &request.kind {
                OrderKind::TaskForce { order, .. } => order.clone(),
                other => panic!("not a task force order: {other:?}"),
            })
            .collect())
    }

    #[test]
    fn a_load_order_may_name_a_unit_and_a_cargo_or_none() {
        let orders = load_orders(
            "|cargo = \"supply\"|cargo = \"fuel\"|unit = 3|unit = 3\ncargo = \"supply\"|unit = 3\ncargo = \"fuel\"|unit = 3\ncargo = \"none\"",
        )
        .expect("loads");
        let unit = LandUnitId::new(3);
        assert_eq!(
            orders,
            vec![
                TaskForceOrder::Load,
                TaskForceOrder::Load,
                TaskForceOrder::LoadFuel,
                TaskForceOrder::LoadUnit {
                    unit,
                    cargo: Some(Cargo::Supply)
                },
                TaskForceOrder::LoadUnit {
                    unit,
                    cargo: Some(Cargo::Supply)
                },
                TaskForceOrder::LoadUnit {
                    unit,
                    cargo: Some(Cargo::Fuel)
                },
                TaskForceOrder::LoadUnit { unit, cargo: None },
            ]
        );
    }

    #[test]
    fn a_load_order_with_no_unit_cannot_name_no_cargo() {
        let error = load_orders("cargo = \"none\"").expect_err("refused");
        let message = error.to_string();
        assert!(
            message.contains("a load with no unit must name supply or fuel"),
            "{message}"
        );
        assert!(load_orders("cargo = \"gold\"").is_err());
    }

    #[test]
    fn the_shipped_landing_scenario_loads_on_the_live_and_the_frozen_figures() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let path = root.join("data/scenarios/21_landing.toml");
        let live = Scenario::load(&path).expect("loads on the live figures");
        let frozen = Scenario::load_with(&path, &LoadOptions::frozen_step1(&root))
            .expect("loads on the frozen figures");
        for scenario in [&live, &frozen] {
            let truth = scenario.world.umpire();
            assert_eq!(scenario.days, 7);
            assert_eq!(truth.land_units.len(), 3);
            let aboard = |ship: u32| truth.ships[&ShipId::new(ship)].troops;
            assert_eq!(
                aboard(1),
                Some(sim_core::state::Troops {
                    unit: LandUnitId::new(1),
                    men: 600
                })
            );
            assert_eq!(
                aboard(2),
                Some(sim_core::state::Troops {
                    unit: LandUnitId::new(2),
                    men: 400
                })
            );
            assert_eq!(aboard(3), None);
            let third = &truth.land_units[&LandUnitId::new(3)];
            assert_eq!((third.men_ashore, third.hex), (500, Some(Hex::new(10, 0))));
            for unit in [1, 2] {
                let unit = &truth.land_units[&LandUnitId::new(unit)];
                assert_eq!((unit.men_ashore, unit.hex), (0, None));
            }
            let ground = scenario.world.ref_data().ground.expect("the figures load");
            assert_eq!(ground.daily_supply_t_per_man, Fixed::from_ratio(1, 200));
            let load = &scenario.scripted_orders()[&1];
            assert!(load.iter().any(|(_, request)| matches!(
                &request.kind,
                OrderKind::TaskForce {
                    order: TaskForceOrder::LoadUnit { cargo: None, .. },
                    ..
                }
            )));
        }
        assert!(live
            .data_files()
            .iter()
            .any(|file| file.ends_with("ground.toml")));
        assert!(frozen
            .data_files()
            .iter()
            .any(|file| file.to_string_lossy().contains("step1/ground.toml")));
    }

    // -----------------------------------------------------------------
    // Opening estimates of the enemy's men ashore (decision 0092, 16).
    // -----------------------------------------------------------------

    fn estimate(body: &str) -> String {
        format!("\n[[estimate]]\n{body}\n")
    }

    #[test]
    fn an_opening_estimate_is_held_on_day_0_with_no_error_and_no_cause() {
        let scenario =
            parse_with(&estimate("side = \"japanese\"\nq = 3\nr = 0\nmen = 300")).expect("loads");
        let view = scenario.world.side_view(Side::Japanese);
        let held = view.ground_estimate(Hex::new(3, 0)).expect("held");
        assert_eq!((held.men, held.day, held.hex), (300, 0, Hex::new(3, 0)));
        assert_eq!(view.ground_estimates().count(), 1);
        // The other side holds none, and is told nothing.
        let other = scenario.world.side_view(Side::Allied);
        assert_eq!(other.ground_estimates().count(), 0);
        assert!(other.events().is_empty());
        // One event on day 0, citing nothing.
        let events: Vec<_> = scenario.world.events().iter().collect();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].day, 0);
        assert!(events[0].causes.is_empty());
        assert!(events[0].breakdown.is_none());
        // A second for another hex, or for the other side at the same hex,
        // is allowed; the scenario keeps what it was given.
        let two = format!(
            "{}{}{}",
            estimate("side = \"japanese\"\nq = 3\nr = 0\nmen = 300"),
            estimate("side = \"japanese\"\nq = 4\nr = 0\nmen = 1"),
            estimate("side = \"allied\"\nq = 3\nr = 0\nmen = 0"),
        );
        let scenario = parse_with(&two).expect("loads");
        assert_eq!(
            scenario
                .world
                .side_view(Side::Japanese)
                .ground_estimates()
                .count(),
            2
        );
        assert_eq!(
            scenario
                .world
                .side_view(Side::Allied)
                .ground_estimate(Hex::new(3, 0))
                .expect("held")
                .men,
            0
        );
        // It is kept through a copy for another seed.
        let other = scenario.with_seed(77).expect("copies");
        assert_eq!(
            other
                .world
                .side_view(Side::Japanese)
                .ground_estimates()
                .count(),
            2
        );
    }

    #[test]
    fn a_bad_opening_estimate_is_a_load_error() {
        // Off the map.
        let message = error_of(&estimate("side = \"japanese\"\nq = 99\nr = 0\nmen = 300"));
        assert!(message.contains("off the map"), "{message}");
        // Two for the same side and hex.
        let message = error_of(&format!(
            "{}{}",
            estimate("side = \"japanese\"\nq = 3\nr = 0\nmen = 300"),
            estimate("side = \"japanese\"\nq = 3\nr = 0\nmen = 400"),
        ));
        assert!(message.contains("second estimate"), "{message}");
        // An unknown key.
        let message = error_of(&estimate(
            "side = \"japanese\"\nq = 3\nr = 0\nmen = 300\nerror = 2",
        ));
        assert!(message.contains("error"), "{message}");
        // A float, or a negative number, for men.
        for men in ["300.5", "0.5", "-1", "\"300\""] {
            let message = error_of(&estimate(&format!(
                "side = \"japanese\"\nq = 3\nr = 0\nmen = {men}"
            )));
            assert!(message.contains("men"), "men = {men}: {message}");
        }
        // A key missing.
        for body in [
            "q = 3\nr = 0\nmen = 300",
            "side = \"japanese\"\nr = 0\nmen = 300",
            "side = \"japanese\"\nq = 3\nmen = 300",
            "side = \"japanese\"\nq = 3\nr = 0",
        ] {
            let message = error_of(&estimate(body));
            assert!(message.contains("missing field"), "{body}: {message}");
        }
        // A side that is neither.
        assert!(parse_with(&estimate("side = \"french\"\nq = 3\nr = 0\nmen = 300")).is_err());
    }

    // -----------------------------------------------------------------
    // Marching overland (decision 0094).
    // -----------------------------------------------------------------

    fn unit_at_base(id: u32) -> String {
        format!(
            "\n[[land_unit]]\nid = {id}\nname = \"Unit {id}\"\nside = \"allied\"\nmen = 100\nq = 0\nr = 0\n"
        )
    }

    #[test]
    fn a_march_order_names_a_land_unit_and_a_hex() {
        let extra = format!(
            "{}\n[[order]]\nday = 2\nside = \"allied\"\nland_unit = 1\nkind = \"march\"\nhex = [3, -2]\n",
            unit_at_base(1)
        );
        let scenario = landing(true, &extra).expect("loads");
        let (side, request) = &scenario.scripted_orders()[&2][0];
        assert_eq!(*side, Side::Allied);
        match &request.kind {
            OrderKind::LandUnit { target, order } => {
                assert_eq!(*target, LandUnitId::new(1));
                assert_eq!(*order, LandUnitOrder::March(Hex::new(3, -2)));
            }
            other => panic!("not a land unit order: {other:?}"),
        }
    }

    #[test]
    fn a_march_order_must_be_for_a_land_unit_and_a_land_unit_takes_only_two_kinds() {
        let order = |target: &str, kind: &str, key: &str| {
            format!(
                "{}\n[[order]]\nday = 1\nside = \"allied\"\n{target}\nkind = \"{kind}\"\n{key}\n",
                unit_at_base(1)
            )
        };
        let message = landing_error(true, &order("task_force = 1", "march", "hex = [1, 0]"));
        assert!(
            message.contains("march") && message.contains("land_unit"),
            "{message}"
        );
        let message = landing_error(
            true,
            &order("land_unit = 1", "set_route", "waypoints = [[1, 0]]"),
        );
        assert!(
            message.contains("set_posture, march or set_activity"),
            "{message}"
        );
        // The hex is two whole numbers.
        let message = landing_error(true, &order("land_unit = 1", "march", "hex = [1]"));
        assert!(!message.is_empty());
    }

    // -----------------------------------------------------------------
    // Experience, training and preparing a landing (decisions 0095, 0097).
    // -----------------------------------------------------------------

    fn unit_with(id: u32, keys: &str) -> String {
        format!(
            "\n[[land_unit]]\nid = {id}\nname = \"Unit {id}\"\nside = \"allied\"\nmen = 100\nq = 0\nr = 0\n{keys}\n"
        )
    }

    #[test]
    fn a_land_unit_takes_experience_and_counts() {
        let scenario = landing(
            true,
            &format!(
                "{}{}",
                unit_ashore(1, 100, 0, 0),
                unit_with(2, "experience = 4\ntraining_days = 29\nfight_days = 9")
            ),
        );
        // Unit 1 is built the usual way; unit 2 gives all three keys.
        let scenario = scenario.expect("loads");
        let truth = scenario.world.umpire();
        let one = &truth.land_units[&LandUnitId::new(1)];
        assert_eq!(
            (one.experience, one.training_days, one.fight_days),
            (2, 0, 0)
        );
        let two = &truth.land_units[&LandUnitId::new(2)];
        assert_eq!(
            (two.experience, two.training_days, two.fight_days),
            (4, 29, 9)
        );
    }

    #[test]
    fn a_land_unit_takes_fieldworks_up_to_the_limit() {
        let scenario = landing(
            true,
            &format!(
                "{}{}{}{}",
                unit_ashore(1, 100, 0, 0),
                unit_with(2, "fieldworks = 30"),
                unit_with(3, "fieldworks = \"12.5\""),
                unit_with(4, "fieldworks = 0"),
            ),
        )
        .expect("loads");
        let truth = scenario.world.umpire();
        let fieldworks = |unit: u32| truth.land_units[&LandUnitId::new(unit)].fieldworks;
        assert_eq!(fieldworks(1), Fixed::ZERO);
        assert_eq!(fieldworks(2), Fixed::from_int(30));
        assert_eq!(fieldworks(3), Fixed::from_ratio(25, 2));
        assert_eq!(fieldworks(4), Fixed::ZERO);
    }

    #[test]
    fn fieldworks_above_the_limit_or_below_nothing_are_refused() {
        for keys in ["fieldworks = 31", "fieldworks = -1", "fieldworks = 3.5"] {
            let message = landing_error(true, &unit_with(1, keys));
            assert!(
                message.contains("land_unit:1") || message.contains("fieldworks"),
                "{keys}: {message}"
            );
        }
        let message = landing_error(true, &unit_with(1, "fieldworks = 31"));
        assert!(message.contains("fieldworks_limit"), "{message}");
        let message = landing_error(true, &unit_with(1, "fieldworks = -1"));
        assert!(message.contains("fieldworks"), "{message}");
    }

    #[test]
    fn fieldworks_are_refused_for_a_working_unit_an_attacker_and_a_unit_aboard() {
        let message = landing_error(true, &unit_with(1, "working = true\nfieldworks = 10"));
        assert!(message.contains("land_unit:1"), "{message}");
        assert!(message.contains("fieldworks"), "{message}");
        let message = landing_error(true, &unit_with(1, "posture = \"attack\"\nfieldworks = 10"));
        assert!(message.contains("land_unit:1"), "{message}");
        assert!(message.contains("fieldworks"), "{message}");
        let aboard = format!("{}fieldworks = 10\n", unit_aboard(1, 100, 1));
        let message = landing_error(true, &aboard);
        assert!(message.contains("land_unit:1"), "{message}");
        assert!(message.contains("fieldworks"), "{message}");
        // Zero fieldworks are no fieldworks: allowed anywhere.
        landing(true, &unit_with(1, "working = true\nfieldworks = 0")).expect("loads");
        landing(true, &unit_with(1, "posture = \"attack\"\nfieldworks = 0")).expect("loads");
    }

    #[test]
    fn a_land_unit_takes_skills_and_any_not_given_is_2() {
        let scenario = landing(
            true,
            &format!(
                "{}{}",
                unit_ashore(1, 100, 0, 0),
                unit_with(
                    2,
                    "skills = { assault = 4, holding = 3, weight = 0, mountain = 4, clear = 1 }"
                )
            ),
        )
        .expect("loads");
        let truth = scenario.world.umpire();
        assert_eq!(
            truth.land_units[&LandUnitId::new(1)].skills,
            Skills::STANDARD_ALL
        );
        assert_eq!(
            truth.land_units[&LandUnitId::new(2)].skills,
            Skills {
                assault: 4,
                holding: 3,
                weight: 0,
                clear: 1,
                jungle: 2,
                mountain: 4,
                atoll: 2,
                works: 2,
            }
        );
    }

    #[test]
    fn a_skill_is_whole_from_0_to_4_and_named() {
        let message = landing_error(true, &unit_with(1, "skills = { weight = 5 }"));
        assert!(message.contains("weight skill of 5"), "{message}");
        for keys in [
            "skills = { assault = -1 }",
            "skills = { holding = 1.5 }",
            "skills = { mountain = \"3\" }",
            "skills = { swimming = 3 }",
        ] {
            assert!(landing(true, &unit_with(1, keys)).is_err(), "{keys}");
        }
        assert!(landing(true, &unit_with(1, "skills = { atoll = 0, jungle = 4 }")).is_ok());
        assert!(landing(true, &unit_with(1, "skills = {}")).is_ok());
    }

    #[test]
    fn experience_and_the_counts_are_whole_and_in_range() {
        let message = landing_error(true, &unit_with(1, "experience = 5"));
        assert!(message.contains("experience 5"), "{message}");
        let message = landing_error(true, &unit_with(1, "experience = -1"));
        assert!(!message.is_empty());
        let message = landing_error(true, &unit_with(1, "experience = 1.5"));
        assert!(!message.is_empty());
        // The live ground figures: 30 days of training and 10 of fighting
        // for a step.
        let message = landing_error(true, &unit_with(1, "training_days = 30"));
        assert!(message.contains("training"), "{message}");
        let message = landing_error(true, &unit_with(1, "fight_days = 10"));
        assert!(message.contains("fighting"), "{message}");
        let message = landing_error(true, &unit_with(1, "fight_days = -1"));
        assert!(!message.is_empty());
        let message = landing_error(true, &unit_with(1, "training_days = 2.5"));
        assert!(!message.is_empty());
        assert!(landing(true, &unit_with(1, "training_days = 29\nfight_days = 9")).is_ok());
    }

    #[test]
    fn a_prepare_landing_order_names_a_force_a_hex_and_a_roster() {
        let order = |roster: &str| {
            format!(
                "{}\n[[order]]\nday = 1\nside = \"allied\"\ntask_force = 1\nkind = \"prepare_landing\"\nhex = [4, 0]\nroster = {roster}\n",
                unit_aboard(1, 100, 1)
            )
        };
        let scenario = landing(true, &order("[1, 3]")).expect("loads");
        let (side, request) = &scenario.scripted_orders()[&1][0];
        assert_eq!(*side, Side::Allied);
        match &request.kind {
            OrderKind::TaskForce { target, order } => {
                assert_eq!(*target, TaskForceId::new(1));
                assert_eq!(
                    *order,
                    TaskForceOrder::PrepareLanding {
                        hex: Hex::new(4, 0),
                        roster: [LandUnitId::new(1), LandUnitId::new(3)]
                            .into_iter()
                            .collect(),
                    }
                );
            }
            other => panic!("not a task force order: {other:?}"),
        }
        // An empty roster is accepted by the loader.
        let scenario = landing(true, &order("[]")).expect("loads");
        match &scenario.scripted_orders()[&1][0].1.kind {
            OrderKind::TaskForce {
                order: TaskForceOrder::PrepareLanding { roster, .. },
                ..
            } => assert!(roster.is_empty()),
            other => panic!("not a prepare order: {other:?}"),
        }
    }

    #[test]
    fn a_prepare_landing_order_is_refused_for_a_land_unit_and_when_malformed() {
        let order = |target: &str, keys: &str| {
            format!(
                "\n[[order]]\nday = 1\nside = \"allied\"\n{target}\nkind = \"prepare_landing\"\n{keys}\n"
            )
        };
        let message = landing_error(
            true,
            &(unit_ashore(1, 100, 0, 0) + &order("land_unit = 1", "hex = [4, 0]\nroster = []")),
        );
        assert!(
            message.contains("set_posture, march or set_activity"),
            "{message}"
        );
        // The hex and the roster are both needed, the hex is two whole
        // numbers and the roster is whole numbers.
        for keys in [
            "roster = []",
            "hex = [4, 0]",
            "hex = [4]\nroster = []",
            "hex = [4, 0]\nroster = [1.5]",
            "hex = [4, 0]\nroster = [-1]",
            "hex = [4, 0]\nroster = []\nmarch = 1",
        ] {
            assert!(
                landing(true, &order("task_force = 1", keys)).is_err(),
                "{keys}"
            );
        }
    }

    #[test]
    fn a_set_activity_order_names_a_land_unit_and_ready_or_train() {
        let order = |activity: &str| {
            format!(
                "{}\n[[order]]\nday = 3\nside = \"allied\"\nland_unit = 1\nkind = \"set_activity\"\nactivity = \"{activity}\"\n",
                unit_at_base(1)
            )
        };
        for (name, wanted) in [("train", Activity::Train), ("ready", Activity::Ready)] {
            let scenario = landing(true, &order(name)).expect("loads");
            match &scenario.scripted_orders()[&3][0].1.kind {
                OrderKind::LandUnit { target, order } => {
                    assert_eq!(*target, LandUnitId::new(1));
                    assert_eq!(*order, LandUnitOrder::SetActivity(wanted));
                }
                other => panic!("not a land unit order: {other:?}"),
            }
        }
        // An unknown activity, none, and the order given to a task force
        // are refused.
        assert!(landing(true, &order("idle")).is_err());
        let none = unit_at_base(1)
            + "\n[[order]]\nday = 3\nside = \"allied\"\nland_unit = 1\nkind = \"set_activity\"\n";
        assert!(landing(true, &none).is_err());
        let message = landing_error(
            true,
            &(unit_at_base(1)
                + "\n[[order]]\nday = 3\nside = \"allied\"\ntask_force = 1\nkind = \"set_activity\"\nactivity = \"train\"\n"),
        );
        assert!(
            message.contains("set_activity") && message.contains("land_unit"),
            "{message}"
        );
    }

    #[test]
    fn a_base_takes_an_airfield_flag_work_done_on_it_and_defences() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        let base = |extra: &str| {
            let head = HEAD.replace("q = 5\nr = 0\n", &format!("q = 5\nr = 0\n{extra}"));
            Scenario::parse(&head, &path)
        };
        let figures = |scenario: Scenario| {
            let base = scenario.world.umpire().bases[&BaseId::new(2)].clone();
            (
                base.airfield,
                base.airfield_work,
                base.defences,
                base.day_work,
            )
        };
        assert_eq!(
            figures(base("").expect("loads")),
            (true, Fixed::ZERO, Fixed::ZERO, None)
        );
        assert_eq!(
            figures(
                base("airfield = false\nairfield_work = 5000\ndefences = \"12.5\"\n")
                    .expect("loads")
            ),
            (false, Fixed::from_int(5000), Fixed::from_ratio(25, 2), None)
        );
        assert_eq!(
            figures(base("airfield = false\nairfield_work = \"0.5\"\n").expect("loads")).1,
            Fixed::from_ratio(1, 2)
        );
        assert_eq!(
            figures(base("defences = 100\n").expect("loads")).2,
            Fixed::from_int(100)
        );
        assert!(figures(base("airfield = false\n").expect("loads")).1 == Fixed::ZERO);
        // Work on an airfield that exists, negative work, defences out of
        // range and floats are refused.
        let message = base("airfield_work = 100\n")
            .map(|_| ())
            .expect_err("refused")
            .to_string();
        assert!(
            message.contains("airfield_work") && message.contains("has an airfield"),
            "{message}"
        );
        assert!(base("airfield = true\nairfield_work = 0\n").is_err());
        assert!(base("airfield = false\nairfield_work = -1\n").is_err());
        assert!(base("airfield = false\nairfield_work = 1.5\n").is_err());
        let message = base("defences = 101\n")
            .map(|_| ())
            .expect_err("refused")
            .to_string();
        assert!(
            message.contains("defences") && message.contains("0 and 100"),
            "{message}"
        );
        assert!(base("defences = -1\n").is_err());
        assert!(base("defences = 1.5\n").is_err());
        assert!(base("airfield = \"no\"\n").is_err());
    }

    #[test]
    fn a_base_takes_airfield_damage_only_with_an_airfield() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        let base = |extra: &str| {
            let head = HEAD.replace("q = 5\nr = 0\n", &format!("q = 5\nr = 0\n{extra}"));
            Scenario::parse(&head, &path)
        };
        let damage =
            |scenario: Scenario| scenario.world.umpire().bases[&BaseId::new(2)].airfield_damage;
        assert_eq!(damage(base("").expect("loads")), Fixed::ZERO);
        assert_eq!(
            damage(base("airfield_damage = 30\n").expect("loads")),
            Fixed::from_int(30)
        );
        assert_eq!(
            damage(base("airfield_damage = \"12.5\"\n").expect("loads")),
            Fixed::from_ratio(25, 2)
        );
        assert_eq!(
            damage(base("airfield_damage = 100\n").expect("loads")),
            Fixed::from_int(100)
        );
        let message = base("airfield = false\nairfield_damage = 10\n")
            .map(|_| ())
            .expect_err("refused")
            .to_string();
        assert!(
            message.contains("airfield_damage") && message.contains("no airfield"),
            "{message}"
        );
        let message = base("airfield_damage = 101\n")
            .map(|_| ())
            .expect_err("refused")
            .to_string();
        assert!(
            message.contains("airfield_damage") && message.contains("0 and 100"),
            "{message}"
        );
        assert!(base("airfield_damage = -1\n").is_err());
        assert!(base("airfield_damage = 1.5\n").is_err());
    }

    #[test]
    fn an_air_group_may_not_start_at_a_base_with_no_airfield() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        let with_group = |extra: &str| {
            let head = HEAD.replace("q = 0\nr = 0\n", &format!("q = 0\nr = 0\n{extra}"));
            Scenario::parse(&format!("{head}\n{}", air_group(1, 1)), &path)
        };
        assert!(with_group("").is_ok());
        let message = with_group("airfield = false\n")
            .map(|_| ())
            .expect_err("refused")
            .to_string();
        assert!(message.contains("no airfield"), "{message}");
    }

    /// Decision 0101: a group with no aircraft and no pilots may start at a
    /// base that is not its side's or has no airfield; with an aircraft or a
    /// pilot it is refused as before.
    #[test]
    fn an_empty_air_group_may_start_at_an_enemy_base_or_one_with_no_airfield() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        let group = |base: u32, extra: &str| {
            format!(
                "[[air_group]]\nid = 1\nname = \"AG\"\nside = \"allied\"\nbase = {base}\naircraft_type = 1\n{extra}"
            )
        };
        let load = |head: &str, group: String| Scenario::parse(&format!("{head}\n{group}"), &path);
        let no_field = HEAD.replace("q = 0\nr = 0\n", "q = 0\nr = 0\nairfield = false\n");
        // The other side's base.
        assert!(load(HEAD, group(2, "ready = 0\n")).is_ok());
        assert!(load(HEAD, group(2, "ready = 1\n"))
            .expect_err("refused")
            .to_string()
            .contains("its base"));
        assert!(load(HEAD, group(2, "ready = 0\ndamaged = 1\n")).is_err());
        assert!(load(HEAD, group(2, "ready = 0\npilots = 1\n")).is_err());
        assert!(load(HEAD, group(2, "ready = 0\npilots = 0\n")).is_ok());
        // Its own side's base with no airfield.
        assert!(load(&no_field, group(1, "ready = 0\n")).is_ok());
        assert!(load(&no_field, group(1, "ready = 2\n"))
            .expect_err("refused")
            .to_string()
            .contains("no airfield"));
        // Both: the enemy's base with no airfield.
        let both = HEAD.replace("q = 5\nr = 0\n", "q = 5\nr = 0\nairfield = false\n");
        assert!(load(&both, group(2, "ready = 0\n")).is_ok());
        assert!(load(&both, group(2, "ready = 1\n")).is_err());
    }

    #[test]
    fn a_working_unit_is_marked_works_and_may_not_attack() {
        let scenario = landing(
            true,
            &format!(
                "{}{}",
                unit_ashore(1, 100, 0, 0),
                unit_with(2, "working = true\nskills = { works = 4 }")
            ),
        )
        .expect("loads");
        let truth = scenario.world.umpire();
        let fighter = &truth.land_units[&LandUnitId::new(1)];
        assert!(!fighter.working);
        assert_eq!(fighter.activity, Activity::Ready);
        assert_eq!(fighter.skills.works, 2);
        let worker = &truth.land_units[&LandUnitId::new(2)];
        assert!(worker.working);
        assert_eq!(worker.activity, Activity::Work(None));
        assert_eq!(worker.skills.works, 4);
        assert!(landing(true, &unit_with(1, "working = false\nposture = \"attack\"")).is_ok());
        assert!(landing(true, &unit_with(1, "working = true\nposture = \"defend\"")).is_ok());
        let message = landing_error(true, &unit_with(1, "working = true\nposture = \"attack\""));
        assert!(
            message.contains("working unit") && message.contains("attack"),
            "{message}"
        );
        // The works skill is whole, 0 to 4.
        let message = landing_error(true, &unit_with(1, "skills = { works = 5 }"));
        assert!(message.contains("works skill of 5"), "{message}");
        assert!(landing(true, &unit_with(1, "skills = { works = 0 }")).is_ok());
        assert!(landing(true, &unit_with(1, "skills = { works = -1 }")).is_err());
        assert!(landing(true, &unit_with(1, "skills = { works = 1.5 }")).is_err());
        assert!(landing(true, &unit_with(1, "working = \"yes\"")).is_err());
    }

    #[test]
    fn a_set_activity_order_may_be_work_with_a_job() {
        let order = |extra: &str| {
            format!(
                "{}\n[[order]]\nday = 3\nside = \"allied\"\nland_unit = 1\nkind = \"set_activity\"\n{extra}",
                unit_at_base(1)
            )
        };
        let wanted_activity = |extra: &str| {
            let scenario = landing(true, &order(extra)).expect("loads");
            match &scenario.scripted_orders()[&3][0].1.kind {
                OrderKind::LandUnit { order, .. } => order.clone(),
                other => panic!("not a land unit order: {other:?}"),
            }
        };
        assert_eq!(
            wanted_activity("activity = \"work\"\n"),
            LandUnitOrder::SetActivity(Activity::Work(None))
        );
        for (name, job) in [
            ("airfield", Job::Airfield),
            ("port", Job::Port),
            ("aircraft", Job::Aircraft),
            ("defences", Job::Defences),
        ] {
            assert_eq!(
                wanted_activity(&format!("activity = \"work\"\njob = \"{name}\"\n")),
                LandUnitOrder::SetActivity(Activity::Work(Some(job)))
            );
        }
        // An unknown job, and a job with another activity, are refused.
        let message = landing_error(true, &order("activity = \"work\"\njob = \"farming\"\n"));
        assert!(
            message.contains("farming") && message.contains("job"),
            "{message}"
        );
        for activity in ["ready", "train"] {
            let message = landing_error(
                true,
                &order(&format!("activity = \"{activity}\"\njob = \"port\"\n")),
            );
            assert!(message.contains("only the activity work"), "{message}");
        }
    }

    #[test]
    fn a_scenario_names_a_land_links_file_that_adds_links_and_is_a_data_file() {
        let dir = std::env::temp_dir().join(format!("flagplot-land-links-{}", std::process::id()));
        fs::create_dir_all(&dir).expect("make the temp dir");
        let map = dir.join("map.toml");
        fs::write(
            &map,
            "placeholder = true\n[[hex]]\nq = 0\nr = 0\nkind = \"coast\"\nport_size = 3\n\
             [[hex]]\nq = 1\nr = 0\nkind = \"coast\"\n\
             [[hex]]\nq = 5\nr = 0\nkind = \"coast\"\n",
        )
        .expect("write the map");
        let links = dir.join("links.csv");
        fs::write(
            &links,
            "# placeholder\nq1,r1,q2,r2,landmass,reason\n0,0,1,0,Placeholder island,invented\n",
        )
        .expect("write the links");
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        let text = HEAD.replace(
            "map = \"../map/placeholder_search.toml\"",
            &format!(
                "map = {:?}\nland_links = {:?}",
                map.to_string_lossy(),
                links.to_string_lossy()
            ),
        );
        let scenario = Scenario::parse(&text, &path).expect("loads");
        assert!(scenario
            .world
            .map()
            .has_land_link(Hex::new(0, 0), Hex::new(1, 0)));
        assert_eq!(scenario.data_files()[0], map);
        assert_eq!(scenario.data_files()[1], links);
        // A scenario without the key has no links.
        let plain = Scenario::parse(HEAD, &path).expect("loads");
        assert_eq!(plain.world.map().land_links().count(), 0);
        // A bad links file is the scenario's error, naming the file.
        fs::write(&links, "q1,r1,q2,r2,landmass,reason\n0,0,5,5,a,b\n").expect("rewrite");
        let error = Scenario::parse(&text, &path)
            .expect_err("refused")
            .to_string();
        assert!(
            error.contains("links.csv") && error.contains("not neighbours"),
            "{error}"
        );
        fs::remove_dir_all(&dir).ok();
    }
}
