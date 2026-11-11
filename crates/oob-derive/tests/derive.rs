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

// Decision 0029: carrier, anti-aircraft and aircraft figures.
//
// # The air figures of the base ship, worked by hand
//
// `air_base()` adds to the base data set: heavy anti-aircraft `aa-h` x 8;
// light `aa-l` x 8 and `aa-l2` x 10; weights 1 and 0.25.
//
// anti_aircraft = 8 x 1 + (8 + 10) x 0.25 = 8 + 4.5 = 12.5, rounded down: 12.
// aircraft_capacity: no row on a cruiser: 0.
// So the class row is the base row with `,0,12` added.
//
// # The aircraft, worked by hand
//
// Rules: gun 6 kg per damage, torpedo 0.4 per kg, fraction 0.333333, sweep 20.
//
// | Type | Radius | Attack | Protection |
// | --- | --- | --- | --- |
// | fighter-a | combat radius 300 | none, 0 | armour 1, tanks 0 |
// | dive-a | no radius; range 528 x 0.333333 = 175.999824, down: 175 | bomb-a 250 kg / 6 = 41.666666 | 1, 1 |
// | torp-a-plane | no radius; range 759 x 0.333333 = 252.999747, down: 252 | torp-a 200 kg x 0.4 = 80 | 0, 0 |

const AIR_RULES_EXTRA: &str = "anti_aircraft_heavy_weight = \"1\"\n\
    anti_aircraft_light_weight = \"0.25\"\n\
    strike_radius_fraction_of_range = \"0.333333\"\n\
    search_sweep_width_nm = 20\n";

fn air_rules(exceptions: &str) -> String {
    format!("{RULES}{AIR_RULES_EXTRA}{exceptions}")
}

fn exception_text(entity: &str, fact: &str, row: u32) -> String {
    format!(
        "\n[[exception]]\nentity = \"{entity}\"\nfact = \"{fact}\"\nrow = {row}\n\
         reason = \"The architect judged it\"\nagreed = \"2026-11-11\"\n"
    )
}

fn append_lines(files: &mut Files, file: &str, text: &str) {
    files.get_mut(file).expect("a file").push_str(text);
}

/// A fit row whose standing is not preferred, with the low confidence the
/// checker wants for it.
fn weak_fit(line: String) -> String {
    line.replace(",medium,stated,", ",low,stated,")
}

fn air_base() -> Files {
    let mut files = base();
    append_lines(
        &mut files,
        "ship_classes.csv",
        "1002,us-carrier-class,Carrier class,us,carrier\n",
    );
    append_lines(
        &mut files,
        "ships.csv",
        "1003,cvx,Cvx,us,us-carrier-class,\n",
    );
    files.insert(
        "aircraft_types.csv".to_string(),
        "id,key,name,nation,designation,role\n\
         1000,fighter-a,Fighter A,us,F-A,fighter\n\
         1001,dive-a,Dive A,us,D-A,dive_bomber\n\
         1002,torp-a-plane,Torpedo A,us,T-A,torpedo_bomber\n\
         1003,boat-a,Boat A,us,B-A,patrol_flying_boat\n\
         1004,odd-a,Odd A,us,O-A,floatplane_fighter\n\
         1005,heavy-a,Heavy A,us,H-A,heavy_bomber\n\
         1006,land-a,Land A,jp,L-A,land_attack_bomber\n\
         1007,amph-a,Amph A,us,M-A,patrol_amphibian\n"
            .to_string(),
    );
    append_lines(
        &mut files,
        "attributes.csv",
        "aircraft_capacity,ship_class,integer,one\n\
         aircraft_capacity,ship,integer,one\n\
         combat_radius_nm,aircraft_type,integer,many\n\
         range_normal_nm,aircraft_type,integer,one\n\
         pilot_armour,aircraft_type,text,one\n\
         self_sealing_tanks,aircraft_type,text,one\n\
         weight_kg,weapon,decimal,one\n",
    );
    files.insert(
        "weapons.csv".to_string(),
        "id,key,kind\n1000,gun,aa_gun\n1001,tube,torpedo\n1002,big-gun,gun\n\
         1003,bomb-a,bomb\n1004,torp-a,torpedo\n1005,aa-h,aa_gun\n1006,aa-l,aa_gun\n\
         1007,aa-l2,aa_gun\n1008,mg-a,machine_gun\n1009,odd-w,gun\n"
            .to_string(),
    );
    append_lines(
        &mut files,
        "weapon_facts.csv",
        &format!(
            "{}\n{}\n",
            fact(7, "bomb-a", "weight_kg", "250"),
            fact(8, "torp-a", "warhead_kg", "200")
        ),
    );
    append_lines(
        &mut files,
        "fits.csv",
        &format!(
            "{}\n{}\n{}\n",
            class_fit(4, "heavy_aa", "weapon", "aa-h", "8"),
            class_fit(5, "light_aa", "weapon", "aa-l", "8"),
            class_fit(6, "light_aa", "weapon", "aa-l2", "10"),
        ),
    );
    // Aircraft facts.
    let mut n = 0;
    let mut add = |files: &mut Files, entity: &str, attribute: &str, value: &str| {
        n += 1;
        append_lines(
            files,
            "aircraft_type_facts.csv",
            &format!("{}\n", fact(n, entity, attribute, value)),
        );
    };
    add(&mut files, "fighter-a", "combat_radius_nm", "300");
    add(&mut files, "fighter-a", "pilot_armour", "yes");
    add(&mut files, "fighter-a", "self_sealing_tanks", "no");
    add(&mut files, "dive-a", "range_normal_nm", "528");
    add(&mut files, "dive-a", "pilot_armour", "yes");
    add(&mut files, "dive-a", "self_sealing_tanks", "yes");
    add(&mut files, "torp-a-plane", "range_normal_nm", "759");
    add(&mut files, "torp-a-plane", "pilot_armour", "no");
    add(&mut files, "torp-a-plane", "self_sealing_tanks", "no");
    for plane in ["boat-a", "heavy-a", "land-a", "amph-a"] {
        add(&mut files, plane, "range_normal_nm", "600");
        add(&mut files, plane, "pilot_armour", "no");
        add(&mut files, plane, "self_sealing_tanks", "no");
    }
    add(&mut files, "odd-a", "range_normal_nm", "600");
    add(&mut files, "odd-a", "pilot_armour", "no");
    add(&mut files, "odd-a", "self_sealing_tanks", "no");
    for (id, plane, slot, weapon, count) in [
        (7, "dive-a", "bomb_load", "bomb-a", "1"),
        (8, "torp-a-plane", "torpedo_load", "torp-a", "1"),
        (9, "heavy-a", "bomb_load", "bomb-a", "4"),
        (10, "land-a", "torpedo_load", "torp-a", "1"),
        (11, "dive-a", "bomb_load", "aa-h", "2"),
    ] {
        append_lines(
            &mut files,
            "fits.csv",
            &format!(
                "{}\n",
                fit_row(
                    id,
                    "aircraft_type",
                    plane,
                    slot,
                    "weapon",
                    weapon,
                    count,
                    ("", ""),
                    "a",
                    "preferred"
                )
            ),
        );
    }
    files
}

/// A list text with extra top-level keys and extra tables.
fn air_list(ships: &[&str], keys: &str, tables: &str) -> String {
    format!("{}{keys}\n{tables}", list(ships, &[]))
}

fn derive_text(list_text: &str, rules: &str, files: &Files) -> Result<Outputs, Vec<String>> {
    derive("list.toml", list_text, rules, files)
        .map_err(|problems| problems.iter().map(ToString::to_string).collect())
}

#[track_caller]
fn ok_text(list_text: &str, rules: &str, files: &Files) -> Outputs {
    derive_text(list_text, rules, files).unwrap_or_else(|p| panic!("{p:#?}"))
}

fn air_ship(ships: &[&str], files: &Files) -> Result<Outputs, Vec<String>> {
    derive_text(
        &air_list(ships, "air_figures = true\n", ""),
        &air_rules(""),
        files,
    )
}

#[track_caller]
fn air_ok(ships: &[&str], files: &Files) -> Outputs {
    air_ship(ships, files).unwrap_or_else(|p| panic!("{p:#?}"))
}

