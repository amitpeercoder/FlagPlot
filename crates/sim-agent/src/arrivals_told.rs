//! What came today and what comes tomorrow (decision 0215, as amended on
//! review): two lists in the briefing's arrivals part, kept beside the
//! entries' own. Each is made from the side's own view: the units it names are
//! its own, and nothing of the enemy is ever given.
//!
//! * `came_today`: each task force, land unit and air group of the side that
//!   entered in the day just played, each air group that received aircraft,
//!   and each unit whose day to be used was that day. A unit that did more
//!   than one has one line. Day 1 has none: what is there at the start is not
//!   a reinforcement, so nothing before day 2 is told. Each line says what the
//!   staff did with the unit, as far as it is explicit, and never that no
//!   order uses it.
//! * `comes_tomorrow`: each that enters, receives aircraft or may first be
//!   used on the day the briefing is for.
//!
//! "The staff gave it an order today" is read from the side's own orders
//! ([`SideView::orders`], each with its issuer and the intent it serves) and
//! the day's own `OrderAccepted` events (the order, the unit it was accepted
//! for and the day): an order whose issuer is the staff, accepted for the
//! unit in the day just played.

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;
use sim_core::command::HoldTarget;
use sim_core::events::EventKind;
use sim_core::ids::{EntityRef, OrderId, ShipId};
use sim_core::orders::{AirMission, Issuer};
use sim_core::side_events::{SideEvent, SideEventKind};
use sim_core::state::{AirGroup, AirGroupLocation};

use crate::common::{Ctx, Handle, Place};
use crate::entries::EntryShips;
use crate::words;

/// An intent the staff serves or a limit names, as its number and its words.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct IntentRef {
    pub intent: u32,
    pub words: String,
}

/// What is explicit of what the staff did with a unit: exactly one of the
/// fields is set.
#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize)]
pub struct StaffPart {
    /// The staff gave the unit an order in the day just played, for this
    /// intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordered: Option<IntentRef>,
    /// A forces limit of this intent names the unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_in_limit: Option<IntentRef>,
    /// The commander holds the unit: the staff gives it no order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub held: Option<bool>,
    /// The commander gave the unit an order in the day just played.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commander_ordered: Option<bool>,
    /// None of these.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_order_today: Option<bool>,
}

/// A land unit carried by a force: its handle and its men.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct AboardLandUnit {
    pub handle: Handle,
    pub men: u32,
}

/// One line of either list.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ArrivalToldLine {
    /// `task_force`, `land_unit` or `air_group`.
    pub kind: &'static str,
    pub unit: Handle,
    /// Why it is on the list, one or more of `enter`, `aircraft` (an air
    /// group receives aircraft) and `usable` (its day to be used).
    pub how: Vec<&'static str>,
    /// The hex it is in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place: Option<Place>,
    /// The base, or the ship, it is at, where there is one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<Handle>,
    /// A force's ships by kind.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ships: Vec<EntryShips>,
    /// The air groups a force carries. They have no lines of their own.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub air_groups: Vec<Handle>,
    /// The land units a force carries, with their men. They have no lines of
    /// their own.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub land_units: Vec<AboardLandUnit>,
    /// A land unit's men.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub men: Option<u32>,
    /// An air group's aircraft.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aircraft: Option<u32>,
    /// The aircraft an air group received (came today) or is to receive
    /// (comes tomorrow).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aircraft_received: Option<u32>,
    /// An air group's mission now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mission: Option<&'static str>,
    /// The base an air group on the base strike mission is to strike.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<Handle>,
    /// What the staff did with it. In `came_today` only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staff: Option<StaffPart>,
}

/// The two lists.
#[derive(Default)]
pub(crate) struct Told {
    pub came_today: Vec<ArrivalToldLine>,
    pub comes_tomorrow: Vec<ArrivalToldLine>,
}

fn rank(unit: EntityRef) -> u8 {
    match unit {
        EntityRef::TaskForce(_) => 0,
        EntityRef::LandUnit(_) => 1,
        _ => 2,
    }
}

