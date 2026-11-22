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

use std::borrow::Cow;
use std::collections::BTreeMap;

use serde_json::{json, Value};
use sim_core::command::{
    CommandKind, IntentKind, Leave, Limit, Maker, Staff, StaffDecisions, Staffs,
};
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, IntentId, LandUnitId, ShipId, TaskForceId};
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
use crate::pursuit;
use crate::queries::{get_entity, list_bases, list_contacts, list_forces};
use crate::record::{
    seat_name, CallError, Post, Seat, SeatKind, TranscriptEntry, COMMAND_TOOLS, TOOLS, TRIAL_TOOLS,
    WATCHER_TOOLS,
};
use crate::staffing;
use crate::told::{events_answer, told_since, DayPart, STOP_ON};
use crate::trial::{
    answer_fields, command_words, described_fields, window_fields, Answered, AskedShown, BesideOf,
    CourseStep, DatesAsked, SecondAsked, SizeAsked, TrialAsked, TrialRunner, TrialStep,
};

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
    // An escort names the force it covers by the side's number, at every
    // seat, as a join names its force; a number that names none of the
    // side's forces becomes the number no force has, so an enemy force and
    // one that does not exist are refused alike (decisions 0138, 5, and
    // 0172, 2).
    if body.get("kind").and_then(Value::as_str) == Some("escort") {
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
    // A fuel_from names the giver by the side's number, as an escort names
    // its charge (decision 0201).
    if body.get("kind").and_then(Value::as_str) == Some("fuel_from") {
        if let Some(number) = body
            .get("giver")
            .and_then(Value::as_str)
            .and_then(|handle| handle.strip_prefix("task_force:"))
            .and_then(|number| number.parse::<u32>().ok())
        {
            let raw = ctx
                .own_force(number)
                .map_or(NO_SUCH_UNIT, |force| force.id.raw());
            body["giver"] = json!(format!("task_force:{raw}"));
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
            let id = match ctx.own_ship_or_held(number) {
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

/// An estimate a seat was shown (decision 0178, rule 19): kept with the
/// session, numbered from 1 among its side's, with the day and the moment it
/// was asked, the revision of the seat's queue then, what was asked and the
/// answer as given. It is no state, no event and no hash, and it is not in a
/// saved game.
struct TrialRecord {
    number: u32,
    side: Side,
    moment: (u32, Option<Pulse>),
    revision: u32,
    asked: Value,
    answer: Value,
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
    /// In a watched game, the call of time the seat made and waits on: the
    /// seat is ready while it holds one (decision 0182, rule 5).
    held: Option<Held>,
    /// In a watched game, the number in the transcript of the seat's last
    /// call of time answered: its calls since its last stop are those after
    /// it (rule 14).
    last_stop: u32,
}

/// A call of time held in a watched game (decision 0182, amended 2 and 3):
/// the call as made, the days resolved when it was made (its transcript
/// entry's), and what its answer is gathered from.
struct Held {
    tool: String,
    arguments: Value,
    after_day: u32,
    wait: Wait,
}

/// What a call of time waits for, and what its answer is made from, as the
/// tool gathers it inside its loop in a game that is not watched.
enum Wait {
    /// `end_day`: until the next dusk.
    EndDay,
    /// `run`: for that many days, or until a dusk after a day on which an
    /// interrupt of the side fired.
    Run(RunWait),
    /// `advance`: for that many pulses, or until the end of a pulse in which
    /// the side was told one of the things named, or a dusk after a day on
    /// which an interrupt fired.
    Advance(AdvanceWait),
}

/// What a call of `run` is answered from.
struct RunWait {
    days: u32,
    /// The first day it resolves: the open day if it was called inside one.
    first: u32,
    /// The days closed since the call.
    count: u32,
    /// The staff's orders for the first day, worked at the call.
    staff_orders: Value,
    stopped: Option<&'static str>,
    /// The askings that stopped it before a day (decision 0214, rule 7).
    asking: Vec<Value>,
}

/// What a call of `advance` is answered from.
struct AdvanceWait {
    pulses: u32,
    stop_on: Vec<&'static str>,
    /// Where the clock stood at the call, in words.
    started: String,
    resolved: Vec<Value>,
    /// What the side was told in the last pulse resolved.
    told: Vec<Value>,
    last_closed: Option<u32>,
    stopped: Option<&'static str>,
    /// The askings that stopped it at dusk, before a day (decision 0214,
    /// rule 7).
    asking: Vec<Value>,
}

/// What a seat's call in a watched game gives (decision 0182, rule 6): an
/// answer now, or nothing yet, the call of time being held until a step of
/// the watcher's brings its stop.
#[derive(Clone, PartialEq, Debug)]
pub enum Called {
    Answered(Result<Value, CallError>),
    Held,
}

/// What a step of the watcher's gives (decision 0182, amended 14): its own
/// answer, or why it was refused, and the held calls whose stops it brought,
/// each with its seat, the Allied seat's first (amended 7).
#[derive(Clone, PartialEq, Debug)]
pub struct Stepped {
    pub answer: Result<Value, CallError>,
    pub answered: Vec<(Side, Result<Value, CallError>)>,
}

/// What a call of time is when it is made: held, or answered at once.
enum Start {
    Hold(Wait),
    Now(Value),
}

/// A game for play: the world, who holds each seat, the days it runs, and what
/// is kept of it (the orders as handed in, the transcript).
pub struct Session {
    world: World,
    scenario: String,
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
    /// The previews shown, each side's numbered 1, 2, 3... among its own
    /// (decision 0172, 4): what a seat is given never counts the other
    /// side's.
    previews: Vec<PreviewRecord>,
    /// Whether a seat may stop the clock between pulses and give orders
    /// inside a day: the scenario's key (decision 0156).
    inside_day: bool,
    /// The script's orders inside a day, for the seats the script holds.
    script_inside: OrdersInsideDay,
    /// The orders handed in to the engine inside a day, with their pulses,
    /// for the saved game.
    handed_in_inside: OrdersInsideDay,
    /// The day the game stopped inside, and the words every later call is
    /// given, once the engine could not resolve a day (decision 0172, 3).
    /// The world is then half advanced: nothing reads or advances it again.
    stopped: Option<(u32, String)>,
    /// What plays a seat's trials, where the caller handed one in: only in a
    /// game that has a trial (decision 0178, rule 6).
    trial: Option<Box<dyn TrialRunner>>,
    /// The estimates the seats were shown (rule 19).
    trials_shown: Vec<TrialRecord>,
    /// Whether the game is watched: a third party, the watcher, holds the
    /// clock, and a seat's calls of time wait on his steps (decision 0182).
    watched: bool,
    /// For each side, the day a stop in the evening was last given before
    /// (decision 0214, rule 7): that day is played the next time the side
    /// calls time, so an asking left unanswered does not stop it again.
    pursuit_stops: BTreeMap<Side, u32>,
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
    /// `scenario` is the name the briefing's heading carries. The days are
    /// handed to the world, which holds the game's length as configuration
    /// (decision 0194, 12), and the session reads them from it: the
    /// briefing, the end of the game and each side's view give one figure.
    pub fn new(
        mut world: World,
        scenario: &str,
        days: u32,
        allied: Seat,
        japanese: Seat,
    ) -> Session {
        world.set_game_days(days);
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
                    held: None,
                    last_stop: 0,
                },
            );
        }
        Session {
            world,
            scenario: scenario.to_string(),
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
            stopped: None,
            trial: None,
            trials_shown: Vec::new(),
            watched: false,
            pursuit_stops: BTreeMap::new(),
        }
    }

    /// This session watched (decision 0182): the watcher holds the clock and
    /// moves it by [`Session::step`], each played seat's call of time is
    /// held until its stop comes, and `run` and `advance` are offered to
    /// every played seat. Nothing in the game is changed by it.
    pub fn watched(mut self) -> Session {
        self.watched = true;
        self
    }

    /// Whether the game is watched.
    pub fn is_watched(&self) -> bool {
        self.watched
    }

    /// This session with what plays a seat's trials (decision 0178, rule
    /// 6): the caller hands one in where the scenario names a file of the
    /// trial's figures, as it hands in the staffs. A session with none has
    /// no trial, and its seats are not given the tools `trial` and
    /// `trials`.
    pub fn with_trial(mut self, runner: Box<dyn TrialRunner>) -> Session {
        self.trial = Some(runner);
        self
    }

    /// Whether the game has a trial.
    pub fn has_trial(&self) -> bool {
        self.trial.is_some()
    }

    /// Whether the side's seat has the tools `trial` and `trials`: a seat
    /// with a staff that plays a whole side, in a game that has a trial
    /// (decision 0178, rule 7).
    pub fn has_trial_tools(&self, side: Side) -> bool {
        self.trial.is_some() && self.has_command_tools(side)
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
    /// pulses and `run` is offered.
    fn advance_offered(&self) -> bool {
        self.inside_day && self.run_offered()
    }

    /// Whether `run` is offered to a seat with the command tools: exactly
    /// one seat is a player's (decision 0107, item 29), or the game is
    /// watched, where it is offered to every played seat (decision 0182,
    /// rule 5).
    fn run_offered(&self) -> bool {
        self.player_seats() == 1 || self.watched
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
            // player (decision 0107, item 29), or in a watched game
            // (decision 0182, rule 5).
            let runnable = self.run_offered();
            tools.extend(
                COMMAND_TOOLS
                    .iter()
                    .filter(|tool| **tool != "run" || runnable),
            );
        }
        // `trial` and `trials` where the game has a trial, listed with the
        // tools of command (decision 0178, rule 7).
        if self.has_trial_tools(side) {
            tools.extend(TRIAL_TOOLS);
        }
        // `advance` is offered where the scenario lets a seat stop between
        // pulses and one seat is a player's (decision 0156, rule 8), or the
        // game is watched (decision 0182, rule 5).
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
        self.world
            .game_days()
            .expect("a session gives its world the game's days")
    }

    pub fn is_over(&self) -> bool {
        self.world.day() >= self.days()
    }

    /// The day the game stopped inside, if the engine could not resolve it
    /// (decision 0172, 3). A stopped game is not over and goes no further:
    /// every call is answered with the words of the stop.
    pub fn stopped_inside_day(&self) -> Option<u32> {
        self.stopped.as_ref().map(|(day, _)| *day)
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
    /// because it is something the player was told. In a watched game a
    /// call of time is held (decision 0182): use [`Session::seat_call`],
    /// which says so; here such a call is held all the same, and answered
    /// with an error that says where its answer comes from.
    pub fn call(&mut self, side: Side, tool: &str, arguments: &Value) -> Result<Value, CallError> {
        if !self.watched {
            return self.call_now(side, tool, arguments);
        }
        match self.seat_call(side, tool, arguments) {
            Called::Answered(outcome) => outcome,
            Called::Held => Err(CallError::new(
                "the call is held: in a watched game a call of time is answered when a step of the watcher's brings its stop",
            )),
        }
    }

    /// The refusal of a call to a seat the script holds.
    fn script_refusal(&self, side: Side) -> CallError {
        let players: Vec<&str> = Side::ALL
            .into_iter()
            .filter(|s| self.seats[s].seat.kind() == SeatKind::Player)
            .map(seat_name)
            .collect();
        CallError::naming(
            format!(
                "the {} seat is held by the script and cannot be called",
                seat_name(side)
            ),
            &players,
        )
    }

    /// A call answered at once, as every call of a game that is not watched
    /// is.
    fn call_now(&mut self, side: Side, tool: &str, arguments: &Value) -> Result<Value, CallError> {
        if self.seats[&side].seat.kind() != SeatKind::Player {
            return Err(self.script_refusal(side));
        }
        let after_day = self.world.day();
        // A game stopped inside a day is neither read nor advanced again:
        // every call is given the words of the stop (decision 0172, 3).
        let outcome = match &self.stopped {
            Some((_, words)) => Err(CallError::new(words.clone())),
            None => self.dispatch(side, tool, arguments),
        };
        self.enter(side, after_day, tool, arguments, &outcome);
        outcome
    }

    /// Enters a call and its answer in the transcript; its number there.
    fn enter(
        &mut self,
        side: Side,
        after_day: u32,
        tool: &str,
        arguments: &Value,
        outcome: &Result<Value, CallError>,
    ) -> u32 {
        let answer = match outcome {
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
        seq
    }

    // -----------------------------------------------------------------
    // A game watched (decision 0182).
    // -----------------------------------------------------------------

    /// A call from a seat in a watched game (decision 0182, rules 5 to 8):
    /// a call of time (`end_day`, `run`, `advance`) whose arguments stand is
    /// held, and the seat is ready until its stop; a call refused for its
    /// arguments is answered at once. While the seat is ready any other call
    /// of it is refused, saying so (amended 8). Every other call is answered
    /// as [`Session::call`] answers it. In a game that is not watched this is
    /// [`Session::call`].
    pub fn seat_call(&mut self, side: Side, tool: &str, arguments: &Value) -> Called {
        if !self.watched {
            return Called::Answered(self.call_now(side, tool, arguments));
        }
        if self.seats[&side].seat.kind() != SeatKind::Player {
            return Called::Answered(Err(self.script_refusal(side)));
        }
        let after_day = self.world.day();
        let outcome = if let Some((_, words)) = &self.stopped {
            Err(CallError::new(words.clone()))
        } else if let Some(held) = &self.seats[&side].held {
            Err(CallError::new(format!(
                "the {} seat is ready and waits for the clock: its call of {} is answered when the clock reaches its stop, and until then it makes no other call",
                seat_name(side),
                held.tool
            )))
        } else if self.is_time_tool(side, tool) {
            match self.start_time(side, tool, arguments) {
                Ok(Start::Hold(wait)) => {
                    self.seats.get_mut(&side).expect("both seats exist").held = Some(Held {
                        tool: tool.to_string(),
                        arguments: arguments.clone(),
                        after_day,
                        wait,
                    });
                    return Called::Held;
                }
                Ok(Start::Now(answer)) => Ok(answer),
                Err(error) => Err(error),
            }
        } else {
            self.dispatch(side, tool, arguments)
        };
        self.enter(side, after_day, tool, arguments, &outcome);
        Called::Answered(outcome)
    }

    /// Whether `tool` is a tool of time the seat has, which a watched game
    /// holds.
    fn is_time_tool(&self, side: Side, tool: &str) -> bool {
        match tool {
            "end_day" => true,
            "run" => self.has_command_tools(side),
            "advance" => self.advance_offered(),
            _ => false,
        }
    }

    /// The call of a tool of time in a watched game: its arguments checked
    /// as the tool checks them, and what it waits for, or its answer now
    /// where it waits for nothing (`end_day` once the game is over).
    fn start_time(&self, side: Side, tool: &str, arguments: &Value) -> Result<Start, CallError> {
        let args = args_of(arguments)?;
        match tool {
            "end_day" if self.is_over() => Ok(Start::Now(self.end_day_answer(side, false)?)),
            "end_day" => Ok(Start::Hold(Wait::EndDay)),
            "run" => Ok(Start::Hold(Wait::Run(self.run_wait(side, &args)?))),
            _ => Ok(Start::Hold(Wait::Advance(self.advance_wait(&args)?))),
        }
    }

    /// The watcher's step (decision 0182, rules 2 and 3, amended 14): one
    /// pulse where the scenario has orders inside a day, one day where it
    /// has not. It is refused in a game that is not watched, once the game
    /// is over or stopped, and while a played seat is not ready, naming the
    /// seats waited for; then nothing moves. After it each held call is read
    /// for its stop, the Allied seat's first, and those whose stops came are
    /// answered. If the engine stops on an error every held call is answered
    /// with the words of the stop (amended 6).
    pub fn step(&mut self) -> Stepped {
        let refused = |error: CallError| Stepped {
            answer: Err(error),
            answered: Vec::new(),
        };
        if !self.watched {
            return refused(CallError::new(
                "this game is not watched: there is no watcher's step, and the seats move the clock",
            ));
        }
        if let Some((_, words)) = &self.stopped {
            return refused(CallError::new(words.clone()));
        }
        if self.is_over() {
            return refused(CallError::new("the game is over: there is no step to take"));
        }
        let waited = self.waited_for();
        if !waited.is_empty() {
            return refused(CallError::new(format!(
                "the clock waits for the {} seat{}: {} not ready, and the clock moves only when every played seat is",
                waited.join(" and the "),
                if waited.len() == 1 { "" } else { "s" },
                if waited.len() == 1 { "it is" } else { "they are" }
            )));
        }
        // Decision 0214, rule 7: at dusk, before a day is opened, a held
        // call of `run` or `advance` whose side has a force asking leave to
        // pursue stops there: the seat is answered and nothing is resolved.
        if self.open_day().is_none() {
            let mut answered = Vec::new();
            for side in Side::ALL {
                let Some(mut held) = self.take_held(side) else {
                    continue;
                };
                let asking = match held.wait {
                    Wait::Run(_) | Wait::Advance(_) => self.stop_before_day(side),
                    Wait::EndDay => None,
                };
                let Some(asking) = asking else {
                    self.seats.get_mut(&side).expect("both seats exist").held = Some(held);
                    continue;
                };
                let answer = match &mut held.wait {
                    Wait::Run(wait) => {
                        wait.asking = asking;
                        wait.stopped = Some("asks_leave");
                        self.run_answer(side, wait)
                    }
                    Wait::Advance(wait) => {
                        wait.asking = asking;
                        wait.stopped = Some("asks_leave");
                        self.advance_answer(side, wait)
                    }
                    Wait::EndDay => unreachable!("end_day is not stopped by an asking"),
                };
                let answer = self.answer_held(side, held, answer);
                answered.push((side, answer));
            }
            if !answered.is_empty() {
                let stops: Vec<&str> = answered.iter().map(|(side, _)| seat_name(*side)).collect();
                let summary = format!(
                    "Resolved nothing; the clock stands at {}. {} stop came before the day: a force asks leave to pursue.",
                    self.clock_words(),
                    match stops.as_slice() {
                        [one] => format!("The {one} seat's"),
                        _ => format!("The {} seats'", stops.join(" and ")),
                    }
                );
                return Stepped {
                    answer: Ok(json!({
                        "format": FORMAT,
                        "version": VERSION,
                        "kind": "step",
                        "resolved": {"day": self.world.day(), "pulses": Vec::<&str>::new()},
                        "clock": self.clock(),
                        "stops": stops,
                        "game_over": self.is_over(),
                        "summary": summary,
                    })),
                    answered,
                };
            }
        }
        // What each side held before the step: what it is told in the pulse
        // is its events after these, as `advance` reads them.
        let before: BTreeMap<Side, usize> = Side::ALL
            .into_iter()
            .map(|side| {
                (
                    side,
                    self.view_of(side).map_or(0, |view| view.events().len()),
                )
            })
            .collect();
        let mut pulses: Vec<Pulse> = Vec::new();
        let mut day = 0;
        let outcome = loop {
            if self.open_day().is_none() {
                if let Err(error) = self.begin_day() {
                    break Err(error);
                }
            }
            match self.resolve_pulse() {
                Ok((d, pulse)) => {
                    day = d;
                    pulses.push(pulse);
                }
                Err(error) => break Err(error),
            }
            let closed = self.open_day().is_none();
            // A step is a pulse where a seat may stop between pulses, else
            // a whole day.
            if self.inside_day || closed {
                break Ok(closed);
            }
        };
        let mut answered = Vec::new();
        let closed = match outcome {
            Ok(closed) => closed,
            Err(error) => {
                for side in Side::ALL {
                    if let Some(held) = self.take_held(side) {
                        let answer = self.answer_held(side, held, Err(error.clone()));
                        answered.push((side, answer));
                    }
                }
                return Stepped {
                    answer: Err(error),
                    answered,
                };
            }
        };
        let pulse = *pulses.last().expect("a step resolves a pulse");
        for side in Side::ALL {
            let Some(mut held) = self.take_held(side) else {
                continue;
            };
            let came = match &mut held.wait {
                Wait::EndDay => closed,
                Wait::Run(wait) => closed && self.run_after_day(side, wait),
                Wait::Advance(wait) => {
                    self.advance_after_pulse(side, wait, day, pulse, closed, before[&side])
                }
            };
            if !came {
                self.seats.get_mut(&side).expect("both seats exist").held = Some(held);
                continue;
            }
            let answer = match &held.wait {
                Wait::EndDay => self.end_day_answer(side, true),
                Wait::Run(wait) => self.run_answer(side, wait),
                Wait::Advance(wait) => self.advance_answer(side, wait),
            };
            let answer = self.answer_held(side, held, answer);
            answered.push((side, answer));
        }
        let stops: Vec<&str> = answered.iter().map(|(side, _)| seat_name(*side)).collect();
        let resolved_words = match pulses.as_slice() {
            [one] => format!("the {} pulse of day {day}", one.name()),
            _ => format!("day {day}"),
        };
        let summary = format!(
            "Resolved {resolved_words}; the clock stands at {}.{}",
            self.clock_words(),
            match stops.as_slice() {
                [] => String::new(),
                [one] => format!(" The {one} seat's stop came."),
                _ => format!(" The {} seats' stops came.", stops.join(" and ")),
            }
        );
        Stepped {
            answer: Ok(json!({
                "format": FORMAT,
                "version": VERSION,
                "kind": "step",
                "resolved": {
                    "day": day,
                    "pulses": pulses.iter().map(|p| p.name()).collect::<Vec<_>>(),
                },
                "clock": self.clock(),
                "stops": stops,
                "game_over": self.is_over(),
                "summary": summary,
            })),
            answered,
        }
    }

    /// The played seats that are not ready, by name, the Allied first.
    fn waited_for(&self) -> Vec<&'static str> {
        Side::ALL
            .into_iter()
            .filter(|side| {
                let state = &self.seats[side];
                state.seat.kind() == SeatKind::Player && state.held.is_none()
            })
            .map(seat_name)
            .collect()
    }

    fn take_held(&mut self, side: Side) -> Option<Held> {
        self.seats
            .get_mut(&side)
            .expect("both seats exist")
            .held
            .take()
    }

    /// Answers a held call: its entry is written in the transcript now, with
    /// the days resolved when it was made (amended 9), and the seat's last
    /// stop is this one.
    fn answer_held(
        &mut self,
        side: Side,
        held: Held,
        outcome: Result<Value, CallError>,
    ) -> Result<Value, CallError> {
        let seq = self.enter(side, held.after_day, &held.tool, &held.arguments, &outcome);
        self.seats
            .get_mut(&side)
            .expect("both seats exist")
            .last_stop = seq;
        outcome
    }

    /// The tools the watcher reads with from the side's seat (decision 0182,
    /// amended 13): those of [`WATCHER_TOOLS`] the seat has.
    pub fn watcher_tools(&self, side: Side) -> Vec<&'static str> {
        let seat = self.tools(side);
        WATCHER_TOOLS
            .iter()
            .copied()
            .filter(|tool| seat.contains(tool))
            .collect()
    }

    /// A read of the watcher's (decision 0182, rules 10 to 12): the side's
    /// seat's read tool, answered as that seat would be answered now. It is
    /// entered in no transcript, and none of these tools changes the game,
    /// the queues or what a preview or a trial is numbered. Any other tool
    /// is refused as one he does not have.
    pub fn watch(&mut self, side: Side, tool: &str, arguments: &Value) -> Result<Value, CallError> {
        if !self.watched {
            return Err(CallError::new(
                "this game is not watched: there is no watcher to read",
            ));
        }
        let tools = self.watcher_tools(side);
        if !tools.contains(&tool) {
            return Err(CallError::naming(
                format!(
                    "\"{tool}\" is not a tool of the watcher's from the {} seat: he reads and changes nothing",
                    seat_name(side)
                ),
                &tools,
            ));
        }
        if let Some((_, words)) = &self.stopped {
            return Err(CallError::new(words.clone()));
        }
        self.dispatch(side, tool, arguments)
    }

    /// Where the game stands, for the watcher (decision 0182, rule 13): the
    /// clock, whether the game is over or stopped, and each seat: who holds
    /// it, whether it is ready and until when, or that it is waited for.
    pub fn standing(&self) -> Value {
        let over = self.is_over() || self.stopped.is_some();
        let seats: Vec<Value> = Side::ALL
            .into_iter()
            .map(|side| {
                let state = &self.seats[&side];
                let kind = state.seat.kind();
                let until = state.held.as_ref().map(|held| self.until(held));
                let waited = kind == SeatKind::Player && state.held.is_none() && !over;
                json!({
                    "seat": seat_name(side),
                    "holder": kind.name(),
                    "ready": kind == SeatKind::Script || state.held.is_some(),
                    "until": until,
                    "waited_for": waited,
                })
            })
            .collect();
        let waiting: Vec<&str> = if over { Vec::new() } else { self.waited_for() };
        json!({
            "format": FORMAT,
            "version": VERSION,
            "kind": "standing",
            "clock": self.clock(),
            "game_over": self.is_over(),
            "stopped": self.stopped.as_ref().map(|(_, words)| words.clone()),
            "seats": seats,
            "waiting_for": waiting,
        })
    }

    /// A held call's stop, for the standing.
    fn until(&self, held: &Held) -> Value {
        let dusk = match self.open_day() {
            Some((day, _)) => day,
            None => self.world.day() + 1,
        };
        match &held.wait {
            Wait::EndDay => json!({
                "tool": "end_day",
                "words": format!("until the dusk at the end of day {dusk}"),
            }),
            Wait::Run(wait) => json!({
                "tool": "run",
                "days": wait.days,
                "days_resolved": wait.count,
                "words": format!(
                    "until the dusk at the end of day {}, or a dusk after a day on which an interrupt of its side fired",
                    wait.first + wait.days - 1
                ),
            }),
            Wait::Advance(wait) => {
                let done = u32::try_from(wait.resolved.len()).expect("few pulses");
                let left = wait.pulses.saturating_sub(done);
                let told = if wait.stop_on.is_empty() {
                    String::new()
                } else {
                    format!(
                        ", or the end of a pulse in which its side is told of {}",
                        wait.stop_on.join(", ")
                    )
                };
                json!({
                    "tool": "advance",
                    "pulses": wait.pulses,
                    "pulses_resolved": done,
                    "stop_on": wait.stop_on,
                    "words": format!(
                        "for {left} more pulse{}{told}, or a dusk after a day on which an interrupt of its side fired",
                        if left == 1 { "" } else { "s" }
                    ),
                })
            }
        }
    }

    /// What a seat did since its last stop, for the watcher (decision 0182,
    /// rule 14): its calls in the transcript after its last call of time
    /// answered, each with its tool, its arguments, whether it was refused
    /// and its `reason` where it gave one; and the call of time it holds,
    /// if any, which is in the transcript only once it is answered.
    pub fn seat_calls(&self, side: Side) -> Value {
        let state = &self.seats[&side];
        let reason = |arguments: &Value| match arguments.get("reason") {
            Some(Value::String(reason)) => json!(reason),
            _ => Value::Null,
        };
        let calls: Vec<Value> = self
            .transcript
            .iter()
            .filter(|entry| entry.seat == side && entry.seq > state.last_stop)
            .map(|entry| {
                json!({
                    "seq": entry.seq,
                    "tool": entry.tool,
                    "arguments": entry.arguments,
                    "refused": entry.answer.get("kind") == Some(&json!("error")),
                    "reason": reason(&entry.arguments),
                })
            })
            .collect();
        let held = state.held.as_ref().map(|held| {
            json!({
                "tool": held.tool,
                "arguments": held.arguments,
                "reason": reason(&held.arguments),
            })
        });
        json!({
            "format": FORMAT,
            "version": VERSION,
            "kind": "seat_calls",
            "seat": seat_name(side),
            "since": state.last_stop,
            "calls": calls,
            "held": held,
        })
    }

    /// The day a watched game was left inside, while a day is open: the
    /// saved game says it stopped inside it (decision 0182, amended 19).
    /// `None` in a game that is not watched, and at dusk.
    pub fn left_inside_day(&self) -> Option<u32> {
        if !self.watched {
            return None;
        }
        self.open_day().map(|(day, _)| day)
    }

    /// The side's seat's queues as they stand: its orders for the coming
    /// day, its orders for the next pulse of the open day and its commands.
    /// For the runner and tests; no tool gives it.
    pub fn queue_of(&self, side: Side) -> (Vec<OrderRequest>, Vec<OrderRequest>, Vec<CommandKind>) {
        let state = &self.seats[&side];
        (
            state.queued.clone(),
            state.queued_inside.clone(),
            state.commands.clone(),
        )
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
                    briefing_for_game(view, &self.scenario, Some(self.days()))
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
                    last_day: Some(self.days()),
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
            "trial" if self.has_trial_tools(side) => self.trial_tool(side, &args),
            "trials" if self.has_trial_tools(side) => self.trials_tool(side, &args),
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
    /// the game is over (decision 0156, rule 8). In a watched game the same
    /// call is held and the watcher's steps resolve the pulses (decision
    /// 0182): the call is [`Session::advance_wait`], each pulse
    /// [`Session::advance_after_pulse`] and the answer
    /// [`Session::advance_answer`], here and there alike.
    fn advance(
        &mut self,
        side: Side,
        args: &serde_json::Map<String, Value>,
    ) -> Result<Value, CallError> {
        let mut wait = self.advance_wait(args)?;
        loop {
            // What the side held before the pulse: what it is told in it is
            // its events after these.
            let before = self.view_of(side).map_or(0, |view| view.events().len());
            if self.open_day().is_none() {
                // Decision 0214, rule 7: at dusk, before the day opens.
                if let Some(asking) = self.stop_before_day(side) {
                    wait.asking = asking;
                    wait.stopped = Some("asks_leave");
                    break;
                }
                self.begin_day()?;
            }
            let (day, pulse) = self.resolve_pulse()?;
            let closed = self.open_day().is_none();
            if self.advance_after_pulse(side, &mut wait, day, pulse, closed, before) {
                break;
            }
        }
        self.advance_answer(side, &wait)
    }

    /// The call of `advance`: its arguments checked, and where the clock
    /// stands when it is made.
    fn advance_wait(
        &self,
        args: &serde_json::Map<String, Value>,
    ) -> Result<AdvanceWait, CallError> {
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
        Ok(AdvanceWait {
            pulses,
            stop_on,
            started: self.clock_words(),
            resolved: Vec::new(),
            told: Vec::new(),
            last_closed: None,
            stopped: None,
            asking: Vec::new(),
        })
    }

    /// One pulse resolved under a call of `advance`: the pulse `day`,
    /// `pulse`, whether it closed the day, and the count of the side's
    /// events before it. True when the call's stop has come.
    fn advance_after_pulse(
        &self,
        side: Side,
        wait: &mut AdvanceWait,
        day: u32,
        pulse: Pulse,
        closed: bool,
        before: usize,
    ) -> bool {
        wait.resolved
            .push(json!({"day": day, "pulse": pulse.name()}));
        if closed {
            wait.last_closed = Some(day);
        }
        wait.told = match self.view_of(side) {
            Ok(view) => told_since(&view, before, &wait.stop_on),
            Err(_) => Vec::new(),
        };
        let stopped = if self.is_over() {
            "game_over"
        } else if closed && staffing::fired_on_day(&self.world.side_view(side), day) {
            "interrupt"
        } else if !wait.told.is_empty() {
            "told"
        } else if u32::try_from(wait.resolved.len()).expect("few pulses") >= wait.pulses {
            "pulses"
        } else {
            return false;
        };
        wait.stopped = Some(stopped);
        true
    }

    /// The answer of `advance` once its stop has come.
    fn advance_answer(&self, side: Side, wait: &AdvanceWait) -> Result<Value, CallError> {
        let stopped = wait.stopped.unwrap_or("pulses");
        let (pulses, stop_on, started, resolved, told) = (
            wait.pulses,
            &wait.stop_on,
            &wait.started,
            &wait.resolved,
            &wait.told,
        );
        let fired = match wait.last_closed {
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
            "asks_leave" => pursuit::asking_words(&wait.asking),
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
        let mut body = json!({
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
        });
        if stopped == "asks_leave" {
            body["asking"] = json!(wait.asking);
        }
        Ok(body)
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
                                result["returns"] = json!(day <= self.days());
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
            "task_force" => ctx
                .own_force_or_held(number)
                .map(|f| f.id.raw())
                .or_else(|| {
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
            "land_unit" => ctx.own_land_unit_or_held(number).map(|u| u.id.raw()),
            _ => ctx.own_group_or_held(number).map(|g| g.id.raw()),
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

    /// The stop in the evening (decision 0214, rule 7): at dusk, before a
    /// day is opened, the askings of a force to go after what it has found
    /// that the staff would hand in for the coming day, worked on the preview
    /// with the side's queued commands taken in, where one would fire the
    /// standing interrupt. `None` where nothing stops the clock. Nothing in
    /// the world changes; the day it stopped before is kept, so the next call
    /// of time plays that day.
    fn stop_before_day(&mut self, side: Side) -> Option<Vec<Value>> {
        if self.open_day().is_some() || self.is_over() || self.at_a_command(side) {
            return None;
        }
        let day = self.world.day() + 1;
        if self.pursuit_stops.get(&side) == Some(&day) {
            return None;
        }
        let staff = self.staffs.get(&side)?;
        let world = self.preview();
        let asking = pursuit::askings(
            &self.world.side_view(side),
            &world.side_view(side),
            staff.as_ref(),
        );
        if asking.is_empty() {
            return None;
        }
        self.pursuit_stops.insert(side, day);
        Some(asking)
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
            "order" => {
                let intent = args
                    .get("intent")
                    .and_then(Value::as_u64)
                    .and_then(|n| u32::try_from(n).ok())
                    .ok_or_else(|| {
                        CallError::new("\"intent\" is needed, as the number of an intent")
                    })?;
                let world = self.preview();
                let answer = staffing::plan_order(&world.side_view(side), staff, intent)?;
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
        let shown = self.previews.iter().filter(|p| p.side == side).count();
        let number = u32::try_from(shown + 1).expect("few previews");
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
    // A seat's trial (decision 0178).
    // -----------------------------------------------------------------

    /// `trial`: an intent of the commander's, or a draft he previewed, tried
    /// in the worlds of a trial at a size, and answered with the estimate in
    /// a commander's words, or with why it cannot be tried (decision 0178,
    /// rules 7 to 19). With `compare` it is set beside the side without the
    /// thing tried; with `dates` it is a window; with `describe` nothing is
    /// played and the answer says what would be (decision 0181). It changes
    /// nothing in the game: no command is queued, no preview is spent, and
    /// the hash is what it was (rule 14). Asked inside a day, at 0600 or
    /// 1200, it is begun on the state as it stands with the open day treated
    /// as closed, and its first day is the day after (decision 0204).
    fn trial_tool(
        &mut self,
        side: Side,
        args: &serde_json::Map<String, Value>,
    ) -> Result<Value, CallError> {
        const ARGUMENTS: [&str; 6] = ["intent", "preview", "size", "compare", "dates", "describe"];
        if let Some(key) = args.keys().find(|k| !ARGUMENTS.contains(&k.as_str())) {
            return Err(CallError::naming(
                format!("\"{key}\" is not an argument of trial"),
                &ARGUMENTS,
            ));
        }
        if self.is_over() {
            return Err(CallError::new(
                "the game is over: there is no coming morning to try",
            ));
        }
        let number = |key: &str| -> Result<Option<u32>, CallError> {
            match args.get(key) {
                None | Some(Value::Null) => Ok(None),
                Some(value) => value
                    .as_u64()
                    .and_then(|n| u32::try_from(n).ok())
                    .map(Some)
                    .ok_or_else(|| CallError::new(format!("\"{key}\" must be a whole number"))),
            }
        };
        let flag = |key: &str| -> Result<Option<bool>, CallError> {
            match args.get(key) {
                None | Some(Value::Null) => Ok(None),
                Some(Value::Bool(b)) => Ok(Some(*b)),
                Some(_) => Err(CallError::new(format!("\"{key}\" must be true or false"))),
            }
        };
        let (intent_arg, preview_arg) = (number("intent")?, number("preview")?);
        let size = match args.get("size") {
            None | Some(Value::Null) => "small".to_string(),
            Some(Value::String(size)) => size.clone(),
            Some(_) => {
                return Err(CallError::naming(
                    "\"size\" must be the name of a size of a trial",
                    &["small", "standard", "full"],
                ))
            }
        };
        let compare_arg = flag("compare")?;
        let describe = flag("describe")?.unwrap_or(false);
        let day = self.command_day();
        // `dates` (decision 0181, rule 9 and amended 8): `chosen`, or one or
        // two days after the coming morning, ascending; the coming morning
        // may lead the list and is dropped, for it is the first date in any
        // case.
        let dates = match args.get("dates") {
            None | Some(Value::Null) => DatesAsked::Coming,
            Some(Value::String(word)) if word == "chosen" => DatesAsked::Chosen,
            Some(Value::Array(items)) => {
                let mut days = Vec::new();
                for item in items {
                    let Some(d) = item.as_u64().and_then(|n| u32::try_from(n).ok()) else {
                        return Err(CallError::new(
                            "\"dates\" must be \"chosen\" or a list of days of the game, each a whole number",
                        ));
                    };
                    days.push(d);
                }
                if days.first() == Some(&day) {
                    days.remove(0);
                }
                if days.is_empty() || days.len() > 2 {
                    return Err(CallError::new(format!(
                        "\"dates\" gives {} day{} after the coming morning, day {day}: a window is tried at the coming morning and one or two later days",
                        days.len(),
                        if days.len() == 1 { "" } else { "s" }
                    )));
                }
                if let Some(early) = days.iter().find(|d| **d < day) {
                    return Err(CallError::new(format!(
                        "\"dates\" gives day {early}, before the coming morning, day {day}: a date is the coming morning or a day after it"
                    )));
                }
                if days.windows(2).any(|pair| pair[0] >= pair[1]) {
                    return Err(CallError::new(
                        "the days of \"dates\" are not each after the one before: give them in order, each once",
                    ));
                }
                if let Some(late) = days.iter().find(|d| **d > self.days()) {
                    return Err(CallError::new(format!(
                        "\"dates\" gives day {late}, after the game's last day, day {}",
                        self.days()
                    )));
                }
                days.insert(0, day);
                DatesAsked::Given(days)
            }
            Some(_) => {
                return Err(CallError::new(
                    "\"dates\" must be \"chosen\" or a list of one or two days of the game after the coming morning",
                ))
            }
        };
        let window = !matches!(dates, DatesAsked::Coming);
        if window && compare_arg == Some(true) {
            return Err(CallError::new(
                "a window has no second trial: give \"dates\" or \"compare\" true, not both",
            ));
        }
        match (intent_arg, preview_arg) {
            (Some(_), Some(_)) => {
                return Err(CallError::new(
                    "a trial is of an intent or of a preview, not both: give \"intent\" or \"preview\"",
                ))
            }
            (None, None) => {
                return Err(CallError::new(
                    "\"intent\" or \"preview\" is needed: the number of an intent of yours, or of a preview you were shown",
                ))
            }
            _ => {}
        }
        // The standing commands (rule 8): the side's commands of that day in
        // the scenario, then what its seat has queued, in that order.
        let mut standing: Vec<CommandKind> = self
            .commands
            .get(&day)
            .into_iter()
            .flatten()
            .filter(|(who, _)| *who == side)
            .map(|(_, command)| command.clone())
            .collect();
        standing.extend(self.seats[&side].commands.iter().cloned());

        // What is tried: the intent, or the preview's command, read on the
        // world as the coming day finds it, with what is queued.
        let coming = self.preview();
        let coming_view = coming.side_view(side);
        let state = coming_view.command_state();
        let (tried, draft, tried_words) = match (intent_arg, preview_arg) {
            (Some(raw), _) => {
                let id = IntentId::new(raw);
                if standing
                    .iter()
                    .any(|c| matches!(c, CommandKind::WithdrawIntent { intent } if *intent == id))
                {
                    return Err(CallError::new(format!(
                        "intent {raw} has its withdrawal queued: it cannot be tried"
                    )));
                }
                let Some(held) = state.intents.get(&id) else {
                    return Err(CallError::new(format!(
                        "intent {raw} is not an intent of yours (command_state lists your intents)"
                    )));
                };
                if held.maker != Maker::Commander {
                    return Err(CallError::new(format!(
                        "intent {raw} is the staff's own, formed under an intent of yours: try the intent of yours it serves"
                    )));
                }
                if !held.in_force() {
                    return Err(CallError::new(format!("intent {raw} is withdrawn")));
                }
                let words = staffing::intent_words(&coming_view, held.kind);
                (id, None, format!("intent {raw}, {words}"))
            }
            (None, Some(raw)) => {
                let Some(record) = self
                    .previews
                    .iter()
                    .find(|p| p.number == raw && p.side == side)
                else {
                    return Err(CallError::new(format!(
                        "preview {raw} is not a preview you were shown"
                    )));
                };
                if record.day != day {
                    return Err(CallError::new(format!(
                        "preview {raw} is void: a day was resolved after it; preview it again"
                    )));
                }
                if record.moment != self.moment() || record.revision != self.revision(side) {
                    return Err(CallError::new(format!(
                        "preview {raw} is void: something was queued after it; preview it again"
                    )));
                }
                let command = record.request.kind.clone();
                let Some(id) = staffing::previewed_intent(&coming_view, &command) else {
                    return Err(CallError::new(format!(
                        "preview {raw} is of a limit on no intent: there is no intent to try"
                    )));
                };
                let words = command_words(&coming_view, &command);
                (id, Some(command), format!("preview {raw}, {words}"))
            }
            (None, None) => unreachable!("one of the two is given"),
        };
        // The world a trial is begun on (decision 0204, 2): the game as it
        // stands, with an open day treated as closed. The command state is
        // the same on it as in the game.
        let closed = self.closed_world();
        // Whether the intent tried stands already: in force in the game this
        // evening, or at the hour asked, or set among the standing commands.
        let in_force = closed
            .side_view(side)
            .command_state()
            .intents
            .get(&tried)
            .is_some_and(|i| i.in_force());
        let sets = |c: &CommandKind| match c {
            CommandKind::SetIntent { intent, .. } => *intent == tried,
            CommandKind::SetLimit(limit) => limit.intent() == Some(tried),
            _ => false,
        };
        let queued = !in_force && standing.iter().any(&sets);
        // What the second trial would be of (decision 0181, amended 1).
        let of = match &draft {
            None if in_force => BesideOf::Withdrawn,
            None => BesideOf::WithoutIt,
            Some(CommandKind::SetIntent { .. }) if !in_force && !queued => BesideOf::WithoutIt,
            Some(_) => BesideOf::AsItStands,
        };
        // Where `compare` is not given (amended 2).
        let other = state.intents_in_force().any(|i| {
            i.id != tried
                && i.maker == Maker::Commander
                && matches!(
                    i.kind,
                    IntentKind::TakePlace { .. }
                        | IntentKind::ControlIsland { .. }
                        | IntentKind::Hold { .. }
                )
        });
        let compare = compare_arg.unwrap_or(match of {
            _ if window => false,
            BesideOf::AsItStands => true,
            BesideOf::WithoutIt => other,
            BesideOf::Withdrawn => false,
        });
        // No later date for an order in force (amended 5).
        if window && in_force {
            return Err(CallError::new(format!(
                "intent {} is in force: an order in force cannot be given later. A landing in force is held to a day by a \"not before\", which can be previewed and compared",
                tried.raw()
            )));
        }
        // A window hands in the whole order at each date (amended 4); the
        // second trial of a queued order leaves it out (amended 1). Either
        // way the commands that set it move or go, and a standing command
        // left behind that names it is refused (amended 7).
        let moves = window || (compare && of == BesideOf::WithoutIt && queued);
        if moves {
            if let Some(after) = standing.iter().find(|c| {
                !sets(c)
                    && matches!(c, CommandKind::SetLimit(Limit::After { other: Some(o), .. }) if *o == tried)
            }) {
                return Err(CallError::new(format!(
                    "\"{}\" stands among your commands for the morning and names intent {}: the trial would {} intent {} and not that command, so the worlds would differ by more than the thing tried. It cannot be tried so",
                    command_words(&coming_view, after),
                    tried.raw(),
                    if window { "move" } else { "leave out" },
                    tried.raw()
                )));
            }
        }
        // The leaves the course gives (rule 9): each the side neither holds
        // nor has among the standing commands. Both trials are given them
        // (decision 0181, rule 4).
        let held = closed.side_view(side).command_state().leaves;
        let mut leaves = Vec::new();
        for (leave, has) in [(Leave::Prepare, held.prepare), (Leave::Launch, held.launch)] {
            let standing_gives = standing.iter().any(
                |c| matches!(c, CommandKind::SetLeave { leave: l, given: true } if *l == leave),
            );
            if !has && !standing_gives {
                leaves.push(CommandKind::SetLeave { leave, given: true });
            }
        }
        // The side tried carries its orders' settings (decision 0214, "Who
        // answers where no one is there"): where the course gives leave to
        // launch, the leave to pursue of an order with none set would read
        // as go, where it reads as ask in the game. Each is set as it reads
        // now, after the commands that make the order.
        let gives_launch = leaves.iter().any(|c| {
            matches!(
                c,
                CommandKind::SetLeave {
                    leave: Leave::Launch,
                    ..
                }
            )
        });
        let drafted = match &draft {
            Some(CommandKind::SetIntent { intent, .. }) => Some(*intent),
            _ => None,
        };
        let settings = |also: Option<IntentId>, except: Option<IntentId>| -> Vec<CommandKind> {
            if gives_launch {
                pursuit::trial_pursuit_settings(state, also, except)
            } else {
                Vec::new()
            }
        };
        let (first_standing, course): (Vec<CommandKind>, Vec<CommandKind>) = if window {
            let mut course: Vec<CommandKind> =
                standing.iter().filter(|c| sets(c)).cloned().collect();
            course.extend(draft.iter().cloned());
            course.extend(leaves.iter().cloned());
            course.extend(settings(drafted, None));
            (
                standing.iter().filter(|c| !sets(c)).cloned().collect(),
                course,
            )
        } else {
            let mut course: Vec<CommandKind> = draft.iter().cloned().collect();
            course.extend(leaves.iter().cloned());
            course.extend(settings(drafted, None));
            (standing.clone(), course)
        };
        let second = compare.then(|| {
            let (second_standing, mut second_course) = match of {
                BesideOf::AsItStands => (standing.clone(), Vec::new()),
                BesideOf::WithoutIt => (
                    standing.iter().filter(|c| !sets(c)).cloned().collect(),
                    Vec::new(),
                ),
                BesideOf::Withdrawn => (
                    standing.clone(),
                    vec![CommandKind::WithdrawIntent { intent: tried }],
                ),
            };
            second_course.extend(leaves.iter().cloned());
            // The second world has the order as it stands, or has it out.
            second_course.extend(settings(
                None,
                (of != BesideOf::AsItStands).then_some(tried),
            ));
            SecondAsked {
                of,
                standing: second_standing,
                course: second_course
                    .into_iter()
                    .map(|command| CourseStep { day: 1, command })
                    .collect(),
            }
        });
        let standing_words: Vec<String> = first_standing
            .iter()
            .map(|c| command_words(&coming_view, c))
            .collect();
        // The settings carried are the side's own, not something tried: they
        // are in the worlds and left out of the words (decision 0214).
        let course_words: Vec<String> = course
            .iter()
            .filter(|c| !matches!(c, CommandKind::SetPursuit { .. }))
            .map(|c| command_words(&coming_view, c))
            .collect();
        drop(coming);
        let asked = TrialAsked {
            trial: 1,
            size: SizeAsked::Named(size.clone()),
            standing: first_standing,
            course: course
                .into_iter()
                .map(|command| CourseStep { day: 1, command })
                .collect(),
            dates: dates.clone(),
            intent: tried,
            second,
        };
        let shown = AskedShown {
            tried: match (intent_arg, preview_arg) {
                (Some(raw), _) => format!("intent {raw}"),
                (_, Some(raw)) => format!("preview {raw}"),
                _ => String::new(),
            },
            intent: intent_arg,
            preview: preview_arg,
            tried_intent: tried.raw(),
            what: tried_words
                .split_once(", ")
                .map_or(tried_words.clone(), |(_, what)| what.to_string()),
            size: size.clone(),
            for_day: day,
            standing: standing_words,
            course: course_words,
            second_trial: asked.second.is_some(),
            // Inside a day the orders given for the day's pulses are left
            // out as well as those queued for the coming day (decision 0204,
            // 5).
            orders_queued: self.seats[&side].queued.len() + self.seats[&side].queued_inside.len(),
            beside: asked.second.as_ref().map(|s| s.of),
            in_force,
            dates,
            inside_day: self.open_day().map(|(day, pulse)| {
                (
                    day,
                    if pulse == Pulse::Afternoon {
                        "1200"
                    } else {
                        "0600"
                    },
                )
            }),
            ashore: closed
                .side_view(side)
                .land_units()
                .any(|unit| unit.men_ashore > 0),
        };
        if describe {
            let described = self.describe_trial(&closed, side, &asked)?;
            let fields = described_fields(&self.world.side_view(side), &shown, &described);
            return Ok(staffing::answer("trial_described", fields));
        }
        let answered = self.play_trial(&closed, side, &asked)?;
        drop(closed);
        let number = u32::try_from(self.trials_shown.iter().filter(|t| t.side == side).count() + 1)
            .expect("few trials");
        let out = match &answered {
            Answered::Trial(answer) => staffing::answer(
                "trial",
                answer_fields(&self.world.side_view(side), number, &shown, answer),
            ),
            Answered::Window(answer) => staffing::answer(
                "window",
                window_fields(&self.world.side_view(side), number, &shown, answer),
            ),
        };
        self.trials_shown.push(TrialRecord {
            number,
            side,
            moment: self.moment(),
            revision: self.revision(side),
            asked: out["asked"].clone(),
            answer: out.clone(),
        });
        Ok(out)
    }

    /// The world a trial is begun on (decision 0204, 2): the game itself at
    /// dusk, and inside a day the copy a preview is worked on, the state as
    /// it stands with the open day treated as closed
    /// ([`World::as_if_day_closed`]). The copy is never resolved and never
    /// hashed.
    fn closed_world(&self) -> Cow<'_, World> {
        if self.open_day().is_some() {
            Cow::Owned(self.world.as_if_day_closed())
        } else {
            Cow::Borrowed(&self.world)
        }
    }

    /// Begins a trial on the side's whole view of `world` (rule 10; the
    /// world of [`Session::closed_world`]), taken here, out of the dispatch
    /// of tools, and steps it to its end (rule 3).
    fn play_trial(
        &self,
        world: &World,
        side: Side,
        asked: &TrialAsked,
    ) -> Result<Answered, CallError> {
        let runner = self
            .trial
            .as_ref()
            .ok_or_else(|| CallError::new("this game has no trial"))?;
        let view = world.side_view(side);
        let mut under_way = runner
            .begin(&view, asked)
            .map_err(|refused| CallError::new(refused.0))?;
        loop {
            if let TrialStep::Done(answer) = under_way.step() {
                return Ok(*answer);
            }
        }
    }

    /// What a trial would play, on the side's whole view of `world` (the
    /// world of [`Session::closed_world`]), with nothing played (decision
    /// 0181, amended 9; decision 0204, 2).
    fn describe_trial(
        &self,
        world: &World,
        side: Side,
        asked: &TrialAsked,
    ) -> Result<crate::trial::Described, CallError> {
        let runner = self
            .trial
            .as_ref()
            .ok_or_else(|| CallError::new("this game has no trial"))?;
        runner
            .describe(&world.side_view(side), asked)
            .map_err(|refused| CallError::new(refused.0))
    }

    /// `trials`: the estimates the seat was shown, each with the evening it
    /// was asked on and what was asked, marked where it was asked on an
    /// earlier evening or before the queue changed; or one again by its
    /// number, unchanged (rule 19).
    fn trials_tool(
        &self,
        side: Side,
        args: &serde_json::Map<String, Value>,
    ) -> Result<Value, CallError> {
        if let Some(key) = args.keys().find(|k| k.as_str() != "estimate") {
            return Err(CallError::naming(
                format!("\"{key}\" is not an argument of trials"),
                &["estimate"],
            ));
        }
        match args.get("estimate") {
            None | Some(Value::Null) => {}
            Some(value) => {
                let number = value
                    .as_u64()
                    .and_then(|n| u32::try_from(n).ok())
                    .ok_or_else(|| {
                        CallError::new(
                            "\"estimate\" must be the number of an estimate you were shown",
                        )
                    })?;
                return self
                    .trials_shown
                    .iter()
                    .find(|t| t.side == side && t.number == number)
                    .map(|t| t.answer.clone())
                    .ok_or_else(|| {
                        CallError::new(format!(
                            "estimate {number} is not one you were shown (trials lists them)"
                        ))
                    });
            }
        }
        let now = self.moment();
        let revision = self.revision(side);
        let inside_now = self.open_day().is_some();
        let mut text = Vec::new();
        let listed: Vec<Value> = self
            .trials_shown
            .iter()
            .filter(|t| t.side == side)
            .map(|t| {
                // A trial asked at an earlier hour of the same day is
                // counted as one asked on an earlier evening (decision 0204,
                // 9).
                let inside = &t.asked["inside_day"];
                let (standing, note) = if t.moment.0 != now.0 || t.moment.1 != now.1 {
                    if t.moment.1.is_some() {
                        ("earlier_evening", "asked at an earlier hour")
                    } else {
                        ("earlier_evening", "asked on an earlier evening")
                    }
                } else if t.revision != revision {
                    ("queue_changed", "asked before your queue changed")
                } else if inside_now {
                    ("current", "asked at this hour, on the queue as it stands")
                } else {
                    ("current", "asked this evening, on the queue as it stands")
                };
                let for_day = t.asked["for_day"].as_u64().unwrap_or(0);
                let when = match (inside["day"].as_u64(), inside["clock"].as_str()) {
                    (Some(day), Some(clock)) => format!("asked at {clock} of day {day}"),
                    _ => format!("asked on the evening before day {for_day}"),
                };
                let how = if t.answer["kind"] == "window" {
                    ", at later dates"
                } else if t.asked["second_trial"] == true {
                    ", compared"
                } else {
                    ""
                };
                text.push(format!(
                    "Estimate {}: {}{how}, {when}, at the size {}; {note}.",
                    t.number,
                    match (t.asked["intent"].as_u64(), t.asked["preview"].as_u64()) {
                        (Some(intent), _) => format!("intent {intent}"),
                        (_, Some(preview)) => format!("preview {preview}"),
                        _ => "a trial".to_string(),
                    },
                    t.asked["size"].as_str().unwrap_or_default()
                ));
                json!({
                    "estimate": t.number,
                    "kind": t.answer["kind"],
                    "asked_on": {"days_resolved": t.moment.0, "for_day": for_day},
                    "inside_day": inside,
                    "asked": t.asked,
                    "standing": standing,
                })
            })
            .collect();
        if listed.is_empty() {
            text.push("You have been shown no estimate.".to_string());
        }
        Ok(staffing::answer(
            "trials",
            json!({"estimates": listed, "text": text}),
        ))
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
        self.end_day_answer(side, resolved)
    }

    /// The answer of `end_day`: whether the day was resolved, the days
    /// resolved, whether the game is over and the result.
    fn end_day_answer(&self, side: Side, resolved: bool) -> Result<Value, CallError> {
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
    /// 36). The queued orders and commands go in on the first day. In a
    /// watched game the same call is held and the watcher's steps resolve
    /// the days (decision 0182): the call is [`Session::run_wait`], each
    /// day closed [`Session::run_after_day`] and the answer
    /// [`Session::run_answer`], here and there alike.
    fn run_days(
        &mut self,
        side: Side,
        args: &serde_json::Map<String, Value>,
    ) -> Result<Value, CallError> {
        let mut wait = self.run_wait(side, args)?;
        loop {
            // Decision 0214, rule 7: before a day is opened, the evening's
            // stop for a force asking leave to pursue.
            if let Some(asking) = self.stop_before_day(side) {
                wait.asking = asking;
                wait.stopped = Some("asks_leave");
                break;
            }
            self.resolve()?;
            if self.run_after_day(side, &mut wait) {
                break;
            }
        }
        self.run_answer(side, &wait)
    }

    /// The call of `run`: its arguments checked, the staff's orders for the
    /// first day worked before it is resolved, and the first day.
    fn run_wait(
        &self,
        side: Side,
        args: &serde_json::Map<String, Value>,
    ) -> Result<RunWait, CallError> {
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
        if !self.run_offered() {
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
        Ok(RunWait {
            days,
            first: self.world.day() + 1,
            count: 0,
            staff_orders,
            stopped: None,
            asking: Vec::new(),
        })
    }

    /// A day closed under a call of `run`. True when the call's stop has
    /// come.
    fn run_after_day(&self, side: Side, wait: &mut RunWait) -> bool {
        wait.count += 1;
        let day = self.world.day();
        let stopped = if self.is_over() {
            "game_over"
        } else if staffing::fired_on_day(&self.world.side_view(side), day) {
            "interrupt"
        } else if wait.count >= wait.days {
            "days"
        } else {
            return false;
        };
        wait.stopped = Some(stopped);
        true
    }

    /// The answer of `run` once its stop has come.
    fn run_answer(&self, side: Side, wait: &RunWait) -> Result<Value, CallError> {
        let (days, first, count) = (wait.days, wait.first, wait.count);
        let stopped = wait.stopped.unwrap_or("days");
        let last = self.world.day();
        // A stop before the first day has resolved none: the firings of the
        // day before it are not this call's.
        let fired = if count == 0 {
            Vec::new()
        } else {
            staffing::firings_json(&self.world.side_view(side), last)
        };
        let result = match self.final_result() {
            Some(result) => to_value(&result_part(&self.world.side_view(side), &result))?,
            None => Value::Null,
        };
        let resolved = if count == 0 {
            "Resolved no day".to_string()
        } else if count == 1 {
            format!("Resolved day {first} (1 day)")
        } else {
            format!("Resolved days {first} to {last} ({count} days)")
        };
        let why_stopped = match stopped {
            "interrupt" => staffing::firing_words(&fired),
            "game_over" => "the game is over".to_string(),
            "asks_leave" => pursuit::asking_words(&wait.asking),
            _ => format!("the {days} days asked for are resolved"),
        };
        let mut summary = if stopped == "asks_leave" {
            format!(
                "{resolved}; stopped at dusk before day {}: {why_stopped}.",
                last + 1
            )
        } else {
            format!("{resolved}; stopped after day {last}: {why_stopped}.")
        };
        if stopped == "game_over" && !fired.is_empty() {
            summary.push_str(&format!(
                " Fired on that day: {}.",
                staffing::firing_words(&fired)
            ));
        }
        let mut body = json!({
            "format": FORMAT,
            "version": VERSION,
            "kind": "run",
            "staff_orders": wait.staff_orders,
            "days_resolved": {"first": first, "last": last, "count": count},
            "stopped": stopped,
            "fired": fired,
            "game_over": self.is_over(),
            "day": last,
            "result": result,
            "summary": summary,
        });
        if stopped == "asks_leave" {
            body["asking"] = json!(wait.asking);
        }
        Ok(body)
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
        opened.map(|_| ()).map_err(|e| self.stop(day, &e))
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
                    .map_err(|e| self.stop(day, &e))?;
            }
        }
        self.world
            .resolve_next_pulse(&mut |_, _| {})
            .map_err(|e| self.stop(day, &e))?;
        if self.open_day().is_none() {
            self.world
                .close_day(&mut |_, _| {})
                .map_err(|e| self.stop(day, &e))?;
        }
        Ok((day, pulse))
    }

    /// The one thing a caller is told when the engine cannot go on, and the
    /// stop recorded, so that every later call is told the same and the
    /// half-advanced world is not read or advanced again (decision 0172, 3).
    /// The engine's own words may name the other side's units: they go to
    /// the umpire's stderr.
    fn stop(&mut self, day: u32, e: &sim_core::SimError) -> CallError {
        eprintln!("operator: day {day} could not be resolved: {e}");
        let words = format!(
            "the day could not be resolved: the game has stopped inside day {day} on an internal error; the operator has the details"
        );
        self.stopped = Some((day, words.clone()));
        CallError::new(words)
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
