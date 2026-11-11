//! The movement step: task forces move in order of clock time, and fights
//! happen where they meet (decision 0022).
//!
//! The step is the `Movement` phase of a pulse. It does these things, in
//! order:
//!
//! 1. The straggler pass for every force (`stragglers.rs`).
//! 2. Each task force that is under way, with a ship afloat and a speed
//!    above zero, gets a distance budget: its speed times the pulse hours,
//!    limited by the fuel aboard (decision 0006). Distance is the budget;
//!    time is worked out from it as distance over speed.
//! 3. Opposing forces in one hex at the start, neither of which will move,
//!    meet (decision 3 of 0022).
//! 4. Position changes are taken in time order. A force's position changes
//!    when its progress on a leg reaches 20 nm (decision 0005, decision 1).
//!    The force whose next change comes earliest in hours steamed this pulse
//!    (the lower ID on a tie) is advanced to that point, and meets every
//!    opposing force in the hex it enters, once per pair per pulse. A
//!    meeting is a fight (decision 0014, in `surface.rs`), then the alarm,
//!    then the straggler pass for the two forces, then each force carries on
//!    at the speed its ships can then make, with its distance left scaled by
//!    the new speed over the old (decision 5 of 0022). A fight costs no
//!    hours.
//! 5. When no force can reach another position change, each steams out what
//!    is left of its budget.
//!
//! A force's steaming is recorded per segment: from the start of the pulse
//! or the last fight to the next fight or the end of the pulse. A force that
//! fights nobody has one segment, and so the events it always had. A segment
//! that ends in a fight is recorded before the fight's events.
//!
//! The working state of each moving force (leg, route index, distance left,
//! segment) lives in a [`Mover`] for the length of the step. The leg and the
//! route index are copied to truth every time a force's position changes, so
//! truth always says where every force is. Fuel is burned and written when a
//! segment is recorded.

use std::collections::{BTreeMap, BTreeSet};

use crate::events::{Breakdown, Cause, EventKind, Factor};
use crate::fixed::Fixed;
use crate::hex::{Hex, HEX_NM};
use crate::ids::{EventId, ShipId, TaskForceId};
use crate::map::Map;
use crate::orders::SpeedSetting;
use crate::refdata::SurfaceCombatFigures;
use crate::rng::RngStream;
use crate::state::{ShipSystem, TaskForceStatus};
use crate::time::{pulse_start, Pulse};
use crate::world::World;

/// What the movement rules need to know about one ship of a task force.
struct Steamer {
    ship: ShipId,
    fuel_t: Fixed,
    /// The ship's own speed: its class speed at the task force's speed
    /// setting, times its propulsion factor (decision 7 of
    /// `docs/decisions/0016-systems-and-stragglers.md`). For a speed in
    /// knots, the lower of the ordered speed and the ship's top speed times
    /// its propulsion factor (decision 7 of
    /// `docs/decisions/0021-readiness-and-alarm.md`).
    speed_kn: Fixed,
    /// 1, the degraded factor or 0, by the state of the ship's propulsion.
    propulsion_factor: Fixed,
    cruise_speed_kn: Fixed,
    fuel_capacity_t: Fixed,
    endurance_nm: Fixed,
}

