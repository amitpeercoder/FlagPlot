//! The playing of a trial (decision 0175).
//!
//! A trial is given a side's view, what that side believed, the trial's
//! figures, the reference data and the map, a seed, a trial's number, a
//! number of worlds and of days, and a course: the commander's commands of
//! the side tried. It makes the pictures of its worlds (decision 0174) and
//! plays each world in the order of its number, from 1: it builds the
//! world, hands in what the enemy is taken to want (rule 19 of decision
//! 0174) and the course for the side tried, both for the first morning, and
//! resolves the days of the horizon with both sides' staffs on doctrine
//! (`sim_staff::staffs`).
//!
//! It is worked a world's day at a time ([`Playing::step`]), so that
//! whoever asked may stop asking: what was played is dropped with the
//! value. A world's play depends on its own picture alone: no world reads
//! another or how many there are. A day that fails stops its world and is
//! kept with it; the other worlds are played.
//!
//! The only things a trial does to a world are the things a game does to
//! it between days: it hands in commands, and resolves a day with the
//! staffs. Every world it touches it built itself; it is given no world.

use std::collections::BTreeMap;

use sim_core::command::{CommandKind, Staff, StaffDecisions};
use sim_core::map::Map;
use sim_core::refdata::{RefData, TrialFigures};
use sim_core::state::Side;
use sim_core::view::SideView;
use sim_core::world::SimError;
use sim_core::World;

use crate::{build_world, pictures, Assumption, Believed, Refused, Trial, TrialPicture};

/// What a trial is asked to play (decision 0175, 1 and 2): its seed, its
/// number and its count of worlds; its horizon in days; and the course, the
/// commander's commands of the side tried, handed in for that side on the
/// first morning of every world. An empty course tries things as they
/// stand.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Asked {
    pub trial: Trial,
    /// The days each world is played: the horizon.
    pub days: u32,
    pub course: Vec<CommandKind>,
}

/// One day of a world as played: its number in the game, the world's hash
/// at its end, and what each side's staff handed in for it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PlayedDay {
    pub day: u32,
    pub hash: u64,
    pub staff: BTreeMap<Side, StaffDecisions>,
}

/// A day that failed in a world (decision 0175, 8): the world's number, the
/// day, and why.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Failed {
    pub world: u32,
    pub day: u32,
    pub error: SimError,
}

/// One world of a trial as played (decision 0175, 7): its number, what its
/// picture assumed, its days, the day that failed if one did, and the world
/// itself as its play left it. A world of a trial is the trial's own
/// invention: reading it is not reading the enemy.
#[derive(Clone, Debug)]
pub struct PlayedWorld {
    /// The world's number in its trial, from 1.
    pub number: u32,
    /// What was assumed in making its picture (decision 0174, 21).
    pub assumptions: Vec<Assumption>,
    /// The days played to their end, in order. A failed day is not here.
    pub days: Vec<PlayedDay>,
    /// The day that stopped the world, if one did.
    pub failed: Option<Failed>,
    /// The world as its play left it: at the end of its last day played.
    pub world: World,
}

/// A trial played out (decision 0175, 7): its worlds in the order of their
/// numbers, and any world whose picture could not be built.
#[derive(Clone, Debug)]
pub struct Played {
    pub worlds: Vec<PlayedWorld>,
    /// The worlds that could not be built from their pictures, each
    /// reported on its first day, in the order of their numbers.
    pub not_built: Vec<Failed>,
}

impl Played {
    /// Every world that failed, one entry each, in the order of the
    /// worlds' numbers: a day that failed, or a world not built. A trial
    /// with a failed world says so (rule 8).
    pub fn failed(&self) -> Vec<&Failed> {
        let mut failed: Vec<&Failed> = self
            .worlds
            .iter()
            .filter_map(|played| played.failed.as_ref())
            .chain(&self.not_built)
            .collect();
        failed.sort_by_key(|f| f.world);
        failed
    }
}

/// What one step of a trial did.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Stepped {
    /// A day of a world was played to its end.
    Day { world: u32, day: u32, hash: u64 },
    /// A day of a world failed; the world is stopped.
    Failed(Failed),
    /// A world could not be built from its picture; it is stopped before
    /// its first day, which is the day reported.
    NotBuilt(Failed),
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
    course: Vec<CommandKind>,
    staffs: BTreeMap<Side, Box<dyn Staff + Send>>,
    in_play: Option<PlayedWorld>,
    played: Vec<PlayedWorld>,
    not_built: Vec<Failed>,
    #[cfg(feature = "test-means")]
    break_on: Option<(u32, u32)>,
}

