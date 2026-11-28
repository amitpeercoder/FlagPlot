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

const TRIAL_LINE: &str = "trial = \"../../reference/trial.toml\"";
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
        // Read from the resolved scenario (decision 0234, reading (a)).
        let resolved: toml::Table =
            common::resolved_text(&repo().join("data/scenarios/table-rear-bases").join(name))
                .parse()
                .expect("the resolved scenario is a table");
        let forces = resolved["task_force"].as_array().expect("task forces");
        let force = forces
            .iter()
            .find(|f| f["id"].as_integer() == Some(30))
            .expect("task force 30");
        assert_eq!(force["available_from"].as_integer(), Some(9), "{name}");
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

fn live(name: &str) -> sim_data::Scenario {
    common::load_live(&repo().join("data/scenarios/table-rear-bases").join(name))
        .expect("the file loads")
}

/// An island as the map gives it: its number, name and the hexes of its
/// places in order.
fn islands_of(scenario: &sim_data::Scenario) -> Vec<(u32, String, Vec<sim_core::hex::Hex>)> {
    scenario
        .world
        .map()
        .islands()
        .map(|i| {
            (
                i.number,
                i.name.clone(),
                i.places.iter().map(|p| p.hex).collect(),
            )
        })
        .collect()
}

#[test]
fn statement_6_the_full_slice_copy_has_islands_1_and_2_as_its_sisters_have_them() {
    let sister = islands_of(&live("07b_slice_submarines.toml"));
    let full = islands_of(&live("07_full_slice_submarines.toml"));
    for number in [1, 2] {
        let a = sister.iter().find(|i| i.0 == number).expect("07b's island");
        let b = full.iter().find(|i| i.0 == number).expect("the island");
        assert_eq!(a, b, "island {number}");
    }
    assert_eq!(sister[0].1, "Guadalcanal");
    assert_eq!(sister[1].1, "Tulagi");
}

#[test]
fn statement_7_each_file_has_the_island_of_two_places() {
    for name in FILES {
        let scenario = live(name);
        let truth = scenario.world.umpire();
        let island = scenario
            .world
            .map()
            .island(3)
            .unwrap_or_else(|| panic!("{name}: island 3"));
        assert_eq!(island.name, "Guadalcanal and Tulagi", "{name}");
        let hexes: Vec<_> = island.places.iter().map(|p| p.hex).collect();
        assert_eq!(
            hexes,
            vec![
                truth.bases[&BaseId::new(1)].hex,
                truth.bases[&BaseId::new(4)].hex
            ],
            "{name}"
        );
        if name != FILES[0] {
            let all = islands_of(&scenario);
            assert_eq!(all.len(), 3, "{name}");
            assert_eq!(all[0].1, "Guadalcanal", "{name}");
            assert_eq!(all[1].1, "Tulagi", "{name}");
        }
    }
}

#[test]
fn statement_8_no_game_moves_by_the_island() {
    // The file's text with the lines 2a adds taken out, loaded from that
    // text (at the file's own path), against the file kept: the hash at the
    // start and the hash at the end of day 1, run with the staffs.
    fn start_and_day_1(mut scenario: sim_data::Scenario) -> (u64, u64) {
        let start = scenario.world.state_hash();
        let held = sim_staff::staffs();
        scenario
            .next_day_watched(sim_staff::as_staffs(&held), &mut |_, _| {})
            .expect("day 1 resolves")
            .expect("there is a day 1");
        (start, scenario.world.state_hash())
    }
    for name in FILES {
        let path = repo().join("data/scenarios/table-rear-bases").join(name);
        let text = read(path.clone());
        // The overlay's own text with its `trial` line and its `[[island]]`
        // rows (the last rows of the file) taken out.
        let mut stripped = common::replace_once(&text, &format!("{TRIAL_LINE}\n"), "");
        let at = stripped.find("[[island]]").expect("its islands");
        stripped.truncate(at);
        let without = sim_data::Scenario::from_text(&stripped, &path).expect("the text loads");
        let kept = common::load_live(&path).expect("the file loads");
        assert_eq!(start_and_day_1(without), start_and_day_1(kept), "{name}");
    }
}

#[test]
fn statement_9_each_file_names_the_trials_figures() {
    let figures = sim_data::load_trial_figures(&common::data_dir().join("reference/trial.toml"))
        .expect("the trial's figures load");
    for name in FILES {
        let scenario = live(name);
        assert_eq!(scenario.trial_figures(), Some(&figures), "{name}");
    }
}
