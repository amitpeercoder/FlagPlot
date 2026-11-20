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
    CommandKind, ForcesLimit, HoldTarget, Intent, IntentKind, Leave, Limit, Maker, NoticeBasis,
    NoticeKind, Plan, PlanOutcome, PlanState, Risk, StaffCommand, StaffNotice, StaffOrder,
};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, EntityRef, IntentId, LandUnitId, ShipId, TaskForceId};
use sim_core::map::HexKind;
use sim_core::orders::{
    Cargo, CargoAction, LandUnitOrder, Mission, OrderKind, Posture, SpeedSetting, TaskForceOrder,
};
use sim_core::state::{LandUnit, ShipLocation, Skills, TaskForce};

use crate::air::Day;
use crate::planner::DEFAULT_INTERVAL_DAYS;
use crate::reasons::{hold_words, serving};
use crate::units::{ceil_days, days_word, floor_whole, tons, whole};

mod coming;

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

/// The days a take looks ahead for a day that brings the men it wants, where
/// the commander gave no latest day (decision 0159, 6). The staff's figure,
/// a placeholder.
pub(crate) const WAIT_HORIZON_DAYS: u32 = 7;

/// The first lift's choice (decision 0142, 6 to 10; decision 0149), or why
/// there is none.
pub(crate) enum FirstChoice {
    Found {
        choice: Choice,
        source: BaseId,
        source_hex: Hex,
        /// The limit the staff sets on the landing it forms.
        limit: ForcesLimit,
    },
    /// The commander named all three and no forces limit is found.
    Unnamed,
    /// The forces limit names no source of the side.
    NoSource,
    /// No transport force or no unit (evidence, working).
    NoChoice(String, String),
    /// The staff can assemble no force (decision 0149, 13).
    NoAssembly(crate::assemble::NoSource),
}

/// What a take's first lift short of the men wanted does instead of sailing
/// short (decision 0159, 6 and 7).
pub(crate) enum Waiting {
    /// Item 6: a later day in reach brings the men wanted; the staff waits
    /// for it.
    For {
        day: u32,
        wanted: u32,
        have: u32,
        /// The men that day's lift gives.
        then: u32,
        /// The units and forces that may be used by that day and not today,
        /// each with its day, in words.
        coming: Vec<String>,
        /// Today's choice, in words.
        words: String,
        /// That day's lift: its force, units and source, what the plan
        /// order works the day ashore from (decision 0159, 15).
        lift: Lift,
    },
    /// Item 7: no day in reach brings them; at balanced the staff hands in
    /// the notice that it cannot bring enough and waits for the commander.
    NoDay {
        wanted: u32,
        have: u32,
        /// The units and forces named that may be used later, each with its
        /// day and why it does not serve, in words.
        coming: Vec<String>,
        /// The days looked at, in words.
        reach: String,
        words: String,
    },
    /// Decision 0169, 5: a lift short of the men wanted, with too few in
    /// reach, whose men cannot be fed: at balanced it does not go.
    Starve {
        starve: Starve,
        /// What the staff read before the count, in words.
        words: String,
    },
}

/// Decision 0169, 2 to 4: the count of a lift short of the men wanted with
/// too few in reach, and whether its men can be fed where they wait.
#[derive(Clone, Debug)]
pub(crate) struct Starve {
    pub(crate) force: TaskForceId,
    pub(crate) wanted: u32,
    /// The fit men of the operation ashore at the place that can attack.
    pub(crate) ashore: u32,
    /// The fighting men the lift carries.
    pub(crate) lift: u32,
    /// The men in reach by `last`.
    pub(crate) reach: u32,
    pub(crate) last: u32,
    /// The words of the count and of the men in reach.
    pub(crate) count: String,
    /// Item 4 in words.
    pub(crate) fed_words: String,
    pub(crate) fed: bool,
}

impl Starve {
    /// The men to hand: those ashore that can attack and those of the lift.
    pub(crate) fn have(&self) -> u32 {
        self.ashore + self.lift
    }

    /// The words of a lift that goes short because its men can be fed
    /// (item 6).
    pub(crate) fn goes_words(&self) -> String {
        format!(
            " {} {} So it goes short: its men can be fed where they wait.",
            self.count, self.fed_words
        )
    }
}

/// A lift as a choice gives it: the transport force, the units and the
/// source.
#[derive(Clone, Debug)]
pub(crate) struct Lift {
    pub(crate) force: TaskForceId,
    pub(crate) units: Vec<LandUnitId>,
    pub(crate) source: BaseId,
    pub(crate) source_hex: Hex,
}

/// What a take waits for in men this morning (decision 0159, rule B), kept
/// for the plan order: the men wanted, the men to hand, and the day that
/// brings them with its lift, or `None` where no day in reach does.
#[derive(Clone, Debug)]
pub(crate) struct MenWait {
    pub(crate) wanted: u32,
    pub(crate) have: u32,
    pub(crate) then: Option<(u32, Lift)>,
}

/// The men the staff wants for each man of the estimate, by risk (item 10).
/// The staff's figures, placeholders.
pub(crate) fn men_per_man(risk: Risk) -> u32 {
    match risk {
        Risk::Preserve => 3,
        Risk::Balanced => 2,
        Risk::Press => 1,
    }
}

/// The men the staff wants against the side's estimate of the enemy at
/// `place`, at `risk` (item 10); `None` where it holds no estimate.
pub(crate) fn men_wanted(
    view: &sim_core::view::SideView<'_>,
    place: Hex,
    risk: Risk,
) -> Option<u32> {
    view.ground_estimate(place)
        .map(|e| e.men * men_per_man(risk))
}

/// Whether the force has been routed to the place: its route acts there, or
/// it is landing there (item 20, read from the force alone).
pub(crate) fn routed(force: &TaskForce, place: Hex) -> bool {
    force.route.act_at.contains(&place) || force.landing_at == Some(place)
}

pub(crate) fn unit_words(unit: &LandUnit) -> String {
    format!("{} ({})", unit.id, unit.name)
}

/// A unit chosen to land, with the ships it takes.
#[derive(Clone, Debug)]
pub(crate) struct Pick {
    pub(crate) id: LandUnitId,
    aboard: bool,
    ships: Vec<ShipId>,
}

/// The choice of item 6 to 10.
#[derive(Clone)]
pub(crate) struct Choice {
    pub(crate) force: TaskForceId,
    pub(crate) picks: Vec<Pick>,
    /// Item 27, in the order a staff works in.
    pub(crate) lines: Vec<String>,
    /// The men wanted, where an estimate is held, and whether the men
    /// chosen fall short of them.
    pub(crate) short: Option<(u32, u32)>,
    /// The men chosen, ashore and aboard (decision 0149, 7: the lift).
    pub(crate) men: u32,
}

/// The supply a landing takes (item 16).
pub(crate) struct Supply {
    /// The level the force is to have aboard.
    pub(crate) level: Fixed,
    /// What is aboard now.
    pub(crate) aboard: Fixed,
    pub(crate) words: String,
}

/// Which lift a passage is (decision 0142, 0143 and 0152): the words of its
/// reason differ. A lift as it stands sent later by a take, and one sent by
/// a hold, carry the head of their reason.
pub(crate) enum LiftKind {
    First,
    Second,
    AsItStands(String),
    Hold(String),
}

impl Supply {
    fn to_take(&self) -> Fixed {
        (self.level - self.aboard).max(Fixed::ZERO)
    }
}

/// The days of item 18.
pub(crate) struct Days {
    embark: u32,
    passage: u32,
    words: String,
}

impl Days {
    /// The day the first men land of a lift whose landing is ready on the
    /// morning of `ready`: it embarks for `embark` days, sails, and lands on
    /// the last day of its passage. The latest day's sum read the other way
    /// (item 13: the landing is to be ready on the morning of the latest day
    /// less the days to embark and of passage, plus 1).
    pub(crate) fn lands_on(&self, ready: u32) -> u32 {
        (ready + self.embark + self.passage).saturating_sub(1)
    }

    /// Item 13: the morning a landing is to be ready by for the place to be
    /// the side's by the end of `by_day`: that day less the days to embark
    /// and of passage, plus 1, and never before day 1.
    pub(crate) fn task_date(&self, by_day: u32) -> u32 {
        (by_day + 1)
            .saturating_sub(self.embark + self.passage)
            .max(1)
    }
}

/// The check that the place can be held (decision 0144, 10 to 17), made
/// where a hold of a base in the place's hex is in force with a source in
/// its forces limit.
struct HoldCheck {
    /// The days of supply that land with the men: the days until supply
    /// follows and the margin.
    days: u32,
    /// The words of the supply's days, for the supply's own words.
    days_words: String,
    /// What is short, each with how much; empty where it can be held.
    short: Vec<String>,
    /// The check in words, in the order a staff works it.
    words: String,
    /// The daily need of the men and the days until supply follows: what
    /// a short lift's supply aboard is weighed against (decision 0169, 4).
    need: Fixed,
    until: u32,
}

/// The share of `attack_fatigue_limit` at or below which a unit not on
/// attack can attack, by risk (decision 0143, 2): the staff's figures,
/// placeholders.
fn attack_share(risk: Risk) -> (i64, i64) {
    match risk {
        Risk::Preserve => (1, 3),
        Risk::Balanced => (2, 3),
        Risk::Press => (1, 1),
    }
}

/// A unit's fit men (decision 0143, 3): its men ashore times 100 less its
/// fatigue over 100, cut to a whole man.
fn fit_men(unit: &LandUnit) -> u32 {
    let hundred = Fixed::from_int(100);
    floor_whole(whole(unit.men_ashore).mul_div(hundred - unit.fatigue, hundred))
}

/// Whether the force is on its way to `place` or landing there now: its
/// route acts at the place and is not done, or it is landing there.
fn under_way(force: &TaskForce, place: Hex) -> bool {
    (force.route.act_at.contains(&place) && force.route.next_index < force.route.waypoints.len())
        || force.landing_at == Some(place)
}

/// Whether the force is on its way to `place` (decision 0152, 5): landing
/// there, or with the place still ahead on its route. A force with no ship
/// afloat is lost, and so is not on its way (decision 0152, reading 7; the
/// architect's ruling of Nov 18, 2026).
pub(crate) fn on_its_way(force: &TaskForce, place: Hex) -> bool {
    if force.ships.is_empty() {
        return false;
    }
    force.landing_at == Some(place)
        || (force.route.act_at.contains(&place)
            && force
                .route
                .waypoints
                .iter()
                .skip(force.route.next_index)
                .any(|h| *h == place))
}

/// The base a hold names, in words.
fn base_text_of(view: &sim_core::view::SideView<'_>, hold: &Intent) -> String {
    hold.kind
        .own_base()
        .map_or_else(String::new, |base| crate::reasons::base_words(view, base))
}

