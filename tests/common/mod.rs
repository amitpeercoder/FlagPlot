//! What every scenario test shares: how it loads a scenario.
//!
//! A hand-calculation test reads its figures from `data/reference/step1`, the
//! figures as they stood at the end of step 1, and not from the live files in
//! `data/reference` that step 2 tunes (docs/step2-spec.md, "Tests while
//! figures move"). It proves the rules compute what the hand calculation
//! says, on known figures; it says nothing about the figures in use. The one
//! place that names the frozen directory is `sim_data::FROZEN_REFERENCE_DIR`;
//! the one place the scenario tests load through is this file.
//!
//! A test that wants figures changed (a variant) never rewrites the
//! scenario's text to point at another file. It asks [`altered_reference`]
//! for a reference root, a copy of the frozen directory with its changes made,
//! and loads the scenario with it.
//!
//! A test that asserts no figure-dependent number and wants the live figures
//! says so by calling [`load_live`].

#![allow(dead_code)]

pub mod no_deck;

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use sim_data::{DataError, LoadOptions, Scenario, FROZEN_REFERENCE_DIR};

/// The repository's `data` directory.
pub fn data_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../data")
}

/// The frozen figures of step 1.
pub fn frozen_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(FROZEN_REFERENCE_DIR)
}

/// Load options that read the reference files from the frozen figures.
pub fn frozen() -> LoadOptions {
    LoadOptions::frozen_step1(&Path::new(env!("CARGO_MANIFEST_DIR")).join(".."))
}

/// Loads a scenario file on the frozen figures.
pub fn load(path: &Path) -> Result<Scenario, DataError> {
    Scenario::load_with(path, &frozen())
}

/// Loads a scenario file on the live figures. For a test that asserts no
/// number which depends on a figure, and says so where it calls this.
pub fn load_live(path: &Path) -> Result<Scenario, DataError> {
    Scenario::load(path)
}

/// Builds a scenario from text, as [`Scenario::from_text`] does (`path` is
/// where the text would sit), on the frozen figures.
pub fn from_text(text: &str, path: &Path) -> Result<Scenario, DataError> {
    from_text_with(text, path, &frozen())
}

/// As [`from_text`], with the given options (usually those of
/// [`altered_reference`]).
pub fn from_text_with(
    text: &str,
    path: &Path,
    options: &LoadOptions,
) -> Result<Scenario, DataError> {
    Scenario::from_text_with(text, path, options)
}

/// Replaces `original` with `changed` in `text`, asserting that `original`
/// appears exactly once.
pub fn replace_once(text: &str, original: &str, changed: &str) -> String {
    assert_eq!(
        text.matches(original).count(),
        1,
        "the text {original:?} appears exactly once"
    );
    text.replace(original, changed)
}

/// A reference root that is the frozen figures with changes: a folder of its
/// own under the build directory holding a copy of every frozen file, then
/// `alter` called with the folder to change what it wants to. Returns the
/// options that read from it. Nothing in the repository's data folder is
/// touched.
pub fn altered_reference(tag: &str, alter: impl FnOnce(&Path)) -> LoadOptions {
    // Tests run in parallel: each root has a folder of its own.
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "reference_{tag}_{}_{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&root).expect("make the reference folder");
    for entry in fs::read_dir(frozen_root()).expect("the frozen folder reads") {
        let entry = entry.expect("an entry reads");
        let name = entry.file_name();
        if name.to_string_lossy().ends_with(".md") {
            continue;
        }
        fs::copy(entry.path(), root.join(&name)).expect("copy a frozen file");
    }
    alter(&root);
    LoadOptions::with_reference_root(root)
}

/// Rewrites the file `name` in `dir` with `change` applied to its text.
pub fn edit_file(dir: &Path, name: &str, change: impl FnOnce(String) -> String) {
    let path = dir.join(name);
    let text = fs::read_to_string(&path).expect("read a file to edit");
    fs::write(&path, change(text)).expect("write the altered file");
}

/// As [`edit_file`], making each exact, asserted-unique replacement in turn.
/// The umpire's key to the numbers of event, order and contact IDs
/// (decision 0086). A test reads the log and the order book, which are the
/// umpire's, so it may read their numbers; it takes the key from a world, as
/// the umpire's tools do. The key is the same for every world.
pub fn umpire_key() -> &'static sim_core::ids::UmpireKey {
    static KEY: std::sync::OnceLock<sim_core::ids::UmpireKey> = std::sync::OnceLock::new();
    KEY.get_or_init(|| {
        load(&data_dir().join("scenarios/01_transit.toml"))
            .expect("scenario 1 loads")
            .world
            .umpire_key()
    })
}

pub fn edit_file_by(dir: &Path, name: &str, edits: &[(&str, &str)]) {
    edit_file(dir, name, |text| {
        edits.iter().fold(text, |text, (original, changed)| {
            replace_once(&text, original, changed)
        })
    });
}
