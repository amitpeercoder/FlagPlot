//! The window (decision 0177, as amended on review): the same course tried
//! at two or three dates, in the same worlds, and the estimates set side by
//! side.
//!
//! - [`dates`] chooses the dates from a side's view and the course's own
//!   commands, with no world built (amended 3): the first day of the trial,
//!   and the earliest two days after it, no later than the last day of the
//!   first date's play, on which something the course may use first becomes
//!   usable. What the course may use is what a take counts
//!   ([`sim_staff::counted_by_a_take`]). [`given_dates`] takes the dates
//!   from whoever asks in their place (amended 4).
//! - [`begin_window`] and [`play_window`] play a window: in each world, the
//!   first date's course from the picture's evening; then the days before
//!   each later date played once with nothing of the course in them, and
//!   each later date's course played on from the world as it stood on its
//!   eve (amended 5). A date is the course with each command handed in as
//!   many days later as the date is after the first (amended 2), played for
//!   the days of the size and as many more. Standing commands, what the side
//!   has handed in for the first morning already, are handed in on the first
//!   day at every date and are not shifted.
//! - [`window`] reads a window from its play (amended 11): for each date its
//!   day, the day its course was taken in, what came due by it, the men its
//!   take chose, its day expected ashore, what the enemy did with the days
//!   (amended 9), and its estimate (decision 0176); and for each later date
//!   against the first, world for world ([`world_for_world`], amended 10).
//!
//! A window says nothing of which date to choose: it gives no chance and no
//! advice. It reads what a trial reads, and the enemy side of a world it
//! built on the eve of each date, once, for the men ashore at the places of
//! the intent's end and their cover: the trial's own reckoning of an enemy
//! it made.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use sim_core::command::{CommandKind, IntentKind, Limit, Staff};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{EntityRef, IntentId, LandUnitId};
use sim_core::map::Map;
use sim_core::refdata::{RefData, TrialFigures};
use sim_core::state::Side;
use sim_core::view::SideView;

use crate::enemy::other;
use crate::estimate::{
    end_of, end_of_kind, estimate, intent_words, Estimate, Spread, WorldFigures,
};
use crate::play::{play_day, setup, start, Asked, CourseCommand, Failed, InPlay, Played};
use crate::play::{PlayedWorld, Setup};
use crate::{pictures, Believed, Refused, Trial, TrialPicture};

/// The most dates a window has (decision 0177, 4).
pub const MOST_DATES: usize = 3;

/// What a thing that came due is (amended 3).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DueKind {
    /// A task force of the side, the day it may first be used.
    TaskForce,
    /// A land unit of the side, the day it may first be used.
    LandUnit,
    /// A task force away at a yard, the day it is back.
    BackFromYard,
}

impl DueKind {
    fn words(self) -> &'static str {
        match self {
            DueKind::TaskForce => "a task force, may first be used",
            DueKind::LandUnit => "a land unit, may first be used",
            DueKind::BackFromYard => "a task force at the yard, is back",
        }
    }
}

/// One thing of the side's that the course may use and that becomes usable
/// on a day after the first (amended 3): what it is, its name and the day.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Due {
    pub unit: EntityRef,
    pub name: String,
    pub kind: DueKind,
    pub day: u32,
}

impl fmt::Display for Due {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}), {} on day {}",
            self.name,
            self.unit,
            self.kind.words(),
            self.day
        )
    }
}

/// A date of a window: its day, and what came due after the date before it
/// and by it (none at the first date).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Date {
    pub day: u32,
    pub due: Vec<Due>,
}

/// The dates of a window, told before any world is played (decision 0177,
/// 11).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dates {
    /// The first day of the trial, the morning after the view's day.
    pub first_day: u32,
    /// The last day of the first date's play: the bound of the dates chosen.
    pub last_day: u32,
    /// The dates, the first first: one to three.
    pub dates: Vec<Date>,
    /// Given by whoever asked, not chosen.
    pub given: bool,
    /// Days within the bound on which something came due and that are not
    /// tried, for the window has three dates already.
    pub not_tried: Vec<Date>,
}

/// The forces limit the course gives `intent`, the last one given (amended
/// 3): read from the course's own commands.
fn course_limit(
    course: &[CourseCommand],
    intent: IntentId,
) -> Option<&sim_core::command::ForcesLimit> {
    course.iter().rev().find_map(|c| match &c.command {
        CommandKind::SetLimit(Limit::Forces { intent: of, limit }) if *of == intent => Some(limit),
        _ => None,
    })
}

