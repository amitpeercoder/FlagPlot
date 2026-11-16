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
//! Holds (a `hold` command, or an order of the commander's to a unit in an
//! intent's forces limit, the override at step 2 of the day) and the end of a
//! run are built (decision 0107, step 3 of stage 1). So are interrupts and
//! the staff's notices (step 4, rulings 31 to 35): the commander sets a
//! days-of-supply threshold, three kinds stand without being set, and each
//! is armed, fired and acknowledged; the staff's notices are recorded and
//! tested each morning after the staff has handed in and before the day's
//! orders (`intake::test_interrupts`).
//!
//! Reads: nothing of the truth but what intake checks against (the side's
//! own ships, forces and bases). Writes: the side's command state, and a
//! base's reserves through the reserve limit. Draws no random number.

pub(crate) mod hold;
pub(crate) mod intake;
pub(crate) mod interrupt;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Mutex;

use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{AirGroupId, BaseId, IntentId, InterruptId, OrderId, ShipId, TaskForceId};
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
/// question 5 as answered); step 2 of stage 2 adds covering a base from the
/// air (decision 0115) and holding a place (decision 0117); step 3 adds
/// neutralising a base and raiding one (decision 0120).
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
    /// Cover `base`, a base of the side, from the air (decision 0115): its
    /// fighters fly the patrol, its search groups search, by the doctrine.
    /// It has no date. A task: the staff forms it under "hold a place", and
    /// a commander may state it himself as a root. Added at the end.
    Cover { base: BaseId },
    /// Hold `base`, a base of the side (decision 0117, 12): an end. It has no
    /// date. Under it the staff may form a cover intent for the same base.
    /// Added at the end.
    Hold { base: BaseId },
    /// Neutralise `target`, a base on the side's map that is not the side's
    /// (decision 0120, 5): an end, addressed to the area. It has no date.
    /// Under it the staff forms a raid task for each base of the side with
    /// bombers that reach the target. Added at the end.
    Neutralise { target: BaseId },
    /// Raid `target` from `base`, a base of the side (decision 0120, 9): a
    /// task, addressed to `base`. It has no date. The staff forms it under
    /// "neutralise a base" naming the same target, and a commander may state
    /// it himself as a root. Added at the end.
    Raid { base: BaseId, target: BaseId },
    /// Deny the enemy the waters within `distance` hexes of `hex` (decision
    /// 0123, 1): an end, addressed to the area. It has no date. Under it the
    /// staff gives each base of the side in reach of those waters the tasks
    /// of striking ships and, where the base has no cover, of searching them.
    /// Added at the end.
    Deny { hex: Hex, distance: u32 },
    /// Strike the ships that come within reach of `base`'s groups, a base of
    /// the side (decision 0123, 5): a task, addressed to `base`. It has no
    /// date. It may stand under "deny an area" and under "hold a place" of
    /// the same base, and a commander may state it himself as a root. Added
    /// at the end.
    StrikeShips { base: BaseId },
    /// Search the waters of the "deny an area" above it from `base`, a base
    /// of the side (decision 0123, 11): a task, addressed to `base`. It has
    /// no date and stands under "deny an area" only, so a commander cannot
    /// state it as a root. Added at the end.
    SearchWaters { base: BaseId },
    /// Stand at `hex` with `force`, a task force of the side (decision 0124,
    /// 7): a task, addressed to the force. It has no date. It may stand under
    /// "deny an area", where the hex is the centre of the waters, and under
    /// "hold a place", where it is the base's hex; and a commander may state
    /// it himself as a root. Added at the end.
    StandAt { force: TaskForceId, hex: Hex },
    /// Operate `force`, a task force of the side with a carrier, from `hex`
    /// (decision 0125, 1): a task, addressed to the force. It has no date. It
    /// may stand under "deny an area" and under "hold a place", and a
    /// commander may state it himself as a root. Added at the end.
    OperateCarriers { force: TaskForceId, hex: Hex },
}

impl IntentKind {
    /// A stable number for the kind. Part of the state hash. Never renumber.
    pub const fn code(self) -> u8 {
        match self {
            IntentKind::KeepSupplied { .. } => 1,
            IntentKind::Cover { .. } => 2,
            IntentKind::Hold { .. } => 3,
            IntentKind::Neutralise { .. } => 4,
            IntentKind::Raid { .. } => 5,
            IntentKind::Deny { .. } => 6,
            IntentKind::StrikeShips { .. } => 7,
            IntentKind::SearchWaters { .. } => 8,
            IntentKind::StandAt { .. } => 9,
            IntentKind::OperateCarriers { .. } => 10,
        }
    }

