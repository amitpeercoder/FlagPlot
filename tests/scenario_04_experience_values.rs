//! Scenario 4 (Savo Island) carries each ship's experience, from the
//! generated file `data/reference/ship_experience_savo_island.ships.csv`
//! (decision 0064). No hand calculation: a table of values and a check that
//! the scenario file and the generated file agree with it.
//!
//! A ship whose value is the standard 2 has no `experience` line in the
//! scenario. No other scenario gets values: its ships are placeholders, and a
//! placeholder ship is 2.
//!
//! The values asserted are the architect's list of Nov 12, 2026: the Japanese
//! ships (Chokai, Aoba, Kako, Kinugasa, Furutaka, Tenryu, Yubari, Yunagi) 3;
//! Canberra, Chicago 3; Vincennes, Quincy 1; Astoria 2; Patterson, Bagley,
//! Helm 3; Wilson 2. The generated file on this branch says 2 for Patterson,
//! Bagley and Helm: the file is being regenerated in another pull request with
//! the list's values. Until it merges the test accepts 2 or 3 from the file for
//! those three, and holds the scenario to 3; when it merges the three
//! exceptions in `PENDING_IN_THE_FILE` go. The scenario is held to the list in
//! every case.

mod common;

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::state::Ship;

/// The scenario's ship name, the generated file's ship key and the value.
const VALUES: [(&str, &str, u8); 17] = [
    ("Chokai", "chokai", 3),
    ("Aoba", "aoba", 3),
    ("Kako", "kako", 3),
    ("Kinugasa", "kinugasa", 3),
    ("Furutaka", "furutaka", 3),
    ("Tenryu", "tenryu", 3),
    ("Yubari", "yubari", 3),
    ("Yunagi", "yunagi", 3),
    ("HMAS Canberra", "hmas-canberra", 3),
    ("USS Chicago", "ca-29-chicago", 3),
    ("USS Patterson", "dd-392-patterson", 3),
    ("USS Bagley", "dd-386-bagley", 3),
    ("USS Vincennes", "ca-44-vincennes", 1),
    ("USS Quincy", "ca-39-quincy", 1),
    ("USS Astoria", "ca-34-astoria", 2),
    ("USS Helm", "dd-388-helm", 3),
    ("USS Wilson", "dd-408-wilson", 2),
];

/// The file on this branch says 2 for these; the list says 3.
const PENDING_IN_THE_FILE: [&str; 3] = ["dd-392-patterson", "dd-386-bagley", "dd-388-helm"];

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
fn every_ship_of_scenario_4_has_her_value() {
    let scenario = common::load(&data("scenarios/04_savo_island.toml")).expect("scenario 4 loads");
    let truth = scenario.world.umpire();
    assert_eq!(
        truth.ships.len(),
        VALUES.len(),
        "every ship is in the table"
    );
    let generated = generated();
    for ship in truth.ships.values() {
        let (_, key, value) = VALUES
            .iter()
            .find(|(name, ..)| *name == ship.name)
            .unwrap_or_else(|| panic!("{} is in the table", ship.name));
        assert_eq!(ship.experience, *value, "{} in the scenario", ship.name);
        let in_file = generated
            .get(*key)
            .unwrap_or_else(|| panic!("{key} is in the generated file"));
        if PENDING_IN_THE_FILE.contains(key) {
            assert!(
                [Ship::DEFAULT_EXPERIENCE, *value].contains(in_file),
                "{key} in the file: {in_file}"
            );
        } else {
            assert_eq!(in_file, value, "{key} in the generated file");
        }
    }
}

#[test]
fn a_ship_with_the_standard_value_has_no_line_in_the_scenario() {
    let text = std::fs::read_to_string(data("scenarios/04_savo_island.toml")).expect("reads");
    let lines = text
        .lines()
        .filter(|l| l.starts_with("experience = "))
        .count();
    let non_standard = VALUES
        .iter()
        .filter(|(.., value)| *value != Ship::DEFAULT_EXPERIENCE)
        .count();
    assert_eq!(lines, non_standard);
}

#[test]
fn no_other_scenario_gives_experience() {
    // Placeholder ships are 2. Scenario 14a is the one that sets values on
    // placeholder ships, to prove the rule.
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
