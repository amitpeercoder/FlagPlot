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
use crate::state::{AirGroupLocation, ShipSystem, Side, SystemState};
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
    /// Dropped by aircraft (decision 0026). Added at the end of the enum.
    Bomb,
}

impl Weapon {
    /// The weapon's name as the event log writes it.
    pub const fn name(self) -> &'static str {
        match self {
            Weapon::Gun => "guns",
            Weapon::Torpedo => "torpedoes",
            Weapon::Bomb => "bombs",
        }
    }
}

/// Who fires in a fight in the air (decision 6 of
/// `docs/decisions/0027-fighters-and-anti-aircraft-fire.md`).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum AirFirer {
    /// The fighters that went with a raid.
    Escorts,
    /// The fighters kept over their own hex.
    Patrol,
}

impl AirFirer {
    /// The name as the event log writes it.
    pub const fn name(self) -> &'static str {
        match self {
            AirFirer::Escorts => "escorts",
            AirFirer::Patrol => "patrol",
        }
    }
}

/// Who is fired at in a fight in the air (decision 6 of 0027).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum AirTarget {
    Patrol,
    Escorts,
    /// The bombers of the raid.
    Bombers,
}

impl AirTarget {
    /// The name as the event log writes it.
    pub const fn name(self) -> &'static str {
        match self {
            AirTarget::Patrol => "patrol",
            AirTarget::Escorts => "escorts",
            AirTarget::Bombers => "bombers",
        }
    }
}

/// One of the two damage tracks of a ship. Named systems are separate
/// (decision 0016).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum DamageTrack {
    Flotation,
    Fire,
}

impl DamageTrack {
    /// Every track, in the order gun hits are assigned to them.
    pub const ALL: [DamageTrack; 2] = [DamageTrack::Flotation, DamageTrack::Fire];

    /// The track's name as the event log writes it.
    pub const fn name(self) -> &'static str {
        match self {
            DamageTrack::Flotation => "flotation",
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

/// Why ships were split off a task force by the straggler rule (decision 12
/// of `docs/decisions/0016-systems-and-stragglers.md`).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum DetachReason {
    /// Stragglers that can steam, sent for the home port with escorts.
    SentHome,
    /// Stragglers left where they are.
    LeftBehind,
    /// Ships detached by a detach order.
    Ordered,
}

impl DetachReason {
    /// A stable number for the reason. Never renumber it; add new reasons at
    /// the end.
    pub const fn code(self) -> u8 {
        match self {
            DetachReason::SentHome => 1,
            DetachReason::LeftBehind => 2,
            DetachReason::Ordered => 3,
        }
    }

