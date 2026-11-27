//! The table's 39 days under decision 0224's rulings of Nov 23, 2026: the
//! three files in `data/scenarios/table-rear-bases/`, made by
//! `scripts/table-rear-bases/make_files.py` from the 39-day files of
//! `data/scenarios/table-boats/` (the games the table plays).
//!
//! The architect's statements for the files (decision 0224), each tested
//! where a test can read it:
//!
//! 1. The script's check passes for each of the three files.
//! 2. Read as data, with every comment set aside, each file differs from the
//!    file it is made from in this and nothing else: its `name`; base 3's
//!    `supply_t`, 2120 for 960; for each of task forces 30 and 32 to 39,
//!    `at_base = 2`, `home_port = 2` and `available_from` one less; and the
//!    data lines of the islands statement 6 adds: in `07b` and `07c` the
//!    four of the third island, after the block of island 2 and before the
//!    next data line of the file it is made from; in `07_full_slice` twelve,
//!    the last data lines of the file. Its comments differ in the header, in
//!    the notes rulings 1 and 2 ask for, beside base 3, base 7 and each run
//!    moved, and in the note above the islands added. (Restated by the
//!    architect on Nov 27, 2026, for decision 0232, step 3: the files had
//!    the same count of data lines before; with the added lines set aside
//!    they still do, and every other difference is counted as before, 1, 1,
//!    9, 9 and 9.)
//! 3. Base 7 has no supply and no fuel on the first day.
//! 4. On the first morning task force 30 may be used (day 9), it lies
//!    stopped at Rabaul's hex with 7 t of supply aboard, as the file it is
//!    made from had it at Shortland.
//! 5. No file under `data/scenarios/` outside the new folder, and no recorded
//!    game, is changed by the pull request. (Decision 0228 turns on the key
//!    that the side's staff chooses its sources in the table's files, so
//!    the pull request that builds it changes the table's files by that key
//!    and by nothing else; the files of the new folder are made from them.)
//!
//! Added by the architect on Nov 27, 2026, before the code, for decision
//! 0232, step 3 (the designer's answer 2 of Nov 26, 2026: the island
//! "Guadalcanal and Tulagi" is added to the table's copies of the 39 days,
//! and to no file of record or of a proof):
//!
//! 6. Each of the three files has three islands when loaded: 1,
//!    "Guadalcanal", of base 1; 2, "Tulagi", of base 4; and 3, "Guadalcanal
//!    and Tulagi", of base 1 and then base 4. In `07b` and `07c` the script
//!    writes the third after the block of island 2: `[[island]]`, `number =
//!    3`, `name = "Guadalcanal and Tulagi"`, `places = [{ base = 1 }, { base
//!    = 4 }]`, those four data lines in that order and no other.
//!    `07_full_slice`, which is made from a file with no island (the table's
//!    list offered its islands all the same, and the engine refused them:
//!    found on Nov 27, 2026), is given all three at its end: the four lines
//!    of island 1 and of island 2 as `07b` has them, then the four of island
//!    3. The files they are made from, in `table-boats`, are as they were:
//!    `07b` and `07c` with islands 1 and 2 and no third, `07_full_slice`
//!    with none. No scenario file outside `table-rear-bases` holds the line
//!    `name = "Guadalcanal and Tulagi"`.
//! 7. An island is map data and nothing of it is hashed: for each of the
//!    three files, the world on the first morning and at the end of day 3
//!    has the same hash as the world of the same file with the lines
//!    statement 6 adds taken out of its text, both loaded on the live figures as
//!    statement 3 loads the file. No number is worked here: the two hashes
//!    are compared with each other.

use crate::common;

use std::path::PathBuf;
use std::process::Command;

use sim_core::ids::{BaseId, ShipId, TaskForceId};

const FILES: [&str; 3] = [
    "07_full_slice_submarines.toml",
    "07b_slice_submarines.toml",
    "07c_slice_submarines.toml",
];

fn repo() -> PathBuf {
    common::data_dir()
        .parent()
        .expect("the repository")
        .to_path_buf()
}

