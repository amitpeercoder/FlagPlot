//! An escort force (decision 0121): a task force on the escort mission with
//! a charge keeps station on it, is met before it, and fires its guns with
//! the charge's at a raid on it.
//!
//! This module holds keeping station (rule 3), the loss of a charge (rule
//! 7), and what the other rules read: which escorts are in station on a
//! charge (rule 2). Who is met first (rules 4 and 5) is in the movement step
//! (`movement.rs`, `World::meet`), and the guns (rule 6) in the raid
//! (`airstrike.rs`).
//!
//! Keeping station runs at the start of each pulse's movement, after the
//! halts steamed on and the straggler pass, in force order. An escort in
//! station takes its charge's remaining route, cut at the waypoint where the
//! charge will halt this pulse, and its charge's speed in knots, so it moves
//! as a force ordered so would and stops where its charge stops. An escort
//! out of station is routed to its charge's hex at cruise. Either is written
//! only where it changes the escort's route, speed or status, and is then
//! recorded as `StationKept`, citing the escort order and, in station, the
//! charge's route order.

use crate::events::{Cause, EventKind};
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{EventId, TaskForceId};
use crate::orders::SpeedSetting;
use crate::state::{Charge, Route, TaskForce, TaskForceStatus};
use crate::time::Pulse;
use crate::world::World;

/// Whether `escort` is in station on its charge (decision 0121, 2): it has a
/// charge, it is in its charge's hex, and both have a ship afloat.
pub(crate) fn in_station(escort: &TaskForce, charge: &TaskForce) -> bool {
    escort.charge.is_some_and(|c| c.force == charge.id)
        && !escort.ships.is_empty()
        && !charge.ships.is_empty()
        && escort.position() == charge.position()
}

impl World {
    /// The escorts in station on `charge`, in task force number order
    /// (decision 0121, 2 and 4).
    pub(crate) fn escorts_in_station(&self, charge: TaskForceId) -> Vec<TaskForceId> {
        let Some(charged) = self.truth.task_forces.get(&charge) else {
            return Vec::new();
        };
        self.truth
            .task_forces
            .values()
            .filter(|escort| in_station(escort, charged))
            .map(|escort| escort.id)
            .collect()
    }

    /// The escorts whose charge is `charge`, in task force number order.
    pub(crate) fn escorts_of(&self, charge: TaskForceId) -> Vec<TaskForceId> {
        self.truth
            .task_forces
            .values()
            .filter(|escort| escort.charge.is_some_and(|c| c.force == charge))
            .map(|escort| escort.id)
            .collect()
    }

    /// Rule 7 for every escort whose charge has no ship afloat (sunk, stood
    /// down or joined into another force): it has no charge from now and
    /// stops where it is. Recorded as `ChargeLost`, citing the escort order
    /// and `cause` where one is given. Returns nothing; the events are
    /// recorded here.
    pub(crate) fn lose_empty_charges(
        &mut self,
        day: u32,
        pulse: Option<Pulse>,
        cause: Option<EventId>,
    ) {
        let lost: Vec<(TaskForceId, Charge)> = self
            .truth
            .task_forces
            .values()
            .filter_map(|escort| {
                let charge = escort.charge?;
                let gone = self
                    .truth
                    .task_forces
                    .get(&charge.force)
                    .is_none_or(|c| c.ships.is_empty());
                gone.then_some((escort.id, charge))
            })
            .collect();
        for (escort, charge) in lost {
            let mut causes = vec![Cause::Order(charge.order)];
            causes.extend(cause.map(Cause::Event));
            self.record_lost_charge(day, pulse, escort, causes);
        }
    }

    /// Clears the charge of `escort` and stops it where it is (decision 0121,
    /// 7, and an escort order that clears the charge): at a centre it keeps
    /// no route; between centres it is routed to the hex it is in, turning
    /// back where that is the shorter way, as a route order would. Returns
    /// the events to record: `ChargeLost` (for a charge lost, not cleared by
    /// order), then `TaskForceTurnedBack` where it turned back.
    pub(crate) fn lose_charge(&mut self, escort: TaskForceId) -> Vec<EventKind> {
        let force = self
            .truth
            .task_forces
            .get_mut(&escort)
            .expect("an escort exists");
        let charge = force.charge.take().map_or(escort, |charge| charge.force);
        let hex = force.position();
        let turned = stop_where_it_is(force, &self.map);
        let mut events = vec![EventKind::ChargeLost {
            escort,
            charge,
            hex,
        }];
        events.extend(turned);
        events
    }

