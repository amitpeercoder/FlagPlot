//! A game held open for play (decisions 1 to 12 of 0050).
//!
//! This is the one module of the library that holds a [`World`]. Everything a
//! seat reads is made by the functions of the other modules from the seat's
//! [`sim_core::view::SideView`]; this module only asks the world for that
//! view, for the engine's own check of an order, and for the day to be
//! resolved. It never reads the truth, the full event log or the other side's
//! state, and no tool answer is built here from anything but a view. The
//! source lint of `tests/leaks.rs` allows the word `World` in this file alone
//! and holds this file to a shorter list of what it may call.

use std::collections::BTreeMap;

use serde_json::{json, Value};
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, LandUnitId, ShipId, TaskForceId};
use sim_core::orders::{OrderKind, OrderRequest, Origin, RejectReason, TaskForceOrder};
use sim_core::state::Side;
use sim_core::World;

use crate::answer::{FORMAT, VERSION};
use crate::briefing::briefing_for_game;
use crate::common::{map_unit_tokens, Ctx};
use crate::explain::explain;
use crate::measure::{measure_in, MeasureContext, Question};
use crate::objectives::result_part;
use crate::orders::{is_orderable_unit, land_unit_number, order_from_json};
use crate::queries::{get_entity, list_bases, list_contacts, list_forces};
use crate::record::{seat_name, CallError, Seat, SeatKind, TranscriptEntry, TOOLS};

/// The umpire's number given to a unit the side named that is none of its
/// own: no unit has it.
const NO_SUCH_UNIT: u32 = u32::MAX;

/// The first umpire's number given to a ship a detach order names that is not
/// the side's own. No ship has one of these either.
const FIRST_STAND_IN: u32 = 4_000_000_000;

/// The body of an order with the side's numbers of ships (in a detach) turned
/// into the umpire's. A number that names none of the side's ships becomes a
/// stand-in, kept in `stand_ins` (stand-in to the number written) so a
/// refusal can show the number the side wrote.
fn unit_numbers_to_ids(ctx: &Ctx<'_>, body: &Value, stand_ins: &mut BTreeMap<u32, u32>) -> Value {
    let mut body = body.clone();
    // A load with a land unit names it by the side's number (decision 0086,
    // 3); a number that names none of the side's units becomes one no unit
    // has, so an enemy unit and one that does not exist are refused alike.
    if body.get("kind").and_then(Value::as_str) == Some("load") {
        if let Some(number) = body
            .get("unit")
            .and_then(Value::as_str)
            .and_then(land_unit_number)
        {
            let raw = ctx
                .own_land_unit(number)
                .map_or(NO_SUCH_UNIT, |unit| unit.id.raw());
            body["unit"] = json!(format!("land_unit:{raw}"));
        }
        return body;
    }
    // A prepare_landing roster names its units by the side's numbers too;
    // one that names none of the side's units becomes the number no unit
    // has, so an enemy unit and one that does not exist are refused alike
    // (decision 0095, 1).
    if body.get("kind").and_then(Value::as_str) == Some("prepare_landing") {
        if let Some(Value::Array(roster)) = body.get_mut("roster") {
            for item in roster.iter_mut() {
                let Some(number) = item.as_str().and_then(land_unit_number) else {
                    continue;
                };
                let raw = ctx
                    .own_land_unit(number)
                    .map_or(NO_SUCH_UNIT, |unit| unit.id.raw());
                *item = json!(format!("land_unit:{raw}"));
            }
        }
        return body;
    }
    if body.get("kind").and_then(Value::as_str) != Some("detach") {
        return body;
    }
    if let Some(Value::Array(ships)) = body.get_mut("ships") {
        for ship in ships.iter_mut() {
            let Some(number) = ship.as_u64().and_then(|n| u32::try_from(n).ok()) else {
                continue;
            };
            let id = match ctx.own_ship(number) {
                Some(own) => own.id.raw(),
                None => {
                    let existing = stand_ins.iter().find(|(_, n)| **n == number);
                    match existing {
                        Some((id, _)) => *id,
                        None => {
                            let id = FIRST_STAND_IN
                                + u32::try_from(stand_ins.len()).expect("few ships in an order");
                            stand_ins.insert(id, number);
                            id
                        }
                    }
                }
            };
            *ship = json!(id);
        }
    }
    body
}

