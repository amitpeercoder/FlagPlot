//! The door of one seat (decision 0229, stage 2), tested against the
//! programs it stands beside: the MCP server a seat is answered by, and
//! `sim-cli play`, which keeps a game as a file of calls.
//!
//! # What the tests are to show
//!
//! Written by the architect in decision 0229, stage 2, statements 4, 5, 5a
//! and 5b, and copied here word for word. No test has a hand calculation:
//! every expected value is what the native program gives for the same
//! calls.
//!
//! 4. For a game of the Eastern Solomons and a game of Midway, every call
//!    made through the door is answered as the same call through `sim-cli
//!    mcp` is answered once put through the bridge's mapping: the same `ok`
//!    value, or the same words of refusal.
//!
//! 5. A game file the door gives is one `sim-cli play`'s replay continues,
//!    and a game file `sim-cli play` wrote, reads and all, is one the door
//!    resumes, each to the same answers as the game played in one piece;
//!    and the door's file, which leaves the reads out, replays to the same
//!    answers and the same saved game as the file with them in.
//!
//! 5a. `saved_game` is refused before the last day is resolved and, after
//!    it, gives byte for byte what `sim-cli play --export-save` writes for
//!    the same game file.
//!
//! 5b. A second `begin` on a door that has a game is refused; no function
//!    of the door takes a side but `begin` and `resume`.
//!
//! # The test owed on pull request 732 (B)
//!
//! Written by the architect in decision 0229, "The architect's rulings on
//! pull request 732", copied word for word:
//!
//! B. Statement 5's last clause is not yet held as written. The statement
//! says a file with its reads left out "replays to the same answers".
//! `the_doors_file_without_reads_replays_as_the_file_with_them` compares the
//! two files' saved games, their game files and one briefing, all after the
//! game is over, on the one day of the Eastern Solomons. That shows the two
//! end alike; it does not show that a call made after a read, in the middle
//! of a game, is answered alike, which is what a kept game taken up in the
//! browser rests on. The test owed, on Midway (five days), the Allied seat:
//! play a file with its reads in, and the same calls with the reads left
//! out, each to the evening of day 3, by `resume` in two doors; then make
//! the same later calls on both and compare every answer, value for value.
//! The later calls must include one whose answer carries a number the game
//! gives out (`queued` after an order is signed, and `take_back` of an entry
//! by its number; a `preview` and its approval where the game offers one)
//! and then `end_day` to the last. The saved games at the end are compared
//! as now. If any answer differs, the developer stops: ruling 33's choice to
//! leave reads out would then be wrong, and it comes back to the architect.
//!
//! # How they are made
//!
//! The games are the table's: the Eastern Solomons over one day, the Allied
//! seat, and Midway over five, the Allied seat. Every file is named by an
//! absolute path, both in the pack the door is given and on the command
//! line of `play`, so that the two pin the same strings. The bridge's
//! mapping (`Game.call` in `table/serve.py`) is written out in
//! [`bridge`]: an error of the protocol or a result marked an error is
//! `{"refused": words}`; text that is JSON is `{"ok": that}`; any other
//! text is `{"ok": {"text": text}}`. The server compared with is the
//! library's own, the one `sim-cli mcp` puts its standard input through.
//!
//! Statement 5b's second half, that no function of the door takes a side
//! but `begin` and `resume`, is held in `sim-web`'s `tests/source.rs`,
//! which reads the signatures of the door.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::{Command, Output};

use serde_json::{json, Value};
use sim_agent::SeatKind;
use sim_cli::game;
use sim_cli::mcp::Server;
use sim_cli::pack;
use sim_core::state::Side;
use sim_data::LoadOptions;
use sim_web::Door;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
const BIN: &str = env!("CARGO_BIN_EXE_sim-cli");

const EASTERN_SOLOMONS: &str = "data/scenarios/second-proof/05c_eastern_solomons.toml";
const MIDWAY: &str = "data/scenarios/table-boats/01b_midway_submarines.toml";

fn scenario(path: &str) -> String {
    format!("{ROOT}/{path}")
}

fn pack_bytes(scenario: &str) -> Vec<u8> {
    let pack = pack::pack_of(&[scenario], &LoadOptions::default()).expect("the pack is made");
    pack::pack_text(&pack)
        .expect("the pack is text")
        .into_bytes()
}

fn begun(scenario: &str, pack: &[u8]) -> Door {
    let mut door = Door::new();
    door.begin(
        pack,
        &json!({"scenario": scenario, "seat": "allied", "seed": null}).to_string(),
    )
    .expect("the door begins");
    door
}

