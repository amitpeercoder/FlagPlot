//! Scenario 49: a strike sized to its target. Eight groups at one base and
//! three contacts in reach: a sized group sends what the report calls for
//! and no more, and goes on to the next contact when one is served.
//!
//! Proves decision 0118 against a hand calculation. The data is placeholder
//! data: see `data/scenarios/49_a_strike_sized_to_its_target.toml`. It runs
//! on the frozen step 1 figures with the table of aircraft for each ship
//! added, as below. There is no enemy unit at all: every strike flies to a
//! contact's hex, finds nothing there and rolls no die.
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
//! As scenario 46: a hex is 40 nm across; the placeholder dive bomber
//! (type 3) has a strike radius of 250 nm; at the end of every pulse a
//! contact not made or refreshed in it gains one pulse of age; strikes are
//! decided in a daylight pulse, in group number order; a group chooses
//! among the contacts it could launch on by the kind first on its target
//! list, then the nearest; the upkeep, wear, fatigue and supply figures are
//! not loaded.
//!
//! Aircraft for each ship reported, in this scenario's own figures and not
//! the live ones: carrier 24, battleship 24, cruiser 12, destroyer 6,
//! transport 6, auxiliary 4.
//!
//! ## Inputs
//!
//! A placeholder map of open sea. Allied base 1 at (0,0). The game is 1
//! day. There is no Japanese unit.
//!
//! Eight Allied groups of placeholder dive bombers at base 1, all on the
//! naval strike mission, with the default target list, on normal reach,
//! each set to launch on a contact up to 3 pulses old:
//!
//! | Group | Ready aircraft | Strike sizing | Share held back |
//! | ----- | -------------- | ------------- | --------------- |
//! | 1     | 10             | on            | not set (0)     |
//! | 2     | 10             | not set (off) | not set         |
//! | 3     | 10             | on            | not set         |
//! | 4     | 12             | on            | 0.25            |
//! | 5     | 10             | on            | not set         |
//! | 6     | 10             | on            | not set         |
//! | 7     | 20             | on            | not set         |
//! | 8     | 10             | on            | not set         |
//!
//! The Allied side starts holding three contacts, numbered from the oldest
//! down:
//!
//! | Contact | Hex    | Ships | Kind      | Age at the start | From base 1 | Need |
//! | ------- | ------ | ----- | --------- | ---------------- | ----------- | ---- |
//! | 1       | (0,4)  | 1     | carrier   | 2                | 160 nm      | 1 x 24 = 24 |
//! | 2       | (3,0)  | 2     | cruiser   | 1                | 120 nm      | 2 x 12 = 24 |
//! | 3       | (-3,0) | 3     | transport | 0                | 120 nm      | 3 x 6 = 18  |
//!
//! ## The night
//!
//! Nothing is decided. At its end the contacts are 3, 2 and 1 pulses old.
//! All three are in every group's reach and no older than 3, so every
//! group could launch on each. Each group's order among them is contact 1,
//! the carrier, then contact 2, then contact 3.
//!
//! ## The morning
//!
//! | Group | Can send | Contact | Its need | Sent there before | It sends | Left on the ground |
//! | ----- | -------- | ------- | -------- | ----------------- | -------- | ------------------ |
//! | 1     | 10       | 1       | 24       | 0                 | 10       | 0 |
//! | 2     | 10       | 1       | not read | not read          | 10       | 0 |
//! | 3     | 10       | 1       | 24       | 20                | 4        | 6 |
//! | 4     | 9        | 2       | 24       | 0                 | 9        | 3 |
//! | 5     | 10       | 2       | 24       | 9                 | 10       | 0 |
//! | 6     | 10       | 2       | 24       | 19                | 5        | 5 |
//! | 7     | 20       | 3       | 18       | 0                 | 18       | 2 |
//! | 8     | 10       | none    |          |                   | 0        | 10 |
//!
//! - **Group 2** has sizing off: it launches all 10 at the first contact,
//!   as it does today. What it sends counts for the groups after it.
//! - **Group 3** finds 20 sent at contact 1 against a need of 24 and sends
//!   4.
//! - **Group 4** is able to fly 12 and holds back 0.25: 12 x 0.75 = 9. It
//!   finds contact 1 served, 24 against 24, and goes on to contact 2.
//! - **Group 6** finds 19 sent at contact 2 and sends 5.
//! - **Group 7** finds contacts 1 and 2 served, 24 against 24 each, and
//!   sends 18 of its 20 at contact 3.
//! - **Group 8** finds all three served. It does not launch and records
//!   nothing.
//!
//! Each sized launch's breakdown gives the aircraft able to fly, the share
//! held, the need, what had been sent and what it sent: 10, 0, 24, 0, 10
//! for group 1; 10, 0, 24, 20, 4 for group 3; 12, 0.25, 24, 0, 9 for group
//! 4; 10, 0, 24, 9, 10 for group 5; 10, 0, 24, 19, 5 for group 6; 20, 0,
//! 18, 0, 18 for group 7. Each is caused by its group's sizing order, with
//! what causes a launch today. Group 2's breakdown and causes are those of
//! scenario 46.
//!
//! In the air combat step there are three raids, one to each hex: 24
//! aircraft at (0,4), 24 at (3,0) and 18 at (-3,0). None finds a ship. Each
//! of the seven groups records that its strike found nothing. Contacts 1,
//! 2 and 3 are dropped as not there, each once, by the first group of its
//! raid: groups 1, 4 and 7.
//!
//! ## The afternoon
//!
//! The Allied side holds no contact. No group launches.
//!
//! ## At the end
//!
//! Seven strikes were launched, with 10, 10, 4, 9, 10, 5 and 18 aircraft:
//! 66 in all. Every group has the aircraft it began with, ready.
//!
//! # Properties
//!
//! - **The game as it was.** With sizing set on no group, every group
//!   launches all it can send at contact 1: 10, 10, 10, 9, 10, 10, 20 and
//!   10, 89 aircraft in one raid. The same with sizing on and no table of
//!   aircraft for each ship in the figures.
//! - **No figure, everything.** With no figure for a carrier, every group
//!   sends all it can at contact 1, which is never served: 89 aircraft in
//!   one raid, as with sizing off. With no figure for a transport and the
//!   others as given, groups 1 to 6 launch as in the main case, group 7
//!   sends all 20 at contact 3 and group 8 its 10: 78 aircraft.
//! - **An unsized group overshoots.** With group 3's sizing off it sends 10
//!   at contact 1, 30 there in all, and every other launch is unchanged: 72
//!   aircraft.
//! - **A wrong report sends the wrong size.** With contact 1 reporting 2
//!   carriers its need is 48: groups 1, 2 and 3 send 10 each, group 4 sends
//!   9 and group 5 sends 9, all at contact 1; group 6 sends 10 and group 7
//!   sends 14 at contact 2; group 8 sends 10 at contact 3. 82 aircraft: 48,
//!   24 and 10.
//! - **Too few is still sent.** With group 1 alone at the base it sends 10
//!   at contact 1, and its breakdown gives a need of 24 and 10 sent.
//! - **A base strike is not sized.** With group 4 on a base strike at a
//!   Japanese base in its reach, it launches 9.
//! - **Aircraft left on the ground are not charged.** With the wear,
//!   fatigue and supply figures loaded, group 3 is charged for 4 sorties
//!   and group 8 for none.
//! - **The strike question** for a group before the morning is answered as
//!   the decision will be made, the groups before it deciding first:
//!   contact 1 and 4 aircraft for group 3, contact 3 and 18 for group 7,
//!   none for group 8.
//! - An order to set sizing on a group that is not the side's is refused.
//! - Every existing scenario's events and end states are what they were.

