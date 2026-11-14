//! The questions of the passage of task forces: turning back or going on when a route changes, and halting at a marked waypoint and steaming on (decision 0100, 1).

use crate::hex::Hex;
use crate::ids::TaskForceId;

/// A question about one of the side's own units in movement.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MovementQuestion {
    /// Whether the force would turn back or go on if a new route began at
    /// `first_waypoint` (decision 0005, 5, amended by 0017, 14). With no
    /// waypoint, the force goes on. The force's own position on its leg is
    /// read when the question is asked.
    Reroute {
        force: TaskForceId,
        first_waypoint: Option<Hex>,
    },
    /// Where the force would halt on its route (decision 0096, 1 and 2):
    /// the first waypoint from its next one on that is marked to act at and
    /// where it has work.
    Halt { force: TaskForceId },
    /// Whether the halted force would steam on now (decision 0096, 4).
    SteamOn { force: TaskForceId },
}

/// The answer to an [`MovementQuestion`].
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MovementAnswer {
    /// To [`MovementQuestion::Reroute`]: `None` if neither way reaches the
    /// first waypoint by sea (the order would be refused).
    Reroute(Option<Rerouted>),
    /// To [`MovementQuestion::Halt`]: the index in the route of the marked
    /// waypoint where the force would halt, or `None`.
    Halt(Option<usize>),
    /// To [`MovementQuestion::SteamOn`]: `true` to steam on, `false` to stay.
    SteamOn(bool),
}

/// Where a new route starts: the answer of [`MovementQuestion::Reroute`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rerouted {
    /// Whether the force turns round to get there.
    pub turns_back: bool,
    /// The hex the new route steers from.
    pub steers_from: Hex,
}
