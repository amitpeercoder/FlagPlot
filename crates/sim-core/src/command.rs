//! What the commander says, as state (decision 0107, step 3, stage 1).
//!
//! Each side has a [`CommandState`]: its intents with their limits, the two
//! leaves, its plans with their baselines, amendments and runs, and the
//! commitments of its ships to them. It is hashed and named beside the hash.
//! It changes only by a [`Command`], handed in beside orders, checked and
//! accepted or refused with a reason as an order is, each change an event
//! with its causes (`CommandTaken`, `CommandRefused`, `PlanDrafted`,
//! `PlanStateChanged`, `PlanAmended`), seen whole by its own side and not at
//! all by the enemy.
//!
//! A command comes from the commander (`Issuer::Player`, taken in at the
//! first step of a day) or from the staff (`Issuer::Staff`, taken in after
//! the staff is asked). What each may do is fixed by the kind of command
//! ([`CommandKind::allowed_from`]); intake refuses anything else.
//!
//! What a staff is lives here too: [`Staff`], given a side's view, returns
//! [`StaffDecisions`]: orders with their reasons, staff commands, and
//! decisions to give no order with their reasons. The function that resolves
//! a day takes each side's staff or none ([`Staffs`]). The staff itself is
//! code outside this crate.
//!
//! Holds, overrides and interrupts (decision 0107, items 20, 28 and the
//! table's step 2) are later steps of stage 1. Their place is marked where
//! they will go: a field of [`CommandState`] each, and the step of the day in
//! `resolve.rs`. None is built.
//!
//! Reads: nothing of the truth but what intake checks against (the side's
//! own ships, forces and bases). Writes: the side's command state, and a
//! base's reserves through the reserve limit. Draws no random number.

pub(crate) mod intake;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Mutex;

use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{BaseId, IntentId, ShipId, TaskForceId};
use crate::orders::{Issuer, OrderKind};
use crate::state::Side;
use crate::view::SideView;

// ---------------------------------------------------------------------------
// Numbers.
// ---------------------------------------------------------------------------

/// A plan of one side, numbered in the side's own series from 1. The side
/// sees its own numbers; nothing of it reaches the other side.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PlanId(u32);

impl PlanId {
    pub const fn new(raw: u32) -> PlanId {
        PlanId(raw)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }
}

impl core::fmt::Display for PlanId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "plan:{}", self.0)
    }
}

// ---------------------------------------------------------------------------
// Intents and limits.
// ---------------------------------------------------------------------------

/// What an intent asks for. Stage 1 has one kind (decision 0107, 17, with
/// question 5 as answered).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IntentKind {
    /// Keep the side's stock in `base`'s hex at `floor_days` days of supply
    /// or above, each run bringing the use of `interval_days` days above the
    /// floor. `None` for the interval: the staff proposes it. It has no date:
    /// it stands until withdrawn.
    KeepSupplied {
        base: BaseId,
        floor_days: u32,
        interval_days: Option<u32>,
    },
}

impl IntentKind {
    /// A stable number for the kind. Part of the state hash. Never renumber.
    pub const fn code(self) -> u8 {
        match self {
            IntentKind::KeepSupplied { .. } => 1,
        }
    }
}

/// The risk level of an intent (the spec's table). Stored and shown; read by
/// nothing in stage 1.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub enum Risk {
    Preserve,
    #[default]
    Balanced,
    Press,
}

impl Risk {
    /// A stable number. Part of the state hash. Never renumber.
    pub const fn code(self) -> u8 {
        match self {
            Risk::Preserve => 1,
            Risk::Balanced => 2,
            Risk::Press => 3,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Risk::Preserve => "preserve",
            Risk::Balanced => "balanced",
            Risk::Press => "press",
        }
    }
}

/// The forces limit of an intent: the ships and forces the staff may use
/// for it, the pool, and for a supply intent exactly one source base
/// (decision 0107, 19).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ForcesLimit {
    pub ships: BTreeSet<ShipId>,
    pub task_forces: BTreeSet<TaskForceId>,
    pub source: BaseId,
}

