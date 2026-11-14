//! Scenario 32 (a beaten side falls back, decision 0104) loads on the live
//! and the frozen step 1 figures and resolves its 6 days. The hand
//! calculation is in `tests/scenario_32_falling_back.rs`; this only proves the
//! file is sound.

use std::path::Path;

use sim_core::hex::Hex;
use sim_core::ids::{BaseId, LandUnitId};
use sim_core::state::Side;
use sim_data::{LoadOptions, Scenario};

#[test]
fn scenario_32_loads_and_resolves_6_days_on_both_sets_of_figures() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let path = root.join("data/scenarios/32_falling_back.toml");
    for frozen in [false, true] {
        let mut scenario = if frozen {
            Scenario::load_with(&path, &LoadOptions::frozen_step1(&root)).expect("loads")
        } else {
            Scenario::load(&path).expect("loads")
        };
        let truth = scenario.world.umpire();
        assert_eq!(truth.land_units.len(), 7);
        assert_eq!(truth.bases.len(), 8);
        assert!(truth.land_units[&LandUnitId::new(1)].working);
        assert!(truth.land_units[&LandUnitId::new(6)].working);
        assert!(!truth.land_units[&LandUnitId::new(3)].working);
        assert_eq!(truth.bases[&BaseId::new(2)].side, Side::Japanese);
        assert_eq!(truth.bases[&BaseId::new(2)].hex, Hex::new(1, 0));
        assert!(truth.bases.values().all(|b| b.airfield));
        let mut days = 0;
        while scenario.next_day().expect("a day resolves").is_some() {
            days += 1;
        }
        assert_eq!(days, 6);
    }
}
