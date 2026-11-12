//! `sim-cli mcp`: one game as a Model Context Protocol server over standard
//! input and output (decisions 1 to 4 of 0051).
//!
//! A thin wrapper. Every tool call goes to [`Session::call`] for the player's
//! seat and comes back as one text content item. Nothing here reads a clock,
//! a file other than the schemas compiled in, or the network, and nothing but
//! protocol messages is written to standard output.
//!
//! # Revisions
//!
//! The current revision of the specification is 2026-07-28. It has no
//! `initialize` handshake: every request carries `_meta` with the protocol
//! version and the client's capabilities, and `server/discover` stands for the
//! handshake. Earlier revisions (2025-11-25 and before) open with
//! `initialize`. The specification lets a server serve both ("dual-era"), and
//! this one does, choosing by how a request opens:
//!
//! * A request whose `params._meta` carries
//!   `io.modelcontextprotocol/protocolVersion` is served as 2026-07-28: the
//!   version must be one we serve (else error -32022 with `supported` and
//!   `requested`), `io.modelcontextprotocol/clientCapabilities` must be
//!   present (else -32602), results carry `resultType: "complete"` and the
//!   server's identity in `_meta`.
//! * `initialize` selects the legacy handshake: the answer names the client's
//!   version if it is one of 2025-11-25, 2025-06-18, 2025-03-26, 2024-11-05
//!   and otherwise 2025-11-25. After it the other requests need no `_meta`.
//! * Any other request without that `_meta`, before `initialize`, is refused
//!   with -32602 saying both ways to begin.

use std::collections::BTreeMap;

use serde_json::{json, Map, Value};
use sim_agent::{parse_seat, CallError, Session, TOOLS};
use sim_core::state::Side;

pub const MODERN: &str = "2026-07-28";
/// Legacy revisions answered to `initialize`, latest first.
pub const LEGACY: [&str; 4] = ["2025-11-25", "2025-06-18", "2025-03-26", "2024-11-05"];

const PARSE_ERROR: i64 = -32700;
const INVALID_REQUEST: i64 = -32600;
const METHOD_NOT_FOUND: i64 = -32601;
const INVALID_PARAMS: i64 = -32602;
const UNSUPPORTED_VERSION: i64 = -32022;

const META_VERSION: &str = "io.modelcontextprotocol/protocolVersion";
const META_CAPS: &str = "io.modelcontextprotocol/clientCapabilities";
const META_SERVER: &str = "io.modelcontextprotocol/serverInfo";

pub const INSTRUCTIONS: &str = "Flag Plot is an operational wargame of the Pacific War in 1942: you command one side's task forces and air groups by orders, and a staff carries them out on a hex map of the Solomon Islands. The game is played in days; a day has three pulses (night, morning, afternoon) that the game resolves by itself once you end the day, so you give orders for the coming day and then read what happened. You see your own units exactly and the enemy only as contacts, which are reports that can be old or wrong. Work in this order each day: call get_briefing and read it; ask list_forces, list_contacts, list_bases and get_entity for detail; use measure to find distances, routes, passage times and air reach before committing; call issue_orders with your orders for the day (an order is checked at once and answered queued or refused with the reason); then call end_day, and read the new briefing. Repeat until end_day says game_over is true. Use explain with an event number from a briefing to learn why something happened.";

// ---------------------------------------------------------------------
// Tool descriptions and input schemas.
// ---------------------------------------------------------------------

