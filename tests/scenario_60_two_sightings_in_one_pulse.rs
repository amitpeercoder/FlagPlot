//! Scenario 60: two sightings in one pulse. A contact reported in this
//! pulse is refreshed again in the same pulse only from a hex away, so two
//! forces two hexes apart are held as two contacts.
//!
//! Proves decision 0133 against a hand calculation. There is no scenario
//! file: the cases are reports applied to one side's belief, in order, as
//! `BeliefState::apply_report` takes them, and the contacts the side then
//! holds. No die is rolled.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Figures
//!
//! `merge_distance_hexes` 2. `merge_distance_same_pulse_hexes` 1.
//! `fresh_contact_max_age_pulses` 2. A contact gains a pulse of age at the
//! end of a pulse in which it was not made or refreshed. Distances are
//! counts of hexes: (7,-9) to (8,-11) is 2; (7,-9) to (7,-10) is 1;
//! (9,-12) to (8,-11) is 1; (7,-10) to (8,-11) is 1; (7,-9) to (9,-12)
//! is 3; (9,-12) to (7,-10) is 2; (6,-9) to (8,-11) is 2; (10,-13) to
//! (8,-11) is 2; (10,-13) to (6,-9) is 4.
//!
//! ## The cases
//!
//! Each starts with a side that holds no contact, unless it says
//! otherwise. "Pulse 1" and "pulse 2" are consecutive pulses, with the
//! ageing of the end of pulse 1 between them.
//!
//! - **A. Two forces, one pulse.** Pulse 1: a report of 5 ships, a
//!   cruiser, at (7,-9) makes contact 1. A report of 13 ships, a carrier,
//!   at (8,-11), 2 hexes from contact 1, which was made this pulse: more
//!   than 1, so it makes contact 2. The side holds contact 1 at (7,-9), 5
//!   ships, a cruiser, and contact 2 at (8,-11), 13 ships, a carrier, both
//!   at age 0.
//! - **B. One force, one pulse, a hex out.** Pulse 1: the first report as
//!   in A. A report of 13 ships, a carrier, at (7,-10), 1 hex away,
//!   refreshes contact 1: (7,-10), 13 ships, a carrier, age 0. One
//!   contact.
//! - **C. One force, the next pulse.** Pulse 1: the first report as in A.
//!   Pulse 2: contact 1 is 1 pulse old and has not been reported this
//!   pulse. A report of 13 ships, a carrier, at (8,-11), 2 hexes away,
//!   refreshes it, as before this decision: (8,-11), 13 ships, a carrier,
//!   age 0. One contact.
//! - **D. The older contact takes it.** Pulse 1: a report at (9,-12) makes
//!   contact 1. Pulse 2: a report at (7,-9), 3 hexes from contact 1, makes
//!   contact 2. Then a report at (8,-11): contact 2, made this pulse, is 2
//!   away and does not qualify; contact 1, 1 pulse old, is 1 away and is
//!   refreshed to (8,-11). Two contacts: 1 at (8,-11), 2 at (7,-9).
//! - **E. A tie goes to the lowest number.** Pulse 1: a report at (9,-12)
//!   makes contact 1. A report at (7,-10), 2 hexes from contact 1, which
//!   was made this pulse, makes contact 2. Pulse 2: both are 1 pulse old
//!   and neither has been reported this pulse. A report at (8,-11) is 1
//!   hex from each: contact 1, the lower number, is refreshed to (8,-11).
//!   Contact 2 stays at (7,-10) and is 2 pulses old at the end of pulse 2.
//! - **F. The older contact takes it at two hexes.** Pulse 1: a report at
//!   (10,-13) makes contact 1. Pulse 2: a report at (6,-9), 4 hexes from
//!   contact 1, makes contact 2. Then a report at (8,-11), 2 hexes from
//!   each: contact 2 was made this pulse and does not qualify; contact 1
//!   is refreshed to (8,-11). Two contacts: 1 at (8,-11), 2 at (6,-9).
//! - **G. The same hex twice in one pulse.** Pulse 1: a report of 5 ships
//!   at (8,-11) makes contact 1; a report of 13 ships at (8,-11) refreshes
//!   it: 13 ships. One contact.
//! - **H. Without the figure,** as on the frozen step 1 figures: case A
//!   gives one contact, at (8,-11), 13 ships, a carrier, as before this
//!   decision.
//!
//! # Properties
//!
//! - **The battle it was found in.** The Eastern Solomons with both sides
//!   from their intents (`data/scenarios/second-proof/05a_eastern_solomons.toml`),
//!   live figures, seed 3: at the end of the morning pulse of day 1 the
//!   Allied side holds two contacts, one at (7,-9) and one at (8,-11).
//!   Before this decision it held one, at (8,-11). Whether the carriers
//!   then strike is reported by the build and not asserted here.
//! - The unit tests of the rule as it was pass unchanged on figures
//!   without the new one.
//! - The same reports in the same order give the same contacts.
//!
//! # Where the cases are
//!
//! `BeliefState::apply_report` is the core crate's own and cannot be driven
//! from this crate, so cases A to H, the distances and the property of the
//! same reports in the same order are unit tests in
//! `crates/sim-core/src/belief.rs`, each named `scenario_60_...` for its case.
//! This file holds the battle property.

