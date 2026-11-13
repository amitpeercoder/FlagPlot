//! The lists and `get_entity` (decisions 8 and 9 of 0048).

use serde::Serialize;
use sim_core::hex::Hex;
use sim_core::state::{Ship, ShipLocation, ShipSystems};
use sim_core::view::SideView;

use crate::answer::{NotKnown, FORMAT, VERSION};
use crate::common::{cruise_range_nm, Ctx, Dec, Handle, Place};
use crate::lines::{
    air_group_line, base_line, fed_last_night, force_line, is_listed, land_unit_hex,
    land_unit_line, AirGroupLine, BaseLine, ContactLine, ForceLine, LandUnitLine,
};
use crate::words;

/// The area a list was asked for: a hex and a radius in miles, by air
/// distance (decision 0040).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize)]
pub struct Area {
    pub q: i32,
    pub r: i32,
    pub radius_nm: u32,
}

fn within(area: Option<(Hex, u32)>, hex: Hex) -> bool {
    area.is_none_or(|(centre, radius)| centre.air_distance_nm(hex) <= radius)
}

fn area_out(area: Option<(Hex, u32)>) -> Option<Area> {
    area.map(|(hex, radius_nm)| Area {
        q: hex.q,
        r: hex.r,
        radius_nm,
    })
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ListForces {
    pub format: &'static str,
    pub version: u32,
    pub kind: &'static str,
    pub area: Option<Area>,
    pub task_forces: Vec<ForceLine>,
    /// The side's own land units (decision 0066, 21), in `area` if one is
    /// given: a unit with men ashore is where they are, one wholly aboard
    /// where its first ship is. Left out when the side has none. A destroyed
    /// unit is listed (it keeps its number) and is in no area.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub land_units: Vec<LandUnitLine>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ListBases {
    pub format: &'static str,
    pub version: u32,
    pub kind: &'static str,
    pub area: Option<Area>,
    pub bases: Vec<BaseLine>,
    /// The bases the side does not own: public facts only (decision 0050).
    pub enemy_bases: Vec<EnemyBaseLine>,
}

/// A base the side does not own, as every side knows it: number, name, place
/// and the side that holds it. Nothing else is known.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct EnemyBaseLine {
    pub base: Handle,
    pub place: Place,
    /// `allied` or `japanese`.
    pub held_by: &'static str,
}

