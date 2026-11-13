//! Flight 300 by the rule (decision 11 of decision 0059, stage 3).
//!
//! # Hand calculation
//!
//! Written by the architect before the test code. The tests below assert
//! these numbers. If the code disagrees, the code or the rule is wrong: do
//! not change a number here to make a test pass.
//!
//! On 24 August 1942 eleven dive bombers of Enterprise, in the air when she
//! was hit, landed at Henderson Field and stayed. Scenario 6, which has no
//! carriers, scripts that as an arrival. The full slice has Enterprise in it,
//! and since stage 2 of decision 0059 the aircraft she has in the air when her
//! flight deck is lost go to another deck or to a base by the rule. So the
//! full slice and the soak no longer script it: air group 12 and its arrival
//! are gone from the full slice (the id is retired), and Henderson Field has
//! those aircraft only in the games where the battle sends them.
//!
//! ## The hand calculation: seed 28, day 4 (24 August), afternoon
//!
//! Pinned outcome: it is what this seed gives on the frozen figures, and it
//! is here because it is the history. No chance is drawn in it.
//!
//! This file runs on the frozen copy of the full slice,
//! `data/scenarios/step2-frozen/07_full_slice.toml`: the scenario as it stood
//! with decision 0063's orders, before decision 0073 set the Japanese search
//! from the record. It proves the rules on known orders and known figures;
//! the live scenario is covered by the run baselines and `validate`. The copy
//! changes only with a rule change's new hand calculation.
//!
//! - Task Force 16 (Enterprise, ship 1) and Task Force 11 (Saratoga, ship 11)
//!   are in (7,-5). Henderson Field, base 1, is at (2,-4), 183 nm away
//!   (the hexes differ by 5 in q and 1 in r, which is 7.937 hex widths of
//!   centre to centre, 7.937 / sqrt 3 = 4.583 steps of 40 nm = 183.3 nm,
//!   cut to a whole mile).
//! - In the air from Enterprise: air group 101 (fighters) has 24 ready and 3
//!   fatigue, so it flies 24 x (100 - 3) / 100 = 23.28, cut to 23, on patrol;
//!   2 are shot down by the escorts of air group 111, so 21. Air group 103
//!   (scouts) flew 11 on search and lost none, so 11. Air groups 102 and 104
//!   flew nothing.
//! - One attack's bombs take her flight deck from working to degraded and from
//!   degraded to out. A deck that is out takes no aircraft.
//! - Saratoga's room: 90 (her aircraft capacity) less 69 (the ready and
//!   damaged aircraft of the groups aboard her: air group 105 with 25 ready
//!   and 1 damaged, 106 with 15, 107 with 15, 108 with 13) = 21.
//! - Group 101 comes first, the lower number: all 21 fit and land on Saratoga,
//!   joining her fighters, air group 105. Her room is then 0.
//! - Group 103 finds no room. The nearest base of its side is Henderson Field,
//!   within the strike radius of their type, 200 nm (the SBD-3; its search
//!   range is 200 nm too): all 11 land there. No group of their type is at the base, so they form a
//!   new one, "VS-5 landed at base:1", numbered from the Allied series (decision 0086), the
//!   first, 1,000,001, with 11 ready, at rest.
//! - Nothing comes down in the sea. Groups 102 and 104, aboard, are untouched.
//!
//! So: `AircraftLandedElsewhere` twice (21 to ship 11 joining group 105; 11 to
//! base 1 forming a group), no `AircraftDitchedNoDeck`, and at the end of day 4
//! a group of 11 scouts at Henderson Field.
//!
//! ## The scenario's own seed, 1942
//!
//! No Allied flight deck is lost in the fourteen days. Nothing lands elsewhere
//! at Henderson Field, and the field has no group beyond those the scenario
//! gives it.
//!
//! ## Properties, over the seeds the test runs
//!
//! - The scenario has no air group 12 and no arrival on day 4; no event names
//!   air group 12.
//! - Every `AircraftLandedElsewhere` and `AircraftDitchedNoDeck` in every run
//!   is for a group aboard a ship whose deck was lost in that pulse, with that
//!   loss as its first cause, and the group that receives a landing is the one
//!   the model of tests/common/no_deck.rs names (the lowest-numbered of the
//!   type and side at the place, else a new one). The counts, the room and the
//!   order are checked whole in the pinned run above and in scenarios 5, 5c,
//!   5d and 12c, not here: see the note below.
//! - Aircraft land at Henderson Field in some runs and not in all (more than
//!   none, fewer than all), and only in a pulse in which an Allied carrier's
//!   deck was lost.
//!
//! # What the properties do not check, and why
//!
//! The model of tests/common/no_deck.rs is used here for what it can be
//! given without loosening it: the property that a homecoming is only for
//! a group aboard a ship whose deck was lost in that pulse (its function
//! `assert_only_for_lost_decks`), and its rule for the receiving group and
//! for forming one, which the test below works out event by event. It is
//! not run on every event of every seed: `homecoming` needs, at the end of
//! the air combat step, the aircraft each group flew less those it lost,
//! the ready and damaged aircraft aboard every carrier, and each ship's
//! hex, and the full slice does not give them at that moment. The
//! `scenario_05d_carrier_duel` test rebuilds them from the events for two
//! carriers; for the full slice's ten groups of fighters, scouts and
//! bombers and their patrol shares, fatigue and deck factors it would be
//! a second copy of the engine.

