//! Building a reason: the five parts of the spec's "Reasons" (serves,
//! evidence, binding, worked, would change). A reason names the side's own
//! units as the engine writes them (`ship:1`, `task_force:1000001`,
//! `base:2`), so a tool can show them in the side's own numbers.

use sim_core::command::{
    ChainLink, CommandState, Evidence, Hold, HoldMaker, HoldTarget, Intent, IntentKind, Plan,
    Proposal, Reason, Serves, StaffDecisions,
};
use sim_core::ids::{BaseId, IntentId, ShipId, TaskForceId};
use sim_core::pictures::CommandRef;
use sim_core::state::ShipLocation;
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

/// The words that say a held state is of another moment (decision 0136,
/// 11): " (as of the morning of day 1)", or nothing where the state is of the
/// pulse just ended, where the area command holds nothing of the command, and
/// on any view but the area command's. With every report on time the words
/// never appear.
pub(crate) fn as_of_note(view: &SideView<'_>, command: CommandRef) -> String {
    match view.held_as_of(command) {
        Some(as_of) if as_of != view.pulse_just_ended() => format!(" (as of {as_of})"),
        _ => String::new(),
    }
}

/// A base as an area doctrine's reason names it where it rests on the
/// base's state: [`base_words`] and the moment that state is of.
pub(crate) fn base_held_words(view: &SideView<'_>, base: BaseId) -> String {
    format!(
        "{}{}",
        base_words(view, base),
        as_of_note(view, CommandRef::Base(base))
    )
}

/// A task force as an area doctrine's reason names it where it rests on the
/// force's state: its handle and the moment that state is of.
pub(crate) fn force_held_words(view: &SideView<'_>, force: TaskForceId) -> String {
    format!("{force}{}", as_of_note(view, CommandRef::TaskForce(force)))
}

/// The words that say the state of the ships `ships` is held as of another
/// moment: the moments of the forces they sail in, distinct, in order, as
/// " (as of the start)" or " (as of the start; as of the morning of day 1)".
/// A ship in port in no force is held by no command, and says nothing.
pub(crate) fn ships_as_of_note(view: &SideView<'_>, ships: &[ShipId]) -> String {
    let mut moments: Vec<String> = Vec::new();
    for ship in ships {
        let Some(force) = view
            .ships()
            .find(|s| s.id == *ship)
            .and_then(|s| match s.location {
                ShipLocation::TaskForce(f) | ShipLocation::Yard(f) => Some(f),
                _ => None,
            })
        else {
            continue;
        };
        if let Some(as_of) = view.held_as_of(CommandRef::TaskForce(force)) {
            if as_of != view.pulse_just_ended() {
                let text = as_of.to_string();
                if !moments.contains(&text) {
                    moments.push(text);
                }
            }
        }
    }
    if moments.is_empty() {
        String::new()
    } else {
        format!(" (as of {})", moments.join("; as of "))
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
        IntentKind::Neutralise { target } => format!("neutralise {}", base_words(view, target)),
        IntentKind::Raid { base, target } => format!(
            "raid {} from {}",
            base_words(view, target),
            base_words(view, base)
        ),
        IntentKind::Deny { hex, distance } => waters_words(hex, distance),
        IntentKind::StrikeShips { base } => {
            format!("strike ships from {}", base_words(view, base))
        }
        IntentKind::SearchWaters { base } => {
            format!("search the waters from {}", base_words(view, base))
        }
        IntentKind::StandAt { force, hex } => format!("stand at {hex} with {force}"),
        IntentKind::OperateCarriers { force, hex } => {
            format!("operate the carrier force {force} from {hex}")
        }
        // Decision 0141, 1 and 8.
        IntentKind::ReadyLanding { force, hex, by_day } => match by_day {
            Some(day) => format!("ready a landing at {hex} by {force} by the morning of day {day}"),
            None => format!("ready a landing at {hex} by {force}"),
        },
        IntentKind::BuildUp { base, wants } => {
            format!("build up {}: {wants}", base_words(view, base))
        }
        // Decision 0142, 1.
        IntentKind::TakePlace { hex, by_day } => match by_day {
            Some(day) => format!("take the place at {hex} by the end of day {day}"),
            None => format!("take the place at {hex}"),
        },
        // Decision 0146, 1.
        IntentKind::Bombard { force, target } => {
            format!("bombard {} with {force}", base_words(view, target))
        }
        // Decision 0151, 3.
        IntentKind::ControlIsland { island, by_day } => {
            let name = view
                .island(island)
                .map_or_else(String::new, |found| format!(" ({})", found.name));
            match by_day {
                Some(day) => format!("control island {island}{name} by the end of day {day}"),
                None => format!("control island {island}{name}"),
            }
        }
    }
}

/// The waters of a "deny an area" end in words (decision 0123, 1): "deny
/// the waters within 0 hexes of (0,5)".
pub(crate) fn waters_words(hex: sim_core::hex::Hex, distance: u32) -> String {
    format!("deny the waters within {distance} hexes of {hex}")
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
    // The level that decided: the addressee of the intent served (decision
    // 0120, 2).
    // A reason that names a level already (an air group deciding the size of
    // its strikes, decision 0123, 9) keeps it.
    if reason.serves.level.is_none() {
        reason.serves.level = Some(intent.addressee);
    }
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
                level: None,
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

    /// The decision rests on the commander's answer for this force's leave
    /// to pursue (decision 0214, rule 10).
    pub(crate) fn answer_of(mut self, force: sim_core::ids::TaskForceId) -> Self {
        self.reason.answer_of = Some(force);
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