fn through(door: &mut Door, call: &(&str, Value)) -> Value {
    let text = door
        .call(call.0, &call.1.to_string())
        .expect("the door answers");
    serde_json::from_str(&text).expect("the door's answer is JSON")
}

/// The bridge's `Game.call`, on the reply of the server to a `tools/call`.
fn bridge(reply: &Value) -> Value {
    if let Some(error) = reply.get("error") {
        return json!({"refused": error["message"].as_str().unwrap_or("refused")});
    }
    let result = &reply["result"];
    let text: String = result["content"]
        .as_array()
        .map(|parts| {
            parts
                .iter()
                .filter_map(|p| p["text"].as_str())
                .collect::<String>()
        })
        .unwrap_or_default();
    if result["isError"] == json!(true) {
        return json!({"refused": text});
    }
    match serde_json::from_str::<Value>(&text) {
        Ok(value) => json!({"ok": value}),
        Err(_) => json!({"ok": {"text": text}}),
    }
}

/// The seat's server, opened as `sim-cli mcp` opens it, and a client that
/// has said `initialize`.
struct Mcp {
    server: Server,
    next: u64,
}

impl Mcp {
    fn open(scenario: &str) -> Mcp {
        let seats = BTreeMap::from([(Side::Allied, SeatKind::Player)]);
        let (session, _) =
            game::open(scenario, None, seats, &BTreeMap::new()).expect("the game opens");
        let mut mcp = Mcp {
            server: Server::new(session, Side::Allied),
            next: 0,
        };
        let init = mcp.send(
            "initialize",
            json!({"protocolVersion": "2025-11-25", "capabilities": {}, "clientInfo": {"name": "test", "version": "1"}}),
        );
        assert!(init.get("result").is_some(), "initialize: {init}");
        mcp
    }

    fn send(&mut self, method: &str, params: Value) -> Value {
        self.next += 1;
        let line = json!({"jsonrpc": "2.0", "id": self.next, "method": method, "params": params});
        let answer = self
            .server
            .handle_line(&line.to_string())
            .expect("a request is answered");
        serde_json::from_str(&answer).expect("an answer is JSON")
    }

    fn call(&mut self, call: &(&str, Value)) -> Value {
        bridge(&self.send("tools/call", json!({"name": call.0, "arguments": call.1})))
    }
}

/// The calls of the Eastern Solomons game, the Allied seat: reads, orders
/// that are queued and refused, a call the protocol refuses, a call whose
/// arguments are not an object, and the day ended to the last.
fn eastern_solomons_calls() -> Vec<(&'static str, Value)> {
    vec![
        ("get_briefing", json!({})),
        ("get_briefing", json!({"text": false})),
        ("list_forces", json!({})),
        ("list_bases", json!({})),
        ("list_contacts", json!({})),
        ("command_state", json!({})),
        ("queued", json!({})),
        (
            "issue_orders",
            json!({"orders": [
                {"unit": "task_force:1", "order": {"kind": "set_route", "waypoints": [[300, 300]]}},
                {"unit": "air_group:1", "order": {"kind": "set_air_mission", "mission": "combat_air_patrol"}},
            ], "reason": "a test of the door"}),
        ),
        (
            "issue_orders",
            json!({"orders": [{"unit": "task_force:77", "order": {"kind": "set_speed", "speed": "full"}}]}),
        ),
        ("no_such_tool", json!({})),
        (
            "measure",
            json!({"question": "air_distance", "from": "task_force:1", "to": "base:1"}),
        ),
        ("queued", json!({})),
        ("events", json!({"day": 1})),
        ("list_forces", json!([1])),
        ("end_day", json!({})),
        ("get_briefing", json!({})),
        ("list_forces", json!({})),
        ("end_day", json!({})),
    ]
}

/// The calls of the Midway game, the Allied seat, five days.
fn midway_calls() -> Vec<(&'static str, Value)> {
    let mut calls = vec![
        ("get_briefing", json!({})),
        ("list_forces", json!({})),
        ("list_bases", json!({})),
        ("list_contacts", json!({})),
        ("command_state", json!({})),
        ("queued", json!({})),
        (
            "issue_orders",
            json!({"orders": [
                {"unit": "task_force:1", "order": {"kind": "set_speed", "speed": "full"}},
                {"unit": "submarine:1", "order": {"kind": "station", "hex": [7, 0]}},
            ]}),
        ),
        ("queued", json!({})),
        ("hold", json!({"task_force": 1})),
        ("no_such_tool", json!({})),
        ("intelligence", json!({})),
    ];
    for day in 1..=5 {
        calls.push(("end_day", json!({})));
        if day < 5 {
            calls.push(("get_briefing", json!({})));
            calls.push(("list_forces", json!({})));
            calls.push(("events", json!({"day": day})));
        }
    }
    calls.push(("get_briefing", json!({})));
    calls
}

