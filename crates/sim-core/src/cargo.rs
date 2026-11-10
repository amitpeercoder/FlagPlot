//! Cargo handling in the loading phase.
//!
//! The rule is `docs/decisions/0011-cargo-handling.md`. A task force that is
//! stopped at a base of its own side moves supply between its ships and the
//! base's stock, at the rates of the port. There is no randomness.

use std::collections::BTreeMap;

use crate::events::{Breakdown, Cause, EventKind, Factor};
use crate::fixed::Fixed;
use crate::ids::{BaseId, EventId, ShipId, TaskForceId};
use crate::orders::CargoAction;
use crate::refdata::PortSize;
use crate::state::{CargoOrder, TaskForceStatus};
use crate::time::{pulse_start, Pulse};
use crate::world::World;

/// A task force that works cargo in this pulse.
struct Participant {
    task_force: TaskForceId,
    base: BaseId,
    cargo: CargoOrder,
    /// The hour the task force starts working: the later of the pulse start
    /// and its arrival.
    work_start: Fixed,
    hours: Fixed,
    ships: Vec<ShipId>,
    port: PortSize,
    arrival_event: Option<EventId>,
}

impl World {
    /// Moves cargo for every task force stopped at a base of its own side.
    pub(crate) fn handle_cargo(&mut self, day: u32, pulse: Pulse) {
        let start = pulse_start(day, pulse);
        let end = start + Fixed::from_int(pulse.hours());

        let mut participants: Vec<Participant> = Vec::new();
        for task_force in self.truth.task_forces.values() {
            let Some(cargo) = task_force.cargo_order else {
                continue;
            };
            // Only a task force that has stopped on purpose works cargo. One
            // that ran dry may be miles from any base, whatever hex
            // it is nearest to.
            if !matches!(
                task_force.status,
                TaskForceStatus::Idle | TaskForceStatus::Arrived
            ) {
                continue;
            }
            let Some(base) = self
                .truth
                .bases
                .values()
                .find(|base| base.side == task_force.side && base.hex == task_force.position())
            else {
                continue;
            };
            let work_start = match task_force.arrived_at_hours {
                Some(arrived) => start.max(arrived),
                None => start,
            };
            let hours = end - work_start;
            if hours <= Fixed::ZERO {
                continue;
            }
            let size = self
                .map
                .get(task_force.position())
                .map_or(0, |hex| hex.port_size);
            participants.push(Participant {
                task_force: task_force.id,
                base: base.id,
                cargo,
                work_start,
                hours,
                ships: task_force.ships.iter().copied().collect(),
                port: self.ref_data.port_sizes[&size],
                arrival_event: task_force.arrival_event,
            });
        }

        // Berths are given out before any cargo moves.
        let mut berths_used: BTreeMap<BaseId, u32> = BTreeMap::new();
        let mut berthed: Vec<ShipId> = Vec::new();
        for participant in &participants {
            let stock = self.truth.bases[&participant.base].supply_t;
            for ship_id in &participant.ships {
                let used = berths_used.entry(participant.base).or_insert(0);
                if *used >= participant.port.berths {
                    break;
                }
                let ship = &self.truth.ships[ship_id];
                let capacity = self.ref_data.ship_classes[&ship.class].cargo_capacity_t;
                let has_cargo_to_move = match participant.cargo.action {
                    CargoAction::Load => {
                        capacity - ship.supply_t > Fixed::ZERO && stock > Fixed::ZERO
                    }
                    CargoAction::Unload => ship.supply_t > Fixed::ZERO,
                };
                if has_cargo_to_move {
                    *used += 1;
                    berthed.push(*ship_id);
                }
            }
        }

        for participant in &participants {
            self.work_cargo(day, pulse, participant, &berthed);
        }
    }

