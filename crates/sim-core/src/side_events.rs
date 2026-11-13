//! What a side may know of what happened (decision 0047).
//!
//! The event log is the truth: it holds both sides' orders, rolls, hits and
//! losses. A side is given its own version of it, built here, and nothing
//! else leaves the crate for a player. Each event of the log is, for one
//! side, whole, cut down to what the side may know, or absent. Nothing is
//! added or altered.
//!
//! # The shape of a cut event
//!
//! [`SideEventKind`] is either the full [`EventKind`] (`Whole`) or one of a
//! few dedicated variants that simply have no field for what the side may not
//! see: the hits scored, and the chance and expected hits that would give
//! them away. A cut event cannot carry a removed field because there is no
//! field to put it in, and no `Option` or zeroed value stands in for it.
//!
//! # The rule, kind by kind
//!
//! [`seen`] is one exhaustive `match` over every [`EventKind`], with no
//! wildcard arm. A new kind of event does not compile until someone decides
//! who may see it.
//!
//! # The rule of a surface fight (decision 5)
//!
//! A side knows of a fight from the round in which its force first sights
//! the enemy or is first fired on. That round is worked out from the fight's
//! own events, which sit between its `EngagementBegun` and its
//! `EngagementEnded`: the side's successful sighting roll, a salvo fired at
//! its ships or hits on them, its force being surprised or slipping away, or
//! its own salvo. A fight the side never knows of is wholly absent for it,
//! including `EngagementBegun` and `EngagementEnded`. For a fight it knows,
//! those two are shown whole, at their place in the log.
//!
//! An event of a fight the side would see by the table (its own) is shown
//! only if it carries a round that is not before the round the side first
//! knew. Nearly every own event is: a force cannot fire or be hit before it
//! knew. The exception is a force that breaks off in a round in which it has
//! neither sighted nor been fired on, because its ships were crippled before
//! the meeting: it is not told of a fight it never knew of.
//!
//! Reads: the event log, which side each unit and order belongs to. Writes
//! nothing. Records no event and draws no random number.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use crate::events::{
    AirFirer, AirTarget, Breakdown, Cause, EngagementEndReason, Event, EventKind, Factor, Weapon,
};
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{
    AirGroupId, BaseId, EntityRef, EventId, LandUnitId, OrderId, ShipId, TaskForceId,
};
use crate::state::{Side, TruthState};
use crate::time::{ClockTime, Pulse};

/// An event as one side may know it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SideEvent {
    /// The event's place in the full log. The number is the umpire's and is
    /// hidden ([`EventId`] is opaque outside this crate): the side can
    /// compare it, and look the event up with [`crate::SideView::event`], but
    /// cannot read how many events the full log holds or which of them it
    /// does not see. What the side counts by is `number`.
    pub id: EventId,
    /// The side's own count of the events it may see: 1 for the first the
    /// view gives, 2 for the next, and so on (decision 0086). It does not
    /// move with what the other side does.
    pub number: u32,
    pub day: u32,
    pub pulse: Option<Pulse>,
    pub kind: SideEventKind,
    /// The side's number of the order this event is about, for an order
    /// accepted or rejected. It is the order's place among the side's own
    /// orders (see [`SideCause`]), so the text of the event can name it.
    pub order_number: Option<u32>,
    /// The causes the side may see: its own orders, and events it may see,
    /// each by the side's own number.
    pub causes: Vec<SideCause>,
    /// True if the event may have causes the side cannot see: an order of the
    /// enemy's or an event of the enemy's. The flag is set by the kind of
    /// the event, as this side is given it, and never by what a particular
    /// event happened to cite, so it does not say whether the enemy gave an
    /// order or did anything at all (decision 0085). A kind that can have
    /// such causes always says so; the causes the side may see are listed in
    /// `causes` whatever the flag says.
    pub hidden_causes: bool,
    /// Present when the log has one. For a whole event it is the log's. For a
    /// cut event it is cut too (see [`SideBreakdown`]).
    pub breakdown: Option<SideBreakdown>,
}

/// A cause of an event as the side counts it (decision 0086): an event by the
/// side's number of it, an order by its place among the side's own orders, 1
/// for the first the side gave, in the order [`crate::SideView::orders`]
/// gives them. Neither is a number of the full log or the order book, which
/// count both sides' events and orders.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum SideCause {
    Event(u32),
    Order(u32),
}

impl fmt::Display for SideCause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SideCause::Event(number) => write!(f, "#{number}"),
            SideCause::Order(number) => write!(f, "order:{number}"),
        }
    }
}

/// A breakdown as a side may see it. For a whole event, every factor and the
/// result. For a cut event, only the factors that are the side's own, and no
/// result, because the result is the quantity that was cut. A claim is the
/// one whole event with no breakdown for its side (decision 0060).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SideBreakdown {
    pub factors: Vec<Factor>,
    pub result: Option<Fixed>,
}

/// What of an event a side may know.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SideEventKind {
    /// The event as the log has it. The one alteration made to it is its
    /// round, if it carries a round of a surface fight: that is counted from
    /// the round in which the side first knew of the fight, as the side's
    /// other events of the fight are (decision 0085, F6).
    Whole(EventKind),
    /// The side's own salvo: that it fired, and with how many shots. Not the
    /// ship it fired at, which is the enemy's (decision 0065).
    Salvo {
        round: u32,
        firer: ShipId,
        weapon: Weapon,
        shots: Fixed,
    },
    /// A salvo fired at the side's ship: that it was shot at, and with what.
    /// Not the shots, nor what they did (the hits it took reach it as
    /// `ShipDamaged`), nor the ship that fired.
    SalvoAgainst {
        round: u32,
        target: ShipId,
        weapon: Weapon,
    },
    /// An air attack on the side's ship: on which ship of which force, and
    /// how many aircraft it can count. Not the chance or the hits, nor the
    /// air group that attacked.
    AirAttackAgainst {
        task_force: TaskForceId,
        ship: ShipId,
        shots: Fixed,
    },
    /// The side's own air attack on a ship. Not the ship or its force.
    AirAttack {
        air_group: AirGroupId,
        weapon: Weapon,
        shots: Fixed,
    },
    /// The side's own fighters firing in a fight in the air. Not the air
    /// group they fired at. `after_attack` says whether it was the part of
    /// the patrol's fire worked after the attack (decision 0087): the side's
    /// own fighters, so nothing of the enemy is given away.
    AirFight {
        firer: AirGroupId,
        firing: AirFirer,
        at: AirTarget,
        shots: Fixed,
        after_attack: bool,
    },
    /// The side's own ships firing at a raid. Not the raid's air group.
    /// `after_attack` says whether it was the part of the fire worked after
    /// the attack (decision 0079): the side's own guns, so nothing of the
    /// enemy is given away.
    AntiAircraftFire {
        task_force: TaskForceId,
        shots: Fixed,
        after_attack: bool,
    },
    /// The side's own base firing at a raid. Not the raid's air group.
    BaseAntiAircraftFire {
        base: BaseId,
        shots: Fixed,
        after_attack: bool,
    },
    /// The side's own bombs falling on an enemy base: that it attacked and
    /// with how many shots, not what they hit. The base is named: a base is
    /// on the map for both sides, and the side ordered the strike on it.
    BaseAttacked {
        air_group: AirGroupId,
        base: BaseId,
        shots: Fixed,
    },
    /// A raid on the side's own base: everything the log has but the air
    /// group that raided.
    BaseAttackedAgainst {
        base: BaseId,
        shots: Fixed,
        chance: Fixed,
        expected: Fixed,
        hits: u32,
        hits_on_aircraft: u32,
        hits_on_supply: u32,
    },
    /// The side's own search that sighted a force: which group. Not the
    /// distance or the chance, which are worked from where the force really
    /// was, while the report that follows may name a neighbouring hex
    /// (decision 0061); and not the force, which the report does not name
    /// either (decisions 0012 and 0065).
    SearchAttempt { air_group: AirGroupId },
    /// A surface fight the side knows of has begun. `task_force` is the
    /// side's own of the two forces the log names. `elapsed_hours` is the
    /// hour of the round in which the side first knew of the fight, not the
    /// hour the forces met (decision 0085, F6).
    EngagementBegun {
        task_force: TaskForceId,
        hex: Hex,
        elapsed_hours: Fixed,
    },
    /// A surface fight the side knows of has ended. `task_force` is the
    /// side's own of the two forces the log names. `round` is counted from
    /// the round in which the side first knew of the fight.
    EngagementEnded {
        round: u32,
        task_force: TaskForceId,
        hex: Hex,
        reason: SideEndReason,
    },
    /// The side's own force looked and sighted. Not the force it sighted.
    SightingRoll {
        round: u32,
        task_force: TaskForceId,
        chance: Fixed,
        fatigue_factor: Fixed,
    },
    /// The side's own force was surprised. Not by which force.
    ForceSurprised { round: u32, task_force: TaskForceId },
    /// The side's own force slipped away. Not from which force.
    ForceSlippedAway { round: u32, task_force: TaskForceId },
    /// The side's own force heard a fight and was alarmed. Not the forces
    /// that fought.
    AlarmRoll {
        task_force: TaskForceId,
        chance: Fixed,
    },
}

/// Why a surface fight ended, as a side is told (decision 0085, F5). It has
/// the reasons of [`EngagementEndReason`] and one more, `ContactLost`, and
/// the two reasons that are an enemy's doing, a force sunk and a force broke
/// off, are the side's only when the force was its own. Otherwise the side is
/// told it lost contact, which is all it can know: the enemy's force is
/// gone, and it cannot tell sunk from broken off from out of sight.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum SideEndReason {
    /// A force that avoids sighted the enemy first and left.
    SlippedAway,
    /// The side's own force had no ship afloat.
    ForceSunk,
    /// The side's own force broke off.
    BrokeOff,
    /// Neither force sighted the other in all the rounds.
    PassedUnseen,
    /// The last round was fought.
    MaxRounds,
    /// The enemy's force was gone: sunk or broken off, the side cannot tell.
    ContactLost,
}

impl SideEndReason {
    /// The reason as the side's event text writes it.
    pub const fn name(self) -> &'static str {
        match self {
            SideEndReason::SlippedAway => EngagementEndReason::SlippedAway.name(),
            SideEndReason::ForceSunk => EngagementEndReason::ForceSunk.name(),
            SideEndReason::BrokeOff => EngagementEndReason::BrokeOff.name(),
            SideEndReason::PassedUnseen => EngagementEndReason::PassedUnseen.name(),
            SideEndReason::MaxRounds => EngagementEndReason::MaxRounds.name(),
            SideEndReason::ContactLost => "contact lost",
        }
    }
}

impl SideEventKind {
    /// The entities the event names. A whole event names only the side's own
    /// units, a base, a contact or the target the side itself gave an order.
    pub fn entities(&self) -> Vec<EntityRef> {
        match self {
            SideEventKind::Whole(kind) => kind.entities(),
            SideEventKind::Salvo { firer: ship, .. }
            | SideEventKind::SalvoAgainst { target: ship, .. } => vec![EntityRef::Ship(*ship)],
            SideEventKind::AirAttackAgainst {
                task_force, ship, ..
            } => vec![EntityRef::TaskForce(*task_force), EntityRef::Ship(*ship)],
            SideEventKind::AirAttack { air_group, .. }
            | SideEventKind::AirFight {
                firer: air_group, ..
            }
            | SideEventKind::SearchAttempt { air_group } => vec![EntityRef::AirGroup(*air_group)],
            SideEventKind::AntiAircraftFire { task_force, .. }
            | SideEventKind::EngagementBegun { task_force, .. }
            | SideEventKind::EngagementEnded { task_force, .. }
            | SideEventKind::SightingRoll { task_force, .. }
            | SideEventKind::ForceSurprised { task_force, .. }
            | SideEventKind::ForceSlippedAway { task_force, .. }
            | SideEventKind::AlarmRoll { task_force, .. } => {
                vec![EntityRef::TaskForce(*task_force)]
            }
            SideEventKind::BaseAntiAircraftFire { base, .. }
            | SideEventKind::BaseAttackedAgainst { base, .. } => vec![EntityRef::Base(*base)],
            SideEventKind::BaseAttacked {
                air_group, base, ..
            } => vec![EntityRef::AirGroup(*air_group), EntityRef::Base(*base)],
        }
    }
}

impl fmt::Display for SideEventKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SideEventKind::Whole(kind) => write!(f, "{kind}"),
            SideEventKind::Salvo {
                round,
                firer,
                weapon,
                shots,
            } => write!(f, "round {round}: {firer} {}: {shots} shots", weapon.name()),
            SideEventKind::SalvoAgainst {
                round,
                target,
                weapon,
            } => write!(f, "round {round}: {} at {target}", weapon.name()),
            SideEventKind::AirAttackAgainst { ship, shots, .. } => {
                write!(f, "{ship} attacked by {shots} aircraft")
            }
            SideEventKind::AirAttack {
                air_group,
                weapon,
                shots,
            } => write!(f, "{air_group} {} at a ship: {shots} shots", weapon.name()),
            SideEventKind::AirFight {
                firer,
                firing,
                at,
                shots,
                after_attack,
            } => write!(
                f,
                "{firer} {} at the enemy's {}: {shots} shots{}",
                firing.name(),
                at.name(),
                if *after_attack { " after the attack" } else { "" }
            ),
            SideEventKind::AntiAircraftFire {
                task_force,
                shots,
                after_attack,
            } => {
                write!(
                    f,
                    "{task_force} guns at a raid: {shots} shots{}",
                    if *after_attack { " after the attack" } else { "" }
                )
            }
            SideEventKind::BaseAntiAircraftFire {
                base,
                shots,
                after_attack,
            } => {
                write!(
                    f,
                    "{base} guns at a raid: {shots} shots{}",
                    if *after_attack { " after the attack" } else { "" }
                )
            }
            SideEventKind::BaseAttacked {
                air_group,
                base,
                shots,
            } => write!(f, "{air_group} bombs at {base}: {shots} shots"),
            SideEventKind::BaseAttackedAgainst {
                base,
                shots,
                chance,
                expected,
                hits,
                hits_on_aircraft,
                hits_on_supply,
            } => write!(
                f,
                "bombs at {base}: {shots} shots x {chance} = {expected} expected, {hits} {}: {hits_on_aircraft} on aircraft, {hits_on_supply} on supply, {} on nothing",
                if *hits == 1 { "hit" } else { "hits" },
                hits - hits_on_aircraft - hits_on_supply
            ),
            SideEventKind::SearchAttempt { air_group } => {
                write!(f, "{air_group} searched and sighted a force")
            }
            SideEventKind::EngagementBegun {
                task_force,
                hex,
                elapsed_hours,
            } => write!(
                f,
                "{task_force} met the enemy at {hex} at {}: engagement begun",
                ClockTime::from_elapsed(*elapsed_hours)
            ),
            SideEventKind::EngagementEnded {
                round,
                task_force,
                hex,
                reason,
            } => write!(
                f,
                "engagement of {task_force} at {hex} ended in round {round}: {}",
                reason.name()
            ),
            SideEventKind::SightingRoll {
                round,
                task_force,
                chance,
                fatigue_factor,
            } => {
                write!(f, "round {round}: {task_force} looked, chance {chance}")?;
                // Shown only where fatigue is in play (decision 0037).
                if *fatigue_factor != Fixed::ONE {
                    write!(f, " (fatigue factor {fatigue_factor})")?;
                }
                write!(f, ": sighted")
            }
            SideEventKind::ForceSurprised { round, task_force } => {
                write!(f, "round {round}: {task_force} was surprised")
            }
            SideEventKind::ForceSlippedAway { round, task_force } => {
                write!(f, "round {round}: {task_force} slipped away")
            }
            SideEventKind::AlarmRoll { task_force, chance } => {
                write!(f, "{task_force} heard a fight, chance {chance}: alarmed")
            }
        }
    }
}

