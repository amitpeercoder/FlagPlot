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
//! staffs handed in; and at each day's end it takes the side tried's view,
//! to record where bases stand, on the first day the side's plan orders
//! (decision 0176, amended 1), and at the end of the day the course is
//! taken in, the side's plan orders and the landings it has formed
//! (decision 0177, amended 7). Every world it touches it built itself; no
//! call of the crate takes a world.

use std::collections::{BTreeMap, BTreeSet};

use sim_core::command::{CommandKind, IntentKind, Leave, PlanOrder, Staff, StaffDecisions};
use sim_core::events::EventKind;
use sim_core::hex::Hex;
use sim_core::ids::{IntentId, LandUnitId, TaskForceId};
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
/// at its end, what each side's staff handed in for it, and where the side
/// tried and the enemy on its map held bases at its end (decision 0176,
/// amended 1).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PlayedDay {
    pub day: u32,
    pub hash: u64,
    pub staff: BTreeMap<Side, StaffDecisions>,
    /// The bases at the day's end, as the side tried sees them.
    pub bases: BasesAt,
}

/// Where bases stood at the end of a day, as a side sees them (decision
/// 0176, amended 1): the hexes in which the side holds a base, and the hexes
/// of the enemy's bases on its map. What the test of a place being the side's
/// reads ([`crate::met`]).
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct BasesAt {
    /// The hexes in which the side holds a base.
    pub own: BTreeSet<Hex>,
    /// The hexes of the enemy's bases on the side's map.
    pub enemy: BTreeSet<Hex>,
}

impl BasesAt {
    /// Where bases stand in a side's view: its own bases' hexes, and the
    /// hexes of the bases on its map that are not its own. A live game's view
    /// gives the same as a world's.
    pub fn of(view: &SideView<'_>) -> BasesAt {
        let side = view.side();
        BasesAt {
            own: view.bases().map(|base| base.hex).collect(),
            enemy: view
                .enemy_bases()
                .filter(|base| base.side != side)
                .map(|base| base.hex)
                .collect(),
        }
    }
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
    /// The plan orders of the side tried, as its staff works them from its
    /// view at the end of the first day played (decision 0176, amended 1);
    /// `None` where no day was played.
    pub plan_orders: Option<Vec<PlanOrder>>,
    /// What the side tried had at the end of the day its course was taken
    /// in (decision 0177, amended 7); `None` where the course has no command
    /// or that day was not played.
    pub taken_in: Option<TakenIn>,
    /// The world as its play left it; `None` where its picture was refused.
    pub world: Option<World>,
}

/// What the side tried had at the end of the day its course's first command
/// was taken in (decision 0177, amended 7): its plan orders as its staff
/// works them, and each landing it has formed.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TakenIn {
    /// The day of the game.
    pub day: u32,
    pub plan_orders: Vec<PlanOrder>,
    /// The side's landings in force, "ready a landing" intents, in the order
    /// of their numbers.
    pub landings: Vec<Landing>,
}

/// A landing a side has formed (decision 0177, amended 7 and 8): its intent,
/// the root of its tree, the force, the place, and the land units its forces
/// limit names, each with its men ashore and aboard at the day's end.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Landing {
    pub intent: IntentId,
    pub root: IntentId,
    pub force: TaskForceId,
    pub hex: Hex,
    pub units: Vec<(LandUnitId, u32)>,
}

impl Landing {
    /// The men of its units.
    pub fn men(&self) -> u32 {
        self.units.iter().map(|(_, men)| men).sum()
    }
}

/// A trial played out (decision 0175, 7): its worlds in the order of their
/// numbers, and what the record of the trial says of itself.
#[derive(Clone, Debug)]
pub struct Played {
    /// The side tried; `None` where the trial was given no picture.
    pub side: Option<Side>,
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
    /// The day of the world the course's first command is taken in.
    taken_in: Option<u32>,
    staffs: BTreeMap<Side, Box<dyn Staff + Send>>,
    in_play: Option<InPlay>,
    played: Vec<PlayedWorld>,
    side: Option<Side>,
    uneven: bool,
    leaves_given: Vec<Leave>,
    /// A day made to fail by a test of the crate: the world and the day of
    /// the world, from 1.
    #[cfg(test)]
    pub(crate) fail_on: Option<(u32, u32)>,
}

