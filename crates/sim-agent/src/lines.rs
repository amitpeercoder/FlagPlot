//! One line for a unit: shared by the briefing and the lists, so a unit is
//! summarised in one way.

use serde::Serialize;
use sim_core::fixed::Fixed;
use sim_core::orders::{AirMission, Mission};
use sim_core::state::{AirGroup, Base, SystemState, TaskForce};

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
    /// `idle`, `underway`, `arrived` or `out_of_fuel`.
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
    /// The base or the ship it is at.
    pub location: Handle,
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
    }
}

pub(crate) fn air_group_line(ctx: &Ctx<'_>, group: &AirGroup) -> AirGroupLine {
    let location = match group.location {
        sim_core::state::AirGroupLocation::Base(base) => ctx.base(base),
        sim_core::state::AirGroupLocation::Ship(ship) => ctx.ship(ship),
    };
    let target = match group.mission {
        AirMission::BaseStrike(base) => Some(ctx.base(base)),
        _ => None,
    };
    let aircraft_type = ctx.view.aircraft_type(group.aircraft_type);
    AirGroupLine {
        strike_radius_nm: Dec(aircraft_type.map_or(Fixed::ZERO, |t| t.strike_range_nm)),
        search_range_nm: Dec(aircraft_type.map_or(Fixed::ZERO, |t| t.search_range_nm)),
        air_group: ctx.group(group.id),
        aircraft_type: aircraft_type
            .map_or_else(|| group.aircraft_type.to_string(), |t| t.name.clone()),
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
