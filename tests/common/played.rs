//! A scenario played to its end with both sides' staffs, for a new test
//! that reads what the staffs handed in, the truth after each day and the
//! day hashes, and that rebuilds a game from the hand-ins as a saved game
//! replays (taken from scenario 107's `play_from` and `rebuilt`).

use std::collections::BTreeMap;

use sim_core::command::{StaffDecisions, Staffs};
use sim_core::state::{Side, TruthState};
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

pub struct Played {
    /// The scenario as it stands at the end: events, day hashes and command
    /// state are read from it.
    pub scenario: Scenario,
    /// What each side's staff handed in, by day.
    pub handed: BTreeMap<Side, BTreeMap<u32, StaffDecisions>>,
    /// `[0]` the start, `[d]` after day `d`.
    pub truth: Vec<TruthState>,
}

/// Plays the scenario to its end with both sides' staffs, `sim_staff::staffs()`.
pub fn play(mut scenario: Scenario) -> Played {
    let held = staffs();
    let mut handed: BTreeMap<Side, BTreeMap<u32, StaffDecisions>> = Side::ALL
        .into_iter()
        .map(|s| (s, BTreeMap::new()))
        .collect();
    let mut truth = vec![scenario.world.umpire().clone()];
    loop {
        let day = scenario.world.day() + 1;
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
        for side in Side::ALL {
            let of = scenario
                .world
                .staff_handed_in()
                .get(&side)
                .cloned()
                .unwrap_or_default();
            handed.get_mut(&side).expect("a side").insert(day, of);
        }
        truth.push(scenario.world.umpire().clone());
    }
    Played {
        scenario,
        handed,
        truth,
    }
}

/// A game rebuilt from what was handed in to day `upto`, as a saved game
/// replays: `scenario` is the freshly loaded game.
pub fn rebuilt(
    mut scenario: Scenario,
    handed: &BTreeMap<Side, BTreeMap<u32, StaffDecisions>>,
    upto: u32,
) -> Scenario {
    for day in 1..=upto {
        for (side, days) in handed {
            scenario.world.submit_staff(*side, days[&day].clone());
        }
        scenario
            .next_day_watched(Staffs::none(), &mut |_, _| {})
            .expect("a day resolves")
            .expect("a day to resolve");
    }
    scenario
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    fn load() -> Scenario {
        common::load(&common::data_dir().join("scenarios/67_taking_a_place.toml"))
            .expect("scenario 67 loads")
    }

    #[test]
    fn a_play_repeats_and_a_rebuilt_game_agrees_with_it() {
        let first = play(load());
        let second = play(load());
        assert_eq!(
            first.scenario.world.day_hashes(),
            second.scenario.world.day_hashes()
        );
        assert_eq!(first.handed, second.handed);
        let last = first.truth.len() as u32 - 1;
        let again = rebuilt(load(), &first.handed, last);
        assert_eq!(again.world.day_hashes(), first.scenario.world.day_hashes());
    }
}
