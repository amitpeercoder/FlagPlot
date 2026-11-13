//! Ships' experience values (decision 0064), against a small synthetic
//! facts directory and against the real files.
//!
//! # The rule, worked by hand for the synthetic cases
//!
//! A value starts at 2. Each question a row answers moves it by 1, once:
//! `night_practice_recent` up, `navy_night_first` up, `no_night_practice`
//! down, `new_ship` down. It is then clamped to 0 to 4. A question no row
//! answers moves nothing.
//!
//! The synthetic engagement `test-battle` has `date_from` 1942-08-08. So:
//!
//! - the three months run from 1942-05-08 to 1942-08-08, both days inside;
//! - the twelve months run from 1941-08-08 to 1942-08-08;
//! - a ship is new if commissioned after 1941-08-08 (1941-08-09 is new;
//!   1941-08-08, exactly a year before, is not).
//!
//! Counted by the calendar, 8 May is 3 months before 8 August, and 8 August
//! 1941 is 12 months before 8 August 1942.
//!
//! | Case | Working | Value |
//! | --- | --- | --- |
//! | no row answers anything | 2 | 2 |
//! | practice at night on 1942-05-08 (the first day of the window) | 2 + 1 | 3 |
//! | the same on 1942-05-07 | outside the window: a problem | none |
//! | practice on 1942-08-08 (the last day) | 2 + 1 | 3 |
//! | practice on 1942-08-09 | outside: a problem | none |
//! | no practice over a period ending 1941-08-08 | reaches the twelve months: 2 - 1 | 1 |
//! | the same ending 1941-08-07 | does not reach: a problem | none |
//! | commissioned 1941-08-09 | new: 2 - 1 | 1 |
//! | commissioned 1941-08-08 | not new: 2 | 2 |
//! | Japanese heavy cruiser, no other row | 2 + 1 (navy) | 3, provisional |
//! | Japanese ship with a recent practice too | 2 + 1 + 1 | 4 |
//! | ship with no practice in the year and new | 2 - 1 - 1 | 0 |
//! | recent practice and no practice in the year (two rows) | 2 + 1 - 1 | 2 |
//! | two rows say the same, yes | still one move: 2 + 1 | 3 |

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use oob_derive::{derive, run, Outcome, Outputs};

type Files = BTreeMap<String, String>;

const FACT_HEADER: &str = "id,entity,attribute,value,valid_from,valid_to,\
    source,locator,accessed,confidence,basis,standing,note\n";

const TRAINING_HEADER: &str = "id,entity,attribute,value,as_stated,valid_from,valid_to,\
    source,locator,accessed,confidence,basis,standing,note\n";

const RULES: &str = "constants_are_placeholders = true\n\
    gun_shell_kg_per_damage = 6\n\
    torpedo_damage_per_warhead_kg = \"0.4\"\n\
    displacement_lt_per_toughness = 25\n\
    [experience]\n\
    recent_months = 3\n\
    lookback_months = 12\n\
    new_ship_months = 12\n\
    [[experience.navy]]\n\
    nation = \"jp\"\n\
    entity = \"ijn-cruiser-destroyer-crews\"\n\
    rows = [1]\n\
    kinds = [\"heavy_cruiser\"]\n";

fn ship_fact(
    id: u32,
    entity: &str,
    attribute: &str,
    value: &str,
    confidence: &str,
    standing: &str,
) -> String {
    format!("{id},{entity},{attribute},{value},,,a,p.1,2026-11-10,{confidence},stated,{standing},a note")
}

fn training(
    id: u32,
    entity: &str,
    attribute: &str,
    source: &str,
    confidence: &str,
    standing: &str,
) -> String {
    format!(
        "{id},{entity},{attribute},a statement,as stated,,,{source},p.1,2026-11-12,{confidence},stated,{standing},a note"
    )
}