    /// Clears the charge of `escort` and stops it (decision 0121, 7), and
    /// records `ChargeLost` citing `causes`, then any turning back citing
    /// it.
    pub(crate) fn record_lost_charge(
        &mut self,
        day: u32,
        pulse: Option<Pulse>,
        escort: TaskForceId,
        causes: Vec<Cause>,
    ) {
        let mut events = self.lose_charge(escort).into_iter();
        let Some(first) = events.next() else {
            return;
        };
        let lost = self.events.record(day, pulse, first, causes, None);
        for kind in events {
            self.events
                .record(day, pulse, kind, vec![Cause::Event(lost)], None);
        }
    }

    /// Keeping station (decision 0121, 3) for every escort with a charge, in
    /// force order. See the module notes.
    pub(crate) fn keep_station(&mut self, day: u32, pulse: Pulse) {
        self.lose_empty_charges(day, Some(pulse), None);
        let escorts: Vec<TaskForceId> = self
            .truth
            .task_forces
            .values()
            .filter(|force| force.charge.is_some() && !force.ships.is_empty())
            .map(|force| force.id)
            .collect();
        for id in escorts {
            self.keep_station_of(day, pulse, id);
        }
    }

    fn keep_station_of(&mut self, day: u32, pulse: Pulse, id: TaskForceId) {
        let escort = &self.truth.task_forces[&id];
        // A force out of fuel takes no route (chapter 2).
        if escort.status == TaskForceStatus::OutOfFuel {
            return;
        }
        let Some(charge) = escort.charge else {
            return;
        };
        let charged = &self.truth.task_forces[&charge.force];
        let station = in_station(escort, charged);
        // What the escort is to do this pulse: a route and a speed, or to
        // stop where it is.
        let wanted: Option<(Vec<Hex>, SpeedSetting)> = if station {
            self.charge_passage(charge.force, pulse)
                .map(|(waypoints, knots)| (waypoints, SpeedSetting::Knots(knots)))
        } else {
            Some((vec![charged.position()], SpeedSetting::Cruise))
        };
        let charge_route_order = charged.route_order;
        let escort = &self.truth.task_forces[&id];
        let remaining: Vec<Hex> = escort
            .route
            .waypoints
            .get(escort.route.next_index..)
            .map(<[Hex]>::to_vec)
            .unwrap_or_default();
        let moving = matches!(
            escort.status,
            TaskForceStatus::Underway | TaskForceStatus::Halted
        );
        let unchanged = match &wanted {
            Some((waypoints, speed)) => {
                escort.status == TaskForceStatus::Underway
                    && escort.halt.is_none()
                    && remaining == *waypoints
                    && escort.route.act_at.is_empty()
                    && escort.route.stand_at.is_empty()
                    && escort.speed == *speed
            }
            None => !moving && escort.toward_hex.is_none(),
        };
        if unchanged {
            return;
        }
        let mut causes = vec![Cause::Order(charge.order)];
        if station {
            causes.extend(charge_route_order.map(Cause::Order));
        }
        let force = self
            .truth
            .task_forces
            .get_mut(&id)
            .expect("the escort exists");
        let (waypoints, knots, turned) = match wanted {
            Some((waypoints, speed)) => {
                let turned = set_route(force, &self.map, waypoints.clone());
                force.speed = speed;
                force.speed_order = Some(charge.order);
                force.route_order = Some(charge.order);
                let knots = match speed {
                    SpeedSetting::Knots(knots) => Some(knots),
                    SpeedSetting::Cruise | SpeedSetting::Full => None,
                };
                let recorded = self.events.record(
                    day,
                    Some(pulse),
                    EventKind::StationKept {
                        escort: id,
                        charge: charge.force,
                        in_station: station,
                        waypoints: waypoints.clone(),
                        knots,
                    },
                    causes,
                    None,
                );
                if let Some(kind) = turned {
                    self.events
                        .record(day, Some(pulse), kind, vec![Cause::Event(recorded)], None);
                }
                return;
            }
            None => {
                let turned = stop_where_it_is(force, &self.map);
                (Vec::new(), None, turned)
            }
        };
        let recorded = self.events.record(
            day,
            Some(pulse),
            EventKind::StationKept {
                escort: id,
                charge: charge.force,
                in_station: station,
                waypoints,
                knots,
            },
            causes,
            None,
        );
        if let Some(kind) = turned {
            self.events
                .record(day, Some(pulse), kind, vec![Cause::Event(recorded)], None);
        }
    }

