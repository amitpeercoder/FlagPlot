//! Scenario 5b: a carrier's scouts find an enemy force and its bombers
//! strike it.
//!
//! Stage 5b of scenario 5. It proves the strike decision, a strike that
//! flies on belief and finds its target or finds nothing, bomb hits through
//! the damage rules of surface combat, and the flight deck. Nobody opposes
//! the strike.
//!
//! The data is placeholder data: see `data/scenarios/05b_strike.toml`. The
//! rule is `docs/decisions/0026-air-strikes.md`.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. The tests below assert these
//! numbers. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass.
//!
//! ## The setting
//!
//! The Allied carrier force, task force 1, lies stopped at (4,0). Aboard
//! carrier A, ship 1, are air group 1, eight scouts, and air group 2,
//! eighteen dive bombers. The Japanese force, task force 2, lies stopped at
//! (9,0), five hexes and 200 nm east: carrier B, ship 3, and two cruisers.
//! Nothing moves. The scenario is one day: no air step in the night pulse,
//! then a morning and an afternoon pulse, each with search, then strike
//! decisions, then air combat.
//!
//! ## Search
//!
//! The scouts search the arc 040 to 130, 90 degrees, out to 300 nm, with a
//! sweep width of 20 nm. The target bears 090 at 200 nm. The chance in each
//! daylight pulse is 57.295779 x 8 x 20 / (90 x 200) = 9167.32464 / 18000 =
//! 0.509295.
//!
//! A report names the true hex with chance 0.7 and each of the six
//! neighbouring hexes with chance 0.05.
//!
//! ## The decision
//!
//! The dive bomber strikes out to 250 nm, and strikes a contact reported in
//! this pulse or the one before. (First written as "no more than one pulse
//! old", which by the way age is counted would also allow a contact from
//! the afternoon before. No figure of this one-day scenario changed.) Every hex next to (9,0) is five or six hexes from (4,0),
//! 240 nm at most, so any contact made by a sighting of task force 2 is in
//! range. The Allied side holds at most one contact: a second report falls
//! within two hexes and two pulses of the first and refreshes it.
//!
//! So in each daylight pulse the bombers launch if, and only if, the Allied
//! side then holds a contact, and all 18 go, to the hex the contact names.
//!
//! - Morning: they launch if the morning search sighted, chance 0.509295.
//! - Afternoon: they launch if either search sighted. The chance that
//!   neither did is 0.490705 x 0.490705 = 0.240791, so 0.759209.
//!
//! | Count of runs, over seeds 1 to 1000 | p        | Mean   | Std dev | Allowed    |
//! | ----------------------------------- | -------- | ------ | ------- | ---------- |
//! | A strike is launched in the morning | 0.509295 | 509.30 | 15.81   | 447 to 572 |
//! | A strike is launched in the afternoon | 0.759209 | 759.21 | 13.52 | 706 to 813 |
//!
//! ## The strike
//!
//! A strike attacks if task force 2 is in the hex it flew to, which is
//! (9,0), and otherwise finds nothing. A morning strike flies to the hex of
//! the morning's search report, so it attacks with chance 0.7: among the
//! runs with a morning strike, the number that attack is within four
//! standard deviations of 0.7 of them.
//!
//! An attack is on carrier B, ship 3, the most important ship of the force:
//! 18 shots at a bomb's chance of 0.2 is 3.6 expected hits, so 3 hits with
//! chance 0.4 and 4 with chance 0.6. Among all attacks, the number with 4
//! hits is within four standard deviations of 0.6 of them.
//!
//! A hit does 100 x 60 / 600 = 10 points, on flotation with chance 0.75 and
//! on fire with chance 0.25. So an attack does 30 or 40 points in all,
//! and the carrier's flotation and fire damage together are 10 times the
//! hits it has taken. Two attacks do 80 points at most, so carrier B is
//! never sunk and is the target of every attack. Each hit is a system hit
//! with chance 0.3, landing on the flight deck with chance 0.5, propulsion
//! 0.2, fire control 0.1 and the main battery 0.2.
//!
//! No aircraft is lost: air group 2 has 18 aircraft at the end of every run.
//! Ships 1, 2, 4 and 5 are never damaged.
//!
//! A strike that attacks sends a report of task force 2, which refreshes the
//! contact. A strike that finds nothing sends none.
//!
//! ## The flight deck
//!
//! Three variants of the scenario, each differing only in the state of
//! carrier A's flight deck at the start:
//!
//! | Flight deck | Scouts flown | Search chance                       | Bombers launched |
//! | ----------- | ------------ | ----------------------------------- | ---------------- |
//! | Working     | 8            | 0.509295                            | 18               |
//! | Degraded    | 4            | 57.295779 x 4 x 20 / 18000 = 0.254647 | 9: 1.8 expected hits, so 1 or 2 |
//! | Out         | none: no search is flown | no attempt              | none: no strike is ever launched |
//!
//! # Properties
//!
//! - A strike is launched only at a contact the launching side holds,
//!   reported in that pulse or the one before, within 250 nm.
//! - A strike attacks only a task force whose position is the hex it flew
//!   to. Otherwise it finds nothing, and nothing is damaged.
//! - Every damage event of the day is caused by an attack, and every attack
//!   by a launch.
//! - The aircraft a group flies never exceed its aircraft, and no track is
//!   above 100.
//! - More bombers never mean fewer expected hits: with 28 bombers, which
//!   with the scouts fills the carrier's 36, the expected hits are 5.6.
//! - A replay of any seed reproduces the daily hash.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use sim_core::events::{Cause, DamageTrack, Event, EventKind, Weapon};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, ContactId, EventId, OrderId, ShipId, TaskForceId};
use sim_core::state::{ShipLocation, ShipSystem, Side, SystemState};
use sim_core::time::Pulse;
use sim_data::Scenario;

