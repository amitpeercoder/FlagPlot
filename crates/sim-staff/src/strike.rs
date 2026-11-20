//! The doctrine for striking ships from a base, at the base's level
//! (decision 0123, rules 5 to 8), and the size of each strike, at the group's
//! level (rule 9, with decision 0118). It belongs to the base, the addressee
//! of the strike intents it serves: it sets the standing orders of the
//! base's bombers from the risk level, and puts its fighters on patrol where
//! the base has no cover. The area's doctrine that forms these tasks is
//! `crate::deny`.
//!
//! The decision of one morning is a pure function of plain numbers,
//! [`decide`], so that it can be tested in a view built for it; the
//! [`Day`](crate::air::Day) reads the side's view into those numbers and
//! hands in the orders and reasons.
//!
//! Each morning, over the groups the task may use that are at its base and
//! not held, in group number order:
//!
//! - a **bomber** (not a fighter, a weapon) is the task's unless a raid task
//!   in force may use it as a bomber and ranks above this task; it is set to
//!   the naval strike mission, or to rest when its fatigue is above the level
//!   its risk gives, and its oldest contact, target list, reach and held
//!   share are set from the risk table; its strike sizing is set on, with a
//!   reason that says the group decided;
//! - a **fighter** is the base's cover's where one is in force, and is then
//!   left to it; with no cover it is set to combat air patrol with a patrol
//!   share of a half, since a base that strikes will be struck;
//! - **any other group** is given no order.

use std::collections::{BTreeMap, BTreeSet};

use sim_core::command::{Addressee, Intent, IntentKind, Risk, StaffOrder};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, IntentId};
use sim_core::orders::{AirGroupOrder, AirMission, OrderKind, Reach};
use sim_core::refdata::{AircraftRole, ShipKind};
use sim_core::state::AirGroupLocation;

use crate::air::{mission_words, Day};
use crate::neutralise::bomber_reach;
use crate::raid::rest_level;
use crate::reasons::{base_words, serving};
use crate::targets::{kinds_words, StrikeTargets};

// ---------------------------------------------------------------------------
// The risk table (decision 0123, 7, the rows of decision 0113's table).
// ---------------------------------------------------------------------------

/// The oldest contact a bomber launches on, in pulses: the figure
/// `strike_contact_max_age_pulses`, and at press the figure and two pulses
/// more, to 4 at most.
pub fn contact_age(risk: Risk, figure: u32) -> u32 {
    match risk {
        Risk::Preserve | Risk::Balanced => figure,
        Risk::Press => figure.saturating_add(2).min(4),
    }
}

/// The target list: carrier, battleship, cruiser at preserve the force,
/// every kind otherwise.
pub fn target_list(risk: Risk) -> Vec<ShipKind> {
    match risk {
        Risk::Preserve => vec![ShipKind::Carrier, ShipKind::Battleship, ShipKind::Cruiser],
        Risk::Balanced | Risk::Press => ShipKind::ALL.to_vec(),
    }
}

/// The reach: extended at press, normal otherwise.
pub fn reach(risk: Risk) -> Reach {
    match risk {
        Risk::Press => Reach::Extended,
        Risk::Preserve | Risk::Balanced => Reach::Normal,
    }
}

/// The share held back: a third at preserve the force, none otherwise.
pub fn held_share(risk: Risk) -> Fixed {
    crate::raid::held_share(risk)
}

/// The patrol share of a fighter the task has, with no cover at the base
/// (rule 8): a half.
pub fn fighter_share() -> Fixed {
    Fixed::from_ratio(1, 2)
}

// ---------------------------------------------------------------------------
// The pure decision.
// ---------------------------------------------------------------------------

/// What a group is to the strike task.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StrikeRole {
    /// Not a fighter, with a weapon: the task's.
    Bomber,
    /// A fighter, with no cover at the base: set to patrol at a half.
    Fighter,
    /// A fighter of the base's cover: no order.
    CoverFighter,
    /// Neither: no order.
    Other,
}

