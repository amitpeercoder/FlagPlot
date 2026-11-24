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

use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Mutex;

use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{
    AirGroupId, BaseId, EntityRef, EventId, IntentId, InterruptId, LandUnitId, OrderId, ShipId,
    SubmarineId, TaskForceId,
};
use crate::orders::{Issuer, OrderKind};
use crate::refdata::ShipKind;
use crate::state::Side;
use crate::time::Pulse;
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
    /// date. Under it the staff may form a cover intent for the same base,
    /// and "keep supplied" for it (decision 0144, 7). It may name a base on
    /// the side's map that is not the side's, and then waits for it to be
    /// the side's (decision 0144, 1). Added at the end.
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
    /// Ready a landing by `force`, a task force of the side, at `hex`, a
    /// coast hex (decision 0141, 1): a task, addressed to the force. The
    /// land units that are to land are named in its forces limit. `by_day`
    /// is the morning by which it is to be ready, or `None`. A commander may
    /// state it himself as a root. Added at the end.
    ReadyLanding {
        force: TaskForceId,
        hex: Hex,
        by_day: Option<u32>,
    },
    /// Build up `base`, a base of the side, with what is wanted there, in
    /// order (decision 0141, 8): a task, addressed to the base. It has no
    /// date. A commander may state it himself as a root. Added at the end.
    BuildUp { base: BaseId, wants: Wants },
    /// Take the place at `hex`, a hex of land on the map (decision 0142, 1):
    /// an end, addressed to the area. The commander names it by its hex or by
    /// a base on the side's map that is not the side's, which means that
    /// base's hex. `by_day` is the day by the end of which the place is to be
    /// the side's, or `None`. Its forces limit names the task forces, the land
    /// units and a source (item 2). It is met when the side holds a base in
    /// the hex and the enemy holds none there (item 3). Under it the staff
    /// forms "ready a landing" (item 12). Added at the end.
    TakePlace { hex: Hex, by_day: Option<u32> },
    /// Bombard `target`, a base on the side's map that is not the side's,
    /// with `force`, a task force of the side on the bombardment mission
    /// (decision 0146, 1): a task, addressed to the force. It has no date.
    /// The staff forms it under "neutralise a base" naming the same target
    /// and under "take a place" whose hex is the target's, and a commander may
    /// state it himself as a root. Added at the end.
    Bombard { force: TaskForceId, target: BaseId },
    /// Control `island`, an island the scenario names (decision 0151, 1 and
    /// 3): an end, addressed to the area, and a root. `by_day` is the day
    /// each "take a place" the staff forms under it is to be met by, or
    /// `None`. Under it the staff forms "take a place", "hold a place",
    /// "deny an area" and "neutralise a base" (item 6). It is never marked
    /// complete (item 12). Added at the end.
    ControlIsland { island: u32, by_day: Option<u32> },
    /// Patrol the waters within `distance` hexes of `hex` with submarines
    /// (decision 0218, 11 and 12): an end, addressed to the area. It has no
    /// date. A commander states it as a root; the staff cannot form it and
    /// nothing stands under it. Added at the end.
    Patrol { hex: Hex, distance: u32 },
    /// Destroy the force of contact `contact` of the commander's picture
    /// (decision 0220, 1): a deny whose centre is read from the force it
    /// follows, within `distance` hexes of it. Addressed to the area, and a
    /// root. With `search_only` it forms the tasks to search waters and no
    /// other. The staff never forms it. The centre is not in the kind: it is
    /// the hex the order keeps ([`Followed`]). Added at the end.
    DestroyForce {
        contact: u32,
        distance: u32,
        search_only: bool,
    },
}

/// One thing a "build up a base" task wants (decision 0141, 8 and 10).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Want {
    /// An airfield: met when the base has one.
    Airfield,
    /// Its airfield's damage mended: met when the field has no damage.
    Mended,
    /// Defences to this many points, from 1 to 100: met at that or above.
    Defences(u32),
    /// The port worked: never met.
    Port,
    /// Aircraft repair worked: never met.
    AircraftRepair,
}

impl Want {
    /// A stable number for the kind of want. Part of the state hash. Never
    /// renumber.
    pub const fn code(self) -> u8 {
        match self {
            Want::Airfield => 1,
            Want::Mended => 2,
            Want::Defences(_) => 3,
            Want::Port => 4,
            Want::AircraftRepair => 5,
        }
    }

    /// The want's name as scenario files, saved games and the tools write
    /// it. Defences carry their points beside the name.
    pub const fn name(self) -> &'static str {
        match self {
            Want::Airfield => "airfield",
            Want::Mended => "mend",
            Want::Defences(_) => "defences",
            Want::Port => "port",
            Want::AircraftRepair => "aircraft",
        }
    }

    /// The want of that name; `points` is read for defences only.
    pub fn from_name(name: &str, points: Option<u32>) -> Option<Want> {
        match name {
            "airfield" => Some(Want::Airfield),
            "mend" => Some(Want::Mended),
            "defences" => points.map(Want::Defences),
            "port" => Some(Want::Port),
            "aircraft" => Some(Want::AircraftRepair),
            _ => None,
        }
    }
}

impl core::fmt::Display for Want {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Want::Airfield => f.write_str("an airfield"),
            Want::Mended => f.write_str("its airfield's damage mended"),
            Want::Defences(points) => write!(f, "defences to {points} points"),
            Want::Port => f.write_str("the port worked"),
            Want::AircraftRepair => f.write_str("aircraft repair"),
        }
    }
}

/// What a "build up a base" task wants, in order (decision 0141, 8): one to
/// five wants, each kind at most once, defences from 1 to 100 points. Made
/// only by [`Wants::new`], which refuses anything else, so every value is
/// one intake may take.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Wants {
    items: [Option<Want>; 5],
}

impl Wants {
    /// The wants in the order given, or `None` if the list is empty, names
    /// a kind twice, or has defences outside 1 to 100.
    pub fn new(list: &[Want]) -> Option<Wants> {
        if list.is_empty() || list.len() > 5 {
            return None;
        }
        let mut items = [None; 5];
        for (i, want) in list.iter().enumerate() {
            if list[..i].iter().any(|seen| seen.code() == want.code()) {
                return None;
            }
            if let Want::Defences(points) = want {
                if !(1..=100).contains(points) {
                    return None;
                }
            }
            items[i] = Some(*want);
        }
        Some(Wants { items })
    }

    /// The wants, in order.
    pub fn iter(&self) -> impl Iterator<Item = Want> + '_ {
        self.items.iter().filter_map(|want| *want)
    }
}

