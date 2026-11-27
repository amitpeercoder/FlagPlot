//! Bombarding a base from the sea (decision 0146, step 5 of stage 3,
//! second part): the task "bombard a base", given to a task force on the
//! bombardment mission, and the doctrine that serves it at the force's level.
//!
//! **Forming** (item 2). Under an end whose plan is released, for each task
//! force on the bombardment mission with a ship afloat that the nearest
//! forces limit on the end's chain names, in task force order, where no
//! bombard task for that force is in force anywhere on the side: under
//! "neutralise a base", at that base; under "take a place", at the enemy's
//! lowest-numbered base on the side's map in the place's hex, and none where
//! there is none.
//!
//! **The force's morning** (items 4 to 11) is the stand doctrine's morning
//! (decision 0124, 9 to 15, in `crate::stand`) with these differences: the
//! target, which ends the sorties when it is no longer the enemy's on the
//! side's map; whether it sorties (every night as a root or under
//! neutralise, at preserve the force only on a night with no contact of
//! more ships than the force has afloat within 15 hexes of the target;
//! under "take a place" on the morning of the day the landing force is due
//! at the place and on no other); and the route, which marks the target's
//! hex to act at, so that the force halts there, fires in the night's
//! bombardment phase and steams on at the morning by the rules as they are.

use sim_core::command::LimitKind;
use std::collections::BTreeSet;

use sim_core::command::{
    CommandKind, Intent, IntentKind, IntentLimits, Maker, NoticeBasis, NoticeKind, Plan, Reason,
    Risk, StaffCommand, StaffNotice, StaffOrder,
};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, TaskForceId};
use sim_core::orders::{Mission, OrderKind, Readiness, SpeedSetting, Stance, TaskForceOrder};
use sim_core::state::{RulesOfEngagement, TaskForceStatus};
use sim_core::view::SideView;

use crate::air::Day;
use crate::reasons::{base_words, serving};
use crate::stand::{
    afloat, break_off, expected_words, fuel_needs, fuelling_hours, home_of, hours_to, margin,
    needs_words, silence_order, stance_words, stopped, usable_day, ShipFuel, NIGHT_HOURS,
    THREAT_HEXES,
};
use crate::units::{ceil_days, whole};

/// What one morning's working of a bombard task hands in.
struct Out {
    orders: Vec<(TaskForceOrder, Reason)>,
    summary: Option<Reason>,
    cannot: bool,
}

/// Each ship's rounds aboard against the level its risk keeps (decision
/// 0127): "ship:1 has 800 rounds against the level of 400".
fn rounds_words(view: &SideView<'_>, force: TaskForceId, risk: Risk) -> String {
    let Some(work) = view.ammunition_work(force) else {
        return "the force's rounds are not known".to_string();
    };
    let lines: Vec<String> = work
        .ships
        .iter()
        .map(|ship| match ship.outfit {
            Some(outfit) => format!(
                "{} has {} rounds against the level of {} ({} of her outfit of {outfit})",
                ship.ship,
                ship.rounds_aboard,
                crate::ammunition::rounds_share(risk) * Fixed::from_int(i64::from(outfit)),
                crate::ammunition::rounds_share(risk)
            ),
            None => format!("{} carries ammunition that is not limiting", ship.ship),
        })
        .collect();
    if lines.is_empty() {
        "the force has no ship whose rounds are counted".to_string()
    } else {
        lines.join(", ")
    }
}

/// Whether the morning's orders already handed in route `force` to act at
/// `place` for `end`.
fn routed_this_morning(
    decisions: &sim_core::command::StaffDecisions,
    end: &Intent,
    force: TaskForceId,
    place: Hex,
) -> bool {
    decisions.orders.iter().any(|o| {
        o.intent == end.id
            && matches!(
                &o.kind,
                OrderKind::TaskForce {
                    target,
                    order: TaskForceOrder::SetRoute { act_at, .. },
                } if *target == force && act_at.contains(&place)
            )
    })
}

