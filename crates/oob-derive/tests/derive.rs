//! The derivation tool against a small synthetic facts directory.
//!
//! `base()` is a complete data set that passes `oob_check::check_files`.
//! Each test edits or adds a few rows in memory and says what must happen.
//!
//! # The base ship, worked by hand
//!
//! The list's date is 1942-08-08 and the rules are those of decision 0020:
//! gun 6 kg per damage point, torpedo 0.4 per kg of warhead, toughness 25
//! long tons per point. Class `us-test-class`, all rows preferred:
//!
//! | Figure | Rows | Working | Result |
//! | --- | --- | --- | --- |
//! | kind | type heavy_cruiser | cruiser | `cruiser` |
//! | cruise_speed_kn | endurance_speed_kn 15 | | 15 |
//! | max_speed_kn | 32.5 | | 32.5 |
//! | fuel_capacity_t | fuel_capacity_lt 1600 | | 1600 |
//! | endurance_nm | 8000 | | 8000 |
//! | cargo_capacity_t | | fixed | 0 |
//! | main_guns | main_battery fit, count 8 | | 8 |
//! | gun_rounds_per_min | rate_of_fire_practical_rpm of the gun, 3.5 | | 3.5 |
//! | gun_rounds_per_gun | main_rounds_per_gun 150 | | 150 |
//! | gun_damage | shell_weight_kg 118 | 118 / 6 = 19.666666 (truncated at six places) | 19.666666 |
//! | torpedo_tubes | torpedo_tubes fit, count 6 | | 6 |
//! | torpedo_damage | warhead_kg 490 | 490 x 0.4 = 196 | 196 |
//! | toughness | displacement_standard_lt 10,000 | 10,000 / 25 = 400 | 400 |
//! | radar | surface_search_radar fit, count 1 | | 1 |
//!
//! So the class row is
//! `1000,Test class,cruiser,15,32.5,1600,8000,0,8,3.5,150,19.666666,6,196,400,1`.
//!
//! The gun's design rate of fire (`rate_of_fire_rpm` 5, a theoretical rate)
//! is also in the facts; the tool does not read it.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use oob_derive::{derive, run, Outcome, Outputs};

type Files = BTreeMap<String, String>;

const DATE: &str = "1942-08-08";

const RULES: &str = "constants_are_placeholders = true\n\
    gun_shell_kg_per_damage = 6\n\
    torpedo_damage_per_warhead_kg = \"0.4\"\n\
    displacement_lt_per_toughness = 25\n";

const BASE_ROW: &str =
    "1000,Test class,cruiser,15,32.5,1600,8000,0,8,3.5,150,19.666666,6,196,400,1";

fn list(ships: &[&str], assumed: &[&str]) -> String {
    let quoted = |names: &[&str]| {
        names
            .iter()
            .map(|n| format!("\"{n}\""))
            .collect::<Vec<_>>()
            .join(", ")
    };
    format!(
        "name = \"Test list\"\ndate = \"{DATE}\"\nrules = \"rules.toml\"\n\
         output = \"../reference/test_out.csv\"\nfirst_class_id = 1000\n\
         ships = [{}]\nradar_unknown_means_none = [{}]\n",
        quoted(ships),
        quoted(assumed)
    )
}

const FACT_HEADER: &str = "id,entity,attribute,value,valid_from,valid_to,\
    source,locator,accessed,confidence,basis,standing,note\n";

const FIT_HEADER: &str = "id,carrier_type,carrier,slot,item_type,item,count,valid_from,valid_to,\
    source,locator,accessed,confidence,basis,standing,note\n";

const SOURCE_TAIL: &str = "p.1,2026-11-10";

/// A fact row. `from` and `to` are the validity.
#[allow(clippy::too_many_arguments)]
fn fact_row(
    id: u32,
    entity: &str,
    attribute: &str,
    value: &str,
    from: &str,
    to: &str,
    confidence: &str,
    standing: &str,
) -> String {
    format!(
        "{id},{entity},{attribute},{value},{from},{to},a,{SOURCE_TAIL},{confidence},stated,{standing},a note"
    )
}

fn fact(id: u32, entity: &str, attribute: &str, value: &str) -> String {
    fact_row(id, entity, attribute, value, "", "", "medium", "preferred")
}

#[allow(clippy::too_many_arguments)]
fn fit_row(
    id: u32,
    carrier_type: &str,
    carrier: &str,
    slot: &str,
    item_type: &str,
    item: &str,
    count: &str,
    validity: (&str, &str),
    source: &str,
    standing: &str,
) -> String {
    format!(
        "{id},{carrier_type},{carrier},{slot},{item_type},{item},{count},{},{},{source},{SOURCE_TAIL},medium,stated,{standing},a note",
        validity.0, validity.1
    )
}

fn class_fit(id: u32, slot: &str, item_type: &str, item: &str, count: &str) -> String {
    fit_row(
        id,
        "ship_class",
        "us-test-class",
        slot,
        item_type,
        item,
        count,
        ("", ""),
        "a",
        "preferred",
    )
}

fn ship_fit(id: u32, ship: &str, slot: &str, item_type: &str, item: &str, count: &str) -> String {
    fit_row(
        id,
        "ship",
        ship,
        slot,
        item_type,
        item,
        count,
        ("", ""),
        "a",
        "preferred",
    )
}

fn table(header: &str, rows: &[String]) -> String {
    let mut text = header.to_string();
    for row in rows {
        text.push_str(row);
        text.push('\n');
    }
    text
}

fn base() -> Files {
    let empty_fact = FACT_HEADER.to_string();
    let files = [
        (
            "sources.csv",
            "id,key,reliability\n1000,a,A\n1001,b,B\n1002,c,C\n".to_string(),
        ),
        ("places.csv", "id,key\n".to_string()),
        (
            "engagements.csv",
            "id,key,date_from,date_to\n".to_string(),
        ),
        (
            "ship_classes.csv",
            "id,key,name,nation,type\n\
             1000,us-test-class,Test class,us,heavy_cruiser\n\
             1001,jp-boat-class,Boat class,jp,seaplane_carrier\n"
                .to_string(),
        ),
        (
            "ships.csv",
            "id,key,name,nation,class,hull_number\n\
             1000,shp,Shp,us,us-test-class,\n\
             1001,shp2,Shp 2,us,us-test-class,\n\
             1002,boat,Boat,jp,jp-boat-class,\n"
                .to_string(),
        ),
        ("aircraft_types.csv", "id,key\n".to_string()),
        (
            "weapons.csv",
            "id,key\n1000,gun\n1001,tube\n1002,big-gun\n".to_string(),
        ),
        (
            "sensors.csv",
            "id,key\n1000,radar-a\n1001,radar-b\n1002,jp-radar-any\n".to_string(),
        ),
        ("air_units.csv", "id,key,parent\n".to_string()),
        (
            "attributes.csv",
            "key,applies_to,value_type,cardinality\n\
             max_speed_kn,ship_class,decimal,one\n\
             max_speed_kn,ship,decimal,one\n\
             endurance_speed_kn,ship_class,decimal,one\n\
             endurance_nm,ship_class,integer,one\n\
             fuel_capacity_lt,ship_class,integer,one\n\
             displacement_standard_lt,ship_class,integer,one\n\
             main_rounds_per_gun,ship_class,integer,one\n\
             shell_weight_kg,weapon,decimal,one\n\
             rate_of_fire_practical_rpm,weapon,decimal,one\n\
             rate_of_fire_rpm,weapon,decimal,one\n\
             warhead_kg,weapon,decimal,one\n"
                .to_string(),
        ),
        ("gaps.csv", "entity_type,entity,what\n".to_string()),
        (
            "ship_class_facts.csv",
            table(
                FACT_HEADER,
                &[
                    fact(1, "us-test-class", "max_speed_kn", "32.5"),
                    fact(2, "us-test-class", "endurance_speed_kn", "15"),
                    fact(3, "us-test-class", "endurance_nm", "8000"),
                    fact(4, "us-test-class", "fuel_capacity_lt", "1600"),
                    fact(5, "us-test-class", "displacement_standard_lt", "10000"),
                    fact(6, "us-test-class", "main_rounds_per_gun", "150"),
                ],
            ),
        ),
        ("ship_facts.csv", empty_fact.clone()),
        ("aircraft_type_facts.csv", empty_fact.clone()),
        (
            "weapon_facts.csv",
            table(
                FACT_HEADER,
                &[
                    fact(1, "gun", "shell_weight_kg", "118"),
                    fact(2, "gun", "rate_of_fire_practical_rpm", "3.5"),
                    fact(3, "tube", "warhead_kg", "490"),
                    fact(4, "big-gun", "shell_weight_kg", "0.1234567"),
                    fact(5, "big-gun", "rate_of_fire_practical_rpm", "2"),
                    fact(6, "gun", "rate_of_fire_rpm", "5"),
                ],
            ),
        ),
        ("sensor_facts.csv", empty_fact.clone()),
        ("air_unit_facts.csv", empty_fact),
        (
            "fits.csv",
            table(
                FIT_HEADER,
                &[
                    class_fit(1, "main_battery", "weapon", "gun", "8"),
                    class_fit(2, "torpedo_tubes", "weapon", "tube", "6"),
                    class_fit(3, "surface_search_radar", "sensor", "radar-a", "1"),
                ],
            ),
        ),
        (
            "engagement_forces.csv",
            "id,engagement,member_type,member,source,locator,accessed,confidence,basis,standing,note\n"
                .to_string(),
        ),
        (
            "engagement_events.csv",
            "id,engagement,time,range_m,bearing_deg,value,\
             source,locator,accessed,confidence,basis,standing,note\n"
                .to_string(),
        ),
        (
            "engagement_positions.csv",
            "id,engagement,time,lat,lon,bearing_deg,distance_nm,course_deg,speed_kn,\
             source,locator,accessed,confidence,basis,standing,note\n"
                .to_string(),
        ),
        (
            "engagement_outcomes.csv",
            "id,engagement,value,source,locator,accessed,confidence,basis,standing,note\n"
                .to_string(),
        ),
    ];
    files
        .into_iter()
        .map(|(name, text)| (name.to_string(), text))
        .collect()
}

