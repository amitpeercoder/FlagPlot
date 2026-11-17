//! Taking a place (decision 0142, step 3 of stage 3): the end "take a
//! place", at the area's level, carried as far as the landing.
//!
//! - **Prepare** (items 6 to 13), [`Day::takes_prepare`], before the tasks
//!   that ready units are served: on the first morning the plan is approved
//!   the staff finds the transport force, chooses the units that land by
//!   what each is good at and by the berths of the force's ships, sizes them
//!   against the side's estimate of the enemy at the place by the risk, and
//!   forms "ready a landing" for the force at the place with the units
//!   chosen as its forces limit. The choice is made once: the task formed
//!   keeps it.
//! - **Leave and move** (items 15 to 20), [`Day::takes_launch`], after the
//!   tasks that ready units are served: when the landing is ready it
//!   releases the plan where leave to launch is given; then each morning one
//!   step: the supply loaded, then one unit ashore at the source set to
//!   attack and ordered aboard, then the route to the place and back acting
//!   at the place, with the unload order and an escort as for a run.
//! - **Land** (items 21 to 24): the landing is the unload order's; a unit
//!   ashore at the place not on attack is set to attack; on the first
//!   morning the intent is met the plan is marked complete and the notice
//!   that it is met handed in, once.
//!
//! The doctrine reads the side's own view and its planned command state and
//! nothing else: the enemy only as its public bases and the side's estimate
//! of its men.

use std::collections::BTreeSet;

use sim_core::command::{
    CommandKind, ForcesLimit, Intent, IntentKind, Leave, Limit, Maker, NoticeBasis, NoticeKind,
    Plan, PlanOutcome, PlanState, Risk, StaffCommand, StaffNotice, StaffOrder,
};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, EntityRef, LandUnitId, ShipId, TaskForceId};
use sim_core::map::HexKind;
use sim_core::orders::{
    Cargo, CargoAction, LandUnitOrder, Mission, OrderKind, Posture, TaskForceOrder,
};
use sim_core::state::{LandUnit, ShipLocation, Skills, TaskForce};

use crate::air::Day;
use crate::planner::DEFAULT_INTERVAL_DAYS;
use crate::reasons::{hold_words, serving};
use crate::units::{ceil_days, days_word, tons, whole};

/// What a "ready a landing" task found of its readiness this morning: what
/// "take a place" reads before it asks leave to launch (item 15).
#[derive(Clone, Debug)]
pub(crate) struct LandingReading {
    pub(crate) ready: bool,
    /// The morning it expects to be ready; `None` where it cannot be.
    pub(crate) ready_on: Option<u32>,
    /// The loading and each unit's rehearsal and experience, in words.
    pub(crate) words: String,
}

/// The men the staff wants for each man of the estimate, by risk (item 10).
/// The staff's figures, placeholders.
fn men_per_man(risk: Risk) -> u32 {
    match risk {
        Risk::Preserve => 3,
        Risk::Balanced => 2,
        Risk::Press => 1,
    }
}

/// Whether the force has been routed to the place: its route acts there, or
/// it is landing there (item 20, read from the force alone).
pub(crate) fn routed(force: &TaskForce, place: Hex) -> bool {
    force.route.act_at.contains(&place) || force.landing_at == Some(place)
}

fn unit_words(unit: &LandUnit) -> String {
    format!("{} ({})", unit.id, unit.name)
}

/// A unit chosen to land, with the ships it takes.
#[derive(Clone, Debug)]
struct Pick {
    id: LandUnitId,
    aboard: bool,
    ships: Vec<ShipId>,
}

/// The choice of item 6 to 10.
struct Choice {
    force: TaskForceId,
    picks: Vec<Pick>,
    /// Item 27, in the order a staff works in.
    lines: Vec<String>,
    /// The men wanted, where an estimate is held, and whether the men
    /// chosen fall short of them.
    short: Option<(u32, u32)>,
}

/// The supply a landing takes (item 16).
struct Supply {
    /// The level the force is to have aboard.
    level: Fixed,
    /// What is aboard now.
    aboard: Fixed,
    words: String,
}

impl Supply {
    fn to_take(&self) -> Fixed {
        (self.level - self.aboard).max(Fixed::ZERO)
    }
}

/// The days of item 18.
struct Days {
    embark: u32,
    passage: u32,
    words: String,
}

impl Day<'_, '_> {
    // -- Prepare --------------------------------------------------------------

    /// Items 3, 4 and 6 to 13: for each "take a place" whose plan is approved
    /// in the planned state, in priority order.
    pub(crate) fn takes_prepare(&mut self) {
        let ends = self.in_priority_order(|i| matches!(i.kind, IntentKind::TakePlace { .. }));
        for id in ends {
            let Some(end) = self.state.intents.get(&id).cloned() else {
                continue;
            };
            let Some(plan) = self.plan_of(&end) else {
                continue;
            };
            if !matches!(plan.state, PlanState::Preparing | PlanState::Released) {
                continue;
            }
            self.prepare_place(&end, &plan);
        }
    }

    /// The "ready a landing" task the staff formed under `end`, if any.
    fn landing_of(&self, end: &Intent) -> Option<Intent> {
        self.state
            .intents_in_force()
            .find(|i| i.parent == Some(end.id) && matches!(i.kind, IntentKind::ReadyLanding { .. }))
            .cloned()
    }

    /// Item 3: the side holds a base in the hex and the enemy holds none.
    fn place_is_ours(&self, place: Hex) -> bool {
        let side = self.view.side();
        self.view.bases().any(|b| b.hex == place)
            && !self
                .view
                .enemy_bases()
                .any(|b| b.hex == place && b.side != side)
    }