/// The limits of one intent as they stand. A limit not set is `None` (or the
/// default). With no forces limit the staff may use nothing. A limit the
/// commander has not set does not bind: with no area limit the sea is open
/// (the architect's ruling, Nov 15, 2026).
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct IntentLimits {
    pub forces: Option<ForcesLimit>,
    /// The hexes the intent's forces may enter. `None`: no area set, and the
    /// intent's forces may go anywhere at sea.
    pub area: Option<BTreeSet<Hex>>,
    /// The intent's rank, 1 first. `None`: after every ranked intent, then
    /// by intent number.
    pub priority: Option<u32>,
    /// Ships the staff may not take from this intent (the protected limit).
    pub protected: BTreeSet<ShipId>,
    pub risk: Risk,
}

/// One limit, as a command sets it. The reserve sets a base's reserves of
/// decision 0111 and names no intent: a reserve is the base's.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Limit {
    Forces {
        intent: IntentId,
        limit: ForcesLimit,
    },
    Area {
        intent: IntentId,
        hexes: BTreeSet<Hex>,
    },
    Priority {
        intent: IntentId,
        rank: Option<u32>,
    },
    Protected {
        intent: IntentId,
        ships: BTreeSet<ShipId>,
    },
    Reserve {
        base: BaseId,
        supply_t: Fixed,
        fuel_t: Fixed,
    },
    Risk {
        intent: IntentId,
        risk: Risk,
    },
}

impl Limit {
    /// The intent the limit is for; `None` for a reserve.
    pub fn intent(&self) -> Option<IntentId> {
        match self {
            Limit::Forces { intent, .. }
            | Limit::Area { intent, .. }
            | Limit::Priority { intent, .. }
            | Limit::Protected { intent, .. }
            | Limit::Risk { intent, .. } => Some(*intent),
            Limit::Reserve { .. } => None,
        }
    }
}

/// An intent in the side's command state.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Intent {
    pub id: IntentId,
    pub kind: IntentKind,
    pub limits: IntentLimits,
    /// The day it was first set.
    pub set_day: u32,
    /// The plan that serves it, drafted when it was first set.
    pub plan: PlanId,
    /// The day it was withdrawn; `None` while it is in force. A withdrawn
    /// intent's number is never used again.
    pub withdrawn_day: Option<u32>,
}

impl Intent {
    pub fn in_force(&self) -> bool {
        self.withdrawn_day.is_none()
    }
}

/// The two leaves of a seat (the spec's "two permissions").
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Leaves {
    /// A draft inside the limits may be made ready without the commander.
    pub prepare: bool,
    /// A plan that is ready may be released without him.
    pub launch: bool,
}

impl Default for Leaves {
    /// The spec's default for a player: preparing is allowed, launching
    /// waits for him.
    fn default() -> Leaves {
        Leaves {
            prepare: true,
            launch: false,
        }
    }
}

/// Which leave a command sets.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Leave {
    Prepare,
    Launch,
}

// ---------------------------------------------------------------------------
// Plans.
// ---------------------------------------------------------------------------

/// A plan's state (the spec's "plan's life").
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum PlanState {
    Draft,
    Preparing,
    Released,
    Complete,
    Failed,
    Cancelled,
}

impl PlanState {
    /// A stable number. Part of the state hash. Never renumber.
    pub const fn code(self) -> u8 {
        match self {
            PlanState::Draft => 1,
            PlanState::Preparing => 2,
            PlanState::Released => 3,
            PlanState::Complete => 4,
            PlanState::Failed => 5,
            PlanState::Cancelled => 6,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            PlanState::Draft => "draft",
            PlanState::Preparing => "preparing",
            PlanState::Released => "released",
            PlanState::Complete => "complete",
            PlanState::Failed => "failed",
            PlanState::Cancelled => "cancelled",
        }
    }

    /// Preparing or released: the spec's "approved".
    pub const fn approved(self) -> bool {
        matches!(self, PlanState::Preparing | PlanState::Released)
    }

