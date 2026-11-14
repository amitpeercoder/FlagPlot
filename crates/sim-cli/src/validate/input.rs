//! What `validate` reads: a batch file of version 5 (decision 0062) and a
//! measures file (`data/validation/README.md`). Only the keys it needs are
//! named; every key it names must be there, so a file from before is refused
//! rather than read as empty.

use serde::Deserialize;

// ----------------------------------------------------------- the batch file

/// Just the header of a batch file, read first so that a file of another
/// version is refused for its version and not for a key it lacks.
#[derive(Deserialize)]
pub struct BatchHeader {
    pub format: String,
    pub version: u32,
}

#[derive(Deserialize)]
pub struct Batch {
    pub scenario: ScenarioIn,
    pub inputs: InputsIn,
    #[serde(default)]
    pub label: Option<String>,
    pub runs: u64,
    pub first_seed: u64,
    pub ships: Vec<ShipIn>,
    pub per_run: Vec<PerRun>,
    #[serde(default)]
    pub results: Option<Vec<ResultRow>>,
    pub task_forces: Vec<IdOnly>,
    pub air_groups: Vec<GroupIn>,
    pub bases: Vec<BaseIn>,
    /// Supply a man ashore costs his base a day, a decimal string; left out
    /// of a batch whose scenario has no ground figures.
    #[serde(default)]
    pub daily_supply_t_per_man: Option<String>,
}

#[derive(Deserialize)]
pub struct ScenarioIn {
    pub file: String,
    pub days: u32,
}

#[derive(Deserialize)]
pub struct InputsIn {
    pub state_version: String,
    pub scenario_hash: String,
    pub data: Vec<FileIn>,
}

#[derive(Deserialize)]
pub struct FileIn {
    pub file: String,
    pub hash: String,
}

#[derive(Deserialize)]
pub struct ShipIn {
    pub id: u32,
    pub task_force: Option<u32>,
}

#[derive(Deserialize)]
pub struct IdOnly {
    pub id: u32,
}

#[derive(Deserialize)]
pub struct BaseIn {
    pub id: u32,
    /// The side that holds the base as the scenario starts. Left out of a
    /// hand-made batch with no `base_holder` measure.
    #[serde(default)]
    pub side: Option<String>,
}

#[derive(Deserialize)]
pub struct GroupIn {
    pub id: u32,
    pub side: String,
}

#[derive(Deserialize)]
pub struct ResultRow {
    pub outcome: String,
    pub runs: u64,
}

#[derive(Deserialize)]
pub struct PerRun {
    pub seed: u64,
    pub sunk: Vec<u32>,
    #[serde(default)]
    pub allied_margin: Option<String>,
    pub hits_on: Vec<HitsOn>,
    pub fire: Fire,
    pub shots_by: Vec<ShotsBy>,
    pub systems_out: Vec<SystemOut>,
    pub air_losses: Vec<AirLoss>,
    pub ready: Vec<ReadyRow>,
    pub claims: Claims,
    pub attacks: Vec<Attack>,
    pub escorts: Vec<Escort>,
    pub launches: Vec<Launch>,
    pub fights: Vec<Fight>,
    pub formed_groups: Vec<Formed>,
    #[serde(default)]
    pub base_changes: Vec<BaseChange>,
    #[serde(default)]
    pub men_lost_by_day: Vec<MenLostDay>,
    #[serde(default)]
    pub men_lost_ashore_by_day: Vec<MenLostAshoreDay>,
    #[serde(default)]
    pub airfields_built: Vec<AirfieldBuilt>,
    #[serde(default)]
    pub arrivals_delivered: Vec<Arrival>,
    #[serde(default)]
    pub ashore_by_day: Vec<Ashore>,
}

#[derive(Deserialize)]
pub struct BaseChange {
    pub day: u32,
    pub base: u32,
    pub to: String,
}

#[derive(Deserialize)]
pub struct MenLostDay {
    pub day: u32,
    pub japanese: u64,
    pub allied: u64,
}

#[derive(Deserialize)]
pub struct MenLostAshoreDay {
    pub day: u32,
    pub q: i32,
    pub r: i32,
    pub japanese: u64,
    pub allied: u64,
}

#[derive(Deserialize)]
pub struct AirfieldBuilt {
    pub day: u32,
    pub base: u32,
}

#[derive(Deserialize)]
pub struct Arrival {
    pub day: u32,
    pub air_group: u32,
}

#[derive(Deserialize)]
pub struct Ashore {
    pub day: u32,
    pub side: String,
    pub q: i32,
    pub r: i32,
    pub men: u64,
    pub supply_t: String,
}

#[derive(Deserialize)]
pub struct HitsOn {
    pub ship: u32,
    pub gun: u64,
    pub torpedo: u64,
    pub bomb: u64,
}

#[derive(Deserialize)]
pub struct ShotsBy {
    pub ship: u32,
    pub gun: String,
    pub torpedo: String,
}

#[derive(Deserialize)]
pub struct Fire {
    pub japanese: SideFire,
    pub allied: SideFire,
}