/// The lines in the order of kind and then of the unit's id.
fn ordered(lines: BTreeMap<EntityRef, ArrivalToldLine>) -> Vec<ArrivalToldLine> {
    let mut all: Vec<(EntityRef, ArrivalToldLine)> = lines.into_iter().collect();
    all.sort_by_key(|(unit, _)| (rank(*unit), *unit));
    all.into_iter().map(|(_, line)| line).collect()
}

fn ships_by_kind<'a>(
    ctx: &Ctx<'_>,
    ships: impl Iterator<Item = &'a sim_core::state::Ship>,
) -> Vec<EntryShips> {
    let mut by_kind: BTreeMap<&'static str, u32> = BTreeMap::new();
    for ship in ships {
        if let Some(class) = ctx.view.ship_class(ship.class) {
            *by_kind.entry(class.kind.name()).or_default() += 1;
        }
    }
    by_kind
        .into_iter()
        .map(|(kind, ships)| EntryShips {
            kind: kind.to_string(),
            ships,
        })
        .collect()
}

fn blank(kind: &'static str, unit: Handle) -> ArrivalToldLine {
    ArrivalToldLine {
        kind,
        unit,
        how: Vec::new(),
        place: None,
        at: None,
        ships: Vec::new(),
        air_groups: Vec::new(),
        land_units: Vec::new(),
        men: None,
        aircraft: None,
        aircraft_received: None,
        mission: None,
        target: None,
        staff: None,
    }
}

/// The hex of the force that holds a ship.
fn ship_place(ctx: &Ctx<'_>, ship: ShipId) -> Option<Place> {
    ctx.forces
        .values()
        .find(|f| f.ships.contains(&ship))
        .map(|f| ctx.place(f.position()))
}

fn group_mission(ctx: &Ctx<'_>, line: &mut ArrivalToldLine, group: &AirGroup) {
    line.mission = Some(words::air_mission(group.mission));
    if let AirMission::BaseStrike(base) = group.mission {
        line.target = Some(ctx.base(base));
    }
}

/// The force that carries a unit on the map: the force of the ship a group is
/// on, or of a ship with men of the land unit aboard.
fn carrier_of(ctx: &Ctx<'_>, unit: EntityRef) -> Option<sim_core::ids::TaskForceId> {
    let ship = match unit {
        EntityRef::AirGroup(id) => match ctx.groups.get(&id)?.location {
            AirGroupLocation::Ship(ship) => ship,
            _ => return None,
        },
        EntityRef::LandUnit(id) => {
            ctx.ships
                .values()
                .find(|s| s.troops.is_some_and(|t| t.unit == id))?
                .id
        }
        _ => return None,
    };
    ctx.forces
        .values()
        .find(|f| f.ships.contains(&ship))
        .map(|f| f.id)
}

/// The line for one of the side's units that is on the map: where it is and
/// what it is. `None` for a unit that is not (a group lost, a unit gone).
fn map_line(ctx: &Ctx<'_>, unit: EntityRef) -> Option<ArrivalToldLine> {
    match unit {
        EntityRef::TaskForce(id) => {
            let force = ctx.forces.get(&id)?;
            let mut line = blank("task_force", ctx.force(id));
            line.place = Some(ctx.place(force.position()));
            line.ships = ships_by_kind(
                ctx,
                force.ships.iter().filter_map(|s| ctx.ships.get(s).copied()),
            );
            line.air_groups = ctx
                .groups
                .keys()
                .filter(|g| carrier_of(ctx, EntityRef::AirGroup(**g)) == Some(id))
                .map(|g| ctx.group(*g))
                .collect();
            line.land_units = ctx
                .land_units
                .values()
                .filter(|u| carrier_of(ctx, EntityRef::LandUnit(u.id)) == Some(id))
                .map(|u| AboardLandUnit {
                    handle: ctx.land_unit(u.id),
                    men: u.men_ashore + ctx.view.men_aboard(u.id).unwrap_or(0),
                })
                .collect();
            Some(line)
        }
        EntityRef::LandUnit(id) => {
            let land = ctx.land_units.get(&id)?;
            let mut line = blank("land_unit", ctx.land_unit(id));
            line.place = land.hex.map(|h| ctx.place(h)).or_else(|| {
                ctx.ships
                    .values()
                    .find(|s| s.troops.is_some_and(|t| t.unit == id))
                    .and_then(|s| ship_place(ctx, s.id))
            });
            line.at = land
                .hex
                .and_then(|h| ctx.bases.values().find(|b| b.hex == h))
                .map(|b| ctx.base(b.id));
            line.men = Some(land.men_ashore + ctx.view.men_aboard(id).unwrap_or(0));
            Some(line)
        }
        EntityRef::AirGroup(id) => {
            let group = ctx.groups.get(&id)?;
            let mut line = blank("air_group", ctx.group(id));
            match group.location {
                AirGroupLocation::Base(base) => {
                    line.place = ctx.bases.get(&base).map(|b| ctx.place(b.hex));
                    line.at = Some(ctx.base(base));
                }
                AirGroupLocation::Ship(ship) => {
                    line.place = ship_place(ctx, ship);
                    line.at = Some(ctx.ship(ship));
                }
                AirGroupLocation::Lost => return None,
            }
            line.aircraft = Some(group.ready + group.damaged);
            group_mission(ctx, &mut line, group);
            Some(line)
        }
        _ => None,
    }
}

