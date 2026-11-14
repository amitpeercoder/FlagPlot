//! The questions of the surface fight and stragglers: a force alerted, ready when a fight begins, breaking off, its stragglers, and where a straggler or a retiring force goes (decision 0100, 1).
//!
//! Each answer is what the engine does with the same inputs, from the
//! functions `alert_choice`, `force_ready` and `break_off_test` in
//! `surface.rs` and `straggler_plan` and `home_hex` in `stragglers.rs`.

use std::collections::BTreeSet;

use crate::hex::Hex;
use crate::ids::{ContactId, ShipId, TaskForceId};
use crate::time::Pulse;

/// A question about one of the side's own forces in the surface fight or its
/// stragglers. Every question names a task force of the asking side.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SurfaceQuestion {
    /// Whether the force is alerted by a contact, stood down or left as it
    /// is, as the pulse begins (decision 3 of 0021). Not asked of a force
    /// at battle readiness, or with no ship afloat: the rule skips those.
    Alert { task_force: TaskForceId },
    /// Whether the force is ready for action as a fight begins in `pulse` of
    /// `day` (decision 0068, 2).
    Ready {
        task_force: TaskForceId,
        day: u32,
        pulse: Pulse,
    },
    /// Whether the force breaks off after a round of a fight (decision 0022).
    /// The three inputs that exist only inside the fight are carried here:
    /// the ships the force began the fight with, whether it was fired on in
    /// the round, and whether it is already avoiding in the pulse (it broke
    /// off or slipped away earlier). A question made by the view asks with
    /// the force's present ships as those it began with, not fired on and
    /// not yet avoiding.
    BreakOff {
        task_force: TaskForceId,
        began_with: Vec<ShipId>,
        fired_on: bool,
        already_avoiding: bool,
    },
    /// What the force does with its stragglers (decisions 10 to 14 of 0016).
    Stragglers { task_force: TaskForceId },
    /// The hex a straggler force, or a retiring one, heads for: its home
    /// port (decision 14 of 0016).
    HeadHome { task_force: TaskForceId },
}

/// The answer to a [`SurfaceQuestion`].
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SurfaceAnswer {
    Alert(AlertChoice),
    Ready(bool),
    /// Whether the force breaks off.
    BreakOff(bool),
    Stragglers(StragglerPlan),
    /// The hex it heads for, or `None` where it has no home port or is
    /// already at it and not under way.
    HeadHome(Option<Hex>),
}

/// What alert from belief does to one force.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AlertChoice {
    /// Not alerted and no fresh contact near, or alerted by a contact that
    /// still holds: nothing changes.
    NoChange,
    /// The force is alerted by this contact, the nearest, the lowest number
    /// on a tie.
    AlertedBy(ContactId),
    /// The force was alerted and no fresh contact is near now: it stands
    /// down.
    StandDown,
}

/// What a force does with its stragglers, and the figures the decision was
/// made on (they go into its breakdown).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StragglerPlan {
    pub action: StragglerAction,
    /// Ships with working propulsion.
    pub sound_ships: usize,
    /// The escorts the stragglers that can steam call for.
    pub escorts_needed: usize,
    /// The escorts taken from the sound ships.
    pub escorts_taken: usize,
    /// The sound ships the parent keeps.
    pub kept: usize,
}

/// The step of the straggler rule a force takes.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum StragglerAction {
    /// Nothing to do: the force keeps its stragglers, or has none.
    None,
    /// Leave: every straggler goes into one new force that stays where it
    /// is.
    Leave { stragglers: BTreeSet<ShipId> },
    /// Detach: the stragglers that can steam go home with `escorts`, and
    /// those dead in the water are left (either set may be empty).
    Detach {
        sent_home: BTreeSet<ShipId>,
        escorts: BTreeSet<ShipId>,
        left_behind: BTreeSet<ShipId>,
    },
    /// Retire: ships dead in the water are left (a set that may be empty)
    /// and the force goes home with the rest.
    Retire { left_behind: BTreeSet<ShipId> },
}
