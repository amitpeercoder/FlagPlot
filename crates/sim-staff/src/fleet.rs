//! The side's ships and forces as the staff reads them, and what a run of a
//! plan is doing: loading, under way or finished. Every answer is from the
//! view (the state of the ships, the forces and the command state), never
//! from a note kept by the staff.

use std::collections::{BTreeMap, BTreeSet};

use sim_core::command::{CommandState, ForcesLimit, HoldTarget, PlanId, Run};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, ShipId, TaskForceId};
use sim_core::state::{Ship, ShipLocation, TaskForce, TaskForceStatus};
use sim_core::view::SideView;

/// The side's ships and forces, looked up by number.
pub(crate) struct Fleet<'v> {
    pub(crate) ships: BTreeMap<ShipId, &'v Ship>,
    pub(crate) forces: BTreeMap<TaskForceId, &'v TaskForce>,
}

impl<'v> Fleet<'v> {
    pub(crate) fn of(view: &SideView<'v>) -> Fleet<'v> {
        Fleet {
            ships: view.ships().map(|ship| (ship.id, ship)).collect(),
            forces: view.task_forces().map(|force| (force.id, force)).collect(),
        }
    }

    /// The force a ship is in, if it is in one.
    pub(crate) fn force_of(&self, ship: ShipId) -> Option<&'v TaskForce> {
        match self.ships.get(&ship)?.location {
            ShipLocation::TaskForce(force) => self.forces.get(&force).copied(),
            _ => None,
        }
    }

    /// The hold that keeps `ship` from the staff, if any: on the ship, or on
    /// the force it lies in or any ship of that force (ruling 18).
    pub(crate) fn hold_on_ship(&self, state: &CommandState, ship: ShipId) -> Option<HoldTarget> {
        if state.holds.is_empty() {
            return None;
        }
        state.hold_on_ship(ship, self.force_of(ship).map(|f| (f.id, &f.ships)))
    }

    /// Whether the ship is afloat: not sunk.
    pub(crate) fn afloat(&self, ship: ShipId) -> bool {
        self.ships
            .get(&ship)
            .is_some_and(|ship| ship.location != ShipLocation::Sunk)
    }

    /// The supply aboard `ships` together.
    pub(crate) fn supply_aboard(&self, ships: &[ShipId]) -> Fixed {
        ships
            .iter()
            .filter_map(|id| self.ships.get(id))
            .fold(Fixed::ZERO, |sum, ship| sum + ship.supply_t)
    }
}

/// Whether the force is stopped at the centre of `hex` with no route left to
/// follow: idle or arrived, not between hexes, not at the yard.
pub(crate) fn force_stopped_at(force: &TaskForce, hex: Hex) -> bool {
    force.from_hex == hex
        && force.toward_hex.is_none()
        && force.progress_nm.is_zero()
        && matches!(
            force.status,
            TaskForceStatus::Idle | TaskForceStatus::Arrived
        )
        && force.yard_trip.is_none()
        && force.route.next_index >= force.route.waypoints.len()
}

/// Whether the ship lies at the source: in port there in no force, or in a
/// force stopped at the source's hex.
pub(crate) fn ship_at_source(fleet: &Fleet<'_>, ship: ShipId, source: BaseId, hex: Hex) -> bool {
    match fleet.ships.get(&ship).map(|ship| ship.location) {
        Some(ShipLocation::Port(base)) => base == source,
        Some(ShipLocation::TaskForce(force)) => fleet
            .forces
            .get(&force)
            .is_some_and(|force| force_stopped_at(force, hex)),
        _ => false,
    }
}

/// What a run is doing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum RunStatus {
    /// Its ships are at the source, loaded or not, and have not sailed.
    Loading,
    /// Its ships are away from the source.
    Underway,
    /// Its ships are back at the source, stopped and empty, after its sail
    /// day: nothing of the run is left to do.
    Finished,
}