/// Everything of the side's that the course may use and that first becomes
/// usable after the view's next day, in day, kind and number order.
fn all_due(view: &SideView<'_>, course: &[CourseCommand], intent: IntentId) -> Vec<Due> {
    let first = view.day() + 1;
    let counted = sim_staff::counted_by_a_take(view, course_limit(course, intent));
    let mut due = Vec::new();
    for force in &counted.task_forces {
        let unit = EntityRef::TaskForce(force.id);
        if let Some(day) = view.available_from(unit).filter(|day| *day > first) {
            due.push(Due {
                unit,
                name: force.name.clone(),
                kind: DueKind::TaskForce,
                day,
            });
        }
        if let Some(trip) = force.yard_trip.filter(|trip| trip.return_day > first) {
            due.push(Due {
                unit,
                name: force.name.clone(),
                kind: DueKind::BackFromYard,
                day: trip.return_day,
            });
        }
    }
    for land in &counted.land_units {
        let unit = EntityRef::LandUnit(land.id);
        if let Some(day) = view.available_from(unit).filter(|day| *day > first) {
            due.push(Due {
                unit,
                name: land.name.clone(),
                kind: DueKind::LandUnit,
                day,
            });
        }
    }
    due.sort_by(|a, b| {
        (a.day, a.kind, a.unit.to_string()).cmp(&(b.day, b.kind, b.unit.to_string()))
    });
    due
}

/// The dates of a window (decision 0177, 3, 4 and amended 3), from the
/// side's view and the course's own commands for `intent`, for a size of
/// `days`: the first day of the trial; then the earliest two days, after it
/// and no later than the last day of the first date's play, on which
/// something the course may use first becomes usable, each naming all that
/// does. With none, one date. No world is built.
pub fn dates(view: &SideView<'_>, course: &[CourseCommand], intent: IntentId, days: u32) -> Dates {
    let first_day = view.day() + 1;
    let last_day = view.day() + days;
    let mut by_day: BTreeMap<u32, Vec<Due>> = BTreeMap::new();
    for due in all_due(view, course, intent) {
        if due.day <= last_day {
            by_day.entry(due.day).or_default().push(due);
        }
    }
    let mut dates = vec![Date {
        day: first_day,
        due: Vec::new(),
    }];
    let mut not_tried = Vec::new();
    for (day, due) in by_day {
        let date = Date { day, due };
        if dates.len() < MOST_DATES {
            dates.push(date);
        } else {
            not_tried.push(date);
        }
    }
    Dates {
        first_day,
        last_day,
        dates,
        given: false,
        not_tried,
    }
}

/// The dates given by whoever asks (amended 4), one to three, in order, none
/// before the first day of the trial; each with what came due after the date
/// before it and by it. Refused otherwise.
pub fn given_dates(
    view: &SideView<'_>,
    course: &[CourseCommand],
    intent: IntentId,
    days: u32,
    given: &[u32],
) -> Result<Dates, Refused> {
    let first_day = view.day() + 1;
    if given.is_empty() || given.len() > MOST_DATES {
        return Err(Refused(format!(
            "a window is given {} dates: it takes one to {MOST_DATES}",
            given.len()
        )));
    }
    if given.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Err(Refused(
            "the dates given are not each after the one before".to_string(),
        ));
    }
    if given[0] < first_day {
        return Err(Refused(format!(
            "a date given, day {}, is before the first day of the trial, day {first_day}",
            given[0]
        )));
    }
    let due = all_due(view, course, intent);
    let dates = given
        .iter()
        .enumerate()
        .map(|(i, day)| Date {
            day: *day,
            due: if i == 0 {
                Vec::new()
            } else {
                due.iter()
                    .filter(|d| d.day > given[i - 1] && d.day <= *day)
                    .cloned()
                    .collect()
            },
        })
        .collect();
    Ok(Dates {
        first_day,
        last_day: view.day() + days,
        dates,
        given: true,
        not_tried: Vec::new(),
    })
}

