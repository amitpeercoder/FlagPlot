//! A carrier on fire among her aircraft, told and shown (decision 0212): the
//! side's briefing says it, its carrier's line and entity show it, and the
//! other side is told nothing of it. Midway (scenario 1a) sets such fires in
//! most games.

mod support;

use std::path::Path;

use serde_json::{json, Value};
use sim_agent::get_entity;
use sim_core::state::Side;
use sim_data::Scenario;

const ROOT: &str = env!("CARGO_MANIFEST_DIR");

#[test]
fn a_fire_among_a_carriers_aircraft_is_told_to_her_side_and_shown_on_her_line() {
    let schemas = support::schemas();
    let path = Path::new(ROOT).join("../../data/scenarios/midway/01a_midway.toml");
    let mut fires = [0u32; 2];
    let mut shown = 0u32;
    for seed in 1..=4u64 {
        let mut scenario = Scenario::load(&path)
            .expect("midway loads")
            .with_seed(seed)
            .expect("a seed can be set");
        let held = sim_staff::staffs();
        let title = scenario.name.clone();
        let last_day = scenario.days;
        loop {
            let s = &scenario;
            for (n, side) in Side::ALL.into_iter().enumerate() {
                let view = s.world.side_view(side);
                let briefing = sim_agent::briefing_for_game(&view, &title, Some(last_day));
                let text = briefing.to_text();
                let data: Value = serde_json::to_value(&briefing).expect("serialises");
                support::check::valid(&schemas, "briefing.schema.json", &data, "briefing");
                let told = data["losses"]["aircraft_fires"]
                    .as_array()
                    .map_or(0, |a| a.len());
                fires[n] += u32::try_from(told).expect("few");
                if told > 0 {
                    assert!(text.contains("among her aircraft"), "{text}");
                    assert!(!text.contains("draw"), "{text}");
                }
                // A carrier that burns is shown so on her own line and
                // entity, and no other ship is.
                for ship in view.ships() {
                    let handle = support::ship_handle(&view, ship.id);
                    let answer = serde_json::to_value(get_entity(&view, &handle))
                        .expect("an answer serialises");
                    let on_fire = answer["entity"]["on_fire_among_aircraft"] == json!(true);
                    assert_eq!(on_fire, ship.aircraft_fire.is_some(), "{handle}");
                    shown += u32::from(on_fire);
                }
            }
            if scenario
                .next_day_staffed(sim_staff::as_staffs(&held))
                .expect("the day resolves")
                .is_none()
            {
                break;
            }
        }
    }
    assert!(fires[0] + fires[1] > 0, "no fire was told in four games");
    assert!(shown > 0, "no burning carrier was shown in four games");
}