const ALLIED_FORCE: TaskForceId = TaskForceId::new(1);
const ENEMY_FORCE: TaskForceId = TaskForceId::new(2);
const SCOUTS: AirGroupId = AirGroupId::new(1);
const BOMBERS: AirGroupId = AirGroupId::new(2);
const CARRIER_A: ShipId = ShipId::new(1);
const CARRIER_B: ShipId = ShipId::new(3);
const SEEDS: u64 = 1000;
const HOME: Hex = Hex::new(4, 0);
const TARGET: Hex = Hex::new(9, 0);

fn scenario_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/05b_strike.toml")
}

fn load() -> Scenario {
    Scenario::load(&scenario_path()).expect("scenario 5b loads")
}

fn millionths(value: i64) -> Fixed {
    Fixed::from_raw(value)
}

fn tenths(value: i64) -> Fixed {
    Fixed::from_ratio(value, 10)
}

/// Whether k out of n is within four standard deviations of probability
/// `p` (in millionths), in exact integer arithmetic.
fn within_four_sigma(k: i64, n: i64, p: i64) -> bool {
    let (k, n, p) = (i128::from(k), i128::from(n), i128::from(p));
    let scale = 1_000_000;
    let off = k * scale - n * p;
    off * off <= 16 * n * p * (scale - p)
}

fn pulse_of(event: &Event) -> Pulse {
    event.pulse.expect("this event has a pulse")
}

fn count(value: usize) -> i64 {
    i64::try_from(value).expect("small")
}

// ---------------------------------------------------------------------
// What one run did, read from its events and final state.
// ---------------------------------------------------------------------

struct Final {
    flotation: Fixed,
    fire: Fixed,
    sunk: bool,
    systems: [SystemState; 5],
}

struct Run {
    seed: u64,
    events: Vec<Event>,
    hashes: Vec<u64>,
    ships: Vec<(u32, Final)>,
    bombers_ready: u32,
    scouts_ready: u32,
    allied_force_at: Hex,
    enemy_force_at: Hex,
}

impl Run {
    fn of(&self, keep: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|event| keep(&event.kind))
            .collect()
    }

    fn flown(&self) -> Vec<(Pulse, u32)> {
        self.of(|k| matches!(k, EventKind::SearchFlown { .. }))
            .iter()
            .map(|event| match event.kind {
                EventKind::SearchFlown { aircraft, .. } => (pulse_of(event), aircraft),
                _ => unreachable!(),
            })
            .collect()
    }

    /// (pulse, distance, chance, sighted) of each search attempt.
    fn attempts(&self) -> Vec<(Pulse, Fixed, Fixed, bool)> {
        self.of(|k| matches!(k, EventKind::SearchAttempt { .. }))
            .iter()
            .map(|event| match event.kind {
                EventKind::SearchAttempt {
                    distance_nm,
                    chance,
                    sighted,
                    ..
                } => (pulse_of(event), distance_nm, chance, sighted),
                _ => unreachable!(),
            })
            .collect()
    }

    fn sighted_in(&self, pulse: Pulse) -> bool {
        self.attempts()
            .iter()
            .any(|attempt| attempt.0 == pulse && attempt.3)
    }

    fn launches(&self) -> Vec<&Event> {
        self.of(|k| matches!(k, EventKind::StrikeLaunched { .. }))
    }

    fn launched_in(&self, pulse: Pulse) -> Option<&Event> {
        self.launches()
            .into_iter()
            .find(|event| pulse_of(event) == pulse)
    }

    fn attacks(&self) -> Vec<&Event> {
        self.of(|k| matches!(k, EventKind::AirAttack { .. }))
    }

    fn found_nothing(&self) -> Vec<&Event> {
        self.of(|k| matches!(k, EventKind::StrikeFoundNothing { .. }))
    }

    fn event(&self, id: EventId) -> &Event {
        self.events
            .iter()
            .find(|event| event.id == id)
            .expect("the event exists")
    }

    fn caused_by(&self, id: EventId) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|event| event.causes.contains(&Cause::Event(id)))
            .collect()
    }

    fn ship(&self, id: u32) -> &Final {
        &self
            .ships
            .iter()
            .find(|(ship, _)| *ship == id)
            .expect("the ship")
            .1
    }

    /// Hits of all the attacks of the run.
    fn hits(&self) -> u32 {
        self.attacks()
            .iter()
            .map(|event| match event.kind {
                EventKind::AirAttack { hits, .. } => hits,
                _ => unreachable!(),
            })
            .sum()
    }
}

