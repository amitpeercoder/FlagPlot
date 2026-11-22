//! Handles, places and numbers, and the lookups that name them.

use std::collections::BTreeMap;

use serde::{Serialize, Serializer};
use sim_core::events::{EventKind, RecognisedWords};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{
    AirGroupId, AircraftTypeId, BaseId, EntityRef, EventId, LandUnitId, OrderId, ShipId,
    TaskForceId,
};
use sim_core::side_events::{SideEvent, SideEventKind};
use sim_core::state::{AirGroup, AirGroupLocation, Base, LandUnit, Ship, ShipLocation, TaskForce};
use sim_core::view::{PublicBase, SideView};

/// A [`Fixed`] written as a decimal string, exactly: `"12"`, `"0.25"`. No
/// float is involved (decision 0048, 12).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Dec(pub Fixed);

impl std::fmt::Display for Dec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Serialize for Dec {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&self.0)
    }
}

/// A unit named by kind and number, as the log names it, with its name when
/// the side knows one. Only a side's own units have a name here.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct Handle {
    /// `ship:7`, `task_force:5`, `air_group:3`, `base:2`, `contact:4`.
    pub handle: String,
    pub name: Option<String>,
}

impl Handle {
    pub(crate) fn text(&self) -> String {
        match &self.name {
            Some(name) => format!("{} \"{}\"", self.handle, name),
            None => self.handle.clone(),
        }
    }
}

/// A place: a hex, its latitude and longitude where the map has them, and
/// the name of the side's own base there, if it has one.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct Place {
    pub q: i32,
    pub r: i32,
    pub lat: Option<Dec>,
    pub lon: Option<Dec>,
    pub name: Option<String>,
}

impl Place {
    /// The hex alone, `(q,r)`.
    pub(crate) fn coords(&self) -> String {
        format!("({},{})", self.q, self.r)
    }

    pub(crate) fn text(&self) -> String {
        match &self.name {
            Some(name) => format!("({},{}) {}", self.q, self.r, name),
            None => format!("({},{})", self.q, self.r),
        }
    }
}

/// The side's own units by number, for naming and for position. Built once
/// from the view.
pub(crate) struct Ctx<'a> {
    pub view: SideView<'a>,
    pub ships: BTreeMap<ShipId, &'a Ship>,
    pub forces: BTreeMap<TaskForceId, &'a TaskForce>,
    pub groups: BTreeMap<AirGroupId, &'a AirGroup>,
    pub bases: BTreeMap<BaseId, &'a Base>,
    /// The side's own land units (decision 0066), destroyed ones too.
    pub land_units: BTreeMap<LandUnitId, &'a LandUnit>,
    /// The public facts of the bases the side does not own.
    pub enemy_bases: BTreeMap<BaseId, PublicBase>,
    /// The side's own units held apart until their day (decision 0195, 13):
    /// not on the map, so not in the maps above, but each has its number
    /// and its name, and the side may name it in an order or a command.
    pub held_ships: BTreeMap<ShipId, &'a Ship>,
    pub held_forces: BTreeMap<TaskForceId, &'a TaskForce>,
    pub held_groups: BTreeMap<AirGroupId, &'a AirGroup>,
    pub held_land_units: BTreeMap<LandUnitId, &'a LandUnit>,
    place_names: BTreeMap<Hex, String>,
    /// The side's own number for each event it sees, by the log's number.
    numbers: BTreeMap<EventId, u32>,
    /// The side's own number for each of its orders, by the log's number.
    order_numbers: BTreeMap<OrderId, u32>,
    /// What each of the side's own orders ordered, in words, for the event
    /// that says it was taken in.
    order_words: BTreeMap<OrderId, String>,
    /// How each base the side's events say was formed came to be, from
    /// `set_events` (decision 0110, item 8).
    formed: BTreeMap<BaseId, crate::briefing::BaseFormedBy>,
    /// The side's own ships, task forces and air groups in the umpire's id
    /// order: a unit's number is its place in the list, counting from 1
    /// (decision 0086). Every unit the view gives is in it, sunk ships and
    /// lost groups too, so a number never changes during a game.
    ship_ids: Vec<ShipId>,
    force_ids: Vec<TaskForceId>,
    group_ids: Vec<AirGroupId>,
    land_unit_ids: Vec<LandUnitId>,
}