// ---------------------------------------------------------------------
// Statement 4.
// ---------------------------------------------------------------------

fn door_answers_as_mcp_does(scenario: &str, calls: &[(&'static str, Value)]) {
    let pack = pack_bytes(scenario);
    let mut door = begun(scenario, &pack);
    let mut mcp = Mcp::open(scenario);
    let listed: Value =
        serde_json::from_str(&door.tools().expect("the tools")).expect("the tools are JSON");
    let own = mcp.send("tools/list", json!({}));
    assert_eq!(listed["tools"], own["result"]["tools"], "the tools");
    for (i, call) in calls.iter().enumerate() {
        let from_door = through(&mut door, call);
        let from_mcp = mcp.call(call);
        assert_eq!(
            from_door,
            from_mcp,
            "call {} ({}) is answered as mcp answers it",
            i + 1,
            call.0
        );
    }
}

#[test]
fn the_door_answers_the_eastern_solomons_as_mcp_does() {
    door_answers_as_mcp_does(&scenario(EASTERN_SOLOMONS), &eastern_solomons_calls());
}

#[test]
fn the_door_answers_midway_as_mcp_does() {
    door_answers_as_mcp_does(&scenario(MIDWAY), &midway_calls());
}

// ---------------------------------------------------------------------
// `play`.
// ---------------------------------------------------------------------

/// A new, empty folder for each call: tests run side by side, and none may
/// write where another reads.
fn temp_dir(label: &str) -> PathBuf {
    static COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let n = COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let dir =
        std::env::temp_dir().join(format!("flagplot-door-{}-{n}-{label}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("make the temp dir");
    dir
}

fn play(args: &[&str]) -> Output {
    Command::new(BIN)
        .arg("play")
        .args(args)
        .current_dir(ROOT)
        .output()
        .expect("the binary runs")
}

/// What `play` printed for a call, as the bridge would give it.
fn play_answer(output: &Output) -> Value {
    assert!(
        output.status.success(),
        "play: {}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8_lossy(&output.stdout);
    let text = text.strip_suffix('\n').unwrap_or(&text);
    match serde_json::from_str::<Value>(text) {
        Ok(value) => json!({"ok": value}),
        Err(_) => json!({"ok": {"text": text}}),
    }
}

/// One call through `play` on the game file `file`; the first call of a new
/// file names the scenario and the seat.
fn play_call(file: &str, scenario: Option<&str>, call: &(&str, Value)) -> Output {
    let arguments = call.1.to_string();
    let mut args = vec!["--game", file];
    if let Some(scenario) = scenario {
        args.extend(["--scenario", scenario, "--seat", "allied"]);
    }
    args.extend([call.0, &arguments]);
    play(&args)
}

fn export_save(file: &str) -> String {
    let dir = temp_dir("export");
    let save = dir.join(format!(
        "{}.save.json",
        std::path::Path::new(file)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("game")
    ));
    let out = play(&[
        "--game",
        file,
        "--export-save",
        save.to_str().expect("a path"),
    ]);
    assert!(
        out.status.success(),
        "export: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    std::fs::read_to_string(&save).expect("the saved game is written")
}

/// The first calls of the Eastern Solomons game, up to the evening: all the
/// reads and orders before the day is ended.
fn before_the_day_ends(calls: &[(&'static str, Value)]) -> usize {
    calls
        .iter()
        .position(|c| c.0 == "end_day")
        .expect("the calls end a day")
}

/// A game played to a point through the door: its game file, with the reads
/// left out.
fn door_played(scenario: &str, pack: &[u8], calls: &[(&'static str, Value)]) -> (Door, String) {
    let mut door = begun(scenario, pack);
    for call in calls {
        through(&mut door, call);
    }
    let file = door.game_file().expect("the game file");
    (door, file)
}

// ---------------------------------------------------------------------
// Statement 5.
// ---------------------------------------------------------------------

#[test]
fn a_game_file_the_door_gives_is_one_play_continues() {
    let scenario = scenario(EASTERN_SOLOMONS);
    let pack = pack_bytes(&scenario);
    let calls = eastern_solomons_calls();
    let cut = before_the_day_ends(&calls);
    let (_, file) = door_played(&scenario, &pack, &calls[..cut]);
    assert!(
        !file.contains("\"tool\":\"get_briefing\"") && !file.contains("\"tool\":\"list_forces\""),
        "the door's file leaves the reads out: {file}"
    );
    assert!(
        file.contains("task_force:77"),
        "and keeps a call the game refused: {file}"
    );
    let dir = temp_dir("continues");
    let path = dir.join("door.game");
    std::fs::write(&path, &file).expect("the file is written");
    let path = path.to_str().expect("a path");
    // The game in one piece, through a door.
    let mut whole = begun(&scenario, &pack);
    for call in &calls[..cut] {
        through(&mut whole, call);
    }
    for call in &calls[cut..] {
        // `play` goes on from the door's file; the one piece goes on in the door.
        let want = through(&mut whole, call);
        if want.get("refused").is_some() {
            continue;
        }
        let got = play_answer(&play_call(path, None, call));
        assert_eq!(
            got, want,
            "{} after the cut is answered as in one piece",
            call.0
        );
    }
}

#[test]
fn a_game_file_play_wrote_is_one_the_door_resumes() {
    let scenario = scenario(EASTERN_SOLOMONS);
    let pack = pack_bytes(&scenario);
    let calls = eastern_solomons_calls();
    let cut = before_the_day_ends(&calls);
    let dir = temp_dir("resumes");
    let path = dir.join("play.game");
    let file = path.to_str().expect("a path");
    for (i, call) in calls[..cut].iter().enumerate() {
        let out = play_call(file, (i == 0).then_some(scenario.as_str()), call);
        // A call the protocol refuses is not kept and `play` exits in error.
        if call.0 == "no_such_tool" {
            assert!(!out.status.success());
        }
    }
    let written = std::fs::read_to_string(file).expect("the game file");
    assert!(
        written.contains("\"tool\":\"get_briefing\""),
        "play keeps the reads: {written}"
    );
    let mut resumed = Door::new();
    resumed.resume(&pack, &written).expect("the door resumes");
    // The same game in one piece, through a door.
    let mut whole = begun(&scenario, &pack);
    for call in &calls[..cut] {
        through(&mut whole, call);
    }
    for call in &calls[cut..] {
        assert_eq!(
            through(&mut resumed, call),
            through(&mut whole, call),
            "{} after the resuming is answered as in one piece",
            call.0
        );
    }
    assert_eq!(
        resumed.saved_game().expect("the saved game"),
        whole.saved_game().expect("the saved game"),
        "the resumed game's saved game is the one played in one piece"
    );
    // And what the resumed door gives of it is a file without the reads.
    assert!(!resumed
        .game_file()
        .expect("a file")
        .contains("\"tool\":\"get_briefing\""));
}

#[test]
fn the_doors_file_without_reads_replays_as_the_file_with_them() {
    let scenario = scenario(EASTERN_SOLOMONS);
    let pack = pack_bytes(&scenario);
    let calls = eastern_solomons_calls();
    // The whole game, to its end, through a door; its file leaves the reads out.
    let (mut door, without) = door_played(&scenario, &pack, &calls);
    let with = {
        // The file with the reads in: `play`'s own, made of the same calls
        // that the protocol does not refuse.
        let dir = temp_dir("with");
        let path = dir.join("with.game");
        let file = path.to_str().expect("a path");
        let mut first = true;
        for call in &calls {
            if call.0 == "no_such_tool" {
                continue;
            }
            let out = play_call(file, first.then_some(scenario.as_str()), call);
            first = false;
            // A call the game refuses is kept by `play` and its words are the
            // error; one the game answers is a success.
            let _ = out;
        }
        std::fs::read_to_string(file).expect("the file with the reads")
    };
    assert!(
        with.lines().count() > without.lines().count(),
        "the file with the reads is longer"
    );
    assert!(with.contains("\"tool\":\"events\"") && !without.contains("\"tool\":\"events\""));
    let dir = temp_dir("both");
    let a = dir.join("with.game");
    let b = dir.join("without.game");
    std::fs::write(&a, &with).expect("written");
    std::fs::write(&b, &without).expect("written");
    // The same answers: the calls that are not reads, replayed from each file
    // by the door, give the same saved game and the same file.
    let mut from_with = Door::new();
    from_with
        .resume(&pack, &with)
        .expect("the door resumes the file with the reads");
    let mut from_without = Door::new();
    from_without
        .resume(&pack, &without)
        .expect("the door resumes the file without them");
    assert_eq!(
        from_with.saved_game().expect("saved"),
        from_without.saved_game().expect("saved")
    );
    assert_eq!(
        from_with.game_file().expect("file"),
        from_without.game_file().expect("file")
    );
    assert_eq!(from_without.game_file().expect("file"), without);
    // And `play` reads each to the same saved game.
    let save_with = export_save(a.to_str().expect("a path"));
    let save_without = export_save(b.to_str().expect("a path"));
    assert_eq!(
        save_with, save_without,
        "play replays both to one saved game"
    );
    assert_eq!(
        save_without,
        door.saved_game().expect("the door's saved game"),
        "and it is the door's"
    );
    // Later calls answer alike from each file, in the door.
    let after = ("get_briefing", json!({}));
    assert_eq!(
        through(&mut from_with, &after),
        through(&mut from_without, &after)
    );
    assert_eq!(through(&mut from_with, &after), through(&mut door, &after));
}

// ---------------------------------------------------------------------
// Statement 5a.
// ---------------------------------------------------------------------

fn saved_game_after_the_last_day(scenario: &str, calls: &[(&'static str, Value)], days: usize) {
    let pack = pack_bytes(scenario);
    let mut door = begun(scenario, &pack);
    let mut ended = 0;
    for call in calls {
        if call.0 == "end_day" {
            ended += 1;
            if ended > days {
                break;
            }
        }
        through(&mut door, call);
        if call.0 == "end_day" && ended < days {
            let refused = door
                .saved_game()
                .expect_err("no saved game before the last day");
            assert!(refused.contains("the game is not over"), "{refused}");
        }
    }
    let saved = door
        .saved_game()
        .expect("the saved game once the game is over");
    let file = door.game_file().expect("the game file");
    let dir = temp_dir("saved");
    let path = dir.join("saved.game");
    std::fs::write(&path, &file).expect("written");
    let made = export_save(path.to_str().expect("a path"));
    assert_eq!(
        saved, made,
        "the door's saved game is what play --export-save writes"
    );
}

#[test]
fn the_saved_game_is_refused_until_the_game_is_over_and_then_is_what_play_writes_for_the_eastern_solomons(
) {
    let scenario = scenario(EASTERN_SOLOMONS);
    let pack = pack_bytes(&scenario);
    let door = begun(&scenario, &pack);
    let refused = door.saved_game().expect_err("none on the first day");
    assert!(refused.contains("the game is not over"), "{refused}");
    saved_game_after_the_last_day(&scenario, &eastern_solomons_calls(), 1);
}

#[test]
fn the_saved_game_is_refused_until_the_game_is_over_and_then_is_what_play_writes_for_midway() {
    saved_game_after_the_last_day(&scenario(MIDWAY), &midway_calls(), 5);
}

// ---------------------------------------------------------------------
// Statement 5b.
// ---------------------------------------------------------------------

#[test]
fn a_second_begin_on_a_door_that_has_a_game_is_refused() {
    let scenario = scenario(EASTERN_SOLOMONS);
    let pack = pack_bytes(&scenario);
    let mut door = begun(&scenario, &pack);
    let before = door.game_file().expect("a file");
    let request = json!({"scenario": scenario, "seat": "japanese", "seed": null}).to_string();
    let refused = door
        .begin(&pack, &request)
        .expect_err("a second begin is refused");
    assert!(refused.contains("this door has a game"), "{refused}");
    // Neither the seat nor the game changed.
    assert_eq!(door.game_file().expect("a file"), before);
    let resumed = door.resume(&pack, &before).expect_err("so is a resume");
    assert!(resumed.contains("this door has a game"), "{resumed}");
    // A door given a game by `resume` refuses a begin as well.
    let mut other = Door::new();
    other.resume(&pack, &before).expect("resumed");
    assert!(other.begin(&pack, &request).is_err());
}

#[test]
fn a_door_with_no_game_answers_nothing_of_one() {
    let mut door = Door::new();
    for answer in [
        door.tools().map(|_| ()),
        door.game_file().map(|_| ()),
        door.saved_game().map(|_| ()),
        door.call("get_briefing", "{}").map(|_| ()),
    ] {
        assert!(answer.expect_err("no game").contains("no game has begun"));
    }
}

#[test]
fn a_seed_is_a_string_of_digits() {
    let scenario = scenario(EASTERN_SOLOMONS);
    let pack = pack_bytes(&scenario);
    for bad in [
        json!(7),
        json!("7x"),
        json!(""),
        json!("99999999999999999999999"),
    ] {
        let mut door = Door::new();
        let request = json!({"scenario": scenario, "seat": "allied", "seed": bad}).to_string();
        assert!(door.begin(&pack, &request).is_err(), "{bad} is refused");
    }
    let mut door = Door::new();
    let request =
        json!({"scenario": scenario, "seat": "allied", "seed": "12345678901234567890"}).to_string();
    door.begin(&pack, &request)
        .expect("a seed of twenty digits");
    assert!(door
        .game_file()
        .expect("a file")
        .contains("12345678901234567890"));
}

// ---------------------------------------------------------------------
// The test owed on pull request 732 (B).
// ---------------------------------------------------------------------

#[test]
fn a_call_after_a_read_in_the_middle_of_midway_is_answered_alike_with_the_reads_or_without() {
    let scenario = scenario(MIDWAY);
    let pack = pack_bytes(&scenario);
    // To the evening of day 3 (the clock stands at dusk before day 3), with
    // reads among the calls and a call the game refuses.
    let before: Vec<(&str, Value)> = vec![
        ("get_briefing", json!({})),
        ("list_forces", json!({})),
        ("queued", json!({})),
        (
            "issue_orders",
            json!({"orders": [{"unit": "task_force:77", "order": {"kind": "set_speed", "speed": "full"}}]}),
        ),
        ("end_day", json!({})),
        ("get_briefing", json!({})),
        ("events", json!({"day": 1})),
        ("intelligence", json!({})),
        ("end_day", json!({})),
        ("list_contacts", json!({})),
        ("command_state", json!({})),
        ("queued", json!({})),
    ];
    // The file without the reads is the door's own; the file with them is
    // the same header and every one of the calls, in `play`'s format.
    let (_, without) = door_played(&scenario, &pack, &before);
    let header = without.lines().next().expect("a header");
    let mut with = format!("{header}\n");
    for (tool, arguments) in &before {
        with.push_str(&json!({"tool": tool, "arguments": arguments}).to_string());
        with.push('\n');
    }
    assert!(with.lines().count() > without.lines().count());
    let mut a = Door::new();
    a.resume(&pack, &with).expect("resumed with its reads");
    let mut b = Door::new();
    b.resume(&pack, &without).expect("resumed without them");

    let mut both = |call: (&str, Value)| -> Value {
        let from_a = through(&mut a, &call);
        let from_b = through(&mut b, &call);
        assert_eq!(from_a, from_b, "{} {} is answered alike", call.0, call.1);
        from_a
    };
    let order = |hex: [i32; 2]| {
        (
            "issue_orders",
            json!({"orders": [{"unit": "submarine:1", "order": {"kind": "station", "hex": hex}}]}),
        )
    };
    both(("get_briefing", json!({})));
    both(order([7, 0]));
    let queued = both(("queued", json!({})));
    let entry = &queued["ok"]["entries"][0];
    assert!(
        entry["place"]["number"].is_number(),
        "an entry has a number: {queued}"
    );
    let taken = both((
        "take_back",
        json!({"queue": entry["place"]["queue"], "number": entry["place"]["number"], "as_of": queued["ok"]["as_of"]}),
    ));
    assert!(taken["ok"].is_object(), "the entry is taken back: {taken}");
    both(("queued", json!({})));
    both(order([6, 1]));
    let preview = both((
        "preview",
        json!({"command": "set_intent", "intent_kind": "patrol", "hex": [5, 0], "distance": 2}),
    ));
    let number = preview["ok"]["preview"].clone();
    assert!(number.is_number(), "a preview has a number: {preview}");
    let approved = both(("approve", json!({"preview": number})));
    assert!(approved["ok"].is_object(), "approved: {approved}");
    both(("queued", json!({})));
    both(("plans", json!({})));
    for _ in 0..3 {
        both(("end_day", json!({})));
        both(("get_briefing", json!({})));
        both(("list_forces", json!({})));
        both(("queued", json!({})));
    }
    let over = both(("get_briefing", json!({})));
    assert!(
        over["ok"].is_object(),
        "the briefing after the last day: {over}"
    );
    assert_eq!(
        a.saved_game().expect("saved game with the reads"),
        b.saved_game().expect("saved game without them"),
        "the saved games are the same"
    );
    assert_eq!(a.game_file().expect("file"), b.game_file().expect("file"));
}
