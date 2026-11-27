use super::*;

// Submarines (decision 0218): the three file keys and the `[[submarine]]`
// table.

const SUBMARINE_KEYS: &str = "submarine_classes = \"../reference/submarine_classes_placeholder_scenario_128.csv\"\nsubmarine_torpedoes = \"../reference/submarine_torpedoes_placeholder_scenario_128.csv\"\nsubmarines = \"../reference/submarines_placeholder_scenario_128.toml\"\n";

/// `HEAD` with the submarine file keys given after the surface combat
/// key and `extra` after it, read with `options`.
fn with_submarines(keys: &str, extra: &str, options: &LoadOptions) -> Result<Scenario, DataError> {
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
    let scenario =
        with_submarines(SUBMARINE_KEYS, &submarine(extra), &LoadOptions::default()).expect("loads");
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
    let scenario =
        with_submarines(SUBMARINE_KEYS, &submarine(""), &LoadOptions::default()).expect("loads");
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
    let live =
        with_submarines(SUBMARINE_KEYS, &submarine(""), &LoadOptions::default()).expect("loads");
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