    /// Complete, failed or cancelled: nothing moves it again.
    pub const fn ended(self) -> bool {
        matches!(
            self,
            PlanState::Complete | PlanState::Failed | PlanState::Cancelled
        )
    }
}

/// What a plan was approved under: its intent and the intent's limits on
/// the day it was first approved (decision 0107, 22). It never changes.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Baseline {
    pub day: u32,
    pub kind: IntentKind,
    pub limits: IntentLimits,
}

/// What an amendment changed: the commander's new intent or limit, applied
/// to a plan that is approved.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AmendmentChange {
    Intent(IntentKind),
    Limit(Limit),
}

/// A numbered change to an approved plan (the spec's "amendment").
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Amendment {
    /// 1 for the plan's first, and so on.
    pub number: u32,
    pub day: u32,
    pub change: AmendmentChange,
}

/// The days of a run, as the staff timed it. Each is a day of the game:
/// loading begins on `load`, the force sails on `sail`, it is due at the
/// destination on `arrive`, and its ships are free (back at the source and
/// refuelled) on `free`. `load <= sail <= arrive <= free`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RunDays {
    pub load: u32,
    pub sail: u32,
    pub arrive: u32,
    pub free: u32,
}

/// What a run is to be: its ships, its load from the source and its days.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RunSpec {
    pub ships: BTreeSet<ShipId>,
    /// Tons of supply. Above zero.
    pub load_t: Fixed,
    pub days: RunDays,
}

/// A convoy run of a supply plan: a task inside its one phase (decision
/// 0107, 22 and 26). Its source is the intent's.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Run {
    /// 1 for the plan's first run, and so on.
    pub number: u32,
    pub ships: BTreeSet<ShipId>,
    pub load_t: Fixed,
    pub source: BaseId,
    pub days: RunDays,
}

/// A plan of one side.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Plan {
    pub id: PlanId,
    pub intent: IntentId,
    pub state: PlanState,
    /// Set when the plan is first approved; `None` while it is a draft.
    pub baseline: Option<Baseline>,
    pub amendments: Vec<Amendment>,
    pub runs: BTreeMap<u32, Run>,
}

/// A ship given to a plan from one day to another (decision 0107, 27). A
/// ship holds at most one commitment on any day; it may have another after
/// it. Freed by the staff.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Commitment {
    pub plan: PlanId,
    /// The run it serves, if any.
    pub run: Option<u32>,
    pub from_day: u32,
    /// The last day it is planned to last. Inclusive.
    pub to_day: u32,
}

/// One side's command state (decision 0107, items 1, 2 and 6).
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct CommandState {
    /// Every intent ever set, in force or withdrawn, by number.
    pub intents: BTreeMap<IntentId, Intent>,
    pub leaves: Leaves,
    pub plans: BTreeMap<PlanId, Plan>,
    /// Each ship's commitments, in order of their first day.
    pub commitments: BTreeMap<ShipId, Vec<Commitment>>,
    /// The number the side's next plan takes. Starts at 1.
    pub next_plan: u32,
    // Holds (a setting, a unit, a parent force) and interrupts (set,
    // fired, acknowledged) go here, each a field hashed and named beside
    // the hash, in the later steps of stage 1. None is built.
}

impl CommandState {
    pub fn new() -> CommandState {
        CommandState {
            next_plan: 1,
            ..CommandState::default()
        }
    }

    /// The intents in force, by number.
    pub fn intents_in_force(&self) -> impl Iterator<Item = &Intent> + '_ {
        self.intents.values().filter(|intent| intent.in_force())
    }

    /// The commitment of `ship` that covers `day`, if any.
    pub fn commitment_on(&self, ship: ShipId, day: u32) -> Option<&Commitment> {
        self.commitments
            .get(&ship)?
            .iter()
            .find(|c| c.from_day <= day && day <= c.to_day)
    }

    /// Whether the command state is empty: nothing was ever set. A side
    /// with an empty command state is played exactly as before stage 1.
    pub fn is_empty(&self) -> bool {
        self.intents.is_empty()
            && self.plans.is_empty()
            && self.commitments.is_empty()
            && self.leaves == Leaves::default()
            && self.next_plan == 1
    }
}

