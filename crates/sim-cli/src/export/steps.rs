//! The state at each step of the replay (replay version 4, decision 0108,
//! item 7): the viewer draws it instead of working it out again from the
//! events.
//!
//! A step is a group of the viewer's `buildSteps`: the day-0 events as
//! "start", then for each day `orders`, `night`, `morning`, `afternoon` and
//! `end`. The state is recorded at the end of each, for every day and every
//! phase whether or not the phase has events, at these moments of
//! [`World::resolve_day_watched`](sim_core::World::resolve_day_watched):
//!
//! | step | moment |
//! | --- | --- |
//! | `start` | `DayBegins { day: 1 }` (the final state if there is no day) |
//! | `(d, orders)` | `PulseBegins { day: d, pulse: Night }` |
//! | `(d, night)` | `PulseBegins { day: d, pulse: Morning }` |
//! | `(d, morning)` | `PulseBegins { day: d, pulse: Afternoon }` |
//! | `(d, afternoon)` | the first `EndOfDay { day: d, .. }` |
//! | `(d, end)` | `DayBegins { day: d + 1 }`, or the world after the last day |
//!
//! The end of day `d` is the moment the next day begins: nothing happens
//! between the day's last phase and the next day's first moment. A state
//! change always records an event, and these moments fall between the
//! functions of the day, so the state at each moment is the state after the
//! step's last event. (A contact growing older is the one change that is not
//! an event of its own kind for every step; it is recorded at the moment all
//! the same.)
//!
//! The first entry carries every record of the start state; each later entry
//! carries only the records whose JSON text differs from the same record in
//! the entry before (`changed`) and the keys of records that are gone
//! (`removed`). A full copy at every step would be about 80 KB a step.

use std::collections::BTreeMap;

use serde::Serialize;
use serde_json::value::RawValue;
use sim_core::choices::Moment;
use sim_core::time::Pulse;
use sim_core::World;

use super::state::{self, KeyedRecords, PerSide};

/// What one entry lists of each kind of record: the records themselves in
/// `changed`, their keys in `removed`. A kind with nothing is left out.
#[derive(Serialize)]
struct Lists<R, E> {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    task_forces: Vec<R>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    ships: Vec<R>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    air_groups: Vec<R>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    bases: Vec<R>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    land_units: Vec<R>,
    #[serde(skip_serializing_if = "PerSide::is_empty")]
    contacts: PerSide<R>,
    #[serde(skip_serializing_if = "PerSide::is_empty")]
    ground_estimates: PerSide<E>,
}

impl<R, E> Lists<R, E> {
    fn is_empty(&self) -> bool {
        self.task_forces.is_empty()
            && self.ships.is_empty()
            && self.air_groups.is_empty()
            && self.bases.is_empty()
            && self.land_units.is_empty()
            && self.contacts.is_empty()
            && self.ground_estimates.is_empty()
    }
}

#[derive(Serialize)]
pub struct StepOut {
    day: u32,
    phase: &'static str,
    #[serde(skip_serializing_if = "Lists::is_empty")]
    changed: Lists<Box<RawValue>, Box<RawValue>>,
    #[serde(skip_serializing_if = "Lists::is_empty")]
    removed: Lists<u32, (i32, i32)>,
}

/// The records of one kind as the last step left them, by key.
struct Seen<K: Ord>(BTreeMap<K, String>);

impl<K: Ord + Copy> Seen<K> {
    /// The records of `now` that differ from the last step's, as JSON, in
    /// the order given, and the keys no longer there, in key order. Leaves
    /// `now` as the last step's.
    fn diff(&mut self, now: Vec<(K, String)>) -> (Vec<Box<RawValue>>, Vec<K>) {
        let mut changed = Vec::new();
        let mut kept = BTreeMap::new();
        for (key, text) in now {
            if self.0.get(&key) != Some(&text) {
                changed.push(RawValue::from_string(text.clone()).expect("a record is JSON"));
            }
            kept.insert(key, text);
        }
        let removed = self
            .0
            .keys()
            .filter(|key| !kept.contains_key(key))
            .copied()
            .collect();
        self.0 = kept;
        (changed, removed)
    }
}

