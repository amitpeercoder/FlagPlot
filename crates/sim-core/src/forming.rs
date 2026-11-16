//! Forces formed, joined and stood down at a base, and the answer to a load
//! the commander orders below a reserve.
//!
//! The rules are decisions 9 and 12 to 17 of
//! `docs/decisions/0111-three-rules-the-staff-needs.md`. Each is done at
//! intake, when its order is accepted; the caller records the event returned
//! after the acceptance, citing the order. There is no randomness.
//!
//! Reads: the ships in port at a base and their side; a force's ships,
//! place and preparation; the base's stock and reserve. Writes: the forces
//! and which ships are in them, a ship's place, a force's orders and its
//! preparation.

use std::collections::BTreeSet;

use crate::events::EventKind;
use crate::fixed::Fixed;
use crate::ids::{BaseId, ShipId, TaskForceId};
use crate::orders::{Cargo, Mission, TaskForceOrder};
use crate::preparation::joined;
use crate::state::{Route, ShipLocation, TaskForce};
use crate::world::World;

impl World {
    /// Forms `ships`, in port at `base`, into a new task force (decision 12
    /// of 0111). It takes the next number of the side's made series
    /// (decision 0086), stopped at the base with the mission `Transport` (no
    /// rule but bombardment reads a mission), cruise speed, the default
    /// engagement settings and readiness, and the base as its home port. An
    /// air group aboard a ship goes with her: its place is the ship.
    pub(crate) fn form_task_force(&mut self, base: BaseId, ships: &[ShipId]) -> EventKind {
        let (side, hex, name) = {
            let base = &self.truth.bases[&base];
            (base.side, base.hex, base.name.clone())
        };
        let id = self.made_ids.alloc_task_force(side);
        let mut force = TaskForce::at(
            id,
            format!("Formed at {name}"),
            side,
            hex,
            Mission::Transport,
        );
        force.home_port = Some(base);
        let listed: BTreeSet<ShipId> = ships.iter().copied().collect();
        for ship in &listed {
            force.ships.insert(*ship);
            self.truth
                .ships
                .get_mut(ship)
                .expect("the order was validated: the ship exists")
                .location = ShipLocation::TaskForce(id);
        }
        self.truth.task_forces.insert(id, force);
        // Formed from ships in port: its picture is a copy of the area
        // command's (decision 0134, 5).
        self.command_formed(id, crate::pictures::CommandRef::Area);
        EventKind::TaskForceFormed {
            task_force: id,
            side,
            base,
            ships: listed.into_iter().collect(),
        }
    }

    /// Joins `from` to `into`, both stopped at one base of their side
    /// (decision 13 of 0111). `into` keeps its number, orders and settings
    /// and takes `from`'s ships with everything aboard them; a landing in
    /// preparation is joined by decision 12 of 0095. `from`'s orders end and
    /// it ceases to be: it stays in the state with no ship, as a force whose
    /// ships were sunk does, and its number is not used again.
    pub(crate) fn join_task_force(
        &mut self,
        from: TaskForceId,
        into: TaskForceId,
    ) -> Vec<EventKind> {
        let base = self.lowest_own_base_at(from);
        let from_force = self
            .truth
            .task_forces
            .get_mut(&from)
            .expect("the order was validated: the force exists");
        let ships = std::mem::take(&mut from_force.ships);
        let from_preparation = from_force.preparation.take();
        end_orders(from_force);
        let into_force = self
            .truth
            .task_forces
            .get_mut(&into)
            .expect("the order was validated: the force exists");
        into_force.preparation = joined(into_force.preparation.as_ref(), from_preparation.as_ref());
        into_force.ships.extend(ships.iter().copied());
        for ship in &ships {
            self.truth
                .ships
                .get_mut(ship)
                .expect("a ship of the force exists")
                .location = ShipLocation::TaskForce(into);
        }
        vec![EventKind::TaskForceJoined {
            joined: from,
            into,
            base,
            ships: ships.into_iter().collect(),
        }]
    }