fn read(path: PathBuf) -> String {
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn data_lines(text: &str) -> Vec<String> {
    text.lines()
        .filter(|l| !l.trim().is_empty() && !l.trim_start().starts_with('#'))
        .map(str::to_string)
        .collect()
}

/// The data lines of the islands statement 6 adds: `07b` and `07c` the four
/// of island 3, after the block of island 2; `07_full_slice` the twelve of
/// islands 1, 2 and 3, the last data lines of the file. Asserts they are
/// those lines, in that order, and returns the lines without them.
fn without_added_islands(name: &str, mut lines: Vec<String>) -> Vec<String> {
    let one = [
        "[[island]]",
        "number = 1",
        "name = \"Guadalcanal\"",
        "places = [{ base = 1 }]",
    ];
    let two = [
        "[[island]]",
        "number = 2",
        "name = \"Tulagi\"",
        "places = [{ base = 4 }]",
    ];
    let three = [
        "[[island]]",
        "number = 3",
        "name = \"Guadalcanal and Tulagi\"",
        "places = [{ base = 1 }, { base = 4 }]",
    ];
    let expected: Vec<&str> = if name.starts_with("07_") {
        one.iter().chain(&two).chain(&three).copied().collect()
    } else {
        three.to_vec()
    };
    let at = if name.starts_with("07_") {
        lines.len() - expected.len()
    } else {
        lines
            .iter()
            .position(|l| l == "number = 3")
            .unwrap_or_else(|| panic!("{name}: no island 3"))
            - 1
    };
    let taken: Vec<String> = lines.drain(at..at + expected.len()).collect();
    assert_eq!(taken, expected, "{name}: the lines the islands add");
    if !name.starts_with("07_") {
        // after island 2's block: the data line before is its places line
        assert_eq!(lines[at - 1], two[3], "{name}: after island 2's block");
    }
    lines
}

#[test]
fn statement_1_the_scripts_check_passes() {
    let out = Command::new("python3")
        .arg(repo().join("scripts/table-rear-bases/make_files.py"))
        .arg("--check")
        .current_dir(repo())
        .output()
        .expect("python3 runs");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stdout)
    );
}

#[test]
fn statement_2_each_file_differs_in_what_the_rulings_say_and_nothing_else() {
    for name in FILES {
        let from = data_lines(&read(repo().join("data/scenarios/table-boats").join(name)));
        let made = without_added_islands(
            name,
            data_lines(&read(
                repo().join("data/scenarios/table-rear-bases").join(name),
            )),
        );
        assert_eq!(from.len(), made.len(), "{name}: the same lines");
        let differing: Vec<(&String, &String)> =
            from.iter().zip(&made).filter(|(a, b)| a != b).collect();
        let mut names = 0;
        let mut supply = 0;
        let mut at_base = 0;
        let mut home_port = 0;
        let mut available = 0;
        for (a, b) in &differing {
            if a.starts_with("name = ") {
                assert!(b.starts_with(&a[..a.len() - 1]), "{a} / {b}");
                names += 1;
            } else if **a == "supply_t = 960" {
                assert_eq!(**b, "supply_t = 2120");
                supply += 1;
            } else if **a == "at_base = 7" {
                assert_eq!(**b, "at_base = 2");
                at_base += 1;
            } else if **a == "home_port = 7" {
                assert_eq!(**b, "home_port = 2");
                home_port += 1;
            } else if let Some(old) = a.strip_prefix("available_from = ") {
                let old: u32 = old.parse().expect("a day");
                assert_eq!(**b, format!("available_from = {}", old - 1));
                available += 1;
            } else {
                panic!("{name}: an unruled difference: {a} / {b}");
            }
        }
        assert_eq!(
            (names, supply, at_base, home_port, available),
            (1, 1, 9, 9, 9),
            "{name}"
        );
    }
}

fn first_morning(name: &str) -> sim_core::state::TruthState {
    let path = repo().join("data/scenarios/table-rear-bases").join(name);
    common::load_live(&path)
        .expect("the file loads")
        .world
        .umpire()
        .clone()
}

#[test]
fn statement_3_shortland_has_no_supply_and_no_fuel() {
    for name in FILES {
        let truth = first_morning(name);
        let base = &truth.bases[&BaseId::new(7)];
        assert!(base.supply_t.is_zero() && base.fuel_t.is_zero(), "{name}");
    }
}

