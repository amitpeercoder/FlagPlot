//! A plan handed in as an order (decision 0159, rule E, items 15 to 17): for
//! each intent of the commander's in force, and each "take a place" in force
//! that the staff formed (decision 0164), what the staff already works out,
//! as data, worked when asked and stored nowhere.
//!
//! - **The plan, its phases and its tasks** are read from the side's command
//!   state as the morning finds it, before that morning's decisions: a
//!   take's prepare (ended by the release), move (ended by the first man
//!   ashore at the place) and land (ended when the place is the side's); a
//!   hold's wait (ended when its base is the side's) and hold, which does not
//!   end, or hold alone where the base is the side's.
//! - **What it waits for, the men and the need to know** are the staff's own
//!   working of that morning ([`Day::run`]), read where the doctrine already
//!   works them: the wait for men of rule B, the limits of rule C
//!   ([`Day::told_limits`]), the need to know of rule D, and the leave the
//!   commander withholds. A thing the doctrine does not wait on is not
//!   listed: a count asked where nothing can look and the take goes all the
//!   same, a leave that is given.
//! - **The day it expects to be ashore** is worked by the sums a take uses
//!   for a latest day ([`Day::lift_days`], [`Day::days_to_ready`]): the
//!   morning the landing is ready, or the latest day a thing waited for is
//!   expected, plus the days to embark and of passage, less 1. A thing
//!   waited for with no day leaves it without one.
//!
//! A day on which a task had no order is not in the order (item 16): it
//! stays in the task's reasons.
//!
//! The tasks are every intent in force beneath (decision 0232, rule 18):
//! those the staff formed and those the commander wrote under his own order.
//! Whose each is, is the intent's `maker` in the command state.

use sim_core::command::{
    Intent, IntentKind, Knowledge, Leave, Maker, MenCount, OrderTask, Phase, PhaseEnd, PhaseName,
    PhaseStage, PlanOrder, PlanState, Waited, WaitedFor,
};
use sim_core::hex::Hex;
use sim_core::ids::LandUnitId;
use sim_core::view::SideView;

use crate::air::Day;
use crate::reasons::statement;

/// The plan order of every intent of the commander's in force and of every
/// "take a place" in force that the staff formed (decision 0164, items 1 and
/// 2), together in number order. `day` is the staff's morning, `None` where
/// the air doctrine has nothing to work (no take and no hold).
pub(crate) fn orders(view: &SideView<'_>, day: Option<&Day<'_, '_>>) -> Vec<PlanOrder> {
    let state = view.command_state();
    state
        .intents_in_force()
        .filter(|intent| has_an_order(intent))
        .filter_map(|intent| order_of(view, day, intent))
        .collect()
}

/// Whether the staff hands in a plan order for `intent`: an intent of the
/// commander's, or a "take a place" the staff formed (decision 0164, item
/// 1). No other intent of the staff's has one.
fn has_an_order(intent: &Intent) -> bool {
    match intent.maker {
        Maker::Commander => true,
        Maker::Staff => matches!(intent.kind, IntentKind::TakePlace { .. }),
    }
}

fn order_of(view: &SideView<'_>, day: Option<&Day<'_, '_>>, intent: &Intent) -> Option<PlanOrder> {
    let state = view.command_state();
    let plan = state.plans.get(&intent.plan)?;
    let mut order = PlanOrder {
        intent: intent.id,
        mission: statement(view, intent),
        plan: plan.id,
        plan_state: plan.state,
        phases: Vec::new(),
        waits_for: Vec::new(),
        men: None,
        expected_ashore: None,
        need_to_know: None,
        tasks: Vec::new(),
    };
    match intent.kind {
        IntentKind::TakePlace { hex: place, .. } => {
            if let Some(day) = day {
                order.phases = take_phases(view, day, plan.state, place);
                take_working(view, day, intent, place, &mut order);
            }
        }
        IntentKind::Hold { base } => {
            order.phases = hold_phases(view, plan.state, base);
        }
        _ => {}
    }
    let under_way = order
        .phases
        .iter()
        .find(|p| p.stage == PhaseStage::UnderWay)
        .map(|p| p.name);
    let is_take = matches!(intent.kind, IntentKind::TakePlace { .. });
    order.tasks = state
        .descendants_in_force(intent.id)
        .into_iter()
        .filter_map(|id| state.intents.get(&id))
        .map(|task| OrderTask {
            intent: task.id,
            kind: task.kind,
            phase: match task.kind {
                IntentKind::ReadyLanding { .. } if is_take => Some(PhaseName::Prepare),
                IntentKind::Bombard { .. } if is_take => Some(PhaseName::Move),
                _ => under_way,
            },
        })
        .collect();
    Some(order)
}

