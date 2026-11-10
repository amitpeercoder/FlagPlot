use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde::Deserialize;
use sim_core::fixed::Fixed;
use sim_core::ids::{AircraftTypeId, ShipClassId};
use sim_core::refdata::{
    AircraftRole, AircraftType, PortSize, RefData, SearchFigures, ShipClass, ShipKind,
    SurfaceCombatFigures,
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
    torpedo_damage: String,
    toughness: String,
    /// 0 or 1.
    radar: u8,
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
            name: row.name,
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
            other => {
                return Err(DataError::in_file(
                    path,
                    format!("{id}: unknown role \"{other}\""),
                ))
            }
        };
        let aircraft_type = AircraftType {
            id,
            role,
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
    initial_confidence: Quantity,
    confidence_decay_per_pulse: Quantity,
    drop_below_confidence: Quantity,
    merge_distance_hexes: u32,
    merge_max_age_pulses: u32,
    position_error_chance: Quantity,
    count_over_chance: Quantity,
    count_under_chance: Quantity,
    notice_chance: Quantity,
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
        initial_confidence: number("initial_confidence", &file.initial_confidence)?,
        confidence_decay_per_pulse: number(
            "confidence_decay_per_pulse",
            &file.confidence_decay_per_pulse,
        )?,
        drop_below_confidence: number("drop_below_confidence", &file.drop_below_confidence)?,
        merge_distance_hexes: file.merge_distance_hexes,
        merge_max_age_pulses: file.merge_max_age_pulses,
        position_error_chance: number("position_error_chance", &file.position_error_chance)?,
        count_over_chance: number("count_over_chance", &file.count_over_chance)?,
        count_under_chance: number("count_under_chance", &file.count_under_chance)?,
        notice_chance: number("notice_chance", &file.notice_chance)?,
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
    alert_contact_max_age_pulses: u32,
    alarm_chance: Quantity,
    gun_hit_chance_night: Quantity,
    gun_hit_chance_day: Quantity,
    torpedo_hit_chance_night: Quantity,
    torpedo_hit_chance_day: Quantity,
    surprised_target_factor: Quantity,
    lit_target_factor: Quantity,
    gun_hit_flotation: Quantity,
    gun_hit_fire: Quantity,
    gun_system_hit_chance: Quantity,
    torpedo_system_hit_chance: Quantity,
    degraded_factor: Quantity,
    fire_control_out_factor: Quantity,
    crippled_flotation: Quantity,
    claimed_sunk_fire: Quantity,
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
        alert_contact_max_age_pulses: file.alert_contact_max_age_pulses,
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
        gun_hit_fire: number("gun_hit_fire", &file.gun_hit_fire)?,
        gun_system_hit_chance: number("gun_system_hit_chance", &file.gun_system_hit_chance)?,
        torpedo_system_hit_chance: number(
            "torpedo_system_hit_chance",
            &file.torpedo_system_hit_chance,
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

    const AIRCRAFT_HEADER: &str = "id,name,role,search_range_nm,sweep_width_nm\n";

    #[test]
    fn aircraft_types_parse() {
        let types = parse_aircraft_types(
            &format!("# c\n{AIRCRAFT_HEADER}1, Flying boat ,patrol,600,\"40.5\"\n"),
            here(),
        )
        .expect("parses");
        let one = &types[&AircraftTypeId::new(1)];
        assert_eq!(one.name, "Flying boat");
        assert_eq!(one.role, AircraftRole::Patrol);
        assert_eq!(one.search_range_nm, Fixed::from_int(600));
        assert_eq!(one.sweep_width_nm, "40.5".parse::<Fixed>().expect("fixed"));
    }

    #[test]
    fn aircraft_type_errors_are_rejected() {
        let bad = |rows: &str| {
            parse_aircraft_types(&format!("{AIRCRAFT_HEADER}{rows}"), here())
                .expect_err("rejected")
                .message
        };
        assert!(bad("1,A,fighter,600,40\n").contains("unknown role"));
        assert!(bad("1,A,patrol,600,40\n1,B,patrol,600,40\n").contains("appears twice"));
        assert!(bad("1,A,patrol,0,40\n").contains("search range"));
        assert!(bad("1,A,patrol,600,x\n").contains("sweep_width_nm"));
        let extra = parse_aircraft_types(
            "id,name,role,search_range_nm,sweep_width_nm,extra\n1,A,patrol,600,40,9\n",
            here(),
        )
        .expect_err("rejected");
        assert!(extra.message.contains("extra"), "{extra}");
    }

    fn search_text() -> String {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/reference/search.toml");
        fs::read_to_string(path).expect("the shipped file reads")
    }

    #[test]
    fn search_figures_parse_with_the_files_values() {
        let figures = parse_search_figures(&search_text(), here()).expect("parses");
        assert_eq!(figures.initial_confidence, Fixed::from_ratio(8, 10));
        assert_eq!(figures.merge_distance_hexes, 2);
        assert_eq!(figures.merge_max_age_pulses, 2);
        let cruiser = &figures.kind_confusion[&ShipKind::Cruiser];
        assert_eq!(cruiser[&ShipKind::Cruiser], Fixed::from_ratio(7, 10));
        assert_eq!(cruiser[&ShipKind::Battleship], Fixed::from_ratio(1, 10));
    }

    #[test]
    fn a_float_in_search_figures_is_rejected() {
        let text = search_text().replace("\"0.8\"", "0.8");
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

    const CLASS_HEADER: &str = "id,name,kind,cruise_speed_kn,max_speed_kn,fuel_capacity_t,endurance_nm,cargo_capacity_t,main_guns,gun_rounds_per_min,gun_rounds_per_gun,gun_damage,torpedo_tubes,torpedo_damage,toughness,radar\n";

    fn classes(rows: &str) -> Result<RefData, DataError> {
        parse_ship_classes(&format!("# c\n{CLASS_HEADER}{rows}"), here())
    }

    #[test]
    fn ship_classes_parse_with_their_combat_columns() {
        let data = classes(
            "1,Cruiser,cruiser,16,32,2000,8000,0,8,3,100,20,8,200,400,1\n\
             2,Cargo,transport,10,12,1000,10000,3000,0,0,0,0,0,0,300,0\n",
        )
        .expect("parses");
        let cruiser = &data.ship_classes[&ShipClassId::new(1)];
        assert_eq!(cruiser.main_guns, 8);
        assert_eq!(cruiser.gun_rounds_per_min, Fixed::from_int(3));
        assert_eq!(cruiser.gun_rounds_per_gun, 100);
        assert_eq!(cruiser.gun_damage, Fixed::from_int(20));
        assert_eq!(cruiser.torpedo_tubes, 8);
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
    fn gun_rate_of_fire_may_be_a_decimal() {
        let data =
            classes("1,A,cruiser,16,32,2000,8000,0,8,3.5,100,20,8,200,400,0\n").expect("parses");
        let rate = data.ship_classes[&ShipClassId::new(1)].gun_rounds_per_min;
        assert_eq!(rate, "3.5".parse::<Fixed>().expect("fixed"));
        assert_eq!(rate * 2, Fixed::from_int(7));
    }

    #[test]
    fn damage_and_toughness_may_be_decimals() {
        let data = classes("1,A,cruiser,16,32,2000,8000,0,9,3.5,100,19.666666,8,196,405.44,0\n")
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
        assert!(bad("1,A,cruiser,16,32,2000,8000,0,8,3,100,20,8,200,0,0\n").contains("toughness"));
        // Radar must be 0 or 1.
        assert!(
            bad("1,A,cruiser,16,32,2000,8000,0,8,3,100,20,8,200,400,2\n").contains("radar is 2")
        );
        // Toughness must be above zero, and damage may not be negative.
        assert!(
            bad("1,A,cruiser,16,32,2000,8000,0,8,3,100,20,8,200,0.0,0\n").contains("toughness")
        );
        assert!(classes("1,A,cruiser,16,32,2000,8000,0,8,3,100,-20,8,200,400,0\n").is_err());
        assert!(classes("1,A,cruiser,16,32,2000,8000,0,8,3,100,20,8,200,1e2,0\n").is_err());
        // Whole numbers only for counts: a fraction, a negative and text are rejected.
        assert!(classes("1,A,cruiser,16,32,2000,8000,0,8.5,3,100,20,8,200,400,0\n").is_err());
        assert!(classes("1,A,cruiser,16,32,2000,8000,0,-8,3,100,20,8,200,400,0\n").is_err());
        assert!(classes("1,A,cruiser,16,32,2000,8000,0,8,3,100,20,8,200,400,yes\n").is_err());
        // A missing column and an extra one.
        let short = "id,name,kind,cruise_speed_kn,max_speed_kn,fuel_capacity_t,endurance_nm,cargo_capacity_t\n1,A,cruiser,16,32,2000,8000,0\n";
        assert!(parse_ship_classes(short, here()).is_err());
        let extra = format!(
            "{}\n1,A,cruiser,16,32,2000,8000,0,8,3,100,20,8,200,400,0,9\n",
            CLASS_HEADER.trim_end().to_string() + ",extra"
        );
        let message = parse_ship_classes(&extra, here())
            .expect_err("rejected")
            .message;
        assert!(message.contains("extra"), "{message}");
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
        assert_eq!(figures.alert_contact_max_age_pulses, 2);
        assert_eq!(figures.alarm_chance, Fixed::from_ratio(1, 2));
        assert_eq!(figures.gun_hit_chance_night, Fixed::from_ratio(3, 100));
        assert_eq!(figures.gun_hit_chance_day, Fixed::from_ratio(5, 100));
        assert_eq!(figures.torpedo_hit_chance_night, Fixed::from_ratio(4, 100));
        assert_eq!(figures.torpedo_hit_chance_day, Fixed::from_ratio(2, 100));
        assert_eq!(figures.surprised_target_factor, Fixed::from_ratio(3, 2));
        assert_eq!(figures.lit_target_factor, Fixed::from_int(2));
        assert_eq!(figures.gun_hit_flotation, Fixed::from_ratio(3, 4));
        assert_eq!(figures.gun_hit_fire, Fixed::from_ratio(1, 4));
        assert_eq!(figures.gun_system_hit_chance, Fixed::from_ratio(3, 10));
        assert_eq!(figures.torpedo_system_hit_chance, Fixed::from_ratio(1, 2));
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
        assert!(bad("gun_hit_fire = \"0.25\"", "gun_hit_fire = \"0.3\"").contains("not 1"));
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
}