/// A unit of the operation ashore at the place, as the morning's judgement
/// reads it (decision 0143, 1 to 4).
struct Ashore {
    unit: LandUnit,
    can: bool,
    fit: u32,
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
            // Decision 0152, 5b: the later lifts this take sends this
            // morning are set aside before a take lower in priority chooses
            // its first lift.
            for force in self.later_lifts_due(id) {
                self.lifts_due.entry(force).or_insert(id);
            }
        }
    }

    /// Decision 0152, 5b: the lifts as they stand the take `id` sends as
    /// later lifts this morning, read as `launch_place` reads them: its plan
    /// released, its landing formed, and the first lift lost, or landed with
    /// men ashore at the place, or been there with none ashore; and the wait
    /// of item 5 not binding.
    fn later_lifts_due(&self, id: IntentId) -> Vec<TaskForceId> {
        let Some(end) = self.state.intents.get(&id) else {
            return Vec::new();
        };
        let IntentKind::TakePlace { hex: place, .. } = end.kind else {
            return Vec::new();
        };
        let released = self
            .plan_of(end)
            .is_some_and(|plan| plan.state == PlanState::Released);
        let Some(task) = self.landing_of(end) else {
            return Vec::new();
        };
        let IntentKind::ReadyLanding { force: first, .. } = task.kind else {
            return Vec::new();
        };
        let view = self.view;
        let Some(figures) = view.ground_figures() else {
            return Vec::new();
        };
        if !released {
            return Vec::new();
        }
        let risk = self.state.risk_read_up(end.id);
        let wanted = men_wanted(view, place, risk);
        // The wait of item 5 with no man of the operation ashore.
        let none_wanted = wanted == Some(0);
        let waits = match view.task_forces().find(|f| f.id == first) {
            // A first lift held apart until its day (decision 0195, 13) is
            // not lost: no later lift goes before it has entered.
            None if crate::available::entering_on(view, EntityRef::TaskForce(first)).is_some() => {
                return Vec::new();
            }
            None => none_wanted,
            Some(force) => {
                if self
                    .state
                    .forces_as_bound(end)
                    .and_then(|f| f.source)
                    .and_then(|s| view.bases().find(|b| b.id == s))
                    .is_none()
                {
                    return Vec::new();
                }
                let units: Vec<LandUnitId> = self
                    .state
                    .land_units_on_chain(task.id)
                    .map(|f| f.into_iter().collect())
                    .unwrap_or_default();
                let landed = units.iter().any(|u| {
                    view.land_units()
                        .any(|l| l.id == *u && l.men_ashore > 0 && l.hex == Some(place))
                });
                if landed {
                    let (ops, _, _) = self.units_ashore(&units, place, risk, figures);
                    let fit: u32 = ops.iter().filter(|a| a.can).map(|a| a.fit).sum();
                    on_its_way(force, place) || wanted.is_some_and(|w| fit >= w)
                } else if routed(force, place) && !on_its_way(force, place) {
                    none_wanted
                } else {
                    return Vec::new();
                }
            }
        };
        if waits {
            return Vec::new();
        }
        self.later_lift_plan(end, first, None)
            .map(|plan| plan.send.into_iter().map(|s| s.force.id).collect())
            .unwrap_or_default()
    }

    /// The "ready a landing" task the staff formed under `end`, if any.
    pub(crate) fn landing_of(&self, end: &Intent) -> Option<Intent> {
        self.state
            .intents_in_force()
            .find(|i| i.parent == Some(end.id) && matches!(i.kind, IntentKind::ReadyLanding { .. }))
            .cloned()
    }

    /// Item 3: the side holds a base in the hex and the enemy holds none.
    pub(crate) fn place_is_ours(&self, place: Hex) -> bool {
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
        // Item 11: the choice is made once, and made again only on the
        // first count seen from the air or by men in contact (decision 0147,
        // 16).
        if let Some(task) = self.landing_of(end) {
            self.choose_again(end, plan, &task);
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
        let risk = self.state.risk_read_up(end.id);
        let (choice, source, source_hex, staff_limit) = match self.first_choice(end, place, risk) {
            FirstChoice::Found {
                choice,
                source,
                source_hex,
                limit,
            } => (choice, source, source_hex, limit),
            FirstChoice::Unnamed => return,
            FirstChoice::NoSource => {
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
            }
            FirstChoice::NoChoice(evidence, worked) => {
                let reason = serving(end, plan)
                    .evidence(evidence, today)
                    .worked(worked)
                    .would_change("a transport force or a unit the side may use")
                    .done();
                self.target(in_scope).no_orders.push(reason);
                return;
            }
            FirstChoice::NoAssembly(none) => {
                // Item 13: the notice once, the reason each morning.
                let noticed = self.noticed(NoticeKind::CannotMeet, plan.id);
                let reason = serving(end, plan)
                    .evidence(none.evidence.clone(), today)
                    .binding("no base can be the source")
                    .worked(format!(
                        "{}{}",
                        none.worked,
                        if noticed {
                            ""
                        } else {
                            " It hands in the notice that it cannot meet the intent."
                        }
                    ))
                    .would_change(
                        "a transport force stopped at a base of the side, with a fighting unit beside it",
                    )
                    .done();
                if noticed {
                    self.target(in_scope).no_orders.push(reason);
                } else {
                    self.notice(in_scope, reason, NoticeKind::CannotMeet, plan);
                }
                return;
            }
        };
        // Decision 0159, 6 and 7: a first lift short of the men wanted waits
        // for the day that brings them, or says it cannot bring them.
        if let Some(waiting) = self.wait_for_men(end, place, by_day, risk, &choice) {
            self.hand_in_wait(end, plan, &waiting);
            return;
        }
        let assembled = !crate::assemble::names_all(end.limits.forces.as_ref());
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
        // Decision 0152, 4: the supply is reckoned for every man aboard.
        let fed = self.with_aboard(&force, &units);
        // Decision 0144, 13: with a hold of the place, the supply that lands
        // with the men is set by the check that it can be held.
        let check = self.hold_check(end, &force, &fed, figures, place);
        let supply = self.supply(
            &force,
            &fed,
            source,
            source_hex,
            figures,
            check.as_ref().map(|c| (c.days, c.days_words.clone())),
            self.draw_on(end, source),
        );
        // Decision 0169, 1 to 6: a first lift with men aboard, short of the
        // men wanted with too few in reach, goes only where its men can be
        // fed. A first lift whose men are all ashore is decision 0159's.
        let mut fed_words = String::new();
        if choice.picks.iter().any(|p| p.aboard) {
            if let Some(starve) =
                self.starve_check(end, &force, force.id, 0, choice.men, &fed, supply.level)
            {
                if !starve.fed {
                    let words = choice
                        .lines
                        .iter()
                        .filter(|l| {
                            !l.ends_with("the staff goes with what it has")
                                && !l.starts_with("short by ")
                        })
                        .cloned()
                        .collect::<Vec<_>>()
                        .join("; ");
                    self.hand_in_wait(end, plan, &Waiting::Starve { starve, words });
                    return;
                }
                fed_words = starve.goes_words();
            }
        }
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
        let task_date = by_day.map(|day| days.task_date(day));
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
        let new_limit = staff_limit;
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
        let unit_list = if assembled {
            crate::assemble::limit_words(&new_limit)
        } else {
            units
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" and ")
        };
        let worked = format!(
            "{choice_words}; {}; {}.{date_words}{fed_words}",
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

    /// The first lift's choice for `end` (decision 0142, 6 to 10, and
    /// decision 0149), as [`Self::prepare_place`] makes it, reading the units
    /// that may be used on the day [`Day::as_of`] names.
    pub(crate) fn first_choice(&self, end: &Intent, place: Hex, risk: Risk) -> FirstChoice {
        let commander = end.limits.forces.clone();
        // Decision 0149: with all three named the end is decision 0142's;
        // otherwise the staff assembles the rest.
        if crate::assemble::names_all(commander.as_ref()) {
            let Some(limit) = commander else {
                return FirstChoice::Unnamed;
            };
            let Some((source, source_hex)) = limit
                .source
                .and_then(|s| self.view.bases().find(|b| b.id == s).map(|b| (s, b.hex)))
            else {
                return FirstChoice::NoSource;
            };
            match self.choose(end, &limit, place, source_hex, risk, false) {
                Ok(choice) => {
                    let limit = ForcesLimit {
                        ships: BTreeSet::new(),
                        task_forces: BTreeSet::new(),
                        air_groups: BTreeSet::new(),
                        source: None,
                        land_units: choice.picks.iter().map(|p| p.id).collect(),
                    };
                    FirstChoice::Found {
                        choice,
                        source,
                        source_hex,
                        limit,
                    }
                }
                Err((evidence, worked)) => FirstChoice::NoChoice(evidence, worked),
            }
        } else {
            // Under "control an island" with no limit of its own, the take's
            // choices are inside the island's limit, kind by kind (decision
            // 0151, 4).
            let within = commander.clone().or_else(|| {
                self.state
                    .island_on_chain(end.id)
                    .and_then(|island| island.limits.forces.clone())
            });
            match self.assemble(end, within.as_ref(), place, risk) {
                Ok(found) => FirstChoice::Found {
                    choice: found.choice,
                    source: found.source,
                    source_hex: found.source_hex,
                    limit: found.limit,
                },
                Err(none) => FirstChoice::NoAssembly(none),
            }
        }
    }

    // -- A take that waits (decision 0159, rule B) -----------------------

    /// Items 4 to 7 for a take whose plan is still a draft, read before the
    /// leave to prepare is acted on: what the take does in place of forming
    /// its landing, or `None` where it is as it was.
    pub(crate) fn draft_take_waits(&self, end: &Intent) -> Option<Waiting> {
        let IntentKind::TakePlace { hex: place, by_day } = end.kind else {
            return None;
        };
        if self.place_is_ours(place) || self.landing_of(end).is_some() {
            return None;
        }
        self.view.ground_figures()?;
        let coast = self
            .view
            .hex(place)
            .is_some_and(|data| data.kind == HexKind::Coast);
        if !coast {
            return None;
        }
        let risk = self.state.risk_read_up(end.id);
        match self.first_choice(end, place, risk) {
            FirstChoice::Found { choice, .. } => {
                self.wait_for_men(end, place, by_day, risk, &choice)
            }
            _ => None,
        }
    }

    /// Items 4 to 7: where today's first lift is short of the men wanted and
    /// none of its men is aboard, the first later day in reach whose lift
    /// gives them (item 6), or, at balanced, that none does (item 7). `None`
    /// at press, where the lift is not short, where it is a lift as it stands,
    /// and at preserve where no day in reach brings them (as before).
    pub(crate) fn wait_for_men(
        &self,
        end: &Intent,
        place: Hex,
        by_day: Option<u32>,
        risk: Risk,
        today_choice: &Choice,
    ) -> Option<Waiting> {
        if risk == Risk::Press {
            return None;
        }
        let (wanted, have) = today_choice.short?;
        // A lift as it stands, or men already aboard, are as they were.
        if today_choice.picks.iter().any(|p| p.aboard) {
            return None;
        }
        let today = self.today;
        let last = by_day.unwrap_or(today + WAIT_HORIZON_DAYS);
        // Today's choice in words, without the line that it goes short.
        let words = today_choice
            .lines
            .iter()
            .filter(|l| {
                !l.ends_with("the staff goes with what it has") && !l.starts_with("short by ")
            })
            .cloned()
            .collect::<Vec<_>>()
            .join("; ");
        for day in (today + 1)..=last {
            self.as_of.set(Some(day));
            let found = self.first_choice(end, place, risk);
            let reach = match &found {
                FirstChoice::Found {
                    choice,
                    source,
                    source_hex,
                    ..
                } if choice.short.is_none() => {
                    let lift = Lift {
                        force: choice.force,
                        units: choice.picks.iter().map(|p| p.id).collect(),
                        source: *source,
                        source_hex: *source_hex,
                    };
                    match by_day {
                        None => Some((choice.men, lift)),
                        Some(date) => self
                            .ready_by(end, choice, *source, *source_hex, place, risk, day, date)
                            .then_some((choice.men, lift)),
                    }
                }
                _ => None,
            };
            let coming = match &found {
                FirstChoice::Found { choice, .. } => self.coming_in(choice, today),
                _ => Vec::new(),
            };
            self.as_of.set(None);
            if let Some((then, lift)) = reach {
                return Some(Waiting::For {
                    day,
                    wanted,
                    have,
                    then,
                    coming,
                    words,
                    lift,
                });
            }
        }
        if risk != Risk::Balanced {
            return None;
        }
        let reach = match by_day {
            Some(date) => format!(
                "No day up to day {date} on which a landing begun could still be ready by its date for day {date} brings them"
            ),
            None => format!(
                "No day up to day {last}, the {WAIT_HORIZON_DAYS} days the staff looks ahead, brings them"
            ),
        };
        let coming = self
            .named_later(end, today_choice)
            .into_iter()
            .map(|(unit, from)| {
                let why = if from > last {
                    match by_day {
                        Some(date) => format!("after day {date}"),
                        None => format!("beyond day {last}, the last the staff looks ahead to"),
                    }
                } else if let Some(date) = by_day {
                    format!("too late to land by day {date}")
                } else {
                    "and with it the men are still too few".to_string()
                };
                format!("{}, {why}", crate::available::named_with_day(unit, from))
            })
            .collect();
        Some(Waiting::NoDay {
            wanted,
            have,
            coming,
            reach,
            words,
        })
    }

    /// The units and the force of `choice` that may not be used `today`,
    /// each with its day, in words.
    fn coming_in(&self, choice: &Choice, today: u32) -> Vec<String> {
        let view = self.view;
        let mut coming = Vec::new();
        if let Some(from) =
            crate::available::not_before_on(view, EntityRef::TaskForce(choice.force), today)
        {
            coming.push(crate::available::named_with_day(choice.force, from));
        }
        for pick in &choice.picks {
            if let Some(from) =
                crate::available::not_before_on(view, EntityRef::LandUnit(pick.id), today)
            {
                let name = view
                    .land_units()
                    .find(|u| u.id == pick.id)
                    .map_or_else(|| pick.id.to_string(), unit_words);
                coming.push(crate::available::named_with_day(name, from));
            }
        }
        coming
    }

    /// The units and task forces the take may use later and not today, each
    /// with its day, in day and then number order: those its forces limit
    /// names, or with none named every fighting unit and transport force of
    /// the side.
    fn named_later(&self, end: &Intent, today_choice: &Choice) -> Vec<(String, u32)> {
        let view = self.view;
        let limit = end.limits.forces.clone().or_else(|| {
            self.state
                .island_on_chain(end.id)
                .and_then(|island| island.limits.forces.clone())
        });
        let mut found: Vec<(u32, u8, u32, String)> = Vec::new();
        let counted = crate::available::counted_by_a_take(view, limit.as_ref());
        for force in counted.task_forces {
            if force.id == today_choice.force {
                continue;
            }
            if let Some(from) = crate::available::not_before(view, EntityRef::TaskForce(force.id)) {
                found.push((from, 0, force.id.raw(), force.id.to_string()));
            }
        }
        for unit in counted.land_units {
            if let Some(from) = crate::available::not_before(view, EntityRef::LandUnit(unit.id)) {
                found.push((from, 1, unit.id.raw(), unit_words(unit)));
            }
        }
        found.sort();
        found
            .into_iter()
            .map(|(from, _, _, words)| (words, from))
            .collect()
    }

    /// Item 6 with a latest day: whether a landing begun on `day` with
    /// `choice` would be ready by the date the take sets its landing, by the
    /// arithmetic of the notice that it is late (decision 0141, 6; decision
    /// 0142, 13).
    #[allow(clippy::too_many_arguments)]
    fn ready_by(
        &self,
        end: &Intent,
        choice: &Choice,
        source: BaseId,
        source_hex: Hex,
        place: Hex,
        risk: Risk,
        day: u32,
        by_day: u32,
    ) -> bool {
        let Some(figures) = self.view.ground_figures() else {
            return false;
        };
        let Some(force) = self
            .view
            .task_forces()
            .find(|f| f.id == choice.force)
            .cloned()
        else {
            return false;
        };
        let units: Vec<LandUnitId> = choice.picks.iter().map(|p| p.id).collect();
        let Ok(days) = self.lift_days(end, &force, &units, source, source_hex, place, figures)
        else {
            return false;
        };
        let date = days.task_date(by_day);
        let left = i64::from(date) - i64::from(day);
        self.days_to_ready(&force, place, &units, risk, Some(left))
            .is_some_and(|d| day + d <= date)
    }

    /// Hands in the reason of a take that waits (item 6) or cannot bring
    /// enough (item 7), with the notice of item 7 once.
    pub(crate) fn hand_in_wait(&mut self, end: &Intent, plan: &Plan, waiting: &Waiting) {
        let in_scope = self.in_scope(end.id);
        let today = self.today;
        // Decision 0159, 15: kept for the plan order.
        let kept = match waiting {
            Waiting::For {
                day,
                wanted,
                have,
                lift,
                ..
            } => MenWait {
                wanted: *wanted,
                have: *have,
                then: Some((*day, lift.clone())),
            },
            Waiting::NoDay { wanted, have, .. } => MenWait {
                wanted: *wanted,
                have: *have,
                then: None,
            },
            Waiting::Starve { starve, .. } => MenWait {
                wanted: starve.wanted,
                have: starve.have(),
                then: None,
            },
        };
        self.men_waits.insert(end.id, kept);
        match waiting {
            Waiting::For {
                day,
                wanted,
                have,
                then,
                coming,
                words,
                ..
            } => {
                let coming_words = if coming.is_empty() {
                    String::new()
                } else {
                    format!(" {}, and", coming.join(", "))
                };
                let worked = format!(
                    "{words}. The staff wants {wanted} men and has {have} to hand.{coming_words} on day {day} the lift gives {then} men: it waits for day {day}, and forms nothing and gives no order under {} until then.",
                    end.id
                );
                let reason = serving(end, plan)
                    .evidence(words.clone(), today)
                    .binding(format!(
                        "{wanted} men wanted and {have} to hand until day {day}"
                    ))
                    .worked(worked.clone())
                    .would_change("a unit's day, the risk, a limit or the estimate")
                    .done();
                self.target(in_scope).no_orders.push(reason);
                if in_scope {
                    self.lines.push(worked);
                }
            }
            Waiting::NoDay {
                wanted,
                have,
                coming,
                reach,
                words,
            } => {
                let noticed = self.noticed(NoticeKind::CannotBringEnough, plan.id);
                let coming_words = if coming.is_empty() {
                    " No unit or force of the side is coming.".to_string()
                } else {
                    format!(" {}.", coming.join("; "))
                };
                let worked = format!(
                    "{words}. The staff wants {wanted} men and has {have} to hand.{coming_words} {reach}: at balanced it does not go with what it has. It forms nothing and gives no order under {}{}, and waits for the commander.",
                    end.id,
                    if noticed {
                        ""
                    } else {
                        ", hands in the notice that it cannot bring enough"
                    }
                );
                let reason = serving(end, plan)
                    .evidence(words.clone(), today)
                    .binding(format!("{wanted} men wanted and {have} to hand"))
                    .worked(worked.clone())
                    .would_change(
                        "the risk, a limit, the estimate, or a unit's day coming into reach",
                    )
                    .done();
                if noticed {
                    self.target(in_scope).no_orders.push(reason);
                } else {
                    self.notice(in_scope, reason, NoticeKind::CannotBringEnough, plan);
                }
                if in_scope {
                    self.lines.push(worked);
                }
            }
            Waiting::Starve { starve, words } => {
                // Once for the plan: two later lifts stopped on one morning
                // hand in one notice.
                let noticed = self.noticed(NoticeKind::CannotBringEnough, plan.id)
                    || self
                        .target(in_scope)
                        .notices
                        .iter()
                        .any(|n| n.kind == NoticeKind::CannotBringEnough && n.plan == plan.id);
                let worked = format!(
                    "{words}. {} {} At balanced the staff does not land men it cannot feed: it forms nothing and gives {} no order under {}{}, and waits for the commander.",
                    starve.count,
                    starve.fed_words,
                    starve.force,
                    end.id,
                    if noticed {
                        ""
                    } else {
                        ", hands in the notice that it cannot bring enough"
                    }
                );
                let reason = serving(end, plan)
                    .evidence(words.clone(), today)
                    .evidence(starve.fed_words.clone(), today)
                    .binding(format!(
                        "{} men wanted, {} in reach by day {}, and the men cannot be fed",
                        starve.wanted, starve.reach, starve.last
                    ))
                    .worked(worked.clone())
                    .would_change(
                        "the risk, a limit, the estimate, a unit's day coming into reach, or supply at the source",
                    )
                    .done();
                if noticed {
                    self.target(in_scope).no_orders.push(reason);
                } else {
                    self.notice(in_scope, reason, NoticeKind::CannotBringEnough, plan);
                }
                if in_scope {
                    self.lines.push(worked);
                }
            }
        }
    }

    /// The fighting men of `units`, ashore and aboard.
    fn fighting_men(&self, units: &[LandUnitId]) -> u32 {
        units
            .iter()
            .filter_map(|id| self.view.land_units().find(|u| u.id == *id))
            .filter(|u| !u.working)
            .map(|u| u.men_ashore + self.view.men_aboard(u.id).unwrap_or(0))
            .sum()
    }

    /// Decision 0169, 1 to 4: for a lift by `force` at balanced, the count
    /// of the men wanted against the fit men ashore that can attack
    /// (`ashore`) and the fighting men of the lift (`lift`), the men in reach
    /// by the take's last day, and whether the men of `fed` can be fed with
    /// `level` aboard when it sails. `None` where the lift goes as before:
    /// not at balanced, no estimate held, the lift not short, or enough in
    /// reach. `first` is the take's first lift, which is never a later lift.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn starve_check(
        &self,
        end: &Intent,
        force: &TaskForce,
        first: TaskForceId,
        ashore: u32,
        lift: u32,
        fed: &[LandUnitId],
        level: Fixed,
    ) -> Option<Starve> {
        let IntentKind::TakePlace { hex: place, by_day } = end.kind else {
            return None;
        };
        let risk = self.state.risk_read_up(end.id);
        if risk != Risk::Balanced {
            return None;
        }
        let view = self.view;
        let figures = view.ground_figures()?;
        // Item 2: short.
        let wanted = men_wanted(view, place, risk)?;
        if ashore + lift >= wanted {
            return None;
        }
        // Item 3: the men in reach by the take's last day, on the horizon of
        // decision 0159, 6.
        let last = by_day.unwrap_or(self.today + WAIT_HORIZON_DAYS);
        self.as_of.set(Some(last));
        let stand = self.later_lift_plan(end, first, None);
        self.as_of.set(None);
        let mut others: Vec<String> = Vec::new();
        let mut more = 0u32;
        for send in stand.map(|s| s.send).unwrap_or_default() {
            if send.force.id == force.id {
                continue;
            }
            let men: u32 = self
                .aboard(send.force.id)
                .into_iter()
                .filter(|(u, _)| send.fighting.contains(&u.id))
                .map(|(_, men)| men)
                .sum();
            if men == 0 {
                continue;
            }
            more += men;
            let from = view
                .available_from(EntityRef::TaskForce(send.force.id))
                .filter(|from| *from > self.today);
            others.push(match from {
                Some(day) => format!(
                    "{men} aboard {}, which may be used from day {day}",
                    send.force.id
                ),
                None => format!("{men} aboard {}", send.force.id),
            });
        }
        let reach = ashore + lift + more;
        if reach >= wanted {
            return None;
        }
        let have = ashore + lift;
        let ashore_words = if ashore == 0 {
            String::new()
        } else {
            format!(" and the units of the operation ashore at {place} that can attack have {ashore} fit men")
        };
        let last_words = match by_day {
            Some(_) => format!("day {last}, the latest day"),
            None => format!("day {last}, the {WAIT_HORIZON_DAYS} days the staff looks ahead"),
        };
        let others_words = if others.is_empty() {
            "no other force with fighting men aboard can be used by then".to_string()
        } else {
            others.join("; ")
        };
        let count = format!(
            "The staff wants {wanted} men; the lift by {} gives {lift} fighting men{ashore_words}: {have}, short by {}. The men in reach by {last_words}: {reach} ({others_words}), fewer than the {wanted} wanted.",
            force.id,
            wanted - have
        );
        // Item 4: fed.
        let Some((hold, _, _)) = self.hold_of_place(place) else {
            return Some(Starve {
                force: force.id,
                wanted,
                ashore,
                lift,
                reach,
                last,
                count,
                fed_words: format!(
                    "No hold of {place} with a source of supply is in force: nothing follows the men, so they cannot be fed."
                ),
                fed: false,
            });
        };
        let Some(check) = self.hold_check(end, force, fed, figures, place) else {
            return Some(Starve {
                force: force.id,
                wanted,
                ashore,
                lift,
                reach,
                last,
                count,
                fed_words: format!(
                    "The check that the place can be held cannot be worked for {} of {}: the staff can reckon no passage between the place and the source, so the men cannot be fed.",
                    hold.id,
                    base_text_of(view, &hold)
                ),
                fed: false,
            });
        };
        if !check.short.is_empty() {
            return Some(Starve {
                force: force.id,
                wanted,
                ashore,
                lift,
                reach,
                last,
                count,
                fed_words: format!("{} So the men cannot be fed.", check.words),
                fed: false,
            });
        }
        let eaten = check.need * whole(check.until);
        let fed_now = level >= eaten;
        let verdict = if fed_now {
            format!("at least the {} they eat: the men can be fed", tons(eaten))
        } else {
            format!("short by {}: the men cannot be fed", tons(eaten - level))
        };
        Some(Starve {
            force: force.id,
            wanted,
            ashore,
            lift,
            reach,
            last,
            count,
            fed_words: format!(
                "{} What the lift has aboard when it sails: {} against the daily need of {} for the {} days until supply follows, {} x {} = {}, {verdict}.",
                check.words,
                tons(level),
                tons(check.need),
                check.until,
                tons(check.need),
                check.until,
                tons(eaten)
            ),
            fed: fed_now,
        })
    }

    /// Items 6 to 10: the transport force and the units that land, or why
    /// there are none (evidence, working).
    #[allow(clippy::too_many_lines)]
    pub(crate) fn choose(
        &self,
        end: &Intent,
        limit: &ForcesLimit,
        place: Hex,
        source_hex: Hex,
        risk: Risk,
        whole: bool,
    ) -> Result<Choice, (String, String)> {
        let view = self.view;
        // Item 6: the transport force.
        let mut refused: Vec<String> = Vec::new();
        let mut transport: Option<TaskForce> = None;
        for id in &limit.task_forces {
            let Some(force) = view.task_forces().find(|f| f.id == *id) else {
                match crate::available::entering_on(view, EntityRef::TaskForce(*id)) {
                    Some(day) => refused.push(crate::available::enters_words(id, day)),
                    None => refused.push(format!("{id} is not one of the side's forces now")),
                }
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
            if let Some(day) = self.not_before_choice(EntityRef::TaskForce(*id)) {
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
                match crate::available::entering_on(view, EntityRef::LandUnit(*id)) {
                    Some(day) => left.push(crate::available::enters_words(id, day)),
                    None => left.push(format!("{id} is not one of the side's units now")),
                }
                continue;
            };
            if unit.working {
                left.push(format!(
                    "{} is a working unit, which this step does not land",
                    unit_words(unit)
                ));
                continue;
            }
            if let Some(day) = self.not_before_choice(EntityRef::LandUnit(*id)) {
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
                // A lift as it stands lands its units whole (decision 0152, 1).
                if !whole && !picks.is_empty() && chosen_men >= want {
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
            men: chosen_men,
        })
    }

    /// Item 16: the supply the force is to have aboard: the daily charge of
    /// the men of `units` for `days` days, with words for what the days are
    /// (`None`: the 15 days of decision 0107; decision 0144, 13, sets them
    /// by the check that the place can be held).
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn supply(
        &self,
        force: &TaskForce,
        units: &[LandUnitId],
        source: BaseId,
        source_hex: Hex,
        figures: &sim_core::refdata::GroundFigures,
        days: Option<(u32, String)>,
        draw: Result<(), String>,
    ) -> Supply {
        let (days, days_words) = days.unwrap_or_else(|| {
            (
                DEFAULT_INTERVAL_DAYS,
                format!("{DEFAULT_INTERVAL_DAYS} days"),
            )
        });
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
        let wanted = charge * whole(days);
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
        let above = if at_source && draw.is_ok() {
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
            "the supply is the daily charge of the {men} men chosen, {} a day, for {days_words} = {}",
            tons(charge),
            tons(wanted)
        );
        if !aboard.is_zero() {
            words.push_str(&format!(", of which {} is aboard", tons(aboard)));
        }
        if level < wanted {
            // Decision 0152, 4: a force that does not lie at its source, or
            // whose source the staff may not draw on, takes nothing more.
            let short = match (&draw, at_source) {
                (_, false) => format!(
                    ": it does not lie at its source, so it takes nothing more and goes with what is aboard, short by {}",
                    tons(wanted - level)
                ),
                (Err(why), true) => format!(
                    ": the staff may not draw on the source ({why}), so it takes nothing more and goes with what is aboard, short by {}",
                    tons(wanted - level)
                ),
                _ => String::new(),
            };
            let short = if aboard.is_zero() && !at_source {
                String::new()
            } else {
                short
            };
            words.push_str(&format!(
                ", cut to {} by {}{short}",
                tons(level),
                if !at_source {
                    "the force lying away from the source".to_string()
                } else if draw.is_err() {
                    "the source the staff may not draw on".to_string()
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

    /// The daily charge of the men of `units`, ashore and aboard: their men
    /// times `daily_supply_t_per_man` times each unit's weight factor; with
    /// the men.
    fn daily_charge(
        &self,
        units: &[LandUnitId],
        figures: &sim_core::refdata::GroundFigures,
    ) -> (u32, Fixed) {
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
        (men, charge)
    }

    /// The hold of a base in the place's hex, in force with a source of the
    /// side in its forces limit (decision 0144, "The check"), the lowest
    /// numbered: the hold and its source.
    fn hold_of_place(&self, place: Hex) -> Option<(Intent, BaseId, Hex)> {
        let view = self.view;
        self.state.intents_in_force().find_map(|hold| {
            let IntentKind::Hold { base } = hold.kind else {
                return None;
            };
            let hex = view
                .bases()
                .find(|b| b.id == base)
                .map(|b| b.hex)
                .or_else(|| view.enemy_bases().find(|b| b.id == base).map(|b| b.hex))?;
            if hex != place {
                return None;
            }
            let source = self.state.nearest_forces(hold.id)?.source?;
            let source_hex = view.bases().find(|b| b.id == source)?.hex;
            Some((hold.clone(), source, source_hex))
        })
    }

    /// Decision 0144, 10 to 15: whether the place can be held, for the men
    /// of `units` landed by `force`. `None` where no hold of the place with a
    /// source is in force: then nothing follows the landing and the supply
    /// is the 15 days of decision 0142, 16.
    #[allow(clippy::too_many_lines)]
    fn hold_check(
        &self,
        end: &Intent,
        force: &TaskForce,
        units: &[LandUnitId],
        figures: &sim_core::refdata::GroundFigures,
        place: Hex,
    ) -> Option<HoldCheck> {
        let view = self.view;
        let (hold, source, source_hex) = self.hold_of_place(place)?;
        // Item 11: the daily need.
        let (men, need) = self.daily_charge(units, figures);
        // Item 12: the days until supply follows. First, to land the men:
        // the men in the fullest ship at the place's rate for a ship.
        let ships: Vec<&sim_core::state::Ship> = view
            .ships()
            .filter(|s| s.location == ShipLocation::TaskForce(force.id))
            .collect();
        let most_berths = ships
            .iter()
            .filter_map(|s| view.ship_class(s.class))
            .map(|c| c.troop_capacity_men)
            .max()
            .unwrap_or(0);
        let fullest = units
            .iter()
            .filter_map(|id| view.land_units().find(|u| u.id == *id))
            .map(|unit| {
                let aboard = ships
                    .iter()
                    .filter_map(|s| s.troops)
                    .filter(|t| t.unit == unit.id)
                    .map(|t| t.men)
                    .max()
                    .unwrap_or(0);
                aboard.max(unit.men_ashore.min(most_berths))
            })
            .max()
            .unwrap_or(0);
        let weight = figures.handling_t_per_man * i64::from(fullest);
        let port = view
            .hex(place)
            .and_then(|data| view.port_size(data.port_size));
        let rate = port.map_or(Fixed::ZERO, |p| {
            if p.berths > 0 {
                p.berth_rate_t_per_h
            } else {
                p.lighter_rate_t_per_h
            }
        });
        let (land_days, land_words) = if rate > Fixed::ZERO {
            let hours = weight / rate;
            let days = ceil_days(hours).max(1);
            (
                days,
                format!(
                    "{days} to land the men ({} in the fullest ship at {rate} t an hour = {hours} hours)",
                    tons(weight)
                ),
            )
        } else {
            (
                1,
                "1 to land the men (the place gives no rate, so one at least)".to_string(),
            )
        };
        let speed = self.speed_of(force)?;
        let area = self.state.area_read_up(end.id);
        let back = crate::route::route_between(view, place, source_hex, area.as_ref())?;
        let out = crate::route::route_between(view, source_hex, place, area.as_ref())?;
        let back_days = ceil_days(whole(back.out_nm) / speed).max(1);
        let out_days = ceil_days(whole(out.out_nm) / speed).max(1);
        let until = land_days + back_days + 1 + out_days;
        // Item 10: the margin, by the risk of the hold.
        let hold_risk = self.state.risk_read_up(hold.id);
        let margin = crate::holding::margin_days(hold_risk);
        // Items 13 and 14.
        let supply = need * whole(until + margin);
        let first_run = need * whole(DEFAULT_INTERVAL_DAYS);
        // Item 15.
        let stock = view
            .bases()
            .find(|b| b.id == source)
            .map_or(Fixed::ZERO, |b| b.above_reserve(Cargo::Supply));
        let room = ships.iter().fold(Fixed::ZERO, |sum, s| {
            let capacity = view
                .ship_class(s.class)
                .map_or(Fixed::ZERO, |c| c.cargo_capacity_t);
            sum + (capacity - s.cargo_fuel_t).max(Fixed::ZERO)
        });
        let both = supply + first_run;
        let mut short = Vec::new();
        if stock < both {
            short.push(format!(
                "{source} has {} above its reserve against the {} + {} = {} wanted, short by {}",
                tons(stock),
                tons(supply),
                tons(first_run),
                tons(both),
                tons(both - stock)
            ));
        }
        for (what, weight) in [
            ("the supply that lands with the men", supply),
            ("the first run", first_run),
        ] {
            if room < weight {
                short.push(format!(
                    "{}'s room of {} is less than {what}, {}, short by {}",
                    force.id,
                    tons(room),
                    tons(weight),
                    tons(weight - room)
                ));
            }
        }
        let verdict = if short.is_empty() {
            "it can be held".to_string()
        } else {
            format!("it cannot be held: {}", short.join("; "))
        };
        let words = format!(
            "The check that the place can be held, for {} of {}: the daily need of the {men} men chosen is {}; the days until supply follows are {land_words}, {back_days} back to {source}, 1 to load and {out_days} out: {until} days; the margin at {} is {margin} {}; the supply that lands with the men is {} x ({until} + {margin}) = {}; the first run is {} x {DEFAULT_INTERVAL_DAYS} = {}; {source} holds {} above its reserve against {} + {} = {}, and {}'s room is {}: {verdict}.",
            hold.id,
            base_text_of(view, &hold),
            tons(need),
            hold_risk.name(),
            days_word(margin),
            tons(need),
            tons(supply),
            tons(need),
            tons(first_run),
            tons(stock),
            tons(supply),
            tons(first_run),
            tons(both),
            force.id,
            tons(room),
        );
        Some(HoldCheck {
            days: until + margin,
            days_words: format!(
                "{} days, the {until} until supply follows and the margin of {margin} (the check that the place can be held)",
                until + margin
            ),
            short,
            words,
            need,
            until,
        })
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

    /// Decision 0163, 4 and 6: where a lift goes when it has landed. On to
    /// the source of the hold of the place, where a hold of it with a source
    /// is in force and every ship of the force has the fuel for the passage
    /// to the place and on to that source, by the engine's measure at the
    /// speed it goes; else back to `source`, where it came from. Where the
    /// two are one base nothing changes, and the words are `None`.
    pub(crate) fn lift_end(
        &self,
        end: &Intent,
        force: &TaskForce,
        place: Hex,
        (source, source_hex): (BaseId, Hex),
    ) -> (BaseId, Hex, Option<String>) {
        let home = (source, source_hex);
        let Some((hold, on, on_hex)) = self.hold_of_place(place) else {
            return (source, source_hex, None);
        };
        if on == source || on_hex == source_hex {
            return (source, source_hex, None);
        }
        let view = self.view;
        let on_text = crate::reasons::base_words(view, on);
        let not_on = |why: String| {
            (
                home.0,
                home.1,
                Some(format!(
                    "It does not go on to {on_text}, the source of supply of {}, the hold of {place}: {why}; so it goes back to {source}, where it came from.",
                    hold.id
                )),
            )
        };
        let waypoints = match self.route_for(force, place, on_hex, end) {
            Ok((waypoints, _, _)) => waypoints,
            Err(why) => return not_on(why),
        };
        let area = self.state.area_read_up(end.id);
        let Some(out) = crate::route::route_between(view, force.position(), place, area.as_ref())
        else {
            return not_on(format!(
                "ships cannot go from {} to {place}",
                force.position()
            ));
        };
        // The speed it goes: by night, the speed the night's choice gives,
        // else cruising speed; by day, its own.
        let speed = match self.lift_night(end, force, place, &waypoints) {
            Some(night) => crate::night::choose(&night.tried, &night.loads(&night.ships))
                .map_or(SpeedSetting::Cruise, |chosen| chosen.speed),
            None => force.speed,
        };
        let speed_text = crate::night::speed_name(speed);
        let measured = view
            .forecast_route(force.id, &out.out, Some(speed))
            .filter(|f| f.refusal.is_none())
            .zip(
                view.forecast_route(force.id, &waypoints, Some(speed))
                    .filter(|f| f.refusal.is_none()),
            );
        let Some((to_place, whole_way)) = measured else {
            return not_on(format!(
                "the engine would refuse the passage to {place} and on to {on_text}"
            ));
        };
        let mut lines = Vec::new();
        let mut short = false;
        for line in &whole_way.ships {
            let out_t = to_place
                .ships
                .iter()
                .find(|l| l.ship == line.ship)
                .map_or(Fixed::ZERO, |l| l.burned_t);
            // A ship the measure runs dry burns only what it has: its need is
            // the burn of the whole way at the speed, by the rule the measure
            // burns by, as the night's measure gives it (chapter 2, Fuel burn).
            let whole_t = if line.runs_dry {
                crate::night::burn_of(view, line.ship, whole_way.route_nm, whole_way.speed_kn)
                    .unwrap_or(line.burned_t)
            } else {
                line.burned_t
            };
            let on_t = (whole_t - out_t).max(Fixed::ZERO);
            let lacks = line.runs_dry || line.fuel_t < whole_t;
            short |= lacks;
            lines.push(format!(
                "{} has {} against {} to {place} and {} on to {on_text}, {}{}",
                line.ship,
                tons(line.fuel_t),
                tons(out_t),
                tons(on_t),
                tons(whole_t),
                if line.runs_dry {
                    ", running dry short of it"
                } else {
                    ""
                }
            ));
        }
        let measure = format!(
            "by the engine's measure at {speed_text}, {}",
            lines.join(", ")
        );
        if short {
            return not_on(format!("not every ship has the fuel, {measure}"));
        }
        (
            on,
            on_hex,
            Some(format!(
                "It goes on to {on_text}, the source of supply of {}, the hold of {place}, to join the ships that feed the place: every ship has the fuel, {measure}.",
                hold.id
            )),
        )
    }

    /// The slowest cruise speed of the force's ships afloat.
    pub(crate) fn speed_of(&self, force: &TaskForce) -> Option<Fixed> {
        self.view
            .ships()
            .filter(|s| s.location == ShipLocation::TaskForce(force.id))
            .filter_map(|s| self.view.ship_class(s.class))
            .map(|c| c.cruise_speed_kn)
            .min()
            .filter(|speed| *speed > Fixed::ZERO)
    }

    /// Item 18 for a lift of `units` by `force` from `source`: the supply it
    /// takes, reckoned for every man aboard and by the check that the place
    /// can be held (decision 0144, 13; decision 0152, 4), and from it the
    /// days to embark and of the passage. The sums a latest day is worked by
    /// ([`Self::ready_by`]), and the plan order's day ashore.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn lift_days(
        &self,
        end: &Intent,
        force: &TaskForce,
        units: &[LandUnitId],
        source: BaseId,
        source_hex: Hex,
        place: Hex,
        figures: &sim_core::refdata::GroundFigures,
    ) -> Result<Days, String> {
        let fed = self.with_aboard(force, units);
        let check = self.hold_check(end, force, &fed, figures, place);
        let supply = self.supply(
            force,
            &fed,
            source,
            source_hex,
            figures,
            check.as_ref().map(|c| (c.days, c.days_words.clone())),
            self.draw_on(end, source),
        );
        self.days(force, units, &supply, place, source_hex, end)
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
                // Decision 0147, 14: the need to know does not wait for the
                // landing to be formed: a plan whose landing the staff cannot
                // form (at preserve the force, with a count that asks for
                // more men than the ships carry) still sends a group to look.
                self.look_orders(&end, &plan);
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
        // Decision 0147, 14 and 15: the group that looks at the place.
        self.look_orders(end, plan);
        let Some(force) = view.task_forces().find(|f| f.id == force_id).cloned() else {
            if let Some(day) = crate::available::entering_on(view, EntityRef::TaskForce(force_id)) {
                // Held apart until its day (decision 0195, 13): not lost, so
                // the later lifts are not sent as for a lost first lift; one
                // reason, no order under the take.
                let reason = serving(end, plan)
                    .evidence(crate::available::enters_words(force_id, day), today)
                    .binding(format!("the day {force_id} enters, {day}"))
                    .worked(format!(
                        "{force_id} enters on day {day}, so the staff gives no order under {} before then.",
                        end.id
                    ))
                    .would_change("the day coming")
                    .done();
                self.target(in_scope).no_orders.push(reason);
                return;
            }
            // Decision 0152, 5: the first lift is lost; the later lifts as
            // they stand go.
            if plan.state == PlanState::Released
                && self.later_lifts_alone(end, plan, task, place, None, force_id)
            {
                return;
            }
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
        // The source as the end's limit binds: the commander's, or the
        // staff's (decision 0149, 3).
        let Some((source, source_hex)) = self
            .state
            .forces_as_bound(end)
            .and_then(|f| f.source)
            .and_then(|s| view.bases().find(|b| b.id == s).map(|b| (s, b.hex)))
        else {
            return;
        };
        // The task's units as its limit binds (decision 0149, 3).
        let units: Vec<LandUnitId> = self
            .state
            .land_units_on_chain(task.id)
            .map(|f| f.into_iter().collect())
            .unwrap_or_default();
        let mut now = plan.state;

        // Decision 0144, 10 to 17: the check that the place can be held,
        // where a hold of it with a source is in force; the supply reckoned
        // for every man aboard (decision 0152, 4).
        let fed = self.with_aboard(&force, &units);
        let check = self.hold_check(end, &force, &fed, figures, place);
        let supply = self.supply(
            &force,
            &fed,
            source,
            source_hex,
            figures,
            check.as_ref().map(|c| (c.days, c.days_words.clone())),
            self.draw_on(end, source),
        );

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
            // Decision 0144, 16 and 17: the check, before leave is asked.
            let risk = self.state.risk_read_up(end.id);
            let mut check_words = String::new();
            if let Some(check) = &check {
                if !check.short.is_empty() && risk == Risk::Preserve {
                    let noticed = self.noticed(NoticeKind::CannotMeet, plan.id);
                    let short = check.short.join("; ");
                    let reason = serving(end, plan)
                        .evidence(reading.words.clone(), today)
                        .evidence(short.clone(), today)
                        .binding(short.clone())
                        .worked(format!(
                            "The landing at {place} is ready ({}). {} At preserve the force the staff lands no men it cannot keep fed: it does not ask leave to launch and does not release {}{}.",
                            reading.words,
                            check.words,
                            plan.id,
                            if noticed {
                                ""
                            } else {
                                ", and hands in the notice that it cannot meet the intent"
                            }
                        ))
                        .would_change("more supply at the source, more room in the ships, a bolder risk level")
                        .done();
                    if noticed {
                        self.target(in_scope).no_orders.push(reason);
                    } else {
                        self.notice(in_scope, reason, NoticeKind::CannotMeet, plan);
                    }
                    return;
                }
                check_words = if check.short.is_empty() {
                    format!(" {}", check.words)
                } else {
                    format!(
                        " {} At {} the staff goes all the same.",
                        check.words,
                        risk.name()
                    )
                };
            }
            // Decision 0147, 18 and 19: not launched without the count the
            // risk asks for. Read after the check of decision 0144, so that a
            // landing that is otherwise ready is one that passes it too.
            let Some(goes_on) = self.need_to_know(end, plan, task, &reading.words) else {
                return;
            };
            check_words.push_str(&goes_on);
            // Decision 0159, 9 to 11: not released while a limit the
            // commander set to tell the plan to wait is unmet, and leave to
            // launch not asked.
            if self.told_to_wait(end, plan, &reading.words) {
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
                        "The landing at {place} is ready ({}) and the commander has not given leave to launch, so the plan waits for him.{when}{check_words}",
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
                    "The landing at {place} is ready ({}) and leave to launch is given: the staff releases {}.{check_words}",
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
        // Decision 0143: once a unit chosen has men ashore at the place, the
        // judgement of each morning and a second lift.
        let landed = units.iter().any(|id| {
            view.land_units()
                .any(|u| u.id == *id && u.men_ashore > 0 && u.hex == Some(place))
        });
        if landed {
            self.ashore(end, plan, task, &force, place, source, source_hex);
            return;
        }
        // Decision 0191: a lift under way with men aboard, read against the
        // danger before the take's own step routes it on.
        if matches!(
            self.turn_back_step(end, plan, &force, place, (source, source_hex)),
            crate::turn_back::Reading::Decided
        ) {
            return;
        }
        if routed(&force, place) {
            // Decision 0152, 5: the first lift has been to the place and no
            // man of it is ashore there; the later lifts as they stand go.
            if !on_its_way(&force, place)
                && self.later_lifts_alone(end, plan, task, place, Some(&force), force_id)
            {
                return;
            }
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
        self.move_step(
            end,
            plan,
            &force,
            &units,
            &supply,
            &LiftKind::First,
            place,
            (source, source_hex),
        );
    }

    /// The steps of a lift (decision 0142, 16 to 19, and 0143, 11), one a
    /// morning: the supply, then one unit ashore at the source a morning
    /// (set to attack before it sails only on the first lift), then the
    /// passage with the unload order and an escort as for a run.
    #[allow(clippy::too_many_lines, clippy::too_many_arguments)]
    pub(crate) fn move_step(
        &mut self,
        end: &Intent,
        plan: &Plan,
        force: &TaskForce,
        units: &[LandUnitId],
        supply: &Supply,
        kind: &LiftKind,
        place: Hex,
        (source, source_hex): (BaseId, Hex),
    ) {
        let first_lift = matches!(kind, LiftKind::First);
        let in_scope = self.in_scope(end.id);
        let today = self.today;
        let view = self.view;
        let force_id = force.id;
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
        // Decision 0163, 4: where it goes when it has landed.
        let (back, back_hex, onward_words) = self.lift_end(end, force, place, (source, source_hex));
        // Decision 0145: a lift timed to the dark.
        let night = self
            .route_for(force, place, back_hex, end)
            .ok()
            .and_then(|(waypoints, _, _)| self.lift_night(end, force, place, &waypoints));

        // Item 16: the supply, first, cut to the night's limit (decision
        // 0145, 7 and 8).
        let mut supply_level = supply.level;
        let mut night_supply_words = String::new();
        if let Some(night) = &night {
            let planned = night.with_supply(supply.level);
            if let Some(chosen) = crate::night::choose(&night.tried, &night.loads(&planned)) {
                let hours = chosen.night.unwrap_or(Fixed::ZERO);
                let lines: Vec<(Fixed, Fixed, Fixed)> = night
                    .ships
                    .iter()
                    .map(|s| {
                        (
                            s.supply + night.handling * i64::from(s.men),
                            s.room,
                            s.rate * hours,
                        )
                    })
                    .collect();
                let aboard = night
                    .ships
                    .iter()
                    .fold(Fixed::ZERO, |sum, s| sum + s.supply);
                let most = aboard + crate::night::weight_within(&lines);
                if supply_level > most {
                    night_supply_words = format!(
                        " By night: {}; {}. At {} the night's hours left on arrival are {hours}, and no ship carries more than it can land in them, so the supply is cut from {} to {}.",
                        night.near,
                        crate::night::tried_words(&night.tried),
                        crate::night::speed_name(chosen.speed),
                        tons(supply_level),
                        tons(most)
                    );
                    supply_level = most.max(supply.aboard);
                }
            }
        }
        let take = (supply_level - supply.aboard).max(Fixed::ZERO);
        if !take.is_zero() {
            let loading = cargo_order.is_some_and(|c| {
                c.action == CargoAction::Load
                    && c.unit.is_none()
                    && c.weight_t == Some(supply_level)
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
                .worked(if first_lift {
                    format!(
                        "{} is released: {}.{night_supply_words} The staff orders {force_id} to load supply at the source until it has {} aboard.",
                        plan.id,
                        supply.words,
                        tons(supply_level)
                    )
                } else {
                    format!(
                        "A second lift: {}.{night_supply_words} The staff orders {force_id} to load supply at the source until it has {} aboard.",
                        supply.words,
                        tons(supply_level)
                    )
                })
                .would_change("the source's stock, or other units chosen")
                .done();
            if in_scope {
                self.lines.push(format!(
                    "{force_id} loads supply to {}.",
                    tons(supply_level)
                ));
            }
            self.target(in_scope).orders.push(StaffOrder {
                kind: OrderKind::TaskForce {
                    target: force_id,
                    order: TaskForceOrder::LoadTo {
                        cargo: Cargo::Supply,
                        weight_t: supply_level,
                        // A lift's load is cut as decision 0145 has it, with
                        // no level (decision 0162, 9).
                        each_t: None,
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
        // Decision 0145, 9: a unit goes only in ships that can each land
        // their part of it in the night's hours; one that cannot is left for
        // a later run.
        let mut left_words: Vec<String> = Vec::new();
        if let Some(night) = &night {
            let mut kept = Vec::new();
            for unit in ashore {
                let ships = night.with_unit(unit.id, unit.men_ashore);
                let eligible = night
                    .tried
                    .iter()
                    .any(|t| t.night.is_some() && t.has_fuel());
                let fits = !eligible
                    || night.tried.iter().any(|t| {
                        t.night.is_some_and(|hours| {
                            t.has_fuel()
                                && night
                                    .loads(&ships)
                                    .iter()
                                    .all(|(load, rate)| *load <= *rate * hours)
                        })
                    });
                if fits {
                    kept.push(unit);
                } else {
                    let best = night
                        .tried
                        .iter()
                        .filter(|t| t.has_fuel())
                        .filter_map(|t| t.night)
                        .max()
                        .unwrap_or(Fixed::ZERO);
                    left_words.push(format!(
                        "{} is left for a later run: taken aboard, its ships would land {} and not every one can in the night's {best} hours",
                        unit_words(unit),
                        night.limits_words(&ships, best)
                    ));
                }
            }
            ashore = kept;
        }
        let left_text = if left_words.is_empty() {
            String::new()
        } else {
            format!(
                " By night: {}; {}.",
                night.as_ref().map_or_else(String::new, |n| n.near.clone()),
                left_words.join("; ")
            )
        };
        if let Some(unit) = ashore.first() {
            let unit = (*unit).clone();
            let hold = HoldTarget::LandUnit(unit.id);
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
            if unit.hex != Some(source_hex) || !crate::fleet::force_stopped_at(force, source_hex) {
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
            if first_lift && unit.posture != Posture::Attack {
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
            let worked = if first_lift {
                format!("{words}. The staff orders {force_id} to take it aboard, with no cargo.")
            } else {
                format!(
                    "{words}. The staff orders {force_id} to take it aboard, with no cargo. It is a unit of a second lift: it is not set to attack before it sails, for the judgement of each morning sets it once it is ashore."
                )
            };
            let reason = serving(end, plan)
                .evidence(words.clone(), today)
                .worked(worked)
                .would_change("another unit next")
                .done();
            if in_scope {
                self.lines.push(if first_lift {
                    format!("{} is set to attack and goes aboard {force_id}.", unit.id)
                } else {
                    format!("{} goes aboard {force_id}.", unit.id)
                });
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

        // Decision 0191, 14, 19 and 20: a first lift with men aboard does
        // not sail where the danger would turn it back at the risk, nor on
        // the first clear morning after it.
        let danger_words = if first_lift {
            match self.before_sailing(end, plan, force, place) {
                Ok(words) => words,
                Err(reason) => {
                    self.target(in_scope).no_orders.push(*reason);
                    return;
                }
            }
        } else {
            String::new()
        };

        // Item 19: the passage.
        let (waypoints, out_nm, back_nm) = match self.route_for(force, place, back_hex, end) {
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
        let Some(speed) = self.speed_of(force) else {
            return;
        };
        let route_nm = whole(out_nm + back_nm);
        // Decision 0145, 5, 11 and 12: the speed that brings it there in the
        // night, with the fuel reckoned at that speed.
        let mut night_words = String::new();
        let mut night_speed: Option<(SpeedSetting, String)> = None;
        if let Some(night) = &night {
            let tried_text = crate::night::tried_words(&night.tried);
            match crate::night::choose(&night.tried, &night.loads(&night.ships)) {
                Some(chosen) => {
                    let hours = chosen.night.unwrap_or(Fixed::ZERO);
                    let name = crate::night::speed_name(chosen.speed);
                    night_words = format!(
                        " By night: {}; {tried_text}. It goes at {name}, with {hours} hours of the night left on arrival: {}; it lands in the night and steams on at the morning.",
                        night.near,
                        night.limits_words(&night.ships, hours)
                    );
                    if force.speed != chosen.speed {
                        night_speed = Some((
                            chosen.speed,
                            format!(
                                "{}; {tried_text}: {force_id} is set to {name}, the first that brings it there in the night with the fuel and room for its load.",
                                night.near
                            ),
                        ));
                    }
                }
                None if night.risk == Risk::Preserve => {
                    let why = format!(
                        "{}; {tried_text}: neither speed brings it there in the night, and at preserve the force it does not sail by day",
                        night.near
                    );
                    // A hold's lift hands in no notice: the hold is not met
                    // or failed by one force.
                    let take = matches!(end.kind, IntentKind::TakePlace { .. });
                    let noticed = !take || self.noticed(NoticeKind::CannotMeet, plan.id);
                    let reason = serving(end, plan)
                        .evidence(why.clone(), today)
                        .binding(why.clone())
                        .worked(format!(
                            "Every unit is aboard and the supply is aboard, but {why}: the force is not routed{}.",
                            if noticed { "" } else { ", and the staff hands in the notice that it cannot meet the intent" }
                        ))
                        .would_change("a speed that arrives in the dark, or a bolder risk level")
                        .done();
                    if noticed {
                        self.target(in_scope).no_orders.push(reason);
                    } else {
                        self.notice(in_scope, reason, NoticeKind::CannotMeet, plan);
                    }
                    return;
                }
                None => {
                    night_words = format!(
                        " By night: {}; {tried_text}: neither speed brings it there in the night, so at balanced it goes by day at cruising speed.",
                        night.near
                    );
                    if force.speed != SpeedSetting::Cruise {
                        night_speed = Some((
                            SpeedSetting::Cruise,
                            format!(
                                "{}; {tried_text}: neither speed brings it there in the night, so {force_id} is set to cruising speed and goes by day.",
                                night.near
                            ),
                        ));
                    }
                }
            }
        }
        let fuel_speed = night_speed
            .as_ref()
            .map(|(s, _)| *s)
            .or_else(|| night.as_ref().map(|_| force.speed));
        // The fuel for the round trip, as for a run (decision 0107); for a
        // lift by night, at the speed it goes at, by the engine's measure.
        let measured: Option<Vec<crate::stand::ShipFuel>> = fuel_speed.and_then(|s| {
            night.as_ref().and_then(|n| {
                n.tried
                    .iter()
                    .find(|t| t.speed == s)
                    .map(|t| t.fuel.clone())
            })
        });
        let mut short: Vec<String> = Vec::new();
        let mut fuel_lines: Vec<String> = Vec::new();
        for ship in view
            .ships()
            .filter(|s| s.location == ShipLocation::TaskForce(force_id))
        {
            let Some(class) = view.ship_class(ship.class) else {
                continue;
            };
            let need = measured
                .as_ref()
                .and_then(|m| m.iter().find(|f| f.ship == ship.id))
                .map_or_else(
                    || route_nm.mul_div(class.fuel_capacity_t, class.endurance_nm),
                    |f| f.need,
                );
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
        let bound_forces: BTreeSet<TaskForceId> = self
            .state
            .forces_as_bound(end)
            .map(|f| f.task_forces.clone())
            .unwrap_or_default();
        let task_forces = &bound_forces;
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
        let loading_words = match kind {
            LiftKind::First | LiftKind::Hold(_) => String::new(),
            LiftKind::Second | LiftKind::AsItStands(_) => {
                " It goes as it stands, with no landing made ready.".to_string()
            }
        };
        let head = match kind {
            LiftKind::First | LiftKind::Second => format!(
                "Every unit chosen is aboard {force_id} and its supply is aboard ({}).{}",
                supply.words,
                self.also_aboard(force, units)
            ),
            LiftKind::AsItStands(head) | LiftKind::Hold(head) => head.clone(),
        };
        let back_words = if back == source {
            format!("{back_nm} nm back to the source {source}")
        } else {
            format!("{back_nm} nm on to {back}")
        };
        let onward_text = onward_words.map_or_else(String::new, |w| format!(" {w}"));
        let worked = format!(
            "{head}{left_text} It sails today (day {today}, hour {sail_hour}): {out_nm} nm out at {speed} kn = {hours_out} hours, so it arrives at {place} at hour {}; {back_words}.{onward_text} Fuel: {}.{}{loading_words}{night_words}{danger_words}",
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
        if let Some((setting, words)) = night_speed {
            let speed_reason = serving(end, plan)
                .evidence(words.clone(), today)
                .binding("a lift by night arrives in the night")
                .worked(words)
                .would_change("no enemy base within 15 hexes of the place, or press")
                .done();
            self.target(in_scope).orders.push(StaffOrder {
                kind: OrderKind::TaskForce {
                    target: force_id,
                    order: TaskForceOrder::SetSpeed(setting),
                },
                intent: end.id,
                reason: speed_reason,
            });
        }
        let route_reason = reason(if back == source {
            "This is the route order: out to the place and back to the source, acting at the place."
        } else {
            "This is the route order: out to the place and on to the source of supply of the hold of the place, acting at the place."
        });
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

    // -- Ashore (decision 0143) ---------------------------------------------

    /// The judgement of each morning (items 1 to 8) and a second lift
    /// (items 9 to 13), on a morning the plan is released, the place is not
    /// taken and a unit chosen has men ashore at the place.
    #[allow(clippy::too_many_lines, clippy::too_many_arguments)]
    fn ashore(
        &mut self,
        end: &Intent,
        plan: &Plan,
        task: &Intent,
        force: &TaskForce,
        place: Hex,
        source: BaseId,
        source_hex: Hex,
    ) {
        let in_scope = self.in_scope(end.id);
        let today = self.today;
        let view = self.view;
        let Some(figures) = view.ground_figures() else {
            return;
        };
        let risk = self.state.risk_read_up(end.id);
        let task_units: Vec<LandUnitId> = self
            .state
            .land_units_on_chain(task.id)
            .map(|f| f.into_iter().collect())
            .unwrap_or_default();
        let (ops, held, mark) = self.units_ashore(&task_units, place, risk, figures);
        // Item 4: the men wanted.
        let estimate = view.ground_estimate(place).map(|e| (e.men, e.day));
        let wanted = estimate.map(|(men, _)| men * men_per_man(risk));
        let fit: u32 = ops.iter().filter(|a| a.can).map(|a| a.fit).sum();

        // Items 9 to 13: what is coming.
        // A unit aboard another force is a later lift as it stands
        // (decision 0152, 5), not the first force's to bring.
        let elsewhere = |id: LandUnitId| {
            view.ships().any(|s| {
                s.troops.is_some_and(|t| t.unit == id)
                    && matches!(s.location, ShipLocation::TaskForce(f) if f != force.id)
            })
        };
        let lift: Vec<LandUnitId> = task_units
            .iter()
            .copied()
            .filter(|id| {
                view.land_units()
                    .find(|u| u.id == *id)
                    .is_some_and(|u| !(u.men_ashore > 0 && u.hex == Some(place)))
                    && (view.land_units().any(|u| u.id == *id && u.men_ashore > 0)
                        || view.men_aboard(*id).unwrap_or(0) > 0)
                    && !elsewhere(*id)
            })
            .collect();
        let short_or_none = wanted.is_none_or(|w| fit < w);
        let mut picks: Vec<(LandUnitId, u32)> = Vec::new();
        let mut lift_lines: Vec<String> = Vec::new();
        let coming = if !lift.is_empty() {
            let list = lift
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" and ");
            if crate::fleet::force_stopped_at(force, source_hex) {
                format!(
                    "{list} of a second lift are to come by {}, which lies at the source {source}",
                    force.id
                )
            } else {
                format!("{list} of a second lift are on their way by {}", force.id)
            }
        } else {
            let (left, left_lines) = self.left_behind(end, task, source_hex);
            lift_lines = left_lines;
            if left.is_empty() {
                if lift_lines.is_empty() {
                    "no unit is left behind to bring".to_string()
                } else {
                    format!(
                        "no unit is left behind to bring ({})",
                        lift_lines.join("; ")
                    )
                }
            } else {
                let empty = view
                    .ships()
                    .filter(|s| s.location == ShipLocation::TaskForce(force.id))
                    .all(|s| {
                        s.supply_t.is_zero() && s.cargo_fuel_t.is_zero() && s.troops.is_none()
                    });
                let at_source = crate::fleet::force_stopped_at(force, source_hex);
                let left_words = left
                    .iter()
                    .map(|u| format!("{} ({} men)", unit_words(u), u.men_ashore))
                    .collect::<Vec<_>>()
                    .join(", ");
                if short_or_none && at_source && empty {
                    picks = self.lift_picks(force, &left, wanted, fit, &mut lift_lines);
                    if picks.is_empty() {
                        format!(
                            "left behind at the source: {left_words}; but {}",
                            lift_lines.join("; ")
                        )
                    } else {
                        format!(
                            "left behind at the source: {left_words}; {} lies stopped at the source with nothing aboard, so {} {} sent now",
                            force.id,
                            picks
                                .iter()
                                .map(|(id, _)| id.to_string())
                                .collect::<Vec<_>>()
                                .join(" and "),
                            if picks.len() == 1 { "is" } else { "are" }
                        )
                    }
                } else if !short_or_none {
                    format!("left behind at the source: {left_words}; not needed while the men are enough")
                } else {
                    format!(
                        "left behind at the source: {left_words}, to be sent when {} lies stopped at the source with nothing aboard",
                        force.id
                    )
                }
            }
        };

        // Decision 0152, 5: the later lifts as they stand.
        let wait = if on_its_way(force, place) {
            Some(format!(
                "the first lift, {}, is still on its way to {place}",
                force.id
            ))
        } else if !short_or_none {
            wanted.map(|w| {
                format!("the {fit} fit men ashore that can attack are at least the {w} wanted")
            })
        } else {
            None
        };
        let stand = self.later_lift_plan(end, force.id, wait.as_deref());
        // Items 5 to 8 and 15: the judgement, in words.
        let estimate_words = match estimate {
            Some((men, day)) => format!(
                "the side's estimate is {men} enemy men at {place}, of day {day}; at {} the staff wants {} men for each, {} men",
                risk.name(),
                men_per_man(risk),
                men * men_per_man(risk)
            ),
            None => format!("the side holds no estimate of the enemy's men at {place}"),
        };
        let unit_lines =
            ops.iter()
                .map(|a| {
                    format!(
                    "{}: {} men ashore, on {}, fatigue {} against the mark of {mark}, {} fit men{}",
                    unit_words(&a.unit),
                    a.unit.men_ashore,
                    a.unit.posture.name(),
                    a.unit.fatigue,
                    a.fit,
                    if a.can { ", can attack" } else { ", cannot attack" }
                )
                })
                .collect::<Vec<_>>()
                .join("; ");
        let mut judged = format!(
            "{estimate_words}; the units ashore: {}",
            if unit_lines.is_empty() {
                "none".to_string()
            } else {
                unit_lines
            }
        );
        if !held.is_empty() {
            judged.push_str(&format!("; {}", held.join("; ")));
        }
        judged.push_str(&format!("; the units that can attack have {fit} fit men"));
        if let Some(w) = wanted {
            if fit >= w {
                judged.push_str(&format!(", at least the {w} wanted"));
            } else {
                judged.push_str(&format!(" against the {w} wanted, short by {}", w - fit));
            }
        }
        judged.push_str(&format!("; coming: {coming}"));
        if let Some(stand) = &stand {
            judged.push_str(&format!(
                "; the forces with men aboard, in force order: {}",
                stand.words.join("; ")
            ));
        }
        // Decision 0193: on a morning the men that can attack are fewer than
        // the men wanted, whether more is coming; with nothing more coming
        // the men wanted fall by the risk (3, 1 or none for each man of the
        // estimate). While more is coming everything is as before.
        let nothing: Option<(u32, String)> = match (estimate, wanted) {
            (Some((men, _)), Some(w)) if fit < w => {
                let found = self.more_coming(end, task, force, place, (source, source_hex), fit, w);
                (!found.more).then(|| {
                    (
                        men * coming::men_per_man_with_nothing_coming(risk),
                        found.words,
                    )
                })
            }
            _ => None,
        };
        // Rule 12: the units on attack the staff sent in under this rule,
        // read from its own reasons in the side's log.
        let kept: BTreeSet<LandUnitId> = if nothing.is_some() {
            let on_attack: Vec<LandUnitId> = ops
                .iter()
                .filter(|a| a.unit.posture == Posture::Attack)
                .map(|a| a.unit.id)
                .collect();
            on_attack
                .into_iter()
                .filter(|id| self.sent_in(*id))
                .collect()
        } else {
            BTreeSet::new()
        };
        let verdict = coming::verdict(
            fit,
            estimate.map(|(men, _)| men),
            risk,
            nothing.is_none(),
            !kept.is_empty(),
        );
        debug_assert!(nothing.is_some() || verdict != coming::Verdict::Committed);
        // Decision 0194, 7 to 9: in the game's last days the staff begins no
        // attack ashore, and a unit on attack whose side's attack at the
        // place failed the day before is set to defend. With 3 days left or
        // more, or in a world with no last day, nothing here changes.
        let left = coming::days_left(view.game_days(), today);
        let last_days = coming::in_last_days(left);
        let failed = last_days && self.attack_failed(place, today - 1);
        // An attack the judgement would begin, and does not (rule 8).
        let held_back = last_days && verdict == coming::Verdict::Attack;
        let attack = verdict == coming::Verdict::Attack && !last_days;
        // Rule 12: once sent in, they stay in; in the last days, not after
        // a failed day (decision 0194, 9).
        let committed = verdict == coming::Verdict::Committed && !failed;
        // Decision 0194, 9: the units on attack the staff takes out, where
        // the judgement itself would not: after a failed day in the last
        // days, whether it would attack or keep them in.
        let pulled = failed && verdict != coming::Verdict::Wait;
        if let Some((w, words)) = &nothing {
            let per = coming::men_per_man_with_nothing_coming(risk);
            judged.push_str(&format!("; {words}"));
            if per == 0 {
                judged.push_str(&format!(
                    "; at {}, with nothing more coming, the staff wants none: every unit that can attack goes in, whatever the estimate",
                    risk.name()
                ));
            } else {
                judged.push_str(&format!(
                    "; at {}, with nothing more coming, the staff wants {per} {} for each, {w} men; the units that can attack have {fit} fit men{}",
                    risk.name(),
                    if per == 1 { "man" } else { "men" },
                    if fit >= *w {
                        format!(", at least the {w} wanted")
                    } else {
                        format!(" against the {w} wanted, short by {}", w - fit)
                    }
                ));
            }
        }
        if let Some(left) = left.filter(|_| last_days) {
            judged.push_str(&format!("; {}", coming::last_days_words(left)));
            if failed {
                judged.push_str(&format!(
                    "; the side's attack at {place} failed on day {}",
                    today - 1
                ));
            }
        }
        let mut orders = Vec::new();
        if attack {
            for a in ops
                .iter()
                .filter(|a| a.can && a.unit.posture != Posture::Attack)
            {
                let mut reason = serving(end, plan)
                    .evidence(judged.clone(), today)
                    .worked(format!(
                        "{judged}. {}: the staff sets {} to attack.",
                        if estimate.is_none() {
                            "With no count of the enemy it attacks"
                        } else if nothing.is_some() {
                            "Nothing more is coming and the men that can attack are enough for it"
                        } else {
                            "The men that can attack are enough"
                        },
                        unit_words(&a.unit)
                    ))
                    .would_change(if nothing.is_some() {
                        "a larger estimate of the enemy, more fatigue, or more men coming"
                    } else {
                        "a larger estimate of the enemy, or more fatigue"
                    });
                if nothing.is_some() {
                    reason = reason.binding(coming::SENT_IN);
                }
                let reason = reason.done();
                if in_scope {
                    self.lines.push(format!("{} is set to attack.", a.unit.id));
                }
                orders.push(StaffOrder {
                    kind: OrderKind::LandUnit {
                        target: a.unit.id,
                        order: LandUnitOrder::SetPosture {
                            posture: Posture::Attack,
                            when_spent: None,
                        },
                    },
                    intent: end.id,
                    reason,
                });
            }
        } else if !held_back || pulled {
            for a in ops.iter().filter(|a| {
                a.unit.posture == Posture::Attack && !(committed && kept.contains(&a.unit.id))
            }) {
                let reason = if pulled {
                    serving(end, plan)
                        .evidence(judged.clone(), today)
                        .worked(format!(
                            "{judged}. The side's attack failed yesterday and the game is in its last days: an attack so late only costs men, so the staff sets {} to defend, where it rests.",
                            unit_words(&a.unit)
                        ))
                        .would_change("the place the side's")
                        .done()
                } else {
                    serving(end, plan)
                        .evidence(judged.clone(), today)
                        .worked(format!(
                            "{judged}. The men that can attack are too few: the staff sets {} to defend, where it rests, and waits.",
                            unit_words(&a.unit)
                        ))
                        .would_change("a smaller estimate, men rested, or more men landed")
                        .done()
                };
                if in_scope {
                    self.lines.push(format!("{} is set to defend.", a.unit.id));
                }
                orders.push(StaffOrder {
                    kind: OrderKind::LandUnit {
                        target: a.unit.id,
                        order: LandUnitOrder::SetPosture {
                            posture: Posture::Defend,
                            when_spent: None,
                        },
                    },
                    intent: end.id,
                    reason,
                });
            }
        }
        // Rule 6 as amended (11): where it waits with nothing more coming,
        // the notice that it cannot bring enough, on each morning that
        // holds; once on a morning for the plan.
        // A wait in the last days hands in no notice (decision 0194, 8): the
        // notice is the judgement's own wait with nothing more coming.
        let waits_with_nothing = nothing.is_some() && verdict == coming::Verdict::Wait;
        if orders.is_empty() || waits_with_nothing || committed || held_back {
            let worked = if held_back {
                let on_attack: Vec<String> = ops
                    .iter()
                    .filter(|a| a.unit.posture == Posture::Attack)
                    .map(|a| a.unit.id.to_string())
                    .collect();
                let verb = |one: &str, many: &str| {
                    if on_attack.len() == 1 { one } else { many }.to_string()
                };
                format!(
                    "{judged}. With fewer than {} days of the game left the staff begins no attack ashore, because an attack begun so late only costs men: it sets no unit on defend to attack{}.",
                    coming::LAST_DAYS,
                    if on_attack.is_empty() {
                        String::new()
                    } else if pulled {
                        format!(
                            ", and {} on attack, whose attack failed yesterday, {} set to defend",
                            on_attack.join(" and "),
                            verb("is", "are")
                        )
                    } else {
                        format!(
                            ", and {} on attack, whose attack did not fail, {} in",
                            on_attack.join(" and "),
                            verb("stays", "stay")
                        )
                    }
                )
            } else if committed {
                format!(
                    "{judged}. The units on attack are committed: {} {} sent in with nothing more coming, and the staff does not set {} to defend; it sets no other unit to attack.",
                    kept.iter().map(ToString::to_string).collect::<Vec<_>>().join(" and "),
                    if kept.len() == 1 { "was" } else { "were" },
                    if kept.len() == 1 { "it" } else { "them" }
                )
            } else if waits_with_nothing {
                format!(
                    "{judged}. Nothing more is coming and the men that can attack are too few: {}the staff waits, hands in the notice that it cannot bring enough, and waits for the commander.",
                    if orders.is_empty() {
                        ""
                    } else {
                        "the units on attack are set to defend; "
                    }
                )
            } else if attack {
                format!(
                    "{judged}. Every unit that can attack is on attack: the staff gives no order to the men ashore."
                )
            } else {
                format!("{judged}. The men that can attack are too few and none is on attack: the staff waits.")
            };
            let reason = serving(end, plan)
                .evidence(judged.clone(), today)
                .worked(worked)
                .would_change(if held_back {
                    "the place the side's"
                } else if nothing.is_some() {
                    "the place the side's, a new estimate, men rested, more men coming, or the risk"
                } else {
                    "the place the side's, a new estimate, men rested or landed"
                })
                .done();
            let handed = self
                .target(in_scope)
                .notices
                .iter()
                .any(|n| n.kind == NoticeKind::CannotBringEnough && n.plan == plan.id);
            if waits_with_nothing && !handed {
                self.notice(in_scope, reason, NoticeKind::CannotBringEnough, plan);
            } else {
                self.target(in_scope).no_orders.push(reason);
            }
        }
        self.target(in_scope).orders.extend(orders);

        // Decision 0152, 5 and 6: each later lift as it stands goes.
        if let Some(stand) = stand {
            self.send_later_lifts(end, plan, task, place, stand, &judged);
        }

        // Items 9 to 13: a second lift.
        if !lift.is_empty() {
            if under_way(force, place) || !crate::fleet::force_stopped_at(force, source_hex) {
                let reason = serving(end, plan)
                    .evidence(format!("{} is not stopped at the source {source}", force.id), today)
                    .worked(format!(
                        "{} is on its way with the second lift and its orders stand: the staff gives it no order.",
                        force.id
                    ))
                    .would_change("the force's orders changed")
                    .done();
                self.target(in_scope).no_orders.push(reason);
                return;
            }
            let mut supply = self.supply(
                force,
                &lift,
                source,
                source_hex,
                figures,
                None,
                self.draw_on(end, source),
            );
            // Decision 0169, 1 to 6: a second lift short of the men wanted
            // with too few in reach goes only where its men can be fed.
            let men = self.fighting_men(&lift);
            let fed = self.with_aboard(force, &lift);
            if let Some(starve) =
                self.starve_check(end, force, force.id, fit, men, &fed, supply.level)
            {
                if !starve.fed {
                    let words = format!(
                        "{judged}. A second lift: {} by {}",
                        lift.iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(" and "),
                        force.id
                    );
                    self.hand_in_wait(end, plan, &Waiting::Starve { starve, words });
                    return;
                }
                supply.words.push_str(&starve.goes_words());
            }
            self.move_step(
                end,
                plan,
                force,
                &lift,
                &supply,
                &LiftKind::Second,
                place,
                (source, source_hex),
            );
            return;
        }
        if picks.is_empty() {
            return;
        }
        // Decision 0169, 1 to 6: a second lift short of the men wanted with
        // too few in reach goes only where its men can be fed; where they
        // cannot, the units are not added to the limit and nothing is
        // ordered.
        let mut second_fed = String::new();
        {
            let pick_ids: Vec<LandUnitId> = picks.iter().map(|(id, _)| *id).collect();
            let men: u32 = picks.iter().map(|(_, men)| *men).sum();
            let level = self
                .supply(
                    force,
                    &pick_ids,
                    source,
                    source_hex,
                    figures,
                    None,
                    self.draw_on(end, source),
                )
                .level;
            let fed = self.with_aboard(force, &pick_ids);
            if let Some(starve) = self.starve_check(end, force, force.id, fit, men, &fed, level) {
                if !starve.fed {
                    let words = format!("{judged}. A second lift: {}", lift_lines.join("; "));
                    self.hand_in_wait(end, plan, &Waiting::Starve { starve, words });
                    return;
                }
                second_fed = starve.goes_words();
            }
        }
        // Item 12: the units of the second lift are kept as the task's
        // forces limit, by `set_limit` on the morning the lift is decided.
        let task = self
            .state
            .intents
            .get(&task.id)
            .cloned()
            .unwrap_or_else(|| task.clone());
        let task = &task;
        let mut limit = task.limits.forces.clone().unwrap_or(ForcesLimit {
            ships: BTreeSet::new(),
            task_forces: BTreeSet::new(),
            air_groups: BTreeSet::new(),
            source: None,
            land_units: BTreeSet::new(),
        });
        // A limit that names no unit binds the units above it (decision
        // 0149, 3): the second lift adds to those.
        if limit.land_units.is_empty() {
            limit.land_units = self.state.land_units_on_chain(task.id).unwrap_or_default();
        }
        limit.land_units.extend(picks.iter().map(|(id, _)| *id));
        if let Some(found) = self.state.intents.get_mut(&task.id) {
            found.limits.forces = Some(limit.clone());
        }
        let pick_ids: Vec<LandUnitId> = picks.iter().map(|(id, _)| *id).collect();
        let unit_list = limit
            .land_units
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        let mut supply = self.supply(
            force,
            &pick_ids,
            source,
            source_hex,
            figures,
            None,
            self.draw_on(end, source),
        );
        supply.words.push_str(&second_fed);
        let reason = serving(end, plan)
            .evidence(judged.clone(), today)
            .evidence(lift_lines.join("; "), today)
            .worked(format!(
                "{judged}. A second lift: {}; {}. The staff sets the forces limit of {} to {unit_list}.",
                lift_lines.join("; "),
                supply.words,
                task.id
            ))
            .would_change("the commander changing the limit")
            .done();
        self.target(in_scope).commands.push(StaffCommand {
            kind: CommandKind::SetLimit(Limit::Forces {
                intent: task.id,
                limit,
            }),
            reason,
        });
        if in_scope {
            self.lines.push(format!(
                "A second lift: {} by {}.",
                pick_ids
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" and "),
                force.id
            ));
        }
        self.move_step(
            end,
            plan,
            force,
            &pick_ids,
            &supply,
            &LiftKind::Second,
            place,
            (source, source_hex),
        );
    }

    /// Items 1 and 2 of decision 0143: the units of the operation ashore at
    /// the place, with the words for those left out, and the mark.
    fn units_ashore(
        &self,
        task_units: &[LandUnitId],
        place: Hex,
        risk: Risk,
        figures: &sim_core::refdata::GroundFigures,
    ) -> (Vec<Ashore>, Vec<String>, Fixed) {
        let view = self.view;
        // Item 2: the mark.
        let (num, den) = attack_share(risk);
        let mark = Fixed::from_int(
            figures
                .attack_fatigue_limit
                .mul_div(Fixed::from_int(num), Fixed::from_int(den))
                .trunc(),
        );
        // Item 1: the units of the operation, ashore at the place.
        let mut ops: Vec<Ashore> = Vec::new();
        let mut held: Vec<String> = Vec::new();
        for id in task_units {
            let Some(unit) = view.land_units().find(|u| u.id == *id) else {
                continue;
            };
            if !(unit.men_ashore > 0 && unit.hex == Some(place)) {
                continue;
            }
            if let Some(day) = crate::available::not_before(view, EntityRef::LandUnit(*id)) {
                held.push(crate::available::named_with_day(unit_words(unit), day));
                continue;
            }
            let target = HoldTarget::LandUnit(*id);
            if let Some(found) = self.state.hold_of(target) {
                held.push(format!(
                    "{} ({} men ashore) is left out: {}",
                    unit_words(unit),
                    unit.men_ashore,
                    hold_words(target, *found)
                ));
                continue;
            }
            ops.push(Ashore {
                can: unit.posture == Posture::Attack || unit.fatigue <= mark,
                fit: fit_men(unit),
                unit: unit.clone(),
            });
        }
        ops.sort_by_key(|a| a.unit.id);
        (ops, held, mark)
    }

    /// Decision 0152, 5: on a morning the plan is released, no unit of the
    /// operation is ashore at the place and the first lift `first_id` has
    /// been there or is lost (`first` is `None`), the later lifts as they
    /// stand. Whether it gave a reason or orders: `false` where the side has
    /// no lift as it stands besides the first, and the morning reads as it
    /// did.
    pub(crate) fn later_lifts_alone(
        &mut self,
        end: &Intent,
        plan: &Plan,
        task: &Intent,
        place: Hex,
        first: Option<&TaskForce>,
        first_id: TaskForceId,
    ) -> bool {
        let risk = self.state.risk_read_up(end.id);
        let wanted = men_wanted(self.view, place, risk);
        // No man of the operation is ashore at the place: the fit men are
        // none.
        let wait = wanted
            .filter(|w| *w == 0)
            .map(|_| "the men wanted are none".to_string());
        let Some(stand) = self.later_lift_plan(end, first_id, wait.as_deref()) else {
            return false;
        };
        let lead = match first {
            Some(force) => format!(
                "{} has been to {place} and no unit of the operation is ashore there",
                force.id
            ),
            None => format!("{first_id}, the first lift, is lost"),
        };
        let judged = format!(
            "{lead}; {}; the forces with men aboard, in force order: {}",
            match wanted {
                Some(w) => format!("the staff wants {w} men"),
                None => format!("the side holds no estimate of the enemy's men at {place}"),
            },
            stand.words.join("; ")
        );
        if stand.send.is_empty() {
            let in_scope = self.in_scope(end.id);
            let reason = serving(end, plan)
                .evidence(judged.clone(), self.today)
                .worked(format!(
                    "{judged}. No later lift as it stands can be sent: the staff gives no order."
                ))
                .would_change("a force with fighting men aboard, free and usable")
                .done();
            self.target(in_scope).no_orders.push(reason);
            return true;
        }
        self.send_later_lifts(end, plan, task, place, stand, &judged);
        true
    }

    /// Decision 0152, 6: each later lift as it stands is routed to the place
    /// and back to its source, marked to act there, with an unload order;
    /// its force and fighting units are added to the forces limit of "ready
    /// a landing" (decision 0143, 12, and the architect's ruling of Nov 18,
    /// 2026: the force too, since its orders are checked against it). It is
    /// not set to attack before it sails.
    pub(crate) fn send_later_lifts(
        &mut self,
        end: &Intent,
        plan: &Plan,
        task: &Intent,
        place: Hex,
        stand: crate::lifts::LiftPlan,
        judged: &str,
    ) {
        if stand.send.is_empty() {
            return;
        }
        let mut stand = stand;
        // Decision 0169, 1 to 6: each later lift, short of the men wanted
        // with too few in reach, goes only where its men can be fed. The
        // forces that do not go are given nothing and not added to the limit.
        let mut fed_words: std::collections::BTreeMap<TaskForceId, String> =
            std::collections::BTreeMap::new();
        if let (IntentKind::ReadyLanding { force: first, .. }, Some(figures)) =
            (task.kind, self.view.ground_figures())
        {
            let risk = self.state.risk_read_up(end.id);
            let task_units: Vec<LandUnitId> = self
                .state
                .land_units_on_chain(task.id)
                .map(|f| f.into_iter().collect())
                .unwrap_or_default();
            let (ops, _, _) = self.units_ashore(&task_units, place, risk, figures);
            let fit: u32 = ops.iter().filter(|a| a.can).map(|a| a.fit).sum();
            let mut kept = Vec::new();
            for send in std::mem::take(&mut stand.send) {
                let men: u32 = self
                    .aboard(send.force.id)
                    .into_iter()
                    .filter(|(u, _)| send.fighting.contains(&u.id))
                    .map(|(_, men)| men)
                    .sum();
                let aboard = self
                    .view
                    .ships()
                    .filter(|s| s.location == ShipLocation::TaskForce(send.force.id))
                    .fold(Fixed::ZERO, |sum, s| sum + s.supply_t);
                let fed = self.with_aboard(&send.force, &send.fighting);
                match self.starve_check(end, &send.force, first, fit, men, &fed, aboard) {
                    Some(starve) if !starve.fed => {
                        let words = format!(
                            "{judged}. {} is a later lift as it stands, with {} aboard",
                            send.force.id, send.words
                        );
                        self.hand_in_wait(end, plan, &Waiting::Starve { starve, words });
                    }
                    Some(starve) => {
                        fed_words.insert(send.force.id, starve.goes_words());
                        kept.push(send);
                    }
                    None => kept.push(send),
                }
            }
            stand.send = kept;
            if stand.send.is_empty() {
                return;
            }
        }
        let in_scope = self.in_scope(end.id);
        let today = self.today;
        let current = self
            .state
            .intents
            .get(&task.id)
            .cloned()
            .unwrap_or_else(|| task.clone());
        let mut limit = current.limits.forces.clone().unwrap_or(ForcesLimit {
            ships: BTreeSet::new(),
            task_forces: BTreeSet::new(),
            air_groups: BTreeSet::new(),
            source: None,
            land_units: BTreeSet::new(),
        });
        if limit.land_units.is_empty() {
            limit.land_units = self.state.land_units_on_chain(task.id).unwrap_or_default();
        }
        for send in &stand.send {
            limit.task_forces.insert(send.force.id);
            limit.land_units.extend(send.fighting.iter().copied());
        }
        let list = stand
            .send
            .iter()
            .map(|s| s.force.id.to_string())
            .collect::<Vec<_>>()
            .join(" and ");
        let reason = serving(end, plan)
            .evidence(judged.to_string(), today)
            .worked(format!(
                "{judged}. Later lifts as they stand: {list}. The staff sets the forces limit of {} to {}.",
                task.id,
                crate::assemble::limit_words(&limit)
            ))
            .would_change("the commander changing the limit")
            .done();
        if let Some(found) = self.state.intents.get_mut(&task.id) {
            found.limits.forces = Some(limit.clone());
        }
        self.target(in_scope).commands.push(StaffCommand {
            kind: CommandKind::SetLimit(Limit::Forces {
                intent: task.id,
                limit,
            }),
            reason,
        });
        let end_now = self
            .state
            .intents
            .get(&end.id)
            .cloned()
            .unwrap_or_else(|| end.clone());
        for send in stand.send {
            self.lifts_sent.insert(send.force.id, end.id);
            let aboard = self
                .view
                .ships()
                .filter(|s| s.location == ShipLocation::TaskForce(send.force.id))
                .fold(Fixed::ZERO, |sum, s| sum + s.supply_t);
            let supply = Supply {
                level: aboard,
                aboard,
                words: format!("{} of supply aboard", tons(aboard)),
            };
            let head = format!(
                "A later lift as it stands: {} has {} aboard and nothing to load ({} of supply aboard); everyone aboard lands. Its units are not set to attack before it sails: the judgement of each morning sets them once they are ashore.{}",
                send.force.id,
                send.words,
                tons(aboard),
                fed_words.get(&send.force.id).map_or("", String::as_str)
            );
            if in_scope {
                self.lines.push(format!(
                    "A later lift as it stands: {} with {}.",
                    send.force.id,
                    send.all
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" and ")
                ));
            }
            self.move_step(
                &end_now,
                plan,
                &send.force,
                &send.fighting,
                &supply,
                &LiftKind::AsItStands(head),
                place,
                send.source,
            );
        }
    }

    /// Item 9: the units left behind, in the order of decision 0142, item
    /// 8, with words for those of the forces limit that are not.
    fn left_behind(
        &self,
        end: &Intent,
        task: &Intent,
        source_hex: Hex,
    ) -> (Vec<LandUnit>, Vec<String>) {
        let view = self.view;
        // Both as the limits bind (decision 0149, 3).
        let chosen: BTreeSet<LandUnitId> =
            self.state.land_units_on_chain(task.id).unwrap_or_default();
        let named: Vec<LandUnitId> = self
            .state
            .forces_as_bound(end)
            .map(|f| f.land_units.iter().copied().collect())
            .unwrap_or_default();
        let mut found = Vec::new();
        let mut lines = Vec::new();
        for id in named {
            if chosen.contains(&id) {
                continue;
            }
            let Some(unit) = view.land_units().find(|u| u.id == id) else {
                // A fighting unit held apart until its day (decision 0195,
                // 13) is coming: named with its day.
                if let Some(day) = crate::available::entering_on(view, EntityRef::LandUnit(id)) {
                    let fighting = view
                        .entry_of(EntityRef::LandUnit(id))
                        .and_then(|entry| entry.land_units.get(&id))
                        .is_some_and(|u| !u.working);
                    if fighting {
                        lines.push(crate::available::enters_words(id, day));
                    }
                }
                continue;
            };
            if unit.working {
                continue;
            }
            if let Some(day) = crate::available::not_before(view, EntityRef::LandUnit(id)) {
                lines.push(crate::available::named_with_day(unit_words(unit), day));
                continue;
            }
            let target = HoldTarget::LandUnit(id);
            if let Some(hold) = self.state.hold_of(target) {
                lines.push(format!(
                    "{}; the staff plans without it",
                    hold_words(target, *hold)
                ));
                continue;
            }
            let all_ashore = unit.men_ashore > 0
                && unit.hex == Some(source_hex)
                && view.men_aboard(id).unwrap_or(0) == 0;
            if !all_ashore {
                lines.push(format!(
                    "{} is not all ashore at the source {source_hex}",
                    unit_words(unit)
                ));
                continue;
            }
            found.push(unit.clone());
        }
        found.sort_by(|a, b| {
            b.skills
                .assault
                .cmp(&a.skills.assault)
                .then(b.experience.cmp(&a.experience))
                .then(a.id.cmp(&b.id))
        });
        (found, lines)
    }

    /// Item 10: the units of a second lift, by the ships of decision 0142,
    /// item 9, until the fit men ashore that can attack and the men taken
    /// are at least the men wanted or no ship is left; with no estimate,
    /// every unit the ships can carry. In number order.
    fn lift_picks(
        &self,
        force: &TaskForce,
        left: &[LandUnit],
        wanted: Option<u32>,
        have: u32,
        lines: &mut Vec<String>,
    ) -> Vec<(LandUnitId, u32)> {
        let view = self.view;
        let mut ships: Vec<(ShipId, u32)> = view
            .ships()
            .filter(|ship| ship.location == ShipLocation::TaskForce(force.id))
            .filter_map(|ship| {
                let class = view.ship_class(ship.class)?;
                (class.troop_capacity_men > 0).then_some((ship.id, class.troop_capacity_men))
            })
            .collect();
        ships.sort_by_key(|(id, _)| *id);
        let mut taken: BTreeSet<ShipId> = BTreeSet::new();
        let mut picks: Vec<(LandUnitId, u32)> = Vec::new();
        let mut men_taken = 0u32;
        for unit in left {
            if let Some(w) = wanted {
                if have + men_taken >= w {
                    lines.push(format!(
                        "{} is not needed: the {have} fit men ashore that can attack and the {men_taken} taken are at least the {w} wanted",
                        unit_words(unit)
                    ));
                    continue;
                }
            }
            let men = unit.men_ashore;
            let free: Vec<(ShipId, u32)> = ships
                .iter()
                .filter(|(id, _)| !taken.contains(id))
                .copied()
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
                lines.push(if free.is_empty() {
                    format!("{} ({men} men) is left out: no ship is left for it", unit_words(unit))
                } else {
                    let free_berths: u32 = free.iter().map(|(_, b)| *b).sum();
                    format!(
                        "{} ({men} men) is left out: the ships left have {free_berths} berths, {} short",
                        unit_words(unit),
                        men - free_berths
                    )
                });
                continue;
            }
            taken.extend(mine.iter().copied());
            men_taken += men;
            lines.push(format!(
                "{} ({men} men) takes {} {}",
                unit_words(unit),
                mine.len(),
                if mine.len() == 1 { "ship" } else { "ships" }
            ));
            picks.push((unit.id, men));
        }
        if let Some(w) = wanted {
            let total = have + men_taken;
            if total < w && !picks.is_empty() {
                lines.push(format!(
                    "with them the men are {total} against the {w} wanted, short by {}: it sends what it has",
                    w - total
                ));
            }
        } else {
            lines.push(
                "with no count of the enemy it sends every unit the ships can carry".to_string(),
            );
        }
        picks.sort_by_key(|(id, _)| *id);
        picks
    }
}

/// A lift timed to the dark (decision 0145): the base that sends it, the
/// two speeds tried, and each ship's rate at the place where men are landed,
/// in ship order.
struct LiftNight {
    near: String,
    risk: Risk,
    tried: Vec<crate::night::Tried>,
    /// Each ship of the force, its rate at the place (times the factor of
    /// the force's loading, where it lands men), the supply and the men's
    /// weight aboard, its room for supply and its berths left.
    ships: Vec<LiftShip>,
    handling: Fixed,
}

#[derive(Clone, Copy)]
struct LiftShip {
    ship: ShipId,
    rate: Fixed,
    supply: Fixed,
    men: u32,
    unit: Option<LandUnitId>,
    room: Fixed,
    berths: u32,
}

impl LiftNight {
    /// The weight each ship would land, the men by their weight and the
    /// supply together, against the rate it lands at.
    fn loads(&self, ships: &[LiftShip]) -> Vec<(Fixed, Fixed)> {
        ships
            .iter()
            .map(|s| (s.supply + self.handling * i64::from(s.men), s.rate))
            .collect()
    }

    fn limits_words(&self, ships: &[LiftShip], night: Fixed) -> String {
        ships
            .iter()
            .map(|s| {
                format!(
                    "{} lands {} t against a limit of {} t an hour x {night} hours = {} t",
                    s.ship,
                    s.supply + self.handling * i64::from(s.men),
                    s.rate,
                    s.rate * night
                )
            })
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// The ships with `unit`'s men ashore taken aboard in ship order into
    /// the berths each has left.
    fn with_unit(&self, unit: LandUnitId, men: u32) -> Vec<LiftShip> {
        let mut left = men;
        self.ships
            .iter()
            .map(|s| {
                let mut s = *s;
                if s.unit.is_none_or(|u| u == unit) {
                    let take = left.min(s.berths);
                    if take > 0 {
                        s.men += take;
                        s.berths -= take;
                        s.unit = Some(unit);
                        left -= take;
                    }
                }
                s
            })
            .collect()
    }

    /// The ships with the supply aboard raised to `level` in ship order.
    fn with_supply(&self, level: Fixed) -> Vec<LiftShip> {
        let aboard = self.ships.iter().fold(Fixed::ZERO, |sum, s| sum + s.supply);
        let mut left = (level - aboard).max(Fixed::ZERO);
        self.ships
            .iter()
            .map(|s| {
                let mut s = *s;
                let add = left.min(s.room);
                s.supply += add;
                s.room -= add;
                left -= add;
                s
            })
            .collect()
    }
}

impl Day<'_, '_> {
    /// Decision 0145, 2 to 6, for a lift of "take a place": where it goes by
    /// night, the speeds tried and the ships as they stand.
    fn lift_night(
        &self,
        end: &Intent,
        force: &TaskForce,
        place: Hex,
        waypoints: &[Hex],
    ) -> Option<LiftNight> {
        let view = self.view;
        let risk = self.state.risk_read_up(end.id);
        let (base, hexes) = crate::night::applies(view, risk, place)?;
        let figures = view.ground_figures()?;
        let near = crate::night::base_near_words(view, base, hexes, place);
        let tried = crate::night::try_force(view, force.id, place, waypoints);
        let port = view
            .hex(place)
            .and_then(|data| view.port_size(data.port_size));
        let factor = view
            .landing_rate_factor(force.id, place)
            .unwrap_or(Fixed::ONE);
        let mut found: Vec<&sim_core::state::Ship> = view
            .ships()
            .filter(|s| s.location == ShipLocation::TaskForce(force.id))
            .collect();
        found.sort_by_key(|s| s.id);
        let ships = found
            .iter()
            .enumerate()
            .map(|(index, s)| {
                let class = view.ship_class(s.class);
                let rate = port.map_or(Fixed::ZERO, |p| {
                    if (index as u32) < p.berths {
                        p.berth_rate_t_per_h
                    } else {
                        p.lighter_rate_t_per_h
                    }
                });
                let capacity = class.map_or(Fixed::ZERO, |c| c.cargo_capacity_t);
                let troops = class.map_or(0, |c| c.troop_capacity_men);
                let men = s.troops.map_or(0, |t| t.men);
                LiftShip {
                    ship: s.id,
                    rate: rate * factor,
                    supply: s.supply_t,
                    men,
                    unit: s.troops.map(|t| t.unit),
                    room: (capacity - s.supply_t - s.cargo_fuel_t).max(Fixed::ZERO),
                    berths: troops.saturating_sub(men),
                }
            })
            .collect();
        Some(LiftNight {
            near,
            risk,
            tried,
            ships,
            handling: figures.handling_t_per_man,
        })
    }
}
