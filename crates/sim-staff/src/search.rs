//! The doctrine for searching waters from a base, at the base's level
//! (decision 0123, rules 11 to 13). It belongs to the base, the addressee of
//! the search intents it serves. The area's doctrine that forms these tasks
//! is `crate::deny`.
//!
//! Each morning, over the groups the task may use that are at its base and
//! not held, in group number order: a group that is not a fighter and can
//! search, and that no strike task in force has as a bomber, is set to search
//! at its type's range over the arc of decision 0115's rule 9 with the hexes
//! of the waters of the deny intent above it as the threats, and rests for a
//! day when its fatigue in the morning is above the level its risk gives
//! (decision 0115, 8, with no exception for a search group). Any other group
//! is given no order.
//!
//! Where a cover of the base is in force no search task is formed (rule 13):
//! the cover's arc takes the waters as threats (`crate::air`).

use sim_core::command::{Intent, IntentKind, Risk, StaffOrder};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, IntentId};
use sim_core::orders::{AirGroupOrder, AirMission, OrderKind};
use sim_core::state::{AirGroupLocation, SearchArc};

use crate::air::{arc_for, arc_words, bearing, mission_words, Day};
use crate::deny::{search_range, waters};
use crate::raid::rest_level;
use crate::reasons::{base_words, serving};

// ---------------------------------------------------------------------------
// A search that can be kept up (decision 0131), the pure parts.
// ---------------------------------------------------------------------------

/// Whether a group the staff sets to search flies it once a day, in the
/// morning (decision 0131, 6): at preserve the force and at balanced; at
/// press it searches in both daylight pulses.
pub fn once_a_day(risk: Risk) -> bool {
    risk != Risk::Press
}

/// The setting a searching group is to have, in words.
pub(crate) fn once_words(risk: Risk) -> String {
    if once_a_day(risk) {
        format!(
            "at {} risk a search is flown once a day, in the morning",
            risk.name()
        )
    } else {
        format!(
            "at {} risk a search is flown in both daylight pulses",
            risk.name()
        )
    }
}

/// A bomber group that can search, as the choice of who takes the search
/// reads it (decision 0131, 7 and 8).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TurnCandidate {
    pub id: AirGroupId,
    pub fatigue: Fixed,
    /// Aircraft able to fly now.
    pub able: u32,
}

/// The group that takes the search this morning (decision 0131, 7): the
/// least tired of the candidates that can fly, then the lowest numbered.
/// `None` where none can fly.
pub fn least_tired(candidates: &[TurnCandidate]) -> Option<AirGroupId> {
    candidates
        .iter()
        .filter(|c| c.able > 0)
        .min_by_key(|c| (c.fatigue, c.id))
        .map(|c| c.id)
}

/// The bomber that takes a base's search this morning (decision 0131, 8):
/// none where the cover or the search task has a group that can only
/// search, or where fewer than two of the task's bomber groups that can
/// search can fly (with one, it holds the strike); else the least tired of
/// them, then the lowest numbered.
pub fn base_turn(has_only_searcher: bool, bombers: &[TurnCandidate]) -> Option<AirGroupId> {
    if has_only_searcher || bombers.iter().filter(|c| c.able > 0).count() < 2 {
        return None;
    }
    least_tired(bombers)
}

/// The bomber a base's cover or search task borrows this morning (decision
/// 0131, 8), with the task that lends it and the words of the choice.
#[derive(Clone, Debug)]
pub(crate) struct Turn {
    pub(crate) group: AirGroupId,
    pub(crate) lender: IntentId,
    pub(crate) borrower: IntentId,
    pub(crate) words: String,
}