impl fmt::Display for SideEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pulse = self.pulse.map_or("-", Pulse::name);
        write!(f, "#{} D{} {pulse:<9} ", self.number, self.day)?;
        // An order is named by the side's own number of it, never the order
        // book's.
        match (&self.kind, self.order_number) {
            (SideEventKind::Whole(EventKind::OrderAccepted { target, .. }), Some(number)) => {
                write!(f, "order:{number} accepted for {target}")?;
            }
            (
                SideEventKind::Whole(EventKind::OrderRejected { target, reason, .. }),
                Some(number),
            ) => write!(f, "order:{number} rejected for {target}: {reason}")?,
            (kind, _) => write!(f, "{kind}")?,
        }
        if !self.causes.is_empty() {
            let causes: Vec<String> = self.causes.iter().map(SideCause::to_string).collect();
            write!(f, "  <- {}", causes.join(", "))?;
        }
        if self.hidden_causes {
            write!(f, "  (and causes you cannot see)")?;
        }
        Ok(())
    }
}

/// Which side each unit and order belongs to. Units are never removed from
/// the state, so a sunk ship is still known by its side.
#[derive(Clone, Default, Debug)]
pub(crate) struct Owners {
    pub(crate) ships: BTreeMap<ShipId, Side>,
    pub(crate) forces: BTreeMap<TaskForceId, Side>,
    pub(crate) air_groups: BTreeMap<AirGroupId, Side>,
    pub(crate) bases: BTreeMap<BaseId, Side>,
    pub(crate) land_units: BTreeMap<LandUnitId, Side>,
    pub(crate) orders: BTreeMap<OrderId, Side>,
    /// The task forces with no ship that are not away at the yard. A force
    /// emptied by sinking is never removed and never given a ship again, so a
    /// force here was emptied once, and for a fight that ended in a force
    /// with no ship afloat, whether the side's own force was the one is read
    /// from this and the log (decision 0085, F5). A force away at the yard
    /// has no ship either but is not sunk, and is given its ships again on
    /// its day of return (decision 0069), so it is left out.
    pub(crate) emptied_forces: BTreeSet<TaskForceId>,
    /// Minutes in a round of a surface fight, from the reference data's
    /// surface combat figures (0 without them, when there are no fights).
    /// The hour a side first knew of a fight is worked from it (decision
    /// 0085, F6).
    pub(crate) round_minutes: u32,
}

impl Owners {
    pub(crate) fn from_state(
        truth: &TruthState,
        orders: &BTreeMap<OrderId, crate::orders::Order>,
        round_minutes: u32,
    ) -> Owners {
        Owners {
            emptied_forces: truth
                .task_forces
                .iter()
                .filter(|(_, f)| f.ships.is_empty() && f.yard_trip.is_none())
                .map(|(id, _)| *id)
                .collect(),
            round_minutes,
            ships: truth.ships.iter().map(|(id, s)| (*id, s.side)).collect(),
            forces: truth
                .task_forces
                .iter()
                .map(|(id, f)| (*id, f.side))
                .collect(),
            air_groups: truth
                .air_groups
                .iter()
                .map(|(id, g)| (*id, g.side))
                .collect(),
            bases: truth.bases.iter().map(|(id, b)| (*id, b.side)).collect(),
            land_units: truth
                .land_units
                .iter()
                .map(|(id, u)| (*id, u.side))
                .collect(),
            orders: orders.iter().map(|(id, o)| (*id, o.side)).collect(),
        }
    }

    fn entity(&self, entity: EntityRef) -> Option<Side> {
        match entity {
            EntityRef::Ship(id) => self.ships.get(&id).copied(),
            EntityRef::TaskForce(id) => self.forces.get(&id).copied(),
            EntityRef::AirGroup(id) => self.air_groups.get(&id).copied(),
            EntityRef::Base(id) => self.bases.get(&id).copied(),
            EntityRef::LandUnit(id) => self.land_units.get(&id).copied(),
            EntityRef::Pilot(_) | EntityRef::Contact(_) => None,
        }
    }
}

/// What a side sees of one event.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) enum Seen {
    Whole,
    Cut(SideEventKind),
    Nothing,
}

/// The one place the rule of decision 0047 is written down, kind by kind.
/// `knows_fight` is, for `EngagementBegun` and `EngagementEnded`, whether the
/// side has come to know of the fight (decision 5); no other kind reads it.
///
/// No wildcard arm: a new kind of event must be placed here.
pub(crate) fn seen(kind: &EventKind, side: Side, owners: &Owners, knows_fight: bool) -> Seen {
    let own_ship = |id: &ShipId| owners.ships.get(id) == Some(&side);
    let own_force = |id: &TaskForceId| owners.forces.get(id) == Some(&side);
    let own_group = |id: &AirGroupId| owners.air_groups.get(id) == Some(&side);
    let own_base = |id: &BaseId| owners.bases.get(id) == Some(&side);
    let own_unit = |id: &LandUnitId| owners.land_units.get(id) == Some(&side);
    let whole_if = |own: bool| if own { Seen::Whole } else { Seen::Nothing };
    let cut_if = |own: bool, cut: SideEventKind| {
        if own {
            Seen::Cut(cut)
        } else {
            Seen::Nothing
        }
    };

    match kind {
        // An order is its issuer's. Its target may be a unit that is not the
        // issuer's (a rejected order), so the order decides.
        EventKind::OrderAccepted { order, target }
        | EventKind::OrderRejected { order, target, .. } => whole_if(
            owners
                .orders
                .get(order)
                .copied()
                .or_else(|| owners.entity(*target))
                == Some(side),
        ),

        // Task forces.
        EventKind::TaskForceSteamed { task_force, .. }
        | EventKind::TaskForceArrived { task_force, .. }
        | EventKind::TaskForceOutOfFuel { task_force, .. }
        | EventKind::TaskForceTurnedBack { task_force, .. }
        | EventKind::TaskForceFatigue { task_force, .. }
        | EventKind::LoadingComplete { task_force, .. }
        | EventKind::UnloadingComplete { task_force, .. }
        | EventKind::ForceRetired { task_force }
        | EventKind::ForceStoodDown { task_force }
        | EventKind::ForceAlertedByContact { task_force, .. }
        | EventKind::ForceBecameAlert { task_force, .. }
        | EventKind::ForceBrokeOff { task_force, .. }
        // Away at the yard and back: the side's own, never shown to the enemy
        // (decision 0069, 9).
        | EventKind::ForceLeftForYard { task_force, .. }
        | EventKind::ForceReturnedFromYard { task_force, .. } => whole_if(own_force(task_force)),
        // The force surprised, without the one that surprised it.
        EventKind::ForceSurprised {
            round, task_force, ..
        } => cut_if(
            own_force(task_force),
            SideEventKind::ForceSurprised {
                round: *round,
                task_force: *task_force,
            },
        ),
        // The force that slipped away, without the one it slipped away from.
        EventKind::ForceSlippedAway {
            round, task_force, ..
        } => cut_if(
            own_force(task_force),
            SideEventKind::ForceSlippedAway {
                round: *round,
                task_force: *task_force,
            },
        ),
        EventKind::ShipsDetached { parent, .. } => whole_if(own_force(parent)),
        // The hearer's, only if it was alarmed: a failed alarm means the
        // force noticed nothing. Without the forces that fought.
        EventKind::AlarmRoll {
            task_force,
            chance,
            alarmed,
            ..
        } => cut_if(
            *alarmed && own_force(task_force),
            SideEventKind::AlarmRoll {
                task_force: *task_force,
                chance: *chance,
            },
        ),

        // Ships.
        EventKind::FuelBurned { ship, .. }
        | EventKind::ShipRefuelled { ship, .. }
        | EventKind::ShipRepaired { ship, .. }
        | EventKind::CargoLoaded { ship, .. }
        | EventKind::CargoUnloaded { ship, .. }
        | EventKind::ShipDamaged { ship, .. }
        | EventKind::SystemHit { ship, .. }
        | EventKind::ShipSunk { ship, .. } => whole_if(own_ship(ship)),

        // Air groups.
        EventKind::SearchFlown { air_group, .. }
        | EventKind::DamagedAircraftDestroyedOnGround { air_group, .. }
        | EventKind::AircraftWorn { air_group, .. }
        | EventKind::AircraftDitched { air_group, .. }
        | EventKind::AircraftDitchedNoDeck { air_group, .. }
        | EventKind::AircraftLandedElsewhere {
            from: air_group, ..
        }
        | EventKind::AircraftRepaired { air_group, .. }
        | EventKind::AirGroupLost { air_group, .. }
        | EventKind::StrikeLaunched { air_group, .. }
        | EventKind::StrikeFoundNothing { air_group, .. }
        | EventKind::EscortsSent { air_group, .. }
        | EventKind::AircraftLost { air_group, .. }
        | EventKind::PilotsLost { air_group, .. }
        | EventKind::ReplacementsReceived { air_group, .. }
        | EventKind::AircraftArrived { air_group, .. }
        | EventKind::BaseStrikeLaunched { air_group, .. }
        | EventKind::AircraftDestroyedOnGround { air_group, .. }
        | EventKind::AirGroupFatigue { air_group, .. }
        | EventKind::AirGroupStoodDown { air_group, .. }
        | EventKind::AirGroupReturned { air_group, .. } => whole_if(own_group(air_group)),

        // Bases.
        EventKind::BaseSupplyDestroyed { base, .. }
        | EventKind::BaseSupplySpent { base, .. }
        // What its own base held at the end of the day: nothing it did not
        // know (decision 0067).
        | EventKind::BaseAtDayEnd { base, .. } => {
            whole_if(own_base(base))
        }

        // Land units (decision 0066, 23): seen whole by their own side and
        // not at all by the enemy. The side that sank a ship carrying men is
        // not told she carried them, and a base formed by a landing is
        // known to its own side only (decision 12).
        EventKind::MenEmbarked { unit, .. }
        | EventKind::MenLanded { unit, .. }
        | EventKind::UnitDivided { from: unit, .. }
        | EventKind::MenLostWithShip { unit, .. }
        | EventKind::MenLostUnfed { unit, .. }
        | EventKind::LandUnitDestroyed { unit } => whole_if(own_unit(unit)),
        EventKind::UnitLoadingComplete { task_force, .. } => whole_if(own_force(task_force)),
        EventKind::BaseFormed { base, .. } => whole_if(own_base(base)),
        EventKind::MenFed { side: owner, .. } => whole_if(*owner == side),

        // A side's own: the report, the claim, what became of its contacts.
        EventKind::ReportReceived { side: owner, .. }
        | EventKind::ContactAged { side: owner, .. }
        | EventKind::ContactDropped { side: owner, .. }
        | EventKind::ClaimReceived { side: owner, .. }
        | EventKind::ContactNotThere { side: owner, .. } => whole_if(*owner == side),

        // For a side that has come to know of the fight, with its own force
        // of the two and without the enemy's.
        EventKind::EngagementBegun {
            a,
            b,
            hex,
            elapsed_hours,
        } => cut_if(
            knows_fight,
            SideEventKind::EngagementBegun {
                task_force: if own_force(a) { *a } else { *b },
                hex: *hex,
                elapsed_hours: *elapsed_hours,
            },
        ),
        EventKind::EngagementEnded {
            round,
            a,
            b,
            hex,
            reason,
        } => cut_if(
            knows_fight,
            SideEventKind::EngagementEnded {
                round: *round,
                task_force: if own_force(a) { *a } else { *b },
                hex: *hex,
                // The truth's reason said without its enemy part; `project`
                // gives the side the reasons that were its own.
                reason: match reason {
                    EngagementEndReason::SlippedAway => SideEndReason::SlippedAway,
                    EngagementEndReason::PassedUnseen => SideEndReason::PassedUnseen,
                    EngagementEndReason::MaxRounds => SideEndReason::MaxRounds,
                    EngagementEndReason::ForceSunk | EngagementEndReason::BrokeOff => {
                        SideEndReason::ContactLost
                    }
                },
            },
        ),

        // Its own roll if it sighted, without the force it sighted. A roll
        // that did not is hidden, for either side: it would say there was
        // something to be found.
        EventKind::SightingRoll {
            round,
            task_force,
            chance,
            fatigue_factor,
            sighted,
            ..
        } => cut_if(
            *sighted && own_force(task_force),
            SideEventKind::SightingRoll {
                round: *round,
                task_force: *task_force,
                chance: *chance,
                fatigue_factor: *fatigue_factor,
            },
        ),

        // Its own firing, without the hits scored.
        EventKind::Salvo {
            round,
            firer,
            target,
            weapon,
            shots,
            ..
        } => {
            if own_ship(firer) {
                Seen::Cut(SideEventKind::Salvo {
                    round: *round,
                    firer: *firer,
                    weapon: *weapon,
                    shots: *shots,
                })
            } else {
                // Fired at: it knows it was shot at, and with what.
                cut_if(
                    own_ship(target),
                    SideEventKind::SalvoAgainst {
                        round: *round,
                        target: *target,
                        weapon: *weapon,
                    },
                )
            }
        }
        EventKind::AirAttack {
            air_group,
            task_force,
            ship,
            weapon,
            shots,
            ..
        } => {
            if own_group(air_group) {
                Seen::Cut(SideEventKind::AirAttack {
                    air_group: *air_group,
                    weapon: *weapon,
                    shots: *shots,
                })
            } else {
                // The ship attacked: it can count the aircraft.
                cut_if(
                    own_ship(ship),
                    SideEventKind::AirAttackAgainst {
                        task_force: *task_force,
                        ship: *ship,
                        shots: *shots,
                    },
                )
            }
        }
        EventKind::AirFight {
            firer,
            firing,
            at,
            shots,
            after_attack,
            ..
        } => cut_if(
            own_group(firer),
            SideEventKind::AirFight {
                firer: *firer,
                firing: *firing,
                at: *at,
                shots: *shots,
                after_attack: *after_attack,
            },
        ),
        EventKind::AntiAircraftFire {
            task_force,
            shots,
            after_attack,
            ..
        } => cut_if(
            own_force(task_force),
            SideEventKind::AntiAircraftFire {
                task_force: *task_force,
                shots: *shots,
                after_attack: *after_attack,
            },
        ),
        EventKind::BaseAntiAircraftFire {
            base,
            shots,
            after_attack,
            ..
        } => cut_if(
            own_base(base),
            SideEventKind::BaseAntiAircraftFire {
                base: *base,
                shots: *shots,
                after_attack: *after_attack,
            },
        ),

        // The attacker sees that it attacked; the defender sees the attack,
        // without the air group that made it.
        EventKind::BaseAttacked {
            air_group,
            base,
            shots,
            chance,
            expected,
            hits,
            hits_on_aircraft,
            hits_on_supply,
        } => {
            if own_base(base) {
                Seen::Cut(SideEventKind::BaseAttackedAgainst {
                    base: *base,
                    shots: *shots,
                    chance: *chance,
                    expected: *expected,
                    hits: *hits,
                    hits_on_aircraft: *hits_on_aircraft,
                    hits_on_supply: *hits_on_supply,
                })
            } else {
                cut_if(
                    own_group(air_group),
                    SideEventKind::BaseAttacked {
                        air_group: *air_group,
                        base: *base,
                        shots: *shots,
                    },
                )
            }
        }

        // Nothing: a pass in the dark.
        EventKind::ForcesPassedUnseen { .. } => Seen::Nothing,

        // The searcher's own act if it sighted, cut: the distance and the
        // chance are worked from the force's true position, and the report
        // may name a neighbouring hex (decision 0061). An attempt that did
        // not sight is hidden from everyone.
        EventKind::SearchAttempt {
            air_group, sighted, ..
        } => cut_if(
            *sighted && own_group(air_group),
            SideEventKind::SearchAttempt {
                air_group: *air_group,
            },
        ),
    }
}

