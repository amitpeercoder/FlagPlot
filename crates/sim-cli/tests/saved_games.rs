//! Integration tests of `sim-cli run --save`, `replay` and `hash`, and of two
//! separate processes of the runner giving identical output (decisions 5 to 9
//! of `docs/decisions/0043-determinism-scenario.md`).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

fn binary(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_sim-cli"))
        .args(args)
        .current_dir(ROOT)
        .output()
        .expect("the binary starts")
}

fn temp_dir(label: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("flagplot-saved-{}-{label}", std::process::id()));
    fs::create_dir_all(&dir).expect("make the temp dir");
    dir
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

fn ok(output: &Output, what: &str) {
    assert!(
        output.status.success(),
        "{what} failed: {}",
        text(&output.stderr)
    );
}

/// Saves a run of `scenario` and returns the saved game's path.
fn save(scenario: &str, label: &str, extra: &[&str]) -> PathBuf {
    let file = temp_dir(label).join("game.json");
    let file_text = file.to_string_lossy().into_owned();
    let mut args = vec!["run", scenario, "--save", file_text.as_str()];
    args.extend_from_slice(extra);
    ok(&binary(&args), "run --save");
    file
}

/// The hashes `hash` prints: `day N <hash>` lines.
fn hash_lines(output: &Output) -> Vec<String> {
    text(&output.stdout).lines().map(str::to_string).collect()
}

const SCENARIOS: [&str; 5] = [
    "data/scenarios/01_transit.toml",
    "data/scenarios/04a_night_action.toml",
    "data/scenarios/05_eastern_solomons.toml",
    "data/scenarios/06_rabaul_guadalcanal.toml",
    "data/scenarios/07_full_slice.toml",
];

#[test]
fn save_then_replay_reproduces_every_hash() {
    for scenario in SCENARIOS {
        let stem = Path::new(scenario)
            .file_stem()
            .expect("a stem")
            .to_string_lossy()
            .into_owned();
        let game = save(scenario, &stem, &[]);
        let game_text = game.to_string_lossy().into_owned();
        let replayed = binary(&["replay", &game_text]);
        ok(&replayed, &format!("replay of {scenario}"));
        let direct = hash_lines(&binary(&["hash", scenario]));
        let printed: Vec<String> = text(&replayed.stdout)
            .lines()
            .filter(|l| l.starts_with("day "))
            .map(str::to_string)
            .collect();
        assert!(!direct.is_empty(), "{scenario}");
        assert_eq!(printed, direct, "{scenario}");
    }
}

#[test]
fn a_saved_game_keeps_its_seed() {
    let game = save(
        "data/scenarios/05_eastern_solomons.toml",
        "seeded",
        &["--seed", "7"],
    );
    let game_text = game.to_string_lossy().into_owned();
    let replayed = binary(&["replay", &game_text]);
    ok(&replayed, "replay");
    let direct = hash_lines(&binary(&[
        "hash",
        "data/scenarios/05_eastern_solomons.toml",
        "--seed",
        "7",
    ]));
    let printed: Vec<String> = text(&replayed.stdout)
        .lines()
        .filter(|l| l.starts_with("day "))
        .map(str::to_string)
        .collect();
    assert_eq!(printed, direct);
    let own = hash_lines(&binary(&[
        "hash",
        "data/scenarios/05_eastern_solomons.toml",
    ]));
    assert_ne!(direct, own);
}

#[test]
fn a_tampered_hash_fails_on_that_day_and_names_both_hashes() {
    let game = save("data/scenarios/02_supply_run.toml", "tamper", &[]);
    let original = fs::read_to_string(&game).expect("the save reads");
    let value: serde_json::Value = serde_json::from_str(&original).expect("JSON");
    let hashes = value["hashes"].as_array().expect("hashes");
    assert!(hashes.len() >= 5);
    let day = 4;
    let real = hashes[day - 1]["hash"]
        .as_str()
        .expect("a hash")
        .to_string();
    assert_eq!(hashes[day - 1]["day"], day);
    let fake = "0123456789abcdef";
    let tampered = original.replacen(&format!("\"{real}\""), &format!("\"{fake}\""), 1);
    assert_ne!(tampered, original);
    fs::write(&game, tampered).expect("write");
    let replayed = binary(&["replay", &game.to_string_lossy()]);
    assert!(!replayed.status.success());
    let error = text(&replayed.stderr);
    assert!(error.contains("day 4"), "{error}");
    assert!(error.contains(fake), "{error}");
    assert!(error.contains(&real), "{error}");
    // Days 1 to 3 matched and were printed; day 4 was the first that failed.
    let printed = text(&replayed.stdout);
    assert!(printed.contains("day 3 "), "{printed}");
    assert!(printed.contains("day 4 "), "{printed}");
    assert!(!printed.contains("day 5 "), "{printed}");
}

