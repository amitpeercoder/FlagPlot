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
use sim_core::pictures::CommandRef;
use sim_core::state::Side;
use sim_core::time::Pulse;
use sim_core::view::SideView;
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
use crate::record::{
    seat_name, CallError, Post, Seat, SeatKind, TranscriptEntry, COMMAND_TOOLS, TOOLS,
};
use crate::staffing;
use crate::told::{events_answer, told_since, DayPart, STOP_ON};

/// The orders given inside a day, by day, each with its pulse and side, in
/// the order handed in (decision 0156): a scenario's script, or what a
/// session handed in, for the saved game.
pub type OrdersInsideDay = BTreeMap<u32, Vec<(Pulse, Side, OrderRequest)>>;

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
    // An escort names the force it covers. A seat at a command names it as
    // it numbers its own forces, and a number that names none of them becomes
    // the number no force has (decision 0138, 5).
    if body.get("kind").and_then(Value::as_str) == Some("escort") && ctx.view.seat().is_some() {
        if let Some(number) = body
            .get("charge")
            .and_then(Value::as_str)
            .and_then(|handle| handle.strip_prefix("task_force:"))
            .and_then(|number| number.parse::<u32>().ok())
        {
            let raw = ctx
                .own_force(number)
                .map_or(NO_SUCH_UNIT, |force| force.id.raw());
            body["charge"] = json!(format!("task_force:{raw}"));
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

/// The one refusal at a seat at a task force or a base for an order to a
/// unit that is none of its command's: another unit of the side's, one of the
/// enemy's and one that does not exist are refused alike (decision 0138, 5).
pub const NOT_YOUR_COMMANDS_UNIT: &str = "not one of your command's units";

/// What a call of the command tools is told at a seat that is not the area
/// command's (decision 0138, 5).
const NO_INTENTS_HERE: &str =
    "stating intents from a seat that is not the area command's is not built (decision 0132)";

/// A preview a seat was shown (decision 0107, step 3, rulings 25 and 26):
/// what is needed to approve it and to say why it is void. It is the
/// session's, not the world's: it is no state, no event and no hash.
struct PreviewRecord {
    number: u32,
    side: Side,
    /// The day the preview was for: the day its commands are taken in.
    day: u32,
    /// Where the clock stood when it was made: the days resolved and the
    /// pulse next if a day was open. A pulse resolved since voids it.
    moment: (u32, Option<Pulse>),
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
    /// The orders queued inside the open day, at 0600 or 1200, for the
    /// next pulse (decision 0156).
    queued_inside: Vec<OrderRequest>,
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
    /// Where each side's player seat sits; the area command where nothing
    /// else was set (decision 0138, 5).
    posts: BTreeMap<Side, Post>,
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
    /// Whether a seat may stop the clock between pulses and give orders
    /// inside a day: the scenario's key (decision 0156).
    inside_day: bool,
    /// The script's orders inside a day, for the seats the script holds.
    script_inside: OrdersInsideDay,
    /// The orders handed in to the engine inside a day, with their pulses,
    /// for the saved game.
    handed_in_inside: OrdersInsideDay,
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
                    queued_inside: Vec::new(),
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
            posts: BTreeMap::new(),
            handed_in: BTreeMap::new(),
            staffs: BTreeMap::new(),
            commands: BTreeMap::new(),
            commands_handed_in: BTreeMap::new(),
            staff_handed_in: BTreeMap::new(),
            transcript: Vec::new(),
            revisions: BTreeMap::new(),
            previews: Vec::new(),
            inside_day: false,
            script_inside: OrdersInsideDay::new(),
            handed_in_inside: OrdersInsideDay::new(),
        }
    }

    /// This session with the scenario's key `orders_inside_a_day` (decision
    /// 0156) and the script's orders inside a day. The script's are handed
    /// in only for a seat the script holds, as its orders of dusk are.
    pub fn with_orders_inside_a_day(mut self, on: bool, script: OrdersInsideDay) -> Session {
        self.inside_day = on;
        self.script_inside = script;
        self
    }

    /// Whether the scenario lets a seat give orders inside a day.
    pub fn orders_inside_a_day(&self) -> bool {
        self.inside_day
    }

    /// The orders handed in to the engine inside a day, by day, each with
    /// its pulse and side, in the order handed in. A saved game keeps them
    /// beside the orders of dusk.
    pub fn handed_in_inside_day(&self) -> &OrdersInsideDay {
        &self.handed_in_inside
    }

    /// The day open and its next pulse, while the clock is stopped at 0600
    /// or 1200 (decision 0156); `None` at dusk.
    pub fn open_day(&self) -> Option<(u32, Pulse)> {
        let in_hand = self.world.day_in_hand()?;
        Some((in_hand.day, in_hand.next_pulse?))
    }

    /// Where the clock stands, in words: "dusk before day N", "0600 of day
    /// N" or "1200 of day N".
    fn clock_words(&self) -> String {
        match self.open_day() {
            Some((day, Pulse::Afternoon)) => format!("1200 of day {day}"),
            Some((day, _)) => format!("0600 of day {day}"),
            None if self.is_over() => {
                format!("dusk at the end of day {}, the game over", self.world.day())
            }
            None => format!("dusk before day {}", self.world.day() + 1),
        }
    }

    /// Where the clock stands, for an answer: the day whose pulse is next,
    /// the stop (`dusk`, `0600` or `1200`), the pulse next (`null` once the
    /// game is over) and the days resolved.
    fn clock(&self) -> Value {
        let (day, at, next) = match self.open_day() {
            Some((day, Pulse::Afternoon)) => (day, "1200", Some("afternoon")),
            Some((day, pulse)) => (day, "0600", Some(pulse.name())),
            None => (
                self.world.day() + 1,
                "dusk",
                (!self.is_over()).then_some("night"),
            ),
        };
        json!({
            "day": day,
            "at": at,
            "next_pulse": next,
            "days_resolved": self.world.day(),
            "words": self.clock_words(),
        })
    }

    /// The day a command queued now is taken in: the day after the open day
    /// while one is open, else the coming day.
    fn command_day(&self) -> u32 {
        match self.open_day() {
            Some((day, _)) => day + 1,
            None => self.world.day() + 1,
        }
    }

    /// The words an answer worked on the state as it stands inside a day
    /// carries (decision 0156, rule 6); `None` at dusk.
    fn inside_day_note(&self, command: bool) -> Option<String> {
        let (day, _) = self.open_day()?;
        let at = self.clock_words();
        Some(if command {
            format!(
                "Given at {at}: the command is queued for dusk at the end of day {day} and is taken in at the start of day {}; the staff is not asked again inside the day. It was checked on the state as it stands, as if day {day} were closed now, before the rest of the day, its fight ashore and its end are resolved; it is checked again when it is taken in at dusk, and if it is refused then its refusal is recorded as any refused command's is.",
                day + 1
            )
        } else {
            format!(
                "Worked at {at} on the state as it stands, as if day {day} were closed now: before the rest of the day, its fight ashore and its end are resolved. The staff is asked at dusk at the end of day {day}, on the state as it is then, for day {}, and may then do otherwise.",
                day + 1
            )
        })
    }

    /// `answer` with the note of [`Session::inside_day_note`] added while a
    /// day is open.
    fn noted(&self, mut answer: Value, command: bool) -> Value {
        if let (Some(note), Some(map)) = (self.inside_day_note(command), answer.as_object_mut()) {
            map.insert("inside_day".into(), json!(note));
        }
        answer
    }

    /// Whether `advance` is offered: the scenario lets a seat stop between
    /// pulses and exactly one seat is a player's, as for `run`.
    fn advance_offered(&self) -> bool {
        self.inside_day && self.player_seats() == 1
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

    /// This session with the side's seat at `post` (decision 0138, 5), before
    /// play. A seat at a task force or a base needs command levels on and a
    /// command of the side at the start; the refusal says which is wrong.
    pub fn with_post(mut self, side: Side, post: Post) -> Result<Session, String> {
        if self.world.day() != 0 {
            return Err("a seat is placed before the first day is played".to_string());
        }
        if let Post::At(command) = post {
            let view = self.world.side_view(side);
            if !view.command_levels() {
                return Err(
                    "command levels are off in this scenario: the only seat is the area command's"
                        .to_string(),
                );
            }
            if !view.commands().contains(&command) {
                return Err(format!(
                    "{command} is not a task force or a base of the {} side at the start",
                    seat_name(side)
                ));
            }
        }
        self.posts.insert(side, post);
        Ok(self)
    }

    /// Where the side's seat sits.
    pub fn post(&self, side: Side) -> Post {
        self.posts.get(&side).copied().unwrap_or_default()
    }

    /// Whether the side's seat has the command tools of [`COMMAND_TOOLS`]:
    /// its side has a staff and the seat is the area command's.
    pub fn has_command_tools(&self, side: Side) -> bool {
        self.has_staff(side) && !self.at_a_command(side)
    }

    /// Whether the side's seat is at a task force or a base.
    fn at_a_command(&self, side: Side) -> bool {
        matches!(self.post(side), Post::At(_))
    }

    /// The side's view as its seat reads it: the side's own for the area seat
    /// (its units are given as the area command holds them by
    /// [`Session::read`]), the side's own with its own picture under the
    /// easier setting, and the view cut to the command at a task force or a
    /// base. Refused if that command has ceased to be.
    fn view_of(&self, side: Side) -> Result<SideView<'_>, CallError> {
        let view = self.world.side_view(side);
        match self.post(side) {
            Post::Area => Ok(view),
            Post::AreaEasier => Ok(view.with_side_picture()),
            Post::At(command) => view.at_seat(command).ok_or_else(|| {
                CallError::new(format!(
                    "{command} has ceased to be a command of the side: this seat holds nothing now; end the day to go on"
                ))
            }),
        }
    }

    /// Runs `read` on the view the side's seat reads its units from: for the
    /// area seat as the area command holds them (decision 0136, 9).
    fn read<R>(&self, side: Side, read: impl FnOnce(&SideView<'_>) -> R) -> Result<R, CallError> {
        let view = self.view_of(side)?;
        Ok(match self.post(side) {
            Post::Area => {
                let known = view.known_to(CommandRef::Area);
                read(&known.view())
            }
            _ => read(&view),
        })
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
        if self.has_command_tools(side) {
            // `run` is offered only where exactly one seat is held by a
            // player (decision 0107, item 29).
            let runnable = self.player_seats() == 1;
            tools.extend(
                COMMAND_TOOLS
                    .iter()
                    .filter(|tool| **tool != "run" || runnable),
            );
        }
        // `advance` is offered where the scenario lets a seat stop between
        // pulses and one seat is a player's (decision 0156, rule 8).
        if self.advance_offered() {
            tools.push("advance");
        }
        tools
    }

    /// How many seats a player holds.
    fn player_seats(&self) -> usize {
        self.seats
            .values()
            .filter(|s| s.seat.kind() == SeatKind::Player)
            .count()
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
            "get_briefing" if self.open_day().is_some() => {
                let (day, _) = self.open_day().expect("a day is open");
                Err(CallError::new(format!(
                    "the briefing is made at dusk, and the clock stands at {}: read events for day {day} so far (events {{\"day\": {day}}}), and list_forces, list_contacts and list_bases for the state as it stands",
                    self.clock_words()
                )))
            }
            "get_briefing" => {
                let text = match args.get("text") {
                    None | Some(Value::Null) => false,
                    Some(Value::Bool(b)) => *b,
                    Some(_) => return Err(CallError::new("\"text\" must be true or false")),
                };
                // The area seat reads its own units as the area command
                // holds them (decision 0136, 9); the umpire's result below
                // is read from the side's own view.
                let mut briefing = self.read(side, |view| {
                    briefing_for_game(view, &self.scenario, Some(self.days))
                })?;
                // The final briefing carries the umpire's result (decision 8
                // of 0054): the game is over, so it is no longer a secret. A
                // seat at a command is not given it: it is the side's.
                if let Some(result) = self.final_result().filter(|_| !self.at_a_command(side)) {
                    briefing = briefing.with_result(&self.world.side_view(side), &result);
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
                to_value(&self.read(side, |view| list_forces(view, area))?)
            }
            "list_bases" => {
                let area = area_of(&args)?;
                to_value(&self.read(side, |view| list_bases(view, area))?)
            }
            "list_contacts" => {
                let area = area_of(&args)?;
                to_value(&list_contacts(&self.view_of(side)?, area))
            }
            "get_entity" => {
                let handle = args.get("handle").and_then(Value::as_str).ok_or_else(|| {
                    CallError::new("\"handle\" is needed, as text such as \"task_force:3\"")
                })?;
                to_value(&self.read(side, |view| get_entity(view, handle))?)
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
                let answer =
                    to_value(&self.read(side, |view| measure_in(view, &question, &context))?)?;
                Ok(self.measure_noted(answer))
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
                to_value(&explain(&self.view_of(side)?, event))
            }
            "events" => self.events_tool(side, &args),
            "advance" if self.advance_offered() => self.advance(side, &args),
            "issue_orders" => self.issue_orders(side, &args),
            "end_day" => self.end_day(side),
            command if COMMAND_TOOLS.contains(&command) && self.at_a_command(side) => {
                Err(CallError::new(NO_INTENTS_HERE))
            }
            command if COMMAND_TOOLS.contains(&command) && self.has_staff(side) => {
                self.command_tool(side, command, &args)
            }
            other => Err(CallError::naming(
                format!("\"{other}\" is not a tool"),
                &self.tools(side),
            )),
        }
    }

    /// A measure of a passage or a yard trip, with its note while a day is
    /// open: the engine works it from the start of the coming day's night,
    /// which inside a day has passed (decision 0156).
    fn measure_noted(&self, mut answer: Value) -> Value {
        let Some((day, pulse)) = self.open_day() else {
            return answer;
        };
        let at = self.clock_words();
        let note = match answer.get("question").and_then(Value::as_str) {
            Some("passage") => format!(
                "Asked at {at}: this is worked as if the force left at 1800, the start of day {day}'s night, which has passed. An order given now is taken in at the next pulse, {} hours later than worked, so the times are early by about that much and the hours of darkness on the passage are not those worked.",
                if pulse == Pulse::Afternoon { 18 } else { 12 }
            ),
            Some("yard") => format!(
                "Asked at {at}: this is worked as if the force left at 1800, the start of day {day}'s night, which has passed. A yard order is given at dusk and taken in at the start of day {}, a day later than worked, so the days of leaving and return are a day early.",
                day + 1
            ),
            _ => return answer,
        };
        if let Some(map) = answer.as_object_mut() {
            map.insert("inside_day".into(), json!(note));
        }
        answer
    }

    /// `events`: the side's own events of a day, or of a part of it, each as
    /// `explain` gives it (decision 0156, rule 9).
    fn events_tool(
        &self,
        side: Side,
        args: &serde_json::Map<String, Value>,
    ) -> Result<Value, CallError> {
        if let Some(key) = args
            .keys()
            .find(|k| !["day", "pulse"].contains(&k.as_str()))
        {
            return Err(CallError::naming(
                format!("\"{key}\" is not an argument of events"),
                &["day", "pulse"],
            ));
        }
        // The day the clock is in: the open day, else the last resolved.
        let current = self.open_day().map_or(self.world.day(), |(day, _)| day);
        let day = match args.get("day") {
            None | Some(Value::Null) => current.max(1),
            Some(value) => value
                .as_u64()
                .and_then(|n| u32::try_from(n).ok())
                .filter(|n| *n >= 1)
                .ok_or_else(|| CallError::new("\"day\" must be a whole number, at least 1"))?,
        };
        let part = match args.get("pulse") {
            None | Some(Value::Null) => None,
            Some(value) => Some(value.as_str().and_then(DayPart::parse).ok_or_else(|| {
                CallError::naming(
                    "\"pulse\" must be one of the parts of a day",
                    &["night", "morning", "afternoon", "end"],
                )
            })?),
        };
        let begun = day <= current;
        let view = self.view_of(side)?;
        Ok(events_answer(&view, day, part, begun, &self.clock_words()))
    }

    /// `advance`: resolves up to `pulses` pulses one at a time, stopping at
    /// the end of the first in which the side was told any of `stop_on`, at
    /// a dusk after a day on which an interrupt of the side fired, or when
    /// the game is over (decision 0156, rule 8).
    fn advance(
        &mut self,
        side: Side,
        args: &serde_json::Map<String, Value>,
    ) -> Result<Value, CallError> {
        if let Some(key) = args
            .keys()
            .find(|k| !["pulses", "stop_on", "reason"].contains(&k.as_str()))
        {
            return Err(CallError::naming(
                format!("\"{key}\" is not an argument of advance"),
                &["pulses", "stop_on", "reason"],
            ));
        }
        match args.get("reason") {
            None | Some(Value::Null) | Some(Value::String(_)) => {}
            Some(_) => return Err(CallError::new("\"reason\" must be text")),
        }
        if self.is_over() {
            return Err(CallError::new("the game is over"));
        }
        let pulses = args
            .get("pulses")
            .and_then(Value::as_u64)
            .and_then(|n| u32::try_from(n).ok())
            .filter(|n| *n >= 1)
            .ok_or_else(|| {
                CallError::new("\"pulses\" is needed, as a whole number of pulses, at least 1")
            })?;
        let mut stop_on: Vec<&'static str> = Vec::new();
        match args.get("stop_on") {
            None | Some(Value::Null) => {}
            Some(Value::Array(items)) => {
                for item in items {
                    let thing = item
                        .as_str()
                        .and_then(|t| STOP_ON.iter().copied().find(|s| *s == t))
                        .ok_or_else(|| {
                            CallError::naming(
                                "each of \"stop_on\" must be one of the things advance can stop on",
                                &STOP_ON,
                            )
                        })?;
                    if !stop_on.contains(&thing) {
                        stop_on.push(thing);
                    }
                }
            }
            Some(_) => {
                return Err(CallError::naming(
                    "\"stop_on\" must be a list of the things advance can stop on",
                    &STOP_ON,
                ))
            }
        }
        let started = self.clock_words();
        let mut resolved: Vec<Value> = Vec::new();
        let mut told: Vec<Value>;
        let mut last_closed: Option<u32> = None;
        let stopped = loop {
            // What the side held before the pulse: what it is told in it is
            // its events after these.
            let before = self.view_of(side).map_or(0, |view| view.events().len());
            if self.open_day().is_none() {
                self.begin_day()?;
            }
            let (day, pulse) = self.resolve_pulse()?;
            resolved.push(json!({"day": day, "pulse": pulse.name()}));
            let closed = self.open_day().is_none();
            if closed {
                last_closed = Some(day);
            }
            told = match self.view_of(side) {
                Ok(view) => told_since(&view, before, &stop_on),
                Err(_) => Vec::new(),
            };
            if self.is_over() {
                break "game_over";
            }
            if closed && staffing::fired_on_day(&self.world.side_view(side), day) {
                break "interrupt";
            }
            if !told.is_empty() {
                break "told";
            }
            if u32::try_from(resolved.len()).expect("few pulses") >= pulses {
                break "pulses";
            }
        };
        let fired = match last_closed {
            Some(day) => staffing::firings_json(&self.world.side_view(side), day),
            None => Vec::new(),
        };
        let result = match self.final_result().filter(|_| !self.at_a_command(side)) {
            Some(result) => to_value(&result_part(&self.world.side_view(side), &result))?,
            None => Value::Null,
        };
        let count = resolved.len();
        let why = match stopped {
            "told" => {
                let things: Vec<String> = told
                    .iter()
                    .map(|t| {
                        let numbers: Vec<String> = t["events"]
                            .as_array()
                            .into_iter()
                            .flatten()
                            .map(|n| n.to_string())
                            .collect();
                        format!(
                            "{} (event{} {})",
                            t["thing"].as_str().unwrap_or_default(),
                            if numbers.len() == 1 { "" } else { "s" },
                            numbers.join(", ")
                        )
                    })
                    .collect();
                format!("you were told of {}", things.join("; "))
            }
            "interrupt" => format!("an interrupt fired: {}", staffing::firing_words(&fired)),
            "game_over" => "the game is over".to_string(),
            _ => format!(
                "the {pulses} pulse{} asked for {} resolved",
                if pulses == 1 { "" } else { "s" },
                if pulses == 1 { "is" } else { "are" }
            ),
        };
        let summary = format!(
            "From {started}, resolved {count} pulse{}; stopped at {}: {why}.",
            if count == 1 { "" } else { "s" },
            self.clock_words()
        );
        Ok(json!({
            "format": FORMAT,
            "version": VERSION,
            "kind": "advance",
            "pulses_resolved": resolved,
            "stop_on": stop_on,
            "stopped": stopped,
            "told": told,
            "clock": self.clock(),
            "fired": fired,
            "game_over": self.is_over(),
            "result": result,
            "summary": summary,
        }))
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
        // While a day is open the orders are given inside it, for its next
        // pulse, and only the seven kinds that may be (decision 0156).
        let inside = self.open_day();
        if clear {
            let state = self.seats.get_mut(&side).expect("both seats exist");
            let queued = if inside.is_some() {
                &mut state.queued_inside
            } else {
                &mut state.queued
            };
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
            let view = self.view_of(side)?;
            let ctx = Ctx::new(view);
            let seat_queue = if inside.is_some() {
                self.seats[&side].queued_inside.clone()
            } else {
                self.seats[&side].queued.clone()
            };
            for (index, entry) in entries.iter().enumerate() {
                let queued: Vec<OrderRequest> =
                    seat_queue.iter().chain(accepted.iter()).cloned().collect();
                match self.judge(side, &ctx, entry, &queued, inside.is_some()) {
                    Ok(request) => {
                        // A yard order's acceptance states the day of return,
                        // worked by the function the order itself uses on the
                        // state as it stands (decision 0069).
                        let mut result = json!({ "index": index, "status": "queued" });
                        if let Some((day, pulse)) = inside {
                            result["for"] =
                                json!(format!("the {} pulse of day {day}", pulse.name()));
                        }
                        if let OrderKind::TaskForce {
                            target,
                            order: TaskForceOrder::SendToYard,
                        } = &request.kind
                        {
                            if let Some(plan) = view.yard_plan(*target) {
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
        let state = self.seats.get_mut(&side).expect("both seats exist");
        if inside.is_some() {
            state.queued_inside.extend(accepted);
        } else {
            state.queued.extend(accepted);
        }
        let refused = results.iter().filter(|r| r["status"] == "refused").count();
        let accepted = results.len() - refused;
        let (queued, for_what, for_day) = match inside {
            Some((day, pulse)) => (
                self.seats[&side].queued_inside.len(),
                format!("for the {} pulse of day {day}", pulse.name()),
                day,
            ),
            None => (
                self.seats[&side].queued.len(),
                format!("for day {}", self.world.day() + 1),
                self.world.day() + 1,
            ),
        };
        let mut summary = format!(
            "In this call {accepted} of {} orders queued and {refused} refused. A refused order is not queued and the others in the call stand. {queued} {} now queued {for_what}.",
            results.len(),
            if queued == 1 { "order is" } else { "orders are" },
        );
        if inside.is_some() {
            summary.push_str(&format!(
                " Given at {}: each is taken in at the start of that pulse and checked again then, on the state as it stands; one to a unit an intent's forces limit holds holds it from the staff. Commands to the staff are taken in at dusk.",
                self.clock_words()
            ));
        }
        let mut answer = json!({
            "format": FORMAT,
            "version": VERSION,
            "kind": "issue_orders",
            "for_day": for_day,
            "cleared": clear,
            "results": results,
            "queued_in_call": accepted,
            "refused_in_call": refused,
            "queued": queued,
            "summary": summary,
        });
        if let Some((_, pulse)) = inside {
            answer["for_pulse"] = json!(pulse.name());
        }
        Ok(answer)
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
        inside_day: bool,
    ) -> Result<OrderRequest, String> {
        // At a seat at a command the one refusal is its own words (decision
        // 0138, 5); everywhere else it is the side's.
        let at_seat = ctx.view.seat().is_some();
        let not_known = || {
            if at_seat {
                NOT_YOUR_COMMANDS_UNIT.to_string()
            } else {
                NOT_KNOWN.to_string()
            }
        };
        let unit = entry
            .get("unit")
            .and_then(Value::as_str)
            .ok_or_else(|| "malformed order: \"unit\" is missing or not text".to_string())?;
        if !is_orderable_unit(unit) {
            return Err(not_known());
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
                // A seat at a command makes no force by an order: a number
                // that is none of its own names nothing (decision 0138, 5).
                if at_seat {
                    return None;
                }
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
        // At a seat at a command an order that names a unit the command does
        // not hold, as its target or in its body, is refused here with the one
        // reason, before the engine is asked, so that no reason of the
        // engine's can tell what the unit is (decision 0138, 5).
        if at_seat
            && (raw == NO_SUCH_UNIT
                || !stand_ins.is_empty()
                || body.to_string().contains(&NO_SUCH_UNIT.to_string()))
        {
            return Err(not_known());
        }
        let kind = order_from_json(&format!("{kind_of_unit}:{raw}"), &body)
            .map_err(|e| format!("malformed order: {e}"))?;
        let request = OrderRequest {
            kind,
            origin: Origin::PLAYER,
        };
        // Inside a day only seven kinds may be given; any other is refused
        // there as given at dusk, as the engine refuses it at intake
        // (decision 0156).
        if inside_day && !request.kind.may_be_given_inside_a_day() {
            return Err(crate::words::refusal(&RejectReason::GivenAtDusk));
        }
        match self.world.check_order_queued(side, &request, queued) {
            Ok(()) => Ok(request),
            Err(RejectReason::NotYourUnit) => Err(not_known()),
            // A posture order for a land unit that is not the side's own
            // (an enemy's, or one that does not exist) is refused alike.
            Err(RejectReason::NotYourLandUnit)
                if matches!(request.kind, OrderKind::LandUnit { .. }) =>
            {
                Err(not_known())
            }
            // The engine's words may name a ship by its umpire's number: show
            // the side's, or the number the side wrote for a stand-in.
            Err(reason) => {
                let words = crate::words::refusal(&reason);
                Err(map_unit_tokens(&words, |kind, raw| match kind {
                    "ship" => stand_ins
                        .get(&raw)
                        .copied()
                        .or_else(|| ctx.ship_number(ShipId::new(raw))),
                    "task_force" => ctx.force_number(TaskForceId::new(raw)).or_else(|| {
                        new_force.and_then(|(made, number)| (made == raw).then_some(number))
                    }),
                    "land_unit" => ctx.land_unit_number(LandUnitId::new(raw)),
                    _ => ctx.group_number(AirGroupId::new(raw)),
                }))
            }
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
        // While a day is open the copy is the state as it stands with the
        // open day treated as closed: no end of the day is resolved, and the
        // staff step taken is that of the day after it, when the commands
        // queued now are taken in (decision 0156). Between days it is a
        // plain copy.
        let mut world = self.world.as_if_day_closed();
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
            // The orders queued at 0600 or 1200 are taken in before that
            // dusk, at the next pulse: the copy takes them in at its dusk,
            // where the override holds a unit as the pulse would.
            world.submit_orders(side, state.queued_inside.clone());
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
                Ok(self.noted(
                    staffing::command_state(
                        &world.side_view(side),
                        staff,
                        &self.seats[&side].commands,
                    ),
                    false,
                ))
            }
            "plans" => Ok(self.noted(
                staffing::plans(&self.preview().side_view(side), staff),
                false,
            )),
            "staff_orders" => {
                let world = self.preview();
                Ok(self.noted(staffing::staff_orders(&world.side_view(side), staff), false))
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
                let answer = staffing::propose(&world.side_view(side), staff, intent)?;
                Ok(self.noted(answer, false))
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
            "preview" => {
                let answer = self.preview_tool(side, args)?;
                Ok(self.noted(answer, false))
            }
            "run" => self.run_days(side, args),
            "report" => {
                if !args.is_empty() {
                    return Err(CallError::new("report takes no arguments"));
                }
                Ok(staffing::report(&self.world.side_view(side)))
            }
            "approve" if args.contains_key("preview") => {
                let answer = self.approve_preview(side, args)?;
                Ok(self.noted(answer, true))
            }
            _ => {
                let answer = self.queue_command(side, tool, args)?;
                Ok(self.noted(answer, true))
            }
        }
    }

    /// Where the clock stands: the days resolved, and the pulse next while
    /// a day is open.
    fn moment(&self) -> (u32, Option<Pulse>) {
        (self.world.day(), self.open_day().map(|(_, pulse)| pulse))
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
        let (next_intent, next_interrupt) = {
            let world = self.preview_with(Some((side, &queue)), None);
            let view = world.side_view(side);
            (
                staffing::next_free_intent(&view, &queue),
                staffing::next_free_interrupt(&view, &queue),
            )
        };
        let request = staffing::request(
            &self.world.side_view(side),
            tool,
            args,
            next_intent,
            next_interrupt,
        )?;
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
        let day = self.command_day();
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
        let request =
            staffing::request(&self.world.side_view(side), command, &rest, next_intent, 0)?;
        let day = self.command_day();
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
            moment: self.moment(),
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
        let (request, day, revision, moment) = (
            record.request.clone(),
            record.day,
            record.revision,
            record.moment,
        );
        let (was_commands, was_orders) = (record.commands.clone(), record.orders);
        let queue = self.seats[&side].commands.clone();
        let void = if day != self.command_day() {
            Some(format!(
                "preview {number} is void: a day was resolved after it"
            ))
        } else if moment != self.moment() {
            Some(format!(
                "preview {number} is void: a pulse was resolved after it; preview it again"
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
        let result = match self.final_result().filter(|_| !self.at_a_command(side)) {
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

    /// `run`: resolves up to `days` days, each exactly as `end_day` does,
    /// and stops after a day on which an interrupt of the side fired, or at
    /// the end of the game, or at `days` (decision 0107, item 29 and ruling
    /// 36). The queued orders and commands go in on the first day.
    fn run_days(
        &mut self,
        side: Side,
        args: &serde_json::Map<String, Value>,
    ) -> Result<Value, CallError> {
        if let Some(key) = args
            .keys()
            .find(|k| !["days", "reason"].contains(&k.as_str()))
        {
            return Err(CallError::naming(
                format!("\"{key}\" is not an argument of run"),
                &["days", "reason"],
            ));
        }
        match args.get("reason") {
            None | Some(Value::Null) | Some(Value::String(_)) => {}
            Some(_) => return Err(CallError::new("\"reason\" must be text")),
        }
        if self.player_seats() != 1 {
            return Err(CallError::new(
                "run is offered only where exactly one seat is held by a player: use end_day",
            ));
        }
        if self.is_over() {
            return Err(CallError::new("the game is over"));
        }
        let days = args
            .get("days")
            .and_then(Value::as_u64)
            .and_then(|n| u32::try_from(n).ok())
            .filter(|n| *n >= 1)
            .ok_or_else(|| {
                CallError::new("\"days\" is needed, as a whole number of days, at least 1")
            })?;
        // The staff's orders for the first day, worked before it is
        // resolved, as staff_orders gives them.
        let staff_orders = {
            let world = self.preview();
            let staff: &dyn Staff = self.staffs[&side].as_ref();
            staffing::staff_orders(&world.side_view(side), staff)
        };
        let first = self.world.day() + 1;
        let mut count = 0;
        let stopped = loop {
            self.resolve()?;
            count += 1;
            let day = self.world.day();
            if self.is_over() {
                break "game_over";
            }
            if staffing::fired_on_day(&self.world.side_view(side), day) {
                break "interrupt";
            }
            if count >= days {
                break "days";
            }
        };
        let last = self.world.day();
        let fired = staffing::firings_json(&self.world.side_view(side), last);
        let result = match self.final_result() {
            Some(result) => to_value(&result_part(&self.world.side_view(side), &result))?,
            None => Value::Null,
        };
        let resolved = if count == 1 {
            format!("Resolved day {first} (1 day)")
        } else {
            format!("Resolved days {first} to {last} ({count} days)")
        };
        let why_stopped = match stopped {
            "interrupt" => staffing::firing_words(&fired),
            "game_over" => "the game is over".to_string(),
            _ => format!("the {days} days asked for are resolved"),
        };
        let mut summary = format!("{resolved}; stopped after day {last}: {why_stopped}.");
        if stopped == "game_over" && !fired.is_empty() {
            summary.push_str(&format!(
                " Fired on that day: {}.",
                staffing::firing_words(&fired)
            ));
        }
        Ok(json!({
            "format": FORMAT,
            "version": VERSION,
            "kind": "run",
            "staff_orders": staff_orders,
            "days_resolved": {"first": first, "last": last, "count": count},
            "stopped": stopped,
            "fired": fired,
            "game_over": self.is_over(),
            "day": last,
            "result": result,
            "summary": summary,
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

    /// Resolves the rest of the day: from dusk, the whole coming day; with a
    /// day open, the rest of it, to its dusk (decision 0156). A day with no
    /// order inside it is resolved exactly as `World::resolve_day_staffed`
    /// resolves it, which is these same parts called in turn.
    fn resolve(&mut self) -> Result<(), CallError> {
        if self.open_day().is_none() {
            self.begin_day()?;
        }
        while self.open_day().is_some() {
            self.resolve_pulse()?;
        }
        Ok(())
    }

    /// Hands in every seat's orders for the coming day through the engine's
    /// own `submit_orders`, the script's commands and each seat's queued
    /// commands through `submit_commands`, and opens the day, asking each
    /// side's staff (decisions 0107 and 0156).
    fn begin_day(&mut self) -> Result<(), CallError> {
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
        let opened = self.world.open_day(staffs, &mut |_, _| {});
        for (side, decisions) in self.world.staff_handed_in() {
            if !decisions.is_empty() {
                self.staff_handed_in
                    .entry(day)
                    .or_default()
                    .insert(*side, decisions.clone());
            }
        }
        opened.map(|_| ()).map_err(internal)
    }

    /// Resolves the next pulse of the open day, and closes the day after its
    /// afternoon (decision 0156). Before the morning or the afternoon it
    /// hands in the orders given inside the day for that pulse: the script's
    /// for a seat the script holds, in the order the script gives them, and
    /// each player seat's queued at 0600 or 1200. Returns the day and the
    /// pulse resolved.
    fn resolve_pulse(&mut self) -> Result<(u32, Pulse), CallError> {
        let (day, pulse) = self
            .open_day()
            .ok_or_else(|| CallError::new("no day is open"))?;
        if pulse != Pulse::Night {
            let mut handed: Vec<(Side, OrderRequest)> = Vec::new();
            for (at, side, request) in self.script_inside.get(&day).into_iter().flatten() {
                if *at == pulse && self.seats[side].seat.kind() == SeatKind::Script {
                    handed.push((*side, request.clone()));
                }
            }
            for side in Side::ALL {
                let state = self.seats.get_mut(&side).expect("both seats exist");
                if state.seat.kind() == SeatKind::Player {
                    for request in std::mem::take(&mut state.queued_inside) {
                        handed.push((side, request));
                    }
                }
            }
            for (side, request) in handed {
                self.handed_in_inside
                    .entry(day)
                    .or_default()
                    .push((pulse, side, request.clone()));
                self.world
                    .submit_orders_inside_day(side, vec![request])
                    .map_err(internal)?;
            }
        }
        self.world
            .resolve_next_pulse(&mut |_, _| {})
            .map_err(internal)?;
        if self.open_day().is_none() {
            self.world.close_day(&mut |_, _| {}).map_err(internal)?;
        }
        Ok((day, pulse))
    }
}

/// The one thing a caller is told when the engine cannot go on. The engine's
/// own words may name the other side's units: they go to the umpire's
/// stderr.
fn internal(e: sim_core::SimError) -> CallError {
    eprintln!("operator: the day could not be resolved: {e}");
    CallError::new(
        "the day could not be resolved: the game has stopped on an internal error; the operator has the details",
    )
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
