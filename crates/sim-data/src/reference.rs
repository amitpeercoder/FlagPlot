use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde::Deserialize;
use sim_core::fixed::Fixed;
use sim_core::ids::{AircraftTypeId, ShipClassId};
use sim_core::refdata::{
    AirCombatFigures, AirSupplyFigures, AirUpkeepFigures, AircraftRole, AircraftType, AttackWeapon,
    BombardmentFigures, FatigueFigures, GroundFigures, PortSize, RefData, ReportError,
    SearchFigures, ShipClass, ShipKind, SurfaceCombatFigures,
};
use sim_core::state::ShipSystem;

use crate::error::DataError;
use crate::quantity::Quantity;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ShipClassRow {
    id: u32,
    name: String,
    kind: String,
    cruise_speed_kn: String,
    max_speed_kn: String,
    fuel_capacity_t: String,
    endurance_nm: String,
    cargo_capacity_t: String,
    main_guns: u32,
    gun_rounds_per_min: String,
    gun_rounds_per_gun: u32,
    gun_damage: String,
    torpedo_tubes: u32,
    /// Tubes that bear on one side (decisions 0074 and 0083). Required.
    torpedo_tubes_one_side: u32,
    torpedo_damage: String,
    toughness: String,
    /// 0 or 1.
    radar: u8,
    /// Optional column (decision 8 of `docs/decisions/0025-carrier-air-groups.md`):
    /// a file without it gives every class a capacity of 0. If the column is
    /// there, every row needs a whole number.
    #[serde(default)]
    aircraft_capacity: u32,
    /// Optional column (decision 0027), after `aircraft_capacity`: a file
    /// without it gives every class no anti-aircraft shots.
    #[serde(default)]
    anti_aircraft: u32,
    /// Optional column (decision 0066), after `anti_aircraft`: the men a
    /// ship of the class carries. A file without it gives every class 0. If
    /// the column is there, every row needs a whole number.
    #[serde(default)]
    troop_capacity_men: u32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PortSizeRow {
    port_size: i64,
    berths: u32,
    berth_rate_t_per_h: String,
    lighter_rate_t_per_h: String,
}

pub(crate) fn parse_kind(text: &str) -> Option<ShipKind> {
    Some(match text {
        "carrier" => ShipKind::Carrier,
        "battleship" => ShipKind::Battleship,
        "cruiser" => ShipKind::Cruiser,
        "destroyer" => ShipKind::Destroyer,
        "transport" => ShipKind::Transport,
        "auxiliary" => ShipKind::Auxiliary,
        _ => return None,
    })
}

/// Loads ship classes from a CSV file.
pub fn load_ship_classes(path: &Path) -> Result<RefData, DataError> {
    let text = fs::read_to_string(path).map_err(|e| DataError::in_file(path, e.to_string()))?;
    parse_ship_classes(&text, path)
}

/// Parses ship classes from CSV text. `path` is only used in messages.
fn parse_ship_classes(text: &str, path: &Path) -> Result<RefData, DataError> {
    let mut reader = csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .trim(csv::Trim::All)
        .from_reader(text.as_bytes());

    let mut ref_data = RefData::default();
    for row in reader.deserialize::<ShipClassRow>() {
        let row = row.map_err(|e| DataError::in_file(path, e.to_string()))?;
        let id = ShipClassId::new(row.id);
        let number = |field: &str, text: &str| -> Result<Fixed, DataError> {
            text.parse::<Fixed>().map_err(|e| {
                DataError::in_file(path, format!("{id}: {field} \"{text}\" is invalid: {e}"))
            })
        };
        let class = ShipClass {
            id,
            kind: parse_kind(&row.kind).ok_or_else(|| {
                DataError::in_file(path, format!("{id}: unknown kind \"{}\"", row.kind))
            })?,
            cruise_speed_kn: number("cruise_speed_kn", &row.cruise_speed_kn)?,
            max_speed_kn: number("max_speed_kn", &row.max_speed_kn)?,
            fuel_capacity_t: number("fuel_capacity_t", &row.fuel_capacity_t)?,
            endurance_nm: number("endurance_nm", &row.endurance_nm)?,
            cargo_capacity_t: number("cargo_capacity_t", &row.cargo_capacity_t)?,
            main_guns: row.main_guns,
            gun_rounds_per_min: number("gun_rounds_per_min", &row.gun_rounds_per_min)?,
            gun_rounds_per_gun: row.gun_rounds_per_gun,
            gun_damage: number("gun_damage", &row.gun_damage)?,
            torpedo_tubes: row.torpedo_tubes,
            torpedo_tubes_one_side: row.torpedo_tubes_one_side,
            torpedo_damage: number("torpedo_damage", &row.torpedo_damage)?,
            toughness: number("toughness", &row.toughness)?,
            radar: match row.radar {
                0 => false,
                1 => true,
                other => {
                    return Err(DataError::in_file(
                        path,
                        format!("{id}: radar is {other}, not 0 or 1"),
                    ))
                }
            },
            aircraft_capacity: row.aircraft_capacity,
            anti_aircraft: row.anti_aircraft,
            name: row.name,
            troop_capacity_men: row.troop_capacity_men,
        };
        if let Some(problem) = class.problem() {
            return Err(DataError::in_file(path, problem));
        }
        if ref_data.ship_classes.insert(id, class).is_some() {
            return Err(DataError::in_file(path, format!("{id} appears twice")));
        }
    }
    Ok(ref_data)
}

/// The port sizes the table must cover, inclusive.
const LARGEST_PORT_SIZE: u8 = 10;

/// Loads the port size table from a CSV file. Every size from 0 to 10 must
/// appear exactly once.
pub fn load_port_sizes(path: &Path) -> Result<BTreeMap<u8, PortSize>, DataError> {
    let text = fs::read_to_string(path).map_err(|e| DataError::in_file(path, e.to_string()))?;
    let mut reader = csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .trim(csv::Trim::All)
        .from_reader(text.as_bytes());

    let mut sizes = BTreeMap::new();
    for row in reader.deserialize::<PortSizeRow>() {
        let row = row.map_err(|e| DataError::in_file(path, e.to_string()))?;
        let size = u8::try_from(row.port_size)
            .ok()
            .filter(|size| *size <= LARGEST_PORT_SIZE)
            .ok_or_else(|| {
                DataError::in_file(
                    path,
                    format!(
                        "port size {} is outside 0 to {LARGEST_PORT_SIZE}",
                        row.port_size
                    ),
                )
            })?;
        let rate = |field: &str, text: &str| -> Result<Fixed, DataError> {
            text.parse::<Fixed>().map_err(|e| {
                DataError::in_file(
                    path,
                    format!("port size {size}: {field} \"{text}\" is invalid: {e}"),
                )
            })
        };
        let port = PortSize {
            berths: row.berths,
            berth_rate_t_per_h: rate("berth_rate_t_per_h", &row.berth_rate_t_per_h)?,
            lighter_rate_t_per_h: rate("lighter_rate_t_per_h", &row.lighter_rate_t_per_h)?,
        };
        if let Some(problem) = port.problem(size) {
            return Err(DataError::in_file(path, problem));
        }
        if sizes.insert(size, port).is_some() {
            return Err(DataError::in_file(
                path,
                format!("port size {size} appears twice"),
            ));
        }
    }
    for size in 0..=LARGEST_PORT_SIZE {
        if !sizes.contains_key(&size) {
            return Err(DataError::in_file(
                path,
                format!("port size {size} is missing"),
            ));
        }
    }
    Ok(sizes)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AircraftTypeRow {
    id: u32,
    name: String,
    role: String,
    search_range_nm: String,
    sweep_width_nm: String,
    strike_range_nm: String,
    attack_weapon: String,
    attack_damage: String,
    pilot_armour: String,
    self_sealing_tanks: String,
}

/// Loads aircraft types from a CSV file.
pub fn load_aircraft_types(
    path: &Path,
) -> Result<BTreeMap<AircraftTypeId, AircraftType>, DataError> {
    let text = fs::read_to_string(path).map_err(|e| DataError::in_file(path, e.to_string()))?;
    parse_aircraft_types(&text, path)
}

/// Parses aircraft types from CSV text. `path` is only used in messages.
fn parse_aircraft_types(
    text: &str,
    path: &Path,
) -> Result<BTreeMap<AircraftTypeId, AircraftType>, DataError> {
    let mut reader = csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .trim(csv::Trim::All)
        .from_reader(text.as_bytes());

    let mut types = BTreeMap::new();
    for row in reader.deserialize::<AircraftTypeRow>() {
        let row = row.map_err(|e| DataError::in_file(path, e.to_string()))?;
        let id = AircraftTypeId::new(row.id);
        let number = |field: &str, text: &str| -> Result<Fixed, DataError> {
            text.parse::<Fixed>().map_err(|e| {
                DataError::in_file(path, format!("{id}: {field} \"{text}\" is invalid: {e}"))
            })
        };
        let role = match row.role.as_str() {
            "patrol" => AircraftRole::Patrol,
            "bomber" => AircraftRole::Bomber,
            "fighter" => AircraftRole::Fighter,
            other => {
                return Err(DataError::in_file(
                    path,
                    format!("{id}: unknown role \"{other}\""),
                ))
            }
        };
        let attack_weapon = match row.attack_weapon.as_str() {
            "none" => AttackWeapon::None,
            "bomb" => AttackWeapon::Bomb,
            "torpedo" => AttackWeapon::Torpedo,
            other => {
                return Err(DataError::in_file(
                    path,
                    format!("{id}: unknown attack weapon \"{other}\""),
                ))
            }
        };
        let flag = |field: &str, text: &str| -> Result<bool, DataError> {
            match text {
                "0" => Ok(false),
                "1" => Ok(true),
                other => Err(DataError::in_file(
                    path,
                    format!("{id}: {field} \"{other}\" must be 0 or 1"),
                )),
            }
        };
        let aircraft_type = AircraftType {
            id,
            pilot_armour: flag("pilot_armour", &row.pilot_armour)?,
            self_sealing_tanks: flag("self_sealing_tanks", &row.self_sealing_tanks)?,
            role,
            strike_range_nm: number("strike_range_nm", &row.strike_range_nm)?,
            attack_weapon,
            attack_damage: number("attack_damage", &row.attack_damage)?,
            search_range_nm: number("search_range_nm", &row.search_range_nm)?,
            sweep_width_nm: number("sweep_width_nm", &row.sweep_width_nm)?,
            name: row.name,
        };
        if let Some(problem) = aircraft_type.problem() {
            return Err(DataError::in_file(path, problem));
        }
        if types.insert(id, aircraft_type).is_some() {
            return Err(DataError::in_file(path, format!("{id} appears twice")));
        }
    }
    Ok(types)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SearchFile {
    /// Required, and ignored once read: the scenario carries the flag.
    #[allow(dead_code)]
    placeholder: bool,
    contact_drop_age_pulses: u32,
    merge_distance_hexes: u32,
    fresh_contact_max_age_pulses: u32,
    position_error_chance: Quantity,
    count_over_chance: Quantity,
    count_under_chance: Quantity,
    /// True kind name, then reported kind name, then the chance.
    kind_confusion: BTreeMap<String, BTreeMap<String, Quantity>>,
}

/// Loads the search, report and contact figures from a TOML file.
pub fn load_search_figures(path: &Path) -> Result<SearchFigures, DataError> {
    let text = fs::read_to_string(path).map_err(|e| DataError::in_file(path, e.to_string()))?;
    parse_search_figures(&text, path)
}

/// Parses search figures from TOML text. `path` is only used in messages.
fn parse_search_figures(text: &str, path: &Path) -> Result<SearchFigures, DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    let file: SearchFile = toml::from_str(text).map_err(|e| fail(e.to_string()))?;
    let number = |field: &str, quantity: &Quantity| -> Result<Fixed, DataError> {
        quantity
            .to_fixed()
            .map_err(|e| fail(format!("{field} {e}")))
    };
    let kind = |name: &str| -> Result<ShipKind, DataError> {
        parse_kind(name).ok_or_else(|| fail(format!("kind_confusion: unknown kind \"{name}\"")))
    };

    let mut kind_confusion = BTreeMap::new();
    for (true_name, row) in &file.kind_confusion {
        let true_kind = kind(true_name)?;
        let mut reported_row = BTreeMap::new();
        for (reported_name, chance) in row {
            let reported = kind(reported_name)?;
            let field = format!("kind_confusion.{true_name}.{reported_name}");
            reported_row.insert(reported, number(&field, chance)?);
        }
        kind_confusion.insert(true_kind, reported_row);
    }

    let figures = SearchFigures {
        contact_drop_age_pulses: file.contact_drop_age_pulses,
        merge_distance_hexes: file.merge_distance_hexes,
        fresh_contact_max_age_pulses: file.fresh_contact_max_age_pulses,
        position_error_chance: number("position_error_chance", &file.position_error_chance)?,
        count_over_chance: number("count_over_chance", &file.count_over_chance)?,
        count_under_chance: number("count_under_chance", &file.count_under_chance)?,
        kind_confusion,
    };
    if let Some(problem) = figures.problem() {
        return Err(fail(problem));
    }
    Ok(figures)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SurfaceCombatFile {
    /// Required, and ignored once read: the scenario carries the flag.
    #[allow(dead_code)]
    placeholder: bool,
    round_minutes: u32,
    max_rounds: u32,
    sighting_chance_night: Quantity,
    sighting_chance_day: Quantity,
    radar_sighting_bonus: Quantity,
    not_alert_sighting_factor: Quantity,
    alert_contact_distance_hexes: u32,
    alarm_chance: Quantity,
    gun_hit_chance_night: Quantity,
    gun_hit_chance_day: Quantity,
    torpedo_hit_chance_night: Quantity,
    torpedo_hit_chance_day: Quantity,
    surprised_target_factor: Quantity,
    lit_target_factor: Quantity,
    gun_hit_flotation: Quantity,
    system_hit_chance_per_point: Quantity,
    degraded_factor: Quantity,
    fire_control_out_factor: Quantity,
    crippled_flotation: Quantity,
    claimed_sunk_fire: Quantity,
    /// The strength of experience in a dark fight, from 0 to 0.5 (decision
    /// 0064). Required like every other key.
    experience_gun_strength: Quantity,
    experience_torpedo_strength: Quantity,
    experience_sighting_strength: Quantity,
    /// The share of its guns a force not ready fires, from 0.1 to 1 (decision
    /// 0068). Required like every other key.
    unready_gun_share: Quantity,
    /// The share of its torpedoes a force not ready fires, from 0.1 to 1
    /// (decision 0071). Required like every other key.
    unready_torpedo_share: Quantity,
    /// Repair (decision 0069). Required like every other key.
    forward_repair_max_flotation: Quantity,
    forward_repair_system_points: Quantity,
    yard_days_per_flotation_point: Quantity,
    yard_days_per_system_degraded: Quantity,
    yard_days_per_system_out: Quantity,
    /// Shore bombardment (decision 0092). All four or none: a file with none
    /// loads as before and no force bombards.
    bombardment_minutes: Option<u32>,
    bombardment_hit_chance: Option<Quantity>,
    bombardment_reference_gun_damage: Option<Quantity>,
    bombardment_false_claim_chance: Option<Quantity>,
    gun_system_hit: SystemTable,
    torpedo_system_hit: SystemTable,
    straggler_escorts: EscortTable,
}

/// A table of shares by system. A system left out has share 0.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SystemTable {
    propulsion: Option<Quantity>,
    fire_control: Option<Quantity>,
    main_battery: Option<Quantity>,
    torpedo_mounts: Option<Quantity>,
    flight_deck: Option<Quantity>,
}

impl SystemTable {
    fn to_map(
        &self,
        table: &str,
        number: &dyn Fn(&str, &Quantity) -> Result<Fixed, DataError>,
    ) -> Result<BTreeMap<ShipSystem, Fixed>, DataError> {
        let entries = [
            (ShipSystem::Propulsion, &self.propulsion),
            (ShipSystem::FireControl, &self.fire_control),
            (ShipSystem::MainBattery, &self.main_battery),
            (ShipSystem::TorpedoMounts, &self.torpedo_mounts),
            (ShipSystem::FlightDeck, &self.flight_deck),
        ];
        let mut map = BTreeMap::new();
        for (system, share) in entries {
            if let Some(share) = share {
                let label = format!("{table} {}", system.name());
                map.insert(system, number(&label, share)?);
            }
        }
        Ok(map)
    }
}

/// Escorts by ship kind. Every kind is required.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EscortTable {
    carrier: u32,
    battleship: u32,
    cruiser: u32,
    destroyer: u32,
    transport: u32,
    auxiliary: u32,
}

/// Loads the surface combat figures from a TOML file.
pub fn load_surface_combat_figures(path: &Path) -> Result<SurfaceCombatFigures, DataError> {
    let text = fs::read_to_string(path).map_err(|e| DataError::in_file(path, e.to_string()))?;
    parse_surface_combat_figures(&text, path)
}

/// Parses surface combat figures from TOML text. `path` is only used in
/// messages.
fn parse_surface_combat_figures(
    text: &str,
    path: &Path,
) -> Result<SurfaceCombatFigures, DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    let file: SurfaceCombatFile = toml::from_str(text).map_err(|e| fail(e.to_string()))?;
    let number = |field: &str, quantity: &Quantity| -> Result<Fixed, DataError> {
        quantity
            .to_fixed()
            .map_err(|e| fail(format!("{field} {e}")))
    };
    let figures = SurfaceCombatFigures {
        round_minutes: file.round_minutes,
        max_rounds: file.max_rounds,
        sighting_chance_night: number("sighting_chance_night", &file.sighting_chance_night)?,
        sighting_chance_day: number("sighting_chance_day", &file.sighting_chance_day)?,
        radar_sighting_bonus: number("radar_sighting_bonus", &file.radar_sighting_bonus)?,
        not_alert_sighting_factor: number(
            "not_alert_sighting_factor",
            &file.not_alert_sighting_factor,
        )?,
        alert_contact_distance_hexes: file.alert_contact_distance_hexes,
        alarm_chance: number("alarm_chance", &file.alarm_chance)?,
        gun_hit_chance_night: number("gun_hit_chance_night", &file.gun_hit_chance_night)?,
        gun_hit_chance_day: number("gun_hit_chance_day", &file.gun_hit_chance_day)?,
        torpedo_hit_chance_night: number(
            "torpedo_hit_chance_night",
            &file.torpedo_hit_chance_night,
        )?,
        torpedo_hit_chance_day: number("torpedo_hit_chance_day", &file.torpedo_hit_chance_day)?,
        surprised_target_factor: number("surprised_target_factor", &file.surprised_target_factor)?,
        lit_target_factor: number("lit_target_factor", &file.lit_target_factor)?,
        gun_hit_flotation: number("gun_hit_flotation", &file.gun_hit_flotation)?,
        system_hit_chance_per_point: number(
            "system_hit_chance_per_point",
            &file.system_hit_chance_per_point,
        )?,
        degraded_factor: number("degraded_factor", &file.degraded_factor)?,
        fire_control_out_factor: number("fire_control_out_factor", &file.fire_control_out_factor)?,
        gun_system_hit: file.gun_system_hit.to_map("gun_system_hit", &number)?,
        torpedo_system_hit: file
            .torpedo_system_hit
            .to_map("torpedo_system_hit", &number)?,
        straggler_escorts: [
            (ShipKind::Carrier, file.straggler_escorts.carrier),
            (ShipKind::Battleship, file.straggler_escorts.battleship),
            (ShipKind::Cruiser, file.straggler_escorts.cruiser),
            (ShipKind::Destroyer, file.straggler_escorts.destroyer),
            (ShipKind::Transport, file.straggler_escorts.transport),
            (ShipKind::Auxiliary, file.straggler_escorts.auxiliary),
        ]
        .into_iter()
        .collect(),
        crippled_flotation: number("crippled_flotation", &file.crippled_flotation)?,
        claimed_sunk_fire: number("claimed_sunk_fire", &file.claimed_sunk_fire)?,
        experience_gun_strength: number("experience_gun_strength", &file.experience_gun_strength)?,
        experience_torpedo_strength: number(
            "experience_torpedo_strength",
            &file.experience_torpedo_strength,
        )?,
        experience_sighting_strength: number(
            "experience_sighting_strength",
            &file.experience_sighting_strength,
        )?,
        unready_gun_share: number("unready_gun_share", &file.unready_gun_share)?,
        unready_torpedo_share: number("unready_torpedo_share", &file.unready_torpedo_share)?,
        forward_repair_max_flotation: number(
            "forward_repair_max_flotation",
            &file.forward_repair_max_flotation,
        )?,
        forward_repair_system_points: number(
            "forward_repair_system_points",
            &file.forward_repair_system_points,
        )?,
        yard_days_per_flotation_point: number(
            "yard_days_per_flotation_point",
            &file.yard_days_per_flotation_point,
        )?,
        yard_days_per_system_degraded: number(
            "yard_days_per_system_degraded",
            &file.yard_days_per_system_degraded,
        )?,
        yard_days_per_system_out: number(
            "yard_days_per_system_out",
            &file.yard_days_per_system_out,
        )?,
        bombardment: match (
            file.bombardment_minutes,
            &file.bombardment_hit_chance,
            &file.bombardment_reference_gun_damage,
            &file.bombardment_false_claim_chance,
        ) {
            (None, None, None, None) => None,
            (Some(minutes), Some(hit), Some(reference), Some(false_claim)) => {
                Some(BombardmentFigures {
                    minutes,
                    hit_chance: number("bombardment_hit_chance", hit)?,
                    reference_gun_damage: number("bombardment_reference_gun_damage", reference)?,
                    false_claim_chance: number("bombardment_false_claim_chance", false_claim)?,
                })
            }
            _ => {
                return Err(fail(
                    "the bombardment figures are bombardment_minutes, bombardment_hit_chance, bombardment_reference_gun_damage and bombardment_false_claim_chance: give all four or none".to_string(),
                ))
            }
        },
    };
    if let Some(problem) = figures.problem() {
        return Err(fail(problem));
    }
    Ok(figures)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AirCombatFile {
    /// Required, and ignored once read: the scenario carries the flag.
    #[allow(dead_code)]
    placeholder: bool,
    strike_contact_max_age_pulses: u32,
    bomb_hit_chance: Quantity,
    torpedo_hit_chance: Quantity,
    fighter_hit_chance: Quantity,
    anti_aircraft_hit_chance: Quantity,
    pilot_armour_factor: Quantity,
    self_sealing_tanks_factor: Quantity,
    bomb_system_hit: SystemTable,
    base_supply_destroyed_per_hit_t: Quantity,
    base_hit: BaseHitTable,
    fighter_false_claim_chance: Quantity,
    anti_aircraft_false_claim_chance: Quantity,
    bomb_false_claim_chance: Quantity,
    /// Reach beyond the radius and running out of fuel (decision 0059).
    /// Required like every other key of the file: a file without them is
    /// refused, naming the key.
    extended_reach_share: Quantity,
    ditch_chance_per_radius_beyond: Quantity,
    crew_recovered_short_of_home: Quantity,
    /// Decision 6a of 0059.
    crew_recovered_over_own_base: Quantity,
    crew_recovered_beside_own_force: Quantity,
    /// Decision 9 of 0059.
    crew_recovered_open_sea: Quantity,
    /// Decision 0079.
    anti_aircraft_before_attack_share: Quantity,
    /// Decision 0080.
    patrol_drawn_off_share: Quantity,
    /// Decision 0087.
    patrol_before_attack_share: Quantity,
}

/// What a bomb hit on a base lands on (decision 7 of 0034). The rest of the
/// hits land on nothing.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BaseHitTable {
    aircraft: Quantity,
    supply: Quantity,
    /// The share for men ashore (decision 0092, 8). Without it the share is
    /// 0 and those hits land on nothing, as before.
    men: Option<Quantity>,
    /// The share for the airfield (decision 0103, 2). Without it the share
    /// is 0 and those hits land as before.
    airfield: Option<Quantity>,
}