    /// The reason as the event log writes it.
    pub const fn name(self) -> &'static str {
        match self {
            DetachReason::SentHome => "sent home",
            DetachReason::LeftBehind => "left behind",
            DetachReason::Ordered => "ordered",
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
    /// An air group flew its search arc from its base, or from the hex of the
    /// ship it is aboard (decision 4 of 0025). `hex` is the hex it flew from.
    SearchFlown {
        air_group: AirGroupId,
        from: AirGroupLocation,
        hex: Hex,
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
        /// Elapsed hours since the scenario began at the meeting: the start
        /// of the pulse for forces that were in one hex at its start, else
        /// the start of the pulse plus the hours the arriving force had
        /// steamed.
        elapsed_hours: Fixed,
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
    /// A hit knocked, or tried to knock, one named system of a ship down one
    /// level (decision 0016). `before == after` when the ship lacks the
    /// system or it was already out.
    SystemHit {
        round: u32,
        ship: ShipId,
        system: ShipSystem,
        before: SystemState,
        after: SystemState,
    },
    /// The straggler rule split ships off a task force into a new one
    /// (decision 0016). `ships` are the stragglers moved and `escorts` the
    /// escorts moved, each in ID order. `escorts` is empty when the ships
    /// are left behind.
    ShipsDetached {
        parent: TaskForceId,
        new_force: TaskForceId,
        ships: Vec<ShipId>,
        escorts: Vec<ShipId>,
        reason: DetachReason,
    },
    /// A task force retired under the straggler rule: it is set to avoid
    /// combat and, if it has a home port, heads for it.
    ForceRetired {
        task_force: TaskForceId,
    },
    /// At the start of a pulse a force's side held a recent contact near it,
    /// and the force became alerted (decision 3 of
    /// `docs/decisions/0021-readiness-and-alarm.md`). `contact` is the
    /// nearest such contact, the lowest ID on a tie.
    ForceAlertedByContact {
        task_force: TaskForceId,
        contact: ContactId,
    },
    /// At the start of a pulse an alerted force no longer had such a contact
    /// and stood down (decision 3 of 0021).
    ForceStoodDown {
        task_force: TaskForceId,
    },
    /// One roll of the alarm of a fight in which shots were fired, for a
    /// force in its hex that was not alert (decision 5 of 0021). Recorded
    /// whether or not it succeeded. `by_a` and `by_b` are the two forces that
    /// fought, the lower ID first. On success the force is alerted by this
    /// event.
    AlarmRoll {
        task_force: TaskForceId,
        by_a: TaskForceId,
        by_b: TaskForceId,
        chance: Fixed,
        alarmed: bool,
    },
    /// An air group aboard a ship that sank was lost with it (decision 5 of
    /// 0025). `aircraft` is the number of ready aircraft it lost.
    AirGroupLost {
        air_group: AirGroupId,
        ship: ShipId,
        aircraft: u32,
    },
    /// An air group on the naval strike mission launched at a contact its
    /// side holds (decisions 3 to 6 of `docs/decisions/0026-air-strikes.md`).
    /// `hex` is the hex the contact names, and where the strike flies to.
    StrikeLaunched {
        air_group: AirGroupId,
        contact: ContactId,
        hex: Hex,
        aircraft: u32,
        distance_nm: Fixed,
    },
    /// A strike reached the hex it flew to and no enemy task force was there
    /// (decision 7 of 0026).
    StrikeFoundNothing {
        air_group: AirGroupId,
        hex: Hex,
    },
    /// A strike attacked one ship of an enemy task force (decisions 7 to 9
    /// of 0026).
    AirAttack {
        air_group: AirGroupId,
        task_force: TaskForceId,
        ship: ShipId,
        weapon: Weapon,
        shots: Fixed,
        chance: Fixed,
        expected: Fixed,
        hits: u32,
    },
    /// A fighter group sent escorts with a raid (decision 5 of
    /// `docs/decisions/0027-fighters-and-anti-aircraft-fire.md`).
    /// `with_air_group` is the first strike group of the raid.
    EscortsSent {
        air_group: AirGroupId,
        with_air_group: AirGroupId,
        aircraft: u32,
    },
    /// Fighters fired at fighters or bombers over the hex a raid attacked
    /// (decision 6 of 0027). `firer` is the first group of the firing
    /// fighters and `target` the first group of those fired at.
    AirFight {
        firer: AirGroupId,
        target: AirGroupId,
        firing: AirFirer,
        at: AirTarget,
        shots: Fixed,
        chance: Fixed,
        expected: Fixed,
        hits: u32,
    },
    /// The ships of a task force fired their anti-aircraft guns at the
    /// bombers of a raid (decision 7 of 0027). `air_group` is the raid's
    /// first strike group.
    AntiAircraftFire {
        task_force: TaskForceId,
        air_group: AirGroupId,
        shots: Fixed,
        chance: Fixed,
        expected: Fixed,
        hits: u32,
    },
    /// An air group lost aircraft to one fight or one round of fire
    /// (decision 11 of 0027). `remaining` is what it has ready afterwards.
    AircraftLost {
        air_group: AirGroupId,
        aircraft: u32,
        remaining: u32,
    },
    /// A strike that found nothing told its side so, and the side dropped the
    /// contact the strike was sent at (decision 0028).
    ContactNotThere {
        side: Side,
        contact: ContactId,
        hex: Hex,
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
                air_group, from, ..
            } => vec![
                EntityRef::AirGroup(*air_group),
                match from {
                    AirGroupLocation::Base(base) => EntityRef::Base(*base),
                    AirGroupLocation::Ship(ship) => EntityRef::Ship(*ship),
                },
            ],
            EventKind::AirGroupLost {
                air_group, ship, ..
            } => vec![EntityRef::AirGroup(*air_group), EntityRef::Ship(*ship)],
            EventKind::StrikeLaunched {
                air_group, contact, ..
            } => vec![
                EntityRef::AirGroup(*air_group),
                EntityRef::Contact(*contact),
            ],
            EventKind::StrikeFoundNothing { air_group, .. } => {
                vec![EntityRef::AirGroup(*air_group)]
            }
            EventKind::ContactNotThere { contact, .. } => vec![EntityRef::Contact(*contact)],
            EventKind::AirAttack {
                air_group,
                task_force,
                ship,
                ..
            } => vec![
                EntityRef::AirGroup(*air_group),
                EntityRef::TaskForce(*task_force),
                EntityRef::Ship(*ship),
            ],
            EventKind::EscortsSent {
                air_group,
                with_air_group,
                ..
            } => vec![
                EntityRef::AirGroup(*air_group),
                EntityRef::AirGroup(*with_air_group),
            ],
            EventKind::AirFight { firer, target, .. } => {
                vec![EntityRef::AirGroup(*firer), EntityRef::AirGroup(*target)]
            }
            EventKind::AntiAircraftFire {
                task_force,
                air_group,
                ..
            } => vec![
                EntityRef::TaskForce(*task_force),
                EntityRef::AirGroup(*air_group),
            ],
            EventKind::AircraftLost { air_group, .. } => vec![EntityRef::AirGroup(*air_group)],
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
            EventKind::ShipDamaged { ship, .. } | EventKind::SystemHit { ship, .. } => {
                vec![EntityRef::Ship(*ship)]
            }
            EventKind::ShipSunk {
                ship, task_force, ..
            } => vec![EntityRef::Ship(*ship), EntityRef::TaskForce(*task_force)],
            EventKind::ForceBecameAlert { task_force, .. }
            | EventKind::ForceBrokeOff { task_force, .. } => {
                vec![EntityRef::TaskForce(*task_force)]
            }
            // A side is not an entity: the claim names no force or ship.
            EventKind::ClaimReceived { .. } => Vec::new(),
            EventKind::ShipsDetached {
                parent,
                new_force,
                ships,
                escorts,
                ..
            } => {
                let mut entities = vec![
                    EntityRef::TaskForce(*parent),
                    EntityRef::TaskForce(*new_force),
                ];
                entities.extend(
                    ships
                        .iter()
                        .chain(escorts)
                        .map(|ship| EntityRef::Ship(*ship)),
                );
                entities
            }
            EventKind::ForceRetired { task_force } | EventKind::ForceStoodDown { task_force } => {
                vec![EntityRef::TaskForce(*task_force)]
            }
            EventKind::ForceAlertedByContact {
                task_force,
                contact,
            } => vec![
                EntityRef::TaskForce(*task_force),
                EntityRef::Contact(*contact),
            ],
            EventKind::AlarmRoll {
                task_force,
                by_a,
                by_b,
                ..
            } => vec![
                EntityRef::TaskForce(*task_force),
                EntityRef::TaskForce(*by_a),
                EntityRef::TaskForce(*by_b),
            ],
        }
    }
}

/// "round N: " before the text of an event of a surface fight. Damage
/// events carry round 0 when the hits fell outside a fight (an air attack,
/// decision 0026), and then write nothing.
struct RoundPrefix(u32);

impl fmt::Display for RoundPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 == 0 {
            Ok(())
        } else {
            write!(f, "round {}: ", self.0)
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
                from,
                hex,
                from_deg,
                to_deg,
                range_nm,
                aircraft,
            } => match from {
                AirGroupLocation::Base(base) => write!(
                    f,
                    "{air_group} searched {from_deg:03}-{to_deg:03} out to {range_nm} nm from {base} with {aircraft} aircraft"
                ),
                AirGroupLocation::Ship(ship) => write!(
                    f,
                    "{air_group} searched {from_deg:03}-{to_deg:03} out to {range_nm} nm from {ship} at {hex} with {aircraft} aircraft"
                ),
            },
            EventKind::AirGroupLost {
                air_group,
                ship,
                aircraft,
            } => write!(f, "{air_group} lost with {ship}: {aircraft} aircraft"),
            EventKind::StrikeLaunched {
                air_group,
                contact,
                hex,
                aircraft,
                distance_nm,
            } => write!(
                f,
                "{air_group} launched {aircraft} aircraft at {contact} at {hex}, {distance_nm} nm"
            ),
            EventKind::StrikeFoundNothing { air_group, hex } => {
                write!(f, "{air_group} found nothing at {hex}")
            }
            EventKind::ContactNotThere { side, contact, hex } => {
                write!(f, "{} {contact} not there at {hex}: dropped", side.name())
            }
            EventKind::AirAttack {
                air_group,
                ship,
                weapon,
                shots,
                chance,
                expected,
                hits,
                ..
            } => write!(
                f,
                "{air_group} {} at {ship}: {shots} shots x {chance} = {expected} expected, {hits} {}",
                weapon.name(),
                if *hits == 1 { "hit" } else { "hits" }
            ),
            EventKind::EscortsSent {
                air_group,
                with_air_group,
                aircraft,
            } => write!(f, "{air_group} sent {aircraft} escorts with {with_air_group}"),
            EventKind::AirFight {
                firer,
                target,
                firing,
                at,
                shots,
                chance,
                expected,
                hits,
            } => write!(
                f,
                "{firer} {} at the {} of {target}: {shots} shots x {chance} = {expected} expected, {hits} {}",
                firing.name(),
                at.name(),
                if *hits == 1 { "hit" } else { "hits" }
            ),
            EventKind::AntiAircraftFire {
                task_force,
                air_group,
                shots,
                chance,
                expected,
                hits,
            } => write!(
                f,
                "{task_force} guns at the raid of {air_group}: {shots} shots x {chance} = {expected} expected, {hits} {}",
                if *hits == 1 { "hit" } else { "hits" }
            ),
            EventKind::AircraftLost {
                air_group,
                aircraft,
                remaining,
            } => write!(f, "{air_group} lost {aircraft} aircraft, {remaining} left"),
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
            } => write_report(
                f,
                (*side, *contact, *hex, *ship_count, *kind, *confidence),
                *created,
                false,
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
            EventKind::EngagementBegun {
                a,
                b,
                hex,
                elapsed_hours,
            } => write!(
                f,
                "{a} and {b} met at {hex} at {}: engagement begun",
                ClockTime::from_elapsed(*elapsed_hours)
            ),
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
                "{}{ship} took {hits} {} on {}, {points} points, now {value_after}",
                RoundPrefix(*round),
                if *hits == 1 { "hit" } else { "hits" },
                track.name()
            ),
            EventKind::SystemHit {
                round,
                ship,
                system,
                before,
                after,
            } => {
                if before == after {
                    write!(
                        f,
                        "{}{ship} {} hit, no change, still {}",
                        RoundPrefix(*round),
                        system.name(),
                        before.name()
                    )
                } else {
                    write!(
                        f,
                        "{}{ship} {} hit, {} -> {}",
                        RoundPrefix(*round),
                        system.name(),
                        before.name(),
                        after.name()
                    )
                }
            }
            EventKind::ShipSunk {
                round,
                ship,
                task_force,
            } => write!(f, "{}{ship} of {task_force} sank", RoundPrefix(*round)),
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
            EventKind::ShipsDetached {
                parent,
                new_force,
                ships,
                escorts,
                reason,
            } => {
                let ships = join_ships(ships);
                match reason {
                    DetachReason::SentHome => {
                        write!(f, "{parent} sent {ships} home as {new_force}")?;
                        if !escorts.is_empty() {
                            write!(f, " with escorts {}", join_ships(escorts))?;
                        }
                        Ok(())
                    }
                    DetachReason::LeftBehind => {
                        write!(f, "{parent} left {ships} behind as {new_force}")
                    }
                    DetachReason::Ordered => {
                        write!(f, "{parent} detached {ships} as {new_force}")
                    }
                }
            }
            EventKind::ForceRetired { task_force } => write!(f, "{task_force} retires"),
            EventKind::ForceAlertedByContact {
                task_force,
                contact,
            } => write!(f, "{task_force} alerted by {contact}"),
            EventKind::ForceStoodDown { task_force } => write!(f, "{task_force} stood down"),
            EventKind::AlarmRoll {
                task_force,
                by_a,
                by_b,
                chance,
                alarmed,
            } => write!(
                f,
                "{task_force} heard the fight of {by_a} and {by_b}, chance {chance}: {}",
                if *alarmed { "alarmed" } else { "not alarmed" }
            ),
        }
    }
}

