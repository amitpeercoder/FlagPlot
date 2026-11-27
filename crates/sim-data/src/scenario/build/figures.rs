use crate::error::DataError;
use crate::map::{load_land_links_from, load_map_from, read_map_frame_from};
use crate::reference::{
    load_air_combat_figures_from, load_air_supply_figures_from, load_air_upkeep_figures_from,
    load_aircraft_types_from, load_class_sides_from, load_fatigue_figures_from,
    load_ground_figures_from, load_port_sizes_from, load_report_figures_from,
    load_search_figures_from, load_ship_classes_marked_from, load_submarine_classes_from,
    load_submarine_figures_from, load_submarine_torpedoes_from, load_surface_combat_figures_from,
    load_trial_figures_from,
};
use crate::source::Source;
use sim_core::ids::{AircraftTypeId, ShipClassId};
use sim_core::refdata::{RefData, SubmarineData};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::super::file::ScenarioFile;
use super::super::LoadOptions;

/// What a scenario's reference files give: the map, the reference data, the
/// trial's figures, and the list of every data file read.
pub(super) struct Figures {
    pub(super) map: sim_core::map::Map,
    pub(super) map_frame: Option<crate::map::MapFrame>,
    pub(super) ref_data: RefData,
    pub(super) data_files: Vec<PathBuf>,
    pub(super) estimated_figures: crate::reference::EstimatedFigures,
    pub(super) trial_figures: Option<sim_core::refdata::TrialFigures>,
}

/// Reads the map, the reference files and the submarine files the scenario
/// names, in the order the checks have always run.
pub(super) fn load_figures(
    source: &dyn Source,
    options: &LoadOptions,
    path: &Path,
    file: &ScenarioFile,
    built_on_files: Vec<PathBuf>,
) -> Result<Figures, DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    let folder = path.parent().unwrap_or_else(|| Path::new("."));
    // Where a named reference file is read from: where the scenario names
    // it, or by file name under the options' reference root.
    let reference = |name: &str| -> Result<PathBuf, DataError> {
        match &options.reference_root {
            None => Ok(folder.join(name)),
            Some(root) => {
                let file_name = Path::new(name)
                    .file_name()
                    .ok_or_else(|| fail(format!("\"{name}\" does not name a reference file")))?;
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
    let mut ship_classes = BTreeMap::new();
    let mut class_owner: BTreeMap<ShipClassId, PathBuf> = BTreeMap::new();
    let mut estimated_figures = crate::reference::EstimatedFigures::new();
    for name in &class_files {
        let file_path = reference(name)?;
        let (classes, marked) = load_ship_classes_marked_from(source, &file_path)?;
        estimated_figures.extend(marked);
        for (id, class) in classes {
            if let Some(first) = class_owner.insert(id, file_path.clone()) {
                return Err(fail(format!(
                    "ship class {id} is in both {} and {}",
                    first.display(),
                    file_path.display()
                )));
            }
            ship_classes.insert(id, class);
        }
    }
    let port_sizes = load_port_sizes_from(source, &reference(&file.port_sizes)?)?;
    let mut aircraft_types = BTreeMap::new();
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
            aircraft_types.insert(id, aircraft_type);
        }
    }
    let search = load_search_figures_from(source, &reference(&file.search)?)?;
    let surface_combat =
        load_surface_combat_figures_from(source, &reference(&file.surface_combat)?)?;
    let mut ref_data = RefData {
        ship_classes,
        port_sizes,
        aircraft_types,
        search,
        surface_combat,
        air_combat: None,
        air_upkeep: None,
        fatigue: None,
        air_supply: None,
        ground: None,
        reports: None,
        class_sides: None,
        submarines: None,
    };
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
    Ok(Figures {
        map,
        map_frame,
        ref_data,
        data_files,
        estimated_figures,
        trial_figures,
    })
}