impl core::fmt::Display for Wants {
    /// "an airfield, then defences to 10 points".
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for (i, want) in self.iter().enumerate() {
            if i > 0 {
                f.write_str(", then ")?;
            }
            write!(f, "{want}")?;
        }
        Ok(())
    }
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
            IntentKind::ReadyLanding { .. } => 11,
            IntentKind::BuildUp { .. } => 12,
            IntentKind::TakePlace { .. } => 13,
            IntentKind::Bombard { .. } => 14,
            IntentKind::ControlIsland { .. } => 15,
            IntentKind::Patrol { .. } => 16,
            IntentKind::DestroyForce { .. } => 17,
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
            IntentKind::ReadyLanding { .. } => "ready_landing",
            IntentKind::BuildUp { .. } => "build_up",
            IntentKind::TakePlace { .. } => "take_place",
            IntentKind::Bombard { .. } => "bombard",
            IntentKind::ControlIsland { .. } => "control_island",
            IntentKind::Patrol { .. } => "patrol",
            IntentKind::DestroyForce { .. } => "destroy_force",
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
            | IntentKind::SearchWaters { base }
            | IntentKind::BuildUp { base, .. } => Some(base),
            IntentKind::Neutralise { target } | IntentKind::Bombard { target, .. } => Some(target),
            IntentKind::Deny { .. }
            | IntentKind::Patrol { .. }
            | IntentKind::DestroyForce { .. }
            | IntentKind::StandAt { .. }
            | IntentKind::OperateCarriers { .. }
            | IntentKind::ReadyLanding { .. }
            | IntentKind::TakePlace { .. }
            | IntentKind::ControlIsland { .. } => None,
        }
    }

    /// The side's own base the intent names, if it names one: every kind but
    /// "neutralise a base", "deny an area" and the tasks of a task force.
    pub const fn own_base(self) -> Option<BaseId> {
        match self {
            IntentKind::Neutralise { .. }
            | IntentKind::Deny { .. }
            | IntentKind::Patrol { .. }
            | IntentKind::DestroyForce { .. }
            | IntentKind::StandAt { .. }
            | IntentKind::OperateCarriers { .. }
            | IntentKind::ReadyLanding { .. }
            | IntentKind::TakePlace { .. }
            | IntentKind::Bombard { .. }
            | IntentKind::ControlIsland { .. } => None,
            other => other.base(),
        }
    }

    /// The task force of the side the intent names, if it names one: a
    /// stand task's (decision 0124, 7), a carrier task's (decision 0125, 1),
    /// a landing's (decision 0141, 1) and a bombarding force's (decision
    /// 0146, 1).
    pub const fn force(self) -> Option<TaskForceId> {
        match self {
            IntentKind::StandAt { force, .. }
            | IntentKind::OperateCarriers { force, .. }
            | IntentKind::ReadyLanding { force, .. }
            | IntentKind::Bombard { force, .. } => Some(force),
            _ => None,
        }
    }

    /// The coast hex of a "ready a landing" task (decision 0141, 1).
    pub const fn landing(self) -> Option<Hex> {
        match self {
            IntentKind::ReadyLanding { hex, .. } => Some(hex),
            _ => None,
        }
    }

    /// The hex of land a "take a place" end names (decision 0142, 1).
    pub const fn place(self) -> Option<Hex> {
        match self {
            IntentKind::TakePlace { hex, .. } => Some(hex),
            _ => None,
        }
    }

    /// The island a "control an island" end names (decision 0151, 3).
    pub const fn island(self) -> Option<u32> {
        match self {
            IntentKind::ControlIsland { island, .. } => Some(island),
            _ => None,
        }
    }

    /// Whether the kind is one of the two tasks that order land units
    /// (decision 0141): ready a landing, and build up a base.
    pub const fn readies_units(self) -> bool {
        matches!(
            self,
            IntentKind::ReadyLanding { .. } | IntentKind::BuildUp { .. }
        )
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

    /// The waters of a "patrol" intent (decision 0218, 11): its hex and its
    /// distance in hexes.
    pub const fn patrol_waters(self) -> Option<(Hex, u32)> {
        match self {
            IntentKind::Patrol { hex, distance } => Some((hex, distance)),
            _ => None,
        }
    }

    /// What an order to destroy a force names (decision 0220): the contact's
    /// number, the distance and whether it is search only.
    pub const fn followed(self) -> Option<(u32, u32, bool)> {
        match self {
            IntentKind::DestroyForce {
                contact,
                distance,
                search_only,
            } => Some((contact, distance, search_only)),
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
    /// target of a neutralise or a raid intent, and of a bombardment
    /// (decision 0146, 1).
    pub const fn target(self) -> Option<BaseId> {
        match self {
            IntentKind::Neutralise { target }
            | IntentKind::Raid { target, .. }
            | IntentKind::Bombard { target, .. } => Some(target),
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
            | IntentKind::Deny { .. }
            | IntentKind::Patrol { .. }
            | IntentKind::DestroyForce { .. }
            | IntentKind::TakePlace { .. }
            | IntentKind::ControlIsland { .. } => Addressee::Area,
            IntentKind::Cover { base }
            | IntentKind::Hold { base }
            | IntentKind::Raid { base, .. }
            | IntentKind::StrikeShips { base }
            | IntentKind::SearchWaters { base }
            | IntentKind::BuildUp { base, .. } => Addressee::Base(base),
            // Decisions 0124, 6 and 7, 0125, 1, and 0141, 1.
            IntentKind::StandAt { force, .. }
            | IntentKind::OperateCarriers { force, .. }
            | IntentKind::ReadyLanding { force, .. }
            | IntentKind::Bombard { force, .. } => Addressee::TaskForce(force),
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
            // Decision 0220, 2: no stand task stands under it.
            (
                IntentKind::StrikeShips { .. }
                | IntentKind::SearchWaters { .. }
                | IntentKind::OperateCarriers { .. },
                IntentKind::DestroyForce { .. },
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
            // Decision 0202, 18: a free carrier force under neutralising a
            // base, its bombers raiding the target.
            (IntentKind::OperateCarriers { .. }, IntentKind::Neutralise { .. }) => true,
            // Decision 0126, 7: a covering force stands at the run's
            // destination, the base's hex, which intake checks.
            (IntentKind::StandAt { .. }, IntentKind::KeepSupplied { .. }) => true,
            // Decision 0142, 12: the landing at the place's hex.
            (IntentKind::ReadyLanding { hex, .. }, IntentKind::TakePlace { hex: place, .. }) => {
                hex == place
            }
            // Decision 0146, 2: a bombardment of the target under
            // neutralising it, and of the base at the place under taking it,
            // whose hex intake checks.
            (IntentKind::Bombard { target, .. }, IntentKind::Neutralise { target: aimed }) => {
                target == aimed
            }
            (IntentKind::Bombard { .. }, IntentKind::TakePlace { .. }) => true,
            // Decision 0148, 1: a carrier force under taking a place.
            (IntentKind::OperateCarriers { .. }, IntentKind::TakePlace { .. }) => true,
            // Decision 0144, 7: the supply that follows, for the held base;
            // or, by decision 0159, A, for a base of the side in the held
            // base's hex, which intake checks.
            (IntentKind::KeepSupplied { .. }, IntentKind::Hold { .. }) => true,
            // Decision 0151, 6: the tree of an island.
            (
                IntentKind::TakePlace { .. }
                | IntentKind::Hold { .. }
                | IntentKind::Deny { .. }
                | IntentKind::Neutralise { .. },
                IntentKind::ControlIsland { .. },
            ) => true,
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
#[derive(Clone, PartialEq, Eq)]
pub struct ForcesLimit {
    pub ships: BTreeSet<ShipId>,
    pub task_forces: BTreeSet<TaskForceId>,
    /// Air groups the staff may order for the intent. Added in step 2 of
    /// stage 2.
    pub air_groups: BTreeSet<AirGroupId>,
    pub source: Option<BaseId>,
    /// Land units the staff may order for the intent (decision 0141, "A
    /// forces limit gains land units"). Added at the end.
    pub land_units: BTreeSet<LandUnitId>,
    /// Submarines the intent may use (decision 0218, 12 and 34): only a
    /// "patrol" may use a boat. Added at the end.
    pub submarines: BTreeSet<SubmarineId>,
    /// The kinds of which the staff may use nothing (decision 0222, the
    /// architect's ruling A): each is bound, with nothing of it named. Never
    /// read from an empty list. Added at the end.
    pub none_of: BTreeSet<ForceKind>,
    /// The task forces the limit names are used from the hex where each
    /// lies, by the forming rule of decisions 0124 and 0125: the carrier pass
    /// stations none of them (decision 0222, the architect's ruling I). Only
    /// with a task force named. Added at the end.
    pub in_place: bool,
}

/// A kind of force a forces limit binds (decision 0222, the architect's
/// ruling A): ships and task forces are one kind. The source is not one.
/// New kinds are added at the end; a code is never renumbered.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ForceKind {
    Ships,
    AirGroups,
    LandUnits,
    Submarines,
}

impl ForceKind {
    /// Every kind, in code order.
    pub const ALL: [ForceKind; 4] = [
        ForceKind::Ships,
        ForceKind::AirGroups,
        ForceKind::LandUnits,
        ForceKind::Submarines,
    ];

    /// The stable code of the kind. Never renumbered.
    pub fn code(self) -> u8 {
        match self {
            ForceKind::Ships => 1,
            ForceKind::AirGroups => 2,
            ForceKind::LandUnits => 3,
            ForceKind::Submarines => 4,
        }
    }

    /// The kind's name, as `none_of` lists it.
    pub fn name(self) -> &'static str {
        match self {
            ForceKind::Ships => "ships",
            ForceKind::AirGroups => "air_groups",
            ForceKind::LandUnits => "land_units",
            ForceKind::Submarines => "submarines",
        }
    }

    /// The kind named `name`, where there is one.
    pub fn from_name(name: &str) -> Option<ForceKind> {
        ForceKind::ALL.into_iter().find(|k| k.name() == name)
    }
}

impl ForcesLimit {
    /// Whether the limit names nothing of any kind: "use nothing", which
    /// binds every kind (decision 0222, rule 3: not the same as no limit).
    ///
    /// A limit that lists a kind in `none_of` names something: it binds that
    /// kind to nothing and leaves the kinds it neither names nor lists to the
    /// staff (the architect's ruling A).
    pub fn names_nothing(&self) -> bool {
        self.ships.is_empty()
            && self.task_forces.is_empty()
            && self.air_groups.is_empty()
            && self.source.is_none()
            && self.land_units.is_empty()
            && self.submarines.is_empty()
            && self.none_of.is_empty()
    }

    /// Whether the limit lists `kind` in `none_of`: none of it may be used.
    pub fn none_of(&self, kind: ForceKind) -> bool {
        self.none_of.contains(&kind)
    }

    /// The first kind the limit both names a unit of and lists in `none_of`,
    /// which intake refuses (the architect's ruling A).
    pub fn named_and_none_of(&self) -> Option<ForceKind> {
        ForceKind::ALL.into_iter().find(|kind| {
            self.none_of(*kind)
                && match kind {
                    ForceKind::Ships => self.names_ships(),
                    ForceKind::AirGroups => !self.air_groups.is_empty(),
                    ForceKind::LandUnits => !self.land_units.is_empty(),
                    ForceKind::Submarines => !self.submarines.is_empty(),
                }
        })
    }

    /// Whether the limit names a ship or a task force. Ships and task forces
    /// are one kind: the pool of ships, a ship lying in a task force it
    /// lists (decision 0222, rule 2).
    pub fn names_ships(&self) -> bool {
        !(self.ships.is_empty() && self.task_forces.is_empty())
    }

    /// Whether the limit binds the ships and task forces (decision 0222,
    /// rule 2): it names one, or it names nothing at all.
    pub fn binds_ships(&self) -> bool {
        self.names_ships() || self.none_of(ForceKind::Ships) || self.names_nothing()
    }

    /// Whether the limit binds the air groups: it names one, or nothing.
    pub fn binds_groups(&self) -> bool {
        !self.air_groups.is_empty() || self.none_of(ForceKind::AirGroups) || self.names_nothing()
    }

    /// Whether the limit binds the land units: it names one, or nothing.
    pub fn binds_land_units(&self) -> bool {
        !self.land_units.is_empty() || self.none_of(ForceKind::LandUnits) || self.names_nothing()
    }

    /// Whether the limit binds the submarines: it names one, or nothing.
    pub fn binds_submarines(&self) -> bool {
        !self.submarines.is_empty() || self.none_of(ForceKind::Submarines) || self.names_nothing()
    }

    /// Whether the limit binds the source: it names one, or nothing.
    pub fn binds_source(&self) -> bool {
        self.source.is_some() || self.names_nothing()
    }
}

/// Written as the derived form was, with the boats only where the limit
/// names one (decision 0218, stage 3): an event's words, and so every digest
/// of events, are what they were for a limit that names no boat.
impl core::fmt::Debug for ForcesLimit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut out = f.debug_struct("ForcesLimit");
        out.field("ships", &self.ships)
            .field("task_forces", &self.task_forces)
            .field("air_groups", &self.air_groups)
            .field("source", &self.source)
            .field("land_units", &self.land_units);
        if !self.submarines.is_empty() {
            out.field("submarines", &self.submarines);
        }
        // `in_place` (ruling I) and `none_of` are left out of the debug text (decision 0222, the
        // architect's ruling B): a closed proof's file that lists it, to say
        // in the new language the order it gave, writes the text it wrote,
        // so every digest of its events is what it was. Its JSON, its words
        // and its hash carry it.
        out.finish()
    }
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
    /// The first day the staff may release the plan (decision 0159, 9: "not
    /// before"). Set only on a "take a place" or a "control an island" of
    /// the commander's. Added at the end.
    pub not_before: Option<u32>,
    /// A task force or land unit of the side the staff waits for before it
    /// releases the plan (decision 0159, 9: "wait for"). Added at the end.
    pub wait_for: Option<Awaited>,
    /// Another "take a place" or "control an island" of the commander's
    /// whose place must be the side's before the staff releases the plan
    /// (decision 0159, 9: "after"). Added at the end.
    pub after: Option<IntentId>,
    /// How much must be known of the enemy at the place before the landing
    /// goes (decision 0159, 12: "knowledge"). Set only on a "take a place"
    /// or a "control an island" of the commander's; `None` where the risk
    /// decides it. Added at the end.
    pub knowledge: Option<Knowledge>,
    /// The kinds of ship to strike, in order, and whether the waters of the
    /// deny an area it stands under come first (decision 0190: "targets").
    /// `None`: not set on this intent; read up the tree, the nearest set on
    /// the chain. Added at the end.
    pub targets: Option<TargetsLimit>,
}

/// The targets limit of an intent (decision 0190): the kinds of ship its
/// bombers strike, in the order they are to be struck, and whether the
/// waters of the "deny an area" it is set on or stands under come first.
/// The staff gives the kinds as the target list of every bomber the limit
/// binds, at every risk, in place of the list the risk gives; and, with
/// `waters_first`, the deny's waters as the bomber's waters to strike first.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TargetsLimit {
    /// One to six kinds, each once, most wanted first.
    pub kinds: Vec<ShipKind>,
    /// True only on a "deny an area" or an intent beneath one.
    pub waters_first: bool,
}

/// How much must be known of the enemy at the place before a landing goes
/// (decision 0159, 12): the commander's "knowledge" limit, or what the risk
/// sets where it is not set.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Knowledge {
    /// The landing may go on any count or on none.
    NoCount,
    /// On a count seen from the air or by men in contact, of any day.
    Count,
    /// On such a count no more than this many days old on the morning it
    /// goes; at least 1.
    RecentCount(u32),
}

impl Knowledge {
    /// The days a recent count may be old where the commander gives no
    /// number (decision 0159, 12). A placeholder.
    pub const RECENT_DAYS: u32 = 3;

    /// A stable number. Part of the state hash. Never renumber.
    pub const fn code(self) -> u8 {
        match self {
            Knowledge::NoCount => 1,
            Knowledge::Count => 2,
            Knowledge::RecentCount(_) => 3,
        }
    }