    /// The kind's name as scenario files, saved games and the tools write it.
    pub const fn name(self) -> &'static str {
        match self {
            IntentKind::KeepSupplied { .. } => "keep_supplied",
            IntentKind::Cover { .. } => "cover",
            IntentKind::Hold { .. } => "hold",
            IntentKind::Neutralise { .. } => "neutralise",
            IntentKind::Raid { .. } => "raid",
            IntentKind::Deny { .. } => "deny",
            IntentKind::StrikeShips { .. } => "strike_ships",
            IntentKind::SearchWaters { .. } => "search_waters",
            IntentKind::StandAt { .. } => "stand_at",
            IntentKind::OperateCarriers { .. } => "operate_carriers",
        }
    }

    /// The base the intent names first, if it names one: the side's own
    /// base for every kind but "neutralise a base", whose one base is its
    /// target, the enemy's, and "deny an area", which names a hex.
    pub const fn base(self) -> Option<BaseId> {
        match self {
            IntentKind::KeepSupplied { base, .. }
            | IntentKind::Cover { base }
            | IntentKind::Hold { base }
            | IntentKind::Raid { base, .. }
            | IntentKind::StrikeShips { base }
            | IntentKind::SearchWaters { base } => Some(base),
            IntentKind::Neutralise { target } => Some(target),
            IntentKind::Deny { .. }
            | IntentKind::StandAt { .. }
            | IntentKind::OperateCarriers { .. } => None,
        }
    }

    /// The side's own base the intent names, if it names one: every kind but
    /// "neutralise a base", "deny an area" and the tasks of a task force.
    pub const fn own_base(self) -> Option<BaseId> {
        match self {
            IntentKind::Neutralise { .. }
            | IntentKind::Deny { .. }
            | IntentKind::StandAt { .. }
            | IntentKind::OperateCarriers { .. } => None,
            other => other.base(),
        }
    }

    /// The task force of the side the intent names, if it names one: a
    /// stand task's (decision 0124, 7) and a carrier task's (decision 0125,
    /// 1).
    pub const fn force(self) -> Option<TaskForceId> {
        match self {
            IntentKind::StandAt { force, .. } | IntentKind::OperateCarriers { force, .. } => {
                Some(force)
            }
            _ => None,
        }
    }

    /// The hex a task of a task force names: a stand task's station
    /// (decision 0124, 7) and the hex a carrier force operates from (decision
    /// 0125, 1).
    pub const fn station(self) -> Option<Hex> {
        match self {
            IntentKind::StandAt { hex, .. } | IntentKind::OperateCarriers { hex, .. } => Some(hex),
            _ => None,
        }
    }

    /// The waters of a "deny an area" intent (decision 0123, 1): its hex and
    /// its distance in hexes.
    pub const fn waters(self) -> Option<(Hex, u32)> {
        match self {
            IntentKind::Deny { hex, distance } => Some((hex, distance)),
            _ => None,
        }
    }

    /// Whether a commander may state an intent of this kind as a root
    /// (decision 0123, 11): every kind but "search waters", which stands
    /// under "deny an area" only.
    pub const fn may_be_root(self) -> bool {
        !matches!(self, IntentKind::SearchWaters { .. })
    }

    /// The enemy base the intent is aimed at, if any (decision 0120): the
    /// target of a neutralise or a raid intent.
    pub const fn target(self) -> Option<BaseId> {
        match self {
            IntentKind::Neutralise { target } | IntentKind::Raid { target, .. } => Some(target),
            _ => None,
        }
    }

    /// The level an intent of this kind is given to (decision 0117, 2): set
    /// by the kind, chosen by nobody. Keeping a base supplied and
    /// neutralising a base are the area's; holding a place, covering a base
    /// and raiding from one are the base's (decision 0120, 5 and 9).
    pub const fn addressee(self) -> Addressee {
        match self {
            IntentKind::KeepSupplied { .. }
            | IntentKind::Neutralise { .. }
            | IntentKind::Deny { .. } => Addressee::Area,
            IntentKind::Cover { base }
            | IntentKind::Hold { base }
            | IntentKind::Raid { base, .. }
            | IntentKind::StrikeShips { base }
            | IntentKind::SearchWaters { base } => Addressee::Base(base),
            // Decisions 0124, 6 and 7, and 0125, 1.
            IntentKind::StandAt { force, .. } | IntentKind::OperateCarriers { force, .. } => {
                Addressee::TaskForce(force)
            }
        }
    }

    /// Whether an intent of this kind may stand under one of `parent`'s kind
    /// (decision 0117, 3 and 12, and 0120, 9): cover from the air under
    /// holding the same base, and a raid under neutralising the same target.
    pub fn may_stand_under(self, parent: IntentKind) -> bool {
        match (self, parent) {
            (IntentKind::Cover { base }, IntentKind::Hold { base: held }) => base == held,
            (IntentKind::Raid { target, .. }, IntentKind::Neutralise { target: aimed }) => {
                target == aimed
            }
            // Decision 0123, 5 and 11.
            (
                IntentKind::StrikeShips { .. } | IntentKind::SearchWaters { .. },
                IntentKind::Deny { .. },
            ) => true,
            (IntentKind::StrikeShips { base }, IntentKind::Hold { base: held }) => base == held,
            // Decision 0124, 7: at the centre of the waters. Under a hold,
            // the hex is the base's, which intake checks.
            (IntentKind::StandAt { hex, .. }, IntentKind::Deny { hex: centre, .. }) => {
                hex == centre
            }
            // Decision 0125, 1.
            (
                IntentKind::StandAt { .. } | IntentKind::OperateCarriers { .. },
                IntentKind::Hold { .. },
            )
            | (IntentKind::OperateCarriers { .. }, IntentKind::Deny { .. }) => true,
            // Decision 0126, 7: a covering force stands at the run's
            // destination, the base's hex, which intake checks.
            (IntentKind::StandAt { .. }, IntentKind::KeepSupplied { .. }) => true,
            _ => false,
        }
    }
}

/// Who made an intent (decision 0117, 1): the commander, by `set_intent`, or
/// the staff, by `form_intent`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Maker {
    Commander,
    Staff,
}

impl Maker {
    /// A stable number. Part of the state hash. Never renumber.
    pub const fn code(self) -> u8 {
        match self {
            Maker::Commander => 1,
            Maker::Staff => 2,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Maker::Commander => "commander",
            Maker::Staff => "staff",
        }
    }
}

/// The level an intent is given to (decision 0117, 1 and 2). The level that
/// owns an intent is the kind of its addressee. A task force is added by
/// decision 0124; a land formation is added with the step that gives it
/// intents.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Addressee {
    /// The area: what it commands is named in a forces limit, or nothing.
    Area,
    /// A base of the side: it commands the side's air groups based there.
    Base(BaseId),
    /// An air group of the side, the level below a base: it decides the size
    /// of its own strikes from the task it is under (decision 0123, 9). No
    /// intent is addressed to it; a reason names it as the level that
    /// decided. Added at the end.
    AirGroup(AirGroupId),
    /// A task force of the side (decision 0124, 6): it commands its own ships
    /// and the air groups aboard them. Added at the end.
    TaskForce(TaskForceId),
}

