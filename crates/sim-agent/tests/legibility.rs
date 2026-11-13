//! The wording fixes of the closing legibility test (docs/validation/
//! closing-tests.md, L2, L3, L5, L8, L12). Each says in the served text what
//! the engine already does; none changes a rule. The briefing is made from a
//! real scenario and one part of its data is set, then read as text.

mod support;

use sim_agent::{briefing, AttackOnShip, Briefing, DamagedShip, Dec, Handle, SystemChange};
use sim_core::fixed::Fixed;
use sim_core::state::Side;
use support::load;

fn made() -> Briefing {
    let scenario = load("05_eastern_solomons.toml", 1942);
    let view = scenario.world.side_view(Side::Allied);
    briefing(&view, "Eastern Solomons")
}

fn handle(h: &str, name: &str) -> Handle {
    Handle {
        handle: h.to_string(),
        name: Some(name.to_string()),
    }
}

#[test]
fn l2_two_groups_under_one_squadron_name_say_they_are_separate() {
    let mut b = made();
    let mut first = b.air_groups[0].clone();
    first.air_group = handle("air_group:4", "VT-3");
    let mut second = first.clone();
    second.air_group = handle("air_group:11", "VT-3, attack group");
    let mut third = first.clone();
    third.air_group = handle("air_group:8", "VT-8");
    b.air_groups = vec![first, second, third];
    let text = b.to_text();
    assert!(
        text.contains("air_group:4 \"VT-3\" (")
            && text.contains("), a separate group from air_group:11, at "),
        "{text}"
    );
    assert!(
        text.contains("air_group:11 \"VT-3, attack group\" (")
            && text.contains("), a separate group from air_group:4, at "),
        "{text}"
    );
    // The lone squadron is not marked.
    let line = text
        .lines()
        .find(|l| l.contains("air_group:8 \"VT-8\""))
        .expect("line");
    assert!(!line.contains("separate group"), "{line}");
}

#[test]
fn l3_a_bases_fuel_is_said_to_be_for_ships() {
    let b = made();
    let text = b.to_text();
    let line = text
        .lines()
        .find(|l| l.contains(": fuel ") || l.contains(" t, fuel "))
        .expect("a base line");
    assert!(
        line.contains(" t (for ships; aircraft do not use it)"),
        "{line}"
    );
}

#[test]
fn l5_the_air_attack_line_carries_the_sides_own_event_numbers() {
    let mut b = made();
    b.action.attacks_on_ships = vec![AttackOnShip {
        by: "aircraft",
        against: handle("ship:1", "USS Enterprise"),
        weapon: None,
        attacks: 2,
        aircraft: Some(Dec(Fixed::from_int(21))),
        events: vec![43, 44],
    }];
    let text = b.to_text();
    assert!(
        text.contains(
            "Attack on ship:1 \"USS Enterprise\" from the air: 21 aircraft, 2 attacks [events 43, 44]"
        ),
        "{text}"
    );
}

#[test]
fn l8_wear_not_in_play_is_said_beside_fatigue_and_only_then() {
    let mut b = made();
    b.heading.rules_in_play.wear = false;
    b.heading.rules_in_play.fatigue = false;
    let text = b.to_text();
    assert!(
        text.contains("- Air groups tired: fatigue is not in play\n- Aircraft wear: not in play\n"),
        "{text}"
    );
    b.heading.rules_in_play.wear = true;
    assert!(!b.to_text().contains("Aircraft wear: not in play"));
}

#[test]
fn l12_a_system_hit_that_changed_nothing_reads_no_change() {
    let mut b = made();
    b.losses.ships_damaged = vec![DamagedShip {
        ship: handle("ship:1", "USS Enterprise"),
        hits: 5,
        flotation_after: None,
        fire_after: None,
        systems: vec![
            SystemChange {
                system: "flight_deck",
                before: "working",
                after: "degraded",
            },
            SystemChange {
                system: "main_battery",
                before: "working",
                after: "working",
            },
            SystemChange {
                system: "flight_deck",
                before: "out",
                after: "out",
            },
        ],
        events: vec![7],
    }];
    let text = b.to_text();
    assert!(
        text.contains(
            ", flight deck working to degraded, main battery hit, no change, flight deck hit, no change [event 7]"
        ),
        "{text}"
    );
}