/// The base set. Ships: `us1` to `us4` (US heavy cruisers), `jp1` (Japanese
/// heavy cruiser), `jpcv` (Japanese carrier). Night practice rows: 101
/// (us1, preferred, medium), 102 (us1, preferred, high), 103 (us1,
/// unconfirmed, low), 104 (us2), 105 (us3), 106 (jp1), 107 (us4).
/// Training rows: 1 (Japan, source a, preferred), 2 (Japan, source b,
/// preferred), 3 (US navy), 4 (Japan, a `training_note`, not night).
fn base() -> Files {
    let empty = FACT_HEADER.to_string();
    let mut ship_facts = FACT_HEADER.to_string();
    for row in [
        ship_fact(
            101,
            "us1",
            "night_practice_note",
            "a practice",
            "medium",
            "preferred",
        ),
        ship_fact(
            102,
            "us1",
            "night_practice_note",
            "another",
            "high",
            "preferred",
        ),
        ship_fact(
            103,
            "us1",
            "night_practice_note",
            "a weak one",
            "low",
            "unconfirmed",
        ),
        ship_fact(
            104,
            "us2",
            "night_practice_note",
            "a practice",
            "medium",
            "preferred",
        ),
        ship_fact(
            105,
            "us3",
            "night_practice_note",
            "a practice",
            "medium",
            "preferred",
        ),
        ship_fact(
            106,
            "jp1",
            "night_practice_note",
            "a practice",
            "medium",
            "preferred",
        ),
        ship_fact(
            107,
            "us4",
            "night_practice_note",
            "a practice",
            "medium",
            "preferred",
        ),
    ] {
        ship_facts.push_str(&row);
        ship_facts.push('\n');
    }
    let mut training_facts = TRAINING_HEADER.to_string();
    for row in [
        training(
            1,
            "ijn-cruiser-destroyer-crews",
            "night_training_note",
            "a",
            "medium",
            "preferred",
        ),
        training(
            2,
            "ijn-cruiser-destroyer-crews",
            "night_training_note",
            "b",
            "high",
            "preferred",
        ),
        training(
            3,
            "usn-cruiser-destroyer-crews",
            "night_training_note",
            "a",
            "medium",
            "preferred",
        ),
        training(
            4,
            "ijn-cruiser-destroyer-crews",
            "training_note",
            "a",
            "medium",
            "preferred",
        ),
    ] {
        training_facts.push_str(&row);
        training_facts.push('\n');
    }
    let files = [
        (
            "sources.csv",
            "id,key,reliability\n1000,a,A\n1001,b,B\n1002,c,C\n".to_string(),
        ),
        ("places.csv", "id,key\n".to_string()),
        (
            "engagements.csv",
            "id,key,date_from,date_to\n1000,test-battle,1942-08-08,1942-08-09\n".to_string(),
        ),
        (
            "ship_classes.csv",
            "id,key,name,nation,type\n\
             1000,us-heavy,US heavy,us,heavy_cruiser\n\
             1001,jp-heavy,JP heavy,jp,heavy_cruiser\n\
             1002,jp-carrier,JP carrier,jp,carrier\n"
                .to_string(),
        ),
        (
            "ships.csv",
            "id,key,name,nation,class,hull_number\n\
             1000,us1,Us 1,us,us-heavy,\n\
             1001,us2,Us 2,us,us-heavy,\n\
             1002,us3,Us 3,us,us-heavy,\n\
             1003,us4,Us 4,us,us-heavy,\n\
             1004,jp1,Jp 1,jp,jp-heavy,\n\
             1005,jpcv,Jp cv,jp,jp-carrier,\n"
                .to_string(),
        ),
        ("aircraft_types.csv", "id,key\n".to_string()),
        ("weapons.csv", "id,key\n".to_string()),
        ("sensors.csv", "id,key\n".to_string()),
        ("air_units.csv", "id,key,parent\n".to_string()),
        (
            "attributes.csv",
            "key,applies_to,value_type,cardinality\n\
             commissioned,ship,date,one\n\
             night_practice_note,ship,text,many\n"
                .to_string(),
        ),
        ("gaps.csv", "entity_type,entity,what\n".to_string()),
        ("ship_class_facts.csv", empty.clone()),
        ("ship_facts.csv", ship_facts),
        ("aircraft_type_facts.csv", empty.clone()),
        ("weapon_facts.csv", empty.clone()),
        ("sensor_facts.csv", empty.clone()),
        ("air_unit_facts.csv", empty),
        ("training_facts.csv", training_facts),
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
            "id,engagement,time,range_m,bearing_deg,value,source,locator,accessed,confidence,basis,standing,note\n"
                .to_string(),
        ),
        (
            "engagement_positions.csv",
            "id,engagement,time,lat,lon,bearing_deg,distance_nm,course_deg,speed_kn,source,locator,accessed,confidence,basis,standing,note\n"
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

fn with_rows(file: &str, rows: &[String]) -> Files {
    let mut files = base();
    let text = files.get_mut(file).expect("a file of the base set");
    for row in rows {
        text.push_str(row);
        text.push('\n');
    }
    files
}

fn commissioned(id: u32, ship: &str, value: &str, standing: &str) -> String {
    let confidence = if standing == "unconfirmed" {
        "low"
    } else {
        "medium"
    };
    ship_fact(id, ship, "commissioned", value, confidence, standing)
}

/// A list with no class ships and the given ships' experience.
fn list(ships: &[&str], tables: &str) -> String {
    let quoted: Vec<String> = ships.iter().map(|s| format!("\"{s}\"")).collect();
    format!(
        "name = \"Test\"\ndate = \"1942-08-08\"\nrules = \"rules.toml\"\nships = []\n\
         [experience]\nengagement = \"test-battle\"\n\
         output = \"../reference/exp.ships.csv\"\nships = [{}]\n{tables}",
        quoted.join(", ")
    )
}

fn answer(ship: &str, question: &str, row: u32, yes: bool, from: &str, to: &str) -> String {
    format!(
        "[[experience.answer]]\nship = \"{ship}\"\nquestion = \"{question}\"\n\
         row = \"ship_facts.csv:{row}\"\nanswer = \"{}\"\nfrom = \"{from}\"\nto = \"{to}\"\n\
         reason = \"a reason\"\n",
        if yes { "yes" } else { "no" }
    )
}

const Q1: &str = "night_practice_recent";
const Q2: &str = "no_night_practice";

fn derive_with(
    ships: &[&str],
    tables: &str,
    rules: &str,
    files: &Files,
) -> Result<Outputs, Vec<String>> {
    derive("list.toml", &list(ships, tables), rules, files)
        .map_err(|problems| problems.iter().map(ToString::to_string).collect())
}

#[track_caller]
fn derives(ships: &[&str], tables: &str, files: &Files) -> String {
    derive_with(ships, tables, RULES, files)
        .unwrap_or_else(|p| panic!("{p:#?}"))
        .experience_csv
        .expect("an experience file")
}

#[track_caller]
fn fails(ships: &[&str], tables: &str, files: &Files) -> Vec<String> {
    derive_with(ships, tables, RULES, files).expect_err("problems expected")
}

#[track_caller]
fn assert_one(problems: &[String], expected: &[&str]) {
    assert_eq!(problems.len(), 1, "{problems:#?}");
    for want in expected {
        assert!(problems[0].contains(want), "`{want}` in {problems:#?}");
    }
}

/// The record of one ship: experience, fixed, the four question columns,
/// rows.
fn record(csv_text: &str, ship: &str) -> Vec<String> {
    csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .from_reader(csv_text.as_bytes())
        .records()
        .map(|r| {
            r.expect("a record")
                .iter()
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .find(|r| r[0] == ship)
        .unwrap_or_else(|| panic!("no record for {ship} in {csv_text}"))
}

#[track_caller]
fn value_of(csv_text: &str, ship: &str) -> String {
    record(csv_text, ship)[1].clone()
}

// Question 1.

#[test]
fn with_no_row_answering_a_ship_stays_at_two() {
    let text = derives(&["us1", "jpcv"], "", &base());
    // 2 + 0.
    let us1 = record(&text, "us1");
    assert_eq!(us1[1..7], ["2", "fixed", "none", "none", "none", "none"]);
    // A Japanese carrier is not a kind the navy row covers: 2.
    assert_eq!(record(&text, "jpcv")[1], "2");
}

#[test]
fn a_recent_night_practice_moves_the_value_up_one_inside_the_window() {
    // 8 May to 8 August 1942, both ends inside.
    for day in ["1942-05-08", "1942-06-15", "1942-08-08"] {
        let text = derives(&["us1"], &answer("us1", Q1, 101, true, day, day), &base());
        let r = record(&text, "us1");
        // 2 + 1 = 3.
        assert_eq!(r[1], "3", "{day}");
        assert_eq!(r[3], "up");
        assert_eq!(r[7], "ship_facts.csv:101");
    }
}

#[test]
fn a_practice_outside_the_three_months_is_a_problem() {
    for day in ["1942-05-07", "1942-08-09", "1942-01-01"] {
        let problems = fails(&["us1"], &answer("us1", Q1, 101, true, day, day), &base());
        assert_one(
            &problems,
            &[
                "us1: experience: night_practice_recent",
                "does not meet the window",
            ],
        );
    }
}

#[test]
fn a_period_in_partial_dates_counts_if_it_can_lie_inside() {
    // May could be after the 8th, July could be before the 8th of August.
    let text = derives(
        &["us1"],
        &answer("us1", Q1, 101, true, "1942-05", "1942-07"),
        &base(),
    );
    assert_eq!(value_of(&text, "us1"), "3");
    // April ends on the 30th, before the window: no.
    let problems = fails(
        &["us1"],
        &answer("us1", Q1, 101, true, "1942-04", "1942-05-07"),
        &base(),
    );
    assert_one(&problems, &["does not meet the window"]);
}

#[test]
fn two_rows_answering_yes_move_the_value_once() {
    let tables = format!(
        "{}{}",
        answer("us1", Q1, 101, true, "1942-06-01", "1942-06-01"),
        answer("us1", Q1, 102, true, "1942-07-01", "1942-07-01")
    );
    let text = derives(&["us1"], &tables, &base());
    let r = record(&text, "us1");
    // 2 + 1, not 2 + 2.
    assert_eq!(r[1], "3");
    assert_eq!(r[7], "ship_facts.csv:101|ship_facts.csv:102");
}

// Question 2.

#[test]
fn no_practice_over_a_period_reaching_into_the_year_moves_down_one() {
    // The period ends on the first day of the twelve months: it reaches in.
    for to in ["1941-08-08", "1942-03", "1942-08-08"] {
        let text = derives(&["us2"], &answer("us2", Q2, 104, true, "", to), &base());
        let r = record(&text, "us2");
        // 2 - 1 = 1.
        assert_eq!(r[1], "1", "{to}");
        assert_eq!(r[4], "down");
    }
}

#[test]
fn a_period_that_ended_before_the_year_does_not_reach_it() {
    let problems = fails(
        &["us2"],
        &answer("us2", Q2, 104, true, "1940-01", "1941-08-07"),
        &base(),
    );
    assert_one(
        &problems,
        &[
            "us2: experience: no_night_practice",
            "does not meet the window",
        ],
    );
}

#[test]
fn a_period_that_began_after_the_battle_does_not_reach_the_year() {
    let problems = fails(
        &["us2"],
        &answer("us2", Q2, 104, true, "1942-09-01", "1942-10-01"),
        &base(),
    );
    assert_one(&problems, &["does not meet the window"]);
}

// Question 3.

#[test]
fn a_ship_commissioned_less_than_a_year_before_is_new() {
    let files = with_rows(
        "ship_facts.csv",
        &[
            commissioned(201, "us1", "1941-08-09", "preferred"),
            commissioned(202, "us2", "1941-08-08", "preferred"),
            commissioned(203, "us3", "1941-09", "preferred"),
        ],
    );
    let text = derives(&["us1", "us2", "us3"], "", &files);
    // 1941-08-09 is after 1941-08-08: new, 2 - 1 = 1.
    let us1 = record(&text, "us1");
    assert_eq!(us1[1], "1");
    assert_eq!(us1[5], "down");
    assert_eq!(us1[7], "ship_facts.csv:201");
    // Exactly a year before is not "less than a year": 2.
    assert_eq!(value_of(&text, "us2"), "2");
    // September 1941 cannot be before the line: new, 1.
    assert_eq!(value_of(&text, "us3"), "1");
}

#[test]
fn a_commissioning_date_that_cannot_be_told_from_the_line_is_a_problem() {
    let files = with_rows(
        "ship_facts.csv",
        &[commissioned(201, "us1", "1941-08", "preferred")],
    );
    let problems = fails(&["us1"], "", &files);
    assert_one(&problems, &["us1: ambiguous-date: new_ship", "1941-08"]);
}

#[test]
fn a_ship_with_no_commissioning_row_is_not_answered() {
    let text = derives(&["us1"], "", &base());
    assert_eq!(record(&text, "us1")[5], "none");
}

#[test]
fn when_every_commissioning_row_is_disputed_they_answer_if_they_agree() {
    // 1942 and 1942-04-02 both say new; 2 - 1 = 1.
    let files = with_rows(
        "ship_facts.csv",
        &[
            commissioned(201, "us1", "1942", "disputed"),
            commissioned(202, "us1", "1942-04-02", "disputed"),
            // Both before the line: not new, 2.
            commissioned(203, "us2", "1938-08-12", "disputed"),
            commissioned(204, "us2", "1939-02-01", "disputed"),
        ],
    );
    let text = derives(&["us1", "us2"], "", &files);
    assert_eq!(value_of(&text, "us1"), "1");
    assert_eq!(value_of(&text, "us2"), "2");
    assert_eq!(
        record(&text, "us1")[7],
        "ship_facts.csv:201|ship_facts.csv:202"
    );
}

#[test]
fn disputed_commissioning_rows_that_disagree_are_a_problem_to_settle() {
    let files = with_rows(
        "ship_facts.csv",
        &[
            commissioned(201, "us1", "1941-01-01", "disputed"),
            commissioned(202, "us1", "1942-01-01", "disputed"),
        ],
    );
    let problems = fails(&["us1"], "", &files);
    assert_one(&problems, &["us1: conflict: new_ship", "settle"]);
}

#[test]
fn a_preferred_commissioning_row_is_read_and_an_alternative_is_not() {
    let files = with_rows(
        "ship_facts.csv",
        &[
            commissioned(201, "us1", "1930-01-01", "preferred"),
            commissioned(202, "us1", "1942-01-01", "alternative"),
        ],
    );
    let text = derives(&["us1"], "", &files);
    let r = record(&text, "us1");
    // The preferred row says 1930: not new, 2. The alternative moves nothing,
    // and is in the list that is printed, not in the file.
    assert_eq!(r[1], "2");
    assert_eq!(r[7], "ship_facts.csv:201");
    assert!(!text.contains("202"), "{text}");
    let unread = derive_with(&["us1"], "", RULES, &files)
        .expect("derives")
        .experience_unread;
    assert!(
        unread[0].contains("ship_facts.csv:202(alternative)"),
        "{unread:?}"
    );
}

#[test]
fn commissioning_rows_none_of_which_is_preferred_nor_all_disputed_are_a_problem() {
    let files = with_rows(
        "ship_facts.csv",
        &[
            commissioned(201, "us1", "1942-01-01", "unconfirmed"),
            commissioned(202, "us1", "1942-02-01", "alternative"),
        ],
    );
    let problems = fails(&["us1"], "", &files);
    assert_one(&problems, &["us1: not-preferred: new_ship"]);
}

// Question 4 and provisional values.

#[test]
fn a_japanese_cruiser_gets_the_navy_row_and_is_provisional() {
    let text = derives(&["jp1"], "", &base());
    let r = record(&text, "jp1");
    // 2 + 1 = 3, provisional: the rules file lists no second source.
    assert_eq!(r[1..7], ["3", "provisional", "none", "none", "none", "up"]);
    assert_eq!(r[7], "training_facts.csv:1");
    assert!(text.contains("# PROVISIONAL:"), "{text}");
}

#[test]
fn a_nation_without_a_navy_row_is_not_answered_no() {
    // The US has no row in the rules file: nothing moves, 2.
    let text = derives(&["us1"], "", &base());
    assert_eq!(value_of(&text, "us1"), "2");
    assert!(!text.contains("PROVISIONAL"), "{text}");
}

#[test]
fn a_second_source_from_another_source_makes_the_value_fixed() {
    let rules = RULES.replace(
        "kinds = [\"heavy_cruiser\"]",
        "kinds = [\"heavy_cruiser\"]\nsecond_source = [2]",
    );
    let text = derive_with(&["jp1"], "", &rules, &base())
        .unwrap_or_else(|p| panic!("{p:#?}"))
        .experience_csv
        .unwrap();
    let r = record(&text, "jp1");
    // Still 2 + 1 = 3, now fixed.
    assert_eq!(r[1..3], ["3", "fixed"]);
    assert!(!text.contains("# PROVISIONAL"), "{text}");
}

#[test]
fn a_second_source_row_must_be_a_night_training_row() {
    let rules = RULES.replace(
        "kinds = [\"heavy_cruiser\"]",
        "kinds = [\"heavy_cruiser\"]\nsecond_source = [4]",
    );
    let problems = derive_with(&["jp1"], "", &rules, &base()).expect_err("problems");
    // Row 4 is a `training_note`, not a night training row.
    assert_one(
        &problems,
        &["rules: rules: navy jp", "training_facts.csv:4"],
    );
}

#[test]
fn a_navy_row_must_exist_and_be_of_the_named_entity() {
    let rules = RULES.replace("rows = [1]", "rows = [99]");
    let problems = derive_with(&["jp1"], "", &rules, &base()).expect_err("problems");
    assert_one(&problems, &["navy jp", "does not exist"]);
    let rules = RULES.replace("rows = [1]", "rows = [3]");
    let problems = derive_with(&["jp1"], "", &rules, &base()).expect_err("problems");
    assert_one(&problems, &["navy jp", "preferred, medium or high"]);
}

#[test]
fn a_second_source_must_not_come_from_the_first_rows_source() {
    // Row 1 is from source a; a second row from source a is no second source.
    let mut files = base();
    files.get_mut("training_facts.csv").unwrap().push_str(
        &(training(
            5,
            "ijn-cruiser-destroyer-crews",
            "night_training_note",
            "a",
            "high",
            "preferred",
        ) + "\n"),
    );
    let rules = RULES.replace(
        "kinds = [\"heavy_cruiser\"]",
        "kinds = [\"heavy_cruiser\"]\nsecond_source = [5]",
    );
    let problems = derive_with(&["jp1"], "", &rules, &files).expect_err("problems");
    assert_one(&problems, &["is from the source of the first rows"]);
}

#[test]
fn a_second_source_may_be_a_row_of_the_entity_the_rules_name_and_need_not_be_preferred() {
    // Row 5: the navy's whole surface fleet, source b, one witness
    // (unconfirmed, low). Not disputed, a different source: it is one.
    let mut files = base();
    files.get_mut("training_facts.csv").unwrap().push_str(
        &(training(
            5,
            "ijn-surface-fleet",
            "night_training_note",
            "b",
            "low",
            "unconfirmed",
        ) + "\n"),
    );
    let rules = RULES.replace(
        "kinds = [\"heavy_cruiser\"]",
        "kinds = [\"heavy_cruiser\"]\nsecond_source = [5]\nsecond_source_entity = \"ijn-surface-fleet\"",
    );
    let text = derive_with(&["jp1"], "", &rules, &files)
        .unwrap_or_else(|p| panic!("{p:#?}"))
        .experience_csv
        .unwrap();
    // 2 + 1 = 3, fixed; the header names both rows.
    assert_eq!(record(&text, "jp1")[1..3], ["3", "fixed"]);
    assert!(
        text.contains("second source: training_facts.csv:5."),
        "{text}"
    );
    // The same row without the entity named is of the wrong entity.
    let rules = RULES.replace(
        "kinds = [\"heavy_cruiser\"]",
        "kinds = [\"heavy_cruiser\"]\nsecond_source = [5]",
    );
    let problems = derive_with(&["jp1"], "", &rules, &files).expect_err("problems");
    assert_one(
        &problems,
        &[
            "navy jp",
            "is not a `night_training_note` row of `ijn-cruiser-destroyer-crews`",
        ],
    );
}

#[test]
fn a_disputed_row_is_no_second_source() {
    let mut files = base();
    files.get_mut("training_facts.csv").unwrap().push_str(
        &(training(
            5,
            "ijn-cruiser-destroyer-crews",
            "night_training_note",
            "b",
            "low",
            "disputed",
        ) + "\n"),
    );
    let rules = RULES.replace(
        "kinds = [\"heavy_cruiser\"]",
        "kinds = [\"heavy_cruiser\"]\nsecond_source = [5]",
    );
    let problems = derive_with(&["jp1"], "", &rules, &files).expect_err("problems");
    assert_one(&problems, &["navy jp", "not disputed"]);
}

// Combinations and the clamp.

#[test]
fn the_questions_add_and_the_value_is_clamped() {
    let files = with_rows(
        "ship_facts.csv",
        &[
            commissioned(201, "us2", "1942-03-01", "preferred"),
            commissioned(202, "us3", "1942-03-01", "preferred"),
        ],
    );
    let tables = format!(
        "{}{}{}{}",
        // Japanese ship with a recent practice: 2 + 1 (practice) + 1 (navy) = 4.
        answer("jp1", Q1, 106, true, "1942-07-01", "1942-07-01"),
        // New and no practice in the year: 2 - 1 - 1 = 0.
        answer("us2", Q2, 104, true, "", "1942-03"),
        // A recent practice and no practice in the year: 2 + 1 - 1 = 2.
        answer("us4", Q1, 107, true, "1942-07-01", "1942-07-01"),
        answer("us4", Q2, 107, true, "", "1942-03"),
    );
    let text = derives(&["jp1", "us2", "us3", "us4"], &tables, &files);
    assert_eq!(value_of(&text, "jp1"), "4");
    assert_eq!(value_of(&text, "us2"), "0");
    // New alone: 2 - 1 = 1.
    assert_eq!(value_of(&text, "us3"), "1");
    assert_eq!(value_of(&text, "us4"), "2");
}

// Conflicts and settlements.

#[test]
fn a_row_for_and_a_row_against_stop_the_derivation() {
    let tables = format!(
        "{}{}",
        answer("us1", Q1, 101, true, "1942-06-01", "1942-06-01"),
        answer("us1", Q1, 102, false, "1942-06-01", "1942-07-01")
    );
    let problems = fails(&["us1"], &tables, &base());
    assert_one(
        &problems,
        &[
            "us1: conflict: night_practice_recent",
            "yes by ship_facts.csv:101",
            "no by ship_facts.csv:102",
        ],
    );
}

#[test]
fn the_list_can_settle_a_conflict_by_naming_the_row_it_reads() {
    let settle = |row: u32| {
        format!(
            "[[experience.settle]]\nship = \"us1\"\nquestion = \"{Q1}\"\nrow = \"ship_facts.csv:{row}\"\n\
             reason = \"the log outranks the summary\"\n"
        )
    };
    let both = format!(
        "{}{}",
        answer("us1", Q1, 101, true, "1942-06-01", "1942-06-01"),
        answer("us1", Q1, 102, false, "1942-06-01", "1942-07-01")
    );
    // Reading the yes row: 2 + 1 = 3, and only that row is cited.
    let text = derives(&["us1"], &format!("{both}{}", settle(101)), &base());
    let r = record(&text, "us1");
    assert_eq!((r[1].as_str(), r[7].as_str()), ("3", "ship_facts.csv:101"));
    assert!(text.contains("# settled: us1 night_practice_recent reads ship_facts.csv:101"));
    // Reading the no row: nothing moves, 2.
    let text = derives(&["us1"], &format!("{both}{}", settle(102)), &base());
    assert_eq!(value_of(&text, "us1"), "2");
}

#[test]
fn a_settlement_with_nothing_to_settle_is_a_problem() {
    let tables = format!(
        "{}[[experience.settle]]\nship = \"us1\"\nquestion = \"{Q1}\"\nrow = \"ship_facts.csv:101\"\nreason = \"x\"\n",
        answer("us1", Q1, 101, true, "1942-06-01", "1942-06-01")
    );
    let problems = fails(&["us1"], &tables, &base());
    assert_one(
        &problems,
        &[
            "us1: experience: night_practice_recent",
            "no conflict to settle",
        ],
    );
}

// What an answer must name.

#[test]
fn an_answer_must_name_a_preferred_row_of_medium_or_high_confidence() {
    // Row 103 is unconfirmed and low: it moves nothing and is a problem to name.
    let problems = fails(
        &["us1"],
        &answer("us1", Q1, 103, true, "1942-06-01", "1942-06-01"),
        &base(),
    );
    assert_one(
        &problems,
        &[
            "us1: not-preferred: night_practice_recent",
            "unconfirmed at low",
        ],
    );
}

#[test]
fn an_answer_must_name_a_row_of_that_ship_and_of_the_right_attribute() {
    // Row 104 is us2's.
    let problems = fails(
        &["us1"],
        &answer("us1", Q1, 104, true, "1942-06-01", "1942-06-01"),
        &base(),
    );
    assert_one(&problems, &["is not a row of this ship"]);
    let files = with_rows(
        "ship_facts.csv",
        &[commissioned(201, "us1", "1930-01-01", "preferred")],
    );
    let problems = fails(
        &["us1"],
        &answer("us1", Q1, 201, true, "1942-06-01", "1942-06-01"),
        &files,
    );
    assert_one(
        &problems,
        &["is a `commissioned` row, not `night_practice_note`"],
    );
    let problems = fails(
        &["us1"],
        &answer("us1", Q1, 999, true, "1942-06-01", "1942-06-01"),
        &base(),
    );
    assert_one(&problems, &["row ship_facts.csv:999 does not exist"]);
}

#[test]
fn a_malformed_answer_is_a_list_problem() {
    let good = answer("us1", Q1, 101, true, "1942-06-01", "1942-06-01");
    for (text, why) in [
        (
            good.replace("us1\"\nquestion", "us9\"\nquestion"),
            "not in the experience ships",
        ),
        (
            good.replace(Q1, "new_ship"),
            "the other two are read from the facts",
        ),
        (
            good.replace("ship_facts.csv:101", "other_facts.csv:1"),
            "is not `ship_facts.csv:<id>` or `training_facts.csv:<id>`",
        ),
        (good.replace("\"yes\"", "\"maybe\""), "is not yes or no"),
        (good.replace("a reason", " "), "`reason` must not be empty"),
        (
            good.replace("to = \"1942-06-01\"", "to = \"1942-13\""),
            "is not a date",
        ),
    ] {
        let problems = fails(&["us1"], &text, &base());
        assert!(
            problems
                .iter()
                .any(|p| p.contains("list: list:") && p.contains(why)),
            "`{why}` in {problems:#?}"
        );
    }
}

#[test]
fn an_unknown_ship_or_engagement_is_a_problem() {
    let problems = fails(&["nobody"], "", &base());
    assert_one(&problems, &["nobody: unknown-ship"]);
    let text = list(&["us1"], "").replace("test-battle", "no-such-battle");
    let problems = derive("list.toml", &text, RULES, &base()).expect_err("problems");
    assert_eq!(problems.len(), 1, "{problems:#?}");
    assert!(problems[0].to_string().contains("no-such-battle"));
}

#[test]
fn a_list_with_an_experience_table_needs_the_rules_to_have_one() {
    let plain = RULES.split("[experience]").next().unwrap();
    let problems = derive_with(&["us1"], "", plain, &base()).expect_err("problems");
    assert_one(
        &problems,
        &["rules: rules: experience", "not in the rules file"],
    );
}

// The rows left unread and the order.

#[test]
fn rows_that_were_not_read_are_listed_with_their_standing_but_not_in_the_file() {
    let tables = answer("us1", Q1, 101, true, "1942-06-01", "1942-06-01");
    let outputs = derive_with(&["us1"], &tables, RULES, &base()).expect("derives");
    // Rows 102 (preferred) and 103 (unconfirmed) were not read.
    assert_eq!(
        outputs.experience_unread,
        ["us1: ship_facts.csv:102(preferred), ship_facts.csv:103(unconfirmed)"]
    );
    let text = outputs.experience_csv.expect("a file");
    assert_eq!(record(&text, "us1").len(), 8);
    assert!(
        !text.contains("102") && !text.contains("not_read,"),
        "{text}"
    );
}

#[test]
fn ships_come_out_in_list_order_and_the_file_says_it_is_generated() {
    let text = derives(&["us3", "jp1", "us1"], "", &base());
    assert!(
        text.starts_with("# GENERATED by oob-derive from the facts in data/oob. Do not edit.\n")
    );
    let order: Vec<String> = csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .from_reader(text.as_bytes())
        .records()
        .map(|r| r.unwrap()[0].to_string())
        .collect();
    assert_eq!(order, ["us3", "jp1", "us1"]);
}

#[test]
fn a_list_with_only_an_experience_table_writes_no_class_files() {
    let outputs = derive_with(&["us1"], "", RULES, &base()).expect("derives");
    assert!(outputs.class_csv.is_empty() && outputs.sources_csv.is_empty());
    assert!(outputs.experience_csv.is_some());
}

// Files: --check writes nothing, a run writes the experience file.

fn scratch(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("oob_derive_exp_{name}"));
    let _ = std::fs::remove_dir_all(&dir);
    for sub in ["derivation", "oob", "reference"] {
        std::fs::create_dir_all(dir.join(sub)).unwrap();
    }
    std::fs::write(dir.join("derivation/list.toml"), list(&["us1"], "")).unwrap();
    std::fs::write(dir.join("derivation/rules.toml"), RULES).unwrap();
    for (file, text) in base() {
        std::fs::write(dir.join("oob").join(file), text).unwrap();
    }
    dir
}

#[test]
fn check_writes_nothing_and_a_run_writes_the_experience_file() {
    let dir = scratch("files");
    let go = |check| run(&dir.join("derivation/list.toml"), &dir.join("oob"), check).expect("run");
    match go(true) {
        Outcome::Clean {
            files,
            written,
            unread,
            ..
        } => {
            assert!(!written);
            // The rows not read are reported by a check, not written.
            assert_eq!(
                unread,
                [
                    "us1: ship_facts.csv:101(preferred), ship_facts.csv:102(preferred), \
                  ship_facts.csv:103(unconfirmed)"
                ]
            );
            assert_eq!(files.len(), 1);
            assert!(files[0].ends_with("reference/exp.ships.csv"));
        }
        other => panic!("{other:?}"),
    }
    assert!(!dir.join("reference/exp.ships.csv").exists());
    match go(false) {
        Outcome::Clean { written, .. } => assert!(written),
        other => panic!("{other:?}"),
    }
    let text = std::fs::read_to_string(dir.join("reference/exp.ships.csv")).unwrap();
    assert_eq!(value_of(&text, "us1"), "2");
}

// The real facts. The architect's hand calculation (Nov 12, 2026).
//
// A value starts at 2 and each question a row answers moves it by 1, once.
//
// # Savo Island, first day 1942-08-08
//
// The three months are 1942-05-08 to 1942-08-08, the twelve months
// 1941-08-08 to 1942-08-08. The statement is sf:1019 to 1021 (Crutchley: night
// training "with Chicago and Destroyer Squadron 4", at Brisbane, May to July
// 1942). The rule for a force's statement, as fixed in pull request 137: "a
// statement about a force or a squadron answers for a ship only if a row names
// her as part of it at that time". "At that time" means a row puts her with
// that force during the period the statement is dated to; a list of the
// squadron on one day does not by itself show that, as the rows' own note says.
//
// - The eight Japanese ships (Chokai, Aoba, Kako, Kinugasa, Furutaka, Tenryu,
//   Yubari, Yunagi): Q4 yes by tf:32 with the second source tf:87; nothing
//   else. 2 + 1 = 3. The Japanese cruiser divisions' war diaries (sf:1086 to
//   1122) change no value: no row says a cruiser fired or exercised at night
//   inside the three months (the drills of July and October say the diary does
//   not say whether by night); the 18th Division's order for the passage to
//   Truk schedules a searchlight drill and a night formation test in June
//   (sf:1117) but an order is not a record that they were done, and sf:1118
//   does not say night, so Tenryu is not answered; and the rows that say the
//   diaries record no night exercise are an absence, of low confidence, and
//   answer no question. The designer's decision: the ships keep the point for
//   their navy's night training (tf:32 and tf:87): "Keep it".
// - Vincennes and Quincy: Q2 yes by sf:1016 and sf:1017 (no night target
//   practice or night battle exercise in at least fifteen months in the
//   Atlantic Fleet, which they left in March 1942 by sf:949 and sf:950, so the
//   period reaches into the twelve months), and no row says either held one
//   after. 2 - 1 = 1.
// - Astoria: sf:1018 says she had "recently" held some night target
//   practices, with no date: Q1 not answered. Q2 not answered (a row says she
//   held some). 2.
// - Chicago, Canberra, Australia: Q1 yes by sf:1021, 1019, 1020 (their own
//   rows, dated 1942-05 to 1942-07). sf:1022 says no ship had fired at night
//   within eight months: that is about firing and does not contradict night
//   training, so it is no conflict. 2 + 1 = 3.
// - Helm: named in the squadron (sf:1064) and joined Task Force 44 at Brisbane
//   on 19 May 1942 (sf:1072). Yes. 2 + 1 = 3.
// - Bagley: named (sf:1068) and arrived at Brisbane on 30 May 1942 (sf:1074).
//   Yes. 3. She fired no gun at Savo Island and Helm fired four shells; both
//   are 3 by the rule, because experience is not readiness (decision 0068).
// - Ralph Talbot: named (sf:1066) and left Brisbane with the force on 14 July
//   1942 (sf:1070), having left Hawaii on 14 June (sf:1078): with the force at
//   Brisbane inside the period, if briefly. Yes. 3.
// - Patterson: named (sf:1065). Her two rows on where she was are disputed
//   between Gill (left Brisbane with the force on 14 July, sf:1069) and the
//   ship dictionary (left Brisbane on 22 June, sf:1075); both put her at
//   Brisbane inside the period. Yes, by the rule for rows that are all
//   disputed and agree. 3.
// - Jarvis: named (sf:1067). Her two rows are disputed and disagree on the
//   answer: Gill has her leaving Brisbane with the force (sf:1071), the
//   dictionary has her at Sydney from 18 June (sf:1076). Not answered. 2.
// - Blue: named (sf:1063) on 7 July, and the dictionary has her on convoys from
//   Pearl Harbor until June and then at Wellington (sf:1077); no row puts her
//   at Brisbane. Not answered. 2.
// - Wilson: no row names her in the squadron. 2.
//
// # Cape Esperance, first day 1942-10-11
//
// The three months are 1942-07-11 to 1942-10-11; a new ship is one
// commissioned after 1941-10-11. The statement is tf:74 (the Navy's combat
// narrative): the force "held target practice 8 Oct and brief antiaircraft
// practice that night", dated 1942-09-20 to 1942-10-08. An antiaircraft
// practice at night is an exercise at night.
//
// - Salt Lake City (sf:1033), San Francisco (sf:1036), Boise (sf:1037): Q1
//   yes, the night firing practice of 7 October 1942. 3. Boise's two
//   commissioning dates are both before the line: Q3 no.
// - Helena: nothing answered (sf:1050 says she joined and does not say she
//   exercised; both her commissioning dates are before the line). 2.
// - Farenholt (sf:1081), Buchanan (sf:1082), Laffey (sf:1083): each named as
//   one of the three destroyers of the force as first organised, with it on 7
//   and 8 October. Q1 yes; Q3 yes (commissioned 1942). 2 - 1 + 1 = 2.
// - Duncan (sf:1084) and McCalla (sf:1085): rows say each was not with the
//   force for the practices of 8 October. Q1 not answered; Q3 yes (Duncan
//   1942-04-16, McCalla 1942-05-27). 2 - 1 = 1.
// - Aoba, Furutaka, Kinugasa, Fubuki, Hatsuyuki: Q4 yes by tf:32 with the
//   second source tf:87. 3.
//
// Seven values changed from the earlier derivation, each by new rows: Helm,
// Bagley, Patterson and Ralph Talbot 2 to 3; Farenholt, Buchanan and Laffey
// 1 to 2.

const SAVO: [(&str, &str, &str); 21] = [
    ("chokai", "3", "fixed"),
    ("aoba", "3", "fixed"),
    ("kako", "3", "fixed"),
    ("kinugasa", "3", "fixed"),
    ("furutaka", "3", "fixed"),
    ("tenryu", "3", "fixed"),
    ("yubari", "3", "fixed"),
    ("yunagi", "3", "fixed"),
    ("hmas-canberra", "3", "fixed"),
    ("hmas-australia", "3", "fixed"),
    ("ca-29-chicago", "3", "fixed"),
    ("dd-386-bagley", "3", "fixed"),
    ("dd-392-patterson", "3", "fixed"),
    ("ca-44-vincennes", "1", "fixed"),
    ("ca-39-quincy", "1", "fixed"),
    ("ca-34-astoria", "2", "fixed"),
    ("dd-388-helm", "3", "fixed"),
    ("dd-408-wilson", "2", "fixed"),
    ("dd-387-blue", "2", "fixed"),
    ("dd-390-ralph-talbot", "3", "fixed"),
    ("dd-393-jarvis", "2", "fixed"),
];

const CAPE_ESPERANCE: [(&str, &str, &str); 14] = [
    ("ca-38-san-francisco", "3", "fixed"),
    ("cl-47-boise", "3", "fixed"),
    ("ca-25-salt-lake-city", "3", "fixed"),
    ("cl-50-helena", "2", "fixed"),
    ("dd-491-farenholt", "2", "fixed"),
    ("dd-485-duncan", "1", "fixed"),
    ("dd-459-laffey", "2", "fixed"),
    ("dd-484-buchanan", "2", "fixed"),
    ("dd-488-mccalla", "1", "fixed"),
    ("aoba", "3", "fixed"),
    ("furutaka", "3", "fixed"),
    ("kinugasa", "3", "fixed"),
    ("fubuki", "3", "fixed"),
    ("hatsuyuki", "3", "fixed"),
];

fn real(list_file: &str) -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let read = |path: &str| std::fs::read_to_string(root.join(path)).expect(path);
    let (facts, unread) = oob_check::read_dir(&root.join("data/oob"));
    assert!(unread.is_empty());
    derive(
        list_file,
        &read(&format!("data/derivation/{list_file}")),
        &read("data/derivation/rules.toml"),
        &facts,
    )
    .unwrap_or_else(|p| panic!("{p:#?}"))
    .experience_csv
    .expect("an experience file")
}

#[track_caller]
fn assert_values(text: &str, expected: &[(&str, &str, &str)]) {
    for (ship, value, fixed) in expected {
        let r = record(text, ship);
        assert_eq!((r[1].as_str(), r[2].as_str()), (*value, *fixed), "{ship}");
    }
    let count = csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .from_reader(text.as_bytes())
        .records()
        .count();
    assert_eq!(count, expected.len());
}

#[test]
fn the_savo_island_values_are_the_hand_calculation_and_the_committed_file() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let text = real("savo_island.toml");
    assert_values(&text, &SAVO);
    assert_eq!(
        text,
        std::fs::read_to_string(root.join("data/reference/ship_experience_savo_island.ships.csv"))
            .unwrap()
    );
}

#[test]
fn the_cape_esperance_values_are_the_hand_calculation_and_the_committed_file() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let text = real("cape_esperance.toml");
    assert_values(&text, &CAPE_ESPERANCE);
    assert_eq!(
        text,
        std::fs::read_to_string(
            root.join("data/reference/ship_experience_cape_esperance.ships.csv")
        )
        .unwrap()
    );
}