fn phase(name: PhaseName, ends: PhaseEnd, stage: PhaseStage) -> Phase {
    Phase { name, ends, stage }
}

/// Item 16: a take's three phases, read from the plan's state and the side's
/// own view.
fn take_phases(view: &SideView<'_>, day: &Day<'_, '_>, state: PlanState, place: Hex) -> Vec<Phase> {
    use PhaseStage::{Done, ToCome, UnderWay};
    let ours = day.place_is_ours(place);
    let ashore = view
        .land_units()
        .any(|unit| unit.men_ashore > 0 && unit.hex == Some(place));
    let (prepare, moving, land) = match state {
        PlanState::Draft => (ToCome, ToCome, ToCome),
        PlanState::Preparing => (UnderWay, ToCome, ToCome),
        PlanState::Released if ours => (Done, Done, Done),
        PlanState::Released if ashore => (Done, Done, UnderWay),
        PlanState::Released => (Done, UnderWay, ToCome),
        PlanState::Complete => (Done, Done, Done),
        // An ended plan that did not meet its end: what the facts show done,
        // and nothing under way.
        PlanState::Failed | PlanState::Cancelled => {
            let landed = if ours { Done } else { ToCome };
            let moved = if ours || ashore { Done } else { ToCome };
            (moved, moved, landed)
        }
    };
    vec![
        phase(PhaseName::Prepare, PhaseEnd::Released, prepare),
        phase(PhaseName::Move, PhaseEnd::FirstManAshore(place), moving),
        phase(PhaseName::Land, PhaseEnd::PlaceIsTheSides(place), land),
    ]
}

/// Item 16: a hold's phases. Of a base not the side's: wait, ended when the
/// base is the side's, then hold; of the side's own base, hold alone.
fn hold_phases(view: &SideView<'_>, state: PlanState, base: sim_core::ids::BaseId) -> Vec<Phase> {
    use PhaseStage::{ToCome, UnderWay};
    let live = matches!(state, PlanState::Preparing | PlanState::Released);
    let ours = view.bases().any(|b| b.id == base);
    let hold = |stage| phase(PhaseName::Hold, PhaseEnd::Never, stage);
    if ours {
        return vec![hold(if live { UnderWay } else { ToCome })];
    }
    vec![
        phase(
            PhaseName::Wait,
            PhaseEnd::BaseIsTheSides(base),
            if live { UnderWay } else { ToCome },
        ),
        hold(ToCome),
    ]
}

/// The men of `units` ashore and aboard.
fn men_of(view: &SideView<'_>, units: &[LandUnitId]) -> u32 {
    units
        .iter()
        .filter_map(|id| view.land_units().find(|u| u.id == *id))
        .map(|u| u.men_ashore + view.men_aboard(u.id).unwrap_or(0))
        .sum()
}

/// What a take waits for, its men, its day ashore and its need to know, from
/// the staff's morning.
fn take_working(
    view: &SideView<'_>,
    day: &Day<'_, '_>,
    end: &Intent,
    place: Hex,
    order: &mut PlanOrder,
) {
    let state = view.command_state();
    let risk = state.risk_read_up(end.id);
    let task = state
        .intents_in_force()
        .find(|i| i.parent == Some(end.id) && matches!(i.kind, IntentKind::ReadyLanding { .. }))
        .cloned();
    let task_units: Option<Vec<LandUnitId>> = task.as_ref().map(|t| {
        state
            .land_units_on_chain(t.id)
            .map(|units| units.into_iter().collect())
            .unwrap_or_default()
    });
    let men_wait = day.men_waits.get(&end.id);
    let need = day.needs.iter().find(|n| n.intent == end.id).cloned();

    // The men wanted and to hand: against the count held, the task's units
    // where it is formed, else the men the wait for men found to hand.
    let to_hand = match (&task_units, men_wait) {
        (Some(units), _) => Some(men_of(view, units)),
        (None, Some(wait)) => Some(wait.have),
        (None, None) => None,
    };
    order.men = crate::take::men_wanted(view, place, risk)
        .zip(to_hand)
        .map(|(wanted, to_hand)| MenCount { wanted, to_hand });
    order.need_to_know = need.clone();

    // What it waits for: nothing once the plan is released or ended.
    let open = matches!(order.plan_state, PlanState::Draft | PlanState::Preparing);
    if open {
        if let Some(wait) = men_wait {
            order.waits_for.push(Waited {
                thing: WaitedFor::Men {
                    wanted: wait.wanted,
                    to_hand: wait.have,
                },
                met: false,
                expected_day: wait.then.as_ref().map(|(d, _)| *d),
            });
        }
        // The count asked, where the staff waits on it: met, or not met
        // with a group looking, a force moved to look, or a recent count
        // that is not launched on none. A count asked where nothing can
        // look does not hold the landing, and is not listed.
        if let Some(need) = need
            .as_ref()
            .filter(|n| n.asked != Knowledge::NoCount)
            .filter(|n| {
                n.met
                    || n.looking.is_some()
                    || n.moved_to_look.is_some()
                    || matches!(n.asked, Knowledge::RecentCount(_))
            })
        {
            order.waits_for.push(Waited {
                thing: WaitedFor::Knowledge(need.asked),
                met: need.met,
                expected_day: None,
            });
        }
        for told in day.told_limits(end) {
            order.waits_for.push(Waited {
                thing: told.thing,
                met: told.met,
                expected_day: told.expected_day,
            });
        }
        let leaves = state.leaves;
        let withheld = match order.plan_state {
            PlanState::Draft if !leaves.prepare => Some(Leave::Prepare),
            PlanState::Preparing if !leaves.launch => Some(Leave::Launch),
            _ => None,
        };
        if let Some(leave) = withheld {
            order.waits_for.push(Waited {
                thing: WaitedFor::Leave(leave),
                met: false,
                expected_day: None,
            });
        }
    }

    order.expected_ashore = expected_ashore(view, day, end, place, task.as_ref(), order);
}

