//! A day that fails in play (decision 0175, amended point 11; scenario 94's
//! property "a world that cannot be played"): a test of the crate, which
//! alone can make a day's step fail. It is compiled only for the crate's
//! own tests, and the source test does not read it.

use std::path::Path;

use sim_core::command::{CommandKind, IntentKind, Leave};
use sim_core::ids::IntentId;
use sim_core::state::Side;
use sim_data::{LoadOptions, Scenario};

use super::{begin, Asked, CourseCommand, Stepped};
use crate::Believed;

/// Scenario 94 on the frozen step 1 figures.
fn scenario_94() -> Scenario {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    Scenario::load_with(
        &root.join("data/scenarios/94_a_trial.toml"),
        &LoadOptions::frozen_step1(&root),
    )
    .expect("scenario 94 loads")
}

fn course() -> Vec<CourseCommand> {
    [
        CommandKind::SetIntent {
            parent: None,
            intent: IntentId::new(1),
            kind: IntentKind::ControlIsland {
                island: 1,
                by_day: None,
            },
        },
        CommandKind::SetLeave {
            leave: Leave::Prepare,
            given: true,
        },
        CommandKind::SetLeave {
            leave: Leave::Launch,
            given: true,
        },
    ]
    .into_iter()
    .map(CourseCommand::first_day)
    .collect()
}

fn playing(scenario: &Scenario) -> super::Playing {
    let view = scenario.world.side_view(Side::Allied);
    begin(
        &view,
        &Believed::default(),
        scenario.trial_figures().expect("its trial's figures"),
        scenario.world.ref_data(),
        scenario.world.map(),
        Asked {
            trial: 1,
            worlds: 4,
            days: 8,
            course: course(),
        },
    )
    .expect("the trial begins")
}

#[test]
fn a_day_that_fails_stops_its_world_and_the_others_are_played() {
    let scenario = scenario_94();
    let whole = playing(&scenario).play_out();
    let mut broken = playing(&scenario);
    broken.fail_on = Some((2, 2));
    let mut failed_steps = Vec::new();
    while let Some(stepped) = broken.step() {
        if let Stepped::Failed(failed) = stepped {
            failed_steps.push((failed.world, failed.day));
        }
    }
    assert_eq!(failed_steps, vec![(2, 2)]);
    let played = broken.play_out();
    let failed = played.failed();
    assert_eq!(failed.len(), 1);
    assert_eq!((failed[0].world, failed[0].day), (2, 2));
    assert_eq!(
        failed[0].words,
        "invalid setup: a day made to fail by a test of the crate"
    );
    assert_eq!(played.worlds.len(), 4);
    for (w, want) in played.worlds.iter().zip(&whole.worlds) {
        assert_eq!(w.number, want.number);
        if w.number == 2 {
            // The day before it, and no more.
            assert_eq!(w.days.len(), 1);
            assert_eq!(w.days[0], want.days[0]);
            assert_eq!(w.world.as_ref().map(|world| world.day()), Some(1));
        } else {
            assert!(w.failed.is_none());
            assert_eq!(w.days, want.days, "world {}", w.number);
        }
    }
}
