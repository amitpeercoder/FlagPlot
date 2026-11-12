//! Refuelling in port at the start of a day.
//!
//! The rule is `docs/decisions/0045-refuelling-in-port.md`. There is no
//! randomness and no order: a task force that is stopped at the centre of a
//! hex where a base of its own side lies takes fuel from the base's stock
//! before the day's orders are taken in.

use crate::events::{Cause, EventKind};
use crate::fixed::Fixed;
use crate::ids::{BaseId, ShipId, TaskForceId};
use crate::state::{ShipLocation, TaskForceStatus};
use crate::world::World;

impl World {
    /// Refuels every task force stopped at a base of its own side (decisions
    /// 2 to 6 of 0045). Forces are served in order of their number and the
    /// ships of a force in order of theirs, each up to its class's fuel
    /// capacity or what the base has left.
    ///
    /// A force is stopped when it is not under way and sits exactly at the
    /// centre of a hex: it arrived, never left, or ran dry there. A force
    /// that ran dry between two hexes is adrift and takes nothing. Where two
    /// bases of the side share the hex, the lowest-numbered base that has
    /// fuel serves it.
    pub(crate) fn refuel_in_port(&mut self, day: u32) {
        let forces: Vec<TaskForceId> = self.truth.task_forces.keys().copied().collect();
        for id in forces {
            let force = &self.truth.task_forces[&id];
            if force.status == TaskForceStatus::Underway || force.toward_hex.is_some() {
                continue;
            }
            let ships: Vec<ShipId> = force
                .ships
                .iter()
                .copied()
                .filter(|ship| self.truth.ships[ship].location != ShipLocation::Sunk)
                .collect();
            if ships.is_empty() {
                continue;
            }
            let (side, hex, arrival_event) = (force.side, force.from_hex, force.arrival_event);
            let was_dry = force.status == TaskForceStatus::OutOfFuel;
            let causes: Vec<Cause> = arrival_event.into_iter().map(Cause::Event).collect();
            // A force that ran dry here is stopped again once a ship has
            // taken fuel (decision 6): idle, with the route it could not
            // finish dropped, as a force that sends its stragglers home. The
            // first refuelling event says so.
            let mut restore = was_dry;
            for ship_id in ships {
                let Some(base_id): Option<BaseId> = self
                    .truth
                    .bases
                    .values()
                    .find(|base| base.side == side && base.hex == hex && base.fuel_t > Fixed::ZERO)
                    .map(|base| base.id)
                else {
                    break;
                };
                let ship = &self.truth.ships[&ship_id];
                let capacity = self.ref_data.ship_classes[&ship.class].fuel_capacity_t;
                let lacks = capacity - ship.fuel_t;
                let base_fuel = self.truth.bases[&base_id].fuel_t;
                let taken = if lacks < base_fuel { lacks } else { base_fuel };
                if taken <= Fixed::ZERO {
                    continue;
                }
                let ship = self.truth.ships.get_mut(&ship_id).expect("ship exists");
                ship.fuel_t += taken;
                let aboard = ship.fuel_t;
                let base = self.truth.bases.get_mut(&base_id).expect("base exists");
                base.fuel_t -= taken;
                let left = base.fuel_t;
                if restore {
                    let force = self
                        .truth
                        .task_forces
                        .get_mut(&id)
                        .expect("task force exists");
                    force.status = TaskForceStatus::Idle;
                    force.route = crate::state::Route::default();
                    force.route_order = None;
                }
                self.events.record(
                    day,
                    None,
                    EventKind::ShipRefuelled {
                        ship: ship_id,
                        task_force: id,
                        base: base_id,
                        taken_t: taken,
                        fuel_t: aboard,
                        base_fuel_t: left,
                        force_restored: restore,
                    },
                    causes.clone(),
                    None,
                );
                restore = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{Cause, EventKind};
    use crate::hex::Hex;
    use crate::ids::ShipClassId;
    use crate::map::{HexData, Map};
    use crate::orders::{Mission, OrderKind, OrderRequest, Origin, TaskForceOrder};
    use crate::refdata::{PortSize, RefData, ShipClass, ShipKind};
    use crate::state::{Base, Ship, Side, TaskForce, TruthState};
    use crate::world::SimConfig;

    /// A task force for `world`: its number, side, hex, and its ships as
    /// (ship number, fuel aboard in tons).
    type Force<'a> = (u32, Side, Hex, &'a [(u32, i64)]);

    /// A base for `world`: its number, side, hex and fuel in tons.
    type Station = (u32, Side, Hex, i64);

    fn tf(id: u32) -> TaskForceId {
        TaskForceId::new(id)
    }

    fn ship(id: u32) -> ShipId {
        ShipId::new(id)
    }

    fn tons(n: i64) -> Fixed {
        Fixed::from_int(n)
    }

    /// Open sea from (-1,-1) to (16,1). One ship class: 2,000 t of fuel, 8,000
    /// nm at `cruise_kn`, so it burns 0.25 t a nm.
    fn world(cruise_kn: i64, forces: &[Force], bases: &[Station]) -> World {
        let mut map = Map::new();
        for q in -1..=16 {
            for r in -1..=1 {
                map.insert(Hex::new(q, r), HexData::open_sea());
            }
        }
        let class = ShipClass {
            id: ShipClassId::new(1),
            name: "Cruiser".to_string(),
            kind: ShipKind::Cruiser,
            cruise_speed_kn: Fixed::from_int(cruise_kn),
            max_speed_kn: Fixed::from_int(cruise_kn + 4),
            fuel_capacity_t: Fixed::from_int(2000),
            endurance_nm: Fixed::from_int(8000),
            cargo_capacity_t: Fixed::ZERO,
            main_guns: 0,
            gun_rounds_per_min: Fixed::ZERO,
            gun_rounds_per_gun: 0,
            gun_damage: Fixed::ZERO,
            torpedo_tubes: 0,
            torpedo_damage: Fixed::ZERO,
            toughness: Fixed::from_int(100),
            radar: false,
            aircraft_capacity: 0,
            anti_aircraft: 0,
        };
        let mut ref_data = RefData::default();
        ref_data.ship_classes.insert(class.id, class);
        ref_data.port_sizes.insert(
            0,
            PortSize {
                berths: 0,
                berth_rate_t_per_h: Fixed::ZERO,
                lighter_rate_t_per_h: Fixed::from_int(20),
            },
        );

        let mut truth = TruthState::default();
        for (id, side, hex, ships) in forces {
            let mut force = TaskForce::at(
                tf(*id),
                format!("Force {id}"),
                *side,
                *hex,
                Mission::Transport,
            );
            for (ship_id, fuel) in *ships {
                force.ships.insert(ship(*ship_id));
                truth.ships.insert(
                    ship(*ship_id),
                    Ship {
                        fuel_t: tons(*fuel),
                        ..Ship::new(
                            ship(*ship_id),
                            format!("Ship {ship_id}"),
                            ShipClassId::new(1),
                            *side,
                            ShipLocation::TaskForce(tf(*id)),
                        )
                    },
                );
            }
            truth.task_forces.insert(tf(*id), force);
        }
        for (id, side, hex, fuel) in bases {
            truth.bases.insert(
                BaseId::new(*id),
                Base {
                    fuel_t: tons(*fuel),
                    ..Base::new(BaseId::new(*id), format!("Base {id}"), *side, *hex)
                },
            );
        }
        World::new(7, ref_data, map, truth).expect("valid setup")
    }

    fn route(target: u32, waypoints: &[Hex]) -> OrderRequest {
        OrderRequest {
            kind: OrderKind::TaskForce {
                target: tf(target),
                order: TaskForceOrder::SetRoute(waypoints.to_vec()),
            },
            origin: Origin::PLAYER,
        }
    }

    /// Every refuelling event so far as (ship, base, taken, aboard, left,
    /// force restored).
    fn refuelled(world: &World) -> Vec<(u32, u32, i64, i64, i64, bool)> {
        world
            .events()
            .iter()
            .filter_map(|event| match &event.kind {
                EventKind::ShipRefuelled {
                    ship,
                    base,
                    taken_t,
                    fuel_t,
                    base_fuel_t,
                    force_restored,
                    ..
                } => Some((
                    ship.raw(),
                    base.raw(),
                    taken_t.trunc(),
                    fuel_t.trunc(),
                    base_fuel_t.trunc(),
                    *force_restored,
                )),
                _ => None,
            })
            .collect()
    }

    fn fuel(world: &World, id: u32) -> Fixed {
        world.umpire().ships[&ship(id)].fuel_t
    }

    fn base_fuel(world: &World, id: u32) -> Fixed {
        world.umpire().bases[&BaseId::new(id)].fuel_t
    }

    const HERE: Hex = Hex::new(0, 0);

    #[test]
    fn forces_then_ships_are_served_in_order_of_number_when_the_stock_runs_short() {
        // Force 1 holds ships 5 and 6, force 2 holds ship 1: the lower force
        // is served first whatever its ships' numbers. Each lacks 100 t and
        // the base has 150 t: ship 5 takes 100, ship 6 takes the last 50,
        // ship 1 gets nothing.
        let mut world = world(
            16,
            &[
                (1, Side::Allied, HERE, &[(5, 1900), (6, 1900)]),
                (2, Side::Allied, HERE, &[(1, 1900)]),
            ],
            &[(1, Side::Allied, HERE, 150)],
        );
        world.resolve_day().expect("day resolves");
        assert_eq!(
            refuelled(&world),
            vec![(5, 1, 100, 2000, 50, false), (6, 1, 50, 1950, 0, false),]
        );
        assert_eq!(fuel(&world, 1), tons(1900));
        assert_eq!(base_fuel(&world, 1), Fixed::ZERO);
        // Nothing orders it and no arrival is on record: no causes.
        assert!(world.events().iter().all(|event| event.causes.is_empty()));
    }

    #[test]
    fn an_arrival_is_the_cause_of_the_refuelling() {
        let mut world = world(
            16,
            &[(1, Side::Allied, HERE, &[(1, 1900)])],
            &[(1, Side::Allied, Hex::new(2, 0), 500)],
        );
        world.submit_orders(Side::Allied, vec![route(1, &[Hex::new(2, 0)])]);
        world.resolve_day().expect("day 1 resolves");
        assert!(refuelled(&world).is_empty());
        let arrival = world
            .events()
            .iter()
            .find(|event| matches!(event.kind, EventKind::TaskForceArrived { .. }))
            .expect("the force arrived")
            .id;
        world.resolve_day().expect("day 2 resolves");
        let events: Vec<_> = world
            .events()
            .iter()
            .filter(|event| matches!(event.kind, EventKind::ShipRefuelled { .. }))
            .collect();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].causes, vec![Cause::Event(arrival)]);
        assert_eq!(events[0].pulse, None);
        assert_eq!(events[0].day, 2);
        // The ship burned 80 nm x 0.25 t = 20 t and took the 100 t it lacked.
        assert_eq!(fuel(&world, 1), tons(2000));
    }

