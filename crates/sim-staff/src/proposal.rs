//! A proposal: what the staff would do for one intent, worked from the view
//! and stored nowhere (decision 0107, 5 and "A proposal"). It is what the
//! staff would hand in today as if the plan were released, the runs the same
//! rules give over the next 30 days on the projection, the projection, what
//! binds, and a reason. In step 1 it displaces nothing: there is one intent.

use std::collections::BTreeMap;

use sim_core::command::{IntentKind, Proposal, ProposedRun, Reason};
use sim_core::fixed::Fixed;
use sim_core::ids::{IntentId, ShipId};
use sim_core::view::SideView;

use crate::doctrine::{pending_loads, serve, setup, Mode, Setup, LOOK_AHEAD_DAYS};
use crate::fleet::Fleet;
use crate::handin::Out;
use crate::planner::{Planner, Unit};
use crate::projection::Readings;
use crate::reasons::serving;
use crate::units::{tons, whole};

/// The days of runs a proposal shows.
pub(crate) const PROPOSAL_DAYS: u32 = 30;

pub(crate) fn propose(view: SideView<'_>, intent: IntentId) -> Option<Proposal> {
    let state = view.command_state();
    let intent = state.intents.get(&intent).filter(|i| i.in_force())?;
    let plan = state.plans.get(&intent.plan)?;
    let today = view.day() + 1;
    let fleet = Fleet::of(&view);
    let IntentKind::KeepSupplied {
        base, floor_days, ..
    } = intent.kind;

    let mut out = Out::default();
    let served = serve(&view, &fleet, intent, Mode::AsReleased, &mut out);
    let decisions = out.finish(view.next_task_force_id());

    let destination_hex = view.bases().find(|b| b.id == base).map(|b| b.hex);
    let projection = destination_hex.and_then(|hex| {
        Readings::build(&view, &fleet, hex, PROPOSAL_DAYS)
            .map(|readings| readings.projection(hex, floor_days))
    });

    let mut runs: Vec<ProposedRun> = Vec::new();
    let mut shortfall = served.shortfall.clone();
    if let Setup::Ready(params) = setup(&view, intent, plan) {
        let planner = Planner::new(&view, &fleet, params);
        if let Some(mut readings) = Readings::build(
            &view,
            &fleet,
            planner.params.destination_hex,
            LOOK_AHEAD_DAYS,
        ) {
            let base_pending = pending_loads(
                &view,
                &fleet,
                planner.params.source,
                planner.params.source_hex,
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
                        taken += run.load_t;
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
                            ));
                        }
                        day += 1;
                    }
                    _ => day += 1,
                }
            }
        }
    }

    let worked = match (&shortfall, runs.len()) {
        (_, 0) if shortfall.is_none() => format!(
            "{} needs no run in the next {PROPOSAL_DAYS} days: the figure stays at or above {} days.",
            intent.id, floor_days
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
                    " It cannot meet the intent in full: {} binds, {} days short.",
                    s.binds, s.shortfall_days
                ),
                None => String::new(),
            }
        ),
    };
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
