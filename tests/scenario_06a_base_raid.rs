//! Scenario 6a: bombers with an escort raid an airfield for three days.
//!
//! Stage 6a of scenario 6 (decision 0034). The data is
//! `data/scenarios/06a_base_raid.toml`, run with seeds 1 to 1000. Everything
//! in it is a placeholder.
//!
//! # Hand calculation
//!
//! ## The setting
//!
//! Airfield A, base 1, is Japanese and at (0,0). Airfield B, base 2, is
//! Allied and at (5,0): 5 hexes, 200 nm. Airfield B has 1,000 t of supply
//! and 20 anti-aircraft shots. Airfield A has no guns and nothing attacks
//! it.
//!
//! | Group | Side | At | Type | Aircraft | Mission |
//! | ----- | ---- | -- | ---- | -------- | ------- |
//! | 1 | Japanese | A | dive bomber, radius 250 nm | 18 | base strike on airfield B |
//! | 2 | Japanese | A | fighter, radius 250 nm | 12 | combat air patrol, share 0.5 |
//! | 3 | Allied | B | fighter | 12 | combat air patrol, share 1 |
//! | 4 | Allied | B | dive bomber | 10 | rest |
//! | 5 | Allied | B | scout | 6 | search |
//!
//! A fighter's shot hits with chance 0.3, an anti-aircraft shot with 0.05
//! and a bomb with 0.2. A hit on a base lands on aircraft on the ground
//! with chance 0.3, on supply with chance 0.3, and on nothing with chance
//! 0.4. A hit on supply destroys 50 t.
//!
//! ## When a raid flies
//!
//! Group 1 launches in the morning pulse of each day, if it has an aircraft:
//! airfield B is 200 nm away, within its 250. It launches in no other
//! pulse. It needs no contact, and no report is made at any time: nobody
//! sights a ship, because there are none. So there are at most three raids,
//! on the mornings of days 1, 2 and 3, and none once group 1 has no
//! aircraft.
//!
//! The Allied groups have no strike mission and never launch.
//!
//! ## The rule, for any raid
//!
//! Write B for group 1's aircraft, F for group 2's and P for group 3's, at
//! the start of the pulse. A group at a base flies all its aircraft.
//!
//! - Group 2 keeps F x 0.5, rounded down, over airfield A. The rest, E,
//!   escort the raid: airfield B is within the fighter's 250 nm.
//! - The raid is alone, so it meets the whole patrol, P.
//! - The escorts fire E shots at the patrol, if there is a patrol.
//! - The patrol fires at the escorts the smaller of P and E shots, and the
//!   rest of P at the bombers.
//! - Airfield B's guns fire 20 shots at the bombers left.
//! - The bombers left drop one bomb each at 0.2.
//! - For each hit, one draw: aircraft on the ground, supply, or nothing.
//!
//! Each shot is rolled on its own (decision 0033): n shots at chance p
//! score from no hit to n, n x p on average, with a standard deviation of
//! the square root of n x p x (1 - p). A group cannot lose more than it
//! has.
//!
//! The aircraft on the ground at airfield B are group 4's, which rests.
//! Group 3 is on patrol and group 5 is searching: both are in the air, and
//! no bomb touches them. Group 5 has 6 aircraft at the end of every run.
//!
//! - Hits on aircraft destroy that many of group 4's aircraft, and no more
//!   than it has at the start of the pulse. A hit on aircraft after the
//!   last one is destroyed does nothing.
//! - Hits on supply destroy 50 t each, and no more than the base has.
//!
//! ## The first raid
//!
//! On the morning of day 1 every group is whole: B = 18, F = 12 so 6 stay
//! and E = 6, P = 12.
//!
//! | Step                  | Shots | Chance | Expected | Std dev  | Lost |
//! | --------------------- | ----- | ------ | -------- | -------- | ---- |
//! | Escorts at the patrol | 6     | 0.3    | 1.8      | 1.122497 | 0 to 6 of group 3 |
//! | Patrol at the escorts | 6     | 0.3    | 1.8      | 1.122497 | 0 to 6 of group 2 |
//! | Patrol at the bombers | 6     | 0.3    | 1.8      | 1.122497 | 0 to 6 of group 1 |
//! | Guns at the bombers   | 20    | 0.05   | 1        | 0.974679 | 0 to those left of group 1 |
//!
//! The bombers that attack are 18 less the two losses of group 1, 15.2 on
//! average, and score 0.2 of their number in hits on average, 3.04. Of the
//! hits, 0.3 land on aircraft and 0.3 on supply on average.
//!
//! Over seeds 1 to 1000 the first raid always flies. The hits of each of
//! the four rolls, summed over the runs, are within four standard
//! deviations of the expected figure times 1,000. The bomb hits are within
//! four standard deviations of 0.2 times the bombers that attacked. Of all
//! the bomb hits of all the raids, the number that land on aircraft is
//! within four standard deviations of 0.3 of them, and so is the number
//! that land on supply.
//!
//! ## Later raids
//!
//! The raids of days 2 and 3 are worked by the same rule from what each
//! group has that morning. The test works every raid of every run from the
//! counts at the start of its pulse.
//!
//! ## At the end of a run
//!
//! - Group 4 has 10 less the aircraft destroyed on the ground, which is the
//!   sum over the raids of the smaller of that raid's hits on aircraft and
//!   what group 4 had that morning.
//! - Airfield B has 1,000 t less 50 t for each hit on supply. Three raids
//!   of 18 bombers cannot score the 20 supply hits that would empty it
//!   unless nearly every bomb hits and lands on supply, so the cap is
//!   tested in a variant.
//! - Group 1 has 18 less its losses to fighters and guns, group 2 has 12
//!   less its losses to the patrol, and group 3 has 12 less its losses to
//!   the escorts. Nothing else changes any group.
//! - Airfield A's supply is 0 throughout.
//!
//! ## Variants
//!
//! Each differs from the scenario in one thing, and the figures are for its
//! first raid.
//!
//! | Variant | What changes |
//! | ------- | ------------ |
//! | Group 4 flies fighters and is on combat air patrol | The patrol is 22: 6 shots at the escorts and 16 at the bombers, 4.8 expected. Nothing is on the ground: no aircraft is destroyed on the ground in any run, whatever the hits on aircraft |
//! | Group 3 rests | No patrol: the escorts fire at nothing, and only the guns cost bombers. 22 aircraft are on the ground, and those destroyed come off group 3 first |
//! | Airfield B has no guns | No anti-aircraft fire is recorded. The bombers attacking are 18 less the patrol's hits |
//! | Airfield B is at (7,0), 280 nm | Beyond the bombers' 250 nm: group 1 never launches and nothing happens in any pulse |
//! | Airfield B has 100 t of supply and a bomb always hits and always lands on supply | Every raid destroys what is left and no more: the supply is 0 after the first raid with two or more bombers attacking, and never below 0 |
//! | Group 1 has the mission naval strike | It never launches: there is no contact. A base strike and a naval strike are different orders |
//!
//! (The first variant first read "group 4 is on combat air patrol". Only a
//! fighter type can fly that mission, decision 0027, so the variant gives
//! the group fighters too. No figure changed.)
//!
//! ## What cannot happen
//!
//! - No raid flies in a night or an afternoon pulse.
//! - No aircraft of group 5 is ever lost, and none of group 3 is destroyed
//!   on the ground.
//! - No report is made and no contact is held by either side.
//! - Airfield A is never attacked.
//!
//! # Properties
//!
//! - Every group's aircraft at the end are those at the start less its
//!   recorded losses, and the supply at the end is that at the start less
//!   the recorded destruction.
//! - Supply is never negative and no group has fewer than no aircraft.
//! - More guns never mean fewer anti-aircraft shots, and the expected loss
//!   to guns never falls with more guns.
//! - More escorts never mean more patrol shots at the bombers.
//! - More aircraft in the air over the base never mean more aircraft
//!   destroyed on the ground: the variant with group 4 on patrol destroys
//!   none.
//! - Recording breakdowns changes neither the events nor the hashes.
//! - The same seed gives the same hashes twice.

// The test code is written by a developer from the hand calculation above.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;

use sim_core::events::{AirFirer, AirTarget, Cause, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, BaseId, EntityRef, EventId, OrderId};
use sim_core::orders::AirMission;
use sim_core::rng::{RngKey, RngStream, Subsystem};
use sim_core::state::Side;
use sim_core::time::Pulse;
use sim_data::Scenario;

const BOMBERS: u32 = 1;
const ESCORTS: u32 = 2;
const DEFENDERS: u32 = 3;
const RESTING: u32 = 4;
const SCOUTS: u32 = 5;
const AIRFIELD_A: BaseId = BaseId::new(1);
const AIRFIELD_B: BaseId = BaseId::new(2);
const SEEDS: u64 = 1000;
const DAYS: u32 = 3;
const START: [u32; 5] = [18, 12, 12, 10, 6];

fn scenario_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data/scenarios/06a_base_raid.toml")
}

fn load() -> Scenario {
    Scenario::load(&scenario_path()).expect("scenario 6a loads")
}

fn tenths(value: i64) -> Fixed {
    Fixed::from_ratio(value, 10)
}

fn hundredths(value: i64) -> Fixed {
    Fixed::from_ratio(value, 100)
}

fn int(value: u32) -> Fixed {
    Fixed::from_int(i64::from(value))
}

