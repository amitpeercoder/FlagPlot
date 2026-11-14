//! The questions of the air steps: aircraft able to fly, a base strike's launch, a fighter group's patrol and escort, the raid the escorts go with, where aircraft land when their deck is gone, and a tired group's stand-down or return (decision 0100, 1).

/// A question about one of the side's own units in air.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AirQuestion {}

/// The answer to an [`AirQuestion`].
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AirAnswer {}
