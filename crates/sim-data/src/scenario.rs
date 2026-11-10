use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde::Deserialize;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{
    AirGroupId, AircraftTypeId, BaseId, IntentId, ShipClassId, ShipId, TaskForceId,
};
use sim_core::orders::{
    AirGroupOrder, AirMission, Issuer, Mission, OrderKind, OrderRequest, Origin, SpeedSetting,
    TaskForceOrder,
};
use sim_core::state::{AirGroup, Base, Ship, ShipLocation, Side, TaskForce, TruthState};
use sim_core::{DaySummary, World};

use crate::error::DataError;
use crate::map::load_map;
use crate::quantity::Quantity;
use crate::reference::{
    load_aircraft_types, load_port_sizes, load_search_figures, load_ship_classes,
};

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
    /// Path to the port size file, relative to the scenario file.
    port_sizes: String,
    /// Path to the aircraft type file, relative to the scenario file.
    aircraft_types: String,
    /// Path to the search figures file, relative to the scenario file.
    search: String,
    #[serde(default)]
    base: Vec<BaseSpec>,
    #[serde(default)]
    air_group: Vec<AirGroupSpec>,
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
    /// Supply stock at the start, in tons. Defaults to 0.
    supply_t: Option<Quantity>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AirGroupSpec {
    id: u32,
    name: String,
    side: SideSpec,
    base: u32,
    aircraft_type: u32,
    /// Aircraft ready to fly at the start.
    ready: u32,
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
    /// Exactly one of `task_force` and `port` must be given.
    task_force: Option<u32>,
    port: Option<u32>,
}

#[derive(Deserialize)]
struct OrderSpec {
    /// The day the order takes effect. Day 1 is the first day.
    day: u32,
    side: SideSpec,
    /// Exactly one of `task_force` and `air_group` must be given.
    task_force: Option<u32>,
    air_group: Option<u32>,
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
#[allow(clippy::enum_variant_names)]
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum ActionSpec {
    SetRoute {
        waypoints: Vec<[i32; 2]>,
    },
    SetSpeed {
        speed: SpeedSpec,
    },
    SetMission {
        mission: MissionSpec,
    },
    Load {},
    Unload {},
    SetAirMission {
        mission: AirMissionSpec,
    },
    SetSearchArc {
        from_deg: i32,
        to_deg: i32,
        range_nm: Quantity,
    },
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum AirMissionSpec {
    Rest,
    Search,
}

impl From<AirMissionSpec> for AirMission {
    fn from(spec: AirMissionSpec) -> AirMission {
        match spec {
            AirMissionSpec::Rest => AirMission::Rest,
            AirMissionSpec::Search => AirMission::Search,
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
        Scenario::parse(&text, path)
    }

    /// Builds a scenario from its text. `path` says where the text came from:
    /// the files it names are read relative to it.
    fn parse(text: &str, path: &Path) -> Result<Scenario, DataError> {
        let fail = |message: String| DataError::in_file(path, message);

        let file: ScenarioFile = toml::from_str(text).map_err(|e| fail(e.to_string()))?;

        let folder = path.parent().unwrap_or_else(|| Path::new("."));
        let map = load_map(&folder.join(&file.map))?;
        let mut ref_data = load_ship_classes(&folder.join(&file.ship_classes))?;
        ref_data.port_sizes = load_port_sizes(&folder.join(&file.port_sizes))?;
        ref_data.aircraft_types = load_aircraft_types(&folder.join(&file.aircraft_types))?;
        ref_data.search = Some(load_search_figures(&folder.join(&file.search))?);

        let mut truth = TruthState::default();

        for spec in &file.base {
            let id = BaseId::new(spec.id);
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
            };
            if truth.bases.insert(id, base).is_some() {
                return Err(fail(format!("{id} appears twice")));
            }
        }

        for spec in &file.air_group {
            let id = AirGroupId::new(spec.id);
            let side: Side = spec.side.into();
            let base_id = BaseId::new(spec.base);
            let base = truth
                .bases
                .get(&base_id)
                .ok_or_else(|| fail(format!("{id} is at unknown base {}", spec.base)))?;
            if base.side != side {
                return Err(fail(format!(
                    "{id} is {} but its base {base_id} is {}",
                    side.name(),
                    base.side.name()
                )));
            }
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
                base: base_id,
                aircraft_type,
                ready: spec.ready,
                mission: AirMission::Rest,
                search: None,
                mission_order: None,
                arc_order: None,
            };
            if truth.air_groups.insert(id, group).is_some() {
                return Err(fail(format!("{id} appears twice")));
            }
        }

        for spec in &file.task_force {
            let id = TaskForceId::new(spec.id);
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
            let task_force = TaskForce::at(
                id,
                spec.name.clone(),
                spec.side.into(),
                hex,
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
                supply_t: match &spec.supply_t {
                    Some(quantity) => quantity
                        .to_fixed()
                        .map_err(|e| fail(format!("{id}: supply_t {e}")))?,
                    None => Fixed::ZERO,
                },
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
            let kind = match (&spec.task_force, &spec.air_group) {
                (Some(raw), None) => {
                    let order = match &spec.action {
                        ActionSpec::SetRoute { waypoints } => TaskForceOrder::SetRoute(
                            waypoints.iter().map(|[q, r]| Hex::new(*q, *r)).collect(),
                        ),
                        ActionSpec::SetSpeed { speed } => TaskForceOrder::SetSpeed((*speed).into()),
                        ActionSpec::SetMission { mission } => {
                            TaskForceOrder::SetMission((*mission).into())
                        }
                        ActionSpec::Load {} => TaskForceOrder::Load,
                        ActionSpec::Unload {} => TaskForceOrder::Unload,
                        ActionSpec::SetAirMission { .. } => {
                            return Err(wrong_target("set_air_mission", "air_group"))
                        }
                        ActionSpec::SetSearchArc { .. } => {
                            return Err(wrong_target("set_search_arc", "air_group"))
                        }
                    };
                    OrderKind::TaskForce {
                        target: TaskForceId::new(*raw),
                        order,
                    }
                }
                (None, Some(raw)) => {
                    let order = match &spec.action {
                        ActionSpec::SetAirMission { mission } => {
                            AirGroupOrder::SetAirMission((*mission).into())
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
                        _ => {
                            return Err(fail(format!(
                                "an order on day {day} for an air_group must be set_air_mission or set_search_arc"
                            )))
                        }
                    };
                    OrderKind::AirGroup {
                        target: AirGroupId::new(*raw),
                        order,
                    }
                }
                _ => {
                    return Err(fail(format!(
                        "the order on day {day} needs exactly one of task_force and air_group"
                    )))
                }
            };
            let request = OrderRequest { kind, origin };
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
        let world = World::new(
            seed,
            self.world.ref_data().clone(),
            self.world.map().clone(),
            self.world.umpire().clone(),
        )?;
        Ok(Scenario {
            name: self.name.clone(),
            placeholder: self.placeholder,
            days: self.days,
            world,
            script: self.script.clone(),
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
                .contains("exactly one of task_force and air_group")
        );
        assert!(error_of(&order("", "kind = \"load\"")).contains("exactly one"));
        let float = arc.replace("range_nm = 600", "range_nm = 600.5");
        assert!(parse_with(&order("air_group = 1", &float)).is_err());
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
}
