//! The day from which a side may use a unit (decision 0140): the one test
//! intake reads. The day itself, the later of a unit's own and those of
//! what carries it, is [`TruthState::available_from`](crate::state::TruthState::available_from).

use crate::ids::EntityRef;
use crate::orders::{BaseOrder, OrderKind, RejectReason, TaskForceOrder};
use crate::state::Side;
use crate::world::World;

impl World {
    /// The refusal of an order of `side` for `day` that is to a unit, or
    /// names a unit, before the day from which the side may use it
    /// (decision 0140, 2), or `None`. The units it reads: the order's own
    /// unit; the ships a base forms a force from; the land unit a task force
    /// is to load. An order to a unit that is not the side's is left to the
    /// checks that refuse it as not the side's, so that no refusal tells a
    /// side an enemy's day.
    pub(crate) fn not_yet_available(
        &self,
        side: Side,
        kind: &OrderKind,
        day: u32,
    ) -> Option<RejectReason> {
        named_units(kind)
            .into_iter()
            .filter(|unit| self.own(side, *unit))
            .find_map(|unit| {
                let from = self.truth.available_from(unit)?;
                (day < from).then_some(RejectReason::NotYetAvailable { unit, day: from })
            })
    }

    /// The refusal of an order of `side` that is to a unit of its own held
    /// apart until its day, or names one (decision 0195, 8 and 15), or
    /// `None`: the same named units as [`World::not_yet_available`]. Read
    /// before it. The enemy's held unit is not here: it is in no state, and
    /// the checks that refuse an order to a unit that is not the side's give
    /// it, with nothing of a day.
    pub(crate) fn not_yet_entered(&self, side: Side, kind: &OrderKind) -> Option<RejectReason> {
        if self.truth.entering.is_empty() {
            return None;
        }
        named_units(kind).into_iter().find_map(|unit| {
            let entry = self.truth.entry_of(unit)?;
            (entry.side == side).then_some(RejectReason::NotYetEntered {
                unit,
                day: entry.day,
                waiting: entry.waiting,
            })
        })
    }

    fn own(&self, side: Side, unit: EntityRef) -> bool {
        let truth = &self.truth;
        match unit {
            EntityRef::Ship(id) => truth.ships.get(&id).is_some_and(|x| x.side == side),
            EntityRef::TaskForce(id) => truth.task_forces.get(&id).is_some_and(|x| x.side == side),
            EntityRef::AirGroup(id) => truth.air_groups.get(&id).is_some_and(|x| x.side == side),
            EntityRef::LandUnit(id) => truth.land_units.get(&id).is_some_and(|x| x.side == side),
            _ => false,
        }
    }
}

/// The units an order names for the day checks: its target; the ships a base
/// forms a force from; the land unit a task force is to load.
fn named_units(kind: &OrderKind) -> Vec<EntityRef> {
    let mut named = vec![kind.target()];
    match kind {
        OrderKind::Base {
            order: BaseOrder::FormTaskForce { ships },
            ..
        } => named.extend(ships.iter().map(|ship| EntityRef::Ship(*ship))),
        OrderKind::TaskForce {
            order: TaskForceOrder::LoadUnit { unit, .. },
            ..
        } => named.push(EntityRef::LandUnit(*unit)),
        _ => {}
    }
    named
}
