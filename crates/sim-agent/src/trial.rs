//! A seat's trial (decision 0178): what a session asks a trial, the trait by
//! which one is begun and stepped, the answer made for a seat, and its words.
//!
//! This crate cannot build a world and must not learn to: a trial is played
//! by a runner its caller hands in, as it hands in the staffs (rules 1, 5
//! and 6). The runner is given a side's whole view and what is asked, and
//! fills [`TrialAnswer`], plain data that holds no more than a seat may be
//! shown (rule 4): counts and spreads of worlds, the side's own units, forces
//! and bases as the live game has them ([`LiveIds`] makes the only values
//! that can name one), places, and what was assumed of the enemy as a kind
//! with a place, men or tons and the belief it was made from. No field can
//! hold a unit made for the enemy, an intent or a plan formed inside a
//! trial's world, or a world's own figures beyond the counts.
//!
//! The session renders the answer for the seat (rules 15 to 17), in the
//! words of the seat's other tools and with no name of the engine's.

use std::collections::BTreeSet;

use serde_json::{json, Value};
use sim_core::belief::GroundGrade;
use sim_core::believed::Standing;
use sim_core::command::{CommandKind, Knowledge, Leave, Limit, PhaseName, Want};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, LandUnitId, ShipId, TaskForceId};
use sim_core::refdata::ShipKind;
use sim_core::view::SideView;

use crate::common::Ctx;

// ------------------------------------------------------------- What is asked

/// The size of a trial as asked: by its name in the trial's figures, or by
/// its two numbers (rule 2).
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SizeAsked {
    Named(String),
    Numbers { worlds: u32, days: u32 },
}

/// One command of a course and the day of the trial's world on whose
/// morning it is handed in, from 1 (decision 0175, amended 1).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CourseStep {
    pub day: u32,
    pub command: CommandKind,
}

/// The dates a trial is asked at (decision 0181, rule 9 and amended 8).
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DatesAsked {
    /// The coming morning alone: a trial, answered as one.
    Coming,
    /// A window whose later dates the runner chooses from what the course
    /// may use (decision 0177, amended 3).
    Chosen,
    /// A window of these days of the game, the coming morning first, then
    /// one or two later days, ascending.
    Given(Vec<u32>),
}

/// What the second trial is of (decision 0181, amended 1): the side without
/// the thing tried.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BesideOf {
    /// A draft of a limit, or of a change to an intent that stands: the plan
    /// as it stands, without the draft. The intent is in both trials.
    AsItStands,
    /// A new intent, previewed or queued: the side without it.
    WithoutIt,
    /// An intent in force: the side with it withdrawn on the coming morning.
    Withdrawn,
}

impl BesideOf {
    pub fn name(self) -> &'static str {
        match self {
            BesideOf::AsItStands => "as_it_stands",
            BesideOf::WithoutIt => "without_it",
            BesideOf::Withdrawn => "withdrawn",
        }
    }
}

/// The second trial (decision 0181, amended 1 and rule 4): what it is of, its
/// own standing commands and its own course, played in the same worlds as the
/// first and given the same leaves.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SecondAsked {
    pub of: BesideOf,
    pub standing: Vec<CommandKind>,
    pub course: Vec<CourseStep>,
}

/// What a trial is asked (rule 2): its number; its size; the standing
/// commands, what the side would take in on the coming morning without it
/// (rule 8), handed in apart from the course and never shifted; the course;
/// the dates to try (decision 0181, rule 9); the intent whose estimate is
/// wanted; and, where a second trial is wanted, its own commands and course
/// (decision 0181, amended 1).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TrialAsked {
    pub trial: u32,
    pub size: SizeAsked,
    pub standing: Vec<CommandKind>,
    pub course: Vec<CourseStep>,
    pub dates: DatesAsked,
    pub intent: IntentId,
    pub second: Option<SecondAsked>,
}

/// Why a trial is refused, in words a seat may be shown.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TrialRefused(pub String);

/// A trial played to its end: answered as a trial, or, where dates were
/// asked, as a window (decision 0181, rule 13).
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Answered {
    Trial(Box<TrialAnswer>),
    Window(WindowAnswer),
}

/// What one step of a trial under way did (rule 3).
pub enum TrialStep {
    /// Some of it was played; more is to come.
    Going,
    /// It is played to its end: the answer.
    Done(Box<Answered>),
}

/// A trial under way (rule 3): stepped until it is done. Dropped, it leaves
/// nothing.
pub trait TrialUnderWay: Send {
    fn step(&mut self) -> TrialStep;
}

/// What plays a seat's trials (rules 1, 3 and 5), handed to the session by
/// its caller. To begin takes a side's whole view, on an evening (inside a
/// day, the view of the state as it stands with the open day treated as
/// closed, decision 0204), and what is asked; it answers with a trial under way, or why it cannot be tried.
/// To describe takes the same and plays nothing: it says the size and the
/// dates, with what is due on each (decision 0181, amended 9).
pub trait TrialRunner: Send {
    fn begin(
        &self,
        view: &SideView<'_>,
        asked: &TrialAsked,
    ) -> Result<Box<dyn TrialUnderWay>, TrialRefused>;

    fn describe(&self, view: &SideView<'_>, asked: &TrialAsked) -> Result<Described, TrialRefused>;
}

/// What would be played, told before anything is (decision 0181, amended
/// 9): the size, and each date with what came due on it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Described {
    pub worlds: u32,
    pub days: u32,
    pub size: Option<String>,
    /// The first day of the trial and the last of the first date's play.
    pub first_day: u32,
    pub last_day: u32,
    /// The dates, the first first; one for a trial of the coming morning.
    pub dates: Vec<DueDateShown>,
    /// Whether the dates were given, not chosen.
    pub given: bool,
    /// Days on which something came due that are not tried.
    pub not_tried: Vec<DueDateShown>,
}

// ------------------------------------------------- Things of the side's own

/// The side's own task forces, ships, land units and bases, and the bases
/// on its map, as a live view gives them when a trial begins. The only maker
/// of the values that name a force, a ship, a unit or a base in an answer:
/// none can name a thing the live view does not have.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct LiveIds {
    forces: BTreeSet<TaskForceId>,
    ships: BTreeSet<ShipId>,
    land_units: BTreeSet<LandUnitId>,
    bases: BTreeSet<BaseId>,
}

impl LiveIds {
    /// What the view has of the side's own, and the bases on its map.
    pub fn of(view: &SideView<'_>) -> LiveIds {
        LiveIds {
            forces: view.task_forces().map(|f| f.id).collect(),
            ships: view.ships().map(|s| s.id).collect(),
            land_units: view.land_units().map(|u| u.id).collect(),
            bases: view
                .bases()
                .map(|b| b.id)
                .chain(view.enemy_bases().map(|b| b.id))
                .collect(),
        }
    }

    /// A task force of the side's that the live game has.
    pub fn force(&self, id: TaskForceId) -> Option<OwnForce> {
        self.forces.contains(&id).then_some(OwnForce(id))
    }

    /// A ship of the side's that the live game has.
    pub fn ship(&self, id: ShipId) -> Option<OwnShip> {
        self.ships.contains(&id).then_some(OwnShip(id))
    }

    /// A land unit of the side's that the live game has.
    pub fn land_unit(&self, id: LandUnitId) -> Option<OwnUnit> {
        self.land_units.contains(&id).then_some(OwnUnit(id))
    }

    /// A base on the side's map, its own or the enemy's.
    pub fn base(&self, id: BaseId) -> Option<MapBase> {
        self.bases.contains(&id).then_some(MapBase(id))
    }
}

/// A task force of the side's that the live game has.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct OwnForce(TaskForceId);
/// A ship of the side's that the live game has.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct OwnShip(ShipId);
/// A land unit of the side's that the live game has.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct OwnUnit(LandUnitId);
/// A base on the side's map, as the live game has it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MapBase(BaseId);

impl OwnForce {
    pub fn id(self) -> TaskForceId {
        self.0
    }
}
impl OwnShip {
    pub fn id(self) -> ShipId {
        self.0
    }
}
impl OwnUnit {
    pub fn id(self) -> LandUnitId {
        self.0
    }
}
impl MapBase {
    pub fn id(self) -> BaseId {
        self.0
    }
}

/// A task force a task names (rule 16): one the live game has, or one the
/// staff would form inside the trial, by its ships.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ForceShown {
    Live(OwnForce),
    Formed { ships: Vec<OwnShip> },
}

/// A base a task names (rule 16): one on the side's map in the live game,
/// or a landing place the staff would make inside the trial, by its place.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum BaseShown {
    Live(MapBase),
    Formed { place: Option<Hex> },
}

// ---------------------------------------------------------------- The answer

/// The lowest, the middle and the highest over some worlds (decision 0176,
/// 10).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TrialSpread {
    pub lowest: u32,
    pub middle: u32,
    pub highest: u32,
}

/// The five costs of a side, each a spread (decision 0176, 8).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TrialCosts {
    pub men_to_enemy: TrialSpread,
    pub men_unfed: TrialSpread,
    pub ships_sunk: TrialSpread,
    pub ships_out_of_action: TrialSpread,
    pub aircraft: TrialSpread,
}

