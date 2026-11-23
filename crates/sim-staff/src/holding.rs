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

use sim_core::command::LimitKind;
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

    /// Decision 0159, A, 1: the side's own base through which its men are
    /// fed in the hex of `held`, a base that is not the side's: the
    /// lowest-numbered base of the side in that hex, where the side has
    /// fighting or working men ashore there. The hex is read from the side's
    /// own map of the bases it does not hold; a base not on it has none.
    pub(crate) fn fed_through(&self, held: BaseId) -> Option<BaseId> {
        let hex = self.view.enemy_bases().find(|b| b.id == held)?.hex;
        let men = self
            .view
            .land_units()
            .any(|u| u.men_ashore > 0 && u.hex == Some(hex));
        if !men {
            return None;
        }
        self.view
            .bases()
            .filter(|b| b.hex == hex)
            .map(|b| b.id)
            .min()
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
        // Outside an island a limit that names nothing binds them too
        // (decision 0222, rule 3), as intake reads it.
        let by_kind = !self.state.reads_as_bound(hold.id);
        let named =
            self.state.forces_on_chain(hold.id).iter().any(|forces| {
                !forces.land_units.is_empty() || (by_kind && forces.binds_land_units())
            });
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

    /// Decision 0222, rule 1: the reason of a hold with no source on it or
    /// above it. Where a base of the side could be one by the island's test
    /// (a base of the side not in the place's hex, or under "control an
    /// island" in none of its places' hexes, that the staff may take, with
    /// supply above its reserve), it names those bases; where none could, it
    /// says the place lives on what it has, with its days of supply today,
    /// and offers nothing.
    fn no_source_reason(
        &self,
        hold: &Intent,
        plan: &Plan,
        base_text: &str,
        fed_text: &str,
        hex: sim_core::hex::Hex,
    ) -> sim_core::command::Reason {
        let today = self.today;
        let (end, excluded) = self
            .island_above(hold.id)
            .unwrap_or_else(|| (hold.clone(), std::iter::once(hex).collect()));
        // Decision 0228, rule 6: for a side whose staff chooses its sources,
        // the bases are weighed by rules 1 and 2 and the words say whether
        // any can spare supply.
        if self.view.staff_chooses_sources() {
            let chain = self.state.island_on_chain(hold.id).is_none().then_some(hold.id);
            let weighing = self.weigh_sources(&end, &excluded, hex, chain);
            if weighing.chosen.is_empty() {
                return self.nothing_to_spare_reason(hold, plan, base_text, fed_text, hex, &weighing);
            }
        }
        let weighed = self.source_candidates(&end, &excluded);
        let candidates: Vec<(BaseId, sim_core::fixed::Fixed)> = weighed
            .iter()
            .filter_map(|(base, test)| test.as_ref().ok().map(|above| (*base, *above)))
            .collect();
        // The bases that hold supply above their reserve and that the staff
        // may not take, with the words why (an empty word: nothing above).
        let taken: Vec<String> = weighed
            .iter()
            .filter_map(|(_, test)| test.as_ref().err())
            .filter(|why| !why.is_empty())
            .cloned()
            .collect();
        let opening = format!(
            "{} holds {base_text} and no source is named on it or above it, so the staff forms no supply for it",
            hold.id
        );
        if candidates.is_empty() {
            let figure = self.view.days_of_supply(hex);
            let days = match figure.and_then(|f| f.days) {
                Some(days) => format!(
                    "{} {} of supply today",
                    crate::units::floor_whole(days),
                    crate::units::days_word(crate::units::floor_whole(days))
                ),
                None => format!(
                    "{} t of supply today and no planned use",
                    figure.map_or(sim_core::fixed::Fixed::ZERO, |f| f.stock_t)
                ),
            };
            let (none, said) = if taken.is_empty() {
                let words = "no other base of the side holds supply above its reserve";
                (words.to_string(), words.to_string())
            } else {
                let why = taken.join("; ");
                (
                    format!("no other base of the side can be the source: {why}"),
                    format!("no other base of the side can be the source ({why})"),
                )
            };
            return serving(hold, plan)
                .evidence(format!("{} names no source", hold.id), today)
                .evidence(none, today)
                .evidence(format!("{fed_text}: {days}"), today)
                .binding("no base of the side can be the source")
                .worked(format!(
                    "{opening}. {}, so {fed_text} lives on what it has: {days}.",
                    capitalised(&said)
                ))
                .would_change("a base of the side with supply above its reserve")
                .done();
        }
        let list = candidates
            .iter()
            .map(|(base, above)| {
                format!(
                    "{}, {} t of supply above its reserve",
                    base_words(self.view, *base),
                    above
                )
            })
            .collect::<Vec<_>>()
            .join("; ");
        let names = candidates
            .iter()
            .map(|(base, _)| base_words(self.view, *base))
            .collect::<Vec<_>>()
            .join(", ");
        let mut reason =
            serving(hold, plan).evidence(format!("{} names no source", hold.id), today);
        for (base, above) in &candidates {
            reason = reason.evidence(
                format!(
                    "{} could be the source: {} t of supply above its reserve",
                    base_words(self.view, *base),
                    above
                ),
                today,
            );
        }
        reason
            .binding("no source in the forces limit")
            .worked(format!(
                "{opening}. A source may be named in the forces limit among the bases of the side that could be one: {list}."
            ))
            .would_change(format!("a source named among {names}"))
            .speaks_of(&self.state, LimitKind::Source)
            .done()
    }

    /// Decision 0228, rule 6: the reason of a hold whose side's staff chooses
    /// its sources and no base of the side can spare supply: the place lives
    /// on what it has, with its days of supply, and each base passed over is
    /// named with its figures.
    fn nothing_to_spare_reason(
        &self,
        hold: &Intent,
        plan: &Plan,
        base_text: &str,
        fed_text: &str,
        hex: sim_core::hex::Hex,
        weighing: &crate::source::Weighing,
    ) -> sim_core::command::Reason {
        let today = self.today;
        let figure = self.view.days_of_supply(hex);
        let days = match figure.and_then(|f| f.days) {
            Some(days) => format!(
                "{} {} of supply today",
                crate::units::floor_whole(days),
                crate::units::days_word(crate::units::floor_whole(days))
            ),
            None => format!(
                "{} t of supply today and no planned use",
                figure.map_or(sim_core::fixed::Fixed::ZERO, |f| f.stock_t)
            ),
        };
        let said = if weighing.passed.is_empty() {
            "no other base of the side can spare supply".to_string()
        } else {
            format!(
                "no other base of the side can spare supply ({})",
                weighing.passed.join("; ")
            )
        };
        serving(hold, plan)
            .evidence(format!("{} names no source", hold.id), today)
            .evidence(said.clone(), today)
            .evidence(format!("{fed_text}: {days}"), today)
            .binding("no base of the side can spare supply")
            .worked(format!(
                "{} holds {base_text} and no source is named on it or above it, so the staff chooses one: {}, so {fed_text} lives on what it has: {days}.",
                hold.id,
                capitalised(&said)
            ))
            .would_change("a base of the side with supply to spare")
            .done()
    }

    /// Decision 0228, rules 9 to 12 as amended: where the side's staff
    /// chooses its sources and the hold has men at its base, the staff forms
    /// "keep supplied" under the hold with the forces limit it sets each
    /// morning: the source (the commander's where he names one, else the
    /// staff's, chosen and kept) and the ships (his where his limits bind
    /// ships, else the transport forces stopped at the source with nothing
    /// aboard, a pool that only grows while the source is the same).
    #[allow(clippy::too_many_lines)]
    fn keyed_hold_supply(
        &mut self,
        hold: &Intent,
        plan: &Plan,
        held: BaseId,
        fed: BaseId,
        hex: sim_core::hex::Hex,
        in_scope: bool,
    ) {
        use sim_core::command::{ForcesLimit, Limit};
        let today = self.today;
        let base_text = base_words(self.view, held);
        let fed_text = base_words(self.view, fed);
        let hex_of = |b: BaseId| self.view.bases().find(|found| found.id == b).map(|f| f.hex);
        let mine: Option<Intent> = self
            .state
            .intents_in_force()
            .find(|i| {
                i.parent == Some(hold.id)
                    && i.maker == Maker::Staff
                    && matches!(i.kind, IntentKind::KeepSupplied { base: b, .. } if hex_of(b) == Some(hex))
            })
            .cloned();
        if mine.is_none()
            && self.state.intents_in_force().any(|i| match i.kind {
                IntentKind::KeepSupplied { base: b, .. } => hex_of(b) == Some(hex),
                _ => false,
            })
        {
            return;
        }
        let owned = |b: BaseId| self.view.bases().any(|found| found.id == b);
        let others = self.others_above(hold);
        let commander = self
            .state
            .commander_source_on_chain(hold.id)
            .filter(|b| owned(*b));
        let kept = mine
            .as_ref()
            .and_then(|m| m.limits.forces.as_ref()?.source)
            .filter(|b| owned(*b));
        let (source, mut pool, words, lines) = if let Some(source) = commander {
            let pool = self.pool_at(source, &others);
            (
                source,
                pool,
                format!("the commander's forces limit names {source} as the source"),
                Vec::new(),
            )
        } else if let Some(source) = kept {
            let pool = self.pool_at(source, &others);
            (
                source,
                pool,
                format!(
                    "the source of supply is {}, chosen on an earlier morning and kept",
                    base_words(self.view, source)
                ),
                Vec::new(),
            )
        } else {
            let excluded: BTreeSet<sim_core::hex::Hex> = std::iter::once(hex).collect();
            let weighing = self.weigh_sources(hold, &excluded, hex, Some(hold.id));
            let Some(pick) = weighing.chosen.first() else {
                let reason =
                    self.nothing_to_spare_reason(hold, plan, &base_text, &fed_text, hex, &weighing);
                self.target(in_scope).no_orders.push(reason);
                return;
            };
            let lines = weighing.lines();
            (
                pick.base,
                pick.forces.clone(),
                format!(
                    "the staff chose the source of supply from the bases of the side not in the place's hex: {}; the source is {}",
                    lines.join("; "),
                    base_words(self.view, pick.base)
                ),
                lines,
            )
        };
        pool.retain(|force| self.chain_allows_force(hold.id, *force));
        // The ships: his where his limits bind ships, else the pool, which
        // only grows while the source is the same.
        let bound = self
            .state
            .nearest_ship_limit(hold.id)
            .map(std::borrow::Cow::into_owned);
        let before: Option<ForcesLimit> = mine.as_ref().and_then(|m| m.limits.forces.clone());
        let (ships, task_forces, in_place) = match &bound {
            Some(limit) => (limit.ships.clone(), limit.task_forces.clone(), limit.in_place),
            None => {
                let mut forces: BTreeSet<sim_core::ids::TaskForceId> = before
                    .as_ref()
                    .filter(|f| f.source == Some(source))
                    .map(|f| f.task_forces.clone())
                    .unwrap_or_default();
                forces.retain(|f| !crate::stand::afloat(self.area, *f).is_empty());
                forces.extend(pool.iter().copied());
                (BTreeSet::new(), forces, false)
            }
        };
        let limit = ForcesLimit {
            ships,
            task_forces: task_forces.clone(),
            air_groups: BTreeSet::new(),
            source: Some(source),
            land_units: BTreeSet::new(),
            submarines: Default::default(),
            none_of: Default::default(),
            in_place,
        };
        let forces_text = if task_forces.is_empty() {
            "no force".to_string()
        } else {
            task_forces
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        };
        let (id, formed) = match &mine {
            Some(found) => (found.id, false),
            None => {
                let risk = self.state.risk_read_up(hold.id);
                let floor_days = margin_days(risk);
                let kind = IntentKind::KeepSupplied {
                    base: fed,
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
                let opening = if fed == held {
                    format!(
                        "{} holds {base_text}, which is the side's, and no supply for it is in force",
                        hold.id
                    )
                } else {
                    format!(
                        "{} holds {base_text}, which is not the side's, and the side's men in its hex are fed through {fed_text}, for which no supply is in force",
                        hold.id
                    )
                };
                let mut builder = serving(hold, plan)
                    .evidence(format!("{base_text} has no supply intent for its hex in force"), today);
                for line in &lines {
                    builder = builder.evidence(line.clone(), today);
                }
                let reason = builder
                    .evidence(words.clone(), today)
                    .binding(format!("{} is held and its plan is released", hold.id))
                    .worked(format!(
                        "{opening}: the staff forms {id_new}, keep {fed_text} supplied, under it, with a floor of {floor_days} {} (the margin at {}) and no interval, so that the staff uses 15 days. {}.",
                        crate::units::days_word(floor_days),
                        risk.name(),
                        capitalised(&words)
                    ))
                    .would_change("a supply intent for the base already in force")
                    .would_change("another source of supply")
                    .speaks_of(&self.state, LimitKind::Source)
                    .done();
                if in_scope {
                    self.lines.push(format!(
                        "{} holds {base_text}: it would form {id_new}, keep {fed_text} supplied from {source}, under it.",
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
                (id_new, true)
            }
        };
        if !formed && before.as_ref() == Some(&limit) {
            return;
        }
        let reason = serving(hold, plan)
            .evidence(words.clone(), today)
            .worked(format!(
                "{}: {words}; the staff sets the forces limit of {id}, the supply of {fed_text}, to {} and {forces_text}.",
                hold.id,
                base_words(self.view, source)
            ))
            .would_change("another source of supply")
            .would_change("another transport force stopped empty at the source")
            .speaks_of(&self.state, LimitKind::Source)
            .done();
        if let Some(found) = self.state.intents.get_mut(&id) {
            found.limits.forces = Some(limit.clone());
        }
        self.target(in_scope).commands.push(StaffCommand {
            kind: CommandKind::SetLimit(Limit::Forces { intent: id, limit }),
            reason,
        });
    }

    /// Items 7 and 9: "keep supplied" formed under the hold, on a morning it
    /// has men at the base (the supply follows the men: a hold with no man
    /// there plays as it did).
    ///
    /// `held` is the base the hold names and `fed` the side's base the supply
    /// goes to: the same base where the hold's base is the side's, else the
    /// side's own base in its hex through which its men there are fed
    /// (decision 0159, A).
    pub(crate) fn hold_supply(
        &mut self,
        hold: &Intent,
        plan: &Plan,
        held: BaseId,
        fed: BaseId,
        in_scope: bool,
    ) {
        let Some(hex) = self.view.bases().find(|b| b.id == fed).map(|b| b.hex) else {
            return;
        };
        let today = self.today;
        let base_text = base_words(self.view, held);
        let fed_text = base_words(self.view, fed);
        // Decision 0228: for a side whose staff chooses its sources, a hold
        // that is not under an island has the supply formed and its forces
        // limit set by the staff's own rule.
        if self.view.staff_chooses_sources() && self.state.island_on_chain(hold.id).is_none() {
            self.keyed_hold_supply(hold, plan, held, fed, hex, in_scope);
            return;
        }
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
            let reason = self.no_source_reason(hold, plan, &base_text, &fed_text, hex);
            self.target(in_scope).no_orders.push(reason);
            return;
        };
        let risk = self.state.risk_read_up(hold.id);
        let floor_days = margin_days(risk);
        let kind = IntentKind::KeepSupplied {
            base: fed,
            floor_days,
            interval_days: None,
        };
        let id_new = self.state.next_intent_number();
        // Under an island's hold it has a plan of its own (decision 0151, 9),
        // drafted as intake drafts it; else the hold's.
        let plan_id = if self.state.forms_own_plan(kind, hold.id) {
            let plan_id = sim_core::command::PlanId::new(self.state.next_plan);
            self.state.next_plan += 1;
            self.state.plans.insert(
                plan_id,
                Plan {
                    id: plan_id,
                    intent: id_new,
                    state: sim_core::command::PlanState::Draft,
                    baseline: None,
                    amendments: Vec::new(),
                    runs: std::collections::BTreeMap::new(),
                },
            );
            plan_id
        } else {
            hold.plan
        };
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
                plan: plan_id,
                withdrawn_day: None,
            },
        );
        let (evidence, opening) = if fed == held {
            (
                format!(
                    "{base_text} is the side's and no supply intent for its hex is in force; the forces limit names {source} as the source"
                ),
                format!(
                    "{} holds {base_text}, which is the side's, and no supply for it is in force",
                    hold.id
                ),
            )
        } else {
            (
                format!(
                    "{base_text} is not the side's, the side's men in its hex are fed through {fed_text}, and no supply intent for its hex is in force; the forces limit names {source} as the source"
                ),
                format!(
                    "{} holds {base_text}, which is not the side's, and the side's men in its hex are fed through {fed_text}, for which no supply is in force",
                    hold.id
                ),
            )
        };
        let reason = serving(hold, plan)
            .evidence(evidence, today)
            .binding(format!("{} is held and its plan is released", hold.id))
            .worked(format!(
                "{opening}: the staff forms {id_new}, keep {fed_text} supplied from {source}, under it, with a floor of {floor_days} {} (the margin at {}) and no interval, so that the staff uses 15 days. Its forces limit is {}'s.",
                crate::units::days_word(floor_days),
                risk.name(),
                hold.id
            ))
            .would_change("a supply intent for the base already in force")
            .would_change("no source in the forces limit")
            .speaks_of(&self.state, LimitKind::Source)
            .done();
        if in_scope {
            self.lines.push(format!(
                "{} holds {base_text}: it would form {id_new}, keep {fed_text} supplied from {source}, under it.",
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

/// `words` with its first letter a capital.
fn capitalised(words: &str) -> String {
    let mut chars = words.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(chars).collect()
    })
}
