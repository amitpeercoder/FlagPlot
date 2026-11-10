use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde::Deserialize;
use sim_core::fixed::Fixed;
use sim_core::ids::{AircraftTypeId, ShipClassId};
use sim_core::refdata::{
    AircraftRole, AircraftType, PortSize, RefData, SearchFigures, ShipClass, ShipKind,
};

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
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PortSizeRow {
    port_size: i64,
    berths: u32,
    berth_rate_t_per_h: String,
    lighter_rate_t_per_h: String,
}

fn parse_kind(text: &str) -> Option<ShipKind> {
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
}