/// Loads the air combat figures from a TOML file.
pub fn load_air_combat_figures(path: &Path) -> Result<AirCombatFigures, DataError> {
    let text = fs::read_to_string(path).map_err(|e| DataError::in_file(path, e.to_string()))?;
    parse_air_combat_figures(&text, path)
}

/// Parses air combat figures from TOML text. `path` is only used in
/// messages.
fn parse_air_combat_figures(text: &str, path: &Path) -> Result<AirCombatFigures, DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    let file: AirCombatFile = toml::from_str(text).map_err(|e| fail(e.to_string()))?;
    let number = |field: &str, quantity: &Quantity| -> Result<Fixed, DataError> {
        quantity
            .to_fixed()
            .map_err(|e| fail(format!("{field} {e}")))
    };
    let figures = AirCombatFigures {
        strike_contact_max_age_pulses: file.strike_contact_max_age_pulses,
        bomb_hit_chance: number("bomb_hit_chance", &file.bomb_hit_chance)?,
        torpedo_hit_chance: number("torpedo_hit_chance", &file.torpedo_hit_chance)?,
        fighter_hit_chance: number("fighter_hit_chance", &file.fighter_hit_chance)?,
        anti_aircraft_hit_chance: number(
            "anti_aircraft_hit_chance",
            &file.anti_aircraft_hit_chance,
        )?,
        pilot_armour_factor: number("pilot_armour_factor", &file.pilot_armour_factor)?,
        self_sealing_tanks_factor: number(
            "self_sealing_tanks_factor",
            &file.self_sealing_tanks_factor,
        )?,
        bomb_system_hit: file.bomb_system_hit.to_map("bomb_system_hit", &number)?,
        base_supply_destroyed_per_hit_t: number(
            "base_supply_destroyed_per_hit_t",
            &file.base_supply_destroyed_per_hit_t,
        )?,
        base_hit_aircraft: number("base_hit.aircraft", &file.base_hit.aircraft)?,
        base_hit_supply: number("base_hit.supply", &file.base_hit.supply)?,
        base_hit_men: match &file.base_hit.men {
            Some(men) => number("base_hit.men", men)?,
            None => Fixed::ZERO,
        },
        base_hit_airfield: match &file.base_hit.airfield {
            Some(airfield) => number("base_hit.airfield", airfield)?,
            None => Fixed::ZERO,
        },
        fighter_false_claim_chance: number(
            "fighter_false_claim_chance",
            &file.fighter_false_claim_chance,
        )?,
        anti_aircraft_false_claim_chance: number(
            "anti_aircraft_false_claim_chance",
            &file.anti_aircraft_false_claim_chance,
        )?,
        bomb_false_claim_chance: number("bomb_false_claim_chance", &file.bomb_false_claim_chance)?,
        extended_reach_share: number("extended_reach_share", &file.extended_reach_share)?,
        ditch_chance_per_radius_beyond: number(
            "ditch_chance_per_radius_beyond",
            &file.ditch_chance_per_radius_beyond,
        )?,
        crew_recovered_short_of_home: number(
            "crew_recovered_short_of_home",
            &file.crew_recovered_short_of_home,
        )?,
        crew_recovered_over_own_base: number(
            "crew_recovered_over_own_base",
            &file.crew_recovered_over_own_base,
        )?,
        crew_recovered_beside_own_force: number(
            "crew_recovered_beside_own_force",
            &file.crew_recovered_beside_own_force,
        )?,
        crew_recovered_open_sea: number("crew_recovered_open_sea", &file.crew_recovered_open_sea)?,
        anti_aircraft_before_attack_share: number(
            "anti_aircraft_before_attack_share",
            &file.anti_aircraft_before_attack_share,
        )?,
        patrol_drawn_off_share: number("patrol_drawn_off_share", &file.patrol_drawn_off_share)?,
        patrol_before_attack_share: number(
            "patrol_before_attack_share",
            &file.patrol_before_attack_share,
        )?,
    };
    if let Some(problem) = figures.problem() {
        return Err(fail(problem));
    }
    Ok(figures)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AirUpkeepFile {
    /// Required, and ignored once read: the scenario carries the flag.
    #[allow(dead_code)]
    placeholder: bool,
    wear_base_chance: Quantity,
    wear_chance_per_100_nm: Quantity,
    write_off_share: Quantity,
    carrier_repairs_per_day: u32,
}

/// Loads the wear and repair figures from a TOML file (decision 0035).
pub fn load_air_upkeep_figures(path: &Path) -> Result<AirUpkeepFigures, DataError> {
    let text = fs::read_to_string(path).map_err(|e| DataError::in_file(path, e.to_string()))?;
    parse_air_upkeep_figures(&text, path)
}

/// Parses wear and repair figures from TOML text. `path` is only used in
/// messages.
fn parse_air_upkeep_figures(text: &str, path: &Path) -> Result<AirUpkeepFigures, DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    let file: AirUpkeepFile = toml::from_str(text).map_err(|e| fail(e.to_string()))?;
    let number = |field: &str, quantity: &Quantity| -> Result<Fixed, DataError> {
        quantity
            .to_fixed()
            .map_err(|e| fail(format!("{field} {e}")))
    };
    let figures = AirUpkeepFigures {
        wear_base_chance: number("wear_base_chance", &file.wear_base_chance)?,
        wear_chance_per_100_nm: number("wear_chance_per_100_nm", &file.wear_chance_per_100_nm)?,
        write_off_share: number("write_off_share", &file.write_off_share)?,
        carrier_repairs_per_day: file.carrier_repairs_per_day,
    };
    if let Some(problem) = figures.problem() {
        return Err(fail(problem));
    }
    Ok(figures)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GroundFile {
    /// Required, and ignored once read: the scenario carries the flag.
    #[allow(dead_code)]
    placeholder: bool,
    handling_t_per_man: Quantity,
    daily_supply_t_per_man: Quantity,
    unfed_loss_share_per_day: Quantity,
    // The fight ashore and a base that changes side (decision 0090).
    men_per_shot: Quantity,
    hit_chance: Quantity,
    defender_hit_chance: Quantity,
    men_per_hit: Quantity,
    fatigue_per_attack_day: Quantity,
    rest_per_day: Quantity,
    attack_fatigue_limit: Quantity,
    beaten_share: Quantity,
    captured_stock_share: Quantity,
    /// Decision 0092. Without `men_lost_per_base_hit` a hit on men removes
    /// nobody; without both report errors no ground report is made.
    men_lost_per_base_hit: Option<Quantity>,
    report_error_low: Option<Quantity>,
    report_error_high: Option<Quantity>,
    /// Marching overland (decision 0094). Required like the older keys.
    /// Days is a whole number, read as one.
    march_days_per_hex: u32,
    fatigue_per_march_day: Quantity,
    march_supply_days: Quantity,
    /// Preparing a landing (decision 0095) and the experience of a land unit
    /// (decision 0097). Required like the older keys. The three counts are
    /// whole numbers, read as such.
    preparation_per_day: Quantity,
    unprepared_landing_share: Quantity,
    preparation_kept_on_change: Quantity,
    unprepared_landing_fatigue: Quantity,
    experience_fire_strength: Quantity,
    training_days_per_level: u32,
    training_ceiling: u32,
    fight_days_per_level: u32,
    /// The strengths of a land unit's skills (decision 0098). Required like
    /// the older keys.
    assault_skill_strength: Quantity,
    movement_skill_strength: Quantity,
    holding_skill_strength: Quantity,
    weight_strength: Quantity,
    /// The figures of work and what it builds (decision 0099). Required like
    /// the older keys.
    work_per_man_day: Quantity,
    airfield_work: Quantity,
    work_to_double_unloading: Quantity,
    work_per_aircraft_repair: Quantity,
    work_per_defence_point: Quantity,
    defences_strength: Quantity,
    fighting_unit_work_share: Quantity,
    /// The figures of an airfield damaged and mended (decision 0103).
    /// Required like the older keys.
    airfield_damage_per_hit: Quantity,
    work_per_airfield_damage_point: Quantity,
    beaten_fatigue: Quantity,
    airfield_landing_refused_at: Quantity,
}