impl Day<'_, '_> {
    /// Item 2: under `end`, a bombard task at `target` for each named force
    /// on the bombardment mission with a ship afloat, in task force order,
    /// where no bombard task for that force is in force anywhere on the side.
    fn form_bombard_tasks(&mut self, end: &Intent, plan: &Plan, target: BaseId, in_scope: bool) {
        let named: BTreeSet<TaskForceId> = self
            .state
            .nearest_forces(end.id)
            .map(|f| f.task_forces.clone())
            .unwrap_or_default();
        let today = self.today;
        let target_text = base_words(self.view, target);
        for id in named {
            let Some(force) = self.area.task_forces().find(|f| f.id == id) else {
                continue;
            };
            if force.mission != Mission::Bombardment {
                continue;
            }
            // Decision 0232, rule 10: under an island the bombardment forces
            // are the island's, and a force the commander named for a landing
            // he wrote is not this landing's.
            if crate::assemble::written_take(&self.state, end) {
                self.not_this_landings(
                    end,
                    plan,
                    id,
                    "bombardment",
                    "bombardment forces",
                    in_scope,
                );
                continue;
            }
            // A force before its day is no force a task may use (decision
            // 0140, 3).
            if crate::available::force_waiting(self.area, id).is_some() {
                continue;
            }
            let ships = afloat(self.area, id);
            if ships.is_empty() {
                continue;
            }
            let held = crate::reasons::force_held_words(self.area, id);
            let why = format!(
                "{held} is on the bombardment mission with {} ship(s) afloat",
                ships.len()
            );
            let kind = IntentKind::Bombard { force: id, target };
            let standing = self
                .state
                .intents_in_force()
                .find(|other| other.kind.force() == Some(id) && other.kind.code() == kind.code())
                .map(|other| (other.id, other.kind, other.parent));
            if let Some((task, task_kind, parent)) = standing {
                let aim = crate::reasons::aim(
                    self.view,
                    &Intent {
                        kind: task_kind,
                        ..end.clone()
                    },
                );
                let place = match parent {
                    Some(p) => format!("under {p}"),
                    None => "as a root".to_string(),
                };
                let reason = serving(end, plan)
                    .evidence(why.clone(), today)
                    .evidence(format!("{task}, a bombard task of {id}, is in force {place}"), today)
                    .binding(format!("{task} stands"))
                    .worked(format!(
                        "{task} ({aim}) stands {place}: a force has one bombard task, so the staff forms no other for {id}. {why}."
                    ))
                    .would_change(format!("{task} withdrawn"))
                    .done();
                self.target(in_scope).no_orders.push(reason);
                continue;
            }
            let id_new = self.state.next_intent_number();
            self.state.intents.insert(
                id_new,
                Intent {
                    id: id_new,
                    kind,
                    parent: Some(end.id),
                    maker: Maker::Staff,
                    addressee: kind.addressee(),
                    limits: IntentLimits::default(),
                    set_day: today,
                    plan: end.plan,
                    withdrawn_day: None,
                },
            );
            let reason = serving(end, plan)
                .evidence(why.clone(), today)
                .binding(format!("{id} is named in the forces limit of {}", end.id))
                .worked(format!(
                    "{id} is named in the forces limit of {}, its plan is released and no bombard task of {id} is in force: {why}, so the staff forms {id_new}, to bombard {target_text} with {id}, under it, for {id} to carry out.",
                    end.id
                ))
                .would_change(format!("{id} not named in the forces limit"))
                .would_change("such a task already in force")
                .speaks_of(&self.state, LimitKind::Forces)
                .done();
            self.target(in_scope).commands.push(StaffCommand {
                kind: CommandKind::FormIntent {
                    parent: end.id,
                    kind,
                },
                reason,
            });
            if in_scope {
                self.lines.push(format!(
                    "{} would form {id_new}, to bombard {target_text} with {id}, under it ({why}).",
                    end.id
                ));
            }
        }
    }

    /// Decision 0151, 8c: the first neutralising in force under `island`, in
    /// the island's order, of an enemy base at a named place: for each place
    /// in turn, the neutralising of the lowest-numbered enemy base on the
    /// side's map in its hex.
    fn first_island_neutralise(&self, island: &Intent) -> Option<IntentId> {
        let IntentKind::ControlIsland { island: number, .. } = island.kind else {
            return None;
        };
        let side = self.view.side();
        let places = self.view.island(number)?;
        let mut under: Vec<(BaseId, IntentId)> = self
            .state
            .intents_in_force()
            .filter(|i| i.parent == Some(island.id))
            .filter_map(|i| match i.kind {
                IntentKind::Neutralise { target } => Some((target, i.id)),
                _ => None,
            })
            .collect();
        under.sort();
        places.places.iter().find_map(|place| {
            under.iter().find_map(|(target, id)| {
                self.view
                    .enemy_bases()
                    .any(|b| b.id == *target && b.side != side && b.hex == place.hex)
                    .then_some(*id)
            })
        })
    }

