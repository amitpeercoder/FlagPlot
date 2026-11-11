//! Wear, repair and operational losses (decision
//! `docs/decisions/0035-wear-repair-and-operational-losses.md`).
//!
//! # What flew
//!
//! The search step and the air combat step note what each air group flew in
//! the pulse as [`Sortie`]s, kept on the world until the wear step of the
//! same pulse takes them (they are not part of the state hash). They are noted
//! only when the scenario loaded the wear figures. A sortie says which group,
//! what kind, how many aircraft flew and, once the air combat step has taken
//! its losses off, how many came back, how far the sortie was, and the events
//! that say it flew:
//!
//! - a search: the aircraft that flew it, twice the range of its arc;
//! - a strike at a contact or a base: the aircraft launched, less those shot
//!   down, twice the distance from the group to the hex it was sent to;
//! - a patrol: the aircraft a fighter group kept over its hex at the start of
//!   the air combat step (its share, or all it can fly where its escorts do
//!   not go, decision 0030), less those shot down, no distance. A patrol is
//!   noted in every daylight pulse, raid or not;
//! - escorts: the aircraft a fighter group sent with a raid, less those shot
//!   down, twice the distance from the group to the raid's hex.
//!
//! # The wear step
//!
//! [`World::wear_aircraft`] runs in a daylight pulse after the air combat
//! step. For each sortie, in order of air group (a fighter group's patrol
//! before its escorts), the aircraft that came back are each rolled at the
//! wear chance ([`AirUpkeepFigures::wear_chance`]) and each worn aircraft is
//! rolled again at the write-off share, all from one stream of the upkeep
//! subsystem keyed on the air group. The worn leave `ready`; those not written
//! off join `damaged`. A group aboard a ship that sank in the pulse, which has
//! no position, rolls nothing. A sortie of no aircraft is not rolled and
//! records nothing; any other records `AircraftWorn`, whether or not an
//! aircraft was worn.
//!
//! # Repair
//!
//! [`World::repair_aircraft`] runs first in the night pulse, the start of the
//! game day (decision 9 of 0035): each base in order repairs up to its figure
//! of the damaged aircraft of the groups located there, the lowest group
//! first, then each ship afloat that carries groups repairs up to the carrier
//! figure in the same order. A repaired aircraft is ready. No random number.

use std::collections::{BTreeMap, BTreeSet};

use crate::damage::roll_shots;
use crate::events::{Breakdown, Cause, EventKind, Factor, SortieKind};
use crate::fixed::Fixed;
use crate::ids::{AirGroupId, EntityRef};
use crate::rng::{RngStream, Subsystem};
use crate::state::{AirGroupLocation, ShipLocation};
use crate::time::Pulse;
use crate::world::World;

/// What one air group flew in the pulse, for the wear step.
#[derive(Clone, Debug)]
pub(crate) struct Sortie {
    pub(crate) air_group: AirGroupId,
    pub(crate) kind: SortieKind,
    /// The aircraft that flew and came back: those that flew, less those
    /// the air combat step shot down.
    pub(crate) aircraft: u32,
    pub(crate) distance_nm: Fixed,
    /// The events or orders that say the group flew.
    pub(crate) causes: Vec<Cause>,
}

impl World {
    /// Takes `lost` off the sortie of `kind` (a strike of either kind, if
    /// `kind` is [`SortieKind::Strike`]) that `group` flew, if one is noted.
    pub(crate) fn note_loss(&mut self, group: AirGroupId, kind: SortieKind, lost: u32) {
        let matches = |sortie: &Sortie| {
            sortie.air_group == group
                && match kind {
                    SortieKind::Strike | SortieKind::BaseStrike => {
                        matches!(sortie.kind, SortieKind::Strike | SortieKind::BaseStrike)
                    }
                    other => sortie.kind == other,
                }
        };
        if let Some(sortie) = self.sorties.iter_mut().find(|sortie| matches(sortie)) {
            sortie.aircraft = sortie.aircraft.saturating_sub(lost);
        }
    }

