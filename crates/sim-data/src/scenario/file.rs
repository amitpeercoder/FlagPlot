use crate::commands::{BelievedIntentSpec, CommandSpec};
use serde::Deserialize;

use super::air::{AirGroupSpec, ArrivalSpec, ReplacementSpec};
use super::bases::BaseSpec;
use super::beliefs::{
    BelievedForceSpec, BelievedMenSpec, ContactSpec, EstimateSpec, SuspectedWatersSpec,
};
use super::commanders::{CommanderSpec, SidesSpec};
use super::islands::IslandSpec;
use super::land::{FormationSpec, LandUnitSpec};
use super::objectives::ObjectivesSpec;
use super::orders::OrderSpec;
use super::ships::{ShipSpec, TaskForceSpec};
use super::submarines::SubmarineSpec;
use super::supply::{MenReplacementSpec, SupplyLineSpec};

/// One file name or a list of them (decision 0043).
#[derive(Deserialize)]
#[serde(untagged)]
pub(super) enum FileList {
    One(String),
    Many(Vec<String>),
}

impl FileList {
    pub(super) fn names(&self) -> Vec<&str> {
        match self {
            FileList::One(name) => vec![name.as_str()],
            FileList::Many(names) => names.iter().map(String::as_str).collect(),
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ScenarioFile {
    pub(super) name: String,
    /// True when the scenario's units and geography are invented.
    pub(super) placeholder: bool,
    pub(super) seed: u64,
    /// How many days the scenario runs.
    pub(super) days: u32,
    /// Path to the map file, relative to the scenario file.
    pub(super) map: String,
    /// Path to a land links file, relative to the scenario file. Optional:
    /// its links are added to the map, whatever its format (decision 0094).
    pub(super) land_links: Option<String>,
    /// Path to the ship class file, or a list of them, relative to the
    /// scenario file. Ids must not repeat across files.
    pub(super) ship_classes: FileList,
    /// Path to the port size file, relative to the scenario file.
    pub(super) port_sizes: String,
    /// Path to the aircraft type file, or a list of them, relative to the
    /// scenario file. Ids must not repeat across files.
    pub(super) aircraft_types: FileList,
    /// Path to the search figures file, relative to the scenario file.
    pub(super) search: String,
    /// Path to the surface combat figures file, relative to the scenario file.
    pub(super) surface_combat: String,
    /// Path to the air combat figures file, relative to the scenario file.
    /// Optional: without it no air group can be ordered to strike.
    pub(super) air_combat: Option<String>,
    /// Path to the wear and repair figures, relative to the scenario file.
    /// Optional: without it no aircraft is worn or repaired (decision 0035).
    pub(super) air_upkeep: Option<String>,
    /// Path to the fatigue figures, relative to the scenario file. Optional:
    /// without it no air group tires or rests (decision 0036).
    pub(super) fatigue: Option<String>,
    /// Path to the base supply figures, relative to the scenario file.
    /// Optional: without it nothing is spent and no base is out of supply
    /// (decision 0039).
    pub(super) air_supply: Option<String>,
    /// Path to the ground figures, relative to the scenario file. Optional,
    /// but a scenario with a land unit must name it (decision 0066, 3).
    pub(super) ground: Option<String>,
    /// Path to the report figures, relative to the scenario file. Optional
    /// (decision 0135): without it, with command levels on, every report
    /// reaches every command at once. Not a reference file redirected by
    /// `LoadOptions::reference_root`: it is read where the scenario names it,
    /// as the map is, since the frozen step 1 figures have no report figures.
    pub(super) reports: Option<String>,
    /// Path to the file of sides, relative to the scenario file (decision
    /// 0173, 1): whose each class of ship and type of aircraft is. Optional:
    /// without it no class or type has a side. A reference file, redirected
    /// by `LoadOptions::reference_root` as the class files are.
    pub(super) class_sides: Option<String>,
    /// Path to a trial's figures, relative to the scenario file (decision
    /// 0174): what a base of the enemy's is given when a side's picture is
    /// made. Optional: a scenario that is not to be tried names none. A
    /// reference file, redirected by `LoadOptions::reference_root` as the
    /// class files are. Kept with the scenario, never put into the world.
    pub(super) trial: Option<String>,
    /// Path to the submarine class file, relative to the scenario file
    /// (decision 0218). A reference file, redirected by
    /// `LoadOptions::reference_root` as the class files are. A scenario
    /// names all of `submarine_classes`, `submarine_torpedoes` and
    /// `submarines`, or none.
    pub(super) submarine_classes: Option<String>,
    /// Path to the submarine torpedo file (decision 0218): a reference file,
    /// redirected as the class files are.
    pub(super) submarine_torpedoes: Option<String>,
    /// Path to the submarine figures, relative to the scenario file
    /// (decision 0218). Not a reference file redirected by
    /// `LoadOptions::reference_root`: it is read where the scenario names
    /// it, as `reports` is, since the frozen step 1 figures give none and a
    /// frozen game has no boat.
    pub(super) submarines: Option<String>,
    #[serde(default)]
    pub(super) base: Vec<BaseSpec>,
    #[serde(default)]
    pub(super) land_unit: Vec<LandUnitSpec>,
    /// Submarines (decision 0218): units of their own kind.
    #[serde(default)]
    pub(super) submarine: Vec<SubmarineSpec>,
    #[serde(default)]
    pub(super) air_group: Vec<AirGroupSpec>,
    #[serde(default)]
    pub(super) replacement: Vec<ReplacementSpec>,
    #[serde(default)]
    pub(super) arrival: Vec<ArrivalSpec>,
    #[serde(default)]
    pub(super) supply_line: Vec<SupplyLineSpec>,
    #[serde(default)]
    pub(super) men_replacement: Vec<MenReplacementSpec>,
    #[serde(default)]
    pub(super) task_force: Vec<TaskForceSpec>,
    #[serde(default)]
    pub(super) ship: Vec<ShipSpec>,
    #[serde(default)]
    pub(super) order: Vec<OrderSpec>,
    /// What a side's commander says, by day (decision 0107, item 16).
    #[serde(default)]
    pub(super) command: Vec<CommandSpec>,
    #[serde(default)]
    pub(super) contact: Vec<ContactSpec>,
    /// Opening estimates of the enemy's men ashore (decision 0092, 16).
    #[serde(default)]
    pub(super) estimate: Vec<EstimateSpec>,
    /// The waters where a side suspects enemy submarines work (decision
    /// 0218, 23 and 47).
    #[serde(default)]
    pub(super) suspected_waters: Vec<SuspectedWatersSpec>,
    /// The enemy forces a side believes in and holds no contact of
    /// (decision 0173, 5).
    #[serde(default)]
    pub(super) believed_force: Vec<BelievedForceSpec>,
    /// The intents a side believes the enemy holds (decision 0173, 7).
    #[serde(default)]
    pub(super) believed_intent: Vec<BelievedIntentSpec>,
    /// The men of the enemy's a side believes may be ashore at a place
    /// beyond its estimate (decision 0176, 1).
    #[serde(default)]
    pub(super) believed_men: Vec<BelievedMenSpec>,
    /// Missions and objectives (decision 0054). Optional.
    pub(super) objectives: Option<ObjectivesSpec>,
    /// Whether command levels are on (decision 0134, 1). Optional; off when
    /// absent.
    pub(super) command_levels: Option<bool>,
    /// Whether a side may give orders inside a day, at 0600 and 1200
    /// (decision 0156, 1). Optional; off when absent.
    pub(super) orders_inside_a_day: Option<bool>,
    /// The commanders of the side's commands (decision 0137, 1 and 2).
    #[serde(default)]
    pub(super) commander: Vec<CommanderSpec>,
    /// Formations of land units, each with its parent (decision 0140, 7).
    #[serde(default)]
    pub(super) formation: Vec<FormationSpec>,
    /// Islands: named places a side may be asked to control (decision 0151,
    /// 1). Data on the map, not state.
    #[serde(default)]
    pub(super) island: Vec<IslandSpec>,
    /// What a scenario states of each side's doctrine for its navy and date
    /// (decision 0223, stage 0): `[side.allied]` and `[side.japanese]`, each
    /// optional.
    pub(super) side: Option<SidesSpec>,
}
