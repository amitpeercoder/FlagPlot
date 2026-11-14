//! A side's questions of the surface fight and stragglers: a force alerted, ready when a fight begins, breaking off, its stragglers, and where a straggler or a retiring force goes (decision 0100, 2).

use crate::choices::surface::{SurfaceAnswer, SurfaceQuestion};
use crate::view::SideView;

impl SideView<'_> {
    /// The answer to `question` from what this side holds now. `None`
    /// where the question names no unit of this side, or the choice does not
    /// arise.
    pub(crate) fn ask_surface(&self, question: &SurfaceQuestion) -> Option<SurfaceAnswer> {
        match *question {}
    }

    /// Every question of this kind this side could ask now, one for each of
    /// its units the choice applies to.
    pub(crate) fn surface_questions(&self) -> Vec<SurfaceQuestion> {
        Vec::new()
    }
}