const ISSUE_ORDERS_DESCRIPTION: &str = "Queue orders for your side for the coming day. Each order is {\"unit\": \"task_force:N\" or \"air_group:N\", \"order\": {\"kind\": ..., ...}}; the handles are those the briefing shows. The answer says, for each order in the order given, \"queued\" or \"refused\" with the reason (the engine's own check, made against the state as it is now; a refused order is not queued, the others in the call stand). An order can still be refused when the day begins if the state has changed; the next briefing says so. \"clear\": true empties your queue first, so a call with {\"orders\": [], \"clear\": true} withdraws everything. \"reason\": \"...\" says in your own words why you are giving these orders; give one with every call, since it is kept in the record of the game for whoever reads it afterwards (the game itself never reads it). Orders stand after the day ends until another replaces them. Order kinds for a task force (unit \"task_force:N\"): set_route {waypoints: [[q, r], ...]} (hexes to steam to, in order); set_speed {speed: \"cruise\" or \"full\"} or {knots: \"17\"}; set_mission {mission: transport, surface_combat, carrier, bombardment or escort}; load {cargo: supply or fuel}; unload {}; set_rules_of_engagement {stance: engage or avoid, break_off: decimal, stragglers: keep, detach, leave or retire, keep_share: decimal}; set_home_port {base: N}; detach {ships: [ship numbers]}; set_readiness {readiness: normal or battle}. Order kinds for an air group (unit \"air_group:N\"): set_air_mission {mission: rest, search, naval_strike or combat_air_patrol} or {mission: \"base_strike\", target_base: N}; set_search_arc {from_deg, to_deg, range_nm: decimal}; set_patrol_share {share: decimal}; set_rest_threshold {fatigue: decimal}. Every quantity is a decimal string such as \"0.5\" and every count a whole number; the input schema lists the fields of each kind exactly. Example: {\"orders\": [{\"unit\": \"task_force:1\", \"order\": {\"kind\": \"set_route\", \"waypoints\": [[5, -4], [3, -4]]}}, {\"unit\": \"air_group:1\", \"order\": {\"kind\": \"set_air_mission\", \"mission\": \"combat_air_patrol\"}}], \"reason\": \"Close the anchorage by night and keep fighters over the carrier.\"}. Use it after reading the briefing and before end_day; you cannot order after end_day until the day has been resolved.";

const MEASURE_DESCRIPTION: &str = "Ask the game a question about distance, route, passage time or air reach, answered with the functions the rules themselves use, so you can check a plan before ordering it. Give one question as the arguments; \"question\" names it. A place is {\"q\": 2, \"r\": -4} (a hex), or the handle of one of your own ships, task forces, air groups or bases, or of a contact you hold (which stands for the hex you believe it is in). \"air_distance\" {from, to}: the straight-line air distance in whole miles. \"sea_route\" {from, to}: whether ships can get there, the hexes entered in order and the miles. \"passage\" {task_force: \"task_force:N\", to: place, speed (optional): \"cruise\", \"full\" or knots such as 17}: a forecast of that force's passage, the route, speed, hours, the day, pulse and clock time of arrival and each ship's fuel; it assumes the force leaves at the start of the coming day, meets nobody and keeps its speed, and says \"refused\" if the engine would refuse the route order. \"air_reach\" {air_group: \"air_group:N\", to: [places]}: the aircraft type's radius and, for each place, the distance and whether it is within the radius (the test a strike decision makes). Anything you do not know of (an enemy unit, a number that does not exist, a wrong kind of unit) answers \"not known\". Nothing is changed by asking. Example: {\"question\": \"passage\", \"task_force\": \"task_force:1\", \"to\": {\"q\": 2, \"r\": -4}, \"speed\": \"full\"}.";

