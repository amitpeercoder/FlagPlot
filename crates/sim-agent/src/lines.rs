//! One line for a unit: shared by the briefing and the lists, so a unit is
//! summarised in one way.

use serde::Serialize;
use std::collections::BTreeMap;

use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::EntityRef;
use sim_core::orders::{AirMission, Mission, Reach};
use sim_core::refdata::ShipKind;
use sim_core::side_events::SideEventKind;
use sim_core::state::{AirGroup, Base, LandUnit, ShipLocation, SystemState, TaskForce};
use sim_core::time::Pulse;

use crate::common::{cruise_range_nm, Ctx, Dec, Handle, Place};
use crate::words;

/// A ship's fuel and how far it gives.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct FuelLine {
    pub ship: Handle,
    pub fuel_t: Dec,
    /// Whole miles at the class's cruising speed, rounded down. `None` if the
    /// class has no figures for it.
    pub range_nm: Option<u32>,
}

/// A task force in one line.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ForceLine {
    pub task_force: Handle,
    pub place: Place,
    /// `idle`, `underway`, `arrived`, `out_of_fuel` or `away_at_yard`
    /// (decision 0069).
    pub status: &'static str,
    pub mission: &'static str,
    /// `cruise`, `full` or knots, such as `24 kn`.
    pub speed: String,
    /// The last waypoint of the route, while there is one still to reach.
    pub destination: Option<Place>,
    /// `normal` or `battle`.
    pub readiness: &'static str,
    pub alert: bool,
    pub fatigue: Dec,
    pub ships_afloat: u32,
    /// The ship afloat with the least fuel (the lowest number if two tie).
    pub least_fuelled: Option<FuelLine>,
    /// `normal`, `slowed` or `cannot_move`. A force steams at the speed of
    /// its slowest ship, and a ship whose propulsion is out has no speed, so
    /// one such ship stops the whole force whatever its route and status
    /// say; a ship with propulsion degraded slows it.
    pub movement: &'static str,
    /// The ships afloat whose propulsion is not working, with the state each
    /// is in.
    pub propulsion_damaged: Vec<PropulsionLine>,
    /// Tons of supply carried as cargo, summed over the ships afloat. Set
    /// when above zero (here or in `cargo_fuel_t`) or when the force's
    /// mission is transport; `None` otherwise.
    pub cargo_supply_t: Option<Dec>,
    /// Tons of fuel carried as cargo (not the fuel the ships burn), summed
    /// over the ships afloat. Set under the same condition.
    pub cargo_fuel_t: Option<Dec>,
    /// The waypoints of the force's route it is marked to act at (decision
    /// 0096, 1), in hex order: only those still ahead of it, and never the
    /// last waypoint, which needs no mark. Left out when none.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub act_at: Vec<Place>,
    /// The force's halt, while it is halted at a marked waypoint (decision
    /// 0096, 3). Left out when it is not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub halt: Option<HaltLine>,
}

/// A force halted at a marked waypoint to act there (decision 0096).
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct HaltLine {
    /// Where it lies: the centre of the hex.
    pub place: Place,
    /// The clock time it halted, such as `D1 22:00`.
    pub since: String,
    /// What it waits to do: `bombard`, `cargo` or both, in that order.
    pub waits_for: Vec<&'static str>,
    /// Whether it has bombarded since it halted. Left out until it has.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub bombarded: bool,
}

/// A ship afloat whose propulsion is not working.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct PropulsionLine {
    pub ship: Handle,
    /// `degraded` or `out`.
    pub propulsion: &'static str,
}

