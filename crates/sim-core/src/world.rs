//! The game: reference data, truth, both sides' beliefs, orders and events.

use core::fmt;
use std::collections::BTreeMap;

use crate::belief::BeliefState;
use crate::events::EventLog;
use crate::hash;
use crate::ids::{EntityRef, IdAllocator, OrderId, TaskForceId};
use crate::invariants::{self, InvariantViolation};
use crate::map::Map;
use crate::orders::{Order, OrderRequest};
use crate::refdata::RefData;
use crate::rng::{RngKey, RngStream, Subsystem};
use crate::state::{ShipSystem, Side, SystemState, TruthState};
use crate::time::Pulse;
use crate::view::SideView;

/// Switches that change what is recorded, never what happens.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SimConfig {
    /// Record the factors behind each computed quantity. Batch validation can
    /// turn this off for speed. Events are recorded either way, and the state
    /// hash is the same either way.
    pub record_breakdowns: bool,
}

impl Default for SimConfig {
    fn default() -> Self {
        SimConfig {
            record_breakdowns: true,
        }
    }
}

/// Why the simulation could not start or continue.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SimError {
    /// The starting state or reference data is not usable.
    InvalidSetup(String),
    /// The state broke an invariant. This is always a bug in the rules.
    InvariantsViolated {
        day: u32,
        violations: Vec<InvariantViolation>,
    },
}

