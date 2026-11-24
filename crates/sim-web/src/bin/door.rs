//! `door`: a [`Door`] held open for a whole game and answered from standard
//! input, one request a line (decision 0229, ruling 3). It is what the
//! browser's module is compared with: the same `Door`, built natively.
//!
//! A request is a JSON object `{"function": NAME, "args": [..]}` with the
//! arguments as strings: `versions` and `tools` and `game_file` and
//! `saved_game` take none; `begin` takes the path of a pack file and the
//! request of the door's `begin`; `call` takes the tool's name and its
//! arguments as JSON text; `resume` takes the path of a pack file and the
//! text of a game file. The answer is one line, `{"ok": TEXT}` with the
//! door's own string, or `{"error": WORDS}`, both written by `serde_json`.

use std::io::{BufRead, Write};

use serde_json::{json, Value};
use sim_web::Door;

fn arguments(request: &Value, count: usize) -> Result<Vec<String>, String> {
    let list: Vec<String> = request
        .get("args")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();
    if list.len() != count {
        return Err(format!("the function takes {count} arguments, as text"));
    }
    Ok(list)
}

fn pack_bytes(path: &str) -> Result<Vec<u8>, String> {
    std::fs::read(path).map_err(|e| format!("{path}: {e}"))
}

fn answer(door: &mut Door, line: &str) -> Result<String, String> {
    let request: Value =
        serde_json::from_str(line).map_err(|e| format!("the request is not JSON: {e}"))?;
    let function = request
        .get("function")
        .and_then(Value::as_str)
        .ok_or_else(|| "the request needs \"function\"".to_string())?;
    match function {
        "versions" => {
            arguments(&request, 0)?;
            Ok(door.versions())
        }
        "begin" => {
            let args = arguments(&request, 2)?;
            door.begin(&pack_bytes(&args[0])?, &args[1])?;
            Ok(String::new())
        }
        "tools" => {
            arguments(&request, 0)?;
            door.tools()
        }
        "call" => {
            let args = arguments(&request, 2)?;
            door.call(&args[0], &args[1])
        }
        "game_file" => {
            arguments(&request, 0)?;
            door.game_file()
        }
        "resume" => {
            let args = arguments(&request, 2)?;
            door.resume(&pack_bytes(&args[0])?, &args[1])?;
            Ok(String::new())
        }
        "saved_game" => {
            arguments(&request, 0)?;
            door.saved_game()
        }
        other => Err(format!("no function {other}")),
    }
}

fn main() {
    let mut door = Door::new();
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    for line in stdin.lock().lines() {
        let Ok(line) = line else { break };
        if line.trim().is_empty() {
            continue;
        }
        let reply = match answer(&mut door, &line) {
            Ok(text) => json!({"ok": text}),
            Err(words) => json!({"error": words}),
        };
        if writeln!(out, "{reply}").and_then(|()| out.flush()).is_err() {
            break;
        }
    }
}
