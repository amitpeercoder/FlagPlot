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
//! A strike that finds nothing drops the contact (decision 0028).
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
//! Each shot is rolled on its own (decision 0033): a roll of n shots at
//! chance p scores anything from no hit to n, n x p on average, with a
//! standard deviation of the square root of n x p x (1 - p). A hit is an
//! aircraft lost, and a group cannot lose more than it has. (Before decision
//! 0033 a roll scored the whole part of n x p, or one more.)
//!
//! A strike that finds nothing meets no fighters and no guns, and loses
//! nothing.
//!
//! ## The first attack of a run
//!
//! Until the first attack every group is whole and every flight deck works:
//! F = 12, so 6 stay over carrier A and E = 6. P = 12. B = 18.
//!
//! | Step                       | Shots | Chance | Expected | Std dev  | Lost               |
//! | -------------------------- | ----- | ------ | -------- | -------- | ------------------ |
//! | Escorts at the patrol      | 6     | 0.3    | 1.8      | 1.122497 | 0 to 6 of group 4  |
//! | Patrol at the escorts      | 6     | 0.3    | 1.8      | 1.122497 | 0 to 6 of group 3  |
//! | Patrol at the bombers      | 6     | 0.3    | 1.8      | 1.122497 | 0 to 6 of group 2  |
//! | Guns at the bombers        | 36    | 0.05   | 1.8      | 1.307670 | 0 to those left of group 2 |
//!
//! The bombers that attack are 18 less the two losses of group 2: 14.4 on
//! average. Each drops one bomb at 0.2, so an attack by b bombers scores
//! from 0 to b hits, 0.2 x b on average.
//!
//! After it group 2 has the bombers that attacked, group 3 has 12 less its
//! loss and group 4 has 12 less its loss.
//!
//! Over seeds 1 to 1000, among the runs with an attack, the hits of each of
//! the four rolls of the first attack, summed over those runs, are within
//! four standard deviations of 1.8 times their number: the standard
//! deviation of the sum is the figure in the table times the square root of
//! the number of runs. The bomb hits of the first attacks are within four
//! standard deviations of 0.2 times the bombers that attacked, summed the
//! same way.
//!
//! ## A second attack
//!
//! A run has at most two attacks, one in each daylight pulse. The second is
//! worked by the same rule from what each group then has and the state of
//! carrier B's flight deck, which the first attack may have hit. The test
//! works every attack of every run from the counts and flight decks at the
//! start of its pulse.
//!
//! Carrier B is sunk by ten hits on flotation, which two attacks can now
//! do, though rarely: two attacks average under 6 hits, and a quarter of
//! them start fires instead. While she is afloat every attack is on her and
//! the guns fire 36 shots. If a morning attack sinks her, the aircraft of air
//! group 4 that were aboard her go down with her (decisions 0025 and 10 of
//! 0059), and those it had in the air, its patrol, come down in the sea (a
//! ship of its side, a cruiser, is afloat in the hex; there is no other
//! carrier and no base here: decisions 7 to 10 of 0059). Group 4 ends with
//! none, so the afternoon strike meets no patrol
//! and group 3's escorts fire at nothing; the cruisers fire 16 shots; and
//! the bombers attack the lower-numbered cruiser afloat, at 15 points a hit.
//! (Before decision 0033 two attacks did 80 points at most and she could
//! not be sunk.)
//!
//! ## Variants
//!
//! Each differs from the scenario in one order, and the figures are for its
//! first attack.
//!
//! | Variant                                | Escorts at patrol | Patrol at escorts | Patrol at bombers | Guns | Bombers attacking |
//! | -------------------------------------- | ----------------- | ----------------- | ----------------- | ---- | ----------------- |
//! | Group 3 keeps a share of 1: no escort  | none              | none              | 12 shots, 3.6 expected | 36 shots, 1.8 | 12.6 on average |
//! | Group 3 keeps a share of 0: 12 escorts | 12 shots, 3.6 expected | 12 shots, 3.6 expected | none        | 36 shots, 1.8 | 16.2 on average |
//! | Group 4 is not on patrol               | none              | none              | none              | 36 shots, 1.8 | 16.2 on average |
//! | The fighters' radius is 190 nm, not 250 | none             | none              | 12 shots, 3.6 expected | 36 shots, 1.8 | 12.6 on average |
//!
//! In each variant the shots and the chance of every roll of a first attack
//! are exactly those of its row, the bombers attacking are 18 less the
//! losses of group 2, and over the runs the hits of each roll are within
//! four standard deviations of the expected figure times the number of
//! first attacks.
//!
//! In the last variant a strike sent to (9,0), 200 nm away, is beyond the
//! fighters' radius: group 3 sends no escort with it and all twelve of its
//! fighters stay over carrier A (decision 0030). Every attack is such a
//! strike, so every attack meets the whole Japanese patrol, as it does when
//! group 3 keeps a share of 1. A report can name a hex 160 nm away; a
//! strike sent there is within the radius and takes its six escorts, and
//! finds nothing. (The first version of this paragraph said group 3 never
//! sends an escort. The row's figures were right.) In the scenario itself
//! the placeholder fighter's radius is 250 nm and the escorts go.
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

mod common;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use sim_core::events::{AirFirer, AirTarget, Cause, DamageTrack, Event, EventKind, Weapon};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, ContactId, EntityRef, EventId, OrderId, ShipId, TaskForceId};
use sim_core::rng::{Draw, RngKey, RngStream, Subsystem};
use sim_core::state::{ShipLocation, ShipSystem, Side, SystemState};
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
    common::load(&scenario_path()).expect("scenario 5c loads")
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
    /// The orders that set group 4's mission and share: the causes of its
    /// patrol (decisions 7 to 10 of 0059 cite them when it comes home).
    patrol_orders_of_4: Vec<Cause>,
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
        // The claims from the air (decision 0056) are not part of what this
        // scenario works out: they come after each raid is resolved and are
        // asserted in scenario_11_claims.rs.
        events: run
            .world
            .events()
            .iter()
            .filter(|e| {
                !matches!(
                    e.kind,
                    sim_core::events::EventKind::ClaimReceived { from_air: true, .. }
                )
            })
            .cloned()
            .collect(),
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
        patrol_orders_of_4: truth.air_groups[&FIGHTERS_B]
            .mission_order
            .into_iter()
            .chain(truth.air_groups[&FIGHTERS_B].share_order)
            .map(Cause::Order)
            .collect(),
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
    /// A bomb's chance to hit, in tenths (2 in the scenario).
    bomb_tenths: i64,
}

