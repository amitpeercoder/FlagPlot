//! The submarine classes and their torpedoes (decision 0218, stage 1, rules
//! 28 and 29), on a small data set of this file's own that passes
//! `oob_check::check_files`.
//!
//! # The data, worked by hand
//!
//! The list's date is 1942-08-07 and the rules give `torpedo_damage_per_warhead_kg`
//! 0.4. Five classes of the type submarine are on file, each with all rows
//! preferred:
//!
//! | Class | tubes bow | carried | endurance speed | max speed | torpedo model |
//! | --- | --- | --- | --- | --- | --- |
//! | `us-sub-a` | 4 | 22 | 10 | 20 | `us-torp-a` |
//! | `us-sub-b` | 6 | 16 | 15 | 20 | none: `torpedo_from` the group `us-sub-fleet` |
//! | `jp-sub-c` | 4 | 12 | 14 | 20 | none: `torpedo_from` the boats of her group |
//! | `jp-sub-d` | 6 | 12 | none: estimated | 20 | `jp-torp` |
//! | `us-sub-fleet` | not a class: left out; its row 6 says `us-torp-a` |
//!
//! Salvo is the tubes; salvoes are carried over salvo, rounded down:
//! `us-sub-a` 22 / 4 = 5.5, so 5; `us-sub-b` 16 / 6 = 2.666666, so 2;
//! `jp-sub-c` 12 / 4 = 3; `jp-sub-d` 12 / 6 = 2.
//!
//! The passage speed is the endurance speed: 10, 15, 14. For `jp-sub-d` there
//! is none, and the list estimates it by ratio from her max_speed_kn
//! (decision 0207): the like classes are the other classes of the type with
//! rows of both: `us-sub-a` 10 / 20 = 0.5, `us-sub-b` 15 / 20 = 0.75,
//! `jp-sub-c` 14 / 20 = 0.7; the mean is (0.5 + 0.75 + 0.7) / 3 = 0.65, and
//! her 20 x 0.65 = 13.
//!
//! The torpedo ids are pinned: `us-torp-a` 6100, `jp-torp` 6101,
//! `jp-torp-b` 6102. The damage of a hit is the warhead x 0.4: 200 kg gives
//! 80, 300.5 kg gives 120.2, 100 kg gives 40.
//!
//! So the class rows are
//! `6000,Sub A class,4,5,10,6100,`, `6001,Sub B class,6,2,15,6100,torpedo_id`,
//! `6002,Sub C class,4,3,14,6101,torpedo_id` and
//! `6003,Sub D class,6,2,13,6101,passage_speed_kn`, and the torpedo rows
//! `6100,Torpedo A,80`, `6101,Torpedo C,120.2` and `6102,Torpedo B,40`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use oob_derive::{derive, run, Outcome, Outputs};

type Files = BTreeMap<String, String>;

const RULES: &str = "constants_are_placeholders = true\n\
    gun_shell_kg_per_damage = 6\n\
    torpedo_damage_per_warhead_kg = \"0.4\"\n\
    displacement_lt_per_toughness = 25\n";

const FACT_HEADER: &str = "id,entity,attribute,value,valid_from,valid_to,\
    source,locator,accessed,confidence,basis,standing,note\n";

fn fact_row(id: u32, entity: &str, attribute: &str, value: &str, standing: &str) -> String {
    format!("{id},{entity},{attribute},{value},,,a,p.1,2026-11-10,medium,stated,{standing},a note")
}

fn fact(id: u32, entity: &str, attribute: &str, value: &str) -> String {
    fact_row(id, entity, attribute, value, "preferred")
}

fn table(header: &str, rows: &[String]) -> String {
    let mut text = header.to_string();
    for row in rows {
        text.push_str(row);
        text.push('\n');
    }
    text
}

fn event(id: u32, actor: &str, means: &str, standing: &str) -> String {
    format!(
        "{id},eng,1942-05-05,,,2,torpedo_launch,{actor},a ship,{means},a,p.1,2026-11-10,medium,stated,{standing},a note"
    )
}

