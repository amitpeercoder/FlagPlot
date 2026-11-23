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

mod common;

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
