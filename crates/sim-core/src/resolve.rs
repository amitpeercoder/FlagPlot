//! Resolving one day.
//!
//! The order of phases below is the one in the spec ("Daily resolution
//! order"). Only orders intake, task force movement and closing the day do
//! anything so far. The other phases are listed so the sequence is fixed
//! from the start, and each is filled in by the scenario that needs it.

use crate::events::{Breakdown, Cause, EventKind, Factor};
use crate::fixed::Fixed;
use crate::hex::HEX_NM;
use crate::ids::{EventId, ShipId, TaskForceId};
use crate::invariants;
use crate::orders::{Order, OrderKind, RejectReason, SpeedSetting, TaskForceOrder};
use crate::state::{Route, TaskForceStatus};
use crate::time::{pulse_start, Pulse};
use crate::world::{DaySummary, SimError, World};

/// A step inside a pulse.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum PulsePhase {
    Movement,
    NightSearch,
    AirSearch,
    StrikeDecisions,
    AirCombat,
    SurfaceEngagements,
    ShoreBombardment,
    Loading,
}

/// A step after the last pulse.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EndOfDayPhase {
    GroundCombat,
    Logistics,
    AirGroupUpkeep,
    IntelligenceUpdate,
}

const NIGHT_SEQUENCE: [PulsePhase; 5] = [
    PulsePhase::Movement,
    PulsePhase::NightSearch,
    PulsePhase::SurfaceEngagements,
    PulsePhase::ShoreBombardment,
    PulsePhase::Loading,
];

const DAYLIGHT_SEQUENCE: [PulsePhase; 6] = [
    PulsePhase::AirSearch,
    PulsePhase::StrikeDecisions,
    PulsePhase::AirCombat,
    PulsePhase::Movement,
    PulsePhase::SurfaceEngagements,
    PulsePhase::Loading,
];

const END_OF_DAY_SEQUENCE: [EndOfDayPhase; 4] = [
    EndOfDayPhase::GroundCombat,
    EndOfDayPhase::Logistics,
    EndOfDayPhase::AirGroupUpkeep,
    EndOfDayPhase::IntelligenceUpdate,
];

const fn pulse_sequence(pulse: Pulse) -> &'static [PulsePhase] {
    match pulse {
        Pulse::Night => &NIGHT_SEQUENCE,
        Pulse::Morning | Pulse::Afternoon => &DAYLIGHT_SEQUENCE,
    }
}

/// What the movement rules need to know about one ship of a task force.
struct Steamer {
    ship: ShipId,
    fuel_t: Fixed,
    /// The ship's own speed at the task force's speed setting.
    setting_speed_kn: Fixed,
    cruise_speed_kn: Fixed,
    fuel_capacity_t: Fixed,
    endurance_nm: Fixed,
}

/// Fuel burn per mile rises with the square of speed relative to cruise.
/// This is a placeholder model: see `docs/decisions/0006-fuel-model.md`.
fn speed_multiplier(speed_kn: Fixed, cruise_speed_kn: Fixed) -> Fixed {
    let ratio = speed_kn / cruise_speed_kn;
    ratio * ratio
}

impl Steamer {
    /// Fuel needed to steam `distance_nm` at `speed_kn`, in tons.
    fn fuel_for(&self, distance_nm: Fixed, speed_kn: Fixed) -> Fixed {
        distance_nm.mul_div(self.fuel_capacity_t, self.endurance_nm)
            * speed_multiplier(speed_kn, self.cruise_speed_kn)
    }

    /// How far the fuel aboard lasts at `speed_kn`. `None` when the burn rate
    /// rounds to nothing, which means fuel does not limit the ship.
    fn range_nm(&self, speed_kn: Fixed) -> Option<Fixed> {
        let multiplier = speed_multiplier(speed_kn, self.cruise_speed_kn);
        if multiplier.is_zero() {
            return None;
        }
        Some((self.fuel_t / multiplier).mul_div(self.endurance_nm, self.fuel_capacity_t))
    }
}