mod common;

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::choices::Moment;
use sim_core::events::{Cause, Event, EventKind, SortieKind, SupplyUse};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::AirGroupId;
use sim_core::orders::{AirGroupOrder, AirMission, OrderKind, OrderRequest, Origin, RejectReason};
use sim_core::state::Side;
use sim_core::time::Pulse;
use sim_core::NavalStrike;
use sim_data::{LoadOptions, Scenario};

// ------------------------------------------------------------------ Helpers

fn path() -> PathBuf {
    common::data_dir().join("scenarios/49_a_strike_sized_to_its_target.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn d(text: &str) -> Fixed {
    text.parse().expect("a decimal figure")
}

fn group(id: u32) -> AirGroupId {
    AirGroupId::new(id)
}

/// The hexes of the three contacts, by the comment's numbers 1, 2 and 3.
const C1: Hex = Hex::new(0, 4);
const C2: Hex = Hex::new(3, 0);
const C3: Hex = Hex::new(-3, 0);

/// This scenario's own figures: the aircraft for each ship, every kind given.
const FULL_TABLE: &[(&str, u32)] = &[
    ("carrier", 24),
    ("battleship", 24),
    ("cruiser", 12),
    ("destroyer", 6),
    ("transport", 6),
    ("auxiliary", 4),
];

/// The frozen figures with a `[strike_aircraft_per_ship]` table of `kinds`
/// appended to `air_combat.toml`, and `extra` replacements in other files.
fn figures(tag: &str, kinds: &[(&str, u32)]) -> LoadOptions {
    common::altered_reference(tag, |dir| {
        common::edit_file(dir, "air_combat.toml", |mut text| {
            if !text.ends_with('\n') {
                text.push('\n');
            }
            for (kind, per_ship) in kinds {
                text.push_str(&format!("{kind} = {per_ship}\n"));
            }
            text
        });
    })
}

fn table_without(kind: &str) -> Vec<(&'static str, u32)> {
    FULL_TABLE
        .iter()
        .copied()
        .filter(|(name, _)| *name != kind)
        .collect()
}

/// What the side held and what the strike question answered at a moment.
#[derive(Default, Clone, Debug)]
struct Look {
    /// (hex, age in pulses, contact number) of each contact held.
    contacts: Vec<(Hex, u32, u32)>,
    /// For groups 1 to 8, the answer to the strike question.
    strike: BTreeMap<u32, Option<NavalStrike>>,
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
    for id in 1..=8 {
        look.strike.insert(id, view.naval_strike(group(id)));
    }
    look
}

/// How an order is named in a launch's causes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Why {
    Mission,
    Sizing,
    Other,
    Report,
}

