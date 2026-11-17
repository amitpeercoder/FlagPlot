//! The doctrine: keep a base supplied (decision 0107, 37 to 45, and "As
//! built (step 1, the staff)"). Each day, for each supply intent in force,
//! in priority order:
//!
//! 1. No forces limit, a source or destination not the side's, a plan
//!    ended: no order, a reason. A plan whose pool has no ship afloat is
//!    marked failed.
//! 2. A draft with leave to prepare is approved; a plan preparing with
//!    leave to launch is released, the same day. Without the leave, a
//!    no-order reason says what waits.
//! 3. Each run not finished is carried on: on the first day its load is
//!    complete, and the plan is released, a route to the destination and
//!    back, acting at the destination, and an unload order.
//! 4. If no run of the plan is loading, a run is needed when, without a new
//!    one, the figure of the day after it would arrive is below the floor.
//!    Its ships, its load and its days are the planner's.
//! 5. When it cannot, it says which binds, the shortfall and the day the
//!    source runs dry.
//! 6. A day it gives an intent no order has one reason.
//! 7. An ended plan's ships committed from today on are freed.
//!
//! Step 2 of stage 1 (doctrine `supply-2`) adds, for more than one intent:
//!
//! - A [`Morning`] holds the runs this call has added for an intent served
//!   before: their ships are not free for a later intent and their tons at
//!   the source are still to be taken (rulings 11 to 13).
//! - A ship protected for another intent in force is in no pool but that
//!   intent's; an intent whose pool holds only such ships gives no order and
//!   says so (ruling 16).
//! - An intent that needs a run and cannot have one, or has its load cut,
//!   because a ship or stock at the source is committed to another plan,
//!   says what displaced it, and its shortfall in days at the reading it
//!   checked (ruling 14).

use sim_core::command::{
    CommandKind, CommandState, Committed, Displacement, Hold, HoldTarget, Intent, IntentKind,
    Leave, NoticeBasis, NoticeKind, Plan, PlanId, PlanOutcome, PlanState, Reason, Risk, RunSpec,
    Shortfall, StaffDecisions,
};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, ShipId, TaskForceId};
use sim_core::pictures::CommandRef;
use sim_core::view::SideView;
use std::collections::BTreeSet;

use crate::fleet::{
    pool_ships, run_movement, run_state, waiting_ships, Fleet, RunState, RunStatus,
};
use crate::handin::{Out, Step, Who};
use crate::morning::{priority_order, protected_for_others, Morning, TakenRun};
use crate::planner::DEFAULT_INTERVAL_DAYS;
use crate::planner::{Blocker, Evaluation, Params, Planner, RunPlan, Timing, Unit};
use crate::projection::Readings;
use crate::reasons::{as_of_note, hold_words, plan_for, serving, ships_as_of_note, ReasonBuilder};
use crate::route::route_between;
use crate::units::{days_word, over, tons, whole};

/// The days the staff looks ahead, to size a run and to say when the next
/// is expected.
pub(crate) const LOOK_AHEAD_DAYS: u32 = 120;

/// Whether the plan is taken as it stands or as if it were released (what a
/// proposal shows).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Mode {
    Live,
    AsReleased,
}

/// What `serve` found, for a proposal to build on.
#[derive(Default)]
pub(crate) struct Served {
    pub(crate) shortfall: Option<Shortfall>,
    pub(crate) ordered: bool,
}

/// The staff's decisions for the day, from the view alone.
pub(crate) fn decide(view: SideView<'_>) -> StaffDecisions {
    // The supply doctrine is addressed to the area: it reads the side's
    // units as the area command holds them (decision 0136, 9). With no
    // report figures, or command levels off, that is the side's own view.
    let known = view.known_to(CommandRef::Area);
    let view = known.view();
    let state = view.command_state();
    let fleet = Fleet::of(&view);
    let mut out = Out::default();
    let mut morning = Morning::begin(&view, &fleet);
    morning.hand_in_aside(&view, &mut out, None);
    // The ships of an ended plan are freed before any intent is served: the
    // doctrine reads them as free, so an intent served this morning may
    // commit them, and the engine refuses a commitment that meets one still
    // standing (found by the soak ashore, scenario 45, seed 79).
    free_ended(&view, &mut out);
    for intent in priority_order(state)
        .into_iter()
        .filter(|intent| matches!(intent.kind, IntentKind::KeepSupplied { .. }))
    {
        serve(&view, &fleet, intent, Mode::Live, &mut out, &mut morning);
    }
    out.finish(view.next_task_force_id())
}

/// The parameters of a supply intent: its base and limits, or why it has
/// none (as a reason's first words).
pub(crate) enum Setup<'a> {
    Ready(Params<'a>),
    NoForces,
    BaseNotYours(&'static str),
}

/// A supply intent with no forces limit of its own reads the nearest one up
/// its tree: the hold's, for one the staff formed under "hold a place"
/// (decision 0144, 7). A root with none has none.
pub(crate) fn setup<'a>(
    view: &SideView<'_>,
    state: &'a CommandState,
    intent: &'a Intent,
    plan: &'a Plan,
) -> Setup<'a> {
    let IntentKind::KeepSupplied {
        base,
        floor_days,
        interval_days,
    } = intent.kind
    else {
        return Setup::NoForces;
    };
    // Intake refuses a supply intent's forces limit with no source.
    let Some((forces, source_id)) = intent
        .limits
        .forces
        .as_ref()
        .or_else(|| state.nearest_forces(intent.id))
        .and_then(|forces| forces.source.map(|source| (forces, source)))
    else {
        return Setup::NoForces;
    };
    let Some(destination) = view.bases().find(|b| b.id == base) else {
        return Setup::BaseNotYours("the destination");
    };
    let Some(source) = view.bases().find(|b| b.id == source_id) else {
        return Setup::BaseNotYours("the source");
    };
    Setup::Ready(Params {
        intent,
        plan,
        forces,
        destination: base,
        destination_hex: destination.hex,
        source: source_id,
        source_hex: source.hex,
        floor_days,
        interval_days: interval_days.unwrap_or(DEFAULT_INTERVAL_DAYS),
        interval_set: interval_days.is_some(),
        area: intent.limits.area.as_ref(),
    })
}

/// The tons the runs of every plan still loading at `source` have yet to
/// take, and the loads of the runs of this morning there.
pub(crate) fn pending_loads(
    view: &SideView<'_>,
    fleet: &Fleet<'_>,
    source: BaseId,
    source_hex: Hex,
    morning: &Morning,
) -> Fixed {
    let today = view.day() + 1;
    let mut pending = morning.pending_at(source);
    let state = morning.state();
    for plan in state.plans.values() {
        for run in plan.runs.values().filter(|run| run.source == source) {
            let found = run_state(fleet, state, plan.id, run, source_hex, today);
            if found.status == RunStatus::Loading {
                pending += (run.load_t - found.aboard_t).max(Fixed::ZERO);
            }
        }
    }
    pending
}

