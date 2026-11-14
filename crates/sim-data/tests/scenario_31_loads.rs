//! Scenario 31 (an airfield damaged and mended, decision 0103) loads on the
//! live and the frozen step 1 figures and resolves its 6 days. The hand
//! calculation is in `tests/scenario_31_airfield_damage.rs`; this only proves
//! the file is sound.

use std::path::Path;

use sim_core::fixed::Fixed;
use sim_core::ids::{AirGroupId, BaseId, LandUnitId};
use sim_data::{LoadOptions, Scenario};

#[test]
fn scenario_31_loads_and_resolves_6_days_on_both_sets_of_figures() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let path = root.join("data/scenarios/31_airfield_damage.toml");
    for frozen in [false, true] {
        let mut scenario = if frozen {
            Scenario::load_with(&path, &LoadOptions::frozen_step1(&root)).expect("loads")
        } else {
            Scenario::load(&path).expect("loads")
        };
        let truth = scenario.world.umpire();
        assert_eq!(truth.land_units.len(), 2);
        assert_eq!(truth.bases.len(), 4);
        assert_eq!(truth.air_groups.len(), 5);
        assert!(truth.land_units[&LandUnitId::new(1)].working);
        assert!(!truth.land_units[&LandUnitId::new(2)].working);
        assert_eq!(
            truth.bases[&BaseId::new(1)].airfield_damage,
            Fixed::from_int(30)
        );
        assert_eq!(
            truth.bases[&BaseId::new(2)].airfield_damage,
            Fixed::from_int(60)
        );
        assert!(truth.bases[&BaseId::new(3)].airfield);
        assert!(!truth.bases[&BaseId::new(4)].airfield);
        assert!(truth.air_groups.contains_key(&AirGroupId::new(5)));
        let mut days = 0;
        while scenario.next_day().expect("a day resolves").is_some() {
            days += 1;
        }
        assert_eq!(days, 6);
    }
}
