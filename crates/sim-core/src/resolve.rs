//! Resolving one day.
//!
//! The order of phases below is the one in the spec ("Daily resolution
//! order"). Orders intake, task force movement (`movement.rs`: it begins with
//! the straggler pass, `stragglers.rs`, and runs the surface fights,
//! `surface.rs`, where forces meet), cargo handling, air search
//! (`search.rs`), contact ageing at the end of every pulse and closing the
//! day do something so far. The other phases, the surface engagement slot
//! among them (decision 8 of
//! `docs/decisions/0022-meeting-and-breaking-off.md`), are listed so the
//! sequence is fixed from the start, and each is filled in by the scenario
//! that needs it.

use crate::events::{Cause, DetachReason, EventKind};
use crate::fixed::Fixed;
use crate::ids::{BaseId, EventId, OrderId, ShipId, TaskForceId};
use crate::invariants;
use crate::orders::{
    AirGroupOrder, AirMission, CargoAction, Order, OrderKind, OrderRequest, Reach, RejectReason,
    SpeedSetting, TaskForceOrder,
};
use crate::refdata::{AircraftRole, AttackWeapon};
use crate::state::{
    CargoOrder, Route, RulesOfEngagement, SearchArc, ShipLocation, Side, TaskForce, TaskForceStatus,
};
use std::collections::BTreeSet;

use crate::time::Pulse;
use crate::world::{DaySummary, SimError, World};

/// A step inside a pulse.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum PulsePhase {
    Movement,
    NightSearch,
    AirSearch,
    StrikeDecisions,
    AirCombat,
    SurfaceEngagements,
    ShoreBombardment,
    Loading,
    /// Damaged aircraft are repaired, first in the night pulse (decision 9
    /// of 0035).
    AircraftRepair,
    /// Aircraft the scenario flies in on the day arrive, after the repair
    /// and before the replacements (decision 2 of 0041).
    AirArrival,
    /// Replacement aircraft and pilots arrive at the bases the scenario names,
    /// right after the repair, from day 2 (decisions 6 to 8 of 0038).
    AirReplacement,
    /// Each base pays the daily supply for its aircraft, after the repair
    /// and the replacements (decisions 2 and 5 of 0039).
    AirSupplyUpkeep,
    /// Aircraft that flew roll for wear, after the air combat step of a
    /// daylight pulse (decisions 3 to 7 of 0035).
    AircraftWear,
    /// The staff stands tired groups down and returns rested ones, right
    /// after the repair at the start of the day (decision 8 of 0036).
    AirGroupRest,
}

/// A step after the last pulse.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EndOfDayPhase {
    GroundCombat,
    Logistics,
    AirGroupUpkeep,
    IntelligenceUpdate,
    /// What each base holds at the end of the day is recorded, last of all
    /// (decision 0067).
    BaseCount,
}

const NIGHT_SEQUENCE: [PulsePhase; 10] = [
    PulsePhase::AircraftRepair,
    PulsePhase::AirArrival,
    PulsePhase::AirReplacement,
    PulsePhase::AirSupplyUpkeep,
    PulsePhase::AirGroupRest,
    PulsePhase::Movement,
    PulsePhase::NightSearch,
    PulsePhase::SurfaceEngagements,
    PulsePhase::ShoreBombardment,
    PulsePhase::Loading,
];

const DAYLIGHT_SEQUENCE: [PulsePhase; 7] = [
    PulsePhase::AirSearch,
    PulsePhase::StrikeDecisions,
    PulsePhase::AirCombat,
    PulsePhase::AircraftWear,
    PulsePhase::Movement,
    PulsePhase::SurfaceEngagements,
    PulsePhase::Loading,
];

const END_OF_DAY_SEQUENCE: [EndOfDayPhase; 5] = [
    EndOfDayPhase::GroundCombat,
    EndOfDayPhase::Logistics,
    EndOfDayPhase::AirGroupUpkeep,
    EndOfDayPhase::IntelligenceUpdate,
    EndOfDayPhase::BaseCount,
];

const fn pulse_sequence(pulse: Pulse) -> &'static [PulsePhase] {
    match pulse {
        Pulse::Night => &NIGHT_SEQUENCE,
        Pulse::Morning | Pulse::Afternoon => &DAYLIGHT_SEQUENCE,
    }
}

