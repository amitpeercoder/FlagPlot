//! The briefing, the lists, `get_entity` and `explain` on whole scenarios:
//! every answer is valid against its schema, every number in the text is in
//! the data, and no enemy unit appears except as a contact or in an event the
//! side may see (decision 0048).

mod support;

use sim_core::state::Side;
use support::check::check_view;
use support::{load, play, schemas};

const SIDES: [Side; 2] = [Side::Allied, Side::Japanese];

#[test]
fn the_base_raid_every_day_for_both_sides() {
    let schemas = schemas();
    let mut scenario = load("06a_base_raid.toml", 1942);
    let name = scenario.name.clone();
    let mut days = 0;
    play(&mut scenario, |s| {
        for side in SIDES {
            let view = s.world.side_view(side);
            let checked = check_view(&view, &name, &schemas, true);
            if view.day() > 0 {
                assert!(
                    checked.explained > 0,
                    "day {}: nothing to explain",
                    view.day()
                );
            }
        }
        days += 1;
    });
    // The starts of days 1, 2 and 3, and the state after the run.
    assert_eq!(days, 4);
}

#[test]
fn eastern_solomons_on_day_one_and_after_the_run() {
    let schemas = schemas();
    let mut scenario = load("05_eastern_solomons.toml", 1942);
    let name = scenario.name.clone();
    let mut seen = Vec::new();
    play(&mut scenario, |s| {
        for side in SIDES {
            let view = s.world.side_view(side);
            check_view(&view, &name, &schemas, true);
        }
        seen.push(s.world.day());
    });
    assert_eq!(seen, vec![0, 1]);
}

#[test]
fn the_night_action_after_the_fight() {
    let schemas = schemas();
    let mut scenario = load("04a_night_action.toml", 1942);
    let name = scenario.name.clone();
    scenario.run_to_end().expect("the run resolves");
    let mut fights = 0;
    for side in SIDES {
        let view = scenario.world.side_view(side);
        let checked = check_view(&view, &name, &schemas, true);
        fights += checked.briefing.action.fights.len();
        assert!(checked.explained > 0);
    }
    assert!(fights > 0, "a side knew of the fight");
}

#[test]
fn the_full_slice_every_day_for_both_sides() {
    let schemas = schemas();
    let mut scenario = load("07_full_slice.toml", 1942);
    let name = scenario.name.clone();
    let mut days = 0;
    let mut explained = 0;
    play(&mut scenario, |s| {
        for side in SIDES {
            let view = s.world.side_view(side);
            // Every event is explained on the last state, and a tenth of
            // them on the days before.
            let last = s.world.day() == s.days;
            let checked = check_view(&view, &name, &schemas, last);
            explained += checked.explained;
        }
        days += 1;
    });
    assert_eq!(days, 15);
    assert!(explained > 100);
}

const PARTS: [&str; 11] = [
    "heading",
    "objectives",
    "orders",
    "contacts",
    "action",
    "losses",
    "task_forces",
    "air_groups",
    "bases",
    "arrivals",
    "warnings",
];

const HEADINGS: [&str; 11] = [
    "HEADING",
    "OBJECTIVES",
    "ORDERS",
    "CONTACTS",
    "ACTION YESTERDAY",
    "LOSSES YESTERDAY",
    "TASK FORCES",
    "AIR GROUPS",
    "BASES",
    "ARRIVALS",
    "WARNINGS",
];

#[test]
fn the_ten_parts_come_in_the_same_order_every_day_and_day_one_has_no_yesterday() {
    for name in [
        "06a_base_raid.toml",
        "05_eastern_solomons.toml",
        "07_full_slice.toml",
    ] {
        let mut scenario = load(name, 1942);
        let title = scenario.name.clone();
        play(&mut scenario, |s| {
            for side in SIDES {
                let view = s.world.side_view(side);
                let b = sim_agent::briefing(&view, &title);
                // The data: the parts in order, by their place in the JSON text.
                let json = serde_json::to_string(&b).unwrap();
                let at: Vec<usize> = PARTS
                    .iter()
                    .map(|p| json.find(&format!("\"{p}\":")).expect("a part"))
                    .collect();
                assert!(at.windows(2).all(|w| w[0] < w[1]), "{name}: {at:?}");
                // The text: the ten headings, each on a line of its own, in order.
                let text = b.to_text();
                let lines: Vec<&str> = text.lines().collect();
                let heads: Vec<usize> = HEADINGS
                    .iter()
                    .map(|h| lines.iter().position(|l| l == h).expect("a heading"))
                    .collect();
                assert!(heads.windows(2).all(|w| w[0] < w[1]), "{name}");
                // Every part says something, if only "none"; every fact is a "- " line.
                for (i, start) in heads.iter().enumerate() {
                    let end = heads.get(i + 1).copied().unwrap_or(lines.len());
                    let body: Vec<&&str> = lines[start + 1..end]
                        .iter()
                        .filter(|l| !l.is_empty())
                        .collect();
                    assert!(!body.is_empty(), "{name}: {} is empty", HEADINGS[i]);
                    assert!(
                        body.iter()
                            .all(|l| l.starts_with("- ") || l.starts_with("  - ")),
                        "{name}: {body:?}"
                    );
                }
                if view.day() == 0 {
                    assert_eq!(b.heading.day, 1);
                    assert_eq!(b.heading.yesterday, None);
                    assert!(text.contains("Covers: the state only"));
                    assert_eq!(b.orders.accepted, 0);
                    assert!(b.orders.refused.is_empty() && b.losses.ships_sunk.is_empty());
                    assert!(b.action.fights.is_empty() && b.action.strikes.is_empty());
                } else {
                    assert_eq!(b.heading.yesterday, Some(view.day()));
                }
            }
        });
    }
}
