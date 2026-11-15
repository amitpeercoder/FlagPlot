//! What the staff has handed in for a higher intent this morning (decision
//! 0107, rulings 11 to 13 and 16). One `decide` call serves the intents in
//! priority order; each `serve` reads the command state, which does not yet
//! hold what the same call has handed in for an intent served before it. The
//! [`Morning`] holds those runs, so that a ship and the tons at the source
//! are committed to the first intent that takes them and counted as such for
//! every intent after it. It is no state: it lives for one call.

use std::collections::{BTreeMap, BTreeSet};

use sim_core::command::{CommandKind, CommandState, Hold, HoldTarget, Intent, PlanId, RunDays};
use sim_core::fixed::Fixed;
use sim_core::ids::{BaseId, IntentId, ShipId};
use sim_core::view::SideView;

use crate::fleet::{run_state, Fleet, RunStatus};
use crate::handin::Out;
use crate::reasons::{hold_words, serving_intent};
use crate::units::tons;

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
    /// The tons the run has still to take at its source (not the supply
    /// already aboard its ships).
    pub(crate) load_t: Fixed,
    pub(crate) days: RunDays,
}

/// A run set aside this morning because one of its ships is held and it has
/// not sailed (rulings 19 and 20).
#[derive(Clone, Debug)]
pub(crate) struct SetAside {
    pub(crate) plan: PlanId,
    pub(crate) run: u32,
    /// Where the run's ships are committed to the plan, to be freed.
    pub(crate) freed: BTreeSet<ShipId>,
    pub(crate) target: HoldTarget,
    pub(crate) hold: Hold,
    /// The supply aboard the run's ships, which stays aboard.
    pub(crate) aboard_t: Fixed,
}

/// The runs of this morning, and the command state as the staff reads it
/// this morning: with the runs set aside counted ended and their ships
/// uncommitted.
#[derive(Clone, Debug, Default)]
pub(crate) struct Morning {
    pub(crate) runs: Vec<TakenRun>,
    pub(crate) aside: Vec<SetAside>,
    state: CommandState,
}

impl Morning {
    /// The start of a morning: every approved plan's run that has not
    /// sailed and has a held ship is set aside, in the command state the
    /// staff reads from here on. The commands that say so are
    /// [`Morning::hand_in_aside`]. `decide`, a proposal and the projection
    /// all begin here, so that all see the same morning.
    pub(crate) fn begin(view: &SideView<'_>, fleet: &Fleet<'_>) -> Morning {
        let state = view.command_state();
        let today = view.day() + 1;
        let mut effective = state.clone();
        let mut aside = Vec::new();
        if !state.holds.is_empty() {
            for plan in state.plans.values().filter(|plan| plan.state.approved()) {
                for run in plan.runs.values() {
                    let Some(base) = view.bases().find(|base| base.id == run.source) else {
                        continue;
                    };
                    let found = run_state(fleet, state, plan.id, run, base.hex, today);
                    if found.status != RunStatus::Loading {
                        continue;
                    }
                    let Some(target) = found
                        .afloat
                        .iter()
                        .find_map(|ship| fleet.hold_on_ship(state, *ship))
                    else {
                        continue;
                    };
                    let Some(hold) = state.hold_of(target).copied() else {
                        continue;
                    };
                    let freed: BTreeSet<ShipId> = found
                        .afloat
                        .iter()
                        .copied()
                        .filter(|ship| {
                            state
                                .commitments
                                .get(ship)
                                .is_some_and(|list| list.iter().any(|c| c.plan == plan.id))
                        })
                        .collect();
                    if let Some(run) = effective
                        .plans
                        .get_mut(&plan.id)
                        .and_then(|plan| plan.runs.get_mut(&run.number))
                    {
                        run.ended = Some(today);
                    }
                    for ship in &freed {
                        if let Some(list) = effective.commitments.get_mut(ship) {
                            list.retain(|c| c.plan != plan.id);
                            if list.is_empty() {
                                effective.commitments.remove(ship);
                            }
                        }
                    }
                    aside.push(SetAside {
                        plan: plan.id,
                        run: run.number,
                        freed,
                        target,
                        hold,
                        aboard_t: found.aboard_t,
                    });
                }
            }
        }
        Morning {
            runs: Vec::new(),
            aside,
            state: effective,
        }
    }

    /// The command state as the staff reads it this morning.
    pub(crate) fn state(&self) -> &CommandState {
        &self.state
    }

    /// Hands in the commands that set the runs aside: the end of each run
    /// and the freeing of its ships' commitment. With `only` set, those of
    /// that plan alone.
    pub(crate) fn hand_in_aside(&self, out: &mut Out, only: Option<PlanId>) {
        for set in self
            .aside
            .iter()
            .filter(|set| only.is_none_or(|p| p == set.plan))
        {
            let Some(plan) = self.state.plans.get(&set.plan) else {
                continue;
            };
            let held = hold_words(set.target, set.hold);
            let aboard = if set.aboard_t.is_zero() {
                "nothing is aboard".to_string()
            } else {
                format!("the {} aboard stay aboard", tons(set.aboard_t))
            };
            out.command(
                CommandKind::EndRun {
                    plan: set.plan,
                    run: set.run,
                },
                serving_intent(plan.intent, plan)
                    .evidence(held.clone(), set.hold.day)
                    .binding(format!("the hold on {}", set.target))
                    .worked(format!(
                        "Run {} of {} is ended: {held}; {aboard}.",
                        set.run, set.plan
                    ))
                    .would_change(
                        "the run not yet sailed, or the hold released before the staff is asked",
                    )
                    .done(),
            );
            if !set.freed.is_empty() {
                let ships = set
                    .freed
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                out.command(
                    CommandKind::FreeShips {
                        plan: set.plan,
                        ships: set.freed.clone(),
                    },
                    serving_intent(plan.intent, plan)
                        .evidence(held.clone(), set.hold.day)
                        .worked(format!(
                            "Run {} of {} is ended, so the commitment of {ships} to {} is freed; {aboard}.",
                            set.run, set.plan, set.plan
                        ))
                        .done(),
                );
            }
        }
    }

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
