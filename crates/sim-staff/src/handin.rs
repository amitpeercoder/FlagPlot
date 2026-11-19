//! What the staff hands in for a day, being built: commands, orders and
//! decisions to give no order, each with its reason. Orders are drafted
//! against a force that may not exist yet (a forming order makes it) and
//! given their numbers when the hand-in is finished, in the order intake
//! will take the forming orders in (decision 0107, ruling 7: a side numbers
//! its own forces, so the staff knows the number the new force will have).

use std::collections::BTreeSet;

use sim_core::command::{
    CommandKind, NoticeBasis, NoticeKind, PlanId, Reason, StaffCommand, StaffDecisions,
    StaffNotice, StaffOrder,
};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, ShipId, TaskForceId};
use sim_core::orders::{BaseOrder, Cargo, OrderKind, SpeedSetting, TaskForceOrder};

/// A force an order is for: one that exists, or the `k`-th the staff forms
/// this day.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Who {
    Existing(TaskForceId),
    Formed(usize),
}

/// One order to a force.
#[derive(Clone, Debug)]
pub(crate) enum Step {
    Route {
        waypoints: Vec<Hex>,
        act_at: BTreeSet<Hex>,
    },
    Unload,
    /// A speed for the force (decision 0145, 5).
    Speed(SpeedSetting),
    LoadTo {
        weight_t: Fixed,
        /// The most a ship is to hold (decision 0162, 5): named on a supply
        /// run by night.
        each_t: Option<Fixed>,
    },
    Join {
        into: Who,
    },
    /// Give the force a charge, or clear it (decision 0121).
    Escort {
        charge: Option<Who>,
    },
}

#[derive(Clone, Debug)]
enum Draft {
    Form { source: BaseId, ships: Vec<ShipId> },
    Task { who: Who, step: Step },
}

#[derive(Clone, Debug)]
struct DraftOrder {
    intent: IntentId,
    draft: Draft,
    reason: Reason,
}

/// The hand-in of one day in the making.
#[derive(Default)]
pub(crate) struct Out {
    commands: Vec<StaffCommand>,
    drafts: Vec<DraftOrder>,
    no_orders: Vec<Reason>,
    notices: Vec<StaffNotice>,
    formed: usize,
}

impl Out {
    /// Hands in a command and gives its index in the final hand-in.
    pub(crate) fn command(&mut self, kind: CommandKind, reason: Reason) -> usize {
        self.commands.push(StaffCommand { kind, reason });
        self.commands.len() - 1
    }

    /// Hands in a notice, in the order made: of `kind` for `plan` (and `run`
    /// where there is one), resting on the reason `rests_on` names.
    pub(crate) fn notice(
        &mut self,
        kind: NoticeKind,
        plan: PlanId,
        run: Option<u32>,
        rests_on: NoticeBasis,
    ) {
        self.notices.push(StaffNotice {
            kind,
            plan,
            run,
            rests_on,
        });
    }

    /// The forming orders drafted so far.
    pub(crate) fn formed(&self) -> usize {
        self.formed
    }

    /// Hands in a decision to give no order and gives its index in the
    /// final hand-in.
    pub(crate) fn no_order(&mut self, reason: Reason) -> usize {
        self.no_orders.push(reason);
        self.no_orders.len() - 1
    }

    /// Drafts a forming order at `source` and says which force it makes.
    pub(crate) fn form(
        &mut self,
        intent: IntentId,
        source: BaseId,
        ships: Vec<ShipId>,
        reason: Reason,
    ) -> Who {
        let who = Who::Formed(self.formed);
        self.formed += 1;
        self.drafts.push(DraftOrder {
            intent,
            draft: Draft::Form { source, ships },
            reason,
        });
        who
    }

    pub(crate) fn task(&mut self, intent: IntentId, who: Who, step: Step, reason: Reason) {
        self.drafts.push(DraftOrder {
            intent,
            draft: Draft::Task { who, step },
            reason,
        });
    }

    /// The hand-in, with the forces the day's forming orders make numbered
    /// as intake will number them: from `next`, the forming orders by the
    /// base they are at and then in the order handed in. Forming orders are
    /// listed first, then the rest in the order drafted.
    pub(crate) fn finish(self, next: TaskForceId) -> StaffDecisions {
        let mut at: Vec<(BaseId, usize)> = Vec::new();
        for draft in &self.drafts {
            if let Draft::Form { source, .. } = &draft.draft {
                at.push((*source, at.len()));
            }
        }
        at.sort();
        let id_of = |k: usize| -> TaskForceId {
            let rank = at.iter().position(|(_, index)| *index == k).unwrap_or(0);
            TaskForceId::new(next.raw() + u32::try_from(rank).unwrap_or(0))
        };
        let force = |who: Who| match who {
            Who::Existing(id) => id,
            Who::Formed(k) => id_of(k),
        };
        let mut orders = Vec::new();
        for draft in &self.drafts {
            if let Draft::Form { source, ships } = &draft.draft {
                orders.push(StaffOrder {
                    kind: OrderKind::Base {
                        target: *source,
                        order: BaseOrder::FormTaskForce {
                            ships: ships.clone(),
                        },
                    },
                    intent: draft.intent,
                    reason: draft.reason.clone(),
                });
            }
        }
        for draft in &self.drafts {
            if let Draft::Task { who, step } = &draft.draft {
                let order = match step {
                    Step::Route { waypoints, act_at } => TaskForceOrder::SetRoute {
                        waypoints: waypoints.clone(),
                        act_at: act_at.clone(),
                        stand_at: BTreeSet::new(),
                    },
                    Step::Unload => TaskForceOrder::Unload,
                    Step::Speed(speed) => TaskForceOrder::SetSpeed(*speed),
                    Step::LoadTo { weight_t, each_t } => TaskForceOrder::LoadTo {
                        cargo: Cargo::Supply,
                        weight_t: *weight_t,
                        each_t: *each_t,
                    },
                    Step::Join { into } => TaskForceOrder::JoinTo(force(*into)),
                    Step::Escort { charge } => TaskForceOrder::Escort(charge.map(force)),
                };
                orders.push(StaffOrder {
                    kind: OrderKind::TaskForce {
                        target: force(*who),
                        order,
                    },
                    intent: draft.intent,
                    reason: draft.reason.clone(),
                });
            }
        }
        StaffDecisions {
            orders,
            commands: self.commands,
            no_orders: self.no_orders,
            notices: self.notices,
        }
    }
}
