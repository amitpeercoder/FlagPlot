//! Taking commands in (decision 0107, items 1, 2 and 4): the commander's at
//! the first step of a day, the staff's after the staff is asked. Each is
//! checked against the side's command state and its own units, and accepted
//! or refused with a reason, every change an event with its causes.

use std::collections::BTreeMap;

use crate::command::{Command, Reason, StaffDecisions, Staffs};
use crate::orders::{Issuer, OrderRequest};
use crate::state::Side;
use crate::world::World;

impl World {
    /// Step 1 of the day: takes in the commander's commands handed in for
    /// the day, side by side in a fixed order and each side's in the order
    /// handed in.
    pub(crate) fn intake_commands(&mut self, day: u32) {
        let pending = std::mem::take(&mut self.pending_commands);
        for (side, commands) in pending {
            for kind in commands {
                self.take_in_command(
                    day,
                    side,
                    Command {
                        kind,
                        issuer: Issuer::Player,
                    },
                );
            }
        }
    }

    /// Takes in one command: checks it, applies it if it is sound, and
    /// records `CommandTaken` or `CommandRefused` and the follow-ups.
    pub(crate) fn take_in_command(&mut self, day: u32, side: Side, command: Command) {
        let _ = (day, side, command);
        todo!("decision 0107: built by the command task")
    }

    /// Step 4 of the day: asks each side's staff, if it has one, on its view
    /// as it now stands, and adds what it hands in to what was given back
    /// for it (`World::submit_staff`). Returns each side's hand-in for the
    /// day; a side with no staff and nothing given back is not in it.
    pub(crate) fn ask_staffs(&mut self, staffs: &Staffs<'_>) -> BTreeMap<Side, StaffDecisions> {
        let mut handed_in = std::mem::take(&mut self.pending_staff);
        for side in Side::ALL {
            if let Some(staff) = staffs.get(side) {
                let decisions = staff.decide(self.side_view(side));
                let entry = handed_in.entry(side).or_default();
                entry.orders.extend(decisions.orders);
                entry.commands.extend(decisions.commands);
                entry.no_orders.extend(decisions.no_orders);
            }
        }
        handed_in
    }

    /// Step 5 of the day, its first part: takes in each side's staff
    /// commands in the order handed in, and records each decision to give
    /// no order with its reason. The staff's orders are taken in with the
    /// commander's, by `intake_orders`.
    pub(crate) fn intake_staff_commands(
        &mut self,
        day: u32,
        handed_in: &BTreeMap<Side, StaffDecisions>,
    ) {
        if handed_in.is_empty() {
            return;
        }
        let _ = day;
        todo!("decision 0107: built by the command task")
    }

    /// Takes in one order a staff handed in, with its reason: refused
    /// `OutsideForcesLimit` if its intent is not in force or its unit is not
    /// in that intent's forces limit, else checked as any order is; then its
    /// reason is recorded (`StaffReason`), citing the order and its
    /// acceptance or refusal.
    pub(crate) fn take_in_staff_order(
        &mut self,
        day: u32,
        side: Side,
        request: OrderRequest,
        reason: &Reason,
    ) {
        let _ = (day, side, request, reason);
        todo!("decision 0107: built by the command task")
    }
}
