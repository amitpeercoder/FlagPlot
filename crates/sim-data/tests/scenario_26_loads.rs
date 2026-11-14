//! Scenario 26 (a force that acts at a place and steams on, decision 0096)
//! loads on the frozen step 1 figures and its route orders carry the marks
//! the file gives. The hand calculation is in
//! `tests/scenario_26_act_and_steam_on.rs`; this only proves the file is
//! sound.

use std::collections::BTreeSet;
use std::path::Path;

use sim_core::hex::Hex;
use sim_core::orders::{OrderKind, TaskForceOrder};
use sim_core::state::Side;
use sim_data::{LoadOptions, Scenario};

/// The route orders of day 1 as (side, task force, waypoints, marks).
fn route_orders(scenario: &Scenario) -> Vec<(Side, u32, Vec<Hex>, BTreeSet<Hex>)> {
    scenario.scripted_orders()[&1]
        .iter()
        .filter_map(|(side, request)| match &request.kind {
            OrderKind::TaskForce {
                target,
                order: TaskForceOrder::SetRoute { waypoints, act_at },
            } => Some((*side, target.raw(), waypoints.clone(), act_at.clone())),
            _ => None,
        })
        .collect()
}

#[test]
fn scenario_26_loads_and_its_route_orders_carry_the_marks_of_the_file() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let path = root.join("data/scenarios/26_act_and_steam_on.toml");
    let scenario = Scenario::load_with(&path, &LoadOptions::frozen_step1(&root)).expect("loads");
    let mut routes = route_orders(&scenario);
    routes.sort_by_key(|route| route.1);
    let marks = |hexes: &[Hex]| hexes.iter().copied().collect::<BTreeSet<Hex>>();
    assert_eq!(
        routes,
        vec![
            (
                Side::Japanese,
                1,
                vec![Hex::new(0, 0), Hex::new(4, 0)],
                marks(&[Hex::new(0, 0)])
            ),
            (
                Side::Allied,
                2,
                vec![Hex::new(0, 3), Hex::new(4, 3)],
                marks(&[Hex::new(0, 3)])
            ),
            (
                Side::Allied,
                3,
                vec![Hex::new(2, 5), Hex::new(4, 5)],
                marks(&[Hex::new(2, 5)])
            ),
        ]
    );
}

#[test]
fn a_route_order_with_no_act_at_line_carries_no_mark() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let path = root.join("data/scenarios/26_act_and_steam_on.toml");
    let text = std::fs::read_to_string(&path)
        .expect("the file reads")
        .replace("act_at = [[0, 0]]\n", "");
    let scenario =
        Scenario::from_text_with(&text, &path, &LoadOptions::frozen_step1(&root)).expect("loads");
    let routes = route_orders(&scenario);
    let one = routes.iter().find(|route| route.1 == 1).expect("force 1");
    assert!(one.3.is_empty());
    assert_eq!(routes.iter().filter(|route| route.3.is_empty()).count(), 1);
}