/// An air group in one line.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct AirGroupLine {
    pub air_group: Handle,
    pub aircraft_type: String,
    /// The base or the ship it is at; `None` for a group lost with a base
    /// that changed side (decision 0090, 20), which has no place.
    pub location: Option<Handle>,
    /// Set only for a group lost with its base (decision 0090, 20): it has
    /// no place, no aircraft and takes no order. Left out otherwise.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub lost_with_base: bool,
    pub place: Option<Place>,
    pub ready: u32,
    pub damaged: u32,
    /// `None` if the scenario does not track this group's pilots.
    pub pilots: Option<u32>,
    pub fatigue: Dec,
    pub stood_down: bool,
    pub rest_above: Option<Dec>,
    pub mission: &'static str,
    /// The base a base strike is ordered at.
    pub target: Option<Handle>,
    /// The aircraft type's strike radius in miles: how far from the group's
    /// place a strike can be flown (for a fighter, the radius within which
    /// its escorts go with a raid). Zero for a type that cannot strike.
    pub strike_radius_nm: Dec,
    /// The aircraft type's search range in miles: the furthest a search arc
    /// can reach. Zero for a type that cannot search.
    pub search_range_nm: Dec,
    /// The group's reach in force in miles, given only when it is on
    /// extended reach (decision 0059); on normal reach the reach is
    /// `strike_radius_nm` and the field is left out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reach_nm: Option<Dec>,
    /// The kinds of ship the group's strikes go for, in order, given only
    /// when the list is not the default (every kind in the order of
    /// importance; decision 0063).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_priority: Option<Vec<&'static str>>,
    /// The contact the group would launch a strike at if the strike decision
    /// were made now (decision 6 of 0063): `SideView::strike_choice`, the
    /// function the decision itself calls. Left out for a group that is not
    /// on naval strike, has no ready aircraft, or would launch at nothing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub would_strike: Option<WouldStrike>,
    /// Set only when the group was moved to another base by a rebase order
    /// on the day just resolved (decision 0091): it flew nothing on that
    /// day, and its place is the new base. Left out otherwise.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub moved_yesterday: bool,
}

/// The contact a naval strike group would launch at now.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct WouldStrike {
    pub contact: Handle,
    /// The kind the contact is taken for.
    pub taken_for: &'static str,
    /// The contact's air distance from the group, in whole miles.
    pub distance_nm: u32,
}

/// A base in one line.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct BaseLine {
    pub base: Handle,
    pub place: Place,
    pub supply_t: Dec,
    pub fuel_t: Dec,
    /// 0 if the hex has no port.
    pub port_size: u8,
    /// 0 if the hex has no airfield.
    pub airfield_size: u8,
}

/// A contact in one line.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ContactLine {
    pub contact: Handle,
    pub place: Place,
    pub ship_count: u32,
    /// What the report says the most important ship is.
    pub taken_for: &'static str,
    pub age_pulses: u32,
}

pub(crate) fn fuel_line(ctx: &Ctx<'_>, ship: &sim_core::state::Ship) -> FuelLine {
    FuelLine {
        ship: ctx.ship(ship.id),
        fuel_t: Dec(ship.fuel_t),
        range_nm: cruise_range_nm(&ctx.view, ship),
    }
}

pub(crate) fn force_line(ctx: &Ctx<'_>, force: &TaskForce) -> ForceLine {
    if force.yard_trip.is_some() {
        return away_force_line(ctx, force);
    }
    let afloat: Vec<_> = ctx.afloat_in(force).collect();
    let least = afloat
        .iter()
        .copied()
        .min_by_key(|s| (s.fuel_t, s.id))
        .map(|s| fuel_line(ctx, s));
    let destination = force
        .route
        .waypoints
        .get(force.route.next_index)
        .and_then(|_| force.route.waypoints.last())
        .map(|hex| ctx.place(*hex));
    let propulsion_damaged: Vec<PropulsionLine> = afloat
        .iter()
        .filter(|s| s.systems.propulsion != SystemState::Working)
        .map(|s| PropulsionLine {
            ship: ctx.ship(s.id),
            propulsion: words::system_state(s.systems.propulsion),
        })
        .collect();
    let movement = if afloat
        .iter()
        .any(|s| s.systems.propulsion == SystemState::Out)
    {
        "cannot_move"
    } else if propulsion_damaged.is_empty() {
        "normal"
    } else {
        "slowed"
    };
    let cargo_supply = afloat.iter().fold(Fixed::ZERO, |sum, s| sum + s.supply_t);
    let cargo_fuel = afloat
        .iter()
        .fold(Fixed::ZERO, |sum, s| sum + s.cargo_fuel_t);
    let shows_cargo = force.mission == Mission::Transport
        || cargo_supply > Fixed::ZERO
        || cargo_fuel > Fixed::ZERO;
    ForceLine {
        cargo_supply_t: shows_cargo.then_some(Dec(cargo_supply)),
        cargo_fuel_t: shows_cargo.then_some(Dec(cargo_fuel)),
        task_force: ctx.force(force.id),
        place: ctx.place(force.position()),
        status: words::status(force.status),
        mission: words::mission(force.mission),
        speed: words::speed(force.speed),
        destination,
        readiness: force.readiness.name(),
        alert: force.is_alert(),
        fatigue: Dec(force.fatigue),
        ships_afloat: u32::try_from(afloat.len()).expect("a force has few ships"),
        least_fuelled: least,
        movement,
        propulsion_damaged,
        act_at: marks_ahead(ctx, force),
        halt: force.halt.map(|halt| HaltLine {
            place: ctx.place(force.position()),
            since: sim_core::time::ClockTime::from_elapsed(halt.at_hours).to_string(),
            waits_for: halt_work_words(halt.work),
            bombarded: halt.bombarded.is_some(),
        }),
    }
}

