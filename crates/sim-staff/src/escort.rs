//! Escorted and covered runs (decision 0126), at the area's level, where
//! "keep supplied" is: the doctrine that uses decision 0121's escort rule.
//!
//! - **What the commander names.** A task force named in the forces limit
//!   of a keep-supplied intent on the escort mission is an escort the staff
//!   may give to a run (rule 1). One on the surface combat mission is a
//!   covering force, given a stand task by the air doctrine's forming rule
//!   (rule 7, `crate::stand`). With no task force named, a run sails as it
//!   always did and nothing here is read (rule 2).
//! - **The threat to a run** (rule 3) is read on the morning the run is
//!   routed: a contact of ships the side holds within
//!   [`crate::stand::THREAT_HEXES`] of the destination, the nearest, the
//!   lowest number on a tie. Every contact counts at every risk level: the
//!   run is no force that could outnumber one.
//! - **Whether it is escorted** (rule 4): at preserve the force always,
//!   where an escort is free, and with a threat and none free it does not
//!   sail and the staff hands in that it cannot meet the intent; at balanced
//!   on a threat, where one is free, else it sails alone and says so; at
//!   press never.
//! - **A free escort** (rule 5): a named force on the escort mission with a
//!   ship afloat, stopped at the run's source, with no charge, not held,
//!   whose every ship has the fuel for the passage out and back by the
//!   engine's measure at the run's speed, times the risk's margin. The
//!   lowest number is taken.
//! - **The orders** (rule 6): the escort is given the run's force as its
//!   charge on the morning the run is routed; on a later morning on which
//!   its charge is stopped at the source, its charge is cleared.

use std::collections::BTreeSet;

use sim_core::command::{Addressee, Risk};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::TaskForceId;
use sim_core::orders::{Mission, SpeedSetting};
use sim_core::state::TaskForce;
use sim_core::view::SideView;

use crate::handin::{Out, Step, Who};
use crate::planner::Planner;
use crate::reasons::serving;
use crate::stand::{afloat, margin, stopped, threat, Sighted, THREAT_HEXES};
use crate::units::tons;

/// What the doctrine decided for a run about to be routed.
pub(crate) enum ForRun {
    /// It sails. `words` is said in the route's reason (none where no task
    /// force is named); `escort`, the escort it takes and the reason of its
    /// order.
    Sails {
        words: Option<String>,
        escort: Option<(TaskForceId, sim_core::command::Reason)>,
    },
    /// It does not sail today (preserve the force, a threat and no escort
    /// free): why, for the reason of the day.
    Held(String),
}

/// The forces the intent's forces limit names that are on the escort
/// mission and have a ship afloat, in number order.
fn named_escorts<'v>(planner: &Planner<'v, '_>) -> Vec<&'v TaskForce> {
    let view = planner.view;
    planner
        .params
        .forces
        .task_forces
        .iter()
        .filter_map(|id| view.task_forces().find(|f| f.id == *id))
        .filter(|f| f.mission == Mission::Escort && !afloat(view, f.id).is_empty())
        .collect()
}

/// The threat to a run to `destination` (rule 3), in words, with the contact
/// where there is one.
fn run_threat(
    view: &SideView<'_>,
    addressee: Addressee,
    destination: Hex,
) -> (Option<Sighted>, String) {
    // The picture of the command the intent is addressed to (decision 0134,
    // 13).
    let picture = crate::picture::picture_of(view, addressee);
    let mut sighted: Vec<Sighted> = picture
        .contacts()
        .map(|c| Sighted {
            number: picture.contact_number(c.id),
            hexes: c.estimated_hex.distance(destination),
            ships: c.estimated_ship_count,
        })
        .collect();
    sighted.sort_by_key(|c| (c.hexes, c.number));
    // Every contact counts: the balanced reading of decision 0124's threat.
    let found = threat(Risk::Balanced, u32::MAX, &sighted);
    let words = match found {
        Some(c) => format!(
            "{} of {} ship(s) is {} hexes from {destination}, within {THREAT_HEXES}: the run is threatened",
            crate::picture::contact_words(&picture, c.number),
            c.ships,
            c.hexes
        ),
        None => format!(
            "no contact of ships is held within {THREAT_HEXES} hexes of {destination}: the run is not threatened"
        ),
    };
    (found, words)
}