fn group(id: u32) -> AirGroupId {
    AirGroupId::new(id)
}

/// Whether k out of n is within four standard deviations of probability
/// `p` (in millionths), in exact integer arithmetic.
fn within_four_sigma(k: i64, n: i64, p: i64) -> bool {
    let (k, n, p) = (i128::from(k), i128::from(n), i128::from(p));
    let scale = 1_000_000;
    let off = k * scale - n * p;
    off * off <= 16 * n * p * (scale - p)
}

fn count(value: usize) -> i64 {
    i64::try_from(value).expect("small")
}

// ---------------------------------------------------------------------
// What one run did, read from its events and final state.
// ---------------------------------------------------------------------

struct Run {
    seed: u64,
    events: Vec<Event>,
    hashes: Vec<u64>,
    /// The aircraft ready at the end of groups 1 to 5.
    ready: [u32; 5],
    /// The supply of airfields A and B at the end.
    supply: [Fixed; 2],
    /// The contacts each side holds at the end, Allied then Japanese.
    contacts: [usize; 2],
    /// The orders accepted for each air group, in the order of their ids.
    orders: BTreeMap<u32, Vec<OrderId>>,
}

impl Run {
    fn of(&self, keep: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|event| keep(&event.kind))
            .collect()
    }

    fn orders_of(&self, group: u32) -> Vec<OrderId> {
        self.orders.get(&group).cloned().unwrap_or_default()
    }
}

fn run_of(base: &Scenario, seed: u64) -> Run {
    let mut run = base.with_seed(seed).expect("a seed can be set");
    run.run_to_end().expect("the run resolves");
    digest(&run, seed)
}

