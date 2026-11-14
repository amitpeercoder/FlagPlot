//! The choices a side's own units make inside a day, as questions a side can
//! ask (decision 0100).
//!
//! Each choice the rules make for a unit from what its side knows (its own
//! units, its standing orders, its belief and the figures) is one function of
//! those inputs. The engine calls that function when it makes the choice, and
//! a side asks the same function through its view ([`SideView::ask`]), so
//! there is one answer and one place it is written. An answer is what the
//! engine would do at this moment with what the side holds. It is not a
//! forecast: it says nothing of what the enemy will do or what the dice will
//! give.
//!
//! The five choices made at the point of contact (which enemy force a raid
//! attacks, which ship a strike group attacks, how a patrol divides between
//! raids, whether a force that avoids slips away, and who fires at whom in a
//! surface fight) read what is truly there and stay with the rules: no
//! question here asks them (decision 0100, 4).
//!
//! The questions are grouped by the code that makes the choice: the air
//! steps ([`air`]), the surface fight and stragglers ([`surface`]), the
//! passage of task forces ([`movement`]) and the land units ([`ground`]).
//!
//! # The trace
//!
//! For the test that sets every answer beside what the engine then did
//! (decision 0100, 7), a world can keep a trace of the choices it makes: for
//! each, the side, the question, the answer and the moment of the day. It is
//! off unless [`World::trace_choices`] turns it on. It is not state: it is
//! not hashed, not saved and read by no rule, and turning it on or off
//! changes nothing that happens.
//!
//! [`SideView::ask`]: crate::view::SideView::ask
//! [`World::trace_choices`]: crate::world::World::trace_choices

pub mod air;
pub mod ground;
pub mod movement;
pub mod surface;

use crate::state::Side;
use crate::time::Pulse;

pub use crate::resolve::{EndOfDayPhase, PulsePhase};

/// A question a side may ask of one of its own units' choices.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Question {
    Air(air::AirQuestion),
    Surface(surface::SurfaceQuestion),
    Movement(movement::MovementQuestion),
    Ground(ground::GroundQuestion),
}

/// The answer to a [`Question`]: what the engine would choose now.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Answer {
    Air(air::AirAnswer),
    Surface(surface::SurfaceAnswer),
    Movement(movement::MovementAnswer),
    Ground(ground::GroundAnswer),
}

/// A point in the resolution of a day, in the order the day passes them.
/// [`World::resolve_day_watched`](crate::world::World::resolve_day_watched)
/// stops at each, and a traced choice records the last one passed.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Moment {
    /// Before anything of the day: the orders submitted for it are not yet
    /// taken in.
    DayBegins { day: u32 },
    /// After forces back from the yard are home and ships have refuelled in
    /// port, before the day's orders are taken in.
    Orders { day: u32 },
    /// After the orders and forward repair, before the pulse's alert from
    /// belief, fatigue of forces and grounded bases.
    PulseBegins { day: u32, pulse: Pulse },
    /// Before one step of a pulse.
    Phase {
        day: u32,
        pulse: Pulse,
        phase: PulsePhase,
    },
    /// Before one step after the last pulse.
    EndOfDay { day: u32, phase: EndOfDayPhase },
}

impl Moment {
    /// The day this moment belongs to.
    pub fn day(self) -> u32 {
        match self {
            Moment::DayBegins { day }
            | Moment::Orders { day }
            | Moment::PulseBegins { day, .. }
            | Moment::Phase { day, .. }
            | Moment::EndOfDay { day, .. } => day,
        }
    }
}

/// One choice the engine made, as the trace records it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TracedChoice {
    /// The last moment of the day passed before the choice was made.
    pub moment: Moment,
    /// The side whose unit chose.
    pub side: Side,
    pub question: Question,
    pub answer: Answer,
}

/// The trace a world keeps while [`World::trace_choices`] is on.
///
/// [`World::trace_choices`]: crate::world::World::trace_choices
#[derive(Clone, Debug, Default)]
pub(crate) struct Trace {
    pub(crate) moment: Option<Moment>,
    pub(crate) entries: Vec<TracedChoice>,
}

/// The cell a world keeps its trace in. A mutex and not a `RefCell`, so that
/// a world can still be shared between threads for reading (a test runs
/// seeds side by side); the lock is never contended in a game, which runs on
/// one thread. Cloning a world clones its trace.
#[derive(Debug, Default)]
pub(crate) struct TraceCell(std::sync::Mutex<Option<Trace>>);

impl TraceCell {
    pub(crate) fn lock(&self) -> std::sync::MutexGuard<'_, Option<Trace>> {
        // A panic while the lock was held leaves only a trace, never state.
        self.0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    pub(crate) fn get_mut(&mut self) -> &mut Option<Trace> {
        self.0
            .get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

impl Clone for TraceCell {
    fn clone(&self) -> Self {
        TraceCell(std::sync::Mutex::new(self.lock().clone()))
    }
}