/// Serves one intent for the day.
pub(crate) fn serve(
    view: &SideView<'_>,
    fleet: &Fleet<'_>,
    intent: &Intent,
    mode: Mode,
    out: &mut Out,
    morning: &mut Morning,
) -> Served {
    let state_now: CommandState = morning.state().clone();
    let state = &state_now;
    let today = view.day() + 1;
    let Some(plan) = state.plans.get(&intent.plan) else {
        return Served::default();
    };
    let params = match setup(view, state, intent, plan) {
        Setup::Ready(params) => params,
        Setup::NoForces => {
            out.no_order(
                serving(intent, plan)
                    .evidence("the intent has no forces limit", today)
                    .binding("no forces limit: the staff may use no ship")
                    .worked(format!(
                        "{} has no forces limit, so the staff has no ship to send and gives no order.",
                        intent.id
                    ))
                    .would_change("a forces limit naming ships and a source base")
                    .done(),
            );
            return Served::default();
        }
        Setup::BaseNotYours(which) => {
            if plan.state.approved() && mode == Mode::Live {
                out.command(
                    CommandKind::MarkPlan {
                        plan: plan.id,
                        outcome: PlanOutcome::Failed,
                    },
                    serving(intent, plan)
                        .evidence(format!("{which} base is not one of the side's"), today)
                        .binding(format!("{which} base is not the side's"))
                        .worked(format!(
                            "{} cannot be carried out: {which} base is not one of the side's bases, so {} is marked failed.",
                            intent.id, plan.id
                        ))
                        .would_change("a new intent with a base of the side")
                        .done(),
                );
            }
            out.no_order(
                serving(intent, plan)
                    .evidence(format!("{which} base is not one of the side's"), today)
                    .binding(format!("{which} base is not the side's"))
                    .worked(format!(
                        "{which} base of {} is not one of the side's bases, so there is nothing to send.",
                        intent.id
                    ))
                    .would_change("a base of the side")
                    .done(),
            );
            return Served::default();
        }
    };
    if plan.state.ended() {
        if plan.state == PlanState::Cancelled && send_home(view, fleet, state, &params, out, today)
        {
            return Served {
                ordered: true,
                ..Served::default()
            };
        }
        out.no_order(
            serving(intent, plan)
                .evidence(format!("{} is {}", plan.id, plan.state.name()), today)
                .binding(format!("{} is {}", plan.id, plan.state.name()))
                .worked(format!(
                    "{} is {}, so the staff gives no order for {}.",
                    plan_for(view, intent, plan),
                    plan.state.name(),
                    intent.id
                ))
                .would_change("a new plan for the intent")
                .done(),
        );
        return Served::default();
    }
    let pool = pool_ships(fleet, params.forces);
    let waiting = waiting_ships(fleet, params.forces);
    if pool.is_empty() && !waiting.is_empty() {
        // Every ship of the pool is one the side may not use yet
        // (decision 0140, 3): nothing is ordered, and the plan is not failed,
        // for the ships are there and their day will come.
        let list = crate::available::list_words(&waiting);
        out.no_order(
            serving(intent, plan)
                .evidence(format!("every ship of the pool is left out for its day: {list}"), today)
                .binding("no ship of the pool may be used yet")
                .worked(format!(
                    "No ship of the pool of {} may be used yet: {list}, so there is nothing to send.",
                    intent.id
                ))
                .would_change("the day a ship of the pool may be used from")
                .done(),
        );
        return Served::default();
    }
    if pool.is_empty() {
        let reason = |text: String| {
            serving(intent, plan)
                .evidence("no ship of the pool is afloat", today)
                .binding("the pool has no ship afloat")
                .worked(text)
                .would_change("a ship afloat in the forces limit")
                .done()
        };
        if plan.state.approved() && mode == Mode::Live {
            out.command(
                CommandKind::MarkPlan {
                    plan: plan.id,
                    outcome: PlanOutcome::Failed,
                },
                reason(format!(
                    "No ship of the pool of {} is afloat: {} cannot be carried out and is marked failed.",
                    intent.id, plan.id
                )),
            );
        }
        out.no_order(reason(format!(
            "No ship of the pool of {} is afloat, so there is nothing to send.",
            intent.id
        )));
        return Served::default();
    }

    let protected = protected_for_others(state, intent.id);
    if pool.iter().all(|ship| protected.contains_key(ship)) {
        let list = pool
            .iter()
            .filter_map(|ship| {
                protected
                    .get(ship)
                    .map(|other| format!("{ship} is protected for {other}"))
            })
            .collect::<Vec<_>>()
            .join(", ");
        out.no_order(
            serving(intent, plan)
                .evidence(
                    "every ship of the pool is protected for another intent",
                    today,
                )
                .binding("no ship of the pool may be taken")
                .worked(format!(
                    "No ship of the pool of {} may be taken: {list}.",
                    intent.id
                ))
                .would_change("a ship of the pool no longer protected for another intent")
                .would_change("a ship added to the pool")
                .done(),
        );
        return Served::default();
    }

    // 2. The leaves.
    let leaves = state.leaves;
    let mut eff = if mode == Mode::AsReleased {
        PlanState::Released
    } else {
        plan.state
    };
    if eff == PlanState::Draft {
        if !leaves.prepare {
            let waits = out.no_order(
                serving(intent, plan)
                    .evidence(format!("{} is a draft", plan.id), today)
                    .binding("leave to prepare is withheld")
                    .worked(format!(
                        "{} is a draft and the commander has not given leave to prepare, so it waits for him.",
                        plan_for(view, intent, plan)
                    ))
                    .would_change("leave to prepare given, or the commander approving the plan")
                    .done(),
            );
            out.notice(
                NoticeKind::WaitsForLeave(Leave::Prepare),
                plan.id,
                None,
                NoticeBasis::NoOrder(waits),
            );
            return Served::default();
        }
        out.command(
            CommandKind::Approve { plan: plan.id },
            serving(intent, plan)
                .evidence(format!("{} is a draft", plan.id), today)
                .evidence("leave to prepare is given", today)
                .worked(format!(
                    "{} is a draft inside the limits of {} and leave to prepare is given: the staff makes it ready (preparing).",
                    plan_for(view, intent, plan),
                    intent.id
                ))
                .would_change("leave to prepare withheld")
                .done(),
        );
        eff = PlanState::Preparing;
    }
    if eff == PlanState::Preparing && leaves.launch && mode == Mode::Live {
        out.command(
            CommandKind::Approve { plan: plan.id },
            serving(intent, plan)
                .evidence(format!("{} is preparing", plan.id), today)
                .evidence("leave to launch is given", today)
                .worked(format!(
                    "{} is ready and leave to launch is given: the staff releases it.",
                    plan_for(view, intent, plan)
                ))
                .would_change("leave to launch withheld")
                .done(),
        );
        eff = PlanState::Released;
    }

    let planner = Planner::new(view, fleet, params, morning);
    let params = &planner.params;
    let Some(readings) = Readings::build(
        view,
        fleet,
        planner.state(),
        params.destination_hex,
        LOOK_AHEAD_DAYS,
    ) else {
        return Served::default();
    };
    let pending_t = pending_loads(view, fleet, params.source, params.source_hex, morning);
    let mut served = Served::default();
    let mut loading = false;
    let mut waiting_for_launch = false;
    let mut cannot_meet = false;
    let mut cannot_sail: Option<String> = None;
    let mut underway: Vec<String> = Vec::new();

    // Decision 0126, 6: an escort whose run is home has its charge cleared.
    if mode == Mode::Live && eff == PlanState::Released {
        crate::escort::clear_charges(&planner, out, today);
    }

    // 3. Each run not finished is carried on.
    for run in plan.runs.values() {
        let found = run_state(fleet, state, plan.id, run, params.source_hex, today);
        match found.status {
            RunStatus::Finished => {}
            RunStatus::Underway => underway.push(format!(
                "Run {} is under way with {} aboard{}, due at {} on day {}.",
                run.number,
                tons(found.aboard_t),
                ships_as_of_note(planner.view, &found.afloat),
                params.destination,
                run.days.arrive
            )),
            RunStatus::Loading => {
                let ready = today > run.days.load && load_complete(&planner, run.load_t, &found);
                if !ready {
                    loading = true;
                    underway.push(format!(
                        "Run {} is loading at {}{}: {} of {} aboard.",
                        run.number,
                        params.source,
                        match ships_as_of_note(planner.view, &found.afloat) {
                            note if note.is_empty() => {
                                as_of_note(planner.view, CommandRef::Base(params.source))
                            }
                            note => note,
                        },
                        tons(found.aboard_t),
                        tons(run.load_t)
                    ));
                } else if eff != PlanState::Released {
                    loading = true;
                    waiting_for_launch = true;
                    underway.push(format!(
                        "Run {} is loaded ({} aboard{}) and waits for leave to launch.",
                        run.number,
                        tons(found.aboard_t),
                        ships_as_of_note(planner.view, &found.afloat)
                    ));
                } else {
                    match dispatch(&planner, out, run.number, &found, today) {
                        Ok(()) => served.ordered = true,
                        Err(Unsent::Cannot(why)) => {
                            loading = true;
                            cannot_sail = Some(why);
                        }
                        Err(Unsent::Held(why)) => {
                            loading = true;
                            cannot_sail = Some(why);
                            cannot_meet = true;
                        }
                    }
                }
            }
        }
    }

    // 4. A new run, if none is loading and one is needed.
    let units = planner.free_units();
    let mut evaluation: Option<Evaluation> = None;
    let mut hold_note: Option<WithoutHold> = None;
    if !loading {
        evaluation = planner.evaluate(&readings, today, &units, pending_t);
        // Decision 0145, 7 and 8: a run by night carries no more than the
        // night's hours can land.
        if let Some(run) = evaluation.as_mut().and_then(|ev| ev.run.as_mut()) {
            night_load(&planner, run);
        }
        if let Some(ev) = &evaluation {
            if ev.needed {
                let note = without_hold(&planner, &readings, ev, pending_t);
                match (&ev.run, &ev.blocker) {
                    (Some(run), _) => {
                        if !run.cuts.is_empty() {
                            served.shortfall = Some(shortfall(
                                &planner,
                                &readings,
                                ev,
                                run.load_t,
                                pending_t,
                                Some(&units),
                            ));
                        }
                        let displaced = served
                            .shortfall
                            .as_ref()
                            .map_or(&[][..], |s| s.displaced_by.as_slice());
                        let added = new_run(
                            &planner,
                            out,
                            morning,
                            &readings,
                            ev,
                            run,
                            (displaced, note.as_ref()),
                        );
                        // A cut that leaves the reading below the floor.
                        if served
                            .shortfall
                            .as_ref()
                            .is_some_and(|s| s.shortfall_days > Fixed::ZERO)
                        {
                            out.notice(
                                NoticeKind::CannotMeet,
                                plan.id,
                                None,
                                NoticeBasis::Command(added),
                            );
                        }
                        served.ordered = true;
                    }
                    (None, Some(_)) => {
                        cannot_meet = true;
                        hold_note = note;
                        served.shortfall = Some(shortfall(
                            &planner,
                            &readings,
                            ev,
                            Fixed::ZERO,
                            pending_t,
                            Some(&units),
                        ));
                    }
                    (None, None) => {}
                }
            }
        }
    }

    // 6. No order: one reason. A day a loaded run is dispatched and a new
    // run is needed and can be had by none gives one too, about the next
    // run, for the cannot-meet notice to rest on.
    if !served.ordered || cannot_meet {
        let dispatched = served.ordered;
        let idle = IdleFacts {
            eff,
            underway,
            loading,
            waiting_for_launch,
            cannot_sail,
            evaluation,
            shortfall: served.shortfall.clone(),
            pending_t,
            units,
            hold_note,
        };
        let mut idle_why = idle_reason(&planner, &readings, &idle, today);
        if dispatched {
            idle_why.worked = format!("No new run: {}", idle_why.worked);
        }
        let idle_index = out.no_order(idle_why);
        if cannot_meet {
            out.notice(
                NoticeKind::CannotMeet,
                plan.id,
                None,
                NoticeBasis::NoOrder(idle_index),
            );
        }
        if waiting_for_launch {
            out.notice(
                NoticeKind::WaitsForLeave(Leave::Launch),
                plan.id,
                None,
                NoticeBasis::NoOrder(idle_index),
            );
        }
    }
    served
}