/// A copy of `scenario` in a temp folder whose file names are absolute, so
/// that it can be changed without touching the repository.
fn standalone(scenario: &str, label: &str) -> PathBuf {
    let dir = temp_dir(label);
    let source = fs::read_to_string(Path::new(ROOT).join(scenario)).expect("reads");
    let changed = source.replace("\"../", &format!("\"{ROOT}/data/"));
    let path = dir.join("scenario.toml");
    fs::write(&path, changed).expect("write");
    path
}

/// A copy of the figures files of `data/reference` in a temp folder, to be
/// read with `--reference`, so that one can be changed without touching the
/// repository.
fn reference_copy(label: &str) -> PathBuf {
    let dir = temp_dir(label).join("reference");
    fs::create_dir_all(&dir).expect("make the folder");
    for entry in fs::read_dir(Path::new(ROOT).join("data/reference")).expect("reads") {
        let entry = entry.expect("an entry");
        if entry.file_type().expect("a type").is_file() {
            fs::copy(entry.path(), dir.join(entry.file_name())).expect("copy");
        }
    }
    dir
}

#[test]
fn a_changed_scenario_file_or_data_file_is_refused_naming_the_file() {
    let scenario = standalone("data/scenarios/01_transit.toml", "changed");
    let reference = reference_copy("changed-reference");
    let search = reference.join("search.toml");
    let scenario_text = scenario.to_string_lossy().into_owned();
    let reference_text = reference.to_string_lossy().into_owned();
    let game = save(
        &scenario_text,
        "changed-save",
        &["--reference", &reference_text],
    );
    let game_text = game.to_string_lossy().into_owned();
    let replay_with = |extra: &[&str]| {
        let mut args = vec!["replay", game_text.as_str()];
        args.extend_from_slice(extra);
        binary(&args)
    };
    ok(
        &replay_with(&["--reference", &reference_text]),
        "replay of the untouched files",
    );

    // A game saved with a reference directory names that directory's files,
    // so it is not replayed without it (the scenario's own files are not the
    // ones it ran on).
    let without = replay_with(&[]);
    assert!(!without.status.success());
    let error = text(&without.stderr);
    assert!(error.contains("different data files"), "{error}");

    // A data file the scenario names, read from the reference directory,
    // changes.
    let original = fs::read_to_string(&search).expect("reads");
    fs::write(&search, format!("{original}\n# changed\n")).expect("write");
    let refused = replay_with(&["--reference", &reference_text]);
    assert!(!refused.status.success());
    let error = text(&refused.stderr);
    assert!(error.contains("search.toml"), "{error}");
    assert!(error.contains("changed"), "{error}");
    assert!(!text(&refused.stdout).contains("day 1 "), "nothing ran");
    fs::write(&search, original).expect("restore");
    ok(
        &replay_with(&["--reference", &reference_text]),
        "replay with the file restored",
    );

    // The scenario file changes.
    let scenario_source = fs::read_to_string(&scenario).expect("reads");
    fs::write(&scenario, format!("{scenario_source}\n# changed\n")).expect("write");
    let refused = replay_with(&["--reference", &reference_text]);
    assert!(!refused.status.success());
    let error = text(&refused.stderr);
    assert!(error.contains("scenario.toml"), "{error}");
    assert!(error.contains("changed"), "{error}");
}

