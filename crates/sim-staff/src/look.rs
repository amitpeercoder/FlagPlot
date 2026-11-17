//! Reconnaissance for an operation (decision 0147, items 13 to 20): the need
//! to know of a "take a place" whose plan is preparing, and the group the
//! staff sends to look.
//!
//! - **The need** (item 13), by the risk: at preserve the force a count of
//!   the enemy at the place seen from the air or by men in contact, dated no
//!   more than [`RECENT_DAYS`] days before the morning; at balanced such a
//!   count of any day; at press none. A scenario's opening estimate (told)
//!   never meets it.
//! - **The group** (item 14), [`Day::look_orders`]: each morning the need is
//!   unmet and the side has no fighting man ashore in the place's hex, one
//!   air group is set to reconnaissance of the place; when the need is met
//!   it is set to rest (item 15). The group is the lowest-numbered of those
//!   the end's forces limit names, read up the tree, that can search, that
//!   the side may use, that the commander does not hold, that no task served
//!   earlier this morning has taken, and whose range reaches the place from
//!   where it is based. At press none is sent.
//! - **The choice made again** (item 16), [`Day::choose_again`]: on the
//!   morning after a count seen from the air or by men in contact is first
//!   held, while the plan is preparing and no unit is aboard, the units are
//!   chosen again by decision 0142, items 8 to 10, on that count; where the
//!   choice differs the forces limit of the task "ready a landing" is set to
//!   it, and that task orders the landing again with the new roster.
//! - **Not launched without it** (items 18 and 19), [`Day::need_to_know`]:
//!   the landing is ready for leave and the release only when the need is
//!   met too. Where nothing can look: at preserve the force the plan is not
//!   released, the notice that the intent cannot be met is handed in once and
//!   a reason is given each morning after; at balanced it goes, and says on
//!   what.
//!
//! The doctrine reads the side's own view and its planned command state and
//! nothing else.

use std::collections::BTreeSet;

use sim_core::belief::{GroundEstimate, GroundGrade};
use sim_core::command::{
    CommandKind, ForcesLimit, Intent, IntentKind, Limit, NoticeKind, Plan, PlanState, Risk,
    StaffCommand, StaffOrder,
};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, LandUnitId};
use sim_core::orders::{AirGroupOrder, AirMission, OrderKind};
use sim_core::state::{AirGroup, AirGroupLocation, ShipLocation};

use crate::air::{mission_words, Day};
use crate::reasons::serving;

/// The days before the morning within which a count seen from the air or by
/// men in contact meets the need at preserve the force (decision 0147, 13).
/// A placeholder.
pub(crate) const RECENT_DAYS: u32 = 3;

/// The group that is to look, with its range and its distance from the
/// place.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Sender {
    pub(crate) id: AirGroupId,
    pub(crate) range: Fixed,
    pub(crate) distance: u32,
}

/// What the staff holds of the enemy at the place against what the risk
/// asks, and who could look.
#[derive(Clone, Debug)]
pub(crate) struct Knowledge {
    pub(crate) estimate: Option<GroundEstimate>,
    pub(crate) met: bool,
    /// What the risk asks for, in words.
    pub(crate) asks: String,
    /// The side has men of a fighting unit ashore in the place's hex.
    pub(crate) fighting_ashore: bool,
    /// The group to send where none is sent already.
    pub(crate) sender: Option<Sender>,
    /// The groups the end may use that are on reconnaissance of the place.
    pub(crate) on_it: Vec<AirGroupId>,
    /// Why the groups the end names are not sent, in words.
    pub(crate) left: Vec<String>,
}

/// An estimate in words: "the side's estimate of the enemy at (4, 0) is 150
/// men, seen from the air, of day 1".
pub(crate) fn estimate_words(place: Hex, estimate: Option<&GroundEstimate>) -> String {
    match estimate {
        Some(e) => format!(
            "the side's estimate of the enemy's men at {place} is {} men, {}, of day {}",
            e.men,
            e.grade.name(),
            e.day
        ),
        None => format!("the side holds no estimate of the enemy's men at {place}"),
    }
}