    /// Decision 0151, 8c: under the island's first neutralising of an enemy
    /// base at a named place, a bombard task for each task force on the
    /// bombardment mission with a ship afloat that the staff may take
    /// (decision 0149, 5; inside the island's forces limit where it names
    /// task forces) and that lies stopped at a base of the side, in task
    /// force order, where no bombard task for that force is in force
    /// anywhere on the side. One the island's own tree formed stands
    /// without a word.
    #[allow(clippy::too_many_lines)]
    fn form_island_bombard_tasks(
        &mut self,
        island: &Intent,
        end: &Intent,
        plan: &Plan,
        target: BaseId,
        in_scope: bool,
    ) {
        let today = self.today;
        let target_text = base_words(self.view, target);
        let island_text = match island.kind {
            IntentKind::ControlIsland { island: n, .. } => self.view.island(n).map_or_else(
                || format!("island {n}"),
                |i| format!("island {n} ({})", i.name),
            ),
            _ => return,
        };
        let others = self.others_above(island);
        let named: Option<BTreeSet<TaskForceId>> = island
            .limits
            .forces
            .as_ref()
            .filter(|f| !f.task_forces.is_empty())
            .map(|f| f.task_forces.clone());
        let mut forces: Vec<_> = self
            .area
            .task_forces()
            .filter(|f| f.mission == Mission::Bombardment)
            .cloned()
            .collect();
        forces.sort_by_key(|f| f.id);
        for force in forces {
            let id = force.id;
            if named.as_ref().is_some_and(|n| !n.contains(&id)) {
                continue;
            }
            let ships = afloat(self.area, id);
            if ships.is_empty() {
                continue;
            }
            if let Err(why) = self.may_take_force(&force, &others) {
                let reason = serving(end, plan)
                    .evidence(why.clone(), today)
                    .worked(format!(
                        "{}: {why}, so the staff forms no bombard task for it under {}.",
                        end.id, end.id
                    ))
                    .would_change("the force free to take")
                    .done();
                self.target(in_scope).no_orders.push(reason);
                continue;
            }
            let Some(at) = self
                .view
                .bases()
                .filter(|b| crate::fleet::force_stopped_at(&force, b.hex))
                .map(|b| b.id)
                .min()
            else {
                continue;
            };
            let kind = IntentKind::Bombard { force: id, target };
            let standing = self
                .state
                .intents_in_force()
                .find(|other| other.kind.force() == Some(id) && other.kind.code() == kind.code())
                .map(|other| (other.id, other.kind, other.parent));
            if let Some((task, task_kind, parent)) = standing {
                if self.state.root_of(task) == island.id {
                    continue;
                }
                let aim = crate::reasons::aim(
                    self.view,
                    &Intent {
                        kind: task_kind,
                        ..end.clone()
                    },
                );
                let place = match parent {
                    Some(p) => format!("under {p}"),
                    None => "as a root".to_string(),
                };
                let reason = serving(end, plan)
                    .evidence(format!("{task}, a bombard task of {id}, is in force {place}"), today)
                    .binding(format!("{task} stands"))
                    .worked(format!(
                        "{task} ({aim}) stands {place}: a force has one bombard task, so the staff forms no other for {id}."
                    ))
                    .would_change(format!("{task} withdrawn"))
                    .done();
                self.target(in_scope).no_orders.push(reason);
                continue;
            }
            let held = crate::reasons::force_held_words(self.area, id);
            let why = format!(
                "{held} is on the bombardment mission with {} ship(s) afloat and lies stopped at {}",
                ships.len(),
                base_words(self.view, at)
            );
            let id_new = self.state.next_intent_number();
            self.state.intents.insert(
                id_new,
                Intent {
                    id: id_new,
                    kind,
                    parent: Some(end.id),
                    maker: Maker::Staff,
                    addressee: kind.addressee(),
                    limits: IntentLimits::default(),
                    set_day: today,
                    plan: end.plan,
                    withdrawn_day: None,
                },
            );
            let reason = serving(end, plan)
                .evidence(why.clone(), today)
                .binding(format!(
                    "the bombardment forces are {}'s, under its first neutralising of an enemy base at a named place",
                    island.id
                ))
                .worked(format!(
                    "{} neutralises {target_text}, the enemy's base at a named place of {island_text}, its plan is released and no bombard task of {id} is in force: {why}. The staff forms {id_new}, to bombard {target_text} with {id}, under {}, for {id} to carry out.",
                    end.id, end.id
                ))
                .would_change(format!("{id} taken by an intent higher in priority"))
                .would_change("such a task already in force")
                .speaks_of(&self.state, LimitKind::Priority)
                .done();
            self.target(in_scope).commands.push(StaffCommand {
                kind: CommandKind::FormIntent {
                    parent: end.id,
                    kind,
                },
                reason,
            });
            if in_scope {
                self.lines.push(format!(
                    "{} would form {id_new}, to bombard {target_text} with {id}, under it ({why}).",
                    end.id
                ));
            }
        }
    }