fn digest(run: &Scenario, seed: u64) -> Run {
    let truth = run.world.umpire();
    let events: Vec<Event> = run.world.events().iter().cloned().collect();
    let mut orders: BTreeMap<u32, Vec<OrderId>> = BTreeMap::new();
    for event in &events {
        if let EventKind::OrderAccepted {
            order,
            target: EntityRef::AirGroup(target),
        } = event.kind
        {
            orders.entry(target.raw()).or_default().push(order);
        }
    }
    Run {
        seed,
        events,
        hashes: run.world.day_hashes().to_vec(),
        ready: [BOMBERS, ESCORTS, DEFENDERS, RESTING, SCOUTS]
            .map(|id| truth.air_groups[&group(id)].ready),
        supply: [AIRFIELD_A, AIRFIELD_B].map(|id| truth.bases[&id].supply_t),
        contacts: [Side::Allied, Side::Japanese]
            .map(|side| run.world.side_view(side).contacts().count()),
        orders,
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

/// Every raid of every run of the scenario, worked and asserted, once.
fn worked_batch() -> &'static Vec<Vec<Raid>> {
    static WORKED: OnceLock<Vec<Vec<Raid>>> = OnceLock::new();
    WORKED.get_or_init(|| {
        batch()
            .iter()
            .map(|run| work_the_run(run, &scenario_config()))
            .collect()
    })
}

/// The first raid of each run of a set of runs worked by `config`.
fn first_raids(runs: &[Run], config: &Config) -> Vec<Raid> {
    let firsts: Vec<Raid> = runs
        .iter()
        .map(|run| {
            work_the_run(run, config)
                .into_iter()
                .next()
                .expect("the first raid flies")
        })
        .collect();
    assert_eq!(firsts.len(), runs.len());
    firsts
}

fn stream_of(seed: u64, day: u32, pulse: Pulse, entity: EntityRef, other: EntityRef) -> RngStream {
    RngKey {
        seed,
        day,
        pulse: Some(pulse),
        subsystem: Subsystem::AirCombat,
        entity,
        other: Some(other),
    }
    .stream()
}

/// The hits a roll of `shots` at `chance` gives from its stream: one draw
/// for each shot (decision 0033).
fn hits_from(stream: &mut RngStream, shots: u32, chance: Fixed) -> u32 {
    (0..shots).map(|_| u32::from(stream.chance(chance))).sum()
}

fn shots_text(shots: u32, chance: Fixed, expected: Fixed, hits: u32) -> String {
    format!(
        "{shots} shots x {chance} = {expected} expected, {hits} {}",
        if hits == 1 { "hit" } else { "hits" }
    )
}

/// Takes `hits` from the pool, the lowest group first, each until it has none
/// left. Returns what was taken from each group.
fn take(hits: u32, pool: &mut [(u32, u32)]) -> Vec<(u32, u32)> {
    let mut left = hits;
    let mut taken = Vec::new();
    for (group, count) in pool.iter_mut() {
        let now = left.min(*count);
        if now > 0 {
            *count -= now;
            left -= now;
            taken.push((*group, now));
        }
    }
    taken
}

// ---------------------------------------------------------------------
// Working every raid of a run from the counts at the start of its pulse.
// ---------------------------------------------------------------------

/// What differs between the scenario and its variants.
#[derive(Clone)]
struct Config {
    /// The share of group 2 kept over airfield A, in tenths.
    keep_tenths: u32,
    /// Airfield B's anti-aircraft shots.
    guns: u32,
    /// The Allied groups on combat air patrol over airfield B.
    patrol_groups: Vec<u32>,
    /// The Allied groups that are on the ground there: the others of groups
    /// 3 and 4 are in the air or not there.
    ground_groups: Vec<u32>,
    /// The chance a bomb hits, and where a hit lands.
    bomb: Fixed,
    lands_on_aircraft: Fixed,
    lands_on_supply: Fixed,
    /// Airfield B's supply at the start, in tons.
    supply: i64,
}

fn scenario_config() -> Config {
    Config {
        keep_tenths: 5,
        guns: 20,
        patrol_groups: vec![DEFENDERS],
        ground_groups: vec![RESTING],
        bomb: tenths(2),
        lands_on_aircraft: tenths(3),
        lands_on_supply: tenths(3),
        supply: 1000,
    }
}

/// One raid as the rule works it, with what it found in the events.
struct Raid {
    day: u32,
    /// B, F, E and P of the hand calculation.
    bombers: u32,
    fighters: u32,
    escorts: u32,
    patrol: u32,
    /// The aircraft on the ground at the airfield at the start.
    on_ground: u32,
    /// The shots and hits of each roll, or `None` for a roll not made.
    escorts_at_patrol: Option<(u32, u32)>,
    patrol_at_escorts: Option<(u32, u32)>,
    patrol_at_bombers: Option<(u32, u32)>,
    guns: Option<(u32, u32)>,
    bombers_attacking: u32,
    bomb_hits: u32,
    on_aircraft: u32,
    on_supply: u32,
    /// Aircraft destroyed on the ground and tons of supply destroyed.
    destroyed_on_ground: u32,
    supply_destroyed: i64,
    /// Group 1's losses in the air to the patrol and to the guns.
    bombers_lost: u32,
}

/// Works every raid of `run` by the rule of the hand calculation from the
/// counts at the start of its pulse, asserts every event of the air combat
/// step against it, and returns what was worked. The counts are carried
/// from raid to raid by the rule, not read from the events.
fn work_the_run(run: &Run, config: &Config) -> Vec<Raid> {
    let seed = run.seed;
    let mut ready = START;
    let mut supply = config.supply;
    let per_hit = 50;
    let mut worked = Vec::new();
    let of = |group: u32| usize::try_from(group - 1).expect("a group number");
    for day in 1..=DAYS {
        let launches: Vec<(usize, &Event)> = run
            .events
            .iter()
            .enumerate()
            .filter(|(_, event)| {
                event.day == day && matches!(event.kind, EventKind::BaseStrikeLaunched { .. })
            })
            .collect();
        let b = ready[of(BOMBERS)];
        if b == 0 {
            assert!(
                launches.is_empty(),
                "seed {seed}, day {day}: no aircraft, no raid"
            );
            continue;
        }
        assert_eq!(launches.len(), 1, "seed {seed}, day {day}");
        let (index, launch) = launches[0];
        let pulse = Pulse::Morning;
        assert_eq!(
            launch.pulse,
            Some(pulse),
            "seed {seed}: a raid launches in the morning"
        );
        assert_eq!(
            launch.kind,
            EventKind::BaseStrikeLaunched {
                air_group: group(BOMBERS),
                base: AIRFIELD_B,
                hex: Hex::new(5, 0),
                aircraft: b,
                distance_nm: Fixed::from_int(200),
            },
            "seed {seed}"
        );
        assert_eq!(
            launch.kind.to_string(),
            format!("air_group:1 launched {b} aircraft at base:2 at (5,0), 200 nm"),
            "seed {seed}"
        );
        assert_eq!(
            launch.causes,
            run.orders_of(BOMBERS)
                .into_iter()
                .map(Cause::Order)
                .collect::<Vec<_>>(),
            "seed {seed}: the launch cites the mission order alone"
        );
        let step: Vec<&Event> = run.events[index + 1..]
            .iter()
            .take_while(|event| event.day == day && event.pulse == Some(pulse))
            .collect();
        let kinds = |keep: fn(&EventKind) -> bool| -> Vec<&Event> {
            step.iter().copied().filter(|e| keep(&e.kind)).collect()
        };
        let sent = kinds(|k| matches!(k, EventKind::EscortsSent { .. }));
        let fights = kinds(|k| matches!(k, EventKind::AirFight { .. }));
        let guns = kinds(|k| matches!(k, EventKind::BaseAntiAircraftFire { .. }));
        let lost = kinds(|k| matches!(k, EventKind::AircraftLost { .. }));
        let attacked = kinds(|k| matches!(k, EventKind::BaseAttacked { .. }));
        let on_ground_events = kinds(|k| matches!(k, EventKind::AircraftDestroyedOnGround { .. }));
        let supply_events = kinds(|k| matches!(k, EventKind::BaseSupplyDestroyed { .. }));
        // No ship was ever there: nothing else is recorded by the raid.
        assert!(
            kinds(|k| matches!(
                k,
                EventKind::StrikeLaunched { .. }
                    | EventKind::StrikeFoundNothing { .. }
                    | EventKind::AirAttack { .. }
                    | EventKind::AntiAircraftFire { .. }
                    | EventKind::ReportReceived { .. }
                    | EventKind::ContactNotThere { .. }
            ))
            .is_empty(),
            "seed {seed}"
        );

        // F, E, P and B at the start of the pulse.
        let f = ready[of(ESCORTS)];
        let e = f - f * config.keep_tenths / 10;
        let mut patrol_pool: Vec<(u32, u32)> = config
            .patrol_groups
            .iter()
            .map(|g| (*g, ready[of(*g)]))
            .filter(|(_, count)| *count > 0)
            .collect();
        let p: u32 = patrol_pool.iter().map(|(_, count)| count).sum();
        let mut ground_pool: Vec<(u32, u32)> = config
            .ground_groups
            .iter()
            .map(|g| (*g, ready[of(*g)]))
            .filter(|(_, count)| *count > 0)
            .collect();
        let on_ground: u32 = ground_pool.iter().map(|(_, count)| count).sum();

        // The escorts go if there are any.
        if e > 0 {
            assert_eq!(sent.len(), 1, "seed {seed}");
            assert_eq!(
                sent[0].kind,
                EventKind::EscortsSent {
                    air_group: group(ESCORTS),
                    with_air_group: group(BOMBERS),
                    aircraft: e
                },
                "seed {seed}"
            );
            assert_eq!(
                sent[0].kind.to_string(),
                format!("air_group:2 sent {e} escorts with air_group:1")
            );
            let mut causes: Vec<Cause> = run
                .orders_of(ESCORTS)
                .into_iter()
                .map(Cause::Order)
                .collect();
            causes.push(Cause::Event(launch.id));
            assert_eq!(sent[0].causes, causes, "seed {seed}");
        } else {
            assert!(sent.is_empty(), "seed {seed}");
        }

        // The fights, on the counts at the start. The raid is alone, so it
        // meets the whole patrol.
        let chance = tenths(3);
        let first_patrol = patrol_pool.first().map(|(g, _)| *g);
        let mut planned: Vec<(u32, u32, AirFirer, AirTarget, u32)> = Vec::new();
        if let Some(first_patrol) = first_patrol {
            if e > 0 {
                planned.push((
                    ESCORTS,
                    first_patrol,
                    AirFirer::Escorts,
                    AirTarget::Patrol,
                    e,
                ));
                planned.push((
                    first_patrol,
                    ESCORTS,
                    AirFirer::Patrol,
                    AirTarget::Escorts,
                    p.min(e),
                ));
            }
            let at_bombers = p - p.min(e);
            if at_bombers > 0 {
                planned.push((
                    first_patrol,
                    BOMBERS,
                    AirFirer::Patrol,
                    AirTarget::Bombers,
                    at_bombers,
                ));
            }
        }
        assert_eq!(fights.len(), planned.len(), "seed {seed}, day {day}");
        let mut escort_stream = stream_of(
            seed,
            day,
            pulse,
            EntityRef::AirGroup(group(ESCORTS)),
            EntityRef::Base(AIRFIELD_B),
        );
        let mut patrol_stream = first_patrol.map(|first| {
            stream_of(
                seed,
                day,
                pulse,
                EntityRef::AirGroup(group(first)),
                EntityRef::AirGroup(group(BOMBERS)),
            )
        });
        let (mut r_escorts, mut r_patrol_e, mut r_patrol_b) = (None, None, None);
        // (group, aircraft, remaining, the roll that cost it) in order.
        let mut expected_losses: Vec<(u32, u32, u32, EventId)> = Vec::new();
        let mut now = ready;
        let mut bomber_pool = [(BOMBERS, b)];
        let mut escort_pool = [(ESCORTS, e)];
        for ((firer, target, firing, at, shots), event) in planned.iter().zip(&fights) {
            let hits = match at {
                AirTarget::Patrol => hits_from(&mut escort_stream, *shots, chance),
                _ => hits_from(patrol_stream.as_mut().expect("a patrol"), *shots, chance),
            };
            let expected = int(*shots) * chance;
            assert_eq!(
                event.kind,
                EventKind::AirFight {
                    firer: group(*firer),
                    target: group(*target),
                    firing: *firing,
                    at: *at,
                    shots: int(*shots),
                    chance,
                    expected,
                    hits
                },
                "seed {seed}, day {day}"
            );
            let who = if *firing == AirFirer::Escorts {
                "escorts"
            } else {
                "patrol"
            };
            let what = match at {
                AirTarget::Patrol => "patrol",
                AirTarget::Escorts => "escorts",
                AirTarget::Bombers => "bombers",
            };
            assert_eq!(
                event.kind.to_string(),
                format!(
                    "air_group:{firer} {who} at the {what} of air_group:{target}: {}",
                    shots_text(*shots, chance, expected, hits)
                ),
                "seed {seed}"
            );
            let mut causes = vec![Cause::Event(launch.id)];
            if *firing == AirFirer::Escorts || *at == AirTarget::Escorts {
                causes.push(Cause::Event(sent[0].id));
            }
            assert_eq!(event.causes, causes, "seed {seed}");
            let breakdown = event.breakdown.as_ref().expect("breakdown");
            assert_eq!(breakdown.result, expected);
            let mut factors = if *firing == AirFirer::Patrol {
                vec![
                    ("patrol", int(p)),
                    ("raid_aircraft", int(b + e)),
                    ("all_raids_aircraft", int(b + e)),
                    ("patrol_share", int(p)),
                ]
            } else {
                Vec::new()
            };
            factors.extend([("shots", int(*shots)), ("hit_chance", chance)]);
            assert_eq!(
                breakdown
                    .factors
                    .iter()
                    .map(|f| (f.name, f.value))
                    .collect::<Vec<_>>(),
                factors
            );
            // A group cannot lose more than it has: the losses come off the
            // pool the roll was at, the lowest group first.
            let taken = match at {
                AirTarget::Patrol => {
                    r_escorts = Some((*shots, hits));
                    take(hits, &mut patrol_pool)
                }
                AirTarget::Escorts => {
                    r_patrol_e = Some((*shots, hits));
                    take(hits, &mut escort_pool)
                }
                AirTarget::Bombers => {
                    r_patrol_b = Some((*shots, hits));
                    take(hits, &mut bomber_pool)
                }
            };
            for (g, n) in taken {
                now[of(g)] -= n;
                expected_losses.push((g, n, now[of(g)], event.id));
            }
        }

        // The guns: airfield B's, at the bombers left, from the stream of
        // (the airfield, the raid's first group).
        let mut r_guns = None;
        if bomber_pool[0].1 > 0 && config.guns > 0 {
            assert_eq!(guns.len(), 1, "seed {seed}, day {day}");
            let mut stream = stream_of(
                seed,
                day,
                pulse,
                EntityRef::Base(AIRFIELD_B),
                EntityRef::AirGroup(group(BOMBERS)),
            );
            let hits = hits_from(&mut stream, config.guns, hundredths(5));
            let expected = int(config.guns) * hundredths(5);
            assert_eq!(
                guns[0].kind,
                EventKind::BaseAntiAircraftFire {
                    base: AIRFIELD_B,
                    air_group: group(BOMBERS),
                    shots: int(config.guns),
                    chance: hundredths(5),
                    expected,
                    hits
                },
                "seed {seed}"
            );
            assert_eq!(
                guns[0].kind.to_string(),
                format!(
                    "base:2 guns at the raid of air_group:1: {}",
                    shots_text(config.guns, hundredths(5), expected, hits)
                ),
                "seed {seed}"
            );
            assert_eq!(guns[0].causes, vec![Cause::Event(launch.id)], "seed {seed}");
            let breakdown = guns[0].breakdown.as_ref().expect("breakdown");
            assert_eq!(breakdown.result, expected);
            assert_eq!(
                breakdown
                    .factors
                    .iter()
                    .map(|f| (f.name, f.value))
                    .collect::<Vec<_>>(),
                vec![("shots", int(config.guns)), ("hit_chance", hundredths(5))]
            );
            r_guns = Some((config.guns, hits));
            for (g, n) in take(hits, &mut bomber_pool) {
                now[of(g)] -= n;
                expected_losses.push((g, n, now[of(g)], guns[0].id));
            }
        } else {
            assert!(guns.is_empty(), "seed {seed}, day {day}");
        }

        // The aircraft lost in the air, one event for each group and roll.
        assert_eq!(lost.len(), expected_losses.len(), "seed {seed}, day {day}");
        for (event, (g, n, remaining, cause)) in lost.iter().zip(&expected_losses) {
            assert_eq!(
                event.kind,
                EventKind::AircraftLost {
                    air_group: group(*g),
                    aircraft: *n,
                    remaining: *remaining
                },
                "seed {seed}"
            );
            assert_eq!(event.causes, vec![Cause::Event(*cause)], "seed {seed}");
        }
        let bombers_left = bomber_pool[0].1;
        let bombers_lost = b - bombers_left;

        // The bombers left each drop one bomb, whatever the type carries.
        let mut hits = 0;
        let (mut on_aircraft, mut on_supply) = (0, 0);
        let mut destroyed_on_ground = 0;
        let mut supply_destroyed = 0;
        if bombers_left == 0 {
            assert!(
                attacked.is_empty() && on_ground_events.is_empty() && supply_events.is_empty(),
                "seed {seed}, day {day}: no bombers left, no attack"
            );
        } else {
            assert_eq!(attacked.len(), 1, "seed {seed}, day {day}");
            let mut stream = stream_of(
                seed,
                day,
                pulse,
                EntityRef::AirGroup(group(BOMBERS)),
                EntityRef::Base(AIRFIELD_B),
            );
            hits = hits_from(&mut stream, bombers_left, config.bomb);
            let below_aircraft = config.lands_on_aircraft.raw().unsigned_abs();
            let below_supply = below_aircraft + config.lands_on_supply.raw().unsigned_abs();
            for _ in 0..hits {
                let draw = stream.below(Fixed::SCALE.unsigned_abs());
                if draw < below_aircraft {
                    on_aircraft += 1;
                } else if draw < below_supply {
                    on_supply += 1;
                }
            }
            let expected = int(bombers_left) * config.bomb;
            assert_eq!(
                attacked[0].kind,
                EventKind::BaseAttacked {
                    air_group: group(BOMBERS),
                    base: AIRFIELD_B,
                    shots: int(bombers_left),
                    chance: config.bomb,
                    expected,
                    hits,
                    hits_on_aircraft: on_aircraft,
                    hits_on_supply: on_supply,
                },
                "seed {seed}, day {day}"
            );
            assert_eq!(
                attacked[0].kind.to_string(),
                format!(
                    "air_group:1 bombs at base:2: {}: {on_aircraft} on aircraft, {on_supply} on supply, {} on nothing",
                    shots_text(bombers_left, config.bomb, expected, hits),
                    hits - on_aircraft - on_supply
                ),
                "seed {seed}"
            );
            // It cites the launch and every loss of the group's in the air.
            let mut causes = vec![Cause::Event(launch.id)];
            causes.extend(
                expected_losses
                    .iter()
                    .zip(&lost)
                    .filter(|((g, ..), _)| *g == BOMBERS)
                    .map(|(_, event)| Cause::Event(event.id)),
            );
            assert_eq!(attacked[0].causes, causes, "seed {seed}");
            let breakdown = attacked[0].breakdown.as_ref().expect("breakdown");
            assert_eq!(breakdown.result, expected);
            assert_eq!(
                breakdown
                    .factors
                    .iter()
                    .map(|f| (f.name, f.value))
                    .collect::<Vec<_>>(),
                vec![
                    ("aircraft", int(bombers_left)),
                    ("hit_chance", config.bomb),
                    ("hit_on_aircraft_chance", config.lands_on_aircraft),
                    ("hit_on_supply_chance", config.lands_on_supply),
                ]
            );

            // Hits on aircraft destroy aircraft on the ground, the lowest
            // group first, and no more than were there.
            let taken = take(on_aircraft, &mut ground_pool);
            assert_eq!(
                on_ground_events.len(),
                taken.len(),
                "seed {seed}, day {day}"
            );
            for (event, (g, n)) in on_ground_events.iter().zip(&taken) {
                now[of(*g)] -= n;
                destroyed_on_ground += n;
                assert_eq!(
                    event.kind,
                    EventKind::AircraftDestroyedOnGround {
                        air_group: group(*g),
                        base: AIRFIELD_B,
                        aircraft: *n,
                        remaining: now[of(*g)],
                    },
                    "seed {seed}"
                );
                assert_eq!(
                    event.kind.to_string(),
                    format!(
                        "air_group:{g} lost {n} aircraft on the ground at base:2, {} left",
                        now[of(*g)]
                    )
                );
                assert_eq!(
                    event.causes,
                    vec![Cause::Event(attacked[0].id)],
                    "seed {seed}"
                );
                let breakdown = event.breakdown.as_ref().expect("breakdown");
                assert_eq!(breakdown.result, int(on_aircraft.min(on_ground)));
                assert_eq!(
                    breakdown
                        .factors
                        .iter()
                        .map(|f| (f.name, f.value))
                        .collect::<Vec<_>>(),
                    vec![
                        ("hits_on_aircraft", int(on_aircraft)),
                        ("aircraft_on_ground", int(on_ground)),
                    ]
                );
            }
            // Hits on supply destroy 50 t each, and no more than it has.
            let wanted = per_hit * i64::from(on_supply);
            supply_destroyed = wanted.min(supply);
            if supply_destroyed > 0 {
                assert_eq!(supply_events.len(), 1, "seed {seed}, day {day}");
                let before = supply;
                supply -= supply_destroyed;
                assert_eq!(
                    supply_events[0].kind,
                    EventKind::BaseSupplyDestroyed {
                        base: AIRFIELD_B,
                        destroyed_t: Fixed::from_int(supply_destroyed),
                        supply_t: Fixed::from_int(supply),
                    },
                    "seed {seed}"
                );
                assert_eq!(
                    supply_events[0].kind.to_string(),
                    format!("base:2 lost {supply_destroyed} t of supply, {supply} t left")
                );
                assert_eq!(supply_events[0].causes, vec![Cause::Event(attacked[0].id)]);
                let breakdown = supply_events[0].breakdown.as_ref().expect("breakdown");
                assert_eq!(breakdown.result, Fixed::from_int(supply_destroyed));
                assert_eq!(
                    breakdown
                        .factors
                        .iter()
                        .map(|f| (f.name, f.value))
                        .collect::<Vec<_>>(),
                    vec![
                        ("hits_on_supply", int(on_supply)),
                        ("tons_per_hit", Fixed::from_int(per_hit)),
                        ("supply_before", Fixed::from_int(before)),
                    ]
                );
            } else {
                assert!(supply_events.is_empty(), "seed {seed}, day {day}");
            }
        }
        ready = now;
        worked.push(Raid {
            day,
            bombers: b,
            fighters: f,
            escorts: e,
            patrol: p,
            on_ground,
            escorts_at_patrol: r_escorts,
            patrol_at_escorts: r_patrol_e,
            patrol_at_bombers: r_patrol_b,
            guns: r_guns,
            bombers_attacking: bombers_left,
            bomb_hits: hits,
            on_aircraft,
            on_supply,
            destroyed_on_ground,
            supply_destroyed,
            bombers_lost,
        });
    }

    // The end of the run.
    assert_eq!(run.ready, ready, "seed {seed}: the counts at the end");
    assert_eq!(
        run.supply,
        [Fixed::ZERO, Fixed::from_int(supply)],
        "seed {seed}: the supply at the end"
    );
    worked
}

// ---------------------------------------------------------------------
// Variants: the scenario with one thing changed, written under the build
// directory. Nothing in the repository's data folder is touched.
// ---------------------------------------------------------------------

/// Copies the scenario and its data into a folder under the build directory,
/// makes each exact, asserted-unique text replacement in the copied files,
/// appends `append` to the scenario, and loads it.
fn variant(
    tag: &str,
    edits: &[(&str, &str)],
    append: &str,
    aircraft_edits: &[(&str, &str)],
    air_combat_edits: &[(&str, &str)],
) -> Scenario {
    let data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../data");
    // Tests run in parallel: each variant has a folder of its own.
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "base_raid_{tag}_{}_{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    for folder in ["scenarios", "map", "reference"] {
        fs::create_dir_all(root.join(folder)).expect("make the variant folder");
    }
    for file in [
        "map/placeholder_two_airfields.toml",
        "reference/ship_classes.csv",
        "reference/port_sizes.csv",
        "reference/aircraft_types.csv",
        "reference/search.toml",
        "reference/surface_combat.toml",
        "reference/air_combat.toml",
    ] {
        fs::copy(data.join(file), root.join(file)).expect("copy a data file");
    }
    let edit = |path: &Path, edits: &[(&str, &str)], append: &str| {
        let mut text = fs::read_to_string(path).expect("read a file to edit");
        for (original, changed) in edits {
            assert_eq!(
                text.matches(original).count(),
                1,
                "the text {original:?} appears exactly once"
            );
            text = text.replace(original, changed);
        }
        text.push_str(append);
        fs::write(path, text).expect("write the variant");
    };
    edit(
        &root.join("reference/aircraft_types.csv"),
        aircraft_edits,
        "",
    );
    edit(
        &root.join("reference/air_combat.toml"),
        air_combat_edits,
        "",
    );
    let scenario_file = root.join("scenarios/06a_base_raid.toml");
    fs::copy(data.join("scenarios/06a_base_raid.toml"), &scenario_file).expect("copy the scenario");
    edit(&scenario_file, edits, append);
    Scenario::load(&scenario_file).expect("the variant loads")
}

