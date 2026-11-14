//! The troop figure `troop_capacity_men` (decision 0093, 8), against a small
//! synthetic facts directory built in memory.
//!
//! # The hand calculation
//!
//! The list's date is 1942-08-08 and its voyage is 1942-08-02. The class
//! `us-test-class` is a destroyer; every other figure is as in
//! `tests/derive.rs`. Four ships of it, each pinned to an id of its own:
//!
//! | Ship | Rows | Rule step | Result |
//! | --- | --- | --- | --- |
//! | a | `troop_capacity_men` 1388, preferred | 1, stated | 1388 |
//! | b | no stated row; `troops_carried_men` 1199 on 1942-08-02, preferred | 2, voyage | 1199 |
//! | c | neither; in a run, `troops_carried` 900, `ships_in_run` read as 6 | 3, run: 900 / 6 | 150 |
//! | d | no troop row, no run | 4, a warship | 0 |
//!
//! Other runs, read for ship c alone: 1200 men over 8 ships is 150; 900 over
//! 4 is 225; 1000 over 6 is 166 (exact 166.666666, rounded down).
//!
//! Ship b with a `troop_capacity_men` row that is `unconfirmed`: the row is
//! not read, the note names it, and the figure falls to the voyage, 1199.
//! A transport-type class with no troop rows is a `missing` problem.
//! With `troop_figures` absent the class file has no `troop_capacity_men`
//! column and its output does not change by a byte.

#![allow(dead_code)]

use std::collections::BTreeMap;

use oob_derive::{derive, Outputs};

type Files = BTreeMap<String, String>;

const DATE: &str = "1942-08-08";

const RULES: &str = "constants_are_placeholders = true\n\
    gun_shell_kg_per_damage = 6\n\
    torpedo_damage_per_warhead_kg = \"0.4\"\n\
    displacement_lt_per_toughness = 25\n";

const TROOP_RULE: &str = "troop_capacity_men of the ship, else of its class, as a source states it; else, where the list names a voyage, troops_carried_men of the ship on that voyage (a load on one voyage, not a capacity); else, where the list names a run for the ship, troops_carried of the run over the ships in the run as the list reads them from the named ships_in_run row, rounded down (a run total, a weaker kind of support than a stated capacity); else 0 for a warship (no troop row: carries none) and an error for a transport or cargo ship (decision 0093, 8)";

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
            "id,key\n1000,radar-a\n1001,radar-b\n1002,jp-radar-any\n1003,radar-c\n1004,radar-d\n".to_string(),
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
             warhead_kg,weapon,decimal,one\n\
             display,sensor,text,one\n"
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
        (
            "sensor_facts.csv",
            table(
                FACT_HEADER,
                &[
                    fact(1, "radar-a", "display", "PPI and A-scope"),
                    fact(2, "radar-b", "display", "A-scope"),
                    fact(3, "radar-d", "display", "A-scope and PPI"),
                ],
            ),
        ),
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

const TROOP_HEADER: &str = "id,entity,attribute,value,as_stated,valid_from,valid_to,\
    source,locator,accessed,confidence,basis,standing,note\n";

fn troop_row(id: u32, entity: &str, attribute: &str, value: &str, standing: &str) -> String {
    format!(
        "{id},{entity},{attribute},{value},{value},,,a,{SOURCE_TAIL},medium,stated,{standing},a note"
    )
}

fn dated_fact(id: u32, entity: &str, attribute: &str, value: &str, day: &str) -> String {
    fact_row(
        id,
        entity,
        attribute,
        value,
        day,
        day,
        "medium",
        "preferred",
    )
}

