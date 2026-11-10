//! The event log.
//!
//! Every state change is written here as an event, with references to the
//! orders and earlier events that caused it. The log can answer "what
//! happened to this ship today" and "why" without re-running the day.

use core::fmt;

use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{BaseId, EntityRef, EventId, OrderId, ShipId, TaskForceId};
use crate::orders::RejectReason;
use crate::time::{ClockTime, Pulse};

/// Something that led to an event.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Cause {
    Order(OrderId),
    Event(EventId),
}

impl fmt::Display for Cause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cause::Order(id) => write!(f, "{id}"),
            Cause::Event(id) => write!(f, "#{}", id.raw()),
        }
    }
}

/// One input to a computed quantity.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Factor {
    pub name: &'static str,
    pub value: Fixed,
}

/// How a computed quantity came about: its inputs and its result.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Breakdown {
    pub factors: Vec<Factor>,
    pub result: Fixed,
}

/// What happened.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum EventKind {
    OrderAccepted {
        order: OrderId,
        target: EntityRef,
    },
    OrderRejected {
        order: OrderId,
        target: EntityRef,
        reason: RejectReason,
    },
    /// A task force moved during a pulse.
    TaskForceSteamed {
        task_force: TaskForceId,
        from: Hex,
        to: Hex,
        distance_nm: Fixed,
        speed_kn: Fixed,
        /// Each hex entered, in order.
        hexes_entered: Vec<Hex>,
    },
    FuelBurned {
        ship: ShipId,
        task_force: TaskForceId,
        burned_t: Fixed,
        remaining_t: Fixed,
    },
    /// A task force reached the last waypoint of its route.
    TaskForceArrived {
        task_force: TaskForceId,
        hex: Hex,
        /// Elapsed hours since the scenario began.
        elapsed_hours: Fixed,
    },
    /// A task force stopped because a ship ran dry.
    TaskForceOutOfFuel {
        task_force: TaskForceId,
        hex: Hex,
    },
    /// A ship took supply aboard from a base.
    CargoLoaded {
        ship: ShipId,
        task_force: TaskForceId,
        base: BaseId,
        loaded_t: Fixed,
        /// Tons aboard the ship after loading.
        aboard_t: Fixed,
        /// The base's stock after loading.
        base_stock_t: Fixed,
    },
    /// A ship put supply ashore into a base's stock.
    CargoUnloaded {
        ship: ShipId,
        task_force: TaskForceId,
        base: BaseId,
        unloaded_t: Fixed,
        /// Tons aboard the ship after unloading.
        aboard_t: Fixed,
        /// The base's stock after unloading.
        base_stock_t: Fixed,
    },
    /// Every ship of a task force is full, or the base has no stock left.
    LoadingComplete {
        task_force: TaskForceId,
        base: BaseId,
        /// Elapsed hours since the scenario began.
        elapsed_hours: Fixed,
    },
    /// Every ship of a task force has put all its supply ashore.
    UnloadingComplete {
        task_force: TaskForceId,
        base: BaseId,
        /// Elapsed hours since the scenario began.
        elapsed_hours: Fixed,
    },
}

impl EventKind {
    /// The entities this event is about.
    pub fn entities(&self) -> Vec<EntityRef> {
        match self {
            EventKind::OrderAccepted { target, .. } | EventKind::OrderRejected { target, .. } => {
                vec![*target]
            }
            EventKind::TaskForceSteamed { task_force, .. }
            | EventKind::TaskForceArrived { task_force, .. }
            | EventKind::TaskForceOutOfFuel { task_force, .. } => {
                vec![EntityRef::TaskForce(*task_force)]
            }
            EventKind::FuelBurned {
                ship, task_force, ..
            } => vec![EntityRef::Ship(*ship), EntityRef::TaskForce(*task_force)],
            EventKind::CargoLoaded {
                ship,
                task_force,
                base,
                ..
            }
            | EventKind::CargoUnloaded {
                ship,
                task_force,
                base,
                ..
            } => vec![
                EntityRef::Ship(*ship),
                EntityRef::TaskForce(*task_force),
                EntityRef::Base(*base),
            ],
            EventKind::LoadingComplete {
                task_force, base, ..
            }
            | EventKind::UnloadingComplete {
                task_force, base, ..
            } => vec![EntityRef::TaskForce(*task_force), EntityRef::Base(*base)],
        }
    }
}