    #[test]
    fn a_force_exactly_at_a_centre_but_under_way_does_not_refuel() {
        // At 20 kn a day is 480 nm, twelve hexes. A route to (12,0) and on to
        // (14,0) leaves the force exactly at the centre of (12,0) at the end of
        // day 1, with a waypoint still to go: under way, at a centre, in the
        // hex of a base with fuel.
        let mut world = world(
            20,
            &[(1, Side::Allied, HERE, &[(1, 1500)])],
            &[(1, Side::Allied, Hex::new(12, 0), 500)],
        );
        world.submit_orders(
            Side::Allied,
            vec![route(1, &[Hex::new(12, 0), Hex::new(14, 0)])],
        );
        world.resolve_day().expect("day 1 resolves");
        {
            let force = &world.umpire().task_forces[&tf(1)];
            assert_eq!(force.status, TaskForceStatus::Underway);
            assert_eq!(force.toward_hex, None);
            assert_eq!(force.position(), Hex::new(12, 0));
        }
        world.resolve_day().expect("day 2 resolves");
        // Nothing was refuelled at the start of day 2, nor on arrival day 3.
        assert!(refuelled(&world).is_empty());
        assert_eq!(base_fuel(&world, 1), tons(500));
    }

    #[test]
    fn a_force_at_a_base_of_the_other_side_takes_nothing() {
        let mut world = world(
            16,
            &[(1, Side::Allied, HERE, &[(1, 1000)])],
            &[(1, Side::Japanese, HERE, 500)],
        );
        world.resolve_day().expect("day resolves");
        assert!(refuelled(&world).is_empty());
        assert_eq!(fuel(&world, 1), tons(1000));
        assert_eq!(base_fuel(&world, 1), tons(500));
    }