    /// The setting's name, as a command writes it.
    pub const fn name(self) -> &'static str {
        match self {
            Knowledge::NoCount => "none",
            Knowledge::Count => "count",
            Knowledge::RecentCount(_) => "recent_count",
        }
    }

    /// The days of a recent count; `None` for the other settings.
    pub const fn days(self) -> Option<u32> {
        match self {
            Knowledge::RecentCount(days) => Some(days),
            Knowledge::NoCount | Knowledge::Count => None,
        }
    }

    /// What the risk sets where the limit is not set (decision 0147, 13):
    /// press none, balanced a count, preserve a recent count of
    /// [`Knowledge::RECENT_DAYS`].
    pub const fn of_risk(risk: Risk) -> Knowledge {
        match risk {
            Risk::Press => Knowledge::NoCount,
            Risk::Balanced => Knowledge::Count,
            Risk::Preserve => Knowledge::RecentCount(Knowledge::RECENT_DAYS),
        }
    }
}

impl core::fmt::Display for Knowledge {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Knowledge::NoCount => write!(f, "no count"),
            Knowledge::Count => write!(f, "a count"),
            Knowledge::RecentCount(1) => write!(f, "a count no more than 1 day old"),
            Knowledge::RecentCount(days) => write!(f, "a count no more than {days} days old"),
        }
    }
}

/// What a "wait for" limit names (decision 0159, 9): a task force or a land
/// unit of the side.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Awaited {
    TaskForce(TaskForceId),
    LandUnit(LandUnitId),
}

impl Awaited {
    /// A stable number. Part of the state hash. Never renumber.
    pub const fn code(self) -> u8 {
        match self {
            Awaited::TaskForce(_) => 1,
            Awaited::LandUnit(_) => 2,
        }
    }

    /// The raw number of the force or unit.
    pub const fn raw(self) -> u32 {
        match self {
            Awaited::TaskForce(force) => force.raw(),
            Awaited::LandUnit(unit) => unit.raw(),
        }
    }
}

impl core::fmt::Display for Awaited {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Awaited::TaskForce(force) => write!(f, "{force}"),
            Awaited::LandUnit(unit) => write!(f, "{unit}"),
        }
    }
}

/// The limits that tell a plan to wait (decision 0159, 9), read up the tree
/// from an intent: every one on the chain binds. Each is given with the
/// intent it is set on.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct WaitLimits {
    /// The latest "not before" day on the chain, with its intent.
    pub not_before: Option<(u32, IntentId)>,
    /// Every "wait for" on the chain, nearest first.
    pub wait_for: Vec<(Awaited, IntentId)>,
    /// Every "after" on the chain, nearest first.
    pub after: Vec<(IntentId, IntentId)>,
}

impl WaitLimits {
    /// Whether no such limit is set on the chain.
    pub fn is_empty(&self) -> bool {
        self.not_before.is_none() && self.wait_for.is_empty() && self.after.is_empty()
    }
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
    /// Decision 0159, 9: the plan is not released before `day`; `None`
    /// clears it. Added at the end.
    NotBefore {
        intent: IntentId,
        day: Option<u32>,
    },
    /// Decision 0159, 9: the plan waits for the force or unit; `None`
    /// clears it. Added at the end.
    WaitFor {
        intent: IntentId,
        awaited: Option<Awaited>,
    },
    /// Decision 0159, 9: the plan waits until `other`'s place is the side's;
    /// `None` clears it. Added at the end.
    After {
        intent: IntentId,
        other: Option<IntentId>,
    },
    /// Decision 0159, 12: how much must be known before the landing goes;
    /// `None` clears it, and the risk decides. Added at the end.
    Knowledge {
        intent: IntentId,
        knowledge: Option<Knowledge>,
    },
    /// Decision 0190: the kinds of ship to strike, in order, and whether the
    /// deny's waters come first; `None` clears it. Added at the end.
    Targets {
        intent: IntentId,
        targets: Option<TargetsLimit>,
    },
    /// Decision 0222, rule 3: the forces limit set on the intent is cleared,
    /// so that it binds no more and what it named is the staff's choice
    /// again, read up the tree. Not the same as a forces limit that names
    /// nothing, which is "use nothing". Added at the end.
    ForcesCleared {
        intent: IntentId,
    },
    /// Decision 0222, rule 3: the area set on the intent is cleared. Added at
    /// the end.
    AreaCleared {
        intent: IntentId,
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
            | Limit::Risk { intent, .. }
            | Limit::NotBefore { intent, .. }
            | Limit::WaitFor { intent, .. }
            | Limit::After { intent, .. }
            | Limit::Knowledge { intent, .. }
            | Limit::Targets { intent, .. }
            | Limit::ForcesCleared { intent }
            | Limit::AreaCleared { intent } => Some(*intent),
            Limit::Reserve { .. } => None,
        }
    }

    /// Whether the limit is one of the three that tell a plan to wait
    /// (decision 0159, 9).
    pub fn is_wait(&self) -> bool {
        matches!(
            self,
            Limit::NotBefore { .. } | Limit::WaitFor { .. } | Limit::After { .. }
        )
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

/// The leave to pursue set on an order (decision 0214, rule 4): whether a
/// carrier force may leave its order to go after a find. Read up the tree as
/// the other leaves are.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum PursueLeave {
    /// The staff asks leave for each find.
    Ask,
    /// The staff takes the leave as given and tells the commander.
    Go,
    /// Nothing changes from today.
    Stay,
}

impl PursueLeave {
    /// A stable number. Part of the state hash. Never renumber.
    pub const fn code(self) -> u8 {
        match self {
            PursueLeave::Ask => 1,
            PursueLeave::Go => 2,
            PursueLeave::Stay => 3,
        }
    }

    /// The setting's name as scenario files, saved games and the tools write
    /// it.
    pub const fn name(self) -> &'static str {
        match self {
            PursueLeave::Ask => "ask",
            PursueLeave::Go => "go",
            PursueLeave::Stay => "stay",
        }
    }

    /// The setting of that name.
    pub fn from_name(name: &str) -> Option<PursueLeave> {
        [PursueLeave::Ask, PursueLeave::Go, PursueLeave::Stay]
            .into_iter()
            .find(|leave| leave.name() == name)
    }
}

/// How a force came by its leave to pursue (decision 0214, rules 8 and 10).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PursuitBy {
    /// The commander's yes: the `CommandTaken` event of his answer.
    Answer(EventId),
    /// Taken by the staff, at go.
    Taken,
}

impl PursuitBy {
    /// A stable number. Part of the state hash. Never renumber.
    pub const fn code(self) -> u8 {
        match self {
            PursuitBy::Answer(_) => 1,
            PursuitBy::Taken => 2,
        }
    }
}

/// A force's leave to pursue under way (decision 0214, rules 8, 10 and 12).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pursuit {
    pub by: PursuitBy,
    /// The commander's number of the contact it began on (a number in the
    /// side's contact series).
    pub contact: u32,
    /// The day it began.
    pub since_day: u32,
}

// ---------------------------------------------------------------------------
// Plans.
// ---------------------------------------------------------------------------

/// The phase of a "take a place" plan (decision 0142, 5): read, not stored.
/// Prepare while the plan is preparing; move from its release until the first
/// man of the side lands at the place; land from then until the intent is
/// met. Follow up is step 4.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum PlacePhase {
    Prepare,
    Move,
    Land,
}

impl PlacePhase {
    /// The phase's name as the tools write it.
    pub const fn name(self) -> &'static str {
        match self {
            PlacePhase::Prepare => "prepare",
            PlacePhase::Move => "move",
            PlacePhase::Land => "land",
        }
    }
}

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
    /// A land unit: the staff gives it no order (decision 0141, 14). Made by
    /// a `hold` command or by a commander's order to a unit a task may use.
    /// Added at the end.
    LandUnit(LandUnitId),
    /// A submarine: the staff gives her no order (decision 0218, 34). Made
    /// by a `hold` command or by a commander's order to a boat a patrol may
    /// use. Added at the end.
    Submarine(SubmarineId),
}

