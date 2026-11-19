//! A game file pinned to its inputs (decision 0172, 1): `play` records in
//! the header of a new game file the seed the game is played on, the hash of
//! the scenario file and of each data file it reads, and the rules version,
//! by the means a saved game uses (`save.rs`), and refuses a call on a game
//! whose inputs have changed since, naming what changed and writing nothing.
//!
//! # What the tests are to show
//!
//! Written by the architect in decision 0172; the tests are written to it.
//!
//! 1. **A game file.**
//!    - A game of scenario 1 begun and given one `end_day`, then called
//!      again with nothing changed: it resumes on day 1, not over, as before.
//!    - The same after the scenario's `days` is changed from 3 to 1 (in a
//!      copy made by the test): the next call is refused, names the scenario
//!      file as changed, and the game file is byte for byte what it was.
//!    - The same after a figures file the scenario reads is changed in a
//!      copy: refused, naming that file.
//!    - A game file with no record of its inputs: a reading call replays it;
//!      a call that would add to it is refused; with the caller's word it is
//!      continued.
//!    - Every recorded game of the repository replays as it did.
//!
//! "Resumes on day 1, not over" is read from the briefing: one day resolved
//! ("Day: 2 of 3", covering day 1), no result. "A reading call" is an export
//! (`--export-transcript`): every tool call `play` answers is added to the
//! game file, so the export is the one way to read a game without adding to
//! it. The caller's word is `--unpinned`.
//!
//! The recorded games (`tests/recorded_games/*/game.jsonl`) are game files
//! with no record of their inputs; `every_recorded_game_replays_to_its_
//! transcript_and_save` in `tests/mcp.rs` replays each, unchanged, to its
//! transcript and saved game. The last test here checks that they are such
//! files, so that the test in `mcp.rs` is the one the record asks for.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
const BIN: &str = env!("CARGO_BIN_EXE_sim-cli");

