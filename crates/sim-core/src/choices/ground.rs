//! The questions of the land units: going back to defend, and what a unit's day is (decision 0100, 1).

/// A question about one of the side's own units in ground.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum GroundQuestion {}

/// The answer to an [`GroundQuestion`].
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum GroundAnswer {}