impl HoldTarget {
    /// A stable number. Part of the state hash. Never renumber.
    pub const fn code(self) -> u8 {
        match self {
            HoldTarget::Ship(_) => 1,
            HoldTarget::TaskForce(_) => 2,
            HoldTarget::Setting { .. } => 3,
            HoldTarget::AirGroup(_) => 4,
            HoldTarget::LandUnit(_) => 5,
            HoldTarget::Submarine(_) => 6,
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
            HoldTarget::LandUnit(unit) => write!(f, "{unit}"),
            HoldTarget::Submarine(boat) => write!(f, "{boat}"),
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
    /// A "take a place" cannot bring the men it wants on any day in reach (a
    /// notice of the staff's, decision 0159, 7). Added at the end.
    CannotBringEnough,
    /// A "take a place" whose lift the staff turned back to its source for
    /// enemy carriers near its way (a notice of the staff's, decision 0191,
    /// 9). Added at the end.
    LiftTurnedBack,
    /// A carrier force asks leave to pursue a find (a notice of the staff's,
    /// decision 0214, 7). Its subject is the force. Added at the end
    /// (decision 0214).
    AsksLeaveToPursue,
}

impl StandingKind {
    /// Every kind, in code order.
    pub const ALL: [StandingKind; 6] = [
        StandingKind::CannotMeet,
        StandingKind::WaitsForLeave,
        StandingKind::ShipLost,
        StandingKind::CannotBringEnough,
        StandingKind::LiftTurnedBack,
        StandingKind::AsksLeaveToPursue,
    ];

    /// A stable number. Part of the state hash. Never renumber.
    pub const fn code(self) -> u8 {
        match self {
            StandingKind::CannotMeet => 1,
            StandingKind::WaitsForLeave => 2,
            StandingKind::ShipLost => 3,
            StandingKind::CannotBringEnough => 4,
            StandingKind::LiftTurnedBack => 5,
            StandingKind::AsksLeaveToPursue => 6,
        }
    }

    /// The kind's name as scenario files, saved games and the tools write it.
    pub const fn name(self) -> &'static str {
        match self {
            StandingKind::CannotMeet => "cannot_meet",
            StandingKind::WaitsForLeave => "waits_for_leave",
            StandingKind::ShipLost => "ship_lost",
            StandingKind::CannotBringEnough => "cannot_bring_enough",
            StandingKind::LiftTurnedBack => "lift_turned_back",
            StandingKind::AsksLeaveToPursue => "asks_leave_to_pursue",
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
    /// A task force: the subject of `AsksLeaveToPursue`. Added at the end
    /// (decision 0214).
    TaskForce(TaskForceId),
}

impl StandingSubject {
    /// A stable number. Part of the state hash. Never renumber.
    pub const fn code(self) -> u8 {
        match self {
            StandingSubject::Plan(_) => 1,
            StandingSubject::Ship(_) => 2,
            StandingSubject::TaskForce(_) => 3,
        }
    }

    /// The subject's number in its own series.
    pub const fn raw(self) -> u32 {
        match self {
            StandingSubject::Plan(plan) => plan.raw(),
            StandingSubject::Ship(ship) => ship.raw(),
            StandingSubject::TaskForce(force) => force.raw(),
        }
    }
}

impl core::fmt::Display for StandingSubject {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            StandingSubject::Plan(plan) => write!(f, "{plan}"),
            StandingSubject::Ship(ship) => write!(f, "{ship}"),
            StandingSubject::TaskForce(force) => write!(f, "{force}"),
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
    /// The leave to pursue set on an order (decision 0214, rule 4). Added at
    /// the end.
    pub pursue_leaves: BTreeMap<IntentId, PursueLeave>,
    /// Each force's leave to pursue under way, by answer or taken (decision
    /// 0214, rules 8, 10 and 12). Added at the end.
    pub pursuits: BTreeMap<TaskForceId, Pursuit>,
    /// The commander's no, by force and contact number (decision 0214, rule
    /// 8). Kept: a contact number is never reused in a side's series, so a
    /// refusal of a contact no longer held binds nothing. Added at the end.
    pub pursuit_refusals: BTreeSet<(TaskForceId, u32)>,
    /// How the side's staff uses its boats, as its scenario states it for
    /// its navy and date (decision 0223, stage 0). Set when the game is made
    /// and never changed by a rule or a command. Added at the end.
    pub submarine_doctrine: SubmarineDoctrine,
    /// Whether the side's scenario states that its staff chooses its sources
    /// of supply (decision 0228, amended on review, A). Set when the game is
    /// made and never changed by a rule or a command. Added at the end.
    pub staff_chooses_sources: bool,
    /// What each order to destroy a force keeps of the force it follows
    /// (decision 0220, 5): by order. Written at the giving by intake and at
    /// a notice by the engine; removed when the order is withdrawn. Added at
    /// the end.
    pub followed: BTreeMap<IntentId, Followed>,
}

/// What an order to destroy a force keeps of the force it follows (decision
/// 0220, 5 and R3): the contact's number in the commander's picture, the hex
/// and the day and pulse of its last report, whether it was taken for a
/// carrier, and whether the order is lost-contact.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Followed {
    /// The number of the contact followed, in the commander's picture.
    pub contact: u32,
    /// The hex of the last report kept: the order's centre.
    pub hex: Hex,
    /// The day and pulse of that report.
    pub day: u32,
    pub pulse: Pulse,
    /// How many ships the last report kept says (decision 0220, ruling 6 after
    /// the build), written with the hex.
    pub ship_count: u32,
    /// The kind the last report kept says, written with the hex. Not the
    /// sort: the sort changes only at a taking.
    pub kind: ShipKind,
    /// Whether the force was taken for a carrier, when the order was given or
    /// last took a contact (the sort, rule 7).
    pub carrier: bool,
    /// Whether the order is lost-contact (rule 9).
    pub lost: bool,
}

/// What a notice of following says (decision 0220, 8 and R3): the order, the
/// contact and the report.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FollowNotice {
    /// The order.
    pub intent: IntentId,
    /// Follows: the contact followed now. Lost and ended: the contact last
    /// followed.
    pub contact: u32,
    /// Follows: the number followed before, where the number changed (a
    /// taking); `None` where the same contact was refreshed. `None` for lost
    /// and ended.
    pub taken_from: Option<u32>,
    /// Follows, at a taking: the contact taken is of the other sort (a
    /// carrier for a force that was not, or the other way). False otherwise.
    pub other_sort: bool,
    /// The report: hex, day and pulse (for lost and ended, the report kept).
    pub hex: Hex,
    pub day: u32,
    pub pulse: Pulse,
    /// What the report says: the count and the kind; whether it is a carrier.
    pub ship_count: u32,
    pub kind: ShipKind,
    pub carrier: bool,
}

/// The absolute number of a pulse: three to a day, night 0, morning 1,
/// afternoon 2 (decision 0220, R2).
fn absolute_pulse(day: u32, pulse: Pulse) -> u64 {
    3 * u64::from(day)
        + match pulse {
            Pulse::Night => 0,
            Pulse::Morning => 1,
            Pulse::Afternoon => 2,
        }
}

/// The day and pulse of the last report of a contact of age `age` pulses, at
/// a boundary whose next pulse to be played is `next_pulse` of `next_day`
/// (decision 0220, R2): the pulse `age + 1` before that one. At dusk of day
/// N the next pulse is the night of day N; inside day N after its night it is
/// the morning of day N. So a contact a side starts with, at age 0 at the
/// dusk of day 1, is read as one reported in the afternoon of day 0. A report
/// that would be before the night of day 0 is read as the night of day 0.
pub fn report_pulse(next_day: u32, next_pulse: Pulse, age: u32) -> (u32, Pulse) {
    let next = absolute_pulse(next_day, next_pulse);
    let back = u64::from(age) + 1;
    let at = next.saturating_sub(back);
    let day = u32::try_from(at / 3).unwrap_or(u32::MAX);
    let pulse = match at % 3 {
        0 => Pulse::Night,
        1 => Pulse::Morning,
        _ => Pulse::Afternoon,
    };
    (day, pulse)
}

/// The hours a pulse starts at in its day (decision 0220, R4): the night 12
/// hours from hour 0, the morning 6 from hour 12, the afternoon 6 from hour
/// 18.
const fn pulse_start_hour(pulse: Pulse) -> u64 {
    match pulse {
        Pulse::Night => 0,
        Pulse::Morning => 12,
        Pulse::Afternoon => 18,
    }
}

/// The hours a pulse lasts (decision 0220, R4).
const fn pulse_hours(pulse: Pulse) -> u64 {
    match pulse {
        Pulse::Night => 12,
        Pulse::Morning | Pulse::Afternoon => 6,
    }
}

/// How far a contact may have moved between two reports, in whole hexes
/// (decision 0220, R4): the hours from the start of the pulse of report A to
/// the end of the pulse of report B, at `knots`, cut to whole hexes of 40
/// nautical miles. Zero where B is not after A.
pub fn report_distance_hexes(
    a_day: u32,
    a_pulse: Pulse,
    b_day: u32,
    b_pulse: Pulse,
    knots: u32,
) -> u32 {
    let start = 24 * u64::from(a_day) + pulse_start_hour(a_pulse);
    let end = 24 * u64::from(b_day) + pulse_start_hour(b_pulse) + pulse_hours(b_pulse);
    let hours = end.saturating_sub(start);
    let hexes = hours * u64::from(knots) / 40;
    u32::try_from(hexes).unwrap_or(u32::MAX)
}

/// The pattern the side's staff places the boats of a patrol in (decision
/// 0223, stage A): off enemy bases (the American way of 1942) or in lines
/// across the approach (the Japanese way of 1942).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SubmarinePattern {
    OffBases,
    Lines,
}

impl SubmarinePattern {
    /// A stable number for the pattern. Never renumber: it feeds the state
    /// hash. 0 is no pattern.
    pub const fn code(self) -> u8 {
        match self {
            SubmarinePattern::OffBases => 1,
            SubmarinePattern::Lines => 2,
        }
    }

    /// The word a scenario, a save and an answer use: `off_bases` or
    /// `lines`.
    pub const fn name(self) -> &'static str {
        match self {
            SubmarinePattern::OffBases => "off_bases",
            SubmarinePattern::Lines => "lines",
        }
    }

    /// The pattern a word names, if it names one.
    pub fn from_name(word: &str) -> Option<SubmarinePattern> {
        match word {
            "off_bases" => Some(SubmarinePattern::OffBases),
            "lines" => Some(SubmarinePattern::Lines),
            _ => None,
        }
    }
}

