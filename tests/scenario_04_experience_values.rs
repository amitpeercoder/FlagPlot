//! Scenario 4 (Savo Island) carries each ship's experience, from the
//! generated file `data/reference/ship_experience_savo_island.ships.csv`
//! (decision 0064). No hand calculation: a table of values and a check that
//! the scenario file and the generated file agree with it.
//!
//! A ship whose value is the standard 2 has no `experience` line in the
//! scenario. No other scenario gets values: its ships are placeholders, and a
//! placeholder ship is 2.
//!
//! The values are the generated file's, and no list of values is kept here:
//! every ship of the scenario has the value the file gives her. The table
//! below only says which file row is which scenario ship.

mod common;

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::state::Ship;

/// The scenario's ship name and the generated file's ship key.
const SHIP_KEYS: [(&str, &str); 17] = [
    ("Chokai", "chokai"),
    ("Aoba", "aoba"),
    ("Kako", "kako"),
    ("Kinugasa", "kinugasa"),
    ("Furutaka", "furutaka"),
    ("Tenryu", "tenryu"),
    ("Yubari", "yubari"),
    ("Yunagi", "yunagi"),
    ("HMAS Canberra", "hmas-canberra"),
    ("USS Chicago", "ca-29-chicago"),
    ("USS Patterson", "dd-392-patterson"),
    ("USS Bagley", "dd-386-bagley"),
    ("USS Vincennes", "ca-44-vincennes"),
    ("USS Quincy", "ca-39-quincy"),
    ("USS Astoria", "ca-34-astoria"),
    ("USS Helm", "dd-388-helm"),
    ("USS Wilson", "dd-408-wilson"),
];

fn data(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../data")
        .join(relative)
}

/// The generated file's value for each ship key.
fn generated() -> BTreeMap<String, u8> {
    let mut reader = csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .from_path(data("reference/ship_experience_savo_island.ships.csv"))
        .expect("the generated file opens");
    let headers = reader.headers().expect("a header").clone();
    let column = |name: &str| {
        headers
            .iter()
            .position(|h| h == name)
            .unwrap_or_else(|| panic!("the file has a {name} column"))
    };
    let (ship, experience) = (column("ship"), column("experience"));
    reader
        .records()
        .map(|record| {
            let record = record.expect("a row reads");
            (
                record[ship].to_string(),
                record[experience].parse().expect("a whole number"),
            )
        })
        .collect()
}

#[test]
fn every_ship_of_scenario_4_has_the_value_the_file_gives_her() {
    let scenario = common::load(&data("scenarios/04_savo_island.toml")).expect("scenario 4 loads");
    let truth = scenario.world.umpire();
    assert_eq!(
        truth.ships.len(),
        SHIP_KEYS.len(),
        "every ship is in the table"
    );
    let generated = generated();
    for ship in truth.ships.values() {
        let (_, key) = SHIP_KEYS
            .iter()
            .find(|(name, _)| *name == ship.name)
            .unwrap_or_else(|| panic!("{} is in the table", ship.name));
        let in_file = generated
            .get(*key)
            .unwrap_or_else(|| panic!("{key} is in the generated file"));
        assert_eq!(ship.experience, *in_file, "{} ({key})", ship.name);
    }
}

#[test]
fn a_ship_with_the_standard_value_has_no_line_in_the_scenario() {
    let text = std::fs::read_to_string(data("scenarios/04_savo_island.toml")).expect("reads");
    let lines = text
        .lines()
        .filter(|l| l.starts_with("experience = "))
        .count();
    let generated = generated();
    let non_standard = SHIP_KEYS
        .iter()
        .filter(|(_, key)| generated[*key] != Ship::DEFAULT_EXPERIENCE)
        .count();
    assert_eq!(lines, non_standard);
}

#[test]
fn no_other_scenario_gives_experience() {
    // Placeholder ships are 2. Scenario 14a is the one that sets values on
    // placeholder ships, to prove the rule; scenario 16 does too, so that
    // experience acts beside the other rules of step 2 (authorised by the
    // architect, Nov 13, 2026).
    let dir = data("scenarios");
    for entry in std::fs::read_dir(dir).expect("the folder reads") {
        let path = entry.expect("an entry").path();
        let name = path
            .file_name()
            .expect("a name")
            .to_string_lossy()
            .into_owned();
        if !name.ends_with(".toml")
            || name == "04_savo_island.toml"
            || name == "14a_experience.toml"
            || name == "16_all_together.toml"
        {
            continue;
        }
        let text = std::fs::read_to_string(&path).expect("reads");
        assert!(
            !text.lines().any(|l| l.starts_with("experience = ")),
            "{name} gives experience"
        );
    }
}