/// Whether a run's load is complete: the supply aboard is its load, or the
/// load cannot grow (the source has nothing above its reserve, or the ships
/// are full), and something is aboard.
fn load_complete(planner: &Planner<'_, '_>, load_t: Fixed, found: &RunState) -> bool {
    if found.aboard_t.is_zero() {
        return false;
    }
    if found.aboard_t >= load_t {
        return true;
    }
    let (available, _, _) = planner.source_available(Fixed::ZERO);
    let room = found
        .afloat
        .iter()
        .filter_map(|id| planner.fleet.ships.get(id))
        .fold(Fixed::ZERO, |sum, ship| {
            let capacity = planner
                .view
                .ship_class(ship.class)
                .map_or(Fixed::ZERO, |class| class.cargo_capacity_t);
            sum + (capacity - ship.supply_t - ship.cargo_fuel_t).max(Fixed::ZERO)
        });
    available.is_zero() || room.is_zero()
}

/// Why a loaded run was not sent today.
enum Unsent {
    /// It cannot be: the plan or its ships do not allow it.
    Cannot(String),
    /// It is held for want of an escort (decision 0126, 4): the staff hands
    /// in that it cannot meet the intent.
    Held(String),
}

/// The orders that send a loaded run: time it again if its sail day is not
/// today (renewing its commitment), then the route out and back acting at
/// the destination, and the unload order.
fn dispatch(
    planner: &Planner<'_, '_>,
    out: &mut Out,
    number: u32,
    found: &RunState,
    today: u32,
) -> Result<(), Unsent> {
    let params = &planner.params;
    let intent = params.intent;
    let plan = params.plan;
    let run = plan
        .runs
        .get(&number)
        .ok_or_else(|| Unsent::Cannot("the run is not in the plan".to_string()))?;
    let mut forces = BTreeSet::new();
    for ship in &found.afloat {
        match planner.fleet.force_of(*ship) {
            Some(force) => {
                forces.insert(force.id);
            }
            None => return Err(Unsent::Cannot(format!("{ship} lies in no force"))),
        }
    }
    let [force] = forces.iter().copied().collect::<Vec<_>>()[..] else {
        return Err(Unsent::Cannot(
            "the run's ships lie in more than one force".to_string(),
        ));
    };
    let route = match &planner.route {
        Some(route) if route.in_area => route,
        _ => return Err(Unsent::Cannot("no route in the area".to_string())),
    };
    let timing = planner
        .timing_loaded(&found.afloat, route)
        .ok_or_else(|| Unsent::Cannot("no time can be worked for the passage".to_string()))?;
    let mut waypoints = route.out.clone();
    waypoints.extend(route.back.iter().copied());
    // Decision 0145: the run timed to the dark, or not sent.
    let (night_words, speed) = night_dispatch(planner, force, &found.afloat, &waypoints)?;
    // Decision 0126: an escort, or the run held, or it sails alone.
    let (escort_words, escort) = match crate::escort::for_run(
        &crate::escort::RunContext::of(planner),
        force,
        &waypoints,
        timing.speed_kn,
        today,
    ) {
        crate::escort::ForRun::Held(why) => return Err(Unsent::Held(why)),
        crate::escort::ForRun::Sails { words, escort } => (words, escort),
    };
    let mut new_days = run.days;
    new_days.sail = today;
    new_days.arrive = today + timing.out_days - 1;
    new_days.free = today + timing.free_after_sail();
    new_days.load = run.days.load.min(today);
    if new_days != run.days {
        out.command(
            CommandKind::TimeRun {
                plan: plan.id,
                run: number,
                days: new_days,
            },
            serving(intent, plan)
                .evidence(
                    format!("run {number} is loaded and ready on day {today}"),
                    today,
                )
                .worked(format!(
                    "Run {number} was timed to sail on day {}; it is ready on day {today}, so it sails today, is due at {} on day {} and its ships are free on day {}.",
                    run.days.sail, params.destination, new_days.arrive, new_days.free
                ))
                .would_change("a load that was not yet complete")
                .done(),
        );
        if new_days.free != run.days.free {
            let ships: BTreeSet<ShipId> = found.afloat.iter().copied().collect();
            out.command(
                CommandKind::FreeShips {
                    plan: plan.id,
                    ships: ships.clone(),
                },
                serving(intent, plan)
                    .worked(format!(
                        "The commitment of run {number}'s ships to {} is made again for the new timing.",
                        plan_for(planner.view, intent, plan)
                    ))
                    .done(),
            );
            out.command(
                CommandKind::CommitShips {
                    plan: plan.id,
                    run: Some(number),
                    ships,
                    from_day: today,
                    to_day: new_days.free - 1,
                },
                serving(intent, plan)
                    .worked(format!(
                        "Run {number}'s ships are committed to {} from day {today} to day {}, the day before they are free.",
                        plan_for(planner.view, intent, plan),
                        new_days.free - 1
                    ))
                    .done(),
            );
        }
    }
    let act_at: BTreeSet<Hex> = BTreeSet::from([params.destination_hex]);
    let hour_of_sail = Fixed::from_int(24) * whole(today - 1);
    let arrival_hour = hour_of_sail + timing.out_hours;
    let worked = format!(
        "Run {number} of {}: the {}{} aboard are its load of {}; the load is complete, so it sails today (day {today}, hour {hour_of_sail}). \
         {} nm out at {} kn = {} hours, so it arrives at {} at hour {arrival_hour} (day {}); {} nm back = {} hours. \
         It acts at {}: {} landed in the {} hours of day {} left, at {}; unloading takes {} hours, {} {}.",
        plan.id,
        tons(found.aboard_t),
        ships_as_of_note(planner.view, &found.afloat),
        tons(run.load_t),
        route.out_nm,
        timing.speed_kn,
        timing.out_hours,
        params.destination,
        new_days.arrive,
        route.back_nm,
        timing.back_hours,
        params.destination,
        tons(timing.landed_on_arrival_t),
        timing.hours_left_on_arrival,
        new_days.arrive,
        rates(&timing.unload.lines),
        timing.unload.hours,
        timing.unload.days,
        days_word(timing.unload.days),
    );
    let reason = |extra: &str| -> Reason {
        serving(intent, plan)
            .evidence(
                format!(
                    "{} aboard run {number}'s ships, its load",
                    tons(found.aboard_t)
                ),
                today,
            )
            .evidence(format!("{} is released", plan.id), today)
            .binding(format!("area: {}", area_text(params.area)))
            .worked({
                let worked = match &night_words {
                    Some(words) => format!("{worked} {words}"),
                    None => worked.clone(),
                };
                match &escort_words {
                    Some(words) => format!("{worked} {words} {extra}"),
                    None => format!("{worked} {extra}"),
                }
            })
            .would_change("a load not complete, or a plan not released")
            .done()
    };
    if let Some((escort, escort_reason)) = escort {
        out.task(
            intent.id,
            Who::Existing(escort),
            Step::Escort {
                charge: Some(Who::Existing(force)),
            },
            escort_reason,
        );
    }
    if let Some((speed, speed_words)) = speed {
        out.task(
            intent.id,
            Who::Existing(force),
            Step::Speed(speed),
            serving(intent, plan)
                .evidence(night_words.clone().unwrap_or_default(), today)
                .binding("a run by night arrives in the night")
                .worked(speed_words)
                .would_change("no enemy base within 15 hexes of the destination, or press")
                .done(),
        );
    }
    out.task(
        intent.id,
        Who::Existing(force),
        Step::Route { waypoints, act_at },
        reason(
            "This is the route order: out to the destination and back, acting at the destination.",
        ),
    );
    out.task(
        intent.id,
        Who::Existing(force),
        Step::Unload,
        reason("This is the unload order the route acts on."),
    );
    Ok(())
}

