use crate::error::DataError;
use crate::map::{load_land_links_from, load_map_from, read_map_frame_from};
use crate::quantity::Quantity;
use crate::reference::{
    load_air_combat_figures_from, load_air_supply_figures_from, load_air_upkeep_figures_from,
    load_aircraft_types_from, load_class_sides_from, load_fatigue_figures_from,
    load_ground_figures_from, load_port_sizes_from, load_report_figures_from,
    load_search_figures_from, load_ship_classes_marked_from, load_submarine_classes_from,
    load_submarine_figures_from, load_submarine_torpedoes_from, load_surface_combat_figures_from,
    load_trial_figures_from, parse_kind,
};
use crate::source::Source;
use sim_core::command::CommandKind;
use sim_core::commander::{Commander, Initiative, NamedCommander};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{
    AirGroupId, AircraftTypeId, BaseId, EntityRef, FormationId, IntentId, LandUnitId, ShipClassId,
    ShipId, SubmarineClassId, SubmarineId, TaskForceId,
};
use sim_core::map::{HexKind, Island, IslandPlace};
use sim_core::orders::{
    Activity, AirGroupOrder, AirMission, BaseOrder, Cargo, Job, LandUnitOrder, OrderKind,
    OrderRequest, Origin, Posture, Reach, Readiness, SpeedSetting, SubmarineOrder, TaskForceOrder,
};
use sim_core::pictures::{CommandRef, NamedSuperior};
use sim_core::refdata::{RefData, ShipKind, SubmarineData};
use sim_core::state::{
    AirGroup, AirGroupLocation, Arrival, Base, LandUnit, Replacement, Ship, ShipLocation,
    ShipSystem, ShipSystems, Side, Skills, Submarine, SubmarineCondition, SubmarinePassage,
    SystemState, TaskForce, TruthState, WatersWaiting,
};
use sim_core::time::Pulse;
use sim_core::world::MADE_IDS_FLOOR;
use sim_core::{StartingContact, StartingEstimate, World};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use super::beliefs::{believed_force, believed_intent, believed_men};
use super::built_on::{self, Chain};
use super::file::ScenarioFile;
use super::land::{formations, LandUnitSpec};
use super::objectives::build_objectives;
use super::orders::{ActionSpec, ActivitySpec, AirMissionSpec, CargoSpec, PostureSpec};
use super::{available_day, LoadOptions, Scenario, ScriptInsideDay};

impl Scenario {
    /// As `parse_with`, reading the files the scenario names through `source`.
    pub(super) fn parse_via(
        source: &dyn Source,
        text: &str,
        path: &Path,
        options: &LoadOptions,
    ) -> Result<Scenario, DataError> {
        let fail = |message: String| DataError::in_file(path, message);

        // A file that states no `built_on` is read as it always was, so its
        // errors keep their lines; an overlay is laid over the files it is
        // built on as tables and read from the result (decision 0234).
        let mut built_on_files: Vec<PathBuf> = Vec::new();
        let file: ScenarioFile = if built_on::may_state_built_on(text)
            && toml::from_str::<toml::Table>(text).is_ok_and(|t| t.contains_key("built_on"))
        {
            let chain = Chain::read(source, text, path)?;
            built_on_files = chain.paths[1..].to_vec();
            chain
                .laid()?
                .try_into()
                .map_err(|e: toml::de::Error| fail(e.to_string()))?
        } else {
            toml::from_str(text).map_err(|e| fail(e.to_string()))?
        };

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
                    if source.is_file(&found) {
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
        let mut map = load_map_from(source, &folder.join(&file.map))?;
        let map_frame = read_map_frame_from(source, &folder.join(&file.map))?;
        let land_links_file = file.land_links.as_ref().map(|name| folder.join(name));
        if let Some(links) = &land_links_file {
            load_land_links_from(source, links, &mut map)?;
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
            let (classes, marked) = load_ship_classes_marked_from(source, &file_path)?;
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
        ref_data.port_sizes = load_port_sizes_from(source, &reference(&file.port_sizes)?)?;
        let mut type_owner: BTreeMap<AircraftTypeId, PathBuf> = BTreeMap::new();
        for name in &type_files {
            let file_path = reference(name)?;
            for (id, aircraft_type) in load_aircraft_types_from(source, &file_path)? {
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
        ref_data.search = Some(load_search_figures_from(source, &reference(&file.search)?)?);
        ref_data.surface_combat = Some(load_surface_combat_figures_from(
            source,
            &reference(&file.surface_combat)?,
        )?);
        if let Some(air_combat) = &file.air_combat {
            ref_data.air_combat = Some(load_air_combat_figures_from(
                source,
                &reference(air_combat)?,
            )?);
        }
        if let Some(air_upkeep) = &file.air_upkeep {
            ref_data.air_upkeep = Some(load_air_upkeep_figures_from(
                source,
                &reference(air_upkeep)?,
            )?);
        }
        if let Some(fatigue) = &file.fatigue {
            ref_data.fatigue = Some(load_fatigue_figures_from(source, &reference(fatigue)?)?);
        }
        if let Some(air_supply) = &file.air_supply {
            ref_data.air_supply = Some(load_air_supply_figures_from(
                source,
                &reference(air_supply)?,
            )?);
        }
        if let Some(ground) = &file.ground {
            ref_data.ground = Some(load_ground_figures_from(source, &reference(ground)?)?);
        }
        let reports_file = file.reports.as_ref().map(|name| folder.join(name));
        if let Some(reports) = &reports_file {
            ref_data.reports = Some(load_report_figures_from(source, reports)?);
        }
        // The file of sides (decision 0173, 1 and 2): whole and true of the
        // reference files. Whether a ship or an air group is of a class or
        // type given to the other side alone is checked once they are built.
        let class_sides_file = match &file.class_sides {
            Some(name) => Some(reference(name)?),
            None => None,
        };
        if let Some(sides_path) = &class_sides_file {
            let sides = load_class_sides_from(source, sides_path)?;
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
            Some(path) => Some(load_trial_figures_from(source, path)?),
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
                    classes: load_submarine_classes_from(source, &classes_path)?,
                    torpedoes: load_submarine_torpedoes_from(source, &torpedoes_path)?,
                    figures: load_submarine_figures_from(source, &figures_path)?,
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

        // The files the scenario is built on, nearest first, after every
        // file a scenario without `built_on` lists (decision 0234, item 11).
        data_files.extend(built_on_files);

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
        // Each side's submarine doctrine, as the scenario states it (decision
        // 0223, stage 0).
        if let Some(sides) = &file.side {
            for (side, spec) in [
                (Side::Allied, sides.allied.as_ref()),
                (Side::Japanese, sides.japanese.as_ref()),
            ] {
                if let Some(spec) = spec {
                    let doctrine = spec.doctrine(side).map_err(&fail)?;
                    world
                        .set_submarine_doctrine(side, doctrine)
                        .map_err(|e| fail(e.to_string()))?;
                    if spec.staff_chooses_supply_sources == Some(true) {
                        world
                            .set_staff_chooses_sources(side)
                            .map_err(|e| fail(e.to_string()))?;
                    }
                }
            }
        }
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
}
