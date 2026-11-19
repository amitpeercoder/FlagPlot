//! What a side believed of the enemy when the game began (decision 0173, 5
//! to 9): the forces it believed in and held no contact of, and the intents
//! it believed the enemy held; and the men it believed may be ashore at a
//! place beyond its estimate (decision 0176, 1).
//!
//! Plain data, the scenario's and not the world's. A believed force and a
//! believed intent are not state and not belief: they are never put into a
//! [`World`](crate::World), never hashed, and no rule of the engine reads
//! them. The loader checks them and keeps them with the scenario, so that a
//! crate that builds the enemy's half of a picture can be handed them with a
//! side's view.

use crate::command::IntentKind;
use crate::hex::Hex;
use crate::ids::BaseId;
use crate::refdata::ShipKind;
use crate::state::Side;

/// How a side holds a believed force (decision 0173, 6, and decision 0161,
/// 7). The loader gives no chance to any of them.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Standing {
    /// Believed present: it is there.
    Present,
    /// Believed absent: it is not there; the entry records that the side
    /// ruled it out.
    Absent,
    /// Not known: the side does not know whether it is there.
    Unknown,
}

impl Standing {
    /// Every standing, in the order the record gives them.
    pub const ALL: [Standing; 3] = [Standing::Present, Standing::Absent, Standing::Unknown];

    /// The word a scenario gives it: `present`, `absent` or `unknown`.
    pub const fn name(self) -> &'static str {
        match self {
            Standing::Present => "present",
            Standing::Absent => "absent",
            Standing::Unknown => "unknown",
        }
    }

    /// The standing a scenario's word names, if it names one.
    pub fn from_name(name: &str) -> Option<Standing> {
        Standing::ALL.into_iter().find(|s| s.name() == name)
    }
}

/// Where a believed force was last known (decision 0173, 5).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LastKnown {
    /// The side does not know where it is.
    NotKnown,
    /// At an enemy base on the side's map, with that base's hex, on `day`
    /// (0 for before the first day).
    AtBase { base: BaseId, hex: Hex, day: u32 },
    /// At a hex of the map, on `day` (0 for before the first day).
    AtHex { hex: Hex, day: u32 },
}

impl LastKnown {
    /// The hex it was last known at, if known.
    pub const fn hex(self) -> Option<Hex> {
        match self {
            LastKnown::NotKnown => None,
            LastKnown::AtBase { hex, .. } | LastKnown::AtHex { hex, .. } => Some(hex),
        }
    }

    /// The day it was known there, if known.
    pub const fn day(self) -> Option<u32> {
        match self {
            LastKnown::NotKnown => None,
            LastKnown::AtBase { day, .. } | LastKnown::AtHex { day, .. } => Some(day),
        }
    }
}

/// One force of the enemy's that a side believes in and holds no contact of
/// (decision 0173, 5), as the scenario states it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BelievedForce {
    /// The side that believes it.
    pub side: Side,
    /// Words, for the estimate to name it by.
    pub name: String,
    /// The kind of its most important ship, as a contact has one.
    pub kind: ShipKind,
    /// How many ships it is believed to have: 1 or more.
    pub ships: u32,
    pub standing: Standing,
    pub last_known: LastKnown,
    /// True where the belief rests on no source.
    pub placeholder: bool,
    /// What the entry rests on; required where it is a placeholder.
    pub note: Option<String>,
}

impl BelievedForce {
    /// The side whose force it is believed to be.
    pub const fn enemy(&self) -> Side {
        other(self.side)
    }
}

/// One intent a side believes the enemy holds (decision 0173, 7), in the
/// staff's own vocabulary: the kind of a `set_intent` command for the
/// enemy's side.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BelievedIntent {
    /// The side that believes it.
    pub side: Side,
    /// What the enemy is believed to intend.
    pub kind: IntentKind,
    /// True where the belief rests on no source.
    pub placeholder: bool,
    /// What the entry rests on; required where it is a placeholder.
    pub note: Option<String>,
}

impl BelievedIntent {
    /// The side believed to hold the intent: the believing side's enemy.
    pub const fn enemy(&self) -> Side {
        other(self.side)
    }
}

/// Men of the enemy's that a side believes may be ashore at a place beyond
/// those it holds an estimate of (decision 0176, 1, amended 6 and 7), as the
/// scenario states them. Plain data, as a believed force is: not state, not
/// belief, never put into a world and never hashed.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BelievedMen {
    /// The side that believes it.
    pub side: Side,
    /// Words, for the estimate to name it by.
    pub name: String,
    /// The hex of the place: the base's where `base` is named.
    pub hex: Hex,
    /// The base of the enemy's on the side's map the scenario names the
    /// place by, if it names one; `None` where it gives a hex.
    pub base: Option<BaseId>,
    /// How many men: 1 or more.
    pub men: u32,
    pub standing: Standing,
    /// True where the belief rests on no source.
    pub placeholder: bool,
    /// What the entry rests on; required where it is a placeholder.
    pub note: Option<String>,
}

impl BelievedMen {
    /// The side whose men they are believed to be.
    pub const fn enemy(&self) -> Side {
        other(self.side)
    }
}

const fn other(side: Side) -> Side {
    match side {
        Side::Allied => Side::Japanese,
        Side::Japanese => Side::Allied,
    }
}
