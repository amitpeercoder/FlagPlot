//! Forces with men aboard (decision 0152): a transport force with men aboard
//! is a lift as it stands.
//!
//! - **A lift as it stands** (item 1): a transport force the staff may take
//!   with men aboard. Its units are those aboard it, whole; its source is its
//!   home port, or with none the nearest base of the side by the engine's sea
//!   path, the lower number on a tie. It is free where it has no route left
//!   to follow and is not landing: a force under way on an order keeps it
//!   (added by the architect on Nov 18, 2026, on the build's finding).
//! - **"Take a place"** takes one with a fighting unit aboard as its first
//!   lift before any base (item 2, in `assemble`), and sends each later one
//!   once its first lift is in (items 5 to 7, in `take`).
//! - **"Hold a place"** sends one no take can use to its base (item 8), here.
//!
//! The doctrine reads the side's own view and its planned command state and
//! nothing else.

use std::collections::BTreeSet;

use sim_core::command::{
    CommandKind, ForcesLimit, Intent, IntentKind, Limit, Maker, Plan, StaffCommand,
};
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, LandUnitId, TaskForceId};
use sim_core::orders::Mission;
use sim_core::state::{LandUnit, ShipLocation, TaskForce};

use crate::air::Day;
use crate::reasons::{base_words, serving};
use crate::take::{unit_words, LiftKind, Supply};
use crate::units::tons;

/// A lift as it stands the staff sends this morning.
pub(crate) struct Send {
    pub(crate) force: TaskForce,
    /// Its fighting units, in unit order.
    pub(crate) fighting: Vec<LandUnitId>,
    /// Every unit aboard, in unit order.
    pub(crate) all: Vec<LandUnitId>,
    pub(crate) source: (BaseId, Hex),
    /// What it carries, in words.
    pub(crate) words: String,
}

/// The lifts as they stand an intent surveyed this morning: the words for
/// each, and those it sends.
pub(crate) struct LiftPlan {
    pub(crate) words: Vec<String>,
    pub(crate) send: Vec<Send>,
}

impl Day<'_, '_> {
    /// The units with men aboard `force`, in unit order, with the men aboard
    /// it.
    pub(crate) fn aboard(&self, force: TaskForceId) -> Vec<(LandUnit, u32)> {
        let mut men: std::collections::BTreeMap<LandUnitId, u32> =
            std::collections::BTreeMap::new();
        for ship in self.view.ships() {
            if ship.location != ShipLocation::TaskForce(force) {
                continue;
            }
            if let Some(troops) = ship.troops {
                *men.entry(troops.unit).or_insert(0) += troops.men;
            }
        }
        men.into_iter()
            .filter_map(|(id, n)| {
                self.view
                    .land_units()
                    .find(|u| u.id == id)
                    .map(|u| (u.clone(), n))
            })
            .collect()
    }

    /// `units` and every other unit with men aboard `force`, in unit order:
    /// the men a lift feeds (decision 0152, 4).
    pub(crate) fn with_aboard(&self, force: &TaskForce, units: &[LandUnitId]) -> Vec<LandUnitId> {
        let mut all: BTreeSet<LandUnitId> = units.iter().copied().collect();
        all.extend(self.aboard(force.id).into_iter().map(|(u, _)| u.id));
        all.into_iter().collect()
    }

    /// The words for the units aboard `force` that are not `units`: they
    /// land with it and are not units of the operation (decision 0152, 3).
    /// Empty where there are none.
    pub(crate) fn also_aboard(&self, force: &TaskForce, units: &[LandUnitId]) -> String {
        let others: Vec<String> = self
            .aboard(force.id)
            .into_iter()
            .filter(|(u, _)| !units.contains(&u.id))
            .map(|(u, men)| {
                format!(
                    "{}, {men} men, {}",
                    unit_words(&u),
                    if u.working {
                        "a working unit"
                    } else {
                        "a fighting unit"
                    }
                )
            })
            .collect();
        if others.is_empty() {
            return String::new();
        }
        format!(
            " Also aboard {}: {}; everyone aboard lands, and {} not of the operation.",
            force.id,
            others.join("; "),
            if others.len() == 1 {
                "it is"
            } else {
                "they are"
            }
        )
    }

    /// What is aboard `force`, in words: "unit:1 (Name), 600 men, a fighting
    /// unit, and unit:2 (Name), 400 men, a working unit".
    pub(crate) fn aboard_words(&self, force: TaskForceId) -> String {
        self.aboard(force)
            .into_iter()
            .map(|(u, men)| {
                format!(
                    "{}, {men} men, {}",
                    unit_words(&u),
                    if u.working {
                        "a working unit"
                    } else {
                        "a fighting unit"
                    }
                )
            })
            .collect::<Vec<_>>()
            .join(", and ")
    }