/// The base set with four destroyers a, b, c, d of the test class and the
/// two lift files. Ship a states a capacity; b has a load on the voyage; c
/// is in run `run-1` (row 1: 900 men, unconfirmed; row 2: the ships; row 3:
/// another figure for the run, not read); d has nothing.
fn troop_base() -> Files {
    let mut files = base();
    let text = files.get_mut("ship_classes.csv").unwrap();
    *text = text.replace("Test class,us,heavy_cruiser", "Test class,us,destroyer");
    add_row(
        &mut files,
        "ships.csv",
        "1010,a,A,us,us-test-class,".to_string(),
    );
    add_row(
        &mut files,
        "ships.csv",
        "1011,b,B,us,us-test-class,".to_string(),
    );
    add_row(
        &mut files,
        "ships.csv",
        "1012,c,C,us,us-test-class,".to_string(),
    );
    add_row(
        &mut files,
        "ships.csv",
        "1013,d,D,us,us-test-class,".to_string(),
    );
    add_row(
        &mut files,
        "attributes.csv",
        "troop_capacity_men,ship_class,integer,one".to_string(),
    );
    add_row(
        &mut files,
        "attributes.csv",
        "troop_capacity_men,ship,integer,one".to_string(),
    );
    add_row(
        &mut files,
        "attributes.csv",
        "troops_carried_men,ship,integer,many".to_string(),
    );
    add_row(
        &mut files,
        "attributes.csv",
        "troop_run,ship,text,many".to_string(),
    );
    add_row(
        &mut files,
        "ship_facts.csv",
        fact(1, "a", "troop_capacity_men", "1388"),
    );
    add_row(
        &mut files,
        "ship_facts.csv",
        dated_fact(2, "b", "troops_carried_men", "1199", "1942-08-02"),
    );
    add_row(
        &mut files,
        "ship_facts.csv",
        fact(3, "c", "troop_run", "run-1"),
    );
    files.insert(
        "lift_entities.csv".to_string(),
        "id,key,name,kind,nation\n1000,run-1,Run one,run,jp\n1001,run-2,Run two,run,jp\n\
         1002,ship-x,Ship x,ship,us\n"
            .to_string(),
    );
    files.insert(
        "troop_lift_facts.csv".to_string(),
        table(
            TROOP_HEADER,
            &[
                troop_row(1, "run-1", "troops_carried", "900", "unconfirmed"),
                troop_row(2, "run-1", "ships_in_run", "six destroyers", "unconfirmed"),
                troop_row(3, "run-1", "troops_carried", "800", "preferred"),
                troop_row(4, "run-2", "troops_carried", "500", "preferred"),
            ],
        ),
    );
    files
}

const PINS: &str = "class_ids = { \"1000\" = [\"a\"], \"1001\" = [\"b\"], \"1002\" = [\"c\"], \"1003\" = [\"d\"] }\n";

const RUN_ONE: &str = "[[troops.run]]\nrun = \"run-1\"\nships = [\"c\"]\nmen_row = 1\n\
    ships_row = 2\nships_in_run = 6\nreason = \"the run total is all there is\"\n";

/// A list of the ships named, each pinned to an id of its own. `top` are
/// top-level keys and `tables` the tables that follow them.
fn troop_list(ships: &[&str], top: &str, tables: &str) -> String {
    let quoted: Vec<String> = ships.iter().map(|s| format!("\"{s}\"")).collect();
    let pins: Vec<String> = ships
        .iter()
        .enumerate()
        .map(|(n, s)| format!("\"{}\" = [\"{s}\"]", 1000 + n))
        .collect();
    format!(
        "name = \"Test list\"\ndate = \"{DATE}\"\nrules = \"rules.toml\"\n\
         output = \"../reference/test_out.csv\"\nfirst_class_id = 1000\n\
         ships = [{}]\nclass_ids = {{ {} }}\n{top}{tables}",
        quoted.join(", "),
        pins.join(", ")
    )
}

fn run_list(
    ships: &[&str],
    top: &str,
    tables: &str,
    files: &Files,
) -> Result<Outputs, Vec<String>> {
    derive("list.toml", &troop_list(ships, top, tables), RULES, files)
        .map_err(|problems| problems.iter().map(ToString::to_string).collect())
}

fn troop_tables(run_block: &str) -> String {
    format!("[troops]\nvoyage = \"1942-08-02\"\n{run_block}")
}

#[track_caller]
fn troop_ok(ships: &[&str], run_block: &str, files: &Files) -> Outputs {
    run_list(
        ships,
        "troop_figures = true\n",
        &troop_tables(run_block),
        files,
    )
    .unwrap_or_else(|problems| panic!("{problems:#?}"))
}

/// The class id of a ship and the troop figure's sources row.
fn troop_source(outputs: &Outputs, ship: &str) -> Vec<String> {
    let ships = records(&outputs.ships_csv);
    let class_id = &ships.iter().find(|r| r[0] == ship).expect("the ship")[1];
    source_of(outputs, class_id, "troop_capacity_men")
}