fn area_text(area: Option<&BTreeSet<Hex>>) -> String {
    match area {
        None => "none set, the sea is open".to_string(),
        Some(area) => format!("{} hexes", area.len()),
    }
}

fn rates(lines: &[(ShipId, Fixed, Fixed)]) -> String {
    let mut text = String::new();
    for (i, (ship, _, rate)) in lines.iter().enumerate() {
        if i > 0 {
            text.push_str(", ");
        }
        text.push_str(&format!("{rate} t an hour for {ship}"));
    }
    text
}

/// The slowest ship's load line, for the words: "ship:1 360 t at 50 t an
/// hour = 7.2 hours".
fn slowest_line(work: &crate::planner::Work) -> String {
    let mut best: Option<(ShipId, Fixed, Fixed, Fixed)> = None;
    for (ship, tons, rate) in &work.lines {
        if *tons <= Fixed::ZERO || *rate <= Fixed::ZERO {
            continue;
        }
        let hours = *tons / *rate;
        if best.is_none_or(|(_, _, _, h)| hours > h) {
            best = Some((*ship, *tons, *rate, hours));
        }
    }
    match best {
        Some((ship, tons, rate, hours)) => {
            format!("{ship} {tons} t at {rate} t an hour = {hours} hours")
        }
        None => "nothing to move".to_string(),
    }
}

/// The commands and orders of a new run: the plan's next run, its ships
/// committed, the force formed or joined, and the load.
fn new_run(
    planner: &Planner<'_, '_>,
    out: &mut Out,
    morning: &mut Morning,
    readings: &Readings,
    ev: &Evaluation,
    run: &RunPlan,
    (displaced, held): (&[Displacement], Option<&WithoutHold>),
) -> usize {
    let today = planner.today;
    let params = &planner.params;
    let intent = params.intent;
    let plan = params.plan;
    let number = plan.runs.keys().next_back().map_or(1, |last| last + 1);
    morning.push(TakenRun {
        intent: intent.id,
        plan: plan.id,
        run: number,
        ships: run.ships.iter().copied().collect(),
        source: params.source,
        load_t: run.taken_t,
        days: run.days,
    });
    let days = run.days;
    let stock_today = readings.stock(today).unwrap_or(Fixed::ZERO);
    let use_today = readings.use_on(today).unwrap_or(Fixed::ZERO);
    let figure = match readings.days(today) {
        Some(days) => format!(" = {days} days"),
        None => " (no figure)".to_string(),
    };
    let mut worked = format!(
        "Reading today at {}{}: {} over {} a day{figure}, against a floor of {} days. \
         A run ordered on day {today} loads on day {today} ({}), sails on day {}, and is due at {} on day {}. \
         Without it the reading of day {} is {} against {} days x {} = {}: below the floor, so a run is needed. \
         The stock projected for the reading of day {} is {}. \
         Load = max(0, {} - {}) + {} ({} days of use) = {} t.",
        params.destination,
        as_of_note(planner.view, CommandRef::Base(params.destination)),
        tons(stock_today),
        tons(use_today),
        params.floor_days,
        loading_words(&run.timing),
        days.sail,
        params.destination,
        days.arrive,
        ev.check_day,
        tons(ev.check_stock_t),
        params.floor_days,
        tons(ev.check_use_t),
        tons(whole(params.floor_days) * ev.check_use_t),
        ev.arrive,
        tons(ev.stock_at_arrival_t),
        ev.floor_t,
        ev.stock_at_arrival_t,
        ev.interval_use_t,
        params.interval_days,
        ev.uncapped_t,
    );
    if !run.cuts.is_empty() {
        worked.push_str(&format!(
            " It is cut to {}: {}.",
            tons(run.load_t),
            run.cuts.join("; ")
        ));
        worked.push_str(&displaced_sentence(displaced, params.source));
    }
    if !run.aboard_before_t.is_zero() {
        worked.push_str(&format!(
            " The ships have {} aboard, which counts toward the load: the run carries the {} it has aboard",
            tons(run.aboard_before_t),
            tons(run.aboard_before_t)
        ));
        if run.taken_t.is_zero() {
            if ev.uncapped_t < run.aboard_before_t {
                worked.push_str(&format!(
                    "; its load is {} by the sum, which is less than is aboard, so the run carries what is aboard and no load order is needed.",
                    tons(ev.uncapped_t)
                ));
            } else {
                worked.push_str(" and no load order is needed.");
            }
        } else {
            worked.push_str(&format!(
                " and takes {} more, {} in all.",
                tons(run.taken_t),
                tons(run.load_t)
            ));
        }
    }
    if let Some(note) = held {
        worked.push(' ');
        worked.push_str(&note.sentence());
    }
    if let Some(night) = &run.night {
        worked.push(' ');
        worked.push_str(night);
    }
    worked.push_str(&format!(
        " Ships {}{}: {} x {} = a room that covers the load.",
        ships_text(&run.ships),
        ships_as_of_note(planner.view, &run.ships.to_vec()),
        run.ships.len(),
        "the pool's free ships in number order"
    ));
    worked.push_str(&format!(
        " Passage {} nm out at {} kn = {} hours, {} {}; {} nm back = {} hours, {} {}; unloading {} at {} = {} hours, {} {}.{}",
        planner.route.as_ref().map_or(0, |r| r.out_nm),
        run.timing.speed_kn,
        run.timing.out_hours,
        run.timing.out_days,
        days_word(run.timing.out_days),
        planner.route.as_ref().map_or(0, |r| r.back_nm),
        run.timing.back_hours,
        run.timing.back_days,
        days_word(run.timing.back_days),
        tons(run.weight_t),
        params.destination,
        run.timing.unload.hours,
        run.timing.unload.days,
        days_word(run.timing.unload.days),
        free_words(&run.timing, &days),
    ));
    let base_reason = |extra: &str| -> ReasonBuilder {
        let mut b = serving(intent, plan)
            .evidence(
                format!(
                    "{}{} holds {} and uses {} a day",
                    params.destination,
                    as_of_note(planner.view, CommandRef::Base(params.destination)),
                    tons(stock_today),
                    tons(use_today)
                ),
                today,
            )
            .evidence(
                format!(
                    "the reading of day {} is projected at {}",
                    ev.check_day,
                    tons(ev.check_stock_t)
                ),
                today,
            )
            .binding(format!(
                "floor of {} days at {}",
                params.floor_days, params.destination
            ))
            .binding(format!(
                "interval of {} days{}",
                params.interval_days,
                if params.interval_set {
                    ""
                } else {
                    " (the staff's, none set)"
                }
            ))
            .binding(format!("area: {}", area_text(params.area)))
            .worked(format!("{worked} {extra}"))
            .would_change(format!(
                "a reading of day {} at or above {} would call no run",
                ev.check_day,
                tons(whole(params.floor_days) * ev.check_use_t)
            ))
            .would_change("a free ship of the pool earlier in number order would carry it instead");
        for cut in &run.cuts {
            b = b.binding(cut.clone());
        }
        for other in other_plans(displaced, plan.id) {
            b = b.binding(format!("displaced by {other}"));
        }
        if let Some(note) = held {
            b = note.apply(b, today);
        }
        b
    };
    let added = out.command(
        CommandKind::AddRun {
            plan: plan.id,
            run: RunSpec {
                ships: run.ships.iter().copied().collect(),
                load_t: run.load_t,
                days,
            },
        },
        base_reason("The staff adds this run to the plan.").done(),
    );
    out.command(
        CommandKind::CommitShips {
            plan: plan.id,
            run: Some(number),
            ships: run.ships.iter().copied().collect(),
            from_day: today,
            to_day: days.free - 1,
        },
        base_reason(&format!(
            "The ships are committed from day {today} to day {}, the day before they are free.",
            days.free - 1
        ))
        .done(),
    );

    // The orders: form or join, then the load.
    let target = form_or_join(planner, out, run, &base_reason);
    // Nothing to take, no load order: the supply aboard is the load.
    if run.taken_t > Fixed::ZERO {
        out.task(
            intent.id,
            target,
            Step::LoadTo {
                weight_t: run.weight_t,
            },
            base_reason(&format!(
                "Load order: until {} is aboard ({} aboard now + the load of {}); {}.",
                tons(run.weight_t),
                tons(run.aboard_before_t),
                tons(run.taken_t),
                slowest_line(&run.timing.load)
            ))
            .done(),
        );
    }
    added
}