/// Ship IDs written as a list, such as `ship:2, ship:4`.
fn join_ships(ships: &[ShipId]) -> String {
    let names: Vec<String> = ships.iter().map(ShipId::to_string).collect();
    names.join(", ")
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

/// The text of a `ReportReceived` event. A report logged on day 0, before the
/// first day, is a contact a scenario started a side with, and says so
/// (decision 8 of `docs/decisions/0021-readiness-and-alarm.md`). Every other
/// report reads as before.
fn write_report(
    f: &mut fmt::Formatter<'_>,
    (side, contact, hex, ship_count, kind, confidence): (
        Side,
        ContactId,
        Hex,
        u32,
        ShipKind,
        Fixed,
    ),
    created: bool,
    held_at_start: bool,
) -> fmt::Result {
    write!(
        f,
        "{} report: {ship_count} {}, {}, at {hex}, confidence {confidence}{}: {} {contact}",
        side.name(),
        if ship_count == 1 { "ship" } else { "ships" },
        kind.name(),
        if held_at_start {
            ", held at the start"
        } else {
            ""
        },
        if created { "new" } else { "refreshed" }
    )
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pulse = self.pulse.map_or("-", Pulse::name);
        write!(f, "#{} D{} {pulse:<9} ", self.id.raw(), self.day)?;
        match &self.kind {
            EventKind::ReportReceived {
                side,
                contact,
                hex,
                ship_count,
                kind,
                confidence,
                created,
            } if self.day == 0 => write_report(
                f,
                (*side, *contact, *hex, *ship_count, *kind, *confidence),
                *created,
                true,
            )?,
            kind => write!(f, "{kind}")?,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_system_hit_reads_as_a_change_or_as_no_change() {
        let hit = |before, after| EventKind::SystemHit {
            round: 2,
            ship: ShipId::new(5),
            system: ShipSystem::Propulsion,
            before,
            after,
        };
        assert_eq!(
            hit(SystemState::Working, SystemState::Degraded).to_string(),
            "round 2: ship:5 propulsion hit, working -> degraded"
        );
        assert_eq!(
            hit(SystemState::Out, SystemState::Out).to_string(),
            "round 2: ship:5 propulsion hit, no change, still out"
        );
    }

    #[test]
    fn a_system_hit_is_about_its_ship() {
        let kind = EventKind::SystemHit {
            round: 1,
            ship: ShipId::new(3),
            system: ShipSystem::MainBattery,
            before: SystemState::Working,
            after: SystemState::Degraded,
        };
        assert_eq!(kind.entities(), vec![EntityRef::Ship(ShipId::new(3))]);
    }

    #[test]
    fn detaching_ships_reads_clearly() {
        let ships = |raw: &[u32]| raw.iter().map(|n| ShipId::new(*n)).collect::<Vec<_>>();
        let sent = EventKind::ShipsDetached {
            parent: TaskForceId::new(1),
            new_force: TaskForceId::new(2),
            ships: ships(&[2]),
            escorts: ships(&[4, 5]),
            reason: DetachReason::SentHome,
        };
        assert_eq!(
            sent.to_string(),
            "task_force:1 sent ship:2 home as task_force:2 with escorts ship:4, ship:5"
        );
        let alone = EventKind::ShipsDetached {
            parent: TaskForceId::new(1),
            new_force: TaskForceId::new(2),
            ships: ships(&[2, 3]),
            escorts: Vec::new(),
            reason: DetachReason::SentHome,
        };
        assert_eq!(
            alone.to_string(),
            "task_force:1 sent ship:2, ship:3 home as task_force:2"
        );
        let left = EventKind::ShipsDetached {
            parent: TaskForceId::new(1),
            new_force: TaskForceId::new(3),
            ships: ships(&[6]),
            escorts: Vec::new(),
            reason: DetachReason::LeftBehind,
        };
        assert_eq!(
            left.to_string(),
            "task_force:1 left ship:6 behind as task_force:3"
        );
        assert_eq!(
            left.entities(),
            vec![
                EntityRef::TaskForce(TaskForceId::new(1)),
                EntityRef::TaskForce(TaskForceId::new(3)),
                EntityRef::Ship(ShipId::new(6)),
            ]
        );
        let retired = EventKind::ForceRetired {
            task_force: TaskForceId::new(1),
        };
        assert_eq!(retired.to_string(), "task_force:1 retires");
        assert_eq!(
            retired.entities(),
            vec![EntityRef::TaskForce(TaskForceId::new(1))]
        );
    }

    #[test]
    fn detach_reason_codes_are_stable() {
        assert_eq!(DetachReason::SentHome.code(), 1);
        assert_eq!(DetachReason::LeftBehind.code(), 2);
        assert_eq!(DetachReason::SentHome.name(), "sent home");
        assert_eq!(DetachReason::LeftBehind.name(), "left behind");
    }
}
