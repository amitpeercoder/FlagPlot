//! `sim-cli serve --watched` driven through the real binary: statement 20 of
//! scenario 101 (`tests/scenario_101_a_game_watched.rs`, decision 0182). A
//! held call's answer line comes after the line of the step that made it
//! due, with the call's own `id`; two due on one step come Allied first; a
//! seat's tool definitions and opening words are answered; a request that is
//! none of a seat's call, a step or a request of the watcher's is answered
//! with an error and changes nothing; and without `--watched` a step and a
//! watcher's request are refused.

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

use serde_json::{json, Value};

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
const SCENARIO: &str = "data/scenarios/orders-inside-proof/05_eastern_solomons_on.toml";

fn temp_dir(label: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "flagplot-serve-watched-{}-{label}",
        std::process::id()
    ));
    fs::create_dir_all(&dir).expect("make the temp dir");
    dir
}

fn run(command: &str, args: &[&str], input: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_sim-cli"))
        .arg(command)
        .args(args)
        .current_dir(ROOT)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the binary starts");
    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(input.as_bytes())
        .expect("input is written");
    child.wait_with_output().expect("the binary finishes")
}

fn answers(output: &Output) -> Vec<Value> {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|l| serde_json::from_str(l).expect("an answer is JSON"))
        .collect()
}

fn call(id: u32, seat: &str, tool: &str, arguments: Value) -> String {
    json!({"id": id, "seat": seat, "tool": tool, "arguments": arguments}).to_string()
}

#[test]
fn a_watched_game_is_served_line_by_line() {
    let dir = temp_dir("game");
    let (save, transcript) = (dir.join("game.json"), dir.join("transcript.jsonl"));
    let lines = [
        json!({"id": 1, "seat": "allied", "request": "tools"}).to_string(),
        json!({"id": 2, "seat": "japanese", "request": "instructions"}).to_string(),
        call(3, "allied", "advance", json!({"pulses": 1})),
        call(4, "japanese", "advance", json!({"pulses": 1})),
        json!({"id": 5, "watcher": "standing"}).to_string(),
        json!({"id": 6, "watcher": "read", "side": "japanese", "tool": "list_forces", "arguments": {}}).to_string(),
        // None of a seat's call, a step or a request of the watcher's.
        json!({"id": 7, "watcher": "dance"}).to_string(),
        json!({"id": 8, "nonsense": true}).to_string(),
        json!({"id": 9, "watcher": "read", "side": "allied", "tool": "issue_orders", "arguments": {"orders": []}}).to_string(),
        json!({"id": 10, "seat": "allied", "request": "everything"}).to_string(),
        "not JSON".to_string(),
        json!({"id": 11, "watcher": "step"}).to_string(),
        json!({"id": 12, "watcher": "seat_calls", "side": "allied"}).to_string(),
        call(13, "allied", "end_day", json!({})),
        call(14, "japanese", "run", json!({"days": 1})),
        json!({"id": 15, "watcher": "step"}).to_string(),
        json!({"id": 16, "watcher": "step"}).to_string(),
        json!({"id": 17, "watcher": "step"}).to_string(),
    ];
    let output = run(
        "serve",
        &[
            SCENARIO,
            "--seat",
            "allied=player",
            "--seat",
            "japanese=player",
            "--watched",
            "--save",
            save.to_str().unwrap(),
            "--transcript",
            transcript.to_str().unwrap(),
        ],
        &(lines.join("\n") + "\n"),
    );
    let out = answers(&output);
    let ids: Vec<Value> = out.iter().map(|a| a["id"].clone()).collect();
    assert_eq!(
        ids,
        vec![
            json!(1),
            json!(2),
            json!(5),
            json!(6),
            json!(7),
            json!(8),
            json!(9),
            json!(10),
            Value::Null,
            json!(11),
            json!(3),
            json!(4),
            json!(12),
            json!(15),
            json!(16),
            json!(13),
            json!(14),
            json!(17),
        ]
    );
    // The seat's tool definitions and opening words.
    let tools: Vec<&str> = out[0]["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["name"].as_str().unwrap())
        .collect();
    assert!(
        tools.contains(&"run") && tools.contains(&"advance"),
        "{tools:?}"
    );
    assert!(out[0]["result"]["tools"][0]["inputSchema"].is_object());
    let words = out[1]["result"]["instructions"].as_str().unwrap();
    assert!(words.starts_with("Flag Plot is an operational wargame"));
    assert!(words.contains("Stopping between pulses"));
    // The standing and the watcher's read.
    assert_eq!(out[2]["result"]["kind"], "standing");
    assert_eq!(out[2]["result"]["waiting_for"], json!([]));
    assert_eq!(out[3]["result"]["kind"], "list_forces");
    // Errors that change nothing.
    for answer in &out[4..9] {
        assert_eq!(answer["error"]["kind"], "error", "{answer}");
    }
    // The step, then the held calls it made due, the Allied first.
    assert_eq!(out[9]["result"]["kind"], "step");
    assert_eq!(out[9]["result"]["stops"], json!(["allied", "japanese"]));
    assert_eq!(out[10]["result"]["kind"], "advance");
    assert_eq!(out[11]["result"]["kind"], "advance");
    assert_eq!(out[12]["result"]["kind"], "seat_calls");
    assert_eq!(out[12]["result"]["calls"], json!([]));
    // The last step ends the game and answers both.
    assert_eq!(out[17 - 2]["result"]["kind"], "end_day");
    assert_eq!(out[17 - 1]["result"]["kind"], "run");
    assert_eq!(
        out[17]["error"]["message"],
        "the game is over: there is no step to take"
    );
    let step = &out[14]["result"];
    assert_eq!(step["game_over"], true);

    // Nothing but the seats' calls is in the transcript.
    let entries: Vec<Value> = fs::read_to_string(&transcript)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    let tools: Vec<&str> = entries
        .iter()
        .map(|e| e["tool"].as_str().unwrap())
        .collect();
    assert_eq!(tools, vec!["advance", "advance", "end_day", "run"]);
    // The saved game replays with no watcher.
    let replay = run("replay", &[save.to_str().unwrap()], "");
    assert!(
        replay.status.success(),
        "{}",
        String::from_utf8_lossy(&replay.stderr)
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn without_watched_a_step_and_a_watchers_request_are_refused() {
    let lines = [
        json!({"id": 1, "watcher": "step"}).to_string(),
        json!({"id": 2, "watcher": "standing"}).to_string(),
        json!({"id": 3, "watcher": "read", "side": "allied", "tool": "list_forces"}).to_string(),
        json!({"id": 4, "seat": "allied", "request": "tools"}).to_string(),
        call(5, "allied", "advance", json!({"pulses": 1})),
    ];
    let dir = temp_dir("plain");
    let transcript = dir.join("transcript.jsonl");
    let output = run(
        "serve",
        &[
            SCENARIO,
            "--seat",
            "allied=player",
            "--transcript",
            transcript.to_str().unwrap(),
        ],
        &(lines.join("\n") + "\n"),
    );
    let out = answers(&output);
    assert_eq!(out.len(), 5);
    for answer in &out[..3] {
        assert_eq!(
            answer["error"]["message"], "\"seat\" must be one of the seats",
            "{answer}"
        );
    }
    assert_eq!(out[3]["error"]["message"], "\"tool\" must name a tool");
    // A call of time is answered at once, as before.
    assert_eq!(out[4]["id"], 5);
    assert_eq!(out[4]["result"]["kind"], "advance");
    let entries = fs::read_to_string(&transcript).unwrap();
    assert_eq!(entries.lines().count(), 1);
    let _ = fs::remove_dir_all(&dir);
}