fn run_of(base: &Scenario, seed: u64) -> Run {
    let mut run = base.with_seed(seed).expect("a seed can be set");
    run.run_to_end().expect("the run resolves");
    digest(&run, seed)
}

fn digest(run: &Scenario, seed: u64) -> Run {
    let truth = run.world.umpire();
    Run {
        seed,
        events: run.world.events().iter().cloned().collect(),
        hashes: run.world.day_hashes().to_vec(),
        ships: truth
            .ships
            .values()
            .map(|ship| {
                (
                    ship.id.raw(),
                    Final {
                        flotation: ship.flotation_damage,
                        fire: ship.fire_damage,
                        sunk: ship.location == ShipLocation::Sunk,
                        systems: ShipSystem::ALL.map(|system| ship.systems.get(system)),
                    },
                )
            })
            .collect(),
        bombers_ready: truth.air_groups[&BOMBERS].ready,
        scouts_ready: truth.air_groups[&SCOUTS].ready,
        allied_force_at: truth.task_forces[&ALLIED_FORCE].position(),
        enemy_force_at: truth.task_forces[&ENEMY_FORCE].position(),
    }
}

fn batch() -> &'static Vec<Run> {
    static BATCH: OnceLock<Vec<Run>> = OnceLock::new();
    BATCH.get_or_init(|| {
        let base = load();
        (1..=SEEDS).map(|seed| run_of(&base, seed)).collect()
    })
}

const ALL_WORKING: [SystemState; 5] = [SystemState::Working; 5];

// ---------------------------------------------------------------------
// The setting and the search.
// ---------------------------------------------------------------------

#[test]
fn the_setting_is_as_the_hand_calculation_says_and_nothing_moves() {
    let scenario = load();
    let truth = scenario.world.umpire();
    assert_eq!(truth.task_forces[&ALLIED_FORCE].position(), HOME);
    assert_eq!(truth.task_forces[&ENEMY_FORCE].position(), TARGET);
    assert_eq!(HOME.distance(TARGET), 5);
    assert_eq!(scenario.days, 1);
    let ships = |force: TaskForceId| -> Vec<u32> {
        truth.task_forces[&force]
            .ships
            .iter()
            .map(|ship| ship.raw())
            .collect()
    };
    assert_eq!(ships(ALLIED_FORCE), vec![1, 2]);
    assert_eq!(ships(ENEMY_FORCE), vec![3, 4, 5]);
    assert_eq!(truth.air_groups[&SCOUTS].ready, 8);
    assert_eq!(truth.air_groups[&BOMBERS].ready, 18);
    for run in batch() {
        assert_eq!(run.allied_force_at, HOME, "seed {}", run.seed);
        assert_eq!(run.enemy_force_at, TARGET, "seed {}", run.seed);
    }
}

#[test]
fn the_search_chance_is_0_509295_in_each_daylight_pulse_and_none_flies_in_the_night() {
    for run in batch() {
        assert_eq!(
            run.flown(),
            vec![(Pulse::Morning, 8), (Pulse::Afternoon, 8)],
            "seed {}",
            run.seed
        );
        let attempts = run.attempts();
        assert_eq!(attempts.len(), 2, "seed {}", run.seed);
        for (attempt, pulse) in attempts.iter().zip([Pulse::Morning, Pulse::Afternoon]) {
            // 57.295779 x 8 x 20 / (90 x 200) = 9167.32464 / 18000.
            assert_eq!(
                (attempt.0, attempt.1, attempt.2),
                (pulse, Fixed::from_int(200), millionths(509_295)),
                "seed {}",
                run.seed
            );
        }
        // Nothing of the air war happens in the night pulse, and nothing
        // before the day.
        for event in &run.events {
            assert_eq!(event.day, 1);
            if matches!(
                event.kind,
                EventKind::SearchFlown { .. }
                    | EventKind::SearchAttempt { .. }
                    | EventKind::StrikeLaunched { .. }
                    | EventKind::AirAttack { .. }
                    | EventKind::StrikeFoundNothing { .. }
            ) {
                assert_ne!(event.pulse, Some(Pulse::Night), "seed {}", run.seed);
            }
        }
    }
    let sighted: Vec<i64> = [Pulse::Morning, Pulse::Afternoon]
        .iter()
        .map(|pulse| count(batch().iter().filter(|run| run.sighted_in(*pulse)).count()))
        .collect();
    eprintln!("sighted by pulse: {sighted:?}");
    for count in sighted {
        assert!(within_four_sigma(count, 1000, 509_295), "{count}");
    }
}