/// What a window is asked to play (decision 0177): a trial's number, its
/// count of worlds and its size in days, the intent tried, the standing
/// commands (what the side has handed in for the first morning already, put
/// at the head of a course and never shifted), the course, and the dates
/// where whoever asks gives them.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WindowAsked {
    pub trial: u32,
    pub worlds: u32,
    /// The days each date is played from the morning its course is handed
    /// in: the size.
    pub days: u32,
    pub intent: IntentId,
    pub standing: Vec<CommandKind>,
    pub course: Vec<CourseCommand>,
    /// The dates given, or `None` for the window to choose them.
    pub dates: Option<Vec<u32>>,
}

/// The men ashore of the enemy at a place, and the least cover of his units
/// there, in one world on a date's eve (amended 7 and 9).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnemyAt {
    pub hex: Hex,
    pub men: u32,
    /// The cover of his unit there with the least: the higher of its
    /// fieldworks and his base's defences there. `None` with no unit there.
    pub least_cover: Option<Fixed>,
}

/// One date as played: its day, its shift, the day its course was taken in,
/// the trial it is (each world's record from the first day of the trial),
/// and what each world held of the enemy on its eve.
#[derive(Clone, Debug)]
pub struct DatePlayed {
    pub date: Date,
    /// The date less the first day of the trial.
    pub shift: u32,
    /// The day of the game the course's first command is handed in; `None`
    /// for a course with no command.
    pub taken_in: Option<u32>,
    pub played: Played,
    /// Each world's reading on the eve, by its number; none where the world
    /// could not be played to the eve.
    pub eves: Vec<(u32, Vec<EnemyAt>)>,
}

/// A window played out: what was asked, its dates, the places of the
/// intent's end, and each date as played.
#[derive(Clone, Debug)]
pub struct PlayedWindow {
    pub side: Option<Side>,
    pub intent: IntentId,
    pub worlds: u32,
    pub days: u32,
    pub kind: Option<IntentKind>,
    pub places: Vec<Hex>,
    pub dates: Dates,
    pub played: Vec<DatePlayed>,
}

/// One date's own setting.
struct DateSet {
    date: Date,
    shift: u32,
    taken_in: Option<u32>,
    setup: Setup,
    worlds: Vec<PlayedWorld>,
    eves: Vec<(u32, Vec<EnemyAt>)>,
}

/// A window being played, a world at a time (decision 0177, 11). It holds
/// what it was given and the worlds it built, and nothing of the live game:
/// dropping it keeps nothing.
pub struct WindowPlaying {
    to_begin: std::vec::IntoIter<TrialPicture>,
    ref_data: RefData,
    map: Map,
    staffs: BTreeMap<Side, Box<dyn Staff + Send>>,
    /// The standing commands, on the first day: the days before a date.
    line: Vec<CourseCommand>,
    side: Option<Side>,
    intent: IntentId,
    worlds: u32,
    days: u32,
    kind: Option<IntentKind>,
    places: Vec<Hex>,
    dates: Dates,
    uneven: bool,
    sets: Vec<DateSet>,
}

/// The kind of `intent` as the standing commands or the course state it,
/// the last stated; `None` where neither does (an intent the side holds
/// already has its end read from the view).
fn intent_kind(asked: &WindowAsked) -> Option<IntentKind> {
    asked
        .standing
        .iter()
        .chain(asked.course.iter().map(|c| &c.command))
        .rev()
        .find_map(|command| match command {
            CommandKind::SetIntent { intent, kind } if *intent == asked.intent => Some(*kind),
            _ => None,
        })
}

