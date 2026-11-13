//! The briefings as text, byte for byte. The files are in `tests/golden/` and
//! are read by the architect line by line. To write them again after a change
//! to the text, run with `FLAGPILOT_UPDATE_GOLDEN=1` and read the difference.

mod support;

use std::path::Path;

use sim_core::state::Side;
use support::{load, play};

/// The text of the briefing of `side` at the start of each day it is asked
/// for, by the day about to be played.
fn briefings(name: &str, side: Side, days: &[u32]) -> Vec<(u32, String)> {
    let mut scenario = load(name, 1942);
    let title = scenario.name.clone();
    let last_day = scenario.days;
    let mut out = Vec::new();
    play(&mut scenario, |s| {
        let view = s.world.side_view(side);
        let day = view.day() + 1;
        if days.contains(&day) {
            out.push((
                day,
                sim_agent::briefing_for_game(&view, &title, Some(last_day)).to_text(),
            ));
        }
    });
    out
}

fn assert_golden(file: &str, actual: &str) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/golden")
        .join(file);
    if std::env::var("FLAGPILOT_UPDATE_GOLDEN").is_ok() {
        std::fs::write(&path, actual).expect("the golden file writes");
        return;
    }
    let expected = std::fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("{file} is missing; run with FLAGPILOT_UPDATE_GOLDEN=1"));
    assert_eq!(expected, actual, "{file} differs");
}

#[test]
fn the_base_raid_briefings_of_the_allied_side() {
    for (day, text) in briefings("06a_base_raid.toml", Side::Allied, &[2, 3]) {
        assert_golden(&format!("06a_base_raid_allied_day{day}.txt"), &text);
    }
}

#[test]
fn eastern_solomons_after_the_day_for_the_allied_side() {
    // After the one day of the scenario: the start of day 2.
    for (_, text) in briefings("05_eastern_solomons.toml", Side::Allied, &[2]) {
        assert_golden("05_eastern_solomons_allied_after_day1.txt", &text);
    }
}

#[test]
fn the_landing_briefings_of_the_allied_side() {
    // The land units part (decision 0066, 21): before the landing (day 1),
    // after the first night ashore (day 3), and at the end of day 5.
    for (day, text) in briefings("21_landing.toml", Side::Allied, &[1, 3, 6]) {
        assert_golden(&format!("21_landing_allied_day{day}.txt"), &text);
    }
}

#[test]
fn a_briefing_is_the_same_text_every_time_it_is_made() {
    let once = briefings("06a_base_raid.toml", Side::Japanese, &[1, 2, 3]);
    let twice = briefings("06a_base_raid.toml", Side::Japanese, &[1, 2, 3]);
    assert_eq!(once, twice);
    assert_eq!(once.len(), 3);
}