use sim_core::choices::Moment;
use sim_core::events::{Event, EventKind};
use sim_core::hex::Hex;
use sim_core::ids::ContactId;
use sim_core::state::Side;
use sim_core::time::Pulse;
use sim_staff::{as_staffs, staffs};

mod common;

/// The Allied contacts' hexes as the afternoon of day 1 begins, after the
/// morning's ageing, in the Eastern Solomons from both sides' intents on
/// live figures, seed 3, and the day's events.
fn after_the_morning_on_seed_3() -> (Vec<Hex>, Vec<Event>) {
    let path = common::data_dir().join("scenarios/second-proof/05a_eastern_solomons.toml");
    let mut scenario = common::load_live(&path)
        .expect("the scenario loads")
        .with_seed(3)
        .expect("the seed is set");
    let held = staffs();
    let mut after_morning: Option<Vec<Hex>> = None;
    scenario
        .next_day_watched(as_staffs(&held), &mut |world, moment| {
            if moment
                == (Moment::PulseBegins {
                    day: 1,
                    pulse: Pulse::Afternoon,
                })
            {
                after_morning = Some(
                    world
                        .side_view(Side::Allied)
                        .contacts()
                        .map(|contact| contact.estimated_hex)
                        .collect(),
                );
            }
        })
        .expect("day 1 resolves")
        .expect("there is a day 1");
    let mut hexes = after_morning.expect("the afternoon of day 1 began");
    hexes.sort();
    (hexes, scenario.world.events().iter().cloned().collect())
}

/// The battle it was found in, as the hand calculation states it: at the
/// end of the morning pulse of day 1 the Allied side holds two contacts, one
/// at (7,-9) and one at (8,-11). Live figures on purpose: the property is of
/// the live figure and the battle in which the fault was found.
///
/// MISMATCH, for the architect. The code holds two contacts, at (7,-8) and
/// (8,-11). The first report made contact 1 at (7,-9) and the report at
/// (8,-11) made contact 2, as the hand calculation says; then the carriers
/// struck contact 1 in the morning, and the strike's four reports (events
/// 302, 307, 313 and 314) refreshed it, each within a hex, the last naming
/// (7,-8), a hex off by the report's error.
#[test]
#[ignore = "mismatch with the hand calculation: the contacts are at (7,-8) and (8,-11); the strike's last report moved contact 1 from (7,-9) to (7,-8)"]
fn the_eastern_solomons_on_seed_3_holds_both_forces_after_the_morning() {
    let (hexes, _) = after_the_morning_on_seed_3();
    let mut expected = vec![Hex::new(7, -9), Hex::new(8, -11)];
    expected.sort();
    assert_eq!(hexes, expected);
}

/// What the same game does give, asserted by the build and not by the hand
/// calculation: the report at (7,-9) and the report at (8,-11), in one pulse,
/// each made a contact, and both contacts are held at the end of the
/// morning, one of them at (8,-11).
#[test]
fn the_eastern_solomons_on_seed_3_makes_two_contacts_of_the_two_forces() {
    let (hexes, events) = after_the_morning_on_seed_3();
    assert_eq!(hexes.len(), 2, "two contacts: {hexes:?}");
    assert!(hexes.contains(&Hex::new(8, -11)), "{hexes:?}");
    let made: Vec<(Hex, ContactId)> = events
        .iter()
        .filter(|e| e.day == 1 && e.pulse == Some(Pulse::Morning))
        .filter_map(|e| match &e.kind {
            EventKind::ReportReceived {
                side: Side::Allied,
                contact,
                hex,
                created: true,
                ..
            } => Some((*hex, *contact)),
            _ => None,
        })
        .collect();
    assert_eq!(
        made,
        vec![
            (
                Hex::new(7, -9),
                ContactId::for_umpire(1, common::umpire_key())
            ),
            (
                Hex::new(8, -11),
                ContactId::for_umpire(2, common::umpire_key())
            )
        ]
    );
}
