//! One failing example per rule, against a small data set held in memory.
//!
//! `base()` is a complete data set with no problems. Each test adds or
//! replaces a few rows and names the problem it expects.

use std::collections::BTreeMap;

const FACT_HEADER: &str = "id,entity,attribute,value,valid_from,valid_to,\
source,locator,accessed,confidence,basis,standing,note\n";

/// The source columns of a row: source, locator, accessed, confidence,
/// basis `stated`, standing, and a note.
fn src(source: &str, confidence: &str, standing: &str) -> String {
    format!("{source},p.1,2026-11-10,{confidence},stated,{standing},a note")
}

const PASSAGE_HEADER: &str = "id,subject,ships,from_place,from_lat,from_lon,depart_time,\
to_place,to_lat,to_lon,arrive_time,zone,speed_kn,speed_note,distance_nm_stated,steady,flags,\
as_stated,source,locator,accessed,confidence,basis,standing,note\n";

fn base() -> BTreeMap<String, String> {
    let ok = src("a", "medium", "preferred");
    let files = [
        (
            "sources.csv",
            "# Comment lines come before the header.\n# Two of them.\n\
             id,key,reliability\n1000,a,A\n1001,b,B\n1002,c,C\n1003,c2,C\n"
                .to_string(),
        ),
        ("places.csv", "id,key\n1000,rabaul\n".to_string()),
        (
            "engagements.csv",
            "id,key,date_from,date_to\n1000,savo,1942-08-08,1942-08-09\n".to_string(),
        ),
        ("ship_classes.csv", "id,key\n1000,cls\n".to_string()),
        (
            "ships.csv",
            "id,key,class\n1000,shp,cls\n1001,tug,\n".to_string(),
        ),
        ("aircraft_types.csv", "id,key\n1000,plane\n".to_string()),
        ("weapons.csv", "id,key\n1000,gun\n".to_string()),
        ("sensors.csv", "id,key\n1000,radar\n".to_string()),
        (
            "air_units.csv",
            "id,key,parent\n1000,wing,\n1001,sqn,wing\n".to_string(),
        ),
        (
            "attributes.csv",
            "key,applies_to,value_type,cardinality\n\
             speed,ship_class,decimal,one\n\
             crew,ship_class,integer,one\n\
             speed,aircraft_type,decimal,one\n\
             sunk,ship,date,one\n\
             fate,ship,text,one\n\
             alias,ship,text,many\n\
             ship_class,ship,ref:ship_class,one\n\
             equipped_with,air_unit,ref:aircraft_type,many\n\
             based_at,air_unit,ref:place_or_ship,many\n\
             calibre,weapon,decimal,one\n\
             power,sensor,decimal,one\n"
                .to_string(),
        ),
        (
            "gaps.csv",
            "entity_type,entity,what\nship,shp,radar\n".to_string(),
        ),
        (
            "ship_class_facts.csv",
            format!("{FACT_HEADER}1,cls,speed,30,,,{ok}\n"),
        ),
        ("ship_facts.csv", FACT_HEADER.to_string()),
        ("aircraft_type_facts.csv", FACT_HEADER.to_string()),
        ("weapon_facts.csv", FACT_HEADER.to_string()),
        ("sensor_facts.csv", FACT_HEADER.to_string()),
        ("air_unit_facts.csv", FACT_HEADER.to_string()),
        (
            "fits.csv",
            format!(
                "id,carrier_type,carrier,slot,item_type,item,count,valid_from,valid_to,\
                 source,locator,accessed,confidence,basis,standing,note\n\
                 1,ship_class,cls,main_battery,weapon,gun,8,,,{ok}\n"
            ),
        ),
        (
            "engagement_forces.csv",
            format!(
                "id,engagement,member_type,member,\
                 source,locator,accessed,confidence,basis,standing,note\n\
                 1,savo,ship,shp,{ok}\n"
            ),
        ),
        (
            "engagement_events.csv",
            format!(
                "id,engagement,time,range_m,bearing_deg,value,\
                 source,locator,accessed,confidence,basis,standing,note\n\
                 1,savo,1942-08-09T01:43,5000,045,4,{ok}\n"
            ),
        ),
        (
            "engagement_positions.csv",
            format!(
                "id,engagement,time,lat,lon,bearing_deg,distance_nm,course_deg,speed_kn,\
                 source,locator,accessed,confidence,basis,standing,note\n\
                 1,savo,,-9.1,159.5,,,040,12.5,{ok}\n"
            ),
        ),
        (
            "engagement_outcomes.csv",
            format!(
                "id,engagement,value,\
                 source,locator,accessed,confidence,basis,standing,note\n\
                 1,savo,sunk by gunfire,{ok}\n"
            ),
        ),
        (
            "passages.csv",
            format!("{PASSAGE_HEADER}1,run,shp|a formation,rabaul,,,1942-08-09T06:00,,-9.1,159.5,1942-08-10,,12.5,,300,no,,as stated,{ok}\n"),
        ),
    ];
    files
        .into_iter()
        .map(|(name, text)| (name.to_string(), text))
        .collect()
}

/// Checks `files` and returns each problem as `file|row|message`.
fn problems_in(files: &BTreeMap<String, String>) -> Vec<String> {
    oob_check::check_files(files)
        .problems
        .iter()
        .map(|p| format!("{}|{}|{}", p.file, p.row, p.message))
        .collect()
}

/// The base data set with `lines` added to the end of `file`.
fn adding(file: &str, lines: &[String]) -> Vec<String> {
    let mut files = base();
    let text = files.get_mut(file).expect("a file of the base set");
    for line in lines {
        text.push_str(line);
        text.push('\n');
    }
    problems_in(&files)
}

fn adding_one(file: &str, line: &str) -> Vec<String> {
    adding(file, &[line.to_string()])
}

/// Asserts that exactly `count` problems were found and that each of
/// `expected` (`file|row|` followed by part of the message) is among them.
#[track_caller]
fn assert_problems(found: &[String], count: usize, expected: &[&str]) {
    for want in expected {
        let (prefix, text) = want.rsplit_once('|').expect("file|row|text");
        assert!(
            found
                .iter()
                .any(|p| p.starts_with(&format!("{prefix}|")) && p.contains(text)),
            "expected `{want}` among {found:#?}"
        );
    }
    assert_eq!(found.len(), count, "{found:#?}");
}

#[track_caller]
fn assert_clean(found: &[String]) {
    assert!(found.is_empty(), "{found:#?}");
}

#[test]
fn base_data_set_is_clean() {
    assert_clean(&problems_in(&base()));
}