    /// Stands `force` down at the base it is stopped at (decision 14 of
    /// 0111): its ships lie in port there in no force, with what they carry,
    /// its orders end and it ceases to be.
    pub(crate) fn stand_down_task_force(&mut self, force: TaskForceId) -> Vec<EventKind> {
        let base = self.lowest_own_base_at(force);
        let record = self
            .truth
            .task_forces
            .get_mut(&force)
            .expect("the order was validated: the force exists");
        let ships = std::mem::take(&mut record.ships);
        record.preparation = None;
        end_orders(record);
        for ship in &ships {
            self.truth
                .ships
                .get_mut(ship)
                .expect("a ship of the force exists")
                .location = ShipLocation::Port(base);
        }
        vec![EventKind::TaskForceStoodDownInPort {
            task_force: force,
            base,
            ships: ships.into_iter().collect(),
        }]
    }

    /// The lowest-numbered base of the force's side in its hex. Only for a
    /// force an order found stopped at a base of its side.
    fn lowest_own_base_at(&self, force: TaskForceId) -> BaseId {
        let force = &self.truth.task_forces[&force];
        self.truth
            .bases
            .values()
            .find(|base| base.side == force.side && base.hex == force.from_hex)
            .map(|base| base.id)
            .expect("the order was validated: a base of the side is here")
    }

    /// The answer to a load the commander orders where the base holds less
    /// above its reserve than the order asks (decision 9 of 0111): the tons
    /// it may take below the reserve, as it stands at intake. What it asks is
    /// the weight less what is aboard, or with no weight the force's room,
    /// and no more than the stock. `None` for an order that is not a load of
    /// supply or fuel, where the side has no base, or with nothing below.
    pub(crate) fn load_below_reserve(
        &self,
        force: TaskForceId,
        order: &TaskForceOrder,
    ) -> Option<EventKind> {
        let (cargo, weight) = match order {
            TaskForceOrder::Load => (Cargo::Supply, None),
            TaskForceOrder::LoadFuel => (Cargo::Fuel, None),
            TaskForceOrder::LoadTo { cargo, weight_t } => (*cargo, Some(*weight_t)),
            TaskForceOrder::LoadUnit {
                cargo: Some(cargo), ..
            } => (*cargo, None),
            _ => return None,
        };
        let record = &self.truth.task_forces[&force];
        let base = self
            .truth
            .bases
            .values()
            .find(|base| base.side == record.side && base.hex == record.position())?;
        let (aboard, room) =
            record
                .ships
                .iter()
                .fold((Fixed::ZERO, Fixed::ZERO), |(aboard, room), id| {
                    let ship = &self.truth.ships[id];
                    let capacity = self.ref_data.ship_classes[&ship.class].cargo_capacity_t;
                    let of_cargo = match cargo {
                        Cargo::Supply => ship.supply_t,
                        Cargo::Fuel => ship.cargo_fuel_t,
                    };
                    (
                        aboard + of_cargo,
                        room + capacity - ship.supply_t - ship.cargo_fuel_t,
                    )
                });
        let asks = match weight {
            Some(weight) => (weight - aboard).max(Fixed::ZERO).min(room),
            None => room,
        };
        let below = asks.min(base.stock_of(cargo)) - base.above_reserve(cargo);
        (below > Fixed::ZERO).then_some(EventKind::LoadBelowReserve {
            task_force: force,
            base: base.id,
            cargo,
            below_t: below,
            reserve_t: base.reserve_of(cargo),
        })
    }
}

/// Ends every order of a force that ceases to be (decisions 13 and 14 of
/// 0111): its route, its cargo order, a halt and a landing begun.
fn end_orders(force: &mut TaskForce) {
    force.route = Route::default();
    force.route_order = None;
    force.cargo_order = None;
    force.halt = None;
    force.landing_at = None;
}