/// The line for one unit held apart. A force's line lists the groups and
/// land units that enter aboard it; they have no lines of their own.
fn entry_lines(
    ctx: &Ctx<'_>,
    entry: &sim_core::entering::Entering,
) -> Vec<(EntityRef, ArrivalToldLine)> {
    let line = crate::entries::entry_line(ctx, entry);
    let unit = entry.unit();
    let mut out = Vec::new();
    let mut main = blank(line.kind, line.unit.clone());
    main.place = line.hex.clone();
    main.at = line.base.clone();
    main.ships = line.ships.clone();
    if !matches!(unit, EntityRef::TaskForce(_)) {
        main.men = line.land_units.first().map(|u| u.men);
        main.aircraft = line.air_groups.first().map(|g| g.aircraft);
        if let EntityRef::AirGroup(id) = unit {
            if let Some(group) = entry.air_groups.get(&id) {
                group_mission(ctx, &mut main, group);
            }
        }
    }
    if matches!(unit, EntityRef::TaskForce(_)) {
        main.air_groups = line
            .air_groups
            .iter()
            .map(|g| g.air_group.clone())
            .collect();
        main.land_units = line
            .land_units
            .iter()
            .map(|u| AboardLandUnit {
                handle: u.land_unit.clone(),
                men: u.men,
            })
            .collect();
    }
    main.how.push("enter");
    out.push((unit, main));
    out
}

/// The day a unit held apart may first be used by its own days and its
/// force's, not counting the day it enters.
fn own_day(entry: &sim_core::entering::Entering, unit: EntityRef) -> Option<u32> {
    let force = entry.task_force.as_ref().and_then(|f| f.available_from);
    let own = match unit {
        EntityRef::TaskForce(_) => None,
        EntityRef::LandUnit(id) => entry.land_units.get(&id).and_then(|u| u.available_from),
        EntityRef::AirGroup(id) => entry.air_groups.get(&id).and_then(|g| g.available_from),
        _ => None,
    };
    own.max(force)
}

/// Adds `line` under `unit`: where one is there already, fills what it lacks
/// and joins the reasons, so a unit has one line.
fn put(lines: &mut BTreeMap<EntityRef, ArrivalToldLine>, unit: EntityRef, line: ArrivalToldLine) {
    match lines.get_mut(&unit) {
        None => {
            lines.insert(unit, line);
        }
        Some(have) => {
            for how in line.how {
                if !have.how.contains(&how) {
                    have.how.push(how);
                }
            }
            have.place = have.place.take().or(line.place);
            have.at = have.at.take().or(line.at);
            if have.ships.is_empty() {
                have.ships = line.ships;
            }
            if have.air_groups.is_empty() {
                have.air_groups = line.air_groups;
            }
            if have.land_units.is_empty() {
                have.land_units = line.land_units;
            }
            have.men = have.men.or(line.men);
            have.aircraft = have.aircraft.or(line.aircraft);
            have.aircraft_received = match (have.aircraft_received, line.aircraft_received) {
                (Some(a), Some(b)) => Some(a + b),
                (a, b) => a.or(b),
            };
            have.mission = have.mission.or(line.mission);
            have.target = have.target.take().or(line.target);
        }
    }
}