fn tool_definitions() -> Vec<(&'static str, String, Value)> {
    let area = json!({
        "type": "object",
        "description": "Only items within radius_nm miles of the hex (q, r).",
        "required": ["q", "r", "radius_nm"],
        "additionalProperties": false,
        "properties": {
            "q": {"type": "integer"},
            "r": {"type": "integer"},
            "radius_nm": {"type": "integer", "minimum": 0}
        }
    });
    let area_only = json!({
        "type": "object",
        "additionalProperties": false,
        "properties": {"area": area}
    });
    let place = json!({
        "description": "A hex {\"q\": 2, \"r\": -4}, or a handle: your own \"ship:N\", \"task_force:N\", \"air_group:N\" or \"base:N\", or a contact \"contact:N\" you hold.",
        "oneOf": [
            {"type": "string"},
            {
                "type": "object",
                "required": ["q", "r"],
                "additionalProperties": false,
                "properties": {"q": {"type": "integer"}, "r": {"type": "integer"}}
            }
        ]
    });
    // Flat on purpose: some clients refuse a top-level oneOf in a tool's
    // input schema. Which fields go with which question is in the description.
    let measure = json!({
        "type": "object",
        "required": ["question"],
        "additionalProperties": false,
        "properties": {
            "question": {
                "enum": ["air_distance", "sea_route", "passage", "air_reach"],
                "description": "air_distance and sea_route take from and to; passage takes task_force, to and optionally speed; air_reach takes air_group and to (a list of places)."
            },
            "from": place,
            "to": {
                "description": "A place; for air_reach, a list of places.",
                "oneOf": [place, {"type": "array", "items": place}]
            },
            "task_force": {"type": "string", "description": "\"task_force:N\" (passage)."},
            "air_group": {"type": "string", "description": "\"air_group:N\" (air_reach)."},
            "speed": {
                "description": "(passage) \"cruise\", \"full\", or knots above zero, 17 or \"17.5\".",
                "oneOf": [
                    {"enum": ["cruise", "full"]},
                    {"type": "number", "exclusiveMinimum": 0},
                    {"type": "string"}
                ]
            }
        }
    });
    vec![
        (
            "get_briefing",
            "Your side's situation at the start of the day: what your orders did yesterday, new and dropped contacts, fights, losses, your task forces, air groups and bases with their handles, arrivals due, and warnings. It answers as readable text by default; call it with {\"text\": false} for the same briefing as data. Read it first every day, and again after end_day. Example arguments: {} or {\"text\": false}.".to_string(),
            json!({
                "type": "object",
                "additionalProperties": false,
                "properties": {
                    "text": {"type": "boolean", "description": "true (the default here) for the briefing as text, false for data."}
                }
            }),
        ),
        (
            "list_forces",
            "One line for each of your task forces: its handle, place, status, mission, speed, destination, readiness, ships afloat and least-fuelled ship. Use it for a quick roll call. Optional {\"area\": {\"q\": 7, \"r\": -5, \"radius_nm\": 120}} keeps only forces within that distance of the hex. Example arguments: {}.".to_string(),
            area_only.clone(),
        ),
        (
            "list_contacts",
            "The enemy contacts your side holds, each with its handle, where you believe it is, how old the report is and how sure. You see the enemy only as contacts: reports that can be old, wrong or missing. Use it before deciding where to search or strike. Optional {\"area\": {\"q\", \"r\", \"radius_nm\"}} keeps only contacts within that distance of the hex. Example arguments: {}.".to_string(),
            area_only.clone(),
        ),
        (
            "list_bases",
            "Your own bases (handle, place, airfield and port size, supply and fuel) and the enemy's bases by number, name, place and holder only: where every base is is on the map, not intelligence. Use it to find a base number for a base_strike or a home port. Optional {\"area\": {\"q\", \"r\", \"radius_nm\"}}. Example arguments: {}.".to_string(),
            area_only,
        ),
        (
            "get_entity",
            "Everything your side knows of one unit or contact: a ship, task force, air group, base or contact, named by its handle. Your own units are known exactly; an enemy unit only as the contact you hold. A handle you do not know of, an enemy unit you hold no contact for or a number that does not exist all answer \"not known\" alike. Example arguments: {\"handle\": \"task_force:1\"}.".to_string(),
            json!({
                "type": "object",
                "required": ["handle"],
                "additionalProperties": false,
                "properties": {
                    "handle": {"type": "string", "description": "Such as \"task_force:3\", \"ship:9\", \"air_group:2\", \"base:1\" or \"contact:4\"."}
                }
            }),
        ),
        ("measure", MEASURE_DESCRIPTION.to_string(), measure),
        (
            "explain",
            "Why something happened: an event as your side may see it and the visible events and orders that caused it, with the figures behind each. Give the number of an event the briefing showed you. An event you were not shown answers \"not known\". Use it to learn why a strike missed, a ship was lost or an order was refused. Example arguments: {\"event\": 42}.".to_string(),
            json!({
                "type": "object",
                "required": ["event"],
                "additionalProperties": false,
                "properties": {
                    "event": {"type": "integer", "minimum": 0, "description": "The number of an event shown in a briefing."}
                }
            }),
        ),
        ("issue_orders", ISSUE_ORDERS_DESCRIPTION.to_string(), issue_orders_schema()),
        (
            "end_day",
            "Say you have finished giving orders for the coming day. The game then resolves the day (night, morning and afternoon pulses): your queued orders and the other side's are carried out, and you may read a new briefing. The answer says whether the day was resolved, how many days have been resolved and whether the game is over; when game_over is true there is nothing more to order, though you can still read. Call it once per day after issue_orders; orders cannot be given between end_day and the next day. Example arguments: {}.".to_string(),
            json!({"type": "object", "additionalProperties": false}),
        ),
    ]
}

// The schema files, compiled in. `issue_orders`' input schema is the
// arguments schema with every `$ref` replaced by what it points at, because a
// client cannot be expected to fetch `order.schema.json` or resolve
// `defs.schema.json#/$defs/dec`.
const ARGUMENTS_SCHEMA: &str =
    include_str!("../../sim-agent/schemas/issue_orders_arguments.schema.json");