fn asks_words(risk: Risk, place: Hex) -> String {
    match risk {
        Risk::Preserve => format!(
            "at preserve the force it asks for a count of the enemy at {place} seen from the air or by men in contact, dated no more than {RECENT_DAYS} days before the morning"
        ),
        Risk::Balanced => format!(
            "at balanced it asks for a count of the enemy at {place} seen from the air or by men in contact, of any day"
        ),
        Risk::Press => "at press it asks for no count".to_string(),
    }
}

fn met(risk: Risk, estimate: Option<&GroundEstimate>, today: u32) -> bool {
    match risk {
        Risk::Press => true,
        Risk::Balanced => estimate.is_some_and(|e| e.grade != GroundGrade::Told),
        Risk::Preserve => estimate.is_some_and(|e| {
            e.grade != GroundGrade::Told && today.saturating_sub(e.day) <= RECENT_DAYS
        }),
    }
}

fn list(units: &BTreeSet<LandUnitId>) -> String {
    if units.is_empty() {
        "no unit".to_string()
    } else {
        units
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" and ")
    }
}

impl Day<'_, '_> {
    /// Where the air group is based now: its base's hex or its ship's
    /// position.
    fn group_hex(&self, group: &AirGroup) -> Option<Hex> {
        match group.location {
            AirGroupLocation::Base(base) => self.view.bases().find(|b| b.id == base).map(|b| b.hex),
            AirGroupLocation::Ship(ship) => {
                let ship = self.view.ships().find(|s| s.id == ship)?;
                match ship.location {
                    ShipLocation::TaskForce(force) => self
                        .view
                        .task_forces()
                        .find(|f| f.id == force)
                        .map(|f| f.position()),
                    ShipLocation::Port(base) => {
                        self.view.bases().find(|b| b.id == base).map(|b| b.hex)
                    }
                    ShipLocation::Yard(_) | ShipLocation::Sunk => None,
                }
            }
            AirGroupLocation::Lost => None,
        }
    }

    /// Item 13 and the groups of item 14, for the place at the risk.
    pub(crate) fn knowledge(&self, end: &Intent, place: Hex, risk: Risk) -> Knowledge {
        let estimate = self.view.ground_estimate(place).copied();
        let fighting_ashore = self
            .view
            .land_units()
            .any(|u| u.hex == Some(place) && u.men_ashore > 0 && !u.working);
        let mut left: Vec<String> = Vec::new();
        let mut sender: Option<Sender> = None;
        let mut on_it: Vec<AirGroupId> = Vec::new();
        let may = self.may_use(end);
        if may.is_empty() {
            left.push(format!(
                "the forces limit of {} names no air group that the side may use",
                end.id
            ));
        }
        for id in &may {
            let Some(group) = self.view.air_groups().find(|g| g.id == *id) else {
                continue;
            };
            if group.mission == AirMission::Reconnaissance(place) {
                on_it.push(group.id);
            }
            if self.state.hold_on_group(group.id).is_some() {
                left.push(format!("{} is held by the commander", group.id));
                continue;
            }
            if let Some(task) = self
                .taken
                .get(&group.id)
                .or_else(|| self.raid_taken.get(&group.id))
                .or_else(|| self.strike_taken.get(&group.id))
            {
                left.push(format!(
                    "{} serves {task}, a task served before this one",
                    group.id
                ));
                continue;
            }
            let Some(range) = crate::deny::search_range(self.view, group) else {
                left.push(format!("{} cannot search", group.id));
                continue;
            };
            let Some(origin) = self.group_hex(group) else {
                left.push(format!("{} has no place", group.id));
                continue;
            };
            let distance = origin.air_distance_nm(place);
            if Fixed::from_int(i64::from(distance)) > range {
                left.push(format!(
                    "{}'s range of {range} nm does not reach {place}, {distance} nm away",
                    group.id
                ));
                continue;
            }
            if sender.is_none() {
                sender = Some(Sender {
                    id: group.id,
                    range,
                    distance,
                });
            }
        }
        Knowledge {
            met: met(risk, estimate.as_ref(), self.today),
            asks: asks_words(risk, place),
            estimate,
            fighting_ashore,
            sender,
            on_it,
            left,
        }
    }

