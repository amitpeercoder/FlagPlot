//! Scenario 46: two settings of a strike group. Three groups at one base
//! differ only in the oldest contact they launch on; three at a second base
//! differ only in the share of their aircraft they hold back.
//!
//! Proves decision 0114 against a hand calculation. The data is placeholder
//! data: see `data/scenarios/46_two_strike_settings.toml`. It runs on the
//! frozen step 1 figures. There is no enemy unit at all: every strike flies
//! to a contact's hex, finds nothing there and rolls no die.
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
//! `strike_contact_max_age_pulses` 0. `contact_drop_age_pulses` 5.
//! `merge_distance_hexes` 2. The placeholder dive bomber (type 3): a strike
//! radius of 250 nm, a bomb. A hex is 40 nm across. A day is a night pulse
//! and two daylight pulses, morning and afternoon. At the end of every
//! pulse a contact not made or refreshed in it gains one pulse of age.
//! Strikes are decided in a daylight pulse, in group number order, on the
//! side's contacts as they stand. The upkeep, wear, fatigue and supply
//! figures are not loaded.
//!
//! ## Inputs
//!
//! A placeholder map of open sea with two Allied bases: base 1 at (0,0) and
//! base 2 at (20,0), 800 nm apart. The game is 1 day. There is no Japanese
//! unit.
//!
//! Six Allied groups of placeholder dive bombers, all on the naval strike
//! mission, with the default target list, on normal reach:
//!
//! | Group | Base | Ready aircraft | Oldest contact it launches on | Share held back |
//! | ----- | ---- | -------------- | ----------------------------- | --------------- |
//! | 1     | 1    | 10             | not set (the figure, 0)       | not set (0)     |
//! | 2     | 1    | 10             | 1                             | not set         |
//! | 3     | 1    | 10             | 3                             | not set         |
//! | 4     | 2    | 12             | 1                             | 0.333333        |
//! | 5     | 2    | 10             | 1                             | 0.25            |
//! | 6     | 2    | 10             | 1                             | not set         |
//!
//! The Allied side starts holding three contacts, in this order:
//!
//! | Contact | Hex    | Ships | Kind      | Age at the start | From base 1 | From base 2 |
//! | ------- | ------ | ----- | --------- | ---------------- | ----------- | ----------- |
//! | 1       | (3,0)  | 3     | transport | 0                | 120 nm      | 680 nm      |
//! | 2       | (0,4)  | 1     | carrier   | 2                | 160 nm      | out of reach |
//! | 3       | (22,0) | 2     | cruiser   | 0                | 880 nm      | 80 nm       |
//!
//! There is no order.
//!
//! ## The night
//!
//! Nothing is decided at night. At its end each contact gains a pulse:
//! contact 1 is 1 pulse old, contact 2 is 3, contact 3 is 1.
//!
//! ## The morning
//!
//! At base 1, in group order:
//!
//! - **Group 1** launches on a contact no older than 0. Contacts 1 and 2
//!   are in its reach, at ages 1 and 3. Neither qualifies. It does nothing
//!   and records nothing.
//! - **Group 2** launches on a contact no older than 1. Contact 1 qualifies
//!   and contact 2, at 3, does not. It launches its 10 aircraft at contact
//!   1, to (3,0), 120 nm.
//! - **Group 3** launches on a contact no older than 3. Both qualify. The
//!   carrier comes before the transport on its list: it launches its 10
//!   aircraft at contact 2, to (0,4), 160 nm.
//!
//! At base 2 only contact 3 is in reach, at age 1, and all three groups
//! launch on a contact no older than 1:
//!
//! - **Group 4** is able to fly 12 and holds back 0.333333: 12 x 0.666667 =
//!   8.000004, cut to 8. It launches 8 at contact 3, to (22,0), 80 nm.
//! - **Group 5** is able to fly 10 and holds back 0.25: 10 x 0.75 = 7.5,
//!   cut to 7. It launches 7.
//! - **Group 6** holds none back and launches 10.
//!
//! Each launch's breakdown gives the aircraft able to fly, the share held
//! and the aircraft sent: 10, 0, 10 for groups 2, 3 and 6; 12, 0.333333, 8
//! for group 4; 10, 0.25, 7 for group 5.
//!
//! In the air combat step there are three raids, one to each hex, and none
//! finds a ship. Each of the five groups records that its strike found
//! nothing. Contacts 1, 2 and 3 are dropped as not there, each once: for
//! contact 3 by group 4's strike, the first of its raid.
//!
//! ## The afternoon
//!
//! The Allied side holds no contact. No group launches and nothing is
//! recorded for any.
//!
//! ## At the end
//!
//! Five strikes were launched, with 10, 10, 8, 7 and 10 aircraft: 45 in
//! all. Group 1 launched none. Every group has the aircraft it began with,
//! ready: 10, 10, 10, 12, 10 and 10. The Allied side holds no contact.
//!
//! # Properties
//!
//! - With no setting on any group, no group launches on any pulse of the
//!   day: every contact is 1 pulse old or more in the morning. That is the
//!   game as it was before this rule.
//! - With every group's age set to 4, groups 1, 2 and 3 all launch at
//!   contact 2, the carrier, in the morning, 30 aircraft in one raid.
//! - An order for an age of 5, or of a negative number, is refused and
//!   names the range 0 to 4. An order for a share above 1 or below 0 is
//!   refused.
//! - A group that holds back all it has, a share of 1, does not launch and
//!   records nothing; so does a group able to fly 1 with a share of 0.5.
//! - With group 4 on a base strike at a Japanese base in its reach, it
//!   launches 8 of its 12.
//! - The held aircraft are not charged: with the wear, fatigue and supply
//!   figures loaded, group 4 is charged for 8 sorties and not for 12.
//! - The answer to the strike question for groups 1, 2 and 3 before the
//!   morning is none, contact 1 and contact 2.
//! - Every existing scenario's events and end states are what they were.
//
// # Contact numbers
//
// The comment's tables number the contacts in the order the scenario file
// enters them: 1 at (3,0), 2 at (0,4), 3 at (22,0). The engine numbers a
// starting contact oldest first (`start.rs`, `hold_for_side`), so the
// carrier of age 2 is contact 1, the transport contact 2 and the cruiser
// contact 3. The tests below name a contact by its hex, which is what
// the hand calculation fixes, and `the_numbers_the_comment_gives_to_the_contacts`
// (ignored) asserts the comment's own numbers.