#[test]
fn the_air_base_data_set_passes_the_checker() {
    let report = oob_check::check_files(&air_base());
    assert!(report.problems.is_empty(), "{:#?}", report.problems);
}

#[test]
fn a_list_without_air_figures_writes_no_new_columns() {
    let files = air_base();
    let outputs = ok_text(&list(&["shp"], &[]), &air_rules(""), &files);
    assert_eq!(class_lines(&outputs), [BASE_ROW]);
    let header = outputs
        .class_csv
        .lines()
        .find(|l| l.starts_with("id,"))
        .unwrap();
    assert!(header.ends_with(",toughness,radar"), "{header}");
    assert!(!comments(&outputs.class_csv).contains("anti_aircraft"));
    assert!(!comments(&outputs.class_csv).contains("not modelled"));
    assert!(outputs.aircraft_csv.is_none());
    assert!(outputs.aircraft_sources_csv.is_none());
}

#[test]
fn air_figures_add_the_two_columns_last() {
    let outputs = air_ok(&["shp"], &air_base());
    let header = outputs
        .class_csv
        .lines()
        .find(|l| l.starts_with("id,"))
        .unwrap();
    assert!(
        header.ends_with(",radar,aircraft_capacity,anti_aircraft"),
        "{header}"
    );
    assert_eq!(class_lines(&outputs), [format!("{BASE_ROW},0,12")]);
    let header = comments(&outputs.class_csv);
    assert!(
        header.contains("anti_aircraft_heavy_weight = 1"),
        "{header}"
    );
    assert!(
        header.contains("anti_aircraft_light_weight = 0.25"),
        "{header}"
    );
}

#[test]
fn anti_aircraft_adds_every_weapon_of_each_slot_and_cites_them_all() {
    let outputs = air_ok(&["shp"], &air_base());
    let row = source_of(&outputs, "1000", "anti_aircraft");
    assert_eq!(row[2], "12");
    assert_eq!(row[4], "fits.csv:4 fits.csv:5 fits.csv:6");
    assert!(row[5].contains("8 heavy, 18 light"), "{row:?}");
    assert!(row[5].contains("exact 12.5, rounded down"), "{row:?}");
}

#[test]
fn a_slot_with_no_rows_is_zero() {
    let mut files = air_base();
    for id in [4, 5, 6] {
        remove_row(&mut files, "fits.csv", id);
    }
    let outputs = air_ok(&["shp"], &files);
    assert_eq!(class_lines(&outputs), [format!("{BASE_ROW},0,0")]);
    let row = source_of(&outputs, "1000", "anti_aircraft");
    assert_eq!(row[4], "");
    assert!(row[5].contains("0 heavy, 0 light"), "{row:?}");
}

#[test]
fn a_ships_own_row_in_a_slot_replaces_the_classs_rows_of_that_slot_only() {
    let mut files = air_base();
    append_lines(
        &mut files,
        "fits.csv",
        &format!(
            "{}\n",
            ship_fit(20, "shp", "heavy_aa", "weapon", "aa-h", "4")
        ),
    );
    let outputs = air_ok(&["shp"], &files);
    // 4 x 1 + 18 x 0.25 = 8.5: 8.
    assert_eq!(class_lines(&outputs), [format!("{BASE_ROW},0,8")]);
    let row = source_of(&outputs, "1000", "anti_aircraft");
    assert_eq!(row[4], "fits.csv:5 fits.csv:6 fits.csv:20");
    assert!(row[5].contains("4 heavy, 18 light"), "{row:?}");
}

#[test]
fn a_ships_own_row_for_another_item_replaces_the_whole_slot() {
    let mut files = air_base();
    append_lines(
        &mut files,
        "fits.csv",
        &format!(
            "{}\n",
            ship_fit(20, "shp", "light_aa", "weapon", "aa-l2", "4")
        ),
    );
    let outputs = air_ok(&["shp"], &files);
    // 8 + 4 x 0.25 = 9.
    assert_eq!(class_lines(&outputs), [format!("{BASE_ROW},0,9")]);
}

#[test]
fn a_ship_row_that_is_not_readable_does_not_replace_the_slot_and_is_noted() {
    let mut files = air_base();
    append_lines(
        &mut files,
        "fits.csv",
        &format!(
            "{}\n",
            weak_fit(fit_row(
                20,
                "ship",
                "shp",
                "heavy_aa",
                "weapon",
                "aa-h",
                "2",
                ("", ""),
                "a",
                "unconfirmed"
            ))
        ),
    );
    let outputs = air_ok(&["shp"], &files);
    assert_eq!(class_lines(&outputs), [format!("{BASE_ROW},0,12")]);
    let row = source_of(&outputs, "1000", "anti_aircraft");
    assert!(row[5].contains("fits.csv:20 (unconfirmed)"), "{row:?}");
}

#[test]
fn rows_for_one_item_that_agree_are_all_cited_and_counted_once() {
    let mut files = air_base();
    append_lines(
        &mut files,
        "fits.csv",
        &format!("{}\n", class_fit(21, "heavy_aa", "weapon", "aa-h", "8")),
    );
    let outputs = air_ok(&["shp"], &files);
    assert_eq!(class_lines(&outputs), [format!("{BASE_ROW},0,12")]);
    assert_eq!(
        source_of(&outputs, "1000", "anti_aircraft")[4],
        "fits.csv:4 fits.csv:5 fits.csv:6 fits.csv:21"
    );
}

#[test]
fn rows_for_one_item_that_disagree_are_a_conflict_naming_them() {
    let mut files = air_base();
    append_lines(
        &mut files,
        "fits.csv",
        &format!("{}\n", class_fit(21, "heavy_aa", "weapon", "aa-h", "6")),
    );
    let problems = air_ship(&["shp"], &files).expect_err("a conflict");
    assert_one(
        &problems,
        &[
            "shp: conflict: anti_aircraft:",
            "heavy_aa fit of class us-test-class, item aa-h",
            "fits.csv:4",
            "fits.csv:21",
        ],
    );
}

#[test]
fn an_exception_naming_one_of_two_disagreeing_rows_settles_it() {
    let mut files = air_base();
    append_lines(
        &mut files,
        "fits.csv",
        &format!("{}\n", class_fit(21, "heavy_aa", "weapon", "aa-h", "6")),
    );
    let rules = air_rules(&exception_text("us-test-class", "heavy_aa", 21));
    let outputs = derive_text(
        &air_list(&["shp"], "air_figures = true\n", ""),
        &rules,
        &files,
    )
    .unwrap_or_else(|p| panic!("{p:#?}"));
    // 6 + 4.5 = 10.5: 10.
    assert_eq!(class_lines(&outputs), [format!("{BASE_ROW},0,10")]);
    assert_eq!(
        source_of(&outputs, "1000", "anti_aircraft")[4],
        "fits.csv:5 fits.csv:6 fits.csv:21"
    );
    assert!(comments(&outputs.class_csv)
        .contains("us-test-class heavy_aa: fits.csv row 21 (preferred)"));
}

#[test]
fn an_item_that_holds_but_is_not_preferred_is_a_problem_and_one_that_does_not_hold_is_ignored() {
    let mut files = air_base();
    append_lines(
        &mut files,
        "fits.csv",
        &format!(
            "{}\n{}\n",
            fit_row(
                22,
                "ship_class",
                "us-test-class",
                "light_aa",
                "weapon",
                "aa-l",
                "9",
                ("1930", "1931"),
                "a",
                "preferred"
            ),
            weak_fit(fit_row(
                23,
                "ship_class",
                "us-test-class",
                "heavy_aa",
                "weapon",
                "gun",
                "3",
                ("", ""),
                "a",
                "unconfirmed"
            )),
        ),
    );
    let problems = air_ship(&["shp"], &files).expect_err("a problem");
    // The 1930 row is a different row of an item that holds by row 5; only
    // item `gun` (unconfirmed, holds) is a problem. Item aa-l has rows 5
    // (holds) and 22 (does not): read as 8.
    assert_one(
        &problems,
        &[
            "shp: not-preferred: anti_aircraft:",
            "item gun",
            "fits.csv:23 (unconfirmed",
        ],
    );
}