    /// The wear step of a daylight pulse (decisions 3 to 7 of 0035). See the
    /// module comment.
    pub(crate) fn wear_aircraft(&mut self, day: u32, pulse: Pulse) {
        let mut sorties = std::mem::take(&mut self.sorties);
        let Some(figures) = self.ref_data.air_upkeep else {
            return;
        };
        // Stable: a fighter group's patrol stays before its escorts.
        sorties.sort_by_key(|sortie| sortie.air_group);

        let mut streams: BTreeMap<AirGroupId, RngStream> = BTreeMap::new();
        for sortie in sorties {
            if sortie.aircraft == 0 {
                continue;
            }
            let Some(group) = self.truth.air_groups.get(&sortie.air_group) else {
                continue;
            };
            // A group aboard a ship that sank in the pulse has no position
            // and rolls nothing.
            if self.truth.air_group_hex(group).is_none() {
                continue;
            }
            let chance = figures.wear_chance(sortie.distance_nm);
            let flew = Fixed::from_int(i64::from(sortie.aircraft));
            let expected = flew * chance;
            let stream = streams.entry(sortie.air_group).or_insert_with(|| {
                self.rng(
                    day,
                    Some(pulse),
                    Subsystem::Upkeep,
                    EntityRef::AirGroup(sortie.air_group),
                )
            });
            let worn = roll_shots(stream, flew, chance);
            let written_off = roll_shots(
                stream,
                Fixed::from_int(i64::from(worn)),
                figures.write_off_share,
            );

            let group = self
                .truth
                .air_groups
                .get_mut(&sortie.air_group)
                .expect("the air group exists");
            group.ready -= worn;
            group.damaged += worn - written_off;
            let (ready, damaged) = (group.ready, group.damaged);
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "aircraft",
                        value: flew,
                    },
                    Factor {
                        name: "wear_base_chance",
                        value: figures.wear_base_chance,
                    },
                    Factor {
                        name: "wear_chance_per_100_nm",
                        value: figures.wear_chance_per_100_nm,
                    },
                    Factor {
                        name: "distance_nm",
                        value: sortie.distance_nm,
                    },
                    Factor {
                        name: "chance",
                        value: chance,
                    },
                    Factor {
                        name: "write_off_share",
                        value: figures.write_off_share,
                    },
                ],
                result: expected,
            });
            self.events.record(
                day,
                Some(pulse),
                EventKind::AircraftWorn {
                    air_group: sortie.air_group,
                    sortie: sortie.kind,
                    distance_nm: sortie.distance_nm,
                    aircraft: sortie.aircraft,
                    chance,
                    expected,
                    worn,
                    written_off,
                    ready,
                    damaged,
                },
                sortie.causes,
                breakdown,
            );
        }
    }

    /// The repair step, first in the night pulse (decisions 9 and 10 of 0035).
    /// See the module comment.
    pub(crate) fn repair_aircraft(&mut self, day: u32, pulse: Pulse) {
        let Some(figures) = self.ref_data.air_upkeep else {
            return;
        };
        // Places in order: bases by number, then ships by number that carry a
        // group and are afloat, each with what it can repair.
        let mut places: Vec<(AirGroupLocation, u32)> = self
            .truth
            .bases
            .values()
            .filter(|base| base.aircraft_repairs_per_day > 0)
            .map(|base| {
                (
                    AirGroupLocation::Base(base.id),
                    base.aircraft_repairs_per_day,
                )
            })
            .collect();
        if figures.carrier_repairs_per_day > 0 {
            let carriers: BTreeSet<_> = self
                .truth
                .air_groups
                .values()
                .filter_map(|group| match group.location {
                    AirGroupLocation::Ship(ship) => Some(ship),
                    AirGroupLocation::Base(_) => None,
                })
                .filter(|ship| {
                    self.truth
                        .ships
                        .get(ship)
                        .is_some_and(|ship| ship.location != ShipLocation::Sunk)
                })
                .collect();
            places.extend(carriers.into_iter().map(|ship| {
                (
                    AirGroupLocation::Ship(ship),
                    figures.carrier_repairs_per_day,
                )
            }));
        }

        for (place, mut left) in places {
            let groups: Vec<AirGroupId> = self
                .truth
                .air_groups
                .values()
                .filter(|group| group.location == place && group.damaged > 0)
                .map(|group| group.id)
                .collect();
            for id in groups {
                if left == 0 {
                    break;
                }
                let group = self
                    .truth
                    .air_groups
                    .get_mut(&id)
                    .expect("the air group exists");
                let repaired = left.min(group.damaged);
                left -= repaired;
                group.damaged -= repaired;
                group.ready += repaired;
                let (ready, damaged) = (group.ready, group.damaged);
                // The latest wear that left aircraft damaged says why there
                // are any. Aircraft damaged from the start have no cause.
                let causes: Vec<Cause> = self
                    .events
                    .latest_where(|kind| {
                        matches!(kind, EventKind::AircraftWorn {
                            air_group, worn, written_off, ..
                        } if *air_group == id && worn > written_off)
                    })
                    .map(Cause::Event)
                    .into_iter()
                    .collect();
                self.events.record(
                    day,
                    Some(pulse),
                    EventKind::AircraftRepaired {
                        air_group: id,
                        at: place,
                        repaired,
                        ready,
                        damaged,
                    },
                    causes,
                    None,
                );
            }
        }
    }
}
