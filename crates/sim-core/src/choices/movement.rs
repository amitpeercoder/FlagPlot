//! The questions of the passage of task forces: turning back or going on when a route changes, and halting at a marked waypoint and steaming on (decision 0100, 1).

/// A question about one of the side's own units in movement.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MovementQuestion {}

/// The answer to an [`MovementQuestion`].
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MovementAnswer {}
