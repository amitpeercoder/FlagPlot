//! Commands and their pictures (decision 0134, step 1 of stage 2b).
//!
//! A scenario may turn command levels on. Each side then has an area
//! command, and each of its task forces and bases is a command under it,
//! each with a picture of its own: a [`BeliefState`], changed only through
//! [`BeliefState::apply_report`] and by ageing, as a side's belief is.
//!
//! There is one path. Every report a side receives goes through
//! [`World::apply_report`], which applies it to the side's own belief and
//! then to every command's picture of the side, in the same pulse and the
//! same order (item 10), or, with report figures (decision 0135), to the
//! pictures of the commands that hold it first, the rest being told by
//! [`World::pass_report`](crate::passing); every ageing goes through
//! [`World::age_pictures`].
//! Every rule that reads what a side believes asks [`World::picture`] (or
//! [`SideView`](crate::view::SideView)'s own reader, which calls the same
//! [`picture_of`]) for the picture of the command that owns the decision
//! (item 13). With command levels off there is no command and the picture
//! of every command is the side's own belief (item 16), so a game with them
//! off reads exactly what it read before.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::belief::{AgeOutcome, ApplyOutcome, BeliefState, Report};
use crate::ids::{AirGroupId, BaseId, EventId, LandUnitId, TaskForceId};
use crate::refdata::SearchFigures;
use crate::state::{AirGroupLocation, ShipLocation, Side, TruthState};
use crate::time::Pulse;
use crate::world::{SimError, World};

/// A command of a side (decision 0134, 3 and 4): its area command, one of
/// its task forces or one of its bases. A ship, an air group and a land unit
/// are not commands; each belongs to one (item 8).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum CommandRef {
    /// The side's area command, at the top of its chain.
    Area,
    /// A task force of the side.
    TaskForce(TaskForceId),
    /// A base of the side.
    Base(BaseId),
}

impl CommandRef {
    /// A stable number. Part of the state hash. Never renumber.
    pub const fn code(self) -> u8 {
        match self {
            CommandRef::Area => 1,
            CommandRef::TaskForce(_) => 2,
            CommandRef::Base(_) => 3,
        }
    }

    /// The number of the task force or base, 0 for the area command. Part of
    /// the state hash, after the code.
    pub const fn raw(self) -> u32 {
        match self {
            CommandRef::Area => 0,
            CommandRef::TaskForce(force) => force.raw(),
            CommandRef::Base(base) => base.raw(),
        }
    }
}

impl fmt::Display for CommandRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandRef::Area => f.write_str("the area command"),
            CommandRef::TaskForce(force) => write!(f, "{force}"),
            CommandRef::Base(base) => write!(f, "{base}"),
        }
    }
}

/// A command a scenario places under another command than its side's area
/// command (decision 0134, 4). Loaded and checked; two levels are what is
/// built and proved, and in this step no rule reads a superior.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct NamedSuperior {
    pub side: Side,
    pub command: CommandRef,
    pub superior: CommandRef,
}

/// One command of a side while command levels are on: its superior (none
/// for the area command) and its picture.
#[derive(Clone, Debug)]
pub struct CommandPost {
    pub(crate) superior: Option<CommandRef>,
    pub(crate) picture: BeliefState,
    /// The reports this command has heard (decision 0135, 13), by the
    /// report's event, each with the day and pulse it was made: so that one
    /// reaching it by two roads is applied once. Pruned when older than a
    /// contact may be.
    pub(crate) heard: BTreeMap<EventId, (u32, Pulse)>,
}

impl CommandPost {
    /// The command this one is under: `None` for the area command.
    pub fn superior(&self) -> Option<CommandRef> {
        self.superior
    }

    /// The command's picture.
    pub fn picture(&self) -> &BeliefState {
        &self.picture
    }

    /// The reports the command has heard, by the report's event, each with
    /// the day and pulse it was made (decision 0135, 13). For the hash and
    /// the umpire.
    pub fn heard(&self) -> &BTreeMap<EventId, (u32, Pulse)> {
        &self.heard
    }
}

/// Who holds a report first (decision 0135, 1): the command or commands that
/// saw the thing, or every command at once.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) enum Holders {
    /// Every command, in the pulse the report is made: what a scenario
    /// starts a side with, and every report when there are no report
    /// figures.
    Every,
    /// These commands first; the rest are told by
    /// [`World::pass_report`](crate::passing). `key` is the event the report
    /// is known by (its own event; for "nothing there" the strike's), and
    /// `day` and `pulse` when it is made. `pulse` is `None` for a report
    /// made in an end-of-day phase, which belongs to no pulse: it is passed
    /// as the afternoon's last word, its draws and events keyed on no pulse.
    Commands {
        commands: Vec<CommandRef>,
        key: EventId,
        day: u32,
        pulse: Option<Pulse>,
    },
}