mod common;

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use sim_core::events::{Cause, Event, EventKind};
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, AircraftTypeId, BaseId, ShipId};
use sim_core::state::{AirGroupLocation, ShipSystem, Side, SystemState};
use sim_core::time::Pulse;
use sim_data::Scenario;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
/// The frozen copy of the full slice (decision 0073), not the live file.
const FULL: &str = "data/scenarios/step2-frozen/07_full_slice.toml";
const ENTERPRISE: ShipId = ShipId::new(1);
const SARATOGA: ShipId = ShipId::new(11);
const HENDERSON: BaseId = BaseId::new(1);

fn load(seed: u64) -> Scenario {
    let path = PathBuf::from(ROOT).join(FULL);
    common::load(&path)
        .expect("the full slice loads")
        .with_seed(seed)
        .expect("a seed can be set")
}

fn run(seed: u64) -> Scenario {
    let mut scenario = load(seed);
    scenario.run_to_end().expect("the run resolves");
    scenario
}

fn group(n: u32) -> AirGroupId {
    AirGroupId::new(n)
}

fn is_afternoon(day: u32, event: &Event) -> bool {
    event.day == day && event.pulse == Some(Pulse::Afternoon)
}

/// The landings of a run: (event, from, to, at, aircraft, formed).
fn landings(events: &[Event]) -> Vec<(&Event, u32, u32, AirGroupLocation, u32, bool)> {
    events
        .iter()
        .filter_map(|e| match e.kind {
            EventKind::AircraftLandedElsewhere {
                from,
                to,
                at,
                aircraft,
                formed,
                ..
            } => Some((e, from.raw(), to.raw(), at, aircraft, formed)),
            _ => None,
        })
        .collect()
}

/// The event that took the flight deck of `ship` to out, or sank her.
fn deck_closer(events: &[Event], day: u32, pulse: Pulse, ship: ShipId) -> Option<&Event> {
    events.iter().rev().find(|e| {
        e.day == day
            && e.pulse == Some(pulse)
            && match e.kind {
                EventKind::SystemHit {
                    ship: hit,
                    system: ShipSystem::FlightDeck,
                    before,
                    after: SystemState::Out,
                    ..
                } => hit == ship && before != SystemState::Out,
                EventKind::ShipSunk { ship: sunk, .. } => sunk == ship,
                _ => false,
            }
    })
}