const GUNS: &str = "anti_aircraft = 20\n";
const KEEP_ORDER: &str = "share = \"0.5\"";
const GROUP_3_MISSION: &str =
    "side = \"allied\"\nair_group = 3\nkind = \"set_air_mission\"\nmission = \"combat_air_patrol\"";
const GROUP_4_AS_FIGHTERS: &str = "aircraft_type = 3\nready = 10";
const GROUP_4_PATROL: &str = "\n[[order]]\nday = 1\nside = \"allied\"\nair_group = 4\nkind = \"set_air_mission\"\nmission = \"combat_air_patrol\"\n";

fn with_guns(guns: u32) -> Scenario {
    let edit = if guns == 0 {
        String::new()
    } else {
        format!("anti_aircraft = {guns}\n")
    };
    variant(&format!("guns_{guns}"), &[(GUNS, &edit)], "", &[], &[])
}

fn with_keep(tenths: u32) -> Scenario {
    let share = format!("share = \"{}\"", Fixed::from_ratio(i64::from(tenths), 10));
    variant(
        &format!("keep_{tenths}"),
        &[(KEEP_ORDER, &share)],
        "",
        &[],
        &[],
    )
}

/// Group 4 flies fighters and is on combat air patrol with group 3.
fn group_4_on_patrol() -> Scenario {
    variant(
        "group_4_patrol",
        &[(GROUP_4_AS_FIGHTERS, "aircraft_type = 4\nready = 10")],
        GROUP_4_PATROL,
        &[],
        &[],
    )
}

