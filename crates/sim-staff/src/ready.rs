//! Units made ready, and a base built up (decision 0141, step 2 of stage
//! 3): the two tasks a commander may state that give orders to land units.
//!
//! - **Ready a landing** (items 1 to 7), at the level of its task force:
//!   the force is given `prepare_landing` for the task's hex with the task's
//!   units as its roster, where its preparation differs; each fighting unit
//!   on the roster is set each morning to train or to stand ready (which on
//!   a roster is to rehearse) by its experience, the risk and the date; and
//!   the staff hands in a notice once when the task is ready, and once when
//!   it sees the task cannot be ready by its date.
//! - **Build up a base** (items 8 to 12), at the level of its base: each
//!   unit the task may use, at the base and not held, is set to the job of
//!   the first want not yet met; with every want met the staff gives no
//!   order and hands in a notice once.
//!
//! Both (items 13 to 16): a unit serves one task, the higher in priority; a
//! hold is honoured; nothing is ordered without leave to prepare (the plan
//! is preparing or released); a unit before its day is left out.
//!
//! The doctrine reads the side's own view and its planned command state and
//! nothing else. "Once" is read from the side's own events: a notice of the
//! kind already handed in for the plan is not handed in again.

use std::collections::{BTreeMap, BTreeSet};

use sim_core::command::{
    Intent, IntentKind, NoticeBasis, NoticeKind, Plan, PlanState, Reason, Risk, StaffNotice,
    StaffOrder, Want,
};
use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{EntityRef, IntentId, LandUnitId};
use sim_core::orders::{Activity, Job, LandUnitOrder, OrderKind, TaskForceOrder};
use sim_core::refdata::GroundFigures;
use sim_core::side_events::SideEventKind;
use sim_core::state::{Base, LandUnit, Skills};

use crate::air::Day;
use crate::reasons::{hold_words, serving};

/// A land unit's service's standard of experience (decision 0097, 1): 2.
const STANDARD: u8 = 2;

/// The whole days `need` takes at `rate` a day, rounded up: 0 where nothing
/// is needed, `None` where the rate is 0 and something is.
fn days_for(need: Fixed, rate: Fixed) -> Option<u32> {
    if need <= Fixed::ZERO {
        return Some(0);
    }
    if rate <= Fixed::ZERO {
        return None;
    }
    let quotient = need / rate;
    let whole = quotient.trunc();
    let days = if Fixed::from_int(whole) < quotient {
        whole + 1
    } else {
        whole
    };
    u32::try_from(days).ok()
}

fn days_word(days: u32) -> &'static str {
    if days == 1 {
        "day"
    } else {
        "days"
    }
}

fn risk_name(risk: Risk) -> &'static str {
    match risk {
        Risk::Preserve => "preserve the force",
        Risk::Balanced => "balanced",
        Risk::Press => "press",
    }
}

/// What the task asks before it is ready, by risk level (item 5): the
/// force's loading, each unit's rehearsal, and whether each unit must be at
/// the standard.
fn thresholds(risk: Risk) -> (Fixed, Fixed, bool) {
    match risk {
        Risk::Preserve => (Fixed::from_int(100), Fixed::from_int(100), true),
        Risk::Balanced => (Fixed::from_int(100), Fixed::from_int(100), false),
        Risk::Press => (Fixed::from_int(50), Fixed::from_int(50), false),
    }
}

/// A unit's name in a reason: "land_unit:1 (Placeholder Raw)".
fn unit_words(unit: &LandUnit) -> String {
    format!("{} ({})", unit.id, unit.name)
}

/// A fighting unit on a landing's roster as the doctrine reads it this
/// morning.
struct Member {
    id: LandUnitId,
    words: String,
    experience: u8,
    training_days: u32,
    activity: Activity,
    /// Its rehearsal for the task's hex: 0 where it is for another.
    rehearsal: Fixed,
    /// The rehearsal it gains in a day: `preparation_per_day` times its
    /// assault factor.
    gain: Fixed,
    can_train: bool,
}

impl Member {
    fn below(&self) -> bool {
        self.experience < STANDARD
    }

    /// The days of training left to the standard (decision 0097, 3).
    fn training_left(&self, figures: &GroundFigures) -> u32 {
        let levels = u32::from(STANDARD.saturating_sub(self.experience));
        (levels * figures.training_days_per_level).saturating_sub(self.training_days)
    }

    /// The days its rehearsal still needs to reach `to`.
    fn rehearsal_days(&self, to: Fixed) -> Option<u32> {
        days_for(to - self.rehearsal, self.gain)
    }
}

/// What a projection of the landing found binding (item 6).
enum Binds {
    Loading,
    Training(String),
    Rehearsal(String),
}

