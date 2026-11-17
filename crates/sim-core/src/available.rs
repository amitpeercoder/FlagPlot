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
            .into_iter()
            .filter(|unit| self.own(side, *unit))
            .find_map(|unit| {
                let from = self.truth.available_from(unit)?;
                (day < from).then_some(RejectReason::NotYetAvailable { unit, day: from })
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
