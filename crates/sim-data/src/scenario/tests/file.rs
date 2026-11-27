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
    let with = with_air_upkeep("air_upkeep = \"../reference/air_upkeep.toml\"\n").expect("loads");
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
fn the_surface_combat_path_is_required_and_the_file_is_loaded() {
    let scenario = parse_with(&task_force("at_base = 1")).expect("loads");
    let figures = &scenario.world.ref_data().surface_combat;
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
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scenarios/04a_night_action.toml");
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