/// The distinct hexes of the route's waypoints still ahead that it is marked
/// to act at, in hex order (decision 0096, 1). A halted force's route still
/// points at the waypoint it lies at, so that hex is included while it lies
/// there.
fn marks_ahead(ctx: &Ctx<'_>, force: &TaskForce) -> Vec<Place> {
    let route = &force.route;
    let marks: std::collections::BTreeSet<Hex> = (route.next_index..route.waypoints.len())
        .filter(|i| route.marked(*i))
        .map(|i| route.waypoints[i])
        .collect();
    marks.into_iter().map(|hex| ctx.place(hex)).collect()
}

/// The line of a force away at the yard (decision 0069): at its port, with
/// its ships (which are off the map) counted, and nothing of fuel or
/// movement to say.
fn away_force_line(ctx: &Ctx<'_>, force: &TaskForce) -> ForceLine {
    let at_yard = ctx
        .ships
        .values()
        .filter(|s| s.location == ShipLocation::Yard(force.id))
        .count();
    ForceLine {
        task_force: ctx.force(force.id),
        place: ctx.place(force.position()),
        status: "away_at_yard",
        mission: words::mission(force.mission),
        speed: words::speed(force.speed),
        destination: None,
        readiness: force.readiness.name(),
        alert: false,
        fatigue: Dec(force.fatigue),
        ships_afloat: u32::try_from(at_yard).expect("a force has few ships"),
        least_fuelled: None,
        movement: "normal",
        propulsion_damaged: Vec::new(),
        cargo_supply_t: None,
        cargo_fuel_t: None,
        act_at: Vec::new(),
        halt: None,
    }
}

pub(crate) fn air_group_line(ctx: &Ctx<'_>, group: &AirGroup) -> AirGroupLine {
    let location = match group.location {
        sim_core::state::AirGroupLocation::Base(base) => Some(ctx.base(base)),
        sim_core::state::AirGroupLocation::Ship(ship) => Some(ctx.ship(ship)),
        sim_core::state::AirGroupLocation::Lost => None,
    };
    let target = match group.mission {
        AirMission::BaseStrike(base) => Some(ctx.base(base)),
        _ => None,
    };
    let aircraft_type = ctx.view.aircraft_type(group.aircraft_type);
    AirGroupLine {
        strike_radius_nm: Dec(aircraft_type.map_or(Fixed::ZERO, |t| t.strike_range_nm)),
        search_range_nm: Dec(aircraft_type.map_or(Fixed::ZERO, |t| t.search_range_nm)),
        reach_nm: (group.reach == Reach::Extended).then(|| Dec(ctx.view.reach_nm(group))),
        target_priority: (group.target_priority != ShipKind::ALL).then(|| {
            group
                .target_priority
                .iter()
                .map(|kind| kind.name())
                .collect()
        }),
        would_strike: ctx
            .view
            .strike_choice(group.id)
            .filter(|_| group.ready > 0)
            .map(|choice| WouldStrike {
                contact: ctx.entity(EntityRef::Contact(choice.contact)),
                taken_for: choice.kind.name(),
                distance_nm: u32::try_from(choice.distance_nm.trunc())
                    .expect("an air distance is a whole number of miles"),
            }),
        moved_yesterday: group.rebased_day == Some(ctx.view.day()),
        air_group: ctx.group(group.id),
        aircraft_type: aircraft_type
            .map_or_else(|| group.aircraft_type.to_string(), |t| t.name.clone()),
        lost_with_base: location.is_none(),
        location,
        place: ctx.group_hex(group).map(|hex| ctx.place(hex)),
        ready: group.ready,
        damaged: group.damaged,
        pilots: group.pilots,
        fatigue: Dec(group.fatigue),
        stood_down: group.stood_down,
        rest_above: group.rest_above.map(Dec),
        mission: words::air_mission(group.mission),
        target,
    }
}

