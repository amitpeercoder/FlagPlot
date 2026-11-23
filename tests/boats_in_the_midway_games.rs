//! The Midway games with submarines: decision 0225, as amended on review on
//! Nov 23, 2026.
//!
//! Four copies of Midway in `data/scenarios/table-boats/`, made by
//! `scripts/table-boats/make_files.py`: of the table's copies of files b, c
//! and d (`data/scenarios/table-hunger/`), the three copies being what the
//! table's Midway plays (the designer's answer of Nov 23, 2026), and of file a
//! (`data/scenarios/midway/01a_midway.toml`, on the table's ground figures),
//! kept for the measure of amended rule 8 and not offered. Each has five
//! boats on station: USS Tambor, USS Trout, USS Nautilus and USS Gato of
//! the Allied side, on the arcs of Bates's disposition of 2 June read by
//! amended rule 2, and I-168 of the Japanese side in Midway's hex (amended
//! rule 3). The hexes are worked by hand in the decision ("The hexes,
//! worked by hand").
//!
//! The copies play on the live figures, as the table plays them: no
//! statement here holds a number that rests on a figure (a hex, a class id,
//! a base, a count of boats, the text of a file), and the class's torpedo
//! and salvoes are compared with the class file the copy loads, not with a
//! number.
//!
//! # The statements the new test holds
//!
//! The statements are the architect's, written before the code (decision
//! 0225, "The statements the new test holds"), and are kept here as
//! written. A mismatch goes back to the architect, and nobody else changes
//! a statement here.
//!
//! Each copy, loaded:
//!
//! 1. It has five submarines: Tambor, Trout, Nautilus and Gato of the
//!    Allied side and I-168 of the Japanese, and no other.
//! 2. They start at (1, 3), (1, 1), (4, -2), (1, -1) and (5, 1) in that
//!    order, each on station there, each in a hex ships can enter.
//! 3. Tambor and Trout are of class 6001, Gato of 6002, Nautilus of 6011
//!    and I-168 of 6005, each with her class's torpedo and salvoes.
//! 4. The Allied boats' home is Midway and I-168's is Wake.
//! 5. It has no suspected waters.
//! 6. Taken with its submarines and their three keys removed, it is the
//!    game it copies, line for line but for its header and its ground
//!    figures as the copy it comes from has them.
//! 7. Played on seed 1 with no order to any boat, every boat still afloat
//!    at the end is in the hex she started in.
//!
//! The script that makes the copies is run with `--check`: each copy kept
//! is the one it makes, and the folder holds nothing else.

mod common;

use std::path::PathBuf;

use sim_core::hex::Hex;
use sim_core::ids::{BaseId, SubmarineClassId, SubmarineId};
use sim_core::state::{Side, SubmarineCondition, TruthState};
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

const LETTERS: [char; 4] = ['a', 'b', 'c', 'd'];

