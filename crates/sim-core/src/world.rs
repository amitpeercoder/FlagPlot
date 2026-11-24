//! The game: reference data, truth, both sides' beliefs, orders and events.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use crate::airstrike::StrikeInFlight;
use crate::belief::BeliefState;
use crate::choices::{Answer, Moment, Question, Trace, TraceCell, TracedChoice};
use crate::command::{CommandKind, CommandState, StaffDecisions};
use crate::events::EventLog;
use crate::fixed::Fixed;
use crate::hash;
use crate::ids::{AirGroupId, BaseId, EntityRef, IdAllocator, OrderId, TaskForceId};
use crate::invariants::{self, InvariantViolation};
use crate::map::Map;
use crate::objectives::{self, GameResult, Objectives};
use crate::orders::{Order, OrderRequest};
use crate::refdata::RefData;
use crate::rng::{Domain, Draw, RngKey, RngStream, Subsystem};
use crate::state::{ShipSystem, Side, SystemState, TruthState};
use crate::time::Pulse;
use crate::upkeep::Sortie;
use crate::view::{PulseScratch, SideView};

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
        let series = || {
            Side::ALL
                .into_iter()
                .map(|side| (side, Self::first(side)))
                .collect()
        };
        Self {
            task_forces: series(),
            air_groups: series(),
        }
    }

    /// The counters as a picture gives them (decision 0170, 10): for each
    /// side, the next task force and the next air group. A side not given
    /// starts at its first numbers.
    pub(crate) fn at(next: &BTreeMap<Side, (u32, u32)>) -> Self {
        let mut made = Self::new();
        for (side, (task_force, air_group)) in next {
            made.task_forces.insert(*side, *task_force);
            made.air_groups.insert(*side, *air_group);
        }
        made
    }

    /// The first number of `side`'s series: its counters start here.
    pub const fn first(side: Side) -> u32 {
        match side {
            Side::Allied => MADE_IDS_FLOOR + 1,
            Side::Japanese => 2 * MADE_IDS_FLOOR + 1,
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
/// The label of the rules a game is played under (decision 0100, 8). A
/// saved game records it with the version of the state's layout, and a
/// replay under other rules is refused before it runs. It began at 1 with
/// decision 0100, is 2 from decision 0103 (an airfield damaged and mended,
/// which changes scenario 24), 3 from decision 0104 (a beaten side falls
/// back, which changes scenarios 23 and 30) and 4 from decision 0105 (who is
/// engaged in a fight ashore and an attack that fails, which changes every
/// live fight ashore) and 5 from decision 0106 (cover, fieldworks and
/// strain, which changes every live scenario with men ashore under fire or
/// in a fight), 6 from decision 0111 (a load up to a weight, a reserve at
/// a base, and forces formed, joined and stood down: orders and state a game
/// under the old rules could not hold) and 7 from decision 0109 (a unit
/// falling back is out of a fight its own side starts, which changes every
/// live fight where one is) and 8 from the architect's ruling on decision
/// 0106 (a unit's loss to fire is cut to a whole man once, which changes the
/// live scenarios with men under fire behind cover) and 9 from decision 0110
/// (a side's men report an empty hex, which changes every live scenario where
/// a side holds an estimate of a hex its fighting men stand in and the enemy
/// has left) and 10 from step 3 of decision 0107 (an order of the commander's
/// to a unit in an intent's forces limit holds it, and the staff's orders to
/// what is held are refused: a game saved under the old rules could play
/// differently) and 11 from decision 0112 (refuelling takes the hours of a
/// pulse at the port's fuelling rate, in the cargo rule's queue, which changes
/// every live scenario with a force in port) and 12 from decision 0114 (two
/// standing orders of a strike group: a game saved under the old rules could
/// not hold them) and 13 from decision 0118 (a strike sized to its target:
/// an order a game saved under the old rules could not hold, and the live
/// figures' table of aircraft for each ship) and 14 from decision 0124 (a
/// route's marks to stand at for the night: a route a game saved under the
/// old rules could not hold) and 15 from decision 0121 (an escort force: a
/// force's charge, an order a game saved under the old rules could not
/// hold) and 16 from decision 0122 (rearming: a base's ammunition and a
/// ship's tubes fired, which a game saved under the old rules could not
/// hold) and 17 from decision 0131 (a search flown once a day and a search's
/// own fatigue figure: an order a game saved under the old rules could not
/// hold, and the live figure changes every live game in which a group
/// searches) and 18 from decision 0133 (a contact reported this pulse is
/// refreshed again in it only from a hex away, which changes every live game
/// with two reports two hexes apart in one pulse) and 19 from decision 0147 (an
/// estimate's grade and the reconnaissance mission, which a game saved under
/// the old rules could not hold, and the live figures' two reconnaissance
/// figures) and 20 from decision 0162 (a load to a weight may name the most
/// a ship is to hold: an order a game saved under the old rules could not
/// hold, and the staff's supply run by night names it) and 21 from decision
/// 0191 (the order to do no cargo work, which a game saved under the old
/// rules could not hold, and a lift turned back for carriers near its way,
/// which changes games with a take and a carrier contact held near its
/// lift) and 22 from decision 0192 (a stand's force with no home port
/// stands from where it is, and one with a home port stopped at sea goes
/// home, which changes games with such a stand) and 23 from decision 0190
/// (the targets limit and an air group's waters to strike first, which a
/// game saved under the old rules could not hold) and 24 from decision
/// 0193 (men ashore with no more coming attack at balanced and at press,
/// which changes games with a take whose men ashore wait with nothing more
/// coming) and 25 from decision 0194 (no attack ashore begun in a game's
/// last days, and nothing coming after its end, which changes games with men
/// ashore in their last days) and 26 from decision 0201 (fuelling at sea: an
/// order and a task force's field that a game saved under the old rules
/// could not hold) and 27 from decision 0214 (a force asks leave to go after
/// a find, or goes, which changes games with a carrier force and a contact
/// within two nights' steaming) and 28 from decisions 0211 and 0212 (a
/// strike spread over the ships of its kind and fire among a carrier's
/// aircraft, which change every game in which raids attack a force or a
/// carrier is bombed, and a ship's field a game saved under the old rules
/// could not hold) and 29 from decision 0218 (submarines: a boat as a unit
/// of her own kind with her state, which a game saved under the old rules
/// could not hold) and 30 from decision 0218, stage 3 (the patrol order, a hold
/// on a boat and a forces limit naming boats, which a game saved under the
/// old rules could not hold) and 31 from decision 0222 (a forces limit binds
/// only the kinds it names or lists in `none_of`, a forces limit and an area
/// cleared, which change games whose limits name some kinds and not others,
/// and a limit's field a game saved under the old rules could not hold) and
/// 32 from decision 0223, stages 0 and A (a side's submarine doctrine, which
/// a game saved under the old rules could not hold, and the pattern of a
/// patrol, which changes the stations of a side given one) and 33 from
/// decision 0220 (the order to destroy a force, what it keeps and its three
/// notices, which a game saved under the old rules could not hold) and 34 from
/// decision 0229 (a "keep supplied" is taken under "control an island", with its
/// own plan and a floor of 0 days, which a command the old rules refused and
/// which changes games with an island and a base of the side beside a place
/// with men). A
/// pull request
/// raises it by one
/// when it changes a rule in a way that changes what an existing game would do; a
/// change that leaves every existing game as it was leaves it alone.
pub const RULES_VERSION: u32 = 34;

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
    /// A part of a day was called out of its turn (decision 0156): a day
    /// opened while one is open, a pulse resolved with no day open or after
    /// the afternoon, or a day closed before its afternoon is resolved.
    /// Nothing of the world changed. `in_hand` is the day that was open
    /// when the call was made, if any.
    DayPartOutOfTurn {
        part: DayPart,
        in_hand: Option<DayInHand>,
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
            SimError::DayPartOutOfTurn { part, in_hand } => {
                let asked = match part {
                    DayPart::Open => "open a day",
                    DayPart::Pulse => "resolve a pulse",
                    DayPart::Close => "close a day",
                    DayPart::Orders => {
                        "hand in orders inside a day, which is done only at 0600 or 1200, \
                         with the morning or the afternoon pulse next"
                    }
                };
                match in_hand {
                    None => write!(f, "cannot {asked}: no day is open"),
                    Some(DayInHand {
                        day,
                        next_pulse: Some(pulse),
                        ..
                    }) => write!(
                        f,
                        "cannot {asked}: day {day} is open and its {} pulse is next",
                        pulse.name()
                    ),
                    Some(DayInHand {
                        day,
                        next_pulse: None,
                        ..
                    }) => write!(
                        f,
                        "cannot {asked}: day {day} is open and its afternoon is resolved"
                    ),
                }
            }
        }
    }
}

