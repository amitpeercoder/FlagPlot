//! Resolving one day.
//!
//! The order of phases below is the one in the spec ("Daily resolution
//! order"). Orders intake, task force movement, cargo handling, air search
//! (`search.rs`), contact ageing at the end of every pulse and closing the
//! day do something so far. The other phases are listed so the sequence is
//! fixed from the start, and each is filled in by the scenario that needs it.

use crate::events::{Breakdown, Cause, EventKind, Factor};
use crate::fixed::Fixed;
use crate::hex::HEX_NM;
use crate::ids::{EventId, ShipId, TaskForceId};
use crate::invariants;
use crate::orders::{
    AirGroupOrder, CargoAction, Order, OrderKind, RejectReason, SpeedSetting, TaskForceOrder,
};
use crate::state::{CargoOrder, Route, SearchArc, Side, TaskForceStatus};
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

/// Fuel burn per mile rises with the square of speed relative to cruise, and
/// never falls below the cruise rate: at cruise speed or slower the
/// multiplier is 1 (the floor, decision 3 of
/// `docs/decisions/0006-fuel-model.md`). This is a placeholder model.
fn speed_multiplier(speed_kn: Fixed, cruise_speed_kn: Fixed) -> Fixed {
    if speed_kn <= cruise_speed_kn {
        return Fixed::ONE;
    }
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
            self.age_contacts_at_end_of_pulse(day, pulse);
        }
        for phase in END_OF_DAY_SEQUENCE {
            self.run_end_of_day_phase(day, phase);
        }

        self.truth.day = day;

        let violations = invariants::check(&self.truth, &self.map, &self.ref_data);
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
            PulsePhase::Loading => self.handle_cargo(day, pulse),
            PulsePhase::AirSearch => self.search_from_the_air(day, pulse),
            // Not implemented yet. Each arrives with the scenario that needs it.
            // Night search is decision 7 territory: there is no search where it
            // is dark, so this phase stays empty.
            PulsePhase::NightSearch
            | PulsePhase::StrikeDecisions
            | PulsePhase::AirCombat
            | PulsePhase::SurfaceEngagements
            | PulsePhase::ShoreBombardment => {}
        }
    }

    fn run_end_of_day_phase(&mut self, _day: u32, phase: EndOfDayPhase) {
        match phase {
            // Not implemented yet. Each arrives with the scenario that needs it.
            // IntelligenceUpdate stays empty on purpose: contacts age at the end
            // of every pulse and reports merge on arrival, which covers "age
            // contacts, merge duplicates" (decision 15 of
            // `docs/decisions/0012-search-and-contacts.md`).
            EndOfDayPhase::GroundCombat
            | EndOfDayPhase::Logistics
            | EndOfDayPhase::AirGroupUpkeep
            | EndOfDayPhase::IntelligenceUpdate => {}
        }
    }

    /// Ages each side's contacts after the pulse's last phase, night
    /// included (decision 14 of `docs/decisions/0012-search-and-contacts.md`).
    /// Each aged or dropped contact is an event citing the report that last
    /// refreshed it.
    fn age_contacts_at_end_of_pulse(&mut self, day: u32, pulse: Pulse) {
        // With no search figures there are no air groups, so no contacts.
        let Some(figures) = self.ref_data.search.clone() else {
            return;
        };
        for side in Side::ALL {
            let outcomes = self
                .beliefs
                .get_mut(&side)
                .expect("both sides have a belief state")
                .age_contacts(&figures);
            for outcome in outcomes {
                let kind = if outcome.dropped {
                    EventKind::ContactDropped {
                        side,
                        contact: outcome.contact,
                        age_pulses: outcome.age_pulses,
                        confidence: outcome.confidence,
                    }
                } else {
                    EventKind::ContactAged {
                        side,
                        contact: outcome.contact,
                        age_pulses: outcome.age_pulses,
                        confidence: outcome.confidence,
                    }
                };
                self.events.record(
                    day,
                    Some(pulse),
                    kind,
                    vec![Cause::Event(outcome.last_report_event)],
                    None,
                );
            }
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
                    .ok_or(RejectReason::NotYourUnit)?;
                if task_force.side != order.side {
                    return Err(RejectReason::NotYourUnit);
                }
                match task_force_order {
                    TaskForceOrder::SetRoute(waypoints) => {
                        // The new route starts from the centre the force
                        // reaches first: the same choice apply_order makes.
                        let (_, mut current) =
                            task_force.reroute_centre(waypoints.first().copied());
                        // A force between centres always has somewhere to go:
                        // the leg it must finish. So "stop at the hex ahead"
                        // and "go back to the hex I left" are real routes.
                        let mut goes_somewhere =
                            task_force.toward_hex.is_some() && !waypoints.is_empty();
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
                    TaskForceOrder::Load => {
                        if task_force.status == TaskForceStatus::Underway {
                            return Err(RejectReason::UnderWay);
                        }
                        let own_base_here = self.truth.bases.values().any(|base| {
                            base.side == order.side && base.hex == task_force.position()
                        });
                        if own_base_here {
                            Ok(())
                        } else {
                            Err(RejectReason::NoOwnBaseHere)
                        }
                    }
                    TaskForceOrder::SetSpeed(_)
                    | TaskForceOrder::SetMission(_)
                    | TaskForceOrder::Unload => Ok(()),
                }
            }
            OrderKind::AirGroup {
                target,
                order: air_group_order,
            } => {
                let group = self
                    .truth
                    .air_groups
                    .get(target)
                    .ok_or(RejectReason::NotYourUnit)?;
                if group.side != order.side {
                    return Err(RejectReason::NotYourUnit);
                }
                match air_group_order {
                    AirGroupOrder::SetAirMission(_) => Ok(()),
                    AirGroupOrder::SetSearchArc {
                        from_deg,
                        to_deg,
                        range_nm,
                    } => {
                        for bearing in [*from_deg, *to_deg] {
                            if !(0..=350).contains(&bearing) || bearing % 10 != 0 {
                                return Err(RejectReason::InvalidBearing(bearing));
                            }
                        }
                        // World::new checked that every group's type exists.
                        let max_nm = self
                            .ref_data
                            .aircraft_types
                            .get(&group.aircraft_type)
                            .expect("air group has a known aircraft type")
                            .search_range_nm;
                        if *range_nm <= Fixed::ZERO || *range_nm > max_nm {
                            return Err(RejectReason::InvalidSearchRange {
                                range_nm: *range_nm,
                                max_nm,
                            });
                        }
                        Ok(())
                    }
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
                        // Turning back is decided on the state the order
                        // found, and the leg is then finished by movement.
                        let (turns_back, _) = task_force.reroute_centre(waypoints.first().copied());
                        if turns_back {
                            task_force.turn_back();
                        }
                        task_force.route = Route {
                            waypoints: waypoints.clone(),
                            next_index: 0,
                        };
                        task_force.status = TaskForceStatus::Underway;
                        task_force.route_order = Some(order.id);
                        task_force.arrived_at_hours = None;
                        task_force.arrival_event = None;
                    }
                    TaskForceOrder::SetSpeed(speed) => {
                        task_force.speed = *speed;
                        task_force.speed_order = Some(order.id);
                    }
                    TaskForceOrder::SetMission(mission) => {
                        task_force.mission = *mission;
                    }
                    TaskForceOrder::Load => {
                        task_force.cargo_order = Some(CargoOrder {
                            action: CargoAction::Load,
                            order: order.id,
                        });
                    }
                    TaskForceOrder::Unload => {
                        task_force.cargo_order = Some(CargoOrder {
                            action: CargoAction::Unload,
                            order: order.id,
                        });
                    }
                }
            }
            OrderKind::AirGroup {
                target,
                order: air_group_order,
            } => {
                let group = self
                    .truth
                    .air_groups
                    .get_mut(target)
                    .expect("order was validated against this air group");
                match air_group_order {
                    AirGroupOrder::SetAirMission(mission) => {
                        group.mission = *mission;
                        group.mission_order = Some(order.id);
                    }
                    AirGroupOrder::SetSearchArc {
                        from_deg,
                        to_deg,
                        range_nm,
                    } => {
                        group.search = Some(SearchArc {
                            from_deg: u32::try_from(*from_deg).expect("bearing was validated"),
                            to_deg: u32::try_from(*to_deg).expect("bearing was validated"),
                            range_nm: *range_nm,
                        });
                        group.arc_order = Some(order.id);
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
    /// first. A leg between two hex centres is 40 nm and, once started, is
    /// always finished: the next centre reached is the hex ahead, whatever
    /// the route says. At a centre the next leg goes to the neighbour nearest
    /// the current waypoint. The task force enters a hex (its position
    /// changes) when it passes the midpoint of the leg, 20 nm along, so
    /// `hexes_entered` and the `from` and `to` of the event are positions,
    /// not centres reached.
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
        let half_hex = one_hex / Fixed::from_int(2);
        let waypoints = task_force.route.waypoints.clone();
        let from = task_force.position();
        let mut from_hex = task_force.from_hex;
        let mut toward_hex = task_force.toward_hex;
        let mut progress_nm = task_force.progress_nm;
        let mut next_index = task_force.route.next_index;
        let mut travelled_nm = Fixed::ZERO;
        let mut hexes_entered = Vec::new();

        loop {
            if toward_hex.is_none() {
                // At a centre: advance past a waypoint reached, or stop.
                let Some(target) = waypoints.get(next_index) else {
                    break;
                };
                if from_hex == *target {
                    next_index += 1;
                    continue;
                }
            }
            let remaining_nm = budget_nm - travelled_nm;
            if remaining_nm.is_zero() {
                break;
            }
            let leg_end = match toward_hex {
                Some(toward) => toward,
                None => {
                    // Only start a leg with distance left to steam it, so
                    // zero progress always means no hex ahead.
                    let target = waypoints[next_index];
                    let toward = from_hex.step_toward(target);
                    toward_hex = Some(toward);
                    toward
                }
            };
            let to_centre_nm = one_hex - progress_nm;
            if remaining_nm >= to_centre_nm {
                travelled_nm += to_centre_nm;
                if progress_nm < half_hex {
                    hexes_entered.push(leg_end);
                }
                from_hex = leg_end;
                toward_hex = None;
                progress_nm = Fixed::ZERO;
            } else {
                travelled_nm += remaining_nm;
                if progress_nm < half_hex && progress_nm + remaining_nm >= half_hex {
                    hexes_entered.push(leg_end);
                }
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
        task_force.from_hex = from_hex;
        task_force.toward_hex = toward_hex;
        task_force.progress_nm = progress_nm;
        let hex = task_force.position();
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
            let elapsed_hours = pulse_start(day, pulse) + travelled_nm / speed_kn;
            let event = self.events.record(
                day,
                Some(pulse),
                EventKind::TaskForceArrived {
                    task_force: id,
                    hex,
                    elapsed_hours,
                },
                outcome_causes,
                None,
            );
            let task_force = self
                .truth
                .task_forces
                .get_mut(&id)
                .expect("task force exists");
            task_force.arrived_at_hours = Some(elapsed_hours);
            task_force.arrival_event = Some(event);
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
    use crate::ids::{AirGroupId, AircraftTypeId, BaseId, ShipClassId};
    use crate::map::{HexData, HexKind, Map};
    use crate::orders::{AirMission, Mission, OrderRequest, Origin};
    use crate::refdata::{
        AircraftRole, AircraftType, PortSize, RefData, SearchFigures, ShipClass, ShipKind,
    };
    use crate::state::{AirGroup, Base, Ship, ShipLocation, Side, TaskForce, TruthState};

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
            cargo_capacity_t: Fixed::ZERO,
        };
        let mut ref_data = RefData::default();
        ref_data.ship_classes.insert(class.id, class);
        // Test-only figures: the test base sits in open sea, port size 0.
        ref_data.port_sizes.insert(
            0,
            PortSize {
                berths: 0,
                berth_rate_t_per_h: Fixed::ZERO,
                lighter_rate_t_per_h: Fixed::from_int(20),
            },
        );

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
                supply_t: Fixed::ZERO,
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
                supply_t: Fixed::ZERO,
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
        assert_eq!(truth.task_forces[&TF].from_hex, Hex::new(12, 0));
        assert_eq!(truth.task_forces[&TF].toward_hex, None);
        assert_eq!(truth.task_forces[&TF].progress_nm, Fixed::ZERO);
        assert_eq!(truth.task_forces[&TF].position(), Hex::new(12, 0));
        assert_eq!(truth.ships[&SHIP].fuel_t, Fixed::from_int(2000 - 480));
    }

    #[test]
    fn slow_steaming_burns_the_cruise_rate_per_mile() {
        let cruise = Fixed::from_int(16);
        assert_eq!(speed_multiplier(Fixed::from_int(8), cruise), Fixed::ONE);
        assert_eq!(speed_multiplier(cruise, cruise), Fixed::ONE);
        assert_eq!(
            speed_multiplier(Fixed::from_int(32), cruise),
            Fixed::from_int(4)
        );

        // End to end: a second ship whose cruise speed is 32 kn rides in a
        // task force held to 16 kn by the first. It steams below its own
        // cruise speed, so it burns exactly its cruise rate per mile:
        // 8 hexes = 320 nm at 2000/8000 = 0.25 t/nm is 80 t, the same as the
        // slow ship, not less.
        let mut world = world_with_fuel(2000);
        let mut fast = world.ref_data.ship_classes[&ShipClassId::new(1)].clone();
        fast.id = ShipClassId::new(2);
        fast.cruise_speed_kn = Fixed::from_int(32);
        fast.max_speed_kn = Fixed::from_int(36);
        world.ref_data.ship_classes.insert(fast.id, fast);
        let fast_ship = ShipId::new(2);
        world.truth.ships.insert(
            fast_ship,
            Ship {
                id: fast_ship,
                name: "Fast ship".to_string(),
                class: ShipClassId::new(2),
                side: Side::Allied,
                fuel_t: Fixed::from_int(2000),
                supply_t: Fixed::ZERO,
                location: ShipLocation::TaskForce(TF),
            },
        );
        world
            .truth
            .task_forces
            .get_mut(&TF)
            .expect("task force exists")
            .ships
            .insert(fast_ship);
        world.submit_orders(Side::Allied, vec![route_to(8, 0)]);
        world.resolve_day().expect("day resolves");

        let truth = world.umpire();
        assert_eq!(truth.task_forces[&TF].status, TaskForceStatus::Arrived);
        assert_eq!(truth.ships[&SHIP].fuel_t, Fixed::from_int(2000 - 80));
        assert_eq!(truth.ships[&fast_ship].fuel_t, Fixed::from_int(2000 - 80));
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
        assert_eq!(task_force.from_hex, Hex::new(1, 0));
        assert_eq!(task_force.toward_hex, None);
        assert_eq!(task_force.position(), Hex::new(1, 0));
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
        assert_eq!(rejection(&world), Some(RejectReason::NotYourUnit));
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
        // 16 kn for 24 h = 384 nm: nine hexes and 24 nm. It has left (9,0),
        // is 24 nm toward (10,0) and so is in (10,0).
        let task_force = &world.umpire().task_forces[&TF];
        assert_eq!(task_force.from_hex, Hex::new(9, 0));
        assert_eq!(task_force.toward_hex, Some(Hex::new(10, 0)));
        assert_eq!(task_force.progress_nm, Fixed::from_int(24));
        assert_eq!(task_force.position(), Hex::new(10, 0));

        world.submit_orders(Side::Allied, vec![route_to(9, 2)]);
        world.resolve_day().expect("day resolves");
        let task_force = &world.umpire().task_forces[&TF];
        // Going on costs 16 + 40 x 2 = 96 nm, turning back 24 + 40 x 2 =
        // 104 nm, so it finishes the leg to (10,0) and goes on to (9,2).
        assert_eq!(task_force.status, TaskForceStatus::Arrived);
        assert_eq!(task_force.from_hex, Hex::new(9, 2));
        assert_eq!(task_force.toward_hex, None);
        assert_eq!(task_force.position(), Hex::new(9, 2));
    }

    #[test]
    fn load_needs_a_stopped_task_force_at_an_own_base() {
        // At the base and idle: accepted, and it becomes the cargo order
        // without touching the route or speed.
        let mut world = world_with_fuel(2000);
        world.submit_orders(Side::Allied, vec![order(TaskForceOrder::Load)]);
        world.resolve_day().expect("day resolves");
        assert_eq!(rejection(&world), None);
        let task_force = &world.umpire().task_forces[&TF];
        let cargo = task_force.cargo_order.expect("cargo order is set");
        assert_eq!(cargo.action, CargoAction::Load);
        assert_eq!(task_force.status, TaskForceStatus::Idle);
        assert_eq!(task_force.route_order, None);

        // Under way: rejected, even before the base question.
        let mut world = world_with_fuel(2000);
        world.submit_orders(Side::Allied, vec![route_to(12, 0)]);
        world.resolve_day().expect("day resolves");
        world.submit_orders(Side::Allied, vec![order(TaskForceOrder::Load)]);
        world.resolve_day().expect("day resolves");
        // 480 nm at 16 kn takes 30 hours, so it is still under way.
        assert_eq!(rejection(&world), Some(RejectReason::UnderWay));

        // Stopped away from any base: rejected.
        let mut world = world_with_fuel(2000);
        world.submit_orders(Side::Allied, vec![route_to(2, 0)]);
        world.resolve_day().expect("day resolves");
        world.submit_orders(Side::Allied, vec![order(TaskForceOrder::Load)]);
        world.resolve_day().expect("day resolves");
        assert_eq!(rejection(&world), Some(RejectReason::NoOwnBaseHere));
    }

    #[test]
    fn unload_is_always_accepted_and_replaces_load() {
        let mut world = world_with_fuel(2000);
        world.submit_orders(
            Side::Allied,
            vec![
                order(TaskForceOrder::Load),
                route_to(2, 0),
                order(TaskForceOrder::Unload),
            ],
        );
        world.resolve_day().expect("day resolves");
        assert_eq!(rejection(&world), None);
        let task_force = &world.umpire().task_forces[&TF];
        assert_eq!(
            task_force.cargo_order.map(|cargo| cargo.action),
            Some(CargoAction::Unload)
        );
        assert!(task_force.route_order.is_some());
    }

    #[test]
    fn arrival_is_recorded_and_a_new_route_clears_it() {
        let mut world = world_with_fuel(2000);
        world.submit_orders(Side::Allied, vec![route_to(2, 0)]);
        world.resolve_day().expect("day resolves");
        // 80 nm at 16 kn takes 5 hours.
        let task_force = &world.umpire().task_forces[&TF];
        assert_eq!(task_force.arrived_at_hours, Some(Fixed::from_int(5)));
        let event = task_force.arrival_event.expect("arrival event recorded");
        assert!(matches!(
            world.events().get(event).map(|e| &e.kind),
            Some(EventKind::TaskForceArrived { .. })
        ));

        world.submit_orders(Side::Allied, vec![route_to(2, 2)]);
        world.resolve_day().expect("day resolves");
        let task_force = &world.umpire().task_forces[&TF];
        assert_eq!(task_force.status, TaskForceStatus::Arrived);
        // It arrived again, so the fields describe the new arrival.
        assert_ne!(task_force.arrival_event, Some(event));
    }

    // ---- Air group orders (decision 0012) ----

    const GROUP: AirGroupId = AirGroupId::new(1);
    const SEARCH_RANGE_NM: i64 = 600;

    /// The standard test world plus one Allied air group of three aircraft at
    /// the test base. The aircraft type has test-only figures (range 600 nm).
    fn world_with_air_group() -> World {
        let mut world = world_with_fuel(2000);
        let aircraft_type = AircraftType {
            id: AircraftTypeId::new(1),
            name: "Test flying boat".to_string(),
            role: AircraftRole::Patrol,
            search_range_nm: nm(SEARCH_RANGE_NM),
            sweep_width_nm: nm(40),
        };
        world
            .ref_data
            .aircraft_types
            .insert(aircraft_type.id, aircraft_type);
        world.ref_data.search = Some(SearchFigures::from_search_toml());
        world.truth.air_groups.insert(
            GROUP,
            AirGroup {
                id: GROUP,
                name: "Test group".to_string(),
                side: Side::Allied,
                base: BaseId::new(1),
                aircraft_type: AircraftTypeId::new(1),
                ready: 3,
                mission: AirMission::Rest,
                search: None,
                mission_order: None,
                arc_order: None,
            },
        );
        assert_sound(&world);
        world
    }

    fn air_order(target: AirGroupId, order: AirGroupOrder) -> OrderRequest {
        OrderRequest {
            kind: OrderKind::AirGroup { target, order },
            origin: Origin::PLAYER,
        }
    }

    fn arc_order(from_deg: i32, to_deg: i32, range_nm: Fixed) -> OrderRequest {
        air_order(
            GROUP,
            AirGroupOrder::SetSearchArc {
                from_deg,
                to_deg,
                range_nm,
            },
        )
    }

    /// Submits one Allied order and returns the rejection, if any.
    fn air_rejection(world: &mut World, request: OrderRequest) -> Option<RejectReason> {
        world.submit_orders(Side::Allied, vec![request]);
        world.resolve_day().expect("day resolves");
        rejection(world)
    }

    #[test]
    fn an_air_group_order_for_an_unknown_group_is_rejected() {
        let mut world = world_with_air_group();
        let request = air_order(
            AirGroupId::new(99),
            AirGroupOrder::SetAirMission(AirMission::Search),
        );
        assert_eq!(
            air_rejection(&mut world, request),
            Some(RejectReason::NotYourUnit)
        );
    }

    #[test]
    fn an_air_group_order_from_the_other_side_is_rejected() {
        let mut world = world_with_air_group();
        world.submit_orders(
            Side::Japanese,
            vec![air_order(
                GROUP,
                AirGroupOrder::SetAirMission(AirMission::Search),
            )],
        );
        world.resolve_day().expect("day resolves");
        assert_eq!(rejection(&world), Some(RejectReason::NotYourUnit));
        assert_eq!(world.umpire().air_groups[&GROUP].mission, AirMission::Rest);
    }

    #[test]
    fn a_bad_bearing_is_rejected_from_first_then_to() {
        for bad in [-10, 5, 355, 360, 370] {
            let mut world = world_with_air_group();
            assert_eq!(
                air_rejection(&mut world, arc_order(bad, 90, nm(300))),
                Some(RejectReason::InvalidBearing(bad)),
                "from {bad}"
            );
            let mut world = world_with_air_group();
            assert_eq!(
                air_rejection(&mut world, arc_order(0, bad, nm(300))),
                Some(RejectReason::InvalidBearing(bad)),
                "to {bad}"
            );
        }
        // Both bad: the "from" bearing is the one named.
        let mut world = world_with_air_group();
        assert_eq!(
            air_rejection(&mut world, arc_order(15, 25, nm(300))),
            Some(RejectReason::InvalidBearing(15))
        );
        // A bad bearing is checked before a bad range.
        let mut world = world_with_air_group();
        assert_eq!(
            air_rejection(&mut world, arc_order(0, 25, nm(0))),
            Some(RejectReason::InvalidBearing(25))
        );
        assert_eq!(world.umpire().air_groups[&GROUP].search, None);
    }

    #[test]
    fn the_bearing_message_names_the_bearing() {
        assert_eq!(
            RejectReason::InvalidBearing(15).to_string(),
            "bearing 15 is not a multiple of 10 from 0 to 350"
        );
    }

    #[test]
    fn a_search_range_of_zero_below_zero_or_beyond_the_type_is_rejected() {
        for bad in [nm(0), nm(-5), nm(SEARCH_RANGE_NM) + Fixed::from_raw(1)] {
            let mut world = world_with_air_group();
            let rejected = air_rejection(&mut world, arc_order(0, 90, bad));
            assert_eq!(
                rejected,
                Some(RejectReason::InvalidSearchRange {
                    range_nm: bad,
                    max_nm: nm(SEARCH_RANGE_NM),
                })
            );
            assert_eq!(world.umpire().air_groups[&GROUP].search, None);
            assert_eq!(world.umpire().air_groups[&GROUP].arc_order, None);
        }
        let message = RejectReason::InvalidSearchRange {
            range_nm: nm(700),
            max_nm: nm(600),
        }
        .to_string();
        assert!(message.contains("700") && message.contains("600"));
    }

    #[test]
    fn a_search_range_equal_to_the_types_is_accepted_and_stored() {
        let mut world = world_with_air_group();
        let rejected = air_rejection(&mut world, arc_order(270, 30, nm(SEARCH_RANGE_NM)));
        assert_eq!(rejected, None);
        let group = &world.umpire().air_groups[&GROUP];
        assert_eq!(
            group.search,
            Some(SearchArc {
                from_deg: 270,
                to_deg: 30,
                range_nm: nm(SEARCH_RANGE_NM),
            })
        );
        // The arc order is the first order the world numbered.
        assert_eq!(group.arc_order, Some(crate::ids::OrderId::new(1)));
        assert_eq!(group.mission_order, None);
        assert_eq!(group.mission, AirMission::Rest);
    }

    #[test]
    fn a_mission_order_and_an_arc_order_do_not_clear_each_other() {
        let mut world = world_with_air_group();
        world.submit_orders(
            Side::Allied,
            vec![
                arc_order(0, 90, nm(300)),
                air_order(GROUP, AirGroupOrder::SetAirMission(AirMission::Search)),
            ],
        );
        world.resolve_day().expect("day resolves");
        assert_eq!(rejection(&world), None);
        let group = world.umpire().air_groups[&GROUP].clone();
        assert_eq!(group.mission, AirMission::Search);
        assert_eq!(group.mission_order, Some(crate::ids::OrderId::new(2)));
        assert_eq!(group.arc_order, Some(crate::ids::OrderId::new(1)));
        assert!(group.search.is_some());

        // A new arc replaces the arc and its order, and leaves the mission.
        world.submit_orders(Side::Allied, vec![arc_order(100, 200, nm(100))]);
        world.resolve_day().expect("day resolves");
        let group = &world.umpire().air_groups[&GROUP];
        assert_eq!(group.mission, AirMission::Search);
        assert_eq!(group.mission_order, Some(crate::ids::OrderId::new(2)));
        assert_eq!(group.arc_order, Some(crate::ids::OrderId::new(3)));
        assert_eq!(group.search.map(|arc| arc.from_deg), Some(100));

        // A new mission replaces the mission and leaves the arc.
        world.submit_orders(
            Side::Allied,
            vec![air_order(
                GROUP,
                AirGroupOrder::SetAirMission(AirMission::Rest),
            )],
        );
        world.resolve_day().expect("day resolves");
        let group = &world.umpire().air_groups[&GROUP];
        assert_eq!(group.mission, AirMission::Rest);
        assert_eq!(group.mission_order, Some(crate::ids::OrderId::new(4)));
        assert_eq!(group.arc_order, Some(crate::ids::OrderId::new(3)));
        assert_eq!(group.search.map(|arc| arc.to_deg), Some(200));
    }

    #[test]
    fn world_new_rejects_bad_air_group_setups() {
        let good = world_with_air_group();
        let build = |change: &dyn Fn(&mut RefData, &mut TruthState)| {
            let mut ref_data = good.ref_data.clone();
            let mut truth = good.truth.clone();
            change(&mut ref_data, &mut truth);
            World::new(1, ref_data, good.map.clone(), truth)
        };
        assert!(build(&|_, _| {}).is_ok());
        assert!(build(&|r, _| {
            r.aircraft_types
                .get_mut(&AircraftTypeId::new(1))
                .expect("type")
                .sweep_width_nm = Fixed::ZERO;
        })
        .is_err());
        assert!(build(&|r, _| {
            r.search.as_mut().expect("search").notice_chance = nm(2);
        })
        .is_err());
        assert!(build(&|_, t| {
            t.air_groups.get_mut(&GROUP).expect("group").aircraft_type = AircraftTypeId::new(9);
        })
        .is_err());
        assert!(build(&|r, _| r.search = None).is_err());
        // No air groups and no search figures is fine.
        assert!(build(&|r, t| {
            r.search = None;
            t.air_groups.clear();
        })
        .is_ok());
    }

    #[test]
    fn the_new_air_group_and_notice_fields_are_part_of_the_state_hash() {
        let base = world_with_air_group();
        let mut fewer_ready = base.clone();
        fewer_ready
            .truth
            .air_groups
            .get_mut(&GROUP)
            .expect("group")
            .ready = 2;
        assert_ne!(base.state_hash(), fewer_ready.state_hash());

        let mut noticed = base.clone();
        noticed
            .truth
            .task_forces
            .get_mut(&TF)
            .expect("task force")
            .noticed_sighted_at_hours = Some(nm(8));
        assert_ne!(base.state_hash(), noticed.state_hash());
        let mut noticed_later = noticed.clone();
        noticed_later
            .truth
            .task_forces
            .get_mut(&TF)
            .expect("task force")
            .noticed_sighted_at_hours = Some(nm(16));
        assert_ne!(noticed.state_hash(), noticed_later.state_hash());
    }

    // ---- Movement between hex centres (decision 0005, decisions 1 and 5) ----

    fn hx(q: i32, r: i32) -> Hex {
        Hex::new(q, r)
    }

    fn nm(value: i64) -> Fixed {
        Fixed::from_int(value)
    }

    /// Puts the task force part-way along a leg by writing the stored state.
    fn set_leg(world: &mut World, from: Hex, toward: Option<Hex>, progress_nm: Fixed) {
        let task_force = world.truth.task_forces.get_mut(&TF).expect("task force");
        task_force.from_hex = from;
        task_force.toward_hex = toward;
        task_force.progress_nm = progress_nm;
        assert_sound(world);
    }

    fn assert_sound(world: &World) {
        assert_eq!(
            invariants::check(&world.truth, &world.map, &world.ref_data),
            Vec::new()
        );
    }

    fn leg(world: &World) -> (Hex, Option<Hex>, Fixed) {
        let task_force = &world.truth.task_forces[&TF];
        (
            task_force.from_hex,
            task_force.toward_hex,
            task_force.progress_nm,
        )
    }

    /// Submits `orders` and takes them in, without moving anything, so the
    /// state an order leaves behind can be looked at.
    fn take_in(world: &mut World, orders: Vec<OrderRequest>) {
        world.submit_orders(Side::Allied, orders);
        let day = world.truth.day + 1;
        world.intake_orders(day);
    }

    /// Resolves days until the force stops, and returns the miles steamed.
    fn miles_until_stopped(world: &mut World) -> Fixed {
        let first = world.events().len();
        world.resolve_day().expect("day resolves");
        for _ in 0..30 {
            if world.truth.task_forces[&TF].status != TaskForceStatus::Underway {
                break;
            }
            world.resolve_day().expect("day resolves");
        }
        world
            .events()
            .from_index(first)
            .iter()
            .filter_map(|event| match &event.kind {
                EventKind::TaskForceSteamed { distance_nm, .. } => Some(*distance_nm),
                _ => None,
            })
            .fold(Fixed::ZERO, |total, distance| total + distance)
    }

    #[test]
    fn position_flips_to_the_hex_ahead_at_exactly_20_nm() {
        let mut world = world_with_fuel(2000);
        // 20 nm less the smallest step (a millionth) is still nearer (0,0).
        let just_under = nm(20) - Fixed::from_raw(1);
        set_leg(&mut world, hx(0, 0), Some(hx(1, 0)), just_under);
        assert_eq!(world.truth.task_forces[&TF].position(), hx(0, 0));
        set_leg(&mut world, hx(0, 0), Some(hx(1, 0)), nm(20));
        assert_eq!(world.truth.task_forces[&TF].position(), hx(1, 0));
        set_leg(&mut world, hx(0, 0), Some(hx(1, 0)), Fixed::from_raw(1));
        assert_eq!(world.truth.task_forces[&TF].position(), hx(0, 0));
        set_leg(&mut world, hx(0, 0), Some(hx(1, 0)), nm(39));
        assert_eq!(world.truth.task_forces[&TF].position(), hx(1, 0));
    }

    #[test]
    fn a_pulse_enters_the_hex_ahead_at_20_nm_and_not_at_the_centre() {
        // From (0,0), the 12-hour night pulse at 16 kn is 192 nm: four
        // centres (160 nm) and 32 nm of the fifth leg. Hexes entered are at
        // 20, 60, 100, 140 and 180 nm: (1,0) to (5,0). Position (5,0).
        let mut world = world_with_fuel(2000);
        world.submit_orders(Side::Allied, vec![route_to(12, 0)]);
        world.resolve_day().expect("day resolves");
        let night = world
            .events()
            .iter()
            .find_map(|event| match &event.kind {
                EventKind::TaskForceSteamed {
                    from,
                    to,
                    hexes_entered,
                    ..
                } if event.pulse == Some(Pulse::Night) => Some((*from, *to, hexes_entered.clone())),
                _ => None,
            })
            .expect("night movement");
        assert_eq!(
            night,
            (
                hx(0, 0),
                hx(5, 0),
                vec![hx(1, 0), hx(2, 0), hx(3, 0), hx(4, 0), hx(5, 0)]
            )
        );
    }

    #[test]
    fn a_reroute_ahead_finishes_the_leg_then_follows_the_new_route() {
        // From (0,0), 10 nm toward (1,0). New first waypoint (5,0):
        // ahead = (40 - 10) + 40 x 4 = 190, back = 10 + 40 x 5 = 210.
        // Ahead wins. 190 nm at 16 kn is 11.875 h, inside the night pulse.
        let mut world = world_with_fuel(2000);
        set_leg(&mut world, hx(0, 0), Some(hx(1, 0)), nm(10));
        take_in(&mut world, vec![route_to(5, 0)]);
        assert_eq!(leg(&world), (hx(0, 0), Some(hx(1, 0)), nm(10)));

        assert_eq!(miles_until_stopped(&mut world), nm(190));
        assert_eq!(leg(&world), (hx(5, 0), None, Fixed::ZERO));
        assert_eq!(
            world.truth.task_forces[&TF].status,
            TaskForceStatus::Arrived
        );
    }

    #[test]
    fn a_reroute_back_reverses_and_reaches_the_hex_it_left_after_exactly_p_more_miles() {
        // From (0,0), 10 nm toward (1,0). New first waypoint (0,1), one hex
        // from both: ahead = 30 + 40 = 70, back = 10 + 40 = 50. Back wins.
        let mut world = world_with_fuel(2000);
        set_leg(&mut world, hx(0, 0), Some(hx(1, 0)), nm(10));
        take_in(&mut world, vec![route_to(0, 1)]);
        // Swapped: left (1,0), toward (0,0), 40 - 10 = 30 nm along.
        assert_eq!(leg(&world), (hx(1, 0), Some(hx(0, 0)), nm(30)));
        assert_sound(&world);

        // Total: 10 nm to (0,0), then 40 nm to (0,1) = 50 nm.
        assert_eq!(miles_until_stopped(&mut world), nm(50));
        assert_eq!(leg(&world), (hx(0, 1), None, Fixed::ZERO));

        // Exactly 10 more miles reach (0,0), never more: 2.5 t of fuel at
        // 0.25 t/nm is 10 nm, and the force stops dead on the centre.
        let mut world = world_with_fuel(2000);
        world.truth.ships.get_mut(&SHIP).expect("ship").fuel_t = Fixed::from_ratio(5, 2);
        set_leg(&mut world, hx(0, 0), Some(hx(1, 0)), nm(10));
        take_in(&mut world, vec![route_to(0, 1)]);
        assert_eq!(miles_until_stopped(&mut world), nm(10));
        assert_eq!(leg(&world), (hx(0, 0), None, Fixed::ZERO));
        assert_eq!(
            world.truth.task_forces[&TF].status,
            TaskForceStatus::OutOfFuel
        );
    }

    #[test]
    fn a_tie_between_ahead_and_back_goes_ahead() {
        // From (0,0), 20 nm toward (1,0). New first waypoint (0,1) is one
        // hex from both: ahead = 20 + 40 = 60, back = 20 + 40 = 60. Tie.
        let mut world = world_with_fuel(2000);
        set_leg(&mut world, hx(0, 0), Some(hx(1, 0)), nm(20));
        take_in(&mut world, vec![route_to(0, 1)]);
        assert_eq!(leg(&world), (hx(0, 0), Some(hx(1, 0)), nm(20)));
        assert_eq!(miles_until_stopped(&mut world), nm(60));
        assert_eq!(leg(&world), (hx(0, 1), None, Fixed::ZERO));
    }

    #[test]
    fn a_reroute_never_gains_distance() {
        let from = hx(5, 1);
        let toward = hx(6, 1);
        let waypoints = [
            hx(9, 1),
            hx(1, 1),
            hx(5, 4),
            hx(6, -1),
            hx(8, 3),
            hx(2, 4),
            hx(5, 0),
            hx(7, 0),
        ];
        let one_hex = nm(HEX_NM);
        for progress in [1, 10, 19, 20, 21, 39] {
            for waypoint in waypoints {
                let mut world = world_with_fuel(2000);
                set_leg(&mut world, from, Some(toward), nm(progress));
                world.submit_orders(
                    Side::Allied,
                    vec![order(TaskForceOrder::SetRoute(vec![waypoint]))],
                );
                let steamed = miles_until_stopped(&mut world);
                assert_eq!(rejection(&world), None, "p={progress} W={waypoint}");

                // cost ahead = (40 - p) + 40 x d(toward, W)
                // cost back  = p + 40 x d(from, W)
                let ahead = nm(40 - progress) + one_hex * i64::from(toward.distance(waypoint));
                let back = nm(progress) + one_hex * i64::from(from.distance(waypoint));
                assert_eq!(steamed, ahead.min(back), "p={progress} W={waypoint}");

                // Position-independent bound: no shorter than the trip from
                // the nearer of the two centres.
                let bound =
                    one_hex * i64::from(from.distance(waypoint).min(toward.distance(waypoint)));
                assert!(steamed >= bound, "p={progress} W={waypoint}");
                assert_eq!(leg(&world), (waypoint, None, Fixed::ZERO));
            }
        }
    }

    #[test]
    fn turning_back_twice_restores_the_stored_state() {
        // Directly: swap and 40 - p, twice.
        let mut world = world_with_fuel(2000);
        set_leg(&mut world, hx(0, 0), Some(hx(1, 0)), nm(10));
        let original = leg(&world);
        let task_force = world.truth.task_forces.get_mut(&TF).expect("task force");
        task_force.turn_back();
        assert_eq!(
            (
                task_force.from_hex,
                task_force.toward_hex,
                task_force.progress_nm
            ),
            (hx(1, 0), Some(hx(0, 0)), nm(30))
        );
        task_force.turn_back();
        assert_eq!(leg(&world), original);

        // Through two orders. First (0,1): back (see above), giving
        // (1,0) -> (0,0) at 30 nm. Then (2,0): ahead = (40 - 30) + 40 x 2 =
        // 90, back = 30 + 40 x 1 = 70, so it turns back again.
        take_in(&mut world, vec![route_to(0, 1)]);
        assert_eq!(leg(&world), (hx(1, 0), Some(hx(0, 0)), nm(30)));
        world.submit_orders(Side::Allied, vec![route_to(2, 0)]);
        world.intake_orders(2);
        assert_eq!(leg(&world), original);
        assert_sound(&world);
    }

    #[test]
    fn a_route_rejected_at_intake_leaves_the_leg_unchanged() {
        // Off the map: rejected before any choice is made.
        let mut world = world_with_fuel(2000);
        set_leg(&mut world, hx(1, 0), Some(hx(2, 0)), nm(10));
        let before = world.truth.task_forces[&TF].clone();
        take_in(&mut world, vec![route_to(40, 40)]);
        assert_eq!(
            rejection(&world),
            Some(RejectReason::WaypointOffMap(hx(40, 40)))
        );
        assert_eq!(world.truth.task_forces[&TF], before);

        // Blocked: from (2,0), going ahead (190 vs 210 back), the greedy path
        // to (3,3) runs (3,0), (3,1), (3,2), which is land.
        let mut world = world_with_fuel(2000);
        set_leg(&mut world, hx(1, 0), Some(hx(2, 0)), nm(10));
        let before = world.truth.task_forces[&TF].clone();
        take_in(&mut world, vec![route_to(3, 3)]);
        assert_eq!(
            rejection(&world),
            Some(RejectReason::RouteBlocked(hx(3, 2)))
        );
        assert_eq!(world.truth.task_forces[&TF], before);

        // No waypoints at all: empty, even between centres.
        let mut world = world_with_fuel(2000);
        set_leg(&mut world, hx(1, 0), Some(hx(2, 0)), nm(10));
        let before = world.truth.task_forces[&TF].clone();
        take_in(&mut world, vec![order(TaskForceOrder::SetRoute(vec![]))]);
        assert_eq!(rejection(&world), Some(RejectReason::EmptyRoute));
        assert_eq!(world.truth.task_forces[&TF], before);
    }

    #[test]
    fn between_centres_the_hex_ahead_and_the_hex_left_are_real_routes() {
        // Ordered to stop at the hex ahead: accepted. Ahead = (40 - 10) + 0 =
        // 30, back = 10 + 40 = 50, so it goes on and arrives after 30 nm.
        let mut world = world_with_fuel(2000);
        set_leg(&mut world, hx(1, 0), Some(hx(2, 0)), nm(10));
        world
            .truth
            .task_forces
            .get_mut(&TF)
            .expect("task force")
            .status = TaskForceStatus::Underway;
        world.submit_orders(Side::Allied, vec![route_to(2, 0)]);
        world.resolve_day().expect("day resolves");
        assert_eq!(rejection(&world), None);
        assert_eq!(leg(&world), (hx(2, 0), None, nm(0)));
        assert_eq!(
            world.truth.task_forces[&TF].status,
            TaskForceStatus::Arrived
        );
        // 30 nm at 0.25 t/nm is 7.5 t.
        assert_eq!(
            world.truth.ships[&SHIP].fuel_t,
            Fixed::from_int(2000) - Fixed::from_int(15) / Fixed::from_int(2)
        );

        // Ordered back to the hex it left, 1 nm out: accepted. Ahead =
        // (40 - 1) + 40 = 79, back = 1 + 0 = 1, so it turns back and arrives
        // after 1 nm, burning 0.25 t. This is the case decision 5 was changed
        // for.
        let mut world = world_with_fuel(2000);
        set_leg(&mut world, hx(1, 0), Some(hx(2, 0)), nm(1));
        world
            .truth
            .task_forces
            .get_mut(&TF)
            .expect("task force")
            .status = TaskForceStatus::Underway;
        world.submit_orders(Side::Allied, vec![route_to(1, 0)]);
        world.resolve_day().expect("day resolves");
        assert_eq!(rejection(&world), None);
        assert_eq!(leg(&world), (hx(1, 0), None, nm(0)));
        assert_eq!(
            world.truth.task_forces[&TF].status,
            TaskForceStatus::Arrived
        );
        assert_eq!(
            world.truth.ships[&SHIP].fuel_t,
            Fixed::from_int(2000) - Fixed::ONE / Fixed::from_int(4)
        );
    }

    #[test]
    fn an_out_of_fuel_force_mid_leg_is_sound_and_reports_its_position() {
        // 5 t at 0.25 t/nm lasts 20 nm: the force stops 20 nm out of (0,0)
        // toward (1,0), which puts it in (1,0).
        let mut world = world_with_fuel(5);
        world.submit_orders(Side::Allied, vec![route_to(12, 0)]);
        world.resolve_day().expect("invariants hold");

        assert_eq!(leg(&world), (hx(0, 0), Some(hx(1, 0)), nm(20)));
        let task_force = &world.truth.task_forces[&TF];
        assert_eq!(task_force.status, TaskForceStatus::OutOfFuel);
        assert_eq!(task_force.position(), hx(1, 0));
        let reported = world.events().iter().find_map(|event| match event.kind {
            EventKind::TaskForceOutOfFuel { hex, .. } => Some(hex),
            _ => None,
        });
        assert_eq!(reported, Some(hx(1, 0)));
        let entered = world.events().iter().find_map(|event| match &event.kind {
            EventKind::TaskForceSteamed {
                to, hexes_entered, ..
            } => Some((*to, hexes_entered.clone())),
            _ => None,
        });
        assert_eq!(entered, Some((hx(1, 0), vec![hx(1, 0)])));
    }

    #[test]
    fn the_hex_ahead_is_part_of_the_state_hash() {
        let mut a = world_with_fuel(2000);
        set_leg(&mut a, hx(0, 0), Some(hx(1, 0)), nm(10));
        let mut b = world_with_fuel(2000);
        set_leg(&mut b, hx(0, 0), Some(hx(0, 1)), nm(10));
        assert_ne!(a.state_hash(), b.state_hash());
        let mut c = world_with_fuel(2000);
        set_leg(&mut c, hx(0, 0), Some(hx(1, 0)), nm(10));
        assert_eq!(a.state_hash(), c.state_hash());
    }

    #[test]
    fn the_leg_invariants_catch_a_broken_state() {
        let mut world = world_with_fuel(2000);
        let task_force = world.truth.task_forces.get_mut(&TF).expect("task force");
        // A hex ahead with no progress.
        task_force.toward_hex = Some(hx(1, 0));
        assert_eq!(
            invariants::check(&world.truth, &world.map, &world.ref_data),
            vec![crate::invariants::InvariantViolation::TaskForceLegStateInconsistent(TF)]
        );
        // Progress and a hex ahead, but the hex ahead is two hexes away.
        let task_force = world.truth.task_forces.get_mut(&TF).expect("task force");
        task_force.toward_hex = Some(hx(2, 0));
        task_force.progress_nm = nm(5);
        assert_eq!(
            invariants::check(&world.truth, &world.map, &world.ref_data),
            vec![crate::invariants::InvariantViolation::TaskForceTowardNotAdjacent(TF, hx(2, 0))]
        );
        // Off the map: (-2,0) is adjacent to (-1,0) only.
        let task_force = world.truth.task_forces.get_mut(&TF).expect("task force");
        task_force.from_hex = hx(-1, 0);
        task_force.toward_hex = Some(hx(-2, 0));
        assert_eq!(
            invariants::check(&world.truth, &world.map, &world.ref_data),
            vec![crate::invariants::InvariantViolation::TaskForceTowardOffMap(TF, hx(-2, 0))]
        );
    }
}