/// Fuel burn per mile rises with the square of speed relative to cruise, and
/// never falls below the cruise rate: at cruise speed or slower the
/// multiplier is 1 (the floor, decision 3 of
/// `docs/decisions/0006-fuel-model.md`). This is a placeholder model.
pub(crate) fn speed_multiplier(speed_kn: Fixed, cruise_speed_kn: Fixed) -> Fixed {
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

fn one_hex() -> Fixed {
    Fixed::from_int(HEX_NM)
}

/// Progress at which a force enters the hex ahead.
fn half_hex() -> Fixed {
    one_hex() / Fixed::from_int(2)
}

/// The pair in (lower ID, higher ID) order.
fn ordered(a: TaskForceId, b: TaskForceId) -> (TaskForceId, TaskForceId) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

/// One moving task force's working state for the step.
struct Mover {
    /// Has a budget and can still steam. A force that has none left (sunk,
    /// dead in the water after a fight) is kept for its hours used.
    active: bool,
    waypoints: Vec<Hex>,
    from_hex: Hex,
    toward_hex: Option<Hex>,
    progress_nm: Fixed,
    next_index: usize,
    /// The distance field to the waypoint being steamed to, worked out once
    /// per waypoint (decision 0017, decision 12: the path itself is never
    /// stored).
    field: Option<(usize, BTreeMap<Hex, u32>)>,
    steamers: Vec<Steamer>,
    /// The slowest ship's speed, and its propulsion factor.
    speed_kn: Fixed,
    propulsion_factor: Fixed,
    /// The distance the speed allows for what is left of the pulse, and the
    /// distance the fuel allows, if it limits at all, for the breakdown.
    time_limit_nm: Fixed,
    fuel_limit_nm: Option<Fixed>,
    /// The fuel was what limited the budget.
    fuel_is_the_limit: bool,
    /// What is left of the budget.
    left_nm: Fixed,
    /// The open segment: where it began, how far and through which hexes.
    segment_from: Hex,
    segment_nm: Fixed,
    segment_hexes: Vec<Hex>,
    /// Hours steamed in the segments recorded so far, each worked out as one
    /// division of the segment's distance by its speed.
    hours_done: Fixed,
    /// The last `TaskForceSteamed` event of the pulse.
    last_steamed: Option<EventId>,
    /// `TaskForceArrived` has been recorded for this mover (with the segment
    /// in which the force reached its last waypoint).
    arrival_recorded: bool,
}

impl Mover {
    /// Where the force is: the nearest centre (decision 0005).
    fn position(&self) -> Hex {
        match self.toward_hex {
            Some(toward) if self.progress_nm >= half_hex() => toward,
            _ => self.from_hex,
        }
    }

    /// The distance field to the waypoint at `index`.
    fn field_to(&mut self, map: &Map, index: usize) -> &BTreeMap<Hex, u32> {
        if self.field.as_ref().map(|(i, _)| *i) != Some(index) {
            self.field = Some((index, map.sea_distances_to(self.waypoints[index])));
        }
        &self
            .field
            .as_ref()
            .expect("the field was just worked out")
            .1
    }

    /// Steams up to `distance_nm` along the route.
    ///
    /// A leg between two hex centres is 40 nm and, once started, is always
    /// finished: the next centre reached is the hex ahead, whatever the
    /// route says. At a centre the next leg goes to the next hex on the
    /// shortest sea path to the current waypoint (`Map::next_step_in`,
    /// decision 0017). The force enters a hex (its position changes) when it
    /// passes the midpoint of the leg, 20 nm along, so `hexes_entered` and
    /// the `from` and `to` of the event are positions, not centres reached.
    /// Stops early at the end of the route. Adds what it steamed to the open
    /// segment and takes it off the distance left.
    fn steam(&mut self, map: &Map, distance_nm: Fixed) {
        let one_hex = one_hex();
        let half_hex = half_hex();
        let mut travelled_nm = Fixed::ZERO;
        loop {
            if self.toward_hex.is_none() {
                // At a centre: advance past a waypoint reached, or stop.
                let Some(target) = self.waypoints.get(self.next_index) else {
                    break;
                };
                if self.from_hex == *target {
                    self.next_index += 1;
                    continue;
                }
            }
            let remaining_nm = distance_nm - travelled_nm;
            if remaining_nm.is_zero() {
                break;
            }
            let leg_end = match self.toward_hex {
                Some(toward) => toward,
                None => {
                    // Only start a leg with distance left to steam it, so
                    // zero progress always means no hex ahead.
                    let index = self.next_index;
                    let from = self.from_hex;
                    let field = self.field_to(map, index);
                    // Intake proved every waypoint reachable and the map never
                    // changes, so there is always a next step. If there were
                    // none, stop here and stay under way rather than panic.
                    let Some(toward) = map.next_step_in(field, from) else {
                        break;
                    };
                    self.toward_hex = Some(toward);
                    toward
                }
            };
            let to_centre_nm = one_hex - self.progress_nm;
            if remaining_nm >= to_centre_nm {
                travelled_nm += to_centre_nm;
                if self.progress_nm < half_hex {
                    self.segment_hexes.push(leg_end);
                }
                self.from_hex = leg_end;
                self.toward_hex = None;
                self.progress_nm = Fixed::ZERO;
            } else {
                travelled_nm += remaining_nm;
                if self.progress_nm < half_hex && self.progress_nm + remaining_nm >= half_hex {
                    self.segment_hexes.push(leg_end);
                }
                self.progress_nm += remaining_nm;
            }
        }
        self.segment_nm += travelled_nm;
        self.left_nm -= travelled_nm;
    }

    /// The distance from now to the force's next change of position, if it
    /// has one: the rest of the way to 20 nm along its leg, or, past that,
    /// the rest of the leg plus 20 nm along the next one. `None` if the route
    /// ends (or cannot go on) first. Pure: steaming that distance gives
    /// exactly the state that steaming more in one go gives at that point.
    fn distance_to_next_change(&mut self, map: &Map) -> Option<Fixed> {
        let mut walked_nm = Fixed::ZERO;
        let mut at = self.from_hex;
        if let Some(toward) = self.toward_hex {
            if self.progress_nm < half_hex() {
                return Some(half_hex() - self.progress_nm);
            }
            walked_nm = one_hex() - self.progress_nm;
            at = toward;
        }
        // At a centre: skip the waypoints already reached.
        let mut index = self.next_index;
        while *self.waypoints.get(index)? == at {
            index += 1;
        }
        let field = self.field_to(map, index);
        map.next_step_in(field, at)?;
        Some(walked_nm + half_hex())
    }
}

/// Everything that lasts for one movement step and is not truth state.
struct Step {
    day: u32,
    pulse: Pulse,
    figures: Option<SurfaceCombatFigures>,
    movers: BTreeMap<TaskForceId, Mover>,
    /// The pairs that have fought this pulse (decision 4 of 0022). Local to
    /// the step, not state.
    fought: BTreeSet<(TaskForceId, TaskForceId)>,
    /// The forces that broke off or slipped away in this step. Decision 9 of
    /// `docs/decisions/0022-meeting-and-breaking-off.md` makes avoiding last
    /// only for the rest of the pulse, so it is not state: the set dies with
    /// the step and is fresh each pulse, and the `ForceBrokeOff` and
    /// `ForceSlippedAway` events are the record.
    avoiding: BTreeSet<TaskForceId>,
    /// One alarm stream per force, kept across every fight of the step.
    alarms: BTreeMap<TaskForceId, RngStream>,
}

impl Step {
    /// The force can steam this pulse: it has a budget above zero.
    fn is_moving(&self, id: TaskForceId) -> bool {
        self.movers
            .get(&id)
            .is_some_and(|mover| mover.active && !mover.left_nm.is_zero())
    }

    /// Hours the force has steamed in the segments recorded so far.
    fn hours_used(&self, id: TaskForceId) -> Fixed {
        self.movers
            .get(&id)
            .map_or(Fixed::ZERO, |mover| mover.hours_done)
    }
}

impl World {
    /// The movement step of one pulse. See the module notes.
    pub(crate) fn move_task_forces(&mut self, day: u32, pulse: Pulse) {
        self.handle_stragglers(day, pulse);
        let mut step = Step {
            day,
            pulse,
            figures: self.ref_data.surface_combat.clone(),
            movers: BTreeMap::new(),
            fought: BTreeSet::new(),
            avoiding: BTreeSet::new(),
            alarms: BTreeMap::new(),
        };

        // Budgets. The list of forces is taken after the straggler pass, so a
        // force it makes moves in this same step.
        let ids: Vec<TaskForceId> = self.truth.task_forces.keys().copied().collect();
        for id in ids {
            if let Some(mover) = self.new_mover(id, pulse, Fixed::ZERO) {
                step.movers.insert(id, mover);
            }
        }

        // Meetings at the start (decision 3 of 0022): forces in one hex
        // neither of which will move. The list is taken once.
        if step.figures.is_some() {
            let pairs: Vec<(TaskForceId, TaskForceId)> = self
                .opposing_pairs_in_one_hex()
                .into_iter()
                .filter(|(a, b)| !step.is_moving(*a) && !step.is_moving(*b))
                .collect();
            for (a, b) in pairs {
                self.meet(&mut step, a, b, pulse_start(day, pulse), None);
            }
        }

        // Position changes in time order.
        loop {
            let mut next: Option<(Fixed, TaskForceId, Fixed)> = None;
            for (id, mover) in &mut step.movers {
                if !mover.active {
                    continue;
                }
                let Some(distance_nm) = mover.distance_to_next_change(&self.map) else {
                    continue;
                };
                if distance_nm > mover.left_nm {
                    continue;
                }
                // Forces come in ID order and only an earlier time replaces
                // the best, so a tie goes to the lower ID.
                let hours = mover.hours_done + (mover.segment_nm + distance_nm) / mover.speed_kn;
                if next.is_none_or(|(best, _, _)| hours < best) {
                    next = Some((hours, *id, distance_nm));
                }
            }
            let Some((hours, id, distance_nm)) = next else {
                break;
            };
            let mover = step.movers.get_mut(&id).expect("the mover exists");
            mover.steam(&self.map, distance_nm);
            let (from_hex, toward_hex, progress_nm, next_index) = (
                mover.from_hex,
                mover.toward_hex,
                mover.progress_nm,
                mover.next_index,
            );
            let hex = mover.position();
            let task_force = self
                .truth
                .task_forces
                .get_mut(&id)
                .expect("task force exists");
            task_force.from_hex = from_hex;
            task_force.toward_hex = toward_hex;
            task_force.progress_nm = progress_nm;
            task_force.route.next_index = next_index;
            self.meet_in(&mut step, id, hex, pulse_start(day, pulse) + hours);
        }

        // Steam out what is left, and record the last segment of each force
        // that is still moving, in ID order.
        let ids: Vec<TaskForceId> = step
            .movers
            .iter()
            .filter(|(_, mover)| mover.active)
            .map(|(id, _)| *id)
            .collect();
        for id in ids {
            self.finish_steaming(&mut step, id);
        }
    }

    /// A mover for the force, if it is under way with a ship afloat and a
    /// speed above zero: the slowest ship sets the speed (the lowest ship ID
    /// on a tie). Its budget is that speed times the hours of the pulse it
    /// has not yet used, limited by the fuel aboard. It starts with
    /// `hours_used` already steamed.
    fn new_mover(&self, id: TaskForceId, pulse: Pulse, hours_used: Fixed) -> Option<Mover> {
        let task_force = self.truth.task_forces.get(&id)?;
        if task_force.status != TaskForceStatus::Underway {
            return None;
        }

        let figures: Option<&SurfaceCombatFigures> = self.ref_data.surface_combat.as_ref();
        let steamers: Vec<Steamer> = task_force
            .ships
            .iter()
            .map(|ship_id| {
                let ship = &self.truth.ships[ship_id];
                let class = &self.ref_data.ship_classes[&ship.class];
                // World::new rejects a damaged ship without figures, so
                // with none every ship's propulsion works.
                let propulsion_factor = match figures {
                    Some(figures) => {
                        figures.system_factor(ShipSystem::Propulsion, ship.systems.propulsion)
                    }
                    None => Fixed::ONE,
                };
                let speed_kn = match task_force.speed {
                    SpeedSetting::Cruise => class.cruise_speed_kn * propulsion_factor,
                    SpeedSetting::Full => class.max_speed_kn * propulsion_factor,
                    // Decision 7 of 0021: the ordered speed, or the best
                    // speed of the ship (top speed times its propulsion
                    // factor) if that is lower.
                    SpeedSetting::Knots(knots) => knots.min(class.max_speed_kn * propulsion_factor),
                };
                Steamer {
                    ship: *ship_id,
                    fuel_t: ship.fuel_t,
                    speed_kn,
                    propulsion_factor,
                    cruise_speed_kn: class.cruise_speed_kn,
                    fuel_capacity_t: class.fuel_capacity_t,
                    endurance_nm: class.endurance_nm,
                }
            })
            .collect();
        // The slowest ship sets the speed; the lowest ship ID on a tie.
        // A task force with no ships goes nowhere.
        let slowest = steamers.iter().min_by_key(|s| s.speed_kn)?;
        let speed_kn = slowest.speed_kn;
        let propulsion_factor = slowest.propulsion_factor;
        if speed_kn.is_zero() {
            // Dead in the water: no movement, no event, no fuel burned. The
            // force stays under way.
            return None;
        }

        let hours_left = (Fixed::from_int(pulse.hours()) - hours_used).max(Fixed::ZERO);
        let time_limit_nm = speed_kn * hours_left;
        let fuel_limit_nm = steamers.iter().filter_map(|s| s.range_nm(speed_kn)).min();
        let budget_nm = match fuel_limit_nm {
            Some(limit) => time_limit_nm.min(limit),
            None => time_limit_nm,
        };
        Some(Mover {
            active: true,
            waypoints: task_force.route.waypoints.clone(),
            from_hex: task_force.from_hex,
            toward_hex: task_force.toward_hex,
            progress_nm: task_force.progress_nm,
            next_index: task_force.route.next_index,
            field: None,
            steamers,
            speed_kn,
            propulsion_factor,
            time_limit_nm,
            fuel_limit_nm,
            fuel_is_the_limit: budget_nm < time_limit_nm,
            left_nm: budget_nm,
            segment_from: task_force.position(),
            segment_nm: Fixed::ZERO,
            segment_hexes: Vec::new(),
            hours_done: hours_used,
            last_steamed: None,
            arrival_recorded: false,
        })
    }

    /// The force at `id` has just changed position to `hex`, `elapsed_hours`
    /// into the game: it meets every opposing force with a ship afloat in
    /// that hex that it has not fought this pulse, in ID order of the other
    /// force (decision 2 of 0022). The list is taken once, so forces the
    /// fights make are not on it. A meeting is skipped at its turn if either
    /// force has no ship afloat, and the mover stops meeting when it has
    /// none.
    fn meet_in(&mut self, step: &mut Step, id: TaskForceId, hex: Hex, elapsed_hours: Fixed) {
        if step.figures.is_none() {
            return;
        }
        let side = self.truth.task_forces[&id].side;
        let others: Vec<TaskForceId> = self
            .truth
            .task_forces
            .values()
            .filter(|other| {
                other.side != side
                    && !other.ships.is_empty()
                    && other.position() == hex
                    && !step.fought.contains(&ordered(id, other.id))
            })
            .map(|other| other.id)
            .collect();
        for other in others {
            if self.truth.task_forces[&id].ships.is_empty() {
                break;
            }
            let (a, b) = ordered(id, other);
            self.meet(step, a, b, elapsed_hours, Some(id));
        }
    }

    /// One meeting of `a` and `b` (the lower ID first): both forces' open
    /// segments are recorded, they fight in full, the alarm sounds, the
    /// straggler pass runs for the two forces, and each carries on.
    ///
    /// `mover` is the force whose change of position caused the meeting, if
    /// any; the steaming that brought it into the hex is a cause of the
    /// fight's `EngagementBegun`.
    fn meet(
        &mut self,
        step: &mut Step,
        a: TaskForceId,
        b: TaskForceId,
        elapsed_hours: Fixed,
        mover: Option<TaskForceId>,
    ) {
        let Some(figures) = step.figures.clone() else {
            return;
        };
        // A meeting is held only if both forces are still in one hex when
        // its turn comes (decision 7 of 0022): an earlier fight at the same
        // moment may have sent one of them home, turning it out of the hex.
        if self.truth.task_forces[&a].ships.is_empty()
            || self.truth.task_forces[&b].ships.is_empty()
            || self.truth.task_forces[&a].position() != self.truth.task_forces[&b].position()
            || !step.fought.insert((a, b))
        {
            return;
        }
        // Steaming so far goes into the log before the fight does, with the
        // state in truth (the leg and route index already are).
        let steamed_a = self.record_segment(step, a);
        let steamed_b = self.record_segment(step, b);
        // The steaming that brought the mover into the hex: the segment just
        // closed, or, if it has already fought here at this moment, the one
        // closed for that fight.
        let arrival_cause = mover.and_then(|id| {
            let steamed = if id == a { steamed_a } else { steamed_b };
            steamed.or_else(|| step.movers.get(&id).and_then(|m| m.last_steamed))
        });

        self.hold_meeting(
            step.day,
            step.pulse,
            &figures,
            (a, b),
            (elapsed_hours, arrival_cause),
            (&mut step.avoiding, &mut step.alarms),
        );

        // Stragglers are dealt with straight after the fight (decision 0016,
        // decision 12).
        let first_event = self.events.len();
        self.handle_stragglers_after_fight(step.day, step.pulse, [a, b]);
        let made: Vec<(TaskForceId, TaskForceId)> = self
            .events
            .from_index(first_event)
            .iter()
            .filter_map(|event| match event.kind {
                EventKind::ShipsDetached {
                    parent, new_force, ..
                } => Some((parent, new_force)),
                _ => None,
            })
            .collect();

        for id in [a, b] {
            self.carry_on_after_fight(step, id);
        }
        // A force the pass made moves in this same step, with what is left
        // of the pulse after the parent's hours.
        for (parent, new_force) in made {
            let hours_used = step.hours_used(parent);
            if let Some(mover) = self.new_mover(new_force, step.pulse, hours_used) {
                step.movers.insert(new_force, mover);
            }
        }
    }

    /// Works out how a force carries on after a fight and its straggler
    /// pass (decision 5 of 0022).
    ///
    /// - No ship afloat, not under way, or a speed of zero: it stops.
    /// - It could steam before and can now: its distance left is what it
    ///   was, times the new speed over the old, truncated. If the speed is
    ///   unchanged the distance left and the fuel limit flag are unchanged.
    /// - It could not steam before and can now (the pass left its dead ships
    ///   behind, or sent it home): a fresh budget from what is left of the
    ///   pulse after its hours used, like a force just detached.
    ///
    /// If the speed changed, the scaled distance is also limited by the fuel
    /// range at the new speed with the fuel now aboard (decision 5 of 0022),
    /// and the fuel limit flag is true if that limit bites and false if not.
    fn carry_on_after_fight(&mut self, step: &mut Step, id: TaskForceId) {
        let old = step
            .movers
            .get(&id)
            .filter(|mover| mover.active)
            .map(|mover| (mover.speed_kn, mover.left_nm, mover.fuel_is_the_limit));
        let hours_used = step.hours_used(id);
        let last_steamed = step.movers.get(&id).and_then(|mover| mover.last_steamed);

        let Some(mut mover) = self.new_mover(id, step.pulse, hours_used) else {
            if let Some(mover) = step.movers.get_mut(&id) {
                mover.active = false;
            }
            return;
        };
        mover.last_steamed = last_steamed;
        if let Some((old_speed, old_left_nm, old_flag)) = old {
            if mover.speed_kn == old_speed {
                mover.left_nm = old_left_nm;
                mover.fuel_is_the_limit = old_flag;
            } else {
                let scaled_nm = old_left_nm.mul_div(mover.speed_kn, old_speed);
                mover.time_limit_nm = scaled_nm;
                mover.fuel_is_the_limit =
                    mover.fuel_limit_nm.is_some_and(|limit| limit < scaled_nm);
                mover.left_nm = match mover.fuel_limit_nm {
                    Some(limit) if mover.fuel_is_the_limit => limit,
                    _ => scaled_nm,
                };
            }
        }
        step.movers.insert(id, mover);
    }

    /// Records the open segment of a force, if it steamed any distance in
    /// it: `TaskForceSteamed`, then a `FuelBurned` for each ship, which also
    /// takes the fuel off the ship. The segment's hours are one division of
    /// its distance by its speed. The next segment starts where this one
    /// ended. Returns the `TaskForceSteamed` event.
    fn record_segment(&mut self, step: &mut Step, id: TaskForceId) -> Option<EventId> {
        let mover = step.movers.get_mut(&id)?;
        if mover.segment_nm.is_zero() {
            return None;
        }
        let (day, pulse) = (step.day, step.pulse);
        let task_force = &self.truth.task_forces[&id];
        let standing_orders: Vec<Cause> = [task_force.route_order, task_force.speed_order]
            .into_iter()
            .flatten()
            .map(Cause::Order)
            .collect();
        let travelled_nm = mover.segment_nm;
        let speed_kn = mover.speed_kn;
        let to = mover.position();

        let breakdown = self.config.record_breakdowns.then(|| {
            let mut factors = vec![
                Factor {
                    name: "speed_kn",
                    value: speed_kn,
                },
                Factor {
                    name: "propulsion_factor",
                    value: mover.propulsion_factor,
                },
                Factor {
                    name: "pulse_hours",
                    value: Fixed::from_int(pulse.hours()),
                },
                Factor {
                    name: "time_limit_nm",
                    value: mover.time_limit_nm,
                },
            ];
            if let Some(limit) = mover.fuel_limit_nm {
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
                from: mover.segment_from,
                to,
                distance_nm: travelled_nm,
                speed_kn,
                hexes_entered: std::mem::take(&mut mover.segment_hexes),
            },
            standing_orders,
            breakdown,
        );

        for steamer in &mover.steamers {
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

        mover.hours_done += travelled_nm / speed_kn;
        mover.segment_nm = Fixed::ZERO;
        mover.segment_from = to;
        mover.last_steamed = Some(event);

        // A force that reached its last waypoint in this segment arrives with
        // it: the event comes right after the segment's fuel events and cites
        // the segment, and truth says so now, before any fight.
        if !mover.arrival_recorded && mover.next_index >= mover.waypoints.len() {
            mover.arrival_recorded = true;
            let elapsed_hours = pulse_start(day, pulse) + mover.hours_done;
            let arrived = self.events.record(
                day,
                Some(pulse),
                EventKind::TaskForceArrived {
                    task_force: id,
                    hex: to,
                    elapsed_hours,
                },
                vec![Cause::Event(event)],
                None,
            );
            let task_force = self
                .truth
                .task_forces
                .get_mut(&id)
                .expect("task force exists");
            task_force.status = TaskForceStatus::Arrived;
            task_force.arrived_at_hours = Some(elapsed_hours);
            task_force.arrival_event = Some(arrived);
        }
        Some(event)
    }

    /// The end of the step for one force that can still steam: it steams out
    /// what is left of its budget, its last segment is recorded, and it
    /// arrives, runs dry or stays under way.
    fn finish_steaming(&mut self, step: &mut Step, id: TaskForceId) {
        let (day, pulse) = (step.day, step.pulse);
        let mover = step.movers.get_mut(&id).expect("the mover exists");
        let left_nm = mover.left_nm;
        mover.steam(&self.map, left_nm);

        let arrived = mover.next_index >= mover.waypoints.len();
        let ran_dry = !arrived && mover.fuel_is_the_limit;
        let status = if arrived {
            TaskForceStatus::Arrived
        } else if ran_dry {
            TaskForceStatus::OutOfFuel
        } else {
            TaskForceStatus::Underway
        };
        let task_force = self
            .truth
            .task_forces
            .get_mut(&id)
            .expect("task force exists");
        task_force.from_hex = mover.from_hex;
        task_force.toward_hex = mover.toward_hex;
        task_force.progress_nm = mover.progress_nm;
        let hex = task_force.position();
        task_force.route.next_index = mover.next_index;
        task_force.status = status;
        let standing_orders: Vec<Cause> = [task_force.route_order, task_force.speed_order]
            .into_iter()
            .flatten()
            .map(Cause::Order)
            .collect();

        let steamed = self.record_segment(step, id);
        let mover = &step.movers[&id];

        if arrived && !mover.arrival_recorded {
            // No distance in the last segment, so none recorded it: cites the
            // standing orders.
            let outcome_causes = match steamed {
                Some(event) => vec![Cause::Event(event)],
                None => standing_orders,
            };
            let elapsed_hours = pulse_start(day, pulse) + mover.hours_done;
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
            // Cites the force's last steaming of the pulse, else the orders.
            let outcome_causes = match mover.last_steamed {
                Some(event) => vec![Cause::Event(event)],
                None => standing_orders,
            };
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
    use crate::events::Event;
    use crate::ids::ShipClassId;
    use crate::map::{HexData, Map};
    use crate::orders::Mission;
    use crate::refdata::{PortSize, RefData, SearchFigures, ShipClass, ShipKind};
    use crate::resolve::PulsePhase;
    use crate::state::{
        Route, Ship, ShipLocation, ShipSystems, Side, SystemState, TaskForce, TruthState,
    };

    // ---- a fixture to build movement and meetings from ----
    //
    // Task forces on a strip of open sea, each of cruisers (the placeholder
    // cruiser: 16 kn cruise, 32 kn top, 2000 t of fuel over 8000 nm, so 0.25
    // t per nm at cruise speed or slower), a seed, and the movement step of
    // the night pulse of day 1 run once. Every force is at battle readiness.

    #[derive(Clone)]
    struct Force {
        id: u32,
        side: Side,
        hex: Hex,
        route: Vec<Hex>,
        speed: SpeedSetting,
        ships: usize,
        propulsion: SystemState,
    }

    impl Force {
        fn new(id: u32, side: Side, hex: Hex) -> Force {
            Force {
                id,
                side,
                hex,
                route: Vec::new(),
                speed: SpeedSetting::Cruise,
                ships: 1,
                propulsion: SystemState::Working,
            }
        }

        fn route(mut self, waypoints: &[Hex]) -> Force {
            self.route = waypoints.to_vec();
            self
        }

        fn knots(mut self, knots: i64) -> Force {
            self.speed = SpeedSetting::Knots(Fixed::from_int(knots));
            self
        }

        fn propulsion(mut self, state: SystemState) -> Force {
            self.propulsion = state;
            self
        }
    }

    fn hex(q: i32) -> Hex {
        Hex::new(q, 0)
    }

    fn world(seed: u64, forces: &[Force]) -> World {
        let mut map = Map::new();
        for q in -10..=20 {
            for r in -2..=2 {
                map.insert(Hex::new(q, r), HexData::open_sea());
            }
        }
        let mut ref_data = RefData::default();
        ref_data.ship_classes.insert(
            ShipClassId::new(1),
            ShipClass {
                id: ShipClassId::new(1),
                name: "Test cruiser".to_string(),
                kind: ShipKind::Cruiser,
                cruise_speed_kn: Fixed::from_int(16),
                max_speed_kn: Fixed::from_int(32),
                fuel_capacity_t: Fixed::from_int(2000),
                endurance_nm: Fixed::from_int(8000),
                cargo_capacity_t: Fixed::ZERO,
                main_guns: 8,
                gun_rounds_per_min: Fixed::from_int(3),
                gun_rounds_per_gun: 100,
                gun_damage: Fixed::from_int(20),
                torpedo_tubes: 8,
                torpedo_damage: Fixed::from_int(200),
                toughness: Fixed::from_int(400),
                radar: false,
                aircraft_capacity: 0,
                anti_aircraft: 0,
            },
        );
        ref_data.port_sizes.insert(
            0,
            PortSize {
                berths: 0,
                berth_rate_t_per_h: Fixed::ZERO,
                lighter_rate_t_per_h: Fixed::from_int(20),
            },
        );
        ref_data.search = Some(SearchFigures::from_search_toml());
        ref_data.surface_combat = Some(SurfaceCombatFigures::from_surface_combat_toml());

        let mut truth = TruthState::default();
        let mut next_ship = 1;
        for spec in forces {
            let id = TaskForceId::new(spec.id);
            let mut force = TaskForce::at(
                id,
                format!("Test force {}", spec.id),
                spec.side,
                spec.hex,
                Mission::SurfaceCombat,
            );
            force.readiness = crate::orders::Readiness::Battle;
            force.speed = spec.speed;
            if !spec.route.is_empty() {
                force.route = Route {
                    waypoints: spec.route.clone(),
                    next_index: 0,
                };
                force.status = TaskForceStatus::Underway;
            }
            for _ in 0..spec.ships {
                let ship_id = ShipId::new(next_ship);
                next_ship += 1;
                force.ships.insert(ship_id);
                truth.ships.insert(
                    ship_id,
                    Ship {
                        id: ship_id,
                        name: format!("Test ship {next_ship}"),
                        class: ShipClassId::new(1),
                        side: spec.side,
                        fuel_t: Fixed::from_int(2000),
                        supply_t: Fixed::ZERO,
                        location: ShipLocation::TaskForce(id),
                        flotation_damage: Fixed::ZERO,
                        fire_damage: Fixed::ZERO,
                        gun_rounds: Fixed::from_int(800),
                        torpedoes_ready: true,
                        systems: ShipSystems {
                            propulsion: spec.propulsion,
                            ..ShipSystems::ALL_WORKING
                        },
                    },
                );
            }
            truth.task_forces.insert(id, force);
        }
        World::new(seed, ref_data, map, truth).expect("valid setup")
    }

    /// The night movement step of day 1, run once.
    fn night(seed: u64, forces: &[Force]) -> World {
        let mut world = world(seed, forces);
        world.run_pulse_phase(1, Pulse::Night, PulsePhase::Movement);
        world
    }

    fn all_events(world: &World) -> Vec<Event> {
        world.events().iter().cloned().collect()
    }

    fn begun(events: &[Event]) -> Vec<&Event> {
        events
            .iter()
            .filter(|e| matches!(e.kind, EventKind::EngagementBegun { .. }))
            .collect()
    }

    /// The `TaskForceSteamed` events of a force: (event, from, to, distance,
    /// speed).
    fn steamed(events: &[Event], force: u32) -> Vec<(EventId, Hex, Hex, Fixed, Fixed)> {
        events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceSteamed {
                    task_force,
                    from,
                    to,
                    distance_nm,
                    speed_kn,
                    ..
                } if task_force.raw() == force => Some((e.id, *from, *to, *distance_nm, *speed_kn)),
                _ => None,
            })
            .collect()
    }

    /// Whether the ship was hit in its propulsion or sunk in the run.
    fn crippled(events: &[Event], ship: u32) -> bool {
        events.iter().any(|e| match &e.kind {
            EventKind::ShipSunk { ship: s, .. } => s.raw() == ship,
            EventKind::SystemHit {
                ship: s, system, ..
            } => s.raw() == ship && *system == ShipSystem::Propulsion,
            _ => false,
        })
    }

    fn sunk(events: &[Event], ship: u32) -> bool {
        events
            .iter()
            .any(|e| matches!(&e.kind, EventKind::ShipSunk { ship: s, .. } if s.raw() == ship))
    }

    fn burned_by(events: &[Event], ship: u32, before: EventId) -> Fixed {
        events
            .iter()
            .filter(|e| e.id < before)
            .filter_map(|e| match &e.kind {
                EventKind::FuelBurned {
                    ship: s, burned_t, ..
                } if s.raw() == ship => Some(*burned_t),
                _ => None,
            })
            .fold(Fixed::ZERO, |sum, t| sum + t)
    }

    // ---- a. the accepted approximation ----

    // Decision 0022, decision 6: a force met under way has its steaming
    // closed at its last change of position.
    //
    // Force 1 steams from (0,0) to (10,0) at 16 kn: it enters (1,0), (2,0),
    // (3,0) and (4,0) after 20, 60, 100 and 140 nm, at 1.25, 3.75, 6.25 and
    // 8.75 hours. Force 2 steams from (6,0) to (0,0) at 8 kn and enters (5,0)
    // after 20 nm (2.5 hours) and (4,0) after 60 nm (7.5 hours). At 8.75
    // hours force 1 enters (4,0), where force 2 is: they meet. Force 2 has
    // then steamed 8 kn x 8.75 h = 70 nm, 10 nm past its last change of
    // position. Its steaming is closed at that
    // change: 60 nm, burning 60 x 0.25 = 15 t, and the 10 nm are still to
    // come. Its night budget is 8 x 12 = 96 nm, so with its speed unchanged
    // it has 36 nm left and its total is 60 + 36 = 96 nm, 24 t.
    #[test]
    fn a_force_met_under_way_has_its_steaming_closed_at_its_last_position_change() {
        let forces = [
            Force::new(1, Side::Japanese, hex(0)).route(&[hex(10)]),
            Force::new(2, Side::Allied, hex(6))
                .route(&[hex(0)])
                .knots(8),
        ];
        let mut unchanged = 0;
        for seed in 1..=40 {
            let world = night(seed, &forces);
            let events = all_events(&world);
            let fights = begun(&events);
            assert_eq!(fights.len(), 1, "seed {seed}");
            let fight = fights[0];
            let EventKind::EngagementBegun {
                hex: at,
                elapsed_hours,
                ..
            } = fight.kind
            else {
                unreachable!("a fight");
            };
            assert_eq!(at, hex(4), "seed {seed}");
            assert_eq!(elapsed_hours, Fixed::from_raw(8_750_000), "seed {seed}");

            let steps = steamed(&events, 2);
            let before: Vec<_> = steps.iter().filter(|s| s.0 < fight.id).collect();
            assert_eq!(before.len(), 1, "seed {seed}");
            // Not 70 nm, the distance to the exact moment of the meeting.
            assert_eq!(
                (before[0].1, before[0].2, before[0].3, before[0].4),
                (hex(6), hex(4), Fixed::from_int(60), Fixed::from_int(8)),
                "seed {seed}"
            );
            assert_eq!(
                burned_by(&events, 2, fight.id),
                Fixed::from_int(15),
                "seed {seed}"
            );
            let first = steamed(&events, 1);
            assert_eq!(first[0].3, Fixed::from_int(140), "seed {seed}");
            assert!(first[0].0 < fight.id);

            // The distance is not lost: 96 nm in all and 24 t, if its speed
            // did not change.
            if !crippled(&events, 2) {
                unchanged += 1;
                let total = steps.iter().fold(Fixed::ZERO, |sum, s| sum + s.3);
                assert_eq!(total, Fixed::from_int(96), "seed {seed}");
                assert_eq!(
                    burned_by(&events, 2, EventId::new(u32::MAX)),
                    Fixed::from_int(24),
                    "seed {seed}"
                );
            }
        }
        assert!(unchanged > 0, "some seed leaves force 2's speed alone");
    }

    // ---- b. a force that reaches its last waypoint ----

    // Force 1 steams from (0,0) to (2,0) at 16 kn: 80 nm, 5 hours. Force 2
    // steams from (6,0) for (0,0) and enters (2,0) after 140 nm, at 8.75
    // hours, where force 1 lies. The 20 nm force 1 has to go from its last
    // change of position to the centre of (2,0) are steamed after the fight
    // (decision 0022, decision 6). It arrives once, citing the segment, with
    // the status Arrived.
    #[test]
    fn a_force_arrives_once_citing_its_steaming_when_met_at_its_destination() {
        let forces = [
            Force::new(1, Side::Japanese, hex(0)).route(&[hex(2)]),
            Force::new(2, Side::Allied, hex(6)).route(&[hex(0)]),
        ];
        let mut arrived_after_fight = 0;
        for seed in 1..=40 {
            let world = night(seed, &forces);
            let events = all_events(&world);
            assert_eq!(begun(&events).len(), 1, "seed {seed}");
            // A ship sunk or left dead in the water does not arrive.
            if crippled(&events, 1) {
                continue;
            }
            let arrivals: Vec<&Event> = events
                .iter()
                .filter(|e| {
                    matches!(&e.kind, EventKind::TaskForceArrived { task_force, .. }
                        if task_force.raw() == 1)
                })
                .collect();
            assert_eq!(arrivals.len(), 1, "seed {seed}");
            let Cause::Event(cause) = arrivals[0].causes[0] else {
                panic!("seed {seed}: arrival cites an order");
            };
            assert!(
                steamed(&events, 1).iter().any(|s| s.0 == cause),
                "seed {seed}: the arrival cites a steaming of force 1"
            );
            let status = world.umpire().task_forces[&TaskForceId::new(1)].status;
            assert_eq!(status, TaskForceStatus::Arrived, "seed {seed}");
            arrived_after_fight += 1;
            let EventKind::TaskForceArrived {
                hex: at,
                elapsed_hours,
                ..
            } = arrivals[0].kind
            else {
                unreachable!("an arrival");
            };
            assert_eq!(at, hex(2), "seed {seed}");
            assert_eq!(elapsed_hours, Fixed::from_int(5), "seed {seed}");
        }
        assert!(arrived_after_fight > 0);
    }

    // ---- c. forces in one hex at the start of the pulse ----

    fn fights_at_start(forces: &[Force]) -> Vec<usize> {
        (1..=10)
            .map(|seed| {
                let world = night(seed, forces);
                let events = all_events(&world);
                let fights = begun(&events);
                if let Some(first) = fights.first() {
                    let EventKind::EngagementBegun {
                        elapsed_hours,
                        hex: at,
                        ..
                    } = first.kind
                    else {
                        unreachable!("a fight");
                    };
                    assert_eq!(elapsed_hours, Fixed::ZERO, "seed {seed}");
                    assert_eq!(at, hex(3), "seed {seed}");
                }
                fights.len()
            })
            .collect()
    }

    #[test]
    fn opposing_forces_in_one_hex_fight_at_the_start_if_neither_will_move() {
        let idle = [
            Force::new(1, Side::Japanese, hex(3)),
            Force::new(2, Side::Allied, hex(3)),
        ];
        assert_eq!(fights_at_start(&idle), vec![1; 10]);
    }

    #[test]
    fn a_force_that_has_a_route_but_cannot_move_does_not_leave() {
        let stuck = [
            Force::new(1, Side::Japanese, hex(3))
                .route(&[hex(10)])
                .propulsion(SystemState::Out),
            Force::new(2, Side::Allied, hex(3)),
        ];
        assert_eq!(fights_at_start(&stuck), vec![1; 10]);
    }

    #[test]
    fn opposing_forces_in_one_hex_do_not_fight_at_the_start_if_one_is_leaving() {
        // The lower ID leaves, then the higher.
        for forces in [
            [
                Force::new(1, Side::Japanese, hex(3)).route(&[hex(10)]),
                Force::new(2, Side::Allied, hex(3)),
            ],
            [
                Force::new(1, Side::Japanese, hex(3)),
                Force::new(2, Side::Allied, hex(3)).route(&[hex(10)]),
            ],
        ] {
            assert_eq!(fights_at_start(&forces), vec![0; 10]);
        }
    }

    // ---- d. a pair fights once in a pulse ----

    // Force 2 lies in (1,0). Force 1 steams from (0,0) for (3,0) and back to
    // (0,0) at 16 kn: it enters (1,0) after 20 nm, reaches (3,0) after 120
    // nm and enters (1,0) again after 180 nm, at 11.25 hours, inside the
    // 12-hour pulse. The two have fought already: they do not fight again.
    #[test]
    fn a_pair_fights_at_most_once_in_a_pulse() {
        let forces = [
            Force::new(1, Side::Japanese, hex(0)).route(&[hex(3), hex(0)]),
            Force::new(2, Side::Allied, hex(1)),
        ];
        let mut back_again = 0;
        for seed in 1..=60 {
            let world = night(seed, &forces);
            let events = all_events(&world);
            assert_eq!(begun(&events).len(), 1, "seed {seed}");
            let entered_1_0 = events
                .iter()
                .filter_map(|e| match &e.kind {
                    EventKind::TaskForceSteamed {
                        task_force,
                        hexes_entered,
                        ..
                    } if task_force.raw() == 1 => Some(hexes_entered.clone()),
                    _ => None,
                })
                .flatten()
                .filter(|h| *h == hex(1))
                .count();
            if entered_1_0 == 2 && !sunk(&events, 2) {
                back_again += 1;
            }
        }
        assert!(back_again > 0, "force 1 comes back into (1,0) in some seed");
    }

    // ---- e. what caused a meeting ----

    /// The first cause of the fight's `EngagementBegun` is a
    /// `TaskForceSteamed` of the given force, which ends in the meeting hex.
    fn assert_caused_by_steaming_of(forces: &[Force], mover: u32, at: Hex) {
        for seed in 1..=10 {
            let world = night(seed, forces);
            let events = all_events(&world);
            let fights = begun(&events);
            assert_eq!(fights.len(), 1, "seed {seed}");
            let Cause::Event(cause) = fights[0].causes[0] else {
                panic!("seed {seed}: the first cause is an order");
            };
            let found = steamed(&events, mover)
                .into_iter()
                .find(|s| s.0 == cause)
                .unwrap_or_else(|| panic!("seed {seed}: cause {cause:?} is not a steaming"));
            assert_eq!(found.2, at, "seed {seed}");
            assert!(cause < fights[0].id);
        }
    }

    #[test]
    fn the_first_cause_of_a_fight_is_the_steaming_of_the_force_that_moved_in() {
        // Lower ID moves in on a force that lies still.
        assert_caused_by_steaming_of(
            &[
                Force::new(1, Side::Japanese, hex(0)).route(&[hex(10)]),
                Force::new(2, Side::Allied, hex(3)),
            ],
            1,
            hex(3),
        );
        // Higher ID moves in on a force that lies still.
        assert_caused_by_steaming_of(
            &[
                Force::new(1, Side::Japanese, hex(3)),
                Force::new(2, Side::Allied, hex(0)).route(&[hex(10)]),
            ],
            2,
            hex(3),
        );
        // Head on, at the same moment: the lower ID is taken first, so the
        // higher ID is the one that enters the hex the other is in.
        assert_caused_by_steaming_of(
            &[
                Force::new(1, Side::Japanese, hex(0)).route(&[hex(10)]),
                Force::new(2, Side::Allied, hex(10)).route(&[hex(0)]),
            ],
            2,
            hex(5),
        );
    }
}