/// Whether `force` is a free escort for a run through `waypoints` at
/// `speed_kn` from `source` (rule 5): `Ok` with the fuel figures in words,
/// or `Err` with why not.
fn free_escort(
    planner: &Planner<'_, '_>,
    force: &TaskForce,
    source: Hex,
    waypoints: &[Hex],
    speed_kn: Fixed,
    risk: Risk,
) -> Result<String, String> {
    let id = force.id;
    // The escort as the area command holds it, and the moment that is of
    // where it is not the pulse just ended (decision 0136, 11).
    let held = crate::reasons::force_held_words(planner.view, id);
    if let Some(day) = crate::available::force_waiting(planner.view, id) {
        return Err(format!("{held} {}", crate::available::from_day_words(day)));
    }
    if !(stopped(force) && force.position() == source) {
        return Err(format!("{held} is not stopped at the source {source}"));
    }
    if let Some(charge) = force.charge {
        return Err(format!("{held} escorts {} already", charge.force));
    }
    if let Some(hold) = planner.state().hold_on_force(id, &force.ships) {
        return Err(format!("{hold} is held by the commander"));
    }
    // Decision 0127, 2: a force short of ammunition is not free.
    if let Some(short) = crate::ammunition::short(planner.view, id, risk) {
        return Err(format!("{held} is not free: {}", short.words));
    }
    let need_margin = margin(risk);
    let Some(forecast) = planner
        .view
        .forecast_route(id, waypoints, Some(SpeedSetting::Knots(speed_kn)))
        .filter(|f| f.refusal.is_none())
    else {
        return Err(format!(
            "{held} cannot be routed out and back by the engine's measure"
        ));
    };
    // The engine's forecast stops where a ship runs dry: such a ship's need
    // is the whole passage at its class's rate, as the engine burns it.
    let route_nm = planner.route.as_ref().map_or(Fixed::ZERO, |r| {
        Fixed::from_int(i64::from(r.out_nm + r.back_nm))
    });
    let needs: Vec<(sim_core::ids::ShipId, Fixed, Fixed)> = forecast
        .ships
        .iter()
        .map(|line| {
            let burned = if line.runs_dry {
                planner
                    .view
                    .ships()
                    .find(|ship| ship.id == line.ship)
                    .and_then(|ship| planner.view.ship_class(ship.class))
                    .map_or(line.burned_t, |class| {
                        let multiplier = if speed_kn <= class.cruise_speed_kn {
                            Fixed::ONE
                        } else {
                            let ratio = speed_kn / class.cruise_speed_kn;
                            ratio * ratio
                        };
                        route_nm.mul_div(class.fuel_capacity_t, class.endurance_nm) * multiplier
                    })
            } else {
                line.burned_t
            };
            (line.ship, line.fuel_t, burned * need_margin)
        })
        .collect();
    let words: Vec<String> = needs
        .iter()
        .map(|(ship, aboard, need)| {
            format!(
                "{ship} has {} aboard against {} needed",
                tons(*aboard),
                tons(*need)
            )
        })
        .collect();
    let figures = format!(
        "the passage out and back at {speed_kn} kn, times {need_margin} at {} risk: {}",
        risk.name(),
        words.join(", ")
    );
    if needs.iter().any(|(_, aboard, need)| aboard < need) {
        return Err(format!("{held} is short of fuel for {figures}"));
    }
    Ok(figures)
}

