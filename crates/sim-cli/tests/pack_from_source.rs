//! `sim-data` reads through a source (decision 0229, stage 1): the disk, or a
//! pack made by `sim-data` itself from a load.
//!
//! # What the tests are to show
//!
//! Written by the architect in decision 0229, stage 1 (and its "What the
//! review changed", item 20); the tests are written to it. No statement has a
//! hand calculation: every expected value is what the load from the disk
//! gives on the same commit.
//!
//! 1. For each of the eleven scenario files in which the table offers a person
//!    against the engine, named in the test itself, a scenario loaded from its
//!    pack has the same state hash at the start as the one loaded from the
//!    disk, names the same data files in the same order with the same hashes,
//!    and gives the same hash for its first day when run with its staffs.
//!    (The whole games are compared in stage 3; a test in `cargo test` that
//!    ran the 39 days four times would be most of a shard.)
//! 2. A pack holds no file the load did not read, and a load from a pack with
//!    one file taken out fails naming that file's path.
//! 3. Every existing test passes unchanged.
//!
//! Statement 3 is held by `cargo test --workspace` as a whole: no existing
//! test, golden file or expected number was edited for this stage.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use sim_cli::{pack, save};
use sim_data::{Disk, LoadOptions, Noting, Pack, Scenario};

/// The eleven scenario files in which the table offers a person against the
/// engine (`table/scenarios.json`), written out here as the record says.
const OFFERED: [&str; 11] = [
    "data/scenarios/second-proof/05c_eastern_solomons.toml",
    "data/scenarios/second-proof/05e_eastern_solomons.toml",
    "data/scenarios/table-boats/03b_coral_sea_both_oilers_submarines.toml",
    "data/scenarios/table-boats/03c_coral_sea_both_oilers_submarines.toml",
    "data/scenarios/table-boats/01b_midway_submarines.toml",
    "data/scenarios/table-boats/01c_midway_submarines.toml",
    "data/scenarios/table-boats/02b_eastern_solomons_oilers_submarines.toml",
    "data/scenarios/table-boats/02c_eastern_solomons_oilers_submarines.toml",
    "data/scenarios/table-rear-bases/07b_slice_submarines.toml",
    "data/scenarios/table-rear-bases/07c_slice_submarines.toml",
    "data/scenarios/ladder/138_one_ship.toml",
];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// A scenario's path as the test names it: under the top of the repository.
fn path_of(file: &str) -> PathBuf {
    repo_root().join(file)
}

fn pack_for(file: &str) -> Pack {
    Scenario::pack_of(&Disk, &path_of(file), &LoadOptions::default())
        .unwrap_or_else(|e| panic!("{file}: the pack is made: {e}"))
}

/// The data files a scenario names, with the hash of each, as a saved game
/// records them, and the hash of the scenario file.
fn named_files(
    source: &dyn sim_data::Source,
    file: &str,
    scenario: &Scenario,
) -> (Vec<(String, String)>, String) {
    let remembered = save::remember_from(source, scenario).expect("the files are hashed");
    let inputs = save::inputs_from(source, path_of(file).to_str().expect("a path"), scenario)
        .expect("inputs");
    // The two ways of hashing must agree with each other as well.
    let from_inputs: Vec<(String, String)> = inputs
        .data
        .iter()
        .map(|f| (f.file.clone(), f.hash.clone()))
        .collect();
    assert_eq!(remembered.files(), from_inputs.as_slice(), "{file}");
    (remembered.files().to_vec(), inputs.scenario_hash)
}

#[test]
fn a_scenario_from_its_pack_is_the_scenario_from_the_disk() {
    for file in OFFERED {
        let path = path_of(file);
        let options = LoadOptions::default();
        let mut disk = Scenario::load_with(&path, &options).expect("loads from the disk");
        let pack = pack_for(file);
        let mut packed = Scenario::load_via(&pack, &path, &options).expect("loads from the pack");

        assert_eq!(
            disk.world.state_hash(),
            packed.world.state_hash(),
            "{file}: the hash at the start"
        );

        let (disk_files, disk_scenario_hash) = named_files(&Disk, file, &disk);
        let (pack_files, pack_scenario_hash) = named_files(&pack, file, &packed);
        assert_eq!(
            disk.data_files(),
            packed.data_files(),
            "{file}: the files named"
        );
        assert_eq!(disk_files, pack_files, "{file}: the files and their hashes");
        assert_eq!(
            disk_scenario_hash, pack_scenario_hash,
            "{file}: the scenario's hash"
        );

        let held = sim_staff::staffs();
        let first_disk = disk
            .next_day_staffed(sim_staff::as_staffs(&held))
            .expect("the first day resolves")
            .expect("there is a first day");
        let held = sim_staff::staffs();
        let first_pack = packed
            .next_day_staffed(sim_staff::as_staffs(&held))
            .expect("the first day resolves")
            .expect("there is a first day");
        assert_eq!(
            first_disk.state_hash, first_pack.state_hash,
            "{file}: the hash of the first day"
        );
    }
}

#[test]
fn a_pack_holds_the_files_the_load_read_and_no_others() {
    for file in OFFERED {
        let path = path_of(file);
        let pack = pack_for(file);
        // Loading from the pack reads every file it holds, and only those:
        // the load would fail on one it asked for and the pack lacked.
        let noting = Noting::new(&pack);
        Scenario::load_via(&noting, &path, &LoadOptions::default()).expect("loads from the pack");
        let read_pack = noting.pack();
        let read: BTreeSet<&PathBuf> = read_pack.files().keys().collect();
        let held: BTreeSet<&PathBuf> = pack.files().keys().collect();
        assert_eq!(read, held, "{file}: the files the load read are the pack's");
        assert!(
            pack.contains(&path),
            "{file}: the scenario's own file is in its pack"
        );
    }
}

#[test]
fn a_load_from_a_pack_with_a_file_taken_out_names_that_file() {
    // One scenario, every file of its pack taken out in turn; the scenarios'
    // packs share their reference files, so the others are covered by it.
    let file = OFFERED[4];
    let path = path_of(file);
    let pack = pack_for(file);
    assert!(pack.files().len() > 5, "a pack of several files");
    for taken in pack.files().keys() {
        let mut short = pack.clone();
        short.remove(taken).expect("the file was there");
        let error = match Scenario::load_via(&short, &path, &LoadOptions::default()) {
            Ok(_) => panic!("{}: loaded without it", taken.display()),
            Err(error) => error,
        };
        assert_eq!(error.path.as_ref(), Some(taken), "{}", error);
        assert!(error.to_string().contains(&taken.display().to_string()));
    }
}

#[test]
fn the_pack_file_round_trips_and_holds_this_programs_versions() {
    let file = OFFERED[4];
    let options = LoadOptions::default();
    let made = pack::pack_of(&[path_of(file).to_str().expect("a path")], &options).expect("made");
    let text = pack::pack_text(&made).expect("written");
    let (versions, read) = pack::read_pack(&text).expect("read back");
    assert_eq!(versions, pack::versions());
    assert_eq!(read, made);
}
