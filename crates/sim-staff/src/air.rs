//! The air doctrine: hold a place and cover a base from the air (decisions
//! 0117, rule 13, and 0115, items 1 to 11). It runs after the supply
//! doctrine and hands in nothing for a side with no hold or cover intent in
//! force, so every game of stage 1 gives the orders it gave.
//!
//! Each morning, on a clone of the side's command state that it updates as
//! it hands in (the plans the leaves approve this morning, the sub-intents it
//! forms this morning), in this order:
//!
//! 1. The leaves, for each root that is a hold or a cover intent: a draft
//!    with leave to prepare is approved, a plan preparing with leave to
//!    launch is released the same morning; without a leave, a reason and a
//!    notice say it waits; a plan that has ended has one reason.
//! 2. The hold doctrine, for each hold intent whose plan is released: where
//!    a cover intent for the base stands, none is formed; where nothing at
//!    the base can cover it, none is formed; otherwise cover from the air is
//!    formed under it.
//! 3. The neutralise doctrine, at the area's level (decision 0120, 6 and 7;
//!    `crate::neutralise`): for each neutralise intent whose plan is
//!    released, a raid task for each base of the side with bombers that
//!    reach the target. It forms sub-intents only and gives no order.
//! 4. The cover doctrine, for each cover intent whose plan is released, in
//!    priority order, over the groups it may use that are at its base and
//!    not held: fighters fly the patrol or rest, groups that can search
//!    search or rest, other groups are left for a strike intent. A group a
//!    raid task in force may use as a bomber is the raid's and not the
//!    cover's (decision 0120, 16, amending decision 0115, 7).
//! 5. The raid doctrine, at the base's level (decision 0120, 10 to 16;
//!    `crate::raid`): for each raid intent whose plan is released, which
//!    mornings its bombers go, with their escort.
//!
//! Each doctrine belongs to a level (decision 0120, 1): the addressee of the
//! intents it serves. Every reason carries it (`Serves::level`).
//!
//! The doctrine reads the side's own view and nothing else. A proposal runs
//! the same day over one tree as if its plan were released and keeps what is
//! handed in for that tree.

use std::collections::BTreeMap;

use sim_core::bearing::in_arc;
use sim_core::command::{
    Addressee, CommandKind, CommandState, Intent, IntentKind, IntentLimits, Leave, Maker,
    NoticeBasis, NoticeKind, Plan, PlanState, Proposal, Reason, Risk, StaffCommand, StaffDecisions,
    StaffNotice, StaffOrder,
};
use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, IntentId};
use sim_core::orders::{AirGroupOrder, AirMission, OrderKind};
use sim_core::refdata::AircraftRole;
use sim_core::side_events::SideEventKind;
use sim_core::state::{AirGroupLocation, SearchArc};
use sim_core::view::SideView;

use crate::reasons::{base_words, fill_decisions, fill_reason, plan_for, serving};

// ---------------------------------------------------------------------------
// The figures of decision 0115.
// ---------------------------------------------------------------------------

/// The fatigue above which a group rests, by risk level (item 8).
fn level_of(risk: Risk) -> Fixed {
    Fixed::from_int(match risk {
        Risk::Preserve => 30,
        Risk::Balanced => 45,
        Risk::Press => 60,
    })
}

/// The share of its aircraft a fighter group keeps over the base, by risk
/// level (item 5).
pub(crate) fn base_share_of(risk: Risk) -> Fixed {
    match risk {
        Risk::Preserve => Fixed::from_ratio(3, 4),
        Risk::Balanced => Fixed::from_ratio(1, 2),
        Risk::Press => Fixed::from_ratio(1, 4),
    }
}

/// The raise of the share after a raid (item 6).
pub(crate) fn raise() -> Fixed {
    Fixed::from_ratio(1, 4)
}

/// The days before today on which a raid raises the share (item 6).
pub(crate) const RAID_DAYS: u32 = 3;

// ---------------------------------------------------------------------------
// The search arc (item 9).
// ---------------------------------------------------------------------------

/// The arc the threats give: the narrow arc that holds them all, and the
/// arc widened by 30 degrees at each end that is ordered.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct Arc {
    /// The narrowest arc on the ten-degree grid, `None` where there is no
    /// threat or none is narrower than the whole circle.
    pub(crate) narrow: Option<(u32, u32)>,
    /// The bearings ordered; equal means the whole circle.
    pub(crate) from: u32,
    pub(crate) to: u32,
}

impl Arc {
    const WHOLE: Arc = Arc {
        narrow: None,
        from: 0,
        to: 0,
    };

    pub(crate) fn whole(&self) -> bool {
        self.from == self.to
    }
}

/// A bearing as three digits.
pub(crate) fn bearing(deg: u32) -> String {
    format!("{deg:03}")
}

/// The arc of decision 0115, item 9, from the base's hex: the narrowest arc
/// of ten-degree steps that holds every threat by the engine's own test, the
/// lowest start where two are as narrow, widened by 30 degrees at each end,
/// and the whole circle where that comes to 360 or more. With no threat, the
/// whole circle.
pub(crate) fn arc_for(base: Hex, threats: &[Hex]) -> Arc {
    if threats.is_empty() {
        return Arc::WHOLE;
    }
    for width in (10..=350u32).step_by(10) {
        for start in (0..360u32).step_by(10) {
            let end = (start + width) % 360;
            if threats.iter().all(|hex| in_arc(base, *hex, start, end)) {
                if width + 60 >= 360 {
                    return Arc {
                        narrow: Some((start, end)),
                        from: 0,
                        to: 0,
                    };
                }
                return Arc {
                    narrow: Some((start, end)),
                    from: (start + 330) % 360,
                    to: (end + 30) % 360,
                };
            }
        }
    }
    Arc::WHOLE
}

pub(crate) fn arc_words(arc: &Arc) -> String {
    if arc.whole() {
        "the whole circle".to_string()
    } else {
        format!("{} to {}", bearing(arc.from), bearing(arc.to))
    }
}