impl Day<'_, '_> {
    /// The bombers that take the search this morning, by base (decision
    /// 0131, 8), worked out once before the cover is served so that the
    /// cover, the search task, the raid and the strike read the same: for
    /// each cover and each search task in force whose plan is released, in
    /// priority order, at a base none has been worked for yet.
    pub(crate) fn work_turns(&mut self) {
        let borrowers = self.in_priority_order(|i| {
            matches!(
                i.kind,
                IntentKind::Cover { .. } | IntentKind::SearchWaters { .. }
            )
        });
        for id in borrowers {
            let Some(intent) = self.state.intents.get(&id).cloned() else {
                continue;
            };
            if !self.released(&intent) {
                continue;
            }
            let base = match intent.kind {
                IntentKind::Cover { base } | IntentKind::SearchWaters { base } => base,
                _ => continue,
            };
            if self.turns.contains_key(&base) {
                continue;
            }
            let own: Vec<AirGroupId> = self
                .at_base(&intent, base)
                .iter()
                .filter(|m| self.state.hold_on_group(m.id).is_none())
                .map(|m| m.id)
                .collect();
            if let Some(turn) = self.turn_at(base, &own, id) {
                self.turns.insert(base, turn);
            }
        }
    }

    /// The bomber that takes the search at `base` this morning for
    /// `borrower`, a cover or search task whose own groups are `own`: where
    /// none of them can only search, the first raid or strike task in force
    /// at the base, by priority, with two or more bomber groups that can
    /// search and can fly and are not held lends the least tired of them
    /// (decision 0131, 8).
    fn turn_at(&self, base: BaseId, own: &[AirGroupId], borrower: IntentId) -> Option<Turn> {
        let only_searcher = own.iter().any(|id| {
            self.view
                .air_groups()
                .find(|g| g.id == *id)
                .is_some_and(|g| {
                    search_range(self.view, g).is_some()
                        && crate::neutralise::bomber_reach(self.view, g).is_none()
                })
        });
        if only_searcher {
            return None;
        }
        // Whose each bomber is: a raid's first, then a strike task's, as the
        // cover reads them (decisions 0120, 16, and 0123, 7).
        let mut owners = self.raid_bombers();
        for (group, task) in self.strike_bombers() {
            owners.entry(group).or_insert(task);
        }
        let mut lenders: Vec<IntentId> = owners.values().copied().collect();
        lenders.sort_unstable();
        lenders.dedup();
        lenders.sort_by_key(|id| {
            let rank = self.state.priority_read_up(*id);
            (rank.is_none(), rank.unwrap_or(0), *id)
        });
        for lender in lenders {
            let candidates: Vec<TurnCandidate> = owners
                .iter()
                .filter(|(_, task)| **task == lender)
                .filter_map(|(id, _)| self.view.air_groups().find(|g| g.id == *id))
                .filter(|g| {
                    g.location == AirGroupLocation::Base(base)
                        && search_range(self.view, g).is_some()
                        && self.state.hold_on_group(g.id).is_none()
                })
                .map(|g| TurnCandidate {
                    id: g.id,
                    fatigue: g.fatigue,
                    able: self.view.able_to_fly(g.id).unwrap_or(0),
                })
                .collect();
            if let Some(group) = base_turn(false, &candidates) {
                let list = candidates
                    .iter()
                    .map(|c| {
                        format!(
                            "{} at fatigue {} with {} able to fly",
                            c.id, c.fatigue, c.able
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                return Some(Turn {
                    group,
                    lender,
                    borrower,
                    words: format!(
                        "no group of it can only search, and {lender} has {} bomber groups that can search ({list}): the least tired that can fly, then the lowest numbered, {group}, takes the search today and does not strike",
                        candidates.len()
                    ),
                });
            }
        }
        None
    }

    /// The search doctrine for the morning (decision 0123, 12).
    pub(crate) fn searches(&mut self) {
        let tasks = self.in_priority_order(|i| matches!(i.kind, IntentKind::SearchWaters { .. }));
        for id in tasks {
            let Some(task) = self.state.intents.get(&id).cloned() else {
                continue;
            };
            if !self.released(&task) {
                continue;
            }
            self.serve_search(&task);
        }
    }

    /// The waters of the deny intent above `task`, if there is one.
    fn waters_above(&self, task: &Intent) -> Option<(Hex, u32)> {
        self.state
            .ancestors(task.id)
            .into_iter()
            .find_map(|above| above.kind.waters())
    }

    #[allow(clippy::too_many_lines)]
    fn serve_search(&mut self, task: &Intent) {
        let Some(plan) = self.plan_of(task) else {
            return;
        };
        let IntentKind::SearchWaters { base } = task.kind else {
            return;
        };
        let in_scope = self.in_scope(task.id);
        let today = self.today;
        let base_text = base_words(self.view, base);
        let risk = self.state.risk_read_up(task.id);
        let level = rest_level(risk);
        let risk_text = format!("{} risk", risk.name());
        let (hex, distance) = self.waters_above(task).unwrap_or((Hex::new(0, 0), 0));
        let hexes = waters(self.view, hex, distance);
        let base_hex = self.view.bases().find(|b| b.id == base).map(|b| b.hex);
        let arc = base_hex.map_or_else(|| arc_for(Hex::new(0, 0), &[]), |at| arc_for(at, &hexes));
        let threats_text = hexes
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        let arc_text = match arc.narrow {
            Some((from, to)) if !arc.whole() => format!(
                "the waters within {distance} hexes of {hex}: {threats_text}; the narrowest arc that holds them is {} to {}, widened by 30 degrees at each end to {} to {}",
                bearing(from),
                bearing(to),
                bearing(arc.from),
                bearing(arc.to)
            ),
            _ => format!(
                "the waters within {distance} hexes of {hex}: {threats_text}; the arc is the whole circle"
            ),
        };
        let mut strike_bombers = self.strike_bombers();
        // The bomber a strike or raid task lends the search today (decision
        // 0131, 8): the search's, whatever its limit says.
        let turn = self
            .turns
            .get(&base)
            .filter(|t| t.borrower == task.id)
            .cloned();
        let once = once_a_day(risk);
        let once_text = once_words(risk);

        let mut members = self.at_base(task, base);
        if let Some(turn) = &turn {
            strike_bombers.remove(&turn.group);
            if !members.iter().any(|m| m.id == turn.group) {
                if let Some(group) = self.view.air_groups().find(|g| g.id == turn.group) {
                    members.push(self.member(group));
                    members.sort_by_key(|m| m.id);
                }
            }
        }
        members.retain(|m| self.state.hold_on_group(m.id).is_none());
        let mut orders: Vec<StaffOrder> = Vec::new();
        let mut unordered: Vec<String> = Vec::new();
        let mut everyone: Vec<String> = Vec::new();
        let give =
            |orders: &mut Vec<StaffOrder>, group: AirGroupId, order: AirGroupOrder, reason| {
                orders.push(StaffOrder {
                    kind: OrderKind::AirGroup {
                        target: group,
                        order,
                    },
                    intent: task.id,
                    reason,
                });
            };
        for m in &members {
            let g = m.id;
            let Some(group) = self.view.air_groups().find(|own| own.id == g) else {
                continue;
            };
            let range = search_range(self.view, group);
            let line;
            let before = orders.len();
            match (range, strike_bombers.get(&g)) {
                (Some(_), Some(strike)) => {
                    line = format!(
                        "{g} is the strike task's: {strike} may use it as a bomber, so the search gives it no order"
                    );
                }
                (None, _) => {
                    line = format!("{g} is a fighter or cannot search: no order");
                }
                (Some(range), None) => {
                    let numbers = match &turn {
                        Some(turn) if turn.group == g => format!(
                            "{g}, a bomber group of {} that can search, has fatigue {} against the {level} of the {risk_text}; {}",
                            turn.lender, m.fatigue, turn.words
                        ),
                        _ => format!(
                            "{g}, a search group, has fatigue {} against the {level} of the {risk_text}",
                            m.fatigue
                        ),
                    };
                    if m.fatigue > level {
                        if m.mission != AirMission::Rest {
                            give(
                                &mut orders,
                                g,
                                AirGroupOrder::SetAirMission(AirMission::Rest),
                                serving(task, &plan)
                                    .evidence(format!("{numbers}; it is above the level"), today)
                                    .evidence(format!("its mission is {}", mission_words(m.mission)), today)
                                    .binding(format!("rest above fatigue {level} at the {risk_text}"))
                                    .worked(format!(
                                        "{numbers}; it is above the level and a search group is never kept: it rests for a day, so its mission is set from {} to rest.",
                                        mission_words(m.mission)
                                    ))
                                    .would_change("a lower fatigue, or a bolder risk level")
                                    .done(),
                            );
                        }
                        line = format!("{numbers}; it is above the level, so it rests");
                    } else {
                        if m.mission != AirMission::Search {
                            give(
                                &mut orders,
                                g,
                                AirGroupOrder::SetAirMission(AirMission::Search),
                                serving(task, &plan)
                                    .evidence(format!("{numbers}; it is not above the level"), today)
                                    .evidence(format!("its mission is {}", mission_words(m.mission)), today)
                                    .binding(format!("rest above fatigue {level} at the {risk_text}"))
                                    .worked(format!(
                                        "{numbers}; it is not above the level, so it searches the waters: its mission is set from {} to search.",
                                        mission_words(m.mission)
                                    ))
                                    .would_change("a fatigue above the level")
                                    .done(),
                            );
                        }
                        if m.search_once != once {
                            give(
                                &mut orders,
                                g,
                                AirGroupOrder::SetSearchOnce { on: once },
                                serving(task, &plan)
                                    .evidence(format!("{numbers}; it is not above the level, so it searches"), today)
                                    .evidence(
                                        format!(
                                            "its search once a day is {}",
                                            if m.search_once { "on" } else { "off" }
                                        ),
                                        today,
                                    )
                                    .binding(once_text.clone())
                                    .worked(format!(
                                        "{numbers}; it searches the waters, and {once_text}: its search once a day is set {}.",
                                        if once { "on" } else { "off" }
                                    ))
                                    .would_change("a different risk level")
                                    .done(),
                            );
                        }
                        let wanted = SearchArc {
                            from_deg: arc.from,
                            to_deg: arc.to,
                            range_nm: range,
                        };
                        if group.search != Some(wanted) {
                            give(
                                &mut orders,
                                g,
                                AirGroupOrder::SetSearchArc {
                                    from_deg: i32::try_from(arc.from).unwrap_or(0),
                                    to_deg: i32::try_from(arc.to).unwrap_or(0),
                                    range_nm: range,
                                },
                                serving(task, &plan)
                                    .evidence(format!("{numbers}; it is not above the level"), today)
                                    .evidence(
                                        match group.search {
                                            Some(arc) => format!(
                                                "its arc is {} to {} at {} nm",
                                                bearing(arc.from_deg),
                                                bearing(arc.to_deg),
                                                arc.range_nm
                                            ),
                                            None => "it has no search arc".to_string(),
                                        },
                                        today,
                                    )
                                    .binding(format!(
                                        "the arc the waters give, at its type's range of {range} nm"
                                    ))
                                    .worked(format!(
                                        "The search arc of {g} is set to {} out to {range} nm: {arc_text}.",
                                        arc_words(&arc)
                                    ))
                                    .would_change("other waters")
                                    .done(),
                            );
                        }
                        line = format!(
                            "{numbers}; it searches {} out to {range} nm; {once_text}",
                            arc_words(&arc)
                        );
                    }
                }
            }
            if orders.len() == before {
                unordered.push(line.clone());
            }
            everyone.push(line);
        }
        let intent_words = format!("{}, searching the waters from {base_text}", task.id);
        let out = self.target(in_scope);
        out.orders.extend(orders);
        if !unordered.is_empty() || members.is_empty() {
            let body = if members.is_empty() {
                "it has no group at the base that the staff may order".to_string()
            } else {
                unordered.join("; ")
            };
            let reason = serving(task, &plan)
                .evidence(
                    format!(
                        "{intent_words}, with {} group(s) at the base that the staff may order",
                        members.len()
                    ),
                    today,
                )
                .binding(format!("rest above fatigue {level} at the {risk_text}"))
                .binding(format!("search arc: {arc_text}"))
                .worked(format!(
                    "No order is given for these groups for {intent_words} this morning: {body}. The arc is {} ({arc_text}).",
                    arc_words(&arc)
                ))
                .would_change("a group whose fatigue or mission differs from what the doctrine wants")
                .done();
            out.no_orders.push(reason);
        }
        if in_scope {
            self.lines.push(format!(
                "{intent_words}: {}. The arc is {} ({arc_text}).",
                if everyone.is_empty() {
                    "it has no group at the base that the staff may order".to_string()
                } else {
                    everyone.join("; ")
                },
                arc_words(&arc)
            ));
        }
    }
}