/// The one refusal for an order to a unit the side does not own, whether the
/// unit is the enemy's or does not exist (decision 5 of 0050). The same text
/// answers a unit that is not written as one an order can be for.
pub const NOT_KNOWN: &str = "not known";

struct SeatState {
    seat: Seat,
    /// The orders queued for the coming day, in the order given.
    queued: Vec<OrderRequest>,
    /// The seat has said it has finished its orders for the coming day.
    ended: bool,
}

/// A game for play: the world, who holds each seat, the days it runs, and what
/// is kept of it (the orders as handed in, the transcript).
pub struct Session {
    world: World,
    scenario: String,
    days: u32,
    seats: BTreeMap<Side, SeatState>,
    handed_in: BTreeMap<u32, Vec<(Side, OrderRequest)>>,
    transcript: Vec<TranscriptEntry>,
}

/// The arguments of a call as a map: an object, or nothing.
fn args_of(arguments: &Value) -> Result<serde_json::Map<String, Value>, CallError> {
    match arguments {
        Value::Object(map) => Ok(map.clone()),
        Value::Null => Ok(serde_json::Map::new()),
        _ => Err(CallError::new("the arguments must be a JSON object")),
    }
}

impl Session {
    /// A session over a game at its starting state, for `days` days.
    /// `scenario` is the name the briefing's heading carries.
    pub fn new(world: World, scenario: &str, days: u32, allied: Seat, japanese: Seat) -> Session {
        let mut seats = BTreeMap::new();
        for (side, seat) in [(Side::Allied, allied), (Side::Japanese, japanese)] {
            seats.insert(
                side,
                SeatState {
                    seat,
                    queued: Vec::new(),
                    ended: false,
                },
            );
        }
        Session {
            world,
            scenario: scenario.to_string(),
            days,
            seats,
            handed_in: BTreeMap::new(),
            transcript: Vec::new(),
        }
    }

    /// Who holds a seat.
    pub fn seat_kind(&self, side: Side) -> SeatKind {
        self.seats[&side].seat.kind()
    }

    /// How many days have been resolved.
    pub fn day(&self) -> u32 {
        self.world.day()
    }

    /// How many days the game runs.
    pub fn days(&self) -> u32 {
        self.days
    }

    pub fn is_over(&self) -> bool {
        self.world.day() >= self.days
    }

    /// The state hash at the end of each resolved day.
    pub fn day_hashes(&self) -> &[u64] {
        self.world.day_hashes()
    }

    /// The hash of the state as it stands now. For the runner and tests; no
    /// tool gives it.
    pub fn state_hash(&self) -> u64 {
        self.world.state_hash()
    }

    /// The orders as they were handed in to the engine, by day and, within a
    /// day, side by side (the allied seat first), each side's in the order the
    /// seat gave them. A saved game made from these replays with no player.
    pub fn handed_in(&self) -> &BTreeMap<u32, Vec<(Side, OrderRequest)>> {
        &self.handed_in
    }

    /// Every call a player's seat made, in order.
    pub fn transcript(&self) -> &[TranscriptEntry] {
        &self.transcript
    }

    /// The game itself, for the runner and the tests once play is done. The
    /// session gives no way to it while the game is on, and no tool reaches it.
    pub fn into_world(self) -> World {
        self.world
    }

    /// A call from a seat: one tool, its arguments, an answer. A seat held by
    /// the script cannot be called. A call that is refused as a whole
    /// changes nothing in the game; it is still entered in the transcript,
    /// because it is something the player was told.
    pub fn call(&mut self, side: Side, tool: &str, arguments: &Value) -> Result<Value, CallError> {
        if self.seats[&side].seat.kind() != SeatKind::Player {
            let players: Vec<&str> = Side::ALL
                .into_iter()
                .filter(|s| self.seats[s].seat.kind() == SeatKind::Player)
                .map(seat_name)
                .collect();
            return Err(CallError::naming(
                format!(
                    "the {} seat is held by the script and cannot be called",
                    seat_name(side)
                ),
                &players,
            ));
        }
        let after_day = self.world.day();
        let outcome = self.dispatch(side, tool, arguments);
        let answer = match &outcome {
            Ok(value) => value.clone(),
            Err(error) => error.to_json(),
        };
        let seq = u32::try_from(self.transcript.len() + 1).expect("fewer than 2^32 calls");
        self.transcript.push(TranscriptEntry {
            seq,
            after_day,
            seat: side,
            tool: tool.to_string(),
            arguments: arguments.clone(),
            answer,
        });
        outcome
    }