    #[test]
    fn a_force_in_a_hex_with_no_base_takes_nothing() {
        let mut world = world(
            16,
            &[(1, Side::Allied, HERE, &[(1, 1000)])],
            &[(1, Side::Allied, Hex::new(1, 0), 500)],
        );
        world.resolve_day().expect("day resolves");
        assert!(refuelled(&world).is_empty());
        assert_eq!(fuel(&world, 1), tons(1000));
        assert_eq!(base_fuel(&world, 1), tons(500));
    }

    #[test]
    fn a_sunk_ship_takes_nothing_and_a_force_with_none_afloat_is_skipped() {
        let mut world = world(
            16,
            &[
                (1, Side::Allied, HERE, &[(1, 1000), (2, 1000)]),
                (2, Side::Allied, HERE, &[(3, 1000)]),
            ],
            &[(1, Side::Allied, HERE, 5000)],
        );
        // Ship 1 is sunk but still listed by its force; force 2's only ship
        // is sunk and listed too.
        for id in [1, 3] {
            let sunk = world.truth.ships.get_mut(&ship(id)).expect("ship");
            sunk.location = ShipLocation::Sunk;
            sunk.flotation_damage = tons(100);
        }
        // The invariants would refuse a sunk ship that its force still lists,
        // so run only the refuelling step, which must skip it all the same.
        world.refuel_in_port(1);
        assert_eq!(refuelled(&world), vec![(2, 1, 1000, 2000, 4000, false)]);
        assert_eq!(fuel(&world, 1), tons(1000));
        assert_eq!(fuel(&world, 3), tons(1000));
        assert_eq!(base_fuel(&world, 1), tons(4000));
    }

