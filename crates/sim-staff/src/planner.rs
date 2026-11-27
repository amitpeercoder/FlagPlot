//! Sizing and timing a run (decision 0107, 38 to 42, with rulings 4 to 6 of
//! the hand calculations). Given the projection, the pool and the route, it
//! says whether a run ordered on a day is needed, which ships take it, how
//! much it loads and its four days.
//!
//! - **Needed** (ruling 4): without it, the figure read on the day after the
//!   run would arrive is below the intent's floor.
//! - **Its ships** are the pool's free ships in number order until their
//!   room covers the load; a ship in a force brings the whole force.
//! - **Its load** (ruling 5): the floor's stock on the arrival day, less the
//!   stock projected for that day, not below 0, plus the planned use of the
//!   interval's days from the arrival day. It is cut to the ships' room and
//!   to the source's stock above its reserve and above the loads still to
//!   be taken there.
//! - **Its days** (ruling 6): whole days. Loading is `ceil(hours / 24)` at
//!   the source's berth rate (its lighter rate where it has no berth), at
//!   least 1; each passage at cruise is `ceil(hours / 24)`; unloading as
//!   loading at the destination. Load day today; sail day the load day plus
//!   the loading days; arrival the sail day plus the passage out less one.
//!   Free (decision 0112, with ruling 8 of decision 0107): the morning after
//!   the day the ship has finished fuelling at the source. The hours count
//!   from the sail day's first hour: the passage out, the unloading in whole
//!   days, the passage back at cruise, then the fuelling, at the source's
//!   fuel berth rate where it has a berth (else its lighter rate), of what
//!   the ship then lacks (its capacity less the fuel it has now and less
//!   what the round trip burns), the longest ship's hours. Free is the day
//!   that hour falls in, plus one, and never earlier than the sail day plus
//!   the passage out, the unloading and the passage back. Where the source
//!   gives no fuel rate (no limit) the fuelling takes no hours and free is
//!   that sum, the morning after the ship is back.

use std::collections::{BTreeMap, BTreeSet};

use sim_core::command::{
    CommandState, Commitment, Committed, Displacement, ForcesLimit, Hold, HoldTarget, Intent,
    IntentKind, Plan, PlanId, RunDays,
};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, ShipId, TaskForceId};
use sim_core::orders::Cargo;
use sim_core::refdata::PortSize;
use sim_core::state::{Ship, ShipLocation};
use sim_core::view::SideView;

use crate::fleet::{
    force_stopped_at, held_ships, pool_ships, run_state, waiting_ships, Fleet, RunStatus,
};
use crate::morning::{protected_for_others, Morning};
use crate::projection::Readings;
use crate::route::{route_between, RouteInfo};
use crate::units::{ceil_days, whole, HOURS_PER_DAY};

/// The days of use each run brings above the floor where the commander has
/// set no interval (question 5 of decision 0107, as answered).
pub(crate) const DEFAULT_INTERVAL_DAYS: u32 = 15;

/// What the planner reads of an intent.
pub(crate) struct Params<'a> {
    pub(crate) intent: &'a Intent,
    pub(crate) plan: &'a Plan,
    pub(crate) forces: &'a ForcesLimit,
    pub(crate) destination: BaseId,
    pub(crate) destination_hex: Hex,
    pub(crate) source: BaseId,
    pub(crate) source_hex: Hex,
    pub(crate) floor_days: u32,
    pub(crate) interval_days: u32,
    pub(crate) interval_set: bool,
    pub(crate) area: Option<&'a BTreeSet<Hex>>,
    /// Whether the source is the staff's choice (decision 0228, rule 8): the
    /// side's staff chooses its sources and no intent of the commander's on
    /// the chain names one. A run from it is cut to what it can spare.
    pub(crate) staff_source: bool,
}

/// Ships that move together: one ship lying in port in no force, or a
/// force stopped at the source with all its ships.
#[derive(Clone, Debug)]
pub(crate) struct Unit {
    /// In ship order.
    pub(crate) ships: Vec<ShipId>,
    /// The force, if the unit is one.
    pub(crate) force: Option<TaskForceId>,
}

/// The work of a load or an unload: the ships' tons, the hours the slowest
/// takes and the whole days that is.
#[derive(Clone, Debug)]
pub(crate) struct Work {
    pub(crate) hours: Fixed,
    pub(crate) days: u32,
    /// Each ship, the tons it moves and the rate it moves them at.
    pub(crate) lines: Vec<(ShipId, Fixed, Fixed)>,
}

/// The times of a run in hours and whole days.
#[derive(Clone, Debug)]
pub(crate) struct Timing {
    pub(crate) speed_kn: Fixed,
    pub(crate) load: Work,
    pub(crate) out_hours: Fixed,
    pub(crate) out_days: u32,
    pub(crate) back_hours: Fixed,
    pub(crate) back_days: u32,
    pub(crate) unload: Work,
    /// The tons landed on the arrival day, in the hours it has left.
    pub(crate) landed_on_arrival_t: Fixed,
    /// The hours of the arrival day left when the force arrives.
    pub(crate) hours_left_on_arrival: Fixed,
    /// The hours the longest ship takes to fuel at the source on its return
    /// (decision 0112): 0 where the source gives no fuel rate (no limit) or
    /// no ship lacks any.
    pub(crate) fuelling_hours: Fixed,
    /// The rate it fuels at, where one is given.
    pub(crate) fuelling_rate: Option<Fixed>,
}