    #[allow(clippy::too_many_lines)]
    fn prepare_place(&mut self, end: &Intent, plan: &Plan) {
        let IntentKind::TakePlace { hex: place, by_day } = end.kind else {
            return;
        };
        let in_scope = self.in_scope(end.id);
        let today = self.today;
        // Items 3 and 23: met.
        if self.place_is_ours(place) {
            let reason = serving(end, plan)
                .evidence(
                    format!("the side holds a base at {place} and the enemy holds none there"),
                    today,
                )
                .worked(format!(
                    "The place at {place} is the side's: {} is met. The staff marks the plan complete and hands in the notice that the intent is met; it orders nothing else.",
                    end.id
                ))
                .would_change("an enemy base at the place")
                .done();
            let out = self.target(in_scope);
            out.commands.push(StaffCommand {
                kind: CommandKind::MarkPlan {
                    plan: plan.id,
                    outcome: PlanOutcome::Complete,
                },
                reason,
            });
            let index = out.commands.len() - 1;
            if !self.noticed(NoticeKind::Met, plan.id) {
                self.target(in_scope).notices.push(StaffNotice {
                    kind: NoticeKind::Met,
                    plan: plan.id,
                    run: None,
                    rests_on: NoticeBasis::Command(index),
                });
            }
            if in_scope {
                self.lines.push(format!(
                    "{} is met: the place at {place} is the side's.",
                    end.id
                ));
            }
            self.set_plan(plan.id, PlanState::Complete);
            return;
        }
        // Item 11: the choice is made once.
        if self.landing_of(end).is_some() {
            return;
        }
        let Some(figures) = self.view.ground_figures() else {
            let reason = serving(end, plan)
                .evidence("no ground figures are loaded", today)
                .worked(format!(
                    "With no ground figures no landing is made, so the staff gives no order under {}.",
                    end.id
                ))
                .done();
            self.target(in_scope).no_orders.push(reason);
            return;
        };
        // Item 4: the staff lands at the place itself.
        let coast = self
            .view
            .hex(place)
            .is_some_and(|data| data.kind == HexKind::Coast);
        if !coast {
            let reason = serving(end, plan)
                .evidence(format!("{place} is not a coast hex"), today)
                .binding("a march from a beach is step 4")
                .worked(format!(
                    "The place at {place} is not on the coast, and the staff has no way there: it lands at the place itself, so it gives no order under {}.",
                    end.id
                ))
                .would_change("a place on the coast")
                .done();
            self.target(in_scope).no_orders.push(reason);
            return;
        }
        let Some(limit) = end.limits.forces.clone() else {
            let reason = serving(end, plan)
                .evidence(format!("{} has no forces limit", end.id), today)
                .binding("a forces limit is required in this step")
                .worked(format!(
                    "{} names no task force, land unit or source, so the staff gives no order under it.",
                    end.id
                ))
                .would_change("a forces limit with a source")
                .done();
            self.target(in_scope).no_orders.push(reason);
            return;
        };
        let Some((source, source_hex)) = limit
            .source
            .and_then(|s| self.view.bases().find(|b| b.id == s).map(|b| (s, b.hex)))
        else {
            let reason = serving(end, plan)
                .evidence("the forces limit names no source of the side", today)
                .worked(format!(
                    "{} has no source of the side to sail from, so the staff gives no order under it.",
                    end.id
                ))
                .would_change("a source of the side")
                .done();
            self.target(in_scope).no_orders.push(reason);
            return;
        };
        let risk = self.state.risk_read_up(end.id);
        let choice = match self.choose(end, &limit, place, source_hex, risk) {
            Ok(choice) => choice,
            Err((evidence, worked)) => {
                let reason = serving(end, plan)
                    .evidence(evidence, today)
                    .worked(worked)
                    .would_change("a transport force or a unit the side may use")
                    .done();
                self.target(in_scope).no_orders.push(reason);
                return;
            }
        };
        let choice_words = choice.lines.join("; ");
        // Item 10: at preserve the force, too few men is no landing.
        if let (Risk::Preserve, Some((wanted, chosen))) = (risk, choice.short) {
            let noticed = self.noticed(NoticeKind::CannotMeet, plan.id);
            let reason = serving(end, plan)
                .evidence(choice_words.clone(), today)
                .binding(format!(
                    "the ships can carry {chosen} men against the {wanted} wanted"
                ))
                .worked(format!(
                    "{choice_words}. At preserve the force the staff lands only with the men wanted: the ships can carry {chosen} men against the {wanted} wanted, so it forms nothing, asks no leave{}.",
                    if noticed {
                        ""
                    } else {
                        " and hands in the notice that it cannot meet the intent"
                    }
                ))
                .would_change("more ships, a smaller estimate, or a bolder risk level")
                .done();
            if noticed {
                self.target(in_scope).no_orders.push(reason);
            } else {
                self.notice(in_scope, reason, NoticeKind::CannotMeet, plan);
            }
            return;
        }
        let Some(force) = self
            .view
            .task_forces()
            .find(|f| f.id == choice.force)
            .cloned()
        else {
            return;
        };
        let units: Vec<LandUnitId> = choice.picks.iter().map(|p| p.id).collect();
        let supply = self.supply(&force, &units, source, source_hex, figures);
        let days = match self.days(&force, &units, &supply, place, source_hex, end) {
            Ok(days) => days,
            Err(why) => {
                let reason = serving(end, plan)
                    .evidence(why.clone(), today)
                    .worked(format!(
                        "{choice_words}. But {why}, so the staff forms nothing under {}.",
                        end.id
                    ))
                    .would_change("a route to the place")
                    .done();
                self.target(in_scope).no_orders.push(reason);
                return;
            }
        };
        // Item 13: the task's date.
        let task_date =
            by_day.map(|day| (day + 1).saturating_sub(days.embark + days.passage).max(1));
        let date_words = match (by_day, task_date) {
            (Some(day), Some(date)) => format!(
                " The place is to be the side's by the end of day {day}: the landing is to be ready on the morning of day {day} - {} - {} + 1 = {date}.",
                days.embark, days.passage
            ),
            _ => String::new(),
        };
        let kind = IntentKind::ReadyLanding {
            force: choice.force,
            hex: place,
            by_day: task_date,
        };
        let new_id = self.state.next_intent_number();
        let chosen_set: BTreeSet<LandUnitId> = units.iter().copied().collect();
        let new_limit = ForcesLimit {
            ships: BTreeSet::new(),
            task_forces: BTreeSet::new(),
            air_groups: BTreeSet::new(),
            source: None,
            land_units: chosen_set.clone(),
        };
        self.state.intents.insert(
            new_id,
            Intent {
                id: new_id,
                kind,
                parent: Some(end.id),
                maker: Maker::Staff,
                addressee: kind.addressee(),
                limits: sim_core::command::IntentLimits {
                    forces: Some(new_limit.clone()),
                    ..Default::default()
                },
                set_day: today,
                plan: end.plan,
                withdrawn_day: None,
            },
        );
        let unit_list = units
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" and ");
        let worked = format!(
            "{choice_words}; {}; {}.{date_words}",
            supply.words, days.words
        );
        let form_reason = serving(end, plan)
            .evidence(choice_words.clone(), today)
            .worked(format!(
                "{worked} The staff forms {new_id}, ready a landing by {} at {place}, under {}.",
                choice.force, end.id
            ))
            .would_change("other units or ships in the forces limit")
            .done();
        let limit_reason = serving(end, plan)
            .evidence(choice_words.clone(), today)
            .evidence(supply.words.clone(), today)
            .evidence(days.words.clone(), today)
            .worked(format!(
                "{worked} The staff sets the forces limit of {new_id} to {unit_list}."
            ))
            .would_change("the commander changing the limit")
            .done();
        let out = self.target(in_scope);
        out.commands.push(StaffCommand {
            kind: CommandKind::FormIntent {
                parent: end.id,
                kind,
            },
            reason: form_reason,
        });
        out.commands.push(StaffCommand {
            kind: CommandKind::SetLimit(Limit::Forces {
                intent: new_id,
                limit: new_limit,
            }),
            reason: limit_reason,
        });
        if in_scope {
            self.lines.push(format!(
                "It forms {new_id}, ready a landing by {} at {place}, with {unit_list}: {worked}",
                choice.force
            ));
        }
    }

    /// Items 6 to 10: the transport force and the units that land, or why
    /// there are none (evidence, working).
    #[allow(clippy::too_many_lines)]
    fn choose(
        &self,
        end: &Intent,
        limit: &ForcesLimit,
        place: Hex,
        source_hex: Hex,
        risk: Risk,
    ) -> Result<Choice, (String, String)> {
        let view = self.view;
        // Item 6: the transport force.
        let mut refused: Vec<String> = Vec::new();
        let mut transport: Option<TaskForce> = None;
        for id in &limit.task_forces {
            let Some(force) = view.task_forces().find(|f| f.id == *id) else {
                refused.push(format!("{id} is not one of the side's forces now"));
                continue;
            };
            if force.mission != Mission::Transport {
                refused.push(format!("{id} is not on the transport mission"));
                continue;
            }
            let carries = view.ships().any(|ship| {
                ship.location == ShipLocation::TaskForce(*id)
                    && view
                        .ship_class(ship.class)
                        .is_some_and(|c| c.troop_capacity_men > 0)
            });
            if !carries {
                refused.push(format!("{id} has no ship afloat that carries men"));
                continue;
            }
            if let Some(day) = crate::available::force_waiting(view, *id) {
                refused.push(crate::available::named_with_day(id, day));
                continue;
            }
            if let Some(hold) = self.state.hold_on_force(*id, &force.ships) {
                let words = self
                    .state
                    .hold_of(hold)
                    .map_or_else(|| format!("{hold} is held"), |h| hold_words(hold, *h));
                refused.push(format!("{id} is left out: {words}"));
                continue;
            }
            let at_source = crate::fleet::force_stopped_at(force, source_hex);
            let has_men = view.ships().any(|ship| {
                ship.location == ShipLocation::TaskForce(*id)
                    && ship
                        .troops
                        .is_some_and(|t| limit.land_units.contains(&t.unit))
            });
            if !(at_source || has_men) {
                refused.push(format!(
                    "{id} is not stopped at the source {source_hex} and has no men of the limit's units aboard"
                ));
                continue;
            }
            transport = Some(force.clone());
            break;
        }
        let Some(force) = transport else {
            let why = if refused.is_empty() {
                "the forces limit names no task force".to_string()
            } else {
                refused.join("; ")
            };
            return Err((
                format!("no transport force: {why}"),
                format!(
                    "The staff has no transport force for {}: {why}. It forms nothing and gives no order.",
                    end.id
                ),
            ));
        };
        let force_id = force.id;
        // The force's ships that carry men, in number order, with their
        // berths and the unit aboard each.
        let mut ships: Vec<(ShipId, u32, Option<LandUnitId>)> = view
            .ships()
            .filter(|ship| ship.location == ShipLocation::TaskForce(force_id))
            .filter_map(|ship| {
                let class = view.ship_class(ship.class)?;
                (class.troop_capacity_men > 0).then_some((
                    ship.id,
                    class.troop_capacity_men,
                    ship.troops.map(|t| t.unit),
                ))
            })
            .collect();
        ships.sort_by_key(|(id, _, _)| *id);
        // Item 7: the units that may land.
        let mut left: Vec<String> = Vec::new();
        let mut candidates: Vec<(LandUnit, bool)> = Vec::new();
        for id in &limit.land_units {
            let Some(unit) = view.land_units().find(|u| u.id == *id) else {
                left.push(format!("{id} is not one of the side's units now"));
                continue;
            };
            if unit.working {
                left.push(format!(
                    "{} is a working unit, which this step does not land",
                    unit_words(unit)
                ));
                continue;
            }
            if let Some(day) = crate::available::not_before(view, EntityRef::LandUnit(*id)) {
                left.push(crate::available::named_with_day(unit_words(unit), day));
                continue;
            }
            let hold = sim_core::command::HoldTarget::LandUnit(*id);
            if let Some(found) = self.state.hold_of(hold) {
                left.push(format!(
                    "{}; the staff plans without it",
                    hold_words(hold, *found)
                ));
                continue;
            }
            let aboard_total = view.men_aboard(*id).unwrap_or(0);
            let aboard_here: u32 = view
                .ships()
                .filter(|s| s.location == ShipLocation::TaskForce(force_id))
                .filter_map(|s| s.troops)
                .filter(|t| t.unit == *id)
                .map(|t| t.men)
                .sum();
            let ashore_at_source =
                unit.men_ashore > 0 && unit.hex == Some(source_hex) && aboard_total == 0;
            let all_aboard = unit.men_ashore == 0 && aboard_here > 0 && aboard_here == aboard_total;
            if !(ashore_at_source || all_aboard) {
                left.push(format!(
                    "{} is neither all ashore at the source {source_hex} nor all aboard {force_id}",
                    unit_words(unit)
                ));
                continue;
            }
            candidates.push((unit.clone(), all_aboard));
        }
        // Item 8: the order, by what each is good at.
        candidates.sort_by(|(a, _), (b, _)| {
            b.skills
                .assault
                .cmp(&a.skills.assault)
                .then(b.experience.cmp(&a.experience))
                .then(a.id.cmp(&b.id))
        });
        // Item 10: the men wanted.
        let estimate = view.ground_estimate(place).map(|e| e.men);
        let wanted = estimate.map(|men| men * men_per_man(risk));
        let mut lines: Vec<String> = Vec::new();
        lines.push(match (estimate, wanted) {
            (Some(est), Some(want)) => format!(
                "the side's estimate is {est} enemy men at {place}; at {} the staff wants {} men for each, {want} men",
                risk.name(),
                men_per_man(risk)
            ),
            _ => format!(
                "the side holds no estimate of the enemy's men at {place}: the staff takes every unit the ships can carry and goes with no count of the enemy"
            ),
        });
        let order_words = candidates
            .iter()
            .map(|(u, _)| {
                format!(
                    "{}: assault skill {}, experience {}, {} men",
                    unit_words(u),
                    u.skills.assault,
                    u.experience,
                    u.men_ashore + view.men_aboard(u.id).unwrap_or(0)
                )
            })
            .collect::<Vec<_>>()
            .join("; ");
        lines.push(format!(
            "the units in their order, the higher assault skill first, then the higher experience, then the lower number: {}",
            if order_words.is_empty() {
                "none".to_string()
            } else {
                order_words
            }
        ));
        // Item 9: the ships. A unit aboard keeps the ships it is in.
        let mut taken: BTreeSet<ShipId> = ships
            .iter()
            .filter(|(_, _, unit)| unit.is_some())
            .map(|(id, _, _)| *id)
            .collect();
        let mut picks: Vec<Pick> = Vec::new();
        let mut chosen_men = 0u32;
        let mut berths_used = 0u32;
        let mut ship_count = 0usize;
        for (unit, aboard) in &candidates {
            let men = unit.men_ashore + view.men_aboard(unit.id).unwrap_or(0);
            if let Some(want) = wanted {
                if !picks.is_empty() && chosen_men >= want {
                    left.push(format!(
                        "{} is not needed: the {chosen_men} men chosen are at least the {want} wanted",
                        unit_words(unit)
                    ));
                    continue;
                }
            }
            if *aboard {
                let own: Vec<ShipId> = ships
                    .iter()
                    .filter(|(_, _, u)| *u == Some(unit.id))
                    .map(|(id, _, _)| *id)
                    .collect();
                berths_used += ships
                    .iter()
                    .filter(|(id, _, _)| own.contains(id))
                    .map(|(_, b, _)| *b)
                    .sum::<u32>();
                ship_count += own.len();
                chosen_men += men;
                picks.push(Pick {
                    id: unit.id,
                    aboard: true,
                    ships: own,
                });
                continue;
            }
            let free: Vec<(ShipId, u32)> = ships
                .iter()
                .filter(|(id, _, _)| !taken.contains(id))
                .map(|(id, b, _)| (*id, *b))
                .collect();
            let mut mine = Vec::new();
            let mut berths = 0u32;
            for (id, b) in &free {
                if berths >= men {
                    break;
                }
                mine.push(*id);
                berths += *b;
            }
            if berths < men {
                let free_berths: u32 = free.iter().map(|(_, b)| *b).sum();
                left.push(if free.is_empty() {
                    format!(
                        "{} ({men} men) is left out: no ship is left for it",
                        unit_words(unit)
                    )
                } else {
                    format!(
                        "{} ({men} men) is left out: the ships left have {free_berths} berths, {} short",
                        unit_words(unit),
                        men - free_berths
                    )
                });
                continue;
            }
            for id in &mine {
                taken.insert(*id);
            }
            berths_used += berths;
            ship_count += mine.len();
            chosen_men += men;
            picks.push(Pick {
                id: unit.id,
                aboard: false,
                ships: mine,
            });
        }
        let pick_words = picks
            .iter()
            .map(|p| {
                let n = p.ships.len();
                format!(
                    "{} takes {n} {}{}",
                    p.id,
                    if n == 1 { "ship" } else { "ships" },
                    if p.aboard { ", the ships it is in" } else { "" }
                )
            })
            .collect::<Vec<_>>()
            .join("; ");
        lines.push(format!(
            "the berths: {chosen_men} men in {berths_used} berths in {ship_count} {} of {force_id}{}",
            if ship_count == 1 { "ship" } else { "ships" },
            if pick_words.is_empty() {
                String::new()
            } else {
                format!(" ({pick_words})")
            }
        ));
        if left.is_empty() {
            lines.push("no unit is left out".to_string());
        } else {
            lines.push(format!("left out: {}", left.join("; ")));
        }
        if picks.is_empty() {
            return Err((
                lines.join("; "),
                format!(
                    "{}. No unit can land, so the staff forms nothing and gives no order under {}.",
                    lines.join("; "),
                    end.id
                ),
            ));
        }
        let short = wanted.filter(|w| chosen_men < *w).map(|w| (w, chosen_men));
        if let (Some((want, have)), false) = (short, risk == Risk::Preserve) {
            lines.push(format!(
                "the ships carry {have} men, fewer than the {want} wanted: at {} the staff goes with what it has",
                risk.name()
            ));
        }
        // Units in number order for the orders that follow.
        picks.sort_by_key(|p| p.id);
        Ok(Choice {
            force: force_id,
            picks,
            lines,
            short,
        })
    }

    /// Item 16: the supply the force is to have aboard.
    fn supply(
        &self,
        force: &TaskForce,
        units: &[LandUnitId],
        source: BaseId,
        source_hex: Hex,
        figures: &sim_core::refdata::GroundFigures,
    ) -> Supply {
        let view = self.view;
        let mut men = 0u32;
        let mut charge = Fixed::ZERO;
        for id in units {
            let Some(unit) = view.land_units().find(|u| u.id == *id) else {
                continue;
            };
            let unit_men = unit.men_ashore + view.men_aboard(*id).unwrap_or(0);
            men += unit_men;
            let factor = Skills::factor(unit.skills.weight, figures.weight_strength);
            charge += figures.daily_supply_t_per_man * factor * i64::from(unit_men);
        }
        let wanted = charge * whole(DEFAULT_INTERVAL_DAYS);
        let force_ships: Vec<&sim_core::state::Ship> = view
            .ships()
            .filter(|s| s.location == ShipLocation::TaskForce(force.id))
            .collect();
        let aboard = force_ships
            .iter()
            .fold(Fixed::ZERO, |sum, s| sum + s.supply_t);
        let room = force_ships.iter().fold(Fixed::ZERO, |sum, s| {
            let capacity = view
                .ship_class(s.class)
                .map_or(Fixed::ZERO, |c| c.cargo_capacity_t);
            sum + (capacity - s.supply_t - s.cargo_fuel_t).max(Fixed::ZERO)
        });
        // The source's stock above its reserve, where the force lies at it.
        let at_source = crate::fleet::force_stopped_at(force, source_hex);
        let above = if at_source {
            view.bases()
                .find(|b| b.id == source)
                .map_or(Fixed::ZERO, |b| b.above_reserve(Cargo::Supply))
        } else {
            Fixed::ZERO
        };
        let level = wanted
            .min(aboard + above)
            .min(aboard + room)
            .max(aboard.min(wanted));
        let mut words = format!(
            "the supply is the daily charge of the {men} men chosen, {} a day, for {DEFAULT_INTERVAL_DAYS} days = {}",
            tons(charge),
            tons(wanted)
        );
        if !aboard.is_zero() {
            words.push_str(&format!(", of which {} is aboard", tons(aboard)));
        }
        if level < wanted {
            words.push_str(&format!(
                ", cut to {} by {}",
                tons(level),
                if !at_source {
                    "the force lying away from the source".to_string()
                } else if above <= room {
                    format!("the source's {} above its reserve", tons(above))
                } else {
                    format!("the ships' room of {}", tons(room))
                }
            ));
        }
        let take = (level - aboard).max(Fixed::ZERO);
        if take.is_zero() {
            words.push_str(": nothing to take");
        } else {
            words.push_str(&format!(": {} to take", tons(take)));
        }
        Supply {
            level,
            aboard,
            words,
        }
    }

    /// The route to the place and back to the source (item 19): waypoints,
    /// the miles out and back, or why there is none.
    fn route_for(
        &self,
        force: &TaskForce,
        place: Hex,
        source_hex: Hex,
        end: &Intent,
    ) -> Result<(Vec<Hex>, u32, u32), String> {
        let area = self.state.area_read_up(end.id);
        let from = force.position();
        let out = crate::route::route_between(self.view, from, place, area.as_ref())
            .ok_or_else(|| format!("ships cannot go from {from} to {place}"))?;
        let back = crate::route::route_between(self.view, place, source_hex, area.as_ref())
            .ok_or_else(|| format!("ships cannot go from {place} to {source_hex}"))?;
        if !(out.in_area && back.in_area) {
            return Err(format!(
                "no route from {from} to {place} and back to {source_hex} lies in the area"
            ));
        }
        let mut waypoints = out.out.clone();
        waypoints.extend(back.out.iter().copied());
        Ok((waypoints, out.out_nm, back.out_nm))
    }

    /// The slowest cruise speed of the force's ships afloat.
    fn speed_of(&self, force: &TaskForce) -> Option<Fixed> {
        self.view
            .ships()
            .filter(|s| s.location == ShipLocation::TaskForce(force.id))
            .filter_map(|s| self.view.ship_class(s.class))
            .map(|c| c.cruise_speed_kn)
            .min()
            .filter(|speed| *speed > Fixed::ZERO)
    }

    /// Item 18: the days to embark and of the passage.
    fn days(
        &self,
        force: &TaskForce,
        units: &[LandUnitId],
        supply: &Supply,
        place: Hex,
        source_hex: Hex,
        end: &Intent,
    ) -> Result<Days, String> {
        let ashore = units
            .iter()
            .filter(|id| {
                self.view
                    .land_units()
                    .any(|u| u.id == **id && u.men_ashore > 0)
            })
            .count() as u32;
        let for_supply = u32::from(!supply.to_take().is_zero());
        let embark = for_supply + ashore;
        let (_, out_nm, _) = self.route_for(force, place, source_hex, end)?;
        let speed = self
            .speed_of(force)
            .ok_or_else(|| format!("{} has no ship that can steam", force.id))?;
        let hours = whole(out_nm) / speed;
        let passage = ceil_days(hours).max(1);
        let words = format!(
            "the days: {embark} to embark ({for_supply} for the supply and 1 for each of the {ashore} {} ashore) and {passage} {} of passage ({out_nm} nm at {speed} kn = {hours} hours)",
            if ashore == 1 { "unit" } else { "units" },
            days_word(passage)
        );
        Ok(Days {
            embark,
            passage,
            words,
        })
    }

    // -- Leave, move and land -----------------------------------------------

    /// Items 15 to 24: for each "take a place" with its landing formed and
    /// its plan approved in the planned state, in priority order. A proposal
    /// shows the tree as it is formed and goes no further (item 28).
    pub(crate) fn takes_launch(&mut self) {
        if self.scope.is_some() {
            return;
        }
        let ends = self.in_priority_order(|i| matches!(i.kind, IntentKind::TakePlace { .. }));
        for id in ends {
            let Some(end) = self.state.intents.get(&id).cloned() else {
                continue;
            };
            let Some(plan) = self.plan_of(&end) else {
                continue;
            };
            if !matches!(plan.state, PlanState::Preparing | PlanState::Released) {
                continue;
            }
            let Some(task) = self.landing_of(&end) else {
                continue;
            };
            self.launch_place(&end, &plan, &task);
        }
    }

    #[allow(clippy::too_many_lines)]
    fn launch_place(&mut self, end: &Intent, plan: &Plan, task: &Intent) {
        let IntentKind::TakePlace { hex: place, .. } = end.kind else {
            return;
        };
        let IntentKind::ReadyLanding {
            force: force_id, ..
        } = task.kind
        else {
            return;
        };
        let in_scope = self.in_scope(end.id);
        let today = self.today;
        let view = self.view;
        let Some(figures) = view.ground_figures() else {
            return;
        };
        let Some(force) = view.task_forces().find(|f| f.id == force_id).cloned() else {
            let reason = serving(end, plan)
                .evidence(
                    format!("{force_id} is not one of the side's forces now"),
                    today,
                )
                .worked(format!(
                    "{force_id} is gone, so the staff gives no order under {}.",
                    end.id
                ))
                .done();
            self.target(in_scope).no_orders.push(reason);
            return;
        };
        let Some((source, source_hex)) = end
            .limits
            .forces
            .as_ref()
            .and_then(|f| f.source)
            .and_then(|s| view.bases().find(|b| b.id == s).map(|b| (s, b.hex)))
        else {
            return;
        };
        let units: Vec<LandUnitId> = task
            .limits
            .forces
            .as_ref()
            .map(|f| f.land_units.iter().copied().collect())
            .unwrap_or_default();
        let supply = self.supply(&force, &units, source, source_hex, figures);
        let mut now = plan.state;

        // Item 15: leave to launch, on a morning the landing is ready.
        if now == PlanState::Preparing {
            let Some(reading) = self.landings.get(&task.id).cloned() else {
                let reason = serving(end, plan)
                    .evidence(format!("{} gave no reading of its readiness", task.id), today)
                    .worked(format!(
                        "The landing of {} gave no reading this morning, so the staff gives no order under {}.",
                        task.id, end.id
                    ))
                    .done();
                self.target(in_scope).no_orders.push(reason);
                return;
            };
            if !reading.ready {
                let expect = match reading.ready_on {
                    Some(day) => {
                        format!("it expects the landing to be ready on the morning of day {day}")
                    }
                    None => "it cannot see a morning the landing would be ready".to_string(),
                };
                let reason = serving(end, plan)
                    .evidence(reading.words.clone(), today)
                    .worked(format!(
                        "The landing at {place} is not ready: {}; {expect}. The staff gives no order under {} before then.",
                        reading.words, end.id
                    ))
                    .would_change("the landing ready")
                    .done();
                self.target(in_scope).no_orders.push(reason);
                return;
            }
            if !self.state.leaves.launch {
                let days = self.days(&force, &units, &supply, place, source_hex, end);
                let when = match days {
                    Ok(d) => {
                        let lands = if d.passage == 1 {
                            "lands that day".to_string()
                        } else {
                            format!(
                                "lands {} {} after that",
                                d.passage - 1,
                                days_word(d.passage - 1)
                            )
                        };
                        format!(
                            " With leave on a morning it sails {} {} after ({}) and {lands}.",
                            d.embark,
                            days_word(d.embark),
                            d.words
                        )
                    }
                    Err(_) => String::new(),
                };
                let reason = serving(end, plan)
                    .evidence(reading.words.clone(), today)
                    .binding("leave to launch is withheld")
                    .worked(format!(
                        "The landing at {place} is ready ({}) and the commander has not given leave to launch, so the plan waits for him.{when}",
                        reading.words
                    ))
                    .would_change("leave to launch given, or the commander releasing the plan")
                    .done();
                self.waits(
                    in_scope,
                    reason,
                    NoticeKind::WaitsForLeave(Leave::Launch),
                    plan.id,
                );
                return;
            }
            let reason = serving(end, plan)
                .evidence(reading.words.clone(), today)
                .evidence("leave to launch is given", today)
                .worked(format!(
                    "The landing at {place} is ready ({}) and leave to launch is given: the staff releases {}.",
                    reading.words, plan.id
                ))
                .would_change("leave to launch withheld")
                .done();
            self.target(in_scope).commands.push(StaffCommand {
                kind: CommandKind::Approve { plan: plan.id },
                reason,
            });
            self.set_plan(plan.id, PlanState::Released);
            now = PlanState::Released;
        }
        if now != PlanState::Released {
            return;
        }

        // Items 22 and 24: the land phase.
        let ashore_here: Vec<&LandUnit> = view
            .land_units()
            .filter(|u| u.men_ashore > 0 && u.hex == Some(place))
            .collect();
        if !ashore_here.is_empty() {
            let mut orders = Vec::new();
            for unit in &ashore_here {
                if !units.contains(&unit.id) || unit.posture == Posture::Attack {
                    continue;
                }
                let hold = sim_core::command::HoldTarget::LandUnit(unit.id);
                if self.state.hold_of(hold).is_some() {
                    continue;
                }
                let reason = serving(end, plan)
                    .evidence(
                        format!("{} has {} men ashore at {place}", unit_words(unit), unit.men_ashore),
                        today,
                    )
                    .worked(format!(
                        "{} could not be set to attack before it sailed, for it was aboard already; its men are ashore at {place}, so the staff sets it to attack.",
                        unit_words(unit)
                    ))
                    .would_change("the place the side's")
                    .done();
                if in_scope {
                    self.lines.push(format!("{} is set to attack.", unit.id));
                }
                orders.push(StaffOrder {
                    kind: OrderKind::LandUnit {
                        target: unit.id,
                        order: LandUnitOrder::SetPosture {
                            posture: Posture::Attack,
                            when_spent: None,
                        },
                    },
                    intent: end.id,
                    reason,
                });
            }
            if orders.is_empty() {
                let men: u32 = ashore_here.iter().map(|u| u.men_ashore).sum();
                let side = view.side();
                let enemy: Vec<String> = view
                    .enemy_bases()
                    .filter(|b| b.hex == place && b.side != side)
                    .map(|b| format!("{} ({})", b.id, b.name))
                    .collect();
                let holds = if enemy.is_empty() {
                    "the enemy holds no base there".to_string()
                } else {
                    format!("the enemy holds {} there", enemy.join(" and "))
                };
                let reason = serving(end, plan)
                    .evidence(format!("{men} men of the side are ashore at {place}"), today)
                    .evidence(holds.clone(), today)
                    .worked(format!(
                        "The side has {men} men ashore at {place} and {holds}: the place is not yet the side's, and what the men do ashore is step 4, so the staff gives no order."
                    ))
                    .would_change("the place the side's")
                    .done();
                self.target(in_scope).no_orders.push(reason);
            }
            self.target(in_scope).orders.extend(orders);
            return;
        }
        if routed(&force, place) {
            let reason = serving(end, plan)
                .evidence(format!("{force_id}'s route acts at {place}"), today)
                .worked(format!(
                    "{force_id} is routed to {place} and its orders stand: the staff gives no order."
                ))
                .would_change("the force's orders changed")
                .done();
            self.target(in_scope).no_orders.push(reason);
            return;
        }
        // Item 26: a held force is given nothing.
        if let Some(hold) = self.state.hold_on_force(force_id, &force.ships) {
            let words = self
                .state
                .hold_of(hold)
                .map_or_else(|| format!("{hold} is held"), |h| hold_words(hold, *h));
            let reason = serving(end, plan)
                .evidence(words.clone(), today)
                .binding(words.clone())
                .worked(format!(
                    "{words}, so the staff gives {force_id} no order under {}.",
                    end.id
                ))
                .would_change("the hold released")
                .done();
            self.target(in_scope).no_orders.push(reason);
            return;
        }
        let cargo_order = force.cargo_order.as_ref();

        // Item 16: the supply, first.
        let take = supply.to_take();
        if !take.is_zero() {
            let loading = cargo_order.is_some_and(|c| {
                c.action == CargoAction::Load
                    && c.unit.is_none()
                    && c.weight_t == Some(supply.level)
            });
            if loading {
                let reason = serving(end, plan)
                    .evidence(supply.words.clone(), today)
                    .worked(format!(
                        "{force_id} is loading its supply ({}); the order stands, so the staff gives no order.",
                        supply.words
                    ))
                    .would_change("the load complete")
                    .done();
                self.target(in_scope).no_orders.push(reason);
                return;
            }
            let reason = serving(end, plan)
                .evidence(supply.words.clone(), today)
                .worked(format!(
                    "{} is released: {}. The staff orders {force_id} to load supply at the source until it has {} aboard.",
                    plan.id,
                    supply.words,
                    tons(supply.level)
                ))
                .would_change("the source's stock, or other units chosen")
                .done();
            if in_scope {
                self.lines.push(format!(
                    "{force_id} loads supply to {}.",
                    tons(supply.level)
                ));
            }
            self.target(in_scope).orders.push(StaffOrder {
                kind: OrderKind::TaskForce {
                    target: force_id,
                    order: TaskForceOrder::LoadTo {
                        cargo: Cargo::Supply,
                        weight_t: supply.level,
                    },
                },
                intent: end.id,
                reason,
            });
            return;
        }

        // Item 17: one unit ashore at the source a morning, in number order.
        let mut ashore: Vec<&LandUnit> = units
            .iter()
            .filter_map(|id| view.land_units().find(|u| u.id == *id))
            .filter(|u| u.men_ashore > 0)
            .collect();
        ashore.sort_by_key(|u| u.id);
        if let Some(unit) = ashore.first() {
            let unit = (*unit).clone();
            let hold = sim_core::command::HoldTarget::LandUnit(unit.id);
            if let Some(found) = self.state.hold_of(hold) {
                let words = hold_words(hold, *found);
                let reason = serving(end, plan)
                    .evidence(words.clone(), today)
                    .binding(words.clone())
                    .worked(format!(
                        "{} is to go aboard next, but {words}: the staff gives no order and waits.",
                        unit_words(&unit)
                    ))
                    .would_change("the hold released")
                    .done();
                self.target(in_scope).no_orders.push(reason);
                return;
            }
            if unit.hex != Some(source_hex) || !crate::fleet::force_stopped_at(&force, source_hex) {
                let reason = serving(end, plan)
                    .evidence(
                        format!("{} has men ashore away from {force_id}", unit_words(&unit)),
                        today,
                    )
                    .worked(format!(
                        "{} has men ashore that {force_id} cannot take aboard where it lies, so the staff gives no order.",
                        unit_words(&unit)
                    ))
                    .would_change("the force and the unit at the source")
                    .done();
                self.target(in_scope).no_orders.push(reason);
                return;
            }
            let loading = cargo_order
                .is_some_and(|c| c.action == CargoAction::LoadNoCargo && c.unit == Some(unit.id));
            if loading {
                let reason = serving(end, plan)
                    .evidence(
                        format!("{} has {} men ashore", unit_words(&unit), unit.men_ashore),
                        today,
                    )
                    .worked(format!(
                        "{} is going aboard {force_id}; the order stands, so the staff gives no order.",
                        unit_words(&unit)
                    ))
                    .would_change("the unit aboard")
                    .done();
                self.target(in_scope).no_orders.push(reason);
                return;
            }
            let words = format!(
                "{} has {} men ashore at the source {source}: it is the next unit to go aboard, in number order",
                unit_words(&unit),
                unit.men_ashore
            );
            if unit.posture != Posture::Attack {
                let reason = serving(end, plan)
                    .evidence(words.clone(), today)
                    .worked(format!(
                        "{words}. It is set to attack the morning it goes aboard, so that it attacks when it lands."
                    ))
                    .would_change("another unit next")
                    .done();
                self.target(in_scope).orders.push(StaffOrder {
                    kind: OrderKind::LandUnit {
                        target: unit.id,
                        order: LandUnitOrder::SetPosture {
                            posture: Posture::Attack,
                            when_spent: None,
                        },
                    },
                    intent: end.id,
                    reason,
                });
            }
            let reason = serving(end, plan)
                .evidence(words.clone(), today)
                .worked(format!(
                    "{words}. The staff orders {force_id} to take it aboard, with no cargo."
                ))
                .would_change("another unit next")
                .done();
            if in_scope {
                self.lines.push(format!(
                    "{} is set to attack and goes aboard {force_id}.",
                    unit.id
                ));
            }
            self.target(in_scope).orders.push(StaffOrder {
                kind: OrderKind::TaskForce {
                    target: force_id,
                    order: TaskForceOrder::LoadUnit {
                        unit: unit.id,
                        cargo: None,
                    },
                },
                intent: end.id,
                reason,
            });
            return;
        }

        // Item 19: the passage.
        let (waypoints, out_nm, back_nm) = match self.route_for(&force, place, source_hex, end) {
            Ok(found) => found,
            Err(why) => {
                let reason = serving(end, plan)
                    .evidence(why.clone(), today)
                    .binding(why.clone())
                    .worked(format!(
                        "Every unit is aboard and the supply is aboard, but {why}: the force is not routed."
                    ))
                    .would_change("a route to the place")
                    .done();
                self.target(in_scope).no_orders.push(reason);
                return;
            }
        };
        let Some(speed) = self.speed_of(&force) else {
            return;
        };
        // The fuel for the round trip, as for a run (decision 0107).
        let route_nm = whole(out_nm + back_nm);
        let mut short: Vec<String> = Vec::new();
        let mut fuel_lines: Vec<String> = Vec::new();
        for ship in view
            .ships()
            .filter(|s| s.location == ShipLocation::TaskForce(force_id))
        {
            let Some(class) = view.ship_class(ship.class) else {
                continue;
            };
            let need = route_nm.mul_div(class.fuel_capacity_t, class.endurance_nm);
            fuel_lines.push(format!(
                "{} has {} against {} needed",
                ship.id,
                tons(ship.fuel_t),
                tons(need)
            ));
            if ship.fuel_t < need {
                short.push(format!(
                    "{} has {} of fuel against {} for the passage out and back",
                    ship.id,
                    tons(ship.fuel_t),
                    tons(need)
                ));
            }
        }
        if !short.is_empty() {
            let why = short.join("; ");
            let reason = serving(end, plan)
                .evidence(why.clone(), today)
                .binding(why.clone())
                .worked(format!(
                    "Every unit is aboard and the supply is aboard, but {why}: the force is not routed."
                ))
                .would_change("the ships fuelled")
                .done();
            self.target(in_scope).no_orders.push(reason);
            return;
        }
        let empty: BTreeSet<TaskForceId> = BTreeSet::new();
        let task_forces = end
            .limits
            .forces
            .as_ref()
            .map_or(&empty, |f| &f.task_forces);
        let ctx = crate::escort::RunContext {
            view,
            state: &self.state,
            intent: end,
            plan,
            task_forces,
            source,
            source_hex,
            destination_hex: place,
            route_nm,
        };
        let (escort_words, escort) = match crate::escort::for_run(
            &ctx, force_id, &waypoints, speed, today,
        ) {
            crate::escort::ForRun::Held(why) => {
                let reason = serving(end, plan)
                        .evidence(why.clone(), today)
                        .binding(why.clone())
                        .worked(format!(
                            "Every unit is aboard and the supply is aboard, but {why}: the force does not sail today."
                        ))
                        .would_change("an escort free, or no threat")
                        .done();
                self.target(in_scope).no_orders.push(reason);
                return;
            }
            crate::escort::ForRun::Sails { words, escort } => (words, escort),
        };
        let hours_out = whole(out_nm) / speed;
        let sail_hour = Fixed::from_int(24) * whole(today - 1);
        let worked = format!(
            "Every unit chosen is aboard {force_id} and its supply is aboard ({}). It sails today (day {today}, hour {sail_hour}): {out_nm} nm out at {speed} kn = {hours_out} hours, so it arrives at {place} at hour {}; {back_nm} nm back to the source {source}. Fuel: {}.{}",
            supply.words,
            sail_hour + hours_out,
            fuel_lines.join(", "),
            escort_words.map_or_else(String::new, |w| format!(" {w}"))
        );
        if let Some((escort_id, escort_reason)) = escort {
            if in_scope {
                self.lines.push(format!("{escort_id} escorts {force_id}."));
            }
            self.target(in_scope).orders.push(StaffOrder {
                kind: OrderKind::TaskForce {
                    target: escort_id,
                    order: TaskForceOrder::Escort(Some(force_id)),
                },
                intent: end.id,
                reason: escort_reason,
            });
        }
        let reason = |extra: &str| {
            serving(end, plan)
                .evidence(supply.words.clone(), today)
                .evidence(format!("{} is released", plan.id), today)
                .worked(format!("{worked} {extra}"))
                .would_change("a unit not aboard, or a plan not released")
                .done()
        };
        let act_at: BTreeSet<Hex> = BTreeSet::from([place]);
        if in_scope {
            self.lines.push(format!(
                "{force_id} sails for {place} and lands what it carries."
            ));
        }
        let route_reason = reason(
            "This is the route order: out to the place and back to the source, acting at the place.",
        );
        let unload_reason = reason("This is the unload order the route acts on: the landing.");
        let out = self.target(in_scope);
        out.orders.push(StaffOrder {
            kind: OrderKind::TaskForce {
                target: force_id,
                order: TaskForceOrder::SetRoute {
                    waypoints,
                    act_at,
                    stand_at: BTreeSet::new(),
                },
            },
            intent: end.id,
            reason: route_reason,
        });
        out.orders.push(StaffOrder {
            kind: OrderKind::TaskForce {
                target: force_id,
                order: TaskForceOrder::Unload,
            },
            intent: end.id,
            reason: unload_reason,
        });
    }
}