fn replace_row(files: &mut Files, file: &str, id: u32, line: String) {
    let text = files.get_mut(file).expect("a file of the base set");
    let prefix = format!("{id},");
    let mut found = false;
    let rebuilt: Vec<String> = text
        .lines()
        .map(|l| {
            if l.starts_with(&prefix) {
                found = true;
                line.clone()
            } else {
                l.to_string()
            }
        })
        .collect();
    assert!(found, "row {id} of {file}");
    *text = rebuilt.join("\n") + "\n";
}

fn remove_row(files: &mut Files, file: &str, id: u32) {
    let text = files.get_mut(file).expect("a file of the base set");
    let prefix = format!("{id},");
    let kept: Vec<&str> = text.lines().filter(|l| !l.starts_with(&prefix)).collect();
    assert!(kept.len() < text.lines().count(), "row {id} of {file}");
    *text = kept.join("\n") + "\n";
}

fn add_row(files: &mut Files, file: &str, line: String) {
    let text = files.get_mut(file).expect("a file of the base set");
    text.push_str(&line);
    text.push('\n');
}

fn derived(ships: &[&str], files: &Files) -> Result<Outputs, Vec<String>> {
    derive_with_rules(ships, &[], RULES, files)
}

fn derive_with_rules(
    ships: &[&str],
    assumed: &[&str],
    rules: &str,
    files: &Files,
) -> Result<Outputs, Vec<String>> {
    derive("list.toml", &list(ships, assumed), rules, files)
        .map_err(|problems| problems.iter().map(ToString::to_string).collect())
}

#[track_caller]
fn derives(ships: &[&str], files: &Files) -> Outputs {
    derived(ships, files).unwrap_or_else(|problems| panic!("{problems:#?}"))
}

#[track_caller]
fn fails(ships: &[&str], files: &Files) -> Vec<String> {
    derived(ships, files).expect_err("problems expected")
}

#[track_caller]
fn assert_one(problems: &[String], expected: &[&str]) {
    assert_eq!(problems.len(), 1, "{problems:#?}");
    for want in expected {
        assert!(problems[0].contains(want), "`{want}` in {problems:#?}");
    }
}

/// The records of a generated CSV, without its comment lines or header.
fn records(text: &str) -> Vec<Vec<String>> {
    csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .from_reader(text.as_bytes())
        .records()
        .map(|r| r.expect("a record").iter().map(str::to_string).collect())
        .collect()
}

/// The sources row of a figure of a class row.
fn source_of(outputs: &Outputs, class_id: &str, figure: &str) -> Vec<String> {
    records(&outputs.sources_csv)
        .into_iter()
        .find(|r| r[0] == class_id && r[1] == figure)
        .unwrap_or_else(|| panic!("no sources row for {class_id} {figure}"))
}

fn class_lines(outputs: &Outputs) -> Vec<String> {
    outputs
        .class_csv
        .lines()
        .filter(|l| !l.starts_with('#'))
        .skip(1)
        .map(str::to_string)
        .collect()
}

