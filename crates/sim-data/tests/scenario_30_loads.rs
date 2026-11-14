//! Scenario 30 (units that work, decision 0099) loads on the live and the
//! frozen step 1 figures and resolves its 6 days. The hand calculation is in
//! `tests/scenario_30_working_units.rs`; this only proves the file is sound.

use std::path::Path;

use sim_core::ids::{BaseId, LandUnitId};
use sim_data::{LoadOptions, Scenario};

#[test]
fn scenario_30_loads_and_resolves_6_days_on_both_sets_of_figures() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let path = root.join("data/scenarios/30_working_units.toml");
    for frozen in [false, true] {
        let mut scenario = if frozen {
            Scenario::load_with(&path, &LoadOptions::frozen_step1(&root)).expect("loads")
        } else {
            Scenario::load(&path).expect("loads")
        };
        let truth = scenario.world.umpire();
        assert_eq!(truth.land_units.len(), 12);
        assert_eq!(truth.bases.len(), 7);
        assert!(truth.land_units[&LandUnitId::new(1)].working);
        assert!(!truth.land_units[&LandUnitId::new(4)].working);
        assert!(!truth.bases[&BaseId::new(1)].airfield);
        assert!(truth.bases[&BaseId::new(2)].airfield);
        let mut days = 0;
        while scenario.next_day().expect("a day resolves").is_some() {
            days += 1;
        }
        assert_eq!(days, 6);
    }
}
