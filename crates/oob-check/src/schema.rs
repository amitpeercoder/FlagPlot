//! Which files make up the data set, and the columns the rules read.

use crate::table::Data;
use crate::Problem;

/// Entity type and the identity file that lists its keys.
pub const ENTITY_FILES: [(&str, &str); 6] = [
    ("ship_class", "ship_classes.csv"),
    ("ship", "ships.csv"),
    ("aircraft_type", "aircraft_types.csv"),
    ("weapon", "weapons.csv"),
    ("sensor", "sensors.csv"),
    ("air_unit", "air_units.csv"),
];

/// Identity files that are not an entity type a fact can be about.
pub const OTHER_IDENTITY_FILES: [&str; 3] = ["sources.csv", "places.csv", "engagements.csv"];

/// Facts file and the entity type its rows are about.
pub const FACT_FILES: [(&str, &str); 6] = [
    ("ship_class_facts.csv", "ship_class"),
    ("ship_facts.csv", "ship"),
    ("aircraft_type_facts.csv", "aircraft_type"),
    ("weapon_facts.csv", "weapon"),
    ("sensor_facts.csv", "sensor"),
    ("air_unit_facts.csv", "air_unit"),
];

pub const ENGAGEMENT_FILES: [&str; 4] = [
    "engagement_forces.csv",
    "engagement_events.csv",
    "engagement_positions.csv",
    "engagement_outcomes.csv",
];

/// The columns every sourced row has.
const SOURCE_COLUMNS: [&str; 7] = [
    "source",
    "locator",
    "accessed",
    "confidence",
    "basis",
    "standing",
    "note",
];

const FACT_COLUMNS: [&str; 6] = [
    "id",
    "entity",
    "attribute",
    "value",
    "valid_from",
    "valid_to",
];

/// Known files that may be absent. If present they are held to their
/// columns and rules. `oob-derive` runs this checker over small data sets of
/// its own that have no passages or training statements, so these files are
/// not yet required.
const OPTIONAL_FILES: [&str; 4] = [
    "passages.csv",
    "training_facts.csv",
    "lift_entities.csv",
    "troop_lift_facts.csv",
];

/// The identity file for an entity type, if the type is known.
pub fn entity_file(entity_type: &str) -> Option<&'static str> {
    ENTITY_FILES
        .iter()
        .find(|(t, _)| *t == entity_type)
        .map(|(_, f)| *f)
}

pub fn is_identity_file(file: &str) -> bool {
    ENTITY_FILES.iter().any(|(_, f)| *f == file) || OTHER_IDENTITY_FILES.contains(&file)
}

/// File name, the columns the rules read, and whether its rows are sourced.
fn required() -> Vec<(&'static str, Vec<&'static str>, bool)> {
    let mut files = vec![
        ("sources.csv", vec!["id", "key", "reliability"], false),
        ("places.csv", vec!["id", "key"], false),
        (
            "engagements.csv",
            vec!["id", "key", "date_from", "date_to"],
            false,
        ),
        ("ship_classes.csv", vec!["id", "key"], false),
        ("ships.csv", vec!["id", "key", "class"], false),
        ("aircraft_types.csv", vec!["id", "key"], false),
        ("weapons.csv", vec!["id", "key"], false),
        ("sensors.csv", vec!["id", "key"], false),
        ("air_units.csv", vec!["id", "key", "parent"], false),
        (
            "attributes.csv",
            vec!["key", "applies_to", "value_type", "cardinality"],
            false,
        ),
        ("gaps.csv", vec![], false),
        (
            "fits.csv",
            vec![
                "id",
                "carrier_type",
                "carrier",
                "slot",
                "item_type",
                "item",
                "count",
                "valid_from",
                "valid_to",
            ],
            true,
        ),
        (
            "engagement_forces.csv",
            vec!["id", "engagement", "member_type", "member"],
            true,
        ),
        (
            "engagement_events.csv",
            vec![
                "id",
                "engagement",
                "time",
                "range_m",
                "bearing_deg",
                "value",
            ],
            true,
        ),
        (
            "engagement_positions.csv",
            vec![
                "id",
                "engagement",
                "time",
                "lat",
                "lon",
                "bearing_deg",
                "distance_nm",
                "course_deg",
                "speed_kn",
            ],
            true,
        ),
        (
            "engagement_outcomes.csv",
            vec!["id", "engagement", "value"],
            true,
        ),
        (
            "passages.csv",
            crate::passages::COLUMNS[..crate::passages::OWN_COLUMNS].to_vec(),
            true,
        ),
        (
            "training_facts.csv",
            crate::training::COLUMNS[..crate::training::OWN_COLUMNS].to_vec(),
            true,
        ),
        (
            "lift_entities.csv",
            crate::lift::ENTITY_COLUMNS.to_vec(),
            false,
        ),
        (
            "troop_lift_facts.csv",
            crate::lift::COLUMNS[..crate::lift::OWN_COLUMNS].to_vec(),
            true,
        ),
    ];
    for (file, _) in FACT_FILES {
        files.push((file, FACT_COLUMNS.to_vec(), true));
    }
    files
}

/// Reports every known file that is missing or lacks a column a rule reads.
/// A file with a `standing` column that the list does not know is held to
/// the source columns too.
pub fn check(data: &Data, out: &mut Vec<Problem>) {
    let known = required();
    for (file, columns, sourced) in &known {
        let Some(table) = data.tables.get(*file) else {
            if !OPTIONAL_FILES.contains(file) {
                out.push(Problem::file(file, "file is missing"));
            }
            continue;
        };
        let source_columns: &[&str] = if *sourced { &SOURCE_COLUMNS } else { &[] };
        for column in columns.iter().chain(source_columns) {
            if !table.has(column) {
                out.push(Problem::file(file, format!("no `{column}` column")));
            }
        }
    }
    for (file, table) in &data.tables {
        if table.has("standing") && !known.iter().any(|(f, _, _)| f == file) {
            for column in SOURCE_COLUMNS {
                if !table.has(column) {
                    out.push(Problem::file(file, format!("no `{column}` column")));
                }
            }
        }
    }
}
