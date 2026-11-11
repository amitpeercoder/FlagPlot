//! Scenario 5c: a strike with an escort meets a fighter patrol and the guns
//! of the force it attacks.
//!
//! Stage 5c of scenario 5. It is scenario 5b with fighters on both carriers
//! and anti-aircraft guns on the Japanese ships. It proves escort against
//! patrol, anti-aircraft fire and aircraft losses.
//!
//! The data is placeholder data: see `data/scenarios/05c_air_combat.toml`.
//! The rule is `docs/decisions/0027-fighters-and-anti-aircraft-fire.md`, on
//! top of `docs/decisions/0026-air-strikes.md`.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. The tests below assert these
//! numbers. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass.
//!
//! ## The setting
//!
//! As scenario 5b: carrier A, ship 1, lies stopped at (4,0), and carrier B,
//! ship 3, with cruisers 4 and 5, lies stopped at (9,0), 200 nm east. The
//! search chance, when a strike is launched and whether it finds task force
//! 2 are exactly as in scenario 5b: 0.509295 a pulse, a launch in every
//! daylight pulse in which the Allied side holds a contact reported in that
//! pulse or the one before, and an attack only if the strike flew to (9,0).
//!
//! What is new:
//!
//! | Air group | Aboard    | Aircraft      | Mission |
//! | --------- | --------- | ------------- | ------- |
//! | 1         | carrier A | 8 scouts      | search |
//! | 2         | carrier A | 18 dive bombers | naval strike |
//! | 3         | carrier A | 12 fighters   | combat air patrol, share 0.5 |
//! | 4         | carrier B | 12 fighters   | combat air patrol, share 1 |
//!
//! Carrier A holds 48 aircraft and has 38. Carrier B has 20 anti-aircraft
//! shots and each cruiser 8: 36 in all while all three are afloat. Nobody
//! attacks carrier A's force, so its guns never fire. (First written as
//! "carrier A's force has none": carrier A is of the same class as carrier
//! B and has 20. No figure changed.)
//!
//! A fighter's shot hits with chance 0.3 and an anti-aircraft shot with 0.05.
//! A bomb hits with chance 0.2 and does 10 points.
//!
//! ## The rule, for any attack
//!
//! Write F for the aircraft air group 3 can fly, P for those air group 4
//! can fly, and B for those air group 2 can fly, all at the start of the
//! pulse. A group aboard a carrier flies all its aircraft, half of them
//! rounded down if the flight deck is degraded, and none if it is out.
//!
//! - Group 3 keeps F x 0.5, rounded down, overhead. The rest, E, escort the
//!   strike. All of B go.
//! - The escorts fire E shots at the patrol.
//! - The patrol fires at the escorts the smaller of P and E shots, and the
//!   rest of P at the bombers.
//! - Then the ships afloat in task force 2 fire their anti-aircraft shots at
//!   the bombers left.
//! - Then the bombers left attack carrier B: one shot each.
//!
//! Each of these is shots x chance expected hits: the whole part, plus one
//! more with a chance equal to the fraction. A hit is an aircraft lost, and
//! a group cannot lose more than it has.
//!
//! A strike that finds nothing meets no fighters and no guns, and loses
//! nothing.
//!
//! ## The first attack of a run
//!
//! Until the first attack every group is whole and every flight deck works:
//! F = 12, so 6 stay over carrier A and E = 6. P = 12. B = 18.
//!
//! | Step                       | Shots | Chance | Expected | Lost            |
//! | -------------------------- | ----- | ------ | -------- | --------------- |
//! | Escorts at the patrol      | 6     | 0.3    | 1.8      | 1 or 2 of group 4 |
//! | Patrol at the escorts      | 6     | 0.3    | 1.8      | 1 or 2 of group 3 |
//! | Patrol at the bombers      | 6     | 0.3    | 1.8      | 1 or 2 of group 2 |
//! | Guns at the bombers        | 36    | 0.05   | 1.8      | 1 or 2 of group 2 |
//!
//! Each is 2 with chance 0.8. So 16, 15 or 14 bombers attack:
//!
//! | Bombers attacking | Expected hits | Hits      |
//! | ----------------- | ------------- | --------- |
//! | 16                | 3.2           | 3 or 4    |
//! | 15                | 3             | exactly 3 |
//! | 14                | 2.8           | 2 or 3    |
//!
//! After it group 2 has 16, 15 or 14 aircraft, group 3 has 11 or 10, and
//! group 4 has 11 or 10.
//!
//! Over seeds 1 to 1000, among the runs with an attack, the number in which
//! the first attack costs the patrol 2 is within four standard deviations of
//! 0.8 of them, and likewise the escorts 2, the bombers 2 to fighters, and
//! the bombers 2 to guns.
//!
//! ## A second attack
//!
//! A run has at most two attacks, one in each daylight pulse. The second is
//! worked by the same rule from what each group then has and the state of
//! carrier B's flight deck, which the first attack may have hit. The test
//! works every attack of every run from the counts and flight decks at the
//! start of its pulse.
//!
//! Two attacks do at most 4 + 4 hits, 80 points, so carrier B is never sunk,
//! and the cruisers are never attacked: the guns always fire 36 shots.
//!
//! ## Variants
//!
//! Each differs from the scenario in one order, and the figures are for its
//! first attack.
//!
//! | Variant                                | Escorts at patrol | Patrol at escorts | Patrol at bombers | Guns | Bombers attacking |
//! | -------------------------------------- | ----------------- | ----------------- | ----------------- | ---- | ----------------- |
//! | Group 3 keeps a share of 1: no escort  | none              | none              | 12 shots, 3.6: 3 or 4 | 1.8: 1 or 2 | 12 to 14 |
//! | Group 3 keeps a share of 0: 12 escorts | 12 shots, 3.6: 3 or 4 | 12 shots, 3.6: 3 or 4 | none        | 1.8: 1 or 2 | 16 or 17 |
//! | Group 4 is not on patrol               | none              | none              | none              | 1.8: 1 or 2 | 16 or 17 |
//!
//! # Properties
//!
//! - Every group's aircraft at the end of the day are its aircraft at the
//!   start less the aircraft its loss events record. No group gains
//!   aircraft, and none goes below zero.
//! - Air group 1 and the six fighters over carrier A never lose an aircraft.
//! - No aircraft is lost in a pulse in which no strike attacks.
//! - More escorts never mean more patrol shots at the bombers: with a share
//!   of 0.5 the patrol fires 6 at the bombers, with 0 none, with 1 all 12.
//! - The bombers that attack are the bombers that flew less the bombers
//!   lost to fighters and guns in that attack.
//! - A replay of any seed reproduces the daily hash.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;

