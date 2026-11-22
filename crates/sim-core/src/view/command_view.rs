//! A view for a command (decision 0134, 15), as [`SideView`] is for a side:
//! one command's picture of the enemy, its contacts, claims and estimates,
//! and nothing else. It is reached only through [`SideView::command`], for a
//! command of the view's own side, and holds that command's picture only:
//! no method returns another command's picture or the truth (item 14). A
//! side's own units are still known to it exactly through the side's view.
//!
//! With command levels off a command's view holds the side's own belief
//! (item 16).

use std::collections::BTreeSet;

use crate::belief::{
    BattleClaim, BeliefState, Contact, GroundEstimate, SubmarineReport, SuspectedWaters,
};
use crate::commander::Commander;
use crate::hex::Hex;
use crate::ids::{AirGroupId, ContactId, LandUnitId, TaskForceId};
use crate::pictures::{self, CommandRef};
use crate::state::Side;

use super::SideView;

/// One command's view of the enemy.
///
/// `Debug` is written by hand and prints the side and the command only.
#[derive(Clone, Copy)]
pub struct CommandView<'a> {
    side: Side,
    command: CommandRef,
    superior: Option<CommandRef>,
    picture: &'a BeliefState,
    reports_travel: bool,
    /// The contacts that are the command's own, with command levels on
    /// (decision 0137, 5); `None` with them off.
    own: Option<&'a BTreeSet<ContactId>>,
    /// The command's commander, with command levels on (decision 0137, 3).
    commander: Option<&'a Commander>,
}

impl std::fmt::Debug for CommandView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandView")
            .field("side", &self.side)
            .field("command", &self.command)
            .finish_non_exhaustive()
    }
}

impl<'a> CommandView<'a> {
    /// The side's own belief as a view, every report at once: the picture of
    /// the easier setting at the area seat (decision 0119, answer 5, and
    /// decision 0135, 19). It is the side's own and never the enemy's.
    pub(super) fn of_side_belief(side: Side, belief: &'a BeliefState) -> CommandView<'a> {
        CommandView {
            side,
            command: CommandRef::Area,
            superior: None,
            picture: belief,
            reports_travel: false,
            own: None,
            commander: None,
        }
    }

    /// The side the command belongs to.
    pub fn side(&self) -> Side {
        self.side
    }

    /// The command this view is of.
    pub fn command(&self) -> CommandRef {
        self.command
    }

    /// The command it is under: `None` for the area command, and for every
    /// command while command levels are off.
    pub fn superior(&self) -> Option<CommandRef> {
        self.superior
    }

    /// What the command believes is at sea, in contact number order.
    pub fn contacts(&self) -> impl Iterator<Item = &'a Contact> + 'a {
        self.picture.contacts()
    }

    /// Whether reports travel between commands in this game (decision 0135):
    /// command levels are on and report figures are loaded. Then pictures can
    /// differ, and a command's picture holds only what it was sent.
    pub fn reports_travel(&self) -> bool {
        self.reports_travel
    }

    /// The command's number of a contact. Each picture numbers its own
    /// contacts from 1 (decision 0135, 20), so the same contact can have
    /// another number in another command's picture, and a number means
    /// something only in the picture of the command that gives it.
    pub fn contact_number(&self, id: ContactId) -> u32 {
        id.raw()
    }

    /// One contact the command holds, by its number. `None` if it holds no
    /// such contact.
    pub fn contact_by_number(&self, number: u32) -> Option<&'a Contact> {
        self.picture
            .contacts()
            .find(|contact| contact.id.raw() == number)
    }

    /// Whether the contact `id` of this picture is the command's own
    /// (decision 0137, 5): the report that last made or refreshed it here was
    /// the command's own search, strike or fight. False for one it was told
    /// of, and for every contact while command levels are off.
    pub fn is_own(&self, id: ContactId) -> bool {
        self.own.is_some_and(|own| own.contains(&id))
    }