/// One group as the decision reads it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StrikeGroup {
    pub id: AirGroupId,
    pub role: StrikeRole,
    pub fatigue: Fixed,
    pub mission: AirMission,
    /// Its oldest contact as it stands, its own setting or the figure.
    pub contact_age: u32,
    pub target_priority: Vec<ShipKind>,
    pub reach: Reach,
    pub held_share: Fixed,
    pub sizing: bool,
    pub patrol_share: Fixed,
    /// Its waters to strike first as they stand (decision 0190).
    pub waters: BTreeSet<Hex>,
}

/// The orders of one morning, in group number order: for each group its
/// mission first, then its settings, each only where it differs from what
/// the group has. For a bomber: mission, oldest contact, target list (the
/// targets limit's kinds where one binds, decision 0190, 3), waters to
/// strike first (those `targets` gives, the empty set where none binds,
/// decision 0190, 33), reach, held share, sizing.
pub fn decide(
    risk: Risk,
    figure: u32,
    targets: &StrikeTargets,
    groups: &[StrikeGroup],
) -> Vec<(AirGroupId, AirGroupOrder)> {
    let level = rest_level(risk);
    let mut sorted: Vec<&StrikeGroup> = groups.iter().collect();
    sorted.sort_by_key(|g| g.id);
    let mut orders = Vec::new();
    for g in sorted {
        match g.role {
            StrikeRole::Bomber => {
                let wanted = if g.fatigue > level {
                    AirMission::Rest
                } else {
                    AirMission::NavalStrike
                };
                if g.mission != wanted {
                    orders.push((g.id, AirGroupOrder::SetAirMission(wanted)));
                }
                let age = contact_age(risk, figure);
                if g.contact_age != age {
                    orders.push((
                        g.id,
                        AirGroupOrder::SetStrikeContactAge {
                            pulses: i64::from(age),
                        },
                    ));
                }
                let kinds = targets.list_or(target_list(risk));
                if g.target_priority != kinds {
                    orders.push((g.id, AirGroupOrder::SetTargetPriority { kinds }));
                }
                if g.waters != targets.waters {
                    orders.push((
                        g.id,
                        AirGroupOrder::SetStrikeFirstWaters {
                            hexes: targets.waters.iter().copied().collect(),
                        },
                    ));
                }
                if g.reach != reach(risk) {
                    orders.push((g.id, AirGroupOrder::SetReach { reach: reach(risk) }));
                }
                if g.held_share != held_share(risk) {
                    orders.push((
                        g.id,
                        AirGroupOrder::SetStrikeHeldShare {
                            share: held_share(risk),
                        },
                    ));
                }
                if !g.sizing {
                    orders.push((g.id, AirGroupOrder::SetStrikeSizing { on: true }));
                }
            }
            StrikeRole::Fighter => {
                if g.mission != AirMission::CombatAirPatrol {
                    orders.push((
                        g.id,
                        AirGroupOrder::SetAirMission(AirMission::CombatAirPatrol),
                    ));
                }
                if g.patrol_share != fighter_share() {
                    orders.push((
                        g.id,
                        AirGroupOrder::SetPatrolShare {
                            share: fighter_share(),
                        },
                    ));
                }
            }
            StrikeRole::CoverFighter | StrikeRole::Other => {}
        }
    }
    orders
}

// ---------------------------------------------------------------------------
// The morning, from the side's view.
// ---------------------------------------------------------------------------

fn reach_words(reach: Reach) -> &'static str {
    match reach {
        Reach::Normal => "normal",
        Reach::Extended => "extended",
    }
}