/// Begins a window (decision 0177): makes the pictures of its worlds as a
/// trial does, takes or checks its dates, and plays nothing yet. Refused as a
/// trial is refused, and where the dates given are refused.
pub fn begin_window(
    view: &SideView<'_>,
    believed: &Believed,
    figures: &TrialFigures,
    ref_data: &RefData,
    map: &Map,
    asked: WindowAsked,
) -> Result<WindowPlaying, Refused> {
    let made = pictures(
        view,
        believed,
        figures,
        Trial {
            seed: figures.seed,
            trial: asked.trial,
            worlds: asked.worlds,
        },
    )?;
    let dates = match &asked.dates {
        Some(given) => given_dates(view, &asked.course, asked.intent, asked.days, given)?,
        None => dates(view, &asked.course, asked.intent, asked.days),
    };
    if let Some(outside) = asked
        .course
        .iter()
        .find(|c| c.day == 0 || c.day > asked.days)
    {
        return Err(Refused(format!(
            "a command of the course is for day {} of the world, outside the size of {} days",
            outside.day, asked.days
        )));
    }
    let kind = intent_kind(&asked);
    let places = match kind {
        Some(kind) => end_of_kind(view, kind),
        None => end_of(view, asked.intent),
    }
    .map(|end| end.places)
    .unwrap_or_default();
    let line: Vec<CourseCommand> = asked
        .standing
        .iter()
        .cloned()
        .map(CourseCommand::first_day)
        .collect();
    let first_command = asked.course.iter().map(|c| c.day).min();
    let mut sets = Vec::new();
    for date in &dates.dates {
        let shift = date.day - dates.first_day;
        let mut course = line.clone();
        course.extend(asked.course.iter().map(|c| CourseCommand {
            day: c.day + shift,
            command: c.command.clone(),
        }));
        let mut set = setup(
            &made,
            &Asked {
                trial: asked.trial,
                worlds: asked.worlds,
                days: asked.days + shift,
                course,
            },
        )?;
        // The day of the world the course proper is taken in: the
        // standing commands are not the course.
        let taken = first_command.map(|day| day + shift);
        set.taken_in = taken;
        sets.push(DateSet {
            date: date.clone(),
            shift,
            taken_in: taken,
            setup: set,
            worlds: Vec::new(),
            eves: Vec::new(),
        });
    }
    Ok(WindowPlaying {
        side: made.first().map(|first| first.side),
        to_begin: made.into_iter(),
        ref_data: ref_data.clone(),
        map: map.clone(),
        staffs: sim_staff::staffs(),
        line,
        intent: asked.intent,
        worlds: asked.worlds,
        days: asked.days,
        kind,
        places,
        dates,
        uneven: !asked.worlds.is_power_of_two(),
        sets,
    })
}

/// A window played out: [`begin_window`], then every world.
pub fn play_window(
    view: &SideView<'_>,
    believed: &Believed,
    figures: &TrialFigures,
    ref_data: &RefData,
    map: &Map,
    asked: WindowAsked,
) -> Result<PlayedWindow, Refused> {
    Ok(begin_window(view, believed, figures, ref_data, map, asked)?.play_out())
}

/// The enemy of the side tried at each place on the eve: read once from the
/// enemy's side of a world the window built, its men ashore there and the
/// least cover of his units there. The trial's own reckoning of an enemy it
/// made.
fn enemy_on_the_eve(current: &InPlay, places: &[Hex]) -> Vec<EnemyAt> {
    let Some(built) = current.record.world.as_ref() else {
        return Vec::new();
    };
    let enemy = built.side_view(other(current.side));
    places
        .iter()
        .map(|hex| {
            let there: Vec<_> = enemy
                .land_units()
                .filter(|unit| unit.hex == Some(*hex) && unit.men_ashore > 0)
                .collect();
            EnemyAt {
                hex: *hex,
                men: there.iter().map(|unit| unit.men_ashore).sum(),
                least_cover: there.iter().filter_map(|unit| enemy.cover(unit.id)).min(),
            }
        })
        .collect()
}

impl WindowPlaying {
    /// The dates, told before any world is played.
    pub fn dates(&self) -> &Dates {
        &self.dates
    }

    /// Plays the next world at every date: the first date's course from the
    /// picture's evening; the days before each later date once, with the
    /// standing commands alone; and each later date's course on from the
    /// world on its eve. The world's number, or `None` once every world is
    /// played.
    pub fn step(&mut self) -> Option<u32> {
        let picture = self.to_begin.next()?;
        let number = picture.world;
        let mut line = match start(picture, &self.ref_data, &self.map) {
            Ok(begun) => begun,
            Err(refused) => {
                for set in &mut self.sets {
                    set.worlds.push((*refused).clone());
                }
                return Some(number);
            }
        };
        for set in &mut self.sets {
            // The days before the date, played once, as things stand.
            while line.record.failed.is_none()
                && u32::try_from(line.record.days.len()).ok() < Some(set.shift)
            {
                play_day(&self.staffs, &self.line, None, &mut line, false);
            }
            if line.record.failed.is_some() {
                set.worlds.push(line.record.clone());
                continue;
            }
            set.eves
                .push((number, enemy_on_the_eve(&line, &self.places)));
            let mut branch = line.clone();
            while branch.record.failed.is_none()
                && u32::try_from(branch.record.days.len()).ok() < Some(set.setup.days)
            {
                play_day(
                    &self.staffs,
                    &set.setup.course,
                    set.taken_in,
                    &mut branch,
                    false,
                );
            }
            set.worlds.push(branch.record);
        }
        Some(number)
    }