/// The names of the factors of a cut event's breakdown that are the side's
/// own. Every other factor is the target's, a rule's or a mixture of both
/// sides' counts, and is dropped.
fn own_factors(kind: &SideEventKind) -> &'static [&'static str] {
    match kind {
        // Whole, or cut of nothing but the enemy's name: `side_breakdown`
        // gives these their whole breakdown and never asks.
        SideEventKind::Whole(_)
        | SideEventKind::BaseAttackedAgainst { .. }
        | SideEventKind::EngagementBegun { .. }
        | SideEventKind::EngagementEnded { .. }
        | SideEventKind::SightingRoll { .. }
        | SideEventKind::ForceSurprised { .. }
        | SideEventKind::ForceSlippedAway { .. }
        | SideEventKind::AlarmRoll { .. }
        | SideEventKind::SalvoAgainst { .. }
        | SideEventKind::AirAttackAgainst { .. } => &[],
        SideEventKind::Salvo { .. } => &[
            "guns",
            "tubes",
            "rate",
            "minutes",
            "main_battery_factor",
            // The firing force's own readiness (decision 0068, 9): the share
            // of its guns it fired, because it was not ready. The side fired
            // on is given no breakdown at all.
            "unready_gun_share",
            "torpedo_mount_factor",
            // The same for its torpedoes (decision 0071).
            "unready_torpedo_share",
            "fire_control_factor",
            // The firing ship's own experience (decision 0064, 12). The
            // target's side is given no breakdown at all.
            "experience_factor",
        ],
        SideEventKind::AirAttack { .. } | SideEventKind::BaseAttacked { .. } => &["aircraft"],
        SideEventKind::SearchAttempt { .. } => &["aircraft", "sweep_width_nm", "arc_deg"],
        SideEventKind::AirFight { .. }
        | SideEventKind::AntiAircraftFire { .. }
        | SideEventKind::BaseAntiAircraftFire { .. } => &["shots"],
    }
}

/// True for the cut events that lack only the name of an enemy unit, and a
/// flag that is always true for an event the side is given (decision 0065).
/// Every quantity of the event is still shown, so its breakdown is shown
/// whole: a factor is a name and a number and names no unit.
fn only_a_name_is_cut(kind: &SideEventKind) -> bool {
    matches!(
        kind,
        SideEventKind::BaseAttackedAgainst { .. }
            | SideEventKind::EngagementBegun { .. }
            | SideEventKind::EngagementEnded { .. }
            | SideEventKind::SightingRoll { .. }
            | SideEventKind::ForceSurprised { .. }
            | SideEventKind::ForceSlippedAway { .. }
            | SideEventKind::AlarmRoll { .. }
    )
}

fn side_breakdown(kind: &SideEventKind, breakdown: &Option<Breakdown>) -> Option<SideBreakdown> {
    let breakdown = breakdown.as_ref()?;
    // A claim is seen whole, and its breakdown not at all (decision 0060).
    // Every factor of it is the truth the claim hides or gives that truth
    // away: the real counts are the enemy's real losses, the false counts are
    // the claim less the real ones, and the misses are the shots less the
    // real hits. The result is the claim's own counts added, which the event
    // already carries. The breakdown is the umpire's.
    if matches!(kind, SideEventKind::Whole(EventKind::ClaimReceived { .. })) {
        return None;
    }
    // A raw random draw is never given to a side (decision 0085, C): it is
    // the die, not a factor, and with the chance it says which side of the
    // roll the die fell on. Today only the alarm roll records one. The
    // umpire's breakdown keeps it.
    let given = |factor: &&Factor| factor.name != "draw";
    if matches!(kind, SideEventKind::Whole(_)) || only_a_name_is_cut(kind) {
        return Some(SideBreakdown {
            factors: breakdown.factors.iter().filter(given).copied().collect(),
            result: Some(breakdown.result),
        });
    }
    let own = own_factors(kind);
    let factors: Vec<Factor> = breakdown
        .factors
        .iter()
        .filter(given)
        .filter(|factor| own.contains(&factor.name))
        .copied()
        .collect();
    if factors.is_empty() {
        None
    } else {
        Some(SideBreakdown {
            factors,
            result: None,
        })
    }
}

/// The round in which `side`'s force in a fight first sighted the enemy or
/// was first fired on, from the fight's own events, or `None` if it never
/// did. `fight` is the events from after `EngagementBegun` on.
fn first_known_round(side: Side, fight: &[Event], owners: &Owners) -> Option<u32> {
    let own_ship = |id: &ShipId| owners.ships.get(id) == Some(&side);
    let own_force = |id: &TaskForceId| owners.forces.get(id) == Some(&side);
    fight
        .iter()
        .filter_map(|event| match &event.kind {
            // It sighted the enemy by a roll.
            EventKind::SightingRoll {
                round,
                task_force,
                sighted: true,
                ..
            } if own_force(task_force) => Some(*round),
            // It was fired on and had not sighted, or it sighted and left.
            EventKind::ForceSurprised {
                round, task_force, ..
            }
            | EventKind::ForceSlippedAway {
                round, task_force, ..
            } if own_force(task_force) => Some(*round),
            // It fired, so it had sighted; or it was fired at.
            EventKind::Salvo {
                round,
                firer,
                target,
                ..
            } if own_ship(firer) || own_ship(target) => Some(*round),
            // Hits on its ships, whatever put them there.
            EventKind::ShipDamaged { round, ship, .. }
            | EventKind::SystemHit { round, ship, .. }
            | EventKind::ShipSunk { round, ship, .. }
                if *round > 0 && own_ship(ship) =>
            {
                Some(*round)
            }
            _ => None,
        })
        .min()
}

/// Where an event sits in relation to a surface fight, for one side.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Gate {
    /// Not between a fight's beginning and its end.
    Outside,
    /// In a fight, and the side first knew of it in this round, if it ever
    /// did.
    Inside(Option<u32>),
}

/// For each event of the log, whether it is in a fight and, if so, the round
/// in which the side first knew of that fight.
fn fight_gates(side: Side, log: &[Event], owners: &Owners) -> Vec<Gate> {
    let mut gates = vec![Gate::Outside; log.len()];
    let mut index = 0;
    while index < log.len() {
        let EventKind::EngagementBegun { a, b, .. } = &log[index].kind else {
            index += 1;
            continue;
        };
        let (a, b) = (*a, *b);
        // The fight runs to its own end, or to the end of the log if the log
        // stops in the middle of it.
        let last = log[index + 1..]
            .iter()
            .position(|event| {
                matches!(&event.kind, EventKind::EngagementEnded { a: x, b: y, .. }
                    if *x == a && *y == b)
            })
            .map_or(log.len() - 1, |offset| index + 1 + offset);
        let known = first_known_round(side, &log[index + 1..=last.max(index)], owners);
        for gate in &mut gates[index..=last] {
            *gate = Gate::Inside(known);
        }
        index = last + 1;
    }
    gates
}

/// The round a fight event carries, for the kinds a fight fires.
fn fight_round(kind: &EventKind) -> Option<u32> {
    match kind {
        EventKind::SightingRoll { round, .. }
        | EventKind::ForceSurprised { round, .. }
        | EventKind::Salvo { round, .. }
        | EventKind::ForceBecameAlert { round, .. }
        | EventKind::ForceBrokeOff { round, .. }
        | EventKind::ForceSlippedAway { round, .. } => Some(*round),
        // Hits outside a fight carry round 0.
        EventKind::ShipDamaged { round, .. }
        | EventKind::SystemHit { round, .. }
        | EventKind::ShipSunk { round, .. }
            if *round > 0 =>
        {
            Some(*round)
        }
        _ => None,
    }
}

/// The ending a side is told of a fight whose truth ending is `reason`, which
/// ended at `log[index]` (decision 0085, F5). The enemy's doings are not the
/// side's to know: a force sunk or broken off is told only if it was the
/// side's own, and otherwise as contact lost (already the reason in `seen`).
///
/// - `ForceSunk` is the side's only if its own force was emptied in this
///   fight: that force has no ship now (an emptied force is never refilled)
///   and the latest `ShipSunk` of it in the whole log lies inside this fight.
/// - `BrokeOff` is the side's only if its own force broke off in the round
///   the fight ended: a `ForceBrokeOff` of it in this fight in that round
///   (the truth ending's causes are the `ForceBrokeOff` events of the forces
///   that broke off, and the side's own is the one it may see).
fn own_ending(
    side: Side,
    log: &[Event],
    index: usize,
    owners: &Owners,
    told: SideEndReason,
) -> SideEndReason {
    let EventKind::EngagementEnded {
        round,
        a,
        b,
        reason,
        ..
    } = &log[index].kind
    else {
        return told;
    };
    let own = [*a, *b]
        .into_iter()
        .find(|id| owners.forces.get(id) == Some(&side));
    let Some(own) = own else {
        return told;
    };
    // The fight's own events: after its `EngagementBegun`, before this end.
    let begun = log[..index].iter().rposition(|event| {
        matches!(&event.kind, EventKind::EngagementBegun { a: x, b: y, .. } if x == a && y == b)
    });
    let Some(begun) = begun else {
        return told;
    };
    match reason {
        EngagementEndReason::ForceSunk => {
            let latest = log.iter().rposition(|event| {
                matches!(&event.kind, EventKind::ShipSunk { task_force, .. } if *task_force == own)
            });
            if owners.emptied_forces.contains(&own)
                && latest.is_some_and(|sunk| sunk > begun && sunk < index)
            {
                SideEndReason::ForceSunk
            } else {
                told
            }
        }
        EngagementEndReason::BrokeOff => {
            let broke = log[begun + 1..index].iter().any(|event| {
                matches!(&event.kind, EventKind::ForceBrokeOff { round: r, task_force, .. }
                    if *task_force == own && r == round)
            });
            if broke {
                SideEndReason::BrokeOff
            } else {
                told
            }
        }
        EngagementEndReason::SlippedAway
        | EngagementEndReason::PassedUnseen
        | EngagementEndReason::MaxRounds => told,
    }
}

/// Counts the rounds of a fight event from the round the side first knew of
/// the fight (`known`), so a side that first knew of a fight in round 3 is
/// told that round as its round 1 and learns nothing of how many rounds it
/// did not know of; and gives `EngagementBegun` the hour of that round
/// rather than the hour the forces met (decision 0085, F6). Every kind a
/// side is given from inside a fight that carries a fight round is here, the
/// cut kinds and the whole ones; a whole kind is altered in its round and
/// nothing else. A round of 0 is a hit outside a fight and is left.
fn count_from_known_round(kind: SideEventKind, known: u32, round_minutes: u32) -> SideEventKind {
    let before = known.saturating_sub(1);
    if before == 0 {
        return kind;
    }
    let count = |round: u32| round.saturating_sub(before);
    match kind {
        SideEventKind::Salvo {
            round,
            firer,
            weapon,
            shots,
        } => SideEventKind::Salvo {
            round: count(round),
            firer,
            weapon,
            shots,
        },
        SideEventKind::SalvoAgainst {
            round,
            target,
            weapon,
        } => SideEventKind::SalvoAgainst {
            round: count(round),
            target,
            weapon,
        },
        SideEventKind::EngagementBegun {
            task_force,
            hex,
            elapsed_hours,
        } => SideEventKind::EngagementBegun {
            task_force,
            hex,
            elapsed_hours: elapsed_hours
                + Fixed::from_ratio(i64::from(round_minutes) * i64::from(before), 60),
        },
        SideEventKind::EngagementEnded {
            round,
            task_force,
            hex,
            reason,
        } => SideEventKind::EngagementEnded {
            round: count(round),
            task_force,
            hex,
            reason,
        },
        SideEventKind::SightingRoll {
            round,
            task_force,
            chance,
            fatigue_factor,
        } => SideEventKind::SightingRoll {
            round: count(round),
            task_force,
            chance,
            fatigue_factor,
        },
        SideEventKind::ForceSurprised { round, task_force } => SideEventKind::ForceSurprised {
            round: count(round),
            task_force,
        },
        SideEventKind::ForceSlippedAway { round, task_force } => SideEventKind::ForceSlippedAway {
            round: count(round),
            task_force,
        },
        SideEventKind::Whole(whole) => SideEventKind::Whole(match whole {
            EventKind::ShipDamaged {
                round,
                ship,
                track,
                hits,
                points,
                value_after,
            } if round > 0 => EventKind::ShipDamaged {
                round: count(round),
                ship,
                track,
                hits,
                points,
                value_after,
            },
            EventKind::SystemHit {
                round,
                ship,
                system,
                before,
                after,
            } if round > 0 => EventKind::SystemHit {
                round: count(round),
                ship,
                system,
                before,
                after,
            },
            EventKind::ShipSunk {
                round,
                ship,
                task_force,
            } if round > 0 => EventKind::ShipSunk {
                round: count(round),
                ship,
                task_force,
            },
            EventKind::ForceBecameAlert { round, task_force } => EventKind::ForceBecameAlert {
                round: count(round),
                task_force,
            },
            EventKind::ForceBrokeOff {
                round,
                task_force,
                share,
            } => EventKind::ForceBrokeOff {
                round: count(round),
                task_force,
                share,
            },
            other => other,
        }),
        // No other kind a side is given carries a round of a fight.
        other @ (SideEventKind::AirAttackAgainst { .. }
        | SideEventKind::AirAttack { .. }
        | SideEventKind::AirFight { .. }
        | SideEventKind::AntiAircraftFire { .. }
        | SideEventKind::BaseAntiAircraftFire { .. }
        | SideEventKind::BaseAttacked { .. }
        | SideEventKind::BaseAttackedAgainst { .. }
        | SideEventKind::SearchAttempt { .. }
        | SideEventKind::AlarmRoll { .. }) => other,
    }
}