#[test]
fn the_japanese_values_are_fixed_by_the_second_source_and_the_file_says_what_it_is() {
    for file in ["savo_island.toml", "cape_esperance.toml"] {
        let text = real(file);
        assert!(!text.contains("PROVISIONAL"), "{file}");
        assert!(
            text.contains(
                "# navy_night_first for `jp`: rows training_facts.csv:32; second source: \
                 training_facts.csv:87."
            ),
            "{file}"
        );
    }
}

// Force statements, dates read from the rows, and the rows not read.
//
// # The synthetic cases, worked by hand
//
// The window is the same: the three months of `test-battle` run from
// 1942-05-08 to 1942-08-08. The statement row 301 (a night practice row of
// `us1`, preferred, high) is dated 1942-05 to 1942-07, that is 1 May to 31
// July: some day of it is inside the window, so it is dated inside it.
//
// A row puts a ship with the force inside the statement's period when its
// own period meets 1 May to 31 July: it begins on or before 31 July and it
// has not ended before 1 May; a row with a start and no end goes on, and a
// row with neither says nothing of when.
//
// | Case | Working | Value |
// | --- | --- | --- |
// | us2, named by 311 (from 1942-06-01, no end, preferred, high) | begins 1 June, before 31 July, still going: with the force. 2 + 1 | 3 |
// | us2, named by 312 (from 1942-08-01) | begins after 31 July: a problem | none |
// | us2, named by 313 (1942-01-01 to 1942-04-30) | ended before 1 May: a problem | none |
// | us3, named by 321 (preferred, 7 July), 322 and 323 (disputed, 14 July and 22 June) | the disputed rows are both dated inside the period and both are named: 2 + 1 | 3 |
// | us3, named by 321 and 322 | 323 is disputed, bears on the period and is not named: a problem | none |
// | us3, named by 321, 322, 323 and 324 (disputed, September) | 324 is dated after the period: a problem | none |
// | us2 named by an unconfirmed row, a low row, another ship's row, a row of another attribute, a row that does not exist | each is a problem | none |
// | the statement dated 1942-03 | outside the window: a problem | none |
// | the statement unconfirmed and low | not preferred: a problem | none |
// | the statement a `commissioned` row | not an allowed attribute: a problem | none |
// | us2 named by 311 under a training_facts statement dated 1942-06 | 2 + 1 | 3 |
// | a row dated 1942-06-10 and an answer with no dates | dates read from the row: 2 + 1 | 3 |
// | the answer gives the row's dates | agree: 2 + 1 | 3 |
// | the answer gives other dates | disagree: a problem | none |
// | a row with no dates and an answer with none | a problem | none |
// | a new night practice row for us1 that no answer cites | not read: the file is as before | same |