    /// Whether `force` is a lift as it stands in kind: on the transport
    /// mission with men aboard a ship afloat (item 1).
    pub(crate) fn is_lift(&self, force: &TaskForce) -> bool {
        force.mission == Mission::Transport && !self.aboard(force.id).is_empty()
    }

    /// The side's lifts as they stand, in force order.
    pub(crate) fn lifts(&self) -> Vec<TaskForce> {
        let mut found: Vec<TaskForce> = self
            .view
            .task_forces()
            .filter(|f| self.is_lift(f))
            .cloned()
            .collect();
        found.sort_by_key(|f| f.id);
        found
    }

    /// Why `force` is not free, where it is not: under way on its orders, or
    /// landing (the architect's ruling on item 1, Nov 18, 2026).
    pub(crate) fn not_free(force: &TaskForce) -> Option<String> {
        if let Some(hex) = force.landing_at {
            return Some(format!("{} is landing at {hex}", force.id));
        }
        let route_left = force.route.next_index < force.route.waypoints.len();
        if route_left || force.toward_hex.is_some() || force.yard_trip.is_some() {
            return Some(format!(
                "{} is under way on its orders, which stand",
                force.id
            ));
        }
        None
    }

    /// Item 1: the source of a lift as it stands: its home port where that
    /// is the side's, else the nearest base of the side by the engine's sea
    /// path, the lower number on a tie.
    pub(crate) fn lift_source(&self, force: &TaskForce) -> Option<(BaseId, Hex)> {
        if let Some(port) = force.home_port {
            if let Some(base) = self.view.bases().find(|b| b.id == port) {
                return Some((base.id, base.hex));
            }
        }
        let from = force.position();
        self.view
            .bases()
            .filter_map(|b| {
                let nm = if b.hex == from {
                    0
                } else {
                    crate::route::route_between(self.view, from, b.hex, None)?.out_nm
                };
                Some((nm, b.id, b.hex))
            })
            .min()
            .map(|(_, id, hex)| (id, hex))
    }

    /// The take whose landing has `force` as its lift, other than `end`,
    /// where its plan has not ended: the force is that take's.
    pub(crate) fn lift_of_take(
        &self,
        force: TaskForceId,
        end: Option<IntentId>,
    ) -> Option<IntentId> {
        self.state.intents_in_force().find_map(|task| {
            let IntentKind::ReadyLanding { force: own, .. } = task.kind else {
                return None;
            };
            let named = own == force
                || task
                    .limits
                    .forces
                    .as_ref()
                    .is_some_and(|f| f.task_forces.contains(&force));
            let take = task.parent?;
            if !named || Some(take) == end {
                return None;
            }
            let ended = self
                .state
                .intents
                .get(&take)
                .and_then(|t| self.state.plans.get(&t.plan))
                .is_none_or(|p| p.state.ended());
            (!ended).then_some(take)
        })
    }

    /// The takes in force for a place not yet the side's, whose plan has not
    /// ended (item 8): those that may use a lift with fighting men aboard.
    pub(crate) fn takes_wanting(&self) -> Vec<IntentId> {
        self.state
            .intents_in_force()
            .filter_map(|end| match end.kind {
                IntentKind::TakePlace { hex, .. } => Some((end, hex)),
                _ => None,
            })
            .filter(|(end, hex)| {
                self.state
                    .plans
                    .get(&end.plan)
                    .is_some_and(|p| !p.state.ended())
                    && !self.place_is_ours(*hex)
            })
            .map(|(end, _)| end.id)
            .collect()
    }

    /// Item 4: whether the staff may draw on `source` for the lift of `end`:
    /// a source the commander named, his or his island's; else one the
    /// staff may take (decision 0149, 5). With an `Err`, why not.
    pub(crate) fn draw_on(&self, end: &Intent, source: BaseId) -> Result<(), String> {
        let named = end
            .limits
            .forces
            .as_ref()
            .and_then(|f| f.source)
            .or_else(|| {
                self.state
                    .island_on_chain(end.id)
                    .and_then(|island| island.limits.forces.as_ref()?.source)
            });
        if named == Some(source) {
            return Ok(());
        }
        let others = self.others_above(end);
        self.may_take_base(source, &others)
    }