fn comments(text: &str) -> String {
    text.lines()
        .filter(|l| l.starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n")
}

// The data set itself.

#[test]
fn the_base_data_set_passes_the_checker() {
    let report = oob_check::check_files(&base());
    assert!(report.is_clean(), "{:#?}", report.problems);
}

// A ship with everything preferred.

#[test]
fn a_ship_with_everything_preferred_derives_exactly() {
    let outputs = derives(&["shp"], &base());

    // 118 kg / 6 = 19.666666, written exactly; 490 x 0.4 = 196;
    // 10,000 / 25 = 400; 3.5 rounds a minute stays "3.5".
    assert_eq!(class_lines(&outputs), [BASE_ROW]);
    assert_eq!(
        outputs.class_csv.lines().find(|l| l.starts_with("id,")),
        Some(
            "id,name,kind,cruise_speed_kn,max_speed_kn,fuel_capacity_t,endurance_nm,\
             cargo_capacity_t,main_guns,gun_rounds_per_min,gun_rounds_per_gun,gun_damage,\
             torpedo_tubes,torpedo_damage,toughness,radar"
        )
    );

    let damage = source_of(&outputs, "1000", "gun_damage");
    assert_eq!(damage[2], "19.666666");
    assert_eq!(
        damage[3],
        "shell_weight_kg of the main_battery weapon / gun_shell_kg_per_damage"
    );
    assert_eq!(damage[4], "fits.csv:1 weapon_facts.csv:1");
    assert_eq!(damage[5], "");
    assert_eq!(source_of(&outputs, "1000", "torpedo_damage")[2], "196");
    assert_eq!(
        source_of(&outputs, "1000", "torpedo_damage")[4],
        "fits.csv:2 weapon_facts.csv:3"
    );
    let toughness = source_of(&outputs, "1000", "toughness");
    assert_eq!((toughness[2].as_str(), toughness[5].as_str()), ("400", ""));
    assert_eq!(toughness[4], "ship_class_facts.csv:5");
    assert_eq!(source_of(&outputs, "1000", "gun_rounds_per_min")[2], "3.5");
    // The practical rate is the one read; the design rate is not.
    assert_eq!(
        source_of(&outputs, "1000", "gun_rounds_per_min")[4],
        "fits.csv:1 weapon_facts.csv:2"
    );
    assert_eq!(source_of(&outputs, "1000", "cargo_capacity_t")[4], "");
    assert_eq!(
        source_of(&outputs, "1000", "kind")[4],
        "ship_classes.csv:1000 ships.csv:1000"
    );
    // 14 figures for the one class row.
    assert_eq!(records(&outputs.sources_csv).len(), 14);

    assert_eq!(records(&outputs.ships_csv), [["shp", "1000", "Test class"]]);
}

#[test]
fn the_header_states_the_constants_as_placeholders() {
    let outputs = derives(&["shp"], &base());
    let header = comments(&outputs.class_csv);
    assert!(header.contains("GENERATED by oob-derive"));
    assert!(header.contains("List: list.toml: Test list (as of 1942-08-08)"));
    assert!(header.contains("Rules: rules.toml"));
    assert!(header.contains("MODEL PARAMETERS AND PLACEHOLDERS, not history"));
    assert!(header.contains("gun_shell_kg_per_damage = 6"));
    assert!(header.contains("torpedo_damage_per_warhead_kg = 0.4"));
    assert!(header.contains("displacement_lt_per_toughness = 25"));
    assert!(header.contains("Exceptions used (decision 0020, 3): none"));
    assert!(header.contains("Radar assumptions used (decision 0019, 5): none"));
}

#[test]
fn a_decimal_with_more_than_six_places_is_a_bad_value() {
    // Weapon `big-gun` has a shell of 0.1234567 kg: seven places.
    let mut files = base();
    replace_row(
        &mut files,
        "fits.csv",
        1,
        class_fit(1, "main_battery", "weapon", "big-gun", "8"),
    );
    let problems = fails(&["shp"], &files);
    assert_one(
        &problems,
        &[
            "shp: bad-value: gun_damage:",
            "weapon_facts.csv:4",
            "more than 6 decimal places",
        ],
    );
}

// Rate of fire: the practical rate only, with no fallback.

#[test]
fn a_weapon_with_only_a_design_rate_is_missing_a_practical_rate() {
    // The base gun has a practical rate (row 2) and a design rate (row 6).
    // Without the practical one, the design rate is not used.
    let mut files = base();
    remove_row(&mut files, "weapon_facts.csv", 2);
    let problems = fails(&["shp"], &files);
    assert_one(
        &problems,
        &[
            "shp: missing: gun_rounds_per_min:",
            "rate_of_fire_practical_rpm of weapon gun: no rows in the facts",
            "a rate_of_fire_rpm row exists (weapon_facts.csv:6 (preferred; 5))",
            "a theoretical or maximum rate, which the tool does not read",
        ],
    );
}

#[test]
fn a_weapon_with_no_rate_at_all_says_so() {
    let mut files = base();
    remove_row(&mut files, "weapon_facts.csv", 2);
    remove_row(&mut files, "weapon_facts.csv", 6);
    let problems = fails(&["shp"], &files);
    assert_one(
        &problems,
        &[
            "shp: missing: gun_rounds_per_min:",
            "no rate_of_fire_rpm row exists either",
        ],
    );
}

#[test]
fn a_practical_rate_that_is_not_readable_is_reported_by_its_standing() {
    // Rows exist but none is preferred: the usual kind says why.
    let mut files = base();
    replace_row(
        &mut files,
        "weapon_facts.csv",
        2,
        fact_row(
            2,
            "gun",
            "rate_of_fire_practical_rpm",
            "3.5",
            "",
            "",
            "low",
            "unconfirmed",
        ),
    );
    let problems = fails(&["shp"], &files);
    assert_one(
        &problems,
        &[
            "shp: not-preferred: gun_rounds_per_min:",
            "weapon_facts.csv:2 (unconfirmed",
        ],
    );
}

// Each kind of problem.

#[test]
fn an_unknown_ship_gives_one_problem_and_nothing_more() {
    let problems = fails(&["nosuch"], &base());
    assert_eq!(problems.len(), 1);
    assert!(
        problems[0].starts_with("nosuch: unknown-ship: "),
        "{problems:?}"
    );
}

#[test]
fn a_missing_fact_names_the_ship_the_fact_and_the_class() {
    let mut files = base();
    remove_row(&mut files, "ship_class_facts.csv", 3);
    let problems = fails(&["shp"], &files);
    assert_one(
        &problems,
        &[
            "shp: missing: endurance_nm:",
            "endurance_nm of class us-test-class",
        ],
    );
}

#[test]
fn a_disputed_fact_is_not_preferred_and_lists_the_rows() {
    let mut files = base();
    replace_row(
        &mut files,
        "ship_class_facts.csv",
        1,
        fact_row(
            1,
            "us-test-class",
            "max_speed_kn",
            "32.5",
            "",
            "",
            "medium",
            "disputed",
        ),
    );
    add_row(
        &mut files,
        "ship_class_facts.csv",
        fact_row(
            7,
            "us-test-class",
            "max_speed_kn",
            "33",
            "",
            "",
            "medium",
            "disputed",
        ),
    );
    let problems = fails(&["shp"], &files);
    assert_one(
        &problems,
        &[
            "shp: not-preferred: max_speed_kn:",
            "ship_class_facts.csv:1 (disputed; 32.5",
            "ship_class_facts.csv:7 (disputed; 33",
            "us-test-class",
        ],
    );
}

#[test]
fn rows_for_other_dates_only_are_not_on_date() {
    let mut files = base();
    replace_row(
        &mut files,
        "ship_class_facts.csv",
        1,
        fact_row(
            1,
            "us-test-class",
            "max_speed_kn",
            "32.5",
            "",
            "1942-01-01",
            "medium",
            "preferred",
        ),
    );
    let problems = fails(&["shp"], &files);
    assert_one(
        &problems,
        &[
            "shp: not-on-date: max_speed_kn:",
            "ship_class_facts.csv:1 (preferred",
        ],
    );
}

#[test]
fn two_preferred_rows_that_disagree_are_a_conflict() {
    // The checker does not compare fits, so two preferred main battery
    // counts can disagree.
    let mut files = base();
    add_row(
        &mut files,
        "fits.csv",
        class_fit(10, "main_battery", "weapon", "gun", "9"),
    );
    let problems = fails(&["shp"], &files);
    assert_one(
        &problems,
        &[
            "shp: conflict: main_guns:",
            "fits.csv:1 (preferred; gun x 8",
            "fits.csv:10 (preferred; gun x 9",
        ],
    );
}

#[test]
fn two_preferred_rows_that_agree_are_fine_and_both_are_cited() {
    let mut files = base();
    add_row(
        &mut files,
        "fits.csv",
        fit_row(
            10,
            "ship_class",
            "us-test-class",
            "main_battery",
            "weapon",
            "gun",
            "8",
            ("", ""),
            "b",
            "preferred",
        ),
    );
    let outputs = derives(&["shp"], &files);
    assert_eq!(class_lines(&outputs), [BASE_ROW]);
    assert_eq!(
        source_of(&outputs, "1000", "main_guns")[4],
        "fits.csv:1 fits.csv:10"
    );
    assert_eq!(
        source_of(&outputs, "1000", "gun_damage")[4],
        "fits.csv:1 fits.csv:10 weapon_facts.csv:1"
    );
}

#[test]
fn unknown_radar_is_a_problem_naming_what_was_found() {
    let mut files = base();
    remove_row(&mut files, "fits.csv", 3);
    let problems = fails(&["shp"], &files);
    assert_one(
        &problems,
        &[
            "shp: radar-unknown: radar:",
            "air_search_radar fit of class us-test-class: no rows",
            "surface_search_radar fit of class us-test-class: no rows",
        ],
    );
}

#[test]
fn a_count_zero_of_a_specific_set_does_not_say_there_was_no_radar() {
    let mut files = base();
    replace_row(
        &mut files,
        "fits.csv",
        3,
        class_fit(3, "surface_search_radar", "sensor", "radar-b", "0"),
    );
    let problems = fails(&["shp"], &files);
    assert_one(
        &problems,
        &[
            "shp: radar-unknown: radar:",
            "fits.csv:3 says radar-b x 0 (a specific set absent",
        ],
    );
}

#[test]
fn a_stand_in_item_with_count_zero_means_no_radar() {
    let mut files = base();
    replace_row(
        &mut files,
        "fits.csv",
        3,
        class_fit(3, "surface_search_radar", "sensor", "jp-radar-any", "0"),
    );
    let outputs = derives(&["shp"], &files);
    assert!(
        class_lines(&outputs)[0].ends_with(",400,0"),
        "{:?}",
        class_lines(&outputs)
    );
    assert_eq!(source_of(&outputs, "1000", "radar")[2], "0");
    assert_eq!(source_of(&outputs, "1000", "radar")[4], "fits.csv:3");
    assert_eq!(source_of(&outputs, "1000", "radar")[5], "");
}

#[test]
fn count_zero_in_both_radar_slots_means_no_radar() {
    let mut files = base();
    replace_row(
        &mut files,
        "fits.csv",
        3,
        class_fit(3, "surface_search_radar", "sensor", "radar-b", "0"),
    );
    add_row(
        &mut files,
        "fits.csv",
        class_fit(10, "air_search_radar", "sensor", "radar-b", "0"),
    );
    let outputs = derives(&["shp"], &files);
    assert!(class_lines(&outputs)[0].ends_with(",400,0"));
    assert_eq!(
        source_of(&outputs, "1000", "radar")[4],
        "fits.csv:3 fits.csv:10"
    );
}

#[test]
fn an_air_search_radar_alone_is_a_radar() {
    let mut files = base();
    replace_row(
        &mut files,
        "fits.csv",
        3,
        class_fit(3, "air_search_radar", "sensor", "radar-a", "1"),
    );
    let outputs = derives(&["shp"], &files);
    assert!(class_lines(&outputs)[0].ends_with(",400,1"));
}

#[test]
fn an_unmapped_class_type_is_a_problem() {
    let problems = fails(&["boat"], &base());
    assert!(
        problems
            .iter()
            .any(|p| p.starts_with("boat: unmapped-type: kind: ") && p.contains("seaplane_carrier")),
        "{problems:#?}"
    );
}

#[test]
fn an_ambiguous_partial_date_on_a_preferred_row_is_a_problem() {
    let mut files = base();
    replace_row(
        &mut files,
        "ship_class_facts.csv",
        1,
        fact_row(
            1,
            "us-test-class",
            "max_speed_kn",
            "32.5",
            "",
            "1942-08",
            "medium",
            "preferred",
        ),
    );
    let problems = fails(&["shp"], &files);
    assert_one(
        &problems,
        &[
            "shp: ambiguous-date: max_speed_kn:",
            "ship_class_facts.csv:1 (preferred",
        ],
    );
}

#[test]
fn a_ship_without_a_class_is_a_problem() {
    let mut files = base();
    files
        .get_mut("ships.csv")
        .unwrap()
        .push_str("1003,tug,Tug,us,,\n");
    let problems = fails(&["tug"], &files);
    assert_one(&problems, &["tug: no-class: class:", "ships.csv:1003"]);
}

#[test]
fn a_toughness_that_is_not_above_zero_is_a_problem() {
    let mut files = base();
    replace_row(
        &mut files,
        "ship_class_facts.csv",
        5,
        fact(5, "us-test-class", "displacement_standard_lt", "0"),
    );
    let problems = fails(&["shp"], &files);
    assert_one(
        &problems,
        &[
            "shp: zero-toughness: toughness:",
            "= 0, which is not above zero",
        ],
    );
}

#[test]
fn a_toughness_below_one_is_kept_as_a_decimal() {
    // 24 / 25 = 0.96: the figure is no longer cut to a whole number.
    let mut files = base();
    replace_row(
        &mut files,
        "ship_class_facts.csv",
        5,
        fact(5, "us-test-class", "displacement_standard_lt", "24"),
    );
    let outputs = derives(&["shp"], &files);
    assert_eq!(
        class_lines(&outputs),
        ["1000,Test class,cruiser,15,32.5,1600,8000,0,8,3.5,150,19.666666,6,196,0.96,1"]
    );
    assert_eq!(source_of(&outputs, "1000", "toughness")[2], "0.96");
    assert_eq!(source_of(&outputs, "1000", "toughness")[5], "");
}

#[test]
fn decimal_damage_and_toughness_are_written_exactly_with_trailing_zeros_trimmed() {
    // Shell 118.5 kg / 6 = 19.75. Warhead 490.5 kg x 0.4 = 196.2.
    // Displacement 10,136 lt / 25 = 405.44.
    let mut files = base();
    replace_row(
        &mut files,
        "weapon_facts.csv",
        1,
        fact(1, "gun", "shell_weight_kg", "118.5"),
    );
    replace_row(
        &mut files,
        "weapon_facts.csv",
        3,
        fact(3, "tube", "warhead_kg", "490.5"),
    );
    replace_row(
        &mut files,
        "ship_class_facts.csv",
        5,
        fact(5, "us-test-class", "displacement_standard_lt", "10136"),
    );
    let outputs = derives(&["shp"], &files);
    assert_eq!(
        class_lines(&outputs),
        ["1000,Test class,cruiser,15,32.5,1600,8000,0,8,3.5,150,19.75,6,196.2,405.44,1"]
    );
    assert_eq!(source_of(&outputs, "1000", "gun_damage")[2], "19.75");
    assert_eq!(source_of(&outputs, "1000", "torpedo_damage")[2], "196.2");
    assert_eq!(source_of(&outputs, "1000", "toughness")[2], "405.44");
    // Nothing was truncated, so nothing is noted.
    assert_eq!(source_of(&outputs, "1000", "toughness")[5], "");
}

// Exceptions and assumptions.

fn unconfirmed_displacement(files: &mut Files) {
    replace_row(
        files,
        "ship_class_facts.csv",
        5,
        fact_row(
            5,
            "us-test-class",
            "displacement_standard_lt",
            "10000",
            "",
            "",
            "low",
            "unconfirmed",
        ),
    );
}

fn rules_with_exception(row: u32) -> String {
    format!(
        "{RULES}\n[[exception]]\nentity = \"us-test-class\"\nfact = \"displacement_standard_lt\"\n\
         row = {row}\nreason = \"Only one source; the designer accepts it\"\nagreed = \"2026-11-10\"\n"
    )
}

#[test]
fn an_exception_makes_an_unconfirmed_row_readable_and_is_recorded() {
    let mut files = base();
    unconfirmed_displacement(&mut files);
    let without = fails(&["shp"], &files);
    assert_one(
        &without,
        &[
            "shp: not-preferred: toughness:",
            "ship_class_facts.csv:5 (unconfirmed",
        ],
    );

    let outputs = derive_with_rules(&["shp"], &[], &rules_with_exception(5), &files)
        .unwrap_or_else(|p| panic!("{p:#?}"));
    assert_eq!(class_lines(&outputs), [BASE_ROW]);
    let header = comments(&outputs.class_csv);
    assert!(
        header.contains(
            "us-test-class displacement_standard_lt: ship_class_facts.csv row 5 (unconfirmed); \
             reason: Only one source; the designer accepts it; agreed 2026-11-10"
        ),
        "{header}"
    );
    assert_eq!(
        source_of(&outputs, "1000", "toughness")[5],
        "rests on an exception"
    );
    assert_eq!(
        source_of(&outputs, "1000", "toughness")[4],
        "ship_class_facts.csv:5"
    );
    assert_eq!(source_of(&outputs, "1000", "max_speed_kn")[5], "");
}

#[test]
fn an_exception_for_a_row_that_does_not_exist_is_a_problem() {
    let problems = derive_with_rules(&["shp"], &[], &rules_with_exception(99), &base())
        .expect_err("a problem");
    let problems: Vec<String> = problems.to_vec();
    assert_one(
        &problems,
        &["rules: exception: displacement_standard_lt:", "row 99"],
    );
}

#[test]
fn an_exception_for_a_preferred_row_is_unused_and_not_listed() {
    let outputs = derive_with_rules(&["shp"], &[], &rules_with_exception(5), &base())
        .unwrap_or_else(|p| panic!("{p:#?}"));
    assert!(comments(&outputs.class_csv).contains("Exceptions used (decision 0020, 3): none"));
    assert_eq!(source_of(&outputs, "1000", "toughness")[5], "");
}

#[test]
fn a_radar_assumption_is_recorded_in_the_header_and_the_sources() {
    let mut files = base();
    remove_row(&mut files, "fits.csv", 3);
    let outputs =
        derive_with_rules(&["shp"], &["shp"], RULES, &files).unwrap_or_else(|p| panic!("{p:#?}"));
    assert!(class_lines(&outputs)[0].ends_with(",400,0"));
    let header = comments(&outputs.class_csv);
    assert!(
        header.contains("shp: radar unknown in the facts, treated as not fitted"),
        "{header}"
    );
    assert_eq!(
        source_of(&outputs, "1000", "radar")[5],
        "radar assumed none for shp"
    );
}

#[test]
fn an_assumption_is_not_used_when_the_radar_is_known() {
    let outputs =
        derive_with_rules(&["shp"], &["shp"], RULES, &base()).unwrap_or_else(|p| panic!("{p:#?}"));
    assert_eq!(class_lines(&outputs), [BASE_ROW]);
    assert!(
        comments(&outputs.class_csv).contains("Radar assumptions used (decision 0019, 5): none")
    );
}

#[test]
fn list_problems_are_reported() {
    let text = "name = \"x\"\ndate = \"1942-08\"\nrules = \"rules.toml\"\n\
                output = \"../out.csv\"\nfirst_class_id = 1000\n\
                ships = [\"shp\", \"shp\"]\nradar_unknown_means_none = [\"zzz\"]\n";
    let problems: Vec<String> = derive("list.toml", text, RULES, &base())
        .unwrap_err()
        .iter()
        .map(ToString::to_string)
        .collect();
    assert!(
        problems.iter().any(|p| p.starts_with("list: list: date:")),
        "{problems:#?}"
    );
    let text = text.replace("1942-08", "1942-08-08");
    let problems: Vec<String> = derive("list.toml", &text, RULES, &base())
        .unwrap_err()
        .iter()
        .map(ToString::to_string)
        .collect();
    assert_eq!(problems.len(), 2, "{problems:#?}");
    assert!(problems[0].starts_with("list: list: ships: ship `shp` is listed more than once"));
    assert!(problems[1].starts_with("list: list: radar_unknown_means_none: `zzz` is not in ships"));
}

#[test]
fn rules_problems_are_reported() {
    let rules = "constants_are_placeholders = true\n\
                 gun_shell_kg_per_damage = 6.0\n\
                 torpedo_damage_per_warhead_kg = \"0.4000001\"\n\
                 displacement_lt_per_toughness = 25\n\
                 [[exception]]\nentity = \"x\"\nfact = \"y\"\nrow = 1\nreason = \"\"\nagreed = \"2026\"\n";
    let problems: Vec<String> = derive("list.toml", &list(&["shp"], &[]), rules, &base())
        .unwrap_err()
        .iter()
        .map(ToString::to_string)
        .collect();
    assert_eq!(problems.len(), 4, "{problems:#?}");
    assert!(problems[0].starts_with("rules: rules: gun_shell_kg_per_damage: a TOML float"));
    assert!(problems[1].starts_with("rules: rules: torpedo_damage_per_warhead_kg: `0.4000001`"));
    assert!(problems[2].contains("`reason` is empty"));
    assert!(problems[3].contains("`agreed` is `2026`"));
}

#[test]
fn facts_that_fail_the_checker_are_reported_and_nothing_is_derived() {
    let mut files = base();
    files
        .get_mut("places.csv")
        .unwrap()
        .push_str("1000,x\n1000,y\n");
    let problems = fails(&["shp", "nosuch"], &files);
    assert!(!problems.is_empty());
    assert!(
        problems.iter().all(|p| p.starts_with("facts: facts: ")),
        "{problems:#?}"
    );
}

// Reading on a date.

#[test]
fn a_ship_fit_replaces_the_class_fit_for_that_slot_only() {
    let mut files = base();
    add_row(
        &mut files,
        "fits.csv",
        ship_fit(10, "shp2", "main_battery", "weapon", "gun", "9"),
    );
    let outputs = derives(&["shp", "shp2"], &files);
    // shp2 has 9 guns; its other slots still come from the class.
    assert_eq!(
        class_lines(&outputs),
        [
            "1000,\"Test class, main_guns 8\",cruiser,15,32.5,1600,8000,0,8,3.5,150,19.666666,6,196,400,1",
            "1001,\"Test class, main_guns 9\",cruiser,15,32.5,1600,8000,0,9,3.5,150,19.666666,6,196,400,1",
        ]
    );
    assert_eq!(source_of(&outputs, "1001", "main_guns")[4], "fits.csv:10");
    assert_eq!(
        source_of(&outputs, "1001", "torpedo_tubes")[4],
        "fits.csv:2"
    );
}

#[test]
fn a_ship_fact_replaces_the_class_fact() {
    let mut files = base();
    add_row(
        &mut files,
        "ship_facts.csv",
        fact(1, "shp", "max_speed_kn", "34"),
    );
    let outputs = derives(&["shp"], &files);
    assert_eq!(source_of(&outputs, "1000", "max_speed_kn")[2], "34");
    assert_eq!(
        source_of(&outputs, "1000", "max_speed_kn")[4],
        "ship_facts.csv:1"
    );
    // A ship row that does not hold on the date leaves the class fact.
    let mut files = base();
    add_row(
        &mut files,
        "ship_facts.csv",
        fact_row(
            1,
            "shp",
            "max_speed_kn",
            "34",
            "",
            "1942-01-01",
            "medium",
            "preferred",
        ),
    );
    let outputs = derives(&["shp"], &files);
    assert_eq!(source_of(&outputs, "1000", "max_speed_kn")[2], "32.5");
}

#[test]
fn a_ship_fact_that_is_not_preferred_does_not_replace_the_class_fact_and_is_noted() {
    let mut files = base();
    add_row(
        &mut files,
        "ship_facts.csv",
        fact_row(1, "shp", "max_speed_kn", "34", "", "", "low", "unconfirmed"),
    );
    let outputs = derives(&["shp"], &files);
    let speed = source_of(&outputs, "1000", "max_speed_kn");
    assert_eq!(speed[2], "32.5");
    assert_eq!(speed[4], "ship_class_facts.csv:1");
    assert_eq!(
        speed[5],
        "ship rows holding on the date but not read: ship_facts.csv:1 (unconfirmed)"
    );
    // Only that figure carries the note.
    assert_eq!(source_of(&outputs, "1000", "endurance_nm")[5], "");
}

#[test]
fn a_ship_fact_named_by_an_exception_does_replace_the_class_fact() {
    let mut files = base();
    add_row(
        &mut files,
        "ship_facts.csv",
        fact_row(1, "shp", "max_speed_kn", "34", "", "", "low", "unconfirmed"),
    );
    let rules = format!(
        "{RULES}\n[[exception]]\nentity = \"shp\"\nfact = \"max_speed_kn\"\n\
         row = 1\nreason = \"Only one source\"\nagreed = \"2026-11-10\"\n"
    );
    let outputs =
        derive_with_rules(&["shp"], &[], &rules, &files).unwrap_or_else(|p| panic!("{p:#?}"));
    let speed = source_of(&outputs, "1000", "max_speed_kn");
    assert_eq!(speed[2], "34");
    assert_eq!(speed[4], "ship_facts.csv:1");
    assert_eq!(speed[5], "rests on an exception");
}

#[test]
fn a_ship_fit_that_is_not_preferred_does_not_replace_the_class_fit_and_is_noted() {
    let mut files = base();
    add_row(
        &mut files,
        "fits.csv",
        fit_row(
            10,
            "ship",
            "shp2",
            "main_battery",
            "weapon",
            "gun",
            "9",
            ("", ""),
            "a",
            "unconfirmed",
        )
        // Unconfirmed means weakly supported.
        .replace(",medium,", ",low,"),
    );
    let outputs = derives(&["shp", "shp2"], &files);
    // Both ships read the class's 8 guns, so they share one row.
    assert_eq!(class_lines(&outputs), [BASE_ROW]);
    let note = "ship rows holding on the date but not read: fits.csv:10 (unconfirmed)";
    assert_eq!(source_of(&outputs, "1000", "main_guns")[4], "fits.csv:1");
    assert_eq!(source_of(&outputs, "1000", "main_guns")[5], note);
    // The fit says which weapon, so the weapon's figures carry the doubt.
    assert_eq!(source_of(&outputs, "1000", "gun_damage")[5], note);
    assert_eq!(source_of(&outputs, "1000", "gun_rounds_per_min")[5], note);
    assert_eq!(source_of(&outputs, "1000", "torpedo_tubes")[5], "");
}

#[test]
fn a_ship_row_that_is_not_readable_and_not_on_the_date_is_not_noted() {
    let mut files = base();
    add_row(
        &mut files,
        "ship_facts.csv",
        fact_row(
            1,
            "shp",
            "max_speed_kn",
            "34",
            "",
            "1942-01-01",
            "low",
            "unconfirmed",
        ),
    );
    let outputs = derives(&["shp"], &files);
    assert_eq!(source_of(&outputs, "1000", "max_speed_kn")[5], "");
}

#[test]
fn a_ship_row_that_may_hold_and_is_not_readable_is_noted_as_such() {
    let mut files = base();
    add_row(
        &mut files,
        "ship_facts.csv",
        fact_row(
            1,
            "shp",
            "max_speed_kn",
            "34",
            "1942-08",
            "",
            "low",
            "unconfirmed",
        ),
    );
    let outputs = derives(&["shp"], &files);
    assert_eq!(
        source_of(&outputs, "1000", "max_speed_kn")[5],
        "ship rows holding on the date but not read: ship_facts.csv:1 (unconfirmed, may hold)"
    );
}

#[test]
fn unreadable_ship_rows_are_named_when_the_class_rows_fail_too() {
    let mut files = base();
    remove_row(&mut files, "ship_class_facts.csv", 1);
    add_row(
        &mut files,
        "ship_facts.csv",
        fact_row(1, "shp", "max_speed_kn", "34", "", "", "low", "unconfirmed"),
    );
    let problems = fails(&["shp"], &files);
    assert_one(
        &problems,
        &[
            "shp: missing: max_speed_kn:",
            "max_speed_kn of class us-test-class: no rows in the facts",
            "ship rows not read: ship_facts.csv:1 (unconfirmed)",
        ],
    );
}

#[test]
fn a_ship_row_that_cannot_be_placed_in_time_is_an_ambiguous_date() {
    let mut files = base();
    add_row(
        &mut files,
        "ship_facts.csv",
        fact_row(
            1,
            "shp",
            "max_speed_kn",
            "34",
            "1942-08",
            "",
            "medium",
            "preferred",
        ),
    );
    let problems = fails(&["shp"], &files);
    assert_one(
        &problems,
        &["shp: ambiguous-date: max_speed_kn:", "ship_facts.csv:1"],
    );
}

fn with_class_speed_validity(from: &str, to: &str) -> Files {
    let mut files = base();
    replace_row(
        &mut files,
        "ship_class_facts.csv",
        1,
        fact_row(
            1,
            "us-test-class",
            "max_speed_kn",
            "32.5",
            from,
            to,
            "medium",
            "preferred",
        ),
    );
    files
}

#[test]
fn a_row_ending_before_the_date_is_not_read() {
    let problems = fails(&["shp"], &with_class_speed_validity("", "1942-08-07"));
    assert_one(&problems, &["shp: not-on-date: max_speed_kn:"]);
}

#[test]
fn valid_to_is_exclusive_so_a_row_ending_on_the_date_is_not_read() {
    let problems = fails(&["shp"], &with_class_speed_validity("", "1942-08-08"));
    assert_one(&problems, &["shp: not-on-date: max_speed_kn:"]);
    // The day after, it is read.
    derives(&["shp"], &with_class_speed_validity("", "1942-08-09"));
}

#[test]
fn valid_from_is_inclusive_so_a_row_starting_on_the_date_is_read() {
    derives(&["shp"], &with_class_speed_validity("1942-08-08", ""));
    let problems = fails(&["shp"], &with_class_speed_validity("1942-08-09", ""));
    assert_one(&problems, &["shp: not-on-date: max_speed_kn:"]);
}

#[test]
fn the_single_date_form_is_read_on_that_date_only() {
    derives(
        &["shp"],
        &with_class_speed_validity("1942-08-08", "1942-08-08"),
    );
    let problems = fails(
        &["shp"],
        &with_class_speed_validity("1942-08-09", "1942-08-09"),
    );
    assert_one(&problems, &["shp: not-on-date: max_speed_kn:"]);
    // A partial single date holding on the date cannot be told.
    let problems = fails(&["shp"], &with_class_speed_validity("1942-08", "1942-08"));
    assert_one(&problems, &["shp: ambiguous-date: max_speed_kn:"]);
}

// Torpedoes, grouping, all problems.

#[test]
fn a_count_zero_torpedo_row_gives_no_tubes_and_no_damage() {
    let mut files = base();
    add_row(
        &mut files,
        "fits.csv",
        ship_fit(10, "shp2", "torpedo_tubes", "weapon", "tube", "0"),
    );
    // No warhead is needed when there are no tubes: remove it.
    remove_row(&mut files, "weapon_facts.csv", 3);
    let outputs = derives(&["shp2"], &files);
    let row = &class_lines(&outputs)[0];
    assert_eq!(
        row,
        "1000,Test class,cruiser,15,32.5,1600,8000,0,8,3.5,150,19.666666,0,0,400,1"
    );
    assert_eq!(
        source_of(&outputs, "1000", "torpedo_tubes")[4],
        "fits.csv:10"
    );
    assert_eq!(source_of(&outputs, "1000", "torpedo_damage")[2], "0");
    assert_eq!(
        source_of(&outputs, "1000", "torpedo_damage")[4],
        "fits.csv:10"
    );
}

#[test]
fn two_ships_of_one_class_with_different_radar_get_two_named_rows() {
    let mut files = base();
    add_row(
        &mut files,
        "fits.csv",
        ship_fit(
            10,
            "shp2",
            "surface_search_radar",
            "sensor",
            "jp-radar-any",
            "0",
        ),
    );
    let outputs = derives(&["shp", "shp2"], &files);
    // Ids by figures, ascending: radar 0 sorts before radar 1.
    assert_eq!(
        class_lines(&outputs),
        [
            "1000,\"Test class, no radar\",cruiser,15,32.5,1600,8000,0,8,3.5,150,19.666666,6,196,400,0",
            "1001,\"Test class, radar\",cruiser,15,32.5,1600,8000,0,8,3.5,150,19.666666,6,196,400,1",
        ]
    );
    assert_eq!(
        records(&outputs.ships_csv),
        [
            ["shp", "1001", "Test class, radar"],
            ["shp2", "1000", "Test class, no radar"],
        ]
    );
}

#[test]
fn ships_with_equal_figures_share_a_row_even_when_one_rests_on_an_assumption() {
    let mut files = base();
    add_row(
        &mut files,
        "fits.csv",
        ship_fit(
            10,
            "shp2",
            "surface_search_radar",
            "sensor",
            "jp-radar-any",
            "0",
        ),
    );
    add_row(
        &mut files,
        "fits.csv",
        ship_fit(
            11,
            "shp",
            "surface_search_radar",
            "sensor",
            "jp-radar-any",
            "0",
        ),
    );
    remove_row(&mut files, "fits.csv", 10);
    // shp: known no radar. shp2: class radar-a (radar). Remove it so shp2
    // is unknown, and let the list assume none for shp2.
    remove_row(&mut files, "fits.csv", 3);
    let outputs = derive_with_rules(&["shp", "shp2"], &["shp2"], RULES, &files)
        .unwrap_or_else(|p| panic!("{p:#?}"));
    assert_eq!(class_lines(&outputs).len(), 1);
    assert_eq!(
        records(&outputs.ships_csv),
        [
            ["shp", "1000", "Test class"],
            ["shp2", "1000", "Test class"]
        ]
    );
    assert_eq!(
        source_of(&outputs, "1000", "radar")[5],
        "radar assumed none for shp2"
    );
    assert!(comments(&outputs.class_csv).contains("shp2: radar unknown in the facts"));
}

#[test]
fn several_problems_across_several_ships_are_all_reported() {
    let mut files = base();
    remove_row(&mut files, "fits.csv", 3);
    remove_row(&mut files, "ship_class_facts.csv", 3);
    let problems = fails(&["boat", "nosuch", "shp"], &files);
    let subjects: Vec<&str> = problems
        .iter()
        .map(|p| p.split(':').next().unwrap())
        .collect();
    // In list order: boat first, nosuch once, shp last.
    let first_nosuch = subjects.iter().position(|s| *s == "nosuch").unwrap();
    let first_shp = subjects.iter().position(|s| *s == "shp").unwrap();
    assert!(subjects[..first_nosuch].iter().all(|s| *s == "boat"));
    assert_eq!(subjects.iter().filter(|s| **s == "nosuch").count(), 1);
    assert!(first_nosuch < first_shp);
    assert!(subjects[first_shp..].iter().all(|s| *s == "shp"));
    // shp: missing endurance and radar unknown.
    assert!(problems
        .iter()
        .any(|p| p.starts_with("shp: missing: endurance_nm:")));
    assert!(problems
        .iter()
        .any(|p| p.starts_with("shp: radar-unknown: radar:")));
    // boat: an unmapped type and its missing facts, all of them.
    assert!(problems
        .iter()
        .any(|p| p.starts_with("boat: unmapped-type: kind:")));
    assert!(problems
        .iter()
        .any(|p| p.starts_with("boat: missing: cruise_speed_kn:")));
    assert!(problems
        .iter()
        .any(|p| p.starts_with("boat: missing: toughness:")));
    assert!(problems
        .iter()
        .any(|p| p.starts_with("boat: missing: main_guns:")));
    // No rate of fire or gun damage problem on top of the missing battery.
    assert!(!problems
        .iter()
        .any(|p| p.starts_with("boat: missing: gun_rounds_per_min:")));
}

#[test]
fn a_figure_needing_an_earlier_one_that_failed_adds_no_problem() {
    let mut files = base();
    remove_row(&mut files, "fits.csv", 1);
    let problems = fails(&["shp"], &files);
    assert_one(
        &problems,
        &[
            "shp: missing: main_guns:",
            "main_battery fit of class us-test-class",
        ],
    );
}

// Files: nothing written with a problem, nothing written by --check.

fn scratch(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("oob_derive_{name}"));
    let _ = std::fs::remove_dir_all(&dir);
    for sub in ["derivation", "oob", "reference"] {
        std::fs::create_dir_all(dir.join(sub)).unwrap();
    }
    dir
}