/// The arc and how it was found, in words.
pub(crate) fn arc_working(arc: &Arc, threats: &[Threat]) -> String {
    if threats.is_empty() {
        return "no enemy base on the map and no contact: the arc is the whole circle".to_string();
    }
    let list = threats
        .iter()
        .map(|t| t.words.clone())
        .collect::<Vec<_>>()
        .join(", ");
    match arc.narrow {
        Some((from, to)) if !arc.whole() => format!(
            "threats: {list}; the narrowest arc that holds them is {} to {}, widened by 30 degrees at each end to {} to {}",
            bearing(from),
            bearing(to),
            bearing(arc.from),
            bearing(arc.to)
        ),
        Some((from, to)) => format!(
            "threats: {list}; the narrowest arc that holds them is {} to {}, which widened by 30 degrees at each end is the whole circle",
            bearing(from),
            bearing(to)
        ),
        None => format!("threats: {list}; no arc of under 360 degrees holds them: the whole circle"),
    }
}

/// One threat the arc points at: the hex of an enemy base on the side's map
/// or of a contact it holds.
#[derive(Clone, Debug)]
pub(crate) struct Threat {
    pub(crate) hex: Hex,
    pub(crate) words: String,
}

// ---------------------------------------------------------------------------
// The rest of the fighters (item 8).
// ---------------------------------------------------------------------------

/// What the rest rule says of one fighter group.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Verdict {
    /// At or below the level: flies the patrol.
    Flies,
    /// Above the level but the last fighter group that can fly: kept.
    Kept,
    /// Above the level and has no aircraft able to fly: rests.
    RestsUnable,
    /// Above the level, and another fighter group can fly the patrol: rests.
    RestsSpared,
}

impl Verdict {
    pub(crate) fn rests(self) -> bool {
        matches!(self, Verdict::RestsUnable | Verdict::RestsSpared)
    }
}

