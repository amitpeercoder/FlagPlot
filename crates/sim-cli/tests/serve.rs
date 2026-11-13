//! `sim-cli serve` driven through the real binary (decisions 11 to 15 of
//! `docs/decisions/0050-playing-a-side.md`).

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use serde_json::{json, Value};

#[allow(dead_code)]
#[path = "../../sim-agent/tests/support/validator.rs"]
mod validator;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
const SCENARIO: &str = "data/scenarios/05_eastern_solomons.toml";

fn temp_dir(label: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("flagplot-serve-{}-{label}", std::process::id()));
    fs::create_dir_all(&dir).expect("make the temp dir");
    dir
}

/// Runs `serve` with `input` on standard input and returns the output.
fn serve(args: &[&str], input: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_sim-cli"))
        .arg("serve")
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

fn request(id: Value, seat: &str, tool: &str, arguments: Value) -> String {
    json!({"id": id, "seat": seat, "tool": tool, "arguments": arguments}).to_string()
}

/// An order as `issue_orders` takes it: its unit and the ships of a detach by
/// the side's own numbers, 1, 2, 3... among its own in the order of the
/// scenario's numbers (decision 0086).
fn served_order(view: &sim_core::view::SideView<'_>, r: &sim_core::orders::OrderRequest) -> Value {
    let mut doc =
        json!({"unit": sim_agent::unit_text(&r.kind), "order": sim_agent::order_to_json(&r.kind)});
    let number = |mut ids: Vec<u32>, raw: u32| -> u32 {
        ids.sort();
        u32::try_from(ids.iter().position(|i| *i == raw).expect("an own unit") + 1).unwrap()
    };
    let unit = match &r.kind {
        sim_core::orders::OrderKind::TaskForce { target, .. } => format!(
            "task_force:{}",
            number(
                view.task_forces().map(|f| f.id.raw()).collect(),
                target.raw()
            )
        ),
        sim_core::orders::OrderKind::AirGroup { target, .. } => format!(
            "air_group:{}",
            number(
                view.air_groups().map(|g| g.id.raw()).collect(),
                target.raw()
            )
        ),
    };
    doc["unit"] = json!(unit);
    if let Some(ships) = doc["order"].get_mut("ships").and_then(Value::as_array_mut) {
        for ship in ships {
            let raw = u32::try_from(ship.as_u64().expect("a ship")).unwrap();
            *ship = json!(number(view.ships().map(|s| s.id.raw()).collect(), raw));
        }
    }
    doc
}

/// The Allied orders of day 1 of the scenario, as `issue_orders` takes them.
fn scripted_orders() -> Vec<Value> {
    let scenario = sim_data::Scenario::load(&Path::new(ROOT).join(SCENARIO)).expect("loads");
    let view = scenario.world.side_view(sim_core::state::Side::Allied);
    scenario
        .scripted_orders()
        .get(&1)
        .expect("orders on day 1")
        .iter()
        .filter(|(side, _)| *side == sim_core::state::Side::Allied)
        .map(|(_, r)| served_order(&view, r))
        .collect()
}

/// The lines the test client sends, and how many of them are calls of the
/// player's seat (the transcript's entries).
fn client_lines() -> (Vec<String>, usize) {
    let mut lines = vec![
        request(json!(1), "allied", "get_briefing", json!({})),
        request(json!("two"), "allied", "list_forces", json!({})),
        request(json!(3), "allied", "list_bases", json!({})),
        request(json!(4), "allied", "list_contacts", json!({})),
        request(
            json!(5),
            "allied",
            "measure",
            json!({"question": "air_distance", "from": "task_force:1", "to": "base:1"}),
        ),
        request(
            json!(6),
            "allied",
            "get_entity",
            json!({"handle": "task_force:1"}),
        ),
        request(
            json!(7),
            "allied",
            "get_entity",
            json!({"handle": "task_force:3"}),
        ),
        request(json!(8), "allied", "explain", json!({"event": 999})),
        request(
            json!(9),
            "allied",
            "issue_orders",
            json!({"orders": scripted_orders()}),
        ),
        request(json!(10), "allied", "end_day", json!({})),
        request(json!(11), "allied", "get_briefing", json!({"text": true})),
        request(json!(12), "allied", "end_day", json!({})),
        request(json!(13), "allied", "issue_orders", json!({"orders": []})),
    ];
    let calls = lines.len();
    // Not calls of a player: they are answered and change nothing.
    lines.push("this is not JSON".to_string());
    lines.push("[1, 2]".to_string());
    lines.push(request(json!(14), "japanese", "get_briefing", json!({})));
    lines.push(request(json!(15), "nobody", "get_briefing", json!({})));
    lines.push(json!({"id": 16, "seat": "allied"}).to_string());
    // A call of the player's seat that is not a tool: answered and recorded.
    lines.push(request(json!(17), "allied", "get_truth", json!({})));
    (lines, calls + 1)
}

fn answers(output: &Output) -> Vec<Value> {
    text(&output.stdout)
        .lines()
        .map(|l| serde_json::from_str(l).expect("an answer is JSON"))
        .collect()
}

fn played(label: &str) -> (Vec<String>, Output, PathBuf, PathBuf, usize) {
    let dir = temp_dir(label);
    let save = dir.join("game.json");
    let transcript = dir.join("transcript.jsonl");
    let (lines, recorded) = client_lines();
    let input = lines.join("\n") + "\n";
    let output = serve(
        &[
            SCENARIO,
            "--seat",
            "allied=player",
            "--save",
            save.to_str().unwrap(),
            "--transcript",
            transcript.to_str().unwrap(),
        ],
        &input,
    );
    assert!(output.status.success(), "{}", text(&output.stderr));
    (lines, output, save, transcript, recorded)
}

#[test]
fn a_played_game_answers_line_for_line_saves_and_replays() {
    let (lines, output, save, transcript, recorded) = played("a");
    let out = answers(&output);
    assert_eq!(out.len(), lines.len(), "one answer for each line");
    assert_eq!(out[0]["id"], 1);
    assert_eq!(out[1]["id"], "two");
    for (i, kind) in [
        "briefing",
        "list_forces",
        "list_bases",
        "list_contacts",
        "measure",
        "entity",
        "not_known",
        "not_known",
        "issue_orders",
        "end_day",
        "briefing_text",
        "end_day",
    ]
    .iter()
    .enumerate()
    {
        assert_eq!(out[i]["result"]["kind"], *kind, "answer {i}: {}", out[i]);
    }
    assert_eq!(out[9]["result"]["resolved"], true);
    assert_eq!(out[9]["result"]["game_over"], true);
    assert_eq!(out[11]["result"]["resolved"], false);
    assert_eq!(out[12]["error"]["message"], "the game is over");
    for answer in &out[13..=18] {
        assert!(answer["error"].is_object(), "{answer}");
    }
    assert_eq!(out[13]["id"], Value::Null);
    assert!(out[15]["error"]["message"]
        .as_str()
        .unwrap()
        .contains("script"));
    assert_eq!(out[15]["error"]["allowed"], json!(["allied"]));
    assert_eq!(out[16]["error"]["allowed"], json!(["allied", "japanese"]));
    assert_eq!(out[18]["error"]["allowed"].as_array().unwrap().len(), 9);

    // The saved game replays with no player.
    let replay = Command::new(env!("CARGO_BIN_EXE_sim-cli"))
        .args(["replay", save.to_str().unwrap()])
        .current_dir(ROOT)
        .output()
        .expect("replay runs");
    assert!(replay.status.success(), "{}", text(&replay.stderr));
    let day_lines: Vec<String> = text(&replay.stdout)
        .lines()
        .filter(|l| l.starts_with("day "))
        .map(str::to_string)
        .collect();
    let direct = Command::new(env!("CARGO_BIN_EXE_sim-cli"))
        .args(["hash", SCENARIO])
        .current_dir(ROOT)
        .output()
        .expect("hash runs");
    let direct: Vec<String> = text(&direct.stdout).lines().map(str::to_string).collect();
    assert_eq!(day_lines, direct, "the played game is the scripted one");
    assert!(text(&replay.stdout).contains("replayed with no player"));
    let saved: Value = serde_json::from_str(&fs::read_to_string(&save).unwrap()).unwrap();
    assert_eq!(
        saved["seats"],
        json!({"allied": "player", "japanese": "script"})
    );
    let sides: std::collections::BTreeSet<&str> = saved["orders"]
        .as_array()
        .unwrap()
        .iter()
        .map(|o| o["side"].as_str().unwrap())
        .collect();
    assert_eq!(sides.len(), 2, "both sides' orders are in the save");

    // The transcript: one entry for each call of the player's seat, in order,
    // each valid against its schema, with the answer that was given.
    let schemas = validator::Schemas::load(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("../sim-agent/schemas"),
    );
    let entries: Vec<Value> = fs::read_to_string(&transcript)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    assert_eq!(entries.len(), recorded);
    for (i, entry) in entries.iter().enumerate() {
        let errors = schemas.check("transcript_entry.schema.json", entry);
        assert!(errors.is_empty(), "{entry}: {errors:?}");
        assert_eq!(entry["seq"], i + 1);
        let kind = entry["answer"]["kind"].as_str().unwrap();
        let errors = schemas.check(&format!("{kind}.schema.json"), &entry["answer"]);
        assert!(errors.is_empty(), "{kind}: {errors:?}");
        // The last entry is the unknown tool, whose line is the last answer.
        let given = if i + 1 == entries.len() {
            &out[18]
        } else {
            &out[i]
        };
        let given = given.get("result").unwrap_or(&given["error"]);
        assert_eq!(&entry["answer"], given, "entry {i}");
    }
    assert_eq!(entries[0]["tool"], "get_briefing");
    assert_eq!(entries[9]["after_day"], 0);
    assert_eq!(entries[10]["after_day"], 1);
    assert_eq!(entries.last().unwrap()["tool"], "get_truth");
}

#[test]
fn the_same_requests_give_byte_identical_output_transcript_and_save() {
    let (_, one, save_one, transcript_one, _) = played("b1");
    let (_, two, save_two, transcript_two, _) = played("b2");
    assert_eq!(one.stdout, two.stdout);
    assert_eq!(
        fs::read(&transcript_one).unwrap(),
        fs::read(&transcript_two).unwrap()
    );
    assert_eq!(fs::read(&save_one).unwrap(), fs::read(&save_two).unwrap());
}

#[test]
fn a_save_without_seats_is_an_old_save_and_still_replays() {
    let (_, _, save, _, _) = played("c");
    let mut value: Value = serde_json::from_str(&fs::read_to_string(&save).unwrap()).unwrap();
    value.as_object_mut().unwrap().remove("seats");
    let old = save.with_file_name("old.json");
    fs::write(&old, serde_json::to_string_pretty(&value).unwrap()).unwrap();
    let replay = Command::new(env!("CARGO_BIN_EXE_sim-cli"))
        .args(["replay", old.to_str().unwrap()])
        .current_dir(ROOT)
        .output()
        .unwrap();
    assert!(replay.status.success(), "{}", text(&replay.stderr));
    assert!(!text(&replay.stdout).contains("no player"));
}

#[test]
fn serve_needs_a_player_and_a_scenario_and_rejects_bad_seats() {
    for args in [
        vec![SCENARIO],
        vec![SCENARIO, "--seat", "allied=script"],
        vec![
            SCENARIO,
            "--seat",
            "allied=player",
            "--seat",
            "allied=script",
        ],
        vec![SCENARIO, "--seat", "navy=player"],
        vec![SCENARIO, "--seat", "allied=robot"],
        vec!["--seat", "allied=player"],
    ] {
        let output = serve(&args, "");
        assert!(!output.status.success(), "{args:?}");
        assert!(text(&output.stderr).contains("error"), "{args:?}");
    }
}

#[test]
fn two_players_resolve_the_day_when_both_have_ended_it() {
    let input = [
        request(json!(1), "allied", "end_day", json!({})),
        request(json!(2), "japanese", "end_day", json!({})),
    ]
    .join("\n")
        + "\n";
    let output = serve(
        &[
            SCENARIO,
            "--seat",
            "allied=player",
            "--seat",
            "japanese=player",
        ],
        &input,
    );
    let out = answers(&output);
    assert_eq!(out[0]["result"]["resolved"], false);
    assert_eq!(out[1]["result"]["resolved"], true);
}
