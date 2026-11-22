//! Reconnaissance for an operation (decision 0147, items 13 to 20): the need
//! to know of a "take a place" whose plan is preparing, and the group the
//! staff sends to look.
//!
//! - **The need** (item 13), by the risk: at preserve the force a count of
//!   the enemy at the place seen from the air or by men in contact, dated no
//!   more than [`RECENT_DAYS`] days before the morning; at balanced such a
//!   count of any day; at press none. A scenario's opening estimate (told)
//!   never meets it. Where the commander sets the limit "knowledge" on the
//!   take or above it (decision 0159, 12), it and not the risk says how much
//!   must be known: none, a count, or a recent count of so many days; the
//!   risk still says the men wanted. Where nothing can look, a recent count
//!   is not launched on no count, as preserve was not, and a count goes, as
//!   balanced did.
//! - **The need as data** (decision 0159, 13): each morning a take's plan is
//!   preparing, what is asked, what is held with its age and what it could
//!   be in truth ([`could_be`]), who is looking and since when, and whether
//!   it is met, kept for the staff's `needs_to_know` and stored nowhere.
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
    CommandKind, CountRange, ForcesLimit, HeldCount, Intent, IntentKind, Knowledge, Limit, Looking,
    MovedToLook, NeedToKnow, NoticeKind, Plan, PlanState, Risk, StaffCommand, StaffOrder,
};
use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, IntentId, LandUnitId};
use sim_core::orders::{AirGroupOrder, AirMission, OrderKind};
use sim_core::refdata::ReportError;
use sim_core::side_events::SideEventKind;
use sim_core::state::{AirGroup, AirGroupLocation, ShipLocation};

use crate::air::{mission_words, Day};
use crate::reasons::serving;

