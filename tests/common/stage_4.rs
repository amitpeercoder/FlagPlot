//! What stage 4's proof's cases share (decision 0161): a game played with
//! both staffs and read day by day, a trial stepped day by day, and the
//! course of a scenario's commands of one side as a trial takes it.
//!
//! The umpire's reading: a test reads the truth, as the scenario tests do.

use std::collections::BTreeMap;
use std::path::Path;

use sim_core::command::{CommandKind, StaffDecisions};
use sim_core::events::Event;
use sim_core::hex::Hex;
use sim_core::ids::LandUnitId;
use sim_core::refdata::TrialFigures;
use sim_core::state::{Side, TruthState};
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};
use sim_trial::{begin, Asked, Believed, Costs, CourseCommand, Losses, Played, Stepped};

use super::{data_dir, from_text};

/// The scenario file of a case of the proof, by its name.
pub fn case_path(name: &str) -> std::path::PathBuf {
    data_dir().join(format!("scenarios/stage-4-proof/{name}.toml"))
}

pub fn case_text(name: &str) -> String {
    std::fs::read_to_string(case_path(name)).expect("the case's file reads")
}

pub fn load_case(name: &str, text: &str) -> Scenario {
    from_text(text, &case_path(name)).expect("the case loads")
}

/// The commands of `side` of every day of `scenario` as a trial's course:
/// each on its own day.
pub fn course_of(scenario: &Scenario, side: Side) -> Vec<CourseCommand> {
    scenario
        .scripted_commands()
        .iter()
        .flat_map(|(day, commands)| {
            commands.iter().filter(move |(s, _)| *s == side).map(
                move |(_, command): &(Side, CommandKind)| CourseCommand {
                    day: *day,
                    command: command.clone(),
                },
            )
        })
        .collect()
}

pub fn believed(scenario: &Scenario, side: Side) -> Believed {
    Believed {
        forces: scenario.believed_forces(side).cloned().collect(),
        intents: scenario.believed_intents(side).cloned().collect(),
        men: scenario.believed_men(side).cloned().collect(),
    }
}

pub fn figures(scenario: &Scenario) -> TrialFigures {
    scenario
        .trial_figures()
        .cloned()
        .expect("the case names its trial's figures")
}

/// What one day of a world or a game left, read by the umpire.
pub struct Snapshot {
    pub world: u32,
    pub day: u32,
    pub truth: TruthState,
    /// The side's own firing men of each of its land units, by its view.
    pub fit: BTreeMap<u32, u32>,
    /// What the side lost by the end of the day.
    pub lost: Losses,
}

fn fit_of(view: &sim_core::view::SideView<'_>) -> BTreeMap<u32, u32> {
    view.land_units()
        .filter_map(|u| view.firing_men(u.id).map(|m| (u.id.raw(), m)))
        .collect()
}

/// A trial of `side` on `live` stepped to its end: its played worlds and
/// each world's each day.
pub struct Stepped4 {
    pub played: Played,
    pub days: Vec<Snapshot>,
}

impl Stepped4 {
    pub fn snapshot(&self, world: u32, day: u32) -> &Snapshot {
        self.days
            .iter()
            .find(|s| s.world == world && s.day == day)
            .expect("a day played")
    }
}

pub fn step_trial(
    live: &Scenario,
    side: Side,
    trial: u32,
    worlds: u32,
    days: u32,
    course: Vec<CourseCommand>,
) -> Stepped4 {
    let mut playing = begin(
        &live.world.side_view(side),
        &believed(live, side),
        &figures(live),
        live.world.ref_data(),
        live.world.map(),
        Asked {
            trial,
            worlds,
            days,
            course,
        },
    )
    .expect("the trial begins");
    let mut out = Vec::new();
    while let Some(stepped) = playing.step() {
        let Stepped::Day { world, day, .. } = stepped else {
            panic!("a world could not be played: {stepped:?}");
        };
        let current = playing
            .in_play()
            .filter(|w| w.number == world)
            .or_else(|| playing.played().last())
            .expect("the world just played");
        let built = current.world.as_ref().expect("built");
        let view = built.side_view(side);
        out.push(Snapshot {
            world,
            day,
            truth: built.umpire().clone(),
            fit: fit_of(&view),
            lost: Losses::of(&view),
        });
    }
    Stepped4 {
        played: playing.play_out(),
        days: out,
    }
}

/// A game of the scenario played to its end with both staffs and its own
/// commands: each day's truth, what each staff handed in, and the log.
pub struct Game {
    pub days: Vec<Snapshot>,
    pub handed: BTreeMap<u32, BTreeMap<Side, StaffDecisions>>,
    pub events: Vec<Event>,
}

impl Game {
    pub fn day(&self, day: u32) -> &Snapshot {
        &self.days[day as usize - 1]
    }
}

