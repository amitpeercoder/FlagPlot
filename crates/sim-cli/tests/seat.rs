//! A seat at a task force or a base, started from the command line (decision
//! 0138, item 5): `serve --at allied=task_force:2`, `mcp --at task_force:2`
//! and `play --at task_force:2`, kept in `play`'s game file. Over scenario 62
//! (command levels on).

use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use serde_json::{json, Value};

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
const BIN: &str = env!("CARGO_BIN_EXE_sim-cli");
const LEVELS_ON: &str = "data/scenarios/62_reports_between_commands.toml";
const LEVELS_OFF: &str = "data/scenarios/01_transit.toml";

fn temp_dir(label: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("flagplot-seat-{}-{label}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("make the temp dir");
    dir
}

fn run_with_input(args: &[&str], input: &str) -> Output {
    let mut child = Command::new(BIN)
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

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

fn lines(output: &Output) -> Vec<Value> {
    text(&output.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).expect("an answer is JSON"))
        .collect()
}

fn request(id: u32, tool: &str, arguments: Value) -> String {
    format!(
        "{}\n",
        json!({"id": id, "seat": "allied", "tool": tool, "arguments": arguments})
    )
}

#[test]
fn serve_places_a_seat_at_a_task_force_and_refuses_what_it_does_not_hold() {
    let input = [
        request(1, "list_forces", json!({})),
        request(2, "command_state", json!({})),
        request(
            3,
            "issue_orders",
            json!({"orders": [
                {"unit": "task_force:1", "order": {"kind": "set_speed", "speed": "full"}},
                {"unit": "task_force:2", "order": {"kind": "set_speed", "speed": "full"}},
            ]}),
        ),
        request(4, "get_briefing", json!({"text": true})),
    ]
    .concat();
    let output = run_with_input(
        &[
            "serve",
            LEVELS_ON,
            "--seat",
            "allied=player",
            "--at",
            "allied=task_force:2",
        ],
        &input,
    );
    assert!(output.status.success(), "{}", text(&output.stderr));
    let answers = lines(&output);
    let forces = &answers[0]["result"]["task_forces"];
    assert_eq!(forces.as_array().unwrap().len(), 1, "{forces}");
    assert_eq!(
        forces[0]["task_force"]["name"],
        "Placeholder carrier force 2"
    );
    assert_eq!(
        answers[1]["error"]["message"],
        "stating intents from a seat that is not the area command's is not built (decision 0132)"
    );
    let results = &answers[2]["result"]["results"];
    assert_eq!(results[0]["status"], "queued", "{results}");
    assert_eq!(results[1]["status"], "refused", "{results}");
    assert_eq!(results[1]["reason"], "not one of your command's units");
    let briefing = answers[3]["result"]["text"].as_str().unwrap();
    assert!(briefing.contains("none given to this seat"), "{briefing}");
}

#[test]
fn serve_places_a_seat_at_a_base_and_at_the_easier_setting() {
    let input = [
        request(1, "list_bases", json!({})),
        request(2, "list_forces", json!({})),
    ]
    .concat();
    let at_base = run_with_input(
        &[
            "serve",
            LEVELS_ON,
            "--seat",
            "allied=player",
            "--at",
            "allied=base:1",
        ],
        &input,
    );
    assert!(at_base.status.success(), "{}", text(&at_base.stderr));
    let answers = lines(&at_base);
    assert_eq!(answers[0]["result"]["bases"].as_array().unwrap().len(), 1);
    assert!(answers[1]["result"]["task_forces"]
        .as_array()
        .unwrap()
        .is_empty());
    let easier = run_with_input(
        &[
            "serve",
            LEVELS_ON,
            "--seat",
            "allied=player",
            "--at",
            "allied=area:easier",
        ],
        &input,
    );
    assert!(easier.status.success(), "{}", text(&easier.stderr));
    let answers = lines(&easier);
    assert_eq!(
        answers[1]["result"]["task_forces"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
}

#[test]
fn a_seat_that_cannot_be_placed_is_refused_before_the_game_begins() {
    // Command levels off: only the area command's seat.
    let off = run_with_input(
        &[
            "serve",
            LEVELS_OFF,
            "--seat",
            "allied=player",
            "--at",
            "allied=task_force:1",
        ],
        "",
    );
    assert!(!off.status.success());
    assert!(
        text(&off.stderr).contains(
            "command levels are off in this scenario: the only seat is the area command's"
        ),
        "{}",
        text(&off.stderr)
    );
    // A command the side does not have at the start.
    let missing = run_with_input(
        &[
            "serve",
            LEVELS_ON,
            "--seat",
            "allied=player",
            "--at",
            "allied=task_force:9",
        ],
        "",
    );
    assert!(!missing.status.success());
    assert!(text(&missing.stderr).contains("is not a task force or a base of the allied side"));
    // A seat that is not a seat, and a side held by the script.
    for at in ["allied=ship:1", "allied=task_force:x", "nobody"] {
        let output = run_with_input(
            &["serve", LEVELS_ON, "--seat", "allied=player", "--at", at],
            "",
        );
        assert!(!output.status.success(), "{at}");
    }
    let script = run_with_input(
        &[
            "serve",
            LEVELS_ON,
            "--seat",
            "allied=player",
            "--at",
            "japanese=base:1",
        ],
        "",
    );
    assert!(!script.status.success());
    assert!(text(&script.stderr).contains("held by the script"));
}

/// A scripted MCP client: one line in, one line out.
fn mcp(args: &[&str], requests: &[Value]) -> Vec<Value> {
    let mut child = Command::new(BIN)
        .arg("mcp")
        .arg(LEVELS_ON)
        .args(["--seat", "allied"])
        .args(args)
        .current_dir(ROOT)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the binary starts");
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = BufReader::new(child.stdout.take().expect("stdout"));
    let mut answers = Vec::new();
    for message in requests {
        writeln!(stdin, "{message}").expect("write");
        stdin.flush().expect("flush");
        if message.get("id").is_some() {
            let mut line = String::new();
            stdout.read_line(&mut line).expect("read");
            answers.push(serde_json::from_str(&line).expect("a response is JSON"));
        }
    }
    drop(stdin);
    let output = child.wait_with_output().expect("wait");
    assert!(output.status.success(), "{}", text(&output.stderr));
    answers
}

#[test]
fn mcp_at_a_seat_offers_the_nine_tools_and_says_where_the_seat_is() {
    let answers = mcp(
        &["--at", "task_force:2"],
        &[
            json!({"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2025-11-25", "capabilities": {}, "clientInfo": {"name": "test", "version": "0"}}}),
            json!({"jsonrpc": "2.0", "method": "notifications/initialized"}),
            json!({"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}),
            json!({"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "set_intent", "arguments": {}}}),
            json!({"jsonrpc": "2.0", "id": 4, "method": "tools/call", "params": {"name": "list_forces", "arguments": {}}}),
        ],
    );
    let instructions = answers[0]["result"]["instructions"].as_str().unwrap();
    assert!(
        instructions.contains("Your seat is at task_force:2"),
        "{instructions}"
    );
    assert!(
        instructions.contains("not one of your command's units"),
        "{instructions}"
    );
    let tools: Vec<&str> = answers[1]["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["name"].as_str().unwrap())
        .collect();
    assert_eq!(
        tools,
        [
            "get_briefing",
            "list_forces",
            "list_contacts",
            "list_bases",
            "get_entity",
            "measure",
            "explain",
            "events",
            "issue_orders",
            "end_day"
        ]
    );
    // A command tool is not a tool of this seat.
    assert!(
        answers[2]["error"]["message"]
            .as_str()
            .unwrap()
            .contains("Unknown tool"),
        "{}",
        answers[2]
    );
    let forces = answers[3]["result"]["content"][0]["text"].as_str().unwrap();
    assert!(forces.contains("Placeholder carrier force 2"), "{forces}");
    assert!(!forces.contains("carrier force 1"), "{forces}");
}

#[test]
fn mcp_at_the_area_seat_still_offers_the_staff_tools() {
    let answers = mcp(
        &[],
        &[
            json!({"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2025-11-25", "capabilities": {}, "clientInfo": {"name": "test", "version": "0"}}}),
            json!({"jsonrpc": "2.0", "method": "notifications/initialized"}),
            json!({"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}),
        ],
    );
    assert_eq!(answers[1]["result"]["tools"].as_array().unwrap().len(), 28);
}

fn play(game: &Path, args: &[&str]) -> Output {
    let mut all = vec!["play", "--game", game.to_str().expect("a path is text")];
    all.extend_from_slice(args);
    run_with_input(&all, "")
}

#[test]
fn play_keeps_the_seat_in_its_game_file_and_replays_it() {
    let dir = temp_dir("play");
    let game = dir.join("game.txt");
    let first = play(
        &game,
        &[
            "--scenario",
            LEVELS_ON,
            "--seat",
            "allied",
            "--at",
            "task_force:2",
            "list_forces",
        ],
    );
    assert!(first.status.success(), "{}", text(&first.stderr));
    let header = fs::read_to_string(&game).expect("the game file");
    let first_line = header.lines().next().unwrap();
    assert!(
        first_line.contains("\"at\":\"task_force:2\""),
        "{first_line}"
    );
    // A later call needs no seat: the file has it. The answer is the seat's.
    let again = play(&game, &["list_forces"]);
    assert!(again.status.success(), "{}", text(&again.stderr));
    assert_eq!(first.stdout, again.stdout);
    assert!(text(&again.stdout).contains("Placeholder carrier force 2"));
    assert!(!text(&again.stdout).contains("carrier force 1"));
    // Another seat is not this game's.
    let other = play(&game, &["--at", "base:1", "list_forces"]);
    assert!(!other.status.success());
    assert!(text(&other.stderr).contains("is not the game's seat"));
    // A day is played and the game replays with its transcript.
    let end = play(&game, &["end_day"]);
    assert!(end.status.success(), "{}", text(&end.stderr));
    let transcript = dir.join("transcript.jsonl");
    let exported = play(
        &game,
        &["--export-transcript", transcript.to_str().unwrap()],
    );
    assert!(exported.status.success(), "{}", text(&exported.stderr));
    let transcript = fs::read_to_string(&transcript).expect("the transcript");
    assert_eq!(transcript.lines().count(), 3, "{transcript}");
    // A game at the area command keeps no seat in its header.
    let area = dir.join("area.txt");
    let made = play(
        &area,
        &["--scenario", LEVELS_ON, "--seat", "allied", "list_forces"],
    );
    assert!(made.status.success(), "{}", text(&made.stderr));
    let header = fs::read_to_string(&area).expect("the game file");
    assert!(!header.lines().next().unwrap().contains("\"at\""));
    let _ = fs::remove_dir_all(&dir);
}