#[test]
fn a_search_report_names_the_true_hex_with_chance_0_7_and_each_neighbour_with_0_05() {
    let mut reports = 0;
    let mut at_true = 0;
    let mut at_neighbour = [0i64; 6];
    for run in batch() {
        for event in run.of(|k| matches!(k, EventKind::ReportReceived { .. })) {
            let from_search = event.causes.iter().any(|cause| match cause {
                Cause::Event(id) => matches!(run.event(*id).kind, EventKind::SearchAttempt { .. }),
                Cause::Order(_) => false,
            });
            if !from_search {
                continue;
            }
            let EventKind::ReportReceived { hex, .. } = event.kind else {
                unreachable!()
            };
            reports += 1;
            if hex == TARGET {
                at_true += 1;
            } else {
                let slot = TARGET
                    .neighbors()
                    .iter()
                    .position(|neighbour| *neighbour == hex)
                    .expect("a report names the hex or a neighbour");
                at_neighbour[slot] += 1;
            }
        }
    }
    eprintln!("search reports {reports}: true {at_true}, neighbours {at_neighbour:?}");
    assert!(within_four_sigma(at_true, reports, 700_000));
    for count in at_neighbour {
        assert!(within_four_sigma(count, reports, 50_000), "{count}");
    }
}

// ---------------------------------------------------------------------
// The decision.
// ---------------------------------------------------------------------

#[test]
fn the_bombers_launch_if_and_only_if_the_allied_side_holds_a_contact_and_all_18_go() {
    let mut morning = 0;
    let mut afternoon = 0;
    for run in batch() {
        let seed = run.seed;
        let morning_sighted = run.sighted_in(Pulse::Morning);
        let afternoon_sighted = run.sighted_in(Pulse::Afternoon);
        assert_eq!(
            run.launched_in(Pulse::Morning).is_some(),
            morning_sighted,
            "seed {seed}"
        );
        assert_eq!(
            run.launched_in(Pulse::Afternoon).is_some(),
            morning_sighted || afternoon_sighted,
            "seed {seed}"
        );
        morning += i64::from(morning_sighted);
        afternoon += i64::from(morning_sighted || afternoon_sighted);

        // At most one launch a pulse, and the allied side never holds more
        // than the one contact: every report refreshes contact 1.
        assert!(run.launches().len() <= 2);
        for event in run.of(|k| matches!(k, EventKind::ReportReceived { .. })) {
            let EventKind::ReportReceived {
                side,
                contact,
                created,
                ..
            } = event.kind
            else {
                unreachable!()
            };
            assert_eq!(side, Side::Allied);
            assert_eq!(contact, ContactId::new(1));
            let first = run
                .of(|k| matches!(k, EventKind::ReportReceived { .. }))
                .first()
                .expect("a report")
                .id;
            assert_eq!(created, event.id == first, "seed {seed}");
        }
    }
    eprintln!("launched in the morning {morning}, in the afternoon {afternoon}");
    // 447 to 572 and 706 to 813, four standard deviations either side.
    assert!((447..=572).contains(&morning), "{morning}");
    assert!((706..=813).contains(&afternoon), "{afternoon}");
    assert!(within_four_sigma(morning, 1000, 509_295));
    assert!(within_four_sigma(afternoon, 1000, 759_209));
}

#[test]
fn a_strike_is_launched_only_at_a_contact_held_no_more_than_one_pulse_old_within_250_nm() {
    for run in batch() {
        for event in run.launches() {
            let EventKind::StrikeLaunched {
                air_group,
                contact,
                hex,
                aircraft,
                distance_nm,
            } = event.kind
            else {
                unreachable!()
            };
            assert_eq!(air_group, BOMBERS);
            assert_eq!(contact, ContactId::new(1));
            assert_eq!(aircraft, 18);
            assert_eq!(
                distance_nm,
                Fixed::from_int(i64::from(HOME.distance(hex)) * 40)
            );
            assert!(distance_nm <= Fixed::from_int(250), "seed {}", run.seed);
            // Every hex next to the target is five or six hexes out.
            assert!(
                hex == TARGET || TARGET.neighbors().contains(&hex),
                "seed {}",
                run.seed
            );
            assert!(distance_nm <= Fixed::from_int(240), "seed {}", run.seed);

            // It cites the order that set the mission and the report that
            // last refreshed the contact.
            let [Cause::Order(order), Cause::Event(report)] = event.causes[..] else {
                panic!("a launch cites an order and a report: {:?}", event.causes);
            };
            assert_eq!(order, OrderId::new(3));
            let report_event = run.event(report);
            let EventKind::ReportReceived {
                contact: reported,
                hex: reported_hex,
                ..
            } = report_event.kind
            else {
                panic!("a launch cites a report");
            };
            assert_eq!((reported, reported_hex), (contact, hex));
            // Reported in this pulse or the one before: this pulse's, or the
            // morning's for an afternoon strike.
            let this = pulse_of(event);
            let then = pulse_of(report_event);
            assert!(
                then == this || (then == Pulse::Morning && this == Pulse::Afternoon),
                "seed {}: a {then:?} report for a {this:?} strike",
                run.seed
            );
            assert!(report_event.id < event.id);
        }
    }
}

// ---------------------------------------------------------------------
// The strike.
// ---------------------------------------------------------------------