/// Base plus rows for each file.
fn plus(mut files: Files, file: &str, rows: &[String]) -> Files {
    let text = files.get_mut(file).expect("a file of the base set");
    for row in rows {
        text.push_str(row);
        text.push('\n');
    }
    files
}

fn dated_fact(
    id: u32,
    entity: &str,
    attribute: &str,
    from: &str,
    to: &str,
    confidence: &str,
    standing: &str,
) -> String {
    // Each row's value is its own, so that no row "supports" another.
    let value = if attribute == "commissioned" {
        "1942-06-01".to_string()
    } else {
        format!("a statement {id}")
    };
    format!(
        "{id},{entity},{attribute},{value},{from},{to},a,p.1,2026-11-10,{confidence},stated,{standing},a note"
    )
}

fn dated_training(
    id: u32,
    entity: &str,
    attribute: &str,
    from: &str,
    to: &str,
    confidence: &str,
    standing: &str,
) -> String {
    format!(
        "{id},{entity},{attribute},a statement {id},as stated,{from},{to},a,p.1,2026-11-12,{confidence},stated,{standing},a note"
    )
}

const RULES_FORCE: &str = "constants_are_placeholders = true\n\
    gun_shell_kg_per_damage = 6\n\
    torpedo_damage_per_warhead_kg = \"0.4\"\n\
    displacement_lt_per_toughness = 25\n\
    [experience]\n\
    recent_months = 3\n\
    lookback_months = 12\n\
    new_ship_months = 12\n\
    [[experience.navy]]\n\
    nation = \"jp\"\n\
    entity = \"ijn-cruiser-destroyer-crews\"\n\
    rows = [1]\n\
    kinds = [\"heavy_cruiser\"]\n\
    [experience.force]\n\
    statement = [\"ship_facts.csv:night_practice_note\", \"training_facts.csv:night_training_note\"]\n\
    named_by = [\"ship_facts.csv:time_in_theatre_note\", \"ship_facts.csv:night_practice_note\"]\n";