#[test]
fn an_item_none_of_whose_rows_holds_on_the_date_is_not_fitted() {
    let mut files = air_base();
    append_lines(
        &mut files,
        "fits.csv",
        &format!(
            "{}\n",
            weak_fit(fit_row(
                22,
                "ship_class",
                "us-test-class",
                "heavy_aa",
                "weapon",
                "gun",
                "3",
                ("1930", "1931"),
                "a",
                "unconfirmed"
            )),
        ),
    );
    let outputs = air_ok(&["shp"], &files);
    assert_eq!(class_lines(&outputs), [format!("{BASE_ROW},0,12")]);
}

#[test]
fn a_row_with_no_count_is_not_a_count_and_is_listed() {
    let mut files = air_base();
    append_lines(
        &mut files,
        "fits.csv",
        &format!("{}\n", class_fit(24, "light_aa", "weapon", "gun", "")),
    );
    let outputs = air_ok(&["shp"], &files);
    assert_eq!(class_lines(&outputs), [format!("{BASE_ROW},0,12")]);
    let row = source_of(&outputs, "1000", "anti_aircraft");
    assert!(row[5].contains("fits.csv:24 count not given"), "{row:?}");
    assert!(!row[4].contains("fits.csv:24"), "{row:?}");
}

#[test]
fn a_ship_row_with_no_count_does_not_replace_the_classs_rows() {
    let mut files = air_base();
    append_lines(
        &mut files,
        "fits.csv",
        &format!(
            "{}\n",
            ship_fit(25, "shp", "heavy_aa", "weapon", "aa-h", "")
        ),
    );
    let outputs = air_ok(&["shp"], &files);
    assert_eq!(class_lines(&outputs), [format!("{BASE_ROW},0,12")]);
}

#[test]
fn a_cruiser_with_no_capacity_row_carries_none_and_a_carrier_with_none_is_missing() {
    let outputs = air_ok(&["shp"], &air_base());
    let row = source_of(&outputs, "1000", "aircraft_capacity");
    assert_eq!(row[2], "0");
    assert!(row[5].contains("none carried"), "{row:?}");

    // A carrier needs the fact; give it the other figures too.
    let mut files = air_base();
    for (id, attribute, value) in [
        (20, "max_speed_kn", "32"),
        (21, "endurance_speed_kn", "15"),
        (22, "endurance_nm", "9000"),
        (23, "fuel_capacity_lt", "5000"),
        (24, "displacement_standard_lt", "20000"),
    ] {
        append_lines(
            &mut files,
            "ship_class_facts.csv",
            &format!("{}\n", fact(id, "us-carrier-class", attribute, value)),
        );
    }
    let problems = air_ship(&["cvx"], &files).expect_err("missing");
    let capacity: Vec<&String> = problems
        .iter()
        .filter(|p| p.contains("aircraft_capacity"))
        .collect();
    assert_eq!(capacity.len(), 1, "{problems:#?}");
    assert!(
        capacity[0].contains("cvx: missing: aircraft_capacity:"),
        "{problems:#?}"
    );
}

#[test]
fn a_carriers_capacity_is_read_from_its_class_fact() {
    let mut files = air_base();
    for (id, attribute, value) in [
        (20, "max_speed_kn", "32"),
        (21, "endurance_speed_kn", "15"),
        (22, "endurance_nm", "9000"),
        (23, "fuel_capacity_lt", "5000"),
        (24, "displacement_standard_lt", "20000"),
        (25, "aircraft_capacity", "72"),
        (26, "main_rounds_per_gun", "100"),
    ] {
        append_lines(
            &mut files,
            "ship_class_facts.csv",
            &format!("{}\n", fact(id, "us-carrier-class", attribute, value)),
        );
    }
    for (id, slot, item, count) in [
        (30, "main_battery", "gun", "8"),
        (31, "torpedo_tubes", "tube", "0"),
        (32, "surface_search_radar", "radar-a", "1"),
    ] {
        append_lines(
            &mut files,
            "fits.csv",
            &format!(
                "{}\n",
                fit_row(
                    id,
                    "ship_class",
                    "us-carrier-class",
                    slot,
                    if slot == "surface_search_radar" {
                        "sensor"
                    } else {
                        "weapon"
                    },
                    item,
                    count,
                    ("", ""),
                    "a",
                    "preferred"
                )
            ),
        );
    }
    let outputs = air_ok(&["cvx"], &files);
    let row = source_of(&outputs, "1000", "aircraft_capacity");
    assert_eq!(row[2], "72");
    assert_eq!(row[4], "ship_class_facts.csv:25");
    assert!(
        class_lines(&outputs)[0].ends_with(",72,0"),
        "{:?}",
        class_lines(&outputs)
    );
}

#[test]
fn a_capacity_row_that_cannot_be_read_is_a_problem_even_without_a_carrier() {
    let mut files = air_base();
    append_lines(
        &mut files,
        "ship_class_facts.csv",
        &format!(
            "{}\n",
            fact_row(
                30,
                "us-test-class",
                "aircraft_capacity",
                "3",
                "",
                "",
                "low",
                "unconfirmed"
            )
        ),
    );
    let problems = air_ship(&["shp"], &files).expect_err("a problem");
    assert_one(
        &problems,
        &[
            "shp: not-preferred: aircraft_capacity:",
            "ship_class_facts.csv:30",
        ],
    );
}

#[test]
fn two_different_radar_sets_do_not_conflict() {
    let mut files = air_base();
    append_lines(
        &mut files,
        "fits.csv",
        &format!(
            "{}\n",
            class_fit(40, "surface_search_radar", "sensor", "radar-b", "1")
        ),
    );
    let outputs = derive_text(&list(&["shp"], &[]), &air_rules(""), &files)
        .unwrap_or_else(|p| panic!("{p:#?}"));
    assert_eq!(class_lines(&outputs), [BASE_ROW]);
    assert_eq!(
        source_of(&outputs, "1000", "radar")[4],
        "fits.csv:3 fits.csv:40"
    );
}

#[test]
fn a_fitted_set_and_a_sourced_none_at_once_are_a_conflict() {
    let mut files = air_base();
    append_lines(
        &mut files,
        "fits.csv",
        &format!(
            "{}\n",
            class_fit(40, "air_search_radar", "sensor", "jp-radar-any", "0")
        ),
    );
    let problems = derive_text(&list(&["shp"], &[]), &air_rules(""), &files).expect_err("conflict");
    assert_one(
        &problems,
        &["shp: conflict: radar:", "fits.csv:3", "fits.csv:40"],
    );
}

#[test]
fn a_row_naming_a_real_set_with_no_count_means_radar() {
    let mut files = air_base();
    remove_row(&mut files, "fits.csv", 3);
    append_lines(
        &mut files,
        "fits.csv",
        &format!(
            "{}\n",
            class_fit(40, "air_search_radar", "sensor", "radar-a", "")
        ),
    );
    let outputs = derive_text(&list(&["shp"], &[]), &air_rules(""), &files)
        .unwrap_or_else(|p| panic!("{p:#?}"));
    assert_eq!(class_lines(&outputs), [BASE_ROW]);
    assert_eq!(source_of(&outputs, "1000", "radar")[4], "fits.csv:40");
}

// Surface weapons not modelled.

fn not_modelled_ok(ships: &[&str], keys: &str, files: &Files) -> Outputs {
    ok_text(&air_list(ships, keys, ""), &air_rules(""), files)
}

const NOT_MODELLED_ROW_TAIL: &str = "0,0,0,0,0,0,400,1";

#[test]
fn all_ships_not_modelled_get_zero_surface_weapons_without_reading_any_fact() {
    let mut files = base();
    // No weapons facts or fits at all for the main battery or tubes.
    remove_row(&mut files, "fits.csv", 1);
    remove_row(&mut files, "fits.csv", 2);
    let outputs = not_modelled_ok(
        &["shp", "shp2"],
        "surface_weapons_not_modelled = \"all\"\n",
        &files,
    );
    assert_eq!(
        class_lines(&outputs),
        [format!(
            "1000,Test class,cruiser,15,32.5,1600,8000,0,{NOT_MODELLED_ROW_TAIL}"
        )]
    );
    for figure in [
        "main_guns",
        "gun_rounds_per_min",
        "gun_rounds_per_gun",
        "gun_damage",
        "torpedo_tubes",
        "torpedo_damage",
    ] {
        let row = source_of(&outputs, "1000", figure);
        assert_eq!(row[2], "0", "{figure}");
        assert_eq!(row[4], "", "{figure}");
        assert_eq!(
            row[5],
            "assumption of the scenario, not a fact: surface weapons not modelled (decision 0029)",
            "{figure}"
        );
    }
    let header = comments(&outputs.class_csv);
    assert!(
        header.contains("Surface weapons not modelled (decision 0029, 4)"),
        "{header}"
    );
    assert!(
        header.contains("  shp\n# ") || header.contains("  shp"),
        "{header}"
    );
    assert!(header.contains("  shp2"), "{header}");
    assert_eq!(source_of(&outputs, "1000", "toughness")[5], "");
}