fn loading_words(timing: &Timing) -> String {
    format!(
        "{} hours at the source = {} {}",
        timing.load.hours,
        timing.load.days,
        days_word(timing.load.days)
    )
}

fn ships_text(ships: &[ShipId]) -> String {
    ships
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

/// Forms the run's ships lying loose into a force, and joins forces that lie
/// apart; returns the force that will load.
fn form_or_join(
    planner: &Planner<'_, '_>,
    out: &mut Out,
    run: &RunPlan,
    base_reason: &dyn Fn(&str) -> ReasonBuilder,
) -> Who {
    let params = &planner.params;
    let forces: Vec<&Unit> = run.units.iter().filter(|u| u.force.is_some()).collect();
    let loose: Vec<ShipId> = run
        .units
        .iter()
        .filter(|u| u.force.is_none())
        .flat_map(|u| u.ships.iter().copied())
        .collect();
    let intent = params.intent.id;
    let home = forces.first().and_then(|u| u.force);
    match (home, loose.is_empty()) {
        (None, _) => out.form(
            intent,
            params.source,
            loose.clone(),
            base_reason(&format!(
                "Form {} lying in port at {} in no force into a force.",
                ships_text(&loose),
                params.source
            ))
            .done(),
        ),
        (Some(home), loose_empty) => {
            if !loose_empty {
                let formed = out.form(
                    intent,
                    params.source,
                    loose.clone(),
                    base_reason(&format!(
                        "Form {} lying in port at {} in no force into a force, to join it to the force that loads.",
                        ships_text(&loose),
                        params.source
                    ))
                    .done(),
                );
                out.task(
                    intent,
                    formed,
                    Step::Join {
                        into: Who::Existing(home),
                    },
                    base_reason("Join the new force to the force that loads.").done(),
                );
            }
            for unit in forces.iter().skip(1) {
                if let Some(other) = unit.force {
                    out.task(
                        intent,
                        Who::Existing(other),
                        Step::Join {
                            into: Who::Existing(home),
                        },
                        base_reason("Join this force to the force that loads, so the run's ships are one force.")
                            .done(),
                    );
                }
            }
            Who::Existing(home)
        }
    }
}

/// What the staff found on a day it gave an intent no order.
struct IdleFacts {
    eff: PlanState,
    underway: Vec<String>,
    loading: bool,
    waiting_for_launch: bool,
    cannot_sail: Option<String>,
    evaluation: Option<Evaluation>,
    /// The shortfall of the evaluation, where it has a blocker.
    shortfall: Option<Shortfall>,
    pending_t: Fixed,
    units: Vec<Unit>,
    /// What the staff would have done with no hold, where a hold is what
    /// keeps it from a run.
    hold_note: Option<WithoutHold>,
}

/// What the staff would have done with no hold (ruling 21): the ships it
/// would have sent, and the holds that kept them.
pub(crate) struct WithoutHold {
    pub(crate) ships: Vec<ShipId>,
    pub(crate) holds: Vec<(HoldTarget, Hold)>,
}

impl WithoutHold {
    /// "Without the hold, ship:1 would have been sent."
    fn sentence(&self) -> String {
        format!(
            "Without the hold, {} would have been sent.",
            ships_text(&self.ships)
        )
    }

    /// The hold words as evidence and binding, and what would change it.
    fn apply(&self, mut builder: ReasonBuilder, today: u32) -> ReasonBuilder {
        for (target, hold) in &self.holds {
            let words = hold_words(*target, *hold);
            builder = builder
                .evidence(words.clone(), today)
                .binding(words)
                .binding(format!("the hold on {target}"))
                .would_change(format!("{target} released: it would then be free"));
        }
        builder
    }
}

/// Evaluates the run again on the free units counted as if no hold existed,
/// the same day on the same readings (ruling 21). It says something where
/// that choice takes a held ship the real one did not, or where the real one
/// is blocked for want of a ship and the hold-free one is not.
fn without_hold(
    planner: &Planner<'_, '_>,
    readings: &Readings,
    ev: &Evaluation,
    pending_t: Fixed,
) -> Option<WithoutHold> {
    if planner.holds.is_empty() {
        return None;
    }
    let free = planner.free_units_without_holds();
    let again = planner.evaluate(readings, planner.today, &free, pending_t)?;
    let would = again.run?;
    let real: Vec<ShipId> = ev
        .run
        .as_ref()
        .map(|run| run.ships.clone())
        .unwrap_or_default();
    let holds = planner.holds_on(&would.ships, &real);
    if holds.is_empty() {
        return None;
    }
    Some(WithoutHold {
        ships: would.ships,
        holds,
    })
}

/// The one reason of a day with no order: what the runs are doing, and the
/// reading the next run waits for with the day it is expected to be ordered.
fn idle_reason(
    planner: &Planner<'_, '_>,
    readings: &Readings,
    idle: &IdleFacts,
    today: u32,
) -> Reason {
    let params = &planner.params;
    let stock = readings.stock(today).unwrap_or(Fixed::ZERO);
    let used = readings.use_on(today).unwrap_or(Fixed::ZERO);
    let figure = match readings.days(today) {
        Some(days) => format!(" = {days} days"),
        None => " (no figure)".to_string(),
    };
    let mut worked = format!(
        "Reading today at {}{}: {} over {} a day{figure}, against a floor of {} days ({}).",
        params.destination,
        as_of_note(planner.view, CommandRef::Base(params.destination)),
        tons(stock),
        tons(used),
        params.floor_days,
        tons(whole(params.floor_days) * used)
    );
    for line in &idle.underway {
        worked.push(' ');
        worked.push_str(line);
    }
    if let Some(why) = &idle.cannot_sail {
        worked.push_str(&format!(" The loaded run cannot sail: {why}."));
    }
    if idle.loading {
        worked.push_str(" One run loads at a time, so no new run is considered.");
    } else if let Some(ev) = &idle.evaluation {
        let floor_check = whole(params.floor_days) * ev.check_use_t;
        worked.push_str(&format!(
            " A run ordered today would be due on day {}; the reading of day {} is projected at {} against {} days x {} = {}: {}.",
            ev.arrive,
            ev.check_day,
            tons(ev.check_stock_t),
            params.floor_days,
            tons(ev.check_use_t),
            tons(floor_check),
            if ev.needed { "below the floor" } else { "not below the floor, so no run is needed" }
        ));
        if let Some(blocker) = &ev.blocker {
            let detail = blocker.detail();
            worked.push_str(&format!(
                " It cannot be met: {}{}.",
                blocker.binds(),
                if detail.is_empty() {
                    String::new()
                } else {
                    format!(" ({detail})")
                }
            ));
            if let Some(short) = &idle.shortfall {
                worked.push_str(&format!(
                    " Short {} {} at the reading of day {}: {} against {}.",
                    short.shortfall_days,
                    crate::units::days_word_of(short.shortfall_days),
                    short.check_day,
                    tons(ev.check_stock_t),
                    tons(floor_check)
                ));
                worked.push_str(&displaced_sentence(&short.displaced_by, params.source));
            }
        }
    }
    let mut binding = vec![
        format!(
            "floor of {} days at {}",
            params.floor_days, params.destination
        ),
        format!(
            "interval of {} days{}",
            params.interval_days,
            if params.interval_set {
                ""
            } else {
                " (the staff's, none set)"
            }
        ),
    ];
    if let Some(ev) = &idle.evaluation {
        if let Some(blocker) = &ev.blocker {
            binding.push(blocker.binds());
            if matches!(blocker, Blocker::NoRoute) {
                binding.push("no route in the area".to_string());
            }
        }
    }
    let displaced: &[Displacement] = idle
        .shortfall
        .as_ref()
        .map_or(&[][..], |s| s.displaced_by.as_slice());
    for other in other_plans(displaced, params.plan.id) {
        binding.push(format!("displaced by {other}"));
    }
    // The next run: the first day from tomorrow on that a run ordered would
    // be needed, from the projection, and not before a ship of the pool is
    // free where none is today.
    let units: Vec<Unit> = idle.units.clone();
    let from = if units.is_empty() {
        planner
            .earliest_free()
            .map_or(today + 1, |free| free.max(today + 1))
    } else {
        today + 1
    };
    let next = planner.next_needed(readings, from, &units, idle.pending_t);
    let mut builder = serving(params.intent, params.plan).evidence(
        format!(
            "{}{} holds {} and uses {} a day",
            params.destination,
            as_of_note(planner.view, CommandRef::Base(params.destination)),
            tons(stock),
            tons(used)
        ),
        today,
    );
    if let Some(note) = &idle.hold_note {
        worked.push(' ');
        worked.push_str(&note.sentence());
        builder = note.apply(builder, today);
    }
    match &next {
        Some(next) => {
            worked.push_str(&format!(
                " The next run waits for the reading of day {} to fall below {} days ({}); it is expected to be ordered on day {}, due on day {}.",
                next.check_day,
                params.floor_days,
                tons(whole(params.floor_days) * next.check_use_t),
                next.order_day,
                next.arrive
            ));
            builder = builder.would_change(format!(
                "a reading of day {} below {} would call a run on day {}",
                next.check_day,
                tons(whole(params.floor_days) * next.check_use_t),
                next.order_day
            ));
        }
        None => {
            worked.push_str(&format!(
                " No run is expected within {LOOK_AHEAD_DAYS} days of the projection."
            ));
        }
    }
    if idle.waiting_for_launch || idle.eff == PlanState::Preparing {
        builder = builder
            .binding("leave to launch is withheld")
            .would_change("leave to launch given, or the commander approving the plan");
    }
    for line in binding {
        builder = builder.binding(line);
    }
    for change in displacement_changes(displaced, params.plan.id) {
        builder = builder.would_change(change);
    }
    builder =
        builder.would_change("a larger use at the destination would bring the next run forward");
    builder.worked(worked).done()
}

/// The shortfall of a run that cannot be met in full: what binds, the days
/// short on the day the run would arrive and the day the source falls to its
/// reserve at the present rate.
pub(crate) fn shortfall(
    planner: &Planner<'_, '_>,
    readings: &Readings,
    ev: &Evaluation,
    sent_t: Fixed,
    pending_t: Fixed,
    units: Option<&[Unit]>,
) -> Shortfall {
    let params = &planner.params;
    let binds = match &ev.blocker {
        Some(blocker) => blocker.binds(),
        None => match &ev.run {
            Some(run) if run.cuts.iter().any(|c| c.starts_with("the source")) => {
                if planner
                    .view
                    .bases()
                    .find(|b| b.id == params.source)
                    .is_some_and(|b| b.reserve_of(sim_core::orders::Cargo::Supply) > Fixed::ZERO)
                {
                    "the source's stock above its reserve".to_string()
                } else {
                    "the source's stock".to_string()
                }
            }
            Some(_) => "the hex uses more than the pool can carry".to_string(),
            None => "the intent cannot be met".to_string(),
        },
    };
    let have_days = over(ev.check_stock_t + sent_t, ev.check_use_t);
    let shortfall_days = (whole(params.floor_days) - have_days).max(Fixed::ZERO);
    let own_use = if params.source_hex == params.destination_hex {
        Fixed::ZERO
    } else {
        planner
            .view
            .days_of_supply(params.source_hex)
            .map_or(Fixed::ZERO, |figure| figure.planned_use.total_t())
    };
    let rate = readings.use_on(readings.today).unwrap_or(Fixed::ZERO) + own_use;
    let (available, _, _) = planner.source_available(pending_t);
    let source_dry_day =
        (!rate.is_zero()).then(|| readings.today + crate::units::floor_whole(available / rate));
    let displaced_by = units.map_or_else(Vec::new, |units| displacements(planner, ev, units));
    Shortfall {
        binds,
        shortfall_days,
        source_dry_day,
        check_day: ev.check_day,
        displaced_by,
    }
}

/// The commitments to other plans that keep the intent from a run or cut
/// its load, listed whole (the intent's own plan's among them) where at
/// least one is another plan's; none where none is (ruling 14). A ship held
/// where no ship is free or the room cuts the load; tons where the source
/// binds or cuts it.
fn displacements(planner: &Planner<'_, '_>, ev: &Evaluation, units: &[Unit]) -> Vec<Displacement> {
    let cut_by = |prefix: &str| {
        ev.run
            .as_ref()
            .is_some_and(|run| run.cuts.iter().any(|cut| cut.starts_with(prefix)))
    };
    let (ships_held, tons_held) = match &ev.blocker {
        Some(Blocker::NoShip { .. }) => (true, false),
        Some(Blocker::Source { .. }) => (false, true),
        Some(_) => (false, false),
        None => (cut_by("the ships' room"), cut_by("the source")),
    };
    let mut holders = Vec::new();
    if ships_held {
        holders.extend(planner.ship_holders(units));
    }
    if tons_held {
        holders.extend(planner.tons_holders());
    }
    if holders.iter().any(|h| h.plan != planner.params.plan.id) {
        holders
    } else {
        Vec::new()
    }
}

/// The plans, other than `own`, that the displacements name, in order.
fn other_plans(displaced: &[Displacement], own: PlanId) -> BTreeSet<PlanId> {
    displaced
        .iter()
        .map(|d| d.plan)
        .filter(|plan| *plan != own)
        .collect()
}

/// One commitment in words: "ship:1 committed to run 1 of plan:1 (intent:1)
/// from day 1 to day 4", or "30 t at base:1 still to be taken by run 1 of
/// plan:1 (intent:1), committed from day 1 to day 1".
pub(crate) fn displacement_words(d: &Displacement, source: BaseId) -> String {
    match &d.committed {
        Committed::Ships(ships) => {
            let ships = ships
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ");
            let to = if d.run == 0 {
                format!("{} ({})", d.plan, d.intent)
            } else {
                format!("run {} of {} ({})", d.run, d.plan, d.intent)
            };
            format!(
                "{ships} committed to {to} from day {} to day {}",
                d.from_day, d.to_day
            )
        }
        Committed::Tons(t) => format!(
            "{} at {source} still to be taken by run {} of {} ({}), committed from day {} to day {}",
            tons(*t),
            d.run,
            d.plan,
            d.intent,
            d.from_day,
            d.to_day
        ),
    }
}

/// " It is displaced by A; B." or nothing where nothing displaces.
pub(crate) fn displaced_sentence(displaced: &[Displacement], source: BaseId) -> String {
    if displaced.is_empty() {
        return String::new();
    }
    format!(
        " It is displaced by {}.",
        displaced
            .iter()
            .map(|d| displacement_words(d, source))
            .collect::<Vec<_>>()
            .join("; ")
    )
}

/// What would change it, for each other plan that displaces: its commitment
/// of ships ending, with the earliest day a ship is free, or its run loading
/// its tons.
fn displacement_changes(displaced: &[Displacement], own: PlanId) -> Vec<String> {
    let mut changes = Vec::new();
    for other in other_plans(displaced, own) {
        let ships = displaced
            .iter()
            .filter(|d| d.plan == other && matches!(d.committed, Committed::Ships(_)))
            .min_by_key(|d| d.to_day);
        if let Some(Displacement {
            committed: Committed::Ships(ships),
            to_day,
            ..
        }) = ships
        {
            changes.push(format!(
                "{other}'s commitment ending: {} free on day {}",
                ships
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", "),
                to_day + 1
            ));
        }
        if displaced
            .iter()
            .any(|d| d.plan == other && matches!(d.committed, Committed::Tons(_)))
        {
            changes.push(format!("{other}'s run loading its tons"));
        }
    }
    changes
}