fn group_3_rests() -> Scenario {
    variant(
        "group_3_rests",
        &[(
            GROUP_3_MISSION,
            "side = \"allied\"\nair_group = 3\nkind = \"set_air_mission\"\nmission = \"rest\"",
        )],
        "",
        &[],
        &[],
    )
}

fn airfield_b_at_280_nm() -> Scenario {
    variant(
        "far",
        &[(
            "q = 5\nr = 0\nsupply_t = 1000",
            "q = 7\nr = 0\nsupply_t = 1000",
        )],
        "",
        &[],
        &[],
    )
}

fn certain_supply_hits() -> Scenario {
    variant(
        "supply_100",
        &[("supply_t = 1000", "supply_t = 100")],
        "",
        &[],
        &[
            ("bomb_hit_chance = \"0.2\"", "bomb_hit_chance = \"1\""),
            ("aircraft = \"0.3\"", "aircraft = \"0\""),
            ("supply = \"0.3\"", "supply = \"1\""),
        ],
    )
}

fn group_1_on_naval_strike() -> Scenario {
    variant(
        "naval",
        &[(
            "mission = \"base_strike\"\ntarget_base = 2",
            "mission = \"naval_strike\"",
        )],
        "",
        &[],
        &[],
    )
}

fn patrol_variant_config() -> Config {
    Config {
        patrol_groups: vec![DEFENDERS, RESTING],
        ground_groups: vec![],
        ..scenario_config()
    }
}

fn rest_variant_config() -> Config {
    Config {
        patrol_groups: vec![],
        ground_groups: vec![DEFENDERS, RESTING],
        ..scenario_config()
    }
}

// ---------------------------------------------------------------------
// The setting.
// ---------------------------------------------------------------------

#[test]
fn the_setting_is_as_the_hand_calculation_says() {
    let scenario = load();
    assert_eq!(scenario.days, 3);
    let truth = scenario.world.umpire();
    let (a, b) = (&truth.bases[&AIRFIELD_A], &truth.bases[&AIRFIELD_B]);
    assert_eq!((a.side, a.hex), (Side::Japanese, Hex::new(0, 0)));
    assert_eq!((b.side, b.hex), (Side::Allied, Hex::new(5, 0)));
    // 5 hexes, 200 nm.
    assert_eq!(a.hex.distance(b.hex), 5);
    assert_eq!(b.supply_t, Fixed::from_int(1000));
    assert_eq!((a.anti_aircraft, b.anti_aircraft), (0, 20));
    assert_eq!(a.supply_t, Fixed::ZERO);
    let aircraft = |id: u32| {
        let g = &truth.air_groups[&group(id)];
        (
            g.side,
            g.location,
            g.aircraft_type.raw(),
            g.ready,
            g.mission,
        )
    };
    let at = |base: BaseId| sim_core::state::AirGroupLocation::Base(base);
    assert_eq!(
        aircraft(BOMBERS),
        (Side::Japanese, at(AIRFIELD_A), 3, 18, AirMission::Rest)
    );
    assert_eq!(
        aircraft(ESCORTS),
        (Side::Japanese, at(AIRFIELD_A), 4, 12, AirMission::Rest)
    );
    assert_eq!(
        aircraft(DEFENDERS),
        (Side::Allied, at(AIRFIELD_B), 4, 12, AirMission::Rest)
    );
    assert_eq!(
        aircraft(RESTING),
        (Side::Allied, at(AIRFIELD_B), 3, 10, AirMission::Rest)
    );
    assert_eq!(
        aircraft(SCOUTS),
        (Side::Allied, at(AIRFIELD_B), 2, 6, AirMission::Rest)
    );
    // The types: a dive bomber and a fighter, each with a radius of 250 nm.
    let figures = scenario.world.ref_data();
    for id in [3, 4] {
        let kind = &figures.aircraft_types[&sim_core::ids::AircraftTypeId::new(id)];
        assert_eq!(kind.strike_range_nm, Fixed::from_int(250));
    }
    // A fighter's shot 0.3, a gun's 0.05, a bomb 0.2; a hit on a base lands
    // on aircraft with chance 0.3, on supply 0.3, and destroys 50 t.
    let air = figures.air_combat.as_ref().expect("air combat figures");
    assert_eq!(air.fighter_hit_chance, tenths(3));
    assert_eq!(air.anti_aircraft_hit_chance, hundredths(5));
    assert_eq!(air.bomb_hit_chance, tenths(2));
    assert_eq!(air.base_hit_aircraft, tenths(3));
    assert_eq!(air.base_hit_supply, tenths(3));
    assert_eq!(air.base_supply_destroyed_per_hit_t, Fixed::from_int(50));
    // The orders, as they stand after the run: the bombers on a base strike
    // on airfield B, group 2 on patrol at share 0.5, group 3 on patrol at
    // the default 1.
    let mut scenario = load();
    scenario.run_to_end().expect("runs");
    let truth = scenario.world.umpire();
    let mission = |id: u32| truth.air_groups[&group(id)].mission;
    assert_eq!(mission(BOMBERS), AirMission::BaseStrike(AIRFIELD_B));
    assert_eq!(mission(ESCORTS), AirMission::CombatAirPatrol);
    assert_eq!(truth.air_groups[&group(ESCORTS)].patrol_share, tenths(5));
    assert_eq!(mission(DEFENDERS), AirMission::CombatAirPatrol);
    assert_eq!(truth.air_groups[&group(DEFENDERS)].patrol_share, Fixed::ONE);
    assert_eq!(mission(RESTING), AirMission::Rest);
    assert_eq!(mission(SCOUTS), AirMission::Search);
}

// ---------------------------------------------------------------------
// When a raid flies.
// ---------------------------------------------------------------------

#[test]
fn a_raid_flies_each_morning_while_group_1_has_an_aircraft_and_in_no_other_pulse() {
    let mut raids = [0usize; 4];
    for (run, worked) in batch().iter().zip(worked_batch()) {
        // Worked by the rule, the raids are those of the mornings of days 1,
        // 2 and 3 on which group 1 had an aircraft, in order, none skipped
        // while it had one.
        let days: Vec<u32> = worked.iter().map(|raid| raid.day).collect();
        assert!(
            days.starts_with(&[1]),
            "seed {}: the first raid always flies",
            run.seed
        );
        for pair in days.windows(2) {
            assert_eq!(pair[1], pair[0] + 1, "seed {}", run.seed);
        }
        raids[worked.len()] += 1;
        let launches = run.of(|k| matches!(k, EventKind::BaseStrikeLaunched { .. }));
        assert_eq!(launches.len(), worked.len(), "seed {}", run.seed);
        for launch in &launches {
            assert_eq!(launch.pulse, Some(Pulse::Morning), "seed {}", run.seed);
        }
        // The Allied groups never launch: nothing but group 1 does.
        for event in &launches {
            assert!(matches!(
                event.kind,
                EventKind::BaseStrikeLaunched { air_group, .. } if air_group == group(BOMBERS)
            ));
        }
        // After the last raid group 1 had no aircraft, or the days ran out.
        if worked.len() < 3 {
            assert_eq!(run.ready[0], 0, "seed {}", run.seed);
        }
    }
    eprintln!(
        "raids per run: 0 {}, 1 {}, 2 {}, 3 {}",
        raids[0], raids[1], raids[2], raids[3]
    );
    assert_eq!(raids[0], 0);
    assert!(raids[3] > 900, "three raids are flown in nearly every run");
}

#[test]
fn no_launch_needs_a_contact_and_no_report_is_made_and_no_contact_is_held() {
    for run in batch() {
        assert_eq!(run.contacts, [0, 0], "seed {}", run.seed);
        assert!(
            run.of(|k| matches!(
                k,
                EventKind::ReportReceived { .. }
                    | EventKind::ContactAged { .. }
                    | EventKind::ContactDropped { .. }
                    | EventKind::ContactNotThere { .. }
                    | EventKind::SearchAttempt { .. }
                    | EventKind::StrikeLaunched { .. }
                    | EventKind::StrikeFoundNothing { .. }
            ))
            .is_empty(),
            "seed {}: nothing is sighted, nobody has a ship to sight",
            run.seed
        );
    }
}

