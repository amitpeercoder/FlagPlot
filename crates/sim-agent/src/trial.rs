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

/// What a trial is asked (rule 2): its number; its size; the standing
/// commands, what the side would take in on the coming morning without it
/// (rule 8), handed in apart from the course and never shifted; the course;
/// the dates to try, days of the game (one here, the coming morning); the
/// intent whose estimate is wanted; and, where a second trial of things as
/// they stand is wanted, its standing commands (rule 11), tried with an
/// empty course.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TrialAsked {
    pub trial: u32,
    pub size: SizeAsked,
    pub standing: Vec<CommandKind>,
    pub course: Vec<CourseStep>,
    pub dates: Vec<u32>,
    pub intent: IntentId,
    pub as_they_stand: Option<Vec<CommandKind>>,
}

/// Why a trial is refused, in words a seat may be shown.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TrialRefused(pub String);

/// What one step of a trial under way did (rule 3).
pub enum TrialStep {
    /// Some of it was played; more is to come.
    Going,
    /// It is played to its end: the answer.
    Done(Box<TrialAnswer>),
}

/// A trial under way (rule 3): stepped until it is done. Dropped, it leaves
/// nothing.
pub trait TrialUnderWay: Send {
    fn step(&mut self) -> TrialStep;
}

/// What plays a seat's trials (rules 1, 3 and 5), handed to the session by
/// its caller. To begin takes a side's whole view, on an evening, and what
/// is asked; it answers with a trial under way, or why it cannot be tried.
pub trait TrialRunner: Send {
    fn begin(
        &self,
        view: &SideView<'_>,
        asked: &TrialAsked,
    ) -> Result<Box<dyn TrialUnderWay>, TrialRefused>;
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
    let of = answer.went.played.len();
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
    if asked.orders_queued > 0 {
        text.push(format!(
            "You have {} order{} to units queued: a trial does not try orders to units, and they are not in it.",
            asked.orders_queued,
            if asked.orders_queued == 1 { "" } else { "s" }
        ));
    }

    // How it went.
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

    // What it cost the side, and what the trial's enemy lost.
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

    // What it needs and has, and when it could go.
    if let Some(needs) = &answer.needs {
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
        text.push(line);
    }

    // What its other plans lost.
    match &answer.other_plans {
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

    // What was assumed.
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
                "Leave to {} given to the side by the trial, not by you: it frees every plan of the side that waits for it, not this one alone.",
                leave.name()
            ));
        }
    }
    if answer.uneven {
        text.push(
            "The count of worlds is not a power of two: what you do not know was not tried evenly."
                .to_string(),
        );
    }

    // Where the worlds part.
    for parting in &answer.partings {
        let mut by = n.thing_words(&parting.by);
        if let Some(from) = n.made_from_words(&parting.from) {
            by.push_str(", ");
            by.push_str(&from);
        }
        text.push(format!(
            "Where the worlds part, by {by}. With them, {}. Without them, {}.",
            n.part_words(&parting.with),
            n.part_words(&parting.without)
        ));
    }

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
    json!({
        "estimate": number,
        "asked": {
            "intent": asked.intent,
            "preview": asked.preview,
            "tried_intent": asked.tried_intent,
            "size": asked.size,
            "worlds": answer.worlds,
            "days": answer.days,
            "for_day": asked.for_day,
            "standing": asked.standing,
            "course": asked.course,
            "second_trial": asked.second_trial,
            "orders_not_tried": asked.orders_queued,
        },
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
        "text": text,
    })
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
