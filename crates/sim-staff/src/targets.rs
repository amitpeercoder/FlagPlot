//! What to strike first (decision 0190): the targets limit and the waters to
//! strike first as the staff reads them for the bombers of one task.
//!
//! - **The list.** A targets limit on the chain of a task that strikes ships
//!   (a strike-ships task, or "operate a carrier force"), the nearest set,
//!   gives each bomber's target list: the limit's kinds in its order, at
//!   every risk. For a carrier force, where its own chain has none, the
//!   lowest-numbered deny in force whose plan is released and whose nearest
//!   forces limit names the force, and whose chain carries a targets limit,
//!   gives it (rule 28).
//! - **The waters.** A deny binds the bombers of a task when its plan is
//!   released and the targets limit read for it has `waters_first` (rule
//!   27): a deny above the task, read on the task's chain (the nearest
//!   replaces); and, for a carrier force, a deny that names the force and
//!   is not above its task, read on the deny's chain. The waters to strike
//!   first are the hexes of every deny that binds, together (rule 33).
//!
//! With no targets limit anywhere nothing here gives anything, and every
//! order and every choice is today's (rule 27).

use std::collections::BTreeSet;

use sim_core::command::{Intent, IntentKind};
use sim_core::hex::Hex;
use sim_core::ids::{IntentId, TaskForceId};
use sim_core::refdata::ShipKind;

use crate::air::Day;
use crate::deny::waters;

/// A deny whose waters a task's bombers strike first.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WatersDeny {
    pub deny: IntentId,
    /// The intent the targets limit with `waters_first` is set on.
    pub limit_on: IntentId,
    /// The deny's centre and distance in hexes.
    pub centre: Hex,
    pub distance: u32,
    /// Its waters on the map, in hex order.
    pub hexes: Vec<Hex>,
    /// Whether the deny stands above the task (and so already reaches the
    /// search arc of a carrier force under it, as today).
    pub above: bool,
    /// Whether it is an order to destroy a force, whose waters are first by
    /// the order itself and by no targets limit (decision 0220, 2).
    pub destroy: bool,
}

/// What the targets limits on and around a task give its bombers.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct StrikeTargets {
    /// The kinds a targets limit gives, in order, with the intent it is set
    /// on; `None`: the risk's list, as today.
    pub kinds: Option<(Vec<ShipKind>, IntentId)>,
    /// The waters to strike first: the hexes of every deny that binds.
    pub waters: BTreeSet<Hex>,
    /// The denies that bind, by number.
    pub denies: Vec<WatersDeny>,
}

impl StrikeTargets {
    /// The list the staff gives a bomber: the limit's kinds, or the risk's.
    pub fn list_or(&self, risk_list: Vec<ShipKind>) -> Vec<ShipKind> {
        self.kinds
            .as_ref()
            .map_or(risk_list, |(kinds, _)| kinds.clone())
    }

    /// The lowest-numbered deny whose waters hold `hex`.
    pub fn deny_of(&self, hex: Hex) -> Option<&WatersDeny> {
        self.denies.iter().find(|d| d.hexes.contains(&hex))
    }

    /// The deny an order setting the waters serves: the lowest numbered.
    pub fn first_deny(&self) -> Option<&WatersDeny> {
        self.denies.first()
    }

    /// The words for the waters: "the waters of intent:3 (deny the waters
    /// within 1 hexes of (4,0), 7 hexes), first by the targets limit on
    /// intent:3", joined for every deny.
    pub fn waters_words(&self) -> String {
        self.denies
            .iter()
            .map(|d| {
                if d.destroy {
                    return format!(
                        "the waters of {}, the order to destroy a force (the waters within {} hexes of {}, {} hex(es) on the map), first by that order",
                        d.deny,
                        d.distance,
                        d.centre,
                        d.hexes.len()
                    );
                }
                format!(
                    "the waters of {}, {} ({} hex(es) on the map), first by the targets limit set on {}",
                    d.deny,
                    crate::reasons::waters_words(d.centre, d.distance),
                    d.hexes.len(),
                    d.limit_on
                )
            })
            .collect::<Vec<_>>()
            .join("; ")
    }
}

/// The kinds of a list, in words: "every kind" for the default list.
pub(crate) fn kinds_words(kinds: &[ShipKind]) -> String {
    if kinds == ShipKind::ALL {
        return "every kind".to_string();
    }
    kinds
        .iter()
        .map(|k| k.name())
        .collect::<Vec<_>>()
        .join(", ")
}

/// What a group has or is given as its waters to strike first, in words.
pub(crate) fn waters_had(hexes: &BTreeSet<Hex>) -> String {
    if hexes.is_empty() {
        return "no waters to strike first".to_string();
    }
    format!(
        "waters to strike first of {} hex(es): {}",
        hexes.len(),
        hexes
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    )
}

/// The reason speaks of the targets limits that put the waters first
/// (decision 0222, rule 5): the limit each deny that binds has set on its
/// chain; with no deny, the targets limit of the task's chain.
pub(crate) fn speaks_of_waters(
    builder: crate::reasons::ReasonBuilder,
    state: &sim_core::command::CommandState,
    task: IntentId,
    targets: &StrikeTargets,
) -> crate::reasons::ReasonBuilder {
    use sim_core::command::LimitKind;
    if targets.denies.is_empty() {
        return builder.speaks_of_on(
            LimitKind::Targets,
            state.limit_set_on(task, LimitKind::Targets),
        );
    }
    targets.denies.iter().fold(builder, |b, d| {
        if d.destroy {
            // The order puts its waters first itself: no limit says it.
            b
        } else {
            b.speaks_of_on(LimitKind::Targets, d.limit_on)
        }
    })
}