/// The unit kinds whose numbers are the side's own.
const OWN_KINDS: [&str; 4] = ["ship", "task_force", "air_group", "land_unit"];

impl<'a> Ctx<'a> {
    pub fn new(view: SideView<'a>) -> Ctx<'a> {
        let ships: BTreeMap<_, _> = view.ships().map(|s| (s.id, s)).collect();
        let forces: BTreeMap<_, _> = view.task_forces().map(|f| (f.id, f)).collect();
        let groups: BTreeMap<_, _> = view.air_groups().map(|g| (g.id, g)).collect();
        let bases: BTreeMap<_, _> = view.bases().map(|b| (b.id, b)).collect();
        let land_units: BTreeMap<_, _> = view.land_units().map(|u| (u.id, u)).collect();
        let enemy_bases: BTreeMap<_, _> = view.enemy_bases().map(|b| (b.id, b)).collect();
        let mut place_names: BTreeMap<Hex, String> = BTreeMap::new();
        for base in bases.values() {
            let entry = place_names.entry(base.hex).or_default();
            if !entry.is_empty() {
                entry.push_str(" / ");
            }
            entry.push_str(&base.name);
        }
        let order_words = view
            .orders()
            .map(|o| (o.id, crate::orders::order_words(&o.kind)))
            .collect();
        let order_numbers = view
            .orders()
            .enumerate()
            .map(|(i, o)| (o.id, u32::try_from(i + 1).expect("few orders")))
            .collect();
        let mut held_ships: BTreeMap<ShipId, &Ship> = BTreeMap::new();
        let mut held_forces: BTreeMap<TaskForceId, &TaskForce> = BTreeMap::new();
        let mut held_groups: BTreeMap<AirGroupId, &AirGroup> = BTreeMap::new();
        let mut held_land_units: BTreeMap<LandUnitId, &LandUnit> = BTreeMap::new();
        for entry in view.entering() {
            held_ships.extend(entry.ships.iter().map(|(id, s)| (*id, s)));
            held_forces.extend(entry.task_force.iter().map(|f| (f.id, f)));
            held_groups.extend(entry.air_groups.iter().map(|(id, g)| (*id, g)));
            held_land_units.extend(entry.land_units.iter().map(|(id, u)| (*id, u)));
        }
        // A held unit has its number from the start, in the umpire's id
        // order with the rest, so that no number changes the day it enters
        // (decision 0195, 13).
        let ship_ids = sorted_ids(ships.keys().chain(held_ships.keys()));
        let force_ids = sorted_ids(forces.keys().chain(held_forces.keys()));
        let group_ids = sorted_ids(groups.keys().chain(held_groups.keys()));
        let land_unit_ids = sorted_ids(land_units.keys().chain(held_land_units.keys()));
        Ctx {
            held_ships,
            held_forces,
            held_groups,
            held_land_units,
            view,
            order_numbers,
            order_words,
            ship_ids,
            force_ids,
            group_ids,
            land_unit_ids,
            land_units,
            ships,
            forces,
            groups,
            bases,
            enemy_bases,
            place_names,
            numbers: BTreeMap::new(),
            formed: BTreeMap::new(),
        }
    }

    /// Numbers the side's events 1, 2, 3... in the order the view gives them.
    /// These are the numbers a player sees; the log's own never leave the
    /// crate (decision 0048, as amended: a gap in the log's numbers would
    /// tell a side that something happened it cannot see).
    pub fn set_events(&mut self, events: &[SideEvent]) {
        self.numbers = events
            .iter()
            .enumerate()
            .map(|(i, e)| (e.id, u32::try_from(i + 1).expect("few events")))
            .collect();
        let mut formed = BTreeMap::new();
        for (i, e) in events.iter().enumerate() {
            let number = u32::try_from(i + 1).expect("few events");
            if let SideEventKind::Whole(kind) = &e.kind {
                match kind {
                    EventKind::BaseFormed {
                        base, task_force, ..
                    } => {
                        formed.insert(
                            *base,
                            crate::briefing::BaseFormedBy {
                                how: "landing",
                                by: self.force(*task_force),
                                day: e.day,
                                event: number,
                            },
                        );
                    }
                    EventKind::MarchBaseFormed { base, unit, .. } => {
                        formed.insert(
                            *base,
                            crate::briefing::BaseFormedBy {
                                how: "march",
                                by: self.land_unit(*unit),
                                day: e.day,
                                event: number,
                            },
                        );
                    }
                    _ => {}
                }
            }
        }
        self.formed = formed;
    }

    /// How a base the side holds was formed, when its events say a landing
    /// or a march did it. Needs `set_events`.
    pub fn formed_by(&self, base: BaseId) -> Option<crate::briefing::BaseFormedBy> {
        self.formed.get(&base).cloned()
    }

    /// Whether the side holds another base in the hex of `base`.
    pub fn shares_hex(&self, base: &Base) -> bool {
        self.bases
            .values()
            .filter(|other| other.hex == base.hex)
            .count()
            > 1
    }

    /// The side's number of an event it sees.
    pub fn number(&self, id: EventId) -> u32 {
        *self
            .numbers
            .get(&id)
            .expect("an event the side sees has a number")
    }

    /// The side's number of an event, or `None` for one the side is not
    /// shown (the report an estimate rests on, at a seat that does not see
    /// the side's reports).
    pub fn number_if_seen(&self, id: EventId) -> Option<u32> {
        self.numbers.get(&id).copied()
    }

    /// The side's number of one of its own orders: 1, 2, 3... in the order
    /// they were taken in. The log numbers both sides' orders in one run, so
    /// its numbers would tell a side how many orders the enemy gave
    /// (decision 0055). They never leave the crate.
    pub fn order_number(&self, id: OrderId) -> u32 {
        *self
            .order_numbers
            .get(&id)
            .expect("an order the side is shown is its own")
    }

    /// The side's number of one of its own orders, or `None` for an order
    /// the side is not shown.
    pub fn order_number_if_own(&self, id: OrderId) -> Option<u32> {
        self.order_numbers.get(&id).copied()
    }

    /// The line the log writes for an event, with an order named by the
    /// side's own number.
    pub fn event_text(&self, kind: &SideEventKind) -> String {
        let text = match kind {
            SideEventKind::Whole(EventKind::OrderAccepted { order, target }) => {
                // What was ordered, said to the side that gave it: without
                // it a day's events do not tell what an order changed
                // (decision 0156, the mend after its proof).
                match self.order_words.get(order) {
                    Some(words) => format!(
                        "order:{} accepted for {target}: {words}",
                        self.order_number(*order)
                    ),
                    None => format!("order:{} accepted for {target}", self.order_number(*order)),
                }
            }
            SideEventKind::Whole(EventKind::OrderRejected {
                order,
                target,
                reason,
            }) => format!(
                "order:{} rejected for {target}: {}",
                self.order_number(*order),
                crate::words::refusal(reason)
            ),
            // The type the side's people recognised is said by the name its
            // data has (decision 0167, 6), not by its number.
            SideEventKind::AirAttackAgainst {
                recognised: Some(aircraft_type),
                ..
            }
            | SideEventKind::BaseAttackedAgainst {
                recognised: Some(aircraft_type),
                ..
            } => kind.to_string().replace(
                &RecognisedWords(Some(*aircraft_type)).to_string(),
                &format!("recognised as {}", self.aircraft_type_name(*aircraft_type)),
            ),
            // A claim names types by the name its data has (decisions 0179,
            // 8, and 0210, 8) and units as the seat's tools name them, one no longer in
            // the game "since dissolved" (decision 0180, 10). Each unit is
            // written by the umpire's token, which the mapping below turns
            // into the side's number.
            SideEventKind::Whole(EventKind::ClaimReceived {
                side,
                hex,
                claimed_sunk,
                claimed_hits,
                claimed_kind,
                claimed_shot_down,
                claimed_on_ground,
                from_air,
                from_bombardment,
                ground_types,
                by_unit,
                air_types,
            }) if *from_air => {
                let types: Vec<(String, u32)> = ground_types
                    .iter()
                    .map(|(t, n)| (self.aircraft_type_name(*t), *n))
                    .collect();
                let in_the_air: Vec<(String, u32)> = air_types
                    .iter()
                    .map(|(t, n)| (self.aircraft_type_name(*t), *n))
                    .collect();
                let units: Vec<(String, sim_core::claims::UnitClaim)> = by_unit
                    .iter()
                    .map(|line| {
                        let name = match line.unit {
                            EntityRef::AirGroup(id) => self.groups.get(&id).map(|g| g.name.clone()),
                            EntityRef::TaskForce(id) => {
                                self.forces.get(&id).map(|f| f.name.clone())
                            }
                            other => self.entity(other).name,
                        };
                        let words = match (self.unit_in_game(line.unit), name) {
                            (true, Some(name)) => format!("{} \"{name}\"", line.unit),
                            (true, None) => line.unit.to_string(),
                            (false, _) => format!("{}, since dissolved", line.unit),
                        };
                        (words, *line)
                    })
                    .collect();
                sim_core::events::claim_words(
                    *side,
                    *hex,
                    *claimed_sunk,
                    *claimed_hits,
                    *claimed_kind,
                    *claimed_shot_down,
                    *claimed_on_ground,
                    *from_air,
                    *from_bombardment,
                    &types,
                    &units,
                    &in_the_air,
                )
            }
            other => other.to_string(),
        };
        self.map_text(&text)
    }

    /// The name the data gives an aircraft type: reference data, so common
    /// knowledge. Its number where the data has no such type.
    pub fn aircraft_type_name(&self, id: AircraftTypeId) -> String {
        self.view
            .aircraft_type(id)
            .map_or_else(|| id.to_string(), |t| t.name.clone())
    }

    /// The side's number of one of its own ships (decision 0086).
    pub fn ship_number(&self, id: ShipId) -> Option<u32> {
        self.ship_ids.binary_search(&id).ok().map(position_number)
    }

    pub fn force_number(&self, id: TaskForceId) -> Option<u32> {
        self.force_ids.binary_search(&id).ok().map(position_number)
    }

    /// How many task forces the side has had, the ceased ones counted: its
    /// last force's number.
    pub fn force_count(&self) -> u32 {
        u32::try_from(self.force_ids.len()).expect("few forces")
    }

    /// The number the side's next task force made in play will have: one
    /// past its last of the series that starts above `floor`, or the first
    /// of the series (decision 0086).
    pub fn next_made_force(&self, floor: u32) -> u32 {
        self.force_ids
            .iter()
            .map(|id| id.raw())
            .filter(|raw| *raw > floor && *raw < floor + sim_core::world::MADE_IDS_FLOOR)
            .max()
            .map_or(floor + 1, |last| last + 1)
    }

    pub fn group_number(&self, id: AirGroupId) -> Option<u32> {
        self.group_ids.binary_search(&id).ok().map(position_number)
    }

    /// The side's number of one of its own land units, destroyed ones
    /// counted (decision 0086, 3; decision 0066, 1).
    pub fn land_unit_number(&self, id: LandUnitId) -> Option<u32> {
        self.land_unit_ids
            .binary_search(&id)
            .ok()
            .map(position_number)
    }

    /// The own land unit a side number names; `None` for a number that
    /// names none.
    pub fn own_land_unit(&self, number: u32) -> Option<&'a LandUnit> {
        let id = *self.land_unit_ids.get(number_position(number)?)?;
        self.land_units.get(&id).copied()
    }