#[test]
fn a_strike_attacks_if_the_force_is_in_the_hex_it_flew_to_and_otherwise_finds_nothing() {
    let mut morning_launches = 0;
    let mut morning_attacks = 0;
    let mut finds_nothing = 0;
    for run in batch() {
        for launch in run.launches() {
            let EventKind::StrikeLaunched { hex, .. } = launch.kind else {
                unreachable!()
            };
            let results = run.caused_by(launch.id);
            assert_eq!(results.len(), 1, "seed {}", run.seed);
            let result = results[0];
            let attacked = matches!(result.kind, EventKind::AirAttack { .. });
            assert_eq!(attacked, hex == TARGET, "seed {}", run.seed);
            if !attacked {
                assert_eq!(
                    result.kind,
                    EventKind::StrikeFoundNothing {
                        air_group: BOMBERS,
                        hex
                    }
                );
                assert_eq!(
                    result.kind.to_string(),
                    format!("air_group:2 found nothing at {hex}")
                );
                finds_nothing += 1;
            }
            assert_eq!(pulse_of(result), pulse_of(launch));
            if pulse_of(launch) == Pulse::Morning {
                morning_launches += 1;
                morning_attacks += i64::from(attacked);
            }
        }
        assert_eq!(
            run.attacks().len() + run.found_nothing().len(),
            run.launches().len()
        );
    }
    eprintln!(
        "morning strikes {morning_launches}, attacking {morning_attacks}; found nothing {finds_nothing}"
    );
    // A morning strike flies to the hex of the morning's search report, so
    // it attacks with chance 0.7.
    assert!(within_four_sigma(
        morning_attacks,
        morning_launches,
        700_000
    ));
}

#[test]
fn an_attack_is_18_bombs_at_0_2_on_carrier_b_with_3_hits_at_0_4_and_4_at_0_6() {
    let mut attacks = 0;
    let mut four_hits = 0;
    for run in batch() {
        for event in run.attacks() {
            let EventKind::AirAttack {
                air_group,
                task_force,
                ship,
                weapon,
                shots,
                chance,
                expected,
                hits,
            } = event.kind
            else {
                unreachable!()
            };
            assert_eq!(
                (air_group, task_force, ship, weapon),
                (BOMBERS, ENEMY_FORCE, CARRIER_B, Weapon::Bomb)
            );
            assert_eq!(
                (shots, chance, expected),
                (Fixed::from_int(18), tenths(2), tenths(36))
            );
            assert!(hits == 3 || hits == 4, "seed {}: {hits}", run.seed);
            attacks += 1;
            four_hits += i64::from(hits == 4);
            assert_eq!(
                event.kind.to_string(),
                format!("air_group:2 bombs at ship:3: 18 shots x 0.2 = 3.6 expected, {hits} hits")
            );
            // It cites its launch, and carries the breakdown of its chance.
            let launch = run
                .launches()
                .into_iter()
                .find(|launch| event.causes == vec![Cause::Event(launch.id)]);
            assert!(launch.is_some(), "seed {}", run.seed);
            let breakdown = event.breakdown.as_ref().expect("breakdown");
            assert_eq!(breakdown.result, tenths(36));
        }
    }
    eprintln!("attacks {attacks}, with four hits {four_hits}");
    assert!(attacks > 500);
    assert!(within_four_sigma(four_hits, attacks, 600_000));
}

#[test]
fn a_hit_does_10_points_on_flotation_at_0_75_and_fire_at_0_25_and_carrier_b_is_never_sunk() {
    let mut hits = 0i64;
    let mut flotation = 0i64;
    let mut system_hits = 0i64;
    let mut by_system = [0i64; 5];
    for run in batch() {
        let mut points_total = Fixed::ZERO;
        for attack in run.attacks() {
            let EventKind::AirAttack { hits: expected, .. } = attack.kind else {
                unreachable!()
            };
            let mut counted = 0;
            for event in run.caused_by(attack.id) {
                match event.kind {
                    EventKind::ShipDamaged {
                        round,
                        ship,
                        track,
                        hits: track_hits,
                        points,
                        value_after: _,
                    } => {
                        assert_eq!((round, ship), (0, CARRIER_B));
                        assert_eq!(points, Fixed::from_int(10) * i64::from(track_hits));
                        points_total += points;
                        counted += track_hits;
                        if track == DamageTrack::Flotation {
                            flotation += i64::from(track_hits);
                        }
                    }
                    EventKind::SystemHit {
                        round,
                        ship,
                        system,
                        ..
                    } => {
                        assert_eq!((round, ship), (0, CARRIER_B));
                        system_hits += 1;
                        by_system[ShipSystem::ALL
                            .iter()
                            .position(|s| *s == system)
                            .expect("a system")] += 1;
                    }
                    EventKind::ReportReceived { .. } => {}
                    ref other => panic!("an attack caused {other:?}"),
                }
            }
            assert_eq!(counted, expected, "seed {}", run.seed);
            hits += i64::from(expected);
        }
        // Damage, in total, is 10 a hit. Two attacks do 80 points at most.
        let carrier = run.ship(3);
        assert_eq!(
            carrier.flotation + carrier.fire,
            points_total,
            "seed {}",
            run.seed
        );
        assert_eq!(
            points_total,
            Fixed::from_int(10) * i64::from(run.hits()),
            "seed {}",
            run.seed
        );
        assert!(points_total <= Fixed::from_int(80));
        assert!(!carrier.sunk);
        assert!(run
            .of(|k| matches!(k, EventKind::ShipSunk { .. }))
            .is_empty());
    }
    eprintln!(
        "hits {hits}: flotation {flotation}, system hits {system_hits}, by system {by_system:?}"
    );
    assert!(within_four_sigma(flotation, hits, 750_000));
    // A hit is a system hit with chance 0.3; the system with 0.5, 0.2, 0.1
    // and 0.2 of those (flight deck, propulsion, fire control, main battery).
    assert!(within_four_sigma(system_hits, hits, 300_000));
    let shares = [
        (ShipSystem::FlightDeck, 150_000),
        (ShipSystem::Propulsion, 60_000),
        (ShipSystem::FireControl, 30_000),
        (ShipSystem::MainBattery, 60_000),
    ];
    for (system, share) in shares {
        let slot = ShipSystem::ALL
            .iter()
            .position(|s| *s == system)
            .expect("a system");
        assert!(
            within_four_sigma(by_system[slot], hits, share),
            "{}: {} of {hits}",
            system.name(),
            by_system[slot]
        );
    }
    // Nothing hits the torpedo mounts: not in the table.
    assert_eq!(by_system[3], 0);
}

