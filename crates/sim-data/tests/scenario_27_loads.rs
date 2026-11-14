//! Scenario 27 (preparing a landing, decision 0095) loads on the live and the
//! frozen step 1 figures and resolves its twelve days. The hand calculation is
//! in `tests/scenario_27_preparing_a_landing.rs`; this only proves the file is
//! sound.

use std::path::Path;

use sim_core::ids::{LandUnitId, TaskForceId};
use sim_data::{LoadOptions, Scenario};

#[test]
fn scenario_27_loads_and_resolves_twelve_days_on_both_sets_of_figures() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let path = root.join("data/scenarios/27_preparing_a_landing.toml");
    for frozen in [false, true] {
        let mut scenario = if frozen {
            Scenario::load_with(&path, &LoadOptions::frozen_step1(&root)).expect("loads")
        } else {
            Scenario::load(&path).expect("loads")
        };
        let truth = scenario.world.umpire();
        assert_eq!(truth.task_forces.len(), 5);
        assert_eq!(truth.land_units.len(), 6);
        assert!(truth
            .task_forces
            .values()
            .all(|force| force.preparation.is_none()));
        let mut days = 0;
        while scenario.next_day().expect("a day resolves").is_some() {
            days += 1;
        }
        assert_eq!(days, 12);
        let truth = scenario.world.umpire();
        // Units 1 and 2 are ashore at the end on both sets of figures.
        for unit in [1, 2] {
            assert_eq!(truth.land_units[&LandUnitId::new(unit)].men_ashore, 600);
        }
        // Force 3's order stands: it never landed at B.
        let force = &truth.task_forces[&TaskForceId::new(3)];
        assert!(force.preparation.is_some());
    }
}
