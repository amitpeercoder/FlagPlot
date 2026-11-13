//! The game: reference data, truth, both sides' beliefs, orders and events.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use crate::airstrike::StrikeInFlight;
use crate::belief::BeliefState;
use crate::events::EventLog;
use crate::hash;
use crate::ids::{AirGroupId, BaseId, EntityRef, IdAllocator, OrderId, TaskForceId};
use crate::invariants::{self, InvariantViolation};
use crate::map::Map;
use crate::objectives::{self, GameResult, Objectives};
use crate::orders::{Order, OrderRequest};
use crate::refdata::RefData;
use crate::rng::{Draw, RngKey, RngStream, Subsystem};
use crate::state::{ShipSystem, Side, SystemState, TruthState};
use crate::time::Pulse;
use crate::upkeep::Sortie;
use crate::view::SideView;

/// The first number of a side's task forces and air groups made during play
/// (decision 0086): the Allied side's series starts at 1,000,001 and the
/// Japanese side's at 2,000,001. A scenario's own numbers are below
/// [`MADE_IDS_FLOOR`], so the series never meet them.
pub const MADE_IDS_FLOOR: u32 = 1_000_000;

/// One counter for each side and each kind of unit made during play
/// (decision 0086). A unit's number, and so its random draws, depend on its
/// own side's history only.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MadeIds {
    task_forces: BTreeMap<Side, u32>,
    air_groups: BTreeMap<Side, u32>,
}

impl MadeIds {
    pub fn new() -> Self {
        let start = |side: Side| match side {
            Side::Allied => MADE_IDS_FLOOR + 1,
            Side::Japanese => 2 * MADE_IDS_FLOOR + 1,
        };
        let series = || {
            Side::ALL
                .into_iter()
                .map(|side| (side, start(side)))
                .collect()
        };
        Self {
            task_forces: series(),
            air_groups: series(),
        }
    }

    /// The next task force number of `side`'s series.
    pub fn alloc_task_force(&mut self, side: Side) -> TaskForceId {
        TaskForceId::new(Self::take(&mut self.task_forces, side))
    }

    /// The next air group number of `side`'s series.
    pub fn alloc_air_group(&mut self, side: Side) -> AirGroupId {
        AirGroupId::new(Self::take(&mut self.air_groups, side))
    }

    fn take(counters: &mut BTreeMap<Side, u32>, side: Side) -> u32 {
        let next = counters.get_mut(&side).expect("a counter for each side");
        let id = *next;
        *next = next.checked_add(1).expect("ran out of entity IDs");
        id
    }

    /// The number the next task force of `side` will have. Part of the hash.
    pub fn next_task_force(&self, side: Side) -> u32 {
        self.task_forces[&side]
    }

    /// The number the next air group of `side` will have. Part of the hash.
    pub fn next_air_group(&self, side: Side) -> u32 {
        self.air_groups[&side]
    }
}