use sim_core::events::{AirFirer, AirTarget, Cause, Event, EventKind, Weapon};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, ContactId, EntityRef, EventId, OrderId, ShipId, TaskForceId};
use sim_core::rng::{RngKey, RngStream, Subsystem};
use sim_core::state::{ShipLocation, ShipSystem, SystemState};
use sim_core::time::Pulse;
use sim_data::Scenario;

const ALLIED_FORCE: TaskForceId = TaskForceId::new(1);
const ENEMY_FORCE: TaskForceId = TaskForceId::new(2);
const SCOUTS: AirGroupId = AirGroupId::new(1);
const BOMBERS: AirGroupId = AirGroupId::new(2);
const FIGHTERS_A: AirGroupId = AirGroupId::new(3);
const FIGHTERS_B: AirGroupId = AirGroupId::new(4);
const CARRIER_A: ShipId = ShipId::new(1);
const CARRIER_B: ShipId = ShipId::new(3);
const SEEDS: u64 = 1000;
const HOME: Hex = Hex::new(4, 0);
const TARGET: Hex = Hex::new(9, 0);

fn scenario_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/05c_air_combat.toml")
}

fn load() -> Scenario {
    Scenario::load(&scenario_path()).expect("scenario 5c loads")
}

fn millionths(value: i64) -> Fixed {
    Fixed::from_raw(value)
}

fn tenths(value: i64) -> Fixed {
    Fixed::from_ratio(value, 10)
}

fn hundredths(value: i64) -> Fixed {
    Fixed::from_ratio(value, 100)
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

fn int(value: u32) -> Fixed {
    Fixed::from_int(i64::from(value))
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
    /// The aircraft ready at the end of the day of groups 1 to 4.
    ready: [u32; 4],
    allied_force_at: Hex,
    enemy_force_at: Hex,
    /// The patrol share of group 3 and the mission of group 4, as set.
    share_of_3: Fixed,
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
        ready: [SCOUTS, BOMBERS, FIGHTERS_A, FIGHTERS_B]
            .map(|group| truth.air_groups[&group].ready),
        allied_force_at: truth.task_forces[&ALLIED_FORCE].position(),
        enemy_force_at: truth.task_forces[&ENEMY_FORCE].position(),
        share_of_3: truth.air_groups[&FIGHTERS_A].patrol_share,
    }
}

fn batch() -> &'static Vec<Run> {
    static BATCH: OnceLock<Vec<Run>> = OnceLock::new();
    BATCH.get_or_init(|| {
        let base = load();
        (1..=SEEDS).map(|seed| run_of(&base, seed)).collect()
    })
}

fn batch_of(base: &Scenario, seeds: u64) -> Vec<Run> {
    (1..=seeds).map(|seed| run_of(base, seed)).collect()
}

const ALL_WORKING: [SystemState; 5] = [SystemState::Working; 5];

// ---------------------------------------------------------------------
// Working every attack of a run from the counts at the start of its pulse.
// ---------------------------------------------------------------------

/// What differs between the scenario and its variants.
#[derive(Clone, Copy)]
struct Config {
    /// Group 3's patrol share, in tenths.
    share_tenths: u32,
    /// Whether group 4 is on combat air patrol.
    group_4_on_patrol: bool,
}

const SCENARIO: Config = Config {
    share_tenths: 5,
    group_4_on_patrol: true,
};

/// One attack as the rule works it, with what it found in the events.
struct Worked {
    pulse: Pulse,
    /// F, E, P and B of the hand calculation.
    flyable_3: u32,
    escorts: u32,
    patrol: u32,
    bombers: u32,
    /// The hits of each roll, or `None` for a roll with no shots.
    escorts_at_patrol: Option<u32>,
    patrol_at_escorts: Option<u32>,
    patrol_at_bombers: Option<u32>,
    shots_at_bombers: u32,
    guns: Option<u32>,
    bombers_attacking: u32,
    attack_hits: u32,
    /// The aircraft of groups 2, 3 and 4 afterwards.
    after: [u32; 3],
    /// Whether carrier B's flight deck was degraded or out at the start.
    deck_b: SystemState,
}

/// The hits a roll of `shots` at `chance` gives from its stream: the whole
/// part of shots x chance, plus one with chance equal to the fraction.
fn hits_from(stream: &mut RngStream, shots: u32, chance: Fixed) -> u32 {
    let expected = int(shots) * chance;
    let whole = expected.trunc();
    let fraction = expected - Fixed::from_int(whole);
    u32::try_from(whole).expect("small") + u32::from(stream.chance(fraction))
}

fn stream_of(seed: u64, pulse: Pulse, entity: EntityRef, other: EntityRef) -> RngStream {
    RngKey {
        seed,
        day: 1,
        pulse: Some(pulse),
        subsystem: Subsystem::AirCombat,
        entity,
        other: Some(other),
    }
    .stream()
}

/// Flyable aircraft of a group aboard a carrier by the state of its deck:
/// all, half rounded down, none.
fn flyable(ready: u32, deck: SystemState) -> u32 {
    match deck {
        SystemState::Working => ready,
        SystemState::Degraded => ready / 2,
        SystemState::Out => 0,
    }
}

fn shots_text(shots: u32, chance: &str, expected: Fixed, hits: u32) -> String {
    format!(
        "{shots} shots x {chance} = {expected} expected, {hits} {}",
        if hits == 1 { "hit" } else { "hits" }
    )
}