// ---------------------------------------------------------------------------
// Commands.
// ---------------------------------------------------------------------------

/// How a staff marks a plan's end.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum PlanOutcome {
    Complete,
    Failed,
}

/// A command: a change to the side's command state. The first six are the
/// commander's; the rest the staff's, with `Approve` for both (the staff
/// only where the commander gave leave).
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CommandKind {
    /// Set an intent, or change one in force. The commander names its
    /// number. The first time a number is set its plan is drafted; a change
    /// to an intent whose plan is approved is an amendment of it.
    SetIntent { intent: IntentId, kind: IntentKind },
    /// Withdraw an intent in force. Its plan ends: complete if released,
    /// else cancelled.
    WithdrawIntent { intent: IntentId },
    /// Set one limit. A change to a limit of an intent whose plan is
    /// approved is an amendment of it.
    SetLimit(Limit),
    /// Give or withhold one leave.
    SetLeave { leave: Leave, given: bool },
    /// Prepare a draft, or release a plan that is preparing.
    Approve { plan: PlanId },
    /// Cancel a plan that has not ended. Its ships come home: the staff's
    /// orders, by the plan's fallback.
    CancelPlan { plan: PlanId },
    /// The staff's: commit ships of the intent's pool to an approved plan
    /// from one day to another.
    CommitShips {
        plan: PlanId,
        run: Option<u32>,
        ships: BTreeSet<ShipId>,
        from_day: u32,
        to_day: u32,
    },
    /// The staff's: end the commitments of ships to a plan.
    FreeShips {
        plan: PlanId,
        ships: BTreeSet<ShipId>,
    },
    /// The staff's: add a run inside an approved plan. It takes the plan's
    /// next run number.
    AddRun { plan: PlanId, run: RunSpec },
    /// The staff's: time a run again.
    TimeRun {
        plan: PlanId,
        run: u32,
        days: RunDays,
    },
    /// The staff's: mark an approved plan complete or failed.
    MarkPlan { plan: PlanId, outcome: PlanOutcome },
}

impl CommandKind {
    /// Whether `issuer` may give this command at all. Intake refuses it
    /// otherwise (`CommandRefusal::NotYours`). The staff's `Approve` is
    /// further bound by the leaves.
    pub fn allowed_from(&self, issuer: Issuer) -> bool {
        match self {
            CommandKind::SetIntent { .. }
            | CommandKind::WithdrawIntent { .. }
            | CommandKind::SetLimit(_)
            | CommandKind::SetLeave { .. }
            | CommandKind::CancelPlan { .. } => issuer == Issuer::Player,
            CommandKind::Approve { .. } => {
                matches!(issuer, Issuer::Player | Issuer::Staff)
            }
            CommandKind::CommitShips { .. }
            | CommandKind::FreeShips { .. }
            | CommandKind::AddRun { .. }
            | CommandKind::TimeRun { .. }
            | CommandKind::MarkPlan { .. } => issuer == Issuer::Staff,
        }
    }

    /// The command's name as scenario files, saved games and the log write
    /// it.
    pub const fn name(&self) -> &'static str {
        match self {
            CommandKind::SetIntent { .. } => "set_intent",
            CommandKind::WithdrawIntent { .. } => "withdraw_intent",
            CommandKind::SetLimit(_) => "set_limit",
            CommandKind::SetLeave { .. } => "set_leave",
            CommandKind::Approve { .. } => "approve",
            CommandKind::CancelPlan { .. } => "cancel_plan",
            CommandKind::CommitShips { .. } => "commit_ships",
            CommandKind::FreeShips { .. } => "free_ships",
            CommandKind::AddRun { .. } => "add_run",
            CommandKind::TimeRun { .. } => "time_run",
            CommandKind::MarkPlan { .. } => "mark_plan",
        }
    }
}