#[test]
fn ships_not_modelled_by_name_get_a_separate_class_row() {
    let outputs = not_modelled_ok(
        &["shp", "shp2"],
        "surface_weapons_not_modelled = [\"shp2\"]\n",
        &base(),
    );
    let lines: Vec<String> = class_lines(&outputs)
        .iter()
        .map(|l| l.split_once(',').expect("an id").1.to_string())
        .collect();
    assert_eq!(lines.len(), 2, "{lines:#?}");
    assert!(
        lines.contains(&BASE_ROW.split_once(',').unwrap().1.to_string()),
        "{lines:#?}"
    );
    assert!(
        lines.contains(&format!(
            "\"Test class, surface weapons not modelled\",cruiser,15,32.5,1600,8000,0,{NOT_MODELLED_ROW_TAIL}"
        )),
        "{lines:#?}"
    );
    let ships = records(&outputs.ships_csv);
    assert_eq!(ships[0][0], "shp");
    assert_eq!(ships[1][0], "shp2");
    assert_ne!(ships[0][1], ships[1][1]);
    let header = comments(&outputs.class_csv);
    assert!(header.contains("  shp2"), "{header}");
    assert!(
        !header.contains("  shp\n") && !header.ends_with("  shp"),
        "{header}"
    );
}

#[test]
fn a_not_modelled_name_that_is_not_in_ships_is_rejected() {
    let problems = derive_text(
        &air_list(
            &["shp"],
            "surface_weapons_not_modelled = [\"nobody\"]\n",
            "",
        ),
        &air_rules(""),
        &base(),
    )
    .expect_err("a problem");
    assert_one(
        &problems,
        &[
            "list: list: surface_weapons_not_modelled:",
            "`nobody` is not in ships",
        ],
    );
    let problems = derive_text(
        &air_list(&["shp"], "surface_weapons_not_modelled = \"some\"\n", ""),
        &air_rules(""),
        &base(),
    )
    .expect_err("a problem");
    assert_one(&problems, &["surface_weapons_not_modelled", "\"all\""]);
}

// Exceptions and years.

fn speed_row(from: &str, to: &str, standing: &str, confidence: &str) -> String {
    fact_row(
        1,
        "us-test-class",
        "max_speed_kn",
        "32.5",
        from,
        to,
        confidence,
        standing,
    )
}

#[test]
fn a_year_only_row_is_ambiguous_until_an_exception_names_it() {
    let files = with_class_speed_validity("1942", "1943");
    let problems = fails(&["shp"], &files);
    assert_one(
        &problems,
        &[
            "shp: ambiguous-date: max_speed_kn:",
            "ship_class_facts.csv:1",
        ],
    );

    let rules = format!(
        "{RULES}{}",
        exception_text("us-test-class", "max_speed_kn", 1)
    );
    let outputs =
        derive_with_rules(&["shp"], &[], &rules, &files).unwrap_or_else(|p| panic!("{p:#?}"));
    assert_eq!(class_lines(&outputs), [BASE_ROW]);
    let row = source_of(&outputs, "1000", "max_speed_kn");
    assert_eq!(row[4], "ship_class_facts.csv:1");
    assert_eq!(row[5], "rests on an exception");
    assert!(
        comments(&outputs.class_csv)
            .contains("us-test-class max_speed_kn: ship_class_facts.csv row 1 (preferred); reason: The architect judged it; agreed 2026-11-11"),
        "{}",
        comments(&outputs.class_csv)
    );
}

#[test]
fn an_exception_does_not_make_a_year_only_row_hold_when_it_clearly_does_not() {
    let rules = format!(
        "{RULES}{}",
        exception_text("us-test-class", "max_speed_kn", 1)
    );
    for (from, to) in [("1943", "1944"), ("", "1941"), ("1943", "")] {
        let files = with_class_speed_validity(from, to);
        let problems = derive_with_rules(&["shp"], &[], &rules, &files).expect_err("not on date");
        assert_one(
            &problems,
            &["shp: not-on-date: max_speed_kn:", "ship_class_facts.csv:1"],
        );
    }
}

#[test]
fn an_exception_on_a_year_only_row_that_is_not_preferred_is_read_too() {
    let mut files = base();
    replace_row(
        &mut files,
        "ship_class_facts.csv",
        1,
        speed_row("1942", "1943", "unconfirmed", "low"),
    );
    let rules = format!(
        "{RULES}{}",
        exception_text("us-test-class", "max_speed_kn", 1)
    );
    let outputs =
        derive_with_rules(&["shp"], &[], &rules, &files).unwrap_or_else(|p| panic!("{p:#?}"));
    assert_eq!(class_lines(&outputs), [BASE_ROW]);
}

#[test]
fn a_ship_row_with_a_year_only_date_named_by_an_exception_replaces_the_class_row() {
    let mut files = base();
    append_lines(
        &mut files,
        "ship_facts.csv",
        &format!(
            "{}\n",
            fact_row(
                1,
                "shp",
                "max_speed_kn",
                "30",
                "1942",
                "1943",
                "medium",
                "preferred"
            )
        ),
    );
    let problems = fails(&["shp"], &files);
    assert_one(
        &problems,
        &["shp: ambiguous-date: max_speed_kn:", "ship_facts.csv:1"],
    );
    let rules = format!("{RULES}{}", exception_text("shp", "max_speed_kn", 1));
    let outputs =
        derive_with_rules(&["shp"], &[], &rules, &files).unwrap_or_else(|p| panic!("{p:#?}"));
    assert!(
        class_lines(&outputs)[0].contains(",30,"),
        "{:?}",
        class_lines(&outputs)
    );
}

// Aircraft types.

const AIRCRAFT_HEADER: &str =
    "id,name,role,search_range_nm,sweep_width_nm,strike_range_nm,attack_weapon,attack_damage,pilot_armour,self_sealing_tanks";

fn plane_list(planes: &[(&str, Option<&str>)]) -> String {
    let tables: String = planes
        .iter()
        .map(|(kind, load)| match load {
            Some(load) => format!("[[aircraft]]\ntype = \"{kind}\"\nload = \"{load}\"\n"),
            None => format!("[[aircraft]]\ntype = \"{kind}\"\n"),
        })
        .collect();
    air_list(
        &["shp"],
        "aircraft_output = \"../reference/test_aircraft.csv\"\nfirst_aircraft_type_id = 2000\n",
        &tables,
    )
}

fn planes(planes: &[(&str, Option<&str>)], files: &Files) -> Result<Outputs, Vec<String>> {
    derive_text(&plane_list(planes), &air_rules(""), files)
}

#[track_caller]
fn planes_ok(list: &[(&str, Option<&str>)], files: &Files) -> Outputs {
    planes(list, files).unwrap_or_else(|p| panic!("{p:#?}"))
}

fn aircraft_lines(outputs: &Outputs) -> Vec<String> {
    outputs
        .aircraft_csv
        .as_ref()
        .expect("an aircraft file")
        .lines()
        .filter(|l| !l.starts_with('#'))
        .map(str::to_string)
        .collect()
}

fn plane_source(outputs: &Outputs, type_id: &str, figure: &str) -> Vec<String> {
    records(
        outputs
            .aircraft_sources_csv
            .as_ref()
            .expect("a sources file"),
    )
    .into_iter()
    .find(|r| r[0] == type_id && r[1] == figure)
    .unwrap_or_else(|| panic!("no sources row for {type_id} {figure}"))
}

