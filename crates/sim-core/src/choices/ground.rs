//! The questions of the land units: going back to defend, and what a unit's day is (decision 0100, 1).

use crate::fixed::Fixed;
use crate::ids::{LandUnitId, TaskForceId};
use crate::orders::Job;

/// A question about one of the side's own units in ground.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum GroundQuestion {
    /// Whether a land unit goes back to defend (decision 14 of 0090): it is
    /// on attack and its fatigue is above the limit. The question carries the
    /// fatigue the test reads, which the engine takes after the day's
    /// update. Asked from a side's view with no fatigue given, it is the
    /// unit's present fatigue.
    BackToDefend { unit: LandUnitId, fatigue: Fixed },
    /// What a land unit's day is (decision 3b of 0099). Two of its inputs are
    /// known only inside the day's ground step, so the question carries them:
    /// whether the unit marched today (known once the marches have run) and
    /// whether its hex was contested (a fact of the enemy's men too), and
    /// the force whose roster it is on among the forces that qualified to
    /// load today (known once the loading was gained). Asked from a side's
    /// view the staff fills them from what it holds: `marched` is whether
    /// the unit has a march under way; `contested` is whether a fight in its
    /// hex is certain (a fighting unit of its own there is on attack and the
    /// side holds an estimate of enemy men in the hex); `rostered_with` is
    /// the force whose roster the unit is on, if that force would qualify
    /// today.
    UnitDay {
        unit: LandUnitId,
        marched: bool,
        contested: bool,
        rostered_with: Option<TaskForceId>,
    },
}

/// What a land unit's day is (decision 3b of 0099).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UnitDay {
    /// No man ashore and none aboard: the unit has no day.
    NoMen,
    /// It marched: the march is its day.
    Marched,
    /// Its hex was contested: the fight is its day.
    Contested,
    /// It works, at this job (none is the default job).
    Work(Option<Job>),
    /// It trains.
    Train,
    /// It rehearses with this force.
    Rehearse(TaskForceId),
    /// It stands ready with nothing to do.
    Ready,
}

/// The answer to an [`GroundQuestion`].
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum GroundAnswer {
    /// Whether the unit goes back to defend.
    BackToDefend(bool),
    /// What the unit's day is.
    UnitDay(UnitDay),
}
