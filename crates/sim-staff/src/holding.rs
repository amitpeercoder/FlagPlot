//! Holding a place with men, and the supply that follows (decision 0144,
//! step 4 of stage 3, second part, items 4 to 9): what the hold doctrine
//! does on each morning a hold's plan is released and its base is the
//! side's, before the cover.
//!
//! - **The men** (items 4 and 5): each fighting unit with men ashore in the
//!   base's hex that the hold may use, that the side may use and the
//!   commander does not hold, and that serves no "take a place" not yet met,
//!   is set to defend where it is on attack. Nothing else is ordered of it.
//! - **The supply** (items 7 to 9): on a morning the hold has men at the
//!   base, where the hold's forces limit names a
//!   source and no "keep supplied" for the base's hex is in force, the staff
//!   forms one under the hold for the base, with the margin of its risk as
//!   its floor and no interval. With no source named it forms none and says
//!   so.
//!
//! A force on the transport mission named in the hold's forces limit is
//! given no task of its own (item 6): the tasks of decisions 0124 and 0125
//! are formed for forces on the surface combat and the carrier missions
//! only (`crate::stand`), so nothing here forms one.
//!
//! The doctrine reads the side's own view and its planned command state and
//! nothing else.

use std::collections::BTreeSet;

use sim_core::command::{
    CommandKind, HoldTarget, Intent, IntentKind, IntentLimits, Maker, Plan, Risk, StaffCommand,
    StaffOrder,
};
use sim_core::ids::{BaseId, EntityRef, LandUnitId};
use sim_core::orders::{LandUnitOrder, OrderKind, Posture};

use crate::air::Day;
use crate::reasons::{base_words, hold_words, serving};

/// The margin of the check that a place can be held, in days, by the risk
/// of the hold (decision 0144, 10): the staff's figures, placeholders. It
/// is also the floor of the "keep supplied" formed under the hold (item 7).
pub(crate) fn margin_days(risk: Risk) -> u32 {
    match risk {
        Risk::Preserve => 10,
        Risk::Balanced => 5,
        Risk::Press => 0,
    }
}

impl Day<'_, '_> {
    /// The land units that serve a "take a place" not yet met (decision
    /// 0144, 4): those named in its forces limit or in the forces limit of
    /// a task under it, for each such end in force whose plan has not ended
    /// and whose place is not the side's.
    pub(crate) fn serving_a_take(&self) -> BTreeSet<LandUnitId> {
        let mut found = BTreeSet::new();
        for end in self.state.intents_in_force() {
            let IntentKind::TakePlace { hex, .. } = end.kind else {
                continue;
            };
            if self
                .state
                .plans
                .get(&end.plan)
                .is_none_or(|plan| plan.state.ended())
                || self.place_is_ours(hex)
            {
                continue;
            }
            for intent in self.state.intents_in_force() {
                if intent.id == end.id || intent.parent == Some(end.id) {
                    if let Some(forces) = &intent.limits.forces {
                        found.extend(forces.land_units.iter().copied());
                    }
                }
            }
        }
        found
    }

    /// Items 4 and 5: the men of the hold on attack are set to defend.
    /// Whether the hold has men at the base: a fighting unit of item 4,
    /// on attack or not, the side may use and the commander does not hold.
    pub(crate) fn hold_men(
        &mut self,
        hold: &Intent,
        plan: &Plan,
        base: BaseId,
        in_scope: bool,
    ) -> bool {
        let Some(hex) = self.view.bases().find(|b| b.id == base).map(|b| b.hex) else {
            return false;
        };
        let today = self.today;
        let base_text = base_words(self.view, base);
        // Those its forces limit names, read up the tree; with no land unit
        // named anywhere on the chain, all of them.
        let named = self
            .state
            .forces_on_chain(hold.id)
            .iter()
            .any(|forces| !forces.land_units.is_empty());
        let allowed = if named {
            self.state.land_units_on_chain(hold.id)
        } else {
            None
        };
        let taken = self.serving_a_take();
        let mut units: Vec<sim_core::state::LandUnit> = self
            .view
            .land_units()
            .filter(|u| u.men_ashore > 0 && u.hex == Some(hex) && !u.working)
            .filter(|u| allowed.as_ref().is_none_or(|a| a.contains(&u.id)))
            .cloned()
            .collect();
        units.sort_by_key(|u| u.id);
        let mut orders = Vec::new();
        let mut men = false;
        for unit in units {
            if crate::available::not_before(self.view, EntityRef::LandUnit(unit.id)).is_some()
                || taken.contains(&unit.id)
            {
                continue;
            }
            let target = HoldTarget::LandUnit(unit.id);
            let held = self.state.hold_of(target).copied();
            if held.is_none() {
                men = true;
            }
            if unit.posture != Posture::Attack {
                continue;
            }
            if let Some(found) = held {
                let words = hold_words(target, found);
                let reason = serving(hold, plan)
                    .evidence(words.clone(), today)
                    .binding(words.clone())
                    .worked(format!(
                        "{} ({}) is on attack at {base_text}, but {words}: the staff gives it no order.",
                        unit.id, unit.name
                    ))
                    .would_change("the hold released")
                    .done();
                self.target(in_scope).no_orders.push(reason);
                continue;
            }
            let reason = serving(hold, plan)
                .evidence(
                    format!(
                        "{} ({}) has {} men ashore at {base_text} and is on attack",
                        unit.id, unit.name, unit.men_ashore
                    ),
                    today,
                )
                .worked(format!(
                    "{} holds {base_text}, which is the side's: {} ({}) is one of its men there and is on attack, so the staff sets it to defend, where it rests and digs in.",
                    hold.id, unit.id, unit.name
                ))
                .would_change("an operation that is not yet over at the base")
                .done();
            if in_scope {
                self.lines
                    .push(format!("{} is set to defend at {base_text}.", unit.id));
            }
            orders.push(StaffOrder {
                kind: OrderKind::LandUnit {
                    target: unit.id,
                    order: LandUnitOrder::SetPosture {
                        posture: Posture::Defend,
                        when_spent: None,
                    },
                },
                intent: hold.id,
                reason,
            });
        }
        self.target(in_scope).orders.extend(orders);
        men
    }