#[test]
fn the_aircraft_file_has_its_header_and_a_row_per_type_with_hand_worked_figures() {
    let outputs = planes_ok(
        &[
            ("fighter-a", None),
            ("dive-a", Some("bomb-a")),
            ("torp-a-plane", Some("torp-a")),
        ],
        &air_base(),
    );
    assert_eq!(
        aircraft_lines(&outputs),
        [
            AIRCRAFT_HEADER,
            "2000,Fighter A,fighter,300,20,300,none,0,1,0",
            "2001,Dive A,bomber,175,20,175,bomb,41.666666,1,1",
            "2002,Torpedo A,bomber,252,20,252,torpedo,80,0,0",
        ]
    );
    let csv = outputs.aircraft_csv.as_ref().unwrap();
    assert!(csv.starts_with("# GENERATED by oob-derive from the facts in data/oob. Do not edit.\n"));
    let header = comments(csv);
    assert!(
        header.contains("strike_radius_fraction_of_range = 0.333333"),
        "{header}"
    );
    assert!(header.contains("search_sweep_width_nm = 20"), "{header}");
    assert!(
        header.contains("Exceptions used (decision 0020, 3 and 0029, 5): none"),
        "{header}"
    );
    let sources = outputs.aircraft_sources_csv.as_ref().unwrap();
    assert!(sources.starts_with("# GENERATED by oob-derive. Do not edit.\n"));
    assert!(sources.contains("type_id,figure,value,rule,facts,note"));

    // The sources.
    let radius = plane_source(&outputs, "2000", "strike_range_nm");
    assert_eq!(radius[2], "300");
    assert!(
        radius[3].starts_with("combat_radius_nm of the type"),
        "{radius:?}"
    );
    assert_eq!(radius[4], "aircraft_type_facts.csv:1");
    let radius = plane_source(&outputs, "2001", "search_range_nm");
    assert_eq!(radius[2], "175");
    assert!(radius[3].contains("range_normal_nm"), "{radius:?}");
    assert_eq!(radius[4], "aircraft_type_facts.csv:4");
    assert!(
        radius[5].contains("exact 175.999824, rounded down"),
        "{radius:?}"
    );
    assert!(
        radius[5].contains("combat_radius_nm not readable"),
        "{radius:?}"
    );
    let radius = plane_source(&outputs, "2002", "strike_range_nm");
    assert!(
        radius[5].contains("exact 252.999747, rounded down"),
        "{radius:?}"
    );
    let damage = plane_source(&outputs, "2001", "attack_damage");
    assert_eq!(damage[2], "41.666666");
    assert_eq!(damage[4], "fits.csv:7 weapon_facts.csv:7");
    let damage = plane_source(&outputs, "2002", "attack_damage");
    assert_eq!(damage[4], "fits.csv:8 weapon_facts.csv:8");
    let weapon = plane_source(&outputs, "2001", "attack_weapon");
    assert_eq!(
        weapon[4], "fits.csv:7",
        "rows for other weapons are not cited"
    );
    assert_eq!(plane_source(&outputs, "2000", "attack_weapon")[2], "none");
    assert_eq!(plane_source(&outputs, "2000", "sweep_width_nm")[4], "");
    assert_eq!(
        plane_source(&outputs, "2001", "pilot_armour")[4],
        "aircraft_type_facts.csv:5"
    );
    assert_eq!(
        plane_source(&outputs, "2000", "role")[4],
        "aircraft_types.csv:1000"
    );
}

#[test]
fn every_facts_role_maps_to_a_simulation_role() {
    let outputs = planes_ok(
        &[
            ("boat-a", None),
            ("amph-a", None),
            ("heavy-a", Some("bomb-a")),
            ("land-a", Some("torp-a")),
        ],
        &air_base(),
    );
    let lines = aircraft_lines(&outputs);
    assert_eq!(lines[1], "2000,Boat A,patrol,199,20,199,none,0,0,0");
    assert_eq!(lines[2], "2001,Amph A,patrol,199,20,199,none,0,0,0");
    assert_eq!(
        lines[3],
        "2002,Heavy A,bomber,199,20,199,bomb,41.666666,0,0"
    );
    assert_eq!(lines[4], "2003,Land A,bomber,199,20,199,torpedo,80,0,0");
}

#[test]
fn a_role_with_no_simulation_role_is_a_problem_naming_it() {
    let problems = planes(&[("odd-a", None)], &air_base()).expect_err("a problem");
    assert_one(
        &problems,
        &["odd-a: unmapped-type: role:", "floatplane_fighter"],
    );
}

#[test]
fn a_patrol_aircraft_may_carry_a_load() {
    let outputs = planes_ok(&[("boat-a", Some("bomb-a"))], &{
        let mut files = air_base();
        append_lines(
            &mut files,
            "fits.csv",
            &format!(
                "{}\n",
                fit_row(
                    12,
                    "aircraft_type",
                    "boat-a",
                    "bomb_load",
                    "weapon",
                    "bomb-a",
                    "2",
                    ("", ""),
                    "a",
                    "preferred"
                )
            ),
        );
        files
    });
    assert_eq!(
        aircraft_lines(&outputs)[1],
        "2000,Boat A,patrol,199,20,199,bomb,41.666666,0,0"
    );
}

#[test]
fn a_fighter_with_a_load_and_a_bomber_without_one_are_problems() {
    let problems = planes(&[("fighter-a", Some("bomb-a"))], &air_base()).expect_err("a problem");
    assert_one(
        &problems,
        &["fighter-a: bad-value: attack_weapon:", "fighter"],
    );
    let problems = planes(&[("dive-a", None)], &air_base()).expect_err("a problem");
    assert_one(
        &problems,
        &["dive-a: bad-value: attack_weapon:", "bomber", "no load"],
    );
}

#[test]
fn a_load_the_facts_do_not_hold_for_the_type_is_a_problem() {
    // bomb-a is in the facts, but not as a load of the torpedo bomber.
    let problems = planes(&[("torp-a-plane", Some("bomb-a"))], &air_base()).expect_err("a problem");
    assert_one(
        &problems,
        &[
            "torp-a-plane: missing: attack_weapon:",
            "no bomb_load or torpedo_load fit row names `bomb-a`",
        ],
    );
    // A weapon that is not a weapon at all.
    let problems = planes(&[("dive-a", Some("nothing"))], &air_base()).expect_err("a problem");
    assert_one(&problems, &["dive-a: unknown-type: load:", "`nothing`"]);
    let problems = planes(&[("no-such-plane", None)], &air_base()).expect_err("a problem");
    assert_one(&problems, &["no-such-plane: unknown-type: aircraft:"]);
}

#[test]
fn a_load_whose_weapon_has_no_weight_is_missing_its_damage() {
    // dive-a has a bomb_load row for aa-h, which has no weight_kg.
    let problems = planes(&[("dive-a", Some("aa-h"))], &air_base()).expect_err("a problem");
    assert_one(
        &problems,
        &[
            "dive-a: missing: attack_damage:",
            "weight_kg of weapon aa-h",
        ],
    );
}

#[test]
fn rows_of_a_load_that_differ_only_in_count_agree() {
    let mut files = air_base();
    append_lines(
        &mut files,
        "fits.csv",
        &format!(
            "{}\n",
            fit_row(
                12,
                "aircraft_type",
                "dive-a",
                "bomb_load",
                "weapon",
                "bomb-a",
                "2",
                ("", ""),
                "b",
                "preferred"
            )
        ),
    );
    let outputs = planes_ok(&[("dive-a", Some("bomb-a"))], &files);
    assert_eq!(
        plane_source(&outputs, "2000", "attack_weapon")[4],
        "fits.csv:7 fits.csv:12"
    );
}

#[test]
fn a_sourced_radius_is_used_where_exactly_one_value_can_be_read() {
    let mut files = air_base();
    // Two preferred rows that agree are fine, and both are cited.
    append_lines(
        &mut files,
        "aircraft_type_facts.csv",
        &format!("{}\n", fact(30, "dive-a", "combat_radius_nm", "210")),
    );
    append_lines(
        &mut files,
        "aircraft_type_facts.csv",
        &format!("{}\n", fact(31, "dive-a", "combat_radius_nm", "210")),
    );
    let outputs = planes_ok(&[("dive-a", Some("bomb-a"))], &files);
    assert_eq!(
        aircraft_lines(&outputs)[1],
        "2000,Dive A,bomber,210,20,210,bomb,41.666666,1,1"
    );
    assert_eq!(
        plane_source(&outputs, "2000", "strike_range_nm")[4],
        "aircraft_type_facts.csv:30 aircraft_type_facts.csv:31"
    );
}