impl Addressee {
    /// A stable number. Part of the state hash. Never renumber.
    pub const fn code(self) -> u8 {
        match self {
            Addressee::Area => 1,
            Addressee::Base(_) => 2,
            Addressee::AirGroup(_) => 3,
            Addressee::TaskForce(_) => 4,
        }
    }
}

impl core::fmt::Display for Addressee {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Addressee::Area => f.write_str("the area"),
            Addressee::Base(base) => write!(f, "{base}"),
            Addressee::AirGroup(group) => write!(f, "{group}"),
            Addressee::TaskForce(force) => write!(f, "{force}"),
        }
    }
}

/// The risk level of an intent (the spec's table). Read by nothing in stage
/// 1; the cover doctrine reads it (decision 0115, 4, 5 and 8). Ordered from
/// the most careful to the boldest, which is how a limit on a sub-intent
/// narrows (decision 0117, 7).
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
/// for it, the pool, the air groups it may use (decision 0115, 2), and for a
/// supply intent exactly one source base (decision 0107, 19). A supply
/// intent's forces limit must name a source; another kind's need not.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ForcesLimit {
    pub ships: BTreeSet<ShipId>,
    pub task_forces: BTreeSet<TaskForceId>,
    /// Air groups the staff may order for the intent. Added in step 2 of
    /// stage 2.
    pub air_groups: BTreeSet<AirGroupId>,
    pub source: Option<BaseId>,
}

/// The limits of one intent as they stand. A limit not set is `None` (or the
/// default). A limit the commander has not set does not bind: with no area
/// limit the sea is open (the architect's ruling, Nov 15, 2026). An intent
/// with no limit of a kind has its parent's, read up the tree to the root
/// (decision 0117, 7: [`CommandState::forces_read_up`] and the rest). With no
/// forces limit anywhere above it, an intent may use what its addressee
/// commands: for the area, nothing (decision 0117, 8).
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
    /// `None`: not set on this intent; read up the tree, and balanced where
    /// nothing above sets it (decision 0115, 4).
    pub risk: Option<Risk>,
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
    /// The intent it stands under, or `None` for a root (decision 0117, 1).
    pub parent: Option<IntentId>,
    /// Who made it (decision 0117, 1).
    pub maker: Maker,
    /// The level it is given to, set by its kind (decision 0117, 2).
    pub addressee: Addressee,
    pub limits: IntentLimits,
    /// The day it was first set.
    pub set_day: u32,
    /// The plan that serves it: for a root, drafted when it was first set;
    /// for a sub-intent, its root's (decision 0117, 9: one plan for a tree).
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
    /// The day a staff command ended the run (decision 0107, ruling 20);
    /// `None` while it has not ended. Nothing else follows from it: what is
    /// aboard stays aboard, and freeing commitments is a command of its own.
    pub ended: Option<u32>,
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

// ---------------------------------------------------------------------------
// Holds.
// ---------------------------------------------------------------------------

/// A setting of an air group a hold can keep from the staff.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum HeldSetting {
    /// The group's search arc (and its mission, which sets it flying).
    SearchArc,
}

impl HeldSetting {
    /// A stable number. Part of the state hash. Never renumber.
    pub const fn code(self) -> u8 {
        match self {
            HeldSetting::SearchArc => 1,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            HeldSetting::SearchArc => "search_arc",
        }
    }
}

/// What a hold keeps from the staff: a ship, a task force, a setting of an
/// air group (decision 0107, ruling 17), or a whole air group (decision 0115,
/// 3).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum HoldTarget {
    Ship(ShipId),
    TaskForce(TaskForceId),
    Setting {
        group: AirGroupId,
        setting: HeldSetting,
    },
    /// Every setting of the group: the staff gives it no order. Made by a
    /// `hold` command or by a commander's order to a group an intent may use.
    /// Added at the end.
    AirGroup(AirGroupId),
}

impl HoldTarget {
    /// A stable number. Part of the state hash. Never renumber.
    pub const fn code(self) -> u8 {
        match self {
            HoldTarget::Ship(_) => 1,
            HoldTarget::TaskForce(_) => 2,
            HoldTarget::Setting { .. } => 3,
            HoldTarget::AirGroup(_) => 4,
        }
    }
}

impl core::fmt::Display for HoldTarget {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            HoldTarget::Ship(ship) => write!(f, "{ship}"),
            HoldTarget::TaskForce(force) => write!(f, "{force}"),
            HoldTarget::Setting { group, setting } => match setting {
                HeldSetting::SearchArc => write!(f, "the search arc of {group}"),
            },
            HoldTarget::AirGroup(group) => write!(f, "{group}"),
        }
    }
}

/// Who made a hold: the commander's `hold` command, or an order of his (the
/// override, decision 0107, ruling 23).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HoldMaker {
    Command,
    Order(OrderId),
}

/// A hold: the day it was made and by what.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Hold {
    pub day: u32,
    pub by: HoldMaker,
}

// ---------------------------------------------------------------------------
// Interrupts.
// ---------------------------------------------------------------------------

/// What an interrupt the commander sets waits for (ruling 33). One kind in
/// stage 1.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum InterruptCondition {
    /// The days of supply of the side's base's hex are below `days`. It does
    /// not hold where the hex has no figure.
    SupplyBelow { base: BaseId, days: u32 },
}

impl InterruptCondition {
    /// A stable number. Part of the state hash. Never renumber.
    pub const fn code(self) -> u8 {
        match self {
            InterruptCondition::SupplyBelow { .. } => 1,
        }
    }
}

impl core::fmt::Display for InterruptCondition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            InterruptCondition::SupplyBelow { base, days } => {
                write!(f, "{base} below {days} days of supply")
            }
        }
    }
}