/// How it went, by the worlds' numbers (decision 0176, 7).
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct TrialWent {
    pub played: Vec<u32>,
    pub met_kept: Vec<u32>,
    pub met_days: Option<TrialSpread>,
    pub lost_again: Vec<u32>,
    pub not_met: Vec<u32>,
}

/// One thing a plan waits for (decision 0159, 15).
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum WaitShown {
    Men {
        wanted: u32,
        to_hand: u32,
    },
    Leave(Leave),
    Day(u32),
    Force(ForceShown),
    LandUnit(OwnUnit),
    /// Another intent of the commander's, by his number.
    Intent(IntentId),
    Knowledge(Knowledge),
}

/// The count asked of the enemy at a take's place, and what is held
/// (decision 0159, 13).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CountShown {
    pub place: Hex,
    pub asked: Knowledge,
    /// The men held, how the count came, and the day of its report.
    pub held: Option<(u32, GroundGrade, u32)>,
    pub met: bool,
}

/// What a task of the plan is (rule 16): its kind with its places, bases,
/// forces and days, and no number of its own.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TaskShown {
    KeepSupplied {
        base: BaseShown,
        floor_days: u32,
    },
    Cover {
        base: BaseShown,
    },
    Hold {
        base: BaseShown,
    },
    Neutralise {
        target: BaseShown,
    },
    Raid {
        base: BaseShown,
        target: BaseShown,
    },
    Deny {
        hex: Hex,
        distance: u32,
    },
    StrikeShips {
        base: BaseShown,
    },
    SearchWaters {
        base: BaseShown,
    },
    StandAt {
        force: ForceShown,
        hex: Hex,
    },
    OperateCarriers {
        force: ForceShown,
        hex: Hex,
    },
    ReadyLanding {
        force: ForceShown,
        hex: Hex,
        by_day: Option<u32>,
    },
    BuildUp {
        base: BaseShown,
        wants: Vec<Want>,
    },
    TakePlace {
        hex: Hex,
        by_day: Option<u32>,
    },
    Bombard {
        force: ForceShown,
        target: BaseShown,
    },
    ControlIsland {
        island: u32,
        by_day: Option<u32>,
    },
}

/// A task of the plan and the phase it serves.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PlanTaskShown {
    pub task: TaskShown,
    pub phase: Option<PhaseName>,
}

/// What it needs and has, and when it could go (decision 0176, amended 5):
/// the plan order of a world, at the end of a day.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NeedsShown {
    pub world: u32,
    pub day: u32,
    pub men: Option<(u32, u32)>,
    /// The things it waits for that are not met.
    pub waits_for: Vec<WaitShown>,
    pub expected_ashore: Option<u32>,
    pub count: Option<CountShown>,
    pub tasks: Vec<PlanTaskShown>,
    /// The worlds whose plan order differs.
    pub differs_in: Vec<u32>,
}

/// What a force assumed for the enemy was made from (rule 16).
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MadeFrom {
    /// The side's map: a base of the enemy's on it.
    Map,
    /// A contact of the side's, at its place.
    Contact { at: Hex },
    /// Two contacts of the side's taken for one force.
    Pair { first: Hex, second: Hex },
    /// The side's estimate of the men at a place.
    Estimate,
    /// A force or body of men the side believed in, by its name and
    /// standing.
    Believed { name: String, standing: Standing },
    /// Nothing of the side's: what the trial gives the enemy to know or want.
    Trial,
}

/// What a thing assumed is (rule 16): by its kind and its place, never by a
/// number given to a thing made.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AssumedThing {
    /// A base of the enemy's on the side's map, with what it was given.
    EnemyBase {
        base: MapBase,
        place: Hex,
        supply_t: Fixed,
        fuel_t: Fixed,
    },
    /// Men ashore at a place.
    Men { place: Hex, men: u32 },
    /// A force of ships, of a kind, at a place where known.
    Force {
        kind: Option<ShipKind>,
        ships: Option<u32>,
        place: Option<Hex>,
    },
    /// The enemy given no belief: no contact, no claim, no estimate.
    NoBelief,
    /// An intent the enemy is taken to hold, by its kind's words and place.
    EnemyIntent {
        words: String,
        place: Option<Hex>,
        base: Option<MapBase>,
    },
    /// The enemy given no intent.
    NoIntent,
}

/// What else was assumed of a thing, with the count of worlds.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AssumedNoteShown {
    PlacedByReach {
        worlds: u32,
    },
    AtEnemyBase {
        worlds: u32,
    },
    TakenForContact {
        at: Option<Hex>,
        worlds: u32,
    },
    NotMade {
        worlds: u32,
    },
    KindRaised {
        from: ShipKind,
        to: ShipKind,
        worlds: u32,
    },
    ShipsLeftOut {
        kind: ShipKind,
        count: u32,
        worlds: u32,
    },
    GroupLeftOut {
        aircraft: u32,
        worlds: u32,
    },
    RuledOut {
        worlds: u32,
    },
    SetAsideForACount {
        day: u32,
        worlds: u32,
    },
    SetAsideThePlaceIsYours {
        worlds: u32,
    },
}

/// One line of what was assumed (decision 0176, amended 9).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AssumedShown {
    pub thing: AssumedThing,
    pub from: MadeFrom,
    pub made_in: Vec<u32>,
    pub not_in: Vec<u32>,
    pub notes: Vec<AssumedNoteShown>,
}

/// One side of a parting (decision 0176, amended 8).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PartShown {
    pub went: TrialWent,
    pub side: Option<TrialCosts>,
    pub enemy: Option<TrialCosts>,
}

/// Where the worlds part, by a thing of unknown standing.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PartingShown {
    pub by: AssumedThing,
    pub from: MadeFrom,
    pub with: PartShown,
    pub without: PartShown,
}

/// Another of the commander's intents with an end, met and kept with what
/// is tried and without (decision 0176, amended 11).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OtherPlanShown {
    /// The commander's own number.
    pub intent: IntentId,
    pub task: TaskShown,
    pub with: (Vec<u32>, Option<u32>),
    pub without: (Vec<u32>, Option<u32>),
    /// Met in fewer worlds with it.
    pub fewer: bool,
    /// Met as often, and later in the middle.
    pub later: bool,
}

/// The answer of a trial, made for a seat (rule 4).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TrialAnswer {
    /// The worlds and days of the size tried.
    pub worlds: u32,
    pub days: u32,
    /// The size's name, where it was asked by one.
    pub size: Option<String>,
    /// The first day of the trial and the last played.
    pub first_day: u32,
    pub last_day: Option<u32>,
    /// Whether the intent has an end a trial can read.
    pub has_end: bool,
    pub went: TrialWent,
    /// The worlds that could not be played.
    pub could_not: Vec<u32>,
    pub side_costs: Option<TrialCosts>,
    pub enemy_costs: Option<TrialCosts>,
    pub needs: Option<NeedsShown>,
    /// `None` where no second trial was played.
    pub other_plans: Option<Vec<OtherPlanShown>>,
    pub assumed: Vec<AssumedShown>,
    /// The leaves the course gave that the side had not given.
    pub leaves_given: Vec<Leave>,
    /// The count of worlds is not a power of two.
    pub uneven: bool,
    pub partings: Vec<PartingShown>,
    /// The second trial, where one was played (decision 0181, rule 5).
    pub beside: Option<BesideShown>,
}

/// Two estimates of the same worlds compared one for one (decision 0177,
/// amended 10): met and kept in both, in the first alone, in the second
/// alone and in neither, and the worlds that could not be played at one of
/// the two counted apart.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct MatchedShown {
    pub both: Vec<u32>,
    pub first_alone: Vec<u32>,
    pub later_alone: Vec<u32>,
    pub neither: Vec<u32>,
    pub apart: Vec<u32>,
}

/// The second trial, beside the first (decision 0181, rule 5 and amended
/// 1): what it is of, how the intent went there where it is there, what it
/// cost each side there, and, for the plan as it stands, the two world for
/// world.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BesideShown {
    pub of: BesideOf,
    /// `None` where the intent is not in the second trial to have gone.
    pub went: Option<TrialWent>,
    pub last_day: Option<u32>,
    pub could_not: Vec<u32>,
    pub side_costs: Option<TrialCosts>,
    pub enemy_costs: Option<TrialCosts>,
    /// The first against the second; only for the plan as it stands.
    pub matched: Option<MatchedShown>,
}

/// What came due on a date (decision 0177, amended 3): a thing of the side's
/// own, by the live game's number, and the day.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DueThing {
    /// A task force, the day it may first be used.
    Force(OwnForce),
    /// A task force away at a yard, the day it is back.
    BackFromYard(OwnForce),
    /// A land unit, the day it may first be used.
    LandUnit(OwnUnit),
}

/// One thing that came due, and its day.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DueShown {
    pub thing: DueThing,
    pub day: u32,
}

/// A date and what came due on it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DueDateShown {
    pub day: u32,
    pub due: Vec<DueShown>,
}

/// The men a date's take chose (decision 0177, amended 8): the side's own
/// units, by the live game's numbers, with their men, in the world of the
/// plan order, and the worlds whose landings name other units.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ChosenShown {
    pub world: u32,
    pub units: Vec<(OwnUnit, u32)>,
    pub men: u32,
    pub differs_in: Vec<u32>,
}