    fn work_cargo(&mut self, day: u32, pulse: Pulse, p: &Participant, berthed: &[ShipId]) {
        let mut causes = vec![Cause::Order(p.cargo.order)];
        if let Some(arrival) = p.arrival_event {
            causes.push(Cause::Event(arrival));
        }
        let mut cargo_events: Vec<Cause> = Vec::new();
        let mut longest = Fixed::ZERO;
        let mut moved_any = false;

        for ship_id in &p.ships {
            let on_berth = berthed.contains(ship_id);
            let rate = if on_berth {
                p.port.berth_rate_t_per_h
            } else {
                p.port.lighter_rate_t_per_h
            };
            let capacity_by_time = rate * p.hours;
            let ship = &self.truth.ships[ship_id];
            let capacity_t = self.ref_data.ship_classes[&ship.class].cargo_capacity_t;
            let aboard_before = ship.supply_t;
            let room = capacity_t - aboard_before;
            let stock_before = self.truth.bases[&p.base].supply_t;

            let moved = match p.cargo.action {
                CargoAction::Unload => capacity_by_time.min(aboard_before),
                CargoAction::Load => capacity_by_time.min(room).min(stock_before),
            };
            if moved <= Fixed::ZERO {
                continue;
            }

            let (aboard_after, stock_after) = match p.cargo.action {
                CargoAction::Unload => (aboard_before - moved, stock_before + moved),
                CargoAction::Load => (aboard_before + moved, stock_before - moved),
            };
            self.truth
                .ships
                .get_mut(ship_id)
                .expect("ship exists")
                .supply_t = aboard_after;
            self.truth
                .bases
                .get_mut(&p.base)
                .expect("base exists")
                .supply_t = stock_after;

            let breakdown = self.config.record_breakdowns.then(|| {
                let mut factors = vec![
                    Factor {
                        name: "hours_worked",
                        value: p.hours,
                    },
                    Factor {
                        name: "on_berth",
                        value: if on_berth { Fixed::ONE } else { Fixed::ZERO },
                    },
                    Factor {
                        name: "rate_t_per_h",
                        value: rate,
                    },
                    Factor {
                        name: "capacity_by_time_t",
                        value: capacity_by_time,
                    },
                ];
                match p.cargo.action {
                    CargoAction::Unload => factors.push(Factor {
                        name: "supply_aboard_t",
                        value: aboard_before,
                    }),
                    CargoAction::Load => {
                        factors.push(Factor {
                            name: "room_t",
                            value: room,
                        });
                        factors.push(Factor {
                            name: "base_stock_t",
                            value: stock_before,
                        });
                    }
                }
                Breakdown {
                    factors,
                    result: moved,
                }
            });
            let kind = match p.cargo.action {
                CargoAction::Unload => EventKind::CargoUnloaded {
                    ship: *ship_id,
                    task_force: p.task_force,
                    base: p.base,
                    unloaded_t: moved,
                    aboard_t: aboard_after,
                    base_stock_t: stock_after,
                },
                CargoAction::Load => EventKind::CargoLoaded {
                    ship: *ship_id,
                    task_force: p.task_force,
                    base: p.base,
                    loaded_t: moved,
                    aboard_t: aboard_after,
                    base_stock_t: stock_after,
                },
            };
            let event = self
                .events
                .record(day, Some(pulse), kind, causes.clone(), breakdown);
            cargo_events.push(Cause::Event(event));
            moved_any = true;
            longest = longest.max(moved / rate);
        }

        if !moved_any {
            return;
        }
        let nothing_remains = match p.cargo.action {
            CargoAction::Unload => p
                .ships
                .iter()
                .all(|id| self.truth.ships[id].supply_t.is_zero()),
            CargoAction::Load => {
                self.truth.bases[&p.base].supply_t.is_zero()
                    || p.ships.iter().all(|id| {
                        let ship = &self.truth.ships[id];
                        ship.supply_t == self.ref_data.ship_classes[&ship.class].cargo_capacity_t
                    })
            }
        };
        if !nothing_remains {
            return;
        }
        let elapsed_hours = p.work_start + longest;
        let kind = match p.cargo.action {
            CargoAction::Unload => EventKind::UnloadingComplete {
                task_force: p.task_force,
                base: p.base,
                elapsed_hours,
            },
            CargoAction::Load => EventKind::LoadingComplete {
                task_force: p.task_force,
                base: p.base,
                elapsed_hours,
            },
        };
        let breakdown = self.config.record_breakdowns.then(|| Breakdown {
            factors: vec![
                Factor {
                    name: "work_start_h",
                    value: p.work_start,
                },
                Factor {
                    name: "longest_ship_hours",
                    value: longest,
                },
            ],
            result: elapsed_hours,
        });
        self.events
            .record(day, Some(pulse), kind, cargo_events, breakdown);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EventKind;
    use crate::hex::Hex;
    use crate::ids::ShipClassId;
    use crate::map::{HexData, Map};
    use crate::orders::{Mission, OrderKind, OrderRequest, Origin, TaskForceOrder};
    use crate::refdata::{RefData, ShipClass, ShipKind};
    use crate::state::{Base, Ship, ShipLocation, ShipSystems, Side, TaskForce, TruthState};
    use crate::world::SimConfig;

    /// One task force for `world`: its ID, its hex, and its ships as
    /// (ship ID, supply aboard in tons).
    type Force<'a> = (u32, Hex, &'a [(u32, i64)]);