#[test]
fn no_aircraft_is_lost_and_only_carrier_b_is_ever_damaged() {
    for run in batch() {
        assert_eq!(run.bombers_ready, 18, "seed {}", run.seed);
        assert_eq!(run.scouts_ready, 8, "seed {}", run.seed);
        for id in [1, 2, 4, 5] {
            let ship = run.ship(id);
            assert_eq!(ship.flotation, Fixed::ZERO, "seed {} ship {id}", run.seed);
            assert_eq!(ship.fire, Fixed::ZERO);
            assert_eq!(ship.systems, ALL_WORKING);
            assert!(!ship.sunk);
        }
        // A flight deck hit on carrier B is all that can change its systems:
        // carrier B has no guns, so it has no fire control or main battery.
        assert!(run
            .of(|k| matches!(k, EventKind::AirGroupLost { .. }))
            .is_empty());
    }
}

#[test]
fn a_strike_that_attacks_reports_the_force_and_one_that_finds_nothing_sends_no_report() {
    for run in batch() {
        for attack in run.attacks() {
            let reports: Vec<&Event> = run
                .caused_by(attack.id)
                .into_iter()
                .filter(|event| matches!(event.kind, EventKind::ReportReceived { .. }))
                .collect();
            assert_eq!(reports.len(), 1, "seed {}", run.seed);
            let EventKind::ReportReceived {
                side,
                contact,
                hex,
                created,
                ..
            } = reports[0].kind
            else {
                unreachable!()
            };
            // It refreshes the contact, and is after every damage event.
            assert_eq!(
                (side, contact, created),
                (Side::Allied, ContactId::new(1), false)
            );
            assert!(hex == TARGET || TARGET.neighbors().contains(&hex));
            assert!(run
                .caused_by(attack.id)
                .iter()
                .all(|event| event.id <= reports[0].id));
        }
        for nothing in run.found_nothing() {
            assert!(run.caused_by(nothing.id).is_empty(), "seed {}", run.seed);
            let launch = match nothing.causes[..] {
                [Cause::Event(launch)] => launch,
                _ => panic!("found nothing cites its launch"),
            };
            // No report cites the launch of a strike that found nothing.
            assert!(run
                .caused_by(launch)
                .iter()
                .all(|event| matches!(event.kind, EventKind::StrikeFoundNothing { .. })));
        }
    }
}

// ---------------------------------------------------------------------
// Properties.
// ---------------------------------------------------------------------

#[test]
fn every_damage_event_is_caused_by_an_attack_and_every_attack_by_a_launch() {
    for run in batch() {
        for event in run.of(|k| {
            matches!(
                k,
                EventKind::ShipDamaged { .. } | EventKind::SystemHit { .. }
            )
        }) {
            let [Cause::Event(cause)] = event.causes[..] else {
                panic!("damage cites one event: {:?}", event.causes);
            };
            assert!(
                matches!(run.event(cause).kind, EventKind::AirAttack { .. }),
                "seed {}",
                run.seed
            );
        }
        for attack in run.attacks() {
            let [Cause::Event(cause)] = attack.causes[..] else {
                panic!("an attack cites one event");
            };
            assert!(matches!(
                run.event(cause).kind,
                EventKind::StrikeLaunched { .. }
            ));
        }
    }
}