#[test]
fn problems_are_sorted_by_file_then_line() {
    let mut files = base();
    files.get_mut("weapons.csv").unwrap().push_str("1000,Gun\n");
    files
        .get_mut("places.csv")
        .unwrap()
        .push_str("1000,Rabaul\n1000,x\n");
    let report = oob_check::check_files(&files);
    let mut sorted = report.problems.clone();
    sorted.sort();
    assert_eq!(report.problems, sorted);
    assert_eq!(report.problems[0].file, "places.csv");
    assert_eq!(report.problems.last().unwrap().file, "weapons.csv");
    assert!(report.problems[0]
        .to_string()
        .starts_with("places.csv:3: id 1000: "));
}

// Rule 1: files parse, ids and keys.

#[test]
fn rule1_row_with_wrong_field_count() {
    let found = adding_one("places.csv", "1001,buka,extra");
    assert_problems(&found, 1, &["places.csv|line 3|row has 3 fields"]);
    let found = adding_one("gaps.csv", "ship,shp");
    assert_problems(&found, 1, &["gaps.csv|line 3|row has 2 fields"]);
}

#[test]
fn rule1_missing_file_or_column() {
    let mut files = base();
    files.remove("fits.csv");
    assert_problems(&problems_in(&files), 1, &["fits.csv||file is missing"]);

    let mut files = base();
    files.insert("ships.csv".to_string(), "id,key\n1000,shp\n".to_string());
    assert_problems(&problems_in(&files), 1, &["ships.csv||no `class` column"]);

    let mut files = base();
    files.insert("places.csv".to_string(), "# only a comment\n".to_string());
    let found = problems_in(&files);
    assert!(
        found.contains(&"places.csv||no header row".to_string()),
        "{found:#?}"
    );
}

#[test]
fn rule1_duplicate_or_malformed_id() {
    let found = adding_one("places.csv", "1000,buka");
    assert_problems(&found, 1, &["places.csv|id 1000|duplicate id"]);
    let found = adding_one("places.csv", "x7,buka");
    assert_problems(&found, 1, &["places.csv|id x7|not a whole number"]);
    let dup = format!("1,cls,crew,300,,,{}", src("a", "medium", "preferred"));
    let found = adding_one("ship_class_facts.csv", &dup);
    assert_problems(&found, 1, &["ship_class_facts.csv|id 1|duplicate id"]);
}

#[test]
fn rule1_duplicate_or_malformed_key() {
    let found = adding_one("places.csv", "1001,rabaul");
    assert_problems(&found, 1, &["places.csv|id 1001|duplicate key `rabaul`"]);
    for bad in ["Buka", "-buka", "buka_island", "buka island", ""] {
        let found = adding_one("places.csv", &format!("1001,{bad}"));
        assert_problems(&found, 1, &["places.csv|id 1001|must be lower-case"]);
    }
    assert_clean(&adding_one("places.csv", "1001,5th-base.2"));
}

#[test]
fn rule1_attribute_is_unique_per_key_and_entity_type() {
    // The base set already has `speed` for two entity types, which is fine.
    let found = adding_one("attributes.csv", "speed,ship_class,decimal,one");
    assert_problems(
        &found,
        1,
        &["attributes.csv|key speed|duplicate attribute `speed` for `ship_class`"],
    );
    let found = adding_one("attributes.csv", "range,torpedo,float,several");
    assert_problems(
        &found,
        3,
        &[
            "attributes.csv|key range|unknown entity type `torpedo`",
            "attributes.csv|key range|unknown value type `float`",
            "attributes.csv|key range|cardinality `several`",
        ],
    );
}

// Rule 2: references resolve.

#[test]
fn rule2_fact_entity_and_source() {
    let row = format!("2,nobody,crew,300,,,{}", src("a", "medium", "preferred"));
    let found = adding_one("ship_class_facts.csv", &row);
    assert_problems(
        &found,
        1,
        &["ship_class_facts.csv|id 2|unknown entity `nobody`"],
    );

    // A ship key is not a ship class key.
    let row = format!("2,shp,crew,300,,,{}", src("a", "medium", "preferred"));
    let found = adding_one("ship_class_facts.csv", &row);
    assert_problems(
        &found,
        1,
        &["ship_class_facts.csv|id 2|unknown entity `shp`"],
    );

    let row = format!("2,cls,crew,300,,,{}", src("nowhere", "medium", "preferred"));
    let found = adding_one("ship_class_facts.csv", &row);
    assert_problems(
        &found,
        1,
        &["ship_class_facts.csv|id 2|unknown source `nowhere`"],
    );
}

#[test]
fn rule2_fit_carrier_and_item() {
    let ok = src("a", "medium", "preferred");
    let found = adding_one(
        "fits.csv",
        &format!("2,ship,cls,sonar,sensor,radar,1,,,{ok}"),
    );
    assert_problems(&found, 1, &["fits.csv|id 2|unknown carrier `cls` (ship)"]);
    let found = adding_one("fits.csv", &format!("2,ship,shp,sonar,sensor,gun,1,,,{ok}"));
    assert_problems(&found, 1, &["fits.csv|id 2|unknown item `gun` (sensor)"]);
    let found = adding_one(
        "fits.csv",
        &format!("2,fleet,shp,sonar,sensor,radar,1,,,{ok}"),
    );
    assert_problems(&found, 1, &["fits.csv|id 2|unknown carrier type `fleet`"]);
    assert_clean(&adding_one(
        "fits.csv",
        &format!("2,ship,shp,aircraft,aircraft_type,plane,2,,,{ok}"),
    ));
}

#[test]
fn rule2_ship_class_and_air_unit_parent() {
    // The base set has a ship with an empty class and a unit with no parent.
    let found = adding_one("ships.csv", "1002,boat,no-class");
    assert_problems(&found, 1, &["ships.csv|id 1002|unknown class `no-class`"]);
    let found = adding_one("air_units.csv", "1002,flt,no-parent");
    assert_problems(
        &found,
        1,
        &["air_units.csv|id 1002|unknown parent `no-parent`"],
    );
}

#[test]
fn rule2_engagement_and_force_member() {
    let ok = src("a", "medium", "preferred");
    let found = adding_one("engagement_outcomes.csv", &format!("2,midway,3,{ok}"));
    assert_problems(
        &found,
        1,
        &["engagement_outcomes.csv|id 2|unknown engagement `midway`"],
    );
    let found = adding_one("engagement_forces.csv", &format!("2,savo,ship,sqn,{ok}"));
    assert_problems(
        &found,
        1,
        &["engagement_forces.csv|id 2|unknown member `sqn` (ship)"],
    );
    let found = adding_one("engagement_forces.csv", &format!("2,savo,army,sqn,{ok}"));
    assert_problems(
        &found,
        1,
        &["engagement_forces.csv|id 2|unknown member type `army`"],
    );
    assert_clean(&adding_one(
        "engagement_forces.csv",
        &format!("2,savo,air_unit,sqn,{ok}"),
    ));
}

