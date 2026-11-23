//! Controlling an island (decision 0151, step 8 of stage 3, first part): the
//! tree the staff forms each morning under "control an island", in the
//! record's order (item 6), before the doctrines that serve what it forms.
//!
//! - **Withdrawn first** (item 8b): a "neutralise a base" the staff formed
//!   under the island whose target is now the side's is withdrawn.
//! - **In the order of the island's places:** "take a place" where the side
//!   holds no base and no take of the place is in force with a plan that has
//!   not ended, with the island's date and a plan of its own (item 9, the
//!   architect's addition), made ready that morning on leave to prepare;
//!   "hold a place" of the side's base there, else of the enemy's
//!   lowest-numbered base there on the side's map, else none until the side
//!   has a base there; its forces limit set to the source of supply (item 8).
//! - **Deny the waters** within 3 hexes of the first place, once.
//! - **Operate a carrier force** under the denial for each carrier force the
//!   staff may take with a carrier afloat that has an air group aboard, at the
//!   covering hex of decision 0148, item 2, reckoned to the first place.
//! - **Neutralise a base** for each enemy base on the side's map within 15
//!   hexes of a named place, in base order.
//!
//! An intent of the kind and place already in force anywhere on the side,
//! whoever made it, counts, and none is formed beside it: the reason names
//! the one that stands. The doctrine reads the side's own view, its area
//! picture for the forces, and its planned command state.

use std::collections::{BTreeMap, BTreeSet};

use sim_core::command::{
    CommandKind, ForcesLimit, Intent, IntentKind, IntentLimits, Limit, Maker, Plan, PlanId,
    PlanState, StaffCommand,
};
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, TaskForceId};
use sim_core::orders::Mission;
use sim_core::state::AirGroupLocation;

use crate::air::Day;
use crate::cover::NoCoveringHex;
use crate::reasons::{base_words, serving};
use crate::stand::afloat;

/// The waters an island's denial covers, in hexes from its first place
/// (decision 0151, 6): the staff's figure, a placeholder.
pub(crate) const DENY_HEXES: u32 = 3;

/// How near a named place an enemy base is neutralised, in hexes (decision
/// 0151, 6): the placeholder of decision 0124.
pub(crate) const NEUTRALISE_HEXES: u32 = 15;

/// One named place as the staff reads it: its hex and the base the island
/// names there, if any.
#[derive(Clone, Copy)]
struct Place {
    hex: Hex,
}