mod common;

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::choices::Moment;
use sim_core::events::{Event, EventKind, SortieKind, SupplyUse};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::AirGroupId;
use sim_core::orders::{AirGroupOrder, OrderKind, OrderRequest, Origin, RejectReason};
use sim_core::state::Side;
use sim_core::time::Pulse;
use sim_data::Scenario;

// ------------------------------------------------------------------ Helpers

fn path() -> PathBuf {
    common::data_dir().join("scenarios/46_two_strike_settings.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

/// A decimal figure of the hand calculation, such as `"0.25"`.
fn d(text: &str) -> Fixed {
    text.parse().expect("a decimal figure")
}

/// The hexes of the three contacts, by the comment's numbers 1, 2 and 3.
const C1: Hex = Hex::new(3, 0);
const C2: Hex = Hex::new(0, 4);
const C3: Hex = Hex::new(22, 0);

fn group(id: u32) -> AirGroupId {
    AirGroupId::new(id)
}

/// What the side held at a moment, and what the strike question answered.
#[derive(Default, Clone, Debug)]
struct Look {
    /// (hex, age in pulses, contact number) of each contact held.
    contacts: Vec<(Hex, u32, u32)>,
    /// For groups 1 to 6, the hex the strike question answered, if any.
    choice: BTreeMap<u32, Option<Hex>>,
}

struct Run {
    events: Vec<Event>,
    ready: BTreeMap<u32, u32>,
    held_contacts_at_end: usize,
    /// The look before the morning's decision (after the night).
    morning: Look,
    /// The look at the start of the game, before anything is ordered.
    start: Look,
    refused: Vec<RejectReason>,
}

fn look(world: &sim_core::World) -> Look {
    let view = world.side_view(Side::Allied);
    let mut look = Look::default();
    for contact in view.contacts() {
        look.contacts.push((
            contact.estimated_hex,
            contact.age_pulses,
            contact.id.umpire_raw(common::umpire_key()),
        ));
    }
    for id in 1..=6 {
        look.choice
            .insert(id, view.strike_choice(group(id)).map(|c| c.hex));
    }
    look
}

impl Run {
    /// Runs the whole game with breakdowns recorded, which changes nothing
    /// that happens, and with `requests` handed in as orders for day 1.
    fn of(mut scenario: Scenario, requests: Vec<OrderRequest>) -> Run {
        scenario.world.set_config(sim_core::world::SimConfig {
            record_breakdowns: true,
        });
        if !requests.is_empty() {
            scenario.world.submit_orders(Side::Allied, requests);
        }
        let mut start = Look::default();
        let mut morning = Look::default();
        let day = scenario.world.day() + 1;
        scenario
            .next_day_watched(sim_core::Staffs::none(), &mut |world, moment| {
                if moment == (Moment::DayBegins { day }) {
                    start = look(world);
                }
                if moment
                    == (Moment::PulseBegins {
                        day,
                        pulse: Pulse::Morning,
                    })
                {
                    morning = look(world);
                }
            })
            .expect("the day resolves");
        let events: Vec<Event> = scenario.world.events().iter().cloned().collect();
        let refused = events
            .iter()
            .filter_map(|event| match &event.kind {
                EventKind::OrderRejected { reason, .. } => Some(reason.clone()),
                _ => None,
            })
            .collect();
        Run {
            ready: scenario
                .world
                .umpire()
                .air_groups
                .iter()
                .map(|(id, g)| (id.raw(), g.ready))
                .collect(),
            held_contacts_at_end: scenario.world.side_view(Side::Allied).contacts().count(),
            events,
            morning,
            start,
            refused,
        }
    }

    fn the_scenario() -> Run {
        Run::of(common::load(&path()).expect("the scenario loads"), vec![])
    }

    fn text(source: &str) -> Run {
        Run::of(
            common::from_text(source, &path()).expect("the variant loads"),
            vec![],
        )
    }

    /// The `StrikeLaunched` events of a pulse, as (group, contact hex,
    /// aircraft, distance nm), in log order.
    fn launches(&self, pulse: Pulse) -> Vec<(u32, Hex, u32, Fixed)> {
        self.in_pulse(pulse)
            .filter_map(|event| match event.kind {
                EventKind::StrikeLaunched {
                    air_group,
                    hex,
                    aircraft,
                    distance_nm,
                    ..
                } => Some((air_group.raw(), hex, aircraft, distance_nm)),
                _ => None,
            })
            .collect()
    }

    fn launch_events(&self, pulse: Pulse) -> Vec<&Event> {
        self.in_pulse(pulse)
            .filter(|event| matches!(event.kind, EventKind::StrikeLaunched { .. }))
            .collect()
    }

    fn base_launches(&self) -> Vec<(u32, u32, Hex, u32, Fixed, &Event)> {
        self.events
            .iter()
            .filter_map(|event| match event.kind {
                EventKind::BaseStrikeLaunched {
                    air_group,
                    base,
                    hex,
                    aircraft,
                    distance_nm,
                } => Some((
                    air_group.raw(),
                    base.raw(),
                    hex,
                    aircraft,
                    distance_nm,
                    event,
                )),
                _ => None,
            })
            .collect()
    }

    fn in_pulse(&self, pulse: Pulse) -> impl Iterator<Item = &Event> {
        self.events
            .iter()
            .filter(move |event| event.day == 1 && event.pulse == Some(pulse))
    }

    /// (group, hex) of each strike that found nothing, in log order.
    fn found_nothing(&self) -> Vec<(u32, Hex)> {
        self.events
            .iter()
            .filter_map(|event| match event.kind {
                EventKind::StrikeFoundNothing { air_group, hex } => Some((air_group.raw(), hex)),
                _ => None,
            })
            .collect()
    }

    /// (pulse, contact number, the event that caused it) of each contact
    /// dropped as not there.
    fn not_there(&self) -> Vec<(Option<Pulse>, u32, Hex)> {
        self.events
            .iter()
            .filter_map(|event| match event.kind {
                EventKind::ContactNotThere { contact, hex, .. } => {
                    Some((event.pulse, contact.umpire_raw(common::umpire_key()), hex))
                }
                _ => None,
            })
            .collect()
    }

    fn all_launched(&self) -> u32 {
        self.events
            .iter()
            .map(|event| match event.kind {
                EventKind::StrikeLaunched { aircraft, .. }
                | EventKind::BaseStrikeLaunched { aircraft, .. } => aircraft,
                _ => 0,
            })
            .sum()
    }

    fn number_at(&self, hex: Hex) -> u32 {
        self.morning
            .contacts
            .iter()
            .find(|(at, _, _)| *at == hex)
            .map(|(_, _, number)| *number)
            .unwrap_or_else(|| panic!("a contact at {hex}"))
    }
}

/// The three factors of a launch's breakdown: aircraft able to fly, the
/// share held back, and the result (the aircraft sent).
fn breakdown(event: &Event) -> (Fixed, Fixed, Fixed) {
    let breakdown = event.breakdown.as_ref().expect("a breakdown is recorded");
    let factor = |name: &str| {
        breakdown
            .factors
            .iter()
            .find(|factor| factor.name == name)
            .unwrap_or_else(|| panic!("factor {name}"))
            .value
    };
    assert_eq!(breakdown.factors.len(), 2);
    (
        factor("aircraft_able_to_fly"),
        factor("share_held_back"),
        breakdown.result,
    )
}

/// The scenario's text with every air group's setting lines taken out and,
/// if `age` is given, a contact age set on every group.
fn without_settings(age: Option<u32>) -> String {
    let mut out = String::new();
    for line in text().lines() {
        if line.starts_with("strike_contact_age_pulses") || line.starts_with("strike_held_share") {
            continue;
        }
        out.push_str(line);
        out.push('\n');
        if let Some(age) = age {
            if line.starts_with("ready = ") {
                out.push_str(&format!("strike_contact_age_pulses = {age}\n"));
            }
        }
    }
    out
}

fn age_request(group_id: u32, pulses: i64) -> OrderRequest {
    OrderRequest {
        kind: OrderKind::AirGroup {
            target: group(group_id),
            order: AirGroupOrder::SetStrikeContactAge { pulses },
        },
        origin: Origin::PLAYER,
    }
}

fn share_request(group_id: u32, share: &str) -> OrderRequest {
    OrderRequest {
        kind: OrderKind::AirGroup {
            target: group(group_id),
            order: AirGroupOrder::SetStrikeHeldShare { share: d(share) },
        },
        origin: Origin::PLAYER,
    }
}

// ------------------------------------------------------------------- Tests

#[test]
fn the_scenario_is_placeholder_data_with_one_day_and_no_japanese_unit() {
    let scenario = common::load(&path()).expect("the scenario loads");
    assert!(scenario.placeholder);
    assert_eq!(scenario.days, 1);
    let truth = scenario.world.umpire();
    assert!(truth.ships.is_empty());
    assert!(truth.task_forces.is_empty());
    assert!(truth
        .air_groups
        .values()
        .all(|group| group.side == Side::Allied));
    assert_eq!(truth.air_groups.len(), 6);
    // The ready aircraft and the settings of the table.
    let table: [(u32, u32, Option<u32>, &str); 6] = [
        (1, 10, None, "0"),
        (2, 10, Some(1), "0"),
        (3, 10, Some(3), "0"),
        (4, 12, Some(1), "0.333333"),
        (5, 10, Some(1), "0.25"),
        (6, 10, Some(1), "0"),
    ];
    for (id, ready, age, share) in table {
        let air_group = &truth.air_groups[&group(id)];
        assert_eq!(air_group.ready, ready, "group {id}");
        assert_eq!(air_group.strike_contact_age, age, "group {id}");
        assert_eq!(air_group.strike_held_share, d(share), "group {id}");
        assert_eq!(air_group.aircraft_type.raw(), 3, "group {id}");
    }
    // The start: three contacts, of the ages of the table (carrier 2, the
    // others 0) before the night ages them.
    let mut start: Vec<(Hex, u32)> = Run::the_scenario()
        .start
        .contacts
        .iter()
        .map(|(hex, age, _)| (*hex, *age))
        .collect();
    start.sort();
    assert_eq!(start.len(), 3);
}

#[test]
fn after_the_night_the_contacts_are_1_3_and_1_pulses_old() {
    let run = Run::the_scenario();
    let mut ages: Vec<(Hex, u32)> = run
        .morning
        .contacts
        .iter()
        .map(|(hex, age, _)| (*hex, *age))
        .collect();
    ages.sort_by_key(|(hex, _)| (hex.q, hex.r));
    // Contact 2 of the comment (0,4) is 3 pulses old; contacts 1 and 3, 1.
    assert_eq!(ages, vec![(C2, 3), (C1, 1), (C3, 1)]);
    // The night's ageing: three events, one for each contact (the starting
    // ageing of the carrier's two pulses is of day 0).
    let aged: Vec<_> = run
        .in_pulse(Pulse::Night)
        .filter(|event| matches!(event.kind, EventKind::ContactAged { .. }))
        .collect();
    assert_eq!(aged.len(), 3);
}

#[test]
fn the_morning_launches_are_groups_2_to_6_at_the_contacts_of_the_table() {
    let run = Run::the_scenario();
    assert_eq!(
        run.launches(Pulse::Morning),
        vec![
            (2, C1, 10, n(120)),
            (3, C2, 10, n(160)),
            (4, C3, 8, n(80)),
            (5, C3, 7, n(80)),
            (6, C3, 10, n(80)),
        ]
    );
    // The contacts they name are the contacts at those hexes.
    let ids: Vec<(u32, u32)> = run
        .launch_events(Pulse::Morning)
        .iter()
        .map(|event| match event.kind {
            EventKind::StrikeLaunched {
                air_group, contact, ..
            } => (air_group.raw(), contact.umpire_raw(common::umpire_key())),
            _ => unreachable!(),
        })
        .collect();
    assert_eq!(
        ids,
        vec![
            (2, run.number_at(C1)),
            (3, run.number_at(C2)),
            (4, run.number_at(C3)),
            (5, run.number_at(C3)),
            (6, run.number_at(C3)),
        ]
    );
    // Group 1 records nothing at all: no launch, no found-nothing, no
    // breakdown, no event that names it but its mission order.
    assert!(run.events.iter().all(|event| match event.kind {
        EventKind::StrikeLaunched { air_group, .. }
        | EventKind::StrikeFoundNothing { air_group, .. } => air_group.raw() != 1,
        _ => true,
    }));
}

#[test]
#[ignore = "the comment numbers the contacts in file order; the engine numbers a starting contact oldest first, so the run gives contact 1 at (0,4), 2 at (3,0), 3 at (22,0)"]
fn the_numbers_the_comment_gives_to_the_contacts() {
    let run = Run::the_scenario();
    assert_eq!(run.number_at(C1), 1);
    assert_eq!(run.number_at(C2), 2);
    assert_eq!(run.number_at(C3), 3);
}

#[test]
fn the_numbers_the_engine_gives_to_the_contacts_are_oldest_first() {
    let run = Run::the_scenario();
    assert_eq!(
        (run.number_at(C2), run.number_at(C1), run.number_at(C3)),
        (1, 2, 3)
    );
}

#[test]
fn each_launchs_breakdown_gives_the_aircraft_able_the_share_and_the_sent() {
    let run = Run::the_scenario();
    let events = run.launch_events(Pulse::Morning);
    let got: Vec<(Fixed, Fixed, Fixed)> = events.iter().map(|e| breakdown(e)).collect();
    assert_eq!(
        got,
        vec![
            (n(10), n(0), n(10)),
            (n(10), n(0), n(10)),
            (n(12), d("0.333333"), n(8)),
            (n(10), d("0.25"), n(7)),
            (n(10), n(0), n(10)),
        ]
    );
}

#[test]
fn five_strikes_find_nothing_one_for_each_of_groups_2_to_6() {
    let run = Run::the_scenario();
    assert_eq!(
        run.found_nothing(),
        vec![(2, C1), (3, C2), (4, C3), (5, C3), (6, C3)]
    );
    // None of them rolled a die: no attack, no loss, no fire of any kind.
    assert!(run.events.iter().all(|event| !matches!(
        event.kind,
        EventKind::AirAttack { .. }
            | EventKind::AntiAircraftFire { .. }
            | EventKind::BaseAttacked { .. }
    )));
}

#[test]
fn contacts_1_2_and_3_are_dropped_once_each_contact_3_by_group_4s_strike() {
    let run = Run::the_scenario();
    let dropped = run.not_there();
    assert_eq!(dropped.len(), 3);
    assert!(dropped
        .iter()
        .all(|(pulse, _, _)| *pulse == Some(Pulse::Morning)));
    let mut hexes: Vec<Hex> = dropped.iter().map(|(_, _, hex)| *hex).collect();
    hexes.sort_by_key(|hex| (hex.q, hex.r));
    assert_eq!(hexes, vec![C2, C1, C3]);
    let mut numbers: Vec<u32> = dropped.iter().map(|(_, number, _)| *number).collect();
    numbers.sort();
    assert_eq!(numbers, vec![1, 2, 3]);
    // Contact 3 is dropped once, by the first group of its raid, group 4:
    // the event that caused it is group 4's found-nothing event.
    let found_by_4 = run
        .events
        .iter()
        .find(|event| {
            matches!(event.kind, EventKind::StrikeFoundNothing { air_group, .. } if air_group.raw() == 4)
        })
        .expect("group 4 found nothing");
    let drop_of_3 = run
        .events
        .iter()
        .find(|event| matches!(event.kind, EventKind::ContactNotThere { hex, .. } if hex == C3))
        .expect("contact 3 is dropped");
    assert!(drop_of_3
        .causes
        .iter()
        .any(|cause| format!("{cause:?}").contains(&format!("{:?}", found_by_4.id))));
    // And the contact is no longer held.
    assert_eq!(run.held_contacts_at_end, 0);
}

#[test]
fn nothing_is_decided_at_night_or_launched_in_the_afternoon() {
    let run = Run::the_scenario();
    assert!(run.launches(Pulse::Night).is_empty());
    assert!(run.launches(Pulse::Afternoon).is_empty());
    assert!(run.in_pulse(Pulse::Afternoon).all(|event| !matches!(
        event.kind,
        EventKind::StrikeLaunched { .. }
            | EventKind::StrikeFoundNothing { .. }
            | EventKind::BaseStrikeLaunched { .. }
    )));
    assert!(run.base_launches().is_empty());
}

#[test]
fn at_the_end_45_aircraft_were_launched_every_group_has_its_aircraft_and_no_contact_is_held() {
    let run = Run::the_scenario();
    assert_eq!(run.all_launched(), 45);
    assert_eq!(
        run.ready,
        BTreeMap::from([(1, 10), (2, 10), (3, 10), (4, 12), (5, 10), (6, 10)])
    );
    assert_eq!(run.held_contacts_at_end, 0);
    assert!(run.refused.is_empty());
}

// ---------------------------------------------------------------- Properties

#[test]
fn with_no_setting_no_group_launches_all_day() {
    let run = Run::text(&without_settings(None));
    assert!(run.launches(Pulse::Morning).is_empty());
    assert!(run.launches(Pulse::Afternoon).is_empty());
    assert_eq!(run.all_launched(), 0);
    assert!(run.found_nothing().is_empty());
    assert!(run
        .events
        .iter()
        .all(|event| !matches!(event.kind, EventKind::StrikeLaunched { .. })));
    // Every contact is 1 pulse old or more in the morning.
    assert!(run.morning.contacts.iter().all(|(_, age, _)| *age >= 1));
}

#[test]
fn with_every_age_set_to_4_groups_1_2_and_3_launch_at_contact_2_30_aircraft_in_one_raid() {
    let run = Run::text(&without_settings(Some(4)));
    // Contact 2 of the comment is the carrier, at (0,4).
    let morning = run.launches(Pulse::Morning);
    let from_base_1: Vec<_> = morning.iter().filter(|l| l.0 <= 3).collect();
    assert_eq!(
        from_base_1,
        vec![
            &(1, C2, 10, n(160)),
            &(2, C2, 10, n(160)),
            &(3, C2, 10, n(160)),
        ]
    );
    let aircraft: u32 = from_base_1.iter().map(|l| l.2).sum();
    assert_eq!(aircraft, 30);
    // One raid: one place, one hex, so one found-nothing for the hex of each
    // group and one contact dropped.
    let at_c2: Vec<_> = run
        .found_nothing()
        .into_iter()
        .filter(|(_, hex)| *hex == C2)
        .collect();
    assert_eq!(at_c2.len(), 3);
    let drops_at_c2: Vec<_> = run
        .not_there()
        .into_iter()
        .filter(|(_, _, hex)| *hex == C2)
        .collect();
    assert_eq!(drops_at_c2.len(), 1);
}

#[test]
fn an_age_of_5_or_a_negative_age_is_refused_naming_the_range_0_to_4() {
    let scenario = common::load(&path()).expect("the scenario loads");
    for pulses in [5, -1] {
        let refused = scenario
            .world
            .check_order(Side::Allied, &age_request(2, pulses));
        assert_eq!(
            refused,
            Err(RejectReason::StrikeContactAgeOutOfRange {
                pulses,
                max: Some(4)
            })
        );
        let message = refused.unwrap_err().to_string();
        assert!(message.contains("0 and 4"), "{message}");
    }
    assert_eq!(
        scenario.world.check_order(Side::Allied, &age_request(2, 4)),
        Ok(())
    );
    assert_eq!(
        scenario.world.check_order(Side::Allied, &age_request(2, 0)),
        Ok(())
    );
    // Refused when the day begins as well, and the group keeps its setting.
    let run_scenario = common::load(&path()).expect("the scenario loads");
    let mut scenario = run_scenario;
    scenario
        .world
        .submit_orders(Side::Allied, vec![age_request(2, 5), age_request(3, -1)]);
    scenario.run_to_end().expect("the run resolves");
    let reasons: Vec<RejectReason> = scenario
        .world
        .events()
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::OrderRejected { reason, .. } => Some(reason.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(
        reasons,
        vec![
            RejectReason::StrikeContactAgeOutOfRange {
                pulses: 5,
                max: Some(4)
            },
            RejectReason::StrikeContactAgeOutOfRange {
                pulses: -1,
                max: Some(4)
            },
        ]
    );
    let truth = scenario.world.umpire();
    assert_eq!(truth.air_groups[&group(2)].strike_contact_age, Some(1));
    assert_eq!(truth.air_groups[&group(3)].strike_contact_age, Some(3));
}

#[test]
fn a_share_above_1_or_below_0_is_refused() {
    let scenario = common::load(&path()).expect("the scenario loads");
    for share in ["1.000001", "2", "-0.1", "-1"] {
        assert_eq!(
            scenario
                .world
                .check_order(Side::Allied, &share_request(4, share)),
            Err(RejectReason::StrikeHeldShareOutOfRange(d(share))),
            "share {share}"
        );
    }
    for share in ["0", "0.5", "1"] {
        assert_eq!(
            scenario
                .world
                .check_order(Side::Allied, &share_request(4, share)),
            Ok(()),
            "share {share}"
        );
    }
}

#[test]
fn a_share_of_1_holds_back_everything_and_a_group_able_to_fly_1_with_a_share_of_half_does_not_launch(
) {
    // Every group holds back all it has, and launches on a contact of any
    // age.
    let text = without_settings(None);
    let mut with_share = String::new();
    for line in text.lines() {
        with_share.push_str(line);
        with_share.push('\n');
        if line.starts_with("ready = ") {
            with_share.push_str("strike_contact_age_pulses = 4\nstrike_held_share = \"1\"\n");
        }
    }
    let run = Run::text(&with_share);
    assert_eq!(run.all_launched(), 0);
    assert!(run.events.iter().all(|event| !matches!(
        event.kind,
        EventKind::StrikeLaunched { .. }
            | EventKind::StrikeFoundNothing { .. }
            | EventKind::ContactNotThere { .. }
    )));
    assert!(run.base_launches().is_empty());
    // Nothing is recorded: no contact is dropped as not there.
    // The carrier, 2 pulses old at the start, reaches the drop age of 5 at
    // the end of the afternoon and is forgotten by age; the other two, no
    // strike having flown, are held.
    assert_eq!(run.held_contacts_at_end, 2);

    // A group able to fly 1 with a share of 0.5: 1 x 0.5 = 0.5, cut to 0.
    let mut one = String::new();
    for line in without_settings(None).lines() {
        if line == "ready = 10" || line == "ready = 12" {
            one.push_str("ready = 1\nstrike_contact_age_pulses = 4\nstrike_held_share = \"0.5\"\n");
        } else {
            one.push_str(line);
            one.push('\n');
        }
    }
    let run = Run::text(&one);
    assert_eq!(run.all_launched(), 0);
    assert!(run.found_nothing().is_empty());
    // The carrier, 2 pulses old at the start, reaches the drop age of 5 at
    // the end of the afternoon and is forgotten by age; the other two, no
    // strike having flown, are held.
    assert_eq!(run.held_contacts_at_end, 2);
}

/// The scenario with group 4 on a base strike at a Japanese base that is in
/// its reach: base 3, at (22,0), 80 nm from airfield 2. It has no aircraft.
fn with_a_base_strike_by_group_4() -> String {
    let source = text();
    let source = common::replace_once(
        &source,
        "air_group = 4\nkind = \"set_air_mission\"\nmission = \"naval_strike\"",
        "air_group = 4\nkind = \"set_air_mission\"\nmission = \"base_strike\"\ntarget_base = 3",
    );
    format!(
        "{source}\n[[base]]\nid = 3\nname = \"Placeholder airfield 3\"\nside = \"japanese\"\nq = 22\nr = 0\nsupply_t = 1000\n"
    )
}

#[test]
fn group_4_on_a_base_strike_launches_8_of_its_12() {
    let run = Run::text(&with_a_base_strike_by_group_4());
    // The launch event alone: the raid on a base rolls dice, so what comes
    // of it is not asserted here.
    let launches = run.base_launches();
    assert_eq!(launches.len(), 1);
    let (air_group, base, hex, aircraft, distance, event) = &launches[0];
    assert_eq!(
        (*air_group, *base, *hex, *aircraft, *distance),
        (4, 3, C3, 8, n(80))
    );
    assert_eq!(breakdown(event), (n(12), d("0.333333"), n(8)));
    // The naval groups of base 2 are as before.
    assert_eq!(
        run.launches(Pulse::Morning)
            .into_iter()
            .filter(|l| l.0 >= 5)
            .collect::<Vec<_>>(),
        vec![(5, C3, 7, n(80)), (6, C3, 10, n(80))]
    );
}

#[test]
fn with_the_wear_fatigue_and_supply_figures_loaded_group_4_is_charged_for_8_sorties_not_12() {
    let source = text().replace(
        "air_combat = \"../reference/air_combat.toml\"\n",
        "air_combat = \"../reference/air_combat.toml\"\nair_upkeep = \"../reference/air_upkeep.toml\"\nfatigue = \"../reference/fatigue.toml\"\nair_supply = \"../reference/air_supply.toml\"\n",
    );
    let run = Run::text(&source);
    // The launches are those of the plain scenario.
    assert_eq!(
        run.launches(Pulse::Morning),
        vec![
            (2, C1, 10, n(120)),
            (3, C2, 10, n(160)),
            (4, C3, 8, n(80)),
            (5, C3, 7, n(80)),
            (6, C3, 10, n(80)),
        ]
    );
    // Wear: the sorties each group flew are rolled for, 8 for group 4.
    let worn: BTreeMap<u32, u32> = run
        .events
        .iter()
        .filter_map(|event| match event.kind {
            EventKind::AircraftWorn {
                air_group,
                sortie: SortieKind::Strike,
                aircraft,
                ..
            } => Some((air_group.raw(), aircraft)),
            _ => None,
        })
        .collect();
    assert_eq!(
        worn,
        BTreeMap::from([(2, 10), (3, 10), (4, 8), (5, 7), (6, 10)])
    );
    // Supply: base 2 pays for the sorties of the morning, 8 + 7 + 10 = 25
    // and not 12 + 7 + 10 = 29; base 1 pays for 10 + 10.
    let spent: BTreeMap<u32, u32> = run
        .events
        .iter()
        .filter_map(|event| match event.kind {
            EventKind::BaseSupplySpent {
                base,
                reason: SupplyUse::Sorties,
                aircraft,
                ..
            } if event.pulse == Some(Pulse::Morning) => Some((base.raw(), aircraft)),
            _ => None,
        })
        .collect();
    assert_eq!(spent, BTreeMap::from([(1, 20), (2, 25)]));
    // Fatigue: group 4 was charged for a sortie it flew (an event of its
    // own), and group 1, which flew none, was not.
    let fatigued: Vec<u32> = run
        .events
        .iter()
        .filter_map(|event| match event.kind {
            EventKind::AirGroupFatigue { air_group, .. } => Some(air_group.raw()),
            _ => None,
        })
        .collect();
    assert!(fatigued.contains(&4));
    assert!(!fatigued.contains(&1));
}

#[test]
fn the_strike_question_for_groups_1_2_and_3_before_the_morning_is_none_contact_1_and_contact_2() {
    let run = Run::the_scenario();
    // Asked after the night's ageing and before the morning's decision.
    // Contact 1 of the comment is at (3,0), contact 2 at (0,4).
    assert_eq!(run.morning.choice[&1], None);
    assert_eq!(run.morning.choice[&2], Some(C1));
    assert_eq!(run.morning.choice[&3], Some(C2));
    // The questions agree with what was launched.
    for (id, hex, _, _) in run.launches(Pulse::Morning) {
        assert_eq!(run.morning.choice[&id], Some(hex), "group {id}");
    }
}

#[test]
fn every_existing_scenarios_events_and_end_states_are_what_they_were() {
    // Checked by scripts/compare-scenarios.sh against main, not by a test:
    // the hash version is the only thing that changes. This test only holds
    // the place of that property in the list.
}