/// Works every attack of `run` by the rule of the hand calculation from the
/// counts and flight decks at the start of its pulse, asserts every event of
/// the air combat step against it, and returns what was worked. The counts
/// are carried from pulse to pulse from the loss events of the pulses
/// before.
fn work_the_run(run: &Run, config: Config) -> Vec<Worked> {
    let seed = run.seed;
    let mut ready = [18u32, 12, 12];
    let mut deck_b = SystemState::Working;
    let mut worked = Vec::new();
    let events = &run.events;
    for (index, launch) in events.iter().enumerate() {
        match launch.kind {
            EventKind::SystemHit {
                ship,
                system: ShipSystem::FlightDeck,
                after,
                ..
            } if ship == CARRIER_B => deck_b = after,
            _ => {}
        }
        let EventKind::StrikeLaunched {
            air_group,
            aircraft: launched,
            ..
        } = launch.kind
        else {
            continue;
        };
        assert_eq!(air_group, BOMBERS);
        let pulse = pulse_of(launch);
        // The events of the step: the rest of the pulse's events.
        let step: Vec<&Event> = events[index + 1..]
            .iter()
            .take_while(|event| event.pulse == Some(pulse))
            .collect();

        // F, E, P and B at the start of the pulse. Carrier A is never hit.
        let b = ready[0];
        assert_eq!(launched, b, "seed {seed}: all the bombers that can fly go");
        let flown_3 = ready[1];
        let patrol_3 = flown_3 * config.share_tenths / 10;
        let e = flown_3 - patrol_3;
        let p = if config.group_4_on_patrol {
            flyable(ready[2], deck_b)
        } else {
            0
        };

        let kinds = |keep: fn(&EventKind) -> bool| -> Vec<&Event> {
            step.iter().copied().filter(|e| keep(&e.kind)).collect()
        };
        let sent = kinds(|k| matches!(k, EventKind::EscortsSent { .. }));
        let fights = kinds(|k| matches!(k, EventKind::AirFight { .. }));
        let guns = kinds(|k| matches!(k, EventKind::AntiAircraftFire { .. }));
        let lost = kinds(|k| matches!(k, EventKind::AircraftLost { .. }));
        let attacked = kinds(|k| matches!(k, EventKind::AirAttack { .. }));
        let nothing = kinds(|k| matches!(k, EventKind::StrikeFoundNothing { .. }));

        // The escorts go if there are any, whether or not the strike
        // finds anything: group 3 with the strike of group 2.
        if e > 0 {
            assert_eq!(sent.len(), 1, "seed {seed}");
            assert_eq!(
                sent[0].kind,
                EventKind::EscortsSent {
                    air_group: FIGHTERS_A,
                    with_air_group: BOMBERS,
                    aircraft: e
                }
            );
            assert_eq!(
                sent[0].kind.to_string(),
                format!("air_group:3 sent {e} escorts with air_group:2")
            );
            assert_eq!(
                sent[0].causes,
                vec![
                    Cause::Order(OrderId::new(4)),
                    Cause::Order(OrderId::new(5)),
                    Cause::Event(launch.id)
                ],
                "seed {seed}"
            );
        } else {
            assert!(sent.is_empty(), "seed {seed}");
        }

        // A strike that finds nothing meets nobody and loses nothing.
        if attacked.is_empty() {
            assert_eq!(nothing.len(), 1, "seed {seed}");
            assert_eq!(
                nothing[0].causes,
                vec![Cause::Event(launch.id)],
                "seed {seed}"
            );
            assert!(
                fights.is_empty() && guns.is_empty() && lost.is_empty(),
                "seed {seed}: a strike that finds nothing fights nobody"
            );
            continue;
        }
        assert!(nothing.is_empty(), "seed {seed}");
        assert_eq!(attacked.len(), 1, "seed {seed}");

        // The fights, worked by decision 6 on the counts at the start.
        let chance = tenths(3);
        let mut expected_fights: Vec<(AirGroupId, AirGroupId, AirFirer, AirTarget, u32)> =
            Vec::new();
        if e > 0 && p > 0 {
            expected_fights.push((
                FIGHTERS_A,
                FIGHTERS_B,
                AirFirer::Escorts,
                AirTarget::Patrol,
                e,
            ));
        }
        let at_escorts = p.min(e);
        if p > 0 && at_escorts > 0 {
            expected_fights.push((
                FIGHTERS_B,
                FIGHTERS_A,
                AirFirer::Patrol,
                AirTarget::Escorts,
                at_escorts,
            ));
        }
        let at_bombers = p - at_escorts;
        if at_bombers > 0 {
            expected_fights.push((
                FIGHTERS_B,
                BOMBERS,
                AirFirer::Patrol,
                AirTarget::Bombers,
                at_bombers,
            ));
        }
        assert_eq!(fights.len(), expected_fights.len(), "seed {seed}");

        // Replay the streams the decision names: the escorts' fire on
        // (the first escort group, the task force attacked); the patrol's
        // on (the first patrol group, the raid's first group), at the
        // escorts and then at the bombers.
        let mut escort_stream = stream_of(
            seed,
            pulse,
            EntityRef::AirGroup(FIGHTERS_A),
            EntityRef::TaskForce(ENEMY_FORCE),
        );
        let mut patrol_stream = stream_of(
            seed,
            pulse,
            EntityRef::AirGroup(FIGHTERS_B),
            EntityRef::AirGroup(BOMBERS),
        );
        let (mut h_escorts, mut h_patrol_e, mut h_patrol_b) = (None, None, None);
        let mut lost_expected: Vec<(AirGroupId, u32, u32)> = Vec::new();
        let mut now = ready;
        let mut fight_events: Vec<EventId> = Vec::new();
        let mut loss_causes: Vec<EventId> = Vec::new();
        for ((firer, target, firing, at, shots), event) in expected_fights.iter().zip(&fights) {
            let hits = match at {
                AirTarget::Patrol => hits_from(&mut escort_stream, *shots, chance),
                _ => hits_from(&mut patrol_stream, *shots, chance),
            };
            let expected = int(*shots) * chance;
            assert_eq!(
                event.kind,
                EventKind::AirFight {
                    firer: *firer,
                    target: *target,
                    firing: *firing,
                    at: *at,
                    shots: int(*shots),
                    chance,
                    expected,
                    hits
                },
                "seed {seed}"
            );
            let who = match firing {
                AirFirer::Escorts => "escorts",
                AirFirer::Patrol => "patrol",
            };
            let what = match at {
                AirTarget::Patrol => "patrol",
                AirTarget::Escorts => "escorts",
                AirTarget::Bombers => "bombers",
            };
            assert_eq!(
                event.kind.to_string(),
                format!(
                    "air_group:{} {who} at the {what} of air_group:{}: {}",
                    firer.raw(),
                    target.raw(),
                    shots_text(*shots, "0.3", expected, hits)
                ),
                "seed {seed}"
            );
            // The causes: the launch, and the escorts when they are in it.
            let mut causes = vec![Cause::Event(launch.id)];
            if *firing == AirFirer::Escorts || *at == AirTarget::Escorts {
                causes.push(Cause::Event(sent[0].id));
            }
            assert_eq!(event.causes, causes, "seed {seed}");
            let breakdown = event.breakdown.as_ref().expect("breakdown");
            assert_eq!(breakdown.result, expected);
            assert_eq!(
                breakdown
                    .factors
                    .iter()
                    .map(|f| (f.name, f.value))
                    .collect::<Vec<_>>(),
                vec![("shots", int(*shots)), ("hit_chance", chance)]
            );
            fight_events.push(event.id);
            // A group cannot lose more than it has.
            match at {
                AirTarget::Patrol => {
                    h_escorts = Some(hits);
                    let taken = hits.min(p);
                    now[2] -= taken;
                    if taken > 0 {
                        lost_expected.push((FIGHTERS_B, taken, now[2]));
                        loss_causes.push(event.id);
                    }
                }
                AirTarget::Escorts => {
                    h_patrol_e = Some(hits);
                    let taken = hits.min(e);
                    now[1] -= taken;
                    if taken > 0 {
                        lost_expected.push((FIGHTERS_A, taken, now[1]));
                        loss_causes.push(event.id);
                    }
                }
                AirTarget::Bombers => {
                    h_patrol_b = Some(hits);
                    let taken = hits.min(b);
                    now[0] -= taken;
                    if taken > 0 {
                        lost_expected.push((BOMBERS, taken, now[0]));
                        loss_causes.push(event.id);
                    }
                }
            }
        }

        // The guns: every ship afloat in task force 2 (the carrier's 20 and
        // two cruisers' 8 each: none is ever attacked, so none is sunk),
        // at the bombers left.
        let left_after_fighters = now[0] - (ready[0] - b);
        assert_eq!(left_after_fighters, now[0], "the bombers are group 2's own");
        let bombers_left = b - (ready[0] - now[0]);
        let mut gun_hits = None;
        if bombers_left > 0 {
            assert_eq!(guns.len(), 1, "seed {seed}");
            let mut stream = stream_of(
                seed,
                pulse,
                EntityRef::Ship(CARRIER_B),
                EntityRef::AirGroup(BOMBERS),
            );
            let hits = hits_from(&mut stream, 36, hundredths(5));
            let expected = int(36) * hundredths(5);
            assert_eq!(expected, tenths(18));
            assert_eq!(
                guns[0].kind,
                EventKind::AntiAircraftFire {
                    task_force: ENEMY_FORCE,
                    air_group: BOMBERS,
                    shots: int(36),
                    chance: hundredths(5),
                    expected,
                    hits
                },
                "seed {seed}"
            );
            assert_eq!(
                guns[0].kind.to_string(),
                format!(
                    "task_force:2 guns at the raid of air_group:2: {}",
                    shots_text(36, "0.05", expected, hits)
                ),
                "seed {seed}"
            );
            assert_eq!(guns[0].causes, vec![Cause::Event(launch.id)]);
            assert_eq!(
                guns[0].breakdown.as_ref().expect("breakdown").result,
                expected
            );
            gun_hits = Some(hits);
            let taken = hits.min(bombers_left);
            now[0] -= taken;
            if taken > 0 {
                lost_expected.push((BOMBERS, taken, now[0]));
                loss_causes.push(guns[0].id);
            }
        } else {
            assert!(guns.is_empty(), "seed {seed}");
        }

        // The losses: one per group and cause, in the order of their causes.
        assert_eq!(lost.len(), lost_expected.len(), "seed {seed}");
        for ((group, aircraft, remaining), (event, cause)) in
            lost_expected.iter().zip(lost.iter().zip(&loss_causes))
        {
            assert_eq!(
                event.kind,
                EventKind::AircraftLost {
                    air_group: *group,
                    aircraft: *aircraft,
                    remaining: *remaining
                },
                "seed {seed}"
            );
            assert_eq!(
                event.kind.to_string(),
                format!(
                    "air_group:{} lost {aircraft} aircraft, {remaining} left",
                    group.raw()
                )
            );
            assert_eq!(event.causes, vec![Cause::Event(*cause)], "seed {seed}");
        }

        // The attack is of the bombers left, one shot each, at carrier B.
        let bombers_attacking = now[0];
        assert_eq!(bombers_attacking, b - (ready[0] - now[0]));
        assert!(
            bombers_attacking > 0,
            "seed {seed}: fighters and guns never take all 18"
        );
        let mut stream = stream_of(
            seed,
            pulse,
            EntityRef::AirGroup(BOMBERS),
            EntityRef::TaskForce(ENEMY_FORCE),
        );
        let attack_hits = hits_from(&mut stream, bombers_attacking, tenths(2));
        let attack_expected = int(bombers_attacking) * tenths(2);
        assert_eq!(
            attacked[0].kind,
            EventKind::AirAttack {
                air_group: BOMBERS,
                task_force: ENEMY_FORCE,
                ship: CARRIER_B,
                weapon: Weapon::Bomb,
                shots: int(bombers_attacking),
                chance: tenths(2),
                expected: attack_expected,
                hits: attack_hits
            },
            "seed {seed}"
        );
        assert_eq!(
            attacked[0].kind.to_string(),
            format!(
                "air_group:2 bombs at ship:3: {bombers_attacking} shots x 0.2 = {attack_expected} expected, {attack_hits} {}",
                if attack_hits == 1 { "hit" } else { "hits" }
            )
        );
        // It cites its launch and the losses of the bombers.
        let bomber_losses: Vec<Cause> = lost
            .iter()
            .filter(|event| {
                matches!(event.kind, EventKind::AircraftLost { air_group, .. } if air_group == BOMBERS)
            })
            .map(|event| Cause::Event(event.id))
            .collect();
        let mut causes = vec![Cause::Event(launch.id)];
        causes.extend(bomber_losses);
        assert_eq!(attacked[0].causes, causes, "seed {seed}");
        // Whatever the attack found, the order of the log is fixed.
        let mut order: Vec<EventId> = vec![launch.id];
        order.extend(sent.iter().map(|e| e.id));
        order.extend(fights.iter().map(|e| e.id));
        order.extend(guns.iter().map(|e| e.id));
        order.extend(lost.iter().map(|e| e.id));
        order.push(attacked[0].id);
        assert!(
            order.windows(2).all(|pair| pair[0] < pair[1]),
            "seed {seed}"
        );

        worked.push(Worked {
            pulse,
            flyable_3: flown_3,
            escorts: e,
            patrol: p,
            bombers: b,
            escorts_at_patrol: h_escorts,
            patrol_at_escorts: h_patrol_e,
            patrol_at_bombers: h_patrol_b,
            shots_at_bombers: at_bombers,
            guns: gun_hits,
            bombers_attacking,
            attack_hits,
            after: now,
            deck_b,
        });
        ready = now;
    }
    // The end of the day: every group's aircraft are those at the start
    // less the losses of its events, and nobody gained.
    assert_eq!(
        [run.ready[1], run.ready[2], run.ready[3]],
        ready,
        "seed {seed}"
    );
    worked
}

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
    let ready = |group: AirGroupId| truth.air_groups[&group].ready;
    assert_eq!(
        [
            ready(SCOUTS),
            ready(BOMBERS),
            ready(FIGHTERS_A),
            ready(FIGHTERS_B)
        ],
        [8, 18, 12, 12]
    );
    // Carrier A holds 48 aircraft and has 38.
    let classes = scenario.world.ref_data();
    let capacity = |ship: ShipId| classes.ship_classes[&truth.ships[&ship].class].aircraft_capacity;
    let guns =
        |ship: u32| classes.ship_classes[&truth.ships[&ShipId::new(ship)].class].anti_aircraft;
    assert_eq!(capacity(CARRIER_A), 48);
    assert_eq!(ready(SCOUTS) + ready(BOMBERS) + ready(FIGHTERS_A), 38);
    // Carrier B has 20 anti-aircraft shots and each cruiser 8: 36.
    assert_eq!([guns(3), guns(4), guns(5)], [20, 8, 8]);
    assert_eq!(guns(3) + guns(4) + guns(5), 36);
    // (The destroyer has none. Carrier A, of the same class as carrier B,
    // has the same 20 shots; nobody attacks it.)
    assert_eq!(guns(2), 0);
    // The figures: a fighter's shot 0.3, a gun's 0.05, a bomb 0.2 doing 10
    // points (100 x 60 / 600).
    let air = classes.air_combat.as_ref().expect("air combat figures");
    assert_eq!(air.fighter_hit_chance, tenths(3));
    assert_eq!(air.anti_aircraft_hit_chance, hundredths(5));
    assert_eq!(air.bomb_hit_chance, tenths(2));
    // The orders: group 3 on patrol at share 0.5, group 4 on patrol at the
    // default of 1 (set by no order). After the day the share stands.
    for run in batch() {
        assert_eq!(run.allied_force_at, HOME, "seed {}", run.seed);
        assert_eq!(run.enemy_force_at, TARGET, "seed {}", run.seed);
        assert_eq!(run.share_of_3, tenths(5));
    }
    let mut scenario = load();
    scenario.run_to_end().expect("runs");
    let truth = scenario.world.umpire();
    assert_eq!(
        truth.air_groups[&FIGHTERS_A].mission,
        sim_core::orders::AirMission::CombatAirPatrol
    );
    assert_eq!(truth.air_groups[&FIGHTERS_A].patrol_share, tenths(5));
    assert_eq!(
        truth.air_groups[&FIGHTERS_B].mission,
        sim_core::orders::AirMission::CombatAirPatrol
    );
    assert_eq!(truth.air_groups[&FIGHTERS_B].patrol_share, Fixed::ONE);
}