/// Rules 3 to 6 and 8 for a run of `run_force` about to be routed through
/// `waypoints` at `speed_kn`. With no task force named, the run sails as it
/// always did and nothing is said (rule 2).
pub(crate) fn for_run(
    planner: &Planner<'_, '_>,
    run_force: TaskForceId,
    waypoints: &[Hex],
    speed_kn: Fixed,
    today: u32,
) -> ForRun {
    let params = &planner.params;
    if params.forces.task_forces.is_empty() {
        return ForRun::Sails {
            words: None,
            escort: None,
        };
    }
    let view = planner.view;
    let intent = params.intent;
    let risk = planner.state().risk_read_up(intent.id);
    let (found, threat_words) = run_threat(view, intent.kind.addressee(), params.destination_hex);
    let wanted = match risk {
        Risk::Preserve => true,
        Risk::Balanced => found.is_some(),
        Risk::Press => false,
    };
    if !wanted {
        let why = match risk {
            Risk::Press => format!(
                "At {} risk a run is never escorted ({threat_words}).",
                risk.name()
            ),
            _ => format!(
                "At {} risk a run is escorted on a threat, and {threat_words}, so it sails alone.",
                risk.name()
            ),
        };
        return ForRun::Sails {
            words: Some(why),
            escort: None,
        };
    }
    let mut refused: Vec<String> = Vec::new();
    let mut taken: Option<(TaskForceId, String)> = None;
    for force in named_escorts(planner) {
        match free_escort(planner, force, params.source_hex, waypoints, speed_kn, risk) {
            Ok(figures) => {
                taken = Some((force.id, figures));
                break;
            }
            Err(why) => refused.push(why),
        }
    }
    let none_free = if refused.is_empty() {
        "no force on the escort mission is named in the forces limit".to_string()
    } else {
        refused.join("; ")
    };
    match taken {
        Some((escort, figures)) => {
            let escort_held = crate::reasons::force_held_words(planner.view, escort);
            let run_held = crate::reasons::force_held_words(planner.view, run_force);
            let worked = format!(
                "At {} risk a run is escorted {} where an escort is free; {threat_words}. {escort_held} is the lowest numbered free escort: it is stopped at the source {}, has no charge, is not held, and has the fuel for {figures}. So {escort} is given {run_held}, the run's force, as its charge, and keeps station on it.",
                risk.name(),
                if risk == Risk::Preserve { "always" } else { "on a threat" },
                params.source,
            );
            let reason = serving(intent, params.plan)
                .evidence(threat_words.clone(), today)
                .evidence(format!("{escort_held} is free: {figures}"), today)
                .binding(format!("{escort} is named in the forces limit of {}", intent.id))
                .worked(worked)
                .would_change("no threat held at balanced risk, or press risk")
                .would_change("the escort held, short of fuel or away from the source")
                .done();
            ForRun::Sails {
                words: Some(format!("{threat_words}; {escort} escorts it.")),
                escort: Some((escort, reason)),
            }
        }
        None if risk == Risk::Preserve && found.is_some() => ForRun::Held(format!(
            "at {} risk a threatened run does not sail without an escort: {threat_words}, and no escort is free ({none_free})",
            risk.name()
        )),
        None => ForRun::Sails {
            words: Some(if found.is_some() {
                format!("A threat is believed ({threat_words}) and no escort is free ({none_free}), so the run sails alone.")
            } else {
                format!("{threat_words}; no escort is free ({none_free}), so the run sails alone.")
            }),
            escort: None,
        },
    }
}

/// Rule 6, the second half: each named escort whose charge is stopped at
/// the source, and is not held, has its charge cleared this morning.
pub(crate) fn clear_charges(planner: &Planner<'_, '_>, out: &mut Out, today: u32) {
    let params = &planner.params;
    if params.forces.task_forces.is_empty() {
        return;
    }
    let view = planner.view;
    let mut cleared: BTreeSet<TaskForceId> = BTreeSet::new();
    for escort in named_escorts(planner) {
        let Some(charge) = escort.charge else {
            continue;
        };
        if planner
            .state()
            .hold_on_force(escort.id, &escort.ships)
            .is_some()
        {
            continue;
        }
        let Some(run) = view.task_forces().find(|f| f.id == charge.force) else {
            continue;
        };
        if !(stopped(run) && run.position() == params.source_hex) || run.ships.is_empty() {
            continue;
        }
        if !cleared.insert(escort.id) {
            continue;
        }
        let reason = serving(params.intent, params.plan)
            .evidence(
                format!(
                    "{} is back and stopped at the source {}",
                    crate::reasons::force_held_words(planner.view, charge.force),
                    params.source
                ),
                today,
            )
            .worked(format!(
                "The run of {} is home, so {} has its charge cleared; it is free again on the morning after it has fuelled.",
                charge.force, escort.id
            ))
            .would_change("the run not yet home")
            .done();
        out.task(
            params.intent.id,
            Who::Existing(escort.id),
            Step::Escort { charge: None },
            reason,
        );
    }
}