/// What binds a group's waters to strike first, in words.
pub(crate) fn waters_binding(targets: &StrikeTargets) -> String {
    if targets.denies.is_empty() {
        "no deny in force with a targets limit that puts its waters first binds the group: its waters to strike first are cleared".to_string()
    } else {
        format!(
            "a contact in these waters is struck before every one outside them: {}",
            targets.waters_words()
        )
    }
}

/// The words a strike task's doctrine adds after its list: nothing with no
/// waters, else ", the waters of intent:N first".
pub(crate) fn wanted_waters_words(targets: &StrikeTargets) -> String {
    if targets.denies.is_empty() {
        return String::new();
    }
    format!(
        ", a contact in {} first",
        targets
            .denies
            .iter()
            .map(|d| format!("the waters of {}", d.deny))
            .collect::<Vec<_>>()
            .join(" or ")
    )
}

impl Day<'_, '_> {
    /// The reason of an order that sets a group's waters serves the
    /// lowest-numbered deny that binds it, with that deny's plan (the
    /// architect's ruling on decision 0190, 33): the order is handed in
    /// under the task the group serves, which the engine's forces check
    /// reads. An order that clears the waters serves that task.
    pub(crate) fn waters_serve(
        &self,
        targets: &StrikeTargets,
        reason: &mut sim_core::command::Reason,
    ) {
        let Some(first) = targets.first_deny() else {
            return;
        };
        let Some(deny) = self.state.intents.get(&first.deny) else {
            return;
        };
        let Some(plan) = self.plan_of(deny) else {
            return;
        };
        reason.serves.intent = Some(deny.id);
        reason.serves.plan = Some(plan.id);
        reason.serves.amendment = plan.amendments.last().map(|a| a.number);
    }

    /// What the targets limits give the bombers of `task`, a task that
    /// strikes ships; `force` is the carrier force a carrier task operates.
    pub(crate) fn strike_targets(
        &self,
        task: &Intent,
        force: Option<TaskForceId>,
    ) -> StrikeTargets {
        let mut out = StrikeTargets::default();
        let own = self.state.targets_read_up(task.id);
        let ancestors: Vec<IntentId> = self
            .state
            .ancestors(task.id)
            .into_iter()
            .map(|i| i.id)
            .collect();
        if let Some((limit, on)) = own {
            out.kinds = Some((limit.kinds.clone(), on));
            if limit.waters_first {
                for id in &ancestors {
                    if let Some(deny) = self.state.intents.get(id) {
                        self.push_deny(&mut out, deny, on, true);
                    }
                }
            }
        }
        if let Some(force) = force {
            let naming: Vec<Intent> = self
                .state
                .denies_naming(force)
                .into_iter()
                .filter(|d| !ancestors.contains(&d.id))
                .cloned()
                .collect();
            for deny in &naming {
                if !self.released(deny) {
                    continue;
                }
                let Some((limit, on)) = self.state.targets_read_up(deny.id) else {
                    continue;
                };
                if out.kinds.is_none() {
                    out.kinds = Some((limit.kinds.clone(), on));
                }
                if limit.waters_first {
                    self.push_deny(&mut out, deny, on, false);
                }
            }
        }
        // Decision 0220, 2: an order to destroy a force above the task puts
        // its waters first, with no targets limit.
        for id in &ancestors {
            if let Some(order) = self.state.intents.get(id) {
                if matches!(
                    order.kind,
                    IntentKind::DestroyForce {
                        search_only: false,
                        ..
                    }
                ) {
                    self.push_deny(&mut out, order, order.id, true);
                }
            }
        }
        out.denies.sort_by_key(|d| d.deny);
        for d in &out.denies {
            out.waters.extend(d.hexes.iter().copied());
        }
        out
    }

    fn push_deny(&self, out: &mut StrikeTargets, deny: &Intent, on: IntentId, above: bool) {
        let destroy = match deny.kind {
            IntentKind::Deny { .. } => false,
            IntentKind::DestroyForce {
                search_only: false, ..
            } => true,
            _ => return,
        };
        let Some((hex, distance)) = self.waters_of(deny) else {
            return;
        };
        if !deny.in_force() || !self.released(deny) || out.denies.iter().any(|d| d.deny == deny.id)
        {
            return;
        }
        out.denies.push(WatersDeny {
            deny: deny.id,
            limit_on: on,
            centre: hex,
            distance,
            hexes: waters(self.view, hex, distance),
            above,
            destroy,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_no_limit_the_risk_list_stands() {
        let none = StrikeTargets::default();
        assert_eq!(
            none.list_or(vec![ShipKind::Carrier]),
            vec![ShipKind::Carrier]
        );
        assert!(none.waters.is_empty());
        assert!(none.first_deny().is_none());
    }

    #[test]
    fn a_limit_gives_its_kinds_in_order() {
        let t = StrikeTargets {
            kinds: Some((
                vec![ShipKind::Transport, ShipKind::Carrier],
                IntentId::new(2),
            )),
            ..StrikeTargets::default()
        };
        assert_eq!(
            t.list_or(ShipKind::ALL.to_vec()),
            vec![ShipKind::Transport, ShipKind::Carrier]
        );
        assert_eq!(kinds_words(&ShipKind::ALL), "every kind");
        assert_eq!(
            kinds_words(&[ShipKind::Transport, ShipKind::Carrier]),
            "transport, carrier"
        );
    }
}