/// How a side's staff uses its boats (decision 0223, stage 0): the side's
/// own, known to it and to no one else. A side given neither value by its
/// scenario has no pattern and expects nothing, and plays as before.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct SubmarineDoctrine {
    /// The pattern of a patrol, or `None` for the spread of decision 0218.
    pub pattern: Option<SubmarinePattern>,
    /// Whether the side expects enemy boats off its own bases by habit.
    /// Stored and shown; read by no rule yet (decision 0223, stage D1 reads
    /// it).
    pub expects_boats_off_own_bases: bool,
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
    ///
    /// On the chain of a "take a place" each limit is read as it binds there
    /// (decision 0149, 2 and 3): see [`CommandState::forces_as_bound`].
    ///
    /// An island's own limit binds only what it names, kind by kind (decision
    /// 0151, 4): what it does not name of the task forces (with the ships),
    /// the land units and the source is filled from the nearest limit beneath
    /// it on the chain; with none beneath, it is read as set.
    pub fn forces_on_chain(&self, intent: IntentId) -> Vec<Cow<'_, ForcesLimit>> {
        let mut found: Vec<Cow<'_, ForcesLimit>> = Vec::new();
        for each in self.up_the_tree(intent) {
            let Some(bound) = self.forces_as_bound(each) else {
                continue;
            };
            let bound = match (each.kind, found.last()) {
                (IntentKind::ControlIsland { .. }, Some(beneath)) => {
                    Cow::Owned(fill_unnamed(bound.into_owned(), beneath))
                }
                _ => bound,
            };
            found.push(bound);
        }
        found
    }

    /// Whether the forces limits on `intent`'s chain are read as they bind
    /// under a "take a place" or a "control an island" on it (decisions 0149
    /// and 0151), by those orders' own rules; elsewhere each limit binds the
    /// kinds it names (decision 0222, rule 2).
    pub fn reads_as_bound(&self, intent: IntentId) -> bool {
        self.take_place_on_chain(intent).is_some() || self.island_on_chain(intent).is_some()
    }

    /// The forces limits on `intent`'s chain that bind its ships and task
    /// forces, nearest first (decision 0222, rule 2): each that names a ship
    /// or a task force, or names nothing at all. Under a take or an island,
    /// every limit on the chain, read as it binds. Empty where none does:
    /// then the intent may use the ships its addressee commands.
    pub fn ship_limits_on_chain(&self, intent: IntentId) -> Vec<Cow<'_, ForcesLimit>> {
        let by_kind = !self.reads_as_bound(intent);
        self.forces_on_chain(intent)
            .into_iter()
            .filter(|limit| !by_kind || limit.binds_ships())
            .collect()
    }

    /// The nearest forces limit on `intent`'s chain that binds its ships and
    /// task forces (decision 0222, rule 2); for a chain under a take or an
    /// island, the nearest as it binds.
    pub fn nearest_ship_limit(&self, intent: IntentId) -> Option<Cow<'_, ForcesLimit>> {
        self.ship_limits_on_chain(intent).into_iter().next()
    }

    /// Whether the task forces named on `intent`'s chain work from where they
    /// lie (decision 0222, the architect's ruling I): the nearest limit that
    /// binds the ships says `in_place`.
    pub fn in_place_on_chain(&self, intent: IntentId) -> bool {
        self.nearest_ship_limit(intent)
            .is_some_and(|limit| limit.in_place)
    }

    /// The source named on `intent`'s chain (decision 0222, rule 2): of the
    /// nearest limit that binds the source, its own or above it. Under a take
    /// or an island, the nearest limit's, as it binds.
    pub fn source_on_chain(&self, intent: IntentId) -> Option<BaseId> {
        let by_kind = !self.reads_as_bound(intent);
        self.forces_on_chain(intent)
            .into_iter()
            .find(|limit| !by_kind || limit.binds_source())
            .and_then(|limit| limit.source)
    }

    /// The source a commander's own intent on `intent`'s chain names
    /// (decision 0228): the nearest intent of his, itself included, whose
    /// forces limit names one.
    pub fn commander_source_on_chain(&self, intent: IntentId) -> Option<BaseId> {
        self.up_the_tree(intent)
            .filter(|found| found.maker == Maker::Commander)
            .find_map(|found| found.limits.forces.as_ref()?.source)
    }

    /// Where the supply `supply` is supplied from (decision 0228, rule 18):
    /// the source it reads, the nearest forces limit on its chain that names
    /// one, and who chose it: the commander where an intent of his on the
    /// chain names a source, else the staff. Worked from the command state;
    /// stored nowhere.
    pub fn supplied_from_of(&self, supply: IntentId) -> Option<(BaseId, Maker)> {
        let source = self.source_on_chain(supply)?;
        let by = if self.commander_source_on_chain(supply).is_some() {
            Maker::Commander
        } else {
            Maker::Staff
        };
        Some((source, by))
    }

    /// Where the hold or island `intent` is supplied from (decision 0228,
    /// rule 18): of the first "keep supplied" in force under it (under an
    /// island, under its holds), by number. `None` where none is.
    pub fn supplied_from(&self, intent: IntentId) -> Option<(BaseId, Maker)> {
        let mut under = self.children_in_force(intent);
        under.sort();
        for child in under {
            let Some(found) = self.intents.get(&child) else {
                continue;
            };
            match found.kind {
                IntentKind::KeepSupplied { .. } => {
                    if let Some(from) = self.supplied_from_of(child) {
                        return Some(from);
                    }
                }
                IntentKind::Hold { .. } => {
                    if let Some(from) = self.supplied_from(child) {
                        return Some(from);
                    }
                }
                _ => {}
            }
        }
        None
    }

    /// The forces limit a run of the plan of `owner` draws on in place of the
    /// owner's own (decision 0228, the architect's ruling on the build): for a
    /// side whose staff chooses its sources, where `owner` is a "hold a
    /// place", the forces limit of the lowest-numbered "keep supplied" in
    /// force directly under it that has one. `None` otherwise.
    pub fn supply_limit_under_hold(&self, owner: IntentId) -> Option<&ForcesLimit> {
        if !self.staff_chooses_sources {
            return None;
        }
        let hold = self.intents.get(&owner)?;
        if !matches!(hold.kind, IntentKind::Hold { .. }) {
            return None;
        }
        self.intents_in_force()
            .filter(|i| {
                i.parent == Some(owner) && matches!(i.kind, IntentKind::KeepSupplied { .. })
            })
            .find_map(|i| i.limits.forces.as_ref())
    }

    /// Whether `intent` owns a plan of its own: a root, or an intent formed
    /// with one (see [`CommandState::forms_own_plan`]).
    pub fn owns_plan(&self, intent: &Intent) -> bool {
        match intent.parent {
            None => true,
            Some(parent) => self.forms_own_plan(intent.kind, parent),
        }
    }

    /// Whether an intent of `kind` formed under `parent` has a plan of its own
    /// (decision 0151, 9, the architect's additions on the build's findings):
    /// a "take a place" under "control an island", prepared, released and
    /// marked complete as a root take's is; and a "keep supplied" under a
    /// "hold a place" under "control an island", whose runs are its own, as
    /// a root supply intent's are, since the island's holds share one plan.
    pub fn forms_own_plan(&self, kind: IntentKind, parent: IntentId) -> bool {
        let Some(above) = self.intents.get(&parent) else {
            return false;
        };
        match (kind, above.kind) {
            (IntentKind::TakePlace { .. }, IntentKind::ControlIsland { .. }) => true,
            (IntentKind::KeepSupplied { .. }, IntentKind::Hold { .. }) => above
                .parent
                .and_then(|p| self.intents.get(&p))
                .is_some_and(|p| matches!(p.kind, IntentKind::ControlIsland { .. })),
            _ => false,
        }
    }

    /// The intent whose plan `intent` serves: the nearest on its chain,
    /// itself included, that owns one (decision 0117, 9, and 0151, 9).
    pub fn plan_owner(&self, intent: IntentId) -> IntentId {
        self.up_the_tree(intent)
            .find(|found| self.owns_plan(found))
            .map_or(intent, |found| found.id)
    }

    /// The "control an island" on `intent`'s chain: itself or above it
    /// (decision 0151).
    pub fn island_on_chain(&self, intent: IntentId) -> Option<&Intent> {
        self.up_the_tree(intent)
            .find(|found| matches!(found.kind, IntentKind::ControlIsland { .. }))
    }

    /// The nearest forces limit on `intent`'s chain, for words: what binds is
    /// every limit on the chain ([`CommandState::forces_on_chain`]). For a
    /// root, its own.
    pub fn nearest_forces(&self, intent: IntentId) -> Option<Cow<'_, ForcesLimit>> {
        self.forces_on_chain(intent).into_iter().next()
    }

    /// The nearest "take a place" on `intent`'s chain: itself or above it.
    pub fn take_place_on_chain(&self, intent: IntentId) -> Option<&Intent> {
        self.up_the_tree(intent)
            .find(|found| matches!(found.kind, IntentKind::TakePlace { .. }))
    }

    /// The forces limit of the "ready a landing" the staff formed under the
    /// "take a place" `end`, in force: the staff's choice (decision 0149, 3).
    pub fn assembled_forces(&self, end: IntentId) -> Option<&ForcesLimit> {
        self.intents_in_force()
            .find(|found| {
                found.parent == Some(end)
                    && found.maker == Maker::Staff
                    && matches!(found.kind, IntentKind::ReadyLanding { .. })
            })
            .and_then(|found| found.limits.forces.as_ref())
    }

    /// The forces limit set on `found` as it binds (decision 0149, 2 and 3).
    /// Of a "take a place": each of the task forces (with the ships), the
    /// land units and the source is the commander's where he named it, and
    /// where he did not the staff's, kept on the "ready a landing" it formed.
    /// Of an intent under a "take a place": what it does not name of the
    /// three is that end's, as just read. Any other is read as set.
    pub fn forces_as_bound<'a>(&'a self, found: &'a Intent) -> Option<Cow<'a, ForcesLimit>> {
        let own = found.limits.forces.as_ref();
        let fill = match found.kind {
            IntentKind::TakePlace { .. } => self.assembled_forces(found.id).cloned(),
            _ => match self.take_place_on_chain(found.id) {
                Some(end) if own.is_some() => self.forces_as_bound(end).map(Cow::into_owned),
                _ => None,
            },
        };
        let Some(fill) = fill else {
            return own.map(Cow::Borrowed);
        };
        let Some(own) = own else {
            return Some(Cow::Owned(fill));
        };
        Some(Cow::Owned(fill_unnamed(own.clone(), &fill)))
    }

    /// The air groups the forces limits on `intent`'s chain allow: those
    /// named in every one of them. `None` where none is set on the chain.
    ///
    /// Under "control an island" a limit that names no air group does not
    /// bind the air groups (decision 0151, 4, the architect's addition on
    /// the build's finding): with none on the chain naming one, `None`, so
    /// an intent addressed to a base may use the groups based there.
    ///
    /// Elsewhere a limit binds the air groups only where it names one, or
    /// names nothing at all (decision 0222, rule 2): a limit that names task
    /// forces and no air group leaves the groups to the staff.
    pub fn groups_on_chain(&self, intent: IntentId) -> Option<BTreeSet<AirGroupId>> {
        let island = self.island_on_chain(intent).is_some();
        let by_kind = !self.reads_as_bound(intent);
        let mut limits = self.forces_on_chain(intent).into_iter().filter(|limit| {
            if by_kind {
                limit.binds_groups()
            } else {
                !island || !limit.air_groups.is_empty() || limit.none_of(ForceKind::AirGroups)
            }
        });
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

    /// The limits that tell `intent`'s plan to wait, read up the tree
    /// (decision 0159, 9): the latest "not before" on the chain, and every
    /// "wait for" and "after", nearest first. Each binds.
    pub fn waits_read_up(&self, intent: IntentId) -> WaitLimits {
        let mut found = WaitLimits::default();
        for on in self.up_the_tree(intent) {
            if let Some(day) = on.limits.not_before {
                if found.not_before.is_none_or(|(latest, _)| day > latest) {
                    found.not_before = Some((day, on.id));
                }
            }
            if let Some(awaited) = on.limits.wait_for {
                found.wait_for.push((awaited, on.id));
            }
            if let Some(other) = on.limits.after {
                found.after.push((other, on.id));
            }
        }
        found
    }

    /// The "knowledge" limit binding `intent`, read up the tree (decision
    /// 0159, 12): the nearest set on the chain, with the intent it is set
    /// on; `None` where none is set and the risk decides.
    pub fn knowledge_read_up(&self, intent: IntentId) -> Option<(Knowledge, IntentId)> {
        self.up_the_tree(intent)
            .find_map(|on| on.limits.knowledge.map(|knowledge| (knowledge, on.id)))
    }

    /// The leave to pursue binding `intent`, read up the tree (decision 0214,
    /// 4): the nearest set on the chain, with the intent it is set on. Where
    /// none is set, the default and `None`: go where the side's leave to
    /// launch is given (a side with no commander is given both leaves), else
    /// ask.
    pub fn pursue_read_up(&self, intent: IntentId) -> (PursueLeave, Option<IntentId>) {
        // Decision 0220, 2 and 3: an order to destroy a force with no leave set
        // on it reads "go" ("stay" where search only) and stops the walk.
        match self.up_the_tree(intent).find_map(|on| {
            self.pursue_leaves
                .get(&on.id)
                .map(|leave| (*leave, on.id))
                .or(match on.kind {
                    IntentKind::DestroyForce { search_only, .. } => Some((
                        if search_only {
                            PursueLeave::Stay
                        } else {
                            PursueLeave::Go
                        },
                        on.id,
                    )),
                    _ => None,
                })
        }) {
            Some((leave, on)) => (leave, Some(on)),
            None if self.leaves.launch => (PursueLeave::Go, None),
            None => (PursueLeave::Ask, None),
        }
    }

    /// The forces whose leave to pursue is the commander's answer (decision
    /// 0214, 10), in number order.
    pub fn leaves_by_answer(&self) -> impl Iterator<Item = TaskForceId> + '_ {
        self.pursuits
            .iter()
            .filter(|(_, pursuit)| matches!(pursuit.by, PursuitBy::Answer(_)))
            .map(|(force, _)| *force)
    }

    /// The targets limit binding `intent`, read up the tree (decision 0190,
    /// 2): the nearest set on the chain, with the intent it is set on;
    /// `None` where none is set, and every list and choice is today's.
    pub fn targets_read_up(&self, intent: IntentId) -> Option<(&TargetsLimit, IntentId)> {
        self.up_the_tree(intent)
            .find_map(|on| on.limits.targets.as_ref().map(|targets| (targets, on.id)))
    }

    /// The "deny an area" intents in force whose nearest forces limit names
    /// the task force `force` (decision 0190, 28), by number. The strike
    /// step, a carrier force's closing and its search read them.
    pub fn denies_naming(&self, force: TaskForceId) -> Vec<&Intent> {
        self.intents_in_force()
            .filter(|found| matches!(found.kind, IntentKind::Deny { .. }))
            .filter(|found| {
                self.nearest_forces(found.id)
                    .is_some_and(|limit| limit.task_forces.contains(&force))
            })
            .collect()
    }

    /// Whether `intent` is a "deny an area" or stands beneath one (decision
    /// 0190, 27): the intents a targets limit with `waters_first` may be set
    /// on.
    pub fn is_or_under_deny(&self, intent: IntentId) -> bool {
        self.up_the_tree(intent)
            .any(|found| matches!(found.kind, IntentKind::Deny { .. }))
    }

    /// The land units the forces limits on `intent`'s chain allow: those
    /// named in every one of them (decision 0141). `None` where none is set
    /// on the chain.
    ///
    /// Outside a take and an island a limit binds the land units only where
    /// it names one, or names nothing at all (decision 0222, rule 2).
    pub fn land_units_on_chain(&self, intent: IntentId) -> Option<BTreeSet<LandUnitId>> {
        let by_kind = !self.reads_as_bound(intent);
        let mut limits = self
            .forces_on_chain(intent)
            .into_iter()
            .filter(|limit| !by_kind || limit.binds_land_units());
        let first = limits.next()?.land_units.clone();
        Some(limits.fold(first, |allowed, limit| {
            allowed.intersection(&limit.land_units).copied().collect()
        }))
    }

    /// The submarines the forces limits on `intent`'s chain allow (decision
    /// 0218, 12): those named in every limit on the chain that names at
    /// least one boat. A limit naming no boat does not bind the boats, as a
    /// limit naming no air group does not bind the groups under an island.
    /// `None` where no limit on the chain names a boat.
    ///
    /// A limit that names nothing at all binds the boats too: "use nothing"
    /// (decision 0222, rule 3).
    pub fn submarines_on_chain(&self, intent: IntentId) -> Option<BTreeSet<SubmarineId>> {
        let mut limits = self
            .forces_on_chain(intent)
            .into_iter()
            .filter(|limit| limit.binds_submarines());
        let first = limits.next()?.submarines.clone();
        Some(limits.fold(first, |allowed, limit| {
            allowed.intersection(&limit.submarines).copied().collect()
        }))
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

    /// The intent on `intent`'s chain that a limit of `kind` a reason
    /// speaks of is set on (decision 0222, rule 5): the nearest that sets
    /// it, itself first; for the risk, the one whose risk binds (the least
    /// bold, the nearest of those); where none sets it, the root of the
    /// chain, the order the commander would set it on. For a reserve, which
    /// is a base's, the root.
    pub fn limit_set_on(&self, intent: IntentId, kind: LimitKind) -> IntentId {
        let set = |found: &Intent| -> bool {
            let limits = &found.limits;
            match kind {
                LimitKind::Forces => limits.forces.is_some(),
                LimitKind::Source => limits.forces.as_ref().is_some_and(|f| f.source.is_some()),
                LimitKind::Area => limits.area.is_some(),
                LimitKind::Priority => limits.priority.is_some(),
                LimitKind::Protected => !limits.protected.is_empty(),
                LimitKind::Risk => limits.risk == Some(self.risk_read_up(intent)),
                LimitKind::NotBefore => limits.not_before.is_some(),
                LimitKind::WaitFor => limits.wait_for.is_some(),
                LimitKind::After => limits.after.is_some(),
                LimitKind::Knowledge => limits.knowledge.is_some(),
                LimitKind::Targets => limits.targets.is_some(),
                LimitKind::Reserve => false,
            }
        };
        self.up_the_tree(intent)
            .find(|found| set(found))
            .map_or_else(|| self.root_of(intent), |found| found.id)
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

    /// Whether the command state is empty: nothing was ever set, and the
    /// scenario stated no submarine doctrine (decision 0223, stage 0). A side
    /// with an empty command state is played exactly as before stage 1.
    pub fn is_empty(&self) -> bool {
        self.intents.is_empty()
            && self.plans.is_empty()
            && self.commitments.is_empty()
            && self.holds.is_empty()
            && self.interrupts.is_empty()
            && self.standing.is_empty()
            && self.standing_off.is_empty()
            && self.pursue_leaves.is_empty()
            && self.pursuits.is_empty()
            && self.pursuit_refusals.is_empty()
            && self.followed.is_empty()
            && self.leaves == Leaves::default()
            && self.next_plan == 1
            && self.submarine_doctrine == SubmarineDoctrine::default()
            && !self.staff_chooses_sources
    }
}

/// `own` with what it does not name of the task forces (with the ships), the
/// land units and the source taken from `fill` (decisions 0149, 3, and 0151,
/// 4). The air groups are `own`'s.
fn fill_unnamed(mut own: ForcesLimit, fill: &ForcesLimit) -> ForcesLimit {
    // A kind listed in `none_of` is bound to nothing and is not filled.
    if own.task_forces.is_empty() && own.ships.is_empty() && !own.none_of(ForceKind::Ships) {
        own.task_forces.clone_from(&fill.task_forces);
        own.ships.clone_from(&fill.ships);
    }
    if own.land_units.is_empty() && !own.none_of(ForceKind::LandUnits) {
        own.land_units.clone_from(&fill.land_units);
    }
    if own.source.is_none() {
        own.source = fill.source;
    }
    own
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
    /// The commander's: set the leave to pursue on an intent, or clear it
    /// with `None` (it is read up the tree again). Not an amendment of a
    /// plan. Added at the end (decision 0214, rule 4).
    SetPursuit {
        intent: IntentId,
        leave: Option<PursueLeave>,
    },
    /// The commander's: answer a force's asking leave to pursue a contact,
    /// yes or no. A no ends a pursuit of the force that is under way. Added
    /// at the end (decision 0214, rule 8).
    AnswerPursuit {
        force: TaskForceId,
        contact: u32,
        allow: bool,
    },
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
    /// further bound by the leaves. The staff may set a forces limit, and
    /// only on an intent it formed (decision 0142, 12), which intake checks.
    pub fn allowed_from(&self, issuer: Issuer) -> bool {
        match self {
            CommandKind::SetLimit(Limit::Forces { .. }) => {
                matches!(issuer, Issuer::Player | Issuer::Staff)
            }
            // The staff withdraws an intent it formed (decision 0151, 8b);
            // intake refuses it one the commander made.
            CommandKind::WithdrawIntent { .. } => {
                matches!(issuer, Issuer::Player | Issuer::Staff)
            }
            CommandKind::SetIntent { .. }
            | CommandKind::SetLimit(_)
            | CommandKind::SetLeave { .. }
            | CommandKind::CancelPlan { .. }
            | CommandKind::Hold { .. }
            | CommandKind::Release { .. }
            | CommandKind::SetInterrupt(_)
            | CommandKind::SetPursuit { .. }
            | CommandKind::AnswerPursuit { .. }
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
            CommandKind::SetPursuit { .. } => "set_pursuit",
            CommandKind::AnswerPursuit { .. } => "answer_pursuit",
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
    /// The land unit is not one of the side's (decision 0141: a forces limit
    /// or a hold names it). Added at the end.
    NotYourLandUnit(LandUnitId),
    /// The hex of a "ready a landing" task is not a coast hex on the map
    /// (decision 0141, 1). Added at the end.
    NotACoastHex(Hex),
    /// The hex of a "take a place" end is not a hex of land on the map
    /// (decision 0142, 1). Added at the end.
    NotLand(Hex),
    /// The island of a "control an island" end is not one the scenario
    /// names (decision 0151, 1). Added at the end.
    NoSuchIsland(u32),
    /// A "not before", "wait for" or "after" limit is set on an intent that
    /// is not a "take a place" or a "control an island" of the commander's
    /// (decision 0159, 9). Added at the end.
    NoWaitLimitOn(IntentId),
    /// An "after" limit names an intent that is not another "take a place"
    /// or "control an island" of the commander's (decision 0159, 9). Added
    /// at the end.
    NotAnEndToWaitFor(IntentId),
    /// A "knowledge" limit is set on an intent that is not a "take a place"
    /// or a "control an island" of the commander's (decision 0159, 12).
    /// Added at the end.
    NoKnowledgeLimitOn(IntentId),
    /// A "knowledge" limit asks for a recent count of no days (decision
    /// 0159, 12): a recent count is at least 1 day. Added at the end.
    RecentCountOfNoDays(IntentId),
    /// A targets limit names a kind of ship twice, or names none (decision
    /// 0190, 1): one to six kinds, each once. Added at the end.
    BadTargetKinds(IntentId),
    /// A targets limit with `waters_first` is set on an intent that is not a
    /// "deny an area" and does not stand beneath one (decision 0190, 27).
    /// Added at the end.
    NoWatersFirstOn(IntentId),
    /// The submarine is not one of the side's, does not exist or is sunk
    /// (decision 0218, 34: a forces limit or a hold names her). Added at the
    /// end.
    NotYourSubmarine(SubmarineId),
    /// A forces limit names a unit of a kind and lists that kind in
    /// `none_of` (decision 0222, the architect's ruling A). Added at the end.
    NamedAndNoneOf(ForceKind),
    /// A forces limit says `in_place` and names no task force (decision
    /// 0222, the architect's ruling I). Added at the end.
    InPlaceWithNoTaskForce,
    /// An order to destroy a force names a number the commander's picture
    /// does not hold, or none (0) (decision 0220, R12). Added at the end.
    ContactNotHeld(u32),
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
            CommandRefusal::NotYourLandUnit(unit) => {
                write!(f, "{unit} is not one of the side's land units")
            }
            CommandRefusal::NotYourSubmarine(boat) => {
                write!(f, "{boat} is not one of the side's submarines")
            }
            CommandRefusal::InPlaceWithNoTaskForce => f.write_str(
                "the forces limit says its task forces work from where they lie and names no task force",
            ),
            CommandRefusal::NamedAndNoneOf(kind) => write!(
                f,
                "the forces limit names {} and lists them in none_of: a kind is named or listed, not both",
                kind.name()
            ),
            CommandRefusal::NotACoastHex(hex) => write!(f, "{hex} is not a coast hex on the map"),
            CommandRefusal::NotLand(hex) => write!(f, "{hex} is not a hex of land on the map"),
            CommandRefusal::NoSuchIsland(number) => {
                write!(f, "island {number} is not an island of the scenario")
            }
            CommandRefusal::NoWaitLimitOn(id) => write!(
                f,
                "{id} is not a take_place or a control_island of the commander's: only those may be told to wait"
            ),
            CommandRefusal::NotAnEndToWaitFor(id) => write!(
                f,
                "{id} is not another take_place or control_island of the commander's to wait for"
            ),
            CommandRefusal::NoKnowledgeLimitOn(id) => write!(
                f,
                "{id} is not a take_place or a control_island of the commander's: only those may be told how much must be known"
            ),
            CommandRefusal::RecentCountOfNoDays(id) => write!(
                f,
                "the knowledge limit on {id} asks for a recent count of no days: a recent count is at least 1 day old"
            ),
            CommandRefusal::BadTargetKinds(id) => write!(
                f,
                "the targets limit on {id} must name one to six kinds of ship, each once"
            ),
            CommandRefusal::NoWatersFirstOn(id) => write!(
                f,
                "{id} is not a deny and does not stand beneath one: only there may the waters come first"
            ),
            CommandRefusal::ContactNotHeld(0) => f.write_str("no contact is named"),
            CommandRefusal::ContactNotHeld(number) => write!(
                f,
                "contact {number} is not held: the commander's picture holds no contact of that number"
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
                    if let Some(source) = limit.source {
                        write!(f, " from {source}")?;
                    }
                    // Decision 0222, the architect's ruling I.
                    if limit.in_place {
                        write!(f, ", from where they lie")?;
                    }
                    Ok(())
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
                Limit::NotBefore { intent, day } => match day {
                    Some(day) => write!(f, "{intent} not before day {day}"),
                    None => write!(f, "clear {intent}'s not before"),
                },
                Limit::WaitFor { intent, awaited } => match awaited {
                    Some(awaited) => write!(f, "{intent} waits for {awaited}"),
                    None => write!(f, "clear {intent}'s wait for"),
                },
                Limit::After { intent, other } => match other {
                    Some(other) => write!(f, "{intent} after {other}"),
                    None => write!(f, "clear {intent}'s after"),
                },
                Limit::Knowledge { intent, knowledge } => match knowledge {
                    Some(knowledge) => write!(f, "{intent} goes on {knowledge}"),
                    None => write!(f, "clear {intent}'s knowledge"),
                },
                Limit::ForcesCleared { intent } => {
                    write!(f, "clear {intent}'s forces limit")
                }
                Limit::AreaCleared { intent } => write!(f, "clear {intent}'s area"),
                Limit::Targets { intent, targets } => match targets {
                    Some(targets) => {
                        write!(
                            f,
                            "{intent} strikes {}",
                            targets
                                .kinds
                                .iter()
                                .map(|kind| kind.name())
                                .collect::<Vec<_>>()
                                .join(", ")
                        )?;
                        if targets.waters_first {
                            write!(f, ", the waters first")?;
                        }
                        Ok(())
                    }
                    None => write!(f, "clear {intent}'s targets"),
                },
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
            CommandKind::SetPursuit { intent, leave } => match leave {
                Some(leave) => write!(f, "set the leave to pursue on {intent} to {}", leave.name()),
                None => write!(f, "clear the leave to pursue on {intent}"),
            },
            CommandKind::AnswerPursuit {
                force,
                contact,
                allow,
            } => write!(
                f,
                "answer {force}'s asking to pursue contact {contact}: {}",
                if *allow { "allowed" } else { "refused" }
            ),
        }
    }
}

/// The statement of a "patrol" intent in words (decision 0218, 11): "patrol
/// the waters within 3 hexes of (5,2) with submarines"; one hex is "1 hex".
pub fn patrol_words(hex: Hex, distance: u32) -> String {
    let hexes = if distance == 1 { "hex" } else { "hexes" };
    format!("patrol the waters within {distance} {hexes} of {hex} with submarines")
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
        IntentKind::Patrol { hex, distance } => f.write_str(&patrol_words(hex, distance)),
        IntentKind::DestroyForce {
            contact,
            distance,
            search_only,
        } => {
            let hexes = if distance == 1 { "hex" } else { "hexes" };
            let verb = if search_only { "shadow" } else { "destroy" };
            write!(
                f,
                "{verb} the force of contact {contact}, within {distance} {hexes}"
            )
        }
        IntentKind::StrikeShips { base } => write!(f, "strike ships from {base}"),
        IntentKind::SearchWaters { base } => write!(f, "search the waters from {base}"),
        IntentKind::StandAt { force, hex } => write!(f, "stand at {hex} with {force}"),
        IntentKind::OperateCarriers { force, hex } => {
            write!(f, "operate the carrier force {force} from {hex}")
        }
        IntentKind::ReadyLanding { force, hex, by_day } => {
            write!(f, "ready a landing at {hex} by {force}")?;
            match by_day {
                Some(day) => write!(f, " by the morning of day {day}"),
                None => Ok(()),
            }
        }
        IntentKind::BuildUp { base, wants } => write!(f, "build up {base}: {wants}"),
        IntentKind::Bombard { force, target } => write!(f, "bombard {target} with {force}"),
        IntentKind::TakePlace { hex, by_day } => {
            write!(f, "take the place at {hex}")?;
            match by_day {
                Some(day) => write!(f, " by the end of day {day}"),
                None => Ok(()),
            }
        }
        IntentKind::ControlIsland { island, by_day } => {
            write!(f, "control island {island}")?;
            match by_day {
                Some(day) => write!(f, " by the end of day {day}"),
                None => Ok(()),
            }
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
#[derive(Clone, PartialEq, Eq, Default)]
pub struct Reason {
    pub serves: Serves,
    pub evidence: Vec<Evidence>,
    /// The limit, the stock, the distance or the commitment that shaped it.
    pub binding: Vec<String>,
    /// The calculation.
    pub worked: String,
    /// What, if different, would give a different decision.
    pub would_change: Vec<String>,
    /// The decision rests on the commander's answer for this force's leave
    /// to pursue (decision 0214, rule 10). Intake resolves the answer's event
    /// from the side's state and makes it a cause of the order and of its
    /// reason. Added at the end.
    pub answer_of: Option<TaskForceId>,
    /// What the commander should weigh before he approves, each with the unit
    /// or place it is of, said apart from the working (decision 0217, rules 4
    /// and 5). Added at the end.
    pub cautions: Vec<Caution>,
    /// The limits the reason speaks of, where its words of what holds it
    /// back or what would change it name a limit: each with its kind and the
    /// intent it is set on (decision 0222, rule 5). Recorded with the reason;
    /// it changes no state and no hash, and nothing reads it to decide.
    /// Added at the end.
    pub limits: Vec<ReasonLimit>,
}

/// Written as the derived form was, without the limits it speaks of
/// (decision 0222, rule 5): the list is recorded beside the reason and changes
/// nothing, so every digest of a reason's debug text, which the tests of
/// earlier decisions keep, is what it was. The list is read from the field.
impl core::fmt::Debug for Reason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut out = f.debug_struct("Reason");
        out.field("serves", &self.serves)
            .field("evidence", &self.evidence)
            .field("binding", &self.binding)
            .field("worked", &self.worked)
            .field("would_change", &self.would_change)
            .field("answer_of", &self.answer_of)
            .field("cautions", &self.cautions);
        out.finish()
    }
}

/// One limit a reason speaks of (decision 0222, rule 5): its kind, and the
/// intent it is set on, which may be above the one the reason serves; for a
/// base's reserve, the base and no intent. Where the limit is set nowhere on
/// the chain, the intent is the one the commander would set it on: the root
/// of the reason's chain.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ReasonLimit {
    pub kind: LimitKind,
    pub intent: Option<IntentId>,
    pub base: Option<BaseId>,
}

/// The kind of a limit a reason speaks of (decision 0222, rule 5). The
/// eleven limits, numbered as an amendment's hash numbers them, and the
/// source of a forces limit, which the table shows apart. New kinds are
/// added at the end; a code is never renumbered.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum LimitKind {
    Forces,
    Area,
    Priority,
    Protected,
    Reserve,
    Risk,
    NotBefore,
    WaitFor,
    After,
    Knowledge,
    Targets,
    /// The source base of a forces limit.
    Source,
}