#[test]
fn statement_4_task_force_30_may_be_used_on_day_9_and_lies_at_rabaul_with_7_t() {
    for name in FILES {
        let text = read(repo().join("data/scenarios/table-rear-bases").join(name));
        let at = text
            .find("[[task_force]]\nid = 30\n")
            .expect("task force 30");
        let block = &text[at..at + text[at..].find("\n[[ship]]").expect("its ship")];
        assert!(block.contains("\navailable_from = 9\n"), "{name}");
        let truth = first_morning(name);
        let rabaul = truth.bases[&BaseId::new(2)].hex;
        let force = &truth.task_forces[&TaskForceId::new(30)];
        assert_eq!(force.from_hex, rabaul, "{name}");
        assert!(
            force.toward_hex.is_none() && force.progress_nm.is_zero(),
            "{name}"
        );
        assert_eq!(
            truth.ships[&ShipId::new(100)].supply_t,
            sim_core::fixed::Fixed::from_int(7),
            "{name}"
        );
    }
}

#[test]
fn statement_6_each_file_has_the_three_islands() {
    for name in FILES {
        let truth = common::load_live(&repo().join("data/scenarios/table-rear-bases").join(name))
            .expect("the file loads");
        let found: Vec<(u32, String, Vec<Option<BaseId>>)> = truth
            .world
            .map()
            .islands()
            .map(|i| {
                (
                    i.number,
                    i.name.clone(),
                    i.places.iter().map(|p| p.base).collect(),
                )
            })
            .collect();
        let b = |n| Some(BaseId::new(n));
        assert_eq!(
            found,
            vec![
                (1, "Guadalcanal".to_string(), vec![b(1)]),
                (2, "Tulagi".to_string(), vec![b(4)]),
                (3, "Guadalcanal and Tulagi".to_string(), vec![b(1), b(4)]),
            ],
            "{name}"
        );
        // the three files are the table's: the made files are the only ones with island 3
        let source = read(repo().join("data/scenarios/table-boats").join(name));
        assert!(
            !source.contains("name = \"Guadalcanal and Tulagi\""),
            "{name}"
        );
        assert_eq!(
            source.matches("[[island]]").count(),
            if name.starts_with("07_") { 0 } else { 2 },
            "{name}: the file it is made from"
        );
    }
    // no scenario file outside table-rear-bases holds the line
    let mut stack = vec![repo().join("data/scenarios")];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("the folder reads") {
            let path = entry.expect("an entry").path();
            if path.is_dir() {
                if path.file_name().is_some_and(|n| n == "table-rear-bases") {
                    continue;
                }
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "toml") {
                assert!(
                    !read(path.clone()).contains("name = \"Guadalcanal and Tulagi\""),
                    "{}",
                    path.display()
                );
            }
        }
    }
}

/// The text of a file with the lines statement 6 adds taken out: `[[island]]`
/// to its places line for island 3 in `07b` and `07c`; every island block in
/// `07_full_slice`, whose islands are at its end.
fn without_island_lines(name: &str, text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let mut out = Vec::new();
    let mut k = 0;
    while k < lines.len() {
        let is_three = lines[k] == "[[island]]" && lines.get(k + 1) == Some(&"number = 3");
        let is_any = lines[k] == "[[island]]";
        if (name.starts_with("07_") && is_any) || (!name.starts_with("07_") && is_three) {
            k += 4;
        } else {
            out.push(lines[k]);
            k += 1;
        }
    }
    out.join("\n") + "\n"
}

#[test]
fn statement_7_an_island_is_not_hashed() {
    for name in FILES {
        let path = repo().join("data/scenarios/table-rear-bases").join(name);
        let text = read(path.clone());
        let stripped = without_island_lines(name, &text);
        assert!(stripped.len() < text.len(), "{name}");
        let play = |text: &str| {
            let mut scenario =
                common::from_text_with(text, &path, &sim_data::LoadOptions::default())
                    .expect("the file loads");
            let held = sim_staff::staffs();
            let mut hashes = vec![scenario.world.state_hash()];
            for _ in 0..3 {
                scenario
                    .next_day_watched(sim_staff::as_staffs(&held), &mut |_, _| {})
                    .expect("a day resolves")
                    .expect("a day to play");
                hashes.push(scenario.world.state_hash());
            }
            (hashes[0], hashes[3])
        };
        assert_eq!(play(&text), play(&stripped), "{name}");
    }
}