impl World {
    /// Resolves the next day: takes in the orders both sides submitted, runs
    /// the three pulses and the end-of-day phases, checks the invariants and
    /// records the state hash.
    pub fn resolve_day(&mut self) -> Result<DaySummary, SimError> {
        let day = self.truth.day + 1;
        let first_event = self.events.len();

        self.intake_orders(day);

        for pulse in Pulse::ALL {
            for phase in pulse_sequence(pulse) {
                self.run_pulse_phase(day, pulse, *phase);
            }
        }
        for phase in END_OF_DAY_SEQUENCE {
            self.run_end_of_day_phase(day, phase);
        }

        self.truth.day = day;

        let violations = invariants::check(&self.truth, &self.map);
        if !violations.is_empty() {
            return Err(SimError::InvariantsViolated { day, violations });
        }

        let state_hash = self.state_hash();
        self.day_hashes.push(state_hash);
        Ok(DaySummary {
            day,
            state_hash,
            first_event,
            event_count: self.events.len() - first_event,
        })
    }

    fn run_pulse_phase(&mut self, day: u32, pulse: Pulse, phase: PulsePhase) {
        match phase {
            PulsePhase::Movement => self.move_task_forces(day, pulse),
            // Not implemented yet. Each arrives with the scenario that needs it.
            PulsePhase::NightSearch
            | PulsePhase::AirSearch
            | PulsePhase::StrikeDecisions
            | PulsePhase::AirCombat
            | PulsePhase::SurfaceEngagements
            | PulsePhase::ShoreBombardment
            | PulsePhase::Loading => {}
        }
    }

    fn run_end_of_day_phase(&mut self, _day: u32, phase: EndOfDayPhase) {
        match phase {
            // Not implemented yet. Each arrives with the scenario that needs it.
            EndOfDayPhase::GroundCombat
            | EndOfDayPhase::Logistics
            | EndOfDayPhase::AirGroupUpkeep
            | EndOfDayPhase::IntelligenceUpdate => {}
        }
    }

    /// Takes in every order submitted since the last day. Sides are handled
    /// in a fixed order, and each side's orders in the order it gave them.
    fn intake_orders(&mut self, day: u32) {
        let pending = std::mem::take(&mut self.pending);
        for (side, requests) in pending {
            for request in requests {
                let id = self.order_ids.alloc();
                let order = Order {
                    id,
                    side,
                    day_issued: day,
                    kind: request.kind,
                    origin: request.origin,
                };
                let target = order.kind.target();
                let kind = match self.validate_order(&order) {
                    Ok(()) => {
                        self.apply_order(&order);
                        EventKind::OrderAccepted { order: id, target }
                    }
                    Err(reason) => EventKind::OrderRejected {
                        order: id,
                        target,
                        reason,
                    },
                };
                self.events
                    .record(day, None, kind, vec![Cause::Order(id)], None);
                self.orders.insert(id, order);
            }
        }
    }

    /// Orders are checked against the truth, as the spec says. Note that a
    /// rejection can therefore tell a side something its own view would not.
    fn validate_order(&self, order: &Order) -> Result<(), RejectReason> {
        match &order.kind {
            OrderKind::TaskForce {
                target,
                order: task_force_order,
            } => {
                let task_force = self
                    .truth
                    .task_forces
                    .get(target)
                    .ok_or(RejectReason::UnknownTarget)?;
                if task_force.side != order.side {
                    return Err(RejectReason::NotYourUnit);
                }
                match task_force_order {
                    TaskForceOrder::SetRoute(waypoints) => {
                        let mut current = task_force.hex;
                        let mut goes_somewhere = false;
                        for waypoint in waypoints {
                            if !self.map.contains(*waypoint) {
                                return Err(RejectReason::WaypointOffMap(*waypoint));
                            }
                            for step in current.steps_to(*waypoint) {
                                if !self.map.is_navigable(step) {
                                    return Err(RejectReason::RouteBlocked(step));
                                }
                                goes_somewhere = true;
                            }
                            current = *waypoint;
                        }
                        if !goes_somewhere {
                            return Err(RejectReason::EmptyRoute);
                        }
                        Ok(())
                    }
                    TaskForceOrder::SetSpeed(_) | TaskForceOrder::SetMission(_) => Ok(()),
                }
            }
        }
    }