/// The day the take expects its first men ashore, by the sums of a latest
/// day; `None` where its men are ashore or the place is the side's, or where
/// a thing waited for has no day.
fn expected_ashore(
    view: &SideView<'_>,
    day: &Day<'_, '_>,
    end: &Intent,
    place: Hex,
    task: Option<&Intent>,
    order: &PlanOrder,
) -> Option<u32> {
    let state = view.command_state();
    let figures = view.ground_figures()?;
    let ashore = view
        .land_units()
        .any(|unit| unit.men_ashore > 0 && unit.hex == Some(place));
    if ashore || day.place_is_ours(place) {
        return None;
    }
    let unmet: Vec<&Waited> = order.waits_for.iter().filter(|w| !w.met).collect();
    if unmet.iter().any(|w| w.expected_day.is_none()) {
        return None;
    }
    let waited_until = unmet.iter().filter_map(|w| w.expected_day).max();
    let risk = state.risk_read_up(end.id);
    let source = state
        .forces_as_bound(end)
        .and_then(|f| f.source)
        .and_then(|s| view.bases().find(|b| b.id == s).map(|b| (s, b.hex)));
    let lift_of = |force_id, units: &[LandUnitId], source: (sim_core::ids::BaseId, Hex)| {
        let force = view.task_forces().find(|f| f.id == force_id)?.clone();
        day.lift_days(end, &force, units, source.0, source.1, place, figures)
            .ok()
            .map(|days| (force, days))
    };
    match (order.plan_state, task) {
        (PlanState::Released, Some(task)) => {
            let IntentKind::ReadyLanding { force, .. } = task.kind else {
                return None;
            };
            let units: Vec<LandUnitId> = state
                .land_units_on_chain(task.id)
                .map(|u| u.into_iter().collect())
                .unwrap_or_default();
            let (_, days) = lift_of(force, &units, source?)?;
            Some(days.lands_on(day.today))
        }
        (PlanState::Preparing, Some(task)) => {
            let IntentKind::ReadyLanding { force, .. } = task.kind else {
                return None;
            };
            let ready = day.landings.get(&task.id)?.ready_on?;
            let units: Vec<LandUnitId> = state
                .land_units_on_chain(task.id)
                .map(|u| u.into_iter().collect())
                .unwrap_or_default();
            let (_, days) = lift_of(force, &units, source?)?;
            let start = waited_until.map_or(ready, |w| w.max(ready));
            Some(days.lands_on(start))
        }
        (PlanState::Draft | PlanState::Preparing, None) => {
            // A take that waits for its men: a landing begun on the day
            // that brings them, with that day's lift, ready by the
            // projection a latest day is worked by.
            let (begun, lift) = day.men_waits.get(&end.id)?.then.clone()?;
            let (force, days) = lift_of(lift.force, &lift.units, (lift.source, lift.source_hex))?;
            let by_day = day.take_date(end);
            let left = by_day.map(|latest| i64::from(days.task_date(latest)) - i64::from(begun));
            let ready = begun + day.days_to_ready(&force, place, &lift.units, risk, left)?;
            let start = waited_until.map_or(ready, |w| w.max(ready));
            Some(days.lands_on(start))
        }
        _ => None,
    }
}