    /// The command's commander (decision 0137, 1 and 3): `None` for a
    /// command with none named, who is steady, and for every command while
    /// command levels are off.
    pub fn commander(&self) -> Option<&'a Commander> {
        self.commander
    }

    /// What the command believes came of its side's fights, in the order the
    /// claims arrived.
    pub fn claims(&self) -> &'a [BattleClaim] {
        self.picture.claims()
    }

    /// The command's estimates of the enemy's men ashore, in hex order.
    pub fn ground_estimates(&self) -> impl Iterator<Item = &'a GroundEstimate> + 'a {
        self.picture.ground_estimates()
    }

    /// The command's estimate of the enemy's men ashore in `hex`, if any.
    pub fn ground_estimate(&self, hex: Hex) -> Option<&'a GroundEstimate> {
        self.picture.ground_estimate(hex)
    }

    /// The command's reports of submarines, in hex order (decision 0218,
    /// 44).
    pub fn submarine_reports(&self) -> impl Iterator<Item = &'a SubmarineReport> + 'a {
        self.picture.submarine_reports()
    }

    /// The command's suspected waters, in the order told (decision 0218,
    /// 47).
    pub fn suspected_waters(&self) -> &'a [SuspectedWaters] {
        self.picture.suspected_waters()
    }
}

impl<'a> SideView<'a> {
    /// Whether command levels are on in this game (decision 0134, 1).
    pub fn command_levels(&self) -> bool {
        self.commands.is_some()
    }

    /// Whether reports travel between the side's commands (decision 0135):
    /// command levels are on and the scenario has report figures. Then a
    /// command's picture holds what it saw or was sent, and pictures can
    /// differ from one another and from the side's own.
    pub fn reports_travel(&self) -> bool {
        self.commands.is_some() && self.ref_data.reports.is_some()
    }

    /// The side's commands, in command order: the area command, then its
    /// task forces, then its bases. Empty with command levels off: there are
    /// no commands then (decision 0134, 1).
    pub fn commands(&self) -> Vec<CommandRef> {
        // A seat at a command knows its own command and no other.
        if let Some(seat) = self.seat {
            return vec![seat];
        }
        self.commands
            .map(|commands| commands.keys().copied().collect())
            .unwrap_or_default()
    }

    /// The view of one of the side's own commands (decision 0134, 15).
    /// `None` for a command that is not the side's. With command levels off
    /// the area command and each of the side's own task forces and bases has
    /// a view, and each holds the side's own belief (item 16).
    pub fn command(&self, command: CommandRef) -> Option<CommandView<'a>> {
        // A seat at a command is given its own command's view and no other
        // (decision 0138, 5).
        if self.seat.is_some_and(|seat| seat != command) {
            return None;
        }
        match self.commands {
            Some(commands) => commands.get(&command).map(|post| CommandView {
                side: self.side,
                command,
                superior: post.superior(),
                picture: post.picture(),
                reports_travel: self.reports_travel(),
                own: Some(post.own()),
                commander: post.commander(),
            }),
            None => {
                let own = match command {
                    CommandRef::Area => true,
                    CommandRef::TaskForce(force) => self
                        .truth
                        .task_forces
                        .get(&force)
                        .is_some_and(|record| record.side == self.side),
                    CommandRef::Base(base) => self
                        .truth
                        .bases
                        .get(&base)
                        .is_some_and(|record| record.side == self.side),
                };
                own.then_some(CommandView {
                    side: self.side,
                    command,
                    superior: None,
                    picture: self.belief,
                    reports_travel: false,
                    own: None,
                    commander: None,
                })
            }
        }
    }

    /// The command one of the side's own air groups belongs to (decision
    /// 0134, 8). `None` for a group that is not the side's.
    pub fn command_of_group(&self, group: AirGroupId) -> Option<CommandRef> {
        self.truth
            .air_groups
            .get(&group)
            .filter(|record| self.owns_group(record))?;
        pictures::command_of_group(self.truth, group)
    }

    /// The command one of the side's own task forces is: itself, or the area
    /// command once it has ceased to be. `None` for a force not the side's.
    pub fn command_of_task_force(&self, force: TaskForceId) -> Option<CommandRef> {
        self.truth
            .task_forces
            .get(&force)
            .filter(|record| self.owns_force(record))?;
        pictures::command_of_task_force(self.truth, force)
    }

    /// The command one of the side's own land units belongs to (decision
    /// 0134, 8). `None` for a unit that is not the side's.
    pub fn command_of_land_unit(&self, unit: LandUnitId) -> Option<CommandRef> {
        self.truth
            .land_units
            .get(&unit)
            .filter(|record| self.owns_land_unit(record))?;
        pictures::command_of_land_unit(self.truth, unit)
    }

    /// The picture of one of the side's own commands, for the rules the
    /// view's choices share with the engine: the one reader (decision 0134,
    /// 16), the side's own belief with command levels off.
    pub(crate) fn picture(&self, command: CommandRef) -> &'a BeliefState {
        pictures::picture_of(self.belief, self.commands, command)
    }
}
