//! What is held apart for the side until its day (decision 0195, 7 and 13):
//! the side's own units that are not on the map yet. Each entry says its day,
//! its place (a hex for a task force, a base for a land unit or an air group),
//! what it is and whether it waits. Only the side's own entries are ever
//! given; the enemy's are not reachable through the view.

use std::collections::BTreeMap;

use serde::Serialize;
use sim_core::entering::{Entering, EntryPlace, EntryWait};
use sim_core::events::EventKind;
use sim_core::ids::EntityRef;

use crate::common::{Ctx, Handle, Place};

/// The word for why an entry waits.
pub(crate) fn wait_word(wait: EntryWait) -> &'static str {
    match wait {
        EntryWait::NotTheSidesBase => "not_the_sides_base",
        EntryWait::TakesNoAircraft => "takes_no_aircraft",
    }
}

/// The ships of a held force of one kind.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct EntryShips {
    pub kind: String,
    pub ships: u32,
}

/// An air group that enters, with its aircraft (ready and damaged).
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct EntryGroup {
    pub air_group: Handle,
    pub aircraft: u32,
}

/// A land unit that enters, with its men (ashore, and aboard the force that
/// brings it).
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct EntryLandUnit {
    pub land_unit: Handle,
    pub men: u32,
}

/// One unit held apart for the side: when it enters, where, what it is.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct EntryLine {
    /// The day the scenario names; while `waiting` it is past.
    pub day: u32,
    /// `task_force`, `land_unit` or `air_group`.
    pub kind: &'static str,
    pub unit: Handle,
    /// Where a task force enters. Left out for a unit that enters at a base.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<Place>,
    /// Where a land unit or an air group enters. Left out for a force.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<Handle>,
    /// The ships of a force by kind. Left out when none.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ships: Vec<EntryShips>,
    /// The air groups that enter (aboard the force, or the one group).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub air_groups: Vec<EntryGroup>,
    /// The land units that enter (aboard the force, or the one unit).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub land_units: Vec<EntryLandUnit>,
    /// True once its day has come and it could not enter.
    pub waiting: bool,
    /// `not_the_sides_base` or `takes_no_aircraft`, while it waits and the
    /// entry says why.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waits_for: Option<&'static str>,
}

/// A force or a unit came onto the map yesterday.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct EnteredLine {
    pub unit: Handle,
    pub place: Place,
    /// The base of the side the unit entered at, or the force's hex holds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<Handle>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ships: Vec<Handle>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub air_groups: Vec<Handle>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub land_units: Vec<Handle>,
    /// Men ashore, for a land unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub men: Option<u32>,
    /// Aircraft, for an air group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aircraft: Option<u32>,
    pub event: u32,
}

/// A unit that could not enter yesterday and waits.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct EntryWaitingLine {
    pub unit: Handle,
    pub base: Handle,
    /// The day it was to enter.
    pub day: u32,
    pub waits_for: &'static str,
    pub event: u32,
}

/// What yesterday's events of the side say of its entries.
#[derive(Default)]
pub(crate) struct EntriesDay {
    pub entered: Vec<EnteredLine>,
    pub waiting: Vec<EntryWaitingLine>,
}

impl EntriesDay {
    /// Reads one event; true if it was an entry event.
    pub fn read(&mut self, ctx: &Ctx<'_>, kind: &EventKind, event: u32) -> bool {
        match kind {
            EventKind::ForceEntered {
                force,
                hex,
                base,
                ships,
                air_groups,
                land_units,
                ..
            } => self.entered.push(EnteredLine {
                unit: ctx.force(*force),
                place: ctx.place(*hex),
                base: base.map(|b| ctx.base(b)),
                ships: ships.iter().map(|s| ctx.ship(*s)).collect(),
                air_groups: air_groups.iter().map(|g| ctx.group(*g)).collect(),
                land_units: land_units.iter().map(|u| ctx.land_unit(*u)).collect(),
                men: None,
                aircraft: None,
                event,
            }),
            EventKind::UnitEntered {
                unit,
                base,
                hex,
                men,
                aircraft,
                ..
            } => self.entered.push(EnteredLine {
                unit: ctx.entity(*unit),
                place: ctx.place(*hex),
                base: Some(ctx.base(*base)),
                ships: Vec::new(),
                air_groups: Vec::new(),
                land_units: Vec::new(),
                men: matches!(unit, EntityRef::LandUnit(_)).then_some(*men),
                aircraft: matches!(unit, EntityRef::AirGroup(_)).then_some(*aircraft),
                event,
            }),
            EventKind::EntryWaiting {
                unit,
                base,
                day,
                wait,
                ..
            } => self.waiting.push(EntryWaitingLine {
                unit: ctx.entity(*unit),
                base: ctx.base(*base),
                day: *day,
                waits_for: wait_word(*wait),
                event,
            }),
            _ => return false,
        }
        true
    }
}