    /// The common checks of a lift as it stands for `intent` (items 5 and
    /// 8): `Err(words)` where it may not be sent.
    fn lift_checks(
        &self,
        intent: &Intent,
        force: &TaskForce,
        others: &crate::assemble::Others,
    ) -> Result<(BaseId, Hex), String> {
        self.may_take_force(force, others)?;
        if let Some(other) = self.lifts_sent.get(&force.id) {
            if *other != intent.id {
                return Err(format!("{} is sent by {other} this morning", force.id));
            }
        }
        let own_take = matches!(intent.kind, IntentKind::TakePlace { .. }).then_some(intent.id);
        if let Some(take) = self.lift_of_take(force.id, own_take) {
            return Err(format!("{} is the lift of {take}", force.id));
        }
        if let Some(why) = Self::not_free(force) {
            return Err(why);
        }
        self.lift_source(force)
            .ok_or_else(|| format!("{} has no base of the side to come back to", force.id))
    }

    /// Item 5: the later lifts as they stand of the take `end`, besides its
    /// first lift `first`, with `wait` the reason they wait where the first
    /// lift is not in or the men are enough. `None` where the side has no
    /// lift as it stands besides the first.
    pub(crate) fn later_lift_plan(
        &self,
        end: &Intent,
        first: TaskForceId,
        wait: Option<&str>,
    ) -> Option<LiftPlan> {
        let forces: Vec<TaskForce> = self.lifts().into_iter().filter(|f| f.id != first).collect();
        if forces.is_empty() {
            return None;
        }
        let others = self.others_above(end);
        // Inside what the commander named, his or his island's (decision
        // 0151, 4).
        let within = end.limits.forces.clone().or_else(|| {
            self.state
                .island_on_chain(end.id)
                .and_then(|island| island.limits.forces.clone())
        });
        let named = within.filter(crate::assemble::names_forces);
        let mut plan = LiftPlan {
            words: Vec::new(),
            send: Vec::new(),
        };
        for force in forces {
            let aboard = self.aboard_words(force.id);
            if named.as_ref().is_some_and(|f| {
                !(f.task_forces.contains(&force.id)
                    || (!force.ships.is_empty() && force.ships.iter().all(|s| f.ships.contains(s))))
            }) {
                plan.words
                    .push(format!("{} is not in the forces limit", force.id));
                continue;
            }
            let source = match self.lift_checks(end, &force, &others) {
                Ok(source) => source,
                Err(why) => {
                    plan.words.push(why);
                    continue;
                }
            };
            let fighting: Vec<LandUnitId> = self
                .aboard(force.id)
                .into_iter()
                .filter(|(u, _)| !u.working)
                .map(|(u, _)| u.id)
                .collect();
            if fighting.is_empty() {
                plan.words.push(format!(
                    "{} has {aboard} aboard, all of working units",
                    force.id
                ));
                continue;
            }
            if let Some(wait) = wait {
                plan.words
                    .push(format!("{}, with {aboard} aboard, waits: {wait}", force.id));
                continue;
            }
            plan.words.push(format!(
                "{}, with {aboard} aboard, is a later lift as it stands and is sent now",
                force.id
            ));
            plan.send.push(Send {
                all: self
                    .aboard(force.id)
                    .into_iter()
                    .map(|(u, _)| u.id)
                    .collect(),
                fighting,
                source,
                words: aboard,
                force,
            });
        }
        Some(plan)
    }

    /// Item 8: the lifts as they stand the hold `hold` may send to its base
    /// at `hex`. `None` where the side has no lift as it stands.
    fn hold_lift_plan(&self, hold: &Intent) -> Option<LiftPlan> {
        let forces = self.lifts();
        if forces.is_empty() {
            return None;
        }
        let others = self.others_above(hold);
        let takes = self.takes_wanting();
        let limits = self.state.forces_on_chain(hold.id);
        let mut plan = LiftPlan {
            words: Vec::new(),
            send: Vec::new(),
        };
        for force in forces {
            // A take's own lift is the take's: the hold says nothing of it.
            if self.lift_of_take(force.id, None).is_some() {
                continue;
            }
            let aboard = self.aboard_words(force.id);
            let source = match self.lift_checks(hold, &force, &others) {
                Ok(source) => source,
                Err(why) => {
                    plan.words.push(why);
                    continue;
                }
            };
            let fighting: Vec<LandUnitId> = self
                .aboard(force.id)
                .into_iter()
                .filter(|(u, _)| !u.working)
                .map(|(u, _)| u.id)
                .collect();
            if let (false, Some(take)) = (fighting.is_empty(), takes.first()) {
                plan.words.push(format!(
                    "{} has fighting men aboard and {take} is in force for a place not yet the side's: it is the takes'",
                    force.id
                ));
                continue;
            }
            // A hold the commander made sends only what his limit allows; a
            // hold the staff made has the force added to the limit it sets.
            if hold.maker == Maker::Commander {
                let inside = !limits.is_empty()
                    && limits.iter().all(|f| {
                        f.task_forces.contains(&force.id)
                            || (!force.ships.is_empty()
                                && force.ships.iter().all(|s| f.ships.contains(s)))
                    });
                if !inside {
                    plan.words.push(format!(
                        "{} is not in the forces limit of {}",
                        force.id, hold.id
                    ));
                    continue;
                }
            }
            plan.words
                .push(format!("{}, with {aboard} aboard, is sent", force.id));
            plan.send.push(Send {
                all: self
                    .aboard(force.id)
                    .into_iter()
                    .map(|(u, _)| u.id)
                    .collect(),
                fighting,
                source,
                words: aboard,
                force,
            });
        }
        Some(plan)
    }