/// Loads the ground figures from a TOML file (decision 0066).
pub fn load_ground_figures(path: &Path) -> Result<GroundFigures, DataError> {
    let text = fs::read_to_string(path).map_err(|e| DataError::in_file(path, e.to_string()))?;
    parse_ground_figures(&text, path)
}

/// Parses ground figures from TOML text. `path` is only used in messages.
fn parse_ground_figures(text: &str, path: &Path) -> Result<GroundFigures, DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    let file: GroundFile = toml::from_str(text).map_err(|e| fail(e.to_string()))?;
    let number = |field: &str, quantity: &Quantity| -> Result<Fixed, DataError> {
        quantity
            .to_fixed()
            .map_err(|e| fail(format!("{field} {e}")))
    };
    let figures = GroundFigures {
        handling_t_per_man: number("handling_t_per_man", &file.handling_t_per_man)?,
        daily_supply_t_per_man: number("daily_supply_t_per_man", &file.daily_supply_t_per_man)?,
        unfed_loss_share_per_day: number(
            "unfed_loss_share_per_day",
            &file.unfed_loss_share_per_day,
        )?,
        men_per_shot: number("men_per_shot", &file.men_per_shot)?,
        hit_chance: number("hit_chance", &file.hit_chance)?,
        defender_hit_chance: number("defender_hit_chance", &file.defender_hit_chance)?,
        men_per_hit: number("men_per_hit", &file.men_per_hit)?,
        fatigue_per_attack_day: number("fatigue_per_attack_day", &file.fatigue_per_attack_day)?,
        rest_per_day: number("rest_per_day", &file.rest_per_day)?,
        attack_fatigue_limit: number("attack_fatigue_limit", &file.attack_fatigue_limit)?,
        beaten_share: number("beaten_share", &file.beaten_share)?,
        captured_stock_share: number("captured_stock_share", &file.captured_stock_share)?,
        men_lost_per_base_hit: match &file.men_lost_per_base_hit {
            Some(men) => number("men_lost_per_base_hit", men)?,
            None => Fixed::ZERO,
        },
        report_error: match (&file.report_error_low, &file.report_error_high) {
            (None, None) => None,
            (Some(low), Some(high)) => Some(ReportError {
                low: number("report_error_low", low)?,
                high: number("report_error_high", high)?,
            }),
            _ => {
                return Err(fail(
                    "give report_error_low and report_error_high together, or neither".to_string(),
                ))
            }
        },
        march_days_per_hex: file.march_days_per_hex,
        fatigue_per_march_day: number("fatigue_per_march_day", &file.fatigue_per_march_day)?,
        march_supply_days: number("march_supply_days", &file.march_supply_days)?,
        preparation_per_day: number("preparation_per_day", &file.preparation_per_day)?,
        unprepared_landing_share: number(
            "unprepared_landing_share",
            &file.unprepared_landing_share,
        )?,
        preparation_kept_on_change: number(
            "preparation_kept_on_change",
            &file.preparation_kept_on_change,
        )?,
        unprepared_landing_fatigue: number(
            "unprepared_landing_fatigue",
            &file.unprepared_landing_fatigue,
        )?,
        experience_fire_strength: number(
            "experience_fire_strength",
            &file.experience_fire_strength,
        )?,
        training_days_per_level: file.training_days_per_level,
        training_ceiling: file.training_ceiling,
        fight_days_per_level: file.fight_days_per_level,
        assault_skill_strength: number("assault_skill_strength", &file.assault_skill_strength)?,
        movement_skill_strength: number("movement_skill_strength", &file.movement_skill_strength)?,
        holding_skill_strength: number("holding_skill_strength", &file.holding_skill_strength)?,
        weight_strength: number("weight_strength", &file.weight_strength)?,
        work_per_man_day: number("work_per_man_day", &file.work_per_man_day)?,
        airfield_work: number("airfield_work", &file.airfield_work)?,
        work_to_double_unloading: number(
            "work_to_double_unloading",
            &file.work_to_double_unloading,
        )?,
        work_per_aircraft_repair: number(
            "work_per_aircraft_repair",
            &file.work_per_aircraft_repair,
        )?,
        work_per_defence_point: number("work_per_defence_point", &file.work_per_defence_point)?,
        defences_strength: number("defences_strength", &file.defences_strength)?,
        fighting_unit_work_share: number(
            "fighting_unit_work_share",
            &file.fighting_unit_work_share,
        )?,
        airfield_damage_per_hit: number("airfield_damage_per_hit", &file.airfield_damage_per_hit)?,
        beaten_fatigue: number("beaten_fatigue", &file.beaten_fatigue)?,
        work_per_airfield_damage_point: number(
            "work_per_airfield_damage_point",
            &file.work_per_airfield_damage_point,
        )?,
        airfield_landing_refused_at: number(
            "airfield_landing_refused_at",
            &file.airfield_landing_refused_at,
        )?,
    };
    if let Some(problem) = figures.problem() {
        return Err(fail(problem));
    }
    Ok(figures)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AirSupplyFile {
    /// Required, and ignored once read: the scenario carries the flag.
    #[allow(dead_code)]
    placeholder: bool,
    sortie_supply_t: Quantity,
    sortie_supply_t_per_100_nm: Quantity,
    daily_supply_t_per_aircraft: Quantity,
    repair_supply_t: Quantity,
}

/// Loads the base supply figures from a TOML file (decision 0039).
pub fn load_air_supply_figures(path: &Path) -> Result<AirSupplyFigures, DataError> {
    let text = fs::read_to_string(path).map_err(|e| DataError::in_file(path, e.to_string()))?;
    parse_air_supply_figures(&text, path)
}

/// Parses base supply figures from TOML text. `path` is only used in
/// messages.
fn parse_air_supply_figures(text: &str, path: &Path) -> Result<AirSupplyFigures, DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    let file: AirSupplyFile = toml::from_str(text).map_err(|e| fail(e.to_string()))?;
    let number = |field: &str, quantity: &Quantity| -> Result<Fixed, DataError> {
        quantity
            .to_fixed()
            .map_err(|e| fail(format!("{field} {e}")))
    };
    let figures = AirSupplyFigures {
        sortie_supply_t: number("sortie_supply_t", &file.sortie_supply_t)?,
        sortie_supply_t_per_100_nm: number(
            "sortie_supply_t_per_100_nm",
            &file.sortie_supply_t_per_100_nm,
        )?,
        daily_supply_t_per_aircraft: number(
            "daily_supply_t_per_aircraft",
            &file.daily_supply_t_per_aircraft,
        )?,
        repair_supply_t: number("repair_supply_t", &file.repair_supply_t)?,
    };
    if let Some(problem) = figures.problem() {
        return Err(fail(problem));
    }
    Ok(figures)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FatigueFile {
    /// Required, and ignored once read: the scenario carries the flag.
    #[allow(dead_code)]
    placeholder: bool,
    air_fatigue_per_sortie: Quantity,
    air_fatigue_per_100_nm: Quantity,
    air_rest_per_pulse: Quantity,
    force_fatigue_per_pulse_alert: Quantity,
    force_rest_per_pulse: Quantity,
}

/// Loads the fatigue figures from a TOML file (decision 0036).
pub fn load_fatigue_figures(path: &Path) -> Result<FatigueFigures, DataError> {
    let text = fs::read_to_string(path).map_err(|e| DataError::in_file(path, e.to_string()))?;
    parse_fatigue_figures(&text, path)
}