struct Run {
    events: Vec<Event>,
    ready: BTreeMap<u32, u32>,
    held_contacts_at_end: usize,
    /// The look before the morning's decision (after the night).
    morning: Look,
    /// The look at the start of the game.
    start: Look,
    /// The kind of each order, by the group it is for.
    order_kinds: BTreeMap<u32, (u32, Why)>,
}

impl Run {
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
        let mut order_kinds = BTreeMap::new();
        for order in scenario.world.orders() {
            if let OrderKind::AirGroup {
                target,
                order: kind,
            } = &order.kind
            {
                let why = match kind {
                    AirGroupOrder::SetAirMission(_) => Why::Mission,
                    AirGroupOrder::SetStrikeSizing { .. } => Why::Sizing,
                    _ => Why::Other,
                };
                order_kinds.insert(
                    order.id.umpire_raw(common::umpire_key()),
                    (target.raw(), why),
                );
            }
        }
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
            order_kinds,
        }
    }

    fn the_scenario() -> Run {
        Run::of(
            common::from_text_with(&text(), &path(), &main_options()).expect("loads"),
            vec![],
        )
    }

    /// The scenario's text, with the full table of aircraft for each ship.
    fn text(source: &str) -> Run {
        Run::text_with(source, &main_options())
    }

    fn text_with(source: &str, options: &LoadOptions) -> Run {
        Run::of(
            common::from_text_with(source, &path(), options).expect("the variant loads"),
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

    /// (contact number, hex, the event that caused it) of each contact
    /// dropped as not there.
    fn not_there(&self) -> Vec<(u32, Hex, &Event)> {
        self.events
            .iter()
            .filter_map(|event| match event.kind {
                EventKind::ContactNotThere { contact, hex, .. } => {
                    Some((contact.umpire_raw(common::umpire_key()), hex, event))
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

    /// The aircraft launched in the morning, in all. The variants' totals
    /// of the calculation (89, 78, 72, 82) are the morning's: the contacts
    /// a raid did not serve are still held in the afternoon.
    fn morning_launched(&self) -> u32 {
        self.launches(Pulse::Morning).iter().map(|l| l.2).sum()
    }

    /// Aircraft launched at each hex in the morning, in all.
    fn raids(&self) -> BTreeMap<(i32, i32), u32> {
        let mut raids = BTreeMap::new();
        for (_, hex, aircraft, _) in self.launches(Pulse::Morning) {
            *raids.entry((hex.q, hex.r)).or_insert(0) += aircraft;
        }
        raids
    }

    /// (group, hex, aircraft) of the morning's launches.
    fn sent(&self) -> Vec<(u32, Hex, u32)> {
        self.launches(Pulse::Morning)
            .into_iter()
            .map(|(g, hex, aircraft, _)| (g, hex, aircraft))
            .collect()
    }

    /// What a launch's causes are, in order, named by the kind of each.
    fn causes_of(&self, event: &Event) -> Vec<Why> {
        event
            .causes
            .iter()
            .map(|cause| match cause {
                Cause::Order(id) => self
                    .order_kinds
                    .get(&id.umpire_raw(common::umpire_key()))
                    .map(|(_, why)| *why)
                    .expect("an air group order"),
                Cause::Event(_) => Why::Report,
            })
            .collect()
    }
}

fn main_options() -> LoadOptions {
    figures("s49", FULL_TABLE)
}

/// A launch's breakdown as (name, value) of each factor and the result.
fn breakdown(event: &Event) -> (Vec<(&'static str, Fixed)>, Fixed) {
    let breakdown = event.breakdown.as_ref().expect("a breakdown is recorded");
    (
        breakdown
            .factors
            .iter()
            .map(|factor| (factor.name, factor.value))
            .collect(),
        breakdown.result,
    )
}

/// The five figures of a sized launch: aircraft able to fly, share held,
/// need, sent before, and the aircraft sent.
fn sized(event: &Event) -> [Fixed; 5] {
    let (factors, result) = breakdown(event);
    let names: Vec<&str> = factors.iter().map(|(name, _)| *name).collect();
    assert_eq!(
        names,
        vec![
            "aircraft_able_to_fly",
            "share_held_back",
            "need_of_contact",
            "sent_before"
        ]
    );
    [
        factors[0].1,
        factors[1].1,
        factors[2].1,
        factors[3].1,
        result,
    ]
}

/// The two figures of an unsized launch, and its result.
fn unsized_launch(event: &Event) -> [Fixed; 3] {
    let (factors, result) = breakdown(event);
    let names: Vec<&str> = factors.iter().map(|(name, _)| *name).collect();
    assert_eq!(names, vec!["aircraft_able_to_fly", "share_held_back"]);
    [factors[0].1, factors[1].1, result]
}

/// The scenario's text with every air group in `keep` and no other: the
/// blocks of the others, and the orders for them, taken out.
fn only_groups(keep: &[u32]) -> String {
    let mut out = Vec::new();
    for (index, block) in text().split("\n[[").enumerate() {
        if index == 0 {
            out.push(block.to_string());
            continue;
        }
        let id_of = |key: &str| -> Option<u32> {
            block
                .lines()
                .find_map(|line| line.strip_prefix(key))
                .and_then(|rest| rest.trim().parse().ok())
        };
        let owner = if block.starts_with("air_group]]") {
            id_of("id = ")
        } else if block.starts_with("order]]") {
            id_of("air_group = ")
        } else {
            None
        };
        if owner.is_some_and(|id| !keep.contains(&id)) {
            continue;
        }
        out.push(format!("[[{block}"));
    }
    // The first block's "[[" was cut by the split too.
    let mut joined = out.join("\n");
    if !joined.ends_with('\n') {
        joined.push('\n');
    }
    joined.replacen("\n\n[[", "\n[[", 0)
}

fn sizing_off_for_all() -> String {
    let source = text();
    assert_eq!(
        source
            .matches("kind = \"set_strike_sizing\"\non = true")
            .count(),
        7
    );
    source.replace(
        "kind = \"set_strike_sizing\"\non = true",
        "kind = \"set_strike_sizing\"\non = false",
    )
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
    assert_eq!(truth.air_groups.len(), 8);
    // The ready aircraft, sizing and share of the table.
    let table: [(u32, u32, bool, &str); 8] = [
        (1, 10, true, "0"),
        (2, 10, false, "0"),
        (3, 10, true, "0"),
        (4, 12, true, "0.25"),
        (5, 10, true, "0"),
        (6, 10, true, "0"),
        (7, 20, true, "0"),
        (8, 10, true, "0"),
    ];
    for (id, ready, sizing, share) in table {
        let air_group = &truth.air_groups[&group(id)];
        assert_eq!(air_group.ready, ready, "group {id}");
        // The sizing orders of day 1 have not yet taken effect.
        let _ = sizing;
        assert!(!air_group.strike_sizing, "group {id}");
        assert_eq!(air_group.strike_held_share, d(share), "group {id}");
        assert_eq!(air_group.strike_contact_age, Some(3), "group {id}");
        assert_eq!(air_group.aircraft_type.raw(), 3, "group {id}");
    }
    // The start: three contacts of ages 2, 1 and 0, numbered oldest first.
    let mut start: Vec<(Hex, u32, u32)> = Run::the_scenario().start.contacts.clone();
    start.sort_by_key(|(_, _, number)| *number);
    assert_eq!(start, vec![(C1, 2, 1), (C2, 1, 2), (C3, 0, 3)]);
}

#[test]
fn after_the_night_the_contacts_are_3_2_and_1_pulses_old() {
    let run = Run::the_scenario();
    let mut contacts = run.morning.contacts.clone();
    contacts.sort_by_key(|(_, _, number)| *number);
    assert_eq!(contacts, vec![(C1, 3, 1), (C2, 2, 2), (C3, 1, 3)]);
    // Nothing is decided at night.
    assert!(run.launches(Pulse::Night).is_empty());
}

#[test]
fn the_morning_launches_are_those_of_the_table() {
    let run = Run::the_scenario();
    assert_eq!(
        run.launches(Pulse::Morning),
        vec![
            (1, C1, 10, n(160)),
            (2, C1, 10, n(160)),
            (3, C1, 4, n(160)),
            (4, C2, 9, n(120)),
            (5, C2, 10, n(120)),
            (6, C2, 5, n(120)),
            (7, C3, 18, n(120)),
        ]
    );
    // The contacts they name are contacts 1, 1, 1, 2, 2, 2 and 3.
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
        vec![(1, 1), (2, 1), (3, 1), (4, 2), (5, 2), (6, 2), (7, 3)]
    );
    // Group 8 records nothing at all.
    assert!(run.events.iter().all(|event| match event.kind {
        EventKind::StrikeLaunched { air_group, .. }
        | EventKind::StrikeFoundNothing { air_group, .. } => air_group.raw() != 8,
        _ => true,
    }));
}

#[test]
fn each_sized_launchs_breakdown_gives_able_share_need_sent_before_and_sent() {
    let run = Run::the_scenario();
    let events = run.launch_events(Pulse::Morning);
    assert_eq!(events.len(), 7);
    let table: [(usize, [Fixed; 5]); 6] = [
        (0, [n(10), n(0), n(24), n(0), n(10)]),
        (2, [n(10), n(0), n(24), n(20), n(4)]),
        (3, [n(12), d("0.25"), n(24), n(0), n(9)]),
        (4, [n(10), n(0), n(24), n(9), n(10)]),
        (5, [n(10), n(0), n(24), n(19), n(5)]),
        (6, [n(20), n(0), n(18), n(0), n(18)]),
    ];
    for (index, expected) in table {
        assert_eq!(sized(events[index]), expected, "event {index}");
    }
    // Group 2's breakdown is that of scenario 46: two factors and the result.
    assert_eq!(unsized_launch(events[1]), [n(10), n(0), n(10)]);
}

#[test]
fn each_launch_is_caused_by_its_mission_order_the_report_and_its_sizing_order() {
    let run = Run::the_scenario();
    let events = run.launch_events(Pulse::Morning);
    for event in &events {
        let EventKind::StrikeLaunched { air_group, .. } = event.kind else {
            unreachable!()
        };
        let whys = run.causes_of(event);
        // The mission order, the contact's report event, then the sizing
        // order, each only where the group has one.
        if air_group.raw() == 2 {
            assert_eq!(whys, vec![Why::Mission, Why::Report], "group 2");
        } else {
            assert_eq!(
                whys,
                vec![Why::Mission, Why::Report, Why::Sizing],
                "group {}",
                air_group.raw()
            );
        }
        // Each order is the group's own.
        for cause in &event.causes {
            if let Cause::Order(id) = cause {
                let (owner, _) = run.order_kinds[&id.umpire_raw(common::umpire_key())];
                assert_eq!(owner, air_group.raw());
            }
        }
    }
}

#[test]
fn seven_strikes_find_nothing_and_three_contacts_are_dropped_by_groups_1_4_and_7() {
    let run = Run::the_scenario();
    assert_eq!(
        run.found_nothing(),
        vec![
            (1, C1),
            (2, C1),
            (3, C1),
            (4, C2),
            (5, C2),
            (6, C2),
            (7, C3)
        ]
    );
    // Three raids: 24 aircraft at (0,4), 24 at (3,0) and 18 at (-3,0).
    assert_eq!(
        run.raids(),
        BTreeMap::from([((0, 4), 24), ((3, 0), 24), ((-3, 0), 18)])
    );
    // No die: no attack, no loss, no fire of any kind.
    assert!(run.events.iter().all(|event| !matches!(
        event.kind,
        EventKind::AirAttack { .. }
            | EventKind::AntiAircraftFire { .. }
            | EventKind::BaseAttacked { .. }
    )));
    // Contacts 1, 2 and 3 are dropped once each, in the morning, by the
    // first group of their raids: groups 1, 4 and 7.
    let dropped = run.not_there();
    assert_eq!(dropped.len(), 3);
    for (event_pulse, (number, hex, event)) in [Pulse::Morning; 3].iter().zip(dropped.iter()) {
        assert_eq!(event.pulse, Some(*event_pulse));
        let first_group = match number {
            1 => 1,
            2 => 4,
            3 => 7,
            other => panic!("contact {other}"),
        };
        assert_eq!(
            *hex,
            match number {
                1 => C1,
                2 => C2,
                _ => C3,
            }
        );
        let found = run
            .events
            .iter()
            .find(|e| {
                matches!(e.kind, EventKind::StrikeFoundNothing { air_group, .. } if air_group.raw() == first_group)
            })
            .expect("the first group of the raid found nothing");
        assert!(
            event.causes.contains(&Cause::Event(found.id)),
            "contact {number} is dropped by group {first_group}'s strike"
        );
    }
    let mut numbers: Vec<u32> = dropped.iter().map(|(number, _, _)| *number).collect();
    numbers.sort();
    assert_eq!(numbers, vec![1, 2, 3]);
}

#[test]
fn the_afternoon_has_no_contact_and_no_launch() {
    let run = Run::the_scenario();
    assert!(run.launches(Pulse::Afternoon).is_empty());
    assert!(run.in_pulse(Pulse::Afternoon).all(|event| !matches!(
        event.kind,
        EventKind::StrikeLaunched { .. }
            | EventKind::StrikeFoundNothing { .. }
            | EventKind::BaseStrikeLaunched { .. }
    )));
    assert!(run.base_launches().is_empty());
    assert_eq!(run.held_contacts_at_end, 0);
}

#[test]
fn at_the_end_66_aircraft_were_launched_and_every_group_has_its_aircraft_ready() {
    let run = Run::the_scenario();
    assert_eq!(run.all_launched(), 66);
    let sizes: Vec<u32> = run
        .sent()
        .iter()
        .map(|(_, _, aircraft)| *aircraft)
        .collect();
    assert_eq!(sizes, vec![10, 10, 4, 9, 10, 5, 18]);
    assert_eq!(
        run.ready,
        BTreeMap::from([
            (1, 10),
            (2, 10),
            (3, 10),
            (4, 12),
            (5, 10),
            (6, 10),
            (7, 20),
            (8, 10)
        ])
    );
    assert_eq!(run.held_contacts_at_end, 0);
}

// ---------------------------------------------------------------- Properties

const AS_IT_WAS: [u32; 8] = [10, 10, 10, 9, 10, 10, 20, 10];

fn assert_all_at_contact_1(run: &Run, sizes: &[u32]) {
    let expected: Vec<(u32, Hex, u32)> = sizes
        .iter()
        .enumerate()
        .map(|(index, aircraft)| (index as u32 + 1, C1, *aircraft))
        .collect();
    assert_eq!(run.sent(), expected);
    assert_eq!(
        run.raids(),
        BTreeMap::from([((0, 4), sizes.iter().sum::<u32>())])
    );
}

#[test]
fn the_game_as_it_was_with_sizing_on_no_group_every_group_launches_all_it_can_at_contact_1() {
    let run = Run::text(&sizing_off_for_all());
    assert_all_at_contact_1(&run, &AS_IT_WAS);
    assert_eq!(run.morning_launched(), 89);
    // Every launch is of the unsized kind: two factors.
    for event in run.launch_events(Pulse::Morning) {
        let [_, _, _] = unsized_launch(event);
    }
}

#[test]
fn the_game_as_it_was_with_sizing_on_and_no_table_of_aircraft_for_each_ship() {
    // The plain frozen figures: the table is empty.
    let scenario = common::from_text(&text(), &path()).expect("loads");
    let run = Run::of(scenario, vec![]);
    assert_all_at_contact_1(&run, &AS_IT_WAS);
    assert_eq!(run.morning_launched(), 89);
}

#[test]
fn with_no_figure_for_a_carrier_every_group_sends_all_it_can_at_contact_1() {
    let options = figures("s49_no_carrier", &table_without("carrier"));
    let run = Run::text_with(&text(), &options);
    assert_all_at_contact_1(&run, &AS_IT_WAS);
    assert_eq!(run.morning_launched(), 89);
    // The sized groups' breakdowns give no need of the contact.
    let events = run.launch_events(Pulse::Morning);
    let (factors, result) = breakdown(events[0]);
    let names: Vec<&str> = factors.iter().map(|(name, _)| *name).collect();
    assert_eq!(
        names,
        vec!["aircraft_able_to_fly", "share_held_back", "sent_before"]
    );
    assert_eq!(result, n(10));
}

#[test]
fn with_no_figure_for_a_transport_groups_1_to_6_are_as_before_and_groups_7_and_8_send_all() {
    let options = figures("s49_no_transport", &table_without("transport"));
    let run = Run::text_with(&text(), &options);
    assert_eq!(
        run.sent(),
        vec![
            (1, C1, 10),
            (2, C1, 10),
            (3, C1, 4),
            (4, C2, 9),
            (5, C2, 10),
            (6, C2, 5),
            (7, C3, 20),
            (8, C3, 10),
        ]
    );
    assert_eq!(run.morning_launched(), 78);
}

#[test]
fn an_unsized_group_overshoots() {
    let source = common::replace_once(
        &text(),
        "air_group = 3\nkind = \"set_strike_sizing\"\non = true",
        "air_group = 3\nkind = \"set_strike_sizing\"\non = false",
    );
    let run = Run::text(&source);
    assert_eq!(
        run.sent(),
        vec![
            (1, C1, 10),
            (2, C1, 10),
            (3, C1, 10),
            (4, C2, 9),
            (5, C2, 10),
            (6, C2, 5),
            (7, C3, 18),
        ]
    );
    // 30 at contact 1 in all, and 72 aircraft.
    assert_eq!(run.raids()[&(0, 4)], 30);
    assert_eq!(run.morning_launched(), 72);
}

#[test]
fn a_wrong_report_sends_the_wrong_size() {
    let source = common::replace_once(
        &text(),
        "ship_count = 1\nkind = \"carrier\"",
        "ship_count = 2\nkind = \"carrier\"",
    );
    let run = Run::text(&source);
    assert_eq!(
        run.sent(),
        vec![
            (1, C1, 10),
            (2, C1, 10),
            (3, C1, 10),
            (4, C1, 9),
            (5, C1, 9),
            (6, C2, 10),
            (7, C2, 14),
            (8, C3, 10),
        ]
    );
    assert_eq!(run.morning_launched(), 82);
    assert_eq!(
        run.raids(),
        BTreeMap::from([((0, 4), 48), ((3, 0), 24), ((-3, 0), 10)])
    );
}

#[test]
fn too_few_is_still_sent_group_1_alone_sends_10_at_contact_1() {
    let run = Run::text(&only_groups(&[1]));
    assert_eq!(run.sent(), vec![(1, C1, 10)]);
    let events = run.launch_events(Pulse::Morning);
    assert_eq!(events.len(), 1);
    assert_eq!(sized(events[0]), [n(10), n(0), n(24), n(0), n(10)]);
}

#[test]
fn a_base_strike_is_not_sized() {
    // A Japanese base in group 4's reach, 120 nm from base 1.
    let source = common::replace_once(
        &text(),
        "air_group = 4\nkind = \"set_air_mission\"\nmission = \"naval_strike\"",
        "air_group = 4\nkind = \"set_air_mission\"\nmission = \"base_strike\"\ntarget_base = 2",
    );
    let source = format!(
        "{source}\n[[base]]\nid = 2\nname = \"Placeholder airfield 2\"\nside = \"japanese\"\nq = 3\nr = 0\nsupply_t = 1000\n"
    );
    let run = Run::text(&source);
    let launches = run.base_launches();
    assert_eq!(launches.len(), 1);
    let (air_group, base, hex, aircraft, distance, event) = &launches[0];
    assert_eq!(
        (*air_group, *base, *hex, *aircraft, *distance),
        (4, 2, C2, 9, n(120))
    );
    // Not sized: two factors, able to fly and the share held.
    assert_eq!(unsized_launch(event), [n(12), d("0.25"), n(9)]);
}

#[test]
fn aircraft_left_on_the_ground_are_not_charged_group_3_for_4_sorties_and_group_8_for_none() {
    let source = text().replace(
        "air_combat = \"../reference/air_combat.toml\"\n",
        "air_combat = \"../reference/air_combat.toml\"\nair_upkeep = \"../reference/air_upkeep.toml\"\nfatigue = \"../reference/fatigue.toml\"\nair_supply = \"../reference/air_supply.toml\"\n",
    );
    let run = Run::text(&source);
    // The launches are those of the plain scenario.
    assert_eq!(
        run.sent(),
        vec![
            (1, C1, 10),
            (2, C1, 10),
            (3, C1, 4),
            (4, C2, 9),
            (5, C2, 10),
            (6, C2, 5),
            (7, C3, 18),
        ]
    );
    // Wear: the sorties each group flew are rolled for.
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
        BTreeMap::from([(1, 10), (2, 10), (3, 4), (4, 9), (5, 10), (6, 5), (7, 18)])
    );
    assert_eq!(worn[&3], 4);
    assert!(!worn.contains_key(&8));
    // Supply: the base pays for the 66 sorties of the morning.
    let spent: u32 = run
        .events
        .iter()
        .filter_map(|event| match event.kind {
            EventKind::BaseSupplySpent {
                reason: SupplyUse::Sorties,
                aircraft,
                ..
            } if event.pulse == Some(Pulse::Morning) => Some(aircraft),
            _ => None,
        })
        .sum();
    assert_eq!(spent, 66);
    // Fatigue: group 3 was charged for a sortie it flew, group 8 was not.
    let fatigued: Vec<u32> = run
        .events
        .iter()
        .filter_map(|event| match event.kind {
            EventKind::AirGroupFatigue { air_group, .. } => Some(air_group.raw()),
            _ => None,
        })
        .collect();
    assert!(fatigued.contains(&3));
    assert!(!fatigued.contains(&8));
}

#[test]
fn the_strike_question_before_the_morning_is_answered_as_the_decision_will_be_made() {
    let run = Run::the_scenario();
    let answer = |id: u32| run.morning.strike[&id];
    // Group 3: contact 1, 4 aircraft.
    let g3 = answer(3).expect("group 3 has a strike");
    assert_eq!((g3.choice.hex, g3.aircraft), (C1, 4));
    assert_eq!(g3.choice.contact.umpire_raw(common::umpire_key()), 1);
    let sizing = g3.sizing.expect("group 3 is sized");
    assert_eq!((sizing.need, sizing.sent_before), (Some(24), 20));
    // Group 7: contact 3, 18 aircraft.
    let g7 = answer(7).expect("group 7 has a strike");
    assert_eq!((g7.choice.hex, g7.aircraft), (C3, 18));
    assert_eq!(g7.choice.contact.umpire_raw(common::umpire_key()), 3);
    let sizing = g7.sizing.expect("group 7 is sized");
    assert_eq!((sizing.need, sizing.sent_before), (Some(18), 0));
    // Group 8: none.
    assert_eq!(answer(8), None);
    // The question agrees with each launch, group 2 being unsized.
    for (id, hex, aircraft, _) in run.launches(Pulse::Morning) {
        let strike = answer(id).expect("a strike");
        assert_eq!(
            (strike.choice.hex, strike.aircraft),
            (hex, aircraft),
            "group {id}"
        );
        assert_eq!(strike.sizing.is_none(), id == 2, "group {id}");
    }
}

#[test]
fn an_order_to_set_sizing_on_a_group_that_is_not_the_sides_is_refused() {
    let scenario = common::load(&path()).expect("the scenario loads");
    let request = OrderRequest {
        kind: OrderKind::AirGroup {
            target: group(1),
            order: AirGroupOrder::SetStrikeSizing { on: true },
        },
        origin: Origin::PLAYER,
    };
    assert_eq!(
        scenario.world.check_order(Side::Japanese, &request),
        Err(RejectReason::NotYourUnit)
    );
    // And a group that does not exist is refused the same way.
    let nobody = OrderRequest {
        kind: OrderKind::AirGroup {
            target: group(99),
            order: AirGroupOrder::SetStrikeSizing { on: true },
        },
        origin: Origin::PLAYER,
    };
    assert_eq!(
        scenario.world.check_order(Side::Allied, &nobody),
        Err(RejectReason::NotYourUnit)
    );
    // Submitted at the day's start it is rejected, and group 1 keeps its
    // setting.
    let mut scenario = common::load(&path()).expect("the scenario loads");
    scenario.world.submit_orders(Side::Japanese, vec![request]);
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
    assert_eq!(reasons, vec![RejectReason::NotYourUnit]);
    assert!(scenario.world.umpire().air_groups[&group(1)].strike_sizing);
    // The mission is untouched, too.
    assert_eq!(
        scenario.world.umpire().air_groups[&group(1)].mission,
        AirMission::NavalStrike
    );
}

#[test]
fn every_existing_scenarios_events_and_end_states_are_what_they_were() {
    // Checked by scripts against main, not by a test: the hash version is
    // the only thing that changes. This test only holds the place of that
    // property in the list.
}