/// The kinds that stand without being set, until the commander switches them
/// off (ruling 33).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum StandingKind {
    /// The staff cannot meet a plan's intent (a notice of the staff's).
    CannotMeet,
    /// A plan waits for the commander's leave (a notice of the staff's).
    WaitsForLeave,
    /// A ship of the side was lost the day before.
    ShipLost,
}

impl StandingKind {
    /// Every kind, in code order.
    pub const ALL: [StandingKind; 3] = [
        StandingKind::CannotMeet,
        StandingKind::WaitsForLeave,
        StandingKind::ShipLost,
    ];

    /// A stable number. Part of the state hash. Never renumber.
    pub const fn code(self) -> u8 {
        match self {
            StandingKind::CannotMeet => 1,
            StandingKind::WaitsForLeave => 2,
            StandingKind::ShipLost => 3,
        }
    }

    /// The kind's name as scenario files, saved games and the tools write it.
    pub const fn name(self) -> &'static str {
        match self {
            StandingKind::CannotMeet => "cannot_meet",
            StandingKind::WaitsForLeave => "waits_for_leave",
            StandingKind::ShipLost => "ship_lost",
        }
    }

    /// The kind of that name.
    pub fn from_name(name: &str) -> Option<StandingKind> {
        StandingKind::ALL
            .into_iter()
            .find(|kind| kind.name() == name)
    }
}

/// What a standing interrupt is about: a plan (the staff's notices) or a ship
/// lost. A standing interrupt is armed apart for each subject (ruling 33).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum StandingSubject {
    Plan(PlanId),
    Ship(ShipId),
}

impl StandingSubject {
    /// A stable number. Part of the state hash. Never renumber.
    pub const fn code(self) -> u8 {
        match self {
            StandingSubject::Plan(_) => 1,
            StandingSubject::Ship(_) => 2,
        }
    }

    /// The subject's number in its own series.
    pub const fn raw(self) -> u32 {
        match self {
            StandingSubject::Plan(plan) => plan.raw(),
            StandingSubject::Ship(ship) => ship.raw(),
        }
    }
}

impl core::fmt::Display for StandingSubject {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            StandingSubject::Plan(plan) => write!(f, "{plan}"),
            StandingSubject::Ship(ship) => write!(f, "{ship}"),
        }
    }
}

/// An interrupt, set or standing, as a command or an event names it.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum InterruptRef {
    Set(InterruptId),
    Standing {
        kind: StandingKind,
        subject: StandingSubject,
    },
}

impl core::fmt::Display for InterruptRef {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            InterruptRef::Set(id) => write!(f, "{id}"),
            InterruptRef::Standing { kind, subject } => {
                write!(f, "the standing interrupt {} for {subject}", kind.name())
            }
        }
    }
}

/// Where an interrupt stands: armed or not, the day it last fired, whether
/// that firing is acknowledged (ruling 32). The default is armed, never
/// fired, not acknowledged.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct InterruptStatus {
    pub armed: bool,
    pub last_fired: Option<u32>,
    pub acknowledged: bool,
}

impl Default for InterruptStatus {
    fn default() -> InterruptStatus {
        InterruptStatus {
            armed: true,
            last_fired: None,
            acknowledged: false,
        }
    }
}