fn temp_dir(label: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("flagplot-pin-{}-{label}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("make the temp dir");
    dir
}

fn s(path: &Path) -> &str {
    path.to_str().expect("a path in UTF-8")
}

fn run(args: &[&str]) -> Output {
    Command::new(BIN)
        .args(args)
        .current_dir(ROOT)
        .output()
        .expect("the binary runs")
}

fn out(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn err(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn ok(output: &Output, what: &str) {
    assert!(
        output.status.success(),
        "{what}: {}{}",
        out(output),
        err(output)
    );
}

/// Scenario 1 and every file it reads, copied into `dir` in the same layout
/// (`scenarios/`, `map/`, `reference/`), so that the test may change them.
/// Returns the copy of the scenario.
fn copy_of_scenario_1(dir: &Path) -> PathBuf {
    let root = Path::new(ROOT).join("data");
    for (from, to) in [
        ("scenarios/01_transit.toml", "scenarios/01_transit.toml"),
        (
            "map/placeholder_transit.toml",
            "map/placeholder_transit.toml",
        ),
        ("reference/ship_classes.csv", "reference/ship_classes.csv"),
        ("reference/port_sizes.csv", "reference/port_sizes.csv"),
        (
            "reference/aircraft_types.csv",
            "reference/aircraft_types.csv",
        ),
        ("reference/search.toml", "reference/search.toml"),
        (
            "reference/surface_combat.toml",
            "reference/surface_combat.toml",
        ),
    ] {
        let target = dir.join(to);
        fs::create_dir_all(target.parent().expect("a folder")).expect("make the folder");
        fs::copy(root.join(from), &target).expect("copy a data file");
    }
    dir.join("scenarios/01_transit.toml")
}

/// Asserts the game has resolved one day of three and is not over.
fn resumes_on_day_1(game: &Path) {
    let briefing = run(&["play", "--game", s(game), "get_briefing"]);
    ok(&briefing, "the resumed briefing");
    let text = out(&briefing);
    assert!(text.contains("- Day: 2 of 3"), "{text}");
    assert!(
        text.contains("- Covers: day 1 and the state at the start of day 2"),
        "{text}"
    );
    assert!(!text.contains("RESULT"), "{text}");
}

#[test]
fn a_game_file_is_refused_a_call_when_its_scenario_or_a_data_file_has_changed() {
    let dir = temp_dir("changed");
    let scenario = copy_of_scenario_1(&dir.join("data"));
    let game = dir.join("g.txt");
    ok(
        &run(&[
            "play",
            "--game",
            s(&game),
            "--scenario",
            s(&scenario),
            "--seat",
            "allied",
            "end_day",
        ]),
        "the first end_day",
    );
    // The header records the inputs.
    let content = fs::read_to_string(&game).expect("the game file reads");
    let header: Value =
        serde_json::from_str(content.lines().next().expect("a header")).expect("JSON");
    let inputs = &header["inputs"];
    assert_eq!(inputs["seed"], 1942, "{header}");
    assert_eq!(
        inputs["rules_version"],
        sim_core::world::RULES_VERSION,
        "{header}"
    );
    assert!(inputs["scenario_hash"].is_string(), "{header}");
    let files: Vec<&str> = inputs["data_files"]
        .as_array()
        .expect("data files")
        .iter()
        .map(|f| f["file"].as_str().expect("a file"))
        .collect();
    assert_eq!(files.len(), 6, "{header}");
    assert!(
        files.iter().any(|f| f.ends_with("reference/search.toml")),
        "{header}"
    );

    // Nothing changed: it resumes on day 1, not over, as before.
    resumes_on_day_1(&game);
    let before = fs::read(&game).expect("the game file reads");

    // The scenario's days changed from 3 to 1: refused, naming the scenario
    // file, and the game file is what it was.
    let original = fs::read_to_string(&scenario).expect("the scenario reads");
    let changed = original.replacen("days = 3\n", "days = 1\n", 1);
    assert_ne!(changed, original);
    fs::write(&scenario, &changed).expect("the scenario is changed");
    let refused = run(&["play", "--game", s(&game), "get_briefing"]);
    assert_eq!(refused.status.code(), Some(1), "{}", out(&refused));
    let why = err(&refused);
    assert!(
        why.contains(&format!("the scenario file {} has changed", s(&scenario))),
        "{why}"
    );
    assert_eq!(fs::read(&game).expect("reads"), before);
    // Nor is an export of it made.
    let transcript = dir.join("t.jsonl");
    let refused = run(&[
        "play",
        "--game",
        s(&game),
        "--export-transcript",
        s(&transcript),
    ]);
    assert_eq!(refused.status.code(), Some(1));
    assert!(!transcript.exists());

    // Put back, it resumes again.
    fs::write(&scenario, &original).expect("the scenario is put back");
    resumes_on_day_1(&game);
    let before = fs::read(&game).expect("the game file reads");

    // A figures file the scenario reads, changed: refused, naming it.
    let search = dir.join("data/reference/search.toml");
    let mut figures = fs::read_to_string(&search).expect("the figures read");
    figures.push_str("\n# changed by the test\n");
    fs::write(&search, figures).expect("the figures are changed");
    let refused = run(&["play", "--game", s(&game), "get_briefing"]);
    assert_eq!(refused.status.code(), Some(1), "{}", out(&refused));
    let why = err(&refused);
    assert!(why.contains("the data file "), "{why}");
    assert!(why.contains("reference/search.toml has changed"), "{why}");
    assert!(!why.contains("scenario file"), "{why}");
    assert_eq!(fs::read(&game).expect("reads"), before);

    // The caller's word does not lift the pin.
    let refused = run(&["play", "--game", s(&game), "--unpinned", "get_briefing"]);
    assert_eq!(refused.status.code(), Some(1));
    assert_eq!(fs::read(&game).expect("reads"), before);
}

#[test]
fn a_game_file_with_no_record_of_its_inputs_is_read_and_continued_only_on_the_callers_word() {
    let dir = temp_dir("unpinned");
    let game = dir.join("g.txt");
    ok(
        &run(&[
            "play",
            "--game",
            s(&game),
            "--scenario",
            "data/scenarios/01_transit.toml",
            "--seat",
            "allied",
            "end_day",
        ]),
        "the first end_day",
    );
    // The game file as one made before game files were pinned: its header
    // without the record of its inputs.
    let content = fs::read_to_string(&game).expect("the game file reads");
    let (first, rest) = content.split_once('\n').expect("a header line");
    let mut header: Value = serde_json::from_str(first).expect("JSON");
    assert!(header
        .as_object_mut()
        .expect("an object")
        .remove("inputs")
        .is_some());
    let old = format!("{}\n{rest}", serde_json::to_string(&header).expect("JSON"));
    fs::write(&game, &old).expect("the game file is written");

    // A reading call replays it.
    let transcript = dir.join("t.jsonl");
    ok(
        &run(&[
            "play",
            "--game",
            s(&game),
            "--export-transcript",
            s(&transcript),
        ]),
        "the export",
    );
    let lines = fs::read_to_string(&transcript).expect("the transcript reads");
    assert_eq!(lines.lines().count(), 1);
    assert!(lines.contains("\"end_day\""), "{lines}");

    // A call that would add to it is refused, and nothing is written.
    let refused = run(&["play", "--game", s(&game), "get_briefing"]);
    assert_eq!(refused.status.code(), Some(1), "{}", out(&refused));
    assert!(err(&refused).contains("--unpinned"), "{}", err(&refused));
    assert_eq!(fs::read_to_string(&game).expect("reads"), old);

    // With the caller's word it is continued, and stays as it was begun.
    ok(
        &run(&["play", "--game", s(&game), "--unpinned", "get_briefing"]),
        "the call with --unpinned",
    );
    let now = fs::read_to_string(&game).expect("reads");
    assert!(now.starts_with(&old), "the file is added to");
    assert_eq!(now.lines().count(), old.lines().count() + 1);
    assert!(!now.lines().next().expect("a header").contains("inputs"));

    // A new game is always pinned: the word is refused there.
    let new = dir.join("new.txt");
    let refused = run(&[
        "play",
        "--game",
        s(&new),
        "--scenario",
        "data/scenarios/01_transit.toml",
        "--seat",
        "allied",
        "--unpinned",
        "get_briefing",
    ]);
    assert_eq!(refused.status.code(), Some(1));
    assert!(!new.exists());
}

#[test]
fn the_recorded_games_are_game_files_with_no_record_of_their_inputs() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/recorded_games");
    let mut games = 0;
    for entry in fs::read_dir(&root).expect("the recorded games read") {
        let file = entry.expect("an entry").path().join("game.jsonl");
        if !file.exists() {
            continue;
        }
        let content = fs::read_to_string(&file).expect("a game file reads");
        let header: Value =
            serde_json::from_str(content.lines().next().expect("a header")).expect("JSON");
        assert_eq!(header["format"], "flagplot-game", "{}", file.display());
        assert!(header.get("inputs").is_none(), "{}", file.display());
        games += 1;
    }
    assert!(games >= 3, "the recorded games are there");
}