impl Day<'_, '_> {
    /// The groups a strike-ships task in force may use as bombers at its
    /// base, each with the first such task (by priority): what a cover leaves
    /// out (decision 0123, 7, as decision 0120, 16) and what a raid of lower
    /// rank leaves out.
    pub(crate) fn strike_bombers(&self) -> BTreeMap<AirGroupId, IntentId> {
        let mut found: BTreeMap<AirGroupId, IntentId> = BTreeMap::new();
        for id in self.in_priority_order(|i| matches!(i.kind, IntentKind::StrikeShips { .. })) {
            let Some(task) = self.state.intents.get(&id) else {
                continue;
            };
            let IntentKind::StrikeShips { base } = task.kind else {
                continue;
            };
            let may = self.may_use(task);
            for group in self.view.air_groups().filter(|g| {
                may.contains(&g.id)
                    && g.location == AirGroupLocation::Base(base)
                    && bomber_reach(self.view, g).is_some()
            }) {
                found.entry(group.id).or_insert(id);
            }
        }
        found
    }

    /// Whether `first` comes before `second` in the morning's priority order
    /// (ranked first, then unranked, each by number).
    pub(crate) fn ranks_above(&self, first: IntentId, second: IntentId) -> bool {
        let key = |id: IntentId| {
            let rank = self.state.priority_read_up(id);
            (rank.is_none(), rank.unwrap_or(0), id)
        };
        key(first) < key(second)
    }

    /// The strike doctrine for the morning (decision 0123, 7 to 9).
    pub(crate) fn strikes(&mut self) {
        let tasks = self.in_priority_order(|i| matches!(i.kind, IntentKind::StrikeShips { .. }));
        for id in tasks {
            let Some(task) = self.state.intents.get(&id).cloned() else {
                continue;
            };
            if !self.released(&task) {
                continue;
            }
            self.serve_strike(&task);
        }
    }

