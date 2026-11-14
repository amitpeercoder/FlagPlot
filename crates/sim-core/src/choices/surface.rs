//! The questions of the surface fight and stragglers: a force alerted, ready when a fight begins, breaking off, its stragglers, and where a straggler or a retiring force goes (decision 0100, 1).

/// A question about one of the side's own units in surface.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SurfaceQuestion {}

/// The answer to an [`SurfaceQuestion`].
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SurfaceAnswer {}