// ---------------------------------------------------------------------
// The pinned day: seed 28, 24 August, afternoon.
// ---------------------------------------------------------------------

#[test]
fn seed_28_the_figures_of_the_hand_calculation() {
    let scenario = load(28);
    let world = &scenario.world;
    let data = world.ref_data();
    let truth = world.umpire();
    // Both carriers in (7,-5); Henderson (2,-4) 183 nm from it.
    let hex_of = |ship: ShipId| match truth.ships[&ship].location {
        sim_core::state::ShipLocation::TaskForce(force) => truth.task_forces[&force].position(),
        other => panic!("{ship} is not in a task force: {other:?}"),
    };
    assert_eq!(hex_of(ENTERPRISE), Hex::new(7, -5));
    assert_eq!(hex_of(SARATOGA), Hex::new(7, -5));
    assert_eq!(truth.bases[&HENDERSON].hex, Hex::new(2, -4));
    assert_eq!(Hex::new(7, -5).air_distance_nm(Hex::new(2, -4)), 183);
    // Capacities and the SBD-3's radii.
    assert_eq!(
        data.ship_classes[&truth.ships[&SARATOGA].class].aircraft_capacity,
        90
    );
    let sbd = &data.aircraft_types[&AircraftTypeId::new(2001)];
    assert_eq!(sbd.search_range_nm.trunc(), 200);
    assert_eq!(sbd.strike_range_nm.trunc(), 200);
    // The groups: 101 fighters, 103 scouts (type 2001), 105 Saratoga's
    // fighters; the scenario's highest group is 120.
    assert_eq!(
        truth.air_groups[&group(103)].aircraft_type,
        AircraftTypeId::new(2001)
    );
    assert_eq!(truth.air_groups.keys().max(), Some(&group(120)));
    assert!(!truth.air_groups.contains_key(&group(12)));
}