    fn tf(id: u32) -> TaskForceId {
        TaskForceId::new(id)
    }

    fn ship(id: u32) -> ShipId {
        ShipId::new(id)
    }

    /// Open sea, base 1 at (0,0) with port size `port` and `stock_t` tons,
    /// base 2 at (4,0) with port size 1 and no stock. Cargo ships carry 3000 t
    /// and steam at 16 kn. Test-only port figures:
    ///
    /// - size 0: no berths, lighters 20 t/h
    /// - size 1: `berths` berths at 50 t/h, lighters 20 t/h
    /// - size 2: 2 berths at 250 t/h, lighters 20 t/h
    fn world(port: u8, berths: u32, stock_t: i64, forces: &[Force]) -> World {
        let mut map = Map::new();
        for q in -1..=8 {
            for r in -1..=2 {
                map.insert(Hex::new(q, r), HexData::open_sea());
            }
        }
        for (hex, size) in [(Hex::new(0, 0), port), (Hex::new(4, 0), 1)] {
            map.insert(
                hex,
                HexData {
                    port_size: size,
                    ..HexData::open_sea()
                },
            );
        }

        let class = ShipClass {
            id: ShipClassId::new(1),
            name: "Cargo".to_string(),
            kind: ShipKind::Transport,
            cruise_speed_kn: Fixed::from_int(16),
            max_speed_kn: Fixed::from_int(20),
            fuel_capacity_t: Fixed::from_int(2000),
            endurance_nm: Fixed::from_int(8000),
            cargo_capacity_t: Fixed::from_int(3000),
            main_guns: 0,
            gun_rounds_per_min: Fixed::ZERO,
            gun_rounds_per_gun: 0,
            gun_damage: Fixed::ZERO,
            torpedo_tubes: 0,
            torpedo_damage: Fixed::ZERO,
            toughness: Fixed::from_int(100),
            radar: false,
        };
        let mut ref_data = RefData::default();
        ref_data.ship_classes.insert(class.id, class);
        for (size, number, rate) in [(0, 0, 0), (1, berths, 50), (2, 2, 250)] {
            ref_data.port_sizes.insert(
                size,
                PortSize {
                    berths: number,
                    berth_rate_t_per_h: Fixed::from_int(rate),
                    lighter_rate_t_per_h: Fixed::from_int(20),
                },
            );
        }

        let mut truth = TruthState::default();
        for (id, hex, ships) in forces {
            let mut force = TaskForce::at(
                tf(*id),
                format!("Force {id}"),
                Side::Allied,
                *hex,
                Mission::Transport,
            );
            for (ship_id, supply) in *ships {
                force.ships.insert(ship(*ship_id));
                truth.ships.insert(
                    ship(*ship_id),
                    Ship {
                        id: ship(*ship_id),
                        name: format!("Ship {ship_id}"),
                        class: ShipClassId::new(1),
                        side: Side::Allied,
                        fuel_t: Fixed::from_int(2000),
                        supply_t: Fixed::from_int(*supply),
                        location: ShipLocation::TaskForce(tf(*id)),
                        flotation_damage: Fixed::ZERO,
                        fire_damage: Fixed::ZERO,
                        gun_rounds: Fixed::ZERO,
                        torpedoes_ready: false,
                        systems: ShipSystems::ALL_WORKING,
                    },
                );
            }
            truth.task_forces.insert(tf(*id), force);
        }
        for (id, hex, stock) in [(1, Hex::new(0, 0), stock_t), (2, Hex::new(4, 0), 0)] {
            truth.bases.insert(
                BaseId::new(id),
                Base {
                    id: BaseId::new(id),
                    name: format!("Base {id}"),
                    side: Side::Allied,
                    hex,
                    supply_t: Fixed::from_int(stock),
                },
            );
        }
        World::new(7, ref_data, map, truth).expect("valid setup")
    }

