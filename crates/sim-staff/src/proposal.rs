//! A proposal: what the staff would do for one intent, worked from the view
//! and stored nowhere (decision 0107, 5 and "A proposal"). It is what the
//! staff would hand in today as if the plan were released, the runs the same
//! rules give over the next 30 days on the projection, the projection, what
//! binds, and a reason.
//!
//! From step 2 of stage 1 it is worked as `decide` would work the day: the
//! intents that come before this one in priority order are served first (into
//! a hand-in thrown away), then this one as if released, then those after it,
//! so that the proposal says what this intent is displaced by (a ship or
//! tons committed this morning to a higher intent) and which lower intents
//! it would displace (ruling 14, "both directions"). With one intent it
//! displaces nothing and is displaced by nothing.

use std::collections::BTreeMap;

use sim_core::command::{Displaced, IntentKind, Proposal, ProposedRun, Reason, Shortfall};
use sim_core::fixed::Fixed;
use sim_core::ids::{IntentId, ShipId, TaskForceId};
use sim_core::view::SideView;

use crate::doctrine::{
    displaced_sentence, pending_loads, serve, setup, Mode, Setup, LOOK_AHEAD_DAYS,
};
use crate::fleet::Fleet;
use crate::handin::Out;
use crate::morning::{priority_order, Morning};
use crate::planner::{Planner, Unit};
use crate::projection::Readings;
use crate::reasons::serving;
use crate::units::{tons, whole};

/// The days of runs a proposal shows.
pub(crate) const PROPOSAL_DAYS: u32 = 30;