const SCENARIO: Config = Config {
    share_tenths: 5,
    group_4_on_patrol: true,
    bomb_tenths: 2,
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
    /// The anti-aircraft shots of the ships afloat at the start of the
    /// pulse, and the hits of the guns, or `None` if they had no target.
    gun_shots: u32,
    guns: Option<u32>,
    bombers_attacking: u32,
    attack_hits: u32,
    /// The ship the attack was on: carrier B while she is afloat, else the
    /// lower-numbered cruiser afloat.
    target: ShipId,
    /// Whether the attack sank carrier B, taking air group 4 with her.
    sank_b: bool,
    /// The aircraft of groups 2, 3 and 4 after the fights and the bombs, and
    /// after the attack's damage has landed (the same unless carrier B sank:
    /// then group 4 is lost with her).
    after_fight: [u32; 3],
    after: [u32; 3],
    /// Whether carrier B's flight deck was degraded or out at the start.
    deck_b: SystemState,
}

/// The hits a roll of `shots` at `chance` gives from its stream: one draw for
/// each shot (decision 0033).
fn hits_from(stream: &mut RngStream, shots: u32, chance: Fixed) -> u32 {
    (0..shots).map(|_| u32::from(stream.chance(chance))).sum()
}

fn stream_of(seed: u64, pulse: Pulse, entity: EntityRef, other: EntityRef) -> RngStream {
    RngKey {
        seed,
        day: 1,
        pulse: Some(pulse),
        subsystem: Subsystem::AirCombat,
        entity,
        other: Some(other),
        draw: Draw::Legacy,
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
/// The ships, groups and bases of the scenario as loaded, for the model of
/// where aircraft go when their deck is lost.
#[allow(clippy::type_complexity)]
fn no_deck_facts() -> &'static (
    Vec<common::no_deck::ModelShip>,
    Vec<common::no_deck::ModelGroup>,
    Vec<common::no_deck::ModelBase>,
) {
    static FACTS: OnceLock<(
        Vec<common::no_deck::ModelShip>,
        Vec<common::no_deck::ModelGroup>,
        Vec<common::no_deck::ModelBase>,
    )> = OnceLock::new();
    FACTS.get_or_init(|| common::no_deck::initial(&load().world))
}

fn work_the_run(run: &Run, config: Config) -> Vec<Worked> {
    let seed = run.seed;
    let mut ready = [18u32, 12, 12];
    let mut deck_b = SystemState::Working;
    // The ships of task force 2 (carrier B, then the cruisers) still afloat,
    // and the flotation damage each has taken, from the damage events.
    let mut afloat: BTreeSet<ShipId> = [3, 4, 5].map(ShipId::new).into();
    let mut flotation: BTreeMap<ShipId, Fixed> =
        [3, 4, 5].map(|id| (ShipId::new(id), Fixed::ZERO)).into();
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
        let damaged = kinds(|k| matches!(k, EventKind::ShipDamaged { .. }));
        let sunk = kinds(|k| matches!(k, EventKind::ShipSunk { .. }));
        let groups_lost = kinds(|k| matches!(k, EventKind::AirGroupLost { .. }));

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
        let EventKind::StrikeLaunched { hex, .. } = launch.kind else {
            unreachable!()
        };
        if hex != TARGET {
            assert!(attacked.is_empty(), "seed {seed}");
            assert_eq!(nothing.len(), 1, "seed {seed}");
            assert_eq!(
                nothing[0].causes,
                vec![Cause::Event(launch.id)],
                "seed {seed}"
            );
            assert!(
                fights.is_empty()
                    && guns.is_empty()
                    && lost.is_empty()
                    && damaged.is_empty()
                    && sunk.is_empty()
                    && groups_lost.is_empty(),
                "seed {seed}: a strike that finds nothing fights nobody"
            );
            continue;
        }
        assert!(nothing.is_empty(), "seed {seed}");

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
            // The patrol's fire also records its share of the patrol
            // (decision 0033, 7): one raid, so the whole patrol, of the
            // bombers and the escorts sent.
            let mut expected_factors = if *firing == AirFirer::Patrol {
                vec![
                    ("patrol", int(p)),
                    ("raid_aircraft", int(b + e)),
                    ("all_raids_aircraft", int(b + e)),
                    ("patrol_share", int(p)),
                ]
            } else {
                Vec::new()
            };
            expected_factors.extend([("shots", int(*shots)), ("hit_chance", chance)]);
            assert_eq!(
                breakdown
                    .factors
                    .iter()
                    .map(|f| (f.name, f.value))
                    .collect::<Vec<_>>(),
                expected_factors
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

        // The guns: every ship afloat in task force 2 at the start of the
        // pulse (the carrier's 20 and each cruiser's 8: 36 while carrier B is
        // afloat, 16 once she is sunk), at the bombers left. The stream is
        // that of the lowest-numbered ship afloat.
        let gun_shots: u32 = afloat
            .iter()
            .map(|ship| if *ship == CARRIER_B { 20 } else { 8 })
            .sum();
        let lowest = *afloat.iter().next().expect("a ship afloat");
        // The target of the attack is the same ship.
        let target = lowest;
        let left_after_fighters = now[0] - (ready[0] - b);
        assert_eq!(left_after_fighters, now[0], "the bombers are group 2's own");
        let bombers_left = b - (ready[0] - now[0]);
        let mut gun_hits = None;
        if bombers_left > 0 {
            assert_eq!(guns.len(), 1, "seed {seed}");
            let mut stream = stream_of(
                seed,
                pulse,
                EntityRef::Ship(lowest),
                EntityRef::AirGroup(BOMBERS),
            );
            let hits = hits_from(&mut stream, gun_shots, hundredths(5));
            let expected = int(gun_shots) * hundredths(5);
            assert_eq!(
                guns[0].kind,
                EventKind::AntiAircraftFire {
                    task_force: ENEMY_FORCE,
                    air_group: BOMBERS,
                    shots: int(gun_shots),
                    chance: hundredths(5),
                    expected,
                    hits,
                    after_attack: false,
                },
                "seed {seed}"
            );
            assert_eq!(
                guns[0].kind.to_string(),
                format!(
                    "task_force:2 guns at the raid of air_group:2: {}",
                    shots_text(gun_shots, "0.05", expected, hits)
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
        if bombers_attacking == 0 {
            // Fighters and guns took every bomber (a second strike of a few,
            // against a patrol of 12): nothing attacks, so there is no attack,
            // no damage and no finding.
            assert!(
                attacked.is_empty()
                    && nothing.is_empty()
                    && damaged.is_empty()
                    && sunk.is_empty()
                    && groups_lost.is_empty(),
                "seed {seed}"
            );
            ready = now;
            continue;
        }
        assert_eq!(attacked.len(), 1, "seed {seed}");
        let mut stream = stream_of(
            seed,
            pulse,
            EntityRef::AirGroup(BOMBERS),
            EntityRef::TaskForce(ENEMY_FORCE),
        );
        let bomb_chance = tenths(config.bomb_tenths);
        let attack_hits = hits_from(&mut stream, bombers_attacking, bomb_chance);
        let attack_expected = int(bombers_attacking) * bomb_chance;
        assert_eq!(
            attacked[0].kind,
            EventKind::AirAttack {
                air_group: BOMBERS,
                task_force: ENEMY_FORCE,
                ship: target,
                weapon: Weapon::Bomb,
                shots: int(bombers_attacking),
                chance: bomb_chance,
                expected: attack_expected,
                hits: attack_hits
            },
            "seed {seed}"
        );
        assert_eq!(
            attacked[0].kind.to_string(),
            format!(
                "air_group:2 bombs at ship:{}: {bombers_attacking} shots x {bomb_chance} = {attack_expected} expected, {attack_hits} {}",
                target.raw(),
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

        // The damage lands: the hits are 10 points each on the carrier and 15
        // on a cruiser, flotation or fire; flotation of 100 sinks the ship,
        // and a sunk carrier takes the air group aboard it.
        let per_hit = if target == CARRIER_B {
            Fixed::from_int(10)
        } else {
            Fixed::from_int(15)
        };
        let mut track_hits = 0;
        let mut last_flotation = None;
        let mut sinks = false;
        let mut last_track: Option<DamageTrack> = None;
        for event in &damaged {
            let EventKind::ShipDamaged {
                round,
                ship,
                track,
                hits: hits_on_track,
                points,
                value_after,
            } = event.kind
            else {
                unreachable!()
            };
            assert_eq!(event.causes, vec![Cause::Event(attacked[0].id)]);
            assert_eq!((round, ship), (0, target), "seed {seed}");
            assert!(hits_on_track > 0);
            assert!(last_track.is_none_or(|before| (before as usize) < (track as usize)));
            last_track = Some(track);
            assert_eq!(points, per_hit * i64::from(hits_on_track), "seed {seed}");
            track_hits += hits_on_track;
            if track == DamageTrack::Flotation {
                let before = flotation[&target];
                let after = (before + points).min(Fixed::from_int(100));
                assert_eq!(value_after, after, "seed {seed}");
                flotation.insert(target, after);
                sinks = after >= Fixed::from_int(100);
                last_flotation = Some(event.id);
            }
        }
        assert_eq!(
            track_hits, attack_hits,
            "seed {seed}: every hit is on a track"
        );
        let mut after_sinking = now;
        let mut sank_b = false;
        // The aircraft group 4 has in the air at the end of the step: its
        // patrol less what it lost (decision 7 of 0059).
        let in_air_4 = p - (ready[2] - now[2]);
        if sinks {
            assert_eq!(sunk.len(), 1, "seed {seed}");
            assert_eq!(
                sunk[0].kind,
                EventKind::ShipSunk {
                    round: 0,
                    ship: target,
                    task_force: ENEMY_FORCE
                },
                "seed {seed}"
            );
            assert_eq!(
                sunk[0].causes,
                vec![Cause::Event(last_flotation.expect("a flotation hit"))]
            );
            afloat.remove(&target);
            if target == CARRIER_B {
                // Air group 4 is the only one aboard. What was aboard goes
                // down with her; its patrol, in the air, is kept and comes
                // home below (decisions 7 to 10 of 0059).
                let aboard = now[2] - in_air_4;
                assert_eq!(groups_lost.len(), 1, "seed {seed}");
                assert_eq!(
                    groups_lost[0].kind,
                    EventKind::AirGroupLost {
                        air_group: FIGHTERS_B,
                        ship: CARRIER_B,
                        aircraft: aboard
                    },
                    "seed {seed}"
                );
                assert_eq!(
                    groups_lost[0].kind.to_string(),
                    format!("air_group:4 lost with ship:3: {aboard} aircraft")
                );
                assert_eq!(groups_lost[0].causes, vec![Cause::Event(sunk[0].id)]);
                after_sinking[2] = in_air_4;
                sank_b = true;
            } else {
                assert!(groups_lost.is_empty(), "seed {seed}");
            }
        } else {
            assert!(sunk.is_empty() && groups_lost.is_empty(), "seed {seed}");
        }

        // ---- The aircraft of group 4 in the air come home if carrier B's deck
        // is lost: she sank, or her flight deck went out in this pulse. There
        // is no other carrier and no base here, so they come down in the sea,
        // beside the cruisers while one is afloat. ----
        {
            let mut deck_losses: Vec<common::no_deck::DeckLoss> = Vec::new();
            let mut deck_after = deck_b;
            let mut sunk_so_far = false;
            let (mut ships, mut model_groups, bases) = no_deck_facts().clone();
            let hex = ships
                .iter()
                .find(|s| s.id == CARRIER_B.raw())
                .map(|s| s.hex)
                .expect("carrier B");
            for event in &step {
                match event.kind {
                    EventKind::ShipSunk { ship, .. } if ship == CARRIER_B => {
                        sunk_so_far = true;
                        deck_losses.push(common::no_deck::DeckLoss {
                            ship: ship.raw(),
                            hex,
                            cause: event.id,
                        });
                    }
                    EventKind::SystemHit {
                        ship,
                        system: ShipSystem::FlightDeck,
                        after,
                        ..
                    } if ship == CARRIER_B && !sunk_so_far => {
                        if after == SystemState::Out && deck_after != SystemState::Out {
                            deck_losses.push(common::no_deck::DeckLoss {
                                ship: ship.raw(),
                                hex,
                                cause: event.id,
                            });
                        }
                        deck_after = after;
                    }
                    _ => {}
                }
            }
            for ship in &mut ships {
                ship.afloat = afloat.contains(&ShipId::new(ship.id));
                ship.deck_out = ship.id == CARRIER_B.raw() && deck_after == SystemState::Out;
            }
            for group in &mut model_groups {
                match group.id {
                    2 => group.ready = after_sinking[0],
                    3 => group.ready = after_sinking[1],
                    4 => {
                        group.ready = after_sinking[2];
                        group.in_air = in_air_4;
                    }
                    _ => {}
                }
            }
            let homing = kinds(|k| {
                matches!(
                    k,
                    EventKind::AircraftLandedElsewhere { .. }
                        | EventKind::AircraftDitchedNoDeck { .. }
                )
            });
            let expected =
                common::no_deck::homecoming(&ships, &mut model_groups, &bases, &deck_losses);
            assert_eq!(homing.len(), expected.len(), "seed {seed}");
            for (event, (home, closing)) in homing.iter().zip(&expected) {
                let common::no_deck::Home::NoDeck {
                    group,
                    hex,
                    aircraft,
                    beside_own_force,
                } = *home
                else {
                    panic!("seed {seed}: nothing lands elsewhere here: {home:?}");
                };
                assert_eq!(group, FIGHTERS_B.raw());
                // Its patrol sortie's causes follow the closing event.
                let mut causes: Vec<Cause> = closing.map(Cause::Event).into_iter().collect();
                causes.extend(run.patrol_orders_of_4.iter().copied());
                assert_eq!(
                    event.kind,
                    EventKind::AircraftDitchedNoDeck {
                        air_group: FIGHTERS_B,
                        hex,
                        aircraft,
                        beside_own_force,
                        // The frozen figures: both crew figures are 1.
                        chance: Fixed::ONE,
                        crews_recovered: None,
                        pilots_lost: 0,
                        ready: model_groups
                            .iter()
                            .find(|g| g.id == group)
                            .map(|g| g.ready)
                            .expect("group 4")
                    },
                    "seed {seed}"
                );
                assert_eq!(event.causes, causes, "seed {seed}");
            }
            after_sinking[2] = model_groups
                .iter()
                .find(|g| g.id == 4)
                .map(|g| g.ready)
                .expect("group 4");
        }

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
            gun_shots,
            guns: gun_hits,
            bombers_attacking,
            attack_hits,
            target,
            sank_b,
            after_fight: now,
            after: after_sinking,
            deck_b,
        });
        ready = after_sinking;
    }
    // The end of the day: every group's aircraft are those at the start
    // less the losses of its events, and nobody gained.
    assert_eq!(
        [run.ready[1], run.ready[2], run.ready[3]],
        ready,
        "seed {seed}"
    );
    // The ships of task force 2 are as the damage events leave them; nobody
    // else was ever hit.
    for id in [3, 4, 5] {
        let ship = ShipId::new(id);
        let last = run.ship(id);
        assert_eq!(last.flotation, flotation[&ship], "seed {seed}, ship {id}");
        assert_eq!(last.sunk, !afloat.contains(&ship), "seed {seed}, ship {id}");
    }
    for id in [1, 2] {
        let other = run.ship(id);
        assert_eq!(other.flotation, Fixed::ZERO, "ship {id}");
        assert_eq!(other.fire, Fixed::ZERO, "ship {id}");
        assert!(!other.sunk);
        assert_eq!(other.systems, ALL_WORKING, "ship {id}");
    }
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
        // As in 5b: the afternoon search sighted, or the morning's contact
        // is still held, which needs a morning strike that attacked (one
        // that found nothing dropped the contact, decision 0028). No raid
        // here is shot out of the sky before it finds or misses its hex.
        let morning_attacked = run
            .attacks()
            .iter()
            .any(|event| event.pulse == Some(Pulse::Morning));
        let afternoon_launch = afternoon_sighted || morning_attacked;
        assert_eq!(
            run.launched_in(Pulse::Afternoon).is_some(),
            afternoon_launch,
            "seed {}",
            run.seed
        );
        morning += i64::from(morning_sighted);
        afternoon += i64::from(afternoon_launch);
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
            // Contact 1, or contact 2 if a strike dropped contact 1 first.
            assert_eq!(air_group, BOMBERS);
            assert!(
                contact == ContactId::new(1) || contact == ContactId::new(2),
                "seed {}",
                run.seed
            );
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
            // A strike that found nothing drops the contact it was sent at,
            // citing the finding; one that attacked drops nothing.
            let dropped: Vec<&Event> = run
                .events
                .iter()
                .filter(|e| e.causes.contains(&Cause::Event(landed[0].id)))
                .filter(|e| matches!(e.kind, EventKind::ContactNotThere { .. }))
                .collect();
            if hex == TARGET {
                assert!(dropped.is_empty(), "seed {}", run.seed);
            } else {
                assert_eq!(dropped.len(), 1, "seed {}", run.seed);
                assert_eq!(
                    dropped[0].kind,
                    EventKind::ContactNotThere {
                        side: Side::Allied,
                        contact,
                        hex
                    },
                    "seed {}",
                    run.seed
                );
            }
            // The launch cites the order that set the mission.
            assert_eq!(launch.causes[0], Cause::Order(OrderId::new(3)));
        }
    }
    assert!(within_four_sigma(morning, 1000, 509_295), "{morning}");
    assert!(within_four_sigma(afternoon, 1000, 684_234), "{afternoon}");
}

// ---------------------------------------------------------------------
// The first attack.
// ---------------------------------------------------------------------

/// Asserts that over `firsts` the hits of one kind of roll are within four
/// standard deviations of their expected figure. The shots of all the attacks
/// are independent draws at one chance, so the hits summed over them are a
/// binomial count of all the shots: the expected figure is the shots x the
/// chance, the variance the shots x chance x (1 - chance). Each attack's hits
/// must also be from none to its shots, and a roll with no shots has no hits.
fn assert_in_band(
    name: &str,
    firsts: &[Worked],
    chance_millionths: i64,
    roll: impl Fn(&Worked) -> (u32, Option<u32>),
) {
    let (mut shots_total, mut hits_total) = (0i64, 0i64);
    for first in firsts {
        let (shots, hits) = roll(first);
        match hits {
            Some(hits) => {
                assert!(hits <= shots, "{name}: {hits} hits of {shots} shots");
                hits_total += i64::from(hits);
            }
            None => assert_eq!(shots, 0, "{name}: a roll with shots was made"),
        }
        shots_total += i64::from(shots);
    }
    eprintln!("{name}: {hits_total} hits of {shots_total} shots");
    assert!(
        within_four_sigma(hits_total, shots_total, chance_millionths),
        "{name}: {hits_total} hits of {shots_total} shots at {chance_millionths} millionths"
    );
}

/// What every first attack of a set satisfies, for any variant: the bombers
/// attacking are 18 less the losses of group 2 to the patrol's fire at them
/// and to the guns (a group cannot lose more than it has), and the hits of
/// each roll over the runs are within four standard deviations of shots x
/// chance.
fn assert_first_attacks_by_the_rule(firsts: &[Worked]) {
    for first in firsts {
        let at_bombers = first.patrol_at_bombers.unwrap_or(0);
        let lost_to_fighters = at_bombers.min(18);
        let lost_to_guns = first.guns.unwrap_or(0).min(18 - lost_to_fighters);
        assert_eq!(
            first.bombers_attacking,
            18 - lost_to_fighters - lost_to_guns
        );
        assert!(first.attack_hits <= first.bombers_attacking);
    }
    assert_in_band("escorts at the patrol", firsts, 300_000, |w| {
        (
            if w.patrol > 0 { w.escorts } else { 0 },
            w.escorts_at_patrol,
        )
    });
    assert_in_band("patrol at the escorts", firsts, 300_000, |w| {
        (w.escorts.min(w.patrol), w.patrol_at_escorts)
    });
    assert_in_band("patrol at the bombers", firsts, 300_000, |w| {
        (w.shots_at_bombers, w.patrol_at_bombers)
    });
    assert_in_band("guns at the bombers", firsts, 50_000, |w| {
        (w.gun_shots, w.guns)
    });
    assert_in_band("bombs", firsts, 200_000, |w| {
        (w.bombers_attacking, Some(w.attack_hits))
    });
}

#[test]
fn the_first_attack_meets_6_escorts_12_patrol_and_36_gun_shots_by_the_table() {
    let mut firsts = Vec::new();
    for run in batch() {
        let Some(first) = work_the_run(run, SCENARIO).into_iter().next() else {
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
        // Carrier B is afloat, so the attack is on her and the guns fire 36.
        assert_eq!(first.target, CARRIER_B);
        assert_eq!(first.gun_shots, 36);
        // Each roll is shot by shot (decision 0033): 6 shots at 0.3 for the
        // escorts, 6 for the patrol at the escorts and 6 at the bombers, the
        // guns 36 at 0.05; each scores from none to its shots. (The shots and
        // the chance of each are asserted event by event in `work_the_run`.)
        assert_eq!(first.shots_at_bombers, 6);
        let hits = |roll: Option<u32>| roll.expect("the roll was made");
        let (e_p, p_e, p_b, g) = (
            hits(first.escorts_at_patrol),
            hits(first.patrol_at_escorts),
            hits(first.patrol_at_bombers),
            hits(first.guns),
        );
        assert!(e_p <= 6 && p_e <= 6 && p_b <= 6 && g <= 36);
        // The bombers that attack are 18 less the two losses of group 2.
        assert_eq!(first.bombers_attacking, 18 - p_b - g.min(18 - p_b));
        // Afterwards group 2 has what attacked, group 3 has 12 less its loss
        // and group 4 12 less its loss (before the attack's damage lands).
        assert_eq!(first.after_fight[0], first.bombers_attacking);
        assert_eq!(first.after_fight[1], 12 - p_e);
        assert_eq!(first.after_fight[2], 12 - e_p);
        firsts.push(first);
    }
    assert!(firsts.len() > 400);
    // Over the runs, the hits of each roll are within four standard
    // deviations of shots x chance, and the attack is not always 14 to 16.
    assert_first_attacks_by_the_rule(&firsts);
    let attacking: BTreeSet<u32> = firsts.iter().map(|w| w.bombers_attacking).collect();
    eprintln!(
        "first attacks {}; bombers attacking {attacking:?}",
        firsts.len()
    );
    assert!(attacking.len() > 3, "{attacking:?}");
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
            // the bombers that flew are those that came home from the first.
            assert_eq!(second.bombers, first.after[0], "seed {}", run.seed);
            assert!(second.bombers <= 18);
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
        // if twice, and none if carrier B was sunk by the first attack (group
        // 4 is lost with her: `after` has it at 0).
        let kept = first.after[2];
        if first.sank_b {
            assert_eq!((kept, second.patrol), (0, 0), "seed {}", run.seed);
        }
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

/// What a run that sank carrier B must show, and what every run shows while
/// she is afloat. Returns whether she was sunk, whether by the morning
/// attack, and how many attacks followed a sinking.
fn check_sinking(run: &Run, worked: &[Worked]) -> (bool, bool, usize) {
    let mut sunk_before = false;
    let mut sunk_in_the_morning = false;
    let mut after_a_sinking = 0;
    for attack in worked {
        let EventKind::AirAttack { ship, .. } = run
            .attacks()
            .into_iter()
            .find(|event| pulse_of(event) == attack.pulse)
            .expect("the attack")
            .kind
        else {
            unreachable!()
        };
        assert_eq!(ship, attack.target);
        if sunk_before {
            // She was sunk by the morning attack: air group 4 went with her
            // (decision 0025), so the afternoon strike meets no patrol and
            // group 3's escorts fire at nothing; the cruisers fire 16
            // shots; the bombers attack the lower-numbered cruiser afloat,
            // at 15 points a hit (asserted in `work_the_run`).
            assert_eq!(attack.pulse, Pulse::Afternoon);
            assert_eq!(attack.patrol, 0, "seed {}", run.seed);
            assert!(attack.escorts_at_patrol.is_none());
            assert!(attack.patrol_at_escorts.is_none());
            assert!(attack.patrol_at_bombers.is_none());
            assert_eq!(attack.gun_shots, 16, "seed {}", run.seed);
            assert_eq!(attack.target, ShipId::new(4), "seed {}", run.seed);
            assert_eq!(attack.after[2], 0);
            after_a_sinking += 1;
        } else {
            // While she is afloat every attack is on her and the guns fire
            // 36 shots.
            assert_eq!(attack.target, CARRIER_B, "seed {}", run.seed);
            assert_eq!(attack.gun_shots, 36, "seed {}", run.seed);
        }
        if attack.sank_b {
            sunk_in_the_morning |= attack.pulse == Pulse::Morning;
            sunk_before = true;
        }
    }
    // The carrier is sunk in the run if, and only if, an attack sank her.
    assert_eq!(run.ship(3).sunk, sunk_before, "seed {}", run.seed);
    // Carrier A and the destroyer are never touched; the cruisers only after
    // the carrier is sunk.
    for ship in [1, 2] {
        assert_eq!(run.ship(ship).flotation, Fixed::ZERO, "ship {ship}");
    }
    if !sunk_before {
        for ship in [4, 5] {
            let other = run.ship(ship);
            assert_eq!(other.flotation, Fixed::ZERO, "ship {ship}");
            assert_eq!(other.fire, Fixed::ZERO, "ship {ship}");
            assert!(!other.sunk);
            assert_eq!(other.systems, ALL_WORKING, "ship {ship}");
        }
    }
    // Group 4 is lost with her: its aircraft at the end are none.
    if sunk_before {
        assert_eq!(run.ready[3], 0, "seed {}", run.seed);
    }
    (sunk_before, sunk_in_the_morning, after_a_sinking)
}

#[test]
fn carrier_b_can_be_sunk_rarely_and_while_afloat_every_attack_is_on_her_and_the_guns_fire_36() {
    let mut sunk_runs = 0;
    for run in batch() {
        // `work_the_run` asserts the damage, the sinking and the loss of
        // group 4 event by event, and the ships of the run as it leaves them.
        let worked = work_the_run(run, SCENARIO);
        sunk_runs += i32::from(check_sinking(run, &worked).0);
    }
    eprintln!("carrier B sunk in {sunk_runs} of {SEEDS} runs");
    // Rarely: two attacks average under 6 hits and ten on flotation sink her.
    assert!(sunk_runs > 0 && sunk_runs < 100, "{sunk_runs}");
}

#[test]
fn a_morning_attack_that_sinks_carrier_b_takes_group_4_and_the_afternoon_meets_the_cruisers() {
    // At the scenario's bomb chance of 0.2 a morning sinking does not turn up
    // in 1,000 seeds (ten flotation hits from 14 bombers or so). With a bomb
    // chance of 0.8 it does, and every rule of it can be worked.
    let variant = variant_of_data(
        "bombs_0_8",
        &[],
        &[],
        &[("bomb_hit_chance = \"0.2\"", "bomb_hit_chance = \"0.8\"")],
    );
    let config = Config {
        bomb_tenths: 8,
        ..SCENARIO
    };
    let (mut sunk_runs, mut morning, mut afternoons) = (0, 0, 0);
    for run in batch_of(&variant, SEEDS) {
        let worked = work_the_run(&run, config);
        let (sunk, in_the_morning, after) = check_sinking(&run, &worked);
        sunk_runs += i32::from(sunk);
        morning += i32::from(in_the_morning);
        afternoons += after;
    }
    eprintln!("bombs at 0.8: carrier B sunk in {sunk_runs} runs, {morning} by the morning attack, {afternoons} afternoon attacks after it");
    assert!(morning > 50 && afternoons > 50, "{morning} {afternoons}");
}

// ---------------------------------------------------------------------
// The variants: the share, and the second carrier's fighters.
// ---------------------------------------------------------------------

/// Loads the scenario on the frozen figures with each exact, asserted-unique
/// text replacement made in its text. Nothing in the repository's data folder
/// is touched.
fn variant(tag: &str, edits: &[(&str, &str)]) -> Scenario {
    variant_of_data(tag, edits, &[], &[])
}

/// As `variant`, and makes the same kind of replacement in a copy of the
/// frozen aircraft types file and air combat figures, for a variant that
/// changes a figure and not an order.
fn variant_of_data(
    tag: &str,
    edits: &[(&str, &str)],
    aircraft_edits: &[(&str, &str)],
    air_combat_edits: &[(&str, &str)],
) -> Scenario {
    let path = scenario_path();
    let mut text = fs::read_to_string(&path).expect("read the scenario");
    for (original, changed) in edits {
        text = common::replace_once(&text, original, changed);
    }
    let options = common::altered_reference(tag, |dir| {
        common::edit_file_by(dir, "aircraft_types.csv", aircraft_edits);
        common::edit_file_by(dir, "air_combat.toml", air_combat_edits);
    });
    common::from_text_with(&text, &path, &options).expect("the variant loads")
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

/// The placeholder fighter's radius is 190 nm, not 250.
const FIGHTER_ROW: &str = "4,Placeholder fighter,fighter,0,0,250,none,0,0,0";

fn short_legged_fighters() -> Scenario {
    variant_of_data(
        "radius_190",
        &[],
        &[(
            FIGHTER_ROW,
            "4,Placeholder fighter,fighter,0,0,190,none,0,0,0",
        )],
        &[],
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
        bomb_tenths: 2,
    };
    let mut firsts = Vec::new();
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
        for first in work_the_run(&run, config).into_iter().take(1) {
            // 12 shots at 0.3 for the patrol and 36 at 0.05 for the guns:
            // 12.6 bombers attack on average.
            assert_eq!(first.shots_at_bombers, 12);
            assert_eq!(first.gun_shots, 36);
            assert!(first.patrol_at_bombers.is_some() && first.guns.is_some());
            assert!(first.escorts_at_patrol.is_none() && first.patrol_at_escorts.is_none());
            firsts.push(first);
        }
    }
    assert!(firsts.len() > 400);
    assert_first_attacks_by_the_rule(&firsts);
    let attacking: BTreeSet<u32> = firsts.iter().map(|w| w.bombers_attacking).collect();
    assert!(attacking.len() > 3, "{attacking:?}");
}

#[test]
fn with_a_radius_of_190_group_3_sends_no_escorts_and_keeps_all_12_over_carrier_a() {
    let variant = short_legged_fighters();
    // The share order is still 0.5: it is the radius that keeps the escorts
    // home, and everything flyable is patrol, as with a share of 1.
    let config = Config {
        share_tenths: 10,
        group_4_on_patrol: true,
        bomb_tenths: 2,
    };
    let mut firsts = Vec::new();
    let mut runs_out_of_reach = 0;
    for run in batch_of(&variant, SEEDS) {
        assert_eq!(run.share_of_3, tenths(5));
        // A report can put the contact a hex off, so a strike can be 160,
        // 200 or 240 nm out. Escorts go with each strike within 190 nm and
        // with none beyond it (decision 0030).
        let distances: Vec<i64> = run
            .of(|k| matches!(k, EventKind::StrikeLaunched { .. }))
            .iter()
            .map(|launch| match launch.kind {
                EventKind::StrikeLaunched { distance_nm, .. } => distance_nm.trunc(),
                _ => unreachable!(),
            })
            .collect();
        let sent = run.of(|k| matches!(k, EventKind::EscortsSent { .. })).len();
        assert_eq!(
            sent,
            distances
                .iter()
                .filter(|distance| **distance <= 190)
                .count(),
            "seed {}",
            run.seed
        );
        // The figures of the row are for a run whose strikes are all beyond
        // the radius.
        if sent > 0 {
            continue;
        }
        runs_out_of_reach += 1;
        // Group 3 never leaves carrier A and loses no aircraft.
        assert_eq!(run.ready[2], 12, "seed {}", run.seed);
        for (index, fight) in run
            .of(|k| matches!(k, EventKind::AirFight { .. }))
            .iter()
            .enumerate()
        {
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
            if index == 0 {
                assert_eq!(shots, Fixed::from_int(12));
            }
        }
        for first in work_the_run(&run, config).into_iter().take(1) {
            assert_eq!(first.shots_at_bombers, 12);
            assert_eq!(first.gun_shots, 36);
            assert!(first.patrol_at_bombers.is_some() && first.guns.is_some());
            assert!(first.escorts_at_patrol.is_none() && first.patrol_at_escorts.is_none());
            firsts.push(first);
        }
    }
    assert!(runs_out_of_reach > 500);
    assert_first_attacks_by_the_rule(&firsts);
    let attacking: BTreeSet<u32> = firsts.iter().map(|w| w.bombers_attacking).collect();
    assert!(attacking.len() > 3, "{attacking:?}");
}

#[test]
fn a_share_of_0_sends_12_escorts_and_the_patrol_leaves_the_bombers_alone() {
    let variant = share_variant("0");
    let config = Config {
        share_tenths: 0,
        group_4_on_patrol: true,
        bomb_tenths: 2,
    };
    let firsts = first_attacks(&variant, config);
    for first in &firsts {
        assert_eq!((first.escorts, first.patrol), (12, 12));
        // 12 shots each way at 0.3 (3.6 expected each way); both fought.
        assert!(first.escorts_at_patrol.is_some() && first.patrol_at_escorts.is_some());
        // None at the bombers, and the guns 36 shots at 0.05: 16.2 attack on
        // average.
        assert_eq!(first.shots_at_bombers, 0);
        assert!(first.patrol_at_bombers.is_none());
        assert_eq!(first.gun_shots, 36);
        assert!(first.guns.is_some());
    }
    assert_first_attacks_by_the_rule(&firsts);
    let attacking: BTreeSet<u32> = firsts.iter().map(|w| w.bombers_attacking).collect();
    assert!(attacking.len() > 3, "{attacking:?}");
}

#[test]
fn with_group_4_not_on_patrol_nothing_fights_and_only_the_guns_cost_bombers() {
    let variant = group_4_at_rest();
    let config = Config {
        share_tenths: 5,
        group_4_on_patrol: false,
        bomb_tenths: 2,
    };
    let runs = batch_of(&variant, SEEDS);
    let mut firsts = Vec::new();
    for run in &runs {
        assert!(run
            .of(|k| matches!(k, EventKind::AirFight { .. }))
            .is_empty());
        let worked = work_the_run(run, config);
        // Group 4 loses nothing unless carrier B is sunk, and then it is
        // lost with her.
        let sunk = worked.iter().any(|attack| attack.sank_b);
        assert_eq!(run.ready[3], if sunk { 0 } else { 12 }, "seed {}", run.seed);
        if let Some(first) = worked.into_iter().next() {
            assert!(
                first.escorts_at_patrol.is_none()
                    && first.patrol_at_escorts.is_none()
                    && first.patrol_at_bombers.is_none()
            );
            assert_eq!(first.gun_shots, 36);
            assert!(first.guns.is_some());
            firsts.push(first);
        }
    }
    assert!(firsts.len() > 400);
    assert_first_attacks_by_the_rule(&firsts);
    let attacking: BTreeSet<u32> = firsts.iter().map(|w| w.bombers_attacking).collect();
    assert!(attacking.len() > 2, "{attacking:?}");
}

// ---------------------------------------------------------------------
// Properties.
// ---------------------------------------------------------------------

#[test]
fn aircraft_come_home_elsewhere_only_when_carrier_bs_deck_sank_or_went_out_in_the_pulse() {
    // Decisions 7 to 10 of 0059: only air group 4, aboard carrier B, is ever
    // affected, only in a daylight pulse in which she sank or her flight deck
    // went out, and its aircraft in the air then come down in the sea. Group
    // 4 ends with none if she sank.
    let ship_of = |group: AirGroupId| (group == FIGHTERS_B).then_some(CARRIER_B);
    let mut homecomings = 0;
    for run in batch() {
        homecomings += common::no_deck::assert_only_for_lost_decks(&run.events, ship_of);
        assert!(run
            .events
            .iter()
            .all(|e| !matches!(e.kind, EventKind::AircraftLandedElsewhere { .. })));
        if run.ship(3).sunk {
            assert_eq!(run.ready[3], 0, "seed {}", run.seed);
        }
    }
    assert!(homecomings > 0, "carrier B is sunk or hit out in some runs");
}

#[test]
fn every_groups_aircraft_at_the_end_are_those_at_the_start_less_its_recorded_losses() {
    for run in batch() {
        // The aircraft a group lost: its loss events, those aboard its carrier
        // when she sank (air group 4 with carrier B) and those of its own in
        // the air that came down in the sea for want of a deck.
        let lost = |group: AirGroupId| -> u32 {
            run.events
                .iter()
                .map(|event| match event.kind {
                    EventKind::AircraftLost {
                        air_group,
                        aircraft,
                        ..
                    }
                    | EventKind::AirGroupLost {
                        air_group,
                        aircraft,
                        ..
                    }
                    // What came down in the sea for want of a deck (decisions
                    // 7 to 10 of 0059). Nothing lands elsewhere here: there
                    // is no other carrier and no base.
                    | EventKind::AircraftDitchedNoDeck {
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
        // group 3 loses only its escorts, at most 6 an attack and no more
        // than the escorts sent.
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
        assert_eq!(
            run.ready[2],
            12 - lost(FIGHTERS_A),
            "seed {}: the aircraft kept over carrier A are not fired at",
            run.seed
        );
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
        bomb_tenths: 2,
    };
    // Share 1: no escorts, the patrol of 12 fires all 12 at the bombers.
    // Share 0.5: 6 escorts, 6 at the bombers. Share 0: 12 escorts, none.
    // The shots are exact given the counts, attack by attack.
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
    // Each shot is rolled (decision 0033), so one seed can go either way and
    // no run-by-run statement holds. What does hold attack by attack is the
    // expectation: the patrol's shots at the bombers x 0.3 never rises with
    // the escorts (3.6, 1.8, 0). The losses themselves are compared as
    // totals over the 1,000 seeds: the mean number of bombers attacking in
    // the first attacks rises as the share falls, 12.6, 14.4, 16.2.
    let summary = |scenario: &Scenario, config: Config| -> (Fixed, i64) {
        let firsts = first_attacks(scenario, config);
        let expected_to_fighters: BTreeSet<Fixed> = firsts
            .iter()
            .map(|w| int(w.shots_at_bombers) * tenths(3))
            .collect();
        assert_eq!(expected_to_fighters.len(), 1);
        let total: i64 = firsts.iter().map(|w| i64::from(w.bombers_attacking)).sum();
        (
            *expected_to_fighters.first().expect("one"),
            total * 1000 / count(firsts.len()),
        )
    };
    let with = |share_tenths| Config {
        share_tenths,
        group_4_on_patrol: true,
        bomb_tenths: 2,
    };
    let (expected_1, share_1) = summary(&share_variant("1"), with(10));
    let (expected_half, share_half) = summary(&load(), SCENARIO);
    let (expected_0, share_0) = summary(&share_variant("0"), with(0));
    let (expected_rested, rested) = summary(
        &group_4_at_rest(),
        Config {
            share_tenths: 5,
            group_4_on_patrol: false,
            bomb_tenths: 2,
        },
    );
    assert_eq!(
        [expected_1, expected_half, expected_0, expected_rested],
        [tenths(36), tenths(18), Fixed::ZERO, Fixed::ZERO]
    );
    assert!(expected_1 >= expected_half && expected_half >= expected_0);
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
