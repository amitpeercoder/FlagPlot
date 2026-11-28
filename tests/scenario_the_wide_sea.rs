//! The wide sea: the placeholder game that opens the map from Papua to Midway
//! (decision 0242, step 3, and rule 22). No rule is proved here that another
//! test does not prove: what is proved is that the file loads on the map it
//! names, that each of its four bases and each of its four forces is on a
//! coast hex of that map, at the hex the architect's figures give, and that it
//! runs its three days with no order from either side and ends.
//!
//! The hexes are the architect's, worked in
//! `crates/map-tool/tests/papua_to_midway.rs` from the rows of
//! `data/oob/place_positions.csv`: Midway (row 5100) at (31,-41) and Lunga
//! Point (row 6) at (-32,22) for the Allied side; Wake (row 5107) at (2,-26)
//! and Rabaul (row 1) at (-39,14) for the Japanese. No figure is asserted but
//! those, the three days and the four-and-four of bases and forces.
//!
//! The data is placeholder data: see
//! `data/scenarios/wide-map/placeholder_the_wide_sea.toml`, in a folder of its
//! own so that no loop over `data/scenarios/*.toml` runs it (decision 0072).
//! It is loaded on the live figures: it asserts no number that depends on one.

use sim_core::command::Staffs;
use sim_core::hex::Hex;
use sim_core::map::HexKind;
use sim_core::state::Side;
use sim_data::Scenario;

use crate::common::{self, data_dir};

const FILE: &str = "scenarios/wide-map/placeholder_the_wide_sea.toml";

/// Each base, with its side and its hex: the architect's figures.
const BASES: [(u32, Side, i32, i32); 4] = [
    (1, Side::Allied, 31, -41),
    (2, Side::Allied, -32, 22),
    (3, Side::Japanese, 2, -26),
    (4, Side::Japanese, -39, 14),
];

fn load() -> Scenario {
    common::load_live(&data_dir().join(FILE)).expect("the wide sea loads")
}

#[test]
fn it_loads_on_the_map_from_papua_to_midway_and_says_it_is_a_placeholder() {
    let scenario = load();
    assert!(scenario.placeholder);
    assert_eq!(scenario.name, "Placeholder: the wide sea");
    assert_eq!(scenario.days, 3);
    // The map is the 10,395 hexes of the plan.
    let map = scenario.world.map();
    let rows = (-60..=58)
        .flat_map(|r| (-80..=80).map(move |q| Hex::new(q, r)))
        .filter(|h| map.get(*h).is_some())
        .count();
    assert_eq!(rows, 10_395);
}

#[test]
fn every_base_and_force_is_on_a_coast_hex_of_the_map() {
    let scenario = load();
    let truth = scenario.world.umpire();
    let map = scenario.world.map();
    assert_eq!(truth.bases.len(), 4);
    assert_eq!(truth.task_forces.len(), 4);
    for (raw, side, q, r) in BASES {
        let base = &truth.bases[&common::values::base(raw)];
        assert!(
            base.name.starts_with("Placeholder base at "),
            "{}",
            base.name
        );
        assert_eq!(base.side, side);
        assert_eq!(base.hex, Hex::new(q, r), "base {raw}");
        assert_eq!(map.get(base.hex).map(|d| d.kind), Some(HexKind::Coast));
        // One small force at each base, of its side, in the base's hex.
        let force = &truth.task_forces[&common::values::force(raw)];
        assert_eq!(force.side, side);
        assert_eq!(force.position(), base.hex, "force {raw}");
        assert_eq!(force.ships.len(), 2, "force {raw} is small");
        assert_eq!(
            map.get(force.position()).map(|d| d.kind),
            Some(HexKind::Coast)
        );
    }
    // The ships are all of an existing placeholder class.
    assert!(truth
        .ships
        .values()
        .all(|s| s.name.starts_with("Placeholder ")));
}

#[test]
fn it_runs_its_three_days_with_no_orders_from_either_side_and_ends() {
    let mut scenario = load();
    let mut days = 0;
    while scenario
        .next_day_watched(Staffs::none(), &mut |_, _| {})
        .expect("a day resolves")
        .is_some()
    {
        days += 1;
        assert!(days <= 3, "it ends");
    }
    assert_eq!(days, 3);
    assert_eq!(scenario.world.day(), 3);
    // Nobody ordered anything: every force is where it lay.
    let truth = scenario.world.umpire();
    for (raw, _, q, r) in BASES {
        assert_eq!(
            truth.task_forces[&common::values::force(raw)].position(),
            Hex::new(q, r)
        );
    }
}