/// The days before the morning within which a count seen from the air or by
/// men in contact meets the need at preserve the force (decision 0147, 13).
/// A placeholder, kept with the setting it is the default of
/// ([`Knowledge::RECENT_DAYS`], decision 0159, 12).
pub(crate) const RECENT_DAYS: u32 = Knowledge::RECENT_DAYS;

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
pub(crate) struct Known {
    pub(crate) estimate: Option<GroundEstimate>,
    pub(crate) met: bool,
    /// What is asked: the commander's "knowledge" limit read up the tree, or
    /// what the risk sets where none is set (decision 0159, 12).
    pub(crate) asked: Knowledge,
    /// The intent the "knowledge" limit is set on; `None` where the risk
    /// sets what is asked.
    pub(crate) asked_from: Option<IntentId>,
    /// What is asked, in words.
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

/// "1 day" or "2 days".
fn days_words(days: u32) -> String {
    if days == 1 {
        "1 day".to_string()
    } else {
        format!("{days} days")
    }
}

/// What is asked, in words. Where the risk sets it the words are those of
/// decision 0147; where the commander's "knowledge" limit sets it they say
/// so and on which intent it is set (decision 0159, 12).
fn asks_words(risk: Risk, asked: Knowledge, from: Option<IntentId>, place: Hex) -> String {
    let Some(on) = from else {
        return risk_asks_words(risk, place);
    };
    match asked {
        Knowledge::NoCount => format!(
            "the knowledge limit set on {on} asks for no count of the enemy at {place}"
        ),
        Knowledge::Count => format!(
            "the knowledge limit set on {on} asks for a count of the enemy at {place} seen from the air or by men in contact, of any day"
        ),
        Knowledge::RecentCount(days) => format!(
            "the knowledge limit set on {on} asks for a count of the enemy at {place} seen from the air or by men in contact, no more than {} old on the morning it goes",
            days_words(days)
        ),
    }
}

fn risk_asks_words(risk: Risk, place: Hex) -> String {
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

/// Whether the estimate meets what is asked (decision 0147, 13, with the
/// setting of decision 0159, 12): none is met by anything; a count by one
/// seen from the air or by men in contact; a recent count by such a count
/// no more than its days old on the morning. What was told meets neither.
fn met(asked: Knowledge, estimate: Option<&GroundEstimate>, today: u32) -> bool {
    meets(asked, estimate.map(|e| (e.grade, e.day)), today)
}

/// [`met`], on the grade and day of what is held.
fn meets(asked: Knowledge, held: Option<(GroundGrade, u32)>, today: u32) -> bool {
    match asked {
        Knowledge::NoCount => true,
        Knowledge::Count => held.is_some_and(|(grade, _)| grade != GroundGrade::Told),
        Knowledge::RecentCount(days) => held.is_some_and(|(grade, day)| {
            grade != GroundGrade::Told && today.saturating_sub(day) <= days
        }),
    }
}

/// What a count could be in truth, from its grade's error range (decision
/// 0159, 13): a report is the truth times a factor between `low` and
/// `high`, so the truth lies between the count over `high`, cut to a whole
/// man, and the count over `low`, raised to a whole man. A `high` of 0 can
/// only give a count of 0 and bounds nothing below: the lowest is 0. A `low`
/// of 0 bounds nothing above: the highest is `None`.
pub(crate) fn could_be(men: u32, error: ReportError) -> CountRange {
    let scaled = i128::from(men) * i128::from(Fixed::SCALE);
    let high = i128::from(error.high.raw());
    let low = i128::from(error.low.raw());
    let whole = |n: i128| u32::try_from(n).unwrap_or(u32::MAX);
    let lowest = if high > 0 { whole(scaled / high) } else { 0 };
    let highest = (low > 0).then(|| whole((scaled + low - 1) / low));
    CountRange { lowest, highest }
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
    pub(crate) fn group_hex(&self, group: &AirGroup) -> Option<Hex> {
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

    /// What is asked of `end`: the "knowledge" limit read up its tree, with
    /// the intent it is set on, or what the risk sets where none is set
    /// (decision 0159, 12).
    pub(crate) fn asked(&self, end: &Intent, risk: Risk) -> (Knowledge, Option<IntentId>) {
        match self.state.knowledge_read_up(end.id) {
            Some((knowledge, on)) => (knowledge, Some(on)),
            None => (Knowledge::of_risk(risk), None),
        }
    }

    /// Item 13 and the groups of item 14, for the place at the risk.
    pub(crate) fn knowledge(&self, end: &Intent, place: Hex, risk: Risk) -> Known {
        let (asked, asked_from) = self.asked(end, risk);
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
        Known {
            met: met(asked, estimate.as_ref(), self.today),
            asks: asks_words(risk, asked, asked_from, place),
            asked,
            asked_from,
            estimate,
            fighting_ashore,
            sender,
            on_it,
            left,
        }
    }

    /// What is held in words: the estimate, and where the commander's
    /// "knowledge" limit sets what is asked, its age on the morning
    /// (decision 0159, 12). Where the risk sets it the words are those of
    /// decision 0147.
    fn held_words(&self, place: Hex, k: &Known) -> String {
        let have = estimate_words(place, k.estimate.as_ref());
        match (k.asked_from, k.estimate) {
            (Some(_), Some(e)) => format!(
                "{have}, {} old",
                days_words(self.today.saturating_sub(e.day))
            ),
            _ => have,
        }
    }

    /// The day the side's log says `group`'s mission was set by the order
    /// that set it; `None` where no order of the side set it (a mission the
    /// scenario gave at the start).
    fn mission_since(&mut self, group: AirGroupId) -> Option<u32> {
        let order = self
            .view
            .air_groups()
            .find(|g| g.id == group)?
            .mission_order?;
        self.side_events().iter().find_map(|e| match &e.kind {
            SideEventKind::Whole(EventKind::OrderAccepted { order: o, .. }) if *o == order => {
                Some(e.day)
            }
            _ => None,
        })
    }

    /// Decision 0159, 13: the need to know of a take whose plan is
    /// preparing, as data, kept for [`crate::SupplyStaff`]'s
    /// `needs_to_know` and stored nowhere. The group looking is the one on
    /// reconnaissance of the place after this morning's orders: where the
    /// need is met, a group the commander holds on it (the staff sets the
    /// rest to rest); where it is not, the first group on it, or else the
    /// group the staff sends this morning, unless men of the side's own are
    /// ashore there.
    fn record_need(&mut self, end: &Intent, plan: &Plan, place: Hex, k: &Known) {
        let held_on = |day: &Self, id: &AirGroupId| day.state.hold_on_group(*id).is_some();
        let group = if k.met {
            k.on_it.iter().find(|id| held_on(self, id)).copied()
        } else if let Some(first) = k.on_it.first() {
            Some(*first)
        } else if k.fighting_ashore || k.asked == Knowledge::NoCount {
            None
        } else {
            k.sender.map(|sender| sender.id)
        };
        let looking = group.map(|air_group| {
            let since_day = if k.on_it.contains(&air_group) {
                self.mission_since(air_group)
            } else {
                Some(self.today)
            };
            Looking {
                air_group,
                since_day,
            }
        });
        let figures = self.view.ground_figures();
        let held = k.estimate.map(|e| {
            let error = match e.grade {
                GroundGrade::Told => None,
                GroundGrade::SeenFromAir => figures.and_then(|f| f.reconnaissance_error),
                GroundGrade::MenInContact => figures.and_then(|f| f.report_error),
            };
            HeldCount {
                men: e.men,
                grade: e.grade,
                day: e.day,
                age_days: self.today.saturating_sub(e.day),
                could_be: error.map(|error| could_be(e.men, error)),
            }
        });
        // Decision 0159, 14: the carrier force moved to look, while the need
        // is not met. One formed later this morning is set where it is
        // formed ([`Day::form_look_task`]).
        let moved_to_look = if k.met {
            None
        } else {
            self.carrier_task_under(end)
                .map(|(_, task_force, hex)| MovedToLook { task_force, hex })
        };
        self.needs.push(NeedToKnow {
            plan: plan.id,
            intent: end.id,
            place,
            asked: k.asked,
            asked_from: k.asked_from,
            held,
            looking,
            moved_to_look,
            met: k.met,
        });
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
        let k = self.knowledge(end, place, risk);
        self.record_need(end, plan, place, &k);
        // Where no count is asked (at press, or by the commander's
        // "knowledge" limit) none is sent (item 14).
        if k.asked == Knowledge::NoCount {
            return;
        }
        let asker = if k.asked_from.is_some() {
            "the knowledge limit"
        } else {
            "the risk"
        };
        let in_scope = self.in_scope(end.id);
        let today = self.today;
        let have = self.held_words(place, &k);
        if k.met {
            // Item 15: the group is set to rest, by one order; but a group
            // aboard the carrier force moved to look is left to its carrier
            // task, which sets it as on any morning (decision 0159, 14).
            let look_force = self.carrier_task_under(end).map(|(_, force, _)| force);
            for id in &k.on_it {
                let Some(group) = self.view.air_groups().find(|g| g.id == *id) else {
                    continue;
                };
                if self.state.hold_on_group(*id).is_some() {
                    continue;
                }
                if look_force.is_some_and(|force| self.group_aboard(*id, force)) {
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
                    .would_change(format!("a count older than {asker} asks for"))
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
        // A group on reconnaissance of the place while the need is not met
        // is the look's: a carrier task gives it no order (decision 0159,
        // 14).
        self.looking_groups.extend(k.on_it.iter().copied());
        // Item 14: with men of the side's own ashore at the place they
        // report it and no group looks; a group already sent flies.
        if k.fighting_ashore || !k.on_it.is_empty() {
            return;
        }
        let Some(sender) = k.sender else {
            return;
        };
        self.looking_groups.insert(sender.id);
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
            .would_change(format!(
                "a count of {asker}'s grade, or a group that reaches the place"
            ))
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
        let Ok(choice) = self.choose(end, &limit, place, source_hex, risk, false) else {
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
            submarines: Default::default(),
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
        let have = self.held_words(place, &k);
        let asker = if k.asked_from.is_some() {
            "the knowledge limit"
        } else {
            "the risk"
        };
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
                    .would_change(format!("a count of {asker}'s grade"))
                    .done();
                self.target(in_scope).no_orders.push(reason);
                return None;
            }
            // Decision 0159, 14: a carrier force on its way to the hex from
            // which a group of its own will look: the landing waits for it.
            if let Some((look_task, force, hex)) =
                self.carrier_task_under(end).filter(|_| !k.fighting_ashore)
            {
                let reason = serving(end, plan)
                    .evidence(ready_words.to_string(), today)
                    .evidence(have.clone(), today)
                    .evidence(format!("{look_task} operates {force} from {hex}, to look at {place}"), today)
                    .binding(k.asks.clone())
                    .worked(format!(
                        "The landing at {place} is ready ({ready_words}), but {have}; {}. {force} is moved to {hex} under {look_task}, from where a group aboard can look at {place}: the staff does not ask leave to launch and does not release {} until the need is met.",
                        k.asks, plan.id
                    ))
                    .would_change(format!("a count of {asker}'s grade"))
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
            // Where nothing can look, what is asked decides (decision 0159,
            // 12): a recent count is not launched on no count, as preserve
            // the force was not; a count goes, as balanced did.
            match k.asked {
                Knowledge::RecentCount(_) => {
                    let noticed = self.noticed(NoticeKind::CannotMeet, plan.id);
                    let who = if k.asked_from.is_some() {
                        "With a recent count asked".to_string()
                    } else {
                        "At preserve the force".to_string()
                    };
                    let reason = serving(end, plan)
                        .evidence(ready_words.to_string(), today)
                        .evidence(have.clone(), today)
                        .binding(k.asks.clone())
                        .worked(format!(
                            "The landing at {place} is ready ({ready_words}), but {have}; {}. {why}. {who} the staff does not launch on no count: it does not ask leave to launch and does not release {}{}.",
                            k.asks,
                            plan.id,
                            if noticed {
                                ""
                            } else {
                                ", and hands in the notice that it cannot meet the intent"
                            }
                        ))
                        .would_change(if k.asked_from.is_some() {
                            "a group that can look at the place, a count, or a knowledge limit that asks less"
                        } else {
                            "a group that can look at the place, a count, or a bolder risk level"
                        })
                        .done();
                    if noticed {
                        self.target(in_scope).no_orders.push(reason);
                    } else {
                        self.notice(in_scope, reason, NoticeKind::CannotMeet, plan);
                    }
                    return None;
                }
                Knowledge::Count | Knowledge::NoCount => {}
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
                    if let Ok(choice) = self.choose(end, &limit, place, source_hex, risk, false) {
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

#[cfg(test)]
mod tests {
    use super::{could_be, meets};
    use sim_core::belief::GroundGrade;
    use sim_core::command::{CountRange, Knowledge};
    use sim_core::fixed::Fixed;
    use sim_core::refdata::ReportError;

    fn range(low: &str, high: &str) -> ReportError {
        ReportError {
            low: low.parse::<Fixed>().expect("a figure"),
            high: high.parse::<Fixed>().expect("a figure"),
        }
    }

    fn between(lowest: u32, highest: u32) -> CountRange {
        CountRange {
            lowest,
            highest: Some(highest),
        }
    }

    /// Scenario 82's property "The range", on the decision's figures.
    #[test]
    fn what_a_count_could_be_in_truth() {
        // 3,000 seen from the air, 0.25 to 2: 1,500 to 12,000.
        assert_eq!(could_be(3000, range("0.25", "2")), between(1500, 12000));
        // 3,000 by men in contact, 0.5 to 1.5: 2,000 to 6,000.
        assert_eq!(could_be(3000, range("0.5", "1.5")), between(2000, 6000));
        // 100 with 0.25 to 2: 50 to 400.
        assert_eq!(could_be(100, range("0.25", "2")), between(50, 400));
        // 7 with 0.5 to 1.5: 4.67 cut to 4, 14.
        assert_eq!(could_be(7, range("0.5", "1.5")), between(4, 14));
    }

    /// Scenario 82's property "A count, at preserve the force": a count
    /// meets "a count" at any age; a recent count only within its days; what
    /// was told meets neither; "none" is met by nothing held.
    #[test]
    fn what_meets_each_setting() {
        let air = Some((GroundGrade::SeenFromAir, 1));
        assert!(meets(Knowledge::Count, air, 3));
        assert!(meets(Knowledge::Count, air, 1000));
        assert!(meets(Knowledge::RecentCount(1), air, 2));
        assert!(!meets(Knowledge::RecentCount(1), air, 3));
        let told = Some((GroundGrade::Told, 0));
        assert!(!meets(Knowledge::Count, told, 1));
        assert!(!meets(Knowledge::RecentCount(3), told, 1));
        assert!(meets(Knowledge::NoCount, None, 1));
        assert!(!meets(Knowledge::Count, None, 1));
    }
}
