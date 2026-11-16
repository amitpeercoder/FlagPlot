//! Refuelling in port, in the hours of each pulse.
//!
//! The rule is `docs/decisions/0045-refuelling-in-port.md` as amended by
//! `docs/decisions/0112-refuelling-that-takes-time.md`. There is no
//! randomness and no order: a task force that is stopped at the centre of a
//! hex where a base of its own side lies takes the fuel it lacks from the
//! base's stock in the loading phase of each pulse, in the cargo rule's
//! queue ([`World::handle_cargo`]): the berths it gives out are shared with
//! cargo, a ship does its cargo first and fuels with the hours it has left,
//! and the tons it takes in an hour are the port's fuelling rate, at a berth
//! or by lighter. A rate the port does not give is no limit: the ship takes
//! all it lacks, whatever hours its cargo used. Nothing is done at the start
//! of the day.

use std::collections::BTreeMap;

use crate::events::{Breakdown, Cause, EventKind, Factor};
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{BaseId, EventId, ShipId, TaskForceId};
use crate::orders::Cargo;
use crate::refdata::PortSize;
use crate::state::{ShipLocation, Side, TaskForceStatus};
use crate::time::Pulse;
use crate::world::World;

/// A task force stopped where it may take fuel in this pulse (decision
/// 0112), and what the fuelling reads of it.
pub(crate) struct FuelStop<'a> {
    pub task_force: TaskForceId,
    pub side: Side,
    pub hex: Hex,
    pub port: PortSize,
    /// The hours the force works in this pulse: from the later of the
    /// pulse's start and its stop to the pulse's end.
    pub hours: Fixed,
    /// Every ship the force lists, in ship order. A sunk one takes nothing.
    pub ships: &'a [ShipId],
    /// The force's arrival, or its halt: a cause of its refuelling events.
    pub arrival_event: Option<EventId>,
}

impl World {
    /// The lowest-numbered base of `side` in `hex` that has fuel above its
    /// reserve: it serves a ship, down to that reserve (decision 7 of 0111).
    fn serving_base(&self, side: Side, hex: Hex) -> Option<BaseId> {
        self.truth
            .bases
            .values()
            .find(|base| {
                base.side == side
                    && base.hex == hex
                    && base.above_reserve(Cargo::Fuel) > Fixed::ZERO
            })
            .map(|base| base.id)
    }

    /// Whether `ship` has fuel to take that a berth would speed (decision
    /// 0112): it lacks fuel, a base of the side here holds some above its
    /// reserve, and the port gives a lighter rate. Where the lighter rate is
    /// not given fuelling has no limit and a berth cannot speed it, so fuel
    /// alone does not claim a berth.
    pub(crate) fn has_fuel_to_take(
        &self,
        side: Side,
        hex: Hex,
        port: &PortSize,
        ship: ShipId,
    ) -> bool {
        let record = &self.truth.ships[&ship];
        if record.location == ShipLocation::Sunk || port.fuel_lighter_rate_t_per_h.is_none() {
            return false;
        }
        let capacity = self.ref_data.ship_classes[&record.class].fuel_capacity_t;
        capacity > record.fuel_t && self.serving_base(side, hex).is_some()
    }

