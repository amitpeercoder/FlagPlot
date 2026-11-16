//! Flag Plot staff: what a side's staff decides, from its view alone
//! (decision 0107, step 3, stage 1).
//!
//! `sim-core` defines what a staff is ([`sim_core::command::Staff`]); this
//! crate provides the one there is, [`SupplyStaff`], with the doctrine for
//! keeping a base supplied. Every function here takes a
//! [`sim_core::view::SideView`] (and plain numbers beside it) and nothing
//! else: no world, no umpire's view, no full event log. A test reads the
//! source to check. Floating point is forbidden, as in `sim-core`, because
//! the staff's decisions reach the hash.
//!
//! The staff holds no state of its own: asked twice on the same view it
//! gives the same answer. What it remembers is in the side's command state,
//! through the staff commands it hands in.
//!
//! Every loop that resolves days takes its staffs from [`staffs`], so that a
//! runner and `replay --check-staff` ask the same staff. A saved game
//! records [`DOCTRINE_VERSION`].

#![deny(clippy::float_arithmetic)]

mod air;
mod ammunition;
pub mod carrier;
mod deny;
mod doctrine;
mod escort;
mod fleet;
mod handin;
mod morning;
mod neutralise;
mod planner;
mod projection;
mod proposal;
pub mod raid;
mod reasons;
mod route;
pub mod search;
pub mod stand;
pub mod strike;
mod units;

pub use projection::project;

use std::collections::BTreeMap;

use sim_core::command::{Proposal, Staff, StaffDecisions, Staffs};
use sim_core::ids::IntentId;
use sim_core::state::Side;
use sim_core::view::SideView;

/// The version of the doctrine. A change to what a staff decides from a
/// view changes it. A saved game records it; a game is resumed for more
/// play only under the version it was begun under. `supply-2` adds the
/// priority order with what is handed in this morning counted as committed,
/// the protected limit, and displaced demand. `supply-3` adds holds, runs
/// set aside, cargo aboard carried, and the fallback of a cancelled plan.
/// `supply-4` adds the notices: cannot meet, waits for a leave, a run ended.
/// `supply-5` hands in each reason with its intent's statement in words, and
/// names a plan with what it is for where a reason's working names no base.
/// `supply-6` counts the fuelling hours at the source before it calls a ship
/// free (decision 0112): the morning after the day it has finished fuelling,
/// at the source's fuel berth rate or lighter rate; with no rate given, as
/// before, the morning after it is back.
/// `supply-7` frees the ships of a plan that has ended first in the
/// morning, before any intent is served, where it freed them last; an intent
/// served that morning may then commit them (found by the soak ashore,
/// scenario 45, decision 0093 item 10).
/// `supply-8` adds the air doctrine after the supply doctrine (decisions
/// 0115 and 0117): the leaves for a hold or cover intent, forming cover from
/// the air under a held base, and each cover intent's patrol, search and rest
/// orders for the air groups at its base. A side with no hold or cover
/// intent gets exactly what `supply-7` gave.
/// `supply-9` adds decision 0120: the neutralise doctrine at the area's
/// level, which forms raid tasks and gives no order, and the raid doctrine at
/// the base's level; a group a raid task may use as a bomber is the raid's
/// and not the cover's; every reason says which level decided. A side with
/// no neutralise or raid intent gets the orders `supply-8` gave.
/// `supply-10` adds decision 0123: the deny doctrine at the area's level,
/// which forms strike-ships and search-waters tasks, the strike doctrine at
/// the base's level with each strike's sizing decided at the group's, the
/// search doctrine, and a cover's arc that takes the waters as threats. A side
/// with no deny, strike-ships or search-waters intent gets the orders
/// `supply-9` gave.
/// `supply-11` adds decisions 0124 and 0125: a task force as an addressee,
/// the stand and carrier tasks formed under a deny or a hold intent whose
/// forces limit names task forces, and their doctrines at the force's level.
/// A side whose intents name no task force gets the orders `supply-10` gave.
/// `supply-12` adds decision 0126: under a keep-supplied intent whose forces
/// limit names task forces, the threat to a run, an escort from the named
/// forces on the escort mission by the risk level, the escort's charge given
/// and cleared, and a stand task at the destination for each named surface
/// force. An intent that names no task force gets the orders `supply-11`
/// gave.
/// `supply-13` adds decision 0127: a surface force short of ammunition by its
/// risk level does not sortie and is not a free escort, with the day from
/// which it may be used read with its fuel. A force short of nothing gets
/// the orders `supply-12` gave.
/// `supply-14` adds the architect's ruling of Nov 16, 2026, on decisions
/// 0124, 8, and 0125, 2: the staff forms no stand task and no carrier task
/// for a force that already has a task of that kind in force, whoever made
/// it and wherever it stands. A side with no such task stated beside an end
/// that names the force gets the orders `supply-13` gave.
/// `supply-15` adds decision 0130: a carrier force is routed at cruise where
/// that brings it there in the night and at full speed where it does not,
/// and closes where the first fresh contact of a kind it wants within a
/// night and its bombers' reach is beyond that reach, stays while it is in
/// reach and goes back to its hex when there is none; the passage home for
/// fuel is at cruise, and a ship the engine's measure has run dry short of
/// a passage is short of the fuel for it, in the carrier and stand doctrines.
/// `supply-16` adds decision 0131: a search is flown once a day at preserve
/// the force and at balanced (in both pulses at press) by the cover, the
/// search of waters and the carrier force; the least tired bomber group
/// that can search and fly takes the search each morning on a carrier force
/// with no group that can only search, and at a base whose cover or search
/// task has none where a strike or raid task there has two or more; and a
/// carrier force with a ship out of fuel is given no route.
pub const DOCTRINE_VERSION: &str = "supply-16";

/// The staff: the doctrine for keeping a base supplied (stage 1) and, after
/// it, the air doctrine for holding a place and covering a base from the air
/// (step 2 of stage 2). A side with no intent in force and no commitment to
/// free gets nothing from it; one with no hold or cover intent gets nothing
/// from the air doctrine.
#[derive(Clone, Copy, Debug, Default)]
pub struct SupplyStaff;

impl Staff for SupplyStaff {
    fn decide(&self, view: SideView<'_>) -> StaffDecisions {
        let mut decisions = doctrine::decide(view);
        reasons::state_intents(&view, &mut decisions);
        // The air doctrine states its own reasons on the state it planned.
        decisions.append(air::decide(&view));
        decisions
    }

    fn propose(&self, view: SideView<'_>, intent: IntentId) -> Option<Proposal> {
        if view
            .command_state()
            .intents
            .get(&intent)
            .is_some_and(|found| air::is_air(found.kind))
        {
            return air::propose(&view, intent);
        }
        let mut proposal = proposal::propose(view, intent)?;
        reasons::state_intents_of_proposal(&view, &mut proposal);
        Some(proposal)
    }
}

/// The staff of each side: the supply staff for both. A side with no intent
/// has a staff that does nothing.
pub fn staffs() -> BTreeMap<Side, Box<dyn Staff + Send>> {
    Side::ALL
        .into_iter()
        .map(|side| (side, Box::new(SupplyStaff) as Box<dyn Staff + Send>))
        .collect()
}

/// The engine's [`Staffs`] over staffs held in a map, as [`staffs`] gives
/// them.
pub fn as_staffs(held: &BTreeMap<Side, Box<dyn Staff + Send>>) -> Staffs<'_> {
    let mut staffs = Staffs::none();
    for (side, staff) in held {
        staffs = staffs.with(*side, staff.as_ref());
    }
    staffs
}