/// Whether an event of this kind, as the side is given it, can have causes
/// the side may not see: an order of the enemy's, or an event of the enemy's
/// or one the side is not given (decision 0085, F7). The flag is set by the
/// kind and never by an event, so it cannot say whether the enemy gave an
/// order or did anything: an enemy order that changes nothing changes nothing
/// the side is given.
///
/// One exhaustive `match`, over the side's kinds and, for a whole event,
/// over every kind of the log, so a new kind does not compile until it is
/// decided. Each answer is read from where the code records the kind's
/// causes. A kind is `true` if any cause it can carry may be one the side is
/// not given; a kind that cites only the side's own orders and events it is
/// given is `false`.
fn may_have_hidden_causes(kind: &SideEventKind) -> bool {
    match kind {
        SideEventKind::Whole(whole) => whole_may_have_hidden_causes(whole),
        // Salvos, sighting and a slip cite the fight's beginning, which the
        // side that knows of the fight is given, and the side's own roll and
        // order. An own air attack on a ship or a base cites its own launch
        // and its own losses. A search that sighted cites its own flight.
        SideEventKind::Salvo { .. }
        | SideEventKind::SalvoAgainst { .. }
        | SideEventKind::SightingRoll { .. }
        | SideEventKind::ForceSlippedAway { .. }
        | SideEventKind::AirAttack { .. }
        | SideEventKind::BaseAttacked { .. }
        | SideEventKind::SearchAttempt { .. } => false,
        // Surprised: cites the enemy's sighting roll. A fight's beginning:
        // the steaming that brought the enemy in, and both sides' orders. Its
        // end: the enemy's sinking, break-off or slipping away. An alarm: the
        // end of a fight the side may not know of. A raid on the side, and
        // the side's fire at one: the enemy's launch (and escorts).
        SideEventKind::ForceSurprised { .. }
        | SideEventKind::EngagementBegun { .. }
        | SideEventKind::EngagementEnded { .. }
        | SideEventKind::AlarmRoll { .. }
        | SideEventKind::AirAttackAgainst { .. }
        | SideEventKind::BaseAttackedAgainst { .. }
        | SideEventKind::AirFight { .. }
        | SideEventKind::AntiAircraftFire { .. }
        | SideEventKind::BaseAntiAircraftFire { .. } => true,
    }
}

/// [`may_have_hidden_causes`] for an event the side is given whole.
fn whole_may_have_hidden_causes(kind: &EventKind) -> bool {
    match kind {
        // Aircraft lost to fire: the cause is the roll that shot them down,
        // the enemy's fighters or guns unless the side's own fighters fired
        // it, so it may be one the side is not given.
        EventKind::AircraftLost { .. } => true,

        // These cite only the side's own orders and its own events, or events
        // the side is given: a steaming, a search, a report, a launch, a
        // fatigue, and a hit on its own ship (whose cause is the salvo or air
        // attack on that ship, which the side is given cut).
        EventKind::OrderAccepted { .. }
        | EventKind::OrderRejected { .. }
        | EventKind::TaskForceSteamed { .. }
        | EventKind::FuelBurned { .. }
        | EventKind::TaskForceArrived { .. }
        | EventKind::TaskForceOutOfFuel { .. }
        | EventKind::CargoLoaded { .. }
        | EventKind::CargoUnloaded { .. }
        | EventKind::LoadingComplete { .. }
        | EventKind::UnloadingComplete { .. }
        | EventKind::SearchFlown { .. }
        | EventKind::ReportReceived { .. }
        | EventKind::ContactAged { .. }
        | EventKind::ContactDropped { .. }
        | EventKind::ShipDamaged { .. }
        | EventKind::ShipSunk { .. }
        | EventKind::ForceBecameAlert { .. }
        | EventKind::ForceBrokeOff { .. }
        | EventKind::ClaimReceived { .. }
        | EventKind::SystemHit { .. }
        | EventKind::ShipsDetached { .. }
        | EventKind::ForceRetired { .. }
        | EventKind::ForceAlertedByContact { .. }
        | EventKind::ForceStoodDown { .. }
        | EventKind::DamagedAircraftDestroyedOnGround { .. }
        | EventKind::AircraftWorn { .. }
        | EventKind::AircraftRepaired { .. }
        | EventKind::AirGroupLost { .. }
        | EventKind::StrikeLaunched { .. }
        | EventKind::StrikeFoundNothing { .. }
        | EventKind::EscortsSent { .. }
        | EventKind::PilotsLost { .. }
        | EventKind::ReplacementsReceived { .. }
        | EventKind::AircraftArrived { .. }
        | EventKind::ContactNotThere { .. }
        | EventKind::BaseStrikeLaunched { .. }
        | EventKind::AircraftDestroyedOnGround { .. }
        | EventKind::BaseSupplyDestroyed { .. }
        | EventKind::BaseSupplySpent { .. }
        | EventKind::AirGroupFatigue { .. }
        | EventKind::AirGroupStoodDown { .. }
        | EventKind::AirGroupReturned { .. }
        | EventKind::TaskForceFatigue { .. }
        | EventKind::TaskForceTurnedBack { .. }
        | EventKind::ShipRefuelled { .. }
        | EventKind::ShipRepaired { .. }
        | EventKind::ForceLeftForYard { .. }
        | EventKind::ForceReturnedFromYard { .. }
        | EventKind::AircraftDitched { .. }
        | EventKind::AircraftLandedElsewhere { .. }
        | EventKind::AircraftDitchedNoDeck { .. }
        | EventKind::BaseAtDayEnd { .. }
        // Land units cite their own orders, the arrival of their own force,
        // the sinking of their own ship and their own feeding (decision
        // 0066).
        | EventKind::MenEmbarked { .. }
        | EventKind::MenLanded { .. }
        | EventKind::BaseFormed { .. }
        | EventKind::UnitDivided { .. }
        | EventKind::MenLostWithShip { .. }
        | EventKind::MenFed { .. }
        | EventKind::MenLostUnfed { .. }
        | EventKind::LandUnitDestroyed { .. }
        | EventKind::UnitLoadingComplete { .. } => false,

        // Never given whole: each of these is cut (and decided in
        // `may_have_hidden_causes`) or hidden from every side. If one were
        // ever given whole, it is flagged.
        EventKind::EngagementBegun { .. }
        | EventKind::SightingRoll { .. }
        | EventKind::ForceSurprised { .. }
        | EventKind::Salvo { .. }
        | EventKind::ForceSlippedAway { .. }
        | EventKind::ForcesPassedUnseen { .. }
        | EventKind::EngagementEnded { .. }
        | EventKind::SearchAttempt { .. }
        | EventKind::AlarmRoll { .. }
        | EventKind::AirAttack { .. }
        | EventKind::AirFight { .. }
        | EventKind::AntiAircraftFire { .. }
        | EventKind::BaseAntiAircraftFire { .. }
        | EventKind::BaseAttacked { .. } => true,
    }
}

/// The side's number of each of its own orders: its place among the side's
/// orders in the order book's order, from 1 (decision 0086).
pub(crate) fn order_numbers(side: Side, owners: &Owners) -> BTreeMap<OrderId, u32> {
    owners
        .orders
        .iter()
        .filter(|(_, owner)| **owner == side)
        .zip(1u32..)
        .map(|((order, _), number)| (*order, number))
        .collect()
}

