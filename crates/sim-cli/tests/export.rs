//! Integration tests of `sim-cli export`, and golden files that pin the
//! output of `sim-cli run`.
//!
//! The export is checked against a world the test runs itself through
//! `sim-data`, so the file must say what the simulation says.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;
use sim_data::Scenario;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

fn binary(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_sim-cli"))
        .args(args)
        .current_dir(ROOT)
        .output()
        .expect("the binary starts")
}

/// Every `data/scenarios/*.toml`, as paths relative to the repo root, sorted.
fn scenarios() -> Vec<String> {
    let mut paths: Vec<String> = fs::read_dir(Path::new(ROOT).join("data/scenarios"))
        .expect("the scenarios folder exists")
        .map(|entry| entry.expect("a directory entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "toml"))
        .map(|path| {
            format!(
                "data/scenarios/{}",
                path.file_name().expect("a file name").to_string_lossy()
            )
        })
        .collect();
    paths.sort();
    assert!(!paths.is_empty(), "no scenarios found");
    paths
}

fn stem(scenario: &str) -> String {
    Path::new(scenario)
        .file_stem()
        .expect("a file stem")
        .to_string_lossy()
        .into_owned()
}

fn temp_file(label: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("flagplot-export-test-{}", std::process::id()));
    fs::create_dir_all(&dir).expect("make the temp dir");
    dir.join(format!("{label}.json"))
}

/// Exports `scenario` with `extra` arguments and returns the file's bytes.
fn export(label: &str, scenario: &str, extra: &[&str]) -> Vec<u8> {
    let out = temp_file(label);
    let out_text = out.to_string_lossy().into_owned();
    let mut args = vec!["export", scenario, "--out", out_text.as_str()];
    args.extend_from_slice(extra);
    let result = binary(&args);
    assert!(
        result.status.success(),
        "export of {scenario} failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    let bytes = fs::read(&out).expect("the export file exists");
    let _ = fs::remove_file(&out);
    bytes
}

fn parse(bytes: &[u8]) -> Value {
    serde_json::from_slice(bytes).expect("the export is valid JSON")
}

fn hashes(replay: &Value) -> Vec<String> {
    replay["days"]
        .as_array()
        .expect("days is a list")
        .iter()
        .map(|day| day["hash"].as_str().expect("a hash").to_string())
        .collect()
}

#[test]
fn every_scenario_exports_what_the_world_logged() {
    for scenario in scenarios() {
        let bytes = export(&format!("log-{}", stem(&scenario)), &scenario, &[]);
        assert!(bytes.ends_with(b"\n"), "{scenario}: no trailing newline");
        let replay = parse(&bytes);
        assert_eq!(replay["format"], "flagplot-replay");
        assert_eq!(replay["version"], 1);
        assert_eq!(replay["scenario"]["path"], scenario.as_str());

        let mut loaded = Scenario::load(&Path::new(ROOT).join(&scenario)).expect("loads");
        loaded.run_to_end().expect("runs");
        let world = &loaded.world;

        assert_eq!(replay["scenario"]["seed"], world.seed());
        let events = replay["events"].as_array().expect("events is a list");
        assert_eq!(
            events.len(),
            world.events().len(),
            "{scenario}: event count"
        );
        for (exported, event) in events.iter().zip(world.events().iter()) {
            assert_eq!(exported["text"], event.to_string(), "{scenario}");
            assert_eq!(exported["id"], event.id.raw(), "{scenario}");
            assert_eq!(
                exported["causes"].as_array().expect("causes").len(),
                event.causes.len(),
                "{scenario}"
            );
            assert_eq!(
                exported["entities"].as_array().expect("entities").len(),
                event.kind.entities().len(),
                "{scenario}"
            );
            assert!(exported["data"].is_object(), "{scenario}");
        }

        let expected: Vec<String> = world
            .day_hashes()
            .iter()
            .map(|hash| format!("{hash:016x}"))
            .collect();
        assert_eq!(hashes(&replay), expected, "{scenario}: day hashes");
        let days = replay["days"].as_array().expect("days is a list");
        let counted: u64 = days
            .iter()
            .map(|day| day["event_count"].as_u64().expect("a count"))
            .sum();
        // Events recorded before day 1 (contacts held at the start, and
        // their ageing to the start) belong to no day.
        let before_day_1 = events
            .iter()
            .filter(|event| event["day"].as_u64() == Some(0))
            .count();
        assert_eq!(
            counted as usize + before_day_1,
            events.len(),
            "{scenario}: days cover events"
        );

        let orders = replay["orders"].as_array().expect("orders is a list");
        assert_eq!(orders.len(), world.orders().count(), "{scenario}: orders");
    }
}

#[test]
fn exporting_twice_gives_identical_bytes() {
    for scenario in scenarios() {
        let first = export(&format!("a-{}", stem(&scenario)), &scenario, &[]);
        let second = export(&format!("b-{}", stem(&scenario)), &scenario, &[]);
        assert!(first == second, "{scenario}: two exports differ");
    }
}

#[test]
fn the_top_level_keys_come_in_the_documented_order() {
    let bytes = export("order", "data/scenarios/04a_night_action.toml", &[]);
    let text = String::from_utf8(bytes).expect("UTF-8");
    let mut at = 0;
    for key in [
        "format",
        "version",
        "scenario",
        "map",
        "directions",
        "labels",
        "bases",
        "initial",
        "orders",
        "events",
        "days",
        "final",
    ] {
        // Top-level keys follow `}` or `]` or a value, never a nested key of
        // the same name that comes earlier, so search forward only.
        let found = text[at..]
            .find(&format!("\"{key}\":"))
            .unwrap_or_else(|| panic!("key {key} is missing or out of order"));
        at += found;
    }
}

#[test]
fn a_different_seed_changes_the_hashes() {
    let scenario = "data/scenarios/03_search.toml";
    let default = parse(&export("seed-default", scenario, &[]));
    let seven = parse(&export("seed-7", scenario, &["--seed", "7"]));
    assert_eq!(seven["scenario"]["seed"], 7);
    assert_ne!(default["scenario"]["seed"], seven["scenario"]["seed"]);
    assert_ne!(hashes(&default), hashes(&seven));
}

#[test]
fn labels_add_one_entry_per_row_of_the_anchors_file() {
    let anchors = "data/map/south_pacific_anchors.csv";
    let scenario = "data/scenarios/04b_santo_to_lunga.toml";

    let text = fs::read_to_string(Path::new(ROOT).join(anchors)).expect("anchors file");
    let mut reader = csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .from_reader(text.as_bytes());
    let places: Vec<String> = reader
        .records()
        .map(|record| record.expect("a row")[0].to_string())
        .collect();
    assert!(!places.is_empty());

    let with = parse(&export("labels", scenario, &["--labels", anchors]));
    let labels = with["labels"].as_array().expect("labels is a list");
    let exported: Vec<&str> = labels
        .iter()
        .map(|label| label["place"].as_str().expect("a place"))
        .collect();
    assert_eq!(exported, places);
    for label in labels {
        assert!(label["lat"].is_number() && label["lon"].is_number());
        assert!(label["feature"].is_string());
    }
    assert_eq!(with["map"]["real"], true);

    let without = parse(&export("no-labels", scenario, &[]));
    assert_eq!(without["labels"], serde_json::json!([]));
}

#[test]
fn labels_keep_their_decimal_text() {
    let bytes = export(
        "label-text",
        "data/scenarios/04b_santo_to_lunga.toml",
        &["--labels", "data/map/south_pacific_anchors.csv"],
    );
    let text = String::from_utf8(bytes).expect("UTF-8");
    // From the anchors file: rabaul, -4.22224, 152.16544.
    assert!(text.contains("\"lat\":-4.22224,\"lon\":152.16544"));
}

#[test]
fn only_generated_maps_are_real() {
    let placeholder = parse(&export("toml-map", "data/scenarios/01_transit.toml", &[]));
    assert_eq!(placeholder["map"]["real"], false);
    let hexes = placeholder["map"]["hexes"].as_array().expect("hexes");
    assert!(hexes.iter().all(|hex| hex.get("lat").is_none()));
    let real = parse(&export(
        "csv-map",
        "data/scenarios/04b_santo_to_lunga.toml",
        &[],
    ));
    assert_eq!(real["map"]["real"], true);
}

#[test]
fn a_bad_command_line_fails_without_writing() {
    let out = temp_file("never");
    let out_text = out.to_string_lossy().into_owned();
    let result = binary(&["export", "data/scenarios/01_transit.toml"]);
    assert!(!result.status.success());
    let result = binary(&[
        "export",
        "data/scenarios/does_not_exist.toml",
        "--out",
        &out_text,
    ]);
    assert!(!result.status.success());
    assert!(!out.exists());
}

#[test]
fn run_output_matches_the_golden_files() {
    let golden = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/run_baseline");
    // Scenarios whose event log is too large to keep as a baseline file: the
    // soak's is 1.98 MB (decision 0044). Their summary file, with every
    // day's hash, is still compared.
    const NO_EVENTS_BASELINE: [&str; 1] = ["08_soak"];
    for scenario in scenarios() {
        let name = stem(&scenario);
        for (suffix, extra) in [("", None), (".events", Some("--events"))] {
            if extra.is_some() && NO_EVENTS_BASELINE.contains(&name.as_str()) {
                continue;
            }
            let file = golden.join(format!("{name}{suffix}.txt"));
            let expected = fs::read(&file)
                .unwrap_or_else(|_| panic!("{scenario} has no golden file {}", file.display()));
            let mut args = vec!["run", scenario.as_str()];
            args.extend(extra);
            let result = binary(&args);
            assert!(result.status.success(), "{scenario} failed to run");
            assert!(
                result.stdout == expected,
                "{scenario}: `run {}` output differs from {}",
                extra.unwrap_or(""),
                file.display()
            );
        }
    }
}

/// A copy of scenario 4c2a with one Allied starting contact, 2 pulses old, at
/// (5,0), in the temp folder. Its relative paths are made absolute so that
/// they still resolve from there.
struct TempScenario(PathBuf);

impl TempScenario {
    fn with_contact(label: &str) -> TempScenario {
        let source =
            fs::read_to_string(Path::new(ROOT).join("data/scenarios/04c_three_forces.toml"))
                .expect("scenario 4c2a exists");
        let data = Path::new(ROOT)
            .join("data")
            .canonicalize()
            .expect("data folder");
        let source = source.replace("= \"../", &format!("= \"{}/", data.display()));
        let dir = std::env::temp_dir().join(format!("flagplot-export-test-{}", std::process::id()));
        fs::create_dir_all(&dir).expect("make the temp dir");
        let path = dir.join(format!("{label}.toml"));
        let text = format!(
            "{source}\n[[contact]]\nside = \"allied\"\nq = 5\nr = 0\nship_count = 3\nkind = \"cruiser\"\nage_pulses = 2\n"
        );
        fs::write(&path, text).expect("write the temp scenario");
        TempScenario(path)
    }

    fn path(&self) -> String {
        self.0.to_string_lossy().into_owned()
    }
}

impl Drop for TempScenario {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

#[test]
fn starting_contacts_are_events_before_day_one_in_no_days_range() {
    // One report and two ageing events on day 0 (age 2), ahead of day 1's
    // events in the events array, and in none of the days' ranges.
    let scenario = TempScenario::with_contact("export_day_zero");
    let replay = parse(&export("day-zero", &scenario.path(), &[]));
    let events = replay["events"].as_array().expect("events");
    let kinds: Vec<&str> = events
        .iter()
        .take(3)
        .map(|event| event["kind"].as_str().expect("a kind"))
        .collect();
    assert_eq!(kinds, ["report_received", "contact_aged", "contact_aged"]);
    for event in &events[..3] {
        assert_eq!(event["day"], 0);
        assert!(event["pulse"].is_null());
    }
    assert!(events[3..].iter().all(|event| event["day"] != 0));
    assert!(events[0]["text"]
        .as_str()
        .expect("text")
        .contains("held at the start: new contact:1"));

    let days = replay["days"].as_array().expect("days");
    assert_eq!(days[0]["first_event"], 3);
    let counted: u64 = days
        .iter()
        .map(|day| day["event_count"].as_u64().expect("a count"))
        .sum();
    assert_eq!(counted + 3, events.len() as u64);

    // The run command prints the day-0 events before day 1's.
    let result = binary(&["run", &scenario.path(), "--events"]);
    let text = String::from_utf8_lossy(&result.stdout).into_owned();
    let day0: Vec<&str> = text.lines().filter(|line| line.contains(" D0 ")).collect();
    assert_eq!(day0.len(), 3, "{text}");
    let first_day_one = text.find(" D1 ").expect("day 1 events");
    assert!(text.find(" D0 ").expect("day 0 events") < first_day_one);
}