const ORDER_SCHEMA: &str = include_str!("../../sim-agent/schemas/order.schema.json");
const DEFS_SCHEMA: &str = include_str!("../../sim-agent/schemas/defs.schema.json");

fn issue_orders_schema() -> Value {
    let docs: BTreeMap<&str, Value> = [
        (
            "issue_orders_arguments.schema.json",
            serde_json::from_str(ARGUMENTS_SCHEMA).expect("the arguments schema is JSON"),
        ),
        (
            "order.schema.json",
            serde_json::from_str(ORDER_SCHEMA).expect("the order schema is JSON"),
        ),
        (
            "defs.schema.json",
            serde_json::from_str(DEFS_SCHEMA).expect("the defs schema is JSON"),
        ),
    ]
    .into_iter()
    .collect();
    let root = &docs["issue_orders_arguments.schema.json"];
    let mut out = inline(root, "issue_orders_arguments.schema.json", &docs, 0);
    if let Some(map) = out.as_object_mut() {
        for key in ["$schema", "$id", "$defs"] {
            map.remove(key);
        }
    }
    out
}

/// Replaces every `$ref` of `value` (a part of the document `doc`) by its
/// target, resolved against the files compiled in.
fn inline(value: &Value, doc: &str, docs: &BTreeMap<&str, Value>, depth: u32) -> Value {
    assert!(depth < 32, "the schema files refer to themselves");
    match value {
        Value::Object(map) => {
            if let Some(Value::String(target)) = map.get("$ref") {
                let (file, pointer) = target.split_once('#').unwrap_or((target, ""));
                let file = if file.is_empty() { doc } else { file };
                let root = docs
                    .get(file)
                    .unwrap_or_else(|| panic!("the schema {file} is not compiled in"));
                let part = if pointer.is_empty() {
                    root
                } else {
                    root.pointer(pointer)
                        .unwrap_or_else(|| panic!("{file} has no {pointer}"))
                };
                let mut resolved = inline(part, file, docs, depth + 1);
                if pointer.is_empty() {
                    if let Some(m) = resolved.as_object_mut() {
                        for key in ["$schema", "$id", "$defs"] {
                            m.remove(key);
                        }
                    }
                }
                return resolved;
            }
            let mut out = Map::new();
            for (key, v) in map {
                out.insert(key.clone(), inline(v, doc, docs, depth));
            }
            Value::Object(out)
        }
        Value::Array(items) => {
            Value::Array(items.iter().map(|v| inline(v, doc, docs, depth)).collect())
        }
        other => other.clone(),
    }
}

/// The tools as `tools/list` gives them, in the order of [`TOOLS`] (which is
/// the interface's own order; it never varies).
pub fn tool_list() -> Vec<Value> {
    tool_definitions()
        .into_iter()
        .map(|(name, description, schema)| {
            json!({"name": name, "description": description, "inputSchema": schema})
        })
        .collect()
}

// ---------------------------------------------------------------------
// The server.
// ---------------------------------------------------------------------

/// One game, one seat for the player, and the protocol state: whether a legacy
/// `initialize` has been seen.
pub struct Server {
    pub session: Session,
    side: Side,
    initialized: bool,
}

fn rpc_error(id: Value, code: i64, message: &str, data: Option<Value>) -> String {
    let mut error = json!({"code": code, "message": message});
    if let Some(data) = data {
        error["data"] = data;
    }
    serde_json::to_string(&json!({"jsonrpc": "2.0", "id": id, "error": error}))
        .expect("an answer serialises")
}

fn rpc_result(id: Value, result: Value) -> String {
    serde_json::to_string(&json!({"jsonrpc": "2.0", "id": id, "result": result}))
        .expect("an answer serialises")
}

fn server_info() -> Value {
    json!({"name": "flag-plot", "title": "Flag Plot", "version": env!("CARGO_PKG_VERSION")})
}

fn tool_text(text: String, is_error: bool) -> Value {
    json!({"content": [{"type": "text", "text": text}], "isError": is_error})
}

impl Server {
    pub fn new(session: Session, side: Side) -> Server {
        Server {
            session,
            side,
            initialized: false,
        }
    }