/// The lowest, the middle and the highest of a cover over the worlds.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CoverShown {
    pub lowest: Fixed,
    pub middle: Fixed,
    pub highest: Fixed,
}

/// The enemy at a place on a date's eve (decision 0177, amended 9): men and
/// cover at a place, the trial's own reckoning, never a unit.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct EnemyEveShown {
    pub place: Hex,
    /// The day whose evening it is.
    pub eve: u32,
    pub men: Option<TrialSpread>,
    pub least_cover: Option<CoverShown>,
}

/// One date of a window (decision 0181, rule 13).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DateShown {
    pub day: u32,
    pub after_first: u32,
    /// The day of the game the order was handed in.
    pub taken_in: Option<u32>,
    pub due: Vec<DueShown>,
    pub chosen: Option<ChosenShown>,
    pub expected_ashore: Option<u32>,
    pub enemy: Vec<EnemyEveShown>,
    /// This date against the first, world for world; `None` at the first.
    pub against_first: Option<MatchedShown>,
    /// The fields a trial's answer has, for this date.
    pub trial: TrialAnswer,
}

/// The answer of a window, made for a seat (decision 0181, rule 13).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WindowAnswer {
    pub worlds: u32,
    pub days: u32,
    pub size: Option<String>,
    pub given: bool,
    pub first_day: u32,
    /// The last day of the first date's play: the bound of the dates.
    pub last_day: u32,
    pub dates: Vec<DateShown>,
    pub not_tried: Vec<DueDateShown>,
}

// ------------------------------------------------------------------- Words

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

/// "2 of 4 worlds (worlds 1 and 3)", "none of the 4 worlds".
fn of_worlds(worlds: &[u32], of: usize) -> String {
    let noun = if of == 1 { "world" } else { "worlds" };
    if worlds.is_empty() {
        return format!("none of the {of} {noun}");
    }
    format!(
        "{} of {of} {noun} ({} {})",
        worlds.len(),
        if worlds.len() == 1 { "world" } else { "worlds" },
        list(worlds)
    )
}

/// A number with a comma between the thousands.
fn thousands(number: u32) -> String {
    let digits = number.to_string();
    let mut out = String::new();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
}

fn spread_json(spread: TrialSpread) -> Value {
    json!({"lowest": spread.lowest, "middle": spread.middle, "highest": spread.highest})
}

fn spread_words(spread: TrialSpread) -> String {
    format!(
        "{}, {}, {}",
        thousands(spread.lowest),
        thousands(spread.middle),
        thousands(spread.highest)
    )
}

fn costs_json(costs: &TrialCosts) -> Value {
    json!({
        "men_lost_to_the_enemy": spread_json(costs.men_to_enemy),
        "men_lost_unfed": spread_json(costs.men_unfed),
        "ships_sunk": spread_json(costs.ships_sunk),
        "ships_out_of_action": spread_json(costs.ships_out_of_action),
        "aircraft_lost": spread_json(costs.aircraft),
    })
}

fn costs_words(costs: &TrialCosts) -> String {
    format!(
        "men lost to the enemy {}; men lost unfed {}; ships sunk {}; ships out of action at the end {}; aircraft lost {}",
        spread_words(costs.men_to_enemy),
        spread_words(costs.men_unfed),
        spread_words(costs.ships_sunk),
        spread_words(costs.ships_out_of_action),
        spread_words(costs.aircraft)
    )
}

/// How the seat's tools name things: its own units by its own numbers, a
/// base by its number and name, a place by its hex.
pub(crate) struct Naming<'a> {
    ctx: Ctx<'a>,
}

