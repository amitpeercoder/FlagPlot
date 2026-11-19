//! The playing of a trial (decision 0175, as amended on review).
//!
//! A trial is given a side's view, what that side believed, the trial's
//! figures (its seed among them), the reference data and the map, a
//! trial's number, a number of worlds and of days, and a course: the
//! commander's commands of the side tried, each on a day of the world. It
//! makes the pictures of its worlds (decision 0174) and plays each world in
//! the order of its number, from 1: it builds the world (which hands in
//! what the enemy is taken to want, once, decision 0174, 19), and resolves
//! the days of the horizon with both sides' staffs on doctrine
//! (`sim_staff::staffs`), handing in each command of the course for the
//! morning of its day.
//!
//! It is worked a world's day at a time ([`Playing::step`]), so that
//! whoever asked may stop asking: what was played is dropped with the
//! value. A world's play depends on its own picture alone: no world reads
//! another or how many there are. The course is no part of a picture or of
//! a draw. A world that cannot be played (its picture refused when built,
//! or a day the engine fails) is kept with its record; the other worlds
//! are played.
//!
//! The only things a trial does to a world are the things a game does to
//! it between days: it hands in commands, and resolves a day with the
//! staffs; it reads the world's day, a day's hash and events, and what the
//! staffs handed in. Every world it touches it built itself; no call of
//! the crate takes a world.

use std::collections::BTreeMap;

use sim_core::command::{CommandKind, Leave, Staff, StaffDecisions};
use sim_core::events::EventKind;
use sim_core::map::Map;
use sim_core::orders::Issuer;
use sim_core::refdata::{RefData, TrialFigures};
use sim_core::state::Side;
use sim_core::view::SideView;
use sim_core::World;

use crate::{build_world, pictures, Assumption, Believed, Refused, Trial, TrialPicture};

/// One command of a course (amended point 1): a commander's command of the
/// side tried and the day of the world on whose morning it is handed in,
/// from 1, the world's first day.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CourseCommand {
    pub day: u32,
    pub command: CommandKind,
}

impl CourseCommand {
    /// A command for the world's first morning, the day a course's command
    /// has where none is given.
    pub fn first_day(command: CommandKind) -> CourseCommand {
        CourseCommand { day: 1, command }
    }
}

/// What a trial is asked to play (decision 0175, 1 and 2): its number, its
/// count of worlds, its horizon in days, and the course. The seed is the
/// trial's figures' (amended point 7). An empty course tries things as they
/// stand.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Asked {
    /// The trial's number, given by whoever asks: the same number is the
    /// same worlds.
    pub trial: u32,
    /// How many worlds: 1 to this.
    pub worlds: u32,
    /// The days each world is played: the horizon, played whole.
    pub days: u32,
    pub course: Vec<CourseCommand>,
}

/// One day of a world as played: its number in the game, the world's hash
/// at its end, and what each side's staff handed in for it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PlayedDay {
    pub day: u32,
    pub hash: u64,
    pub staff: BTreeMap<Side, StaffDecisions>,
}

/// A command of the course that a world refused (amended point 5): the day
/// of the game it was handed in for, the command and the engine's words.
/// The world plays on without it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RefusedCommand {
    pub day: u32,
    pub command: CommandKind,
    pub words: String,
}

/// Why a world could not be played (amended point 11): its number, the day
/// (0 where its picture was refused when its world was built), and the
/// engine's words.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Failed {
    pub world: u32,
    pub day: u32,
    pub words: String,
}

/// One world of a trial as played (amended point 10): its number, what its
/// picture assumed, its days, the commands of the course it refused, why it
/// could not be played further if it could not, and the world as it stands
/// at the end. A world of a trial is the trial's own invention: reading it
/// is not reading the enemy.
#[derive(Clone, Debug)]
pub struct PlayedWorld {
    /// The world's number in its trial, from 1.
    pub number: u32,
    /// What was assumed in making its picture (decision 0174, 21).
    pub assumptions: Vec<Assumption>,
    /// The days played to their end, in order.
    pub days: Vec<PlayedDay>,
    /// The commands of the course this world refused, in the order taken.
    pub refused: Vec<RefusedCommand>,
    /// Why the world was stopped, where it was.
    pub failed: Option<Failed>,
    /// The world as its play left it; `None` where its picture was refused.
    pub world: Option<World>,
}

/// A trial played out (decision 0175, 7): its worlds in the order of their
/// numbers, and what the record of the trial says of itself.
#[derive(Clone, Debug)]
pub struct Played {
    pub worlds: Vec<PlayedWorld>,
    /// The count of worlds is not a power of two, so the forces of unknown
    /// standing were not tried evenly (amended point 9).
    pub uneven: bool,
    /// The leaves the course gives that the side had not given, in the
    /// order of the course (amended point 4).
    pub leaves_given: Vec<Leave>,
}