impl fmt::Display for SimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimError::InvalidSetup(message) => write!(f, "invalid setup: {message}"),
            SimError::InvariantsViolated { day, violations } => {
                write!(f, "invariants violated on day {day}:")?;
                for violation in violations {
                    write!(f, " {violation};")?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for SimError {}

/// What a resolved day produced.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DaySummary {
    pub day: u32,
    pub state_hash: u64,
    /// Position in the event log where this day's events begin.
    pub first_event: usize,
    pub event_count: usize,
}

/// A game in progress.
#[derive(Clone, Debug)]
pub struct World {
    pub(crate) seed: u64,
    pub(crate) config: SimConfig,
    pub(crate) ref_data: RefData,
    pub(crate) map: Map,
    pub(crate) truth: TruthState,
    pub(crate) beliefs: BTreeMap<Side, BeliefState>,
    /// Every order received, accepted or not, by ID.
    pub(crate) orders: BTreeMap<OrderId, Order>,
    pub(crate) order_ids: IdAllocator<OrderId>,
    /// Hands out IDs to task forces made during a day (decision 0016). Every
    /// starting task force's ID is reserved.
    pub(crate) task_force_ids: IdAllocator<TaskForceId>,
    /// Orders submitted for the coming day, not yet taken in.
    pub(crate) pending: BTreeMap<Side, Vec<OrderRequest>>,
    pub(crate) events: EventLog,
    pub(crate) day_hashes: Vec<u64>,
}

impl World {
    /// Starts a game. Fails if the reference data is unusable (a ship class,
    /// port size, aircraft type, the search figures or the surface combat
    /// figures with bad figures), a
    /// ship has an unknown class, an air group has an unknown aircraft type,
    /// there are air groups but no search figures, a base
    /// sits in a port size with no entry in the port table, or the starting
    /// state already breaks an invariant.
    pub fn new(
        seed: u64,
        ref_data: RefData,
        map: Map,
        truth: TruthState,
    ) -> Result<World, SimError> {
        for class in ref_data.ship_classes.values() {
            if let Some(problem) = class.problem() {
                return Err(SimError::InvalidSetup(problem));
            }
        }
        for aircraft_type in ref_data.aircraft_types.values() {
            if let Some(problem) = aircraft_type.problem() {
                return Err(SimError::InvalidSetup(problem));
            }
        }
        if let Some(problem) = ref_data.search.as_ref().and_then(|search| search.problem()) {
            return Err(SimError::InvalidSetup(problem));
        }
        if let Some(problem) = ref_data
            .surface_combat
            .as_ref()
            .and_then(|figures| figures.problem())
        {
            return Err(SimError::InvalidSetup(problem));
        }
        for group in truth.air_groups.values() {
            if !ref_data.aircraft_types.contains_key(&group.aircraft_type) {
                return Err(SimError::InvalidSetup(format!(
                    "{} has unknown aircraft type {}",
                    group.id, group.aircraft_type
                )));
            }
        }
        if !truth.air_groups.is_empty() && ref_data.search.is_none() {
            return Err(SimError::InvalidSetup(
                "there are air groups but no search figures".to_string(),
            ));
        }
        if ref_data.surface_combat.is_some() && ref_data.search.is_none() {
            // A fight reports what it sighted through the search figures.
            return Err(SimError::InvalidSetup(
                "there are surface combat figures but no search figures".to_string(),
            ));
        }
        for ship in truth.ships.values() {
            if !ref_data.ship_classes.contains_key(&ship.class) {
                return Err(SimError::InvalidSetup(format!(
                    "{} has unknown class {}",
                    ship.id, ship.class
                )));
            }
        }

        for (size, port) in &ref_data.port_sizes {
            if let Some(problem) = port.problem(*size) {
                return Err(SimError::InvalidSetup(problem));
            }
        }
        for base in truth.bases.values() {
            // A base off the map is reported by the invariant check below.
            if let Some(hex) = map.get(base.hex) {
                if !ref_data.port_sizes.contains_key(&hex.port_size) {
                    return Err(SimError::InvalidSetup(format!(
                        "{} is in a hex of port size {}, which has no entry in the port table",
                        base.id, hex.port_size
                    )));
                }
            }
        }

        if ref_data.surface_combat.is_none() {
            // Without the figures a damaged system has no factor to apply.
            for ship in truth.ships.values() {
                for system in ShipSystem::ALL {
                    if ship.systems.get(system) != SystemState::Working {
                        return Err(SimError::InvalidSetup(format!(
                            "{} has {} {} but there are no surface combat figures",
                            ship.id,
                            system.name(),
                            ship.systems.get(system).name()
                        )));
                    }
                }
            }
        }

        let violations = invariants::check(&truth, &map, &ref_data);
        if !violations.is_empty() {
            return Err(SimError::InvariantsViolated {
                day: truth.day,
                violations,
            });
        }

        let beliefs = Side::ALL
            .into_iter()
            .map(|side| (side, BeliefState::new(side)))
            .collect();

        let mut task_force_ids = IdAllocator::new();
        for id in truth.task_forces.keys() {
            task_force_ids.reserve(*id);
        }

        Ok(World {
            seed,
            config: SimConfig::default(),
            ref_data,
            map,
            truth,
            beliefs,
            orders: BTreeMap::new(),
            order_ids: IdAllocator::new(),
            task_force_ids,
            pending: BTreeMap::new(),
            events: EventLog::default(),
            day_hashes: Vec::new(),
        })
    }

    pub fn set_config(&mut self, config: SimConfig) {
        self.config = config;
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// The last day resolved. 0 before the first day.
    pub fn day(&self) -> u32 {
        self.truth.day
    }

    /// Queues orders from one side for the coming day. They take effect, or
    /// are rejected, at the start of the next [`World::resolve_day`].
    pub fn submit_orders(&mut self, side: Side, requests: Vec<OrderRequest>) {
        self.pending.entry(side).or_default().extend(requests);
    }

    /// What one side is allowed to see. This is what a player, the staff
    /// planner or an agent is given.
    pub fn side_view(&self, side: Side) -> SideView<'_> {
        let belief = self
            .beliefs
            .get(&side)
            .expect("both sides have a belief state");
        SideView::new(side, &self.truth, belief, &self.map, &self.ref_data)
    }

    /// The truth. For tests, the command-line runner and hashing only. Never
    /// hand this to anything that plays the game.
    pub fn umpire(&self) -> &TruthState {
        &self.truth
    }

    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn ref_data(&self) -> &RefData {
        &self.ref_data
    }

    pub fn events(&self) -> &EventLog {
        &self.events
    }

    /// Every order received so far, accepted or not, in ID order.
    pub fn orders(&self) -> impl Iterator<Item = &Order> {
        self.orders.values()
    }

    pub fn order(&self, id: OrderId) -> Option<&Order> {
        self.orders.get(&id)
    }

    /// The state hash at the end of each resolved day, in order.
    pub fn day_hashes(&self) -> &[u64] {
        &self.day_hashes
    }

    /// The hash of the state as it stands now.
    pub fn state_hash(&self) -> u64 {
        hash::hash_world(self)
    }

    /// Opens the random number stream for one entity in one subsystem at one
    /// moment. This is the only source of randomness the rules may use.
    pub fn rng(
        &self,
        day: u32,
        pulse: Option<Pulse>,
        subsystem: Subsystem,
        entity: EntityRef,
    ) -> RngStream {
        RngKey {
            seed: self.seed,
            day,
            pulse,
            subsystem,
            entity,
            other: None,
        }
        .stream()
    }

    /// Opens the random number stream for a pair of entities, such as an air
    /// group looking for a task force. See
    /// `docs/decisions/0013-random-keys-for-pairs.md`. The order of `entity`
    /// and `other` is fixed by the rule that draws, and swapping them gives a
    /// different stream.
    pub fn rng_pair(
        &self,
        day: u32,
        pulse: Option<Pulse>,
        subsystem: Subsystem,
        entity: EntityRef,
        other: EntityRef,
    ) -> RngStream {
        RngKey {
            seed: self.seed,
            day,
            pulse,
            subsystem,
            entity,
            other: Some(other),
        }
        .stream()
    }
}