/// Why a command was refused. A unit or base that is not the side's own and
/// one that does not exist are refused alike, so a refusal never says that
/// an enemy unit exists.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CommandRefusal {
    /// The issuer may not give this kind of command.
    NotYours,
    /// The intent's number is 0, or was withdrawn and is not used again.
    IntentNumberNotUsable(IntentId),
    /// No intent of that number is in force.
    NoSuchIntent(IntentId),
    /// The base is not one of the side's own.
    NotYourBase(BaseId),
    /// A ship is not one of the side's own afloat.
    NotYourShip(ShipId),
    /// A task force is not one of the side's own.
    NotYourForce(TaskForceId),
    /// A number of days that must be at least 1 is 0.
    NoDays,
    /// The area names no hex, or a hex off the map.
    BadArea,
    /// A reserve below zero, or a load not above zero.
    BadWeight(Fixed),
    /// No plan of that number.
    NoSuchPlan(PlanId),
    /// The plan is not in a state the command applies to.
    WrongPlanState(PlanState),
    /// The staff moved a plan the commander has not given it leave to move.
    NoLeave(Leave),
    /// A ship is not in the pool of the plan's intent.
    NotInPool(ShipId),
    /// A ship is committed to another plan on a day asked for.
    AlreadyCommitted(ShipId),
    /// A ship is not committed to the plan.
    NotCommitted(ShipId),
    /// The days are out of order, or begin before the day the command is
    /// taken in.
    BadDays,
    /// No run of that number in the plan.
    NoSuchRun(u32),
    /// The plan's intent has no forces limit.
    NoForcesLimit,
    /// A run names no ship. Added at the end (decision 0107).
    NoShips,
}

impl core::fmt::Display for CommandRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            CommandRefusal::NotYours => f.write_str("the issuer may not give this command"),
            CommandRefusal::IntentNumberNotUsable(id) => {
                write!(f, "{id} is not a number an intent may take")
            }
            CommandRefusal::NoSuchIntent(id) => write!(f, "{id} is not in force"),
            CommandRefusal::NotYourBase(base) => write!(f, "{base} is not one of the side's bases"),
            CommandRefusal::NotYourShip(ship) => write!(f, "{ship} is not one of the side's ships"),
            CommandRefusal::NotYourForce(force) => {
                write!(f, "{force} is not one of the side's forces")
            }
            CommandRefusal::NoDays => f.write_str("a number of days is 0"),
            CommandRefusal::BadArea => f.write_str("the area is empty or off the map"),
            CommandRefusal::BadWeight(weight) => write!(f, "{weight} t is not an allowed weight"),
            CommandRefusal::NoSuchPlan(plan) => write!(f, "{plan} does not exist"),
            CommandRefusal::WrongPlanState(state) => {
                write!(
                    f,
                    "the plan is {}, which the command does not apply to",
                    state.name()
                )
            }
            CommandRefusal::NoLeave(leave) => write!(f, "no leave to {}", leave.name()),
            CommandRefusal::NotInPool(ship) => write!(f, "{ship} is not in the intent's pool"),
            CommandRefusal::AlreadyCommitted(ship) => {
                write!(f, "{ship} is committed on a day asked for")
            }
            CommandRefusal::NotCommitted(ship) => write!(f, "{ship} is not committed to the plan"),
            CommandRefusal::BadDays => f.write_str("the days are out of order"),
            CommandRefusal::NoSuchRun(run) => write!(f, "the plan has no run {run}"),
            CommandRefusal::NoForcesLimit => f.write_str("the intent has no forces limit"),
            CommandRefusal::NoShips => f.write_str("the run names no ship"),
        }
    }
}

impl Leave {
    pub const fn name(self) -> &'static str {
        match self {
            Leave::Prepare => "prepare",
            Leave::Launch => "launch",
        }
    }
}

