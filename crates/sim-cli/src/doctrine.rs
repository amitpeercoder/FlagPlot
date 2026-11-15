//! The staff a runner passes to the engine, and the version of the doctrine
//! that gives it (decision 0107, "As built").
//!
//! Every loop that resolves days (`run`, `batch`, `export`, `hash`, `time`,
//! `replay --check-staff`, and a game held open for play) takes its staffs
//! from [`staffs_for`], so that the runner and the replay check ask the same
//! staff. A saved game records [`DOCTRINE_VERSION`], and a game is resumed
//! (`play --game`) only under the version it was begun under.
//!
//! There is no doctrine yet, so the version is `"none"` and the staff of
//! either side is none. The second team of step 3, stage 1 (the `sim-staff`
//! crate) moves the version into `sim-staff` and replaces [`staffs_for`] with
//! the doctrine's own: the version and the staff a runner passes then come
//! from one place in `sim-staff`.

use std::collections::BTreeMap;

use sim_core::command::{Staff, Staffs};
use sim_core::state::Side;
use sim_data::Scenario;

/// The version of the doctrine whose staff is passed in. A change to what a
/// staff decides from a view changes it. `"none"` is the runner with no staff
/// for either side.
pub const DOCTRINE_VERSION: &str = "none";

/// The staff for each side that has one, for a run of `scenario`. None for
/// either side for now.
pub fn staffs_for(_scenario: &Scenario) -> BTreeMap<Side, Box<dyn Staff + Send>> {
    BTreeMap::new()
}

/// The engine's [`Staffs`] over staffs held in a map, as [`staffs_for`] gives
/// them.
pub fn as_staffs(held: &BTreeMap<Side, Box<dyn Staff + Send>>) -> Staffs<'_> {
    let mut staffs = Staffs::none();
    for (side, staff) in held {
        staffs = staffs.with(*side, staff.as_ref());
    }
    staffs
}