    /// Plays every world left and gives back the window as played.
    pub fn play_out(mut self) -> PlayedWindow {
        while self.step().is_some() {}
        let view_day = self.dates.first_day - 1;
        let side = self.side;
        let uneven = self.uneven;
        PlayedWindow {
            side,
            intent: self.intent,
            worlds: self.worlds,
            days: self.days,
            kind: self.kind,
            places: self.places,
            dates: self.dates,
            played: self
                .sets
                .into_iter()
                .map(|set| DatePlayed {
                    date: set.date,
                    shift: set.shift,
                    taken_in: set.taken_in.map(|day| view_day + day),
                    played: Played {
                        side,
                        worlds: set.worlds,
                        uneven,
                        leaves_given: set.setup.leaves_given,
                    },
                    eves: set.eves,
                })
                .collect(),
        }
    }
}

// ---------------------------------------------------------------- The window

/// The lowest, the middle and the highest of some worlds' cover (as
/// [`Spread`]).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CoverSpread {
    pub lowest: Fixed,
    pub middle: Fixed,
    pub highest: Fixed,
}

impl CoverSpread {
    fn of(values: &[Fixed]) -> Option<CoverSpread> {
        let mut sorted = values.to_vec();
        sorted.sort_unstable();
        let n = sorted.len();
        if n == 0 {
            return None;
        }
        Some(CoverSpread {
            lowest: sorted[0],
            middle: sorted[n.div_ceil(2) - 1],
            highest: sorted[n - 1],
        })
    }
}

/// What the enemy did with the days, at one place on a date's eve (amended
/// 9): his men ashore there, and the least cover of his units there, each
/// over the worlds read on that eve. The trial's own reckoning.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnemyOnTheEve {
    pub hex: Hex,
    /// The day whose evening it is: the date less one.
    pub eve: u32,
    pub men: Option<Spread>,
    /// Over the worlds with a unit of his there.
    pub least_cover: Option<CoverSpread>,
}

/// The men a date's take chose (amended 8): the units of the landings formed
/// under the intent by the end of the day its course was taken in, in the
/// world of its plan order, with their men; and the worlds whose landings
/// name other units.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Chosen {
    pub world: u32,
    pub units: Vec<(LandUnitId, u32)>,
    pub men: u32,
    pub differs_in: Vec<u32>,
}

/// One date of a window (amended 11).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WindowDate {
    pub day: u32,
    /// The date less the first date.
    pub after_first: u32,
    /// The day its course was taken in.
    pub taken_in: Option<u32>,
    /// What came due after the date before it and by it, by name and kind.
    pub due: Vec<Due>,
    /// What its take chose; `None` where no landing was formed by then.
    pub chosen: Option<Chosen>,
    /// The day expected ashore, where its plan order gives one.
    pub expected_ashore: Option<u32>,
    pub enemy: Vec<EnemyOnTheEve>,
    pub estimate: Estimate,
}

/// The worlds of two estimates of the same worlds compared one for one
/// (amended 10): those in which the end was met and kept at both, at the
/// first alone, at the later alone, and at neither; and those that could not
/// be played at either, counted apart.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct WorldForWorld {
    pub both: Vec<u32>,
    pub first_alone: Vec<u32>,
    pub later_alone: Vec<u32>,
    pub neither: Vec<u32>,
    pub apart: Vec<u32>,
}

/// World for world (amended 10): one function of two estimates' records of
/// their worlds, for any two courses in the same worlds. Met and lost again
/// counts as not met and kept.
pub fn world_for_world(first: &Estimate, later: &Estimate) -> WorldForWorld {
    compare(
        &first.worlds,
        &first
            .could_not
            .iter()
            .map(|f| f.world)
            .collect::<Vec<u32>>(),
        &later.worlds,
        &later
            .could_not
            .iter()
            .map(|f| f.world)
            .collect::<Vec<u32>>(),
    )
}