impl std::error::Error for SimError {}

/// The three parts a day is resolved in (decision 0156): its opening, each
/// of its pulses and its close. Named by [`SimError::DayPartOutOfTurn`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DayPart {
    /// [`World::open_day`].
    Open,
    /// [`World::resolve_next_pulse`].
    Pulse,
    /// [`World::close_day`].
    Close,
    /// [`World::submit_orders_inside_day`]: orders handed in while a day is
    /// open, only with its morning or afternoon pulse next.
    Orders,
}

/// The day opened and not yet closed (decision 0156): which day it is, the
/// pulse to be resolved next (`None` once the afternoon is resolved and the
/// day is to be closed) and where in the event log its events begin, for
/// its [`DaySummary`]. Held only between [`World::open_day`] and
/// [`World::close_day`], so never at a day boundary.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DayInHand {
    pub day: u32,
    pub next_pulse: Option<Pulse>,
    pub first_event: usize,
}

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
    /// The commands of each side and their pictures, while command levels
    /// are on (decision 0134); `None` with them off. Hashed after the
    /// command states.
    pub(crate) chain: Option<BTreeMap<Side, crate::pictures::SideCommands>>,
    /// The reports passed to a command that reach it in a later pulse
    /// (decision 0135, 12). Empty with command levels off or no report
    /// figures. Hashed after the commands.
    pub(crate) reports_in_flight: Vec<crate::passing::ReportInFlight>,
    /// What each side's area command holds of the commands under it, as last
    /// reported, while reports travel (decision 0136); `None` with command
    /// levels off or no report figures. Hashed after the reports in flight.
    pub(crate) held:
        Option<BTreeMap<Side, BTreeMap<crate::pictures::CommandRef, crate::held::HeldState>>>,
    /// State reports that reach the area command in a later pulse (decision
    /// 0136, 2). Hashed after `held`.
    pub(crate) state_reports_in_flight: Vec<crate::held::StateReportInFlight>,
    /// The task forces on radio silence (decision 0136, 15), set by a
    /// scenario or an order. Hashed.
    pub(crate) radio_silent: BTreeSet<TaskForceId>,
    /// A silent force's own reports it has not sent (decision 0136, 17), in
    /// the order they were made. Hashed.
    pub(crate) unsent_reports: BTreeMap<TaskForceId, Vec<crate::passing::UnsentReport>>,
    /// The silent forces that broke silence in the pulse in hand (decision
    /// 0136, 17). Cleared when the pulse's state reports are made, which is
    /// the end of the pulse (the afternoon's, the end of the day), so empty
    /// at every day boundary and not hashed.
    pub(crate) broke_silence: BTreeSet<TaskForceId>,
    /// The commands of task forces that ceased to be in the pulse in hand,
    /// kept until the pulse's state reports are made (decision 0136, 2); for
    /// the afternoon, through the day's end phases. Empty at every day
    /// boundary, so not hashed.
    pub(crate) gone_commands: Vec<(Side, crate::pictures::CommandRef)>,
    /// The pulse being resolved, while a day is: the moment a command formed
    /// in it is held as of (decision 0136, 5). `None` outside the day's
    /// pulses and end phases, so not hashed.
    pub(crate) pulse_in_hand: Option<Pulse>,
    /// The day opened and not yet closed, while a day is resolved in parts
    /// (decision 0156). `None` at every day boundary: set by
    /// [`World::open_day`] and cleared by [`World::close_day`] before the
    /// hash is taken, so not hashed.
    pub(crate) day_in_hand: Option<DayInHand>,
    /// Every order received, accepted or not, by ID.
    pub(crate) orders: BTreeMap<OrderId, Order>,
    pub(crate) order_ids: IdAllocator<OrderId>,
    /// Hands out the numbers of task forces and air groups made during play,
    /// from each side's own series (decisions 0016 and 0086).
    pub(crate) made_ids: MadeIds,
    /// Orders submitted for the coming day, not yet taken in.
    pub(crate) pending: BTreeMap<Side, Vec<OrderRequest>>,
    /// Orders handed in while a day is open, at 0600 or 1200, not yet taken
    /// in (decision 0156): taken in at the start of the next pulse, so empty
    /// at every day boundary and not hashed, as `pending` is not.
    pub(crate) pending_inside_day: BTreeMap<Side, Vec<OrderRequest>>,
    /// Each side's command state (decision 0107): intents, limits, leaves,
    /// plans and commitments. Hashed after the beliefs.
    pub(crate) command: BTreeMap<Side, CommandState>,
    /// The commander's commands handed in for the coming day, not yet taken
    /// in: taken at the first step of the day.
    pub(crate) pending_commands: BTreeMap<Side, Vec<CommandKind>>,
    /// What a staff is to hand in for the coming day without being asked:
    /// what a replay gives back. Taken in at the step a staff is asked.
    pub(crate) pending_staff: BTreeMap<Side, StaffDecisions>,
    /// What each side's staff handed in on the last day resolved, asked or
    /// given back, for a runner to save. Not state.
    pub(crate) staff_handed_in: BTreeMap<Side, StaffDecisions>,
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
    /// The damage of each base whose airfield is damaged, at the start of the
    /// pulse (decision 0103): its groups fly the rest of their ready
    /// aircraft. Worked out with `grounded_bases`, so not hashed.
    pub(crate) field_damage: BTreeMap<BaseId, Fixed>,
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
    /// The number of days the game has (decision 0194, 1 and 12), where the
    /// scenario or the session gives it. Fixed configuration, like the
    /// objectives and the reference data: set once where the game is made,
    /// the same for both sides, and never changed by play, so it is not
    /// state and is not hashed. `None` for a world given no length. It is
    /// not the objectives' horizon (decision 0088), which is the days the
    /// result is judged over.
    pub(crate) game_days: Option<u32>,
    /// The side a dry run makes up, taken as supplied in full in its fights
    /// and charged nothing for them (decision 0209, 12). A setting of the
    /// world the dry run builds, like the game's days: set once there and
    /// never changed by play, so it is not state and is not hashed. `None`
    /// for a live game.
    pub(crate) supplied_in_full: Option<Side>,
    /// The trace of the choices made, while it is on (decision 0100, 7).
    /// Not state: how to record, never what happens, like `config`. A cell,
    /// because the choices are made by functions that only read the world.
    pub(crate) choice_trace: TraceCell,
    /// The domain of every draw, for a world built from a picture on a
    /// stream of its own (decision 0170, 12); `None` for a live game and for
    /// the umpire's rebuild on the live stream. Hashed last, and only where
    /// there is one, so a live game's hash is what it was.
    pub(crate) domain: Option<Domain>,
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
        if let Some(problem) = ref_data
            .ground
            .as_ref()
            .and_then(|figures| figures.problem())
        {
            return Err(SimError::InvalidSetup(problem));
        }
        // The submarines (decision 0218): the data is whole, and each boat's
        // class, hexes and home base are in it and on the map.
        if let Some(data) = ref_data.submarines.as_ref() {
            data.validate().map_err(SimError::InvalidSetup)?;
        }
        if !truth.submarines.is_empty() && ref_data.submarines.is_none() {
            return Err(SimError::InvalidSetup(
                "there are submarines but no submarine data".to_string(),
            ));
        }
        // A boat's claim and the report of her to the side fired on are told
        // through the search figures' error and confusion of kinds.
        if !truth.submarines.is_empty() && ref_data.search.is_none() {
            return Err(SimError::InvalidSetup(
                "there are submarines but no search figures".to_string(),
            ));
        }
        for (key, boat) in &truth.submarines {
            let known = ref_data
                .submarines
                .as_ref()
                .is_some_and(|data| data.classes.contains_key(&boat.class));
            if !known {
                return Err(SimError::InvalidSetup(format!(
                    "{} has unknown submarine class {}",
                    boat.id, boat.class
                )));
            }
            if boat.id != *key || boat.id.raw() >= MADE_IDS_FLOOR {
                return Err(SimError::InvalidSetup(format!(
                    "{} has a number of 1,000,000 or more, which is for units made in play, \
                     or is not kept under it",
                    boat.id
                )));
            }
            for (what, hex) in [("hex", boat.hex), ("station", boat.station)] {
                if !map.is_navigable(hex) {
                    return Err(SimError::InvalidSetup(format!(
                        "{}'s {what} {hex} is not a hex ships can enter",
                        boat.id
                    )));
                }
            }
            if !truth.bases.contains_key(&boat.home) {
                return Err(SimError::InvalidSetup(format!(
                    "{} has home base {}, which does not exist",
                    boat.id, boat.home
                )));
            }
            // Her home base is a hex ships can enter and a sea route reaches
            // it from her station, or she could never be taken off the map
            // there (decision 0218, rules 3 and 21; row 68 of the
            // differences).
            let base = &truth.bases[&boat.home];
            if !map.is_navigable(base.hex) {
                return Err(SimError::InvalidSetup(format!(
                    "{}'s home base {} is at {}, which is not a hex ships can enter",
                    boat.id, boat.home, base.hex
                )));
            }
            if map.sea_distance(boat.station, base.hex).is_none() {
                return Err(SimError::InvalidSetup(format!(
                    "no sea route reaches {}'s home base {} at {} from her station {}",
                    boat.id, boat.home, base.hex, boat.station
                )));
            }
        }
        // What is held apart until its day is checked as what is on the map
        // is (decision 0195, 12): it enters the same world.
        if (!truth.land_units.is_empty() || truth.held_land_units().next().is_some())
            && ref_data.ground.is_none()
        {
            return Err(SimError::InvalidSetup(
                "there are land units but no ground figures".to_string(),
            ));
        }
        for group in truth.air_groups.values().chain(truth.held_air_groups()) {
            if !ref_data.aircraft_types.contains_key(&group.aircraft_type) {
                return Err(SimError::InvalidSetup(format!(
                    "{} has unknown aircraft type {}",
                    group.id, group.aircraft_type
                )));
            }
        }
        if (!truth.air_groups.is_empty() || truth.held_air_groups().next().is_some())
            && ref_data.search.is_none()
        {
            return Err(SimError::InvalidSetup(
                "there are air groups but no search figures".to_string(),
            ));
        }
        let reports_ashore = ref_data
            .ground
            .is_some_and(|ground| ground.report_error.is_some());
        if reports_ashore && ref_data.search.is_none() {
            // A ground report is written into belief with the search figures
            // (decision 0092, 12).
            return Err(SimError::InvalidSetup(
                "there are ground report figures but no search figures".to_string(),
            ));
        }
        if ref_data.surface_combat.is_some() && ref_data.search.is_none() {
            // A fight reports what it sighted through the search figures.
            return Err(SimError::InvalidSetup(
                "there are surface combat figures but no search figures".to_string(),
            ));
        }
        for ship in truth.ships.values().chain(truth.held_ships()) {
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
            for ship in truth.ships.values().chain(truth.held_ships()) {
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
            chain: None,
            reports_in_flight: Vec::new(),
            held: None,
            state_reports_in_flight: Vec::new(),
            radio_silent: BTreeSet::new(),
            unsent_reports: BTreeMap::new(),
            broke_silence: BTreeSet::new(),
            gone_commands: Vec::new(),
            pulse_in_hand: None,
            day_in_hand: None,
            orders: BTreeMap::new(),
            order_ids: IdAllocator::new(),
            made_ids: MadeIds::new(),
            pending: BTreeMap::new(),
            pending_inside_day: BTreeMap::new(),
            command: Side::ALL
                .into_iter()
                .map(|side| (side, CommandState::new()))
                .collect(),
            pending_commands: BTreeMap::new(),
            pending_staff: BTreeMap::new(),
            staff_handed_in: BTreeMap::new(),
            events: EventLog::default(),
            day_hashes: Vec::new(),
            strikes_in_flight: Vec::new(),
            sorties: Vec::new(),
            grounded_bases: BTreeSet::new(),
            field_damage: BTreeMap::new(),
            deck_losses: Vec::new(),
            objectives: None,
            game_days: None,
            supplied_in_full: None,
            choice_trace: TraceCell::default(),
            domain: None,
        })
    }

    /// Gives the game its number of days (decision 0194, 1 and 12): the
    /// scenario's, or the session's. Configuration, as the objectives are:
    /// it changes no state, no event and no hash, and both sides' views
    /// give it.
    pub fn set_game_days(&mut self, days: u32) {
        self.game_days = Some(days);
    }

    /// Gives `side` the submarine doctrine its scenario states (decision
    /// 0223, stage 0): set when the game is made, before its first day, and
    /// never changed after. State of the side's own, hashed with its command
    /// state.
    pub fn set_submarine_doctrine(
        &mut self,
        side: Side,
        doctrine: crate::command::SubmarineDoctrine,
    ) -> Result<(), SimError> {
        if self.truth.day != 0 || self.day_in_hand.is_some() {
            return Err(SimError::InvalidSetup(
                "a side's submarine doctrine is set before the first day".to_string(),
            ));
        }
        // Every side has a command state from the making of the world.
        if let Some(state) = self.command.get_mut(&side) {
            state.submarine_doctrine = doctrine;
        }
        Ok(())
    }

    /// States that `side`'s staff chooses its sources of supply (decision
    /// 0228, A): set before the first day and never changed after. State of
    /// the side's own, hashed with its command state.
    pub fn set_staff_chooses_sources(&mut self, side: Side) -> Result<(), SimError> {
        if self.truth.day != 0 || self.day_in_hand.is_some() {
            return Err(SimError::InvalidSetup(
                "a side's staff chooses its sources of supply from its scenario, before the first day"
                    .to_string(),
            ));
        }
        if let Some(state) = self.command.get_mut(&side) {
            state.staff_chooses_sources = true;
        }
        Ok(())
    }

    /// The number of days the game has (decision 0194), `None` where it was
    /// given none.
    pub fn game_days(&self) -> Option<u32> {
        self.game_days
    }

    /// Takes `side` as supplied in full in its fights ashore, and charges it
    /// nothing for them (decision 0209, 12): the setting a dry run gives the
    /// side it makes up, since a side does not know the enemy's stocks.
    /// Changes no state, no event and no hash; it acts only where the
    /// figures of decision 0209 are given.
    pub fn set_supplied_in_full(&mut self, side: Side) {
        self.supplied_in_full = Some(side);
    }

    /// The side taken as supplied in full (decision 0209, 12), `None` for a
    /// live game.
    pub fn supplied_in_full(&self) -> Option<Side> {
        self.supplied_in_full
    }

    /// Gives the game its missions and objectives (decision 0054). Fails if
    /// an objective names a base the game does not have, or if the horizon
    /// is 0 (decision 0088). Changes no state and no hash.
    pub fn set_objectives(&mut self, mut objectives: Objectives) -> Result<(), SimError> {
        if objectives.horizon_days == 0 {
            return Err(SimError::InvalidSetup(
                "the objectives' horizon_days must be at least 1".to_string(),
            ));
        }
        // The hex of each base named, for a side that does not have the
        // base on its map (decision 0186, 17). Bases do not move.
        objectives.base_hexes.clear();
        for base in objectives.bases_named() {
            let Some(found) = self.truth.bases.get(&base) else {
                return Err(SimError::InvalidSetup(format!(
                    "an objective names unknown {base}"
                )));
            };
            objectives.base_hexes.insert(base, found.hex);
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

    /// Turns the trace of choices on or off (decision 0100, 7). Turning it
    /// on starts an empty trace; turning it off drops it. It changes nothing
    /// that happens and no hash.
    pub fn trace_choices(&mut self, on: bool) {
        *self.choice_trace.get_mut() = on.then(Trace::default);
    }

    /// The choices traced since the trace was turned on or last taken, in
    /// the order they were made. Empty when the trace is off.
    pub fn take_choice_trace(&mut self) -> Vec<TracedChoice> {
        match self.choice_trace.get_mut() {
            Some(trace) => std::mem::take(&mut trace.entries),
            None => Vec::new(),
        }
    }

    /// Records a choice the engine made, if the trace is on. The question
    /// and answer are built only then.
    pub(crate) fn trace_choice(&self, side: Side, choice: impl FnOnce() -> (Question, Answer)) {
        let mut cell = self.choice_trace.lock();
        if let Some(trace) = cell.as_mut() {
            let (question, answer) = choice();
            if let Some(moment) = trace.moment {
                trace.entries.push(TracedChoice {
                    moment,
                    side,
                    question,
                    answer,
                });
            }
        }
    }

    /// Marks the moment of the day the resolution has reached, for the
    /// trace.
    pub(crate) fn pass_moment(&self, moment: Moment) {
        if let Some(trace) = self.choice_trace.lock().as_mut() {
            trace.moment = Some(moment);
        }
    }

    pub fn set_config(&mut self, config: SimConfig) {
        self.config = config;
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// The domain of the world's draws (decision 0170, 12): `None` for a
    /// live game.
    pub fn domain(&self) -> Option<Domain> {
        self.domain
    }

    /// The last day resolved. 0 before the first day.
    pub fn day(&self) -> u32 {
        self.truth.day
    }

    /// The day opened by [`World::open_day`] and not yet closed: which day,
    /// and which pulse comes next. `None` between days (decision 0156).
    pub fn day_in_hand(&self) -> Option<DayInHand> {
        self.day_in_hand
    }

    /// Queues orders from one side for the coming day. They take effect, or
    /// are rejected, at the start of the next [`World::resolve_day`].
    pub fn submit_orders(&mut self, side: Side, requests: Vec<OrderRequest>) {
        self.pending.entry(side).or_default().extend(requests);
    }

    /// Hands in orders from one side while a day is open, at 0600 or 1200
    /// (decision 0156): with the day's night or morning resolved and its
    /// morning or afternoon pulse next. They wait, and are taken in at the
    /// start of that pulse, each checked then against the state as it
    /// stands, with the pulse on its events. Only seven kinds may be given
    /// inside a day ([`OrderKind::may_be_given_inside_a_day`]); any other is
    /// refused there as given at dusk ([`RejectReason::GivenAtDusk`]).
    /// Returns the pulse they will be taken in at.
    ///
    /// Fails with [`SimError::DayPartOutOfTurn`], changing nothing, if no
    /// day is open, if its night is next or if its afternoon is resolved.
    /// Whether a scenario allows orders inside a day is the scenario's to
    /// say: the engine takes them wherever it is asked to.
    ///
    /// [`OrderKind::may_be_given_inside_a_day`]: crate::orders::OrderKind::may_be_given_inside_a_day
    /// [`RejectReason::GivenAtDusk`]: crate::orders::RejectReason::GivenAtDusk
    pub fn submit_orders_inside_day(
        &mut self,
        side: Side,
        requests: Vec<OrderRequest>,
    ) -> Result<Pulse, SimError> {
        match self.day_in_hand {
            Some(DayInHand {
                next_pulse: Some(pulse @ (Pulse::Morning | Pulse::Afternoon)),
                ..
            }) => {
                self.pending_inside_day
                    .entry(side)
                    .or_default()
                    .extend(requests);
                Ok(pulse)
            }
            in_hand => Err(SimError::DayPartOutOfTurn {
                part: DayPart::Orders,
                in_hand,
            }),
        }
    }

    /// The orders one side has handed in inside the open day and not yet
    /// taken in, in the order handed in (decision 0156). Empty between days.
    pub fn orders_waiting_inside_day(&self, side: Side) -> &[OrderRequest] {
        self.pending_inside_day
            .get(&side)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    /// Queues the commander's commands from one side for the coming day
    /// (decision 0107). They are taken in, or refused, at the first step of
    /// the next day, in the order handed in.
    pub fn submit_commands(&mut self, side: Side, commands: Vec<CommandKind>) {
        self.pending_commands
            .entry(side)
            .or_default()
            .extend(commands);
    }

    /// Queues what one side's staff hands in for the coming day, without a
    /// staff being asked: what a replay gives back (decision 0107, 9). It is
    /// taken in at the step a staff is asked, as if the staff had given it.
    pub fn submit_staff(&mut self, side: Side, decisions: StaffDecisions) {
        self.pending_staff
            .entry(side)
            .or_default()
            .append(decisions);
    }

    /// What each side's staff handed in on the last day resolved, asked or
    /// given back, by side. A side with no staff that day is not in it. For
    /// the runner that saves the game: it is not state.
    pub fn staff_handed_in(&self) -> &BTreeMap<Side, StaffDecisions> {
        &self.staff_handed_in
    }

    /// One side's command state, for the umpire, the runner and hashing. A
    /// side reads its own through [`SideView::command_state`].
    pub fn command_state(&self, side: Side) -> &CommandState {
        self.command
            .get(&side)
            .expect("both sides have a command state")
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
            self.chain.as_ref().map(|chain| &chain[&side]),
            self.held.as_ref().map(|held| &held[&side]),
            &self.radio_silent,
            &self.map,
            &self.ref_data,
            &self.events,
            &self.orders,
            self.objectives.as_ref(),
            self.game_days,
            self.command_state(side),
            TaskForceId::new(self.made_ids.next_task_force(side)),
            AirGroupId::new(self.made_ids.next_air_group(side)),
            PulseScratch {
                grounded_bases: &self.grounded_bases,
                field_damage: &self.field_damage,
                strikes_in_flight: &self.strikes_in_flight,
                sorties: &self.sorties,
                deck_losses: &self.deck_losses,
            },
            self.day_in_hand,
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
            domain: self.domain,
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
            domain: self.domain,
        }
        .stream()
    }
}
