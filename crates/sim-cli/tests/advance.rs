//! The tools of decision 0156, step 3, through the real binary: the tool
//! list of the MCP server with the key on and off, and a game played with an
//! order at 0600 through `play`, exported with `--export-save` and replayed
//! by `replay` to the same daily hashes, which are scenario 78's.

use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

use serde_json::{json, Value};

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
const BIN: &str = env!("CARGO_BIN_EXE_sim-cli");
const KEY_ON: &str = "data/scenarios/78_orders_inside_a_day.toml";
const KEY_OFF: &str = "data/scenarios/01_transit.toml";

fn temp_dir(label: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("flagplot-advance-{}-{label}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("make the temp dir");
    dir
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

fn binary(args: &[&str]) -> Output {
    Command::new(BIN)
        .args(args)
        .current_dir(ROOT)
        .output()
        .expect("the binary runs")
}

/// A scripted MCP client on `scenario` at the Allied seat.
fn mcp(scenario: &str, requests: &[Value]) -> Vec<Value> {
    let mut child = Command::new(BIN)
        .args(["mcp", scenario, "--seat", "allied"])
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

fn opening() -> Vec<Value> {
    vec![
        json!({"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2025-11-25", "capabilities": {}, "clientInfo": {"name": "test", "version": "0"}}}),
        json!({"jsonrpc": "2.0", "method": "notifications/initialized"}),
        json!({"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}),
    ]
}

fn names(answer: &Value) -> Vec<String> {
    answer["result"]["tools"]
        .as_array()
        .expect("tools")
        .iter()
        .map(|t| t["name"].as_str().expect("a name").to_string())
        .collect()
}

#[test]
fn the_tool_lists_with_the_key_on_and_off() {
    let off = mcp(KEY_OFF, &opening());
    let off_names = names(&off[1]);
    assert_eq!(off_names.len(), 31);
    assert!(off_names.contains(&"events".to_string()));
    assert!(!off_names.contains(&"advance".to_string()));
    let instructions = off[0]["result"]["instructions"].as_str().unwrap();
    assert!(!instructions.contains("Stopping between pulses"));

    let on = mcp(KEY_ON, &opening());
    let on_names = names(&on[1]);
    assert_eq!(on_names.len(), 32);
    assert_eq!(on_names.last().map(String::as_str), Some("advance"));
    assert_eq!(&on_names[..31], off_names.as_slice());
    let instructions = on[0]["result"]["instructions"].as_str().unwrap();
    assert!(instructions.contains("Stopping between pulses"));
    let advance = &on[1]["result"]["tools"][31];
    let description = advance["description"].as_str().unwrap();
    for kind in [
        "report_received",
        "air_attack_against",
        "base_attacked_against",
        "shore_bombardment_against",
        "salvo_against",
        "engagement_begun",
        "ship_sunk",
        "system_hit",
        "strike_launched",
        "base_strike_launched",
        "men_landed",
        "ground_fight",
    ] {
        assert!(description.contains(kind), "the description lacks {kind}");
    }
    assert_eq!(advance["inputSchema"]["required"], json!(["pulses"]));

    // play --tools needs no game: the tools of a seat with a staff and the
    // key off.
    let listed = text(&binary(&["play", "--game", "unused", "--tools"]).stdout);
    assert_eq!(listed.matches("\nTOOL ").count() + 1, 31);
    assert!(listed.contains("TOOL events\n"));

    // With a scenario and a seat it lists that game's seat's own tools: the
    // key on offers `advance`, a scenario that names a trial's figures offers
    // `trial` and `trials`, and neither is in the list that needs no game.
    let of = |scenario: &str| {
        text(
            &binary(&[
                "play",
                "--game",
                "unused",
                "--scenario",
                scenario,
                "--seat",
                "allied",
                "--tools",
            ])
            .stdout,
        )
    };
    assert!(!listed.contains("TOOL advance\n") && !listed.contains("TOOL trial\n"));
    assert!(of(KEY_ON).contains("TOOL advance\n"));
    let tried = of("data/scenarios/100_what_ifs_from_a_seat.toml");
    assert!(tried.contains("TOOL trial\n") && tried.contains("TOOL trials\n"));
    assert_eq!(of(KEY_OFF), listed);
    // A game file that exists is asked for its own seat's tools, with no
    // scenario named again.
    let dir = temp_dir("tools-of-a-game");
    let game = dir.join("g.txt");
    let game = game.to_str().unwrap();
    ok(
        &play(
            game,
            &[
                "--scenario",
                "data/scenarios/100_what_ifs_from_a_seat.toml",
                "--seat",
                "allied",
                "command_state",
            ],
        ),
        "the first call",
    );
    assert_eq!(text(&play(game, &["--tools"]).stdout), tried);
}

fn play(game: &str, args: &[&str]) -> Output {
    let mut all = vec!["play", "--game", game];
    all.extend_from_slice(args);
    binary(&all)
}

fn ok(output: &Output, what: &str) -> String {
    assert!(
        output.status.success(),
        "{what}: {}{}",
        text(&output.stdout),
        text(&output.stderr)
    );
    text(&output.stdout)
}

fn hash_lines(output: &Output) -> Vec<String> {
    ok(output, "hash")
        .lines()
        .filter(|l| l.starts_with("day "))
        .map(str::to_string)
        .collect()
}

/// Scenario 1 with the key on (scenario 78 with its order inside the day
/// left out) played by `play` at the Allied seat: the dusk orders, one
/// pulse, the 8-knot order at 0600, and on to the end. Its saved game
/// replays to its own hashes, which are scenario 78's run by `sim-cli`.
#[test]
fn a_game_with_an_order_at_0600_exported_replays_to_the_same_hashes() {
    let dir = temp_dir("export");
    // Scenario 1's setup with the key on and no order inside a day: the
    // Allied seat is the player's, so the file's orders, of dusk and of 0600,
    // are not handed in; the player gives them.
    let game = dir.join("game.txt");
    let game = game.to_str().unwrap();
    let first = play(
        game,
        &[
            "--scenario",
            KEY_ON,
            "--seat",
            "allied",
            "issue_orders",
            r#"{"orders": [{"unit": "task_force:1", "order": {"kind": "set_route", "waypoints": [[8, 3]]}}, {"unit": "task_force:1", "order": {"kind": "set_speed", "speed": "cruise"}}]}"#,
        ],
    );
    ok(&first, "the dusk orders");
    let advanced: Value = serde_json::from_str(&ok(
        &play(game, &["advance", r#"{"pulses": 1}"#]),
        "advance",
    ))
    .expect("JSON");
    assert_eq!(advanced["clock"]["at"], "0600");
    let given: Value = serde_json::from_str(&ok(
        &play(
            game,
            &[
                "issue_orders",
                r#"{"orders": [{"unit": "task_force:1", "order": {"kind": "set_speed", "knots": "8"}}]}"#,
            ],
        ),
        "the order at 0600",
    ))
    .expect("JSON");
    assert_eq!(given["for_pulse"], "morning");
    let to_end: Value = serde_json::from_str(&ok(
        &play(game, &["advance", r#"{"pulses": 20}"#]),
        "advance to the end",
    ))
    .expect("JSON");
    assert_eq!(to_end["stopped"], "game_over");

    let save = dir.join("save.json");
    let save = save.to_str().unwrap();
    ok(
        &play(
            game,
            &[
                "--export-save",
                save,
                "--export-transcript",
                &format!("{save}.t"),
            ],
        ),
        "export",
    );
    let saved: Value = serde_json::from_str(&fs::read_to_string(save).unwrap()).unwrap();
    assert_eq!(saved["version"], 23);
    let orders = saved["orders"].as_array().unwrap();
    assert_eq!(orders.len(), 3);
    assert!(orders[..2].iter().all(|o| o.get("pulse").is_none()));
    assert_eq!(orders[2]["pulse"], "morning");
    assert_eq!(orders[2]["day"], 1);

    let replayed = binary(&["replay", save]);
    let replayed_lines: Vec<String> = ok(&replayed, "replay")
        .lines()
        .filter(|l| l.starts_with("day "))
        .map(str::to_string)
        .collect();
    let file = hash_lines(&binary(&["hash", KEY_ON]));
    assert_eq!(file.len(), 3);
    assert_eq!(replayed_lines, file);
    let saved_hashes: Vec<String> = saved["hashes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|h| format!("day {} {}", h["day"], h["hash"].as_str().unwrap()))
        .collect();
    assert_eq!(saved_hashes, file);
}
