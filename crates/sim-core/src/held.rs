//! What an area command holds of the commands under it (decision 0136, step
//! 3 of stage 2b).
//!
//! With command levels on and report figures loaded, a task force and a base
//! report their own state to the area command at the end of each pulse (item
//! 1): a copy of what the side's units of that command are at that moment. The
//! report arrives at once, a pulse late or never, by one draw (item 2). The
//! area command holds, for each command under it, the latest report that has
//! arrived and the moment it is of ([`HeldState`], [`AsOf`]); an older report
//! arriving after a newer one changes nothing (item 3). At the start every
//! command is held exactly (item 4); one formed in the game is held as it was
//! formed (item 5).
//!
//! What the area command reads of a force a level down is the truth with the
//! held states written over it ([`held_truth`]): one view for a command, as
//! decision 0134 has it for pictures. With no report figures, or command
//! levels off, nothing of this exists and a superior knows his forces exactly
//! (item 7).
//!
//! Reads: the truth at the moment of a report; the report figures; each
//! force's radio silence. Writes: the held states, the reports in flight, the
//! two events of a late or lost report. Draws one number for each report.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::events::{Arrival, Breakdown, EventKind, Factor};
use crate::fixed::Fixed;
use crate::ids::{AirGroupId, EntityRef, LandUnitId, ShipId};
use crate::orders::{Issuer, RejectReason};
use crate::passing::{arrival_of, next_pulse};
use crate::pictures::{self, CommandRef};
use crate::rng::{Draw, Subsystem};
use crate::state::{AirGroup, Base, LandUnit, Ship, ShipLocation, Side, TaskForce, TruthState};
use crate::time::Pulse;
use crate::world::World;

/// The moment a held state is of: the start of the game, or the end of a
/// pulse. Ordered: the start first, then by day and pulse.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AsOf {
    /// The start of the game, before day 1.
    Start,
    /// The end of the pulse of that day.
    Pulse { day: u32, pulse: Pulse },
}

impl AsOf {
    /// The moment as a day and pulse, or `None` for the start.
    pub fn day_and_pulse(self) -> Option<(u32, Pulse)> {
        match self {
            AsOf::Start => None,
            AsOf::Pulse { day, pulse } => Some((day, pulse)),
        }
    }
}

impl fmt::Display for AsOf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AsOf::Start => f.write_str("the start"),
            AsOf::Pulse { day, pulse } => write!(f, "the {} of day {day}", pulse.name()),
        }
    }
}

/// What the area command holds of one command under it (decision 0136, 1 and
/// 3): a copy of the command's own state as of `as_of`. A task force's is its
/// record, its ships and the air groups aboard; a base's is its record, the
/// air groups at it and the land units at it. Hashed.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HeldState {
    pub(crate) as_of: AsOf,
    /// A task force's record: where it is, where it is going.
    pub(crate) task_force: Option<TaskForce>,
    /// A base's record: its stocks, its airfield.
    pub(crate) base: Option<Base>,
    /// A force's ships: those whose place is the force or its yard.
    pub(crate) ships: BTreeMap<ShipId, Ship>,
    /// The air groups of the command (`pictures::command_of_group`).
    pub(crate) air_groups: BTreeMap<AirGroupId, AirGroup>,
    /// A base's land units (`pictures::command_of_land_unit`).
    pub(crate) land_units: BTreeMap<LandUnitId, LandUnit>,
}

impl HeldState {
    /// The moment the state is of.
    pub fn as_of(&self) -> AsOf {
        self.as_of
    }

    /// A task force's record as held, for a task force command.
    pub fn task_force(&self) -> Option<&TaskForce> {
        self.task_force.as_ref()
    }

    /// A base's record as held, for a base command.
    pub fn base(&self) -> Option<&Base> {
        self.base.as_ref()
    }

    /// The ships as held, in ship order.
    pub fn ships(&self) -> impl Iterator<Item = &Ship> {
        self.ships.values()
    }

    /// The air groups as held, in group order.
    pub fn air_groups(&self) -> impl Iterator<Item = &AirGroup> {
        self.air_groups.values()
    }

    /// The land units as held, in unit order.
    pub fn land_units(&self) -> impl Iterator<Item = &LandUnit> {
        self.land_units.values()
    }
}

