//! Forces that arrive (decision 0195): a task force with everything aboard
//! it, or a land unit or an air group at a base, held apart from the state
//! the day's rules walk until the day the scenario names, and brought onto
//! the map first thing on that day, before the day's commands are taken in.
//!
//! What is held apart is in [`TruthState::entering`], keyed by the unit that
//! enters. Nothing in it is in a hex: no rule of a day reads it, since every
//! rule walks the state's own ships, forces, groups and units and these are
//! not among them. Only the side's own view gives it (decision 0195, 13).

use std::collections::BTreeMap;

use crate::commander::Commander;
use crate::events::EventKind;
use crate::hex::Hex;
use crate::ids::{AirGroupId, BaseId, EntityRef, LandUnitId, ShipId};
use crate::pictures::CommandRef;
use crate::state::{
    AirGroup, AirGroupLocation, LandUnit, Ship, ShipLocation, Side, TaskForce, TruthState,
};
use crate::world::World;

/// Where an entry comes onto the map (decision 0195, 3, 4, 16, 17): a task
/// force at a hex (sea or coast, or a base of its side whose hex is so); a
/// land unit or an air group at a base.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EntryPlace {
    Hex(Hex),
    Base(BaseId),
}

/// Why an entry at a base waits (decision 0195, 4 and 16), as an arrival of
/// aircraft waits (decision 0101, ruling A; decision 5 of 0103).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EntryWait {
    /// The base is not the unit's side's on the day.
    NotTheSidesBase,
    /// The base is the side's, but takes no aircraft and the group has
    /// aircraft.
    TakesNoAircraft,
}

impl EntryWait {
    /// A stable number, for the hash and the event's words. Never renumber.
    pub const fn code(self) -> u8 {
        match self {
            EntryWait::NotTheSidesBase => 1,
            EntryWait::TakesNoAircraft => 2,
        }
    }
}

/// One unit held apart until its day (decision 0195), whole, as the
/// scenario wrote it.
///
/// For a task force: the force, its ships, the air groups aboard them and
/// the land units whose men are all aboard them. For a land unit or an air
/// group at a base: that one unit, in `land_units` or `air_groups`, and no
/// force or ship.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Entering {
    /// The day the scenario names: from 2 to the game's last day.
    pub day: u32,
    pub side: Side,
    pub place: EntryPlace,
    /// True once its day has come and it could not enter (its base not its
    /// side's, or taking no aircraft): it is tried again each day.
    pub waiting: bool,
    pub task_force: Option<TaskForce>,
    pub ships: BTreeMap<ShipId, Ship>,
    pub air_groups: BTreeMap<AirGroupId, AirGroup>,
    pub land_units: BTreeMap<LandUnitId, LandUnit>,
    /// The command a task force is placed under when it enters, with command
    /// levels on (decision 0195, 19): the scenario's `superior`, `None` for
    /// the area command.
    pub superior: Option<CommandRef>,
    /// The commander the scenario names for a task force, held only with
    /// command levels on, as at the start (decision 0137).
    pub commander: Option<Commander>,
    /// Whether a task force keeps radio silence from the moment it enters
    /// (decision 0136, 15).
    pub radio_silent: bool,
}

impl Entering {
    /// The unit that enters: the task force, or the one land unit or air
    /// group.
    pub fn unit(&self) -> EntityRef {
        if let Some(force) = &self.task_force {
            EntityRef::TaskForce(force.id)
        } else if let Some(id) = self.land_units.keys().next() {
            EntityRef::LandUnit(*id)
        } else {
            EntityRef::AirGroup(
                *self
                    .air_groups
                    .keys()
                    .next()
                    .expect("an entry holds a force, a land unit or an air group"),
            )
        }
    }

    /// Whether `unit` is held in this entry: the force, one of its ships, a
    /// group aboard or a land unit aboard, or the unit at a base.
    pub fn holds(&self, unit: EntityRef) -> bool {
        match unit {
            EntityRef::TaskForce(id) => self.task_force.as_ref().is_some_and(|f| f.id == id),
            EntityRef::Ship(id) => self.ships.contains_key(&id),
            EntityRef::AirGroup(id) => self.air_groups.contains_key(&id),
            EntityRef::LandUnit(id) => self.land_units.contains_key(&id),
            _ => false,
        }
    }