    fn order(target: u32, order: TaskForceOrder) -> OrderRequest {
        OrderRequest {
            kind: OrderKind::TaskForce {
                target: tf(target),
                order,
            },
            origin: Origin::PLAYER,
        }
    }

    fn on_berth(event: &crate::events::Event) -> Fixed {
        event
            .breakdown
            .as_ref()
            .expect("breakdown recorded")
            .factors
            .iter()
            .find(|factor| factor.name == "on_berth")
            .expect("on_berth factor")
            .value
    }

    #[test]
    fn load_fills_ships_in_id_order_when_the_stock_runs_short() {
        // Port size 2: 2 berths at 250 t/h, so the night pulse (12 h) can move
        // 3000 t per ship. Stock 4000 t: ship 1 takes 3000 t, ship 2 the
        // remaining 1000 t. Ship 1 works 12 h, ship 2 works 1000/250 = 4 h.
        // The stock is gone in the night pulse, so loading completes at
        // elapsed hour 0 + 12.
        let mut world = world(2, 1, 4000, &[(1, Hex::new(0, 0), &[(1, 0), (2, 0)])]);
        world.submit_orders(Side::Allied, vec![order(1, TaskForceOrder::Load)]);
        world.resolve_day().expect("day resolves");

        let truth = world.umpire();
        assert_eq!(truth.ships[&ship(1)].supply_t, Fixed::from_int(3000));
        assert_eq!(truth.ships[&ship(2)].supply_t, Fixed::from_int(1000));
        assert_eq!(truth.bases[&BaseId::new(1)].supply_t, Fixed::ZERO);

        let loaded: Vec<_> = world
            .events()
            .iter()
            .filter(|event| matches!(event.kind, EventKind::CargoLoaded { .. }))
            .collect();
        assert_eq!(loaded.len(), 2);
        let completes: Vec<_> = world
            .events()
            .iter()
            .filter(|event| matches!(event.kind, EventKind::LoadingComplete { .. }))
            .collect();
        assert_eq!(completes.len(), 1);
        let complete = completes[0];
        assert_eq!(complete.pulse, Some(Pulse::Night));
        assert_eq!(
            complete.kind,
            EventKind::LoadingComplete {
                task_force: tf(1),
                base: BaseId::new(1),
                elapsed_hours: Fixed::from_int(12),
            }
        );
        let expected_causes: Vec<Cause> = loaded.iter().map(|e| Cause::Event(e.id)).collect();
        assert_eq!(complete.causes, expected_causes);
    }

    #[test]
    fn a_task_force_whose_ships_are_all_sunk_works_no_cargo() {
        for action in [TaskForceOrder::Load, TaskForceOrder::Unload] {
            let mut world = world(1, 1, 1000, &[(1, Hex::new(0, 0), &[(1, 0), (2, 500)])]);
            for raw in [1, 2] {
                let record = world.truth.ships.get_mut(&ship(raw)).expect("ship");
                record.location = ShipLocation::Sunk;
                record.flotation_damage = Fixed::from_int(100);
            }
            world
                .truth
                .task_forces
                .get_mut(&tf(1))
                .expect("task force")
                .ships
                .clear();
            world.submit_orders(Side::Allied, vec![order(1, action)]);
            world.resolve_day().expect("day resolves");

            assert!(world.events().iter().all(|event| !matches!(
                event.kind,
                EventKind::CargoLoaded { .. }
                    | EventKind::CargoUnloaded { .. }
                    | EventKind::LoadingComplete { .. }
                    | EventKind::UnloadingComplete { .. }
            )));
            let truth = world.umpire();
            assert_eq!(truth.bases[&BaseId::new(1)].supply_t, Fixed::from_int(1000));
            // The sunk ship keeps what it carried: nothing is unloaded from it.
            assert_eq!(truth.ships[&ship(2)].supply_t, Fixed::from_int(500));
        }
    }