/// Begins a trial (decision 0175): makes the pictures of worlds 1 to
/// `asked.trial.worlds` from the side's view and beliefs, as [`pictures`]
/// does, and plays nothing yet. Refused as [`pictures`] refuses, and where
/// the horizon has no day.
pub fn begin(
    view: &SideView<'_>,
    believed: &Believed,
    figures: &TrialFigures,
    ref_data: &RefData,
    map: &Map,
    asked: Asked,
) -> Result<Playing, Refused> {
    if asked.days == 0 {
        return Err(Refused("a trial's horizon has no day".to_string()));
    }
    let made = pictures(view, believed, figures, asked.trial)?;
    Ok(Playing {
        to_begin: made.into_iter(),
        ref_data: ref_data.clone(),
        map: map.clone(),
        days: asked.days,
        course: asked.course,
        staffs: sim_staff::staffs(),
        in_play: None,
        played: Vec::new(),
        not_built: Vec::new(),
        #[cfg(feature = "test-means")]
        break_on: None,
    })
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

impl Playing {
    /// Plays the next day of the world in play, beginning the next world
    /// first where none is in play: builds it, hands in what the enemy is
    /// taken to want and the course, and resolves its first day. `None`
    /// once every world is played.
    pub fn step(&mut self) -> Option<Stepped> {
        if self.in_play.is_none() {
            let picture = self.to_begin.next()?;
            match self.begin_world(&picture) {
                Ok(played) => self.in_play = Some(played),
                Err(failed) => {
                    self.not_built.push(failed.clone());
                    return Some(Stepped::NotBuilt(failed));
                }
            }
        }
        let mut current = self.in_play.take()?;
        let day = current.world.day() + 1;
        #[cfg(feature = "test-means")]
        if self.break_on == Some((current.number, day)) {
            current.world.break_an_invariant_for_a_test();
        }
        let resolved = current
            .world
            .resolve_day_staffed(sim_staff::as_staffs(&self.staffs));
        let stepped = match resolved {
            Ok(summary) => {
                current.days.push(PlayedDay {
                    day: summary.day,
                    hash: summary.state_hash,
                    staff: current.world.staff_handed_in().clone(),
                });
                Stepped::Day {
                    world: current.number,
                    day: summary.day,
                    hash: summary.state_hash,
                }
            }
            Err(error) => {
                let failed = Failed {
                    world: current.number,
                    day,
                    error,
                };
                current.failed = Some(failed.clone());
                Stepped::Failed(failed)
            }
        };
        let finished =
            current.failed.is_some() || u32::try_from(current.days.len()).ok() >= Some(self.days);
        if finished {
            self.played.push(current);
        } else {
            self.in_play = Some(current);
        }
        Some(stepped)
    }

    /// The world in play, at the end of its last day played, if one is.
    pub fn in_play(&self) -> Option<&PlayedWorld> {
        self.in_play.as_ref()
    }

    /// The worlds played to their end so far, in the order of their
    /// numbers.
    pub fn played(&self) -> &[PlayedWorld] {
        &self.played
    }

    /// The worlds that could not be built so far.
    pub fn not_built(&self) -> &[Failed] {
        &self.not_built
    }

    /// Plays every step left and gives back the worlds.
    pub fn play_out(mut self) -> Played {
        while self.step().is_some() {}
        Played {
            worlds: self.played,
            not_built: self.not_built,
        }
    }

    /// A means for tests only (decision 0175), built only with the feature
    /// `test-means`: world `world` is made to break an invariant before its
    /// day `day` is resolved, so that that day fails.
    #[cfg(feature = "test-means")]
    pub fn break_world_on(&mut self, world: u32, day: u32) {
        self.break_on = Some((world, day));
    }

    /// Builds the world of a picture and hands in the course for the side
    /// tried, for the first morning. A world that cannot be built is played
    /// no further: it is reported as failed on its first day.
    fn begin_world(&self, picture: &TrialPicture) -> Result<PlayedWorld, Failed> {
        let first_day = picture.picture.day + 1;
        let mut built =
            build_world(picture, self.ref_data.clone(), self.map.clone()).map_err(|error| {
                Failed {
                    world: picture.world,
                    day: first_day,
                    error,
                }
            })?;
        if !self.course.is_empty() {
            built.submit_commands(picture.side, self.course.clone());
        }
        Ok(PlayedWorld {
            number: picture.world,
            assumptions: picture.assumptions.clone(),
            days: Vec::new(),
            failed: None,
            world: built,
        })
    }
}