    /// The own land unit a side number names, one held apart included: the
    /// side may name it in an order or a command and the engine says it is
    /// not on the map yet (decision 0195, 7 and 8).
    pub fn own_land_unit_or_held(&self, number: u32) -> Option<&'a LandUnit> {
        let id = *self.land_unit_ids.get(number_position(number)?)?;
        self.land_units
            .get(&id)
            .or_else(|| self.held_land_units.get(&id))
            .copied()
    }

    /// As [`Ctx::own_land_unit_or_held`], for a ship.
    pub fn own_ship_or_held(&self, number: u32) -> Option<&'a Ship> {
        let id = *self.ship_ids.get(number_position(number)?)?;
        self.ships
            .get(&id)
            .or_else(|| self.held_ships.get(&id))
            .copied()
    }

    /// As [`Ctx::own_land_unit_or_held`], for a task force.
    pub fn own_force_or_held(&self, number: u32) -> Option<&'a TaskForce> {
        let id = *self.force_ids.get(number_position(number)?)?;
        self.forces
            .get(&id)
            .or_else(|| self.held_forces.get(&id))
            .copied()
    }

    /// As [`Ctx::own_land_unit_or_held`], for an air group.
    pub fn own_group_or_held(&self, number: u32) -> Option<&'a AirGroup> {
        let id = *self.group_ids.get(number_position(number)?)?;
        self.groups
            .get(&id)
            .or_else(|| self.held_groups.get(&id))
            .copied()
    }

    /// The own ship a side number names; `None` for a number that names none.
    pub fn own_ship(&self, number: u32) -> Option<&'a Ship> {
        let id = *self.ship_ids.get(number_position(number)?)?;
        self.ships.get(&id).copied()
    }

    pub fn own_force(&self, number: u32) -> Option<&'a TaskForce> {
        let id = *self.force_ids.get(number_position(number)?)?;
        self.forces.get(&id).copied()
    }

    pub fn own_group(&self, number: u32) -> Option<&'a AirGroup> {
        let id = *self.group_ids.get(number_position(number)?)?;
        self.groups.get(&id).copied()
    }

    /// Text with each token `ship:N`, `task_force:N` or `air_group:N` that
    /// names an own unit by the umpire's id rewritten to the side's number
    /// (decision 0086). One pass: a rewritten number is not read again. A
    /// token that names no own unit is left as it is.
    pub fn map_text(&self, text: &str) -> String {
        map_unit_tokens(text, |kind, raw| match kind {
            "ship" => self.ship_number(ShipId::new(raw)),
            "task_force" => self.force_number(TaskForceId::new(raw)),
            "land_unit" => self.land_unit_number(LandUnitId::new(raw)),
            _ => self.group_number(AirGroupId::new(raw)),
        })
    }

    pub fn place(&self, hex: Hex) -> Place {
        let data = self.view.hex(hex);
        Place {
            q: hex.q,
            r: hex.r,
            lat: data.and_then(|d| d.lat).map(Dec),
            lon: data.and_then(|d| d.lon).map(Dec),
            name: self.place_names.get(&hex).cloned(),
        }
    }

    pub fn ship(&self, id: ShipId) -> Handle {
        Handle {
            handle: self
                .ship_number(id)
                .map_or_else(|| id.to_string(), |n| format!("ship:{n}")),
            name: self
                .ships
                .get(&id)
                .or_else(|| self.held_ships.get(&id))
                .map(|s| s.name.clone()),
        }
    }

    pub fn force(&self, id: TaskForceId) -> Handle {
        Handle {
            handle: self
                .force_number(id)
                .map_or_else(|| id.to_string(), |n| format!("task_force:{n}")),
            name: self
                .forces
                .get(&id)
                .or_else(|| self.held_forces.get(&id))
                .map(|f| f.name.clone()),
        }
    }

    pub fn group(&self, id: AirGroupId) -> Handle {
        Handle {
            handle: self
                .group_number(id)
                .map_or_else(|| id.to_string(), |n| format!("air_group:{n}")),
            name: self
                .groups
                .get(&id)
                .or_else(|| self.held_groups.get(&id))
                .map(|g| g.name.clone()),
        }
    }

    pub fn land_unit(&self, id: LandUnitId) -> Handle {
        Handle {
            handle: self
                .land_unit_number(id)
                .map_or_else(|| id.to_string(), |n| format!("land_unit:{n}")),
            name: self
                .land_units
                .get(&id)
                .or_else(|| self.held_land_units.get(&id))
                .map(|u| u.name.clone()),
        }
    }

    pub fn base(&self, id: BaseId) -> Handle {
        Handle {
            handle: id.to_string(),
            name: self
                .bases
                .get(&id)
                .map(|b| b.name.clone())
                .or_else(|| self.enemy_bases.get(&id).map(|b| b.name.clone())),
        }
    }

    /// Whether a unit a claim names is still in the game (decision 0180,
    /// 10): an own air group or task force the view still gives, or a base
    /// of either side.
    pub fn unit_in_game(&self, unit: EntityRef) -> bool {
        match unit {
            EntityRef::AirGroup(id) => self.groups.contains_key(&id),
            EntityRef::TaskForce(id) => self.forces.contains_key(&id),
            EntityRef::Base(id) => {
                self.bases.contains_key(&id) || self.enemy_bases.contains_key(&id)
            }
            _ => false,
        }
    }

    pub fn entity(&self, entity: EntityRef) -> Handle {
        match entity {
            EntityRef::Ship(id) => self.ship(id),
            EntityRef::TaskForce(id) => self.force(id),
            EntityRef::AirGroup(id) => self.group(id),
            EntityRef::Base(id) => self.base(id),
            EntityRef::Pilot(id) => bare(id.to_string()),
            EntityRef::LandUnit(id) => self.land_unit(id),
            EntityRef::Contact(id) => bare(format!("contact:{}", self.view.contact_number(id))),
            // Only a draw's key (decision 0135, 11); no event names it.
            EntityRef::Event(id) => match self.numbers.get(&id) {
                Some(number) => bare(format!("event:{number}")),
                None => bare("event".to_string()),
            },
            // Only a trial's draw's key (decision 0174, 3); no event names
            // it.
            EntityRef::Place(place) => bare(format!("place:{place}")),
        }
    }

    /// Where an own ship is: its task force's hex, or its port. `None` for a
    /// ship that is sunk, and for one away at the yard (decision 0069), which
    /// is afloat but off the map.
    pub fn ship_hex(&self, ship: &Ship) -> Option<Hex> {
        match ship.location {
            ShipLocation::TaskForce(id) => self.forces.get(&id).map(|f| f.position()),
            ShipLocation::Port(base) => self.bases.get(&base).map(|b| b.hex),
            ShipLocation::Sunk | ShipLocation::Yard(_) => None,
        }
    }

    /// Where an own air group is.
    pub fn group_hex(&self, group: &AirGroup) -> Option<Hex> {
        match group.location {
            // An empty group may wait at a base that is not its side's
            // (decision 0101): the hex of an enemy base is public.
            AirGroupLocation::Base(base) => self
                .bases
                .get(&base)
                .map(|b| b.hex)
                .or_else(|| self.enemy_bases.get(&base).map(|b| b.hex)),
            AirGroupLocation::Ship(ship) => self.ships.get(&ship).and_then(|s| self.ship_hex(s)),
            // Lost with a base that changed side: no place (decision 0090, 20).
            AirGroupLocation::Lost => None,
        }
    }

    /// The own ships of a task force that are afloat, in number order.
    pub fn afloat_in<'s>(&'s self, force: &'s TaskForce) -> impl Iterator<Item = &'a Ship> + 's {
        force
            .ships
            .iter()
            .filter_map(|id| self.ships.get(id).copied())
            .filter(|s| s.location != ShipLocation::Sunk)
    }
}