    #[test]
    fn unload_without_a_base_here_does_nothing() {
        let mut world = world(1, 1, 0, &[(1, Hex::new(2, 0), &[(1, 500)])]);
        world.submit_orders(Side::Allied, vec![order(1, TaskForceOrder::Unload)]);
        world.resolve_day().expect("day resolves");

        assert!(!world.events().iter().any(|event| matches!(
            event.kind,
            EventKind::CargoUnloaded { .. } | EventKind::UnloadingComplete { .. }
        )));
        assert_eq!(
            world.umpire().ships[&ship(1)].supply_t,
            Fixed::from_int(500)
        );
        assert_eq!(world.umpire().bases[&BaseId::new(1)].supply_t, Fixed::ZERO);
    }

    #[test]
    fn a_task_force_that_ran_dry_does_not_work_cargo() {
        // 1 t of fuel at 0.25 t/nm lasts 4 nm. The force runs dry 4 nm out of
        // (0,0). It still counts as being in that hex, where base 1 is, but it
        // is adrift and must not unload.
        let mut world = world(1, 1, 0, &[(1, Hex::new(0, 0), &[(1, 1000)])]);
        world
            .truth
            .ships
            .get_mut(&ship(1))
            .expect("ship exists")
            .fuel_t = Fixed::ONE;
        world.submit_orders(
            Side::Allied,
            vec![
                order(1, TaskForceOrder::SetRoute(vec![Hex::new(4, 0)])),
                order(1, TaskForceOrder::Unload),
            ],
        );
        world.resolve_day().expect("day resolves");

        let truth = world.umpire();
        assert_eq!(truth.task_forces[&tf(1)].status, TaskForceStatus::OutOfFuel);
        let task_force = &truth.task_forces[&tf(1)];
        // 4 nm along its first leg: still nearest (0,0).
        assert_eq!(task_force.position(), Hex::new(0, 0));
        assert_eq!(task_force.from_hex, Hex::new(0, 0));
        assert_eq!(task_force.toward_hex, Some(Hex::new(1, 0)));
        assert_eq!(task_force.progress_nm, Fixed::from_int(4));
        assert_eq!(truth.ships[&ship(1)].supply_t, Fixed::from_int(1000));
        assert!(!world
            .events()
            .iter()
            .any(|event| matches!(event.kind, EventKind::CargoUnloaded { .. })));
    }