impl Played {
    /// Every world that could not be played, in the order of the worlds'
    /// numbers. A trial with one says so (rule 8).
    pub fn failed(&self) -> Vec<&Failed> {
        self.worlds
            .iter()
            .filter_map(|played| played.failed.as_ref())
            .collect()
    }
}

/// What one step of a trial did.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Stepped {
    /// A day of a world was played to its end.
    Day { world: u32, day: u32, hash: u64 },
    /// A world could not be played: its picture refused (day 0), or a day
    /// failed. The world is stopped.
    Failed(Failed),
}

/// A trial being played, a world's day at a time (decision 0175, 5). It
/// holds what it was given and the worlds it built, and nothing of the live
/// game: dropping it keeps nothing. Stepping it to the end and
/// [`Playing::play_out`] give the same worlds.
pub struct Playing {
    /// The pictures of the worlds still to begin, the next first.
    to_begin: std::vec::IntoIter<TrialPicture>,
    ref_data: RefData,
    map: Map,
    days: u32,
    course: Vec<CourseCommand>,
    staffs: BTreeMap<Side, Box<dyn Staff + Send>>,
    in_play: Option<InPlay>,
    played: Vec<PlayedWorld>,
    uneven: bool,
    leaves_given: Vec<Leave>,
    /// A day made to fail by a test of the crate: the world and the day of
    /// the world, from 1.
    #[cfg(test)]
    pub(crate) fail_on: Option<(u32, u32)>,
}

/// A world in play: its record and the side tried.
struct InPlay {
    record: PlayedWorld,
    side: Side,
}

/// Begins a trial (decision 0175): makes the pictures of worlds 1 to
/// `asked.worlds` from the side's view and beliefs on the trial's figures'
/// seed and `asked.trial`, as [`pictures`] does, and plays nothing yet.
/// Refused as [`pictures`] refuses, and as [`begin_from_pictures`] refuses.
pub fn begin(
    view: &SideView<'_>,
    believed: &Believed,
    figures: &TrialFigures,
    ref_data: &RefData,
    map: &Map,
    asked: Asked,
) -> Result<Playing, Refused> {
    let trial = Trial {
        seed: figures.seed,
        trial: asked.trial,
        worlds: asked.worlds,
    };
    let made = pictures(view, believed, figures, trial)?;
    begin_from_pictures(made, ref_data, map, asked)
}

/// A trial played out: [`begin`], then every step.
pub fn play(
    view: &SideView<'_>,
    believed: &Believed,
    figures: &TrialFigures,
    ref_data: &RefData,
    map: &Map,
    asked: Asked,
) -> Result<Played, Refused> {
    Ok(begin(view, believed, figures, ref_data, map, asked)?.play_out())
}

/// Begins a trial from pictures handed in, played in the order given, in
/// place of the pictures [`begin`] makes: for a test that alters one. Every
/// picture must be of one side. Refused where the horizon has no day, or a
/// command of the course has a day outside the horizon.
pub fn begin_from_pictures(
    made: Vec<TrialPicture>,
    ref_data: &RefData,
    map: &Map,
    asked: Asked,
) -> Result<Playing, Refused> {
    if asked.days == 0 {
        return Err(Refused("a trial's horizon has no day".to_string()));
    }
    if let Some(outside) = asked
        .course
        .iter()
        .find(|c| c.day == 0 || c.day > asked.days)
    {
        return Err(Refused(format!(
            "a command of the course is for day {} of the world, outside the horizon of {} days",
            outside.day, asked.days
        )));
    }
    if made.windows(2).any(|pair| pair[0].side != pair[1].side) {
        return Err(Refused(
            "the pictures handed in are of two sides".to_string(),
        ));
    }
    // The leaves the course gives that the side has not given (amended
    // point 4), read from its own half: the same in every picture.
    let held = made
        .first()
        .and_then(|first| first.picture.command.get(&first.side))
        .map(|state| state.leaves)
        .unwrap_or_default();
    let mut leaves_given = Vec::new();
    for c in &asked.course {
        if let CommandKind::SetLeave { leave, given: true } = c.command {
            let had = match leave {
                Leave::Prepare => held.prepare,
                Leave::Launch => held.launch,
            };
            if !had && !leaves_given.contains(&leave) {
                leaves_given.push(leave);
            }
        }
    }
    Ok(Playing {
        to_begin: made.into_iter(),
        ref_data: ref_data.clone(),
        map: map.clone(),
        days: asked.days,
        course: asked.course,
        staffs: sim_staff::staffs(),
        in_play: None,
        played: Vec::new(),
        uneven: !asked.worlds.is_power_of_two(),
        leaves_given,
        #[cfg(test)]
        fail_on: None,
    })
}