fn install(dir: &Path, ships: &[&str], assumed: &[&str], files: &Files) {
    std::fs::write(dir.join("derivation/list.toml"), list(ships, assumed)).unwrap();
    std::fs::write(dir.join("derivation/rules.toml"), RULES).unwrap();
    for (name, text) in files {
        std::fs::write(dir.join("oob").join(name), text).unwrap();
    }
}

fn reference_files(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir.join("reference"))
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

fn go(dir: &Path, check_only: bool) -> Outcome {
    run(
        &dir.join("derivation/list.toml"),
        &dir.join("oob"),
        check_only,
    )
    .expect("run")
}

#[test]
fn with_a_problem_nothing_is_written() {
    let dir = scratch("problem");
    let mut files = base();
    remove_row(&mut files, "fits.csv", 3);
    install(&dir, &["shp"], &[], &files);
    match go(&dir, false) {
        Outcome::Problems(problems) => assert_eq!(problems.len(), 1),
        other => panic!("{other:?}"),
    }
    assert!(reference_files(&dir).is_empty());
}

#[test]
fn check_never_writes_even_when_clean() {
    let dir = scratch("check");
    install(&dir, &["shp"], &[], &base());
    match go(&dir, true) {
        Outcome::Clean { files, written } => {
            assert!(!written);
            let names: Vec<_> = files
                .iter()
                .map(|f| f.file_name().unwrap().to_string_lossy().into_owned())
                .collect();
            assert_eq!(
                names,
                ["test_out.csv", "test_out.sources.csv", "test_out.ships.csv"]
            );
        }
        other => panic!("{other:?}"),
    }
    assert!(reference_files(&dir).is_empty());
}