    /// The day the entry is next tried, read on a state whose last day
    /// resolved is `last_day`: its own day, or for one that waits the next
    /// day to come.
    pub fn next_try(&self, last_day: u32) -> u32 {
        if self.waiting {
            self.day.max(last_day + 1)
        } else {
            self.day
        }
    }

    /// The day from which the side may use `unit` once it has entered,
    /// as [`TruthState::available_from`] reads a unit on the map (decision
    /// 0140, 2), read inside this entry alone. `None` where nothing in it has
    /// a day of its own.
    fn own_available_from(&self, unit: EntityRef) -> Option<u32> {
        let force_day = self.task_force.as_ref().and_then(|f| f.available_from);
        match unit {
            EntityRef::Ship(id) => later(self.ships.get(&id)?.available_from, force_day),
            EntityRef::TaskForce(_) => later(
                force_day,
                self.ships.values().filter_map(|s| s.available_from).max(),
            ),
            EntityRef::AirGroup(id) => {
                let group = self.air_groups.get(&id)?;
                let carrier = match group.location {
                    AirGroupLocation::Ship(ship) => self.own_available_from(EntityRef::Ship(ship)),
                    _ => None,
                };
                later(group.available_from, carrier)
            }
            EntityRef::LandUnit(id) => {
                let own = self.land_units.get(&id)?.available_from;
                let ships = self
                    .ships
                    .values()
                    .filter(|s| s.troops.is_some_and(|t| t.unit == id))
                    .filter_map(|s| self.own_available_from(EntityRef::Ship(s.id)))
                    .max();
                later(own, ships)
            }
            _ => None,
        }
    }
}

impl TruthState {
    /// The entry that holds `unit` (decision 0195): the force, a ship of it,
    /// a group or a land unit aboard, or a land unit or group at a base.
    /// `None` for a unit that is on the map or does not exist. Both sides'
    /// entries: for the engine and the umpire. A side reads its own through
    /// its view.
    pub fn entry_of(&self, unit: EntityRef) -> Option<&Entering> {
        if self.entering.is_empty() {
            return None;
        }
        self.entering.values().find(|entry| entry.holds(unit))
    }

    /// The day from which `unit`, held apart, may be used (decision 0195, 8
    /// and 9): the later of the day it is next tried ([`Entering::next_try`])
    /// and its days once on the map (decision 0140). `None` for a unit not
    /// held apart.
    pub(crate) fn held_available_from(&self, unit: EntityRef) -> Option<u32> {
        let entry = self.entry_of(unit)?;
        later(
            Some(entry.next_try(self.day)),
            entry.own_available_from(unit),
        )
    }

    /// The number of the next land unit made in play (decision 0195, 18):
    /// one above the highest of the state's and of what is held apart,
    /// together. With nothing held apart, one above the state's highest, as
    /// before.
    pub(crate) fn next_land_unit_id(&self) -> LandUnitId {
        let held = self
            .entering
            .values()
            .filter_map(|entry| entry.land_units.keys().next_back())
            .max()
            .copied();
        let highest = self.land_units.keys().next_back().copied().max(held);
        LandUnitId::new(highest.map_or(1, |id| id.raw() + 1))
    }

    /// Every ship held apart, in the entries' key order (decision 0195, 12):
    /// for the checks of a world's setup, which read what is held as they
    /// read what is on the map.
    pub(crate) fn held_ships(&self) -> impl Iterator<Item = &Ship> {
        self.entering
            .values()
            .flat_map(|entry| entry.ships.values())
    }

    /// Every air group held apart, in the entries' key order.
    pub(crate) fn held_air_groups(&self) -> impl Iterator<Item = &AirGroup> {
        self.entering
            .values()
            .flat_map(|entry| entry.air_groups.values())
    }

    /// Every land unit held apart, in the entries' key order.
    pub(crate) fn held_land_units(&self) -> impl Iterator<Item = &LandUnit> {
        self.entering
            .values()
            .flat_map(|entry| entry.land_units.values())
    }