#[track_caller]
fn assert_problem(problems: &[String], expected: &str) {
    assert!(
        problems.iter().any(|p| p.contains(expected)),
        "`{expected}` in {problems:#?}"
    );
}

#[test]
fn the_troop_base_passes_the_checker() {
    let report = oob_check::check_files(&troop_base());
    assert!(report.is_clean(), "{:#?}", report.problems);
}

#[test]
fn the_four_ships_of_the_hand_calculation() {
    let outputs = troop_ok(&["a", "b", "c", "d"], RUN_ONE, &troop_base());
    let header = outputs
        .class_csv
        .lines()
        .find(|l| l.starts_with("id,"))
        .unwrap();
    assert!(header.ends_with(",radar,troop_capacity_men"), "{header}");
    let expected = [("a", "1388"), ("b", "1199"), ("c", "150"), ("d", "0")];
    for (ship, value) in expected {
        let row = troop_source(&outputs, ship);
        assert_eq!(row[2], value, "{ship}");
        assert_eq!(row[3], TROOP_RULE);
    }
    // The class rows end with the figure.
    let lines = class_lines(&outputs);
    for (line, (_, value)) in lines.iter().zip(expected) {
        assert!(line.ends_with(&format!(",1,{value}")), "{line}");
    }

    let a = troop_source(&outputs, "a");
    assert_eq!((a[4].as_str(), a[5].as_str()), ("ship_facts.csv:1", ""));
    let b = troop_source(&outputs, "b");
    assert_eq!(b[4], "ship_facts.csv:2");
    assert_eq!(
        b[5],
        "men she carried on the voyage of 1942-08-02: a load on one voyage, not a stated capacity"
    );
    let c = troop_source(&outputs, "c");
    assert_eq!(
        c[4],
        "ship_facts.csv:3 troop_lift_facts.csv:1 troop_lift_facts.csv:2"
    );
    assert!(c[5].contains(
        "men carried on run run-1: 900 men (troop_lift_facts.csv:1) over 6 ships (troop_lift_facts.csv:2), rounded down: a run total, a weaker kind of support than a stated capacity; exact 150; the run total is all there is"
    ), "{}", c[5]);
    assert!(
        c[5].contains("other rows of the run not read: troop_lift_facts.csv:3 (preferred; 800)")
    );
    let d = troop_source(&outputs, "d");
    assert_eq!(d[4], "");
    assert_eq!(d[5], "no troop row: carries none");

    let header = comments(&outputs.class_csv);
    assert!(header.contains("the voyage whose loads are read is 1942-08-02"));
    assert!(header.contains("run-1, troop_lift_facts.csv:1, troop_lift_facts.csv:2, 6"));
}

fn run_with(men: &str, ships_in_run: u32) -> Vec<String> {
    let mut files = troop_base();
    replace_row(
        &mut files,
        "troop_lift_facts.csv",
        1,
        troop_row(1, "run-1", "troops_carried", men, "unconfirmed"),
    );
    let block = RUN_ONE.replace(
        "ships_in_run = 6",
        &format!("ships_in_run = {ships_in_run}"),
    );
    let outputs = troop_ok(&["c"], &block, &files);
    troop_source(&outputs, "c")
}

#[test]
fn a_run_is_divided_and_rounded_down() {
    assert_eq!(run_with("1200", 8)[2], "150");
    assert_eq!(run_with("900", 4)[2], "225");
    let row = run_with("1000", 6);
    assert_eq!(row[2], "166");
    assert!(row[5].contains("exact 166.666666;"), "{}", row[5]);
}

#[test]
fn an_unconfirmed_stated_row_is_not_read_and_the_voyage_is() {
    let mut files = troop_base();
    add_row(
        &mut files,
        "ship_facts.csv",
        fact_row(
            4,
            "b",
            "troop_capacity_men",
            "1500",
            "",
            "",
            "low",
            "unconfirmed",
        ),
    );
    let outputs = troop_ok(&["b"], "", &files);
    let row = troop_source(&outputs, "b");
    assert_eq!(row[2], "1199");
    assert!(
        row[5].contains("stated rows not read: ship_facts.csv:4 (unconfirmed)"),
        "{}",
        row[5]
    );
}