    // -- The group (items 14 and 15) -----------------------------------------

    /// Items 14 and 15: the order that sends a group to look at the place, or
    /// sets it to rest where the need is met. Runs after the landing task is
    /// served, with the plan preparing.
    pub(crate) fn look_orders(&mut self, end: &Intent, plan: &Plan) {
        let IntentKind::TakePlace { hex: place, .. } = end.kind else {
            return;
        };
        // The need is for a plan that is preparing.
        if plan.state != PlanState::Preparing {
            return;
        }
        let risk = self.state.risk_read_up(end.id);
        // At press none is sent (item 14).
        if risk == Risk::Press {
            return;
        }
        let in_scope = self.in_scope(end.id);
        let today = self.today;
        let k = self.knowledge(end, place, risk);
        let have = estimate_words(place, k.estimate.as_ref());
        if k.met {
            // Item 15: the group is set to rest, by one order.
            for id in &k.on_it {
                let Some(group) = self.view.air_groups().find(|g| g.id == *id) else {
                    continue;
                };
                if self.state.hold_on_group(*id).is_some() {
                    continue;
                }
                let reason = serving(end, plan)
                    .evidence(have.clone(), today)
                    .evidence(
                        format!("{id}'s mission is {}", mission_words(group.mission)),
                        today,
                    )
                    .binding(k.asks.clone())
                    .worked(format!(
                        "{have}; {}: the need is met, so {id} is set from {} to rest and is the staff's to use again.",
                        k.asks,
                        mission_words(group.mission)
                    ))
                    .would_change("a count older than the risk asks for")
                    .done();
                self.target(in_scope).orders.push(StaffOrder {
                    kind: OrderKind::AirGroup {
                        target: *id,
                        order: AirGroupOrder::SetAirMission(AirMission::Rest),
                    },
                    intent: end.id,
                    reason,
                });
            }
            return;
        }
        // Item 14: with men of the side's own ashore at the place they
        // report it and no group looks; a group already sent flies.
        if k.fighting_ashore || !k.on_it.is_empty() {
            return;
        }
        let Some(sender) = k.sender else {
            return;
        };
        let reason = serving(end, plan)
            .evidence(have.clone(), today)
            .evidence(
                format!(
                    "{} has a search range of {} nm and is {} nm from {place}",
                    sender.id, sender.range, sender.distance
                ),
                today,
            )
            .binding(k.asks.clone())
            .worked(format!(
                "{have}; {}: the need is not met, so the staff sets {}, the lowest-numbered group the forces limit names that can search, may be used, is not held and reaches {place} (range {} nm, {} nm away), to reconnaissance of {place}: one aircraft looks each morning until the need is met.",
                k.asks, sender.id, sender.range, sender.distance
            ))
            .would_change("a count of the risk's grade, or a group that reaches the place")
            .done();
        self.target(in_scope).orders.push(StaffOrder {
            kind: OrderKind::AirGroup {
                target: sender.id,
                order: AirGroupOrder::SetAirMission(AirMission::Reconnaissance(place)),
            },
            intent: end.id,
            reason,
        });
    }

    // -- The choice made again (item 16) -------------------------------------

    /// The count of the grade seen from the air or by men in contact that
    /// the side holds, while the plan is preparing and no man of a unit of
    /// the task is aboard: what the units are chosen on (item 16).
    fn count_held(&self, plan: &Plan, task: &Intent, place: Hex) -> Option<GroundEstimate> {
        if plan.state != PlanState::Preparing {
            return None;
        }
        let estimate = self.view.ground_estimate(place).copied()?;
        if estimate.grade == GroundGrade::Told {
            return None;
        }
        let units = self.state.land_units_on_chain(task.id)?;
        if units
            .iter()
            .any(|id| self.view.men_aboard(*id).unwrap_or(0) > 0)
        {
            return None;
        }
        Some(estimate)
    }

    /// The count of [`Day::count_held`] where it is first held this morning
    /// (reported the day before): the units are chosen again on it.
    fn rechoice_count(&self, plan: &Plan, task: &Intent, place: Hex) -> Option<GroundEstimate> {
        self.count_held(plan, task, place)
            .filter(|estimate| estimate.day + 1 == self.today)
    }