/// The commands of one side, by command.
pub type SideCommands = BTreeMap<CommandRef, CommandPost>;

/// The picture of `command`, from a side's own belief and, with command
/// levels on, its commands. The one reader every rule goes through (item
/// 16): with command levels off it is the side's own belief, whatever the
/// command. With them on a command that is not one is a bug in the rules.
pub(crate) fn picture_of<'a>(
    own: &'a BeliefState,
    commands: Option<&'a SideCommands>,
    command: CommandRef,
) -> &'a BeliefState {
    match commands {
        None => own,
        Some(commands) => {
            &commands
                .get(&command)
                .unwrap_or_else(|| {
                    panic!(
                        "{command} is not a command of the {} side",
                        own.side().name()
                    )
                })
                .picture
        }
    }
}

/// Whether task force `force` has ceased to be (decision 0134, 6): no ship
/// in it and none away at a yard to come back to it (decision 0069). Merged,
/// sunk or dissolved, it stays in the state with no ship.
fn force_is_gone(truth: &TruthState, force: TaskForceId) -> bool {
    let Some(record) = truth.task_forces.get(&force) else {
        return true;
    };
    record.ships.is_empty()
        && !truth
            .ships
            .values()
            .any(|ship| ship.location == ShipLocation::Yard(force))
}

/// The command task force `force` of `side` is: itself while it is one,
/// else the area command.
fn command_of_force(truth: &TruthState, side: Side, force: TaskForceId) -> CommandRef {
    match truth.task_forces.get(&force) {
        Some(record) if record.side == side && !force_is_gone(truth, force) => {
            CommandRef::TaskForce(force)
        }
        _ => CommandRef::Area,
    }
}

/// The command base `base` is for `side`: itself while it is the side's,
/// else the area command.
fn command_of_base(truth: &TruthState, side: Side, base: BaseId) -> CommandRef {
    match truth.bases.get(&base) {
        Some(record) if record.side == side => CommandRef::Base(base),
        _ => CommandRef::Area,
    }
}

/// The command an air group belongs to (decision 0134, 8): the task force
/// of the ship it is aboard, or its base. Aboard a ship in port (in no
/// force), the base she is at; a ship away at a yard, the force she will
/// rejoin. Anywhere else (a place lost, a base or force not its side's), the
/// area command. `None` for no such group.
pub(crate) fn command_of_group(truth: &TruthState, group: AirGroupId) -> Option<CommandRef> {
    let record = truth.air_groups.get(&group)?;
    let side = record.side;
    Some(match record.location {
        AirGroupLocation::Base(base) => command_of_base(truth, side, base),
        AirGroupLocation::Ship(ship) => match truth.ships.get(&ship).map(|s| s.location) {
            Some(ShipLocation::TaskForce(force) | ShipLocation::Yard(force)) => {
                command_of_force(truth, side, force)
            }
            Some(ShipLocation::Port(base)) => command_of_base(truth, side, base),
            Some(ShipLocation::Sunk) | None => CommandRef::Area,
        },
        AirGroupLocation::Lost => CommandRef::Area,
    })
}

/// The command a land unit belongs to (decision 0134, 8): the base of its
/// side in the hex its men ashore are in, the lowest-numbered if there are
/// two; the area command where it is at no base of its side (aboard ship,
/// or ashore away from one). `None` for no such unit.
pub(crate) fn command_of_land_unit(truth: &TruthState, unit: LandUnitId) -> Option<CommandRef> {
    let record = truth.land_units.get(&unit)?;
    let Some(hex) = record.hex else {
        return Some(CommandRef::Area);
    };
    Some(
        truth
            .bases
            .values()
            .find(|base| base.side == record.side && base.hex == hex)
            .map_or(CommandRef::Area, |base| CommandRef::Base(base.id)),
    )
}

/// The command whose picture a fight ashore in `hex` reads (decision 0134,
/// 13): the base of `side` there, the lowest-numbered if there are two, else
/// the area command.
pub(crate) fn command_at_hex(truth: &TruthState, side: Side, hex: crate::hex::Hex) -> CommandRef {
    truth
        .bases
        .values()
        .find(|base| base.side == side && base.hex == hex)
        .map_or(CommandRef::Area, |base| CommandRef::Base(base.id))
}

