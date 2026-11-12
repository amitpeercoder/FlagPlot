//! What a played game keeps and what a call can answer with (decisions 11 and
//! 12 of 0050): the error document, the seats and the transcript's entries.
//! Nothing here touches the game.

use std::collections::BTreeMap;

use serde_json::{json, Value};
use sim_core::orders::OrderRequest;
use sim_core::state::Side;

use crate::answer::{FORMAT, VERSION};

/// The tools of the interface: the whole vocabulary (decision 14 of 0050).
pub const TOOLS: [&str; 9] = [
    "get_briefing",
    "list_forces",
    "list_contacts",
    "list_bases",
    "get_entity",
    "measure",
    "explain",
    "issue_orders",
    "end_day",
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