/// Parses fatigue figures from TOML text. `path` is only used in messages.
fn parse_fatigue_figures(text: &str, path: &Path) -> Result<FatigueFigures, DataError> {
    let fail = |message: String| DataError::in_file(path, message);
    let file: FatigueFile = toml::from_str(text).map_err(|e| fail(e.to_string()))?;
    let number = |field: &str, quantity: &Quantity| -> Result<Fixed, DataError> {
        quantity
            .to_fixed()
            .map_err(|e| fail(format!("{field} {e}")))
    };
    let figures = FatigueFigures {
        air_fatigue_per_sortie: number("air_fatigue_per_sortie", &file.air_fatigue_per_sortie)?,
        air_fatigue_per_100_nm: number("air_fatigue_per_100_nm", &file.air_fatigue_per_100_nm)?,
        air_rest_per_pulse: number("air_rest_per_pulse", &file.air_rest_per_pulse)?,
        force_fatigue_per_pulse_alert: number(
            "force_fatigue_per_pulse_alert",
            &file.force_fatigue_per_pulse_alert,
        )?,
        force_rest_per_pulse: number("force_rest_per_pulse", &file.force_rest_per_pulse)?,
    };
    if let Some(problem) = figures.problem() {
        return Err(fail(problem));
    }
    Ok(figures)
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEADER: &str = "port_size,berths,berth_rate_t_per_h,lighter_rate_t_per_h\n";

    /// Rows for every size 0 to 10, each with one berth.
    fn full_rows() -> String {
        (0..=10).map(|size| format!("{size},1,50,20\n")).collect()
    }

    /// Writes `text` to a file of its own and loads it as a port table.
    fn load(name: &str, text: &str) -> Result<BTreeMap<u8, PortSize>, DataError> {
        let path = std::env::temp_dir().join(format!(
            "flagplot_port_sizes_{}_{name}.csv",
            std::process::id()
        ));
        fs::write(&path, text).expect("temp file is writable");
        let result = load_port_sizes(&path);
        let _ = fs::remove_file(&path);
        result
    }

    fn message(result: Result<BTreeMap<u8, PortSize>, DataError>) -> String {
        result.expect_err("the table is rejected").message
    }

    fn here() -> &'static Path {
        Path::new("test.file")
    }

    const AIRCRAFT_HEADER: &str =
        "id,name,role,search_range_nm,sweep_width_nm,strike_range_nm,attack_weapon,attack_damage,pilot_armour,self_sealing_tanks\n";

    #[test]
    fn aircraft_types_parse() {
        let types = parse_aircraft_types(
            &format!("# c\n{AIRCRAFT_HEADER}1, Flying boat ,patrol,600,\"40.5\",0,none,0,0,0\n"),
            here(),
        )
        .expect("parses");
        let one = &types[&AircraftTypeId::new(1)];
        assert_eq!(one.name, "Flying boat");
        assert_eq!(one.role, AircraftRole::Patrol);
        assert_eq!(one.search_range_nm, Fixed::from_int(600));
        assert_eq!(one.sweep_width_nm, "40.5".parse::<Fixed>().expect("fixed"));
        assert_eq!(one.strike_range_nm, Fixed::ZERO);
        assert_eq!(one.attack_weapon, AttackWeapon::None);
        assert_eq!(one.attack_damage, Fixed::ZERO);
        assert!(!one.pilot_armour && !one.self_sealing_tanks);
    }

    #[test]
    fn the_protection_columns_are_read_and_required() {
        let types = parse_aircraft_types(
            &format!(
                "{AIRCRAFT_HEADER}1,A,fighter,0,0,0,none,0,1,0\n2,B,fighter,0,0,0,none,0,0,1\n"
            ),
            here(),
        )
        .expect("parses");
        let a = &types[&AircraftTypeId::new(1)];
        assert!(a.pilot_armour && !a.self_sealing_tanks);
        let b = &types[&AircraftTypeId::new(2)];
        assert!(!b.pilot_armour && b.self_sealing_tanks);
        // A file without the columns is rejected, each one named.
        for header in [
            "id,name,role,search_range_nm,sweep_width_nm,strike_range_nm,attack_weapon,attack_damage\n1,A,fighter,0,0,0,none,0\n",
            "id,name,role,search_range_nm,sweep_width_nm,strike_range_nm,attack_weapon,attack_damage,pilot_armour\n1,A,fighter,0,0,0,none,0,0\n",
        ] {
            let error = parse_aircraft_types(header, here()).expect_err("rejected");
            assert!(
                error.message.contains("pilot_armour")
                    || error.message.contains("self_sealing_tanks"),
                "{error}"
            );
        }
    }

    #[test]
    fn a_protection_value_other_than_0_or_1_names_the_row() {
        for (row, field) in [
            ("1,A,fighter,0,0,0,none,0,2,0\n", "pilot_armour"),
            ("1,A,fighter,0,0,0,none,0,0,yes\n", "self_sealing_tanks"),
            ("1,A,fighter,0,0,0,none,0,,0\n", "pilot_armour"),
        ] {
            let error = parse_aircraft_types(&format!("{AIRCRAFT_HEADER}{row}"), here())
                .expect_err("rejected");
            assert!(error.message.contains(field), "{error}");
            assert!(error.message.contains("0 or 1"), "{error}");
            assert!(
                error.message.contains(&AircraftTypeId::new(1).to_string()),
                "{error}"
            );
        }
    }

    #[test]
    fn the_strike_columns_parse_for_a_bomber_and_a_torpedo_bomber() {
        let types = parse_aircraft_types(
            &format!(
                "{AIRCRAFT_HEADER}1,Dive bomber,bomber,0,0,250,bomb,60,0,0\n2,Torpedo bomber,bomber,0,0,\"200.5\",torpedo,\"120.5\",1,1\n"
            ),
            here(),
        )
        .expect("parses");
        let dive = &types[&AircraftTypeId::new(1)];
        assert_eq!(dive.role, AircraftRole::Bomber);
        assert_eq!(dive.strike_range_nm, Fixed::from_int(250));
        assert_eq!(dive.attack_weapon, AttackWeapon::Bomb);
        assert_eq!(dive.attack_damage, Fixed::from_int(60));
        let torpedo = &types[&AircraftTypeId::new(2)];
        assert_eq!(torpedo.attack_weapon, AttackWeapon::Torpedo);
        assert_eq!(
            torpedo.strike_range_nm,
            "200.5".parse::<Fixed>().expect("fixed")
        );
        assert_eq!(
            torpedo.attack_damage,
            "120.5".parse::<Fixed>().expect("fixed")
        );
    }

    #[test]
    fn the_strike_columns_are_required() {
        let old = parse_aircraft_types(
            "id,name,role,search_range_nm,sweep_width_nm\n1,A,patrol,600,40\n",
            here(),
        )
        .expect_err("rejected");
        assert!(old.message.contains("strike_range_nm"), "{old}");
    }

    #[test]
    fn aircraft_type_errors_are_rejected() {
        let bad = |rows: &str| {
            parse_aircraft_types(&format!("{AIRCRAFT_HEADER}{rows}"), here())
                .expect_err("rejected")
                .message
        };
        assert!(bad("1,A,interceptor,600,40,0,none,0,0,0\n").contains("unknown role"));
        assert!(
            bad("1,A,patrol,600,40,0,none,0,0,0\n1,B,patrol,600,40,0,none,0,0,0\n")
                .contains("appears twice")
        );
        assert!(bad("1,A,patrol,0,40,0,none,0,0,0\n").contains("search range"));
        assert!(bad("1,A,patrol,600,x,0,none,0,0,0\n").contains("sweep_width_nm"));
        assert!(bad("1,A,bomber,0,0,250,cannon,60,0,0\n").contains("unknown attack weapon"));
        assert!(bad("1,A,bomber,0,0,x,bomb,60,0,0\n").contains("strike_range_nm"));
        assert!(bad("1,A,bomber,0,0,250,bomb,y,0,0\n").contains("attack_damage"));
        // A weapon needs a range and damage to go with it.
        assert!(bad("1,A,bomber,0,0,0,bomb,60,0,0\n").contains("strike range"));
        assert!(bad("1,A,bomber,0,0,250,torpedo,0,0,0\n").contains("attack damage"));
        assert!(bad("1,A,bomber,0,0,-1,none,0,0,0\n").contains("negative strike range"));
        let extra = parse_aircraft_types(
            "id,name,role,search_range_nm,sweep_width_nm,strike_range_nm,attack_weapon,attack_damage,pilot_armour,self_sealing_tanks,extra\n1,A,patrol,600,40,0,none,0,0,0,9\n",
            here(),
        )
        .expect_err("rejected");
        assert!(extra.message.contains("extra"), "{extra}");
    }

    fn air_combat_text() -> String {
        let path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/reference/air_combat.toml");
        fs::read_to_string(path).expect("the shipped file reads")
    }

    #[test]
    fn air_combat_figures_parse_with_the_files_values() {
        let figures = parse_air_combat_figures(&air_combat_text(), here()).expect("parses");
        assert_eq!(figures.strike_contact_max_age_pulses, 0);
        assert_eq!(figures.bomb_hit_chance, Fixed::from_ratio(2, 10));
        assert_eq!(figures.torpedo_hit_chance, Fixed::from_ratio(15, 100));
        assert_eq!(figures.fighter_hit_chance, Fixed::from_ratio(3, 10));
        assert_eq!(figures.anti_aircraft_hit_chance, Fixed::from_ratio(5, 100));
        assert_eq!(figures.pilot_armour_factor, Fixed::from_ratio(75, 100));
        assert_eq!(
            figures.self_sealing_tanks_factor,
            Fixed::from_ratio(75, 100)
        );
        assert_eq!(
            figures.bomb_system_hit[&ShipSystem::FlightDeck],
            Fixed::from_ratio(5, 10)
        );
        assert_eq!(
            figures.bomb_system_hit[&ShipSystem::MainBattery],
            Fixed::from_ratio(2, 10)
        );
        assert_eq!(figures.bomb_system_hit.len(), 4);
        assert_eq!(figures.base_supply_destroyed_per_hit_t, Fixed::from_int(50));
        assert_eq!(figures.base_hit_aircraft, Fixed::from_ratio(6, 100));
        assert_eq!(figures.base_hit_supply, Fixed::from_ratio(3, 10));
        assert_eq!(figures.fighter_false_claim_chance, Fixed::from_ratio(7, 10));
        assert_eq!(
            figures.anti_aircraft_false_claim_chance,
            Fixed::from_ratio(5, 100)
        );
        assert_eq!(figures.bomb_false_claim_chance, Fixed::from_ratio(1, 10));
        // Decision 0079: the live figure is a placeholder, 0.
        assert_eq!(figures.anti_aircraft_before_attack_share, Fixed::ZERO);
        // Decision 0080: the live figure is a placeholder, 0.
        assert_eq!(figures.patrol_drawn_off_share, Fixed::ZERO);
        // Decision 0087: the live figure is a placeholder, 0.4.
        assert_eq!(figures.patrol_before_attack_share, Fixed::from_ratio(4, 10));
    }

    #[test]
    fn the_base_raid_figures_are_required_in_range_and_not_floats() {
        let bad = |from: &str, to: &str| {
            let text = air_combat_text().replace(from, to);
            assert_ne!(text, air_combat_text(), "{from} is in the file");
            parse_air_combat_figures(&text, here())
                .expect_err("rejected")
                .message
        };
        assert!(bad("base_supply_destroyed_per_hit_t = 50\n", "")
            .contains("base_supply_destroyed_per_hit_t"));
        assert!(bad(
            "base_supply_destroyed_per_hit_t = 50",
            "base_supply_destroyed_per_hit_t = -5"
        )
        .contains("negative"));
        // The table goes, its share for men (decision 0092) with it.
        let no_table = air_combat_text()
            .replace("[base_hit]\naircraft = \"0.06\"\nsupply = \"0.3\"\n", "")
            .replace("men = \"0.2\"\n", "")
            .replace("airfield = \"0.2\"\n", "");
        assert!(parse_air_combat_figures(&no_table, here())
            .expect_err("rejected")
            .message
            .contains("base_hit"));
        assert!(bad("aircraft = \"0.06\"", "aircraft = 0.3").contains("aircraft = 0.3"));
        assert!(bad("aircraft = \"0.06\"", "aircraft = \"1.5\"").contains("base hit chance"));
        assert!(bad("supply = \"0.3\"", "supply = \"-0.1\"").contains("base hit chance"));
        // Together they may not pass 1.
        assert!(bad("aircraft = \"0.06\"", "aircraft = \"0.8\"").contains("add to"));
    }

    #[test]
    fn the_false_claim_chances_are_required_in_range_and_not_floats() {
        // Decision 12 of 0056: three chances, between 0 and 1, as strings.
        let bad = |from: &str, to: &str| {
            let text = air_combat_text().replace(from, to);
            assert_ne!(text, air_combat_text(), "{from} is in the file");
            parse_air_combat_figures(&text, here())
                .expect_err("rejected")
                .message
        };
        for (key, value, label) in [
            (
                "fighter_false_claim_chance",
                "0.7",
                "fighter false claim chance",
            ),
            (
                "anti_aircraft_false_claim_chance",
                "0.05",
                "anti-aircraft false claim chance",
            ),
            ("bomb_false_claim_chance", "0.1", "bomb false claim chance"),
        ] {
            let line = format!("{key} = \"{value}\"");
            assert!(
                bad(&format!("{line}\n"), "").contains(key),
                "{key} is required"
            );
            assert!(
                bad(&line, &format!("{key} = \"1.5\"")).contains(label),
                "{key} above 1"
            );
            assert!(
                bad(&line, &format!("{key} = \"-0.1\"")).contains(label),
                "{key} below 0"
            );
            assert!(
                bad(&line, &format!("{key} = {value}")).contains(key),
                "{key} as a float"
            );
            // The ends of the range are allowed.
            for edge in ["0", "1"] {
                let text = air_combat_text().replace(&line, &format!("{key} = \"{edge}\""));
                parse_air_combat_figures(&text, here()).expect("an edge of the range loads");
            }
        }
    }

    #[test]
    fn the_reach_and_ditching_figures_are_required_in_range_and_not_floats() {
        // Decisions 2, 3 and 6 of 0059. A file without a key is refused,
        // naming it, as with every other key of the file.
        let bad = |from: &str, to: &str| {
            let text = air_combat_text().replace(from, to);
            assert_ne!(text, air_combat_text(), "{from} is in the file");
            parse_air_combat_figures(&text, here())
                .expect_err("rejected")
                .message
        };
        let figures = parse_air_combat_figures(&air_combat_text(), here()).expect("parses");
        assert_eq!(figures.extended_reach_share, Fixed::from_ratio(25, 100));
        assert_eq!(figures.ditch_chance_per_radius_beyond, Fixed::from_int(2));
        assert_eq!(
            figures.crew_recovered_short_of_home,
            Fixed::from_ratio(7, 10)
        );
        // (key, value as written, the largest allowed, a value just above it,
        // the same as a TOML float).
        for (key, value, top, over, float) in [
            ("extended_reach_share", "0.25", "1", "1.5", "0.25"),
            ("ditch_chance_per_radius_beyond", "2", "100", "101", "2.5"),
            ("crew_recovered_short_of_home", "0.7", "1", "1.5", "0.7"),
            ("crew_recovered_over_own_base", "0.6", "1", "1.5", "0.6"),
            (
                "crew_recovered_beside_own_force",
                "0.95",
                "1",
                "1.5",
                "0.95",
            ),
            ("crew_recovered_open_sea", "0.5", "1", "1.5", "0.5"),
            // Decision 0079; the live value is 0.
            ("anti_aircraft_before_attack_share", "0", "1", "1.5", "0.5"),
            // Decision 0080; the live value is 0.
            ("patrol_drawn_off_share", "0", "1", "1.5", "0.5"),
            // Decision 0087; the live value is 0.4.
            ("patrol_before_attack_share", "0.4", "1", "1.5", "0.5"),
        ] {
            let line = format!("{key} = \"{value}\"");
            assert!(
                bad(&format!("{line}\n"), "").contains(key),
                "{key} is required"
            );
            assert!(
                bad(&line, &format!("{key} = \"{over}\"")).contains("not between"),
                "{key} above its range"
            );
            assert!(
                bad(&line, &format!("{key} = \"-0.1\"")).contains("not between"),
                "{key} below 0"
            );
            assert!(
                bad(&line, &format!("{key} = {float}")).contains(key),
                "{key} as a float"
            );
            for edge in ["0", top] {
                let text = air_combat_text().replace(&line, &format!("{key} = \"{edge}\""));
                parse_air_combat_figures(&text, here()).expect("an edge of the range loads");
            }
        }
    }

    #[test]
    fn a_float_in_air_combat_figures_is_rejected() {
        let text = air_combat_text().replace("\"0.2\"\ntorpedo", "0.2\ntorpedo");
        assert_ne!(text, air_combat_text());
        let error = parse_air_combat_figures(&text, here()).expect_err("rejected");
        assert!(error.message.contains("bomb_hit_chance"), "{error}");
    }

    #[test]
    fn air_combat_figures_are_checked() {
        let bad = |from: &str, to: &str| {
            let text = air_combat_text().replace(from, to);
            assert_ne!(text, air_combat_text(), "{from} is in the file");
            parse_air_combat_figures(&text, here())
                .expect_err("rejected")
                .message
        };
        assert!(
            bad("bomb_hit_chance = \"0.2\"", "bomb_hit_chance = \"1.2\"")
                .contains("bomb hit chance")
        );
        assert!(bad(
            "torpedo_hit_chance = \"0.15\"",
            "torpedo_hit_chance = \"-0.1\""
        )
        .contains("torpedo hit chance"));
        assert!(bad(
            "fighter_hit_chance = \"0.3\"",
            "fighter_hit_chance = \"1.5\""
        )
        .contains("fighter hit chance"));
        assert!(bad(
            "anti_aircraft_hit_chance = \"0.05\"",
            "anti_aircraft_hit_chance = \"-0.05\""
        )
        .contains("anti-aircraft hit chance"));
        // Both are required, and neither may be a float.
        assert!(bad("fighter_hit_chance = \"0.3\"\n", "").contains("fighter_hit_chance"));
        assert!(
            bad("anti_aircraft_hit_chance = \"0.05\"\n", "").contains("anti_aircraft_hit_chance")
        );
        assert!(
            bad("fighter_hit_chance = \"0.3\"", "fighter_hit_chance = 0.3")
                .contains("fighter_hit_chance")
        );
        // The two protection factors are required, in range, and not floats.
        assert!(bad("pilot_armour_factor = \"0.75\"\n", "").contains("pilot_armour_factor"));
        assert!(
            bad("self_sealing_tanks_factor = \"0.75\"\n", "").contains("self_sealing_tanks_factor")
        );
        assert!(bad(
            "pilot_armour_factor = \"0.75\"",
            "pilot_armour_factor = \"1.5\""
        )
        .contains("pilot armour factor"));
        assert!(bad(
            "self_sealing_tanks_factor = \"0.75\"",
            "self_sealing_tanks_factor = \"-0.1\""
        )
        .contains("self-sealing tanks factor"));
        assert!(bad(
            "pilot_armour_factor = \"0.75\"",
            "pilot_armour_factor = 0.75"
        )
        .contains("pilot_armour_factor"));
        assert!(bad("flight_deck = \"0.5\"", "flight_deck = \"0.6\"").contains("add to"));
        assert!(bad("[bomb_system_hit]", "[bomb_system_hits]").contains("bomb_system_hits"));
    }

    fn search_text() -> String {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/reference/search.toml");
        fs::read_to_string(path).expect("the shipped file reads")
    }

    #[test]
    fn search_figures_parse_with_the_files_values() {
        let figures = parse_search_figures(&search_text(), here()).expect("parses");
        assert_eq!(figures.contact_drop_age_pulses, 5);
        assert_eq!(figures.merge_distance_hexes, 2);
        assert_eq!(figures.fresh_contact_max_age_pulses, 2);
        let cruiser = &figures.kind_confusion[&ShipKind::Cruiser];
        assert_eq!(cruiser[&ShipKind::Cruiser], Fixed::from_ratio(7, 10));
        assert_eq!(cruiser[&ShipKind::Battleship], Fixed::from_ratio(1, 10));
    }

    #[test]
    fn a_float_in_search_figures_is_rejected() {
        let text = search_text().replace("\"0.3\"", "0.3");
        assert!(parse_search_figures(&text, here()).is_err());
    }

    #[test]
    fn an_unknown_kind_in_search_figures_is_rejected() {
        let text =
            search_text().replace("[kind_confusion.auxiliary]", "[kind_confusion.submarine]");
        let message = parse_search_figures(&text, here())
            .expect_err("rejected")
            .message;
        assert!(message.contains("unknown kind \"submarine\""), "{message}");
        let text = search_text().replace("transport = \"0.3\"", "submarine = \"0.3\"");
        assert!(parse_search_figures(&text, here()).is_err());
    }

    #[test]
    fn a_row_not_adding_to_one_is_rejected() {
        let text = search_text().replace(
            "cruiser = \"0.2\"\n\n[kind_confusion.cruiser]",
            "cruiser = \"0.3\"\n\n[kind_confusion.cruiser]",
        );
        let message = parse_search_figures(&text, here())
            .expect_err("rejected")
            .message;
        assert!(message.contains("not 1"), "{message}");
    }

    #[test]
    fn an_unknown_search_field_is_rejected() {
        let text = format!("{}\nextra = 1\n", search_text());
        assert!(parse_search_figures(&text, here()).is_err());
    }

    #[test]
    fn a_complete_table_loads() {
        let sizes = load("complete", &format!("{HEADER}# a comment\n{}", full_rows()));
        assert_eq!(sizes.expect("loads").len(), 11);
    }

    #[test]
    fn a_missing_size_is_rejected() {
        let rows: String = (0..=10)
            .filter(|size| *size != 7)
            .map(|size| format!("{size},1,50,20\n"))
            .collect();
        let text = message(load("missing", &format!("{HEADER}{rows}")));
        assert!(text.contains("port size 7 is missing"), "{text}");
    }

    #[test]
    fn a_duplicate_size_is_rejected() {
        let text = message(load(
            "duplicate",
            &format!("{HEADER}{}3,1,50,20\n", full_rows()),
        ));
        assert!(text.contains("port size 3 appears twice"), "{text}");
    }

    #[test]
    fn a_size_outside_zero_to_ten_is_rejected() {
        let text = message(load(
            "range",
            &format!("{HEADER}{}11,1,50,20\n", full_rows()),
        ));
        assert!(text.contains("port size 11 is outside"), "{text}");
    }

    #[test]
    fn a_fractional_berth_count_is_rejected() {
        let text = message(load("fraction", &format!("{HEADER}0,1.5,50,20\n")));
        assert!(text.contains("invalid digit"), "{text}");
    }

    #[test]
    fn an_unknown_column_is_rejected() {
        let text = message(load(
            "column",
            "port_size,berths,berth_rate_t_per_h,lighter_rate_t_per_h,extra\n0,1,50,20,9\n",
        ));
        assert!(text.contains("extra"), "{text}");
    }

    #[test]
    fn berths_without_a_berth_rate_are_rejected() {
        let text = message(load("rate", &format!("{HEADER}0,1,0,20\n")));
        assert!(text.contains("berth rate"), "{text}");
    }

    #[test]
    fn the_real_port_table_loads() {
        let path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/reference/port_sizes.csv");
        let sizes = load_port_sizes(&path).expect("the shipped table loads");
        assert_eq!(sizes.len(), 11);
        let one = sizes[&1];
        assert_eq!(one.berths, 1);
        assert_eq!(one.berth_rate_t_per_h, Fixed::from_int(50));
        assert_eq!(one.lighter_rate_t_per_h, Fixed::from_int(20));
        let five = sizes[&5];
        assert_eq!(five.berths, 4);
        assert_eq!(five.berth_rate_t_per_h, Fixed::from_int(150));
        assert_eq!(five.lighter_rate_t_per_h, Fixed::from_int(20));
    }

    const CLASS_HEADER: &str = "id,name,kind,cruise_speed_kn,max_speed_kn,fuel_capacity_t,endurance_nm,cargo_capacity_t,main_guns,gun_rounds_per_min,gun_rounds_per_gun,gun_damage,torpedo_tubes,torpedo_tubes_one_side,torpedo_damage,toughness,radar\n";

    fn classes(rows: &str) -> Result<RefData, DataError> {
        parse_ship_classes(&format!("# c\n{CLASS_HEADER}{rows}"), here())
    }

    #[test]
    fn ship_classes_parse_with_their_combat_columns() {
        let data = classes(
            "1,Cruiser,cruiser,16,32,2000,8000,0,8,3,100,20,8,8,200,400,1\n\
             2,Cargo,transport,10,12,1000,10000,3000,0,0,0,0,0,0,0,300,0\n",
        )
        .expect("parses");
        let cruiser = &data.ship_classes[&ShipClassId::new(1)];
        assert_eq!(cruiser.main_guns, 8);
        assert_eq!(cruiser.gun_rounds_per_min, Fixed::from_int(3));
        assert_eq!(cruiser.gun_rounds_per_gun, 100);
        assert_eq!(cruiser.gun_damage, Fixed::from_int(20));
        assert_eq!(cruiser.torpedo_tubes, 8);
        assert_eq!(cruiser.torpedo_tubes_one_side, 8);
        assert_eq!(cruiser.torpedo_damage, Fixed::from_int(200));
        assert_eq!(cruiser.toughness, Fixed::from_int(400));
        assert!(cruiser.radar);
        let cargo = &data.ship_classes[&ShipClassId::new(2)];
        assert_eq!(cargo.main_guns, 0);
        assert_eq!(cargo.torpedo_tubes, 0);
        assert_eq!(cargo.toughness, Fixed::from_int(300));
        assert!(!cargo.radar);
    }

    #[test]
    fn the_tubes_on_one_side_column_is_read_required_and_checked() {
        // A wing fit: 8 tubes, 4 on a side.
        let data =
            classes("1,Wing,cruiser,16,32,2000,8000,0,8,3,100,20,8,4,200,400,1\n").expect("parses");
        assert_eq!(data.ship_classes[&ShipClassId::new(1)].torpedo_tubes, 8);
        assert_eq!(
            data.ship_classes[&ShipClassId::new(1)].torpedo_tubes_one_side,
            4
        );
        // A file without the column is refused, not read as the whole count.
        let old_header = CLASS_HEADER.replace("torpedo_tubes_one_side,", "");
        let old =
            format!("# c\n{old_header}1,A,cruiser,16,32,2000,8000,0,8,3,100,20,8,200,400,1\n");
        let message = parse_ship_classes(&old, here())
            .expect_err("refused")
            .message;
        assert!(message.contains("torpedo_tubes_one_side"), "{message}");
        // More on a side than tubes, and none on a side with tubes, are refused.
        let more = classes("1,A,cruiser,16,32,2000,8000,0,8,3,100,20,8,9,200,400,1\n")
            .expect_err("refused")
            .message;
        assert!(more.contains("one side"), "{more}");
        let none = classes("1,A,cruiser,16,32,2000,8000,0,8,3,100,20,8,0,200,400,1\n")
            .expect_err("refused")
            .message;
        assert!(none.contains("one side"), "{none}");
    }

    #[test]
    fn gun_rate_of_fire_may_be_a_decimal() {
        let data =
            classes("1,A,cruiser,16,32,2000,8000,0,8,3.5,100,20,8,8,200,400,0\n").expect("parses");
        let rate = data.ship_classes[&ShipClassId::new(1)].gun_rounds_per_min;
        assert_eq!(rate, "3.5".parse::<Fixed>().expect("fixed"));
        assert_eq!(rate * 2, Fixed::from_int(7));
    }

    #[test]
    fn damage_and_toughness_may_be_decimals() {
        let data = classes("1,A,cruiser,16,32,2000,8000,0,9,3.5,100,19.666666,8,8,196,405.44,0\n")
            .expect("parses");
        let class = &data.ship_classes[&ShipClassId::new(1)];
        assert_eq!(class.gun_damage.raw(), 19_666_666);
        assert_eq!(class.torpedo_damage, Fixed::from_int(196));
        assert_eq!(class.toughness.raw(), 405_440_000);
    }

    #[test]
    fn ship_class_combat_errors_are_rejected() {
        let bad = |row: &str| classes(row).expect_err("rejected").message;
        // Zero toughness.
        assert!(
            bad("1,A,cruiser,16,32,2000,8000,0,8,3,100,20,8,8,200,0,0\n").contains("toughness")
        );
        // Radar must be 0 or 1.
        assert!(
            bad("1,A,cruiser,16,32,2000,8000,0,8,3,100,20,8,8,200,400,2\n").contains("radar is 2")
        );
        // Toughness must be above zero, and damage may not be negative.
        assert!(
            bad("1,A,cruiser,16,32,2000,8000,0,8,3,100,20,8,8,200,0.0,0\n").contains("toughness")
        );
        assert!(classes("1,A,cruiser,16,32,2000,8000,0,8,3,100,-20,8,8,200,400,0\n").is_err());
        assert!(classes("1,A,cruiser,16,32,2000,8000,0,8,3,100,20,8,8,200,1e2,0\n").is_err());
        // Whole numbers only for counts: a fraction, a negative and text are rejected.
        assert!(classes("1,A,cruiser,16,32,2000,8000,0,8.5,3,100,20,8,8,200,400,0\n").is_err());
        assert!(classes("1,A,cruiser,16,32,2000,8000,0,-8,3,100,20,8,8,200,400,0\n").is_err());
        assert!(classes("1,A,cruiser,16,32,2000,8000,0,8,3,100,20,8,8,200,400,yes\n").is_err());
        // A missing column and an extra one.
        let short = "id,name,kind,cruise_speed_kn,max_speed_kn,fuel_capacity_t,endurance_nm,cargo_capacity_t\n1,A,cruiser,16,32,2000,8000,0\n";
        assert!(parse_ship_classes(short, here()).is_err());
        let extra = format!(
            "{}\n1,A,cruiser,16,32,2000,8000,0,8,3,100,20,8,8,200,400,0,9\n",
            CLASS_HEADER.trim_end().to_string() + ",extra"
        );
        let message = parse_ship_classes(&extra, here())
            .expect_err("rejected")
            .message;
        assert!(message.contains("extra"), "{message}");
    }

    #[test]
    fn the_aircraft_capacity_column_is_optional() {
        // Without the column every class carries no aircraft (decision 8 of
        // 0025).
        let data = classes("1,A,cruiser,16,32,2000,8000,0,8,3,100,20,8,8,200,400,0\n")
            .expect("parses without the column");
        assert_eq!(data.ship_classes[&ShipClassId::new(1)].aircraft_capacity, 0);

        // With the column, each row's whole number is read.
        let with_column = |rows: &str| {
            parse_ship_classes(
                &format!(
                    "{}{rows}",
                    CLASS_HEADER.trim_end().to_string() + ",aircraft_capacity\n"
                ),
                here(),
            )
        };
        let data = with_column(
            "1,A,cruiser,16,32,2000,8000,0,8,3,100,20,8,8,200,400,0,0\n\
             2,B,carrier,16,30,3000,9000,0,0,0,0,0,0,0,0,600,0,36\n",
        )
        .expect("parses with the column");
        assert_eq!(data.ship_classes[&ShipClassId::new(1)].aircraft_capacity, 0);
        assert_eq!(
            data.ship_classes[&ShipClassId::new(2)].aircraft_capacity,
            36
        );

        // With the column, every row needs a whole number.
        for bad in ["", "-1", "2.5", "many"] {
            let row = format!("1,A,carrier,16,32,2000,8000,0,8,3,100,20,8,8,200,400,0,{bad}\n");
            assert!(with_column(&row).is_err(), "capacity \"{bad}\"");
        }
        // A row that stops before the column is not read as 0.
        assert!(with_column("1,A,carrier,16,32,2000,8000,0,8,3,100,20,8,8,200,400,0\n").is_err());
    }

    #[test]
    fn the_anti_aircraft_column_is_optional_and_read_when_there() {
        // Absent: no shots, whether or not the capacity column is there.
        let data = classes("1,A,cruiser,16,32,2000,8000,0,8,3,100,20,8,8,200,400,0\n")
            .expect("parses without the columns");
        assert_eq!(data.ship_classes[&ShipClassId::new(1)].anti_aircraft, 0);

        let with_column = |rows: &str| {
            parse_ship_classes(
                &format!(
                    "{}{rows}",
                    CLASS_HEADER.trim_end().to_string() + ",aircraft_capacity,anti_aircraft\n"
                ),
                here(),
            )
        };
        let data = with_column(
            "1,A,cruiser,16,32,2000,8000,0,8,3,100,20,8,8,200,400,0,0,8\n\
             2,B,carrier,16,30,3000,9000,0,0,0,0,0,0,0,0,600,0,36,20\n",
        )
        .expect("parses with the column");
        assert_eq!(data.ship_classes[&ShipClassId::new(1)].anti_aircraft, 8);
        assert_eq!(data.ship_classes[&ShipClassId::new(2)].anti_aircraft, 20);
        assert_eq!(
            data.ship_classes[&ShipClassId::new(2)].aircraft_capacity,
            36
        );
        for bad in ["", "-1", "2.5", "many"] {
            let row = format!("1,A,carrier,16,32,2000,8000,0,8,3,100,20,8,8,200,400,0,0,{bad}\n");
            assert!(with_column(&row).is_err(), "anti_aircraft \"{bad}\"");
        }

        // The shipped classes: 1 to 4 have none, 5 and 6 have guns.
        let path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/reference/ship_classes.csv");
        let data = load_ship_classes(&path).expect("the shipped classes load");
        for id in 1..=4 {
            assert_eq!(data.ship_classes[&ShipClassId::new(id)].anti_aircraft, 0);
        }
        assert_eq!(data.ship_classes[&ShipClassId::new(5)].anti_aircraft, 20);
        assert_eq!(data.ship_classes[&ShipClassId::new(6)].anti_aircraft, 8);
    }

    #[test]
    fn a_fighter_type_loads_and_must_attack_with_nothing() {
        let types = parse_aircraft_types(
            &format!("{AIRCRAFT_HEADER}4,Fighter,fighter,0,0,0,none,0,0,0\n"),
            here(),
        )
        .expect("parses");
        assert_eq!(types[&AircraftTypeId::new(4)].role, AircraftRole::Fighter);
        let shipped = load_aircraft_types(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/reference/aircraft_types.csv"),
        )
        .expect("the shipped types load");
        assert_eq!(shipped[&AircraftTypeId::new(4)].role, AircraftRole::Fighter);
        let error = parse_aircraft_types(
            &format!("{AIRCRAFT_HEADER}4,Fighter,fighter,0,0,250,bomb,60,0,0\n"),
            here(),
        )
        .expect_err("a fighter that bombs is rejected");
        assert!(
            error.message.contains("must attack with nothing"),
            "{error}"
        );
    }

    #[test]
    fn the_savo_island_classes_have_no_capacity_column_and_load_with_none() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../data/reference/ship_classes_savo_island.csv");
        let data = load_ship_classes(&path).expect("the generated classes load");
        assert!(!data.ship_classes.is_empty());
        assert!(data
            .ship_classes
            .values()
            .all(|class| class.aircraft_capacity == 0));
    }

    #[test]
    fn the_real_ship_classes_load() {
        let path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/reference/ship_classes.csv");
        let data = load_ship_classes(&path).expect("the shipped classes load");
        let cruiser = &data.ship_classes[&ShipClassId::new(1)];
        assert_eq!(cruiser.main_guns, 8);
        assert_eq!(cruiser.toughness, Fixed::from_int(400));
        assert!(data
            .ship_classes
            .values()
            .all(|class| class.toughness > Fixed::ZERO));
        assert_eq!(
            data.ship_classes[&ShipClassId::new(4)].aircraft_capacity,
            36
        );
    }

    fn combat_text() -> String {
        let path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/reference/surface_combat.toml");
        fs::read_to_string(path).expect("the shipped file reads")
    }

    #[test]
    fn surface_combat_figures_parse_with_the_files_values() {
        let figures = parse_surface_combat_figures(&combat_text(), here()).expect("parses");
        assert_eq!(figures.round_minutes, 5);
        assert_eq!(figures.max_rounds, 6);
        assert_eq!(figures.sighting_chance_night, Fixed::from_ratio(1, 2));
        assert_eq!(figures.sighting_chance_day, Fixed::ONE);
        assert_eq!(figures.radar_sighting_bonus, Fixed::from_ratio(1, 5));
        assert_eq!(figures.not_alert_sighting_factor, Fixed::from_ratio(1, 2));
        assert_eq!(figures.alert_contact_distance_hexes, 3);
        assert_eq!(figures.alarm_chance, Fixed::from_ratio(1, 2));
        assert_eq!(figures.gun_hit_chance_night, Fixed::from_ratio(3, 100));
        assert_eq!(figures.gun_hit_chance_day, Fixed::from_ratio(5, 100));
        assert_eq!(figures.torpedo_hit_chance_night, Fixed::from_ratio(4, 100));
        assert_eq!(figures.torpedo_hit_chance_day, Fixed::from_ratio(2, 100));
        // 2 since tuning round 2 (docs/validation/tuning/r2-surprised-target-factor.md).
        assert_eq!(figures.surprised_target_factor, Fixed::from_int(2));
        assert_eq!(figures.lit_target_factor, Fixed::from_int(2));
        assert_eq!(figures.gun_hit_flotation, Fixed::from_ratio(3, 4));
        assert_eq!(
            figures.system_hit_chance_per_point,
            Fixed::from_ratio(6, 100)
        );
        assert_eq!(figures.degraded_factor, Fixed::from_ratio(1, 2));
        assert_eq!(figures.fire_control_out_factor, Fixed::from_ratio(1, 4));
        let tenths = |entries: &[(ShipSystem, i64)]| -> BTreeMap<ShipSystem, Fixed> {
            entries
                .iter()
                .map(|(system, n)| (*system, Fixed::from_ratio(*n, 100)))
                .collect()
        };
        assert_eq!(
            figures.gun_system_hit,
            tenths(&[
                (ShipSystem::Propulsion, 20),
                (ShipSystem::FireControl, 30),
                (ShipSystem::MainBattery, 35),
                (ShipSystem::TorpedoMounts, 15),
            ])
        );
        // The torpedo table leaves torpedo mounts out: absent, not zero.
        assert_eq!(
            figures.torpedo_system_hit,
            tenths(&[
                (ShipSystem::Propulsion, 80),
                (ShipSystem::FireControl, 10),
                (ShipSystem::MainBattery, 10),
            ])
        );
        let escorts: Vec<(ShipKind, u32)> = figures.straggler_escorts.into_iter().collect();
        assert_eq!(
            escorts,
            vec![
                (ShipKind::Carrier, 2),
                (ShipKind::Battleship, 2),
                (ShipKind::Cruiser, 1),
                (ShipKind::Destroyer, 0),
                (ShipKind::Transport, 1),
                (ShipKind::Auxiliary, 0),
            ]
        );
        assert_eq!(figures.crippled_flotation, Fixed::from_int(50));
        assert_eq!(figures.claimed_sunk_fire, Fixed::from_int(30));
        // The strengths of experience (decision 0064): the placeholder
        // 0.25 in the live file, and 0.5 for guns since tuning round 1
        // (docs/validation/tuning/r1-experience-gun-strength.md).
        assert_eq!(figures.experience_gun_strength, Fixed::from_ratio(1, 2));
        assert_eq!(figures.experience_torpedo_strength, Fixed::from_ratio(1, 4));
        assert_eq!(
            figures.experience_sighting_strength,
            Fixed::from_ratio(1, 4)
        );
    }

    #[test]
    fn the_live_file_has_the_placeholder_share_and_the_frozen_one_switches_it_off() {
        let live = parse_surface_combat_figures(&combat_text(), here()).expect("parses");
        assert_eq!(live.unready_gun_share, Fixed::from_ratio(1, 2));
        assert_eq!(live.unready_torpedo_share, Fixed::from_ratio(1, 2));
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../data/reference/step1/surface_combat.toml");
        let frozen = load_surface_combat_figures(&path).expect("the frozen file loads");
        assert_eq!(frozen.unready_gun_share, Fixed::ONE);
        assert_eq!(frozen.unready_torpedo_share, Fixed::ONE);
    }

    #[test]
    fn the_repair_figures_are_read_and_checked_in_both_files() {
        let live = parse_surface_combat_figures(&combat_text(), here()).expect("parses");
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../data/reference/step1/surface_combat.toml");
        let frozen = load_surface_combat_figures(&path).expect("the frozen file loads");
        for figures in [&live, &frozen] {
            assert_eq!(figures.forward_repair_max_flotation, Fixed::from_int(10));
            assert_eq!(figures.forward_repair_system_points, Fixed::from_int(10));
            assert_eq!(figures.yard_days_per_flotation_point, Fixed::from_int(2));
            assert_eq!(figures.yard_days_per_system_degraded, Fixed::from_int(5));
            assert_eq!(figures.yard_days_per_system_out, Fixed::from_int(30));
        }
        for (key, to, ok) in [
            ("forward_repair_max_flotation", "\"101\"", false),
            ("forward_repair_max_flotation", "\"-1\"", false),
            ("forward_repair_max_flotation", "\"0\"", true),
            ("forward_repair_system_points", "\"0\"", false),
            ("forward_repair_system_points", "\"2.5\"", true),
            ("yard_days_per_flotation_point", "\"-1\"", false),
            ("yard_days_per_system_degraded", "\"-1\"", false),
            ("yard_days_per_system_degraded", "\"31\"", false),
            ("yard_days_per_system_out", "\"4\"", false),
            ("yard_days_per_system_out", "5.5", false),
            ("yard_days_per_system_out", "\"5\"", true),
        ] {
            let text = combat_text();
            let line = text
                .lines()
                .find(|line| line.starts_with(key))
                .expect("the key is in the file")
                .to_string();
            let text = text.replace(&line, &format!("{key} = {to}"));
            let result = parse_surface_combat_figures(&text, here());
            assert_eq!(result.is_ok(), ok, "{key} = {to}");
        }
        for key in [
            "forward_repair_max_flotation",
            "forward_repair_system_points",
            "yard_days_per_flotation_point",
            "yard_days_per_system_degraded",
            "yard_days_per_system_out",
        ] {
            let text = combat_text();
            let line = text
                .lines()
                .find(|line| line.starts_with(key))
                .expect("the key is in the file")
                .to_string();
            let text = text.replace(&format!("{line}\n"), "");
            let message = parse_surface_combat_figures(&text, here())
                .expect_err("a missing key is refused")
                .message;
            assert!(message.contains(key), "{message}");
        }
    }

    #[test]
    fn an_unready_torpedo_share_outside_a_tenth_to_one_or_missing_is_refused() {
        let from = "unready_torpedo_share = \"0.5\"";
        for (to, ok) in [
            ("\"0.1\"", true),
            ("\"1\"", true),
            ("1", true),
            ("\"0.09\"", false),
            ("\"1.01\"", false),
            ("\"0\"", false),
            ("2", false),
            ("0.5", false),
        ] {
            let text = combat_text().replace(from, &format!("unready_torpedo_share = {to}"));
            assert_ne!(text, combat_text(), "{from} is in the file");
            let result = parse_surface_combat_figures(&text, here());
            assert_eq!(result.is_ok(), ok, "unready_torpedo_share = {to}");
        }
        let text = combat_text().replace(&format!("{from}\n"), "");
        let message = parse_surface_combat_figures(&text, here())
            .expect_err("a missing key is refused")
            .message;
        assert!(message.contains("unready_torpedo_share"), "{message}");
    }

    #[test]
    fn an_unready_gun_share_outside_a_tenth_to_one_or_missing_is_refused() {
        let from = "unready_gun_share = \"0.5\"";
        for (to, ok) in [
            ("\"0.1\"", true),
            ("\"1\"", true),
            ("1", true),
            ("\"0.09\"", false),
            ("\"1.01\"", false),
            ("\"0\"", false),
            ("2", false),
            ("0.5", false),
        ] {
            let text = combat_text().replace(from, &format!("unready_gun_share = {to}"));
            assert_ne!(text, combat_text(), "{from} is in the file");
            let result = parse_surface_combat_figures(&text, here());
            assert_eq!(result.is_ok(), ok, "unready_gun_share = {to}");
        }
        let text = combat_text().replace(&format!("{from}\n"), "");
        let message = parse_surface_combat_figures(&text, here())
            .expect_err("a missing key is refused")
            .message;
        assert!(message.contains("unready_gun_share"), "{message}");
    }

    #[test]
    fn the_frozen_surface_combat_file_switches_experience_off() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../data/reference/step1/surface_combat.toml");
        let figures = load_surface_combat_figures(&path).expect("the frozen file loads");
        assert_eq!(figures.experience_gun_strength, Fixed::ZERO);
        assert_eq!(figures.experience_torpedo_strength, Fixed::ZERO);
        assert_eq!(figures.experience_sighting_strength, Fixed::ZERO);
    }

    #[test]
    fn a_strength_of_experience_outside_0_to_half_or_missing_is_refused() {
        for key in [
            "experience_gun_strength",
            "experience_torpedo_strength",
            "experience_sighting_strength",
        ] {
            // The value the live file holds: 0.5 for guns since tuning
            // round 1, 0.25 for the other two.
            let held = if key == "experience_gun_strength" {
                "0.5"
            } else {
                "0.25"
            };
            let from = format!("{key} = \"{held}\"");
            for (to, ok) in [
                ("\"0\"", true),
                ("\"0.5\"", true),
                ("\"0.51\"", false),
                ("\"-0.01\"", false),
                ("1", false),
                ("0.25", false),
            ] {
                let text = combat_text().replace(&from, &format!("{key} = {to}"));
                assert!(combat_text().contains(&from), "{from} is in the file");
                let result = parse_surface_combat_figures(&text, here());
                assert_eq!(result.is_ok(), ok, "{key} = {to}");
            }
            let text = combat_text().replace(&format!("{from}\n"), "");
            let message = parse_surface_combat_figures(&text, here())
                .expect_err("a missing key is refused")
                .message;
            assert!(message.contains(key), "{message}");
        }
    }

    #[test]
    fn a_float_in_surface_combat_figures_is_rejected() {
        let text = combat_text().replace("\"0.03\"", "0.03");
        assert!(parse_surface_combat_figures(&text, here()).is_err());
        let text = combat_text().replace("crippled_flotation = 50", "crippled_flotation = 50.5");
        assert!(parse_surface_combat_figures(&text, here()).is_err());
        let text = combat_text().replace("round_minutes = 5", "round_minutes = 5.0");
        assert!(parse_surface_combat_figures(&text, here()).is_err());
    }

    #[test]
    fn an_unknown_or_missing_surface_combat_field_is_rejected() {
        let text = format!("{}\nextra = 1\n", combat_text());
        assert!(parse_surface_combat_figures(&text, here()).is_err());
        let text = combat_text().replace("max_rounds = 6\n", "");
        let message = parse_surface_combat_figures(&text, here())
            .expect_err("rejected")
            .message;
        assert!(message.contains("max_rounds"), "{message}");
    }

    #[test]
    fn bad_surface_combat_figures_are_rejected_by_their_checks() {
        let bad = |from: &str, to: &str| {
            let text = combat_text().replace(from, to);
            assert_ne!(text, combat_text(), "{from} is in the file");
            parse_surface_combat_figures(&text, here())
                .expect_err("rejected")
                .message
        };
        assert!(bad(
            "gun_hit_flotation = \"0.75\"",
            "gun_hit_flotation = \"1.5\""
        )
        .contains("flotation share"));
        assert!(bad("propulsion = \"0.2\"", "propulsion = \"0.3\"").contains("gun system hit"));
        assert!(
            bad("degraded_factor = \"0.5\"", "degraded_factor = \"1.5\"")
                .contains("degraded factor")
        );
        assert!(bad("round_minutes = 5", "round_minutes = 0").contains("round minutes"));
        assert!(bad("max_rounds = 6", "max_rounds = 0").contains("max rounds"));
        assert!(bad(
            "sighting_chance_night = \"0.5\"",
            "sighting_chance_night = \"1.5\""
        )
        .contains("night sighting"));
        assert!(
            bad("lit_target_factor = \"2\"", "lit_target_factor = \"-2\"").contains("lit target")
        );
        assert!(bad("claimed_sunk_fire = 30", "claimed_sunk_fire = 101").contains("claimed sunk"));
        assert!(bad(
            "radar_sighting_bonus = \"0.2\"",
            "radar_sighting_bonus = \"a fifth\""
        )
        .contains("radar_sighting_bonus"));
    }

    #[test]
    fn a_misspelled_system_key_or_an_unknown_table_is_rejected() {
        let text = combat_text().replace("fire_control = \"0.3\"", "fire_controls = \"0.3\"");
        assert_ne!(text, combat_text());
        assert!(parse_surface_combat_figures(&text, here()).is_err());
        let text = combat_text().replace("main_battery = \"0.1\"", "main_batery = \"0.1\"");
        assert_ne!(text, combat_text());
        assert!(parse_surface_combat_figures(&text, here()).is_err());
        let text = format!("{}\n[extra_table]\npropulsion = \"1\"\n", combat_text());
        assert!(parse_surface_combat_figures(&text, here()).is_err());
        let text = combat_text().replace("destroyer = 0", "destroyers = 0");
        assert_ne!(text, combat_text());
        assert!(parse_surface_combat_figures(&text, here()).is_err());
    }

    #[test]
    fn a_missing_escort_kind_or_system_table_is_rejected() {
        let text = combat_text().replace("transport = 1\n", "");
        assert_ne!(text, combat_text());
        let message = parse_surface_combat_figures(&text, here())
            .expect_err("rejected")
            .message;
        assert!(message.contains("transport"), "{message}");
        let text = combat_text().replace("[straggler_escorts]", "[straggler_escorts_gone]");
        assert!(parse_surface_combat_figures(&text, here()).is_err());
    }

    #[test]
    fn a_float_in_a_system_table_or_the_escorts_is_rejected() {
        let text = combat_text().replace("propulsion = \"0.8\"", "propulsion = 0.8");
        assert_ne!(text, combat_text());
        assert!(parse_surface_combat_figures(&text, here()).is_err());
        let text = combat_text().replace("carrier = 2", "carrier = 2.0");
        assert_ne!(text, combat_text());
        assert!(parse_surface_combat_figures(&text, here()).is_err());
    }

    #[test]
    fn an_absent_system_in_a_table_is_a_share_of_zero_not_an_error() {
        // The torpedo table in the file has no torpedo_mounts key; removing
        // fire control and moving its share to propulsion still adds to 1.
        let text = combat_text()
            .replace("propulsion = \"0.8\"", "propulsion = \"0.9\"")
            .replace("fire_control = \"0.1\"\n", "");
        let figures = parse_surface_combat_figures(&text, here()).expect("parses");
        assert!(!figures
            .torpedo_system_hit
            .contains_key(&ShipSystem::FireControl));
        assert_eq!(figures.torpedo_system_hit.len(), 2);
    }

    fn air_upkeep_text() -> String {
        let path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/reference/air_upkeep.toml");
        fs::read_to_string(path).expect("the shipped file reads")
    }

    #[test]
    fn air_upkeep_figures_parse_with_the_files_values() {
        let figures = parse_air_upkeep_figures(&air_upkeep_text(), here()).expect("parses");
        assert_eq!(figures.wear_base_chance, Fixed::from_ratio(5, 100));
        assert_eq!(figures.wear_chance_per_100_nm, Fixed::from_ratio(2, 100));
        assert_eq!(figures.write_off_share, Fixed::from_ratio(2, 10));
        assert_eq!(figures.carrier_repairs_per_day, 2);
    }

    #[test]
    fn air_upkeep_figures_are_required_in_range_and_not_floats() {
        let bad = |from: &str, to: &str| {
            let text = air_upkeep_text().replace(from, to);
            assert_ne!(text, air_upkeep_text(), "{from} is in the file");
            parse_air_upkeep_figures(&text, here())
                .expect_err("rejected")
                .message
        };
        assert!(
            bad("wear_base_chance = \"0.05\"", "wear_base_chance = 0.05")
                .contains("wear_base_chance")
        );
        assert!(
            bad("wear_base_chance = \"0.05\"", "wear_base_chance = \"1.5\"")
                .contains("wear base chance")
        );
        assert!(bad(
            "wear_chance_per_100_nm = \"0.02\"",
            "wear_chance_per_100_nm = \"-0.02\""
        )
        .contains("per 100 nm"));
        assert!(
            bad("write_off_share = \"0.2\"", "write_off_share = \"2\"").contains("write-off share")
        );
        assert!(bad("write_off_share = \"0.2\"\n", "").contains("write_off_share"));
        assert!(bad("carrier_repairs_per_day = 2\n", "").contains("carrier_repairs_per_day"));
        assert!(bad(
            "carrier_repairs_per_day = 2",
            "carrier_repairs_per_day = -1"
        )
        .contains("-1"));
        assert!(bad("placeholder = true", "placeholder = true\nextra = 1").contains("extra"));
    }
    fn supply_text() -> String {
        let path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/reference/air_supply.toml");
        fs::read_to_string(path).expect("the shipped file reads")
    }

    #[test]
    fn air_supply_figures_parse_with_the_files_values() {
        let figures = parse_air_supply_figures(&supply_text(), here()).expect("parses");
        assert_eq!(
            figures.sortie_supply_t,
            "0.5".parse::<Fixed>().expect("fixed")
        );
        assert_eq!(
            figures.sortie_supply_t_per_100_nm,
            "0.25".parse::<Fixed>().expect("fixed")
        );
        assert_eq!(
            figures.daily_supply_t_per_aircraft,
            "0.2".parse::<Fixed>().expect("fixed")
        );
        assert_eq!(figures.repair_supply_t, Fixed::from_int(1));
    }

    #[test]
    fn air_supply_figures_are_required_not_negative_and_not_floats() {
        let bad = |from: &str, to: &str| {
            let text = supply_text().replace(from, to);
            assert_ne!(text, supply_text(), "{from} is in the file");
            parse_air_supply_figures(&text, here())
                .expect_err("rejected")
                .message
        };
        assert!(bad("repair_supply_t = \"1\"", "repair_supply_t = 1.5").contains("repair_supply_t"));
        assert!(bad("repair_supply_t = \"1\"", "repair_supply_t = -1").contains("repair"));
        assert!(
            bad("sortie_supply_t = \"0.5\"", "sortie_supply_t = \"-0.5\"").contains("negative")
        );
        assert!(bad("repair_supply_t = \"1\"", "").contains("repair_supply_t"));
        assert!(bad("placeholder = true", "placeholder = true\nextra = 1").contains("extra"));
        // Whole numbers are accepted.
        let text = supply_text().replace("repair_supply_t = \"1\"", "repair_supply_t = 2");
        assert_eq!(
            parse_air_supply_figures(&text, here())
                .expect("parses")
                .repair_supply_t,
            Fixed::from_int(2)
        );
    }

    fn ground_text() -> String {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/reference/ground.toml");
        fs::read_to_string(path).expect("the shipped file reads")
    }

    #[test]
    fn ground_figures_parse_with_the_files_values() {
        let figures = parse_ground_figures(&ground_text(), here()).expect("parses");
        assert_eq!(figures.handling_t_per_man, Fixed::from_ratio(1, 4));
        assert_eq!(figures.daily_supply_t_per_man, Fixed::from_ratio(1, 200));
        assert_eq!(figures.unfed_loss_share_per_day, Fixed::from_ratio(1, 20));
    }

    #[test]
    fn ground_figures_reject_floats_unknown_keys_and_bad_values() {
        let bad = |from: &str, to: &str| {
            let text = ground_text().replace(from, to);
            assert_ne!(text, ground_text(), "{from} is in the file");
            parse_ground_figures(&text, here())
                .expect_err("rejected")
                .message
        };
        assert!(
            bad("handling_t_per_man = \"0.25\"", "handling_t_per_man = 0.25")
                .contains("handling_t_per_man")
        );
        assert!(bad("placeholder = true", "placeholder = true\nextra = 1").contains("extra"));
        assert!(bad("daily_supply_t_per_man = \"0.005\"", "").contains("daily_supply_t_per_man"));
        assert!(bad(
            "unfed_loss_share_per_day = \"0.05\"",
            "unfed_loss_share_per_day = \"1.5\""
        )
        .contains("unfed_loss_share_per_day"));
        assert!(
            bad("handling_t_per_man = \"0.25\"", "handling_t_per_man = 0")
                .contains("handling_t_per_man")
        );
        // Whole numbers are accepted.
        let text = ground_text().replace(
            "unfed_loss_share_per_day = \"0.05\"",
            "unfed_loss_share_per_day = 1",
        );
        assert_eq!(
            parse_ground_figures(&text, here())
                .expect("parses")
                .unfed_loss_share_per_day,
            Fixed::ONE
        );
    }

    #[test]
    fn the_march_figures_are_read_and_each_is_required() {
        let figures = parse_ground_figures(&ground_text(), here()).expect("parses");
        assert_eq!(figures.march_days_per_hex, 3);
        assert_eq!(figures.fatigue_per_march_day, Fixed::from_int(10));
        assert_eq!(figures.march_supply_days, Fixed::from_int(5));
        for key in [
            "march_days_per_hex",
            "fatigue_per_march_day",
            "march_supply_days",
        ] {
            let text = without_keys(&ground_text(), &[key]);
            assert!(
                parse_ground_figures(&text, here())
                    .expect_err("a missing key is refused")
                    .message
                    .contains(key),
                "without {key}"
            );
        }
    }

    #[test]
    fn the_march_figures_refuse_floats_and_bad_values() {
        let bad = |key: &str, value: &str| {
            parse_ground_figures(&with_key(&ground_text(), key, value), here())
                .expect_err("rejected")
                .message
        };
        // Days: a whole number of 1 or more.
        assert!(bad("march_days_per_hex", "0").contains("march_days_per_hex"));
        assert!(bad("march_days_per_hex", "-1").contains("march_days_per_hex"));
        assert!(bad("march_days_per_hex", "2.5").contains("march_days_per_hex"));
        assert!(bad("march_days_per_hex", "\"3\"").contains("march_days_per_hex"));
        let one = with_key(&ground_text(), "march_days_per_hex", "1");
        assert_eq!(
            parse_ground_figures(&one, here())
                .expect("1 loads")
                .march_days_per_hex,
            1
        );
        // Fatigue: 0 to 100, no float.
        assert!(bad("fatigue_per_march_day", "-1").contains("fatigue_per_march_day"));
        assert!(bad("fatigue_per_march_day", "101").contains("fatigue_per_march_day"));
        assert!(bad("fatigue_per_march_day", "10.5").contains("fatigue_per_march_day"));
        // Supply days: 0 or more, no float.
        assert!(bad("march_supply_days", "-1").contains("march_supply_days"));
        assert!(bad("march_supply_days", "5.5").contains("march_supply_days"));
        let text = with_key(&ground_text(), "march_supply_days", "\"2.5\"");
        assert_eq!(
            parse_ground_figures(&text, here())
                .expect("a decimal string loads")
                .march_supply_days,
            Fixed::from_ratio(5, 2)
        );
        let zero = with_key(&ground_text(), "march_supply_days", "0");
        assert!(parse_ground_figures(&zero, here()).is_ok());
    }

    #[test]
    fn the_preparation_and_experience_figures_are_read_and_each_is_required() {
        let figures = parse_ground_figures(&ground_text(), here()).expect("parses");
        assert_eq!(figures.preparation_per_day, Fixed::from_int(5));
        assert_eq!(figures.unprepared_landing_share, Fixed::from_ratio(1, 2));
        assert_eq!(figures.preparation_kept_on_change, Fixed::from_ratio(1, 2));
        assert_eq!(figures.unprepared_landing_fatigue, Fixed::from_int(40));
        assert_eq!(figures.experience_fire_strength, Fixed::from_ratio(1, 4));
        assert_eq!(figures.training_days_per_level, 30);
        assert_eq!(figures.training_ceiling, 2);
        assert_eq!(figures.fight_days_per_level, 10);
        for key in PREPARATION_KEYS {
            let text = without_keys(&ground_text(), &[key]);
            assert!(
                parse_ground_figures(&text, here())
                    .expect_err("a missing key is refused")
                    .message
                    .contains(key),
                "without {key}"
            );
        }
    }

    const SKILL_KEYS: [&str; 4] = [
        "assault_skill_strength",
        "movement_skill_strength",
        "holding_skill_strength",
        "weight_strength",
    ];

    #[test]
    fn the_skill_strengths_are_read_and_each_is_required() {
        let figures = parse_ground_figures(&ground_text(), here()).expect("parses");
        assert_eq!(figures.assault_skill_strength, Fixed::from_ratio(1, 4));
        assert_eq!(figures.movement_skill_strength, Fixed::from_ratio(1, 4));
        assert_eq!(figures.holding_skill_strength, Fixed::from_ratio(15, 100));
        assert_eq!(figures.weight_strength, Fixed::from_ratio(1, 4));
        for key in SKILL_KEYS {
            let text = without_keys(&ground_text(), &[key]);
            assert!(
                parse_ground_figures(&text, here())
                    .expect_err("a missing key is refused")
                    .message
                    .contains(key),
                "without {key}"
            );
        }
    }

    #[test]
    fn the_skill_strengths_are_at_least_0_and_below_one_half_and_no_float() {
        let bad = |key: &str, value: &str| {
            parse_ground_figures(&with_key(&ground_text(), key, value), here())
                .expect_err("rejected")
                .message
        };
        let ok = |key: &str, value: &str| {
            parse_ground_figures(&with_key(&ground_text(), key, value), here())
                .unwrap_or_else(|e| panic!("{key} {value} loads: {}", e.message))
        };
        for key in SKILL_KEYS {
            assert!(bad(key, "-1").contains(key));
            assert!(bad(key, "\"-0.1\"").contains(key));
            assert!(bad(key, "\"0.5\"").contains(key));
            assert!(bad(key, "1").contains(key));
            assert!(bad(key, "0.25").contains(key));
            ok(key, "0");
            ok(key, "\"0.49\"");
        }
    }

    const WORK_KEYS: [&str; 7] = [
        "work_per_man_day",
        "airfield_work",
        "work_to_double_unloading",
        "work_per_aircraft_repair",
        "work_per_defence_point",
        "defences_strength",
        "fighting_unit_work_share",
    ];

    #[test]
    fn the_work_figures_are_read_and_each_is_required() {
        let figures = parse_ground_figures(&ground_text(), here()).expect("parses");
        assert_eq!(figures.work_per_man_day, Fixed::ONE);
        assert_eq!(figures.airfield_work, Fixed::from_int(20_000));
        assert_eq!(figures.work_to_double_unloading, Fixed::from_int(1_000));
        assert_eq!(figures.work_per_aircraft_repair, Fixed::from_int(200));
        assert_eq!(figures.work_per_defence_point, Fixed::from_int(200));
        assert_eq!(figures.defences_strength, Fixed::from_ratio(1, 2));
        assert_eq!(figures.fighting_unit_work_share, Fixed::from_ratio(1, 2));
        for key in WORK_KEYS {
            let text = without_keys(&ground_text(), &[key]);
            assert!(
                parse_ground_figures(&text, here())
                    .expect_err("a missing key is refused")
                    .message
                    .contains(key),
                "without {key}"
            );
        }
    }

    #[test]
    fn the_beaten_fatigue_is_read_required_and_in_range() {
        let figures = parse_ground_figures(&ground_text(), here()).expect("parses");
        assert_eq!(figures.beaten_fatigue, Fixed::from_int(30));
        let text = without_keys(&ground_text(), &["beaten_fatigue"]);
        assert!(parse_ground_figures(&text, here())
            .expect_err("a missing key is refused")
            .message
            .contains("beaten_fatigue"));
        let bad = |value: &str| {
            parse_ground_figures(&with_key(&ground_text(), "beaten_fatigue", value), here())
                .expect_err("rejected")
                .message
        };
        assert!(bad("101").contains("beaten_fatigue"));
        assert!(bad("-1").contains("beaten_fatigue"));
        assert!(bad("0.5").contains("beaten_fatigue"));
        for value in ["0", "100"] {
            parse_ground_figures(&with_key(&ground_text(), "beaten_fatigue", value), here())
                .expect("loads");
        }
    }

    const AIRFIELD_KEYS: [&str; 3] = [
        "airfield_damage_per_hit",
        "work_per_airfield_damage_point",
        "airfield_landing_refused_at",
    ];

    #[test]
    fn the_airfield_figures_are_read_each_is_required_and_in_range() {
        let figures = parse_ground_figures(&ground_text(), here()).expect("parses");
        assert_eq!(figures.airfield_damage_per_hit, Fixed::from_int(2));
        assert_eq!(figures.work_per_airfield_damage_point, Fixed::from_int(50));
        assert_eq!(figures.airfield_landing_refused_at, Fixed::from_int(50));
        for key in AIRFIELD_KEYS {
            let text = without_keys(&ground_text(), &[key]);
            assert!(
                parse_ground_figures(&text, here())
                    .expect_err("a missing key is refused")
                    .message
                    .contains(key),
                "without {key}"
            );
        }
        let bad = |key: &str, value: &str| {
            parse_ground_figures(&with_key(&ground_text(), key, value), here())
                .expect_err("rejected")
                .message
        };
        let ok = |key: &str, value: &str| {
            parse_ground_figures(&with_key(&ground_text(), key, value), here())
                .unwrap_or_else(|e| panic!("{key} {value} loads: {}", e.message))
        };
        assert!(bad("airfield_damage_per_hit", "-1").contains("airfield_damage_per_hit"));
        ok("airfield_damage_per_hit", "0");
        assert!(
            bad("work_per_airfield_damage_point", "0").contains("work_per_airfield_damage_point")
        );
        ok("work_per_airfield_damage_point", "1");
        assert!(bad("airfield_landing_refused_at", "101").contains("airfield_landing_refused_at"));
        assert!(bad("airfield_landing_refused_at", "-1").contains("airfield_landing_refused_at"));
        ok("airfield_landing_refused_at", "0");
        ok("airfield_landing_refused_at", "100");
        for key in AIRFIELD_KEYS {
            assert!(bad(key, "0.5").contains(key));
        }
    }

    #[test]
    fn the_work_figures_are_in_range_and_no_float() {
        let bad = |key: &str, value: &str| {
            parse_ground_figures(&with_key(&ground_text(), key, value), here())
                .expect_err("rejected")
                .message
        };
        let ok = |key: &str, value: &str| {
            parse_ground_figures(&with_key(&ground_text(), key, value), here())
                .unwrap_or_else(|e| panic!("{key} {value} loads: {}", e.message))
        };
        for key in WORK_KEYS {
            assert!(bad(key, "-1").contains(key));
            assert!(bad(key, "0.5").contains(key));
            ok(key, "\"0.5\"");
        }
        // The man-day work may be 0; the four that divide may not.
        ok("work_per_man_day", "0");
        for key in [
            "airfield_work",
            "work_to_double_unloading",
            "work_per_aircraft_repair",
            "work_per_defence_point",
        ] {
            assert!(bad(key, "0").contains(key));
            ok(key, "1");
        }
        // The two shares run from 0 to 1.
        for key in ["defences_strength", "fighting_unit_work_share"] {
            ok(key, "0");
            ok(key, "1");
            assert!(bad(key, "\"1.5\"").contains(key));
            assert!(bad(key, "2").contains(key));
        }
    }

    const PREPARATION_KEYS: [&str; 8] = [
        "preparation_per_day",
        "unprepared_landing_share",
        "preparation_kept_on_change",
        "unprepared_landing_fatigue",
        "experience_fire_strength",
        "training_days_per_level",
        "training_ceiling",
        "fight_days_per_level",
    ];

    #[test]
    fn the_preparation_and_experience_figures_refuse_floats_and_bad_values() {
        let bad = |key: &str, value: &str| {
            parse_ground_figures(&with_key(&ground_text(), key, value), here())
                .expect_err("rejected")
                .message
        };
        let ok = |key: &str, value: &str| {
            parse_ground_figures(&with_key(&ground_text(), key, value), here())
                .unwrap_or_else(|e| panic!("{key} {value} loads: {}", e.message))
        };
        // Quantities to 100: 0 to 100, no float.
        for key in ["preparation_per_day", "unprepared_landing_fatigue"] {
            assert!(bad(key, "-1").contains(key));
            assert!(bad(key, "101").contains(key));
            assert!(bad(key, "5.5").contains(key));
            ok(key, "0");
            ok(key, "100");
            ok(key, "\"2.5\"");
        }
        // Shares: 0 to 1, no float.
        for key in ["unprepared_landing_share", "preparation_kept_on_change"] {
            assert!(bad(key, "-1").contains(key));
            assert!(bad(key, "2").contains(key));
            assert!(bad(key, "\"1.01\"").contains(key));
            assert!(bad(key, "0.5").contains(key));
            ok(key, "0");
            ok(key, "1");
        }
        // Strength: 0 to 0.5.
        let key = "experience_fire_strength";
        assert!(bad(key, "-1").contains(key));
        assert!(bad(key, "\"0.51\"").contains(key));
        assert!(bad(key, "1").contains(key));
        assert!(bad(key, "0.25").contains(key));
        ok(key, "0");
        ok(key, "\"0.5\"");
        // Counts: whole numbers of 1 or more, and a ceiling of 0 to 4.
        for key in ["training_days_per_level", "fight_days_per_level"] {
            assert!(bad(key, "0").contains(key));
            assert!(bad(key, "-1").contains(key));
            assert!(bad(key, "2.5").contains(key));
            assert!(bad(key, "\"3\"").contains(key));
            let one = ok(key, "1");
            assert_eq!(one.training_days_per_level.min(one.fight_days_per_level), 1);
        }
        let key = "training_ceiling";
        assert!(bad(key, "5").contains(key));
        assert!(bad(key, "-1").contains(key));
        assert!(bad(key, "2.5").contains(key));
        assert_eq!(ok(key, "0").training_ceiling, 0);
        assert_eq!(ok(key, "4").training_ceiling, 4);
    }

    #[test]
    fn the_troop_capacity_column_defaults_to_0_and_is_read_when_present() {
        let without = classes("1,Cargo,transport,10,12,1000,10000,3000,0,0,0,0,0,0,0,300,0\n")
            .expect("parses");
        assert_eq!(
            without.ship_classes[&ShipClassId::new(1)].troop_capacity_men,
            0
        );
        let header = CLASS_HEADER.trim_end().to_string()
            + ",aircraft_capacity,anti_aircraft,troop_capacity_men\n";
        let with = parse_ship_classes(
            &format!("# c\n{header}1,Troop ship,transport,10,12,1000,10000,3000,0,0,0,0,0,0,0,300,0,0,0,600\n"),
            here(),
        )
        .expect("parses");
        assert_eq!(
            with.ship_classes[&ShipClassId::new(1)].troop_capacity_men,
            600
        );
        for bad in ["-1", "1.5", "many"] {
            let text = format!("# c\n{header}1,Troop ship,transport,10,12,1000,10000,3000,0,0,0,0,0,0,0,300,0,0,0,{bad}\n");
            assert!(
                parse_ship_classes(&text, here()).is_err(),
                "{bad} is refused"
            );
        }
    }

    fn fatigue_text() -> String {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/reference/fatigue.toml");
        fs::read_to_string(path).expect("the shipped file reads")
    }

    #[test]
    fn fatigue_figures_parse_with_the_files_values() {
        let figures = parse_fatigue_figures(&fatigue_text(), here()).expect("parses");
        assert_eq!(figures.air_fatigue_per_sortie, Fixed::from_int(3));
        assert_eq!(figures.air_fatigue_per_100_nm, Fixed::from_int(2));
        assert_eq!(figures.air_rest_per_pulse, Fixed::from_int(5));
        assert_eq!(figures.force_fatigue_per_pulse_alert, Fixed::from_int(5));
        assert_eq!(figures.force_rest_per_pulse, Fixed::from_int(3));
    }

    #[test]
    fn fatigue_figures_are_required_not_negative_and_not_floats() {
        let bad = |from: &str, to: &str| {
            let text = fatigue_text().replace(from, to);
            assert_ne!(text, fatigue_text(), "{from} is in the file");
            parse_fatigue_figures(&text, here())
                .expect_err("rejected")
                .message
        };
        assert!(
            bad("air_fatigue_per_sortie = 3", "air_fatigue_per_sortie = 3.5")
                .contains("air_fatigue_per_sortie")
        );
        assert!(
            bad("air_fatigue_per_100_nm = 2", "air_fatigue_per_100_nm = -2").contains("per 100 nm")
        );
        assert!(bad("air_rest_per_pulse = 5", "air_rest_per_pulse = \"101\"").contains("rest"));
        assert!(bad("air_rest_per_pulse = 5\n", "").contains("air_rest_per_pulse"));
        // The two force figures (decision 0037) are required, whole numbers
        // or decimal strings, and between 0 and 100.
        assert!(bad("force_rest_per_pulse = 3\n", "").contains("force_rest_per_pulse"));
        assert!(bad("force_fatigue_per_pulse_alert = 5\n", "")
            .contains("force_fatigue_per_pulse_alert"));
        assert!(bad(
            "force_fatigue_per_pulse_alert = 5",
            "force_fatigue_per_pulse_alert = 5.5"
        )
        .contains("force_fatigue_per_pulse_alert"));
        assert!(bad("force_rest_per_pulse = 3", "force_rest_per_pulse = -3").contains("force rest"));
        assert!(bad(
            "force_fatigue_per_pulse_alert = 5",
            "force_fatigue_per_pulse_alert = \"101\""
        )
        .contains("force fatigue"));
        assert!(bad("placeholder = true", "placeholder = true\nextra = 1").contains("extra"));
        // A decimal string is accepted.
        let text = fatigue_text().replace("air_rest_per_pulse = 5", "air_rest_per_pulse = \"2.5\"");
        let figures = parse_fatigue_figures(&text, here()).expect("parses");
        assert_eq!(
            figures.air_rest_per_pulse,
            "2.5".parse::<Fixed>().expect("fixed")
        );
        let text =
            fatigue_text().replace("force_rest_per_pulse = 3", "force_rest_per_pulse = \"1.5\"");
        let figures = parse_fatigue_figures(&text, here()).expect("parses");
        assert_eq!(
            figures.force_rest_per_pulse,
            "1.5".parse::<Fixed>().expect("fixed")
        );
    }

    // ------------------------------------------------------------------
    // The figures of decision 0092: shore bombardment, the share of a base
    // hit that lands on men, and the ground report.
    // ------------------------------------------------------------------

    /// `text` without the lines that give any of `keys`.
    fn without_keys(text: &str, keys: &[&str]) -> String {
        let kept: Vec<&str> = text
            .lines()
            .filter(|line| {
                !keys
                    .iter()
                    .any(|key| line.trim_start().starts_with(&format!("{key} =")))
            })
            .collect();
        format!("{}\n", kept.join("\n"))
    }

    /// `text` with the line giving `key` replaced by `key = value`.
    fn with_key(text: &str, key: &str, value: &str) -> String {
        let mut found = false;
        let lines: Vec<String> = text
            .lines()
            .map(|line| {
                if line.trim_start().starts_with(&format!("{key} =")) {
                    found = true;
                    format!("{key} = {value}")
                } else {
                    line.to_string()
                }
            })
            .collect();
        assert!(found, "{key} is in the file");
        format!("{}\n", lines.join("\n"))
    }

    const BOMBARDMENT_KEYS: [&str; 4] = [
        "bombardment_minutes",
        "bombardment_hit_chance",
        "bombardment_reference_gun_damage",
        "bombardment_false_claim_chance",
    ];

    #[test]
    fn the_bombardment_figures_load_all_four_or_none() {
        let loaded = parse_surface_combat_figures(&combat_text(), here()).expect("parses");
        assert!(loaded.bombardment.is_some(), "the shipped file has them");
        // None of the four: the file loads and no force bombards.
        let none = without_keys(&combat_text(), &BOMBARDMENT_KEYS);
        assert_eq!(
            parse_surface_combat_figures(&none, here())
                .expect("loads as before")
                .bombardment,
            None
        );
        // Any part of the set is refused, with the four named.
        for count in 1..=3 {
            for start in 0..4 {
                let dropped: Vec<&str> = (0..count)
                    .map(|i| BOMBARDMENT_KEYS[(start + i) % 4])
                    .collect();
                let partial = without_keys(&combat_text(), &dropped);
                let message = parse_surface_combat_figures(&partial, here())
                    .expect_err("a partial set is refused")
                    .message;
                assert!(
                    message.contains("give all four or none"),
                    "without {dropped:?}: {message}"
                );
            }
        }
    }

    #[test]
    fn the_bombardment_figures_are_in_range_and_not_floats() {
        let bad = |key: &str, value: &str| {
            parse_surface_combat_figures(&with_key(&combat_text(), key, value), here())
                .expect_err("rejected")
                .message
        };
        assert!(bad("bombardment_minutes", "0").contains("bombardment_minutes"));
        assert!(bad("bombardment_reference_gun_damage", "0")
            .contains("bombardment_reference_gun_damage"));
        assert!(bad("bombardment_reference_gun_damage", "-5")
            .contains("bombardment_reference_gun_damage"));
        assert!(bad("bombardment_hit_chance", "\"1.5\"").contains("bombardment_hit_chance"));
        assert!(bad("bombardment_hit_chance", "\"-0.1\"").contains("bombardment_hit_chance"));
        assert!(
            bad("bombardment_false_claim_chance", "2").contains("bombardment_false_claim_chance")
        );
        assert!(bad("bombardment_false_claim_chance", "\"-0.5\"")
            .contains("bombardment_false_claim_chance"));
        // A TOML float is refused for a quantity, and a fraction of a minute
        // for the whole number of minutes.
        assert!(bad("bombardment_hit_chance", "0.05").contains("bombardment_hit_chance"));
        assert!(bad("bombardment_reference_gun_damage", "20.5")
            .contains("bombardment_reference_gun_damage"));
        assert!(!bad("bombardment_minutes", "30.5").is_empty());
        // The edges are allowed: a chance of 0 and of 1.
        for edge in ["0", "1"] {
            let text = with_key(&combat_text(), "bombardment_hit_chance", edge);
            let figures = parse_surface_combat_figures(&text, here()).expect("the edge loads");
            assert_eq!(
                figures.bombardment.expect("held").hit_chance,
                Fixed::from_int(edge.parse().unwrap())
            );
        }
    }

    #[test]
    fn the_ground_report_error_is_both_keys_or_neither_and_in_range() {
        let loaded = parse_ground_figures(&ground_text(), here()).expect("parses");
        let error = loaded.report_error.expect("the shipped file has them");
        assert!(error.low <= error.high);
        // Only one of the two is refused.
        for dropped in ["report_error_low", "report_error_high"] {
            let text = without_keys(&ground_text(), &[dropped]);
            let message = parse_ground_figures(&text, here())
                .expect_err("one alone is refused")
                .message;
            assert!(
                message.contains("report_error_low and report_error_high together"),
                "without {dropped}: {message}"
            );
        }
        // Out of range: below 0, or the low above the high.
        let bad = |key: &str, value: &str| {
            parse_ground_figures(&with_key(&ground_text(), key, value), here())
                .expect_err("rejected")
                .message
        };
        assert!(bad("report_error_low", "\"-0.1\"").contains("report_error_low"));
        assert!(bad("report_error_high", "\"0.1\"")
            .contains("not 0 or more with the low not above the high"));
        assert!(bad("report_error_low", "\"2\"").contains("report_error_low"));
        // A float is refused, as every quantity.
        assert!(bad("report_error_low", "0.5").contains("report_error_low"));
        assert!(bad("report_error_high", "1.5").contains("report_error_high"));
        // The edge where they meet is allowed: the report is then exact.
        let exact = with_key(&ground_text(), "report_error_low", "1");
        let exact = with_key(&exact, "report_error_high", "1");
        let figures = parse_ground_figures(&exact, here()).expect("an exact report loads");
        let error = figures.report_error.expect("held");
        assert_eq!((error.low, error.high), (Fixed::ONE, Fixed::ONE));
    }

    #[test]
    fn the_men_lost_per_base_hit_is_a_whole_number_of_0_or_more() {
        let bad = |value: &str| {
            parse_ground_figures(
                &with_key(&ground_text(), "men_lost_per_base_hit", value),
                here(),
            )
            .expect_err("rejected")
            .message
        };
        assert!(bad("-1").contains("men_lost_per_base_hit"));
        assert!(bad("\"2.5\"").contains("men_lost_per_base_hit"));
        assert!(bad("2.5").contains("men_lost_per_base_hit"));
        // 0 is allowed: a hit on men then removes nobody.
        let zero = with_key(&ground_text(), "men_lost_per_base_hit", "0");
        assert_eq!(
            parse_ground_figures(&zero, here())
                .expect("0 loads")
                .men_lost_per_base_hit,
            Fixed::ZERO
        );
    }

    #[test]
    fn the_share_of_hits_that_land_on_men_is_in_range_and_adds_up_with_the_others() {
        let bad = |value: &str| {
            parse_air_combat_figures(&with_key(&air_combat_text(), "men", value), here())
                .expect_err("rejected")
                .message
        };
        assert!(bad("\"1.5\"").contains("base hit chance on men"));
        assert!(bad("\"-0.1\"").contains("base hit chance on men"));
        assert!(bad("0.2").contains("men"));
        // Together with aircraft and supply it may not pass 1.
        assert!(bad("\"0.99\"").contains("add to"));
    }

    #[test]
    fn files_without_the_figures_of_decision_0092_load_as_before() {
        let surface = without_keys(&combat_text(), &BOMBARDMENT_KEYS);
        assert_eq!(
            parse_surface_combat_figures(&surface, here())
                .expect("loads")
                .bombardment,
            None
        );
        let air = without_keys(&air_combat_text(), &["men"]);
        assert_eq!(
            parse_air_combat_figures(&air, here())
                .expect("loads")
                .base_hit_men,
            Fixed::ZERO,
            "no share for men"
        );
        let ground = without_keys(
            &ground_text(),
            &[
                "men_lost_per_base_hit",
                "report_error_low",
                "report_error_high",
            ],
        );
        let figures = parse_ground_figures(&ground, here()).expect("loads");
        assert_eq!(figures.men_lost_per_base_hit, Fixed::ZERO);
        assert_eq!(figures.report_error, None);
        // The rest of the ground figures are untouched.
        let full = parse_ground_figures(&ground_text(), here()).expect("parses");
        assert_eq!(figures.men_per_shot, full.men_per_shot);
        assert_eq!(figures.beaten_share, full.beaten_share);
        // The frozen files of step 1 have the figures too, with their first
        // values.
        let frozen = |name: &str| {
            fs::read_to_string(
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../../data/reference/step1")
                    .join(name),
            )
            .expect("the frozen file reads")
        };
        let ground = parse_ground_figures(&frozen("ground.toml"), here()).expect("parses");
        assert_eq!(ground.men_lost_per_base_hit, Fixed::from_int(10));
        assert_eq!(ground.march_days_per_hex, 3);
        assert_eq!(ground.fatigue_per_march_day, Fixed::from_int(10));
        assert_eq!(ground.march_supply_days, Fixed::from_int(5));
        // The first values of decisions 0095 and 0097 switch the rules off:
        // no unprepared landing and no effect of experience.
        assert_eq!(ground.preparation_per_day, Fixed::from_int(5));
        assert_eq!(ground.unprepared_landing_share, Fixed::ONE);
        assert_eq!(ground.preparation_kept_on_change, Fixed::from_ratio(1, 2));
        assert_eq!(ground.unprepared_landing_fatigue, Fixed::ZERO);
        assert_eq!(ground.experience_fire_strength, Fixed::ZERO);
        assert_eq!(ground.training_days_per_level, 30);
        assert_eq!(ground.training_ceiling, 2);
        assert_eq!(ground.fight_days_per_level, 10);
        // The strengths of decision 0098 are 0: no skill does anything.
        assert_eq!(ground.assault_skill_strength, Fixed::ZERO);
        assert_eq!(ground.movement_skill_strength, Fixed::ZERO);
        assert_eq!(ground.holding_skill_strength, Fixed::ZERO);
        assert_eq!(ground.weight_strength, Fixed::ZERO);
        // No frozen scenario has a working unit, so the figures of decision
        // 0099 do nothing there.
        assert_eq!(ground.work_per_man_day, Fixed::ONE);
        assert_eq!(ground.airfield_work, Fixed::from_int(20_000));
        let error = ground.report_error.expect("held");
        assert_eq!(
            (error.low, error.high),
            (Fixed::from_ratio(1, 2), Fixed::from_ratio(3, 2))
        );
        let surface =
            parse_surface_combat_figures(&frozen("surface_combat.toml"), here()).expect("parses");
        let bombardment = surface.bombardment.expect("held");
        assert_eq!(bombardment.minutes, 30);
        assert_eq!(bombardment.hit_chance, Fixed::from_ratio(5, 100));
        assert_eq!(bombardment.reference_gun_damage, Fixed::from_int(20));
        assert_eq!(bombardment.false_claim_chance, Fixed::from_ratio(1, 100));
        let air = parse_air_combat_figures(&frozen("air_combat.toml"), here()).expect("parses");
        assert_eq!(air.base_hit_men, Fixed::from_ratio(2, 10));
        // The share for the airfield (decision 0103) is 0 in the frozen figures.
        assert_eq!(air.base_hit_airfield, Fixed::ZERO);
    }
}
