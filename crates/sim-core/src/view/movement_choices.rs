//! A side's questions of the passage of task forces: turning back or going on when a route changes, and halting at a marked waypoint and steaming on (decision 0100, 2).

use crate::choices::movement::{MovementAnswer, MovementQuestion};
use crate::view::SideView;

impl SideView<'_> {
    /// The answer to `question` from what this side holds now. `None`
    /// where the question names no unit of this side, or the choice does not
    /// arise.
    pub(crate) fn ask_movement(&self, question: &MovementQuestion) -> Option<MovementAnswer> {
        match *question {}
    }

    /// Every question of this kind this side could ask now, one for each of
    /// its units the choice applies to.
    pub(crate) fn movement_questions(&self) -> Vec<MovementQuestion> {
        Vec::new()
    }
}