impl World {
    /// Resolves the next day: takes in the orders both sides submitted, runs
    /// the three pulses and the end-of-day phases, checks the invariants and
    /// records the state hash.
    pub fn resolve_day(&mut self) -> Result<DaySummary, SimError> {
        let day = self.truth.day + 1;
        let first_event = self.events.len();

        // Fuel comes before the day's orders (decision 2 of 0045).
        self.refuel_in_port(day);
        self.intake_orders(day);

        for pulse in Pulse::ALL {
            // Alert from belief comes before the pulse's first phase
            // (decision 3 of `docs/decisions/0021-readiness-and-alarm.md`).
            self.alert_from_belief(day, pulse);
            // Then fatigue of the forces, which reads who is alert
            // (decision 2 of `docs/decisions/0037-fatigue-at-battle-readiness.md`).
            self.tire_and_rest_forces(day, pulse);
            // A base with no supply at the start of the pulse flies nothing
            // in it (decision 7 of 0039).
            self.work_out_grounded_bases();
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

    pub(crate) fn run_pulse_phase(&mut self, day: u32, pulse: Pulse, phase: PulsePhase) {
        match phase {
            PulsePhase::Movement => self.move_task_forces(day, pulse),
            PulsePhase::Loading => self.handle_cargo(day, pulse),
            PulsePhase::AirSearch => self.search_from_the_air(day, pulse),
            PulsePhase::StrikeDecisions => self.decide_strikes(day, pulse),
            PulsePhase::AirCombat => self.resolve_strikes(day, pulse),
            PulsePhase::AircraftRepair => self.repair_aircraft(day, pulse),
            PulsePhase::AircraftWear => self.wear_aircraft(day, pulse),
            PulsePhase::AirArrival => self.receive_arrivals(day, pulse),
            PulsePhase::AirReplacement => self.receive_replacements(day, pulse),
            PulsePhase::AirSupplyUpkeep => self.pay_daily_supply(day, pulse),
            PulsePhase::AirGroupRest => self.stand_down_and_return_air_groups(day, pulse),
            // Fights happen inside the movement step, where forces meet, and
            // so do the straggler passes after them (decision 0022,
            // decision 8). The slot stays, like night search.
            PulsePhase::SurfaceEngagements => {}
            // Not implemented yet. Each arrives with the scenario that needs it.
            // Night search is decision 7 territory: there is no search where it
            // is dark, so this phase stays empty.
            PulsePhase::NightSearch | PulsePhase::ShoreBombardment => {}
        }
    }

    fn run_end_of_day_phase(&mut self, day: u32, phase: EndOfDayPhase) {
        match phase {
            EndOfDayPhase::BaseCount => self.record_bases_at_day_end(day),
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

    /// Records what every base holds at the end of the day: its supply and
    /// the ready aircraft of the air groups at it (decision 0067). Bases are
    /// taken in base order. It is recorded for every base, whatever the
    /// objectives say: no rule reads the objectives. The event belongs to no
    /// pulse and has no cause. Reads the bases and the air groups; writes no
    /// state and draws no random number.
    fn record_bases_at_day_end(&mut self, day: u32) {
        let counts: Vec<(BaseId, Fixed, u32)> = self
            .truth
            .bases
            .values()
            .map(|base| {
                let ready = self
                    .truth
                    .air_groups
                    .values()
                    .filter(|group| group.location == crate::state::AirGroupLocation::Base(base.id))
                    .map(|group| group.ready)
                    .sum();
                (base.id, base.supply_t, ready)
            })
            .collect();
        for (base, supply_t, aircraft_ready) in counts {
            self.events.record(
                day,
                None,
                EventKind::BaseAtDayEnd {
                    base,
                    supply_t,
                    aircraft_ready,
                },
                Vec::new(),
                None,
            );
        }
    }

    /// Ages each side's contacts after the pulse's last phase, night
    /// included (decision 14 of `docs/decisions/0012-search-and-contacts.md`).
    /// Each aged or dropped contact is an event citing the report that last
    /// refreshed it.
    pub(crate) fn age_contacts_at_end_of_pulse(&mut self, day: u32, pulse: Pulse) {
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
                    }
                } else {
                    EventKind::ContactAged {
                        side,
                        contact: outcome.contact,
                        age_pulses: outcome.age_pulses,
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
    /// in a fixed order. Each side's orders are put in order of the unit they
    /// are for (task forces before air groups, then by number) before they
    /// are numbered, so the numbers do not depend on the order they were
    /// handed in. One unit's orders keep their order, so a later one still
    /// replaces an earlier one (decision 0043, 4a to 4b).
    fn intake_orders(&mut self, day: u32) {
        let pending = std::mem::take(&mut self.pending);
        for (side, mut requests) in pending {
            requests.sort_by_key(|request| request.kind.target());
            for request in requests {
                self.take_in_order(day, None, side, request, Vec::new());
            }
        }
    }

    /// Takes in one order: numbers it, checks it, applies it if it is sound,
    /// records `OrderAccepted` or `OrderRejected` and stores it. The event's
    /// causes are the order itself followed by `extra_causes`. `pulse` is
    /// `None` at the start of a day and the current pulse for an order the
    /// simulation issues mid-day. Returns the order's ID, the event's ID and
    /// whether it was accepted.
    pub(crate) fn take_in_order(
        &mut self,
        day: u32,
        pulse: Option<Pulse>,
        side: Side,
        request: OrderRequest,
        extra_causes: Vec<Cause>,
    ) -> (OrderId, EventId, bool) {
        let id = self.order_ids.alloc();
        let order = Order {
            id,
            side,
            day_issued: day,
            kind: request.kind,
            origin: request.origin,
        };
        let target = order.kind.target();
        let result = self.validate_order(&order);
        let accepted = result.is_ok();
        let mut turned_back: Option<EventKind> = None;
        let kind = match result {
            Ok(()) => {
                turned_back = self.apply_order(&order);
                EventKind::OrderAccepted { order: id, target }
            }
            Err(reason) => EventKind::OrderRejected {
                order: id,
                target,
                reason,
            },
        };
        let mut causes = vec![Cause::Order(id)];
        causes.extend(extra_causes);
        let event = self.events.record(day, pulse, kind, causes, None);
        // A route order that turned the force round between two hexes changed
        // its leg and maybe its position: say so, citing the order.
        if let Some(turned_back) = turned_back {
            self.events
                .record(day, pulse, turned_back, vec![Cause::Order(id)], None);
        }
        self.orders.insert(id, order);
        (id, event, accepted)
    }

    /// Splits `ships` off `parent` into a new task force and returns its ID
    /// (decision 15 of `docs/decisions/0016-systems-and-stragglers.md`). The
    /// new force has the parent's side, mission, speed setting and speed
    /// order, home port, readiness and readiness order, alerted flag and the event that alerted it,
    /// fatigue (decision 0037), rules of engagement and rules order, and its
    /// exact position. It is idle with no route and no other standing
    /// order. The ships move into it. Records no event: the caller does.
    /// Every ship must be in the parent.
    pub(crate) fn split_task_force(
        &mut self,
        parent: TaskForceId,
        ships: &BTreeSet<ShipId>,
        name: String,
    ) -> TaskForceId {
        let parent_side = self.truth.task_forces[&parent].side;
        let id = self.made_ids.alloc_task_force(parent_side);
        let parent_force = self
            .truth
            .task_forces
            .get_mut(&parent)
            .expect("the parent task force exists");
        let mut force = TaskForce::at(
            id,
            name,
            parent_force.side,
            parent_force.from_hex,
            parent_force.mission,
        );
        force.speed = parent_force.speed;
        force.speed_order = parent_force.speed_order;
        force.home_port = parent_force.home_port;
        force.readiness = parent_force.readiness;
        force.readiness_order = parent_force.readiness_order;
        force.alerted = parent_force.alerted;
        force.alerted_by = parent_force.alerted_by;
        force.fatigue = parent_force.fatigue;
        force.rules_of_engagement = parent_force.rules_of_engagement;
        force.rules_order = parent_force.rules_order;
        force.toward_hex = parent_force.toward_hex;
        force.progress_nm = parent_force.progress_nm;
        force.leg_nm = parent_force.leg_nm;
        for ship in ships {
            let removed = parent_force.ships.remove(ship);
            assert!(removed, "{ship} is in the parent task force");
            force.ships.insert(*ship);
        }
        for ship in ships {
            self.truth
                .ships
                .get_mut(ship)
                .expect("ship exists")
                .location = ShipLocation::TaskForce(id);
        }
        self.truth.task_forces.insert(id, force);
        id
    }

    /// Whether the engine would accept this order from `side` if it were
    /// taken in now: the same check intake makes (`validate_order`), against
    /// the state as it stands. Changes nothing, numbers nothing and records
    /// nothing. An order queued is checked again when the day begins, and the
    /// state may have changed by then (decision 4 of 0050).
    pub fn check_order(&self, side: Side, request: &OrderRequest) -> Result<(), RejectReason> {
        let order = Order {
            id: OrderId::new(0),
            side,
            day_issued: self.truth.day + 1,
            kind: request.kind.clone(),
            origin: request.origin,
        };
        self.validate_order(&order)
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
                // A force whose every ship is sunk has nothing to order.
                if task_force.ships.is_empty() {
                    return Err(RejectReason::NoShipAfloat);
                }
                match task_force_order {
                    TaskForceOrder::SetRoute(waypoints) => {
                        task_force.check_route(&self.map, waypoints)
                    }
                    TaskForceOrder::Load | TaskForceOrder::LoadFuel => {
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
                    TaskForceOrder::SetRulesOfEngagement {
                        break_off,
                        keep_share,
                        ..
                    } => {
                        if *break_off < Fixed::ZERO || *break_off > Fixed::ONE {
                            return Err(RejectReason::BreakOffOutOfRange(*break_off));
                        }
                        if *keep_share < Fixed::ZERO || *keep_share > Fixed::ONE {
                            return Err(RejectReason::KeepShareOutOfRange(*keep_share));
                        }
                        Ok(())
                    }
                    TaskForceOrder::SetHomePort(base) => match self.truth.bases.get(base) {
                        Some(base) if base.side == order.side => Ok(()),
                        _ => Err(RejectReason::NotYourBase),
                    },
                    TaskForceOrder::DetachShips(ships) => {
                        if ships.is_empty() {
                            return Err(RejectReason::NoShipsToDetach);
                        }
                        for ship in ships {
                            if !task_force.ships.contains(ship) {
                                return Err(RejectReason::ShipNotInTaskForce(*ship));
                            }
                        }
                        let listed: BTreeSet<ShipId> = ships.iter().copied().collect();
                        if listed.len() == task_force.ships.len() {
                            return Err(RejectReason::WouldLeaveTaskForceEmpty);
                        }
                        Ok(())
                    }
                    TaskForceOrder::SetSpeed(SpeedSetting::Knots(knots)) => {
                        if *knots <= Fixed::ZERO {
                            Err(RejectReason::SpeedNotAboveZero(*knots))
                        } else {
                            Ok(())
                        }
                    }
                    TaskForceOrder::SetSpeed(_)
                    | TaskForceOrder::SetMission(_)
                    | TaskForceOrder::SetReadiness(_)
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
                // A group lost with the ship it was aboard has nothing to
                // order.
                if let crate::state::AirGroupLocation::Ship(ship) = group.location {
                    let afloat = self
                        .truth
                        .ships
                        .get(&ship)
                        .is_some_and(|ship| ship.location != ShipLocation::Sunk);
                    if !afloat {
                        return Err(RejectReason::AirGroupLost);
                    }
                }
                match air_group_order {
                    AirGroupOrder::SetAirMission(AirMission::NavalStrike) => {
                        // Decision 2 of 0026. World::new checked that every
                        // group's type exists.
                        if self.ref_data.air_combat.is_none() {
                            return Err(RejectReason::NoAirCombatFigures);
                        }
                        let weapon = self
                            .ref_data
                            .aircraft_types
                            .get(&group.aircraft_type)
                            .expect("air group has a known aircraft type")
                            .attack_weapon;
                        if weapon == AttackWeapon::None {
                            return Err(RejectReason::AircraftAttackWithNothing);
                        }
                        Ok(())
                    }
                    AirGroupOrder::SetAirMission(AirMission::BaseStrike(base)) => {
                        // Decision 1 of 0034: the same checks as the naval
                        // strike, and the base must be an enemy's.
                        if self.ref_data.air_combat.is_none() {
                            return Err(RejectReason::NoAirCombatFigures);
                        }
                        let weapon = self
                            .ref_data
                            .aircraft_types
                            .get(&group.aircraft_type)
                            .expect("air group has a known aircraft type")
                            .attack_weapon;
                        if weapon == AttackWeapon::None {
                            return Err(RejectReason::AircraftAttackWithNothing);
                        }
                        match self.truth.bases.get(base) {
                            Some(base) if base.side != group.side => Ok(()),
                            _ => Err(RejectReason::NotAnEnemyBase),
                        }
                    }
                    AirGroupOrder::SetAirMission(AirMission::CombatAirPatrol) => {
                        // Decision 1 of 0027.
                        if self.ref_data.air_combat.is_none() {
                            return Err(RejectReason::NoAirCombatFigures);
                        }
                        let role = self
                            .ref_data
                            .aircraft_types
                            .get(&group.aircraft_type)
                            .expect("air group has a known aircraft type")
                            .role;
                        if role != AircraftRole::Fighter {
                            return Err(RejectReason::NotAFighter);
                        }
                        Ok(())
                    }
                    AirGroupOrder::SetAirMission(_) => Ok(()),
                    AirGroupOrder::SetPatrolShare { share } => {
                        if *share < Fixed::ZERO || *share > Fixed::ONE {
                            Err(RejectReason::PatrolShareOutOfRange(*share))
                        } else {
                            Ok(())
                        }
                    }
                    AirGroupOrder::SetRestThreshold { fatigue } => {
                        if *fatigue < Fixed::ZERO || *fatigue > Fixed::from_int(100) {
                            Err(RejectReason::FatigueOutOfRange(*fatigue))
                        } else {
                            Ok(())
                        }
                    }
                    AirGroupOrder::SetReach { reach } => {
                        // Decision 1 of 0059. Normal reach is the rule as it
                        // was and is always accepted, so a group can go back
                        // to it. Extended reach means something only where
                        // the air combat figures are.
                        if *reach == Reach::Extended && self.ref_data.air_combat.is_none() {
                            Err(RejectReason::NoAirCombatFigures)
                        } else {
                            Ok(())
                        }
                    }
                    AirGroupOrder::SetTargetPriority { kinds } => {
                        // Decision 0063: a list names at least one kind, and
                        // each kind once.
                        if kinds.is_empty() {
                            return Err(RejectReason::TargetListEmpty);
                        }
                        for (index, kind) in kinds.iter().enumerate() {
                            if kinds[..index].contains(kind) {
                                return Err(RejectReason::TargetKindRepeated(*kind));
                            }
                        }
                        Ok(())
                    }
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
    ///
    /// Returns the event to record after the order's own, if the order turned
    /// a task force round between two hexes (a route order does: see
    /// `TaskForce::turn_back`).
    fn apply_order(&mut self, order: &Order) -> Option<EventKind> {
        match &order.kind {
            OrderKind::TaskForce {
                target,
                order: task_force_order,
            } => {
                if let TaskForceOrder::DetachShips(ships) = task_force_order {
                    let listed: BTreeSet<ShipId> = ships.iter().copied().collect();
                    let name = format!("{}, detached", self.truth.task_forces[target].name);
                    let new_force = self.split_task_force(*target, &listed, name);
                    // Name the new force in the log: the order's acceptance
                    // names only the parent.
                    self.events.record(
                        order.day_issued,
                        None,
                        EventKind::ShipsDetached {
                            parent: *target,
                            new_force,
                            ships: listed.iter().copied().collect(),
                            escorts: Vec::new(),
                            reason: DetachReason::Ordered,
                        },
                        vec![Cause::Order(order.id)],
                        None,
                    );
                    return None;
                }
                let task_force = self
                    .truth
                    .task_forces
                    .get_mut(target)
                    .expect("order was validated against this task force");
                let mut turned_back = None;
                match task_force_order {
                    TaskForceOrder::SetRoute(waypoints) => {
                        // Turning back is decided on the state the order
                        // found, and the leg is then finished by movement.
                        let (turns_back, _) = task_force
                            .reroute_centre(&self.map, waypoints.first().copied())
                            .expect("order was validated: the first waypoint is reachable");
                        if turns_back {
                            task_force.turn_back();
                            if let Some(toward) = task_force.toward_hex {
                                turned_back = Some(EventKind::TaskForceTurnedBack {
                                    task_force: task_force.id,
                                    from: task_force.from_hex,
                                    toward,
                                    progress_nm: task_force.progress_nm,
                                    position: task_force.position(),
                                });
                            }
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
                    TaskForceOrder::LoadFuel => {
                        task_force.cargo_order = Some(CargoOrder {
                            action: CargoAction::LoadFuel,
                            order: order.id,
                        });
                    }
                    TaskForceOrder::Unload => {
                        task_force.cargo_order = Some(CargoOrder {
                            action: CargoAction::Unload,
                            order: order.id,
                        });
                    }
                    TaskForceOrder::SetRulesOfEngagement {
                        stance,
                        break_off,
                        stragglers,
                        keep_share,
                    } => {
                        task_force.rules_of_engagement = RulesOfEngagement {
                            stance: *stance,
                            break_off: *break_off,
                            stragglers: *stragglers,
                            keep_share: *keep_share,
                        };
                        task_force.rules_order = Some(order.id);
                    }
                    TaskForceOrder::SetHomePort(base) => {
                        task_force.home_port = Some(*base);
                    }
                    TaskForceOrder::SetReadiness(readiness) => {
                        task_force.readiness = *readiness;
                        task_force.readiness_order = Some(order.id);
                    }
                    TaskForceOrder::DetachShips(_) => {
                        unreachable!("detach orders are applied above")
                    }
                }
                turned_back
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
                    AirGroupOrder::SetPatrolShare { share } => {
                        group.patrol_share = *share;
                        group.share_order = Some(order.id);
                    }
                    AirGroupOrder::SetRestThreshold { fatigue } => {
                        group.rest_above = Some(*fatigue);
                        group.rest_order = Some(order.id);
                    }
                    AirGroupOrder::SetReach { reach } => {
                        group.reach = *reach;
                        group.reach_order = Some(order.id);
                    }
                    AirGroupOrder::SetTargetPriority { kinds } => {
                        group.target_priority = kinds.clone();
                        group.target_order = Some(order.id);
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
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_night_pays_the_daily_supply_after_the_repair_and_replacements_and_before_the_rest() {
        let at = |phase: PulsePhase| {
            NIGHT_SEQUENCE
                .iter()
                .position(|p| *p == phase)
                .expect("the phase is in the night")
        };
        assert!(at(PulsePhase::AircraftRepair) < at(PulsePhase::AirReplacement));
        assert!(at(PulsePhase::AirReplacement) < at(PulsePhase::AirSupplyUpkeep));
        assert!(at(PulsePhase::AirSupplyUpkeep) < at(PulsePhase::AirGroupRest));
        assert!(!DAYLIGHT_SEQUENCE.contains(&PulsePhase::AirSupplyUpkeep));
    }

    use super::*;
    use crate::belief::{BattleClaim, Report};
    use crate::hex::Hex;
    use crate::ids::{AirGroupId, AircraftTypeId, BaseId, ShipClassId};
    use crate::map::{HexData, HexKind, Map};
    use crate::movement::speed_multiplier;
    use crate::orders::{
        AirMission, Mission, OrderRequest, Origin, Readiness, Stance, StragglerInstruction,
    };
    use crate::refdata::{
        AircraftRole, AircraftType, PortSize, RefData, SearchFigures, ShipClass, ShipKind,
        SurfaceCombatFigures,
    };
    use crate::state::{
        AirGroup, AirGroupLocation, Base, Ship, ShipLocation, Side, SystemState, TaskForce,
        TruthState,
    };

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
                fuel_t: Fixed::from_int(fuel_t),
                ..Ship::new(
                    SHIP,
                    "Test ship".to_string(),
                    ShipClassId::new(1),
                    Side::Allied,
                    ShipLocation::TaskForce(TF),
                )
            },
        );
        truth.bases.insert(
            BaseId::new(1),
            Base::new(
                BaseId::new(1),
                "Test base".to_string(),
                Side::Allied,
                Hex::new(0, 0),
            ),
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

    fn knots(k: i64) -> OrderRequest {
        order(TaskForceOrder::SetSpeed(SpeedSetting::Knots(nm(k))))
    }

    #[test]
    fn a_speed_in_knots_is_capped_by_the_best_speed_of_the_slowest_ship() {
        // The class steams at 16 kn at cruise and 32 kn at full. The degraded
        // factor is 1/2.
        //   24 ordered, working:  min(24, 32 x 1)   = 24 kn
        //   32 ordered, working:  min(32, 32 x 1)   = 32 kn
        //   40 ordered, working:  min(40, 32 x 1)   = 32 kn (top speed)
        //   24 ordered, degraded: min(24, 32 x 1/2) = 16 kn (best speed 16)
        // A force goes at its slowest ship: a working ship and a degraded
        // one, 24 ordered, give min(24, 32) = 24 and min(24, 16) = 16, so 16.
        for (ordered, degraded_ships, expected) in
            [(24, 0, 24), (32, 0, 32), (40, 0, 32), (24, 1, 16)]
        {
            let mut world = damage_world();
            if degraded_ships == 1 {
                add_ship(&mut world, 2, SystemState::Degraded);
            }
            world.submit_orders(Side::Allied, vec![route_to(12, 0), knots(ordered)]);
            world.resolve_day().expect("day resolves");
            let speeds = steamed_speeds(&world);
            assert!(!speeds.is_empty());
            assert!(
                speeds.iter().all(|s| *s == nm(expected)),
                "{ordered} kn ordered: {speeds:?}"
            );
            assert_eq!(
                world.truth.task_forces[&TF].speed,
                SpeedSetting::Knots(nm(ordered))
            );
        }
        // A degraded ship alone, 24 ordered: min(24, 32 x 1/2) = 16 kn too.
        let mut world = damage_world();
        set_propulsion(&mut world, SHIP, SystemState::Degraded);
        world.submit_orders(Side::Allied, vec![route_to(12, 0), knots(24)]);
        world.resolve_day().expect("day resolves");
        assert!(steamed_speeds(&world).iter().all(|s| *s == nm(16)));
    }

    #[test]
    fn fuel_burn_in_knots_uses_the_speed_steamed_against_the_cruise_speed() {
        // Cruise 16 kn, burn 2000 t / 8000 nm = 0.25 t/nm at cruise. The
        // route to (6,0) is 6 hexes = 240 nm, so at cruise rate it takes
        // 240 x 0.25 = 60 t.
        //   24 kn: (24/16)^2 = 2.25, 0.5625 t/nm, 240 nm -> 135 t
        //   32 kn: (32/16)^2 = 4,    1 t/nm,      240 nm -> 240 t
        //   16 kn: multiplier 1,     0.25 t/nm,   240 nm -> 60 t
        //   degraded, 24 ordered: steams 16 kn (best speed 32 x 1/2), so
        //   multiplier 1 against its own cruise 16 kn -> 60 t
        for (ordered, degraded, burned) in [
            (24, false, 135),
            (32, false, 240),
            (16, false, 60),
            (24, true, 60),
        ] {
            let mut world = damage_world();
            if degraded {
                set_propulsion(&mut world, SHIP, SystemState::Degraded);
            }
            world.submit_orders(Side::Allied, vec![route_to(6, 0), knots(ordered)]);
            world.resolve_day().expect("day resolves");
            let truth = world.umpire();
            assert_eq!(truth.task_forces[&TF].status, TaskForceStatus::Arrived);
            assert_eq!(
                truth.ships[&SHIP].fuel_t,
                nm(2000 - burned),
                "{ordered} kn, degraded {degraded}"
            );
        }
    }

    #[test]
    fn a_speed_in_knots_must_be_above_zero() {
        // 0 and -1 are rejected with the new reason and leave the speed as
        // it was; 1 kn (and a fraction) is accepted.
        let mut world = damage_world();
        let fraction = order(TaskForceOrder::SetSpeed(SpeedSetting::Knots(nm_frac(1, 2))));
        world.submit_orders(Side::Allied, vec![knots(0), knots(-1), knots(1), fraction]);
        world.resolve_day().expect("day resolves");
        assert_eq!(
            rejections(&world),
            vec![
                RejectReason::SpeedNotAboveZero(nm(0)),
                RejectReason::SpeedNotAboveZero(nm(-1)),
            ]
        );
        assert_eq!(
            world.truth.task_forces[&TF].speed,
            SpeedSetting::Knots(nm_frac(1, 2))
        );
        assert_eq!(
            RejectReason::SpeedNotAboveZero(nm(-1)).to_string(),
            "speed -1 kn is not above 0"
        );
        assert_sound(&world);
    }

    #[test]
    fn the_speed_setting_is_part_of_the_state_hash() {
        // Cruise, full and knots differ, and so do two knots values.
        let hash_with = |speed: SpeedSetting| {
            let mut world = damage_world();
            world.truth.task_forces.get_mut(&TF).expect("force").speed = speed;
            world.state_hash()
        };
        let hashes = [
            hash_with(SpeedSetting::Cruise),
            hash_with(SpeedSetting::Full),
            hash_with(SpeedSetting::Knots(nm(24))),
            hash_with(SpeedSetting::Knots(nm(25))),
        ];
        let distinct: BTreeSet<u64> = hashes.iter().copied().collect();
        assert_eq!(distinct.len(), 4);
    }

    #[test]
    fn a_readiness_order_sets_readiness_and_its_order_and_nothing_else() {
        let mut world = damage_world();
        {
            // Alerted already; readiness must not touch it.
            let force = world.truth.task_forces.get_mut(&TF).expect("force");
            force.alerted = true;
            force.alerted_by = Some(EventId::new(1));
        }
        // Alert from belief (decision 3 of 0021) runs at the start of every
        // pulse and would stand the force down with no contact near it. So
        // the Allied side holds a contact on the force that never ages out of
        // the alert limit or drops, and the force stays alerted by it.
        {
            let position = world.truth.task_forces[&TF].position();
            let mut search = world.ref_data.search.clone().expect("search figures");
            search.contact_drop_age_pulses = 1000;
            search.fresh_contact_max_age_pulses = 1000;
            world.ref_data.search = Some(search.clone());
            world
                .beliefs
                .get_mut(&Side::Allied)
                .expect("belief")
                .apply_report(
                    &Report::Sighting(crate::belief::SightingReport {
                        estimated_hex: position,
                        estimated_ship_count: 1,
                        estimated_kind: ShipKind::Cruiser,
                        received_event: EventId::new(1),
                    }),
                    &search,
                );
        }
        let before = world.truth.task_forces[&TF].clone();
        assert_eq!(before.readiness, Readiness::Normal);
        assert_eq!(before.readiness_order, None);
        // Orders are numbered from 1 in the order they are taken in.
        world.submit_orders(
            Side::Allied,
            vec![order(TaskForceOrder::SetReadiness(Readiness::Battle))],
        );
        world.resolve_day().expect("day resolves");
        assert!(rejections(&world).is_empty());
        let after = &world.truth.task_forces[&TF];
        assert_eq!(after.readiness, Readiness::Battle);
        assert_eq!(after.readiness_order, Some(OrderId::new(1)));
        // Everything else is as it was.
        let mut expected = before;
        expected.readiness = Readiness::Battle;
        expected.readiness_order = Some(OrderId::new(1));
        assert_eq!(*after, expected);
        assert!(after.alerted);
        assert_eq!(after.alerted_by, Some(EventId::new(1)));
        assert_sound(&world);

        // A later order back to normal replaces both; alerted stays.
        world.submit_orders(
            Side::Allied,
            vec![order(TaskForceOrder::SetReadiness(Readiness::Normal))],
        );
        world.resolve_day().expect("day resolves");
        let after = &world.truth.task_forces[&TF];
        assert_eq!(after.readiness, Readiness::Normal);
        assert_eq!(after.readiness_order, Some(OrderId::new(2)));
        assert!(after.alerted);
    }

    #[test]
    fn a_readiness_order_for_the_other_sides_force_is_rejected() {
        let mut world = damage_world();
        world.submit_orders(
            Side::Japanese,
            vec![order(TaskForceOrder::SetReadiness(Readiness::Battle))],
        );
        world.resolve_day().expect("day resolves");
        assert_eq!(rejections(&world), vec![RejectReason::NotYourUnit]);
        let force = &world.truth.task_forces[&TF];
        assert_eq!(force.readiness, Readiness::Normal);
        assert_eq!(force.readiness_order, None);
    }

    #[test]
    fn readiness_has_stable_codes_and_names() {
        assert_eq!(Readiness::Normal.code(), 1);
        assert_eq!(Readiness::Battle.code(), 2);
        assert_eq!(Readiness::Normal.name(), "normal");
        assert_eq!(Readiness::Battle.name(), "battle");
        assert_eq!(Readiness::default(), Readiness::Normal);
    }

    #[test]
    fn a_force_is_alert_at_battle_readiness_or_when_alerted() {
        // is_alert = battle OR alerted.
        //   normal, not alerted -> false     normal, alerted -> true
        //   battle, not alerted -> true      battle, alerted -> true
        let world = damage_world();
        let mut force = world.truth.task_forces[&TF].clone();
        for (readiness, alerted, expected) in [
            (Readiness::Normal, false, false),
            (Readiness::Normal, true, true),
            (Readiness::Battle, false, true),
            (Readiness::Battle, true, true),
        ] {
            force.readiness = readiness;
            force.alerted = alerted;
            assert_eq!(
                force.is_alert(),
                expected,
                "{readiness:?}, alerted {alerted}"
            );
        }
        // A new force is at normal readiness and not alerted.
        assert!(!TaskForce::at(
            TF,
            "F".to_string(),
            Side::Allied,
            hx(0, 0),
            Mission::Transport
        )
        .is_alert());
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
                fuel_t: Fixed::from_int(2000),
                ..Ship::new(
                    fast_ship,
                    "Fast ship".to_string(),
                    ShipClassId::new(2),
                    Side::Allied,
                    ShipLocation::TaskForce(TF),
                )
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
        // Only the bases' counts at the end of the day are recorded
        // (decision 0067).
        let events_before = world.events().len();
        world.resolve_day().expect("day resolves");
        assert!(world
            .events()
            .iter()
            .skip(events_before)
            .all(|event| matches!(event.kind, EventKind::BaseAtDayEnd { .. })));
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

        // The single land hex (3,2) is on the map but no ship can reach it:
        // unreachable, not off the map (decision 0017, decision 13).
        let mut world = world_with_fuel(2000);
        world.submit_orders(Side::Allied, vec![route_to(3, 2)]);
        world.resolve_day().expect("day resolves");
        assert_eq!(
            rejection(&world),
            Some(RejectReason::WaypointUnreachable(Hex::new(3, 2)))
        );

        // (3,3) lies behind that hex on the straight line, but ships can go
        // round it: 6 steps either way, so the order is accepted.
        let mut world = world_with_fuel(2000);
        assert_eq!(
            world.map.sea_distance(Hex::new(0, 0), Hex::new(3, 3)),
            Some(6)
        );
        world.submit_orders(Side::Allied, vec![route_to(3, 3)]);
        world.resolve_day().expect("day resolves");
        assert_eq!(rejection(&world), None);

        // A wall of land across the whole strip at q = 4 cuts (6,0) off.
        let mut world = world_with_fuel(2000);
        wall_at_q(&mut world, 4);
        world.submit_orders(Side::Allied, vec![route_to(6, 0)]);
        world.resolve_day().expect("day resolves");
        assert_eq!(
            rejection(&world),
            Some(RejectReason::WaypointUnreachable(Hex::new(6, 0)))
        );
        assert_eq!(
            world.umpire().task_forces[&TF].status,
            TaskForceStatus::Idle
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
            strike_range_nm: Fixed::ZERO,
            attack_weapon: crate::refdata::AttackWeapon::None,
            attack_damage: Fixed::ZERO,
            pilot_armour: false,
            self_sealing_tanks: false,
        };
        world
            .ref_data
            .aircraft_types
            .insert(aircraft_type.id, aircraft_type);
        world.ref_data.search = Some(SearchFigures::from_search_toml());
        world.truth.air_groups.insert(
            GROUP,
            AirGroup {
                ready: 3,
                strength: 3,
                ..AirGroup::new(
                    GROUP,
                    "Test group".to_string(),
                    Side::Allied,
                    AirGroupLocation::Base(BaseId::new(1)),
                    AircraftTypeId::new(1),
                )
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
            r.search.as_mut().expect("search").position_error_chance = nm(2);
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
    fn world_new_checks_air_groups_aboard_ships_against_capacity() {
        // Ship 1 is an Allied ship of class 1 in the standard test world.
        let good = world_with_air_group();
        let ship = ShipId::new(1);
        let build = |capacity: u32, location: AirGroupLocation| {
            let mut ref_data = good.ref_data.clone();
            ref_data
                .ship_classes
                .get_mut(&ShipClassId::new(1))
                .expect("class")
                .aircraft_capacity = capacity;
            let mut truth = good.truth.clone();
            truth.air_groups.get_mut(&GROUP).expect("group").location = location;
            World::new(1, ref_data, good.map.clone(), truth)
        };
        // The group has 3 aircraft.
        assert!(build(3, AirGroupLocation::Ship(ship)).is_ok());
        assert!(build(36, AirGroupLocation::Ship(ship)).is_ok());
        assert!(build(2, AirGroupLocation::Ship(ship)).is_err());
        assert!(build(0, AirGroupLocation::Ship(ship)).is_err());
        // A ship that does not exist.
        assert!(build(36, AirGroupLocation::Ship(ShipId::new(9))).is_err());
        // The capacity of a ship only matters to groups aboard it.
        assert!(build(0, AirGroupLocation::Base(BaseId::new(1))).is_ok());
    }

    #[test]
    fn where_an_air_group_is_based_is_part_of_the_state_hash() {
        let at_base = world_with_air_group();
        let mut aboard = at_base.clone();
        aboard
            .truth
            .air_groups
            .get_mut(&GROUP)
            .expect("group")
            .location = AirGroupLocation::Ship(ShipId::new(1));
        let mut aboard_other = aboard.clone();
        aboard_other
            .truth
            .air_groups
            .get_mut(&GROUP)
            .expect("group")
            .location = AirGroupLocation::Ship(ShipId::new(2));
        let hashes = [
            at_base.state_hash(),
            aboard.state_hash(),
            aboard_other.state_hash(),
        ];
        assert_ne!(hashes[0], hashes[1]);
        assert_ne!(hashes[1], hashes[2]);
        assert_ne!(hashes[0], hashes[2]);
    }

    #[test]
    fn the_new_air_group_fields_are_part_of_the_state_hash() {
        let base = world_with_air_group();
        let mut fewer_ready = base.clone();
        fewer_ready
            .truth
            .air_groups
            .get_mut(&GROUP)
            .expect("group")
            .ready = 2;
        assert_ne!(base.state_hash(), fewer_ready.state_hash());
    }

    // ---- Rules of engagement and surface combat state (decision 0014) ----

    fn roe_order(stance: Stance, break_off: Fixed) -> OrderRequest {
        order(TaskForceOrder::SetRulesOfEngagement {
            stance,
            break_off,
            stragglers: StragglerInstruction::Keep,
            keep_share: Fixed::ZERO,
        })
    }

    #[test]
    fn a_rules_of_engagement_order_is_accepted_and_stored() {
        let mut world = world_with_fuel(2000);
        let task_force = &world.truth.task_forces[&TF];
        assert_eq!(task_force.rules_of_engagement, RulesOfEngagement::DEFAULT);
        assert_eq!(task_force.rules_of_engagement.stance, Stance::Engage);
        assert_eq!(task_force.rules_of_engagement.break_off, nm_frac(5, 10));
        assert_eq!(task_force.rules_order, None);

        world.submit_orders(
            Side::Allied,
            vec![roe_order(Stance::Avoid, nm_frac(25, 100))],
        );
        world.resolve_day().expect("day resolves");
        assert_eq!(rejection(&world), None);
        let task_force = &world.truth.task_forces[&TF];
        assert_eq!(task_force.rules_of_engagement.stance, Stance::Avoid);
        assert_eq!(task_force.rules_of_engagement.break_off, nm_frac(25, 100));
        assert_eq!(task_force.rules_order, Some(crate::ids::OrderId::new(1)));
        assert!(world
            .events()
            .iter()
            .any(|event| matches!(event.kind, EventKind::OrderAccepted { .. })));
    }

    #[test]
    fn break_off_levels_of_zero_and_one_are_accepted() {
        for level in [Fixed::ZERO, Fixed::ONE] {
            let mut world = world_with_fuel(2000);
            world.submit_orders(Side::Allied, vec![roe_order(Stance::Engage, level)]);
            world.resolve_day().expect("day resolves");
            assert_eq!(rejection(&world), None, "{level}");
            assert_eq!(
                world.truth.task_forces[&TF].rules_of_engagement.break_off,
                level
            );
        }
    }

    #[test]
    fn break_off_levels_outside_zero_to_one_are_rejected() {
        for raw in [-1, 1_000_001] {
            let level = Fixed::from_raw(raw);
            let mut world = world_with_fuel(2000);
            world.submit_orders(Side::Allied, vec![roe_order(Stance::Avoid, level)]);
            world.resolve_day().expect("day resolves");
            assert_eq!(
                rejection(&world),
                Some(RejectReason::BreakOffOutOfRange(level))
            );
            let task_force = &world.truth.task_forces[&TF];
            assert_eq!(task_force.rules_of_engagement, RulesOfEngagement::DEFAULT);
            assert_eq!(task_force.rules_order, None);
        }
        assert!(RejectReason::BreakOffOutOfRange(Fixed::from_raw(-1))
            .to_string()
            .contains("-0.000001"));
    }

    #[test]
    fn a_rules_of_engagement_order_does_not_clear_the_route_or_speed_order() {
        let mut world = world_with_fuel(2000);
        world.submit_orders(
            Side::Allied,
            vec![
                route_to(12, 0),
                order(TaskForceOrder::SetSpeed(SpeedSetting::Full)),
            ],
        );
        world.resolve_day().expect("day resolves");
        let before = world.truth.task_forces[&TF].clone();
        assert!(before.route_order.is_some());
        assert!(before.speed_order.is_some());

        world.submit_orders(Side::Allied, vec![roe_order(Stance::Avoid, nm_frac(1, 10))]);
        world.resolve_day().expect("day resolves");
        let after = &world.truth.task_forces[&TF];
        assert_eq!(after.route, before.route);
        assert_eq!(after.route_order, before.route_order);
        assert_eq!(after.speed, SpeedSetting::Full);
        assert_eq!(after.speed_order, before.speed_order);
        assert_eq!(after.cargo_order, before.cargo_order);
        assert_eq!(after.mission, before.mission);
        assert_eq!(after.rules_of_engagement.stance, Stance::Avoid);
        assert_eq!(after.rules_order, Some(crate::ids::OrderId::new(3)));
    }

    #[test]
    fn a_route_or_speed_order_does_not_clear_the_rules_of_engagement() {
        let mut world = world_with_fuel(2000);
        world.submit_orders(Side::Allied, vec![roe_order(Stance::Avoid, nm_frac(1, 10))]);
        world.resolve_day().expect("day resolves");
        world.submit_orders(
            Side::Allied,
            vec![
                route_to(2, 0),
                order(TaskForceOrder::SetSpeed(SpeedSetting::Full)),
            ],
        );
        world.resolve_day().expect("day resolves");
        let task_force = &world.truth.task_forces[&TF];
        assert_eq!(task_force.rules_of_engagement.stance, Stance::Avoid);
        assert_eq!(task_force.rules_of_engagement.break_off, nm_frac(1, 10));
        assert_eq!(task_force.rules_order, Some(crate::ids::OrderId::new(1)));
    }

    #[test]
    fn a_rules_of_engagement_order_for_the_other_sides_force_is_rejected() {
        let mut world = world_with_fuel(2000);
        world.submit_orders(
            Side::Japanese,
            vec![roe_order(Stance::Avoid, nm_frac(1, 10))],
        );
        world.resolve_day().expect("day resolves");
        assert_eq!(rejection(&world), Some(RejectReason::NotYourUnit));
        let task_force = &world.truth.task_forces[&TF];
        assert_eq!(task_force.rules_of_engagement, RulesOfEngagement::DEFAULT);
        assert_eq!(task_force.rules_order, None);
    }

    #[test]
    fn world_new_checks_the_surface_combat_figures_and_class_toughness() {
        let good = world_with_fuel(2000);
        let build = |change: &dyn Fn(&mut RefData)| {
            let mut ref_data = good.ref_data.clone();
            change(&mut ref_data);
            World::new(1, ref_data, good.map.clone(), good.truth.clone())
        };
        // No figures at all is fine for a world with no fights.
        assert!(build(&|_| {}).is_ok());
        assert!(build(&|r| {
            r.search = Some(SearchFigures::from_search_toml());
            r.surface_combat = Some(SurfaceCombatFigures::from_surface_combat_toml());
        })
        .is_ok());
        // Fights report through the search figures, so they need them.
        assert!(build(&|r| {
            r.surface_combat = Some(SurfaceCombatFigures::from_surface_combat_toml());
        })
        .is_err());
        assert!(build(&|r| {
            r.search = Some(SearchFigures::from_search_toml());
            let mut figures = SurfaceCombatFigures::from_surface_combat_toml();
            figures.gun_hit_flotation = nm(2);
            r.surface_combat = Some(figures);
        })
        .is_err());
        assert!(build(&|r| {
            r.ship_classes
                .get_mut(&ShipClassId::new(1))
                .expect("class")
                .toughness = Fixed::ZERO;
        })
        .is_err());
    }

    #[test]
    fn the_surface_combat_fields_are_part_of_the_state_hash() {
        let base = world_with_fuel(2000);
        let hash_after = |change: &dyn Fn(&mut World)| {
            let mut changed = base.clone();
            change(&mut changed);
            assert_ne!(base.state_hash(), changed.state_hash());
            changed.state_hash()
        };
        fn ship(world: &mut World) -> &mut Ship {
            world.truth.ships.get_mut(&SHIP).expect("ship")
        }
        fn force(world: &mut World) -> &mut TaskForce {
            world.truth.task_forces.get_mut(&TF).expect("force")
        }

        hash_after(&|w| ship(w).flotation_damage = nm(10));
        hash_after(&|w| ship(w).systems.propulsion = SystemState::Degraded);
        hash_after(&|w| ship(w).systems.torpedo_mounts = SystemState::Out);
        hash_after(&|w| force(w).home_port = Some(crate::ids::BaseId::new(1)));
        hash_after(&|w| ship(w).fire_damage = nm(10));
        hash_after(&|w| ship(w).gun_rounds = nm(10));
        hash_after(&|w| ship(w).torpedoes_ready = true);
        hash_after(&|w| ship(w).torpedoes_one_side_fired = true);
        hash_after(&|w| ship(w).location = ShipLocation::Sunk);
        hash_after(&|w| force(w).readiness = Readiness::Battle);
        hash_after(&|w| force(w).readiness_order = Some(crate::ids::OrderId::new(1)));
        hash_after(&|w| force(w).alerted = true);
        hash_after(&|w| force(w).alerted_by = Some(EventId::new(1)));
        hash_after(&|w| force(w).fatigue = nm(1));
        hash_after(&|w| force(w).rules_of_engagement.stance = Stance::Avoid);
        hash_after(&|w| force(w).rules_of_engagement.break_off = nm_frac(1, 10));
        hash_after(&|w| force(w).rules_order = Some(crate::ids::OrderId::new(1)));
        hash_after(&|w| {
            force(w).rules_of_engagement.stragglers = StragglerInstruction::Retire;
        });
        hash_after(&|w| force(w).rules_of_engagement.keep_share = nm_frac(1, 2));

        // Two tracks with the same value are not interchangeable.
        let mut flotation = base.clone();
        ship(&mut flotation).flotation_damage = nm(10);
        let mut fire = base.clone();
        ship(&mut fire).fire_damage = nm(10);
        assert_ne!(flotation.state_hash(), fire.state_hash());

        // Two systems in the same state are not interchangeable either.
        let mut propulsion = base.clone();
        ship(&mut propulsion).systems.propulsion = SystemState::Out;
        let mut mounts = base.clone();
        ship(&mut mounts).systems.torpedo_mounts = SystemState::Out;
        assert_ne!(propulsion.state_hash(), mounts.state_hash());

        // A claim is part of the hash, and so is each of its fields.
        let claim = BattleClaim {
            hex: hx(5, 0),
            day: 1,
            pulse: Pulse::Night,
            claimed_sunk: 2,
            claimed_hits: 0,
            claimed_kind: None,
            claimed_shot_down: 0,
            claimed_on_ground: 0,
            from_air: false,
            received_event: EventId::new(7),
        };
        let with_claim = |claim: &BattleClaim| {
            let mut world = base.clone();
            world
                .beliefs
                .get_mut(&Side::Allied)
                .expect("belief")
                .apply_report(
                    &Report::BattleClaim(claim.clone()),
                    &SearchFigures::from_search_toml(),
                );
            world.state_hash()
        };
        let claimed = with_claim(&claim);
        assert_ne!(base.state_hash(), claimed);
        assert_eq!(claimed, with_claim(&claim));
        let variants = [
            BattleClaim {
                hex: hx(6, 0),
                ..claim.clone()
            },
            BattleClaim {
                day: 2,
                ..claim.clone()
            },
            BattleClaim {
                pulse: Pulse::Morning,
                ..claim.clone()
            },
            BattleClaim {
                claimed_sunk: 3,
                ..claim.clone()
            },
            BattleClaim {
                claimed_hits: 1,
                ..claim.clone()
            },
            BattleClaim {
                claimed_kind: Some(ShipKind::Carrier),
                ..claim.clone()
            },
            BattleClaim {
                claimed_kind: Some(ShipKind::Cruiser),
                claimed_hits: 1,
                ..claim.clone()
            },
            BattleClaim {
                claimed_shot_down: 1,
                ..claim.clone()
            },
            BattleClaim {
                claimed_on_ground: 1,
                ..claim.clone()
            },
            BattleClaim {
                from_air: true,
                ..claim.clone()
            },
            BattleClaim {
                received_event: EventId::new(8),
                ..claim.clone()
            },
        ];
        for variant in variants {
            assert_ne!(claimed, with_claim(&variant), "{variant:?}");
        }
    }

    // ---- Sunk ships are out of the rules ----

    /// Sinks `ship` the way combat will: location Sunk, full flotation
    /// damage, and gone from its task force's set.
    fn sink(world: &mut World, ship: ShipId) {
        let record = world.truth.ships.get_mut(&ship).expect("ship");
        record.location = ShipLocation::Sunk;
        record.flotation_damage = nm(100);
        for task_force in world.truth.task_forces.values_mut() {
            task_force.ships.remove(&ship);
        }
    }

    fn steamed(world: &World) -> usize {
        world
            .events()
            .iter()
            .filter(|event| matches!(event.kind, EventKind::TaskForceSteamed { .. }))
            .count()
    }

    #[test]
    fn a_task_force_whose_ships_are_all_sunk_does_not_move() {
        let mut world = world_with_fuel(2000);
        sink(&mut world, SHIP);
        assert_sound(&world);
        world.submit_orders(Side::Allied, vec![route_to(12, 0)]);
        world.resolve_day().expect("day resolves");

        // The order is refused because the force has no ship afloat.
        assert_eq!(rejection(&world), Some(RejectReason::NoShipAfloat));
        let truth = world.umpire();
        let task_force = &truth.task_forces[&TF];
        assert_eq!(task_force.position(), hx(0, 0));
        assert_eq!(task_force.toward_hex, None);
        assert_eq!(task_force.progress_nm, Fixed::ZERO);
        assert!(task_force.route.waypoints.is_empty());
        assert_eq!(steamed(&world), 0);
        assert!(world
            .events()
            .iter()
            .all(|event| !matches!(event.kind, EventKind::FuelBurned { .. })));
        assert_eq!(truth.ships[&SHIP].location, ShipLocation::Sunk);
        assert_eq!(truth.ships[&SHIP].fuel_t, Fixed::from_int(2000));
    }

    #[test]
    fn a_task_force_with_some_ships_sunk_moves_and_burns_fuel_for_those_afloat() {
        let mut world = world_with_fuel(2000);
        let second = ShipId::new(2);
        let mut ship = world.truth.ships[&SHIP].clone();
        ship.id = second;
        ship.name = "Second ship".to_string();
        world.truth.ships.insert(second, ship);
        world
            .truth
            .task_forces
            .get_mut(&TF)
            .expect("task force")
            .ships
            .insert(second);
        sink(&mut world, SHIP);
        assert_sound(&world);
        world.submit_orders(Side::Allied, vec![route_to(8, 0)]);
        world.resolve_day().expect("day resolves");

        // 8 hexes = 320 nm at 0.25 t/nm: 80 t for the ship afloat, nothing
        // for the sunk one.
        let truth = world.umpire();
        assert_eq!(truth.task_forces[&TF].status, TaskForceStatus::Arrived);
        assert_eq!(truth.task_forces[&TF].position(), hx(8, 0));
        assert_eq!(truth.ships[&second].fuel_t, Fixed::from_int(2000 - 80));
        assert_eq!(truth.ships[&SHIP].fuel_t, Fixed::from_int(2000));
        let burned: Vec<ShipId> = world
            .events()
            .iter()
            .filter_map(|event| match event.kind {
                EventKind::FuelBurned { ship, .. } => Some(ship),
                _ => None,
            })
            .collect();
        assert!(!burned.is_empty());
        assert!(burned.iter().all(|ship| *ship == second));
    }

    // ---- Movement between hex centres (decision 0005, decisions 1 and 5) ----

    fn hx(q: i32, r: i32) -> Hex {
        Hex::new(q, r)
    }

    /// Turns every hex of column `q` into land, so nothing crosses it.
    fn wall_at_q(world: &mut World, q: i32) {
        for r in -1..=5 {
            world.map.insert(
                hx(q, r),
                HexData {
                    kind: HexKind::Land,
                    ..HexData::open_sea()
                },
            );
        }
    }

    fn nm_frac(num: i64, den: i64) -> Fixed {
        Fixed::from_ratio(num, den)
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

                // cost ahead = (40 - p) + charged miles from toward to W
                // cost back  = p + charged miles from from to W
                // (decision 0058: a step is charged the miles it brings the
                // force closer where the way is clear).
                let charged = |hex: Hex| {
                    nm(i64::from(
                        world
                            .map
                            .sea_charged_nm(hex, waypoint)
                            .expect("the waypoint is reachable"),
                    ))
                };
                let ahead = nm(40 - progress) + charged(toward);
                let back = nm(progress) + charged(from);
                assert_eq!(steamed, ahead.min(back), "p={progress} W={waypoint}");

                // Position-independent bound: no shorter than the trip from
                // the nearer of the two centres.
                let bound = charged(from).min(charged(toward));
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
    fn a_turn_back_keeps_the_charge_of_the_step_and_the_miles_to_go_are_the_charge_less_progress() {
        // A step charged 34 miles, 10 along (decision 0058, 7). Turned
        // back, the force has 34 - 10 = 24 miles to go to the hex it left,
        // on the same charge.
        let mut world = world_with_fuel(2000);
        set_leg(&mut world, hx(0, 0), Some(hx(1, 0)), nm(10));
        let task_force = world.truth.task_forces.get_mut(&TF).expect("task force");
        task_force.leg_nm = nm(34);
        assert_eq!(task_force.position(), hx(0, 0));
        task_force.turn_back();
        assert_eq!(
            (
                task_force.from_hex,
                task_force.toward_hex,
                task_force.progress_nm,
                task_force.leg_nm
            ),
            (hx(1, 0), Some(hx(0, 0)), nm(24), nm(34))
        );
        // 24 is past the midpoint (17): it is in the hex ahead, (0,0).
        assert_eq!(task_force.position(), hx(0, 0));
        task_force.turn_back();
        assert_eq!(
            (task_force.progress_nm, task_force.leg_nm),
            (nm(10), nm(34))
        );
        assert_sound(&world);
    }

    /// The turn-back events in the log, as (task force, from, toward,
    /// progress, position, causes).
    fn turned_back(world: &World) -> Vec<(TaskForceId, Hex, Hex, Fixed, Hex, Vec<Cause>)> {
        world
            .events()
            .iter()
            .filter_map(|event| match &event.kind {
                EventKind::TaskForceTurnedBack {
                    task_force,
                    from,
                    toward,
                    progress_nm,
                    position,
                } => Some((
                    *task_force,
                    *from,
                    *toward,
                    *progress_nm,
                    *position,
                    event.causes.clone(),
                )),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn a_turn_back_mid_leg_records_an_event_that_cites_the_order() {
        let mut world = world_with_fuel(2000);
        set_leg(&mut world, hx(0, 0), Some(hx(1, 0)), nm(10));
        // (0,1) is nearer by turning back (see the test above): the force
        // goes from (1,0) toward (0,0), 30 nm along.
        take_in(&mut world, vec![route_to(0, 1)]);
        assert_eq!(leg(&world), (hx(1, 0), Some(hx(0, 0)), nm(30)));
        let order = world.orders().next().expect("an order").id;
        assert_eq!(
            turned_back(&world),
            vec![(
                TF,
                hx(1, 0),
                hx(0, 0),
                nm(30),
                hx(0, 0),
                vec![Cause::Order(order)]
            )]
        );
        // It follows the acceptance of the order in the log, and reads in the
        // plain style of the movement events.
        let log: Vec<&EventKind> = world.events().iter().map(|e| &e.kind).collect();
        assert!(matches!(log[0], EventKind::OrderAccepted { .. }));
        assert_eq!(
            log[1].to_string(),
            "task_force:1 turned back toward (0,0), 30 nm from (1,0), now in (0,0)"
        );
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn at_exactly_20_nm_the_position_in_the_event_is_the_hex_now_ahead() {
        let mut world = world_with_fuel(2000);
        set_leg(&mut world, hx(0, 0), Some(hx(1, 0)), nm(20));
        // At 20 nm the force is in the hex ahead, (1,0).
        assert_eq!(world.truth.task_forces[&TF].position(), hx(1, 0));
        // (-1,0) is nearer by turning back: from (1,0) toward (0,0), 20 nm.
        take_in(&mut world, vec![route_to(-1, 0)]);
        assert_eq!(leg(&world), (hx(1, 0), Some(hx(0, 0)), nm(20)));
        // The same point on the sea is now in the hex it is heading for.
        assert_eq!(world.truth.task_forces[&TF].position(), hx(0, 0));
        let events = turned_back(&world);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].4, hx(0, 0));
    }

    #[test]
    fn a_route_order_at_a_centre_records_no_turn_back() {
        let mut world = world_with_fuel(2000);
        take_in(&mut world, vec![route_to(3, 0)]);
        assert!(turned_back(&world).is_empty());
        // Going on rather than back records none either.
        let mut world = world_with_fuel(2000);
        set_leg(&mut world, hx(0, 0), Some(hx(1, 0)), nm(10));
        take_in(&mut world, vec![route_to(4, 0)]);
        assert_eq!(leg(&world).0, hx(0, 0));
        assert!(turned_back(&world).is_empty());
    }

    /// An order for a task force with no ship afloat is rejected, whatever
    /// its kind, and changes nothing.
    #[test]
    fn every_order_for_a_force_with_no_ship_afloat_is_rejected_and_changes_nothing() {
        let mut world = world_with_fuel(2000);
        sink(&mut world, SHIP);
        let before = world.truth.task_forces[&TF].clone();
        let orders = vec![
            route_to(3, 0),
            order_for(1, TaskForceOrder::SetSpeed(SpeedSetting::Full)),
            order_for(1, TaskForceOrder::Unload),
            order_for(1, TaskForceOrder::SetReadiness(Readiness::Battle)),
            order_for(1, TaskForceOrder::SetHomePort(BaseId::new(1))),
        ];
        let count = orders.len();
        take_in(&mut world, orders);
        let reasons: Vec<RejectReason> = world
            .events()
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::OrderRejected { reason, .. } => Some(reason.clone()),
                _ => None,
            })
            .collect();
        assert_eq!(reasons, vec![RejectReason::NoShipAfloat; count]);
        assert_eq!(world.truth.task_forces[&TF], before);
        assert_eq!(
            RejectReason::NoShipAfloat.to_string(),
            "the task force has no ship afloat"
        );
    }

    #[test]
    fn an_order_for_an_air_group_lost_with_its_ship_is_rejected() {
        let mut world = world_with_air_group();
        // Aboard the ship, which is afloat: accepted.
        world
            .truth
            .air_groups
            .get_mut(&GROUP)
            .expect("group")
            .location = AirGroupLocation::Ship(SHIP);
        take_in(
            &mut world,
            vec![air_order(
                GROUP,
                AirGroupOrder::SetAirMission(AirMission::Search),
            )],
        );
        assert_eq!(rejection(&world), None);
        // The ship sinks and the group with it: refused, and the mission it
        // had stays.
        sink(&mut world, SHIP);
        take_in(
            &mut world,
            vec![air_order(
                GROUP,
                AirGroupOrder::SetAirMission(AirMission::Rest),
            )],
        );
        assert_eq!(rejection(&world), Some(RejectReason::AirGroupLost));
        assert_eq!(world.truth.air_groups[&GROUP].mission, AirMission::Search);
        assert_eq!(
            RejectReason::AirGroupLost.to_string(),
            "the air group was lost with its ship"
        );
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

        // Unreachable: a wall of land at q = 4 cuts (6,0) off from both the
        // hex ahead and the hex left, so there is no side to go to.
        let mut world = world_with_fuel(2000);
        wall_at_q(&mut world, 4);
        set_leg(&mut world, hx(1, 0), Some(hx(2, 0)), nm(10));
        let before = world.truth.task_forces[&TF].clone();
        take_in(&mut world, vec![route_to(6, 0)]);
        assert_eq!(
            rejection(&world),
            Some(RejectReason::WaypointUnreachable(hx(6, 0)))
        );
        assert_eq!(world.truth.task_forces[&TF], before);

        // Land: (3,2) is on the map but cannot be entered.
        let mut world = world_with_fuel(2000);
        set_leg(&mut world, hx(1, 0), Some(hx(2, 0)), nm(10));
        let before = world.truth.task_forces[&TF].clone();
        take_in(&mut world, vec![route_to(3, 2)]);
        assert_eq!(
            rejection(&world),
            Some(RejectReason::WaypointUnreachable(hx(3, 2)))
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

    // ---- Damage to named systems, and stragglers (decision 0016) ----

    const ENEMY_BASE: BaseId = BaseId::new(2);

    /// The one-ship test world with surface combat figures (degraded factor
    /// 1/2) and an enemy base at (1, 0), so propulsion can be damaged.
    fn damage_world() -> World {
        let good = world_with_fuel(2000);
        let mut ref_data = good.ref_data.clone();
        ref_data.search = Some(SearchFigures::from_search_toml());
        ref_data.surface_combat = Some(SurfaceCombatFigures::from_surface_combat_toml());
        let mut truth = good.truth.clone();
        truth.bases.insert(
            ENEMY_BASE,
            Base::new(
                ENEMY_BASE,
                "Enemy base".to_string(),
                Side::Japanese,
                hx(1, 0),
            ),
        );
        World::new(7, ref_data, good.map.clone(), truth).expect("valid setup")
    }

    /// Adds a copy of ship 1 as ship `raw` to task force 1, with its
    /// propulsion in `state`.
    fn add_ship(world: &mut World, raw: u32, state: SystemState) -> ShipId {
        let id = ShipId::new(raw);
        let mut ship = world.truth.ships[&SHIP].clone();
        ship.id = id;
        ship.name = format!("Ship {raw}");
        ship.systems.propulsion = state;
        world.truth.ships.insert(id, ship);
        world
            .truth
            .task_forces
            .get_mut(&TF)
            .expect("task force")
            .ships
            .insert(id);
        id
    }

    fn set_propulsion(world: &mut World, ship: ShipId, state: SystemState) {
        world
            .truth
            .ships
            .get_mut(&ship)
            .expect("ship")
            .systems
            .propulsion = state;
    }

    fn steamed_speeds(world: &World) -> Vec<Fixed> {
        world
            .events()
            .iter()
            .filter_map(|event| match &event.kind {
                EventKind::TaskForceSteamed { speed_kn, .. } => Some(*speed_kn),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn force_speed_follows_each_ships_propulsion() {
        // The class steams at 16 kn at cruise and 32 kn at full. The
        // degraded factor is 1/2, out is 0.
        //   working:  16 x 1   = 16 kn;  full 32 x 1   = 32 kn
        //   degraded: 16 x 1/2 = 8 kn;   full 32 x 1/2 = 16 kn
        //   out:      16 x 0   = 0 kn
        // A force goes at its slowest ship: working + degraded = 8 kn.
        for (state, setting, expected) in [
            (SystemState::Working, SpeedSetting::Cruise, 16),
            (SystemState::Degraded, SpeedSetting::Cruise, 8),
            (SystemState::Working, SpeedSetting::Full, 32),
            (SystemState::Degraded, SpeedSetting::Full, 16),
        ] {
            let mut world = damage_world();
            set_propulsion(&mut world, SHIP, state);
            world.submit_orders(
                Side::Allied,
                vec![route_to(12, 0), order(TaskForceOrder::SetSpeed(setting))],
            );
            world.resolve_day().expect("day resolves");
            let speeds = steamed_speeds(&world);
            assert!(!speeds.is_empty());
            assert!(speeds.iter().all(|s| *s == Fixed::from_int(expected)));
        }

        let mut world = damage_world();
        add_ship(&mut world, 2, SystemState::Degraded);
        world.submit_orders(Side::Allied, vec![route_to(12, 0)]);
        world.resolve_day().expect("day resolves");
        assert!(steamed_speeds(&world)
            .iter()
            .all(|s| *s == Fixed::from_int(8)));
    }

    #[test]
    fn the_steamed_breakdown_names_the_propulsion_factor_of_the_slowest_ship() {
        // Ships 1 and 3 work (factor 1); ship 2 is degraded (factor 1/2) and
        // sets the speed.
        let mut world = damage_world();
        add_ship(&mut world, 2, SystemState::Degraded);
        add_ship(&mut world, 3, SystemState::Working);
        world.submit_orders(Side::Allied, vec![route_to(12, 0)]);
        world.resolve_day().expect("day resolves");
        let event = world
            .events()
            .iter()
            .find(|e| matches!(e.kind, EventKind::TaskForceSteamed { .. }))
            .expect("steamed");
        let breakdown = event.breakdown.as_ref().expect("breakdown");
        let factor = |name: &str| {
            breakdown
                .factors
                .iter()
                .find(|f| f.name == name)
                .expect("factor")
                .value
        };
        assert_eq!(factor("propulsion_factor"), nm_frac(1, 2));
        assert_eq!(factor("speed_kn"), nm(8));
    }

    #[test]
    fn a_force_dead_in_the_water_stays_underway_and_burns_nothing() {
        // The second ship is out: 16 kn x 0 = 0 kn, so the force speed is 0.
        // Nothing moves, no event is recorded and no fuel is burned.
        let mut world = damage_world();
        add_ship(&mut world, 2, SystemState::Out);
        world.submit_orders(Side::Allied, vec![route_to(12, 0)]);
        world.resolve_day().expect("day resolves");
        world.resolve_day().expect("day resolves");

        let truth = world.umpire();
        let force = &truth.task_forces[&TF];
        assert_eq!(force.status, TaskForceStatus::Underway);
        assert_eq!(force.position(), hx(0, 0));
        assert_eq!(force.toward_hex, None);
        assert_eq!(force.progress_nm, Fixed::ZERO);
        assert_eq!(force.route.next_index, 0);
        assert_eq!(steamed(&world), 0);
        assert!(world.events().iter().all(|event| !matches!(
            event.kind,
            EventKind::FuelBurned { .. } | EventKind::TaskForceArrived { .. }
        )));
        for ship in truth.ships.values() {
            assert_eq!(ship.fuel_t, Fixed::from_int(2000));
        }
        assert_sound(&world);
    }

    #[test]
    fn half_speed_burns_the_cruise_rate_per_mile() {
        // Degraded: 16 kn x 1/2 = 8 kn, below cruise, so the multiplier
        // floors at 1 and the burn is 0.25 t per nm.
        // Night pulse: 12 h x 8 kn = 96 nm, burning 96 x 0.25 = 24 t.
        // Whole day: 24 h x 8 kn = 192 nm, 48 t, which is 4 hexes and 32 nm
        // along the leg to hex 5: the position is hex 5 (32 nm is over 20).
        let mut world = damage_world();
        set_propulsion(&mut world, SHIP, SystemState::Degraded);
        world.submit_orders(Side::Allied, vec![route_to(12, 0)]);
        world.resolve_day().expect("day resolves");

        let night = world
            .events()
            .iter()
            .find_map(|event| match &event.kind {
                EventKind::TaskForceSteamed { distance_nm, .. } => Some(*distance_nm),
                _ => None,
            })
            .expect("steamed");
        assert_eq!(night, nm(96));
        let first_burn = world
            .events()
            .iter()
            .find_map(|event| match &event.kind {
                EventKind::FuelBurned { burned_t, .. } => Some(*burned_t),
                _ => None,
            })
            .expect("burned");
        assert_eq!(first_burn, nm(24));

        let truth = world.umpire();
        assert_eq!(truth.ships[&SHIP].fuel_t, nm(2000 - 48));
        let force = &truth.task_forces[&TF];
        assert_eq!(force.from_hex, hx(4, 0));
        assert_eq!(force.toward_hex, Some(hx(5, 0)));
        assert_eq!(force.progress_nm, nm(32));
        assert_eq!(force.position(), hx(5, 0));
    }

    #[test]
    fn breakdown_recording_does_not_change_the_hash_with_a_damaged_ship() {
        let run = |record_breakdowns: bool| {
            let mut world = damage_world();
            add_ship(&mut world, 2, SystemState::Degraded);
            world.set_config(crate::SimConfig { record_breakdowns });
            world.submit_orders(Side::Allied, vec![route_to(12, 0)]);
            world.resolve_day().expect("day resolves");
            world.state_hash()
        };
        assert_eq!(run(true), run(false));
    }

    #[test]
    fn world_new_rejects_a_damaged_system_without_surface_combat_figures() {
        let good = world_with_fuel(2000);
        let mut truth = good.truth.clone();
        truth.ships.get_mut(&SHIP).expect("ship").systems.propulsion = SystemState::Degraded;
        let error = World::new(1, good.ref_data.clone(), good.map.clone(), truth.clone())
            .expect_err("rejected");
        assert!(matches!(error, SimError::InvalidSetup(_)), "{error}");
        let mut ref_data = good.ref_data.clone();
        ref_data.search = Some(SearchFigures::from_search_toml());
        ref_data.surface_combat = Some(SurfaceCombatFigures::from_surface_combat_toml());
        assert!(World::new(1, ref_data, good.map.clone(), truth).is_ok());
    }

    fn full_roe(share: Fixed, stragglers: StragglerInstruction) -> OrderRequest {
        order(TaskForceOrder::SetRulesOfEngagement {
            stance: Stance::Avoid,
            break_off: nm_frac(1, 4),
            stragglers,
            keep_share: share,
        })
    }

    /// The rejection reasons of every order in the log, in order.
    fn rejections(world: &World) -> Vec<RejectReason> {
        world
            .events()
            .iter()
            .filter_map(|event| match &event.kind {
                EventKind::OrderRejected { reason, .. } => Some(reason.clone()),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn the_straggler_instruction_and_keep_share_are_accepted_and_stored() {
        let mut world = damage_world();
        let rules = world.truth.task_forces[&TF].rules_of_engagement;
        assert_eq!(rules.stragglers, StragglerInstruction::Keep);
        assert_eq!(rules.keep_share, Fixed::ZERO);
        world.submit_orders(
            Side::Allied,
            vec![full_roe(nm_frac(1, 2), StragglerInstruction::Detach)],
        );
        world.resolve_day().expect("day resolves");
        assert_eq!(rejections(&world), Vec::new());
        let rules = world.truth.task_forces[&TF].rules_of_engagement;
        assert_eq!(rules.stragglers, StragglerInstruction::Detach);
        assert_eq!(rules.keep_share, nm_frac(1, 2));
        assert_eq!(rules.break_off, nm_frac(1, 4));
        assert_eq!(rules.stance, Stance::Avoid);
        assert_eq!(StragglerInstruction::Keep.code(), 1);
        assert_eq!(StragglerInstruction::Detach.code(), 2);
        assert_eq!(StragglerInstruction::Leave.code(), 3);
        assert_eq!(StragglerInstruction::Retire.code(), 4);
    }

    #[test]
    fn keep_shares_of_zero_and_one_are_accepted_and_others_rejected() {
        let mut world = damage_world();
        world.submit_orders(
            Side::Allied,
            vec![
                full_roe(Fixed::ZERO, StragglerInstruction::Leave),
                full_roe(Fixed::ONE, StragglerInstruction::Retire),
                full_roe(nm_frac(-1, 100), StragglerInstruction::Leave),
                full_roe(nm_frac(101, 100), StragglerInstruction::Leave),
                // break_off is checked first.
                order(TaskForceOrder::SetRulesOfEngagement {
                    stance: Stance::Engage,
                    break_off: nm(2),
                    stragglers: StragglerInstruction::Keep,
                    keep_share: nm(2),
                }),
            ],
        );
        world.resolve_day().expect("day resolves");
        assert_eq!(
            rejections(&world),
            vec![
                RejectReason::KeepShareOutOfRange(nm_frac(-1, 100)),
                RejectReason::KeepShareOutOfRange(nm_frac(101, 100)),
                RejectReason::BreakOffOutOfRange(nm(2)),
            ]
        );
        // The last accepted order stands: retire, keep 1.
        let rules = world.truth.task_forces[&TF].rules_of_engagement;
        assert_eq!(rules.stragglers, StragglerInstruction::Retire);
        assert_eq!(rules.keep_share, Fixed::ONE);
    }

    #[test]
    fn a_home_port_order_needs_a_base_of_your_side() {
        let mut world = damage_world();
        let home = |base: u32| order(TaskForceOrder::SetHomePort(BaseId::new(base)));
        // Unknown base 9 and enemy base 2 give the same reason.
        world.submit_orders(Side::Allied, vec![home(9), home(2), home(1)]);
        world.resolve_day().expect("day resolves");
        assert_eq!(
            rejections(&world),
            vec![RejectReason::NotYourBase, RejectReason::NotYourBase]
        );
        assert_eq!(world.truth.task_forces[&TF].home_port, Some(BaseId::new(1)));
        assert_sound(&world);
        // The other side cannot use the Allied base either.
        let mut world = damage_world();
        world.truth.task_forces.get_mut(&TF).expect("force").side = Side::Japanese;
        for ship in world.truth.ships.values_mut() {
            ship.side = Side::Japanese;
        }
        world.submit_orders(Side::Japanese, vec![home(1), home(2)]);
        world.resolve_day().expect("day resolves");
        assert_eq!(rejections(&world), vec![RejectReason::NotYourBase]);
        assert_eq!(world.truth.task_forces[&TF].home_port, Some(BaseId::new(2)));
    }

    #[test]
    fn detach_orders_are_checked_in_order_and_accepted_when_sound() {
        let mut world = damage_world();
        let second = add_ship(&mut world, 2, SystemState::Working);
        let third = add_ship(&mut world, 3, SystemState::Working);
        let detach = |ships: Vec<ShipId>| order(TaskForceOrder::DetachShips(ships));
        sink(&mut world, third);
        world.submit_orders(
            Side::Allied,
            vec![
                detach(vec![]),
                // Ship 9 does not exist; ship 3 is sunk. The first listed
                // ship that is not afloat in the force is reported.
                detach(vec![second, ShipId::new(9), third]),
                detach(vec![third, ShipId::new(9)]),
                // Every ship afloat in the force, duplicates and all.
                detach(vec![second, SHIP, second]),
                detach(vec![SHIP, second]),
                // A good one, with a duplicate.
                detach(vec![second, second]),
            ],
        );
        world.resolve_day().expect("day resolves");
        assert_eq!(
            rejections(&world),
            vec![
                RejectReason::NoShipsToDetach,
                RejectReason::ShipNotInTaskForce(ShipId::new(9)),
                RejectReason::ShipNotInTaskForce(third),
                RejectReason::WouldLeaveTaskForceEmpty,
                RejectReason::WouldLeaveTaskForceEmpty,
            ]
        );
        let truth = world.umpire();
        assert_eq!(truth.task_forces.len(), 2);
        assert_eq!(
            truth.task_forces[&TF].ships,
            [SHIP].into_iter().collect::<BTreeSet<_>>()
        );
        let new = &truth.task_forces[&TaskForceId::new(1_000_001)];
        assert_eq!(new.name, "Test force, detached");
        assert_eq!(new.ships, [second].into_iter().collect::<BTreeSet<_>>());
        assert_eq!(
            truth.ships[&second].location,
            ShipLocation::TaskForce(new.id)
        );
        // The accepted order is the record: its target is the parent.
        let accepted: Vec<_> = world
            .events()
            .iter()
            .filter(|e| matches!(e.kind, EventKind::OrderAccepted { .. }))
            .collect();
        assert_eq!(accepted.len(), 1);
        assert!(matches!(
            accepted[0].kind,
            EventKind::OrderAccepted { target, .. } if target == crate::ids::EntityRef::TaskForce(TF)
        ));
        assert_sound(&world);
    }

    #[test]
    fn a_detached_force_starts_mid_leg_with_the_parents_settings() {
        let mut world = damage_world();
        let second = add_ship(&mut world, 2, SystemState::Working);
        {
            let force = world.truth.task_forces.get_mut(&TF).expect("force");
            force.from_hex = hx(2, 0);
            force.toward_hex = Some(hx(3, 0));
            force.progress_nm = nm(13);
            force.status = TaskForceStatus::Underway;
            force.route = Route {
                waypoints: vec![hx(9, 0)],
                next_index: 0,
            };
            force.readiness = Readiness::Battle;
            force.readiness_order = Some(crate::ids::OrderId::new(44));
            force.alerted = true;
            force.alerted_by = Some(EventId::new(4));
            force.fatigue = nm(35);
            force.home_port = Some(BaseId::new(1));
            force.mission = Mission::SurfaceCombat;
            force.speed = SpeedSetting::Full;
            force.rules_of_engagement = RulesOfEngagement {
                stance: Stance::Avoid,
                break_off: nm_frac(1, 4),
                stragglers: StragglerInstruction::Retire,
                keep_share: nm_frac(3, 4),
            };
            force.rules_order = Some(crate::ids::OrderId::new(40));
            force.speed_order = Some(crate::ids::OrderId::new(41));
            force.route_order = Some(crate::ids::OrderId::new(42));
            force.cargo_order = Some(CargoOrder {
                action: CargoAction::Load,
                order: crate::ids::OrderId::new(43),
            });
            force.arrived_at_hours = Some(nm(5));
            force.arrival_event = Some(EventId::new(3));
        }
        assert_sound(&world);
        let parent_before = world.truth.task_forces[&TF].clone();
        let ids = [second].into_iter().collect::<BTreeSet<_>>();
        let new_id = world.split_task_force(TF, &ids, "Split".to_string());
        assert_sound(&world);

        let truth = world.umpire();
        let new = &truth.task_forces[&new_id];
        assert_eq!(new_id, TaskForceId::new(1_000_001));
        assert_eq!(new.name, "Split");
        assert_eq!(new.side, Side::Allied);
        assert_eq!(new.from_hex, hx(2, 0));
        assert_eq!(new.toward_hex, Some(hx(3, 0)));
        assert_eq!(new.progress_nm, nm(13));
        assert_eq!(new.position(), hx(2, 0));
        assert_eq!(new.status, TaskForceStatus::Idle);
        assert_eq!(new.route, Route::default());
        assert_eq!(new.route_order, None);
        assert_eq!(new.cargo_order, None);
        assert_eq!(new.arrived_at_hours, None);
        assert_eq!(new.arrival_event, None);
        assert_eq!(new.readiness, Readiness::Battle);
        assert_eq!(new.readiness_order, Some(crate::ids::OrderId::new(44)));
        assert!(new.alerted);
        assert_eq!(new.alerted_by, Some(EventId::new(4)));
        // A detachment is as tired as the force it left (decision 0037).
        assert_eq!(new.fatigue, nm(35));
        assert_eq!(new.home_port, Some(BaseId::new(1)));
        assert_eq!(new.mission, Mission::SurfaceCombat);
        assert_eq!(new.speed, SpeedSetting::Full);
        assert_eq!(new.speed_order, parent_before.speed_order);
        assert_eq!(new.rules_of_engagement, parent_before.rules_of_engagement);
        assert_eq!(new.rules_order, parent_before.rules_order);
        assert_eq!(new.ships, ids);
        assert_eq!(
            truth.ships[&second].location,
            ShipLocation::TaskForce(new_id)
        );
        // The parent keeps everything but the ship.
        let mut expected = parent_before;
        expected.ships.remove(&second);
        assert_eq!(truth.task_forces[&TF], expected);
        // Splitting records no event.
        assert!(world.events().iter().next().is_none());
    }

    #[test]
    fn created_task_forces_get_the_next_ids_and_the_counter_is_hashed() {
        // The Allied series starts at 1,000,001, whatever the scenario holds:
        // the first new force is 1,000,001, then 1,000,002.
        let mut world = damage_world();
        assert_eq!(world.made_ids.next_task_force(Side::Allied), 1_000_001);
        assert_eq!(world.made_ids.next_task_force(Side::Japanese), 2_000_001);
        let a = add_ship(&mut world, 2, SystemState::Working);
        let b = add_ship(&mut world, 3, SystemState::Working);
        let first = world.split_task_force(TF, &[a].into_iter().collect(), "A".to_string());
        let second = world.split_task_force(TF, &[b].into_iter().collect(), "B".to_string());
        assert_eq!(
            (first, second),
            (TaskForceId::new(1_000_001), TaskForceId::new(1_000_002))
        );
        assert_eq!(world.made_ids.next_task_force(Side::Allied), 1_000_003);
        // The Japanese series is not touched by the Allied side's forces.
        assert_eq!(world.made_ids.next_task_force(Side::Japanese), 2_000_001);

        // A scenario whose highest force ID is 7 still hands out 1,000,001.
        let mut high = damage_world();
        let mut moved = high.truth.task_forces.remove(&TF).expect("force");
        moved.id = TaskForceId::new(7);
        high.truth.task_forces.insert(moved.id, moved);
        for ship in high.truth.ships.values_mut() {
            ship.location = ShipLocation::TaskForce(TaskForceId::new(7));
        }
        let rebuilt = World::new(
            7,
            high.ref_data.clone(),
            high.map.clone(),
            high.truth.clone(),
        )
        .expect("valid");
        assert_eq!(rebuilt.made_ids.next_task_force(Side::Allied), 1_000_001);

        // Two worlds identical but for a counter hash differently.
        let base = damage_world();
        let mut counted = base.clone();
        counted.made_ids.alloc_task_force(Side::Allied);
        assert_eq!(base.truth, counted.truth);
        assert_ne!(base.state_hash(), counted.state_hash());
        let mut japanese = base.clone();
        japanese.made_ids.alloc_task_force(Side::Japanese);
        assert_ne!(base.state_hash(), japanese.state_hash());
        assert_ne!(counted.state_hash(), japanese.state_hash());
        let mut group = base.clone();
        group.made_ids.alloc_air_group(Side::Allied);
        assert_ne!(base.state_hash(), group.state_hash());
    }

    #[test]
    fn a_staff_order_taken_in_mid_day_is_stored_and_cites_its_causes() {
        let mut world = damage_world();
        let cause = world.events.record(
            1,
            Some(Pulse::Morning),
            EventKind::TaskForceOutOfFuel {
                task_force: TF,
                hex: hx(0, 0),
            },
            Vec::new(),
            None,
        );
        let staff = OrderRequest {
            kind: OrderKind::TaskForce {
                target: TF,
                order: TaskForceOrder::SetRoute(vec![hx(2, 0)]),
            },
            origin: Origin {
                issuer: crate::orders::Issuer::Staff,
                intent: None,
            },
        };
        let (id, event, accepted) = world.take_in_order(
            1,
            Some(Pulse::Morning),
            Side::Allied,
            staff,
            vec![Cause::Event(cause)],
        );
        assert!(accepted);
        let stored = world.order(id).expect("stored");
        assert_eq!(stored.origin.issuer, crate::orders::Issuer::Staff);
        assert_eq!(stored.side, Side::Allied);
        assert_eq!(stored.day_issued, 1);
        let recorded = world.events().get(event).expect("event");
        assert_eq!(recorded.pulse, Some(Pulse::Morning));
        assert_eq!(recorded.day, 1);
        assert_eq!(recorded.causes, vec![Cause::Order(id), Cause::Event(cause)]);
        assert!(matches!(
            recorded.kind,
            EventKind::OrderAccepted { order, .. } if order == id
        ));
        let force = &world.truth.task_forces[&TF];
        assert_eq!(force.status, TaskForceStatus::Underway);
        assert_eq!(force.route_order, Some(id));

        // A rejection returns false and cites the same causes.
        let bad = order(TaskForceOrder::SetHomePort(ENEMY_BASE));
        let (id, event, accepted) =
            world.take_in_order(1, Some(Pulse::Morning), Side::Allied, bad, vec![]);
        assert!(!accepted);
        assert_eq!(
            world.events().get(event).expect("event").causes,
            vec![Cause::Order(id)]
        );
    }

    // Pathfinding at intake and in the turn-back choice (decision 0017).

    /// Sets hexes to land on the test map.
    fn make_land(world: &mut World, hexes: &[(i32, i32)]) {
        for (q, r) in hexes {
            world.map.insert(
                hx(*q, *r),
                HexData {
                    kind: HexKind::Land,
                    ..HexData::open_sea()
                },
            );
        }
    }

    #[test]
    fn a_pocket_makes_the_force_turn_back_though_the_straight_line_says_ahead() {
        // Test map: q -1 to 12, r -1 to 5, land already at (3,2). Add land at
        // (3,1), (2,1), (1,2) and (1,3), so the hex (2,2) is a pocket: its
        // only sea neighbour is (2,3).
        //
        //   r=-1:     .  .  W  .  .          W = (2,-1)
        //   r= 0:    .  .  .  .  .
        //   r= 1:   .  .  #  #  .            (2,1) (3,1) land
        //   r= 2:  .  #  P  #  .             (1,2) land, P = (2,2), (3,2) land
        //   r= 3: .  #  F  .  .              (1,3) land, F = (2,3)
        //
        // The force is on the leg F (2,3) -> P (2,2), 10 nm out, ordered to W.
        // Straight line: P is 3 from W and F is 4, so
        //   ahead = (40 - 10) + 40 x 3 = 150, back = 10 + 40 x 4 = 170:
        //   the old rule goes ahead.
        // By sea: F is 7 from W ((3,3), (4,2), (4,1), (4,0), (3,0), (3,-1),
        // (2,-1)) and P is 8 (it must go back through F), so
        //   ahead = 30 + 40 x 8 = 350, back = 10 + 40 x 7 = 290:
        //   it turns back.
        let mut world = world_with_fuel(2000);
        make_land(&mut world, &[(3, 1), (2, 1), (1, 2), (1, 3)]);
        set_leg(&mut world, hx(2, 3), Some(hx(2, 2)), nm(10));
        let waypoint = hx(2, -1);
        assert_eq!(world.map.sea_distance(hx(2, 3), waypoint), Some(7));
        assert_eq!(world.map.sea_distance(hx(2, 2), waypoint), Some(8));
        assert_eq!(hx(2, 2).distance(waypoint), 3);
        assert_eq!(hx(2, 3).distance(waypoint), 4);

        let task_force = &world.truth.task_forces[&TF];
        assert_eq!(
            task_force.reroute_centre(&world.map, Some(waypoint)),
            Some((true, hx(2, 3)))
        );

        take_in(
            &mut world,
            vec![order(TaskForceOrder::SetRoute(vec![waypoint]))],
        );
        assert_eq!(rejection(&world), None);
        // Turned round: the two hexes swap and 40 - 10 = 30 nm are covered.
        assert_eq!(leg(&world), (hx(2, 2), Some(hx(2, 3)), nm(30)));
        assert_sound(&world);
    }

    #[test]
    fn the_turn_back_choice_takes_the_only_side_that_can_reach_the_waypoint() {
        // Two adjacent hexes you can enter are always in the same part of the
        // sea, so in a sound state either both reach the waypoint or neither
        // does. The one-sided cases are therefore tested on the function
        // itself, with a state the invariants would reject: the hex ahead, or
        // the hex left, is land.
        let mut world = world_with_fuel(2000);
        let waypoint = hx(8, 0);
        set_leg(&mut world, hx(1, 0), Some(hx(2, 0)), nm(10));
        // Open sea: ahead = 30 + 40 x 6 = 270, back = 10 + 40 x 7 = 290.
        let task_force = world.truth.task_forces[&TF].clone();
        assert_eq!(
            task_force.reroute_centre(&world.map, Some(waypoint)),
            Some((false, hx(2, 0)))
        );

        // The hex ahead is land: it cannot reach the waypoint, so the force
        // turns back, though ahead would otherwise win.
        make_land(&mut world, &[(2, 0)]);
        assert_eq!(
            task_force.reroute_centre(&world.map, Some(waypoint)),
            Some((true, hx(1, 0)))
        );

        // The hex left is land: back cannot reach it. Put the force where
        // turning back would win on distance (1 nm out, waypoint behind it),
        // and it still goes ahead.
        let mut world = world_with_fuel(2000);
        make_land(&mut world, &[(1, 0)]);
        let mut task_force = task_force;
        task_force.progress_nm = nm(1);
        assert_eq!(
            task_force.reroute_centre(&world.map, Some(hx(0, 0))),
            Some((false, hx(2, 0)))
        );

        // Neither side can reach it: no choice, so the order is rejected.
        make_land(&mut world, &[(2, 0)]);
        assert_eq!(task_force.reroute_centre(&world.map, Some(hx(0, 0))), None);
        // At a centre there is no choice to make, whatever the map.
        task_force.toward_hex = None;
        task_force.progress_nm = Fixed::ZERO;
        assert_eq!(
            task_force.reroute_centre(&world.map, Some(hx(0, 0))),
            Some((false, hx(1, 0)))
        );
    }

    #[test]
    fn a_waypoint_off_the_map_is_rejected_even_mid_leg() {
        // The off-map check comes before the turn-back choice, so the reason
        // is the same between centres as at one.
        for leg_state in [None, Some((hx(1, 0), hx(2, 0), 10))] {
            let mut world = world_with_fuel(2000);
            if let Some((from, toward, progress)) = leg_state {
                set_leg(&mut world, from, Some(toward), nm(progress));
            }
            take_in(&mut world, vec![route_to(40, 40)]);
            assert_eq!(
                rejection(&world),
                Some(RejectReason::WaypointOffMap(hx(40, 40)))
            );
        }
    }

    #[test]
    fn a_waypoint_across_land_with_no_way_round_is_rejected() {
        // Land across the whole strip at q = 4: (6,0) is on the map, ships
        // are in the part of the sea at q 3 or less, and nothing links them.
        // A later waypoint is checked from the one before: (2,0) is fine,
        // (6,0) after it is not.
        let mut world = world_with_fuel(2000);
        make_land(
            &mut world,
            &[(4, -1), (4, 0), (4, 1), (4, 2), (4, 3), (4, 4), (4, 5)],
        );
        take_in(
            &mut world,
            vec![order(TaskForceOrder::SetRoute(vec![hx(2, 0), hx(6, 0)]))],
        );
        assert_eq!(
            rejection(&world),
            Some(RejectReason::WaypointUnreachable(hx(6, 0)))
        );
        assert_eq!(world.truth.task_forces[&TF].status, TaskForceStatus::Idle);
        // The message says why.
        assert_eq!(
            RejectReason::WaypointUnreachable(hx(6, 0)).to_string(),
            "waypoint (6,0) cannot be reached by sea"
        );
    }

    #[test]
    fn a_land_waypoint_is_rejected_as_unreachable() {
        // (3,2) is land and on the map: not off the map, but unreachable.
        let mut world = world_with_fuel(2000);
        take_in(&mut world, vec![route_to(3, 2)]);
        assert_eq!(
            rejection(&world),
            Some(RejectReason::WaypointUnreachable(hx(3, 2)))
        );
    }

    #[test]
    fn a_force_that_must_go_round_land_does_so_and_arrives() {
        // Wall (3,-1) to (3,2) with the strip open at r = 3: from (0,0) to
        // (6,0) the force has to go down to r = 3 and back. The route is
        // accepted and finished: the movement loop uses the path.
        let mut world = world_with_fuel(2000);
        make_land(&mut world, &[(3, -1), (3, 0), (3, 1), (3, 2)]);
        let steps = world
            .map
            .sea_distance(hx(0, 0), hx(6, 0))
            .expect("reachable");
        assert!(steps > 6);
        world.submit_orders(Side::Allied, vec![route_to(6, 0)]);
        for _ in 0..3 {
            world.resolve_day().expect("day resolves");
        }
        assert_eq!(rejection(&world), None);
        assert_eq!(
            world.truth.task_forces[&TF].status,
            TaskForceStatus::Arrived
        );
        assert_eq!(world.truth.task_forces[&TF].position(), hx(6, 0));
        // The way round is a detour, so a hex a step at first and the
        // straight miles once clear of the wall (decision 0058), at 0.25
        // t/nm.
        let charged = i64::from(
            world
                .map
                .sea_charged_nm(hx(0, 0), hx(6, 0))
                .expect("reachable"),
        );
        assert!(charged > 6 * 40 && charged <= 40 * i64::from(steps));
        assert_eq!(
            world.truth.ships[&SHIP].fuel_t,
            Fixed::from_ratio(2000 * 4 - charged, 4)
        );
        assert_sound(&world);
    }

    #[test]
    fn a_force_in_a_hex_ships_cannot_enter_breaks_an_invariant() {
        let mut world = world_with_fuel(2000);
        make_land(&mut world, &[(0, 0)]);
        assert_eq!(
            invariants::check(&world.truth, &world.map, &world.ref_data),
            vec![crate::invariants::InvariantViolation::TaskForceNotNavigable(TF, hx(0, 0))]
        );
        // The hex ahead too.
        let mut world = world_with_fuel(2000);
        make_land(&mut world, &[(1, 0)]);
        let task_force = world.truth.task_forces.get_mut(&TF).expect("task force");
        task_force.toward_hex = Some(hx(1, 0));
        task_force.progress_nm = nm(5);
        assert_eq!(
            invariants::check(&world.truth, &world.map, &world.ref_data),
            vec![crate::invariants::InvariantViolation::TaskForceNotNavigable(TF, hx(1, 0))]
        );
        assert_eq!(
            crate::invariants::InvariantViolation::TaskForceNotNavigable(TF, hx(1, 0)).to_string(),
            format!("{TF} is at (1,0), which ships cannot enter")
        );
    }
    // ---- the order orders are taken in (decision 0043, 4a to 4b) ----

    fn order_for(target: u32, order: TaskForceOrder) -> OrderRequest {
        OrderRequest {
            kind: OrderKind::TaskForce {
                target: TaskForceId::new(target),
                order,
            },
            origin: Origin::PLAYER,
        }
    }

    fn two_force_world() -> World {
        let mut world = world_with(7, 2000);
        let mut truth = world.umpire().clone();
        let mut second = TaskForce::at(
            TaskForceId::new(2),
            "Second".to_string(),
            Side::Allied,
            Hex::new(0, 1),
            Mission::Transport,
        );
        // A force with no ship afloat takes no orders, so it has a ship.
        let mut ship = truth.ships[&SHIP].clone();
        ship.id = ShipId::new(2);
        ship.location = ShipLocation::TaskForce(TaskForceId::new(2));
        second.ships.insert(ship.id);
        truth.ships.insert(ship.id, ship);
        truth.task_forces.insert(TaskForceId::new(2), second);
        world.truth = truth;
        world
    }

    #[test]
    fn orders_are_numbered_by_unit_whatever_order_they_were_handed_in() {
        let route = |q| TaskForceOrder::SetRoute(vec![Hex::new(q, 0)]);
        let knots_order = |k| TaskForceOrder::SetSpeed(SpeedSetting::Knots(nm(k)));
        let a = vec![
            order_for(2, route(5)),
            order_for(1, route(6)),
            order_for(1, knots_order(16)),
            order_for(2, knots_order(12)),
        ];
        let b = vec![
            order_for(1, route(6)),
            order_for(1, knots_order(16)),
            order_for(2, route(5)),
            order_for(2, knots_order(12)),
        ];
        let mut results = Vec::new();
        for requests in [a, b] {
            let mut world = two_force_world();
            // Handed in as two calls, too: the split must not matter.
            let (first, second) = requests.split_at(1);
            world.submit_orders(Side::Allied, first.to_vec());
            world.submit_orders(Side::Allied, second.to_vec());
            world.resolve_day().expect("day resolves");
            let events: Vec<_> = world.events().iter().cloned().collect();
            let orders: Vec<_> = world.orders().cloned().collect();
            results.push((world.state_hash(), events, orders));
        }
        assert_eq!(results[0], results[1]);
        // Forces 1 first, each force's orders in the order given.
        let orders = &results[0].2;
        assert_eq!(orders[0].id, OrderId::new(1));
        assert_eq!(
            orders[0].kind.target(),
            crate::ids::EntityRef::TaskForce(TF)
        );
        assert_eq!(
            orders[1].kind.target(),
            crate::ids::EntityRef::TaskForce(TF)
        );
        assert!(matches!(
            orders[0].kind,
            OrderKind::TaskForce {
                order: TaskForceOrder::SetRoute(_),
                ..
            }
        ));
        assert_eq!(
            orders[2].kind.target(),
            crate::ids::EntityRef::TaskForce(TaskForceId::new(2))
        );
    }

    #[test]
    fn one_units_orders_keep_their_order_so_the_later_one_wins() {
        let mut world = two_force_world();
        world.submit_orders(
            Side::Allied,
            vec![
                order_for(1, TaskForceOrder::SetSpeed(SpeedSetting::Knots(nm(10)))),
                order_for(2, TaskForceOrder::SetSpeed(SpeedSetting::Knots(nm(11)))),
                order_for(1, TaskForceOrder::SetSpeed(SpeedSetting::Knots(nm(14)))),
            ],
        );
        world.resolve_day().expect("day resolves");
        assert_eq!(
            world.truth.task_forces[&TF].speed,
            SpeedSetting::Knots(nm(14))
        );
        assert_eq!(
            world.truth.task_forces[&TaskForceId::new(2)].speed,
            SpeedSetting::Knots(nm(11))
        );
    }
}