/// A state report on its way to the area command, to arrive at the report
/// moment of `arrives` (decision 0136, 2). Hashed.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StateReportInFlight {
    pub(crate) side: Side,
    pub(crate) command: CommandRef,
    pub(crate) state: HeldState,
    /// The day and pulse at whose end it arrives.
    pub(crate) arrives: (u32, Pulse),
}

/// The state of `command` at this moment, as it reports it (decision 0136, 1):
/// the area command has none.
pub(crate) fn state_of(truth: &TruthState, command: CommandRef, as_of: AsOf) -> HeldState {
    let mut state = HeldState {
        as_of,
        task_force: None,
        base: None,
        ships: BTreeMap::new(),
        air_groups: BTreeMap::new(),
        land_units: BTreeMap::new(),
    };
    match command {
        CommandRef::Area => return state,
        CommandRef::TaskForce(force) => {
            state.task_force = truth.task_forces.get(&force).cloned();
            state.ships = truth
                .ships
                .iter()
                .filter(|(_, ship)| {
                    ship.location == ShipLocation::TaskForce(force)
                        || ship.location == ShipLocation::Yard(force)
                })
                .map(|(id, ship)| (*id, ship.clone()))
                .collect();
        }
        CommandRef::Base(base) => {
            state.base = truth.bases.get(&base).cloned();
            state.land_units = truth
                .land_units
                .keys()
                .filter(|unit| pictures::command_of_land_unit(truth, **unit) == Some(command))
                .map(|unit| (*unit, truth.land_units[unit].clone()))
                .collect();
        }
    }
    state.air_groups = truth
        .air_groups
        .keys()
        .filter(|group| pictures::command_of_group(truth, **group) == Some(command))
        .map(|group| (*group, truth.air_groups[group].clone()))
        .collect();
    state
}

/// The truth as an area command reads it (decision 0136, 9): the truth with
/// each held state written over it, in `(as_of, command)` order so that the
/// newest word wins for an entry two reports name. An entry of the side that
/// no held state names (a ship in port in no force, a land unit aboard ship,
/// a group lost) stays as the truth. The enemy's entries are untouched.
pub(crate) fn held_truth(truth: &TruthState, held: &BTreeMap<CommandRef, HeldState>) -> TruthState {
    // A submarine is no command and has no held state: her side's highest
    // command knows her exactly, so the boats are copied as they are
    // (decision 0218, 35).
    let mut overlay = truth.clone();
    let mut states: Vec<(AsOf, CommandRef, &HeldState)> = held
        .iter()
        .map(|(command, state)| (state.as_of, *command, state))
        .collect();
    states.sort_by_key(|(as_of, command, _)| (*as_of, *command));
    for (_, _, state) in states {
        if let Some(force) = &state.task_force {
            overlay.task_forces.insert(force.id, force.clone());
        }
        if let Some(base) = &state.base {
            overlay.bases.insert(base.id, base.clone());
        }
        for (id, ship) in &state.ships {
            overlay.ships.insert(*id, ship.clone());
        }
        for (id, group) in &state.air_groups {
            overlay.air_groups.insert(*id, group.clone());
        }
        for (id, unit) in &state.land_units {
            overlay.land_units.insert(*id, unit.clone());
        }
    }
    overlay
}

impl World {
    /// Whether the area commands hold their forces as last reported: the
    /// same as whether reports travel.
    pub fn holds_as_reported(&self) -> bool {
        self.held.is_some()
    }

    /// What `side`'s area command holds of each command under it, for the
    /// umpire, tests and hashing. `None` unless reports travel. A side reads
    /// it through [`SideView::known_to`](crate::view::SideView::known_to).
    pub fn held(&self, side: Side) -> Option<&BTreeMap<CommandRef, HeldState>> {
        self.held.as_ref().map(|held| &held[&side])
    }