/// The fallback of a cancelled plan (ruling 29): each force of the plan on
/// passage out (its remaining route still holds the destination, and it is
/// not stopped there) is given a route home to the source inside the area,
/// with no mark to act at. Its cargo stays aboard and its unload order
/// stands, so it is put back ashore at the source. A force that has begun to
/// unload at the destination, or is loading at the source, is given no
/// order, and neither is a force that is held. Where it gives one it says
/// so, and the reason of the day with no order is not needed.
fn send_home(
    view: &SideView<'_>,
    fleet: &Fleet<'_>,
    state: &CommandState,
    params: &Params<'_>,
    out: &mut Out,
    today: u32,
) -> bool {
    let plan = params.plan;
    let intent = params.intent;
    let mut sent: BTreeSet<TaskForceId> = BTreeSet::new();
    let mut gave = false;
    for run in plan.runs.values().filter(|run| run.ended.is_none()) {
        let found = run_movement(fleet, state, plan.id, run, params.source_hex, today);
        if found.status != RunStatus::Underway {
            continue;
        }
        let mut forces: BTreeSet<TaskForceId> = BTreeSet::new();
        let mut loose = false;
        for ship in &found.afloat {
            match fleet.force_of(*ship) {
                Some(force) => {
                    forces.insert(force.id);
                }
                None => loose = true,
            }
        }
        let [id] = forces.iter().copied().collect::<Vec<_>>()[..] else {
            continue;
        };
        if loose || !sent.insert(id) {
            continue;
        }
        let Some(force) = fleet.forces.get(&id) else {
            continue;
        };
        let ahead = force
            .route
            .waypoints
            .get(force.route.next_index.min(force.route.waypoints.len())..)
            .unwrap_or(&[]);
        let at_destination = force.toward_hex.is_none() && force.from_hex == params.destination_hex;
        if !ahead.contains(&params.destination_hex) || at_destination {
            continue;
        }
        if state.hold_on_force(id, &force.ships).is_some() {
            continue;
        }
        let waypoints = route_between(view, force.position(), params.source_hex, params.area)
            .map_or_else(|| vec![params.source_hex], |route| route.out);
        out.task(
            intent.id,
            Who::Existing(id),
            Step::Route {
                waypoints,
                act_at: BTreeSet::new(),
            },
            serving(intent, plan)
                .evidence(format!("{} is {}", plan.id, plan.state.name()), today)
                .evidence(
                    format!("{id} is on passage out to {}", params.destination),
                    today,
                )
                .binding(format!("area: {}", area_text(params.area)))
                .worked(format!(
                    "{} is cancelled: {id} on passage out to {} is sent home to {}; its cargo stays aboard and its unload order stands, so it is put back ashore at {}.",
                    plan.id, params.destination, params.source, params.source
                ))
                .would_change("a plan that was not cancelled")
                .done(),
        );
        gave = true;
    }
    gave
}

