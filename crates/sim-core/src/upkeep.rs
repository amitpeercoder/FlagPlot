//! Wear, repair and operational losses (decision
//! `docs/decisions/0035-wear-repair-and-operational-losses.md`), and fatigue
//! and rest (decision `docs/decisions/0036-fatigue-and-rest.md`).
//!
//! # What flew
//!
//! The search step and the air combat step note what each air group flew in
//! the pulse as [`Sortie`]s, kept on the world until the wear step of the
//! same pulse takes them (they are not part of the state hash). They are noted
//! only when the scenario loaded the wear figures or the fatigue figures. A
//! sortie says which group, what kind, how many aircraft flew (`sent`) and,
//! once the air combat step has taken its losses off, how many came back, how
//! far the sortie was, and the events that say it flew:
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
//! # Fatigue (0036)
//!
//! [`World::wear_aircraft`] is the whole daylight upkeep step: it takes the
//! sorties, rolls wear if the wear figures are loaded, and then, if the
//! fatigue figures are loaded, works fatigue ([`World::tire_and_rest_air_groups`]),
//! so wear always comes first. Each air group with a position, in order of
//! group number, that flew at least one aircraft (a sortie with aircraft
//! `sent`, shot down or not) gains the fatigue figure for a sortie plus the
//! figure per 100 nm of its longest sortie, once, 100 at most; any other
//! loses the rest figure, 0 at least. `AirGroupFatigue` is recorded only when
//! the fatigue changes, citing the events that say the group flew, or, for a
//! group that rested, its latest stand-down if it is stood down and else its
//! mission order. No random number.
//!
//! [`World::stand_down_and_return_air_groups`] runs in the night pulse right
//! after the repair, the start of the day: each group with a rest threshold,
//! in order of group number, is stood down if its fatigue is above the
//! threshold and returned if it was stood down and its fatigue is not,
//! recording `AirGroupStoodDown` or `AirGroupReturned`, caused by the
//! threshold order, only on a change. A stood-down group flies nothing
//! (`World::aircraft_able_to_fly` is none), and its aircraft are on the
//! ground.
//!
//! # Replacements (0038)
//!
//! [`World::receive_replacements`] runs in the night pulse right after the
//! repair and before the stand-down decisions, from day 2 on (day 1 has
//! nothing to replace). For each of the scenario's streams, in the order
//! given, if the base is still held by the stream's side: its aircraft go to
//! the side's groups located at the base that fly the stream's type, lowest
//! group first, each up to its strength less the aircraft it has (ready and
//! damaged); its pilots go the same way, to groups whose pilots are tracked,
//! up to strength less pilots. A replacement aircraft arrives ready. What
//! cannot be placed is lost. `ReplacementsReceived` is recorded for each
//! group that receives anything, with no cause (nothing orders it). No
//! random number.
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
use crate::events::{Breakdown, Cause, EventKind, Factor, FatigueChange, SortieKind};
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
    /// The aircraft that flew, shot down or not (decision 4 of 0036: a group
    /// whose aircraft were all shot down still flew).
    pub(crate) sent: u32,
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

    /// The wear step of a daylight pulse (decisions 3 to 7 of 0035), and then
    /// the fatigue step (decisions 4 and 5 of 0036), each if its figures are
    /// loaded. See the module comment.
    pub(crate) fn wear_aircraft(&mut self, day: u32, pulse: Pulse) {
        let mut sorties = std::mem::take(&mut self.sorties);
        // Stable: a fighter group's patrol stays before its escorts.
        sorties.sort_by_key(|sortie| sortie.air_group);
        self.roll_wear(day, pulse, &sorties);
        self.tire_and_rest_air_groups(day, pulse, &sorties);
    }

    /// Whether the air and search steps note what each group flew: when the
    /// wear figures or the fatigue figures are loaded.
    pub(crate) fn notes_sorties(&self) -> bool {
        self.ref_data.air_upkeep.is_some() || self.ref_data.fatigue.is_some()
    }

    fn roll_wear(&mut self, day: u32, pulse: Pulse, sorties: &[Sortie]) {
        let Some(figures) = self.ref_data.air_upkeep else {
            return;
        };

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
                sortie.causes.clone(),
                breakdown,
            );
        }
    }

    /// The fatigue step of a daylight pulse (decisions 4 and 5 of 0036), after
    /// the wear step. Each air group with a position, in order of group
    /// number: if it flew an aircraft in the pulse (a sortie with aircraft
    /// sent, whether or not they came back) it gains the figure for a sortie
    /// plus the figure per 100 nm of its longest sortie, once, 100 at most;
    /// otherwise it loses the rest figure, 0 at least. An event only when the
    /// fatigue changes. No random number.
    fn tire_and_rest_air_groups(&mut self, day: u32, pulse: Pulse, sorties: &[Sortie]) {
        let Some(figures) = self.ref_data.fatigue else {
            return;
        };
        let full = Fixed::from_int(100);
        let ids: Vec<AirGroupId> = self.truth.air_groups.keys().copied().collect();
        for id in ids {
            let group = &self.truth.air_groups[&id];
            // A group aboard a ship that sank in the pulse has no position.
            if self.truth.air_group_hex(group).is_none() {
                continue;
            }
            let before = group.fatigue;
            let stood_down = group.stood_down;
            let mission_order = group.mission_order;
            // The longest of its sorties; the first of equal ones.
            let flown: Vec<&Sortie> = sorties
                .iter()
                .filter(|sortie| sortie.air_group == id && sortie.sent > 0)
                .collect();
            let longest = flown
                .iter()
                .fold(None::<&&Sortie>, |best, sortie| match best {
                    Some(best) if best.distance_nm >= sortie.distance_nm => Some(best),
                    _ => Some(sortie),
                });

            let (change, amount, after, causes, breakdown) = match longest {
                Some(longest) => {
                    let gain = figures.gain(longest.distance_nm);
                    let after = (before + gain).min(full);
                    let mut causes: Vec<Cause> = Vec::new();
                    for sortie in &flown {
                        for cause in &sortie.causes {
                            if !causes.contains(cause) {
                                causes.push(*cause);
                            }
                        }
                    }
                    let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                        factors: vec![
                            Factor {
                                name: "fatigue_before",
                                value: before,
                            },
                            Factor {
                                name: "air_fatigue_per_sortie",
                                value: figures.air_fatigue_per_sortie,
                            },
                            Factor {
                                name: "air_fatigue_per_100_nm",
                                value: figures.air_fatigue_per_100_nm,
                            },
                            Factor {
                                name: "distance_nm",
                                value: longest.distance_nm,
                            },
                            Factor {
                                name: "gain",
                                value: gain,
                            },
                        ],
                        result: after,
                    });
                    (
                        FatigueChange::Gained {
                            sortie: longest.kind,
                            distance_nm: longest.distance_nm,
                        },
                        after - before,
                        after,
                        causes,
                        breakdown,
                    )
                }
                None => {
                    let after = (before - figures.air_rest_per_pulse).max(Fixed::ZERO);
                    // A stood-down group rests because it was stood down; any
                    // other because it was not told to fly: its mission order
                    // is the nearest thing, and a group with none has no cause.
                    let cause = if stood_down {
                        self.events
                            .latest_where(|kind| {
                                matches!(kind, EventKind::AirGroupStoodDown {
                                    air_group, ..
                                } if *air_group == id)
                            })
                            .map(Cause::Event)
                    } else {
                        mission_order.map(Cause::Order)
                    };
                    let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                        factors: vec![
                            Factor {
                                name: "fatigue_before",
                                value: before,
                            },
                            Factor {
                                name: "air_rest_per_pulse",
                                value: figures.air_rest_per_pulse,
                            },
                        ],
                        result: after,
                    });
                    (
                        FatigueChange::Rested,
                        before - after,
                        after,
                        cause.into_iter().collect(),
                        breakdown,
                    )
                }
            };
            if after == before {
                continue;
            }
            self.truth
                .air_groups
                .get_mut(&id)
                .expect("the air group exists")
                .fatigue = after;
            self.events.record(
                day,
                Some(pulse),
                EventKind::AirGroupFatigue {
                    air_group: id,
                    change,
                    amount,
                    fatigue: after,
                },
                causes,
                breakdown,
            );
        }
    }

    /// The staff's rest step, in the night pulse right after the repair (the
    /// start of the day, decision 8 of 0036): each air group with a rest
    /// threshold, in order of group number, is stood down if its fatigue is
    /// above the threshold and returned to flying if it was stood down and
    /// its fatigue is not. An event only on a change, caused by the
    /// threshold order. Nothing happens without the fatigue figures.
    pub(crate) fn stand_down_and_return_air_groups(&mut self, day: u32, pulse: Pulse) {
        if self.ref_data.fatigue.is_none() {
            return;
        }
        let ids: Vec<AirGroupId> = self.truth.air_groups.keys().copied().collect();
        for id in ids {
            let group = self
                .truth
                .air_groups
                .get_mut(&id)
                .expect("the air group exists");
            let Some(threshold) = group.rest_above else {
                continue;
            };
            let above = group.fatigue > threshold;
            if above == group.stood_down {
                continue;
            }
            group.stood_down = above;
            let fatigue = group.fatigue;
            let causes: Vec<Cause> = group.rest_order.map(Cause::Order).into_iter().collect();
            let kind = if above {
                EventKind::AirGroupStoodDown {
                    air_group: id,
                    fatigue,
                    threshold,
                }
            } else {
                EventKind::AirGroupReturned {
                    air_group: id,
                    fatigue,
                    threshold,
                }
            };
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "fatigue",
                        value: fatigue,
                    },
                    Factor {
                        name: "rest_threshold",
                        value: threshold,
                    },
                ],
                result: fatigue,
            });
            self.events
                .record(day, Some(pulse), kind, causes, breakdown);
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

    /// The replacement step (decisions 6 to 8 and 10 of 0038). See the
    /// module comment.
    pub(crate) fn receive_replacements(&mut self, day: u32, pulse: Pulse) {
        // Day 1 has nothing to replace.
        if day < 2 {
            return;
        }
        let streams = self.truth.replacements.clone();
        for stream in streams {
            let held = self
                .truth
                .bases
                .get(&stream.base)
                .is_some_and(|base| base.side == stream.side);
            if !held {
                continue;
            }
            let ids: Vec<AirGroupId> = self
                .truth
                .air_groups
                .values()
                .filter(|group| {
                    group.location == AirGroupLocation::Base(stream.base)
                        && group.side == stream.side
                        && group.aircraft_type == stream.aircraft_type
                })
                .map(|group| group.id)
                .collect();
            let mut aircraft_left = stream.aircraft_per_day;
            let mut pilots_left = stream.pilots_per_day;
            for id in ids {
                let group = self
                    .truth
                    .air_groups
                    .get_mut(&id)
                    .expect("the air group exists");
                let aircraft =
                    aircraft_left.min(group.strength.saturating_sub(group.ready + group.damaged));
                let pilots = match group.pilots {
                    Some(have) => pilots_left.min(group.strength.saturating_sub(have)),
                    None => 0,
                };
                if aircraft == 0 && pilots == 0 {
                    continue;
                }
                aircraft_left -= aircraft;
                pilots_left -= pilots;
                group.ready += aircraft;
                if let Some(have) = group.pilots.as_mut() {
                    *have += pilots;
                }
                let (ready, pilots_now) = (group.ready, group.pilots.unwrap_or(0));
                self.events.record(
                    day,
                    Some(pulse),
                    EventKind::ReplacementsReceived {
                        air_group: id,
                        base: stream.base,
                        aircraft,
                        pilots,
                        ready,
                        pilots_now,
                    },
                    Vec::new(),
                    None,
                );
            }
        }
    }
}