    #[allow(clippy::too_many_lines)]
    fn serve_strike(&mut self, task: &Intent) {
        let Some(plan) = self.plan_of(task) else {
            return;
        };
        let IntentKind::StrikeShips { base } = task.kind else {
            return;
        };
        let in_scope = self.in_scope(task.id);
        let today = self.today;
        let base_text = base_words(self.view, base);
        let risk = self.state.risk_read_up(task.id);
        let level = rest_level(risk);
        let risk_text = format!("{} risk", risk.name());
        let figure = self
            .view
            .air_combat_figures()
            .map_or(0, |f| f.strike_contact_max_age_pulses);
        let has_cover = self.cover_in_force(base);
        let targets = self.strike_targets(task, None);

        let mut members = self.at_base(task, base);
        members.retain(|m| self.state.hold_on_group(m.id).is_none());
        let mut groups: Vec<StrikeGroup> = Vec::new();
        let mut lines: BTreeMap<AirGroupId, String> = BTreeMap::new();
        for m in &members {
            let Some(group) = self.view.air_groups().find(|g| g.id == m.id) else {
                continue;
            };
            // A group a raid served earlier this morning has, by its rank
            // (decision 0123, 7), or a strike task served earlier.
            if self.raid_taken.get(&m.id).is_some_and(|by| *by != task.id)
                || self
                    .strike_taken
                    .get(&m.id)
                    .is_some_and(|by| *by != task.id)
            {
                continue;
            }
            let fighter = self
                .view
                .aircraft_type(group.aircraft_type)
                .is_some_and(|t| t.role == AircraftRole::Fighter);
            // The bomber this task lends a cover or search task of the base
            // to search today (decision 0131, 8).
            let lent = self
                .turns
                .get(&base)
                .filter(|t| t.group == m.id && t.lender == task.id)
                .cloned();
            let role = if lent.is_some() {
                StrikeRole::Other
            } else if fighter {
                if has_cover {
                    StrikeRole::CoverFighter
                } else {
                    StrikeRole::Fighter
                }
            } else if bomber_reach(self.view, group).is_some() {
                StrikeRole::Bomber
            } else {
                StrikeRole::Other
            };
            if matches!(role, StrikeRole::Bomber | StrikeRole::Fighter) {
                self.strike_taken.insert(m.id, task.id);
            }
            let line = match role {
                StrikeRole::Bomber => format!(
                    "{}, a bomber group, has fatigue {}, {} the {level} of the {risk_text}",
                    m.id,
                    m.fatigue,
                    if m.fatigue > level {
                        "above"
                    } else {
                        "not above"
                    }
                ),
                StrikeRole::Fighter => format!(
                    "{}, a fighter group: {base_text} has no cover, and a base that strikes will be struck, so it flies the patrol at a share of {}",
                    m.id,
                    fighter_share()
                ),
                StrikeRole::CoverFighter => format!(
                    "{}, a fighter group, is the cover's: no order, and its escorts are what its patrol share leaves",
                    m.id
                ),
                StrikeRole::Other => match &lent {
                    Some(turn) => format!(
                        "{}, a bomber group with fatigue {}, searches today for {} and does not strike: {}",
                        m.id, m.fatigue, turn.borrower, turn.words
                    ),
                    None => format!(
                        "{}, a group that is neither a fighter nor carries a weapon: no order",
                        m.id
                    ),
                },
            };
            lines.insert(m.id, line);
            groups.push(StrikeGroup {
                id: m.id,
                role,
                fatigue: m.fatigue,
                mission: m.mission,
                contact_age: group.strike_contact_age.unwrap_or(figure),
                target_priority: group.target_priority.clone(),
                reach: group.reach,
                held_share: group.strike_held_share,
                sizing: group.strike_sizing,
                patrol_share: group.patrol_share,
                waters: group.strike_first_waters.clone(),
            });
        }
        let morning = decide(risk, figure, &targets, &groups);

        let mut orders: Vec<StaffOrder> = Vec::new();
        for (group, order) in &morning {
            let line = lines.get(group).cloned().unwrap_or_default();
            let Some(before) = groups.iter().find(|g| g.id == *group) else {
                continue;
            };
            let mut builder = serving(task, &plan).evidence(line.clone(), today);
            let (has, wanted, binding, would_change) = match order {
                AirGroupOrder::SetAirMission(mission) => (
                    mission_words(before.mission),
                    mission_words(*mission),
                    format!("rest above fatigue {level} at the {risk_text}"),
                    "a different fatigue or risk level",
                ),
                AirGroupOrder::SetStrikeContactAge { pulses } => (
                    format!("an oldest contact of {} pulse(s)", before.contact_age),
                    format!("{pulses} pulse(s)"),
                    format!(
                        "the oldest contact of the {risk_text}: the figure of {figure}{}",
                        if risk == Risk::Press {
                            " and two pulses more, to 4 at most"
                        } else {
                            ""
                        }
                    ),
                    "a different risk level",
                ),
                AirGroupOrder::SetTargetPriority { kinds } => match &targets.kinds {
                    Some((_, on)) => (
                        format!("a target list of {}", kinds_words(&before.target_priority)),
                        kinds_words(kinds),
                        format!(
                            "the targets limit set on {on}: {}, in that order, at every risk",
                            kinds_words(kinds)
                        ),
                        "the targets limit changed or cleared",
                    ),
                    None => (
                        format!("a target list of {}", kinds_words(&before.target_priority)),
                        kinds_words(kinds),
                        format!("the target list of the {risk_text}"),
                        "a different risk level",
                    ),
                },
                AirGroupOrder::SetStrikeFirstWaters { hexes } => (
                    crate::targets::waters_had(&before.waters),
                    crate::targets::waters_had(&hexes.iter().copied().collect()),
                    crate::targets::waters_binding(&targets),
                    "the deny withdrawn, or its targets limit changed or cleared",
                ),
                AirGroupOrder::SetReach { reach } => (
                    format!("{} reach", reach_words(before.reach)),
                    format!("{} reach", reach_words(*reach)),
                    format!("the reach of the {risk_text}"),
                    "a different risk level",
                ),
                AirGroupOrder::SetStrikeHeldShare { share } => (
                    format!("a held share of {}", before.held_share),
                    share.to_string(),
                    format!("the share held back at the {risk_text}"),
                    "a different risk level",
                ),
                AirGroupOrder::SetStrikeSizing { .. } => (
                    "strike sizing off".to_string(),
                    "on".to_string(),
                    "a group under a task to strike what comes within reach has more to hit than one force".to_string(),
                    "a task to destroy one named force",
                ),
                AirGroupOrder::SetPatrolShare { share } => (
                    format!("a patrol share of {}", before.patrol_share),
                    share.to_string(),
                    "a fighter the task has, with no cover at the base, flies a patrol share of a half".to_string(),
                    "a cover of the base in force",
                ),
                _ => (String::new(), String::new(), String::new(), "a different risk level"),
            };
            builder = builder
                .evidence(format!("it has {has}"), today)
                .binding(binding);
            let worked = match order {
                AirGroupOrder::SetStrikeSizing { .. } => format!(
                    "{line}. The group decides the size of its strikes from the task it is under: {} strikes ships that come within reach of {base_text}, which is more to hit than one force, so each strike is sized by doctrine, which sends everything at a kind it gives no figure for. Its strike sizing is set on.",
                    task.id
                ),
                _ => format!("{line}: it is set from {has} to {wanted}."),
            };
            let mut reason = builder.worked(worked).would_change(would_change).done();
            if matches!(order, AirGroupOrder::SetStrikeSizing { .. }) {
                // The group decided (decision 0123, 9).
                reason.serves.level = Some(Addressee::AirGroup(*group));
            }
            if matches!(order, AirGroupOrder::SetStrikeFirstWaters { .. }) {
                self.waters_serve(&targets, &mut reason);
            }
            orders.push(StaffOrder {
                kind: OrderKind::AirGroup {
                    target: *group,
                    order: order.clone(),
                },
                intent: task.id,
                reason,
            });
        }

        let ordered: Vec<AirGroupId> = morning.iter().map(|(g, _)| *g).collect();
        let unordered: Vec<String> = groups
            .iter()
            .filter(|g| !ordered.contains(&g.id))
            .filter_map(|g| lines.get(&g.id).cloned())
            .collect();
        let intent_words = format!("{}, striking ships from {base_text}", task.id);
        let wanted_text = format!(
            "at the {risk_text} a bomber flies the naval strike mission up to a fatigue of {level}, launches on a contact up to {} pulse(s) old, goes for {}{}, on {} reach, holding back {}, and sizes its strikes",
            contact_age(risk, figure),
            kinds_words(&targets.list_or(target_list(risk))),
            crate::targets::wanted_waters_words(&targets),
            reach_words(reach(risk)),
            held_share(risk)
        );
        let waiting = self.waiting_note(task, base);
        let out = self.target(in_scope);
        out.orders.extend(orders);
        if !unordered.is_empty() || groups.is_empty() {
            let body = if groups.is_empty() {
                "it has no group at the base that the staff may order".to_string()
            } else {
                unordered.join("; ")
            };
            let body = format!("{body}{waiting}");
            let head = if ordered.is_empty() {
                "No order is given"
            } else {
                "No order is given for these groups"
            };
            let reason = serving(task, &plan)
                .evidence(
                    format!(
                        "{intent_words}, with {} group(s) at the base that the staff may order",
                        groups.len()
                    ),
                    today,
                )
                .binding(wanted_text.clone())
                .worked(format!(
                    "{head} for {intent_words} this morning: {body}. {wanted_text}."
                ))
                .would_change(
                    "a group whose fatigue or standing orders differ from what the doctrine wants",
                )
                .done();
            out.no_orders.push(reason);
        }
        if in_scope {
            let groups_text = if lines.is_empty() {
                "it has no group at the base that the staff may order".to_string()
            } else {
                lines.values().cloned().collect::<Vec<_>>().join("; ")
            };
            self.lines
                .push(format!("{intent_words}: {groups_text}; {wanted_text}."));
        }
    }