impl Default for MadeIds {
    fn default() -> Self {
        Self::new()
    }
}

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
    /// Hands out the numbers of task forces and air groups made during play,
    /// from each side's own series (decisions 0016 and 0086).
    pub(crate) made_ids: MadeIds,
    /// Orders submitted for the coming day, not yet taken in.
    pub(crate) pending: BTreeMap<Side, Vec<OrderRequest>>,
    pub(crate) events: EventLog,
    pub(crate) day_hashes: Vec<u64>,
    /// Strikes launched in a pulse's decision step that have not landed yet.
    /// Empty outside the air steps of one pulse, so not hashed.
    pub(crate) strikes_in_flight: Vec<StrikeInFlight>,
    /// What each air group flew in the pulse so far, for the wear step
    /// (decision 0035). Filled by the search and air combat steps only when
    /// the wear figures are loaded, emptied by the wear step of the same
    /// pulse, so not hashed.
    pub(crate) sorties: Vec<Sortie>,
    /// The bases with no supply at the start of the pulse (decision 7 of
    /// 0039): their groups fly nothing in it. Worked out once at the start
    /// of each pulse, so not hashed.
    pub(crate) grounded_bases: BTreeSet<BaseId>,
    /// The ships that sank, or whose flight deck went out, in the air combat
    /// step of this pulse, with where and what caused it (decision 7 of
    /// 0059). Read by the landing step of the same pulse's wear phase and
    /// emptied at the next air combat step, so not hashed.
    pub(crate) deck_losses: Vec<crate::landing::DeckLoss>,
    /// The scenario's missions and objectives (decision 0054). Fixed
    /// configuration, like the reference data: no rule reads it and nothing
    /// in it changes, so it is not state and is not hashed. The result is a
    /// function of the final truth and the event log.
    pub(crate) objectives: Option<Objectives>,
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
        if let Some(problem) = ref_data
            .air_upkeep
            .as_ref()
            .and_then(|figures| figures.problem())
        {
            return Err(SimError::InvalidSetup(problem));
        }
        if let Some(problem) = ref_data
            .fatigue
            .as_ref()
            .and_then(|figures| figures.problem())
        {
            return Err(SimError::InvalidSetup(problem));
        }
        if let Some(problem) = ref_data
            .air_supply
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

        Ok(World {
            seed,
            config: SimConfig::default(),
            ref_data,
            map,
            truth,
            beliefs,
            orders: BTreeMap::new(),
            order_ids: IdAllocator::new(),
            made_ids: MadeIds::new(),
            pending: BTreeMap::new(),
            events: EventLog::default(),
            day_hashes: Vec::new(),
            strikes_in_flight: Vec::new(),
            sorties: Vec::new(),
            grounded_bases: BTreeSet::new(),
            deck_losses: Vec::new(),
            objectives: None,
        })
    }

    /// Gives the game its missions and objectives (decision 0054). Fails if
    /// an objective names a base the game does not have. Changes no state
    /// and no hash.
    pub fn set_objectives(&mut self, objectives: Objectives) -> Result<(), SimError> {
        for base in objectives.bases_named() {
            if !self.truth.bases.contains_key(&base) {
                return Err(SimError::InvalidSetup(format!(
                    "an objective names unknown {base}"
                )));
            }
        }
        self.objectives = Some(objectives);
        Ok(())
    }

    /// The scenario's missions and objectives, if it has any. Both sides' are
    /// here: this is for the loader, the runner and the umpire, not for
    /// anything that plays the game, which has [`SideView::mission`] and
    /// [`SideView::objectives`].
    pub fn objectives(&self) -> Option<&Objectives> {
        self.objectives.as_ref()
    }

    /// The result as the final truth and the log give it, for the umpire.
    /// `None` if the scenario has no objectives. It reads the truth as it
    /// stands, so it is a result only when the game is over; asked sooner it
    /// is the score so far. Never hand it to a side before the game ends.
    pub fn result(&self) -> Option<GameResult> {
        let objectives = self.objectives.as_ref()?;
        Some(objectives::result(
            objectives,
            &self.truth,
            &self.events,
            &self.ref_data,
        ))
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
        SideView::new(
            side,
            &self.truth,
            belief,
            &self.map,
            &self.ref_data,
            &self.events,
            &self.orders,
            self.objectives.as_ref(),
        )
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

    /// The umpire's key to the numbers of event, order and contact IDs.
    /// Whoever holds a `World` is the umpire; a side holds a `SideView`, and
    /// a `SideView` has no way to this key (decision 0086).
    pub fn umpire_key(&self) -> crate::ids::UmpireKey {
        crate::ids::UmpireKey::new()
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
    /// `draw` names the rule that draws; a new rule adds its own value to
    /// [`Draw`] (decision 0055).
    pub fn rng(
        &self,
        day: u32,
        pulse: Option<Pulse>,
        subsystem: Subsystem,
        draw: Draw,
        entity: EntityRef,
    ) -> RngStream {
        RngKey {
            seed: self.seed,
            day,
            pulse,
            subsystem,
            entity,
            other: None,
            draw,
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
        draw: Draw,
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
            draw,
        }
        .stream()
    }
}