/// The base set with a statement row 301 (us1, 1942-05 to 1942-07) and the
/// ships' naming rows: 311 for us2 (from 1942-06-01, no end), and for us3 321
/// (preferred, 1942-07-07), 322 (disputed, 1942-07-14) and 323 (disputed,
/// 1942-06-22).
fn force_base() -> Files {
    let mut files = plus(
        base(),
        "ship_facts.csv",
        &[
            dated_fact(
                301,
                "us1",
                "night_practice_note",
                "1942-05",
                "1942-07",
                "high",
                "preferred",
            ),
            dated_fact(
                311,
                "us2",
                "time_in_theatre_note",
                "1942-06-01",
                "",
                "high",
                "preferred",
            ),
            dated_fact(
                321,
                "us3",
                "time_in_theatre_note",
                "1942-07-07",
                "1942-07-07",
                "high",
                "preferred",
            ),
            dated_fact(
                322,
                "us3",
                "time_in_theatre_note",
                "1942-07-14",
                "1942-07-14",
                "high",
                "disputed",
            ),
            dated_fact(
                323,
                "us3",
                "time_in_theatre_note",
                "1942-06-22",
                "1942-06-22",
                "high",
                "disputed",
            ),
        ],
    );
    // The test's attribute list does not have the new attribute yet.
    let attributes = files.get_mut("attributes.csv").expect("attributes");
    attributes.push_str("time_in_theatre_note,ship,text,many\n");
    files
}

