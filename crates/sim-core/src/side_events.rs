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

use crate::events::{AirFirer, AirTarget, Breakdown, Cause, Event, EventKind, Factor, Weapon};
use crate::fixed::Fixed;
use crate::ids::{AirGroupId, BaseId, EntityRef, EventId, OrderId, ShipId, TaskForceId};
use crate::state::{Side, TruthState};
use crate::time::Pulse;

/// An event as one side may know it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SideEvent {
    /// The event's number in the full log, so that two sides and the umpire
    /// can talk about the same event.
    pub id: EventId,
    pub day: u32,
    pub pulse: Option<Pulse>,
    pub kind: SideEventKind,
    /// The causes the side may see: its own orders, and events it may see.
    pub causes: Vec<Cause>,
    /// True if the event has causes the side may not see.
    pub hidden_causes: bool,
    /// Present when the log has one. For a whole event it is the log's. For a
    /// cut event it is cut too (see [`SideBreakdown`]).
    pub breakdown: Option<SideBreakdown>,
}

/// A breakdown as a side may see it. For a whole event, every factor and the
/// result. For a cut event, only the factors that are the side's own, and no
/// result, because the result is the quantity that was cut.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SideBreakdown {
    pub factors: Vec<Factor>,
    pub result: Option<Fixed>,
}

/// What of an event a side may know.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SideEventKind {
    /// The event as the log has it.
    Whole(EventKind),
    /// The side's own salvo: that it fired, at what, and with how many shots.
    Salvo {
        round: u32,
        firer: ShipId,
        target: ShipId,
        weapon: Weapon,
        shots: Fixed,
    },
    /// A salvo fired at the side's ship: that it was shot at, by which ship
    /// and with what. Not the shots, nor what they did (the hits it took
    /// reach it as `ShipDamaged`).
    SalvoAgainst {
        round: u32,
        firer: ShipId,
        target: ShipId,
        weapon: Weapon,
    },
    /// An air attack on the side's ship: by which group, on which ship, and
    /// how many aircraft it can count. Not the chance or the hits.
    AirAttackAgainst {
        air_group: AirGroupId,
        task_force: TaskForceId,
        ship: ShipId,
        shots: Fixed,
    },
    /// The side's own air attack on a ship.
    AirAttack {
        air_group: AirGroupId,
        task_force: TaskForceId,
        ship: ShipId,
        weapon: Weapon,
        shots: Fixed,
    },
    /// The side's own fighters firing in a fight in the air.
    AirFight {
        firer: AirGroupId,
        target: AirGroupId,
        firing: AirFirer,
        at: AirTarget,
        shots: Fixed,
    },
    /// The side's own ships firing at a raid.
    AntiAircraftFire {
        task_force: TaskForceId,
        air_group: AirGroupId,
        shots: Fixed,
    },
    /// The side's own base firing at a raid.
    BaseAntiAircraftFire {
        base: BaseId,
        air_group: AirGroupId,
        shots: Fixed,
    },
    /// The side's own bombs falling on an enemy base: that it attacked and
    /// with how many shots, not what they hit.
    BaseAttacked {
        air_group: AirGroupId,
        base: BaseId,
        shots: Fixed,
    },
}

impl SideEventKind {
    /// The entities the event is about, as the full event names them.
    pub fn entities(&self) -> Vec<EntityRef> {
        match self {
            SideEventKind::Whole(kind) => kind.entities(),
            SideEventKind::Salvo { firer, target, .. }
            | SideEventKind::SalvoAgainst { firer, target, .. } => {
                vec![EntityRef::Ship(*firer), EntityRef::Ship(*target)]
            }
            SideEventKind::AirAttack {
                air_group,
                task_force,
                ship,
                ..
            }
            | SideEventKind::AirAttackAgainst {
                air_group,
                task_force,
                ship,
                ..
            } => vec![
                EntityRef::AirGroup(*air_group),
                EntityRef::TaskForce(*task_force),
                EntityRef::Ship(*ship),
            ],
            SideEventKind::AirFight { firer, target, .. } => {
                vec![EntityRef::AirGroup(*firer), EntityRef::AirGroup(*target)]
            }
            SideEventKind::AntiAircraftFire {
                task_force,
                air_group,
                ..
            } => vec![
                EntityRef::TaskForce(*task_force),
                EntityRef::AirGroup(*air_group),
            ],
            SideEventKind::BaseAntiAircraftFire {
                base, air_group, ..
            }
            | SideEventKind::BaseAttacked {
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
                target,
                weapon,
                shots,
            } => write!(
                f,
                "round {round}: {firer} {} at {target}: {shots} shots",
                weapon.name()
            ),
            SideEventKind::SalvoAgainst {
                round,
                firer,
                target,
                weapon,
            } => write!(f, "round {round}: {firer} {} at {target}", weapon.name()),
            SideEventKind::AirAttackAgainst {
                air_group,
                ship,
                shots,
                ..
            } => write!(f, "{air_group} attacks {ship} with {shots} aircraft"),
            SideEventKind::AirAttack {
                air_group,
                ship,
                weapon,
                shots,
                ..
            } => write!(f, "{air_group} {} at {ship}: {shots} shots", weapon.name()),
            SideEventKind::AirFight {
                firer,
                target,
                firing,
                at,
                shots,
            } => write!(
                f,
                "{firer} {} at the {} of {target}: {shots} shots",
                firing.name(),
                at.name()
            ),
            SideEventKind::AntiAircraftFire {
                task_force,
                air_group,
                shots,
            } => write!(
                f,
                "{task_force} guns at the raid of {air_group}: {shots} shots"
            ),
            SideEventKind::BaseAntiAircraftFire {
                base,
                air_group,
                shots,
            } => write!(f, "{base} guns at the raid of {air_group}: {shots} shots"),
            SideEventKind::BaseAttacked {
                air_group,
                base,
                shots,
            } => write!(f, "{air_group} bombs at {base}: {shots} shots"),
        }
    }
}

