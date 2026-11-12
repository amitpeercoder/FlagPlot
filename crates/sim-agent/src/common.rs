//! Handles, places and numbers, and the lookups that name them.

use std::collections::BTreeMap;

use serde::{Serialize, Serializer};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, EntityRef, EventId, ShipId, TaskForceId};
use sim_core::side_events::{SideEvent, SideEventKind};
use sim_core::state::{AirGroup, AirGroupLocation, Base, Ship, ShipLocation, TaskForce};
use sim_core::view::SideView;

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
    place_names: BTreeMap<Hex, String>,
    /// The side's own number for each event it sees, by the log's number.
    numbers: BTreeMap<u32, u32>,
}

impl<'a> Ctx<'a> {
    pub fn new(view: SideView<'a>) -> Ctx<'a> {
        let ships: BTreeMap<_, _> = view.ships().map(|s| (s.id, s)).collect();
        let forces: BTreeMap<_, _> = view.task_forces().map(|f| (f.id, f)).collect();
        let groups: BTreeMap<_, _> = view.air_groups().map(|g| (g.id, g)).collect();
        let bases: BTreeMap<_, _> = view.bases().map(|b| (b.id, b)).collect();
        let mut place_names: BTreeMap<Hex, String> = BTreeMap::new();
        for base in bases.values() {
            let entry = place_names.entry(base.hex).or_default();
            if !entry.is_empty() {
                entry.push_str(" / ");
            }
            entry.push_str(&base.name);
        }
        Ctx {
            view,
            ships,
            forces,
            groups,
            bases,
            place_names,
            numbers: BTreeMap::new(),
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
            .map(|(i, e)| (e.id.raw(), u32::try_from(i + 1).expect("few events")))
            .collect();
    }

    /// The side's number of an event it sees.
    pub fn number(&self, id: EventId) -> u32 {
        *self
            .numbers
            .get(&id.raw())
            .expect("an event the side sees has a number")
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
            handle: id.to_string(),
            name: self.ships.get(&id).map(|s| s.name.clone()),
        }
    }

    pub fn force(&self, id: TaskForceId) -> Handle {
        Handle {
            handle: id.to_string(),
            name: self.forces.get(&id).map(|f| f.name.clone()),
        }
    }

    pub fn group(&self, id: AirGroupId) -> Handle {
        Handle {
            handle: id.to_string(),
            name: self.groups.get(&id).map(|g| g.name.clone()),
        }
    }

    pub fn base(&self, id: BaseId) -> Handle {
        Handle {
            handle: id.to_string(),
            name: self.bases.get(&id).map(|b| b.name.clone()),
        }
    }

    pub fn entity(&self, entity: EntityRef) -> Handle {
        match entity {
            EntityRef::Ship(id) => self.ship(id),
            EntityRef::TaskForce(id) => self.force(id),
            EntityRef::AirGroup(id) => self.group(id),
            EntityRef::Base(id) => self.base(id),
            EntityRef::Pilot(id) => bare(id.to_string()),
            EntityRef::LandUnit(id) => bare(id.to_string()),
            EntityRef::Contact(id) => bare(id.to_string()),
        }
    }

    /// Where an own ship is: its task force's hex, or its port. `None` for a
    /// ship that is sunk.
    pub fn ship_hex(&self, ship: &Ship) -> Option<Hex> {
        match ship.location {
            ShipLocation::TaskForce(id) => self.forces.get(&id).map(|f| f.position()),
            ShipLocation::Port(base) => self.bases.get(&base).map(|b| b.hex),
            ShipLocation::Sunk => None,
        }
    }

    /// Where an own air group is.
    pub fn group_hex(&self, group: &AirGroup) -> Option<Hex> {
        match group.location {
            AirGroupLocation::Base(base) => self.bases.get(&base).map(|b| b.hex),
            AirGroupLocation::Ship(ship) => self.ships.get(&ship).and_then(|s| self.ship_hex(s)),
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