#[test]
fn seed_28_day_4_afternoon_the_fighters_land_on_saratoga_and_the_scouts_form_a_group_at_henderson()
{
    let mut scenario = load(28);
    for _ in 0..3 {
        scenario.next_day().expect("a day resolves");
    }
    // The state at the start of day 4, to show that groups 102 and 104 are
    // not touched by the landing step.
    let before: BTreeMap<u32, (u32, u32)> = [102u32, 104]
        .into_iter()
        .map(|n| {
            let g = &scenario.world.umpire().air_groups[&group(n)];
            (n, (g.ready, g.damaged))
        })
        .collect();
    assert_eq!(before[&102], (17, 0));
    assert_eq!(before[&104], (14, 0));
    scenario.next_day().expect("day 4 resolves");
    let world = &scenario.world;
    let events: Vec<Event> = world.events().iter().cloned().collect();
    let pm: Vec<&Event> = events.iter().filter(|e| is_afternoon(4, e)).collect();

    // --- The two deck hits: one attack's bombs, working -> degraded -> out.
    let deck_hits: Vec<&&Event> = pm
        .iter()
        .filter(|e| {
            matches!(
                e.kind,
                EventKind::SystemHit {
                    ship: ENTERPRISE,
                    system: ShipSystem::FlightDeck,
                    ..
                }
            )
        })
        .collect();
    // The attack of the first raid: two steps down from one attack; the
    // second raid's hit finds the deck already out (no change).
    let steps: Vec<(SystemState, SystemState)> = deck_hits
        .iter()
        .map(|e| match e.kind {
            EventKind::SystemHit { before, after, .. } => (before, after),
            _ => unreachable!(),
        })
        .collect();
    assert_eq!(
        steps,
        vec![
            (SystemState::Working, SystemState::Degraded),
            (SystemState::Degraded, SystemState::Out),
            (SystemState::Out, SystemState::Out),
        ]
    );
    assert_eq!(
        deck_hits[0].causes, deck_hits[1].causes,
        "one attack's bombs"
    );
    let Cause::Event(attack_id) = deck_hits[1].causes[0] else {
        panic!("the deck hit cites the attack");
    };
    let attack = events
        .iter()
        .find(|e| e.id == attack_id)
        .expect("the attack");
    assert!(
        matches!(
            attack.kind,
            EventKind::AirAttack {
                ship: ENTERPRISE,
                ..
            }
        ),
        "{}",
        attack.kind
    );
    let closer = deck_closer(&events, 4, Pulse::Afternoon, ENTERPRISE).expect("the deck is out");
    assert_eq!(closer.id, deck_hits[1].id, "the second hit closes the deck");
    // Saratoga is untouched, and nobody is sunk on the Allied side.
    assert!(!events.iter().any(|e| matches!(
        e.kind,
        EventKind::SystemHit { ship: SARATOGA, .. }
            | EventKind::ShipSunk {
                ship: ENTERPRISE | SARATOGA,
                ..
            }
    )));

    // --- The patrol: 101 flew 23 (24 ready x (100 - 3) / 100 = 23.28), the
    // escorts of 111 hit it twice, of 114 not at all: 2 lost, 21 left in the
    // air.
    let fights_at_101: Vec<u32> = pm
        .iter()
        .filter_map(|e| match e.kind {
            EventKind::AirFight { target, hits, .. } if target == group(101) => Some(hits),
            _ => None,
        })
        .collect();
    assert_eq!(fights_at_101, vec![2, 0]);
    let lost_101: Vec<(u32, u32)> = pm
        .iter()
        .filter_map(|e| match e.kind {
            EventKind::AircraftLost {
                air_group,
                aircraft,
                remaining,
            } if air_group == group(101) => Some((aircraft, remaining)),
            _ => None,
        })
        .collect();
    assert_eq!(lost_101, vec![(2, 22)]);
    let morning_101 = events.iter().rev().find_map(|e| match e.kind {
        EventKind::AircraftWorn {
            air_group,
            ready,
            damaged,
            ..
        } if air_group == group(101) && e.day == 4 && e.pulse == Some(Pulse::Morning) => {
            Some((ready, damaged))
        }
        _ => None,
    });
    assert_eq!(
        morning_101,
        Some((24, 2)),
        "24 ready at the start of the afternoon"
    );

    // --- The scouts flew 11 and lost none.
    let searched: Vec<(&Event, u32)> = pm
        .iter()
        .filter_map(|e| match e.kind {
            EventKind::SearchFlown {
                air_group,
                aircraft,
                ..
            } if air_group == group(103) => Some((*e, aircraft)),
            _ => None,
        })
        .collect();
    assert_eq!(searched.len(), 1);
    assert_eq!(searched[0].1, 11);
    assert!(!pm.iter().any(|e| matches!(
        e.kind,
        EventKind::AircraftLost { air_group, .. } if air_group == group(103)
    )));
    // 102 and 104 flew nothing.
    assert!(!pm.iter().any(|e| match e.kind {
        EventKind::SearchFlown { air_group, .. }
        | EventKind::StrikeLaunched { air_group, .. }
        | EventKind::AircraftLost { air_group, .. } =>
            air_group == group(102) || air_group == group(104),
        EventKind::EscortsSent { air_group, .. } =>
            air_group == group(102) || air_group == group(104),
        _ => false,
    }));

    // --- The two landings, whole, with their causes; nothing in the sea.
    let homing: Vec<&&Event> = pm
        .iter()
        .filter(|e| {
            matches!(
                e.kind,
                EventKind::AircraftLandedElsewhere { .. } | EventKind::AircraftDitchedNoDeck { .. }
            )
        })
        .collect();
    assert_eq!(homing.len(), 2, "two landings and nothing else");
    assert_eq!(
        homing[0].kind,
        EventKind::AircraftLandedElsewhere {
            from: group(101),
            to: group(105),
            at: AirGroupLocation::Ship(SARATOGA),
            aircraft: 21,
            pilots: 0,
            formed: false,
        }
    );
    // The cause of the deck first, then the order that kept the patrol up.
    assert_eq!(
        homing[0].causes,
        vec![
            Cause::Event(closer.id),
            Cause::Order(sim_core::ids::OrderId::for_umpire(25, common::umpire_key()))
        ]
    );
    assert_eq!(
        homing[1].kind,
        EventKind::AircraftLandedElsewhere {
            from: group(103),
            to: group(1_000_001),
            at: AirGroupLocation::Base(HENDERSON),
            aircraft: 11,
            pilots: 0,
            formed: true,
        }
    );
    assert_eq!(
        homing[1].causes,
        vec![Cause::Event(closer.id), Cause::Event(searched[0].0.id)]
    );
    assert!(homing[0].id < homing[1].id, "group 101 comes first");
    assert_eq!(
        homing[1].kind.to_string(),
        "air_group:103 had no deck: 11 aircraft landed at base:1, forming air_group:1000001"
    );
    assert!(!events
        .iter()
        .any(|e| matches!(e.kind, EventKind::AircraftDitchedNoDeck { air_group, .. } if air_group.raw() < 100)));

    // --- The state at the end of day 4.
    let truth = world.umpire();
    let formed = &truth.air_groups[&group(1_000_001)];
    assert_eq!(formed.name, "VS-5 landed at base:1");
    assert_eq!(formed.location, AirGroupLocation::Base(HENDERSON));
    // The 11 landed ready (the landing event above says so). The day's wear
    // then drew for them keyed on their new number (decision 0086), and took
    // it 9 ready and 1 damaged of the 11: a pinned outcome of seed 28, was (11, 0) when the
    // group was numbered 121.
    let worn = events.iter().find_map(|e| match e.kind {
        EventKind::AircraftWorn {
            air_group,
            aircraft,
            ready,
            damaged,
            ..
        } if air_group == group(1_000_001) && e.day == 4 => Some((aircraft, ready, damaged)),
        _ => None,
    });
    assert_eq!(worn, Some((11, 9, 1)), "the wear at the end of the day");
    assert_eq!((formed.ready, formed.damaged), (9, 1));
    assert_eq!(formed.aircraft_type, AircraftTypeId::new(2001));
    assert_eq!(formed.side, Side::Allied);
    assert_eq!(formed.mission, sim_core::orders::AirMission::Rest);
    assert_eq!(truth.air_groups.keys().max(), Some(&group(1_000_001)));
    // Saratoga holds her 69 and the 21: 90, her capacity, and no more.
    let aboard: u32 = truth
        .air_groups
        .values()
        .filter(|g| g.location == AirGroupLocation::Ship(SARATOGA))
        .map(|g| g.ready + g.damaged)
        .sum();
    assert_eq!(aboard, 90);
    let g105 = &truth.air_groups[&group(105)];
    assert_eq!(
        g105.ready + g105.damaged,
        26 + 21,
        "105 had 25 ready and 1 damaged"
    );
    // 102 and 104 are as they were at the start of the day.
    for n in [102u32, 104] {
        let g = &truth.air_groups[&group(n)];
        assert_eq!((g.ready, g.damaged), before[&n], "group {n}");
    }
}