    /// Makes an accepted order the standing one. It stays in force until
    /// another order replaces it.
    fn apply_order(&mut self, order: &Order) {
        match &order.kind {
            OrderKind::TaskForce {
                target,
                order: task_force_order,
            } => {
                let task_force = self
                    .truth
                    .task_forces
                    .get_mut(target)
                    .expect("order was validated against this task force");
                match task_force_order {
                    TaskForceOrder::SetRoute(waypoints) => {
                        task_force.route = Route {
                            waypoints: waypoints.clone(),
                            next_index: 0,
                        };
                        task_force.status = TaskForceStatus::Underway;
                        task_force.route_order = Some(order.id);
                    }
                    TaskForceOrder::SetSpeed(speed) => {
                        task_force.speed = *speed;
                        task_force.speed_order = Some(order.id);
                    }
                    TaskForceOrder::SetMission(mission) => {
                        task_force.mission = *mission;
                    }
                }
            }
        }
    }

    fn move_task_forces(&mut self, day: u32, pulse: Pulse) {
        let ids: Vec<TaskForceId> = self.truth.task_forces.keys().copied().collect();
        for id in ids {
            self.move_task_force(id, day, pulse);
        }
    }

    /// Moves one task force along its route for one pulse.
    ///
    /// The task force steams at the speed of its slowest ship for the whole
    /// pulse, unless it reaches the end of its route or a ship runs dry
    /// first. It enters the next hex once it has covered the full 40 nm
    /// between hex centres.
    fn move_task_force(&mut self, id: TaskForceId, day: u32, pulse: Pulse) {
        let Some(task_force) = self.truth.task_forces.get(&id) else {
            return;
        };
        if task_force.status != TaskForceStatus::Underway {
            return;
        }

        let steamers: Vec<Steamer> = task_force
            .ships
            .iter()
            .map(|ship_id| {
                let ship = &self.truth.ships[ship_id];
                let class = &self.ref_data.ship_classes[&ship.class];
                Steamer {
                    ship: *ship_id,
                    fuel_t: ship.fuel_t,
                    setting_speed_kn: match task_force.speed {
                        SpeedSetting::Cruise => class.cruise_speed_kn,
                        SpeedSetting::Full => class.max_speed_kn,
                    },
                    cruise_speed_kn: class.cruise_speed_kn,
                    fuel_capacity_t: class.fuel_capacity_t,
                    endurance_nm: class.endurance_nm,
                }
            })
            .collect();
        let Some(speed_kn) = steamers.iter().map(|s| s.setting_speed_kn).min() else {
            // A task force with no ships goes nowhere.
            return;
        };

        let time_limit_nm = speed_kn * pulse.hours();
        let fuel_limit_nm = steamers.iter().filter_map(|s| s.range_nm(speed_kn)).min();
        let budget_nm = match fuel_limit_nm {
            Some(limit) => time_limit_nm.min(limit),
            None => time_limit_nm,
        };
        let fuel_is_the_limit = budget_nm < time_limit_nm;

        let one_hex = Fixed::from_int(HEX_NM);
        let waypoints = task_force.route.waypoints.clone();
        let from = task_force.hex;
        let mut hex = from;
        let mut progress_nm = task_force.progress_nm;
        let mut next_index = task_force.route.next_index;
        let mut travelled_nm = Fixed::ZERO;
        let mut hexes_entered = Vec::new();

        while next_index < waypoints.len() {
            let target = waypoints[next_index];
            if hex == target {
                next_index += 1;
                continue;
            }
            let remaining_nm = budget_nm - travelled_nm;
            if remaining_nm.is_zero() {
                break;
            }
            let to_next_hex_nm = one_hex - progress_nm;
            if remaining_nm >= to_next_hex_nm {
                travelled_nm += to_next_hex_nm;
                progress_nm = Fixed::ZERO;
                hex = hex.step_toward(target);
                hexes_entered.push(hex);
            } else {
                travelled_nm += remaining_nm;
                progress_nm += remaining_nm;
            }
        }

        let arrived = next_index >= waypoints.len();
        let ran_dry = !arrived && fuel_is_the_limit;
        let status = if arrived {
            TaskForceStatus::Arrived
        } else if ran_dry {
            TaskForceStatus::OutOfFuel
        } else {
            TaskForceStatus::Underway
        };

        let standing_orders: Vec<Cause> = [task_force.route_order, task_force.speed_order]
            .into_iter()
            .flatten()
            .map(Cause::Order)
            .collect();

        let task_force = self
            .truth
            .task_forces
            .get_mut(&id)
            .expect("task force exists");
        task_force.hex = hex;
        task_force.progress_nm = progress_nm;
        task_force.route.next_index = next_index;
        task_force.status = status;

        let mut steamed: Option<EventId> = None;
        if !travelled_nm.is_zero() {
            let breakdown = self.config.record_breakdowns.then(|| {
                let mut factors = vec![
                    Factor {
                        name: "speed_kn",
                        value: speed_kn,
                    },
                    Factor {
                        name: "pulse_hours",
                        value: Fixed::from_int(pulse.hours()),
                    },
                    Factor {
                        name: "time_limit_nm",
                        value: time_limit_nm,
                    },
                ];
                if let Some(limit) = fuel_limit_nm {
                    factors.push(Factor {
                        name: "fuel_limit_nm",
                        value: limit,
                    });
                }
                Breakdown {
                    factors,
                    result: travelled_nm,
                }
            });
            let event = self.events.record(
                day,
                Some(pulse),
                EventKind::TaskForceSteamed {
                    task_force: id,
                    from,
                    to: hex,
                    distance_nm: travelled_nm,
                    speed_kn,
                    hexes_entered,
                },
                standing_orders.clone(),
                breakdown,
            );
            steamed = Some(event);

            for steamer in &steamers {
                // The fuel limit and the burn round separately, so cap the
                // burn at what is aboard. Fuel never goes negative.
                let burned_t = steamer.fuel_for(travelled_nm, speed_kn).min(steamer.fuel_t);
                let remaining_t = steamer.fuel_t - burned_t;
                self.truth
                    .ships
                    .get_mut(&steamer.ship)
                    .expect("ship exists")
                    .fuel_t = remaining_t;

                let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                    factors: vec![
                        Factor {
                            name: "distance_nm",
                            value: travelled_nm,
                        },
                        Factor {
                            name: "fuel_capacity_t",
                            value: steamer.fuel_capacity_t,
                        },
                        Factor {
                            name: "endurance_nm",
                            value: steamer.endurance_nm,
                        },
                        Factor {
                            name: "speed_kn",
                            value: speed_kn,
                        },
                        Factor {
                            name: "cruise_speed_kn",
                            value: steamer.cruise_speed_kn,
                        },
                        Factor {
                            name: "speed_multiplier",
                            value: speed_multiplier(speed_kn, steamer.cruise_speed_kn),
                        },
                    ],
                    result: burned_t,
                });
                self.events.record(
                    day,
                    Some(pulse),
                    EventKind::FuelBurned {
                        ship: steamer.ship,
                        task_force: id,
                        burned_t,
                        remaining_t,
                    },
                    vec![Cause::Event(event)],
                    breakdown,
                );
            }
        }

        let outcome_causes = match steamed {
            Some(event) => vec![Cause::Event(event)],
            None => standing_orders,
        };
        if arrived {
            self.events.record(
                day,
                Some(pulse),
                EventKind::TaskForceArrived {
                    task_force: id,
                    hex,
                    elapsed_hours: pulse_start(day, pulse) + travelled_nm / speed_kn,
                },
                outcome_causes,
                None,
            );
        } else if ran_dry {
            self.events.record(
                day,
                Some(pulse),
                EventKind::TaskForceOutOfFuel {
                    task_force: id,
                    hex,
                },
                outcome_causes,
                None,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hex::Hex;
    use crate::ids::{BaseId, ShipClassId};
    use crate::map::{HexData, HexKind, Map};
    use crate::orders::{Mission, OrderRequest, Origin};
    use crate::refdata::{RefData, ShipClass, ShipKind};
    use crate::state::{Base, Ship, ShipLocation, Side, TaskForce, TruthState};

    const TF: TaskForceId = TaskForceId::new(1);
    const SHIP: ShipId = ShipId::new(1);

    /// A strip of open sea with one land hex at (3, 2), and a task force of
    /// one ship at the origin. The class burns 0.25 t per nm at 16 kn.
    fn world_with(seed: u64, fuel_t: i64) -> World {
        let mut map = Map::new();
        for q in -1..=12 {
            for r in -1..=5 {
                map.insert(Hex::new(q, r), HexData::open_sea());
            }
        }
        map.insert(
            Hex::new(3, 2),
            HexData {
                kind: HexKind::Land,
                ..HexData::open_sea()
            },
        );

        let class = ShipClass {
            id: ShipClassId::new(1),
            name: "Test class".to_string(),
            kind: ShipKind::Cruiser,
            cruise_speed_kn: Fixed::from_int(16),
            max_speed_kn: Fixed::from_int(32),
            fuel_capacity_t: Fixed::from_int(2000),
            endurance_nm: Fixed::from_int(8000),
        };
        let mut ref_data = RefData::default();
        ref_data.ship_classes.insert(class.id, class);

        let mut truth = TruthState::default();
        let mut task_force = TaskForce::at(
            TF,
            "Test force".to_string(),
            Side::Allied,
            Hex::new(0, 0),
            Mission::Transport,
        );
        task_force.ships.insert(SHIP);
        truth.task_forces.insert(TF, task_force);
        truth.ships.insert(
            SHIP,
            Ship {
                id: SHIP,
                name: "Test ship".to_string(),
                class: ShipClassId::new(1),
                side: Side::Allied,
                fuel_t: Fixed::from_int(fuel_t),
                location: ShipLocation::TaskForce(TF),
            },
        );
        truth.bases.insert(
            BaseId::new(1),
            Base {
                id: BaseId::new(1),
                name: "Test base".to_string(),
                side: Side::Allied,
                hex: Hex::new(0, 0),
            },
        );

        World::new(seed, ref_data, map, truth).expect("valid setup")
    }

    fn world_with_fuel(fuel_t: i64) -> World {
        world_with(7, fuel_t)
    }

    fn order(order: TaskForceOrder) -> OrderRequest {
        OrderRequest {
            kind: OrderKind::TaskForce { target: TF, order },
            origin: Origin::PLAYER,
        }
    }

    fn route_to(q: i32, r: i32) -> OrderRequest {
        order(TaskForceOrder::SetRoute(vec![Hex::new(q, r)]))
    }

    fn rejection(world: &World) -> Option<RejectReason> {
        world.events().iter().find_map(|event| match &event.kind {
            EventKind::OrderRejected { reason, .. } => Some(reason.clone()),
            _ => None,
        })
    }

    #[test]
    fn full_speed_burns_four_times_the_fuel_per_mile() {
        let mut world = world_with_fuel(2000);
        world.submit_orders(
            Side::Allied,
            vec![
                route_to(12, 0),
                order(TaskForceOrder::SetSpeed(SpeedSetting::Full)),
            ],
        );
        world.resolve_day().expect("day resolves");

        // Hand calculation: 32 kn is twice cruise, so the multiplier is 4.
        // The route is 12 hexes = 480 nm, reached in 15 hours.
        // Fuel: 480 nm x 0.25 t/nm x 4 = 480 t.
        let truth = world.umpire();
        assert_eq!(truth.task_forces[&TF].status, TaskForceStatus::Arrived);
        assert_eq!(truth.task_forces[&TF].hex, Hex::new(12, 0));
        assert_eq!(truth.ships[&SHIP].fuel_t, Fixed::from_int(2000 - 480));
    }

    #[test]
    fn a_task_force_stops_when_a_ship_runs_dry() {
        // 10 t at 0.25 t/nm lasts exactly 40 nm: one hex.
        let mut world = world_with_fuel(10);
        world.submit_orders(Side::Allied, vec![route_to(12, 0)]);
        world.resolve_day().expect("day resolves");

        let truth = world.umpire();
        let task_force = &truth.task_forces[&TF];
        assert_eq!(task_force.status, TaskForceStatus::OutOfFuel);
        assert_eq!(task_force.hex, Hex::new(1, 0));
        assert_eq!(task_force.progress_nm, Fixed::ZERO);
        assert_eq!(truth.ships[&SHIP].fuel_t, Fixed::ZERO);

        let ran_dry = world
            .events()
            .iter()
            .filter(|event| matches!(event.kind, EventKind::TaskForceOutOfFuel { .. }))
            .count();
        assert_eq!(ran_dry, 1);

        // It stays stopped and burns nothing more.
        let events_before = world.events().len();
        world.resolve_day().expect("day resolves");
        assert_eq!(world.events().len(), events_before);
    }

    #[test]
    fn orders_for_the_other_sides_units_are_rejected() {
        let mut world = world_with_fuel(2000);
        world.submit_orders(Side::Japanese, vec![route_to(5, 0)]);
        world.resolve_day().expect("day resolves");

        assert_eq!(rejection(&world), Some(RejectReason::NotYourUnit));
        assert_eq!(
            world.umpire().task_forces[&TF].status,
            TaskForceStatus::Idle
        );
    }

    #[test]
    fn orders_for_unknown_units_are_rejected() {
        let mut world = world_with_fuel(2000);
        world.submit_orders(
            Side::Allied,
            vec![OrderRequest {
                kind: OrderKind::TaskForce {
                    target: TaskForceId::new(99),
                    order: TaskForceOrder::SetSpeed(SpeedSetting::Full),
                },
                origin: Origin::PLAYER,
            }],
        );
        world.resolve_day().expect("day resolves");
        assert_eq!(rejection(&world), Some(RejectReason::UnknownTarget));
    }

    #[test]
    fn routes_off_the_map_or_through_land_are_rejected() {
        let mut world = world_with_fuel(2000);
        world.submit_orders(Side::Allied, vec![route_to(40, 40)]);
        world.resolve_day().expect("day resolves");
        assert_eq!(
            rejection(&world),
            Some(RejectReason::WaypointOffMap(Hex::new(40, 40)))
        );

        // (0,0) to (3,3) steps along q to (3,0), then up r through (3,2).
        let mut world = world_with_fuel(2000);
        world.submit_orders(Side::Allied, vec![route_to(3, 3)]);
        world.resolve_day().expect("day resolves");
        assert_eq!(
            rejection(&world),
            Some(RejectReason::RouteBlocked(Hex::new(3, 2)))
        );

        let mut world = world_with_fuel(2000);
        world.submit_orders(Side::Allied, vec![route_to(0, 0)]);
        world.resolve_day().expect("day resolves");
        assert_eq!(rejection(&world), Some(RejectReason::EmptyRoute));
    }

    #[test]
    fn a_rejected_order_changes_nothing_but_the_log() {
        let mut quiet = world_with_fuel(2000);
        quiet.resolve_day().expect("day resolves");

        let mut noisy = world_with_fuel(2000);
        noisy.submit_orders(Side::Allied, vec![route_to(40, 40)]);
        noisy.resolve_day().expect("day resolves");

        assert_eq!(quiet.umpire(), noisy.umpire());
        // The hash still differs: the order and its rejection are recorded.
        assert_ne!(quiet.state_hash(), noisy.state_hash());
    }

    #[test]
    fn breakdown_recording_does_not_change_the_state_hash() {
        let run = |record_breakdowns: bool| {
            let mut world = world_with_fuel(2000);
            world.set_config(crate::SimConfig { record_breakdowns });
            world.submit_orders(Side::Allied, vec![route_to(12, 0)]);
            world.resolve_day().expect("day resolves");
            let has_breakdowns = world.events().iter().any(|event| event.breakdown.is_some());
            (world.state_hash(), has_breakdowns)
        };
        let (with_hash, with_breakdowns) = run(true);
        let (without_hash, without_breakdowns) = run(false);
        assert!(with_breakdowns);
        assert!(!without_breakdowns);
        assert_eq!(with_hash, without_hash);
    }

    #[test]
    fn the_seed_is_part_of_the_state_hash() {
        // Nothing here is random, so two seeds give the same outcome. The
        // hashes must still differ, because the seed decides what happens
        // later.
        let run = |seed: u64| {
            let mut world = world_with(seed, 2000);
            world.submit_orders(Side::Allied, vec![route_to(12, 0)]);
            world.resolve_day().expect("day resolves");
            world
        };
        let (a, b) = (run(1), run(2));
        assert_eq!(a.umpire(), b.umpire());
        assert_ne!(a.state_hash(), b.state_hash());
    }

    #[test]
    fn a_new_route_replaces_the_standing_one() {
        let mut world = world_with_fuel(2000);
        world.submit_orders(Side::Allied, vec![route_to(12, 0)]);
        world.resolve_day().expect("day resolves");
        // 16 kn for 24 h = 384 nm: nine hexes and 24 nm.
        assert_eq!(world.umpire().task_forces[&TF].hex, Hex::new(9, 0));

        world.submit_orders(Side::Allied, vec![route_to(9, 2)]);
        world.resolve_day().expect("day resolves");
        let task_force = &world.umpire().task_forces[&TF];
        assert_eq!(task_force.status, TaskForceStatus::Arrived);
        assert_eq!(task_force.hex, Hex::new(9, 2));
    }
}