#[test]
fn a_clean_run_writes_the_three_files_and_a_second_run_gives_the_same_bytes() {
    let first = scratch("first");
    let second = scratch("second");
    for dir in [&first, &second] {
        install(dir, &["shp", "shp2"], &[], &base());
        match go(dir, false) {
            Outcome::Clean { written, files } => {
                assert!(written);
                assert_eq!(files.len(), 3);
            }
            other => panic!("{other:?}"),
        }
    }
    // Run the first again over its own output.
    assert!(matches!(go(&first, false), Outcome::Clean { .. }));
    let names = [
        "reference/test_out.csv",
        "reference/test_out.sources.csv",
        "reference/test_out.ships.csv",
    ];
    for name in names {
        let a = std::fs::read(first.join(name)).unwrap();
        let b = std::fs::read(second.join(name)).unwrap();
        assert_eq!(a, b, "{name}");
        let text = String::from_utf8(a).unwrap();
        assert!(!text.contains(env!("CARGO_TARGET_TMPDIR")), "{name}");
        assert!(!text.contains("/Users"), "{name}");
    }
    assert_eq!(
        reference_files(&first),
        ["test_out.csv", "test_out.ships.csv", "test_out.sources.csv"]
    );
}

#[test]
fn the_pure_function_is_deterministic() {
    let a = derives(&["shp", "shp2"], &base());
    let b = derives(&["shp", "shp2"], &base());
    assert_eq!(a, b);
}