impl core::fmt::Display for CommandKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            CommandKind::SetIntent { intent, kind } => match kind {
                IntentKind::KeepSupplied {
                    base,
                    floor_days,
                    interval_days,
                } => {
                    write!(
                        f,
                        "set {intent} to keep {base} supplied at {floor_days} days"
                    )?;
                    match interval_days {
                        Some(days) => write!(f, ", a run every {days} days"),
                        None => Ok(()),
                    }
                }
            },
            CommandKind::WithdrawIntent { intent } => write!(f, "withdraw {intent}"),
            CommandKind::SetLimit(limit) => match limit {
                Limit::Forces { intent, limit } => write!(
                    f,
                    "limit {intent}'s forces to {} ships and {} task forces from {}",
                    limit.ships.len(),
                    limit.task_forces.len(),
                    limit.source
                ),
                Limit::Area { intent, hexes } => {
                    write!(f, "limit {intent}'s area to {} hexes", hexes.len())
                }
                Limit::Priority { intent, rank } => match rank {
                    Some(rank) => write!(f, "rank {intent} at {rank}"),
                    None => write!(f, "leave {intent} unranked"),
                },
                Limit::Protected { intent, ships } => {
                    write!(f, "protect {} ships of {intent}", ships.len())
                }
                Limit::Reserve {
                    base,
                    supply_t,
                    fuel_t,
                } => write!(
                    f,
                    "reserve {supply_t} t of supply and {fuel_t} t of fuel at {base}"
                ),
                Limit::Risk { intent, risk } => write!(f, "risk of {intent} is {}", risk.name()),
            },
            CommandKind::SetLeave { leave, given } => write!(
                f,
                "{} the leave to {}",
                if *given { "give" } else { "withhold" },
                leave.name()
            ),
            CommandKind::Approve { plan } => write!(f, "approve {plan}"),
            CommandKind::CancelPlan { plan } => write!(f, "cancel {plan}"),
            CommandKind::CommitShips {
                plan,
                run,
                ships,
                from_day,
                to_day,
            } => {
                write!(f, "commit {} ships to {plan}", ships.len())?;
                if let Some(run) = run {
                    write!(f, " run {run}")?;
                }
                write!(f, " from day {from_day} to day {to_day}")
            }
            CommandKind::FreeShips { plan, ships } => {
                write!(f, "free {} ships from {plan}", ships.len())
            }
            CommandKind::AddRun { plan, run } => write!(
                f,
                "add a run of {} t with {} ships to {plan}, loading day {}, due day {}",
                run.load_t,
                run.ships.len(),
                run.days.load,
                run.days.arrive
            ),
            CommandKind::TimeRun { plan, run, days } => write!(
                f,
                "time run {run} of {plan}: load day {}, sail {}, arrive {}, free {}",
                days.load, days.sail, days.arrive, days.free
            ),
            CommandKind::MarkPlan { plan, outcome } => write!(
                f,
                "mark {plan} {}",
                match outcome {
                    PlanOutcome::Complete => "complete",
                    PlanOutcome::Failed => "failed",
                }
            ),
        }
    }
}

/// A command handed in: what and by whom.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Command {
    pub kind: CommandKind,
    pub issuer: Issuer,
}

// ---------------------------------------------------------------------------
// Reasons.
// ---------------------------------------------------------------------------

/// What a staff decision serves: the intent, and the plan and amendment.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Serves {
    pub intent: Option<IntentId>,
    pub plan: Option<PlanId>,
    /// The plan's amendment it was made under; `None` for the baseline.
    pub amendment: Option<u32>,
}

/// One piece of what the side held that mattered, with its day.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Evidence {
    pub what: String,
    pub day: u32,
}

/// The record of one staff decision, an order given or withheld (the spec's
/// "Reasons", five parts). It names the side's own ships freely and the
/// enemy only as contacts, claims and estimates: that is the staff's to keep.
/// It is handed in with its order or with the decision to give none,
/// recorded as an event (`StaffReason`) and kept in the log, so it resolves
/// by that event's number after its intent is replaced.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Reason {
    pub serves: Serves,
    pub evidence: Vec<Evidence>,
    /// The limit, the stock, the distance or the commitment that shaped it.
    pub binding: Vec<String>,
    /// The calculation.
    pub worked: String,
    /// What, if different, would give a different decision.
    pub would_change: Vec<String>,
}

// ---------------------------------------------------------------------------
// The staff.
// ---------------------------------------------------------------------------

