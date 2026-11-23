use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use sim_core::believed::{BelievedForce, BelievedIntent, BelievedMen, LastKnown, Standing};
use sim_core::choices::Moment;
use sim_core::command::CommandKind;
use sim_core::commander::{Commander, Initiative, NamedCommander};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{
    AirGroupId, AircraftTypeId, BaseId, EntityRef, FormationId, IntentId, LandUnitId, ShipClassId,
    ShipId, SubmarineClassId, SubmarineId, TaskForceId,
};
use sim_core::map::{HexKind, Island, IslandPlace};
use sim_core::objectives::{Levels, Objective, Objectives, ShipPoints, SideObjectives};
use sim_core::orders::{
    Activity, AirGroupOrder, AirMission, BaseOrder, Cargo, Issuer, Job, LandUnitOrder, Mission,
    OrderKind, OrderRequest, Origin, Posture, Reach, Readiness, SpeedSetting, Stance,
    StragglerInstruction, SubmarineOrder, TaskForceOrder, WhenSpent,
};
use sim_core::pictures::{CommandRef, NamedSuperior};
use sim_core::refdata::{RefData, ShipKind, SubmarineData, TrialFigures};
use sim_core::state::{
    AirGroup, AirGroupLocation, Arrival, Base, Formation, LandUnit, Replacement, Ship,
    ShipLocation, ShipSystem, ShipSystems, Side, Skills, Submarine, SubmarineCondition,
    SubmarinePassage, SystemState, TaskForce, TruthState, WatersWaiting,
};
use sim_core::time::Pulse;
use sim_core::world::MADE_IDS_FLOOR;
use sim_core::{DaySummary, Staffs, StartingContact, StartingEstimate, World};

use crate::commands::{BelievedIntentSpec, CommandSpec};
use crate::error::DataError;
use crate::map::{load_land_links, load_map, read_map_frame, MapFrame};
use crate::quantity::Quantity;
use crate::reference::{
    load_air_combat_figures, load_air_supply_figures, load_air_upkeep_figures, load_aircraft_types,
    load_class_sides, load_fatigue_figures, load_ground_figures, load_port_sizes,
    load_report_figures, load_search_figures, load_ship_classes_marked, load_submarine_classes,
    load_submarine_figures, load_submarine_torpedoes, load_surface_combat_figures,
    load_trial_figures, parse_kind,
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
    /// Path to the report figures, relative to the scenario file. Optional
    /// (decision 0135): without it, with command levels on, every report
    /// reaches every command at once. Not a reference file redirected by
    /// `LoadOptions::reference_root`: it is read where the scenario names it,
    /// as the map is, since the frozen step 1 figures have no report figures.
    reports: Option<String>,
    /// Path to the file of sides, relative to the scenario file (decision
    /// 0173, 1): whose each class of ship and type of aircraft is. Optional:
    /// without it no class or type has a side. A reference file, redirected
    /// by `LoadOptions::reference_root` as the class files are.
    class_sides: Option<String>,
    /// Path to a trial's figures, relative to the scenario file (decision
    /// 0174): what a base of the enemy's is given when a side's picture is
    /// made. Optional: a scenario that is not to be tried names none. A
    /// reference file, redirected by `LoadOptions::reference_root` as the
    /// class files are. Kept with the scenario, never put into the world.
    trial: Option<String>,
    /// Path to the submarine class file, relative to the scenario file
    /// (decision 0218). A reference file, redirected by
    /// `LoadOptions::reference_root` as the class files are. A scenario
    /// names all of `submarine_classes`, `submarine_torpedoes` and
    /// `submarines`, or none.
    submarine_classes: Option<String>,
    /// Path to the submarine torpedo file (decision 0218): a reference file,
    /// redirected as the class files are.
    submarine_torpedoes: Option<String>,
    /// Path to the submarine figures, relative to the scenario file
    /// (decision 0218). Not a reference file redirected by
    /// `LoadOptions::reference_root`: it is read where the scenario names
    /// it, as `reports` is, since the frozen step 1 figures give none and a
    /// frozen game has no boat.
    submarines: Option<String>,
    #[serde(default)]
    base: Vec<BaseSpec>,
    #[serde(default)]
    land_unit: Vec<LandUnitSpec>,
    /// Submarines (decision 0218): units of their own kind.
    #[serde(default)]
    submarine: Vec<SubmarineSpec>,
    #[serde(default)]
    air_group: Vec<AirGroupSpec>,
    #[serde(default)]
    replacement: Vec<ReplacementSpec>,
    #[serde(default)]
    arrival: Vec<ArrivalSpec>,
    #[serde(default)]
    supply_line: Vec<SupplyLineSpec>,
    #[serde(default)]
    men_replacement: Vec<MenReplacementSpec>,
    #[serde(default)]
    task_force: Vec<TaskForceSpec>,
    #[serde(default)]
    ship: Vec<ShipSpec>,
    #[serde(default)]
    order: Vec<OrderSpec>,
    /// What a side's commander says, by day (decision 0107, item 16).
    #[serde(default)]
    command: Vec<CommandSpec>,
    #[serde(default)]
    contact: Vec<ContactSpec>,
    /// Opening estimates of the enemy's men ashore (decision 0092, 16).
    #[serde(default)]
    estimate: Vec<EstimateSpec>,
    /// The waters where a side suspects enemy submarines work (decision
    /// 0218, 23 and 47).
    #[serde(default)]
    suspected_waters: Vec<SuspectedWatersSpec>,
    /// The enemy forces a side believes in and holds no contact of
    /// (decision 0173, 5).
    #[serde(default)]
    believed_force: Vec<BelievedForceSpec>,
    /// The intents a side believes the enemy holds (decision 0173, 7).
    #[serde(default)]
    believed_intent: Vec<BelievedIntentSpec>,
    /// The men of the enemy's a side believes may be ashore at a place
    /// beyond its estimate (decision 0176, 1).
    #[serde(default)]
    believed_men: Vec<BelievedMenSpec>,
    /// Missions and objectives (decision 0054). Optional.
    objectives: Option<ObjectivesSpec>,
    /// Whether command levels are on (decision 0134, 1). Optional; off when
    /// absent.
    command_levels: Option<bool>,
    /// Whether a side may give orders inside a day, at 0600 and 1200
    /// (decision 0156, 1). Optional; off when absent.
    orders_inside_a_day: Option<bool>,
    /// The commanders of the side's commands (decision 0137, 1 and 2).
    #[serde(default)]
    commander: Vec<CommanderSpec>,
    /// Formations of land units, each with its parent (decision 0140, 7).
    #[serde(default)]
    formation: Vec<FormationSpec>,
    /// Islands: named places a side may be asked to control (decision 0151,
    /// 1). Data on the map, not state.
    #[serde(default)]
    island: Vec<IslandSpec>,
}

/// An island (decision 0151, 1): a number, a name and its places in order.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct IslandSpec {
    number: u32,
    name: String,
    places: Vec<IslandPlaceSpec>,
}

/// One place of an island: exactly one of a base of the scenario and a coast
/// hex as `[q, r]`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct IslandPlaceSpec {
    base: Option<u32>,
    hex: Option<[i32; 2]>,
}

/// A formation of land units (decision 0140, 7): its number, name, side and
/// the formation it belongs to, if any, which must be there and of the same
/// side. No formation may be its own ancestor.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FormationSpec {
    id: u32,
    name: String,
    side: SideSpec,
    parent: Option<u32>,
}

/// A commander of one command (decision 0137, 1 and 2): the side, exactly
/// one of `task_force`, `base` or `area = true`, his name, his initiative
/// rating (`cautious`, `steady` or `bold`) and the note of what the rating
/// rests on, which is required. Every rating is a placeholder.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CommanderSpec {
    side: SideSpec,
    task_force: Option<u32>,
    base: Option<u32>,
    area: Option<bool>,
    name: String,
    initiative: String,
    note: Option<String>,
}

/// The command a task force or a base is placed under (decision 0134, 4):
/// exactly one of `task_force` or `base`, as in
/// `superior = { task_force = 1 }`.
#[derive(Deserialize, Clone, Copy)]
#[serde(deny_unknown_fields)]
struct SuperiorSpec {
    task_force: Option<u32>,
    base: Option<u32>,
}

impl SuperiorSpec {
    fn command(&self) -> Result<CommandRef, String> {
        match (self.task_force, self.base) {
            (Some(force), None) => Ok(CommandRef::TaskForce(TaskForceId::new(force))),
            (None, Some(base)) => Ok(CommandRef::Base(BaseId::new(base))),
            _ => Err("a superior names exactly one of task_force or base".to_string()),
        }
    }
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
    /// Decision 0187, 22: `true` for a scenario for play, which has a scale
    /// of its own; absent or `false`, the scenario is of record.
    for_play: Option<bool>,
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
    /// Decision 0186: points for each 100 men the side's ships land in the
    /// base's hex and do not take off again.
    MenLanded {
        base: u32,
        points_per_100_men: Quantity,
    },
    /// Decision 0218, 32: points for each enemy submarine sunk.
    SubmarinesSunk {
        points: Quantity,
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
    /// The one command that saw it (decision 0137, 6), as in
    /// `seen_by = { task_force = 1 }`: with command levels on it is in that
    /// command's picture alone, as its own, and sent to no one. With them
    /// off it is the side's, as any other. Defaults to none: every command
    /// was told of it.
    seen_by: Option<SuperiorSpec>,
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

/// The waters where a side believes enemy submarines work (decision 0218, 23
/// and 47): a hex and a distance round it, the day the side is told (the
/// first day by default), the words of the belief, the rows it rests on and
/// the architect's reading of the rows into this hex and distance, marked.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SuspectedWatersSpec {
    side: SideSpec,
    q: i32,
    r: i32,
    distance: u32,
    day: Option<u32>,
    words: String,
    rows: String,
    reading: String,
}

/// A force of the enemy's a side believes in and holds no contact of
/// (decision 0173, 5): the side that believes it, a name, the kind of its
/// most important ship, its ships, its standing (`present`, `absent` or
/// `unknown`), and where it was last known: `at_base`, an enemy base on the
/// side's map, or `q` and `r`, with `day` (0, the default, for before the
/// first day); or neither, where the side does not know where. `placeholder`
/// is true where the belief rests on no source, and then `note` is required.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BelievedForceSpec {
    side: SideSpec,
    name: String,
    kind: String,
    ships: u32,
    standing: String,
    at_base: Option<u32>,
    q: Option<i32>,
    r: Option<i32>,
    day: Option<u32>,
    placeholder: Option<bool>,
    note: Option<String>,
}

/// Men of the enemy's a side believes may be ashore at a place beyond those
/// it holds an estimate of (decision 0176, 1, amended 6 and 7): the side that
/// believes it, a name, the place (`at_base`, a base of the enemy's on the
/// side's map, or `q` and `r`, a hex of land on the map), the men, 1 or more,
/// and the standing (`present`, `absent` or `unknown`). `placeholder` is true
/// where the belief rests on no source, and then `note` is required.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BelievedMenSpec {
    side: SideSpec,
    name: String,
    men: u32,
    standing: String,
    at_base: Option<u32>,
    q: Option<i32>,
    r: Option<i32>,
    placeholder: Option<bool>,
    note: Option<String>,
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
    /// The supply the staff's own loads leave in the base (decision 0111).
    /// Defaults to 0. May not be negative.
    supply_reserve_t: Option<Quantity>,
    /// The fuel refuelling leaves in the base (decision 0111). Defaults to
    /// 0. May not be negative.
    fuel_reserve_t: Option<Quantity>,
    /// Ammunition for ships at the start, in tons (decision 0122). Defaults
    /// to 0. May not be negative.
    ammunition_t: Option<Quantity>,
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
    /// The command this base is placed under, if not its side's area
    /// command (decision 0134, 4). Needs `command_levels = true`.
    superior: Option<SuperiorSpec>,
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
    /// The day from which the side may use it (decision 0140, 1): a whole
    /// number from 1. Without it, from day 1.
    available_from: Option<u32>,
    /// The day the unit enters the game (decision 0195, 1): a whole number
    /// from 2 up to the scenario's `days`. Until then it is held apart, in
    /// no hex and in no count. Without it, the unit is there on day 1.
    /// Placed at a base (a land unit ashore in a hex with a base); not one
    /// aboard a force, which enters with its force.
    enters: Option<u32>,
    /// The formation the unit belongs to (decision 0140, 7): a formation of
    /// its side.
    formation: Option<u32>,
    /// The men the unit should have (decision 0197, 1 and 19): not below
    /// its `men`. Without it, its `men`.
    establishment: Option<u32>,
    /// The unit's nights of hunger at the start (decision 0205, 7): a whole
    /// number. Without it, 0. Only the hunger figures read it.
    hunger_nights: Option<u32>,
}

/// A submarine (decision 0218): a unit of her own kind with a station.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SubmarineSpec {
    id: u32,
    name: String,
    side: SideSpec,
    class: u32,
    /// The hex she starts in.
    q: i32,
    r: i32,
    /// Her station. Without it, her hex. A station that is not her hex
    /// means she starts on passage to it.
    station_q: Option<i32>,
    station_r: Option<i32>,
    /// The base she goes home to: required.
    home: u32,
    /// The day she leaves for home (rule 3): a whole number from 1.
    leave_day: Option<u32>,
    /// The day she enters the game (decision 0218, 33): a whole number from
    /// 2 up to the scenario's `days`. Until then she is held apart, in no
    /// hex for any rule.
    enters: Option<u32>,
    /// The day from which her side may use her: a whole number from 1.
    available_from: Option<u32>,
    /// Salvoes at the start: from 0, at most her class's. Without it, her
    /// class's.
    salvoes: Option<u32>,
    /// `sound` or `damaged`. A damaged boat starts going home. Without it,
    /// sound.
    condition: Option<String>,
    /// Pulses she starts held down, counted from the first, the night:
    /// 0 for none. Without it, 0.
    held_down_pulses: Option<u32>,
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
    /// The oldest contact, in pulses, the group launches a naval strike on
    /// at the start (decision 0114). A whole number from 0 up to one less
    /// than the search figures' contact drop age. Without it the group uses
    /// the air combat figure. Signed so that a negative number is refused
    /// here with a message that names the range.
    strike_contact_age_pulses: Option<i64>,
    /// The share of its aircraft the group holds back from a strike at the
    /// start, 0 to 1 (decision 0114). Defaults to 0.
    strike_held_share: Option<Quantity>,
    /// The day from which the side may use it (decision 0140, 1): a whole
    /// number from 1. Without it, from day 1.
    available_from: Option<u32>,
    /// The day the unit enters the game (decision 0195, 1): a whole number
    /// from 2 up to the scenario's `days`. Until then it is held apart, in
    /// no hex and in no count. Without it, the unit is there on day 1.
    /// At a base; a group aboard a ship enters with its ship's force.
    enters: Option<u32>,
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

/// A supply line: tons of supply and fuel a day that appear at a base from
/// off the map, on the days from `first_day` to `last_day` (decision 0196).
/// Its side may differ from the base's side at the start.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SupplyLineSpec {
    base: u32,
    /// Tons of supply a day. Defaults to 0.
    supply_t: Option<Quantity>,
    /// Tons of fuel a day. Defaults to 0.
    fuel_t: Option<Quantity>,
    /// Defaults to 1.
    first_day: Option<u32>,
    /// Defaults to the game's last day.
    last_day: Option<u32>,
    /// Defaults to the base's side at the start.
    side: Option<SideSpec>,
}

