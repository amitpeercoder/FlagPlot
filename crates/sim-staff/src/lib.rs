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

use std::collections::BTreeMap;

use sim_core::command::{Proposal, Staff, StaffDecisions, Staffs};
use sim_core::ids::IntentId;
use sim_core::state::Side;
use sim_core::view::SideView;

/// The version of the doctrine. A change to what a staff decides from a
/// view changes it. A saved game records it; a game is resumed for more
/// play only under the version it was begun under.
pub const DOCTRINE_VERSION: &str = "supply-1";

/// The staff of stage 1: the doctrine for keeping a base supplied. A side
/// with no intent in force and no commitment to free gets nothing from it.
#[derive(Clone, Copy, Debug, Default)]
pub struct SupplyStaff;

impl Staff for SupplyStaff {
    fn decide(&self, _view: SideView<'_>) -> StaffDecisions {
        StaffDecisions::default()
    }

    fn propose(&self, _view: SideView<'_>, _intent: IntentId) -> Option<Proposal> {
        None
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