#[test]
fn the_search_and_the_launches_are_as_in_scenario_5b() {
    let mut morning = 0;
    let mut afternoon = 0;
    for run in batch() {
        assert_eq!(
            run.flown(),
            vec![(Pulse::Morning, 8), (Pulse::Afternoon, 8)],
            "seed {}",
            run.seed
        );
        for attempt in run.attempts() {
            assert_eq!(attempt.1, Fixed::from_int(200));
            assert_eq!(attempt.2, millionths(509_295), "seed {}", run.seed);
        }
        let morning_sighted = run.sighted_in(Pulse::Morning);
        let afternoon_sighted = run.sighted_in(Pulse::Afternoon);
        assert_eq!(
            run.launched_in(Pulse::Morning).is_some(),
            morning_sighted,
            "seed {}",
            run.seed
        );
        assert_eq!(
            run.launched_in(Pulse::Afternoon).is_some(),
            morning_sighted || afternoon_sighted,
            "seed {}",
            run.seed
        );
        morning += i64::from(morning_sighted);
        afternoon += i64::from(morning_sighted || afternoon_sighted);
        // A strike attacks only if it flew to (9,0): it attacks and the
        // launch finds it, exactly as in 5b.
        for launch in run.launches() {
            let EventKind::StrikeLaunched {
                air_group,
                contact,
                hex,
                distance_nm,
                ..
            } = launch.kind
            else {
                unreachable!()
            };
            assert_eq!((air_group, contact), (BOMBERS, ContactId::new(1)));
            assert!(distance_nm <= Fixed::from_int(240));
            let landed: Vec<&Event> = run
                .events
                .iter()
                .filter(|e| e.causes.contains(&Cause::Event(launch.id)))
                .filter(|e| {
                    matches!(
                        e.kind,
                        EventKind::AirAttack { .. } | EventKind::StrikeFoundNothing { .. }
                    )
                })
                .collect();
            assert_eq!(landed.len(), 1, "seed {}", run.seed);
            assert_eq!(
                matches!(landed[0].kind, EventKind::AirAttack { .. }),
                hex == TARGET,
                "seed {}",
                run.seed
            );
            // The launch cites the order that set the mission.
            assert_eq!(launch.causes[0], Cause::Order(OrderId::new(3)));
        }
    }
    assert!(within_four_sigma(morning, 1000, 509_295), "{morning}");
    assert!(within_four_sigma(afternoon, 1000, 759_209), "{afternoon}");
}