/// A question 1 answer for `ship` by `row`, named by `named_by`, with no dates.
fn force_answer(ship: &str, row: &str, named_by: &[u32]) -> String {
    let named: Vec<String> = named_by
        .iter()
        .map(|id| format!("\"ship_facts.csv:{id}\""))
        .collect();
    format!(
        "[[experience.answer]]\nship = \"{ship}\"\nquestion = \"{Q1}\"\nrow = \"{row}\"\n\
         named_by = [{}]\nanswer = \"yes\"\nreason = \"a reason\"\n",
        named.join(", ")
    )
}

#[track_caller]
fn force_derives(ships: &[&str], tables: &str, files: &Files) -> String {
    derive_with(ships, tables, RULES_FORCE, files)
        .unwrap_or_else(|p| panic!("{p:#?}"))
        .experience_csv
        .expect("an experience file")
}

#[track_caller]
fn force_fails(ships: &[&str], tables: &str, files: &Files) -> Vec<String> {
    derive_with(ships, tables, RULES_FORCE, files).expect_err("problems expected")
}

#[track_caller]
fn assert_problem(problems: &[String], expected: &[&str]) {
    assert!(
        problems
            .iter()
            .any(|p| expected.iter().all(|want| p.contains(want))),
        "{expected:?} in {problems:#?}"
    );
}