/// An interrupt the commander set.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SetInterrupt {
    pub id: InterruptId,
    pub condition: InterruptCondition,
    pub set_day: u32,
    pub status: InterruptStatus,
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
    /// What the commander holds from the staff (decision 0107, step 3).
    pub holds: BTreeMap<HoldTarget, Hold>,
    /// The number the side's next plan takes. Starts at 1.
    pub next_plan: u32,
    /// The interrupts the commander set, by his number (ruling 31).
    pub interrupts: BTreeMap<InterruptId, SetInterrupt>,
    /// Standing interrupts whose condition has held at least once; a subject
    /// absent is armed and never fired.
    pub standing: BTreeMap<(StandingKind, StandingSubject), InterruptStatus>,
    /// Standing kinds the commander has switched off.
    pub standing_off: BTreeSet<StandingKind>,
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

    /// The hold on `target`, if any.
    pub fn hold_of(&self, target: HoldTarget) -> Option<&Hold> {
        self.holds.get(&target)
    }

    /// The hold that keeps air group `group` from the staff: on the group
    /// itself, else on its search arc (which holds its mission too).
    pub fn hold_on_group(&self, group: AirGroupId) -> Option<HoldTarget> {
        [
            HoldTarget::AirGroup(group),
            HoldTarget::Setting {
                group,
                setting: HeldSetting::SearchArc,
            },
        ]
        .into_iter()
        .find(|target| self.holds.contains_key(target))
    }

    // -- The tree (decision 0117) -----------------------------------------

    /// The intents above `intent`, nearest first, ending at its root. Empty
    /// for a root or an intent that does not exist. A parent that is
    /// missing ends the walk; a loop, which intake cannot make, is cut where
    /// it would repeat.
    pub fn ancestors(&self, intent: IntentId) -> Vec<&Intent> {
        let mut found: Vec<&Intent> = Vec::new();
        let mut next = self.intents.get(&intent).and_then(|i| i.parent);
        while let Some(id) = next {
            let Some(parent) = self.intents.get(&id) else {
                break;
            };
            if parent.id == intent || found.iter().any(|seen| seen.id == parent.id) {
                break;
            }
            found.push(parent);
            next = parent.parent;
        }
        found
    }

    /// The root of `intent`'s tree: the last of its ancestors, or itself.
    pub fn root_of(&self, intent: IntentId) -> IntentId {
        self.ancestors(intent).last().map_or(intent, |root| root.id)
    }

    /// `intent` and the intents above it, nearest first.
    fn up_the_tree(&self, intent: IntentId) -> impl Iterator<Item = &Intent> + '_ {
        self.intents
            .get(&intent)
            .into_iter()
            .chain(self.ancestors(intent))
    }

    /// The forces limits on `intent`'s chain: its own and those set above
    /// it, nearest first (decision 0117, 7). Every one of them binds what is
    /// beneath it: the narrowest binds (the architect's ruling after the
    /// build). Empty where none is set on it or above it: then it may use
    /// what its addressee commands (rule 8).
    pub fn forces_on_chain(&self, intent: IntentId) -> Vec<&ForcesLimit> {
        self.up_the_tree(intent)
            .filter_map(|found| found.limits.forces.as_ref())
            .collect()
    }

    /// The nearest forces limit on `intent`'s chain, for words: what binds is
    /// every limit on the chain ([`CommandState::forces_on_chain`]). For a
    /// root, its own.
    pub fn nearest_forces(&self, intent: IntentId) -> Option<&ForcesLimit> {
        self.forces_on_chain(intent).into_iter().next()
    }

    /// The air groups the forces limits on `intent`'s chain allow: those
    /// named in every one of them. `None` where none is set on the chain.
    pub fn groups_on_chain(&self, intent: IntentId) -> Option<BTreeSet<AirGroupId>> {
        let mut limits = self.forces_on_chain(intent).into_iter();
        let first = limits.next()?.air_groups.clone();
        Some(limits.fold(first, |allowed, limit| {
            allowed.intersection(&limit.air_groups).copied().collect()
        }))
    }

    /// The risk level of `intent` read up the tree: the least bold of those
    /// set on it and above it, so that every risk on the chain binds (the
    /// architect's ruling after the build); balanced where none is set
    /// (decision 0115, 4).
    pub fn risk_read_up(&self, intent: IntentId) -> Risk {
        self.up_the_tree(intent)
            .filter_map(|found| found.limits.risk)
            .min()
            .unwrap_or_default()
    }

    /// The rank of `intent` read up the tree; `None` where none is set on it
    /// or above it.
    pub fn priority_read_up(&self, intent: IntentId) -> Option<u32> {
        self.up_the_tree(intent)
            .find_map(|found| found.limits.priority)
    }

    /// The area of `intent` read up the tree: the hexes in every area set on
    /// it and above it (the architect's ruling after the build); `None` where
    /// none is set on the chain.
    pub fn area_read_up(&self, intent: IntentId) -> Option<BTreeSet<Hex>> {
        let mut areas = self
            .up_the_tree(intent)
            .filter_map(|found| found.limits.area.as_ref());
        let first = areas.next()?.clone();
        Some(areas.fold(first, |kept, area| {
            kept.intersection(area).copied().collect()
        }))
    }

    /// The intents in force directly under `intent`, by number.
    pub fn children_in_force(&self, intent: IntentId) -> Vec<IntentId> {
        self.intents_in_force()
            .filter(|found| found.parent == Some(intent))
            .map(|found| found.id)
            .collect()
    }

    /// Every intent in force beneath `intent`, depth first, each branch by
    /// number.
    pub fn descendants_in_force(&self, intent: IntentId) -> Vec<IntentId> {
        let mut found = Vec::new();
        let mut stack: Vec<IntentId> = self.children_in_force(intent).into_iter().rev().collect();
        while let Some(next) = stack.pop() {
            if next == intent || found.contains(&next) {
                continue;
            }
            found.push(next);
            stack.extend(self.children_in_force(next).into_iter().rev());
        }
        found
    }

    /// The number a staff-formed intent takes: one above the highest the
    /// side has ever used (decision 0117, 3).
    pub fn next_intent_number(&self) -> IntentId {
        IntentId::new(
            self.intents
                .keys()
                .next_back()
                .map_or(1, |last| last.raw() + 1),
        )
    }

    /// The hold that keeps a force from the staff: on the force itself, else
    /// on the first of its `ships` (in ship order) that is held.
    pub fn hold_on_force(
        &self,
        force: TaskForceId,
        ships: &BTreeSet<ShipId>,
    ) -> Option<HoldTarget> {
        let whole = HoldTarget::TaskForce(force);
        if self.holds.contains_key(&whole) {
            return Some(whole);
        }
        ships
            .iter()
            .map(|ship| HoldTarget::Ship(*ship))
            .find(|target| self.holds.contains_key(target))
    }

    /// The hold that keeps a ship from the staff (ruling 18): on the ship,
    /// else, where it lies in a force (given with the force's ships), on that
    /// force or on any ship of it.
    pub fn hold_on_ship(
        &self,
        ship: ShipId,
        force: Option<(TaskForceId, &BTreeSet<ShipId>)>,
    ) -> Option<HoldTarget> {
        let own = HoldTarget::Ship(ship);
        if self.holds.contains_key(&own) {
            return Some(own);
        }
        force.and_then(|(force, ships)| self.hold_on_force(force, ships))
    }

    /// Whether the command state is empty: nothing was ever set. A side
    /// with an empty command state is played exactly as before stage 1.
    pub fn is_empty(&self) -> bool {
        self.intents.is_empty()
            && self.plans.is_empty()
            && self.commitments.is_empty()
            && self.holds.is_empty()
            && self.interrupts.is_empty()
            && self.standing.is_empty()
            && self.standing_off.is_empty()
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
    /// The commander's: hold a ship, a force or a setting of an air group
    /// from the staff. Added at the end (decision 0107, step 3).
    Hold { target: HoldTarget },
    /// The commander's: release a hold, whatever made it.
    Release { target: HoldTarget },
    /// The staff's: end a run of an approved plan (ruling 20).
    EndRun { plan: PlanId, run: u32 },
    /// The commander's: set an interrupt, or switch a standing kind off or
    /// on (rulings 31 and 33). Added at the end (decision 0107, step 4).
    SetInterrupt(InterruptSetting),
    /// The commander's: mark an interrupt's last firing seen. It arms
    /// nothing (ruling 32). Added at the end.
    Acknowledge { interrupt: InterruptRef },
    /// The staff's: form a sub-intent of `kind` under `parent` (decision
    /// 0117, 3). It takes the number one above the highest the side has
    /// used, its maker is the staff, its plan its root's. Added at the end.
    FormIntent { parent: IntentId, kind: IntentKind },
}