// ---------------------------------------------------------------------
// The first attack.
// ---------------------------------------------------------------------

#[test]
fn the_first_attack_meets_6_escorts_12_patrol_and_36_gun_shots_by_the_table() {
    let mut firsts = 0i64;
    let mut patrol_two = 0i64;
    let mut escorts_two = 0i64;
    let mut bombers_two_fighters = 0i64;
    let mut bombers_two_guns = 0i64;
    let mut attacking = BTreeSet::new();
    for run in batch() {
        let worked = work_the_run(run, SCENARIO);
        let Some(first) = worked.first() else {
            assert!(run.attacks().is_empty());
            continue;
        };
        // Until the first attack every group is whole and every deck works.
        assert_eq!(
            (first.flyable_3, first.escorts, first.patrol, first.bombers),
            (12, 6, 12, 18),
            "seed {}",
            run.seed
        );
        assert_eq!(first.deck_b, SystemState::Working);
        // Each roll is 6 shots at 0.3 (1.8 expected: 1 or 2 hits), and the
        // guns 36 at 0.05 (1.8).
        assert_eq!(first.shots_at_bombers, 6);
        let hits = |roll: Option<u32>| roll.expect("the roll was made");
        let (e_p, p_e, p_b, g) = (
            hits(first.escorts_at_patrol),
            hits(first.patrol_at_escorts),
            hits(first.patrol_at_bombers),
            hits(first.guns),
        );
        for roll in [e_p, p_e, p_b, g] {
            assert!(roll == 1 || roll == 2, "seed {}: {roll}", run.seed);
        }
        // 16, 15 or 14 attack, with 3 or 4, exactly 3, and 2 or 3 hits.
        assert_eq!(first.bombers_attacking, 18 - p_b - g);
        match first.bombers_attacking {
            16 => assert!(first.attack_hits == 3 || first.attack_hits == 4),
            15 => assert_eq!(first.attack_hits, 3),
            14 => assert!(first.attack_hits == 2 || first.attack_hits == 3),
            other => panic!("seed {}: {other} bombers attacked", run.seed),
        }
        attacking.insert(first.bombers_attacking);
        // Afterwards group 2 has what attacked, groups 3 and 4 have 11 or 10.
        assert_eq!(first.after[0], first.bombers_attacking);
        assert_eq!(first.after[1], 12 - p_e);
        assert_eq!(first.after[2], 12 - e_p);
        assert!(matches!(first.after[1], 10 | 11) && matches!(first.after[2], 10 | 11));
        firsts += 1;
        patrol_two += i64::from(e_p == 2);
        escorts_two += i64::from(p_e == 2);
        bombers_two_fighters += i64::from(p_b == 2);
        bombers_two_guns += i64::from(g == 2);
    }
    eprintln!(
        "first attacks {firsts}: patrol 2 {patrol_two}, escorts 2 {escorts_two}, bombers 2 to fighters {bombers_two_fighters}, to guns {bombers_two_guns}; attacking {attacking:?}"
    );
    assert!(firsts > 400);
    for twos in [
        patrol_two,
        escorts_two,
        bombers_two_fighters,
        bombers_two_guns,
    ] {
        assert!(
            within_four_sigma(twos, firsts, 800_000),
            "{twos} of {firsts}"
        );
    }
    assert_eq!(attacking, BTreeSet::from([14, 15, 16]));
}