#[test]
fn the_command_exits_2_on_a_usage_error_and_1_on_problems() {
    let exe = env!("CARGO_BIN_EXE_oob-derive");
    let status = std::process::Command::new(exe).status().unwrap();
    assert_eq!(status.code(), Some(2));
    let status = std::process::Command::new(exe)
        .args(["list.toml", "--bogus"])
        .status()
        .unwrap();
    assert_eq!(status.code(), Some(2));

    let dir = scratch("exit");
    let mut files = base();
    remove_row(&mut files, "fits.csv", 3);
    install(&dir, &["shp"], &[], &files);
    let output = std::process::Command::new(exe)
        .arg(dir.join("derivation/list.toml"))
        .arg("--facts")
        .arg(dir.join("oob"))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("shp: radar-unknown: radar:"), "{stdout}");
    assert!(stdout.trim_end().ends_with(": 1 problem"), "{stdout}");
    assert!(reference_files(&dir).is_empty());

    install(&dir, &["shp"], &[], &base());
    let output = std::process::Command::new(exe)
        .arg(dir.join("derivation/list.toml"))
        .arg("--facts")
        .arg(dir.join("oob"))
        .arg("--check")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert!(String::from_utf8(output.stdout)
        .unwrap()
        .contains("would write:"));
    assert!(reference_files(&dir).is_empty());
}

