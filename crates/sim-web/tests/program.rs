//! The native program `door` holds a `Door` open for a game and answers
//! requests read as lines from standard input (decision 0229, ruling 3). It
//! is what the browser's module is compared with in stage 3, so what it
//! says is the door's own string, written once by `serde_json`.
//!
//! No expected value is written here: the program's answer to each request
//! is compared with what a `Door` in this process gives for the same call.

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

use serde_json::{json, Value};
use sim_cli::pack;
use sim_data::LoadOptions;
use sim_web::Door;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

#[test]
fn the_program_door_answers_as_the_library_door_does() {
    let scenario = format!("{ROOT}/data/scenarios/second-proof/05c_eastern_solomons.toml");
    let pack = pack::pack_of(&[scenario.as_str()], &LoadOptions::default()).expect("a pack");
    let text = pack::pack_text(&pack).expect("its text");
    let dir = std::env::temp_dir().join(format!("flagplot-door-program-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a folder");
    let pack_file = dir.join("pack.json");
    std::fs::write(&pack_file, &text).expect("the pack is written");
    let pack_path = pack_file.to_str().expect("a path");

    let mut child = Command::new(env!("CARGO_BIN_EXE_door"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("the program starts");
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = BufReader::new(child.stdout.take().expect("stdout"));
    let mut ask = |function: &str, args: Vec<&str>| -> Value {
        writeln!(stdin, "{}", json!({"function": function, "args": args})).expect("written");
        let mut line = String::new();
        stdout.read_line(&mut line).expect("an answer line");
        serde_json::from_str(&line).expect("the answer is JSON")
    };

    let mut door = Door::new();
    let request = json!({"scenario": scenario, "seat": "allied"}).to_string();
    assert_eq!(ask("versions", vec![]), json!({"ok": door.versions()}));
    assert_eq!(ask("begin", vec![pack_path, &request]), json!({"ok": ""}));
    door.begin(text.as_bytes(), &request).expect("begun");
    assert!(ask("begin", vec![pack_path, &request])["error"].is_string());
    assert_eq!(ask("tools", vec![]), json!({"ok": door.tools().unwrap()}));
    for (tool, arguments) in [
        ("get_briefing", "{}"),
        ("list_forces", "{}"),
        (
            "issue_orders",
            r#"{"orders": [{"unit": "air_group:1", "order": {"kind": "set_air_mission", "mission": "combat_air_patrol"}}]}"#,
        ),
        ("no_such_tool", "{}"),
        ("end_day", "{}"),
    ] {
        assert_eq!(
            ask("call", vec![tool, arguments]),
            json!({"ok": door.call(tool, arguments).unwrap()}),
            "{tool}"
        );
    }
    let file = door.game_file().expect("the game file");
    assert_eq!(ask("game_file", vec![]), json!({"ok": file}));
    assert_eq!(
        ask("saved_game", vec![]),
        json!({"ok": door.saved_game().unwrap()})
    );
    assert!(ask("nonsense", vec![])["error"].is_string());
    drop(stdin);
    child.wait().expect("the program ends with its input");
}