pub fn play_game(mut scenario: Scenario, side: Side) -> Game {
    let held = staffs();
    let mut days = Vec::new();
    let mut handed = BTreeMap::new();
    loop {
        let day = scenario.world.day() + 1;
        let summary = scenario
            .next_day_staffed(as_staffs(&held))
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
        handed.insert(day, scenario.world.staff_handed_in().clone());
        let view = scenario.world.side_view(side);
        days.push(Snapshot {
            world: 0,
            day,
            truth: scenario.world.umpire().clone(),
            fit: fit_of(&view),
            lost: Losses::of(&view),
        });
    }
    Game {
        days,
        handed,
        events: scenario.world.events().iter().cloned().collect(),
    }
}

pub fn costs_men(costs: &Costs) -> (u32, u32, u32) {
    (
        costs.men_to_enemy.lowest,
        costs.men_to_enemy.middle,
        costs.men_to_enemy.highest,
    )
}

pub fn men_of(truth: &TruthState, unit: u32) -> u32 {
    truth
        .land_units
        .get(&LandUnitId::new(unit))
        .map_or(0, |u| u.men_ashore)
}

pub use super::values::hex;

pub fn path_exists(p: &Path) -> bool {
    p.exists()
}

/// An order of the staff, plainly. Any order a case does not name is
/// `Other`, with its debug text.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Given {
    Landing(Hex, Vec<u32>),
    LoadTo(sim_core::fixed::Fixed),
    Attack(u32),
    Defend(u32),
    LoadUnit(u32),
    Route(Vec<Hex>, Vec<Hex>),
    Unload,
    Other(String),
}

pub fn given(o: &sim_core::command::StaffOrder) -> Given {
    use sim_core::orders::{Cargo, LandUnitOrder, OrderKind, Posture, TaskForceOrder};
    match &o.kind {
        OrderKind::TaskForce { order, .. } => match order {
            TaskForceOrder::PrepareLanding { hex, roster } => {
                Given::Landing(*hex, roster.iter().map(|u| u.raw()).collect())
            }
            TaskForceOrder::LoadTo {
                cargo: Cargo::Supply,
                weight_t,
                ..
            } => Given::LoadTo(*weight_t),
            TaskForceOrder::LoadUnit { unit, cargo: None } => Given::LoadUnit(unit.raw()),
            TaskForceOrder::SetRoute {
                waypoints,
                act_at,
                stand_at,
            } if stand_at.is_empty() => {
                Given::Route(waypoints.clone(), act_at.iter().copied().collect())
            }
            TaskForceOrder::Unload => Given::Unload,
            other => Given::Other(format!("{other:?}")),
        },
        OrderKind::LandUnit {
            target,
            order:
                LandUnitOrder::SetPosture {
                    posture,
                    when_spent: None,
                },
        } => match posture {
            Posture::Attack => Given::Attack(target.raw()),
            Posture::Defend => Given::Defend(target.raw()),
        },
        other => Given::Other(format!("{other:?}")),
    }
}

/// The orders `side`'s staff handed in on `day` of a game, plainly.
pub fn orders_on(game: &Game, day: u32, side: Side) -> Vec<Given> {
    game.handed
        .get(&day)
        .and_then(|d| d.get(&side))
        .map(|s| s.orders.iter().map(given).collect())
        .unwrap_or_default()
}

/// The commands `side`'s staff took on `day` of a game, as debug text.
pub fn commands_on(game: &Game, day: u32, side: Side) -> Vec<String> {
    game.handed
        .get(&day)
        .and_then(|d| d.get(&side))
        .map(|s| s.commands.iter().map(|c| format!("{:?}", c.kind)).collect())
        .unwrap_or_default()
}

/// The days on which `side`'s staff handed in a notice of `kind`.
pub fn notice_days(game: &Game, side: Side, kind: sim_core::command::NoticeKind) -> Vec<u32> {
    game.handed
        .iter()
        .filter(|(_, d)| {
            d.get(&side)
                .is_some_and(|s| s.notices.iter().any(|n| n.kind == kind))
        })
        .map(|(day, _)| *day)
        .collect()
}

/// The orders `side`'s staff handed in on `day` of a trial's world.
pub fn world_orders_on(world: &sim_trial::PlayedWorld, day: u32, side: Side) -> Vec<Given> {
    world
        .days
        .iter()
        .find(|d| d.day == day)
        .and_then(|d| d.staff.get(&side))
        .map(|s| s.orders.iter().map(given).collect())
        .unwrap_or_default()
}

/// The commands taken on `day` of a trial's world, as debug text.
pub fn world_commands_on(world: &sim_trial::PlayedWorld, day: u32, side: Side) -> Vec<String> {
    world
        .days
        .iter()
        .find(|d| d.day == day)
        .and_then(|d| d.staff.get(&side))
        .map(|s| s.commands.iter().map(|c| format!("{:?}", c.kind)).collect())
        .unwrap_or_default()
}
