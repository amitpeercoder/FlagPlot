use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde::Deserialize;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, ShipClassId, ShipId, TaskForceId};
use sim_core::orders::{
    Issuer, Mission, OrderKind, OrderRequest, Origin, SpeedSetting, TaskForceOrder,
};
use sim_core::state::{Base, Ship, ShipLocation, Side, TaskForce, TruthState};
use sim_core::{DaySummary, World};

use crate::error::DataError;
use crate::map::load_map;
use crate::reference::load_ship_classes;

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
    /// Path to the ship class file, relative to the scenario file.
    ship_classes: String,
    #[serde(default)]
    base: Vec<BaseSpec>,
    #[serde(default)]
    task_force: Vec<TaskForceSpec>,
    #[serde(default)]
    ship: Vec<ShipSpec>,
    #[serde(default)]
    order: Vec<OrderSpec>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BaseSpec {
    id: u32,
    name: String,
    side: SideSpec,
    q: i32,
    r: i32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TaskForceSpec {
    id: u32,
    name: String,
    side: SideSpec,
    /// The base whose hex the task force starts in.
    at_base: u32,
    mission: MissionSpec,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ShipSpec {
    id: u32,
    name: String,
    class: u32,
    side: SideSpec,
    fuel_t: Quantity,
    /// Exactly one of `task_force` and `port` must be given.
    task_force: Option<u32>,
    port: Option<u32>,
}

#[derive(Deserialize)]
struct OrderSpec {
    /// The day the order takes effect. Day 1 is the first day.
    day: u32,
    side: SideSpec,
    task_force: u32,
    #[serde(default)]
    issuer: IssuerSpec,
    intent: Option<u32>,
    #[serde(flatten)]
    action: ActionSpec,
}

// The variant names are the `kind` strings in scenario files.
#[allow(clippy::enum_variant_names)]
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum ActionSpec {
    SetRoute { waypoints: Vec<[i32; 2]> },
    SetSpeed { speed: SpeedSpec },
    SetMission { mission: MissionSpec },
}

/// A quantity written as a whole number or a decimal string. A TOML float
/// matches neither form and is rejected.
#[derive(Deserialize)]
#[serde(untagged)]
enum Quantity {
    Whole(i64),
    Decimal(String),
}

impl Quantity {
    fn to_fixed(&self) -> Result<Fixed, String> {
        match self {
            Quantity::Whole(n) => Ok(Fixed::from_int(*n)),
            Quantity::Decimal(text) => text
                .parse::<Fixed>()
                .map_err(|e| format!("\"{text}\" is invalid: {e}")),
        }
    }
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
}

impl Scenario {
    /// Loads a scenario and the map and reference files it names.
    pub fn load(path: &Path) -> Result<Scenario, DataError> {
        let fail = |message: String| DataError::in_file(path, message);

        let text = fs::read_to_string(path).map_err(|e| fail(e.to_string()))?;
        let file: ScenarioFile = toml::from_str(&text).map_err(|e| fail(e.to_string()))?;

        let folder = path.parent().unwrap_or_else(|| Path::new("."));
        let map = load_map(&folder.join(&file.map))?;
        let ref_data = load_ship_classes(&folder.join(&file.ship_classes))?;

        let mut truth = TruthState::default();

        for spec in &file.base {
            let id = BaseId::new(spec.id);
            let base = Base {
                id,
                name: spec.name.clone(),
                side: spec.side.into(),
                hex: Hex::new(spec.q, spec.r),
            };
            if truth.bases.insert(id, base).is_some() {
                return Err(fail(format!("{id} appears twice")));
            }
        }

        for spec in &file.task_force {
            let id = TaskForceId::new(spec.id);
            let base = truth
                .bases
                .get(&BaseId::new(spec.at_base))
                .ok_or_else(|| fail(format!("{id} starts at unknown base {}", spec.at_base)))?;
            let task_force = TaskForce::at(
                id,
                spec.name.clone(),
                spec.side.into(),
                base.hex,
                spec.mission.into(),
            );
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
            let ship = Ship {
                id,
                name: spec.name.clone(),
                class,
                side: spec.side.into(),
                fuel_t: spec
                    .fuel_t
                    .to_fixed()
                    .map_err(|e| fail(format!("{id}: fuel_t {e}")))?,
                location,
            };
            if truth.ships.insert(id, ship).is_some() {
                return Err(fail(format!("{id} appears twice")));
            }
        }

        let mut script: BTreeMap<u32, Vec<(Side, OrderRequest)>> = BTreeMap::new();
        for spec in &file.order {
            if spec.day == 0 || spec.day > file.days {
                return Err(fail(format!(
                    "an order is set for day {}, outside days 1 to {}",
                    spec.day, file.days
                )));
            }
            let order = match &spec.action {
                ActionSpec::SetRoute { waypoints } => TaskForceOrder::SetRoute(
                    waypoints.iter().map(|[q, r]| Hex::new(*q, *r)).collect(),
                ),
                ActionSpec::SetSpeed { speed } => TaskForceOrder::SetSpeed((*speed).into()),
                ActionSpec::SetMission { mission } => TaskForceOrder::SetMission((*mission).into()),
            };
            let request = OrderRequest {
                kind: OrderKind::TaskForce {
                    target: TaskForceId::new(spec.task_force),
                    order,
                },
                origin: Origin {
                    issuer: spec.issuer.into(),
                    intent: spec.intent.map(IntentId::new),
                },
            };
            script
                .entry(spec.day)
                .or_default()
                .push((spec.side.into(), request));
        }

        let world = World::new(file.seed, ref_data, map, truth).map_err(|e| fail(e.to_string()))?;

        Ok(Scenario {
            name: file.name,
            placeholder: file.placeholder,
            days: file.days,
            world,
            script,
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
}