    /// Decision 0146: the forming under each released "neutralise a base"
    /// and "take a place", then each bombard task's morning, in priority
    /// order.
    pub(crate) fn bombards(&mut self) {
        let side = self.view.side();
        let ends = self.in_priority_order(|i| {
            matches!(
                i.kind,
                IntentKind::Neutralise { .. } | IntentKind::TakePlace { .. }
            )
        });
        for id in ends {
            let Some(end) = self.state.intents.get(&id).cloned() else {
                continue;
            };
            if !self.released(&end) {
                continue;
            }
            let Some(plan) = self.plan_of(&end) else {
                continue;
            };
            let target = match end.kind {
                IntentKind::Neutralise { target } => Some(target),
                IntentKind::TakePlace { hex, .. } => self
                    .view
                    .enemy_bases()
                    .filter(|b| b.hex == hex && b.side != side)
                    .map(|b| b.id)
                    .min(),
                _ => None,
            };
            let Some(target) = target else {
                continue;
            };
            let in_scope = self.in_scope(id);
            // Under "control an island" the bombardment forces are the
            // island's (decision 0151, 8c): a neutralising formed under an
            // island takes them by that rule and by no other.
            let island = end
                .parent
                .and_then(|p| self.state.intents.get(&p).cloned())
                .filter(|p| matches!(p.kind, IntentKind::ControlIsland { .. }));
            if let (Some(island), IntentKind::Neutralise { .. }) = (island, end.kind) {
                if self.first_island_neutralise(&island) == Some(end.id) {
                    self.form_island_bombard_tasks(&island, &end, &plan, target, in_scope);
                }
                continue;
            }
            self.form_bombard_tasks(&end, &plan, target, in_scope);
        }
        let tasks = self.in_priority_order(|i| matches!(i.kind, IntentKind::Bombard { .. }));
        for id in tasks {
            let Some(task) = self.state.intents.get(&id).cloned() else {
                continue;
            };
            // Released this morning, or released at the staff's hour and
            // ended by the staff this morning (a "take a place" met, whose
            // force is then set back to normal readiness, item 4).
            let released_at_hour = self
                .view
                .command_state()
                .plans
                .get(&task.plan)
                .is_some_and(|p| p.state == sim_core::command::PlanState::Released);
            if !self.released(&task) && !released_at_hour {
                continue;
            }
            self.serve_bombard(&task);
        }
    }

    /// Why the landing force is not on the map: it is held apart until its
    /// day (decision 0195, 13), or it is not one of the side's forces.
    fn not_on_the_map(&self, force: TaskForceId) -> String {
        match crate::available::entering_on(self.view, sim_core::ids::EntityRef::TaskForce(force)) {
            Some(day) => crate::available::enters_words(force, day),
            None => format!("{force} is not one of the side's forces"),
        }
    }

    /// Item 5, under "take a place": the day the landing force is due at the
    /// place (decision 0142, 18), or why it is not known. On the morning it
    /// is routed it is that day plus the days of the passage at the cruise
    /// speed of its slowest class less one; while it is under way to the
    /// place, the day the engine's forecast brings it there. Before it is
    /// routed, and once it has come, there is none.
    fn landing_due(&self, end: &Intent) -> Result<u32, String> {
        let IntentKind::TakePlace { hex: place, .. } = end.kind else {
            return Err("the end is not to take a place".to_string());
        };
        let Some(landing) = self.landing_of(end) else {
            return Err(format!(
                "{} has no landing formed under it yet, so the day the landing force is due at {place} is not known",
                end.id
            ));
        };
        let Some(force_id) = landing.kind.force() else {
            return Err("the landing names no force".to_string());
        };
        if routed_this_morning(&self.real, end, force_id, place)
            || routed_this_morning(&self.scratch, end, force_id, place)
        {
            let Some(force) = self.view.task_forces().find(|f| f.id == force_id) else {
                return Err(self.not_on_the_map(force_id));
            };
            let speed = self
                .speed_of(force)
                .ok_or_else(|| format!("{force_id} has no ship that can steam"))?;
            let area = self.state.area_read_up(end.id);
            let out =
                crate::route::route_between(self.view, force.position(), place, area.as_ref())
                    .ok_or_else(|| {
                        format!("ships cannot go from {} to {place}", force.position())
                    })?;
            let passage = ceil_days(whole(out.out_nm) / speed).max(1);
            return Ok(self.today + passage - 1);
        }
        let Some(force) = self.view.task_forces().find(|f| f.id == force_id) else {
            return Err(self.not_on_the_map(force_id));
        };
        let on_way = force.status == TaskForceStatus::Underway
            && force.route.act_at.contains(&place)
            && force.position() != place;
        if on_way {
            if let Some(sim_core::PassageEnd::Arrives { day, .. }) = self
                .view
                .forecast_passage(force_id, place, None)
                .map(|f| f.end)
            {
                return Ok(day);
            }
        }
        if force.landing_at == Some(place) || force.position() == place {
            return Err(format!(
                "the landing force {force_id} has come to {place}: the day it was due is past"
            ));
        }
        Err(format!(
            "the landing force {force_id} is not yet routed to {place}, so the day it is due there is not known"
        ))
    }