// ---------------------------------------------------------------------
// The first raid.
// ---------------------------------------------------------------------

#[test]
fn the_first_raid_meets_6_escorts_12_patrol_and_20_gun_shots_by_the_table() {
    let mut firsts = Vec::new();
    for worked in worked_batch() {
        let first = &worked[0];
        assert_eq!(first.day, 1);
        // B = 18, F = 12 so 6 stay and E = 6, P = 12; group 4's 10 are on
        // the ground.
        assert_eq!(
            (
                first.bombers,
                first.fighters,
                first.escorts,
                first.patrol,
                first.on_ground
            ),
            (18, 12, 6, 12, 10)
        );
        // The table: shots, and from none to the group's aircraft.
        let (shots, hits) = first.escorts_at_patrol.expect("rolled");
        assert_eq!(shots, 6);
        assert!(hits <= 6);
        let (shots, hits) = first.patrol_at_escorts.expect("rolled");
        assert_eq!(shots, 6);
        assert!(hits <= 6);
        let (shots, hits) = first.patrol_at_bombers.expect("rolled");
        assert_eq!(shots, 6);
        assert!(hits <= 6);
        let (shots, hits) = first.guns.expect("rolled: bombers are left");
        assert_eq!(shots, 20);
        assert!(hits <= 20);
        // The bombers that attack are 18 less the losses to the patrol and
        // the guns, a group losing no more than it has.
        let to_patrol = first.patrol_at_bombers.expect("rolled").1;
        let to_guns = first.guns.expect("rolled").1.min(18 - to_patrol);
        assert_eq!(first.bombers_lost, to_patrol + to_guns);
        assert_eq!(first.bombers_attacking, 18 - to_patrol - to_guns);
        assert!(first.bomb_hits <= first.bombers_attacking);
        firsts.push(first);
    }
    // The chances and expected hits are exact in every event, asserted for
    // every raid in the rule's rebuild. Here, the table's expected column.
    for run in batch().iter().take(50) {
        let fights = run.of(|k| matches!(k, EventKind::AirFight { .. }));
        for fight in &fights[..3] {
            match fight.kind {
                EventKind::AirFight {
                    shots,
                    chance,
                    expected,
                    ..
                } => {
                    assert_eq!((shots, chance, expected), (int(6), tenths(3), tenths(18)));
                }
                _ => unreachable!(),
            }
        }
        let guns = run.of(|k| matches!(k, EventKind::BaseAntiAircraftFire { .. }));
        match guns[0].kind {
            EventKind::BaseAntiAircraftFire {
                shots,
                chance,
                expected,
                ..
            } => {
                assert_eq!((shots, chance, expected), (int(20), hundredths(5), int(1)));
            }
            _ => unreachable!(),
        }
    }

    // Over the runs, the hits of each roll are within four standard
    // deviations of the expected figure times 1,000.
    let band = |name: &str, shots: i64, hits: i64, p: i64| {
        eprintln!("{name}: {hits} hits of {shots} shots");
        assert!(
            within_four_sigma(hits, shots, p),
            "{name}: {hits} hits of {shots} shots at {p} millionths"
        );
    };
    let total = |roll: fn(&Raid) -> Option<(u32, u32)>| -> (i64, i64) {
        firsts
            .iter()
            .map(|raid| roll(raid).expect("rolled"))
            .fold((0, 0), |(s, h), (shots, hits)| {
                (s + i64::from(shots), h + i64::from(hits))
            })
    };
    let (shots, hits) = total(|r| r.escorts_at_patrol);
    band("escorts at the patrol", shots, hits, 300_000);
    let (shots, hits) = total(|r| r.patrol_at_escorts);
    band("patrol at the escorts", shots, hits, 300_000);
    let (shots, hits) = total(|r| r.patrol_at_bombers);
    band("patrol at the bombers", shots, hits, 300_000);
    let (shots, hits) = total(|r| r.guns);
    band("guns at the bombers", shots, hits, 50_000);
    // The bomb hits are within four standard deviations of 0.2 times the
    // bombers that attacked.
    let bombers: i64 = firsts.iter().map(|r| i64::from(r.bombers_attacking)).sum();
    let bomb_hits: i64 = firsts.iter().map(|r| i64::from(r.bomb_hits)).sum();
    band("bombs", bombers, bomb_hits, 200_000);
    // On average 15.2 bombers attack and score 3.04 hits.
    let mean_bombers = bombers * 100 / count(firsts.len());
    eprintln!(
        "mean bombers attacking x100: {mean_bombers}, mean hits x100: {}",
        bomb_hits * 100 / count(firsts.len())
    );
    assert!((1501..=1539).contains(&mean_bombers), "{mean_bombers}");
}

#[test]
fn of_all_the_bomb_hits_of_all_the_raids_3_in_10_land_on_aircraft_and_3_in_10_on_supply() {
    let (mut hits, mut aircraft, mut supply) = (0i64, 0i64, 0i64);
    for raid in worked_batch().iter().flatten() {
        hits += i64::from(raid.bomb_hits);
        aircraft += i64::from(raid.on_aircraft);
        supply += i64::from(raid.on_supply);
        assert!(raid.on_aircraft + raid.on_supply <= raid.bomb_hits);
    }
    eprintln!("{hits} bomb hits: {aircraft} on aircraft, {supply} on supply");
    assert!(
        within_four_sigma(aircraft, hits, 300_000),
        "{aircraft} of {hits}"
    );
    assert!(
        within_four_sigma(supply, hits, 300_000),
        "{supply} of {hits}"
    );
}

// ---------------------------------------------------------------------
// Later raids, and every raid by the rule.
// ---------------------------------------------------------------------

#[test]
fn every_raid_of_every_run_is_worked_by_the_rule_from_the_counts_at_the_start_of_its_pulse() {
    // work_the_run asserts every event of every raid: the launch, the
    // escorts, each fight's shots, chance, expected figure, hits, causes and
    // breakdown, the guns, the losses, the attack and where each hit landed,
    // the aircraft destroyed on the ground and the supply destroyed. Here, the
    // counts it carried.
    let (mut raids, mut later) = (0usize, 0usize);
    for (run, worked) in batch().iter().zip(worked_batch()) {
        let mut bombers = 18;
        let mut fighters = 12;
        let mut defenders = 12;
        let mut ground = 10;
        for raid in worked {
            raids += 1;
            if raid.day > 1 {
                later += 1;
            }
            // A raid starts from what its groups had after the raids before.
            assert_eq!(
                (raid.bombers, raid.fighters, raid.patrol, raid.on_ground),
                (bombers, fighters, defenders, ground),
                "seed {}, day {}",
                run.seed,
                raid.day
            );
            assert_eq!(raid.escorts, fighters - fighters / 2);
            bombers -= raid.bombers_lost;
            fighters -= raid
                .patrol_at_escorts
                .map_or(0, |(_, hits)| hits.min(raid.escorts));
            defenders -= raid
                .escorts_at_patrol
                .map_or(0, |(_, hits)| hits.min(raid.patrol));
            ground -= raid.destroyed_on_ground;
        }
    }
    eprintln!("{raids} raids, {later} of them after the first day");
    assert!(later > 1000);
}

#[test]
fn a_later_raid_has_the_aircraft_the_groups_have_that_morning() {
    let mut smaller = 0;
    for worked in worked_batch() {
        for pair in worked.windows(2) {
            // Bombers never come back: each raid has no more than the last.
            assert!(pair[1].bombers <= pair[0].bombers);
            assert!(pair[1].on_ground <= pair[0].on_ground);
            assert!(pair[1].patrol <= pair[0].patrol);
            // And in fact exactly what was left of them.
            assert_eq!(pair[1].bombers, pair[0].bombers - pair[0].bombers_lost);
            assert_eq!(
                pair[1].on_ground,
                pair[0].on_ground - pair[0].destroyed_on_ground
            );
            if pair[1].bombers < pair[0].bombers {
                smaller += 1;
            }
        }
    }
    assert!(smaller > 1000);
}

// ---------------------------------------------------------------------
// The end of a run.
// ---------------------------------------------------------------------