#[test]
fn replay_submits_the_saved_orders_and_not_the_scenarios() {
    let game = save("data/scenarios/02_supply_run.toml", "edited", &[]);
    let original = fs::read_to_string(&game).expect("the save reads");
    // Day 2 of the supply run orders cruise speed. Save the same order as
    // full speed: the convoy then burns more fuel and the hashes differ from
    // day 2 on, though the scenario file is unchanged.
    let edited = original.replacen("\"speed\": \"cruise\"", "\"speed\": \"full\"", 1);
    assert_ne!(edited, original);
    fs::write(&game, edited).expect("write");
    let replayed = binary(&["replay", &game.to_string_lossy()]);
    assert!(!replayed.status.success());
    let error = text(&replayed.stderr);
    let printed = text(&replayed.stdout);
    // Day 1 is the same (the order is on day 2); the first difference is
    // day 2.
    assert!(printed.contains("day 1 "), "{printed}");
    assert!(error.contains("day 2"), "{error}");
    assert!(!printed.contains("day 3 "), "{printed}");

    // And the saved orders are what is run: an unedited save of the same
    // scenario replays cleanly, so the scenario's own orders are not the
    // difference.
    let plain = save("data/scenarios/02_supply_run.toml", "unedited", &[]);
    ok(&binary(&["replay", &plain.to_string_lossy()]), "replay");
}

#[test]
fn the_saved_game_holds_orders_with_their_origin_and_the_files_it_names() {
    let game = save("data/scenarios/07_full_slice.toml", "format", &[]);
    let value: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&game).expect("reads")).expect("JSON");
    assert_eq!(value["format"], "flagplot-saved-game");
    assert_eq!(value["version"], 21);
    assert_eq!(
        value["state_version"],
        std::str::from_utf8(sim_core::hash::STATE_VERSION).expect("the version is text")
    );
    assert_eq!(value["rules_version"], sim_core::world::RULES_VERSION);
    assert_eq!(
        value["scenario"]["file"],
        "data/scenarios/07_full_slice.toml"
    );
    assert_eq!(value["seed"], 1942);
    assert_eq!(value["data_files"].as_array().expect("files").len(), 17);
    assert_eq!(value["hashes"].as_array().expect("hashes").len(), 39);
    let orders = value["orders"].as_array().expect("orders");
    assert!(orders.len() > 50);
    assert_eq!(orders[0]["origin"]["issuer"], "player");
    assert!(orders[0]["origin"]["intent"].is_null());
}

/// A load of supply is saved as it always was, so a game saved before fuel
/// was a cargo still replays; a load of fuel names its cargo (decision 0046).
#[test]
fn a_load_order_is_saved_with_its_cargo_only_for_fuel() {
    let load_orders = |scenario: &str, label: &str| -> (Vec<serde_json::Value>, PathBuf) {
        let game = save(scenario, label, &[]);
        let value: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&game).expect("reads")).expect("JSON");
        let orders = value["orders"]
            .as_array()
            .expect("orders")
            .iter()
            .map(|entry| entry["order"].clone())
            .filter(|order| order["kind"] == "load")
            .collect();
        (orders, game)
    };
    let (supply, game) = load_orders("data/scenarios/02_supply_run.toml", "load-supply");
    assert_eq!(supply, vec![serde_json::json!({ "kind": "load" })]);
    ok(
        &binary(&["replay", &game.to_string_lossy()]),
        "replay of 02",
    );
    let (fuel, game) = load_orders("data/scenarios/02b_fuel_run.toml", "load-fuel");
    assert_eq!(
        fuel,
        vec![serde_json::json!({ "kind": "load", "cargo": "fuel" })]
    );
    ok(
        &binary(&["replay", &game.to_string_lossy()]),
        "replay of 02b",
    );
}

/// The soak (decision 0044): 180 days saved and replayed, and every one of
/// the 180 hashes the replay prints is the one a direct run prints.
#[test]
fn the_soak_saves_and_replays_all_180_hashes() {
    let scenario = "data/scenarios/08_soak.toml";
    let game = save(scenario, "soak", &[]);
    let value: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&game).expect("reads")).expect("JSON");
    assert_eq!(value["hashes"].as_array().expect("hashes").len(), 180);
    assert!(value["orders"].as_array().expect("orders").len() > 250);
    let replayed = binary(&["replay", &game.to_string_lossy()]);
    ok(&replayed, "replay of the soak");
    let printed: Vec<String> = text(&replayed.stdout)
        .lines()
        .filter(|l| l.starts_with("day "))
        .map(str::to_string)
        .collect();
    let direct = hash_lines(&binary(&["hash", scenario]));
    assert_eq!(direct.len(), 180);
    assert_eq!(printed, direct);
}

