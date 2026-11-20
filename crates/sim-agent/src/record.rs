//! What a played game keeps and what a call can answer with (decisions 11 and
//! 12 of 0050): the error document, the seats and the transcript's entries.
//! Nothing here touches the game.

use std::collections::BTreeMap;

use serde_json::{json, Value};
use sim_core::ids::{BaseId, TaskForceId};
use sim_core::orders::OrderRequest;
use sim_core::pictures::CommandRef;
use sim_core::state::Side;

use crate::answer::{FORMAT, VERSION};

/// The tools of the interface: the whole vocabulary (decision 14 of 0050).
pub const TOOLS: [&str; 10] = [
    "get_briefing",
    "list_forces",
    "list_contacts",
    "list_bases",
    "get_entity",
    "measure",
    "explain",
    "events",
    "issue_orders",
    "end_day",
];

/// The tools of a seat whose side has a staff, beside [`TOOLS`] (decision
/// 0107, "As built", "The tools"). A session made without staffs has none of
/// them: calling one says it is not a tool.
pub const COMMAND_TOOLS: [&str; 19] = [
    "command_state",
    "plans",
    "set_intent",
    "withdraw_intent",
    "set_limit",
    "set_leave",
    "hold",
    "release",
    "preview",
    "approve",
    "cancel_plan",
    "propose",
    "order",
    "staff_orders",
    "set_interrupt",
    "acknowledge",
    "run",
    "report",
    "why",
];

/// The tools of a seat with a staff that plays a whole side, in a game that
/// has a trial (decision 0178, rule 7), listed after [`COMMAND_TOOLS`].
pub const TRIAL_TOOLS: [&str; 2] = ["trial", "trials"];

/// The tools a watcher reads with, from either side's seat (decision 0182,
/// amended 13): a seat's reads that leave no mark in the session. Not
/// `preview` and not `trial`, which are recorded, and none that orders,
/// commands, acknowledges or moves time. He is given those of them the
/// seat has.
pub const WATCHER_TOOLS: [&str; 16] = [
    "get_briefing",
    "list_forces",
    "list_bases",
    "list_contacts",
    "get_entity",
    "measure",
    "explain",
    "events",
    "command_state",
    "plans",
    "staff_orders",
    "propose",
    "order",
    "why",
    "report",
    "trials",
];

/// The seat's name as the interface writes it: `allied` or `japanese`.
pub fn seat_name(side: Side) -> &'static str {
    match side {
        Side::Allied => "allied",
        Side::Japanese => "japanese",
    }
}

/// The side a seat name stands for.
pub fn parse_seat(name: &str) -> Option<Side> {
    match name {
        "allied" => Some(Side::Allied),
        "japanese" => Some(Side::Japanese),
        _ => None,
    }
}

/// Who holds a seat, as recorded in a saved game.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SeatKind {
    /// Whoever is on the other end of the tools.
    Player,
    /// The scenario's own orders for the side, day by day.
    Script,
}

impl SeatKind {
    pub const fn name(self) -> &'static str {
        match self {
            SeatKind::Player => "player",
            SeatKind::Script => "script",
        }
    }

    pub fn parse(name: &str) -> Option<SeatKind> {
        match name {
            "player" => Some(SeatKind::Player),
            "script" => Some(SeatKind::Script),
            _ => None,
        }
    }
}

/// A seat as a session is given it.
#[derive(Clone, Debug)]
pub enum Seat {
    Player,
    /// The orders the script gives for the side, by day, in the order they
    /// are handed in.
    Script(BTreeMap<u32, Vec<OrderRequest>>),
}

impl Seat {
    pub fn kind(&self) -> SeatKind {
        match self {
            Seat::Player => SeatKind::Player,
            Seat::Script(_) => SeatKind::Script,
        }
    }
}