    /// The answer to one line of input: `None` for a notification (and for a
    /// response, which a server has no use for), else one line of output.
    pub fn handle_line(&mut self, line: &str) -> Option<String> {
        let message: Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(e) => {
                return Some(rpc_error(
                    Value::Null,
                    PARSE_ERROR,
                    &format!("parse error: {e}"),
                    None,
                ))
            }
        };
        let Some(object) = message.as_object() else {
            return Some(rpc_error(
                Value::Null,
                INVALID_REQUEST,
                "a message is one JSON object (batches are not accepted)",
                None,
            ));
        };
        let id = object.get("id").cloned();
        if object.get("jsonrpc") != Some(&json!("2.0")) {
            return Some(rpc_error(
                id.unwrap_or(Value::Null),
                INVALID_REQUEST,
                "\"jsonrpc\" must be \"2.0\"",
                None,
            ));
        }
        let Some(method) = object.get("method").and_then(Value::as_str) else {
            // A response from the client: nothing is waiting for one.
            if object.contains_key("result") || object.contains_key("error") {
                return None;
            }
            return Some(rpc_error(
                id.unwrap_or(Value::Null),
                INVALID_REQUEST,
                "\"method\" is needed",
                None,
            ));
        };
        let Some(id) = id else {
            // A notification: no answer, whatever it says.
            return None;
        };
        if !(id.is_string() || id.is_i64() || id.is_u64()) {
            return Some(rpc_error(
                Value::Null,
                INVALID_REQUEST,
                "\"id\" must be a string or an integer",
                None,
            ));
        }
        let params = object.get("params").cloned().unwrap_or(Value::Null);
        if !(params.is_null() || params.is_object()) {
            return Some(rpc_error(
                id,
                INVALID_PARAMS,
                "\"params\" must be an object",
                None,
            ));
        }
        Some(match self.request(method, &params) {
            Ok(result) => rpc_result(id, result),
            Err((code, message, data)) => rpc_error(id, code, &message, data),
        })
    }

    #[allow(clippy::type_complexity)]
    fn request(
        &mut self,
        method: &str,
        params: &Value,
    ) -> Result<Value, (i64, String, Option<Value>)> {
        let err = |code: i64, message: &str| Err((code, message.to_string(), None));
        let meta = params.get("_meta");
        let modern = meta.and_then(|m| m.get(META_VERSION));
        let modern = match modern {
            None => false,
            Some(Value::String(version)) if version == MODERN => {
                if meta.and_then(|m| m.get(META_CAPS)).is_none() {
                    return err(
                        INVALID_PARAMS,
                        "_meta needs io.modelcontextprotocol/clientCapabilities",
                    );
                }
                true
            }
            Some(other) => {
                let mut supported = vec![MODERN];
                supported.extend(LEGACY);
                return Err((
                    UNSUPPORTED_VERSION,
                    "Unsupported protocol version".to_string(),
                    Some(json!({"supported": supported, "requested": other})),
                ));
            }
        };
        // Results of the modern revision say they are complete and who we are.
        let finish = |mut result: Value| {
            if modern {
                result["resultType"] = json!("complete");
                result["_meta"] = json!({ META_SERVER: server_info() });
            }
            Ok(result)
        };
        match method {
            "initialize" => {
                let asked = params.get("protocolVersion").and_then(Value::as_str);
                let version = match asked {
                    Some(v) if LEGACY.contains(&v) => v,
                    _ => LEGACY[0],
                };
                self.initialized = true;
                Ok(json!({
                    "protocolVersion": version,
                    "capabilities": {"tools": {"listChanged": false}},
                    "serverInfo": server_info(),
                    "instructions": INSTRUCTIONS,
                }))
            }
            "ping" => finish(json!({})),
            "server/discover" => {
                let mut supported = vec![MODERN];
                supported.extend(LEGACY);
                let mut result = json!({
                    "supportedVersions": supported,
                    "capabilities": {"tools": {"listChanged": false}},
                    "instructions": INSTRUCTIONS,
                });
                result["resultType"] = json!("complete");
                result["_meta"] = json!({ META_SERVER: server_info() });
                Ok(result)
            }
            "tools/list" | "tools/call" => {
                if !modern && !self.initialized {
                    return err(
                        INVALID_PARAMS,
                        "send initialize first, or put io.modelcontextprotocol/protocolVersion (2026-07-28) and io.modelcontextprotocol/clientCapabilities in params._meta",
                    );
                }
                if method == "tools/list" {
                    finish(json!({"tools": tool_list()}))
                } else {
                    let result = self.call_tool(params)?;
                    finish(result)
                }
            }
            other => Err((METHOD_NOT_FOUND, format!("method not found: {other}"), None)),
        }
    }

    #[allow(clippy::type_complexity)]
    fn call_tool(&mut self, params: &Value) -> Result<Value, (i64, String, Option<Value>)> {
        let bad = |message: String| Err((INVALID_PARAMS, message, None));
        let Some(name) = params.get("name").and_then(Value::as_str) else {
            return bad("\"name\" is needed: the tool to call".to_string());
        };
        if !TOOLS.contains(&name) {
            return bad(format!("Unknown tool: {name}"));
        }
        let mut arguments = match params.get("arguments") {
            None | Some(Value::Null) => json!({}),
            Some(value @ Value::Object(_)) => value.clone(),
            Some(_) => return bad("\"arguments\" must be an object".to_string()),
        };
        // The briefing is text unless asked for data (decision 3).
        let mut as_text = false;
        if name == "get_briefing" {
            let map = arguments.as_object_mut().expect("arguments is an object");
            match map.get("text") {
                None | Some(Value::Null) => {
                    map.insert("text".to_string(), json!(true));
                    as_text = true;
                }
                Some(Value::Bool(b)) => as_text = *b,
                Some(_) => {}
            }
        }
        Ok(match self.session.call(self.side, name, &arguments) {
            Ok(answer) => {
                let text = match (as_text, answer.get("text").and_then(Value::as_str)) {
                    (true, Some(text)) => text.to_string(),
                    _ => serde_json::to_string_pretty(&answer).expect("an answer serialises"),
                };
                tool_text(text, false)
            }
            Err(error) => tool_text(error_text(&error), true),
        })
    }
}