/// A world in play: its record and the side tried.
#[derive(Clone)]
pub(crate) struct InPlay {
    pub(crate) record: PlayedWorld,
    pub(crate) side: Side,
}

/// The world of a picture, built and not yet played; or, where its picture
/// is refused, the record of a world that could not be played (day 0).
pub(crate) fn start(
    picture: TrialPicture,
    ref_data: &RefData,
    map: &Map,
) -> Result<InPlay, Box<PlayedWorld>> {
    match build_world(&picture, ref_data.clone(), map.clone()) {
        Ok(built) => Ok(InPlay {
            record: PlayedWorld {
                number: picture.world,
                assumptions: picture.assumptions,
                days: Vec::new(),
                refused: Vec::new(),
                failed: None,
                plan_orders: None,
                taken_in: None,
                world: Some(built),
            },
            side: picture.side,
        }),
        Err(error) => Err(Box::new(PlayedWorld {
            number: picture.world,
            assumptions: picture.assumptions,
            days: Vec::new(),
            refused: Vec::new(),
            failed: Some(Failed {
                world: picture.world,
                day: 0,
                words: error.to_string(),
            }),
            plan_orders: None,
            taken_in: None,
            world: None,
        })),
    }
}

/// What a trial is to play, read from what it was asked (decision 0175):
/// the course, the day its first command is taken in, the horizon, and the
/// leaves the course gives that the side had not. Refused where the horizon
/// has no day, or a command of the course has a day outside it.
pub(crate) struct Setup {
    pub(crate) course: Vec<CourseCommand>,
    pub(crate) taken_in: Option<u32>,
    pub(crate) days: u32,
    pub(crate) leaves_given: Vec<Leave>,
}

pub(crate) fn setup(made: &[TrialPicture], asked: &Asked) -> Result<Setup, Refused> {
    setup_with_standing(made, asked, 0)
}

/// [`setup`] where the first `standing` commands of the course are standing
/// commands (decision 0177, amended 2): what the side had handed in for the
/// morning already. A leave among them is the commander's own and is not
/// counted as given by the course (decision 0178, rule 9); a leave the
/// course gives after them is counted only where the side, with them taken
/// in, does not hold it.
pub(crate) fn setup_with_standing(
    made: &[TrialPicture],
    asked: &Asked,
    standing: usize,
) -> Result<Setup, Refused> {
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
    let mut held = made
        .first()
        .and_then(|first| first.picture.command.get(&first.side))
        .map(|state| state.leaves)
        .unwrap_or_default();
    let mut leaves_given = Vec::new();
    for (index, c) in asked.course.iter().enumerate() {
        if let CommandKind::SetLeave { leave, given } = c.command {
            let had = match leave {
                Leave::Prepare => &mut held.prepare,
                Leave::Launch => &mut held.launch,
            };
            // A standing command's leave is the commander's own: it is taken
            // in as his, and the course gives only what he does not hold
            // after it (decision 0178, rule 9).
            if index >= standing && given && !*had && !leaves_given.contains(&leave) {
                leaves_given.push(leave);
            }
            if index < standing {
                *had = given;
            }
        }
    }
    Ok(Setup {
        course: asked.course.clone(),
        taken_in: asked.course.iter().map(|c| c.day).min(),
        days: asked.days,
        leaves_given,
    })
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
    let set = setup(&made, &asked)?;
    let side = made.first().map(|first| first.side);
    Ok(Playing {
        side,
        to_begin: made.into_iter(),
        ref_data: ref_data.clone(),
        map: map.clone(),
        days: set.days,
        course: set.course,
        taken_in: set.taken_in,
        staffs: sim_staff::staffs(),
        in_play: None,
        played: Vec::new(),
        uneven: !asked.worlds.is_power_of_two(),
        leaves_given: set.leaves_given,
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
            match start(picture, &self.ref_data, &self.map) {
                Ok(begun) => self.in_play = Some(begun),
                Err(refused) => {
                    let failed = refused
                        .failed
                        .clone()
                        .expect("a world refused has its fault");
                    self.played.push(*refused);
                    return Some(Stepped::Failed(failed));
                }
            }
        }
        let mut current = self.in_play.take()?;
        #[cfg(test)]
        let fail = self.fail_on
            == Some((
                current.record.number,
                u32::try_from(current.record.days.len()).unwrap_or(u32::MAX) + 1,
            ));
        #[cfg(not(test))]
        let fail = false;
        let stepped = play_day(
            &self.staffs,
            &self.course,
            self.taken_in,
            &mut current,
            fail,
        );
        let finished = current.record.failed.is_some()
            || u32::try_from(current.record.days.len()).ok() >= Some(self.days);
        if finished {
            self.played.push(current.record);
        } else {
            self.in_play = Some(current);
        }
        Some(stepped)
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
            side: self.side,
            worlds: self.played,
            uneven: self.uneven,
            leaves_given: self.leaves_given,
        }
    }
}