    /// Whether a cover of `base` is in force.
    pub(crate) fn cover_in_force(&self, base: BaseId) -> bool {
        self.state
            .intents_in_force()
            .any(|i| i.kind == IntentKind::Cover { base })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn none() -> StrikeTargets {
        StrikeTargets::default()
    }

    fn bomber(id: u32) -> StrikeGroup {
        StrikeGroup {
            id: AirGroupId::new(id),
            role: StrikeRole::Bomber,
            fatigue: Fixed::ZERO,
            mission: AirMission::Rest,
            contact_age: 0,
            target_priority: ShipKind::ALL.to_vec(),
            reach: Reach::Normal,
            held_share: Fixed::ZERO,
            sizing: false,
            patrol_share: Fixed::ONE,
            waters: BTreeSet::new(),
        }
    }

    #[test]
    fn press_gives_four_orders_balanced_two_preserve_four() {
        assert_eq!(decide(Risk::Press, 0, &none(), &[bomber(1)]).len(), 4);
        assert_eq!(decide(Risk::Balanced, 0, &none(), &[bomber(1)]).len(), 2);
        assert_eq!(decide(Risk::Preserve, 0, &none(), &[bomber(1)]).len(), 4);
    }

    #[test]
    fn the_oldest_contact_at_press_is_capped_at_4() {
        assert_eq!(contact_age(Risk::Press, 0), 2);
        assert_eq!(contact_age(Risk::Press, 3), 4);
        assert_eq!(contact_age(Risk::Balanced, 1), 1);
    }

    fn limited(kinds: &[ShipKind], waters: &[Hex]) -> StrikeTargets {
        StrikeTargets {
            kinds: Some((kinds.to_vec(), IntentId::new(1))),
            waters: waters.iter().copied().collect(),
            denies: Vec::new(),
        }
    }

    #[test]
    fn a_targets_limit_sets_the_list_at_every_risk_and_the_waters_after_it() {
        let kinds = [ShipKind::Transport, ShipKind::Carrier];
        let waters = [Hex::new(4, 0), Hex::new(5, 0)];
        for risk in [Risk::Preserve, Risk::Balanced, Risk::Press] {
            let orders = decide(risk, 0, &limited(&kinds, &waters), &[bomber(1)]);
            let list = orders
                .iter()
                .position(|(_, o)| {
                    *o == AirGroupOrder::SetTargetPriority {
                        kinds: kinds.to_vec(),
                    }
                })
                .expect("the list is ordered");
            let set = orders
                .iter()
                .position(|(_, o)| {
                    *o == AirGroupOrder::SetStrikeFirstWaters {
                        hexes: waters.to_vec(),
                    }
                })
                .expect("the waters are ordered");
            assert_eq!(set, list + 1, "{risk:?}");
        }
    }

    #[test]
    fn with_no_limit_nothing_new_is_ordered_and_waters_left_over_are_cleared() {
        for risk in [Risk::Preserve, Risk::Balanced, Risk::Press] {
            assert!(decide(risk, 0, &none(), &[bomber(1)])
                .iter()
                .all(|(_, o)| !matches!(o, AirGroupOrder::SetStrikeFirstWaters { .. })));
        }
        let mut had = bomber(1);
        had.waters = [Hex::new(4, 0)].into_iter().collect();
        assert!(decide(Risk::Balanced, 0, &none(), &[had]).contains(&(
            AirGroupId::new(1),
            AirGroupOrder::SetStrikeFirstWaters { hexes: vec![] }
        )));
    }

    #[test]
    fn the_six_kinds_in_the_default_order_give_the_orders_of_no_limit_at_balanced_and_press() {
        for risk in [Risk::Balanced, Risk::Press] {
            assert_eq!(
                decide(
                    risk,
                    1,
                    &limited(&ShipKind::ALL, &[]),
                    &[bomber(1), bomber(2)]
                ),
                decide(risk, 1, &none(), &[bomber(1), bomber(2)])
            );
        }
    }

    #[test]
    fn a_tired_bomber_rests() {
        let mut tired = bomber(1);
        tired.fatigue = Fixed::from_int(61);
        tired.mission = AirMission::NavalStrike;
        let orders = decide(Risk::Press, 0, &none(), &[tired]);
        assert_eq!(orders[0].1, AirGroupOrder::SetAirMission(AirMission::Rest));
    }
}
