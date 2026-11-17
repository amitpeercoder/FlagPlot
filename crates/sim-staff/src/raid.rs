//! The raid doctrine, at the base's level (decision 0120, rules 10 to 16).
//! It belongs to the base, the addressee of the raid intents it serves: it
//! decides on which mornings the base's bombers go, how much they send and
//! with what escort. The area's doctrine that forms raid tasks is
//! `crate::neutralise`.
//!
//! The decision of one morning is a pure function of plain numbers,
//! [`decide`], so that it can be tested in a view built for it; the
//! [`Day`](crate::air::Day) reads the side's view into those numbers and
//! hands in the orders and reasons.
//!
//! Each morning, over the groups the raid may use that are at its base and
//! not held, and that are its to order (rule 16), in group number order:
//!
//! - a **bomber** (not a fighter, a weapon, the target in reach on normal
//!   reach) is set to strike the target on a morning it goes and the raid
//!   goes, and to rest otherwise; its held share is set from the risk level;
//! - a **fighter** the raid has is set to combat air patrol with a patrol
//!   share of 0, so that every aircraft it can fly is escort, and is given no
//!   rest order; a fighter the base's cover has is left to it, and its
//!   escorts are what its patrol share leaves;
//! - **any other group** is given no order.
//!
//! A bomber goes unless its fatigue is above the level its risk gives, or it
//! lost on its last raid, the day before, at least the share its risk gives
//! of what it sent. The raid goes when the bombers that go send between them
//! at least the weight the risk asks and, at preserve the force, when at
//! least one fighter can go with them.

use std::collections::BTreeMap;

use sim_core::command::{
    Intent, IntentKind, NoticeBasis, NoticeKind, Risk, StaffNotice, StaffOrder,
};
use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::ids::{AirGroupId, BaseId, IntentId};
use sim_core::orders::{AirGroupOrder, AirMission, OrderKind};
use sim_core::refdata::AircraftRole;
use sim_core::side_events::SideEventKind;
use sim_core::state::AirGroupLocation;
use sim_core::time::Pulse;

use crate::air::{mission_words, Day};
use crate::neutralise::{bomber_reach, distance_nm};
use crate::reasons::{base_words, serving};

// ---------------------------------------------------------------------------
// The figures of the risk table (decision 0120).
// ---------------------------------------------------------------------------

/// The fatigue above which a bomber group rests: the cover doctrine's (30,
/// 45, 60).
pub fn rest_level(risk: Risk) -> Fixed {
    Fixed::from_int(match risk {
        Risk::Preserve => 30,
        Risk::Balanced => 45,
        Risk::Press => 60,
    })
}

/// The share of its bombers a group holds back: a third at preserve the
/// force, none otherwise.
pub fn held_share(risk: Risk) -> Fixed {
    match risk {
        Risk::Preserve => Fixed::from_ratio(1, 3),
        Risk::Balanced | Risk::Press => Fixed::ZERO,
    }
}

/// The least a raid sends: 12, 6, 1. Placeholders that rest on no row.
pub fn least_weight(risk: Risk) -> u32 {
    match risk {
        Risk::Preserve => 12,
        Risk::Balanced => 6,
        Risk::Press => 1,
    }
}

/// Whether a raid goes without an escort: not at preserve the force.
pub fn goes_unescorted(risk: Risk) -> bool {
    risk != Risk::Preserve
}

/// The share of what it sent that a group must lose on its last raid to
/// stand down the morning after: a tenth, a quarter, never.
pub fn stand_down_share(risk: Risk) -> Option<Fixed> {
    match risk {
        Risk::Preserve => Some(Fixed::from_ratio(1, 10)),
        Risk::Balanced => Some(Fixed::from_ratio(1, 4)),
        Risk::Press => None,
    }
}

/// What a group can send: the aircraft it is able to fly less its held
/// share, cut to a whole aircraft (the engine's own rule, decision 0114, 8).
pub fn can_send(able: u32, held: Fixed) -> u32 {
    let sent = (Fixed::from_int(i64::from(able)) * (Fixed::ONE - held)).trunc();
    u32::try_from(sent).unwrap_or(0)
}