/// Records the state at each step of a run, as the day resolves. Hand
/// [`StepRecorder::watch`] to the run as its watcher and call
/// [`StepRecorder::finish`] with the world after the last day.
pub struct StepRecorder {
    steps: Vec<StepOut>,
    day: u32,
    afternoon_taken: bool,
    task_forces: Seen<u32>,
    ships: Seen<u32>,
    air_groups: Seen<u32>,
    bases: Seen<u32>,
    land_units: Seen<u32>,
    contacts: [Seen<u32>; 2],
    estimates: [Seen<(i32, i32)>; 2],
}

impl StepRecorder {
    pub fn new() -> StepRecorder {
        StepRecorder {
            steps: Vec::new(),
            day: 0,
            afternoon_taken: false,
            task_forces: Seen(BTreeMap::new()),
            ships: Seen(BTreeMap::new()),
            air_groups: Seen(BTreeMap::new()),
            bases: Seen(BTreeMap::new()),
            land_units: Seen(BTreeMap::new()),
            contacts: [Seen(BTreeMap::new()), Seen(BTreeMap::new())],
            estimates: [Seen(BTreeMap::new()), Seen(BTreeMap::new())],
        }
    }

    /// The watcher of `World::resolve_day_watched`.
    pub fn watch(&mut self, world: &World, moment: Moment) {
        match moment {
            Moment::DayBegins { day } => {
                if day == 1 {
                    self.record(0, "start", world);
                } else {
                    self.record(day - 1, "end", world);
                }
                self.day = day;
                self.afternoon_taken = false;
            }
            Moment::PulseBegins { day, pulse } => {
                let phase = match pulse {
                    Pulse::Night => "orders",
                    Pulse::Morning => "night",
                    Pulse::Afternoon => "morning",
                };
                self.record(day, phase, world);
            }
            Moment::EndOfDay { day, .. } => {
                if !self.afternoon_taken {
                    self.afternoon_taken = true;
                    self.record(day, "afternoon", world);
                }
            }
            Moment::Orders { .. } | Moment::Phase { .. } => {}
        }
    }

    /// Records the last step, from the world after the last day (or the
    /// start, when no day was resolved), and returns the steps.
    pub fn finish(mut self, world: &World) -> Vec<StepOut> {
        if self.steps.is_empty() {
            self.record(0, "start", world);
        } else {
            let day = self.day;
            self.record(day, "end", world);
        }
        self.steps
    }

    fn record(&mut self, day: u32, phase: &'static str, world: &World) {
        let KeyedRecords {
            task_forces,
            ships,
            air_groups,
            bases,
            land_units,
            contacts,
            ground_estimates,
        } = state::records(world).keyed();
        let (task_forces_changed, task_forces_removed) = self.task_forces.diff(task_forces);
        let (ships_changed, ships_removed) = self.ships.diff(ships);
        let (air_groups_changed, air_groups_removed) = self.air_groups.diff(air_groups);
        let (bases_changed, bases_removed) = self.bases.diff(bases);
        let (land_units_changed, land_units_removed) = self.land_units.diff(land_units);
        let (allied_contacts, allied_contacts_gone) = self.contacts[0].diff(contacts.allied);
        let (japanese_contacts, japanese_contacts_gone) = self.contacts[1].diff(contacts.japanese);
        let (allied_estimates, allied_estimates_gone) =
            self.estimates[0].diff(ground_estimates.allied);
        let (japanese_estimates, japanese_estimates_gone) =
            self.estimates[1].diff(ground_estimates.japanese);
        self.steps.push(StepOut {
            day,
            phase,
            changed: Lists {
                task_forces: task_forces_changed,
                ships: ships_changed,
                air_groups: air_groups_changed,
                bases: bases_changed,
                land_units: land_units_changed,
                contacts: PerSide {
                    allied: allied_contacts,
                    japanese: japanese_contacts,
                },
                ground_estimates: PerSide {
                    allied: allied_estimates,
                    japanese: japanese_estimates,
                },
            },
            removed: Lists {
                task_forces: task_forces_removed,
                ships: ships_removed,
                air_groups: air_groups_removed,
                bases: bases_removed,
                land_units: land_units_removed,
                contacts: PerSide {
                    allied: allied_contacts_gone,
                    japanese: japanese_contacts_gone,
                },
                ground_estimates: PerSide {
                    allied: allied_estimates_gone,
                    japanese: japanese_estimates_gone,
                },
            },
        });
    }
}
