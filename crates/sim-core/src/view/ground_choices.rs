//! A side's questions of the land units: going back to defend, and what a unit's day is (decision 0100, 2).

use crate::choices::ground::{GroundAnswer, GroundQuestion};
use crate::view::SideView;

impl SideView<'_> {
    /// The answer to `question` from what this side holds now. `None`
    /// where the question names no unit of this side, or the choice does not
    /// arise.
    pub(crate) fn ask_ground(&self, question: &GroundQuestion) -> Option<GroundAnswer> {
        match *question {}
    }

    /// Every question of this kind this side could ask now, one for each of
    /// its units the choice applies to.
    pub(crate) fn ground_questions(&self) -> Vec<GroundQuestion> {
        Vec::new()
    }
}