    #[test]
    fn a_force_that_ran_dry_exactly_in_a_base_hex_refuels_and_can_be_ordered_on() {
        // 10 t at 0.25 t a nm is 40 nm: the force runs dry at the centre of
        // (1,0), which holds a base with fuel.
        let mut world = world(
            16,
            &[(1, Side::Allied, HERE, &[(1, 10)])],
            &[(1, Side::Allied, Hex::new(1, 0), 500)],
        );
        world.submit_orders(Side::Allied, vec![route(1, &[Hex::new(5, 0)])]);
        world.resolve_day().expect("day 1 resolves");
        {
            let force = &world.umpire().task_forces[&tf(1)];
            assert_eq!(force.status, TaskForceStatus::OutOfFuel);
            assert_eq!(force.toward_hex, None);
            assert_eq!(force.position(), Hex::new(1, 0));
        }
        assert!(refuelled(&world).is_empty());

        world.submit_orders(Side::Allied, vec![route(1, &[Hex::new(5, 0)])]);
        world.resolve_day().expect("day 2 resolves");
        // The ship had 0 t and lacked 2,000 t: it took the base's 500 t, was
        // restored, took the new order and steamed on.
        assert_eq!(refuelled(&world), vec![(1, 1, 500, 500, 0, true)]);
        let force = &world.umpire().task_forces[&tf(1)];
        assert_eq!(force.position(), Hex::new(5, 0));
        assert_eq!(force.status, TaskForceStatus::Arrived);
        // 160 nm at 0.25 t a nm.
        assert_eq!(fuel(&world, 1), tons(460));
    }

