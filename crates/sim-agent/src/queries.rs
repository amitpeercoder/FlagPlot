//! The lists and `get_entity` (decisions 8 and 9 of 0048).

use serde::Serialize;
use sim_core::hex::Hex;
use sim_core::ids::EntityRef;
use sim_core::state::{Ship, ShipLocation, ShipSystems};
use sim_core::view::SideView;

use crate::answer::{NotKnown, FORMAT, VERSION};
use crate::briefing::{ground_estimate_lines, GroundEstimateLine};
use crate::common::{cruise_range_nm, day_not_come, Ctx, Dec, Handle, Place};
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
    /// The side's own forces and units held apart until their day (decision
    /// 0195, 7): not on the map, so not in `task_forces` or `land_units`. In
    /// `area` if one is given, an entry at a base by the base's hex where
    /// that is known. Left out when none is held.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub to_enter: Vec<crate::entries::EntryLine>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ListBases {
    pub format: &'static str,
    pub version: u32,
    pub kind: &'static str,
    pub area: Option<Area>,
    pub bases: Vec<BaseLine>,
    /// Each hex in `area` where the side holds more than one base, shown as
    /// one place: the stock of all its bases and the days of supply for the
    /// hex, once (decision 0110, item 8). The bases themselves are still
    /// listed in `bases`, each with how it was formed. Left out when no hex
    /// holds more than one.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub shared_hexes: Vec<crate::briefing::SharedHexLine>,
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
    /// The side's estimates of the enemy's men ashore (decision 0092, 19), in
    /// `area` if one is given: one for each hex, the last report there, with
    /// its day and age. They are estimates and may be wrong. Left out when
    /// the side holds none.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ground_estimates: Vec<GroundEstimateLine>,
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
        to_enter: view
            .entering()
            .filter(|entry| {
                area.is_none()
                    || match entry.place {
                        sim_core::entering::EntryPlace::Hex(hex) => within(area, hex),
                        sim_core::entering::EntryPlace::Base(base) => ctx
                            .bases
                            .get(&base)
                            .map(|b| b.hex)
                            .or_else(|| ctx.enemy_bases.get(&base).map(|b| b.hex))
                            .is_some_and(|hex| within(area, hex)),
                    }
            })
            .map(|entry| crate::entries::entry_line(&ctx, entry))
            .collect(),
    }
}

/// A line for each of the side's bases, in `area` if one is given.
pub fn list_bases(view: &SideView<'_>, area: Option<(Hex, u32)>) -> ListBases {
    let mut ctx = Ctx::new(*view);
    ctx.set_events(&view.events());
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
        shared_hexes: crate::briefing::shared_hexes(&ctx)
            .into_iter()
            .filter(|h| {
                area.is_none_or(|(centre, radius)| {
                    centre.air_distance_nm(sim_core::hex::Hex::new(h.place.q, h.place.r)) <= radius
                })
            })
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
    let mut ctx = Ctx::new(*view);
    // Only when there is an estimate: the numbers of the events are wanted
    // for the estimates alone.
    // The area command's picture (decision 0134, 13).
    let picture = crate::common::area_picture(view);
    if picture.ground_estimates().next().is_some() {
        ctx.set_events(&view.events());
    }
    let ground_estimates = ground_estimate_lines(&ctx, view, Some(&|hex| within(area, hex)));
    ListContacts {
        format: FORMAT,
        version: VERSION,
        kind: "list_contacts",
        area: area_out(area),
        contacts: picture
            .contacts()
            .filter(|c| within(area, c.estimated_hex))
            .map(|c| contact_line(&ctx, c))
            .collect(),
        ground_estimates,
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
    /// A unit held apart until its day (decision 0195, 7).
    ToEnter(crate::entries::EntryDetail),
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
    /// The day from which the side may use the unit (decision 0140, 5),
    /// given only while that day has not come. Left out otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_from: Option<u32>,
    pub class: String,
    pub ship_kind: Option<&'static str>,
    pub sunk: bool,
    pub task_force: Option<Handle>,
    pub port: Option<Handle>,
    pub place: Option<Place>,
    pub fuel_t: Dec,
    /// The fuel her class's tanks hold, so that `fuel_t` reads as a level.
    /// Left out where her class is not known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_capacity_t: Option<Dec>,
    pub supply_t: Dec,
    pub cargo_fuel_t: Dec,
    pub flotation_damage: Dec,
    pub fire_damage: Dec,
    /// Given (true) only for a carrier with a fire among her aircraft
    /// (decision 0212): while it burns she flies no aircraft off and lands
    /// none on. Left out otherwise. Only the side's own ships are asked about.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_fire_among_aircraft: Option<bool>,
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
    /// The men her class can carry (decision 0066, 4). Left out for a ship
    /// that carries none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub troop_capacity_men: Option<u32>,
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
    /// The formation the unit belongs to and the formations above it up to
    /// the top (decision 0140, 8), written only where it has one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formation: Option<FormationLine>,
}

/// A formation by number and name (decision 0140, 8).
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct FormationRef {
    pub number: u32,
    pub name: String,
}

/// A land unit's formation, with the chain of parents above it, the nearest
/// first.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct FormationLine {
    pub number: u32,
    pub name: String,
    /// The formation it belongs to, then that one's parent, up to the top.
    /// Left out when it has no parent.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parents: Vec<FormationRef>,
}