    /// The words for the first number that is both in the state and held
    /// apart, or held apart in two entries (decision 0195, 20): a ship, a
    /// task force, an air group or a land unit, each in its own series. A
    /// world is not made from such a state. `None` for a sound one.
    pub(crate) fn held_apart_twice(&self) -> Option<String> {
        let mut ships: BTreeMap<ShipId, EntityRef> = BTreeMap::new();
        let mut groups: BTreeMap<AirGroupId, EntityRef> = BTreeMap::new();
        let mut units: BTreeMap<LandUnitId, EntityRef> = BTreeMap::new();
        for (key, entry) in &self.entering {
            if let Some(force) = &entry.task_force {
                if self.task_forces.contains_key(&force.id) {
                    return Some(format!(
                        "{} is both in the state and held apart until day {}",
                        force.id, entry.day
                    ));
                }
            }
            for id in entry.ships.keys() {
                if self.ships.contains_key(id) || ships.insert(*id, *key).is_some() {
                    return Some(format!(
                        "{id} is both in the state and held apart until day {}, or held apart twice",
                        entry.day
                    ));
                }
            }
            for id in entry.air_groups.keys() {
                if self.air_groups.contains_key(id) || groups.insert(*id, *key).is_some() {
                    return Some(format!(
                        "{id} is both in the state and held apart until day {}, or held apart twice",
                        entry.day
                    ));
                }
            }
            for id in entry.land_units.keys() {
                if self.land_units.contains_key(id) || units.insert(*id, *key).is_some() {
                    return Some(format!(
                        "{id} is both in the state and held apart until day {}, or held apart twice",
                        entry.day
                    ));
                }
            }
        }
        None
    }

    /// Holds `unit` apart until `day` (decision 0195), for the loader, before
    /// the world is made: moves it, with everything aboard a task force, out
    /// of the state into [`TruthState::entering`].
    ///
    /// - A task force: the force, its ships, the air groups aboard them and
    ///   the land units whose men are all aboard them; it enters at its hex.
    /// - A land unit with men ashore in a hex with a base, none aboard: at
    ///   that base (its side's, where both sides have one there).
    /// - An air group at a base: at that base.
    ///
    /// Fails, changing nothing, where the unit is not there, is already held
    /// apart, is a land unit with men aboard or in a hex with no base, is an
    /// air group aboard a ship, or is a force carrying a land unit that has
    /// men ashore or aboard another force.
    pub fn hold_apart(&mut self, unit: EntityRef, day: u32) -> Result<(), String> {
        if self.entry_of(unit).is_some() || self.entering.contains_key(&unit) {
            return Err(format!("{unit} is held apart already"));
        }
        let mut entry = match unit {
            EntityRef::TaskForce(id) => {
                let force = self
                    .task_forces
                    .get(&id)
                    .ok_or_else(|| format!("{unit} does not exist"))?;
                let ships: Vec<ShipId> = self
                    .ships
                    .values()
                    .filter(|s| {
                        matches!(s.location, ShipLocation::TaskForce(f) | ShipLocation::Yard(f) if f == id)
                    })
                    .map(|s| s.id)
                    .collect();
                let groups: Vec<AirGroupId> = self
                    .air_groups
                    .values()
                    .filter(
                        |g| matches!(g.location, AirGroupLocation::Ship(s) if ships.contains(&s)),
                    )
                    .map(|g| g.id)
                    .collect();
                let mut units: Vec<LandUnitId> = ships
                    .iter()
                    .filter_map(|s| self.ships[s].troops.map(|t| t.unit))
                    .collect();
                units.sort_unstable();
                units.dedup();
                for land_unit in &units {
                    let found = self
                        .land_units
                        .get(land_unit)
                        .ok_or_else(|| format!("{land_unit} aboard {unit} does not exist"))?;
                    if found.men_ashore > 0 {
                        return Err(format!(
                            "{land_unit} is aboard {unit}, which enters on day {day}, and has men ashore"
                        ));
                    }
                    let elsewhere = self.ships.values().any(|s| {
                        !ships.contains(&s.id) && s.troops.is_some_and(|t| t.unit == *land_unit)
                    });
                    if elsewhere {
                        return Err(format!(
                            "{land_unit} is aboard {unit}, which enters on day {day}, and has men aboard another force"
                        ));
                    }
                }
                let place = EntryPlace::Hex(force.position());
                let side = force.side;
                Entering {
                    day,
                    side,
                    place,
                    waiting: false,
                    task_force: self.task_forces.remove(&id),
                    ships: ships
                        .iter()
                        .map(|s| (*s, self.ships.remove(s).expect("listed above")))
                        .collect(),
                    air_groups: groups
                        .iter()
                        .map(|g| (*g, self.air_groups.remove(g).expect("listed above")))
                        .collect(),
                    land_units: units
                        .iter()
                        .map(|u| (*u, self.land_units.remove(u).expect("checked above")))
                        .collect(),
                    superior: None,
                    commander: None,
                    radio_silent: false,
                }
            }
            EntityRef::LandUnit(id) => {
                let found = self
                    .land_units
                    .get(&id)
                    .ok_or_else(|| format!("{unit} does not exist"))?;
                if self.men_aboard(id) > 0 {
                    return Err(format!(
                        "{unit} has men aboard a ship: a land unit that enters is placed at a base"
                    ));
                }
                let hex = found
                    .hex
                    .filter(|_| found.men_ashore > 0)
                    .ok_or_else(|| format!("{unit} has no men ashore in a hex"))?;
                let side = found.side;
                let base = self
                    .bases
                    .values()
                    .filter(|b| b.hex == hex)
                    .min_by_key(|b| (b.side != side, b.id))
                    .map(|b| b.id)
                    .ok_or_else(|| {
                        format!(
                            "{unit} enters on day {day} in {hex}, where there is no base: a land unit that enters is placed at a base"
                        )
                    })?;
                let mut land_units = BTreeMap::new();
                land_units.insert(id, self.land_units.remove(&id).expect("found above"));
                Entering {
                    day,
                    side,
                    place: EntryPlace::Base(base),
                    waiting: false,
                    task_force: None,
                    ships: BTreeMap::new(),
                    air_groups: BTreeMap::new(),
                    land_units,
                    superior: None,
                    commander: None,
                    radio_silent: false,
                }
            }
            EntityRef::AirGroup(id) => {
                let found = self
                    .air_groups
                    .get(&id)
                    .ok_or_else(|| format!("{unit} does not exist"))?;
                let AirGroupLocation::Base(base) = found.location else {
                    return Err(format!(
                        "{unit} is not at a base: an air group that enters is placed at a base"
                    ));
                };
                let side = found.side;
                let mut air_groups = BTreeMap::new();
                air_groups.insert(id, self.air_groups.remove(&id).expect("found above"));
                Entering {
                    day,
                    side,
                    place: EntryPlace::Base(base),
                    waiting: false,
                    task_force: None,
                    ships: BTreeMap::new(),
                    air_groups,
                    land_units: BTreeMap::new(),
                    superior: None,
                    commander: None,
                    radio_silent: false,
                }
            }
            _ => return Err(format!("{unit} cannot be held apart")),
        };
        entry.waiting = false;
        self.entering.insert(unit, entry);
        Ok(())
    }
}