/// The events of `log` a side may see, each cut down to what it may know, in
/// log order.
pub(crate) fn project(side: Side, log: &[Event], owners: &Owners) -> Vec<SideEvent> {
    let gates = fight_gates(side, log, owners);
    let seen: Vec<Seen> = log
        .iter()
        .zip(&gates)
        .map(|(event, gate)| {
            let knows_fight = matches!(gate, Gate::Inside(Some(_)));
            let seen = seen(&event.kind, side, owners, knows_fight);
            // Decision 5: a fight is seen from the round the side first knew
            // of it. An event of the fight from an earlier round, or of a
            // fight it never knew of, is not (a force can break off before
            // it has sighted or been fired on).
            match (gate, fight_round(&event.kind)) {
                (Gate::Inside(known), Some(round)) if known.is_none_or(|k| round < k) => {
                    Seen::Nothing
                }
                _ => seen,
            }
        })
        .collect();
    // The side's own numbers: its events counted from 1 in log order, and its
    // orders counted from 1 in the order book's order (decision 0086).
    let mut event_numbers: BTreeMap<EventId, u32> = BTreeMap::new();
    for (event, _) in log
        .iter()
        .zip(&seen)
        .filter(|(_, seen)| **seen != Seen::Nothing)
    {
        let number = u32::try_from(event_numbers.len() + 1).expect("ran out of event numbers");
        event_numbers.insert(event.id, number);
    }
    let order_numbers = order_numbers(side, owners);
    log.iter()
        .zip(seen)
        .zip(&gates)
        .enumerate()
        .filter_map(|(index, ((event, seen), gate))| {
            let mut kind = match seen {
                Seen::Nothing => return None,
                Seen::Whole => SideEventKind::Whole(event.kind.clone()),
                Seen::Cut(kind) => kind,
            };
            if let SideEventKind::EngagementEnded { reason, .. } = &mut kind {
                *reason = own_ending(side, log, index, owners, *reason);
            }
            // Decision 0085, F6: a fight is counted from the round the side
            // first knew of it.
            if let Gate::Inside(Some(known)) = gate {
                kind = count_from_known_round(kind, *known, owners.round_minutes);
            }
            let causes: Vec<SideCause> = event
                .causes
                .iter()
                .filter_map(|cause| match cause {
                    Cause::Order(order) => order_numbers.get(order).copied().map(SideCause::Order),
                    Cause::Event(earlier) => {
                        event_numbers.get(earlier).copied().map(SideCause::Event)
                    }
                })
                .collect();
            let order_number = match &kind {
                SideEventKind::Whole(
                    EventKind::OrderAccepted { order, .. } | EventKind::OrderRejected { order, .. },
                ) => order_numbers.get(order).copied(),
                _ => None,
            };
            let hidden_causes = may_have_hidden_causes(&kind);
            let breakdown = side_breakdown(&kind, &event.breakdown);
            Some(SideEvent {
                id: event.id,
                number: event_numbers[&event.id],
                day: event.day,
                pulse: event.pulse,
                kind,
                order_number,
                causes,
                hidden_causes,
                breakdown,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{
        DamageTrack, DetachReason, EngagementEndReason, FatigueChange, ForceFatigueChange,
        SortieKind, SupplyUse,
    };
    use crate::hex::Hex;
    use crate::ids::ContactId;
    use crate::orders::{Cargo, RejectReason};
    use crate::refdata::ShipKind;
    use crate::state::{AirGroupLocation, ShipSystem, SystemState};

    fn ship(n: u32) -> ShipId {
        ShipId::new(n)
    }
    fn force(n: u32) -> TaskForceId {
        TaskForceId::new(n)
    }
    fn group(n: u32) -> AirGroupId {
        AirGroupId::new(n)
    }
    fn base(n: u32) -> BaseId {
        BaseId::new(n)
    }
    fn order(n: u32) -> OrderId {
        OrderId::new(n)
    }
    fn hex() -> Hex {
        Hex::new(3, -2)
    }
    fn fx(n: i64) -> Fixed {
        Fixed::from_int(n)
    }

    /// Unit 1 of each kind is the Allied side's and unit 2 the Japanese
    /// side's. In the mirror the sides are swapped.
    fn owners(mirror: bool) -> Owners {
        let (one, two) = if mirror {
            (Side::Japanese, Side::Allied)
        } else {
            (Side::Allied, Side::Japanese)
        };
        let mut owners = Owners {
            // The reference data's round (decision 0085, F6): the hand-built
            // logs below have no reference data to read it from.
            round_minutes: 5,
            ..Owners::default()
        };
        for (n, side) in [(1, one), (2, two)] {
            owners.ships.insert(ship(n), side);
            owners.forces.insert(force(n), side);
            owners.air_groups.insert(group(n), side);
            owners.bases.insert(base(n), side);
            owners.land_units.insert(unit(n), side);
            owners.orders.insert(order(n), side);
        }
        owners
    }

    fn unit(n: u32) -> LandUnitId {
        LandUnitId::new(n)
    }

    fn log(kinds: Vec<EventKind>) -> Vec<Event> {
        kinds
            .into_iter()
            .enumerate()
            .map(|(index, kind)| Event {
                id: EventId::new(u32::try_from(index + 1).expect("few events")),
                day: 1,
                pulse: Some(Pulse::Night),
                kind,
                causes: Vec::new(),
                breakdown: None,
            })
            .collect()
    }

    // ---- the table of the 76 kinds ----

    /// What a side sees of an event.
    #[derive(Clone, Copy, Debug)]
    enum Expect {
        Whole,
        Nothing,
        /// The cut variant's name and the fields of the full kind it lacks.
        Cut(&'static str, &'static [&'static str]),
        /// The same, for the two kinds whose cut form holds the side's own
        /// force of the log's `a` and `b` in the one field `task_force`.
        CutOwnForce(&'static str, &'static [&'static str]),
    }
    use Expect::{Cut, CutOwnForce, Nothing, Whole};

    struct Row {
        /// The events the example sits in, and which of them it is.
        log: Vec<EventKind>,
        at: usize,
        allied: Expect,
        japanese: Expect,
        /// False for a kind that names a side, which a mirror does not swap.
        mirrors: bool,
    }

    fn sided(mut row: Row) -> Row {
        row.mirrors = false;
        row
    }

    fn row(kind: EventKind, allied: Expect, japanese: Expect) -> Row {
        Row {
            log: vec![kind],
            at: 0,
            allied,
            japanese,
            mirrors: true,
        }
    }

    /// A fight both sides know of: the Allied ship fires at the Japanese
    /// one in round 1.
    fn known_fight() -> Vec<EventKind> {
        vec![
            EventKind::EngagementBegun {
                a: force(1),
                b: force(2),
                hex: hex(),
                elapsed_hours: fx(3),
            },
            EventKind::Salvo {
                round: 1,
                firer: ship(1),
                target: ship(2),
                weapon: Weapon::Gun,
                shots: fx(120),
                chance: Fixed::from_ratio(1, 20),
                expected: fx(6),
                hits: 5,
            },
            EventKind::EngagementEnded {
                round: 1,
                a: force(1),
                b: force(2),
                hex: hex(),
                reason: EngagementEndReason::MaxRounds,
            },
        ]
    }

    /// The firer's cut of an event that also names the enemy's air group.
    const GUNS_CUT: &[&str] = &["air_group", "chance", "expected", "hits"];

    /// One row for every kind of event, and the order and fight they need.
    /// An Allied unit is the first of each kind and a Japanese unit the
    /// second. Kinds with more than one row are the cases that differ.
    #[allow(clippy::too_many_lines)]
    fn table() -> Vec<Row> {
        let allied_report = |side| EventKind::ReportReceived {
            side,
            contact: ContactId::new(1),
            hex: hex(),
            ship_count: 2,
            kind: ShipKind::Cruiser,
            created: true,
        };
        let mut rows = vec![
            row(
                EventKind::OrderAccepted {
                    order: order(1),
                    target: EntityRef::Ship(ship(1)),
                },
                Whole,
                Nothing,
            ),
            // The Japanese side ordered an Allied ship. The order is its
            // issuer's, whatever it targeted.
            row(
                EventKind::OrderRejected {
                    order: order(2),
                    target: EntityRef::Ship(ship(1)),
                    reason: RejectReason::NotYourUnit,
                },
                Nothing,
                Whole,
            ),
            row(
                EventKind::TaskForceSteamed {
                    task_force: force(1),
                    from: hex(),
                    to: Hex::new(4, -2),
                    distance_nm: fx(20),
                    speed_kn: fx(10),
                    hexes_entered: vec![Hex::new(4, -2)],
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::FuelBurned {
                    ship: ship(1),
                    task_force: force(1),
                    burned_t: fx(5),
                    remaining_t: fx(95),
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::TaskForceArrived {
                    task_force: force(1),
                    hex: hex(),
                    elapsed_hours: fx(7),
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::TaskForceOutOfFuel {
                    task_force: force(1),
                    hex: hex(),
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::CargoLoaded {
                    ship: ship(1),
                    task_force: force(1),
                    base: base(1),
                    cargo: Cargo::Supply,
                    loaded_t: fx(10),
                    aboard_t: fx(10),
                    base_stock_t: fx(90),
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::CargoUnloaded {
                    ship: ship(1),
                    task_force: force(1),
                    base: base(1),
                    cargo: Cargo::Fuel,
                    unloaded_t: fx(10),
                    aboard_t: fx(0),
                    base_stock_t: fx(100),
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::LoadingComplete {
                    task_force: force(1),
                    base: base(1),
                    cargo: Cargo::Supply,
                    elapsed_hours: fx(5),
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::UnloadingComplete {
                    task_force: force(1),
                    base: base(1),
                    elapsed_hours: fx(5),
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::SearchFlown {
                    air_group: group(1),
                    from: AirGroupLocation::Base(base(1)),
                    hex: hex(),
                    from_deg: 0,
                    to_deg: 90,
                    range_nm: fx(300),
                    aircraft: 6,
                },
                Whole,
                Nothing,
            ),
            // A search attempt that sighted is the searcher's own act, cut
            // of the distance and the chance (decision 0061) and of the force
            // (decision 0065); one that did not is hidden from everyone.
            row(
                EventKind::SearchAttempt {
                    air_group: group(1),
                    task_force: force(2),
                    distance_nm: fx(200),
                    chance: Fixed::from_ratio(1, 2),
                    sighted: true,
                },
                Cut(
                    "SearchAttempt",
                    &["task_force", "distance_nm", "chance", "sighted"],
                ),
                Nothing,
            ),
            row(
                EventKind::SearchAttempt {
                    air_group: group(1),
                    task_force: force(2),
                    distance_nm: fx(200),
                    chance: Fixed::from_ratio(1, 2),
                    sighted: false,
                },
                Nothing,
                Nothing,
            ),
            sided(row(allied_report(Side::Allied), Whole, Nothing)),
            sided(row(
                EventKind::ContactAged {
                    side: Side::Japanese,
                    contact: ContactId::new(1),
                    age_pulses: 2,
                },
                Nothing,
                Whole,
            )),
            sided(row(
                EventKind::ContactDropped {
                    side: Side::Allied,
                    contact: ContactId::new(1),
                    age_pulses: 8,
                },
                Whole,
                Nothing,
            )),
            // The roll of the side's own force that sighted. The enemy's roll
            // that sighted, and any roll that did not, are hidden.
            row(
                EventKind::SightingRoll {
                    round: 1,
                    task_force: force(1),
                    enemy: force(2),
                    chance: Fixed::from_ratio(1, 2),
                    fatigue_factor: Fixed::ONE,
                    sighted: true,
                },
                Cut("SightingRoll", &["enemy", "sighted"]),
                Nothing,
            ),
            row(
                EventKind::SightingRoll {
                    round: 1,
                    task_force: force(1),
                    enemy: force(2),
                    chance: Fixed::from_ratio(1, 2),
                    fatigue_factor: Fixed::ONE,
                    sighted: false,
                },
                Nothing,
                Nothing,
            ),
            // The force surprised, not the one that surprised it.
            row(
                EventKind::ForceSurprised {
                    round: 1,
                    task_force: force(1),
                    enemy: force(2),
                },
                Cut("ForceSurprised", &["enemy"]),
                Nothing,
            ),
            // The firer's own, without the hits scored.
            row(
                EventKind::Salvo {
                    round: 1,
                    firer: ship(1),
                    target: ship(2),
                    weapon: Weapon::Gun,
                    shots: fx(120),
                    chance: Fixed::from_ratio(1, 20),
                    expected: fx(6),
                    hits: 5,
                },
                Cut("Salvo", &["target", "chance", "expected", "hits"]),
                Cut(
                    "SalvoAgainst",
                    &["firer", "shots", "chance", "expected", "hits"],
                ),
            ),
            row(
                EventKind::ShipDamaged {
                    round: 1,
                    ship: ship(1),
                    track: DamageTrack::Flotation,
                    hits: 2,
                    points: fx(10),
                    value_after: fx(10),
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::ShipSunk {
                    round: 1,
                    ship: ship(1),
                    task_force: force(1),
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::ForceBecameAlert {
                    round: 1,
                    task_force: force(1),
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::ForceBrokeOff {
                    round: 1,
                    task_force: force(1),
                    share: Fixed::from_ratio(1, 2),
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::ForceSlippedAway {
                    round: 1,
                    task_force: force(1),
                    enemy: force(2),
                },
                Cut("ForceSlippedAway", &["enemy"]),
                Nothing,
            ),
            row(
                EventKind::ForcesPassedUnseen {
                    round: 4,
                    a: force(1),
                    b: force(2),
                },
                Nothing,
                Nothing,
            ),
            // Begun and ended are shown to a side that has come to know of
            // the fight, each with its own force and without the enemy's
            // (decision 0065). Both did here: one fired and one was fired on
            // in round 1.
            Row {
                log: known_fight(),
                at: 0,
                allied: CutOwnForce("EngagementBegun", &["a", "b"]),
                japanese: CutOwnForce("EngagementBegun", &["a", "b"]),
                mirrors: true,
            },
            Row {
                log: known_fight(),
                at: 2,
                allied: CutOwnForce("EngagementEnded", &["a", "b"]),
                japanese: CutOwnForce("EngagementEnded", &["a", "b"]),
                mirrors: true,
            },
            sided(row(
                EventKind::ClaimReceived {
                    side: Side::Allied,
                    hex: hex(),
                    claimed_sunk: 1,
                    claimed_hits: 0,
                    claimed_kind: None,
                    claimed_shot_down: 0,
                    claimed_on_ground: 0,
                    from_air: false,
                },
                Whole,
                Nothing,
            )),
            row(
                EventKind::SystemHit {
                    round: 1,
                    ship: ship(1),
                    system: ShipSystem::Propulsion,
                    before: SystemState::Working,
                    after: SystemState::Degraded,
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::ShipsDetached {
                    parent: force(1),
                    new_force: force(3),
                    ships: vec![ship(1)],
                    escorts: Vec::new(),
                    reason: DetachReason::Ordered,
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::ForceRetired {
                    task_force: force(1),
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::ForceAlertedByContact {
                    task_force: force(1),
                    contact: ContactId::new(1),
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::ForceStoodDown {
                    task_force: force(1),
                },
                Whole,
                Nothing,
            ),
            // The hearer's own.
            row(
                EventKind::AlarmRoll {
                    task_force: force(1),
                    by_a: force(1),
                    by_b: force(2),
                    chance: Fixed::from_ratio(1, 2),
                    alarmed: true,
                },
                Cut("AlarmRoll", &["by_a", "by_b", "alarmed"]),
                Nothing,
            ),
            // A failed alarm means the force noticed nothing.
            row(
                EventKind::AlarmRoll {
                    task_force: force(1),
                    by_a: force(1),
                    by_b: force(2),
                    chance: Fixed::from_ratio(1, 2),
                    alarmed: false,
                },
                Nothing,
                Nothing,
            ),
            row(
                EventKind::DamagedAircraftDestroyedOnGround {
                    air_group: group(1),
                    base: base(1),
                    aircraft: 2,
                    remaining: 1,
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::AircraftWorn {
                    air_group: group(1),
                    sortie: SortieKind::Search,
                    distance_nm: fx(300),
                    aircraft: 6,
                    chance: Fixed::from_ratio(1, 10),
                    expected: Fixed::from_ratio(3, 5),
                    worn: 1,
                    written_off: 0,
                    ready: 5,
                    damaged: 1,
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::AircraftDitched {
                    air_group: group(1),
                    sortie: SortieKind::Strike,
                    distance_nm: fx(280),
                    radius_nm: fx(250),
                    aircraft: 18,
                    chance: Fixed::from_ratio(24, 100),
                    expected: Fixed::from_ratio(432, 100),
                    ditched: 5,
                    crews_recovered: Some(3),
                    pilots_lost: 2,
                    ready: 13,
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::AircraftLandedElsewhere {
                    from: group(1),
                    to: group(9),
                    at: AirGroupLocation::Base(base(1)),
                    aircraft: 6,
                    pilots: 6,
                    formed: true,
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::AircraftDitchedNoDeck {
                    air_group: group(1),
                    hex: Hex::new(9, 0),
                    aircraft: 10,
                    beside_own_force: true,
                    chance: Fixed::from_ratio(95, 100),
                    crews_recovered: Some(9),
                    pilots_lost: 1,
                    ready: 0,
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::AircraftRepaired {
                    air_group: group(1),
                    at: AirGroupLocation::Base(base(1)),
                    repaired: 1,
                    ready: 6,
                    damaged: 0,
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::AirGroupLost {
                    air_group: group(1),
                    ship: ship(1),
                    aircraft: 12,
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::StrikeLaunched {
                    air_group: group(1),
                    contact: ContactId::new(1),
                    hex: hex(),
                    aircraft: 9,
                    distance_nm: fx(150),
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::StrikeFoundNothing {
                    air_group: group(1),
                    hex: hex(),
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::AirAttack {
                    air_group: group(1),
                    task_force: force(2),
                    ship: ship(2),
                    weapon: Weapon::Bomb,
                    shots: fx(9),
                    chance: Fixed::from_ratio(2, 5),
                    expected: fx(4),
                    hits: 3,
                },
                Cut(
                    "AirAttack",
                    &["task_force", "ship", "chance", "expected", "hits"],
                ),
                Cut(
                    "AirAttackAgainst",
                    &["air_group", "weapon", "chance", "expected", "hits"],
                ),
            ),
            row(
                EventKind::EscortsSent {
                    air_group: group(1),
                    with_air_group: group(1),
                    aircraft: 6,
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::AirFight {
                    firer: group(1),
                    target: group(2),
                    firing: AirFirer::Escorts,
                    at: AirTarget::Bombers,
                    shots: fx(10),
                    chance: Fixed::from_ratio(1, 4),
                    expected: Fixed::from_ratio(5, 2),
                    hits: 2,
                    after_attack: false,
                },
                Cut("AirFight", &["target", "chance", "expected", "hits"]),
                Nothing,
            ),
            row(
                EventKind::AntiAircraftFire {
                    task_force: force(1),
                    air_group: group(2),
                    shots: fx(30),
                    chance: Fixed::from_ratio(1, 10),
                    expected: fx(3),
                    hits: 1,
                    after_attack: false,
                },
                Cut("AntiAircraftFire", GUNS_CUT),
                Nothing,
            ),
            row(
                EventKind::AircraftLost {
                    air_group: group(1),
                    aircraft: 2,
                    remaining: 4,
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::PilotsLost {
                    air_group: group(1),
                    lost: 1,
                    left: 5,
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::ReplacementsReceived {
                    air_group: group(1),
                    base: base(1),
                    aircraft: 2,
                    pilots: 2,
                    ready: 8,
                    pilots_now: 8,
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::AircraftArrived {
                    air_group: group(1),
                    aircraft: 2,
                    pilots: 2,
                    ready: 8,
                    pilots_now: 8,
                    strength: 12,
                },
                Whole,
                Nothing,
            ),
            sided(row(
                EventKind::ContactNotThere {
                    side: Side::Allied,
                    contact: ContactId::new(1),
                    hex: hex(),
                },
                Whole,
                Nothing,
            )),
            row(
                EventKind::BaseStrikeLaunched {
                    air_group: group(1),
                    base: base(2),
                    hex: hex(),
                    aircraft: 9,
                    distance_nm: fx(150),
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::BaseAntiAircraftFire {
                    base: base(1),
                    air_group: group(2),
                    shots: fx(30),
                    chance: Fixed::from_ratio(1, 10),
                    expected: fx(3),
                    hits: 1,
                    after_attack: false,
                },
                Cut("BaseAntiAircraftFire", GUNS_CUT),
                Nothing,
            ),
            // The attacker sees that it attacked; the defender sees the raid,
            // without the air group that made it.
            row(
                EventKind::BaseAttacked {
                    air_group: group(1),
                    base: base(2),
                    shots: fx(9),
                    chance: Fixed::from_ratio(2, 5),
                    expected: fx(4),
                    hits: 3,
                    hits_on_aircraft: 2,
                    hits_on_supply: 1,
                },
                Cut(
                    "BaseAttacked",
                    &[
                        "chance",
                        "expected",
                        "hits",
                        "hits_on_aircraft",
                        "hits_on_supply",
                    ],
                ),
                Cut("BaseAttackedAgainst", &["air_group"]),
            ),
            row(
                EventKind::AircraftDestroyedOnGround {
                    air_group: group(1),
                    base: base(1),
                    aircraft: 2,
                    remaining: 4,
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::BaseSupplyDestroyed {
                    base: base(1),
                    destroyed_t: fx(5),
                    supply_t: fx(95),
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::BaseSupplySpent {
                    base: base(1),
                    reason: SupplyUse::Upkeep,
                    aircraft: 6,
                    asked_t: fx(3),
                    paid_t: fx(3),
                    left_t: fx(97),
                    men: 0,
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::AirGroupFatigue {
                    air_group: group(1),
                    change: FatigueChange::Rested,
                    amount: fx(5),
                    fatigue: fx(10),
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::AirGroupStoodDown {
                    air_group: group(1),
                    fatigue: fx(70),
                    threshold: fx(60),
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::AirGroupReturned {
                    air_group: group(1),
                    fatigue: fx(50),
                    threshold: fx(60),
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::TaskForceFatigue {
                    task_force: force(1),
                    change: ForceFatigueChange::Gained,
                    amount: fx(5),
                    fatigue: fx(15),
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::TaskForceTurnedBack {
                    task_force: force(1),
                    from: hex(),
                    toward: Hex::new(4, -2),
                    progress_nm: fx(10),
                    position: hex(),
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::ShipRefuelled {
                    ship: ship(1),
                    task_force: force(1),
                    base: base(1),
                    taken_t: fx(10),
                    fuel_t: fx(100),
                    base_fuel_t: fx(50),
                    force_restored: false,
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::BaseAtDayEnd {
                    base: base(1),
                    supply_t: fx(40),
                    aircraft_ready: 6,
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::ShipRepaired {
                    ship: ship(1),
                    task_force: force(1),
                    base: base(1),
                    points: fx(6),
                    fire_damage: fx(0),
                    flotation_damage: fx(4),
                    restored: vec![crate::state::ShipSystem::FireControl],
                    system_repair_points: fx(2),
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::ForceLeftForYard {
                    task_force: force(1),
                    base: base(1),
                    passage_days: 4,
                    ships: vec![(ship(1), 30)],
                    days_away: 34,
                    return_day: 36,
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::ForceReturnedFromYard {
                    task_force: force(1),
                    base: base(1),
                    ships: vec![ship(1)],
                },
                Whole,
                Nothing,
            ),
            // Land units (decision 0066, 23): their own side's whole, the
            // enemy's nothing.
            row(
                EventKind::MenEmbarked {
                    unit: unit(1),
                    ship: ship(1),
                    task_force: force(1),
                    hex: hex(),
                    men: 200,
                    aboard: 200,
                    ashore: 275,
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::MenLanded {
                    unit: unit(1),
                    ship: ship(1),
                    task_force: force(1),
                    hex: hex(),
                    men: 320,
                    aboard: 280,
                    ashore: 320,
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::BaseFormed {
                    base: base(1),
                    side: Side::Allied,
                    hex: hex(),
                    task_force: force(1),
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::UnitDivided {
                    from: unit(1),
                    unit: LandUnitId::new(4),
                    task_force: force(1),
                    men: 200,
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::MenLostWithShip {
                    unit: unit(1),
                    ship: ship(1),
                    men: 200,
                },
                Whole,
                Nothing,
            ),
            sided(row(
                EventKind::MenFed {
                    side: Side::Allied,
                    hex: hex(),
                    base: Some(base(1)),
                    men: 1200,
                    fed: 600,
                    unfed: 600,
                },
                Whole,
                Nothing,
            )),
            row(
                EventKind::MenLostUnfed {
                    unit: unit(1),
                    hex: hex(),
                    lost: 15,
                    ashore: 585,
                },
                Whole,
                Nothing,
            ),
            row(
                EventKind::LandUnitDestroyed { unit: unit(1) },
                Whole,
                Nothing,
            ),
            row(
                EventKind::UnitLoadingComplete {
                    task_force: force(1),
                    unit: unit(1),
                    base: None,
                    cargo: None,
                    elapsed_hours: Fixed::from_raw(2_500_000),
                },
                Whole,
                Nothing,
            ),
        ];
        // Every row is also run in the mirror, with the sides' units swapped.
        rows.shrink_to_fit();
        rows
    }

    /// The variant names of `EventKind`, read from the source.
    fn event_kind_names() -> BTreeSet<String> {
        let source = include_str!("events.rs");
        let start = source
            .find("pub enum EventKind {")
            .expect("the enum is there");
        let body = &source[start..];
        let end = body.find("\n}\n").expect("the enum ends");
        body[..end]
            .lines()
            .filter_map(|line| {
                let rest = line.strip_prefix("    ")?;
                if !rest.chars().next()?.is_ascii_uppercase() {
                    return None;
                }
                Some(
                    rest.chars()
                        .take_while(|c| c.is_ascii_alphanumeric())
                        .collect::<String>(),
                )
            })
            .collect()
    }

    fn kind_name(kind: &EventKind) -> String {
        format!("{kind:?}")
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric())
            .collect()
    }

    #[test]
    fn the_table_has_a_row_for_every_kind_of_event() {
        let all = event_kind_names();
        assert_eq!(all.len(), 76, "the log has 76 kinds of event");
        let covered: BTreeSet<String> = table()
            .iter()
            .map(|row| kind_name(&row.log[row.at]))
            .collect();
        let missing: Vec<&String> = all.difference(&covered).collect();
        assert!(missing.is_empty(), "no row for {missing:?}");
        let unknown: Vec<&String> = covered.difference(&all).collect();
        assert!(
            unknown.is_empty(),
            "rows for kinds that are not there: {unknown:?}"
        );
    }

    /// The names of the fields a `Debug` print of a struct variant lists.
    fn field_names(printed: &str) -> BTreeSet<String> {
        let mut names = BTreeSet::new();
        let Some(open) = printed.find('{') else {
            return names;
        };
        let (mut depth, mut token) = (0usize, String::new());
        for c in printed[open..].chars() {
            match c {
                '{' | '(' | '[' => depth += 1,
                '}' | ')' | ']' => depth -= 1,
                _ => {}
            }
            if depth == 1 && (c.is_ascii_alphanumeric() || c == '_') {
                token.push(c);
            } else {
                if depth == 1 && c == ':' && !token.is_empty() {
                    names.insert(token.clone());
                }
                token.clear();
            }
        }
        names
    }

    fn check(expect: Expect, full: &Event, got: &[SideEvent], who: &str) {
        let here: Vec<&SideEvent> = got.iter().filter(|e| e.id == full.id).collect();
        let name = kind_name(&full.kind);
        match expect {
            Nothing => assert!(here.is_empty(), "{who} sees {name}, which it should not"),
            Whole => {
                assert_eq!(here.len(), 1, "{who} does not see {name} whole");
                assert_eq!(
                    here[0].kind,
                    SideEventKind::Whole(full.kind.clone()),
                    "{who}: {name}"
                );
            }
            Cut(variant, removed) | CutOwnForce(variant, removed) => {
                assert_eq!(here.len(), 1, "{who} does not see a cut {name}");
                let shown = format!("{:?}", here[0].kind);
                let shown_name: String = shown
                    .chars()
                    .take_while(|c| c.is_ascii_alphanumeric())
                    .collect();
                assert_eq!(shown_name, variant, "{who}");
                let full_fields = field_names(&format!("{:?}", full.kind));
                let mut cut_fields = field_names(&shown);
                if matches!(expect, CutOwnForce(..)) {
                    // The one field that is not a field of the full kind: it
                    // holds the side's own force of the log's `a` and `b`.
                    assert!(cut_fields.remove("task_force"), "{who}: {name}");
                    let own = match &here[0].kind {
                        SideEventKind::EngagementBegun { task_force, .. }
                        | SideEventKind::EngagementEnded { task_force, .. } => *task_force,
                        other => panic!("{other:?}"),
                    };
                    let (EventKind::EngagementBegun { a, b, .. }
                    | EventKind::EngagementEnded { a, b, .. }) = &full.kind
                    else {
                        panic!("{name}");
                    };
                    assert!(own == *a || own == *b, "{who}: {name}");
                }
                let lacks: BTreeSet<String> =
                    full_fields.difference(&cut_fields).cloned().collect();
                let expected: BTreeSet<String> = removed.iter().map(ToString::to_string).collect();
                assert_eq!(lacks, expected, "{who}: fields cut from {name}");
                assert!(
                    cut_fields.is_subset(&full_fields),
                    "{who}: {name} gained a field"
                );
            }
        }
    }

    #[test]
    fn every_kind_is_whole_cut_or_absent_for_each_side_as_the_table_says() {
        for row in table() {
            let events = log(row.log.clone());
            let full = &events[row.at];
            for mirror in [false, true] {
                if mirror && !row.mirrors {
                    continue;
                }
                let owners = owners(mirror);
                let (allied, japanese) = if mirror {
                    // The same row with the sides swapped.
                    (row.japanese, row.allied)
                } else {
                    (row.allied, row.japanese)
                };
                check(
                    allied,
                    full,
                    &project(Side::Allied, &events, &owners),
                    "Allied",
                );
                check(
                    japanese,
                    full,
                    &project(Side::Japanese, &events, &owners),
                    "Japanese",
                );
            }
        }
    }

    #[test]
    fn no_event_a_side_is_given_names_an_enemy_ship_force_or_air_group() {
        // Decision 0065. Over every row of the table, both ways round: each
        // unit a side's event names is not the enemy's (a force made in the
        // game is not in the table's owners), or is a base or a contact. The one exception is the target of an order the side
        // itself gave, which is the side's own words.
        for row in table() {
            let events = log(row.log.clone());
            for mirror in [false, true] {
                if mirror && !row.mirrors {
                    continue;
                }
                let owners = owners(mirror);
                for side in [Side::Allied, Side::Japanese] {
                    for event in project(side, &events, &owners) {
                        if matches!(
                            event.kind,
                            SideEventKind::Whole(
                                EventKind::OrderAccepted { .. } | EventKind::OrderRejected { .. }
                            )
                        ) {
                            continue;
                        }
                        for entity in event.kind.entities() {
                            let own = match entity {
                                EntityRef::Ship(_)
                                | EntityRef::TaskForce(_)
                                | EntityRef::AirGroup(_) => {
                                    owners.entity(entity).is_none_or(|owner| owner == side)
                                }
                                EntityRef::Base(_)
                                | EntityRef::Contact(_)
                                | EntityRef::Pilot(_)
                                | EntityRef::LandUnit(_) => true,
                            };
                            assert!(own, "{side:?} is shown {entity} in {:?}", event.kind);
                        }
                        // And the text of it names none either: unit 1 of
                        // each kind is one side's and unit 2 the other's.
                        let enemy = if (side == Side::Allied) == mirror {
                            1
                        } else {
                            2
                        };
                        let text = event.kind.to_string();
                        for unit in ["ship", "task_force", "air_group"] {
                            assert!(
                                !text.contains(&format!("{unit}:{enemy}")),
                                "{side:?} reads {text}"
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn an_event_cut_of_nothing_but_the_enemys_name_keeps_its_whole_breakdown() {
        let factor = |name, n| Factor { name, value: fx(n) };
        let mut events = log(vec![begun(), roll(1, 1, true)]);
        events[1].breakdown = Some(Breakdown {
            factors: vec![factor("base_chance", 1), factor("night_factor", 2)],
            result: fx(2),
        });
        let shown = project(Side::Allied, &events, &owners(false));
        assert_eq!(
            shown[1].kind,
            SideEventKind::SightingRoll {
                round: 1,
                task_force: force(1),
                chance: Fixed::from_ratio(1, 4),
                fatigue_factor: Fixed::ONE,
            }
        );
        assert_eq!(
            shown[1].breakdown,
            Some(SideBreakdown {
                factors: vec![factor("base_chance", 1), factor("night_factor", 2)],
                result: Some(fx(2)),
            })
        );
        assert_eq!(
            shown[0].to_string(),
            "#1 D1 night     task_force:1 met the enemy at (3,-2) at D1 19:00: engagement begun  (and causes you cannot see)"
        );
    }

    #[test]
    fn a_cut_event_is_a_removal_the_whole_is_the_log_event_and_nothing_is_added() {
        // Whole: equal to the log's kind. Cut: only fields of the full kind,
        // with the values the log has.
        let events = log(vec![EventKind::Salvo {
            round: 2,
            firer: ship(1),
            target: ship(2),
            weapon: Weapon::Torpedo,
            shots: fx(8),
            chance: Fixed::from_ratio(1, 10),
            expected: Fixed::from_ratio(4, 5),
            hits: 1,
        }]);
        let shown = project(Side::Allied, &events, &owners(false));
        assert_eq!(
            shown[0].kind,
            SideEventKind::Salvo {
                round: 2,
                firer: ship(1),
                weapon: Weapon::Torpedo,
                shots: fx(8),
            }
        );
        assert_eq!(
            shown[0].to_string(),
            "#1 D1 night     round 2: ship:1 torpedoes: 8 shots"
        );
    }

    // ---- causes ----

    fn caused(mut events: Vec<Event>, index: usize, causes: Vec<Cause>) -> Vec<Event> {
        events[index].causes = causes;
        events
    }

    #[test]
    fn a_cause_that_is_an_own_order_or_a_visible_event_stays_and_any_other_is_dropped() {
        // 1 the Allied salvo (cut, visible), 2 the Japanese ship's damage
        // (hidden from the Allied side), 3 an Allied ship's damage.
        let events = caused(
            log(vec![
                EventKind::Salvo {
                    round: 1,
                    firer: ship(1),
                    target: ship(2),
                    weapon: Weapon::Gun,
                    shots: fx(10),
                    chance: Fixed::ONE,
                    expected: fx(10),
                    hits: 10,
                },
                EventKind::ShipDamaged {
                    round: 1,
                    ship: ship(2),
                    track: DamageTrack::Fire,
                    hits: 1,
                    points: fx(1),
                    value_after: fx(1),
                },
                // A kind that can have causes the side cannot see.
                EventKind::AircraftLost {
                    air_group: group(1),
                    aircraft: 1,
                    remaining: 9,
                },
            ]),
            2,
            vec![
                Cause::Order(order(1)),
                Cause::Order(order(2)),
                Cause::Event(EventId::new(1)),
                Cause::Event(EventId::new(2)),
            ],
        );
        let allied = project(Side::Allied, &events, &owners(false));
        let burned = allied
            .iter()
            .find(|e| e.id == EventId::new(3))
            .expect("seen");
        assert_eq!(
            burned.causes,
            // The side's own numbers: its first order, and the first event
            // it sees (decision 0086).
            vec![SideCause::Order(1), SideCause::Event(1)]
        );
        assert!(
            burned.hidden_causes,
            "the Japanese order and event were dropped"
        );
        // The Japanese side sees the same event of the log as nothing.
        let japanese = project(Side::Japanese, &events, &owners(false));
        assert!(japanese.iter().all(|e| e.id != EventId::new(3)));

        // The flag is the kind's, so an event of the kind with every cause
        // visible says the same (decision 0085, F7).
        let clean = caused(events, 2, vec![Cause::Order(order(1))]);
        let allied = project(Side::Allied, &clean, &owners(false));
        let burned = allied
            .iter()
            .find(|e| e.id == EventId::new(3))
            .expect("seen");
        assert!(burned.hidden_causes);
    }

    // ---- breakdowns ----

    #[test]
    fn a_search_that_sighted_keeps_its_own_factors_and_loses_the_distance_and_the_chance() {
        // Decision 0061: the distance and the chance are worked from where
        // the force really was, and the report may name a neighbouring hex.
        let factor = |name, n| Factor { name, value: fx(n) };
        let mut events = log(vec![EventKind::SearchAttempt {
            air_group: group(1),
            task_force: force(2),
            distance_nm: fx(200),
            chance: Fixed::from_ratio(1, 2),
            sighted: true,
        }]);
        events[0].breakdown = Some(Breakdown {
            factors: vec![
                factor("degrees_per_radian", 57),
                factor("aircraft", 15),
                factor("sweep_width_nm", 20),
                factor("arc_deg", 180),
                factor("distance_nm", 200),
            ],
            result: Fixed::from_ratio(1, 2),
        });
        let shown = project(Side::Allied, &events, &owners(false));
        assert_eq!(
            shown[0].kind,
            SideEventKind::SearchAttempt {
                air_group: group(1),
            }
        );
        assert_eq!(
            shown[0].breakdown,
            Some(SideBreakdown {
                factors: vec![
                    factor("aircraft", 15),
                    factor("sweep_width_nm", 20),
                    factor("arc_deg", 180),
                ],
                result: None,
            })
        );
        assert!(project(Side::Japanese, &events, &owners(false)).is_empty());
    }

    #[test]
    fn a_claim_is_whole_to_its_side_and_its_breakdown_is_not_shown() {
        // Decision 0060: the claim's breakdown says how much of the claim is
        // real, which is the enemy's truth. The side keeps the claim and its
        // causes and gets no breakdown; the log keeps it for the umpire.
        let factor = |name, n| Factor { name, value: fx(n) };
        let claim = EventKind::ClaimReceived {
            side: Side::Allied,
            hex: hex(),
            claimed_sunk: 0,
            claimed_hits: 0,
            claimed_kind: None,
            claimed_shot_down: 30,
            claimed_on_ground: 0,
            from_air: true,
        };
        let mut events = log(vec![claim.clone()]);
        events[0].breakdown = Some(Breakdown {
            factors: vec![
                factor("real_shot_down", 18),
                factor("fighter_misses", 15),
                factor("gun_misses", 118),
                factor("false_shot_down", 12),
            ],
            result: fx(30),
        });
        let shown = project(Side::Allied, &events, &owners(false));
        assert_eq!(shown.len(), 1);
        assert_eq!(shown[0].kind, SideEventKind::Whole(claim));
        assert_eq!(shown[0].breakdown, None);
        // The log is untouched, and the enemy still sees nothing of it.
        assert!(events[0].breakdown.is_some());
        assert!(project(Side::Japanese, &events, &owners(false)).is_empty());
    }

    #[test]
    fn a_cut_event_keeps_only_the_factors_that_are_the_sides_own_and_no_result() {
        let factor = |name, n| Factor { name, value: fx(n) };
        let mut events = log(vec![EventKind::Salvo {
            round: 1,
            firer: ship(1),
            target: ship(2),
            weapon: Weapon::Gun,
            shots: fx(120),
            chance: Fixed::from_ratio(1, 20),
            expected: fx(6),
            hits: 5,
        }]);
        events[0].breakdown = Some(Breakdown {
            factors: vec![
                factor("guns", 8),
                factor("rate", 3),
                factor("minutes", 5),
                factor("main_battery_factor", 1),
                factor("base_chance", 1),
                factor("surprised_factor", 2),
                factor("lit_factor", 3),
                factor("fire_control_factor", 1),
            ],
            result: fx(6),
        });
        let shown = project(Side::Allied, &events, &owners(false));
        let breakdown = shown[0].breakdown.as_ref().expect("a breakdown");
        let names: Vec<&str> = breakdown.factors.iter().map(|f| f.name).collect();
        assert_eq!(
            names,
            vec![
                "guns",
                "rate",
                "minutes",
                "main_battery_factor",
                "fire_control_factor"
            ]
        );
        assert_eq!(breakdown.result, None);

        // A whole event keeps its breakdown, result included.
        let mut events = log(vec![EventKind::FuelBurned {
            ship: ship(1),
            task_force: force(1),
            burned_t: fx(1),
            remaining_t: fx(9),
        }]);
        events[0].breakdown = Some(Breakdown {
            factors: vec![factor("distance", 20)],
            result: fx(1),
        });
        let shown = project(Side::Allied, &events, &owners(false));
        assert_eq!(
            shown[0].breakdown,
            Some(SideBreakdown {
                factors: vec![factor("distance", 20)],
                result: Some(fx(1)),
            })
        );
    }

    #[test]
    fn the_firers_side_keeps_its_experience_factor_and_the_targets_side_sees_no_breakdown() {
        // Decision 0064, 12: the factor is the firer's own, so its side keeps
        // it with the rest of its own factors; the side fired on is given a
        // `SalvoAgainst` with no breakdown at all.
        let factor = |name, n| Factor { name, value: fx(n) };
        let mut events = log(vec![EventKind::Salvo {
            round: 1,
            firer: ship(1),
            target: ship(2),
            weapon: Weapon::Gun,
            shots: fx(120),
            chance: Fixed::from_ratio(1, 20),
            expected: fx(6),
            hits: 5,
        }]);
        events[0].breakdown = Some(Breakdown {
            factors: vec![
                factor("guns", 8),
                factor("base_chance", 1),
                factor("fire_control_factor", 1),
                factor("experience_factor", 2),
            ],
            result: fx(6),
        });
        let shown = project(Side::Allied, &events, &owners(false));
        let names: Vec<&str> = shown[0]
            .breakdown
            .as_ref()
            .expect("a breakdown")
            .factors
            .iter()
            .map(|f| f.name)
            .collect();
        assert_eq!(
            names,
            vec!["guns", "fire_control_factor", "experience_factor"]
        );
        let against = project(Side::Japanese, &events, &owners(false));
        assert!(matches!(
            against[0].kind,
            SideEventKind::SalvoAgainst { .. }
        ));
        assert_eq!(against[0].breakdown, None);
    }

    #[test]
    fn the_firers_side_keeps_its_unready_gun_share_and_the_targets_side_sees_no_breakdown() {
        // Decision 0068, 9: the share is the firing force's own condition, so
        // its side keeps it, after the main battery factor; the side fired on
        // is given a `SalvoAgainst` with no breakdown at all.
        let factor = |name, n| Factor { name, value: fx(n) };
        let mut events = log(vec![EventKind::Salvo {
            round: 1,
            firer: ship(1),
            target: ship(2),
            weapon: Weapon::Gun,
            shots: fx(60),
            chance: Fixed::from_ratio(1, 20),
            expected: fx(3),
            hits: 2,
        }]);
        events[0].breakdown = Some(Breakdown {
            factors: vec![
                factor("guns", 8),
                factor("main_battery_factor", 1),
                Factor {
                    name: "unready_gun_share",
                    value: Fixed::from_ratio(1, 2),
                },
                factor("base_chance", 1),
            ],
            result: fx(3),
        });
        let shown = project(Side::Allied, &events, &owners(false));
        let names: Vec<&str> = shown[0]
            .breakdown
            .as_ref()
            .expect("a breakdown")
            .factors
            .iter()
            .map(|f| f.name)
            .collect();
        assert_eq!(
            names,
            vec!["guns", "main_battery_factor", "unready_gun_share"]
        );
        let against = project(Side::Japanese, &events, &owners(false));
        assert!(matches!(
            against[0].kind,
            SideEventKind::SalvoAgainst { .. }
        ));
        assert_eq!(against[0].breakdown, None);
    }

    #[test]
    fn the_firers_side_keeps_its_unready_torpedo_share_and_the_targets_side_sees_no_breakdown() {
        // Decision 0071: the share is the firing force's own condition, so its
        // side keeps it, after the torpedo mount factor; the side fired on is
        // given a `SalvoAgainst` with no breakdown at all.
        let factor = |name, n| Factor { name, value: fx(n) };
        let mut events = log(vec![EventKind::Salvo {
            round: 1,
            firer: ship(1),
            target: ship(2),
            weapon: Weapon::Torpedo,
            shots: fx(4),
            chance: Fixed::from_ratio(1, 25),
            expected: Fixed::from_ratio(4, 25),
            hits: 0,
        }]);
        events[0].breakdown = Some(Breakdown {
            factors: vec![
                factor("tubes", 8),
                factor("torpedo_mount_factor", 1),
                Factor {
                    name: "unready_torpedo_share",
                    value: Fixed::from_ratio(1, 2),
                },
                factor("base_chance", 1),
            ],
            result: Fixed::from_ratio(4, 25),
        });
        let shown = project(Side::Allied, &events, &owners(false));
        let names: Vec<&str> = shown[0]
            .breakdown
            .as_ref()
            .expect("a breakdown")
            .factors
            .iter()
            .map(|f| f.name)
            .collect();
        assert_eq!(
            names,
            vec!["tubes", "torpedo_mount_factor", "unready_torpedo_share"]
        );
        let against = project(Side::Japanese, &events, &owners(false));
        assert!(matches!(
            against[0].kind,
            SideEventKind::SalvoAgainst { .. }
        ));
        assert_eq!(against[0].breakdown, None);
    }

    // ---- the round rule of decision 5 ----

    fn roll(round: u32, task_force: u32, sighted: bool) -> EventKind {
        EventKind::SightingRoll {
            round,
            task_force: force(task_force),
            enemy: force(3 - task_force),
            chance: Fixed::from_ratio(1, 4),
            fatigue_factor: Fixed::ONE,
            sighted,
        }
    }

    fn salvo(round: u32, firer: u32, target: u32) -> EventKind {
        EventKind::Salvo {
            round,
            firer: ship(firer),
            target: ship(target),
            weapon: Weapon::Gun,
            shots: fx(100),
            chance: Fixed::from_ratio(1, 20),
            expected: fx(5),
            hits: 5,
        }
    }

    fn damaged(round: u32, ship_id: u32) -> EventKind {
        EventKind::ShipDamaged {
            round,
            ship: ship(ship_id),
            track: DamageTrack::Flotation,
            hits: 5,
            points: fx(25),
            value_after: fx(25),
        }
    }

    fn begun() -> EventKind {
        EventKind::EngagementBegun {
            a: force(1),
            b: force(2),
            hex: hex(),
            elapsed_hours: fx(1),
        }
    }

    fn ended(round: u32, reason: EngagementEndReason) -> EventKind {
        EventKind::EngagementEnded {
            round,
            a: force(1),
            b: force(2),
            hex: hex(),
            reason,
        }
    }

    fn ids(events: &[SideEvent]) -> Vec<u32> {
        events.iter().map(|e| e.id.raw()).collect()
    }

    #[test]
    fn a_side_that_sights_late_and_was_never_fired_on_sees_the_fight_from_its_own_round() {
        // Allied force 1 sights in round 1 and has no salvo to fire (it is
        // out of ammunition). Japanese force 2 fails to sight in rounds 1
        // and 2, sights in round 3 and fires at the Allied ship.
        //  1 begun
        //  2 round 1: A sights        3 round 1: J fails
        //  4 round 2: J fails
        //  5 round 3: J sights        6 round 3: J fires
        //  7 round 3: A ship hit      8 ended
        let events = log(vec![
            begun(),
            roll(1, 1, true),
            roll(1, 2, false),
            roll(2, 2, false),
            roll(3, 2, true),
            salvo(3, 2, 1),
            damaged(3, 1),
            ended(3, EngagementEndReason::MaxRounds),
        ]);
        let owners = owners(false);
        // Allied: known from round 1, by its own roll. It sees the fight
        // begin, its own roll, the hit it took and the end. It does not see
        // the Japanese failed rolls, success or salvo.
        let allied = project(Side::Allied, &events, &owners);
        assert_eq!(ids(&allied), vec![1, 2, 6, 7, 8]);
        // Japanese: known from round 3, by its own roll. It sees the begin at
        // its place, its own roll and its own salvo, cut, and the end. It
        // sees nothing of rounds 1 and 2: not the Allied sighting, and not
        // its own failed rolls.
        let japanese = project(Side::Japanese, &events, &owners);
        assert_eq!(ids(&japanese), vec![1, 5, 6, 8]);
        // Its rounds are counted from the round it first knew (decision 0085,
        // F6): its round 3 is the round 1 it is told of.
        assert!(matches!(
            japanese[2].kind,
            SideEventKind::Salvo { round: 1, .. }
        ));
        assert!(japanese
            .iter()
            .filter(|e| e.id.raw() != 1 && e.id.raw() != 8)
            .all(|e| matches!(e.kind, SideEventKind::Salvo { round: 1, .. })
                || matches!(&e.kind, SideEventKind::SightingRoll { round: 1, .. })));
    }

    #[test]
    fn a_side_fired_on_in_round_1_knows_of_the_fight_from_round_1_though_it_sighted_in_round_3() {
        // The case of the brief: A sights in round 1 and fires in rounds 1 and
        // 2; B sights only in round 3. B is fired on in round 1, so by
        // decision 5 it knows of the fight from round 1: it sees what was
        // done to its ships from round 1, and nothing of A's rolls or salvoes.
        //  1 begun          2 R1 A sights      3 R1 B fails
        //  4 R1 A fires     5 R1 B ship hit    6 R2 B fails
        //  7 R2 A fires     8 R2 B ship hit    9 R3 B sights
        // 10 R3 B fires    11 R3 A ship hit   12 ended
        let events = log(vec![
            begun(),
            roll(1, 1, true),
            roll(1, 2, false),
            salvo(1, 1, 2),
            damaged(1, 2),
            roll(2, 2, false),
            salvo(2, 1, 2),
            damaged(2, 2),
            roll(3, 2, true),
            salvo(3, 2, 1),
            damaged(3, 1),
            ended(3, EngagementEndReason::MaxRounds),
        ]);
        // As the rules record them, each hit cites the salvo that made it.
        let events = caused(events, 4, vec![Cause::Event(EventId::new(4))]);
        let events = caused(events, 7, vec![Cause::Event(EventId::new(7))]);
        let owners = owners(false);
        let japanese = project(Side::Japanese, &events, &owners);
        assert_eq!(ids(&japanese), vec![1, 4, 5, 7, 8, 9, 10, 12]);
        // A's rolls and salvoes in rounds 1 and 2 are not there, nor its own
        // failed rolls; the hits on its ships are, with causes it cannot see.
        assert!(matches!(
            japanese[1].kind,
            SideEventKind::SalvoAgainst { round: 1, .. }
        ));
        // Log event 4 is the second event the Japanese side sees.
        assert_eq!(japanese[2].causes, vec![SideCause::Event(2)]);
        assert!(!japanese[2].hidden_causes);
        // The Allied side: its own roll, its own salvoes cut, the hit it took
        // in round 3.
        let allied = project(Side::Allied, &events, &owners);
        assert_eq!(ids(&allied), vec![1, 2, 4, 7, 10, 11, 12]);
    }

    #[test]
    fn a_fight_neither_side_knows_of_is_absent_for_both_and_one_side_knows_is_shown_to_it_only() {
        let owners = owners(false);
        // Passed unseen: no roll sighted, nobody fired.
        let unseen = log(vec![
            begun(),
            roll(1, 1, false),
            roll(1, 2, false),
            EventKind::ForcesPassedUnseen {
                round: 1,
                a: force(1),
                b: force(2),
            },
            ended(1, EngagementEndReason::PassedUnseen),
        ]);
        assert!(project(Side::Allied, &unseen, &owners).is_empty());
        assert!(project(Side::Japanese, &unseen, &owners).is_empty());

        // Allied force 1 sights and slips away: only it sees the fight.
        let slipped = log(vec![
            begun(),
            roll(1, 1, true),
            roll(1, 2, false),
            EventKind::ForceSlippedAway {
                round: 1,
                task_force: force(1),
                enemy: force(2),
            },
            ended(1, EngagementEndReason::SlippedAway),
        ]);
        assert_eq!(
            ids(&project(Side::Allied, &slipped, &owners)),
            vec![1, 2, 4, 5]
        );
        assert!(project(Side::Japanese, &slipped, &owners).is_empty());
    }

    #[test]
    fn a_fight_the_log_stops_in_the_middle_of_is_read_as_far_as_it_goes() {
        let owners = owners(false);
        let events = log(vec![begun(), roll(1, 1, true)]);
        assert_eq!(ids(&project(Side::Allied, &events, &owners)), vec![1, 2]);
        assert!(project(Side::Japanese, &events, &owners).is_empty());
    }

    #[test]
    fn a_missed_salvo_still_tells_the_target_it_was_shot_at() {
        // The target is fired on, and so knows of the fight; the table lets
        // it see the salvo only if the salvo is its own, and no ship was hit.
        let owners = owners(false);
        let events = log(vec![
            begun(),
            salvo(1, 1, 2),
            ended(1, EngagementEndReason::MaxRounds),
        ]);
        assert_eq!(
            ids(&project(Side::Japanese, &events, &owners)),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn the_second_of_two_fights_is_read_on_its_own() {
        // A fight between forces 1 and 2, then one between forces 1 and 3
        // (force 3 is Japanese). The Japanese side knows of the second only.
        let mut owners = owners(false);
        owners.forces.insert(force(3), Side::Japanese);
        owners.ships.insert(ship(3), Side::Japanese);
        let events = log(vec![
            begun(),
            roll(1, 1, false),
            ended(1, EngagementEndReason::PassedUnseen),
            EventKind::EngagementBegun {
                a: force(1),
                b: force(3),
                hex: hex(),
                elapsed_hours: fx(2),
            },
            salvo(1, 1, 3),
            EventKind::EngagementEnded {
                round: 1,
                a: force(1),
                b: force(3),
                hex: hex(),
                reason: EngagementEndReason::MaxRounds,
            },
        ]);
        assert_eq!(
            ids(&project(Side::Japanese, &events, &owners)),
            vec![4, 5, 6]
        );
        assert_eq!(ids(&project(Side::Allied, &events, &owners)), vec![4, 5, 6]);
    }

    #[test]
    fn the_begun_event_of_a_fight_loses_the_orders_of_the_other_side() {
        let mut events = log(vec![
            begun(),
            salvo(1, 1, 2),
            ended(1, EngagementEndReason::MaxRounds),
        ]);
        events[0].causes = vec![Cause::Order(order(1)), Cause::Order(order(2))];
        let owners = owners(false);
        let allied = project(Side::Allied, &events, &owners);
        assert_eq!(allied[0].causes, vec![SideCause::Order(1)]);
        assert!(allied[0].hidden_causes);
        let japanese = project(Side::Japanese, &events, &owners);
        // The Japanese side's only order is its first, whatever its place
        // in the order book.
        assert_eq!(japanese[0].causes, vec![SideCause::Order(1)]);
    }

    use std::collections::BTreeSet;

    #[test]
    fn a_force_that_breaks_off_before_it_knew_of_the_fight_is_not_told_of_it() {
        // Allied force 1 has ships crippled before the meeting and breaks off
        // in round 1 with no sighting and no shot at it.
        let events = log(vec![
            begun(),
            roll(1, 1, false),
            EventKind::ForceBrokeOff {
                round: 1,
                task_force: force(1),
                share: Fixed::from_ratio(1, 2),
            },
            ended(1, EngagementEndReason::BrokeOff),
        ]);
        let owners = owners(false);
        assert!(project(Side::Allied, &events, &owners).is_empty());
        assert!(project(Side::Japanese, &events, &owners).is_empty());
    }

    // ---- findings of the outside review of Nov 13, 2026 (prompt 11) ----
    //
    // Each is kept switched off: it records a leak the review found and the
    // architect has not yet ruled on. Run with `--ignored` to see it.

    #[test]
    fn a_side_whose_force_survived_is_not_told_the_enemy_force_was_sunk() {
        // Allied force 1 sights and fires in round 1 and sinks the one ship
        // of Japanese force 2; its own ship is never hit. The Allied side
        // knows of the fight from round 1.
        //  1 begun   2 R1 A sights   3 R1 A fires   4 R1 J ship sunk   5 ended
        let events = log(vec![
            begun(),
            roll(1, 1, true),
            salvo(1, 1, 2),
            EventKind::ShipSunk {
                round: 1,
                ship: ship(2),
                task_force: force(2),
            },
            ended(1, EngagementEndReason::ForceSunk),
        ]);
        let allied = project(Side::Allied, &events, &owners(false));
        // The side does not see the sinking (event 4): its claim is what it
        // learns of it. The ending must not say it either.
        assert!(!ids(&allied).contains(&4));
        let ending = allied
            .iter()
            .find_map(|e| match e.kind {
                SideEventKind::EngagementEnded { reason, .. } => Some(reason),
                _ => None,
            })
            .expect("the side knows the fight ended");
        assert_ne!(
            ending,
            SideEndReason::ForceSunk,
            "the Allied side, its own ship afloat, is told a force was sunk: the enemy's"
        );
    }

    #[test]
    fn a_side_that_learns_of_a_fight_late_is_not_given_the_hour_it_began() {
        // The log of the test above: the Japanese side first knows of the
        // fight in round 3, by its own sighting. Decision 0047, 5: before
        // that it does not know there is a fight.
        let events = log(vec![
            begun(),
            roll(1, 1, true),
            roll(1, 2, false),
            roll(2, 2, false),
            roll(3, 2, true),
            salvo(3, 2, 1),
            damaged(3, 1),
            ended(3, EngagementEndReason::MaxRounds),
        ]);
        let japanese = project(Side::Japanese, &events, &owners(false));
        let met_at = japanese.iter().find_map(|e| match e.kind {
            SideEventKind::EngagementBegun { elapsed_hours, .. } => Some(elapsed_hours),
            _ => None,
        });
        assert_ne!(
            met_at,
            Some(fx(1)),
            "the Japanese side is given the hour the forces met, two rounds before it knew of them"
        );
    }

    #[test]
    fn the_flag_of_hidden_causes_does_not_say_whether_the_enemy_gave_an_order() {
        // The fight begins in both logs; in the second the Japanese order 2
        // (its rules of engagement, restated) is among the beginning's
        // causes, as `surface.rs` records it. Nothing else differs.
        let plain = log(vec![
            begun(),
            roll(1, 1, true),
            ended(1, EngagementEndReason::MaxRounds),
        ]);
        let ordered = caused(plain.clone(), 0, vec![Cause::Order(order(2))]);
        let owners = owners(false);
        let one = project(Side::Allied, &plain, &owners);
        let two = project(Side::Allied, &ordered, &owners);
        assert_eq!(
            one, two,
            "the Allied side's events differ only by an order of the enemy's it may not see"
        );
    }

    // ---- decision 0085: what a side is told of a fight's ending, hour, rounds ----

    fn ending_of(events: &[SideEvent]) -> SideEndReason {
        events
            .iter()
            .find_map(|e| match e.kind {
                SideEventKind::EngagementEnded { reason, .. } => Some(reason),
                _ => None,
            })
            .expect("the side knows the fight ended")
    }

    fn broke_off(round: u32, task_force: u32) -> EventKind {
        EventKind::ForceBrokeOff {
            round,
            task_force: force(task_force),
            share: Fixed::from_ratio(1, 2),
        }
    }

    fn sunk(round: u32, ship_id: u32, task_force: u32) -> EventKind {
        EventKind::ShipSunk {
            round,
            ship: ship(ship_id),
            task_force: force(task_force),
        }
    }

    #[test]
    fn a_fight_the_enemy_force_broke_off_ends_for_a_side_in_contact_lost() {
        // Allied force 1 sights and fires; Japanese force 2 breaks off.
        let events = log(vec![
            begun(),
            roll(1, 1, true),
            salvo(1, 1, 2),
            broke_off(1, 2),
            ended(1, EngagementEndReason::BrokeOff),
        ]);
        let allied = project(Side::Allied, &events, &owners(false));
        assert_eq!(ending_of(&allied), SideEndReason::ContactLost);
        // The side is not given the enemy's break-off either.
        assert!(!ids(&allied).contains(&4));
        assert_eq!(
            allied.last().expect("the end").to_string(),
            "#4 D1 night     engagement of task_force:1 at (3,-2) ended in round 1: contact lost  (and causes you cannot see)"
        );
    }

    #[test]
    fn a_fight_the_sides_own_force_broke_off_ends_for_it_in_broke_off() {
        // Allied force 1 sights, is fired on, and breaks off.
        let events = log(vec![
            begun(),
            roll(1, 1, true),
            salvo(1, 2, 1),
            damaged(1, 1),
            broke_off(1, 1),
            ended(1, EngagementEndReason::BrokeOff),
        ]);
        let allied = project(Side::Allied, &events, &owners(false));
        assert_eq!(ending_of(&allied), SideEndReason::BrokeOff);
        // The enemy is told the other way: its force broke nothing off.
        let japanese = project(Side::Japanese, &events, &owners(false));
        assert_eq!(ending_of(&japanese), SideEndReason::ContactLost);
    }

    #[test]
    fn a_force_that_lost_a_ship_and_is_away_at_the_yard_is_not_told_its_force_was_sunk() {
        use crate::state::{TaskForce, TruthState, YardTrip};
        // Allied force 1 loses its ship 1 in round 1 of the fight; Japanese
        // force 2 loses its only ship 2 and the fight ends force-sunk by that
        // loss. Allied force 1 was left with ships and has since left for the
        // yard, so it has none now, but it is not sunk.
        let events = log(vec![
            begun(),
            roll(1, 1, true),
            salvo(1, 2, 1),
            sunk(1, 1, 1),
            salvo(1, 1, 2),
            sunk(1, 2, 2),
            ended(1, EngagementEndReason::ForceSunk),
        ]);
        let mut truth = TruthState::default();
        for (n, side) in [(1, Side::Allied), (2, Side::Japanese)] {
            truth.task_forces.insert(
                force(n),
                TaskForce::at(
                    force(n),
                    format!("Force {n}"),
                    side,
                    hex(),
                    crate::orders::Mission::SurfaceCombat,
                ),
            );
        }
        for (n, side) in [(1, Side::Allied), (2, Side::Japanese)] {
            truth.ships.insert(
                ship(n),
                crate::state::Ship::new(
                    ship(n),
                    format!("Ship {n}"),
                    crate::ids::ShipClassId::new(1),
                    side,
                    crate::state::ShipLocation::Sunk,
                ),
            );
        }
        let orders = BTreeMap::new();
        // Before it leaves, with no ship, the Allied force would be read as
        // emptied and told its force was sunk.
        let emptied = Owners::from_state(&truth, &orders, 5);
        assert!(emptied.emptied_forces.contains(&force(1)));
        assert_eq!(
            ending_of(&project(Side::Allied, &events, &emptied)),
            SideEndReason::ForceSunk
        );
        // Away at the yard, it is not emptied: its own true reason is the one
        // a side that lost nothing of that force is told.
        truth
            .task_forces
            .get_mut(&force(1))
            .expect("force")
            .yard_trip = Some(YardTrip {
            base: base(1),
            return_day: 30,
            departure: EventId::new(1),
        });
        let away = Owners::from_state(&truth, &orders, 5);
        assert!(!away.emptied_forces.contains(&force(1)));
        assert!(away.emptied_forces.contains(&force(2)));
        assert_eq!(
            ending_of(&project(Side::Allied, &events, &away)),
            SideEndReason::ContactLost
        );
        // The Japanese side, whose force was emptied for good, is still told
        // its force was sunk.
        assert_eq!(
            ending_of(&project(Side::Japanese, &events, &away)),
            SideEndReason::ForceSunk
        );
    }

    #[test]
    fn a_fight_the_sides_own_force_was_emptied_in_ends_for_it_in_force_sunk() {
        // Japanese force 2's one ship is sunk by Allied fire in round 1.
        let events = log(vec![
            begun(),
            roll(1, 1, true),
            salvo(1, 1, 2),
            sunk(1, 2, 2),
            ended(1, EngagementEndReason::ForceSunk),
        ]);
        let mut owners = owners(false);
        owners.emptied_forces.insert(force(2));
        let japanese = project(Side::Japanese, &events, &owners);
        assert_eq!(ending_of(&japanese), SideEndReason::ForceSunk);
        // The Allied side, whose force was not emptied, is told contact lost.
        let allied = project(Side::Allied, &events, &owners);
        assert_eq!(ending_of(&allied), SideEndReason::ContactLost);
        // A force emptied in an earlier fight, whose last sinking is not in
        // this one, is not "the force sunk" of this one.
        let earlier = log(vec![
            sunk(1, 1, 1),
            begun(),
            roll(1, 1, true),
            salvo(1, 2, 1),
            ended(1, EngagementEndReason::ForceSunk),
        ]);
        owners.emptied_forces.insert(force(1));
        let allied = project(Side::Allied, &earlier, &owners);
        assert_eq!(ending_of(&allied), SideEndReason::ContactLost);
    }

    #[test]
    fn a_side_that_learns_of_a_fight_late_is_given_every_round_counted_from_its_own_first() {
        // Japanese force 2 first knows of the fight in round 3 (its own
        // sighting), has a ship hit, becomes alert and sinks, and sees an end
        // in round 3. Every round it is given, whole kinds included, is
        // counted from round 3 as round 1, and only that is altered in a
        // whole kind.
        let events = log(vec![
            begun(),
            roll(1, 1, true),
            roll(1, 2, false),
            roll(3, 2, true),
            salvo(3, 2, 1),
            salvo(3, 1, 2),
            damaged(3, 2),
            EventKind::ForceBecameAlert {
                round: 3,
                task_force: force(2),
            },
            sunk(3, 2, 2),
            ended(3, EngagementEndReason::MaxRounds),
        ]);
        let japanese = project(Side::Japanese, &events, &owners(false));
        let rounds: Vec<Option<u32>> = japanese
            .iter()
            .map(|e| match &e.kind {
                SideEventKind::Salvo { round, .. }
                | SideEventKind::SalvoAgainst { round, .. }
                | SideEventKind::SightingRoll { round, .. }
                | SideEventKind::EngagementEnded { round, .. }
                | SideEventKind::Whole(
                    EventKind::ShipDamaged { round, .. }
                    | EventKind::ShipSunk { round, .. }
                    | EventKind::ForceBecameAlert { round, .. },
                ) => Some(*round),
                _ => None,
            })
            .collect();
        // begun, sighting, salvo, salvo against, damaged, alert, sunk, ended.
        assert_eq!(
            rounds,
            vec![
                None,
                Some(1),
                Some(1),
                Some(1),
                Some(1),
                Some(1),
                Some(1),
                Some(1)
            ]
        );
        // The hour: 1 + 2 rounds of 5 minutes is 1 h 10 min.
        let hour = japanese.iter().find_map(|e| match e.kind {
            SideEventKind::EngagementBegun { elapsed_hours, .. } => Some(elapsed_hours),
            _ => None,
        });
        assert_eq!(hour, Some(fx(1) + Fixed::from_ratio(10, 60)));
        // The hit is as the log has it, but for its round.
        let hit = japanese
            .iter()
            .find_map(|e| match &e.kind {
                SideEventKind::Whole(kind @ EventKind::ShipDamaged { .. }) => Some(kind.clone()),
                _ => None,
            })
            .expect("the hit on its ship");
        let EventKind::ShipDamaged {
            ship: hit_ship,
            track,
            hits,
            points,
            value_after,
            ..
        } = damaged(1, 2)
        else {
            unreachable!()
        };
        assert_eq!(
            hit,
            EventKind::ShipDamaged {
                round: 1,
                ship: hit_ship,
                track,
                hits,
                points,
                value_after,
            }
        );
        // A side that knows from round 1 is given the log's rounds and hour.
        let allied = project(Side::Allied, &events, &owners(false));
        assert!(allied
            .iter()
            .any(|e| matches!(e.kind, SideEventKind::SightingRoll { round: 1, .. })));
        assert!(allied.iter().any(|e| matches!(
            e.kind,
            SideEventKind::EngagementBegun { elapsed_hours, .. } if elapsed_hours == fx(1)
        )));
    }

    #[test]
    fn a_side_is_given_no_raw_draw_in_the_breakdown_of_its_alarm_roll() {
        let factor = |name, n| Factor { name, value: fx(n) };
        let mut events = log(vec![EventKind::AlarmRoll {
            task_force: force(1),
            by_a: force(2),
            by_b: force(3),
            chance: Fixed::from_ratio(1, 4),
            alarmed: true,
        }]);
        events[0].breakdown = Some(Breakdown {
            factors: vec![factor("alarm_chance", 1), factor("draw", 123_456)],
            result: Fixed::from_ratio(1, 4),
        });
        let shown = project(Side::Allied, &events, &owners(false));
        let names: Vec<&str> = shown[0]
            .breakdown
            .as_ref()
            .expect("a breakdown")
            .factors
            .iter()
            .map(|f| f.name)
            .collect();
        assert_eq!(names, vec!["alarm_chance"]);
        // The umpire's breakdown, the log's, keeps the draw.
        assert!(events[0]
            .breakdown
            .as_ref()
            .expect("the log's")
            .factors
            .iter()
            .any(|f| f.name == "draw"));
    }
}