    fn dispatch(&mut self, side: Side, tool: &str, arguments: &Value) -> Result<Value, CallError> {
        let args = args_of(arguments)?;
        match tool {
            "get_briefing" => {
                let text = match args.get("text") {
                    None | Some(Value::Null) => false,
                    Some(Value::Bool(b)) => *b,
                    Some(_) => return Err(CallError::new("\"text\" must be true or false")),
                };
                let view = self.world.side_view(side);
                let mut briefing = briefing_for_game(&view, &self.scenario, Some(self.days));
                // The final briefing carries the umpire's result (decision 8
                // of 0054): the game is over, so it is no longer a secret.
                if let Some(result) = self.final_result() {
                    briefing = briefing.with_result(&view, &result);
                }
                if text {
                    Ok(json!({
                        "format": FORMAT,
                        "version": VERSION,
                        "kind": "briefing_text",
                        "text": briefing.to_text(),
                    }))
                } else {
                    to_value(&briefing)
                }
            }
            "list_forces" => {
                let area = area_of(&args)?;
                to_value(&list_forces(&self.world.side_view(side), area))
            }
            "list_bases" => {
                let area = area_of(&args)?;
                to_value(&list_bases(&self.world.side_view(side), area))
            }
            "list_contacts" => {
                let area = area_of(&args)?;
                to_value(&list_contacts(&self.world.side_view(side), area))
            }
            "get_entity" => {
                let handle = args.get("handle").and_then(Value::as_str).ok_or_else(|| {
                    CallError::new("\"handle\" is needed, as text such as \"task_force:3\"")
                })?;
                to_value(&get_entity(&self.world.side_view(side), handle))
            }
            "measure" => {
                let question = Question::from_json(&arguments.to_string())
                    .map_err(|e| CallError::new(format!("not a question to measure: {e}")))?;
                // The engine's own check of a yard order, and the last day,
                // for the yard question.
                let world = &self.world;
                let check = |id: TaskForceId| {
                    world.check_order(
                        side,
                        &OrderRequest {
                            kind: OrderKind::TaskForce {
                                target: id,
                                order: TaskForceOrder::SendToYard,
                            },
                            origin: Origin::PLAYER,
                        },
                    )
                };
                let context = MeasureContext {
                    last_day: Some(self.days),
                    yard_check: Some(&check),
                };
                to_value(&measure_in(
                    &self.world.side_view(side),
                    &question,
                    &context,
                ))
            }
            "explain" => {
                let event = args
                    .get("event")
                    .and_then(Value::as_u64)
                    .and_then(|n| u32::try_from(n).ok())
                    .ok_or_else(|| {
                        CallError::new(
                            "\"event\" is needed, as the number of an event you were shown",
                        )
                    })?;
                to_value(&explain(&self.world.side_view(side), event))
            }
            "issue_orders" => self.issue_orders(side, &args),
            "end_day" => self.end_day(side),
            other => Err(CallError::naming(
                format!("\"{other}\" is not a tool"),
                &TOOLS,
            )),
        }
    }

    // -----------------------------------------------------------------
    // Orders (decisions 3 to 7).
    // -----------------------------------------------------------------