/// [`world_for_world`] on the worlds' own figures and the numbers of the
/// worlds that could not be played.
pub(crate) fn compare(
    first: &[WorldFigures],
    first_not: &[u32],
    later: &[WorldFigures],
    later_not: &[u32],
) -> WorldForWorld {
    let kept = |w: &WorldFigures| w.met_day.is_some() && !w.lost_again;
    let numbers: BTreeSet<u32> = first
        .iter()
        .chain(later)
        .map(|w| w.number)
        .chain(first_not.iter().copied())
        .chain(later_not.iter().copied())
        .collect();
    let mut out = WorldForWorld::default();
    for number in numbers {
        let a = first.iter().find(|w| w.number == number);
        let b = later.iter().find(|w| w.number == number);
        match (a, b) {
            (Some(a), Some(b)) => {
                let list = match (kept(a), kept(b)) {
                    (true, true) => &mut out.both,
                    (true, false) => &mut out.first_alone,
                    (false, true) => &mut out.later_alone,
                    (false, false) => &mut out.neither,
                };
                list.push(number);
            }
            _ => out.apart.push(number),
        }
    }
    out
}

/// A window read from its play (decision 0177, amended 11).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Window {
    pub side: Option<Side>,
    pub intent: IntentId,
    pub kind: Option<IntentKind>,
    pub worlds: u32,
    pub days: u32,
    pub given: bool,
    pub first_day: u32,
    pub last_day: u32,
    pub dates: Vec<WindowDate>,
    pub not_tried: Vec<Date>,
    /// Each later date against the first, world for world, in the order of
    /// the dates.
    pub against_first: Vec<(u32, WorldForWorld)>,
}

/// The landings under `intent` in a world's record at the end of the day its
/// course was taken in: their units with their men.
fn chosen_in(w: &PlayedWorld, intent: IntentId) -> Option<Vec<(LandUnitId, u32)>> {
    let taken = w.taken_in.as_ref()?;
    let units: Vec<(LandUnitId, u32)> = taken
        .landings
        .iter()
        .filter(|landing| landing.root == intent)
        .flat_map(|landing| landing.units.iter().copied())
        .collect();
    (!units.is_empty()).then_some(units)
}

/// The window, read from its play (decision 0177): each date's own figures
/// and estimate, and each later date against the first.
pub fn window(played: &PlayedWindow) -> Window {
    let intent = played.intent;
    let first_date = played.dates.dates.first().map_or(0, |d| d.day);
    let dates: Vec<WindowDate> = played
        .played
        .iter()
        .map(|date| {
            let estimate = estimate(&date.played, intent, None);
            let reference = date
                .played
                .worlds
                .iter()
                .find_map(|w| chosen_in(w, intent).map(|units| (w.number, units)));
            let chosen = reference.map(|(world, units)| {
                let names: Vec<LandUnitId> = units.iter().map(|(u, _)| *u).collect();
                Chosen {
                    world,
                    men: units.iter().map(|(_, men)| men).sum(),
                    units,
                    differs_in: date
                        .played
                        .worlds
                        .iter()
                        .filter(|w| w.taken_in.is_some())
                        .filter(|w| {
                            chosen_in(w, intent)
                                .map(|units| units.iter().map(|(u, _)| *u).collect::<Vec<_>>())
                                .as_ref()
                                != Some(&names)
                        })
                        .map(|w| w.number)
                        .collect(),
                }
            });
            let enemy = played
                .places
                .iter()
                .map(|hex| {
                    let at: Vec<&EnemyAt> = date
                        .eves
                        .iter()
                        .filter_map(|(_, reading)| reading.iter().find(|e| e.hex == *hex))
                        .collect();
                    EnemyOnTheEve {
                        hex: *hex,
                        eve: date.date.day - 1,
                        men: Spread::of(&at.iter().map(|e| e.men).collect::<Vec<u32>>()),
                        least_cover: CoverSpread::of(
                            &at.iter()
                                .filter_map(|e| e.least_cover)
                                .collect::<Vec<Fixed>>(),
                        ),
                    }
                })
                .collect();
            WindowDate {
                day: date.date.day,
                after_first: date.date.day - first_date,
                taken_in: date.taken_in,
                due: date.date.due.clone(),
                chosen,
                expected_ashore: estimate
                    .plan_order
                    .as_ref()
                    .and_then(|order| order.expected_ashore),
                enemy,
                estimate,
            }
        })
        .collect();
    let against_first = match dates.split_first() {
        Some((first, later)) => later
            .iter()
            .map(|date| (date.day, world_for_world(&first.estimate, &date.estimate)))
            .collect(),
        None => Vec::new(),
    };
    Window {
        side: played.side,
        intent,
        kind: played.kind,
        worlds: played.worlds,
        days: played.days,
        given: played.dates.given,
        first_day: played.dates.first_day,
        last_day: played.dates.last_day,
        dates,
        not_tried: played.dates.not_tried.clone(),
        against_first,
    }
}

