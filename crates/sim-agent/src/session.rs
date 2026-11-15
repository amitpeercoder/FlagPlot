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
use sim_core::command::{CommandKind, Staff, StaffDecisions, Staffs};
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, LandUnitId, ShipId, TaskForceId};
use sim_core::orders::{OrderKind, OrderRequest, Origin, RejectReason, TaskForceOrder};
use sim_core::state::Side;
use sim_core::world::MADE_IDS_FLOOR;
use sim_core::World;

use crate::answer::{FORMAT, VERSION};
use crate::briefing::briefing_for_game;
use crate::common::{map_unit_tokens, Ctx};
use crate::explain::explain;
use crate::measure::{measure_in, MeasureContext, Question};
use crate::objectives::result_part;
use crate::orders::{is_orderable_unit, land_unit_number, order_from_json};
use crate::queries::{get_entity, list_bases, list_contacts, list_forces};
use crate::record::{seat_name, CallError, Seat, SeatKind, TranscriptEntry, COMMAND_TOOLS, TOOLS};
use crate::staffing;

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
    // A join names the force joined to by the side's number (decision
    // 0111); a number that names none of the side's forces becomes the
    // number no force has, so an enemy force and one that does not exist
    // are refused alike.
    if body.get("kind").and_then(Value::as_str) == Some("join") {
        if let Some(number) = body
            .get("into")
            .and_then(Value::as_str)
            .and_then(|handle| handle.strip_prefix("task_force:"))
            .and_then(|number| number.parse::<u32>().ok())
        {
            let raw = ctx
                .own_force(number)
                .map_or(NO_SUCH_UNIT, |force| force.id.raw());
            body["into"] = json!(format!("task_force:{raw}"));
        }
        return body;
    }
    if !matches!(
        body.get("kind").and_then(Value::as_str),
        Some("detach" | "form_task_force")
    ) {
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

/// A preview a seat was shown (decision 0107, step 3, rulings 25 and 26):
/// what is needed to approve it and to say why it is void. It is the
/// session's, not the world's: it is no state, no event and no hash.
struct PreviewRecord {
    number: u32,
    side: Side,
    /// The day the preview was for: the day the game was on plus one.
    day: u32,
    /// The side's revision when the preview was made.
    revision: u32,
    /// The command as the tool read it, with the numbers the side wrote.
    request: staffing::Request,
    /// The side's queued commands and the count of its queued orders when
    /// the preview was made, to say what was queued after it.
    commands: Vec<CommandKind>,
    orders: usize,
}

struct SeatState {
    seat: Seat,
    /// The orders queued for the coming day, in the order given.
    queued: Vec<OrderRequest>,
    /// The commands queued for the coming day, in the order given, with the
    /// umpire's ids inside (decision 0107, "The tools").
    commands: Vec<CommandKind>,
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
    /// The staff each side is given, if any (decision 0107). The caller
    /// hands them in: this crate holds no doctrine.
    staffs: BTreeMap<Side, Box<dyn Staff + Send>>,
    /// The commands still to be given, by day: the script's. A seat's own are
    /// queued in its [`SeatState`].
    commands: BTreeMap<u32, Vec<(Side, CommandKind)>>,
    /// The commands handed in to the engine, by day, for the saved game.
    commands_handed_in: BTreeMap<u32, Vec<(Side, CommandKind)>>,
    /// What each side's staff handed in, by day, for the saved game. A side
    /// or a day with nothing handed in is not in it.
    staff_handed_in: BTreeMap<u32, BTreeMap<Side, StaffDecisions>>,
    transcript: Vec<TranscriptEntry>,
    /// Each side's revision: it rises whenever the side queues an order or a
    /// command, or empties its queue, and whenever a day is resolved. A
    /// preview is void when it has risen since (ruling 26).
    revisions: BTreeMap<Side, u32>,
    /// The previews shown, numbered 1, 2, 3... in the session.
    previews: Vec<PreviewRecord>,
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
                    commands: Vec::new(),
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
            staffs: BTreeMap::new(),
            commands: BTreeMap::new(),
            commands_handed_in: BTreeMap::new(),
            staff_handed_in: BTreeMap::new(),
            transcript: Vec::new(),
            revisions: BTreeMap::new(),
            previews: Vec::new(),
        }
    }

    /// This session with a staff for each side in `staffs`, asked on its
    /// view each day as the engine resolves it (decision 0107). A session
    /// made with [`Session::new`] has none.
    pub fn with_staffs(mut self, staffs: BTreeMap<Side, Box<dyn Staff + Send>>) -> Session {
        self.staffs = staffs;
        self
    }

    /// This session with the commands the script gives, by day, submitted
    /// as the commander's before each day resolves. They are kept with what
    /// was handed in, whoever holds the seats.
    pub fn with_commands(mut self, commands: BTreeMap<u32, Vec<(Side, CommandKind)>>) -> Session {
        self.commands = commands;
        self
    }

    /// Whether the side has a staff in this session, and with it the tools
    /// of [`COMMAND_TOOLS`].
    pub fn has_staff(&self, side: Side) -> bool {
        self.staffs.contains_key(&side)
    }

    /// The tools a seat has, in the order of the interface: [`TOOLS`], then
    /// [`COMMAND_TOOLS`] for a side with a staff.
    pub fn tools(&self, side: Side) -> Vec<&'static str> {
        let mut tools: Vec<&'static str> = TOOLS.to_vec();
        if self.has_staff(side) {
            tools.extend(COMMAND_TOOLS);
        }
        tools
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

    /// The commands handed in to the engine, by day, side by side in the
    /// order given. A saved game keeps them beside the orders.
    pub fn commands_handed_in(&self) -> &BTreeMap<u32, Vec<(Side, CommandKind)>> {
        &self.commands_handed_in
    }

    /// What each side's staff handed in, by day, for the days and sides that
    /// handed in anything. A saved game keeps it beside the orders.
    pub fn staff_handed_in(&self) -> &BTreeMap<u32, BTreeMap<Side, StaffDecisions>> {
        &self.staff_handed_in
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
            command if COMMAND_TOOLS.contains(&command) && self.has_staff(side) => {
                self.command_tool(side, command, &args)
            }
            other => Err(CallError::naming(
                format!("\"{other}\" is not a tool"),
                &self.tools(side),
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
            let queued = &mut self.seats.get_mut(&side).expect("both seats exist").queued;
            if !queued.is_empty() {
                queued.clear();
                self.bump(side);
            }
        }
        // Each order is judged against the state as it is now and beside the
        // orders already queued, the seat's and this call's before it
        // (decision 0107, "As built": `World::check_order_queued`), so that
        // an order to the force a queued forming order makes is accepted. A
        // forming order after the order that names its force does not
        // help it: orders are judged in the order given.
        let mut accepted: Vec<OrderRequest> = Vec::new();
        let mut results = Vec::new();
        {
            let ctx = Ctx::new(self.world.side_view(side));
            let seat_queue = self.seats[&side].queued.clone();
            for (index, entry) in entries.iter().enumerate() {
                let queued: Vec<OrderRequest> =
                    seat_queue.iter().chain(accepted.iter()).cloned().collect();
                match self.judge(side, &ctx, entry, &queued) {
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
                        // A load the commander gives may take a stock below
                        // its reserve; the answer says so (decision 9 of
                        // 0111).
                        if let Some(below) = crate::orders::below_reserve(&ctx, &request.kind) {
                            result["below_reserve"] = below;
                        }
                        accepted.push(request);
                        results.push(result);
                    }
                    Err(reason) => {
                        results
                            .push(json!({ "index": index, "status": "refused", "reason": reason }));
                    }
                }
            }
        }
        if !accepted.is_empty() {
            self.bump(side);
        }
        self.seats
            .get_mut(&side)
            .expect("both seats exist")
            .queued
            .extend(accepted);
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
    fn judge(
        &self,
        side: Side,
        ctx: &Ctx<'_>,
        entry: &Value,
        queued: &[OrderRequest],
    ) -> Result<OrderRequest, String> {
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
        let mut new_force: Option<(u32, u32)> = None;
        let raw = match kind_of_unit {
            // A base keeps its number (decision 0086): the side's own only.
            "base" => ctx
                .bases
                .contains_key(&BaseId::new(number))
                .then_some(number),
            "task_force" => ctx.own_force(number).map(|f| f.id.raw()).or_else(|| {
                // A number just past the side's forces names the force a
                // queued forming order will make, in the order intake takes
                // them in (decision 0111); any other number names nothing.
                let k = number.checked_sub(ctx.force_count() + 1)?;
                let forming = queued
                    .iter()
                    .filter(|queued| queued.kind.forms_task_force())
                    .count();
                if usize::try_from(k).ok()? >= forming {
                    return None;
                }
                let floor = match side {
                    Side::Allied => MADE_IDS_FLOOR,
                    Side::Japanese => 2 * MADE_IDS_FLOOR,
                };
                let raw = ctx.next_made_force(floor) + k;
                new_force = Some((raw, number));
                Some(raw)
            }),
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
        match self.world.check_order_queued(side, &request, queued) {
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
                    "task_force" => ctx.force_number(TaskForceId::new(raw)).or_else(|| {
                        new_force.and_then(|(made, number)| (made == raw).then_some(number))
                    }),
                    "land_unit" => ctx.land_unit_number(LandUnitId::new(raw)),
                    _ => ctx.group_number(AirGroupId::new(raw)),
                },
            )),
        }
    }

    // -----------------------------------------------------------------
    // Commands and the staff (decision 0107, "As built", "The tools").
    // -----------------------------------------------------------------

    /// A copy of the world as the coming day will find it for the staff: the
    /// commands of the script and the queued commands and orders of every
    /// seat handed in, and the first three steps of the day taken
    /// ([`World::at_staff_step`]). `queue` stands in for one seat's queued
    /// commands and `extra` is one more command at the end of one side's.
    /// Nothing outside this module may name a world, so every tool function
    /// is given a view of the copy and nothing else.
    fn preview_with(
        &self,
        queue: Option<(Side, &[CommandKind])>,
        extra: Option<(Side, &CommandKind)>,
    ) -> World {
        let mut world = self.world.clone();
        let day = world.day() + 1;
        for (side, command) in self.commands.get(&day).into_iter().flatten() {
            world.submit_commands(*side, vec![command.clone()]);
        }
        for side in Side::ALL {
            let state = &self.seats[&side];
            let commands: &[CommandKind] = match queue {
                Some((who, list)) if who == side => list,
                _ => &state.commands,
            };
            world.submit_commands(side, commands.to_vec());
            world.submit_orders(side, state.queued.clone());
        }
        if let Some((side, command)) = extra {
            world.submit_commands(side, vec![command.clone()]);
        }
        world.at_staff_step()
    }

    /// The world as the coming day will find it, with what is queued.
    fn preview(&self) -> World {
        self.preview_with(None, None)
    }

    fn command_tool(
        &mut self,
        side: Side,
        tool: &str,
        args: &serde_json::Map<String, Value>,
    ) -> Result<Value, CallError> {
        let staff: &dyn Staff = self.staffs[&side].as_ref();
        match tool {
            "command_state" => {
                let world = self.preview();
                Ok(staffing::command_state(
                    &world.side_view(side),
                    staff,
                    &self.seats[&side].commands,
                ))
            }
            "plans" => Ok(staffing::plans(&self.preview().side_view(side))),
            "staff_orders" => {
                let world = self.preview();
                Ok(staffing::staff_orders(&world.side_view(side), staff))
            }
            "propose" => {
                let intent = args
                    .get("intent")
                    .and_then(Value::as_u64)
                    .and_then(|n| u32::try_from(n).ok())
                    .ok_or_else(|| {
                        CallError::new("\"intent\" is needed, as the number of an intent")
                    })?;
                let world = self.preview();
                staffing::propose(&world.side_view(side), staff, intent)
            }
            "why" => {
                let number = args
                    .get("number")
                    .and_then(Value::as_u64)
                    .and_then(|n| u32::try_from(n).ok())
                    .ok_or_else(|| {
                        CallError::new(
                            "\"number\" is needed, as the number of a staff reason you were shown",
                        )
                    })?;
                Ok(staffing::why(&self.world.side_view(side), number))
            }
            "preview" => self.preview_tool(side, args),
            "approve" if args.contains_key("preview") => self.approve_preview(side, args),
            _ => self.queue_command(side, tool, args),
        }
    }

    /// The side's revision, which a queued order or command and a resolved
    /// day raise.
    fn revision(&self, side: Side) -> u32 {
        self.revisions.get(&side).copied().unwrap_or(0)
    }

    fn bump(&mut self, side: Side) {
        *self.revisions.entry(side).or_insert(0) += 1;
    }

    /// Queues one command for the coming day if the engine's own check, made
    /// by taking it in on a copy of the world after the commands already
    /// queued, would take it.
    fn queue_command(
        &mut self,
        side: Side,
        tool: &str,
        args: &serde_json::Map<String, Value>,
    ) -> Result<Value, CallError> {
        if self.is_over() {
            return Err(CallError::new("the game is over"));
        }
        if self.seats[&side].ended {
            return Err(CallError::new("the day's orders are closed"));
        }
        let clear = match args.get("clear") {
            None | Some(Value::Null) => false,
            Some(Value::Bool(b)) => *b,
            Some(_) => return Err(CallError::new("\"clear\" must be true or false")),
        };
        match args.get("reason") {
            None | Some(Value::Null) | Some(Value::String(_)) => {}
            Some(_) => return Err(CallError::new("\"reason\" must be text")),
        }
        let queue: Vec<CommandKind> = if clear {
            Vec::new()
        } else {
            self.seats[&side].commands.clone()
        };
        let next_intent = {
            let world = self.preview_with(Some((side, &queue)), None);
            staffing::next_free_intent(&world.side_view(side), &queue)
        };
        let request = staffing::request(&self.world.side_view(side), tool, args, next_intent)?;
        self.queue_request(side, tool, request, queue, None)
    }

    /// Takes a command read from a call (or from a preview) in after
    /// `queue`, the commands queued before it, and queues it if the engine
    /// would take it. `via` is the preview it was approved from.
    fn queue_request(
        &mut self,
        side: Side,
        tool: &str,
        request: staffing::Request,
        queue: Vec<CommandKind>,
        via: Option<u32>,
    ) -> Result<Value, CallError> {
        // The script's commands of the side come first on the day, then the
        // seat's: the new command is the one after both.
        let day = self.world.day() + 1;
        let before = self.commands.get(&day).map_or(0, |list| {
            list.iter().filter(|(who, _)| *who == side).count()
        }) + queue.len();
        let world = self.preview_with(Some((side, &queue)), Some((side, &request.kind)));
        let view = world.side_view(side);
        let verdict = staffing::verdict(&view, before).ok_or_else(|| {
            CallError::new("the command could not be checked: the game has no verdict on it")
        })?;
        let state = self.seats.get_mut(&side).expect("both seats exist");
        let was = std::mem::replace(&mut state.commands, queue);
        if verdict.is_ok() {
            state.commands.push(request.kind.clone());
        }
        let queued = state.commands.len();
        let changed = state.commands != was;
        let mut out = staffing::command_answer(&view, &request, tool, &verdict, queued);
        if changed {
            self.bump(side);
        }
        if let (Some(number), Some(map)) = (via, out.as_object_mut()) {
            map.insert("preview".into(), json!(number));
        }
        Ok(out)
    }

    /// `preview`: what a changed intent or limit would do, worked on copies
    /// of the world and stored nowhere in it (rulings 25 to 28).
    fn preview_tool(
        &mut self,
        side: Side,
        args: &serde_json::Map<String, Value>,
    ) -> Result<Value, CallError> {
        if self.is_over() {
            return Err(CallError::new("the game is over"));
        }
        let command = args.get("command").and_then(Value::as_str);
        let (Some(command @ ("set_intent" | "set_limit")), false) =
            (command, args.contains_key("clear"))
        else {
            return Err(CallError::naming(
                "\"command\" is needed, as \"set_intent\" or \"set_limit\" (and \"clear\" is not an argument of preview)",
                &["set_intent", "set_limit"],
            ));
        };
        let mut rest = args.clone();
        rest.remove("command");
        let queue = self.seats[&side].commands.clone();
        let next_intent = {
            let world = self.preview_with(Some((side, &queue)), None);
            staffing::next_free_intent(&world.side_view(side), &queue)
        };
        let request = staffing::request(&self.world.side_view(side), command, &rest, next_intent)?;
        let day = self.world.day() + 1;
        let before = self.commands.get(&day).map_or(0, |list| {
            list.iter().filter(|(who, _)| *who == side).count()
        }) + queue.len();
        let with = self.preview_with(None, Some((side, &request.kind)));
        let verdict = staffing::verdict(&with.side_view(side), before).ok_or_else(|| {
            CallError::new("the command could not be checked: the game has no verdict on it")
        })?;
        if verdict.is_err() {
            return Ok(staffing::command_answer(
                &with.side_view(side),
                &request,
                "preview",
                &verdict,
                queue.len(),
            ));
        }
        let as_is = self.preview();
        let number = u32::try_from(self.previews.len() + 1).expect("few previews");
        let staff: &dyn Staff = self.staffs[&side].as_ref();
        let out = staffing::preview(
            number,
            &as_is.side_view(side),
            &with.side_view(side),
            staff,
            &request,
        );
        self.previews.push(PreviewRecord {
            number,
            side,
            day,
            revision: self.revision(side),
            request,
            commands: queue,
            orders: self.seats[&side].queued.len(),
        });
        Ok(out)
    }

    /// `approve` of a preview: queues exactly the command the preview
    /// showed, if nothing was queued and no day resolved since (ruling 26).
    fn approve_preview(
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
        if let Some(key) = ["plan", "clear"]
            .into_iter()
            .find(|k| args.contains_key(*k))
        {
            return Err(CallError::new(format!(
                "\"{key}\" is not an argument of approve with a preview: the preview is approved as it was shown"
            )));
        }
        match args.get("reason") {
            None | Some(Value::Null) | Some(Value::String(_)) => {}
            Some(_) => return Err(CallError::new("\"reason\" must be text")),
        }
        let number = args
            .get("preview")
            .and_then(Value::as_u64)
            .and_then(|n| u32::try_from(n).ok())
            .ok_or_else(|| {
                CallError::new("\"preview\" is needed, as the number of a preview you were shown")
            })?;
        let Some(record) = self
            .previews
            .iter()
            .find(|p| p.number == number && p.side == side)
        else {
            return Err(CallError::new(format!(
                "preview {number} is not a preview you were shown"
            )));
        };
        let (request, day, revision) = (record.request.clone(), record.day, record.revision);
        let (was_commands, was_orders) = (record.commands.clone(), record.orders);
        let queue = self.seats[&side].commands.clone();
        let void = if day != self.world.day() + 1 {
            Some(format!(
                "preview {number} is void: a day was resolved after it"
            ))
        } else if revision != self.revision(side) {
            let world = self.preview();
            let view = world.side_view(side);
            let mut what: Vec<String> = Vec::new();
            if queue.len() > was_commands.len() && queue[..was_commands.len()] == was_commands[..] {
                let words = staffing::commands_text(&view, &queue[was_commands.len()..]);
                what.push(format!("you queued: {}", words.join("; ")));
            } else if queue != was_commands {
                what.push("your queued commands were changed or cleared".to_string());
            }
            let orders = self.seats[&side].queued.len();
            if orders > was_orders {
                let more = orders - was_orders;
                what.push(format!(
                    "{more} order{} queued",
                    if more == 1 { " was" } else { "s were" }
                ));
            } else if orders < was_orders {
                what.push("your queued orders were cleared".to_string());
            }
            if what.is_empty() {
                what.push("your queued orders or commands were changed".to_string());
            }
            Some(format!(
                "preview {number} is void: something was queued after it ({}); preview it again",
                what.join("; ")
            ))
        } else {
            None
        };
        if let Some(reason) = void {
            let world = self.preview();
            return Ok(staffing::refused_answer(
                &world.side_view(side),
                &request,
                "approve",
                reason,
                queue.len(),
            ));
        }
        self.queue_request(side, "approve", request, queue, Some(number))
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
    /// own `submit_orders`, the script's commands through `submit_commands`,
    /// and resolves the day, asking each side's staff (decision 0107).
    fn resolve(&mut self) -> Result<(), CallError> {
        let day = self.world.day() + 1;
        for side in Side::ALL {
            self.bump(side);
        }
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
        // The commands of the script, in the order it gives them, then each
        // seat's queued commands, side by side in the order given.
        for (side, command) in self.commands.remove(&day).unwrap_or_default() {
            self.commands_handed_in
                .entry(day)
                .or_default()
                .push((side, command.clone()));
            self.world.submit_commands(side, vec![command]);
        }
        for side in Side::ALL {
            let queued =
                std::mem::take(&mut self.seats.get_mut(&side).expect("both seats").commands);
            for command in queued {
                self.commands_handed_in
                    .entry(day)
                    .or_default()
                    .push((side, command.clone()));
                self.world.submit_commands(side, vec![command]);
            }
        }
        let mut staffs = Staffs::none();
        for (side, staff) in &self.staffs {
            staffs = staffs.with(*side, staff.as_ref());
        }
        let resolved = self.world.resolve_day_staffed(staffs);
        for (side, decisions) in self.world.staff_handed_in() {
            if !decisions.is_empty() {
                self.staff_handed_in
                    .entry(day)
                    .or_default()
                    .insert(*side, decisions.clone());
            }
        }
        resolved.map(|_| ()).map_err(|e| {
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
