//! The architect's seven changes to stage 9b, each tested where it shows.

mod support;

use serde_json::Value;
use sim_agent::{briefing, explain};
use sim_core::state::Side;
use support::{load, play};

const SIDES: [Side; 2] = [Side::Allied, Side::Japanese];

/// Change 1: what a side sees of the log is the same after later days are
/// resolved: its list after day D is a prefix of its list after day D + 1. This is what
/// makes its own numbering (1, 2, 3...) stable.
#[test]
fn a_sides_events_after_a_day_are_a_prefix_of_its_events_after_the_next() {
    for name in [
        "07_full_slice.toml",
        "06_rabaul_guadalcanal.toml",
        "05_eastern_solomons.toml",
    ] {
        let mut scenario = load(name, 1942);
        let mut before: [Vec<sim_core::side_events::SideEvent>; 2] = [Vec::new(), Vec::new()];
        let mut days = 0;
        play(&mut scenario, |s| {
            for (i, side) in SIDES.iter().enumerate() {
                let now = s.world.side_view(*side).events();
                assert!(now.len() >= before[i].len(), "{name} {side:?}");
                assert!(
                    now[..before[i].len()] == before[i][..],
                    "{name} {side:?}: the list after day {} is not a prefix of the list after day {}",
                    s.world.day().saturating_sub(1),
                    s.world.day()
                );
                before[i] = now;
            }
            days += 1;
        });
        assert!(days > 1);
    }
}

/// Every event number in a briefing is the side's own: 1 to how many events
/// the side sees, and `explain` of it is that event.
fn numbers_in(value: &Value, out: &mut Vec<u64>) {
    match value {
        Value::Array(a) => a.iter().for_each(|v| numbers_in(v, out)),
        Value::Object(o) => {
            for (k, v) in o {
                match (k.as_str(), v) {
                    ("event" | "last_report_event", Value::Number(n)) => out.extend(n.as_u64()),
                    ("events", Value::Array(a)) => out.extend(a.iter().filter_map(Value::as_u64)),
                    _ => numbers_in(v, out),
                }
            }
        }
        _ => {}
    }
}

#[test]
fn the_event_numbers_of_a_briefing_are_the_sides_own_and_explain_takes_them() {
    let mut scenario = load("05_eastern_solomons.toml", 1942);
    let title = scenario.name.clone();
    scenario.run_to_end().unwrap();
    let mut checked = 0;
    for side in SIDES {
        let view = scenario.world.side_view(side);
        let events = view.events();
        let b = serde_json::to_value(briefing(&view, &title)).unwrap();
        let mut numbers = Vec::new();
        numbers_in(&b, &mut numbers);
        for n in numbers {
            assert!((1..=events.len() as u64).contains(&n), "{side:?}: {n}");
            let answer = serde_json::to_value(explain(&view, n as u32)).unwrap();
            assert_eq!(answer["kind"], "explain");
            assert_eq!(answer["event"], n);
            assert_eq!(answer["nodes"][0]["day"], events[n as usize - 1].day);
            checked += 1;
        }
    }
    assert!(checked > 10);
}

/// Change 2: Own fire covers the air and the guns.
#[test]
fn own_fire_lists_the_fighters_and_the_guns() {
    let mut scenario = load("06a_base_raid.toml", 1942);
    let title = scenario.name.clone();
    scenario.run_to_end().unwrap();
    let view = scenario.world.side_view(Side::Allied);
    let b = briefing(&view, &title);
    assert!(!b.action.air_fire.is_empty(), "the patrol fired");
    assert!(b
        .action
        .air_fire
        .iter()
        .all(|f| f.firer.handle == "air_group:3"));
    assert_eq!(b.action.guns.len(), 1);
    assert_eq!(b.action.guns[0].by.handle, "base:2");
    assert_eq!(b.action.guns[0].raid.handle, "air_group:1");
    assert_eq!(
        b.action.guns[0].shots.0,
        sim_core::fixed::Fixed::from_int(20)
    );
    let text = b.to_text();
    assert!(
        text.contains("Patrol fire: air_group:3 \"Placeholder defending fighters\" at the"),
        "{text}"
    );
    assert!(
        text.contains(
            "Guns: base:2 \"Placeholder airfield B\" at the raid of air_group:1, 20 shots [event "
        ),
        "{text}"
    );
    // Never the hits.
    assert!(!text.contains("hits scored"));
}

/// Change 3: A strike is followed by what it reported.
#[test]
fn every_attack_and_every_strike_that_found_nothing_has_its_report_line() {
    let mut attacks = 0;
    let mut nothing = 0;
    let mut dropped = 0;
    for name in [
        "05_eastern_solomons.toml",
        "05b_strike.toml",
        "05d_carrier_duel.toml",
        "07_full_slice.toml",
    ] {
        let mut scenario = load(name, 1942);
        let title = scenario.name.clone();
        play(&mut scenario, |s| {
            for side in SIDES {
                let b = briefing(&s.world.side_view(side), &title);
                let text = b.to_text();
                for strike in &b.action.strikes {
                    if strike.kind == "air_attack" {
                        attacks += 1;
                        assert!(strike.reports.iter().all(|r| r.event >= 1));
                    }
                    if strike.kind == "strike_found_nothing" {
                        nothing += 1;
                        // Only the first of several that find the same contact
                        // gone drops it; the others report nothing.
                        dropped += strike
                            .reports
                            .iter()
                            .filter(|r| {
                                r.result == "contact_dropped" || r.result == "contact_not_there"
                            })
                            .count();
                    }
                }
                // A report line follows each attack line.
                let lines: Vec<&str> = text.lines().collect();
                for (i, line) in lines.iter().enumerate() {
                    if line.starts_with("- Air attack: ")
                        || line.starts_with("- Strike found nothing: ")
                    {
                        assert!(lines[i + 1].starts_with("  - Report: "), "{line}");
                    }
                }
            }
        });
    }
    println!("attacks {attacks}, found nothing {nothing}, contacts dropped {dropped}");
    assert!(attacks > 0 && nothing > 0 && dropped > 0);
}

