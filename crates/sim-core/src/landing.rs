//! Aircraft in the air when their own deck cannot take them (decisions 7 to
//! 10 of `docs/decisions/0059-aircraft-that-cannot-land-where-they-took-off.md`).
//!
//! # Which aircraft, and when
//!
//! In a daylight pulse the aircraft a group flew and that were not shot down
//! are in the air until the wear phase: they are the group's noted sorties
//! (`World::sorties`), the aircraft that came back. A group aboard a ship
//! that sinks in the air combat step keeps exactly those, as `ready`
//! aircraft, and loses the rest with the ship (decision 10; see
//! `World::sink_ship`). A flight deck that goes out in the step is noted
//! too. The wear phase then, after the fuel roll of decision 3 and before
//! wear, brings them home ([`World::come_home`]).
//!
//! # Where they go
//!
//! For each group in group number order whose ship has sunk or whose flight
//! deck is out (a degraded deck takes them; a group at a base is never
//! affected), the aircraft that are left in its sorties after the fuel roll,
//! as one body, go in this order:
//!
//! 1. Another carrier (a ship of the carrier kind) of the group's side in the same hex (the hex of its
//!    ship, or where it sank), afloat, flight deck not out, lowest ship
//!    number first, as many as fit under her aircraft capacity (the ready
//!    and damaged aircraft of the groups aboard her at that moment), the rest
//!    going on to the next.
//! 2. A base of its side with an airfield (decision 6 of 0099) within the
//!    type's strike radius of that hex (the
//!    search range for a type with no strike radius, as some scouts are): the
//!    nearest, then the lowest number. A base takes any number.
//! 3. The sea: the aircraft are ditched, and where pilots are counted each
//!    crew is recovered with `crew_recovered_beside_own_force` if a ship of
//!    its side is afloat in the hex and else `crew_recovered_open_sea`. The
//!    draws are one stream of the upkeep subsystem with [`Draw::NoDeck`],
//!    keyed on the group, one for each ditched aircraft.
//!
//! Aircraft that land join the lowest-numbered group of their type and side
//! at that ship or base, or form a new group there. Their sortie goes with
//! them, so wear, fatigue and supply are worked for the group that has them.

use std::collections::BTreeSet;

use crate::damage::roll_shots;
use crate::events::{Breakdown, Cause, EventKind, Factor};
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{AirGroupId, BaseId, EntityRef, EventId, ShipId};
use crate::orders::{AirMission, Reach};
use crate::refdata::ShipKind;
use crate::rng::{Draw, Subsystem};
use crate::state::{AirGroup, AirGroupLocation, ShipLocation, ShipSystem, SystemState};
use crate::time::Pulse;
use crate::upkeep::Sortie;
use crate::world::World;

/// A ship that lost its deck in this pulse's air combat step: where, and the
/// event that did it (the sinking, or the system hit that put the deck out).
#[derive(Clone, Copy, Debug)]
pub(crate) struct DeckLoss {
    pub(crate) ship: ShipId,
    pub(crate) hex: Hex,
    pub(crate) cause: EventId,
}

impl World {
    /// The hex of a ship afloat: its task force's position, or its port's.
    pub(crate) fn ship_hex(&self, ship: ShipId) -> Option<Hex> {
        match self.truth.ships.get(&ship)?.location {
            ShipLocation::TaskForce(force) => {
                self.truth.task_forces.get(&force).map(|f| f.position())
            }
            ShipLocation::Port(base) => self.truth.bases.get(&base).map(|b| b.hex),
            // Off the map while at the yard (decision 0069).
            ShipLocation::Sunk | ShipLocation::Yard(_) => None,
        }
    }

    /// Whether the flight deck of `ship` cannot take aircraft: she has
    /// sunk, or her deck is out. A degraded deck takes them.
    fn deck_is_lost(&self, ship: ShipId) -> bool {
        match self.truth.ships.get(&ship) {
            None => false,
            Some(ship) => {
                ship.location == ShipLocation::Sunk
                    || ship.systems.get(ShipSystem::FlightDeck) == SystemState::Out
            }
        }
    }

    /// The aircraft aboard `ship`: the ready and damaged aircraft of every
    /// group aboard her.
    fn aircraft_aboard(&self, ship: ShipId) -> u32 {
        self.truth
            .air_groups
            .values()
            .filter(|group| group.location == AirGroupLocation::Ship(ship))
            .map(|group| group.ready + group.damaged)
            .sum()
    }