    fn issue_orders(
        &mut self,
        side: Side,
        args: &serde_json::Map<String, Value>,
    ) -> Result<Value, CallError> {
        if self.is_over() {
            return Err(CallError::new("the game is over"));
        }
        if self.seats[&side].ended {
            return Err(CallError::new("the day's orders are closed"));
        }
        // The whole call is checked before anything is changed.
        let entries = args
            .get("orders")
            .and_then(Value::as_array)
            .ok_or_else(|| CallError::new("\"orders\" is needed, as a list of orders"))?;
        let clear = match args.get("clear") {
            None | Some(Value::Null) => false,
            Some(Value::Bool(b)) => *b,
            Some(_) => return Err(CallError::new("\"clear\" must be true or false")),
        };
        // A reason is for the reader of the transcript. The game never reads
        // it (decision 0051, as built), but it must be text.
        match args.get("reason") {
            None | Some(Value::Null) | Some(Value::String(_)) => {}
            Some(_) => return Err(CallError::new("\"reason\" must be text")),
        }
        if clear {
            self.seats
                .get_mut(&side)
                .expect("both seats exist")
                .queued
                .clear();
        }
        // Each order is judged against the state as it is now, which queuing
        // the others does not change, so the judging is done first.
        let judged: Vec<Result<OrderRequest, String>> = {
            let ctx = Ctx::new(self.world.side_view(side));
            entries
                .iter()
                .map(|entry| self.judge(side, &ctx, entry))
                .collect()
        };
        let mut results = Vec::new();
        for (index, judgement) in judged.into_iter().enumerate() {
            match judgement {
                Ok(request) => {
                    // A yard order's acceptance states the day of return,
                    // worked by the function the order itself uses on the
                    // state as it stands (decision 0069).
                    let mut result = json!({ "index": index, "status": "queued" });
                    if let OrderKind::TaskForce {
                        target,
                        order: TaskForceOrder::SendToYard,
                    } = &request.kind
                    {
                        if let Some(plan) = self.world.side_view(side).yard_plan(*target) {
                            let day = plan.return_day(self.world.day() + 1);
                            result["day_of_return"] = json!(day);
                            result["returns"] = json!(day <= self.days);
                        }
                    }
                    self.seats
                        .get_mut(&side)
                        .expect("both seats exist")
                        .queued
                        .push(request);
                    results.push(result);
                }
                Err(reason) => {
                    results.push(json!({ "index": index, "status": "refused", "reason": reason }));
                }
            }
        }
        let refused = results.iter().filter(|r| r["status"] == "refused").count();
        let accepted = results.len() - refused;
        let queued = self.seats[&side].queued.len();
        let summary = format!(
            "In this call {accepted} of {} orders queued and {refused} refused. A refused order is not queued and the others in the call stand. {queued} {} now queued for day {}.",
            results.len(),
            if queued == 1 { "order is" } else { "orders are" },
            self.world.day() + 1
        );
        Ok(json!({
            "format": FORMAT,
            "version": VERSION,
            "kind": "issue_orders",
            "for_day": self.world.day() + 1,
            "cleared": clear,
            "results": results,
            "queued_in_call": accepted,
            "refused_in_call": refused,
            "queued": queued,
            "summary": summary,
        }))
    }

    /// Decodes one order and asks the engine's own check, against the state as
    /// it is now. `Err` is the reason to show. A refusal for a unit the side
    /// does not own is the one text [`NOT_KNOWN`], whatever the engine's
    /// reason: it is the engine's `NotYourUnit` for an enemy unit and for a
    /// number that is nothing, and a unit not written as an orderable one is
    /// given the same.
    fn judge(&self, side: Side, ctx: &Ctx<'_>, entry: &Value) -> Result<OrderRequest, String> {
        let unit = entry
            .get("unit")
            .and_then(Value::as_str)
            .ok_or_else(|| "malformed order: \"unit\" is missing or not text".to_string())?;
        if !is_orderable_unit(unit) {
            return Err(NOT_KNOWN.to_string());
        }
        let body = entry
            .get("order")
            .ok_or_else(|| "malformed order: \"order\" is missing".to_string())?;
        // The unit and the ships of a detach are named by the side's numbers
        // (decision 0086); the engine is asked in the umpire's. A number
        // that names no unit of the side becomes a number no unit has, so a
        // unit of the enemy and one that does not exist are refused alike.
        let (kind_of_unit, number) = unit
            .split_once(':')
            .expect("an orderable unit is kind:number");
        let number: u32 = number.parse().expect("an orderable unit has a number");
        let raw = match kind_of_unit {
            "task_force" => ctx.own_force(number).map(|f| f.id.raw()),
            "land_unit" => ctx.own_land_unit(number).map(|u| u.id.raw()),
            _ => ctx.own_group(number).map(|g| g.id.raw()),
        }
        .unwrap_or(NO_SUCH_UNIT);
        let mut stand_ins: BTreeMap<u32, u32> = BTreeMap::new();
        let body = unit_numbers_to_ids(ctx, body, &mut stand_ins);
        let kind = order_from_json(&format!("{kind_of_unit}:{raw}"), &body)
            .map_err(|e| format!("malformed order: {e}"))?;
        let request = OrderRequest {
            kind,
            origin: Origin::PLAYER,
        };
        match self.world.check_order(side, &request) {
            Ok(()) => Ok(request),
            Err(RejectReason::NotYourUnit) => Err(NOT_KNOWN.to_string()),
            // A posture order for a land unit that is not the side's own
            // (an enemy's, or one that does not exist) is refused alike.
            Err(RejectReason::NotYourLandUnit)
                if matches!(request.kind, OrderKind::LandUnit { .. }) =>
            {
                Err(NOT_KNOWN.to_string())
            }
            // The engine's words may name a ship by its umpire's number: show
            // the side's, or the number the side wrote for a stand-in.
            Err(reason) => Err(map_unit_tokens(
                &crate::words::refusal(&reason),
                |kind, raw| match kind {
                    "ship" => stand_ins
                        .get(&raw)
                        .copied()
                        .or_else(|| ctx.ship_number(ShipId::new(raw))),
                    "task_force" => ctx.force_number(TaskForceId::new(raw)),
                    "land_unit" => ctx.land_unit_number(LandUnitId::new(raw)),
                    _ => ctx.group_number(AirGroupId::new(raw)),
                },
            )),
        }
    }