#[test]
fn rule2_reference_values() {
    let ok = src("a", "medium", "preferred");
    let found = adding_one("ship_facts.csv", &format!("1,shp,ship_class,shp,,,{ok}"));
    assert_problems(
        &found,
        1,
        &["ship_facts.csv|id 1|not a valid ref:ship_class"],
    );
    let found = adding_one(
        "air_unit_facts.csv",
        &format!("1,sqn,equipped_with,gun,,,{ok}"),
    );
    assert_problems(
        &found,
        1,
        &["air_unit_facts.csv|id 1|not a valid ref:aircraft_type"],
    );
    let found = adding_one("air_unit_facts.csv", &format!("1,sqn,based_at,cls,,,{ok}"));
    assert_problems(
        &found,
        1,
        &["air_unit_facts.csv|id 1|not a valid ref:place_or_ship"],
    );
    // A place or a ship will do.
    assert_clean(&adding(
        "air_unit_facts.csv",
        &[
            format!("1,sqn,based_at,rabaul,,,{ok}"),
            format!("2,sqn,based_at,shp,,,{ok}"),
            format!("3,sqn,equipped_with,plane,,,{ok}"),
        ],
    ));
}

// Rule 3: attributes and value forms.

#[test]
fn rule3_attribute_must_apply_to_the_entity_type() {
    let ok = src("a", "medium", "preferred");
    // `calibre` is a weapon attribute, not a ship class one.
    let found = adding_one("ship_class_facts.csv", &format!("2,cls,calibre,203,,,{ok}"));
    assert_problems(
        &found,
        1,
        &["ship_class_facts.csv|id 2|`calibre` is not in attributes.csv for `ship_class`"],
    );
}

#[test]
fn rule3_value_must_have_its_declared_type() {
    let ok = src("a", "medium", "preferred");
    let cases = [
        (
            "ship_class_facts.csv",
            "cls,crew,300.5",
            "not a valid integer",
        ),
        (
            "ship_class_facts.csv",
            "cls,crew,\"1,000\"",
            "not a valid integer",
        ),
        ("ship_class_facts.csv", "cls,crew,", "not a valid integer"),
        (
            "weapon_facts.csv",
            "gun,calibre,8 in",
            "not a valid decimal",
        ),
        (
            "weapon_facts.csv",
            "gun,calibre,203.",
            "not a valid decimal",
        ),
        (
            "ship_facts.csv",
            "shp,sunk,9 August 1942",
            "not a valid date",
        ),
        ("ship_facts.csv", "shp,sunk,1942-13-01", "not a valid date"),
        ("ship_facts.csv", "shp,sunk,1942-8-9", "not a valid date"),
        ("ship_facts.csv", "shp,fate,", "not a valid text"),
    ];
    for (file, fields, message) in cases {
        let found = adding_one(file, &format!("7,{fields},,,{ok}"));
        assert_problems(&found, 1, &[&format!("{file}|id 7|{message}")]);
    }
    assert_clean(&adding(
        "ship_facts.csv",
        &[
            format!("7,shp,sunk,1942,,,{ok}"),
            format!("8,tug,sunk,1942-08,,,{ok}"),
            format!("9,tug,fate,Scuttled,,,{ok}"),
        ],
    ));
}

#[test]
fn rule3_no_exponent_floats() {
    let ok = src("a", "medium", "preferred");
    let found = adding_one("weapon_facts.csv", &format!("1,gun,calibre,2.03e2,,,{ok}"));
    assert_problems(
        &found,
        2,
        &[
            "weapon_facts.csv|id 1|looks like a float with an exponent",
            "weapon_facts.csv|id 1|not a valid decimal",
        ],
    );
    // Not even where the attribute is text.
    let found = adding_one("ship_facts.csv", &format!("1,shp,fate,1E5,,,{ok}"));
    assert_problems(
        &found,
        1,
        &["ship_facts.csv|id 1|looks like a float with an exponent"],
    );
    let found = adding_one("engagement_outcomes.csv", &format!("2,savo,1e3,{ok}"));
    assert_problems(
        &found,
        1,
        &["engagement_outcomes.csv|id 2|looks like a float"],
    );
}

#[test]
fn rule3_valid_from_and_valid_to_are_dates() {
    let ok = src("a", "medium", "preferred");
    let row = format!("2,cls,crew,300,Aug 1942,1942-09-31x,{ok}");
    let found = adding_one("ship_class_facts.csv", &row);
    assert_problems(
        &found,
        2,
        &[
            "ship_class_facts.csv|id 2|valid_from `Aug 1942` is not a date",
            "ship_class_facts.csv|id 2|valid_to `1942-09-31x` is not a date",
        ],
    );
    let row = format!("2,ship,shp,sonar,sensor,radar,1,08/1942,,{ok}");
    let found = adding_one("fits.csv", &row);
    assert_problems(
        &found,
        1,
        &["fits.csv|id 2|valid_from `08/1942` is not a date"],
    );
}

#[test]
fn rule3_engagement_time_and_numbers() {
    let ok = src("a", "medium", "preferred");
    for bad in [
        "01:43",
        "1942-08-09 01:43",
        "1942-08-09T25:00",
        "1942-08-09T01:43:00",
    ] {
        let found = adding_one("engagement_events.csv", &format!("2,savo,{bad},,,,{ok}"));
        assert_problems(&found, 1, &["engagement_events.csv|id 2|time"]);
    }
    // A bare date is allowed.
    assert_clean(&adding_one(
        "engagement_events.csv",
        &format!("2,savo,1942-08-09,,,,{ok}"),
    ));
    let found = adding_one(
        "engagement_events.csv",
        &format!("2,savo,,\"5,000\",NNE,,{ok}"),
    );
    assert_problems(
        &found,
        2,
        &[
            "engagement_events.csv|id 2|range_m `5,000` is not a decimal",
            "engagement_events.csv|id 2|bearing_deg `NNE` is not a decimal",
        ],
    );
    let found = adding_one(
        "engagement_positions.csv",
        &format!("2,savo,,9-50S,159E,,5 nm,NE,12 kn,{ok}"),
    );
    assert_problems(
        &found,
        5,
        &[
            "engagement_positions.csv|id 2|lat",
            "engagement_positions.csv|id 2|lon",
            "engagement_positions.csv|id 2|distance_nm",
            "engagement_positions.csv|id 2|course_deg",
            "engagement_positions.csv|id 2|speed_kn",
        ],
    );
}