    /// Items 7 and 9: "keep supplied" formed under the hold, on a morning it
    /// has men at the base (the supply follows the men: a hold with no man
    /// there plays as it did).
    pub(crate) fn hold_supply(&mut self, hold: &Intent, plan: &Plan, base: BaseId, in_scope: bool) {
        let Some(hex) = self.view.bases().find(|b| b.id == base).map(|b| b.hex) else {
            return;
        };
        let today = self.today;
        let base_text = base_words(self.view, base);
        let standing = self.state.intents_in_force().any(|i| match i.kind {
            IntentKind::KeepSupplied { base: b, .. } => {
                self.view
                    .bases()
                    .find(|found| found.id == b)
                    .map(|found| found.hex)
                    == Some(hex)
            }
            _ => false,
        });
        if standing {
            return;
        }
        let source = self
            .state
            .nearest_forces(hold.id)
            .and_then(|forces| forces.source);
        let Some(source) = source else {
            let reason = serving(hold, plan)
                .evidence(format!("{} names no source", hold.id), today)
                .binding("no source in the forces limit")
                .worked(format!(
                    "{} holds {base_text} and its forces limit names no source, so the staff forms no supply for it: the commander may name a source in the forces limit, or state \"keep supplied\" himself.",
                    hold.id
                ))
                .would_change("a source in the forces limit")
                .done();
            self.target(in_scope).no_orders.push(reason);
            return;
        };
        let risk = self.state.risk_read_up(hold.id);
        let floor_days = margin_days(risk);
        let kind = IntentKind::KeepSupplied {
            base,
            floor_days,
            interval_days: None,
        };
        let id_new = self.state.next_intent_number();
        self.state.intents.insert(
            id_new,
            Intent {
                id: id_new,
                kind,
                parent: Some(hold.id),
                maker: Maker::Staff,
                addressee: kind.addressee(),
                limits: IntentLimits::default(),
                set_day: today,
                plan: hold.plan,
                withdrawn_day: None,
            },
        );
        let reason = serving(hold, plan)
            .evidence(
                format!(
                    "{base_text} is the side's and no supply intent for its hex is in force; the forces limit names {source} as the source"
                ),
                today,
            )
            .binding(format!("{} is held and its plan is released", hold.id))
            .worked(format!(
                "{} holds {base_text}, which is the side's, and no supply for it is in force: the staff forms {id_new}, keep {base_text} supplied from {source}, under it, with a floor of {floor_days} {} (the margin at {}) and no interval, so that the staff uses 15 days. Its forces limit is {}'s.",
                hold.id,
                crate::units::days_word(floor_days),
                risk.name(),
                hold.id
            ))
            .would_change("a supply intent for the base already in force")
            .would_change("no source in the forces limit")
            .done();
        if in_scope {
            self.lines.push(format!(
                "{} holds {base_text}: it would form {id_new}, keep {base_text} supplied from {source}, under it.",
                hold.id
            ));
        }
        self.target(in_scope).commands.push(StaffCommand {
            kind: CommandKind::FormIntent {
                parent: hold.id,
                kind,
            },
            reason,
        });
    }
}