// ---------------------------------------------------------------------
// The scenario's own seed.
// ---------------------------------------------------------------------

#[test]
fn seed_1942_no_allied_deck_is_lost_and_henderson_gets_no_new_group() {
    let scenario = run(1942);
    let events: Vec<Event> = scenario.world.events().iter().cloned().collect();
    assert!(!events.iter().any(|e| matches!(
        e.kind,
        EventKind::SystemHit {
            ship: ENTERPRISE | SARATOGA,
            system: ShipSystem::FlightDeck,
            after: SystemState::Out,
            ..
        } | EventKind::ShipSunk {
            ship: ENTERPRISE | SARATOGA,
            ..
        }
    )));
    assert!(!events.iter().any(|e| matches!(
        e.kind,
        EventKind::AircraftLandedElsewhere {
            at: AirGroupLocation::Base(HENDERSON),
            ..
        }
    )));
    let at_henderson: Vec<u32> = scenario
        .world
        .umpire()
        .air_groups
        .values()
        .filter(|g| g.location == AirGroupLocation::Base(HENDERSON))
        .map(|g| g.id.raw())
        .collect();
    assert_eq!(at_henderson, vec![10, 11, 13, 14, 15, 16, 17]);
}

// ---------------------------------------------------------------------
// Properties.
// ---------------------------------------------------------------------