#[test]
fn hash_prints_what_run_prints() {
    for scenario in SCENARIOS {
        let hashed = hash_lines(&binary(&["hash", scenario]));
        let run = binary(&["run", scenario]);
        ok(&run, "run");
        let printed: Vec<String> = text(&run.stdout)
            .lines()
            .filter(|l| l.starts_with("day "))
            .map(|l| {
                // `day   1  hash 0123456789abcdef  53 events`
                let words: Vec<&str> = l.split_whitespace().collect();
                format!("day {} {}", words[1], words[3])
            })
            .collect();
        assert_eq!(hashed, printed, "{scenario}");
        assert!(hashed.iter().all(|l| l.split(' ').count() == 3));
    }
}

#[test]
fn two_processes_give_byte_identical_output() {
    for scenario in SCENARIOS {
        let a = binary(&["run", scenario, "--events"]);
        let b = binary(&["run", scenario, "--events"]);
        ok(&a, "first run");
        ok(&b, "second run");
        assert_eq!(a.stdout, b.stdout, "{scenario}");
        assert_eq!(
            binary(&["hash", scenario]).stdout,
            binary(&["hash", scenario]).stdout
        );
    }
    let a = binary(&[
        "run",
        "data/scenarios/07_full_slice.toml",
        "--events",
        "--seed",
        "3",
    ]);
    let b = binary(&[
        "run",
        "data/scenarios/07_full_slice.toml",
        "--events",
        "--seed",
        "3",
    ]);
    assert_eq!(a.stdout, b.stdout);
}

#[test]
fn every_scenario_in_the_folder_saves_and_replays() {
    let mut count = 0;
    for entry in fs::read_dir(Path::new(ROOT).join("data/scenarios")).expect("the folder") {
        let path = entry.expect("an entry").path();
        if path.extension().is_none_or(|ext| ext != "toml") {
            continue;
        }
        let name = path
            .file_name()
            .expect("a name")
            .to_string_lossy()
            .into_owned();
        let scenario = format!("data/scenarios/{name}");
        let game = save(&scenario, &format!("all-{name}"), &[]);
        ok(
            &binary(&["replay", &game.to_string_lossy()]),
            &format!("replay of {scenario}"),
        );
        count += 1;
    }
    assert!(count >= 24, "{count} scenarios");
}

/// Decision 0100, 8: a saved game records what it was played under, and a
/// replay of a game played under anything else is refused before it runs.
/// `change` edits the saved game's JSON; the returned text is the refusal.
fn refusal_of_edited_save(label: &str, change: impl FnOnce(&mut serde_json::Value)) -> String {
    let game = save("data/scenarios/01_transit.toml", label, &[]);
    let mut value: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&game).expect("reads")).expect("JSON");
    change(&mut value);
    fs::write(&game, serde_json::to_string_pretty(&value).expect("JSON")).expect("writes");
    let game = game.to_string_lossy().into_owned();
    let replay = binary(&["replay", &game]);
    assert!(!replay.status.success(), "the replay was refused");
    assert!(
        !text(&replay.stdout).contains("day 1 "),
        "refused before it ran: {}",
        text(&replay.stdout)
    );
    // `events` and `export` read it the same way.
    let events = binary(&["events", &game, "--unit", "ship:1"]);
    assert!(!events.status.success());
    assert_eq!(events.stderr, replay.stderr, "events says the same");
    let export = binary(&["export", &game, "--out", &format!("{game}.out.json")]);
    assert!(!export.status.success());
    assert_eq!(export.stderr, replay.stderr, "export says the same");
    text(&replay.stderr)
}

#[test]
fn a_saved_game_of_the_current_format_replays() {
    let game = save("data/scenarios/01_transit.toml", "v2-replays", &[]);
    ok(&binary(&["replay", &game.to_string_lossy()]), "replay");
}