    /// Item 16: the units chosen again on the count first held this morning;
    /// where the choice differs the forces limit of the task is set to it.
    pub(crate) fn choose_again(&mut self, end: &Intent, plan: &Plan, task: &Intent) {
        let IntentKind::TakePlace { hex: place, .. } = end.kind else {
            return;
        };
        let Some(estimate) = self.rechoice_count(plan, task, place) else {
            return;
        };
        // The end's limit as it binds, with the units at the source the
        // staff may take where the commander named none (decision 0149, 4).
        let Some(limit) = self.rechoice_limit(end, task) else {
            return;
        };
        let Some(source_hex) = limit
            .source
            .and_then(|s| self.view.bases().find(|b| b.id == s).map(|b| b.hex))
        else {
            return;
        };
        let risk = self.state.risk_read_up(end.id);
        let Ok(choice) = self.choose(end, &limit, place, source_hex, risk) else {
            return;
        };
        // At preserve the force too few men is no landing (item 10 of 0142):
        // the choice is left as it is, and the launch is held back.
        if risk == Risk::Preserve && choice.short.is_some() {
            return;
        }
        let was = self.state.land_units_on_chain(task.id).unwrap_or_default();
        let now: BTreeSet<LandUnitId> = choice.picks.iter().map(|p| p.id).collect();
        if was == now {
            return;
        }
        let in_scope = self.in_scope(end.id);
        let today = self.today;
        let words = estimate_words(place, Some(&estimate));
        let choice_words = choice.lines.join("; ");
        let mut new_limit = task.limits.forces.clone().unwrap_or(ForcesLimit {
            ships: BTreeSet::new(),
            task_forces: BTreeSet::new(),
            air_groups: BTreeSet::new(),
            source: None,
            land_units: BTreeSet::new(),
        });
        new_limit.land_units = now.clone();
        if let Some(found) = self.state.intents.get_mut(&task.id) {
            if let Some(forces) = found.limits.forces.as_mut() {
                forces.land_units = now.clone();
            }
        }
        let reason = serving(end, plan)
            .evidence(words.clone(), today)
            .evidence(choice_words.clone(), today)
            .worked(format!(
                "{words}. The count first held changes the choice of units: {choice_words}. The roster of {} was {} and is now {}, so the staff sets the forces limit of {} to {}; the landing is ordered again with it, at the cost a change of roster has (decision 0095).",
                task.id,
                list(&was),
                list(&now),
                task.id,
                list(&now)
            ))
            .would_change("another count of the enemy at the place, or other units in the forces limit")
            .done();
        self.target(in_scope).commands.push(StaffCommand {
            kind: CommandKind::SetLimit(Limit::Forces {
                intent: task.id,
                limit: new_limit,
            }),
            reason,
        });
        if in_scope {
            self.lines.push(format!(
                "On the count first held, {words}, it sets the forces limit of {} to {}.",
                task.id,
                list(&now)
            ));
        }
    }

    // -- Not launched without it (items 18 and 19) ---------------------------