    /// The charge's passage for the pulse, as its escort in station takes
    /// it: its waypoints still to reach, cut at the one where it will halt
    /// in `pulse`, and the speed in knots its slowest ship makes at its
    /// speed setting. `None` where the charge will not steam this pulse: it
    /// is stopped, halted, out of fuel, has no route left or makes no speed.
    pub(crate) fn charge_passage(
        &self,
        charge: TaskForceId,
        pulse: Pulse,
    ) -> Option<(Vec<Hex>, Fixed)> {
        let charged = &self.truth.task_forces[&charge];
        if charged.status != TaskForceStatus::Underway {
            return None;
        }
        let knots = crate::movement::force_speed_kn(
            charged,
            |ship| &self.truth.ships[&ship],
            &self.ref_data,
        )?;
        let ground = self.halt_ground();
        let side = charged.side;
        let halt = crate::movement::halt_index(&ground, charged, pulse, &|at| {
            self.truth
                .bases
                .values()
                .find(|base| base.hex == at && base.side != side && base.on_map)
                .map(|base| base.id)
        });
        let next = charged.route.next_index;
        let end = halt.map_or(charged.route.waypoints.len(), |index| index + 1);
        let waypoints: Vec<Hex> = charged.route.waypoints.get(next..end)?.to_vec();
        if waypoints.is_empty() {
            return None;
        }
        Some((waypoints, knots))
    }
}

/// Gives `force` the route `waypoints` with no mark, from the centre it
/// reaches first, as a route order's acceptance does (chapter 2): it turns
/// back between centres where that is the shorter way, is under way, and
/// any halt or arrival ends. Returns the `TaskForceTurnedBack` event to
/// record where it turned back.
pub(crate) fn set_route(
    force: &mut TaskForce,
    map: &crate::map::Map,
    waypoints: Vec<Hex>,
) -> Option<EventKind> {
    let turns_back = force
        .reroute_centre(map, waypoints.first().copied())
        .is_some_and(|(turns_back, _)| turns_back);
    let mut turned = None;
    if turns_back {
        force.turn_back();
        if let Some(toward) = force.toward_hex {
            turned = Some(EventKind::TaskForceTurnedBack {
                task_force: force.id,
                from: force.from_hex,
                toward,
                progress_nm: force.progress_nm,
                position: force.position(),
            });
        }
    }
    force.route = Route {
        waypoints,
        next_index: 0,
        act_at: std::collections::BTreeSet::new(),
        stand_at: std::collections::BTreeSet::new(),
    };
    force.status = TaskForceStatus::Underway;
    force.halt = None;
    force.arrived_at_hours = None;
    force.arrival_event = None;
    turned
}

/// Stops `force` where it is (decision 0121, 7): at a centre it keeps no
/// route and, if it was steaming or halted, is idle; between centres it is
/// routed to the hex it is in, by the shorter way. Returns the
/// `TaskForceTurnedBack` event to record where it turned back.
pub(crate) fn stop_where_it_is(force: &mut TaskForce, map: &crate::map::Map) -> Option<EventKind> {
    if force.toward_hex.is_none() {
        force.route = Route::default();
        force.halt = None;
        if matches!(
            force.status,
            TaskForceStatus::Underway | TaskForceStatus::Halted
        ) {
            force.status = TaskForceStatus::Idle;
        }
        return None;
    }
    let hex = force.position();
    set_route(force, map, vec![hex])
}