    /// Item 8: on a morning the hold's plan is released and its base is the
    /// side's, each lift as it stands no take can use, in force order, that
    /// no hold higher in priority has taken, is routed to the base and back
    /// to its source, marked to act there, with an unload order.
    pub(crate) fn hold_lifts(&mut self, hold: &Intent, plan: &Plan, base: BaseId, in_scope: bool) {
        let Some(hex) = self.view.bases().find(|b| b.id == base).map(|b| b.hex) else {
            return;
        };
        let Some(found) = self.hold_lift_plan(hold) else {
            return;
        };
        let today = self.today;
        let base_text = base_words(self.view, base);
        let survey = found.words.join("; ");
        if found.send.is_empty() && found.words.is_empty() {
            return;
        }
        if found.send.is_empty() {
            let reason = serving(hold, plan)
                .evidence(survey.clone(), today)
                .worked(format!(
                    "{} holds {base_text}, which is the side's. The forces with men aboard: {survey}. The staff sends none to {base_text}.",
                    hold.id
                ))
                .would_change("a force with men aboard that no take can use, free and usable")
                .done();
            self.target(in_scope).no_orders.push(reason);
            return;
        }
        // A hold the staff formed has each force it sends added to its
        // forces limit, so that its orders are inside it (the architect's
        // ruling on item 8, Nov 18, 2026).
        if hold.maker == Maker::Staff {
            let mut limit = hold.limits.forces.clone().unwrap_or(ForcesLimit {
                ships: BTreeSet::new(),
                task_forces: BTreeSet::new(),
                air_groups: BTreeSet::new(),
                source: None,
                land_units: BTreeSet::new(),
                submarines: Default::default(),
                none_of: Default::default(),
            });
            let before = limit.clone();
            limit
                .task_forces
                .extend(found.send.iter().map(|s| s.force.id));
            if limit != before {
                let list = found
                    .send
                    .iter()
                    .map(|s| s.force.id.to_string())
                    .collect::<Vec<_>>()
                    .join(" and ");
                let reason = serving(hold, plan)
                    .evidence(survey.clone(), today)
                    .worked(format!(
                        "{} holds {base_text}, which is the side's, and sends {list}, with men aboard that no take can use: the staff sets the forces limit of {} to {}.",
                        hold.id,
                        hold.id,
                        crate::assemble::limit_words(&limit)
                    ))
                    .would_change("the commander changing the limit")
                    .done();
                if let Some(intent) = self.state.intents.get_mut(&hold.id) {
                    intent.limits.forces = Some(limit.clone());
                }
                self.target(in_scope).commands.push(StaffCommand {
                    kind: CommandKind::SetLimit(Limit::Forces {
                        intent: hold.id,
                        limit,
                    }),
                    reason,
                });
            }
        }
        let hold_now = self
            .state
            .intents
            .get(&hold.id)
            .cloned()
            .unwrap_or_else(|| hold.clone());
        let no_take = if self.takes_wanting().is_empty() {
            "no take is in force for a place not yet the side's"
        } else {
            "a take is in force, but its men are all of working units"
        };
        for send in found.send {
            self.lifts_sent.insert(send.force.id, hold.id);
            let supply_aboard = send
                .force
                .ships
                .iter()
                .filter_map(|id| self.view.ships().find(|s| s.id == *id))
                .fold(sim_core::fixed::Fixed::ZERO, |sum, s| sum + s.supply_t);
            let supply = Supply {
                level: supply_aboard,
                aboard: supply_aboard,
                words: format!("{} of supply aboard", tons(supply_aboard)),
            };
            let head = format!(
                "{} holds {base_text}, which is the side's: {} is a force with men aboard ({}) and {}, so the hold sends it to {base_text} as it stands, with {} of supply aboard and nothing to load; everyone aboard lands.",
                hold.id,
                send.force.id,
                send.words,
                no_take,
                tons(supply_aboard)
            );
            if in_scope {
                self.lines.push(format!(
                    "{} holds {base_text}: it sends {} with {} aboard.",
                    hold.id, send.force.id, send.words
                ));
            }
            self.move_step(
                &hold_now,
                plan,
                &send.force,
                &send.fighting,
                &supply,
                &LiftKind::Hold(head),
                hex,
                send.source,
            );
        }
    }
}
