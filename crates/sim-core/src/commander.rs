//! Commanders (decision 0137, step 4 of stage 2b).
//!
//! A command may have a commander, named in the scenario: a name and an
//! initiative rating, cautious, steady or bold, each a placeholder with a
//! note of what it rests on (items 1 and 2). He is held in his command's
//! post while command levels are on, and goes with it (item 4). With them off
//! there are no commands and no commander is held or read (item 3).
//!
//! One rule reads the rating, the closing rule of a carrier force in the
//! staff's carrier doctrine (items 7 to 10); nothing in the engine reads it.

use std::fmt;

use crate::pictures::CommandRef;
use crate::state::Side;

/// A commander's initiative rating (decision 0137, 1).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub enum Initiative {
    Cautious,
    /// What a command with no commander named is (item 3).
    #[default]
    Steady,
    Bold,
}

impl Initiative {
    /// A stable number. Part of the state hash. Never renumber.
    pub const fn code(self) -> u8 {
        match self {
            Initiative::Cautious => 1,
            Initiative::Steady => 2,
            Initiative::Bold => 3,
        }
    }

    /// The rating in words, as a scenario spells it.
    pub const fn name(self) -> &'static str {
        match self {
            Initiative::Cautious => "cautious",
            Initiative::Steady => "steady",
            Initiative::Bold => "bold",
        }
    }

    /// The rating a scenario spells, if it is one.
    pub fn parse(text: &str) -> Option<Initiative> {
        match text {
            "cautious" => Some(Initiative::Cautious),
            "steady" => Some(Initiative::Steady),
            "bold" => Some(Initiative::Bold),
            _ => None,
        }
    }
}

impl fmt::Display for Initiative {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// A commander (decision 0137, 1 and 2): his name, his rating and the note
/// of what the rating rests on. Every rating is a placeholder.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Commander {
    pub name: String,
    pub initiative: Initiative,
    /// What the rating rests on. Never empty.
    pub note: String,
}

/// A commander a scenario names for one command of a side.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NamedCommander {
    pub side: Side,
    pub command: CommandRef,
    pub commander: Commander,
}