#[test]
fn every_attack_of_every_run_is_worked_by_the_rule_from_the_counts_at_the_start_of_its_pulse() {
    let mut attacks = 0;
    let mut second_attacks = 0;
    let mut deck_hit = 0;
    for run in batch() {
        // `work_the_run` asserts every event of every attack against the rule.
        let worked = work_the_run(run, SCENARIO);
        assert_eq!(worked.len(), run.attacks().len(), "seed {}", run.seed);
        assert!(worked.len() <= 2);
        attacks += worked.len();
        if let [first, second] = &worked[..] {
            second_attacks += 1;
            assert_eq!(first.pulse, Pulse::Morning);
            assert_eq!(second.pulse, Pulse::Afternoon);
            // The second attack is worked from what the groups had left:
            // the bombers that flew are the 14 to 16 that came home.
            assert_eq!(second.bombers, first.after[0], "seed {}", run.seed);
            assert!(matches!(second.bombers, 14..=16));
            assert_eq!(second.flyable_3, first.after[1]);
        }
        deck_hit += worked
            .iter()
            .filter(|attack| attack.deck_b != SystemState::Working)
            .count();
        for attack in &worked {
            assert!(attack.patrol <= 12);
        }
    }
    eprintln!(
        "attacks {attacks}, second attacks {second_attacks}, with carrier B's deck hit {deck_hit}"
    );
    assert!(attacks > 500);
    assert!(second_attacks > 50);
    // Carrier B's flight deck is hit often enough that the patrol it can
    // fly shrinks in some second attacks.
    assert!(deck_hit > 0);
}

#[test]
fn a_second_attack_meets_the_patrol_carrier_b_can_still_fly_and_the_escorts_left() {
    // A closer look at the second attack of runs where it is not the same
    // as the first: its F, E and P follow the first attack's losses and the
    // state of carrier B's deck.
    let mut seen_deck_states = BTreeSet::new();
    let mut checked = 0;
    for run in batch() {
        let worked = work_the_run(run, SCENARIO);
        let [first, second] = &worked[..] else {
            continue;
        };
        // Group 3 has what it had less the escorts lost, and flies all
        // of it from a deck nobody hit.
        assert_eq!(second.flyable_3, first.after[1], "seed {}", run.seed);
        assert_eq!(second.escorts, second.flyable_3 - second.flyable_3 / 2);
        // Group 4's patrol: all it has, half if the deck was hit once, none
        // if twice.
        let kept = first.after[2];
        let expected_patrol = match second.deck_b {
            SystemState::Working => kept,
            SystemState::Degraded => kept / 2,
            SystemState::Out => 0,
        };
        assert_eq!(second.patrol, expected_patrol, "seed {}", run.seed);
        seen_deck_states.insert(second.deck_b);
        checked += 1;
    }
    eprintln!("second attacks {checked}, deck states {seen_deck_states:?}");
    assert!(seen_deck_states.contains(&SystemState::Working));
    assert!(seen_deck_states.contains(&SystemState::Degraded));
    assert!(seen_deck_states.contains(&SystemState::Out));
}

// ---------------------------------------------------------------------
// What a run leaves.
// ---------------------------------------------------------------------