#[test]
fn rule3_fit_count() {
    let ok = src("a", "medium", "preferred");
    for bad in ["-1", "2.5", "two"] {
        let row = format!("2,ship,shp,sonar,sensor,radar,{bad},,,{ok}");
        let found = adding_one("fits.csv", &row);
        assert_problems(&found, 1, &["fits.csv|id 2|is not a whole number"]);
    }
    // Empty and zero are both allowed: unknown, and "none fitted".
    assert_clean(&adding(
        "fits.csv",
        &[
            format!("2,ship,shp,sonar,sensor,radar,,,,{ok}"),
            format!("3,ship,tug,sonar,sensor,radar,0,,,{ok}"),
        ],
    ));
}

// Rule 4: sourced rows.

#[test]
fn rule4_source_locator_and_date_read() {
    let found = adding_one(
        "ship_class_facts.csv",
        "2,cls,crew,300,,,,,,medium,stated,preferred,",
    );
    assert_problems(
        &found,
        3,
        &[
            "ship_class_facts.csv|id 2|no source",
            "ship_class_facts.csv|id 2|no locator",
            "ship_class_facts.csv|id 2|accessed `` is not a date",
        ],
    );
    let found = adding_one(
        "engagement_outcomes.csv",
        "2,savo,3,a,p.1,last week,medium,stated,preferred,",
    );
    assert_problems(
        &found,
        1,
        &["engagement_outcomes.csv|id 2|accessed `last week`"],
    );
}

#[test]
fn rule4_grades() {
    let found = adding_one(
        "ship_class_facts.csv",
        "2,cls,crew,300,,,a,p.1,2026-11-10,certain,guessed,chosen,",
    );
    assert_problems(
        &found,
        3,
        &[
            "ship_class_facts.csv|id 2|unknown confidence `certain`",
            "ship_class_facts.csv|id 2|unknown basis `guessed`",
            "ship_class_facts.csv|id 2|unknown standing `chosen`",
        ],
    );
}

#[test]
fn rule4_grade_c_source_is_capped_at_low() {
    let row = format!("2,cls,crew,300,,,{}", src("c", "medium", "preferred"));
    let found = adding_one("ship_class_facts.csv", &row);
    assert_problems(&found, 1, &["ship_class_facts.csv|id 2|graded C"]);
    // Low is fine, with a second source so the standing rule is content.
    assert_clean(&adding(
        "ship_class_facts.csv",
        &[
            format!("2,cls,crew,300,,,{}", src("c", "low", "preferred")),
            format!("3,cls,crew,300,,,{}", src("b", "low", "preferred")),
        ],
    ));
}

#[test]
fn rule4_weak_rows_need_a_note() {
    let tail = "a,p.1,2026-11-10";
    let cases = [
        // Supported by a medium row beside it, so only the note is missing.
        vec![
            format!("2,cls,crew,300,,,{tail},low,stated,preferred,"),
            format!("3,cls,crew,300,,,{tail},medium,stated,preferred,"),
        ],
        vec![format!(
            "2,cls,crew,300,,,{tail},medium,inferred,preferred,"
        )],
        vec![
            format!("2,cls,crew,310,,,{tail},medium,stated,alternative,"),
            format!("3,cls,crew,300,,,{tail},medium,stated,preferred,"),
        ],
        vec![format!("2,cls,crew,300,,,{tail},low,stated,unconfirmed,")],
    ];
    for rows in cases {
        let found = adding("ship_class_facts.csv", &rows);
        assert_problems(&found, 1, &["ship_class_facts.csv|id 2|needs a note"]);
    }
    // A firm, stated, preferred row may leave the note empty.
    assert_clean(&adding_one(
        "ship_class_facts.csv",
        &format!("2,cls,crew,300,,,{tail},medium,stated,preferred,"),
    ));
}

// Rule 5: chosen rows of a single-valued attribute do not contradict.

fn crew(id: u32, value: &str, from: &str, to: &str, source: &str, standing: &str) -> String {
    format!(
        "{id},cls,crew,{value},{from},{to},{}",
        src(source, "medium", standing)
    )
}

#[test]
fn rule5_two_chosen_rows_with_different_values() {
    let found = adding(
        "ship_class_facts.csv",
        &[
            crew(2, "300", "", "", "a", "preferred"),
            crew(3, "310", "", "", "b", "preferred"),
        ],
    );
    assert_problems(
        &found,
        1,
        &["ship_class_facts.csv|id 2|preferred row and preferred row id 3 overlap"],
    );
}

#[test]
fn rule5_ranges_from_inclusive_to_exclusive() {
    // Consecutive ranges do not overlap.
    assert_clean(&adding(
        "ship_class_facts.csv",
        &[
            crew(2, "300", "", "1942-08-01", "a", "preferred"),
            crew(3, "310", "1942-08-01", "", "b", "preferred"),
        ],
    ));
    // One day more and they do.
    let found = adding(
        "ship_class_facts.csv",
        &[
            crew(2, "300", "", "1942-08-02", "a", "preferred"),
            crew(3, "310", "1942-08-01", "", "b", "preferred"),
        ],
    );
    assert_problems(&found, 1, &["ship_class_facts.csv|id 2|overlap"]);
}

#[test]
fn rule5_equal_from_and_to_is_a_point() {
    let found = adding(
        "ship_class_facts.csv",
        &[
            crew(2, "300", "1942-08-09", "1942-08-09", "a", "preferred"),
            crew(3, "310", "1942-08-01", "1942-09-01", "b", "preferred"),
        ],
    );
    assert_problems(&found, 1, &["ship_class_facts.csv|id 2|overlap"]);
    assert_clean(&adding(
        "ship_class_facts.csv",
        &[
            crew(2, "300", "1942-08-09", "1942-08-09", "a", "preferred"),
            crew(3, "310", "1942-08-10", "", "b", "preferred"),
        ],
    ));
}

#[test]
fn rule5_numbers_compare_as_numbers_and_text_without_case() {
    let ok = src("a", "medium", "preferred");
    let ok_b = src("b", "medium", "preferred");
    assert_clean(&adding(
        "aircraft_type_facts.csv",
        &[
            format!("1,plane,speed,100,,,{ok}"),
            format!("2,plane,speed,100.0,,,{ok_b}"),
        ],
    ));
    assert_clean(&adding(
        "ship_facts.csv",
        &[
            format!("1,shp,fate,Scuttled,,,{ok}"),
            format!("2,shp,fate, scuttled ,,,{ok_b}"),
        ],
    ));
}