/// An order the staff gives, with the intent it serves and its reason. Its
/// origin is the staff and that intent; intake refuses it if its unit is not
/// in the intent's forces limit.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StaffOrder {
    pub kind: OrderKind,
    pub intent: IntentId,
    pub reason: Reason,
}

/// A command the staff gives, with its reason. Intake records the reason as
/// a `StaffReason` with no order, citing the command's `CommandTaken` or
/// `CommandRefused`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StaffCommand {
    pub kind: CommandKind,
    pub reason: Reason,
}

/// What a staff hands in for one day (decision 0107, 4): orders with their
/// reasons, staff commands, and decisions to give no order with theirs. It
/// is what a saved game keeps for the day, and what a replay hands in again
/// without asking the staff.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct StaffDecisions {
    pub orders: Vec<StaffOrder>,
    pub commands: Vec<StaffCommand>,
    pub no_orders: Vec<Reason>,
}

impl StaffDecisions {
    pub fn is_empty(&self) -> bool {
        self.orders.is_empty() && self.commands.is_empty() && self.no_orders.is_empty()
    }
}

/// A staff: given a side's view, it decides the day. It is a function of the
/// view: `&self`, so that asked twice on the same view it gives the same
/// answer, which `replay --check-staff` tests. It is asked once a day, at
/// step 4 of the day's order, after the commander's commands are taken in
/// and ships in port have refuelled, and before any order is taken in.
pub trait Staff {
    fn decide(&self, view: SideView<'_>) -> StaffDecisions;

    /// What the staff would do for one intent, worked from the view and
    /// stored nowhere (decision 0107, 5). `None` by default: a staff with no
    /// doctrine for the intent proposes nothing.
    fn propose(&self, _view: SideView<'_>, _intent: IntentId) -> Option<Proposal> {
        None
    }
}

/// One reading of the staff's projection: the stock at the start of `day`,
/// the planned use of that day, and the figure (`None` where the use is 0).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProjectedReading {
    pub day: u32,
    pub stock_t: Fixed,
    pub use_t: Fixed,
    pub days: Option<Fixed>,
}

/// The staff's projection of days of supply in a hex (decision 0107, 35).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SupplyProjection {
    pub hex: Hex,
    pub floor_days: u32,
    pub readings: Vec<ProjectedReading>,
    /// The first day whose figure is below `floor_days`.
    pub first_day_below: Option<u32>,
}

/// A run the staff proposes: its ships, its load in tons of supply, its
/// four days and why.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProposedRun {
    pub ships: BTreeSet<ShipId>,
    pub load_t: Fixed,
    pub days: RunDays,
    pub reason: Reason,
}

/// What stops the staff meeting an intent.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Shortfall {
    /// What binds, in words.
    pub binds: String,
    /// The shortfall in days at the reading it checked.
    pub shortfall_days: Fixed,
    /// The day the source falls to its reserve at the present rate.
    pub source_dry_day: Option<u32>,
    /// The day whose reading the shortfall was taken at.
    pub check_day: u32,
    /// The commitments to a plan that keep the intent from a run or cut its
    /// load. Empty where nothing of the sort does. Not state: not hashed,
    /// not saved.
    pub displaced_by: Vec<Displacement>,
}

/// What a commitment to a plan holds: ships, or tons at the source still to
/// be loaded.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Committed {
    Ships(BTreeSet<ShipId>),
    Tons(Fixed),
}

/// A commitment that keeps an intent from a run or cuts its load.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Displacement {
    pub intent: IntentId,
    pub plan: PlanId,
    pub run: u32,
    pub committed: Committed,
    pub from_day: u32,
    pub to_day: u32,
}

/// An intent a proposal would displace, with its shortfall (whose
/// `displaced_by` names only the proposing plan's commitments).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Displaced {
    pub intent: IntentId,
    pub plan: PlanId,
    pub shortfall: Shortfall,
}

/// What the staff would do for one intent, worked from the view and stored
/// nowhere (decision 0107, 5).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Proposal {
    pub intent: IntentId,
    pub plan: PlanId,
    pub plan_state: PlanState,
    pub projection: Option<SupplyProjection>,
    pub decisions: StaffDecisions,
    pub runs: Vec<ProposedRun>,
    pub shortfall: Option<Shortfall>,
    /// The lower intents this proposal would displace.
    pub displaces: Vec<Displaced>,
    pub reason: Reason,
}