/// A boat as the statements give her: id, name, side, class, start, home.
type Boat = (u32, &'static str, Side, u32, (i32, i32), u32);

/// The five boats, in the order of the statements.
const BOATS: [Boat; 5] = [
    (1, "USS Tambor", Side::Allied, 6001, (1, 3), 1),
    (2, "USS Trout", Side::Allied, 6001, (1, 1), 1),
    (3, "USS Nautilus", Side::Allied, 6011, (4, -2), 1),
    (4, "USS Gato", Side::Allied, 6002, (1, -1), 1),
    (5, "I-168", Side::Japanese, 6005, (5, 1), 2),
];

const KEYS: [&str; 3] = [
    "submarine_classes = \"../../reference/submarine_classes_1942.csv\"",
    "submarine_torpedoes = \"../../reference/submarine_torpedoes_1942.csv\"",
    "submarines = \"../../reference/submarines.toml\"",
];

/// Where the section of boats begins: the blank line before it is the
/// last line of the file it copies.
const SECTION: &str = "\n\n# SUBMARINES (decision 0225";

fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn copy(letter: char) -> PathBuf {
    common::data_dir().join(format!(
        "scenarios/table-boats/01{letter}_midway_submarines.toml"
    ))
}

/// The file each copy copies.
fn source(letter: char) -> PathBuf {
    if letter == 'a' {
        common::data_dir().join("scenarios/midway/01a_midway.toml")
    } else {
        common::data_dir().join(format!("scenarios/table-hunger/01{letter}_midway.toml"))
    }
}

/// Loaded on the live figures, as the table plays it (no number asserted
/// rests on a figure).
fn load(letter: char) -> Scenario {
    common::load_live(&copy(letter)).unwrap_or_else(|e| panic!("copy {letter} loads: {e}"))
}

fn read(path: &PathBuf) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// A file without its header: the comment lines (and blank lines) before
/// its first key.
fn body(text: &str) -> Vec<&str> {
    let lines: Vec<&str> = text.lines().collect();
    let first = lines
        .iter()
        .position(|l| {
            let l = l.trim_start();
            !l.is_empty() && !l.starts_with('#')
        })
        .expect("a key");
    lines[first..].to_vec()
}

#[test]
fn statement_1_five_submarines_and_no_other() {
    for letter in LETTERS {
        let scenario = load(letter);
        let truth = scenario.world.umpire();
        let have: Vec<(u32, String, Side)> = truth
            .submarines
            .values()
            .map(|b| (b.id.raw(), b.name.clone(), b.side))
            .collect();
        let want: Vec<(u32, String, Side)> =
            BOATS.iter().map(|b| (b.0, b.1.to_string(), b.2)).collect();
        assert_eq!(have, want, "copy {letter}");
    }
}

#[test]
fn statement_2_they_start_on_station_in_hexes_ships_can_enter() {
    for letter in LETTERS {
        let scenario = load(letter);
        let truth = scenario.world.umpire();
        for (id, name, _, _, (q, r), _) in BOATS {
            let boat = &truth.submarines[&SubmarineId::new(id)];
            let start = Hex::new(q, r);
            assert_eq!(boat.hex, start, "copy {letter}: {name}");
            assert_eq!(boat.station, start, "copy {letter}: {name}");
            assert!(boat.passage.is_none(), "copy {letter}: {name}");
            assert!(boat.on_station(1), "copy {letter}: {name} on station");
            assert!(
                scenario.world.map().is_navigable(start),
                "copy {letter}: {name} at {start:?}, a hex ships can enter"
            );
        }
    }
}

#[test]
fn statement_3_each_of_her_class_with_its_torpedo_and_salvoes() {
    for letter in LETTERS {
        let scenario = load(letter);
        let data = scenario
            .world
            .ref_data()
            .submarines
            .as_ref()
            .expect("the submarine files");
        let truth = scenario.world.umpire();
        for (id, name, _, class, _, _) in BOATS {
            let boat = &truth.submarines[&SubmarineId::new(id)];
            assert_eq!(
                boat.class,
                SubmarineClassId::new(class),
                "copy {letter}: {name}"
            );
            let of_class = &data.classes[&boat.class];
            assert!(
                data.torpedoes.contains_key(&of_class.torpedo),
                "copy {letter}: {name}'s class names a torpedo the files give"
            );
            assert_eq!(
                boat.salvoes, of_class.salvoes_carried,
                "copy {letter}: {name} has her class's salvoes"
            );
            assert!(boat.salvoes > 0, "copy {letter}: {name}");
        }
    }
}

#[test]
fn statement_4_homes_midway_and_wake() {
    for letter in LETTERS {
        let scenario = load(letter);
        let truth = scenario.world.umpire();
        assert_eq!(truth.bases[&BaseId::new(1)].name, "Midway");
        assert_eq!(truth.bases[&BaseId::new(2)].name, "Wake");
        for (id, name, _, _, _, home) in BOATS {
            let boat = &truth.submarines[&SubmarineId::new(id)];
            assert_eq!(boat.home, BaseId::new(home), "copy {letter}: {name}");
        }
    }
}

#[test]
fn statement_5_no_suspected_waters() {
    for letter in LETTERS {
        let scenario = load(letter);
        assert!(
            scenario.world.umpire().waters_waiting.is_empty(),
            "copy {letter}"
        );
        for side in [Side::Allied, Side::Japanese] {
            assert!(
                scenario.world.side_view(side).suspected_waters().is_empty(),
                "copy {letter}: {side:?}"
            );
        }
        assert!(
            !read(&copy(letter))
                .lines()
                .any(|l| l.trim() == "[[suspected_waters]]"),
            "copy {letter}"
        );
    }
}

#[test]
fn statement_6_without_its_boats_it_is_the_game_it_copies() {
    for letter in LETTERS {
        let text = read(&copy(letter));
        let at = text.find(SECTION).expect("the section of boats");
        let without = &text[..at + 1];
        assert_eq!(
            text[at + 1..].matches("[[submarine]]").count(),
            5,
            "copy {letter}: the section holds the five boats"
        );
        assert_eq!(
            without.matches("submarine]]").count(),
            0,
            "copy {letter}: no boat before the section"
        );
        let mut lines = body(without);
        for key in KEYS {
            let found: Vec<usize> = lines
                .iter()
                .enumerate()
                .filter(|(_, l)| **l == key)
                .map(|(i, _)| i)
                .collect();
            assert_eq!(found.len(), 1, "copy {letter}: {key}");
            lines.remove(found[0]);
        }
        let original = read(&source(letter));
        let mut want = body(&original);
        if letter == 'a' {
            // File a's copy is on the table's ground figures, as the copies
            // of b, c and d come from files that are.
            let i = want
                .iter()
                .position(|l| *l == "ground = \"../../reference/ground.toml\"")
                .expect("file a names the live ground figures");
            want[i] = "ground = \"../../reference/ground_hunger.toml\"";
        } else {
            assert!(
                want.contains(&"ground = \"../../reference/ground_hunger.toml\""),
                "the table's copy {letter} names the table's ground figures"
            );
        }
        assert_eq!(lines, want, "copy {letter}");
    }
}

fn played_on_seed_1(letter: char) -> TruthState {
    let mut scenario = load(letter).with_seed(1).expect("seed 1");
    let held = staffs();
    while scenario
        .next_day_watched(as_staffs(&held), &mut |_, _| {})
        .expect("a day resolves")
        .is_some()
    {}
    scenario.world.umpire().clone()
}

#[test]
fn statement_7_played_on_seed_1_every_boat_afloat_is_where_she_started() {
    for letter in LETTERS {
        let end = played_on_seed_1(letter);
        for (id, name, _, _, (q, r), _) in BOATS {
            let boat = &end.submarines[&SubmarineId::new(id)];
            if boat.condition == SubmarineCondition::Sunk {
                continue;
            }
            assert_eq!(
                boat.hex,
                Hex::new(q, r),
                "copy {letter}: {name}, {:?}",
                boat.condition
            );
        }
    }
}

#[test]
fn the_script_says_each_copy_is_the_one_it_makes() {
    let out = std::process::Command::new("python3")
        .arg(repo().join("scripts/table-boats/make_files.py"))
        .arg(repo())
        .arg("--check")
        .output()
        .expect("python3 runs the script");
    let said = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "{said}{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(said.matches(": as made").count(), 4, "{said}");
}