/// The ids in one list, in the umpire's order.
fn sorted_ids<'i, T: Copy + Ord + 'i>(ids: impl Iterator<Item = &'i T>) -> Vec<T> {
    let mut ids: Vec<T> = ids.copied().collect();
    ids.sort_unstable();
    ids
}

/// A unit's number from its place in the list: places count from 0, numbers
/// from 1.
fn position_number(position: usize) -> u32 {
    u32::try_from(position + 1).expect("few units")
}

/// The place in the list of a unit's number; `None` for 0.
fn number_position(number: u32) -> Option<usize> {
    usize::try_from(number.checked_sub(1)?).ok()
}

/// Rewrites each `ship:N`, `task_force:N` or `air_group:N` of `text` for
/// which `number_of(kind, N)` gives a number to `kind:number`, in one pass.
/// A token must start at a word boundary and end at the end of its digits.
pub(crate) fn map_unit_tokens(text: &str, number_of: impl Fn(&str, u32) -> Option<u32>) -> String {
    let bytes = text.as_bytes();
    let mut out = String::with_capacity(text.len());
    let mut copied = 0;
    let mut i = 0;
    while i < bytes.len() {
        let at_boundary = i == 0 || !(bytes[i - 1].is_ascii_alphanumeric() || bytes[i - 1] == b'_');
        if at_boundary {
            if let Some(kind) = OWN_KINDS.iter().find(|k| {
                text.get(i..).is_some_and(|t| t.starts_with(**k))
                    && bytes.get(i + k.len()) == Some(&b':')
            }) {
                let digits_at = i + kind.len() + 1;
                let digits = text[digits_at..]
                    .bytes()
                    .take_while(u8::is_ascii_digit)
                    .count();
                if digits > 0 {
                    if let Some(number) = text[digits_at..digits_at + digits]
                        .parse::<u32>()
                        .ok()
                        .and_then(|raw| number_of(kind, raw))
                    {
                        out.push_str(&text[copied..digits_at]);
                        out.push_str(&number.to_string());
                        copied = digits_at + digits;
                    }
                    i = digits_at + digits;
                    continue;
                }
            }
        }
        i += 1;
    }
    out.push_str(&text[copied..]);
    out
}