/// The command task force `force` is, for the side that has it: itself
/// while it is one, else the area command. `None` for no such force.
pub(crate) fn command_of_task_force(truth: &TruthState, force: TaskForceId) -> Option<CommandRef> {
    let side = truth.task_forces.get(&force)?.side;
    Some(command_of_force(truth, side, force))
}

impl World {
    /// Turns command levels on (decision 0134, 1 to 4 and 12), before day 1
    /// and before any event: each side gets its area command, and each of
    /// its task forces with a ship and each of its bases a command under the
    /// area command, or under the command `named` gives it. Each picture
    /// starts as a copy of the side's own belief; a side's starting contacts
    /// and opening estimates, given after, reach every picture as they reach
    /// the side's. Fails if the game has begun, if command levels are on
    /// already, or if a named superior is not a command of the side at the
    /// start, names its own command, names the area command as the command
    /// placed, is named twice for one command, or makes a loop.
    pub fn turn_command_levels_on(&mut self, named: &[NamedSuperior]) -> Result<(), SimError> {
        if self.truth.day > 0 || !self.events.is_empty() {
            return Err(SimError::InvalidSetup(
                "command levels can only be turned on before day 1 and before any event"
                    .to_string(),
            ));
        }
        if self.chain.is_some() {
            return Err(SimError::InvalidSetup(
                "command levels are on already".to_string(),
            ));
        }
        let mut chain: BTreeMap<Side, SideCommands> = BTreeMap::new();
        for side in Side::ALL {
            let own = &self.beliefs[&side];
            let mut posts = SideCommands::new();
            posts.insert(
                CommandRef::Area,
                CommandPost {
                    superior: None,
                    picture: own.clone(),
                    heard: BTreeMap::new(),
                },
            );
            for force in self.truth.task_forces.values() {
                if force.side == side && !force_is_gone(&self.truth, force.id) {
                    posts.insert(
                        CommandRef::TaskForce(force.id),
                        CommandPost {
                            superior: Some(CommandRef::Area),
                            picture: own.clone(),
                            heard: BTreeMap::new(),
                        },
                    );
                }
            }
            for base in self.truth.bases.values() {
                if base.side == side {
                    posts.insert(
                        CommandRef::Base(base.id),
                        CommandPost {
                            superior: Some(CommandRef::Area),
                            picture: own.clone(),
                            heard: BTreeMap::new(),
                        },
                    );
                }
            }
            chain.insert(side, posts);
        }
        let mut placed: BTreeSet<(Side, CommandRef)> = BTreeSet::new();
        for entry in named {
            let describe = || {
                format!(
                    "the {} superior named for {}",
                    entry.side.name(),
                    entry.command
                )
            };
            let posts = chain.get_mut(&entry.side).expect("both sides");
            if entry.command == CommandRef::Area {
                return Err(SimError::InvalidSetup(format!(
                    "{}: the area command is under no command",
                    describe()
                )));
            }
            if !posts.contains_key(&entry.command) {
                return Err(SimError::InvalidSetup(format!(
                    "{}: {} is not a command of the side at the start",
                    describe(),
                    entry.command
                )));
            }
            if !posts.contains_key(&entry.superior) {
                return Err(SimError::InvalidSetup(format!(
                    "{}: {} is not a command of the side at the start",
                    describe(),
                    entry.superior
                )));
            }
            if entry.superior == entry.command {
                return Err(SimError::InvalidSetup(format!(
                    "{}: a command cannot be its own superior",
                    describe()
                )));
            }
            if !placed.insert((entry.side, entry.command)) {
                return Err(SimError::InvalidSetup(format!(
                    "{}: a superior is named for it twice",
                    describe()
                )));
            }
            posts
                .get_mut(&entry.command)
                .expect("checked above")
                .superior = Some(entry.superior);
        }
        // Every chain must reach the area command.
        for (side, posts) in &chain {
            for start in posts.keys() {
                let mut at = *start;
                let mut steps = 0usize;
                while let Some(up) = posts[&at].superior {
                    at = up;
                    steps += 1;
                    if steps > posts.len() {
                        return Err(SimError::InvalidSetup(format!(
                            "the {} superiors named make a loop through {start}",
                            side.name()
                        )));
                    }
                }
            }
        }
        self.chain = Some(chain);
        // With report figures the area command holds each command under it,
        // exactly, as of the start (decision 0136, 4).
        self.begin_holding();
        Ok(())
    }

    /// Whether command levels are on (decision 0134, 1).
    pub fn command_levels(&self) -> bool {
        self.chain.is_some()
    }

    /// The commands of `side` and their pictures, while command levels are
    /// on: for the umpire, tests and hashing. A side reads its own commands'
    /// pictures through [`SideView::command`](crate::view::SideView::command).
    pub fn commands(&self, side: Side) -> Option<&SideCommands> {
        self.chain.as_ref().map(|chain| &chain[&side])
    }