/// A run with what the view says of it.
#[derive(Clone, Debug)]
pub(crate) struct RunState {
    pub(crate) status: RunStatus,
    /// The run's ships that are afloat, in ship order.
    pub(crate) afloat: Vec<ShipId>,
    /// The supply aboard them.
    pub(crate) aboard_t: Fixed,
}

/// Whether a later run (of any plan) has taken one of `run`'s ships: a ship
/// serves one run at a time, so the earlier run is over whatever the ship is
/// doing now.
fn superseded(state: &CommandState, plan: PlanId, run: &Run) -> bool {
    state.plans.values().any(|other| {
        other.runs.values().any(|later| {
            (later.days.load, other.id, later.number) > (run.days.load, plan, run.number)
                && later.ships.iter().any(|ship| run.ships.contains(ship))
        })
    })
}

/// The state of `run` of `plan`, whose source is at `source_hex`, on
/// `today`. A run a later run has taken the ships of is finished, and so is
/// a run that has ended and a run of a plan that has ended (rulings 20 and
/// 29): such a run holds no ship, is pending no load and is counted by
/// nothing. [`run_movement`] says what its ships are doing all the same.
pub(crate) fn run_state(
    fleet: &Fleet<'_>,
    state: &CommandState,
    plan: PlanId,
    run: &Run,
    source_hex: Hex,
    today: u32,
) -> RunState {
    let mut found = run_movement(fleet, state, plan, run, source_hex, today);
    if run.ended.is_some() || state.plans.get(&plan).is_some_and(|p| p.state.ended()) {
        found.status = RunStatus::Finished;
    }
    found
}

/// What a run's ships are doing, whether or not the run or its plan has
/// ended: the true movement, for the fallback of a cancelled plan.
pub(crate) fn run_movement(
    fleet: &Fleet<'_>,
    state: &CommandState,
    plan: PlanId,
    run: &Run,
    source_hex: Hex,
    today: u32,
) -> RunState {
    let afloat: Vec<ShipId> = run
        .ships
        .iter()
        .copied()
        .filter(|ship| fleet.afloat(*ship))
        .collect();
    let aboard_t = fleet.supply_aboard(&afloat);
    let at_source = afloat
        .iter()
        .all(|ship| ship_at_source(fleet, *ship, run.source, source_hex));
    let status = if afloat.is_empty() || superseded(state, plan, run) {
        RunStatus::Finished
    } else if !at_source {
        RunStatus::Underway
    } else if today > run.days.sail && aboard_t.is_zero() {
        RunStatus::Finished
    } else {
        RunStatus::Loading
    };
    RunState {
        status,
        afloat,
        aboard_t,
    }
}

/// The ships held by a run not finished, of any plan of the side.
pub(crate) fn held_ships(
    view: &SideView<'_>,
    fleet: &Fleet<'_>,
    state: &CommandState,
    today: u32,
) -> BTreeSet<ShipId> {
    let mut held = BTreeSet::new();
    for plan in state.plans.values() {
        for run in plan.runs.values() {
            let Some(base) = view.bases().find(|base| base.id == run.source) else {
                // A source no longer the side's holds its ships no more.
                continue;
            };
            let st = run_state(fleet, state, plan.id, run, base.hex, today);
            if st.status != RunStatus::Finished {
                held.extend(run.ships.iter().copied());
            }
        }
    }
    held
}

/// The ships of the side afloat that the forces limit gives to the staff:
/// those it lists and those in a force it lists.
pub(crate) fn pool_ships(fleet: &Fleet<'_>, forces: &ForcesLimit) -> BTreeSet<ShipId> {
    fleet
        .ships
        .values()
        .filter(|ship| ship.location != ShipLocation::Sunk)
        .filter(|ship| {
            forces.ships.contains(&ship.id)
                || matches!(
                    ship.location,
                    ShipLocation::TaskForce(force) | ShipLocation::Yard(force)
                        if forces.task_forces.contains(&force)
                )
        })
        .map(|ship| ship.id)
        .collect()
}