/// A stream of replacement men: so many men a day who join the land units
/// of its side ashore at a base that are below their establishment
/// (decision 0197).
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct MenReplacementSpec {
    base: u32,
    /// Men a day, a whole number above 0.
    men: u32,
    /// Defaults to 1; nothing is given on day 1.
    first_day: Option<u32>,
    /// Defaults to the game's last day.
    last_day: Option<u32>,
    /// Defaults to the base's side at the start.
    side: Option<SideSpec>,
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
    /// The stance of its rules of engagement at the start, `engage` or
    /// `avoid` (decision 0124's scenarios). Defaults to engage, the rest of
    /// the rules of engagement as they default.
    stance: Option<StanceSpec>,
    /// The command this force is placed under, if not its side's area
    /// command (decision 0134, 4). Needs `command_levels = true`.
    superior: Option<SuperiorSpec>,
    /// Whether the force keeps radio silence from the start (decision 0136,
    /// 15). Defaults to false. It has an effect only where reports travel.
    radio_silence: Option<bool>,
    /// The day from which the side may use it (decision 0140, 1): a whole
    /// number from 1. Without it, from day 1.
    available_from: Option<u32>,
    /// The day the unit enters the game (decision 0195, 1): a whole number
    /// from 2 up to the scenario's `days`. Until then it is held apart, in
    /// no hex and in no count. Without it, the unit is there on day 1.
    /// The force enters at its hex, with its ships and all aboard.
    enters: Option<u32>,
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
    /// Torpedo tubes that have fired at the start, the highest numbered
    /// (decision 0122). Defaults to 0; at most the class's tubes. With every
    /// tube fired the ship has no torpedo ready.
    tubes_fired: Option<u32>,
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
    /// The day from which the side may use it (decision 0140, 1): a whole
    /// number from 1. Without it, from day 1.
    available_from: Option<u32>,
    /// The day the unit enters the game (decision 0195, 1): a whole number
    /// from 2 up to the scenario's `days`. Until then it is held apart, in
    /// no hex and in no count. Without it, the unit is there on day 1.
    /// Refused: a ship enters with its force (decision 0195, 1).
    enters: Option<u32>,
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
    /// Exactly one of `task_force`, `air_group`, `land_unit`, `base` and
    /// `submarine` must be given.
    task_force: Option<u32>,
    air_group: Option<u32>,
    /// A base, for a `form_task_force` order (decision 0111).
    base: Option<u32>,
    /// A land unit, for a `set_posture`, `march` or `set_activity` order.
    land_unit: Option<u32>,
    /// A submarine, for a `station` order (decision 0218).
    submarine: Option<u32>,
    #[serde(default)]
    issuer: IssuerSpec,
    intent: Option<u32>,
    /// The pulse an order given inside a day is handed in for: `"morning"`
    /// (given at 0600) or `"afternoon"` (given at 1200). Left out for an
    /// order of dusk. Only in a scenario with `orders_inside_a_day = true`
    /// (decision 0156).
    pulse: Option<String>,
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
        /// `stand_at` marks waypoints to stand at for the night, by hex
        /// (decision 0124, 1); each must be one of the waypoints.
        #[serde(default)]
        stand_at: Vec<[i32; 2]>,
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
    /// With `weight_t` and no `unit` (decision 0111): load until the force
    /// has that many tons aboard. With `weight_t` it may give `each_t`
    /// (decision 0162): the most any one ship is to hold, above 0.
    Load {
        cargo: Option<CargoSpec>,
        unit: Option<u32>,
        weight_t: Option<Quantity>,
        each_t: Option<Quantity>,
    },
    Unload {},
    /// Join the force to another of its side stopped at the same base
    /// (decision 0111).
    Join {
        into: u32,
    },
    /// Stand the force down into port (decision 0111). No other key.
    StandDown {},
    /// Give the force a charge to escort, another task force of its side by
    /// number, or with no `charge` clear it (decision 0121).
    Escort {
        charge: Option<u32>,
    },
    /// Fuel at sea from another task force of the side by number, the giver
    /// (decision 0201, 4).
    FuelFrom {
        giver: u32,
    },
    /// An order to a base: form the ships, lying in port there in no force,
    /// into a task force (decision 0111).
    FormTaskForce {
        ships: Vec<u32>,
    },
    /// `target_base` is required for `base_strike` and given for no other
    /// mission (decision 0034).
    /// `hex` is required for `reconnaissance` and given for no other mission
    /// (decision 0147).
    SetAirMission {
        mission: AirMissionSpec,
        target_base: Option<u32>,
        hex: Option<[i32; 2]>,
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
    /// The oldest contact, in pulses, the group launches a naval strike on
    /// (decision 0114). The range is checked at intake, as for the patrol
    /// share.
    SetStrikeContactAge {
        pulses: i64,
    },
    /// The share of its aircraft the group holds back from a strike
    /// (decision 0114).
    SetStrikeHeldShare {
        share: Quantity,
    },
    /// Switch on or off the sizing of the group's strikes to their target
    /// (decision 0118).
    SetStrikeSizing {
        on: bool,
    },
    /// Switch on or off the group's flying its search once a day, in the
    /// morning (decision 0131).
    SetSearchOnce {
        on: bool,
    },
    /// The waters the group strikes first, as `[q, r]` pairs (decision
    /// 0190, 13). An empty list clears them; a hex off the map or named
    /// twice is refused when the order reaches intake.
    SetStrikeFirstWaters {
        hexes: Vec<[i32; 2]>,
    },
    /// Move the air group to the named base of its side (decision 0091). A
    /// base that is not the side's, or out of reach, is refused at intake.
    /// The base is the order's `base` key, which `OrderSpec` reads first
    /// (decision 0111: it is also the target key of a base order).
    Rebase {},
    SetRulesOfEngagement {
        stance: StanceSpec,
        break_off: Quantity,
        #[serde(default)]
        stragglers: StragglersSpec,
        keep_share: Option<Quantity>,
    },
    /// The base is the order's `base` key, as for `Rebase`.
    SetHomePort {},
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
    /// Do no cargo work: clear the force's cargo order (decision 0191). No
    /// other key.
    NoCargoWork {},
    /// Send a submarine to a station and keep it (decision 0218). The hex is
    /// `[q, r]`.
    Station {
        hex: [i32; 2],
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
    Reconnaissance,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SideSpec {
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
/// A unit's day from the scenario (decision 0140, 1): a whole number from
/// 1, or none.
fn available_day(day: Option<u32>, unit: impl std::fmt::Display) -> Result<Option<u32>, String> {
    match day {
        Some(0) => Err(format!(
            "{unit} has available_from = 0: days are numbered from 1"
        )),
        day => Ok(day),
    }
}

/// The formations of a scenario (decision 0140, 7), checked: each number
/// once, each parent there and of the same side, and none its own ancestor.
fn formations(specs: &[FormationSpec]) -> Result<BTreeMap<FormationId, Formation>, String> {
    let mut found: BTreeMap<FormationId, Formation> = BTreeMap::new();
    for spec in specs {
        let id = FormationId::new(spec.id);
        if spec.id == 0 {
            return Err(format!("{id} is numbered 0, which is no formation"));
        }
        let formation = Formation {
            id,
            name: spec.name.clone(),
            side: spec.side.into(),
            parent: spec.parent.map(FormationId::new),
        };
        if found.insert(id, formation).is_some() {
            return Err(format!("{id} appears twice"));
        }
    }
    for formation in found.values() {
        let Some(parent) = formation.parent else {
            continue;
        };
        let Some(above) = found.get(&parent) else {
            return Err(format!(
                "{} has the parent {parent}, which is not there",
                formation.id
            ));
        };
        if above.side != formation.side {
            return Err(format!(
                "{} is {} but its parent {parent} is {}",
                formation.id,
                formation.side.name(),
                above.side.name()
            ));
        }
        // Up the tree: a loop comes back to where it began within as many
        // steps as there are formations.
        let mut at = Some(parent);
        for _ in 0..found.len() {
            match at {
                Some(next) if next == formation.id => {
                    return Err(format!("{} is its own ancestor", formation.id));
                }
                Some(next) => at = found.get(&next).and_then(|f| f.parent),
                None => break,
            }
        }
    }
    Ok(found)
}

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
                ObjectiveSpec::MenLanded {
                    base,
                    points_per_100_men,
                } => Objective::MenLanded {
                    base: BaseId::new(*base),
                    points_per_100_men: points_of(
                        points_per_100_men,
                        &format!("{what} points_per_100_men"),
                    )?,
                },
                ObjectiveSpec::SubmarinesSunk { points } => Objective::SubmarinesSunk {
                    points: points_of(points, &format!("{what} points"))?,
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
        for_play: spec.for_play.unwrap_or(false),
        // Filled by `World::set_objectives` from the scenario's bases.
        base_hexes: BTreeMap::new(),
    })
}

/// The note an entry rests on, or why it is wrong: a placeholder needs one,
/// and a note given is not empty (decision 0173, 5).
fn entry_note(
    placeholder: bool,
    note: &Option<String>,
    what: &str,
) -> Result<Option<String>, String> {
    match note {
        Some(text) if text.trim().is_empty() => Err(format!("{what} has an empty note")),
        Some(text) => Ok(Some(text.clone())),
        None if placeholder => Err(format!(
            "{what} is a placeholder and has no note of what it rests on"
        )),
        None => Ok(None),
    }
}

/// A believed force as the scenario states it, checked (decision 0173, 5,
/// 6 and 9) against the world as loaded: its base is an enemy base on the
/// believing side's map, its hex is on the map, its day no later than the
/// last.
fn believed_force(
    spec: &BelievedForceSpec,
    index: usize,
    days: u32,
    world: &World,
) -> Result<BelievedForce, String> {
    let side: Side = spec.side.into();
    let what = format!("believed force {} ({})", index + 1, spec.name);
    let kind = parse_kind(&spec.kind).ok_or_else(|| {
        format!(
            "{what} has unknown kind \"{}\": carrier, battleship, cruiser, destroyer, transport or auxiliary",
            spec.kind
        )
    })?;
    if spec.ships == 0 {
        return Err(format!(
            "{what} has no ship: it is believed to have 1 or more"
        ));
    }
    let standing = Standing::from_name(&spec.standing).ok_or_else(|| {
        format!(
            "{what} has unknown standing \"{}\": present, absent or unknown",
            spec.standing
        )
    })?;
    let day = spec.day.unwrap_or(0);
    if day > days {
        return Err(format!(
            "{what} was known there on day {day}, after the scenario's last, day {days}"
        ));
    }
    let last_known = match (spec.at_base, spec.q, spec.r) {
        (Some(_), Some(_), _) | (Some(_), _, Some(_)) => {
            return Err(format!(
                "{what} has both \"at_base\" and a hex; it takes one or neither"
            ))
        }
        (Some(raw), None, None) => {
            let id = BaseId::new(raw);
            let base = world.umpire().bases.get(&id).ok_or_else(|| {
                format!("{what} is at base {raw}, which is not a base of the scenario")
            })?;
            if base.side == side {
                return Err(format!(
                    "{what} is at base {raw} ({}), which is the {} side's own: a believed force is the enemy's",
                    base.name,
                    side.name()
                ));
            }
            if !base.on_map {
                return Err(format!(
                    "{what} is at base {raw} ({}), which is not on the {} side's map",
                    base.name,
                    side.name()
                ));
            }
            LastKnown::AtBase {
                base: id,
                hex: base.hex,
                day,
            }
        }
        (None, Some(q), Some(r)) => {
            let hex = Hex::new(q, r);
            if !world.map().contains(hex) {
                return Err(format!("{what} is at {q},{r}, off the map"));
            }
            LastKnown::AtHex { hex, day }
        }
        (None, Some(_), None) | (None, None, Some(_)) => {
            return Err(format!(
                "{what} gives one of \"q\" and \"r\"; a hex takes both"
            ))
        }
        (None, None, None) => {
            if spec.day.is_some() {
                return Err(format!(
                    "{what} gives a day and no place; the day is the day it was known at a place"
                ));
            }
            LastKnown::NotKnown
        }
    };
    let placeholder = spec.placeholder.unwrap_or(false);
    Ok(BelievedForce {
        side,
        name: spec.name.clone(),
        kind,
        ships: spec.ships,
        standing,
        last_known,
        placeholder,
        note: entry_note(placeholder, &spec.note, &what)?,
    })
}

/// Believed men as the scenario states them, checked (decision 0176, 1 and
/// amended 7) as a believed force is, against the world as loaded: a place,
/// a base of the enemy's on the believing side's map or a hex of the map that
/// is not sea; 1 or more men; a known standing.
fn believed_men(
    spec: &BelievedMenSpec,
    index: usize,
    world: &World,
) -> Result<BelievedMen, String> {
    let side: Side = spec.side.into();
    let what = format!("believed men {} ({})", index + 1, spec.name);
    if spec.men == 0 {
        return Err(format!("{what} have no man: they are 1 or more"));
    }
    let standing = Standing::from_name(&spec.standing).ok_or_else(|| {
        format!(
            "{what} have unknown standing \"{}\": present, absent or unknown",
            spec.standing
        )
    })?;
    let (hex, base) = match (spec.at_base, spec.q, spec.r) {
        (Some(_), Some(_), _) | (Some(_), _, Some(_)) => {
            return Err(format!(
                "{what} have both \"at_base\" and a hex; they take one"
            ))
        }
        (Some(raw), None, None) => {
            let id = BaseId::new(raw);
            let base = world.umpire().bases.get(&id).ok_or_else(|| {
                format!("{what} are at base {raw}, which is not a base of the scenario")
            })?;
            if base.side == side {
                return Err(format!(
                    "{what} are at base {raw} ({}), which is the {} side's own: believed men are the enemy's",
                    base.name,
                    side.name()
                ));
            }
            if !base.on_map {
                return Err(format!(
                    "{what} are at base {raw} ({}), which is not on the {} side's map",
                    base.name,
                    side.name()
                ));
            }
            (base.hex, Some(id))
        }
        (None, Some(q), Some(r)) => {
            let hex = Hex::new(q, r);
            let Some(data) = world.map().get(hex) else {
                return Err(format!("{what} are at {q},{r}, off the map"));
            };
            if !matches!(data.kind, HexKind::Land | HexKind::Coast) {
                return Err(format!(
                    "{what} are at {q},{r}, which is sea: men are ashore"
                ));
            }
            (hex, None)
        }
        (None, Some(_), None) | (None, None, Some(_)) => {
            return Err(format!(
                "{what} give one of \"q\" and \"r\"; a hex takes both"
            ))
        }
        (None, None, None) => {
            return Err(format!(
                "{what} have no place: they take \"at_base\" or \"q\" and \"r\""
            ))
        }
    };
    let placeholder = spec.placeholder.unwrap_or(false);
    Ok(BelievedMen {
        side,
        name: spec.name.clone(),
        hex,
        base,
        men: spec.men,
        standing,
        placeholder,
        note: entry_note(placeholder, &spec.note, &what)?,
    })
}

/// A believed intent as the scenario states it, checked (decision 0173, 7
/// and 9) as a `set_intent` command of its kind would be for the enemy's
/// side, and every base it names on the believing side's map.
fn believed_intent(
    spec: &BelievedIntentSpec,
    index: usize,
    world: &World,
) -> Result<BelievedIntent, String> {
    let side: Side = spec.side.into();
    let what = format!("believed intent {} of the {} side", index + 1, side.name());
    let kind = spec.intent_kind(&what)?;
    let enemy = match side {
        Side::Allied => Side::Japanese,
        Side::Japanese => Side::Allied,
    };
    // Every base it names is on the believing side's map, read from that
    // side's own view: one of its own bases, or an enemy base it sees.
    let view = world.side_view(side);
    for raw in [kind.own_base(), kind.target()].into_iter().flatten() {
        let on_map = view.bases().any(|base| base.id == raw)
            || view.enemy_bases().any(|base| base.id == raw);
        if !on_map {
            return Err(format!(
                "{what} names base {}, which is not on the {} side's map",
                raw.raw(),
                side.name()
            ));
        }
    }
    // A kind that names a task force is refused before this (the
    // architect's ruling on reading 10). For the kinds left, the command's
    // check reads, of the enemy, only the side of a base it names, which the
    // believing side's map shows (checked above); of the believing side, the
    // `on_map` of its own base (a hold or target of it); and the map.
    debug_assert!(kind.force().is_none());
    world.check_root_intent(enemy, &kind).map_err(|refusal| {
        format!(
            "{what} could not be given to the {} side: {refusal}",
            enemy.name()
        )
    })?;
    let placeholder = spec.placeholder.unwrap_or(false);
    Ok(BelievedIntent {
        side,
        kind,
        placeholder,
        note: entry_note(placeholder, &spec.note, &what)?,
    })
}

/// The orders a scenario's script gives inside a day (decision 0156), by day,
/// each with the pulse it is handed in for (morning or afternoon) and its
/// side.
pub type ScriptInsideDay = BTreeMap<u32, Vec<(Pulse, Side, OrderRequest)>>;

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
    /// Whether a side may give orders inside a day (decision 0156): the
    /// scenario's `orders_inside_a_day`, off when absent.
    orders_inside_a_day: bool,
    /// The orders still to be given inside a day, by day, each with the
    /// pulse it is handed in for (morning or afternoon), in the order the
    /// file gives them (decision 0156). Empty where the key is off.
    script_inside_day: ScriptInsideDay,
    /// The commander's commands still to be given, by day, as they will be
    /// submitted (decision 0107, item 16).
    commands: BTreeMap<u32, Vec<(Side, CommandKind)>>,
    /// The contacts the sides start with, each with the command that saw
    /// it if the scenario names one (decision 0137, 6), kept so that a world
    /// built afresh for another seed holds them too.
    contacts: Vec<(StartingContact, Option<CommandRef>)>,
    /// The opening estimates, kept for the same reason (decision 0092, 16).
    estimates: Vec<StartingEstimate>,
    /// The suspected waters the scenario gives, kept for the same reason
    /// (decision 0218, 47).
    waters: Vec<WatersWaiting>,
    /// The superiors the scenario names, when command levels are on
    /// (decision 0134); `None` when they are off.
    command_levels: Option<Vec<NamedSuperior>>,
    /// The task forces that start on radio silence (decision 0136, 15), kept
    /// so that a world built afresh for another seed has them too.
    radio_silent: Vec<TaskForceId>,
    /// The commanders the scenario names (decision 0137), kept for the same
    /// reason. Held in the world only with command levels on.
    commanders: Vec<NamedCommander>,
    /// Every file the scenario names, as the paths it reads them from, in a
    /// fixed order (decision 0043, 9).
    data_files: Vec<PathBuf>,
    /// The figures of each class that rest on an estimate, from the class
    /// files' `estimated` column (decision 0207, rule 10): never hashed and
    /// read by no rule.
    estimated_figures: crate::reference::EstimatedFigures,
    /// The enemy forces each side believed in (decision 0173, 5): the
    /// scenario's, not the world's, never hashed and read by no rule.
    believed_forces: Vec<BelievedForce>,
    /// The intents each side believed the enemy held (decision 0173, 7),
    /// kept as the forces are.
    believed_intents: Vec<BelievedIntent>,
    /// The men of the enemy's each side believed may be ashore at a place
    /// beyond its estimate (decision 0176, 1), kept as the forces are.
    believed_men: Vec<BelievedMen>,
    /// A trial's figures (decision 0174), where the scenario names them:
    /// the scenario's, not the world's, never hashed and read by no rule.
    trial_figures: Option<TrialFigures>,
    /// The frame of the map the scenario is played on, from the map's header
    /// (decision 0199, 18): `None` for a hand-written map. The tools' and
    /// the export's, never the world's, never hashed and read by no rule.
    map_frame: Option<MapFrame>,
}

/// How a scenario is loaded, beyond the file itself.
///
/// The default reads every file where the scenario names it.
#[derive(Clone, Debug, Default)]
pub struct LoadOptions {
    /// A directory to read the scenario's reference (figures) files from, in
    /// place of the places it names. Each reference file the scenario names
    /// (ship classes, port sizes, aircraft types, and the search, surface
    /// combat, air combat, air upkeep, fatigue and base supply figures, the
    /// file of sides of decision 0173 and a trial's figures of decision 0174)
    /// is
    /// looked up by its file name directly under this directory. A file that
    /// is not there is an error that names it: there is no fallback to the
    /// file the scenario names, which would defeat the purpose. The map is
    /// not a reference file and is read where the scenario names it, and so
    /// are the report figures (decision 0135), which the frozen step 1
    /// figures do not have.
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
        let map_frame = read_map_frame(&folder.join(&file.map))?;
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
        let mut estimated_figures = crate::reference::EstimatedFigures::new();
        for name in &class_files {
            let file_path = reference(name)?;
            let (classes, marked) = load_ship_classes_marked(&file_path)?;
            estimated_figures.extend(marked);
            for (id, class) in classes.ship_classes {
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
        let reports_file = file.reports.as_ref().map(|name| folder.join(name));
        if let Some(reports) = &reports_file {
            ref_data.reports = Some(load_report_figures(reports)?);
        }
        // The file of sides (decision 0173, 1 and 2): whole and true of the
        // reference files. Whether a ship or an air group is of a class or
        // type given to the other side alone is checked once they are built.
        let class_sides_file = match &file.class_sides {
            Some(name) => Some(reference(name)?),
            None => None,
        };
        if let Some(sides_path) = &class_sides_file {
            let sides = load_class_sides(sides_path)?;
            let shown = sides_path.display();
            for id in ref_data.ship_classes.keys() {
                if !sides.ship_classes.contains_key(id) {
                    return Err(fail(format!(
                        "ship class {} has no row in the file of sides {shown}",
                        id.raw()
                    )));
                }
            }
            for id in sides.ship_classes.keys() {
                if !ref_data.ship_classes.contains_key(id) {
                    return Err(fail(format!(
                        "the file of sides {shown} gives ship class {}, which the scenario's class files do not give",
                        id.raw()
                    )));
                }
            }
            for id in ref_data.aircraft_types.keys() {
                if !sides.aircraft_types.contains_key(id) {
                    return Err(fail(format!(
                        "aircraft type {} has no row in the file of sides {shown}",
                        id.raw()
                    )));
                }
            }
            for id in sides.aircraft_types.keys() {
                if !ref_data.aircraft_types.contains_key(id) {
                    return Err(fail(format!(
                        "the file of sides {shown} gives aircraft type {}, which the scenario's type files do not give",
                        id.raw()
                    )));
                }
            }
            ref_data.class_sides = Some(sides);
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
        data_files.extend(reports_file);
        data_files.extend(class_sides_file);
        // A trial's figures (decision 0174): kept with the scenario, never
        // put into the world. Named last, so that the files of a scenario
        // that names none are listed as they were.
        let trial_file = match &file.trial {
            Some(name) => Some(reference(name)?),
            None => None,
        };
        let trial_figures = match &trial_file {
            Some(path) => Some(load_trial_figures(path)?),
            None => None,
        };
        data_files.extend(trial_file);

        // The submarine files (decision 0218): all three or none. The class
        // and torpedo files are reference files; the figures are read where
        // the scenario names them.
        match (
            &file.submarine_classes,
            &file.submarine_torpedoes,
            &file.submarines,
        ) {
            (None, None, None) => {
                if !file.submarine.is_empty() {
                    return Err(fail(
                        "there are submarines but the scenario names no submarine_classes, submarine_torpedoes and submarines files (decision 0218)"
                            .to_string(),
                    ));
                }
            }
            (Some(classes), Some(torpedoes), Some(figures)) => {
                let classes_path = reference(classes)?;
                let torpedoes_path = reference(torpedoes)?;
                let figures_path = folder.join(figures);
                let data = SubmarineData {
                    classes: load_submarine_classes(&classes_path)?,
                    torpedoes: load_submarine_torpedoes(&torpedoes_path)?,
                    figures: load_submarine_figures(&figures_path)?,
                };
                data.validate().map_err(|message| {
                    fail(format!("the submarine files do not fit together: {message}"))
                })?;
                ref_data.submarines = Some(data);
                data_files.push(classes_path);
                data_files.push(torpedoes_path);
                data_files.push(figures_path);
            }
            _ => {
                return Err(fail(
                    "a scenario names all of submarine_classes, submarine_torpedoes and submarines, or none of them (decision 0218)"
                        .to_string(),
                ))
            }
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
                supply_reserve_t: match &spec.supply_reserve_t {
                    Some(quantity) => {
                        let reserve = quantity
                            .to_fixed()
                            .map_err(|e| fail(format!("{id}: supply_reserve_t {e}")))?;
                        if reserve < Fixed::ZERO {
                            return Err(fail(format!("{id}: supply_reserve_t is negative")));
                        }
                        reserve
                    }
                    None => Fixed::ZERO,
                },
                // No men's rations landed yet (decision 0205, 5).
                rations_landed: false,
                ammunition_t: match &spec.ammunition_t {
                    Some(quantity) => {
                        let tons = quantity
                            .to_fixed()
                            .map_err(|e| fail(format!("{id}: ammunition_t {e}")))?;
                        if tons < Fixed::ZERO {
                            return Err(fail(format!("{id}: ammunition_t is negative")));
                        }
                        tons
                    }
                    None => Fixed::ZERO,
                },
                fuel_reserve_t: match &spec.fuel_reserve_t {
                    Some(quantity) => {
                        let reserve = quantity
                            .to_fixed()
                            .map_err(|e| fail(format!("{id}: fuel_reserve_t {e}")))?;
                        if reserve < Fixed::ZERO {
                            return Err(fail(format!("{id}: fuel_reserve_t is negative")));
                        }
                        reserve
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
            if let Some(stance) = spec.stance {
                task_force.rules_of_engagement.stance = stance.into();
            }
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
            task_force.available_from = available_day(spec.available_from, id).map_err(&fail)?;
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
            let tubes_fired = spec.tubes_fired.unwrap_or(0);
            if tubes_fired > ship_class.torpedo_tubes {
                return Err(fail(format!(
                    "{id}: tubes_fired {tubes_fired} is more than its class's {} tubes",
                    ship_class.torpedo_tubes
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
                torpedoes_ready: tubes_fired < ship_class.torpedo_tubes,
                torpedoes_one_side_fired: false,
                torpedoes_fired: tubes_fired,
                systems,
                experience,
                system_repair_points: Fixed::ZERO,
                troops: None,
                available_from: available_day(spec.available_from, id).map_err(&fail)?,
                aircraft_fire: None,
            };
            if truth.ships.insert(id, ship).is_some() {
                return Err(fail(format!("{id} appears twice")));
            }
        }

        // Formations come before land units, which name them (decision
        // 0140, 7).
        truth.formations = formations(&file.formation).map_err(&fail)?;

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
        // Each unit's establishment, kept below only where the scenario has
        // a stream of men or names one (decision 0197, 1).
        let mut establishments: BTreeMap<LandUnitId, u32> = BTreeMap::new();
        let mut named_establishment = false;
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
            let establishment = spec.establishment.unwrap_or(spec.men);
            if establishment < spec.men {
                return Err(fail(format!(
                    "{id} has an establishment of {establishment}, below its {} men",
                    spec.men
                )));
            }
            establishments.insert(id, establishment);
            named_establishment |= spec.establishment.is_some();
            let side: Side = spec.side.into();
            let mut unit = LandUnit::new(id, spec.name.clone(), side, 0, None);
            unit.available_from = available_day(spec.available_from, id).map_err(&fail)?;
            if let Some(raw) = spec.formation {
                let formation = FormationId::new(raw);
                match truth.formations.get(&formation) {
                    None => {
                        return Err(fail(format!(
                            "{id} belongs to {formation}, which is not there"
                        )))
                    }
                    Some(found) if found.side != side => {
                        return Err(fail(format!(
                            "{id} is {} but its formation {formation} is {}",
                            side.name(),
                            found.side.name()
                        )))
                    }
                    Some(_) => unit.formation = Some(formation),
                }
            }
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
            // The count it starts with (decision 0205, 7).
            unit.hunger_nights = spec.hunger_nights.unwrap_or(0);
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
                strike_contact_age: match spec.strike_contact_age_pulses {
                    Some(pulses) => {
                        // Decision 0114, 3: from 0 up to one less than the
                        // age at which a contact is dropped, where the
                        // search figures are loaded.
                        let max = ref_data
                            .search
                            .as_ref()
                            .map(|search| search.contact_drop_age_pulses.saturating_sub(1));
                        if pulses < 0 || max.is_some_and(|max| pulses > i64::from(max)) {
                            let range = match max {
                                Some(max) => format!("between 0 and {max}"),
                                None => "0 or more".to_string(),
                            };
                            return Err(fail(format!(
                                "{id} has a strike_contact_age_pulses of {pulses}, not {range}"
                            )));
                        }
                        Some(u32::try_from(pulses).expect("the age was checked"))
                    }
                    None => None,
                },
                strike_age_order: None,
                strike_held_share: match &spec.strike_held_share {
                    Some(quantity) => {
                        let share = quantity
                            .to_fixed()
                            .map_err(|e| fail(format!("{id} strike_held_share {e}")))?;
                        if share < Fixed::ZERO || share > Fixed::ONE {
                            return Err(fail(format!(
                                "{id} has a strike_held_share of {share}, not between 0 and 1"
                            )));
                        }
                        share
                    }
                    None => Fixed::ZERO,
                },
                strike_share_order: None,
                strike_sizing: false,
                strike_sizing_order: None,
                search_once: false,
                search_once_order: None,
                available_from: available_day(spec.available_from, id).map_err(&fail)?,
                strike_first_waters: std::collections::BTreeSet::new(),
                waters_order: None,
                // Nothing claimed at the start of the game (decision 0180, 6).
                claims: sim_core::state::ClaimTally::default(),
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

        // Supply lines come after bases are known (decision 0196).
        for (index, spec) in file.supply_line.iter().enumerate() {
            let number = index + 1;
            let base_id = BaseId::new(spec.base);
            let base = truth.bases.get(&base_id).ok_or_else(|| {
                fail(format!(
                    "supply line {number} is at unknown base {}",
                    spec.base
                ))
            })?;
            let side: Side = spec.side.map_or(base.side, Into::into);
            let tons = |q: &Option<Quantity>, what: &str| -> Result<Fixed, DataError> {
                let t = match q {
                    None => Fixed::ZERO,
                    Some(q) => q
                        .to_fixed()
                        .map_err(|e| fail(format!("supply line {number} {what}: {e}")))?,
                };
                if t < Fixed::ZERO {
                    return Err(fail(format!("supply line {number} has {what} below 0")));
                }
                Ok(t)
            };
            let supply_t = tons(&spec.supply_t, "supply_t")?;
            let fuel_t = tons(&spec.fuel_t, "fuel_t")?;
            if supply_t == Fixed::ZERO && fuel_t == Fixed::ZERO {
                return Err(fail(format!(
                    "supply line {number} gives nothing: neither supply nor fuel is above 0"
                )));
            }
            let first_day = spec.first_day.unwrap_or(1);
            let last_day = spec.last_day.unwrap_or(file.days);
            if first_day < 1 {
                return Err(fail(format!(
                    "supply line {number} has a first day below 1"
                )));
            }
            if first_day > file.days || last_day > file.days {
                return Err(fail(format!(
                    "supply line {number} has days {first_day} to {last_day}, past the game's last day {}",
                    file.days
                )));
            }
            if last_day < first_day {
                return Err(fail(format!(
                    "supply line {number} has its last day {last_day} before its first day {first_day}"
                )));
            }
            truth.supply_lines.push(sim_core::state::SupplyLine {
                side,
                base: base_id,
                supply_t,
                fuel_t,
                first_day,
                last_day,
            });
        }

        // Streams of replacement men come after bases are known (decision
        // 0197, 2).
        for (index, spec) in file.men_replacement.iter().enumerate() {
            let number = index + 1;
            let base_id = BaseId::new(spec.base);
            let base = truth.bases.get(&base_id).ok_or_else(|| {
                fail(format!(
                    "men replacement {number} is at unknown base {}",
                    spec.base
                ))
            })?;
            let side: Side = spec.side.map_or(base.side, Into::into);
            if spec.men == 0 {
                return Err(fail(format!(
                    "men replacement {number} gives no men: men must be above 0"
                )));
            }
            let first_day = spec.first_day.unwrap_or(1);
            let last_day = spec.last_day.unwrap_or(file.days);
            if first_day < 1 {
                return Err(fail(format!(
                    "men replacement {number} has a first day below 1"
                )));
            }
            if first_day > file.days || last_day > file.days {
                return Err(fail(format!(
                    "men replacement {number} has days {first_day} to {last_day}, past the game's last day {}",
                    file.days
                )));
            }
            if last_day < first_day {
                return Err(fail(format!(
                    "men replacement {number} has its last day {last_day} before its first day {first_day}"
                )));
            }
            truth
                .men_replacement
                .streams
                .push(sim_core::state::MenStream {
                    side,
                    base: base_id,
                    men_per_day: spec.men,
                    first_day,
                    last_day,
                });
        }
        if !truth.men_replacement.streams.is_empty() || named_establishment {
            truth.men_replacement.units = establishments
                .into_iter()
                .map(|(id, establishment)| {
                    (
                        id,
                        sim_core::state::Strength {
                            establishment,
                            joined: 0,
                            last_fought: None,
                        },
                    )
                })
                .collect();
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

        let orders_inside_a_day = file.orders_inside_a_day == Some(true);
        let mut script: BTreeMap<u32, Vec<(Side, OrderRequest)>> = BTreeMap::new();
        let mut script_inside_day = ScriptInsideDay::new();
        for spec in &file.order {
            if spec.day == 0 || spec.day > file.days {
                return Err(fail(format!(
                    "an order is set for day {}, outside days 1 to {}",
                    spec.day, file.days
                )));
            }
            // An order given inside a day (decision 0156): only where the
            // scenario says, and only at 0600 or 1200.
            let pulse = match spec.pulse.as_deref() {
                None => None,
                Some(_) if !orders_inside_a_day => {
                    return Err(fail(format!(
                        "the order on day {} has a pulse, and orders inside a day need orders_inside_a_day = true at the top of the scenario",
                        spec.day
                    )))
                }
                Some("morning") => Some(Pulse::Morning),
                Some("afternoon") => Some(Pulse::Afternoon),
                Some(other) => {
                    return Err(fail(format!(
                        "the order on day {} has pulse \"{other}\": an order inside a day is given for the \"morning\" (at 0600) or the \"afternoon\" (at 1200)",
                        spec.day
                    )))
                }
            };
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
            // `base` names the target of a base order, but for a home port
            // or a rebase it is the base the order names, not the target.
            let names_a_base = matches!(
                spec.action,
                ActionSpec::SetHomePort {} | ActionSpec::Rebase {}
            );
            let (target_base, named_base) = if names_a_base {
                (None, spec.base)
            } else {
                (spec.base, None)
            };
            let need_base = |kind: &str| {
                named_base
                    .ok_or_else(|| fail(format!("the {kind} order on day {day} needs a base")))
            };
            let kind = match (
                &spec.task_force,
                &spec.air_group,
                &spec.land_unit,
                &target_base,
                &spec.submarine,
            ) {
                (Some(raw), None, None, None, None) => {
                    let order = match &spec.action {
                        ActionSpec::SetRoute {
                            waypoints,
                            act_at,
                            stand_at,
                        } => TaskForceOrder::SetRoute {
                            waypoints: waypoints.iter().map(|[q, r]| Hex::new(*q, *r)).collect(),
                            act_at: act_at.iter().map(|[q, r]| Hex::new(*q, *r)).collect(),
                            stand_at: stand_at.iter().map(|[q, r]| Hex::new(*q, *r)).collect(),
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
                        ActionSpec::Load {
                            cargo,
                            unit,
                            weight_t: Some(weight),
                            each_t,
                        } => {
                            // A weight does not apply to men (decision 5 of
                            // 0111). Its sign is for intake to refuse.
                            if unit.is_some() {
                                return Err(fail(format!(
                                    "the load order on day {day} gives a weight_t and a unit: a weight does not apply to men"
                                )));
                            }
                            let cargo = match cargo {
                                None | Some(CargoSpec::Supply) => Cargo::Supply,
                                Some(CargoSpec::Fuel) => Cargo::Fuel,
                                Some(CargoSpec::NoCargo) => {
                                    return Err(fail(format!(
                                        "the load order on day {day} gives a weight_t with cargo none: a weight needs supply or fuel"
                                    )))
                                }
                            };
                            // The level is refused at 0 or below as it is
                            // read (decision 0162, 3).
                            let each_t = match each_t {
                                Some(level) => {
                                    let level = level
                                        .to_fixed()
                                        .map_err(|e| fail(format!("each_t on day {day} {e}")))?;
                                    if level <= Fixed::ZERO {
                                        return Err(fail(format!(
                                            "the load order on day {day} gives an each_t of {level}: the most a ship is to hold must be above 0"
                                        )));
                                    }
                                    Some(level)
                                }
                                None => None,
                            };
                            TaskForceOrder::LoadTo {
                                cargo,
                                weight_t: weight
                                    .to_fixed()
                                    .map_err(|e| fail(format!("weight_t on day {day} {e}")))?,
                                each_t,
                            }
                        }
                        ActionSpec::Load {
                            each_t: Some(_), ..
                        } => {
                            return Err(fail(format!(
                                "the load order on day {day} gives an each_t and no weight_t: the most a ship is to hold goes with a weight"
                            )))
                        }
                        ActionSpec::Load {
                            cargo,
                            unit,
                            weight_t: None,
                            each_t: None,
                        } => match (unit, cargo) {
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
                        ActionSpec::Join { into } => {
                            TaskForceOrder::JoinTo(TaskForceId::new(*into))
                        }
                        ActionSpec::StandDown {} => TaskForceOrder::StandDown,
                        ActionSpec::NoCargoWork {} => TaskForceOrder::NoCargoWork,
                        ActionSpec::FuelFrom { giver } => {
                            TaskForceOrder::FuelFrom(TaskForceId::new(*giver))
                        }
                        ActionSpec::Escort { charge } => {
                            TaskForceOrder::Escort(charge.map(TaskForceId::new))
                        }
                        ActionSpec::FormTaskForce { .. } => {
                            return Err(wrong_target("form_task_force", "base"))
                        }
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
                        ActionSpec::SetHomePort {} => {
                            TaskForceOrder::SetHomePort(BaseId::new(need_base("set_home_port")?))
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
                        ActionSpec::SetStrikeContactAge { .. } => {
                            return Err(wrong_target("set_strike_contact_age", "air_group"))
                        }
                        ActionSpec::SetStrikeHeldShare { .. } => {
                            return Err(wrong_target("set_strike_held_share", "air_group"))
                        }
                        ActionSpec::SetStrikeSizing { .. } => {
                            return Err(wrong_target("set_strike_sizing", "air_group"))
                        }
                        ActionSpec::SetSearchOnce { .. } => {
                            return Err(wrong_target("set_search_once", "air_group"))
                        }
                        ActionSpec::SetStrikeFirstWaters { .. } => {
                            return Err(wrong_target("set_strike_first_waters", "air_group"))
                        }
                        ActionSpec::Rebase {} => {
                            return Err(wrong_target("rebase", "air_group"))
                        }
                        ActionSpec::SetPosture { .. } => {
                            return Err(wrong_target("set_posture", "land_unit"))
                        }
                        ActionSpec::March { .. } => return Err(wrong_target("march", "land_unit")),
                        ActionSpec::SetActivity { .. } => {
                            return Err(wrong_target("set_activity", "land_unit"))
                        }
                        ActionSpec::Station { .. } => {
                            return Err(wrong_target("station", "submarine"))
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
                (None, Some(raw), None, None, None) => {
                    let order = match &spec.action {
                        ActionSpec::SetAirMission {
                            mission,
                            target_base,
                            hex,
                        } => {
                            let mission = match (mission, target_base) {
                                (AirMissionSpec::Reconnaissance, None) => match hex {
                                    Some([q, r]) => AirMission::Reconnaissance(Hex::new(*q, *r)),
                                    None => {
                                        return Err(fail(format!(
                                            "the reconnaissance order on day {day} needs a hex"
                                        )))
                                    }
                                },
                                (_, None) if hex.is_some() => {
                                    return Err(fail(format!(
                                        "hex on day {day} is only for the reconnaissance mission"
                                    )))
                                }
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
                        ActionSpec::SetStrikeContactAge { pulses } => {
                            AirGroupOrder::SetStrikeContactAge { pulses: *pulses }
                        }
                        ActionSpec::SetStrikeHeldShare { share } => {
                            AirGroupOrder::SetStrikeHeldShare {
                                share: share
                                    .to_fixed()
                                    .map_err(|e| fail(format!("share on day {day} {e}")))?,
                            }
                        }
                        ActionSpec::SetStrikeSizing { on } => {
                            AirGroupOrder::SetStrikeSizing { on: *on }
                        }
                        ActionSpec::SetSearchOnce { on } => {
                            AirGroupOrder::SetSearchOnce { on: *on }
                        }
                        ActionSpec::SetStrikeFirstWaters { hexes } => {
                            AirGroupOrder::SetStrikeFirstWaters {
                                hexes: hexes.iter().map(|[q, r]| Hex::new(*q, *r)).collect(),
                            }
                        }
                        ActionSpec::Rebase {} => AirGroupOrder::Rebase {
                            base: BaseId::new(need_base("rebase")?),
                        },
                        _ => {
                            return Err(fail(format!(
                                "an order on day {day} for an air_group must be set_air_mission, set_search_arc, set_patrol_share, set_rest_threshold, set_reach, set_target_priority, set_strike_contact_age, set_strike_held_share, set_strike_sizing, set_search_once, set_strike_first_waters or rebase"
                            )))
                        }
                    };
                    OrderKind::AirGroup {
                        target: AirGroupId::new(*raw),
                        order,
                    }
                }
                (None, None, Some(raw), None, None) => {
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
                (None, None, None, Some(raw), None) => {
                    let order = match &spec.action {
                        ActionSpec::FormTaskForce { ships } => BaseOrder::FormTaskForce {
                            ships: ships.iter().map(|raw| ShipId::new(*raw)).collect(),
                        },
                        _ => {
                            return Err(fail(format!(
                                "an order on day {day} for a base must be form_task_force"
                            )))
                        }
                    };
                    OrderKind::Base {
                        target: BaseId::new(*raw),
                        order,
                    }
                }
                (None, None, None, None, Some(raw)) => {
                    let order = match &spec.action {
                        ActionSpec::Station { hex: [q, r] } => {
                            SubmarineOrder::Station(Hex::new(*q, *r))
                        }
                        _ => {
                            return Err(fail(format!(
                                "an order on day {day} for a submarine must be station"
                            )))
                        }
                    };
                    OrderKind::Submarine {
                        target: SubmarineId::new(*raw),
                        order,
                    }
                }
                _ => {
                    return Err(fail(format!(
                        "the order on day {day} needs exactly one of task_force, air_group, land_unit and base (or submarine, for a station order)"
                    )))
                }
            };
            let request = OrderRequest { kind, origin };
            match pulse {
                None => script
                    .entry(spec.day)
                    .or_default()
                    .push((spec.side.into(), request)),
                Some(pulse) => script_inside_day.entry(spec.day).or_default().push((
                    pulse,
                    spec.side.into(),
                    request,
                )),
            }
        }

        let mut commands: BTreeMap<u32, Vec<(Side, CommandKind)>> = BTreeMap::new();
        for spec in &file.command {
            let (day, side, kind) = spec.build(file.days).map_err(&fail)?;
            commands.entry(day).or_default().push((side, kind));
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
        truth
            .supply_lines
            .sort_by_key(sim_core::state::SupplyLine::order_key);
        truth
            .men_replacement
            .streams
            .sort_by_key(sim_core::state::MenStream::order_key);

        // Islands are map data (decision 0151, 1); they are added before the
        // world is built, so both sides' maps have them.
        let mut islands_seen: BTreeSet<u32> = BTreeSet::new();
        for spec in &file.island {
            if !islands_seen.insert(spec.number) {
                return Err(fail(format!("island {} appears twice", spec.number)));
            }
            if spec.places.is_empty() {
                return Err(fail(format!("island {} has no place", spec.number)));
            }
            let mut places = Vec::new();
            for (index, place) in spec.places.iter().enumerate() {
                let what = format!("island {} place {}", spec.number, index + 1);
                match (place.base, place.hex) {
                    (Some(raw), None) => {
                        let base_id = BaseId::new(raw);
                        let base = truth.bases.get(&base_id).ok_or_else(|| {
                            fail(format!(
                                "{what} names {base_id}, which is not a base of the scenario"
                            ))
                        })?;
                        places.push(IslandPlace {
                            base: Some(base_id),
                            hex: base.hex,
                        });
                    }
                    (None, Some([q, r])) => {
                        let hex = Hex::new(q, r);
                        let data = map
                            .get(hex)
                            .ok_or_else(|| fail(format!("{what} is at {q},{r}, off the map")))?;
                        if data.kind != HexKind::Coast {
                            return Err(fail(format!("{what} is at {q},{r}, which is not coast")));
                        }
                        places.push(IslandPlace { base: None, hex });
                    }
                    (Some(_), Some(_)) => {
                        return Err(fail(format!(
                            "{what} has both \"base\" and \"hex\"; it takes exactly one"
                        )));
                    }
                    (None, None) => {
                        return Err(fail(format!(
                            "{what} has neither \"base\" nor \"hex\"; it takes exactly one"
                        )));
                    }
                }
            }
            map.add_island(Island {
                number: spec.number,
                name: spec.name.clone(),
                places,
            });
        }

        // No ship or air group is of a class or type the file of sides gives
        // to the other side alone (decision 0173, 2).
        if let Some(sides) = &ref_data.class_sides {
            for ship in truth.ships.values() {
                if let Some(side) = sides.ship_classes.get(&ship.class) {
                    if !side.used_by(ship.side) {
                        return Err(fail(format!(
                            "ship {} ({}) is {} and of ship class {}, which the file of sides gives to the {} side alone",
                            ship.id.raw(),
                            ship.name,
                            ship.side.name(),
                            ship.class.raw(),
                            side.name()
                        )));
                    }
                }
            }
            for group in truth.air_groups.values() {
                if let Some(side) = sides.aircraft_types.get(&group.aircraft_type) {
                    if !side.used_by(group.side) {
                        return Err(fail(format!(
                            "air group {} ({}) is {} and of aircraft type {}, which the file of sides gives to the {} side alone",
                            group.id.raw(),
                            group.name,
                            group.side.name(),
                            group.aircraft_type.raw(),
                            side.name()
                        )));
                    }
                }
            }
        }

        // Submarines (decision 0218): built against the data and the bases;
        // the world's own checks then see that each is whole.
        for spec in &file.submarine {
            let id = SubmarineId::new(spec.id);
            if truth.submarines.contains_key(&id) {
                return Err(fail(format!("submarine {} is given twice", spec.id)));
            }
            let what = format!("submarine {}", spec.id);
            let class_id = SubmarineClassId::new(spec.class);
            let class = ref_data
                .submarines
                .as_ref()
                .and_then(|data| data.classes.get(&class_id))
                .ok_or_else(|| {
                    fail(format!(
                        "{what} has class {}, which the submarine class file does not give",
                        spec.class
                    ))
                })?;
            let hex = Hex::new(spec.q, spec.r);
            let station = match (spec.station_q, spec.station_r) {
                (None, None) => hex,
                (Some(q), Some(r)) => Hex::new(q, r),
                _ => {
                    return Err(fail(format!(
                        "{what} gives one of station_q and station_r: give both or neither"
                    )))
                }
            };
            let salvoes = spec.salvoes.unwrap_or(class.salvoes_carried);
            if salvoes == 0 {
                return Err(fail(format!(
                    "{what} has salvoes = 0: a boat with no salvo is sent home, so a scenario gives 1 to her class's {} (decision 0218, rule 43)",
                    class.salvoes_carried
                )));
            }
            if salvoes > class.salvoes_carried {
                return Err(fail(format!(
                    "{what} has salvoes = {salvoes}, more than her class carries ({})",
                    class.salvoes_carried
                )));
            }
            let condition = match spec.condition.as_deref() {
                None | Some("sound") => SubmarineCondition::Sound,
                Some("damaged") => SubmarineCondition::Damaged,
                Some(other) => {
                    return Err(fail(format!(
                        "{what} has condition \"{other}\": sound or damaged"
                    )))
                }
            };
            for (key, day) in [
                ("leave_day", spec.leave_day),
                ("available_from", spec.available_from),
            ] {
                if day == Some(0) {
                    return Err(fail(format!("{what} has {key} = 0: days count from 1")));
                }
            }
            if let Some(day) = spec.enters {
                if day < 2 {
                    return Err(fail(format!(
                        "{what} has enters = {day}: a unit there on day 1 needs no `enters`, and a unit enters on day 2 or later (decision 0195, 1)"
                    )));
                }
                if day > file.days {
                    return Err(fail(format!(
                        "{what} has enters = {day}, after the last day of the scenario ({}): it would never enter",
                        file.days
                    )));
                }
            }
            let home = BaseId::new(spec.home);
            // A damaged boat starts going home; a boat whose station is not
            // her hex starts on passage to it (decision 0218, 43).
            let passage = match (condition, truth.bases.get(&home)) {
                (SubmarineCondition::Damaged, Some(base)) => Some(SubmarinePassage {
                    to: base.hex,
                    home: true,
                    toward_hex: None,
                    progress_nm: Fixed::ZERO,
                    leg_nm: Fixed::ZERO,
                    arrived: false,
                    order: None,
                }),
                (SubmarineCondition::Damaged, None) => None,
                _ if station != hex => Some(SubmarinePassage {
                    to: station,
                    home: false,
                    toward_hex: None,
                    progress_nm: Fixed::ZERO,
                    leg_nm: Fixed::ZERO,
                    arrived: false,
                    order: None,
                }),
                _ => None,
            };
            // Held down for that many pulses from the first, the night
            // counted as the first: the last pulse she is held is that many
            // pulses after the night of day 1, the night included.
            let held_down_until = match spec.held_down_pulses.unwrap_or(0) {
                0 => None,
                pulses => {
                    let place = usize::try_from(pulses - 1).expect("a count of pulses");
                    let day = 1 + u32::try_from(place / 3).expect("a count of days");
                    Some((day, Pulse::ALL[place % 3]))
                }
            };
            truth.submarines.insert(
                id,
                Submarine {
                    id,
                    name: spec.name.clone(),
                    side: spec.side.into(),
                    class: class_id,
                    hex,
                    station,
                    passage,
                    salvoes,
                    held_down_until,
                    condition,
                    home,
                    leave_day: spec.leave_day,
                    enters_day: spec.enters,
                    available_from: spec.available_from,
                    station_order: None,
                },
            );
        }

        // Units that enter later (decision 0195, 1, 16 to 19): checked here,
        // against the whole truth, then held apart before the world is made.
        let mut held: Vec<(EntityRef, u32)> = Vec::new();
        {
            let mut force_day: BTreeMap<u32, u32> = BTreeMap::new();
            let mut group_day: BTreeMap<u32, u32> = BTreeMap::new();
            let mut unit_day: BTreeMap<u32, u32> = BTreeMap::new();
            let check_day = |what: &str, day: u32| -> Result<(), DataError> {
                if day < 2 {
                    return Err(fail(format!(
                        "{what} has enters = {day}: a unit there on day 1 needs no `enters`, and a unit enters on day 2 or later (decision 0195, 1)"
                    )));
                }
                if day > file.days {
                    return Err(fail(format!(
                        "{what} has enters = {day}, after the last day of the scenario ({}): it would never enter",
                        file.days
                    )));
                }
                Ok(())
            };
            for spec in &file.ship {
                if spec.enters.is_some() {
                    return Err(fail(format!(
                        "ship {} has `enters`: a ship enters with its task force, so put `enters` on the force (decision 0195, 1)",
                        spec.id
                    )));
                }
            }
            for spec in &file.task_force {
                let Some(day) = spec.enters else { continue };
                let what = format!("task force {}", spec.id);
                check_day(&what, day)?;
                let id = TaskForceId::new(spec.id);
                let hex = truth.task_forces[&id].position();
                match map.get(hex) {
                    None => {
                        return Err(fail(format!(
                            "{what} enters on day {day} at {},{}, which is off the map: a force enters at a sea or coast hex (decision 0195, 17)",
                            hex.q, hex.r
                        )))
                    }
                    Some(data) if !data.is_navigable() => {
                        return Err(fail(format!(
                            "{what} enters on day {day} at {},{}, which is land: a force enters at a sea or coast hex, or at a base of its side whose hex is sea or coast (decision 0195, 17)",
                            hex.q, hex.r
                        )))
                    }
                    Some(_) => {}
                }
                force_day.insert(spec.id, day);
            }
            for spec in &file.air_group {
                let Some(day) = spec.enters else { continue };
                let what = format!("air group {}", spec.id);
                check_day(&what, day)?;
                if spec.ship.is_some() {
                    return Err(fail(format!(
                        "{what} is aboard a ship and has `enters`: it enters with its ship's task force, so put `enters` on the force (decision 0195, 1)"
                    )));
                }
                group_day.insert(spec.id, day);
            }
            for spec in &file.land_unit {
                let Some(day) = spec.enters else { continue };
                let what = format!("land unit {}", spec.id);
                check_day(&what, day)?;
                if spec.task_force.is_some() {
                    return Err(fail(format!(
                        "{what} is aboard a task force and has `enters`: it enters with its force, so put `enters` on the force (decision 0195, 1)"
                    )));
                }
                unit_day.insert(spec.id, day);
            }
            // An arrival of aircraft dated before its group is there.
            for (index, spec) in file.arrival.iter().enumerate() {
                let group = AirGroupId::new(spec.air_group);
                let entered = match group_day.get(&spec.air_group) {
                    Some(day) => Some(*day),
                    None => match truth.air_groups.get(&group).map(|g| g.location) {
                        Some(AirGroupLocation::Ship(ship)) => match truth.ships.get(&ship) {
                            Some(Ship {
                                location: ShipLocation::TaskForce(force) | ShipLocation::Yard(force),
                                ..
                            }) => force_day.get(&force.raw()).copied(),
                            _ => None,
                        },
                        _ => None,
                    },
                };
                if let Some(day) = entered {
                    if spec.day < day {
                        return Err(fail(format!(
                            "arrival {} is on day {}, before air group {} enters on day {day}: an arrival is dated on its group's day or after (decision 0195, 19)",
                            index + 1,
                            spec.day,
                            spec.air_group
                        )));
                    }
                }
            }
            // An escort and its charge enter on one day or both are there.
            for spec in &file.order {
                if let (
                    Some(escort),
                    ActionSpec::Escort {
                        charge: Some(charge),
                    },
                ) = (spec.task_force, &spec.action)
                {
                    let escort_day = force_day.get(&escort).copied();
                    let charge_day = force_day.get(charge).copied();
                    if escort_day != charge_day {
                        let says = |day: Option<u32>| match day {
                            Some(day) => format!("enters on day {day}"),
                            None => "is there from day 1".to_string(),
                        };
                        return Err(fail(format!(
                            "the escort order on day {} names task force {escort}, which {}, to escort task force {charge}, which {}: an escort and its charge enter on one day or neither enters (decision 0195, 19)",
                            spec.day,
                            says(escort_day),
                            says(charge_day)
                        )));
                    }
                }
            }
            for (raw, day) in force_day {
                held.push((EntityRef::TaskForce(TaskForceId::new(raw)), day));
            }
            for (raw, day) in group_day {
                held.push((EntityRef::AirGroup(AirGroupId::new(raw)), day));
            }
            for (raw, day) in unit_day {
                held.push((EntityRef::LandUnit(LandUnitId::new(raw)), day));
            }
        }
        for (unit, day) in held {
            truth
                .hold_apart(unit, day)
                .map_err(|message| fail(format!("{message} (`enters`, decision 0195)")))?;
        }

        let mut world =
            World::new(file.seed, ref_data, map, truth).map_err(|e| fail(e.to_string()))?;
        // The game's days, configuration beside the objectives (decision
        // 0194, 12): set once, here, where the world is made.
        world.set_game_days(file.days);
        // Command levels go on before anything that reaches a belief
        // (decision 0134, 12): the starting contacts and estimates then reach
        // every picture as they reach the side's.
        let command_levels = if file.command_levels == Some(true) {
            let mut named = Vec::new();
            for spec in &file.task_force {
                if let Some(superior) = &spec.superior {
                    named.push(NamedSuperior {
                        side: spec.side.into(),
                        command: CommandRef::TaskForce(TaskForceId::new(spec.id)),
                        superior: superior.command().map_err(|message| {
                            fail(format!("task force {}: {message}", spec.id))
                        })?,
                    });
                }
            }
            for spec in &file.base {
                if let Some(superior) = &spec.superior {
                    named.push(NamedSuperior {
                        side: spec.side.into(),
                        command: CommandRef::Base(BaseId::new(spec.id)),
                        superior: superior
                            .command()
                            .map_err(|message| fail(format!("base {}: {message}", spec.id)))?,
                    });
                }
            }
            world
                .turn_command_levels_on(&named)
                .map_err(|e| fail(e.to_string()))?;
            Some(named)
        } else {
            for spec in &file.task_force {
                if spec.superior.is_some() {
                    return Err(fail(format!(
                        "task force {} names a superior and command levels are off",
                        spec.id
                    )));
                }
            }
            for spec in &file.base {
                if spec.superior.is_some() {
                    return Err(fail(format!(
                        "base {} names a superior and command levels are off",
                        spec.id
                    )));
                }
            }
            None
        };
        // The commanders (decision 0137, 1 to 3): checked against the
        // scenario's commands whether command levels are on or not, and held
        // only when they are.
        let mut commanders = Vec::new();
        for (index, spec) in file.commander.iter().enumerate() {
            let side: Side = spec.side.into();
            let describe = format!("commander {} ({})", index + 1, spec.name);
            let command = match (spec.task_force, spec.base, spec.area) {
                (Some(force), None, None | Some(false)) => {
                    if !file
                        .task_force
                        .iter()
                        .any(|t| t.id == force && Side::from(t.side) == side)
                    {
                        return Err(fail(format!(
                            "{describe} is named for task force {force}, which is not a task force of the {} side",
                            side.name()
                        )));
                    }
                    CommandRef::TaskForce(TaskForceId::new(force))
                }
                (None, Some(base), None | Some(false)) => {
                    if !file
                        .base
                        .iter()
                        .any(|b| b.id == base && Side::from(b.side) == side)
                    {
                        return Err(fail(format!(
                            "{describe} is named for base {base}, which is not a base of the {} side",
                            side.name()
                        )));
                    }
                    CommandRef::Base(BaseId::new(base))
                }
                (None, None, Some(true)) => CommandRef::Area,
                _ => {
                    return Err(fail(format!(
                        "{describe} names exactly one of task_force, base or area = true"
                    )))
                }
            };
            let initiative = Initiative::parse(&spec.initiative).ok_or_else(|| {
                fail(format!(
                    "{describe} has unknown initiative \"{}\": cautious, steady or bold",
                    spec.initiative
                ))
            })?;
            let note = spec.note.clone().unwrap_or_default();
            if note.trim().is_empty() {
                return Err(fail(format!(
                    "{describe}: the rating {} has no note of what it rests on; every rating is a placeholder and its note is required",
                    initiative.name()
                )));
            }
            if commanders
                .iter()
                .any(|c: &NamedCommander| c.side == side && c.command == command)
            {
                return Err(fail(format!(
                    "{describe}: {command} has a commander already"
                )));
            }
            commanders.push(NamedCommander {
                side,
                command,
                commander: Commander {
                    name: spec.name.clone(),
                    initiative,
                    note,
                },
            });
        }
        world
            .name_commanders(&commanders)
            .map_err(|e| fail(e.to_string()))?;
        let mut radio_silent = Vec::new();
        for spec in &file.task_force {
            if spec.radio_silence == Some(true) {
                let force = TaskForceId::new(spec.id);
                world
                    .keep_radio_silence_from_the_start(force)
                    .map_err(|e| fail(e.to_string()))?;
                radio_silent.push(force);
            }
        }
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
            let seen_by = match &spec.seen_by {
                None => None,
                Some(seen_by) => {
                    let command = seen_by.command().map_err(|_| {
                        fail(format!(
                            "contact {}: seen_by names exactly one of task_force or base",
                            index + 1
                        ))
                    })?;
                    let side: Side = spec.side.into();
                    let known = match command {
                        CommandRef::TaskForce(force) => file
                            .task_force
                            .iter()
                            .any(|t| t.id == force.raw() && Side::from(t.side) == side),
                        CommandRef::Base(base) => file
                            .base
                            .iter()
                            .any(|b| b.id == base.raw() && Side::from(b.side) == side),
                        CommandRef::Area => true,
                    };
                    if !known {
                        return Err(fail(format!(
                            "contact {} is seen by {command}, which is not a command of the {} side",
                            index + 1,
                            side.name()
                        )));
                    }
                    Some(command)
                }
            };
            contacts.push((
                StartingContact {
                    side: spec.side.into(),
                    hex: Hex::new(spec.q, spec.r),
                    ship_count: spec.ship_count,
                    kind,
                    age_pulses: spec.age_pulses.unwrap_or(0),
                },
                seen_by,
            ));
        }
        world
            .hold_starting_contacts_seen(contacts.clone())
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
        // The waters a side suspects enemy submarines work (decision 0218,
        // 47): told on their days.
        let mut waters = Vec::new();
        for (index, spec) in file.suspected_waters.iter().enumerate() {
            let what = format!("suspected waters {}", index + 1);
            let day = spec.day.unwrap_or(1);
            if day == 0 || day > file.days {
                return Err(fail(format!(
                    "{what} are told on day {day}, which is not a day of the game (1 to {})",
                    file.days
                )));
            }
            for (name, text) in [
                ("words", &spec.words),
                ("rows", &spec.rows),
                ("reading", &spec.reading),
            ] {
                if text.trim().is_empty() {
                    return Err(fail(format!("{what} have no {name}")));
                }
            }
            waters.push(WatersWaiting {
                side: spec.side.into(),
                hex: Hex::new(spec.q, spec.r),
                distance_hexes: spec.distance,
                words: spec.words.clone(),
                rows: spec.rows.clone(),
                reading: spec.reading.clone(),
                day,
            });
        }
        world
            .hold_suspected_waters(waters.clone())
            .map_err(|e| fail(e.to_string()))?;

        // What each side believed of the enemy (decision 0173, 5 to 9):
        // checked and kept with the scenario, never put into the world.
        let mut believed_forces = Vec::new();
        for (index, spec) in file.believed_force.iter().enumerate() {
            believed_forces.push(believed_force(spec, index, file.days, &world).map_err(&fail)?);
        }
        let mut believed_intents = Vec::new();
        for (index, spec) in file.believed_intent.iter().enumerate() {
            believed_intents.push(believed_intent(spec, index, &world).map_err(&fail)?);
        }
        let mut believed_men_list = Vec::new();
        for (index, spec) in file.believed_men.iter().enumerate() {
            believed_men_list.push(believed_men(spec, index, &world).map_err(&fail)?);
        }

        Ok(Scenario {
            name: file.name,
            placeholder: file.placeholder,
            days: file.days,
            world,
            script,
            orders_inside_a_day,
            script_inside_day,
            commands,
            contacts,
            estimates,
            waters,
            command_levels,
            radio_silent,
            commanders,
            data_files,
            estimated_figures,
            believed_forces,
            believed_intents,
            believed_men: believed_men_list,
            trial_figures,
            map_frame,
        })
    }

    /// The frame of the scenario's map, from its header (decision 0199):
    /// `None` for a hand-written map, which has none.
    pub fn map_frame(&self) -> Option<&MapFrame> {
        self.map_frame.as_ref()
    }

    /// A trial's figures (decision 0174), where the scenario names them.
    pub fn trial_figures(&self) -> Option<&TrialFigures> {
        self.trial_figures.as_ref()
    }

    /// The enemy forces `side` believed in and held no contact of when the
    /// game began (decision 0173, 5), in the order the file gives them.
    pub fn believed_forces(&self, side: Side) -> impl Iterator<Item = &BelievedForce> + '_ {
        self.believed_forces
            .iter()
            .filter(move |force| force.side == side)
    }

    /// The intents `side` believed the enemy held when the game began
    /// (decision 0173, 7), in the order the file gives them.
    pub fn believed_intents(&self, side: Side) -> impl Iterator<Item = &BelievedIntent> + '_ {
        self.believed_intents
            .iter()
            .filter(move |intent| intent.side == side)
    }

    /// The men of the enemy's `side` believed may be ashore at a place beyond
    /// its estimate when the game began (decision 0176, 1), in the order the
    /// file gives them.
    pub fn believed_men(&self, side: Side) -> impl Iterator<Item = &BelievedMen> + '_ {
        self.believed_men.iter().filter(move |men| men.side == side)
    }

    /// The superiors the scenario names, if command levels are on (decision
    /// 0134): `None` when they are off, `Some` (empty if it names none) when
    /// on.
    pub fn command_levels(&self) -> Option<&[NamedSuperior]> {
        self.command_levels.as_deref()
    }

    /// The files the scenario names (map, ship classes, port sizes, aircraft
    /// types, then the figures files it has), as the paths they are read from.
    /// The classes the scenario loads with figures that rest on an estimate,
    /// each with the figures (decision 0207, rule 10).
    pub fn estimated_figures(&self) -> &crate::reference::EstimatedFigures {
        &self.estimated_figures
    }

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

    /// Whether a side may give orders inside a day, at 0600 and 1200
    /// (decision 0156): the scenario's key `orders_inside_a_day`, off when
    /// absent.
    pub fn orders_inside_a_day(&self) -> bool {
        self.orders_inside_a_day
    }

    /// The orders still to be given inside a day, by day, each with its
    /// pulse and side, in the order the file gives them (decision 0156).
    pub fn scripted_orders_inside_day(&self) -> &ScriptInsideDay {
        &self.script_inside_day
    }

    /// Replaces the orders given inside a day, for replaying a saved game
    /// (decision 0156), as [`Scenario::replace_script`] replaces those of
    /// dusk.
    pub fn replace_script_inside_day(&mut self, script: ScriptInsideDay) {
        self.script_inside_day = script;
    }

    /// The commander's commands still to be given, by day, as they will be
    /// submitted (decision 0107, item 16).
    pub fn scripted_commands(&self) -> &BTreeMap<u32, Vec<(Side, CommandKind)>> {
        &self.commands
    }

    /// Replaces the scripted commands, for replaying a saved game: the
    /// commands it handed in are submitted day by day in place of the
    /// scenario's own.
    pub fn replace_commands(&mut self, commands: BTreeMap<u32, Vec<(Side, CommandKind)>>) {
        self.commands = commands;
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
        if let Some(named) = &self.command_levels {
            world.turn_command_levels_on(named)?;
        }
        world.name_commanders(&self.commanders)?;
        for force in &self.radio_silent {
            world.keep_radio_silence_from_the_start(*force)?;
        }
        world.hold_starting_contacts_seen(self.contacts.clone())?;
        world.hold_starting_estimates(self.estimates.clone())?;
        world.hold_suspected_waters(self.waters.clone())?;
        if let Some(objectives) = self.world.objectives() {
            world.set_objectives(objectives.clone())?;
        }
        if let Some(days) = self.world.game_days() {
            world.set_game_days(days);
        }
        Ok(Scenario {
            name: self.name.clone(),
            placeholder: self.placeholder,
            days: self.days,
            world,
            script: self.script.clone(),
            orders_inside_a_day: self.orders_inside_a_day,
            script_inside_day: self.script_inside_day.clone(),
            commands: self.commands.clone(),
            contacts: self.contacts.clone(),
            estimates: self.estimates.clone(),
            waters: self.waters.clone(),
            command_levels: self.command_levels.clone(),
            radio_silent: self.radio_silent.clone(),
            commanders: self.commanders.clone(),
            data_files: self.data_files.clone(),
            estimated_figures: self.estimated_figures.clone(),
            believed_forces: self.believed_forces.clone(),
            believed_intents: self.believed_intents.clone(),
            believed_men: self.believed_men.clone(),
            trial_figures: self.trial_figures.clone(),
            map_frame: self.map_frame,
        })
    }

    /// Submits the scripted orders for the next day and resolves it, with no
    /// staff for either side. Returns `None` once the scenario has run all
    /// its days.
    pub fn next_day(&mut self) -> Result<Option<DaySummary>, DataError> {
        self.next_day_watched(Staffs::none(), &mut |_, _| {})
    }

    /// [`Scenario::next_day`], with each side's staff in `staffs` asked at
    /// its step of the day (decision 0107).
    pub fn next_day_staffed(
        &mut self,
        staffs: Staffs<'_>,
    ) -> Result<Option<DaySummary>, DataError> {
        self.next_day_watched(staffs, &mut |_, _| {})
    }

    /// [`Scenario::next_day_staffed`], handing the world, read only, to
    /// `watch` at each moment of the day as [`World::resolve_day_watched`]
    /// does. The day is the same day: `next_day_staffed` is this with a
    /// watcher that does nothing.
    pub fn next_day_watched(
        &mut self,
        staffs: Staffs<'_>,
        watch: &mut dyn FnMut(&World, Moment),
    ) -> Result<Option<DaySummary>, DataError> {
        let day = self.world.day() + 1;
        if day > self.days {
            return Ok(None);
        }
        // The commander's commands are handed in before the day resolves and
        // taken in at its first step (decision 0107).
        for (side, command) in self.commands.remove(&day).unwrap_or_default() {
            self.world.submit_commands(side, vec![command]);
        }
        for (side, request) in self.script.remove(&day).unwrap_or_default() {
            self.world.submit_orders(side, vec![request]);
        }
        // A day with no order inside it resolves whole, as it always has.
        let inside = self.script_inside_day.remove(&day).unwrap_or_default();
        if inside.is_empty() {
            return Ok(Some(self.world.resolve_day_watched(staffs, watch)?));
        }
        // A day with orders inside it is resolved in its parts (decision
        // 0156), each order handed in before the pulse it is for, in the
        // order the file gives them.
        self.world.open_day(staffs, watch)?;
        for pulse in Pulse::ALL {
            for (at, side, request) in &inside {
                if *at == pulse {
                    self.world
                        .submit_orders_inside_day(*side, vec![request.clone()])?;
                }
            }
            self.world.resolve_next_pulse(watch)?;
        }
        Ok(Some(self.world.close_day(watch)?))
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
    use std::collections::BTreeSet;

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

    /// Decision 0156: an order may carry the pulse it is given for inside a
    /// day only where the scenario turns orders inside a day on, and only
    /// the morning (0600) or the afternoon (1200). The key is off when
    /// absent, and an order with no pulse is an order of dusk.
    #[test]
    fn an_order_inside_a_day_needs_the_key_and_the_morning_or_the_afternoon() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        let keyed = |on: &str, extra: &str| {
            let head = HEAD.replacen("days = 2\n", &format!("days = 2\n{on}"), 1);
            Scenario::parse(&format!("{head}{extra}"), &path)
        };
        let order = |pulse: &str| {
            format!(
                "\n[[order]]\nday = 1\n{pulse}side = \"allied\"\ntask_force = 1\nkind = \"set_speed\"\nspeed = \"full\"\n"
            )
        };
        // Off when absent, and an order of dusk has no pulse.
        let plain = parse_with(&order("")).expect("loads");
        assert!(!plain.orders_inside_a_day());
        assert_eq!(plain.scripted_orders()[&1].len(), 1);
        assert!(plain.scripted_orders_inside_day().is_empty());
        // On, with an order at 0600 and one at 1200.
        let on = keyed(
            "orders_inside_a_day = true\n",
            &format!(
                "{}{}{}",
                order(""),
                order("pulse = \"afternoon\"\n"),
                order("pulse = \"morning\"\n")
            ),
        )
        .expect("loads");
        assert!(on.orders_inside_a_day());
        assert_eq!(on.scripted_orders()[&1].len(), 1);
        let pulses: Vec<Pulse> = on.scripted_orders_inside_day()[&1]
            .iter()
            .map(|(pulse, side, _)| {
                assert_eq!(*side, Side::Allied);
                *pulse
            })
            .collect();
        assert_eq!(pulses, vec![Pulse::Afternoon, Pulse::Morning]);
        // A pulse without the key, or with it off.
        for off in ["", "orders_inside_a_day = false\n"] {
            let message = keyed(off, &order("pulse = \"morning\"\n"))
                .expect_err("refused")
                .message;
            assert!(
                message.contains("orders inside a day need orders_inside_a_day = true"),
                "{message}"
            );
        }
        // Any pulse but the morning or the afternoon.
        for bad in ["night", "dusk", "Morning"] {
            let message = keyed(
                "orders_inside_a_day = true\n",
                &order(&format!("pulse = \"{bad}\"\n")),
            )
            .expect_err("refused")
            .message;
            assert!(
                message.contains(&format!("has pulse \"{bad}\""))
                    && message.contains("\"morning\" (at 0600) or the \"afternoon\" (at 1200)"),
                "{message}"
            );
        }
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

    fn supply_line(extra: &str) -> String {
        format!("[[supply_line]]\n{extra}")
    }

    #[test]
    fn supply_lines_load_with_defaults_in_one_order_and_are_checked() {
        let none = parse_with("").expect("loads");
        assert!(none.world.umpire().supply_lines.is_empty());
        // Defaults: the base's side, day 1 to the last day, fuel 0.
        let one = parse_with(&supply_line("base = 2\nsupply_t = 10\n")).expect("loads");
        let lines = &one.world.umpire().supply_lines;
        assert_eq!(lines.len(), 1);
        assert_eq!(
            (
                lines[0].side,
                lines[0].base.raw(),
                lines[0].first_day,
                lines[0].last_day
            ),
            (Side::Japanese, 2, 1, 2)
        );
        assert_eq!(lines[0].supply_t, Fixed::from_int(10));
        assert_eq!(lines[0].fuel_t, Fixed::ZERO);
        // A decimal string loads exactly; a TOML float is refused.
        let dec = parse_with(&supply_line("base = 1\nfuel_t = \"12.5\"\n")).expect("loads");
        assert_eq!(
            dec.world.umpire().supply_lines[0].fuel_t,
            "12.5".parse::<Fixed>().unwrap()
        );
        assert!(parse_with(&supply_line("base = 1\nfuel_t = 12.5\n")).is_err());
        // An explicit side may differ from the base's.
        let other = parse_with(&supply_line(
            "base = 1\nside = \"japanese\"\nsupply_t = 1\n",
        ))
        .expect("loads");
        assert_eq!(other.world.umpire().supply_lines[0].side, Side::Japanese);
        // Refusals.
        let bad = |extra: &str| error_of(&supply_line(extra));
        let m = bad("base = 9\nsupply_t = 1\n");
        assert!(
            m.contains("supply line 1") && m.contains("unknown base"),
            "{m}"
        );
        let m = bad("base = 1\nsupply_t = -1\nfuel_t = 5\n");
        assert!(m.contains("supply line 1") && m.contains("below 0"), "{m}");
        let m = bad("base = 1\nsupply_t = 1\nfuel_t = -1\n");
        assert!(m.contains("supply line 1") && m.contains("below 0"), "{m}");
        let m = bad("base = 1\n");
        assert!(
            m.contains("supply line 1") && m.contains("gives nothing"),
            "{m}"
        );
        let m = bad("base = 1\nsupply_t = 0\nfuel_t = 0\n");
        assert!(m.contains("gives nothing"), "{m}");
        let m = bad("base = 1\nsupply_t = 1\nfirst_day = 0\n");
        assert!(
            m.contains("supply line 1") && m.contains("first day below 1"),
            "{m}"
        );
        let m = bad("base = 1\nsupply_t = 1\nfirst_day = 3\n");
        assert!(
            m.contains("supply line 1") && m.contains("past the game's last day"),
            "{m}"
        );
        let m = bad("base = 1\nsupply_t = 1\nlast_day = 3\n");
        assert!(m.contains("past the game's last day"), "{m}");
        let m = bad("base = 1\nsupply_t = 1\nfirst_day = 2\nlast_day = 1\n");
        assert!(
            m.contains("supply line 1") && m.contains("before its first day"),
            "{m}"
        );
        // The number is the line's place in the file.
        let second = format!(
            "{}{}",
            supply_line("base = 1\nsupply_t = 1\n"),
            supply_line("base = 9\nsupply_t = 1\n")
        );
        assert!(error_of(&second).contains("supply line 2"));
        assert!(parse_with(&supply_line("base = 1\nsupply_t = 1\nextra = 1\n")).is_err());
        // Two lines to one base are both kept, in one order.
        let a = supply_line("base = 1\nsupply_t = 5\nfirst_day = 2\n");
        let b = supply_line("base = 1\nsupply_t = 3\n");
        let ab = parse_with(&format!("{a}{b}")).expect("loads");
        let ba = parse_with(&format!("{b}{a}")).expect("loads");
        let (x, y) = (
            &ab.world.umpire().supply_lines,
            &ba.world.umpire().supply_lines,
        );
        assert_eq!(x.len(), 2);
        assert_eq!(x, y);
        assert_eq!((x[0].first_day, x[1].first_day), (1, 2));
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
                .contains("exactly one of task_force, air_group, land_unit and base")
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

    /// `HEAD` with the reports key added after the surface combat key, read
    /// with `options`.
    fn with_reports(key: &str, options: &LoadOptions) -> Result<Scenario, DataError> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        let head = HEAD.replace(
            "surface_combat = \"../reference/surface_combat.toml\"\n",
            &format!("surface_combat = \"../reference/surface_combat.toml\"\n{key}"),
        );
        Scenario::parse_with(&head, &path, options)
    }

    #[test]
    fn the_reports_key_is_optional_and_is_read_where_the_scenario_names_it() {
        assert!(parse_with("")
            .expect("loads")
            .world
            .ref_data()
            .reports
            .is_none());
        let key = "reports = \"../reference/reports.toml\"\n";
        let with = with_reports(key, &LoadOptions::default()).expect("loads");
        let figures = with
            .world
            .ref_data()
            .reports
            .expect("the figures are loaded");
        assert_eq!(figures.up.on_time, Fixed::from_ratio(7, 10));
        assert_eq!(figures.reach_base_nm, Fixed::from_int(1000));
        // It is among the files the scenario records as its inputs.
        assert!(with
            .data_files
            .iter()
            .any(|file| file.ends_with("reports.toml")));
        // A reference root does not redirect it: the frozen step 1 figures
        // have no report figures, and the scenario still gets its own.
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let frozen = with_reports(key, &LoadOptions::frozen_step1(&root)).expect("loads");
        assert_eq!(
            frozen.world.ref_data().reports,
            with.world.ref_data().reports
        );
        let error = with_reports(
            "reports = \"../reference/nothing.toml\"\n",
            &LoadOptions::default(),
        )
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

    #[test]
    fn an_air_group_takes_optional_strike_settings_checked_at_load() {
        let group = |extra: &str| {
            let text = air_group(1, 3).replace("ready = ", &format!("{extra}ready = "));
            parse_with(&text).map(|scenario| {
                let group = scenario.world.umpire().air_groups[&AirGroupId::new(1)].clone();
                (
                    group.strike_contact_age,
                    group.strike_held_share,
                    group.strike_age_order,
                    group.strike_share_order,
                )
            })
        };
        assert_eq!(group("").expect("loads"), (None, Fixed::ZERO, None, None));
        assert_eq!(
            group("strike_contact_age_pulses = 0\nstrike_held_share = \"0.25\"\n").expect("loads"),
            (Some(0), Fixed::from_ratio(1, 4), None, None)
        );
        // The search figures drop a contact at 5 pulses, so 4 is the top.
        assert_eq!(
            group("strike_contact_age_pulses = 4\n").expect("loads").0,
            Some(4)
        );
        assert!(
            error_of_group(&group("strike_contact_age_pulses = 5\n")).contains("between 0 and 4")
        );
        assert!(group("strike_contact_age_pulses = -1\n").is_err());
        assert!(group("strike_contact_age_pulses = 1.5\n").is_err());
        assert_eq!(
            group("strike_held_share = 1\n").expect("loads").1,
            Fixed::ONE
        );
        assert!(group("strike_held_share = \"1.5\"\n").is_err());
        assert!(group("strike_held_share = \"-0.1\"\n").is_err());
        assert!(group("strike_held_share = 0.5\n").is_err());
    }

    fn error_of_group<T>(result: &Result<T, DataError>) -> String {
        match result {
            Ok(_) => panic!("expected an error"),
            Err(error) => error.to_string(),
        }
    }

    #[test]
    fn the_strike_setting_orders_load_for_an_air_group_and_are_checked_at_intake() {
        let age = |pulses: &str| format!("kind = \"set_strike_contact_age\"\npulses = {pulses}");
        let share = |share: &str| format!("kind = \"set_strike_held_share\"\nshare = {share}");
        parse_with(&order("air_group = 1", &age("1"))).expect("an age loads");
        // The range is the world's to refuse, at intake.
        parse_with(&order("air_group = 1", &age("99"))).expect("too old loads");
        parse_with(&order("air_group = 1", &age("-1"))).expect("negative loads");
        assert!(parse_with(&order("air_group = 1", &age("1.5"))).is_err());
        assert!(parse_with(&order("air_group = 1", "kind = \"set_strike_contact_age\"")).is_err());
        assert!(error_of(&order("task_force = 1", &age("1"))).contains("air_group"));
        assert!(parse_with(&order("air_group = 1", &format!("{}\nextra = 1", age("1")))).is_err());
        parse_with(&order("air_group = 1", &share("\"0.25\""))).expect("a share loads");
        parse_with(&order("air_group = 1", &share("\"1.5\""))).expect("a share out of range loads");
        assert!(parse_with(&order("air_group = 1", &share("0.25"))).is_err());
        assert!(parse_with(&order("air_group = 1", "kind = \"set_strike_held_share\"")).is_err());
        assert!(error_of(&order("task_force = 1", &share("\"0.25\""))).contains("air_group"));
        assert!(parse_with(&order(
            "air_group = 1",
            &format!("{}\nextra = 1", share("\"0.25\""))
        ))
        .is_err());
    }

    #[test]
    fn the_strike_sizing_order_loads_for_an_air_group() {
        let sizing = |on: &str| format!("kind = \"set_strike_sizing\"\non = {on}");
        parse_with(&order("air_group = 1", &sizing("true"))).expect("on loads");
        parse_with(&order("air_group = 1", &sizing("false"))).expect("off loads");
        assert!(parse_with(&order("air_group = 1", "kind = \"set_strike_sizing\"")).is_err());
        assert!(parse_with(&order("air_group = 1", &sizing("1"))).is_err());
        assert!(error_of(&order("task_force = 1", &sizing("true"))).contains("air_group"));
    }

    #[test]
    fn the_search_once_order_loads_for_an_air_group() {
        let once = |on: &str| format!("kind = \"set_search_once\"\non = {on}");
        parse_with(&order("air_group = 1", &once("true"))).expect("on loads");
        parse_with(&order("air_group = 1", &once("false"))).expect("off loads");
        assert!(parse_with(&order("air_group = 1", "kind = \"set_search_once\"")).is_err());
        assert!(parse_with(&order("air_group = 1", &once("1"))).is_err());
        assert!(error_of(&order("task_force = 1", &once("true"))).contains("air_group"));
    }

    #[test]
    fn the_strike_first_waters_order_loads_for_an_air_group() {
        let waters = |hexes: &str| format!("kind = \"set_strike_first_waters\"\nhexes = {hexes}");
        parse_with(&order("air_group = 1", &waters("[[0, 0], [1, -1]]"))).expect("hexes load");
        parse_with(&order("air_group = 1", &waters("[]"))).expect("an empty list loads");
        assert!(parse_with(&order(
            "air_group = 1",
            "kind = \"set_strike_first_waters\""
        ))
        .is_err());
        assert!(error_of(&order("task_force = 1", &waters("[[0, 0]]"))).contains("air_group"));
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

    /// `HEAD` with `command_levels` as given (nothing when `None`), two
    /// allied task forces each with a ship, task force 2 with `superior_2`
    /// as an extra key of its table, and `base_extra` added to base 1.
    fn command_text(levels: Option<&str>, superior_2: &str, base_extra: &str) -> String {
        let head = HEAD.replace(
            "days = 2\n",
            &format!(
                "days = 2\n{}",
                levels.map_or(String::new(), |v| format!("command_levels = {v}\n"))
            ),
        );
        let head = head.replace("q = 0\nr = 0\n", &format!("q = 0\nr = 0\n{base_extra}\n"));
        let force = |id: u32, extra: &str| {
            format!(
                "[[task_force]]\nid = {id}\nname = \"TF\"\nside = \"allied\"\nat_base = 1\nmission = \"escort\"\n{extra}\n[[ship]]\nid = {id}\nname = \"S\"\nclass = 1\nside = \"allied\"\nfuel_t = 100\ntask_force = {id}\n"
            )
        };
        format!("{head}{}{}", force(1, ""), force(2, superior_2))
    }

    fn load_command_text(text: &str) -> Result<Scenario, DataError> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        Scenario::parse(text, &path)
    }

    fn command_ids(scenario: &Scenario, side: Side) -> Vec<CommandRef> {
        scenario
            .world
            .commands(side)
            .map(|posts| posts.keys().copied().collect())
            .unwrap_or_default()
    }

    #[test]
    fn command_levels_are_off_when_the_key_is_absent_or_false() {
        for levels in [None, Some("false")] {
            let scenario = load_command_text(&command_text(levels, "", "")).expect("loads");
            assert!(!scenario.world.command_levels());
            assert!(scenario.world.commands(Side::Allied).is_none());
            assert!(scenario.command_levels().is_none());
        }
    }

    #[test]
    fn command_levels_on_gives_each_side_its_commands() {
        let scenario = load_command_text(&command_text(Some("true"), "", "")).expect("loads");
        assert!(scenario.world.command_levels());
        assert_eq!(scenario.command_levels(), Some(&[][..]));
        let force = |raw| CommandRef::TaskForce(TaskForceId::new(raw));
        assert_eq!(
            command_ids(&scenario, Side::Allied),
            vec![
                CommandRef::Area,
                force(1),
                force(2),
                CommandRef::Base(BaseId::new(1))
            ]
        );
        assert_eq!(
            command_ids(&scenario, Side::Japanese),
            vec![CommandRef::Area, CommandRef::Base(BaseId::new(2))]
        );
        let posts = scenario.world.commands(Side::Allied).expect("on");
        assert_eq!(posts[&CommandRef::Area].superior(), None);
        assert_eq!(posts[&force(1)].superior(), Some(CommandRef::Area));
        // The seed copy turns them on likewise.
        let other = scenario.with_seed(9).expect("copies");
        assert_eq!(
            command_ids(&other, Side::Allied),
            command_ids(&scenario, Side::Allied)
        );
        assert_eq!(other.command_levels(), scenario.command_levels());
    }

    #[test]
    fn a_named_superior_is_kept_and_with_seed_names_it_again() {
        let scenario = load_command_text(&command_text(
            Some("true"),
            "superior = { task_force = 1 }",
            "superior = { task_force = 2 }",
        ))
        .expect("loads");
        let force = |raw| CommandRef::TaskForce(TaskForceId::new(raw));
        let posts = scenario.world.commands(Side::Allied).expect("on");
        assert_eq!(posts[&force(2)].superior(), Some(force(1)));
        assert_eq!(
            posts[&CommandRef::Base(BaseId::new(1))].superior(),
            Some(force(2))
        );
        assert_eq!(scenario.command_levels().expect("on").len(), 2);
        let other = scenario.with_seed(3).expect("copies");
        let posts = other.world.commands(Side::Allied).expect("on");
        assert_eq!(posts[&force(2)].superior(), Some(force(1)));
        assert_eq!(
            posts[&CommandRef::Base(BaseId::new(1))].superior(),
            Some(force(2))
        );
    }

    #[test]
    fn a_superior_with_command_levels_off_is_refused_naming_the_entry() {
        for levels in [None, Some("false")] {
            let message =
                load_command_text(&command_text(levels, "superior = { task_force = 1 }", ""))
                    .expect_err("refused")
                    .message;
            assert!(message.contains("task force 2"), "{message}");
            assert!(message.contains("command levels are off"), "{message}");
            let message =
                load_command_text(&command_text(levels, "", "superior = { task_force = 1 }"))
                    .expect_err("refused")
                    .message;
            assert!(message.contains("base 1"), "{message}");
        }
    }

    #[test]
    fn a_superior_that_is_no_command_names_itself_or_makes_a_loop_is_refused() {
        // An unknown command, and a base that is not the side's.
        for superior in [
            "superior = { task_force = 9 }",
            "superior = { base = 2 }",
            "superior = { base = 7 }",
        ] {
            let message = load_command_text(&command_text(Some("true"), superior, ""))
                .expect_err("refused")
                .message;
            assert!(
                message.contains("is not a command"),
                "{superior}: {message}"
            );
        }
        // Itself.
        let message = load_command_text(&command_text(
            Some("true"),
            "superior = { task_force = 2 }",
            "",
        ))
        .expect_err("refused")
        .message;
        assert!(message.contains("its own superior"), "{message}");
        // A loop: base 1 under task force 2, task force 2 under base 1.
        let message = load_command_text(&command_text(
            Some("true"),
            "superior = { base = 1 }",
            "superior = { task_force = 2 }",
        ))
        .expect_err("refused")
        .message;
        assert!(message.contains("loop"), "{message}");
        // Neither or both of task_force and base.
        for superior in [
            "superior = {}",
            "superior = { task_force = 1, base = 1 }",
            "superior = { fleet = 1 }",
        ] {
            assert!(
                load_command_text(&command_text(Some("true"), superior, "")).is_err(),
                "{superior}"
            );
        }
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

    #[test]
    fn men_landed_loads_with_its_base_and_points_and_is_refused_as_the_others_are() {
        let body = "[[objectives.allied.objective]]\nkind = \"men_landed\"\nbase = 1\npoints_per_100_men = \"0.5\"\n";
        let scenario = parse_with(&objectives(LEVELS, body)).expect("loads");
        let found = scenario.world.objectives().expect("has objectives");
        assert_eq!(
            found.allied.objectives,
            vec![Objective::MenLanded {
                base: BaseId::new(1),
                points_per_100_men: "0.5".parse().expect("a decimal"),
            }]
        );
        // The world keeps the hex of the base named.
        let hex = scenario.world.umpire().bases[&BaseId::new(1)].hex;
        assert_eq!(found.base_hexes.get(&BaseId::new(1)), Some(&hex));
        let message = error_of(&objectives(
            LEVELS,
            "[[objectives.allied.objective]]\nkind = \"men_landed\"\nbase = 1\npoints_per_100_men = -1\n",
        ));
        assert!(
            message.contains("points_per_100_men is negative"),
            "{message}"
        );
        let message = error_of(&objectives(
            LEVELS,
            "[[objectives.allied.objective]]\nkind = \"men_landed\"\nbase = 77\npoints_per_100_men = 1\n",
        ));
        assert!(message.contains("unknown base:77"), "{message}");
    }

    #[test]
    fn submarines_sunk_loads_with_its_points_and_is_refused_as_the_others_are() {
        let body =
            "[[objectives.allied.objective]]\nkind = \"submarines_sunk\"\npoints = \"2.5\"\n";
        let scenario = parse_with(&objectives(LEVELS, body)).expect("loads");
        let found = scenario.world.objectives().expect("has objectives");
        assert_eq!(
            found.allied.objectives,
            vec![Objective::SubmarinesSunk {
                points: "2.5".parse().expect("a decimal"),
            }]
        );
        assert_eq!(found.allied.objectives[0].name(), "submarines_sunk");
        let message = error_of(&objectives(
            LEVELS,
            "[[objectives.allied.objective]]\nkind = \"submarines_sunk\"\npoints = -1\n",
        ));
        assert!(message.contains("points is negative"), "{message}");
    }

    #[test]
    fn a_scenario_is_of_record_unless_its_objectives_say_it_is_for_play() {
        let of = |levels: &str| {
            parse_with(&objectives(levels, ""))
                .expect("loads")
                .world
                .objectives()
                .expect("has objectives")
                .for_play
        };
        assert!(!of(LEVELS));
        assert!(!of(&format!("{LEVELS}\nfor_play = false")));
        assert!(of(&format!("{LEVELS}\nfor_play = true")));
        let message = error_of(&objectives(&format!("{LEVELS}\nfor_play = 1"), ""));
        assert!(!message.is_empty());
    }

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
        // The live figures load; no value of them is asserted, so a tuning
        // round changes nothing here (the architect's ruling of Nov 15, 2026).
        assert!(
            scenario.world.ref_data().ground.is_some(),
            "the figures load"
        );
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
    fn a_base_takes_optional_reserves_that_are_not_negative() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        let base = |extra: &str| {
            let head = HEAD.replace("q = 5\nr = 0\n", &format!("q = 5\nr = 0\n{extra}"));
            Scenario::parse(&head, &path)
        };
        let reserves = |scenario: Scenario| {
            let base = scenario.world.umpire().bases[&BaseId::new(2)].clone();
            (base.supply_reserve_t, base.fuel_reserve_t)
        };
        assert_eq!(
            reserves(base("").expect("loads")),
            (Fixed::ZERO, Fixed::ZERO)
        );
        assert_eq!(
            reserves(base("supply_reserve_t = 400\nfuel_reserve_t = 300\n").expect("loads")),
            (Fixed::from_int(400), Fixed::from_int(300))
        );
        assert_eq!(
            reserves(base("fuel_reserve_t = \"0.25\"\n").expect("loads")),
            (Fixed::ZERO, Fixed::from_ratio(1, 4))
        );
        assert!(base("supply_reserve_t = -1\n").is_err());
        assert!(base("fuel_reserve_t = \"-0.5\"\n").is_err());
        assert!(base("supply_reserve_t = 1.5\n").is_err());
        assert!(base("fuel_reserve_t = 1.5\n").is_err());
    }

    #[test]
    fn a_load_order_may_name_a_weight_for_supply_or_fuel() {
        let orders = loaded_orders("kind = \"load\"\nweight_t = 250").expect("loads");
        assert_eq!(
            orders,
            vec![TaskForceOrder::LoadTo {
                cargo: Cargo::Supply,
                weight_t: Fixed::from_int(250),
                each_t: None,
            }]
        );
        assert_eq!(
            loaded_orders("kind = \"load\"\ncargo = \"supply\"\nweight_t = \"2.5\"")
                .expect("loads"),
            vec![TaskForceOrder::LoadTo {
                cargo: Cargo::Supply,
                weight_t: Fixed::from_ratio(5, 2),
                each_t: None,
            }]
        );
        assert_eq!(
            loaded_orders("kind = \"load\"\ncargo = \"fuel\"\nweight_t = 80").expect("loads"),
            vec![TaskForceOrder::LoadTo {
                cargo: Cargo::Fuel,
                weight_t: Fixed::from_int(80),
                each_t: None,
            }]
        );
        // The sign is for intake to refuse, as it is for knots.
        assert_eq!(
            loaded_orders("kind = \"load\"\nweight_t = 0").expect("loads"),
            vec![TaskForceOrder::LoadTo {
                cargo: Cargo::Supply,
                weight_t: Fixed::ZERO,
                each_t: None,
            }]
        );
        assert!(loaded_orders("kind = \"load\"\nweight_t = -5").is_ok());
        // A float is never a quantity; an unload takes no weight.
        assert!(loaded_orders("kind = \"load\"\nweight_t = 2.5").is_err());
        assert!(loaded_orders("kind = \"unload\"\nweight_t = 5").is_err());
    }

    #[test]
    fn a_load_to_a_weight_may_name_a_level_above_0() {
        assert_eq!(
            loaded_orders("kind = \"load\"\nweight_t = 100\neach_t = 40").expect("loads"),
            vec![TaskForceOrder::LoadTo {
                cargo: Cargo::Supply,
                weight_t: Fixed::from_int(100),
                each_t: Some(Fixed::from_int(40)),
            }]
        );
        assert_eq!(
            loaded_orders("kind = \"load\"\ncargo = \"fuel\"\nweight_t = 100\neach_t = \"2.5\"")
                .expect("loads"),
            vec![TaskForceOrder::LoadTo {
                cargo: Cargo::Fuel,
                weight_t: Fixed::from_int(100),
                each_t: Some(Fixed::from_ratio(5, 2)),
            }]
        );
        // Refused with no weight, with a unit, at 0 or below, and as a float
        // (decision 0162, 3).
        for text in [
            "kind = \"load\"\neach_t = 40",
            "kind = \"load\"\nunit = 3\neach_t = 40",
            "kind = \"load\"\nunit = 3\nweight_t = 100\neach_t = 40",
            "kind = \"load\"\nweight_t = 100\neach_t = 0",
            "kind = \"load\"\nweight_t = 100\neach_t = -5",
            "kind = \"load\"\nweight_t = 100\neach_t = 2.5",
            "kind = \"unload\"\neach_t = 5",
        ] {
            assert!(loaded_orders(text).is_err(), "{text}");
        }
    }

    #[test]
    fn a_weight_does_not_apply_to_men_or_to_no_cargo() {
        let message = loaded_orders("kind = \"load\"\nunit = 3\nweight_t = 100")
            .expect_err("refused")
            .to_string();
        assert!(
            message.contains("a weight does not apply to men"),
            "{message}"
        );
        let message = loaded_orders("kind = \"load\"\nunit = 3\ncargo = \"fuel\"\nweight_t = 100")
            .expect_err("refused")
            .to_string();
        assert!(
            message.contains("a weight does not apply to men"),
            "{message}"
        );
        let message = loaded_orders("kind = \"load\"\ncargo = \"none\"\nweight_t = 100")
            .expect_err("refused")
            .to_string();
        assert!(
            message.contains("a weight needs supply or fuel"),
            "{message}"
        );
    }

    #[test]
    fn a_no_cargo_work_order_loads_for_a_task_force_with_no_other_key() {
        assert_eq!(
            loaded_orders("kind = \"no_cargo_work\"").expect("loads"),
            vec![TaskForceOrder::NoCargoWork]
        );
        assert!(loaded_orders("kind = \"no_cargo_work\"\nweight_t = 1").is_err());
        assert!(loaded_orders("kind = \"no_cargo_work\"\ninto = 7").is_err());
        let body = "kind = \"no_cargo_work\"";
        assert!(parse_with(&order("air_group = 1", body)).is_err());
        assert!(parse_with(&order("land_unit = 1", body)).is_err());
        assert!(error_of(&order("base = 1", body)).contains("for a base must be form_task_force"));
    }

    #[test]
    fn join_and_stand_down_orders_load_for_a_task_force() {
        assert_eq!(
            loaded_orders("kind = \"join\"\ninto = 7").expect("loads"),
            vec![TaskForceOrder::JoinTo(TaskForceId::new(7))]
        );
        assert_eq!(
            loaded_orders("kind = \"stand_down\"").expect("loads"),
            vec![TaskForceOrder::StandDown]
        );
        // A join names the force it joins; neither takes another key.
        assert!(loaded_orders("kind = \"join\"").is_err());
        assert!(loaded_orders("kind = \"join\"\ninto = \"7\"").is_err());
        assert!(loaded_orders("kind = \"join\"\ninto = 7\nother = 1").is_err());
        assert!(loaded_orders("kind = \"stand_down\"\ninto = 7").is_err());
        assert!(loaded_orders("kind = \"stand_down\"\nweight_t = 1").is_err());
        // Both are addressed to a task force.
        for body in ["kind = \"join\"\ninto = 7", "kind = \"stand_down\""] {
            assert!(parse_with(&order("air_group = 1", body)).is_err());
            assert!(parse_with(&order("land_unit = 1", body)).is_err());
        }
    }

    #[test]
    fn a_base_order_forms_a_task_force_and_takes_no_other_kind() {
        let form = "kind = \"form_task_force\"\nships = [5, 6]";
        let scenario = parse_with(&order("base = 1", form)).expect("loads");
        let found: Vec<OrderKind> = scenario.scripted_orders()[&1]
            .iter()
            .map(|(_, request)| request.kind.clone())
            .filter(|kind| matches!(kind, OrderKind::Base { .. }))
            .collect();
        assert_eq!(
            found,
            vec![OrderKind::Base {
                target: BaseId::new(1),
                order: BaseOrder::FormTaskForce {
                    ships: vec![ShipId::new(5), ShipId::new(6)]
                }
            }]
        );
        // An empty list loads; intake refuses it (NoShipsToForm).
        parse_with(&order("base = 1", "kind = \"form_task_force\"\nships = []")).expect("loads");
        // The ships are a list; the kind is for a base only.
        assert!(parse_with(&order("base = 1", "kind = \"form_task_force\"")).is_err());
        assert!(parse_with(&order("base = 1", "kind = \"form_task_force\"\nships = 5")).is_err());
        for target in ["task_force = 1", "air_group = 1", "land_unit = 1"] {
            assert!(parse_with(&order(target, form)).is_err());
        }
        assert!(error_of(&order("task_force = 1", form)).contains("must be addressed to a base"));
        // A base takes that kind and no other, and is the only target given.
        for body in [
            "kind = \"load\"",
            "kind = \"unload\"",
            "kind = \"join\"\ninto = 2",
            "kind = \"stand_down\"",
            "kind = \"set_posture\"\nposture = \"defend\"",
        ] {
            assert!(
                error_of(&order("base = 1", body)).contains("for a base must be form_task_force")
            );
        }
        assert!(error_of(&order("base = 1\ntask_force = 1", form))
            .contains("exactly one of task_force, air_group, land_unit and base"));
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
    fn a_land_unit_takes_its_nights_of_hunger() {
        // Decision 0205, 7: a scenario may give the count a unit starts
        // with; without it the count is nothing.
        let scenario = landing(
            true,
            &format!(
                "{}{}",
                unit_ashore(1, 100, 0, 0),
                unit_with(2, "hunger_nights = 14")
            ),
        )
        .expect("loads");
        let truth = scenario.world.umpire();
        assert_eq!(truth.land_units[&LandUnitId::new(1)].hunger_nights, 0);
        assert_eq!(truth.land_units[&LandUnitId::new(2)].hunger_nights, 14);
        let message = landing_error(true, &unit_with(1, "hunger_nights = -1"));
        assert!(message.contains("hunger_nights"), "{message}");
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

    // -----------------------------------------------------------------
    // A scenario's commands (decision 0107, item 16).
    // -----------------------------------------------------------------

    fn command(body: &str) -> String {
        format!("\n[[command]]\nday = 1\nside = \"allied\"\n{body}\n")
    }

    fn only(extra: &str) -> CommandKind {
        let scenario = parse_with(extra).expect("loads");
        let list = &scenario.scripted_commands()[&1];
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].0, Side::Allied);
        list[0].1.clone()
    }

    #[test]
    fn the_pursuit_commands_load_and_refuse_what_they_do_not_take() {
        use sim_core::command::{
            InterruptRef, InterruptSetting, PursueLeave, StandingKind, StandingSubject,
        };
        use sim_core::ids::TaskForceId;
        for (name, leave) in [
            ("ask", PursueLeave::Ask),
            ("go", PursueLeave::Go),
            ("stay", PursueLeave::Stay),
        ] {
            assert_eq!(
                only(&command(&format!(
                    "kind = \"set_pursuit\"\nintent = 4\nleave = \"{name}\""
                ))),
                CommandKind::SetPursuit {
                    intent: IntentId::new(4),
                    leave: Some(leave),
                }
            );
        }
        // Left out, the setting is cleared.
        assert_eq!(
            only(&command("kind = \"set_pursuit\"\nintent = 4")),
            CommandKind::SetPursuit {
                intent: IntentId::new(4),
                leave: None,
            }
        );
        assert!(error_of(&command(
            "kind = \"set_pursuit\"\nintent = 4\nleave = \"maybe\""
        ))
        .contains("not ask, go or stay"));
        assert!(error_of(&command("kind = \"set_pursuit\"\nleave = \"go\""))
            .contains("needs \"intent\""));
        assert!(
            error_of(&command("kind = \"set_pursuit\"\nintent = 4\nbase = 1"))
                .contains("does not take \"base\"")
        );
        for allow in [true, false] {
            assert_eq!(
                only(&command(&format!(
                    "kind = \"answer_pursuit\"\ntask_force = 2\ncontact = 7\nallow = {allow}"
                ))),
                CommandKind::AnswerPursuit {
                    force: TaskForceId::new(2),
                    contact: 7,
                    allow,
                }
            );
        }
        for missing in [
            "kind = \"answer_pursuit\"\ncontact = 7\nallow = true",
            "kind = \"answer_pursuit\"\ntask_force = 2\nallow = true",
            "kind = \"answer_pursuit\"\ntask_force = 2\ncontact = 7",
        ] {
            assert!(error_of(&command(missing)).contains("needs"), "{missing}");
        }
        assert!(error_of(&command(
            "kind = \"answer_pursuit\"\ntask_force = 2\ncontact = 7\nallow = true\nintent = 1"
        ))
        .contains("does not take \"intent\""));
        // The interrupt's standing kind and its subject.
        assert_eq!(
            only(&command(
                "kind = \"acknowledge\"\nstanding = \"asks_leave_to_pursue\"\ntask_force = 2"
            )),
            CommandKind::Acknowledge {
                interrupt: InterruptRef::Standing {
                    kind: StandingKind::AsksLeaveToPursue,
                    subject: StandingSubject::TaskForce(TaskForceId::new(2)),
                }
            }
        );
        assert_eq!(
            only(&command(
                "kind = \"set_interrupt\"\nstanding = \"asks_leave_to_pursue\"\non = false"
            )),
            CommandKind::SetInterrupt(InterruptSetting::Standing {
                kind: StandingKind::AsksLeaveToPursue,
                on: false
            })
        );
    }

    #[test]
    fn the_targets_limit_loads_with_kind_names_and_clears_with_none() {
        use sim_core::command::{Limit, TargetsLimit};
        use sim_core::refdata::ShipKind;
        let limit = |keys: &str| {
            command(&format!(
                "kind = \"set_limit\"\nlimit = \"targets\"\nintent = 4{keys}"
            ))
        };
        assert_eq!(
            only(&limit(
                "\nkinds = [\"transport\", \"carrier\"]\nwaters_first = true"
            )),
            CommandKind::SetLimit(Limit::Targets {
                intent: IntentId::new(4),
                targets: Some(TargetsLimit {
                    kinds: vec![ShipKind::Transport, ShipKind::Carrier],
                    waters_first: true,
                }),
            })
        );
        assert_eq!(
            only(&limit("\nkinds = [\"cruiser\"]")),
            CommandKind::SetLimit(Limit::Targets {
                intent: IntentId::new(4),
                targets: Some(TargetsLimit {
                    kinds: vec![ShipKind::Cruiser],
                    waters_first: false,
                }),
            })
        );
        for cleared in ["", "\nkinds = []"] {
            assert_eq!(
                only(&limit(cleared)),
                CommandKind::SetLimit(Limit::Targets {
                    intent: IntentId::new(4),
                    targets: None,
                })
            );
        }
        assert!(error_of(&limit("\nkinds = [\"submarine\"]")).contains("submarine"));
        assert!(parse_with(&limit("\nkinds = [\"carrier\"]\nrank = 1")).is_err());
    }

    #[test]
    fn every_commander_command_loads_as_the_command_it_names() {
        use sim_core::command::{ForcesLimit, IntentKind, Leave, Limit, PlanId, Risk, Want, Wants};
        use sim_core::ids::LandUnitId;
        let intent = IntentId::new(1);
        assert_eq!(
            only(&command(
                "kind = \"set_intent\"\nintent = 1\nbase = 2\nfloor_days = 10\ninterval_days = 15"
            )),
            CommandKind::SetIntent {
                intent,
                kind: IntentKind::KeepSupplied {
                    base: BaseId::new(2),
                    floor_days: 10,
                    interval_days: Some(15),
                },
            }
        );
        // The interval is optional.
        assert_eq!(
            only(&command(
                "kind = \"set_intent\"\nintent = 1\nbase = 2\nfloor_days = 10"
            )),
            CommandKind::SetIntent {
                intent,
                kind: IntentKind::KeepSupplied {
                    base: BaseId::new(2),
                    floor_days: 10,
                    interval_days: None,
                },
            }
        );
        assert_eq!(
            only(&command("kind = \"withdraw_intent\"\nintent = 1")),
            CommandKind::WithdrawIntent { intent }
        );
        assert_eq!(
            only(&command(
                "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\nships = [4, 3]\ntask_forces = [2]\nsource = 1"
            )),
            CommandKind::SetLimit(Limit::Forces {
                intent,
                limit: ForcesLimit {
                    ships: [ShipId::new(3), ShipId::new(4)].into_iter().collect(),
                    task_forces: [TaskForceId::new(2)].into_iter().collect(),
                    air_groups: BTreeSet::new(),
                    land_units: BTreeSet::new(),
                    source: Some(BaseId::new(1)),
                    submarines: Default::default(),
                    none_of: Default::default(),
                    in_place: false,
                },
            })
        );
        // Decision 0141: land units in a forces limit.
        assert_eq!(
            only(&command(
                "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 2\nland_units = [5, 3]"
            )),
            CommandKind::SetLimit(Limit::Forces {
                intent: IntentId::new(2),
                limit: ForcesLimit {
                    ships: BTreeSet::new(),
                    task_forces: BTreeSet::new(),
                    air_groups: BTreeSet::new(),
                    land_units: [LandUnitId::new(3), LandUnitId::new(5)]
                        .into_iter()
                        .collect(),
                    source: None,
                    submarines: Default::default(),
                    none_of: Default::default(),
                    in_place: false,
                },
            })
        );
        // Decision 0141: the two new intent kinds.
        assert_eq!(
            only(&command(
                "kind = \"set_intent\"\nintent = 3\nintent_kind = \"ready_landing\"\ntask_force = 2\nhex = [6, 0]\nby_day = 6"
            )),
            CommandKind::SetIntent {
                intent: IntentId::new(3),
                kind: IntentKind::ReadyLanding {
                    force: TaskForceId::new(2),
                    hex: Hex::new(6, 0),
                    by_day: Some(6),
                },
            }
        );
        assert_eq!(
            only(&command(
                "kind = \"set_intent\"\nintent = 3\nintent_kind = \"ready_landing\"\ntask_force = 2\nhex = [6, 0]"
            )),
            CommandKind::SetIntent {
                intent: IntentId::new(3),
                kind: IntentKind::ReadyLanding {
                    force: TaskForceId::new(2),
                    hex: Hex::new(6, 0),
                    by_day: None,
                },
            }
        );
        assert_eq!(
            only(&command(
                "kind = \"set_intent\"\nintent = 4\nintent_kind = \"build_up\"\nbase = 1\nwants = [\"airfield\", \"defences\"]\ndefences = 10"
            )),
            CommandKind::SetIntent {
                intent: IntentId::new(4),
                kind: IntentKind::BuildUp {
                    base: BaseId::new(1),
                    wants: Wants::new(&[Want::Airfield, Want::Defences(10)]).expect("good wants"),
                },
            }
        );
        // Decision 0142: a place to take, with and without a date.
        assert_eq!(
            only(&command(
                "kind = \"set_intent\"\nintent = 5\nintent_kind = \"take_place\"\nhex = [6, 0]\nby_day = 9"
            )),
            CommandKind::SetIntent {
                intent: IntentId::new(5),
                kind: IntentKind::TakePlace {
                    hex: Hex::new(6, 0),
                    by_day: Some(9),
                },
            }
        );
        assert_eq!(
            only(&command(
                "kind = \"set_intent\"\nintent = 5\nintent_kind = \"take_place\"\nhex = [6, 0]"
            )),
            CommandKind::SetIntent {
                intent: IntentId::new(5),
                kind: IntentKind::TakePlace {
                    hex: Hex::new(6, 0),
                    by_day: None,
                },
            }
        );
        // Decision 0151: an island to control, with and without a date.
        assert_eq!(
            only(&command(
                "kind = \"set_intent\"\nintent = 6\nintent_kind = \"control_island\"\nisland = 2\nby_day = 9"
            )),
            CommandKind::SetIntent {
                intent: IntentId::new(6),
                kind: IntentKind::ControlIsland {
                    island: 2,
                    by_day: Some(9),
                },
            }
        );
        assert_eq!(
            only(&command(
                "kind = \"set_intent\"\nintent = 6\nintent_kind = \"control_island\"\nisland = 2"
            )),
            CommandKind::SetIntent {
                intent: IntentId::new(6),
                kind: IntentKind::ControlIsland {
                    island: 2,
                    by_day: None,
                },
            }
        );
        // Decision 0146: a base to bombard with a task force.
        assert_eq!(
            only(&command(
                "kind = \"set_intent\"\nintent = 5\nintent_kind = \"bombard\"\ntask_force = 3\ntarget = 12"
            )),
            CommandKind::SetIntent {
                intent: IntentId::new(5),
                kind: IntentKind::Bombard {
                    force: TaskForceId::new(3),
                    target: BaseId::new(12),
                },
            }
        );
        // The step 2 shapes: air groups, no source, no ships, no task forces.
        assert_eq!(
            only(&command(
                "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 2\nair_groups = [9, 7]"
            )),
            CommandKind::SetLimit(Limit::Forces {
                intent: IntentId::new(2),
                limit: ForcesLimit {
                    ships: BTreeSet::new(),
                    task_forces: BTreeSet::new(),
                    air_groups: [AirGroupId::new(7), AirGroupId::new(9)]
                        .into_iter()
                        .collect(),
                    land_units: BTreeSet::new(),
                    source: None,
                    submarines: Default::default(),
                    none_of: Default::default(),
                    in_place: false,
                },
            })
        );
        // The intent kinds: cover and hold name a base and nothing else.
        for (name, kind) in [
            (
                "cover",
                IntentKind::Cover {
                    base: BaseId::new(2),
                },
            ),
            (
                "hold",
                IntentKind::Hold {
                    base: BaseId::new(2),
                },
            ),
        ] {
            assert_eq!(
                only(&command(&format!(
                    "kind = \"set_intent\"\nintent = 3\nintent_kind = \"{name}\"\nbase = 2"
                ))),
                CommandKind::SetIntent {
                    intent: IntentId::new(3),
                    kind
                }
            );
        }
        assert_eq!(
            only(&command(
                "kind = \"set_intent\"\nintent = 1\nintent_kind = \"keep_supplied\"\nbase = 2\nfloor_days = 10"
            )),
            CommandKind::SetIntent {
                intent,
                kind: IntentKind::KeepSupplied {
                    base: BaseId::new(2),
                    floor_days: 10,
                    interval_days: None,
                },
            }
        );
        assert_eq!(
            only(&command(
                "kind = \"set_limit\"\nlimit = \"area\"\nintent = 1\nhexes = [[0, 0], [1, -1]]"
            )),
            CommandKind::SetLimit(Limit::Area {
                intent,
                hexes: [Hex::new(0, 0), Hex::new(1, -1)].into_iter().collect(),
            })
        );
        assert_eq!(
            only(&command(
                "kind = \"set_limit\"\nlimit = \"priority\"\nintent = 1\nrank = 2"
            )),
            CommandKind::SetLimit(Limit::Priority {
                intent,
                rank: Some(2)
            })
        );
        assert_eq!(
            only(&command(
                "kind = \"set_limit\"\nlimit = \"protected\"\nintent = 1\nships = [3]"
            )),
            CommandKind::SetLimit(Limit::Protected {
                intent,
                ships: [ShipId::new(3)].into_iter().collect(),
            })
        );
        // A weight is a whole number or a decimal string.
        assert_eq!(
            only(&command(
                "kind = \"set_limit\"\nlimit = \"reserve\"\nbase = 1\nsupply_t = 400\nfuel_t = \"12.5\""
            )),
            CommandKind::SetLimit(Limit::Reserve {
                base: BaseId::new(1),
                supply_t: Fixed::from_int(400),
                fuel_t: "12.5".parse().expect("a weight"),
            })
        );
        assert_eq!(
            only(&command(
                "kind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"press\""
            )),
            CommandKind::SetLimit(Limit::Risk {
                intent,
                risk: Risk::Press
            })
        );
        assert_eq!(
            only(&command(
                "kind = \"set_leave\"\nleave = \"launch\"\ngiven = true"
            )),
            CommandKind::SetLeave {
                leave: Leave::Launch,
                given: true
            }
        );
        assert_eq!(
            only(&command("kind = \"approve\"\nplan = 1")),
            CommandKind::Approve {
                plan: PlanId::new(1)
            }
        );
        assert_eq!(
            only(&command("kind = \"cancel_plan\"\nplan = 3")),
            CommandKind::CancelPlan {
                plan: PlanId::new(3)
            }
        );
    }

    #[test]
    fn a_landing_or_build_up_intent_with_wrong_keys_is_refused() {
        let intent = |body: &str| {
            error_of(&command(&format!(
                "kind = \"set_intent\"\nintent = 1\n{body}"
            )))
        };
        let landing = "intent_kind = \"ready_landing\"\ntask_force = 2\nhex = [6, 0]";
        assert!(intent(&format!("{landing}\nbase = 1")).contains("does not take \"base\""));
        assert!(intent(&format!("{landing}\nby_day = 0")).contains("by_day 0"));
        assert!(intent("intent_kind = \"ready_landing\"\nhex = [6, 0]")
            .contains("needs \"task_force\""));
        let build = "intent_kind = \"build_up\"\nbase = 1";
        assert!(intent(build).contains("needs \"wants\""));
        assert!(intent(&format!("{build}\nwants = [\"defences\"]")).contains("needs \"defences\""));
        assert!(
            intent(&format!("{build}\nwants = [\"airfield\"]\ndefences = 5"))
                .contains("does not want")
        );
        assert!(
            intent(&format!("{build}\nwants = [\"airfield\", \"airfield\"]"))
                .contains("name a kind twice")
        );
        assert!(intent(&format!("{build}\nwants = []")).contains("empty"));
        assert!(
            intent(&format!("{build}\nwants = [\"defences\"]\ndefences = 101"))
                .contains("outside 1 to 100")
        );
        assert!(intent(&format!("{build}\nwants = [\"dock\"]")).contains("not airfield"));
    }

    #[test]
    fn a_take_place_intent_with_wrong_keys_is_refused() {
        let intent = |body: &str| {
            error_of(&command(&format!(
                "kind = \"set_intent\"\nintent = 1\n{body}"
            )))
        };
        let place = "intent_kind = \"take_place\"\nhex = [6, 0]";
        assert!(intent(&format!("{place}\nbase = 1")).contains("does not take \"base\""));
        assert!(
            intent(&format!("{place}\ntask_force = 1")).contains("does not take \"task_force\"")
        );
        assert!(intent(&format!("{place}\ntarget = 1")).contains("does not take \"target\""));
        assert!(intent(&format!("{place}\ndistance = 1")).contains("does not take \"distance\""));
        assert!(intent(&format!("{place}\nby_day = 0")).contains("by_day 0"));
        assert!(intent("intent_kind = \"take_place\"").contains("needs \"hex\""));
    }

    #[test]
    fn a_control_island_intent_with_wrong_keys_is_refused() {
        let intent = |body: &str| {
            error_of(&command(&format!(
                "kind = \"set_intent\"\nintent = 1\n{body}"
            )))
        };
        let island = "intent_kind = \"control_island\"\nisland = 1";
        assert!(intent(&format!("{island}\nbase = 1")).contains("does not take \"base\""));
        assert!(intent(&format!("{island}\nhex = [6, 0]")).contains("does not take \"hex\""));
        assert!(
            intent(&format!("{island}\ntask_force = 1")).contains("does not take \"task_force\"")
        );
        assert!(intent(&format!("{island}\nby_day = 0")).contains("by_day 0"));
        assert!(intent("intent_kind = \"control_island\"").contains("needs \"island\""));
        // Another kind refuses the island key.
        assert!(
            intent("intent_kind = \"take_place\"\nhex = [6, 0]\nisland = 1")
                .contains("does not take \"island\"")
        );
    }

    // Islands (decision 0151, 1).

    #[test]
    fn an_island_loads_with_its_places_as_hexes_in_order() {
        let scenario = landing(
            false,
            "\n[[island]]\nnumber = 3\nname = \"Placeholder island\"\nplaces = [{ hex = [8, 0] }, { base = 1 }]\n",
        )
        .expect("loads");
        let map = scenario.world.map();
        let island = map.island(3).expect("the island is on the map");
        assert_eq!(island.name, "Placeholder island");
        assert_eq!(island.places.len(), 2);
        assert_eq!(island.places[0].base, None);
        assert_eq!(island.places[0].hex, Hex::new(8, 0));
        assert_eq!(island.places[1].base, Some(BaseId::new(1)));
        assert_eq!(island.places[1].hex, Hex::new(0, 0));
        assert!(map.island(4).is_none());
        // It is kept through a copy for another seed.
        let other = scenario.with_seed(77).expect("copies");
        assert!(other.world.map().island(3).is_some());
    }

    #[test]
    fn a_bad_island_is_a_load_error() {
        let island = |body: &str| format!("\n[[island]]\nnumber = 1\nname = \"Island\"\n{body}\n");
        // No place.
        let message = landing_error(false, &island("places = []"));
        assert!(message.contains("has no place"), "{message}");
        // A base the scenario does not have.
        let message = landing_error(false, &island("places = [{ base = 9 }]"));
        assert!(message.contains("not a base of the scenario"), "{message}");
        // A hex off the map.
        let message = landing_error(false, &island("places = [{ hex = [99, 0] }]"));
        assert!(message.contains("off the map"), "{message}");
        // A hex that is not coast.
        let message = landing_error(false, &island("places = [{ hex = [5, 0] }]"));
        assert!(message.contains("not coast"), "{message}");
        // Both keys, and neither.
        let message = landing_error(false, &island("places = [{ base = 1, hex = [8, 0] }]"));
        assert!(message.contains("both"), "{message}");
        let message = landing_error(false, &island("places = [{}]"));
        assert!(message.contains("neither"), "{message}");
        // A number used twice.
        let message = landing_error(
            false,
            &format!(
                "{}{}",
                island("places = [{ base = 1 }]"),
                island("places = [{ base = 1 }]")
            ),
        );
        assert!(message.contains("appears twice"), "{message}");
        // An unknown key.
        let message = landing_error(false, &island("places = [{ base = 1 }]\ncolour = 1"));
        assert!(message.contains("colour"), "{message}");
    }

    #[test]
    fn a_bombard_intent_with_wrong_keys_is_refused() {
        let intent = |body: &str| {
            error_of(&command(&format!(
                "kind = \"set_intent\"\nintent = 1\n{body}"
            )))
        };
        let bombard = "intent_kind = \"bombard\"\ntask_force = 3\ntarget = 12";
        for key in ["base = 1", "hex = [6, 0]", "by_day = 2", "distance = 1"] {
            let name = key.split(' ').next().expect("a key");
            assert!(
                intent(&format!("{bombard}\n{key}")).contains(&format!("does not take \"{name}\""))
            );
        }
        assert!(intent("intent_kind = \"bombard\"\ntarget = 12").contains("needs \"task_force\""));
        assert!(intent("intent_kind = \"bombard\"\ntask_force = 3").contains("needs \"target\""));
    }

    #[test]
    fn a_hold_or_release_loads_with_its_one_target() {
        use sim_core::command::{HeldSetting, HoldTarget};
        use sim_core::ids::{AirGroupId, LandUnitId};
        for (name, make) in [
            (
                "hold",
                (|t| CommandKind::Hold { target: t }) as fn(HoldTarget) -> CommandKind,
            ),
            ("release", |t| CommandKind::Release { target: t }),
        ] {
            assert_eq!(
                only(&command(&format!("kind = \"{name}\"\nship = 5"))),
                make(HoldTarget::Ship(ShipId::new(5)))
            );
            assert_eq!(
                only(&command(&format!(
                    "kind = \"{name}\"\ntask_force = 1000001"
                ))),
                make(HoldTarget::TaskForce(TaskForceId::new(1000001)))
            );
            assert_eq!(
                only(&command(&format!(
                    "kind = \"{name}\"\nair_group = 7\nsetting = \"search_arc\""
                ))),
                make(HoldTarget::Setting {
                    group: AirGroupId::new(7),
                    setting: HeldSetting::SearchArc
                })
            );
            // With no setting the whole group is held.
            assert_eq!(
                only(&command(&format!("kind = \"{name}\"\nair_group = 7"))),
                make(HoldTarget::AirGroup(AirGroupId::new(7)))
            );
            // Decision 0141: a land unit.
            assert_eq!(
                only(&command(&format!("kind = \"{name}\"\nland_unit = 4"))),
                make(HoldTarget::LandUnit(LandUnitId::new(4)))
            );
        }
    }

    #[test]
    fn a_hold_or_release_with_a_wrong_target_or_key_is_refused_and_end_run_cannot_be_written() {
        for name in ["hold", "release"] {
            let with = |body: &str| error_of(&command(&format!("kind = \"{name}\"\n{body}")));
            assert!(with("").contains("exactly one of"));
            assert!(with("ship = 1\ntask_force = 2").contains("exactly one of"));
            assert!(with("ship = 1\nair_group = 2\nsetting = \"search_arc\"")
                .contains("exactly one of"));
            assert!(with("air_group = 2\nsetting = \"mission\"").contains("not search_arc"));
            assert!(with("ship = 1\nsetting = \"search_arc\"").contains("only with \"air_group\""));
            assert!(with("task_force = 1\nsetting = \"search_arc\"")
                .contains("only with \"air_group\""));
            assert!(with("ship = 1\nland_unit = 2").contains("exactly one of"));
            assert!(
                with("land_unit = 2\nsetting = \"search_arc\"").contains("only with \"air_group\"")
            );
            assert!(with("ship = 1\nplan = 1").contains("does not take \"plan\""));
            assert!(parse_with(&command(&format!("kind = \"{name}\"\nship = 1.5"))).is_err());
        }
        assert!(error_of(&command("kind = \"end_run\"\nplan = 1"))
            .contains("the staff's commands are the staff's to give"));
    }

    #[test]
    fn an_interrupt_command_loads_in_each_of_its_forms() {
        use sim_core::command::{
            InterruptCondition, InterruptRef, InterruptSetting, StandingKind, StandingSubject,
        };
        use sim_core::ids::InterruptId;
        assert_eq!(
            only(&command(
                "kind = \"set_interrupt\"\ninterrupt = 1\nbase = 2\nbelow_days = 35"
            )),
            CommandKind::SetInterrupt(InterruptSetting::Set {
                interrupt: InterruptId::new(1),
                condition: InterruptCondition::SupplyBelow {
                    base: BaseId::new(2),
                    days: 35
                }
            })
        );
        for (name, kind) in [
            ("cannot_meet", StandingKind::CannotMeet),
            ("cannot_bring_enough", StandingKind::CannotBringEnough),
            ("lift_turned_back", StandingKind::LiftTurnedBack),
            ("waits_for_leave", StandingKind::WaitsForLeave),
            ("ship_lost", StandingKind::ShipLost),
        ] {
            assert_eq!(
                only(&command(&format!(
                    "kind = \"set_interrupt\"\nstanding = \"{name}\"\non = false"
                ))),
                CommandKind::SetInterrupt(InterruptSetting::Standing { kind, on: false })
            );
        }
        assert_eq!(
            only(&command("kind = \"acknowledge\"\ninterrupt = 3")),
            CommandKind::Acknowledge {
                interrupt: InterruptRef::Set(InterruptId::new(3))
            }
        );
        assert_eq!(
            only(&command(
                "kind = \"acknowledge\"\nstanding = \"waits_for_leave\"\nplan = 1"
            )),
            CommandKind::Acknowledge {
                interrupt: InterruptRef::Standing {
                    kind: StandingKind::WaitsForLeave,
                    subject: StandingSubject::Plan(sim_core::command::PlanId::new(1))
                }
            }
        );
        assert_eq!(
            only(&command(
                "kind = \"acknowledge\"\nstanding = \"ship_lost\"\nship = 4"
            )),
            CommandKind::Acknowledge {
                interrupt: InterruptRef::Standing {
                    kind: StandingKind::ShipLost,
                    subject: StandingSubject::Ship(ShipId::new(4))
                }
            }
        );
    }

    #[test]
    fn an_interrupt_command_with_a_wrong_key_or_value_is_refused() {
        let set = |body: &str| error_of(&command(&format!("kind = \"set_interrupt\"\n{body}")));
        let ack = |body: &str| error_of(&command(&format!("kind = \"acknowledge\"\n{body}")));
        // A missing key.
        assert!(set("interrupt = 1\nbase = 2").contains("needs \"below_days\""));
        assert!(set("interrupt = 1\nbelow_days = 5").contains("needs \"base\""));
        assert!(set("base = 2\nbelow_days = 5").contains("needs \"interrupt\""));
        assert!(set("standing = \"cannot_meet\"").contains("needs \"on\""));
        assert!(ack("").contains("needs \"interrupt\""));
        assert!(ack("standing = \"cannot_meet\"").contains("exactly one of"));
        assert!(ack("standing = \"cannot_meet\"\nplan = 1\nship = 2").contains("exactly one of"));
        // A key the kind does not take.
        assert!(set("interrupt = 1\nbase = 2\nbelow_days = 5\nplan = 1")
            .contains("does not take \"plan\""));
        assert!(ack("interrupt = 1\nbase = 2").contains("does not take \"base\""));
        // A mixture of the two forms.
        assert!(set(
            "interrupt = 1\nbase = 2\nbelow_days = 5\nstanding = \"ship_lost\"\non = true"
        )
        .contains("mixes the two forms"));
        assert!(set("interrupt = 1\nstanding = \"ship_lost\"\non = true")
            .contains("mixes the two forms"));
        assert!(ack("interrupt = 1\nstanding = \"cannot_meet\"\nplan = 1")
            .contains("mixes the two forms"));
        // An unknown standing kind.
        assert!(set("standing = \"cannot_swim\"\non = true").contains("not cannot_meet"));
        assert!(ack("standing = \"cannot_swim\"\nplan = 1").contains("not cannot_meet"));
        // A TOML float.
        assert!(parse_with(&command(
            "kind = \"set_interrupt\"\ninterrupt = 1\nbase = 2\nbelow_days = 5.5"
        ))
        .is_err());
        assert!(parse_with(&command("kind = \"acknowledge\"\ninterrupt = 1.5")).is_err());
    }

    #[test]
    fn commands_are_kept_by_day_in_the_order_given_and_survive_a_new_seed() {
        let text = format!(
            "{}{}",
            command("kind = \"approve\"\nplan = 1"),
            "\n[[command]]\nday = 2\nside = \"japanese\"\nkind = \"withdraw_intent\"\nintent = 4\n\n[[command]]\nday = 1\nside = \"japanese\"\nkind = \"cancel_plan\"\nplan = 2\n"
        );
        let scenario = parse_with(&text).expect("loads");
        assert_eq!(scenario.scripted_commands().len(), 2);
        let first: Vec<Side> = scenario.scripted_commands()[&1]
            .iter()
            .map(|(side, _)| *side)
            .collect();
        assert_eq!(first, [Side::Allied, Side::Japanese]);
        let again = scenario.with_seed(77).expect("a new seed");
        assert_eq!(again.scripted_commands(), scenario.scripted_commands());
        // A scenario with none has none.
        assert!(parse_with("")
            .expect("loads")
            .scripted_commands()
            .is_empty());
    }

    /// Decision 0218, stage 3: the patrol order, a hold on a boat and a
    /// forces limit naming boats are given by a scenario's commands.
    #[test]
    fn the_patrol_order_a_hold_on_a_boat_and_a_limit_naming_boats_load() {
        use sim_core::command::{ForcesLimit, HoldTarget, IntentKind, Limit};
        use sim_core::ids::SubmarineId;
        assert_eq!(
            only(&command(
                "kind = \"set_intent\"\nintent = 3\nintent_kind = \"patrol\"\nhex = [5, -2]\ndistance = 4"
            )),
            CommandKind::SetIntent {
                intent: IntentId::new(3),
                kind: IntentKind::Patrol {
                    hex: Hex::new(5, -2),
                    distance: 4
                }
            }
        );
        // A patrol takes a hex and a distance and no base.
        for body in [
            "intent_kind = \"patrol\"\nhex = [5, -2]",
            "intent_kind = \"patrol\"\ndistance = 4",
            "intent_kind = \"patrol\"\nhex = [5, -2]\ndistance = 4\nbase = 1",
        ] {
            assert!(
                parse_with(&command(&format!(
                    "kind = \"set_intent\"\nintent = 3\n{body}"
                )))
                .is_err(),
                "{body}"
            );
        }
        for (kind, make) in [("hold", true), ("release", false)] {
            let target = HoldTarget::Submarine(SubmarineId::new(6));
            assert_eq!(
                only(&command(&format!("kind = \"{kind}\"\nsubmarine = 6"))),
                if make {
                    CommandKind::Hold { target }
                } else {
                    CommandKind::Release { target }
                }
            );
        }
        // A hold names exactly one target.
        assert!(parse_with(&command("kind = \"hold\"\nsubmarine = 6\nship = 1")).is_err());
        assert!(parse_with(&command(
            "kind = \"hold\"\nsubmarine = 6\nsetting = \"search_arc\""
        ))
        .is_err());
        assert_eq!(
            only(&command(
                "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 3\nsubmarines = [2, 6]"
            )),
            CommandKind::SetLimit(Limit::Forces {
                intent: IntentId::new(3),
                limit: ForcesLimit {
                    ships: Default::default(),
                    task_forces: Default::default(),
                    air_groups: Default::default(),
                    source: None,
                    land_units: Default::default(),
                    submarines: [SubmarineId::new(2), SubmarineId::new(6)]
                        .into_iter()
                        .collect(),
                    none_of: Default::default(),
                    in_place: false,
                }
            })
        );
    }

    #[test]
    fn a_command_with_a_wrong_key_or_value_is_refused() {
        let ok = "kind = \"set_intent\"\nintent = 1\nbase = 2\nfloor_days = 10";
        parse_with(&command(ok)).expect("loads");
        // A key no command has, and a key the kind does not take.
        assert!(parse_with(&command(&format!("{ok}\nspeed = 3"))).is_err());
        assert!(error_of(&command(&format!("{ok}\nplan = 3"))).contains("does not take \"plan\""));
        // A TOML float is refused for a number of days and for a weight.
        assert!(parse_with(&command(
            &ok.replace("floor_days = 10", "floor_days = 10.5")
        ))
        .is_err());
        assert!(parse_with(&command(
            "kind = \"set_limit\"\nlimit = \"reserve\"\nbase = 1\nsupply_t = 4.5\nfuel_t = 3"
        ))
        .is_err());
        // A missing key.
        assert!(
            error_of(&command("kind = \"set_intent\"\nintent = 1\nbase = 2"))
                .contains("needs \"floor_days\"")
        );
        assert!(error_of(&command("kind = \"approve\"")).contains("needs \"plan\""));
        // A cover or hold intent takes no days; a kind that is none is refused.
        for name in ["cover", "hold"] {
            let with = |extra: &str| {
                error_of(&command(&format!(
                    "kind = \"set_intent\"\nintent = 1\nintent_kind = \"{name}\"\nbase = 2\n{extra}"
                )))
            };
            assert!(with("floor_days = 10").contains("does not take \"floor_days\""));
            assert!(with("interval_days = 5").contains("does not take \"interval_days\""));
        }
        assert!(error_of(&command(
            "kind = \"set_intent\"\nintent = 1\nintent_kind = \"win\"\nbase = 2"
        ))
        .contains(
            "not keep_supplied, cover, hold, neutralise, raid, deny, strike_ships, search_waters, stand_at, operate_carriers, ready_landing, build_up, take_place, control_island or bombard"
        ));
        // Neutralise and raid name a target; a neutralise takes no base.
        let aimed = |kind: &str, extra: &str| {
            command(&format!(
                "kind = \"set_intent\"\nintent = 4\nintent_kind = \"{kind}\"\n{extra}"
            ))
        };
        assert_eq!(
            only(&aimed("neutralise", "target = 9")),
            CommandKind::SetIntent {
                intent: IntentId::new(4),
                kind: sim_core::command::IntentKind::Neutralise {
                    target: BaseId::new(9)
                },
            }
        );
        assert_eq!(
            only(&aimed("raid", "base = 2\ntarget = 9")),
            CommandKind::SetIntent {
                intent: IntentId::new(4),
                kind: sim_core::command::IntentKind::Raid {
                    base: BaseId::new(2),
                    target: BaseId::new(9)
                },
            }
        );
        assert_eq!(
            only(&aimed("deny", "hex = [3, -1]\ndistance = 4")),
            CommandKind::SetIntent {
                intent: IntentId::new(4),
                kind: sim_core::command::IntentKind::Deny {
                    hex: sim_core::hex::Hex::new(3, -1),
                    distance: 4
                },
            }
        );
        assert_eq!(
            only(&aimed("strike_ships", "base = 2")),
            CommandKind::SetIntent {
                intent: IntentId::new(4),
                kind: sim_core::command::IntentKind::StrikeShips {
                    base: BaseId::new(2)
                },
            }
        );
        assert_eq!(
            only(&aimed("search_waters", "base = 2")),
            CommandKind::SetIntent {
                intent: IntentId::new(4),
                kind: sim_core::command::IntentKind::SearchWaters {
                    base: BaseId::new(2)
                },
            }
        );
        assert!(error_of(&aimed("deny", "distance = 4")).contains("needs \"hex\""));
        assert!(error_of(&aimed("deny", "hex = [3, -1]")).contains("needs \"distance\""));
        assert!(
            error_of(&aimed("deny", "hex = [3, -1]\ndistance = 4\nbase = 2"))
                .contains("does not take \"base\"")
        );
        assert!(error_of(&aimed("neutralise", "base = 2\ntarget = 9"))
            .contains("does not take \"base\""));
        assert!(error_of(&aimed("neutralise", "")).contains("needs \"target\""));
        assert!(error_of(&aimed("raid", "base = 2")).contains("needs \"target\""));
        assert!(error_of(&aimed("raid", "target = 9")).contains("needs \"base\""));
        assert!(
            error_of(&aimed("cover", "base = 2\ntarget = 9")).contains("does not take \"target\"")
        );
        // A kind that is the staff's, or none, and a limit that is none.
        assert!(error_of(&command("kind = \"commit_ships\"\nplan = 1"))
            .contains("not set_intent, withdraw_intent"));
        assert!(error_of(&command("kind = \"set_limit\"\nintent = 1")).contains("needs a limit"));
        assert!(error_of(&command(
            "kind = \"set_limit\"\nlimit = \"speed\"\nintent = 1"
        ))
        .contains("not forces, area"));
        assert!(error_of(&command(
            "kind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"rash\""
        ))
        .contains("not preserve, balanced or press"));
        assert!(error_of(&command(
            "kind = \"set_leave\"\nleave = \"sail\"\ngiven = true"
        ))
        .contains("not prepare or launch"));
        // A day outside the scenario's days.
        assert!(error_of(
            "\n[[command]]\nday = 3\nside = \"allied\"\nkind = \"approve\"\nplan = 1\n"
        )
        .contains("outside days 1 to 2"));
        assert!(error_of(
            "\n[[command]]\nday = 0\nside = \"allied\"\nkind = \"approve\"\nplan = 1\n"
        )
        .contains("outside days 1 to 2"));
    }

    #[test]
    fn a_scenarios_commands_are_taken_in_on_their_day() {
        // An intent set on day 1 is in force after it, and one withdrawn on
        // day 2 is withdrawn.
        let text = format!(
            "{}{}",
            command("kind = \"set_intent\"\nintent = 1\nbase = 1\nfloor_days = 10"),
            "\n[[command]]\nday = 2\nside = \"allied\"\nkind = \"withdraw_intent\"\nintent = 1\n"
        );
        let mut scenario = parse_with(&text).expect("loads");
        scenario.next_day().expect("day 1");
        let state = scenario.world.command_state(Side::Allied);
        assert!(state.intents[&IntentId::new(1)].in_force());
        assert!(scenario.scripted_commands().get(&1).is_none());
        scenario.next_day().expect("day 2");
        let state = scenario.world.command_state(Side::Allied);
        assert_eq!(state.intents[&IntentId::new(1)].withdrawn_day, Some(2));
        assert!(scenario.world.command_state(Side::Japanese).is_empty());
    }

    // -----------------------------------------------------------------
    // What a side believed, and whose a class is (decision 0173).
    // -----------------------------------------------------------------

    /// Writes a file of sides with `rows` under the system's temporary
    /// folder and returns its absolute path, which a scenario may name.
    fn sides_file(rows: &str) -> PathBuf {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "flagplot_class_sides_{}_{}.csv",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::write(&path, format!("table,id,side,note\n{rows}")).expect("write the file of sides");
        path
    }

    /// The rows of a whole file of sides for the placeholder classes and
    /// types of `HEAD` (the record's statement 1): class 1 the Allied side's,
    /// every other class and every type both sides'.
    const PLACEHOLDER_SIDES: &str = "ship_class,1,allied,test\n\
        ship_class,2,both,test\n\
        ship_class,3,both,test\n\
        ship_class,4,both,test\n\
        ship_class,5,both,test\n\
        ship_class,6,both,test\n\
        ship_class,8,both,test\n\
        aircraft_type,1,both,test\n\
        aircraft_type,2,both,test\n\
        aircraft_type,3,both,test\n\
        aircraft_type,4,both,test\n";

    /// `HEAD` naming the file of sides `rows`, plus `extra`.
    fn parse_with_sides(rows: &str, extra: &str) -> Result<Scenario, DataError> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        let sides = sides_file(rows);
        let head = HEAD.replacen(
            "days = 2\n",
            &format!("days = 2\nclass_sides = \"{}\"\n", sides.display()),
            1,
        );
        let result = Scenario::parse(&format!("{head}{extra}"), &path);
        fs::remove_file(&sides).expect("remove the file of sides");
        result
    }

    /// Statement 1: a file of sides, by hand.
    #[test]
    fn a_file_of_sides_says_whose_a_class_is_and_is_refused_where_it_is_not_whole_or_true() {
        use sim_core::refdata::ClassSide;
        let scenario = parse_with_sides(PLACEHOLDER_SIDES, "").expect("loads");
        let ref_data = scenario.world.ref_data();
        assert_eq!(
            ref_data.ship_class_side(ShipClassId::new(1)),
            Some(ClassSide::Allied)
        );
        assert_eq!(
            ref_data.ship_class_side(ShipClassId::new(2)),
            Some(ClassSide::Both)
        );
        assert_eq!(
            ref_data.aircraft_type_side(AircraftTypeId::new(3)),
            Some(ClassSide::Both)
        );
        // A side's view reads it as it reads a class.
        let view = scenario.world.side_view(Side::Japanese);
        assert_eq!(
            view.ship_class_side(ShipClassId::new(1)),
            Some(ClassSide::Allied)
        );
        assert_eq!(
            view.ship_class_side(ShipClassId::new(2)),
            Some(ClassSide::Both)
        );
        assert_eq!(
            view.aircraft_type_side(AircraftTypeId::new(4)),
            Some(ClassSide::Both)
        );
        // The file is one of the scenario's data files, last.
        assert!(scenario
            .data_files()
            .last()
            .expect("files")
            .to_string_lossy()
            .contains("flagplot_class_sides_"));
        // A scenario that names no file has none.
        let plain = parse_with("").expect("loads");
        assert_eq!(plain.world.ref_data().class_sides, None);
        assert_eq!(
            plain
                .world
                .side_view(Side::Allied)
                .ship_class_side(ShipClassId::new(1)),
            None
        );

        let refused = |rows: &str, extra: &str| -> String {
            parse_with_sides(rows, extra)
                .expect_err("the scenario is refused")
                .message
        };
        // A class the files give with no row.
        let message = refused(
            &PLACEHOLDER_SIDES.replace("ship_class,4,both,test\n", ""),
            "",
        );
        assert!(
            message.contains("ship class 4 has no row in the file of sides"),
            "{message}"
        );
        // A type the files give with no row.
        let message = refused(
            &PLACEHOLDER_SIDES.replace("aircraft_type,2,both,test\n", ""),
            "",
        );
        assert!(
            message.contains("aircraft type 2 has no row in the file of sides"),
            "{message}"
        );
        // A row for a class they do not give.
        let message = refused(&format!("{PLACEHOLDER_SIDES}ship_class,99,both,test\n"), "");
        assert!(
            message.contains("gives ship class 99, which the scenario's class files do not give"),
            "{message}"
        );
        // A class given twice.
        let message = refused(
            &format!("{PLACEHOLDER_SIDES}ship_class,2,japanese,test\n"),
            "",
        );
        assert!(message.contains("ship_class 2 is given twice"), "{message}");
        // A Japanese ship of class 1.
        let japanese_force = "[[task_force]]\nid = 1\nname = \"TF\"\nside = \"japanese\"\nat_base = 2\nmission = \"escort\"\n";
        let japanese_ship = "[[ship]]\nid = 1\nname = \"S\"\nclass = 1\nside = \"japanese\"\nfuel_t = 100\ntask_force = 1\n";
        let message = refused(
            PLACEHOLDER_SIDES,
            &format!("\n{japanese_force}\n{japanese_ship}"),
        );
        assert!(
            message.contains("ship 1 (S) is Japanese and of ship class 1, which the file of sides gives to the allied side alone"),
            "{message}"
        );
        // The same ship of class 2, both sides', loads.
        parse_with_sides(
            PLACEHOLDER_SIDES,
            &format!(
                "\n{japanese_force}\n{}",
                japanese_ship.replace("class = 1", "class = 2")
            ),
        )
        .expect("a class of both sides loads");
        // A side and a table the file does not know, and an empty note.
        let message = refused(
            &PLACEHOLDER_SIDES.replace("1,allied,test", "1,french,test"),
            "",
        );
        assert!(message.contains("unknown side \"french\""), "{message}");
        let message = refused(&format!("{PLACEHOLDER_SIDES}land_unit,1,both,test\n"), "");
        assert!(message.contains("unknown table \"land_unit\""), "{message}");
        let message = refused(&PLACEHOLDER_SIDES.replace("1,allied,test", "1,allied,"), "");
        assert!(message.contains("the note is empty"), "{message}");
    }

    fn believed_force(body: &str) -> String {
        format!("\n[[believed_force]]\nside = \"allied\"\nname = \"A force\"\nkind = \"carrier\"\n{body}\n")
    }

    /// Statement 2: a believed force.
    #[test]
    fn a_believed_force_is_kept_as_written_and_refused_where_it_cannot_be() {
        use sim_core::believed::{LastKnown, Standing};
        let text = format!(
            "{}{}{}",
            believed_force("ships = 6\nstanding = \"present\"\nat_base = 2\nplaceholder = true\nnote = \"Rests on no source.\""),
            believed_force("ships = 8\nstanding = \"unknown\"").replace("A force", "Carriers"),
            believed_force("ships = 1\nstanding = \"absent\"\nq = 7\nr = -1\nday = 2")
                .replace("carrier", "cruiser")
                .replace("A force", "Ruled out"),
        );
        let scenario = parse_with(&text).expect("loads");
        let forces: Vec<_> = scenario.believed_forces(Side::Allied).collect();
        assert_eq!(forces.len(), 3);
        // Present, at a base: kept with that base's hex, before the first day.
        assert_eq!(forces[0].side, Side::Allied);
        assert_eq!(forces[0].enemy(), Side::Japanese);
        assert_eq!(forces[0].name, "A force");
        assert_eq!(forces[0].kind, ShipKind::Carrier);
        assert_eq!(forces[0].ships, 6);
        assert_eq!(forces[0].standing, Standing::Present);
        assert_eq!(
            forces[0].last_known,
            LastKnown::AtBase {
                base: BaseId::new(2),
                hex: Hex::new(5, 0),
                day: 0
            }
        );
        assert!(forces[0].placeholder);
        assert_eq!(forces[0].note.as_deref(), Some("Rests on no source."));
        // Unknown, where not known.
        assert_eq!(forces[1].name, "Carriers");
        assert_eq!(forces[1].ships, 8);
        assert_eq!(forces[1].standing, Standing::Unknown);
        assert_eq!(forces[1].last_known, LastKnown::NotKnown);
        assert!(!forces[1].placeholder);
        assert_eq!(forces[1].note, None);
        // Absent, at a hex on a day.
        assert_eq!(forces[2].kind, ShipKind::Cruiser);
        assert_eq!(forces[2].standing, Standing::Absent);
        assert_eq!(
            forces[2].last_known,
            LastKnown::AtHex {
                hex: Hex::new(7, -1),
                day: 2
            }
        );
        // The other side believes none; they are kept for another seed.
        assert_eq!(scenario.believed_forces(Side::Japanese).count(), 0);
        let other = scenario.with_seed(77).expect("copies");
        assert_eq!(
            other
                .believed_forces(Side::Allied)
                .cloned()
                .collect::<Vec<_>>(),
            scenario
                .believed_forces(Side::Allied)
                .cloned()
                .collect::<Vec<_>>()
        );

        let refused = |body: &str| error_of(&believed_force(body));
        for (body, words) in [
            ("ships = 0\nstanding = \"present\"", "has no ship"),
            (
                "ships = 2\nstanding = \"likely\"",
                "unknown standing \"likely\"",
            ),
            (
                "ships = 2\nstanding = \"present\"\nat_base = 1",
                "the Allied side's own",
            ),
            (
                "ships = 2\nstanding = \"present\"\nq = 99\nr = 0",
                "at 99,0, off the map",
            ),
            (
                "ships = 2\nstanding = \"present\"\nat_base = 2\nq = 5\nr = 0",
                "both \"at_base\" and a hex",
            ),
            (
                "ships = 2\nstanding = \"present\"\nat_base = 2\nday = 3",
                "on day 3, after the scenario's last, day 2",
            ),
            (
                "ships = 2\nstanding = \"present\"\nspeed = 3",
                "unknown field `speed`",
            ),
            (
                "ships = 2\nstanding = \"present\"\nplaceholder = true",
                "no note of what it rests on",
            ),
            (
                "ships = 2\nstanding = \"present\"\nday = 1",
                "gives a day and no place",
            ),
            (
                "ships = 2\nstanding = \"present\"\nq = 3",
                "a hex takes both",
            ),
            (
                "ships = 2\nstanding = \"present\"\nat_base = 9",
                "not a base of the scenario",
            ),
        ] {
            let message = refused(body);
            assert!(message.contains(words), "{body}: {message}");
        }
        // An unknown kind.
        let message = error_of(
            &believed_force("ships = 2\nstanding = \"present\"").replace("carrier", "dreadnought"),
        );
        assert!(
            message.contains("unknown kind \"dreadnought\""),
            "{message}"
        );
        // A base off the believing side's map.
        let hidden = HEAD.replacen("q = 5\nr = 0\n", "q = 5\nr = 0\non_map = false\n", 1);
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        let message = Scenario::parse(
            &format!(
                "{hidden}{}",
                believed_force("ships = 2\nstanding = \"present\"\nat_base = 2")
            ),
            &path,
        )
        .expect_err("refused")
        .message;
        assert!(
            message.contains("not on the Allied side's map"),
            "{message}"
        );
        // A side it does not know.
        assert!(parse_with(
            &believed_force("ships = 2\nstanding = \"present\"")
                .replace("\"allied\"", "\"french\"")
        )
        .is_err());
    }

    fn believed_intent(body: &str) -> String {
        format!("\n[[believed_intent]]\nside = \"allied\"\n{body}\n")
    }

    /// Statement 3: a believed intent.
    #[test]
    fn a_believed_intent_is_checked_as_the_enemys_command_on_the_believing_sides_map() {
        use sim_core::command::IntentKind;
        let scenario = parse_with(&believed_intent(
            "intent_kind = \"hold\"\nbase = 2\nplaceholder = true\nnote = \"Rests on no source.\"",
        ))
        .expect("loads");
        let intents: Vec<_> = scenario.believed_intents(Side::Allied).collect();
        assert_eq!(intents.len(), 1);
        assert_eq!(intents[0].side, Side::Allied);
        assert_eq!(intents[0].enemy(), Side::Japanese);
        assert_eq!(
            intents[0].kind,
            IntentKind::Hold {
                base: BaseId::new(2)
            }
        );
        assert!(intents[0].placeholder);
        assert_eq!(scenario.believed_intents(Side::Japanese).count(), 0);
        // It is no command: neither side's command state holds an intent.
        assert!(scenario.world.command_state(Side::Japanese).is_empty());
        assert!(scenario.scripted_commands().is_empty());

        // An intent naming a base not on the Allied side's map.
        let hidden = HEAD.replacen("q = 5\nr = 0\n", "q = 5\nr = 0\non_map = false\n", 1);
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        let message = Scenario::parse(
            &format!(
                "{hidden}{}",
                believed_intent("intent_kind = \"hold\"\nbase = 2")
            ),
            &path,
        )
        .expect_err("refused")
        .message;
        assert!(
            message.contains("names base 2, which is not on the Allied side's map"),
            "{message}"
        );
        // A kind only the staff may give.
        let message = error_of(&believed_intent(
            "intent_kind = \"search_waters\"\nbase = 2",
        ));
        assert!(
            message.contains("could not be given to the Japanese side"),
            "{message}"
        );
        // An unknown key, and a key the kind does not take.
        let message = error_of(&believed_intent(
            "intent_kind = \"hold\"\nbase = 2\nday = 1",
        ));
        assert!(message.contains("unknown field `day`"), "{message}");
        let message = error_of(&believed_intent(
            "intent_kind = \"hold\"\nbase = 2\ntarget = 1",
        ));
        assert!(message.contains("does not take \"target\""), "{message}");
        // An intent the enemy could not be given: a supply intent for a base
        // that is not his, a neutralise of his own base, no kind.
        let message = error_of(&believed_intent(
            "intent_kind = \"keep_supplied\"\nbase = 1\nfloor_days = 3",
        ));
        assert!(
            message.contains("could not be given to the Japanese side"),
            "{message}"
        );
        let message = error_of(&believed_intent("intent_kind = \"neutralise\"\ntarget = 2"));
        assert!(
            message.contains("could not be given to the Japanese side"),
            "{message}"
        );
        let message = error_of(&believed_intent("base = 2"));
        assert!(message.contains("needs \"intent_kind\""), "{message}");
        // A hold of the believing side's own base, on both maps, loads.
        parse_with(&believed_intent("intent_kind = \"hold\"\nbase = 1")).expect("loads");
    }

    /// The architect's ruling on reading 10 of decision 0173's "As built": a
    /// believed intent may not name a force of the enemy's, and the check of
    /// the kinds left reads nothing of the enemy's units.
    #[test]
    fn a_believed_intent_names_no_enemy_force_and_reads_none() {
        for body in [
            "intent_kind = \"stand_at\"\ntask_force = 1\nhex = [3, 0]",
            "intent_kind = \"operate_carriers\"\ntask_force = 1\nhex = [3, 0]",
            "intent_kind = \"ready_landing\"\ntask_force = 1\nhex = [0, 0]",
            "intent_kind = \"bombard\"\ntask_force = 1\ntarget = 1",
            // Refused by the kind alone, whatever the keys.
            "intent_kind = \"bombard\"",
        ] {
            let message = error_of(&believed_intent(body));
            assert!(
                message.contains("a side cannot name a force of the enemy's"),
                "{body}: {message}"
            );
        }
        // Two scenarios that differ only in the enemy's forces: the second
        // has one more Japanese task force, with a ship. Both load the same
        // believed intents, of each kind that names a base, a hex or an
        // island.
        let intents = format!(
            "{}{}{}{}{}{}{}{}{}{}",
            believed_intent("intent_kind = \"cover\"\nbase = 2"),
            believed_intent("intent_kind = \"strike_ships\"\nbase = 2"),
            believed_intent("intent_kind = \"build_up\"\nbase = 2\nwants = [\"airfield\"]"),
            believed_intent("intent_kind = \"hold\"\nbase = 2"),
            believed_intent("intent_kind = \"hold\"\nbase = 1"),
            believed_intent("intent_kind = \"keep_supplied\"\nbase = 2\nfloor_days = 3"),
            believed_intent("intent_kind = \"neutralise\"\ntarget = 1"),
            believed_intent("intent_kind = \"raid\"\nbase = 2\ntarget = 1"),
            believed_intent("intent_kind = \"deny\"\nhex = [3, 0]\ndistance = 2"),
            believed_intent("intent_kind = \"take_place\"\nhex = [0, 0]"),
        );
        let enemy_force = "\n[[task_force]]\nid = 7\nname = \"Enemy TF\"\nside = \"japanese\"\nat_base = 2\nmission = \"escort\"\n\n[[ship]]\nid = 7\nname = \"Enemy ship\"\nclass = 1\nside = \"japanese\"\nfuel_t = 100\ntask_force = 7\n";
        let without = parse_with(&intents).expect("loads without the force");
        let with = parse_with(&format!("{intents}{enemy_force}")).expect("loads with the force");
        let kept = |scenario: &Scenario| -> Vec<BelievedIntent> {
            scenario.believed_intents(Side::Allied).cloned().collect()
        };
        assert_eq!(kept(&without).len(), 10);
        assert_eq!(kept(&without), kept(&with));
    }

    /// Statement 5, in part: entries and a file of sides change no game.
    /// The world loaded with them is the world loaded without them, hash and
    /// log alike.
    #[test]
    fn believed_entries_and_a_file_of_sides_change_nothing_in_the_world() {
        let plain = parse_with("").expect("loads");
        let with = parse_with_sides(
            PLACEHOLDER_SIDES,
            &format!(
                "{}{}",
                believed_force("ships = 6\nstanding = \"present\"\nat_base = 2"),
                believed_intent("intent_kind = \"hold\"\nbase = 2")
            ),
        )
        .expect("loads");
        assert_eq!(plain.world.state_hash(), with.world.state_hash());
        assert_eq!(plain.world.events().len(), with.world.events().len());
    }

    // Forces that arrive (decision 0195): the key `enters`.

    /// `HEAD` for a game of six days with a ground file, then `extra`.
    fn arriving(extra: &str) -> Result<Scenario, DataError> {
        arriving_on("placeholder_search", "", extra)
    }

    /// As `arriving`, on the named map, with `top` added to the top keys.
    fn arriving_on(map: &str, top: &str, extra: &str) -> Result<Scenario, DataError> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        let head = HEAD
            .replace("placeholder_search", map)
            .replace("days = 2\n", &format!("days = 2\n{top}"))
            .replace("days = 2\n", "days = 6\n")
            .replace(
                "ship_classes = \"../reference/ship_classes.csv\"",
                "ship_classes = [\"../reference/ship_classes.csv\", \"../reference/ship_classes_placeholder_transports.csv\"]",
            )
            .replace("search = ", "ground = \"../reference/ground.toml\"\nsearch = ");
        Scenario::parse(&format!("{head}{extra}"), &path)
    }

    fn arriving_error(extra: &str) -> String {
        arriving(extra)
            .expect_err("the scenario is refused")
            .message
    }

    /// Allied task force 1 (ships 1 and 2, a carrier with air group 1
    /// aboard, 400 men of land unit 1 aboard) with `force_extra`.
    fn arriving_force(force_extra: &str) -> String {
        format!(
            "\n[[task_force]]\nid = 1\nname = \"Force\"\nside = \"allied\"\nq = 3\nr = 0\nmission = \"transport\"\n{force_extra}\n\
             [[ship]]\nid = 1\nname = \"S1\"\nclass = 10\nside = \"allied\"\nfuel_t = 100\ntask_force = 1\n\
             [[ship]]\nid = 2\nname = \"S2\"\nclass = 4\nside = \"allied\"\nfuel_t = 100\ntask_force = 1\n\
             [[air_group]]\nid = 1\nname = \"AG1\"\nside = \"allied\"\nship = 2\naircraft_type = 1\nready = 4\n\
             [[land_unit]]\nid = 1\nname = \"LU1\"\nside = \"allied\"\nmen = 400\ntask_force = 1\n"
        )
    }

    const ARRIVING_GROUP_2: &str = "\n[[air_group]]\nid = 2\nname = \"AG2\"\nside = \"allied\"\nbase = 1\naircraft_type = 1\nready = 3\n";
    const ARRIVING_UNIT_2: &str =
        "\n[[land_unit]]\nid = 2\nname = \"LU2\"\nside = \"allied\"\nmen = 300\nq = 0\nr = 0\n";

    #[test]
    fn units_that_enter_are_held_apart_and_a_seeded_copy_holds_the_same() {
        let text = format!(
            "{}{}{}\n[[arrival]]\nday = 3\nair_group = 2\naircraft = 2\n\n[[arrival]]\nday = 3\nair_group = 1\naircraft = 1\n",
            arriving_force("enters = 3"),
            ARRIVING_GROUP_2.replace("ready = 3", "ready = 3\nenters = 3"),
            ARRIVING_UNIT_2.replace("r = 0", "r = 0\nenters = 3"),
        );
        let scenario = arriving(&text).expect("loads");
        let truth = scenario.world.umpire();
        let keys: Vec<EntityRef> = truth.entering.keys().copied().collect();
        assert_eq!(
            keys,
            vec![
                EntityRef::TaskForce(TaskForceId::new(1)),
                EntityRef::AirGroup(AirGroupId::new(2)),
                EntityRef::LandUnit(LandUnitId::new(2)),
            ]
        );
        assert!(truth.task_forces.is_empty());
        assert!(truth.ships.is_empty());
        assert!(truth.air_groups.is_empty());
        assert!(truth.land_units.is_empty());
        let force = &truth.entering[&keys[0]];
        assert_eq!(force.day, 3);
        assert_eq!(force.ships.len(), 2);
        assert_eq!(force.air_groups.len(), 1);
        assert_eq!(force.land_units.len(), 1);
        assert_eq!(force.land_units[&LandUnitId::new(1)].men_ashore, 0);
        let other = scenario.with_seed(9).expect("a seeded copy");
        assert_eq!(
            other
                .world
                .umpire()
                .entering
                .keys()
                .copied()
                .collect::<Vec<_>>(),
            keys
        );
        assert!(other.world.umpire().task_forces.is_empty());
        // A scenario with no `enters` holds nothing.
        let plain = arriving(&arriving_force("")).expect("loads");
        assert!(plain.world.umpire().entering.is_empty());
        assert_eq!(plain.world.umpire().ships.len(), 2);
    }

    #[test]
    fn an_arrival_dated_on_its_groups_day_is_taken_and_one_before_is_refused() {
        let group = ARRIVING_GROUP_2.replace("ready = 3", "ready = 3\nenters = 4");
        let arrival = |day: u32, group: u32| {
            format!("\n[[arrival]]\nday = {day}\nair_group = {group}\naircraft = 1\n")
        };
        let ok = arriving(&format!("{group}{}", arrival(4, 2))).expect("on the day");
        assert_eq!(ok.world.umpire().arrivals.len(), 1);
        let message = arriving_error(&format!("{group}{}", arrival(3, 2)));
        assert!(
            message.contains("before air group 2 enters on day 4"),
            "{message}"
        );
        // A group aboard a ship takes its force's day.
        let force = arriving_force("enters = 5");
        let message = arriving_error(&format!("{force}{}", arrival(4, 1)));
        assert!(
            message.contains("before air group 1 enters on day 5"),
            "{message}"
        );
        arriving(&format!("{force}{}", arrival(5, 1))).expect("on the force's day");
    }

    #[test]
    fn enters_is_refused_with_its_reason() {
        for (extra, words) in [
            (
                arriving_force("enters = 1"),
                "task force 1 has enters = 1: a unit there on day 1 needs no `enters`",
            ),
            (arriving_force("enters = 0"), "has enters = 0"),
            (
                arriving_force("enters = 7"),
                "enters = 7, after the last day of the scenario (6)",
            ),
            (
                arriving_force("").replace(
                    "fuel_t = 100\ntask_force = 1\n[[ship]]",
                    "fuel_t = 100\ntask_force = 1\nenters = 3\n[[ship]]",
                ),
                "ship 1 has `enters`: a ship enters with its task force",
            ),
            (
                arriving_force("").replace("ready = 4\n", "ready = 4\nenters = 3\n"),
                "air group 1 is aboard a ship and has `enters`",
            ),
        ] {
            let message = arriving_error(&extra);
            assert!(message.contains(words), "{words}: {message}");
        }
        let aboard = arriving_force("").replace("men = 400\n", "men = 400\nenters = 3\n");
        let message = arriving_error(&aboard);
        assert!(
            message.contains("land unit 1 is aboard a task force and has `enters`"),
            "{message}"
        );
        // Group and unit at a base: below 2 and above the days.
        let message =
            arriving_error(&ARRIVING_GROUP_2.replace("ready = 3", "ready = 3\nenters = 1"));
        assert!(message.contains("air group 2 has enters = 1"), "{message}");
        let message = arriving_error(&ARRIVING_UNIT_2.replace("r = 0", "r = 0\nenters = 9"));
        assert!(message.contains("land unit 2 has enters = 9"), "{message}");
    }

    #[test]
    fn a_force_enters_at_sea_or_coast_on_the_map_only() {
        let off = arriving_force("enters = 3").replace("q = 3", "q = 500");
        let message = arriving_error(&off);
        assert!(
            message.contains("task force 1 enters on day 3 at 500,0, which is off the map"),
            "{message}"
        );
        // A land hex, on a map with land.
        let land = arriving_force("enters = 3").replace("q = 3", "q = 4");
        let message = arriving_on("placeholder_island", "", &land)
            .expect_err("refused")
            .message;
        assert!(
            message.contains("task force 1 enters on day 3 at 4,0, which is land"),
            "{message}"
        );
        let sea = arriving_force("enters = 3");
        arriving_on("placeholder_island", "", &sea).expect("sea is fine");
    }

    #[test]
    fn a_land_unit_that_enters_needs_a_base_in_its_hex() {
        // (4,0) is land on the island map, with no base.
        let unit = ARRIVING_UNIT_2.replace("q = 0\nr = 0", "q = 4\nr = 0\nenters = 3");
        let message = arriving_on("placeholder_island", "", &unit)
            .expect_err("refused")
            .message;
        assert!(message.contains("land_unit:2 enters on day 3"), "{message}");
        assert!(message.contains("where there is no base"), "{message}");
    }

    #[test]
    fn an_escort_and_its_charge_enter_on_one_day() {
        let second = |extra: &str| {
            format!(
                "\n[[task_force]]\nid = 2\nname = \"Escort\"\nside = \"allied\"\nq = 3\nr = 0\nmission = \"escort\"\n{extra}\n\
                 [[ship]]\nid = 9\nname = \"S9\"\nclass = 1\nside = \"allied\"\nfuel_t = 100\ntask_force = 2\n\
                 [[order]]\nday = 4\nside = \"allied\"\ntask_force = 2\nkind = \"escort\"\ncharge = 1\n"
            )
        };
        // Both enter on day 3.
        arriving(&format!(
            "{}{}",
            arriving_force("enters = 3"),
            second("enters = 3")
        ))
        .expect("one day");
        // Neither enters.
        arriving(&format!("{}{}", arriving_force(""), second(""))).expect("neither");
        let message = arriving_error(&format!("{}{}", arriving_force("enters = 3"), second("")));
        assert!(message.contains("task force 2, which is there from day 1, to escort task force 1, which enters on day 3"), "{message}");
        let message = arriving_error(&format!("{}{}", arriving_force(""), second("enters = 3")));
        assert!(message.contains("task force 2, which enters on day 3, to escort task force 1, which is there from day 1"), "{message}");
        let message = arriving_error(&format!(
            "{}{}",
            arriving_force("enters = 3"),
            second("enters = 4")
        ));
        assert!(
            message.contains("an escort and its charge enter on one day"),
            "{message}"
        );
    }

    #[test]
    fn a_held_force_may_have_a_commander_and_radio_silence_but_is_no_superior() {
        let levels =
            |extra: &str| arriving_on("placeholder_search", "command_levels = true\n", extra);
        let held = arriving_force("enters = 3\nradio_silence = true\nsuperior = { base = 1 }");
        let commander = "\n[[commander]]\nside = \"allied\"\ntask_force = 1\nname = \"Hold\"\ninitiative = \"bold\"\nnote = \"a placeholder\"\n";
        let scenario =
            levels(&format!("{held}{commander}")).expect("a held force may be commanded");
        let entry = &scenario.world.umpire().entering[&EntityRef::TaskForce(TaskForceId::new(1))];
        assert!(entry.radio_silent);
        assert_eq!(
            entry.commander.as_ref().map(|c| c.name.as_str()),
            Some("Hold")
        );
        assert!(entry.superior.is_some());
        let again = scenario.with_seed(3).expect("a seeded copy");
        assert!(
            again.world.umpire().entering[&EntityRef::TaskForce(TaskForceId::new(1))].radio_silent
        );
        // A held force named as another's superior is refused, with sim-core's reason.
        let other = "\n[[task_force]]\nid = 2\nname = \"Other\"\nside = \"allied\"\nat_base = 1\nmission = \"escort\"\nsuperior = { task_force = 1 }\n\
                     [[ship]]\nid = 9\nname = \"S9\"\nclass = 1\nside = \"allied\"\nfuel_t = 100\ntask_force = 2\n";
        let message = levels(&format!("{}{other}", arriving_force("enters = 3")))
            .expect_err("refused")
            .message;
        assert!(message.contains("command"), "{message}");
    }

    // Submarines (decision 0218): the three file keys and the `[[submarine]]`
    // table.

    const SUBMARINE_KEYS: &str = "submarine_classes = \"../reference/submarine_classes_placeholder_scenario_128.csv\"\nsubmarine_torpedoes = \"../reference/submarine_torpedoes_placeholder_scenario_128.csv\"\nsubmarines = \"../reference/submarines_placeholder_scenario_128.toml\"\n";

    /// `HEAD` with the submarine file keys given after the surface combat
    /// key and `extra` after it, read with `options`.
    fn with_submarines(
        keys: &str,
        extra: &str,
        options: &LoadOptions,
    ) -> Result<Scenario, DataError> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/test.toml");
        let head = HEAD.replace(
            "surface_combat = \"../reference/surface_combat.toml\"\n",
            &format!("surface_combat = \"../reference/surface_combat.toml\"\n{keys}"),
        );
        Scenario::parse_with(&format!("{head}{extra}"), &path, options)
    }

    fn submarine(extra: &str) -> String {
        format!(
            "[[submarine]]\nid = 1\nname = \"Boat\"\nside = \"japanese\"\nclass = 6900\nq = 5\nr = 0\nhome = 2\n{extra}"
        )
    }

    fn boat_of(extra: &str) -> sim_core::state::Submarine {
        let scenario = with_submarines(SUBMARINE_KEYS, &submarine(extra), &LoadOptions::default())
            .expect("loads");
        scenario
            .world
            .umpire()
            .submarines
            .get(&SubmarineId::new(1))
            .expect("the boat")
            .clone()
    }

    fn submarine_error(keys: &str, extra: &str) -> String {
        match with_submarines(keys, extra, &LoadOptions::default()) {
            Ok(_) => panic!("the scenario was accepted"),
            Err(error) => error.to_string(),
        }
    }

    #[test]
    fn the_three_submarine_keys_load_the_data_and_the_boat_with_its_defaults() {
        let scenario = with_submarines(SUBMARINE_KEYS, &submarine(""), &LoadOptions::default())
            .expect("loads");
        let data = scenario
            .world
            .ref_data()
            .submarines
            .as_ref()
            .expect("the data");
        let class = &data.classes[&SubmarineClassId::new(6900)];
        assert_eq!((class.salvo, class.salvoes_carried), (4, 2));
        assert_eq!(class.passage_speed_kn, Fixed::from_int(10));
        let torpedo = &data.torpedoes[&class.torpedo];
        assert_eq!(torpedo.damage, Fixed::from_int(200));
        assert_eq!(
            data.figures.hit_chance[&class.torpedo],
            Fixed::from_ratio(1, 2)
        );
        assert_eq!(data.figures.held_down_pulses, 2);
        assert_eq!(data.figures.attacks_per_hour, Fixed::from_ratio(2, 100));
        // All three are among the files the scenario records as its inputs.
        for name in [
            "submarine_classes_placeholder_scenario_128.csv",
            "submarine_torpedoes_placeholder_scenario_128.csv",
            "submarines_placeholder_scenario_128.toml",
        ] {
            assert!(
                scenario.data_files.iter().any(|file| file.ends_with(name)),
                "{name}"
            );
        }
        let boat = &scenario.world.umpire().submarines[&SubmarineId::new(1)];
        assert_eq!(boat.name, "Boat");
        assert_eq!(boat.side, Side::Japanese);
        assert_eq!(boat.class, SubmarineClassId::new(6900));
        assert_eq!((boat.hex, boat.station), (Hex::new(5, 0), Hex::new(5, 0)));
        assert_eq!(boat.passage, None);
        assert_eq!(boat.salvoes, 2);
        assert_eq!(boat.held_down_until, None);
        assert_eq!(boat.condition, SubmarineCondition::Sound);
        assert_eq!(boat.home, BaseId::new(2));
        assert_eq!(
            (boat.leave_day, boat.enters_day, boat.available_from),
            (None, None, None)
        );
        // Without the keys there is no submarine data.
        assert!(parse_with("")
            .expect("loads")
            .world
            .ref_data()
            .submarines
            .is_none());
    }

    #[test]
    fn a_scenario_names_all_three_submarine_files_or_none() {
        let words = submarine_error(
            &SUBMARINE_KEYS.replace("submarines = ", "# submarines = "),
            "",
        );
        assert!(words.contains("all of submarine_classes"), "{words}");
        let one = "submarines = \"../reference/submarines.toml\"\n";
        assert!(submarine_error(one, "").contains("all of submarine_classes"));
        // A boat with none of the keys is refused.
        let words = submarine_error("", &submarine(""));
        assert!(words.contains("no submarine_classes"), "{words}");
        // A file that is not there is named.
        let words = submarine_error(
            &SUBMARINE_KEYS.replace("scenario_128.toml", "nothing.toml"),
            "",
        );
        assert!(words.contains("nothing.toml"), "{words}");
    }

    #[test]
    fn the_class_and_torpedo_files_follow_the_reference_root_and_the_figures_do_not() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let frozen = with_submarines(
            SUBMARINE_KEYS,
            &submarine(""),
            &LoadOptions::frozen_step1(&root),
        )
        .expect("loads");
        let live = with_submarines(SUBMARINE_KEYS, &submarine(""), &LoadOptions::default())
            .expect("loads");
        assert_eq!(
            frozen.world.ref_data().submarines,
            live.world.ref_data().submarines
        );
        // The class and torpedo files are the frozen copies; the figures are
        // the scenario's own.
        for (name, folder) in [
            ("submarine_classes_placeholder_scenario_128.csv", "step1"),
            ("submarine_torpedoes_placeholder_scenario_128.csv", "step1"),
        ] {
            assert!(
                frozen
                    .data_files
                    .iter()
                    .any(|file| file.ends_with(format!("{folder}/{name}"))),
                "{name}"
            );
        }
        assert!(frozen
            .data_files
            .iter()
            .any(|file| file.ends_with("reference/submarines_placeholder_scenario_128.toml")));
    }

    #[test]
    fn a_boat_takes_a_station_a_condition_salvoes_and_days() {
        // A station that is not her hex: she starts on passage to it.
        let boat = boat_of("station_q = 7\nstation_r = 0\n");
        assert_eq!(boat.station, Hex::new(7, 0));
        let passage = boat.passage.expect("a passage");
        assert_eq!(
            (passage.to, passage.home, passage.arrived),
            (Hex::new(7, 0), false, false)
        );
        // Damaged: she starts going home, to the base's hex.
        let boat = boat_of("condition = \"damaged\"\n");
        assert_eq!(boat.condition, SubmarineCondition::Damaged);
        let passage = boat.passage.expect("a passage");
        assert_eq!((passage.to, passage.home), (Hex::new(5, 0), true));
        // Salvoes, at most her class's.
        assert_eq!(boat_of("salvoes = 1\n").salvoes, 1);
        // Days.
        let boat = boat_of("leave_day = 2\navailable_from = 2\nenters = 2\n");
        assert_eq!(
            (boat.leave_day, boat.available_from, boat.enters_day),
            (Some(2), Some(2), Some(2))
        );
        // Held down for that many pulses, the night counted as the first.
        for (pulses, until) in [
            (0, None),
            (1, Some((1, Pulse::Night))),
            (2, Some((1, Pulse::Morning))),
            (3, Some((1, Pulse::Afternoon))),
            (4, Some((2, Pulse::Night))),
        ] {
            assert_eq!(
                boat_of(&format!("held_down_pulses = {pulses}\n")).held_down_until,
                until,
                "{pulses} pulses"
            );
        }
    }

    #[test]
    fn a_bad_boat_is_refused_with_its_reason() {
        for (extra, words) in [
            ("salvoes = 3\n", "more than her class carries"),
            ("salvoes = 0\n", "submarine 1 has salvoes = 0"),
            ("condition = \"afloat\"\n", "sound or damaged"),
            ("enters = 1\n", "enters = 1"),
            ("enters = 3\n", "after the last day"),
            ("leave_day = 0\n", "leave_day = 0"),
            ("available_from = 0\n", "available_from = 0"),
            ("station_q = 7\n", "give both or neither"),
            ("station_r = 7\n", "give both or neither"),
            ("station_q = 99\nstation_r = 0\n", "station"),
        ] {
            let message = submarine_error(SUBMARINE_KEYS, &submarine(extra));
            assert!(message.contains(words), "{extra}: {message}");
        }
        // A class the file does not give.
        let message = submarine_error(
            SUBMARINE_KEYS,
            &submarine("").replace("class = 6900", "class = 1"),
        );
        assert!(message.contains("class 1"), "{message}");
        // A base that is not there.
        let message = submarine_error(
            SUBMARINE_KEYS,
            &submarine("").replace("home = 2", "home = 99"),
        );
        assert!(message.contains("home base"), "{message}");
        // The home base is required.
        let message = submarine_error(SUBMARINE_KEYS, &submarine("").replace("home = 2\n", ""));
        assert!(message.contains("home"), "{message}");
        // Two boats of one number.
        let message = submarine_error(
            SUBMARINE_KEYS,
            &format!("{}{}", submarine(""), submarine("")),
        );
        assert!(message.contains("given twice"), "{message}");
        // An unknown key.
        let message = submarine_error(SUBMARINE_KEYS, &submarine("speed = 5\n"));
        assert!(message.contains("speed"), "{message}");
    }
}