    /// The picture of `command` of `side` (decision 0134, 13 and 16): with
    /// command levels off, the side's own belief.
    pub(crate) fn picture(&self, side: Side, command: CommandRef) -> &BeliefState {
        picture_of(
            &self.beliefs[&side],
            self.chain.as_ref().map(|chain| &chain[&side]),
            command,
        )
    }

    /// The command an air group belongs to (decision 0134, 8).
    pub(crate) fn command_of_group(&self, group: AirGroupId) -> CommandRef {
        command_of_group(&self.truth, group).expect("the group exists")
    }

    /// The command a task force is: itself while it is one, else (a force
    /// that has ceased to be) the area command (decision 0134, 6).
    pub(crate) fn command_of_force(&self, force: TaskForceId) -> CommandRef {
        command_of_task_force(&self.truth, force).expect("the force exists")
    }

    /// Whether reports travel between commands: command levels are on and
    /// the scenario has report figures (decision 0135, 10).
    pub(crate) fn reports_travel(&self) -> bool {
        self.chain.is_some() && self.ref_data.reports.is_some()
    }

    /// Applies a report to `side`'s belief and then, with command levels on,
    /// to the pictures of the commands that hold it first, in command order
    /// (decision 0134, 10; decision 0135, 1). The one way a report reaches a
    /// side. Returns what it did to the side's own belief.
    ///
    /// With command levels off the side's belief alone changes. With them on
    /// and no report figures, or [`Holders::Every`], every command's picture
    /// gets it at once, as in decision 0134. With report figures and
    /// [`Holders::Commands`] only the holders' pictures get it, each marked
    /// as having heard it; the rest are told by [`World::pass_report`], once
    /// the caller has recorded the report's event. A holder that is not a
    /// command of the side now is replaced by the area command.
    ///
    /// "Nothing there" (decision 0135, 16): with report figures it reaches a
    /// holder's picture by contact number, and the side's own by hex; the
    /// rest hear it by hex when it is passed on.
    pub(crate) fn apply_report(
        &mut self,
        side: Side,
        report: &Report,
        holders: &Holders,
        figures: &SearchFigures,
    ) -> ApplyOutcome {
        let travel = self.reports_travel();
        let holders = match holders {
            Holders::Commands {
                commands,
                key,
                day,
                pulse,
            } if travel => Some((self.resolve_holders(side, commands), *key, *day, *pulse)),
            _ => None,
        };
        let by_hex = by_hex(report);
        let own_report = if holders.is_some() { &by_hex } else { report };
        let outcome = self
            .beliefs
            .get_mut(&side)
            .expect("both sides have a belief state")
            .apply_report(own_report, figures);
        if let Some(chain) = self.chain.as_mut() {
            let posts = chain.get_mut(&side).expect("both sides");
            match holders {
                None => {
                    for post in posts.values_mut() {
                        post.picture.apply_report(report, figures);
                    }
                }
                Some((holders, key, day, pulse)) => {
                    for (command, post) in posts.iter_mut() {
                        if holders.contains(command) {
                            post.picture.apply_report(report, figures);
                            post.heard
                                .insert(key, (day, pulse.unwrap_or(Pulse::Afternoon)));
                        }
                    }
                }
            }
        }
        outcome
    }

    /// Whether `command` is one of `side`'s now (the area command always).
    pub(crate) fn is_command(&self, side: Side, command: CommandRef) -> bool {
        self.chain
            .as_ref()
            .is_some_and(|chain| chain[&side].contains_key(&command))
    }

    /// The holders as commands of the side now, without repeats, in command
    /// order: one that is not a command (a force that has ceased to be) is
    /// replaced by the area command.
    pub(crate) fn resolve_holders(&self, side: Side, commands: &[CommandRef]) -> Vec<CommandRef> {
        let held: BTreeSet<CommandRef> = commands
            .iter()
            .map(|command| {
                if self.is_command(side, *command) {
                    *command
                } else {
                    CommandRef::Area
                }
            })
            .collect();
        held.into_iter().collect()
    }

    /// Ages `side`'s belief and, with command levels on, every command's
    /// picture of the side (decision 0134, 9 and 10). Returns what it did to
    /// the side's own belief.
    pub(crate) fn age_pictures(&mut self, side: Side, figures: &SearchFigures) -> Vec<AgeOutcome> {
        let outcomes = self
            .beliefs
            .get_mut(&side)
            .expect("both sides have a belief state")
            .age_contacts(figures);
        if let Some(chain) = self.chain.as_mut() {
            for post in chain.get_mut(&side).expect("both sides").values_mut() {
                post.picture.age_contacts(figures);
            }
        }
        outcomes
    }