impl Playing {
    /// Plays the next day of the world in play, beginning the next world
    /// first where none is in play. `None` once every world is played.
    pub fn step(&mut self) -> Option<Stepped> {
        if self.in_play.is_none() {
            let picture = self.to_begin.next()?;
            match build_world(&picture, self.ref_data.clone(), self.map.clone()) {
                Ok(built) => {
                    self.in_play = Some(InPlay {
                        record: PlayedWorld {
                            number: picture.world,
                            assumptions: picture.assumptions,
                            days: Vec::new(),
                            refused: Vec::new(),
                            failed: None,
                            world: Some(built),
                        },
                        side: picture.side,
                    });
                }
                Err(error) => {
                    let failed = Failed {
                        world: picture.world,
                        day: 0,
                        words: error.to_string(),
                    };
                    self.played.push(PlayedWorld {
                        number: picture.world,
                        assumptions: picture.assumptions,
                        days: Vec::new(),
                        refused: Vec::new(),
                        failed: Some(failed.clone()),
                        world: None,
                    });
                    return Some(Stepped::Failed(failed));
                }
            }
        }
        let mut current = self.in_play.take()?;
        let stepped = self.play_day(&mut current);
        let finished = current.record.failed.is_some()
            || u32::try_from(current.record.days.len()).ok() >= Some(self.days);
        if finished {
            self.played.push(current.record);
        } else {
            self.in_play = Some(current);
        }
        Some(stepped)
    }

    /// Hands in the course's commands of the world's next day and resolves
    /// it with the staffs.
    fn play_day(&self, current: &mut InPlay) -> Stepped {
        let number = current.record.number;
        let of_world = u32::try_from(current.record.days.len()).unwrap_or(u32::MAX) + 1;
        let Some(world) = current.record.world.as_mut() else {
            // A world with no world is recorded as refused when it was
            // begun, and never kept in play.
            return Stepped::Failed(Failed {
                world: number,
                day: 0,
                words: "the world was not built".to_string(),
            });
        };
        let day = world.day() + 1;
        let commands: Vec<CommandKind> = self
            .course
            .iter()
            .filter(|c| c.day == of_world)
            .map(|c| c.command.clone())
            .collect();
        if !commands.is_empty() {
            world.submit_commands(current.side, commands);
        }
        #[cfg(test)]
        let resolved = if self.fail_on == Some((number, of_world)) {
            Err(sim_core::world::SimError::InvalidSetup(
                "a day made to fail by a test of the crate".to_string(),
            ))
        } else {
            world.resolve_day_staffed(sim_staff::as_staffs(&self.staffs))
        };
        #[cfg(not(test))]
        let resolved = world.resolve_day_staffed(sim_staff::as_staffs(&self.staffs));
        match resolved {
            Ok(summary) => {
                // The commands of the side tried that the engine refused this
                // day: only the course hands in a commander's command for that
                // side.
                for event in world
                    .events()
                    .iter()
                    .skip(summary.first_event)
                    .take(summary.event_count)
                {
                    if let EventKind::CommandRefused {
                        side,
                        issuer: Issuer::Player,
                        command,
                        reason,
                    } = &event.kind
                    {
                        if *side == current.side {
                            current.record.refused.push(RefusedCommand {
                                day: summary.day,
                                command: command.clone(),
                                words: reason.to_string(),
                            });
                        }
                    }
                }
                current.record.days.push(PlayedDay {
                    day: summary.day,
                    hash: summary.state_hash,
                    staff: world.staff_handed_in().clone(),
                });
                Stepped::Day {
                    world: number,
                    day: summary.day,
                    hash: summary.state_hash,
                }
            }
            Err(error) => {
                let failed = Failed {
                    world: number,
                    day,
                    words: error.to_string(),
                };
                current.record.failed = Some(failed.clone());
                Stepped::Failed(failed)
            }
        }
    }

    /// The pictures of the worlds not yet begun, the next first: before the
    /// first step, every picture of the trial.
    pub fn to_play(&self) -> &[TrialPicture] {
        self.to_begin.as_slice()
    }

    /// The world in play, at the end of its last day played, if one is.
    pub fn in_play(&self) -> Option<&PlayedWorld> {
        self.in_play.as_ref().map(|current| &current.record)
    }

    /// The worlds played to their end, or stopped, so far, in the order of
    /// their numbers.
    pub fn played(&self) -> &[PlayedWorld] {
        &self.played
    }

    /// Plays every step left and gives back the worlds.
    pub fn play_out(mut self) -> Played {
        while self.step().is_some() {}
        Played {
            worlds: self.played,
            uneven: self.uneven,
            leaves_given: self.leaves_given,
        }
    }
}

#[cfg(test)]
#[path = "play_tests.rs"]
mod tests;