/// What a `set_interrupt` command sets.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InterruptSetting {
    /// Set interrupt `interrupt` to `condition`, or set it again: a new
    /// record, armed, never fired, set today.
    Set {
        interrupt: InterruptId,
        condition: InterruptCondition,
    },
    /// Switch a standing kind off or on. Switching it to the state it is in
    /// is accepted and changes nothing.
    Standing { kind: StandingKind, on: bool },
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
            | CommandKind::CancelPlan { .. }
            | CommandKind::Hold { .. }
            | CommandKind::Release { .. }
            | CommandKind::SetInterrupt(_)
            | CommandKind::Acknowledge { .. } => issuer == Issuer::Player,
            CommandKind::Approve { .. } => {
                matches!(issuer, Issuer::Player | Issuer::Staff)
            }
            CommandKind::CommitShips { .. }
            | CommandKind::FreeShips { .. }
            | CommandKind::AddRun { .. }
            | CommandKind::TimeRun { .. }
            | CommandKind::MarkPlan { .. }
            | CommandKind::EndRun { .. }
            | CommandKind::FormIntent { .. } => issuer == Issuer::Staff,
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
            CommandKind::Hold { .. } => "hold",
            CommandKind::Release { .. } => "release",
            CommandKind::EndRun { .. } => "end_run",
            CommandKind::SetInterrupt(_) => "set_interrupt",
            CommandKind::Acknowledge { .. } => "acknowledge",
            CommandKind::FormIntent { .. } => "form_intent",
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
    /// An air group is not one of the side's own (or does not exist).
    NotYourGroup(AirGroupId),
    /// The target is held already.
    AlreadyHeld(HoldTarget),
    /// The target is not held.
    NotHeld(HoldTarget),
    /// The run has ended.
    RunEnded(u32),
    /// The ship is held by the commander, itself or through its force.
    Held(HoldTarget),
    /// The interrupt's number is 0. Added at the end (decision 0107, step 4).
    InterruptNumberNotUsable(InterruptId),
    /// No interrupt of that number was set.
    NoSuchInterrupt(InterruptId),
    /// The interrupt has never fired.
    NotFired(InterruptRef),
    /// The interrupt's last firing is acknowledged already.
    AlreadyAcknowledged(InterruptRef),
    /// The intent was made by the staff: the commander shapes it by its
    /// limits, holds and parent, and may not set or withdraw it (decision
    /// 0117, 5). Added at the end (step 2 of stage 2).
    MadeByStaff(IntentId),
    /// A limit on a sub-intent is wider than what its parent allows, read up
    /// the tree: a force or group the parent may not use, or a risk bolder
    /// than the parent's (decision 0117, 7).
    WiderThanParent(IntentId),
    /// The kind cannot stand under the parent's kind (decision 0117, 3).
    CannotStandUnder(IntentId),
    /// A supply intent's forces limit names no source base (decision 0107,
    /// 19).
    NoSource,
    /// A `set_intent` on an intent in force names another kind of intent: it
    /// may change the intent's figures and not its kind (the architect's
    /// ruling after the build of decision 0117). He withdraws it and states
    /// another. Added at the end.
    KindCannotChange(IntentId),
    /// The target of a neutralise or a raid intent is not a base on the
    /// side's map that is not the side's (decision 0120, 5 and 9). Added at
    /// the end.
    NotAnEnemyBase(BaseId),
    /// The hex of a "deny an area" intent is not on the map (decision 0123,
    /// 1). Added at the end.
    NotOnMap(Hex),
    /// A kind a commander may not state as a root: "search waters" stands
    /// under "deny an area" only (decision 0123, 11). Added at the end.
    NotARoot,
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
            CommandRefusal::NotYourGroup(group) => {
                write!(f, "{group} is not one of the side's air groups")
            }
            CommandRefusal::AlreadyHeld(target) => {
                write!(f, "{target} is already held by the commander")
            }
            CommandRefusal::NotHeld(target) => write!(f, "{target} is not held"),
            CommandRefusal::RunEnded(run) => write!(f, "run {run} has already ended"),
            CommandRefusal::Held(target) => write!(f, "{target} is held by the commander"),
            CommandRefusal::InterruptNumberNotUsable(id) => {
                write!(f, "{id} is not a number an interrupt may take")
            }
            CommandRefusal::NoSuchInterrupt(id) => write!(f, "{id} was not set"),
            CommandRefusal::NotFired(interrupt) => write!(f, "{interrupt} has not fired"),
            CommandRefusal::AlreadyAcknowledged(interrupt) => {
                write!(f, "the last firing of {interrupt} is acknowledged already")
            }
            CommandRefusal::MadeByStaff(id) => write!(
                f,
                "{id} was formed by the staff: the commander shapes it by its limits, holds and parent"
            ),
            CommandRefusal::WiderThanParent(parent) => {
                write!(f, "the limit is wider than what its parent {parent} allows")
            }
            CommandRefusal::CannotStandUnder(parent) => {
                write!(f, "an intent of that kind cannot stand under {parent}")
            }
            CommandRefusal::NoSource => {
                f.write_str("a supply intent's forces limit must name a source base")
            }
            CommandRefusal::KindCannotChange(id) => write!(
                f,
                "{id} is another kind of intent: withdraw it and state a new one"
            ),
            CommandRefusal::NotAnEnemyBase(base) => write!(
                f,
                "{base} is not a base on the side's map that is not the side's"
            ),
            CommandRefusal::NotOnMap(hex) => write!(f, "{hex} is not on the map"),
            CommandRefusal::NotARoot => f.write_str(
                "this kind of intent stands only under an end the staff serves, and the staff forms it",
            ),
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

/// A count with its noun in agreement: "1 ship", "2 ships".
struct Count(usize, &'static str, &'static str);