/// Step 7: the ships of a plan that has ended, committed from today on, are
/// freed.
fn free_ended(view: &SideView<'_>, out: &mut Out) {
    let state = view.command_state();
    let today = view.day() + 1;
    for plan in state.plans.values().filter(|plan| plan.state.ended()) {
        let ships: BTreeSet<ShipId> = state
            .commitments
            .iter()
            .filter(|(_, list)| list.iter().any(|c| c.plan == plan.id && c.to_day >= today))
            .map(|(ship, _)| *ship)
            .collect();
        if ships.is_empty() {
            continue;
        }
        let reason = Reason {
            serves: sim_core::command::Serves {
                intent: Some(plan.intent),
                plan: Some(plan.id),
                amendment: plan.amendments.last().map(|a| a.number),
                statement: String::new(),
                chain: Vec::new(),
                level: None,
            },
            evidence: vec![sim_core::command::Evidence {
                what: format!("{} is {}", plan.id, plan.state.name()),
                day: today,
            }],
            binding: vec![format!("{} is {}", plan.id, plan.state.name())],
            worked: format!(
                "{} is {} and {} of its ships are committed from today on: the staff frees them ({}).",
                state
                    .intents
                    .get(&plan.intent)
                    .map_or_else(|| plan.id.to_string(), |intent| plan_for(view, intent, plan)),
                plan.state.name(),
                ships.len(),
                ships.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", ")
            ),
            would_change: vec!["a plan that was not ended".to_string()],
        };
        out.command(
            CommandKind::FreeShips {
                plan: plan.id,
                ships,
            },
            reason,
        );
    }
}

/// How a run's free day is counted, in words (decision 0112). Where the
/// source gives no fuel rate the ships are free the morning after they are
/// back, as before; where it gives one, the morning after the day the
/// fuelling ends.
fn free_words(timing: &crate::planner::Timing, days: &sim_core::command::RunDays) -> String {
    match timing.fuelling_rate {
        None => format!(
            " Its ships are free on day {} ({} + {} + {} + {}).",
            days.free,
            days.sail,
            timing.out_days,
            timing.unload.days,
            timing.back_days,
        ),
        Some(rate) => format!(
            " Back at the source at hour {} of the run and fuelled by hour {} ({} hours at {} t an hour), its ships are free on day {}, the morning after the day the fuelling ends.",
            timing.back_and_fuelled_hour() - timing.fuelling_hours,
            timing.back_and_fuelled_hour(),
            timing.fuelling_hours,
            rate,
            days.free,
        ),
    }
}