    /// Puts a task force on radio silence at the start of a game (decision
    /// 0136, 15), for the loader. Fails if the game has begun or the force
    /// does not exist. It has an effect only where reports travel. A force
    /// held apart until its day (decision 0195, 11) keeps silence from the
    /// day it enters: its entry is marked, and it goes into the set then. Run
    /// again on a truth that holds the entries it changes nothing.
    pub fn keep_radio_silence_from_the_start(
        &mut self,
        force: crate::ids::TaskForceId,
    ) -> Result<(), crate::world::SimError> {
        if self.truth.day > 0 || !self.events.is_empty() {
            return Err(crate::world::SimError::InvalidSetup(
                "radio silence can only be set at the start before day 1 and before any event"
                    .to_string(),
            ));
        }
        if let Some(entry) = self
            .truth
            .entering
            .get_mut(&crate::ids::EntityRef::TaskForce(force))
        {
            entry.radio_silent = true;
            return Ok(());
        }
        if !self.truth.task_forces.contains_key(&force) {
            return Err(crate::world::SimError::InvalidSetup(format!(
                "radio silence is set for unknown {force}"
            )));
        }
        self.radio_silent.insert(force);
        Ok(())
    }

    /// Whether a task force is on radio silence, for the umpire and tests. A
    /// side reads it through [`SideView::radio_silence`](crate::view::SideView::radio_silence).
    pub fn radio_silence(&self, force: crate::ids::TaskForceId) -> bool {
        self.radio_silent.contains(&force)
    }

    /// Begins holding (decision 0136, 4), when command levels are turned on
    /// with report figures loaded and before day 1: every task force and base
    /// command of each side is held exactly, as of the start.
    pub(crate) fn begin_holding(&mut self) {
        let Some(chain) = self.chain.as_ref() else {
            return;
        };
        if self.ref_data.reports.is_none() {
            return;
        }
        let held = Side::ALL
            .into_iter()
            .map(|side| {
                let states = chain[&side]
                    .keys()
                    .filter(|command| **command != CommandRef::Area)
                    .map(|command| (*command, state_of(&self.truth, *command, AsOf::Start)))
                    .collect();
                (side, states)
            })
            .collect();
        self.held = Some(held);
    }

    /// The moment in hand: the pulse being resolved, else the end of the last
    /// day (`Start` before day 1).
    fn moment(&self) -> AsOf {
        match self.pulse_in_hand {
            Some(pulse) => AsOf::Pulse {
                day: self.truth.day + 1,
                pulse,
            },
            None if self.truth.day == 0 => AsOf::Start,
            None => AsOf::Pulse {
                day: self.truth.day,
                pulse: Pulse::Afternoon,
            },
        }
    }

    /// A command formed in the game, or a base that changed side to `side`,
    /// is held at once as it is now (decision 0136, 5). A no-op unless
    /// reports travel.
    pub(crate) fn hold_now(&mut self, side: Side, command: CommandRef) {
        if self.held.is_none() {
            return;
        }
        let state = state_of(&self.truth, command, self.moment());
        self.held
            .as_mut()
            .expect("checked above")
            .get_mut(&side)
            .expect("both sides")
            .insert(command, state);
    }

    /// A base is no longer `side`'s: its area command drops what it held of
    /// it, and any state report of it still on its way (decision 0136).
    pub(crate) fn drop_held(&mut self, side: Side, command: CommandRef) {
        if let Some(held) = self.held.as_mut() {
            held.get_mut(&side).expect("both sides").remove(&command);
        }
        self.state_reports_in_flight
            .retain(|flight| !(flight.side == side && flight.command == command));
    }