impl LimitKind {
    /// Every kind, in code order.
    pub const ALL: [LimitKind; 12] = [
        LimitKind::Forces,
        LimitKind::Area,
        LimitKind::Priority,
        LimitKind::Protected,
        LimitKind::Reserve,
        LimitKind::Risk,
        LimitKind::NotBefore,
        LimitKind::WaitFor,
        LimitKind::After,
        LimitKind::Knowledge,
        LimitKind::Targets,
        LimitKind::Source,
    ];

    /// The stable code of the kind. Never renumbered.
    pub fn code(self) -> u8 {
        match self {
            LimitKind::Forces => 1,
            LimitKind::Area => 2,
            LimitKind::Priority => 3,
            LimitKind::Protected => 4,
            LimitKind::Reserve => 5,
            LimitKind::Risk => 6,
            LimitKind::NotBefore => 7,
            LimitKind::WaitFor => 8,
            LimitKind::After => 9,
            LimitKind::Knowledge => 10,
            LimitKind::Targets => 11,
            LimitKind::Source => 12,
        }
    }

    /// The name of the kind, as `set_limit` names the limit, a saved game
    /// and the agent write it.
    pub fn name(self) -> &'static str {
        match self {
            LimitKind::Forces => "forces",
            LimitKind::Area => "area",
            LimitKind::Priority => "priority",
            LimitKind::Protected => "protected",
            LimitKind::Reserve => "reserve",
            LimitKind::Risk => "risk",
            LimitKind::NotBefore => "not_before",
            LimitKind::WaitFor => "wait_for",
            LimitKind::After => "after",
            LimitKind::Knowledge => "knowledge",
            LimitKind::Targets => "targets",
            LimitKind::Source => "source",
        }
    }

    /// The kind named `name`, where there is one.
    pub fn from_name(name: &str) -> Option<LimitKind> {
        LimitKind::ALL.into_iter().find(|k| k.name() == name)
    }
}