impl Timing {
    /// The hour, counted from the first hour of the sail day, at which the
    /// ships have finished fuelling at the source: the passage out and the
    /// unloading in whole days, the passage back, then the fuelling.
    pub(crate) fn back_and_fuelled_hour(&self) -> Fixed {
        Fixed::from_int(HOURS_PER_DAY) * whole(self.out_days + self.unload.days)
            + self.back_hours
            + self.fuelling_hours
    }

    /// The days from the sail day to the day the ships are free: the
    /// morning after the day fuelling ends, and never fewer than the whole
    /// days of the passages and the unloading, as before decision 0112.
    pub(crate) fn free_after_sail(&self) -> u32 {
        let whole_days = self.out_days + self.unload.days + self.back_days;
        ceil_days(self.back_and_fuelled_hour()).max(whole_days)
    }

    /// The four days of a run ordered on `order_day`.
    pub(crate) fn days(&self, order_day: u32) -> RunDays {
        let sail = order_day + self.load.days;
        RunDays {
            load: order_day,
            sail,
            arrive: sail + self.out_days - 1,
            free: sail + self.free_after_sail(),
        }
    }
}

/// What a run would be.
#[derive(Clone, Debug)]
pub(crate) struct RunPlan {
    pub(crate) units: Vec<Unit>,
    pub(crate) ships: Vec<ShipId>,
    /// What the run carries, the supply aboard now plus what it takes: the
    /// load the plan's run records (ruling 22).
    pub(crate) load_t: Fixed,
    /// The tons to take aboard at the source: the load less what is aboard.
    /// No load order is given where it is 0.
    pub(crate) taken_t: Fixed,
    /// What the load order names: the supply aboard now plus the tons taken.
    pub(crate) weight_t: Fixed,
    pub(crate) aboard_before_t: Fixed,
    pub(crate) days: RunDays,
    pub(crate) timing: Timing,
    /// What cut the load below what the intent called for.
    pub(crate) cuts: Vec<String>,
    /// The run by night, in words, where it goes by night (decision 0145).
    pub(crate) night: Option<String>,
    /// The most a ship is to hold, where the run goes by night, and the
    /// words of the load order that give it (decision 0162, 5 and 10).
    pub(crate) level: Option<(Fixed, String)>,
}

/// Why a run that is needed cannot be made.
#[derive(Clone, Debug)]
pub(crate) enum Blocker {
    /// No ship of the pool is free; the earliest day one is, if any is
    /// expected to be, and the ships of the forces limit left out of the pool
    /// for their day (decision 0140, 3), each with it.
    NoShip {
        free_from: Option<u32>,
        waiting: Vec<(ShipId, u32)>,
    },
    /// The source has nothing above its reserve to give.
    Source {
        stock_t: Fixed,
        reserve_t: Fixed,
        pending_t: Fixed,
    },
    /// No route lies inside the area.
    NoRoute,
    /// Ships that are free lack the fuel for the round trip.
    Fuel { ships: Vec<ShipId> },
}

impl Blocker {
    /// What binds, in a few words.
    pub(crate) fn binds(&self) -> String {
        match self {
            Blocker::NoShip {
                free_from: Some(day),
                ..
            } => {
                format!("no ship of the pool is free until day {day}")
            }
            Blocker::NoShip {
                free_from: None, ..
            } => "no ship of the pool is free".to_string(),
            Blocker::Source { reserve_t, .. } if *reserve_t > Fixed::ZERO => {
                "the source's stock above its reserve".to_string()
            }
            Blocker::Source { .. } => "the source's stock".to_string(),
            Blocker::NoRoute => "no route in the area".to_string(),
            Blocker::Fuel { .. } => "ships without the fuel for the round trip".to_string(),
        }
    }
}