impl core::fmt::Display for Count {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Count(n, one, many) = *self;
        write!(f, "{n} {}", if n == 1 { one } else { many })
    }
}

impl core::fmt::Display for CommandKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            CommandKind::SetIntent { intent, kind } => {
                write!(f, "set {intent} to ")?;
                write_intent_kind(f, *kind)
            }
            CommandKind::WithdrawIntent { intent } => write!(f, "withdraw {intent}"),
            CommandKind::SetLimit(limit) => match limit {
                Limit::Forces { intent, limit } => {
                    write!(
                        f,
                        "limit {intent}'s forces to {} and {}",
                        Count(limit.ships.len(), "ship", "ships"),
                        Count(limit.task_forces.len(), "task force", "task forces"),
                    )?;
                    if !limit.air_groups.is_empty() {
                        write!(
                            f,
                            " and {}",
                            Count(limit.air_groups.len(), "air group", "air groups")
                        )?;
                    }
                    match limit.source {
                        Some(source) => write!(f, " from {source}"),
                        None => Ok(()),
                    }
                }
                Limit::Area { intent, hexes } => {
                    write!(
                        f,
                        "limit {intent}'s area to {}",
                        Count(hexes.len(), "hex", "hexes")
                    )
                }
                Limit::Priority { intent, rank } => match rank {
                    Some(rank) => write!(f, "rank {intent} at {rank}"),
                    None => write!(f, "leave {intent} unranked"),
                },
                Limit::Protected { intent, ships } => {
                    write!(
                        f,
                        "protect {} of {intent}",
                        Count(ships.len(), "ship", "ships")
                    )
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
                write!(
                    f,
                    "commit {} to {plan}",
                    Count(ships.len(), "ship", "ships")
                )?;
                if let Some(run) = run {
                    write!(f, " run {run}")?;
                }
                write!(f, " from day {from_day} to day {to_day}")
            }
            CommandKind::FreeShips { plan, ships } => {
                write!(
                    f,
                    "free {} from {plan}",
                    Count(ships.len(), "ship", "ships")
                )
            }
            CommandKind::AddRun { plan, run } => write!(
                f,
                "add a run of {} t with {} to {plan}, loading day {}, due day {}",
                run.load_t,
                Count(run.ships.len(), "ship", "ships"),
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
            CommandKind::Hold { target } => write!(f, "hold {target}"),
            CommandKind::Release { target } => write!(f, "release {target}"),
            CommandKind::EndRun { plan, run } => write!(f, "end run {run} of {plan}"),
            CommandKind::SetInterrupt(setting) => match setting {
                InterruptSetting::Set {
                    interrupt,
                    condition,
                } => write!(f, "set {interrupt}: {condition}"),
                InterruptSetting::Standing { kind, on } => write!(
                    f,
                    "switch {} the standing interrupt {}",
                    if *on { "on" } else { "off" },
                    kind.name()
                ),
            },
            CommandKind::Acknowledge { interrupt } => match interrupt {
                InterruptRef::Set(id) => write!(f, "acknowledge {id}"),
                InterruptRef::Standing { kind, subject } => write!(
                    f,
                    "acknowledge the standing interrupt {} for {subject}",
                    kind.name()
                ),
            },
            CommandKind::FormIntent { parent, kind } => {
                write!(f, "form under {parent} an intent to ")?;
                write_intent_kind(f, *kind)
            }
        }
    }
}

/// An intent's kind as a command's words give it: "keep base:2 supplied at
/// 30 days, a run every 15 days", "cover base:1 from the air", "hold
/// base:1".
fn write_intent_kind(f: &mut core::fmt::Formatter<'_>, kind: IntentKind) -> core::fmt::Result {
    match kind {
        IntentKind::KeepSupplied {
            base,
            floor_days,
            interval_days,
        } => {
            write!(f, "keep {base} supplied at {floor_days} days")?;
            match interval_days {
                Some(days) => write!(f, ", a run every {days} days"),
                None => Ok(()),
            }
        }
        IntentKind::Cover { base } => write!(f, "cover {base} from the air"),
        IntentKind::Hold { base } => write!(f, "hold {base}"),
        IntentKind::Neutralise { target } => write!(f, "neutralise {target}"),
        IntentKind::Raid { base, target } => write!(f, "raid {target} from {base}"),
        IntentKind::Deny { hex, distance } => {
            write!(f, "deny the waters within {distance} hexes of {hex}")
        }
        IntentKind::StrikeShips { base } => write!(f, "strike ships from {base}"),
        IntentKind::SearchWaters { base } => write!(f, "search the waters from {base}"),
        IntentKind::StandAt { force, hex } => write!(f, "stand at {hex} with {force}"),
        IntentKind::OperateCarriers { force, hex } => {
            write!(f, "operate the carrier force {force} from {hex}")
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

/// What a staff decision serves: the intent, and the plan and amendment,
/// with the intent in words.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Serves {
    pub intent: Option<IntentId>,
    pub plan: Option<PlanId>,
    /// The plan's amendment it was made under; `None` for the baseline.
    pub amendment: Option<u32>,
    /// The intent as the commander stated it, in words, as it stood when the
    /// reason was made ("keep base:2 (Placeholder base T) at 30 days of
    /// supply, a run every 15 days"). The staff hands it in with the reason,
    /// so the reason still says it after the intent is replaced or withdrawn
    /// (decision 0107, item 12). Empty where a reason has none, as in a game
    /// saved before it was handed in. The engine never reads it.
    pub statement: String,
    /// The intents above `intent`, nearest first, each as it stood when the
    /// reason was made (decision 0117, 10). The last is one the commander
    /// stated. Empty where the intent is a root. The engine never reads it.
    pub chain: Vec<ChainLink>,
    /// The level that decided (decision 0120, 2): the addressee of the
    /// intent served. `None` where a reason has none, as in a game saved
    /// before it was handed in. The engine never reads it.
    pub level: Option<Addressee>,
}

/// One intent of a reason's chain: its number, its maker and its statement
/// in words as it stood.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ChainLink {
    pub intent: IntentId,
    pub maker: Maker,
    pub statement: String,
}

/// "intent 1: keep base:2 (Placeholder base T) at 30 days of supply, a run
/// every 15 days; plan 1, amendment 1", each part only where there is one,
/// then the level that decided, "; decided at the level of base:1", then the
/// chain: "; under intent 1 (the commander's): hold base:1".
impl core::fmt::Display for Serves {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut gap = "";
        if let Some(intent) = self.intent {
            write!(f, "intent {}", intent.raw())?;
            gap = "; ";
        }
        if !self.statement.is_empty() {
            if self.intent.is_some() {
                write!(f, ": ")?;
            }
            write!(f, "{}", self.statement)?;
            gap = "; ";
        }
        if let Some(plan) = self.plan {
            write!(f, "{gap}plan {}", plan.raw())?;
            gap = ", ";
        }
        if let Some(amendment) = self.amendment {
            write!(f, "{gap}amendment {amendment}")?;
            gap = "; ";
        }
        if let Some(level) = self.level {
            let before = if gap.is_empty() { "" } else { "; " };
            write!(f, "{before}decided at the level of {level}")?;
            gap = "; ";
        }
        for link in &self.chain {
            write!(
                f,
                "{gap}under intent {} (the {}'s)",
                link.intent.raw(),
                link.maker.name()
            )?;
            if !link.statement.is_empty() {
                write!(f, ": {}", link.statement)?;
            }
            gap = "; ";
        }
        Ok(())
    }
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

/// What a staff notice says (decision 0107, rulings 34 and 35).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum NoticeKind {
    /// The staff cannot meet the plan's intent: it needs a run today and can
    /// have none, or the run it can have leaves the reading it checks below
    /// the floor.
    CannotMeet,
    /// The plan waits for the commander's leave (to prepare or to launch).
    WaitsForLeave(Leave),
    /// The staff has ended a run of the plan.
    RunEnded,
}