impl fmt::Display for SideEvent {
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
    pub(crate) orders: BTreeMap<OrderId, Side>,
}

impl Owners {
    pub(crate) fn from_state(
        truth: &TruthState,
        orders: &BTreeMap<OrderId, crate::orders::Order>,
    ) -> Owners {
        Owners {
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
            orders: orders.iter().map(|(id, o)| (*id, o.side)).collect(),
        }
    }

    fn entity(&self, entity: EntityRef) -> Option<Side> {
        match entity {
            EntityRef::Ship(id) => self.ships.get(&id).copied(),
            EntityRef::TaskForce(id) => self.forces.get(&id).copied(),
            EntityRef::AirGroup(id) => self.air_groups.get(&id).copied(),
            EntityRef::Base(id) => self.bases.get(&id).copied(),
            EntityRef::Pilot(_) | EntityRef::LandUnit(_) | EntityRef::Contact(_) => None,
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
        // The force surprised, not the one that surprised it.
        | EventKind::ForceSurprised { task_force, .. }
        | EventKind::ForceBecameAlert { task_force, .. }
        | EventKind::ForceBrokeOff { task_force, .. }
        // The force that slipped away, not the one it slipped away from.
        | EventKind::ForceSlippedAway { task_force, .. } => whole_if(own_force(task_force)),
        EventKind::ShipsDetached { parent, .. } => whole_if(own_force(parent)),
        // The hearer's, only if it was alarmed: a failed alarm means the
        // force noticed nothing. It names the forces that fought.
        EventKind::AlarmRoll {
            task_force,
            alarmed,
            ..
        } => whole_if(*alarmed && own_force(task_force)),

        // Ships.
        EventKind::FuelBurned { ship, .. }
        | EventKind::ShipRefuelled { ship, .. }
        | EventKind::CargoLoaded { ship, .. }
        | EventKind::CargoUnloaded { ship, .. }
        | EventKind::ShipDamaged { ship, .. }
        | EventKind::SystemHit { ship, .. }
        | EventKind::ShipSunk { ship, .. } => whole_if(own_ship(ship)),

        // Air groups.
        EventKind::SearchFlown { air_group, .. }
        | EventKind::DamagedAircraftDestroyedOnGround { air_group, .. }
        | EventKind::AircraftWorn { air_group, .. }
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
        | EventKind::BaseSupplySpent { base, .. } => whole_if(own_base(base)),

        // A side's own: the report, the claim, what became of its contacts.
        EventKind::ReportReceived { side: owner, .. }
        | EventKind::ContactAged { side: owner, .. }
        | EventKind::ContactDropped { side: owner, .. }
        | EventKind::ClaimReceived { side: owner, .. }
        | EventKind::ContactNotThere { side: owner, .. } => whole_if(*owner == side),

        // Whole to a side that has come to know of the fight.
        EventKind::EngagementBegun { .. } | EventKind::EngagementEnded { .. } => {
            whole_if(knows_fight)
        }

        // Its own roll if it sighted. A roll that did not is hidden, for
        // either side: it would say there was something to be found.
        EventKind::SightingRoll {
            task_force,
            sighted,
            ..
        } => whole_if(*sighted && own_force(task_force)),

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
                    target: *target,
                    weapon: *weapon,
                    shots: *shots,
                })
            } else {
                // Fired at: it knows it was shot at, by what.
                cut_if(
                    own_ship(target),
                    SideEventKind::SalvoAgainst {
                        round: *round,
                        firer: *firer,
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
                    task_force: *task_force,
                    ship: *ship,
                    weapon: *weapon,
                    shots: *shots,
                })
            } else {
                // The ship attacked: it can count the aircraft.
                cut_if(
                    own_ship(ship),
                    SideEventKind::AirAttackAgainst {
                        air_group: *air_group,
                        task_force: *task_force,
                        ship: *ship,
                        shots: *shots,
                    },
                )
            }
        }
        EventKind::AirFight {
            firer,
            target,
            firing,
            at,
            shots,
            ..
        } => cut_if(
            own_group(firer),
            SideEventKind::AirFight {
                firer: *firer,
                target: *target,
                firing: *firing,
                at: *at,
                shots: *shots,
            },
        ),
        EventKind::AntiAircraftFire {
            task_force,
            air_group,
            shots,
            ..
        } => cut_if(
            own_force(task_force),
            SideEventKind::AntiAircraftFire {
                task_force: *task_force,
                air_group: *air_group,
                shots: *shots,
            },
        ),
        EventKind::BaseAntiAircraftFire {
            base,
            air_group,
            shots,
            ..
        } => cut_if(
            own_base(base),
            SideEventKind::BaseAntiAircraftFire {
                base: *base,
                air_group: *air_group,
                shots: *shots,
            },
        ),

        // The attacker sees that it attacked; the defender sees the attack.
        EventKind::BaseAttacked {
            air_group,
            base,
            shots,
            ..
        } => {
            if own_base(base) {
                Seen::Whole
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

        // The searcher's own act if it sighted, as a sighting roll is. An
        // attempt that did not is hidden from everyone.
        EventKind::SearchAttempt {
            air_group, sighted, ..
        } => whole_if(*sighted && own_group(air_group)),
    }
}