#[test]
fn a_force_statement_answers_for_a_ship_a_preferred_row_puts_with_the_force() {
    // 2 + 1: the statement 301 is dated inside the window, and 311 puts us2
    // with the force from 1 June, with no end.
    let tables = force_answer("us2", "ship_facts.csv:301", &[311]);
    let text = force_derives(&["us2", "us3"], &tables, &force_base());
    let us2 = record(&text, "us2");
    assert_eq!(us2[1..7], ["3", "fixed", "up", "none", "none", "none"]);
    assert_eq!(us2[7], "ship_facts.csv:301|ship_facts.csv:311");
    // us3 has no answer: nothing moves, however many rows name her.
    assert_eq!(value_of(&text, "us3"), "2");
    assert!(
        text.contains(
            "# answer: us2 night_practice_recent yes by ship_facts.csv:301 (she is named in the \
             force by ship_facts.csv:311), dated 1942-05..1942-07: a reason"
        ),
        "{text}"
    );
}

#[test]
fn a_naming_row_dated_outside_the_statements_period_is_a_problem() {
    // 312 begins 1 August, after 31 July. 313 ended 30 April, before 1 May.
    let files = plus(
        force_base(),
        "ship_facts.csv",
        &[
            dated_fact(
                312,
                "us2",
                "time_in_theatre_note",
                "1942-08-01",
                "",
                "high",
                "preferred",
            ),
            dated_fact(
                313,
                "us2",
                "time_in_theatre_note",
                "1942-01-01",
                "1942-04-30",
                "high",
                "preferred",
            ),
        ],
    );
    for row in [312, 313] {
        let problems = force_fails(
            &["us2"],
            &force_answer("us2", "ship_facts.csv:301", &[row]),
            &files,
        );
        assert_one(
            &problems,
            &[
                "us2: not-on-date",
                &format!("named_by row ship_facts.csv:{row}"),
                "does not put her with the force inside the statement's period",
            ],
        );
    }
}

#[test]
fn disputed_naming_rows_that_all_put_her_with_the_force_answer_if_all_are_named() {
    // us3: 321 preferred (7 July), 322 and 323 disputed (14 July, 22 June),
    // both inside 1 May to 31 July, and all three named: 2 + 1.
    let tables = force_answer("us3", "ship_facts.csv:301", &[321, 322, 323]);
    let text = force_derives(&["us3"], &tables, &force_base());
    let r = record(&text, "us3");
    assert_eq!(r[1], "3");
    assert_eq!(
        r[7],
        "ship_facts.csv:301|ship_facts.csv:321|ship_facts.csv:322|ship_facts.csv:323"
    );
}

#[test]
fn a_disputed_row_named_without_every_disputed_row_that_bears_on_it_is_a_problem() {
    // 323 is disputed, dated inside the period and not named.
    let problems = force_fails(
        &["us3"],
        &force_answer("us3", "ship_facts.csv:301", &[321, 322]),
        &force_base(),
    );
    assert_one(
        &problems,
        &[
            "us3: conflict",
            "every disputed row",
            "missing ship_facts.csv:323",
        ],
    );
}

#[test]
fn disputed_naming_rows_that_disagree_are_a_problem() {
    // 324 is disputed and dated September, after the period: the rows do not
    // all put her with the force inside it. 323 and 322 are named, so the
    // only problem is 324.
    let files = plus(
        force_base(),
        "ship_facts.csv",
        &[dated_fact(
            324,
            "us3",
            "time_in_theatre_note",
            "1942-09-01",
            "1942-09-30",
            "high",
            "disputed",
        )],
    );
    let problems = force_fails(
        &["us3"],
        &force_answer("us3", "ship_facts.csv:301", &[321, 322, 323, 324]),
        &files,
    );
    assert_one(
        &problems,
        &["us3: not-on-date", "named_by row ship_facts.csv:324"],
    );
}

#[test]
fn a_naming_row_must_be_firm_hers_and_of_an_allowed_attribute() {
    let files = plus(
        force_base(),
        "ship_facts.csv",
        &[
            dated_fact(
                331,
                "us2",
                "time_in_theatre_note",
                "1942-06-01",
                "",
                "low",
                "unconfirmed",
            ),
            dated_fact(
                332,
                "us2",
                "time_in_theatre_note",
                "1942-06-01",
                "",
                "low",
                "disputed",
            ),
            dated_fact(
                333,
                "us2",
                "commissioned",
                "1942-06-01",
                "",
                "high",
                "preferred",
            ),
        ],
    );
    let tables = |row: u32| force_answer("us2", "ship_facts.csv:301", &[row]);
    // Not preferred, and not disputed.
    assert_one(
        &force_fails(&["us2"], &tables(331), &files),
        &["us2: not-preferred", "named_by row ship_facts.csv:331"],
    );
    // Low confidence.
    assert_one(
        &force_fails(&["us2"], &tables(332), &files),
        &["us2: not-preferred", "named_by row ship_facts.csv:332"],
    );
    // An attribute the rules file does not allow.
    assert_one(
        &force_fails(&["us2"], &tables(333), &files),
        &["named_by row ship_facts.csv:333", "does not allow"],
    );
    // Another ship's row (321 is us3's), and a row that does not exist.
    assert_one(
        &force_fails(&["us2"], &tables(321), &files),
        &["named_by row ship_facts.csv:321 is not a row of this ship"],
    );
    assert_one(
        &force_fails(&["us2"], &tables(999), &files),
        &["named_by row ship_facts.csv:999 is not a row of this ship"],
    );
}

#[test]
fn a_ship_no_row_names_is_not_answered_and_a_statement_of_another_ship_needs_named_by() {
    // No answer for us4: the statement names her in no row. 2.
    let text = force_derives(&["us4"], "", &force_base());
    assert_eq!(value_of(&text, "us4"), "2");
    // An answer by 301 (us1's row) for us2 without `named_by` is a problem.
    let tables = answer("us2", Q1, 301, true, "", "");
    let problems = force_fails(&["us2"], &tables, &force_base());
    assert_one(
        &problems,
        &["row ship_facts.csv:301 is not a row of this ship"],
    );
}