fn error_text(error: &CallError) -> String {
    if error.allowed.is_empty() {
        error.message.clone()
    } else {
        format!("{} (one of: {})", error.message, error.allowed.join(", "))
    }
}

// ---------------------------------------------------------------------
// The command.
// ---------------------------------------------------------------------

struct Options<'a> {
    scenario: &'a str,
    side: Side,
    seed: Option<u64>,
    save: Option<&'a str>,
    transcript: Option<&'a str>,
}

fn parse<'a>(args: &'a [String], usage: &str) -> Result<Options<'a>, String> {
    let mut scenario = None;
    let mut side = None;
    let mut seed = None;
    let mut save = None;
    let mut transcript = None;
    let mut words = args.iter().map(String::as_str);
    while let Some(word) = words.next() {
        let mut value = |what: &str| {
            words
                .next()
                .ok_or_else(|| format!("{what} needs a value\n\n{usage}"))
        };
        match word {
            "--seat" => {
                let v = value("--seat")?;
                side = Some(
                    parse_seat(v)
                        .ok_or_else(|| format!("--seat \"{v}\" is not allied or japanese"))?,
                );
            }
            "--seed" => {
                let v = value("--seed")?;
                seed = Some(
                    v.parse::<u64>()
                        .map_err(|_| format!("--seed \"{v}\" is not a whole number"))?,
                );
            }
            "--save" => save = Some(value("--save")?),
            "--transcript" => transcript = Some(value("--transcript")?),
            flag if flag.starts_with('-') => {
                return Err(format!("unknown option \"{flag}\"\n\n{usage}"))
            }
            path if scenario.is_none() => scenario = Some(path),
            extra => return Err(format!("unexpected argument \"{extra}\"\n\n{usage}")),
        }
    }
    Ok(Options {
        scenario: scenario.ok_or_else(|| format!("no scenario given\n\n{usage}"))?,
        side: side
            .ok_or_else(|| format!("--seat allied or --seat japanese is needed\n\n{usage}"))?,
        seed,
        save,
        transcript,
    })
}

pub fn command(args: &[String], usage: &str) -> Result<(), String> {
    use std::io::{BufRead, Write};
    let options = parse(args, usage)?;
    let seats = BTreeMap::from([(options.side, sim_agent::SeatKind::Player)]);
    let (session, kept) = crate::serve::open(options.scenario, options.seed, seats)?;
    let mut server = Server::new(session, options.side);
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    for line in stdin.lock().lines() {
        let line = line.map_err(|e| format!("standard input: {e}"))?;
        if line.trim().is_empty() {
            continue;
        }
        if let Some(answer) = server.handle_line(&line) {
            writeln!(out, "{answer}")
                .and_then(|()| out.flush())
                .map_err(|e| format!("standard output: {e}"))?;
        }
    }
    kept.finish(&server.session, options.save, options.transcript)
}