    #[test]
    fn a_force_that_ran_dry_between_hexes_is_adrift_and_takes_nothing() {
        // 5 t is 20 nm: dry half way to (1,0), nearest hex (0,0) with a base.
        let mut world = world(
            16,
            &[(1, Side::Allied, HERE, &[(1, 5)])],
            &[(1, Side::Allied, HERE, 500)],
        );
        world.submit_orders(Side::Allied, vec![route(1, &[Hex::new(5, 0)])]);
        // The base is in the force's own hex: it refuels on day 1 before it
        // sails, so empty the base for the test.
        world
            .truth
            .bases
            .get_mut(&BaseId::new(1))
            .expect("base")
            .fuel_t = Fixed::ZERO;
        world.resolve_day().expect("day 1 resolves");
        assert_eq!(
            world.umpire().task_forces[&tf(1)].status,
            TaskForceStatus::OutOfFuel
        );
        assert!(world.umpire().task_forces[&tf(1)].toward_hex.is_some());
        world
            .truth
            .bases
            .get_mut(&BaseId::new(1))
            .expect("base")
            .fuel_t = tons(500);
        world.resolve_day().expect("day 2 resolves");
        assert!(refuelled(&world).is_empty());
        assert_eq!(fuel(&world, 1), Fixed::ZERO);
    }

    #[test]
    fn where_two_bases_share_a_hex_the_lowest_numbered_with_fuel_serves_each_ship() {
        // Base 1 has 30 t and serves ship 1, which lacks 50 and takes the 30;
        // base 1 is then empty, so base 2 serves ship 2. Then base 1 is empty
        // from the start and base 2 serves ship 1.
        let mut world = world(
            16,
            &[(1, Side::Allied, HERE, &[(1, 1950), (2, 1900)])],
            &[(1, Side::Allied, HERE, 30), (2, Side::Allied, HERE, 100)],
        );
        world.resolve_day().expect("day resolves");
        assert_eq!(
            refuelled(&world),
            vec![(1, 1, 30, 1980, 0, false), (2, 2, 100, 2000, 0, false),]
        );

        let mut world = self::world(
            16,
            &[(1, Side::Allied, HERE, &[(1, 1950)])],
            &[(1, Side::Allied, HERE, 0), (2, Side::Allied, HERE, 100)],
        );
        world.resolve_day().expect("day resolves");
        assert_eq!(refuelled(&world), vec![(1, 2, 50, 2000, 50, false)]);
    }

    #[test]
    fn with_no_fuel_at_any_base_nothing_is_recorded_and_the_hash_is_untouched() {
        let mut empty = world(
            16,
            &[(1, Side::Allied, HERE, &[(1, 1000)])],
            &[(1, Side::Allied, HERE, 0)],
        );
        let before = empty.state_hash();
        let summary = empty.resolve_day().expect("day resolves");
        assert!(refuelled(&empty).is_empty());
        assert_eq!(summary.event_count, 0);
        assert_eq!(fuel(&empty, 1), tons(1000));

        // The same world with a fuel stock hashes differently, and two equal
        // worlds hash alike: the stock is read by the hash and a stock of
        // nothing adds nothing to it.
        let stocked = world(
            16,
            &[(1, Side::Allied, HERE, &[(1, 1000)])],
            &[(1, Side::Allied, HERE, 7)],
        );
        let same = world(
            16,
            &[(1, Side::Allied, HERE, &[(1, 1000)])],
            &[(1, Side::Allied, HERE, 0)],
        );
        assert_ne!(stocked.state_hash(), before);
        assert_eq!(same.state_hash(), before);
    }

    #[test]
    fn recording_breakdowns_changes_neither_the_events_nor_the_hash() {
        let run = |record_breakdowns: bool| {
            let mut world = world(
                16,
                &[(1, Side::Allied, HERE, &[(1, 1000)])],
                &[(1, Side::Allied, HERE, 300)],
            );
            world.set_config(SimConfig { record_breakdowns });
            world.resolve_day().expect("day resolves");
            (
                world.events().iter().cloned().collect::<Vec<_>>(),
                world.day_hashes().to_vec(),
            )
        };
        assert_eq!(run(true), run(false));
    }
}