/// The escorts a fighter that can fly `able` aircraft gives at patrol share
/// `share`: what the share leaves (decision 0027, 2).
pub fn escorts_at(able: u32, share: Fixed) -> u32 {
    let patrol = (Fixed::from_int(i64::from(able)) * share).trunc();
    able.saturating_sub(u32::try_from(patrol).unwrap_or(0))
}

// ---------------------------------------------------------------------------
// The pure decision.
// ---------------------------------------------------------------------------

/// A group's last raid as its side's own events give it: the day, the
/// aircraft it sent and those it lost on it (shot down or ditched in that
/// pulse).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LastRaid {
    pub day: u32,
    pub sent: u32,
    pub lost: u32,
}

/// What a group is to the raid (rule 10, with rule 16).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RaidRole {
    /// It can bomb the target: the raid's.
    Bomber,
    /// A fighter the raid has: set to patrol at a share of 0.
    Fighter,
    /// A fighter the base's cover has: the raid gives it no order and draws
    /// what its patrol share leaves; `None` where the cover rests it this
    /// morning.
    CoverFighter(Option<Fixed>),
    /// Neither: no order.
    Other,
}

/// One group as the decision reads it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RaidGroup {
    pub id: AirGroupId,
    pub role: RaidRole,
    pub fatigue: Fixed,
    /// The aircraft it is able to fly this morning.
    pub able: u32,
    pub mission: AirMission,
    pub patrol_share: Fixed,
    pub held_share: Fixed,
    pub last_raid: Option<LastRaid>,
}

/// What the decision says of one bomber group.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BomberVerdict {
    /// It goes, and would send this many.
    Goes { sends: u32 },
    /// Its fatigue is above the level: it rests.
    Tired,
    /// It lost, on its last raid the day before, at least the share its
    /// risk gives: it stands down for the morning.
    StandsDown { lost: u32, sent: u32 },
}

/// Why the staff cannot meet the raid intent this morning (rule 15).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CannotRaid {
    /// No bomber group it may use can reach the target.
    NoBomber,
    /// At preserve the force, the base has no fighter group it may use or
    /// draw escorts from.
    NoFighter,
}

/// The decision of one morning.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RaidMorning {
    /// Each bomber group's verdict, in group number order.
    pub bombers: Vec<(AirGroupId, BomberVerdict)>,
    /// What the bombers that go send between them.
    pub weight: u32,
    /// The weight the risk asks.
    pub least: u32,
    /// The escorts the fighters at the base can give.
    pub escorts: u32,
    /// Whether the raid goes.
    pub goes: bool,
    /// The held share each bomber is to have.
    pub held: Fixed,
    pub cannot: Option<CannotRaid>,
    /// The orders, in group number order: for each group its mission first,
    /// then its settings, each only where it differs from what it has.
    pub orders: Vec<(AirGroupId, AirGroupOrder)>,
}

/// Whether a group stands down this morning for what its last raid cost
/// (rule 13): the raid was the day before and it lost at least the share
/// its risk gives of what it sent.
pub fn stands_down(risk: Risk, last: Option<LastRaid>, today: u32) -> bool {
    let (Some(last), Some(share)) = (last, stand_down_share(risk)) else {
        return false;
    };
    last.day + 1 == today
        && last.sent > 0
        && Fixed::from_int(i64::from(last.lost)) >= share * Fixed::from_int(i64::from(last.sent))
}