fn enemy_base_line(ctx: &Ctx<'_>, base: &sim_core::view::PublicBase) -> EnemyBaseLine {
    EnemyBaseLine {
        base: ctx.base(base.id),
        place: ctx.place(base.hex),
        held_by: crate::record::seat_name(base.side),
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ListContacts {
    pub format: &'static str,
    pub version: u32,
    pub kind: &'static str,
    pub area: Option<Area>,
    pub contacts: Vec<ContactLine>,
}

/// A line for each of the side's task forces that has a ship afloat, in
/// `area` if one is given.
pub fn list_forces(view: &SideView<'_>, area: Option<(Hex, u32)>) -> ListForces {
    let ctx = Ctx::new(*view);
    ListForces {
        format: FORMAT,
        version: VERSION,
        kind: "list_forces",
        area: area_out(area),
        task_forces: ctx
            .forces
            .values()
            .filter(|f| is_listed(&ctx, f) && within(area, f.position()))
            .map(|f| force_line(&ctx, f))
            .collect(),
        land_units: {
            let fed = fed_last_night(&ctx);
            ctx.land_units
                .values()
                .filter(|u| {
                    area.is_none() || land_unit_hex(&ctx, u).is_some_and(|hex| within(area, hex))
                })
                .map(|u| land_unit_line(&ctx, u, &fed))
                .collect()
        },
    }
}

/// A line for each of the side's bases, in `area` if one is given.
pub fn list_bases(view: &SideView<'_>, area: Option<(Hex, u32)>) -> ListBases {
    let ctx = Ctx::new(*view);
    ListBases {
        format: FORMAT,
        version: VERSION,
        kind: "list_bases",
        area: area_out(area),
        bases: ctx
            .bases
            .values()
            .filter(|b| within(area, b.hex))
            .map(|b| base_line(&ctx, b))
            .collect(),
        enemy_bases: ctx
            .enemy_bases
            .values()
            .filter(|b| within(area, b.hex))
            .map(|b| enemy_base_line(&ctx, b))
            .collect(),
    }
}

/// A line for each contact the side holds, in `area` if one is given.
pub fn list_contacts(view: &SideView<'_>, area: Option<(Hex, u32)>) -> ListContacts {
    let ctx = Ctx::new(*view);
    ListContacts {
        format: FORMAT,
        version: VERSION,
        kind: "list_contacts",
        area: area_out(area),
        contacts: view
            .contacts()
            .filter(|c| within(area, c.estimated_hex))
            .map(|c| contact_line(&ctx, c))
            .collect(),
    }
}

fn contact_line(ctx: &Ctx<'_>, c: &sim_core::belief::Contact) -> ContactLine {
    ContactLine {
        contact: ctx.entity(sim_core::ids::EntityRef::Contact(c.id)),
        place: ctx.place(c.estimated_hex),
        ship_count: c.estimated_ship_count,
        taken_for: c.estimated_kind.name(),
        age_pulses: c.age_pulses,
    }
}

/// The answer to `get_entity`: the entity, or the one answer given for
/// everything the side does not know.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
#[serde(untagged)]
pub enum EntityAnswer {
    Found(Box<Entity>),
    NotKnown(NotKnown),
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct Entity {
    pub format: &'static str,
    pub version: u32,
    pub kind: &'static str,
    pub entity: Detail,
}

/// Everything the side knows of one unit or contact. Each detail names its
/// own kind in `entity_kind`.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
#[serde(untagged)]
pub enum Detail {
    Ship(ShipDetail),
    TaskForce(ForceDetail),
    AirGroup(AirGroupDetail),
    LandUnit(LandUnitDetail),
    Base(BaseDetail),
    EnemyBase(EnemyBaseDetail),
    Contact(ContactDetail),
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct SystemStates {
    pub propulsion: &'static str,
    pub fire_control: &'static str,
    pub main_battery: &'static str,
    pub torpedo_mounts: &'static str,
    pub flight_deck: &'static str,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ShipDetail {
    pub entity_kind: &'static str,
    pub ship: Handle,
    pub class: String,
    pub ship_kind: Option<&'static str>,
    pub sunk: bool,
    pub task_force: Option<Handle>,
    pub port: Option<Handle>,
    pub place: Option<Place>,
    pub fuel_t: Dec,
    pub supply_t: Dec,
    pub cargo_fuel_t: Dec,
    pub flotation_damage: Dec,
    pub fire_damage: Dec,
    pub gun_rounds: Dec,
    pub torpedoes_ready: bool,
    pub systems: SystemStates,
    /// The crew's experience, 0 to 4, shown only when it is not the standard
    /// 2 (decision 0064, 12). Only the side's own ships are asked about.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience: Option<u8>,
    /// Whole miles at cruising speed on the fuel aboard.
    pub range_nm: Option<u32>,
    pub air_groups: Vec<Handle>,
    /// The men of a land unit the ship carries (decision 0066, 21). Left out
    /// when she carries none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub troops: Option<TroopsAboard>,
}

/// The men a ship carries and whose they are.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct TroopsAboard {
    pub land_unit: Handle,
    pub men: u32,
}

fn troops_aboard(ctx: &Ctx<'_>, ship: &Ship) -> Option<TroopsAboard> {
    ship.troops.map(|t| TroopsAboard {
        land_unit: ctx.land_unit(t.unit),
        men: t.men,
    })
}

/// A land unit of the side in detail (decision 0066, 21).
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct LandUnitDetail {
    pub entity_kind: &'static str,
    pub summary: LandUnitLine,
}

/// A ship in a task force's detail.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ShipLine {
    pub ship: Handle,
    pub class: String,
    pub sunk: bool,
    pub fuel_t: Dec,
    pub flotation_damage: Dec,
    pub fire_damage: Dec,
    /// As in [`ShipDetail`]: shown only when it is not 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience: Option<u8>,
    /// As in [`ShipDetail`]: left out when she carries no men.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub troops: Option<TroopsAboard>,
}

/// A ship's experience as the side's own view shows it: nothing when it is
/// the standard value (decision 0064, 12).
fn shown_experience(ship: &sim_core::state::Ship) -> Option<u8> {
    (ship.experience != sim_core::state::Ship::DEFAULT_EXPERIENCE).then_some(ship.experience)
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ForceDetail {
    pub entity_kind: &'static str,
    pub summary: ForceLine,
    pub route: Vec<Place>,
    /// The index in `route` of the waypoint being steamed to; equal to the
    /// length of `route` once the route is complete.
    pub next_waypoint: u32,
    /// `engage` or `avoid`.
    pub stance: &'static str,
    pub break_off: Dec,
    pub home_port: Option<Handle>,
    pub ships: Vec<ShipLine>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct AirGroupDetail {
    pub entity_kind: &'static str,
    pub summary: AirGroupLine,
    pub patrol_share: Dec,
    pub strength: u32,
    pub search_from_deg: Option<u32>,
    pub search_to_deg: Option<u32>,
    pub search_range_nm: Option<Dec>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct BaseDetail {
    pub entity_kind: &'static str,
    pub summary: BaseLine,
    pub anti_aircraft: u32,
    pub aircraft_repairs_per_day: u32,
    pub air_groups: Vec<Handle>,
    pub ships_in_port: Vec<Handle>,
}

/// What is known of a base the side does not own: the public facts, and a
/// statement that nothing more is known.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct EnemyBaseDetail {
    pub entity_kind: &'static str,
    pub summary: EnemyBaseLine,
    /// Always `"number, name, place and holder only"`.
    pub known: &'static str,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ContactDetail {
    pub entity_kind: &'static str,
    pub summary: ContactLine,
    /// The side's number of the event that recorded the report that last
    /// made or refreshed it.
    pub last_report_event: Option<u32>,
}

fn systems(s: &ShipSystems) -> SystemStates {
    SystemStates {
        propulsion: words::system_state(s.propulsion),
        fire_control: words::system_state(s.fire_control),
        main_battery: words::system_state(s.main_battery),
        torpedo_mounts: words::system_state(s.torpedo_mounts),
        flight_deck: words::system_state(s.flight_deck),
    }
}

fn class_name(ctx: &Ctx<'_>, ship: &Ship) -> String {
    ctx.view
        .ship_class(ship.class)
        .map_or_else(|| ship.class.to_string(), |c| c.name.clone())
}

/// Everything the side knows of one of its own task forces, ships, air
/// groups, land units or bases, or of one contact it holds, named `kind:number` as the log
/// names it. For anything else the answer is [`NotKnown`], the same document
/// whether the number is an enemy unit, does not exist or is not a handle.
pub fn get_entity(view: &SideView<'_>, handle: &str) -> EntityAnswer {
    match entity_of(view, handle) {
        Some(detail) => EntityAnswer::Found(Box::new(Entity {
            format: FORMAT,
            version: VERSION,
            kind: "entity",
            entity: detail,
        })),
        None => EntityAnswer::NotKnown(NotKnown::of("get_entity")),
    }
}

fn entity_of(view: &SideView<'_>, handle: &str) -> Option<Detail> {
    let (kind, number) = handle.split_once(':')?;
    let id: u32 = number.parse().ok()?;
    let ctx = Ctx::new(*view);
    match kind {
        "ship" => {
            let ship = ctx.own_ship(id)?;
            let (task_force, port) = match ship.location {
                // Away at the yard with the force named (decision 0069).
                ShipLocation::TaskForce(f) | ShipLocation::Yard(f) => (Some(ctx.force(f)), None),
                ShipLocation::Port(b) => (None, Some(ctx.base(b))),
                ShipLocation::Sunk => (None, None),
            };
            Some(Detail::Ship(ShipDetail {
                entity_kind: "ship",
                ship: ctx.ship(ship.id),
                class: class_name(&ctx, ship),
                ship_kind: view.ship_class(ship.class).map(|c| c.kind.name()),
                sunk: ship.location == ShipLocation::Sunk,
                task_force,
                port,
                place: ctx.ship_hex(ship).map(|h| ctx.place(h)),
                fuel_t: Dec(ship.fuel_t),
                supply_t: Dec(ship.supply_t),
                cargo_fuel_t: Dec(ship.cargo_fuel_t),
                flotation_damage: Dec(ship.flotation_damage),
                fire_damage: Dec(ship.fire_damage),
                gun_rounds: Dec(ship.gun_rounds),
                torpedoes_ready: ship.torpedoes_ready,
                systems: systems(&ship.systems),
                experience: shown_experience(ship),
                range_nm: cruise_range_nm(view, ship),
                air_groups: ctx
                    .groups
                    .values()
                    .filter(|g| g.location == sim_core::state::AirGroupLocation::Ship(ship.id))
                    .map(|g| ctx.group(g.id))
                    .collect(),
                troops: troops_aboard(&ctx, ship),
            }))
        }
        "land_unit" => {
            let unit = ctx.own_land_unit(id)?;
            Some(Detail::LandUnit(LandUnitDetail {
                entity_kind: "land_unit",
                summary: land_unit_line(&ctx, unit, &fed_last_night(&ctx)),
            }))
        }
        "task_force" => {
            let force = ctx.own_force(id)?;
            Some(Detail::TaskForce(ForceDetail {
                entity_kind: "task_force",
                summary: force_line(&ctx, force),
                route: force
                    .route
                    .waypoints
                    .iter()
                    .map(|h| ctx.place(*h))
                    .collect(),
                next_waypoint: u32::try_from(force.route.next_index)
                    .expect("a route has few waypoints"),
                stance: words::stance(force.rules_of_engagement.stance),
                break_off: Dec(force.rules_of_engagement.break_off),
                home_port: force.home_port.map(|b| ctx.base(b)),
                ships: force
                    .ships
                    .iter()
                    .filter_map(|s| ctx.ships.get(s))
                    .map(|s| ShipLine {
                        ship: ctx.ship(s.id),
                        class: class_name(&ctx, s),
                        sunk: s.location == ShipLocation::Sunk,
                        fuel_t: Dec(s.fuel_t),
                        flotation_damage: Dec(s.flotation_damage),
                        fire_damage: Dec(s.fire_damage),
                        experience: shown_experience(s),
                        troops: troops_aboard(&ctx, s),
                    })
                    .collect(),
            }))
        }
        "air_group" => {
            let group = ctx.own_group(id)?;
            Some(Detail::AirGroup(AirGroupDetail {
                entity_kind: "air_group",
                summary: air_group_line(&ctx, group),
                patrol_share: Dec(group.patrol_share),
                strength: group.strength,
                search_from_deg: group.search.as_ref().map(|a| a.from_deg),
                search_to_deg: group.search.as_ref().map(|a| a.to_deg),
                search_range_nm: group.search.as_ref().map(|a| Dec(a.range_nm)),
            }))
        }
        "base" => {
            let base_id = sim_core::ids::BaseId::new(id);
            if let Some(public) = ctx.enemy_bases.get(&base_id) {
                return Some(Detail::EnemyBase(EnemyBaseDetail {
                    entity_kind: "enemy_base",
                    summary: enemy_base_line(&ctx, public),
                    known: "number, name, place and holder only",
                }));
            }
            let base = *ctx.bases.get(&base_id)?;
            Some(Detail::Base(BaseDetail {
                entity_kind: "base",
                summary: base_line(&ctx, base),
                anti_aircraft: base.anti_aircraft,
                aircraft_repairs_per_day: base.aircraft_repairs_per_day,
                air_groups: ctx
                    .groups
                    .values()
                    .filter(|g| g.location == sim_core::state::AirGroupLocation::Base(base.id))
                    .map(|g| ctx.group(g.id))
                    .collect(),
                ships_in_port: ctx
                    .ships
                    .values()
                    .filter(|s| s.location == ShipLocation::Port(base.id))
                    .map(|s| ctx.ship(s.id))
                    .collect(),
            }))
        }
        "contact" => {
            let contact = view.contact_by_number(id)?;
            let events = view.events();
            let number = events
                .iter()
                .position(|e| e.id == contact.last_report_event)
                .map(|i| u32::try_from(i + 1).expect("few events"));
            Some(Detail::Contact(ContactDetail {
                entity_kind: "contact",
                summary: contact_line(&ctx, contact),
                last_report_event: number,
            }))
        }
        _ => None,
    }
}