/// The rest rule over fighter groups given as (fatigue, aircraft able to
/// fly), already in the order they are taken: most tired first, then by
/// number. A fighter above the level rests if it cannot fly or if more than
/// one fighter that can fly is still not rested; otherwise it flies. So the
/// last fighter group that can fly is never rested.
pub(crate) fn fighter_verdicts(fighters: &[(Fixed, u32)], level: Fixed) -> Vec<Verdict> {
    let mut flying = fighters.iter().filter(|(_, able)| *able > 0).count();
    fighters
        .iter()
        .map(|(fatigue, able)| {
            if *fatigue <= level {
                Verdict::Flies
            } else if *able == 0 {
                Verdict::RestsUnable
            } else if flying > 1 {
                flying -= 1;
                Verdict::RestsSpared
            } else {
                Verdict::Kept
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// The groups.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Kind {
    Fighter,
    Searcher,
    Other,
}

/// An air group of the side as the doctrine reads it this morning.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Member {
    pub(crate) id: AirGroupId,
    pub(crate) kind: Kind,
    pub(crate) fatigue: Fixed,
    pub(crate) able: u32,
    pub(crate) mission: AirMission,
    pub(crate) patrol_share: Fixed,
    search: Option<SearchArc>,
    arc_order: Option<sim_core::ids::OrderId>,
    range: Fixed,
    /// Its setting to search once a day (decision 0131, 1).
    pub(crate) search_once: bool,
}

pub(crate) fn mission_words(mission: AirMission) -> String {
    match mission {
        AirMission::Rest => "rest".to_string(),
        AirMission::Search => "search".to_string(),
        AirMission::NavalStrike => "naval strike".to_string(),
        AirMission::CombatAirPatrol => "combat air patrol".to_string(),
        AirMission::BaseStrike(base) => format!("strike {base}"),
    }
}

fn kind_words(kind: Kind) -> &'static str {
    match kind {
        Kind::Fighter => "fighter group",
        Kind::Searcher => "search group",
        Kind::Other => "group that is neither a fighter nor can search",
    }
}

fn setting_words(m: &Member) -> String {
    match m.search {
        Some(arc) if arc.from_deg == arc.to_deg => {
            format!("search arc the whole circle at {} nm", arc.range_nm)
        }
        Some(arc) => format!(
            "search arc {} to {} at {} nm",
            bearing(arc.from_deg),
            bearing(arc.to_deg),
            arc.range_nm
        ),
        None => "no search arc".to_string(),
    }
}

// ---------------------------------------------------------------------------
// The day.
// ---------------------------------------------------------------------------

/// The morning's work in the making.
pub(crate) struct Day<'v, 'a> {
    pub(crate) view: &'v SideView<'a>,
    /// The command state as planned: the plans the leaves approve this
    /// morning and the intents formed this morning are in it.
    pub(crate) state: CommandState,
    pub(crate) today: u32,
    /// For a proposal, the root whose tree is asked for.
    scope: Option<IntentId>,
    /// What is handed in for the tree in scope (all of it, with no scope).
    real: StaffDecisions,
    /// What is handed in for the rest, thrown away.
    scratch: StaffDecisions,
    /// The groups cover intents served earlier this morning have taken.
    taken: BTreeMap<AirGroupId, IntentId>,
    /// The groups raid intents served earlier this morning have taken
    /// (decision 0120, 16: rank decides between two tasks of one kind).
    pub(crate) raid_taken: BTreeMap<AirGroupId, IntentId>,
    /// The groups strike-ships tasks served earlier this morning have taken
    /// (decision 0123, 7).
    pub(crate) strike_taken: BTreeMap<AirGroupId, IntentId>,
    /// The fighter groups the covers served this morning have, with the
    /// patrol share each is to fly at, or `None` where the cover rests it:
    /// what a raid from the same base may draw escorts from (decision 0120,
    /// 10 and 16).
    pub(crate) cover_fighters: BTreeMap<AirGroupId, Option<Fixed>>,
    /// The bomber each base's cover or search task borrows to search this
    /// morning (decision 0131, 8), worked out before the cover is served.
    pub(crate) turns: BTreeMap<BaseId, crate::search::Turn>,
    /// What the tree in scope would do, in words, for a proposal.
    pub(crate) lines: Vec<String>,
    /// The side's events, read when a reason needs them.
    events: Option<Vec<sim_core::side_events::SideEvent>>,
}

/// The kinds the air doctrine serves: hold, cover, neutralise, raid, deny,
/// strike ships and search waters, and the tasks of a task force, standing
/// at a hex by night and operating a carrier force (decisions 0124 and 0125),
/// which run in the same morning.
pub(crate) fn is_air(kind: IntentKind) -> bool {
    matches!(
        kind,
        IntentKind::Hold { .. }
            | IntentKind::Cover { .. }
            | IntentKind::Neutralise { .. }
            | IntentKind::Raid { .. }
            | IntentKind::Deny { .. }
            | IntentKind::StrikeShips { .. }
            | IntentKind::SearchWaters { .. }
            | IntentKind::StandAt { .. }
            | IntentKind::OperateCarriers { .. }
    )
}

/// The staff's air decisions for the morning.
pub(crate) fn decide(view: &SideView<'_>) -> StaffDecisions {
    match Day::run(view, None) {
        Some(day) => {
            let mut decisions = day.real;
            fill_decisions(view, &day.state, &mut decisions);
            decisions
        }
        None => StaffDecisions::default(),
    }
}

/// What the air doctrine would hand in this morning for the tree of `intent`
/// (a hold or a cover intent), as if its plan were released, stored nowhere.
pub(crate) fn propose(view: &SideView<'_>, intent: IntentId) -> Option<Proposal> {
    let state = view.command_state();
    let found = state
        .intents
        .get(&intent)
        .filter(|i| i.in_force() && is_air(i.kind))?;
    let plan = state.plans.get(&found.plan)?;
    let root = state.root_of(intent);
    let day = Day::run(view, Some(root))?;
    let ended = plan.state.ended();
    let worked = if ended {
        format!(
            "{} is {}, so the staff would hand in nothing for the tree of {}.",
            plan_for(view, found, plan),
            plan.state.name(),
            found.id
        )
    } else if day.lines.is_empty() {
        format!(
            "Taking {} as released, the staff would form no intent and give no order for the tree of {}.",
            plan_for(view, found, plan),
            found.id
        )
    } else {
        format!(
            "Taking {} as released, the staff would do this for the tree of {}: {}",
            plan_for(view, found, plan),
            found.id,
            day.lines.join(" ")
        )
    };
    let mut reason = serving(found, plan)
        .evidence(
            format!(
                "the plan is {}; the proposal takes it as released",
                plan.state.name()
            ),
            day.today,
        )
        .worked(worked)
        .would_change("a different risk level, forces limit or hold")
        .would_change("a group at the base, or one that can fly")
        .done();
    let mut decisions = day.real;
    fill_decisions(view, &day.state, &mut decisions);
    fill_reason(view, &day.state, &mut reason);
    Some(Proposal {
        intent: found.id,
        plan: plan.id,
        plan_state: plan.state,
        projection: None,
        decisions,
        runs: Vec::new(),
        shortfall: None,
        displaces: Vec::new(),
        reason,
    })
}

impl<'v, 'a> Day<'v, 'a> {
    /// Runs the morning, or `None` for a side with no hold or cover intent
    /// in force (it hands in nothing). With a `scope` (a proposal) the root
    /// of the tree asked for is taken as released and only what is handed in
    /// for its tree is kept.
    fn run(view: &'v SideView<'a>, scope: Option<IntentId>) -> Option<Day<'v, 'a>> {
        let state = view.command_state();
        // A keep-supplied intent whose forces limit names task forces may
        // form covering forces' stand tasks (decision 0126, 7).
        let covers_a_run = |i: &Intent| {
            matches!(i.kind, IntentKind::KeepSupplied { .. })
                && i.limits
                    .forces
                    .as_ref()
                    .is_some_and(|f| !f.task_forces.is_empty())
        };
        if !state
            .intents_in_force()
            .any(|i| is_air(i.kind) || covers_a_run(i))
        {
            return None;
        }
        let mut day = Day {
            view,
            state: state.clone(),
            today: view.day() + 1,
            scope,
            real: StaffDecisions::default(),
            scratch: StaffDecisions::default(),
            taken: BTreeMap::new(),
            raid_taken: BTreeMap::new(),
            strike_taken: BTreeMap::new(),
            cover_fighters: BTreeMap::new(),
            turns: BTreeMap::new(),
            lines: Vec::new(),
            events: None,
        };
        day.leaves();
        day.holds();
        day.neutralises();
        day.denials();
        day.work_turns();
        day.covers();
        day.raids();
        day.strikes();
        day.searches();
        day.supply_covers();
        day.stands();
        day.carrier_forces();
        Some(day)
    }

    /// The side's events, read once a morning.
    pub(crate) fn side_events(&mut self) -> &[sim_core::side_events::SideEvent] {
        if self.events.is_none() {
            self.events = Some(self.view.events());
        }
        self.events.as_deref().unwrap_or(&[])
    }

    pub(crate) fn in_scope(&self, intent: IntentId) -> bool {
        self.scope
            .is_none_or(|root| self.state.root_of(intent) == root)
    }

    pub(crate) fn target(&mut self, in_scope: bool) -> &mut StaffDecisions {
        if in_scope {
            &mut self.real
        } else {
            &mut self.scratch
        }
    }

    /// The intents of the planned state that pass `keep`, in priority order:
    /// ranked first, then unranked, each by number (decision 0107, ruling
    /// 10, with the rank read up the tree).
    pub(crate) fn in_priority_order(&self, keep: impl Fn(&Intent) -> bool) -> Vec<IntentId> {
        let mut found: Vec<IntentId> = self
            .state
            .intents_in_force()
            .filter(|intent| keep(intent))
            .map(|intent| intent.id)
            .collect();
        found.sort_by_key(|id| {
            let rank = self.state.priority_read_up(*id);
            (rank.is_none(), rank.unwrap_or(0), *id)
        });
        found
    }

    pub(crate) fn plan_of(&self, intent: &Intent) -> Option<Plan> {
        self.state.plans.get(&intent.plan).cloned()
    }

    pub(crate) fn released(&self, intent: &Intent) -> bool {
        self.state
            .plans
            .get(&intent.plan)
            .is_some_and(|plan| plan.state == PlanState::Released)
    }

    // -- 1. The leaves ------------------------------------------------------

    fn leaves(&mut self) {
        let roots = self.in_priority_order(|i| is_air(i.kind) && i.parent.is_none());
        for id in roots {
            let Some(intent) = self.state.intents.get(&id).cloned() else {
                continue;
            };
            let Some(plan) = self.plan_of(&intent) else {
                continue;
            };
            let in_scope = self.in_scope(id);
            let today = self.today;
            let mut now = plan.state;
            if now.ended() {
                let reason = serving(&intent, &plan)
                    .evidence(format!("{} is {}", plan.id, now.name()), today)
                    .binding(format!("{} is {}", plan.id, now.name()))
                    .worked(format!(
                        "{} is {}, so the staff forms nothing and gives no order under {}.",
                        plan_for(self.view, &intent, &plan),
                        now.name(),
                        intent.id
                    ))
                    .would_change("a new plan for the intent")
                    .done();
                self.target(in_scope).no_orders.push(reason);
                continue;
            }
            if self.scope == Some(id) {
                // A proposal takes the plan as released.
                self.set_plan(plan.id, PlanState::Released);
                continue;
            }
            let leaves = self.state.leaves;
            if now == PlanState::Draft {
                if !leaves.prepare {
                    let reason = serving(&intent, &plan)
                        .evidence(format!("{} is a draft", plan.id), today)
                        .binding("leave to prepare is withheld")
                        .worked(format!(
                            "{} is a draft and the commander has not given leave to prepare, so it waits for him.",
                            plan_for(self.view, &intent, &plan)
                        ))
                        .would_change("leave to prepare given, or the commander approving the plan")
                        .done();
                    self.waits(
                        in_scope,
                        reason,
                        NoticeKind::WaitsForLeave(Leave::Prepare),
                        plan.id,
                    );
                    continue;
                }
                let reason = serving(&intent, &plan)
                    .evidence(format!("{} is a draft", plan.id), today)
                    .evidence("leave to prepare is given", today)
                    .worked(format!(
                        "{} is a draft inside the limits of {} and leave to prepare is given: the staff makes it ready (preparing).",
                        plan_for(self.view, &intent, &plan),
                        intent.id
                    ))
                    .would_change("leave to prepare withheld")
                    .done();
                self.target(in_scope).commands.push(StaffCommand {
                    kind: CommandKind::Approve { plan: plan.id },
                    reason,
                });
                now = PlanState::Preparing;
                self.set_plan(plan.id, now);
            }
            if now == PlanState::Preparing {
                if !leaves.launch {
                    let reason = serving(&intent, &plan)
                        .evidence(format!("{} is preparing", plan.id), today)
                        .binding("leave to launch is withheld")
                        .worked(format!(
                            "{} is ready and the commander has not given leave to launch, so it waits for him.",
                            plan_for(self.view, &intent, &plan)
                        ))
                        .would_change("leave to launch given, or the commander releasing the plan")
                        .done();
                    self.waits(
                        in_scope,
                        reason,
                        NoticeKind::WaitsForLeave(Leave::Launch),
                        plan.id,
                    );
                    continue;
                }
                let reason = serving(&intent, &plan)
                    .evidence(format!("{} is preparing", plan.id), today)
                    .evidence("leave to launch is given", today)
                    .worked(format!(
                        "{} is ready and leave to launch is given: the staff releases it.",
                        plan_for(self.view, &intent, &plan)
                    ))
                    .would_change("leave to launch withheld")
                    .done();
                self.target(in_scope).commands.push(StaffCommand {
                    kind: CommandKind::Approve { plan: plan.id },
                    reason,
                });
                self.set_plan(plan.id, PlanState::Released);
            }
        }
    }

    fn set_plan(&mut self, plan: sim_core::command::PlanId, state: PlanState) {
        if let Some(found) = self.state.plans.get_mut(&plan) {
            found.state = state;
        }
    }

    /// A no-order reason and the notice that the plan waits for a leave.
    fn waits(
        &mut self,
        in_scope: bool,
        reason: Reason,
        kind: NoticeKind,
        plan: sim_core::command::PlanId,
    ) {
        let out = self.target(in_scope);
        out.no_orders.push(reason);
        let index = out.no_orders.len() - 1;
        out.notices.push(StaffNotice {
            kind,
            plan,
            run: None,
            rests_on: NoticeBasis::NoOrder(index),
        });
    }

    // -- 2. Hold a place ----------------------------------------------------

    /// The air groups the intent may use, at its base or not (decision 0117,
    /// 7 and 8): those named in every forces limit on its chain (the
    /// narrowest binds, the architect's ruling after the build), else, with
    /// none, the side's groups at the base it is addressed to.
    pub(crate) fn may_use(&self, intent: &Intent) -> Vec<AirGroupId> {
        match self.state.groups_on_chain(intent.id) {
            Some(allowed) => allowed.into_iter().collect(),
            None => match intent.addressee {
                Addressee::Base(base) => self
                    .view
                    .air_groups()
                    .filter(|group| group.location == AirGroupLocation::Base(base))
                    .map(|group| group.id)
                    .collect(),
                _ => Vec::new(),
            },
        }
    }

    pub(crate) fn kind_of(&self, group: &sim_core::state::AirGroup) -> Kind {
        match self.view.aircraft_type(group.aircraft_type) {
            Some(t) if t.role == AircraftRole::Fighter => Kind::Fighter,
            Some(t) if t.search_range_nm > Fixed::ZERO => Kind::Searcher,
            _ => Kind::Other,
        }
    }

    /// The groups the intent may use that are at `base`, in group number
    /// order: held or not.
    pub(crate) fn at_base(&self, intent: &Intent, base: BaseId) -> Vec<Member> {
        let may = self.may_use(intent);
        let mut members: Vec<Member> = self
            .view
            .air_groups()
            .filter(|group| {
                may.contains(&group.id) && group.location == AirGroupLocation::Base(base)
            })
            .map(|group| self.member(group))
            .collect();
        members.sort_by_key(|m| m.id);
        members
    }

    /// A group of the side as the doctrine reads it this morning.
    pub(crate) fn member(&self, group: &sim_core::state::AirGroup) -> Member {
        Member {
            id: group.id,
            kind: self.kind_of(group),
            fatigue: group.fatigue,
            able: self.view.able_to_fly(group.id).unwrap_or(0),
            mission: group.mission,
            patrol_share: group.patrol_share,
            search: group.search,
            arc_order: group.arc_order,
            search_once: group.search_once,
            range: self
                .view
                .aircraft_type(group.aircraft_type)
                .map_or(Fixed::ZERO, |t| t.search_range_nm),
        }
    }

    fn holds(&mut self) {
        let holds = self.in_priority_order(|i| matches!(i.kind, IntentKind::Hold { .. }));
        for id in holds {
            let Some(hold) = self.state.intents.get(&id).cloned() else {
                continue;
            };
            if !self.released(&hold) {
                continue;
            }
            let Some(plan) = self.plan_of(&hold) else {
                continue;
            };
            let in_scope = self.in_scope(id);
            let today = self.today;
            let Some(base) = hold.kind.own_base() else {
                continue;
            };
            // The tasks of the forces its forces limit names (decisions
            // 0124, 8, and 0125, 2): cover from the sea, and carriers.
            if let Some(hex) = self.view.bases().find(|b| b.id == base).map(|b| b.hex) {
                self.form_force_tasks(&hold, &plan, hex, in_scope, false);
            }
            let base_text = base_words(self.view, base);
            // (a) A cover intent for the base stands, whoever made it.
            let standing = self
                .state
                .intents_in_force()
                .find(|c| matches!(c.kind, IntentKind::Cover { base: b } if b == base))
                .map(|c| (c.id, c.maker));
            if let Some((cover, maker)) = standing {
                let by = match maker {
                    Maker::Commander => "the commander's",
                    Maker::Staff => "the staff's own",
                };
                let reason = serving(&hold, &plan)
                    .evidence(format!("{cover}, {by} cover of {base_text} from the air, is in force"), today)
                    .binding(format!("{cover} stands"))
                    .worked(format!(
                        "{} is held by {cover}, which covers {base_text} from the air and is {by}: the staff forms no other.",
                        hold.id
                    ))
                    .would_change(format!("{cover} withdrawn"))
                    .done();
                self.target(in_scope).no_orders.push(reason);
                continue;
            }
            // (b) Something at the base to cover it with: a fighter group or
            // a group that can search, among those the intent may use.
            let usable = self
                .at_base(&hold, base)
                .into_iter()
                .filter(|m| m.kind != Kind::Other)
                .count();
            if usable == 0 {
                let reason = serving(&hold, &plan)
                    .evidence(
                        format!("{} may use no fighter group and no group that can search at {base_text}", hold.id),
                        today,
                    )
                    .binding("no air group at the base that a cover could use")
                    .worked(format!(
                        "There is nothing at {base_text} to cover it with: {} may use no fighter group and no group that can search there, so the staff forms no cover from the air.",
                        hold.id
                    ))
                    .would_change("a fighter group or a search group at the base, or in the forces limit")
                    .done();
                self.target(in_scope).no_orders.push(reason);
                continue;
            }
            // (c) Form cover from the air under it.
            let id_new = self.state.next_intent_number();
            self.state.intents.insert(
                id_new,
                Intent {
                    id: id_new,
                    kind: IntentKind::Cover { base },
                    parent: Some(hold.id),
                    maker: Maker::Staff,
                    addressee: IntentKind::Cover { base }.addressee(),
                    limits: IntentLimits::default(),
                    set_day: today,
                    plan: hold.plan,
                    withdrawn_day: None,
                },
            );
            let reason = serving(&hold, &plan)
                .evidence(
                    format!("no cover intent for {base_text} stands; {usable} group(s) at the base can fly a patrol or search"),
                    today,
                )
                .binding(format!("{} is held and its plan is released", hold.id))
                .worked(format!(
                    "{} holds {base_text}, its plan is released and no cover of it stands: the staff forms {id_new}, cover of {base_text} from the air, under it.",
                    hold.id
                ))
                .would_change("a cover intent already standing for the base")
                .would_change("no group at the base that can fly a patrol or search")
                .done();
            self.target(in_scope).commands.push(StaffCommand {
                kind: CommandKind::FormIntent {
                    parent: hold.id,
                    kind: IntentKind::Cover { base },
                },
                reason,
            });
            if in_scope {
                self.lines.push(format!(
                    "{} holds {base_text}: it would form {id_new}, cover of {base_text} from the air, under it.",
                    hold.id
                ));
            }
        }
    }

    // -- 3. Cover a base from the air ---------------------------------------

    /// The share of the patrol and what raised it: the latest day before
    /// today, in the three days before, on which the base was raided.
    fn share_for(&self, base: BaseId, risk: Risk) -> (Fixed, Option<u32>) {
        let first = self.today.saturating_sub(RAID_DAYS);
        let raid = self
            .view
            .air_raids_on(base)
            .iter()
            .map(|raid| raid.day)
            .filter(|day| *day >= first && *day < self.today)
            .max();
        let base_share = base_share_of(risk);
        match raid {
            Some(day) => (std::cmp::min(Fixed::ONE, base_share + raise()), Some(day)),
            None => (base_share, None),
        }
    }

    pub(crate) fn threats(&self) -> Vec<Threat> {
        let mut found: Vec<Threat> = self
            .view
            .enemy_bases()
            .map(|b| Threat {
                hex: b.hex,
                words: format!("{} at {}", base_words(self.view, b.id), b.hex),
            })
            .collect();
        found.extend(self.view.contacts().map(|c| Threat {
            hex: c.estimated_hex,
            words: format!(
                "contact {} at {}",
                self.view.contact_number(c.id),
                c.estimated_hex
            ),
        }));
        found
    }

    /// The contacts the side held when the group's arc was last set and has
    /// since lost, each with the day it was dropped: from the side's own
    /// events, the contacts dropped on or after the day of the group's arc
    /// order that it no longer holds.
    fn gone_since_arc(&mut self, m: &Member) -> Vec<(u32, u32)> {
        let Some(order) = m.arc_order else {
            return Vec::new();
        };
        let Some(set_day) = self
            .view
            .orders()
            .find(|o| o.id == order)
            .map(|o| o.day_issued)
        else {
            return Vec::new();
        };
        if self.events.is_none() {
            self.events = Some(self.view.events());
        }
        let mut gone: Vec<(u32, u32)> = Vec::new();
        for event in self.events.iter().flatten() {
            if event.day < set_day {
                continue;
            }
            if let SideEventKind::Whole(EventKind::ContactDropped { contact, .. }) = &event.kind {
                let number = self.view.contact_number(*contact);
                if self.view.contact_by_number(number).is_none()
                    && !gone.iter().any(|(n, _)| *n == number)
                {
                    gone.push((number, event.day));
                }
            }
        }
        gone.sort_unstable();
        gone
    }

    fn covers(&mut self) {
        let covers = self.in_priority_order(|i| matches!(i.kind, IntentKind::Cover { .. }));
        for id in covers {
            let Some(cover) = self.state.intents.get(&id).cloned() else {
                continue;
            };
            if !self.released(&cover) {
                continue;
            }
            self.serve_cover(&cover);
        }
    }

    fn serve_cover(&mut self, cover: &Intent) {
        let Some(plan) = self.plan_of(cover) else {
            return;
        };
        let in_scope = self.in_scope(cover.id);
        let today = self.today;
        let Some(base) = cover.kind.own_base() else {
            return;
        };
        let base_text = base_words(self.view, base);
        let risk = self.state.risk_read_up(cover.id);
        let level = level_of(risk);
        let (share, raised_by) = self.share_for(base, risk);
        let share_note = match raised_by {
            Some(day) => format!(
                "{} at {} raised by {} because {base_text} was attacked from the air on day {day}",
                base_share_of(risk),
                risk.name(),
                raise()
            ),
            None => format!(
                "{} at {}, the base not attacked from the air on the three days before",
                base_share_of(risk),
                risk.name()
            ),
        };

        // The groups: at the base, not held, not taken by a cover served
        // before this one, in group number order.
        let mut members: Vec<Member> = self.at_base(cover, base);
        // A group a raid task in force may use as a bomber is the raid's,
        // though it can also search (decision 0120, 16, amending decision
        // 0115, 7): the cover leaves it out.
        let mut raid_bombers = self.raid_bombers();
        // So is a group a strike-ships task in force may use as a bomber
        // (decision 0123, 7, as decision 0120, 16).
        for (group, task) in self.strike_bombers() {
            raid_bombers.entry(group).or_insert(task);
        }
        // Save the one that task lends the cover to search today (decision
        // 0131, 8).
        let turn = self
            .turns
            .get(&base)
            .filter(|t| t.borrower == cover.id)
            .cloned();
        if let Some(turn) = &turn {
            raid_bombers.remove(&turn.group);
            // Lent, it is the cover's today whatever the cover's limit says.
            if !members.iter().any(|m| m.id == turn.group) {
                if let Some(group) = self.view.air_groups().find(|g| g.id == turn.group) {
                    members.push(self.member(group));
                    members.sort_by_key(|m| m.id);
                }
            }
        }
        let raids_have: Vec<String> = members
            .iter()
            .filter_map(|m| {
                raid_bombers.get(&m.id).map(|raid| {
                    format!(
                        "{} is the {}'s: {raid} may use it as a bomber (fatigue {}), so the cover gives it no order",
                        m.id,
                        if self.state.intents.get(raid).is_some_and(|i| matches!(i.kind, IntentKind::StrikeShips { .. })) {
                            "strike task"
                        } else {
                            "raid"
                        },
                        m.fatigue
                    )
                })
            })
            .collect();
        members.retain(|m| !raid_bombers.contains_key(&m.id));
        members.retain(|m| {
            self.state.hold_on_group(m.id).is_none()
                && self.taken.get(&m.id).is_none_or(|by| *by == cover.id)
        });
        for m in &members {
            self.taken.insert(m.id, cover.id);
        }

        // The arc for the groups that search. The waters of a deny intent its
        // searchers reach are threats beside the rest: one arc for a base's
        // searchers (decision 0123, 13).
        let mut threats = self.threats();
        let base_hex = self.view.bases().find(|b| b.id == base).map(|b| b.hex);
        if let Some(at) = base_hex {
            let ranges: Vec<Fixed> = members
                .iter()
                .filter(|m| m.kind == Kind::Searcher)
                .map(|m| m.range)
                .collect();
            for hex in self.waters_for_cover(at, &ranges) {
                threats.push(Threat {
                    hex,
                    words: format!("the waters to deny at {hex}"),
                });
            }
        }
        let hexes: Vec<Hex> = threats.iter().map(|t| t.hex).collect();
        let arc = base_hex.map_or(Arc::WHOLE, |hex| arc_for(hex, &hexes));
        let arc_text = arc_working(&arc, &threats);

        let once = crate::search::once_a_day(risk);
        let once_text = crate::search::once_words(risk);

        // The fighters, most tired first.
        let mut fighters: Vec<Member> = members
            .iter()
            .filter(|m| m.kind == Kind::Fighter)
            .copied()
            .collect();
        fighters.sort_by_key(|m| (std::cmp::Reverse(m.fatigue), m.id));
        let tuples: Vec<(Fixed, u32)> = fighters.iter().map(|m| (m.fatigue, m.able)).collect();
        let verdicts: BTreeMap<AirGroupId, Verdict> = fighters
            .iter()
            .zip(fighter_verdicts(&tuples, level))
            .map(|(m, v)| (m.id, v))
            .collect();
        // What a raid from this base may draw escorts from (decision 0120,
        // 16): each fighter the cover has, with the share it is to fly at,
        // or none where it rests.
        for (group, verdict) in &verdicts {
            let flying = if verdict.rests() { None } else { Some(share) };
            self.cover_fighters.entry(*group).or_insert(flying);
        }

        let mut orders: Vec<StaffOrder> = Vec::new();
        let mut unordered: Vec<String> = raids_have.clone();
        let mut everyone: Vec<String> = raids_have.clone();
        let mut plan_lines: Vec<String> = raids_have;
        let risk_text = format!("{} risk", risk.name());
        let give = |orders: &mut Vec<StaffOrder>,
                    group: AirGroupId,
                    order: AirGroupOrder,
                    reason: Reason| {
            orders.push(StaffOrder {
                kind: OrderKind::AirGroup {
                    target: group,
                    order,
                },
                intent: cover.id,
                reason,
            });
        };

        for m in &members {
            let g = m.id;
            let numbers = match &turn {
                Some(turn) if turn.group == g => format!(
                    "{g}, a bomber group of {} that can search, has fatigue {} against the {level} of the {risk_text}; {}",
                    turn.lender,
                    m.fatigue,
                    turn.words
                ),
                _ => format!(
                    "{g}, a {}, has fatigue {} against the {level} of the {risk_text}",
                    kind_words(m.kind),
                    m.fatigue
                ),
            };
            let before = orders.len();
            let line;
            match m.kind {
                Kind::Fighter => {
                    let verdict = verdicts.get(&g).copied().unwrap_or(Verdict::Flies);
                    if verdict.rests() {
                        let why = match verdict {
                            Verdict::RestsUnable => {
                                "it is above the level and has no aircraft able to fly".to_string()
                            }
                            _ => {
                                "it is above the level and another fighter group can fly the patrol"
                                    .to_string()
                            }
                        };
                        if m.mission != AirMission::Rest {
                            give(
                                &mut orders,
                                g,
                                AirGroupOrder::SetAirMission(AirMission::Rest),
                                serving(cover, &plan)
                                    .evidence(format!("{numbers}; {why}"), today)
                                    .evidence(format!("its mission is {}", mission_words(m.mission)), today)
                                    .binding(format!("rest above fatigue {level} at {risk_text}"))
                                    .worked(format!(
                                        "{numbers}; {why}: it rests for a day, so its mission is set from {} to rest.",
                                        mission_words(m.mission)
                                    ))
                                    .would_change("a lower fatigue, or a bolder risk level")
                                    .done(),
                            );
                        }
                        line = format!(
                            "{numbers}; {why}: it rests (mission {})",
                            if m.mission == AirMission::Rest {
                                "rest already".to_string()
                            } else {
                                "set to rest".to_string()
                            }
                        );
                        plan_lines.push(format!("{g} rests (fatigue {} above {level})", m.fatigue));
                    } else {
                        let kept = if verdict == Verdict::Kept {
                            "; it is above the level but the last fighter group that can fly, so it is never rested"
                        } else {
                            "; it is not above the level, so it flies"
                        };
                        if m.mission != AirMission::CombatAirPatrol {
                            give(
                                &mut orders,
                                g,
                                AirGroupOrder::SetAirMission(AirMission::CombatAirPatrol),
                                serving(cover, &plan)
                                    .evidence(format!("{numbers}{kept}"), today)
                                    .evidence(format!("its mission is {}", mission_words(m.mission)), today)
                                    .binding(format!("rest above fatigue {level} at {risk_text}"))
                                    .worked(format!(
                                        "{numbers}{kept} the combat air patrol of {base_text}: its mission is set from {} to combat air patrol.",
                                        mission_words(m.mission)
                                    ))
                                    .would_change("a fatigue above the level with another fighter group able to fly")
                                    .done(),
                            );
                        }
                        if m.patrol_share != share {
                            give(
                                &mut orders,
                                g,
                                AirGroupOrder::SetPatrolShare { share },
                                serving(cover, &plan)
                                    .evidence(format!("{numbers}{kept}"), today)
                                    .evidence(format!("its patrol share is {}", m.patrol_share), today)
                                    .binding(format!("the patrol share of the {risk_text}: {share_note}"))
                                    .worked(format!(
                                        "The patrol share of {g} is set from {} to {share}: {share_note}.",
                                        m.patrol_share
                                    ))
                                    .would_change("a different risk level")
                                    .would_change("an air raid on the base, or none on the three days before")
                                    .done(),
                            );
                        }
                        line = format!(
                            "{numbers}{kept}; mission {}, patrol share {} (wanted {share}: {share_note})",
                            mission_words(m.mission),
                            m.patrol_share
                        );
                        plan_lines.push(format!(
                            "{g} flies the combat air patrol at share {share} (fatigue {} against {level})",
                            m.fatigue
                        ));
                    }
                }
                Kind::Searcher => {
                    if m.fatigue > level {
                        if m.mission != AirMission::Rest {
                            give(
                                &mut orders,
                                g,
                                AirGroupOrder::SetAirMission(AirMission::Rest),
                                serving(cover, &plan)
                                    .evidence(format!("{numbers}; it is above the level"), today)
                                    .evidence(format!("its mission is {}", mission_words(m.mission)), today)
                                    .binding(format!("rest above fatigue {level} at {risk_text}"))
                                    .worked(format!(
                                        "{numbers}; it is above the level and a search group is never kept: it rests for a day, so its mission is set from {} to rest.",
                                        mission_words(m.mission)
                                    ))
                                    .would_change("a lower fatigue, or a bolder risk level")
                                    .done(),
                            );
                        }
                        line = format!(
                            "{numbers}; it is above the level, so it rests (mission {})",
                            if m.mission == AirMission::Rest {
                                "rest already".to_string()
                            } else {
                                "set to rest".to_string()
                            }
                        );
                        plan_lines.push(format!("{g} rests (fatigue {} above {level})", m.fatigue));
                    } else {
                        if m.mission != AirMission::Search {
                            give(
                                &mut orders,
                                g,
                                AirGroupOrder::SetAirMission(AirMission::Search),
                                serving(cover, &plan)
                                    .evidence(format!("{numbers}; it is not above the level"), today)
                                    .evidence(format!("its mission is {}", mission_words(m.mission)), today)
                                    .binding(format!("rest above fatigue {level} at {risk_text}"))
                                    .worked(format!(
                                        "{numbers}; it is not above the level, so it searches: its mission is set from {} to search.",
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
                                serving(cover, &plan)
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
                                        "{numbers}; it searches, and {once_text}: its search once a day is set {}.",
                                        if once { "on" } else { "off" }
                                    ))
                                    .would_change("a different risk level")
                                    .done(),
                            );
                        }
                        let wanted = SearchArc {
                            from_deg: arc.from,
                            to_deg: arc.to,
                            range_nm: m.range,
                        };
                        if m.search != Some(wanted) {
                            let gone = self.gone_since_arc(m);
                            let mut reason = serving(cover, &plan)
                                .evidence(format!("{numbers}; it is not above the level"), today)
                                .evidence(format!("its arc is: {}", setting_words(m)), today)
                                .binding(format!(
                                    "the arc the threats give, at its type's range of {} nm",
                                    m.range
                                ));
                            for (number, day) in &gone {
                                reason = reason.evidence(
                                    format!(
                                        "contact {number}, held when its arc was set, was dropped on day {day} and is gone"
                                    ),
                                    today,
                                );
                            }
                            let gone_text = if gone.is_empty() {
                                String::new()
                            } else {
                                format!(
                                    "; {} held when its arc was set {} gone",
                                    gone.iter()
                                        .map(|(n, d)| format!("contact {n} (dropped on day {d})"))
                                        .collect::<Vec<_>>()
                                        .join(", "),
                                    if gone.len() == 1 { "is" } else { "are" }
                                )
                            };
                            let reason = reason
                                .worked(format!(
                                    "The search arc of {g} is set to {} out to {} nm: {arc_text}{gone_text}.",
                                    arc_words(&arc),
                                    m.range
                                ))
                                .would_change("a new contact, or a contact dropped")
                                .would_change("an enemy base on the side's map that was not, or the reverse")
                                .done();
                            give(
                                &mut orders,
                                g,
                                AirGroupOrder::SetSearchArc {
                                    from_deg: i32::try_from(arc.from).unwrap_or(0),
                                    to_deg: i32::try_from(arc.to).unwrap_or(0),
                                    range_nm: m.range,
                                },
                                reason,
                            );
                        }
                        line = format!(
                            "{numbers}; it is not above the level, so it searches; mission {}, {} ({arc_text}); {once_text}",
                            mission_words(m.mission),
                            setting_words(m)
                        );
                        plan_lines.push(format!(
                            "{g} searches {} out to {} nm ({arc_text})",
                            arc_words(&arc),
                            m.range
                        ));
                    }
                }
                Kind::Other => {
                    line = format!(
                        "{g} is neither a fighter group nor can it search (fatigue {}): no order, it is left for a strike intent",
                        m.fatigue
                    );
                    plan_lines.push(format!("{g} is left for a strike intent"));
                }
            }
            if orders.len() == before {
                unordered.push(line.clone());
            }
            everyone.push(line);
        }

        // Cannot meet (item 10): no fighter group of the intent can fly.
        let cannot_meet = !members
            .iter()
            .any(|m| m.kind == Kind::Fighter && m.able > 0);

        let intent_words = format!("{}, the cover of {base_text} from the air", cover.id);
        let mut notice_basis: Option<usize> = None;
        let out = self.target(in_scope);
        out.orders.extend(orders.iter().cloned());
        if !unordered.is_empty() || cannot_meet {
            let (lines, head) = if unordered.is_empty() {
                (everyone.clone(), "No order is given".to_string())
            } else {
                (
                    unordered.clone(),
                    "No order is given for these groups".to_string(),
                )
            };
            let why = if cannot_meet {
                let fighters = members.iter().filter(|m| m.kind == Kind::Fighter).count();
                if fighters == 0 {
                    format!(
                        " The staff cannot meet {}: it may use no fighter group at the base that is not held.",
                        cover.id
                    )
                } else {
                    format!(
                        " The staff cannot meet {}: no fighter group of it has an aircraft able to fly.",
                        cover.id
                    )
                }
            } else {
                String::new()
            };
            let body = if lines.is_empty() {
                "it has no group at the base that the staff may order".to_string()
            } else {
                lines.join("; ")
            };
            let mut builder = serving(cover, &plan)
                .evidence(
                    format!(
                        "{intent_words}, with {} group(s) at the base that the staff may order",
                        members.len()
                    ),
                    today,
                )
                .binding(format!("the patrol share of the {risk_text}: {share_note}"))
                .binding(format!("rest above fatigue {level} at {risk_text}"))
                .binding(format!("search arc: {arc_text}"));
            if cannot_meet {
                builder =
                    builder.binding("no fighter group of the intent has an aircraft able to fly");
            }
            let reason = builder
                .worked(format!(
                    "{head} for {intent_words} this morning: {body}. The share is {share} ({share_note}); the arc is {} ({arc_text}).{why}",
                    arc_words(&arc)
                ))
                .would_change("a group whose fatigue or mission differs from what the doctrine wants")
                .would_change("a strike intent for the groups left for it")
                .done();
            out.no_orders.push(reason);
            notice_basis = Some(out.no_orders.len() - 1);
        }
        if cannot_meet {
            if let Some(index) = notice_basis {
                out.notices.push(StaffNotice {
                    kind: NoticeKind::CannotMeet,
                    plan: plan.id,
                    run: None,
                    rests_on: NoticeBasis::NoOrder(index),
                });
            }
        }
        if in_scope {
            let groups = if plan_lines.is_empty() {
                "it has no group at the base that the staff may order".to_string()
            } else {
                plan_lines.join("; ")
            };
            self.lines.push(format!(
                "{intent_words}: {groups}. The arc is {} ({arc_text}).",
                arc_words(&arc)
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fx(n: i64) -> Fixed {
        Fixed::from_int(n)
    }

    #[test]
    fn the_arc_of_two_threats_and_of_one() {
        let base = Hex { q: 0, r: 0 };
        let both = arc_for(base, &[Hex { q: 5, r: 0 }, Hex { q: 0, r: 5 }]);
        assert_eq!(both.narrow, Some((90, 150)));
        assert_eq!((both.from, both.to), (60, 180));
        let one = arc_for(base, &[Hex { q: 5, r: 0 }]);
        assert_eq!(one.narrow, Some((80, 90)));
        assert_eq!((one.from, one.to), (50, 120));
    }

    #[test]
    fn no_threat_is_the_whole_circle() {
        let arc = arc_for(Hex { q: 0, r: 0 }, &[]);
        assert!(arc.whole());
        assert_eq!(arc.narrow, None);
    }

    #[test]
    fn a_wide_spread_is_the_whole_circle() {
        let base = Hex { q: 0, r: 0 };
        // Six hexes round the base: the gaps are 60 degrees, so the narrow
        // arc is 300 and widened it is 360.
        let arc = arc_for(
            base,
            &[
                Hex { q: 5, r: 0 },
                Hex { q: -5, r: 0 },
                Hex { q: 0, r: 5 },
                Hex { q: 0, r: -5 },
                Hex { q: 5, r: -5 },
                Hex { q: -5, r: 5 },
            ],
        );
        assert!(arc.whole());
        assert!(arc.narrow.is_some());
    }

    #[test]
    fn the_last_fighter_that_can_fly_is_not_rested() {
        let level = fx(45);
        // Day 9 of scenario 47: both at 48; most tired first, then by number.
        assert_eq!(
            fighter_verdicts(&[(fx(48), 6), (fx(48), 4)], level),
            vec![Verdict::RestsSpared, Verdict::Kept]
        );
        // Day 10: group 2 at 54, group 1 at 38.
        assert_eq!(
            fighter_verdicts(&[(fx(54), 4), (fx(38), 6)], level),
            vec![Verdict::RestsSpared, Verdict::Flies]
        );
        // Day 11: both at 44.
        assert_eq!(
            fighter_verdicts(&[(fx(44), 6), (fx(44), 4)], level),
            vec![Verdict::Flies, Verdict::Flies]
        );
        // Day 12: both at 50.
        assert_eq!(
            fighter_verdicts(&[(fx(50), 5), (fx(50), 4)], level),
            vec![Verdict::RestsSpared, Verdict::Kept]
        );
    }

    #[test]
    fn a_fighter_that_cannot_fly_rests_and_does_not_count() {
        let level = fx(45);
        assert_eq!(
            fighter_verdicts(&[(fx(60), 0), (fx(50), 3)], level),
            vec![Verdict::RestsUnable, Verdict::Kept]
        );
    }

    #[test]
    fn levels_and_shares() {
        assert_eq!(level_of(Risk::Preserve), fx(30));
        assert_eq!(level_of(Risk::Balanced), fx(45));
        assert_eq!(level_of(Risk::Press), fx(60));
        assert_eq!(
            base_share_of(Risk::Balanced) + raise(),
            Fixed::from_ratio(3, 4)
        );
        assert_eq!(base_share_of(Risk::Preserve) + raise(), Fixed::ONE);
    }
}