/// The seeds the property tests run: 1 to 60, which include 28 and 37, and
/// the scenario's own, 1942.
fn seeds() -> Vec<u64> {
    (1..=60).chain([1942]).collect()
}

#[test]
fn the_scenario_has_no_air_group_12_and_no_arrival_on_day_4() {
    let text = fs::read_to_string(PathBuf::from(ROOT).join(FULL)).expect("the scenario reads");
    assert!(
        !text.lines().any(|l| l.trim() == "name = \"Flight 300\""),
        "no group named Flight 300"
    );
    assert!(
        !text.lines().any(|l| l.trim() == "air_group = 12"),
        "no order or arrival for group 12"
    );
    // The scenario that has no carriers still scripts it.
    let six =
        fs::read_to_string(PathBuf::from(ROOT).join("data/scenarios/06_rabaul_guadalcanal.toml"))
            .expect("scenario 6 reads");
    assert!(six.lines().any(|l| l.trim() == "name = \"Flight 300\""));
    assert!(six.lines().any(|l| l.trim() == "air_group = 12"));

    for seed in seeds() {
        let scenario = run(seed);
        let world = &scenario.world;
        assert!(
            !world.umpire().air_groups.contains_key(&group(12)),
            "seed {seed}"
        );
        for event in world.events().iter() {
            let text = event.kind.to_string();
            assert!(
                !text
                    .split(|c: char| !(c.is_alphanumeric() || c == ':'))
                    .any(|word| word == "air_group:12"),
                "seed {seed}: an event names air group 12: {text}"
            );
            assert!(
                !(event.day == 4 && matches!(event.kind, EventKind::AircraftArrived { .. })),
                "seed {seed}: an arrival on day 4"
            );
        }
    }
}