#[test]
fn a_version_1_saved_game_is_refused_with_the_reason() {
    let error = refusal_of_edited_save("v1", |v| {
        v["version"] = serde_json::json!(1);
        let map = v.as_object_mut().expect("object");
        map.remove("state_version");
        map.remove("rules_version");
    });
    assert!(
        error.contains("saved before saved games recorded the rules"),
        "{error}"
    );
    assert!(error.contains("not converted"), "{error}");
}

#[test]
fn a_saved_game_of_other_rules_is_refused_naming_both() {
    let error = refusal_of_edited_save("rules", |v| v["rules_version"] = serde_json::json!(99));
    assert!(error.contains("rules version 99"), "{error}");
    assert!(
        error.contains(&format!(
            "engine has rules version {}",
            sim_core::world::RULES_VERSION
        )),
        "{error}"
    );
}

#[test]
fn a_saved_game_of_another_state_layout_is_refused_naming_both() {
    let error = refusal_of_edited_save("state", |v| {
        v["state_version"] = serde_json::json!("flagplot-state-v0");
    });
    assert!(error.contains("flagplot-state-v0"), "{error}");
    assert!(
        error.contains(std::str::from_utf8(sim_core::hash::STATE_VERSION).expect("text")),
        "{error}"
    );
}

// ---------------------------------------------------------------------------
// Commands and the staff in a saved game (decision 0107, "As built").
// ---------------------------------------------------------------------------

/// The commander's commands for scenario 1, a small game of three days: an
/// intent to keep Espiritu Santo supplied with the transit force as its pool,
/// a risk and a leave a day later, and the plan approved on the third.
const COMMANDS: &str = r#"
[[command]]
day = 1
side = "allied"
kind = "set_intent"
intent = 1
base = 2
floor_days = 10
interval_days = 15

[[command]]
day = 1
side = "allied"
kind = "set_limit"
limit = "forces"
intent = 1
ships = [1, 2]
task_forces = [1]
source = 1

[[command]]
day = 2
side = "allied"
kind = "set_limit"
limit = "risk"
intent = 1
risk = "press"

[[command]]
day = 2
side = "allied"
kind = "set_leave"
leave = "launch"
given = true

[[command]]
day = 3
side = "allied"
kind = "approve"
plan = 1
"#;

/// A copy of scenario 1 with the commands above, in a temp folder.
fn with_commands(label: &str) -> String {
    let path = standalone("data/scenarios/01_transit.toml", label);
    let mut text = fs::read_to_string(&path).expect("reads");
    text.push_str(COMMANDS);
    fs::write(&path, text).expect("writes");
    path.to_string_lossy().into_owned()
}

fn printed_hashes(output: &Output) -> Vec<String> {
    text(&output.stdout)
        .lines()
        .filter(|l| l.starts_with("day "))
        .map(str::to_string)
        .collect()
}

#[test]
fn a_scenario_with_commands_saves_them_and_replays_to_the_same_hashes() {
    let scenario = with_commands("commands-replay");
    let game = save(&scenario, "commands-replay-save", &[]);
    let value: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&game).expect("reads")).expect("JSON");
    assert_eq!(value["version"], 21);
    assert_eq!(value["doctrine_version"], sim_staff::DOCTRINE_VERSION);
    let commands = value["commands"].as_array().expect("commands");
    assert_eq!(commands.len(), 5);
    assert_eq!(commands[0]["day"], 1);
    assert_eq!(commands[0]["side"], "allied");
    assert_eq!(commands[0]["command"]["kind"], "set_intent");
    assert_eq!(commands[1]["command"]["ships"], serde_json::json!([1, 2]));
    // The supply staff of each side is asked every day; the game's intent has
    // a pool, so on day 1 the Allied staff prepares the draft and says so.
    let staff = value["staff"].as_array().expect("staff");
    assert!(!staff.is_empty());
    assert_eq!(staff[0]["day"], 1);
    assert_eq!(staff[0]["side"], "allied");
    assert_eq!(staff[0]["decisions"]["commands"][0]["kind"], "approve");

    // The replay hands in the saved commands, and its hashes are the run's.
    let replayed = binary(&["replay", &game.to_string_lossy()]);
    ok(&replayed, "replay");
    let direct = hash_lines(&binary(&["hash", &scenario]));
    assert_eq!(direct.len(), 3);
    assert_eq!(printed_hashes(&replayed), direct);
    // The commands changed the game: scenario 1 without them hashes otherwise.
    let plain = hash_lines(&binary(&["hash", "data/scenarios/01_transit.toml"]));
    assert_ne!(direct, plain);

    // A replay hands in the saved commands, not the scenario's: without them
    // the replay is the plain game, and the hashes no longer match.
    let mut edited = value.clone();
    edited["commands"] = serde_json::json!([]);
    let stripped = game.with_file_name("stripped.json");
    fs::write(&stripped, edited.to_string()).expect("writes");
    let refused = binary(&["replay", &stripped.to_string_lossy()]);
    assert!(!refused.status.success());
    assert!(text(&refused.stderr).contains("replay differs on day 1"));
}