    /// Unloaded tons by ship and whether it was on a berth, in the night pulse.
    fn night_unloads(world: &World) -> Vec<(ShipId, Fixed, Fixed)> {
        world
            .events()
            .iter()
            .filter(|event| event.pulse == Some(Pulse::Night))
            .filter_map(|event| match event.kind {
                EventKind::CargoUnloaded {
                    ship, unloaded_t, ..
                } => Some((ship, unloaded_t, on_berth(event))),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn berths_go_by_task_force_then_ship_id() {
        let forces: [Force; 2] = [
            (1, Hex::new(0, 0), &[(1, 1000), (2, 1000)]),
            (2, Hex::new(0, 0), &[(3, 1000), (4, 1000)]),
        ];
        // Port size 1 with one berth at 50 t/h, lighters 20 t/h, 12 hours:
        // the berth moves 600 t, a lighter 240 t.
        let mut one = world(1, 1, 0, &forces);
        for id in [1, 2] {
            one.submit_orders(Side::Allied, vec![order(id, TaskForceOrder::Unload)]);
        }
        one.resolve_day().expect("day resolves");
        let (t600, t240) = (Fixed::from_int(600), Fixed::from_int(240));
        assert_eq!(
            night_unloads(&one),
            vec![
                (ship(1), t600, Fixed::ONE),
                (ship(2), t240, Fixed::ZERO),
                (ship(3), t240, Fixed::ZERO),
                (ship(4), t240, Fixed::ZERO),
            ]
        );

        // Two berths: ships 1 and 2 are berthed, ships 3 and 4 use lighters.
        let mut two = world(1, 2, 0, &forces);
        for id in [1, 2] {
            two.submit_orders(Side::Allied, vec![order(id, TaskForceOrder::Unload)]);
        }
        two.resolve_day().expect("day resolves");
        assert_eq!(
            night_unloads(&two),
            vec![
                (ship(1), t600, Fixed::ONE),
                (ship(2), t600, Fixed::ONE),
                (ship(3), t240, Fixed::ZERO),
                (ship(4), t240, Fixed::ZERO),
            ]
        );
    }

    #[test]
    fn unload_waits_for_arrival_and_counts_hours_from_it() {
        // (0,0) to (4,0) is 160 nm: 10 hours at 16 kn. The force arrives
        // 10 hours into the 12-hour night pulse and unloads for 2 hours at
        // the berth rate of 50 t/h: 100 t.
        let mut world = world(1, 1, 0, &[(1, Hex::new(0, 0), &[(1, 1000)])]);
        world.submit_orders(
            Side::Allied,
            vec![
                order(1, TaskForceOrder::SetRoute(vec![Hex::new(4, 0)])),
                order(1, TaskForceOrder::Unload),
            ],
        );
        world.resolve_day().expect("day resolves");

        let truth = world.umpire();
        let force = &truth.task_forces[&tf(1)];
        let unload_order = force.cargo_order.expect("cargo order").order;
        let arrival = force.arrival_event.expect("arrival event");
        assert_eq!(force.arrived_at_hours, Some(Fixed::from_int(10)));

        let unloads: Vec<_> = world
            .events()
            .iter()
            .filter(|event| matches!(event.kind, EventKind::CargoUnloaded { .. }))
            .collect();
        let first = unloads[0];
        assert_eq!(first.pulse, Some(Pulse::Night));
        assert_eq!(
            first.kind,
            EventKind::CargoUnloaded {
                ship: ship(1),
                task_force: tf(1),
                base: BaseId::new(2),
                unloaded_t: Fixed::from_int(100),
                aboard_t: Fixed::from_int(900),
                base_stock_t: Fixed::from_int(100),
            }
        );
        assert_eq!(
            first.causes,
            vec![Cause::Order(unload_order), Cause::Event(arrival)]
        );
        // Nothing moved at base 1 where the force started.
        assert_eq!(truth.bases[&BaseId::new(1)].supply_t, Fixed::ZERO);
        // Day 1 total: 2 h, then 6 h and 6 h of full pulses at 50 t/h.
        assert_eq!(
            truth.ships[&ship(1)].supply_t,
            Fixed::from_int(1000 - 100 - 300 - 300)
        );
    }

    #[test]
    fn breakdown_recording_does_not_change_the_hash_with_cargo() {
        let run = |record_breakdowns: bool| {
            let mut world = world(2, 1, 4000, &[(1, Hex::new(0, 0), &[(1, 0), (2, 0)])]);
            world.set_config(SimConfig { record_breakdowns });
            world.submit_orders(Side::Allied, vec![order(1, TaskForceOrder::Load)]);
            world.resolve_day().expect("day resolves");
            let cargo_events: Vec<_> = world
                .events()
                .iter()
                .filter(|event| {
                    matches!(
                        event.kind,
                        EventKind::CargoLoaded { .. } | EventKind::LoadingComplete { .. }
                    )
                })
                .collect();
            let all_have = cargo_events.iter().all(|event| event.breakdown.is_some());
            let none_have = cargo_events.iter().all(|event| event.breakdown.is_none());
            (
                world.state_hash(),
                !cargo_events.is_empty(),
                all_have,
                none_have,
            )
        };
        let (with_hash, with_events, with_all, _) = run(true);
        let (without_hash, without_events, _, without_none) = run(false);
        assert!(with_events && without_events);
        assert!(with_all);
        assert!(without_none);
        assert_eq!(with_hash, without_hash);
    }
}
