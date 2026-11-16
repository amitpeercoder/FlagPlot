//! What a command knows of its forces (decision 0136, 8 to 12).
//!
//! A command knows its own units exactly. The area command knows a force a
//! level down, and a base, as last reported: [`SideView::known_to`] hands a
//! view whose units are the truth with what the area command holds written
//! over it ([`crate::held::held_truth`]), and every method of the view reads
//! that and no second copy of any rule. With no report figures, or command
//! levels off, there is nothing held and the view is the side's own, whole
//! (item 7).

use crate::held::{held_truth, AsOf};
use crate::ids::TaskForceId;
use crate::pictures::CommandRef;
use crate::state::TruthState;
use crate::time::Pulse;

use super::SideView;

/// A view of the side's own units as one command of it knows them: the
/// side's own view for any command but the area command, and, for the area
/// command while reports travel, a view over what it holds. It owns the
/// overlay the view reads; [`KnownView::view`] borrows it.
pub struct KnownView<'a> {
    pub(super) view: SideView<'a>,
    pub(super) overlay: Option<Box<TruthState>>,
}

impl std::fmt::Debug for KnownView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KnownView")
            .field("side", &self.view.side)
            .field("known_to", &self.view.known_to)
            .finish_non_exhaustive()
    }
}

impl KnownView<'_> {
    /// The view. For the area command while reports travel its units are as
    /// the area command holds them; otherwise it is the side's own view.
    pub fn view(&self) -> SideView<'_> {
        match &self.overlay {
            Some(overlay) => SideView {
                truth: overlay,
                known_to: Some(CommandRef::Area),
                ..self.view
            },
            None => self.view,
        }
    }
}

impl<'a> SideView<'a> {
    /// The side's units as `command` knows them (decision 0136, 8 and 9). A
    /// command knows its own units exactly, so any command but the area
    /// command gets the side's own view. The area command, while reports
    /// travel, gets a view in which each force, base, ship, air group and
    /// land unit a held state names is as that state gives it; the rest of
    /// the side's units, and the enemy's, are as they are. With no report
    /// figures, or command levels off, it is the side's own view.
    pub fn known_to(&self, command: CommandRef) -> KnownView<'a> {
        let overlay = match (command, self.held) {
            (CommandRef::Area, Some(held)) => Some(Box::new(held_truth(self.truth, held))),
            _ => None,
        };
        KnownView {
            view: *self,
            overlay,
        }
    }

    /// The moment the area command's held state of `command` is of, on a
    /// view known to the area command (decision 0136, 11). `None` on any
    /// other view, and for a command the area command does not hold.
    pub fn held_as_of(&self, command: CommandRef) -> Option<AsOf> {
        if self.known_to != Some(CommandRef::Area) {
            return None;
        }
        self.held?.get(&command).map(|state| state.as_of())
    }

    /// The pulse just ended: `Start` before day 1, else the afternoon of the
    /// last day resolved. A reason or a line says "as of ..." only where a
    /// held state is of another moment (decision 0136, 11).
    pub fn pulse_just_ended(&self) -> AsOf {
        match self.truth.day {
            0 => AsOf::Start,
            day => AsOf::Pulse {
                day,
                pulse: Pulse::Afternoon,
            },
        }
    }

    /// Whether the side's task force `force` is on radio silence (decision
    /// 0136, 15). `None` for a force that is not the side's.
    pub fn radio_silence(&self, force: TaskForceId) -> Option<bool> {
        self.truth
            .task_forces
            .get(&force)
            .filter(|record| record.side == self.side)?;
        Some(self.radio_silent.contains(&force))
    }
}