#[test]
fn the_aircraft_flown_never_exceed_the_group_and_no_track_is_above_100() {
    for run in batch() {
        for (_, aircraft) in run.flown() {
            assert!(aircraft <= 8);
        }
        for launch in run.launches() {
            let EventKind::StrikeLaunched { aircraft, .. } = launch.kind else {
                unreachable!()
            };
            assert!(aircraft <= 18);
        }
        for event in run.of(|k| matches!(k, EventKind::ShipDamaged { .. })) {
            let EventKind::ShipDamaged { value_after, .. } = event.kind else {
                unreachable!()
            };
            assert!(value_after <= Fixed::from_int(100));
        }
        for (_, ship) in &run.ships {
            assert!(ship.flotation <= Fixed::from_int(100));
            assert!(ship.fire <= Fixed::from_int(100));
        }
    }
}

#[test]
fn within_a_pulse_search_comes_before_the_decision_and_the_decision_before_the_attack() {
    for run in batch() {
        for pulse in [Pulse::Morning, Pulse::Afternoon] {
            let ids = |keep: fn(&EventKind) -> bool| -> Vec<EventId> {
                run.events
                    .iter()
                    .filter(|event| event.pulse == Some(pulse) && keep(&event.kind))
                    .map(|event| event.id)
                    .collect()
            };
            let search = ids(|k| {
                matches!(
                    k,
                    EventKind::SearchFlown { .. } | EventKind::SearchAttempt { .. }
                )
            });
            let launches = ids(|k| matches!(k, EventKind::StrikeLaunched { .. }));
            let landings = ids(|k| {
                matches!(
                    k,
                    EventKind::AirAttack { .. } | EventKind::StrikeFoundNothing { .. }
                )
            });
            let last_search = search.iter().max().copied();
            if let (Some(search), Some(launch)) = (last_search, launches.first()) {
                assert!(search < *launch, "seed {}", run.seed);
            }
            if let (Some(launch), Some(landing)) = (launches.last(), landings.first()) {
                assert!(launch < landing, "seed {}", run.seed);
            }
        }
    }
}

#[test]
fn a_replay_reproduces_every_daily_hash() {
    let base = load();
    let mut all_hashes: Vec<Vec<u64>> = Vec::new();
    for seed in [1u64, 2, 77, 1942] {
        let hashes: Vec<Vec<u64>> = (0..2)
            .map(|_| {
                let mut run = base.with_seed(seed).expect("a seed can be set");
                run.run_to_end().expect("the run resolves");
                run.world.day_hashes().to_vec()
            })
            .collect();
        assert_eq!(hashes[0].len(), 1);
        assert_eq!(hashes[0], hashes[1], "seed {seed}");
        all_hashes.push(hashes[0].clone());
    }
    // The seed matters.
    assert!(all_hashes.windows(2).any(|pair| pair[0] != pair[1]));
    // The batch holds the same hashes as a fresh run of the same seed.
    let mut again = base.with_seed(77).expect("a seed can be set");
    again.run_to_end().expect("the run resolves");
    assert_eq!(batch()[76].seed, 77);
    assert_eq!(batch()[76].hashes, again.world.day_hashes());
    // The file's own seed gives the same hashes as a seeded copy.
    let mut own = load();
    own.run_to_end().expect("the run resolves");
    let mut copy = base.with_seed(1942).expect("a seed can be set");
    copy.run_to_end().expect("the run resolves");
    assert_eq!(own.world.day_hashes(), copy.world.day_hashes());
}

// ---------------------------------------------------------------------
// The variants: the flight deck, and more bombers.
// ---------------------------------------------------------------------

/// Copies the scenario and its data into a folder under the build directory,
/// makes each exact, asserted-unique text replacement in the copied scenario
/// file, and loads it. Nothing in the repository's data folder is touched.
fn variant(tag: &str, edits: &[(&str, &str)]) -> Scenario {
    let data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data");
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("strike_{tag}"));
    for folder in ["scenarios", "map", "reference"] {
        fs::create_dir_all(root.join(folder)).expect("make the variant folder");
    }
    for file in [
        "map/placeholder_search.toml",
        "reference/ship_classes.csv",
        "reference/port_sizes.csv",
        "reference/aircraft_types.csv",
        "reference/search.toml",
        "reference/surface_combat.toml",
        "reference/air_combat.toml",
    ] {
        fs::copy(data.join(file), root.join(file)).expect("copy a data file");
    }
    let mut text =
        fs::read_to_string(data.join("scenarios/05b_strike.toml")).expect("read the scenario");
    for (original, changed) in edits {
        assert_eq!(
            text.matches(original).count(),
            1,
            "the text {original:?} appears exactly once"
        );
        text = text.replace(original, changed);
    }
    fs::write(root.join("scenarios/05b_strike.toml"), text).expect("write the variant");
    Scenario::load(&root.join("scenarios/05b_strike.toml")).expect("the variant loads")
}

fn deck_variant(state: &str) -> Scenario {
    variant(
        &format!("deck_{state}"),
        &[(
            "name = \"Placeholder carrier A\"\n",
            &format!("name = \"Placeholder carrier A\"\nflight_deck = \"{state}\"\n"),
        )],
    )
}

fn batch_of(base: &Scenario, seeds: u64) -> Vec<Run> {
    (1..=seeds).map(|seed| run_of(base, seed)).collect()
}