    /// Items 18 and 19, for a landing that is ready and a plan preparing:
    /// `Some(words)` where it may go on (the words say on what count it
    /// goes), `None` where the reason or the notice is handed in and it
    /// does not.
    #[allow(clippy::too_many_lines)]
    pub(crate) fn need_to_know(
        &mut self,
        end: &Intent,
        plan: &Plan,
        task: &Intent,
        ready_words: &str,
    ) -> Option<String> {
        let IntentKind::TakePlace { hex: place, .. } = end.kind else {
            return Some(String::new());
        };
        let in_scope = self.in_scope(end.id);
        let today = self.today;
        let risk = self.state.risk_read_up(end.id);
        let k = self.knowledge(end, place, risk);
        let have = estimate_words(place, k.estimate.as_ref());
        if !k.met {
            // A group sent, or sent this morning: the landing waits for the
            // count (item 18).
            if k.sender.is_some() || !k.on_it.is_empty() {
                let who = match (&k.on_it[..], k.sender) {
                    ([first, ..], _) => format!("{first} is on reconnaissance of {place}"),
                    ([], Some(sender)) => format!(
                        "{} is sent to {place} (range {} nm, {} nm away)",
                        sender.id, sender.range, sender.distance
                    ),
                    ([], None) => String::new(),
                };
                let reason = serving(end, plan)
                    .evidence(ready_words.to_string(), today)
                    .evidence(have.clone(), today)
                    .binding(k.asks.clone())
                    .worked(format!(
                        "The landing at {place} is ready ({ready_words}), but {have}; {}. {who}: the staff does not ask leave to launch and does not release {} until the need is met.",
                        k.asks, plan.id
                    ))
                    .would_change("a count of the risk's grade")
                    .done();
                self.target(in_scope).no_orders.push(reason);
                return None;
            }
            let why = if k.fighting_ashore {
                format!(
                    "the side's own men ashore at {place} have given no count, and no group is sent to look where they are"
                )
            } else {
                format!("nothing it may use can look: {}", k.left.join("; "))
            };
            match risk {
                Risk::Preserve => {
                    let noticed = self.noticed(NoticeKind::CannotMeet, plan.id);
                    let reason = serving(end, plan)
                        .evidence(ready_words.to_string(), today)
                        .evidence(have.clone(), today)
                        .binding(k.asks.clone())
                        .worked(format!(
                            "The landing at {place} is ready ({ready_words}), but {have}; {}. {why}. At preserve the force the staff does not launch on no count: it does not ask leave to launch and does not release {}{}.",
                            k.asks,
                            plan.id,
                            if noticed {
                                ""
                            } else {
                                ", and hands in the notice that it cannot meet the intent"
                            }
                        ))
                        .would_change("a group that can look at the place, a count, or a bolder risk level")
                        .done();
                    if noticed {
                        self.target(in_scope).no_orders.push(reason);
                    } else {
                        self.notice(in_scope, reason, NoticeKind::CannotMeet, plan);
                    }
                    return None;
                }
                Risk::Balanced | Risk::Press => {}
            }
        }
        // Decision 0142, 10, on the count held: at preserve the force
        // the men the ships carry fall short of the men wanted.
        if risk == Risk::Preserve {
            if let (Some(_), Some(limit)) = (
                self.count_held(plan, task, place),
                self.rechoice_limit(end, task),
            ) {
                if let Some(source_hex) = limit
                    .source
                    .and_then(|s| self.view.bases().find(|b| b.id == s).map(|b| b.hex))
                {
                    if let Ok(choice) = self.choose(end, &limit, place, source_hex, risk) {
                        if let Some((wanted, chosen)) = choice.short {
                            let noticed = self.noticed(NoticeKind::CannotMeet, plan.id);
                            let reason = serving(end, plan)
                                .evidence(ready_words.to_string(), today)
                                .evidence(have.clone(), today)
                                .binding(format!(
                                    "the ships can carry {chosen} men against the {wanted} wanted"
                                ))
                                .worked(format!(
                                    "The landing at {place} is ready ({ready_words}), but {have}: at preserve the force the staff lands only with the men wanted, and the ships can carry {chosen} men against the {wanted} wanted. It does not ask leave to launch and does not release {}{}.",
                                    plan.id,
                                    if noticed {
                                        ""
                                    } else {
                                        ", and hands in the notice that it cannot meet the intent"
                                    }
                                ))
                                .would_change("more ships, a smaller count, or a bolder risk level")
                                .done();
                            if noticed {
                                self.target(in_scope).no_orders.push(reason);
                            } else {
                                self.notice(in_scope, reason, NoticeKind::CannotMeet, plan);
                            }
                            return None;
                        }
                    }
                }
            }
        }
        // It goes: on what count (item 20).
        let goes = match k.estimate {
            None => format!(" It goes on no count of the enemy at {place}: {}.", k.asks),
            Some(e) if e.grade == GroundGrade::Told => format!(
                " It goes on what it was told, not on a count it has seen: {have}; {}.",
                k.asks
            ),
            Some(_) => format!(" It goes on the count held: {have}."),
        };
        Some(goes)
    }
}
