//! The override (decision 0107, step 3, ruling 23): step 2 of the day. A
//! commander's order to a unit that lies in the forces limit of an intent in
//! force is taken in at once, before the staff is asked, and holds the unit:
//! the staff's orders to it are refused for as long as the hold stands
//! (rulings 18 and 30). It once ran after the morning's refuelling on purpose;
//! since decision 0112 ships take fuel in the pulses and the order of the two
//! no longer matters.
//!
//! Reads: the side's pending orders, its intents in force and their forces
//! limits, the truth's ships, forces and air groups. Writes: the orders taken in, the
//! side's holds, and a `HeldByOrder` event for each hold, citing the order and
//! its acceptance. Draws no random number.

use std::collections::BTreeSet;

use crate::command::{Addressee, Hold, HoldMaker, HoldTarget};
use crate::events::{Cause, EventKind};
use crate::ids::ShipId;
use crate::orders::{BaseOrder, Issuer, OrderKind, OrderRequest, TaskForceOrder};
use crate::state::Side;
use crate::world::World;

impl World {
    /// Step 2 of the day: takes in, ahead of the staff, each side's pending
    /// commander orders that are for a unit in the forces limit of an intent
    /// in force, and holds what they were for. The orders are walked in the
    /// order intake sorts them. Orders that are not overrides stay pending,
    /// in the order handed in, for step 5. A side with no intent in force is
    /// not touched.
    pub(crate) fn override_by_orders(&mut self, day: u32) {
        for side in Side::ALL {
            if self.command[&side].intents_in_force().next().is_none() {
                continue;
            }
            let Some(pending) = self.pending.get(&side) else {
                continue;
            };
            if pending.is_empty() {
                continue;
            }
            let pending = std::mem::take(self.pending.get_mut(&side).expect("checked"));
            let mut order: Vec<usize> = (0..pending.len()).collect();
            order.sort_by_key(|i| {
                let kind = &pending[*i].kind;
                (!kind.forms_task_force(), kind.target())
            });
            let mut taken = BTreeSet::new();
            for i in order {
                let Some(targets) = self.override_targets(side, &pending[i]) else {
                    continue;
                };
                taken.insert(i);
                let request = pending[i].clone();
                let (order_id, answer, accepted) =
                    self.take_in_order(day, None, side, request, Vec::new());
                if !accepted {
                    continue;
                }
                for target in targets {
                    if self.command[&side].holds.contains_key(&target) {
                        continue;
                    }
                    self.command
                        .get_mut(&side)
                        .expect("side has command state")
                        .holds
                        .insert(
                            target,
                            Hold {
                                day,
                                by: HoldMaker::Order(order_id),
                            },
                        );
                    self.events.record(
                        day,
                        None,
                        EventKind::HeldByOrder {
                            side,
                            target,
                            order: order_id,
                        },
                        vec![Cause::Order(order_id), Cause::Event(answer)],
                        None,
                    );
                }
            }
            let rest: Vec<OrderRequest> = pending
                .into_iter()
                .enumerate()
                .filter(|(i, _)| !taken.contains(i))
                .map(|(_, request)| request)
                .collect();
            self.pending.insert(side, rest);
        }
    }

    /// What taking in `request` would hold, if it is an override: its unit
    /// lies in the forces limit of some intent of the side in force (a task
    /// force listed, or one whose every ship the limit lists, or one an intent
    /// in force is addressed to; a forming order
    /// naming at least one ship of a pool; an air group some intent in force
    /// may use, which holds the whole group; a land unit some intent in force
    /// may use, decision 0141, 14). `None` if it is not an override.
    fn override_targets(&self, side: Side, request: &OrderRequest) -> Option<Vec<HoldTarget>> {
        // Only the commander's own orders override: a scenario's scripted
        // order with the issuer `staff` is a statement of the scenario, not
        // an order of his (decision 0107, "A staff's order").
        if request.origin.issuer == Issuer::Staff {
            return None;
        }
        let limits: Vec<_> = self.command[&side]
            .intents_in_force()
            .filter_map(|intent| intent.limits.forces.as_ref())
            .collect();
        // An air group an intent in force may use, by its forces limit read
        // up the tree or by its addressee (decision 0115, 3).
        if let OrderKind::AirGroup { target, .. } = &request.kind {
            let usable = self.command[&side]
                .intents_in_force()
                .any(|intent| self.may_use_group(side, intent.id, *target));
            return usable.then(|| vec![HoldTarget::AirGroup(*target)]);
        }
        // A land unit a task in force may use (decision 0141, 14).
        if let OrderKind::LandUnit { target, .. } = &request.kind {
            let usable = self.command[&side]
                .intents_in_force()
                .any(|intent| self.may_use_land_unit(side, intent.id, *target));
            return usable.then(|| vec![HoldTarget::LandUnit(*target)]);
        }
        match &request.kind {
            // A word about the radio does not take a force out of the
            // staff's hands (decision 0136, 15).
            OrderKind::TaskForce {
                order: TaskForceOrder::SetRadioSilence { .. },
                ..
            } => None,
            // A force listed or inside a limit, or one an intent in force is
            // addressed to (decision 0124, 6).
            OrderKind::TaskForce { target, .. } => (limits
                .iter()
                .any(|forces| self.force_in_forces_limit(side, forces, *target))
                || self.command[&side]
                    .intents_in_force()
                    .any(|intent| intent.addressee == Addressee::TaskForce(*target)))
            .then(|| vec![HoldTarget::TaskForce(*target)]),
            OrderKind::Base {
                order: BaseOrder::FormTaskForce { ships },
                ..
            } => {
                let held: BTreeSet<ShipId> = ships
                    .iter()
                    .copied()
                    .filter(|ship| limits.iter().any(|forces| self.in_pool(forces, *ship)))
                    .collect();
                (!held.is_empty()).then(|| held.into_iter().map(HoldTarget::Ship).collect())
            }
            _ => None,
        }
    }
}