#[test]
fn rule5_chosen_row_beside_a_disputed_row() {
    let found = adding(
        "ship_class_facts.csv",
        &[
            crew(2, "300", "", "", "a", "preferred"),
            crew(3, "310", "", "", "b", "disputed"),
        ],
    );
    assert_problems(
        &found,
        1,
        &["ship_class_facts.csv|id 2|preferred row overlaps disputed row id 3"],
    );
    // All disputed is the way to record it. So is preferred and alternative.
    assert_clean(&adding(
        "ship_class_facts.csv",
        &[
            crew(2, "300", "", "", "a", "disputed"),
            crew(3, "310", "", "", "b", "disputed"),
        ],
    ));
    assert_clean(&adding(
        "ship_class_facts.csv",
        &[
            crew(2, "300", "", "", "a", "preferred"),
            crew(3, "310", "", "", "b", "alternative"),
        ],
    ));
}

#[test]
fn rule5_does_not_apply_to_cardinality_many() {
    let ok = src("a", "medium", "preferred");
    assert_clean(&adding(
        "ship_facts.csv",
        &[
            format!("1,shp,alias,Old Name,,,{ok}"),
            format!("2,shp,alias,Other Name,,,{ok}"),
        ],
    ));
}

// Rule 6: a value has to earn `preferred`.

fn low_crew(id: u32, value: &str, source: &str, standing: &str) -> String {
    format!("{id},cls,crew,{value},,,{}", src(source, "low", standing))
}

#[test]
fn rule6_one_low_row_cannot_be_preferred() {
    let found = adding_one(
        "ship_class_facts.csv",
        &low_crew(2, "300", "b", "preferred"),
    );
    assert_problems(
        &found,
        1,
        &["ship_class_facts.csv|id 2|should be unconfirmed"],
    );
    assert_clean(&adding_one(
        "ship_class_facts.csv",
        &low_crew(2, "300", "b", "unconfirmed"),
    ));
}

#[test]
fn rule6_a_medium_row_cannot_be_unconfirmed() {
    let row = format!("2,cls,crew,300,,,{}", src("b", "medium", "unconfirmed"));
    let found = adding_one("ship_class_facts.csv", &row);
    assert_problems(
        &found,
        1,
        &["ship_class_facts.csv|id 2|should be preferred"],
    );
}

#[test]
fn rule6_two_agreeing_sources_one_not_graded_c() {
    assert_clean(&adding(
        "ship_class_facts.csv",
        &[
            low_crew(2, "300", "b", "preferred"),
            low_crew(3, "300", "c", "preferred"),
        ],
    ));
    // `100` and `100.0` are one value.
    assert_clean(&adding(
        "aircraft_type_facts.csv",
        &[
            format!("1,plane,speed,100,,,{}", src("b", "low", "preferred")),
            format!("2,plane,speed,100.0,,,{}", src("c", "low", "preferred")),
        ],
    ));
    let found = adding(
        "ship_class_facts.csv",
        &[
            low_crew(2, "300", "b", "unconfirmed"),
            low_crew(3, "300", "c", "unconfirmed"),
        ],
    );
    assert_problems(
        &found,
        2,
        &[
            "ship_class_facts.csv|id 2|should be preferred",
            "ship_class_facts.csv|id 3|should be preferred",
        ],
    );
}

#[test]
fn rule6_two_grade_c_sources_or_one_source_twice_is_not_enough() {
    let found = adding(
        "ship_class_facts.csv",
        &[
            low_crew(2, "300", "c", "preferred"),
            low_crew(3, "300", "c2", "preferred"),
        ],
    );
    assert_problems(
        &found,
        2,
        &[
            "ship_class_facts.csv|id 2|should be unconfirmed",
            "ship_class_facts.csv|id 3|should be unconfirmed",
        ],
    );
    let found = adding(
        "ship_class_facts.csv",
        &[
            low_crew(2, "300", "b", "preferred"),
            low_crew(3, "300", "b", "preferred"),
        ],
    );
    assert_problems(
        &found,
        2,
        &["ship_class_facts.csv|id 2|should be unconfirmed"],
    );
    // Sources that disagree do not support each other.
    let found = adding(
        "ship_class_facts.csv",
        &[
            low_crew(2, "300", "b", "disputed"),
            low_crew(3, "310", "a", "disputed"),
            low_crew(4, "300", "c", "preferred"),
        ],
    );
    assert_problems(
        &found,
        1,
        &["ship_class_facts.csv|id 4|overlaps disputed row id 3"],
    );
}

#[test]
fn rule6_support_counts_rows_of_any_standing() {
    // The disputed row from `b` agrees with the row from `c`, so the value
    // has two sources. Rule 5 still objects to the pair with id 3.
    let found = adding(
        "ship_class_facts.csv",
        &[
            low_crew(2, "300", "b", "disputed"),
            low_crew(3, "310", "a", "disputed"),
            low_crew(4, "300", "c", "unconfirmed"),
        ],
    );
    assert_problems(
        &found,
        2,
        &[
            "ship_class_facts.csv|id 4|should be preferred",
            "ship_class_facts.csv|id 4|overlaps disputed row id 3",
        ],
    );
}

#[test]
fn rule6_cardinality_many_groups_by_value_and_valid_from() {
    let alias = |id: u32, value: &str, from: &str, source: &str| {
        format!(
            "{id},shp,alias,{value},{from},,{}",
            src(source, "low", "unconfirmed")
        )
    };
    // Same value from different dates: two statements, each with one source.
    assert_clean(&adding(
        "ship_facts.csv",
        &[
            alias(1, "Old Name", "1942", "b"),
            alias(2, "Old Name", "1943", "c"),
        ],
    ));
    // Same value from the same date: one statement with two sources.
    let found = adding(
        "ship_facts.csv",
        &[
            alias(1, "Old Name", "1942", "b"),
            alias(2, "old name", "1942", "c"),
        ],
    );
    assert_problems(
        &found,
        2,
        &[
            "ship_facts.csv|id 1|should be preferred",
            "ship_facts.csv|id 2|should be preferred",
        ],
    );
}