impl Binds {
    fn words(&self) -> String {
        match self {
            Binds::Loading => "the force's loading".to_string(),
            Binds::Training(unit) => format!("the training of {unit}"),
            Binds::Rehearsal(unit) => format!("the rehearsal of {unit}"),
        }
    }
}

impl Day<'_, '_> {
    /// Serves every "ready a landing" and "build up a base" task in force
    /// whose plan has leave to prepare, in priority order, each taking the
    /// units no higher task has taken (item 13).
    pub(crate) fn readies(&mut self) {
        let tasks = self.in_priority_order(|i| i.kind.readies_units());
        let mut taken: BTreeMap<LandUnitId, IntentId> = BTreeMap::new();
        for id in tasks {
            let Some(task) = self.state.intents.get(&id).cloned() else {
                continue;
            };
            let Some(plan) = self.plan_of(&task) else {
                continue;
            };
            // Item 15: nothing is ordered without leave to prepare.
            if !matches!(plan.state, PlanState::Preparing | PlanState::Released) {
                continue;
            }
            // Decision 0142, 20: the ready task under "take a place" is
            // given nothing from the morning its force is routed.
            if let IntentKind::ReadyLanding { force, hex, .. } = task.kind {
                let under_place = task
                    .parent
                    .and_then(|p| self.state.intents.get(&p))
                    .is_some_and(|p| matches!(p.kind, IntentKind::TakePlace { .. }));
                let routed = self
                    .view
                    .task_forces()
                    .find(|f| f.id == force)
                    .is_some_and(|f| crate::take::routed(f, hex));
                // Decision 0143, 11: after the first landing a later lift
                // goes as the force stands, with no landing made ready.
                let landed = self
                    .state
                    .land_units_on_chain(task.id)
                    .is_some_and(|units| {
                        units.iter().any(|id| {
                            self.view
                                .land_units()
                                .any(|u| u.id == *id && u.men_ashore > 0 && u.hex == Some(hex))
                        })
                    });
                if under_place && (routed || landed) {
                    continue;
                }
            }
            match task.kind {
                IntentKind::ReadyLanding { .. } => self.serve_landing(&task, &plan, &mut taken),
                IntentKind::BuildUp { .. } => self.serve_build_up(&task, &plan, &mut taken),
                _ => {}
            }
        }
    }

    /// Whether a notice of `kind` was handed in for `plan` on an earlier
    /// morning: what makes a notice handed in once.
    pub(crate) fn noticed(&mut self, kind: NoticeKind, plan: sim_core::command::PlanId) -> bool {
        self.side_events().iter().any(|event| {
            matches!(
                &event.kind,
                SideEventKind::Whole(EventKind::StaffNotice { kind: k, plan: p, .. })
                    if *k == kind && *p == plan
            )
        })
    }

    /// Hands in `reason` as a decision to give no order, with a notice of
    /// `kind` resting on it.
    pub(crate) fn notice(&mut self, in_scope: bool, reason: Reason, kind: NoticeKind, plan: &Plan) {
        let out = self.target(in_scope);
        out.no_orders.push(reason);
        let index = out.no_orders.len() - 1;
        out.notices.push(StaffNotice {
            kind,
            plan: plan.id,
            run: None,
            rests_on: NoticeBasis::NoOrder(index),
        });
    }

    /// The land units the task may use before it is screened (items 1, 8
    /// and 13): those named in every forces limit on its chain; with none, a
    /// build-up's working units of the side with men ashore at its base.
    fn units_named(&self, task: &Intent) -> Vec<LandUnitId> {
        match self.state.land_units_on_chain(task.id) {
            Some(allowed) => allowed.into_iter().collect(),
            None => match task.kind {
                IntentKind::BuildUp { base, .. } => {
                    let hex = self.view.bases().find(|b| b.id == base).map(|b| b.hex);
                    self.view
                        .land_units()
                        .filter(|u| {
                            u.working && u.men_ashore > 0 && u.hex.is_some() && u.hex == hex
                        })
                        .map(|u| u.id)
                        .collect()
                }
                _ => Vec::new(),
            },
        }
    }

