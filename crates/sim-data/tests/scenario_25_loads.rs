//! Scenario 25 (marching overland, decision 0094) loads on the frozen step 1
//! figures and resolves its five days. The hand calculation is in
//! `tests/scenario_25_march.rs`; this only proves the file is sound.

use std::path::Path;

use sim_core::hex::Hex;
use sim_core::ids::LandUnitId;
use sim_data::{LoadOptions, Scenario};

#[test]
fn scenario_25_loads_and_resolves_five_days_with_both_marches_arriving() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let path = root.join("data/scenarios/25_march.toml");
    let mut scenario =
        Scenario::load_with(&path, &LoadOptions::frozen_step1(&root)).expect("loads");
    assert!(scenario
        .world
        .map()
        .has_land_link(Hex::new(0, 0), Hex::new(1, 0)));
    assert!(scenario
        .world
        .map()
        .has_land_link(Hex::new(2, 0), Hex::new(3, 0)));
    assert!(!scenario
        .world
        .map()
        .has_land_link(Hex::new(1, 0), Hex::new(2, 0)));
    let mut days = 0;
    while scenario.next_day().expect("a day resolves").is_some() {
        days += 1;
        if std::env::var("SHOW_EVENTS").is_ok() {
            for event in scenario.world.events().iter().filter(|e| e.day == days) {
                println!("{event}");
            }
        }
    }
    assert_eq!(days, 5);
    let truth = scenario.world.umpire();
    assert_eq!(
        truth.land_units[&LandUnitId::new(1)].hex,
        Some(Hex::new(1, 0))
    );
    assert_eq!(
        truth.land_units[&LandUnitId::new(3)].hex,
        Some(Hex::new(3, 0))
    );
}