/// A data set that passes the checker, with the five classes above.
fn base() -> Files {
    let empty = FACT_HEADER.to_string();
    let files = [
        (
            "sources.csv",
            "id,key,reliability\n1000,a,A\n".to_string(),
        ),
        ("places.csv", "id,key\n".to_string()),
        (
            "engagements.csv",
            "id,key,date_from,date_to\n1000,eng,1942-01-01,1942-12-31\n".to_string(),
        ),
        (
            "ship_classes.csv",
            "id,key,name,nation,type\n\
             1000,us-sub-a,Sub A class,us,submarine\n\
             1001,us-sub-b,Sub B class,us,submarine\n\
             1002,jp-sub-c,Sub C class,jp,submarine\n\
             1003,jp-sub-d,Sub D class,jp,submarine\n\
             1004,us-sub-fleet,Fleet type in general,us,submarine\n\
             1005,us-cruiser,Cruiser class,us,heavy_cruiser\n"
                .to_string(),
        ),
        ("ships.csv", "id,key,name,nation,class,hull_number\n".to_string()),
        ("aircraft_types.csv", "id,key\n".to_string()),
        (
            "weapons.csv",
            "id,key,name,nation,kind\n\
             1000,us-torp-a,Torpedo A,us,torpedo\n\
             1001,jp-torp,Torpedo C,jp,torpedo\n\
             1002,jp-torp-b,Torpedo B,jp,torpedo\n\
             1003,a-gun,A gun,us,gun\n"
                .to_string(),
        ),
        ("sensors.csv", "id,key\n".to_string()),
        ("air_units.csv", "id,key,parent\n".to_string()),
        (
            "attributes.csv",
            "key,applies_to,value_type,cardinality\n\
             max_speed_kn,ship_class,decimal,one\n\
             endurance_speed_kn,ship_class,decimal,one\n\
             torpedo_tubes_bow,ship_class,integer,one\n\
             torpedoes_carried,ship_class,integer,one\n\
             torpedo_model,ship_class,text,many\n\
             warhead_kg,weapon,decimal,one\n"
                .to_string(),
        ),
        (
            "gaps.csv",
            "entity_type,entity,what\nship_class,jp-sub-d,endurance_speed_kn\n".to_string(),
        ),
        (
            "ship_class_facts.csv",
            table(
                FACT_HEADER,
                &[
                    fact(1, "us-sub-a", "torpedo_tubes_bow", "4"),
                    fact(2, "us-sub-a", "torpedoes_carried", "22"),
                    fact(3, "us-sub-a", "endurance_speed_kn", "10"),
                    fact(4, "us-sub-a", "max_speed_kn", "20"),
                    fact(5, "us-sub-a", "torpedo_model", "us-torp-a"),
                    fact(6, "us-sub-fleet", "torpedo_model", "us-torp-a"),
                    fact(7, "us-sub-b", "torpedo_tubes_bow", "6"),
                    fact(8, "us-sub-b", "torpedoes_carried", "16"),
                    fact(9, "us-sub-b", "endurance_speed_kn", "15"),
                    fact(10, "us-sub-b", "max_speed_kn", "20"),
                    fact(11, "jp-sub-c", "torpedo_tubes_bow", "4"),
                    fact(12, "jp-sub-c", "torpedoes_carried", "12"),
                    fact(13, "jp-sub-c", "endurance_speed_kn", "14"),
                    fact(14, "jp-sub-c", "max_speed_kn", "20"),
                    fact(15, "jp-sub-d", "torpedo_tubes_bow", "6"),
                    fact(16, "jp-sub-d", "torpedoes_carried", "12"),
                    fact(17, "jp-sub-d", "max_speed_kn", "20"),
                    fact(18, "jp-sub-d", "torpedo_model", "jp-torp"),
                ],
            ),
        ),
        ("ship_facts.csv", empty.clone()),
        ("aircraft_type_facts.csv", empty.clone()),
        (
            "weapon_facts.csv",
            table(
                FACT_HEADER,
                &[
                    fact(1, "us-torp-a", "warhead_kg", "200"),
                    fact(2, "jp-torp", "warhead_kg", "300.5"),
                    fact(3, "jp-torp-b", "warhead_kg", "100"),
                ],
            ),
        ),
        ("sensor_facts.csv", empty.clone()),
        ("air_unit_facts.csv", empty),
        (
            "fits.csv",
            "id,carrier_type,carrier,slot,item_type,item,count,valid_from,valid_to,\
             source,locator,accessed,confidence,basis,standing,note\n"
                .to_string(),
        ),
        (
            "engagement_forces.csv",
            "id,engagement,member_type,member,source,locator,accessed,confidence,basis,standing,note\n"
                .to_string(),
        ),
        (
            "engagement_events.csv",
            table(
                "id,engagement,time,range_m,bearing_deg,value,kind,actor,target,means,\
                 source,locator,accessed,confidence,basis,standing,note\n",
                &[
                    event(1, "jp-boat-1", "two torpedoes (jp-torp)", "preferred"),
                    event(2, "jp-boat-2", "a torpedo", "unconfirmed"),
                ],
            ),
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

/// The list of the data above. `entries` and `extra` are the lists' own
/// text: `entries` the [[submarine]] entries and [[left_out]] entries.
const HEAD: &str = "name = \"Test submarines\"\n\
    date = \"1942-08-07\"\n\
    rules = \"rules.toml\"\n\
    ships = []\n\
    submarine_output = \"../reference/subs.csv\"\n\
    submarine_torpedo_output = \"../reference/torps.csv\"\n\
    torpedoes = [\"us-torp-a\", \"jp-torp\", \"jp-torp-b\"]\n";

const ENTRIES: &str = r#"
[[submarine]]
class = "us-sub-a"

[[submarine]]
class = "us-sub-b"
[submarine.torpedo_from]
weapon = "us-torp-a"
group = "us-sub-fleet"
rows = ["ship_class_facts.csv:6"]
by = "the test"
date = "2026-11-22"
reason = "No row names the torpedo of the Sub B class; the fleet type's row does."

[[submarine]]
class = "jp-sub-c"
[submarine.torpedo_from]
weapon = "jp-torp"
rows = ["engagement_events.csv:1", "engagement_events.csv:2"]
by = "the test"
date = "2026-11-22"
reason = "Her boats fired it."

[[submarine]]
class = "jp-sub-d"

[[left_out]]
class = "us-sub-fleet"
reason = "Not a class."
"#;

const PINS: &str = r#"
[submarine_class_ids]
6000 = "us-sub-a"
6001 = "us-sub-b"
6002 = "jp-sub-c"
6003 = "jp-sub-d"

[submarine_torpedo_ids]
6100 = "us-torp-a"
6101 = "jp-torp"
6102 = "jp-torp-b"
"#;

const ESTIMATE: &str = r#"
[[estimate]]
class = "jp-sub-d"
attribute = "endurance_speed_kn"
method = "ratio"
from = "max_speed_kn"
by = "the test"
date = "2026-11-22"
reason = "No row of her speed of endurance."
"#;

/// The list text: the head, the keys of `top` (which must come before any
/// table), then the tables.
fn list_text(top: &str, tables: &str) -> String {
    // Top-level keys go before the first table: the head holds only keys.
    format!("{HEAD}{top}{tables}")
}

fn the_list() -> String {
    list_text("", &format!("{ESTIMATE}{ENTRIES}{PINS}"))
}

fn derive_list(list: &str, files: &Files) -> Result<Outputs, Vec<String>> {
    derive("subs.toml", list, RULES, files)
        .map_err(|problems| problems.iter().map(ToString::to_string).collect())
}

#[track_caller]
fn derives(list: &str, files: &Files) -> Outputs {
    derive_list(list, files).unwrap_or_else(|p| panic!("{p:#?}"))
}

#[track_caller]
fn fails(list: &str, files: &Files) -> Vec<String> {
    derive_list(list, files).expect_err("problems expected")
}

#[track_caller]
fn assert_one(problems: &[String], expected: &[&str]) {
    assert_eq!(problems.len(), 1, "{problems:#?}");
    for want in expected {
        assert!(problems[0].contains(want), "`{want}` in {problems:#?}");
    }
}

fn records(text: &str) -> Vec<Vec<String>> {
    csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .from_reader(text.as_bytes())
        .records()
        .map(|r| r.expect("a record").iter().map(str::to_string).collect())
        .collect()
}

fn lines(text: &str) -> Vec<&str> {
    text.lines().filter(|l| !l.starts_with('#')).collect()
}

fn comments(text: &str) -> String {
    text.lines()
        .filter(|l| l.starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n")
}

fn source_of(sources: &str, id: &str, figure: &str) -> Vec<String> {
    records(sources)
        .into_iter()
        .find(|r| r[0] == id && r[1] == figure)
        .unwrap_or_else(|| panic!("no sources row for {id} {figure}"))
}

fn replace_row(files: &mut Files, file: &str, id: u32, line: String) {
    let text = files.get_mut(file).expect("a file of the set");
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
    let text = files.get_mut(file).expect("a file of the set");
    let prefix = format!("{id},");
    let kept: Vec<&str> = text.lines().filter(|l| !l.starts_with(&prefix)).collect();
    assert!(kept.len() < text.lines().count(), "row {id} of {file}");
    *text = kept.join("\n") + "\n";
}

fn add_row(files: &mut Files, file: &str, line: String) {
    let text = files.get_mut(file).expect("a file of the set");
    text.push_str(&line);
    text.push('\n');
}

#[test]
fn the_data_set_passes_the_checker() {
    let report = oob_check::check_files(&base());
    assert!(report.problems.is_empty(), "{:#?}", report.problems);
}

#[test]
fn the_classes_and_torpedoes_derive_as_worked_by_hand() {
    let outputs = derives(&the_list(), &base());
    let files = outputs.submarine.expect("submarine files");
    assert_eq!(
        lines(&files.class_csv),
        [
            "id,name,salvo,salvoes_carried,passage_speed_kn,torpedo_id,estimated",
            "6000,Sub A class,4,5,10,6100,",
            "6001,Sub B class,6,2,15,6100,torpedo_id",
            "6002,Sub C class,4,3,14,6101,torpedo_id",
            "6003,Sub D class,6,2,13,6101,passage_speed_kn",
        ]
    );
    assert_eq!(
        lines(&files.torpedo_csv),
        [
            "id,name,damage",
            "6100,Torpedo A,80",
            "6101,Torpedo C,120.2",
            "6102,Torpedo B,40",
        ]
    );
    // No ship file and no ships.
    assert!(outputs.class_csv.is_empty());
    assert!(outputs.ships_csv.is_empty());
}

#[test]
fn salvoes_are_rounded_down_and_the_note_says_so() {
    let files = derives(&the_list(), &base()).submarine.unwrap();
    let a = source_of(&files.class_sources_csv, "6000", "salvoes_carried");
    assert_eq!(a[2], "5");
    assert_eq!(a[4], "ship_class_facts.csv:1 ship_class_facts.csv:2");
    assert!(a[5].contains("exact 5.5, rounded down"), "{}", a[5]);
    let b = source_of(&files.class_sources_csv, "6001", "salvoes_carried");
    assert_eq!(b[2], "2");
    assert!(b[5].contains("exact 2.666666"), "{}", b[5]);
    // A whole result has no such note.
    let c = source_of(&files.class_sources_csv, "6002", "salvoes_carried");
    assert_eq!(c[5], "");
    let salvo = source_of(&files.class_sources_csv, "6000", "salvo");
    assert_eq!(salvo[2], "4");
    assert_eq!(salvo[4], "ship_class_facts.csv:1");
    let passage = source_of(&files.class_sources_csv, "6000", "passage_speed_kn");
    assert_eq!(passage[2], "10");
    assert_eq!(passage[4], "ship_class_facts.csv:3");
}

#[test]
fn a_torpedo_read_from_the_class_row_cites_it_and_is_not_marked() {
    let files = derives(&the_list(), &base()).submarine.unwrap();
    let torpedo = source_of(&files.class_sources_csv, "6000", "torpedo_id");
    assert_eq!(torpedo[2], "6100");
    assert_eq!(torpedo[4], "ship_class_facts.csv:5");
    assert!(torpedo[3].starts_with("torpedo_model of the class"));
    assert_eq!(torpedo[5], "");
    // Class D's row is read too, and her mark is only the speed's.
    let d = source_of(&files.class_sources_csv, "6003", "torpedo_id");
    assert_eq!(d[2], "6101");
    assert_eq!(d[4], "ship_class_facts.csv:18");
}

#[test]
fn a_torpedo_taken_from_another_group_is_marked_and_says_how() {
    let files = derives(&the_list(), &base()).submarine.unwrap();
    let b = source_of(&files.class_sources_csv, "6001", "torpedo_id");
    assert_eq!(b[2], "6100");
    assert_eq!(b[4], "ship_class_facts.csv:6");
    assert!(b[3].starts_with("taken from like boats"), "{}", b[3]);
    assert!(
        b[5].contains("taken from the group nearest in date, us-sub-fleet"),
        "{}",
        b[5]
    );
    assert!(
        b[5].contains("ship_class_facts.csv:6 (preferred; us-sub-fleet torpedo_model us-torp-a)"),
        "{}",
        b[5]
    );
    assert!(b[5].contains("entry by the test, 2026-11-22"), "{}", b[5]);
    assert!(
        b[5].contains("reason: No row names the torpedo"),
        "{}",
        b[5]
    );
}

#[test]
fn a_torpedo_taken_from_her_own_boats_cites_their_events() {
    let files = derives(&the_list(), &base()).submarine.unwrap();
    let c = source_of(&files.class_sources_csv, "6002", "torpedo_id");
    assert_eq!(c[2], "6101");
    assert_eq!(c[4], "engagement_events.csv:1 engagement_events.csv:2");
    assert!(
        c[5].contains("taken from the boats of her group"),
        "{}",
        c[5]
    );
    assert!(
        c[5].contains("engagement_events.csv:1 (time 1942-05-05; actor jp-boat-1; means two torpedoes (jp-torp); preferred)"),
        "{}",
        c[5]
    );
    assert!(c[5].contains(
        "engagement_events.csv:2 (time 1942-05-05; actor jp-boat-2; means a torpedo; unconfirmed)"
    ));
}

#[test]
fn the_passage_speed_of_a_class_is_estimated_by_decision_0207_and_marked() {
    let outputs = derives(&the_list(), &base());
    let files = outputs.submarine.as_ref().unwrap();
    let d = source_of(&files.class_sources_csv, "6003", "passage_speed_kn");
    assert_eq!(d[2], "13");
    assert!(
        d[3].starts_with("estimated, not read from a row (decision 0207)"),
        "{}",
        d[3]
    );
    assert!(
        d[5].contains("x 0.65, the mean over 3 like classes"),
        "{}",
        d[5]
    );
    assert!(d[5].contains("us-sub-a 10 / 20 = 0.5"), "{}", d[5]);
    assert!(d[5].contains("entry by the test, 2026-11-22"), "{}", d[5]);
    let header = comments(&files.class_csv);
    assert!(header.contains(
        "Estimated or taken from like boats, not read (decision 0207; decision 0218, rule 29)"
    ));
    assert!(
        header.contains("jp-sub-d endurance_speed_kn: by ratio"),
        "{header}"
    );
    // The count line the command prints: one speed and three torpedoes.
    assert_eq!(
        outputs.estimated[0],
        "3 figures estimated or taken from like boats (decision 0207; decision 0218, rule 29), for jp-sub-c, jp-sub-d, us-sub-b"
    );
    assert_eq!(outputs.estimated.len(), 4);
}

#[test]
fn the_headers_state_the_rules_the_constant_and_what_is_left_out() {
    let files = derives(&the_list(), &base()).submarine.unwrap();
    let header = comments(&files.class_csv);
    assert!(header.contains("GENERATED by oob-derive"));
    assert!(header.contains("List: subs.toml: Test submarines (as of 1942-08-07)"));
    assert!(header.contains("Rules: rules.toml"));
    assert!(header.contains("decision 0218, rule 28"));
    assert!(header.contains("decision 0218, rule 29"));
    assert!(header.contains("MODEL PARAMETERS AND PLACEHOLDERS"));
    assert!(header.contains("torpedo_damage_per_warhead_kg = 0.4"));
    assert!(header.contains("Exceptions used (decision 0020, 3): none"));
    assert!(header.contains("Left out (decision 0218, rule 1)"));
    assert!(header.contains("us-sub-fleet: Not a class."));
    let torpedo_header = comments(&files.torpedo_csv);
    assert!(torpedo_header.contains("torpedo_damage_per_warhead_kg = 0.4"));
    assert!(torpedo_header.contains("Left out (decision 0218, rule 1): none"));
    let sources = source_of(&files.torpedo_sources_csv, "6101", "damage");
    assert_eq!(sources[2], "120.2");
    assert_eq!(sources[4], "weapon_facts.csv:2");
}

// ------------------------------------------------------------- the refusals

#[test]
fn a_class_with_no_torpedo_row_and_no_entry_is_a_problem() {
    let mut files = base();
    remove_row(&mut files, "ship_class_facts.csv", 5);
    let problems = fails(&the_list(), &files);
    assert_one(&problems, &["us-sub-a", "torpedo_id", "torpedo_from entry"]);
}

#[test]
fn rows_that_disagree_on_the_torpedo_are_a_problem() {
    let mut files = base();
    add_row(
        &mut files,
        "ship_class_facts.csv",
        fact(19, "us-sub-a", "torpedo_model", "jp-torp"),
    );
    // `torpedo_model` allows many rows; the read ones must agree.
    let problems = fails(&the_list(), &files);
    assert_one(&problems, &["us-sub-a", "disagree"]);
}

#[test]
fn a_torpedo_from_is_refused_where_the_class_has_a_torpedo_model_row() {
    // A row of any standing drives the entry out.
    for (standing, value) in [("preferred", "us-torp-a"), ("unconfirmed", "jp-torp-b")] {
        let mut files = base();
        add_row(
            &mut files,
            "ship_class_facts.csv",
            fact_row(19, "us-sub-b", "torpedo_model", value, standing).replace(
                ",medium,",
                if standing == "preferred" {
                    ",medium,"
                } else {
                    ",low,"
                },
            ),
        );
        let problems = fails(&the_list(), &files);
        assert_one(
            &problems,
            &[
                "us-sub-b",
                "a row drives the entry out: take out torpedo_from",
                "ship_class_facts.csv:19",
            ],
        );
    }
}

#[test]
fn a_cited_class_row_of_another_weapon_or_entity_is_a_problem() {
    // Another weapon.
    let mut files = base();
    replace_row(
        &mut files,
        "ship_class_facts.csv",
        6,
        fact(6, "us-sub-fleet", "torpedo_model", "jp-torp"),
    );
    let problems = fails(&the_list(), &files);
    assert_one(
        &problems,
        &[
            "us-sub-b",
            "ship_class_facts.csv:6",
            "whose value is us-torp-a",
        ],
    );
    // Another entity: row 5 is the class A row, not the group's.
    let list = the_list().replace(
        "rows = [\"ship_class_facts.csv:6\"]",
        "rows = [\"ship_class_facts.csv:5\"]",
    );
    let problems = fails(&list, &base());
    assert_one(&problems, &["ship_class_facts.csv:5", "of us-sub-fleet"]);
    // Another attribute: row 4 is a speed.
    let list = the_list().replace(
        "rows = [\"ship_class_facts.csv:6\"]",
        "rows = [\"ship_class_facts.csv:4\"]",
    );
    let problems = fails(&list, &base());
    assert_one(&problems, &["ship_class_facts.csv:4", "torpedo_model row"]);
}

#[test]
fn a_cited_row_that_does_not_exist_is_a_problem() {
    let list = the_list().replace(
        "rows = [\"engagement_events.csv:1\", \"engagement_events.csv:2\"]",
        "rows = [\"engagement_events.csv:1\", \"engagement_events.csv:99\"]",
    );
    let problems = fails(&list, &base());
    assert_one(
        &problems,
        &["jp-sub-c", "engagement_events.csv:99", "not a row"],
    );
    let list = the_list().replace(
        "rows = [\"ship_class_facts.csv:6\"]",
        "rows = [\"ship_class_facts.csv:999\"]",
    );
    let problems = fails(&list, &base());
    assert_one(
        &problems,
        &["us-sub-b", "ship_class_facts.csv:999", "not a row"],
    );
}

#[test]
fn a_row_of_another_file_or_a_bad_row_is_a_list_problem() {
    for bad in [
        "weapon_facts.csv:1",
        "ship_class_facts.csv",
        "engagement_events.csv:x",
    ] {
        let list = the_list().replace("ship_class_facts.csv:6", bad);
        let problems = fails(&list, &base());
        assert_one(&problems, &["list", "is not file:id"]);
    }
}

#[test]
fn a_torpedo_from_with_no_rows_is_a_problem() {
    let list = the_list().replace("rows = [\"ship_class_facts.csv:6\"]", "rows = []");
    let problems = fails(&list, &base());
    assert_one(&problems, &["`rows` is empty"]);
}

#[test]
fn a_group_that_is_not_another_submarine_class_is_a_problem() {
    let list = the_list().replace("group = \"us-sub-fleet\"", "group = \"us-sub-b\"");
    assert!(fails(&list, &base())
        .iter()
        .any(|p| p.contains("us-sub-b") && p.contains("the class itself")));
    let list = the_list().replace("group = \"us-sub-fleet\"", "group = \"us-cruiser\"");
    assert!(fails(&list, &base())
        .iter()
        .any(|p| p.contains("us-cruiser") && p.contains("not submarine")));
    let list = the_list().replace("group = \"us-sub-fleet\"", "group = \"no-such-class\"");
    assert!(fails(&list, &base())
        .iter()
        .any(|p| p.contains("no-such-class") && p.contains("no such class key")));
}

#[test]
fn a_weapon_that_is_not_a_torpedo_or_not_in_the_list_is_a_problem() {
    let list = the_list().replace("weapon = \"us-torp-a\"", "weapon = \"a-gun\"");
    let problems = fails(&list, &base());
    assert!(
        problems
            .iter()
            .any(|p| p.contains("a-gun") && p.contains("kind torpedo")),
        "{problems:#?}"
    );
    // A torpedo missing from `torpedoes` has no id.
    let list = the_list().replace(
        "torpedoes = [\"us-torp-a\", \"jp-torp\", \"jp-torp-b\"]",
        "torpedoes = [\"jp-torp\", \"jp-torp-b\"]",
    );
    let problems = fails(&list, &base());
    assert!(
        problems
            .iter()
            .any(|p| p.contains("us-torp-a") && p.contains("not in the list's `torpedoes`")),
        "{problems:#?}"
    );
}

#[test]
fn a_class_neither_derived_nor_left_out_is_a_problem() {
    let list = the_list().replace(
        "[[left_out]]\nclass = \"us-sub-fleet\"\nreason = \"Not a class.\"\n",
        "",
    );
    let problems = fails(&list, &base());
    assert_one(
        &problems,
        &[
            "us-sub-fleet",
            "neither a [[submarine]] entry nor [[left_out]]",
        ],
    );
}

#[test]
fn a_class_both_derived_and_left_out_is_a_problem() {
    let list = the_list().replace(
        "[[left_out]]\nclass = \"us-sub-fleet\"",
        "[[left_out]]\nclass = \"us-sub-a\"\nreason = \"Also left out.\"\n\n[[left_out]]\nclass = \"us-sub-fleet\"",
    );
    let problems = fails(&list, &base());
    assert_one(
        &problems,
        &["us-sub-a", "both a [[submarine]] entry and [[left_out]]"],
    );
}

#[test]
fn a_class_derived_twice_or_left_out_twice_is_a_problem() {
    let list = the_list().replace(
        "[[submarine]]\nclass = \"jp-sub-d\"\n",
        "[[submarine]]\nclass = \"jp-sub-d\"\n\n[[submarine]]\nclass = \"jp-sub-d\"\n",
    );
    assert_one(
        &fails(&list, &base()),
        &["more than one [[submarine]] entry"],
    );
    let list = the_list().replace(
        "reason = \"Not a class.\"\n",
        "reason = \"Not a class.\"\n\n[[left_out]]\nclass = \"us-sub-fleet\"\nreason = \"Again.\"\n",
    );
    assert_one(&fails(&list, &base()), &["left out more than once"]);
}

#[test]
fn a_left_out_entry_names_one_thing_and_gives_a_reason() {
    let list = the_list().replace(
        "[[left_out]]\nclass = \"us-sub-fleet\"\nreason = \"Not a class.\"\n",
        "[[left_out]]\nclass = \"us-sub-fleet\"\nweapon = \"jp-torp-b\"\nreason = \"Both.\"\n",
    );
    assert!(fails(&list, &base())
        .iter()
        .any(|p| p.contains("exactly one of `class` or `weapon`")));
    let list = the_list().replace("reason = \"Not a class.\"", "reason = \" \"");
    assert!(fails(&list, &base())
        .iter()
        .any(|p| p.contains("`reason` must not be empty")));
}

#[test]
fn a_left_out_weapon_is_a_torpedo_not_derived() {
    // A torpedo that is derived cannot also be left out.
    let list = the_list().replace(
        "[[left_out]]\nclass = \"us-sub-fleet\"",
        "[[left_out]]\nweapon = \"jp-torp-b\"\nreason = \"No.\"\n\n[[left_out]]\nclass = \"us-sub-fleet\"",
    );
    assert_one(
        &fails(&list, &base()),
        &["jp-torp-b", "both in `torpedoes` and left out"],
    );
    // A gun is not a torpedo.
    let list = the_list().replace(
        "[[left_out]]\nclass = \"us-sub-fleet\"",
        "[[left_out]]\nweapon = \"a-gun\"\nreason = \"No.\"\n\n[[left_out]]\nclass = \"us-sub-fleet\"",
    );
    assert_one(&fails(&list, &base()), &["a-gun", "kind torpedo"]);
    // A torpedo not in the list may be left out, with its reason.
    let list = the_list()
        .replace(
            "torpedoes = [\"us-torp-a\", \"jp-torp\", \"jp-torp-b\"]",
            "torpedoes = [\"us-torp-a\", \"jp-torp\"]",
        )
        .replace("6102 = \"jp-torp-b\"\n", "")
        .replace(
            "[[left_out]]\nclass = \"us-sub-fleet\"",
            "[[left_out]]\nweapon = \"jp-torp-b\"\nreason = \"Not carried by a class on file.\"\n\n[[left_out]]\nclass = \"us-sub-fleet\"",
        );
    let files = derives(&list, &base()).submarine.unwrap();
    let header = comments(&files.torpedo_csv);
    assert!(
        header.contains("jp-torp-b: Not carried by a class on file."),
        "{header}"
    );
}

#[test]
fn a_pin_missing_doubled_or_for_nothing_is_a_problem() {
    // Missing.
    let list = the_list().replace("6003 = \"jp-sub-d\"\n", "");
    assert_one(&fails(&list, &base()), &["jp-sub-d", "no id pinned"]);
    let list = the_list().replace("6102 = \"jp-torp-b\"\n", "");
    assert_one(&fails(&list, &base()), &["jp-torp-b", "no id pinned"]);
    // A class pinned twice (two ids).
    let list = the_list().replace(
        "6003 = \"jp-sub-d\"\n",
        "6003 = \"jp-sub-d\"\n6004 = \"jp-sub-d\"\n",
    );
    assert_one(&fails(&list, &base()), &["jp-sub-d", "pinned twice"]);
    // A torpedo pinned twice.
    let list = the_list().replace(
        "6102 = \"jp-torp-b\"\n",
        "6102 = \"jp-torp-b\"\n6103 = \"jp-torp-b\"\n",
    );
    assert_one(&fails(&list, &base()), &["jp-torp-b", "pinned twice"]);
    // An id given to something not derived.
    let list = the_list().replace(
        "6003 = \"jp-sub-d\"\n",
        "6003 = \"jp-sub-d\"\n6004 = \"us-sub-fleet\"\n",
    );
    assert_one(
        &fails(&list, &base()),
        &["us-sub-fleet", "not a [[submarine]] entry"],
    );
    let list = the_list().replace(
        "6102 = \"jp-torp-b\"\n",
        "6102 = \"jp-torp-b\"\n6103 = \"a-gun\"\n",
    );
    assert_one(&fails(&list, &base()), &["a-gun", "not in `torpedoes`"]);
    // The pin tables are required.
    let list = the_list().replace(PINS, "");
    let problems = fails(&list, &base());
    assert!(problems.iter().any(|p| p.contains("submarine_class_ids")));
    assert!(problems.iter().any(|p| p.contains("submarine_torpedo_ids")));
}

#[test]
fn the_two_outputs_go_together() {
    let list = the_list().replace(
        "submarine_torpedo_output = \"../reference/torps.csv\"\n",
        "",
    );
    assert_one(
        &fails(&list, &base()),
        &["submarine_torpedo_output", "both or neither"],
    );
}

#[test]
fn a_figure_that_is_not_a_whole_number_or_is_zero_is_a_problem() {
    let mut files = base();
    replace_row(
        &mut files,
        "ship_class_facts.csv",
        1,
        fact(1, "us-sub-a", "torpedo_tubes_bow", "0"),
    );
    let problems = fails(&the_list(), &files);
    assert_one(&problems, &["us-sub-a", "torpedo_tubes_bow", "above 0"]);
    // Fewer torpedoes than one salvo: 0 salvoes.
    let mut files = base();
    replace_row(
        &mut files,
        "ship_class_facts.csv",
        2,
        fact(2, "us-sub-a", "torpedoes_carried", "3"),
    );
    let problems = fails(&the_list(), &files);
    assert_one(
        &problems,
        &["us-sub-a", "salvoes_carried", "under one salvo"],
    );
}

#[test]
fn a_fact_that_cannot_be_read_is_a_problem_as_for_a_ship() {
    // Not preferred, and no exception.
    let mut files = base();
    replace_row(
        &mut files,
        "ship_class_facts.csv",
        2,
        fact_row(2, "us-sub-a", "torpedoes_carried", "22", "disputed"),
    );
    let problems = fails(&the_list(), &files);
    assert_one(&problems, &["us-sub-a", "salvoes_carried", "not preferred"]);
    // A torpedo with no warhead row.
    let mut files = base();
    remove_row(&mut files, "weapon_facts.csv", 3);
    let problems = fails(&the_list(), &files);
    assert_one(&problems, &["jp-torp-b", "damage"]);
}

#[test]
fn an_estimate_is_refused_where_a_row_of_the_attribute_is_filed() {
    let mut files = base();
    add_row(
        &mut files,
        "ship_class_facts.csv",
        fact_row(19, "jp-sub-d", "endurance_speed_kn", "12", "disputed"),
    );
    let problems = fails(&the_list(), &files);
    assert_one(
        &problems,
        &[
            "jp-sub-d",
            "a row is filed (ship_class_facts.csv:19)",
            "decision 0207, rule 11",
        ],
    );
}

#[test]
fn an_estimate_is_refused_where_no_gap_row_names_the_class() {
    let mut files = base();
    files.insert(
        "gaps.csv".to_string(),
        "entity_type,entity,what\n".to_string(),
    );
    let problems = fails(&the_list(), &files);
    assert_one(
        &problems,
        &["class jp-sub-d", "gaps.csv", "decision 0207, rule 2"],
    );
}

#[test]
fn without_an_estimate_a_class_with_no_speed_is_a_problem() {
    let list = list_text("", &format!("{ENTRIES}{PINS}"));
    let problems = fails(&list, &base());
    assert_one(&problems, &["jp-sub-d", "passage_speed_kn", "no rows"]);
}

#[test]
fn an_estimate_no_figure_reads_is_a_problem() {
    // The class has her row and the entry would be driven out; here the
    // estimate names a class that is not listed at all.
    let tables = format!(
        "{ESTIMATE}{}{ENTRIES}{PINS}",
        ESTIMATE.replace("jp-sub-d", "us-sub-fleet")
    );
    let problems = fails(&list_text("", &tables), &base());
    assert!(
        problems
            .iter()
            .any(|p| p.contains("us-sub-fleet") && p.contains("read by no figure")),
        "{problems:#?}"
    );
}

#[test]
fn only_the_speed_of_a_submarine_class_may_be_estimated() {
    let tables = format!(
        "{ESTIMATE}{ENTRIES}{PINS}{}",
        r#"
[[estimate]]
class = "jp-sub-d"
attribute = "max_speed_kn"
method = "ratio"
from = "endurance_speed_kn"
by = "the test"
date = "2026-11-22"
reason = "No."
"#
    );
    let problems = fails(&list_text("", &tables), &base());
    assert!(
        problems
            .iter()
            .any(|p| p.contains("only endurance_speed_kn of a submarine class may be estimated")),
        "{problems:#?}"
    );
}

#[test]
fn an_estimate_never_rests_on_an_estimate() {
    // A like class whose speed is estimated in another list of the folder.
    let other = BTreeMap::from([(
        "other.toml".to_string(),
        "[[estimate]]\nclass = \"us-sub-a\"\nattribute = \"endurance_speed_kn\"\nmethod = \"ratio\"\nfrom = \"max_speed_kn\"\nby = \"x\"\ndate = \"2026-11-22\"\nreason = \"x\"\n".to_string(),
    )]);
    let problems = oob_derive::derive_among("subs.toml", &the_list(), RULES, &base(), &other)
        .expect_err("problems expected");
    assert!(
        problems.iter().any(|p| p
            .to_string()
            .contains("an estimate never rests on an estimate")),
        "{problems:#?}"
    );
}

#[test]
fn a_like_class_that_cannot_be_read_fails_the_estimate() {
    let mut files = base();
    replace_row(
        &mut files,
        "ship_class_facts.csv",
        14,
        fact_row(14, "jp-sub-c", "max_speed_kn", "20", "disputed"),
    );
    let problems = fails(&the_list(), &files);
    assert!(
        problems.iter().any(|p| p.contains("jp-sub-d")
            && p.contains("jp-sub-c")
            && p.contains("not preferred")),
        "{problems:#?}"
    );
}

#[test]
fn a_ship_estimate_on_a_submarine_list_is_still_refused() {
    let tables = ESTIMATE.replace("class = \"jp-sub-d\"", "ship = \"jp-sub-d\"");
    let list = list_text("", &format!("{tables}{ENTRIES}{PINS}"));
    let problems = fails(&list, &base());
    assert!(
        problems
            .iter()
            .any(|p| p.contains("`jp-sub-d` is not in ships")),
        "{problems:#?}"
    );
}

#[test]
fn every_problem_is_reported_and_nothing_is_written() {
    // Two faults: both are reported in one run.
    let mut files = base();
    remove_row(&mut files, "ship_class_facts.csv", 5);
    remove_row(&mut files, "weapon_facts.csv", 3);
    let problems = fails(&the_list(), &files);
    assert_eq!(problems.len(), 2, "{problems:#?}");

    // The run writes no file on a problem, and all four on none.
    let dir = scratch("nothing_written");
    for (name, text) in &files {
        std::fs::write(dir.join("oob").join(name), text).unwrap();
    }
    std::fs::write(dir.join("derivation/subs.toml"), the_list()).unwrap();
    std::fs::write(dir.join("derivation/rules.toml"), RULES).unwrap();
    let list_path = dir.join("derivation/subs.toml");
    match run(&list_path, &dir.join("oob"), false).expect("run") {
        Outcome::Problems(found) => assert_eq!(found.len(), 2),
        Outcome::Clean { .. } => panic!("problems expected"),
    }
    assert!(std::fs::read_dir(dir.join("reference"))
        .unwrap()
        .next()
        .is_none());

    for (name, text) in &base() {
        std::fs::write(dir.join("oob").join(name), text).unwrap();
    }
    // A check writes nothing either.
    match run(&list_path, &dir.join("oob"), true).expect("run") {
        Outcome::Clean { files, written, .. } => {
            assert!(!written);
            assert_eq!(files.len(), 4);
        }
        Outcome::Problems(found) => panic!("{found:#?}"),
    }
    assert!(std::fs::read_dir(dir.join("reference"))
        .unwrap()
        .next()
        .is_none());
    match run(&list_path, &dir.join("oob"), false).expect("run") {
        Outcome::Clean { written, .. } => assert!(written),
        Outcome::Problems(found) => panic!("{found:#?}"),
    }
    let mut written: Vec<String> = std::fs::read_dir(dir.join("reference"))
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    written.sort();
    assert_eq!(
        written,
        [
            "subs.csv",
            "subs.sources.csv",
            "torps.csv",
            "torps.sources.csv"
        ]
    );
}

fn scratch(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("oob_derive_subs_{name}"));
    let _ = std::fs::remove_dir_all(&dir);
    for sub in ["derivation", "oob", "reference"] {
        std::fs::create_dir_all(dir.join(sub)).unwrap();
    }
    dir
}

// ----------------------------------------------------- the real data and rules

#[test]
fn the_submarine_list_derives_exactly_the_files_in_data_reference() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let read = |path: &str| std::fs::read_to_string(root.join(path)).expect(path);
    let (facts, unread) = oob_check::read_dir(&root.join("data/oob"));
    assert!(unread.is_empty());
    let outputs = derive(
        "submarines_1942.toml",
        &read("data/derivation/submarines_1942.toml"),
        &read("data/derivation/rules.toml"),
        &facts,
    )
    .unwrap_or_else(|p| panic!("{p:#?}"));
    let files = outputs.submarine.expect("submarine files");
    assert_eq!(
        files.class_csv,
        read("data/reference/submarine_classes_1942.csv")
    );
    assert_eq!(
        files.class_sources_csv,
        read("data/reference/submarine_classes_1942.sources.csv")
    );
    assert_eq!(
        files.torpedo_csv,
        read("data/reference/submarine_torpedoes_1942.csv")
    );
    assert_eq!(
        files.torpedo_sources_csv,
        read("data/reference/submarine_torpedoes_1942.sources.csv")
    );
}