    /// A task force was formed during the game (decision 0134, 5): with
    /// command levels on it is a command under the area command, with a copy
    /// of the picture of `from`, the command that formed it.
    pub(crate) fn command_formed(&mut self, force: TaskForceId, from: CommandRef) {
        let side = self.truth.task_forces[&force].side;
        self.add_command(side, CommandRef::TaskForce(force), from);
        self.hold_now(side, CommandRef::TaskForce(force));
    }

    /// A base was formed during the game, by a landing (decision 0066, 11):
    /// with command levels on it is a command under the area command, with a
    /// copy of the picture of `from`, the command whose landing formed it
    /// (a reading of decision 0134, 5, which names task forces only).
    pub(crate) fn base_formed(&mut self, base: BaseId, from: CommandRef) {
        let side = self.truth.bases[&base].side;
        self.add_command(side, CommandRef::Base(base), from);
        self.hold_now(side, CommandRef::Base(base));
    }

    /// A base changed side from `from` to `to` (decision 0134, 7): with
    /// command levels on it stops being a command of `from` and becomes one
    /// of `to`, under its area command, with a copy of that area command's
    /// picture.
    pub(crate) fn base_changed_side(&mut self, base: BaseId, from: Side) {
        let to = self.truth.bases[&base].side;
        let Some(chain) = self.chain.as_mut() else {
            return;
        };
        chain
            .get_mut(&from)
            .expect("both sides")
            .remove(&CommandRef::Base(base));
        rehome(chain.get_mut(&from).expect("both sides"));
        self.add_command(to, CommandRef::Base(base), CommandRef::Area);
        // The losing side's area command drops what it held of the base; the
        // winning side's holds it as of now (decision 0136, 5).
        self.drop_held(from, CommandRef::Base(base));
        self.hold_now(to, CommandRef::Base(base));
    }

    /// Ends the commands of task forces that have ceased to be (decision
    /// 0134, 6): their pictures are gone, and a command that was under one
    /// is under the area command. Run at the end of every pulse, before the
    /// pictures age: a force joined, stood down or sunk in a pulse, or at
    /// intake, is a command until the end of that pulse.
    ///
    /// The commands that ended are kept in `gone_commands` until the pulse's
    /// state reports are made: each makes one last report (decision 0136, 2).
    pub(crate) fn end_gone_commands(&mut self) {
        let Some(chain) = self.chain.as_mut() else {
            return;
        };
        let truth = &self.truth;
        let reporting = self.held.is_some();
        for (side, posts) in chain.iter_mut() {
            let before: Vec<CommandRef> = posts.keys().copied().collect();
            posts.retain(|command, _| match command {
                CommandRef::TaskForce(force) => !force_is_gone(truth, *force),
                CommandRef::Area | CommandRef::Base(_) => true,
            });
            rehome(posts);
            if reporting {
                self.gone_commands.extend(
                    before
                        .into_iter()
                        .filter(|command| !posts.contains_key(command))
                        .map(|command| (*side, command)),
                );
            }
        }
    }

    fn add_command(&mut self, side: Side, command: CommandRef, from: CommandRef) {
        let Some(chain) = self.chain.as_mut() else {
            return;
        };
        let posts = chain.get_mut(&side).expect("both sides");
        let source = posts
            .get(&from)
            .or_else(|| posts.get(&CommandRef::Area))
            .expect("a side always has its area command");
        let picture = source.picture.clone();
        // A command formed by copy has heard what the picture it copies has
        // (decision 0135, 13).
        let heard = source.heard.clone();
        posts.insert(
            command,
            CommandPost {
                superior: Some(CommandRef::Area),
                picture,
                heard,
            },
        );
    }
}

/// The report as a picture that did not fly the strike hears it (decision
/// 0135, 16): "nothing there" by its hex alone. Any other report as it is.
pub(crate) fn by_hex(report: &Report) -> Report {
    match report {
        Report::NothingThere { hex, .. } => Report::NothingThere {
            contact: None,
            hex: *hex,
        },
        other => other.clone(),
    }
}

/// A command whose superior is gone is under the area command (a reading:
/// decision 0134 builds and proves two levels and does not say).
fn rehome(posts: &mut SideCommands) {
    let present: BTreeSet<CommandRef> = posts.keys().copied().collect();
    for post in posts.values_mut() {
        if let Some(up) = post.superior {
            if !present.contains(&up) {
                post.superior = Some(CommandRef::Area);
            }
        }
    }
}