#[test]
fn a_transport_with_no_troop_row_is_a_missing_problem() {
    let mut files = troop_base();
    let text = files.get_mut("ship_classes.csv").unwrap();
    *text = text.replace("Test class,us,destroyer", "Test class,us,transport");
    let problems = run_list(&["d"], "troop_figures = true\n", "", &files).expect_err("problems");
    assert_problem(
        &problems,
        "d: missing: troop_capacity_men: troop_capacity_men of ship d: no stated capacity, no load on the list's voyage and no run, and a transport must have one",
    );
}

#[test]
fn the_runs_of_the_list_are_checked() {
    let files = troop_base();
    // The men row is a ships_in_run row.
    let block = RUN_ONE.replace("men_row = 1", "men_row = 2");
    let problems = run_list(
        &["c"],
        "troop_figures = true\n",
        &troop_tables(&block),
        &files,
    )
    .expect_err("problems");
    assert_problem(
        &problems,
        "run-1: list: troop_capacity_men: men_row 2 is not a `troops_carried` row",
    );
    // The ships row is a troops_carried row.
    let block = RUN_ONE.replace("ships_row = 2", "ships_row = 1");
    let problems = run_list(
        &["c"],
        "troop_figures = true\n",
        &troop_tables(&block),
        &files,
    )
    .expect_err("problems");
    assert_problem(&problems, "ships_row 1 is not a `ships_in_run` row");
    // A row of another run.
    let block = RUN_ONE.replace("men_row = 1", "men_row = 4");
    let problems = run_list(
        &["c"],
        "troop_figures = true\n",
        &troop_tables(&block),
        &files,
    )
    .expect_err("problems");
    assert_problem(
        &problems,
        "men_row 4 is not a `troops_carried` row of `run-1`",
    );
    // A run that is not a run.
    let block = RUN_ONE.replace("run = \"run-1\"", "run = \"ship-x\"");
    let problems = run_list(
        &["c"],
        "troop_figures = true\n",
        &troop_tables(&block),
        &files,
    )
    .expect_err("problems");
    assert_problem(
        &problems,
        "`ship-x` is a key of lift_entities.csv of kind `ship`, not a run",
    );
    let block = RUN_ONE.replace("run = \"run-1\"", "run = \"run-9\"");
    let problems = run_list(
        &["c"],
        "troop_figures = true\n",
        &troop_tables(&block),
        &files,
    )
    .expect_err("problems");
    assert_problem(&problems, "`run-9` is not a key of lift_entities.csv");
    // Fewer ships in the run than the list names for it.
    let block = RUN_ONE
        .replace("ships = [\"c\"]", "ships = [\"c\", \"d\"]")
        .replace("ships_in_run = 6", "ships_in_run = 1");
    let problems = run_list(
        &["c", "d"],
        "troop_figures = true\n",
        &troop_tables(&block),
        &files,
    )
    .expect_err("problems");
    assert_problem(&problems, "ships_in_run 1 is fewer than the 2 ships");
    // A ship not in the list.
    let block = RUN_ONE.replace("ships = [\"c\"]", "ships = [\"d\"]");
    let problems = run_list(
        &["c"],
        "troop_figures = true\n",
        &troop_tables(&block),
        &files,
    )
    .expect_err("problems");
    assert_problem(&problems, "`d` is not in ships");
    // An empty reason.
    let block = RUN_ONE.replace("the run total is all there is", " ");
    let problems = run_list(
        &["c"],
        "troop_figures = true\n",
        &troop_tables(&block),
        &files,
    )
    .expect_err("problems");
    assert_problem(&problems, "`reason` must not be empty");
}

#[test]
fn a_ship_in_two_runs_is_a_problem() {
    let block = format!(
        "{RUN_ONE}{}",
        RUN_ONE
            .replace("run-1", "run-2")
            .replace("men_row = 1", "men_row = 4")
    );
    let problems = run_list(
        &["c"],
        "troop_figures = true\n",
        &troop_tables(&block),
        &troop_base(),
    )
    .expect_err("problems");
    assert_problem(&problems, "ship `c` is already read for run run-1");
}