#[test]
fn replay_check_staff_finds_no_difference_where_the_staff_gives_what_it_gave() {
    let scenario = with_commands("commands-check");
    let game = save(&scenario, "commands-check-save", &[]);
    let checked = binary(&["replay", &game.to_string_lossy(), "--check-staff"]);
    ok(&checked, "replay --check-staff");
    let out = text(&checked.stdout);
    assert!(
        out.contains("the staff gave what was saved on every day"),
        "{out}"
    );
    // The hashes are printed as a replay prints them.
    assert_eq!(
        printed_hashes(&checked),
        hash_lines(&binary(&["hash", &scenario]))
    );
}

#[test]
fn replay_check_staff_names_the_first_day_and_side_where_the_doctrines_staff_gives_something_else()
{
    let scenario = with_commands("commands-staff");
    let game = save(&scenario, "commands-staff-save", &[]);
    let mut value: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&game).expect("reads")).expect("JSON");
    // The saved hand-ins are the staff's own: asked again it gives them. Two
    // are altered, the Allied side's on day 2 and the first of day 1's
    // reasons: the first difference is day 1, Allied, command 1.
    let staff = value["staff"].as_array_mut().expect("staff");
    let first = staff
        .iter_mut()
        .find(|e| e["day"] == 1 && e["side"] == "allied")
        .expect("the Allied staff handed in on day 1");
    first["decisions"]["commands"][0]["reason"]["worked"] =
        serde_json::json!("the saved staff said something else");
    let later = staff
        .iter_mut()
        .find(|e| e["day"] == 2 && e["side"] == "allied")
        .expect("the Allied staff handed in on day 2");
    later["decisions"]["no_orders"] = serde_json::json!([]);
    let edited = game.with_file_name("edited.json");
    fs::write(&edited, value.to_string()).expect("writes");
    let edited = edited.to_string_lossy().into_owned();
    let checked = binary(&["replay", &edited, "--check-staff"]);
    assert!(!checked.status.success());
    let error = text(&checked.stderr);
    assert!(
        error.contains("the staff differs on day 1, allied side: command 1"),
        "{error}"
    );
    assert!(
        !text(&checked.stdout).contains("the staff gave what was saved"),
        "{}",
        text(&checked.stdout)
    );
    assert!(!error.contains("day 2"), "stops at the first: {error}");
}

#[test]
fn a_saved_game_of_version_2_is_refused_not_converted() {
    let error = refusal_of_edited_save("v2", |v| {
        v["version"] = serde_json::json!(2);
        let map = v.as_object_mut().expect("object");
        map.remove("commands");
        map.remove("staff");
        map.remove("doctrine_version");
    });
    assert!(error.contains("saved game version 2, now 21"), "{error}");
    assert!(error.contains("not converted"), "{error}");
}

