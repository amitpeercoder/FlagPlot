//! `sim-cli mcp` and `sim-cli play` through the real binary (decisions 1 to 11
//! of `docs/decisions/0051-mcp-server-and-scenario-9.md`).
//!
//! Four things are tested: the protocol, with a scripted client; `play`
//! against a server held open (the same calls give the same answers, save and
//! transcript); the harness that replays recorded games kept under
//! `tests/recorded_games/` (scenario 9's record), proved on a game made here;
//! and determinism.

use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use serde_json::{json, Value};

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
const S05: &str = "data/scenarios/05_eastern_solomons.toml";
const S07: &str = "data/scenarios/07_full_slice.toml";
const BIN: &str = env!("CARGO_BIN_EXE_sim-cli");

fn temp_dir(label: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("flagplot-mcp-{}-{label}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("make the temp dir");
    dir
}

fn s(path: &Path) -> &str {
    path.to_str().expect("a path is text")
}

/// A scripted client: `sim-cli mcp` as a child, one line in and one line out.
struct Client {
    child: Child,
    stdin: Option<ChildStdin>,
    stdout: BufReader<ChildStdout>,
    next: i64,
}

impl Client {
    fn start(scenario: &str, extra: &[&str]) -> Client {
        let mut child = Command::new(BIN)
            .arg("mcp")
            .arg(scenario)
            .args(["--seat", "allied"])
            .args(extra)
            .current_dir(ROOT)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("the binary starts");
        let stdin = child.stdin.take();
        let stdout = BufReader::new(child.stdout.take().expect("stdout"));
        Client {
            child,
            stdin,
            stdout,
            next: 1,
        }
    }

    fn send_raw(&mut self, line: &str) {
        let stdin = self.stdin.as_mut().expect("stdin is open");
        stdin.write_all(line.as_bytes()).expect("write");
        stdin.write_all(b"\n").expect("write");
        stdin.flush().expect("flush");
    }

    fn read_line(&mut self) -> Value {
        let mut line = String::new();
        self.stdout.read_line(&mut line).expect("read");
        assert!(!line.is_empty(), "the server closed its output");
        assert!(line.ends_with('\n'));
        let value: Value = serde_json::from_str(&line).expect("a response is JSON");
        assert_eq!(value["jsonrpc"], "2.0", "{line}");
        value
    }

    /// A request, and its response with the id checked.
    fn request(&mut self, method: &str, params: Value) -> Value {
        let id = self.next;
        self.next += 1;
        self.send_raw(
            &json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params}).to_string(),
        );
        let response = self.read_line();
        assert_eq!(response["id"], id, "{response}");
        response
    }

    fn handshake(&mut self) -> Value {
        let init = self.request(
            "initialize",
            json!({"protocolVersion": "2025-11-25", "capabilities": {}, "clientInfo": {"name": "test", "version": "0"}}),
        );
        self.send_raw(
            &json!({"jsonrpc": "2.0", "method": "notifications/initialized"}).to_string(),
        );
        init
    }

    /// A tool call: the text of the answer and whether it is marked an error.
    fn call(&mut self, tool: &str, arguments: Value) -> (String, bool) {
        let response = self.request("tools/call", json!({"name": tool, "arguments": arguments}));
        let result = &response["result"];
        assert!(result.is_object(), "{response}");
        let content = result["content"].as_array().expect("content");
        assert_eq!(content.len(), 1);
        assert_eq!(content[0]["type"], "text");
        let text = content[0]["text"].as_str().expect("text").to_string();
        assert!(!text.is_empty());
        let is_error = result["isError"].as_bool().expect("isError is stated");
        (text, is_error)
    }

    /// Ends the input and waits for the server, returning its standard error.
    fn finish(mut self) -> String {
        drop(self.stdin.take());
        let mut rest = String::new();
        self.stdout.read_line(&mut rest).expect("read");
        assert_eq!(rest, "", "nothing more is written after the last answer");
        let output = self.child.wait_with_output().expect("wait");
        assert!(output.status.success());
        String::from_utf8_lossy(&output.stderr).into_owned()
    }
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
        sim_core::orders::OrderKind::LandUnit { target, .. } => format!(
            "land_unit:{}",
            number(
                view.land_units().map(|u| u.id.raw()).collect(),
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

/// The scenario's Allied orders for a day, as `issue_orders` takes them.
fn scripted_orders(scenario: &str, day: u32) -> Vec<Value> {
    let scenario = sim_data::Scenario::load(&Path::new(ROOT).join(scenario)).expect("loads");
    let view = scenario.world.side_view(sim_core::state::Side::Allied);
    scenario
        .scripted_orders()
        .get(&day)
        .map(|list| {
            list.iter()
                .filter(|(side, _)| *side == sim_core::state::Side::Allied)
                .map(|(_, r)| served_order(&view, r))
                .collect()
        })
        .unwrap_or_default()
}

fn run(args: &[&str]) -> std::process::Output {
    Command::new(BIN)
        .args(args)
        .current_dir(ROOT)
        .output()
        .expect("the binary runs")
}

fn out_text(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

// ---------------------------------------------------------------------
// The protocol.
// ---------------------------------------------------------------------

#[test]
fn a_scripted_client_plays_scenario_5_through_the_protocol() {
    let dir = temp_dir("protocol");
    let (save, transcript) = (dir.join("game.json"), dir.join("t.jsonl"));
    let mut c = Client::start(S05, &["--save", s(&save), "--transcript", s(&transcript)]);

    // Before the handshake there is nothing to call.
    let early = c.request("tools/list", json!({}));
    assert_eq!(early["error"]["code"], -32602, "{early}");

    let init = c.handshake();
    assert_eq!(init["result"]["protocolVersion"], "2025-11-25");
    assert_eq!(
        init["result"]["capabilities"],
        json!({"tools": {"listChanged": false}})
    );
    assert!(init["result"]["serverInfo"]["name"].is_string());
    let instructions = init["result"]["instructions"].as_str().unwrap();
    assert!(instructions.contains("end_day") && instructions.contains("get_briefing"));
    // A client that names a version we do not know is answered with ours.
    let other = c.request("initialize", json!({"protocolVersion": "1999-01-01"}));
    assert_eq!(other["result"]["protocolVersion"], "2025-11-25");
    let older = c.request("initialize", json!({"protocolVersion": "2025-03-26"}));
    assert_eq!(older["result"]["protocolVersion"], "2025-03-26");

    let ping = c.request("ping", json!({}));
    assert_eq!(ping["result"], json!({}));

    let listed = c.request("tools/list", json!({}));
    let tools = listed["result"]["tools"].as_array().expect("tools");
    assert_eq!(tools.len(), 9);
    let names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert_eq!(
        names,
        [
            "get_briefing",
            "list_forces",
            "list_contacts",
            "list_bases",
            "get_entity",
            "measure",
            "explain",
            "issue_orders",
            "end_day"
        ]
    );
    for tool in tools {
        assert!(!tool["description"].as_str().unwrap().is_empty());
        assert_eq!(tool["inputSchema"]["type"], "object", "{}", tool["name"]);
        let text = tool["inputSchema"].to_string();
        assert!(!text.contains("$ref"), "{}: a $ref is left", tool["name"]);
    }
    let orders = tools.iter().find(|t| t["name"] == "issue_orders").unwrap();
    for kind in [
        "set_route",
        "set_air_mission",
        "set_rules_of_engagement",
        "detach",
    ] {
        assert!(orders["description"].as_str().unwrap().contains(kind));
        assert!(orders["inputSchema"].to_string().contains(kind));
    }
    // What the missions do, the arc convention and the length of the game are
    // said where a player reads them (the review of scenario 9 play).
    let description = orders["description"].as_str().unwrap();
    for phrase in [
        "set missions BEFORE the contact appears",
        "naval_strike: the staff launches the group, in the same pulse",
        "base_strike {target_base: N}: in the first daylight pulse of a day",
        "TARGET PRIORITY. A naval_strike group has a target list",
        "set_target_priority {kinds:",
        "combat_air_patrol (fighters)",
        "A group flies only the mission it is on",
        "with no arc set it flies nothing",
        "0 is north",
        "clockwise from from_deg to to_deg",
        "multiples of 10 from 0 to 350",
        "at most the aircraft type's search range",
        "queued_in_call",
        "the other orders in the call stand",
    ] {
        assert!(description.contains(phrase), "description lacks: {phrase}");
    }
    let schema_text = orders["inputSchema"].to_string();
    for phrase in ["0 is north", "multiple of 10 from 0 to 350", "search range"] {
        assert!(schema_text.contains(phrase), "input schema lacks: {phrase}");
    }
    for phrase in [
        "A scenario without objectives has no result",
        "Day 2 of 5",
        "BEFORE the contact appears",
        "pilots of null",
        "due day N",
        "not known",
    ] {
        assert!(instructions.contains(phrase), "instructions lack: {phrase}");
    }

    // The wording pass after the second round: what a player of one day, of
    // patrol shares, of contacts and of the pulses had to guess.
    for phrase in [
        // 4. A one-day game.
        "A scenario can be only one day long",
        "you must give all your orders for the whole game before the first end_day",
        // 1. Levels and the history note.
        "the levels (the margin of your points over the enemy's",
        "a stalemate has no level",
        "what happened in the real battle, not in your game",
        // 10. The patrol share.
        "set_patrol_share {share} is the share of the group kept over its own hex: 1 (where it starts) keeps every fighter at home and sends no escort, and 0.5 keeps half at home and lets half escort",
        // 11. A contact's kind.
        "the kind it is taken for and the number of ships are what the sighting reported. They can be wrong, and can change when the contact is seen again",
        // 12. The pulses.
        "A day is three pulses: night, morning and afternoon",
        "begins at 18:00 on the evening before the day",
        "Searches and strikes fly only in the morning and afternoon pulses; the afternoon pulse is the last of the day",
        // 13. Radius.
        "give each type's radius",
        // Ground (decision 0090).
        "you attack and hold ground with set_posture {posture: attack or defend}",
        "a base with no man of its side ashore falls to one man on attack in its hex",
        "Fly your air groups out by rebase a day before",
        // Shells and bombs ashore, and the estimate (decision 0092).
        "set_mission {mission: bombardment}",
        "end the night's move in the hex of an enemy base that is on your map",
        "a claim of aircraft destroyed on the ground, never the true hits",
        "A base that is not on your map cannot be bombarded or struck until it is found",
        "wrong by up to half either way",
        "an opening estimate of the enemy's men, dated day 0, which can be far wrong",
        "Bombs from the air now also hit men ashore",
        // Marching (decision 0094).
        "marches with march {hex: [q, r]}",
        "lists the hexes it can march to, under march_to",
        "A march takes the scenario's march_days_per_hex days",
        "fatigue_per_march_day",
        "march_supply_days days of its men's daily charge",
        "the enemy does not know of it until a ground report finds it",
        "ordering the unit's own hex stops the march",
        "A map with no land links allows no march",
        // Acting at a place and steaming on (decision 0096).
        "set_route {waypoints: [[q, r], ...], act_at: [[q, r], ...]}",
        "A force halts at a marked waypoint only if it has work there",
        "by day it waits for the night",
        "The last waypoint needs no mark",
        "a new route order ends a halt",
        "To raid by night and be gone by day, route the force to the enemy base marked to act there and then back out",
        "To send a convoy that comes home by itself, give an unload order and a route to the base marked to act there, then home",
        "The enemy sees none of it",
    ] {
        assert!(instructions.contains(phrase), "instructions lack: {phrase}");
    }
    for phrase in [
        // 5. Load and unload.
        "LOAD AND UNLOAD",
        "work only while the force is stopped (idle or arrived) in the hex of a base of your own side",
        "a load in the same day as a sailing is refused",
        "unload {} is never refused, wherever the force is",
        "an unload given while the force is at sea is queued, stands as the force's cargo order, and is carried out after the force arrives",
        // Ground (decision 0090).
        "FIGHT ASHORE",
        "takes one order, set_posture {posture: attack or defend}",
        "goes back to defend and its order is spent",
        "falls to a single man of yours on attack in its hex, with no fight",
        // Bombardment (decision 0092).
        "BOMBARDMENT. set_mission {mission: bombardment} is a standing order",
        "a base not on your map cannot be bombarded",
        // Marching (decision 0094).
        "march {hex: [q, r]}, for moving (see MARCH)",
        "MARCH. march {hex: [q, r]} sends a land unit's men ashore to a neighbouring hex joined to the unit's hex by a land link",
        "the unit's own hex stops a march",
        "no land link joins its hex to the hex named",
        // Acting at a place and steaming on (decision 0096).
        "set_route {waypoints: [[q, r], ...], act_at: [[q, r], ...] (optional)}",
        "ACT AT A PLACE. A route may mark waypoints to act at with act_at",
        "the last waypoint needs no mark",
        "A new route order ends a halt",
        "To raid by night and be gone by day: route the force to the enemy base",
        "To send a convoy that comes home by itself: give an unload order and a route to the base marked to act there, then home",
        // 10. The patrol share.
        "set_patrol_share {share} is the share of the group kept over its own hex: 1 (where it starts) keeps every fighter at home and sends no escort, 0.5 keeps half at home and lets half escort",
    ] {
        assert!(description.contains(phrase), "issue_orders lacks: {phrase}");
    }
    let tool_description = |name: &str| {
        tools.iter().find(|t| t["name"] == name).unwrap()["description"]
            .as_str()
            .unwrap()
            .to_string()
    };
    // 12 and 9. The passage's departure, and air_reach's places.
    let measure_description = tool_description("measure");
    for phrase in [
        "\"leaves\" at the start of the coming day's night pulse, which is 18:00 on the evening before the day",
        "one place alone is taken as a list of one",
        "one such place anywhere in the question makes the whole answer \"not known\"",
    ] {
        assert!(measure_description.contains(phrase), "measure lacks: {phrase}");
    }
    // 11. The kind a contact is taken for.
    for tool in ["list_contacts", "get_entity"] {
        assert!(
            tool_description(tool)
                .contains("can be wrong, and can change when the contact is seen again"),
            "{tool}"
        );
    }
    // The estimates of the enemy's men ashore (decision 0092).
    assert!(tool_description("list_contacts").contains("ground_estimates"));
    assert!(tool_description("get_briefing").contains("estimates of the enemy's men ashore"));
    // 6, 2, 7, 13, 4. What the briefing and the lists now show.
    assert!(tool_description("list_forces").contains("cargo_supply_t and cargo_fuel_t"));
    let briefing_description = tool_description("get_briefing");
    for phrase in [
        "strike groups that hold contacts or a target but have none within their radius",
        "stocks yesterday by cause",
        "each type's radius and search range in miles",
        "says so in a plain line when the day is the last",
        "history note of what happened in the real battle",
    ] {
        assert!(
            briefing_description.contains(phrase),
            "get_briefing lacks: {phrase}"
        );
    }
    let orders_schema = orders["inputSchema"].to_string();
    assert!(orders_schema.contains("The share of the group kept over its own hex"));

    // Every tool, with valid arguments.
    let (briefing, bad) = c.call("get_briefing", json!({}));
    assert!(
        !bad && briefing.starts_with("FLAG PLOT BRIEFING"),
        "{briefing}"
    );
    let (data, bad) = c.call("get_briefing", json!({"text": false}));
    assert!(!bad);
    assert_eq!(
        serde_json::from_str::<Value>(&data).unwrap()["kind"],
        "briefing"
    );
    let (forces, _) = c.call("list_forces", json!({}));
    assert_eq!(
        serde_json::from_str::<Value>(&forces).unwrap()["kind"],
        "list_forces"
    );
    let (_, bad) = c.call(
        "list_contacts",
        json!({"area": {"q": 7, "r": -5, "radius_nm": 100}}),
    );
    assert!(!bad);
    let (bases, _) = c.call("list_bases", json!({}));
    assert_eq!(
        serde_json::from_str::<Value>(&bases).unwrap()["kind"],
        "list_bases"
    );
    let (entity, bad) = c.call("get_entity", json!({"handle": "task_force:1"}));
    assert!(!bad && entity.contains("\"entity\""));
    let (measure, bad) = c.call(
        "measure",
        json!({"question": "air_distance", "from": "task_force:1", "to": "base:1"}),
    );
    assert!(!bad && measure.contains("distance_nm"));
    let (explain, bad) = c.call("explain", json!({"event": 999}));
    assert!(!bad && explain.contains("not_known"));

    // A call the game refuses as a whole is a tool error; the text says why.
    let (why, bad) = c.call("get_entity", json!({}));
    assert!(bad && why.contains("handle"), "{why}");
    let (why, bad) = c.call("explain", json!({"event": "x"}));
    assert!(bad, "{why}");
    let (why, bad) = c.call("measure", json!({"question": "teleport"}));
    assert!(bad, "{why}");
    // An order for an enemy unit is answered, not an error: the refusal is in the answer.
    let (refusal, bad) = c.call(
        "issue_orders",
        json!({"orders": [{"unit": "task_force:3", "order": {"kind": "set_speed", "speed": "full"}}]}),
    );
    assert!(!bad, "{refusal}");
    let refusal: Value = serde_json::from_str(&refusal).unwrap();
    assert_eq!(refusal["results"][0]["status"], "refused");
    assert_eq!(refusal["results"][0]["reason"], "not known");
    // A malformed order is refused inside the answer too.
    let (malformed, bad) = c.call(
        "issue_orders",
        json!({"orders": [{"unit": "task_force:1"}]}),
    );
    assert!(!bad && malformed.contains("malformed order"));
    // The scenario's own orders.
    let (queued, bad) = c.call(
        "issue_orders",
        json!({"orders": scripted_orders(S05, 1), "clear": true}),
    );
    assert!(!bad);
    let queued: Value = serde_json::from_str(&queued).unwrap();
    assert!(queued["queued"].as_u64().unwrap() > 0);
    assert!(queued["results"]
        .as_array()
        .unwrap()
        .iter()
        .all(|r| r["status"] == "queued"));

    let (ended, bad) = c.call("end_day", json!({}));
    assert!(!bad);
    let ended: Value = serde_json::from_str(&ended).unwrap();
    assert_eq!(ended["resolved"], true);
    assert_eq!(ended["game_over"], true, "the game-over answer arrives");
    // After the last day the seat still reads and cannot order.
    let (after, bad) = c.call("get_briefing", json!({}));
    assert!(!bad && after.starts_with("FLAG PLOT BRIEFING"), "{after}");
    let (why, bad) = c.call("issue_orders", json!({"orders": []}));
    assert!(bad && why == "the game is over", "{why}");

    // Protocol errors.
    let r = c.request("resources/list", json!({}));
    assert_eq!(r["error"]["code"], -32601, "{r}");
    let r = c.request("tools/call", json!({"name": "get_truth", "arguments": {}}));
    assert_eq!(r["error"]["code"], -32602, "{r}");
    assert!(r["error"]["message"]
        .as_str()
        .unwrap()
        .contains("get_truth"));
    let r = c.request("tools/call", json!({"arguments": {}}));
    assert_eq!(r["error"]["code"], -32602, "{r}");
    let r = c.request("tools/call", json!({"name": "end_day", "arguments": [1]}));
    assert_eq!(r["error"]["code"], -32602, "{r}");
    c.send_raw("this is not json");
    let r = c.read_line();
    assert_eq!(r["error"]["code"], -32700, "{r}");
    assert_eq!(r["id"], Value::Null);
    c.send_raw("[1, 2]");
    assert_eq!(c.read_line()["error"]["code"], -32600);
    c.send_raw(&json!({"jsonrpc": "2.0", "id": null, "method": "ping"}).to_string());
    assert_eq!(c.read_line()["error"]["code"], -32600);
    c.send_raw(&json!({"jsonrpc": "2.0", "id": 77}).to_string());
    let r = c.read_line();
    assert_eq!(r["error"]["code"], -32600);
    assert_eq!(r["id"], 77);
    // Notifications get no reply: the next line read answers the ping after them.
    c.send_raw(
        &json!({"jsonrpc": "2.0", "method": "notifications/cancelled", "params": {"requestId": 1}})
            .to_string(),
    );
    c.send_raw(&json!({"jsonrpc": "2.0", "method": "no/such/notification"}).to_string());
    c.send_raw(&json!({"jsonrpc": "2.0", "id": "after", "method": "ping"}).to_string());
    let r = c.read_line();
    assert_eq!(r["id"], "after", "{r}");
    c.next += 1;

    let stderr = c.finish();
    assert!(!stderr.contains("panicked"), "{stderr}");

    // The saved game replays to the hashes of the scenario's ordinary run.
    let replay = run(&["replay", s(&save)]);
    assert!(
        replay.status.success(),
        "{}",
        String::from_utf8_lossy(&replay.stderr)
    );
    let replayed: Vec<String> = out_text(&replay)
        .lines()
        .filter(|l| l.starts_with("day "))
        .map(str::to_string)
        .collect();
    let direct: Vec<String> = out_text(&run(&["hash", S05]))
        .lines()
        .map(str::to_string)
        .collect();
    assert_eq!(replayed, direct);

    // The transcript has the calls that reached the game, and not the
    // protocol errors: nine tools called, the briefing's arguments as sent
    // with the text it defaults to made explicit.
    let entries: Vec<Value> = fs::read_to_string(&transcript)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    assert_eq!(entries[0]["tool"], "get_briefing");
    assert_eq!(entries[0]["arguments"], json!({"text": true}));
    assert_eq!(entries[0]["answer"]["kind"], "briefing_text");
    assert!(entries.iter().all(|e| e["tool"] != "get_truth"));
}

#[test]
fn the_modern_revision_is_served_by_per_request_metadata() {
    let mut c = Client::start(S05, &[]);
    let meta = json!({
        "io.modelcontextprotocol/protocolVersion": "2026-07-28",
        "io.modelcontextprotocol/clientInfo": {"name": "test", "version": "0"},
        "io.modelcontextprotocol/clientCapabilities": {}
    });
    // No handshake at all.
    let d = c.request("server/discover", json!({"_meta": meta}));
    assert_eq!(d["result"]["resultType"], "complete");
    assert_eq!(d["result"]["supportedVersions"][0], "2026-07-28");
    assert_eq!(
        d["result"]["capabilities"],
        json!({"tools": {"listChanged": false}})
    );
    assert!(d["result"]["instructions"].is_string());
    assert!(d["result"]["_meta"]["io.modelcontextprotocol/serverInfo"]["name"].is_string());
    let l = c.request("tools/list", json!({"_meta": meta}));
    assert_eq!(l["result"]["tools"].as_array().unwrap().len(), 9);
    assert_eq!(l["result"]["resultType"], "complete");
    let r = c.request(
        "tools/call",
        json!({"name": "list_bases", "arguments": {}, "_meta": meta}),
    );
    assert_eq!(r["result"]["resultType"], "complete");
    assert_eq!(r["result"]["isError"], false);
    // A version we do not serve: -32022 naming what we do.
    let bad = c.request(
        "tools/list",
        json!({"_meta": {
            "io.modelcontextprotocol/protocolVersion": "1900-01-01",
            "io.modelcontextprotocol/clientCapabilities": {}
        }}),
    );
    assert_eq!(bad["error"]["code"], -32022, "{bad}");
    assert_eq!(bad["error"]["data"]["requested"], "1900-01-01");
    assert_eq!(bad["error"]["data"]["supported"][0], "2026-07-28");
    // Capabilities are required.
    let bad = c.request(
        "tools/list",
        json!({"_meta": {"io.modelcontextprotocol/protocolVersion": "2026-07-28"}}),
    );
    assert_eq!(bad["error"]["code"], -32602, "{bad}");
    c.finish();
}

// ---------------------------------------------------------------------
// `play` and the server held open give the same game.
// ---------------------------------------------------------------------

/// The calls of a five-day (or one-day) game: read, order as the scenario
/// does, end the day.
fn day_calls(scenario: &str, days: u32) -> Vec<(String, Value)> {
    let mut calls = Vec::new();
    for day in 1..=days {
        calls.push(("get_briefing".to_string(), json!({})));
        calls.push(("list_forces".to_string(), json!({})));
        calls.push((
            "issue_orders".to_string(),
            json!({"orders": scripted_orders(scenario, day)}),
        ));
        calls.push(("end_day".to_string(), json!({})));
    }
    calls.push(("get_briefing".to_string(), json!({"text": false})));
    // A call the game answers as not known, and one it refuses as a whole.
    calls.push(("explain".to_string(), json!({"event": 1})));
    calls.push(("get_entity".to_string(), json!({})));
    calls
}

/// Plays the calls through one `mcp` process: the answer texts, the save and
/// the transcript.
fn through_mcp(
    scenario: &str,
    calls: &[(String, Value)],
    dir: &Path,
) -> (Vec<(String, bool)>, PathBuf, PathBuf) {
    let (save, transcript) = (dir.join("mcp-save.json"), dir.join("mcp-transcript.jsonl"));
    let mut c = Client::start(
        scenario,
        &["--save", s(&save), "--transcript", s(&transcript)],
    );
    c.handshake();
    let answers = calls.iter().map(|(t, a)| c.call(t, a.clone())).collect();
    c.finish();
    (answers, save, transcript)
}

/// Plays the calls through `play`, one process each.
fn through_play(
    scenario: &str,
    calls: &[(String, Value)],
    dir: &Path,
) -> (Vec<(String, bool)>, PathBuf, PathBuf) {
    let game = dir.join("game.txt");
    let mut answers = Vec::new();
    for (i, (tool, arguments)) in calls.iter().enumerate() {
        let arguments = arguments.to_string();
        let mut args = vec!["play", "--game", s(&game)];
        if i == 0 {
            args.extend(["--scenario", scenario, "--seat", "allied"]);
        }
        args.extend([tool.as_str(), arguments.as_str()]);
        let output = run(&args);
        answers.push(if output.status.success() {
            // `play` prints the text and a newline.
            let text = out_text(&output);
            assert!(output.stderr.is_empty());
            (
                text.strip_suffix('\n').expect("a newline").to_string(),
                false,
            )
        } else {
            assert_eq!(output.status.code(), Some(1));
            assert!(output.stdout.is_empty());
            let err = String::from_utf8_lossy(&output.stderr).into_owned();
            let err = err.strip_prefix("error: ").expect("the error prefix");
            (err.strip_suffix('\n').expect("a newline").to_string(), true)
        });
    }
    let (save, transcript) = (
        dir.join("play-save.json"),
        dir.join("play-transcript.jsonl"),
    );
    let exported = run(&[
        "play",
        "--game",
        s(&game),
        "--export-save",
        s(&save),
        "--export-transcript",
        s(&transcript),
    ]);
    assert!(
        exported.status.success(),
        "{}",
        String::from_utf8_lossy(&exported.stderr)
    );
    (answers, save, transcript)
}

fn play_equals_mcp(scenario: &str, days: u32, label: &str) {
    let dir = temp_dir(label);
    let calls = day_calls(scenario, days);
    let (a, save_a, transcript_a) = through_play(scenario, &calls, &dir);
    let (b, save_b, transcript_b) = through_mcp(scenario, &calls, &dir);
    assert_eq!(a.len(), b.len());
    for (i, (x, y)) in a.iter().zip(&b).enumerate() {
        assert_eq!(x, y, "answer {i}: {:?}", calls[i]);
    }
    assert!(
        b.iter().any(|(_, error)| *error),
        "an answer that is a tool error is in the run"
    );
    assert_eq!(
        fs::read(&transcript_a).unwrap(),
        fs::read(&transcript_b).unwrap()
    );
    assert_eq!(fs::read(&save_a).unwrap(), fs::read(&save_b).unwrap());
    let replay = run(&["replay", s(&save_a)]);
    assert!(
        replay.status.success(),
        "{}",
        String::from_utf8_lossy(&replay.stderr)
    );
}

#[test]
fn play_and_the_held_open_server_agree_on_scenario_5() {
    play_equals_mcp(S05, 1, "equal5");
}

#[test]
fn play_and_the_held_open_server_agree_over_the_fourteen_days_of_the_full_slice() {
    // To the end: `play` gives a saved game only once the game is over.
    play_equals_mcp(S07, 14, "equal7");
}

#[test]
fn play_checks_its_game_file_and_keeps_only_calls_the_protocol_accepted() {
    let dir = temp_dir("playfile");
    let game = dir.join("g.txt");
    // A mistake in the command line is not kept and creates nothing.
    let bad = run(&[
        "play",
        "--game",
        s(&game),
        "--scenario",
        S05,
        "--seat",
        "allied",
        "get_truth",
    ]);
    assert_eq!(bad.status.code(), Some(1));
    assert!(!game.exists());
    let bad = run(&[
        "play",
        "--game",
        s(&game),
        "--scenario",
        S05,
        "--seat",
        "allied",
        "end_day",
        "{oops",
    ]);
    assert_eq!(bad.status.code(), Some(1));
    assert!(!game.exists());
    // A new game needs its scenario and seat.
    let bad = run(&["play", "--game", s(&game), "get_briefing"]);
    assert_eq!(bad.status.code(), Some(1));
    // A call the game answers with an error is kept.
    let kept = run(&[
        "play",
        "--game",
        s(&game),
        "--scenario",
        S05,
        "--seat",
        "allied",
        "get_entity",
        "{}",
    ]);
    assert_eq!(kept.status.code(), Some(1));
    assert_eq!(fs::read_to_string(&game).unwrap().lines().count(), 2);
    // Later calls may leave the options out; wrong ones are refused.
    let ok = run(&["play", "--game", s(&game), "list_bases"]);
    assert!(ok.status.success());
    let wrong = run(&[
        "play",
        "--game",
        s(&game),
        "--seat",
        "japanese",
        "list_bases",
    ]);
    assert_eq!(wrong.status.code(), Some(1));
    let wrong = run(&["play", "--game", s(&game), "--scenario", S07, "list_bases"]);
    assert_eq!(wrong.status.code(), Some(1));
    let wrong = run(&["play", "--game", s(&game), "--seed", "5", "list_bases"]);
    assert_eq!(wrong.status.code(), Some(1));
    assert_eq!(fs::read_to_string(&game).unwrap().lines().count(), 3);
    // Usage and the tool list.
    let usage = run(&["play"]);
    assert!(usage.status.success() && out_text(&usage).contains("Usage: sim-cli play"));
    let usage = run(&["play", "--game", s(&game)]);
    assert!(usage.status.success() && out_text(&usage).contains("Usage: sim-cli play"));
    let tools = run(&["play", "--game", s(&game), "--tools"]);
    let text = out_text(&tools);
    assert!(tools.status.success());
    for name in ["get_briefing", "issue_orders", "measure", "end_day"] {
        assert!(text.contains(&format!("TOOL {name}")));
    }
    assert!(text.contains("input schema:"));
    // A game with a seed keeps it.
    let seeded = dir.join("seeded.txt");
    let ok = run(&[
        "play",
        "--game",
        s(&seeded),
        "--scenario",
        S05,
        "--seat",
        "allied",
        "--seed",
        "7",
        "end_day",
    ]);
    assert!(ok.status.success());
    assert!(fs::read_to_string(&seeded).unwrap().contains("\"seed\":7"));
}

#[test]
fn the_same_input_gives_byte_identical_output() {
    let dir = temp_dir("det");
    let calls = day_calls(S05, 1);
    let one = through_mcp(S05, &calls, &dir.join("x").tap_create());
    let two = through_mcp(S05, &calls, &dir.join("y").tap_create());
    assert_eq!(one.0, two.0);
    assert_eq!(fs::read(&one.1).unwrap(), fs::read(&two.1).unwrap());
    assert_eq!(fs::read(&one.2).unwrap(), fs::read(&two.2).unwrap());
    let a = through_play(S05, &calls, &dir.join("p").tap_create());
    let b = through_play(S05, &calls, &dir.join("q").tap_create());
    assert_eq!(a.0, b.0);
    assert_eq!(fs::read(&a.1).unwrap(), fs::read(&b.1).unwrap());
    assert_eq!(fs::read(&a.2).unwrap(), fs::read(&b.2).unwrap());
    // And the tool list and the usage.
    assert_eq!(
        run(&["play", "--tools", "--game", "x"]).stdout,
        run(&["play", "--tools", "--game", "y"]).stdout
    );
}

trait TapCreate {
    fn tap_create(self) -> PathBuf;
}

impl TapCreate for PathBuf {
    fn tap_create(self) -> PathBuf {
        fs::create_dir_all(&self).expect("make dir");
        self
    }
}

// ---------------------------------------------------------------------
// The recorded games (scenario 9's record).
// ---------------------------------------------------------------------

/// What the transcript's answer says as the text the server gives for it.
fn expected_text(answer: &Value, tool: &str, arguments: &Value) -> (String, bool) {
    if answer["kind"] == "error" {
        let mut text = answer["message"].as_str().unwrap().to_string();
        if let Some(allowed) = answer["allowed"].as_array() {
            let names: Vec<&str> = allowed.iter().map(|a| a.as_str().unwrap()).collect();
            text = format!("{text} (one of: {})", names.join(", "));
        }
        return (text, true);
    }
    let wants_text = tool == "get_briefing" && arguments.get("text") != Some(&json!(false));
    if wants_text {
        return (answer["text"].as_str().unwrap().to_string(), false);
    }
    (serde_json::to_string_pretty(answer).unwrap(), false)
}

/// Replays `game.jsonl` through the MCP server and checks it against the kept
/// transcript and save. `Err` says what differs.
fn check_recorded_game(dir: &Path) -> Result<(), String> {
    let read = |name: &str| fs::read_to_string(dir.join(name)).map_err(|e| format!("{name}: {e}"));
    let game = read("game.jsonl")?;
    let kept_transcript = read("transcript.jsonl")?;
    let kept_save = fs::read(dir.join("save.json")).map_err(|e| format!("save.json: {e}"))?;
    let mut lines = game.lines().filter(|l| !l.trim().is_empty());
    let header: Value = serde_json::from_str(lines.next().ok_or("game.jsonl is empty")?)
        .map_err(|e| format!("the header of game.jsonl: {e}"))?;
    let scenario = header["scenario"]
        .as_str()
        .ok_or("the header names no scenario")?;
    let seat = header["seat"].as_str().ok_or("the header names no seat")?;
    let tmp = std::env::temp_dir().join(format!(
        "flagplot-recorded-{}-{}",
        std::process::id(),
        dir.file_name().and_then(|n| n.to_str()).unwrap_or("game")
    ));
    fs::create_dir_all(&tmp).map_err(|e| e.to_string())?;
    let (save, transcript) = (tmp.join("save.json"), tmp.join("transcript.jsonl"));
    let mut args: Vec<String> = vec![
        "mcp".into(),
        scenario.into(),
        "--seat".into(),
        seat.into(),
        "--save".into(),
        s(&save).into(),
        "--transcript".into(),
        s(&transcript).into(),
    ];
    if let Some(seed) = header["seed"].as_u64() {
        args.extend(["--seed".into(), seed.to_string()]);
    }
    let mut child = Command::new(BIN)
        .args(&args)
        .current_dir(ROOT)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    let mut input = vec![
        json!({"jsonrpc": "2.0", "id": 0, "method": "initialize", "params": {"protocolVersion": "2025-11-25"}}).to_string(),
        json!({"jsonrpc": "2.0", "method": "notifications/initialized"}).to_string(),
    ];
    let mut calls = Vec::new();
    for (i, line) in lines.enumerate() {
        let call: Value =
            serde_json::from_str(line).map_err(|e| format!("game.jsonl line {}: {e}", i + 2))?;
        let tool = call["tool"]
            .as_str()
            .ok_or(format!("game.jsonl line {}: no tool", i + 2))?;
        let arguments = call.get("arguments").cloned().unwrap_or_else(|| json!({}));
        input.push(
            json!({"jsonrpc": "2.0", "id": i + 1, "method": "tools/call", "params": {"name": tool, "arguments": arguments}})
                .to_string(),
        );
        calls.push((tool.to_string(), arguments));
    }
    child
        .stdin
        .take()
        .expect("stdin")
        .write_all((input.join("\n") + "\n").as_bytes())
        .map_err(|e| e.to_string())?;
    let output = child.wait_with_output().map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err(format!(
            "the server failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let answers: Vec<Value> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|l| serde_json::from_str(l).map_err(|e| e.to_string()))
        .collect::<Result<_, _>>()?;
    if answers.len() != calls.len() + 1 {
        return Err(format!(
            "{} answers for {} calls",
            answers.len(),
            calls.len()
        ));
    }
    let kept: Vec<Value> = kept_transcript
        .lines()
        .map(|l| serde_json::from_str(l).map_err(|e| format!("transcript.jsonl: {e}")))
        .collect::<Result<_, _>>()?;
    if kept.len() != calls.len() {
        return Err(format!(
            "the transcript has {} entries for {} calls",
            kept.len(),
            calls.len()
        ));
    }
    for (i, ((tool, arguments), entry)) in calls.iter().zip(&kept).enumerate() {
        let result = &answers[i + 1]["result"];
        let text = result["content"][0]["text"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        let is_error = result["isError"] == json!(true);
        let (expected, expected_error) = expected_text(&entry["answer"], tool, arguments);
        if (text.clone(), is_error) != (expected, expected_error) {
            return Err(format!(
                "call {} ({tool}): the answer differs from the transcript's",
                i + 1
            ));
        }
        if entry["tool"] != json!(tool) {
            return Err(format!(
                "call {}: the transcript names tool {}",
                i + 1,
                entry["tool"]
            ));
        }
    }
    let regenerated = fs::read_to_string(&transcript).map_err(|e| e.to_string())?;
    if regenerated != kept_transcript {
        return Err("the transcript regenerated is not the one kept".to_string());
    }
    if fs::read(&save).map_err(|e| e.to_string())? != kept_save {
        return Err("the saved game regenerated is not the one kept".to_string());
    }
    let replay = run(&["replay", s(&dir.join("save.json"))]);
    if !replay.status.success() {
        return Err(format!(
            "replay of save.json failed: {}",
            String::from_utf8_lossy(&replay.stderr)
        ));
    }
    let _ = fs::remove_dir_all(&tmp);
    Ok(())
}

fn recorded_game_dirs(root: &Path) -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = fs::read_dir(root)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .map(|e| e.path())
                .filter(|p| p.join("game.jsonl").exists())
                .collect()
        })
        .unwrap_or_default();
    dirs.sort();
    dirs
}

#[test]
fn every_recorded_game_replays_to_its_transcript_and_save() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/recorded_games");
    let dirs = recorded_game_dirs(&root);
    if dirs.is_empty() {
        println!(
            "no recorded games under {}: nothing to replay",
            root.display()
        );
    }
    for dir in dirs {
        if let Err(why) = check_recorded_game(&dir) {
            panic!("{}: {why}", dir.display());
        }
        println!("recorded game {} replays", dir.display());
    }
}

#[test]
fn the_recorded_game_harness_passes_a_good_game_and_fails_an_altered_one() {
    let root = temp_dir("recorded");
    let dir = root.join("scenario5-test");
    fs::create_dir_all(&dir).unwrap();
    // Made as a model's games are: with `play`, then exported.
    let game = dir.join("game.jsonl");
    let calls = day_calls(S05, 1);
    for (i, (tool, arguments)) in calls.iter().enumerate() {
        let arguments = arguments.to_string();
        let mut args = vec!["play", "--game", s(&game)];
        if i == 0 {
            args.extend(["--scenario", S05, "--seat", "allied"]);
        }
        args.extend([tool.as_str(), arguments.as_str()]);
        run(&args);
    }
    let exported = run(&[
        "play",
        "--game",
        s(&game),
        "--export-save",
        s(&dir.join("save.json")),
        "--export-transcript",
        s(&dir.join("transcript.jsonl")),
    ]);
    assert!(exported.status.success());
    assert_eq!(recorded_game_dirs(&root), vec![dir.clone()]);
    check_recorded_game(&dir).expect("a recorded game made by play passes");

    // One answer of the transcript altered: it fails.
    let transcript = dir.join("transcript.jsonl");
    let original = fs::read_to_string(&transcript).unwrap();
    let altered = original.replacen("Task Force 16", "Task Force 17", 1);
    assert_ne!(altered, original);
    fs::write(&transcript, &altered).unwrap();
    let why = check_recorded_game(&dir).expect_err("an altered transcript fails");
    assert!(why.contains("differs"), "{why}");
    fs::write(&transcript, &original).unwrap();
    check_recorded_game(&dir).expect("restored, it passes again");

    // The save altered: it fails.
    let save = dir.join("save.json");
    let mut bytes = fs::read(&save).unwrap();
    bytes.push(b' ');
    fs::write(&save, bytes).unwrap();
    assert!(check_recorded_game(&dir).is_err());
}

/// Finding 2 of the outside review of Nov 13, 2026 (prompt 11), ruled in
/// decision 0085: `play --export-save` refuses a game that is not over, so no
/// enemy order leaves it; `--export-transcript` alone stays allowed.
#[test]
fn play_exports_no_enemy_order_from_a_game_that_is_not_over() {
    let dir = temp_dir("review11");
    let (game, save) = (dir.join("g.txt"), dir.join("save.json"));
    let ended = run(&[
        "play",
        "--game",
        s(&game),
        "--scenario",
        S07,
        "--seat",
        "allied",
        "end_day",
        "{}",
    ]);
    assert!(
        ended.status.success(),
        "{}",
        String::from_utf8_lossy(&ended.stderr)
    );
    let answer: Value = serde_json::from_str(&out_text(&ended)).expect("end_day answers JSON");
    assert_eq!(
        answer["game_over"], false,
        "the game must still be on: {answer}"
    );
    let exported = run(&["play", "--game", s(&game), "--export-save", s(&save)]);
    assert_eq!(exported.status.code(), Some(1));
    assert!(exported.stdout.is_empty());
    let why = String::from_utf8_lossy(&exported.stderr).into_owned();
    assert!(why.contains("the game is not over"), "{why}");
    // Given with the transcript, the whole call is refused too.
    let transcript = dir.join("t.jsonl");
    let both = run(&[
        "play",
        "--game",
        s(&game),
        "--export-save",
        s(&save),
        "--export-transcript",
        s(&transcript),
    ]);
    assert_eq!(both.status.code(), Some(1));
    assert!(!transcript.exists(), "a refused call writes nothing");
    // The transcript alone is allowed.
    let alone = run(&[
        "play",
        "--game",
        s(&game),
        "--export-transcript",
        s(&transcript),
    ]);
    assert!(alone.status.success());
    assert!(!save.exists(), "no saved game is written mid-game");
    // No saved game exists, so no Japanese order was exported (the finding).
}