/// The raid doctrine's decision of one morning over `groups` (rules 10 to
/// 15), each already the raid's to read by rule 16, for a raid at `target`
/// at `risk` on day `today`.
pub fn decide(today: u32, risk: Risk, target: BaseId, groups: &[RaidGroup]) -> RaidMorning {
    let level = rest_level(risk);
    let held = held_share(risk);
    let least = least_weight(risk);
    let mut sorted: Vec<RaidGroup> = groups.to_vec();
    sorted.sort_by_key(|g| g.id);

    let mut bombers: Vec<(AirGroupId, BomberVerdict)> = Vec::new();
    let mut weight: u32 = 0;
    for g in sorted.iter().filter(|g| g.role == RaidRole::Bomber) {
        let verdict = if g.fatigue > level {
            BomberVerdict::Tired
        } else if stands_down(risk, g.last_raid, today) {
            let last = g.last_raid.unwrap_or(LastRaid {
                day: 0,
                sent: 0,
                lost: 0,
            });
            BomberVerdict::StandsDown {
                lost: last.lost,
                sent: last.sent,
            }
        } else {
            let sends = can_send(g.able, held);
            weight += sends;
            BomberVerdict::Goes { sends }
        };
        bombers.push((g.id, verdict));
    }
    let escorts: u32 = sorted
        .iter()
        .map(|g| match g.role {
            RaidRole::Fighter => g.able,
            RaidRole::CoverFighter(Some(share)) => escorts_at(g.able, share),
            _ => 0,
        })
        .sum();
    let has_fighter = sorted
        .iter()
        .any(|g| matches!(g.role, RaidRole::Fighter | RaidRole::CoverFighter(_)));
    let cannot = if bombers.is_empty() {
        Some(CannotRaid::NoBomber)
    } else if !goes_unescorted(risk) && !has_fighter {
        Some(CannotRaid::NoFighter)
    } else {
        None
    };
    let goes =
        cannot.is_none() && weight >= least && weight > 0 && (goes_unescorted(risk) || escorts > 0);

    let mut orders: Vec<(AirGroupId, AirGroupOrder)> = Vec::new();
    for g in &sorted {
        match g.role {
            RaidRole::Bomber => {
                let flies = goes
                    && bombers
                        .iter()
                        .any(|(id, v)| *id == g.id && matches!(v, BomberVerdict::Goes { .. }));
                let wanted = if flies {
                    AirMission::BaseStrike(target)
                } else {
                    AirMission::Rest
                };
                if g.mission != wanted {
                    orders.push((g.id, AirGroupOrder::SetAirMission(wanted)));
                }
                if g.held_share != held {
                    orders.push((g.id, AirGroupOrder::SetStrikeHeldShare { share: held }));
                }
            }
            RaidRole::Fighter => {
                if g.mission != AirMission::CombatAirPatrol {
                    orders.push((
                        g.id,
                        AirGroupOrder::SetAirMission(AirMission::CombatAirPatrol),
                    ));
                }
                if g.patrol_share != Fixed::ZERO {
                    orders.push((g.id, AirGroupOrder::SetPatrolShare { share: Fixed::ZERO }));
                }
            }
            RaidRole::CoverFighter(_) | RaidRole::Other => {}
        }
    }
    RaidMorning {
        bombers,
        weight,
        least,
        escorts,
        goes,
        held,
        cannot,
        orders,
    }
}

/// Whose a group is, where a cover and a raid task at one base may both use
/// it (rule 16): by its kind, never by the rank of the intents. A fighter is
/// the cover's where a cover of its base that may use it is in force, and
/// the raid's otherwise; a group that can bomb the target is the raid's,
/// though it can also search; any other is the cover's (it searches) where
/// there is one.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Owner {
    Cover,
    Raid,
    Neither,
}

/// Rule 16 for one group: `fighter` and `bomber` are what it is (a bomber
/// being one that can bomb the raid's target), `cover` and `raid` whether a
/// task of that kind in force at its base may use it.
pub fn whose(fighter: bool, bomber: bool, cover: bool, raid: bool) -> Owner {
    if fighter {
        if cover {
            Owner::Cover
        } else if raid {
            Owner::Raid
        } else {
            Owner::Neither
        }
    } else if bomber && raid {
        Owner::Raid
    } else if cover {
        Owner::Cover
    } else {
        Owner::Neither
    }
}

// ---------------------------------------------------------------------------
// The morning, from the side's view.
// ---------------------------------------------------------------------------

fn verdict_words(verdict: BomberVerdict, fatigue: Fixed, level: Fixed, risk: Risk) -> String {
    match verdict {
        BomberVerdict::Goes { sends } => format!(
            "fatigue {fatigue}, not above the {level} of the {} risk: it can send {sends}",
            risk.name()
        ),
        BomberVerdict::Tired => format!(
            "fatigue {fatigue}, above the {level} of the {} risk: it rests",
            risk.name()
        ),
        BomberVerdict::StandsDown { lost, sent } => format!(
            "on its last raid, yesterday, it lost {lost} of {sent}, at least the {} of what it sent that the {} risk allows: it stands down this morning",
            stand_down_share(risk).map_or_else(|| "none".to_string(), |s| s.to_string()),
            risk.name()
        ),
    }
}

