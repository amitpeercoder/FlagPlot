//! The measure of a deny intent (decision 0123, 4), over scenario 5b: the
//! allied carrier force searches and strikes the Japanese force at (9, 0).
//!
//! The contacts held in the waters are counted two ways that must agree: by
//! the measure, rebuilt from the side's events, and from the contacts the
//! side holds when the day is done (the scenario is one day long, so the
//! contacts held at its end are the contacts held in the waters at the end
//! of the day in force). The side that gave no deny intent has no measure,
//! and neither has an intent that is not a deny.

use crate::support;

use sim_agent::deny_measure;
use sim_core::ids::IntentId;
use sim_core::state::Side;
use support::{load_text, scenario_text};

const NAME: &str = "05b_strike.toml";

fn text_with(command: &str) -> String {
    format!(
        "{}\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_intent\"\nintent = 1\n{command}\n",
        scenario_text(NAME)
    )
}

#[test]
fn the_contacts_held_in_the_waters_are_those_the_side_holds_at_the_end_of_the_day() {
    let mut scenario = load_text(
        &text_with("intent_kind = \"deny\"\nhex = [9, 0]\ndistance = 2"),
        NAME,
        1942,
    );
    let view = scenario.world.side_view(Side::Allied);
    assert_eq!(
        deny_measure(&view, IntentId::new(1)),
        None,
        "not yet taken in"
    );
    scenario.next_day().expect("the day resolves");
    let view = scenario.world.side_view(Side::Allied);
    let m = deny_measure(&view, IntentId::new(1)).expect("a deny intent");
    assert_eq!(m.days_in_force, 1);
    let centre = sim_core::hex::Hex::new(9, 0);
    let held = view
        .contacts()
        .filter(|c| c.estimated_hex.distance(centre) <= 2)
        .count();
    assert_eq!(m.contacts_held as usize, held);
    assert_eq!(m.days_met, u32::from(held == 0));
    // No base was given the task of striking ships under it.
    assert_eq!(m.strikes_launched, 0);
    // The other side has no such intent.
    let japanese = scenario.world.side_view(Side::Japanese);
    assert_eq!(deny_measure(&japanese, IntentId::new(1)), None);
}

#[test]
fn waters_that_hold_no_contact_meet_every_day() {
    // Waters far from the enemy force: no contact is held there.
    let mut scenario = load_text(
        &text_with("intent_kind = \"deny\"\nhex = [0, 0]\ndistance = 1"),
        NAME,
        1942,
    );
    scenario.next_day().expect("the day resolves");
    let view = scenario.world.side_view(Side::Allied);
    let m = deny_measure(&view, IntentId::new(1)).expect("a deny intent");
    assert_eq!((m.days_in_force, m.days_met, m.contacts_held), (1, 1, 0));
    assert_eq!((m.claimed_sunk, m.claimed_hits), (0, 0));
}

#[test]
fn an_intent_that_is_not_a_deny_has_no_deny_measure() {
    let mut scenario = load_text(
        &text_with("intent_kind = \"strike_ships\"\nbase = 1"),
        NAME,
        1942,
    );
    scenario.next_day().expect("the day resolves");
    let view = scenario.world.side_view(Side::Allied);
    assert_eq!(deny_measure(&view, IntentId::new(1)), None);
}