#[test]
fn preferred_radii_that_disagree_are_a_conflict_until_an_exception_names_one() {
    let mut files = air_base();
    for (id, value) in [(30, "210"), (31, "250")] {
        append_lines(
            &mut files,
            "aircraft_type_facts.csv",
            &format!("{}\n", fact(id, "dive-a", "combat_radius_nm", value)),
        );
    }
    let problems = planes(&[("dive-a", Some("bomb-a"))], &files).expect_err("a conflict");
    assert_one(
        &problems,
        &[
            "dive-a: conflict: strike_range_nm:",
            "aircraft_type_facts.csv:30",
            "aircraft_type_facts.csv:31",
        ],
    );
    let rules = air_rules(&exception_text("dive-a", "combat_radius_nm", 31));
    let outputs = derive_text(&plane_list(&[("dive-a", Some("bomb-a"))]), &rules, &files)
        .unwrap_or_else(|p| panic!("{p:#?}"));
    assert_eq!(
        aircraft_lines(&outputs)[1],
        "2000,Dive A,bomber,250,20,250,bomb,41.666666,1,1"
    );
    let radius = plane_source(&outputs, "2000", "search_range_nm");
    assert_eq!(radius[4], "aircraft_type_facts.csv:31");
    assert_eq!(radius[5], "rests on an exception");
    let header = comments(outputs.aircraft_csv.as_ref().unwrap());
    assert!(
        header.contains("dive-a combat_radius_nm: aircraft_type_facts.csv row 31 (preferred)"),
        "{header}"
    );
}

#[test]
fn a_radius_row_that_is_not_readable_falls_back_to_the_range_and_says_so() {
    let mut files = air_base();
    append_lines(
        &mut files,
        "aircraft_type_facts.csv",
        &format!(
            "{}\n",
            fact_row(
                30,
                "dive-a",
                "combat_radius_nm",
                "999",
                "",
                "",
                "low",
                "unconfirmed"
            )
        ),
    );
    let outputs = planes_ok(&[("dive-a", Some("bomb-a"))], &files);
    assert_eq!(
        aircraft_lines(&outputs)[1],
        "2000,Dive A,bomber,175,20,175,bomb,41.666666,1,1"
    );
    let note = &plane_source(&outputs, "2000", "strike_range_nm")[5];
    assert!(note.contains("combat_radius_nm not readable"), "{note}");
    assert!(
        note.contains("aircraft_type_facts.csv:30 (unconfirmed"),
        "{note}"
    );
}

#[test]
fn the_range_fraction_rounds_down_and_759_gives_252() {
    // 528 x 0.333333 = 175.999824; 759 x 0.333333 = 252.999747.
    let outputs = planes_ok(
        &[("dive-a", Some("bomb-a")), ("torp-a-plane", Some("torp-a"))],
        &air_base(),
    );
    let lines = aircraft_lines(&outputs);
    assert!(
        lines[1].starts_with("2000,Dive A,bomber,175,20,175,"),
        "{lines:?}"
    );
    assert!(
        lines[2].starts_with("2001,Torpedo A,bomber,252,20,252,"),
        "{lines:?}"
    );
}

#[test]
fn with_neither_a_radius_nor_a_range_there_is_a_problem() {
    let mut files = air_base();
    remove_row(&mut files, "aircraft_type_facts.csv", 4);
    let problems = planes(&[("dive-a", Some("bomb-a"))], &files).expect_err("a problem");
    assert_one(
        &problems,
        &[
            "dive-a: missing: strike_range_nm:",
            "combat_radius_nm",
            "range_normal_nm",
        ],
    );
}

#[test]
fn protection_flags_are_yes_or_no_and_anything_else_is_a_problem() {
    let mut files = air_base();
    replace_row(
        &mut files,
        "aircraft_type_facts.csv",
        5,
        fact(5, "dive-a", "pilot_armour", "partial"),
    );
    let problems = planes(&[("dive-a", Some("bomb-a"))], &files).expect_err("a problem");
    assert_one(
        &problems,
        &["dive-a: bad-value: pilot_armour:", "`partial`"],
    );

    let mut files = air_base();
    remove_row(&mut files, "aircraft_type_facts.csv", 3);
    let problems = planes(&[("fighter-a", None)], &files).expect_err("missing");
    assert_one(&problems, &["fighter-a: missing: self_sealing_tanks:"]);
}

#[test]
fn a_year_only_protection_row_needs_an_exception() {
    let mut files = air_base();
    replace_row(
        &mut files,
        "aircraft_type_facts.csv",
        5,
        fact_row(
            5,
            "dive-a",
            "pilot_armour",
            "yes",
            "1942",
            "",
            "medium",
            "preferred",
        ),
    );
    let problems = planes(&[("dive-a", Some("bomb-a"))], &files).expect_err("ambiguous");
    assert_one(
        &problems,
        &[
            "dive-a: ambiguous-date: pilot_armour:",
            "aircraft_type_facts.csv:5",
        ],
    );
    let rules = air_rules(&exception_text("dive-a", "pilot_armour", 5));
    let outputs = derive_text(&plane_list(&[("dive-a", Some("bomb-a"))]), &rules, &files)
        .unwrap_or_else(|p| panic!("{p:#?}"));
    assert_eq!(
        aircraft_lines(&outputs)[1],
        "2000,Dive A,bomber,175,20,175,bomb,41.666666,1,1"
    );
    assert_eq!(
        plane_source(&outputs, "2000", "pilot_armour")[5],
        "rests on an exception"
    );
}

#[test]
fn exceptions_may_name_rows_of_the_aircraft_facts_and_must_name_real_ones() {
    let rules = air_rules(&exception_text("dive-a", "pilot_armour", 99));
    let problems = derive_text(
        &plane_list(&[("dive-a", Some("bomb-a"))]),
        &rules,
        &air_base(),
    )
    .expect_err("a problem");
    assert_one(&problems, &["rules: exception: pilot_armour:", "row 99"]);
    let rules = air_rules(&exception_text("dive-a", "pilot_armour", 5));
    derive_text(
        &plane_list(&[("dive-a", Some("bomb-a"))]),
        &rules,
        &air_base(),
    )
    .unwrap_or_else(|p| panic!("{p:#?}"));
}

#[test]
fn an_exception_may_read_an_unconfirmed_weapon_weight() {
    let mut files = air_base();
    replace_row(
        &mut files,
        "weapon_facts.csv",
        7,
        fact_row(
            7,
            "bomb-a",
            "weight_kg",
            "250",
            "",
            "",
            "low",
            "unconfirmed",
        ),
    );
    let problems = planes(&[("dive-a", Some("bomb-a"))], &files).expect_err("not preferred");
    assert_one(
        &problems,
        &[
            "dive-a: not-preferred: attack_damage:",
            "weapon_facts.csv:7",
        ],
    );
    let rules = air_rules(&exception_text("bomb-a", "weight_kg", 7));
    let outputs = derive_text(&plane_list(&[("dive-a", Some("bomb-a"))]), &rules, &files)
        .unwrap_or_else(|p| panic!("{p:#?}"));
    assert_eq!(
        plane_source(&outputs, "2000", "attack_damage")[5],
        "rests on an exception; load bomb-a"
    );
}

#[test]
fn aircraft_list_problems_are_reported() {
    // Entries without an output path and an id.
    let problems = derive_text(
        &air_list(&["shp"], "", "[[aircraft]]\ntype = \"fighter-a\"\n"),
        &air_rules(""),
        &air_base(),
    )
    .expect_err("problems");
    assert!(
        problems
            .iter()
            .any(|p| p.contains("list: list: aircraft_output:")),
        "{problems:#?}"
    );
    assert!(
        problems
            .iter()
            .any(|p| p.contains("list: list: first_aircraft_type_id:")),
        "{problems:#?}"
    );
    // Twice the same type.
    let problems =
        planes(&[("fighter-a", None), ("fighter-a", None)], &air_base()).expect_err("problems");
    assert_one(&problems, &["list: list: aircraft:", "more than once"]);
    // An output without entries.
    let problems = derive_text(
        &air_list(&["shp"], "aircraft_output = \"../reference/x.csv\"\n", ""),
        &air_rules(""),
        &air_base(),
    )
    .expect_err("problems");
    assert_one(
        &problems,
        &["list: list: aircraft_output:", "no [[aircraft]]"],
    );
}