impl Day<'_, '_> {
    /// The groups a raid task in force may use as bombers, each with the
    /// first such raid (by number): those at its base that can bomb its
    /// target (rule 16). The cover leaves them out.
    pub(crate) fn raid_bombers(&self) -> BTreeMap<AirGroupId, IntentId> {
        let mut found: BTreeMap<AirGroupId, IntentId> = BTreeMap::new();
        let raids: Vec<Intent> = self
            .state
            .intents_in_force()
            .filter(|i| matches!(i.kind, IntentKind::Raid { .. }))
            .cloned()
            .collect();
        for raid in raids {
            let IntentKind::Raid { base, target } = raid.kind else {
                continue;
            };
            for group in self.bombers_of(&raid, base, target) {
                found.entry(group).or_insert(raid.id);
            }
        }
        found
    }

    /// The groups `raid` may use at `base` that can bomb `target` on normal
    /// reach.
    fn bombers_of(&self, raid: &Intent, base: BaseId, target: BaseId) -> Vec<AirGroupId> {
        let Some(base_hex) = self.view.bases().find(|b| b.id == base).map(|b| b.hex) else {
            return Vec::new();
        };
        let Some(target_hex) = self
            .view
            .enemy_bases()
            .find(|b| b.id == target)
            .map(|b| b.hex)
        else {
            return Vec::new();
        };
        let distance = distance_nm(base_hex, target_hex);
        let may = self.may_use(raid);
        self.view
            .air_groups()
            .filter(|g| may.contains(&g.id) && g.location == AirGroupLocation::Base(base))
            .filter(|g| bomber_reach(self.view, g).is_some_and(|reach| reach >= distance))
            .map(|g| g.id)
            .collect()
    }

    /// Whether a cover in force of `base` may use `group` (rule 16).
    fn cover_may_use(&self, base: BaseId, group: AirGroupId) -> bool {
        let covers: Vec<Intent> = self
            .state
            .intents_in_force()
            .filter(|i| i.kind == IntentKind::Cover { base })
            .cloned()
            .collect();
        covers.iter().any(|c| self.may_use(c).contains(&group))
    }

    /// The last raid of `group` as the side's events give it.
    fn last_raid_of(&mut self, group: AirGroupId) -> Option<LastRaid> {
        let events = self.side_events();
        let mut last: Option<(u32, Option<Pulse>, u32)> = None;
        for event in events {
            if let SideEventKind::Whole(EventKind::BaseStrikeLaunched {
                air_group,
                aircraft,
                ..
            }) = &event.kind
            {
                if *air_group == group {
                    last = Some((event.day, event.pulse, *aircraft));
                }
            }
        }
        let (day, pulse, sent) = last?;
        let mut lost: u32 = 0;
        for event in events {
            if event.day != day || event.pulse != pulse {
                continue;
            }
            match &event.kind {
                SideEventKind::Whole(EventKind::AircraftLost {
                    air_group,
                    aircraft,
                    ..
                }) if *air_group == group => lost += *aircraft,
                SideEventKind::Whole(EventKind::AircraftDitched {
                    air_group, ditched, ..
                }) if *air_group == group => lost += *ditched,
                _ => {}
            }
        }
        Some(LastRaid { day, sent, lost })
    }

    /// The raid doctrine for the morning (decision 0120, 10 to 16).
    pub(crate) fn raids(&mut self) {
        let raids = self.in_priority_order(|i| matches!(i.kind, IntentKind::Raid { .. }));
        for id in raids {
            let Some(raid) = self.state.intents.get(&id).cloned() else {
                continue;
            };
            if !self.released(&raid) {
                continue;
            }
            self.serve_raid(&raid);
        }
    }

