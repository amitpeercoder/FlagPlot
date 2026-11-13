#![allow(dead_code)]

pub mod check;
pub mod play;
pub mod validator;

use std::path::{Path, PathBuf};

use sim_data::{LoadOptions, Scenario};

pub const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

/// Where a scenario is read. Every load in the agent's tests is on the frozen
/// figures, and frozen figures go with frozen scenarios (decision 0073,
/// `data/scenarios/step2-frozen/README.md`): scenarios 5, 7 and 8 are read from
/// their frozen copies in `data/scenarios/step2-frozen`, whose orders the
/// frozen figures accept. Every other scenario is read from
/// `data/scenarios`.
pub fn scenario_path(name: &str) -> PathBuf {
    let frozen_copy = matches!(
        name,
        "05_eastern_solomons.toml" | "07_full_slice.toml" | "08_soak.toml"
    );
    let folder = if frozen_copy {
        "data/scenarios/step2-frozen"
    } else {
        "data/scenarios"
    };
    Path::new(ROOT).join(folder).join(name)
}

pub fn scenario_text(name: &str) -> String {
    std::fs::read_to_string(scenario_path(name)).expect("the scenario reads")
}

/// The agent's tests read the figures frozen at the end of step 1
/// (`data/reference/step1`), not the ones step 2 tunes: the goldens and the
/// passages and ranges they assert are worked on known figures.
pub fn frozen() -> LoadOptions {
    LoadOptions::frozen_step1(Path::new(ROOT))
}

pub fn load_text(text: &str, name: &str, seed: u64) -> Scenario {
    Scenario::from_text_with(text, &scenario_path(name), &frozen())
        .expect("the scenario loads")
        .with_seed(seed)
        .expect("a seed can be set")
}

pub fn load(name: &str, seed: u64) -> Scenario {
    load_text(&scenario_text(name), name, seed)
}

/// Calls `at_start` with the scenario at the start of every day it plays, and
/// once more after the last day. The view's day is then one less than the
/// day about to be played.
pub fn play(scenario: &mut Scenario, mut at_start: impl FnMut(&Scenario)) {
    loop {
        at_start(scenario);
        if scenario.next_day().expect("the day resolves").is_none() {
            break;
        }
    }
}

pub fn schemas() -> validator::Schemas {
    validator::Schemas::load(&Path::new(env!("CARGO_MANIFEST_DIR")).join("schemas"))
}