/// What the side's orders accepted in the day just played say of each unit:
/// the orders accepted for it.
fn accepted_for(events: &[SideEvent]) -> BTreeMap<EntityRef, Vec<OrderId>> {
    let mut found: BTreeMap<EntityRef, Vec<OrderId>> = BTreeMap::new();
    for event in events {
        if let SideEventKind::Whole(EventKind::OrderAccepted { order, target }) = &event.kind {
            found.entry(*target).or_default().push(*order);
        }
    }
    found
}

/// What is explicit of what the staff did with `unit` in the day just
/// played. In the order: the staff ordered it; the commander ordered it; the
/// commander holds it; an intent's forces limit names it; none of these.
fn staff_part(
    ctx: &Ctx<'_>,
    unit: EntityRef,
    accepted: &BTreeMap<EntityRef, Vec<OrderId>>,
) -> StaffPart {
    let state = ctx.view.command_state();
    let intent_ref = |id: sim_core::ids::IntentId| -> Option<IntentRef> {
        let intent = state.intents.get(&id)?;
        Some(IntentRef {
            intent: id.raw(),
            words: crate::staffing::intent_words(&ctx.view, intent.kind),
        })
    };
    let orders: Vec<&sim_core::orders::Order> = accepted
        .get(&unit)
        .map(|ids| {
            ids.iter()
                .filter_map(|id| ctx.view.orders().find(|o| o.id == *id))
                .collect()
        })
        .unwrap_or_default();
    let mut part = StaffPart::default();
    if let Some(order) = orders.iter().find(|o| o.origin.issuer == Issuer::Staff) {
        part.ordered = Some(
            order
                .origin
                .intent
                .and_then(intent_ref)
                .unwrap_or(IntentRef {
                    intent: 0,
                    words: String::new(),
                }),
        );
        return part;
    }
    if orders.iter().any(|o| o.origin.issuer == Issuer::Player) {
        part.commander_ordered = Some(true);
        return part;
    }
    let held = match unit {
        EntityRef::TaskForce(id) => state.holds.contains_key(&HoldTarget::TaskForce(id)),
        EntityRef::LandUnit(id) => state.holds.contains_key(&HoldTarget::LandUnit(id)),
        EntityRef::AirGroup(id) => state.holds.contains_key(&HoldTarget::AirGroup(id)),
        _ => false,
    };
    if held {
        part.held = Some(true);
        return part;
    }
    let named = state.intents_in_force().find(|intent| {
        intent
            .limits
            .forces
            .as_ref()
            .is_some_and(|limit| match unit {
                EntityRef::TaskForce(id) => limit.task_forces.contains(&id),
                EntityRef::LandUnit(id) => limit.land_units.contains(&id),
                EntityRef::AirGroup(id) => limit.air_groups.contains(&id),
                _ => false,
            })
    });
    if let Some(intent) = named {
        part.named_in_limit = intent_ref(intent.id);
        if part.named_in_limit.is_some() {
            return part;
        }
    }
    part.no_order_today = Some(true);
    part
}

/// Every unit of the side on the map whose day to be used is `day`.
fn usable_on(ctx: &Ctx<'_>, day: u32) -> Vec<EntityRef> {
    let from = |unit: EntityRef| ctx.view.available_from(unit) == Some(day);
    let mut found: Vec<EntityRef> = Vec::new();
    found.extend(
        ctx.forces
            .keys()
            .map(|id| EntityRef::TaskForce(*id))
            .filter(|u| from(*u)),
    );
    found.extend(
        ctx.land_units
            .keys()
            .map(|id| EntityRef::LandUnit(*id))
            .filter(|u| from(*u)),
    );
    found.extend(
        ctx.groups
            .keys()
            .map(|id| EntityRef::AirGroup(*id))
            .filter(|u| from(*u)),
    );
    found
}

