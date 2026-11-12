#![allow(dead_code)]

pub mod check;
pub mod play;
pub mod validator;

use std::path::{Path, PathBuf};

use sim_data::Scenario;

pub const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

pub fn scenario_path(name: &str) -> PathBuf {
    Path::new(ROOT).join("data/scenarios").join(name)
}

pub fn scenario_text(name: &str) -> String {
    std::fs::read_to_string(scenario_path(name)).expect("the scenario reads")
}

pub fn load_text(text: &str, name: &str, seed: u64) -> Scenario {
    Scenario::from_text(text, &scenario_path(name))
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