/// A short statement of something a commander should weigh before he
/// approves a plan, with the unit or place it is of (decision 0217, rule 4).
/// It names only what the side holds: its own forces, the enemy bases on its
/// map and its contacts.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Caution {
    pub kind: CautionKind,
    /// The unit it is of: an enemy base whose circle holds the station, or
    /// the force whose fuel was not measured.
    pub of: Option<EntityRef>,
    /// The place it is of: the centre of the circle that holds the station.
    pub hex: Option<Hex>,
    pub words: String,
}

/// What a caution says (decision 0217, rule 4). New kinds are added at the
/// end; a code is never renumbered.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum CautionKind {
    /// The station chosen lies inside the circle of an enemy base or of a
    /// contact.
    StationInDanger,
    /// The force has no home port the staff knows, so its fuel was not
    /// measured against a passage home.
    FuelNotMeasured,
    /// The station chosen lies inside submarine waters the side holds, no
    /// place outside them serving (decision 0218, rule 55).
    StationInSubmarineWaters,
}

impl CautionKind {
    /// The stable code of the kind. Never renumbered.
    pub fn code(self) -> u8 {
        match self {
            CautionKind::StationInDanger => 1,
            CautionKind::FuelNotMeasured => 2,
            CautionKind::StationInSubmarineWaters => 3,
        }
    }

    /// The name of the kind, as a saved game and the agent write it.
    pub fn name(self) -> &'static str {
        match self {
            CautionKind::StationInDanger => "station_in_danger",
            CautionKind::FuelNotMeasured => "fuel_not_measured",
            CautionKind::StationInSubmarineWaters => "station_in_submarine_waters",
        }
    }

    /// The kind named `name`, where there is one.
    pub fn from_name(name: &str) -> Option<CautionKind> {
        [
            CautionKind::StationInDanger,
            CautionKind::FuelNotMeasured,
            CautionKind::StationInSubmarineWaters,
        ]
        .into_iter()
        .find(|k| k.name() == name)
    }
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
    /// A "ready a landing" task is ready (decision 0141, 5). Handed in once.
    /// Added at the end.
    Ready,
    /// A "ready a landing" task cannot be ready by its date (decision 0141,
    /// 6): its reason gives the days late and what binds. Handed in once.
    /// Added at the end.
    Late,
    /// Every want of a "build up a base" task is met (decision 0141, 11).
    /// Handed in once. Added at the end.
    Met,
    /// A "take a place" cannot bring the men it wants on any day in reach
    /// (decision 0159, 7): its reason gives the men wanted, the men to hand
    /// and any unit coming with its day. The sibling of `CannotMeet`. Handed
    /// in once. Added at the end.
    CannotBringEnough,
    /// The staff turned a lift of a "take a place" back to its source for
    /// enemy carriers near its way (decision 0191, 9): its reason gives the
    /// contact, its kind, age and distance, and the risk. Handed in on the
    /// morning it is turned back, not on the mornings it is kept back. Added
    /// at the end.
    LiftTurnedBack,
    /// A carrier force asks leave to pursue a find (decision 0214, 6): the
    /// notice's `pursuit` gives the force, the contact, where the force
    /// would go and the order it would leave. Added at the end (decision
    /// 0214).
    AsksLeaveToPursue,
    /// A carrier force pursues a find on leave the staff took (decision
    /// 0214, 10). Stops nothing. Added at the end (decision 0214).
    Pursues,
    /// A force's pursuit is ended (decision 0214, 12). Added at the end
    /// (decision 0214).
    PursuitEnded,
    /// An order to destroy a force follows its contact, refreshed or taken
    /// anew (decision 0220, 8 and R3): the notice's `follow` says. Only a
    /// taking of the other sort may stop the clock. Added at the end.
    Follows,
    /// An order to destroy a force is lost-contact (decision 0220, 9). Handed
    /// in once. Added at the end.
    ContactLost,
    /// An order to destroy a force has found nothing in `lost_contact_days`
    /// and is ended (decision 0220, 10). Added at the end.
    ForceNotFound,
}

impl NoticeKind {
    /// A stable number. Never renumber; add new values at the end.
    pub const fn code(self) -> u8 {
        match self {
            NoticeKind::CannotMeet => 1,
            NoticeKind::WaitsForLeave(Leave::Prepare) => 2,
            NoticeKind::WaitsForLeave(Leave::Launch) => 3,
            NoticeKind::RunEnded => 4,
            NoticeKind::Ready => 5,
            NoticeKind::Late => 6,
            NoticeKind::Met => 7,
            NoticeKind::CannotBringEnough => 8,
            NoticeKind::LiftTurnedBack => 9,
            NoticeKind::AsksLeaveToPursue => 10,
            NoticeKind::Pursues => 11,
            NoticeKind::PursuitEnded => 12,
            NoticeKind::Follows => 13,
            NoticeKind::ContactLost => 14,
            NoticeKind::ForceNotFound => 15,
        }
    }