#[test]
fn rule6_fits() {
    let fit = |id: u32, count: &str, source: &str, standing: &str| {
        format!(
            "{id},ship,shp,sonar,sensor,radar,{count},1942-08,,{}",
            src(source, "low", standing)
        )
    };
    let found = adding_one("fits.csv", &fit(2, "1", "b", "preferred"));
    assert_problems(&found, 1, &["fits.csv|id 2|should be unconfirmed"]);
    assert_clean(&adding(
        "fits.csv",
        &[fit(2, "1", "b", "preferred"), fit(3, "1", "c", "preferred")],
    ));
    // A different count is a different statement.
    let found = adding(
        "fits.csv",
        &[
            fit(2, "1", "b", "preferred"),
            fit(3, "2", "c", "unconfirmed"),
        ],
    );
    assert_problems(&found, 1, &["fits.csv|id 2|should be unconfirmed"]);
    let found = adding(
        "fits.csv",
        &[
            fit(2, "1", "b", "unconfirmed"),
            fit(3, "1", "c", "unconfirmed"),
        ],
    );
    assert_problems(&found, 2, &["fits.csv|id 3|should be preferred"]);
}

#[test]
fn rule6_low_engagement_row_cannot_be_preferred() {
    let low = src("b", "low", "preferred");
    let rows = [
        ("engagement_forces.csv", format!("2,savo,ship,tug,{low}")),
        ("engagement_events.csv", format!("2,savo,,,,,{low}")),
        ("engagement_positions.csv", format!("2,savo,,,,,,,,{low}")),
        ("engagement_outcomes.csv", format!("2,savo,3,{low}")),
    ];
    for (file, row) in rows {
        let found = adding_one(file, &row);
        assert_problems(&found, 1, &[&format!("{file}|id 2|cannot be preferred")]);
    }
    assert_clean(&adding_one(
        "engagement_outcomes.csv",
        &format!("2,savo,3,{}", src("b", "low", "unconfirmed")),
    ));
}

/// A passage row: the fields before the source columns, then a good source.
fn passage(fields: &str) -> String {
    format!("{fields},{}", src("a", "medium", "preferred"))
}

/// A good passage (id 2, 18 fields before the source columns) with the
/// fields at the given 0-based positions replaced.
fn passage_with(changes: &[(usize, &str)]) -> String {
    let mut fields: Vec<String> = "2,run,shp,rabaul,,,,rabaul,,,,,,,,no,,as stated"
        .split(',')
        .map(str::to_string)
        .collect();
    for (index, value) in changes {
        fields[*index] = (*value).to_string();
    }
    passage(&fields.join(","))
}

#[test]
fn rule5_passages_good_rows() {
    assert_clean(&adding(
        "passages.csv",
        &[
            passage_with(&[]),
            // Ends by coordinates, an open flag list, a ship that is a key
            // and one that is a plain name, dates alone.
            passage_with(&[
                (0, "3"),
                (2, "shp|Task Force 17"),
                (3, ""),
                (4, "-9.5"),
                (5, "160"),
                (6, "1942-08-09"),
                (7, ""),
                (8, "-10"),
                (9, "161.25"),
                (10, "1942-08-10T23:30"),
                (12, "0.5"),
                (14, "250"),
                (15, "unknown"),
                (16, "date-only|refuelled-at-Efate"),
            ]),
            // An end the source does not locate, and says so.
            passage_with(&[(0, "4"), (7, ""), (16, "to-not-located")]),
            passage_with(&[(0, "5"), (3, ""), (16, "from-not-located")]),
        ],
    ));
}

#[test]
fn rule5_passages_columns_in_order() {
    let mut files = base();
    files.insert(
        "passages.csv".to_string(),
        PASSAGE_HEADER.replace("subject,ships", "ships,subject"),
    );
    assert_problems(
        &problems_in(&files),
        1,
        &["passages.csv||columns must be, in order"],
    );
    let mut files = base();
    files.remove("passages.csv");
    // The file is optional (see `OPTIONAL_FILES` in `schema.rs`).
    assert_clean(&problems_in(&files));
    files.insert(
        "passages.csv".to_string(),
        "id,subject\n1,run\n".to_string(),
    );
    let found = problems_in(&files);
    assert!(
        found.contains(&"passages.csv||no `ships` column".to_string()),
        "{found:#?}"
    );
}

#[test]
fn rule5_passages_blank_columns() {
    let found = adding_one(
        "passages.csv",
        &passage_with(&[(1, ""), (2, ""), (15, ""), (17, "")]),
    );
    assert_problems(
        &found,
        4,
        &[
            "passages.csv|id 2|subject is blank",
            "passages.csv|id 2|ships is blank",
            "passages.csv|id 2|steady is blank",
            "passages.csv|id 2|as_stated is blank",
        ],
    );
}

#[test]
fn rule5_passages_ids_and_source() {
    let found = adding_one("passages.csv", &passage_with(&[(0, "1")]));
    assert_problems(&found, 1, &["passages.csv|id 1|duplicate id"]);
    let found = adding_one("passages.csv", &passage_with(&[(0, "x")]));
    assert_problems(&found, 1, &["passages.csv|id x|is not a whole number"]);
    let row = passage_with(&[]).replace(",a,p.1", ",nobody,p.1");
    let found = adding_one("passages.csv", &row);
    assert_problems(&found, 1, &["passages.csv|id 2|unknown source `nobody`"]);
}

#[test]
fn rule5_passages_steady_and_lists() {
    let found = adding_one("passages.csv", &passage_with(&[(15, "maybe")]));
    assert_problems(&found, 1, &["passages.csv|id 2|steady `maybe`"]);
    let found = adding_one("passages.csv", &passage_with(&[(2, "shp||tug")]));
    assert_problems(&found, 1, &["passages.csv|id 2|ships has an empty member"]);
    let found = adding_one("passages.csv", &passage_with(&[(16, "zigzag||waited")]));
    assert_problems(&found, 1, &["passages.csv|id 2|flags has an empty member"]);
}

#[test]
fn rule5_passages_places_and_located_ends() {
    let found = adding_one("passages.csv", &passage_with(&[(3, "nowhere")]));
    assert_problems(
        &found,
        1,
        &["passages.csv|id 2|unknown from_place `nowhere`"],
    );
    let found = adding_one("passages.csv", &passage_with(&[(7, "nowhere")]));
    assert_problems(&found, 1, &["passages.csv|id 2|unknown to_place `nowhere`"]);
    // Unlocated without the flag, at either end.
    let found = adding_one("passages.csv", &passage_with(&[(3, "")]));
    assert_problems(
        &found,
        1,
        &["passages.csv|id 2|the from end is not located"],
    );
    let found = adding_one("passages.csv", &passage_with(&[(7, "")]));
    assert_problems(&found, 1, &["passages.csv|id 2|the to end is not located"]);
    // The flag for the other end does not help.
    let found = adding_one(
        "passages.csv",
        &passage_with(&[(7, ""), (16, "from-not-located")]),
    );
    assert_problems(&found, 1, &["passages.csv|id 2|the to end is not located"]);
}