    // -----------------------------------------------------------------
    // Time (decisions 8 to 10).
    // -----------------------------------------------------------------

    fn end_day(&mut self, side: Side) -> Result<Value, CallError> {
        let mut resolved = false;
        if !self.is_over() {
            self.seats.get_mut(&side).expect("both seats exist").ended = true;
            let waiting = self
                .seats
                .values()
                .any(|s| s.seat.kind() == SeatKind::Player && !s.ended);
            if !waiting {
                self.resolve()?;
                resolved = true;
            }
        }
        // The answer that reports the game over carries the umpire's result
        // for both sides, when the scenario has objectives (decision 8 of
        // 0054). Otherwise it is null.
        let result = match self.final_result() {
            Some(result) => to_value(&result_part(&self.world.side_view(side), &result))?,
            None => Value::Null,
        };
        Ok(json!({
            "format": FORMAT,
            "version": VERSION,
            "kind": "end_day",
            "resolved": resolved,
            "day": self.world.day(),
            "game_over": self.is_over(),
            "result": result,
        }))
    }

    /// The umpire's result, once the game is over and only then, if the
    /// scenario has objectives.
    fn final_result(&self) -> Option<sim_core::objectives::GameResult> {
        if self.is_over() {
            self.world.result()
        } else {
            None
        }
    }

    /// Hands in every seat's orders for the coming day through the engine's
    /// own `submit_orders` and resolves the day.
    fn resolve(&mut self) -> Result<(), CallError> {
        let day = self.world.day() + 1;
        for side in Side::ALL {
            let state = self.seats.get_mut(&side).expect("both seats exist");
            let orders = match &mut state.seat {
                Seat::Player => std::mem::take(&mut state.queued),
                Seat::Script(script) => script.remove(&day).unwrap_or_default(),
            };
            state.ended = false;
            for request in &orders {
                self.handed_in
                    .entry(day)
                    .or_default()
                    .push((side, request.clone()));
            }
            self.world.submit_orders(side, orders);
        }
        self.world.resolve_day().map(|_| ()).map_err(|e| {
            // The engine's own words may name the other side's units: they go
            // to the umpire's stderr, and the caller is told one fixed thing.
            eprintln!("operator: the day could not be resolved: {e}");
            CallError::new(
                "the day could not be resolved: the game has stopped on an internal error; the operator has the details",
            )
        })
    }
}

fn to_value<T: serde::Serialize>(answer: &T) -> Result<Value, CallError> {
    serde_json::to_value(answer).map_err(|e| CallError::new(e.to_string()))
}

/// The optional `area` of a list: `{"q": .., "r": .., "radius_nm": ..}`.
fn area_of(args: &serde_json::Map<String, Value>) -> Result<Option<(Hex, u32)>, CallError> {
    let bad = || {
        CallError::new("\"area\" must be {\"q\": whole number, \"r\": whole number, \"radius_nm\": whole number}")
    };
    let Some(area) = args.get("area") else {
        return Ok(None);
    };
    if area.is_null() {
        return Ok(None);
    }
    let get = |key: &str| area.get(key).and_then(Value::as_i64);
    let q = get("q")
        .and_then(|n| i32::try_from(n).ok())
        .ok_or_else(bad)?;
    let r = get("r")
        .and_then(|n| i32::try_from(n).ok())
        .ok_or_else(bad)?;
    let radius = get("radius_nm")
        .and_then(|n| u32::try_from(n).ok())
        .ok_or_else(bad)?;
    Ok(Some((Hex::new(q, r), radius)))
}