#[test]
fn a_ship_with_no_troop_run_row_for_her_run_is_a_problem() {
    // Ship d has no troop_run row at all.
    let block = RUN_ONE.replace("ships = [\"c\"]", "ships = [\"d\"]");
    let problems = run_list(
        &["d"],
        "troop_figures = true\n",
        &troop_tables(&block),
        &troop_base(),
    )
    .expect_err("problems");
    assert_problem(
        &problems,
        "d: list: troop_capacity_men: no ship_facts.csv row says troop_run run-1",
    );
    // A disputed row does not count.
    let mut files = troop_base();
    replace_row(
        &mut files,
        "ship_facts.csv",
        3,
        fact_row(3, "c", "troop_run", "run-1", "", "", "medium", "disputed"),
    );
    let problems = run_list(
        &["c"],
        "troop_figures = true\n",
        &troop_tables(RUN_ONE),
        &files,
    )
    .expect_err("problems");
    assert_problem(
        &problems,
        "c: list: troop_capacity_men: no ship_facts.csv row says troop_run run-1",
    );
}

#[test]
fn troops_without_troop_figures_is_a_problem() {
    let problems =
        run_list(&["c"], "", &troop_tables(RUN_ONE), &troop_base()).expect_err("problems");
    assert_problem(&problems, "a [troops] table needs troop_figures = true");
}

#[test]
fn a_run_needs_the_lift_files() {
    let mut files = troop_base();
    files.remove("lift_entities.csv");
    files.remove("troop_lift_facts.csv");
    let problems = run_list(
        &["c"],
        "troop_figures = true\n",
        &troop_tables(RUN_ONE),
        &files,
    )
    .expect_err("problems");
    assert_problem(&problems, "`run-1` is not a key of lift_entities.csv");
}

#[test]
fn without_troop_figures_the_output_does_not_change() {
    let plain = run_list(&["a", "b", "c", "d"], "", "", &troop_base()).expect("derives");
    let header = plain
        .class_csv
        .lines()
        .find(|l| l.starts_with("id,"))
        .unwrap();
    assert!(!header.contains("troop_capacity_men"), "{header}");
    assert!(!plain.class_csv.contains("Troop figure"));
    // The lift files and the troop rows change nothing for a list that does
    // not ask: the same list over the plain base gives the same files.
    let mut files = base();
    let text = files.get_mut("ship_classes.csv").unwrap();
    *text = text.replace("Test class,us,heavy_cruiser", "Test class,us,destroyer");
    for (key, id) in [("a", 1010), ("b", 1011), ("c", 1012), ("d", 1013)] {
        add_row(
            &mut files,
            "ships.csv",
            format!("{id},{key},{}, us,us-test-class,", key.to_uppercase()).replace(", us", ",us"),
        );
    }
    let bare = run_list(&["a", "b", "c", "d"], "", "", &files).expect("derives");
    assert_eq!(plain, bare);
    assert_eq!(records(&plain.sources_csv).len(), 15 * 4);
}

#[test]
fn the_troop_figure_follows_the_air_figures() {
    let rules = format!("{RULES}anti_aircraft_heavy_weight = 4\nanti_aircraft_light_weight = 1\n");
    let list = troop_list(&["a"], "air_figures = true\ntroop_figures = true\n", "");
    let outputs =
        derive("list.toml", &list, &rules, &troop_base()).unwrap_or_else(|p| panic!("{p:#?}"));
    let header = outputs
        .class_csv
        .lines()
        .find(|l| l.starts_with("id,"))
        .unwrap();
    assert!(
        header.ends_with(",radar,aircraft_capacity,anti_aircraft,troop_capacity_men"),
        "{header}"
    );
    assert!(class_lines(&outputs)[0].ends_with(",1,0,0,1388"));
}

#[test]
fn ships_pinned_together_must_share_the_troop_figure() {
    // a (1388) and d (0) pinned to one id.
    let list = "name = \"Test list\"\ndate = \"1942-08-08\"\nrules = \"rules.toml\"\n\
        output = \"../reference/test_out.csv\"\nfirst_class_id = 1000\n\
        ships = [\"a\", \"d\"]\nclass_ids = { \"1000\" = [\"a\", \"d\"] }\ntroop_figures = true\n";
    let problems: Vec<String> = derive("list.toml", list, RULES, &troop_base())
        .expect_err("problems")
        .iter()
        .map(ToString::to_string)
        .collect();
    assert_problem(&problems, "troop_capacity_men");
    assert_problem(&problems, "must have the same figures");
}