impl<'a> Naming<'a> {
    pub(crate) fn new(view: &SideView<'a>) -> Naming<'a> {
        Naming {
            ctx: Ctx::new(*view),
        }
    }

    fn place(&self, hex: Hex) -> String {
        self.ctx.place(hex).text()
    }

    fn base(&self, base: MapBase) -> String {
        self.ctx.base(base.id()).text()
    }

    fn force(&self, force: OwnForce) -> String {
        self.ctx.force(force.id()).text()
    }

    fn ship(&self, ship: OwnShip) -> String {
        self.ctx.ship(ship.id()).text()
    }

    fn unit(&self, unit: OwnUnit) -> String {
        self.ctx.land_unit(unit.id()).text()
    }

    fn force_shown(&self, force: &ForceShown) -> String {
        match force {
            ForceShown::Live(own) => self.force(*own),
            ForceShown::Formed { ships } if ships.is_empty() => {
                "a force the staff would form".to_string()
            }
            ForceShown::Formed { ships } => format!(
                "a force the staff would form of {}",
                ships
                    .iter()
                    .map(|s| self.ship(*s))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }

    fn force_json(&self, force: &ForceShown) -> Value {
        match force {
            ForceShown::Live(own) => {
                json!({"task_force": self.ctx.force(own.id()), "formed": false})
            }
            ForceShown::Formed { ships } => json!({
                "task_force": null,
                "formed": true,
                "ships": ships.iter().map(|s| self.ctx.ship(s.id())).collect::<Vec<_>>(),
            }),
        }
    }

    fn base_shown(&self, base: &BaseShown) -> String {
        match base {
            BaseShown::Live(own) => self.base(*own),
            BaseShown::Formed { place: Some(hex) } => {
                format!(
                    "a landing place the staff would make at {}",
                    self.place(*hex)
                )
            }
            BaseShown::Formed { place: None } => "a landing place the staff would make".to_string(),
        }
    }

    fn base_json(&self, base: &BaseShown) -> Value {
        match base {
            BaseShown::Live(own) => json!({"base": self.ctx.base(own.id()), "formed": false}),
            BaseShown::Formed { place } => json!({
                "base": null,
                "formed": true,
                "place": place.map(|hex| self.ctx.place(hex)),
            }),
        }
    }

    /// A task in the words the tool `order` uses for it.
    pub(crate) fn task_words(&self, task: &TaskShown) -> String {
        match task {
            TaskShown::KeepSupplied { base, floor_days } => format!(
                "keep {} at {floor_days} day{} of supply",
                self.base_shown(base),
                if *floor_days == 1 { "" } else { "s" }
            ),
            TaskShown::Cover { base } => format!("cover {} from the air", self.base_shown(base)),
            TaskShown::Hold { base } => format!("hold {}", self.base_shown(base)),
            TaskShown::Neutralise { target } => format!("neutralise {}", self.base_shown(target)),
            TaskShown::Raid { base, target } => format!(
                "raid {} from {}",
                self.base_shown(target),
                self.base_shown(base)
            ),
            TaskShown::Deny { hex, distance } => format!(
                "deny the waters within {distance} hexes of {}",
                self.place(*hex)
            ),
            TaskShown::StrikeShips { base } => {
                format!("strike ships from {}", self.base_shown(base))
            }
            TaskShown::SearchWaters { base } => {
                format!("search the waters from {}", self.base_shown(base))
            }
            TaskShown::StandAt { force, hex } => format!(
                "stand at {} with {}",
                self.place(*hex),
                self.force_shown(force)
            ),
            TaskShown::OperateCarriers { force, hex } => format!(
                "operate the carrier force {} from {}",
                self.force_shown(force),
                self.place(*hex)
            ),
            TaskShown::ReadyLanding { force, hex, by_day } => match by_day {
                Some(day) => format!(
                    "ready a landing at {} by {} by the morning of day {day}",
                    self.place(*hex),
                    self.force_shown(force)
                ),
                None => format!(
                    "ready a landing at {} by {}",
                    self.place(*hex),
                    self.force_shown(force)
                ),
            },
            TaskShown::BuildUp { base, wants } => format!(
                "build up {}: {}",
                self.base_shown(base),
                wants
                    .iter()
                    .map(want_words)
                    .collect::<Vec<_>>()
                    .join(", then ")
            ),
            TaskShown::TakePlace { hex, by_day } => match by_day {
                Some(day) => format!(
                    "take the place at {} by the end of day {day}",
                    self.place(*hex)
                ),
                None => format!("take the place at {}", self.place(*hex)),
            },
            TaskShown::Bombard { force, target } => format!(
                "bombard {} with {}",
                self.base_shown(target),
                self.force_shown(force)
            ),
            TaskShown::ControlIsland { island, by_day } => match by_day {
                Some(day) => format!("control island {island} by the end of day {day}"),
                None => format!("control island {island}"),
            },
        }
    }

    fn task_json(&self, task: &TaskShown) -> Value {
        let (kind, place, day, force, bases): (
            &str,
            Option<Hex>,
            Option<u32>,
            Option<&ForceShown>,
            Vec<&BaseShown>,
        ) = match task {
            TaskShown::KeepSupplied { base, .. } => ("keep_supplied", None, None, None, vec![base]),
            TaskShown::Cover { base } => ("cover", None, None, None, vec![base]),
            TaskShown::Hold { base } => ("hold", None, None, None, vec![base]),
            TaskShown::Neutralise { target } => ("neutralise", None, None, None, vec![target]),
            TaskShown::Raid { base, target } => ("raid", None, None, None, vec![base, target]),
            TaskShown::Deny { hex, .. } => ("deny", Some(*hex), None, None, vec![]),
            TaskShown::StrikeShips { base } => ("strike_ships", None, None, None, vec![base]),
            TaskShown::SearchWaters { base } => ("search_waters", None, None, None, vec![base]),
            TaskShown::StandAt { force, hex } => {
                ("stand_at", Some(*hex), None, Some(force), vec![])
            }
            TaskShown::OperateCarriers { force, hex } => {
                ("operate_carriers", Some(*hex), None, Some(force), vec![])
            }
            TaskShown::ReadyLanding { force, hex, by_day } => {
                ("ready_landing", Some(*hex), *by_day, Some(force), vec![])
            }
            TaskShown::BuildUp { base, .. } => ("build_up", None, None, None, vec![base]),
            TaskShown::TakePlace { hex, by_day } => {
                ("take_place", Some(*hex), *by_day, None, vec![])
            }
            TaskShown::Bombard { force, target } => {
                ("bombard", None, None, Some(force), vec![target])
            }
            TaskShown::ControlIsland { by_day, .. } => {
                ("control_island", None, *by_day, None, vec![])
            }
        };
        json!({
            "kind": kind,
            "place": place.map(|hex| self.ctx.place(hex)),
            "day": day,
            "force": force.map(|f| self.force_json(f)),
            "bases": bases.iter().map(|b| self.base_json(b)).collect::<Vec<_>>(),
            "text": self.task_words(task),
        })
    }

    fn wait_words(&self, wait: &WaitShown) -> String {
        match wait {
            WaitShown::Men { wanted, to_hand } => format!(
                "men, {} wanted and {} to hand",
                thousands(*wanted),
                thousands(*to_hand)
            ),
            WaitShown::Leave(leave) => format!("your leave to {}", leave.name()),
            WaitShown::Day(day) => format!("day {day}, which you set it not to go before"),
            WaitShown::Force(force) => self.force_shown(force),
            WaitShown::LandUnit(unit) => self.unit(*unit),
            WaitShown::Intent(intent) => format!("your intent {}", intent.raw()),
            WaitShown::Knowledge(knowledge) => format!("{knowledge} of the enemy"),
        }
    }

    fn thing_words(&self, thing: &AssumedThing) -> String {
        match thing {
            AssumedThing::EnemyBase {
                base,
                place,
                supply_t,
                fuel_t,
            } => format!(
                "the enemy's {} at {}, holding {supply_t} t of supply and {fuel_t} t of fuel",
                self.base(*base),
                self.place(*place)
            ),
            AssumedThing::Men { place, men } => {
                format!("{} men at {}", thousands(*men), self.place(*place))
            }
            AssumedThing::Force { kind, ships, place } => {
                let what = match (kind, ships) {
                    (Some(kind), Some(n)) => format!(
                        "a force of {n} ship{} led by a {}",
                        if *n == 1 { "" } else { "s" },
                        kind.name().replace('_', " ")
                    ),
                    (Some(kind), None) => {
                        format!("a force led by a {}", kind.name().replace('_', " "))
                    }
                    _ => "a force of ships".to_string(),
                };
                match place {
                    Some(hex) => format!("{what} near {}", self.place(*hex)),
                    None => what,
                }
            }
            AssumedThing::NoBelief => {
                "the enemy given nothing he could know of you: no contact, no claim and no estimate"
                    .to_string()
            }
            AssumedThing::EnemyIntent { words, place, base } => {
                let at = match (base, place) {
                    (Some(base), _) => format!(" {}", self.base(*base)),
                    (None, Some(hex)) => format!(" at {}", self.place(*hex)),
                    _ => String::new(),
                };
                format!("the enemy taken to intend to {words}{at}")
            }
            AssumedThing::NoIntent => {
                "the enemy given no intent: he holds what he holds".to_string()
            }
        }
    }

    fn made_from_words(&self, from: &MadeFrom) -> Option<String> {
        match from {
            MadeFrom::Map => Some("from your map".to_string()),
            MadeFrom::Contact { at } => Some(format!("from your contact at {}", self.place(*at))),
            MadeFrom::Pair { first, second } => Some(format!(
                "from your contacts at {} and {}, taken for one force",
                self.place(*first),
                self.place(*second)
            )),
            MadeFrom::Estimate => Some("from your estimate of the place".to_string()),
            MadeFrom::Believed { name, standing } => Some(match standing {
                Standing::Unknown => format!("believed, standing unknown (\"{name}\")"),
                Standing::Present => format!("believed present (\"{name}\")"),
                Standing::Absent => format!("believed absent (\"{name}\")"),
            }),
            MadeFrom::Trial => None,
        }
    }

    fn made_from_json(&self, from: &MadeFrom) -> Value {
        match from {
            MadeFrom::Map => json!({"kind": "map"}),
            MadeFrom::Contact { at } => json!({"kind": "contact", "place": self.ctx.place(*at)}),
            MadeFrom::Pair { first, second } => json!({
                "kind": "contacts_as_one",
                "places": [self.ctx.place(*first), self.ctx.place(*second)],
            }),
            MadeFrom::Estimate => json!({"kind": "estimate"}),
            MadeFrom::Believed { name, standing } => json!({
                "kind": "believed",
                "name": name,
                "standing": standing.name(),
            }),
            MadeFrom::Trial => json!({"kind": "trial"}),
        }
    }

    fn thing_json(&self, thing: &AssumedThing) -> Value {
        match thing {
            AssumedThing::EnemyBase {
                base,
                place,
                supply_t,
                fuel_t,
            } => json!({
                "kind": "enemy_base",
                "base": self.ctx.base(base.id()),
                "place": self.ctx.place(*place),
                "supply_t": crate::common::Dec(*supply_t),
                "fuel_t": crate::common::Dec(*fuel_t),
            }),
            AssumedThing::Men { place, men } => json!({
                "kind": "men",
                "place": self.ctx.place(*place),
                "men": men,
            }),
            AssumedThing::Force { kind, ships, place } => json!({
                "kind": "force",
                "ship_kind": kind.map(|k| k.name()),
                "ships": ships,
                "place": place.map(|hex| self.ctx.place(hex)),
            }),
            AssumedThing::NoBelief => json!({"kind": "no_belief"}),
            AssumedThing::EnemyIntent { words, place, base } => json!({
                "kind": "enemy_intent",
                "intent": words,
                "place": place.map(|hex| self.ctx.place(hex)),
                "base": base.map(|b| self.ctx.base(b.id())),
            }),
            AssumedThing::NoIntent => json!({"kind": "no_intent"}),
        }
    }

    fn note_words(&self, note: &AssumedNoteShown) -> String {
        let w = |n: &u32| format!("{n} world{}", if *n == 1 { "" } else { "s" });
        match note {
            AssumedNoteShown::PlacedByReach { worlds } => {
                format!(
                    "placed where it could have reached since it was last seen, in {}",
                    w(worlds)
                )
            }
            AssumedNoteShown::AtEnemyBase { worlds } => {
                format!(
                    "put at a base of the enemy's, its place not known, in {}",
                    w(worlds)
                )
            }
            AssumedNoteShown::TakenForContact { at, worlds } => match at {
                Some(hex) => format!(
                    "your contact at {} taken for it, in {}",
                    self.place(*hex),
                    w(worlds)
                ),
                None => format!("a contact of yours taken for it, in {}", w(worlds)),
            },
            AssumedNoteShown::NotMade { worlds } => format!("not made, in {}", w(worlds)),
            AssumedNoteShown::KindRaised { from, to, worlds } => format!(
                "taken for a {} where the contact said a {}, by a claim of yours, in {}",
                to.name().replace('_', " "),
                from.name().replace('_', " "),
                w(worlds)
            ),
            AssumedNoteShown::ShipsLeftOut {
                kind,
                count,
                worlds,
            } => format!(
                "{count} {} left out, in {}",
                kind.name().replace('_', " "),
                w(worlds)
            ),
            AssumedNoteShown::GroupLeftOut { aircraft, worlds } => {
                format!(
                    "an air group of {aircraft} aircraft left out, in {}",
                    w(worlds)
                )
            }
            AssumedNoteShown::RuledOut { worlds } => {
                format!("believed absent and so left out, in {}", w(worlds))
            }
            AssumedNoteShown::SetAsideForACount { day, worlds } => format!(
                "set aside, for you have a count of the place of day {day}, in {}",
                w(worlds)
            ),
            AssumedNoteShown::SetAsideThePlaceIsYours { worlds } => {
                format!("set aside, for the place is yours, in {}", w(worlds))
            }
        }
    }

    fn assumed_words(&self, line: &AssumedShown, of: usize) -> String {
        let mut out = self.thing_words(&line.thing);
        if let Some(from) = self.made_from_words(&line.from) {
            out.push_str(", ");
            out.push_str(&from);
        }
        if !line.made_in.is_empty() || !line.not_in.is_empty() {
            out.push_str(&format!(", in {}", of_worlds(&line.made_in, of)));
            if !line.not_in.is_empty() {
                out.push_str(&format!(" and not in {}", of_worlds(&line.not_in, of)));
            }
        }
        for note in &line.notes {
            out.push_str("; ");
            out.push_str(&self.note_words(note));
        }
        out
    }

    fn went_json(&self, went: &TrialWent) -> Value {
        json!({
            "worlds": went.played,
            "met_and_kept": went.met_kept,
            "met_days": went.met_days.map(|d| json!({"earliest": d.lowest, "middle": d.middle, "latest": d.highest})),
            "met_and_lost_again": went.lost_again,
            "not_met": went.not_met,
        })
    }

    fn part_words(&self, part: &PartShown) -> String {
        let of = part.went.played.len();
        let mut out = format!(
            "{} world{} ({} {}): met and kept in {}",
            of,
            if of == 1 { "" } else { "s" },
            if of == 1 { "world" } else { "worlds" },
            list(&part.went.played),
            match part.went.met_kept.len() {
                0 => "none".to_string(),
                n => n.to_string(),
            }
        );
        if let Some(days) = part.went.met_days {
            if days.lowest == days.highest {
                out.push_str(&format!(" on day {}", days.lowest));
            } else {
                out.push_str(&format!(
                    " on days {} to {}, the middle day {}",
                    days.lowest, days.highest, days.middle
                ));
            }
        }
        if !part.went.lost_again.is_empty() {
            out.push_str(&format!(
                ", met and lost again in {}",
                part.went.lost_again.len()
            ));
        }
        if let Some(costs) = &part.side {
            out.push_str(&format!(", your side's {}", costs_in_brief(costs)));
        }
        if let Some(costs) = &part.enemy {
            out.push_str(&format!(", the trial's enemy's {}", costs_in_brief(costs)));
        }
        out
    }
}

/// The costs of one side of a parting, in brief: men lost to the enemy
/// always, each other cost only where it is above 0 in some world.
fn costs_in_brief(costs: &TrialCosts) -> String {
    let mut parts = vec![format!(
        "men lost to the enemy {}",
        spread_words(costs.men_to_enemy)
    )];
    for (name, spread) in [
        ("men lost unfed", costs.men_unfed),
        ("ships sunk", costs.ships_sunk),
        ("ships out of action at the end", costs.ships_out_of_action),
        ("aircraft lost", costs.aircraft),
    ] {
        if spread.highest > 0 {
            parts.push(format!("{name} {}", spread_words(spread)));
        }
    }
    parts.join(" and ")
}

fn want_words(want: &Want) -> String {
    match want {
        Want::Airfield => "an airfield".to_string(),
        Want::Mended => "its airfield mended".to_string(),
        Want::Defences(points) => format!("defences to {points}"),
        Want::Port => "its port worked".to_string(),
        Want::AircraftRepair => "aircraft repair worked".to_string(),
    }
}

/// What the session says of what was asked, for the answer.
pub(crate) struct AskedShown {
    /// "intent 1" or "preview 2".
    pub tried: String,
    pub intent: Option<u32>,
    pub preview: Option<u32>,
    /// The intent tried, by the commander's number.
    pub tried_intent: u32,
    /// The words of the intent tried, or of the preview's command.
    pub what: String,
    pub size: String,
    pub for_day: u32,
    pub standing: Vec<String>,
    pub course: Vec<String>,
    pub second_trial: bool,
    pub orders_queued: usize,
    /// What the second trial is of, where one is played.
    pub beside: Option<BesideOf>,
    /// The intent tried is in force on the evening asked, or at the hour
    /// asked inside a day.
    pub in_force: bool,
    pub dates: DatesAsked,
    /// The day open and the hour, "0600" or "1200", where the trial was
    /// asked inside a day (decision 0204, 8); `None` at dusk.
    pub inside_day: Option<(u32, &'static str)>,
    /// The side has men ashore: what is ashore is a day behind in a trial
    /// asked inside a day (decision 0204, 4).
    pub ashore: bool,
}

impl AskedShown {
    /// What was asked, as fields.
    fn json(&self, worlds: u32, days: u32) -> Value {
        json!({
            "intent": self.intent,
            "preview": self.preview,
            "tried_intent": self.tried_intent,
            "size": self.size,
            "worlds": worlds,
            "days": days,
            "for_day": self.for_day,
            "standing": self.standing,
            "course": self.course,
            "second_trial": self.second_trial,
            "orders_not_tried": self.orders_queued,
            "compare": self.beside.is_some(),
            "beside": self.beside.map(BesideOf::name),
            "in_force": self.in_force,
            "dates": match &self.dates {
                DatesAsked::Coming => Value::Null,
                DatesAsked::Chosen => json!("chosen"),
                DatesAsked::Given(days) => json!(days),
            },
            "inside_day": self.inside_day_json(),
        })
    }

    /// `inside_day` as a field: `null` at dusk, else the day and the hour.
    pub(crate) fn inside_day_json(&self) -> Value {
        match self.inside_day {
            Some((day, clock)) => json!({"day": day, "clock": clock}),
            None => Value::Null,
        }
    }

    /// "You have 2 orders to units queued: ...", where any are. Inside a day
    /// the count takes in the orders given for the day's pulses, and the
    /// words say the holds they make are not in it either (decision 0204,
    /// 5).
    fn orders_line(&self) -> Option<String> {
        (self.orders_queued > 0).then(|| {
            let s = if self.orders_queued == 1 { "" } else { "s" };
            if self.inside_day.is_some() {
                format!(
                    "You have {} order{s} to units given for the day's pulses or queued for the coming day: a trial does not try orders to units, and they and the holds they make are not in it.",
                    self.orders_queued
                )
            } else {
                format!(
                    "You have {} order{s} to units queued: a trial does not try orders to units, and they are not in it.",
                    self.orders_queued
                )
            }
        })
    }

    /// The one sentence of a trial asked inside a day (decision 0204, 8):
    /// the hour, that the rest of the day was not played, and where the
    /// side has men ashore that what is ashore is a day behind. `played` is
    /// false for a trial described.
    fn inside_line(&self, played: bool) -> Option<String> {
        let (day, clock) = self.inside_day?;
        Some(format!(
            "Asked at {clock} of day {day}, from things as they stand at that hour: the rest of day {day} {} not played{}.",
            if played { "was" } else { "is" },
            if self.ashore {
                ", and what is ashore is a day behind"
            } else {
                ""
            }
        ))
    }
}

/// The words of what a second trial is of.
fn beside_words(of: BesideOf) -> &'static str {
    match of {
        BesideOf::AsItStands => "the plan as it stands, without the draft",
        BesideOf::WithoutIt => "your side without the order",
        BesideOf::Withdrawn => "your side with the order withdrawn on the coming morning",
    }
}

/// How it went, the worlds that could not be played, what it cost the side
/// and what the trial's enemy lost (rule 17), in words.
fn went_and_cost_lines(answer: &TrialAnswer) -> Vec<String> {
    let of = answer.went.played.len();
    let mut text = Vec::new();
    if of == 0 {
        text.push(format!(
            "How it went: none of the {} worlds could be played, so there is no day and no cost to give.",
            answer.worlds
        ));
    } else if answer.has_end {
        let by = answer
            .last_day
            .map_or_else(String::new, |day| format!(" by day {day}"));
        let mut line = format!(
            "How it went, over {of} world{} played: met and kept in {}",
            if of == 1 { "" } else { "s" },
            of_worlds(&answer.went.met_kept, of)
        );
        if let Some(days) = answer.went.met_days {
            line.push_str(&format!(
                ", the earliest day {}, the middle day {} and the latest day {}",
                days.lowest, days.middle, days.highest
            ));
        }
        line.push_str(&format!(
            "; met and lost again in {}; not met{by} in {}.",
            of_worlds(&answer.went.lost_again, of),
            of_worlds(&answer.went.not_met, of)
        ));
        text.push(line);
    } else {
        text.push(format!(
            "How it went: {of} world{} played; this intent has no end a trial can read, so only what it cost is given.",
            if of == 1 { "" } else { "s" }
        ));
    }
    if !answer.could_not.is_empty() {
        text.push(format!(
            "{} could not be played, and {} counted in nothing above.",
            of_worlds(&answer.could_not, answer.worlds as usize),
            if answer.could_not.len() == 1 {
                "it is"
            } else {
                "they are"
            }
        ));
    }
    if let Some(costs) = &answer.side_costs {
        text.push(format!(
            "What it cost your side, the lowest, the middle and the highest over the worlds: {}.",
            costs_words(costs)
        ));
    }
    if let Some(costs) = &answer.enemy_costs {
        text.push(format!(
            "What the trial's enemy lost, by the trial's own reckoning of an enemy it made from your beliefs, not a claim or a report: {}.",
            costs_words(costs)
        ));
    }
    text
}

/// What it needs and has, and when it could go, in words.
fn needs_lines(n: &Naming<'_>, answer: &TrialAnswer) -> Vec<String> {
    let Some(needs) = &answer.needs else {
        return Vec::new();
    };
    let mut parts = Vec::new();
    if let Some((wanted, to_hand)) = needs.men {
        parts.push(format!(
            "men wanted {}, to hand {}",
            thousands(wanted),
            thousands(to_hand)
        ));
    }
    if needs.waits_for.is_empty() {
        parts.push("it waits for nothing".to_string());
    } else {
        parts.push(format!(
            "it waits for {}",
            needs
                .waits_for
                .iter()
                .map(|w| n.wait_words(w))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    match needs.expected_ashore {
        Some(day) => parts.push(format!("the day expected ashore is day {day}")),
        None => parts.push("no day ashore can yet be given".to_string()),
    }
    if let Some(count) = &needs.count {
        let held = match count.held {
            None => "none is held".to_string(),
            Some((men, grade, day)) => format!(
                "the one held is {} men, {} on day {day}",
                thousands(men),
                match grade {
                    GroundGrade::Told => "told",
                    GroundGrade::SeenFromAir => "seen from the air",
                    GroundGrade::MenInContact => "reported by your men in contact",
                }
            ),
        };
        parts.push(format!(
            "{} of the enemy at {} is asked for, and {held}, which {}",
            capital_free(&count.asked.to_string()),
            n.place(count.place),
            if count.met {
                "meets it"
            } else {
                "does not meet it"
            }
        ));
    }
    let mut line = format!(
        "What it needs and has, as your staff worked it in world {} at the end of day {}: {}.",
        needs.world,
        needs.day,
        parts.join("; ")
    );
    if !needs.tasks.is_empty() {
        line.push_str(&format!(
            " Its tasks: {}.",
            needs
                .tasks
                .iter()
                .map(|t| n.task_words(&t.task))
                .collect::<Vec<_>>()
                .join("; ")
        ));
    }
    if !needs.differs_in.is_empty() {
        line.push_str(&format!(
            " It differs in {}.",
            of_worlds(&needs.differs_in, answer.worlds as usize)
        ));
    }
    vec![line]
}

/// "met and kept in both in 2 worlds (worlds 1 and 3)" and the like.
fn in_worlds(worlds: &[u32]) -> String {
    match worlds.len() {
        0 => "in none".to_string(),
        1 => format!("in 1 world (world {})", worlds[0]),
        count => format!("in {count} worlds (worlds {})", list(worlds)),
    }
}

/// The second trial beside the first (decision 0181, rule 5), in words.
fn beside_lines(asked: &AskedShown, beside: &BesideShown) -> Vec<String> {
    let mut text = Vec::new();
    let mut line = format!("Beside it, {}, in the same worlds", beside_words(beside.of));
    match (&beside.went, beside.of) {
        (Some(went), _) => {
            let of = went.played.len();
            let by = beside
                .last_day
                .map_or_else(String::new, |day| format!(" by day {day}"));
            line.push_str(&format!(
                ": met and kept in {}",
                of_worlds(&went.met_kept, of)
            ));
            if let Some(days) = went.met_days {
                line.push_str(&format!(
                    ", the earliest day {}, the middle day {} and the latest day {}",
                    days.lowest, days.middle, days.highest
                ));
            }
            line.push_str(&format!(
                "; met and lost again in {}; not met{by} in {}.",
                of_worlds(&went.lost_again, of),
                of_worlds(&went.not_met, of)
            ));
        }
        (None, BesideOf::Withdrawn) => line.push_str(&format!(
            ": the order is withdrawn on the coming morning, and what was already ordered stands, a landing being readied, ships under way and men ashore as they are. The withdrawal is the trial's, not yours: intent {} is not withdrawn in your game.",
            asked.tried_intent
        )),
        (None, _) => line.push_str(": the order is not there to have gone, so only what it cost is given."),
    }
    text.push(line);
    if !beside.could_not.is_empty() {
        text.push(format!(
            "Beside it, {} could not be played.",
            of_worlds(&beside.could_not, beside.could_not.len().max(1))
        ));
    }
    if let Some(costs) = &beside.side_costs {
        text.push(format!(
            "Beside it, what it cost your side, the lowest, the middle and the highest over the worlds: {}.",
            costs_words(costs)
        ));
    }
    if let Some(costs) = &beside.enemy_costs {
        text.push(format!(
            "Beside it, what the trial's enemy lost, by the trial's own reckoning: {}.",
            costs_words(costs)
        ));
    }
    if let Some(m) = &beside.matched {
        let mut line = format!(
            "Taken world by world, the draft against the plan as it stands: met and kept in both {}; with the draft alone {}; as it stands alone {}; in neither {}",
            in_worlds(&m.both),
            in_worlds(&m.first_alone),
            in_worlds(&m.later_alone),
            in_worlds(&m.neither)
        );
        if !m.apart.is_empty() {
            line.push_str(&format!(
                "; could not be played in one of the two, counted apart, {}",
                in_worlds(&m.apart)
            ));
        }
        line.push('.');
        text.push(line);
    }
    text
}

/// What the other plans lost (decision 0176), in words.
fn other_plans_lines(n: &Naming<'_>, asked: &AskedShown, answer: &TrialAnswer) -> Vec<String> {
    let mut text = Vec::new();
    match &answer.other_plans {
        None if asked.in_force => text.push(format!(
            "Your order is in force: what it costs your other plans is in things as they stand, and no second trial was played. Asked again with \"compare\" true, intent {} is tried beside your side with it withdrawn on the coming morning, which shows what it costs you.",
            asked.tried_intent
        )),
        None => text
            .push("Your side has no other plan to weigh: no second trial was played.".to_string()),
        Some(plans) if plans.is_empty() => {
            text.push("Your side has no other plan with an end to weigh.".to_string());
        }
        Some(plans) => {
            for plan in plans {
                let mut line = format!(
                    "Your intent {}, {}: met and kept in {} world{} with this and {} without",
                    plan.intent.raw(),
                    n.task_words(&plan.task),
                    plan.with.0.len(),
                    if plan.with.0.len() == 1 { "" } else { "s" },
                    plan.without.0.len()
                );
                if plan.fewer {
                    line.push_str(": displaced, met in fewer worlds");
                } else if plan.later {
                    line.push_str(&format!(
                        ": displaced, met later, the middle day {} against {}",
                        plan.with.1.unwrap_or(0),
                        plan.without.1.unwrap_or(0)
                    ));
                }
                line.push('.');
                text.push(line);
            }
        }
    }
    text
}

/// What was assumed, the leaves given and an uneven count, in words.
fn assumed_lines(n: &Naming<'_>, answer: &TrialAnswer, second: bool) -> Vec<String> {
    let of = answer.went.played.len();
    let mut text = Vec::new();
    if !answer.assumed.is_empty() {
        let lines: Vec<String> = answer
            .assumed
            .iter()
            .map(|line| n.assumed_words(line, of.max(answer.worlds as usize)))
            .collect();
        text.push(format!("What was assumed: {}.", lines.join("; ")));
    }
    if answer.leaves_given.is_empty() {
        text.push(
            "No leave was given by the trial: what it needed you had given, or have queued."
                .to_string(),
        );
    } else {
        for leave in &answer.leaves_given {
            text.push(format!(
                "Leave to {} given to the side by the trial, not by you: it frees every plan of the side that waits for it, not this one alone.{}",
                leave.name(),
                if second {
                    " It is given in both trials."
                } else {
                    ""
                }
            ));
        }
    }
    if answer.uneven {
        text.push(
            "The count of worlds is not a power of two: what you do not know was not tried evenly."
                .to_string(),
        );
    }
    text
}

/// Where the worlds part, in words.
fn parting_lines(n: &Naming<'_>, answer: &TrialAnswer) -> Vec<String> {
    answer
        .partings
        .iter()
        .map(|parting| {
            let mut by = n.thing_words(&parting.by);
            if let Some(from) = n.made_from_words(&parting.from) {
                by.push_str(", ");
                by.push_str(&from);
            }
            format!(
                "Where the worlds part, by {by}. With them, {}. Without them, {}.",
                n.part_words(&parting.with),
                n.part_words(&parting.without)
            )
        })
        .collect()
}

fn matched_json(m: &MatchedShown, first: &str, later: &str) -> Value {
    let mut out = serde_json::Map::new();
    out.insert("both".to_string(), json!(m.both));
    out.insert(first.to_string(), json!(m.first_alone));
    out.insert(later.to_string(), json!(m.later_alone));
    out.insert("neither".to_string(), json!(m.neither));
    out.insert("apart".to_string(), json!(m.apart));
    Value::Object(out)
}

/// The fields of an estimate a trial's answer and a window's date share.
fn estimate_json(n: &Naming<'_>, answer: &TrialAnswer) -> serde_json::Map<String, Value> {
    let of = answer.went.played.len();
    let needs = answer.needs.as_ref().map(|needs| {
        json!({
            "world": needs.world,
            "at_the_end_of_day": needs.day,
            "men": needs.men.map(|(wanted, to_hand)| json!({"wanted": wanted, "to_hand": to_hand})),
            "waits_for": needs.waits_for.iter().map(|w| json!({"text": n.wait_words(w)})).collect::<Vec<_>>(),
            "expected_ashore": needs.expected_ashore,
            "count": needs.count.map(|c| json!({
                "place": n.ctx.place(c.place),
                "asked": c.asked.name(),
                "held": c.held.map(|(men, grade, day)| json!({"men": men, "came_from": grade.name(), "day": day})),
                "met": c.met,
            })),
            "tasks": needs.tasks.iter().map(|t| {
                let mut task = n.task_json(&t.task);
                task["phase"] = json!(t.phase.map(PhaseName::name));
                task
            }).collect::<Vec<_>>(),
            "differs_in": needs.differs_in,
        })
    });
    let fields = json!({
        "first_day": answer.first_day,
        "last_day": answer.last_day,
        "has_end": answer.has_end,
        "how_it_went": n.went_json(&answer.went),
        "could_not_be_played": answer.could_not,
        "cost": answer.side_costs.as_ref().map(costs_json),
        "enemy_lost": answer.enemy_costs.as_ref().map(costs_json),
        "needs": needs,
        "other_plans": answer.other_plans.as_ref().map(|plans| plans.iter().map(|p| json!({
            "intent": p.intent.raw(),
            "task": n.task_json(&p.task),
            "with": {"worlds": p.with.0, "middle_day": p.with.1},
            "without": {"worlds": p.without.0, "middle_day": p.without.1},
            "displaced": if p.fewer { json!("in_fewer_worlds") } else if p.later { json!("later") } else { Value::Null },
        })).collect::<Vec<_>>()),
        "assumed": answer.assumed.iter().map(|line| json!({
            "thing": n.thing_json(&line.thing),
            "from": n.made_from_json(&line.from),
            "made_in": line.made_in,
            "not_in": line.not_in,
            "notes": line.notes.iter().map(|note| n.note_words(note)).collect::<Vec<_>>(),
            "text": n.assumed_words(line, of.max(answer.worlds as usize)),
        })).collect::<Vec<_>>(),
        "leaves_given_to_the_side": answer.leaves_given.iter().map(|l| l.name()).collect::<Vec<_>>(),
        "uneven": answer.uneven,
        "partings": answer.partings.iter().map(|p| json!({
            "by": n.thing_json(&p.by),
            "from": n.made_from_json(&p.from),
            "with": {"how_it_went": n.went_json(&p.with.went), "cost": p.with.side.as_ref().map(costs_json), "enemy_lost": p.with.enemy.as_ref().map(costs_json)},
            "without": {"how_it_went": n.went_json(&p.without.went), "cost": p.without.side.as_ref().map(costs_json), "enemy_lost": p.without.enemy.as_ref().map(costs_json)},
        })).collect::<Vec<_>>(),
    });
    match fields {
        Value::Object(map) => map,
        _ => unreachable!("an object"),
    }
}

/// The fields and the words of a trial's answer (rules 15 to 17), named as
/// the seat's tools name things.
pub(crate) fn answer_fields(
    view: &SideView<'_>,
    number: u32,
    asked: &AskedShown,
    answer: &TrialAnswer,
) -> Value {
    let n = Naming::new(view);
    let mut text: Vec<String> = Vec::new();

    // What was tried, and in how many worlds.
    text.push(format!(
        "Tried: {}, {}, as it would go from the morning of day {}, with what you have handed in for that morning, in {} worlds of {} days each (the size {}).",
        asked.tried,
        asked.what,
        asked.for_day,
        answer.worlds,
        answer.days,
        asked.size
    ));
    text.extend(asked.inside_line(true));
    if !asked.standing.is_empty() {
        text.push(format!(
            "Taken in on that morning as you have it: {}.",
            asked.standing.join("; ")
        ));
    }
    if !asked.course.is_empty() {
        text.push(format!(
            "Added by the trial on that morning: {}.",
            asked.course.join("; ")
        ));
    }
    text.extend(asked.orders_line());
    text.extend(went_and_cost_lines(answer));
    text.extend(needs_lines(&n, answer));
    if let Some(beside) = &answer.beside {
        text.extend(beside_lines(asked, beside));
    }
    text.extend(other_plans_lines(&n, asked, answer));
    text.extend(assumed_lines(&n, answer, answer.beside.is_some()));
    text.extend(parting_lines(&n, answer));

    let mut fields = serde_json::Map::new();
    fields.insert("estimate".to_string(), json!(number));
    fields.insert("asked".to_string(), asked.json(answer.worlds, answer.days));
    fields.extend(estimate_json(&n, answer));
    fields.insert(
        "beside".to_string(),
        answer.beside.as_ref().map_or(Value::Null, |b| {
            json!({
                "of": b.of.name(),
                "how_it_went": b.went.as_ref().map(|w| n.went_json(w)),
                "last_day": b.last_day,
                "could_not_be_played": b.could_not,
                "cost": b.side_costs.as_ref().map(costs_json),
                "enemy_lost": b.enemy_costs.as_ref().map(costs_json),
                "world_for_world": b.matched.as_ref().map(|m| matched_json(m, "with_the_draft_alone", "as_it_stands_alone")),
            })
        }),
    );
    fields.insert("text".to_string(), json!(text));
    Value::Object(fields)
}

impl Naming<'_> {
    fn due_words(&self, due: &DueShown) -> String {
        match due.thing {
            DueThing::Force(force) => {
                format!(
                    "{}, may first be used on day {}",
                    self.force(force),
                    due.day
                )
            }
            DueThing::BackFromYard(force) => {
                format!(
                    "{}, back from the yard on day {}",
                    self.force(force),
                    due.day
                )
            }
            DueThing::LandUnit(unit) => {
                format!("{}, may first be used on day {}", self.unit(unit), due.day)
            }
        }
    }

    fn due_json(&self, due: &DueShown) -> Value {
        let (kind, handle) = match due.thing {
            DueThing::Force(force) => ("task_force", json!(self.ctx.force(force.id()))),
            DueThing::BackFromYard(force) => ("back_from_yard", json!(self.ctx.force(force.id()))),
            DueThing::LandUnit(unit) => ("land_unit", json!(self.ctx.land_unit(unit.id()))),
        };
        json!({"kind": kind, "unit": handle, "day": due.day, "text": self.due_words(due)})
    }

    fn dues_words(&self, due: &[DueShown]) -> String {
        due.iter()
            .map(|d| self.due_words(d))
            .collect::<Vec<_>>()
            .join("; ")
    }
}

fn cover_words(cover: Option<CoverShown>) -> String {
    cover.map_or_else(
        || "none, for no unit of his is there".to_string(),
        |c| format!("{}, {}, {}", c.lowest, c.middle, c.highest),
    )
}

/// The fields and the words of a window's answer (decision 0181, rules 13
/// and 14), named as the seat's tools name things.
pub(crate) fn window_fields(
    view: &SideView<'_>,
    number: u32,
    asked: &AskedShown,
    answer: &WindowAnswer,
) -> Value {
    let n = Naming::new(view);
    let mut text: Vec<String> = Vec::new();
    let first = answer.dates.first().map_or(answer.first_day, |d| d.day);
    text.push(format!(
        "Tried at {} date{}: {}, {}, the whole order handed in at each date, in the same {} worlds, each date played for {} days from its morning (the size {}). The dates were {}: {}.",
        answer.dates.len(),
        if answer.dates.len() == 1 { "" } else { "s" },
        asked.tried,
        asked.what,
        answer.worlds,
        answer.days,
        asked.size,
        if answer.given {
            "given by you"
        } else {
            "chosen from what of yours the order may use comes due"
        },
        answer
            .dates
            .iter()
            .map(|d| format!("day {}", d.day))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    text.extend(asked.inside_line(true));
    if !asked.standing.is_empty() {
        text.push(format!(
            "Taken in on the coming morning, day {first}, at every date, as you have it: {}.",
            asked.standing.join("; ")
        ));
    }
    if !asked.course.is_empty() {
        text.push(format!(
            "Handed in on the morning of each date: {}.",
            asked.course.join("; ")
        ));
    }
    text.extend(asked.orders_line());
    if !answer.given && answer.dates.len() == 1 {
        text.push(format!(
            "Nothing of yours that the order may use is due to change by day {}: the window has one date.",
            answer.last_day
        ));
    }
    let mut dates = Vec::new();
    for (i, date) in answer.dates.iter().enumerate() {
        let mut lines = Vec::new();
        if i == 0 {
            lines.push(format!(
                "The first date, day {}, the coming morning: the order handed in on day {}, played to {}.",
                date.day,
                date.taken_in.unwrap_or(date.day),
                date.trial
                    .last_day
                    .map_or_else(|| "no day".to_string(), |d| format!("day {d}"))
            ));
        } else {
            lines.push(format!(
                "A later date, day {}, {} day{} after the first: the order handed in on day {}, played to {}. Nothing of the order was done before day {}: the units it would take lay idle in the days between, and a landing decided on day {} is ashore later than one prepared from day {first} and held.",
                date.day,
                date.after_first,
                if date.after_first == 1 { "" } else { "s" },
                date.taken_in.unwrap_or(date.day),
                date.trial
                    .last_day
                    .map_or_else(|| "no day".to_string(), |d| format!("day {d}")),
                date.day,
                date.day
            ));
            if date.due.is_empty() {
                lines.push(format!(
                    "What came due by day {}: nothing of yours that the order may use.",
                    date.day
                ));
            } else {
                lines.push(format!(
                    "What came due by day {}: {}.",
                    date.day,
                    n.dues_words(&date.due)
                ));
            }
        }
        match &date.chosen {
            Some(chosen) => {
                let units: Vec<String> = chosen
                    .units
                    .iter()
                    .map(|(unit, men)| format!("{}, {} men", n.unit(*unit), thousands(*men)))
                    .collect();
                let mut line = format!(
                    "Its take chose {} men, in world {}: {}.",
                    thousands(chosen.men),
                    chosen.world,
                    units.join("; ")
                );
                if chosen.differs_in.is_empty() {
                    line.push_str(" The same in every world.");
                } else {
                    line.push_str(&format!(" Other units {}.", in_worlds(&chosen.differs_in)));
                }
                lines.push(line);
            }
            None => lines.push(
                "Its take had formed no landing by the end of the day the order was handed in."
                    .to_string(),
            ),
        }
        for enemy in &date.enemy {
            lines.push(format!(
                "The enemy at {} {}, the trial's own reckoning of an enemy it made, the lowest, the middle and the highest: men ashore {}; the least cover {}.",
                n.place(enemy.place),
                match asked.inside_day {
                    // The first eve of a trial asked inside a day is the
                    // state at the hour asked (decision 0204, 9).
                    Some((day, clock)) if enemy.eve + 1 == answer.first_day => {
                        format!("as things stood at {clock} of day {day}")
                    }
                    _ if enemy.eve + 1 == answer.first_day => {
                        format!("on the evening before day {}", answer.first_day)
                    }
                    _ => format!("on the evening of day {}", enemy.eve),
                },
                enemy
                    .men
                    .map_or_else(|| "none read".to_string(), spread_words),
                cover_words(enemy.least_cover)
            ));
        }
        lines.extend(went_and_cost_lines(&date.trial));
        lines.extend(needs_lines(&n, &date.trial));
        lines.extend(assumed_lines(&n, &date.trial, false));
        lines.extend(parting_lines(&n, &date.trial));
        if let Some(m) = &date.against_first {
            let mut line = format!(
                "Day {} against day {first}, world for world: met and kept at both {}; at the first alone {}; at the later alone {}; at neither {}",
                date.day,
                in_worlds(&m.both),
                in_worlds(&m.first_alone),
                in_worlds(&m.later_alone),
                in_worlds(&m.neither)
            );
            if !m.apart.is_empty() {
                line.push_str(&format!(
                    "; could not be played at one of the two, counted apart, {}",
                    in_worlds(&m.apart)
                ));
            }
            line.push('.');
            lines.push(line);
        }
        let mut fields = serde_json::Map::new();
        fields.insert("day".to_string(), json!(date.day));
        fields.insert("after_first".to_string(), json!(date.after_first));
        fields.insert("taken_in".to_string(), json!(date.taken_in));
        fields.insert(
            "due".to_string(),
            json!(date.due.iter().map(|d| n.due_json(d)).collect::<Vec<_>>()),
        );
        fields.insert(
            "chosen".to_string(),
            date.chosen.as_ref().map_or(Value::Null, |c| {
                json!({
                    "world": c.world,
                    "men": c.men,
                    "units": c.units.iter().map(|(u, men)| json!({"land_unit": n.ctx.land_unit(u.id()), "men": men})).collect::<Vec<_>>(),
                    "differs_in": c.differs_in,
                })
            }),
        );
        fields.insert("expected_ashore".to_string(), json!(date.expected_ashore));
        fields.insert(
            "enemy_on_the_eve".to_string(),
            json!(date
                .enemy
                .iter()
                .map(|e| json!({
                    "place": n.ctx.place(e.place),
                    "eve": e.eve,
                    "men": e.men.map(spread_json),
                    "least_cover": e.least_cover.map(|c| json!({
                        "lowest": crate::common::Dec(c.lowest),
                        "middle": crate::common::Dec(c.middle),
                        "highest": crate::common::Dec(c.highest),
                    })),
                }))
                .collect::<Vec<_>>()),
        );
        fields.extend(estimate_json(&n, &date.trial));
        fields.remove("other_plans");
        fields.insert(
            "against_first".to_string(),
            date.against_first.as_ref().map_or(Value::Null, |m| {
                matched_json(m, "first_alone", "later_alone")
            }),
        );
        fields.insert("text".to_string(), json!(lines));
        text.extend(lines.iter().cloned());
        dates.push(Value::Object(fields));
    }
    for date in &answer.not_tried {
        text.push(format!(
            "Not tried, for a window has three dates at most: day {}, {}.",
            date.day,
            n.dues_words(&date.due)
        ));
    }
    text.push(
        "No second trial is played beside a window, and its answer names no other plan."
            .to_string(),
    );
    json!({
        "estimate": number,
        "asked": asked.json(answer.worlds, answer.days),
        "given": answer.given,
        "first_day": answer.first_day,
        "last_day": answer.last_day,
        "dates": dates,
        "not_tried": answer.not_tried.iter().map(|d| json!({
            "day": d.day,
            "due": d.due.iter().map(|due| n.due_json(due)).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
        "text": text,
    })
}

/// The fields and the words of what would be tried, with nothing played
/// (decision 0181, amended 9).
pub(crate) fn described_fields(
    view: &SideView<'_>,
    asked: &AskedShown,
    described: &Described,
) -> Value {
    let n = Naming::new(view);
    let window = !matches!(asked.dates, DatesAsked::Coming);
    let mut text = vec![format!(
        "Not played: this is what would be tried. {}, {}, from the morning of day {}, at the size {}.",
        capital(&asked.tried),
        asked.what,
        asked.for_day,
        asked.size
    )];
    text.extend(asked.inside_line(false));
    if !asked.standing.is_empty() {
        text.push(format!(
            "Taken in on the coming morning{}, as you have it: {}.",
            if window { " at every date" } else { "" },
            asked.standing.join("; ")
        ));
    }
    if !asked.course.is_empty() {
        text.push(format!(
            "{}: {}.",
            if window {
                "Handed in on the morning of each date"
            } else {
                "Added by the trial on that morning"
            },
            asked.course.join("; ")
        ));
    }
    text.extend(asked.orders_line());
    let first = described
        .dates
        .first()
        .map_or(described.first_day, |d| d.day);
    let mut plays = Vec::new();
    for (i, date) in described.dates.iter().enumerate() {
        let days = described.days + (date.day - first);
        plays.push(
            json!({"trial": "first", "date": date.day, "worlds": described.worlds, "days": days}),
        );
        if window {
            let due = if i == 0 {
                "the coming morning".to_string()
            } else if date.due.is_empty() {
                "nothing of yours that the order may use comes due by it".to_string()
            } else {
                format!("by it, {}", n.dues_words(&date.due))
            };
            text.push(format!(
                "Day {}, {due}: {} worlds of {} days each.",
                date.day, described.worlds, days
            ));
        }
    }
    if let Some(of) = asked.beside {
        plays.push(json!({"trial": "second", "date": first, "worlds": described.worlds, "days": described.days}));
        text.push(format!(
            "Two trials of {} worlds of {} days each, in the same worlds: the first with {}, the second {}.",
            described.worlds,
            described.days,
            if asked.preview.is_some() { "the draft" } else { "the order" },
            beside_words(of)
        ));
    } else if !window {
        text.push(format!(
            "One trial of {} worlds of {} days each, with no second trial beside it.",
            described.worlds, described.days
        ));
    }
    for date in &described.not_tried {
        text.push(format!(
            "Not tried, for a window has three dates at most: day {}, {}.",
            date.day,
            n.dues_words(&date.due)
        ));
    }
    text.push("Nothing is played, nothing is changed and no estimate is kept.".to_string());
    json!({
        "asked": asked.json(described.worlds, described.days),
        "given": described.given,
        "first_day": described.first_day,
        "last_day": described.last_day,
        "dates": described.dates.iter().map(|d| json!({
            "day": d.day,
            "due": d.due.iter().map(|due| n.due_json(due)).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
        "not_tried": described.not_tried.iter().map(|d| json!({
            "day": d.day,
            "due": d.due.iter().map(|due| n.due_json(due)).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
        "plays": plays,
        "text": text,
    })
}

/// Words made to begin with a capital, for the head of a sentence.
fn capital(words: &str) -> String {
    let mut out = words.to_string();
    if let Some(first) = out.get_mut(0..1) {
        first.make_ascii_uppercase();
    }
    out
}

/// Words that begin with a capital made to begin with a small letter, for
/// the middle of a sentence.
fn capital_free(words: &str) -> String {
    let mut out = words.to_string();
    if let Some(first) = out.get_mut(0..1) {
        first.make_ascii_lowercase();
    }
    out
}

/// A command handed in, in the words of the seat: a forces limit names each
/// force, ship, group and unit by the side's own number.
pub(crate) fn command_words(view: &SideView<'_>, command: &CommandKind) -> String {
    let n = Naming::new(view);
    match command {
        CommandKind::SetLimit(Limit::Forces { intent, limit }) => {
            let mut named: Vec<String> = Vec::new();
            named.extend(limit.task_forces.iter().map(|f| n.ctx.force(*f).text()));
            named.extend(limit.ships.iter().map(|s| n.ctx.ship(*s).text()));
            named.extend(limit.air_groups.iter().map(|g| n.ctx.group(*g).text()));
            named.extend(limit.land_units.iter().map(|u| n.ctx.land_unit(*u).text()));
            let mut out = format!(
                "limit intent {}'s forces to {}",
                intent.raw(),
                if named.is_empty() {
                    "none named".to_string()
                } else {
                    named.join(", ")
                }
            );
            if let Some(source) = limit.source {
                out.push_str(&format!(", from {}", n.ctx.base(source).text()));
            }
            out
        }
        CommandKind::SetIntent { intent, kind } => format!(
            "set intent {}: {}",
            intent.raw(),
            crate::staffing::intent_words(view, *kind)
        ),
        CommandKind::SetLeave { leave, given } => format!(
            "{} leave to {}",
            if *given { "give" } else { "withhold" },
            leave.name()
        ),
        other => crate::staffing::commands_text(view, std::slice::from_ref(other))
            .pop()
            .unwrap_or_default(),
    }
}