    /// Makes the state reports of the moment `(day, pulse)` (decision 0136, 1
    /// and 2): first those a pulse late that are due now are applied, then
    /// every task force and base command of each side, and each task force
    /// that ceased to be in the pulse, reports its own state as of now, each
    /// drawn once. One that arrives on time is held at once and is no event;
    /// a late one is kept in flight and a lost one is dropped, each an event.
    /// A silent force that has not broken silence this pulse reports nothing
    /// (item 16): no draw, no event. The forces that broke silence this
    /// pulse are forgotten at the end.
    ///
    /// Run at the end of the night and the morning, after the forces that
    /// ceased to be are ended, and for the afternoon at the end of the day,
    /// after the day's end phases: the staff is asked next on the state as the
    /// day ended.
    pub(crate) fn make_state_reports(&mut self, day: u32, pulse: Pulse) {
        let (Some(figures), true) = (self.ref_data.reports, self.held.is_some()) else {
            return;
        };
        let moment = (day, pulse);
        // A report a pulse late reaches the area command first.
        let (due, later): (Vec<StateReportInFlight>, Vec<StateReportInFlight>) =
            std::mem::take(&mut self.state_reports_in_flight)
                .into_iter()
                .partition(|flight| flight.arrives == moment);
        self.state_reports_in_flight = later;
        for flight in due {
            self.hold_reported(flight.side, flight.command, flight.state);
        }
        // Every reporting command, in side and command order, with the forces
        // that ceased to be in this pulse.
        let gone = std::mem::take(&mut self.gone_commands);
        let mut reporters: Vec<(Side, CommandRef)> = Vec::new();
        for side in Side::ALL {
            let mut commands: BTreeSet<CommandRef> = self.chain.as_ref().expect("reports travel")
                [&side]
                .keys()
                .copied()
                .filter(|command| *command != CommandRef::Area)
                .collect();
            commands.extend(
                gone.iter()
                    .filter(|(owner, _)| *owner == side)
                    .map(|(_, command)| *command),
            );
            reporters.extend(commands.into_iter().map(|command| (side, command)));
        }
        for (side, command) in reporters {
            if let CommandRef::TaskForce(force) = command {
                if self.radio_silent.contains(&force) && !self.broke_silence.contains(&force) {
                    continue;
                }
            }
            let as_of = AsOf::Pulse { day, pulse };
            let state = state_of(&self.truth, command, as_of);
            let entity = match command {
                CommandRef::TaskForce(force) => EntityRef::TaskForce(force),
                CommandRef::Base(base) => EntityRef::Base(base),
                CommandRef::Area => continue,
            };
            let mut stream = self.rng(
                day,
                Some(pulse),
                Subsystem::Intelligence,
                Draw::StateReport,
                entity,
            );
            // One number is always taken, whatever the chances.
            let roll = i64::try_from(stream.below(Fixed::SCALE.unsigned_abs()))
                .expect("a roll fits in i64");
            let arrival = arrival_of(figures.up, roll);
            if arrival == Arrival::OnTime {
                self.hold_reported(side, command, state);
                continue;
            }
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "on_time_chance",
                        value: figures.up.on_time,
                    },
                    Factor {
                        name: "late_chance",
                        value: figures.up.late,
                    },
                    // Named "draw", as a raw die is everywhere: a side is
                    // never given it.
                    Factor {
                        name: "draw",
                        value: Fixed::from_raw(roll),
                    },
                ],
                result: if arrival == Arrival::Late {
                    Fixed::ONE
                } else {
                    Fixed::ZERO
                },
            });
            let kind = if arrival == Arrival::Late {
                EventKind::StateReportLate {
                    side,
                    command,
                    as_of,
                }
            } else {
                EventKind::StateReportLost {
                    side,
                    command,
                    as_of,
                }
            };
            self.events
                .record(day, Some(pulse), kind, Vec::new(), breakdown);
            if arrival == Arrival::Late {
                self.state_reports_in_flight.push(StateReportInFlight {
                    side,
                    command,
                    state,
                    arrives: next_pulse(day, pulse),
                });
            }
        }
        self.broke_silence.clear();
    }

    /// The area command of `side` holds `state` of `command`, unless it holds
    /// a newer one already (decision 0136, 3). A base that is no longer the
    /// side's is not held.
    pub(crate) fn hold_reported(&mut self, side: Side, command: CommandRef, state: HeldState) {
        if let CommandRef::Base(base) = command {
            if self.truth.bases.get(&base).map(|record| record.side) != Some(side) {
                return;
            }
        }
        let held = self
            .held
            .as_mut()
            .expect("reports travel")
            .get_mut(&side)
            .expect("both sides");
        match held.get(&command) {
            Some(newer) if newer.as_of > state.as_of => {}
            _ => {
                held.insert(command, state);
            }
        }
    }
}

