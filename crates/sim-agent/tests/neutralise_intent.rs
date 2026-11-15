//! The measure of a neutralise intent (decision 0120, 8), over scenario 6a: a
//! Japanese bomber group on base_strike at the allied airfield (base 2) for
//! three days, the airfield's defenders on patrol.
//!
//! The raids flown are counted two ways that must agree: by the measure, from
//! the Japanese side's view, and by the umpire's log, as the distinct
//! (day, pulse) pairs of the `BaseStrikeLaunched` events of a Japanese group
//! on base 2. The days in force are the days resolved (the intent is set for
//! day 1), the air raids on the side's own bases are none (the Allies have no
//! bombers on the mission), so every day is met. The allied side has no
//! neutralise intent and so no measure, and neither has an intent that is a
//! hold.

mod support;

use std::collections::BTreeSet;

use sim_agent::neutralise_measure;
use sim_core::events::EventKind;
use sim_core::ids::{BaseId, IntentId};
use sim_core::state::Side;
use support::{load_text, scenario_text};

const NAME: &str = "06a_base_raid.toml";

fn text_with_neutralise(day: u32) -> String {
    format!(
        "{}\n[[command]]\nday = {day}\nside = \"japanese\"\nkind = \"set_intent\"\nintent = 1\nintent_kind = \"neutralise\"\ntarget = 2\n",
        scenario_text(NAME)
    )
}

#[test]
fn the_measure_counts_raids_on_the_target_as_the_log_does() {
    let mut scenario = load_text(&text_with_neutralise(1), NAME, 1942);
    for day in 0..3u32 {
        let view = scenario.world.side_view(Side::Japanese);
        assert_eq!(view.day(), day);
        let measure = neutralise_measure(&view, IntentId::new(1));
        if day == 0 {
            // Scripted for day 1: not taken in before it begins.
            assert_eq!(measure, None);
        } else {
            let m = measure.expect("a neutralise intent");
            assert_eq!(m.days_in_force, day);
            assert_eq!(m.raids_on_own_bases, 0);
            assert_eq!(m.days_met, day);
            let raids: BTreeSet<_> = scenario
                .world
                .events()
                .iter()
                .filter(|e| {
                    matches!(&e.kind, EventKind::BaseStrikeLaunched { base, .. }
                        if *base == BaseId::new(2))
                })
                .map(|e| (e.day, e.pulse))
                .collect();
            assert_eq!(m.raids_flown as usize, raids.len(), "day {day}");
        }
        scenario.next_day().expect("the day resolves");
    }
    let view = scenario.world.side_view(Side::Japanese);
    let m = neutralise_measure(&view, IntentId::new(1)).expect("a neutralise intent");
    assert_eq!(m.days_in_force, 3);
    assert!(m.raids_flown >= 1, "the bombers raid the airfield");
    // The other side has no such intent; and base 2 is not a target of its.
    let allied = scenario.world.side_view(Side::Allied);
    assert_eq!(neutralise_measure(&allied, IntentId::new(1)), None);
}

#[test]
fn an_intent_set_later_counts_from_its_own_day() {
    let mut scenario = load_text(&text_with_neutralise(2), NAME, 1942);
    for _ in 0..3 {
        scenario.next_day().expect("the day resolves");
    }
    let view = scenario.world.side_view(Side::Japanese);
    let m = neutralise_measure(&view, IntentId::new(1)).expect("a neutralise intent");
    assert_eq!((m.days_in_force, m.days_met), (2, 2));
}
