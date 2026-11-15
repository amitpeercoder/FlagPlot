//! Building a reason: the five parts of the spec's "Reasons" (serves,
//! evidence, binding, worked, would change). A reason names the side's own
//! units as the engine writes them (`ship:1`, `task_force:1000001`,
//! `base:2`), so a tool can show them in the side's own numbers.

use sim_core::command::{Evidence, Hold, HoldMaker, HoldTarget, Intent, Plan, Reason, Serves};
use sim_core::ids::IntentId;

/// Starts a reason that serves `intent` and its `plan`, under the plan's
/// last amendment (`None` for the baseline).
pub(crate) fn serving(intent: &Intent, plan: &Plan) -> ReasonBuilder {
    serving_intent(intent.id, plan)
}

/// As [`serving`], from the intent's number alone.
pub(crate) fn serving_intent(intent: IntentId, plan: &Plan) -> ReasonBuilder {
    ReasonBuilder {
        reason: Reason {
            serves: Serves {
                intent: Some(intent),
                plan: Some(plan.id),
                amendment: plan.amendments.last().map(|amendment| amendment.number),
            },
            ..Reason::default()
        },
    }
}

/// A reason in the making.
#[derive(Clone, Debug)]
pub(crate) struct ReasonBuilder {
    reason: Reason,
}

impl ReasonBuilder {
    pub(crate) fn evidence(mut self, what: impl Into<String>, day: u32) -> Self {
        self.reason.evidence.push(Evidence {
            what: what.into(),
            day,
        });
        self
    }

    pub(crate) fn binding(mut self, what: impl Into<String>) -> Self {
        self.reason.binding.push(what.into());
        self
    }

    pub(crate) fn worked(mut self, text: impl Into<String>) -> Self {
        self.reason.worked = text.into();
        self
    }

    pub(crate) fn would_change(mut self, what: impl Into<String>) -> Self {
        self.reason.would_change.push(what.into());
        self
    }

    pub(crate) fn done(self) -> Reason {
        self.reason
    }
}

/// A hold in words: "ship:1 is held by the commander's hold of day 8", or
/// "task_force:1000001 is held by the commander's order of day 2".
pub(crate) fn hold_words(target: HoldTarget, hold: Hold) -> String {
    match hold.by {
        HoldMaker::Command => format!(
            "{target} is held by the commander's hold of day {}",
            hold.day
        ),
        HoldMaker::Order(_) => format!(
            "{target} is held by the commander's order of day {}",
            hold.day
        ),
    }
}