pub(crate) fn propose(view: SideView<'_>, intent: IntentId) -> Option<Proposal> {
    let state = view.command_state();
    let intent = state.intents.get(&intent).filter(|i| i.in_force())?;
    let today = view.day() + 1;
    let fleet = Fleet::of(&view);
    // The morning begins as `decide`'s does: runs set aside, the command
    // state as the staff reads it.
    let mut morning = Morning::begin(&view, &fleet);
    let plan = &morning.state().plans.get(&intent.plan)?.clone();
    let IntentKind::KeepSupplied {
        base, floor_days, ..
    } = intent.kind
    else {
        return None;
    };

    // The intents before this one, served as the day would serve them.
    let order = priority_order(state);
    let position = order
        .iter()
        .position(|other| other.id == intent.id)
        .unwrap_or(order.len());
    let mut scratch = Out::default();
    morning.hand_in_aside(&view, &mut scratch, None);
    for earlier in &order[..position] {
        serve(
            &view,
            &fleet,
            earlier,
            Mode::Live,
            &mut scratch,
            &mut morning,
        );
    }
    // What those intents hold this morning, for the forward look below.
    let before = morning.clone();
    let mut out = Out::default();
    morning.hand_in_aside(&view, &mut out, Some(plan.id));
    let served = serve(
        &view,
        &fleet,
        intent,
        Mode::AsReleased,
        &mut out,
        &mut morning,
    );
    let next = view.next_task_force_id();
    let decisions = out.finish(TaskForceId::new(next.raw() + scratch.formed() as u32));

    // The intents after it, to see which the run it takes would displace.
    let mut displaces: Vec<Displaced> = Vec::new();
    let mut later = Out::default();
    for after in order.iter().skip(position + 1) {
        let found = serve(&view, &fleet, after, Mode::Live, &mut later, &mut morning);
        if let Some(short) = found.shortfall {
            let by: Vec<_> = short
                .displaced_by
                .iter()
                .filter(|d| d.plan == plan.id)
                .cloned()
                .collect();
            if !by.is_empty() {
                displaces.push(Displaced {
                    intent: after.id,
                    plan: after.plan,
                    shortfall: Shortfall {
                        displaced_by: by,
                        ..short
                    },
                });
            }
        }
    }

    let destination_hex = view.bases().find(|b| b.id == base).map(|b| b.hex);
    let projection = destination_hex.and_then(|hex| {
        Readings::build(&view, &fleet, before.state(), hex, PROPOSAL_DAYS)
            .map(|readings| readings.projection(hex, floor_days))
    });

    let mut runs: Vec<ProposedRun> = Vec::new();
    let mut shortfall = served.shortfall.clone();
    if let Setup::Ready(params) = setup(&view, intent, plan) {
        let planner = Planner::new(&view, &fleet, params, &before);
        if let Some(mut readings) = Readings::build(
            &view,
            &fleet,
            planner.state(),
            planner.params.destination_hex,
            LOOK_AHEAD_DAYS,
        ) {
            let base_pending = pending_loads(
                &view,
                &fleet,
                planner.params.source,
                planner.params.source_hex,
                &before,
            );
            // The units free today; one used by a proposed run is free again
            // on that run's free day.
            let mut free_from: BTreeMap<usize, u32> = BTreeMap::new();
            let units: Vec<Unit> = planner.free_units();
            let mut taken = Fixed::ZERO;
            let mut day = today;
            while day < today + PROPOSAL_DAYS {
                let available: Vec<Unit> = units
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| free_from.get(i).is_none_or(|free| *free <= day))
                    .map(|(_, unit)| unit.clone())
                    .collect();
                let Some(ev) = planner.evaluate(&readings, day, &available, base_pending + taken)
                else {
                    day += 1;
                    continue;
                };
                match (&ev.run, ev.needed) {
                    (Some(run), true) => {
                        let reason = proposed_reason(&planner, &ev, run, day);
                        for (i, unit) in units.iter().enumerate() {
                            if unit.ships.iter().any(|s| run.ships.contains(s)) {
                                free_from.insert(i, run.days.free);
                            }
                        }
                        readings.add_landing(run.days.arrive + 1, run.load_t);
                        taken += run.taken_t;
                        runs.push(ProposedRun {
                            ships: run.ships.iter().copied().collect(),
                            load_t: run.load_t,
                            days: run.days,
                            reason,
                        });
                        day = run.days.sail.max(day + 1);
                    }
                    (None, true) => {
                        if shortfall.is_none() {
                            shortfall = Some(crate::doctrine::shortfall(
                                &planner,
                                &readings,
                                &ev,
                                Fixed::ZERO,
                                base_pending + taken,
                                None,
                            ));
                        }
                        day += 1;
                    }
                    _ => day += 1,
                }
            }
        }
    }

    let source = intent
        .limits
        .forces
        .as_ref()
        .and_then(|forces| forces.source)
        .unwrap_or(base);
    let mut worked = match (&shortfall, runs.len()) {
        (_, 0) if shortfall.is_none() => format!(
            "{} needs no run in the next {PROPOSAL_DAYS} days: the figure stays at or above {} days.",
            intent.id, floor_days
        ),
        (Some(s), 0) => format!(
            "To keep the base at {} days the staff would send no run in the next {PROPOSAL_DAYS} days. It cannot meet the intent in full: {} binds, {} {} short at the reading of day {}.",
            floor_days,
            s.binds,
            s.shortfall_days,
            crate::units::days_word_of(s.shortfall_days),
            s.check_day
        ),
        _ => format!(
            "To keep the base at {} days the staff would send {} run{} in the next {PROPOSAL_DAYS} days: {}.{}",
            floor_days,
            runs.len(),
            if runs.len() == 1 { "" } else { "s" },
            runs.iter()
                .enumerate()
                .map(|(i, run)| format!(
                    "run {} of {} on day {} (due day {}) with {}",
                    i + 1,
                    tons(run.load_t),
                    run.days.load,
                    run.days.arrive,
                    run.ships.iter().map(ShipId::to_string).collect::<Vec<_>>().join(", ")
                ))
                .collect::<Vec<_>>()
                .join("; "),
            match &shortfall {
                Some(s) => format!(
                    " It cannot meet the intent in full: {} binds, {} {} short.",
                    s.binds,
                    s.shortfall_days,
                    crate::units::days_word_of(s.shortfall_days)
                ),
                None => String::new(),
            }
        ),
    };
    if let Some(s) = &shortfall {
        worked.push_str(&displaced_sentence(&s.displaced_by, source));
    }
    for later in &displaces {
        worked.push_str(&format!(
            " It displaces {}: short {} {} at the reading of day {}.",
            later.intent,
            later.shortfall.shortfall_days,
            crate::units::days_word_of(later.shortfall.shortfall_days),
            later.shortfall.check_day
        ));
    }
    let mut builder = serving(intent, plan)
        .evidence(
            format!(
                "the plan is {}; the proposal takes it as released",
                plan.state.name()
            ),
            today,
        )
        .worked(worked)
        .would_change("a different floor, interval or limit")
        .would_change("a ship added to the pool");
    if let Some(s) = &shortfall {
        builder = builder.binding(s.binds.clone());
    }
    if let Some(first) = projection.as_ref().and_then(|p| p.first_day_below) {
        builder = builder.evidence(
            format!("the figure first falls below {floor_days} days on day {first}"),
            today,
        );
    }
    let reason = builder.done();
    let _ = whole(0);
    Some(Proposal {
        intent: intent.id,
        plan: plan.id,
        plan_state: plan.state,
        projection,
        decisions,
        runs,
        shortfall,
        displaces,
        reason,
    })
}

fn proposed_reason(
    planner: &Planner<'_, '_>,
    ev: &crate::planner::Evaluation,
    run: &crate::planner::RunPlan,
    day: u32,
) -> Reason {
    let params = &planner.params;
    serving(params.intent, params.plan)
        .evidence(
            format!(
                "the reading of day {} is projected at {} against {} days x {}",
                ev.check_day,
                tons(ev.check_stock_t),
                params.floor_days,
                tons(ev.check_use_t)
            ),
            day,
        )
        .binding(format!(
            "floor of {} days at {}",
            params.floor_days, params.destination
        ))
        .worked(format!(
            "A run ordered on day {day} is due on day {}; load = max(0, {} - {}) + {} = {} t.",
            run.days.arrive, ev.floor_t, ev.stock_at_arrival_t, ev.interval_use_t, ev.uncapped_t
        ))
        .would_change("a different projection of use or of what is on its way")
        .done()
}
