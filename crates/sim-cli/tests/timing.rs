//! `sim-cli time` (decision 0100, 11 to 13): it runs, it reports its
//! sections, and it changes nothing in the game.

use std::process::{Command, Output};

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
const SMALL: &str = "data/scenarios/01_transit.toml";

fn binary(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_sim-cli"))
        .args(args)
        .current_dir(ROOT)
        .output()
        .expect("the binary starts")
}

#[test]
fn time_reports_its_sections_and_changes_nothing() {
    let run_before = binary(&["run", SMALL, "--events"]);
    let hash_before = binary(&["hash", SMALL]);
    assert!(run_before.status.success() && hash_before.status.success());

    let timed = binary(&[
        "time",
        "--machine",
        "test machine",
        "--repeats",
        "1",
        "--games",
        "1",
        "--days",
        "1",
        "--slice",
        SMALL,
        "--soak",
        SMALL,
        "--early-day",
        "0",
        "--late-day",
        "1",
        "--soak-late-day",
        "1",
    ]);
    let out = String::from_utf8_lossy(&timed.stdout).into_owned();
    assert!(
        timed.status.success(),
        "{out}{}",
        String::from_utf8_lossy(&timed.stderr)
    );
    assert!(out.contains("machine: test machine"), "{out}");
    for section in [
        "Start points",
        "a. One day resolved",
        "b. The same day with every question",
        "questions asked in all",
        "c. A copy of the world",
        "the world without its log",
        "d. 1 games of 1 days",
        "e. 1 games of 1 days",
        "What it means",
    ] {
        assert!(out.contains(section), "missing \"{section}\" in\n{out}");
    }

    assert_eq!(
        run_before.stdout,
        binary(&["run", SMALL, "--events"]).stdout
    );
    assert_eq!(hash_before.stdout, binary(&["hash", SMALL]).stdout);
}

#[test]
fn time_needs_a_machine() {
    let out = binary(&["time", "--repeats", "1"]);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("--machine"));
}