/// Makes the two lists. `events` are the side's events of the day just
/// played (empty before day 1); `today` is the day the briefing is for, the
/// day after the one just played.
pub(crate) fn told_of_arrivals(ctx: &Ctx<'_>, events: &[SideEvent], today: u32) -> Told {
    let mut told = Told::default();
    let played = today - 1;

    // Came today: nothing before day 2 (what is there at the start is not a
    // reinforcement).
    if played >= 2 {
        let accepted = accepted_for(events);
        let mut lines: BTreeMap<EntityRef, ArrivalToldLine> = BTreeMap::new();
        let mut entered: BTreeSet<EntityRef> = BTreeSet::new();
        let mut received: BTreeMap<EntityRef, u32> = BTreeMap::new();
        for event in events {
            let SideEventKind::Whole(kind) = &event.kind else {
                continue;
            };
            match kind {
                EventKind::ForceEntered { force, .. } => {
                    // What it carries is in its line.
                    entered.insert(EntityRef::TaskForce(*force));
                }
                EventKind::UnitEntered { unit, .. } => {
                    entered.insert(*unit);
                }
                EventKind::AircraftArrived {
                    air_group,
                    aircraft,
                    ..
                } => {
                    *received.entry(EntityRef::AirGroup(*air_group)).or_default() += aircraft;
                }
                _ => {}
            }
        }
        for unit in &entered {
            if let Some(mut line) = map_line(ctx, *unit) {
                line.how.push("enter");
                put(&mut lines, *unit, line);
            }
        }
        for (unit, aircraft) in &received {
            if let Some(mut line) = map_line(ctx, *unit) {
                line.how.push("aircraft");
                line.aircraft_received = Some(*aircraft);
                put(&mut lines, *unit, line);
            }
        }
        for unit in usable_on(ctx, played) {
            // A unit carried by a force that has a line is in that line.
            if carrier_of(ctx, unit).is_some_and(|f| lines.contains_key(&EntityRef::TaskForce(f))) {
                continue;
            }
            if let Some(mut line) = map_line(ctx, unit) {
                line.how.push("usable");
                put(&mut lines, unit, line);
            }
        }
        for (unit, line) in &mut lines {
            line.staff = Some(staff_part(ctx, *unit, &accepted));
        }
        told.came_today = ordered(lines);
    }

    // Comes tomorrow: the day the briefing is for. Nothing on day 1.
    if today >= 2 {
        let mut lines: BTreeMap<EntityRef, ArrivalToldLine> = BTreeMap::new();
        for entry in ctx.view.entering().filter(|e| e.day == today) {
            for (unit, mut line) in entry_lines(ctx, entry) {
                // A unit held apart that may first be used on its day of
                // entry has one line. A held unit's `available_from` is the
                // later of its entry and its own day, so its own day (and
                // its carrier's) is read from the entry.
                if own_day(entry, unit) == Some(today) {
                    line.how.push("usable");
                }
                put(&mut lines, unit, line);
            }
        }
        for arrival in ctx.view.arrivals().filter(|a| a.day == today && !a.waiting) {
            if ctx.held_groups.contains_key(&arrival.air_group) {
                continue;
            }
            let Some(group) = ctx.groups.get(&arrival.air_group) else {
                continue;
            };
            if !crate::lines::arrival_is_deliverable_to(ctx, group)
                || crate::lines::arrival_waits(ctx, group).is_some()
            {
                continue;
            }
            let unit = EntityRef::AirGroup(arrival.air_group);
            if let Some(mut line) = map_line(ctx, unit) {
                line.how.push("aircraft");
                line.aircraft_received = Some(arrival.aircraft);
                put(&mut lines, unit, line);
            }
        }
        for unit in usable_on(ctx, today) {
            // A unit carried by a force that has a line is in that line.
            if carrier_of(ctx, unit).is_some_and(|f| lines.contains_key(&EntityRef::TaskForce(f))) {
                continue;
            }
            if let Some(mut line) = map_line(ctx, unit) {
                line.how.push("usable");
                put(&mut lines, unit, line);
            }
        }
        told.comes_tomorrow = ordered(lines);
    }
    told
}

/// Words are the table's and the text's: kept here so the two lists say the
/// staff part the same way everywhere.
pub(crate) fn staff_words(staff: &StaffPart) -> String {
    if let Some(i) = &staff.ordered {
        return format!("the staff has put it to order {} ({})", i.intent, i.words);
    }
    if let Some(i) = &staff.named_in_limit {
        return format!("named in the forces of order {}", i.intent);
    }
    if staff.held == Some(true) {
        return "you hold it".to_string();
    }
    if staff.commander_ordered == Some(true) {
        return "under your direct order".to_string();
    }
    "the staff has given it no order today".to_string()
}
