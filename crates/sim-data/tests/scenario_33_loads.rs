//! Scenario 33 (who is engaged in a fight ashore, and an attack that fails,
//! decision 0105) loads on the live and the frozen step 1 figures and
//! resolves its 6 days. The hand calculation is in
//! `tests/scenario_33_engaged_and_failed.rs`; this only proves the file is
//! sound.

use std::path::Path;

use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, LandUnitId};
use sim_core::state::Side;
use sim_data::{LoadOptions, Scenario};

#[test]
fn scenario_33_loads_and_resolves_6_days_on_both_sets_of_figures() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let path = root.join("data/scenarios/33_engaged_and_failed.toml");
    for frozen in [false, true] {
        let mut scenario = if frozen {
            Scenario::load_with(&path, &LoadOptions::frozen_step1(&root)).expect("loads")
        } else {
            Scenario::load(&path).expect("loads")
        };
        let truth = scenario.world.umpire();
        assert_eq!(truth.land_units.len(), 8);
        assert_eq!(truth.bases.len(), 9);
        assert!(truth.land_units.values().all(|u| !u.working));
        assert_eq!(
            truth.land_units[&LandUnitId::new(4)].fatigue,
            Fixed::from_int(40)
        );
        assert_eq!(truth.bases[&BaseId::new(7)].side, Side::Japanese);
        assert_eq!(truth.bases[&BaseId::new(7)].hex, Hex::new(20, 0));
        let mut days = 0;
        while scenario.next_day().expect("a day resolves").is_some() {
            days += 1;
        }
        assert_eq!(days, 6);
    }
}