fn bare(handle: String) -> Handle {
    Handle { handle, name: None }
}

/// The name of an event's kind in snake_case, such as `ship_sunk`, from the
/// name of its variant.
pub(crate) fn kind_name(kind: &SideEventKind) -> String {
    let debug = match kind {
        SideEventKind::Whole(inner) => format!("{inner:?}"),
        cut => format!("{cut:?}"),
    };
    let name: String = debug
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric())
        .collect();
    let mut out = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_ascii_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

/// The day an own unit may be used from, while that day has not come for the
/// orders now being given (decision 0140, 5): the effective day from
/// [`SideView::available_from`], given only while [`SideView::may_use`] says
/// no, so it is the day a refusal would name. `None` once the day has come,
/// for a unit with no day and for any unit that is not the side's.
pub(crate) fn day_not_come(view: &SideView<'_>, unit: EntityRef) -> Option<u32> {
    view.available_from(unit).filter(|_| !view.may_use(unit))
}

/// Whole miles of steaming a ship's fuel gives at its class's cruising
/// speed: fuel x endurance / full load, rounded down. `None` if the class is
/// unknown or has no full load.
pub(crate) fn cruise_range_nm(view: &SideView<'_>, ship: &Ship) -> Option<u32> {
    let class = view.ship_class(ship.class)?;
    if class.fuel_capacity_t.is_zero() {
        return None;
    }
    let range = ship
        .fuel_t
        .mul_div(class.endurance_nm, class.fuel_capacity_t);
    u32::try_from(range.trunc().max(0)).ok()
}

