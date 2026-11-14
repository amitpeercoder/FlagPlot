//! Scenario 29 (what a land unit is good at, decision 0098) loads on the live
//! and the frozen step 1 figures and resolves its 8 days. The hand
//! calculation is in `tests/scenario_29_skills.rs`; this only proves the file
//! is sound.

use std::path::Path;

use sim_core::ids::LandUnitId;
use sim_data::{LoadOptions, Scenario};

#[test]
fn scenario_29_loads_and_resolves_8_days_on_both_sets_of_figures() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let path = root.join("data/scenarios/29_skills.toml");
    for frozen in [false, true] {
        let mut scenario = if frozen {
            Scenario::load_with(&path, &LoadOptions::frozen_step1(&root)).expect("loads")
        } else {
            Scenario::load(&path).expect("loads")
        };
        let truth = scenario.world.umpire();
        assert_eq!(truth.land_units.len(), 11);
        assert_eq!(truth.bases.len(), 7);
        let skills = &truth.land_units[&LandUnitId::new(1)].skills;
        assert_eq!(skills.mountain, 4);
        assert_eq!(skills.assault, 2);
        let mut days = 0;
        while scenario.next_day().expect("a day resolves").is_some() {
            days += 1;
        }
        assert_eq!(days, 8);
    }
}