#[test]
fn the_rules_file_must_hold_the_constants_a_list_needs() {
    let problems =
        derive_text(&plane_list(&[("fighter-a", None)]), RULES, &air_base()).expect_err("problems");
    for name in ["strike_radius_fraction_of_range", "search_sweep_width_nm"] {
        assert!(
            problems
                .iter()
                .any(|p| p.contains(&format!("rules: rules: {name}:"))),
            "{problems:#?}"
        );
    }
    let problems = derive_text(
        &air_list(&["shp"], "air_figures = true\n", ""),
        RULES,
        &air_base(),
    )
    .expect_err("problems");
    assert_eq!(problems.len(), 2, "{problems:#?}");
    // A constant that is not a number is a rules problem.
    let rules = format!("{RULES}strike_radius_fraction_of_range = 0.5\n");
    let problems =
        derive_text(&plane_list(&[("fighter-a", None)]), &rules, &air_base()).expect_err("a float");
    assert!(
        problems.iter().any(|p| p.contains("TOML float")),
        "{problems:#?}"
    );
}

#[test]
fn a_run_writes_the_aircraft_file_and_its_sources_beside_the_others() {
    let dir = scratch("aircraft");
    std::fs::write(
        dir.join("derivation/list.toml"),
        plane_list(&[("fighter-a", None), ("dive-a", Some("bomb-a"))]),
    )
    .unwrap();
    std::fs::write(dir.join("derivation/rules.toml"), air_rules("")).unwrap();
    for (name, text) in &air_base() {
        std::fs::write(dir.join("oob").join(name), text).unwrap();
    }
    match go(&dir, true) {
        Outcome::Clean { written, files } => {
            assert!(!written);
            assert_eq!(files.len(), 5, "{files:?}");
        }
        other => panic!("{other:?}"),
    }
    assert!(reference_files(&dir).is_empty());
    match go(&dir, false) {
        Outcome::Clean { written, files } => {
            assert!(written);
            assert_eq!(files.len(), 5);
        }
        other => panic!("{other:?}"),
    }
    assert_eq!(
        reference_files(&dir),
        [
            "test_aircraft.csv",
            "test_aircraft.sources.csv",
            "test_out.csv",
            "test_out.ships.csv",
            "test_out.sources.csv"
        ]
    );
    let text = std::fs::read_to_string(dir.join("reference/test_aircraft.csv")).unwrap();
    assert!(!text.contains("/Users"));
    let first = std::fs::read(dir.join("reference/test_aircraft.sources.csv")).unwrap();
    assert!(matches!(go(&dir, false), Outcome::Clean { .. }));
    assert_eq!(
        first,
        std::fs::read(dir.join("reference/test_aircraft.sources.csv")).unwrap()
    );
}

// A list with no ships, and a radius read from a row the list names.
//
// # Worked by hand
//
// `dive-a` has range 528 and no readable radius, so the usual rule gives
// 528 x 0.333333 = 175.999824, down: 175. A weak row 40 says 400 for
// 1942-08-07 only (the list's date is 1942-08-08). The list names row 40
// for `dive-a`: radius 400, search 400, strike 400, whatever the date and
// standing of the row. A rules-file exception naming row 41 (preferred,
// 300) for the same figure is not used for that list.

fn shipless_list(tables: &str) -> String {
    format!(
        "name = \"Test planes\"\ndate = \"{DATE}\"\nrules = \"rules.toml\"\nships = []\n\
         aircraft_output = \"../reference/test_aircraft.csv\"\nfirst_aircraft_type_id = 2100\n\
         {tables}"
    )
}

fn radius_files() -> Files {
    let mut files = air_base();
    append_lines(
        &mut files,
        "aircraft_type_facts.csv",
        &format!(
            "{}\n{}\n",
            fact_row(
                40,
                "dive-a",
                "combat_radius_nm",
                "400",
                "1942-08-07",
                "1942-08-07",
                "low",
                "unconfirmed"
            ),
            fact(41, "dive-a", "combat_radius_nm", "300"),
        ),
    );
    files
}

fn chosen(row: &str, reason: &str) -> String {
    let mut text = String::from("[[aircraft]]\ntype = \"dive-a\"\nload = \"bomb-a\"\n");
    if !row.is_empty() {
        text.push_str(&format!("combat_radius_row = {row}\n"));
    }
    if !reason.is_empty() {
        text.push_str(&format!("combat_radius_reason = \"{reason}\"\n"));
    }
    shipless_list(&text)
}

#[test]
fn a_list_with_no_ships_writes_only_the_aircraft_files() {
    let files = air_base();
    let text = shipless_list("[[aircraft]]\ntype = \"fighter-a\"\n");
    let outputs = ok_text(&text, &air_rules(""), &files);
    assert!(outputs.class_csv.is_empty());
    assert!(outputs.sources_csv.is_empty());
    assert!(outputs.ships_csv.is_empty());
    assert_eq!(
        aircraft_lines(&outputs),
        [
            "id,name,role,search_range_nm,sweep_width_nm,strike_range_nm,attack_weapon,attack_damage,pilot_armour,self_sealing_tanks",
            "2100,Fighter A,fighter,300,20,300,none,0,1,0"
        ]
    );
    // A run writes two files, and a check names two.
    let dir = scratch("shipless");
    std::fs::write(dir.join("derivation/list.toml"), text).unwrap();
    std::fs::write(dir.join("derivation/rules.toml"), air_rules("")).unwrap();
    for (name, text) in &files {
        std::fs::write(dir.join("oob").join(name), text).unwrap();
    }
    match go(&dir, true) {
        Outcome::Clean { written, files } => {
            assert!(!written);
            assert_eq!(files.len(), 2, "{files:?}");
        }
        other => panic!("{other:?}"),
    }
    assert!(reference_files(&dir).is_empty());
    assert!(matches!(go(&dir, false), Outcome::Clean { .. }));
    assert_eq!(
        reference_files(&dir),
        ["test_aircraft.csv", "test_aircraft.sources.csv"]
    );
}

#[test]
fn a_list_with_neither_ships_nor_aircraft_is_a_problem() {
    let text = shipless_list("");
    let problems = derive_text(&text, &air_rules(""), &air_base()).expect_err("a problem");
    assert!(
        problems
            .iter()
            .any(|p| p.contains("list: list: ships:") && p.contains("neither ships nor")),
        "{problems:#?}"
    );
}

#[test]
fn a_class_output_without_ships_and_ships_without_one_are_problems() {
    let text = "name = \"t\"\ndate = \"1942-08-08\"\nrules = \"rules.toml\"\nships = []\n\
        output = \"../reference/x.csv\"\nfirst_class_id = 1\n\
        aircraft_output = \"../reference/a.csv\"\nfirst_aircraft_type_id = 1\n\
        [[aircraft]]\ntype = \"fighter-a\"\n";
    let problems = derive_text(text, &air_rules(""), &air_base()).expect_err("problems");
    for field in ["output", "first_class_id"] {
        assert!(
            problems
                .iter()
                .any(|p| p.contains(&format!("list: list: {field}:")) && p.contains("no ships")),
            "{problems:#?}"
        );
    }
    let text = "name = \"t\"\ndate = \"1942-08-08\"\nrules = \"rules.toml\"\nships = [\"shp\"]\n";
    let problems = derive_text(text, &air_rules(""), &air_base()).expect_err("problems");
    for field in ["output", "first_class_id"] {
        assert!(
            problems
                .iter()
                .any(|p| p.contains(&format!("list: list: {field}:"))
                    && p.contains("required when the list has ships")),
            "{problems:#?}"
        );
    }
}