    #[allow(clippy::too_many_lines)]
    fn serve_bombard(&mut self, task: &Intent) {
        let Some(plan) = self.plan_of(task) else {
            return;
        };
        let IntentKind::Bombard { force: id, target } = task.kind else {
            return;
        };
        let in_scope = self.in_scope(task.id);
        let today = self.today;
        let view = self.view;
        let risk = self.state.risk_read_up(task.id);
        let risk_text = format!("{} risk", risk.name());
        let target_text = base_words(view, target);
        let intent_words = format!("{}, bombarding {target_text} with {id}", task.id);
        let mut out = Out {
            orders: Vec::new(),
            summary: None,
            cannot: false,
        };
        let Some(force) = view.task_forces().find(|f| f.id == id).cloned() else {
            if let Some(day) =
                crate::available::entering_on(view, sim_core::ids::EntityRef::TaskForce(id))
            {
                // Held apart until its day (decision 0195, 13): one reason, no
                // order, and the intent is neither failed nor finished.
                out.summary = Some(
                    serving(task, &plan)
                        .evidence(crate::available::enters_words(id, day), today)
                        .binding(format!("the day {id} enters, {day}"))
                        .worked(format!(
                            "{intent_words}: {} is to be served by {id}, which enters on day {day}, so the staff gives {id} no order.",
                            task.id
                        ))
                        .would_change("the day come")
                        .done(),
                );
                self.hand_in_bombard(task, &plan, id, out, in_scope, &intent_words);
                return;
            }
            out.summary = Some(
                serving(task, &plan)
                    .evidence(format!("{id} is not one of the side's forces now"), today)
                    .worked(format!(
                        "{intent_words}: {id} is gone, so the staff gives no order and cannot meet {}.",
                        task.id
                    ))
                    .would_change("the force back")
                    .done(),
            );
            out.cannot = true;
            self.hand_in_bombard(task, &plan, id, out, in_scope, &intent_words);
            return;
        };
        if let Some(day) = crate::available::force_waiting(view, id) {
            let words = crate::available::from_day_words(day);
            out.summary = Some(
                serving(task, &plan)
                    .evidence(format!("{id} {words}"), today)
                    .binding(format!("the day {id} may be used from, {day}"))
                    .worked(format!(
                        "{intent_words}: {} is to be served by {id}, which {words}, so the staff gives {id} no order.",
                        task.id
                    ))
                    .would_change("the day come")
                    .done(),
            );
            self.hand_in_bombard(task, &plan, id, out, in_scope, &intent_words);
            return;
        }
        if let Some(hold) = self.state.hold_on_force(id, &force.ships) {
            out.summary = Some(
                serving(task, &plan)
                    .evidence(format!("{hold} is held by the commander"), today)
                    .binding(format!("the hold on {hold}"))
                    .worked(format!(
                        "{intent_words}: {hold} is held by the commander, so the staff gives {id} no order."
                    ))
                    .would_change("the hold released")
                    .done(),
            );
            self.hand_in_bombard(task, &plan, id, out, in_scope, &intent_words);
            return;
        }
        let ships = afloat(view, id);
        if ships.is_empty() {
            out.summary = Some(
                serving(task, &plan)
                    .evidence(format!("{id} has no ship afloat"), today)
                    .worked(format!(
                        "{intent_words}: {id} has no ship afloat, so the staff gives no order and cannot meet {}.",
                        task.id
                    ))
                    .would_change("a ship afloat in the force")
                    .done(),
            );
            out.cannot = true;
            self.hand_in_bombard(task, &plan, id, out, in_scope, &intent_words);
            return;
        }

        // Item 4: the target.
        let side = view.side();
        let Some(target_hex) = view
            .enemy_bases()
            .find(|b| b.id == target && b.side != side)
            .map(|b| b.hex)
        else {
            let why = if view.bases().any(|b| b.id == target) {
                format!("{target_text} is the side's own now")
            } else {
                format!("{target_text} is not on the side's map")
            };
            if force.readiness != Readiness::Normal {
                out.orders.push((
                    TaskForceOrder::SetReadiness(Readiness::Normal),
                    serving(task, &plan)
                        .evidence(why.clone(), today)
                        .binding("normal readiness on a morning the force does not sortie")
                        .worked(format!(
                            "{intent_words}: {why}, so {id} does not sortie and is set to normal readiness."
                        ))
                        .would_change("the base the enemy's again")
                        .done(),
                ));
            }
            out.summary = Some(
                serving(task, &plan)
                    .evidence(why.clone(), today)
                    .binding("the target must be a base of the enemy's on the side's map")
                    .worked(format!(
                        "{intent_words}: {why}, so the staff gives {id} no route."
                    ))
                    .would_change("the base the enemy's again")
                    .done(),
            );
            self.hand_in_bombard(task, &plan, id, out, in_scope, &intent_words);
            return;
        };
        let here = force.position();

        if let Some(order) = silence_order(view, &self.state, task, &plan, id, risk, today) {
            out.orders.push(order);
        }

        // The home port, as a stand.
        let Some((home, set_home)) = home_of(view, &force) else {
            out.summary = Some(
                serving(task, &plan)
                    .evidence(format!("{id} has no home port and lies at {here}, at no base of the side"), today)
                    .binding("the force must be stopped at its home port")
                    .worked(format!("{intent_words}: {id} has no home port and is not stopped at a base of the side, so the staff gives it no order."))
                    .would_change("a home port set, or the force stopped at a base")
                    .done(),
            );
            self.hand_in_bombard(task, &plan, id, out, in_scope, &intent_words);
            return;
        };
        let home_text = base_words(view, home);
        let Some(home_hex) = view.bases().find(|b| b.id == home).map(|b| b.hex) else {
            return;
        };
        if set_home {
            out.orders.push((
                TaskForceOrder::SetHomePort(home),
                serving(task, &plan)
                    .evidence(format!("{id} has no home port and lies stopped at {home_text}"), today)
                    .binding("the force's home port is the base it lies at when the task is served")
                    .worked(format!("{id} has no home port and lies stopped at {home_text}: its home port is set to {home_text}, where it sorties from and comes back to."))
                    .would_change("a home port set by the commander")
                    .done(),
            ));
        }
        if !(stopped(&force) && here == home_hex) {
            out.summary = Some(
                serving(task, &plan)
                    .evidence(format!("{id} is at {here}, not stopped at its home port {home_text}"), today)
                    .binding("the force sorties only from its home port")
                    .worked(format!(
                        "{intent_words}: {id} is at {here}, not stopped at its home port {home_text}; {}. The staff gives it no order.",
                        expected_words(view, id, home_hex)
                    ))
                    .would_change("the force stopped at its home port")
                    .done(),
            );
            self.hand_in_bombard(task, &plan, id, out, in_scope, &intent_words);
            return;
        }

        // Rules of engagement, set once from the risk.
        let wanted_roe = RulesOfEngagement {
            stance: Stance::Engage,
            break_off: break_off(risk),
            ..force.rules_of_engagement
        };
        if force.rules_of_engagement != wanted_roe {
            out.orders.push((
                TaskForceOrder::SetRulesOfEngagement {
                    stance: wanted_roe.stance,
                    break_off: wanted_roe.break_off,
                    stragglers: wanted_roe.stragglers,
                    keep_share: wanted_roe.keep_share,
                },
                serving(task, &plan)
                    .evidence(format!(
                        "{id} has the stance {} and a break-off level of {}",
                        stance_words(force.rules_of_engagement.stance),
                        force.rules_of_engagement.break_off
                    ), today)
                    .binding(format!("the break-off level of the {risk_text}: {}", break_off(risk)))
                    .worked(format!(
                        "A force that bombards by night engages: the rules of engagement of {id} are set to the stance engage and the break-off level of the {risk_text}, {}; its straggler instruction and keep share are left as they are.",
                        break_off(risk)
                    ))
                    .would_change("a different risk level")
                    .speaks_of(&self.state, LimitKind::Risk)
                    .done(),
            ));
        }

        // Item 5: whether it sorties tonight.
        let distance = here.distance(target_hex);
        let target_line = format!("{target_text} is at {target_hex}, {distance} hexes from {here}");
        let parent_kind = task
            .parent
            .and_then(|p| self.state.intents.get(&p).cloned());
        let (sorties, sortie_text, sortie_rule) = match parent_kind {
            Some(end) if matches!(end.kind, IntentKind::TakePlace { .. }) => {
                match self.landing_due(&end) {
                    Ok(day) if day == today => (
                        true,
                        format!("the landing force of {} is due at the place today, day {day}", end.id),
                        "under take a place the force sorties on the night of the day the landing force is due".to_string(),
                    ),
                    Ok(day) => (
                        false,
                        format!("the landing force of {} is due at the place on day {day}, not today", end.id),
                        "under take a place the force sorties on the night of the day the landing force is due, and on no other".to_string(),
                    ),
                    Err(why) => (
                        false,
                        format!("{why}; the force waits"),
                        "under take a place the force sorties on the night of the day the landing force is due, and on no other".to_string(),
                    ),
                }
            }
            _ => match risk {
                Risk::Press | Risk::Balanced => (
                    true,
                    format!("at {} the force sorties every night", risk.name()),
                    format!("at {} the force sorties every night", risk.name()),
                ),
                Risk::Preserve => {
                    let afloat_count = u32::try_from(ships.len()).unwrap_or(u32::MAX);
                    let picture = crate::picture::picture_of(view, task.kind.addressee());
                    let mut stopping: Vec<(u32, u32, u32)> = picture
                        .contacts()
                        .map(|c| {
                            (
                                c.estimated_hex.distance(target_hex),
                                picture.contact_number(c.id),
                                c.estimated_ship_count,
                            )
                        })
                        .filter(|(hexes, _, ships)| *hexes <= THREAT_HEXES && *ships > afloat_count)
                        .collect();
                    stopping.sort();
                    let rule = format!(
                        "at preserve the force it sorties on a night with no contact of more ships than it has afloat within {THREAT_HEXES} hexes of the target"
                    );
                    match stopping.first() {
                        Some((hexes, number, count)) => (
                            false,
                            format!(
                                "{} reports {count} ship(s) against the {afloat_count} {id} has afloat, {hexes} hexes from {target_text}, within {THREAT_HEXES}",
                                crate::picture::contact_words(&picture, *number)
                            ),
                            rule,
                        ),
                        None => (
                            true,
                            format!(
                                "the side holds no contact of more than {afloat_count} ship(s) within {THREAT_HEXES} hexes of {target_text}"
                            ),
                            rule,
                        ),
                    }
                }
            },
        };

        let mut battle = false;
        let mut route: Option<(TaskForceOrder, Reason)> = None;
        let mut speed_order: Option<(TaskForceOrder, Reason)> = None;
        if !sorties {
            out.summary = Some(
                serving(task, &plan)
                    .evidence(target_line.clone(), today)
                    .evidence(sortie_text.clone(), today)
                    .binding(sortie_rule.clone())
                    .worked(format!(
                        "{intent_words}: {target_line}; {sortie_text}, and {sortie_rule}: {id} does not sortie tonight."
                    ))
                    .would_change("a night on which the rule sends it")
                    .done(),
            );
        } else {
            // Item 6: the speed.
            let cruise = hours_to(view, id, target_hex, SpeedSetting::Cruise);
            let full = hours_to(view, id, target_hex, SpeedSetting::Full);
            let night = Fixed::from_int(NIGHT_HOURS);
            let chosen = match (cruise, full) {
                (Some(h), _) if h <= night => Some((SpeedSetting::Cruise, h)),
                (_, Some(h)) if h <= night => Some((SpeedSetting::Full, h)),
                _ => None,
            };
            let hours_text = format!(
                "the target is {} hours away at cruise and {} at full speed",
                cruise.map_or_else(|| "no".to_string(), |h| h.to_string()),
                full.map_or_else(|| "no".to_string(), |h| h.to_string())
            );
            match chosen {
                None => {
                    out.cannot = true;
                    out.summary = Some(
                        serving(task, &plan)
                            .evidence(target_line.clone(), today)
                            .evidence(hours_text.clone(), today)
                            .binding(format!("a sortie must reach the target within the night's {NIGHT_HOURS} hours"))
                            .worked(format!(
                                "{intent_words}: {target_line}; {sortie_text}, but {hours_text}: not even full speed brings {id} to the target within the night's {NIGHT_HOURS} hours, so there is no sortie and the staff cannot meet {}.",
                                task.id
                            ))
                            .would_change("a home port nearer the target")
                            .done(),
                    );
                }
                Some((speed, hours)) => {
                    // Item 7: fuel and ammunition first.
                    let Some((needs, _)) =
                        fuel_needs(view, id, &[target_hex, home_hex], Some(speed), margin(risk))
                    else {
                        return;
                    };
                    let short: Vec<ShipFuel> =
                        needs.iter().copied().filter(|n| n.short()).collect();
                    let fuel_text = format!(
                        "the passage to the target and home, by the engine's measure, times the {} of the {risk_text}: {}",
                        margin(risk),
                        needs_words(&needs)
                    );
                    let rounds_text = rounds_words(view, id, risk);
                    let ammunition = crate::ammunition::short(view, id, risk);
                    let speed_text = match speed {
                        SpeedSetting::Full => "full speed",
                        _ => "cruise",
                    };
                    if short.is_empty() && ammunition.is_none() {
                        battle = true;
                        if force.speed != speed {
                            speed_order = Some((
                                TaskForceOrder::SetSpeed(speed),
                                serving(task, &plan)
                                    .evidence(hours_text.clone(), today)
                                    .binding(format!("a sortie must reach the target within the night's {NIGHT_HOURS} hours"))
                                    .worked(format!("{hours_text}: {id} is set to {speed_text}, the slowest that reaches the target within the night."))
                                    .would_change("a target nearer, or farther")
                                    .done(),
                            ));
                        }
                        route = Some((
                            TaskForceOrder::SetRoute {
                                waypoints: vec![target_hex, home_hex],
                                act_at: BTreeSet::from([target_hex]),
                                stand_at: BTreeSet::new(),
                            },
                            serving(task, &plan)
                                .evidence(target_line.clone(), today)
                                .evidence(fuel_text.clone(), today)
                                .evidence(rounds_text.clone(), today)
                                .evidence(format!("{hours} hours to the target at {speed_text}"), today)
                                .binding(sortie_rule.clone())
                                .worked(format!(
                                    "{intent_words}: {target_line}; {sortie_text}. {hours} hours to the target at {speed_text}. {fuel_text}: it has the fuel. At {risk_text} {rounds_text}: it has the rounds. It is routed to {target_hex} and home to {home_text}, the target's hex marked to act at: it halts there, fires in the night's bombardment and steams for home at morning."
                                ))
                                .would_change("a night on which the rule does not send it, less fuel or fewer rounds aboard")
                                .done(),
                        ));
                    } else if short.is_empty() {
                        let ammunition = ammunition.expect("short of ammunition");
                        let when_text = match ammunition.hours.map(|h| usable_day(today, h)) {
                            Some(day) => {
                                format!("it rearms at {home_text} and may be used from day {day}")
                            }
                            None => {
                                out.cannot = true;
                                format!(
                                    "{home_text} cannot rearm it, so there is no day from which it may be used, and the staff cannot meet {}",
                                    task.id
                                )
                            }
                        };
                        out.summary = Some(
                            serving(task, &plan)
                                .evidence(target_line.clone(), today)
                                .evidence(fuel_text.clone(), today)
                                .evidence(ammunition.words.clone(), today)
                                .binding("a force short of ammunition does not sortie")
                                .worked(format!(
                                    "{intent_words}: {target_line}; {sortie_text}. {hours} hours to the target at {speed_text}. {fuel_text}: it has the fuel. But {}, and it does not sortie; {when_text}.",
                                    ammunition.words
                                ))
                                .would_change("more ammunition aboard")
                                .done(),
                        );
                    } else {
                        let triples: Vec<(Fixed, Fixed, Fixed)> = short
                            .iter()
                            .map(|n| {
                                let capacity = ships
                                    .iter()
                                    .find(|(s, _, _, _)| *s == n.ship)
                                    .map_or(n.need, |(_, _, c, _)| *c);
                                (n.aboard, n.need, capacity)
                            })
                            .collect();
                        let when =
                            fuelling_hours(view, home, &triples).map(|h| usable_day(today, h));
                        let ammunition_day = ammunition
                            .as_ref()
                            .map(|a| a.hours.map(|h| usable_day(today, h)));
                        let when = match ammunition_day {
                            None => when,
                            Some(day) => when.zip(day).map(|(a, b)| a.max(b)),
                        };
                        let ammunition_words = ammunition
                            .as_ref()
                            .map_or_else(String::new, |a| format!("; and {}", a.words));
                        let when_text = match when {
                            Some(day) if ammunition.is_some() => {
                                format!("it fuels and rearms at {home_text} and may be used from day {day}")
                            }
                            Some(day) => {
                                format!("it fuels at {home_text} and may be used from day {day}")
                            }
                            None => {
                                out.cannot = true;
                                format!(
                                    "{home_text} cannot fuel it, so there is no day from which it may be used, and the staff cannot meet {}",
                                    task.id
                                )
                            }
                        };
                        out.summary = Some(
                            serving(task, &plan)
                                .evidence(target_line.clone(), today)
                                .evidence(fuel_text.clone(), today)
                                .binding("fuel comes first: a force with less aboard than the sortie needs does not sortie")
                                .worked(format!(
                                    "{intent_words}: {target_line}; {sortie_text}, but {fuel_text}: it has less than it needs{ammunition_words}; it does not sortie; {when_text}."
                                ))
                                .would_change("more fuel aboard")
                                .done(),
                        );
                    }
                }
            }
        }

        // Readiness (item 8).
        let wanted = if battle {
            Readiness::Battle
        } else {
            Readiness::Normal
        };
        if force.readiness != wanted {
            out.orders.push((
                TaskForceOrder::SetReadiness(wanted),
                serving(task, &plan)
                    .evidence(sortie_text.clone(), today)
                    .binding("battle readiness on a night it sorties, normal on one it does not")
                    .worked(format!(
                        "{intent_words}: {}, so {id} is set to {} readiness.",
                        if battle {
                            "it sorties tonight"
                        } else {
                            "it does not sortie tonight"
                        },
                        if battle { "battle" } else { "normal" }
                    ))
                    .would_change("a night on which it sorties, or does not")
                    .done(),
            ));
        }
        if let Some(order) = speed_order {
            out.orders.push(order);
        }
        if let Some(order) = route {
            out.orders.push(order);
        }
        self.hand_in_bombard(task, &plan, id, out, in_scope, &intent_words);
    }

    fn hand_in_bombard(
        &mut self,
        task: &Intent,
        plan: &Plan,
        force: TaskForceId,
        out: Out,
        in_scope: bool,
        intent_words: &str,
    ) {
        let mut lines: Vec<String> = Vec::new();
        let target = self.target(in_scope);
        for (order, reason) in out.orders {
            lines.push(reason.worked.clone());
            target.orders.push(StaffOrder {
                kind: OrderKind::TaskForce {
                    target: force,
                    order,
                },
                intent: task.id,
                reason,
            });
        }
        if let Some(reason) = out.summary {
            lines.push(reason.worked.clone());
            target.no_orders.push(reason);
            // The notice on each morning it holds, as the stand doctrine's
            // (decision 0146, 6 and 7).
            if out.cannot {
                let index = target.no_orders.len() - 1;
                target.notices.push(StaffNotice {
                    kind: NoticeKind::CannotMeet,
                    plan: plan.id,
                    run: None,
                    rests_on: NoticeBasis::NoOrder(index),
                    pursuit: None,
                    follow: None,
                });
            }
        }
        if in_scope {
            self.lines
                .push(format!("{intent_words}: {}", lines.join(" ")));
        }
    }
}
