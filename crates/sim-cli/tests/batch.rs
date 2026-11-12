//! Integration tests of `sim-cli batch`. The summary is checked against what
//! `sim-cli run --seed s` prints for the same seeds.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
const FIGHT: &str = "data/scenarios/04a_night_action.toml";
const QUIET: &str = "data/scenarios/01_transit.toml";

fn binary(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_sim-cli"))
        .args(args)
        .current_dir(ROOT)
        .output()
        .expect("the binary starts")
}

fn temp_file(label: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("flagplot-batch-test-{}", std::process::id()));
    fs::create_dir_all(&dir).expect("make the temp dir");
    dir.join(format!("{label}.json"))
}

/// Runs a batch and returns the file's bytes.
fn batch(label: &str, scenario: &str, extra: &[&str]) -> Vec<u8> {
    let out = temp_file(label);
    let out_text = out.to_string_lossy().into_owned();
    let mut args = vec!["batch", scenario, "--out", out_text.as_str()];
    args.extend_from_slice(extra);
    let result = binary(&args);
    assert!(
        result.status.success(),
        "batch of {scenario} failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    let bytes = fs::read(&out).expect("the batch file exists");
    let _ = fs::remove_file(&out);
    bytes
}

fn parse(bytes: &[u8]) -> Value {
    serde_json::from_slice(bytes).expect("the batch is valid JSON")
}

fn ids_of(side: &str, summary: &Value) -> Vec<u64> {
    summary["ships"]
        .as_array()
        .expect("ships")
        .iter()
        .filter(|ship| ship["side"] == side)
        .map(|ship| ship["id"].as_u64().expect("id"))
        .collect()
}

#[test]
fn the_summary_adds_up_and_matches_single_runs() {
    let summary = parse(&batch("fight", FIGHT, &["--runs", "20"]));
    assert_eq!(summary["format"], "flag-plot-batch");
    assert_eq!(summary["version"], 1);
    assert_eq!(summary["runs"], 20);
    assert_eq!(summary["first_seed"], 1);
    assert_eq!(summary["scenario"]["file"], FIGHT);

    let per_ship = summary["per_ship"].as_array().expect("per_ship");
    let per_run = summary["per_run"].as_array().expect("per_run");
    assert_eq!(per_run.len(), 20);

    for side in ["japanese", "allied"] {
        let block = &summary["sides"][side];
        let ids = ids_of(side, &summary);
        assert_eq!(block["ships"], ids.len());
        let histogram = block["sunk_histogram"].as_array().expect("histogram");
        assert_eq!(histogram.len(), ids.len() + 1);
        let runs: u64 = histogram.iter().map(|n| n.as_u64().expect("count")).sum();
        assert_eq!(runs, 20, "{side}: histogram sums to runs");

        let total = block["sunk_total"].as_u64().expect("sunk_total");
        let from_ships: u64 = per_ship
            .iter()
            .filter(|ship| ids.contains(&ship["id"].as_u64().expect("id")))
            .map(|ship| ship["sunk_runs"].as_u64().expect("sunk_runs"))
            .sum();
        let from_runs: u64 = per_run
            .iter()
            .map(|run| {
                run["sunk"]
                    .as_array()
                    .expect("sunk")
                    .iter()
                    .filter(|id| ids.contains(&id.as_u64().expect("id")))
                    .count() as u64
            })
            .sum();
        assert_eq!(total, from_ships, "{side}: per ship");
        assert_eq!(total, from_runs, "{side}: per run");
    }
    assert!(per_ship.iter().all(|ship| ship["crippled_runs"].is_u64()));

    for index in [0usize, 9, 19] {
        let run = &per_run[index];
        let seed = run["seed"].as_u64().expect("seed");
        assert_eq!(seed, 1 + index as u64);
        let output = binary(&["run", FIGHT, "--seed", &seed.to_string()]);
        assert!(output.status.success());
        let text = String::from_utf8(output.stdout).expect("UTF-8");
        let last_hash = text
            .lines()
            .rfind(|line| line.starts_with("day "))
            .and_then(|line| line.split_whitespace().nth(3))
            .expect("a day line with a hash");
        assert_eq!(run["final_hash"], last_hash, "seed {seed}: hash");

        let sunk: Vec<u64> = text
            .lines()
            .filter(|line| line.contains(": SUNK;"))
            .map(|line| line.split_whitespace().next().expect("an id"))
            .map(|id| id.trim_start_matches(|c: char| !c.is_ascii_digit()))
            .map(|id| id.parse().expect("a ship id"))
            .collect();
        let mut sunk = sunk;
        sunk.sort_unstable();
        let listed: Vec<u64> = run["sunk"]
            .as_array()
            .expect("sunk")
            .iter()
            .map(|id| id.as_u64().expect("id"))
            .collect();
        assert_eq!(listed, sunk, "seed {seed}: sunk ships");
    }
}

#[test]
fn two_identical_invocations_give_identical_bytes() {
    let first = batch("a", FIGHT, &["--runs", "5", "--first-seed", "40"]);
    let second = batch("b", FIGHT, &["--runs", "5", "--first-seed", "40"]);
    assert!(first == second);
    assert!(first.ends_with(b"\n"));
    assert_eq!(parse(&first)["per_run"][0]["seed"], 40);
}

#[test]
fn a_scenario_without_fighting_loses_nothing() {
    let summary = parse(&batch("quiet", QUIET, &["--runs", "3"]));
    for side in ["japanese", "allied"] {
        let block = &summary["sides"][side];
        assert_eq!(block["sunk_total"], 0);
        let histogram = block["sunk_histogram"].as_array().expect("histogram");
        assert_eq!(histogram[0], 3);
        assert!(histogram[1..].iter().all(|n| n == 0));
    }
    for ship in summary["per_ship"].as_array().expect("per_ship") {
        assert_eq!(ship["sunk_runs"], 0);
        assert_eq!(ship["crippled_runs"], 0);
    }
}

#[test]
fn a_scenario_with_objectives_adds_a_margin_and_a_result_to_each_run_and_a_tally() {
    let scenario = "data/scenarios/05_eastern_solomons.toml";
    let summary = parse(&batch(
        "objectives",
        scenario,
        &["--runs", "4", "--first-seed", "1940"],
    ));
    let runs = summary["per_run"].as_array().expect("per_run");
    assert_eq!(runs.len(), 4);
    let tally = summary["results"].as_array().expect("a tally of results");
    let names: Vec<&str> = tally
        .iter()
        .map(|row| row["outcome"].as_str().expect("an outcome"))
        .collect();
    assert_eq!(
        names,
        [
            "decisive victory",
            "major victory",
            "minor victory",
            "stalemate",
            "minor defeat",
            "major defeat",
            "decisive defeat"
        ]
    );
    let total: u64 = tally
        .iter()
        .map(|row| row["runs"].as_u64().expect("runs"))
        .sum();
    assert_eq!(total, 4);
    // Each run's own result is counted in the tally, and agrees with what
    // `run --seed` prints for the seed.
    for run in runs {
        let outcome = run["allied_outcome"].as_str().expect("an outcome");
        let row = tally
            .iter()
            .find(|row| row["outcome"] == outcome)
            .expect("a row");
        assert!(row["runs"].as_u64().unwrap() >= 1);
        let seed = run["seed"].as_u64().expect("seed").to_string();
        let printed = binary(&["run", scenario, "--seed", &seed]);
        let text = String::from_utf8_lossy(&printed.stdout).into_owned();
        let margin = run["allied_margin"].as_str().expect("a margin");
        assert!(
            text.contains(&format!("margin {margin}, {outcome}")),
            "seed {seed}: {margin} {outcome} not in\n{text}"
        );
    }
}

#[test]
fn a_scenario_without_objectives_has_no_result_in_its_batch() {
    let summary = parse(&batch("no_objectives", QUIET, &["--runs", "2"]));
    assert!(summary.get("results").is_none());
    assert!(summary["per_run"][0].get("allied_margin").is_none());
    assert!(summary["per_run"][0].get("allied_outcome").is_none());
}

#[test]
fn bad_invocations_fail_and_write_nothing() {
    let out = temp_file("never");
    let _ = fs::remove_file(&out);
    let out_text = out.to_string_lossy().into_owned();
    let cases: Vec<Vec<&str>> = vec![
        vec!["batch", QUIET],
        vec!["batch", QUIET, "--out", &out_text, "--runs", "0"],
        vec!["batch", QUIET, "--out", &out_text, "--runs", "many"],
        vec!["batch", QUIET, "--out", &out_text, "--runs"],
        vec!["batch", QUIET, "--out", &out_text, "--first-seed", "x"],
        vec!["batch", QUIET, "--out", &out_text, "--bogus"],
        vec!["batch", "--out", &out_text],
        vec!["batch", "data/scenarios/missing.toml", "--out", &out_text],
    ];
    for args in cases {
        let result = binary(&args);
        assert!(!result.status.success(), "{args:?} should fail");
        assert!(!result.stderr.is_empty(), "{args:?}: no message");
        assert!(!out.exists(), "{args:?}: wrote a file");
    }
}

#[test]
fn a_batch_says_what_it_was_run_on_and_carries_the_callers_label() {
    // Decision 0057: the hash layout's version, the hash of the scenario
    // file and of every data file it names, and the label as given.
    let plain = parse(&batch("inputs-plain", FIGHT, &["--runs", "2"]));
    let inputs = &plain["inputs"];
    assert_eq!(inputs["state_version"], "flagplot-state-v2");
    let scenario = fs::read(PathBuf::from(ROOT).join(FIGHT)).expect("the scenario reads");
    let mut hasher = sim_core::hash::StateHasher::new();
    hasher.bytes(&scenario);
    assert_eq!(
        inputs["scenario_hash"],
        format!("{:016x}", hasher.finish()).as_str()
    );
    let data = inputs["data"].as_array().expect("data files");
    assert!(!data.is_empty());
    for file in data {
        assert!(file["file"].as_str().is_some_and(|f| !f.is_empty()));
        assert_eq!(file["hash"].as_str().expect("a hash").len(), 16);
    }
    assert!(plain.get("label").is_none());

    let labelled = parse(&batch(
        "inputs-labelled",
        FIGHT,
        &["--runs", "2", "--label", "abc123"],
    ));
    assert_eq!(labelled["label"], "abc123");
    // The label changes nothing else.
    assert_eq!(labelled["inputs"], plain["inputs"]);
    assert_eq!(labelled["per_run"], plain["per_run"]);
}