/// The picture an agent in a seat is shown: the area command's (decision
/// 0134, 13) in the area seat, the seat's own command's at a task force or a
/// base, and the side's own belief under the easier setting (decision 0138,
/// 5). With command levels off it is the side's own belief.
pub(crate) fn area_picture<'v>(
    view: &sim_core::view::SideView<'v>,
) -> sim_core::view::CommandView<'v> {
    view.seat_picture()
}

/// The picture of the command an air group of the side belongs to (decision
/// 0134, 8 and 13): the one its strike decision reads. The area command's
/// for a group the side does not have.
pub(crate) fn group_picture<'v>(
    view: &sim_core::view::SideView<'v>,
    group: sim_core::ids::AirGroupId,
) -> sim_core::view::CommandView<'v> {
    if view.has_one_picture() {
        return area_picture(view);
    }
    view.command_of_group(group)
        .and_then(|command| view.command(command))
        .unwrap_or_else(|| area_picture(view))
}

#[cfg(test)]
mod tests {
    use super::map_unit_tokens;

    fn swap(kind: &str, raw: u32) -> Option<u32> {
        // Own units are the ones numbered 10 to 19, and become 1 to 10.
        (kind != "air_group" && (10..20).contains(&raw)).then(|| raw - 9)
    }

    #[test]
    fn a_token_of_an_own_unit_is_rewritten_once_and_nothing_else_is() {
        assert_eq!(
            map_unit_tokens(
                "ship:12 \"X\" in task_force:10, with ship:2 and air_group:12",
                swap
            ),
            "ship:3 \"X\" in task_force:1, with ship:2 and air_group:12"
        );
        // A rewritten number is not read again, a token inside a word is not
        // one, and digits that go on are one number.
        assert_eq!(map_unit_tokens("ship:10 ship:11", swap), "ship:1 ship:2");
        assert_eq!(
            map_unit_tokens("airship:12 my_ship:12 ship:120", swap),
            "airship:12 my_ship:12 ship:120"
        );
        assert_eq!(map_unit_tokens("ship:", swap), "ship:");
        assert_eq!(map_unit_tokens("é ship:15é", swap), "é ship:6é");
    }
}
