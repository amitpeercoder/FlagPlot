//! Taking commands in (decision 0107, items 1, 2 and 4): the commander's at
//! the first step of a day, the staff's after the staff is asked. Each is
//! checked against the side's command state and its own units, and accepted
//! or refused with a reason, every change an event with its causes.

use std::collections::BTreeMap;

use crate::command::{
    Amendment, AmendmentChange, Baseline, Command, CommandKind, CommandRefusal, CommandState,
    Commitment, ForcesLimit, Intent, IntentKind, IntentLimits, Leave, Limit, Plan, PlanId,
    PlanOutcome, PlanState, Reason, Run, StaffDecisions, Staffs,
};
use crate::events::{Cause, EventKind};
use crate::fixed::Fixed;
use crate::ids::{BaseId, EventId, IntentId, ShipId};
use crate::orders::{BaseOrder, Issuer, OrderKind, OrderRequest, RejectReason};
use crate::state::{ShipLocation, Side};
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
    /// records `CommandTaken` or `CommandRefused` and the follow-ups. A
    /// refused command changes nothing. `CommandTaken` and `CommandRefused`
    /// have no causes: they are the root record. Each follow-up cites the
    /// `CommandTaken` and is recorded after it.
    pub(crate) fn take_in_command(&mut self, day: u32, side: Side, command: Command) -> EventId {
        let Command { kind, issuer } = command;
        match self.check_command(day, side, issuer, &kind) {
            Err(reason) => self.events.record(
                day,
                None,
                EventKind::CommandRefused {
                    side,
                    issuer,
                    command: kind,
                    reason,
                },
                Vec::new(),
                None,
            ),
            Ok(()) => {
                let follow_ups = self.apply_command(day, side, &kind);
                let taken = self.events.record(
                    day,
                    None,
                    EventKind::CommandTaken {
                        side,
                        issuer,
                        command: kind,
                    },
                    Vec::new(),
                    None,
                );
                for follow_up in follow_ups {
                    self.events
                        .record(day, None, follow_up, vec![Cause::Event(taken)], None);
                }
                taken
            }
        }
    }

    /// Whether `ship` is one of `side`'s own and not sunk.
    fn own_ship_afloat(&self, side: Side, ship: ShipId) -> bool {
        self.truth
            .ships
            .get(&ship)
            .is_some_and(|ship| ship.side == side && ship.location != ShipLocation::Sunk)
    }

    /// Whether `base` is one of `side`'s own.
    fn own_base(&self, side: Side, base: BaseId) -> bool {
        self.truth
            .bases
            .get(&base)
            .is_some_and(|base| base.side == side)
    }

    /// Whether `ship` is in the pool of `forces`: listed in it, or in a task
    /// force it lists (afloat in it, or away from it at the yard).
    fn in_pool(&self, forces: &ForcesLimit, ship: ShipId) -> bool {
        if forces.ships.contains(&ship) {
            return true;
        }
        self.truth.ships.get(&ship).is_some_and(|ship| {
            matches!(
                ship.location,
                ShipLocation::TaskForce(force) | ShipLocation::Yard(force)
                    if forces.task_forces.contains(&force)
            )
        })
    }

    /// Checks a command against the side's command state and its own units,
    /// changing nothing. The one place the rules of intake are written.
    fn check_command(
        &self,
        day: u32,
        side: Side,
        issuer: Issuer,
        kind: &CommandKind,
    ) -> Result<(), CommandRefusal> {
        if !kind.allowed_from(issuer) {
            return Err(CommandRefusal::NotYours);
        }
        let state = &self.command[&side];
        // An approved plan of the side, with its intent's forces limit.
        let approved_plan = |plan: PlanId| -> Result<&Plan, CommandRefusal> {
            let plan = state
                .plans
                .get(&plan)
                .ok_or(CommandRefusal::NoSuchPlan(plan))?;
            if !plan.state.approved() {
                return Err(CommandRefusal::WrongPlanState(plan.state));
            }
            Ok(plan)
        };
        let forces_of = |plan: &Plan| -> Result<&ForcesLimit, CommandRefusal> {
            state
                .intents
                .get(&plan.intent)
                .and_then(|intent| intent.limits.forces.as_ref())
                .ok_or(CommandRefusal::NoForcesLimit)
        };
        match kind {
            CommandKind::SetIntent { intent, kind } => {
                if intent.raw() == 0
                    || state
                        .intents
                        .get(intent)
                        .is_some_and(|existing| !existing.in_force())
                {
                    return Err(CommandRefusal::IntentNumberNotUsable(*intent));
                }
                let IntentKind::KeepSupplied {
                    base,
                    floor_days,
                    interval_days,
                } = kind;
                if !self.own_base(side, *base) {
                    return Err(CommandRefusal::NotYourBase(*base));
                }
                if *floor_days < 1 || *interval_days == Some(0) {
                    return Err(CommandRefusal::NoDays);
                }
                Ok(())
            }
            CommandKind::WithdrawIntent { intent } => {
                self.in_force_intent(side, *intent)?;
                Ok(())
            }
            CommandKind::SetLimit(limit) => {
                if let Some(intent) = limit.intent() {
                    self.in_force_intent(side, intent)?;
                }
                match limit {
                    Limit::Forces { limit, .. } => {
                        for ship in &limit.ships {
                            if !self.own_ship_afloat(side, *ship) {
                                return Err(CommandRefusal::NotYourShip(*ship));
                            }
                        }
                        for force in &limit.task_forces {
                            let own = self
                                .truth
                                .task_forces
                                .get(force)
                                .is_some_and(|force| force.side == side);
                            if !own {
                                return Err(CommandRefusal::NotYourForce(*force));
                            }
                        }
                        if !self.own_base(side, limit.source) {
                            return Err(CommandRefusal::NotYourBase(limit.source));
                        }
                    }
                    Limit::Area { hexes, .. } => {
                        if hexes.is_empty() || hexes.iter().any(|hex| !self.map.contains(*hex)) {
                            return Err(CommandRefusal::BadArea);
                        }
                    }
                    Limit::Protected { ships, .. } => {
                        for ship in ships {
                            if !self.own_ship_afloat(side, *ship) {
                                return Err(CommandRefusal::NotYourShip(*ship));
                            }
                        }
                    }
                    Limit::Priority { .. } | Limit::Risk { .. } => {}
                    Limit::Reserve {
                        base,
                        supply_t,
                        fuel_t,
                    } => {
                        if !self.own_base(side, *base) {
                            return Err(CommandRefusal::NotYourBase(*base));
                        }
                        for weight in [*supply_t, *fuel_t] {
                            if weight < Fixed::ZERO {
                                return Err(CommandRefusal::BadWeight(weight));
                            }
                        }
                    }
                }
                Ok(())
            }
            CommandKind::SetLeave { .. } => Ok(()),
            CommandKind::Approve { plan } => {
                let found = state
                    .plans
                    .get(plan)
                    .ok_or(CommandRefusal::NoSuchPlan(*plan))?;
                let leave = match found.state {
                    PlanState::Draft => Leave::Prepare,
                    PlanState::Preparing => Leave::Launch,
                    other => return Err(CommandRefusal::WrongPlanState(other)),
                };
                let given = match leave {
                    Leave::Prepare => state.leaves.prepare,
                    Leave::Launch => state.leaves.launch,
                };
                if issuer == Issuer::Staff && !given {
                    return Err(CommandRefusal::NoLeave(leave));
                }
                Ok(())
            }
            CommandKind::CancelPlan { plan } => {
                let found = state
                    .plans
                    .get(plan)
                    .ok_or(CommandRefusal::NoSuchPlan(*plan))?;
                if found.state.ended() {
                    return Err(CommandRefusal::WrongPlanState(found.state));
                }
                Ok(())
            }
            CommandKind::CommitShips {
                plan,
                run,
                ships,
                from_day,
                to_day,
            } => {
                let found = approved_plan(*plan)?;
                let forces = forces_of(found)?;
                if !(day <= *from_day && from_day <= to_day) {
                    return Err(CommandRefusal::BadDays);
                }
                if let Some(run) = run {
                    if !found.runs.contains_key(run) {
                        return Err(CommandRefusal::NoSuchRun(*run));
                    }
                }
                for ship in ships {
                    if !self.own_ship_afloat(side, *ship) {
                        return Err(CommandRefusal::NotYourShip(*ship));
                    }
                    if !self.in_pool(forces, *ship) {
                        return Err(CommandRefusal::NotInPool(*ship));
                    }
                    let overlaps = state.commitments.get(ship).is_some_and(|list| {
                        list.iter()
                            .any(|c| c.from_day <= *to_day && *from_day <= c.to_day)
                    });
                    if overlaps {
                        return Err(CommandRefusal::AlreadyCommitted(*ship));
                    }
                }
                Ok(())
            }
            CommandKind::FreeShips { plan, ships } => {
                if !state.plans.contains_key(plan) {
                    return Err(CommandRefusal::NoSuchPlan(*plan));
                }
                for ship in ships {
                    let committed = state
                        .commitments
                        .get(ship)
                        .is_some_and(|list| list.iter().any(|c| c.plan == *plan));
                    if !committed {
                        return Err(CommandRefusal::NotCommitted(*ship));
                    }
                }
                Ok(())
            }
            CommandKind::AddRun { plan, run } => {
                let found = approved_plan(*plan)?;
                let forces = forces_of(found)?;
                if run.ships.is_empty() {
                    return Err(CommandRefusal::NoShips);
                }
                for ship in &run.ships {
                    if !self.own_ship_afloat(side, *ship) {
                        return Err(CommandRefusal::NotYourShip(*ship));
                    }
                    if !self.in_pool(forces, *ship) {
                        return Err(CommandRefusal::NotInPool(*ship));
                    }
                }
                if run.load_t <= Fixed::ZERO {
                    return Err(CommandRefusal::BadWeight(run.load_t));
                }
                let days = run.days;
                if !(day <= days.load
                    && days.load <= days.sail
                    && days.sail <= days.arrive
                    && days.arrive <= days.free)
                {
                    return Err(CommandRefusal::BadDays);
                }
                Ok(())
            }
            CommandKind::TimeRun { plan, run, days } => {
                let found = approved_plan(*plan)?;
                if !found.runs.contains_key(run) {
                    return Err(CommandRefusal::NoSuchRun(*run));
                }
                if !(days.load <= days.sail
                    && days.sail <= days.arrive
                    && days.arrive <= days.free
                    && days.free >= day)
                {
                    return Err(CommandRefusal::BadDays);
                }
                Ok(())
            }
            CommandKind::MarkPlan { plan, .. } => {
                approved_plan(*plan)?;
                Ok(())
            }
        }
    }

    /// The intent of that number if it is in force.
    fn in_force_intent(&self, side: Side, intent: IntentId) -> Result<&Intent, CommandRefusal> {
        self.command[&side]
            .intents
            .get(&intent)
            .filter(|intent| intent.in_force())
            .ok_or(CommandRefusal::NoSuchIntent(intent))
    }

    /// Applies a command `check_command` passed and returns its follow-up
    /// events, to be recorded after `CommandTaken`.
    fn apply_command(&mut self, day: u32, side: Side, kind: &CommandKind) -> Vec<EventKind> {
        let mut follow_ups = Vec::new();
        match kind {
            CommandKind::SetIntent { intent, kind } => {
                let state = self.command.get_mut(&side).expect("side has command state");
                // The plan the change falls on, if the intent exists.
                let existing_plan = state.intents.get(intent).map(|existing| existing.plan);
                match existing_plan {
                    None => {
                        let plan = draft_plan(state, *intent);
                        state.intents.insert(
                            *intent,
                            Intent {
                                id: *intent,
                                kind: *kind,
                                limits: IntentLimits::default(),
                                set_day: day,
                                plan,
                                withdrawn_day: None,
                            },
                        );
                        follow_ups.push(EventKind::PlanDrafted {
                            side,
                            plan,
                            intent: *intent,
                        });
                    }
                    Some(plan_id) => {
                        state.intents.get_mut(intent).expect("intent exists").kind = *kind;
                        let plan_state = state.plans[&plan_id].state;
                        if plan_state.approved() {
                            let plan = state.plans.get_mut(&plan_id).expect("plan exists");
                            let number = amend(plan, day, AmendmentChange::Intent(*kind));
                            follow_ups.push(EventKind::PlanAmended {
                                side,
                                plan: plan_id,
                                number,
                            });
                        } else if plan_state.ended() {
                            let plan = draft_plan(state, *intent);
                            state.intents.get_mut(intent).expect("intent exists").plan = plan;
                            follow_ups.push(EventKind::PlanDrafted {
                                side,
                                plan,
                                intent: *intent,
                            });
                        }
                    }
                }
            }
            CommandKind::WithdrawIntent { intent } => {
                let state = self.command.get_mut(&side).expect("side has command state");
                let found = state.intents.get_mut(intent).expect("intent exists");
                found.withdrawn_day = Some(day);
                let plan_id = found.plan;
                let plan = state.plans.get_mut(&plan_id).expect("plan exists");
                if !plan.state.ended() {
                    let from = plan.state;
                    let to = if from == PlanState::Released {
                        PlanState::Complete
                    } else {
                        PlanState::Cancelled
                    };
                    plan.state = to;
                    follow_ups.push(EventKind::PlanStateChanged {
                        side,
                        plan: plan_id,
                        from,
                        to,
                    });
                }
            }
            CommandKind::SetLimit(limit) => {
                if let Limit::Reserve {
                    base,
                    supply_t,
                    fuel_t,
                } = limit
                {
                    let base = self.truth.bases.get_mut(base).expect("base exists");
                    base.supply_reserve_t = *supply_t;
                    base.fuel_reserve_t = *fuel_t;
                } else if let Some(intent_id) = limit.intent() {
                    let state = self.command.get_mut(&side).expect("side has command state");
                    let intent = state.intents.get_mut(&intent_id).expect("intent exists");
                    match limit {
                        Limit::Forces { limit, .. } => intent.limits.forces = Some(limit.clone()),
                        Limit::Area { hexes, .. } => intent.limits.area = Some(hexes.clone()),
                        Limit::Priority { rank, .. } => intent.limits.priority = *rank,
                        Limit::Protected { ships, .. } => {
                            intent.limits.protected = ships.clone();
                        }
                        Limit::Risk { risk, .. } => intent.limits.risk = *risk,
                        Limit::Reserve { .. } => {}
                    }
                    let plan_id = intent.plan;
                    let plan = state.plans.get_mut(&plan_id).expect("plan exists");
                    if plan.state.approved() {
                        let number = amend(plan, day, AmendmentChange::Limit(limit.clone()));
                        follow_ups.push(EventKind::PlanAmended {
                            side,
                            plan: plan_id,
                            number,
                        });
                    }
                }
            }
            CommandKind::SetLeave { leave, given } => {
                let state = self.command.get_mut(&side).expect("side has command state");
                match leave {
                    Leave::Prepare => state.leaves.prepare = *given,
                    Leave::Launch => state.leaves.launch = *given,
                }
            }
            CommandKind::Approve { plan } => {
                let state = self.command.get_mut(&side).expect("side has command state");
                let intent = state.plans[plan].intent;
                let baseline = {
                    let intent = &state.intents[&intent];
                    Baseline {
                        day,
                        kind: intent.kind,
                        limits: intent.limits.clone(),
                    }
                };
                let found = state.plans.get_mut(plan).expect("plan exists");
                let from = found.state;
                let to = match from {
                    PlanState::Draft => {
                        if found.baseline.is_none() {
                            found.baseline = Some(baseline);
                        }
                        PlanState::Preparing
                    }
                    _ => PlanState::Released,
                };
                found.state = to;
                follow_ups.push(EventKind::PlanStateChanged {
                    side,
                    plan: *plan,
                    from,
                    to,
                });
            }
            CommandKind::CancelPlan { plan } => {
                let state = self.command.get_mut(&side).expect("side has command state");
                let found = state.plans.get_mut(plan).expect("plan exists");
                let from = found.state;
                found.state = PlanState::Cancelled;
                follow_ups.push(EventKind::PlanStateChanged {
                    side,
                    plan: *plan,
                    from,
                    to: PlanState::Cancelled,
                });
            }
            CommandKind::CommitShips {
                plan,
                run,
                ships,
                from_day,
                to_day,
            } => {
                let state = self.command.get_mut(&side).expect("side has command state");
                for ship in ships {
                    let list = state.commitments.entry(*ship).or_default();
                    list.push(Commitment {
                        plan: *plan,
                        run: *run,
                        from_day: *from_day,
                        to_day: *to_day,
                    });
                    list.sort_by_key(|c| c.from_day);
                }
            }
            CommandKind::FreeShips { plan, ships } => {
                let state = self.command.get_mut(&side).expect("side has command state");
                for ship in ships {
                    if let Some(list) = state.commitments.get_mut(ship) {
                        list.retain(|c| c.plan != *plan);
                        if list.is_empty() {
                            state.commitments.remove(ship);
                        }
                    }
                }
            }
            CommandKind::AddRun { plan, run } => {
                let state = self.command.get_mut(&side).expect("side has command state");
                let intent = state.plans[plan].intent;
                let source = state.intents[&intent]
                    .limits
                    .forces
                    .as_ref()
                    .expect("the forces limit was checked")
                    .source;
                let found = state.plans.get_mut(plan).expect("plan exists");
                let number = found.runs.keys().next_back().map_or(1, |last| last + 1);
                found.runs.insert(
                    number,
                    Run {
                        number,
                        ships: run.ships.clone(),
                        load_t: run.load_t,
                        source,
                        days: run.days,
                    },
                );
            }
            CommandKind::TimeRun { plan, run, days } => {
                let state = self.command.get_mut(&side).expect("side has command state");
                let found = state.plans.get_mut(plan).expect("plan exists");
                found.runs.get_mut(run).expect("run exists").days = *days;
            }
            CommandKind::MarkPlan { plan, outcome } => {
                let state = self.command.get_mut(&side).expect("side has command state");
                let found = state.plans.get_mut(plan).expect("plan exists");
                let from = found.state;
                let to = match outcome {
                    PlanOutcome::Complete => PlanState::Complete,
                    PlanOutcome::Failed => PlanState::Failed,
                };
                found.state = to;
                follow_ups.push(EventKind::PlanStateChanged {
                    side,
                    plan: *plan,
                    from,
                    to,
                });
            }
        }
        follow_ups
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
        for (side, decisions) in handed_in {
            for staff_command in &decisions.commands {
                let answer = self.take_in_command(
                    day,
                    *side,
                    Command {
                        kind: staff_command.kind.clone(),
                        issuer: Issuer::Staff,
                    },
                );
                // The command's reason, after the command and its
                // follow-ups, citing the command's event.
                self.events.record(
                    day,
                    None,
                    EventKind::StaffReason {
                        side: *side,
                        order: None,
                        reason: Box::new(staff_command.reason.clone()),
                    },
                    vec![Cause::Event(answer)],
                    None,
                );
            }
            // A decision to give no order is a reason with no order and no
            // causes (decision 0107).
            for reason in &decisions.no_orders {
                self.events.record(
                    day,
                    None,
                    EventKind::StaffReason {
                        side: *side,
                        order: None,
                        reason: Box::new(reason.clone()),
                    },
                    Vec::new(),
                    None,
                );
            }
        }
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
        let forced = if self.staff_order_in_forces_limit(side, &request) {
            None
        } else {
            Some(RejectReason::OutsideForcesLimit)
        };
        let (order, answer, _accepted) =
            self.take_in_order_checked(day, None, side, request, Vec::new(), forced);
        self.events.record(
            day,
            None,
            EventKind::StaffReason {
                side,
                order: Some(order),
                reason: Box::new(reason.clone()),
            },
            vec![Cause::Order(order), Cause::Event(answer)],
            None,
        );
    }

    /// Whether a staff's order is for a unit inside the forces limit of the
    /// intent it serves (decision 0107, "A staff's order"): the intent is in
    /// force with a forces limit, and the unit is a task force the limit
    /// lists or one whose every ship (and at least one) is in the pool, or
    /// the intent's source base forming a force from ships in the pool. An
    /// air group, a land unit or any other base order is in no forces limit.
    fn staff_order_in_forces_limit(&self, side: Side, request: &OrderRequest) -> bool {
        let Some(intent) = request.origin.intent else {
            return false;
        };
        let Some(forces) = self.command[&side]
            .intents
            .get(&intent)
            .filter(|intent| intent.in_force())
            .and_then(|intent| intent.limits.forces.as_ref())
        else {
            return false;
        };
        match &request.kind {
            OrderKind::TaskForce { target, .. } => {
                if forces.task_forces.contains(target) {
                    return true;
                }
                self.truth.task_forces.get(target).is_some_and(|force| {
                    force.side == side
                        && !force.ships.is_empty()
                        && force.ships.iter().all(|ship| forces.ships.contains(ship))
                })
            }
            OrderKind::Base {
                target,
                order: BaseOrder::FormTaskForce { ships },
            } => *target == forces.source && ships.iter().all(|ship| self.in_pool(forces, *ship)),
            // Air groups, land units and any other base order are in no
            // forces limit in step 1.
            _ => false,
        }
    }
}

/// Drafts a plan for `intent` in the side's own numbering and returns its
/// number.
fn draft_plan(state: &mut CommandState, intent: IntentId) -> PlanId {
    let id = PlanId::new(state.next_plan);
    state.next_plan += 1;
    state.plans.insert(
        id,
        Plan {
            id,
            intent,
            state: PlanState::Draft,
            baseline: None,
            amendments: Vec::new(),
            runs: BTreeMap::new(),
        },
    );
    id
}

/// Adds the next numbered amendment to an approved plan and returns its
/// number.
fn amend(plan: &mut Plan, day: u32, change: AmendmentChange) -> u32 {
    let number = u32::try_from(plan.amendments.len()).expect("few amendments") + 1;
    plan.amendments.push(Amendment {
        number,
        day,
        change,
    });
    number
}
