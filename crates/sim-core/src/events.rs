//! The event log.
//!
//! Every state change is written here as an event, with references to the
//! orders and earlier events that caused it. The log can answer "what
//! happened to this ship today" and "why" without re-running the day.

use core::fmt;

use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{AirGroupId, BaseId, ContactId, EntityRef, EventId, OrderId, ShipId, TaskForceId};
use crate::orders::RejectReason;
use crate::refdata::ShipKind;
use crate::state::Side;
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

/// The weapon a salvo was fired with.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Weapon {
    Gun,
    Torpedo,
}

impl Weapon {
    /// The weapon's name as the event log writes it.
    pub const fn name(self) -> &'static str {
        match self {
            Weapon::Gun => "guns",
            Weapon::Torpedo => "torpedoes",
        }
    }
}

/// One of the three damage tracks of a ship.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum DamageTrack {
    Flotation,
    System,
    Fire,
}

impl DamageTrack {
    /// Every track, in the order gun hits are assigned to them.
    pub const ALL: [DamageTrack; 3] = [
        DamageTrack::Flotation,
        DamageTrack::System,
        DamageTrack::Fire,
    ];

    /// The track's name as the event log writes it.
    pub const fn name(self) -> &'static str {
        match self {
            DamageTrack::Flotation => "flotation",
            DamageTrack::System => "system",
            DamageTrack::Fire => "fire",
        }
    }
}

/// Why a surface engagement ended.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum EngagementEndReason {
    /// A force that avoids sighted the enemy first and left.
    SlippedAway,
    /// A force had no ship afloat.
    ForceSunk,
    /// A force broke off.
    BrokeOff,
    /// Neither force sighted the other in all the rounds.
    PassedUnseen,
    /// The last round was fought.
    MaxRounds,
}

