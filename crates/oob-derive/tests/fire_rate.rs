//! Decision 0070 against the real data: the achieved rates of fire, derived
//! from the repo's `data/oob` and `data/derivation/rules.toml`, equal the
//! architect's hand derivation exactly.
//!
//! # The cases, worked by hand (decision 0070, "The cases")
//!
//! A case rate is shells / (guns x minutes), truncated at the sixth place.
//!
//! | Kind | Ship, battle | Shells | Span | Guns | Rate | Rows |
//! | --- | --- | --- | --- | --- | --- | --- |
//! | salvo | Chokai, Savo Island | 302 | 20 min | 10 | 302 / 200 = 1.51 | outcomes 3131, 3132; fits 71 |
//! | salvo | Kako, Savo Island | 192 | 30 min | 6 | 192 / 180 = 1.066666 | outcomes 3138, 3140; fits 301 |
//! | salvo | Salt Lake City, Cape Esperance | 296 | 29 min | 10 | 296 / 290 = 1.020689 | outcomes 1704, 1638; fits 669, 670 |
//! | salvo | Washington, 15 November | 75 | 7 min (0100 to 0107) | 9 | 75 / 63 = 1.190476 | outcomes 2645; events 860, 863; fits 251 |
//! | destroyer | Asagumo, 13 November | 88 | 6 min | 6 | 88 / 36 = 2.444444 | outcomes 3146, 3147; fits 147 |
//! | continuous | Helena, Cape Esperance | 580 | 26 min (2346 to 0012) | 15 | 580 / 390 = 1.487179 | outcomes 2980; events 1031, 1034; fits 704 to 706 |
//!
//! Medians:
//!
//! - salvo: the middle two of 1.020689, 1.066666, 1.190476, 1.51 are
//!   1.066666 and 1.190476, so (1.066666 + 1.190476) / 2 = 2.257142 / 2 =
//!   1.128571;
//! - destroyer: 2.444444 (one case);
//! - continuous: 1.487179 (one case).
//!
//! # What the rule gives the classes at Savo Island ("The hand derivation")
//!
//! Every class's sourced rate is above its kind's achieved rate, so each
//! fires at the achieved rate: every cruiser class (Aoba, Furutaka, Takao,
//! Tenryu, Yubari, County, New Orleans, both rows, Northampton) at 1.128571,
//! every destroyer class (Kamikaze, Bagley, both rows, Benham) at 2.444444.
//! Tenryu has no sourced rate (row 432 is not read for her). A round, the
//! rate x guns x 5, for Tenryu: 1.128571 x 4 x 5 = 22.57142.
//! No other column of the class file moves.

use std::collections::BTreeMap;
use std::path::PathBuf;

use oob_derive::{derive, Outputs};

fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn derived(list: &str) -> Outputs {
    let root = repo();
    let (facts, unread) = oob_check::read_dir(&root.join("data/oob"));
    assert!(unread.is_empty(), "{unread:?}");
    let facts: BTreeMap<String, String> = facts.into_iter().collect();
    let list_text =
        std::fs::read_to_string(root.join("data/derivation").join(list)).expect("the list");
    let rules =
        std::fs::read_to_string(root.join("data/derivation/rules.toml")).expect("the rules");
    derive(list, &list_text, &rules, &facts).unwrap_or_else(|problems| {
        panic!(
            "{:#?}",
            problems.iter().map(ToString::to_string).collect::<Vec<_>>()
        )
    })
}

fn header_has(outputs: &Outputs, want: &str) {
    assert!(
        outputs
            .class_csv
            .lines()
            .any(|l| l.starts_with('#') && l.contains(want)),
        "`{want}` not in the header:\n{}",
        outputs
            .class_csv
            .lines()
            .filter(|l| l.starts_with('#'))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn the_case_rates_and_medians_are_the_hand_derivation() {
    let outputs = derived("savo_island.toml");
    for want in [
        "chokai (savo-island), salvo: 302 / (10 x 20) = 1.51;",
        "kako (savo-island), salvo: 192 / (6 x 30) = 1.066666;",
        "ca-25-salt-lake-city (cape-esperance), salvo: 296 / (10 x 29) = 1.020689;",
        "washington (guadalcanal-15-nov), salvo: 75 / (9 x 7) = 1.190476;",
        "asagumo (guadalcanal-13-nov), destroyer: 88 / (6 x 6) = 2.444444;",
        "cl-50-helena (cape-esperance), continuous: 580 / (15 x 26) = 1.487179;",
        "salvo: median 1.128571 of 4 cases",
        "destroyer: median 2.444444 of 1 case",
        "continuous: median 1.487179 of 1 case",
    ] {
        header_has(&outputs, want);
    }
}

#[test]
fn every_class_at_savo_island_fires_at_its_kinds_achieved_rate() {
    let outputs = derived("savo_island.toml");
    let mut rows = csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .from_reader(outputs.class_csv.as_bytes());
    let header = rows.headers().expect("header").clone();
    let column = |name: &str| header.iter().position(|h| h == name).expect(name);
    let (name_col, kind_col, rate_col) =
        (column("name"), column("kind"), column("gun_rounds_per_min"));
    let mut seen = 0;
    for record in rows.records() {
        let record = record.expect("a record");
        let want = match &record[kind_col] {
            "cruiser" => "1.128571",
            "destroyer" => "2.444444",
            other => panic!("kind {other}"),
        };
        assert_eq!(&record[rate_col], want, "{}", &record[name_col]);
        seen += 1;
    }
    // Aoba, Furutaka, Kamikaze, Takao, Tenryu, Yubari, County, Bagley x 2,
    // Benham, New Orleans x 2, Northampton.
    assert_eq!(seen, 13);
}

#[test]
fn the_class_file_differs_from_the_committed_one_only_in_the_rate_and_header() {
    // The committed file is the generated one: the tool is idempotent.
    let outputs = derived("savo_island.toml");
    let committed =
        std::fs::read_to_string(repo().join("data/reference/ship_classes_savo_island.csv"))
            .expect("the committed class file");
    assert_eq!(outputs.class_csv, committed);
}

#[test]
fn the_lists_without_modelled_surface_weapons_have_no_such_section() {
    let outputs = derived("eastern_solomons.toml");
    assert!(!outputs.class_csv.contains("Achieved rates"));
}