#[test]
fn carrier_b_is_never_sunk_the_cruisers_are_never_attacked_and_the_guns_always_fire_36() {
    for run in batch() {
        // Two attacks do 4 + 4 hits of 10 points at most: 80 points.
        assert!(run.hits() <= 8, "seed {}", run.seed);
        let carrier = run.ship(3);
        assert!(!carrier.sunk);
        assert_eq!(
            carrier.flotation + carrier.fire,
            Fixed::from_int(10) * i64::from(run.hits()),
            "seed {}",
            run.seed
        );
        assert!(carrier.flotation + carrier.fire <= Fixed::from_int(80));
        for ship in [1, 2, 4, 5] {
            let other = run.ship(ship);
            assert_eq!(other.flotation, Fixed::ZERO, "ship {ship}");
            assert_eq!(other.fire, Fixed::ZERO, "ship {ship}");
            assert!(!other.sunk);
            assert_eq!(other.systems, ALL_WORKING, "ship {ship}");
        }
        for attack in run.attacks() {
            let EventKind::AirAttack { ship, .. } = attack.kind else {
                unreachable!()
            };
            assert_eq!(ship, CARRIER_B);
        }
        for guns in run.of(|k| matches!(k, EventKind::AntiAircraftFire { .. })) {
            let EventKind::AntiAircraftFire { shots, .. } = guns.kind else {
                unreachable!()
            };
            assert_eq!(shots, Fixed::from_int(36));
        }
    }
}

// ---------------------------------------------------------------------
// The variants: the share, and the second carrier's fighters.
// ---------------------------------------------------------------------

/// Copies the scenario and its data into a folder under the build directory,
/// makes each exact, asserted-unique text replacement in the copied scenario
/// file, and loads it. Nothing in the repository's data folder is touched.
fn variant(tag: &str, edits: &[(&str, &str)]) -> Scenario {
    let data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data");
    // Tests run in parallel: each variant has a folder of its own.
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "air_combat_{tag}_{}_{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
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
        fs::read_to_string(data.join("scenarios/05c_air_combat.toml")).expect("read the scenario");
    for (original, changed) in edits {
        assert_eq!(
            text.matches(original).count(),
            1,
            "the text {original:?} appears exactly once"
        );
        text = text.replace(original, changed);
    }
    fs::write(root.join("scenarios/05c_air_combat.toml"), text).expect("write the variant");
    Scenario::load(&root.join("scenarios/05c_air_combat.toml")).expect("the variant loads")
}

const SHARE_ORDER: &str = "share = \"0.5\"";
const GROUP_4_MISSION: &str = "side = \"japanese\"\nair_group = 4\nkind = \"set_air_mission\"\nmission = \"combat_air_patrol\"";

fn share_variant(share: &str) -> Scenario {
    variant(
        &format!("share_{share}"),
        &[(SHARE_ORDER, &format!("share = \"{share}\""))],
    )
}

fn group_4_at_rest() -> Scenario {
    variant(
        "group_4_rests",
        &[(
            GROUP_4_MISSION,
            "side = \"japanese\"\nair_group = 4\nkind = \"set_air_mission\"\nmission = \"rest\"",
        )],
    )
}

/// The first attack of each run of a variant, worked and asserted.
fn first_attacks(scenario: &Scenario, config: Config) -> Vec<Worked> {
    let mut firsts = Vec::new();
    for run in batch_of(scenario, SEEDS) {
        if let Some(first) = work_the_run(&run, config).into_iter().next() {
            firsts.push(first);
        }
    }
    assert!(firsts.len() > 400);
    firsts
}

#[test]
fn a_share_of_1_sends_no_escorts_and_the_patrol_of_12_fires_all_its_shots_at_the_bombers() {
    let variant = share_variant("1");
    let config = Config {
        share_tenths: 10,
        group_4_on_patrol: true,
    };
    let mut bombers_attacking = BTreeSet::new();
    for run in batch_of(&variant, SEEDS) {
        assert!(run
            .of(|k| matches!(k, EventKind::EscortsSent { .. }))
            .is_empty());
        let fights = run.of(|k| matches!(k, EventKind::AirFight { .. }));
        for (index, fight) in fights.iter().enumerate() {
            let EventKind::AirFight {
                firer,
                target,
                firing,
                at,
                shots,
                ..
            } = fight.kind
            else {
                unreachable!()
            };
            assert_eq!(
                (firer, target, firing, at),
                (FIGHTERS_B, BOMBERS, AirFirer::Patrol, AirTarget::Bombers)
            );
            // The first attack meets all 12; a second meets what carrier
            // B's deck can still fly (worked by `work_the_run` below).
            if index == 0 {
                assert_eq!(shots, Fixed::from_int(12));
            }
        }
        // Group 3 loses nothing, and keeps all 12.
        assert_eq!(run.ready[2], 12);
        for first in work_the_run(&run, config).iter().take(1) {
            // 12 shots at 0.3 = 3.6: 3 or 4 hits, and the guns 1 or 2:
            // 12 to 14 bombers attack.
            let hits = first.patrol_at_bombers.expect("the patrol fired");
            assert!(hits == 3 || hits == 4);
            let guns = first.guns.expect("the guns fired");
            assert!(guns == 1 || guns == 2);
            assert_eq!(first.bombers_attacking, 18 - hits - guns);
            assert!(matches!(first.bombers_attacking, 12..=14));
            assert!(first.escorts_at_patrol.is_none() && first.patrol_at_escorts.is_none());
            bombers_attacking.insert(first.bombers_attacking);
        }
    }
    assert_eq!(bombers_attacking, BTreeSet::from([12, 13, 14]));
}

#[test]
fn a_share_of_0_sends_12_escorts_and_the_patrol_leaves_the_bombers_alone() {
    let variant = share_variant("0");
    let config = Config {
        share_tenths: 0,
        group_4_on_patrol: true,
    };
    let firsts = first_attacks(&variant, config);
    let mut bombers_attacking = BTreeSet::new();
    for first in &firsts {
        assert_eq!((first.escorts, first.patrol), (12, 12));
        // 12 shots each way at 0.3 = 3.6: 3 or 4 hits.
        for roll in [first.escorts_at_patrol, first.patrol_at_escorts] {
            let hits = roll.expect("both fought");
            assert!(hits == 3 || hits == 4);
        }
        // None at the bombers, and the guns 1 or 2: 16 or 17 attack.
        assert!(first.patrol_at_bombers.is_none());
        let guns = first.guns.expect("the guns fired");
        assert!(guns == 1 || guns == 2);
        assert!(matches!(first.bombers_attacking, 16 | 17));
        bombers_attacking.insert(first.bombers_attacking);
    }
    assert_eq!(bombers_attacking, BTreeSet::from([16, 17]));
}