// The generated file loads and runs.

#[test]
fn the_generated_classes_load_and_a_tiny_scenario_runs() {
    use sim_core::fixed::Fixed;
    use sim_core::ids::{ShipClassId, ShipId};

    let dir = scratch("scenario");
    install(&dir, &["shp", "shp2"], &[], &base());
    assert!(matches!(
        go(&dir, false),
        Outcome::Clean { written: true, .. }
    ));

    let classes = sim_data::load_ship_classes(&dir.join("reference/test_out.csv")).expect("loads");
    let class = &classes.ship_classes[&ShipClassId::new(1000)];
    assert_eq!(class.name, "Test class");
    assert_eq!(class.main_guns, 8);
    assert_eq!(class.gun_rounds_per_min, "3.5".parse::<Fixed>().unwrap());
    assert_eq!(class.gun_rounds_per_gun, 150);
    assert_eq!(class.gun_damage, "19.666666".parse::<Fixed>().unwrap());
    assert_eq!(class.torpedo_tubes, 6);
    assert_eq!(class.torpedo_damage, Fixed::from_int(196));
    assert_eq!(class.toughness, Fixed::from_int(400));
    assert!(class.radar);

    // The 01 transit scenario, with the derived class: Noumea to Espiritu
    // Santo, 11 hexes of 40 nm = 440 nm. At the class's 15 kn cruise speed
    // a 24-hour day covers 360 nm and the remaining 80 nm are finished on
    // day 2. Fuel burned per nm is 1600 t / 8000 nm = 0.2 t, so
    // 440 x 0.2 = 88 t, leaving 1512 t.
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let data = |p: &str| repo.join("data").join(p).display().to_string();
    let scenario = format!(
        "name = \"Derived classes, transit\"\nplaceholder = true\nseed = 1\ndays = 2\n\
         map = \"{}\"\nship_classes = \"{}\"\nport_sizes = \"{}\"\naircraft_types = \"{}\"\n\
         search = \"{}\"\nsurface_combat = \"{}\"\n\
         [[base]]\nid = 1\nname = \"Noumea\"\nside = \"allied\"\nq = 0\nr = 0\n\
         [[base]]\nid = 2\nname = \"Espiritu Santo\"\nside = \"allied\"\nq = 8\nr = 3\n\
         [[task_force]]\nid = 1\nname = \"Test force\"\nside = \"allied\"\nat_base = 1\nmission = \"transport\"\n\
         [[ship]]\nid = 1\nname = \"Test ship\"\nclass = 1000\nside = \"allied\"\nfuel_t = 1600\ntask_force = 1\n\
         [[order]]\nday = 1\nside = \"allied\"\ntask_force = 1\nkind = \"set_route\"\nwaypoints = [[8, 3]]\n\
         [[order]]\nday = 1\nside = \"allied\"\ntask_force = 1\nkind = \"set_speed\"\nspeed = \"cruise\"\n",
        data("map/placeholder_transit.toml"),
        dir.join("reference/test_out.csv").display(),
        data("reference/port_sizes.csv"),
        data("reference/aircraft_types.csv"),
        data("reference/search.toml"),
        data("reference/surface_combat.toml"),
    );
    let path = dir.join("derivation/scenario.toml");
    std::fs::write(&path, scenario).unwrap();
    let mut loaded = sim_data::Scenario::load(&path).expect("the scenario loads");
    let days = loaded.run_to_end().expect("it runs");
    assert_eq!(days.len(), 2);
    let truth = loaded.world.umpire();
    assert_eq!(truth.ships[&ShipId::new(1)].fuel_t, Fixed::from_int(1512));
}

// Designed rate of fire by exception, and ammunition not limiting.

fn rules_with_designed_rate(row: u32) -> String {
    format!(
        "{RULES}\n[[exception]]\nentity = \"gun\"\nfact = \"rate_of_fire_rpm\"\n\
         row = {row}\nreason = \"A designed rate, agreed\"\nagreed = \"2026-11-10\"\n"
    )
}

