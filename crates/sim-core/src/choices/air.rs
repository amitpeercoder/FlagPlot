//! The questions of the air steps: aircraft able to fly, a base strike's launch, a fighter group's patrol and escort, the raid the escorts go with, where aircraft land when their deck is gone, and a tired group's stand-down or return (decision 0100, 1).
//!
//! The naval strike choice (which contact a group would launch at) was
//! already a function a side could ask (decision 0049); it is a question
//! here too, so every choice of the air steps is asked the same way.

use crate::airstrike::StrikeChoice;
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{AirGroupId, BaseId};
use crate::state::AirGroupLocation;
use crate::time::Pulse;

/// A question about one of the side's own units in air.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AirQuestion {
    /// The contact a group on the naval strike mission would launch at
    /// (decision 6 of 0063).
    Strike { group: AirGroupId },
    /// How many of a group's aircraft can fly this pulse (decision 14 of
    /// 0026 and decision 2 of 0036).
    AircraftAbleToFly { group: AirGroupId },
    /// Whether a group on the base strike mission launches in `pulse`
    /// (decision 3 of 0034 and decision 7 of 0063).
    BaseStrike { group: AirGroupId, pulse: Pulse },
    /// How a group on combat air patrol divides what it can fly into patrol
    /// and escorts (decision 2 of 0027).
    FighterSplit { group: AirGroupId },
    /// Which of the side's raids of the pulse a fighter group's escorts go
    /// with, or whether they stay (decision 5 of 0027 and decision 0030).
    /// Asked only of a group that has escorts.
    Escort { group: AirGroupId },
    /// Where the aircraft in the air whose own deck is gone land (decisions
    /// 7 to 10 of 0059). The groups are in group number order, each with the
    /// aircraft it has in the air; the carriers' room is shared between them
    /// in that order.
    Landing { groups: Vec<(AirGroupId, u32)> },
    /// Whether a group with a rest threshold stands down, returns to
    /// flying, or stays as it is (decision 7 of 0036).
    Rest { group: AirGroupId },
}

/// The answer to an [`AirQuestion`].
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AirAnswer {
    /// `None`: the group launches nothing.
    Strike(Option<StrikeChoice>),
    AircraftAbleToFly(u32),
    /// `None`: the group launches no base strike this pulse.
    BaseStrike(Option<BaseStrikeLaunch>),
    FighterSplit(FighterSplit),
    Escort(EscortChoice),
    /// In the order of the groups asked about. A group the side cannot
    /// place (no position, no aircraft in the air) is left out.
    Landing(Vec<GroupLanding>),
    Rest(RestChoice),
}

/// A base strike that launches: what the staff records of it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BaseStrikeLaunch {
    pub base: BaseId,
    /// The base's hex: where the strike flies to.
    pub hex: Hex,
    /// The air distance from the group to the base, in whole miles.
    pub distance_nm: Fixed,
    /// The aircraft launched: every one the group can fly.
    pub aircraft: u32,
}

/// A fighter group's aircraft divided into patrol and escorts.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FighterSplit {
    /// Kept over the group's own hex.
    pub patrol: u32,
    /// Aircraft that go with a raid, if one is sent.
    pub escorts: u32,
}

/// What a fighter group's escorts do.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EscortChoice {
    /// No raid leaves from the group's place: the escorts stay unsent.
    NoRaid,
    /// A raid leaves from its place but its target is beyond the escorts'
    /// radius: the escorts join the patrol.
    Stays,
    /// The escorts go with the raid whose lead (lowest-numbered) strike group
    /// is `lead`, to `hex`.
    Goes { lead: AirGroupId, hex: Hex },
}

/// Where the aircraft of one group in the air land.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GroupLanding {
    pub group: AirGroupId,
    /// The ships and bases the aircraft go to, in order, with how many each
    /// takes.
    pub places: Vec<(AirGroupLocation, u32)>,
    /// The aircraft that find no place and go into the sea.
    pub sea: u32,
}

/// What a group with a rest threshold does.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RestChoice {
    /// Fatigue is above the threshold and the group is not stood down yet.
    StandDown,
    /// The group is stood down and its fatigue is not above the threshold.
    Return,
    NoChange,
}