#[test]
fn every_homecoming_follows_the_model_and_aircraft_land_at_henderson_in_some_runs_only() {
    let seed_list = seeds();
    let mut runs_with_landing_at_henderson = 0;
    let mut counts: Vec<u32> = Vec::new();
    let mut homecomings = 0;
    for &seed in &seed_list {
        let scenario = run(seed);
        let world = &scenario.world;
        let events: Vec<Event> = world.events().iter().cloned().collect();
        // The ships and groups as the scenario gives them, before the run.
        let (ships, mut model_groups, _bases) = common::no_deck::initial(&load(seed).world);
        let side_of_ship: BTreeMap<u32, (Side, bool)> =
            ships.iter().map(|s| (s.id, (s.side, s.carrier))).collect();
        let ship_of_group: BTreeMap<u32, ShipId> = model_groups
            .iter()
            .filter_map(|g| match g.place {
                AirGroupLocation::Ship(ship) => Some((g.id, ship)),
                AirGroupLocation::Base(_) => None,
            })
            .collect();
        // Only for a group aboard a ship whose deck sank or went out in the
        // same pulse (the model's own property).
        homecomings += common::no_deck::assert_only_for_lost_decks(&events, |g| {
            ship_of_group.get(&g.raw()).copied()
        });

        // The receiving group, by the model's rule: the lowest-numbered group
        // of the type and side at the place, else a new one numbered next in its
        // side's series (decision 0086); and a landing at a ship goes to a carrier of the side,
        // not the source's own ship.
        let mut henderson = 0;
        for (event, from, to, at, aircraft, formed) in landings(&events) {
            let source = model_groups
                .iter()
                .find(|g| g.id == from)
                .unwrap_or_else(|| panic!("seed {seed}: group {from} exists"))
                .clone();
            assert!(aircraft > 0);
            if let AirGroupLocation::Ship(ship) = at {
                let (side, carrier) = side_of_ship[&ship.raw()];
                assert!(
                    carrier && side == source.side,
                    "seed {seed}: lands on a carrier of its side"
                );
                assert_ne!(Some(&ship), ship_of_group.get(&from), "seed {seed}");
            }
            let existing = model_groups
                .iter()
                .filter(|g| {
                    g.place == at
                        && g.side == source.side
                        && g.aircraft_type == source.aircraft_type
                })
                .map(|g| g.id)
                .min();
            match existing {
                Some(lowest) => {
                    assert!(
                        !formed,
                        "seed {seed}: a group of the type is there: {} {from} {to} {at:?} {lowest}",
                        event.kind
                    );
                    assert_eq!(
                        to, lowest,
                        "seed {seed}: the lowest-numbered group of the type"
                    );
                }
                None => {
                    assert!(
                        formed,
                        "seed {seed}: none of the type is there, so one is formed"
                    );
                    let series: u32 = match source.side {
                        Side::Allied => 1_000_000,
                        Side::Japanese => 2_000_000,
                    };
                    let next = model_groups
                        .iter()
                        .filter(|g| g.side == source.side && g.id > series)
                        .map(|g| g.id)
                        .max()
                        .unwrap_or(series)
                        + 1;
                    assert_eq!(
                        to, next,
                        "seed {seed}: the next number of its side's series"
                    );
                    let mut new = source.clone();
                    new.id = to;
                    new.place = at;
                    new.ready = 0;
                    new.damaged = 0;
                    new.in_air = 0;
                    model_groups.push(new);
                }
            }
            if at == AirGroupLocation::Base(HENDERSON) {
                henderson += aircraft;
                // Only an Allied carrier's lost deck sends aircraft here.
                assert_eq!(source.side, Side::Allied, "seed {seed}");
                let Some(Cause::Event(closing)) = event.causes.first().copied() else {
                    panic!("seed {seed}: the landing cites the closing event");
                };
                let closing = events
                    .iter()
                    .find(|e| e.id == closing)
                    .expect("closing event");
                assert_eq!((closing.day, closing.pulse), (event.day, event.pulse));
                assert!(
                    matches!(
                        closing.kind,
                        EventKind::SystemHit {
                            ship: ENTERPRISE | SARATOGA,
                            after: SystemState::Out,
                            ..
                        } | EventKind::ShipSunk {
                            ship: ENTERPRISE | SARATOGA,
                            ..
                        }
                    ),
                    "seed {seed}: {}",
                    closing.kind
                );
            }
        }
        if henderson > 0 {
            runs_with_landing_at_henderson += 1;
            counts.push(henderson);
        }
    }
    println!(
        "{} runs, {} with aircraft landing at Henderson Field: {:?}; {} homecomings",
        seed_list.len(),
        runs_with_landing_at_henderson,
        counts,
        homecomings
    );
    assert!(homecomings > 0);
    assert!(
        runs_with_landing_at_henderson > 0,
        "some runs send aircraft to Henderson"
    );
    assert!(
        runs_with_landing_at_henderson < seed_list.len(),
        "not all runs do"
    );
}

#[test]
fn seeds_28_and_37_land_exactly_eleven_on_24_august_and_seed_1942_none() {
    for seed in [28u64, 37] {
        let scenario = run(seed);
        let events: Vec<Event> = scenario.world.events().iter().cloned().collect();
        let day4: Vec<u32> = landings(&events)
            .iter()
            .filter(|l| l.0.day == 4 && l.3 == AirGroupLocation::Base(HENDERSON))
            .map(|l| l.4)
            .collect();
        assert_eq!(day4, vec![11], "seed {seed}");
    }
}
