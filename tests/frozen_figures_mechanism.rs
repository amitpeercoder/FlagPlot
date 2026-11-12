//! The frozen figures do what they are for (docs/step2-spec.md, "Tests while
//! figures move").
//!
//! The hand-calculation tests read `data/reference/step1` through
//! `common::load`. Step 2 tunes the live figures in `data/reference`. So that
//! this is true and stays true:
//!
//! 1. A live figure changed in a copy of `data/reference` changes a scenario
//!    loaded on that copy (its figure, its hashes) and changes nothing of the
//!    same scenario loaded on the frozen root.
//! 2. A scenario loaded on the frozen root reads every reference file from
//!    there, and a file missing from the root is an error that names it, not a
//!    silent fall back to the live file.
//!
//! Nothing here assumes the live figures equal the frozen ones: after a
//! tuning round they do not, and these must still hold. The real files are
//! never changed; the changed copy is made under the build directory.

mod common;

use std::fs;
use std::path::{Path, PathBuf};

use sim_core::events::Event;
use sim_core::fixed::Fixed;
use sim_data::{LoadOptions, Scenario};

const SCENARIO: &str = "05b_strike.toml";
const KEY: &str = "bomb_hit_chance";

fn scenario_path() -> PathBuf {
    common::data_dir().join("scenarios").join(SCENARIO)
}

/// A copy of the figures files of the live `data/reference` (its own files,
/// not the frozen folder), as a reference root.
fn live_copy(tag: &str) -> PathBuf {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("live_copy_{tag}_{}", std::process::id()));
    fs::create_dir_all(&root).expect("make the folder");
    for entry in fs::read_dir(common::data_dir().join("reference")).expect("reads") {
        let entry = entry.expect("an entry");
        if entry.file_type().expect("a type").is_file() {
            fs::copy(entry.path(), root.join(entry.file_name())).expect("copy");
        }
    }
    root
}

/// The bomb hit chance a scenario was loaded with.
fn bomb_hit_chance(scenario: &Scenario) -> Fixed {
    scenario
        .world
        .ref_data()
        .air_combat
        .as_ref()
        .expect("the scenario has air combat figures")
        .bomb_hit_chance
}

/// What a run leaves: the daily hashes and the whole log.
fn run(mut scenario: Scenario) -> (Vec<u64>, Vec<Event>) {
    scenario.run_to_end().expect("the run resolves");
    (
        scenario.world.day_hashes().to_vec(),
        scenario.world.events().iter().cloned().collect(),
    )
}

#[test]
fn changing_a_live_figure_moves_the_live_run_and_not_the_frozen_one() {
    let path = scenario_path();
    let frozen = || common::load(&path).expect("the frozen scenario loads");

    let before = frozen();
    let frozen_chance = bomb_hit_chance(&before);
    let (frozen_hashes, frozen_events) = run(before);

    // A live figure changes: the bomb hit chance, in a copy of data/reference,
    // to a value the frozen file does not hold.
    let live = live_copy("moved");
    let file = live.join("air_combat.toml");
    let text = fs::read_to_string(&file).expect("reads");
    let line = text
        .lines()
        .find(|line| line.starts_with(&format!("{KEY} = ")))
        .expect("the figure is in the file")
        .to_string();
    let changed = if frozen_chance == "0.9".parse::<Fixed>().expect("a decimal") {
        "0.1"
    } else {
        "0.9"
    };
    fs::write(
        &file,
        text.replace(&line, &format!("{KEY} = \"{changed}\"")),
    )
    .expect("writes");
    let on_live = common::from_text_with(
        &fs::read_to_string(&path).expect("reads"),
        &path,
        &LoadOptions::with_reference_root(&live),
    )
    .expect("the live copy loads");
    assert_eq!(bomb_hit_chance(&on_live), changed.parse::<Fixed>().unwrap());
    assert_ne!(bomb_hit_chance(&on_live), frozen_chance);
    let (live_hashes, live_events) = run(on_live);

    // The same scenario on the changed figures is another run.
    assert_ne!(
        live_hashes, frozen_hashes,
        "the changed figure moves the hashes"
    );
    assert_ne!(live_events, frozen_events, "and the log");

    // On the frozen root, after the change, it is the run it was before.
    let after = frozen();
    assert_eq!(bomb_hit_chance(&after), frozen_chance);
    let (after_hashes, after_events) = run(after);
    assert_eq!(after_hashes, frozen_hashes);
    assert_eq!(after_events, frozen_events);
}

#[test]
fn a_frozen_scenario_records_the_frozen_files_as_its_inputs() {
    let scenario = common::load(&scenario_path()).expect("loads");
    let frozen = common::frozen_root();
    let files = scenario.data_files();
    // The map is read where the scenario names it; every other file is the
    // frozen one, by name.
    let (maps, figures): (Vec<_>, Vec<_>) = files
        .iter()
        .partition(|file| file.to_string_lossy().contains("/map/"));
    assert_eq!(maps.len(), 1);
    assert!(figures.len() >= 5, "{figures:?}");
    for file in figures {
        assert_eq!(file.parent(), Some(frozen.as_path()), "{file:?}");
    }
    // And on the live figures they are the live files.
    let live = common::load_live(&scenario_path()).expect("loads");
    assert!(live
        .data_files()
        .iter()
        .all(|file| !file.starts_with(&frozen)));
}

#[test]
fn a_reference_file_missing_from_the_root_is_an_error_naming_it() {
    let empty = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("reference_with_a_gap_{}", std::process::id()));
    fs::create_dir_all(&empty).expect("make the folder");
    // Everything but the air combat figures.
    for entry in fs::read_dir(common::frozen_root()).expect("reads") {
        let entry = entry.expect("an entry");
        let name = entry.file_name();
        if name != "air_combat.toml" && !name.to_string_lossy().ends_with(".md") {
            fs::copy(entry.path(), empty.join(&name)).expect("copy");
        }
    }
    let error =
        match Scenario::load_with(&scenario_path(), &LoadOptions::with_reference_root(&empty)) {
            Ok(_) => panic!("a missing figures file is an error"),
            Err(error) => error.to_string(),
        };
    assert!(error.contains("air_combat.toml"), "{error}");
    assert!(error.contains("reference root"), "{error}");
}
