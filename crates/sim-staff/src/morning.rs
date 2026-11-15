//! What the staff has handed in for a higher intent this morning (decision
//! 0107, rulings 11 to 13 and 16). One `decide` call serves the intents in
//! priority order; each `serve` reads the command state, which does not yet
//! hold what the same call has handed in for an intent served before it. The
//! [`Morning`] holds those runs, so that a ship and the tons at the source
//! are committed to the first intent that takes them and counted as such for
//! every intent after it. It is no state: it lives for one call.

use std::collections::{BTreeMap, BTreeSet};

use sim_core::command::{CommandState, Intent, PlanId, RunDays};
use sim_core::fixed::Fixed;
use sim_core::ids::{BaseId, IntentId, ShipId};

/// A run added this morning, for the intent served before the one now
/// being served.
#[derive(Clone, Debug)]
pub(crate) struct TakenRun {
    pub(crate) intent: IntentId,
    pub(crate) plan: PlanId,
    /// The run's number in its plan.
    pub(crate) run: u32,
    pub(crate) ships: BTreeSet<ShipId>,
    pub(crate) source: BaseId,
    pub(crate) load_t: Fixed,
    pub(crate) days: RunDays,
}

/// The runs of this morning.
#[derive(Clone, Debug, Default)]
pub(crate) struct Morning {
    pub(crate) runs: Vec<TakenRun>,
}

impl Morning {
    pub(crate) fn push(&mut self, run: TakenRun) {
        self.runs.push(run);
    }

    /// The run of this morning that holds `ship`, if any.
    pub(crate) fn holding(&self, ship: ShipId) -> Option<&TakenRun> {
        self.runs.iter().find(|run| run.ships.contains(&ship))
    }

    /// The tons the runs of this morning at `source` have yet to take:
    /// nothing is aboard yet.
    pub(crate) fn pending_at(&self, source: BaseId) -> Fixed {
        self.runs
            .iter()
            .filter(|run| run.source == source)
            .fold(Fixed::ZERO, |sum, run| sum + run.load_t)
    }
}

/// The intents in force in the order the staff serves them: ranked first
/// (rank 1 first), then those with no rank, each by number (ruling 10).
pub(crate) fn priority_order(state: &CommandState) -> Vec<&Intent> {
    let mut intents: Vec<&Intent> = state.intents_in_force().collect();
    intents.sort_by_key(|intent| {
        (
            intent.limits.priority.is_none(),
            intent.limits.priority.unwrap_or(0),
            intent.id,
        )
    });
    intents
}

/// The ships protected for an intent in force other than `intent`, each
/// with the first such intent (ruling 16).
pub(crate) fn protected_for_others(
    state: &CommandState,
    intent: IntentId,
) -> BTreeMap<ShipId, IntentId> {
    let mut found = BTreeMap::new();
    for other in state.intents_in_force().filter(|other| other.id != intent) {
        for ship in &other.limits.protected {
            found.entry(*ship).or_insert(other.id);
        }
    }
    found
}