impl NoticeKind {
    /// A stable number. Never renumber; add new values at the end.
    pub const fn code(self) -> u8 {
        match self {
            NoticeKind::CannotMeet => 1,
            NoticeKind::WaitsForLeave(Leave::Prepare) => 2,
            NoticeKind::WaitsForLeave(Leave::Launch) => 3,
            NoticeKind::RunEnded => 4,
        }
    }

    /// The kind's name as saved games and the tools write it.
    pub const fn name(self) -> &'static str {
        match self {
            NoticeKind::CannotMeet => "cannot_meet",
            NoticeKind::WaitsForLeave(Leave::Prepare) => "waits_for_leave_to_prepare",
            NoticeKind::WaitsForLeave(Leave::Launch) => "waits_for_leave_to_launch",
            NoticeKind::RunEnded => "run_ended",
        }
    }

    /// The kind of that name.
    pub fn from_name(name: &str) -> Option<NoticeKind> {
        [
            NoticeKind::CannotMeet,
            NoticeKind::WaitsForLeave(Leave::Prepare),
            NoticeKind::WaitsForLeave(Leave::Launch),
            NoticeKind::RunEnded,
        ]
        .into_iter()
        .find(|kind| kind.name() == name)
    }
}

/// The reason a notice rests on: an index into the same hand-in's
/// `commands` or `no_orders`. The engine records the notice citing that
/// reason's `StaffReason` event; it never reads the reason's words.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NoticeBasis {
    Command(usize),
    NoOrder(usize),
}

/// A structured notice the staff hands in beside its orders, commands and
/// reasons (decision 0107, ruling 34): the plan, the run where there is one,
/// the kind and the reason it rests on. The engine records each as a
/// `StaffNotice` event and tests the standing interrupts on them.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct StaffNotice {
    pub kind: NoticeKind,
    pub plan: PlanId,
    pub run: Option<u32>,
    pub rests_on: NoticeBasis,
}

/// What a staff hands in for one day (decision 0107, 4): orders with their
/// reasons, staff commands, decisions to give no order with theirs, and
/// notices (ruling 34). It is what a saved game keeps for the day, and what
/// a replay hands in again without asking the staff.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct StaffDecisions {
    pub orders: Vec<StaffOrder>,
    pub commands: Vec<StaffCommand>,
    pub no_orders: Vec<Reason>,
    pub notices: Vec<StaffNotice>,
}

impl StaffDecisions {
    /// Adds `other` after what is already here: its orders, commands and
    /// decisions to give no order follow ours, and each of its notices has
    /// its basis shifted by the lengths of ours, so that it still points at
    /// its own reason (ruling 34).
    pub fn append(&mut self, other: StaffDecisions) {
        let commands = self.commands.len();
        let no_orders = self.no_orders.len();
        self.orders.extend(other.orders);
        self.commands.extend(other.commands);
        self.no_orders.extend(other.no_orders);
        self.notices
            .extend(other.notices.into_iter().map(|mut notice| {
                notice.rests_on = match notice.rests_on {
                    NoticeBasis::Command(i) => NoticeBasis::Command(i + commands),
                    NoticeBasis::NoOrder(i) => NoticeBasis::NoOrder(i + no_orders),
                };
                notice
            }));
    }

    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
            && self.commands.is_empty()
            && self.no_orders.is_empty()
            && self.notices.is_empty()
    }
}

/// A staff: given a side's view, it decides the day. It is a function of the
/// view: `&self`, so that asked twice on the same view it gives the same
/// answer, which `replay --check-staff` tests. It is asked once a day, at
/// step 4 of the day's order, after the commander's commands are taken in
/// and forces back from the yard are home, and before any order is taken in
/// (ships in port take fuel in the pulses, not before this step: decision
/// 0112).
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
        .or_else(|| part("notice", &saved.notices, &given.notices))
}
