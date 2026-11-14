//! A side's questions of the air steps: aircraft able to fly, a base strike's launch, a fighter group's patrol and escort, the raid the escorts go with, where aircraft land when their deck is gone, and a tired group's stand-down or return (decision 0100, 2).

use crate::choices::air::{AirAnswer, AirQuestion};
use crate::view::SideView;

impl SideView<'_> {
    /// The answer to `question` from what this side holds now. `None`
    /// where the question names no unit of this side, or the choice does not
    /// arise.
    pub(crate) fn ask_air(&self, question: &AirQuestion) -> Option<AirAnswer> {
        match *question {}
    }

    /// Every question of this kind this side could ask now, one for each of
    /// its units the choice applies to.
    pub(crate) fn air_questions(&self) -> Vec<AirQuestion> {
        Vec::new()
    }
}