impl Day<'_, '_> {
    /// Decision 0151, items 6 to 8b, for each "control an island" whose plan
    /// is released, in priority order.
    pub(crate) fn islands(&mut self) {
        let ends = self.in_priority_order(|i| matches!(i.kind, IntentKind::ControlIsland { .. }));
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
            self.island_tree(&end, &plan);
        }
    }

    /// The tree of one island on this morning.
    #[allow(clippy::too_many_lines)]
    fn island_tree(&mut self, end: &Intent, plan: &Plan) {
        let IntentKind::ControlIsland { island, by_day } = end.kind else {
            return;
        };
        let in_scope = self.in_scope(end.id);
        let today = self.today;
        let Some(found) = self.view.island(island) else {
            let reason = serving(end, plan)
                .evidence(format!("island {island} is not on the map"), today)
                .worked(format!(
                    "{} names island {island}, which the map does not have, so the staff forms nothing under it.",
                    end.id
                ))
                .would_change("the island on the map")
                .done();
            self.target(in_scope).no_orders.push(reason);
            return;
        };
        let island_text = format!("island {island} ({})", found.name);
        let places: Vec<Place> = found.places.iter().map(|p| Place { hex: p.hex }).collect();
        let Some(first) = places.first().copied() else {
            return;
        };

        // Item 8b: a neutralising that has done its work.
        self.withdraw_done(end, plan, in_scope);

        // Item 8: the source of supply for the holds, chosen on the first
        // morning a base fits and kept (decision 0152, 10).
        let source = self.island_source(end, &places);

        // Item 6, in the order of the island's places.
        for place in &places {
            self.island_take(end, plan, place.hex, by_day, &island_text, in_scope);
            self.island_hold(end, plan, place.hex, &source, &island_text, in_scope);
        }

        // Decision 0152, 11: each hold's forces limit set each morning to
        // the source and its pool.
        self.island_pools(end, plan, &source, in_scope);

        // The denial, once, and the carrier forces under it.
        let deny = self.island_deny(end, plan, first.hex, &island_text, in_scope);
        if let Some(deny) = deny {
            self.island_carriers(end, &deny, first.hex, &island_text, in_scope);
        }

        // The neutralising of each enemy base near a named place.
        self.island_neutralise(end, plan, &places, &island_text, in_scope);
    }

    /// Forms `kind` under `parent` in the planned state and hands in the
    /// command, returning the new intent's number. A take under an island has
    /// a plan of its own (decision 0151, 9), drafted here as intake drafts it.
    fn form_under(
        &mut self,
        parent: &Intent,
        kind: IntentKind,
        reason: sim_core::command::Reason,
        in_scope: bool,
    ) -> IntentId {
        let today = self.today;
        let id_new = self.state.next_intent_number();
        let own_plan = self.state.forms_own_plan(kind, parent.id);
        let plan = if own_plan {
            let plan_id = PlanId::new(self.state.next_plan);
            self.state.next_plan += 1;
            self.state.plans.insert(
                plan_id,
                Plan {
                    id: plan_id,
                    intent: id_new,
                    state: PlanState::Draft,
                    baseline: None,
                    amendments: Vec::new(),
                    runs: BTreeMap::new(),
                },
            );
            plan_id
        } else {
            parent.plan
        };
        self.state.intents.insert(
            id_new,
            Intent {
                id: id_new,
                kind,
                parent: Some(parent.id),
                maker: Maker::Staff,
                addressee: kind.addressee(),
                limits: IntentLimits::default(),
                set_day: today,
                plan,
                withdrawn_day: None,
            },
        );
        self.target(in_scope).commands.push(StaffCommand {
            kind: CommandKind::FormIntent {
                parent: parent.id,
                kind,
            },
            reason,
        });
        id_new
    }

    /// The reason given where an intent of the kind and place already stands
    /// (item 6): none is formed beside it.
    fn island_stands(
        &mut self,
        end: &Intent,
        plan: &Plan,
        other: &Intent,
        words: &str,
        in_scope: bool,
    ) {
        // What the island's own tree formed stands without a word.
        let root = self.state.root_of(end.id);
        if self.state.root_of(other.id) == root {
            return;
        }
        let by = match other.maker {
            Maker::Commander => "the commander's",
            Maker::Staff => "the staff's",
        };
        let reason = serving(end, plan)
            .evidence(format!("{}, {by}, is in force", other.id), self.today)
            .binding(format!("{} stands", other.id))
            .worked(format!(
                "{}: {}, {by} intent to {}, is in force, so the staff forms none beside it.",
                end.id, other.id, words
            ))
            .would_change(format!("{} withdrawn", other.id))
            .done();
        self.target(in_scope).no_orders.push(reason);
    }

    /// Item 8b: each "neutralise a base" the staff formed under the island
    /// whose target is the side's this morning is withdrawn.
    fn withdraw_done(&mut self, end: &Intent, plan: &Plan, in_scope: bool) {
        let today = self.today;
        let done: Vec<(IntentId, BaseId)> = self
            .state
            .intents_in_force()
            .filter(|i| i.parent == Some(end.id) && i.maker == Maker::Staff)
            .filter_map(|i| match i.kind {
                IntentKind::Neutralise { target } => Some((i.id, target)),
                _ => None,
            })
            .filter(|(_, target)| self.view.bases().any(|b| b.id == *target))
            .collect();
        for (id, target) in done {
            let target_text = base_words(self.view, target);
            let reason = serving(end, plan)
                .evidence(format!("{target_text} is the side's"), today)
                .binding(format!("{id} has done its work"))
                .worked(format!(
                    "{target_text} is the side's this morning, so the staff withdraws {id}, the neutralising of it it formed under {}.",
                    end.id
                ))
                .would_change(format!("{target_text} the enemy's"))
                .done();
            self.target(in_scope).commands.push(StaffCommand {
                kind: CommandKind::WithdrawIntent { intent: id },
                reason,
            });
            if in_scope {
                self.lines.push(format!(
                    "{} would withdraw {id}: {target_text} is the side's.",
                    end.id
                ));
            }
            for below in self.state.descendants_in_force(id) {
                if let Some(found) = self.state.intents.get_mut(&below) {
                    found.withdrawn_day = Some(today);
                }
            }
            if let Some(found) = self.state.intents.get_mut(&id) {
                found.withdrawn_day = Some(today);
            }
        }
    }

    /// The hex of a base on the side's map, the side's or the enemy's.
    fn hex_of(&self, base: BaseId) -> Option<Hex> {
        self.view
            .bases()
            .find(|b| b.id == base)
            .map(|b| b.hex)
            .or_else(|| {
                self.view
                    .enemy_bases()
                    .find(|b| b.id == base)
                    .map(|b| b.hex)
            })
    }

    /// Item 6: "take a place" for a named place where the side holds no base.
    fn island_take(
        &mut self,
        end: &Intent,
        plan: &Plan,
        hex: Hex,
        by_day: Option<u32>,
        island_text: &str,
        in_scope: bool,
    ) {
        if self.view.bases().any(|b| b.hex == hex) {
            return;
        }
        let standing = self
            .state
            .intents_in_force()
            .filter(|i| matches!(i.kind, IntentKind::TakePlace { hex: h, .. } if h == hex))
            .find(|i| {
                self.state
                    .plans
                    .get(&i.plan)
                    .is_some_and(|p| !p.state.ended())
            })
            .cloned();
        if let Some(other) = standing {
            self.island_stands(
                end,
                plan,
                &other,
                &format!("take the place at {hex}"),
                in_scope,
            );
            return;
        }
        let kind = IntentKind::TakePlace { hex, by_day };
        let date = by_day.map_or_else(String::new, |day| format!(" by the end of day {day}"));
        let reason = serving(end, plan)
            .evidence(format!("the side holds no base at {hex}, a named place of {island_text}"), self.today)
            .binding(format!("{} is released", plan.id))
            .worked(format!(
                "{} controls {island_text}; the side holds no base at {hex}, one of its places, and no take of it is in force: the staff forms a take of the place at {hex}{date} under it, with a plan of its own.",
                end.id
            ))
            .would_change("a base of the side at the place")
            .would_change("a take of the place already in force")
            .done();
        let id_new = self.form_under(end, kind, reason, in_scope);
        if in_scope {
            self.lines.push(format!(
                "{} would form {id_new}, take the place at {hex}{date}, under it.",
                end.id
            ));
        }
        // Its plan is made ready on leave to prepare this morning, as a
        // root's is (item 9).
        self.leave_for(id_new);
    }

    /// Item 6: "hold a place" of the side's base at the place, else the
    /// enemy's lowest-numbered there; with its forces limit (item 8).
    #[allow(clippy::too_many_arguments)]
    fn island_hold(
        &mut self,
        end: &Intent,
        plan: &Plan,
        hex: Hex,
        source: &Source,
        island_text: &str,
        in_scope: bool,
    ) {
        let today = self.today;
        let side = self.view.side();
        // A hold of a base in the place's hex stands, whoever made it.
        let standing = self
            .state
            .intents_in_force()
            .find(|i| match i.kind {
                IntentKind::Hold { base } => self.hex_of(base) == Some(hex),
                _ => false,
            })
            .cloned();
        if let Some(other) = standing {
            let IntentKind::Hold { base } = other.kind else {
                return;
            };
            let words = format!("hold {}", base_words(self.view, base));
            self.island_stands(end, plan, &other, &words, in_scope);
            return;
        }
        let own = self
            .view
            .bases()
            .filter(|b| b.hex == hex)
            .map(|b| b.id)
            .min();
        let enemy = self
            .view
            .enemy_bases()
            .filter(|b| b.hex == hex && b.side != side)
            .map(|b| b.id)
            .min();
        let Some(base) = own.or(enemy) else {
            let reason = serving(end, plan)
                .evidence(format!("no base stands at {hex}, a named place of {island_text}"), today)
                .worked(format!(
                    "{} controls {island_text}; no base stands at {hex}, so the staff forms its hold on the first morning the side has a base there.",
                    end.id
                ))
                .would_change("a base of the side at the place")
                .done();
            self.target(in_scope).no_orders.push(reason);
            return;
        };
        let base_text = base_words(self.view, base);
        let whose = if own.is_some() {
            "the side's base there".to_string()
        } else {
            "the enemy's base there, which it waits to take".to_string()
        };
        let reason = serving(end, plan)
            .evidence(format!("{base_text} stands at {hex}, a named place of {island_text}"), today)
            .binding(format!("{} is released", plan.id))
            .worked(format!(
                "{} controls {island_text}: the staff forms a hold of {base_text}, {whose}, under it.",
                end.id
            ))
            .would_change("a hold of the place already in force")
            .done();
        let id_new = self.form_under(end, IntentKind::Hold { base }, reason, in_scope);
        if in_scope {
            self.lines.push(format!(
                "{} would form {id_new}, hold {base_text}, under it.",
                end.id
            ));
        }
        self.hold_limit(end, plan, id_new, &base_text, source, in_scope);
    }

    /// Item 8: the forces limit of a hold the staff formed: the source and
    /// the transport forces stopped there. With no source, none, and the
    /// reason says so.
    fn hold_limit(
        &mut self,
        end: &Intent,
        plan: &Plan,
        hold: IntentId,
        base_text: &str,
        source: &Source,
        in_scope: bool,
    ) {
        let today = self.today;
        let Some((base, forces)) = &source.chosen else {
            let reason = serving(end, plan)
                .evidence(source.words.clone(), today)
                .binding("no base can be the source of supply")
                .worked(format!(
                    "{}: {}; so the staff sets no forces limit on {hold}, the hold of {base_text}, and it forms no supply.",
                    end.id, source.words
                ))
                .would_change("a base of the side off the island, that the staff may take, with supply above its reserve")
                .done();
            self.target(in_scope).no_orders.push(reason);
            return;
        };
        let limit = ForcesLimit {
            ships: BTreeSet::new(),
            task_forces: forces.clone(),
            air_groups: BTreeSet::new(),
            source: Some(*base),
            land_units: BTreeSet::new(),
            submarines: Default::default(),
            none_of: Default::default(),
        };
        let reason = serving(end, plan)
            .evidence(source.words.clone(), today)
            .worked(format!(
                "{}: {}; the staff sets the forces limit of {hold}, the hold of {base_text}, to {} and {}.",
                end.id,
                source.words,
                base_words(self.view, *base),
                if forces.is_empty() {
                    "no force".to_string()
                } else {
                    forces
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                }
            ))
            .would_change("another source of supply")
            .done();
        if let Some(found) = self.state.intents.get_mut(&hold) {
            found.limits.forces = Some(limit.clone());
        }
        self.target(in_scope).commands.push(StaffCommand {
            kind: CommandKind::SetLimit(Limit::Forces {
                intent: hold,
                limit,
            }),
            reason,
        });
    }

    /// Item 8: the source of supply. The commander's, where his forces limit
    /// names one; else the side's base in no named place's hex that the
    /// staff may take with the most supply above its reserve, then the lower
    /// number, none with nothing above it (decision 0163, 1: no transport
    /// force need lie there). With it, the pool there, which may be empty.
    fn island_source(&self, end: &Intent, places: &[Place]) -> Source {
        let others = self.others_above(end);
        let named = end.limits.forces.as_ref().and_then(|f| f.source);
        // Decision 0152, 10: where the commander names none, the source
        // chosen on an earlier morning is kept: the source of a hold the
        // staff formed under the island, where it is still the side's.
        let kept = if named.is_none() {
            self.state
                .intents_in_force()
                .filter(|i| {
                    i.parent == Some(end.id)
                        && i.maker == Maker::Staff
                        && matches!(i.kind, IntentKind::Hold { .. })
                })
                .find_map(|i| i.limits.forces.as_ref()?.source)
                .filter(|base| self.view.bases().any(|b| b.id == *base))
        } else {
            None
        };
        if let Some(base) = kept {
            let forces = self.pool_at(base, &others);
            return Source {
                words: format!(
                    "the source of supply is {}, chosen on an earlier morning and kept, with {} stopped there with nothing aboard",
                    base_words(self.view, base),
                    if forces.is_empty() {
                        "no transport force".to_string()
                    } else {
                        forces
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(", ")
                    }
                ),
                chosen: Some((base, forces)),
            };
        }
        let place_hexes: BTreeSet<Hex> = places.iter().map(|p| p.hex).collect();
        let mut best: Option<(BaseId, sim_core::fixed::Fixed, BTreeSet<TaskForceId>)> = None;
        let mut lines: Vec<String> = Vec::new();
        let mut bases: Vec<_> = self.view.bases().cloned().collect();
        bases.sort_by_key(|b| b.id);
        for base in bases {
            if named.is_some_and(|n| n != base.id) {
                continue;
            }
            if named.is_none() && place_hexes.contains(&base.hex) {
                continue;
            }
            let text = base_words(self.view, base.id);
            if let Err(why) = self.may_take_base(base.id, &others) {
                lines.push(format!("{text}: {why}"));
                continue;
            }
            // Decision 0163, 1: no transport force need lie there; the pool
            // may be empty (item 2).
            let forces = self.pool_at(base.id, &others);
            let pool_words = if forces.is_empty() {
                "no transport force stopped there with nothing aboard".to_string()
            } else {
                format!(
                    "{} stopped there with nothing aboard",
                    forces
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            };
            let above = base.supply_t - base.supply_reserve_t;
            // The commander's source is his, whatever it holds; one the staff
            // chooses must have supply above its reserve.
            if named.is_none() && above <= sim_core::fixed::Fixed::ZERO {
                lines.push(format!(
                    "{text}: nothing above its reserve, so it cannot be the source"
                ));
                continue;
            }
            lines.push(format!(
                "{text}: {} t of supply above its reserve, with {pool_words}",
                above.max(sim_core::fixed::Fixed::ZERO)
            ));
            let better = best.as_ref().is_none_or(|(_, most, _)| above > *most);
            if better {
                best = Some((base.id, above, forces));
            }
        }
        let named_words = if named.is_some() {
            "the commander's forces limit names the source"
        } else {
            "the bases of the side in no named place's hex"
        };
        match best {
            Some((base, _, forces)) => Source {
                words: format!(
                    "the source of supply, of {named_words}: {}; the source is {}",
                    lines.join("; "),
                    base_words(self.view, base)
                ),
                chosen: Some((base, forces)),
            },
            None => Source {
                words: if lines.is_empty() {
                    format!("no base can be the source of supply ({named_words}: none)")
                } else {
                    format!(
                        "no base can be the source of supply ({named_words}: {})",
                        lines.join("; ")
                    )
                },
                chosen: None,
            },
        }
    }

    /// Decision 0152, 10 and 11: the transport forces the staff may take
    /// that lie stopped at `base` with nothing aboard, a ship afloat each.
    fn pool_at(&self, base: BaseId, others: &crate::assemble::Others) -> BTreeSet<TaskForceId> {
        let Some(hex) = self.view.bases().find(|b| b.id == base).map(|b| b.hex) else {
            return BTreeSet::new();
        };
        let mut forces = BTreeSet::new();
        for force in self.area.task_forces() {
            if force.mission != Mission::Transport
                || !crate::fleet::force_stopped_at(force, hex)
                || afloat(self.area, force.id).is_empty()
            {
                continue;
            }
            let empty = self
                .area
                .ships()
                .filter(|s| s.location == sim_core::state::ShipLocation::TaskForce(force.id))
                .all(|s| s.supply_t.is_zero() && s.cargo_fuel_t.is_zero() && s.troops.is_none());
            if empty && self.may_take_force(force, others).is_ok() {
                forces.insert(force.id);
            }
        }
        forces
    }

    /// Decision 0152, 11: each morning, the forces limit of each hold the
    /// staff formed under the island is set to the source and the transport
    /// forces stopped there with nothing aboard, where that differs from the
    /// limit it has. The pool only grows: a force once named stays while it
    /// has a ship afloat (the architect's ruling of Nov 18, 2026), so that a
    /// ship serving a run stays in the limit its orders are checked against.
    fn island_pools(&mut self, end: &Intent, plan: &Plan, source: &Source, in_scope: bool) {
        let Some((base, pool)) = &source.chosen else {
            return;
        };
        let holds: Vec<Intent> = self
            .state
            .intents_in_force()
            .filter(|i| {
                i.parent == Some(end.id)
                    && i.maker == Maker::Staff
                    && matches!(i.kind, IntentKind::Hold { .. })
            })
            .cloned()
            .collect();
        for hold in holds {
            let mut forces: BTreeSet<TaskForceId> = hold
                .limits
                .forces
                .as_ref()
                .filter(|f| f.source == Some(*base))
                .map(|f| f.task_forces.clone())
                .unwrap_or_default();
            forces.retain(|f| !afloat(self.area, *f).is_empty());
            forces.extend(pool.iter().copied());
            let limit = ForcesLimit {
                ships: BTreeSet::new(),
                task_forces: forces.clone(),
                air_groups: BTreeSet::new(),
                source: Some(*base),
                land_units: BTreeSet::new(),
                submarines: Default::default(),
                none_of: Default::default(),
            };
            if hold.limits.forces.as_ref() == Some(&limit) {
                continue;
            }
            let IntentKind::Hold { base: held } = hold.kind else {
                continue;
            };
            let base_text = base_words(self.view, held);
            let reason = serving(end, plan)
                .evidence(source.words.clone(), self.today)
                .worked(format!(
                    "{}: {}; the staff sets the forces limit of {}, the hold of {base_text}, to {} and {}: the source and the transport forces stopped there with nothing aboard, with those it named before.",
                    end.id,
                    source.words,
                    hold.id,
                    base_words(self.view, *base),
                    if forces.is_empty() {
                        "no force".to_string()
                    } else {
                        forces
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(", ")
                    }
                ))
                .would_change("another transport force stopped empty at the source")
                .done();
            if let Some(found) = self.state.intents.get_mut(&hold.id) {
                found.limits.forces = Some(limit.clone());
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

    /// Item 6: the denial of the waters within 3 hexes of the first place,
    /// once. Returns the denial of the island's tree, formed or standing.
    fn island_deny(
        &mut self,
        end: &Intent,
        plan: &Plan,
        first: Hex,
        island_text: &str,
        in_scope: bool,
    ) -> Option<Intent> {
        let standing = self
            .state
            .intents_in_force()
            .find(|i| matches!(i.kind, IntentKind::Deny { hex, .. } if hex == first))
            .cloned();
        if let Some(other) = standing {
            let words =
                crate::reasons::waters_words(first, other.kind.waters().map_or(0, |(_, d)| d));
            self.island_stands(end, plan, &other, &words, in_scope);
            return (self.state.root_of(other.id) == end.id).then_some(other);
        }
        let kind = IntentKind::Deny {
            hex: first,
            distance: DENY_HEXES,
        };
        let reason = serving(end, plan)
            .evidence(format!("{first} is the first place of {island_text}"), self.today)
            .binding(format!("{} is released", plan.id))
            .worked(format!(
                "{} controls {island_text}: the staff forms a denial of the waters within {DENY_HEXES} hexes of its first place, {first}, under it.",
                end.id
            ))
            .would_change("a denial of those waters already in force")
            .done();
        let id_new = self.form_under(end, kind, reason, in_scope);
        if in_scope {
            self.lines.push(format!(
                "{} would form {id_new}, {}, under it.",
                end.id,
                crate::reasons::waters_words(first, DENY_HEXES)
            ));
        }
        self.state.intents.get(&id_new).cloned()
    }

    /// Item 6: "operate a carrier force" under the denial, for each task
    /// force on the carrier mission the staff may take, with a carrier afloat
    /// that has an air group aboard, in task force order, where no carrier
    /// task for that force is in force anywhere on the side, at the covering
    /// hex of decision 0148, item 2, reckoned to the island's first place.
    #[allow(clippy::too_many_lines)]
    fn island_carriers(
        &mut self,
        end: &Intent,
        deny: &Intent,
        first: Hex,
        island_text: &str,
        in_scope: bool,
    ) {
        let Some(plan) = self.plan_of(deny) else {
            return;
        };
        let today = self.today;
        let others = self.others_above(end);
        let mut forces: Vec<_> = self
            .area
            .task_forces()
            .filter(|f| f.mission == Mission::Carrier)
            .cloned()
            .collect();
        forces.sort_by_key(|f| f.id);
        for force in forces {
            let id = force.id;
            if let Err(why) = self.may_take_force(&force, &others) {
                let reason = serving(deny, &plan)
                    .evidence(why.clone(), today)
                    .worked(format!(
                        "{}: {why}, so the staff forms no carrier task for it under {}.",
                        deny.id, deny.id
                    ))
                    .would_change("the force free to take")
                    .done();
                self.target(in_scope).no_orders.push(reason);
                continue;
            }
            let ships = afloat(self.area, id);
            let carriers: Vec<_> = ships
                .iter()
                .filter(|(_, _, _, kind)| *kind == sim_core::refdata::ShipKind::Carrier)
                .map(|(ship, _, _, _)| *ship)
                .collect();
            let aboard: Vec<_> = self
                .area
                .air_groups()
                .filter(
                    |g| matches!(g.location, AirGroupLocation::Ship(s) if carriers.contains(&s)),
                )
                .collect();
            if aboard.is_empty() {
                continue;
            }
            let standing = self
                .state
                .intents_in_force()
                .find(|other| {
                    other.kind.force() == Some(id)
                        && matches!(other.kind, IntentKind::OperateCarriers { .. })
                })
                .cloned();
            if let Some(other) = standing {
                let words = format!("operate the carrier force {id}");
                self.island_stands(deny, &plan, &other, &words, in_scope);
                continue;
            }
            let held = crate::reasons::force_held_words(self.area, id);
            let here = force.position();
            let why = format!(
                "{held} is on the carrier mission with {} carrier(s) afloat and {} air group(s) aboard, and lies at {here}",
                carriers.len(),
                aboard.len()
            );
            let (hex, hex_words) = match self.covering_hex(id, here, &aboard, first) {
                Ok(found) => found,
                Err(NoCoveringHex { reach, sets }) => {
                    let reason = serving(deny, &plan)
                        .evidence(why.clone(), today)
                        .evidence(
                            format!("no sea route from {here} reaches a hex within {reach} nm of {first}"),
                            today,
                        )
                        .binding("the covering hex is on the shortest sea route to the place")
                        .worked(format!(
                            "{} denies the waters of {island_text}, but no sea route from {here} reaches a hex within the {reach} nm normal strike radius of {first}, set by {sets}, so the staff forms no carrier task for {id}. {why}.",
                            deny.id
                        ))
                        .would_change("a sea route to the place")
                        .done();
                    self.target(in_scope).no_orders.push(reason);
                    continue;
                }
            };
            let kind = IntentKind::OperateCarriers { force: id, hex };
            let id_new = self.state.next_intent_number();
            let reason = serving(deny, &plan)
                .evidence(why.clone(), today)
                .evidence(hex_words.clone(), today)
                .binding(format!("{} denies the waters of {island_text}", deny.id))
                .worked(format!(
                    "{} denies the waters of {island_text}, its plan is released and no carrier task of {id} stands: {why}. For the island's first place at {first}: {hex_words}. The staff forms {id_new}, to operate the carrier force {id} from {hex}, under {}, for {id} to carry out.",
                    deny.id, deny.id
                ))
                .would_change(format!("{id} taken by an intent higher in priority"))
                .would_change("such a task already standing")
                .done();
            let formed = self.form_under(deny, kind, reason, in_scope);
            debug_assert_eq!(formed, id_new);
            if in_scope {
                self.lines.push(format!(
                    "{} would form {id_new}, to operate the carrier force {id} from {hex}, under it ({hex_words}).",
                    deny.id
                ));
            }
        }
    }

    /// Item 6: "neutralise a base" for each enemy base on the side's map
    /// within 15 hexes of a named place, in base order, the base at a place
    /// still to be taken among them.
    fn island_neutralise(
        &mut self,
        end: &Intent,
        plan: &Plan,
        places: &[Place],
        island_text: &str,
        in_scope: bool,
    ) {
        let side = self.view.side();
        let mut targets: Vec<(BaseId, Hex)> = self
            .view
            .enemy_bases()
            .filter(|b| b.side != side)
            .filter(|b| {
                places
                    .iter()
                    .any(|p| p.hex.distance(b.hex) <= NEUTRALISE_HEXES)
            })
            .map(|b| (b.id, b.hex))
            .collect();
        targets.sort_by_key(|(id, _)| *id);
        for (target, target_hex) in targets {
            let target_text = base_words(self.view, target);
            let standing = self
                .state
                .intents_in_force()
                .find(|i| matches!(i.kind, IntentKind::Neutralise { target: t } if t == target))
                .cloned();
            if let Some(other) = standing {
                self.island_stands(
                    end,
                    plan,
                    &other,
                    &format!("neutralise {target_text}"),
                    in_scope,
                );
                continue;
            }
            let near = places
                .iter()
                .map(|p| (p.hex.distance(target_hex), p.hex))
                .min()
                .map_or_else(String::new, |(d, h)| format!("{d} hex(es) from {h}"));
            let at_place = places.iter().any(|p| p.hex == target_hex);
            let why = if at_place {
                format!("{target_text} is the enemy's base at a named place of {island_text}, which is neutralised until it is taken")
            } else {
                format!("{target_text} is an enemy base on the side's map {near}, within {NEUTRALISE_HEXES} hexes of a named place of {island_text}")
            };
            let reason = serving(end, plan)
                .evidence(why.clone(), self.today)
                .binding(format!("{} is released", plan.id))
                .worked(format!(
                    "{} controls {island_text}: {why}, so the staff forms its neutralising under it.",
                    end.id
                ))
                .would_change("a neutralising of it already in force")
                .done();
            let id_new = self.form_under(end, IntentKind::Neutralise { target }, reason, in_scope);
            if in_scope {
                self.lines.push(format!(
                    "{} would form {id_new}, neutralise {target_text}, under it.",
                    end.id
                ));
            }
        }
    }
}

/// The source of supply of an island's holds (item 8): the base chosen and
/// the transport forces stopped there, with the words of the choice.
pub(crate) struct Source {
    chosen: Option<(BaseId, BTreeSet<TaskForceId>)>,
    words: String,
}