/// Where a player's seat sits (decision 0138, 5): at the side's area command
/// (the default), at the area command with the easier setting of decision
/// 0119 (answer 5), or at one of the side's task forces or bases. A seat at
/// a task force or a base is given what that command holds and nothing else
/// of its side's, and may give orders to its own units only.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Post {
    /// The area command: its picture, and its forces and bases as last
    /// reported (decisions 0134 and 0136).
    #[default]
    Area,
    /// The area command, given the side's own picture, every report at once,
    /// and its forces and bases exactly.
    AreaEasier,
    /// A task force or a base, named by the number the scenario file gives it
    /// (the umpire's). Never the area command: that is [`Post::Area`].
    At(CommandRef),
}

impl Post {
    /// A post written `area`, `area:easier`, `task_force:N` or `base:N`, with
    /// `N` the scenario's number of the force or base.
    pub fn parse(text: &str) -> Result<Post, String> {
        let bad = || {
            format!(
                "\"{text}\" is not a seat: write area, area:easier, task_force:N or base:N (N is the number the scenario file gives it)"
            )
        };
        match text {
            "area" => return Ok(Post::Area),
            "area:easier" => return Ok(Post::AreaEasier),
            _ => {}
        }
        let (kind, number) = text.split_once(':').ok_or_else(bad)?;
        let number: u32 = number.parse().map_err(|_| bad())?;
        match kind {
            "task_force" => Ok(Post::At(CommandRef::TaskForce(TaskForceId::new(number)))),
            "base" => Ok(Post::At(CommandRef::Base(BaseId::new(number)))),
            _ => Err(bad()),
        }
    }

    /// The post as [`Post::parse`] reads it.
    pub fn text(self) -> String {
        match self {
            Post::Area => "area".to_string(),
            Post::AreaEasier => "area:easier".to_string(),
            Post::At(CommandRef::TaskForce(force)) => format!("task_force:{}", force.raw()),
            Post::At(CommandRef::Base(base)) => format!("base:{}", base.raw()),
            Post::At(CommandRef::Area) => "area".to_string(),
        }
    }
}

/// A call that was refused as a whole: a tool that is not one, arguments that
/// are not arguments, a seat that is not a player's, orders after the day is
/// closed. The game is unchanged. Serialises as the `error` document.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CallError {
    pub message: String,
    /// What would have been allowed, when that is a short list (the tools, the
    /// seats); empty otherwise.
    pub allowed: Vec<String>,
}

impl CallError {
    pub fn new(message: impl Into<String>) -> CallError {
        CallError {
            message: message.into(),
            allowed: Vec::new(),
        }
    }

    pub fn naming(message: impl Into<String>, allowed: &[&str]) -> CallError {
        CallError {
            message: message.into(),
            allowed: allowed.iter().map(|s| (*s).to_string()).collect(),
        }
    }

    /// The `error` document.
    pub fn to_json(&self) -> Value {
        let mut value = json!({
            "format": FORMAT,
            "version": VERSION,
            "kind": "error",
            "message": self.message,
        });
        if !self.allowed.is_empty() {
            value["allowed"] = json!(self.allowed);
        }
        value
    }
}

impl std::fmt::Display for CallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

/// One call a seat made, with the whole answer it was given (decision 12 of
/// 0050). It carries no time of day.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TranscriptEntry {
    /// 1, 2, 3... in the order the calls were made.
    pub seq: u32,
    /// How many days had been resolved when the call was made.
    pub after_day: u32,
    pub seat: Side,
    pub tool: String,
    pub arguments: Value,
    /// The answer, or the `error` document.
    pub answer: Value,
}

impl TranscriptEntry {
    pub fn to_json(&self) -> Value {
        json!({
            "format": FORMAT,
            "version": VERSION,
            "kind": "transcript_entry",
            "seq": self.seq,
            "after_day": self.after_day,
            "seat": seat_name(self.seat),
            "tool": self.tool,
            "arguments": self.arguments,
            "answer": self.answer,
        })
    }
}