impl EngagementEndReason {
    /// The reason as the event log writes it.
    pub const fn name(self) -> &'static str {
        match self {
            EngagementEndReason::SlippedAway => "slipped away",
            EngagementEndReason::ForceSunk => "a force was sunk",
            EngagementEndReason::BrokeOff => "a force broke off",
            EngagementEndReason::PassedUnseen => "passed unseen",
            EngagementEndReason::MaxRounds => "reached the last round",
        }
    }
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
    /// An air group flew its search arc from its base.
    SearchFlown {
        air_group: AirGroupId,
        base: BaseId,
        from_deg: u32,
        to_deg: u32,
        range_nm: Fixed,
        aircraft: u32,
    },
    /// One roll of an air group looking for one task force in its arc.
    SearchAttempt {
        air_group: AirGroupId,
        task_force: TaskForceId,
        distance_nm: Fixed,
        chance: Fixed,
        sighted: bool,
    },
    /// A sighting report reached the searching side's belief.
    ReportReceived {
        side: Side,
        contact: ContactId,
        hex: Hex,
        ship_count: u32,
        kind: ShipKind,
        confidence: Fixed,
        /// True if the report made a new contact, false if it refreshed one.
        created: bool,
    },
    /// A sighted task force noticed the aircraft that sighted it.
    SightingNoticed {
        task_force: TaskForceId,
        air_group: AirGroupId,
        /// Elapsed hours since the scenario began, at the start of the pulse.
        elapsed_hours: Fixed,
    },
    /// A contact went a pulse without a fresh report.
    ContactAged {
        side: Side,
        contact: ContactId,
        age_pulses: u32,
        confidence: Fixed,
    },
    /// A contact's confidence fell below the floor and it was forgotten.
    /// The age and confidence are what it would have had.
    ContactDropped {
        side: Side,
        contact: ContactId,
        age_pulses: u32,
        confidence: Fixed,
    },
    /// Two opposing task forces met in one hex and a fight began.
    EngagementBegun {
        a: TaskForceId,
        b: TaskForceId,
        hex: Hex,
    },
    /// One roll of a force looking for the enemy force.
    SightingRoll {
        round: u32,
        task_force: TaskForceId,
        enemy: TaskForceId,
        chance: Fixed,
        sighted: bool,
    },
    /// A force that has not sighted the enemy is fired on by an enemy that
    /// has.
    ForceSurprised {
        round: u32,
        task_force: TaskForceId,
        enemy: TaskForceId,
    },
    /// One ship fired one weapon at one ship.
    Salvo {
        round: u32,
        firer: ShipId,
        target: ShipId,
        weapon: Weapon,
        shots: Fixed,
        chance: Fixed,
        expected: Fixed,
        hits: u32,
    },
    /// The hits of one salvo on one damage track of the target.
    ShipDamaged {
        round: u32,
        ship: ShipId,
        track: DamageTrack,
        hits: u32,
        points: Fixed,
        value_after: Fixed,
    },
    /// A ship's flotation damage reached 100 and it left its task force.
    ShipSunk {
        round: u32,
        ship: ShipId,
        task_force: TaskForceId,
    },
    /// A force sighted the enemy or was fired on and was not alert.
    ForceBecameAlert {
        round: u32,
        task_force: TaskForceId,
    },
    /// A force broke off. `share` is the share of the ships it began the
    /// fight with that are sunk or crippled.
    ForceBrokeOff {
        round: u32,
        task_force: TaskForceId,
        share: Fixed,
    },
    /// A force that avoids sighted the enemy first and left unseen.
    ForceSlippedAway {
        round: u32,
        task_force: TaskForceId,
        enemy: TaskForceId,
    },
    /// Neither force sighted the other in the last round.
    ForcesPassedUnseen {
        round: u32,
        a: TaskForceId,
        b: TaskForceId,
    },
    /// A fight ended.
    EngagementEnded {
        round: u32,
        a: TaskForceId,
        b: TaskForceId,
        hex: Hex,
        reason: EngagementEndReason,
    },
    /// A battle claim reached a side's belief.
    ClaimReceived {
        side: Side,
        hex: Hex,
        claimed_sunk: u32,
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
            EventKind::SearchFlown {
                air_group, base, ..
            } => vec![EntityRef::AirGroup(*air_group), EntityRef::Base(*base)],
            EventKind::SearchAttempt {
                air_group,
                task_force,
                ..
            } => vec![
                EntityRef::AirGroup(*air_group),
                EntityRef::TaskForce(*task_force),
            ],
            EventKind::ReportReceived { contact, .. }
            | EventKind::ContactAged { contact, .. }
            | EventKind::ContactDropped { contact, .. } => vec![EntityRef::Contact(*contact)],
            EventKind::SightingNoticed {
                task_force,
                air_group,
                ..
            } => vec![
                EntityRef::TaskForce(*task_force),
                EntityRef::AirGroup(*air_group),
            ],
            EventKind::EngagementBegun { a, b, .. }
            | EventKind::ForcesPassedUnseen { a, b, .. }
            | EventKind::EngagementEnded { a, b, .. } => {
                vec![EntityRef::TaskForce(*a), EntityRef::TaskForce(*b)]
            }
            EventKind::SightingRoll {
                task_force, enemy, ..
            }
            | EventKind::ForceSurprised {
                task_force, enemy, ..
            }
            | EventKind::ForceSlippedAway {
                task_force, enemy, ..
            } => vec![
                EntityRef::TaskForce(*task_force),
                EntityRef::TaskForce(*enemy),
            ],
            EventKind::Salvo { firer, target, .. } => {
                vec![EntityRef::Ship(*firer), EntityRef::Ship(*target)]
            }
            EventKind::ShipDamaged { ship, .. } => vec![EntityRef::Ship(*ship)],
            EventKind::ShipSunk {
                ship, task_force, ..
            } => vec![EntityRef::Ship(*ship), EntityRef::TaskForce(*task_force)],
            EventKind::ForceBecameAlert { task_force, .. }
            | EventKind::ForceBrokeOff { task_force, .. } => {
                vec![EntityRef::TaskForce(*task_force)]
            }
            // A side is not an entity: the claim names no force or ship.
            EventKind::ClaimReceived { .. } => Vec::new(),
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
            EventKind::SearchFlown {
                air_group,
                base,
                from_deg,
                to_deg,
                range_nm,
                aircraft,
            } => write!(
                f,
                "{air_group} searched {from_deg:03}-{to_deg:03} out to {range_nm} nm from {base} with {aircraft} aircraft"
            ),
            EventKind::SearchAttempt {
                air_group,
                task_force,
                distance_nm,
                chance,
                sighted,
            } => write!(
                f,
                "{air_group} searched for {task_force} at {distance_nm} nm, chance {chance}: {}",
                if *sighted { "sighted" } else { "not sighted" }
            ),
            EventKind::ReportReceived {
                side,
                contact,
                hex,
                ship_count,
                kind,
                confidence,
                created,
            } => write!(
                f,
                "{} report: {ship_count} {}, {}, at {hex}, confidence {confidence}: {} {contact}",
                side.name(),
                if *ship_count == 1 { "ship" } else { "ships" },
                kind.name(),
                if *created { "new" } else { "refreshed" }
            ),
            EventKind::SightingNoticed {
                task_force,
                air_group,
                elapsed_hours,
            } => write!(
                f,
                "{task_force} noticed it was sighted by {air_group} at {}",
                ClockTime::from_elapsed(*elapsed_hours)
            ),
            EventKind::ContactAged {
                side,
                contact,
                age_pulses,
                confidence,
            } => write!(
                f,
                "{} {contact} aged to {age_pulses} pulses, confidence {confidence}",
                side.name()
            ),
            EventKind::ContactDropped {
                side,
                contact,
                age_pulses,
                confidence,
            } => write!(
                f,
                "{} {contact} dropped at {age_pulses} pulses, confidence {confidence}",
                side.name()
            ),
            EventKind::EngagementBegun { a, b, hex } => {
                write!(f, "{a} and {b} met at {hex}: engagement begun")
            }
            EventKind::SightingRoll {
                round,
                task_force,
                enemy,
                chance,
                sighted,
            } => write!(
                f,
                "round {round}: {task_force} looked for {enemy}, chance {chance}: {}",
                if *sighted { "sighted" } else { "not sighted" }
            ),
            EventKind::ForceSurprised {
                round,
                task_force,
                enemy,
            } => write!(f, "round {round}: {task_force} was surprised by {enemy}"),
            EventKind::Salvo {
                round,
                firer,
                target,
                weapon,
                shots,
                chance,
                expected,
                hits,
            } => write!(
                f,
                "round {round}: {firer} {} at {target}: {shots} shots x {chance} = {expected} expected, {hits} {}",
                weapon.name(),
                if *hits == 1 { "hit" } else { "hits" }
            ),
            EventKind::ShipDamaged {
                round,
                ship,
                track,
                hits,
                points,
                value_after,
            } => write!(
                f,
                "round {round}: {ship} took {hits} {} on {}, {points} points, now {value_after}",
                if *hits == 1 { "hit" } else { "hits" },
                track.name()
            ),
            EventKind::ShipSunk {
                round,
                ship,
                task_force,
            } => write!(f, "round {round}: {ship} of {task_force} sank"),
            EventKind::ForceBecameAlert { round, task_force } => {
                write!(f, "round {round}: {task_force} became alert")
            }
            EventKind::ForceBrokeOff {
                round,
                task_force,
                share,
            } => write!(
                f,
                "round {round}: {task_force} broke off, {share} of its ships sunk or crippled"
            ),
            EventKind::ForceSlippedAway {
                round,
                task_force,
                enemy,
            } => write!(f, "round {round}: {task_force} slipped away from {enemy}"),
            EventKind::ForcesPassedUnseen { round, a, b } => {
                write!(f, "round {round}: {a} and {b} passed unseen")
            }
            EventKind::EngagementEnded {
                round,
                a,
                b,
                hex,
                reason,
            } => write!(
                f,
                "engagement of {a} and {b} at {hex} ended in round {round}: {}",
                reason.name()
            ),
            EventKind::ClaimReceived {
                side,
                hex,
                claimed_sunk,
            } => write!(
                f,
                "{} claims {claimed_sunk} enemy {} sunk at {hex}",
                side.name(),
                if *claimed_sunk == 1 { "ship" } else { "ships" }
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

    /// The ID the next recorded event will get.
    pub(crate) fn next_id(&self) -> EventId {
        EventId::new(u32::try_from(self.events.len() + 1).expect("ran out of event IDs"))
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
