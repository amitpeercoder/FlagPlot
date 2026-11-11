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