impl World {
    /// Checks an order against the truth, as the engine always has, and, for
    /// an order from the area command, tells a refusal that rests on the
    /// state the area command did not hold from any other (decision 0136, 13
    /// and 14): where reports travel and the order is the commander's or a
    /// staff order whose intent is addressed to the area, a refusal the state
    /// the area command holds would not have given is
    /// [`RejectReason::NotAsHeld`], naming what it held and the moment that
    /// was of. Any other order is checked as before.
    pub(crate) fn validate_order(&self, order: &crate::orders::Order) -> Result<(), RejectReason> {
        // A unit before its day (decision 0140, 2) is refused first, from
        // anyone: a side knows its own days exactly, so it is no matter of
        // what the area command holds.
        // A unit of its own held apart until its day (decision 0195, 8 and
        // 15) is refused before that.
        if let Some(refused) = self.not_yet_entered(order.side, &order.kind) {
            return Err(refused);
        }
        if let Some(refused) = self.not_yet_available(order.side, &order.kind, order.day_issued) {
            return Err(refused);
        }
        let reason = match self.validate_order_on_truth(order) {
            Ok(()) => return Ok(()),
            Err(reason) => reason,
        };
        if !self.reports_travel() || !self.order_is_from_the_area(order) {
            return Err(reason);
        }
        let held = &self.held.as_ref().expect("reports travel")[&order.side];
        let mut as_held = self.clone();
        as_held.truth = held_truth(&self.truth, held);
        match as_held.validate_order_on_truth(order) {
            Ok(()) => Err(RejectReason::NotAsHeld {
                held: self.held_words(order.side, order.kind.target()),
                reason: Box::new(reason),
            }),
            Err(_) => Err(reason),
        }
    }

    /// Whether an order comes from the area command: the commander's own, or
    /// the staff's for an intent addressed to the area (decision 0136, 13).
    /// An order of the staff's that serves an intent addressed to a task
    /// force or a base, a scripted order of a scenario and a local
    /// commander's are not.
    fn order_is_from_the_area(&self, order: &crate::orders::Order) -> bool {
        match order.origin.issuer {
            Issuer::Player => true,
            Issuer::Staff => order
                .origin
                .intent
                .and_then(|intent| self.command[&order.side].intents.get(&intent))
                .is_some_and(|intent| intent.addressee == crate::command::Addressee::Area),
            Issuer::LocalCommander => false,
        }
    }

    /// What `side`'s area command held of `target`, in words: where it was,
    /// what it had, and the moment that was of.
    fn held_words(&self, side: Side, target: EntityRef) -> String {
        let held = &self.held.as_ref().expect("reports travel")[&side];
        let found = |command: CommandRef| held.get(&command);
        let nothing = || format!("nothing of {target}");
        match target {
            EntityRef::TaskForce(force) => match found(CommandRef::TaskForce(force)) {
                Some(state) => {
                    let (hex, afloat) = state.task_force().map_or((None, 0), |record| {
                        (
                            Some(record.position()),
                            state
                                .ships()
                                .filter(|ship| ship.location != ShipLocation::Sunk)
                                .count(),
                        )
                    });
                    let at = hex.map_or(String::new(), |hex| format!(" at {hex}"));
                    format!(
                        "{target}{at} with {afloat} ship{} afloat, as of {}",
                        if afloat == 1 { "" } else { "s" },
                        state.as_of()
                    )
                }
                None => nothing(),
            },
            EntityRef::Base(base) => match found(CommandRef::Base(base)) {
                Some(state) => {
                    let at = state
                        .base()
                        .map_or(String::new(), |record| format!(" at {}", record.hex));
                    format!("{target}{at}, as of {}", state.as_of())
                }
                None => nothing(),
            },
            EntityRef::AirGroup(group) => held
                .values()
                .filter(|state| state.air_groups.contains_key(&group))
                .max_by_key(|state| state.as_of)
                .map_or_else(nothing, |state| {
                    let record = &state.air_groups[&group];
                    format!(
                        "{target} with {} ready aircraft at {}, as of {}",
                        record.ready,
                        record.location,
                        state.as_of()
                    )
                }),
            EntityRef::LandUnit(unit) => held
                .values()
                .filter(|state| state.land_units.contains_key(&unit))
                .max_by_key(|state| state.as_of)
                .map_or_else(nothing, |state| {
                    let record = &state.land_units[&unit];
                    format!(
                        "{target} with {} men ashore, as of {}",
                        record.men_ashore,
                        state.as_of()
                    )
                }),
            _ => nothing(),
        }
    }
}
