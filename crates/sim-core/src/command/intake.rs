//! Taking commands in (decision 0107, items 1, 2 and 4): the commander's at
//! the first step of a day, the staff's after the staff is asked. Each is
//! checked against the side's command state and its own units, and accepted
//! or refused with a reason, every change an event with its causes.
//!
//! The order of the staff's step of a day (decision 0107, step 4): the staff
//! is asked (`ask_staffs`; a hand-in given back by a replay comes first and
//! the notices' bases of the one asked are shifted behind it); the figure of
//! each set interrupt's base is read on that view
//! (`read_interrupt_figures`); the staff's commands are taken in each with its
//! reason, then each of its notices is recorded citing the reason it rests on
//! (`intake_staff_commands`); the interrupts are tested, side by side
//! (`test_interrupts`, in `interrupt.rs`); and only then are the day's orders
//! taken in (`intake_orders`).

use std::collections::BTreeMap;

use crate::command::{
    Addressee, Amendment, AmendmentChange, Baseline, Command, CommandKind, CommandRefusal,
    CommandState, Commitment, ForcesLimit, Hold, HoldMaker, HoldTarget, Intent, IntentKind,
    IntentLimits, InterruptCondition, InterruptRef, InterruptSetting, InterruptStatus, Leave,
    Limit, Maker, NoticeBasis, Plan, PlanId, PlanOutcome, PlanState, Reason, Run, SetInterrupt,
    StaffDecisions, StaffNotice, Staffs, StandingSubject,
};
use crate::events::{Cause, EventKind};
use crate::fixed::Fixed;
use crate::ids::{AirGroupId, BaseId, EventId, IntentId, ShipId};
use crate::orders::{AirGroupOrder, BaseOrder, Issuer, OrderKind, OrderRequest, RejectReason};
use crate::state::{AirGroupLocation, ShipLocation, Side};
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
    pub(crate) fn in_pool(&self, forces: &ForcesLimit, ship: ShipId) -> bool {
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

    /// Whether `group` is one of `side`'s air groups that `intent` may use
    /// (decisions 0115, 2, and 0117, 7 and 8): named in every forces limit on
    /// its chain (the narrowest binds, the architect's ruling after the
    /// build); or, with no forces limit on it or above it, based at the base
    /// it is addressed to. An intent addressed to the area may use no group
    /// without a forces limit. An intent addressed to a task force (decisions
    /// 0124, 6, and 0125, 1) may use the groups aboard the force's ships and
    /// nothing else: a group aboard, where every forces limit on its chain
    /// names the group or holds the force.
    pub(crate) fn may_use_group(&self, side: Side, intent: IntentId, group: AirGroupId) -> bool {
        let state = &self.command[&side];
        let Some(found) = state.intents.get(&intent) else {
            return false;
        };
        let own = self.truth.air_groups.get(&group).filter(|g| g.side == side);
        let Some(own) = own else {
            return false;
        };
        if let Addressee::TaskForce(force) = found.addressee {
            let aboard = match own.location {
                AirGroupLocation::Ship(ship) => self
                    .truth
                    .task_forces
                    .get(&force)
                    .is_some_and(|found| found.ships.contains(&ship)),
                _ => false,
            };
            return aboard
                && state.forces_on_chain(intent).iter().all(|forces| {
                    forces.air_groups.contains(&group)
                        || self.force_in_forces_limit(side, forces, force)
                });
        }
        match state.groups_on_chain(intent) {
            Some(allowed) => allowed.contains(&group),
            None => match found.addressee {
                Addressee::Area => false,
                Addressee::Base(base) => own.location == AirGroupLocation::Base(base),
                Addressee::AirGroup(addressed) => addressed == group,
                Addressee::TaskForce(_) => false,
            },
        }
    }

    /// Whether `ship` is in the pool of every forces limit in `limits`.
    fn in_every_pool(&self, limits: &[&ForcesLimit], ship: ShipId) -> bool {
        limits.iter().all(|forces| self.in_pool(forces, ship))
    }

    /// Whether a forces limit set on `intent`, which has a parent, names
    /// only what the parent may use, read up the tree (decision 0117, 7):
    /// each ship in the pool of every forces limit on the parent's chain,
    /// each task force inside every one, each group one the parent may use,
    /// and a source only where it is the source of every one. With no forces
    /// limit above, the parent may use only what its addressee commands: a
    /// base's air groups, and no ship.
    fn forces_narrow(&self, side: Side, parent: IntentId, limit: &ForcesLimit) -> bool {
        let state = &self.command[&side];
        let groups_ok = limit
            .air_groups
            .iter()
            .all(|group| self.may_use_group(side, parent, *group));
        let above = state.forces_on_chain(parent);
        if above.is_empty() {
            return groups_ok
                && limit.ships.is_empty()
                && limit.task_forces.is_empty()
                && limit.source.is_none();
        }
        groups_ok
            && limit
                .ships
                .iter()
                .all(|ship| self.in_every_pool(&above, *ship))
            && limit.task_forces.iter().all(|force| {
                above
                    .iter()
                    .all(|forces| self.force_in_forces_limit(side, forces, *force))
            })
            && limit
                .source
                .is_none_or(|source| above.iter().all(|forces| forces.source == Some(source)))
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
            let forces = state
                .intents
                .get(&plan.intent)
                .and_then(|intent| intent.limits.forces.as_ref())
                .ok_or(CommandRefusal::NoForcesLimit)?;
            if forces.source.is_none() {
                return Err(CommandRefusal::NoSource);
            }
            Ok(forces)
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
                // A staff-made intent is the staff's (decision 0117, 5).
                if state
                    .intents
                    .get(intent)
                    .is_some_and(|existing| existing.maker == Maker::Staff)
                {
                    return Err(CommandRefusal::MadeByStaff(*intent));
                }
                // Its figures may change and not its kind (the architect's
                // ruling after the build of decision 0117).
                if state
                    .intents
                    .get(intent)
                    .is_some_and(|existing| existing.kind.code() != kind.code())
                {
                    return Err(CommandRefusal::KindCannotChange(*intent));
                }
                // "Search waters" stands under "deny an area" only (decision
                // 0123, 11).
                if !kind.may_be_root() {
                    return Err(CommandRefusal::NotARoot);
                }
                self.check_intent_kind(side, kind)
            }
            CommandKind::WithdrawIntent { intent } => {
                let found = self.in_force_intent(side, *intent)?;
                if found.maker == Maker::Staff {
                    return Err(CommandRefusal::MadeByStaff(*intent));
                }
                Ok(())
            }
            CommandKind::FormIntent { parent, kind } => {
                let found = self.in_force_intent(side, *parent)?;
                let plan_state = state
                    .plans
                    .get(&found.plan)
                    .map_or(PlanState::Draft, |plan| plan.state);
                if plan_state != PlanState::Released {
                    return Err(CommandRefusal::WrongPlanState(plan_state));
                }
                if !kind.may_stand_under(found.kind) {
                    return Err(CommandRefusal::CannotStandUnder(*parent));
                }
                // A stand task under "hold a place" stands at the base's hex
                // (decision 0124, 7).
                // So does one under "keep supplied", at the destination's
                // (decision 0126, 7).
                if let (
                    IntentKind::StandAt { hex, .. },
                    IntentKind::Hold { base } | IntentKind::KeepSupplied { base, .. },
                ) = (kind, found.kind)
                {
                    if self.truth.bases.get(&base).map(|b| b.hex) != Some(*hex) {
                        return Err(CommandRefusal::CannotStandUnder(*parent));
                    }
                }
                self.check_intent_kind(side, kind)
            }
            CommandKind::SetLimit(limit) => {
                let mut parent = None;
                if let Some(intent) = limit.intent() {
                    let found = self.in_force_intent(side, intent)?;
                    parent = found.parent;
                    if let Limit::Forces { limit, .. } = limit {
                        if matches!(found.kind, IntentKind::KeepSupplied { .. })
                            && limit.source.is_none()
                        {
                            return Err(CommandRefusal::NoSource);
                        }
                    }
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
                        for group in &limit.air_groups {
                            let own = self
                                .truth
                                .air_groups
                                .get(group)
                                .is_some_and(|found| found.side == side);
                            if !own {
                                return Err(CommandRefusal::NotYourGroup(*group));
                            }
                        }
                        if let Some(source) = limit.source {
                            if !self.own_base(side, source) {
                                return Err(CommandRefusal::NotYourBase(source));
                            }
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
                // On an intent with a parent, a limit must narrow (decision
                // 0117, 7): forces only what the parent may use, a risk no
                // bolder than the parent's, an area inside the parent's.
                if let Some(parent) = parent {
                    let narrows = match limit {
                        Limit::Forces { limit, .. } => self.forces_narrow(side, parent, limit),
                        Limit::Risk { risk, .. } => *risk <= state.risk_read_up(parent),
                        Limit::Area { hexes, .. } => state
                            .area_read_up(parent)
                            .is_none_or(|above| hexes.is_subset(&above)),
                        Limit::Priority { .. }
                        | Limit::Protected { .. }
                        | Limit::Reserve { .. } => true,
                    };
                    if !narrows {
                        return Err(CommandRefusal::WiderThanParent(parent));
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
                    if let Some(target) = self.hold_on_ship_of(side, *ship) {
                        return Err(CommandRefusal::Held(target));
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
                    if let Some(target) = self.hold_on_ship_of(side, *ship) {
                        return Err(CommandRefusal::Held(target));
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
            CommandKind::Hold { target } => {
                match target {
                    HoldTarget::Ship(ship) => {
                        if !self.own_ship_afloat(side, *ship) {
                            return Err(CommandRefusal::NotYourShip(*ship));
                        }
                    }
                    HoldTarget::TaskForce(force) => {
                        let own = self
                            .truth
                            .task_forces
                            .get(force)
                            .is_some_and(|found| found.side == side);
                        if !own {
                            return Err(CommandRefusal::NotYourForce(*force));
                        }
                    }
                    HoldTarget::Setting { group, .. } | HoldTarget::AirGroup(group) => {
                        let own = self
                            .truth
                            .air_groups
                            .get(group)
                            .is_some_and(|found| found.side == side);
                        if !own {
                            return Err(CommandRefusal::NotYourGroup(*group));
                        }
                    }
                }
                if state.holds.contains_key(target) {
                    return Err(CommandRefusal::AlreadyHeld(*target));
                }
                Ok(())
            }
            CommandKind::Release { target } => {
                if !state.holds.contains_key(target) {
                    return Err(CommandRefusal::NotHeld(*target));
                }
                Ok(())
            }
            CommandKind::EndRun { plan, run } => {
                let found = approved_plan(*plan)?;
                let found = found.runs.get(run).ok_or(CommandRefusal::NoSuchRun(*run))?;
                if found.ended.is_some() {
                    return Err(CommandRefusal::RunEnded(*run));
                }
                Ok(())
            }
            CommandKind::SetInterrupt(setting) => match setting {
                InterruptSetting::Set {
                    interrupt,
                    condition,
                } => {
                    if interrupt.raw() == 0 {
                        return Err(CommandRefusal::InterruptNumberNotUsable(*interrupt));
                    }
                    let InterruptCondition::SupplyBelow { base, days } = condition;
                    if !self.own_base(side, *base) {
                        return Err(CommandRefusal::NotYourBase(*base));
                    }
                    if *days == 0 {
                        return Err(CommandRefusal::NoDays);
                    }
                    Ok(())
                }
                InterruptSetting::Standing { .. } => Ok(()),
            },
            CommandKind::Acknowledge { interrupt } => {
                let status = match interrupt {
                    InterruptRef::Set(id) => {
                        state
                            .interrupts
                            .get(id)
                            .ok_or(CommandRefusal::NoSuchInterrupt(*id))?
                            .status
                    }
                    InterruptRef::Standing { kind, subject } => {
                        // A refusal never says that an enemy unit exists: a
                        // ship that is not the side's is refused as any
                        // other command refuses it, whether or not it exists.
                        match subject {
                            StandingSubject::Plan(plan) => {
                                if !state.plans.contains_key(plan) {
                                    return Err(CommandRefusal::NoSuchPlan(*plan));
                                }
                            }
                            StandingSubject::Ship(ship) => {
                                let own = self
                                    .truth
                                    .ships
                                    .get(ship)
                                    .is_some_and(|found| found.side == side);
                                if !own {
                                    return Err(CommandRefusal::NotYourShip(*ship));
                                }
                            }
                        }
                        match state.standing.get(&(*kind, *subject)) {
                            Some(status) => *status,
                            None => return Err(CommandRefusal::NotFired(*interrupt)),
                        }
                    }
                };
                if status.last_fired.is_none() {
                    return Err(CommandRefusal::NotFired(*interrupt));
                }
                if status.acknowledged {
                    return Err(CommandRefusal::AlreadyAcknowledged(*interrupt));
                }
                Ok(())
            }
        }
    }

    /// Checks the kind of an intent set or formed: its own base is the
    /// side's, its target (decision 0120, 5 and 9) is a base on the side's
    /// map that is not the side's, the hex of its waters (decision 0123, 1)
    /// is on the map, and a supply intent's days are at least 1.
    fn check_intent_kind(&self, side: Side, kind: &IntentKind) -> Result<(), CommandRefusal> {
        if let Some(base) = kind.own_base() {
            if !self.own_base(side, base) {
                return Err(CommandRefusal::NotYourBase(base));
            }
        }
        if let Some(target) = kind.target() {
            let enemy = self
                .truth
                .bases
                .get(&target)
                .is_some_and(|found| found.side != side && found.on_map);
            if !enemy {
                return Err(CommandRefusal::NotAnEnemyBase(target));
            }
        }
        // The waters of a "deny an area" intent are about a hex on the map
        // (decision 0123, 1).
        if let Some((hex, _)) = kind.waters() {
            if !self.map.contains(hex) {
                return Err(CommandRefusal::NotOnMap(hex));
            }
        }
        // A task of a task force names a force of the side and a hex on the
        // map (decisions 0124, 7, and 0125, 1).
        if let Some(force) = kind.force() {
            let own = self
                .truth
                .task_forces
                .get(&force)
                .is_some_and(|found| found.side == side);
            if !own {
                return Err(CommandRefusal::NotYourForce(force));
            }
        }
        if let Some(hex) = kind.station() {
            if !self.map.contains(hex) {
                return Err(CommandRefusal::NotOnMap(hex));
            }
        }
        if let IntentKind::KeepSupplied {
            floor_days,
            interval_days,
            ..
        } = kind
        {
            if *floor_days < 1 || *interval_days == Some(0) {
                return Err(CommandRefusal::NoDays);
            }
        }
        Ok(())
    }

    /// The hold that keeps `ship` from the staff (ruling 18), reading the
    /// ship's force from the truth.
    pub(crate) fn hold_on_ship_of(&self, side: Side, ship: ShipId) -> Option<HoldTarget> {
        let state = &self.command[&side];
        if state.holds.is_empty() {
            return None;
        }
        let force = self
            .truth
            .ships
            .get(&ship)
            .and_then(|found| match found.location {
                ShipLocation::TaskForce(force) => self
                    .truth
                    .task_forces
                    .get(&force)
                    .map(|found| (force, &found.ships)),
                _ => None,
            });
        state.hold_on_ship(ship, force)
    }

    /// The hold that keeps task force `force` from the staff, if any.
    fn hold_on_force_of(&self, side: Side, force: crate::ids::TaskForceId) -> Option<HoldTarget> {
        let state = &self.command[&side];
        if state.holds.is_empty() {
            return None;
        }
        let ships = &self.truth.task_forces.get(&force)?.ships;
        state.hold_on_force(force, ships)
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
                                parent: None,
                                maker: Maker::Commander,
                                addressee: kind.addressee(),
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
                        let found = state.intents.get_mut(intent).expect("intent exists");
                        found.kind = *kind;
                        found.addressee = kind.addressee();
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
                // Everything beneath it is withdrawn the same day, each with
                // its own event, caused by this command's (decision 0117, 6).
                for below in state.descendants_in_force(*intent) {
                    let found = state.intents.get_mut(&below).expect("intent exists");
                    found.withdrawn_day = Some(day);
                    let parent = found.parent.unwrap_or(*intent);
                    follow_ups.push(EventKind::IntentWithdrawn {
                        side,
                        intent: below,
                        parent,
                    });
                }
                let found = state.intents.get_mut(intent).expect("intent exists");
                found.withdrawn_day = Some(day);
                let plan_id = found.plan;
                let is_root = found.parent.is_none();
                let plan = state.plans.get_mut(&plan_id).expect("plan exists");
                if is_root && !plan.state.ended() {
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
                        Limit::Risk { risk, .. } => intent.limits.risk = Some(*risk),
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
                    .and_then(|forces| forces.source)
                    .expect("the forces limit and its source were checked");
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
                        ended: None,
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
            CommandKind::Hold { target } => {
                let state = self.command.get_mut(&side).expect("side has command state");
                state.holds.insert(
                    *target,
                    Hold {
                        day,
                        by: HoldMaker::Command,
                    },
                );
            }
            CommandKind::Release { target } => {
                let state = self.command.get_mut(&side).expect("side has command state");
                state.holds.remove(target);
            }
            CommandKind::EndRun { plan, run } => {
                let state = self.command.get_mut(&side).expect("side has command state");
                let found = state.plans.get_mut(plan).expect("plan exists");
                found.runs.get_mut(run).expect("run exists").ended = Some(day);
            }
            CommandKind::SetInterrupt(setting) => {
                let state = self.command.get_mut(&side).expect("side has command state");
                match setting {
                    InterruptSetting::Set {
                        interrupt,
                        condition,
                    } => {
                        state.interrupts.insert(
                            *interrupt,
                            SetInterrupt {
                                id: *interrupt,
                                condition: *condition,
                                set_day: day,
                                status: InterruptStatus::default(),
                            },
                        );
                    }
                    InterruptSetting::Standing { kind, on } => {
                        if *on {
                            state.standing_off.remove(kind);
                        } else {
                            state.standing_off.insert(*kind);
                        }
                    }
                }
            }
            CommandKind::Acknowledge { interrupt } => {
                let state = self.command.get_mut(&side).expect("side has command state");
                let status = match interrupt {
                    InterruptRef::Set(id) => {
                        &mut state
                            .interrupts
                            .get_mut(id)
                            .expect("interrupt exists")
                            .status
                    }
                    InterruptRef::Standing { kind, subject } => state
                        .standing
                        .get_mut(&(*kind, *subject))
                        .expect("standing record exists"),
                };
                status.acknowledged = true;
            }
            CommandKind::FormIntent { parent, kind } => {
                let state = self.command.get_mut(&side).expect("side has command state");
                let id = state.next_intent_number();
                let plan = state.intents[parent].plan;
                state.intents.insert(
                    id,
                    Intent {
                        id,
                        kind: *kind,
                        parent: Some(*parent),
                        maker: Maker::Staff,
                        addressee: kind.addressee(),
                        limits: IntentLimits::default(),
                        set_day: day,
                        plan,
                        withdrawn_day: None,
                    },
                );
                follow_ups.push(EventKind::IntentFormed {
                    side,
                    intent: id,
                    parent: *parent,
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
                // After one given back, its notices' bases shifted to still
                // point at their own reasons.
                handed_in.entry(side).or_default().append(decisions);
            }
        }
        handed_in
    }

    /// Step 5 of the day, its first part: takes in each side's staff
    /// commands in the order handed in, and records each decision to give
    /// no order with its reason; then records each notice the staff handed
    /// in, in the order handed in, citing the `StaffReason` event of the
    /// reason it rests on (decision 0107, ruling 34). A basis that points at
    /// nothing is recorded with no cause. The engine never reads a reason's
    /// words. The staff's orders are taken in with the commander's, by
    /// `intake_orders`. Returns each side's notices with the events that
    /// recorded them, for the test of the standing interrupts.
    pub(crate) fn intake_staff_commands(
        &mut self,
        day: u32,
        handed_in: &BTreeMap<Side, StaffDecisions>,
    ) -> BTreeMap<Side, Vec<(StaffNotice, EventId)>> {
        let mut recorded = BTreeMap::new();
        for (side, decisions) in handed_in {
            let mut command_reasons = Vec::new();
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
                command_reasons.push(self.events.record(
                    day,
                    None,
                    EventKind::StaffReason {
                        side: *side,
                        order: None,
                        command: Some(Box::new(staff_command.kind.clone())),
                        reason: Box::new(staff_command.reason.clone()),
                    },
                    vec![Cause::Event(answer)],
                    None,
                ));
            }
            // A decision to give no order is a reason with no order and no
            // causes (decision 0107).
            let mut no_order_reasons = Vec::new();
            for reason in &decisions.no_orders {
                no_order_reasons.push(self.events.record(
                    day,
                    None,
                    EventKind::StaffReason {
                        side: *side,
                        order: None,
                        command: None,
                        reason: Box::new(reason.clone()),
                    },
                    Vec::new(),
                    None,
                ));
            }
            let mut notices = Vec::new();
            for notice in &decisions.notices {
                let reason = match notice.rests_on {
                    NoticeBasis::Command(i) => command_reasons.get(i),
                    NoticeBasis::NoOrder(i) => no_order_reasons.get(i),
                };
                let event = self.events.record(
                    day,
                    None,
                    EventKind::StaffNotice {
                        side: *side,
                        kind: notice.kind,
                        plan: notice.plan,
                        run: notice.run,
                    },
                    reason.map(|id| Cause::Event(*id)).into_iter().collect(),
                    None,
                );
                notices.push((*notice, event));
            }
            if !notices.is_empty() {
                recorded.insert(*side, notices);
            }
        }
        recorded
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
        // The guard of a hold comes first (decision 0107, rulings 18 and 30).
        let forced = if let Some(target) = self.held_order_target(side, &request.kind) {
            Some(RejectReason::Held(target))
        } else if self.staff_order_in_forces_limit(side, &request) {
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
                command: None,
                reason: Box::new(reason.clone()),
            },
            vec![Cause::Order(order), Cause::Event(answer)],
            None,
        );
    }

    /// Whether task force `target` is inside `forces`: listed in it, or a
    /// force of the side with at least one ship, every ship of it in the
    /// limit's ships.
    pub(crate) fn force_in_forces_limit(
        &self,
        side: Side,
        forces: &ForcesLimit,
        target: crate::ids::TaskForceId,
    ) -> bool {
        if forces.task_forces.contains(&target) {
            return true;
        }
        self.truth.task_forces.get(&target).is_some_and(|force| {
            force.side == side
                && !force.ships.is_empty()
                && force.ships.iter().all(|ship| forces.ships.contains(ship))
        })
    }

    /// What the commander holds that an order is for, if anything: a task
    /// force held or with a held ship; a forming order naming a held ship (or
    /// a ship in a held force); a search arc or air mission order to a group
    /// whose search arc is held. A land unit is never held. The guard binds
    /// orders a staff hands in only.
    fn held_order_target(&self, side: Side, kind: &OrderKind) -> Option<HoldTarget> {
        let state = &self.command[&side];
        if state.holds.is_empty() {
            return None;
        }
        match kind {
            OrderKind::TaskForce { target, .. } => self.hold_on_force_of(side, *target),
            OrderKind::Base {
                order: BaseOrder::FormTaskForce { ships },
                ..
            } => ships
                .iter()
                .find_map(|ship| self.hold_on_ship_of(side, *ship)),
            OrderKind::AirGroup { target, order } => {
                let whole = HoldTarget::AirGroup(*target);
                if state.holds.contains_key(&whole) {
                    return Some(whole);
                }
                let arc_or_mission = matches!(
                    order,
                    AirGroupOrder::SetSearchArc { .. } | AirGroupOrder::SetAirMission(_)
                );
                let setting = HoldTarget::Setting {
                    group: *target,
                    setting: crate::command::HeldSetting::SearchArc,
                };
                (arc_or_mission && state.holds.contains_key(&setting)).then_some(setting)
            }
            _ => None,
        }
    }

    /// Whether a staff's order is for a unit inside the forces limit of the
    /// intent it serves (decision 0107, "A staff's order"), read up the tree
    /// (decision 0117, 7): the intent is in force, and the unit is a task
    /// force the limit lists or one whose every ship (and at least one) is in
    /// the pool, or the limit's source base forming a force from ships in the
    /// pool, or an air group the intent may use (decision 0115, 2: named in
    /// the limit, or with no limit at or above the intent, based at the base
    /// it is addressed to). A land unit or any other base order is in no
    /// forces limit.
    fn staff_order_in_forces_limit(&self, side: Side, request: &OrderRequest) -> bool {
        let Some(intent) = request.origin.intent else {
            return false;
        };
        let state = &self.command[&side];
        if !state.intents.get(&intent).is_some_and(Intent::in_force) {
            return false;
        }
        if let OrderKind::AirGroup { target, .. } = &request.kind {
            return self.may_use_group(side, intent, *target);
        }
        // An intent addressed to a task force commands that force (decision
        // 0124, 6): an order to it is inside, whatever the limits above.
        if let (OrderKind::TaskForce { target, .. }, Addressee::TaskForce(force)) =
            (&request.kind, state.intents[&intent].addressee)
        {
            return *target == force;
        }
        // Every forces limit on the chain binds (the architect's ruling
        // after the build). A root has its own alone.
        let limits = state.forces_on_chain(intent);
        if limits.is_empty() {
            return false;
        }
        match &request.kind {
            OrderKind::TaskForce { target, .. } => limits
                .iter()
                .all(|forces| self.force_in_forces_limit(side, forces, *target)),
            OrderKind::Base {
                target,
                order: BaseOrder::FormTaskForce { ships },
            } => {
                limits.iter().all(|forces| Some(*target) == forces.source)
                    && ships.iter().all(|ship| self.in_every_pool(&limits, *ship))
            }
            // Land units and any other base order are in no forces limit.
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