    /// The kind's name as saved games and the tools write it.
    pub const fn name(self) -> &'static str {
        match self {
            NoticeKind::CannotMeet => "cannot_meet",
            NoticeKind::WaitsForLeave(Leave::Prepare) => "waits_for_leave_to_prepare",
            NoticeKind::WaitsForLeave(Leave::Launch) => "waits_for_leave_to_launch",
            NoticeKind::RunEnded => "run_ended",
            NoticeKind::Ready => "ready",
            NoticeKind::Late => "late",
            NoticeKind::Met => "met",
            NoticeKind::CannotBringEnough => "cannot_bring_enough",
            NoticeKind::LiftTurnedBack => "lift_turned_back",
            NoticeKind::AsksLeaveToPursue => "asks_leave_to_pursue",
            NoticeKind::Pursues => "pursues",
            NoticeKind::PursuitEnded => "pursuit_ended",
            NoticeKind::Follows => "follows",
            NoticeKind::ContactLost => "contact_lost",
            NoticeKind::ForceNotFound => "force_not_found",
        }
    }

    /// The kind of that name.
    pub fn from_name(name: &str) -> Option<NoticeKind> {
        [
            NoticeKind::CannotMeet,
            NoticeKind::WaitsForLeave(Leave::Prepare),
            NoticeKind::WaitsForLeave(Leave::Launch),
            NoticeKind::RunEnded,
            NoticeKind::Ready,
            NoticeKind::Late,
            NoticeKind::Met,
            NoticeKind::CannotBringEnough,
            NoticeKind::LiftTurnedBack,
            NoticeKind::AsksLeaveToPursue,
            NoticeKind::Pursues,
            NoticeKind::PursuitEnded,
            NoticeKind::Follows,
            NoticeKind::ContactLost,
            NoticeKind::ForceNotFound,
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
    /// What a pursuit notice says (decision 0214): `Some` for the three
    /// pursuit kinds, `None` for the rest. Added at the end.
    pub pursuit: Option<PursuitNotice>,
    /// What a notice of following says (decision 0220): `Some` for the three
    /// kinds of following, `None` for the rest. Added at the end.
    pub follow: Option<FollowNotice>,
}

/// What a pursuit notice says (decision 0214, 6): the force, the order it
/// would leave, and the find.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PursuitNotice {
    pub force: TaskForceId,
    /// The order the force would leave.
    pub intent: IntentId,
    /// `Some` for the asking and for "pursues"; `None` for "pursuit ended".
    pub find: Option<PursuitFind>,
}

/// The find of a pursuit notice (decision 0214, 6).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PursuitFind {
    /// The commander's number of the contact.
    pub contact: u32,
    pub kind: ShipKind,
    /// Its age in pulses.
    pub age: u32,
    /// Where it was reported.
    pub reported_at: Hex,
    /// The hex the force would close to.
    pub to: Hex,
    /// The hours to it by the engine's forecast, where it has one.
    pub hours: Option<Fixed>,
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

    /// What must be known before each "take a place" of the side whose plan
    /// is preparing this morning goes, worked from the view as the staff
    /// works it this morning and stored nowhere (decision 0159, 13). Empty
    /// by default: a staff with no doctrine for a take asks nothing.
    fn needs_to_know(&self, _view: SideView<'_>) -> Vec<NeedToKnow> {
        Vec::new()
    }

    /// The plan order of each intent of the commander's in force, and of
    /// each "take a place" in force that the staff formed (decision 0164),
    /// in number order: the mission, the phases, what the plan waits for,
    /// what it needs and has, the day it expects to be ashore, the need to
    /// know and the tasks (decision 0159, rule E). Worked from the view as
    /// the staff works this morning and stored nowhere. Empty by default.
    fn plan_orders(&self, _view: SideView<'_>) -> Vec<PlanOrder> {
        Vec::new()
    }
}

/// A plan handed in as an order (decision 0159, rule E, items 15 to 17): for
/// one intent of the commander's, or one "take a place" the staff formed
/// (decision 0164), what the staff already works out, as data. It is worked
/// when asked, from the side's command state and the staff's own sums, and is
/// not state: not hashed, not saved. The plan's state, its phases and its
/// tasks are read from the command state as the morning finds it, before that
/// morning's decisions; what it waits for, the men, the day ashore and the
/// need to know are the staff's working of that morning.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PlanOrder {
    pub intent: IntentId,
    /// The intent in words, as the staff's reasons state it.
    pub mission: String,
    pub plan: PlanId,
    pub plan_state: PlanState,
    /// The phases in order: a take's prepare, move and land; a hold's wait
    /// and hold, or hold alone where the base is the side's. Empty for the
    /// other kinds.
    pub phases: Vec<Phase>,
    /// What must be so before the phase under way can end, where the staff
    /// waits on it. Empty once a take's plan is released.
    pub waits_for: Vec<Waited>,
    /// The men wanted and the men to hand, where the side holds a count of
    /// the enemy at the place; `None` otherwise.
    pub men: Option<MenCount>,
    /// The day the take expects its first men ashore at the place, by the
    /// sums it uses for a latest day; `None` where it cannot work one (a thing
    /// waited for has no day) or where its men are already ashore.
    pub expected_ashore: Option<u32>,
    /// The need to know of a take whose plan is preparing, as
    /// [`Staff::needs_to_know`] gives it.
    pub need_to_know: Option<NeedToKnow>,
    /// The staff's intents under it, each with its phase.
    pub tasks: Vec<OrderTask>,
}

/// The name of a phase of a plan order (decision 0159, 16).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum PhaseName {
    Prepare,
    Move,
    Land,
    Wait,
    Hold,
}

impl PhaseName {
    /// The name as the tools write it.
    pub const fn name(self) -> &'static str {
        match self {
            PhaseName::Prepare => "prepare",
            PhaseName::Move => "move",
            PhaseName::Land => "land",
            PhaseName::Wait => "wait",
            PhaseName::Hold => "hold",
        }
    }
}

/// The condition that ends a phase.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PhaseEnd {
    /// The plan is released.
    Released,
    /// The first man of the side is ashore at the place.
    FirstManAshore(Hex),
    /// The place is the side's: it holds a base there and the enemy none.
    PlaceIsTheSides(Hex),
    /// The base is the side's.
    BaseIsTheSides(BaseId),
    /// It does not end.
    Never,
}

/// Where a phase stands.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PhaseStage {
    Done,
    UnderWay,
    ToCome,
}

impl PhaseStage {
    /// The word as the tools write it.
    pub const fn name(self) -> &'static str {
        match self {
            PhaseStage::Done => "done",
            PhaseStage::UnderWay => "under_way",
            PhaseStage::ToCome => "to_come",
        }
    }
}

/// One phase of a plan order: its name, what ends it and where it stands.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Phase {
    pub name: PhaseName,
    pub ends: PhaseEnd,
    pub stage: PhaseStage,
}

/// One thing a plan waits for (decision 0159, 15), whether it is met, and the
/// day it is expected to be met where the staff can work one.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Waited {
    pub thing: WaitedFor,
    pub met: bool,
    pub expected_day: Option<u32>,
}

/// The kinds of thing a plan waits for.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WaitedFor {
    /// The men wanted, where the first lift would sail short of them
    /// (decision 0159, rule B).
    Men { wanted: u32, to_hand: u32 },
    /// A leave the commander withholds.
    Leave(Leave),
    /// The commander's limit "not before" this day.
    Day(u32),
    /// The commander's limit "wait for" a task force or a land unit.
    Force(Awaited),
    /// The commander's limit "after" another intent.
    Intent(IntentId),
    /// The count asked before the landing goes (decision 0159, 12).
    Knowledge(Knowledge),
}

/// The men a take wants against the side's count of the enemy, and the men
/// it has to hand for them.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MenCount {
    pub wanted: u32,
    pub to_hand: u32,
}

/// A task under a plan order: a staff intent in force, its kind and the phase
/// it serves.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct OrderTask {
    pub intent: IntentId,
    pub kind: IntentKind,
    pub phase: Option<PhaseName>,
}

/// What must be known of the enemy at a place before a landing goes, and
/// what is held (decision 0159, 13), for one plan of a "take a place". It is
/// worked when asked and is not state: not hashed, not saved. Nothing of the
/// enemy is in it but the side's own estimate.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NeedToKnow {
    pub plan: PlanId,
    /// The "take a place" whose plan it is.
    pub intent: IntentId,
    pub place: Hex,
    /// What is asked: the commander's "knowledge" limit, or what the risk
    /// sets where it is not set.
    pub asked: Knowledge,
    /// The intent the "knowledge" limit is set on; `None` where the risk
    /// sets what is asked.
    pub asked_from: Option<IntentId>,
    /// The side's estimate of the enemy's men at the place; `None` where it
    /// holds none.
    pub held: Option<HeldCount>,
    /// The air group looking at the place after this morning's orders, and
    /// since which day; `None` where none is.
    pub looking: Option<Looking>,
    /// The carrier force the staff moves, or has moved, to a hex from which
    /// one of its groups can look at the place, while the need is not met
    /// (decision 0159, 14); `None` where none is.
    pub moved_to_look: Option<MovedToLook>,
    /// Whether what is held meets what is asked.
    pub met: bool,
}

/// A carrier force the staff moves to a hex in order to look at a take's
/// place (decision 0159, 14): the force of its task "operate the carrier
/// force from a hex", and that hex.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MovedToLook {
    pub task_force: TaskForceId,
    pub hex: Hex,
}

/// The count the side holds of the enemy at a place, its day and age, and
/// what it could be in truth (decision 0159, 13).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct HeldCount {
    pub men: u32,
    pub grade: crate::belief::GroundGrade,
    /// The day of the report it rests on.
    pub day: u32,
    /// Its age in days on the morning: the morning's day less its day.
    pub age_days: u32,
    /// What the count could be in truth, from its grade's error range in
    /// the ground figures; `None` for a count that was told, or where the
    /// figures give no range for the grade.
    pub could_be: Option<CountRange>,
}

/// What a count could be in truth: between the count over the high end of
/// its error range, cut to a whole man, and the count over the low end,
/// raised to a whole man (decision 0159, 13). `highest` is `None` where the
/// low end is 0: the count then bounds nothing above.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CountRange {
    pub lowest: u32,
    pub highest: Option<u32>,
}

/// An air group looking at a place, and the day it was set to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Looking {
    pub air_group: AirGroupId,
    /// The day the order that set it to look was given (today for a group
    /// the staff sends this morning); `None` where no order of the side set
    /// it (a mission the scenario gave at the start).
    pub since_day: Option<u32>,
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
    /// For a patrol order (decision 0218, 54): each boat the staff would
    /// give a station, with that station and her place in the order, in
    /// place order. A boat that keeps her station is here; a boat passed
    /// over is not. Empty for every other intent.
    pub stations: Vec<PatrolStation>,
}

/// One boat's place and station under a patrol order, as the staff works
/// them (decision 0218, 54).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PatrolStation {
    pub boat: SubmarineId,
    /// The station the boat is given.
    pub hex: Hex,
    /// Her place among the boats the order may use, counted from 1: the
    /// boat of place n has station n of the waters.
    pub place: u32,
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