impl Blocker {
    /// The numbers behind what binds, for a reason's worked part.
    pub(crate) fn detail(&self) -> String {
        match self {
            Blocker::NoShip { waiting, .. } => crate::available::list_words(waiting),
            Blocker::NoRoute => String::new(),
            Blocker::Source {
                stock_t,
                reserve_t,
                pending_t,
            } => format!(
                "the source holds {stock_t} t, keeps {reserve_t} t in reserve, and {pending_t} t are still to be taken there by runs loading"
            ),
            Blocker::Fuel { ships } => format!(
                "{} lack the fuel for the round trip",
                ships
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

/// One evaluation of a run ordered on a day.
#[derive(Clone, Debug)]
pub(crate) struct Evaluation {
    pub(crate) order_day: u32,
    pub(crate) arrive: u32,
    /// The day after arrival, whose figure decides whether the run is needed.
    pub(crate) check_day: u32,
    pub(crate) check_stock_t: Fixed,
    pub(crate) check_use_t: Fixed,
    pub(crate) needed: bool,
    pub(crate) stock_at_arrival_t: Fixed,
    /// The floor's stock on the arrival day.
    pub(crate) floor_t: Fixed,
    pub(crate) interval_use_t: Fixed,
    pub(crate) uncapped_t: Fixed,
    pub(crate) run: Option<RunPlan>,
    pub(crate) blocker: Option<Blocker>,
}

struct Pass {
    arrive: u32,
    stock_a: Fixed,
    floor_t: Fixed,
    interval_use_t: Fixed,
    uncapped_t: Fixed,
    chosen: Chosen,
    /// The tons to take at the source.
    taken_t: Fixed,
}

/// A guess of the ships and the tons to take, to time a run by.
struct Guess {
    ships: Vec<ShipId>,
    taken_t: Fixed,
}

struct Chosen {
    units: Vec<Unit>,
    ships: Vec<ShipId>,
    /// The room left to take supply: each ship's capacity less its cargo
    /// fuel and the supply it holds.
    room_t: Fixed,
    /// The supply aboard the chosen ships: it counts toward the load.
    aboard_t: Fixed,
    without_fuel: Vec<ShipId>,
}

/// The planner for one intent.
pub(crate) struct Planner<'a, 'v> {
    pub(crate) view: &'a SideView<'v>,
    pub(crate) fleet: &'a Fleet<'v>,
    pub(crate) params: Params<'a>,
    pub(crate) today: u32,
    /// The route and its miles, `None` where ships cannot go at all.
    pub(crate) route: Option<RouteInfo>,
    /// The ships of the pool afloat that no other intent in force protects.
    pub(crate) pool: BTreeSet<ShipId>,
    /// The ships the forces limit lists that the side may not use yet, each
    /// with its day: in no pool, named in the reason that lists the pool.
    waiting: Vec<(ShipId, u32)>,
    /// The day each busy ship of the pool is free again.
    busy_until: BTreeMap<ShipId, u32>,
    held: BTreeSet<ShipId>,
    /// The ships of the pool the commander holds, and the hold that does it
    /// (ruling 18): a held ship, a ship of a held force, and every ship of a
    /// force that has a held ship in it.
    pub(crate) holds: BTreeMap<ShipId, HoldTarget>,
    /// What the staff handed in this morning for intents served before, and
    /// the command state as the staff reads it this morning.
    morning: Morning,
}

impl<'a, 'v> Planner<'a, 'v> {
    pub(crate) fn new(
        view: &'a SideView<'v>,
        fleet: &'a Fleet<'v>,
        params: Params<'a>,
        morning: &Morning,
    ) -> Planner<'a, 'v> {
        let state = morning.state();
        let today = view.day() + 1;
        let route = route_between(view, params.source_hex, params.destination_hex, params.area);
        let protected = protected_for_others(state, params.intent.id);
        let mut pool = pool_ships(fleet, params.forces);
        pool.retain(|ship| !protected.contains_key(ship));
        let held = held_ships(view, fleet, state, today);
        let holds: BTreeMap<ShipId, HoldTarget> = pool
            .iter()
            .filter_map(|ship| fleet.hold_on_ship(state, *ship).map(|hold| (*ship, hold)))
            .collect();
        let mut busy_until: BTreeMap<ShipId, u32> = BTreeMap::new();
        for ship in pool.iter().filter(|ship| !holds.contains_key(ship)) {
            if let Some(list) = state.commitments.get(ship) {
                // A commitment of a plan that has ended is no commitment.
                for commitment in list.iter().filter(|c| {
                    c.to_day >= today && !state.plans.get(&c.plan).is_some_and(|p| p.state.ended())
                }) {
                    let entry = busy_until.entry(*ship).or_insert(0);
                    *entry = (*entry).max(commitment.to_day + 1);
                }
            }
        }
        for plan in state.plans.values() {
            for run in plan.runs.values() {
                let Some(base) = view.bases().find(|b| b.id == run.source) else {
                    continue;
                };
                if run_state(fleet, state, plan.id, run, base.hex, today).status
                    == RunStatus::Finished
                {
                    continue;
                }
                for ship in run
                    .ships
                    .iter()
                    .filter(|s| pool.contains(s) && !holds.contains_key(s))
                {
                    let entry = busy_until.entry(*ship).or_insert(0);
                    *entry = (*entry).max(run.days.free.max(today + 1));
                }
            }
        }
        for run in &morning.runs {
            for ship in run
                .ships
                .iter()
                .filter(|s| pool.contains(s) && !holds.contains_key(s))
            {
                let entry = busy_until.entry(*ship).or_insert(0);
                *entry = (*entry).max(run.days.free.max(today + 1));
            }
        }
        let waiting = waiting_ships(fleet, params.forces);
        Planner {
            view,
            fleet,
            params,
            today,
            route,
            pool,
            waiting,
            busy_until,
            held,
            holds,
            morning: morning.clone(),
        }
    }

    /// The command state as the staff reads it this morning.
    pub(crate) fn state(&self) -> &CommandState {
        self.morning.state()
    }

    /// The commitment of `ship` that covers `day`, leaving out those of a
    /// plan that has ended.
    fn commitment_on(&self, ship: ShipId, day: u32) -> Option<&Commitment> {
        let state = self.state();
        state.commitments.get(&ship)?.iter().find(|c| {
            c.from_day <= day
                && day <= c.to_day
                && !state.plans.get(&c.plan).is_some_and(|p| p.state.ended())
        })
    }

    /// The earliest day a busy ship of the pool is expected to be free.
    pub(crate) fn earliest_free(&self) -> Option<u32> {
        self.busy_until.values().copied().min()
    }

    /// The units free today: a ship lying in port at the source in no force,
    /// or a force stopped at the source whose ships are all in the pool; in
    /// both, held by no unfinished run and covered by no commitment today.
    pub(crate) fn free_units(&self) -> Vec<Unit> {
        self.free_units_with(true)
    }

    /// The units that would be free if no hold existed: what the staff says
    /// it would have done (ruling 21).
    pub(crate) fn free_units_without_holds(&self) -> Vec<Unit> {
        self.free_units_with(false)
    }

    /// The holds that keep the ships of `would_take` from being free, where
    /// `real` did not take them, each with the hold: one for each target.
    pub(crate) fn holds_on(
        &self,
        would_take: &[ShipId],
        real: &[ShipId],
    ) -> Vec<(HoldTarget, Hold)> {
        let mut found: Vec<(HoldTarget, Hold)> = Vec::new();
        for ship in would_take.iter().filter(|ship| !real.contains(ship)) {
            let Some(target) = self.holds.get(ship).copied() else {
                continue;
            };
            let Some(hold) = self.state().hold_of(target).copied() else {
                continue;
            };
            if !found.iter().any(|(t, _)| *t == target) {
                found.push((target, hold));
            }
        }
        found
    }

    /// Decision 0232, rule 11 ("A force readying a landing is not a supply
    /// run's"): whether a "ready a landing" in force is of the force, its
    /// plan has not ended, and the force's order to prepare the landing
    /// stands. Such a force serves the landing until it has unloaded
    /// (decision 0151, 5).
    fn readying_a_landing(&self, force: &sim_core::state::TaskForce) -> bool {
        if force.preparation.is_none() {
            return false;
        }
        let state = self.state();
        state.intents_in_force().any(|task| {
            let IntentKind::ReadyLanding { force: own, .. } = task.kind else {
                return false;
            };
            let named = own == force.id
                || task
                    .limits
                    .forces
                    .as_ref()
                    .is_some_and(|f| f.task_forces.contains(&force.id));
            named
                && state
                    .plans
                    .get(&task.plan)
                    .is_some_and(|p| !p.state.ended())
        })
    }

    fn free_units_with(&self, respect_holds: bool) -> Vec<Unit> {
        let free = |ship: ShipId| {
            !self.held.contains(&ship)
                && self.commitment_on(ship, self.today).is_none()
                && self.morning.holding(ship).is_none()
                && (!respect_holds || !self.holds.contains_key(&ship))
        };
        let mut seen: BTreeSet<ShipId> = BTreeSet::new();
        let mut units = Vec::new();
        for ship in &self.pool {
            if seen.contains(ship) {
                continue;
            }
            let Some(found) = self.fleet.ships.get(ship) else {
                continue;
            };
            match found.location {
                ShipLocation::Port(base) if base == self.params.source => {
                    seen.insert(*ship);
                    if free(*ship) {
                        units.push(Unit {
                            ships: vec![*ship],
                            force: None,
                        });
                    }
                }
                ShipLocation::TaskForce(id) => {
                    let Some(force) = self.fleet.forces.get(&id) else {
                        continue;
                    };
                    seen.extend(force.ships.iter().copied());
                    let ok = force_stopped_at(force, self.params.source_hex)
                        && !self.readying_a_landing(force)
                        && force
                            .ships
                            .iter()
                            .all(|s| self.pool.contains(s) && free(*s));
                    if ok {
                        units.push(Unit {
                            ships: force.ships.iter().copied().collect(),
                            force: Some(id),
                        });
                    }
                }
                _ => {}
            }
        }
        units
    }

    /// The plan, run and days that hold `ship` today, and the plan's intent:
    /// a run of this morning, else the commitment that covers today, else an
    /// unfinished run that holds it. `None` where nothing does.
    fn holder_of(&self, ship: ShipId) -> Option<(IntentId, PlanId, u32, u32, u32)> {
        if let Some(run) = self.morning.holding(ship) {
            return Some((
                run.intent,
                run.plan,
                run.run,
                self.today,
                run.days.free.saturating_sub(1).max(self.today),
            ));
        }
        if let Some(c) = self.commitment_on(ship, self.today) {
            let intent = self.state().plans.get(&c.plan)?.intent;
            return Some((intent, c.plan, c.run.unwrap_or(0), c.from_day, c.to_day));
        }
        for plan in self.state().plans.values() {
            for run in plan.runs.values().filter(|run| run.ships.contains(&ship)) {
                let Some(base) = self.view.bases().find(|b| b.id == run.source) else {
                    continue;
                };
                let found = run_state(self.fleet, self.state(), plan.id, run, base.hex, self.today);
                if found.status != RunStatus::Finished {
                    return Some((
                        plan.intent,
                        plan.id,
                        run.number,
                        run.days.load,
                        run.days.free.saturating_sub(1).max(run.days.load),
                    ));
                }
            }
        }
        None
    }

    /// The commitments that hold the ships of the pool that are not in
    /// `free`, one for each plan and run, with its days.
    pub(crate) fn ship_holders(&self, free: &[Unit]) -> Vec<Displacement> {
        let free: BTreeSet<ShipId> = free
            .iter()
            .flat_map(|unit| unit.ships.iter().copied())
            .collect();
        let mut groups: BTreeMap<(PlanId, u32), Displacement> = BTreeMap::new();
        for ship in self.pool.iter().filter(|ship| !free.contains(ship)) {
            let Some((intent, plan, run, from_day, to_day)) = self.holder_of(*ship) else {
                continue;
            };
            let entry = groups.entry((plan, run)).or_insert_with(|| Displacement {
                intent,
                plan,
                run,
                committed: Committed::Ships(BTreeSet::new()),
                from_day,
                to_day,
            });
            if let Committed::Ships(ships) = &mut entry.committed {
                ships.insert(*ship);
            }
            entry.from_day = entry.from_day.min(from_day);
            entry.to_day = entry.to_day.max(to_day);
        }
        groups.into_values().collect()
    }

    /// The runs loading at the source with tons still to take: those of the
    /// command state and those of this morning.
    pub(crate) fn tons_holders(&self) -> Vec<Displacement> {
        let mut found = Vec::new();
        for plan in self.state().plans.values() {
            for run in plan
                .runs
                .values()
                .filter(|run| run.source == self.params.source)
            {
                let state = run_state(
                    self.fleet,
                    self.state(),
                    plan.id,
                    run,
                    self.params.source_hex,
                    self.today,
                );
                if state.status != RunStatus::Loading {
                    continue;
                }
                let left = (run.load_t - state.aboard_t).max(Fixed::ZERO);
                if left.is_zero() {
                    continue;
                }
                found.push(Displacement {
                    intent: plan.intent,
                    plan: plan.id,
                    run: run.number,
                    committed: Committed::Tons(left),
                    from_day: run.days.load,
                    to_day: run.days.sail.saturating_sub(1).max(run.days.load),
                });
            }
        }
        for run in self
            .morning
            .runs
            .iter()
            .filter(|run| run.source == self.params.source)
        {
            found.push(Displacement {
                intent: run.intent,
                plan: run.plan,
                run: run.run,
                committed: Committed::Tons(run.load_t),
                from_day: run.days.load,
                to_day: run.days.sail.saturating_sub(1).max(run.days.load),
            });
        }
        found
    }

    /// Every unit of the pool at the source, free or not: what a forward
    /// look uses where none is free today.
    fn timing_ships(&self, units: &[Unit]) -> Vec<ShipId> {
        match units.first() {
            Some(unit) => unit.ships.clone(),
            None => self
                .pool
                .iter()
                .next()
                .map(|s| vec![*s])
                .unwrap_or_default(),
        }
    }

    /// Taking supply on top of what is aboard.
    pub(crate) fn ship_room(&self, ship: &Ship) -> Fixed {
        let capacity = self
            .view
            .ship_class(ship.class)
            .map_or(Fixed::ZERO, |class| class.cargo_capacity_t);
        (capacity - ship.supply_t - ship.cargo_fuel_t).max(Fixed::ZERO)
    }

    /// The fuel a ship needs for the round trip at cruise.
    pub(crate) fn fuel_need_t(&self, ship: &Ship, route: &RouteInfo) -> Fixed {
        let Some(class) = self.view.ship_class(ship.class) else {
            return Fixed::ZERO;
        };
        whole(route.out_nm + route.back_nm).mul_div(class.fuel_capacity_t, class.endurance_nm)
    }

    /// The source's stock above its reserve, less `pending_t` still to be
    /// taken there by runs loading; with the stock and the reserve.
    pub(crate) fn source_available(&self, pending_t: Fixed) -> (Fixed, Fixed, Fixed) {
        let Some(base) = self.view.bases().find(|b| b.id == self.params.source) else {
            return (Fixed::ZERO, Fixed::ZERO, Fixed::ZERO);
        };
        let reserve = base.reserve_of(Cargo::Supply);
        // Decision 0228, rules 7 and D: a source the staff chose gives what it
        // can spare, a source the commander named down to its reserve.
        let above = if self.params.staff_source {
            crate::source::base_spare(self.view, base)
        } else {
            base.above_reserve(Cargo::Supply)
        };
        ((above - pending_t).max(Fixed::ZERO), base.supply_t, reserve)
    }

    fn port_at(&self, hex: Hex) -> Option<&'v PortSize> {
        let size = self.view.hex(hex)?.port_size;
        self.view.port_size(size)
    }

    /// The work of moving `lines` (ship, tons) at the port in `hex`: the
    /// first ships, in ship order, up to the port's berths work at the berth
    /// rate and the rest by lighter. `None` where a ship's rate is 0.
    pub(crate) fn work(&self, hex: Hex, lines: &[(ShipId, Fixed)]) -> Option<Work> {
        let port = self.port_at(hex)?;
        let mut hours = Fixed::ZERO;
        let mut out = Vec::new();
        for (index, (ship, tons)) in lines.iter().enumerate() {
            let rate = if (index as u32) < port.berths {
                port.berth_rate_t_per_h
            } else {
                port.lighter_rate_t_per_h
            };
            if *tons > Fixed::ZERO {
                if rate <= Fixed::ZERO {
                    return None;
                }
                hours = hours.max(*tons / rate);
            }
            out.push((*ship, *tons, rate));
        }
        Some(Work {
            hours,
            days: ceil_days(hours).max(1),
            lines: out,
        })
    }

    /// The tons each of `ships` takes in a load of `load_t`, filled in ship
    /// order, and the supply it then holds.
    pub(crate) fn distribute(
        &self,
        ships: &[ShipId],
        load_t: Fixed,
    ) -> Vec<(ShipId, Fixed, Fixed)> {
        let mut left = load_t;
        let mut lines = Vec::new();
        for id in ships {
            let Some(ship) = self.fleet.ships.get(id) else {
                continue;
            };
            let add = left.min(self.ship_room(ship));
            left -= add;
            lines.push((*id, add, ship.supply_t + add));
        }
        lines
    }

    /// The times of a run of `ships` that takes `load_t` aboard.
    pub(crate) fn timing(
        &self,
        ships: &[ShipId],
        load_t: Fixed,
        route: &RouteInfo,
    ) -> Option<Timing> {
        let lines = self.distribute(ships, load_t);
        self.timing_of(ships, &lines, route)
    }

    /// The times of a run of `ships` that has already loaded: the load takes
    /// no time and the unloading is of what each holds now.
    pub(crate) fn timing_loaded(&self, ships: &[ShipId], route: &RouteInfo) -> Option<Timing> {
        let lines: Vec<(ShipId, Fixed, Fixed)> = ships
            .iter()
            .filter_map(|id| self.fleet.ships.get(id))
            .map(|ship| (ship.id, Fixed::ZERO, ship.supply_t))
            .collect();
        self.timing_of(ships, &lines, route)
    }

    fn timing_of(
        &self,
        ships: &[ShipId],
        lines: &[(ShipId, Fixed, Fixed)],
        route: &RouteInfo,
    ) -> Option<Timing> {
        let speed = ships
            .iter()
            .filter_map(|id| self.fleet.ships.get(id))
            .filter_map(|ship| self.view.ship_class(ship.class))
            .map(|class| class.cruise_speed_kn)
            .min()?;
        if speed <= Fixed::ZERO {
            return None;
        }
        let load = self.work(
            self.params.source_hex,
            &lines
                .iter()
                .map(|(s, add, _)| (*s, *add))
                .collect::<Vec<_>>(),
        )?;
        let unload_lines: Vec<(ShipId, Fixed)> =
            lines.iter().map(|(s, _, after)| (*s, *after)).collect();
        let unload = self.work(self.params.destination_hex, &unload_lines)?;
        let out_hours = whole(route.out_nm) / speed;
        let back_hours = whole(route.back_nm) / speed;
        let out_days = ceil_days(out_hours).max(1);
        let back_days = ceil_days(back_hours).max(1);
        let into_last = out_hours - Fixed::from_int(24) * whole(out_days - 1);
        let left = (Fixed::from_int(24) - into_last).max(Fixed::ZERO);
        let landed = unload
            .lines
            .iter()
            .fold(Fixed::ZERO, |sum, (_, tons, rate)| {
                sum + (*tons).min(*rate * left)
            });
        let (fuelling_hours, fuelling_rate) = self.fuelling_on_return(ships, route);
        Some(Timing {
            speed_kn: speed,
            load,
            out_hours,
            out_days,
            back_hours,
            back_days,
            unload,
            landed_on_arrival_t: landed,
            hours_left_on_arrival: left,
            fuelling_hours,
            fuelling_rate,
        })
    }

    /// The hours `ships` take to fuel at the source on their return, and the
    /// rate (decision 0112): each ship lacks its capacity less the fuel it
    /// has now plus what the round trip burns at cruise, not below 0 and not
    /// above its capacity; the rate is the source's fuel berth rate where it
    /// has a berth, else its lighter rate. The longest ship's hours. A rate
    /// not given is no limit: no hours, and no rate.
    fn fuelling_on_return(&self, ships: &[ShipId], route: &RouteInfo) -> (Fixed, Option<Fixed>) {
        let Some(port) = self.port_at(self.params.source_hex) else {
            return (Fixed::ZERO, None);
        };
        let rate = if port.berths > 0 {
            port.fuel_berth_rate_t_per_h
        } else {
            port.fuel_lighter_rate_t_per_h
        };
        let Some(rate) = rate else {
            return (Fixed::ZERO, None);
        };
        let mut hours = Fixed::ZERO;
        for ship in ships.iter().filter_map(|id| self.fleet.ships.get(id)) {
            let Some(class) = self.view.ship_class(ship.class) else {
                continue;
            };
            let on_return = (ship.fuel_t - self.fuel_need_t(ship, route))
                .max(Fixed::ZERO)
                .min(class.fuel_capacity_t);
            let lacks = class.fuel_capacity_t - on_return;
            hours = hours.max(lacks / rate);
        }
        (hours, Some(rate))
    }

    /// The units, in order, that carry `uncapped_t` (the sum's load), where
    /// the source gives at most `available_t` more: the free units in number
    /// order until what is aboard them and the room they have to take more
    /// covers it, and at least one. A unit with a ship short of fuel for the
    /// round trip is set aside. Supply aboard counts toward the load
    /// (ruling 22), so with nothing aboard the room covers the load or the
    /// source's tons, whichever is less, as before.
    fn choose(
        &self,
        units: &[Unit],
        uncapped_t: Fixed,
        available_t: Fixed,
        route: &RouteInfo,
    ) -> Chosen {
        let mut chosen = Chosen {
            units: Vec::new(),
            ships: Vec::new(),
            room_t: Fixed::ZERO,
            aboard_t: Fixed::ZERO,
            without_fuel: Vec::new(),
        };
        for unit in units {
            let short: Vec<ShipId> = unit
                .ships
                .iter()
                .copied()
                .filter(|id| {
                    self.fleet
                        .ships
                        .get(id)
                        .is_some_and(|ship| ship.fuel_t < self.fuel_need_t(ship, route))
                })
                .collect();
            if !short.is_empty() {
                chosen.without_fuel.extend(short);
                continue;
            }
            chosen.units.push(unit.clone());
            chosen.ships.extend(unit.ships.iter().copied());
            chosen.room_t += unit
                .ships
                .iter()
                .filter_map(|id| self.fleet.ships.get(id))
                .fold(Fixed::ZERO, |sum, ship| sum + self.ship_room(ship));
            chosen.aboard_t += self.fleet.supply_aboard(&unit.ships);
            let to_take = (uncapped_t - chosen.aboard_t).max(Fixed::ZERO);
            if chosen.room_t >= to_take.min(available_t) {
                break;
            }
        }
        chosen
    }

    /// One pass of the sizing: the times from a guess of the ships and the
    /// load, the load that gives, and the ships that carry it.
    fn pass(
        &self,
        readings: &Readings,
        order_day: u32,
        units: &[Unit],
        route: &RouteInfo,
        available_t: Fixed,
        guess: &Guess,
    ) -> Option<Pass> {
        let timing = self.timing(&guess.ships, guess.taken_t, route)?;
        let arrive = order_day + timing.load.days + timing.out_days - 1;
        let stock_a = readings.stock(arrive)?;
        let use_a = readings.use_on(arrive)?;
        let floor_t = whole(self.params.floor_days) * use_a;
        let need_t = (floor_t - stock_a).max(Fixed::ZERO);
        let interval_use_t = readings.use_over(arrive, self.params.interval_days);
        let uncapped_t = need_t + interval_use_t;
        let chosen = self.choose(units, uncapped_t, available_t, route);
        // What is aboard counts toward the load: the run carries the larger
        // of the sum's load and what is aboard, and takes the difference,
        // within the source's tons and the ships' room (ruling 22).
        let to_take = (uncapped_t - chosen.aboard_t).max(Fixed::ZERO);
        let taken_t = to_take.min(available_t).min(chosen.room_t);
        Some(Pass {
            arrive,
            stock_a,
            floor_t,
            interval_use_t,
            uncapped_t,
            chosen,
            taken_t,
        })
    }

    /// A run ordered on `order_day`, on the projection `readings`, from the
    /// free `units`, with `pending_t` still to be taken from the source by
    /// runs loading. `None` where no time can be worked (no route at all,
    /// no ship to time, or a day the projection does not reach).
    pub(crate) fn evaluate(
        &self,
        readings: &Readings,
        order_day: u32,
        units: &[Unit],
        pending_t: Fixed,
    ) -> Option<Evaluation> {
        let route = self.route.as_ref()?;
        let (available_t, stock_t, reserve_t) = self.source_available(pending_t);
        let mut guess = Guess {
            ships: self.timing_ships(units),
            taken_t: Fixed::ZERO,
        };
        let mut last: Option<Pass> = None;
        for _ in 0..5 {
            let pass = self.pass(readings, order_day, units, route, available_t, &guess)?;
            let converged = pass.taken_t == guess.taken_t
                && (pass.chosen.ships == guess.ships || pass.chosen.ships.is_empty());
            if !pass.chosen.ships.is_empty() {
                guess.ships = pass.chosen.ships.clone();
            }
            guess.taken_t = pass.taken_t;
            last = Some(pass);
            if converged {
                break;
            }
        }
        let Pass {
            arrive,
            stock_a,
            floor_t,
            interval_use_t,
            uncapped_t,
            chosen,
            taken_t,
        } = last?;
        let check_day = arrive + 1;
        let check_stock_t = readings.stock(check_day)?;
        let check_use_t = readings.use_on(check_day)?;
        let needed = readings.below_floor(check_day, self.params.floor_days) == Some(true);

        let mut blocker = None;
        let mut run = None;
        if needed {
            if !route.in_area {
                blocker = Some(Blocker::NoRoute);
            } else if units.is_empty() {
                blocker = Some(Blocker::NoShip {
                    free_from: self.earliest_free(),
                    waiting: self.waiting.clone(),
                });
            } else if chosen.ships.is_empty() {
                blocker = Some(Blocker::Fuel {
                    ships: chosen.without_fuel.clone(),
                });
            } else if (chosen.aboard_t + taken_t).is_zero() {
                blocker = Some(Blocker::Source {
                    stock_t,
                    reserve_t,
                    pending_t,
                });
            } else {
                let mut cuts = Vec::new();
                let to_take = (uncapped_t - chosen.aboard_t).max(Fixed::ZERO);
                if to_take > available_t && self.params.staff_source {
                    cuts.push(format!(
                        "the source can spare {} t, so the load is cut to what the source can spare ({} t of its stock, {} t it keeps, {} t still to be taken by runs loading)",
                        available_t,
                        stock_t,
                        (stock_t - available_t - pending_t).max(Fixed::ZERO),
                        pending_t
                    ));
                } else if to_take > available_t {
                    cuts.push(format!(
                        "the source has {} t above its reserve of {} t to give ({} t of its stock, {} t still to be taken by runs loading)",
                        available_t, reserve_t, stock_t, pending_t
                    ));
                }
                if to_take.min(available_t) > chosen.room_t {
                    cuts.push(format!("the ships' room is {} t", chosen.room_t));
                }
                let days_timing = self.timing(&chosen.ships, taken_t, route)?;
                let aboard_before_t = chosen.aboard_t;
                run = Some(RunPlan {
                    days: days_timing.days(order_day),
                    timing: days_timing,
                    units: chosen.units.clone(),
                    ships: chosen.ships.clone(),
                    load_t: aboard_before_t + taken_t,
                    taken_t,
                    weight_t: aboard_before_t + taken_t,
                    aboard_before_t,
                    cuts,
                    night: None,
                    level: None,
                });
            }
        }
        Some(Evaluation {
            order_day,
            arrive,
            check_day,
            check_stock_t,
            check_use_t,
            needed,
            stock_at_arrival_t: stock_a,
            floor_t,
            interval_use_t,
            uncapped_t,
            run,
            blocker,
        })
    }

    /// The first day from `from` on, up to the end of the projection, on
    /// which a run ordered would be needed, by the same rule.
    pub(crate) fn next_needed(
        &self,
        readings: &Readings,
        from: u32,
        units: &[Unit],
        pending_t: Fixed,
    ) -> Option<Evaluation> {
        (from..=readings.last_day()).find_map(|day| {
            self.evaluate(readings, day, units, pending_t)
                .filter(|evaluation| evaluation.needed)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn work(days: u32) -> Work {
        Work {
            hours: Fixed::from_int(i64::from(days) * 24),
            days,
            lines: Vec::new(),
        }
    }

    /// A run that sails at 20 hours out, unloads for one day and sails 20
    /// hours back, with `fuelling_hours` to fuel at the source.
    fn timing(out: i64, back: i64, unload_days: u32, fuelling_hours: Fixed) -> Timing {
        Timing {
            speed_kn: Fixed::from_int(16),
            load: work(1),
            out_hours: Fixed::from_int(out),
            out_days: ceil_days(Fixed::from_int(out)).max(1),
            back_hours: Fixed::from_int(back),
            back_days: ceil_days(Fixed::from_int(back)).max(1),
            unload: work(unload_days),
            landed_on_arrival_t: Fixed::ZERO,
            hours_left_on_arrival: Fixed::ZERO,
            fuelling_hours,
            fuelling_rate: None,
        }
    }

    /// Hand figures. Out 20 hours (1 day), unloading 1 day, back 20 hours
    /// (1 day): the ship is back at the source at hour 24 x (1 + 1) + 20 =
    /// 68 of the run, counted from the first hour of the sail day. Today's
    /// sum is 1 + 1 + 1 = 3 days after the sail day.
    ///
    /// - A source with no rate given (no limit): 0 fuelling hours, the hour
    ///   is 68, ceil(68 / 24) = 3: free 3 days after the sail day, as before.
    /// - A berth at 200 t an hour and a ship lacking 1000 t: 5 hours, done at
    ///   hour 73 in the day that starts at hour 72 (the fourth day, index 3):
    ///   free the morning after it, 4 days after the sail day.
    /// - The same ship lacking 600 t: 3 hours, done at hour 71 in the third
    ///   day (index 2): free 3 days after the sail day, the same as before.
    #[test]
    fn a_ship_is_free_the_morning_after_the_day_it_finishes_fuelling() {
        let none = timing(20, 20, 1, Fixed::ZERO);
        assert_eq!(none.back_and_fuelled_hour(), Fixed::from_int(68));
        assert_eq!(none.free_after_sail(), 3);
        assert_eq!(none.days(10).free, 10 + 1 + 3);

        let long = timing(20, 20, 1, Fixed::from_int(1000) / Fixed::from_int(200));
        assert_eq!(long.back_and_fuelled_hour(), Fixed::from_int(73));
        assert_eq!(long.free_after_sail(), 4);
        // Ordered on day 10 with a loading day: sails on day 11, free on 15.
        assert_eq!(long.days(10).free, 15);

        let short = timing(20, 20, 1, Fixed::from_int(600) / Fixed::from_int(200));
        assert_eq!(short.back_and_fuelled_hour(), Fixed::from_int(71));
        assert_eq!(short.free_after_sail(), 3);
    }

    /// With no rate given the free day is the old sum in every case: a
    /// passage of a whole number of days, of part of one, and none.
    #[test]
    fn with_no_rate_given_the_free_day_is_the_whole_days_sum_as_before() {
        for out in [1, 20, 24, 25, 48, 100] {
            for back in [0, 1, 20, 24, 25, 48, 100] {
                for unload_days in [1, 2, 3] {
                    let t = timing(out, back, unload_days, Fixed::ZERO);
                    assert_eq!(
                        t.free_after_sail(),
                        t.out_days + t.unload.days + t.back_days,
                        "out {out} back {back} unload {unload_days}"
                    );
                }
            }
        }
    }

    /// Fuelling that ends after midnight pushes the day on, never back: a
    /// ship that is back at hour 24 x 2 + 0 (no passage) and takes 30 hours
    /// finishes at hour 78, day index 3: free 4 days after the sail day.
    #[test]
    fn fuelling_never_makes_a_ship_free_earlier() {
        let t = timing(20, 0, 1, Fixed::from_int(30));
        assert_eq!(t.back_and_fuelled_hour(), Fixed::from_int(78));
        assert_eq!(t.free_after_sail(), 4);
        let t = timing(20, 0, 1, Fixed::ZERO);
        assert_eq!(t.free_after_sail(), 3);
    }
}