#[test]
fn the_export_of_a_run_with_commands_carries_each_sides_command_state() {
    let scenario = with_commands("commands-export");
    let out = temp_dir("commands-export-out").join("replay.json");
    let out_text = out.to_string_lossy().into_owned();
    ok(
        &binary(&["export", &scenario, "--out", &out_text]),
        "export",
    );
    let replay: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&out).expect("reads")).expect("JSON");
    let allied = &replay["final"]["commands"]["allied"];
    assert_eq!(allied["intents"][0]["id"], 1);
    assert_eq!(allied["intents"][0]["kind"], "keep_supplied");
    assert_eq!(allied["intents"][0]["base"], 2);
    assert_eq!(allied["intents"][0]["limits"]["risk"], "press");
    assert_eq!(allied["intents"][0]["limits"]["forces"]["source"], 1);
    assert_eq!(allied["leaves"]["launch"], true);
    // The staff prepares the draft on day 1 (leave to prepare is given by
    // default) and releases it on day 2, when the commander gives leave to
    // launch; the commander's approval on day 3 comes to a plan already
    // released and is refused.
    assert_eq!(allied["plans"][0]["state"], "released");
    assert_eq!(allied["plans"][0]["baseline"]["day"], 1);
    // The Japanese side was never given a command: no key.
    assert!(replay["final"]["commands"].get("japanese").is_none());
    // The steps: the first step has none; the state appears in a step of day
    // 1 and again when it changes; the last step's state is `final`'s.
    let steps = replay["steps"].as_array().expect("steps");
    assert!(steps[0].get("commands").is_none());
    let with: Vec<&serde_json::Value> = steps
        .iter()
        .filter(|s| s.get("commands").is_some())
        .collect();
    assert!(!with.is_empty());
    assert_eq!(with.last().expect("one")["commands"]["allied"], *allied);
    assert!(with.iter().all(|s| s["commands"].get("japanese").is_none()));

    // A run with no command has no such keys anywhere.
    let plain = temp_dir("commands-export-plain").join("replay.json");
    let plain_text = plain.to_string_lossy().into_owned();
    ok(
        &binary(&[
            "export",
            "data/scenarios/01_transit.toml",
            "--out",
            &plain_text,
        ]),
        "export",
    );
    let plain_bytes = fs::read_to_string(&plain).expect("reads");
    assert!(!plain_bytes.contains("\"commands\""));
}

#[test]
fn a_game_played_through_serve_saves_the_scenarios_commands_and_replays() {
    use std::io::Write;
    use std::process::Stdio;
    let scenario = with_commands("commands-serve");
    let dir = temp_dir("commands-serve-out");
    let game = dir.join("game.json");
    let mut child = Command::new(env!("CARGO_BIN_EXE_sim-cli"))
        .args([
            "serve",
            &scenario,
            "--seat",
            "allied=player",
            "--save",
            &game.to_string_lossy(),
        ])
        .current_dir(ROOT)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the binary starts");
    let mut input = String::new();
    for id in 1..=3 {
        input.push_str(&format!(
            "{{\"id\": {id}, \"seat\": \"allied\", \"tool\": \"end_day\", \"arguments\": {{}}}}\n"
        ));
    }
    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(input.as_bytes())
        .expect("input is written");
    let output = child.wait_with_output().expect("finishes");
    ok(&output, "serve");
    let value: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&game).expect("reads")).expect("JSON");
    assert_eq!(value["version"], 21);
    assert_eq!(value["commands"].as_array().expect("commands").len(), 5);
    // The staff the session was given handed in what it decided on day 1.
    assert_eq!(value["staff"][0]["day"], 1);
    assert_eq!(value["staff"][0]["side"], "allied");
    assert_eq!(
        value["staff"][0]["decisions"]["commands"][0]["kind"],
        "approve"
    );
    // The game replays to the hashes it was saved with: the player gave no
    // orders, so these are not the scenario's own run's, and the commands
    // handed in are the script's.
    let replayed = binary(&["replay", &game.to_string_lossy()]);
    ok(&replayed, "replay");
    let saved: Vec<String> = value["hashes"]
        .as_array()
        .expect("hashes")
        .iter()
        .map(|h| format!("day {} {}", h["day"], h["hash"].as_str().expect("hash")))
        .collect();
    assert_eq!(printed_hashes(&replayed), saved);
}