    /// Fuels the ships of a force stopped at a base of its own side, for the
    /// hours of this pulse (decision 0112). Ships are served in order of
    /// their number, each up to its class's fuel capacity, what the serving
    /// base has above its reserve, and what its rate gives in the hours its
    /// cargo work left (`cargo_hours`). Where two bases of the side share the
    /// hex, the lowest-numbered base that has fuel above its reserve serves
    /// each ship (decision 7 of 0111).
    ///
    /// A force out of fuel is idle again, with the route it could not finish
    /// dropped, when its first ship has taken fuel (decision 6 of 0045).
    ///
    /// Returns the hours each ship has used in the pulse, its cargo's and
    /// its fuel's, for rearming after them (decision 0122, 5). Fuel taken
    /// with no rate given takes no hours.
    pub(crate) fn take_fuel(
        &mut self,
        day: u32,
        pulse: Pulse,
        stop: &FuelStop,
        berthed: &[ShipId],
        cargo_hours: &BTreeMap<ShipId, Fixed>,
    ) -> BTreeMap<ShipId, Fixed> {
        let mut used: BTreeMap<ShipId, Fixed> = cargo_hours.clone();
        let id = stop.task_force;
        let mut restore = self.truth.task_forces[&id].status == TaskForceStatus::OutOfFuel;
        let causes: Vec<Cause> = stop.arrival_event.into_iter().map(Cause::Event).collect();
        for &ship_id in stop.ships {
            let ship = &self.truth.ships[&ship_id];
            if ship.location == ShipLocation::Sunk {
                continue;
            }
            let capacity = self.ref_data.ship_classes[&ship.class].fuel_capacity_t;
            let lacks = (capacity - ship.fuel_t).max(Fixed::ZERO);
            let serving = self.serving_base(stop.side, stop.hex);
            let Some(base_id) = serving else {
                // No base above its reserve: the ship takes nothing, and if
                // a base here holds fuel the reserve held it back.
                self.held_at_reserve(day, pulse, (ship_id, id, stop.hex), lacks, None, &causes);
                continue;
            };
            let base_above = self.truth.bases[&base_id].above_reserve(Cargo::Fuel);
            let on_berth = berthed.contains(&ship_id);
            let rate = if on_berth {
                stop.port.fuel_berth_rate_t_per_h
            } else {
                stop.port.fuel_lighter_rate_t_per_h
            };
            let cargo_used = cargo_hours.get(&ship_id).copied().unwrap_or(Fixed::ZERO);
            let hours_left = (stop.hours - cargo_used).max(Fixed::ZERO);
            let mut taken = lacks.min(base_above);
            if let Some(rate) = rate {
                taken = taken.min(rate * hours_left);
            }
            if let Some(rate) = rate.filter(|_| taken > Fixed::ZERO) {
                used.insert(ship_id, cargo_used + taken / rate);
            }
            if taken <= Fixed::ZERO {
                self.held_at_reserve(
                    day,
                    pulse,
                    (ship_id, id, stop.hex),
                    lacks,
                    Some(base_id),
                    &causes,
                );
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
            let breakdown = self.config.record_breakdowns.then(|| {
                let mut factors = vec![
                    Factor {
                        name: "hours_worked",
                        value: stop.hours,
                    },
                    Factor {
                        name: "cargo_hours",
                        value: cargo_used,
                    },
                    Factor {
                        name: "hours_left",
                        value: hours_left,
                    },
                    Factor {
                        name: "on_berth",
                        value: if on_berth { Fixed::ONE } else { Fixed::ZERO },
                    },
                ];
                if let Some(rate) = rate {
                    factors.push(Factor {
                        name: "rate_t_per_h",
                        value: rate,
                    });
                }
                factors.push(Factor {
                    name: "lacks_t",
                    value: lacks,
                });
                factors.push(Factor {
                    name: "base_above_reserve_t",
                    value: base_above,
                });
                Breakdown {
                    factors,
                    result: taken,
                }
            });
            let refuelled = self.events.record(
                day,
                Some(pulse),
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
                breakdown,
            );
            restore = false;
            self.held_at_reserve(
                day,
                pulse,
                (ship_id, id, stop.hex),
                lacks - taken,
                Some(base_id),
                &[Cause::Event(refuelled)],
            );
        }
        used
    }

    /// Records that a ship's refuelling stopped at a reserve (decision 7 of
    /// 0111): it is `short` of full, and the base that served it, or with
    /// none any base of the side here, still holds fuel but none above its
    /// reserve. A ship short because its rate or its hours ran out, with fuel
    /// above the reserve left, is not held. It is recorded once (decision
    /// 0112): not again while the latest event of the ship's refuelling is
    /// this same holding, at the same hex and short by the same tons.
    fn held_at_reserve(
        &mut self,
        day: u32,
        pulse: Pulse,
        (ship, task_force, hex): (ShipId, TaskForceId, Hex),
        short: Fixed,
        served_by: Option<BaseId>,
        causes: &[Cause],
    ) {
        if short <= Fixed::ZERO {
            return;
        }
        let side = self.truth.task_forces[&task_force].side;
        let held = self.truth.bases.values().any(|base| {
            served_by.map_or(base.side == side && base.hex == hex, |id| base.id == id)
                && base.fuel_t > Fixed::ZERO
                && base.above_reserve(Cargo::Fuel).is_zero()
        });
        if !held {
            return;
        }
        let latest = self.events.latest_where(|kind| {
            matches!(
                kind,
                EventKind::ShipRefuelled { ship: of, .. }
                    | EventKind::RefuellingHeldAtReserve { ship: of, .. } if *of == ship
            )
        });
        if let Some(EventKind::RefuellingHeldAtReserve {
            hex: before,
            short_t: before_short,
            ..
        }) = latest
            .and_then(|id| self.events.get(id))
            .map(|event| &event.kind)
        {
            if *before == hex && *before_short == short {
                return;
            }
        }
        self.events.record(
            day,
            Some(pulse),
            EventKind::RefuellingHeldAtReserve {
                ship,
                task_force,
                hex,
                short_t: short,
            },
            causes.to_vec(),
            None,
        );
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
    use crate::time::Pulse;
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
            torpedo_tubes_one_side: 0,
            torpedo_damage: Fixed::ZERO,
            toughness: Fixed::from_int(100),
            radar: false,
            aircraft_capacity: 0,
            anti_aircraft: 0,
            troop_capacity_men: 0,
        };
        let mut ref_data = RefData::default();
        ref_data.ship_classes.insert(class.id, class);
        ref_data.port_sizes.insert(
            0,
            PortSize {
                berths: 0,
                berth_rate_t_per_h: Fixed::ZERO,
                lighter_rate_t_per_h: Fixed::from_int(20),
                fuel_berth_rate_t_per_h: None,
                fuel_lighter_rate_t_per_h: None,
                ammunition_berth_rate_t_per_h: None,
                ammunition_lighter_rate_t_per_h: None,
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
                order: TaskForceOrder::route(waypoints.to_vec()),
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
    fn an_arrival_is_the_cause_of_the_refuelling_in_the_pulse_of_the_arrival() {
        let mut world = world(
            16,
            &[(1, Side::Allied, HERE, &[(1, 1900)])],
            &[(1, Side::Allied, Hex::new(2, 0), 500)],
        );
        world.submit_orders(Side::Allied, vec![route(1, &[Hex::new(2, 0)])]);
        world.resolve_day().expect("day 1 resolves");
        let arrival = world
            .events()
            .iter()
            .find(|event| matches!(event.kind, EventKind::TaskForceArrived { .. }))
            .expect("the force arrived")
            .id;
        let events: Vec<_> = world
            .events()
            .iter()
            .filter(|event| matches!(event.kind, EventKind::ShipRefuelled { .. }))
            .collect();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].causes, vec![Cause::Event(arrival)]);
        // 80 nm at 16 kn is 5 hours: the force arrives in the night pulse
        // and refuels in the 7 hours of it that are left (decision 0112),
        // not the morning after.
        assert_eq!(events[0].pulse, Some(Pulse::Night));
        assert_eq!(events[0].day, 1);
        // The ship burned 80 nm x 0.25 t = 20 t and took the 120 t it lacked.
        assert_eq!(fuel(&world, 1), tons(2000));
        world.resolve_day().expect("day 2 resolves");
        assert_eq!(refuelled(&world).len(), 1);
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
        // The force was under way at the base's centre and left it: nothing
        // was refuelled, at the start of day 2 or in any pulse.
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
        // so run only the loading phase, which must skip it all the same.
        world.handle_cargo(1, Pulse::Night);
        assert_eq!(refuelled(&world), vec![(2, 1, 1000, 2000, 4000, false)]);
        assert_eq!(fuel(&world, 1), tons(1000));
        assert_eq!(fuel(&world, 3), tons(1000));
        assert_eq!(base_fuel(&world, 1), tons(4000));
    }

    #[test]
    fn a_force_that_ran_dry_exactly_in_a_base_hex_refuels_from_the_next_pulse_and_can_be_ordered_on(
    ) {
        // 10 t at 0.25 t a nm is 40 nm: the force runs dry at the centre of
        // (1,0), which holds a base with fuel, in the night pulse. It takes
        // no fuel in that pulse (the hour it ran dry is not known) and
        // fuels in the morning pulse, which restores it (decision 0112).
        let mut world = world(
            16,
            &[(1, Side::Allied, HERE, &[(1, 10)])],
            &[(1, Side::Allied, Hex::new(1, 0), 500)],
        );
        world.submit_orders(Side::Allied, vec![route(1, &[Hex::new(5, 0)])]);
        world.resolve_day().expect("day 1 resolves");
        // The ship had 0 t and lacked 2,000 t: it took the base's 500 t in
        // the morning pulse and the force was restored, idle with no route.
        assert_eq!(refuelled(&world), vec![(1, 1, 500, 500, 0, true)]);
        let pulses: Vec<_> = world
            .events()
            .iter()
            .filter(|event| matches!(event.kind, EventKind::ShipRefuelled { .. }))
            .map(|event| (event.day, event.pulse))
            .collect();
        assert_eq!(pulses, vec![(1, Some(Pulse::Morning))]);
        {
            let force = &world.umpire().task_forces[&tf(1)];
            assert_eq!(force.status, TaskForceStatus::Idle);
            assert_eq!(force.toward_hex, None);
            assert_eq!(force.position(), Hex::new(1, 0));
            assert!(force.route_order.is_none());
        }

        world.submit_orders(Side::Allied, vec![route(1, &[Hex::new(5, 0)])]);
        world.resolve_day().expect("day 2 resolves");
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
        // The base is in the force's own hex, but nothing is refuelled
        // before the force sails: it takes fuel only in the pulses, once
        // stopped (decision 0112).
        world.resolve_day().expect("day 1 resolves");
        assert_eq!(
            world.umpire().task_forces[&tf(1)].status,
            TaskForceStatus::OutOfFuel
        );
        assert!(world.umpire().task_forces[&tf(1)].toward_hex.is_some());
        world.resolve_day().expect("day 2 resolves");
        assert!(refuelled(&world).is_empty());
        assert_eq!(fuel(&world, 1), Fixed::ZERO);
        assert_eq!(base_fuel(&world, 1), tons(500));
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
        // Nothing but the base's count at the end of the day (decision 0067).
        assert_eq!(summary.event_count, 1);
        assert!(matches!(
            empty.events().iter().last().expect("an event").kind,
            EventKind::BaseAtDayEnd { .. }
        ));
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

    /// Gives the hex `HERE` the port size 1 with `berths` berths, 50 t an
    /// hour of cargo at a berth, and the fuel rates given (decision 0112).
    fn port_here(
        world: &mut World,
        berths: u32,
        fuel_berth: Option<i64>,
        fuel_lighter: Option<i64>,
    ) {
        world.ref_data.port_sizes.insert(
            1,
            PortSize {
                berths,
                berth_rate_t_per_h: tons(50),
                lighter_rate_t_per_h: tons(20),
                fuel_berth_rate_t_per_h: fuel_berth.map(tons),
                fuel_lighter_rate_t_per_h: fuel_lighter.map(tons),
                ammunition_berth_rate_t_per_h: None,
                ammunition_lighter_rate_t_per_h: None,
            },
        );
        world.map.insert(
            HERE,
            HexData {
                port_size: 1,
                ..HexData::open_sea()
            },
        );
    }

    /// Lets the class carry 3,000 t of cargo, and the base hold `supply` t.
    fn cargo_ships(world: &mut World, supply: i64) {
        world
            .ref_data
            .ship_classes
            .get_mut(&ShipClassId::new(1))
            .expect("class")
            .cargo_capacity_t = tons(3000);
        world
            .truth
            .bases
            .get_mut(&BaseId::new(1))
            .expect("base")
            .supply_t = tons(supply);
    }

    fn load(target: u32) -> OrderRequest {
        OrderRequest {
            kind: OrderKind::TaskForce {
                target: tf(target),
                order: TaskForceOrder::Load,
            },
            origin: Origin::PLAYER,
        }
    }

    /// The refuelling events as (ship, pulse, taken, on berth by the
    /// breakdown), in the order recorded.
    fn fuelled_in_pulses(world: &World) -> Vec<(u32, Option<Pulse>, i64, i64)> {
        world
            .events()
            .iter()
            .filter_map(|event| match &event.kind {
                EventKind::ShipRefuelled { ship, taken_t, .. } => {
                    let on_berth = event.breakdown.as_ref().map_or(-1, |breakdown| {
                        breakdown
                            .factors
                            .iter()
                            .find(|factor| factor.name == "on_berth")
                            .map_or(-1, |factor| factor.value.trunc())
                    });
                    Some((ship.raw(), event.pulse, taken_t.trunc(), on_berth))
                }
                _ => None,
            })
            .collect()
    }

    fn held_events(world: &World) -> Vec<(u32, Option<Pulse>, i64)> {
        world
            .events()
            .iter()
            .filter_map(|event| match &event.kind {
                EventKind::RefuellingHeldAtReserve { ship, short_t, .. } => {
                    Some((ship.raw(), event.pulse, short_t.trunc()))
                }
                _ => None,
            })
            .collect()
    }

    #[test]
    fn a_berth_goes_to_a_ship_that_fuels_and_the_other_fuels_by_lighter() {
        // One berth at 100 t an hour for fuel, 40 by lighter. Ship 1 (force
        // 1) and ship 2 (force 2) lack 2,000 t each; the berth goes to ship
        // 1, the lower force, which takes 1,200 t in the 12 hours of the
        // night; ship 2 takes 480 by lighter.
        let mut world = world(
            16,
            &[
                (1, Side::Allied, HERE, &[(1, 0)]),
                (2, Side::Allied, HERE, &[(2, 0)]),
            ],
            &[(1, Side::Allied, HERE, 9000)],
        );
        port_here(&mut world, 1, Some(100), Some(40));
        world.set_config(SimConfig {
            record_breakdowns: true,
        });
        world.handle_cargo(1, Pulse::Night);
        assert_eq!(
            fuelled_in_pulses(&world),
            vec![
                (1, Some(Pulse::Night), 1200, 1),
                (2, Some(Pulse::Night), 480, 0)
            ]
        );
        assert_eq!(base_fuel(&world, 1), tons(9000 - 1200 - 480));
        // The factors say how it was reckoned.
        let first = world.events().iter().next().expect("an event");
        let factors: Vec<_> = first
            .breakdown
            .as_ref()
            .expect("breakdown")
            .factors
            .iter()
            .map(|factor| (factor.name, factor.value.trunc()))
            .collect();
        assert_eq!(
            factors,
            vec![
                ("hours_worked", 12),
                ("cargo_hours", 0),
                ("hours_left", 12),
                ("on_berth", 1),
                ("rate_t_per_h", 100),
                ("lacks_t", 2000),
                ("base_above_reserve_t", 9000),
            ]
        );
    }

    #[test]
    fn a_ship_takes_fuel_after_its_cargo_in_the_hours_that_are_left() {
        // The load of 500 t of supply at 50 t an hour takes 10 hours of the
        // night; the 2 hours left give 200 t of fuel at the berth's 100 t an
        // hour. The morning pulse gives the last 200 t in 2 hours.
        let mut world = world(
            16,
            &[(1, Side::Allied, HERE, &[(1, 1600)])],
            &[(1, Side::Allied, HERE, 5000)],
        );
        port_here(&mut world, 1, Some(100), Some(40));
        cargo_ships(&mut world, 500);
        world.submit_orders(Side::Allied, vec![load(1)]);
        world.resolve_day().expect("day resolves");
        let taken: Vec<_> = refuelled(&world)
            .into_iter()
            .map(|(_, _, taken, aboard, _, _)| (taken, aboard))
            .collect();
        assert_eq!(taken, vec![(200, 1800), (200, 2000)]);
        let pulses: Vec<_> = world
            .events()
            .iter()
            .filter(|event| matches!(event.kind, EventKind::ShipRefuelled { .. }))
            .map(|event| event.pulse)
            .collect();
        assert_eq!(pulses, vec![Some(Pulse::Night), Some(Pulse::Morning)]);
        assert_eq!(world.umpire().ships[&ship(1)].supply_t, tons(500));
    }

    #[test]
    fn with_no_rate_given_a_ship_fills_at_once_whatever_hours_its_cargo_took() {
        // Neither fuel rate is given: no limit, so the ship fills in the
        // night pulse though its cargo took all 12 hours, and fuel alone
        // does not claim the berth.
        let mut world = world(
            16,
            &[
                (1, Side::Allied, HERE, &[(1, 1600)]),
                (2, Side::Allied, HERE, &[(2, 0)]),
            ],
            &[(1, Side::Allied, HERE, 5000)],
        );
        port_here(&mut world, 1, None, None);
        cargo_ships(&mut world, 5000);
        world.submit_orders(Side::Allied, vec![load(1)]);
        world.resolve_day().expect("day resolves");
        // Ship 1: 12 hours of loading (600 t), and its 400 t of fuel.
        let night: Vec<_> = refuelled(&world)
            .into_iter()
            .map(|(ship, _, taken, _, _, _)| (ship, taken))
            .collect();
        assert_eq!(night, vec![(1, 400), (2, 2000)]);
        assert_eq!(base_fuel(&world, 1), tons(5000 - 2400));
        // The load went on through the three pulses of the day, 50 t an hour.
        assert_eq!(world.umpire().ships[&ship(1)].supply_t, tons(1200));
        let first_pulses: Vec<_> = world
            .events()
            .iter()
            .filter(|event| matches!(event.kind, EventKind::ShipRefuelled { .. }))
            .map(|event| event.pulse)
            .collect();
        assert_eq!(first_pulses, vec![Some(Pulse::Night), Some(Pulse::Night)]);
    }

    #[test]
    fn a_ship_held_at_a_reserve_is_recorded_once_while_nothing_changes() {
        // The base holds 300 t and may give 200, down to its reserve of
        // 100. The ship lacks 500: it takes 200 by lighter at 40 t an hour
        // in the night and is held 300 short, recorded then and not again in
        // the pulses and the day after.
        let mut world = world(
            16,
            &[(1, Side::Allied, HERE, &[(1, 1500)])],
            &[(1, Side::Allied, HERE, 300)],
        );
        port_here(&mut world, 0, None, Some(40));
        world
            .truth
            .bases
            .get_mut(&BaseId::new(1))
            .expect("base")
            .fuel_reserve_t = tons(100);
        world.resolve_day().expect("day 1 resolves");
        world.resolve_day().expect("day 2 resolves");
        assert_eq!(held_events(&world), vec![(1, Some(Pulse::Night), 300)]);
        assert_eq!(fuel(&world, 1), tons(1700));
        assert_eq!(base_fuel(&world, 1), tons(100));
        // It cites the refuelling that took the 200 t.
        let refuelled_id = world
            .events()
            .iter()
            .find(|event| matches!(event.kind, EventKind::ShipRefuelled { .. }))
            .expect("refuelled")
            .id;
        let held = world
            .events()
            .iter()
            .find(|event| matches!(event.kind, EventKind::RefuellingHeldAtReserve { .. }))
            .expect("held");
        assert_eq!(held.causes, vec![Cause::Event(refuelled_id)]);
        // Fuel burned changes the shortfall, so it is recorded again.
        world.truth.ships.get_mut(&ship(1)).expect("ship").fuel_t = tons(1600);
        world.resolve_day().expect("day 3 resolves");
        assert_eq!(
            held_events(&world),
            vec![(1, Some(Pulse::Night), 300), (1, Some(Pulse::Night), 400)]
        );
    }

    #[test]
    fn a_rate_that_runs_out_is_not_a_reserve_hold() {
        // The base has plenty above its reserve: the ship that is short
        // after its 12 hours at 40 t an hour is short of hours, not held.
        let mut world = world(
            16,
            &[(1, Side::Allied, HERE, &[(1, 0)])],
            &[(1, Side::Allied, HERE, 9000)],
        );
        port_here(&mut world, 0, None, Some(40));
        world.handle_cargo(1, Pulse::Night);
        assert_eq!(refuelled(&world).len(), 1);
        assert!(held_events(&world).is_empty());
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
            // The breakdown is what recording adds; every other part of an
            // event, and the hash, is as it was.
            let events: Vec<_> = world
                .events()
                .iter()
                .map(|event| {
                    (
                        event.id,
                        event.day,
                        event.pulse,
                        event.kind.clone(),
                        event.causes.clone(),
                    )
                })
                .collect();
            let breakdowns = world
                .events()
                .iter()
                .filter(|event| event.breakdown.is_some())
                .count();
            (events, world.day_hashes().to_vec(), breakdowns)
        };
        let (with_events, with_hashes, with_breakdowns) = run(true);
        let (without_events, without_hashes, without_breakdowns) = run(false);
        assert_eq!(with_events, without_events);
        assert_eq!(with_hashes, without_hashes);
        assert_eq!(with_breakdowns, 1);
        assert_eq!(without_breakdowns, 0);
    }
}