#[test]
fn a_working_flight_deck_flies_8_scouts_and_launches_18_bombers() {
    let working = deck_variant("working");
    assert_eq!(
        working.world.umpire().ships[&CARRIER_A].systems.flight_deck,
        SystemState::Working
    );
    let runs = batch_of(&working, 50);
    for (run, original) in runs.iter().zip(batch()) {
        // The same as the file's own scenario: a working deck is the default.
        assert_eq!(run.hashes, original.hashes);
    }
}

#[test]
fn a_degraded_flight_deck_flies_4_scouts_at_0_254647_and_launches_9_bombers_for_1_8_expected_hits()
{
    let degraded = deck_variant("degraded");
    assert_eq!(
        degraded.world.umpire().ships[&CARRIER_A]
            .systems
            .flight_deck,
        SystemState::Degraded
    );
    let runs = batch_of(&degraded, SEEDS);
    let mut launches = 0;
    let mut attacks = 0;
    let mut seen = BTreeSet::new();
    for run in &runs {
        // 57.295779 x 4 x 20 / 18000 = 4583.66232 / 18000.
        assert_eq!(
            run.flown(),
            vec![(Pulse::Morning, 4), (Pulse::Afternoon, 4)],
            "seed {}",
            run.seed
        );
        for attempt in run.attempts() {
            assert_eq!(attempt.2, millionths(254_647));
        }
        for launch in run.launches() {
            let EventKind::StrikeLaunched { aircraft, .. } = launch.kind else {
                unreachable!()
            };
            assert_eq!(aircraft, 9, "seed {}", run.seed);
            launches += 1;
        }
        for attack in run.attacks() {
            let EventKind::AirAttack {
                shots,
                expected,
                hits,
                ..
            } = attack.kind
            else {
                unreachable!()
            };
            assert_eq!((shots, expected), (Fixed::from_int(9), tenths(18)));
            assert!(hits == 1 || hits == 2, "seed {}: {hits}", run.seed);
            seen.insert(hits);
            attacks += 1;
        }
        // The deck of carrier A is hit by nobody: it stays degraded.
        assert_eq!(run.ship(1).systems[4], SystemState::Degraded);
        assert_eq!(run.bombers_ready, 18);
    }
    eprintln!("degraded: launches {launches}, attacks {attacks}, hits seen {seen:?}");
    assert!(attacks > 0);
    assert_eq!(seen, BTreeSet::from([1, 2]));
    // The launches follow the sightings, whose chance is now 0.254647.
    let sighted_morning = count(
        runs.iter()
            .filter(|run| run.sighted_in(Pulse::Morning))
            .count(),
    );
    assert!(within_four_sigma(sighted_morning, 1000, 254_647));
}

#[test]
fn a_flight_deck_that_is_out_flies_no_search_and_launches_no_strike() {
    let out = deck_variant("out");
    assert_eq!(
        out.world.umpire().ships[&CARRIER_A].systems.flight_deck,
        SystemState::Out
    );
    for run in batch_of(&out, SEEDS) {
        assert!(run.flown().is_empty(), "seed {}", run.seed);
        assert!(run.attempts().is_empty(), "seed {}", run.seed);
        assert!(run.launches().is_empty(), "seed {}", run.seed);
        assert!(run.attacks().is_empty());
        assert!(run.found_nothing().is_empty());
        // The day records no air event at all, and the enemy is untouched.
        assert!(run
            .of(|k| matches!(
                k,
                EventKind::ReportReceived { .. }
                    | EventKind::SearchFlown { .. }
                    | EventKind::SearchAttempt { .. }
                    | EventKind::ShipDamaged { .. }
            ))
            .is_empty());
        assert_eq!(run.ship(3).systems, ALL_WORKING);
    }
}

#[test]
fn more_bombers_never_mean_fewer_expected_hits() {
    // 28 bombers and 8 scouts fill the carrier's 36: 28 x 0.2 = 5.6.
    let more = variant("bombers_28", &[("ready = 18", "ready = 28")]);
    assert_eq!(more.world.umpire().air_groups[&BOMBERS].ready, 28);
    let expected_of = |scenario: &Scenario| -> Vec<Fixed> {
        let mut found = Vec::new();
        for run in batch_of(scenario, 100) {
            for attack in run.attacks() {
                let EventKind::AirAttack {
                    expected, shots, ..
                } = attack.kind
                else {
                    unreachable!()
                };
                found.push(expected);
                assert!(shots <= Fixed::from_int(28));
            }
        }
        found
    };
    let baseline = expected_of(&load());
    let with_more = expected_of(&more);
    assert!(!baseline.is_empty() && !with_more.is_empty());
    assert!(baseline.iter().all(|expected| *expected == tenths(36)));
    assert!(with_more.iter().all(|expected| *expected == tenths(56)));
    // Degraded (9 bombers), working (18) and full (28): 1.8, 3.6, 5.6.
    assert!(tenths(18) < tenths(36) && tenths(36) < tenths(56));
}