/// The names of the factors of a cut event's breakdown that are the side's
/// own. Every other factor is the target's, a rule's or a mixture of both
/// sides' counts, and is dropped.
fn own_factors(kind: &SideEventKind) -> &'static [&'static str] {
    match kind {
        SideEventKind::Whole(_)
        | SideEventKind::SalvoAgainst { .. }
        | SideEventKind::AirAttackAgainst { .. } => &[],
        SideEventKind::Salvo { .. } => &[
            "guns",
            "tubes",
            "rate",
            "minutes",
            "main_battery_factor",
            "torpedo_mount_factor",
            "fire_control_factor",
        ],
        SideEventKind::AirAttack { .. } | SideEventKind::BaseAttacked { .. } => &["aircraft"],
        SideEventKind::AirFight { .. }
        | SideEventKind::AntiAircraftFire { .. }
        | SideEventKind::BaseAntiAircraftFire { .. } => &["shots"],
    }
}

fn side_breakdown(kind: &SideEventKind, breakdown: &Option<Breakdown>) -> Option<SideBreakdown> {
    let breakdown = breakdown.as_ref()?;
    if matches!(kind, SideEventKind::Whole(_)) {
        return Some(SideBreakdown {
            factors: breakdown.factors.clone(),
            result: Some(breakdown.result),
        });
    }
    let own = own_factors(kind);
    let factors: Vec<Factor> = breakdown
        .factors
        .iter()
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
    let visible: BTreeSet<EventId> = log
        .iter()
        .zip(&seen)
        .filter(|(_, seen)| **seen != Seen::Nothing)
        .map(|(event, _)| event.id)
        .collect();
    log.iter()
        .zip(seen)
        .filter_map(|(event, seen)| {
            let kind = match seen {
                Seen::Nothing => return None,
                Seen::Whole => SideEventKind::Whole(event.kind.clone()),
                Seen::Cut(kind) => kind,
            };
            let mut causes = Vec::new();
            let mut hidden_causes = false;
            for cause in &event.causes {
                let keep = match cause {
                    Cause::Order(order) => owners.orders.get(order) == Some(&side),
                    Cause::Event(earlier) => visible.contains(earlier),
                };
                if keep {
                    causes.push(*cause);
                } else {
                    hidden_causes = true;
                }
            }
            let breakdown = side_breakdown(&kind, &event.breakdown);
            Some(SideEvent {
                id: event.id,
                day: event.day,
                pulse: event.pulse,
                kind,
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
        let mut owners = Owners::default();
        for (n, side) in [(1, one), (2, two)] {
            owners.ships.insert(ship(n), side);
            owners.forces.insert(force(n), side);
            owners.air_groups.insert(group(n), side);
            owners.bases.insert(base(n), side);
            owners.orders.insert(order(n), side);
        }
        owners
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

    // ---- the table of the 60 kinds ----

    /// What a side sees of an event.
    #[derive(Clone, Copy, Debug)]
    enum Expect {
        Whole,
        Nothing,
        /// The cut variant's name and the fields of the full kind it lacks.
        Cut(&'static str, &'static [&'static str]),
    }
    use Expect::{Cut, Nothing, Whole};

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

    const SALVO_CUT: &[&str] = &["chance", "expected", "hits"];

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
            // A search attempt that sighted is the searcher's own act; one
            // that did not is hidden from everyone.
            row(
                EventKind::SearchAttempt {
                    air_group: group(1),
                    task_force: force(2),
                    distance_nm: fx(200),
                    chance: Fixed::from_ratio(1, 2),
                    sighted: true,
                },
                Whole,
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
                Whole,
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
                Whole,
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
                Cut("Salvo", SALVO_CUT),
                Cut("SalvoAgainst", &["shots", "chance", "expected", "hits"]),
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
                Whole,
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
            // Begun and ended are shown, whole, to a side that has come to
            // know of the fight. Both did here: one fired and one was fired
            // on in round 1.
            Row {
                log: known_fight(),
                at: 0,
                allied: Whole,
                japanese: Whole,
                mirrors: true,
            },
            Row {
                log: known_fight(),
                at: 2,
                allied: Whole,
                japanese: Whole,
                mirrors: true,
            },
            sided(row(
                EventKind::ClaimReceived {
                    side: Side::Allied,
                    hex: hex(),
                    claimed_sunk: 1,
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
                Whole,
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
                Cut("AirAttack", SALVO_CUT),
                Cut(
                    "AirAttackAgainst",
                    &["weapon", "chance", "expected", "hits"],
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
                },
                Cut("AirFight", SALVO_CUT),
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
                },
                Cut("AntiAircraftFire", SALVO_CUT),
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
                },
                Cut("BaseAntiAircraftFire", SALVO_CUT),
                Nothing,
            ),
            // The attacker sees that it attacked; the defender sees the raid.
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
                Whole,
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
        assert_eq!(all.len(), 60, "the log has 60 kinds of event");
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
            Cut(variant, removed) => {
                assert_eq!(here.len(), 1, "{who} does not see a cut {name}");
                let shown = format!("{:?}", here[0].kind);
                let shown_name: String = shown
                    .chars()
                    .take_while(|c| c.is_ascii_alphanumeric())
                    .collect();
                assert_eq!(shown_name, variant, "{who}");
                let full_fields = field_names(&format!("{:?}", full.kind));
                let cut_fields = field_names(&shown);
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
                target: ship(2),
                weapon: Weapon::Torpedo,
                shots: fx(8),
            }
        );
        assert_eq!(
            shown[0].to_string(),
            "#1 D1 night     round 2: ship:1 torpedoes at ship:2: 8 shots"
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
                EventKind::FuelBurned {
                    ship: ship(1),
                    task_force: force(1),
                    burned_t: fx(1),
                    remaining_t: fx(9),
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
            vec![Cause::Order(order(1)), Cause::Event(EventId::new(1))]
        );
        assert!(
            burned.hidden_causes,
            "the Japanese order and event were dropped"
        );
        // The Japanese side sees the same event of the log as nothing.
        let japanese = project(Side::Japanese, &events, &owners(false));
        assert!(japanese.iter().all(|e| e.id != EventId::new(3)));

        // An event with every cause visible says so.
        let clean = caused(events, 2, vec![Cause::Order(order(1))]);
        let allied = project(Side::Allied, &clean, &owners(false));
        let burned = allied
            .iter()
            .find(|e| e.id == EventId::new(3))
            .expect("seen");
        assert!(!burned.hidden_causes);
    }

    // ---- breakdowns ----

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
        assert!(matches!(
            japanese[2].kind,
            SideEventKind::Salvo { round: 3, .. }
        ));
        assert!(japanese
            .iter()
            .filter(|e| e.id.raw() != 1 && e.id.raw() != 8)
            .all(|e| matches!(e.kind, SideEventKind::Salvo { round: 3, .. })
                || matches!(
                    &e.kind,
                    SideEventKind::Whole(EventKind::SightingRoll { round: 3, .. })
                )));
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
        assert_eq!(japanese[2].causes, vec![Cause::Event(EventId::new(4))]);
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
        assert_eq!(allied[0].causes, vec![Cause::Order(order(1))]);
        assert!(allied[0].hidden_causes);
        let japanese = project(Side::Japanese, &events, &owners);
        assert_eq!(japanese[0].causes, vec![Cause::Order(order(2))]);
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
}