/// The rate each of `ships` lands at at the destination (the berths first,
/// in ship order, then by lighter), with what each holds.
fn landing_rates(
    planner: &Planner<'_, '_>,
    lines: &[(ShipId, Fixed)],
) -> Vec<(ShipId, Fixed, Fixed)> {
    planner
        .work(planner.params.destination_hex, lines)
        .map(|work| work.lines)
        .unwrap_or_default()
}

/// The two speeds tried for a run's ships: by the engine's forecast where
/// they lie in one force, else by the rules it steams by on the route.
fn tried_for(planner: &Planner<'_, '_>, ships: &[ShipId]) -> Vec<crate::night::Tried> {
    let params = &planner.params;
    let Some(route) = planner.route.as_ref() else {
        return Vec::new();
    };
    let forces: BTreeSet<TaskForceId> = ships
        .iter()
        .filter_map(|s| planner.fleet.force_of(*s).map(|f| f.id))
        .collect();
    let all_in_one = forces.len() == 1
        && ships.iter().all(|s| planner.fleet.force_of(*s).is_some())
        && forces
            .iter()
            .next()
            .and_then(|f| planner.fleet.forces.get(f))
            .is_some_and(|f| f.ships.iter().all(|s| ships.contains(s)));
    if all_in_one {
        let force = *forces.iter().next().expect("one force");
        let mut waypoints = route.out.clone();
        waypoints.extend(route.back.iter().copied());
        crate::night::try_force(planner.view, force, params.destination_hex, &waypoints)
    } else {
        crate::night::try_ships(
            planner.view,
            ships,
            route.out_nm,
            route.out_nm + route.back_nm,
        )
    }
}

/// Each ship's load against its limit, in words.
fn limits_words(lines: &[(ShipId, Fixed, Fixed)], night: Fixed) -> String {
    lines
        .iter()
        .map(|(ship, tons, rate)| {
            format!(
                "{ship} carries {tons} t against a limit of {rate} t an hour x {night} hours = {} t",
                *rate * night
            )
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// Decision 0145, 4 to 8, on the morning a run is ordered: where it goes by
/// night, the speed it will go at, and its load cut to the night's limit.
fn night_load(planner: &Planner<'_, '_>, run: &mut RunPlan) {
    let params = &planner.params;
    let risk = planner.state().risk_read_up(params.intent.id);
    let Some((base, hexes)) = crate::night::applies(planner.view, risk, params.destination_hex)
    else {
        return;
    };
    let near = crate::night::base_near_words(planner.view, base, hexes, params.destination_hex);
    let tried = tried_for(planner, &run.ships);
    let tried_text = crate::night::tried_words(&tried);
    let after: Vec<(ShipId, Fixed)> = planner
        .distribute(&run.ships, run.taken_t)
        .into_iter()
        .map(|(ship, _, after)| (ship, after))
        .collect();
    let rated = landing_rates(planner, &after);
    let loads: Vec<(Fixed, Fixed)> = rated.iter().map(|(_, tons, rate)| (*tons, *rate)).collect();
    let Some(chosen) = crate::night::choose(&tried, &loads) else {
        let then = match risk {
            Risk::Preserve => "at preserve the force it is loaded and will not sail while no speed brings it there in the dark",
            _ => "at balanced it goes by day at cruising speed",
        };
        run.night = Some(format!(
            "By night: {near}; {tried_text}: neither speed brings it there in the night, so {then}."
        ));
        return;
    };
    let night = chosen.night.unwrap_or(Fixed::ZERO);
    let lines: Vec<(Fixed, Fixed, Fixed)> = run
        .ships
        .iter()
        .zip(rated.iter())
        .filter_map(|(id, (_, _, rate))| {
            let ship = planner.fleet.ships.get(id)?;
            Some((ship.supply_t, planner.ship_room(ship), *rate * night))
        })
        .collect();
    let most = crate::night::weight_within(&lines);
    let mut cut_words = String::new();
    if run.weight_t > most {
        let before = run.weight_t;
        let taken = (most - run.aboard_before_t).max(Fixed::ZERO);
        run.taken_t = taken;
        run.weight_t = run.aboard_before_t + taken;
        run.load_t = run.weight_t;
        let cut = format!(
            "the night's limit: no ship carries more than it can land in the night's {night} hours, so the load of {before} t is cut to {} t",
            run.weight_t
        );
        cut_words = format!(" The load of {before} t is cut to {} t.", run.weight_t);
        run.cuts.push(cut);
    }
    let after: Vec<(ShipId, Fixed)> = planner
        .distribute(&run.ships, run.taken_t)
        .into_iter()
        .map(|(ship, _, after)| (ship, after))
        .collect();
    let rated = landing_rates(planner, &after);
    run.night = Some(format!(
        "By night: {near}; {tried_text}. It goes at {}, with {night} hours of the night left on arrival: {}.{cut_words} Its days are reckoned at cruising speed.",
        crate::night::speed_name(chosen.speed),
        limits_words(&rated, night)
    ));
}

/// Decision 0145, 5, 11 and 12, on the morning a loaded run sails: its
/// words, and the speed order where the speed differs from the force's; or
/// the run not sent at preserve the force where no speed brings it there in
/// the dark.
#[allow(clippy::type_complexity)]
fn night_dispatch(
    planner: &Planner<'_, '_>,
    force: TaskForceId,
    ships: &[ShipId],
    waypoints: &[Hex],
) -> Result<
    (
        Option<String>,
        Option<(sim_core::orders::SpeedSetting, String)>,
    ),
    Unsent,
> {
    use sim_core::orders::SpeedSetting;
    let params = &planner.params;
    let risk = planner.state().risk_read_up(params.intent.id);
    let Some((base, hexes)) = crate::night::applies(planner.view, risk, params.destination_hex)
    else {
        return Ok((None, None));
    };
    let Some(current) = planner.fleet.forces.get(&force).map(|f| f.speed) else {
        return Ok((None, None));
    };
    let near = crate::night::base_near_words(planner.view, base, hexes, params.destination_hex);
    let tried = crate::night::try_force(planner.view, force, params.destination_hex, waypoints);
    let tried_text = crate::night::tried_words(&tried);
    let aboard: Vec<(ShipId, Fixed)> = ships
        .iter()
        .filter_map(|id| planner.fleet.ships.get(id).map(|s| (*id, s.supply_t)))
        .collect();
    let rated = landing_rates(planner, &aboard);
    let loads: Vec<(Fixed, Fixed)> = rated.iter().map(|(_, tons, rate)| (*tons, *rate)).collect();
    match crate::night::choose(&tried, &loads) {
        Some(chosen) => {
            let night = chosen.night.unwrap_or(Fixed::ZERO);
            let speed_name = crate::night::speed_name(chosen.speed);
            let words = format!(
                "By night: {near}; {tried_text}. It goes at {speed_name}, with {night} hours of the night left on arrival: {}; it lands its load in the night and steams on at the morning.",
                limits_words(&rated, night)
            );
            let order = (current != chosen.speed).then(|| {
                (
                    chosen.speed,
                    format!("{near}; {tried_text}: {force} is set to {speed_name}, the first that brings it there in the night with the fuel and room for its load."),
                )
            });
            Ok((Some(words), order))
        }
        None if risk == Risk::Preserve => Err(Unsent::Held(format!(
            "{near}; {tried_text}: neither speed brings it there in the night, and at preserve the force it does not sail by day"
        ))),
        None => {
            let words = format!(
                "By night: {near}; {tried_text}: neither speed brings it there in the night, so at balanced it goes by day at cruising speed."
            );
            let order = (current != SpeedSetting::Cruise).then(|| {
                (
                    SpeedSetting::Cruise,
                    format!("{near}; {tried_text}: neither speed brings it there in the night, so {force} is set to cruising speed and goes by day."),
                )
            });
            Ok((Some(words), order))
        }
    }
}