#[test]
fn at_the_end_each_group_and_each_airfield_has_what_the_raids_left_it() {
    for (run, worked) in batch().iter().zip(worked_batch()) {
        let sum = |value: fn(&Raid) -> u32| -> u32 { worked.iter().map(value).sum() };
        // Group 4: 10 less the aircraft destroyed on the ground, which is
        // the sum over the raids of the smaller of the hits on aircraft and
        // what group 4 had that morning.
        let mut had = 10;
        let mut destroyed = 0;
        for raid in worked {
            let now = raid.on_aircraft.min(had);
            assert_eq!(now, raid.destroyed_on_ground);
            had -= now;
            destroyed += now;
        }
        assert_eq!(run.ready[3], 10 - destroyed, "seed {}", run.seed);
        assert_eq!(destroyed, sum(|raid| raid.destroyed_on_ground));
        // Airfield B: 1,000 t less 50 t for each hit on supply. Three raids
        // of 18 bombers cannot empty it, so the cap is not reached.
        let on_supply = i64::from(sum(|raid| raid.on_supply));
        assert!(
            on_supply < 20,
            "seed {}: 20 hits would empty the airfield",
            run.seed
        );
        assert_eq!(
            run.supply[1],
            Fixed::from_int(1000 - 50 * on_supply),
            "seed {}",
            run.seed
        );
        // Group 1: 18 less its losses to fighters and guns; group 2: 12 less
        // its losses to the patrol; group 3: 12 less its losses to the
        // escorts. Nothing else changes any group.
        let lost = |g: u32| -> u32 {
            run.events
                .iter()
                .map(|event| match event.kind {
                    EventKind::AircraftLost {
                        air_group,
                        aircraft,
                        ..
                    }
                    | EventKind::AircraftDestroyedOnGround {
                        air_group,
                        aircraft,
                        ..
                    } if air_group == group(g) => aircraft,
                    _ => 0,
                })
                .sum()
        };
        for (index, g) in [BOMBERS, ESCORTS, DEFENDERS, RESTING, SCOUTS]
            .into_iter()
            .enumerate()
        {
            assert_eq!(
                run.ready[index],
                START[index] - lost(g),
                "seed {}, group {g}",
                run.seed
            );
        }
        assert_eq!(run.ready[4], 6);
        // Airfield A's supply is 0 throughout.
        assert_eq!(run.supply[0], Fixed::ZERO);
    }
}

// ---------------------------------------------------------------------
// The variants, each for its first raid and all its raids by the rule.
// ---------------------------------------------------------------------

#[test]
fn with_group_4_on_patrol_the_patrol_is_22_and_nothing_is_on_the_ground() {
    // Group 4 flies fighters here: only a fighter type can be on patrol.
    let runs = batch_of(&group_4_on_patrol(), SEEDS);
    let config = patrol_variant_config();
    let firsts = first_raids(&runs, &config);
    for first in &firsts {
        assert_eq!((first.patrol, first.escorts, first.on_ground), (22, 6, 0));
        // 6 shots at the escorts, 16 at the bombers: 4.8 expected.
        assert_eq!(first.patrol_at_escorts.expect("rolled").0, 6);
        assert_eq!(first.patrol_at_bombers.expect("rolled").0, 16);
    }
    for run in &runs {
        assert!(
            run.of(|k| matches!(k, EventKind::AircraftDestroyedOnGround { .. }))
                .is_empty(),
            "seed {}: nothing is on the ground, whatever the hits on aircraft",
            run.seed
        );
        for raid in work_the_run(run, &config) {
            assert_eq!(raid.destroyed_on_ground, 0);
        }
    }
    // Some hits on aircraft were wasted.
    let wasted: u32 = firsts.iter().map(|raid| raid.on_aircraft).sum();
    assert!(wasted > 100);
    let expected = tenths(48);
    assert_eq!(int(16) * tenths(3), expected);
}

#[test]
fn with_group_3_resting_there_is_no_patrol_and_22_on_the_ground_taken_from_group_3_first() {
    let runs = batch_of(&group_3_rests(), SEEDS);
    let config = rest_variant_config();
    let firsts = first_raids(&runs, &config);
    let mut from_group_3 = 0;
    for first in &firsts {
        assert_eq!((first.patrol, first.on_ground), (0, 22));
        assert!(first.escorts_at_patrol.is_none() && first.patrol_at_escorts.is_none());
        assert!(first.patrol_at_bombers.is_none());
        // Only the guns cost bombers.
        assert_eq!(
            first.bombers_attacking,
            18 - first.guns.expect("rolled").1.min(18)
        );
    }
    for run in &runs {
        // No aircraft is lost to fighters: nobody fires at anything.
        assert!(run
            .of(|k| matches!(k, EventKind::AirFight { .. }))
            .is_empty());
        // Those destroyed on the ground come off group 3 first: group 4 is
        // hit only once group 3 has nothing left in that raid.
        let ground = run.of(|k| matches!(k, EventKind::AircraftDestroyedOnGround { .. }));
        for event in ground {
            if let EventKind::AircraftDestroyedOnGround { air_group, .. } = event.kind {
                if air_group == group(DEFENDERS) {
                    from_group_3 += 1;
                }
            }
        }
        work_the_run(run, &config);
        assert!(run.ready[3] <= 10 && run.ready[2] <= 12);
        // Group 4 loses nothing while group 3 still had aircraft to lose.
        let first_destroyed = run
            .events
            .iter()
            .filter_map(|event| match event.kind {
                EventKind::AircraftDestroyedOnGround { air_group, .. } => Some(air_group),
                _ => None,
            })
            .next();
        if let Some(first) = first_destroyed {
            assert_eq!(first, group(DEFENDERS), "seed {}", run.seed);
        }
    }
    assert!(from_group_3 > 500);
}

#[test]
fn with_no_guns_no_anti_aircraft_fire_is_recorded_and_the_patrol_alone_costs_bombers() {
    let runs = batch_of(&with_guns(0), SEEDS);
    let mut config = scenario_config();
    config.guns = 0;
    let firsts = first_raids(&runs, &config);
    for (run, first) in runs.iter().zip(&firsts) {
        assert!(first.guns.is_none());
        assert!(run
            .of(|k| matches!(k, EventKind::BaseAntiAircraftFire { .. }))
            .is_empty());
        // The bombers attacking are 18 less the patrol's hits.
        assert_eq!(
            first.bombers_attacking,
            18 - first.patrol_at_bombers.expect("rolled").1
        );
    }
}

#[test]
fn an_airfield_at_280_nm_is_beyond_the_bombers_and_nothing_happens_in_any_pulse() {
    let scenario = airfield_b_at_280_nm();
    assert_eq!(Hex::new(0, 0).distance(Hex::new(7, 0)), 7);
    for run in batch_of(&scenario, 50) {
        assert!(
            run.of(|k| matches!(
                k,
                EventKind::BaseStrikeLaunched { .. }
                    | EventKind::EscortsSent { .. }
                    | EventKind::AirFight { .. }
                    | EventKind::BaseAntiAircraftFire { .. }
                    | EventKind::AircraftLost { .. }
                    | EventKind::BaseAttacked { .. }
                    | EventKind::AircraftDestroyedOnGround { .. }
                    | EventKind::BaseSupplyDestroyed { .. }
            ))
            .is_empty(),
            "seed {}",
            run.seed
        );
        assert_eq!(run.ready, START, "seed {}", run.seed);
        assert_eq!(run.supply, [Fixed::ZERO, Fixed::from_int(1000)]);
    }
}

#[test]
fn with_100_t_and_every_bomb_landing_on_supply_each_raid_destroys_what_is_left_and_no_more() {
    let runs = batch_of(&certain_supply_hits(), SEEDS);
    let config = Config {
        bomb: Fixed::ONE,
        lands_on_aircraft: Fixed::ZERO,
        lands_on_supply: Fixed::ONE,
        supply: 100,
        ..scenario_config()
    };
    let mut emptied = 0;
    for run in &runs {
        let worked = work_the_run(run, &config);
        let first = &worked[0];
        // Every bomb that attacks hits and lands on supply.
        assert_eq!(first.bomb_hits, first.bombers_attacking);
        assert_eq!(first.on_supply, first.bombers_attacking);
        if first.bombers_attacking >= 2 {
            // 2 x 50 t or more: the 100 t are all gone, and no more.
            assert_eq!(first.supply_destroyed, 100, "seed {}", run.seed);
            assert_eq!(run.supply[1], Fixed::ZERO);
            emptied += 1;
            // The later raids find nothing to destroy and record nothing.
            for later in &worked[1..] {
                assert_eq!(later.supply_destroyed, 0);
            }
        }
        // Never below 0, and never more destroyed than there was.
        assert!(!run.supply[1].is_negative());
        let total: i64 = worked.iter().map(|raid| raid.supply_destroyed).sum();
        assert!(total <= 100);
        assert_eq!(run.supply[1], Fixed::from_int(100 - total));
    }
    assert!(emptied > 900);
}

#[test]
fn a_group_on_a_naval_strike_never_launches_because_there_is_no_contact() {
    let scenario = group_1_on_naval_strike();
    for run in batch_of(&scenario, 50) {
        assert!(
            run.of(|k| matches!(
                k,
                EventKind::BaseStrikeLaunched { .. }
                    | EventKind::StrikeLaunched { .. }
                    | EventKind::EscortsSent { .. }
                    | EventKind::AirFight { .. }
                    | EventKind::BaseAttacked { .. }
            ))
            .is_empty(),
            "seed {}: a base strike and a naval strike are different orders",
            run.seed
        );
        assert_eq!(run.ready, START);
    }
}

// ---------------------------------------------------------------------
// What cannot happen.
// ---------------------------------------------------------------------

