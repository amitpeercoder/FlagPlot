//! Building a reason: the five parts of the spec's "Reasons" (serves,
//! evidence, binding, worked, would change). A reason names the side's own
//! units as the engine writes them (`ship:1`, `task_force:1000001`,
//! `base:2`), so a tool can show them in the side's own numbers.

use sim_core::command::{
    ChainLink, CommandState, Evidence, Hold, HoldMaker, HoldTarget, Intent, IntentKind, Plan,
    Proposal, Reason, Serves, StaffDecisions,
};
use sim_core::ids::{BaseId, IntentId};
use sim_core::view::SideView;

use crate::planner::DEFAULT_INTERVAL_DAYS;

fn days_word(n: u32) -> &'static str {
    if n == 1 {
        "day"
    } else {
        "days"
    }
}

/// A base as a reason names it: its handle and, where the side knows it, its
/// name: "base:2 (Placeholder base T)".
pub(crate) fn base_words(view: &SideView<'_>, base: BaseId) -> String {
    let name = view
        .bases()
        .find(|b| b.id == base)
        .map(|b| b.name.clone())
        .or_else(|| view.enemy_bases().find(|b| b.id == base).map(|b| b.name));
    match name {
        Some(name) => format!("{base} ({name})"),
        None => base.to_string(),
    }
}

/// What an intent is for, in words: "keep base:2 (Placeholder base T) at 30
/// days of supply".
pub(crate) fn aim(view: &SideView<'_>, intent: &Intent) -> String {
    match intent.kind {
        IntentKind::KeepSupplied {
            base, floor_days, ..
        } => format!(
            "keep {} at {floor_days} {} of supply",
            base_words(view, base),
            days_word(floor_days)
        ),
        IntentKind::Cover { base } => format!("cover {} from the air", base_words(view, base)),
        IntentKind::Hold { base } => format!("hold {}", base_words(view, base)),
    }
}

/// A plan with what it is for: "plan:1, the plan to keep base:2
/// (Placeholder base T) at 30 days of supply,".
pub(crate) fn plan_for(view: &SideView<'_>, intent: &Intent, plan: &Plan) -> String {
    format!("{}, the plan to {},", plan.id, aim(view, intent))
}

/// The intent as the commander stated it: its aim and its interval, "keep
/// base:2 (Placeholder base T) at 30 days of supply, a run every 15 days",
/// and the day it was withdrawn if it has been.
pub(crate) fn statement(view: &SideView<'_>, intent: &Intent) -> String {
    let mut words = aim(view, intent);
    if let IntentKind::KeepSupplied { interval_days, .. } = intent.kind {
        match interval_days {
            Some(days) => words.push_str(&format!(", a run every {days} {}", days_word(days))),
            None => words.push_str(&format!(
                ", no interval set (the staff takes {DEFAULT_INTERVAL_DAYS} days)"
            )),
        }
    }
    if let Some(day) = intent.withdrawn_day {
        words.push_str(&format!(", withdrawn on day {day}"));
    }
    words
}

/// Gives every reason of a hand-in the statement of the intent it serves,
/// and the chain of intents above it, from the side's command state, so that
/// each reason carries its intent in words wherever it is read and after the
/// intent changes (decision 0107, item 12; decision 0117, 10). The one place
/// the staff's reasons get it.
pub(crate) fn state_intents(view: &SideView<'_>, decisions: &mut StaffDecisions) {
    fill_decisions(view, view.command_state(), decisions);
}

/// As [`state_intents`], on a command state the caller names: the air
/// doctrine's planned state, which holds the intents formed this morning.
pub(crate) fn fill_decisions(
    view: &SideView<'_>,
    state: &CommandState,
    decisions: &mut StaffDecisions,
) {
    for order in &mut decisions.orders {
        fill_reason(view, state, &mut order.reason);
    }
    for command in &mut decisions.commands {
        fill_reason(view, state, &mut command.reason);
    }
    for reason in &mut decisions.no_orders {
        fill_reason(view, state, reason);
    }
}

/// One reason's statement and chain, from `state`.
pub(crate) fn fill_reason(view: &SideView<'_>, state: &CommandState, reason: &mut Reason) {
    let Some(id) = reason.serves.intent else {
        return;
    };
    let Some(intent) = state.intents.get(&id) else {
        return;
    };
    reason.serves.statement = statement(view, intent);
    reason.serves.chain = state
        .ancestors(id)
        .into_iter()
        .map(|above| ChainLink {
            intent: above.id,
            maker: above.maker,
            statement: statement(view, above),
        })
        .collect();
}

/// As [`state_intents`], for a proposal: its reason, its runs' reasons and
/// its hand-in.
pub(crate) fn state_intents_of_proposal(view: &SideView<'_>, proposal: &mut Proposal) {
    state_intents(view, &mut proposal.decisions);
    let state = view.command_state();
    let mut reasons: Vec<&mut Reason> = vec![&mut proposal.reason];
    reasons.extend(proposal.runs.iter_mut().map(|run| &mut run.reason));
    for reason in reasons {
        fill_reason(view, state, reason);
    }
}

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
                statement: String::new(),
                chain: Vec::new(),
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