    #[allow(clippy::too_many_lines)]
    fn serve_raid(&mut self, raid: &Intent) {
        let Some(plan) = self.plan_of(raid) else {
            return;
        };
        let IntentKind::Raid { base, target } = raid.kind else {
            return;
        };
        let in_scope = self.in_scope(raid.id);
        let today = self.today;
        let base_text = base_words(self.view, base);
        let target_text = base_words(self.view, target);
        let risk = self.state.risk_read_up(raid.id);
        let level = rest_level(risk);
        let bombers: Vec<AirGroupId> = self.bombers_of(raid, base, target);

        // The groups: at the base, not held, the raid's by rule 16 and not
        // taken by a raid served before this one, in group number order.
        let mut members = self.at_base(raid, base);
        // A bomber a strike-ships task of higher rank may use is that task's
        // (decision 0123, 7).
        let strike_bombers = self.strike_bombers();
        members.retain(|m| {
            self.state.hold_on_group(m.id).is_none()
                && self.raid_taken.get(&m.id).is_none_or(|by| *by == raid.id)
                && strike_bombers
                    .get(&m.id)
                    .is_none_or(|task| !self.ranks_above(*task, raid.id))
        });
        let mut groups: Vec<RaidGroup> = Vec::new();
        let mut kinds: BTreeMap<AirGroupId, String> = BTreeMap::new();
        for m in &members {
            let fighter = matches!(
                self.view
                    .air_groups()
                    .find(|g| g.id == m.id)
                    .and_then(|g| self.view.aircraft_type(g.aircraft_type))
                    .map(|t| t.role),
                Some(AircraftRole::Fighter)
            );
            let bomber = bombers.contains(&m.id);
            // The bomber this raid lends a cover or search task of the base
            // to search today (decision 0131, 8).
            let lent = self
                .turns
                .get(&base)
                .filter(|t| t.group == m.id && t.lender == raid.id)
                .cloned();
            let role = match whose(fighter, bomber, self.cover_may_use(base, m.id), true) {
                _ if lent.is_some() => RaidRole::Other,
                Owner::Cover if fighter => {
                    RaidRole::CoverFighter(self.cover_fighters.get(&m.id).copied().flatten())
                }
                Owner::Raid if fighter => RaidRole::Fighter,
                Owner::Raid => RaidRole::Bomber,
                _ => RaidRole::Other,
            };
            if matches!(role, RaidRole::Bomber | RaidRole::Fighter) {
                self.raid_taken.insert(m.id, raid.id);
            }
            let held = self
                .view
                .air_groups()
                .find(|g| g.id == m.id)
                .map_or(Fixed::ZERO, |g| g.strike_held_share);
            let last_raid = if role == RaidRole::Bomber {
                self.last_raid_of(m.id)
            } else {
                None
            };
            kinds.insert(
                m.id,
                match role {
                    RaidRole::Other if lent.is_some() => format!(
                        "bomber group that searches today for {} and does not raid (fatigue {}): {}",
                        lent.as_ref().map_or(raid.id, |t| t.borrower),
                        m.fatigue,
                        lent.as_ref().map_or(String::new(), |t| t.words.clone())
                    ),
                    RaidRole::Bomber => "bomber group".to_string(),
                    RaidRole::Fighter => "fighter group".to_string(),
                    RaidRole::CoverFighter(_) => "fighter group of the base's cover".to_string(),
                    RaidRole::Other => {
                        "group that is neither a fighter nor can bomb the target".to_string()
                    }
                },
            );
            groups.push(RaidGroup {
                id: m.id,
                role,
                fatigue: m.fatigue,
                able: m.able,
                mission: m.mission,
                patrol_share: m.patrol_share,
                held_share: held,
                last_raid,
            });
        }
        let morning = decide(today, risk, target, &groups);
        let risk_text = format!("{} risk", risk.name());
        let escort_rule = if goes_unescorted(risk) {
            "it goes without an escort if it must"
        } else {
            "it does not go without an escort"
        };
        let weight_text = format!(
            "the bombers that go can send {} between them, against the {} the {risk_text} asks; the fighters can give {} escorts ({escort_rule})",
            morning.weight, morning.least, morning.escorts
        );
        let decision_text = if morning.goes {
            format!("the raid on {target_text} goes: {weight_text}")
        } else {
            match morning.cannot {
                Some(CannotRaid::NoBomber) => format!(
                    "the raid on {target_text} does not go: no bomber group {} may use at {base_text} can reach it on normal reach",
                    raid.id
                ),
                Some(CannotRaid::NoFighter) => format!(
                    "the raid on {target_text} does not go: at the {risk_text} it needs an escort and {base_text} has no fighter group {} may use or draw escorts from",
                    raid.id
                ),
                None => format!("the raid on {target_text} does not go: {weight_text}"),
            }
        };

        // Each group's line.
        let mut lines: BTreeMap<AirGroupId, String> = BTreeMap::new();
        for g in &groups {
            let kind = kinds.get(&g.id).cloned().unwrap_or_default();
            let line = match g.role {
                RaidRole::Bomber => {
                    let verdict = morning
                        .bombers
                        .iter()
                        .find(|(id, _)| *id == g.id)
                        .map_or(BomberVerdict::Tired, |(_, v)| *v);
                    format!(
                        "{}, a {kind}, has {}",
                        g.id,
                        verdict_words(verdict, g.fatigue, level, risk)
                    )
                }
                RaidRole::Fighter => format!(
                    "{}, a {kind} able to fly {}: it flies the patrol at a share of 0, every aircraft an escort, and only when a raid goes",
                    g.id, g.able
                ),
                RaidRole::CoverFighter(Some(share)) => format!(
                    "{}, a {kind}, is the cover's: it gives no order to it, and its escorts are what its patrol share of {share} leaves, {}",
                    g.id,
                    escorts_at(g.able, share)
                ),
                RaidRole::CoverFighter(None) => format!(
                    "{}, a {kind}, is the cover's and rests this morning: no order, no escort",
                    g.id
                ),
                RaidRole::Other => format!("{}, a {kind}: no order", g.id),
            };
            lines.insert(g.id, line);
        }

        let mut orders: Vec<StaffOrder> = Vec::new();
        for (group, order) in &morning.orders {
            let line = lines.get(group).cloned().unwrap_or_default();
            let before = self
                .view
                .air_groups()
                .find(|g| g.id == *group)
                .map(|g| (g.mission, g.patrol_share, g.strike_held_share));
            let (from, worked, binding) = match order {
                AirGroupOrder::SetAirMission(mission) => (
                    before.map_or_else(String::new, |b| mission_words(b.0)),
                    format!(
                        "{line}; {decision_text}: its mission is set from {} to {}.",
                        before.map_or_else(String::new, |b| mission_words(b.0)),
                        mission_words(*mission)
                    ),
                    format!("rest above fatigue {level}, and the least a raid sends, {}, at the {risk_text}", morning.least),
                ),
                AirGroupOrder::SetPatrolShare { share } => (
                    before.map_or_else(String::new, |b| b.1.to_string()),
                    format!(
                        "{line}: its patrol share is set from {} to {share}, so that every aircraft it can fly goes with a raid.",
                        before.map_or_else(String::new, |b| b.1.to_string())
                    ),
                    "a fighter the raid has flies a patrol share of 0".to_string(),
                ),
                AirGroupOrder::SetStrikeHeldShare { share } => (
                    before.map_or_else(String::new, |b| b.2.to_string()),
                    format!(
                        "{line}: its held share is set from {} to {share}, the share of its bombers the {risk_text} holds back.",
                        before.map_or_else(String::new, |b| b.2.to_string())
                    ),
                    format!("the held share of the {risk_text}: {share}"),
                ),
                _ => (String::new(), line.clone(), String::new()),
            };
            let reason = serving(raid, &plan)
                .evidence(line.clone(), today)
                .evidence(format!("it has {from}"), today)
                .evidence(weight_text.clone(), today)
                .binding(binding)
                .worked(worked)
                .would_change("a different fatigue, risk level, or cost of the last raid")
                .would_change("a cover of the base in force, for a fighter group")
                .done();
            orders.push(StaffOrder {
                kind: OrderKind::AirGroup {
                    target: *group,
                    order: order.clone(),
                },
                intent: raid.id,
                reason,
            });
        }

        let ordered: Vec<AirGroupId> = morning.orders.iter().map(|(g, _)| *g).collect();
        let unordered: Vec<String> = groups
            .iter()
            .filter(|g| !ordered.contains(&g.id))
            .filter_map(|g| lines.get(&g.id).cloned())
            .collect();
        let intent_words = format!("{}, the raid on {target_text} from {base_text}", raid.id);
        let waiting = self.waiting_note(raid, base);
        let out = self.target(in_scope);
        out.orders.extend(orders);
        if !unordered.is_empty() || morning.cannot.is_some() || groups.is_empty() {
            let body = if groups.is_empty() {
                "it has no group at the base that the staff may order".to_string()
            } else if unordered.is_empty() {
                "every group is given an order".to_string()
            } else {
                unordered.join("; ")
            };
            let body = format!("{body}{waiting}");
            let head = if ordered.is_empty() {
                "No order is given"
            } else {
                "No order is given for these groups"
            };
            let why = match morning.cannot {
                Some(_) => format!(" The staff cannot meet {}.", raid.id),
                None => String::new(),
            };
            let mut builder = serving(raid, &plan)
                .evidence(
                    format!(
                        "{intent_words}, with {} group(s) at the base that the staff may order",
                        groups.len()
                    ),
                    today,
                )
                .evidence(weight_text.clone(), today)
                .binding(format!("rest above fatigue {level} at the {risk_text}"))
                .binding(format!("the least a raid sends: {}", morning.least));
            if let Some(cannot) = morning.cannot {
                builder = builder.binding(match cannot {
                    CannotRaid::NoBomber => "no bomber group that can reach the target",
                    CannotRaid::NoFighter => "no fighter group to escort at preserve the force",
                });
            }
            let reason = builder
                .worked(format!(
                    "{head} for {intent_words} this morning: {body}. {decision_text}.{why}"
                ))
                .would_change(
                    "a group whose fatigue or mission differs from what the doctrine wants",
                )
                .done();
            out.no_orders.push(reason);
            let index = out.no_orders.len() - 1;
            if morning.cannot.is_some() {
                out.notices.push(StaffNotice {
                    kind: NoticeKind::CannotMeet,
                    plan: plan.id,
                    run: None,
                    rests_on: NoticeBasis::NoOrder(index),
                });
            }
        }
        if in_scope {
            let groups_text = if lines.is_empty() {
                "it has no group at the base that the staff may order".to_string()
            } else {
                lines.values().cloned().collect::<Vec<_>>().join("; ")
            };
            self.lines
                .push(format!("{intent_words}: {groups_text}; {decision_text}."));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn g(id: u32, role: RaidRole, fatigue: i64, able: u32) -> RaidGroup {
        RaidGroup {
            id: AirGroupId::new(id),
            role,
            fatigue: Fixed::from_int(fatigue),
            able,
            mission: AirMission::Rest,
            patrol_share: Fixed::ONE,
            held_share: Fixed::ZERO,
            last_raid: None,
        }
    }

    #[test]
    fn the_first_morning_of_scenario_50() {
        let target = BaseId::new(2);
        let morning = decide(
            1,
            Risk::Balanced,
            target,
            &[
                g(1, RaidRole::Bomber, 0, 10),
                g(2, RaidRole::Fighter, 0, 8),
                g(3, RaidRole::Bomber, 0, 10),
            ],
        );
        assert!(morning.goes);
        assert_eq!(morning.weight, 20);
        assert_eq!(morning.escorts, 8);
        assert_eq!(morning.orders.len(), 4);
    }

    #[test]
    fn preserve_holds_a_third_back() {
        assert_eq!(can_send(10, held_share(Risk::Preserve)), 6);
        assert_eq!(can_send(9, held_share(Risk::Preserve)), 6);
        assert_eq!(can_send(8, held_share(Risk::Preserve)), 5);
    }

    #[test]
    fn what_the_last_raid_cost() {
        let lost3 = Some(LastRaid {
            day: 4,
            sent: 10,
            lost: 3,
        });
        let lost2 = Some(LastRaid {
            day: 4,
            sent: 10,
            lost: 2,
        });
        assert!(stands_down(Risk::Balanced, lost3, 5));
        assert!(!stands_down(Risk::Balanced, lost2, 5));
        assert!(!stands_down(Risk::Press, lost3, 5));
        assert!(!stands_down(Risk::Balanced, lost3, 6));
    }
}