/// Each side's staff, or none. The function that resolves a day takes it.
#[derive(Clone, Copy, Default)]
pub struct Staffs<'a> {
    allied: Option<&'a dyn Staff>,
    japanese: Option<&'a dyn Staff>,
}

impl<'a> Staffs<'a> {
    /// No staff for either side: the day is resolved as before stage 1.
    pub const fn none() -> Staffs<'a> {
        Staffs {
            allied: None,
            japanese: None,
        }
    }

    /// These staffs with `staff` for `side`.
    pub fn with(mut self, side: Side, staff: &'a dyn Staff) -> Staffs<'a> {
        match side {
            Side::Allied => self.allied = Some(staff),
            Side::Japanese => self.japanese = Some(staff),
        }
        self
    }

    pub fn get(&self, side: Side) -> Option<&'a dyn Staff> {
        match side {
            Side::Allied => self.allied,
            Side::Japanese => self.japanese,
        }
    }
}

impl core::fmt::Debug for Staffs<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Staffs")
            .field("allied", &self.allied.is_some())
            .field("japanese", &self.japanese.is_some())
            .finish()
    }
}

/// Where a staff asked again first gave something other than what was
/// handed in (decision 0107, 10).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StaffDifference {
    pub day: u32,
    pub side: Side,
    /// Which part differs first, in words: "order 2", "command 1", "no-order
    /// reason 3", with what was handed in and what the staff gave.
    pub what: String,
}

/// A staff that hands in what was saved and asks `real` too, keeping the
/// first difference: what `replay --check-staff` runs. The day it resolves is
/// the saved day whatever `real` gives.
pub struct CheckedStaff<'a> {
    real: &'a dyn Staff,
    side: Side,
    day: u32,
    saved: StaffDecisions,
    first_difference: Mutex<Option<StaffDifference>>,
}

impl<'a> CheckedStaff<'a> {
    pub fn new(real: &'a dyn Staff, side: Side, day: u32, saved: StaffDecisions) -> Self {
        CheckedStaff {
            real,
            side,
            day,
            saved,
            first_difference: Mutex::new(None),
        }
    }

    /// The first difference found, once the day is resolved.
    pub fn difference(&self) -> Option<StaffDifference> {
        self.first_difference
            .lock()
            .map(|guard| guard.clone())
            .unwrap_or(None)
    }
}

impl Staff for CheckedStaff<'_> {
    fn decide(&self, view: SideView<'_>) -> StaffDecisions {
        let given = self.real.decide(view);
        if let Some(what) = first_difference(&self.saved, &given) {
            if let Ok(mut slot) = self.first_difference.lock() {
                if slot.is_none() {
                    *slot = Some(StaffDifference {
                        day: self.day,
                        side: self.side,
                        what,
                    });
                }
            }
        }
        self.saved.clone()
    }

    fn propose(&self, view: SideView<'_>, intent: IntentId) -> Option<Proposal> {
        self.real.propose(view, intent)
    }
}

/// The first part in which `given` differs from `saved`, in words, or
/// `None` if they are the same.
pub fn first_difference(saved: &StaffDecisions, given: &StaffDecisions) -> Option<String> {
    fn part<T: PartialEq + core::fmt::Debug>(
        name: &str,
        saved: &[T],
        given: &[T],
    ) -> Option<String> {
        for i in 0..saved.len().max(given.len()) {
            match (saved.get(i), given.get(i)) {
                (Some(a), Some(b)) if a == b => {}
                (a, b) => {
                    return Some(format!(
                        "{name} {}: saved {a:?}, the staff gave {b:?}",
                        i + 1
                    ))
                }
            }
        }
        None
    }
    part("order", &saved.orders, &given.orders)
        .or_else(|| part("command", &saved.commands, &given.commands))
        .or_else(|| part("no-order reason", &saved.no_orders, &given.no_orders))
}