/// The unit's formation and its parents, from the side's own formations.
fn formation_line(view: &SideView<'_>, unit: &sim_core::state::LandUnit) -> Option<FormationLine> {
    let own: std::collections::BTreeMap<_, _> = view.formations().map(|f| (f.id, f)).collect();
    let first = own.get(&unit.formation?)?;
    let mut parents = Vec::new();
    let mut next = first.parent;
    // The loader refuses a loop; the bound keeps a bad state from spinning.
    while let Some(parent) = next.and_then(|id| own.get(&id)) {
        if parents.len() > own.len() {
            break;
        }
        parents.push(FormationRef {
            number: parent.id.raw(),
            name: parent.name.clone(),
        });
        next = parent.parent;
    }
    Some(FormationLine {
        number: first.id.raw(),
        name: first.name.clone(),
        parents,
    })
}

/// A ship in a task force's detail.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ShipLine {
    pub ship: Handle,
    /// The day from which the side may use the unit (decision 0140, 5),
    /// given only while that day has not come. Left out otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_from: Option<u32>,
    pub class: String,
    pub sunk: bool,
    pub fuel_t: Dec,
    /// The fuel her class's tanks hold, so that `fuel_t` reads as a level.
    /// Left out where her class is not known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_capacity_t: Option<Dec>,
    /// Given (true) only for a ship whose class fuels others at sea, an
    /// oiler (decision 0201). Left out otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuels_at_sea: Option<bool>,
    pub flotation_damage: Dec,
    pub fire_damage: Dec,
    /// Given (true) only for a carrier with a fire among her aircraft
    /// (decision 0212): while it burns she flies no aircraft off and lands
    /// none on. Left out otherwise. As in [`ShipDetail`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_fire_among_aircraft: Option<bool>,
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
    /// The force it escorts and keeps station on (decision 0121), written
    /// only where it has one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<Handle>,
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
    let mut ctx = Ctx::new(*view);
    if kind == "base" {
        ctx.set_events(&view.events());
    }
    // A unit held apart is the side's own but not on the map: what it is
    // and when and where it enters (decision 0195, 7).
    let held = match kind {
        "ship" => ctx
            .held_ships
            .keys()
            .find(|s| ctx.ship_number(**s) == Some(id))
            .map(|s| EntityRef::Ship(*s)),
        "task_force" => ctx
            .held_forces
            .keys()
            .find(|f| ctx.force_number(**f) == Some(id))
            .map(|f| EntityRef::TaskForce(*f)),
        "air_group" => ctx
            .held_groups
            .keys()
            .find(|g| ctx.group_number(**g) == Some(id))
            .map(|g| EntityRef::AirGroup(*g)),
        "land_unit" => ctx
            .held_land_units
            .keys()
            .find(|u| ctx.land_unit_number(**u) == Some(id))
            .map(|u| EntityRef::LandUnit(*u)),
        _ => None,
    };
    if let Some(unit) = held {
        return crate::entries::entry_detail(&ctx, unit).map(Detail::ToEnter);
    }
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
                available_from: day_not_come(view, EntityRef::Ship(ship.id)),
                class: class_name(&ctx, ship),
                ship_kind: view.ship_class(ship.class).map(|c| c.kind.name()),
                sunk: ship.location == ShipLocation::Sunk,
                task_force,
                port,
                place: ctx.ship_hex(ship).map(|h| ctx.place(h)),
                fuel_t: Dec(ship.fuel_t),
                fuel_capacity_t: view.ship_class(ship.class).map(|c| Dec(c.fuel_capacity_t)),
                supply_t: Dec(ship.supply_t),
                cargo_fuel_t: Dec(ship.cargo_fuel_t),
                flotation_damage: Dec(ship.flotation_damage),
                fire_damage: Dec(ship.fire_damage),
                on_fire_among_aircraft: ship.aircraft_fire.is_some().then_some(true),
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
                troop_capacity_men: view
                    .ship_class(ship.class)
                    .map(|c| c.troop_capacity_men)
                    .filter(|men| *men > 0),
            }))
        }
        "land_unit" => {
            let unit = ctx.own_land_unit(id)?;
            Some(Detail::LandUnit(LandUnitDetail {
                entity_kind: "land_unit",
                summary: land_unit_line(&ctx, unit, &fed_last_night(&ctx)),
                formation: formation_line(view, unit),
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
                charge: force.charge.map(|c| ctx.force(c.force)),
                ships: force
                    .ships
                    .iter()
                    .filter_map(|s| ctx.ships.get(s))
                    .map(|s| ShipLine {
                        ship: ctx.ship(s.id),
                        available_from: day_not_come(view, EntityRef::Ship(s.id)),
                        class: class_name(&ctx, s),
                        sunk: s.location == ShipLocation::Sunk,
                        fuel_t: Dec(s.fuel_t),
                        fuel_capacity_t: view.ship_class(s.class).map(|c| Dec(c.fuel_capacity_t)),
                        fuels_at_sea: view
                            .ship_class(s.class)
                            .is_some_and(|c| c.fuels_at_sea)
                            .then_some(true),
                        flotation_damage: Dec(s.flotation_damage),
                        fire_damage: Dec(s.fire_damage),
                        on_fire_among_aircraft: s.aircraft_fire.is_some().then_some(true),
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
            let contact = crate::common::area_picture(view).contact_by_number(id)?;
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