fn list_with_ammunition(ships: &[&str], ammunition: &[&str]) -> String {
    let quoted = |names: &[&str]| {
        names
            .iter()
            .map(|n| format!("\"{n}\""))
            .collect::<Vec<_>>()
            .join(", ")
    };
    format!(
        "{}main_gun_ammunition_not_limiting = [{}]\n",
        list(ships, &[]),
        quoted(ammunition)
    )
}

fn derive_ammunition(
    ships: &[&str],
    ammunition: &[&str],
    files: &Files,
) -> Result<Outputs, Vec<String>> {
    derive(
        "list.toml",
        &list_with_ammunition(ships, ammunition),
        RULES,
        files,
    )
    .map_err(|problems| problems.iter().map(ToString::to_string).collect())
}

#[test]
fn a_designed_rate_is_read_only_by_exception_when_there_is_no_practical_rate() {
    let mut files = base();
    remove_row(&mut files, "weapon_facts.csv", 2);
    // Without the exception: the old error, unchanged.
    assert_one(
        &fails(&["shp"], &files),
        &["rate_of_fire_practical_rpm of weapon gun: no rows in the facts"],
    );
    let outputs = derive_with_rules(&["shp"], &[], &rules_with_designed_rate(6), &files)
        .unwrap_or_else(|p| panic!("{p:#?}"));
    // The designed rate is 5, not the practical 3.5.
    assert_eq!(
        class_lines(&outputs),
        ["1000,Test class,cruiser,15,32.5,1600,8000,0,8,5,150,19.666666,6,196,400,1"]
    );
    let source = source_of(&outputs, "1000", "gun_rounds_per_min");
    assert!(
        source[5].contains("designed rate, read by exception"),
        "{source:?}"
    );
    assert!(source[4].contains("weapon_facts.csv:6"), "{source:?}");
    // It is listed among the exceptions used even though the row is preferred.
    let header = comments(&outputs.class_csv);
    assert!(
        header.contains("gun rate_of_fire_rpm: weapon_facts.csv row 6 (preferred); reason: A designed rate, agreed; agreed 2026-11-10"),
        "{header}"
    );
}

#[test]
fn a_designed_rate_by_exception_replaces_a_practical_rate_that_is_not_preferred() {
    let mut files = base();
    replace_row(
        &mut files,
        "weapon_facts.csv",
        2,
        fact_row(
            2,
            "gun",
            "rate_of_fire_practical_rpm",
            "3.5",
            "",
            "",
            "low",
            "unconfirmed",
        ),
    );
    let outputs = derive_with_rules(&["shp"], &[], &rules_with_designed_rate(6), &files)
        .unwrap_or_else(|p| panic!("{p:#?}"));
    assert!(class_lines(&outputs)[0].contains(",8,5,150,"));
}

#[test]
fn without_the_exception_a_designed_rate_is_still_not_read() {
    let mut files = base();
    remove_row(&mut files, "weapon_facts.csv", 2);
    // An exception for another weapon, or another fact, does not help.
    let rules = format!(
        "{RULES}\n[[exception]]\nentity = \"big-gun\"\nfact = \"rate_of_fire_rpm\"\nrow = 6\n\
         reason = \"x\"\nagreed = \"2026-11-10\"\n"
    );
    let problems = derive_with_rules(&["shp"], &[], &rules, &files).expect_err("problems");
    let problems: Vec<String> = problems.iter().map(ToString::to_string).collect();
    assert!(
        problems
            .iter()
            .any(|p| p.contains("a theoretical or maximum rate, which the tool does not read")),
        "{problems:#?}"
    );
}

#[test]
fn a_designed_rate_exception_naming_a_missing_row_is_a_problem() {
    let mut files = base();
    remove_row(&mut files, "weapon_facts.csv", 2);
    let problems =
        derive_with_rules(&["shp"], &[], &rules_with_designed_rate(99), &files).expect_err("p");
    let problems: Vec<String> = problems.iter().map(ToString::to_string).collect();
    assert!(
        problems
            .iter()
            .any(|p| p.contains("rules: exception: rate_of_fire_rpm:") && p.contains("row 99")),
        "{problems:#?}"
    );
}

#[test]
fn an_exception_naming_a_designed_rate_row_for_another_weapon_is_a_problem() {
    // Row 6 is the gun's; naming it for the big gun names no row of it.
    let rules = format!(
        "{RULES}\n[[exception]]\nentity = \"big-gun\"\nfact = \"rate_of_fire_rpm\"\nrow = 6\n\
         reason = \"x\"\nagreed = \"2026-11-10\"\n"
    );
    let problems = derive_with_rules(&["shp"], &[], &rules, &base()).expect_err("a problem");
    assert_eq!(problems.len(), 1, "{problems:#?}");
    assert!(problems[0].to_string().contains("row 6"));
}

#[test]
fn ammunition_not_limiting_is_assumed_only_when_no_row_exists() {
    let mut files = base();
    remove_row(&mut files, "ship_class_facts.csv", 6);
    // Not on the list: the usual missing problem.
    assert_one(
        &fails(&["shp"], &files),
        &["shp: missing: gun_rounds_per_gun:"],
    );
    let outputs =
        derive_ammunition(&["shp"], &["shp"], &files).unwrap_or_else(|p| panic!("{p:#?}"));
    assert_eq!(
        class_lines(&outputs),
        ["1000,Test class,cruiser,15,32.5,1600,8000,0,8,3.5,9999,19.666666,6,196,400,1"]
    );
    let source = source_of(&outputs, "1000", "gun_rounds_per_gun");
    assert_eq!(source[4], "");
    assert_eq!(
        source[5],
        "assumption, not a fact: main-gun ammunition is treated as not limiting (decision 0023); 9999 is a marker, not an outfit"
    );
    let header = comments(&outputs.class_csv);
    assert!(
        header.contains("shp: no main_rounds_per_gun row in the facts, main-gun ammunition treated as not limiting"),
        "{header}"
    );
}

#[test]
fn the_ammunition_assumption_has_no_effect_when_a_row_is_readable() {
    let outputs =
        derive_ammunition(&["shp"], &["shp"], &base()).unwrap_or_else(|p| panic!("{p:#?}"));
    assert_eq!(class_lines(&outputs), [BASE_ROW]);
    assert!(
        comments(&outputs.class_csv).contains("Ammunition assumptions used (decision 0023): none")
    );
    assert_eq!(source_of(&outputs, "1000", "gun_rounds_per_gun")[5], "");
}

#[test]
fn a_disputed_ammunition_row_is_still_a_problem_for_a_ship_on_the_list() {
    let mut files = base();
    replace_row(
        &mut files,
        "ship_class_facts.csv",
        6,
        fact_row(
            6,
            "us-test-class",
            "main_rounds_per_gun",
            "150",
            "",
            "",
            "low",
            "disputed",
        ),
    );
    let problems = derive_ammunition(&["shp"], &["shp"], &files).expect_err("a problem");
    assert_one(
        &problems,
        &[
            "shp: not-preferred: gun_rounds_per_gun:",
            "ship_class_facts.csv:6 (disputed",
        ],
    );
}

#[test]
fn an_assumed_ship_gets_its_own_class_row_beside_one_that_reads_the_fact() {
    let mut files = base();
    add_row(
        &mut files,
        "attributes.csv",
        "main_rounds_per_gun,ship,integer,one".to_string(),
    );
    // Move the class's rounds row to a ship row of shp alone, so shp2 has none.
    remove_row(&mut files, "ship_class_facts.csv", 6);
    add_row(
        &mut files,
        "ship_facts.csv",
        fact(2, "shp", "main_rounds_per_gun", "150"),
    );
    let outputs =
        derive_ammunition(&["shp", "shp2"], &["shp2"], &files).unwrap_or_else(|p| panic!("{p:#?}"));
    let lines = class_lines(&outputs);
    assert_eq!(lines.len(), 2, "{lines:#?}");
    assert!(lines.iter().any(|l| l.contains(",150,")));
    assert!(lines.iter().any(|l| l.contains(",9999,")));
}

#[test]
fn an_ammunition_name_that_is_not_in_ships_is_rejected() {
    let problems = derive_ammunition(&["shp"], &["nosuch"], &base()).expect_err("a list problem");
    assert_one(
        &problems,
        &[
            "list: list: main_gun_ammunition_not_limiting:",
            "`nosuch` is not in ships",
        ],
    );
}