pub(crate) fn base_line(ctx: &Ctx<'_>, base: &Base) -> BaseLine {
    let data = ctx.view.hex(base.hex);
    BaseLine {
        base: ctx.base(base.id),
        place: ctx.place(base.hex),
        supply_t: Dec(base.supply_t),
        fuel_t: Dec(base.fuel_t),
        port_size: data.map_or(0, |d| d.port_size),
        airfield_size: data.map_or(0, |d| d.airfield_size),
    }
}

/// A force with a ship afloat. The briefing and the lists show these only.
pub(crate) fn is_afloat(ctx: &Ctx<'_>, force: &TaskForce) -> bool {
    ctx.afloat_in(force).next().is_some()
}

/// A force the briefing and the lists show: one with a ship afloat, or one
/// away at the yard with its ships (decision 0069).
pub(crate) fn is_listed(ctx: &Ctx<'_>, force: &TaskForce) -> bool {
    is_afloat(ctx, force) || force.yard_trip.is_some()
}

/// Men of a land unit aboard one ship (decision 0066, 21).
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct AboardLine {
    pub ship: Handle,
    pub men: u32,
}

/// A land unit of the side in one line (decision 0066, 21): its men ashore
/// and where, its men aboard by ship, and whether its men were fed last
/// night. Only a side's own units have one; there is none for the enemy's
/// (decision 0066, 22).
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct LandUnitLine {
    pub land_unit: Handle,
    pub men_ashore: u32,
    /// The hex the men ashore are in; `None` when none is ashore.
    pub place: Option<Place>,
    /// The men aboard the side's ships, all together.
    pub men_aboard: u32,
    /// The ships that carry them, in ship order.
    pub aboard: Vec<AboardLine>,
    /// No man left, ashore or aboard. The unit keeps its number.
    pub destroyed: bool,
    /// `fed`, `partly_fed` or `unfed`: how the side's men ashore in the
    /// unit's hex were fed in the last night pulse, from the side's own
    /// `MenFed` event. `not_ashore` for a unit with no man ashore, and
    /// `not_known` when the side holds no such event for the hex (no night
    /// has been resolved, or the unit came ashore since).
    pub fed_last_night: &'static str,
    /// `defend` or `attack` (decision 0090, 1).
    pub posture: &'static str,
    /// How tired the unit is, 0 to 100 (decision 0090, 11). It cuts the men
    /// who fire and nothing else.
    pub fatigue: Dec,
    /// The share of the unit's men fed at the last night's charge, 0 to 1
    /// (decision 0090, 8): 1 until the first night and for a unit that had
    /// no man ashore at the charge.
    pub fed_share: Dec,
    /// The number of the side's standing posture order, if one is in force.
    /// Left out when none is.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posture_order: Option<u32>,
    /// The unit's march, if it has a march order standing (decision 0094, 11):
    /// left out when it has none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub march: Option<MarchLine>,
    /// The hexes the unit's men ashore can be ordered to march to: the
    /// neighbours the map joins to their hex by land, in hex order (decision
    /// 0094, 1 and 3). Left out when there are none, as on a map with no land
    /// links and for a unit with no man ashore.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub march_to: Vec<Place>,
}