#[test]
fn a_game_file_is_resumed_only_under_the_doctrine_it_was_begun_under() {
    let dir = temp_dir("play-doctrine");
    let game = dir.join("g.txt");
    let game_text = game.to_string_lossy().into_owned();
    let begin = binary(&[
        "play",
        "--game",
        &game_text,
        "--scenario",
        "data/scenarios/01_transit.toml",
        "--seat",
        "allied",
        "get_briefing",
    ]);
    ok(&begin, "play");
    let content = fs::read_to_string(&game).expect("reads");
    let header: serde_json::Value =
        serde_json::from_str(content.lines().next().expect("a header")).expect("JSON");
    assert_eq!(header["doctrine_version"], sim_staff::DOCTRINE_VERSION);
    // The same doctrine resumes.
    ok(
        &binary(&["play", "--game", &game_text, "get_briefing"]),
        "resume",
    );
    // Another doctrine is refused, naming both versions.
    let own = format!("\"doctrine_version\":\"{}\"", sim_staff::DOCTRINE_VERSION);
    let changed = content.replacen(&own, "\"doctrine_version\":\"keep-supplied-1\"", 1);
    assert_ne!(changed, content);
    fs::write(&game, changed).expect("writes");
    let refused = binary(&["play", "--game", &game_text, "get_briefing"]);
    assert!(!refused.status.success());
    let error = text(&refused.stderr);
    assert!(
        error.contains("keep-supplied-1") && error.contains(sim_staff::DOCTRINE_VERSION),
        "{error}"
    );
    // A game file made before doctrines were recorded has no version, and is
    // read as `none`: no doctrine existed, and every side has a staff now, so
    // it is not resumed either.
    let old = content.replacen(&format!("{own},"), "", 1);
    assert_ne!(old, content);
    fs::write(&game, old).expect("writes");
    let refused = binary(&["play", "--game", &game_text, "get_briefing"]);
    assert!(!refused.status.success());
    let error = text(&refused.stderr);
    assert!(
        error.contains("\"none\"") && error.contains(sim_staff::DOCTRINE_VERSION),
        "{error}"
    );
}

/// Decision 0156, scenario 78: a saved game keeps the pulse of an order
/// given inside a day (and none on an order of dusk), and its replay hands
/// the order in at that pulse, to the same daily hashes. The same file with
/// the order's pulse changed to the afternoon, and its hashes to those of
/// the scenario with the order at 1200 (case B), replays to case B's hashes.
#[test]
fn scenario_78_saves_and_replays_with_the_pulse_of_its_order() {
    let scenario = "data/scenarios/78_orders_inside_a_day.toml";
    let case_a = hash_lines(&binary(&["hash", scenario]));
    assert_eq!(case_a.len(), 3);
    let game = save(scenario, "78", &[]);
    let game_text = game.to_string_lossy().into_owned();
    let replayed_lines = |output: &Output| -> Vec<String> {
        text(&output.stdout)
            .lines()
            .filter(|l| l.starts_with("day "))
            .map(str::to_string)
            .collect()
    };
    let replayed = binary(&["replay", &game_text]);
    ok(&replayed, "replay of scenario 78");
    assert_eq!(replayed_lines(&replayed), case_a);

    let mut value: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&game).expect("reads")).expect("JSON");
    assert_eq!(value["version"], 21);
    let orders = value["orders"].as_array().expect("orders");
    assert_eq!(orders.len(), 3);
    assert!(orders[..2].iter().all(|order| order.get("pulse").is_none()));
    assert_eq!(orders[2]["pulse"], "morning");
    assert_eq!(orders[2]["day"], 1);

    // Case B's hashes: the scenario with its order at 1200.
    let copy = standalone(scenario, "78-b");
    let source = fs::read_to_string(&copy).expect("reads");
    assert_eq!(source.matches("pulse = \"morning\"").count(), 1);
    fs::write(
        &copy,
        source.replace("pulse = \"morning\"", "pulse = \"afternoon\""),
    )
    .expect("writes");
    let case_b = hash_lines(&binary(&["hash", &copy.to_string_lossy()]));
    assert_eq!(case_b.len(), 3);
    assert_ne!(case_a[0], case_b[0]);

    value["orders"][2]["pulse"] = serde_json::json!("afternoon");
    value["hashes"] = serde_json::Value::Array(
        case_b
            .iter()
            .map(|line| {
                let words: Vec<&str> = line.split(' ').collect();
                serde_json::json!({
                    "day": words[1].parse::<u32>().expect("a day"),
                    "hash": words[2],
                })
            })
            .collect(),
    );
    fs::write(&game, serde_json::to_string_pretty(&value).expect("JSON")).expect("writes");
    let replayed = binary(&["replay", &game_text]);
    ok(&replayed, "replay of scenario 78 at the afternoon");
    assert_eq!(replayed_lines(&replayed), case_b);
}