#[test]
fn no_raid_flies_at_night_or_in_the_afternoon_and_nothing_else_cannot_happen() {
    for run in batch() {
        for event in &run.events {
            let raid_event = matches!(
                event.kind,
                EventKind::BaseStrikeLaunched { .. }
                    | EventKind::EscortsSent { .. }
                    | EventKind::AirFight { .. }
                    | EventKind::BaseAntiAircraftFire { .. }
                    | EventKind::AircraftLost { .. }
                    | EventKind::BaseAttacked { .. }
                    | EventKind::AircraftDestroyedOnGround { .. }
                    | EventKind::BaseSupplyDestroyed { .. }
            );
            if raid_event {
                assert_eq!(
                    event.pulse,
                    Some(Pulse::Morning),
                    "seed {}: {}",
                    run.seed,
                    event.kind
                );
            }
            match event.kind {
                // No aircraft of group 5 is ever lost, and none of group 3
                // is destroyed on the ground.
                EventKind::AircraftLost { air_group, .. } => {
                    assert_ne!(air_group, group(SCOUTS), "seed {}", run.seed);
                    assert_ne!(air_group, group(RESTING), "seed {}", run.seed);
                }
                EventKind::AircraftDestroyedOnGround { air_group, .. } => {
                    assert_eq!(air_group, group(RESTING), "seed {}", run.seed);
                }
                // Airfield A is never attacked and has no guns.
                EventKind::BaseAttacked { base, .. }
                | EventKind::BaseSupplyDestroyed { base, .. }
                | EventKind::BaseAntiAircraftFire { base, .. } => {
                    assert_eq!(base, AIRFIELD_B, "seed {}", run.seed);
                }
                _ => {}
            }
        }
        // Group 5 is searching and flies in every daylight pulse, 6 aircraft.
        assert_eq!(run.ready[4], 6);
        let flights = run.of(|k| matches!(k, EventKind::SearchFlown { .. })).len();
        assert_eq!(
            flights, 6,
            "seed {}: morning and afternoon of 3 days",
            run.seed
        );
        // No report is made and no contact is held (asserted above for the
        // events); and the Japanese side holds none either.
        assert_eq!(run.contacts, [0, 0]);
    }
}

// ---------------------------------------------------------------------
// Properties.
// ---------------------------------------------------------------------

#[test]
fn more_guns_never_mean_fewer_shots_and_never_a_smaller_expected_loss_to_guns() {
    // The hits of the guns on one seed come from one stream, shot by shot,
    // so more shots can only add hits, and the patrol's rolls do not
    // depend on the guns: attack by attack, more guns never leave more
    // bombers.
    let figures = |guns: u32| -> Vec<(u32, Fixed, u32, u32)> {
        let scenario = if guns == 20 { load() } else { with_guns(guns) };
        let runs = batch_of(&scenario, SEEDS);
        let config = Config {
            guns,
            ..scenario_config()
        };
        first_raids(&runs, &config)
            .iter()
            .map(|raid| match raid.guns {
                Some((shots, hits)) => (
                    shots,
                    int(shots) * hundredths(5),
                    hits,
                    raid.bombers_attacking,
                ),
                None => (0, Fixed::ZERO, 0, raid.bombers_attacking),
            })
            .collect()
    };
    let by_guns: Vec<Vec<(u32, Fixed, u32, u32)>> = [0, 10, 20, 40].map(figures).into();
    for (index, guns) in [0u32, 10, 20, 40].into_iter().enumerate() {
        for (shots, expected, ..) in &by_guns[index] {
            assert_eq!(*shots, guns);
            assert_eq!(*expected, int(guns) * hundredths(5));
        }
    }
    for pair in by_guns.windows(2) {
        for (fewer, more) in pair[0].iter().zip(&pair[1]) {
            assert!(fewer.0 <= more.0 && fewer.1 <= more.1);
            assert!(
                fewer.2 <= more.2,
                "more guns never score fewer hits on one seed"
            );
            assert!(fewer.3 >= more.3, "more guns never leave more bombers");
        }
    }
}

#[test]
fn more_escorts_never_mean_more_patrol_shots_at_the_bombers() {
    let shots_at_bombers = |keep_tenths: u32| -> Vec<u32> {
        let scenario = if keep_tenths == 5 {
            load()
        } else {
            with_keep(keep_tenths)
        };
        let runs = batch_of(&scenario, SEEDS);
        let config = Config {
            keep_tenths,
            ..scenario_config()
        };
        first_raids(&runs, &config)
            .iter()
            .map(|raid| raid.patrol_at_bombers.map_or(0, |(shots, _)| shots))
            .collect()
    };
    // Keep all: no escorts, the patrol of 12 fires all 12 at the bombers.
    // Keep half: 6 escorts, 6 at the bombers. Keep none: 12 escorts, none.
    let (all, half, none) = (
        shots_at_bombers(10),
        shots_at_bombers(5),
        shots_at_bombers(0),
    );
    assert!(all.iter().all(|shots| *shots == 12));
    assert!(half.iter().all(|shots| *shots == 6));
    assert!(none.iter().all(|shots| *shots == 0));
}

#[test]
fn more_aircraft_in_the_air_over_the_base_never_mean_more_aircraft_destroyed_on_the_ground() {
    // Total aircraft destroyed on the ground over the runs: none with group 4
    // on patrol, some in the scenario, more with group 3 on the ground too.
    let total = |runs: &[Run], config: &Config| -> u32 {
        runs.iter()
            .map(|run| {
                work_the_run(run, config)
                    .iter()
                    .map(|raid| raid.destroyed_on_ground)
                    .sum::<u32>()
            })
            .sum()
    };
    let in_the_air = total(
        &batch_of(&group_4_on_patrol(), SEEDS),
        &patrol_variant_config(),
    );
    let scenario = total(batch(), &scenario_config());
    let on_the_ground = total(&batch_of(&group_3_rests(), SEEDS), &rest_variant_config());
    eprintln!("destroyed on the ground over 1,000 runs: {in_the_air}, {scenario}, {on_the_ground}");
    assert_eq!(in_the_air, 0);
    assert!(scenario > 0 && scenario <= on_the_ground);
}

#[test]
fn recording_breakdowns_changes_neither_the_events_nor_the_hashes() {
    let base = load();
    for seed in [1u64, 2, 77, 500] {
        let mut with = base.with_seed(seed).expect("a seed can be set");
        let mut without = base.with_seed(seed).expect("a seed can be set");
        without.world.set_config(sim_core::world::SimConfig {
            record_breakdowns: false,
        });
        with.run_to_end().expect("runs");
        without.run_to_end().expect("runs");
        assert_eq!(
            with.world.day_hashes(),
            without.world.day_hashes(),
            "seed {seed}"
        );
        let (a, b): (Vec<&Event>, Vec<&Event>) = (
            with.world.events().iter().collect(),
            without.world.events().iter().collect(),
        );
        assert_eq!(a.len(), b.len());
        let mut breakdowns = 0;
        for (x, y) in a.iter().zip(&b) {
            assert_eq!(
                (x.id, x.day, x.pulse, &x.kind, &x.causes),
                (y.id, y.day, y.pulse, &y.kind, &y.causes)
            );
            assert!(y.breakdown.is_none());
            breakdowns += usize::from(x.breakdown.is_some());
        }
        assert!(breakdowns > 5);
    }
}

#[test]
fn the_same_seed_gives_the_same_hashes_twice_and_the_seed_and_the_mission_matter() {
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
        assert_eq!(hashes[0].len(), 3);
        assert_eq!(hashes[0], hashes[1], "seed {seed}");
        all_hashes.push(hashes[0].clone());
    }
    assert!(all_hashes.windows(2).any(|pair| pair[0] != pair[1]));
    // The batch holds the same hashes as a fresh run of the same seed.
    let mut again = base.with_seed(77).expect("a seed can be set");
    again.run_to_end().expect("the run resolves");
    assert_eq!(batch()[76].seed, 77);
    assert_eq!(batch()[76].hashes, again.world.day_hashes());
    // The base strike and its target are in the hash: the same days with
    // another mission end in another state.
    let mut naval = group_1_on_naval_strike()
        .with_seed(77)
        .expect("a seed can be set");
    naval.run_to_end().expect("runs");
    assert_ne!(naval.world.day_hashes(), again.world.day_hashes());
}

#[test]
fn mean_figures_of_the_scenario_over_1000_seeds() {
    // Not an assertion about the rule: the figures a reader wants, printed
    // with --nocapture.
    let runs = batch();
    let worked = worked_batch();
    let raids: usize = worked.iter().map(Vec::len).sum();
    let attacking: u32 = worked
        .iter()
        .flatten()
        .map(|raid| raid.bombers_attacking)
        .sum();
    let on_ground: u32 = worked
        .iter()
        .flatten()
        .map(|raid| raid.destroyed_on_ground)
        .sum();
    let supply: i64 = worked
        .iter()
        .flatten()
        .map(|raid| raid.supply_destroyed)
        .sum();
    let mean = |total: i64, n: usize| total * 100 / count(n);
    eprintln!(
        "mean bombers attacking per raid x100: {}",
        mean(i64::from(attacking), raids)
    );
    eprintln!(
        "mean raids per run x100: {}",
        mean(count(raids), runs.len())
    );
    eprintln!(
        "mean aircraft destroyed on the ground per run x100: {}",
        mean(i64::from(on_ground), runs.len())
    );
    eprintln!(
        "mean supply destroyed per run x100: {}",
        mean(supply, runs.len())
    );
    for (index, id) in [BOMBERS, ESCORTS, DEFENDERS, RESTING, SCOUTS]
        .into_iter()
        .enumerate()
    {
        let total: u32 = runs.iter().map(|run| run.ready[index]).sum();
        eprintln!(
            "group {id} mean aircraft at the end x100: {}",
            mean(i64::from(total), runs.len())
        );
    }
    assert!(raids > 0);
}