    /// Brings home the aircraft in the air whose own deck is lost. See the
    /// module comment. Run in the wear phase after the fuel roll and before
    /// wear.
    pub(crate) fn come_home(&mut self, day: u32, pulse: Pulse, sorties: &mut Vec<Sortie>) {
        let Some(figures) = self.ref_data.air_combat.clone() else {
            return;
        };
        let groups: BTreeSet<AirGroupId> = sorties
            .iter()
            .filter(|sortie| sortie.aircraft > 0)
            .map(|sortie| sortie.air_group)
            .collect();
        for id in groups {
            let group = self.truth.air_groups[&id].clone();
            let AirGroupLocation::Ship(own_ship) = group.location else {
                continue;
            };
            if !self.deck_is_lost(own_ship) {
                continue;
            }
            // The last noted: a ship whose deck went out and who then sank is
            // closed by her sinking.
            let loss = self.deck_losses.iter().rfind(|loss| loss.ship == own_ship);
            let Some(hex) = loss
                .map(|loss| loss.hex)
                .or_else(|| self.ship_hex(own_ship))
            else {
                continue;
            };
            let closing: Vec<Cause> = loss
                .map(|loss| Cause::Event(loss.cause))
                .into_iter()
                .collect();
            let mut causes = closing.clone();
            for sortie in sorties.iter().filter(|s| s.air_group == id) {
                for cause in &sortie.causes {
                    if !causes.contains(cause) {
                        causes.push(*cause);
                    }
                }
            }
            let in_air: u32 = sorties
                .iter()
                .filter(|s| s.air_group == id)
                .map(|s| s.aircraft)
                .sum();
            if in_air == 0 {
                continue;
            }
            let aircraft_type = self.ref_data.aircraft_types[&group.aircraft_type].clone();
            let radius = if aircraft_type.strike_range_nm > Fixed::ZERO {
                aircraft_type.strike_range_nm
            } else {
                aircraft_type.search_range_nm
            };

            // Where they go, in order.
            let mut places: Vec<(AirGroupLocation, u32)> = Vec::new();
            let mut remaining = in_air;
            // 1. Another carrier of the side in the same hex.
            let carriers: Vec<ShipId> = self
                .truth
                .ships
                .values()
                .filter(|ship| {
                    ship.side == group.side
                        && ship.id != own_ship
                        && self
                            .ref_data
                            .ship_classes
                            .get(&ship.class)
                            .is_some_and(|class| class.kind == ShipKind::Carrier)
                        && ship.location != ShipLocation::Sunk
                        && ship.systems.get(ShipSystem::FlightDeck) != SystemState::Out
                        && self.ship_hex(ship.id) == Some(hex)
                })
                .map(|ship| ship.id)
                .collect();
            let mut taken_by: Vec<(ShipId, u32)> = Vec::new();
            for carrier in carriers {
                if remaining == 0 {
                    break;
                }
                let capacity = self
                    .ref_data
                    .ship_classes
                    .get(&self.truth.ships[&carrier].class)
                    .map_or(0, |class| class.aircraft_capacity);
                let already: u32 = taken_by
                    .iter()
                    .filter(|(ship, _)| *ship == carrier)
                    .map(|(_, n)| n)
                    .sum();
                let room = capacity.saturating_sub(self.aircraft_aboard(carrier) + already);
                let take = room.min(remaining);
                if take > 0 {
                    places.push((AirGroupLocation::Ship(carrier), take));
                    taken_by.push((carrier, take));
                    remaining -= take;
                }
            }
            // 2. A base of the side within the radius: the nearest, then the
            // lowest number.
            if remaining > 0 {
                let base: Option<BaseId> = self
                    .truth
                    .bases
                    .values()
                    .filter(|base| {
                        // Not a base with no airfield (decision 6 of 0099).
                        base.side == group.side
                            && base.airfield
                            && hex.air_distance_within(base.hex, radius)
                    })
                    .min_by_key(|base| (hex.air_distance_nm(base.hex), base.id))
                    .map(|base| base.id);
                if let Some(base) = base {
                    places.push((AirGroupLocation::Base(base), remaining));
                    remaining = 0;
                }
            }
            // Land them, in order, taking each from the group's sorties.
            for (place, n) in places {
                let moved_pilots = if group.pilots.is_some() {
                    n.min(self.truth.air_groups[&id].pilots.unwrap_or(0))
                } else {
                    0
                };
                let (to, formed) = self.receiving_group(&group, place, n, moved_pilots);
                {
                    let source = self
                        .truth
                        .air_groups
                        .get_mut(&id)
                        .expect("the source group");
                    source.ready -= n;
                    if let Some(pilots) = source.pilots {
                        source.pilots = Some(pilots - moved_pilots);
                    }
                }
                self.events.record(
                    day,
                    Some(pulse),
                    EventKind::AircraftLandedElsewhere {
                        from: id,
                        to,
                        at: place,
                        aircraft: n,
                        pilots: moved_pilots,
                        formed,
                    },
                    causes.clone(),
                    None,
                );
                move_sorties(sorties, id, to, n, true);
            }
            // 3. The sea.
            if remaining > 0 {
                let beside = self.truth.ships.values().any(|ship| {
                    ship.side == group.side
                        && ship.location != ShipLocation::Sunk
                        && self.ship_hex(ship.id) == Some(hex)
                });
                let chance = if beside {
                    figures.crew_recovered_beside_own_force
                } else {
                    figures.crew_recovered_open_sea
                };
                let counted = self.truth.air_groups[&id].pilots;
                let crews_recovered = counted.map(|_| {
                    let mut stream = self.rng(
                        day,
                        Some(pulse),
                        Subsystem::Upkeep,
                        Draw::NoDeck,
                        EntityRef::AirGroup(id),
                    );
                    roll_shots(&mut stream, Fixed::from_int(i64::from(remaining)), chance)
                });
                let source = self
                    .truth
                    .air_groups
                    .get_mut(&id)
                    .expect("the source group");
                source.ready -= remaining;
                let pilots_lost = match (source.pilots, crews_recovered) {
                    (Some(pilots), Some(recovered)) => {
                        let lost = (remaining - recovered).min(pilots);
                        source.pilots = Some(pilots - lost);
                        lost
                    }
                    _ => 0,
                };
                let (ready, pilots_left) = (source.ready, source.pilots.unwrap_or(0));
                let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                    factors: vec![
                        Factor {
                            name: "aircraft",
                            value: Fixed::from_int(i64::from(remaining)),
                        },
                        Factor {
                            name: "crew_recovered_chance",
                            value: chance,
                        },
                        Factor {
                            name: "ships_of_its_side_in_the_hex",
                            value: Fixed::from_int(i64::from(beside)),
                        },
                    ],
                    result: Fixed::from_int(i64::from(remaining)) * chance,
                });
                let ditched = self.events.record(
                    day,
                    Some(pulse),
                    EventKind::AircraftDitchedNoDeck {
                        air_group: id,
                        hex,
                        aircraft: remaining,
                        beside_own_force: beside,
                        chance,
                        crews_recovered,
                        pilots_lost,
                        ready,
                    },
                    causes.clone(),
                    breakdown,
                );
                if pilots_lost > 0 {
                    self.events.record(
                        day,
                        Some(pulse),
                        EventKind::PilotsLost {
                            air_group: id,
                            lost: pilots_lost,
                            left: pilots_left,
                        },
                        vec![Cause::Event(ditched)],
                        None,
                    );
                }
                move_sorties(sorties, id, id, remaining, false);
            }
        }
        // Their sorties went with them: keep the order wear works in.
        sorties.sort_by_key(|sortie| sortie.air_group);
    }

    /// The group that receives `n` aircraft (and `pilots` pilots) of `source`
    /// at `place`: the lowest-numbered group of the type and side located
    /// there, which has its aircraft and strength raised, or a new group
    /// formed there. Returns it and whether it was formed.
    fn receiving_group(
        &mut self,
        source: &AirGroup,
        place: AirGroupLocation,
        n: u32,
        pilots: u32,
    ) -> (AirGroupId, bool) {
        let existing = self
            .truth
            .air_groups
            .values()
            .find(|g| {
                g.location == place
                    && g.side == source.side
                    && g.aircraft_type == source.aircraft_type
            })
            .map(|g| g.id);
        if let Some(id) = existing {
            let group = self.truth.air_groups.get_mut(&id).expect("the group");
            group.ready += n;
            group.strength += n;
            if let Some(counted) = group.pilots {
                group.pilots = Some(counted + pilots);
            }
            return (id, false);
        }
        let id = self.made_ids.alloc_air_group(source.side);
        let mut group = AirGroup::new(
            id,
            format!("{} landed at {}", source.name, place),
            source.side,
            place,
            source.aircraft_type,
        );
        group.ready = n;
        group.strength = n;
        group.fatigue = source.fatigue;
        group.mission = AirMission::Rest;
        group.reach = Reach::Normal;
        group.pilots = source.pilots.map(|_| pilots);
        self.truth.air_groups.insert(id, group);
        (id, true)
    }
}

/// Takes `n` aircraft off the sorties of `from`, in order. If `landed` they
/// go with their sortie to the group `to` (the sortie is split, and its
/// aircraft that flew count for the group that has them); otherwise they are
/// gone (ditched) and the sortie keeps its `sent`.
fn move_sorties(sorties: &mut Vec<Sortie>, from: AirGroupId, to: AirGroupId, n: u32, landed: bool) {
    let mut left = n;
    let mut added: Vec<Sortie> = Vec::new();
    for sortie in sorties.iter_mut().filter(|s| s.air_group == from) {
        if left == 0 {
            break;
        }
        let take = left.min(sortie.aircraft);
        if take == 0 {
            continue;
        }
        sortie.aircraft -= take;
        left -= take;
        if landed {
            sortie.sent = sortie.sent.saturating_sub(take);
            added.push(Sortie {
                air_group: to,
                kind: sortie.kind,
                aircraft: take,
                sent: take,
                distance_nm: sortie.distance_nm,
                causes: sortie.causes.clone(),
            });
        }
    }
    sorties.extend(added);
}