/// Change 4: Aircraft are aircraft, shots are shots.
#[test]
fn aircraft_are_counted_as_aircraft_and_guns_as_shots() {
    let mut scenario = load("05_eastern_solomons.toml", 1942);
    let title = scenario.name.clone();
    scenario.run_to_end().unwrap();
    let text = briefing(&scenario.world.side_view(Side::Allied), &title).to_text();
    assert!(
        text.contains("- Air attack: air_group:2 \"VB-6\" on ship:33, 16 aircraft, bombs [event "),
        "{text}"
    );
    assert!(
        text.contains("- Attack on ship:1 \"USS Enterprise\" by air_group:12: 12 aircraft"),
        "{text}"
    );
    assert!(!text.contains("16 shots"));
    let mut scenario = load("06a_base_raid.toml", 1942);
    let title = scenario.name.clone();
    let mut day3 = String::new();
    play(&mut scenario, |s| {
        if s.world.day() == 2 {
            day3 = briefing(&s.world.side_view(Side::Allied), &title).to_text();
        }
    });
    assert!(
        day3.contains("by air_group:1: 15 aircraft, 4 hits, 2 on aircraft, 0 on supply"),
        "{day3}"
    );
}

/// Change 5: Every refresh is listed with what it changed, so 4 ships to 3 can be followed.
#[test]
fn every_refresh_of_the_day_is_listed_and_the_last_is_the_contact_held() {
    let mut scenario = load("05_eastern_solomons.toml", 1942);
    let title = scenario.name.clone();
    scenario.run_to_end().unwrap();
    let b = briefing(&scenario.world.side_view(Side::Allied), &title);
    let counts: Vec<u32> = b.contacts.refreshed.iter().map(|r| r.ship_count).collect();
    assert_eq!(counts, vec![5, 4, 3]);
    let was: Vec<Option<u32>> = b
        .contacts
        .refreshed
        .iter()
        .map(|r| r.was_ship_count)
        .collect();
    assert_eq!(was, vec![Some(4), Some(5), Some(4)]);
    assert_eq!(b.contacts.new[0].ship_count, 4);
    assert_eq!(b.contacts.held[0].ship_count, 3);
    let text = b.to_text();
    assert!(text.contains("5 ships estimated (was 4)"), "{text}");
    assert!(text.contains("3 ships estimated (was 4)"), "{text}");
}

/// Change 6: Rules not in play are not shown.
#[test]
fn what_a_rule_that_is_not_in_play_would_show_is_left_out() {
    let mut scenario = load("05_eastern_solomons.toml", 1942);
    let title = scenario.name.clone();
    scenario.run_to_end().unwrap();
    let b = briefing(&scenario.world.side_view(Side::Allied), &title);
    let rules = b.heading.rules_in_play;
    assert!(!rules.supply && !rules.fatigue && !rules.wear && !rules.pilots);
    let text = b.to_text();
    assert!(
        !text.contains("fatigue 0")
            && !text.contains("damaged,")
            && !text.contains("has no supply")
    );
    assert!(!text.contains("supply 0 t"));
    assert!(text.contains("fuel 0 t"));
    assert!(text.contains("base supply is not in play") && text.contains("fatigue is not in play"));
    // The data keeps the fields.
    assert!(!b.bases.is_empty() && !b.air_groups.is_empty());

    let mut scenario = load("06c_fatigue_and_rest.toml", 1942);
    let title = scenario.name.clone();
    scenario.run_to_end().unwrap();
    let b = briefing(&scenario.world.side_view(Side::Allied), &title);
    assert!(b.heading.rules_in_play.fatigue);
    assert!(b.to_text().contains("fatigue "));
    let mut scenario = load("06d2_base_supply.toml", 1942);
    let title = scenario.name.clone();
    scenario.run_to_end().unwrap();
    let b = briefing(&scenario.world.side_view(Side::Allied), &title);
    assert!(b.heading.rules_in_play.supply);
    assert!(b.to_text().contains("supply "));
}

/// Change 7: The least-fuelled ship sits under its force, indented.
#[test]
fn the_least_fuelled_ship_is_indented_under_its_force() {
    let mut scenario = load("07_full_slice.toml", 1942);
    let title = scenario.name.clone();
    play(&mut scenario, |s| {
        let text = briefing(&s.world.side_view(Side::Allied), &title).to_text();
        let lines: Vec<&str> = text.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            if line.starts_with("  - Least fuelled: ") {
                assert!(lines[i - 1].starts_with("- task_force:"), "{line}");
            }
        }
        assert!(!text.contains("- task_force:1 least fuelled"));
    });
}