#[test]
fn a_radius_row_named_by_the_list_is_read_whatever_its_date_and_standing() {
    let files = radius_files();
    // Without the choice: the usual rule, 175.
    let plain = derive_text(
        &shipless_list("[[aircraft]]\ntype = \"dive-a\"\nload = \"bomb-a\"\n"),
        &air_rules(""),
        &files,
    )
    .unwrap_or_else(|p| panic!("{p:#?}"));
    assert_eq!(
        aircraft_lines(&plain)[1],
        "2100,Dive A,bomber,300,20,300,bomb,41.666666,1,1"
    );
    // With the choice, and a rules exception for the same figure that the
    // list's choice replaces.
    let rules = air_rules(&exception_text("dive-a", "combat_radius_nm", 41));
    let outputs = derive_text(&chosen("40", "flown on the seventh"), &rules, &files)
        .unwrap_or_else(|p| panic!("{p:#?}"));
    assert_eq!(
        aircraft_lines(&outputs)[1],
        "2100,Dive A,bomber,400,20,400,bomb,41.666666,1,1"
    );
    for figure in ["search_range_nm", "strike_range_nm"] {
        let row = plane_source(&outputs, "2100", figure);
        assert_eq!(row[4], "aircraft_type_facts.csv:40");
        assert!(row[5].contains("flown on the seventh"), "{row:?}");
        assert!(
            row[5].contains("own choice of row 40 (unconfirmed)"),
            "{row:?}"
        );
    }
    let header = comments(outputs.aircraft_csv.as_ref().unwrap());
    assert!(
        header.contains(
            "dive-a combat_radius_nm: aircraft_type_facts.csv row 40 (unconfirmed); the list's own choice, reason: flown on the seventh"
        ),
        "{header}"
    );
    // The rules exception for row 41 is not listed: the list did not use it.
    assert!(!header.contains("row 41"), "{header}");
}

#[test]
fn a_radius_row_that_does_not_exist_or_is_not_a_radius_of_the_type_is_a_problem() {
    let files = radius_files();
    let problems = derive_text(&chosen("999", "r"), &air_rules(""), &files).expect_err("missing");
    assert_one(
        &problems,
        &[
            "dive-a: exception: strike_range_nm:",
            "combat_radius_row 999 is not a row",
        ],
    );
    // Row 1 is `fighter-a combat_radius_nm`: another type.
    let problems = derive_text(&chosen("1", "r"), &air_rules(""), &files).expect_err("other type");
    assert_one(
        &problems,
        &["combat_radius_row 1 is a row about `fighter-a`"],
    );
    // Row 5 is `fighter-a`'s pilot_armour in air_base; use dive-a's own
    // range row (4: dive-a range_normal_nm) for the wrong attribute.
    let problems = derive_text(&chosen("4", "r"), &air_rules(""), &files).expect_err("attribute");
    assert_one(
        &problems,
        &["combat_radius_row 4 is a row about `dive-a` range_normal_nm"],
    );
}

#[test]
fn a_radius_row_and_its_reason_go_together_and_the_reason_is_not_empty() {
    let files = radius_files();
    let problems = derive_text(&chosen("40", ""), &air_rules(""), &files).expect_err("no reason");
    assert_one(
        &problems,
        &["list: list: aircraft:", "give both or neither"],
    );
    let problems = derive_text(&chosen("", "because"), &air_rules(""), &files).expect_err("no row");
    assert_one(
        &problems,
        &["list: list: aircraft:", "give both or neither"],
    );
    let problems = derive_text(&chosen("40", "  "), &air_rules(""), &files).expect_err("empty");
    assert_one(&problems, &["list: list: aircraft:", "must not be empty"]);
}

#[test]
fn the_eastern_solomons_list_still_derives_exactly_the_files_in_data_reference() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let read = |path: &str| std::fs::read_to_string(root.join(path)).expect(path);
    let (facts, unread) = oob_check::read_dir(&root.join("data/oob"));
    assert!(unread.is_empty());
    let outputs = derive(
        "eastern_solomons.toml",
        &read("data/derivation/eastern_solomons.toml"),
        &read("data/derivation/rules.toml"),
        &facts,
    )
    .unwrap_or_else(|p| panic!("{p:#?}"));
    assert_eq!(
        outputs.class_csv,
        read("data/reference/ship_classes_eastern_solomons.csv")
    );
    assert_eq!(
        outputs.sources_csv,
        read("data/reference/ship_classes_eastern_solomons.sources.csv")
    );
    assert_eq!(
        outputs.ships_csv,
        read("data/reference/ship_classes_eastern_solomons.ships.csv")
    );
    assert_eq!(
        outputs.aircraft_csv.as_deref(),
        Some(read("data/reference/aircraft_types_eastern_solomons.csv").as_str())
    );
    assert_eq!(
        outputs.aircraft_sources_csv.as_deref(),
        Some(read("data/reference/aircraft_types_eastern_solomons.sources.csv").as_str())
    );
}

#[test]
fn the_rabaul_guadalcanal_list_still_derives_exactly_the_files_in_data_reference() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let read = |path: &str| std::fs::read_to_string(root.join(path)).expect(path);
    let (facts, unread) = oob_check::read_dir(&root.join("data/oob"));
    assert!(unread.is_empty());
    let outputs = derive(
        "rabaul_guadalcanal.toml",
        &read("data/derivation/rabaul_guadalcanal.toml"),
        &read("data/derivation/rules.toml"),
        &facts,
    )
    .unwrap_or_else(|p| panic!("{p:#?}"));
    assert_eq!(
        outputs.aircraft_csv.as_deref(),
        Some(read("data/reference/aircraft_types_rabaul_guadalcanal.csv").as_str())
    );
    assert_eq!(
        outputs.aircraft_sources_csv.as_deref(),
        Some(read("data/reference/aircraft_types_rabaul_guadalcanal.sources.csv").as_str())
    );
}

#[test]
fn the_savo_island_list_still_derives_exactly_the_files_in_data_reference() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let read = |path: &str| std::fs::read_to_string(root.join(path)).expect(path);
    let (facts, unread) = oob_check::read_dir(&root.join("data/oob"));
    assert!(unread.is_empty());
    let outputs = derive(
        "savo_island.toml",
        &read("data/derivation/savo_island.toml"),
        &read("data/derivation/rules.toml"),
        &facts,
    )
    .unwrap_or_else(|p| panic!("{p:#?}"));
    assert_eq!(
        outputs.class_csv,
        read("data/reference/ship_classes_savo_island.csv")
    );
    assert_eq!(
        outputs.sources_csv,
        read("data/reference/ship_classes_savo_island.sources.csv")
    );
    assert_eq!(
        outputs.ships_csv,
        read("data/reference/ship_classes_savo_island.ships.csv")
    );
    assert!(outputs.aircraft_csv.is_none());
}

#[test]
fn machine_guns_are_not_counted_and_their_rows_are_not_read() {
    let mut files = air_base();
    // A machine gun in the slot, with a disputed row and a blank count.
    append_lines(
        &mut files,
        "fits.csv",
        &format!(
            "{}\n{}\n",
            weak_fit(fit_row(
                50,
                "ship_class",
                "us-test-class",
                "light_aa",
                "weapon",
                "mg-a",
                "24",
                ("", ""),
                "a",
                "disputed"
            )),
            class_fit(51, "light_aa", "weapon", "mg-a", ""),
        ),
    );
    let outputs = air_ok(&["shp"], &files);
    assert_eq!(class_lines(&outputs), [format!("{BASE_ROW},0,12")]);
    let row = source_of(&outputs, "1000", "anti_aircraft");
    assert!(
        row[5].contains("machine guns not counted: fits.csv:50 fits.csv:51"),
        "{row:?}"
    );
    assert!(!row[4].contains("fits.csv:50"), "{row:?}");
    assert!(!row[5].contains("count not given"), "{row:?}");
}

#[test]
fn a_weapon_of_another_kind_or_unknown_in_an_aa_slot_is_a_problem() {
    let mut files = air_base();
    append_lines(
        &mut files,
        "fits.csv",
        &format!("{}\n", class_fit(50, "light_aa", "weapon", "odd-w", "2")),
    );
    let problems = air_ship(&["shp"], &files).expect_err("a problem");
    assert_one(
        &problems,
        &[
            "shp: unknown-type: anti_aircraft:",
            "`odd-w` has kind `gun`",
            "fits.csv:50",
        ],
    );
}
