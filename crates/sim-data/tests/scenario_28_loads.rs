//! Scenario 28 (the experience of a land unit and training, decision 0097)
//! loads on the live and the frozen step 1 figures and resolves its 62 days.
//! The hand calculation is in `tests/scenario_28_experience_and_training.rs`;
//! this only proves the file is sound.

use std::path::Path;

use sim_core::ids::LandUnitId;
use sim_data::{LoadOptions, Scenario};

#[test]
fn scenario_28_loads_and_resolves_62_days_on_both_sets_of_figures() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let path = root.join("data/scenarios/28_experience_and_training.toml");
    for frozen in [false, true] {
        let mut scenario = if frozen {
            Scenario::load_with(&path, &LoadOptions::frozen_step1(&root)).expect("loads")
        } else {
            Scenario::load(&path).expect("loads")
        };
        let truth = scenario.world.umpire();
        assert_eq!(truth.land_units.len(), 6);
        let experience = |raw: u32| truth.land_units[&LandUnitId::new(raw)].experience;
        assert_eq!(
            (1..=6).map(experience).collect::<Vec<_>>(),
            vec![0, 2, 3, 0, 4, 1]
        );
        assert_eq!(truth.land_units[&LandUnitId::new(2)].fight_days, 9);
        let mut days = 0;
        while scenario.next_day().expect("a day resolves").is_some() {
            days += 1;
        }
        assert_eq!(days, 62);
    }
}