/// The line for one entry.
pub(crate) fn entry_line(ctx: &Ctx<'_>, entry: &Entering) -> EntryLine {
    let unit = entry.unit();
    let (kind, handle) = match unit {
        EntityRef::TaskForce(id) => ("task_force", ctx.force(id)),
        EntityRef::LandUnit(id) => ("land_unit", ctx.land_unit(id)),
        _ => ("air_group", ctx.entity(unit)),
    };
    let mut by_kind: BTreeMap<&'static str, u32> = BTreeMap::new();
    for ship in entry.ships.values() {
        if let Some(class) = ctx.view.ship_class(ship.class) {
            *by_kind.entry(class.kind.name()).or_default() += 1;
        }
    }
    let men_of = |id| {
        let ashore = entry.land_units.get(&id).map_or(0, |u| u.men_ashore);
        let aboard: u32 = entry
            .ships
            .values()
            .filter_map(|s| s.troops)
            .filter(|t| t.unit == id)
            .map(|t| t.men)
            .sum();
        ashore + aboard
    };
    let (hex, base) = match entry.place {
        EntryPlace::Hex(hex) => (Some(ctx.place(hex)), None),
        EntryPlace::Base(base) => (None, Some(ctx.base(base))),
    };
    EntryLine {
        day: entry.day,
        kind,
        unit: handle,
        hex,
        base,
        ships: by_kind
            .into_iter()
            .map(|(kind, ships)| EntryShips {
                kind: kind.to_string(),
                ships,
            })
            .collect(),
        air_groups: entry
            .air_groups
            .values()
            .map(|g| EntryGroup {
                air_group: ctx.group(g.id),
                aircraft: g.ready + g.damaged,
            })
            .collect(),
        land_units: entry
            .land_units
            .keys()
            .map(|id| EntryLandUnit {
                land_unit: ctx.land_unit(*id),
                men: men_of(*id),
            })
            .collect(),
        waiting: entry.waiting,
        // The entry keeps no reason; the event of the day says it. For a
        // unit at a base the reason is read from the base as the core reads
        // it.
        waits_for: entry.waiting.then(|| waits_for(ctx, entry)).flatten(),
    }
}

/// Why an entry at a base waits, read as the entry step reads it: the base is
/// not the side's, or it is and takes no aircraft.
fn waits_for(ctx: &Ctx<'_>, entry: &Entering) -> Option<&'static str> {
    let EntryPlace::Base(base) = entry.place else {
        return None;
    };
    Some(if ctx.bases.contains_key(&base) {
        wait_word(EntryWait::TakesNoAircraft)
    } else {
        wait_word(EntryWait::NotTheSidesBase)
    })
}

/// The side's entries, in the order of their keys.
pub(crate) fn entry_lines(ctx: &Ctx<'_>) -> Vec<EntryLine> {
    ctx.view.entering().map(|e| entry_line(ctx, e)).collect()
}

/// What `get_entity` says of a unit held apart: what it is, and the entry it
/// is held in.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct EntryDetail {
    pub entity_kind: &'static str,
    /// `task_force`, `ship`, `air_group` or `land_unit`: what was asked of.
    pub held: &'static str,
    pub unit: Handle,
    /// The kind of ship, for a ship.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    /// The day the entry is to come onto the map and where.
    pub enters: EntryLine,
}

/// The detail of a held unit, if `unit` is held apart for the side.
pub(crate) fn entry_detail(ctx: &Ctx<'_>, unit: EntityRef) -> Option<EntryDetail> {
    let entry = ctx.view.entry_of(unit)?;
    let (held, handle, class) = match unit {
        EntityRef::TaskForce(id) => ("task_force", ctx.force(id), None),
        EntityRef::Ship(id) => {
            let class = entry
                .ships
                .get(&id)
                .and_then(|s| ctx.view.ship_class(s.class));
            ("ship", ctx.ship(id), class.map(|c| c.name.clone()))
        }
        EntityRef::AirGroup(id) => ("air_group", ctx.group(id), None),
        EntityRef::LandUnit(id) => ("land_unit", ctx.land_unit(id), None),
        _ => return None,
    };
    Some(EntryDetail {
        entity_kind: "to_enter",
        held,
        unit: handle,
        class,
        enters: entry_line(ctx, entry),
    })
}