impl fmt::Display for EventKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventKind::OrderAccepted { order, target } => {
                write!(f, "{order} accepted for {target}")
            }
            EventKind::OrderRejected {
                order,
                target,
                reason,
            } => write!(f, "{order} rejected for {target}: {reason}"),
            EventKind::TaskForceSteamed {
                task_force,
                from,
                to,
                distance_nm,
                speed_kn,
                ..
            } => write!(
                f,
                "{task_force} steamed {distance_nm} nm at {speed_kn} kn from {from} to {to}"
            ),
            EventKind::FuelBurned {
                ship,
                burned_t,
                remaining_t,
                ..
            } => write!(f, "{ship} burned {burned_t} t, {remaining_t} t remaining"),
            EventKind::TaskForceArrived {
                task_force,
                hex,
                elapsed_hours,
            } => write!(
                f,
                "{task_force} arrived at {hex} at {}",
                ClockTime::from_elapsed(*elapsed_hours)
            ),
            EventKind::TaskForceOutOfFuel { task_force, hex } => {
                write!(f, "{task_force} ran out of fuel at {hex}")
            }
            EventKind::CargoLoaded {
                ship,
                base,
                loaded_t,
                aboard_t,
                base_stock_t,
                ..
            } => write!(
                f,
                "{ship} loaded {loaded_t} t from {base}, {aboard_t} t aboard, {base_stock_t} t in stock"
            ),
            EventKind::CargoUnloaded {
                ship,
                base,
                unloaded_t,
                aboard_t,
                base_stock_t,
                ..
            } => write!(
                f,
                "{ship} unloaded {unloaded_t} t at {base}, {aboard_t} t aboard, {base_stock_t} t in stock"
            ),
            EventKind::LoadingComplete {
                task_force,
                base,
                elapsed_hours,
            } => write!(
                f,
                "{task_force} completed loading at {base} at {}",
                ClockTime::from_elapsed(*elapsed_hours)
            ),
            EventKind::UnloadingComplete {
                task_force,
                base,
                elapsed_hours,
            } => write!(
                f,
                "{task_force} completed unloading at {base} at {}",
                ClockTime::from_elapsed(*elapsed_hours)
            ),
        }
    }
}

/// One entry in the log.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Event {
    pub id: EventId,
    pub day: u32,
    /// `None` for orders intake and the end-of-day phases.
    pub pulse: Option<Pulse>,
    pub kind: EventKind,
    pub causes: Vec<Cause>,
    /// Present for computed quantities when breakdown recording is on.
    pub breakdown: Option<Breakdown>,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pulse = self.pulse.map_or("-", Pulse::name);
        write!(
            f,
            "#{} D{} {pulse:<9} {}",
            self.id.raw(),
            self.day,
            self.kind
        )?;
        if !self.causes.is_empty() {
            let causes: Vec<String> = self.causes.iter().map(Cause::to_string).collect();
            write!(f, "  <- {}", causes.join(", "))?;
        }
        Ok(())
    }
}

/// All events of a game, in the order they happened.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct EventLog {
    events: Vec<Event>,
}

impl EventLog {
    pub(crate) fn record(
        &mut self,
        day: u32,
        pulse: Option<Pulse>,
        kind: EventKind,
        causes: Vec<Cause>,
        breakdown: Option<Breakdown>,
    ) -> EventId {
        let raw = u32::try_from(self.events.len() + 1).expect("ran out of event IDs");
        let id = EventId::new(raw);
        self.events.push(Event {
            id,
            day,
            pulse,
            kind,
            causes,
            breakdown,
        });
        id
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Event> {
        self.events.iter()
    }

    /// The events from position `start` onward. A [`crate::DaySummary`]
    /// gives the position where its day begins.
    pub fn from_index(&self, start: usize) -> &[Event] {
        self.events.get(start..).unwrap_or(&[])
    }

    pub fn get(&self, id: EventId) -> Option<&Event> {
        let index = usize::try_from(id.raw()).ok()?.checked_sub(1)?;
        self.events.get(index)
    }

    /// Every event that involves the entity, in order.
    pub fn involving(&self, entity: EntityRef) -> impl Iterator<Item = &Event> {
        self.events
            .iter()
            .filter(move |event| event.kind.entities().contains(&entity))
    }

    /// Everything that led to an event, following causes back through earlier
    /// events until only orders remain. Nearest causes come first and nothing
    /// is listed twice.
    pub fn trace(&self, id: EventId) -> Vec<Cause> {
        let mut found: Vec<Cause> = Vec::new();
        let mut frontier = vec![id];
        while let Some(current) = frontier.pop() {
            let Some(event) = self.get(current) else {
                continue;
            };
            for cause in &event.causes {
                if found.contains(cause) {
                    continue;
                }
                found.push(*cause);
                if let Cause::Event(earlier) = cause {
                    frontier.push(*earlier);
                }
            }
        }
        found
    }
}