#[test]
fn with_group_4_not_on_patrol_nothing_fights_and_only_the_guns_cost_bombers() {
    let variant = group_4_at_rest();
    let config = Config {
        share_tenths: 5,
        group_4_on_patrol: false,
    };
    let runs = batch_of(&variant, SEEDS);
    let mut bombers_attacking = BTreeSet::new();
    for run in &runs {
        assert!(run
            .of(|k| matches!(k, EventKind::AirFight { .. }))
            .is_empty());
        // Group 4 loses nothing.
        assert_eq!(run.ready[3], 12);
        let worked = work_the_run(run, config);
        if let Some(first) = worked.first() {
            assert!(
                first.escorts_at_patrol.is_none()
                    && first.patrol_at_escorts.is_none()
                    && first.patrol_at_bombers.is_none()
            );
            let guns = first.guns.expect("the guns fired");
            assert!(guns == 1 || guns == 2);
            assert!(matches!(first.bombers_attacking, 16 | 17));
            bombers_attacking.insert(first.bombers_attacking);
        }
    }
    assert_eq!(bombers_attacking, BTreeSet::from([16, 17]));
}

// ---------------------------------------------------------------------
// Properties.
// ---------------------------------------------------------------------

#[test]
fn every_groups_aircraft_at_the_end_are_those_at_the_start_less_its_recorded_losses() {
    for run in batch() {
        let lost = |group: AirGroupId| -> u32 {
            run.of(|k| matches!(k, EventKind::AircraftLost { .. }))
                .iter()
                .map(|event| match event.kind {
                    EventKind::AircraftLost {
                        air_group,
                        aircraft,
                        ..
                    } if air_group == group => aircraft,
                    _ => 0,
                })
                .sum()
        };
        let start = [8u32, 18, 12, 12];
        for (index, group) in [SCOUTS, BOMBERS, FIGHTERS_A, FIGHTERS_B]
            .into_iter()
            .enumerate()
        {
            assert_eq!(
                run.ready[index],
                start[index] - lost(group),
                "seed {}, group {}",
                run.seed,
                group.raw()
            );
        }
        // The scouts and the six fighters over carrier A never lose any:
        // group 3 loses only its escorts, at most 6 an attack.
        assert_eq!(lost(SCOUTS), 0);
        // The six kept over carrier A are never fired at: nobody attacks
        // carrier A.
        let escorts_sent: u32 = run
            .of(|k| matches!(k, EventKind::EscortsSent { .. }))
            .iter()
            .map(|event| match event.kind {
                EventKind::EscortsSent { aircraft, .. } => aircraft,
                _ => unreachable!(),
            })
            .sum();
        assert!(lost(FIGHTERS_A) <= escorts_sent, "seed {}", run.seed);
        assert!(run.ready[2] >= 6, "seed {}", run.seed);
    }
}

#[test]
fn no_aircraft_is_lost_in_a_pulse_in_which_no_strike_attacks() {
    for run in batch() {
        for pulse in [Pulse::Night, Pulse::Morning, Pulse::Afternoon] {
            let in_pulse = |keep: fn(&EventKind) -> bool| {
                run.events
                    .iter()
                    .filter(|event| event.pulse == Some(pulse) && keep(&event.kind))
                    .count()
            };
            let attacked = in_pulse(|k| matches!(k, EventKind::AirAttack { .. }));
            let fought = in_pulse(|k| {
                matches!(
                    k,
                    EventKind::AircraftLost { .. }
                        | EventKind::AirFight { .. }
                        | EventKind::AntiAircraftFire { .. }
                )
            });
            if attacked == 0 {
                assert_eq!(fought, 0, "seed {}, {pulse:?}", run.seed);
            }
        }
    }
}

#[test]
fn more_escorts_never_mean_more_patrol_shots_at_the_bombers() {
    let shots_at_bombers = |share: &str, config: Config| -> BTreeSet<u32> {
        first_attacks(&share_variant(share), config)
            .iter()
            .map(|first| first.shots_at_bombers)
            .collect()
    };
    let with = |share_tenths| Config {
        share_tenths,
        group_4_on_patrol: true,
    };
    // Share 1: no escorts, the patrol of 12 fires all 12 at the bombers.
    // Share 0.5: 6 escorts, 6 at the bombers. Share 0: 12 escorts, none.
    assert_eq!(shots_at_bombers("1", with(10)), BTreeSet::from([12]));
    let half: BTreeSet<u32> = batch()
        .iter()
        .filter_map(|run| {
            work_the_run(run, SCENARIO)
                .first()
                .map(|w| w.shots_at_bombers)
        })
        .collect();
    assert_eq!(half, BTreeSet::from([6]));
    assert_eq!(shots_at_bombers("0", with(0)), BTreeSet::from([0]));
}

#[test]
fn more_escorts_never_cost_the_bombers_more_to_the_fighters() {
    // The mean number of bombers attacking in 1000 first attacks rises
    // as the share falls: 13.x, 15.x, 16.x.
    let mean = |scenario: &Scenario, config: Config| -> i64 {
        let firsts = first_attacks(scenario, config);
        let total: i64 = firsts.iter().map(|w| i64::from(w.bombers_attacking)).sum();
        total * 1000 / count(firsts.len())
    };
    let with = |share_tenths| Config {
        share_tenths,
        group_4_on_patrol: true,
    };
    let share_1 = mean(&share_variant("1"), with(10));
    let share_half = mean(&load(), SCENARIO);
    let share_0 = mean(&share_variant("0"), with(0));
    let rested = mean(
        &group_4_at_rest(),
        Config {
            share_tenths: 5,
            group_4_on_patrol: false,
        },
    );
    eprintln!("mean bombers attacking x1000: share 1 {share_1}, 0.5 {share_half}, 0 {share_0}, group 4 at rest {rested}");
    assert!(share_1 < share_half && share_half < share_0);
    // With nobody to fight, only the guns: 18 - 1.8 = 16.2.
    assert!(rested >= share_0 - 200 && rested <= share_0 + 200);
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
    // The patrol share is in the hash: the same day with a different share
    // ends in a different state.
    let other = share_variant("0");
    let mut other = other.with_seed(1942).expect("a seed can be set");
    other.run_to_end().expect("the run resolves");
    assert_ne!(own.world.day_hashes(), other.world.day_hashes());
}