#[test]
fn rule5_passages_coordinates() {
    let found = adding_one("passages.csv", &passage_with(&[(4, "-9.5")]));
    assert_problems(
        &found,
        1,
        &["passages.csv|id 2|from_lat and from_lon come in pairs"],
    );
    let found = adding_one("passages.csv", &passage_with(&[(9, "159.5")]));
    assert_problems(
        &found,
        1,
        &["passages.csv|id 2|to_lat and to_lon come in pairs"],
    );
    let found = adding_one(
        "passages.csv",
        &passage_with(&[(4, "9-50S"), (5, "159E"), (8, "\"1,5\""), (9, "2")]),
    );
    assert_problems(
        &found,
        3,
        &[
            "passages.csv|id 2|from_lat `9-50S` is not a decimal",
            "passages.csv|id 2|from_lon `159E` is not a decimal",
            "passages.csv|id 2|to_lat `1,5` is not a decimal",
        ],
    );
}

#[test]
fn rule5_passages_times_and_numbers() {
    for bad in ["06:00", "1942-08-09 06:00", "1942-08-09T25:00"] {
        let found = adding_one("passages.csv", &passage_with(&[(6, bad)]));
        assert_problems(&found, 1, &["passages.csv|id 2|depart_time"]);
        let found = adding_one("passages.csv", &passage_with(&[(10, bad)]));
        assert_problems(&found, 1, &["passages.csv|id 2|arrive_time"]);
    }
    for bad in ["0", "0.0", "-5", "fast", "12 kn", "1e2"] {
        let found = adding_one("passages.csv", &passage_with(&[(12, bad)]));
        assert_problems(&found, 1, &["passages.csv|id 2|speed_kn"]);
        let found = adding_one("passages.csv", &passage_with(&[(14, bad)]));
        assert_problems(&found, 1, &["passages.csv|id 2|distance_nm_stated"]);
    }
}

// Rule 7: statements about training (`training_facts.csv`).

const TRAINING_HEADER: &str = "id,entity,attribute,value,as_stated,valid_from,valid_to,\
source,locator,accessed,confidence,basis,standing,note\n";

/// A good training row (id 1) with the fields before the source columns at
/// the given 0-based positions replaced: id, entity, attribute, value,
/// as_stated, valid_from, valid_to.
fn training_with(changes: &[(usize, &str)]) -> String {
    let mut fields: Vec<String> =
        "1,ijn-aviators,flying_hours_on_joining,650,650 hours,1941-12,1941-12"
            .split(',')
            .map(str::to_string)
            .collect();
    for (index, value) in changes {
        fields[*index] = (*value).to_string();
    }
    format!("{},{}", fields.join(","), src("a", "medium", "preferred"))
}

fn with_training(rows: &[String]) -> Vec<String> {
    let mut files = base();
    let mut text = TRAINING_HEADER.to_string();
    for row in rows {
        text.push_str(row);
        text.push('\n');
    }
    files.insert("training_facts.csv".to_string(), text);
    problems_in(&files)
}

#[test]
fn rule7_training_file_is_optional_and_good_rows_pass() {
    assert_clean(&problems_in(&base()));
    assert_clean(&with_training(&[
        training_with(&[]),
        training_with(&[
            (0, "2"),
            (1, "usn-cruiser-destroyer-crews"),
            (2, "night_training_note"),
            (3, "a text"),
            (5, ""),
            (6, ""),
        ]),
        training_with(&[(0, "3"), (5, "1941"), (6, "1942-08-09")]),
        training_with(&[(0, "4"), (5, "1942-08"), (6, "1942-08")]),
    ]));
}

#[test]
fn rule7_training_columns_in_order_and_present() {
    let mut files = base();
    files.insert(
        "training_facts.csv".to_string(),
        "id,attribute,entity,value,as_stated,valid_from,valid_to,source,locator,accessed,\
         confidence,basis,standing,note\n"
            .to_string(),
    );
    assert_problems(
        &problems_in(&files),
        1,
        &["training_facts.csv||columns must be, in order"],
    );
    files.insert(
        "training_facts.csv".to_string(),
        "id,entity,attribute,value,valid_from,valid_to,source,locator,accessed,confidence,\
         basis,standing,note\n"
            .to_string(),
    );
    let found = problems_in(&files);
    assert!(
        found.contains(&"training_facts.csv||no `as_stated` column".to_string()),
        "{found:#?}"
    );
}

#[test]
fn rule7_training_entity_and_attribute_are_closed_lists() {
    let found = with_training(&[training_with(&[(1, "ca-44-vincennes")])]);
    assert_problems(
        &found,
        1,
        &["training_facts.csv|id 1|unknown entity `ca-44-vincennes`"],
    );
    let found = with_training(&[training_with(&[(2, "luck_note")])]);
    assert_problems(
        &found,
        1,
        &["training_facts.csv|id 1|unknown attribute `luck_note`"],
    );
}

#[test]
fn rule7_training_values_by_attribute() {
    for bad in ["650.5", "-3", "many", "", "1e3"] {
        let found = with_training(&[training_with(&[(3, bad)])]);
        assert!(
            found
                .iter()
                .any(|p| p.starts_with("training_facts.csv|id 1|")),
            "`{bad}`: {found:#?}"
        );
    }
    let found = with_training(&[training_with(&[(2, "training_note"), (3, "  ")])]);
    assert_problems(
        &found,
        1,
        &["training_facts.csv|id 1|value of `training_note` is blank"],
    );
}

#[test]
fn rule7_training_dates() {
    let found = with_training(&[training_with(&[(5, "1941-13")])]);
    assert_problems(
        &found,
        1,
        &["training_facts.csv|id 1|valid_from `1941-13` is not a date"],
    );
    let found = with_training(&[training_with(&[(6, "42")])]);
    assert_problems(
        &found,
        1,
        &["training_facts.csv|id 1|valid_to `42` is not a date"],
    );
    let found = with_training(&[training_with(&[(5, "1942-08-09"), (6, "1942-08-08")])]);
    assert_problems(&found, 1, &["training_facts.csv|id 1|is after valid_to"]);
}