/// A land unit's march in progress (decision 0094, 4): where to, the days
/// marched and the days the march takes. The unit is where it was until it
/// has marched them all.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct MarchLine {
    pub to: Place,
    pub days_marched: u32,
    /// The days a march takes: the scenario's `march_days_per_hex`.
    pub of_days: u32,
    /// The number of the side's march order, if it is still known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<u32>,
}

/// How the side's men ashore were fed in the last night pulse, by hex, from
/// the side's own `MenFed` events (decision 0066, 16 and 17).
pub(crate) fn fed_last_night(ctx: &Ctx<'_>) -> BTreeMap<Hex, &'static str> {
    let day = ctx.view.day();
    let mut out = BTreeMap::new();
    for e in ctx.view.events() {
        if e.day != day || e.pulse != Some(Pulse::Night) {
            continue;
        }
        if let SideEventKind::Whole(EventKind::MenFed {
            hex,
            men,
            fed,
            unfed,
            ..
        }) = &e.kind
        {
            let word = if *unfed == 0 && fed == men {
                "fed"
            } else if *fed == 0 {
                "unfed"
            } else {
                "partly_fed"
            };
            out.insert(*hex, word);
        }
    }
    out
}

/// The ships of the side that carry men of `unit`, afloat or at the yard.
fn aboard_lines(ctx: &Ctx<'_>, unit: &LandUnit) -> Vec<AboardLine> {
    ctx.ships
        .values()
        .filter(|s| s.location != ShipLocation::Sunk)
        .filter_map(|s| {
            let troops = s.troops?;
            (troops.unit == unit.id).then(|| AboardLine {
                ship: ctx.ship(s.id),
                men: troops.men,
            })
        })
        .collect()
}

/// Where a land unit is: the hex of its men ashore, else the hex of the first
/// ship that carries it.
pub(crate) fn land_unit_hex(ctx: &Ctx<'_>, unit: &LandUnit) -> Option<Hex> {
    unit.hex.or_else(|| {
        ctx.ships
            .values()
            .filter(|s| s.troops.is_some_and(|t| t.unit == unit.id))
            .find_map(|s| ctx.ship_hex(s))
    })
}

pub(crate) fn land_unit_line(
    ctx: &Ctx<'_>,
    unit: &LandUnit,
    fed: &BTreeMap<Hex, &'static str>,
) -> LandUnitLine {
    let men_aboard = ctx.view.men_aboard(unit.id).unwrap_or(0);
    LandUnitLine {
        land_unit: ctx.land_unit(unit.id),
        men_ashore: unit.men_ashore,
        place: unit.hex.map(|hex| ctx.place(hex)),
        men_aboard,
        aboard: aboard_lines(ctx, unit),
        destroyed: unit.men_ashore == 0 && men_aboard == 0,
        fed_last_night: match unit.hex {
            None => "not_ashore",
            Some(hex) => fed.get(&hex).copied().unwrap_or("not_known"),
        },
        posture: unit.posture.name(),
        fatigue: Dec(unit.fatigue),
        fed_share: Dec(unit.fed_share),
        posture_order: unit.posture_order.and_then(|o| ctx.order_number_if_own(o)),
        march: unit.march.map(|march| MarchLine {
            to: ctx.place(march.to),
            days_marched: march.days,
            of_days: ctx
                .view
                .ground_figures()
                .map_or(0, |figures| figures.march_days_per_hex),
            order: ctx.order_number_if_own(march.order),
        }),
        march_to: ctx
            .view
            .march_destinations(unit.id)
            .unwrap_or_default()
            .into_iter()
            .map(|hex| ctx.place(hex))
            .collect(),
    }
}

/// What a halt is for, in words: `bombard` and `cargo`, in that order.
pub(crate) fn halt_work_words(work: sim_core::state::HaltWork) -> Vec<&'static str> {
    [(work.bombard, "bombard"), (work.cargo, "cargo")]
        .into_iter()
        .filter_map(|(on, word)| on.then_some(word))
        .collect()
}