impl World {
    /// Step 0 of the day (decision 0195, 2): puts on the map what is held
    /// apart until `day` or earlier, before the day's commands are taken in.
    /// Entries are taken in the order of their keys: task forces by number,
    /// then air groups, then land units. With nothing held it does nothing
    /// and records nothing. No random number is drawn.
    ///
    /// - A task force always enters, at its hex, whoever is there, with its
    ///   ships, the groups and the land units aboard, as written. With
    ///   command levels on it becomes a command: under its entry's superior
    ///   where that is still a command of the side, else under the area
    ///   command, with the commander the entry names; and an entry kept
    ///   under radio silence puts it in the set of forces that keep it. `ForceEntered`.
    /// - A land unit enters at its base, in the base's hex with its men as
    ///   written, where the base is its side's; else it waits
    ///   (`EntryWaiting`, on each day it waits). `UnitEntered`.
    /// - An air group enters at its base where the base is its side's and,
    ///   if the group has aircraft, takes aircraft
    ///   ([`crate::landing::takes_aircraft`]); else it waits. `UnitEntered`.
    ///
    /// The three events cite no cause and have no pulse.
    pub(crate) fn bring_in(&mut self, day: u32) {
        if self.truth.entering.is_empty() {
            return;
        }
        let due: Vec<EntityRef> = self
            .truth
            .entering
            .iter()
            .filter(|(_, entry)| entry.day <= day)
            .map(|(key, _)| *key)
            .collect();
        for key in due {
            match self.truth.entering[&key].place {
                EntryPlace::Hex(_) => self.bring_in_force(key, day),
                EntryPlace::Base(base) => self.bring_in_at_base(key, base, day),
            }
        }
    }

