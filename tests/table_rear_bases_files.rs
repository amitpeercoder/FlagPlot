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
//!    `at_base = 2`, `home_port = 2` and `available_from` one less. Its
//!    comments differ in the header and in the notes rulings 1 and 2 ask for,
//!    beside base 3, base 7 and each run moved.
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
//! # Amended and added for decision 0232, step 3: the island
//!
//! Written by the architect on Nov 27, 2026, before the generator is changed
//! (`docs/decisions/as-built/0232-a-plan-the-commander-writes.md`, "Planned:
//! step 3", items 5 to 7), and amended the same day on the plan's review and
//! the designer's answers: he said yes to islands 1 and 2 in the full-slice
//! copy (item 6) and to the trial's figures (item 7), so nothing below is
//! conditional. Statement 2 above is amended by 2a and its test is changed to
//! it, on the architect's word given here; statements 1 and 3 to 5 stand as
//! they are.
//!
//! 2a. Read as data, with every comment set aside, each file is the file it
//!     is made from with the lines of statement 2 changed, as counted there
//!     (one name, one `supply_t`, nine each of `at_base`, `home_port` and
//!     `available_from`), and with these lines added and no other:
//!     - in each of the three, the one line
//!       `trial = "../../reference/trial.toml"`, next after the line
//!       `class_sides = "../../reference/class_sides_full_slice.csv"`;
//!     - in `07_full_slice_submarines.toml` alone, the eight lines of
//!       islands 1 and 2, each line equal to the line of
//!       `07b_slice_submarines.toml`, in that file's order;
//!     - in each of the three, after its islands 1 and 2, the four lines of
//!       island 3: `[[island]]`, `number = 3`,
//!       `name = "Guadalcanal and Tulagi"`,
//!       `places = [{ base = 1 }, { base = 4 }]`. In the full-slice copy the
//!       twelve lines of the three islands are the last lines of the file.
//!     So each slice file has 5 lines more than its source and the
//!     full-slice copy 13. With the added lines taken out, the lines are as
//!     many as the source's and differ as statement 2 says.
//! 6.  Loaded, `07_full_slice_submarines.toml` has islands 1 and 2 with the
//!     names and the places `07b_slice_submarines.toml` gives them.
//! 7.  Loaded, each of the three files has island 3, named "Guadalcanal and
//!     Tulagi", of two places in this order: the hex of base 1, then the hex
//!     of base 4. In the two slice files islands 1 and 2 are as they were.
//! 8.  No game moves by it. For each of the three files, the file's text with
//!     the lines 2a adds taken out, loaded from that text, gives the same
//!     state hash at the start and the same hash at the end of day 1, run
//!     with its staffs, as the file kept. (The whole games are compared
//!     outside `cargo test`, by `sim-cli run --events` before and after with
//!     the hash lines kept, and shown to the architect: the plan's item 3.)
//! 9.  Loaded, each of the three files has a trial's figures, and they are
//!     those of `data/reference/trial.toml`.
//!
//! No expected value here is a figure of the game: each is a line of the
//! file, or what the file made from gives on the same commit.
//!
//! ## At decision 0234's step D
//!
//! Step D makes the three files overlays built on the files of
//! `table-boats` and deletes the generator. Its row for this folder was
//! amended for the island on Nov 27, 2026 (the plan's item 5a), and the
//! statements here are then read by that record's readings (a) to (e), as
//! the plan's item 5b rules: statement 1 is withdrawn (d); statements 2 and
//! 2a are withdrawn as statements about lines and held as that record's R3
//! (c); statement 4's line of the file is read from the resolved scenario
//! (a); statements 3, 5, 6, 7 and 9 stand as written; statement 8 stands
//! with "the file's text with the lines 2a adds taken out" read as the
//! overlay's own text with its `[[island]]` rows and its `trial` line taken
//! out, given to `from_text` with the file's path. Nobody but the architect
//! changes a statement at that step either.

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
        let made = data_lines(&read(
            repo().join("data/scenarios/table-rear-bases").join(name),
        ));
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
fn statement_6_the_full_slice_copy_has_islands_1_and_2_as_its_sisters_have_them() {
    panic!("not built: decision 0232, step 3, the island (statement 6)");
}

#[test]
fn statement_7_each_file_has_the_island_of_two_places() {
    panic!("not built: decision 0232, step 3, the island (statement 7)");
}

#[test]
fn statement_8_no_game_moves_by_the_island() {
    panic!("not built: decision 0232, step 3, the island (statement 8)");
}

#[test]
fn statement_9_each_file_names_the_trials_figures() {
    panic!("not built: decision 0232, step 3, the island (statement 9)");
}