/// Hands in the course's commands of the world's next day and resolves it
/// with the staffs (decision 0175), and records the day: the commands of the
/// side tried the engine refused; at its end where bases stand; on the first
/// day the side's plan orders; and on the day the course is taken in
/// (`taken_in`, a day of the world) the side's plan orders and landings.
/// `fail` makes the day fail, for a test of the crate.
pub(crate) fn play_day(
    staffs: &BTreeMap<Side, Box<dyn Staff + Send>>,
    course: &[CourseCommand],
    taken_in: Option<u32>,
    current: &mut InPlay,
    fail: bool,
) -> Stepped {
    let number = current.record.number;
    let of_world = u32::try_from(current.record.days.len()).unwrap_or(u32::MAX) + 1;
    let Some(world) = current.record.world.as_mut() else {
        // A world with no world is recorded as refused when it was begun,
        // and never kept in play.
        return Stepped::Failed(Failed {
            world: number,
            day: 0,
            words: "the world was not built".to_string(),
        });
    };
    let day = world.day() + 1;
    let commands: Vec<CommandKind> = course
        .iter()
        .filter(|c| c.day == of_world)
        .map(|c| c.command.clone())
        .collect();
    if !commands.is_empty() {
        world.submit_commands(current.side, commands);
    }
    let resolved = if fail {
        Err(sim_core::world::SimError::InvalidSetup(
            "a day made to fail by a test of the crate".to_string(),
        ))
    } else {
        world.resolve_day_staffed(sim_staff::as_staffs(staffs))
    };
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
            // What the estimate and the window read of the day's end
            // (decision 0176, amended 1; decision 0177, amended 7), from the
            // side tried's view of the world, and nothing else of it: where
            // bases stand; on the first day, the side's plan orders as its
            // staff works them; and on the day the course is taken in, its
            // plan orders and the landings it has formed.
            let view = world.side_view(current.side);
            let plan_orders_now = || {
                staffs
                    .get(&current.side)
                    .map(|staff| staff.plan_orders(view))
            };
            if current.record.days.is_empty() {
                current.record.plan_orders = plan_orders_now();
            }
            if taken_in == Some(of_world) {
                current.record.taken_in = Some(TakenIn {
                    day: summary.day,
                    plan_orders: plan_orders_now().unwrap_or_default(),
                    landings: landings(&view),
                });
            }
            current.record.days.push(PlayedDay {
                day: summary.day,
                hash: summary.state_hash,
                staff: world.staff_handed_in().clone(),
                bases: BasesAt::of(&view),
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

/// The side's landings in force (decision 0177, amended 7): each "ready a
/// landing" intent, with the root of its tree and the units its forces limit
/// names, each with its men ashore and aboard, read from the side's own
/// command state through its view.
fn landings(view: &SideView<'_>) -> Vec<Landing> {
    let state = view.command_state();
    state
        .intents_in_force()
        .filter_map(|intent| {
            let IntentKind::ReadyLanding { force, hex, .. } = intent.kind else {
                return None;
            };
            let units = intent
                .limits
                .forces
                .as_ref()
                .map(|limit| {
                    limit
                        .land_units
                        .iter()
                        .map(|unit| {
                            let ashore = view
                                .land_units()
                                .find(|u| u.id == *unit)
                                .map_or(0, |u| u.men_ashore);
                            (*unit, ashore + view.men_aboard(*unit).unwrap_or(0))
                        })
                        .collect()
                })
                .unwrap_or_default();
            Some(Landing {
                intent: intent.id,
                root: state.root_of(intent.id),
                force,
                hex,
                units,
            })
        })
        .collect()
}

#[cfg(test)]
#[path = "play_tests.rs"]
mod tests;