    /// A task force and everything aboard it comes onto the map.
    fn bring_in_force(&mut self, key: EntityRef, day: u32) {
        let entry = self.truth.entering.remove(&key).expect("the entry is held");
        let Entering {
            day: _,
            side,
            place: _,
            waiting: _,
            task_force,
            ships,
            air_groups,
            land_units,
            superior,
            commander,
            radio_silent,
        } = entry;
        let force = task_force.expect("a task force's entry holds its force");
        let id = force.id;
        let hex = force.position();
        let base = self
            .truth
            .bases
            .values()
            .find(|base| base.side == side && base.hex == hex)
            .map(|base| base.id);
        let kind = EventKind::ForceEntered {
            force: id,
            side,
            hex,
            base,
            ships: ships.keys().copied().collect(),
            air_groups: air_groups.keys().copied().collect(),
            land_units: land_units.keys().copied().collect(),
        };
        self.truth.task_forces.insert(id, force);
        self.truth.ships.extend(ships);
        self.truth.air_groups.extend(air_groups);
        self.truth.land_units.extend(land_units);
        self.events.record(day, None, kind, Vec::new(), None);
        // With command levels on the force is a command from now, as one
        // formed in play is, under the command the scenario placed it in
        // (decision 0195, 19).
        if self.command_levels() {
            let from = superior
                .filter(|from| self.is_command(side, *from))
                .unwrap_or(CommandRef::Area);
            self.command_formed(id, from);
            if let Some(post) = self
                .chain
                .as_mut()
                .and_then(|chain| chain.get_mut(&side))
                .and_then(|posts| posts.get_mut(&CommandRef::TaskForce(id)))
            {
                post.superior = Some(from);
                post.commander = commander;
            }
        }
        if radio_silent {
            self.radio_silent.insert(id);
        }
    }

    /// A land unit or an air group comes onto the map at its base, or waits.
    fn bring_in_at_base(&mut self, key: EntityRef, base_id: BaseId, day: u32) {
        let (side, entry_day) = {
            let entry = &self.truth.entering[&key];
            (entry.side, entry.day)
        };
        let base = self.truth.bases.get(&base_id).filter(|b| b.side == side);
        let wait = match (key, base) {
            (_, None) => Some(EntryWait::NotTheSidesBase),
            (EntityRef::AirGroup(id), Some(base)) => {
                let group = &self.truth.entering[&key].air_groups[&id];
                let has_aircraft = group.ready + group.damaged > 0;
                (has_aircraft
                    && !crate::landing::takes_aircraft(base, self.ref_data.ground.as_ref()))
                .then_some(EntryWait::TakesNoAircraft)
            }
            (_, Some(_)) => None,
        };
        if let Some(wait) = wait {
            self.truth
                .entering
                .get_mut(&key)
                .expect("the entry is held")
                .waiting = true;
            self.events.record(
                day,
                None,
                EventKind::EntryWaiting {
                    unit: key,
                    side,
                    base: base_id,
                    day: entry_day,
                    wait,
                },
                Vec::new(),
                None,
            );
            return;
        }
        let base_hex = base.expect("checked above").hex;
        let entry = self.truth.entering.remove(&key).expect("the entry is held");
        let (men, aircraft) = match key {
            EntityRef::LandUnit(id) => {
                let mut unit = entry.land_units.into_values().next().expect("a land unit");
                debug_assert_eq!(unit.id, id);
                if unit.men_ashore > 0 {
                    unit.hex = Some(base_hex);
                }
                let men = unit.men_ashore;
                self.truth.land_units.insert(id, unit);
                (men, 0)
            }
            EntityRef::AirGroup(id) => {
                let group = entry.air_groups.into_values().next().expect("an air group");
                let aircraft = group.ready + group.damaged;
                self.truth.air_groups.insert(id, group);
                (0, aircraft)
            }
            _ => unreachable!("only a land unit or an air group is held at a base"),
        };
        self.events.record(
            day,
            None,
            EventKind::UnitEntered {
                unit: key,
                side,
                base: base_id,
                hex: base_hex,
                men,
                aircraft,
            },
            Vec::new(),
            None,
        );
    }
}

/// The later of two days, either of which may be absent.
fn later(a: Option<u32>, b: Option<u32>) -> Option<u32> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a.max(b)),
        (a, None) => a,
        (None, b) => b,
    }
}