#[derive(Deserialize)]
pub struct SideFire {
    pub gun: WeaponFire,
    pub torpedo: WeaponFire,
}

#[derive(Deserialize)]
pub struct WeaponFire {
    pub shots: String,
    pub hits: u64,
}

#[derive(Deserialize)]
pub struct SystemOut {
    pub ship: u32,
    pub system: String,
}

#[derive(Deserialize)]
pub struct AirLoss {
    pub air_group: u32,
    pub shot_down: u64,
    pub destroyed_on_ground: u64,
    pub damaged_destroyed_on_ground: u64,
    pub lost_with_ship: u64,
    pub worn_out: u64,
    pub ditched_fuel: u64,
    pub ditched_no_deck: u64,
    pub pilots_lost: u64,
}

#[derive(Deserialize)]
pub struct ReadyRow {
    pub air_group: u32,
    pub by_day: Vec<u64>,
}

#[derive(Deserialize)]
pub struct Claims {
    pub japanese: SideClaims,
    pub allied: SideClaims,
}

#[derive(Deserialize)]
pub struct SideClaims {
    pub sunk: u64,
    pub hits: u64,
    pub shot_down: u64,
    pub on_ground: u64,
}

#[derive(Deserialize)]
pub struct Attack {
    pub day: u32,
    pub pulse: String,
    pub air_group: u32,
    pub target_kind: String,
    pub target: u32,
    pub aircraft: u64,
}

#[derive(Deserialize)]
pub struct Escort {
    pub day: u32,
    pub pulse: String,
    pub with_air_group: u32,
    pub aircraft: u64,
}

#[derive(Deserialize)]
pub struct Launch {
    pub day: u32,
    pub pulse: String,
    pub air_group: u32,
    pub aircraft: u64,
    pub target_kind: String,
    pub found_nothing: bool,
    pub task_force: Option<u32>,
}

#[derive(Deserialize)]
pub struct Fight {
    pub pulse: String,
    pub rounds: u64,
    pub sighted_first: String,
    pub forces: Vec<FightForce>,
}

#[derive(Deserialize)]
pub struct FightForce {
    pub task_force: u32,
    #[allow(dead_code)]
    pub side: String,
    #[allow(dead_code)]
    pub first_sighted_round: Option<u32>,
    pub rounds_surprised: u64,
}

#[derive(Deserialize)]
pub struct Formed {
    pub id: u32,
    pub side: String,
    pub from: u32,
}

// -------------------------------------------------------- the measures file

#[derive(Deserialize)]
pub struct MeasuresFile {
    pub battle: String,
    #[serde(default)]
    pub scenario: Option<String>,
    #[allow(dead_code)]
    pub facts: String,
    pub measure: Vec<Measure>,
}

/// One `[[measure]]` as written. Which keys go with which `take` is checked
/// in `take.rs`; a key the README does not name is an error here.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Measure {
    pub id: String,
    pub counts: String,
    pub record_low: i64,
    pub record_high: i64,
    pub judged: bool,
    #[serde(default)]
    pub take: Option<String>,
    pub rows: Vec<i64>,
    #[allow(dead_code)]
    pub reason: String,
    #[serde(default)]
    pub ships: Option<Vec<u32>>,
    #[serde(default)]
    pub system: Option<String>,
    #[serde(default)]
    pub weapons: Option<Vec<String>>,
    #[serde(default)]
    pub weapon: Option<String>,
    #[serde(default)]
    pub side: Option<String>,
    #[serde(default)]
    pub air_groups: Option<Vec<u32>>,
    #[serde(default)]
    pub causes: Option<Vec<String>>,
    #[serde(default)]
    pub day: Option<u32>,
    #[serde(default)]
    pub escorts: Option<bool>,
    #[serde(default)]
    pub target_task_forces: Option<Vec<u32>>,
    #[serde(default)]
    pub target_base: Option<u32>,
    #[serde(default)]
    pub aircraft: Option<u64>,
    #[serde(default)]
    pub claim: Option<String>,
    #[serde(default)]
    pub by: Option<Vec<String>>,
    #[serde(default)]
    pub task_forces: Option<Vec<u32>>,
    /// A single base (`base_holder`, `airfield_built_day`).
    #[serde(default)]
    pub base: Option<u32>,
    /// A single air group (`arrival_day`).
    #[serde(default)]
    pub air_group: Option<u32>,
    #[serde(default)]
    pub due_day: Option<u32>,
    #[serde(default)]
    pub from_day: Option<u32>,
    #[serde(default)]
    pub to_day: Option<u32>,
    /// A hex (`days_of_food`).
    #[serde(default)]
    pub q: Option<i32>,
    #[serde(default)]
    pub r: Option<i32>,
    /// The unit a `shots` take counts in: "shells" (the default) or "tens".
    #[serde(default)]
    pub unit: Option<String>,
    /// Only where the measure's rows are in another fact file than the one
    /// the file names at its head. Nothing checks rows against a fact file.
    #[serde(default)]
    #[allow(dead_code)]
    pub facts: Option<String>,
}