// ---------------------------------------------------------------- Words

/// "1", "1 and 3", "1, 2 and 4".
fn list(numbers: &[u32]) -> String {
    match numbers {
        [] => String::new(),
        [one] => one.to_string(),
        [rest @ .., last] => format!(
            "{} and {last}",
            rest.iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

/// "in 2 worlds (worlds 1 and 3)", "in none".
fn in_worlds(worlds: &[u32]) -> String {
    match worlds.len() {
        0 => "in none".to_string(),
        1 => format!("in 1 world (world {})", worlds[0]),
        n => format!("in {n} worlds (worlds {})", list(worlds)),
    }
}

fn dues_words(due: &[Due]) -> String {
    due.iter()
        .map(Due::to_string)
        .collect::<Vec<_>>()
        .join("; ")
}

impl fmt::Display for Dates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let how = if self.given {
            "given by whoever asked"
        } else {
            "taken from what of the side's the course may use comes due"
        };
        writeln!(
            f,
            "The dates, {how}: {}.",
            self.dates
                .iter()
                .map(|d| format!("day {}", d.day))
                .collect::<Vec<_>>()
                .join(", ")
        )?;
        for (i, date) in self.dates.iter().enumerate() {
            if i == 0 {
                writeln!(f, "- Day {}: the course as it would go now.", date.day)?;
            } else if date.due.is_empty() {
                writeln!(
                    f,
                    "- Day {}: nothing of the side's that the course may use comes due by it.",
                    date.day
                )?;
            } else {
                writeln!(f, "- Day {}: {}.", date.day, dues_words(&date.due))?;
            }
        }
        if !self.given && self.dates.len() == 1 {
            writeln!(
                f,
                "Nothing of the side's that the course may use is due to change by day {}: the \
                 window has one date.",
                self.last_day
            )?;
        }
        for date in &self.not_tried {
            writeln!(
                f,
                "Not tried, for a window has {MOST_DATES} dates at most: day {}, {}.",
                date.day,
                dues_words(&date.due)
            )?;
        }
        Ok(())
    }
}

fn spread_words(spread: Option<Spread>) -> String {
    spread.map_or_else(
        || "none read".to_string(),
        |s| format!("{}, {}, {}", s.lowest, s.middle, s.highest),
    )
}

fn cover_words(spread: Option<CoverSpread>) -> String {
    spread.map_or_else(
        || "none, for no unit of his is there".to_string(),
        |s| format!("{}, {}, {}", s.lowest, s.middle, s.highest),
    )
}

fn compare_words(day: u32, first: u32, w: &WorldForWorld) -> String {
    let mut out = format!(
        "Day {day} against day {first}, world for world: met and kept at both {}; at the first \
         alone {}; at the later alone {}; at neither {}",
        in_worlds(&w.both),
        in_worlds(&w.first_alone),
        in_worlds(&w.later_alone),
        in_worlds(&w.neither)
    );
    if !w.apart.is_empty() {
        out.push_str(&format!(
            "; could not be played at one of the two, counted apart, {}",
            in_worlds(&w.apart)
        ));
    }
    out
}

impl fmt::Display for Window {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let side = self.side.map_or("the", Side::name);
        let what = self
            .kind
            .or_else(|| self.dates.first().and_then(|d| d.estimate.kind))
            .map_or_else(
                || "an intent the side does not hold".to_string(),
                intent_words,
            );
        writeln!(
            f,
            "The window of intent {} of the {side} side, {what}: the same course at {} {} in \
             the same {} worlds, each played for {} days from the morning its course is handed \
             in. The dates were {}.",
            self.intent.raw(),
            self.dates.len(),
            if self.dates.len() == 1 {
                "date"
            } else {
                "dates"
            },
            self.worlds,
            self.days,
            if self.given {
                "given by whoever asked"
            } else {
                "taken from what of the side's the course may use comes due"
            }
        )?;
        if !self.given && self.dates.len() == 1 {
            writeln!(
                f,
                "Nothing of the side's that the course may use is due to change by day {}: the \
                 window has one date.",
                self.last_day
            )?;
        }
        let first = self.dates.first().map_or(0, |d| d.day);
        for (i, date) in self.dates.iter().enumerate() {
            writeln!(f)?;
            let played_to = date
                .estimate
                .last_day
                .map_or_else(|| "no day".to_string(), |day| format!("day {day}"));
            let taken = date.taken_in.map_or_else(
                || "with no command".to_string(),
                |day| format!("on day {day}"),
            );
            if i == 0 {
                writeln!(
                    f,
                    "The first date, day {}: the course handed in {taken}, played to {played_to}.",
                    date.day
                )?;
            } else {
                writeln!(
                    f,
                    "A later date, day {}, {} days after the first: the course handed in {taken}, \
                     played to {played_to}. Nothing of the course was done before day {}: the \
                     units it would take lay idle in the days between, and a landing decided on \
                     day {} is ashore later than one prepared from day {first} and held.",
                    date.day, date.after_first, date.day, date.day
                )?;
                // What it buys: everything that came due since the first.
                let since: Vec<Due> = self.dates[1..=i]
                    .iter()
                    .flat_map(|d| d.due.iter().cloned())
                    .collect();
                if since.is_empty() {
                    writeln!(
                        f,
                        "What came due since the first date: nothing of the side's that the \
                         course may use."
                    )?;
                } else {
                    writeln!(
                        f,
                        "What came due since the first date: {}.",
                        dues_words(&since)
                    )?;
                }
            }
            match &date.chosen {
                Some(chosen) => {
                    let units: Vec<String> = chosen
                        .units
                        .iter()
                        .map(|(unit, men)| format!("{unit}, {men} men"))
                        .collect();
                    write!(
                        f,
                        "Its take chose {} men, in world {}: {}.",
                        chosen.men,
                        chosen.world,
                        units.join("; ")
                    )?;
                    if chosen.differs_in.is_empty() {
                        writeln!(f, " The same in every world.")?;
                    } else {
                        writeln!(
                            f,
                            " Other units in {} worlds (worlds {}).",
                            chosen.differs_in.len(),
                            list(&chosen.differs_in)
                        )?;
                    }
                }
                None => writeln!(
                    f,
                    "Its take had formed no landing by the end of the day its course was taken in."
                )?,
            }
            match date.estimate.plan_order.as_ref().and_then(|o| o.men) {
                Some(men) => writeln!(
                    f,
                    "Its plan order: men wanted {}, to hand {}.",
                    men.wanted, men.to_hand
                )?,
                None => writeln!(
                    f,
                    "Its plan order gives no men wanted or to hand: the side holds no count of \
                     the enemy there."
                )?,
            }
            match date.expected_ashore {
                Some(day) => writeln!(f, "The day expected ashore: day {day}.")?,
                None => writeln!(f, "Its plan order gives no day expected ashore.")?,
            }
            for enemy in &date.enemy {
                writeln!(
                    f,
                    "The enemy at {} on {}, the trial's own reckoning of an enemy it made, \
                     lowest, middle and highest: men ashore {}; the least cover {}.",
                    enemy.hex,
                    if enemy.eve + 1 == self.first_day {
                        format!("the evening before day {}", self.first_day)
                    } else {
                        format!("the evening of day {}", enemy.eve)
                    },
                    spread_words(enemy.men),
                    cover_words(enemy.least_cover)
                )?;
            }
            write!(f, "{}", date.estimate)?;
            if let Some((_, against)) = self.against_first.iter().find(|(day, _)| *day == date.day)
            {
                writeln!(f, "{}.", compare_words(date.day, first, against))?;
            }
        }
        for date in &self.not_tried {
            writeln!(
                f,
                "Not tried, for a window has {MOST_DATES} dates at most: day {}, {}.",
                date.day,
                dues_words(&date.due)
            )?;
        }
        Ok(())
    }
}

/// The failures of a date's worlds, for whoever reads a played window.
impl DatePlayed {
    pub fn failed(&self) -> Vec<&Failed> {
        self.played.failed()
    }
}

#[cfg(test)]
#[path = "window_tests.rs"]
mod tests;