#[test]
fn rule7_training_general_rules_still_apply() {
    let row = format!(
        "1,ijn-aviators,flying_hours_on_joining,650,650 hours,,,{}",
        src("nobody", "medium", "preferred")
    );
    assert_problems(
        &with_training(&[row]),
        1,
        &["training_facts.csv|id 1|unknown source `nobody`"],
    );
    assert_problems(
        &with_training(&[training_with(&[]), training_with(&[])]),
        1,
        &["training_facts.csv|id 1|duplicate id"],
    );
}

// Rule 8: men carried by sea (`lift_entities.csv`, `troop_lift_facts.csv`)
// and the `troop_run` rows of `ship_facts.csv`.

const LIFT_ENTITIES_HEADER: &str = "id,key,name,kind,nation\n";
const TROOP_HEADER: &str = "id,entity,attribute,value,as_stated,valid_from,valid_to,\
source,locator,accessed,confidence,basis,standing,note\n";

/// A good lift entity file: one run and one ship.
fn lift_entities(extra: &[&str]) -> String {
    let mut text = LIFT_ENTITIES_HEADER.to_string();
    text.push_str("1000,run-a,A run,run,jp\n1001,ship-a,A ship,ship,us\n");
    for row in extra {
        text.push_str(row);
        text.push('\n');
    }
    text
}

/// A good troop statement (id 1) with the fields before the source columns
/// at the given 0-based positions replaced.
fn troop_with(changes: &[(usize, &str)]) -> String {
    let mut fields: Vec<String> = "1,run-a,troops_carried,900,900 men,1942-08,1942-08"
        .split(',')
        .map(str::to_string)
        .collect();
    for (index, value) in changes {
        fields[*index] = (*value).to_string();
    }
    format!("{},{}", fields.join(","), src("a", "medium", "preferred"))
}

fn with_lift(entities: &str, troops: &[String]) -> BTreeMap<String, String> {
    let mut files = base();
    files.insert("lift_entities.csv".to_string(), entities.to_string());
    let mut text = TROOP_HEADER.to_string();
    for row in troops {
        text.push_str(row);
        text.push('\n');
    }
    files.insert("troop_lift_facts.csv".to_string(), text);
    files
        .get_mut("attributes.csv")
        .unwrap()
        .push_str("troop_run,ship,text,many\n");
    files
}

#[test]
fn rule8_lift_files_are_optional_and_good_rows_pass() {
    assert_clean(&problems_in(&base()));
    let ok = src("a", "medium", "preferred");
    let mut files = with_lift(
        &lift_entities(&[
            "1002,group-a,A group,transport-group,us",
            "1003,type-a,A type,ship-type,jp",
        ]),
        &[
            troop_with(&[]),
            troop_with(&[(0, "2"), (2, "ships_in_run"), (3, "six destroyers")]),
            troop_with(&[
                (0, "3"),
                (1, "ship-a"),
                (2, "troop_capacity_men"),
                (3, "1388"),
            ]),
            troop_with(&[(0, "4"), (2, "lift_note"), (3, "a note")]),
        ],
    );
    files
        .get_mut("ship_facts.csv")
        .unwrap()
        .push_str(&format!("1,shp,troop_run,run-a,,,{ok}\n"));
    assert_clean(&problems_in(&files));
}

#[test]
fn rule8_lift_entity_kind_nation_and_keys() {
    let files = with_lift(&lift_entities(&["1002,x,An x,fleet,jp"]), &[]);
    assert_problems(
        &problems_in(&files),
        1,
        &["lift_entities.csv|id 1002|kind `fleet`"],
    );
    let files = with_lift(&lift_entities(&["1002,x,An x,run,de"]), &[]);
    assert_problems(
        &problems_in(&files),
        1,
        &["lift_entities.csv|id 1002|nation `de`"],
    );
    let files = with_lift(&lift_entities(&["1002,run-a,Again,run,jp"]), &[]);
    assert_problems(
        &problems_in(&files),
        1,
        &["lift_entities.csv|id 1002|duplicate key"],
    );
    let files = with_lift(&lift_entities(&["1002,Run_B,Bad key,run,jp"]), &[]);
    assert_problems(
        &problems_in(&files),
        1,
        &["lift_entities.csv|id 1002|key `Run_B`"],
    );
}

#[test]
fn rule8_troop_statement_entity_attribute_and_values() {
    let files = with_lift(&lift_entities(&[]), &[troop_with(&[(1, "run-z")])]);
    assert_problems(
        &problems_in(&files),
        1,
        &["troop_lift_facts.csv|id 1|entity `run-z` is not a key"],
    );
    let files = with_lift(&lift_entities(&[]), &[troop_with(&[(2, "troops_aboard")])]);
    assert_problems(
        &problems_in(&files),
        1,
        &["troop_lift_facts.csv|id 1|unknown attribute `troops_aboard`"],
    );
    for attribute in [
        "troop_capacity_men",
        "troops_carried",
        "troops_lost",
        "crew_complement",
        "troops_per_ship_estimated_by_enemy",
        "supply_tons_carried",
    ] {
        for bad in ["about 700", "12.5", "-3", "", "1e3"] {
            let files = with_lift(
                &lift_entities(&[]),
                &[troop_with(&[(2, attribute), (3, bad)])],
            );
            assert!(
                problems_in(&files)
                    .iter()
                    .any(|p| p.starts_with("troop_lift_facts.csv|id 1|")),
                "{attribute} `{bad}`"
            );
        }
    }
}

#[test]
fn rule8_troop_statement_columns_in_order() {
    let mut files = with_lift(&lift_entities(&[]), &[]);
    files.insert(
        "troop_lift_facts.csv".to_string(),
        "id,attribute,entity,value,as_stated,valid_from,valid_to,source,locator,accessed,\
         confidence,basis,standing,note\n"
            .to_string(),
    );
    assert_problems(
        &problems_in(&files),
        1,
        &["troop_lift_facts.csv||columns must be, in order"],
    );
}

#[test]
fn rule8_troop_run_value_must_be_a_run() {
    let ok = src("a", "medium", "preferred");
    for (value, text) in [
        ("run-z", "is not a key of lift_entities.csv"),
        ("ship-a", "of kind `ship`, not a run"),
    ] {
        let mut files = with_lift(&lift_entities(&[]), &[]);
        files
            .get_mut("ship_facts.csv")
            .unwrap()
            .push_str(&format!("1,shp,troop_run,{value},,,{ok}\n"));
        assert_problems(
            &problems_in(&files),
            1,
            &[&format!("ship_facts.csv|id 1|{text}")],
        );
    }
}