    /// The units the task may order this morning, in unit order, and the
    /// words for each it leaves out: not the side's or with no man left, before
    /// its day (item 16), held (item 14), taken by a higher task (item 13),
    /// or, with `at`, not ashore in that hex. The units kept are taken.
    fn screen(
        &self,
        task: &Intent,
        at: Option<Hex>,
        taken: &mut BTreeMap<LandUnitId, IntentId>,
    ) -> (Vec<LandUnit>, Vec<String>) {
        let mut kept = Vec::new();
        let mut left = Vec::new();
        for id in self.units_named(task) {
            let Some(unit) = self.view.land_units().find(|u| u.id == id) else {
                left.push(format!("{id} is not one of the side's units now"));
                continue;
            };
            let aboard = self.view.men_aboard(id).unwrap_or(0);
            if unit.men_ashore == 0 && aboard == 0 {
                left.push(format!("{} has no man left", unit_words(unit)));
                continue;
            }
            if let Some(day) = crate::available::not_before(self.view, EntityRef::LandUnit(id)) {
                left.push(crate::available::named_with_day(unit_words(unit), day));
                continue;
            }
            let hold = sim_core::command::HoldTarget::LandUnit(id);
            if let Some(found) = self.state.hold_of(hold) {
                left.push(format!(
                    "{}; the staff plans without it",
                    hold_words(hold, *found)
                ));
                continue;
            }
            if let Some(other) = taken.get(&id) {
                left.push(format!(
                    "{} serves {other}, which is higher in priority",
                    unit_words(unit)
                ));
                continue;
            }
            if let Some(hex) = at {
                if unit.men_ashore == 0 || unit.hex != Some(hex) {
                    left.push(format!("{} is not ashore at the base", unit_words(unit)));
                    continue;
                }
            }
            taken.insert(id, task.id);
            kept.push(unit.clone());
        }
        (kept, left)
    }

    /// The days from the morning a landing by `force` at `hex` with `units`
    /// on its roster is ordered to the morning it would be ready, by the
    /// projection of item 6 (decision 0159, 6: whether a take begun on a later
    /// day can still be ready by its date). `days_left` is the days from that
    /// morning to the task's date, where it has one. `None` where it cannot
    /// be ready.
    pub(crate) fn days_to_ready(
        &self,
        force: &sim_core::state::TaskForce,
        hex: Hex,
        units: &[LandUnitId],
        risk: Risk,
        days_left: Option<i64>,
    ) -> Option<u32> {
        let figures = self.view.ground_figures()?;
        let found: Vec<LandUnit> = units
            .iter()
            .filter_map(|id| self.view.land_units().find(|u| u.id == *id).cloned())
            .collect();
        let roster: BTreeSet<LandUnitId> = found.iter().map(|u| u.id).collect();
        let prepared = force.preparation.as_ref();
        let differs = prepared.is_none_or(|p| p.hex != hex || p.roster != roster);
        let loading_now = prepared
            .filter(|p| p.hex == hex)
            .map_or(Fixed::ZERO, |p| p.loading);
        let kept = force_preparation_kept(prepared, hex, &roster, figures);
        let loading_after = if differs { kept } else { loading_now };
        let (load_needed, rehearsal_needed, _) = thresholds(risk);
        let loading_days = days_for(load_needed - loading_after, figures.preparation_per_day);
        let members = members_of(self.view, &found, hex, figures);
        let (days, _) = project(
            &members,
            loading_days,
            rehearsal_needed,
            risk,
            days_left,
            figures,
        );
        days.map(|d| d.max(u32::from(differs)))
    }

    // -- Ready a landing ----------------------------------------------------