#[test]
fn the_statement_must_be_firm_dated_inside_the_window_and_of_an_allowed_attribute() {
    let files = plus(
        force_base(),
        "ship_facts.csv",
        &[
            // Dated 1942-03: before the window.
            dated_fact(
                341,
                "us1",
                "night_practice_note",
                "1942-03",
                "1942-03",
                "high",
                "preferred",
            ),
            // Unconfirmed and low.
            dated_fact(
                342,
                "us1",
                "night_practice_note",
                "1942-05",
                "1942-07",
                "low",
                "unconfirmed",
            ),
            // The wrong attribute.
            dated_fact(
                343,
                "us1",
                "commissioned",
                "1942-05",
                "1942-07",
                "high",
                "preferred",
            ),
        ],
    );
    let tables = |row: u32| force_answer("us2", &format!("ship_facts.csv:{row}"), &[311]);
    assert_one(
        &force_fails(&["us2"], &tables(341), &files),
        &["does not meet the window"],
    );
    assert_one(
        &force_fails(&["us2"], &tables(342), &files),
        &[
            "not-preferred",
            "only a preferred row of medium or high confidence",
        ],
    );
    assert_one(
        &force_fails(&["us2"], &tables(343), &files),
        &["is a `commissioned` row", "does not allow as the statement"],
    );
    // `named_by` belongs to question 1 only.
    let q2 = tables(301).replace("night_practice_recent", "no_night_practice");
    let problems = force_fails(&["us2"], &q2, &files);
    assert_problem(
        &problems,
        &[
            "list: list:",
            "`named_by` is for night_practice_recent only",
        ],
    );
}

#[test]
fn a_training_facts_row_may_be_the_statement() {
    // 5 is a night training row of a navy, dated June 1942, preferred, high:
    // inside the window. 311 puts us2 with the force from 1 June: 2 + 1.
    let files = plus(
        force_base(),
        "training_facts.csv",
        &[dated_training(
            5,
            "usn-cruiser-destroyer-crews",
            "night_training_note",
            "1942-06-01",
            "1942-06-30",
            "high",
            "preferred",
        )],
    );
    let tables = force_answer("us2", "training_facts.csv:5", &[311]);
    let text = force_derives(&["us2"], &tables, &files);
    let r = record(&text, "us2");
    assert_eq!(r[1], "3");
    assert_eq!(r[7], "ship_facts.csv:311|training_facts.csv:5");
    // Without `named_by` a training row answers for no ship.
    let problems = force_fails(
        &["us2"],
        &answer("us2", Q1, 5, true, "", "").replace("ship_facts.csv:5", "training_facts.csv:5"),
        &files,
    );
    assert_one(
        &problems,
        &[
            "training_facts.csv:5 is not a row of this ship",
            "only with `named_by`",
        ],
    );
}

#[test]
fn named_by_needs_the_rules_files_force_table() {
    let problems = derive_with(
        &["us2"],
        &force_answer("us2", "ship_facts.csv:301", &[311]),
        RULES,
        &force_base(),
    )
    .expect_err("problems");
    assert_one(&problems, &["us2: rules", "`[experience.force]`"]);
}

#[test]
fn a_malformed_force_table_is_a_rules_problem() {
    let rules = RULES_FORCE.replace(
        "\"ship_facts.csv:time_in_theatre_note\", ",
        "\"time_in_theatre_note\", ",
    );
    let problems = derive_with(&["us2"], "", &rules, &force_base()).expect_err("problems");
    assert_problem(
        &problems,
        &["rules: rules", "is not `ship_facts.csv:<attribute>`"],
    );
}

// Dates.

#[test]
fn the_dates_of_an_answer_are_read_from_the_row() {
    // Row 351 (us1) is dated 1942-06-10, a point inside the window; the
    // answer gives no dates: 2 + 1 = 3.
    let files = plus(
        force_base(),
        "ship_facts.csv",
        &[dated_fact(
            351,
            "us1",
            "night_practice_note",
            "1942-06-10",
            "1942-06-10",
            "medium",
            "preferred",
        )],
    );
    let tables = "[[experience.answer]]\nship = \"us1\"\nquestion = \"night_practice_recent\"\n\
                  row = \"ship_facts.csv:351\"\nanswer = \"yes\"\nreason = \"a reason\"\n";
    let text = force_derives(&["us1"], tables, &files);
    assert_eq!(value_of(&text, "us1"), "3");
    assert!(
        text.contains("by ship_facts.csv:351, dated 1942-06-10..1942-06-10:"),
        "{text}"
    );
    // Dates that agree with the row's are fine.
    let agreeing = "[[experience.answer]]\nship = \"us1\"\nquestion = \"night_practice_recent\"\n\
                    row = \"ship_facts.csv:351\"\nanswer = \"yes\"\nfrom = \"1942-06-10\"\n\
                    to = \"1942-06-10\"\nreason = \"a reason\"\n";
    let text = force_derives(&["us1"], agreeing, &files);
    assert_eq!(value_of(&text, "us1"), "3");
}

#[test]
fn dates_that_disagree_with_the_row_are_a_problem() {
    let files = plus(
        force_base(),
        "ship_facts.csv",
        &[dated_fact(
            351,
            "us1",
            "night_practice_note",
            "1942-06-10",
            "1942-06-10",
            "medium",
            "preferred",
        )],
    );
    let problems = force_fails(
        &["us1"],
        &answer("us1", Q1, 351, true, "1942-06-11", "1942-06-10"),
        &files,
    );
    assert_one(
        &problems,
        &[
            "us1: experience",
            "`valid_from`",
            "disagrees with row ship_facts.csv:351",
        ],
    );
    let problems = force_fails(
        &["us1"],
        &answer("us1", Q1, 351, true, "1942-06-10", "1942-06-12"),
        &files,
    );
    assert_one(&problems, &["`valid_to`", "disagrees with row"]);
}

#[test]
fn a_row_without_dates_needs_the_answer_to_give_them() {
    // Row 101 (base) has no dates. Question 1 needs both.
    let no_dates = "[[experience.answer]]\nship = \"us1\"\nquestion = \"night_practice_recent\"\n\
                    row = \"ship_facts.csv:101\"\nanswer = \"yes\"\nreason = \"a reason\"\n";
    let problems = force_fails(&["us1"], no_dates, &base());
    assert_problem(
        &problems,
        &["has no `valid_from` and the answer gives none"],
    );
    assert_problem(&problems, &["has no `valid_to` and the answer gives none"]);
    // Question 2 may leave `from` open, but needs `to`.
    let q2 = no_dates.replace("night_practice_recent", "no_night_practice");
    let problems = force_fails(&["us1"], &q2, &base());
    assert_one(&problems, &["has no `valid_to` and the answer gives none"]);
    // Given, the answer stands, as before.
    let text = derives(
        &["us1"],
        &answer("us1", Q1, 101, true, "1942-06-01", "1942-06-01"),
        &base(),
    );
    assert_eq!(value_of(&text, "us1"), "3");
}

// The rows not read are not in the file.

#[test]
fn a_new_row_for_a_listed_ship_leaves_the_generated_text_unchanged() {
    let tables = answer("us1", Q1, 101, true, "1942-06-01", "1942-06-01");
    let before = derive_with(&["us1"], &tables, RULES, &base()).expect("derives");
    // A new night practice row for us1 that no answer cites: not read.
    let files = plus(
        base(),
        "ship_facts.csv",
        &[ship_fact(
            401,
            "us1",
            "night_practice_note",
            "a new statement",
            "high",
            "preferred",
        )],
    );
    let after = derive_with(&["us1"], &tables, RULES, &files).expect("derives");
    assert_eq!(before.experience_csv, after.experience_csv);
    assert!(!after.experience_csv.as_ref().unwrap().contains("401"));
    // It is in the list that is printed instead.
    assert_eq!(
        before.experience_unread,
        ["us1: ship_facts.csv:102(preferred), ship_facts.csv:103(unconfirmed)"]
    );
    assert_eq!(
        after.experience_unread,
        [
            "us1: ship_facts.csv:102(preferred), ship_facts.csv:103(unconfirmed), \
             ship_facts.csv:401(preferred)"
        ]
    );
}

#[test]
fn a_ship_with_nothing_unread_has_no_line_in_the_list() {
    let text = derive_with(&["us2"], "", RULES, &base()).expect("derives");
    // us2 has row 104, unread.
    assert_eq!(
        text.experience_unread,
        ["us2: ship_facts.csv:104(preferred)"]
    );
    let text = derive_with(&["jpcv"], "", RULES, &base()).expect("derives");
    assert!(text.experience_unread.is_empty());
}