    #[allow(clippy::too_many_lines)]
    fn serve_landing(
        &mut self,
        task: &Intent,
        plan: &Plan,
        taken: &mut BTreeMap<LandUnitId, IntentId>,
    ) {
        let IntentKind::ReadyLanding {
            force: force_id,
            hex,
            by_day,
        } = task.kind
        else {
            return;
        };
        let in_scope = self.in_scope(task.id);
        let today = self.today;
        let view = self.view;
        let risk = self.state.risk_read_up(task.id);
        let Some(figures) = view.ground_figures() else {
            let reason = serving(task, plan)
                .evidence("no ground figures are loaded", today)
                .worked(format!(
                    "With no ground figures no landing is prepared, so the staff gives no order under {}.",
                    task.id
                ))
                .done();
            self.target(in_scope).no_orders.push(reason);
            return;
        };
        let Some(force) = view.task_forces().find(|f| f.id == force_id).cloned() else {
            let reason = serving(task, plan)
                .evidence(
                    format!("{force_id} is not one of the side's forces now"),
                    today,
                )
                .worked(format!(
                    "{force_id} is gone, so the staff gives no order under {}.",
                    task.id
                ))
                .would_change("a new task for another force")
                .done();
            self.target(in_scope).no_orders.push(reason);
            return;
        };
        if let Some(day) = crate::available::force_waiting(view, force_id) {
            let reason = serving(task, plan)
                .evidence(crate::available::named_with_day(force_id, day), today)
                .binding(crate::available::named_with_day(force_id, day))
                .worked(format!(
                    "{} {}, so the staff gives no order under {} before then.",
                    force_id,
                    crate::available::from_day_words(day),
                    task.id
                ))
                .would_change("the day coming")
                .done();
            self.target(in_scope).no_orders.push(reason);
            return;
        }
        let (units, left) = self.screen(task, None, taken);
        let left_words = if left.is_empty() {
            String::new()
        } else {
            format!(" Left out: {}.", left.join("; "))
        };
        let roster: BTreeSet<LandUnitId> = units.iter().map(|u| u.id).collect();
        let risk_text = risk_name(risk);
        let date_text = by_day.map_or_else(
            || "no date".to_string(),
            |day| format!("a date of the morning of day {day}"),
        );
        let days_left = by_day.map(|day| i64::from(day) - i64::from(today));

        // The force's order (item 2).
        let held_force = self.state.hold_on_force(force_id, &force.ships);
        let prepared = force.preparation.as_ref();
        let differs = prepared.is_none_or(|p| p.hex != hex || p.roster != roster);
        let loading_now = prepared
            .filter(|p| p.hex == hex)
            .map_or(Fixed::ZERO, |p| p.loading);
        let mut orders: Vec<StaffOrder> = Vec::new();
        let mut landing_ordered = false;
        if differs {
            let roster_words = if roster.is_empty() {
                "no unit".to_string()
            } else {
                roster
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            let was = match prepared {
                None => "no landing prepared".to_string(),
                Some(p) => format!(
                    "a landing prepared at {} with {} on its roster and loading {}",
                    p.hex,
                    if p.roster.is_empty() {
                        "no unit".to_string()
                    } else {
                        p.roster
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(", ")
                    },
                    p.loading
                ),
            };
            if let Some(hold) = held_force {
                let found = self.state.hold_of(hold).copied();
                let reason = serving(task, plan)
                    .evidence(format!("{force_id} has {was}"), today)
                    .binding(found.map_or_else(|| format!("{hold} is held"), |h| hold_words(hold, h)))
                    .worked(format!(
                        "{force_id} is held, so the staff gives it no landing order for {hex}.{left_words}"
                    ))
                    .would_change("the hold released")
                    .done();
                self.target(in_scope).no_orders.push(reason);
            } else {
                let reason = serving(task, plan)
                    .evidence(format!("{force_id} has {was}"), today)
                    .worked(format!(
                        "{force_id} is to land at {hex} with {roster_words}: the staff gives it the landing order with that roster.{left_words}"
                    ))
                    .would_change("another hex, or other units in the forces limit")
                    .done();
                orders.push(StaffOrder {
                    kind: OrderKind::TaskForce {
                        target: force_id,
                        order: TaskForceOrder::PrepareLanding {
                            hex,
                            roster: roster.clone(),
                        },
                    },
                    intent: task.id,
                    reason,
                });
                landing_ordered = true;
            }
        }

        // Each fighting unit's day (items 3 and 4).
        let members = members_of(view, &units, hex, figures);
        let hundred = Fixed::from_int(100);
        for m in &members {
            let needs = m.rehearsal_days(hundred);
            let needs_words = needs.map_or_else(
                || "it gains no rehearsal in a day, so it can never be rehearsed".to_string(),
                |d| format!("needs {d} {} at {} a day", days_word(d), m.gain),
            );
            let state_words = format!(
                "{} is at experience {}, {} the standard of {STANDARD}, with {} of the {} days of training a level takes counted; its rehearsal for {hex} is {} and {needs_words}",
                m.words,
                m.experience,
                if m.below() { "below" } else { "at or above" },
                m.training_days,
                figures.training_days_per_level,
                m.rehearsal,
            );
            let (train, why) = if !m.below() {
                (false, "it is at the standard, so it rehearses".to_string())
            } else if !m.can_train {
                (
                    false,
                    "it cannot train (it has men aboard, is at no base of its side, or training takes it no higher), so it rehearses".to_string(),
                )
            } else {
                match risk {
                    Risk::Preserve => (
                        true,
                        "at preserve the force it trains until it is at the standard, whatever the date".to_string(),
                    ),
                    Risk::Press => (
                        false,
                        "at press it does not train: it rehearses from the first day".to_string(),
                    ),
                    Risk::Balanced => match (days_left, needs) {
                        (None, _) => (
                            true,
                            "at balanced with no date it trains to the standard first".to_string(),
                        ),
                        (Some(left_days), Some(need)) if left_days > i64::from(need) => (
                            true,
                            format!(
                                "at balanced, {left_days} days are left before the date, more than the {need} its rehearsal needs, so it trains"
                            ),
                        ),
                        (Some(left_days), _) => (
                            false,
                            format!(
                                "at balanced, {left_days} days are left before the date, no more than its rehearsal needs, so it rehearses"
                            ),
                        ),
                    },
                }
            };
            let want = if train {
                Activity::Train
            } else {
                Activity::Ready
            };
            if m.activity == want {
                continue;
            }
            let verb = if train {
                "sets it to train"
            } else {
                "sets it to stand ready, which on the roster is to rehearse"
            };
            let reason = serving(task, plan)
                .evidence(state_words.clone(), today)
                .evidence(format!("the risk is {risk_text}, with {date_text}"), today)
                .worked(format!("{state_words}; {why}: the staff {verb}."))
                .would_change("another risk level or date, or the unit's experience")
                .done();
            orders.push(StaffOrder {
                kind: OrderKind::LandUnit {
                    target: m.id,
                    order: LandUnitOrder::SetActivity(want),
                },
                intent: task.id,
                reason,
            });
        }

        // Readiness (item 5) and the date (item 6).
        let (load_needed, rehearsal_needed, at_standard) = thresholds(risk);
        // Decision 0142, 14: a force that does not lie at a base of its side
        // can gain no loading and its units no rehearsal, so the task is
        // ready as it stands.
        let at_base = crate::fleet::force_stopped_at(&force, force.from_hex)
            && view.bases().any(|b| b.hex == force.from_hex);
        // Decision 0152, 1 (the architect's ruling of Nov 18, 2026): a lift
        // as it stands, every man of its roster aboard it and none ashore,
        // needs no making ready wherever it lies.
        let all_aboard = !roster.is_empty()
            && units.iter().all(|u| {
                let here: u32 = view
                    .ships()
                    .filter(|s| s.location == sim_core::state::ShipLocation::TaskForce(force_id))
                    .filter_map(|s| s.troops)
                    .filter(|t| t.unit == u.id)
                    .map(|t| t.men)
                    .sum();
                u.men_ashore == 0 && here > 0 && Some(here) == view.men_aboard(u.id)
            });
        let as_it_stands = (!at_base || all_aboard) && !roster.is_empty();
        let ready = as_it_stands
            || (!landing_ordered
                && !differs
                && !roster.is_empty()
                && loading_now >= load_needed
                && members
                    .iter()
                    .all(|m| m.rehearsal >= rehearsal_needed && (!at_standard || !m.below())));
        let loading_words = format!(
            "the force's loading for {hex} is {loading_now}, of the {load_needed} asked at {risk_text}"
        );
        let member_words = members
            .iter()
            .map(|m| {
                format!(
                    "{} has rehearsal {} and experience {}",
                    m.words, m.rehearsal, m.experience
                )
            })
            .collect::<Vec<_>>()
            .join("; ");
        let gave_orders = !orders.is_empty();
        for order in &orders {
            let line = match &order.kind {
                OrderKind::TaskForce { target, .. } => {
                    format!("{target} is given the landing order for {hex}.")
                }
                OrderKind::LandUnit {
                    target,
                    order: LandUnitOrder::SetActivity(activity),
                } => format!("{target} is set to {}.", activity.name()),
                _ => String::new(),
            };
            if in_scope {
                self.lines.push(line);
            }
        }
        self.target(in_scope).orders.extend(orders);

        let without = as_it_stands.then(|| {
            let rehearsals = if members.is_empty() {
                String::new()
            } else {
                format!("; {member_words}")
            };
            if at_base {
                format!(
                    "{force_id} lies at a base of its side with every man of its roster aboard: it is a lift as it stands, which needs no making ready, so the landing is ready as it stands and goes without loading or rehearsal, {loading_words}{rehearsals}"
                )
            } else {
                format!(
                    "{force_id} does not lie at a base of its side, so it can gain no loading and its units no rehearsal: the landing is ready as it stands and goes without them, {loading_words}{rehearsals}"
                )
            }
        });
        if ready {
            self.landings.insert(
                task.id,
                crate::take::LandingReading {
                    ready: true,
                    ready_on: Some(today),
                    words: format!("{loading_words}; {member_words}"),
                },
            );
            if !self.noticed(NoticeKind::Ready, plan.id) {
                let worked = match &without {
                    Some(words) => format!(
                        "{words}. The staff gives no order and hands in the notice that it is ready."
                    ),
                    None => format!(
                        "The landing at {hex} by {force_id} is ready: {loading_words}; {member_words}. The staff gives no order and hands in the notice that it is ready."
                    ),
                };
                let reason = serving(task, plan)
                    .evidence(loading_words.clone(), today)
                    .evidence(member_words.clone(), today)
                    .worked(worked)
                    .would_change("a loss of loading or rehearsal")
                    .done();
                self.notice(in_scope, reason, NoticeKind::Ready, plan);
            } else if !gave_orders {
                let reason = serving(task, plan)
                    .evidence(loading_words.clone(), today)
                    .worked(format!(
                        "The landing at {hex} by {force_id} is ready and its notice was handed in: the staff gives no order.{left_words}"
                    ))
                    .would_change("a loss of loading or rehearsal")
                    .done();
                self.target(in_scope).no_orders.push(reason);
            }
            return;
        }

        // The projection: the morning it would be ready.
        let kept = force_preparation_kept(prepared, hex, &roster, figures);
        let loading_after = if differs { kept } else { loading_now };
        let loading_days = days_for(load_needed - loading_after, figures.preparation_per_day);
        let (days, binds) = project(
            &members,
            loading_days,
            rehearsal_needed,
            risk,
            days_left,
            figures,
        );
        // A landing ordered this morning is not ready this morning (decision
        // 0147, 16: a roster that loses a unit costs no loading, so every
        // other need may already be met): the earliest is tomorrow.
        let ready_on = days.map(|d| today + d.max(u32::from(differs)));
        self.landings.insert(
            task.id,
            crate::take::LandingReading {
                ready: false,
                ready_on,
                words: format!("{loading_words}; {member_words}"),
            },
        );
        let projection = match ready_on {
            Some(morning) => format!("it would be ready on the morning of day {morning}"),
            None => "it cannot be ready while that holds".to_string(),
        };
        if let Some(date) = by_day {
            let late = ready_on.is_none_or(|morning| morning > date);
            if late && !self.noticed(NoticeKind::Late, plan.id) {
                let late_words = match ready_on {
                    Some(morning) => {
                        let d = morning - date;
                        format!("{d} {} late", days_word(d))
                    }
                    None => "it cannot be ready at all".to_string(),
                };
                let reason = serving(task, plan)
                    .evidence(loading_words.clone(), today)
                    .evidence(member_words.clone(), today)
                    .binding(binds.words())
                    .worked(format!(
                        "The landing at {hex} by {force_id} is to be ready by the morning of day {date}; at {risk_text} {projection}: {late_words}, and what binds is {}. The staff hands in the notice that it cannot be ready by its date.",
                        binds.words()
                    ))
                    .would_change("a later date, a bolder risk level, or other units")
                    .done();
                self.notice(in_scope, reason, NoticeKind::Late, plan);
            }
        }
        if !gave_orders {
            let reason = serving(task, plan)
                .evidence(loading_words.clone(), today)
                .evidence(member_words, today)
                .binding(binds.words())
                .worked(format!(
                    "The landing at {hex} by {force_id} is not ready: {loading_words}; at {risk_text} with {date_text} {projection}, and what binds is {}. Every order it needs stands: the staff gives no order.{left_words}",
                    binds.words()
                ))
                .would_change("another risk level or date")
                .done();
            self.target(in_scope).no_orders.push(reason);
        }
    }

    // -- Build up a base ----------------------------------------------------

    #[allow(clippy::too_many_lines)]
    fn serve_build_up(
        &mut self,
        task: &Intent,
        plan: &Plan,
        taken: &mut BTreeMap<LandUnitId, IntentId>,
    ) {
        let IntentKind::BuildUp {
            base: base_id,
            wants,
        } = task.kind
        else {
            return;
        };
        let in_scope = self.in_scope(task.id);
        let today = self.today;
        let view = self.view;
        let base_words = crate::reasons::base_words(view, base_id);
        let Some(base) = view.bases().find(|b| b.id == base_id).cloned() else {
            let reason = serving(task, plan)
                .evidence(
                    format!("{base_id} is not one of the side's bases now"),
                    today,
                )
                .worked(format!(
                    "{base_id} is not the side's, so the staff gives no order under {}.",
                    task.id
                ))
                .done();
            self.target(in_scope).no_orders.push(reason);
            return;
        };
        let Some(figures) = view.ground_figures() else {
            let reason = serving(task, plan)
                .evidence("no ground figures are loaded", today)
                .worked(format!(
                    "With no ground figures no unit works, so the staff gives no order under {}.",
                    task.id
                ))
                .done();
            self.target(in_scope).no_orders.push(reason);
            return;
        };
        let (units, left) = self.screen(task, Some(base.hex), taken);
        let left_words = if left.is_empty() {
            String::new()
        } else {
            format!(" Left out: {}.", left.join("; "))
        };
        // Items 10 and 11: the first want not met.
        let first = wants.iter().find(|want| !met(*want, &base));
        let Some(want) = first else {
            let met_words = wants
                .iter()
                .map(|w| format!("{w}: {}", state_of(w, &base)))
                .collect::<Vec<_>>()
                .join("; ");
            let noticed = self.noticed(NoticeKind::Met, plan.id);
            let reason = serving(task, plan)
                .evidence(met_words.clone(), today)
                .worked(format!(
                    "Every want of {base_words} is met ({met_words}): the staff gives no order{}.{left_words}",
                    if noticed {
                        ""
                    } else {
                        " and hands in the notice that the task is met"
                    }
                ))
                .would_change("a want that stops being met, as a field damaged again")
                .done();
            if noticed {
                self.target(in_scope).no_orders.push(reason);
            } else {
                self.notice(in_scope, reason, NoticeKind::Met, plan);
            }
            return;
        };
        let job = job_for(want);
        // Item 12: the work still to do, the work the units make in a day,
        // and the days that comes to.
        let per_day = units.iter().fold(Fixed::ZERO, |sum, u| {
            sum + sim_core::works::unit_work(
                u.men_ashore,
                u.fed_share,
                u.skills.works,
                u.working,
                figures,
            )
        });
        let todo = work_to_do(want, &base, figures);
        let work_words = match todo {
            Some(todo) => {
                let days = days_for(todo, per_day).map_or_else(
                    || "no day's work brings it".to_string(),
                    |d| format!("{d} {}", days_word(d)),
                );
                format!(
                    "{todo} of work to do on {want}, {per_day} a day by the units at it, {days}"
                )
            }
            None => format!("{want} is never met: it is worked, {per_day} a day by the units at it, while it is the first want left"),
        };
        let mut orders = Vec::new();
        for unit in &units {
            let to = Activity::Work(Some(job));
            if unit.activity == to {
                continue;
            }
            let share = if unit.working {
                String::new()
            } else {
                " (a fighting unit put to work makes the share of a working unit's work the figures give)".to_string()
            };
            let reason = serving(task, plan)
                .evidence(format!("{want} is the first want of {base_words} not met"), today)
                .evidence(work_words.clone(), today)
                .worked(format!(
                    "{want} is the first want of {base_words} not met: {work_words}. The staff sets {} to work at the {} job{share}.{left_words}",
                    unit_words(unit),
                    job.name()
                ))
                .would_change("the want met, or the order of the wants")
                .done();
            if in_scope {
                self.lines.push(format!(
                    "{} is set to work at the {} job.",
                    unit.id,
                    job.name()
                ));
            }
            orders.push(StaffOrder {
                kind: OrderKind::LandUnit {
                    target: unit.id,
                    order: LandUnitOrder::SetActivity(to),
                },
                intent: task.id,
                reason,
            });
        }
        if orders.is_empty() {
            let who = if units.is_empty() {
                "no unit the task may use is at the base".to_string()
            } else {
                format!("every unit is at the {} job already", job.name())
            };
            let reason = serving(task, plan)
                .evidence(format!("{want} is the first want of {base_words} not met"), today)
                .evidence(work_words.clone(), today)
                .worked(format!(
                    "{want} is the first want of {base_words} not met: {work_words}; {who}, so the staff gives no order.{left_words}"
                ))
                .would_change("a unit at the base, or the want met")
                .done();
            self.target(in_scope).no_orders.push(reason);
        }
        self.target(in_scope).orders.extend(orders);
    }
}

/// The days until a landing is ready (item 6): the longest of the force's
/// loading and each unit's training and rehearsal, with what binds; `None`
/// days where it cannot be ready while that holds.
fn project(
    members: &[Member],
    loading_days: Option<u32>,
    rehearsal_needed: Fixed,
    risk: Risk,
    days_left: Option<i64>,
    figures: &GroundFigures,
) -> (Option<u32>, Binds) {
    let hundred = Fixed::from_int(100);
    let mut worst: Option<(Option<u32>, Binds)> = Some((loading_days, Binds::Loading));
    for m in members {
        let rehearse = m.rehearsal_days(rehearsal_needed);
        let train: Option<u32> = if !m.below() {
            Some(0)
        } else {
            match risk {
                Risk::Press => Some(0),
                Risk::Preserve => m.can_train.then(|| m.training_left(figures)),
                Risk::Balanced => {
                    if !m.can_train {
                        Some(0)
                    } else {
                        match (days_left, m.rehearsal_days(hundred)) {
                            (None, _) => Some(m.training_left(figures)),
                            (Some(left_days), Some(need)) => {
                                let room = u32::try_from((left_days - i64::from(need)).max(0))
                                    .unwrap_or(0);
                                Some(m.training_left(figures).min(room))
                            }
                            (Some(_), None) => Some(0),
                        }
                    }
                }
            }
        };
        let total = match (train, rehearse) {
            (Some(t), Some(r)) => Some(t + r),
            _ => None,
        };
        let binds = if train.is_none_or(|t| t > 0) {
            Binds::Training(m.words.clone())
        } else {
            Binds::Rehearsal(m.words.clone())
        };
        let longer = match (&worst, total) {
            (Some((Some(w), _)), Some(t)) => t > *w,
            (Some((Some(_), _)), None) => true,
            _ => false,
        };
        if longer {
            worst = Some((total, binds));
        }
    }
    worst.unwrap_or((Some(0), Binds::Loading))
}

/// The fighting units of a landing's roster as the doctrine reads them this
/// morning (items 3 and 4).
fn members_of(
    view: &sim_core::view::SideView<'_>,
    units: &[LandUnit],
    hex: Hex,
    figures: &GroundFigures,
) -> Vec<Member> {
    let base_hexes: BTreeSet<Hex> = view.bases().map(|b| b.hex).collect();
    let strength = figures.assault_skill_strength;
    units
        .iter()
        .filter(|u| !u.working)
        .map(|u| {
            let aboard = view.men_aboard(u.id).unwrap_or(0);
            Member {
                id: u.id,
                words: unit_words(u),
                experience: u.experience,
                training_days: u.training_days,
                activity: u.activity,
                rehearsal: if u.rehearsal_hex == Some(hex) {
                    u.rehearsal
                } else {
                    Fixed::ZERO
                },
                gain: figures.preparation_per_day * Skills::factor(u.skills.assault, strength),
                can_train: aboard == 0
                    && u.men_ashore > 0
                    && u.hex.is_some_and(|h| base_hexes.contains(&h))
                    && u32::from(u.experience) < figures.training_ceiling,
            }
        })
        .collect()
}

/// The loading a force keeps under a new landing order (decision 0095, 8
/// to 11): 0 for a new hex or none prepared, the loading times
/// `preparation_kept_on_change` where a unit is added, else what it has.
fn force_preparation_kept(
    prepared: Option<&sim_core::state::Preparation>,
    hex: Hex,
    roster: &BTreeSet<LandUnitId>,
    figures: &GroundFigures,
) -> Fixed {
    match prepared {
        Some(p) if p.hex == hex => {
            if roster.iter().any(|u| !p.roster.contains(u)) {
                p.loading * figures.preparation_kept_on_change
            } else {
                p.loading
            }
        }
        _ => Fixed::ZERO,
    }
}

/// Whether a want is met at `base` (item 10). With no airfield there is no
/// damage to mend.
fn met(want: Want, base: &Base) -> bool {
    match want {
        Want::Airfield => base.airfield,
        Want::Mended => base.airfield_damage <= Fixed::ZERO,
        Want::Defences(points) => base.defences >= Fixed::from_int(i64::from(points)),
        Want::Port | Want::AircraftRepair => false,
    }
}

/// A want's state at the base, in words.
fn state_of(want: Want, base: &Base) -> String {
    match want {
        Want::Airfield => {
            if base.airfield {
                "the base has an airfield".to_string()
            } else {
                format!("{} of work done on the airfield", base.airfield_work)
            }
        }
        Want::Mended => format!("the airfield's damage is {}", base.airfield_damage),
        Want::Defences(_) => format!("the defences are {}", base.defences),
        Want::Port | Want::AircraftRepair => "never met".to_string(),
    }
}

/// The job that works a want (item 9).
fn job_for(want: Want) -> Job {
    match want {
        Want::Airfield | Want::Mended => Job::Airfield,
        Want::Defences(_) => Job::Defences,
        Want::Port => Job::Port,
        Want::AircraftRepair => Job::Aircraft,
    }
}

/// The work still to do on a want; `None` for one never met.
fn work_to_do(want: Want, base: &Base, figures: &GroundFigures) -> Option<Fixed> {
    match want {
        Want::Airfield => Some((figures.airfield_work - base.airfield_work).max(Fixed::ZERO)),
        Want::Mended => Some(base.airfield_damage * figures.work_per_airfield_damage_point),
        Want::Defences(points) => Some(sim_core::works::work_to_raise_defences(
            base.defences,
            Fixed::from_int(i64::from(points)),
            figures,
        )),
        Want::Port | Want::AircraftRepair => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn days_are_rounded_up_and_none_at_no_rate() {
        assert_eq!(days_for(Fixed::from_int(100), Fixed::from_int(25)), Some(4));
        assert_eq!(days_for(Fixed::from_int(101), Fixed::from_int(25)), Some(5));
        assert_eq!(days_for(Fixed::ZERO, Fixed::ZERO), Some(0));
        assert_eq!(days_for(Fixed::ONE, Fixed::ZERO), None);
    }
}
